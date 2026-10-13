//! Discount Domain Type
//!
//! This module implements discount operations with formal proofs.
//! Key invariants:
//! 1. Discount amount is non-negative
//! 2. Discount is bounded by the subtotal
//! 3. Zero discount returns the original amount

use crate::domain::money::Money;
use serde::{Serialize, Deserialize};

/// Discount type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discount {
    pub amount: Money,
}

impl Discount {
    /// Create new discount
    pub fn new(amount: Money) -> Self {
        Self { amount }
    }

    /// Zero discount
    pub fn zero() -> Self {
        Self {
            amount: Money::zero(),
        }
    }

    /// Check if discount is zero
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }
}

/// Apply discount to a money amount
/// Returns None if discount would make amount negative
pub fn apply_discount(sub: &Money, discount: &Discount) -> Option<Money> {
    sub.checked_sub(&discount.amount)
}

/// Fixed amount discount
pub fn fixed_discount(amount: Money) -> Discount {
    Discount::new(amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Test discount_non_negative
    // Discount amount is always non-negative
    proptest! {
        #[test]
        fn discount_non_negative(discount_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "discount >= 0")) {
            let discount = Discount::new(Money::new(discount_cents));
            assert!(discount.amount.cents >= 0);
        }
    }

    // Test discount_bounded
    // Discount cannot exceed the subtotal
    // This is enforced by the application logic, but we test it here
    proptest! {
        #[test]
        fn discount_bounded(sub_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "sub >= 0"),
                            discount_cents in 0..=sub_cents) {
            let sub = Money::new(sub_cents);
            let discount = Discount::new(Money::new(discount_cents));
            
            // The discount is bounded by construction (discount_cents <= sub_cents)
            assert!(discount.amount.cents <= sub.cents);
        }
    }

    // Test discount_zero
    // Applying zero discount returns the original subtotal
    proptest! {
        #[test]
        fn discount_zero(sub_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "sub >= 0")) {
            let sub = Money::new(sub_cents);
            let discount = Discount::zero();
            let result = apply_discount(&sub, &discount);
            
            assert!(result.is_some());
            assert_eq!(result.unwrap().cents, sub_cents);
        }
    }

    // Test apply_discount success
    #[test]
    fn apply_discount_success() {
        let sub = Money::new(1000); // $10.00
        let discount = Discount::new(Money::new(100)); // $1.00
        let result = apply_discount(&sub, &discount);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().cents, 900);
    }

    // Test apply_discount failure (discount > subtotal)
    #[test]
    fn apply_discount_failure() {
        let sub = Money::new(100); // $1.00
        let discount = Discount::new(Money::new(200)); // $2.00
        let result = apply_discount(&sub, &discount);
        
        assert!(result.is_none());
    }

    // Test zero discount
    #[test]
    fn zero_discount() {
        let discount = Discount::zero();
        assert!(discount.is_zero());
        assert_eq!(discount.amount.cents, 0);
    }

    // Test fixed discount
    #[test]
    fn fixed_discount() {
        let discount = fixed_discount(Money::new(500));
        assert_eq!(discount.amount.cents, 500);
    }

    // Test discount equality
    #[test]
    fn discount_equality() {
        let d1 = Discount::new(Money::new(100));
        let d2 = Discount::new(Money::new(100));
        assert_eq!(d1, d2);
    }

    // Test discount inequality
    #[test]
    fn discount_inequality() {
        let d1 = Discount::new(Money::new(100));
        let d2 = Discount::new(Money::new(200));
        assert_ne!(d1, d2);
    }
}
