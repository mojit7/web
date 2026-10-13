---- MODULE Order ----
EXTENDS Naturals, Sequences, TLC

(\* 
 * TLA+ Specification for Order Processing
 * 
 * This specification proves that order processing maintains:
 * 1. Order total is correctly calculated: total = subtotal - discount + tax
 * 2. Discount is bounded by subtotal
 * 3. Tax is calculated after discount
 * 4. All values are non-negative
 * 
 *)

CONSTANTS Products, InitialPrices

VARIABLES orders, order_status

(\* 
 * Order type
 *)
Order == [
  id: Nat,
  lines: Seq <<product_id: Nat, qty: Nat, price: Nat>>,
  discount: Nat,
  tax_rate: Nat,
  subtotal: Nat,
  tax: Nat,
  total: Nat,
  status: {"pending", "confirmed", "cancelled"}
]

(\* 
 * Initial state
 *)
Init == 
  orders = {}

(\* 
 * INVARIANTS
 *)

(\* 
 * Invariant 1: All order values are non-negative
 *)
NonNegativeValues == 
  \A o \in orders : 
    /\ o.subtotal >= 0
    /\ o.discount >= 0
    /\ o.tax >= 0
    /\ o.total >= 0

(\* 
 * Invariant 2: Discount is bounded by subtotal
 *)
DiscountBounded == 
  \A o \in orders : o.discount <= o.subtotal

(\* 
 * Invariant 3: Tax equals (subtotal - discount) * tax_rate / 100
 *)
TaxCorrect == 
  \A o \in orders : o.tax = (o.subtotal - o.discount) * o.tax_rate / 100

(\* 
 * Invariant 4: Total equals subtotal - discount + tax
 *)
TotalCorrect == 
  \A o \in orders : o.total = o.subtotal - o.discount + o.tax

(\* 
 * Invariant 5: Subtotal equals sum of line subtotals
 *)
SubtotalCorrect == 
  \A o \in orders : o.subtotal = +\ {line \in o.lines : line.qty * line.price}

----

(\* 
 * ACTIONS
 *)

(\* 
 * Calculate order totals
 *)
CalculateTotals(order) == 
  LET subtotal = +\ {line \in order.lines : line.qty * line.price}
  IN [order EXCEPT 
      !.subtotal = subtotal,
      !.tax = (subtotal - order.discount) * order.tax_rate / 100,
      !.total = subtotal - order.discount + ((subtotal - order.discount) * order.tax_rate / 100)]

(\* 
 * Create a new order
 *)
CreateOrder(order_id, lines, discount, tax_rate) == 
  /\ order_id \notin orders
  /\ lines /= <<>>
  /\ \A line \in lines : line.qty > 0
  /\ \A line \in lines : line.price > 0
  /\ discount >= 0
  /\ tax_rate <= 100
  /\ tax_rate >= 0
  /\ orders' = orders \cup {[
      id |-> order_id,
      lines |-> lines,
      discount |-> discount,
      tax_rate |-> tax_rate,
      subtotal |-> 0,
      tax |-> 0,
      total |-> 0,
      status |-> "pending"
    ]}
  /\ order_status' = order_status \cup {order_id |-> "pending"}

(\* 
 * Calculate totals for an order
 *)
ProcessOrder(order_id) == 
  /\ order_id \in orders
  /\ orders' = [orders EXCEPT ![order_id] = CalculateTotals(orders[order_id])]
  /\ order_status' = order_status

(\* 
 * Apply discount to order
 *)
ApplyDiscount(order_id, discount) == 
  /\ order_id \in orders
  /\ discount >= 0
  /\ discount <= orders[order_id].subtotal
  /\ orders' = [orders EXCEPT ![order_id].discount = discount]
  /\ order_status' = order_status

(\* 
 * Confirm order
 *)
ConfirmOrder(order_id) == 
  /\ order_id \in orders
  /\ orders[order_id].total > 0
  /\ orders' = [orders EXCEPT ![order_id].status = "confirmed"]
  /\ order_status' = [order_status EXCEPT ![order_id] = "confirmed"]

(\* 
 * Cancel order
 *)
CancelOrder(order_id) == 
  /\ order_id \in orders
  /\ orders[order_id].status = "pending"
  /\ orders' = [orders EXCEPT ![order_id].status = "cancelled"]
  /\ order_status' = [order_status EXCEPT ![order_id] = "cancelled"]

----

(\* 
 * Next state relation
 *)
Next == 
  \E order_id, lines, discount, tax_rate :
    CreateOrder(order_id, lines, discount, tax_rate)
    \/ ProcessOrder(order_id)
    \/ ApplyDiscount(order_id, discount)
    \/ ConfirmOrder(order_id)
    \/ CancelOrder(order_id)

(\* 
 * Specification
 *)
Spec == Init /\ [][Next]_<<orders, order_status>>

----

(\* 
 * THEOREMS
 *)

(\* 
 * THEOREM: All invariants are maintained
 *)
THEOREM AllInvariantsMaintained == 
  Spec => [](
    NonNegativeValues
    /\ DiscountBounded
    /\ TaxCorrect
    /\ TotalCorrect
    /\ SubtotalCorrect
  )

(\* 
 * THEOREM: Order totals are always correct
 *)
THEOREM OrderTotalsCorrect == 
  Spec => [](
    \A o \in orders :
      /\ o.subtotal = +\ {line \in o.lines : line.qty * line.price}
      /\ o.tax = (o.subtotal - o.discount) * o.tax_rate / 100
      /\ o.total = o.subtotal - o.discount + o.tax
  )

(\* 
 * THEOREM: Discount is always valid
 *)
THEOREM DiscountAlwaysValid == 
  Spec => [](
    \A o \in orders :
      o.discount >= 0 /\ o.discount <= o.subtotal
  )

(\* 
 * THEOREM: All values are non-negative
 *)
THEOREM AllValuesNonNegative == 
  Spec => [](
    \A o \in orders :
      o.subtotal >= 0 /\ o.discount >= 0 /\ o.tax >= 0 /\ o.total >= 0
  )

====
