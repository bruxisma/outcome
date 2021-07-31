//! [`Outcome<S, M, F>`][`Outcome`] is an augmentation of the [`Result`] type
//! found in the Rust standard library.
//!
//! It is an enum with the variants
//!  - [`Success(S)`], representing success and containing a value
//!  - [`Mistake(M)`], representing an optionally *retryable error* and
//!     containing a value
//!  - [`Failure(F)`], representing failure and containing a value.
//!
//! ```
//! # #[allow(dead_code)]
//! enum Outcome<S, M, F> {
//!   Success(S),
//!   Mistake(M),
//!   Failure(F),
//! }
//! ```
//!
//! [`Outcome`] is an *augmentation* to [`Result`]. It adds a third state to
//! the "success or failure" dichotomy that [`Result<T, E>`] models. This third
//! state is that of a *soft* or *retryable* error. A *retryable* error is one
//! where an operation might not have succeeded, either due to other operations
//! (e.g., a disk read or write not completing), misconfiguration (e.g.,
//! forgetting to set a specific flag before calling a function), or busy
//! resources (e.g., attempting to lock an audio, video, or database resource).
//!
//! ```
//! # use outcome::prelude::*;
//! #[derive(Debug, PartialEq)]
//! enum Version { V1, V2 }
//!
//! #[derive(Debug, PartialEq)]
//! struct EmptyInput;
//!
//! fn parse_version(header: &[u8]) -> Outcome<Version, EmptyInput, &'static str> {
//!   match header.get(0) {
//!     None => Mistake(EmptyInput),
//!     Some(&1) => Success(Version::V1),
//!     Some(&2) => Success(Version::V2),
//!     Some(_) => Failure("invalid or unknown version"),
//!   }
//! }
//!
//! let version = parse_version(&[]);
//! assert_eq!(version, Mistake(EmptyInput));
//! ```
//!
//! # Usage
//!
//! At this time, the `outcome` crate is already taken on [crates.io]. As
//! [crates.io] does not yet support namespaces or collections, we've had to
//! take a *unique* approach to still publish the crate. To do this, we've
//! generated a `UUIDv5` string via python:
//!
//! ```python
//! from uuid import *
//! print(uuid5(uuid5(NAMESPACE_DNS, "occult.work"), "outcome"))
//! ```
//!
//! This *should* generate the string `46f94afc-026f-5511-9d7e-7d1fd495fb5c`.
//! Thus the dependency in your `Cargo.toml` will look something like:
//!
//! ```toml
//! [dependencies]
//! outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c = "*"
//! ```
//!
//! However, the exported library is still named `outcome`, so importing it is
//! treated the same:
//!
//! ```rust
//! use outcome::prelude::*;
//! ```
//!
//! Users can also work around this by using the `package` key in their
//! dependency declaration:
//!
//! ```toml
//! [dependencies.outcome]
//! version = "*"
//! package = "outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c"
//! ```
//!
//! Is this solution friendly to users? No, but neither is the lack of
//! namespacing nor a squatting policy on [crates.io]. If/when this problem is
//! resolved, this crate's documentation (and name!) will be changed and all
//! versions will be yanked.
//!
//! # Features
//!
//! There are several features available to the crate that are disabled by
//! default. These include:
//!
//!  - `unstable` (Enable "unstable" functions that mirror unstable functions
//!      found in [`Result`]. Unlike [`Result`], however, a nightly compiler is
//!      not required.)
//!  - `nightly` (Enable features that require the nightly rust compiler to be
//!      used, such as [`Try`])
//!  - `report` (Enable conversion from [`Aberration`] to an
//!      [`eyre::Report`])
//!
//! Users can also enable `no_std` support by either setting `default-features`
//! to `false` or simply not listing `std` in the list of features.
//!
//!  - `nightly` will enable `unstable`.
//!  - `report` will enable `std`.
//!
//! ### `no_std`
//!
//! Nearly every single feature in `outcome` supports working with `#![no_std]`
//! support, however currently `eyre` *does* require `std` support (Attempts
//! were made at making `no_std` work, but this was removed and has not been
//! available for some time).
//!
//!
//! ```toml
//! [dependencies.outcome]
//! package = "outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c"
//! version = "..."
//! features = ["nightly"]
//! ```
//!
//! ### `unstable`
//!
//! When enabled, the `unstable` feature provides several associated methods
//! for [`Outcome`] that mirror unstable APIs found in [`Result<T, E>`]. If the
//! methods mirrored are changed in any future releases of stable rust, these
//! will as well. Additionally, if any of the APIs are stabilized, they will be
//! moved out of this feature and into the default feature set. Unlike the
//! `nightly` feature, these APIs can be implemented in *stable* rust.
//!
//! ### `nightly`
//!
//! The `nightly` feature set also requires a nightly toolchain. This is detected
//! in outcome's `build.rs` script via the
//! [rustversion](https://crates.io/crates/rustversion) crate. While users can
//! enable the nightly feature on a stable toolchain, nothing additional will
//! be compiled.
//!
//! Once available, users will have to enable specific nightly features for
//! each API set mentioned. These are listed below.
//!
//!  - `#![feature(try_trait_v2)]` &mdash; operator `?` support
//!    - [`Outcome`] may be used with operator `?`, including from functions
//!        that return a [`Result<T, E>`], as long as `E: From<F>`.
//!  - `#![feature(never_type)]` - APIs that return `!`
//!    - [`Outcome`] will have several functions where the `!` type is used in
//!        the function signature. These include `into_success`, and others.
//!  - `#![feature(termination_trait_lib)]` - Exit process with an [`Outcome`]
//!    - **NOTE**: This requires the `std` feature to be enabled as well.
//!    - In addition to being usable with `fn main()`, *any unit test* may
//!        return an [`Outcome`] directly. This works in the same way as
//!        returning a [`Result<T, E>`]
//!
//! ### `report`
//!
//! The `report` feature adds several additional associated methods to beoth
//! [`Outcome`] and [`Aberration`]. These are meant to mimic the [`WrapErr`]
//! functions found on [`Result<T, E>`] that is provided by [`eyre`]. However,
//! to stay in line with `outcome`'s naming convention, instances of `err` have
//! been replaced with `failure`.
//!
//! # Why Augment `Result<T, E>`?
//!
//! [`Outcome`] is *not* intended to fully replace [`Result`], especially at
//! the API boundary (i.e., the API used by clients) when there is a clear
//! success or failure state that can be transferred to users. Instead, it
//! provides the ability to quickly expand the surface area of consumed APIs
//! with finer grained control over errors so that library writers can write
//! *correct* behavior and then return at a later time to compose results,
//! expand error definitions, or to represent different error severities.
//!
//! As an example, the section [making unhandled errors unrepresentable][1] in
//! the post *Error Handling in a Correctness-Critical Rust Project*, the
//! author states:
//!
//! > this led me to go for what felt like the nuclear solution, but after
//! > seeing how many bugs it immediately rooted out by simply refactoring the
//! > codebase, I’m convinced that this is the only way to do error handling in
//! > systems where we have multiple error handling concerns in Rust today.
//!
//! The solution, as they explain in the next paragraph is
//!
//! > make the global `Error` enum specifically only hold errors that should
//! > cause the overall system to halt - reserved for situations that require
//! > human intervention. Keep errors which relate to separate concerns in
//! > totally separate error types. By keeping errors that must be handled
//! > separately in their own types, we reduce the chance that the try `?`
//! > operator will accidentally push a local concern into a caller that can’t
//! > deal with it.
//!
//! As the author of this post later shows, the `sled::Tree::compare_and_swap`
//! function returns a `Result<Result<(), CompareAndSwapError>, sled::Error>`.
//! They state this looks "*way less cute*", but will
//!
//! > improve chances that users will properly handle their compare and
//! > swap-related errors properly\[sic]
//! > ```ignore,compile_fail,E0425
//! > // we can actually use try `?` now
//! > let cas_result = sled.compare_and_swap(
//! >   "dogs",
//! >   "pickles",
//! >   "catfood"
//! > )?;
//! >
//! > if let Err(cas_error) = cas_result {
//! >     // handle expected issue
//! > }
//! > ```
//!
//! The issue with this return type is that there is *technically nothing* to
//! stop a user from using what the creator of the `outcome` crate calls the
//! WTF operator (`??`) to ignore these intermediate errors.
//!
//! ```ignore,compile_fail,E0425
//! let cas = sled.compare_and_swap("dogs", "pickles", "catfood")??;
//! ```
//!
//! It would be hard to *forbid* this kind of usage with tools like clippy due
//! to libraries such as [nom][2] relying on nested results and expecting
//! moderately complex pattern matching to extract relevant information.
//!
//! Luckily, it *is* easier to prevent this issue in the first place if:
//!
//!  - An explicit call to extract an inner `Result<T, E>` must be made
//!  - The call of an easily greppable/searchable function before using the
//!     "WTF" (`??`) operator is permitted.
//!  - The [`Try`] trait returns a type that *must* be decomposed explicitly
//!     and *does not support* the try `?` operator itself.
//!
//! Thanks to [clippy](https://github.com/rust-lang/rust-clippy)'s
//! `disallowed_method` lint, users can rely on the first two options until
//! [`Try`] has been stabilized.
//!
//! # State Escalation (TODO)
//!
//! ---
//!
//! [`Try`]: core::ops::Try
//!
//! [`TryLockError<T>`]: std::sync::TryLockError
//! [`PoisonError<T>`]: std::sync::PoisonError
//! [`WouldBlock`]: std::sync::TryLockError::WouldBlock
//!
//! [`WrapErr`]: eyre::WrapErr
//!
//! [`Success(S)`]: crate::prelude::Success
//! [`Mistake(M)`]: crate::prelude::Mistake
//! [`Failure(F)`]: crate::prelude::Failure
//!
//! [`UnixDatagram::take_error`]: https://doc.rust-lang.org/nightly/std/os/unix/net/struct.UnixDatagram.html#method.take_error
//! [crates.io]: https://crates.io
//! [`eyre`]: https://crates.io/crates/eyre
//!
//! [1]: https://sled.rs/errors.html#making-unhandled-errors-unrepresentable
//! [2]: https://crates.io/crates/nom

#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::let_underscore_drop)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::redundant_else)]
#![warn(clippy::single_match_else)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_self)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
//#![warn(missing_doc_code_examples)]
#![warn(missing_docs)]
#![warn(unsafe_code)]
#![cfg_attr(
  all(nightly, feature = "nightly", feature = "std"),
  feature(process_exitcode_placeholder),
  feature(termination_trait_lib)
)]
#![cfg_attr(
  all(nightly, feature = "nightly"),
  feature(try_trait_v2),
  feature(never_type),
  feature(exhaustive_patterns)
)]
#![cfg_attr(any(docsrs, nightly), feature(doc_cfg))]
#![no_std]

#[cfg(doc)]
extern crate std;

#[cfg_attr(any(docsrs, nightly), doc(cfg(feature = "unstable")))]
#[cfg(feature = "unstable")]
mod unstable;

#[cfg_attr(any(docsrs, nightly), doc(cfg(feature = "nightly")))]
#[cfg(all(nightly, feature = "nightly"))]
mod nightly;

mod aberration;
mod concern;
mod outcome;
mod private;

mod iter;

pub mod convert;
pub mod prelude;

#[cfg_attr(any(docsrs, nightly), doc(cfg(feature = "report")))]
#[cfg(feature = "report")]
pub mod report;

#[cfg_attr(doc, doc(inline))]
pub use crate::{aberration::*, concern::*, convert::*, iter::*, outcome::*};
