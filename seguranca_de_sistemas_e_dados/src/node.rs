use sha256::digest;
use rand::{RngCore};
use super::ID_SIZE;
pub struct Node{
    pub id : String,
    pub ip : String,
    pub port: u16,
}

impl Node{
    pub fn new(ip: String, port: u16) -> Self{
        let mut input = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut input);
        let mut input = digest(&input);
        Node { id: input[..ID_SIZE].to_string(), ip, port}
    }
    pub fn get_info(&self) -> String {
        format!("{}:{}:{}", self.ip, self.port, self.id.clone() )
    }
    pub fn distance(&self, other: Node) -> [u8; ID_SIZE] {
        let mut res = [0u8; ID_SIZE];
        let id_bytes= self.id.as_bytes();
        let other_bytes= other.id.as_bytes();
        for i in 0..ID_SIZE{
            res[i] = id_bytes[i] ^ other_bytes[i];
        }
        res
    }
}