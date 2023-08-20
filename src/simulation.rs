use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    id: u32,
    ip_address: String,
}

struct Network {
    trusted_nodes: HashSet<Node>,
}

impl Network {
    fn add_trusted_node(&mut self, node: Node) {
        self.trusted_nodes.insert(node);
    }
}

fn main() {
    let mut network = Network {
        trusted_nodes: HashSet::new(),
    };
    let node1 = Node {
        id: 1,
        ip_address: "127.0.0.1".to_owned(),
    };
    network.add_trusted_node(node1);
    println!("{:?}", network);
}