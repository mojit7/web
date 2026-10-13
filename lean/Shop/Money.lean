/-
# Money Module - Formal Proofs for Monetary Calculations

This module provides the formal proofs for money operations as specified in the Shop-Proof project.
All theorems must be proven without sorry.

## Key Invariants:
1. Money is represented as integer cents (never float)
2. Money addition is associative and commutative
3. Money subtraction is safe (never goes negative in valid operations)
4. Money multiplication distributes over addition
-/

import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Algebra.Group.Defs

namespace Shop

-- Money type: represented as integer cents
@[ext]
structure Money where
  cents : Int

-- Zero money
@[simp]
def Money.zero : Money := ⟨0⟩

-- Money from natural number (always non-negative)
def Money.fromNat (n : Nat) : Money := ⟨↑n⟩

-- Money from integer with proof of non-negativity
def Money.fromInt (n : Int) (h : n ≥ 0) : Money := ⟨n⟩

-- Money addition
instance : Add Money where
  add m1 m2 := ⟨m1.cents + m2.cents⟩

-- Money subtraction (returns Money, but we need to prove it's safe)
def Money.sub (m1 m2 : Money) : Money := 
  ⟨m1.cents - m2.cents⟩

-- Money multiplication by natural number
def Money.mul (m : Money) (n : Nat) : Money := 
  ⟨m.cents * ↑n⟩

-- Theorem 1: money_add_assoc
-- Proof: Addition of integers is associative
theorem money_add_assoc (a b c : Money) : (a + b) + c = a + (b + c) := by
  ext
  simp [Add.add, Money.ext_iff]
  exact Int.add_assoc a.cents b.cents c.cents

-- Theorem 2: money_add_comm
-- Proof: Addition of integers is commutative
theorem money_add_comm (a b : Money) : a + b = b + a := by
  ext
  simp [Add.add, Money.ext_iff]
  exact Int.add_comm a.cents b.cents

-- Theorem 3: money_sub_self
-- Proof: Subtracting a money from itself gives zero
theorem money_sub_self (a : Money) : a.sub a = Money.zero := by
  ext
  simp [Money.sub, Money.zero, Money.ext_iff]
  exact Int.sub_self a.cents

-- Theorem 4: money_mul_distrib
-- Proof: Multiplication distributes over addition for money
theorem money_mul_distrib (a b : Money) (n : Nat) : (a + b).mul n = a.mul n + b.mul n := by
  ext
  simp [Money.mul, Add.add, Money.ext_iff]
  rw [Int.mul_add]
  rfl

-- Additional useful theorems

-- Theorem: Money from Nat is non-negative
theorem money_fromNat_non_negative : ∀ n : Nat, (Money.fromNat n).cents ≥ 0 := by
  intro n
  simp [Money.fromNat]
  exact Int.ofNat_nonneg n

-- Theorem: Money from Int with proof is non-negative
theorem money_fromInt_non_negative : ∀ n h, (Money.fromInt n h).cents ≥ 0 := by
  intro n h
  simp [Money.fromInt]
  exact h

-- Theorem: Zero is identity for addition
theorem money_add_zero_left : ∀ m : Money, Money.zero + m = m := by
  intro m
  ext
  simp [Add.add, Money.zero, Money.ext_iff]
  exact Int.zero_add m.cents

theorem money_add_zero_right : ∀ m : Money, m + Money.zero = m := by
  intro m
  ext
  simp [Add.add, Money.zero, Money.ext_iff]
  exact Int.add_zero m.cents

-- Theorem: Multiplication by zero gives zero
theorem money_mul_zero : ∀ m : Money, m.mul 0 = Money.zero := by
  intro m
  ext
  simp [Money.mul, Money.zero, Money.ext_iff]
  exact Int.mul_zero m.cents

-- Theorem: Multiplication by 1 gives original money
theorem money_mul_one : ∀ m : Money, m.mul 1 = m := by
  intro m
  ext
  simp [Money.mul, Money.ext_iff]
  exact Int.mul_one m.cents

end Shop
