/-
# Money Module - Formal Proofs for Monetary Calculations

All theorems are proven without sorry, without Mathlib (core Lean 4 only).

## Key Invariants:
1. Money is represented as integer cents (never float)
2. Money addition is associative and commutative
3. Money subtraction of self is zero
4. Money multiplication distributes over addition
-/

namespace Shop

/-- Money: integer cents -/
@[ext] structure Money where
  cents : Int
  deriving Repr, DecidableEq, Inhabited

namespace Money

/-- Zero money -/
@[simp] def zero : Money := ⟨0⟩

/-- Money from a natural number -/
def fromNat (n : Nat) : Money := ⟨↑n⟩

/-- Money from an integer with a proof of non-negativity -/
def fromInt (n : Int) (h : n ≥ 0) : Money := ⟨n⟩

/-- Money addition -/
instance : Add Money where
  add a b := ⟨a.cents + b.cents⟩

/-- Money subtraction -/
def sub (m1 m2 : Money) : Money := ⟨m1.cents - m2.cents⟩

/-- Money multiplication by a natural number -/
def mul (m : Money) (n : Nat) : Money := ⟨m.cents * ↑n⟩

-- Theorem 1: money_add_assoc
theorem money_add_assoc (a b c : Money) : (a + b) + c = a + (b + c) := by
  apply Money.ext
  show a.cents + b.cents + c.cents = a.cents + (b.cents + c.cents)
  omega

-- Theorem 2: money_add_comm
theorem money_add_comm (a b : Money) : a + b = b + a := by
  apply Money.ext
  show a.cents + b.cents = b.cents + a.cents
  omega

-- Theorem 3: money_sub_self
theorem money_sub_self (a : Money) : a.sub a = Money.zero := by
  apply Money.ext
  show a.cents - a.cents = 0
  omega

-- Theorem 4: money_mul_distrib
theorem money_mul_distrib (a b : Money) (n : Nat) :
    (a + b).mul n = a.mul n + b.mul n := by
  apply Money.ext
  show (a.cents + b.cents) * ↑n = a.cents * ↑n + b.cents * ↑n
  rw [Int.add_mul]

-- Theorem: Money from Nat is non-negative
theorem money_fromNat_non_negative (n : Nat) : (Money.fromNat n).cents ≥ 0 := by
  show (n : Int) ≥ 0
  omega

-- Theorem: Money from Int with proof is non-negative
theorem money_fromInt_non_negative (n : Int) (h : n ≥ 0) :
    (Money.fromInt n h).cents ≥ 0 := by
  show n ≥ 0
  exact h

-- Theorem: Zero is identity for addition (left)
theorem money_add_zero_left (m : Money) : Money.zero + m = m := by
  apply Money.ext
  show 0 + m.cents = m.cents
  omega

-- Theorem: Zero is identity for addition (right)
theorem money_add_zero_right (m : Money) : m + Money.zero = m := by
  apply Money.ext
  show m.cents + 0 = m.cents
  omega

-- Theorem: Multiplication by zero gives zero
theorem money_mul_zero (m : Money) : m.mul 0 = Money.zero := by
  apply Money.ext
  show m.cents * ↑0 = 0
  omega

-- Theorem: Multiplication by one gives original money
theorem money_mul_one (m : Money) : m.mul 1 = m := by
  apply Money.ext
  show m.cents * ↑1 = m.cents
  omega

end Money
end Shop
