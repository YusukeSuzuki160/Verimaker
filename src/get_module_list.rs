use std::path::{Path, PathBuf};
use sv_parser::{parse_sv, unwrap_node, Locate, RefNode};
use std::collections::HashMap;

pub fn get_module(path: &Path) -> Result<(Vec<String>, HashMap<String, Vec<String>>), &'static str> {
    
    // The list of defined macros
    let defines = HashMap::new();
    // The list of include paths
    let includes: Vec<PathBuf> = Vec::new();

    // Parse
    let result = parse_sv(&path, &defines, &includes, false, false);
    let mut module_list: Vec<String> = Vec::new();
    let mut dependensies: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_module = String::new();

    if let Ok((syntax_tree, _)) = result {
        // &SyntaxTree is iterable
        for node in &syntax_tree {
            // The type of each node is RefNode
            match node {
                RefNode::ModuleDeclarationNonansi(x) => {
                    // unwrap_node! gets the nearest ModuleIdentifier from x
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();

                    let id = get_identifier(id).unwrap();

                    // Original string can be got by SyntaxTree::get_str(self, locate: &Locate)
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);
                    current_module = id.to_string();
                    module_list.push(id.to_string());
                }
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);
                    current_module = id.to_string();
                    module_list.push(id.to_string());
                }
                RefNode::ModuleInstantiation(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);
                    if dependensies.contains_key(&current_module) {
                        let dependensies_vec : &mut Vec<String> = dependensies.get_mut(&current_module).unwrap();
                        if dependensies_vec.contains(&id.to_string()) {
                            continue;
                        } else {
                            dependensies_vec.push(id.to_string());
                        }
                    } else {
                        dependensies.insert(current_module.to_string(), vec![id.to_string()]);
                    }
                }
                _ => (),
            }
        }
        return Ok((module_list, dependensies));
    } else {
        return Err("Parse failed");

    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_module() {
        let args = Path::new("./testcase/test2.sv");
        let (modue_list, dependensies) = get_module(args).unwrap();
        assert_eq!(modue_list, vec!["top".to_string()]);
        assert_eq!(dependensies, {
            let mut map = HashMap::new();
            map.insert("top".to_string(), vec!["FFT_unit".to_string(), "reorder".to_string()]);
            map
        });
    }
}