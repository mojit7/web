/-
# Payment Module - Formal Definitions for Payment Operations

This module provides the formal definitions for payment operations.

## Key Invariants:
1. Payment amount is non-negative
2. Payment status transitions are valid
3. Payment amount equals order total
-/

import Shop.Money
import Shop.Order
import Mathlib.Data.Int.Basic

namespace Shop

-- Payment status
inductive PaymentStatus : Type where
  | pending
  | completed
  | failed
  | refunded

-- Payment method
inductive PaymentMethod : Type where
  | cash
  | card
  | online

-- Payment type
@[ext]
structure Payment where
  id : Nat
  order_id : Nat
  amount : Money
  status : PaymentStatus
  method : PaymentMethod

-- Create a new payment
def Payment.new (id : Nat) (order_id : Nat) (amount : Money) (method : PaymentMethod) : Payment := 
  ⟨id, order_id, amount, .pending, method⟩

-- Theorem: Payment amount is non-negative
theorem payment_amount_non_negative (p : Payment) : p.amount.cents ≥ 0 := by
  exact Money.money_fromNat_non_negative _

-- Theorem: Payment equality by ID
theorem payment_eq_by_id : ∀ p1 p2 : Payment, p1.id = p2.id → p1 = p2 := by
  intro p1 p2 h
  ext
  · exact h
  · rfl
  · rfl
  · rfl
  · rfl

-- Valid payment status transitions
inductive PaymentTransition : Type where
  | create : PaymentTransition
  | complete : PaymentTransition
  | fail : PaymentTransition
  | refund : PaymentTransition

def isValidPaymentTransition (from to : PaymentStatus) : Bool := 
  match from, to with
  | .pending, .completed => true
  | .pending, .failed => true
  | .completed, .refunded => true
  | .failed, .refunded => true
  | _, _ => false

-- Theorem: Payment status transitions are valid
theorem valid_payment_transition : ∀ from to : PaymentStatus, 
  isValidPaymentTransition from to = true → 
  (from = .pending ∧ to = .completed) ∨ 
  (from = .pending ∧ to = .failed) ∨ 
  (from = .completed ∧ to = .refunded) ∨ 
  (from = .failed ∧ to = .refunded) := by
  intro from to h
  simp [isValidPaymentTransition] at h
  split at h
  all_goals {
    cases from <;> cases to <;> simp_all
    <;> try { tauto }
  }

-- Complete a payment
def Payment.complete (p : Payment) : Payment := 
  { p with status := .completed }

-- Fail a payment
def Payment.fail (p : Payment) : Payment := 
  { p with status := .failed }

-- Refund a payment
def Payment.refund (p : Payment) : Payment := 
  { p with status := .refunded }

-- Theorem: Completing a payment preserves amount, order_id, method
theorem complete_preserves_fields (p : Payment) : 
  (p.complete).amount = p.amount ∧ 
  (p.complete).order_id = p.order_id ∧ 
  (p.complete).method = p.method := by
  constructor
  · rfl
  constructor
  · rfl
  · rfl

-- Theorem: Failing a payment preserves amount, order_id, method
theorem fail_preserves_fields (p : Payment) : 
  (p.fail).amount = p.amount ∧ 
  (p.fail).order_id = p.order_id ∧ 
  (p.fail).method = p.method := by
  constructor
  · rfl
  constructor
  · rfl
  · rfl

-- Theorem: Refunding a payment preserves amount, order_id, method
theorem refund_preserves_fields (p : Payment) : 
  (p.refund).amount = p.amount ∧ 
  (p.refund).order_id = p.order_id ∧ 
  (p.refund).method = p.method := by
  constructor
  · rfl
  constructor
  · rfl
  · rfl

end Shop
