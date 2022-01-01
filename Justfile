# We can't do conditional setting of the shell. Oh well. Only I get to use this
# then
set shell := ["pwsh", "-NoProfile", "-NoLogo", "-Command"]

rustflags := "-Zinstrument-coverage -Zprofile"
rustdocflags := rustflags + " -Z unstable-options --persist-doctests target/coverage"
coverage := join(justfile_directory(), "target/coverage/outcome-%p-%m.profraw")
pwd := justfile_directory()

default: fmt test

@build:
  cargo build --no-default-features
  cargo build --all-features

@clear-reports:
  -Remove-Item -Recurse -Force -ErrorAction SilentlyContinue {{join(justfile_directory(), "target/coverage")}}

@collect type="lcov" $RUSTUP_TOOLCHAIN="nightly": clear-reports coverage
  grcov ${PWD}/target/coverage/ \
    --source-dir {{pwd}} \
    --output-type {{type}} \
    --output-path {{ if type == "lcov" { "coverage.info" } else { "target/collected" } }} \
    --commit-sha $(git rev-parse HEAD) \
    --binary-path {{pwd}}/target/coverage \
    --prefix-dir {{pwd}} \
    --filter covered \
    --branch \
    --llvm \
    --ignore-not-existing \
    --guess-directory-when-missing

coverage $RUSTFLAGS=rustflags $RUSTDOCFLAGS=rustdocflags $LLVM_PROFILE_FILE=coverage:
  cargo +nightly test --no-default-features --doc --quiet --profile=coverage
  cargo +nightly test --no-default-features --lib --quiet --profile=coverage
  cargo +nightly test --all-features --doc --quiet --profile=coverage
  cargo +nightly test --all-features --lib --quiet --profile=coverage

@test: coverage
  cargo test --no-default-features --doc --quiet
  cargo test --no-default-features --lib --quiet
  cargo test --all-features --doc --quiet
  cargo test --all-features --lib --quiet

@fmt:
  @cargo fmt

docs *ARGS:
  @cargo +nightly doc {{ARGS}} --all-features

@check:
  cargo +nightly clippy --all-features -- -D warnings
  @cargo clippy --all-features -- -D warnings

@commit: fmt check docs test
  git commit -v --signoff
