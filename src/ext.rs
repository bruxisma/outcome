extern crate std;

use std::sync::{PoisonError, TryLockError, TryLockResult};
// use std::env::VarError;

#[cfg(feature = "report")]
use eyre::Report;

use super::*;

impl<T> From<TryLockResult<T>> for Outcome<T, TryLockError<T>, PoisonError<T>> {
  fn from(result: TryLockResult<T>) -> Self {
    match result {
      Ok(guard) => Success(guard),
      Err(m @ TryLockError::WouldBlock) => Mistake(m),
      Err(TryLockError::Poisoned(f)) => Failure(f),
    }
  }
}

impl<T> From<Outcome<T, TryLockError<T>, PoisonError<T>>> for TryLockResult<T> {
  fn from(outcome: Outcome<T, TryLockError<T>, PoisonError<T>>) -> Self {
    match outcome {
      Success(s) => Ok(s),
      Mistake(m @ TryLockError::WouldBlock) => Err(m),
      Mistake(TryLockError::Poisoned(f)) => Err(TryLockError::Poisoned(f)),
      Failure(f) => Err(TryLockError::Poisoned(f)),
    }
  }
}

impl<T> From<TryLockError<T>> for Aberration<TryLockError<T>, PoisonError<T>> {
  fn from(error: TryLockError<T>) -> Self {
    match error {
      x @ TryLockError::WouldBlock => Self::Mistake(x),
      TryLockError::Poisoned(p) => Self::Failure(p),
    }
  }
}

#[cfg(feature = "report")]
impl<M, F> From<Aberration<M, F>> for Report
where
  M: std::error::Error + Send + Sync + 'static,
  F: std::error::Error + Send + Sync + 'static,
{
  fn from(aberration: Aberration<M, F>) -> Self {
    match aberration {
      Aberration::Mistake(value) => Self::new(value),
      Aberration::Failure(value) => Self::new(value),
    }
  }
}
