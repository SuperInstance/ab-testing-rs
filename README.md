# ab-testing-rs

Statistical A/B testing in Rust — chi-squared tests for conversion rates, Welch's t-test for continuous metrics, and confidence intervals. All math implemented from scratch.

## What This Gives You

- Compare two conversion rates with a **chi-squared test** and get p-values, significance, and critical values
- Compare two continuous samples (revenue, latency, etc.) with **Welch's t-test** including confidence intervals on the difference
- Compute **confidence intervals** for proportions and means at any confidence level
- Run structured **experiments** with named variants, automatic winner detection, and full reports
- Zero external math dependencies — Lanczos gamma, A&S erf, normal/t CDF approximations all built in

## Quick Start

```rust
use ab_testing::{Experiment, Variant};

// Conversion rate test: which landing page wins?
let mut exp = Experiment::new("landing_page");
exp.add_variant(Variant::new("control").with_conversion(100, 200));
exp.add_variant(Variant::new("treatment").with_conversion(150, 200));

let report = exp.report();
println!("Winner: {:?}", report.winner);
// Each variant gets conversion rate, z-score against the others

// Continuous metric test: does treatment increase revenue?
let mut exp2 = Experiment::new("revenue");
exp2.add_variant(Variant::new("A").with_values(vec![10.0, 12.0, 15.0, 11.0]));
exp2.add_variant(Variant::new("B").with_values(vec![14.0, 16.0, 18.0, 15.0]));

let welch = exp2.run_welch_t().unwrap();
println!("t = {:.3}, p = {:.3}, diff = {:.3} [{:.3}, {:.3}]",
    welch.t_statistic, welch.p_value,
    welch.difference, welch.ci_lower, welch.ci_upper);
```

## API Reference

### Statistical Tests

| Function | Signature | Returns |
|----------|-----------|---------|
| `chi_squared_test` | `(successes_a, trials_a, successes_b, trials_b) → ChiSquaredResult` | chi2 statistic, p-value, significance, critical value |
| `welch_t_test` | `(&[f64], &[f64]) → WelchTResult` | t-statistic, p-value, CI on difference, means |
| `confidence_interval` | `(mean, std_dev, n, confidence) → (f64, f64)` | Lower and upper bounds |

### Experiment Builder

| Type | Purpose |
|------|---------|
| `Experiment` | Named experiment with multiple variants, produces reports |
| `Variant` | Named group with conversion counts or continuous values |
| `ExperimentReport` | Winner, rankings, chi-squared and t-test results |
| `VariantSummary` | Per-variant stats: rate, mean, std dev, sample size |

## How It Fits

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem. Used by:

- **bid-engine-rs** — evaluating auction outcomes statistically
- Any agent that needs to make data-driven decisions between alternatives

The C port lives at [ab-testing-c](https://github.com/SuperInstance/ab-testing-c).

## Testing

**11 tests** covering chi-squared calculation, Welch's t-test, confidence intervals, experiment reports, and edge cases (empty samples, equal groups).

## Installation

```toml
# Cargo.toml
[dependencies]
ab-testing = { git = "https://github.com/SuperInstance/ab-testing-rs" }
```

Requires Rust 2021 edition. Depends on `serde` and `serde_json` for serializable results.
