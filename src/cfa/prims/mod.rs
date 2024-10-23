//! Control flow analysis primitives.


mod precondition_loop;
pub use precondition_loop::CFAPreconditionLoop;

mod postcondition_loop;
pub use postcondition_loop::CFAPostconditionLoop;

mod oneway_conditional;
pub use oneway_conditional::CFAOnewayConditional;

mod oneway_return_conditional;
pub use oneway_return_conditional::CFAOnewayReturnConditional;

mod twoway_conditional;
pub use twoway_conditional::CFATwowayConditional;

mod statement_sequence;
pub use statement_sequence::CFAStatementSequence;


use super::*;
