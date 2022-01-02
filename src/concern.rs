use core::{
  fmt::Debug,
  ops::{Deref, DerefMut},
};

use crate::{iter::*, private::panic};

/// `Concern` is a type that can represent a [`Success`], or [`Mistake`].
///
/// **NOTE**: This type will become a type alias once `!` is stabilized.
///
/// See the [module documentation](crate) for more usage details.
///
/// [`Success`]: Concern::Success
/// [`Mistake`]: Concern::Mistake
/// [`Try`]: core::ops::Try
#[must_use = "This Concern might be a `Mistake`, which should be handled"]
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Concern<S, M> {
  /// Contains the success value
  Success(S),
  /// Contains the mistake value
  Mistake(M),
}

impl<S, M> Concern<S, M> {
  /// Converts from `&Concern<S, M>` to `Concern<&S, &M>`.
  ///
  /// Produces a new `Concern`, containing a reference into the original,
  /// leaving it in place.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// assert_eq!(x.as_ref(), Concern::Success(&42));
  ///
  /// let x: Concern<u32, i32> = Concern::Mistake(47);
  /// assert_eq!(x.as_ref(), Concern::Mistake(&47));
  /// ```
  #[inline]
  pub fn as_ref(&self) -> Concern<&S, &M> {
    match *self {
      Self::Success(ref value) => Concern::Success(value),
      Self::Mistake(ref value) => Concern::Mistake(value),
    }
  }

  /// Converts from `&mut Concern<S, M>` to `Concern<&mut S, &mut F>`
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// fn mutate(c: &mut Concern<u32, i32>) {
  ///   match c.as_mut() {
  ///     Concern::Success(s) => *s = 47,
  ///     Concern::Mistake(m) => *m = 19,
  ///   }
  /// }
  ///
  /// let mut x: Concern<u32, i32> = Concern::Success(42);
  /// mutate(&mut x);
  /// assert_eq!(x.unwrap(), 47);
  ///
  /// let mut x: Concern<u32, i32> = Concern::Mistake(47);
  /// mutate(&mut x);
  /// assert_eq!(x.unwrap_mistake(), 19);
  /// ```
  #[inline]
  pub fn as_mut(&mut self) -> Concern<&mut S, &mut M> {
    match *self {
      Self::Success(ref mut value) => Concern::Success(value),
      Self::Mistake(ref mut value) => Concern::Mistake(value),
    }
  }

  /// Returns an iterator over the possibly contained value.
  ///
  /// The iterator yields one value if the outcome is [`Success`], otherwise
  /// none.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, i32> = Concern::Success(42);
  /// assert_eq!(x.iter().next(), Some(&42));
  ///
  /// let x: Concern<u32, i32> = Concern::Mistake(47);
  /// assert_eq!(x.iter().next(), None);
  /// ```
  ///
  /// [`Success`]: Concern::Success
  #[inline]
  pub fn iter(&self) -> Iter<'_, S> {
    Iter {
      inner: self.as_ref().success(),
    }
  }

  /// Returns a mutable iterator over the possibly contained value.
  ///
  /// The iterator yields one value if the result is [`Success`], otherwise
  /// none.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let mut x: Concern<i32, &str> = Concern::Success(7);
  /// match x.iter_mut().next() {
  ///   Some(v) => *v += 40,
  ///   None => {}
  /// }
  /// assert_eq!(x, Concern::Success(47));
  /// ```
  ///
  /// [`Success`]: Concern::Success
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut<'_, S> {
    IterMut {
      inner: self.as_mut().success(),
    }
  }

  /// Returns `true` if the concern is a [`Success`]
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// assert!(x.is_success());
  ///
  /// let x: Concern<u32, &str> = Concern::Mistake("hello");
  /// assert!(!x.is_success());
  /// ```
  ///
  /// [`Success`]: Concern::Success
  #[must_use = "if you intended to assert a success, consider `.unwrap()` instead"]
  #[inline]
  pub fn is_success(&self) -> bool {
    if let Self::Success(_) = self {
      return true;
    }
    false
  }

  /// Returns `true` if the concern is a [`Mistake`]
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// assert!(!x.is_mistake());
  ///
  /// let x: Concern<u32, &str> = Concern::Mistake("hello");
  /// assert!(x.is_mistake());
  /// ```
  ///
  /// [`Mistake`]: Concern::Mistake
  #[must_use = "if you intended to assert a mistake, consider `.unwrap_mistake()` instead"]
  #[inline]
  pub fn is_mistake(&self) -> bool {
    if let Self::Mistake(_) = self {
      return true;
    }
    false
  }

  /// Converts from `Concern<S, M>` to [`Option<S>`]
  ///
  /// Converts `self` into an [`Option<S>`], consuming `self`, and discarding
  /// the mistake, if any.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// assert_eq!(x.success(), Some(42));
  ///
  /// let x: Concern<u32, &str> = Concern::Mistake("hello");
  /// assert_eq!(x.success(), None);
  /// ```
  #[inline]
  pub fn success(self) -> Option<S> {
    if let Self::Success(value) = self {
      return Some(value);
    }
    None
  }

  /// Converts from `Concern<S, M>` to [`Option<M>`]
  ///
  /// Converts `self` into an [`Option<M>`], consuming `self`, and discarding
  /// the success, if any.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// assert_eq!(x.mistake(), None);
  ///
  /// let x: Concern<u32, &str> = Concern::Mistake("hello");
  /// assert_eq!(x.mistake(), Some("hello"));
  /// ```
  #[inline]
  pub fn mistake(self) -> Option<M> {
    if let Self::Mistake(value) = self {
      return Some(value);
    }
    None
  }

  /// Maps a `Concern<S, M>` to `Concern<T, F>` by applying a function to a
  /// contained [`Success`] value, leaving any [`Mistake`] value untouched.
  ///
  /// [`Success`]: Concern::Success
  /// [`Mistake`]: Concern::Mistake
  #[inline]
  pub fn map<T, C>(self, callable: C) -> Concern<T, M>
  where
    C: FnOnce(S) -> T,
  {
    match self {
      Self::Success(value) => Concern::Success(callable(value)),
      Self::Mistake(value) => Concern::Mistake(value),
    }
  }

  /// Maps a `Concern<S, M>` to `Concern<S, N>` by applying a function to a
  /// contained [`Mistake`] value, leaving any [`Success`] value untouched.
  ///
  /// [`Success`]: Concern::Success
  /// [`Mistake`]: Concern::Mistake
  #[inline]
  pub fn map_mistake<N, C>(self, callable: C) -> Concern<S, N>
  where
    C: FnOnce(M) -> N,
  {
    match self {
      Self::Success(value) => Concern::Success(value),
      Self::Mistake(value) => Concern::Mistake(callable(value)),
    }
  }
}

impl<S, M: Debug> Concern<S, M> {
  /// Returns the contained [`Success`] value, consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is a [`Mistake`] with a panic message provided by
  /// their value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// assert_eq!(x.unwrap(), 42);
  /// ```
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Mistake("hello");
  /// x.unwrap(); // panics with "hello"
  /// ```
  ///
  /// [`Success`]: Concern::Success
  #[track_caller]
  #[inline]
  pub fn unwrap(self) -> S {
    match self {
      Self::Success(s) => s,
      Self::Mistake(m) => panic("Concern::unwrap()", "Mistake", &m),
    }
  }
}

impl<S: Debug, M> Concern<S, M> {
  /// Returns the contained [`Mistake`] value, consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is a [`Success`] with a panic message provided by
  /// their value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Mistake("hello");
  /// assert_eq!(x.unwrap_mistake(), "hello");
  /// ```
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Concern<u32, &str> = Concern::Success(42);
  /// x.unwrap_mistake(); // panics with "42"
  /// ```
  ///
  /// [`Success`]: Concern::Success
  /// [`Mistake`]: Concern::Mistake
  #[track_caller]
  #[inline]
  pub fn unwrap_mistake(self) -> M {
    match self {
      Self::Success(s) => panic("Concern::unwrap_mistake()", "Success", &s),
      Self::Mistake(m) => m,
    }
  }
}

impl<S: Deref, M> Concern<S, M> {
  /// Converts from `Concern<S, M>` (or `&Concern<S, M>`) to `Concern<&<S as
  /// Deref>::Target, M>`.
  ///
  /// Coerces the [`Success`] variant of the original [`Concern`] via [`Deref`]
  /// and returns the new [`Concern`].
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Concern<String, u32> = Concern::Success("hello".to_string());
  /// let y: Concern<&str, &u32> = Concern::Success("hello");
  /// assert_eq!(x.as_deref(), y);
  /// ```
  ///
  /// [`Success`]: Concern::Success
  /// [`Deref`]: core::ops::Deref
  pub fn as_deref(&self) -> Concern<&S::Target, &M> {
    self.as_ref().map(Deref::deref)
  }
}

impl<S: DerefMut, M> Concern<S, M> {
  /// Converts from `Concern<S, M>` (or `&mut Concern<S, M>`) to
  /// `Concern<&mut <S as DerefMut>::Target, &mut M>`.
  ///
  /// Coerces the [`Success`] variant of the original [`Concern`] via
  /// [`DerefMut`] and returns the new [`Concern`]
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let mut s = "HELLO".to_string();
  /// let mut x: Concern<String, u32> = Concern::Success("hello".to_string());
  /// let y: Concern<&mut str, &mut u32> = Concern::Success(&mut s);
  /// assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
  /// ```
  ///
  /// [`DerefMut`]: core::ops::DerefMut
  /// [`Success`]: Concern::Success
  pub fn as_deref_mut(&mut self) -> Concern<&mut S::Target, &mut M> {
    self.as_mut().map(DerefMut::deref_mut)
  }
}

impl<S: Clone, M: Clone> Clone for Concern<S, M> {
  #[inline]
  fn clone(&self) -> Self {
    match self {
      Self::Success(value) => Self::Success(value.clone()),
      Self::Mistake(value) => Self::Mistake(value.clone()),
    }
  }

  #[inline]
  fn clone_from(&mut self, source: &Self) {
    match (self, source) {
      (Self::Success(to), Self::Success(from)) => to.clone_from(from),
      (Self::Mistake(to), Self::Mistake(from)) => to.clone_from(from),
      (to, from) => *to = from.clone(),
    }
  }
}
