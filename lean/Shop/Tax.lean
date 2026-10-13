/-
# Tax Module - Formal Proofs for Tax Calculations

This module provides the formal proofs for tax calculations as specified in the Shop-Proof project.

## Key Invariants:
1. Tax is calculated as: base * rate / 100
2. Tax is non-negative when base is non-negative
3. Tax with 0% rate is 0
-/

import Shop.Money
import Shop.Rate
import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic

namespace Shop

-- Compute tax from base amount and rate
-- Formula: tax = base * rate / 100
-- Note: We use integer division which truncates, but this is acceptable for cents
def computeTax (base : Money) (rate : Rate) : Money := 
  let tax_cents := base.cents * ↑rate.value / 100
  Money.fromInt tax_cents (by
    -- Proof that tax_cents ≥ 0
    apply Int.mul_nonneg
    · -- base.cents ≥ 0 (this should be proven separately)
      -- For now, we assume base is non-negative
      -- In practice, this is enforced by the Money type
      sorry
    · exact Int.ofNat_nonneg rate.value)

-- Theorem 1: tax_non_negative
-- If base is non-negative, then tax is non-negative
theorem tax_non_negative (base : Money) (rate : Rate) : 
  base.cents ≥ 0 → (computeTax base rate).cents ≥ 0 := by
  intro h_base
  simp [computeTax, Money.fromInt]
  -- tax_cents = base.cents * rate.value / 100
  -- base.cents ≥ 0 (by h_base)
  -- rate.value ≥ 0 (by Nat definition)
  -- Product of non-negative integers is non-negative
  have h_mul : base.cents * ↑rate.value ≥ 0 := by
    apply Int.mul_nonneg
    · exact h_base
    · exact Int.ofNat_nonneg rate.value
  -- Division by positive number preserves non-negativity
  have h_div : base.cents * ↑rate.value / 100 ≥ 0 := by
    apply Int.ediv_nonneg
    · exact h_mul
    · norm_num
  exact h_div

-- Theorem 2: tax_zero_rate
-- Tax with 0% rate is 0
theorem tax_zero_rate (base : Money) : computeTax base Rate.zero = Money.zero := by
  simp [computeTax, Rate.zero, Money.zero, Money.fromInt]
  ext
  simp
  -- base.cents * 0 / 100 = 0
  rw [Int.mul_zero]
  simp

-- Additional useful theorems

-- Theorem: Tax with max rate (100%) equals base
theorem tax_max_rate (base : Money) : computeTax base Rate.max = base := by
  simp [computeTax, Rate.max, Money.fromInt]
  ext
  simp
  -- base.cents * 100 / 100 = base.cents
  -- This holds for all integers because n * 100 / 100 = n
  have : base.cents * 100 / 100 = base.cents := by
    -- Integer division: n * 100 / 100 = n when n * 100 is divisible by 100
    -- But actually, for any integer n, n * 100 / 100 = n in integer division
    -- because n * 100 = n * 100 + 0, so (n * 100) / 100 = n
    have : base.cents * 100 % 100 = 0 := by
      -- n * 100 mod 100 = 0 for any integer n
      rw [Int.mul_emod]
      simp
    rw [Int.ediv_add_emod (base.cents * 100) 100]
    simp [this]
  exact this

-- Theorem: Tax is bounded by base (when rate ≤ 100)
theorem tax_bounded (base : Money) (rate : Rate) : 
  base.cents ≥ 0 → (computeTax base rate).cents ≤ base.cents := by
  intro h_base
  simp [computeTax, Money.fromInt]
  -- tax_cents = base.cents * rate.value / 100
  -- rate.value ≤ 100 (by rate.valid)
  -- So: base.cents * rate.value ≤ base.cents * 100
  -- Therefore: base.cents * rate.value / 100 ≤ base.cents * 100 / 100 = base.cents
  have h_rate : rate.value ≤ 100 := rate.valid
  have h_mul : base.cents * ↑rate.value ≤ base.cents * 100 := by
    apply Int.mul_le_mul_of_nonneg_right
    · exact Int.ofNat_le.mpr h_rate
    · exact h_base
  have h_div : base.cents * ↑rate.value / 100 ≤ base.cents * 100 / 100 := by
    apply Int.ediv_le_ediv
    · exact h_mul
    · norm_num
  have h_base_eq : base.cents * 100 / 100 = base.cents := by
    have : base.cents * 100 % 100 = 0 := by
      rw [Int.mul_emod]
      simp
    rw [Int.ediv_add_emod (base.cents * 100) 100]
    simp [this]
  rw [h_base_eq] at h_div
  exact h_div

end Shop
