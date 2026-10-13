/-
# Stock Module - Formal Proofs for Stock/Inventory Management

This module provides the formal proofs for stock operations as specified in the Shop-Proof project.

## Key Invariants:
1. Stock quantity is never negative
2. Stock operations preserve non-negativity
3. Decrementing stock is safe when amount ≤ quantity
-/

import Shop.Product
import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic

namespace Shop

-- Stock type
@[ext]
structure Stock where
  product_id : ProductId
  qty : Nat  -- Quantity is always non-negative (Nat)

-- Create new stock
def Stock.new (product_id : ProductId) (qty : Nat) : Stock := 
  ⟨product_id, qty⟩

-- Zero stock
def Stock.zero (product_id : ProductId) : Stock := 
  ⟨product_id, 0⟩

-- Theorem 1: stock_non_negative
-- Stock quantity is always non-negative (by construction, since qty is Nat)
theorem stock_non_negative (s : Stock) : s.qty ≥ 0 := by
  exact Nat.zero_le s.qty

-- Increment stock
def Stock.increment (s : Stock) (n : Nat) : Stock := 
  ⟨s.product_id, s.qty + n⟩

-- Decrement stock
def Stock.decrement (s : Stock) (n : Nat) : Stock := 
  if n ≤ s.qty then
    ⟨s.product_id, s.qty - n⟩
  else
    s  -- Return unchanged if not enough stock

-- Theorem 2: stock_decrement_safe
-- If n ≤ qty, then decrementing by n gives non-negative result
theorem stock_decrement_safe (s : Stock) (n : Nat) : 
  n ≤ s.qty → (s.decrement n).qty ≥ 0 := by
  intro h
  simp [Stock.decrement]
  split
  · rename_i h_le
    -- s.qty - n ≥ 0 because n ≤ s.qty
    have : s.qty - n ≥ 0 := Nat.sub_le s.qty n
    exact Nat.zero_le (s.qty - n)
  · contradiction

-- Theorem: Decrementing by 0 returns original stock
theorem stock_decrement_zero (s : Stock) : s.decrement 0 = s := by
  simp [Stock.decrement]
  -- 0 ≤ s.qty is always true
  have : 0 ≤ s.qty := Nat.zero_le s.qty
  simp [this]
  -- s.qty - 0 = s.qty
  rfl

-- Theorem: Incrementing stock preserves product_id
theorem stock_increment_preserves_id (s : Stock) (n : Nat) : 
  (s.increment n).product_id = s.product_id := by
  rfl

-- Theorem: Decrementing stock preserves product_id
theorem stock_decrement_preserves_id (s : Stock) (n : Nat) : 
  (s.decrement n).product_id = s.product_id := by
  simp [Stock.decrement]
  split
  · rename_i _
    rfl
  · rfl

-- Theorem: Incrementing and then decrementing by same amount returns original
theorem stock_inc_dec_eq (s : Stock) (n : Nat) : 
  (s.increment n).decrement n = s := by
  simp [Stock.increment, Stock.decrement]
  -- (s.qty + n) - n = s.qty
  have : n ≤ s.qty + n := Nat.le_add_right s.qty n
  simp [this]
  rw [Nat.add_sub_cancel]

-- Check if stock has enough quantity
def Stock.hasEnough (s : Stock) (n : Nat) : Bool := 
  n ≤ s.qty

-- Theorem: If hasEnough is true, then decrement is safe
theorem stock_has_enough_then_safe (s : Stock) (n : Nat) : 
  s.hasEnough n → (s.decrement n).qty ≥ 0 := by
  intro h
  exact stock_decrement_safe s n h

-- Theorem: If hasEnough is false, then decrement returns original
theorem stock_not_enough_returns_original (s : Stock) (n : Nat) : 
  ¬s.hasEnough n → s.decrement n = s := by
  intro h
  simp [Stock.hasEnough, Stock.decrement] at h ⊢
  split at h
  · contradiction
  · rfl

-- Set stock quantity
def Stock.setQty (s : Stock) (qty : Nat) : Stock := 
  ⟨s.product_id, qty⟩

-- Theorem: Setting quantity preserves product_id
theorem stock_set_qty_preserves_id (s : Stock) (qty : Nat) : 
  (s.setQty qty).product_id = s.product_id := by
  rfl

-- Theorem: Setting quantity to 0 gives zero stock
theorem stock_set_zero (s : Stock) : s.setQty 0 = Stock.zero s.product_id := by
  ext
  · rfl
  · rfl

end Shop
