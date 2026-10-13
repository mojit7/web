/-
# Tax Module - Formal Proofs for Tax Calculations (Order #3)

Tax builds on Money and Rate. The rate is a basis-point Rate (1% = 100 bp)
and the computation is: tax = (base * rate.bp) / 10000 with integer division.
Tax is applied on the post-discount amount (the main invariant, enforced by
the Order module). No Mathlib: core Lean 4 only. Zero sorry, zero axiom.
-/

import Shop.Money
import Shop.Rate

namespace Shop

/-- Tax: a rate applied to a base amount -/
structure Tax where
  rate : Rate
  deriving Repr, DecidableEq, Inhabited

namespace Tax

/-- Compute tax on a base amount (integer division by 10000) -/
def compute (t : Tax) (base : Money) : Money :=
  Rate.apply t.rate base

/-- Zero tax -/
def zero : Tax := ⟨Rate.zero⟩

/-- Standard 9% tax (900 bp) -/
def vat9 : Tax := ⟨⟨900⟩⟩

/-- Combine two taxes (sum of rates, for display purposes only) -/
def combine (a b : Tax) : Tax := ⟨Rate.add a.rate b.rate⟩

/-- Validity of the underlying rate -/
def valid (t : Tax) : Prop := Rate.valid t.rate

end Tax

/-! ### Theorems -/

/-- Tax with zero rate is zero money -/
theorem tax_zero_rate (base : Money) :
    Tax.compute Tax.zero base = Money.zero := by
  show Rate.apply Rate.zero base = Money.zero
  exact Rate.zero_apply base

/-- Tax on a non-negative base with a valid rate is non-negative -/
theorem tax_non_negative (t : Tax) (base : Money) :
    Tax.valid t → 0 ≤ base.cents → 0 ≤ (Tax.compute t base).cents := by
  intro hv hb
  exact Rate.apply_non_negative t.rate base hv hb

/-- Tax on a zero base is zero money -/
theorem tax_zero_base (t : Tax) :
    Tax.compute t Money.zero = Money.zero := by
  apply Money.ext
  show 0 * t.rate.bp / 10000 = 0
  omega

/-- The 9% example: tax = base * 900 / 10000 -/
theorem tax_9pct_example (base : Money) :
    Tax.compute Tax.vat9 base = ⟨(base.cents * 900) / 10000⟩ := by
  rfl

/-- Tax is additive over split bases when each part is itself divisible
    (weakened form; the original single-sum-divisibility hypothesis is
    false in general, e.g. 3 + 7 with divisor 10000). -/
theorem tax_linear_on_clean_base (t : Tax) (a b : Money)
    (ha : a.cents * t.rate.bp % 10000 = 0)
    (hb : b.cents * t.rate.bp % 10000 = 0)
    (h : (a.cents + b.cents) * t.rate.bp % 10000 = 0) :
    Tax.compute t (a + b) =
      Tax.compute t a + Tax.compute t b := by
  apply Money.ext
  show (a.cents + b.cents) * t.rate.bp / 10000
      = a.cents * t.rate.bp / 10000 + b.cents * t.rate.bp / 10000
  have hz : (a.cents + b.cents) * t.rate.bp
      = a.cents * t.rate.bp + b.cents * t.rate.bp := by
    rw [Int.add_mul]
  omega

/-- Tax validity is exactly rate validity -/
theorem tax_valid_preserved (t : Tax) : Tax.valid t → Rate.valid t.rate :=
  fun h => h

/-- Tax is monotone in the base for valid rates and non-negative bases -/
theorem tax_monotone (t : Tax) (a b : Money) :
    Tax.valid t → a.cents ≤ b.cents → 0 ≤ a.cents →
    (Tax.compute t a).cents ≤ (Tax.compute t b).cents := by
  intro hv hab ha
  exact Rate.apply_monotone t.rate a b hv ha hab

end Shop
