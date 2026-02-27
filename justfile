set quiet := true
set shell := ["bash", "-cu"]

default:
  @just --list

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

test:
  cargo test --locked --all-features -- --test-threads=1

doc-test:
  cargo test --doc --locked --all-features

typos:
  typos

coverage:
  cargo tarpaulin --engine llvm --locked --all-features --ignore-tests --timeout 120 --out Html --out Xml

ci: fmt-check clippy test doc-test

publish-dry-run:
  cargo publish --locked --dry-run
