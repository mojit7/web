//! Tax Invariants
//!
//! This module contains runtime checks for tax invariants.

use crate::domain::money::Money;
use crate::domain::rate::Rate;
use crate::domain::tax::compute_tax;
use crate::invariants::InvariantError;

/// Check that tax is non-negative when base is non-negative
pub fn check_non_negative(base: &Money, rate: &Rate) -> Result<(), InvariantError> {
    let tax = compute_tax(base, rate);
    if tax.cents < 0 {
        Err(InvariantError::TaxViolation(format!(
            "Tax cannot be negative: base={}, rate={}, tax={}",
            base.cents, rate.value(), tax.cents
        )))
    } else {
        Ok(())
    }
}

/// Check that tax with 0% rate is 0
pub fn check_zero_rate(base: &Money) -> Result<(), InvariantError> {
    let rate = Rate::zero();
    let tax = compute_tax(base, &rate);
    if tax.cents != 0 {
        Err(InvariantError::TaxViolation(format!(
            "Tax with 0% rate should be 0, got: {}",
            tax.cents
        )))
    } else {
        Ok(())
    }
}

/// Check that tax is bounded by base (when rate ≤ 100)
pub fn check_bounded(base: &Money, rate: &Rate) -> Result<(), InvariantError> {
    let tax = compute_tax(base, rate);
    if tax.cents > base.cents {
        Err(InvariantError::TaxViolation(format!(
            "Tax cannot exceed base: base={}, rate={}, tax={}",
            base.cents, rate.value(), tax.cents
        )))
    } else {
        Ok(())
    }
}

/// Check that tax calculation is correct
pub fn check_calculation(base: &Money, rate: &Rate, expected: i64) -> Result<(), InvariantError> {
    let tax = compute_tax(base, rate);
    if tax.cents != expected {
        Err(InvariantError::TaxViolation(format!(
            "Tax calculation incorrect: expected {}, got {}",
            expected, tax.cents
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::money::Money;
    use crate::domain::rate::Rate;

    #[test]
    fn test_non_negative() {
        let base = Money::new(1000);
        let rate = Rate::new(10);
        assert!(check_non_negative(&base, &rate).is_ok());
    }

    #[test]
    fn test_zero_rate() {
        let base = Money::new(1000);
        assert!(check_zero_rate(&base).is_ok());
    }

    #[test]
    fn test_bounded() {
        let base = Money::new(1000);
        let rate = Rate::new(10);
        assert!(check_bounded(&base, &rate).is_ok());
    }

    #[test]
    fn test_calculation() {
        let base = Money::new(1000);
        let rate = Rate::new(10);
        // 1000 * 10 / 100 = 100
        assert!(check_calculation(&base, &rate, 100).is_ok());
    }

    #[test]
    fn test_zero_rate_calculation() {
        let base = Money::new(1000);
        let rate = Rate::zero();
        assert!(check_calculation(&base, &rate, 0).is_ok());
    }

    #[test]
    fn test_max_rate_calculation() {
        let base = Money::new(1000);
        let rate = Rate::max();
        assert!(check_calculation(&base, &rate, 1000).is_ok());
    }
}
