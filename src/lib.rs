//! ab-testing — A/B testing with chi-squared and Welch's t-test.

mod stats;
mod experiment;

pub use stats::{chi_squared_test, ChiSquaredResult, welch_t_test, WelchTResult, confidence_interval};
pub use experiment::{Experiment, Variant, ExperimentReport, VariantSummary};
