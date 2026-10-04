# Releasing iprfc

Use this checklist to validate a release candidate. A candidate version in
metadata or release notes is not published state; only a successful upload
makes that version published.

## Prepare

1. Start from a clean `main` branch with green CI.
2. Run `python3 scripts/update_iana_registry.py check-upstream`. This check is
   read-only. If it finds drift, run `python3 scripts/update_iana_registry.py sync`
   deliberately, review the generated snapshot changes, and repeat the
   validation from the beginning.
3. Before local validation, check release metadata. The package version in
   `Cargo.toml`, both `README.md` dependency examples, and the `CHANGELOG.md`
   candidate heading must agree on the intended version. Confirm that the
   intended version is not already published on crates.io. A candidate label
   has no release date; replace it with the actual release date only after
   explicit release authorization.

   The release metadata check is intentionally fail-closed and
   repository-specific. It accepts only the exact 20-line README badge header,
   one literal `\n\n## Installation\n` boundary, and the supported top-level
   dependency examples. The header SHA-256 is pinned; a legitimate badge-header
   change requires an explicit checksum update in `scripts/check_release_version.py`
   and its tests. Do not broaden the parser to accommodate unrelated Markdown.
   The two supported TOML example documents contain only `dependencies.iprfc`
   and no other top-level or dependency keys.
   The checker requires Python 3.11 or newer for `tomllib`.

4. Ensure the maintainer prerequisites are available before running the local
   matrix:

   ```sh
   python3 --version  # Python 3.11 or newer is required for tomllib.
   rustup toolchain install stable --component rustfmt --component clippy
   rustup toolchain install 1.81.0
   rustup target add thumbv7em-none-eabihf --toolchain stable
   cargo install cargo-semver-checks --locked
   ```

5. Run the local validation matrix:

   ```sh
   cargo fmt --all -- --check
   RUSTFLAGS=-Dwarnings cargo +stable clippy --all-targets --all-features -- -D warnings
   RUSTFLAGS=-Dwarnings cargo +stable test
   RUSTFLAGS=-Dwarnings cargo +stable test --no-default-features
   RUSTFLAGS=-Dwarnings cargo +stable test --no-default-features --features serde
   RUSTFLAGS=-Dwarnings cargo +stable test --all-features
   RUSTFLAGS=-Dwarnings cargo +1.81.0 check --all-features
   RUSTFLAGS=-Dwarnings cargo +1.81.0 test --all-features --all-targets
   RUSTFLAGS=-Dwarnings cargo +1.81.0 test --all-features --doc
   RUSTFLAGS=-Dwarnings cargo +stable check --target thumbv7em-none-eabihf --no-default-features
   RUSTFLAGS=-Dwarnings cargo +stable check --target thumbv7em-none-eabihf --no-default-features --features serde
   RUSTDOCFLAGS=-Dwarnings cargo +stable doc --all-features --no-deps
   python3 -m unittest scripts/test_check_release_version.py
   PACKAGE_VERSION="$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')"
   python3 scripts/check_release_version.py --tag "v${PACKAGE_VERSION}"
   python3 scripts/update_iana_registry.py check
   python3 -m unittest scripts/test_update_iana_registry.py
   cargo semver-checks --baseline-version 0.2.3 --release-type patch --all-features
   ```

   The metadata-check command derives the tag from the root Cargo package. For
   future releases, replace `0.2.3` with the latest published version.
   Keep the strict patch API check, including `--release-type patch`, even when
   preparing a 1.0 candidate. LLVM coverage and actionlint are verified by
   mandatory green CI and are not repeated in this local matrix.

6. Validate the package that would be uploaded and the publish dry run:

   ```sh
   cargo package --list
   cargo package
   cargo publish --dry-run
   ```

   Packaging and the dry run validate the upload without publishing it.

## Trusted publishing setup

The crates.io Trusted Publisher for this crate must match all four values:

- GitHub owner: `al8n`
- Repository: `iprfc`
- Workflow: `crates.yml`
- Environment: `crates-io`

The GitHub `crates-io` environment must require the intended human reviewer,
allow only `v*` tags, and disable administrator bypass when the repository plan
supports that control.
The workflow receives an ephemeral token only in its publish job and does not
store a crates.io token in repository secrets. This library intentionally does
not commit `Cargo.lock`. The unprivileged verify job generates and verifies an
ephemeral lockfile, then transfers that exact file and checksum to the publish
job. The publish job verifies the checksum and publishes with `--locked`; the
repository lockfile policy remains unchanged.

`id-token: write` applies to the entire publish job, not only the authentication
step. That job therefore contains only immutable-pinned checkout and artifact
actions, exact Rust toolchain installation, fixed identity checks, the
immutable-pinned crates.io authentication action, and the final publish command.
Before authentication, the job installs the exact release and host tuple recorded
by the verify job and confirms the exact `rustc` and `cargo` versions. Rustup
relies on HTTPS and checksums for these downloads, not artifact signatures, so
that network step is part of the accepted job-wide OIDC trust boundary. Cargo's
required publish verification still executes the locked dependency graph while
OIDC permission and the ephemeral registry token are available. This residual
supply-chain exposure is accepted for Cargo's standard verified publish path and
is mitigated by the unprivileged lock/test/package/dry-run job plus the required
human environment review.

## Publish requires authorization

Stop after the dry run until a human explicitly authorizes publishing, tag
creation, and GitHub release creation. Then:

1. Confirm the Trusted Publisher and GitHub environment settings above.
2. From clean, current `main`, create the annotated tag `v<version>` and push
   only that tag. The tag-triggered workflow verifies that the tag matches the
   Cargo version and points to the current `main` commit before requesting an
   OIDC token and publishing.
   From tag creation through successful crates.io publication and checksum
   confirmation, keep `main` at the tagged commit: do not merge or push
   `main`. This freeze also applies while investigating an ambiguous failure or
   querying the registry.
3. Approve the `crates-io` environment deployment after reviewing the workflow
   run and commit identity.
4. If publishing fails, never move or delete the tag, invent another version,
   or blindly republish. Query crates.io first to distinguish a rejected upload
   from a successful upload followed by a polling timeout. Correct the
   configuration and rerun the same tag workflow only after confirming the
   version is absent.
5. Create the GitHub release only after crates.io confirms the version and
   checksum match the verify job's preserved archive. The release and the
   annotated tag must point to the same commit. A checksum mismatch blocks the
   GitHub release and must never trigger another publish attempt.

Pushing the tag happens before publication in this OIDC flow. Validation proves
that a candidate is ready for review; it does not itself publish, tag, or
release the crate.
