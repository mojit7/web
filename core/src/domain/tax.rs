//! Tax Domain Type
//!
//! This module implements tax calculations with formal proofs.
//! Key invariants:
//! 1. Tax is calculated as: base * rate / 100
//! 2. Tax is non-negative when base is non-negative
//! 3. Tax with 0% rate is 0

use crate::domain::money::Money;
use crate::domain::rate::Rate;

/// Compute tax from base amount and rate
/// Formula: tax = base * rate / 100
/// Note: We use integer division which truncates, but this is acceptable for cents
pub fn compute_tax(base: &Money, rate: &Rate) -> Money {
    let tax_cents = base.cents * rate.to_int() / 100;
    Money::new(tax_cents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Test tax_non_negative
    // If base is non-negative, then tax is non-negative
    proptest! {
        #[test]
        fn tax_non_negative(base_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "base >= 0"),
                           rate_value in 0..=100u8) {
            let base = Money::new(base_cents);
            let rate = Rate::new(rate_value);
            let tax = compute_tax(&base, &rate);
            assert!(tax.cents >= 0);
        }
    }

    // Test tax_zero_rate
    // Tax with 0% rate is 0
    proptest! {
        #[test]
        fn tax_zero_rate(base_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "base >= 0")) {
            let base = Money::new(base_cents);
            let rate = Rate::zero();
            let tax = compute_tax(&base, &rate);
            assert_eq!(tax.cents, 0);
        }
    }

    // Test tax_max_rate
    // Tax with 100% rate equals base
    proptest! {
        #[test]
        fn tax_max_rate(base_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "base >= 0")) {
            let base = Money::new(base_cents);
            let rate = Rate::max();
            let tax = compute_tax(&base, &rate);
            // For integer division: base * 100 / 100 = base
            // This holds for all integers
            assert_eq!(tax.cents, base_cents);
        }
    }

    // Test tax_bounded
    // Tax is bounded by base (when rate <= 100)
    proptest! {
        #[test]
        fn tax_bounded(base_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "base >= 0"),
                       rate_value in 0..=100u8) {
            let base = Money::new(base_cents);
            let rate = Rate::new(rate_value);
            let tax = compute_tax(&base, &rate);
            assert!(tax.cents <= base_cents);
        }
    }

    // Test tax calculation formula
    #[test]
    fn tax_calculation_formula() {
        let base = Money::new(1000); // $10.00
        let rate = Rate::new(9);   // 9%
        let tax = compute_tax(&base, &rate);
        // 1000 * 9 / 100 = 90 cents = $0.90
        assert_eq!(tax.cents, 90);
    }

    // Test tax with various rates
    #[test]
    fn tax_various_rates() {
        let base = Money::new(1000); // $10.00
        
        // 0% tax
        let tax0 = compute_tax(&base, &Rate::new(0));
        assert_eq!(tax0.cents, 0);
        
        // 10% tax
        let tax10 = compute_tax(&base, &Rate::new(10));
        assert_eq!(tax10.cents, 100);
        
        // 50% tax
        let tax50 = compute_tax(&base, &Rate::new(50));
        assert_eq!(tax50.cents, 500);
        
        // 100% tax
        let tax100 = compute_tax(&base, &Rate::new(100));
        assert_eq!(tax100.cents, 1000);
    }

    // Test tax with large amounts
    #[test]
    fn tax_large_amounts() {
        let base = Money::new(1_000_000); // $10,000.00
        let rate = Rate::new(7);   // 7%
        let tax = compute_tax(&base, &rate);
        // 1000000 * 7 / 100 = 70000 cents = $700.00
        assert_eq!(tax.cents, 70_000);
    }
}
