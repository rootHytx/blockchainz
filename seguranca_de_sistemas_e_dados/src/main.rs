use crate::node::Node;
use std::collections::HashMap;
use std::io::Read;
use crate::network::RPC;

const ID_SIZE: usize = 5;
const K_SIZE: usize = 2;
const N_BUCKETS: usize = K_SIZE*8;
mod node;
mod route;
mod network;

pub fn main() {
    let node1 = Node::new("127.0.0.1".to_string(), 5555 ); //bootstrap node
    let node2 = Node::new("127.0.0.1".to_string(), 8080 );
    println!("{}", node1.get_info());
    println!("{}", node2.get_info());
    //println!("{:?}", node1.distance(node2));
    let message = "Hello World!";
    println!("{:?}", message.as_bytes().to_vec());
    RPC::send(node1, node2, message.as_bytes().to_vec());

}