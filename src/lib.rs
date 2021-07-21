//! [`Outcome<S, M, F>`][`Outcome`] is an augmentation of the [`Result`] type
//! found in the Rust standard library.
//!
//! It is an enum with the variants
//!  - [`Success(S)`], representing success and containing a value
//!  - [`Mistake(M)`], representing an optionally *retryable error* and
//!     containing a value
//!  - [`Failure(F)`], representing failure and containing a value.
//!
//! ```
//! # #[allow(dead_code)]
//! enum Outcome<S, M, F> {
//!   Success(S),
//!   Mistake(M),
//!   Failure(F),
//! }
//! ```
//!
//! [`Outcome`] is an *augmentation* to [`Result`]. It adds a third state to
//! the "success or failure" dichotomy that [`Result<T, E>`] models. This third
//! state is that of a *soft* or *retryable* error. A *retryable* error is one
//! where an operation might not have succeeded, either due to other operations
//! (e.g., a disk read or write not completing), misconfiguration (e.g.,
//! forgetting to set a specific flag before calling a function), or busy
//! resources (e.g., attempting to lock an audio, video, or database resource).
//!
//! ```
//! # use outcome::*;
//! # use std::error::Error;
//! #[derive(Debug, PartialEq)]
//! enum Version { V1, V2 }
//!
//! #[derive(Debug, PartialEq)]
//! struct EmptyInput;
//!
//! fn parse_version(header: &[u8]) -> Outcome<Version, EmptyInput, &'static str> {
//!   match header.get(0) {
//!     None => Mistake(EmptyInput),
//!     Some(&1) => Success(Version::V1),
//!     Some(&2) => Success(Version::V2),
//!     Some(_) => Failure("invalid or unknown version"),
//!   }
//! }
//!
//! let version = parse_version(&[]);
//! assert_eq!(version, Mistake(EmptyInput));
//! ```
//!
//! # Usage
//!
//! At this time, the `outcome` crate is already taken on [crates.io]. As
//! [crates.io] does not yet support namespaces or collections, we've had to
//! take a *unique* approach to still publish the crate. To do this, we've
//! generated a `UUIDv5` string via python:
//!
//! ```python
//! from uuid import *
//! print(uuid5(uuid5(NAMESPACE_DNS, "occult.work"), "outcome"))
//! ```
//!
//! This *should* generate the string `46f94afc-026f-5511-9d7e-7d1fd495fb5c`.
//! Thus the dependency in your `Cargo.toml` will look something like:
//!
//! ```toml
//! [dependencies]
//! outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c = "*"
//! ```
//!
//! Is this solution friendly to users? No, but neither is the lack of
//! namespacing or a squatting policy on [crates.io]. If/when this problem is
//! resolved, this crate's documentation (and name!) will be changed and all
//! versions will be yanked.
//!
//! # Features
//!
//! There are several features available to the crate that are disabled by
//! default. These include:
//!
//!  - `unstable` (Enable "unstable" functions that mirror unstable functions
//!      found in [`Result`]. Unlike [`Result`], however, a nightly compiler is
//!      not required.)
//!  - `nightly` (Enable features that require the nightly rust compiler to be
//!      used, such as [`TryV2`])
//!  - `report` (Enable conversion from [`Aberration`] to an
//!      [`eyre::Report`])
//!
//! Users can also enable `no_std` support by either setting `default-features`
//! to `false` or simply not listing `std` in the list of features.
//!
//!  - `nightly` will enable `unstable`.
//!  - `report` will enable `std`.
//!
//! ### `no_std` support
//!
//! Nearly every single feature in `outcome` supports working with `#![no_std]`
//! support, however currently `eyre` *does* require `std` support (Attempts
//! were made at making `no_std` work, but this was removed and has not been
//! available for some time).
//!
//!
//! ```toml
//! [dependencies]
//! outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c = { version = "...", features = ["nightly"] }
//! ```
//!
//! # Why Augment `Result<T, E>`?
//!
//! [`Outcome`] is *not* intended to fully replace [`Result`], especially at
//! the API boundary (i.e., the API used by clients) when there is a clear
//! success or failure state that can be transferred to users. Instead, it
//! provides the ability to quickly expand the surface area of consumed APIs
//! with finer grained control over errors so that library writers can write
//! *correct* behavior and then return at a later time to compose results,
//! expand error definitions, or to represent different error severities.
//!
//! As an example, the section [making unhandled errors unrepresentable][1] in
//! the post *Error Handling in a Correctness-Critical Rust Project*, the
//! author states:
//!
//! > this led me to go for what felt like the nuclear solution, but after
//! > seeing how many bugs it immediately rooted out by simply refactoring the
//! > codebase, I’m convinced that this is the only way to do error handling in
//! > systems where we have multiple error handling concerns in Rust today.
//!
//! The solution, as they explain in the next paragraph is
//!
//! > make the global `Error` enum specifically only hold errors that should
//! > cause the overall system to halt - reserved for situations that require
//! > human intervention. Keep errors which relate to separate concerns in
//! > totally separate error types. By keeping errors that must be handled
//! > separately in their own types, we reduce the chance that the try `?`
//! > operator will accidentally push a local concern into a caller that can’t
//! > deal with it.
//!
//! As the author of this post later shows, the `Sled::compare_and_swap`
//! function returns a `Result<Result<(), CompareAndSwapError>, sled::Error>`.
//! They state this looks "*way less cute*", but will
//!
//! > improve chances that users will properly handle their compare and
//! > swap-related errors properly\[sic]
//! > ```ignore
//! > // we can actually use try `?` now
//! > let cas_result = sled.compare_and_swap(
//! >   "dogs",
//! >   "pickles",
//! >   "catfood"
//! > )?;
//! >
//! > if let Err(cas_error) = cas_result {
//! >     // handle expected issue
//! > }
//! > ```
//!
//! The issue with this return type is that there is *technically nothing* to
//! stop a user from using what the creator of this crate calls the WTF
//! operator (`??`) to ignore these intermediate errors.
//!
//! ```ignore
//! let cas = sled.compare_and_swap("dogs", "pickles", "catfood")??;
//! ```
//!
//! It would be hard to *forbid* this kind of usage with tools like clippy due
//! to libraries such as [nom][2] relying on nested results and expecting
//! moderately complex pattern matching to extract relevant information.
//!
//! Luckily, it *is* easier to prevent this issue in the first place if:
//!
//!  - An explicit call to extract an inner `Result<T, E>` must be made
//!  - The call of an easily greppable/searchable function before using the
//!      "WTF" (`??`) operator is permitted.
//!  - The [`Try`] or [`TryV2`] trait returns a type that *must* be decomposed
//!      explicitly and *does not support* the try `?` operator itself.
//!
//! Thanks to [clippy](https://github.com/rust-lang/rust-clippy)'s
//! `disallowed_method` lint, users can rely on the first two options until
//! [`TryV2`] has been stabilized.
//!
//! # State Escalation
//!
//! // TODO: ...
//!
//! [`Success(S)`]: Success
//! [`Mistake(M)`]: Mistake
//! [`Failure(F)`]: Failure
//!
//! [`TryLockError<T>`]: std::sync::TryLockError
//! [`PoisonError<T>`]: std::sync::PoisonError
//! [`WouldBlock`]: std::sync::TryLockError::WouldBlock
//!
//! [`UnixDatagram::take_error`]: https://doc.rust-lang.org/nightly/std/os/unix/net/struct.UnixDatagram.html#method.take_error
//! [`TryV2`]: core::ops::TryV2
//! [`Try`]: core::ops::Try
//!
//! [crates.io]: https://crates.io
//!
//! [1]: https://sled.rs/errors.html#making-unhandled-errors-unrepresentable
//! [2]: https://crates.io/crates/nom

#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::let_underscore_drop)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::redundant_else)]
#![warn(clippy::single_match_else)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_self)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
#![warn(missing_docs)]
#![warn(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(doc)]
extern crate std;

use core::{
  fmt::Debug,
  ops::{Deref, DerefMut},
};

#[cfg_attr(docsrs, doc(cfg(feature = "report")))]
#[cfg(feature = "report")]
mod report;

#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[cfg(feature = "unstable")]
mod unstable;

#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
#[cfg(feature = "nightly")]
mod nightly;

mod convert;
mod iter;
mod stable;

/// `Outcome` is a type that can represet a [`Success`], [`Mistake`], or [`Failure`].
///
/// See the [module documentation](self) for details.
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

#[doc(inline)]
pub use crate::{convert::*, iter::*};

#[doc(inline)]
pub use crate::stable::{Aberration, Concern};

#[doc(hidden)]
pub use Outcome::{Failure, Mistake, Success};

impl<S, M, F> Outcome<S, M, F> {
  /// Converts from `&Outcome<S, M, F>` to `Outcome<&S, &M, &F>`.
  ///
  /// Produces a new `Outcome`, containing a reference into the original,
  /// leaving the original in place.
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
  #[inline]
  pub fn as_mut(&mut self) -> Outcome<&mut S, &mut M, &mut F> {
    match *self {
      Success(ref mut value) => Success(value),
      Mistake(ref mut value) => Mistake(value),
      Failure(ref mut value) => Failure(value),
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
  /// # use outcome::*;
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

  /// Returns a mutable iterator over the possibly container value.
  ///
  /// The iterator yields one value if the result is [`Success`], otherwise
  /// none.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # use outcome::*;
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
  /// # use outcome::*;
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
  /// # use outcome::*;
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
  /// # use outcome::*;
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
  /// # use outcome::*;
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
  /// ```
  /// # use outcome::*;
  /// let outcome: Outcome<i32, f32, &str> = Success(4);
  /// assert_eq!(outcome.success(), Some(4));
  ///
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
  /// the success or failure, if any
  ///
  /// ```
  /// # use outcome::*;
  /// let outcome: Outcome<f32, i32, &str> = Mistake(47);
  /// assert_eq!(outcome.mistake(), Some(47));
  ///
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
  /// ```
  /// # use outcome::*;
  /// let outcome: Outcome<f32, (), i32> = Failure(-1);
  /// assert_eq!(outcome.failure(), Some(-1));
  ///
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
  /// # use outcome::*;
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
  /// use outcome::*;
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
  /// # use outcome::*;
  /// let mut s = "HELLO".to_string();
  /// let mut x: Outcome<String, u32, u32> = Success("hello".to_string());
  /// let y: Outcome<&mut str, &mut u32, &mut u32> = Success(&mut s);
  ///assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
  /// ```
  ///
  /// [`DerefMut`]: core::ops::DerefMut
  pub fn as_deref_mut(&mut self) -> Outcome<&mut S::Target, &mut M, &mut F> {
    self.as_mut().map(DerefMut::deref_mut)
  }
}

impl<S, M, F> Outcome<S, M, F>
where
  S: Into<M>,
  M: Into<F>,
{
  /// Escalates the state of the Outcome from Success, to Mistake, to Failure
  /// on each call. Once an Outcome is in a failure state, it cannot escalate
  /// any further.
  pub fn escalate(self) -> Self {
    match self {
      Success(s) => Mistake(s.into()),
      Mistake(m) => Failure(m.into()),
      Failure(f) => Failure(f),
    }
  }

  /// Escalates the state of the Outcome from Success to Mistake to Failure on
  /// each call. If the Outcome is in a failure state when this is called, the
  /// function will panic.
  ///
  /// # Panics
  /// If the outcome is already a [`Failure`], this function will panic.
  #[cfg(feature = "std")]
  pub fn escalate_or_panic(self) -> Self
  where
    F: Debug,
  {
    match self {
      Failure(f) => {
        panic!("Escalation has exceeded safety parameters: {:?}", f)
      }
      _ => self.escalate(),
    }
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
  /// # use outcome::*;
  /// let x: Outcome<u32, &str, &str> = Success(2);
  /// assert_eq!(x.unwrap(), 2);
  /// ```
  ///
  /// ```should_panic
  /// # use outcome::*;
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
      Mistake(m) => {
        panic!("Called `Outcome::unwrap()` on a `Mistake` value: {:?}", m)
      }
      Failure(f) => {
        panic!("Called `Outcome::unwrap()` on a `Failure` value: {:?}", f)
      }
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
  /// # use outcome::*;
  /// let x: Outcome<u32, &str, &str> = Success(47);
  /// x.unwrap_mistake(); // panics with '47'
  /// ```
  ///
  /// ```
  /// # use outcome::*;
  /// let x: Outcome<u32, &str, f32> = Mistake("try again!");
  /// assert_eq!(x.unwrap_mistake(), "try again!");
  /// ```
  #[track_caller]
  #[inline]
  pub fn unwrap_mistake(self) -> M {
    match self {
      Success(s) => panic!(
        "Called `Outcome::unwrap_mistake()` on a `Success` value: {:?}",
        s
      ),
      Mistake(m) => m,
      Failure(f) => panic!(
        "Called `Outcome::unwrap_mistake()` on a `Failure` value: {:?}",
        f
      ),
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
  /// # use outcome::*;
  /// let x: Outcome<u32, &str, &str> = Success(47);
  /// x.unwrap_failure(); // panics with 47
  /// ```
  ///
  /// ```
  /// # use outcome::*;
  /// let x: Outcome<u32, f32, &str> = Failure("failure!");
  /// assert_eq!(x.unwrap_failure(), "failure!");
  /// ```
  #[track_caller]
  #[inline]
  pub fn unwrap_failure(self) -> F {
    match self {
      Success(s) => panic!(
        "Called `Outcome::unwrap_failure()` on a `Success` value: {:?}",
        s
      ),
      Mistake(m) => panic!(
        "Called `Outcome::unwrap_failure()` on a `Mistake` value: {:?}",
        m
      ),
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
  /// # use outcome::*;
  /// let x: Outcome<u32, &str, &str> = Success(47);
  /// x.unwrap_error(); // panics with '47'
  /// ```
  ///
  /// ```
  /// # use outcome::*;
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
      Success(value) => panic!(
        "Called `Outcome::unwrap_error()` on a `Success` value: {:?}",
        value
      ),
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

/* stdlib + core integration */
impl<S, M: Into<F>, F> From<Outcome<S, M, F>> for Result<S, F> {
  fn from(outcome: Outcome<S, M, F>) -> Self {
    match outcome {
      Success(value) => Ok(value),
      Mistake(value) => Err(value.into()),
      Failure(value) => Err(value),
    }
  }
}
