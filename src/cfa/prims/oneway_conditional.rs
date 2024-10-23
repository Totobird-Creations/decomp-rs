use super::*;


/// ```text
/// PSEUDOCODE           GRAPH
/// 
/// if (COND) {           A
///     BODY             / |
/// }                    B |
/// EXIT                 \ |
///                       C
/// ```
pub struct CFAOnewayConditional {
    pub cond : CFGNode,
    pub body : CFGNode,
    pub exit : CFGNode
}


impl CFAOnewayConditional {


    /// Find the first one-way conditional primitive in a `ControlFlowGraph`, or `None` if none could be found.
    pub fn find_first(cfg : &ControlFlowGraph) -> Option<Self> {
        for cond in cfg.nodes() {

            // Verify that cond has two successors (body and end).
            let Some(cond_succs) = cfg.succs(cond) else { continue };
            if (cond_succs.len() != 2) { continue; }

            let mut cond_succs = cond_succs.into_iter();
            let a = cond_succs.next().unwrap();
            let b = cond_succs.next().unwrap();

            if (Self::is_valid(cfg, cond, a, b)) {
                return Some(Self { cond : cond.clone(), body : a.clone(), exit : b.clone() });
            }

            if (Self::is_valid(cfg, cond, b, a)) {
                return Some(Self { cond : cond.clone(), body : b.clone(), exit : a.clone() });
            }

        }
        None
    }


    fn is_valid(cfg : &ControlFlowGraph, cond : &CFGNode, body : &CFGNode, exit : &CFGNode) -> bool {

        // Temporaries sanity check.
        if (cfg.temps().contains(cond.to_succ())) { return false; }
        if (cfg.temps().contains(exit.from_pred())) { return false; }

        // Dominator sanity check.
        if (! cfg.dominates(cond, body)) { return false; }
        if (! cfg.dominates(cond, exit)) { return false; }

        // Verify that cond is dominated by its predecessors.
        if let Some(cond_preds) = cfg.preds(cond) {
            for cond_pred in cond_preds {
                if (! cfg.dominates(cond_pred, cond)) { return false; }
            }
        }

        // Verify that cond has two successors (body and exit).
        let Some(cond_succs) = cfg.succs(cond) else { return false };
        if (cond_succs.len() != 2) { return false; }
        if (! cond_succs.contains(body)) { return false; }
        if (! cond_succs.contains(exit)) { return false; }

        // Verify that body has one predecessor (cond).
        let Some(body_preds) = cfg.preds(body) else { return false };
        if (body_preds.len() != 1) { return false; }

        // Verify that body has one successor (exit).
        let Some(body_succs) = cfg.succs(body) else { return false };
        if (body_succs.len() != 1) { return false; }
        if (! body_succs.contains(exit)) { return false };

        true
    }


    /// Handles the special case where the node is directly at the end of a loop.
    /// An additional temporary node will be added if needed.
    /// 
    /// ```text
    /// while (EXIT) {     <- The exit is here, which makes analysis hard.
    ///     if (COND) {
    ///         BODY
    ///     }
    ///     TEMPORARY     <- This temporary node is added to make the analysis process easier.
    /// }
    /// ```
    pub(crate) fn insert_needed_node(&mut self, cfg : &mut ControlFlowGraph) -> () {
        // If exit does not have two predecessors (cond and body), insert a temporary node.
        if (cfg.preds(&self.exit).map(|preds| preds.len()).unwrap_or(0) != 2) {
            let temporary = cfg.create_temporary_node();
            cfg.insert_node(&temporary, &self.cond, &self.exit);
            cfg.insert_node(&temporary, &self.body, &self.exit);
            self.exit = (&temporary).into();
        }
    }


}


impl fmt::Display for CFAOnewayConditional {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.cond)?;
        write!(f, " \x1b[37m\x1b[1m) {{\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.body)?;
        write!(f, " \x1b[37m\x1b[1m}}\x1b[0m ")?;
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.exit)?;
        Ok(())
    }
}

