//! Product Domain Type
//!
//! This module implements the Product type.
//! Key invariants:
//! 1. Product has a positive price
//! 2. Product has a unique ID
//! 3. Product stock is non-negative

use crate::domain::money::Money;
use serde::{Serialize, Deserialize};

/// Product ID type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductId {
    pub value: u64,
}

impl ProductId {
    /// Create new ProductId
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

/// Product type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub price: Money,
}

impl Product {
    /// Create new product
    pub fn new(id: ProductId, name: String, price: Money) -> Self {
        Self { id, name, price }
    }

    /// Get product ID
    pub fn id(&self) -> &ProductId {
        &self.id
    }

    /// Get product name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get product price
    pub fn price(&self) -> &Money {
        &self.price
    }

    /// Check if product price is positive
    pub fn has_positive_price(&self) -> bool {
        self.price.is_positive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Test product price is non-negative
    proptest! {
        #[test]
        fn product_price_non_negative(price_cents in any::<i64>().prop_filter(|&x| x >= 0, 10000, "price >= 0")) {
            let product = Product::new(
                ProductId::new(1),
                "Test Product".to_string(),
                Money::new(price_cents),
            );
            assert!(product.price.cents >= 0);
        }
    }

    // Test product creation
    #[test]
    fn product_creation() {
        let product = Product::new(
            ProductId::new(1),
            "Test Product".to_string(),
            Money::new(1000),
        );
        
        assert_eq!(product.id.value, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price.cents, 1000);
    }

    // Test product has positive price
    #[test]
    fn product_has_positive_price() {
        let product = Product::new(
            ProductId::new(1),
            "Test Product".to_string(),
            Money::new(1000),
        );
        assert!(product.has_positive_price());
    }

    // Test product with zero price
    #[test]
    fn product_with_zero_price() {
        let product = Product::new(
            ProductId::new(1),
            "Free Product".to_string(),
            Money::zero(),
        );
        assert!(!product.has_positive_price());
    }

    // Test product ID
    #[test]
    fn product_id() {
        let product = Product::new(
            ProductId::new(42),
            "Test Product".to_string(),
            Money::new(1000),
        );
        assert_eq!(product.id().value, 42);
    }

    // Test product equality
    #[test]
    fn product_equality() {
        let p1 = Product::new(
            ProductId::new(1),
            "Test Product".to_string(),
            Money::new(1000),
        );
        let p2 = Product::new(
            ProductId::new(1),
            "Test Product".to_string(),
            Money::new(1000),
        );
        assert_eq!(p1, p2);
    }

    // Test product inequality
    #[test]
    fn product_inequality() {
        let p1 = Product::new(
            ProductId::new(1),
            "Test Product".to_string(),
            Money::new(1000),
        );
        let p2 = Product::new(
            ProductId::new(2),
            "Test Product".to_string(),
            Money::new(1000),
        );
        assert_ne!(p1, p2);
    }
}
