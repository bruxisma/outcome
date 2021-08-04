# Overview

[`Outcome<S, M, F>`][`Outcome`] is an augmentation of the [`Result`] type
found in the Rust standard library.

It is an enum with the variants
 - [`Success(S)`], representing success and containing a value
 - [`Mistake(M)`], representing an optionally *retryable error* and
    containing a value
 - [`Failure(F)`], representing failure and containing a value.

```rust
enum Outcome<S, M, F> {
  Success(S),
  Mistake(M),
  Failure(F),
}
```

[`Outcome`] is an *augmentation* to [`Result`]. It adds a third state to
the "success or failure" dichotomy that [`Result<T, E>`][`Result`] models.
This third state is that of a *soft* or *retryable* error. A *retryable*
error is one where an operation might not have succeeded, either due to
other operations (e.g., a disk read or write not completing),
misconfiguration (e.g., forgetting to set a specific flag before calling a
function), or busy resources (e.g., attempting to lock an audio, video, or
database resource).

```rust
use outcome::prelude::*;

#[derive(Debug, PartialEq)]
enum Version { V1, V2 }

#[derive(Debug, PartialEq)]
struct EmptyInput;

fn parse_version(header: &[u8]) -> Outcome<Version, EmptyInput, &'static str> {
  match header.get(0) {
    None => Mistake(EmptyInput),
    Some(&1) => Success(Version::V1),
    Some(&2) => Success(Version::V2),
    Some(_) => Failure("invalid or unknown version"),
  }
}

let version = parse_version(&[]);
assert_eq!(version, Mistake(EmptyInput));
```

# Usage

At this time, the name `outcome` is already taken on [crates.io]. As
[crates.io] does not yet support namespaces or collections, we've had to
take a *unique* approach to still publish the crate. To do this, we've
generated a `UUIDv5` string via python:

```python
from uuid import *
print(uuid5(uuid5(NAMESPACE_DNS, "occult.work"), "outcome"))
```

This *should* generate the string `46f94afc-026f-5511-9d7e-7d1fd495fb5c`.
Thus the dependency in your `Cargo.toml` will look something like:

```toml
[dependencies]
outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c = "*"
```

However, the exported library is still named `outcome`, so importing it is
treated the same:

```rust
use outcome::prelude::*;
```

Users can also work around this by using the `package` key in their
dependency declaration:

```toml
[dependencies.outcome]
version = "*"
package = "outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c"
```

Is this solution friendly to users? No, but neither is the lack of
namespacing nor a squatting policy on [crates.io]. If/when this problem is
resolved, this crate's documentation (and name!) will be changed and all
versions will be yanked.

[`Result`]: core::result::Result

[`Outcome`]: crate::prelude::Outcome

[`Success(S)`]: crate::prelude::Success
[`Mistake(M)`]: crate::prelude::Mistake
[`Failure(F)`]: crate::prelude::Failure

[crates.io]: https://crates.io
[`eyre`]: https://crates.io/crates/eyre
