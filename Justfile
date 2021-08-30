# We can't do conditional setting of the shell. Oh well. Only I get to use this
# then
set shell := ["pwsh", "-NoProfile", "-NoLogo", "-Command"]


default: fmt test

build:
  @cargo build --no-default-features
  @cargo build --all-features

test:
  @cargo +nightly test --no-default-features --doc --quiet
  @cargo +nightly test --no-default-features --lib --quiet
  @cargo +nightly test --all-features --doc --quiet
  @cargo +nightly test --all-features --lib --quiet
  @cargo test --no-default-features --doc -- --quiet
  @cargo test --no-default-features --lib --quiet
  @cargo test --all-features --doc -- --quiet
  @cargo test --all-features --lib --quiet

fmt:
  @cargo fmt

docs *ARGS:
  @cargo +nightly doc {{ARGS}} --all-features

check:
  @cargo +nightly clippy --all-features -- -D warnings
  @cargo clippy --all-features -- -D warnings

commit: fmt check docs test
  @git commit -v --signoff
