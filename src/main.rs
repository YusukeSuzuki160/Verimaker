use verimaker::gen_source_list;
use verimaker::makefile::Makefile;
use std::path::Path;
use std::fs;
fn main() {
    let mut path = fs::canonicalize(Path::new("./testcase/FFT").to_path_buf()).unwrap();
    let source_list = gen_source_list::gen_source_list(path.clone());
    let makefile = Makefile::new("iverilog -g2009", source_list, "a.out");
    let output = makefile.gen();
    path.push("Makefile");
    fs::write(path, output).expect("Unable to write file");
}
