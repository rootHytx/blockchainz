use std::net::UdpSocket;
use sha256::digest;
use rand::{RngCore};
use crate::network::RPC;
use super::ID_SIZE;
pub struct Node{
    pub id : String,
    pub ip : String,
    pub port: u16,
    pub socket: Option<UdpSocket>,
}

impl Node{
    pub fn init(ip: String, port: u16) -> Self {
        let mut input = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut input);
        let input = digest(&input);
        Node { id: input[..ID_SIZE].to_string(), ip, port, socket: None }
    }
    pub fn new(ip: String, port: u16) -> Self{
        let init=Self::init(ip, port);
        Node{
            id:init.id.clone(),
            ip:init.ip.clone(),
            port:init.port,
            socket: Option::from(RPC::new(init).socket),
        }
    }


    pub fn get_info(&self) -> String {
        format!("{}:{}:{}", self.ip, self.port, self.id.clone() )
    }
    pub fn distance(&self, other: Node) -> i128 {
        let res = i128::from_str_radix(&self.id, 16).unwrap() ^ i128::from_str_radix(&other.id, 16).unwrap();
        res
    }
}