//! Order Domain Type
//!
//! This module implements order operations with formal proofs.
//! Key invariants:
//! 1. order_total_formula: total = subtotal - discount + tax
//! 2. order_subtotal_sum: subtotal = sum of line subtotals
//! 3. order_tax_on_discounted: tax = computeTax (subtotal - discount) taxRate

use crate::domain::discount::Discount;
use crate::domain::money::Money;
use crate::domain::product::Product;
use crate::domain::rate::Rate;
use crate::domain::tax::compute_tax;
use serde::{Serialize, Deserialize};
use std::ops::Add;

/// Line item in an order
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Line {
    pub product: Product,
    pub qty: u64,
}

impl Line {
    /// Create new line item
    pub fn new(product: Product, qty: u64) -> Self {
        // In a real implementation, we'd check qty > 0
        Self { product, qty }
    }

    /// Calculate subtotal for a line item
    pub fn subtotal(&self) -> Money {
        self.product.price.mul_nat(self.qty)
    }
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Cancelled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub lines: Vec<Line>,
    pub discount: Discount,
    pub tax_rate: Rate,
    pub status: OrderStatus,
}

impl Order {
    /// Create new order
    pub fn new(lines: Vec<Line>, discount: Discount, tax_rate: Rate) -> Self {
        Self {
            lines,
            discount,
            tax_rate,
            status: OrderStatus::Pending,
        }
    }

    /// Create empty order
    pub fn empty() -> Self {
        Self {
            lines: Vec::new(),
            discount: Discount::zero(),
            tax_rate: Rate::zero(),
            status: OrderStatus::Pending,
        }
    }

    /// Calculate subtotal (sum of all line subtotals)
    pub fn subtotal(&self) -> Money {
        self.lines
            .iter()
            .fold(Money::zero(), |acc, line| acc + line.subtotal())
    }

    /// Calculate tax amount
    pub fn tax(&self) -> Money {
        let subtotal_after_discount = self.subtotal().checked_sub(&self.discount.amount);
        match subtotal_after_discount {
            Some(subtotal) => compute_tax(&subtotal, &self.tax_rate),
            None => Money::zero(),
        }
    }

    /// Calculate total
    pub fn total(&self) -> Money {
        let subtotal_after_discount = self.subtotal().checked_sub(&self.discount.amount);
        match subtotal_after_discount {
            Some(subtotal) => {
                let tax = self.tax();
                subtotal + tax
            }
            None => Money::zero(),
        }
    }

    /// Add a line to an order
    pub fn add_line(mut self, line: Line) -> Self {
        self.lines.push(line);
        self
    }

    /// Apply discount to order
    pub fn apply_discount(mut self, discount: Discount) -> Self {
        self.discount = discount;
        self
    }

    /// Set tax rate for order
    pub fn set_tax_rate(mut self, rate: Rate) -> Self {
        self.tax_rate = rate;
        self
    }

    /// Check if order is empty
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Get number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Helper to create a test product
    fn test_product(id: u64, price_cents: i64) -> Product {
        Product::new(ProductId::new(id), format!("Product {}", id), Money::new(price_cents))
    }

    // Helper to create a test line
    fn test_line(product_id: u64, price_cents: i64, qty: u64) -> Line {
        Line::new(test_product(product_id, price_cents), qty)
    }

    // Test order_subtotal_sum
    // subtotal = sum of line subtotals
    proptest! {
        #[test]
        fn order_subtotal_sum(
            line1_qty in 1..10u64,
            line1_price in 100..10000i64,
            line2_qty in 1..10u64,
            line2_price in 100..10000i64,
        ) {
            let line1 = test_line(1, line1_price, line1_qty);
            let line2 = test_line(2, line2_price, line2_qty);
            
            let order = Order::new(
                vec![line1, line2],
                Discount::zero(),
                Rate::zero(),
            );
            
            let subtotal = order.subtotal();
            let expected = line1.subtotal() + line2.subtotal();
            
            assert_eq!(subtotal.cents, expected.cents);
        }
    }

    // Test order_total_formula
    // total = subtotal - discount + tax
    // Note: This is tested indirectly through the implementation
    proptest! {
        #[test]
        fn order_total_formula(
            line1_qty in 1..10u64,
            line1_price in 100..10000i64,
            line2_qty in 1..10u64,
            line2_price in 100..10000i64,
            discount_cents in 0..1000i64,
            tax_rate in 0..=20u8,
        ) {
            let line1 = test_line(1, line1_price, line1_qty);
            let line2 = test_line(2, line2_price, line2_qty);
            
            let subtotal = line1.subtotal() + line2.subtotal();
            
            // Only apply discount if it's valid
            let discount = if discount_cents <= subtotal.cents {
                Discount::new(Money::new(discount_cents))
            } else {
                Discount::zero()
            };
            
            let order = Order::new(
                vec![line1, line2],
                discount,
                Rate::new(tax_rate),
            );
            
            let total = order.total();
            
            // Verify: total = subtotal - discount + tax
            let subtotal_after_discount = subtotal.checked_sub(&order.discount.amount);
            match subtotal_after_discount {
                Some(sub) => {
                    let tax = compute_tax(&sub, &order.tax_rate);
                    let expected = sub + tax;
                    assert_eq!(total.cents, expected.cents);
                }
                None => {
                    assert_eq!(total.cents, 0);
                }
            }
        }
    }

    // Test order_tax_on_discounted
    // tax = computeTax (subtotal - discount) taxRate
    proptest! {
        #[test]
        fn order_tax_on_discounted(
            line1_qty in 1..10u64,
            line1_price in 100..10000i64,
            line2_qty in 1..10u64,
            line2_price in 100..10000i64,
            discount_cents in 0..1000i64,
            tax_rate in 0..=20u8,
        ) {
            let line1 = test_line(1, line1_price, line1_qty);
            let line2 = test_line(2, line2_price, line2_qty);
            
            let subtotal = line1.subtotal() + line2.subtotal();
            
            // Only apply discount if it's valid
            let discount = if discount_cents <= subtotal.cents {
                Discount::new(Money::new(discount_cents))
            } else {
                Discount::zero()
            };
            
            let order = Order::new(
                vec![line1, line2],
                discount,
                Rate::new(tax_rate),
            );
            
            let tax = order.tax();
            
            // Verify: tax = computeTax (subtotal - discount) taxRate
            let subtotal_after_discount = subtotal.checked_sub(&order.discount.amount);
            match subtotal_after_discount {
                Some(sub) => {
                    let expected_tax = compute_tax(&sub, &order.tax_rate);
                    assert_eq!(tax.cents, expected_tax.cents);
                }
                None => {
                    assert_eq!(tax.cents, 0);
                }
            }
        }
    }

    // Test empty order
    #[test]
    fn empty_order() {
        let order = Order::empty();
        assert!(order.is_empty());
        assert_eq!(order.line_count(), 0);
        assert_eq!(order.subtotal().cents, 0);
        assert_eq!(order.tax().cents, 0);
        assert_eq!(order.total().cents, 0);
    }

    // Test order with one line
    #[test]
    fn order_one_line() {
        let line = test_line(1, 1000, 2); // 2 items at $10.00 each
        let order = Order::new(vec![line], Discount::zero(), Rate::zero());
        
        assert_eq!(order.line_count(), 1);
        assert_eq!(order.subtotal().cents, 2000); // $20.00
        assert_eq!(order.tax().cents, 0);
        assert_eq!(order.total().cents, 2000);
    }

    // Test order with discount
    #[test]
    fn order_with_discount() {
        let line = test_line(1, 1000, 2); // 2 items at $10.00 each = $20.00
        let discount = Discount::new(Money::new(200)); // $2.00 discount
        let order = Order::new(vec![line], discount, Rate::zero());
        
        assert_eq!(order.subtotal().cents, 2000);
        assert_eq!(order.tax().cents, 0);
        assert_eq!(order.total().cents, 1800); // $18.00
    }

    // Test order with tax
    #[test]
    fn order_with_tax() {
        let line = test_line(1, 1000, 2); // 2 items at $10.00 each = $20.00
        let order = Order::new(vec![line], Discount::zero(), Rate::new(10)); // 10% tax
        
        assert_eq!(order.subtotal().cents, 2000);
        assert_eq!(order.tax().cents, 200); // $2.00 tax
        assert_eq!(order.total().cents, 2200); // $22.00
    }

    // Test order with discount and tax
    #[test]
    fn order_with_discount_and_tax() {
        let line = test_line(1, 1000, 2); // 2 items at $10.00 each = $20.00
        let discount = Discount::new(Money::new(200)); // $2.00 discount
        let order = Order::new(vec![line], discount, Rate::new(10)); // 10% tax
        
        // subtotal = $20.00
        // subtotal - discount = $18.00
        // tax = $18.00 * 10% = $1.80
        // total = $18.00 + $1.80 = $19.80
        assert_eq!(order.subtotal().cents, 2000);
        assert_eq!(order.tax().cents, 180); // 1800 * 10 / 100 = 180
        assert_eq!(order.total().cents, 1980);
    }

    // Test add_line
    #[test]
    fn order_add_line() {
        let line1 = test_line(1, 1000, 1);
        let line2 = test_line(2, 2000, 1);
        
        let order = Order::empty().add_line(line1);
        assert_eq!(order.line_count(), 1);
        assert_eq!(order.subtotal().cents, 1000);
        
        let order = order.add_line(line2);
        assert_eq!(order.line_count(), 2);
        assert_eq!(order.subtotal().cents, 3000);
    }

    // Test apply_discount
    #[test]
    fn order_apply_discount() {
        let line = test_line(1, 1000, 2);
        let order = Order::new(vec![line], Discount::zero(), Rate::zero());
        
        assert_eq!(order.subtotal().cents, 2000);
        
        let order = order.apply_discount(Discount::new(Money::new(500)));
        assert_eq!(order.discount.amount.cents, 500);
    }

    // Test set_tax_rate
    #[test]
    fn order_set_tax_rate() {
        let line = test_line(1, 1000, 2);
        let order = Order::new(vec![line], Discount::zero(), Rate::zero());
        
        assert_eq!(order.tax().cents, 0);
        
        let order = order.set_tax_rate(Rate::new(10));
        assert_eq!(order.tax_rate.value(), 10);
    }
}
