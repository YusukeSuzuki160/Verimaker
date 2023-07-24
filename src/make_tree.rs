use std::collections::HashMap;
use ptree::TreeBuilder;
use ptree::output::write_tree;


pub fn print_dependence_tree(graph: HashMap<String, Vec<String>>, top: String) { // Print dependence tree
    let mut tree = TreeBuilder::new(top.clone());
    for neighbor in &graph[&top] {
        add_tree(&mut tree, neighbor, &graph);
    }
    let tree = tree.build();
    let output = "verilog.tree";
    let output = std::fs::File::create(output).unwrap();
    write_tree(&tree, output);
}

fn add_tree(tree: &mut TreeBuilder, node: &String, graph: &HashMap<String, Vec<String>>) { // Add node to tree
    tree.begin_child(node.clone());
    for neighbor in &graph[node] {
        add_tree(tree, neighbor, graph);
    }
    tree.end_child();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_tree() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["D".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec![]);

        print_dependence_tree(graph);
    }
}