# Features

There are several features available to the crate that are disabled by
default. These include:

 - `unstable` (Enable "unstable" functions that mirror unstable functions
     found in [`Result`]. Unlike [`Result`], however, a nightly compiler is
     not required.)
 - `nightly` (Enable features that require the nightly rust compiler to be
     used, such as [`Try`])
 - `report` (Enable conversion from [`Aberration`] to an
     [`eyre::Report`][`Report`])

Users can also enable `no_std` support by either setting `default-features`
to `false` or simply not listing `std` in the list of features.

 - `nightly` will enable `unstable`.
 - `report` will enable `std`.

### `no_std`

Nearly every single feature in `outcome` supports working with `#![no_std]`
support, however currently `eyre` *does* require `std` support (Attempts
were made at making `no_std` work, but this was removed and has not been
available for some time).


```toml
[dependencies.outcome]
package = "outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c"
version = "..."
features = ["nightly"]
```

### `unstable`

When enabled, the `unstable` feature provides several associated methods for
[`Outcome`] that mirror unstable APIs found in [`Result<T, E>`][`Result`]. If
the methods mirrored are changed in any future releases of stable rust, these
will as well. Additionally, if any of the APIs are stabilized, they will be
moved out of this feature and into the default feature set. Unlike the
`nightly` feature, these APIs can be implemented in *stable* rust.

### `nightly`

The `nightly` feature set also requires a nightly toolchain. This is detected
in outcome's `build.rs` script via the
[rustversion](https://crates.io/crates/rustversion) crate. While users can
enable the nightly feature on a stable toolchain, nothing additional will
be compiled.

Once available, users will have to enable specific nightly features for
each API set mentioned. These are listed below.

 - `#![feature(try_trait_v2)]` &mdash; operator `?` support
   - [`Outcome`] may be used with operator `?`, including from functions
       that return a [`Result<T, E>`], as long as:
       - `E: From<Outcome::Failure>`
 - `#![feature(never_type)]` &mdash; APIs that return `!`
   - [`Outcome`] will have several functions where the `!` type is used in
       the function signature. These include `into_success`, and others.
   - Several stable functions that return an [`Aberration`] will instead return
       an `Outcome<!, M, F>`.
 - `#![feature(termination_trait_lib)]` &mdash; Exit process with an
      [`Outcome`]
   - **NOTE**: This requires the `std` feature to be enabled as well.
   - In addition to being usable with `fn main()`, *any unit test* may
       return an [`Outcome`] directly. This works in the same way as
       returning a [`Result<T, E>`]

### `report`

The `report` feature adds the [`WrapFailure`] trait to both [`Outcome`] and
[`Aberration`]. This trait is meant to mimic the [`WrapErr`] trait found on
[`Result<T, E>`][`Result`] that is provided by [`eyre`].  Therefore, a blanket
implementation is provided for all types that implement [`WrapErr`].  However,
to stay in line with `outcome`'s naming convention, instances of `err` have
been replaced with `failure`.

[`Result`]: core::result::Result
[`Try`]: core::ops::Try

[`WrapErr`]: eyre::WrapErr
[`Report`]: eyre::Report

[`WrapFailure`]: crate::report::WrapFailure
[`Aberration`]: crate::prelude::Aberration
[`Outcome`]: crate::prelude::Outcome

[`eyre`]: https://crates.io/crates/eyre
