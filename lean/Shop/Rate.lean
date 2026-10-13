/-
# Rate Module - Basis-Point Rates

A rate is expressed in basis points (bp): 1% = 100 bp, so 9% = 900 bp.
Rates in [0, 10000] bp are valid. This module is the prerequisite for Tax.
No Mathlib: core Lean 4 only.

## Key facts:
1. apply r base = (base.cents * r.bp) / 10000   (integer division, truncation)
2. A zero rate gives zero money.
3. Applying a valid rate to a non-negative base is non-negative.
4. Application is monotone for valid rates.
-/

import Shop.Money

namespace Shop

/-- Rate in basis points (1% = 100 bp) -/
@[ext] structure Rate where
  bp : Int
  deriving Repr, DecidableEq, Inhabited

namespace Rate

/-- Zero rate -/
def zero : Rate := ⟨0⟩

/-- Sum of two rates (for display purposes only) -/
def add (a b : Rate) : Rate := ⟨a.bp + b.bp⟩

/-- Validity: 0 ≤ bp ≤ 10000 -/
def valid (r : Rate) : Prop := 0 ≤ r.bp ∧ r.bp ≤ 10000

/-- Apply a rate to a money amount: (base * bp) / 10000 -/
def apply (r : Rate) (base : Money) : Money := ⟨base.cents * r.bp / 10000⟩

/-- Zero rate applied to any base gives zero -/
theorem zero_apply (base : Money) : Rate.apply Rate.zero base = Money.zero := by
  apply Money.ext
  show base.cents * 0 / 10000 = 0
  omega

/-- A valid rate applied to a non-negative base is non-negative -/
theorem apply_non_negative (r : Rate) (base : Money)
    (hr : Rate.valid r) (hb : 0 ≤ base.cents) :
    0 ≤ (Rate.apply r base).cents := by
  have h : 0 ≤ base.cents * r.bp := Int.mul_nonneg hb hr.1
  show 0 ≤ base.cents * r.bp / 10000
  omega

/-- Application of a valid rate is monotone in the base -/
theorem apply_monotone (r : Rate) (a b : Money)
    (hr : Rate.valid r) (ha : 0 ≤ a.cents) (hab : a.cents ≤ b.cents) :
    (Rate.apply r a).cents ≤ (Rate.apply r b).cents := by
  have hstep : b.cents * r.bp = a.cents * r.bp + (b.cents - a.cents) * r.bp := by
    have hb' : b.cents = a.cents + (b.cents - a.cents) := by omega
    rw [hb', Int.add_mul]
  have hpos : 0 ≤ (b.cents - a.cents) * r.bp :=
    Int.mul_nonneg (by omega) hr.1
  show a.cents * r.bp / 10000 ≤ b.cents * r.bp / 10000
  rw [hstep]
  omega

end Rate
end Shop
