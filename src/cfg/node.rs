use super::*;


/// A single node on a `ControlFlowGraph`
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct CFGNode {
    /// When nodes are merged during CFA, this is the name of the entry node of the primitive.
    from_pred : Name,
    /// When nodes are merged during CFA, this is the name of the exit node of the primitive.
    to_succ : Name
}


impl CFGNode {

    /// Create a new `CFGNode` with a `from_pred` and `to_succ` value.
    /// 
    /// Generally, this constructor is unneeded as `ControlFlowGraph` automatically creates
    /// them in its `new` constructor.
    pub fn new(from_pred : Name, to_succ : Name) -> Self { Self {
        from_pred,
        to_succ
    } }


    /// When nodes are merged during CFA, this is the name of the entry node of the primitive.
    pub fn from_pred(&self) -> &Name { &self.from_pred }

    /// When nodes are merged during CFA, this is the name of the exit node of the primitive.
    pub fn to_succ(&self) -> &Name { &self.to_succ }

}


impl Into<CFGNode> for &Name {
    fn into(self) -> CFGNode { CFGNode {
        from_pred : self.clone(),
        to_succ : self.clone()
    } }
}

impl Into<CFGNode> for &CFGNode {
    fn into(self) -> CFGNode { self.clone() }
}


impl fmt::Display for CFGNode {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_pred = self.from_pred();
        let to_succ   = self.to_succ();
        if (from_pred == to_succ) {
            write!(f, "{}", from_pred)
        } else {
            write!(f, "({}...{})", from_pred, to_succ)
        }
    }
}

