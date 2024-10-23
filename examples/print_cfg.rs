use decomp::prelude::*;


fn main() {
    let module = Module::from_ir_path("/path/to/file.ll").unwrap();
    for function in &module.functions {
        let cfg = ControlFlowGraph::new(function);
        println!();
        println!("----------");
        println!("FUNCTION {:?}", function.name);
        println!();
        println!("{}", cfg);
        println!("----------");
    }
    println!();
}
