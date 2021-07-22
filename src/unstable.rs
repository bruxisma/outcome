use crate::*;

use core::cmp::PartialEq;

impl<S: Copy, M, F> Outcome<&S, M, F> {
  /// Maps an `Outcome<&S, M, F>` to an `Outcome<S, M, F>` by copying the
  /// contents of the `Success` value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::*;
  /// ```
  pub fn copied(self) -> Outcome<S, M, F> {
    self.map(|&s| s)
  }
}

impl<S: Copy, M, F> Outcome<&mut S, M, F> {
  /// Maps an `Outcome<&mut S, M, F>` to an `Outcome<S, M, F>` by copying the
  /// contents of the `Success` value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::*;
  /// ```
  pub fn copied(self) -> Outcome<S, M, F> {
    self.map(|&mut s| s)
  }
}

impl<S: Clone, M, F> Outcome<&S, M, F> {
  /// Maps an `Outcome<&S, M, F>` to an `Outcome<S, M, F>` by cloning the
  /// contents of the `Success` value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::*;
  /// let val = 47;
  /// let x: Outcome<&i32, u32, f32> = Success(&val);
  /// assert_eq!(x, Success(&47));
  /// let cloned = x.cloned();
  /// assert_eq!(cloned, Success(47));
  /// ```
  pub fn cloned(self) -> Outcome<S, M, F> {
    self.map(Clone::clone)
  }
}

impl<S: Clone, M, F> Outcome<&mut S, M, F> {
  /// Maps an `Outcome<&mut S, M, F>` to an `Outcome<S, M, F>` by cloning the
  /// contents of the `Success` value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::*;
  /// let mut val = 47;
  /// let x: Outcome<&mut i32, u32, i32> = Success(&mut val);
  /// assert_eq!(x, Success(&mut 47));
  /// let cloned = x.cloned();
  /// assert_eq!(cloned, Success(47));
  /// ```
  pub fn cloned(self) -> Outcome<S, M, F> {
    self.map(|s| s.clone())
  }
}

impl<S, M, F> Outcome<S, M, F> {
  /// Returns `true` if the outcome is a [`Success`] value containing the given
  /// value.
  #[must_use]
  #[inline]
  pub fn contains<U>(&self, other: &U) -> bool
  where
    U: PartialEq<S>,
  {
    if let Success(value) = self {
      return other == value;
    }
    false
  }

  /// Returns `true` if the outcome is a [`Mistake`] value containing the given
  /// value.
  #[must_use]
  #[inline]
  pub fn contains_mistake<N>(&self, other: &N) -> bool
  where
    N: PartialEq<M>,
  {
    if let Mistake(value) = self {
      return other == value;
    }
    false
  }

  /// Returns `true` if the outcome is a [`Failure`] value containing the given
  /// value.
  #[must_use]
  #[inline]
  pub fn contains_failure<G>(&self, other: &G) -> bool
  where
    G: PartialEq<F>,
  {
    if let Failure(value) = self {
      return other == value;
    }
    false
  }

  /// Returns the contained [`Success`] value, consuming the `self` value,
  /// without checking that the value is not a [`Mistake`] or [`Failure`].
  ///
  /// # Safety
  ///
  /// Calling this method on a [`Mistake`] or [`Failure`] is *[undefined
  /// behavior]*
  ///
  /// # Examples
  ///
  /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
  #[allow(unsafe_code)]
  pub unsafe fn unwrap_unchecked(self) -> S {
    debug_assert!(self.is_success());
    if let Success(value) = self {
      return value;
    }
    core::hint::unreachable_unchecked();
  }

  /// Returns the contained [`Mistake`] value, consuming the `self` value,
  /// without checking that the value is not a [`Success`] or [`Failure`].
  ///
  /// # Safety
  ///
  /// Calling this method on a [`Success`] or [`Failure`] is *[undefined
  /// behavior]*
  ///
  /// # Examples
  ///
  /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
  #[allow(unsafe_code)]
  pub unsafe fn unwrap_mistake_unchecked(self) -> M {
    debug_assert!(self.is_mistake());
    if let Mistake(value) = self {
      return value;
    }
    core::hint::unreachable_unchecked();
  }

  /// Returns the contained [`Failure`] value, consuming the `self` value
  /// without checking that the value is not a [`Success`] or [`Mistake`].
  ///
  /// # Safety
  ///
  /// Calling this method on a [`Success`] or [`Mistake`] is *[undefined
  /// behavior]*
  ///
  /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
  #[allow(unsafe_code)]
  pub unsafe fn unwrap_failure_unchecked(self) -> F {
    debug_assert!(self.is_failure());
    if let Failure(value) = self {
      return value;
    }
    core::hint::unreachable_unchecked();
  }
}

impl<S, M, F> Outcome<Outcome<S, M, F>, M, F> {
  /// Converts from `Outcome<Outcome<S, M, F>, M, F>` to `Outcome<S, M, F>`
  pub fn flatten(self) -> Outcome<S, M, F> {
    self.and_then(core::convert::identity)
  }
}
