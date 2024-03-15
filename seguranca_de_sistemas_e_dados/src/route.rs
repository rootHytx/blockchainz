use super::{K_SIZE, N_BUCKETS, Node};

pub struct KBucket{
    pub nodes: Vec<Node>,
}

pub struct Routes{
    pub node: Node,
    pub list: Vec<KBucket>,
}

impl KBucket{
    pub fn new() -> Self{
        Self{
            nodes: Vec::new(),
        }
    }
}

impl Routes{
    pub fn new(cur:Node, bootstrap:Option<Node>) -> Self{
        let mut buckets : Vec<KBucket> = Vec::new();
        for i in 0..N_BUCKETS{
            buckets.push(KBucket::new());
        }
        Self{
            node:cur,
            list: buckets,
        }
    }
}