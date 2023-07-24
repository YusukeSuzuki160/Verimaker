use crate::gen_file_module_list::gen_file_module_list;
use crate::topological_sort::topological_sort;
use crate::make_tree::print_dependence_tree;
use std::path::PathBuf;
use std::collections::HashMap;
fn gen_module_list(dependense_graph: HashMap<String, Vec<String>>, top: String) -> Vec<String> { // Generate module list appearing in dependense_graph
    let mut module_list = Vec::new();
    for module in dependense_graph[&top].clone() {
        module_list.push(module.clone());
        for sub_module in gen_module_list(dependense_graph.clone(), module) {
            if module_list.contains(&sub_module) {
                continue;
            }
            module_list.push(sub_module);
        }
    }
    module_list.push(top);
    module_list
}
pub fn gen_source_list(path: PathBuf, top_module: String) -> Vec<String> { // Generate source file list using in the project under path
    let (module_list, dependense_graph) = gen_file_module_list(path).unwrap();
    let dependense_graph = topological_sort(dependense_graph);
    print_dependence_tree(dependense_graph.clone(), top_module.clone());
    let use_module_list = gen_module_list(dependense_graph, top_module);
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
        let top = "top".to_string();
        let source_list = gen_source_list(path, top);
        assert_eq!(source_list, vec!["./testcase/test1.sv", "./testcase/test2.sv"]);
    }
}