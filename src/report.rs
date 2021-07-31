//! Support for the [`eyre`] crate
//!
//! [`eyre`]: https://crates.io/crates/eyre
extern crate std;

use std::{error::Error, fmt::Display};

pub use eyre::Report;

use crate::prelude::*;

/// Provides the `wrap_failure` method for [`Outcome`].
///
/// This trait is sealed and cannot be implemented for types outside of
/// `outcome`.
///
/// Additionally, this trait is meant to *mirror* the [`WrapErr`] trait found
/// in [`eyre`].
///
/// [`Outcome`]: crate::prelude::Outcome
/// [`WrapErr`]: eyre::WrapErr
///
/// [`eyre`]: https://crates.io/crates/eyre
#[cfg(feature = "report")]
pub trait WrapFailure: Sized + crate::private::Sealed {
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
  #[track_caller]
  #[inline]
  fn with_context<D, F>(self, message: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    self.wrap_failure_with(message)
  }

  /// Compatibility re-export of [`wrap_failure`] for interop with
  /// [`anyhow`] and [`eyre`].
  ///
  /// [`wrap_failure`]: WrapFailure::wrap_failure
  /// [`anyhow`]: https://crates.io/crates/anyhow
  /// [`eyre`]: https://crates.io/crates/eyre
  #[track_caller]
  #[inline]
  fn context<D>(self, message: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self.wrap_failure(message)
  }
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
}

impl<T, E> WrapFailure for Result<T, E>
where
  E: Error + Send + Sync + 'static,
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
}
