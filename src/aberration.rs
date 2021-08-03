use core::fmt::Debug;

use crate::private::panic;

/// `Aberration` is a type that can represent a [`Mistake`], or [`Failure`].
///
/// **NOTE**: This type will become a type alias once `!` is stabilized.
///
/// See the [module documentation](crate) for details.
///
/// [`Mistake`]: Aberration::Mistake
/// [`Failure`]: Aberration::Failure
#[must_use = "This Aberration might be a `Mistake`, which should be handled"]
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Aberration<M, F> {
  /// Contains the mistake value. Analagous to
  /// [`Outcome::Mistake`](crate::prelude::Outcome::Mistake)
  Mistake(M),
  /// Contains the failure value. Analagous to
  /// [`Outcome::Failure`](crate::prelude::Outcome::Failure)
  Failure(F),
}

impl<M, F> Aberration<M, F> {
  /// Converts from `&Aberration<M, F>` to `Aberration<&M, &F>`.
  ///
  /// Produces a new `Aberration`, containing a reference into the original,
  /// leaving it in place.
  #[inline]
  pub fn as_ref(&self) -> Aberration<&M, &F> {
    match *self {
      Self::Mistake(ref value) => Aberration::Mistake(value),
      Self::Failure(ref value) => Aberration::Failure(value),
    }
  }

  /// Converts from `&mut Aberration<M, F>` to `Aberration<&mut M, &mut F>`
  #[inline]
  pub fn as_mut(&mut self) -> Aberration<&mut M, &mut F> {
    match *self {
      Self::Mistake(ref mut value) => Aberration::Mistake(value),
      Self::Failure(ref mut value) => Aberration::Failure(value),
    }
  }

  /// Returns `true` if the aberration is a [`Mistake`]
  ///
  /// [`Mistake`]: Aberration::Mistake
  #[must_use = "if you intended to assert a mistake, consider `.unwrap_mistake()` instead"]
  #[inline]
  pub fn is_mistake(&self) -> bool {
    if let Self::Mistake(_) = self {
      return true;
    }
    false
  }

  /// Returns `true` if the aberration is a [`Failure`]
  ///
  /// [`Failure`]: Aberration::Failure
  #[must_use = "if you intended to assert a failure, consider `.unwrap_failure()` instead"]
  #[inline]
  pub fn is_failure(&self) -> bool {
    if let Self::Failure(_) = self {
      return true;
    }
    false
  }

  /// Converts from `Aberration<M, F>` to [`Option<M>`]
  #[inline]
  pub fn mistake(self) -> Option<M> {
    if let Self::Mistake(value) = self {
      return Some(value);
    }
    None
  }

  /// Converts from `Aberration<M, F>` to [`Option<F>`]
  #[inline]
  pub fn failure(self) -> Option<F> {
    if let Self::Failure(value) = self {
      return Some(value);
    }
    None
  }

  /// Maps an `Aberration<M, F>` to `Aberration<N, F>` by applying a function
  /// to a contained [`Mistake`] value, leaving any [`Failure`] value
  /// untouched.
  ///
  /// [`Mistake`]: Aberration::Mistake
  /// [`Failure`]: Aberration::Failure
  #[inline]
  pub fn map_mistake<N, C>(self, callable: C) -> Aberration<N, F>
  where
    C: FnOnce(M) -> N,
  {
    match self {
      Self::Mistake(value) => Aberration::Mistake(callable(value)),
      Self::Failure(value) => Aberration::Failure(value),
    }
  }

  /// Maps an `Aberration<M, F>` to `Aberration<M, G>` by applying a function
  /// to a contained [`Failure`] value, leaving any [`Mistake`] value
  /// untouched.
  ///
  /// [`Mistake`]: Aberration::Mistake
  /// [`Failure`]: Aberration::Failure
  #[inline]
  pub fn map_failure<G, C>(self, callable: C) -> Aberration<M, G>
  where
    C: FnOnce(F) -> G,
  {
    match self {
      Self::Mistake(value) => Aberration::Mistake(value),
      Self::Failure(value) => Aberration::Failure(callable(value)),
    }
  }
}

impl<M, F: Debug> Aberration<M, F> {
  /// Returns the contained [`Mistake`] value, consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is a [`Failure`], with a custom panic message
  /// provided by the failure.
  ///
  /// # Examples
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Aberration<&str, i32> = Aberration::Failure(47);
  /// x.unwrap_mistake(); // panics with '47'
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Aberration<&str, i32> = Aberration::Mistake("try again!");
  /// assert_eq!(x.unwrap_mistake(), "try again!");
  /// ```
  ///
  /// [`Mistake`]: Aberration::Mistake
  /// [`Failure`]: Aberration::Failure
  #[track_caller]
  #[inline]
  pub fn unwrap_mistake(self) -> M {
    match self {
      Self::Mistake(m) => m,
      Self::Failure(f) => panic("Aberration::unwrap_mistake()", "Failure", &f),
    }
  }
}

impl<M: Debug, F> Aberration<M, F> {
  /// Returns the contained [`Failure`] value, consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is a [`Mistake`], with a custom panic message
  /// provided by the mistake.
  ///
  /// # Examples
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Aberration<i32, &str> = Aberration::Mistake(47);
  /// x.unwrap_failure(); // panics with '47'
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Aberration<i32, &str> = Aberration::Failure("error!");
  /// assert_eq!(x.unwrap_failure(), "error!");
  /// ```
  ///
  /// [`Mistake`]: Aberration::Mistake
  /// [`Failure`]: Aberration::Failure
  #[track_caller]
  #[inline]
  pub fn unwrap_failure(self) -> F {
    match self {
      Self::Mistake(m) => panic("Aberration::unwrap_failure()", "Mistake", &m),
      Self::Failure(f) => f,
    }
  }
}

impl<M: Clone, F: Clone> Clone for Aberration<M, F> {
  #[inline]
  fn clone(&self) -> Self {
    match self {
      Self::Mistake(value) => Self::Mistake(value.clone()),
      Self::Failure(value) => Self::Failure(value.clone()),
    }
  }

  #[inline]
  fn clone_from(&mut self, source: &Self) {
    match (self, source) {
      (Self::Mistake(to), Self::Mistake(from)) => to.clone_from(from),
      (Self::Failure(to), Self::Failure(from)) => to.clone_from(from),
      (to, from) => *to = from.clone(),
    }
  }
}
