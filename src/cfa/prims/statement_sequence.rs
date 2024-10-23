use super::*;


/// ```text
/// if (COND) {
///     BODY
/// }
/// EXIT
/// ```
pub struct CFAStatementSequence {
    pub entry : CFGNode,
    pub exit  : CFGNode
}


impl CFAStatementSequence {


    /// Find the first sequential statements primitive in a `ControlFlowGraph`, or `None` if none could be found.
    pub fn find_first(cfg : &ControlFlowGraph) -> Option<Self> {
        for entry in cfg.nodes() {

            // Verify that entry has one successor (exit).
            let Some(entry_succs) = cfg.succs(entry) else { continue };
            if (entry_succs.len() != 1) { continue; }

            let mut entry_succs = entry_succs.into_iter();
            let exit = entry_succs.next().unwrap();

            if (Self::is_valid(cfg, entry, exit)) {
                return Some(Self { entry : entry.clone(), exit : exit.clone() });
            }

        }
        None
    }


    fn is_valid(cfg : &ControlFlowGraph, entry : &CFGNode, exit : &CFGNode) -> bool {

        // Temporaries sanity check.
        if (cfg.temps().contains(entry.to_succ())) { return false; }
        if (cfg.temps().contains(exit.from_pred())) { return false; }

        // Dominator sanity check.
        if (! cfg.dominates(entry, exit)) { return false; }

        // Verify that entry has one successor (exit).
        let Some(entry_succs) = cfg.succs(entry) else { return false };
        if (entry_succs.len() != 1) { return false; }
        if (! entry_succs.contains(exit)) { return false };

        true
    }


    /// Handles the special case where the node is directly at the end of a loop.
    /// An additional temporary node will be added if needed.
    /// 
    /// ```text
    /// while (EXIT) {     <- The exit is here, which makes analysis hard.
    ///     ENTRY
    ///     TEMPORARY     <- This temporary node is added to make the analysis process easier.
    /// }
    /// ```
    pub(crate) fn insert_needed_node(&mut self, cfg : &mut ControlFlowGraph) -> () {
        // If exit does not have one predecessor (entry), insert a temporary node.
        if (cfg.preds(&self.exit).map(|preds| preds.len()).unwrap_or(0) != 1) {
            let temporary = cfg.create_temporary_node();
            cfg.insert_node(&temporary, &self.entry, &self.exit);
            self.exit = (&temporary).into();
        }
    }


}


impl fmt::Display for CFAStatementSequence {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.entry)?;
        write!(f, " \x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.exit)?;
        Ok(())
    }
}
