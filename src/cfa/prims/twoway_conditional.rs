use super::*;


/// ```text
/// if (COND) {
///     BODY_A
/// } else {
///     BODY_B
/// }
/// EXIT
/// ```
pub struct CFATwowayConditional {
    pub cond    : CFGNode,
    pub body_a  : CFGNode,
    pub body_b  : CFGNode,
    pub exit    : CFGNode
}


impl CFATwowayConditional {


    /// Find the first two-way conditional primitive in a `ControlFlowGraph`, or `None` if none could be found.
    pub fn find_first(cfg : &ControlFlowGraph) -> Option<Self> {
        for cond in cfg.nodes() {

            // Verify that cond has two successors (body_a and body_b).
            let Some(cond_succs) = cfg.succs(cond) else { continue };
            if (cond_succs.len() != 2) { continue; }
            let mut cond_succs = cond_succs.into_iter();

            let body_a = cond_succs.next().unwrap();
            let body_b = cond_succs.next().unwrap();

            // Verify that body_a has one successor (exit).
            let Some(body_a_succs) = cfg.succs(body_a) else { continue };
            if (body_a_succs.len() != 1) { continue; }

            let mut body_a_succs = body_a_succs.into_iter();
            let exit = body_a_succs.next().unwrap();

            if (Self::is_valid(cfg, cond, body_a, body_b, exit)) {
                return Some(Self { cond : cond.clone(), body_a : body_a.clone(), body_b : body_b.clone(), exit : exit.clone() });
            }

        }
        None
    }


    fn is_valid(cfg : &ControlFlowGraph, cond : &CFGNode, body_a : &CFGNode, body_b : &CFGNode, exit : &CFGNode) -> bool {

        // Temporaries sanity check.
        if (cfg.temps().contains(cond.to_succ())) { return false; }
        if (cfg.temps().contains(exit.from_pred())) { return false; }

        // Dominator sanity check.
        if (! cfg.dominates(cond, body_a)) { return false; }
        if (! cfg.dominates(cond, body_b)) { return false; }
        if (! cfg.dominates(cond, exit)) { return false; }

        // Verify that cond is dominated by its predecessors.
        if let Some(cond_preds) = cfg.preds(cond) {
            for cond_pred in cond_preds {
                if (! cfg.dominates(cond_pred, cond)) { return false; }
            }
        }

        // Verify that cond has two successors (body_a and body_b).
        let Some(cond_succs) = cfg.succs(cond) else { return false };
        if (cond_succs.len() != 2) { return false; }
        if (! cond_succs.contains(body_a)) { return false; }
        if (! cond_succs.contains(body_b)) { return false; }

        // Verify that body_a has one predecessor (cond).
        let Some(body_a_preds) = cfg.preds(body_a) else { return false };
        if (body_a_preds.len() != 1) { return false; }

        // Verify that body_a has one successor (exit).
        let Some(body_a_succs) = cfg.succs(body_a) else { return false };
        if (body_a_succs.len() != 1) { return false; }
        if (! body_a_succs.contains(exit)) { return false; }

        // Verify that body_b has one predecessor (cond).
        let Some(body_b_preds) = cfg.preds(body_b) else { return false };
        if (body_b_preds.len() != 1) { return false; }

        // Verify that body_b has one successor (exit).
        let Some(body_b_succs) = cfg.succs(body_b) else { return false };
        if (body_b_succs.len() != 1) { return false; }
        if (! body_b_succs.contains(exit)) { return false; }

        true
    }


    /// Handles the special case where the node is directly at the end of a loop.
    /// An additional temporary node will be added if needed.
    /// 
    /// ```text
    /// while (EXIT) {     <- The exit is here, which makes analysis hard.
    ///     if (COND) {
    ///         BODY_A
    ///     } else {
    ///         BODY_B
    ///     }
    ///     TEMPORARY     <- This temporary node is added to make the analysis process easier.
    /// }
    /// ```
    pub(crate) fn insert_needed_node(&mut self, cfg : &mut ControlFlowGraph) -> () {
        // If exit does not have two predecessors (body_a and body_b), insert a temporary node.
        if (cfg.preds(&self.exit).map(|preds| preds.len()).unwrap_or(0) != 2) {
            let temporary = cfg.create_temporary_node();
            cfg.insert_node(&temporary, &self.body_a, &self.exit);
            cfg.insert_node(&temporary, &self.body_b, &self.exit);
            self.exit = (&temporary).into();
        }
    }


}


impl fmt::Display for CFATwowayConditional {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[95m\x1b[1mif\x1b[0m \x1b[37m\x1b[1m(\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.cond)?;
        write!(f, " \x1b[37m\x1b[1m) {{\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.body_a)?;
        write!(f, " \x1b[37m\x1b[1m}}\x1b[0m \x1b[95m\x1b[1melse\x1b[0m \x1b[37m\x1b[1m{{\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.body_b)?;
        write!(f, " \x1b[37m\x1b[1m}}\x1b[0m ")?;
        write!(f, "\x1b[2m->\x1b[0m ")?;
        write!(f, "\x1b[36m{}\x1b[0m", self.exit)?;
        Ok(())
    }
}


