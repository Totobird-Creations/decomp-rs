//! Control Flow Graph
//! 
//! Data structures for control flow graphs of an LLVM `Function`.
//! 
//! Outputs a structure like:
//! ```text
//! %start
//! ↘_ %bb1
//! 
//! ↙‾ %start %bb6 %bb5
//! %bb1
//! ↘_ %bb2 %bb7
//! 
//! ↙‾ %bb1
//! %bb2
//! ↘_ %bb3 %bb4
//! 
//! ↙‾ %bb1
//! %bb7
//! ↘_ %bb13 %bb8
//! 
//! ↙‾ %bb7 %bb10
//! %bb13
//! 
//! ↙‾ %bb7 %bb8
//! %bb8
//! ↘_ %bb10 %bb8
//! 
//! ↙‾ %bb2
//! %bb3
//! ↘_ %bb4
//! 
//! ↙‾ %bb2 %bb3
//! %bb4
//! ↘_ %bb5 %bb6
//! 
//! ↙‾ %bb8
//! %bb10
//! ↘_ %bb13
//! 
//! ↙‾ %bb4
//! %bb5
//! ↘_ %bb1
//! 
//! ↙‾ %bb4
//! %bb6
//! ↘_ %bb1
//! ```


mod node;
pub use node::CFGNode;

use crate::util::unique_vec::UniqueVec;

use std::fmt;
use std::collections::HashMap;
use llvm_ir::{
    Function,
    Name,
    Terminator
};


/// A graph representing the flow of control in an LLVM `Function`.
#[derive(Clone)]
pub struct ControlFlowGraph {
    /// The entry node in the graph.
    entry     : CFGNode,
    /// All of the nodes in the graph.
    nodes     : UniqueVec<CFGNode>,
    /// Predecessors of nodes in the graph.
    preds     : HashMap<CFGNode, UniqueVec<CFGNode>>,
    /// Successors of nodes in the graph.
    succs     : HashMap<CFGNode, UniqueVec<CFGNode>>,
    /// Temporary inserted nodes.
    temps     : UniqueVec<Name>,
    next_temp : u128
}


impl ControlFlowGraph {


    /// Create a control flow graph of an LLVM `Function`.
    pub fn new(function : &Function) -> Self {
        let mut cfg = ControlFlowGraph {
            entry     : (&function.basic_blocks[0].name).into(),
            nodes     : UniqueVec::new(),
            preds     : HashMap::new(),
            succs     : HashMap::new(),
            temps     : UniqueVec::new(),
            next_temp : 0
        };

        for block in &function.basic_blocks { match (&block.term) {

            Terminator::Br(term) => {
                cfg.add_edge(&block.name, &term.dest);
            },

            Terminator::CondBr(term) => {
                cfg.add_edge(&block.name, &term.true_dest);
                cfg.add_edge(&block.name, &term.false_dest);
            },

            Terminator::Switch(term) => {
                for (_, dest) in &term.dests {
                    cfg.add_edge(&block.name, dest);
                }
                cfg.add_edge(&block.name, &term.default_dest);
            },

            Terminator::IndirectBr(term) => {
                for dest in &term.possible_dests {
                    cfg.add_edge(&block.name, dest);
                }
            },

            Terminator::Ret(_) | Terminator::Unreachable(_) => { },

            term @ Terminator::Invoke      (_) |
            term @ Terminator::Resume      (_) |
            term @ Terminator::CleanupRet  (_) |
            term @ Terminator::CatchRet    (_) |
            term @ Terminator::CatchSwitch (_) |
            term @ Terminator::CallBr      (_)
                => { panic!("Unsupported terminator: {}", term) }

        } }

        cfg
    }


    /// Gets the entry node.
    pub fn entry(&self) -> &CFGNode { &self.entry }

    pub(crate) fn set_entry<N : Into<CFGNode>>(&mut self, node : N) -> () { self.entry = node.into(); }

    /// Get all nodes available.
    pub fn nodes(&self) -> &UniqueVec<CFGNode> { &self.nodes }

    /// Get all nodes preceeding the given node.
    pub fn preds<N : Into<CFGNode>>(&self, node : N) -> Option<&UniqueVec<CFGNode>> { self.preds.get(&node.into()) }

    /// Get all nodes succeeding the given node.
    pub fn succs<N : Into<CFGNode>>(&self, node : N) -> Option<&UniqueVec<CFGNode>> { self.succs.get(&node.into()) }

    /// Get all temporary nodes.
    pub fn temps(&self) -> &UniqueVec<Name> { &self.temps }


    /// Creates a unidirectional connection between two nodes.
    /// 
    /// This will create the nodes if they do not already exist.
    pub fn add_edge<F : Into<CFGNode>, T : Into<CFGNode>>(&mut self, from : F, to : T) -> () {
        let from = from.into();
        let to = to.into();
        self.preds.entry(to.clone()).or_insert_with(|| UniqueVec::new()).insert(from.clone());
        self.succs.entry(from.clone()).or_insert_with(|| UniqueVec::new()).insert(to.clone());
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());
    }

    /// Removes a node, along with all connections to or from it.
    pub fn remove_node<N : Into<CFGNode>>(&mut self, node : N) -> () {
        let node = node.into();
        self.nodes.remove(&node);
        self.preds.remove(&node);
        self.succs.remove(&node);
        for (_, preds) in &mut self.preds {
            preds.remove(&node);
        }
        for (_, succs) in &mut self.succs {
            succs.remove(&node);
        }
    }

    /// Inserts the given node between `after` and `before`, destroying the previous connection if needed.
    /// 
    /// ```
    ///          A
    /// A        |
    /// |   ->   N
    /// B        |
    ///          B
    /// ```
    pub fn insert_node<N : Into<CFGNode>, A : Into<CFGNode>, B : Into<CFGNode>>(&mut self, node : N, after : A, before : B) -> () {
        let node = node.into();
        let after = after.into();
        let before = before.into();
        // Disconnect after and before.
        if let Some(succs) = self.succs.get_mut(&after) {
            succs.remove(&before);
        }
        if let Some(preds) = self.preds.get_mut(&before) {
            preds.remove(&after);
        }
        // Insert the new node.
        self.add_edge(&after, &node);
        self.add_edge(&node, &before);
    }

    /// Creates a temporary node which is treated as identical to `point_to`.
    /// 
    /// Used when collapsing the control flow graph down to the primitives.
    pub fn create_temporary_node(&mut self) -> Name {
        let mut name;
        // Find a temporary name that is not used.
        loop {
            name = Name::Name(Box::new(format!("@{}_TEMPORARY_{}", crate::MODULE_NAME.to_uppercase(), self.next_temp)));
            self.next_temp += 1;
            if (! self.nodes.contains(&(&name).into())) { break; }
        }
        self.temps.insert(name.clone());
        name
    }


    /// Returns `true` if every path from entry to `to`, must go through `to`.
    /// 
    /// By definition, a node will always dominate itself.
    /// 
    /// Relevant information: [Dominator (Graph theory)](https://en.wikipedia.org/wiki/Dominator_(graph_theory))
    pub fn dominates<H : Into<CFGNode>, O : Into<CFGNode>>(&self, through : H, to : O) -> bool {
        let through = through.into();
        let to = to.into();
        if (through == to) { return true; }
        self.dominates_inner(&self.entry, &through, &to, &mut Vec::new())
    }
    fn dominates_inner<'l>(&'l self, at : &'l CFGNode, through : &CFGNode, to : &CFGNode, already_checked : &mut Vec<&'l CFGNode>) -> bool {
        already_checked.push(at);
        let Some(succs) = self.succs.get(at) else { return true };
        for succ in succs {
            if (already_checked.contains(&succ)) { continue; }
            if (succ == through) { continue; }
            if (! self.dominates_inner(succ, through, to, already_checked)) {
                return false;
            }
        }
        true
    }


}


impl fmt::Display for ControlFlowGraph {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for node in &self.nodes {
            if (first) { first = false; }
            else { writeln!(f)?; }
            if let Some(preds) = self.preds.get(node) { if (preds.len() > 0) {
                write!(f, "  ↙‾")?;
                for pred in preds {
                    write!(f, " {}", pred)?;
                }
                writeln!(f)?;
            } }
            if (node == &self.entry) {
                writeln!(f, "  \x1b[92m\x1b[1m{}\x1b[0m", node)?;
            } else {
                writeln!(f, "  \x1b[97m\x1b[1m{}\x1b[0m", node)?;
            }
            if let Some(succs) = self.succs.get(node) { if (succs.len() > 0) {
                write!(f, "  ↘_")?;
                for succ in succs {
                    write!(f, " {}", succ)?;
                }
                writeln!(f)?;
            } }
        }
        Ok(())
    }
}
