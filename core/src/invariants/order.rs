//! Order Invariants
//!
//! This module contains runtime checks for order invariants.

use crate::domain::order::Order;
use crate::domain::money::Money;
use crate::invariants::InvariantError;

/// Check that order total formula holds: total = subtotal - discount + tax
pub fn check_total_formula(order: &Order) -> Result<(), InvariantError> {
    let expected_total = order.subtotal().checked_sub(&order.discount.amount);
    match expected_total {
        Some(subtotal_after_discount) => {
            let tax = order.tax();
            let expected = subtotal_after_discount + tax;
            
            if order.total().cents != expected.cents {
                Err(InvariantError::OrderViolation(format!(
                    "Order total formula violated: expected {}, got {}",
                    expected.cents,
                    order.total().cents
                )))
            } else {
                Ok(())
            }
        }
        None => {
            // If subtotal < discount, total should be 0
            if order.total().cents != 0 {
                Err(InvariantError::OrderViolation(
                    "Order total should be 0 when discount exceeds subtotal".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

/// Check that subtotal equals sum of line subtotals
pub fn check_subtotal_sum(order: &Order) -> Result<(), InvariantError> {
    let calculated_subtotal = order.subtotal();
    let sum_of_lines: Money = order.lines
        .iter()
        .map(|line| line.subtotal())
        .fold(Money::zero(), |acc, m| acc + m);
    
    if calculated_subtotal.cents != sum_of_lines.cents {
        Err(InvariantError::OrderViolation(format!(
            "Order subtotal sum violated: expected {}, got {}",
            sum_of_lines.cents,
            calculated_subtotal.cents
        )))
    } else {
        Ok(())
    }
}

/// Check that tax is calculated on discounted amount
pub fn check_tax_on_discounted(order: &Order) -> Result<(), InvariantError> {
    let subtotal_after_discount = order.subtotal().checked_sub(&order.discount.amount);
    match subtotal_after_discount {
        Some(sub) => {
            let expected_tax = crate::domain::tax::compute_tax(&sub, &order.tax_rate);
            let actual_tax = order.tax();
            
            if actual_tax.cents != expected_tax.cents {
                Err(InvariantError::OrderViolation(format!(
                    "Order tax calculation violated: expected {}, got {}",
                    expected_tax.cents,
                    actual_tax.cents
                )))
            } else {
                Ok(())
            }
        }
        None => {
            // If subtotal < discount, tax should be 0
            if order.tax().cents != 0 {
                Err(InvariantError::OrderViolation(
                    "Order tax should be 0 when discount exceeds subtotal".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

/// Check that discount is non-negative
pub fn check_discount_non_negative(order: &Order) -> Result<(), InvariantError> {
    if order.discount.amount.cents < 0 {
        Err(InvariantError::OrderViolation(
            "Order discount cannot be negative".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check that discount is bounded by subtotal
pub fn check_discount_bounded(order: &Order) -> Result<(), InvariantError> {
    if order.discount.amount.cents > order.subtotal().cents {
        Err(InvariantError::OrderViolation(
            "Order discount cannot exceed subtotal".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check that all order invariants hold
pub fn check_all_order_invariants(order: &Order) -> Result<(), InvariantError> {
    check_subtotal_sum(order)?;
    check_total_formula(order)?;
    check_tax_on_discounted(order)?;
    check_discount_non_negative(order)?;
    check_discount_bounded(order)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::order::Order;
    use crate::domain::product::Product;
    use crate::domain::product::ProductId;
    use crate::domain::discount::Discount;
    use crate::domain::rate::Rate;

    fn test_order() -> Order {
        let product = Product::new(
            ProductId::new(1),
            "Test Product".to_string(),
            Money::new(1000),
        );
        let line = crate::domain::order::Line::new(product, 2);
        Order::new(vec![line], Discount::zero(), Rate::zero())
    }

    #[test]
    fn test_total_formula() {
        let order = test_order();
        assert!(check_total_formula(&order).is_ok());
    }

    #[test]
    fn test_subtotal_sum() {
        let order = test_order();
        assert!(check_subtotal_sum(&order).is_ok());
    }

    #[test]
    fn test_tax_on_discounted() {
        let order = test_order();
        assert!(check_tax_on_discounted(&order).is_ok());
    }

    #[test]
    fn test_discount_non_negative() {
        let order = test_order();
        assert!(check_discount_non_negative(&order).is_ok());
    }

    #[test]
    fn test_discount_bounded() {
        let order = test_order();
        assert!(check_discount_bounded(&order).is_ok());
    }

    #[test]
    fn test_all_invariants() {
        let order = test_order();
        assert!(check_all_order_invariants(&order).is_ok());
    }
}
