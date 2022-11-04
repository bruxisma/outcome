use core::iter::FusedIterator;

use crate::prelude::*;

/// An iterator over the value in a [`Success`] variant of an [`Outcome`].
///
/// The iterator yields one value if the result is [`Success`], otherwise none.
///
/// This struct is created by the [`into_iter`] method on [`Outcome`] (via the
/// [`IntoIterator`] trait).
///
/// [`Success`]: crate::prelude::Success
/// [`Outcome`]: crate::prelude::Outcome
/// [`into_iter`]: crate::prelude::Outcome::into_iter
#[derive(Clone, Debug)]
pub struct IntoIter<T> {
  pub(crate) inner: Option<T>,
}

/// An iterator over a mutable reference to the [`Success`] variant of an
/// [`Outcome`].
///
/// Created by [`Outcome::iter_mut`]
///
/// [`Success`]: crate::prelude::Success
/// [`Outcome`]: crate::prelude::Outcome
/// [`Outcome::iter_mut`]: crate::prelude::Outcome::iter_mut
#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
  pub(crate) inner: Option<&'a mut T>,
}

/// An iterator over a reference to the [`Success`] variant of an [`Outcome`].
///
/// The iterator yields one value if the result is [`Success`], otherwise none.
///
/// Created by [`Outcome::iter`].
///
/// [`Success`]: crate::prelude::Success
/// [`Outcome`]: crate::prelude::Outcome
/// [`Outcome::iter`]: crate::prelude::Outcome::iter
#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
  pub(crate) inner: Option<&'a T>,
}

/// An iterator adapter that produces output as long as the underlying iterator
/// produces [`Outcome::Success`] values.
///
/// If an error is encountered, the iterator stops and the error is stored.
struct OutcomeShunt<'a, I, M, F> {
  error: &'a mut Outcome<(), M, F>,
  iter: I,
}

impl<'a, S, M, F> IntoIterator for &'a mut Outcome<S, M, F> {
  type IntoIter = IterMut<'a, S>;
  type Item = &'a mut S;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<'a, S, M, F> IntoIterator for &'a Outcome<S, M, F> {
  type IntoIter = Iter<'a, S>;
  type Item = &'a S;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<S, M, F> IntoIterator for Outcome<S, M, F> {
  type IntoIter = IntoIter<S>;
  type Item = S;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      inner: self.success(),
    }
  }
}

/* Iterator Trait Implementations */
//impl<S, M, F, T: FromIterator<S>> FromIterator<Outcome<S, M, F>>
//  for Outcome<T, M, F>
//{
//  #[inline]
//  fn from_iter<I>(iter: I) -> Outcome<T, M, F>
//  where
//    I: IntoIterator<Item = Outcome<S, M, F>>,
//  {
//    process_outcomes(iter.into_iter(), Iterator::collect)
//  }
//}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  #[inline]
  fn next(&mut self) -> Option<T> {
    self.inner.take()
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let n = usize::from(self.inner.is_some());
    (n, Some(n))
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  #[inline]
  fn next(&mut self) -> Option<&'a mut T> {
    self.inner.take()
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let n = usize::from(self.inner.is_some());
    (n, Some(n))
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  #[inline]
  fn next(&mut self) -> Option<&'a T> {
    self.inner.take()
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let n = usize::from(self.inner.is_some());
    (n, Some(n))
  }
}

impl<I, S, M, F> Iterator for OutcomeShunt<'_, I, M, F>
where
  I: Iterator<Item = Outcome<S, M, F>>,
{
  type Item = S;

  fn next(&mut self) -> Option<Self::Item> {
    self.find(|_| true)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    if self.error.is_error() {
      (0, Some(0))
    } else {
      let (_, upper) = self.iter.size_hint();
      (0, upper)
    }
  }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
  #[inline]
  fn next_back(&mut self) -> Option<T> {
    self.inner.take()
  }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
  #[inline]
  fn next_back(&mut self) -> Option<&'a mut T> {
    self.inner.take()
  }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
  #[inline]
  fn next_back(&mut self) -> Option<&'a T> {
    self.inner.take()
  }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> ExactSizeIterator for IterMut<'_, T> {}
impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for IntoIter<T> {}
impl<T> FusedIterator for IterMut<'_, T> {}
impl<T> FusedIterator for Iter<'_, T> {}
