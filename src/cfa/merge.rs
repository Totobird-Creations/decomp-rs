use super::*;


impl CFAPrim {


    /// Merges the nodes of this primitive into a single node on the control flow graph.
    /// 
    /// **This method may add temporary nodes to the graph to handle certain special cases.**
    /// These temporary nodes can be removed later in the decompilation process.
    pub fn merge(&mut self, cfg : &mut ControlFlowGraph) -> () {
        match (self) {
            CFAPrim::PreconditionLoop        (prim) => prim.insert_needed_node(cfg),
            CFAPrim::PostconditionLoop       (prim) => prim.insert_needed_node(cfg),
            CFAPrim::OnewayConditional       (prim) => prim.insert_needed_node(cfg),
            CFAPrim::OnewayReturnConditional (prim) => prim.insert_needed_node(cfg),
            CFAPrim::TwowayConditional       (prim) => prim.insert_needed_node(cfg),
            CFAPrim::StatementSequence       (prim) => prim.insert_needed_node(cfg),
        }

        let entry = self.entry();
        let exit  = self.exit();
        let nodes = self.nodes();

        let is_root_node = entry == cfg.entry();

        let entry_preds = cfg.preds(entry).map(|x| x.clone());
        let exit_succs = cfg.succs(exit).map(|x| x.clone());

        // Remove old nodes.
        for &node in &nodes {
            cfg.remove_node(node);
        }

        let new_node = CFGNode::new(entry.from_pred().clone(), exit.to_succ().clone());

        // Connect incoming edges.
        if let Some(entry_preds) = entry_preds {
            for entry_pred in &entry_preds {
                if (! nodes.contains(&entry_pred)) {
                    cfg.add_edge(entry_pred, &new_node);
                }
            }
        }

        // Connect outgoing edges.
        if let Some(exit_succs) = exit_succs {
            for exit_succ in &exit_succs {
                if (! nodes.contains(&exit_succ)) {
                    cfg.add_edge(&new_node, exit_succ);
                }
            }
        }

        if (is_root_node) {
            cfg.set_entry(new_node);
        }

    }


}
