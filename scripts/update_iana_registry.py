#!/usr/bin/env python3
"""Update and verify the checked-in IANA special-purpose registry snapshot.

The published crate is intentionally offline. This tool is for maintainers:
check verifies generated Rust and provenance from checked-in XML, check-upstream
only reads IANA's current XML, and sync refreshes checked-in inputs after both
registries validate.
"""

import argparse
import hashlib
import ipaddress
import os
from dataclasses import dataclass
from datetime import date
from pathlib import Path
import re
import sys
import tempfile
from typing import Callable, Dict, Iterable, List, Optional, Sequence, Tuple, Union
from urllib.parse import urlparse
from urllib.request import Request, urlopen
import xml.etree.ElementTree as ElementTree


IANA_NAMESPACE = "http://www.iana.org/assignments"
NAMESPACE_PREFIX = "{" + IANA_NAMESPACE + "}"
XREF_TAG = NAMESPACE_PREFIX + "xref"
ROOT = Path(__file__).resolve().parents[1]
GENERATED_RELATIVE = Path("src/iana_special_registry_data.rs")
PROVENANCE_RELATIVE = Path("data/iana/README.md")
REQUIRED_RECORD_FIELDS = (
    "address",
    "name",
    "spec",
    "allocation",
    "source",
    "destination",
    "forwardable",
    "global",
    "reserved",
)
OPTIONAL_RECORD_FIELDS = ("termination",)
BOOLEAN_RECORD_FIELDS = ("source", "destination", "forwardable", "global", "reserved")
ISO_DATE = re.compile(r"\d{4}-\d{2}-\d{2}\Z")

Network = Union[ipaddress.IPv4Network, ipaddress.IPv6Network]
Fetcher = Callable[[str], bytes]


class RegistryError(ValueError):
    """The checked-in or upstream registry did not match the expected schema."""


@dataclass(frozen=True)
class SourceSpec:
    """The immutable identity and local destination of one IANA registry."""

    family: int
    name: str
    url: str
    root_id: str
    inner_id: str
    fixture: Path


@dataclass(frozen=True)
class Record:
    """One CIDR from a registry row, retaining its explicit forwarding state."""

    network: Network
    forwardable: Optional[bool]


@dataclass(frozen=True)
class Registry:
    """A parsed, version-pinned IANA registry snapshot."""

    spec: SourceSpec
    updated: str
    sha256: str
    row_count: int
    records: Tuple[Record, ...]


def source_specs(repo: Path = ROOT) -> Tuple[SourceSpec, SourceSpec]:
    """Return the two hardcoded IANA sources rooted at repo."""

    repo = Path(repo)
    fixtures = repo / "data" / "iana"
    return (
        SourceSpec(
            family=4,
            name="IPv4",
            url=(
                "https://www.iana.org/assignments/iana-ipv4-special-registry/"
                "iana-ipv4-special-registry.xml"
            ),
            root_id="iana-ipv4-special-registry",
            inner_id="iana-ipv4-special-registry-1",
            fixture=fixtures / "iana-ipv4-special-registry.xml",
        ),
        SourceSpec(
            family=6,
            name="IPv6",
            url=(
                "https://www.iana.org/assignments/iana-ipv6-special-registry/"
                "iana-ipv6-special-registry.xml"
            ),
            root_id="iana-ipv6-special-registry",
            inner_id="iana-ipv6-special-registry-1",
            fixture=fixtures / "iana-ipv6-special-registry.xml",
        ),
    )


def qualified(name: str) -> str:
    """Return an exact IANA XML qualified tag name."""

    return NAMESPACE_PREFIX + name


def sha256(data: bytes) -> str:
    """Return a lowercase SHA-256 digest for provenance and drift reports."""

    return hashlib.sha256(data).hexdigest()


def normalized_text(element: ElementTree.Element, context: str) -> str:
    """Return field text after discarding IANA note xrefs and whitespace.

    IANA uses empty xref elements to annotate address and Boolean text. They
    are metadata, not part of a CIDR or Boolean token. Other inline markup is
    rejected so a schema change cannot become silently meaningful.
    """

    parts = [element.text or ""]
    for child in element:
        if child.tag != XREF_TAG or list(child):
            raise RegistryError(
                "{} contains unsupported inline XML {!r}".format(context, child.tag)
            )
        parts.append(child.tail or "")
    return " ".join("".join(parts).split())


def parse_flag(value: str, context: str) -> Optional[bool]:
    """Parse the only IANA Boolean vocabulary relevant to this snapshot."""

    if value == "True":
        return True
    if value == "False":
        return False
    if value in ("", "N/A"):
        return None
    raise RegistryError(
        "{} has unsupported Boolean value {!r}; expected True, False, blank, or N/A".format(
            context, value
        )
    )


def parse_updated(root: ElementTree.Element, spec: SourceSpec) -> str:
    """Require a single valid ISO date on the exact registry root."""

    updated_nodes = [child for child in root if child.tag == qualified("updated")]
    if len(updated_nodes) != 1:
        raise RegistryError("{} root must contain exactly one updated field".format(spec.name))
    updated = normalized_text(updated_nodes[0], "{} root updated".format(spec.name))
    if not ISO_DATE.fullmatch(updated):
        raise RegistryError(
            "{} root updated must be an ISO YYYY-MM-DD date, got {!r}".format(
                spec.name, updated
            )
        )
    try:
        date.fromisoformat(updated)
    except ValueError as error:
        raise RegistryError(
            "{} root updated is not a valid date: {!r}".format(spec.name, updated)
        ) from error
    return updated


def field_map(
    record: ElementTree.Element, spec: SourceSpec, record_number: int
) -> Dict[str, str]:
    """Validate and normalize every schema field required for a record."""

    fields: Dict[str, str] = {}
    allowed = set(REQUIRED_RECORD_FIELDS + OPTIONAL_RECORD_FIELDS)
    context = "{} record {}".format(spec.name, record_number)
    for child in record:
        if not isinstance(child.tag, str) or not child.tag.startswith(NAMESPACE_PREFIX):
            raise RegistryError("{} has a field outside the IANA namespace".format(context))
        name = child.tag[len(NAMESPACE_PREFIX) :]
        if name not in allowed:
            raise RegistryError("{} has unexpected field {!r}".format(context, name))
        if name in fields:
            raise RegistryError("{} repeats field {!r}".format(context, name))
        fields[name] = normalized_text(child, "{} field {}".format(context, name))

    missing = [name for name in REQUIRED_RECORD_FIELDS if name not in fields]
    if missing:
        raise RegistryError("{} is missing required fields {}".format(context, ", ".join(missing)))

    for name in BOOLEAN_RECORD_FIELDS:
        parse_flag(fields[name], "{} field {}".format(context, name))
    return fields


def parse_networks(
    address_field: str, spec: SourceSpec, record_number: int
) -> Tuple[Network, ...]:
    """Split source-order CIDRs and require canonical ranges of the right family."""

    if not address_field:
        raise RegistryError("{} record {} has an empty address field".format(spec.name, record_number))

    networks: List[Network] = []
    for item in address_field.split(","):
        cidr = item.strip()
        if not cidr:
            raise RegistryError(
                "{} record {} has an empty CIDR in its address list".format(
                    spec.name, record_number
                )
            )
        try:
            network = ipaddress.ip_network(cidr, strict=True)
        except ValueError as error:
            raise RegistryError(
                "{} record {} has invalid CIDR {!r}".format(
                    spec.name, record_number, cidr
                )
            ) from error
        if network.version != spec.family:
            raise RegistryError(
                "{} record {} has IPv{} CIDR {!r}".format(
                    spec.name, record_number, network.version, cidr
                )
            )
        canonical = (
            str(network)
            if network.version == 4
            else canonical_ipv6_cidr(network)  # type: ignore[arg-type]
        )
        if canonical != cidr:
            raise RegistryError(
                "{} record {} has non-canonical CIDR {!r}; use {!r}".format(
                    spec.name, record_number, cidr, canonical
                )
            )
        networks.append(network)
    return tuple(networks)


def canonical_ipv6_cidr(network: ipaddress.IPv6Network) -> str:
    """Render an RFC 5952-style IPv6 CIDR without IPv4-mapped notation.

    The IANA registry expresses the IPv4-mapped prefix as ::ffff:0:0/96.
    Python's standard formatter changes that valid canonical hextet form into
    dotted IPv4 notation, so the parser uses this small explicit formatter.
    """

    packed = network.network_address.packed
    segments = [
        int.from_bytes(packed[index : index + 2], byteorder="big")
        for index in range(0, len(packed), 2)
    ]
    run_start = -1
    run_length = 0
    candidate_start = 0
    while candidate_start < len(segments):
        if segments[candidate_start] != 0:
            candidate_start += 1
            continue
        candidate_end = candidate_start
        while candidate_end < len(segments) and segments[candidate_end] == 0:
            candidate_end += 1
        candidate_length = candidate_end - candidate_start
        if candidate_length >= 2 and candidate_length > run_length:
            run_start = candidate_start
            run_length = candidate_length
        candidate_start = candidate_end

    rendered = ["{:x}".format(segment) for segment in segments]
    if run_start < 0:
        address = ":".join(rendered)
    else:
        before = ":".join(rendered[:run_start])
        after = ":".join(rendered[run_start + run_length :])
        if before and after:
            address = before + "::" + after
        elif before:
            address = before + "::"
        elif after:
            address = "::" + after
        else:
            address = "::"
    return "{}/{}".format(address, network.prefixlen)


def parse_registry(data: bytes, spec: SourceSpec) -> Registry:
    """Parse one exact IANA XML registry without accepting schema drift."""

    try:
        root = ElementTree.fromstring(data)
    except ElementTree.ParseError as error:
        raise RegistryError("{} registry is not well-formed XML".format(spec.name)) from error

    if root.tag != qualified("registry"):
        raise RegistryError("{} root must be IANA registry XML".format(spec.name))
    if root.attrib.get("id") != spec.root_id:
        raise RegistryError(
            "{} root id must be {!r}, got {!r}".format(
                spec.name, spec.root_id, root.attrib.get("id")
            )
        )

    updated = parse_updated(root, spec)
    inner_registries = [child for child in root if child.tag == qualified("registry")]
    if len(inner_registries) != 1 or inner_registries[0].attrib.get("id") != spec.inner_id:
        raise RegistryError(
            "{} must contain exactly inner registry id {!r}".format(spec.name, spec.inner_id)
        )

    records: List[Record] = []
    row_count = 0
    seen = set()
    for element in inner_registries[0]:
        if element.tag != qualified("record"):
            continue
        row_count += 1
        record_number = row_count
        fields = field_map(element, spec, record_number)
        forwardable = parse_flag(
            fields["forwardable"],
            "{} record {} field forwardable".format(spec.name, record_number),
        )
        for network in parse_networks(fields["address"], spec, record_number):
            if network in seen:
                raise RegistryError(
                    "{} record {} repeats CIDR {}".format(spec.name, record_number, network)
                )
            seen.add(network)
            records.append(Record(network=network, forwardable=forwardable))

    if not records:
        raise RegistryError("{} contains no registry records".format(spec.name))
    return Registry(
        spec=spec,
        updated=updated,
        sha256=sha256(data),
        row_count=row_count,
        records=tuple(records),
    )


def read_committed_registries(repo: Path = ROOT) -> Tuple[Registry, Registry]:
    """Read and validate the version-pinned fixtures, without writing anything."""

    registries: List[Registry] = []
    for spec in source_specs(repo):
        try:
            data = spec.fixture.read_bytes()
        except OSError as error:
            raise RegistryError(
                "cannot read committed {} fixture {}: {}".format(
                    spec.name, spec.fixture, error
                )
            ) from error
        registries.append(parse_registry(data, spec))
    return tuple(registries)  # type: ignore[return-value]


def canonical_blacklist(records: Iterable[Record]) -> Tuple[Network, ...]:
    """Return disjoint False regions after the most-specific forwarding overlay.

    None is an explicit unknown state. It replaces a less-specific False parent
    just like True does; a later, still more-specific False record can
    reintroduce a blocked region.
    """

    ordered = sorted(
        records,
        key=lambda record: (
            record.network.version,
            record.network.prefixlen,
            int(record.network.network_address),
        ),
    )
    pieces: List[Tuple[Network, Optional[bool]]] = []
    for record in ordered:
        next_pieces: List[Tuple[Network, Optional[bool]]] = []
        for existing, state in pieces:
            if not existing.overlaps(record.network):
                next_pieces.append((existing, state))
                continue
            if not record.network.subnet_of(existing):
                raise RegistryError(
                    "overlapping CIDRs do not have a strict most-specific order: {} and {}".format(
                        existing, record.network
                    )
                )
            next_pieces.extend(
                (remaining, state) for remaining in existing.address_exclude(record.network)
            )
        next_pieces.append((record.network, record.forwardable))
        pieces = next_pieces

    false_networks = [network for network, state in pieces if state is False]
    collapsed = list(ipaddress.collapse_addresses(false_networks))
    collapsed.sort(
        key=lambda network: (
            network.version,
            int(network.network_address),
            network.prefixlen,
        )
    )
    return tuple(collapsed)


def split_registries(
    registries: Sequence[Registry],
) -> Tuple[Registry, Registry, Tuple[Network, ...], Tuple[Network, ...]]:
    """Return deterministic aggregate and forwarding data for both families."""

    by_family = {registry.spec.family: registry for registry in registries}
    if set(by_family) != {4, 6}:
        raise RegistryError("generation requires exactly one IPv4 and one IPv6 registry")
    ipv4 = by_family[4]
    ipv6 = by_family[6]
    return (
        ipv4,
        ipv6,
        canonical_blacklist(ipv4.records),
        canonical_blacklist(ipv6.records),
    )


def ipv4_expression(network: ipaddress.IPv4Network) -> str:
    """Render a const IPv4 network expression accepted by the crate's MSRV."""

    octets = ", ".join(str(octet) for octet in network.network_address.packed)
    return "Ipv4Net::new_assert(Ipv4Addr::new({}), {})".format(octets, network.prefixlen)


def ipv6_expression(network: ipaddress.IPv6Network) -> str:
    """Render a multiline const IPv6 network expression."""

    packed = network.network_address.packed
    segments = [
        "0x{:04x}".format(
            int.from_bytes(packed[index : index + 2], byteorder="big")
        )
        for index in range(0, len(packed), 2)
    ]
    return "Ipv6Net::new_assert(\n  Ipv6Addr::new(\n    {},\n  ),\n  {},\n)".format(
        ", ".join(segments), network.prefixlen
    )


def render_constants(
    prefix: str, networks: Sequence[Network]
) -> Tuple[List[str], List[str]]:
    """Render typed private constants and return their names."""

    lines: List[str] = []
    names: List[str] = []
    for index, network in enumerate(networks, start=1):
        name = "{}_{:03d}".format(prefix, index)
        names.append(name)
        if network.version == 4:
            expression = ipv4_expression(network)  # type: ignore[arg-type]
            line = "const {}: Ipv4Net = {};".format(name, expression)
            lines.append(
                line
                if len(line) <= 100
                else "const {}: Ipv4Net =\n  {};".format(name, expression)
            )
        else:
            lines.append(
                "const {}: Ipv6Net = {};".format(
                    name, ipv6_expression(network)  # type: ignore[arg-type]
                )
            )
    return lines, names


def render_typed_array(name: str, network_type: str, names: Sequence[str]) -> List[str]:
    """Render a stable, intentionally vertical typed slice."""

    lines = ["pub(super) const {}: &[{}] = &[".format(name, network_type)]
    lines.extend("  {},".format(item) for item in names)
    lines.append("];")
    return lines


def render_ip_array(
    name: str, ipv4_names: Sequence[str], ipv6_names: Sequence[str]
) -> List[str]:
    """Render the enum-typed slice in the same family order as typed slices."""

    lines = ["pub(super) const {}: &[IpNet] = &[".format(name)]
    lines.extend("  IpNet::V4({}),".format(item) for item in ipv4_names)
    lines.extend("  IpNet::V6({}),".format(item) for item in ipv6_names)
    lines.append("];")
    return lines


def generate_rust(registries: Sequence[Registry]) -> bytes:
    """Generate the private no-std Rust data module deterministically."""

    ipv4, ipv6, ipv4_blacklist, ipv6_blacklist = split_registries(registries)
    ipv4_aggregate = tuple(record.network for record in ipv4.records)
    ipv6_aggregate = tuple(record.network for record in ipv6.records)

    lines = [
        "// This file is generated by scripts/update_iana_registry.py sync.",
        "// Do not edit it by hand; see data/iana/README.md for the update workflow.",
        "//",
        "// IPv4 source: {}".format(ipv4.spec.url),
        "// IPv4 source updated: {}".format(ipv4.updated),
        "// IPv4 source SHA-256: {}".format(ipv4.sha256),
        "// IPv6 source: {}".format(ipv6.spec.url),
        "// IPv6 source updated: {}".format(ipv6.updated),
        "// IPv6 source SHA-256: {}".format(ipv6.sha256),
        "//",
        "// Snapshot rows: IPv4 {}; IPv6 {}. Aggregate prefixes: IPv4 {}; IPv6 {}.".format(
            ipv4.row_count,
            ipv6.row_count,
            len(ipv4_aggregate),
            len(ipv6_aggregate),
        ),
        "// Canonical forwarding blacklist prefixes: IPv4 {}; IPv6 {}.".format(
            len(ipv4_blacklist), len(ipv6_blacklist)
        ),
        "",
        "use core::net::{Ipv4Addr, Ipv6Addr};",
        "",
        "use ipnet::{IpNet, Ipv4Net, Ipv6Net};",
        "",
    ]

    groups = (
        ("RFC6890_IPV4", ipv4_aggregate),
        ("RFC6890_IPV6", ipv6_aggregate),
        ("FORWARDING_BLACKLIST_IPV4", ipv4_blacklist),
        ("FORWARDING_BLACKLIST_IPV6", ipv6_blacklist),
    )
    names: Dict[str, List[str]] = {}
    for prefix, networks in groups:
        constants, group_names = render_constants(prefix, networks)
        lines.extend(constants)
        lines.append("")
        names[prefix] = group_names

    lines.extend(
        render_typed_array("RFC6890_IPV4_NETS", "Ipv4Net", names["RFC6890_IPV4"])
    )
    lines.append("")
    lines.extend(
        render_typed_array("RFC6890_IPV6_NETS", "Ipv6Net", names["RFC6890_IPV6"])
    )
    lines.append("")
    lines.extend(
        render_ip_array(
            "RFC6890_IP_NETS", names["RFC6890_IPV4"], names["RFC6890_IPV6"]
        )
    )
    lines.append("")
    lines.extend(
        render_typed_array(
            "FORWARDING_BLACKLIST_IPV4_NETS",
            "Ipv4Net",
            names["FORWARDING_BLACKLIST_IPV4"],
        )
    )
    lines.append("")
    lines.extend(
        render_typed_array(
            "FORWARDING_BLACKLIST_IPV6_NETS",
            "Ipv6Net",
            names["FORWARDING_BLACKLIST_IPV6"],
        )
    )
    lines.append("")
    lines.extend(
        render_ip_array(
            "FORWARDING_BLACKLIST_IP_NETS",
            names["FORWARDING_BLACKLIST_IPV4"],
            names["FORWARDING_BLACKLIST_IPV6"],
        )
    )
    lines.append("")
    return "\n".join(lines).encode("utf-8")


def generate_provenance(registries: Sequence[Registry]) -> bytes:
    """Generate the checked-in human-readable snapshot provenance deterministically."""

    ipv4, ipv6, _, _ = split_registries(registries)
    lines = [
        "<!-- This file is generated by scripts/update_iana_registry.py sync. Do not edit it by hand. -->",
        "",
        "# IANA Special-Purpose Address Space snapshots",
        "",
        "These XML files are the version-pinned, checked-in source data for the",
        "compatibility RFC6890 aggregate and FORWARDING_BLACKLIST pseudo-RFC. The",
        "published crate never fetches them at build time or runtime.",
        "",
        "| Family | Source | XML updated | SHA-256 |",
        "| --- | --- | --- | --- |",
        "| {0.spec.name} | {0.spec.url} | {0.updated} | {0.sha256} |".format(ipv4),
        "| {0.spec.name} | {0.spec.url} | {0.updated} | {0.sha256} |".format(ipv6),
        "",
        "Maintainers use the stdlib-only updater from the repository root:",
        "",
        "    python3 scripts/update_iana_registry.py check",
        "    python3 scripts/update_iana_registry.py check-upstream",
        "    python3 scripts/update_iana_registry.py sync",
        "",
        "check is offline and compares in-memory generated Rust and provenance",
        "with their committed files. check-upstream only fetches and compares source",
        "bytes; it never edits the checkout. sync validates both upstream files and",
        "generates all output before atomically replacing each individual destination.",
        "Review and release the resulting checked-in snapshot deliberately; this",
        "repository does not schedule automatic sync commits, branches, or pull",
        "requests.",
    ]
    return ("\n".join(lines) + "\n").encode("utf-8")


def fetch_source(url: str) -> bytes:
    """Fetch a hardcoded IANA HTTPS source without writing to disk."""

    request = Request(url, headers={"User-Agent": "iprfc-registry-updater/1"})
    try:
        with urlopen(request, timeout=30) as response:
            final_url = response.geturl()
            parsed = urlparse(final_url)
            if parsed.scheme != "https" or parsed.hostname != "www.iana.org":
                raise RegistryError(
                    "IANA source redirected outside https://www.iana.org: {}".format(
                        final_url
                    )
                )
            return response.read()
    except RegistryError:
        raise
    except OSError as error:
        raise RegistryError("failed to fetch {}: {}".format(url, error)) from error


def collect_upstream_registries(
    repo: Path, fetcher: Fetcher
) -> Tuple[Tuple[Registry, Registry], Dict[int, bytes]]:
    """Fetch and validate both registries before a caller can write anything."""

    registries: List[Registry] = []
    source_bytes: Dict[int, bytes] = {}
    errors: List[str] = []
    for spec in source_specs(repo):
        try:
            data = fetcher(spec.url)
            registries.append(parse_registry(data, spec))
            source_bytes[spec.family] = data
        except (OSError, RegistryError) as error:
            errors.append("{}: {}".format(spec.name, error))
    if errors:
        raise RegistryError("upstream validation failed: {}".format("; ".join(errors)))
    return tuple(registries), source_bytes  # type: ignore[return-value]


def generated_path(repo: Path = ROOT) -> Path:
    """Return the checked-in generated Rust destination."""

    return Path(repo) / GENERATED_RELATIVE


def provenance_path(repo: Path = ROOT) -> Path:
    """Return the checked-in generated provenance destination."""

    return Path(repo) / PROVENANCE_RELATIVE


def check(repo: Path = ROOT) -> int:
    """Regenerate from local fixtures in memory and report stale output files."""

    registries = read_committed_registries(repo)
    expected_outputs = (
        (generated_path(repo), generate_rust(registries), "generated Rust data"),
        (
            provenance_path(repo),
            generate_provenance(registries),
            "generated IANA provenance",
        ),
    )
    stale = False
    for destination, expected, description in expected_outputs:
        try:
            actual = destination.read_bytes()
        except OSError as error:
            print(
                "{} is missing or unreadable at {}: {}".format(
                    description, destination, error
                ),
                file=sys.stderr,
            )
            stale = True
            continue
        if actual != expected:
            print("{} is stale: {}".format(description, destination), file=sys.stderr)
            stale = True
    if stale:
        print("Run: python3 scripts/update_iana_registry.py sync", file=sys.stderr)
        return 1
    print("IANA registry fixtures and generated Rust data and provenance are current.")
    return 0


def check_upstream(repo: Path = ROOT, fetcher: Fetcher = fetch_source) -> int:
    """Read and compare upstream source bytes without modifying any files."""

    committed = read_committed_registries(repo)
    upstream, upstream_bytes = collect_upstream_registries(repo, fetcher)
    committed_by_family = {registry.spec.family: registry for registry in committed}
    upstream_by_family = {registry.spec.family: registry for registry in upstream}
    drift = False
    for family in (4, 6):
        old = committed_by_family[family]
        new = upstream_by_family[family]
        old_bytes = old.spec.fixture.read_bytes()
        if old_bytes == upstream_bytes[family]:
            continue
        drift = True
        print("{} IANA registry drift detected:".format(old.spec.name), file=sys.stderr)
        print("  committed updated: {}".format(old.updated), file=sys.stderr)
        print("  committed SHA-256: {}".format(old.sha256), file=sys.stderr)
        print("  upstream updated: {}".format(new.updated), file=sys.stderr)
        print("  upstream SHA-256: {}".format(new.sha256), file=sys.stderr)
        print("  source: {}".format(old.spec.url), file=sys.stderr)
    if drift:
        print(
            "Run: python3 scripts/update_iana_registry.py sync, review the generated diff, "
            "then release from the checked-in snapshot.",
            file=sys.stderr,
        )
        return 1
    print("Checked-in IANA registry fixtures match upstream exactly.")
    return 0


def atomic_replace_all(replacements: Sequence[Tuple[Path, bytes]]) -> None:
    """Stage all files locally, then replace each destination atomically.

    Each individual destination is atomically replaced only after every fetch,
    parse, generation, and temporary-file write succeeded. Filesystems do not
    provide a portable transaction across the separate destination paths.
    """

    staged: List[Tuple[Path, Path]] = []
    try:
        for destination, contents in replacements:
            destination.parent.mkdir(parents=True, exist_ok=True)
            mode = 0o644
            try:
                mode = destination.stat().st_mode & 0o777
            except FileNotFoundError:
                pass
            descriptor, temporary_name = tempfile.mkstemp(
                prefix=".{}.".format(destination.name),
                suffix=".tmp",
                dir=str(destination.parent),
            )
            temporary = Path(temporary_name)
            try:
                with os.fdopen(descriptor, "wb") as output:
                    output.write(contents)
                    output.flush()
                    os.fsync(output.fileno())
                os.chmod(str(temporary), mode)
            except BaseException:
                temporary.unlink(missing_ok=True)
                raise
            staged.append((temporary, destination))

        for temporary, destination in staged:
            os.replace(str(temporary), str(destination))
        staged.clear()
    finally:
        for temporary, _ in staged:
            temporary.unlink(missing_ok=True)


def sync(repo: Path = ROOT, fetcher: Fetcher = fetch_source) -> int:
    """Fetch, validate, generate, stage, and replace the complete snapshot."""

    repo = Path(repo)
    registries, source_bytes = collect_upstream_registries(repo, fetcher)
    generated = generate_rust(registries)
    provenance = generate_provenance(registries)
    replacements: List[Tuple[Path, bytes]] = []
    for spec in source_specs(repo):
        replacements.append((spec.fixture, source_bytes[spec.family]))
    replacements.append((generated_path(repo), generated))
    replacements.append((provenance_path(repo), provenance))
    atomic_replace_all(replacements)
    ipv4, ipv6, ipv4_blacklist, ipv6_blacklist = split_registries(registries)
    print(
        "Synced IANA registries: {} IPv4 rows / {} IPv4 prefixes, {} IPv6 rows / "
        "{} IPv6 prefixes; forwarding blacklist {} IPv4 / {} IPv6 CIDRs.".format(
            ipv4.row_count,
            len(ipv4.records),
            ipv6.row_count,
            len(ipv6.records),
            len(ipv4_blacklist),
            len(ipv6_blacklist),
        )
    )
    return 0


def main(argv: Optional[Sequence[str]] = None) -> int:
    """Run one explicit maintainer command."""

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("check", "check-upstream", "sync"))
    arguments = parser.parse_args(argv)
    try:
        if arguments.command == "check":
            return check()
        if arguments.command == "check-upstream":
            return check_upstream()
        return sync()
    except RegistryError as error:
        print("registry update error: {}".format(error), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
