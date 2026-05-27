# ab-testing-rs

Rust port of [ab-testing](https://github.com/SuperInstance/ab-testing) — A/B testing with chi-squared and Welch's t-test.

## Features

- **Chi-squared test** for comparing two proportions
- **Welch's t-test** for comparing two continuous samples
- **Confidence intervals** for sample means
- **Experiment** builder with variant management and reporting

## Usage

```rust
use ab_testing::{Experiment, Variant};

let mut exp = Experiment::new("landing_page");
exp.add_variant(Variant::new("control").with_conversion(100, 200));
exp.add_variant(Variant::new("treatment").with_conversion(150, 200));

let report = exp.report();
println!("Winner: {:?}", report.winner);

// Continuous metrics
let mut exp2 = Experiment::new("revenue");
exp2.add_variant(Variant::new("A").with_values(vec![10.0, 12.0, 15.0, 11.0]));
exp2.add_variant(Variant::new("B").with_values(vec![14.0, 16.0, 18.0, 15.0]));
let welch = exp2.run_welch_t().unwrap();
println!("t={:.3}, p={:.3}", welch.t_statistic, welch.p_value);
```

## License

MIT
