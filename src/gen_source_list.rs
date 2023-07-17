use crate::gen_file_module_list::gen_file_module_list;
use crate::topological_sort::topological_sort;
use crate::make_tree::print_dependence_tree;
use std::path::PathBuf;
use std::collections::HashMap;
fn gen_module_list(dependense_graph: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut module_list = Vec::new();
    for (module_parent, module_children) in dependense_graph {
        for module_child in module_children {
            if !module_list.contains(&module_child) {
                module_list.push(module_child);
            }
        }
        if !module_list.contains(&module_parent) {
            module_list.push(module_parent);
        }
    }
    module_list
}
pub fn gen_source_list(path: PathBuf) -> Vec<String> {
    let (module_list, dependense_graph) = gen_file_module_list(path).unwrap();
    let dependense_graph = topological_sort(dependense_graph);
    print_dependence_tree(dependense_graph.clone());
    let use_module_list = gen_module_list(dependense_graph);
    let mut source_list = Vec::new();
    for module in use_module_list {
        for (file, modules) in &module_list {
            if modules.contains(&module) {
                source_list.push(file.to_string());
            }
        }
    }
    source_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_gen_source_list() {
        let path = Path::new("./testcase/").to_path_buf();
        let source_list = gen_source_list(path);
        assert_eq!(source_list, vec!["./testcase/test1.sv", "./testcase/test2.sv"]);
    }
}