/-
# Discount Module - Formal Proofs for Discount Calculations (Order #4)

Discounts build on Money and Rate. A discount is either a flat Money amount
or a percentage given as a basis-point Rate (1% = 100 bp).

## Key Invariants:
1. A valid discount amount is never negative (on a non-negative subtotal).
2. A valid discount amount never exceeds the subtotal.
3. Applying a discount never yields a negative result.
No Mathlib: core Lean 4 only. Zero sorry, zero axiom.
-/

import Shop.Money
import Shop.Rate

namespace Shop

/-- Discount type: flat amount or percentage (basis points) -/
inductive Discount where
  | flat : Money → Discount
  | percent : Rate → Discount
  deriving Repr, DecidableEq

namespace Discount

/-- Zero discount -/
def zero : Discount := .flat Money.zero

/-- Flat (fixed amount) discount -/
def mkFlat (m : Money) : Discount := .flat m

/-- Percentage discount (basis points) -/
def mkPercent (r : Rate) : Discount := .percent r

/-- Discount amount on a subtotal: a flat discount is capped at the subtotal -/
def amount (d : Discount) (subtotal : Money) : Money :=
  match d with
  | .flat m => if m.cents ≤ subtotal.cents then m else subtotal
  | .percent r => Rate.apply r subtotal

/-- Validity: flat discounts are non-negative; percent discounts carry a valid rate -/
def valid (d : Discount) : Prop :=
  match d with
  | .flat m => 0 ≤ m.cents
  | .percent r => Rate.valid r

/-- Apply: the discounted amount is subtotal - discount amount -/
def apply (d : Discount) (subtotal : Money) : Money :=
  Money.sub subtotal (Discount.amount d subtotal)

/-! ### Theorems -/

/-- Helper: multiplying by a non-negative factor is monotone
    (proved with core Lean only) -/
private theorem mul_le_mul_const (s t : Int) (hs : 0 ≤ s) (ht0 : 0 ≤ t)
    (ht : t ≤ 10000) : s * t ≤ s * 10000 := by
  have h1 : 0 ≤ s * (10000 - t) := Int.mul_nonneg hs (by omega)
  have h2 : s * 10000 = s * t + s * (10000 - t) := by
    have h3 : t + (10000 - t) = 10000 := by omega
    calc s * 10000 = s * (t + (10000 - t)) := by rw [h3]
      _ = s * t + s * (10000 - t) := Int.mul_add
  omega

/-- A valid discount amount is never negative (non-negative subtotal) -/
theorem discount_non_negative (d : Discount) (subtotal : Money)
    (hd : Discount.valid d) (hs : 0 ≤ subtotal.cents) :
    0 ≤ (Discount.amount d subtotal).cents := by
  cases d with
  | flat m =>
    have hm : 0 ≤ m.cents := hd
    show 0 ≤ (if m.cents ≤ subtotal.cents then m else subtotal).cents
    split
    · next h => exact hm
    · next h => exact hs
  | percent r =>
    have hv : Rate.valid r := hd
    show 0 ≤ (Rate.apply r subtotal).cents
    exact Rate.apply_non_negative r subtotal hv hs

/-- A valid discount amount never exceeds the subtotal -/
theorem discount_bounded (d : Discount) (subtotal : Money)
    (hd : Discount.valid d) (hs : 0 ≤ subtotal.cents) :
    (Discount.amount d subtotal).cents ≤ subtotal.cents := by
  cases d with
  | flat m =>
    show (if m.cents ≤ subtotal.cents then m else subtotal).cents ≤ subtotal.cents
    split
    · next h => exact h
    · next h => exact Int.le_refl subtotal.cents
  | percent r =>
    have hv : Rate.valid r := hd
    have key : subtotal.cents * r.bp ≤ subtotal.cents * 10000 :=
      mul_le_mul_const subtotal.cents r.bp hs hv.1 hv.2
    show subtotal.cents * r.bp / 10000 ≤ subtotal.cents
    omega

/-- The zero discount on a non-negative subtotal is zero -/
theorem discount_zero_amount (subtotal : Money) (hs : 0 ≤ subtotal.cents) :
    Discount.amount Discount.zero subtotal = Money.zero := by
  show (if (0 : Int) ≤ subtotal.cents then Money.zero else subtotal) = Money.zero
  rw [if_pos hs]

/-- The zero discount amount (in cents) is exactly zero -/
theorem discount_zero_nonneg (subtotal : Money) (hs : 0 ≤ subtotal.cents) :
    (Discount.amount Discount.zero subtotal).cents = 0 := by
  rw [discount_zero_amount subtotal hs]

/-- Applying a valid discount to a non-negative subtotal never goes negative -/
theorem apply_non_negative (d : Discount) (subtotal : Money)
    (hd : Discount.valid d) (hs : 0 ≤ subtotal.cents) :
    0 ≤ (Discount.apply d subtotal).cents := by
  have h1 := discount_bounded d subtotal hd hs
  show 0 ≤ subtotal.cents - (Discount.amount d subtotal).cents
  omega

end Discount
end Shop
