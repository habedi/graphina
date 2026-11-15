module

import GraphinaVerification

-- These examples check the imported graph vocabulary, not the Rust algorithms.
example {V : Type} (g : SimpleGraph V) (v : V) : g.Reachable v v :=
  SimpleGraph.Reachable.refl v

example {V : Type} (g : SimpleGraph V) (u v w : V)
    (huv : g.Reachable u v) (hvw : g.Reachable v w) : g.Reachable u w :=
  huv.trans hvw
