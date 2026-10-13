//! Rate Domain Type
//!
//! This module implements the Rate type for tax and discount calculations.
//! Key invariants:
//! 1. Rate is between 0 and 100 (percentage)
//! 2. Rate operations preserve bounds

use serde::{Serialize, Deserialize};
use std::ops::Mul;

/// Rate type: percentage as u8 (0-100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Rate {
    value: u8,
}

impl Rate {
    /// Create new Rate with validation
    /// 
    /// # Panics
    /// Panics if value > 100
    pub fn new(value: u8) -> Self {
        if value > 100 {
            panic!("Rate cannot be greater than 100: {}", value);
        }
        Self { value }
    }

    /// Zero rate
    pub const fn zero() -> Self {
        Self { value: 0 }
    }

    /// Maximum rate (100%)
    pub const fn max() -> Self {
        Self { value: 100 }
    }

    /// Get rate value
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Check if rate is zero
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    /// Convert to integer for calculations
    pub fn to_int(&self) -> i64 {
        self.value as i64
    }

    /// Convert to float for display (but never used in calculations)
    pub fn to_float(&self) -> f64 {
        self.value as f64 / 100.0
    }
}

// Rate is always between 0 and 100
impl Rate {
    /// Returns true if rate is valid (0-100)
    pub fn is_valid(&self) -> bool {
        self.value <= 100
    }
}

// Multiplication of Rate by Money is not implemented here
// It's implemented in the tax module where the formula is defined

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Test rate is always <= 100
    proptest! {
        #[test]
        fn rate_le_100(value in 0..=100u8) {
            let rate = Rate::new(value);
            assert!(rate.value() <= 100);
        }
    }

    // Test rate is always >= 0
    proptest! {
        #[test]
        fn rate_ge_0(value in 0..=100u8) {
            let rate = Rate::new(value);
            assert!(rate.value() >= 0);
        }
    }

    // Test zero rate
    #[test]
    fn zero_rate() {
        let rate = Rate::zero();
        assert_eq!(rate.value(), 0);
        assert!(rate.is_zero());
    }

    // Test max rate
    #[test]
    fn max_rate() {
        let rate = Rate::max();
        assert_eq!(rate.value(), 100);
    }

    // Test rate creation panics on invalid value
    #[test]
    #[should_panic]
    fn rate_invalid_panics() {
        let _ = Rate::new(101);
    }

    // Test rate is valid
    #[test]
    fn rate_is_valid() {
        assert!(Rate::zero().is_valid());
        assert!(Rate::max().is_valid());
        assert!(Rate::new(50).is_valid());
    }

    // Test to_int
    #[test]
    fn to_int() {
        assert_eq!(Rate::zero().to_int(), 0);
        assert_eq!(Rate::max().to_int(), 100);
        assert_eq!(Rate::new(50).to_int(), 50);
    }

    // Test to_float
    #[test]
    fn to_float() {
        assert_eq!(Rate::zero().to_float(), 0.0);
        assert_eq!(Rate::max().to_float(), 1.0);
        assert_eq!(Rate::new(50).to_float(), 0.5);
    }
}
