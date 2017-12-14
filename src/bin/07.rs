use std::collections::HashSet;
use std::env::args;
use std::process::exit;

// Each node in the tree can be represented by its string value
// and the string values of its children
struct Node<'a> {
    value: &'a str,
    child_values: Vec<&'a str>
}

impl<'a> Node<'a> {

    // Each valid tree line has the following format:
    //   $value ($weight) -> $child_0, $child_1, ..., $child_n
    // The '-> ...' sequence is omitted (including ->!) if the
    // node has no children. 
    fn from_str(line: &'a str) -> Node<'a> {
        Node {
            // Every valid node has a value (i.e. no error-check here)
            value: line.split_whitespace().next().unwrap(),

            // Split the line on the '>' from '->'. If this results in
            //   1. If this results in two sections, split the second
            //      (i.e. the children) on ',' to get a list of children
            //   2. If this doesn't result in two sections, there are no
            //      children
            child_values: {
                match line.split(|c| c == '>').skip(1).next() {
                    Some(child_values) => {
                        child_values.split(|c| c == ',')
                                .map(|s| s.trim())
                                .collect()
                    }
                    None => vec![]
                }
            }
        }
    }
}

/// Combines the value and child values of a node into a single vector.
/// For this AoC solution, there's no need to distinguish between the
/// value and child values.
fn flatten<'a>(mut node: Node<'a>) -> Vec<&'a str> {
    node.child_values.push(node.value);
    node.child_values
}

/// Get the root node from a tree (see AoC problem for tree format)
fn root_node(tree: String) -> String {

    let mut tree_nodes = HashSet::with_capacity(tree.lines().count());

    // Every node in the tree appears exactly twice (once as child + once
    // as parent) EXCEPT for the root node (only once as parent). Traverse
    // the tree, inserting all new nodes encountered (including children) and
    // deleting existing nodes when a duplicate is encountered. Since every
    // node except the root node has exactly one duplicate, the resulting 
    // HashSet should ONLY contain the root node   
    for line in tree.lines() {
        let line_nodes = flatten(Node::from_str(line));

        for node in line_nodes {
            if tree_nodes.contains(node) {
                tree_nodes.remove(node);
            }
            else {
                tree_nodes.insert(node);
            }
        }
    }
    String::from(*tree_nodes.iter().next().unwrap())
}

fn print_usage() {
    println!("Day 7: Recursive Circus");
    println!("Usage:");
    println!("07 <input>");
    println!("  <input> - Input tree (see AoC example).");
    println!("            Typically from file (e.g. \"$(cat inputfile)\"");
}

fn main() {
    if let (2, Some(input)) = (args().count(), args().nth(1)) {
        println!("{}", root_node(input));
    }
    else {
        print_usage();
        exit(-1);
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn node_no_child_values() {
        let node = Node::from_str("abcd (45)");
        assert_eq!(node.value, "abcd");
        assert!(node.child_values.is_empty());
        assert_eq!(flatten(node), vec!["abcd"]);
    }

    #[test]
    fn node_one_child() {
        let node = Node::from_str("abc (100) -> defg");
        assert_eq!(node.value, "abc");
        assert_eq!(node.child_values, vec!["defg"]);
    }

    #[test]
    fn node_multiple_child_values() {
        let node = Node::from_str("ab (1) -> cd, efg, hijk");
        assert_eq!(node.value, "ab");
        assert_eq!(node.child_values, vec!["cd", "efg", "hijk"]);
    }

    #[test]
    fn flatten_no_child_values() {
        let node = Node { value: "abcd", child_values: vec![] };
        assert_eq!(flatten(node), vec!["abcd"]);
    }

    #[test]
    fn flatten_with_child_values() {
        let node = Node { value: "ab", child_values: vec!["cd", "ef"] };
        assert_eq!(flatten(node), vec!["cd", "ef", "ab"]);
    }

    #[test]
    fn two_nodes() {
        // a -> b
        let nodes = "a (5) -> b\nb (3)";
        assert_eq!(root_node(String::from(nodes)), "a");
    }

    #[test]
    fn four_nodes() {
        // b -> (a,c)
        // c -> d
        let nodes = "b (5) -> a, c\na (1)\nd (2)\nc (4) -> d";
        assert_eq!(root_node(String::from(nodes)), "b");
    }

}