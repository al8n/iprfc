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

4. Ensure the maintainer prerequisites are available before running the local
   matrix:

   ```sh
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
   python3 scripts/update_iana_registry.py check
   python3 -m unittest scripts/test_update_iana_registry.py
   cargo semver-checks --baseline-version 0.2.3 --release-type patch --all-features
   ```

   For future releases, replace `0.2.3` with the latest published version.
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

## Upload requires authorization

Stop after the dry run. Actual publishing, tag creation, and GitHub release
creation each require explicit human authorization. Validation only proves a
candidate is ready for review; it does not publish, tag, or release it.
