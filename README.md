# peano-axioms

peano-axioms is a crate for type-level arithmetic based on an extension of the Peano axioms.

## Examples

```rust
use peano_axioms::{One, Two, Three, Six, Product, Difference};

assert_eq!(Product::<Two, Three>::VALUE, Six::VALUE);
assert_eq!(Difference::<Three, Two>::VALUE, One::VALUE);

// Fails to compile
// assert_eq!(Difference::<Two, Three>::VALUE, One::VALUE);
```
