use decomp::prelude::*;


fn main() {
    let module = Module::from_ir_path("/home/kyle/Code/rust/wasmdf/examples/hello_world/target/wasm32-unknown-unknown/release/deps/hello_world.ll").unwrap();
    for function in &module.functions {
        let cfg    = ControlFlowGraph::new(function);
        let prims  = CFAPrim::find_all(cfg).unwrap();
        let groups = CFRGroups::new(&prims).unwrap();
        println!();
        println!("----------");
        println!("FUNCTION {:?}", function.name);
        println!();
        println!("{}", groups);
        println!("----------");
        return;
    }
    println!();
}
