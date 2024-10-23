use super::*;


/// ```text
/// loop {
///     if (! COND) { break; }
/// }
/// EXIT
/// ```
pub struct CFAPostconditionLoop {
    pub cond : CFGNode,
    pub exit : CFGNode
}


impl CFAPostconditionLoop {


    /// Find the first precondition loop primitive in a `ControlFlowGraph`, or `None` if none could be found.
    pub fn find_first(cfg : &ControlFlowGraph) -> Option<Self> {
        for cond in cfg.nodes() {

            // Verify that cond has two successors (cond and exit).
            let Some(cond_succs) = cfg.succs(cond) else { continue };
            if (cond_succs.len() != 2) { continue; }

            let mut cond_succs = cond_succs.into_iter();
            let a = cond_succs.next().unwrap();
            let b = cond_succs.next().unwrap();

            if (Self::is_valid(cfg, cond, a)) {
                return Some(Self { cond : cond.clone(), exit : a.clone() });
            }

            if (Self::is_valid(cfg, cond, b)) {
                return Some(Self { cond : cond.clone(), exit : b.clone() });
            }

        }
        None
    }


    fn is_valid(cfg : &ControlFlowGraph, cond : &CFGNode, exit : &CFGNode) -> bool {

        // Temporaries sanity check.
        if (cfg.temps().contains(cond.to_succ())) { return false; }
        if (cfg.temps().contains(exit.from_pred())) { return false; }

        // Dominator sanity check.
        if (! cfg.dominates(cond, exit)) { return false; }

        // Verify that cond has two successors (cond and exit).
        let Some(cond_succs) = cfg.succs(cond) else { return false };
        if (cond_succs.len() != 2) { return false; }
        if (! cond_succs.contains(cond)) { return false; }
        if (! cond_succs.contains(exit)) { return false; }

        true
    }


    /// Handles the special case where the node is directly at the end of a loop.
    /// An additional temporary node will be added if needed.
    /// 
    /// ```text
    /// while (EXIT) {     <- The exit is here, which makes analysis hard.
    ///     loop {
    ///         if (! COND) { break; }
    ///     }
    ///     TEMPORARY     <- This temporary node is added to make the analysis process easier.
    /// }
    /// ```
    pub(crate) fn insert_needed_node(&mut self, cfg : &mut ControlFlowGraph) -> () {
        // If exit does not have one predecessor (cond), insert a temporary node.
        if (cfg.preds(&self.exit).map(|preds| preds.len()).unwrap_or(0) != 1) {
            let temporary = cfg.create_temporary_node();
            cfg.insert_node(&temporary, &self.cond, &self.exit);
            self.exit = (&temporary).into();
        }
    }


}


impl fmt::Display for CFAPostconditionLoop {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[95m\x1b[1mloop\x1b[0m \x1b[37m\x1b[1m{{\x1b[0m \x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[91m!\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.cond)?;
        write!(f, " \x1b[37m\x1b[1m) {{\x1b[0m \x1b[95m\x1b[1mbreak\x1b[0m\x1b[2m;\x1b[0m")?;
        write!(f, " \x1b[37m\x1b[1m}}\x1b[0m ")?;
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.exit)?;
        Ok(())
    }
}
