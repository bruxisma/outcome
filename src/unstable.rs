use core::cmp::PartialEq;

use crate::prelude::*;

impl<S: Copy, M, F> Outcome<&S, M, F> {
  /// Maps an `Outcome<&S, M, F>` to an `Outcome<S, M, F>` by copying the
  /// contents of the `Success` value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let value = 47;
  /// let x: Outcome<&i32, i32, i32> = Success(&value);
  /// assert_eq!(x, Success(&47));
  /// let copied = x.copied();
  /// assert_eq!(copied, Success(47));
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
  /// # use outcome::prelude::*;
  /// let mut value = 47;
  /// let x: Outcome<&mut i32, i32, i32> = Success(&mut value);
  /// assert_eq!(x, Success(&mut 47));
  /// let copied = x.copied();
  /// assert_eq!(copied, Success(47));
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
  /// # use outcome::prelude::*;
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
  /// # use outcome::prelude::*;
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
  /// let x: Outcome<u32, f32, &str> = Failure("Some error messsage");
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
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Success(47);
  /// assert_eq!(unsafe { x.unwrap_unchecked() }, 47);
  /// ```
  ///
  /// ```no_run
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Mistake(0.0f32);
  /// unsafe { x.unwrap_unchecked(); } // Undefined Behavior!
  /// ```
  ///
  /// ```no_run
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Failure("emergency!");
  /// unsafe { x.unwrap_unchecked(); } // Undefined Behavior!
  /// ```
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
  /// ```no_run
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Success(47);
  /// unsafe { x.unwrap_mistake_unchecked(); } // Undefined Behavior!
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, i32, &str> = Mistake(47);
  /// assert_eq!(unsafe { x.unwrap_mistake_unchecked() }, 47);
  /// ```
  ///
  /// ```no_run
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Failure("emergency!");
  /// unsafe { x.unwrap_mistake_unchecked(); } // Undefined Behavior!!
  /// ```
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
  /// # Examples
  ///
  /// ```no_run
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Success(47);
  /// unsafe { x.unwrap_failure_unchecked(); } // Undefined Behavior!
  /// ```
  ///
  /// ```no_run
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, i32, &str> = Mistake(47);
  /// unsafe { x.unwrap_failure_unchecked() }; // Undefined Behavior!
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Failure("emergency!");
  /// assert_eq!(unsafe { x.unwrap_failure_unchecked() }, "emergency!");
  /// ```
  ///
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
