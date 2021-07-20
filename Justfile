# We can't do conditional setting of the shell. Oh well. Only I get to use this
# then
set shell := ["pwsh", "-NoProfile", "-NoLogo", "-Command"]

default: fmt test

build:
  @cargo build --no-default-features
  @cargo build --all-features

test:
  @cargo test --no-default-features --quiet --lib
  @cargo test --all-features --quiet

fmt:
  @cargo fmt

docs *ARGS:
  @cargo doc {{ARGS}} --all-features

check:
  @cargo clippy --all-features
