import Lake
open Lake DSL

package shop where

@[default_target]
lean_lib Shop where
  roots := #[`Shop.Money, `Shop.Rate, `Shop.Tax,
             `Shop.Discount, `Shop.Coupon]
