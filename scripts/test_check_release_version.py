"""Tests for the release metadata gate used by trusted publishing."""

from datetime import date, timedelta
import hashlib
from pathlib import Path
import sys
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

import check_release_version as checker


README_BOUNDARY = "\n\n## Installation\n"
PINNED_README_HEADER_SHA256 = (
    "94c7512ef703fca66a8a49fd4f676425175ebeb68e0bef8a5365db412eb893fb"
)
README_HEADER = (ROOT / "README.md").read_text(encoding="utf-8").split(
    README_BOUNDARY, 1
)[0]


def readme_for_version(version):
    """Build a README fixture with the repository's pinned header."""

    major = version.split(".", 1)[0]
    body = f'''```toml
[dependencies]
iprfc = "{major}"
```

```toml
[dependencies]
iprfc = {{ version = "{major}", default-features = false }}
```
'''

    return README_HEADER + README_BOUNDARY + body


def changelog_for_version(version, release_date):
    """Build a dated release fixture without hardcoding a package version."""

    date_text = "{} {}, {}".format(
        release_date.strftime("%B"), release_date.day, release_date.year
    )
    return f'''# Changelog

## Unreleased

## Released

## {version} ({date_text})

- Stable release.
'''


def next_patch(version):
    """Return a different stable version for wrong-tag tests."""

    major, minor, patch = (int(part) for part in version.split("."))
    return "{}.{}.{}".format(major, minor, patch + 1)


with (ROOT / "Cargo.toml").open("rb") as manifest:
    VERSION = tomllib.load(manifest)["package"]["version"]
MAJOR = VERSION.split(".", 1)[0]
RELEASE_DATE = date.today()
README = readme_for_version(VERSION)
README_BODY = README.split(README_BOUNDARY, 1)[1]
CHANGELOG = changelog_for_version(VERSION, RELEASE_DATE)


class ReleaseMetadataTests(unittest.TestCase):
    """Exercise accepted metadata and every security-relevant rejection."""

    def test_fixture_uses_the_known_pinned_readme_header(self):
        self.assertEqual(len(README_HEADER.splitlines()), 20)
        self.assertEqual(
            hashlib.sha256(README_HEADER.encode("utf-8")).hexdigest(),
            PINNED_README_HEADER_SHA256,
        )
        self.assertEqual(checker.README_HEADER_SHA256, PINNED_README_HEADER_SHA256)
        self.assertEqual(README.count(README_BOUNDARY), 1)

    def test_current_repository_is_consistent(self):
        checker.check_repository()

    def test_accepts_exact_version_tag_and_release_metadata(self):
        checker.validate_release_metadata(VERSION, README, CHANGELOG, "v" + VERSION)

    def test_accepts_future_stable_version_and_release_date(self):
        major, _, _ = VERSION.partition(".")
        future_version = "{}.0.0".format(int(major) + 1)
        future_date = date.today() + timedelta(days=365)
        checker.validate_release_metadata(
            future_version,
            readme_for_version(future_version),
            changelog_for_version(future_version, future_date),
            "v" + future_version,
        )

    def test_nonrelease_check_allows_unreleased_work(self):
        checker.validate_release_metadata(
            VERSION,
            README,
            CHANGELOG.replace(
                "## Unreleased\n\n## Released",
                "## Unreleased\n\n- Work in progress.\n\n## Released",
            ),
        )

    def test_rejects_wrong_tag(self):
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION, README, CHANGELOG, "v" + next_patch(VERSION)
            )

    def test_rejects_prerelease_version(self):
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(VERSION + "-rc.1", README, CHANGELOG)

    def test_rejects_stale_readme_requirement(self):
        stale_major = str(int(MAJOR) + 1)
        stale_readme = README.replace(
            'iprfc = "{}"'.format(MAJOR), 'iprfc = "{}"'.format(stale_major), 1
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(VERSION, stale_readme, CHANGELOG)

    def test_rejects_modified_pinned_readme_header(self):
        modified_header = README_HEADER.replace("</div>", "</div> changed", 1)
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                modified_header + README_BOUNDARY + README_BODY,
                CHANGELOG,
            )

    def test_rejects_unexpected_html_in_pinned_readme_header(self):
        modified_header = README_HEADER.replace("</div>", "</div><pre>", 1)
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                modified_header + README_BOUNDARY + README_BODY,
                CHANGELOG,
            )

    def test_rejects_readme_examples_hidden_in_html_comment(self):
        hidden = "<!--{}-->\n{}".format(README_BODY, README_BODY)
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + hidden,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_readme_examples_after_unclosed_html_comment(self):
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + "<!-- hidden forever\n" + README_BODY,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_readme_same_line_html_decoy_before_toml_fence(self):
        invalid_body = README_BODY.replace(
            "```toml", "<!--decoy-->```toml", 1
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + invalid_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_readme_fence_with_literal_info_suffix(self):
        invalid_body = README_BODY.replace(
            "```toml", "```toml<!-- literalinfo -->", 1
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + invalid_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_html_comment_syntax_inside_toml_fence(self):
        invalid_body = README_BODY.replace(
            'iprfc = "{}"'.format(MAJOR),
            'iprfc <!-- visible literal code -->= "{}"'.format(MAJOR),
            1,
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + invalid_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_iprfc_assignment_outside_dependencies_table(self):
        misplaced_body = f'''```toml
[dependencies]
other = "{MAJOR}"

[package.metadata]
iprfc = "{MAJOR}"
```

```toml
[package.metadata]
iprfc = {{ version = "{MAJOR}", default-features = false }}
```
'''
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + misplaced_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_dependency_source_and_package_extras(self):
        extras = {
            "package": '"not-iprfc"',
            "path": '"../not-registry"',
            "git": '"https://example.invalid/not-iprfc.git"',
            "registry": '"private"',
            "workspace": "true",
        }
        target = 'iprfc = {{ version = "{}", default-features = false }}'.format(
            MAJOR
        )
        for key, value in extras.items():
            with self.subTest(extra=key):
                invalid_body = README_BODY.replace(
                    target,
                    'iprfc = {{ version = "{}", default-features = false, {} = {} }}'.format(
                        MAJOR, key, value
                    ),
                    1,
                )
                with self.assertRaises(checker.ReleaseCheckError):
                    checker.validate_release_metadata(
                        VERSION,
                        README_HEADER + README_BOUNDARY + invalid_body,
                        CHANGELOG,
                        "v" + VERSION,
                    )

    def test_rejects_replace_override_document(self):
        replace_document = f'''```toml
[replace]
"iprfc:{VERSION}" = {{ path = "../shadow" }}
```
'''
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + README_BODY + replace_document,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_patch_crates_io_document(self):
        patch_document = '''```toml
[patch.crates-io]
shadow = { package = "iprfc", path = "../shadow" }
```
'''
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + README_BODY + patch_document,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_shadow_dependency_document(self):
        shadow_document = '''```toml
[dependencies]
shadow = { package = "iprfc", path = "../shadow" }
```
'''
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + README_BODY + shadow_document,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_numeric_default_features(self):
        invalid_body = README_BODY.replace(
            "default-features = false", "default-features = 0", 1
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + invalid_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_raw_html_around_readme_and_changelog(self):
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + "<pre>\n" + README_BODY + "</pre>",
                CHANGELOG,
                "v" + VERSION,
            )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README,
                "<pre>\n" + CHANGELOG + "\n</pre>",
                "v" + VERSION,
            )

    def test_rejects_changelog_heading_prefixed_by_html_comment(self):
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README,
                CHANGELOG.replace("## Unreleased", "<!--x-->## Unreleased", 1),
                "v" + VERSION,
            )

    def test_rejects_dependency_lines_inside_toml_multiline_string(self):
        string_decoys_body = f'''```toml
[dependencies]
example = """
iprfc = "{MAJOR}"
iprfc = {{ version = "{MAJOR}", default-features = false }}
"""
```
'''
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + string_decoys_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_four_space_indented_pseudo_fences(self):
        indented_body = "\n".join(
            "    " + line for line in README_BODY.splitlines()
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README_HEADER + README_BOUNDARY + indented_body,
                CHANGELOG,
                "v" + VERSION,
            )

    def test_rejects_candidate_changelog_heading(self):
        release_heading = "## {} ({})".format(
            VERSION,
            "{} {}, {}".format(
                RELEASE_DATE.strftime("%B"), RELEASE_DATE.day, RELEASE_DATE.year
            ),
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README,
                CHANGELOG.replace(
                    release_heading, "## {} (release candidate)".format(VERSION)
                ),
                "v" + VERSION,
            )

    def test_rejects_changelog_heading_inside_fence(self):
        release_heading = "## {} ({})".format(
            VERSION,
            "{} {}, {}".format(
                RELEASE_DATE.strftime("%B"), RELEASE_DATE.day, RELEASE_DATE.year
            ),
        )
        fenced = CHANGELOG.replace(
            release_heading, "~~~markdown\n{}\n~~~".format(release_heading)
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(VERSION, README, fenced, "v" + VERSION)

    def test_rejects_release_outside_released_section(self):
        release_heading = "## {} ({})".format(
            VERSION,
            "{} {}, {}".format(
                RELEASE_DATE.strftime("%B"), RELEASE_DATE.day, RELEASE_DATE.year
            ),
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README,
                CHANGELOG.replace(
                    "## Released\n\n" + release_heading,
                    release_heading + "\n\n## Released",
                ),
                "v" + VERSION,
            )

    def test_rejects_nonempty_unreleased_section(self):
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(
                VERSION,
                README,
                CHANGELOG.replace(
                    "## Unreleased\n\n## Released",
                    "## Unreleased\n\n- Not released.\n\n## Released",
                ),
                "v" + VERSION,
            )

    def test_rejects_fenced_content_in_unreleased_section(self):
        pending = CHANGELOG.replace(
            "## Unreleased\n\n## Released",
            "## Unreleased\n\n```text\npending\n```\n\n## Released",
        )
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(VERSION, README, pending, "v" + VERSION)

    def test_rejects_four_space_indented_pseudo_headings(self):
        indented = CHANGELOG.replace("## Unreleased", "    ## Unreleased", 1)
        with self.assertRaises(checker.ReleaseCheckError):
            checker.validate_release_metadata(VERSION, README, indented, "v" + VERSION)


if __name__ == "__main__":
    unittest.main()
