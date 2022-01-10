#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/features.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/why-augment-result.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/state-escalation.md"))]
#![doc(test(attr(allow(unused_imports))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(deny(warnings))))]
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
mod wrap;

pub mod convert;
pub mod prelude;


//#[cfg(all(feature = "report", feature = "diagnostic", not(doc), not(test)))]
//compile_error!("`diagnostic` and `report` features are mutually exclusive");

//#[cfg(any(feature = "diagnostic", feature = "report"))]
//#[doc(hidden)]
//pub mod wrap;

#[cfg_attr(any(docsrs, nightly), doc(cfg(feature = "report")))]
#[cfg(feature = "report")]
pub mod report;

#[cfg_attr(any(docsrs, nightly), doc(cfg(feature = "diagnostic")))]
#[cfg(feature = "diagnostic")]
pub mod diagnostic;

#[cfg_attr(doc, doc(inline))]
pub use crate::{aberration::*, concern::*, convert::*, iter::*, outcome::*};
