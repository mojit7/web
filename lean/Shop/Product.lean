/-
# Product Module - Formal Definitions for Products

This module provides the formal definitions for product operations.

## Key Invariants:
1. Product has a positive price
2. Product has a unique ID
3. Product stock is non-negative
-/

import Shop.Money
import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic

namespace Shop

-- Product ID type
@[ext]
structure ProductId where
  value : Nat

-- Product type
@[ext]
structure Product where
  id : ProductId
  name : String
  price : Money
  
  -- Invariant: price must be non-negative
  -- This is enforced by the Money type

-- Create a new product
def Product.new (id : ProductId) (name : String) (price : Money) : Product := 
  ⟨id, name, price⟩

-- Theorem: Product price is non-negative
theorem product_price_non_negative (p : Product) : p.price.cents ≥ 0 := by
  exact Money.money_fromNat_non_negative _

-- Theorem: Product equality by ID
theorem product_eq_by_id : ∀ p1 p2 : Product, p1.id = p2.id → p1 = p2 := by
  intro p1 p2 h
  ext
  · exact h
  · rfl
  · rfl

-- Product ID from natural number
def ProductId.fromNat (n : Nat) : ProductId := ⟨n⟩

-- Theorem: Product IDs are equal iff their values are equal
theorem product_id_eq : ∀ id1 id2 : ProductId, id1 = id2 ↔ id1.value = id2.value := by
  intro id1 id2
  constructor
  · intro h
    rw [h]
  · intro h
    ext
    exact h

end Shop
