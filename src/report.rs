extern crate std;

use crate::Aberration;
use eyre::Report;

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
