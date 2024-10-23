//! Control Flow Recovery
//! 
//! Utilities for converting control flow primitives back into grouped blocks.
//! 
//! Outputs a structure like:
//! ```
//! %start
//! while (
//!   %bb1
//! ) {
//!   if (
//!     %bb2
//!   ) {
//!     %bb3
//!   }
//!   if (
//!     %bb4
//!   ) {
//!     %bb5
//!   } else {
//!     %bb6
//!   }
//! }
//! if (
//!   %bb7
//! ) {
//!   loop { if (!
//!       %bb8
//!   ) { break; } }
//!   %bb10
//! }
//! %bb13
//! ```


use crate::cfg::CFGNode;
use crate::cfa::{
    CFAPrim,
    CFAPrims,
    prims::*
};

use std::fmt;

use llvm_ir::Name;


/// A sequence of recovered CFA instruction groups.
#[derive(Clone)]
pub struct CFRGroups {
    pub groups : Vec<CFRGroup>
}

/// A group of recovered CFA instructions.
#[derive(Clone)]
pub enum CFRGroup {
    Block(Name),

    PreconditionLoop {
        cond : CFRGroups,
        body : CFRGroups
    },

    PostconditionLoop {
        cond : CFRGroups
    },

    OnewayConditional {
        cond : CFRGroups,
        body : CFRGroups
    },

    OnewayReturnConditional {
        cond : CFRGroups,
        body : CFRGroups
    },

    TwowayConditional {
        cond       : CFRGroups,
        body_true  : CFRGroups,
        body_false : CFRGroups
    }

}


impl CFRGroups {


    /// Recover groups from a function and its CFA primitives.
    pub fn new(prims : &CFAPrims) -> Option<Self> {
        Some(Self::handle(prims, prims.entry())?)
    }


    fn handle(prims : &CFAPrims, at : &CFGNode) -> Option<CFRGroups> {
        for prim in prims.prims() {
            // Look for the subnode of at.
            let prim_entry = prim.entry();
            if (prim_entry.from_pred() == at.from_pred() && prim.exit().to_succ() == at.to_succ()) {
                match (prim) {

                    CFAPrim::PreconditionLoop(CFAPreconditionLoop { cond, body, exit }) => {
                        let cond = Self::handle(prims, cond)?;
                        let body = Self::handle(prims, body)?;
                        let exit = Self::handle(prims, exit)?;
                        let mut out = CFRGroups { groups : vec![ CFRGroup::PreconditionLoop { cond, body } ] };
                        out.groups.extend(exit.groups);
                        return Some(out);
                    },

                    CFAPrim::PostconditionLoop(CFAPostconditionLoop { cond, exit }) => {
                        let cond = Self::handle(prims, cond)?;
                        let exit = Self::handle(prims, exit)?;
                        let mut out = CFRGroups { groups : vec![ CFRGroup::PostconditionLoop { cond } ] };
                        out.groups.extend(exit.groups);
                        return Some(out);
                    },

                    CFAPrim::OnewayConditional(CFAOnewayConditional { cond, body, exit }) => {
                        let mut out  = Self::handle(prims, cond)?;
                        let     cond = CFRGroups { groups : vec![ out.groups.pop().unwrap() ] };
                        let     body = Self::handle(prims, body)?;
                        let     exit = Self::handle(prims, exit)?;
                        out.groups.push(CFRGroup::OnewayConditional { cond, body });
                        out.groups.extend(exit.groups);
                        return Some(out);
                    },

                    CFAPrim::OnewayReturnConditional(CFAOnewayReturnConditional { cond, body, exit }) => {
                        let mut out  = Self::handle(prims, cond)?;
                        let     cond = CFRGroups { groups : vec![ out.groups.pop().unwrap() ] };
                        let     body = Self::handle(prims, body)?;
                        let     exit = Self::handle(prims, exit)?;
                        out.groups.push(CFRGroup::OnewayReturnConditional { cond, body });
                        out.groups.extend(exit.groups);
                        return Some(out);
                    },

                    CFAPrim::TwowayConditional(CFATwowayConditional { cond, body_a, body_b, exit }) => {
                        let mut out    = Self::handle(prims, cond)?;
                        let     cond   = CFRGroups { groups : vec![ out.groups.pop().unwrap() ] };
                        let     body_a = Self::handle(prims, body_a)?;
                        let     body_b = Self::handle(prims, body_b)?;
                        let     exit   = Self::handle(prims, exit)?;
                        out.groups.push(
                            CFRGroup::TwowayConditional { cond, body_true : body_a, body_false : body_b });
                        out.groups.extend(exit.groups);
                        return Some(out);
                    },

                    CFAPrim::StatementSequence(CFAStatementSequence { entry, exit }) => {
                        let mut out = Self::handle(prims, entry)?;
                        out.groups.extend(Self::handle(prims, exit)?.groups);
                        return Some(out);
                    }

                }
            }
        }
        let at_from_pred = at.from_pred();
        if (at_from_pred == at.to_succ()) {
            let groups = if (! prims.temps().contains(at_from_pred)) {
                vec![ CFRGroup::Block(at_from_pred.clone()) ]
            } else {
                Vec::new()
            };
            Some(CFRGroups { groups })
        } else { None }
    }


}


impl fmt::Display for CFRGroups {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_inner(f, 0)
    }
}
impl CFRGroups {
    fn fmt_inner(&self, f : &mut fmt::Formatter<'_>, depth : usize) -> fmt::Result {
        for group in &self.groups {
            group.fmt_inner(f, depth)?;
        }
        Ok(())
    }
}

impl fmt::Display for CFRGroup {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_inner(f, 0)
    }
}
impl CFRGroup {
    fn fmt_inner(&self, f : &mut fmt::Formatter<'_>, depth : usize) -> fmt::Result {
        match (self) {

            Self::Block(name) => {
                writeln!(f, "{}\x1b[36m{}\x1b[0m", "  ".repeat(depth), name)?
            },

            Self::PreconditionLoop { cond, body } => {
                writeln!(f, "{}\x1b[95m\x1b[1mwhile\x1b[0m \x1b[37m\x1b[1m(\x1b[0m", "  ".repeat(depth))?;
                cond.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m) {{\x1b[0m", "  ".repeat(depth))?;
                body.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m}}\x1b[0m", "  ".repeat(depth))?;
            },

            Self::PostconditionLoop { cond } => {
                writeln!(f, "{}\x1b[95m\x1b[1mloop\x1b[0m \x1b[37m\x1b[1m{{\x1b[0m \x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[91m!\x1b[0m", "  ".repeat(depth))?;
                cond.fmt_inner(f, depth + 2)?;
                writeln!(f, "{}\x1b[37m\x1b[1m) {{\x1b[0m \x1b[95m\x1b[1mbreak\x1b[0m\x1b[2m;\x1b[0m \x1b[37m\x1b[1m}}\x1b[0m \x1b[37m\x1b[1m}}\x1b[0m", "  ".repeat(depth))?;
            },

            Self::OnewayConditional { cond, body } => {
                writeln!(f, "{}\x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[0m", "  ".repeat(depth))?;
                cond.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m) {{\x1b[0m", "  ".repeat(depth))?;
                body.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m}}\x1b[0m", "  ".repeat(depth))?;
            },

            Self::OnewayReturnConditional { cond, body } => {
                writeln!(f, "{}\x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[0m", "  ".repeat(depth))?;
                cond.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m) {{\x1b[0m", "  ".repeat(depth))?;
                body.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[35m\x1b[1mreturn\x1b[0m\x1b[2m;\x1b[0m", "  ".repeat(depth + 1))?;
                writeln!(f, "{}\x1b[37m\x1b[1m}}\x1b[0m", "  ".repeat(depth))?;
            },

            Self::TwowayConditional { cond, body_true, body_false } => {
                writeln!(f, "{}\x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[0m", "  ".repeat(depth))?;
                cond.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m) {{\x1b[0m", "  ".repeat(depth))?;
                body_true.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m}}\x1b[0m \x1b[95m\x1b[1melse\x1b[0m \x1b[37m\x1b[1m{{\x1b[0m", "  ".repeat(depth))?;
                body_false.fmt_inner(f, depth + 1)?;
                writeln!(f, "{}\x1b[37m\x1b[1m}}\x1b[0m", "  ".repeat(depth))?;
            }

        }
        Ok(())
    }
}
