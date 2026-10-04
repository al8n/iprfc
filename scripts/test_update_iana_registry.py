"""stdlib unittest coverage for the IANA registry updater."""

import contextlib
import io
import ipaddress
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

import update_iana_registry as updater


def record_xml(address="192.0.2.0/24", forwardable="False", omitted=()):
    """Build one minimal valid IANA record, with optional required omissions."""

    fields = (
        ("address", address),
        ("name", "Synthetic"),
        ("spec", ""),
        ("allocation", "2025-01"),
        ("source", "False"),
        ("destination", "False"),
        ("forwardable", forwardable),
        ("global", "False"),
        ("reserved", "False"),
    )
    return "<record>{}</record>".format(
        "".join(
            "<{0}>{1}</{0}>".format(name, value)
            for name, value in fields
            if name not in omitted
        )
    )


def registry_xml(
    spec,
    records,
    namespace=updater.IANA_NAMESPACE,
    root_id=None,
    inner_id=None,
    updated="2025-10-09",
):
    """Build a minimal source document for schema rejection tests."""

    return (
        '<?xml version="1.0" encoding="UTF-8"?>'
        '<registry xmlns="{namespace}" id="{root_id}">'
        "<updated>{updated}</updated>"
        '<registry id="{inner_id}">{records}</registry>'
        "</registry>"
    ).format(
        namespace=namespace,
        root_id=root_id or spec.root_id,
        inner_id=inner_id or spec.inner_id,
        updated=updated,
        records=records,
    ).encode("utf-8")


def state_at(records, address):
    """Return the explicit most-specific forwarding state at an address."""

    matching = [record for record in records if address in record.network]
    if not matching:
        return None
    return max(matching, key=lambda record: record.network.prefixlen).forwardable


def contains_address(networks, address):
    """Return whether a canonical blacklist covers an address."""

    return any(address in network for network in networks)


class RegistryParserTests(unittest.TestCase):
    """Validate strict parser behavior with compact synthetic IANA XML."""

    def setUp(self):
        self.ipv4 = updater.source_specs(ROOT)[0]
        self.ipv6 = updater.source_specs(ROOT)[1]

    def parse(self, document, spec=None):
        return updater.parse_registry(document, spec or self.ipv4)

    def test_comma_split_and_note_xrefs_are_normalized(self):
        document = registry_xml(
            self.ipv4,
            record_xml(
                address=(
                    '192.0.2.0/24 <xref type="note" data="1"/>, '
                    "192.0.3.0/24"
                ),
                forwardable='False <xref type="note" data="2"/>',
            ),
        )
        registry = self.parse(document)
        self.assertEqual(
            [str(record.network) for record in registry.records],
            ["192.0.2.0/24", "192.0.3.0/24"],
        )
        self.assertEqual([record.forwardable for record in registry.records], [False, False])

    def test_unknown_forwardable_is_an_explicit_none(self):
        registry = self.parse(
            registry_xml(self.ipv4, record_xml(forwardable="N/A"))
        )
        self.assertIsNone(registry.records[0].forwardable)

    def test_rejects_namespace_ids_date_family_and_record_schema_drift(self):
        cases = (
            (
                "namespace",
                registry_xml(
                    self.ipv4,
                    record_xml(),
                    namespace="https://example.invalid/iana",
                ),
                self.ipv4,
            ),
            (
                "root id",
                registry_xml(self.ipv4, record_xml(), root_id="wrong"),
                self.ipv4,
            ),
            (
                "inner id",
                registry_xml(self.ipv4, record_xml(), inner_id="wrong"),
                self.ipv4,
            ),
            (
                "updated date",
                registry_xml(self.ipv4, record_xml(), updated="2025-19-40"),
                self.ipv4,
            ),
            (
                "family",
                registry_xml(self.ipv4, record_xml(address="2001:db8::/32")),
                self.ipv4,
            ),
            (
                "noncanonical",
                registry_xml(self.ipv4, record_xml(address="192.0.2.1/24")),
                self.ipv4,
            ),
            (
                "duplicate",
                registry_xml(
                    self.ipv4,
                    record_xml() + record_xml(forwardable="True"),
                ),
                self.ipv4,
            ),
            (
                "missing",
                registry_xml(
                    self.ipv4,
                    record_xml(omitted=("forwardable",)),
                ),
                self.ipv4,
            ),
            (
                "unknown token",
                registry_xml(self.ipv4, record_xml(forwardable="Maybe")),
                self.ipv4,
            ),
        )
        for name, document, spec in cases:
            with self.subTest(name=name):
                with self.assertRaises(updater.RegistryError):
                    self.parse(document, spec)

    def test_accepts_the_registry_ipv4_mapped_ipv6_cidr_spelling(self):
        registry = self.parse(
            registry_xml(
                self.ipv6,
                record_xml(address="::ffff:0:0/96"),
            ),
            self.ipv6,
        )
        self.assertEqual(registry.records[0].network.prefixlen, 96)


class OverlayTests(unittest.TestCase):
    """Exercise True and Unknown holes plus a nested False reintroduction."""

    def test_most_specific_overlay_preserves_nested_false(self):
        records = (
            updater.Record(ipaddress.ip_network("10.0.0.0/8"), False),
            updater.Record(ipaddress.ip_network("10.1.0.0/16"), True),
            updater.Record(ipaddress.ip_network("10.1.1.0/24"), False),
            updater.Record(ipaddress.ip_network("10.2.0.0/16"), None),
        )
        blocks = updater.canonical_blacklist(records)

        expected = {
            "10.0.0.1": True,
            "10.1.0.1": False,
            "10.1.1.1": True,
            "10.2.0.1": False,
        }
        for text, blocked in expected.items():
            with self.subTest(address=text):
                self.assertEqual(
                    contains_address(blocks, ipaddress.ip_address(text)), blocked
                )

        for index, network in enumerate(blocks):
            for later in blocks[index + 1 :]:
                self.assertFalse(network.overlaps(later))


class SnapshotTests(unittest.TestCase):
    """Pin source metadata, generated data, and the public registry contract."""

    @classmethod
    def setUpClass(cls):
        cls.registries = updater.read_committed_registries(ROOT)
        cls.ipv4, cls.ipv6, cls.ipv4_blocks, cls.ipv6_blocks = updater.split_registries(
            cls.registries
        )

    def assert_boundaries_match_oracle(self, records, blocks):
        for record in records:
            for address in (record.network.network_address, record.network.broadcast_address):
                with self.subTest(source_boundary=str(record.network), address=str(address)):
                    self.assertEqual(
                        contains_address(blocks, address),
                        state_at(records, address) is False,
                    )
        for block in blocks:
            for address in (block.network_address, block.broadcast_address):
                with self.subTest(generated_boundary=str(block), address=str(address)):
                    self.assertIs(state_at(records, address), False)
                    self.assertTrue(contains_address(blocks, address))

    def test_exact_snapshot_counts_metadata_and_generated_determinism(self):
        self.assertEqual([registry.row_count for registry in self.registries], [25, 25])
        self.assertEqual([len(registry.records) for registry in self.registries], [26, 25])
        self.assertEqual([len(self.ipv4_blocks), len(self.ipv6_blocks)], [14, 139])
        self.assertEqual(
            [registry.updated for registry in self.registries],
            ["2025-10-09", "2025-10-09"],
        )
        self.assertEqual(
            [registry.sha256 for registry in self.registries],
            [
                "cf24e11f41b7d42c68debe2d18b97cac815084ec413ebb3b244f704028a16f20",
                "c17f4380ba84fb2160dae82ebfd8bd155a5853cfab624ed3a9fd251638a8be02",
            ],
        )
        generated = updater.generate_rust(self.registries)
        self.assertEqual(generated, updater.generate_rust(self.registries))
        self.assertEqual(generated, updater.generated_path(ROOT).read_bytes())
        provenance = updater.generate_provenance(self.registries)
        self.assertEqual(provenance, updater.generate_provenance(self.registries))
        self.assertEqual(provenance, updater.provenance_path(ROOT).read_bytes())

    def test_every_source_and_generated_boundary_matches_the_oracle(self):
        self.assert_boundaries_match_oracle(self.ipv4.records, self.ipv4_blocks)
        self.assert_boundaries_match_oracle(self.ipv6.records, self.ipv6_blocks)

    def test_known_current_registry_exceptions_and_unknown_holes(self):
        cases = (
            (self.ipv4_blocks, "192.0.0.8", True),
            (self.ipv4_blocks, "192.0.0.9", False),
            (self.ipv4_blocks, "192.0.0.10", False),
            (self.ipv4_blocks, "192.0.0.11", True),
            (self.ipv4_blocks, "192.88.99.0", False),
            (self.ipv4_blocks, "192.88.99.2", False),
            (self.ipv6_blocks, "100:0:0:1::", True),
            (self.ipv6_blocks, "2001:1::", True),
            (self.ipv6_blocks, "2001:1::1", False),
            (self.ipv6_blocks, "2001:1::2", False),
            (self.ipv6_blocks, "2001:1::3", False),
            (self.ipv6_blocks, "2001:1::4", True),
            (self.ipv6_blocks, "2001:2::", False),
            (self.ipv6_blocks, "2001:3::", False),
            (self.ipv6_blocks, "2001:4::", True),
            (self.ipv6_blocks, "2001:4:112::", False),
            (self.ipv6_blocks, "2001:10::", False),
            (self.ipv6_blocks, "2001:20::", False),
            (self.ipv6_blocks, "2001:30::", False),
            (self.ipv6_blocks, "3fff::", True),
            (self.ipv6_blocks, "5f00::", False),
        )
        for blocks, address, expected in cases:
            with self.subTest(address=address):
                self.assertEqual(
                    contains_address(blocks, ipaddress.ip_address(address)), expected
                )


class CommandSafetyTests(unittest.TestCase):
    """Check and failed network paths never write checked-in destinations."""

    def file_state(self, repo):
        paths = [updater.generated_path(repo), updater.provenance_path(repo)]
        paths.extend(spec.fixture for spec in updater.source_specs(repo))
        return {path: path.read_bytes() for path in paths}

    def copy_current_snapshot(self, repo, generated=None, provenance=None):
        source_specs = updater.source_specs(ROOT)
        target_specs = updater.source_specs(repo)
        for source, target in zip(source_specs, target_specs):
            target.fixture.parent.mkdir(parents=True, exist_ok=True)
            target.fixture.write_bytes(source.fixture.read_bytes())
        destination = updater.generated_path(repo)
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(
            updater.generated_path(ROOT).read_bytes()
            if generated is None
            else generated
        )
        provenance_destination = updater.provenance_path(repo)
        provenance_destination.parent.mkdir(parents=True, exist_ok=True)
        provenance_destination.write_bytes(
            updater.provenance_path(ROOT).read_bytes()
            if provenance is None
            else provenance
        )

    def test_check_and_check_upstream_do_not_write(self):
        with tempfile.TemporaryDirectory() as temporary:
            repo = Path(temporary)
            self.copy_current_snapshot(repo)
            before = self.file_state(repo)
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(updater.check(repo), 0)

            by_url = {
                spec.url: spec.fixture.read_bytes()
                for spec in updater.source_specs(repo)
            }
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(
                    updater.check_upstream(repo, fetcher=by_url.__getitem__), 0
                )
            self.assertEqual(self.file_state(repo), before)

    def test_stale_provenance_check_and_upstream_drift_do_not_write(self):
        with tempfile.TemporaryDirectory() as temporary:
            repo = Path(temporary)
            self.copy_current_snapshot(repo, provenance=b"stale generated provenance")
            before = self.file_state(repo)
            errors = io.StringIO()
            with contextlib.redirect_stderr(errors):
                self.assertEqual(updater.check(repo), 1)
            self.assertIn("generated IANA provenance is stale", errors.getvalue())

            source_data = {
                spec.url: spec.fixture.read_bytes()
                for spec in updater.source_specs(repo)
            }
            first_url = updater.source_specs(repo)[0].url
            source_data[first_url] = source_data[first_url].replace(
                b"2025-10-09", b"2025-10-10", 1
            )
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(
                    updater.check_upstream(repo, fetcher=source_data.__getitem__), 1
                )
            self.assertEqual(self.file_state(repo), before)

    def test_missing_provenance_check_does_not_write(self):
        with tempfile.TemporaryDirectory() as temporary:
            repo = Path(temporary)
            self.copy_current_snapshot(repo)
            provenance = updater.provenance_path(repo)
            provenance.unlink()
            before = {
                path: path.read_bytes()
                for path in [updater.generated_path(repo)]
                + [spec.fixture for spec in updater.source_specs(repo)]
            }
            errors = io.StringIO()
            with contextlib.redirect_stderr(errors):
                self.assertEqual(updater.check(repo), 1)
            self.assertIn("generated IANA provenance is missing or unreadable", errors.getvalue())
            self.assertFalse(provenance.exists())
            self.assertEqual(
                {
                    path: path.read_bytes()
                    for path in [updater.generated_path(repo)]
                    + [spec.fixture for spec in updater.source_specs(repo)]
                },
                before,
            )

    def test_sync_updates_generated_outputs_for_new_source_metadata(self):
        with tempfile.TemporaryDirectory() as temporary:
            repo = Path(temporary)
            self.copy_current_snapshot(repo)
            rust_before = updater.generated_path(repo).read_bytes()
            provenance_before = updater.provenance_path(repo).read_bytes()
            source_data = {
                spec.url: spec.fixture.read_bytes()
                for spec in updater.source_specs(repo)
            }
            first_url = updater.source_specs(repo)[0].url
            source_data[first_url] = source_data[first_url].replace(
                b"2025-10-09", b"2025-10-10", 1
            )

            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(updater.sync(repo, fetcher=source_data.__getitem__), 0)

            registries = updater.read_committed_registries(repo)
            rust_after = updater.generated_path(repo).read_bytes()
            provenance_after = updater.provenance_path(repo).read_bytes()
            self.assertNotEqual(rust_after, rust_before)
            self.assertNotEqual(provenance_after, provenance_before)
            self.assertEqual(rust_after, updater.generate_rust(registries))
            self.assertEqual(provenance_after, updater.generate_provenance(registries))
            self.assertIn(b"2025-10-10", rust_after)
            self.assertIn(b"2025-10-10", provenance_after)
            self.assertIn(registries[0].sha256.encode("ascii"), rust_after)
            self.assertIn(registries[0].sha256.encode("ascii"), provenance_after)

    def test_failed_sync_fetch_parse_or_generation_writes_nothing(self):
        with tempfile.TemporaryDirectory() as temporary:
            repo = Path(temporary)
            self.copy_current_snapshot(repo, generated=b"existing generated data")
            before = self.file_state(repo)

            def failing_fetcher(url):
                if "ipv4" in url:
                    raise OSError("simulated network failure")
                return b"<not-valid-xml"

            with self.assertRaises(updater.RegistryError):
                updater.sync(repo, fetcher=failing_fetcher)
            self.assertEqual(self.file_state(repo), before)

            source_data = {
                spec.url: spec.fixture.read_bytes()
                for spec in updater.source_specs(repo)
            }
            with mock.patch.object(
                updater,
                "generate_provenance",
                side_effect=updater.RegistryError("simulated generation failure"),
            ):
                with self.assertRaises(updater.RegistryError):
                    updater.sync(repo, fetcher=source_data.__getitem__)
            self.assertEqual(self.file_state(repo), before)


if __name__ == "__main__":
    unittest.main()
