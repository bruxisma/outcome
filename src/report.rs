extern crate std;

use std::{error::Error, fmt::Display};

use eyre::Report;

use crate::prelude::*;

// TODO: Use type Return = T<...Ts, Report>; type Error = Report
//       use a sealed trait for the same reason.

/// Provides easier interop with [`Report`].
///
/// Each associated method is meant to match the equivalent set of calls found
/// in [`WrapErr`], but with `outcome`'s naming convention, where instances of
/// `err` are replaced with `failure`.
///
/// [`WrapErr`]: eyre::WrapErr
/// [`Report`]: eyre::Report
impl<S, M, E> Outcome<S, M, E>
where
  E: Error + Send + Sync + 'static,
{
  /// Wrap the failure value with a new adhoc error
  #[track_caller]
  #[inline]
  pub fn wrap_failure<D>(self, message: D) -> Outcome<S, M, Report>
  where
    D: Display + Send + Sync + 'static,
  {
    match self {
      Success(s) => Success(s),
      Mistake(m) => Mistake(m),
      Failure(f) => Failure(Report::new(f).wrap_err(message)),
    }
  }

  /// Wrap the failure value with a new adhoc error that is evaluated lazily only
  /// once a failure does occur.
  #[track_caller]
  #[inline]
  pub fn wrap_failure_with<D, F>(self, function: F) -> Outcome<S, M, Report>
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    match self {
      Success(s) => Success(s),
      Mistake(m) => Mistake(m),
      Failure(f) => Failure(Report::new(f).wrap_err(function())),
    }
  }

  /// Compatibility re-export of [`wrap_failure`] for interop with [`anyhow`]
  /// and [`eyre`].
  ///
  /// [`wrap_failure`]: crate::prelude::Outcome::wrap_failure
  /// [`anyhow`]: https://crates.io/crates/anyhow
  /// [`eyre`]: https://crates.io/crates/eyre
  #[track_caller]
  #[inline]
  pub fn context<D>(self, message: D) -> Outcome<S, M, Report>
  where
    D: Display + Send + Sync + 'static,
  {
    self.wrap_failure(message)
  }

  /// Compatibility re-export of [`wrap_failure_with`] for interop with
  /// [`anyhow`] and [`eyre`].
  ///
  /// [`wrap_failure_with`]: crate::prelude::Outcome::wrap_failure_with
  /// [`anyhow`]: https://crates.io/crates/anyhow
  /// [`eyre`]: https://crates.io/crates/eyre
  #[track_caller]
  #[inline]
  pub fn with_context<D, F>(self, function: F) -> Outcome<S, M, Report>
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    self.wrap_failure_with(function)
  }
}

impl<M, E> Aberration<M, E>
where
  E: Error + Send + Sync + 'static,
{
  /// Wrap the error value with a new adhoc error
  #[track_caller]
  #[inline]
  pub fn wrap_failure<D>(self, message: D) -> Aberration<M, Report>
  where
    D: Display + Send + Sync + 'static,
  {
    match self {
      Self::Mistake(m) => Aberration::Mistake(m),
      Self::Failure(f) => Aberration::Failure(Report::new(f).wrap_err(message)),
    }
  }

  /// Wrap the error value with a new adhoc error that is evaluated lazily only
  /// once an error does occur.
  #[track_caller]
  #[inline]
  pub fn wrap_failure_with<D, F>(self, function: F) -> Aberration<M, Report>
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    match self {
      Self::Mistake(m) => Aberration::Mistake(m),
      Self::Failure(f) => {
        Aberration::Failure(Report::new(f).wrap_err(function()))
      }
    }
  }

  /// Compatibility re-export of [`wrap_failure`] for interopt with [`anyhow`].
  ///
  /// [`wrap_failure`]: crate::prelude::Outcome::wrap_failure
  /// [`anyhow`]: https://crates.io/crates/anyhow
  #[track_caller]
  #[inline]
  pub fn context<D>(self, message: D) -> Aberration<M, Report>
  where
    D: Display + Send + Sync + 'static,
  {
    self.wrap_failure(message)
  }

  /// Compatibility re-export of [`wrap_failure_with`] for interopt with
  /// [`anyhow`].
  ///
  /// [`wrap_failure_with`]: crate::prelude::Aberration::wrap_failure_with
  /// [`anyhow`]: https://crates.io/crates/anyhow
  #[track_caller]
  #[inline]
  pub fn with_context<D, F>(self, function: F) -> Aberration<M, Report>
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    self.wrap_failure_with(function)
  }
}
