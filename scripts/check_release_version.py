#!/usr/bin/env python3
"""Verify that release-facing metadata agrees with the Cargo package version."""

import argparse
from datetime import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tomllib
from typing import Optional


ROOT = Path(__file__).resolve().parents[1]
STABLE_VERSION = re.compile(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\Z")
README_INSTALLATION_BOUNDARY = "\n\n## Installation\n"
README_HEADER_SHA256 = "94c7512ef703fca66a8a49fd4f676425175ebeb68e0bef8a5365db412eb893fb"
README_FENCES = {"```toml": "toml", "```rust": "rust"}


class ReleaseCheckError(ValueError):
    """Release metadata is inconsistent or unsafe to publish."""


def readme_code_blocks(readme: str):
    """Return code blocks from the repository's constrained README format."""

    if readme.count(README_INSTALLATION_BOUNDARY) != 1:
        raise ReleaseCheckError(
            "README must contain one exact top-level Installation boundary"
        )
    header, body = readme.split(README_INSTALLATION_BOUNDARY, 1)
    header_sha256 = hashlib.sha256(header.encode("utf-8")).hexdigest()
    if header_sha256 != README_HEADER_SHA256:
        raise ReleaseCheckError(
            "README badge header changed; review it and update the pinned digest explicitly"
        )
    if "<" in body or ">" in body:
        raise ReleaseCheckError(
            "README content after Installation must not contain raw HTML or comments"
        )

    blocks = []
    language = None
    block = []
    for line in body.splitlines():
        if language is None:
            if line in README_FENCES:
                language = README_FENCES[line]
                block = []
            elif "```" in line or "~~~" in line:
                raise ReleaseCheckError("README contains an unsupported code fence")
        elif line == "```":
            blocks.append((language, "\n".join(block)))
            language = None
            block = []
        elif "```" in line or "~~~" in line:
            raise ReleaseCheckError("README contains an unsupported code fence")
        else:
            block.append(line)
    if language is not None:
        raise ReleaseCheckError("README contains an unterminated code fence")
    return blocks


def readme_dependencies(readme: str):
    """Return semantic iprfc dependency values from valid README TOML blocks."""

    dependencies = []
    for language, body in readme_code_blocks(readme):
        if language != "toml":
            continue
        try:
            document = tomllib.loads(body)
        except tomllib.TOMLDecodeError as error:
            raise ReleaseCheckError(
                "README contains invalid TOML: {}".format(error)
            ) from error

        if set(document) != {"dependencies"}:
            raise ReleaseCheckError(
                "README TOML examples must contain only a dependencies table"
            )
        table = document["dependencies"]
        if not isinstance(table, dict) or set(table) != {"iprfc"}:
            raise ReleaseCheckError(
                "README dependencies examples must contain only iprfc"
            )
        dependencies.append(table["iprfc"])
    return dependencies


def cargo_package_version(root: Path = ROOT) -> str:
    """Read the root iprfc package version through Cargo metadata."""

    try:
        result = subprocess.run(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            check=True,
            cwd=root,
            capture_output=True,
            text=True,
        )
        metadata = json.loads(result.stdout)
    except (OSError, subprocess.CalledProcessError, json.JSONDecodeError) as error:
        raise ReleaseCheckError("failed to read Cargo metadata: {}".format(error)) from error

    manifest = (root / "Cargo.toml").resolve()
    packages = [
        package
        for package in metadata.get("packages", ())
        if package.get("name") == "iprfc"
        and Path(package.get("manifest_path", "")).resolve() == manifest
    ]
    if len(packages) != 1:
        raise ReleaseCheckError(
            "Cargo metadata must contain exactly one root iprfc package"
        )
    return packages[0]["version"]


def validate_release_metadata(
    version: str, readme: str, changelog: str, tag: Optional[str] = None
) -> None:
    """Validate stable version, tag, README requirements, and release notes."""

    if not STABLE_VERSION.fullmatch(version):
        raise ReleaseCheckError(
            "release version must be stable MAJOR.MINOR.PATCH, got {!r}".format(version)
        )

    expected_tag = "v{}".format(version)
    if tag is not None and tag != expected_tag:
        raise ReleaseCheckError(
            "release tag must be {!r}, got {!r}".format(expected_tag, tag)
        )

    major = version.split(".", 1)[0]
    dependencies = readme_dependencies(readme)
    basic = [value for value in dependencies if value == major]
    no_default = [
        value
        for value in dependencies
        if isinstance(value, dict)
        and set(value) == {"version", "default-features"}
        and value["version"] == major
        and value["default-features"] is False
    ]
    if len(basic) != 1 or len(no_default) != 1 or len(dependencies) != 2:
        raise ReleaseCheckError(
            "README must contain one basic and one no-default iprfc {} dependency example".format(
                major
            )
        )

    if tag is None:
        return

    if "<" in changelog or ">" in changelog:
        raise ReleaseCheckError("CHANGELOG must not contain raw HTML or comments")
    if "```" in changelog or "~~~" in changelog:
        raise ReleaseCheckError("CHANGELOG must not contain fenced code blocks")

    changelog_lines = changelog.splitlines()
    unreleased_matches = [
        index
        for index, line in enumerate(changelog_lines)
        if line == "## Unreleased"
    ]
    released_matches = [
        index
        for index, line in enumerate(changelog_lines)
        if line == "## Released"
    ]
    if len(unreleased_matches) != 1:
        raise ReleaseCheckError("CHANGELOG must contain one Unreleased heading")
    if len(released_matches) != 1:
        raise ReleaseCheckError("CHANGELOG must contain one Released heading")

    heading_pattern = re.compile(
        r"^## {} \(([A-Z][a-z]+ [1-9][0-9]?, [0-9]{{4}})\)$".format(
            re.escape(version)
        )
    )
    matches = [
        (index, match)
        for index, line in enumerate(changelog_lines)
        for match in [heading_pattern.match(line)]
        if match is not None
    ]
    if len(matches) != 1:
        raise ReleaseCheckError(
            "CHANGELOG must contain one dated release heading for {}".format(version)
        )
    try:
        datetime.strptime(matches[0][1].group(1), "%B %d, %Y")
    except ValueError as error:
        raise ReleaseCheckError("CHANGELOG release date is invalid") from error

    unreleased_index = unreleased_matches[0]
    released_index = released_matches[0]
    version_index = matches[0][0]
    if not unreleased_index < released_index < version_index:
        raise ReleaseCheckError(
            "CHANGELOG headings must order Unreleased, Released, then {}".format(
                version
            )
        )
    unreleased_body = changelog_lines[unreleased_index + 1 : released_index]
    if any(line.strip() for line in unreleased_body):
        raise ReleaseCheckError("CHANGELOG Unreleased section must be empty")


def check_repository(root: Path = ROOT, tag: Optional[str] = None) -> str:
    """Validate the checked-out repository and return its package version."""

    version = cargo_package_version(root)
    try:
        readme = (root / "README.md").read_text(encoding="utf-8")
        changelog = (root / "CHANGELOG.md").read_text(encoding="utf-8")
    except OSError as error:
        raise ReleaseCheckError("failed to read release metadata: {}".format(error)) from error
    validate_release_metadata(version, readme, changelog, tag)
    return version


def main() -> int:
    """Run the release metadata check."""

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tag", help="expected release tag including its leading v")
    arguments = parser.parse_args()
    try:
        version = check_repository(tag=arguments.tag)
    except ReleaseCheckError as error:
        print("release metadata check failed: {}".format(error), file=sys.stderr)
        return 1
    print("release metadata is consistent for {}".format(version))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
