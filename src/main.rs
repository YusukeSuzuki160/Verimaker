use verimaker::gen_source_list;
use verimaker::makefile::Makefile;
use std::path::Path;
use std::fs;
use std::env;
fn main() { // usage: cargo run [project path] [top module name]
    let args: Vec<String> = env::args().collect();
    let mut path = fs::canonicalize(Path::new(&args[1]).to_path_buf()).unwrap();
    let top = args[2].clone();
    let run_command = args[3].clone();
    let target = args[4].clone();
    let source_list = gen_source_list::gen_source_list(path.clone(), top);
    let makefile = Makefile::new(&run_command, source_list, &target);
    let output = makefile.gen();
    path.push("Makefile");
    fs::write(path, output).expect("Unable to write file");
}
