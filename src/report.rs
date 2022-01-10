//! Support for the [`eyre`] crate.
//!
//! This module re-exports [`Report`] as well as [`Result`], and provides a
//! trait, [`WrapFailure`], as a mirror to the [`WrapErr`] trait. Additionally,
//! [`WrapFailure`] is also implemented for [`Result<T, E>`]. Lastly, to stay
//! in line with behavior from [`eyre`], the [`WrapFailure`] trait is *also*
//! sealed.
//!
//! [`WrapErr`]: eyre::WrapErr
//! [`Result`]: eyre::Result
//! [`eyre`]: https://crates.io/crates/eyre
extern crate std;

use crate::prelude::*;
use std::{error::Error,fmt::Display};

#[doc(no_inline)]
pub use eyre::{Report, Result};

crate::wrap::r#trait!(Error);
crate::wrap::r#impl!(Error);
crate::wrap::r#result!(eyre);
