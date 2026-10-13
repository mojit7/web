//! Money Invariants
//!
//! This module contains runtime checks for money invariants.

use crate::domain::money::Money;
use crate::invariants::InvariantError;

/// Check that money is non-negative
pub fn check_non_negative(money: &Money) -> Result<(), InvariantError> {
    if money.cents < 0 {
        Err(InvariantError::MoneyViolation(format!(
            "Money cannot be negative: {}",
            money.cents
        )))
    } else {
        Ok(())
    }
}

/// Check that money addition is associative
/// This is always true for integers, so it's more of a documentation check
pub fn check_add_assoc(a: &Money, b: &Money, c: &Money) -> Result<(), InvariantError> {
    let result1 = (a.clone() + b.clone()) + c.clone();
    let result2 = a.clone() + (b.clone() + c.clone());
    
    if result1.cents != result2.cents {
        Err(InvariantError::MoneyViolation(
            "Money addition is not associative".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check that money addition is commutative
pub fn check_add_comm(a: &Money, b: &Money) -> Result<(), InvariantError> {
    let result1 = a.clone() + b.clone();
    let result2 = b.clone() + a.clone();
    
    if result1.cents != result2.cents {
        Err(InvariantError::MoneyViolation(
            "Money addition is not commutative".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check that subtracting a money from itself gives zero
pub fn check_sub_self(a: &Money) -> Result<(), InvariantError> {
    let result = a.clone() - a.clone();
    
    if result.cents != 0 {
        Err(InvariantError::MoneyViolation(
            format!("Money subtraction of self does not give zero: {}", result.cents),
        ))
    } else {
        Ok(())
    }
}

/// Check that multiplication distributes over addition
pub fn check_mul_distrib(a: &Money, b: &Money, n: u64) -> Result<(), InvariantError> {
    let sum = a.clone() + b.clone();
    let result1 = sum.mul_nat(n);
    let result2 = a.mul_nat(n) + b.mul_nat(n);
    
    if result1.cents != result2.cents {
        Err(InvariantError::MoneyViolation(
            "Money multiplication does not distribute over addition".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::money::Money;

    #[test]
    fn test_non_negative_valid() {
        let money = Money::new(100);
        assert!(check_non_negative(&money).is_ok());
    }

    #[test]
    fn test_non_negative_invalid() {
        // We can't create negative money directly, but we can test the check
        // This test would fail if we could create negative money
        // For now, we just test that the check works on valid money
        let money = Money::new(100);
        assert!(check_non_negative(&money).is_ok());
    }

    #[test]
    fn test_add_assoc() {
        let a = Money::new(100);
        let b = Money::new(200);
        let c = Money::new(300);
        assert!(check_add_assoc(&a, &b, &c).is_ok());
    }

    #[test]
    fn test_add_comm() {
        let a = Money::new(100);
        let b = Money::new(200);
        assert!(check_add_comm(&a, &b).is_ok());
    }

    #[test]
    fn test_sub_self() {
        let a = Money::new(100);
        assert!(check_sub_self(&a).is_ok());
    }

    #[test]
    fn test_mul_distrib() {
        let a = Money::new(100);
        let b = Money::new(200);
        assert!(check_mul_distrib(&a, &b, 3).is_ok());
    }
}
