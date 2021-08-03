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

use std::{error::Error, fmt::Display};

#[doc(no_inline)]
pub use eyre::{Report, Result};

use crate::prelude::*;

/// Provides the `wrap_failure` method for [`Outcome`].
///
/// This trait is sealed and cannot be implemented for types outside of
/// `outcome`.
///
/// Additionally, this trait is meant to *mirror* the [`WrapErr`] trait found
/// in [`eyre`], and therefore any type that implements `WrapErr` will
/// automatically work with `WrapFailure`.
///
/// ```
/// # use outcome::prelude::*;
/// use outcome::report::{WrapFailure, Result, Report};
///
/// fn execute() -> Result<()> {
///   # Err(Report::msg("error here"))?;
///   # const IGNORE: &str = stringify! {
///   ...
///   # };
///   # unreachable!()
/// }
///
/// pub fn invoke() -> Result<Vec<u8>> {
///   execute().wrap_failure("Failed to execute correctly")?;
///   Ok(vec![])
/// }
/// ```
///
/// [`Outcome`]: crate::prelude::Outcome
/// [`WrapErr`]: eyre::WrapErr
///
/// [`eyre`]: https://crates.io/crates/eyre
pub trait WrapFailure: crate::private::Sealed {
  /// The expected return type for an `impl`.
  ///
  /// This will always be the same enumeration type, but with a [`Report`]
  /// in the error or failure position.
  type Return;

  /// Wrap the failure value with a new adhoc error.
  fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D;

  /// Wrap the failure value with a new adhoc error that is evaluated lazily
  /// only once an error does occur.
  fn wrap_failure<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static;

  /// Compatibility re-export of [`wrap_failure_with`] for interop with
  /// [`anyhow`] and [`eyre`].
  ///
  /// [`wrap_failure_with`]: WrapFailure::wrap_failure_with
  /// [`anyhow`]: https://crates.io/crates/anyhow
  /// [`eyre`]: https://crates.io/crates/eyre
  fn with_context<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D;

  /// Compatibility re-export of [`wrap_failure`] for interop with
  /// [`anyhow`] and [`eyre`].
  ///
  /// [`wrap_failure`]: WrapFailure::wrap_failure
  /// [`anyhow`]: https://crates.io/crates/anyhow
  /// [`eyre`]: https://crates.io/crates/eyre
  fn context<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static;
}

impl<S, M, E> WrapFailure for Outcome<S, M, E>
where
  E: Error + Send + Sync + 'static,
{
  type Return = Outcome<S, M, Report>;

  #[track_caller]
  #[inline]
  fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    match self {
      Success(s) => Success(s),
      Mistake(m) => Mistake(m),
      Failure(f) => Failure(Report::new(f).wrap_err(message())),
    }
  }

  #[track_caller]
  #[inline]
  fn wrap_failure<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    match self {
      Success(s) => Success(s),
      Mistake(m) => Mistake(m),
      Failure(f) => Failure(Report::new(f).wrap_err(message)),
    }
  }

  #[track_caller]
  #[inline]
  fn with_context<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    self.wrap_failure_with(message)
  }

  #[track_caller]
  #[inline]
  fn context<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self.wrap_failure(message)
  }
}

impl<M, E> WrapFailure for Aberration<M, E>
where
  E: Error + Send + Sync + 'static,
{
  type Return = Aberration<M, Report>;

  #[track_caller]
  #[inline]
  fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    match self {
      Self::Mistake(m) => Aberration::Mistake(m),
      Self::Failure(f) => {
        Aberration::Failure(Report::new(f).wrap_err(message()))
      }
    }
  }

  #[track_caller]
  #[inline]
  fn wrap_failure<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    match self {
      Self::Mistake(m) => Aberration::Mistake(m),
      Self::Failure(f) => Aberration::Failure(Report::new(f).wrap_err(message)),
    }
  }

  #[track_caller]
  #[inline]
  fn with_context<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    self.wrap_failure_with(message)
  }

  #[track_caller]
  #[inline]
  fn context<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self.wrap_failure(message)
  }
}

impl<T, E> WrapFailure for Result<T, E>
where
  Self: eyre::WrapErr<T, E>,
{
  type Return = Result<T, Report>;

  #[track_caller]
  #[inline]
  fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    eyre::WrapErr::wrap_err_with(self, message)
  }

  #[track_caller]
  #[inline]
  fn wrap_failure<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    eyre::WrapErr::wrap_err(self, message)
  }

  #[track_caller]
  #[inline]
  fn with_context<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    eyre::WrapErr::with_context(self, message)
  }

  #[track_caller]
  #[inline]
  fn context<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    eyre::WrapErr::context(self, message)
  }
}
