/-
# Coupon Module - Formal Proofs for Coupons (Order #4)

A coupon is a code plus a Discount plus an applicability condition
(minimum subtotal). It only applies when the condition is met.

## Key Invariants:
1. A coupon whose condition fails yields a zero discount.
2. A valid coupon yields a non-negative discount on a non-negative subtotal.
3. A coupon never discounts more than the subtotal.
No Mathlib: core Lean 4 only. Zero sorry, zero axiom.
-/

import Shop.Money
import Shop.Rate
import Shop.Discount

namespace Shop

/-- Coupon applicability condition -/
structure CouponCondition where
  minSubtotal : Money
  deriving Repr, DecidableEq

/-- Coupon: code + discount + condition -/
structure Coupon where
  code : String
  discount : Discount
  condition : CouponCondition
  deriving Repr, DecidableEq

namespace Coupon

/-- Is the coupon applicable to this subtotal? -/
def isApplicable (c : Coupon) (subtotal : Money) : Prop :=
  c.condition.minSubtotal.cents ≤ subtotal.cents

/-- Apply a coupon: the discount amount if applicable, otherwise zero -/
def apply (c : Coupon) (subtotal : Money) : Money :=
  if c.condition.minSubtotal.cents ≤ subtotal.cents then
    Discount.amount c.discount subtotal
  else
    Money.zero

/-- Validity: valid discount and a non-negative minimum -/
def valid (c : Coupon) : Prop :=
  Discount.valid c.discount ∧ 0 ≤ c.condition.minSubtotal.cents

/-- The zero coupon: empty code, zero discount, no minimum -/
def zero : Coupon :=
  { code := ""
    discount := Discount.zero
    condition := { minSubtotal := Money.zero } }

/-! ### Theorems -/

/-- A non-applicable coupon yields a zero discount -/
theorem coupon_not_applicable_zero (c : Coupon) (subtotal : Money)
    (h : ¬ Coupon.isApplicable c subtotal) :
    Coupon.apply c subtotal = Money.zero := by
  show (if c.condition.minSubtotal.cents ≤ subtotal.cents
      then Discount.amount c.discount subtotal else Money.zero) = Money.zero
  rw [if_neg h]

/-- A valid coupon yields a non-negative discount on a non-negative subtotal -/
theorem coupon_non_negative (c : Coupon) (subtotal : Money)
    (hs : 0 ≤ subtotal.cents) (hc : Coupon.valid c) :
    0 ≤ (Coupon.apply c subtotal).cents := by
  show 0 ≤ (if c.condition.minSubtotal.cents ≤ subtotal.cents
      then Discount.amount c.discount subtotal else Money.zero).cents
  split
  · next h => exact Discount.discount_non_negative c.discount subtotal hc.1 hs
  · next h => exact Int.le_refl 0

/-- A coupon never discounts more than the subtotal -/
theorem coupon_bounded (c : Coupon) (subtotal : Money)
    (hs : 0 ≤ subtotal.cents) (hc : Coupon.valid c) :
    (Coupon.apply c subtotal).cents ≤ subtotal.cents := by
  show (if c.condition.minSubtotal.cents ≤ subtotal.cents
      then Discount.amount c.discount subtotal else Money.zero).cents
      ≤ subtotal.cents
  split
  · next h => exact Discount.discount_bounded c.discount subtotal hc.1 hs
  · next h => exact hs

/-- The zero coupon always yields a zero discount -/
theorem coupon_zero_amount (subtotal : Money) (hs : 0 ≤ subtotal.cents) :
    Coupon.apply Coupon.zero subtotal = Money.zero := by
  show (if (0 : Int) ≤ subtotal.cents
      then Discount.amount Discount.zero subtotal else Money.zero) = Money.zero
  rw [if_pos hs]
  exact Discount.discount_zero_amount subtotal hs

end Coupon
end Shop
