//! Invariants Module
//!
//! This module contains runtime invariant checks for all domain operations.
//! Each check corresponds to a theorem proven in Lean 4.

pub mod money;
pub mod order;
pub mod stock;
pub mod discount;
pub mod tax;

use thiserror::Error;

/// Error type for invariant violations
#[derive(Debug, Error)]
pub enum InvariantError {
    #[error("Money invariant violated: {0}")]
    MoneyViolation(String),
    
    #[error("Order invariant violated: {0}")]
    OrderViolation(String),
    
    #[error("Stock invariant violated: {0}")]
    StockViolation(String),
    
    #[error("Discount invariant violated: {0}")]
    DiscountViolation(String),
    
    #[error("Tax invariant violated: {0}")]
    TaxViolation(String),
}

/// Macro for invariant checks
/// If the condition is false, returns an error
#[macro_export]
macro_rules! check {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error);
        }
    };
}

/// Macro for invariant checks that panic (for internal use)
#[macro_export]
macro_rules! invariant {
    ($condition:expr, $message:expr) => {
        if !$condition {
            panic!("{}: {}", file!(), $message);
        }
    };
}
