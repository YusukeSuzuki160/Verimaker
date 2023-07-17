use std::collections::HashMap;
use ptree::TreeBuilder;
use ptree::output::print_tree;


pub fn print_dependence_tree(graph: HashMap<String, Vec<String>>) {
    let top = graph.keys().next().unwrap();
    let mut tree = TreeBuilder::new(top.clone());
    for neighbor in &graph[top] {
        add_tree(&mut tree, neighbor, &graph);
    }
    let tree = tree.build();
    print_tree(&tree);
}

fn add_tree(tree: &mut TreeBuilder, node: &String, graph: &HashMap<String, Vec<String>>) {
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