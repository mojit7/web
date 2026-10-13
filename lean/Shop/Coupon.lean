/-
# Coupon Module - Formal Proofs for Coupon Operations

This module provides the formal definitions for coupon operations.
Coupons are a type of discount that can be applied to orders.

## Key Invariants:
1. Coupons have a discount value
2. Coupons can be active or expired
3. Coupons can have usage limits
-/

import Shop.Money
import Shop.Discount
import Shop.Rate
import Mathlib.Data.Int.Basic

namespace Shop

-- Coupon type
@[ext]
structure Coupon where
  code : String
  discount : Discount
  is_active : Bool
  max_uses : Nat
  uses : Nat

-- Create a new coupon
def Coupon.new (code : String) (discount : Discount) (max_uses : Nat) : Coupon := 
  ⟨code, discount, true, max_uses, 0⟩

-- Zero coupon (no discount)
def Coupon.zero : Coupon := 
  ⟨"", Discount.zero, false, 0, 0⟩

-- Check if coupon is valid (active and has uses remaining)
def Coupon.isValid (c : Coupon) : Bool := 
  c.is_active && c.uses < c.max_uses

-- Apply coupon to a money amount
def applyCoupon (amount : Money) (coupon : Coupon) : Money := 
  if coupon.isValid then
    amount.sub coupon.discount.amount
  else
    amount

-- Theorem: Applying valid coupon reduces amount
theorem coupon_reduces_amount (amount : Money) (coupon : Coupon) : 
  coupon.isValid → coupon.discount.amount.cents > 0 → 
  (applyCoupon amount coupon).cents < amount.cents := by
  intro h_valid h_positive
  simp [applyCoupon, Coupon.isValid] at h_valid ⊢
  split at h_valid
  · rename_i h_active h_uses
    simp [h_active, h_uses]
    have : coupon.discount.amount.cents > 0 := h_positive
    have : amount.cents - coupon.discount.amount.cents < amount.cents := by
      apply Int.sub_lt_self
      exact Int.ofNat_pos.mpr (Nat.pos_of_ne_zero (by
        -- coupon.discount.amount.cents > 0
        -- This needs to be connected to the Money type
        sorry))
    exact this
  · contradiction

-- Theorem: Applying invalid coupon returns original amount
theorem coupon_invalid_no_change (amount : Money) (coupon : Coupon) : 
  ¬coupon.isValid → applyCoupon amount coupon = amount := by
  intro h_invalid
  simp [applyCoupon, Coupon.isValid]
  split at h_invalid
  · rename_i h_not_active
    simp [h_not_active]
  · rename_i h_uses_exhausted
    simp [h_uses_exhausted]
  · simp [h_invalid]

-- Theorem: Zero coupon returns original amount
theorem coupon_zero_no_change (amount : Money) : 
  applyCoupon amount Coupon.zero = amount := by
  simp [applyCoupon, Coupon.zero, Coupon.isValid, Discount.zero]
  -- Coupon.zero.is_active = false, so coupon is not valid
  -- Therefore, applyCoupon returns amount
  rfl

-- Use a coupon (increment uses counter)
def Coupon.use (c : Coupon) : Coupon := 
  { c with uses := c.uses + 1 }

-- Theorem: Using a coupon increments uses
theorem coupon_use_increments : ∀ c : Coupon, (c.use).uses = c.uses + 1 := by
  intro c
  rfl

-- Theorem: Using a coupon preserves other fields
theorem coupon_use_preserves : ∀ c : Coupon, 
  (c.use).code = c.code ∧ 
  (c.use).discount = c.discount ∧ 
  (c.use).is_active = c.is_active ∧ 
  (c.use).max_uses = c.max_uses := by
  intro c
  constructor
  · rfl
  constructor
  · rfl
  constructor
  · rfl
  · rfl

end Shop
