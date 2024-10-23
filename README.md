[![https://crates.io/crates/decomp](https://img.shields.io/crates/v/decomp)](https://crates.io/crates/decomp)
[![https://docs.rs/decomp/](https://img.shields.io/docsrs/decomp)](https://docs.rs/decomp/)
[![https://unlicense.org](https://img.shields.io/crates/l/decomp)](https://unlicense.org)

Components of a decompilation pipeline.

This library is a Rust reimplementation of [The decomp project](https://github.com/decomp/decomp/tree/master).

The original decomp project is licensed under The Unlicense. In an effort to keep this resource accessible,
  this library is also licensed under The Unlicense.

### Getting Started:
Add `decomp` as a dependency to your `Cargo.toml`.
Make sure to replace the version and llvm version with valid versions.
```toml
[dependencies]
decomp = { version = "X.X.X", features = [ "llvm-X" ] }
```

### Load an LLVM Module:
```rust
use decomp::prelude::*;
// From a textual LLVM IR file.
let module = Module::from_ir_path("/path/to/file.ll").unwrap();
// From a bitcode LLVM IR file.
let module = Module::from_bc_path("/path/to/file.bc").unwrap();
```

### Generate a Control Flow Graph:
See [`cfg`](https://docs.rs/decomp/latest/decomp/cfg/index.html).
```rust
use decomp::prelude::*;
for function in &module.functions {
    let cfg = ControlFlowGraph::new(function);
    println!("{}", cfg);
}
```

### Analyse the Control Flow Graph:
See [`cfa`](https://docs.rs/decomp/latest/decomp/cfa/index.html).
```rust
use decomp::prelude::*;
for function in &module.functions {
    let cfg   = ControlFlowGraph::new(function);
    let prims = CFAPrim::find_all(cfg).unwrap();
    println!("{}", prims);
}
```

### Recover Control Flow Groups:
See [`cfr`](https://docs.rs/decomp/latest/decomp/cfr/index.html).
```rust
use decomp::prelude::*;
for function in &module.functions {
    let cfg    = ControlFlowGraph::new(function);
    let prims  = CFAPrim::find_all(cfg).unwrap();
    let groups = CFRGroups::new(&prims).unwrap();
    println!("{}", groups);
}
```
