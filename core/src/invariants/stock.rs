//! Stock Invariants
//!
//! This module contains runtime checks for stock invariants.

use crate::domain::stock::Stock;
use crate::invariants::InvariantError;

/// Check that stock quantity is non-negative
pub fn check_non_negative(stock: &Stock) -> Result<(), InvariantError> {
    // Since qty is u64, this is always true, but we check for completeness
    if stock.qty < 0 {
        Err(InvariantError::StockViolation(format!(
            "Stock quantity cannot be negative: {}",
            stock.qty
        )))
    } else {
        Ok(())
    }
}

/// Check that decrementing stock is safe
pub fn check_decrement_safe(stock: &Stock, n: u64) -> Result<(), InvariantError> {
    if n > stock.qty {
        Err(InvariantError::StockViolation(format!(
            "Cannot decrement stock by {} when quantity is {}",
            n, stock.qty
        )))
    } else {
        Ok(())
    }
}

/// Check that stock has enough quantity
pub fn check_has_enough(stock: &Stock, n: u64) -> Result<(), InvariantError> {
    if !stock.has_enough(n) {
        Err(InvariantError::StockViolation(format!(
            "Stock does not have enough quantity: requested {}, available {}",
            n, stock.qty
        )))
    } else {
        Ok(())
    }
}

/// Check that stock decrement preserves non-negativity
pub fn check_decrement_preserves_non_negative(stock: &Stock, n: u64) -> Result<(), InvariantError> {
    let result = stock.decrement(n);
    if result.qty < 0 {
        Err(InvariantError::StockViolation(
            "Stock decrement resulted in negative quantity".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check that stock increment preserves non-negativity
pub fn check_increment_preserves_non_negative(stock: &Stock, n: u64) -> Result<(), InvariantError> {
    let result = stock.increment(n);
    if result.qty < 0 {
        Err(InvariantError::StockViolation(
            "Stock increment resulted in negative quantity".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::stock::Stock;
    use crate::domain::product::ProductId;

    fn test_stock() -> Stock {
        Stock::new(ProductId::new(1), 100)
    }

    #[test]
    fn test_non_negative() {
        let stock = test_stock();
        assert!(check_non_negative(&stock).is_ok());
    }

    #[test]
    fn test_decrement_safe_valid() {
        let stock = test_stock();
        assert!(check_decrement_safe(&stock, 50).is_ok());
    }

    #[test]
    fn test_decrement_safe_invalid() {
        let stock = test_stock();
        assert!(check_decrement_safe(&stock, 150).is_err());
    }

    #[test]
    fn test_has_enough_valid() {
        let stock = test_stock();
        assert!(check_has_enough(&stock, 50).is_ok());
    }

    #[test]
    fn test_has_enough_invalid() {
        let stock = test_stock();
        assert!(check_has_enough(&stock, 150).is_err());
    }

    #[test]
    fn test_decrement_preserves_non_negative() {
        let stock = test_stock();
        assert!(check_decrement_preserves_non_negative(&stock, 50).is_ok());
    }

    #[test]
    fn test_increment_preserves_non_negative() {
        let stock = test_stock();
        assert!(check_increment_preserves_non_negative(&stock, 50).is_ok());
    }
}
