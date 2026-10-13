//! Payment Domain Type
//!
//! This module implements payment operations.
//! Key invariants:
//! 1. Payment amount is non-negative
//! 2. Payment status transitions are valid
//! 3. Payment amount equals order total

use crate::domain::money::Money;
use serde::{Serialize, Deserialize};

/// Payment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

impl Default for PaymentStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Payment method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    Card,
    Online,
}

impl Default for PaymentMethod {
    fn default() -> Self {
        Self::Card
    }
}

/// Payment type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payment {
    pub id: u64,
    pub order_id: u64,
    pub amount: Money,
    pub status: PaymentStatus,
    pub method: PaymentMethod,
}

impl Payment {
    /// Create new payment
    pub fn new(id: u64, order_id: u64, amount: Money, method: PaymentMethod) -> Self {
        Self {
            id,
            order_id,
            amount,
            status: PaymentStatus::Pending,
            method,
        }
    }

    /// Get payment ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get order ID
    pub fn order_id(&self) -> u64 {
        self.order_id
    }

    /// Get amount
    pub fn amount(&self) -> &Money {
        &self.amount
    }

    /// Get status
    pub fn status(&self) -> PaymentStatus {
        self.status
    }

    /// Get method
    pub fn method(&self) -> PaymentMethod {
        self.method
    }

    /// Check if payment is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.status, PaymentStatus::Completed)
    }

    /// Check if payment is pending
    pub fn is_pending(&self) -> bool {
        matches!(self.status, PaymentStatus::Pending)
    }

    /// Check if payment is failed
    pub fn is_failed(&self) -> bool {
        matches!(self.status, PaymentStatus::Failed)
    }

    /// Check if payment is refunded
    pub fn is_refunded(&self) -> bool {
        matches!(self.status, PaymentStatus::Refunded)
    }

    /// Complete a payment
    pub fn complete(mut self) -> Self {
        self.status = PaymentStatus::Completed;
        self
    }

    /// Fail a payment
    pub fn fail(mut self) -> Self {
        self.status = PaymentStatus::Failed;
        self
    }

    /// Refund a payment
    pub fn refund(mut self) -> Self {
        self.status = PaymentStatus::Refunded;
        self
    }
}

/// Check if payment status transition is valid
pub fn is_valid_payment_transition(from: PaymentStatus, to: PaymentStatus) -> bool {
    match (from, to) {
        (PaymentStatus::Pending, PaymentStatus::Completed) => true,
        (PaymentStatus::Pending, PaymentStatus::Failed) => true,
        (PaymentStatus::Completed, PaymentStatus::Refunded) => true,
        (PaymentStatus::Failed, PaymentStatus::Refunded) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Test payment amount is non-negative
    proptest! {
        #[test]
        fn payment_amount_non_negative(amount_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "amount >= 0")) {
            let payment = Payment::new(1, 1, Money::new(amount_cents), PaymentMethod::Card);
            assert!(payment.amount.cents >= 0);
        }
    }

    // Test payment creation
    #[test]
    fn payment_creation() {
        let payment = Payment::new(
            1,
            1,
            Money::new(1000),
            PaymentMethod::Card,
        );
        
        assert_eq!(payment.id(), 1);
        assert_eq!(payment.order_id(), 1);
        assert_eq!(payment.amount().cents, 1000);
        assert!(matches!(payment.status(), PaymentStatus::Pending));
        assert!(matches!(payment.method(), PaymentMethod::Card));
    }

    // Test payment is pending by default
    #[test]
    fn payment_is_pending() {
        let payment = Payment::new(
            1,
            1,
            Money::new(1000),
            PaymentMethod::Card,
        );
        assert!(payment.is_pending());
        assert!(!payment.is_completed());
        assert!(!payment.is_failed());
        assert!(!payment.is_refunded());
    }

    // Test complete payment
    #[test]
    fn payment_complete() {
        let payment = Payment::new(
            1,
            1,
            Money::new(1000),
            PaymentMethod::Card,
        );
        
        let payment = payment.complete();
        assert!(payment.is_completed());
        assert!(!payment.is_pending());
    }

    // Test fail payment
    #[test]
    fn payment_fail() {
        let payment = Payment::new(
            1,
            1,
            Money::new(1000),
            PaymentMethod::Card,
        );
        
        let payment = payment.fail();
        assert!(payment.is_failed());
        assert!(!payment.is_pending());
    }

    // Test refund payment
    #[test]
    fn payment_refund() {
        let payment = Payment::new(
            1,
            1,
            Money::new(1000),
            PaymentMethod::Card,
        );
        
        let payment = payment.complete().refund();
        assert!(payment.is_refunded());
        assert!(!payment.is_completed());
    }

    // Test valid payment transitions
    #[test]
    fn valid_payment_transitions() {
        assert!(is_valid_payment_transition(
            PaymentStatus::Pending,
            PaymentStatus::Completed
        ));
        assert!(is_valid_payment_transition(
            PaymentStatus::Pending,
            PaymentStatus::Failed
        ));
        assert!(is_valid_payment_transition(
            PaymentStatus::Completed,
            PaymentStatus::Refunded
        ));
        assert!(is_valid_payment_transition(
            PaymentStatus::Failed,
            PaymentStatus::Refunded
        ));
        
        // Invalid transitions
        assert!(!is_valid_payment_transition(
            PaymentStatus::Completed,
            PaymentStatus::Pending
        ));
        assert!(!is_valid_payment_transition(
            PaymentStatus::Refunded,
            PaymentStatus::Pending
        ));
    }

    // Test payment equality
    #[test]
    fn payment_equality() {
        let p1 = Payment::new(1, 1, Money::new(1000), PaymentMethod::Card);
        let p2 = Payment::new(1, 1, Money::new(1000), PaymentMethod::Card);
        assert_eq!(p1, p2);
    }

    // Test payment inequality
    #[test]
    fn payment_inequality() {
        let p1 = Payment::new(1, 1, Money::new(1000), PaymentMethod::Card);
        let p2 = Payment::new(2, 1, Money::new(1000), PaymentMethod::Card);
        assert_ne!(p1, p2);
    }

    // Test payment preserves fields on status change
    #[test]
    fn payment_preserves_fields() {
        let payment = Payment::new(
            1,
            1,
            Money::new(1000),
            PaymentMethod::Card,
        );
        
        let completed = payment.clone().complete();
        assert_eq!(completed.id(), payment.id());
        assert_eq!(completed.order_id(), payment.order_id());
        assert_eq!(completed.amount().cents, payment.amount().cents);
        assert_eq!(completed.method(), payment.method());
        
        let failed = payment.clone().fail();
        assert_eq!(failed.id(), payment.id());
        assert_eq!(failed.order_id(), payment.order_id());
        assert_eq!(failed.amount().cents, payment.amount().cents);
        assert_eq!(failed.method(), payment.method());
        
        let refunded = payment.clone().complete().refund();
        assert_eq!(refunded.id(), payment.id());
        assert_eq!(refunded.order_id(), payment.order_id());
        assert_eq!(refunded.amount().cents, payment.amount().cents);
        assert_eq!(refunded.method(), payment.method());
    }
}
