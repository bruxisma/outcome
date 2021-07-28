//! The Outcome Prelude
//!
//! The `outcome` library comes with several types and traits. However, several
//! of these are the *most important*, while others are optional to be
//! imported. For this reason, the `prelude` module is provided for quick
//! imports. While it can't be automatically imported, it does contain the
//! *stable* interface available for each support Rust edition.
pub use crate::{aberration::Aberration, concern::Concern, outcome::Outcome};

// TODO: Change this to be an edition setting?
#[cfg_attr(any(docsrs, nightly), doc(cfg(feature = "nightly")))]
#[cfg(all(nightly, feature = "nightly"))]
#[doc(inline)]
pub use crate::convert::{AttemptFrom, AttemptInto};

#[doc(hidden)]
pub use Outcome::{Failure, Mistake, Success};
