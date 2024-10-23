//! Control Flow Analysis
//! 
//! Utilities for finding primitive behaviours in a control flow graph.
//! 
//! Outputs a structure like:
//! ```text
//! -> loop { if (! %bb8 ) { break; } -> %bb10
//! -> if ( %bb2 ) { %bb3 } -> %bb4
//! -> if ( %bb7 ) { (%bb8...%bb10) } -> %bb13
//! -> if ( (%bb2...%bb4) ) { %bb5 } else { %bb6 } -> %@DECOMP_TEMPORARY_0
//! -> while ( %bb1 ) { (%bb2...%@DECOMP_TEMPORARY_0) } -> (%bb7...%bb13)
//! -> %start -> (%bb1...%bb13)
//! ```


pub mod prims;
use prims::*;

mod merge;

use crate::cfg::{
    ControlFlowGraph,
    CFGNode
};

use crate::util::unique_vec::UniqueVec;

use std::fmt;

use llvm_ir::Name;


/// A group of control flow graph nodes which perform a small task.
/// 
/// For more information, see documentation for individual variants.
pub enum CFAPrim {
    PreconditionLoop        (CFAPreconditionLoop        ),
    PostconditionLoop       (CFAPostconditionLoop       ),
    OnewayConditional       (CFAOnewayConditional       ),
    OnewayReturnConditional (CFAOnewayReturnConditional ),
    TwowayConditional       (CFATwowayConditional       ),
    StatementSequence       (CFAStatementSequence       )
}


impl CFAPrim {


    /// Finds all of the primitives in a `ControlFlowGraph`, or `None` if it failed to reduce the graph.
    pub fn find_all(mut cfg : ControlFlowGraph) -> Option<CFAPrims> {
        let mut prims = Vec::new();
        while (cfg.nodes().len() > 1) {
            //println!("{}", cfg);
            let mut prim = CFAPrim::find_first(&mut cfg).unwrap();
            prim.merge(&mut cfg);
            prims.push(prim);
        }
        Some(CFAPrims {
            entry : cfg.entry().clone(),
            temps : cfg.temps().clone(),
            prims
        })
    }


    /// Find the first primitive in a `ControlFlowGraph`, or `None` if none could be found.
    pub fn find_first(cfg : &ControlFlowGraph) -> Option<Self> {

        if let Some(prim) = CFAPreconditionLoop::find_first(cfg) {
            return Some(CFAPrim::PreconditionLoop(prim));
        }

        if let Some(prim) = CFAPostconditionLoop::find_first(cfg) {
            return Some(CFAPrim::PostconditionLoop(prim));
        }

        if let Some(prim) = CFAOnewayConditional::find_first(cfg) {
            return Some(CFAPrim::OnewayConditional(prim));
        }

        if let Some(prim) = CFAOnewayReturnConditional::find_first(cfg) {
            return Some(CFAPrim::OnewayReturnConditional(prim));
        }

        if let Some(prim) = CFATwowayConditional::find_first(cfg) {
            return Some(CFAPrim::TwowayConditional(prim));
        }

        if let Some(prim) = CFAStatementSequence::find_first(cfg) {
            return Some(CFAPrim::StatementSequence(prim));
        }

        None

    }


    /// Get the entry node of the primitive.
    pub fn entry(&self) -> &CFGNode {
        match (self) {
            Self::PreconditionLoop        (CFAPreconditionLoop        { cond,  .. }) => cond,
            Self::PostconditionLoop       (CFAPostconditionLoop       { cond,  .. }) => cond,
            Self::OnewayConditional       (CFAOnewayConditional       { cond,  .. }) => cond,
            Self::OnewayReturnConditional (CFAOnewayReturnConditional { cond,  .. }) => cond,
            Self::TwowayConditional       (CFATwowayConditional       { cond,  .. }) => cond,
            Self::StatementSequence       (CFAStatementSequence       { entry, .. }) => entry
        }
    }


    /// Get the exit node of the primitive.
    pub fn exit(&self) -> &CFGNode {
        match (self) {
            Self::PreconditionLoop        (CFAPreconditionLoop        { exit, .. }) => exit,
            Self::PostconditionLoop       (CFAPostconditionLoop       { exit, .. }) => exit,
            Self::OnewayConditional       (CFAOnewayConditional       { exit, .. }) => exit,
            Self::OnewayReturnConditional (CFAOnewayReturnConditional { exit, .. }) => exit,
            Self::TwowayConditional       (CFATwowayConditional       { exit, .. }) => exit,
            Self::StatementSequence       (CFAStatementSequence       { exit, .. }) => exit
        }
    }


    /// Get all of the nodes in the primitive.
    pub fn nodes(&self) -> Vec<&CFGNode> {
        match (self) {
            Self::PreconditionLoop        (CFAPreconditionLoop        { cond,  body,           exit }) => vec![ cond,  body,          exit ],
            Self::PostconditionLoop       (CFAPostconditionLoop       { cond,                  exit }) => vec![ cond,                 exit ],
            Self::OnewayConditional       (CFAOnewayConditional       { cond,  body,           exit }) => vec![ cond, body,           exit ],
            Self::OnewayReturnConditional (CFAOnewayReturnConditional { cond,  body,           exit }) => vec![ cond,  body,          exit ],
            Self::TwowayConditional       (CFATwowayConditional       { cond,  body_a, body_b, exit }) => vec![ cond, body_a, body_b, exit ],
            Self::StatementSequence       (CFAStatementSequence       { entry,                 exit }) => vec![ entry,                exit ]
        }
    }


}


/// A collection of `CFAPrim`s.
pub struct CFAPrims {
    entry : CFGNode,
    temps : UniqueVec<Name>,
    prims : Vec<CFAPrim>
}

impl CFAPrims {

    /// Get the entry node of the CFG.
    pub fn entry(&self) -> &CFGNode { &self.entry }

    /// Gets all temporary nodes in the CFG.
    pub fn temps(&self) -> &UniqueVec<Name> { &self.temps }

    /// Gets all primitives that were found.
    pub fn prims(&self) -> &Vec<CFAPrim> { &self.prims }

}


impl fmt::Display for CFAPrim {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            Self::PreconditionLoop        (prim) => write!(f, "{}", prim)?,
            Self::PostconditionLoop       (prim) => write!(f, "{}", prim)?,
            Self::OnewayConditional       (prim) => write!(f, "{}", prim)?,
            Self::OnewayReturnConditional (prim) => write!(f, "{}", prim)?,
            Self::TwowayConditional       (prim) => write!(f, "{}", prim)?,
            Self::StatementSequence       (prim) => write!(f, "{}", prim)?
        }
        Ok(())
    }
}

impl fmt::Display for CFAPrims {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for prim in &self.prims {
            if (first) { first = false; }
            else { writeln!(f)?; }
            write!(f, "{}", prim)?;
        }
        Ok(())
    }
}
