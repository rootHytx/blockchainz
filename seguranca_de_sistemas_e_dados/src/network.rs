use std::fmt::format;
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
        let socket = UdpSocket::bind(destination).unwrap();
        Self{
            socket,
            node,
        }
    }
}