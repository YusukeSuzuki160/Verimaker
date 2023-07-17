use std::collections::HashMap;

pub fn topological_sort(graph: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut sorted: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited: HashMap<String, bool> = HashMap::new();

    for (node, _) in &graph {
        visited.insert(node.to_string(), false);
    }

    for (node, _) in &graph {
        if !visited[node] {
            dfs(node, &graph, &mut visited, &mut sorted);
        }
    }

    sorted
}

fn dfs(
    node: &String,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashMap<String, bool>,
    sorted: &mut HashMap<String, Vec<String>>,
) {
    visited.insert(node.to_string(), true);

    for neighbor in &graph[node] {
        if !visited[neighbor] {
            dfs(neighbor, graph, visited, sorted);
        }
    }

    sorted.insert(node.to_string(), graph[node].clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["D".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec![]);

        let sorted = topological_sort(graph);
        assert_eq!(sorted, {
            let mut sorted: HashMap<String, Vec<String>> = HashMap::new();
            sorted.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
            sorted.insert("B".to_string(), vec!["D".to_string()]);
            sorted.insert("C".to_string(), vec!["D".to_string()]);
            sorted.insert("D".to_string(), vec![]);
            sorted
        });
    }
}