import Lake
import Lake.Git

open Lake DSL

package shop_proof_math {
  name := "shop-proof-math"
  leanOptions := #[
    ⟨`pp.unsafe, true⟩,
    ⟨`autoImplicit, false⟩,
    ⟨`relaxedAutoImplicit, false⟩
  ]
  globs := #[.submodules `Shop]
}

lean_lib ShopProofMath {
  name := "ShopProofMath"
  roots := #[.submodules `Shop]
}

@[default_target]
lean_exe proof_checker {
  name := "proof-checker"
  root := `Main
}
