//! [![https://crates.io/crates/decomp](https://img.shields.io/crates/v/decomp)](https://crates.io/crates/decomp)
//! [![https://docs.rs/decomp/](https://img.shields.io/docsrs/decomp)](https://docs.rs/decomp/)
//! [![https://unlicense.org](https://img.shields.io/crates/l/decomp)](https://unlicense.org)
//! 
//! Components of a decompilation pipeline.
//! 
//! This library is a Rust reimplementation of [The decomp project](https://github.com/decomp/decomp/tree/master).
//! 
//! The original decomp project is licensed under The Unlicense. In an effort to keep this resource accessible,
//!   this library is also licensed under The Unlicense.
//! 
//! ### Getting Started:
//! Add `decomp` as a dependency to your `Cargo.toml`.
//! Make sure to replace the version and llvm version with valid versions.
//! ```toml
//! [dependencies]
//! decomp = { version = "X.X.X", features = [ "llvm-X" ] }
//! ```
//! 
//! ### Load an LLVM Module:
//! ```rust
//! use decomp::prelude::*;
//! // From a textual LLVM IR file.
//! let module = Module::from_ir_path("/path/to/file.ll").unwrap();
//! // From a bitcode LLVM IR file.
//! let module = Module::from_bc_path("/path/to/file.bc").unwrap();
//! ```
//! 
//! ### Generate a Control Flow Graph:
//! See [`cfg`](mod@crate::cfg).
//! ```rust
//! use decomp::prelude::*;
//! for function in &module.functions {
//!     let cfg = ControlFlowGraph::new(function);
//!     println!("{}", cfg);
//! }
//! ```
//! 
//! ### Analyse the Control Flow Graph:
//! See [`cfa`](mod@crate::cfa).
//! ```rust
//! use decomp::prelude::*;
//! for function in &module.functions {
//!     let cfg   = ControlFlowGraph::new(function);
//!     let prims = CFAPrim::find_all(cfg).unwrap();
//!     println!("{}", prims);
//! }
//! ```
//! 
//! ### Recover Control Flow Groups:
//! See [`cfr`](mod@crate::cfr).
//! ```rust
//! use decomp::prelude::*;
//! for function in &module.functions {
//!     let cfg    = ControlFlowGraph::new(function);
//!     let prims  = CFAPrim::find_all(cfg).unwrap();
//!     let groups = CFRGroups::new(&prims).unwrap();
//!     println!("{}", groups);
//! }
//! ```
//! 


pub mod cfg;
pub mod cfa;
pub mod cfr;
pub mod util;


/// The essentials for restructuring LLVM IR.
pub mod prelude {

    pub use crate::cfg::{
        ControlFlowGraph,
        CFGNode
    };
    pub use crate::cfa::{
        CFAPrim,
        prims::*
    };
    pub use crate::cfr::CFRGroups;

    /// Re-export `llvm-ir`.
    pub use llvm_ir;
    /// Re-export from `llvm-ir`.
    pub use llvm_ir::{
        BasicBlock,
        Constant,
        ConstantRef,
        DebugLoc,
        HasDebugLoc,
        Function,
        Instruction,
        Module,
        Name,
        Operand,
        FPPredicate,
        IntPredicate,
        Terminator,
        Type,
        TypeRef
    };

}


/// Used when generating temporary control flow node names.
pub(crate) const MODULE_NAME : &'static str = module_path!();
