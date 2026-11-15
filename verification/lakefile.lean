import Lake
open Lake DSL

package graphinaVerification where
  leanOptions := #[
    ⟨`autoImplicit, false⟩,
    ⟨`relaxedAutoImplicit, false⟩,
    ⟨`warningAsError, true⟩
  ]

require "leanprover-community" / "mathlib" @ "git#v4.35.0"

@[default_target]
lean_lib GraphinaVerification

@[test_driver]
lean_lib Tests
