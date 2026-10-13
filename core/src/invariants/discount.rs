//! Discount Invariants
//!
//! This module contains runtime checks for discount invariants.

use crate::domain::discount::Discount;
use crate::domain::money::Money;
use crate::invariants::InvariantError;

/// Check that discount is non-negative
pub fn check_non_negative(discount: &Discount) -> Result<(), InvariantError> {
    if discount.amount.cents < 0 {
        Err(InvariantError::DiscountViolation(format!(
            "Discount cannot be negative: {}",
            discount.amount.cents
        )))
    } else {
        Ok(())
    }
}

/// Check that discount is bounded by subtotal
pub fn check_bounded(discount: &Discount, subtotal: &Money) -> Result<(), InvariantError> {
    if discount.amount.cents > subtotal.cents {
        Err(InvariantError::DiscountViolation(
            "Discount cannot exceed subtotal".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check that applying zero discount returns original amount
pub fn check_zero_discount(sub: &Money, discount: &Discount) -> Result<(), InvariantError> {
    use crate::domain::discount::apply_discount;
    
    if discount.amount.cents == 0 {
        let result = apply_discount(sub, discount);
        match result {
            Some(result) => {
                if result.cents != sub.cents {
                    Err(InvariantError::DiscountViolation(
                        "Zero discount does not return original amount".to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
            None => {
                Err(InvariantError::DiscountViolation(
                    "Zero discount should always succeed".to_string(),
                ))
            }
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::discount::Discount;
    use crate::domain::money::Money;

    #[test]
    fn test_non_negative() {
        let discount = Discount::new(Money::new(100));
        assert!(check_non_negative(&discount).is_ok());
    }

    #[test]
    fn test_bounded_valid() {
        let discount = Discount::new(Money::new(100));
        let subtotal = Money::new(1000);
        assert!(check_bounded(&discount, &subtotal).is_ok());
    }

    #[test]
    fn test_bounded_invalid() {
        let discount = Discount::new(Money::new(1000));
        let subtotal = Money::new(100);
        assert!(check_bounded(&discount, &subtotal).is_err());
    }

    #[test]
    fn test_zero_discount() {
        let discount = Discount::zero();
        let subtotal = Money::new(1000);
        assert!(check_zero_discount(&subtotal, &discount).is_ok());
    }

    #[test]
    fn test_zero_discount_non_zero_amount() {
        let discount = Discount::new(Money::new(100));
        let subtotal = Money::new(1000);
        // This should pass because discount is not zero
        assert!(check_zero_discount(&subtotal, &discount).is_ok());
    }
}
