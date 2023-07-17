pub struct Makefile {
    run: &'static str,
    srcs: Vec<String>,
    target: &'static str,
}

impl Makefile {
    pub fn new(run: &'static str, srcs: Vec<String>, target: &'static str) -> Self {
        Self {
            run,
            srcs,
            target,
        }
    }
    pub fn gen(&self) -> String {
        let mut makefile = String::new();
        makefile.push_str(&format!("RUN = {}\n", self.run));
        makefile.push_str(&format!("SRCS = {}\n", self.srcs.join(" ")));
        makefile.push_str(&format!("TARGET = {}\n\n", self.target));
        makefile.push_str(&format!("$(TARGET): $(SRCS)\n\t$(RUN) $(SRCS)\n"));
        makefile.push_str(&format!("\n.PHONY: clean\n\nclean:\n\trm -f $(TARGET)\n"));
        makefile.push_str(&format!("\n.PHONY: clean_all\n\nclean_all:\n\trm -f $(TARGET)\n\tfind . -name \"*.vcd\" -delete\n"));  
        makefile.push_str(&format!("\n.PHONY: run\n\nrun:\n\t$(RUN) $(SRCS)\n"));
        makefile.push_str(&format!("\t./$(TARGET)\n"));
        makefile
    }
}

#[test]
fn test_makefile_gen() {
    let makefile = Makefile::new("run", vec!["src1".to_string(), "src2".to_string()], "target");
    assert_eq!(makefile.gen(), "RUN = run\nSRCS = src1 src2\nTARGET = target\n\n$(TARGET): $(SRCS)\n\t$(RUN) $(SRCS)\n");
}