use crate::node::Node;
const ID_SIZE: usize = 20;
mod node;

pub fn main() {
    let node1 = Node::new("127.0.0.1".to_string(), 8080 );
    let node2 = Node::new("127.0.0.1".to_string(), 8080 );
    println!("{}", node1.get_info());
    println!("{:?}", node1.distance(node2));
}
