use core::{
  fmt::Debug,
  ops::{Deref, DerefMut},
};

#[doc(hidden)]
pub use crate::iter::*;
use crate::{aberration::*, concern::*, private::*};

// TODO: Add an 'aggregate' set of functions (aggregate(_(mistake|failure))?)
// to collect all success, mistake or failure into iterators/partition an
// iterable of failures, concerns, mistakes, etc.
//
// TODO: Add an aggregate_reports function in crate::report

/// `Outcome` is a type that represents a [`Success`], [`Mistake`], or
/// [`Failure`].
///
/// See the [module documentation](crate) for more details.
///
/// # Example
///
/// The following example shows using Outcome to wrap [`Mutex<T>`] to create a
/// spin lock with [exponential backoff][1], that will not block and is adapted
/// from the C++ code in the blost post [*Using locks in real-time audio
/// processing, safely*][2].
///
/// This is *not* meant to be an example of good API design, but to show how
/// [`Outcome`] can be used to make retryable APIs easier to work with.
///
/// ```
/// # use outcome::prelude::*;
/// use std::sync::{Mutex, MutexGuard, PoisonError, LockResult, TryLockError};
/// #[cfg(target_arch = "x86_64")]
/// use std::arch::x86_64::_mm_pause;
/// #[cfg(target_arch = "x86")]
/// use std::arch::x86::_mm_pause;
///
/// #[cfg(not(any(target_arch = "x86_64", target_arch="x86")))]
/// #[inline(never)]
/// unsafe fn _mm_pause() { }
///
/// struct WouldBlock;
///
/// struct SpinMutex<T: ?Sized> {
///   inner: Mutex<T>,
/// }
///
/// type TryLockOutcome<'a, T> = Outcome<
///   MutexGuard<'a, T>,
///   WouldBlock,
///   PoisonError<MutexGuard<'a, T>>
/// >;
///
/// impl<T> SpinMutex<T> {
///   pub fn try_lock(&self) -> TryLockOutcome<T> {
///     match self.inner.try_lock() {
///       Err(TryLockError::Poisoned(f)) => Failure(f),
///       Err(TryLockError::WouldBlock) => Mistake(WouldBlock),
///       Ok(s) => Success(s),
///     }
///   }
///
///   pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
///     for _ in 0..5 {
///       match self.try_lock() {
///         Success(s) => { return Ok(s); }
///         Mistake(_) => { continue; }
///         Failure(f) => { return Err(f); }
///       }
///     }
///
///     for _ in 0..10 {
///       match self.try_lock() {
///         Success(s) => { return Ok(s); }
///         Mistake(_) => { unsafe { _mm_pause(); } }
///         Failure(f) => { return Err(f); }
///       }
///     }
///
///     let mut n = 0;
///     loop {
///       for _ in 0..3000 {
///         match self.try_lock() {
///           Success(s) => { return Ok(s); }
///           Mistake(_) => {
///             for _ in 0..10 { unsafe { _mm_pause(); } }
///             continue;
///           }
///           Failure(f) => { return Err(f); }
///         }
///       }
///       std::thread::yield_now();
///       n += 1;
///       if n >= 2 { break self.inner.lock(); }
///     }
///   }
/// }
/// ```
/// [`Mutex<T>`]: std::sync::Mutex
///
/// [1]: https://en.wikipedia.org/wiki/Exponential_backoff
/// [2]: https://timur.audio/using-locks-in-real-time-audio-processing-safely
#[must_use = "This `Outcome` might not be a `Success`, which should be handled"]
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Outcome<S, M, F> {
  /// Contains the success value
  Success(S),
  /// Contains the mistake value
  Mistake(M),
  /// Contains the failure value
  Failure(F),
}

use Outcome::{Failure, Mistake, Success};

impl<S, M, F> Outcome<S, M, F> {
  /// Converts from `&Outcome<S, M, F>` to `Outcome<&S, &M, &F>`.
  ///
  /// Produces a new `Outcome`, containing a reference into the original,
  /// leaving the original in place.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Success(2);
  /// assert_eq!(x.as_ref(), Success(&2));
  ///
  /// let x: Outcome<i32, i32, i32> = Mistake(47);
  /// assert_eq!(x.as_ref(), Mistake(&47));
  ///
  /// let x: Outcome<i32, i32, i32> = Failure(42);
  /// assert_eq!(x.as_ref(), Failure(&42));
  /// ```
  #[inline]
  pub fn as_ref(&self) -> Outcome<&S, &M, &F> {
    match *self {
      Success(ref value) => Success(value),
      Mistake(ref value) => Mistake(value),
      Failure(ref value) => Failure(value),
    }
  }

  /// Converts from `&mut Outcome<S, M, F>` to `Outcome<&mut S, &mut M, &mut
  /// F>`.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// fn mutate(o: &mut Outcome<i32, i32, i32>) {
  ///   match o.as_mut() {
  ///     Success(s) => *s = 47,
  ///     Mistake(m) => *m = 19,
  ///     Failure(f) => *f = 0,
  ///   }
  /// }
  ///
  /// let mut x: Outcome<i32, i32, i32> = Success(2);
  /// mutate(&mut x);
  /// assert_eq!(x.unwrap(), 47);
  ///
  /// let mut x: Outcome<i32, i32, i32> = Mistake(47);
  /// mutate(&mut x);
  /// assert_eq!(x.unwrap_mistake(), 19);
  ///
  /// let mut x: Outcome<i32, i32, i32> = Failure(47);
  /// mutate(&mut x);
  /// assert_eq!(x.unwrap_failure(), 0);
  /// ```
  #[inline]
  pub fn as_mut(&mut self) -> Outcome<&mut S, &mut M, &mut F> {
    match *self {
      Success(ref mut value) => Success(value),
      Mistake(ref mut value) => Mistake(value),
      Failure(ref mut value) => Failure(value),
    }
  }

  /// Returns a `Result<Concern<S, M>, F>`, which allows a user to still rely
  /// on the `?` operator until [`Try`] has been stabilized.
  ///
  /// [`Try`]: core::ops::Try
  #[inline]
  pub fn acclimate(self) -> Result<Concern<S, M>, F> {
    match self {
      Success(value) => Ok(Concern::Success(value)),
      Mistake(value) => Ok(Concern::Mistake(value)),
      Failure(value) => Err(value),
    }
  }

  /// Returns an iterator over the possibly contained value.
  ///
  /// The iterators yields one value if the outcome is [`Success`], otherwise
  /// none.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<i32, f32, &str> = Success(47);
  /// assert_eq!(x.iter().next(), Some(&47));
  ///
  /// let x: Outcome<i32, f32, &str> = Mistake(0.0f32);
  /// assert_eq!(x.iter().next(), None);
  ///
  /// let x: Outcome<i32, f32, &str> = Failure("nope!");
  /// assert_eq!(x.iter().next(), None);
  /// ```
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
  /// let mut x: Outcome<i32, f32, &str> = Success(7);
  /// match x.iter_mut().next() {
  ///   Some(v) => *v += 40,
  ///   None => {}
  /// }
  /// assert_eq!(x, Success(47));
  /// ```
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut<'_, S> {
    IterMut {
      inner: self.as_mut().success(),
    }
  }

  /// Returns `true` if the outcome is [`Success`].
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<i32, (), &str> = Success(-1);
  /// assert_eq!(x.is_success(), true);
  ///
  /// let x: Outcome<i32, (), &str> = Mistake(());
  /// assert_eq!(x.is_success(), false);
  ///
  /// let x: Outcome<i32, (), &str> = Failure("Some failure message");
  /// assert_eq!(x.is_success(), false);
  /// ```
  ///
  #[must_use = "if you intended to assert a success, consider `.unwrap()` instead"]
  #[inline]
  pub fn is_success(&self) -> bool {
    if let Success(_) = self {
      return true;
    }
    false
  }

  /// Returns `true` if the outcome is [`Mistake`].
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<(), i32, &str> = Mistake(-1);
  /// assert_eq!(x.is_mistake(), true);
  ///
  /// let x: Outcome<(), i32, &str> = Success(());
  /// assert_eq!(x.is_mistake(), false);
  ///
  /// let x: Outcome<(), i32, &str> = Failure("Some failure message");
  /// assert_eq!(x.is_mistake(), false);
  /// ```
  #[must_use = "if you intended to assert a mistake, consider `.unwrap_mistake()` instead"]
  #[inline]
  pub fn is_mistake(&self) -> bool {
    if let Mistake(_) = self {
      return true;
    }
    false
  }

  /// Returns `true` if the outcome is [`Failure`].
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<i32, f32, &str> = Failure("some failure message");
  /// assert_eq!(x.is_failure(), true);
  ///
  /// let x: Outcome<i32, f32, &str> = Mistake(0.0f32);
  /// assert_eq!(x.is_failure(), false);
  ///
  /// let x: Outcome<i32, f32, &str> = Success(-1);
  /// assert_eq!(x.is_failure(), false);
  /// ```
  #[must_use = "if you intended to assert a failure, consider `.unwrap_failure()` instead"]
  #[inline]
  pub fn is_failure(&self) -> bool {
    if let Failure(_) = self {
      return true;
    }
    false
  }

  /// Returns `true` if the outcome is *not* [`Success`]
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<i32, f32, &str> = Failure("some failure message");
  /// assert_eq!(x.is_error(), true);
  ///
  /// let x: Outcome<i32, f32, &str> = Mistake(0.0f32);
  /// assert_eq!(x.is_error(), true);
  ///
  /// let x: Outcome<i32, f32, &str> = Success(-1);
  /// assert_eq!(x.is_error(), false);
  /// ```
  #[must_use = "If you intended to assert an error, consider `.unwrap_error()` instead"]
  #[inline]
  pub fn is_error(&self) -> bool {
    !self.is_success()
  }

  /// Converts from `Outcome<S, M, F>` to [`Option<S>`].
  ///
  /// Converts `self` into an [`Option<S>`], consuming `self`, and discarding
  /// the mistake or failure, if any.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let outcome: Outcome<i32, f32, &str> = Success(4);
  /// assert_eq!(outcome.success(), Some(4));
  ///
  /// let outcome: Outcome<i32, f32, &str> = Mistake(0.0);
  /// assert_eq!(outcome.success(), None);
  ///
  /// let outcome: Outcome<i32, f32, &str> = Failure("failure");
  /// assert_eq!(outcome.success(), None);
  /// ```
  #[inline]
  pub fn success(self) -> Option<S> {
    if let Success(value) = self {
      return Some(value);
    }
    None
  }

  /// Converts from `Outcome<S, M, F>` to [`Option<M>`].
  ///
  /// Converts `self` into an [`Option<M>`], consuming `self`, and discarding
  /// the success or failure, if any.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let outcome: Outcome<f32, i32, &str> = Mistake(47);
  /// assert_eq!(outcome.mistake(), Some(47));
  ///
  /// let outcome: Outcome<f32, i32, &str> = Success(0.0);
  /// assert_eq!(outcome.mistake(), None);
  ///
  /// let outcome: Outcome<f32, i32, &str> = Failure("failure");
  /// assert_eq!(outcome.mistake(), None);
  /// ```
  #[inline]
  pub fn mistake(self) -> Option<M> {
    if let Mistake(value) = self {
      return Some(value);
    }
    None
  }

  /// Converts from `Outcome<S, M, F>` to [`Option<F>`].
  ///
  /// Converts `self` into an [`Option<F>`], consuming `self`, and discarding
  /// the success or mistake, if any.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let outcome: Outcome<f32, (), i32> = Success(0.0);
  /// assert_eq!(outcome.failure(), None);
  ///
  /// let outcome: Outcome<f32, (), i32> = Mistake(());
  /// assert_eq!(outcome.failure(), None);
  ///
  /// let outcome: Outcome<f32, (), i32> = Failure(-1);
  /// assert_eq!(outcome.failure(), Some(-1));
  /// ```
  #[inline]
  pub fn failure(self) -> Option<F> {
    if let Failure(value) = self {
      return Some(value);
    }
    None
  }

  /// Calls `op` if the result is [`Success`], otherwise returns the
  /// [`Mistake`] or [`Failure`] value of `self`.
  ///
  /// This function can be used for control flow based on `Outcome` values.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_variables)]
  /// # use outcome::prelude::*;
  ///
  /// fn square(x: u32) -> Outcome<u32, u32, u32> { Success(x * x) }
  /// fn mistake(x: u32) -> Outcome<u32, u32, u32> { Mistake(x) }
  /// fn failure(x: u32) -> Outcome<u32, u32, u32> { Failure(0) }
  ///
  /// assert_eq!(Success(2).and_then(square).and_then(square), Success(16));
  /// assert_eq!(Success(2).and_then(square).and_then(failure), Failure(0));
  /// assert_eq!(Success(2).and_then(square).and_then(mistake), Mistake(4));
  /// assert_eq!(Failure(2).and_then(square).and_then(square), Failure(2));
  /// ```
  #[inline]
  pub fn and_then<T, C>(self, callable: C) -> Outcome<T, M, F>
  where
    C: FnOnce(S) -> Outcome<T, M, F>,
  {
    match self {
      Success(value) => callable(value),
      Mistake(value) => Mistake(value),
      Failure(value) => Failure(value),
    }
  }

  /// Maps an `Outcome<S, M, F>` to `Outcome<T, M, F>` by applying a function
  /// to a contained [`Success`] value, leaving any [`Mistake`] or [`Failure`]
  /// value untouched.
  ///
  /// This function can be used to compose the results of two functions.
  #[inline]
  pub fn map<T, C>(self, callable: C) -> Outcome<T, M, F>
  where
    C: FnOnce(S) -> T,
  {
    match self {
      Success(s) => Success(callable(s)),
      Mistake(m) => Mistake(m),
      Failure(f) => Failure(f),
    }
  }

  /// Returns the provided default (if [`Mistake`] or [`Failure`]), or applies
  /// a function to the contained value (if [`Success`]).
  ///
  /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
  /// the result of a function call, it is recommended to use [`map_or_else`],
  /// which is lazily evaluated.
  ///
  /// [`map_or_else`]: Outcome::map_or_else
  #[inline]
  pub fn map_or<T, C>(self, default: T, callable: C) -> T
  where
    C: FnOnce(S) -> T,
  {
    match self {
      Success(value) => callable(value),
      _ => default,
    }
  }

  /// Maps an `Outcome<S, M, F>` to `T` by applying a fallback function to a
  /// contained [`Mistake`] or [`Failure`] value (by way of an [`Aberration`]),
  /// or a default function to a contained [`Success`] value.
  ///
  /// This function can be used to unpack a successful outcome while handling
  /// mistakes or failures.
  #[inline]
  pub fn map_or_else<T, D, C>(self, default: D, callable: C) -> T
  where
    D: FnOnce(Aberration<M, F>) -> T,
    C: FnOnce(S) -> T,
  {
    match self {
      Success(value) => callable(value),
      Mistake(value) => default(Aberration::Mistake(value)),
      Failure(value) => default(Aberration::Failure(value)),
    }
  }

  /// Maps an `Outcome<S, M, F>` to `Outcome<S, N, F>` by applying a function to
  /// a contained [`Mistake`] value, leaving a [`Success`] or [`Failure`] value
  /// untouched.
  ///
  /// This function can be used to pass through a successful outcome while
  /// handling an error.
  #[inline]
  pub fn map_mistake<N, C>(self, callable: C) -> Outcome<S, N, F>
  where
    C: FnOnce(M) -> N,
  {
    match self {
      Success(value) => Success(value),
      Mistake(value) => Mistake(callable(value)),
      Failure(value) => Failure(value),
    }
  }

  /// Maps an `Outcome<S, M, F>` to `Outcome<S, M, G>` by applying a function
  /// to a contained [`Failure`] value, leaving a [`Success`] or [`Failure`]
  /// value untouched.
  ///
  /// This function can be used to pass through a successful outcome while
  /// handling an error.
  #[inline]
  pub fn map_failure<G, C>(self, callable: C) -> Outcome<S, M, G>
  where
    C: FnOnce(F) -> G,
  {
    match self {
      Success(value) => Success(value),
      Mistake(value) => Mistake(value),
      Failure(value) => Failure(callable(value)),
    }
  }
}

/* special interfaces */
#[cfg(not(feature = "nightly"))]
impl<S, M, F> Outcome<S, M, F> {
  /// **`TODO`**: Write documentation
  pub fn escalate_with<C, T>(self, closure: C) -> Aberration<M, F>
  where
    T: Into<M>,
    C: FnOnce(S) -> T,
  {
    match self {
      Success(s) => Aberration::Mistake(closure(s).into()),
      Mistake(m) => Aberration::Mistake(m),
      Failure(f) => Aberration::Failure(f),
    }
  }
}

#[cfg(not(feature = "nightly"))]
impl<S, M, F> Outcome<S, M, F>
where
  S: Into<M>,
  M: Into<F>,
{
  /// Escalates the state of the Outcome from Success, to Mistake, to Failure
  /// on each call.
  ///
  /// Once an Outcome is in a failure state, it cannot escalate any further.
  pub fn escalate(self) -> Aberration<M, F> {
    match self {
      Success(s) => Aberration::Mistake(s.into()),
      Mistake(m) => Aberration::Failure(m.into()),
      Failure(f) => Aberration::Failure(f),
    }
  }
}

impl<S: Deref, M, F> Outcome<S, M, F> {
  /// Converts from `Outcome<S, M, F>` (or `&Outcome<S, M, F>`) to `Outcome<&<S
  /// as Deref>::Target, M, F>`.
  ///
  /// Coerces the [`Success`] variant of the original [`Outcome`] via [`Deref`]
  /// and returns the new [`Outcome`].
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<String, u32, u32> = Success("hello".to_string());
  /// let y: Outcome<&str, &u32, &u32> = Success("hello");
  /// assert_eq!(x.as_deref(), y);
  /// ```
  ///
  /// [`Deref`]: core::ops::Deref
  pub fn as_deref(&self) -> Outcome<&S::Target, &M, &F> {
    self.as_ref().map(Deref::deref)
  }
}

impl<S: DerefMut, M, F> Outcome<S, M, F> {
  /// Converts from `Outcome<S, M, F>` (or `&mut Outcome<S, M, F>`) to
  /// `Outcome<&mut <S as DerefMut>::Target, &mut M, &mut F>`.
  ///
  /// Coerces the [`Success`] variant of the original [`Outcome`] via
  /// [`DerefMut`] and returns the new [`Outcome`].
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let mut s = "HELLO".to_string();
  /// let mut x: Outcome<String, u32, u32> = Success("hello".to_string());
  /// let y: Outcome<&mut str, &mut u32, &mut u32> = Success(&mut s);
  /// assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
  /// ```
  ///
  /// [`DerefMut`]: core::ops::DerefMut
  pub fn as_deref_mut(&mut self) -> Outcome<&mut S::Target, &mut M, &mut F> {
    self.as_mut().map(DerefMut::deref_mut)
  }
}

impl<S, M: Debug, F: Debug> Outcome<S, M, F> {
  /// Returns the contained [`Success`] value, consuming the `self` value.
  ///
  /// Because this function may panic, its use is generally discourged.
  /// Instead, prefer to use pattern matching and handle the [`Mistake`] or
  /// [`Failure`] case explicitly, or call [`unwrap_or`], [`unwrap_or_else`],
  /// or [`unwrap_or_default`].
  ///
  /// # Panics
  ///
  /// Panics if the value is a [`Mistake`] or [`Failure`], wth a panic message
  /// provided by their value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, &str> = Success(2);
  /// assert_eq!(x.unwrap(), 2);
  /// ```
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, &str> = Failure("emergency failure");
  /// x.unwrap(); // panics with "emergency failure"
  /// ```
  ///
  /// [`unwrap_or_default`]: Outcome::unwrap_or_default
  /// [`unwrap_or_else`]: Outcome::unwrap_or_else
  /// [`unwrap_or`]: Outcome::unwrap_or
  #[track_caller]
  #[inline]
  pub fn unwrap(self) -> S {
    match self {
      Success(s) => s,
      Mistake(m) => panic("Outcome::unwrap()", "Mistake", &m),
      Failure(f) => panic("Outcome::unwrap()", "Failure", &f),
    }
  }

  /// Returns the [`Success`] value or a provided default.
  ///
  /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
  /// the result of a function call, it is recommended to use
  /// [`unwrap_or_else`], which is lazily evaluated.
  ///
  /// [`unwrap_or_else`]: Outcome::unwrap_or_else
  #[track_caller]
  #[inline]
  pub fn unwrap_or(self, default: S) -> S {
    if let Success(success) = self {
      return success;
    }
    default
  }

  /// Returns the contained [`Success`] value or computes it from the closure.
  pub fn unwrap_or_else(self, op: impl FnOnce(Aberration<M, F>) -> S) -> S {
    match self {
      Success(value) => value,
      Mistake(value) => op(Aberration::Mistake(value)),
      Failure(value) => op(Aberration::Failure(value)),
    }
  }
}

impl<S: Debug, M, F: Debug> Outcome<S, M, F> {
  /// Returns the contained [`Mistake`] value, consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is either a [`Success`] or [`Failure`], with a custom
  /// panic message provided by either value.
  ///
  /// # Examples
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, &str> = Success(47);
  /// x.unwrap_mistake(); // panics with '47'
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, f32> = Mistake("try again!");
  /// assert_eq!(x.unwrap_mistake(), "try again!");
  /// ```
  #[track_caller]
  #[inline]
  pub fn unwrap_mistake(self) -> M {
    match self {
      Success(s) => panic("Outcome::unwrap_mistake()", "Success", &s),
      Mistake(m) => m,
      Failure(f) => panic("Outcome::unwrap_mistake()", "Failure", &f),
    }
  }
}

impl<S: Debug, M: Debug, F> Outcome<S, M, F> {
  /// Returns the contained [`Failure`] value, consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is either a [`Success`] or [`Mistake`], with a custom
  /// panic message provided by either value.
  ///
  /// # Examples
  ///
  /// ```should_panic
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, &str> = Success(47);
  /// x.unwrap_failure(); // panics with 47
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, f32, &str> = Failure("failure!");
  /// assert_eq!(x.unwrap_failure(), "failure!");
  /// ```
  #[track_caller]
  #[inline]
  pub fn unwrap_failure(self) -> F {
    match self {
      Success(s) => panic("Outcome::unwrap_failure()", "Success", &s),
      Mistake(m) => panic("Outcome::unwrap_failure()", "Mistake", &m),
      Failure(f) => f,
    }
  }
}

impl<S: Debug, M, F> Outcome<S, M, F> {
  /// Returns the contained [`Mistake`] or [`Failure`] value wrapped in an
  /// [`Aberration`], consuming the `self` value.
  ///
  /// # Panics
  ///
  /// Panics if the value is a [`Success`], with a custom panic message
  /// provided by the contained value.
  ///
  /// # Examples
  ///
  /// ```should_panic
  /// # #![allow(unused_must_use)]
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, &str> = Success(47);
  /// x.unwrap_error(); // panics with '47'
  /// ```
  ///
  /// ```
  /// # use outcome::prelude::*;
  /// let x: Outcome<u32, &str, &str> = Failure("failure!");
  /// let ab = x.unwrap_error();
  /// match ab {
  ///   Aberration::Mistake(m) => assert_eq!("mistake!", m),
  ///   Aberration::Failure(f) => assert_eq!("failure!", f),
  /// };
  /// ```
  #[track_caller]
  #[inline]
  pub fn unwrap_error(self) -> Aberration<M, F> {
    match self {
      Success(value) => panic("Outcome::unwrap_error()", "Success", &value),
      Mistake(value) => Aberration::Mistake(value),
      Failure(value) => Aberration::Failure(value),
    }
  }
}

impl<S: Default, M, F> Outcome<S, M, F> {
  /// Returns the contained [`Success`] value or a default.
  ///
  /// Consumes the `self` argument then, if [`Success`], returns the contained
  /// value, otherwise if the outcome is a [`Mistake`] or [`Failure`], returns
  /// the default value for [`Success`]
  #[track_caller]
  #[inline]
  pub fn unwrap_or_default(self) -> S {
    if let Success(success) = self {
      return success;
    }
    S::default()
  }
}

impl<S, M, F> Outcome<Option<S>, M, F> {
  /// Transposes an `Outcome` of an `Option` into an `Option` of an `Outcome`.
  ///
  /// - `Success(None)` will be mapped to `None`.
  /// - `Success(Some(_))`, `Mistake(_)`, and `Failure(_)` will be mapped to
  ///     `Some(Success(_))`, `Some(Mistake(_))`, and `Some(Failure(_))`.
  ///
  pub fn transpose(self) -> Option<Outcome<S, M, F>> {
    match self {
      Success(Some(s)) => Some(Success(s)),
      Success(None) => None,
      Mistake(m) => Some(Mistake(m)),
      Failure(f) => Some(Failure(f)),
    }
  }
}

/* Builtin Trait Implementations */
impl<S: Clone, M: Clone, F: Clone> Clone for Outcome<S, M, F> {
  #[inline]
  fn clone(&self) -> Self {
    match self {
      Success(value) => Success(value.clone()),
      Mistake(value) => Mistake(value.clone()),
      Failure(value) => Failure(value.clone()),
    }
  }

  #[inline]
  fn clone_from(&mut self, source: &Self) {
    match (self, source) {
      (Success(to), Success(from)) => to.clone_from(from),
      (Mistake(to), Mistake(from)) => to.clone_from(from),
      (Failure(to), Failure(from)) => to.clone_from(from),
      (to, from) => *to = from.clone(),
    }
  }
}

#[cfg(all(test, feature = "std"))]
mod tests {
  extern crate std;
  use super::*;
  use std::{string::String, vec, vec::Vec};

  #[test]
  fn filter_map_with() {
    let failures: Vec<Outcome<(), (), String>> = vec![
      Failure("There is an error".into()),
      Mistake(()),
      Failure("There is a second error".into()),
      Success(()),
      Failure("There is a final error".into()),
      Mistake(()),
      Success(()),
    ];

    let filtered: Vec<&str> = failures
      .iter()
      .map(Outcome::as_ref)
      .filter_map(Outcome::failure)
      .map(String::as_str)
      .collect();

    assert_eq!(filtered.len(), 3);
    assert_eq!(failures[0].as_ref().unwrap_failure().as_str(), filtered[0]);
    assert_eq!(failures[2].as_ref().unwrap_failure().as_str(), filtered[1]);
    assert_eq!(failures[4].as_ref().unwrap_failure().as_str(), filtered[2]);
  }
}
