use std::fmt::format;
use std::io::Bytes;
use std::net::UdpSocket;
use std::sync::Arc;
use crate::node::Node;

pub struct RPC{
    pub socket: UdpSocket,
    pub node: Node,
}

impl RPC{
    pub fn new(mut node:Node) -> Self{
        let mut destination = format!("{}:{}", node.ip, node.port);
        //destination.push_str(":".to_string().push_str((node.port).to_string() as str));
        let socket = UdpSocket::bind(destination).expect("couldn't bind to address");
        Self{
            socket,
            node,
        }
    }
    pub fn send(mut n1: Node, mut n2:Node, data: Vec<u8>) {
        let mut attempts=1000;
        while true{
            if n1.clone().socket.unwrap().connect(&n2.clone().socket.unwrap().local_addr().unwrap()).is_err()==false{
                break;
            }
            attempts-=1;
        }
        println!("Successful Connection between Node {} and Node {}", n1.get_info(), n2.get_info());
    }
}