use core::cmp::PartialEq;

use crate::prelude::*;

impl<S, M, F> Outcome<S, M, F> {
  /// Returns `true` if the outcome is a [`Success`] value containing the given
  /// value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Success(47);
  /// assert_eq!(x.contains(&47), true);
  ///
  /// let x: Outcome<u32, f32, &str> = Success(47);
  /// assert_eq!(x.contains(&42), false);
  ///
  /// let x: Outcome<u32, f32, &str> = Mistake(0.0f32);
  /// assert_eq!(x.contains(&47), false);
  ///
  /// let x: Outcome<u32, f32, &str> = Failure("Some error message");
  /// assert_eq!(x.contains(&47), false);
  /// ```
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
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  ///
  /// let x: Outcome<u32, &str, i32> = Success(47);
  /// assert_eq!(x.contains_mistake(&"Some mistake message"), false);
  ///
  /// let x: Outcome<u32, &str, i32> = Mistake("Some mistake message");
  /// assert_eq!(x.contains_mistake(&"Some mistake message"), true);
  ///
  /// let x: Outcome<u32, &str, i32> = Mistake("Some other mistake message");
  /// assert_eq!(x.contains_mistake(&"Some mistake message"), false);
  ///
  /// let x: Outcome<u32, &str, i32> = Failure(47);
  /// assert_eq!(x.contains_mistake(&"Some error message"), false);
  /// ```
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
  ///
  /// # Examples
  /// ```
  /// # use outcome::prelude::*;
  ///
  /// let x: Outcome<u32, i32, &str> = Success(47);
  /// assert_eq!(x.contains_failure(&"Some error message"), false);
  ///
  /// let x: Outcome<u32, i32, &str> = Mistake(47);
  /// assert_eq!(x.contains_failure(&"Some error message"), false);
  ///
  /// let x: Outcome<u32, i32, &str> = Failure("Some error message");
  /// assert_eq!(x.contains_failure(&"Some error message"), true);
  ///
  /// let x: Outcome<u32, u32, &str> = Failure("Some other error message");
  /// assert_eq!(x.contains_failure(&"Some error message"), false);
  /// ```
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
}

impl<S, M, F> Outcome<Outcome<S, M, F>, M, F> {
  /// Converts from `Outcome<Outcome<S, M, F>, M, F>` to `Outcome<S, M, F>`
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<Outcome<&'static str, u32, u32>, u32, u32> = Success(Success("hello"));
  /// assert_eq!(Success("hello"), x.flatten());
  ///
  /// let x: Outcome<Outcome<&'static str, u32, u32>, u32, u32> = Success(Mistake(47));
  /// assert_eq!(Mistake(47), x.flatten());
  ///
  /// let x: Outcome<Outcome<&'static str, u32, u32>, u32, u32> = Success(Failure(47));
  /// assert_eq!(Failure(47), x.flatten());
  ///
  /// let x: Outcome<Outcome<&'static str, u32, u32>, u32, u32> = Mistake(47);
  /// assert_eq!(Mistake(47), x.flatten());
  ///
  /// let x: Outcome<Outcome<&'static str, u32, u32>, u32, u32> = Failure(47);
  /// assert_eq!(Failure(47), x.flatten());
  /// ```
  ///
  /// **NOTE**: Flattening only removes *one* level of nesting at a time:
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// type Nested<T> = Outcome<Outcome<Outcome<T, u32, u32>, u32, u32>, u32, u32>;
  /// let x: Nested<&'static str> = Success(Success(Success("hello")));
  /// assert_eq!(Success(Success("hello")), x.flatten());
  /// assert_eq!(Success("hello"), x.flatten().flatten());
  /// ```
  pub fn flatten(self) -> Outcome<S, M, F> {
    self.and_then(core::convert::identity)
  }
}
