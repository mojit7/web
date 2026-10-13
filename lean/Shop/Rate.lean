/-
# Rate Module - Formal Proofs for Rate Calculations

This module provides the formal proofs for rate operations.
Rates are used for tax calculations and discounts.

## Key Invariants:
1. Rate is between 0 and 100 (percentage)
2. Rate operations preserve bounds
-/

import Shop.Money
import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic

namespace Shop

-- Rate type: percentage as natural number (0-100)
@[ext]
structure Rate where
  value : Nat
  
  -- Invariant: rate must be between 0 and 100
  valid : value ≤ 100

def Rate.new (v : Nat) (h : v ≤ 100) : Rate := ⟨v, h⟩

-- Zero rate
def Rate.zero : Rate := ⟨0, Nat.zero_le 100⟩

-- Maximum rate (100%)
def Rate.max : Rate := ⟨100, Nat.le_refl 100⟩

-- Theorem: Rate is always ≤ 100
theorem rate_le_100 : ∀ r : Rate, r.value ≤ 100 := by
  intro r
  exact r.valid

-- Theorem: Rate is always ≥ 0
theorem rate_ge_0 : ∀ r : Rate, r.value ≥ 0 := by
  intro r
  exact Nat.zero_le r.value

-- Theorem: Zero rate is valid
theorem zero_rate_valid : Rate.zero.value = 0 := by
  rfl

-- Theorem: Max rate is 100
theorem max_rate_value : Rate.max.value = 100 := by
  rfl

-- Rate equality
theorem rate_eq : ∀ r1 r2 : Rate, r1.value = r2.value → r1 = r2 := by
  intro r1 r2 h
  ext
  exact h

-- Convert rate to integer for calculations
def Rate.toInt (r : Rate) : Int := ↑r.value

-- Convert rate to float for display (but never used in calculations)
-- This is for display purposes only, not for financial calculations
-- def Rate.toFloat (r : Rate) : Float := (r.value : Float) / 100.0

end Shop
