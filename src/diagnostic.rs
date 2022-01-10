//! Support for the [`miette`] crate.
//!
//! This module re-exports `miette`'s [`Report`] as well as [`Result`], and
//! provides a trait, [`WrapFailure`], as a mirror to the [`WrapErr`] trait.
//! Additionally, [`WrapFailure`] is also implemented for [`Result<T, E>`].
//! Lastly, to stay in line with behavior from [`miette`], the [`WrapFailure`]
//! trait is *also* sealed.
//!
//! [`WrapErr`]: miette::WrapErr
//! [`miette`]: https://crates.io/crates/miette
extern crate std;

use crate::prelude::*;
use miette::Diagnostic;
use std::fmt::Display;

#[doc(no_inline)]
pub use miette::{Report, Result};

crate::wrap::r#trait!(Diagnostic);
crate::wrap::r#impl!(Diagnostic);
crate::wrap::result!(miette);
