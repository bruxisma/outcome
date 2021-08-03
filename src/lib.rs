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
//! ```rust
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
//! At this time, the name `outcome` is already taken on [crates.io]. As
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
//! [`TryLockError<T>`]: std::sync::TryLockError
//! [`PoisonError<T>`]: std::sync::PoisonError
//! [`WouldBlock`]: std::sync::TryLockError::WouldBlock
//!
//! [`Success(S)`]: crate::prelude::Success
//! [`Mistake(M)`]: crate::prelude::Mistake
//! [`Failure(F)`]: crate::prelude::Failure
//!
//! [crates.io]: https://crates.io
//! [`eyre`]: https://crates.io/crates/eyre
//!
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/features.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/why-augment-result.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/state-escalation.md"))]
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
