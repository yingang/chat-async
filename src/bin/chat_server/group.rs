use async_std::{net::TcpStream, io::WriteExt};
use std::collections::HashMap;

pub struct GroupTable {
    map: HashMap<String, Group>,
}

impl GroupTable {
    pub fn new() -> Self {
        GroupTable { map: HashMap::new() }
    }

    pub fn join_group(&mut self, group_name: &str, addr: &str, stream: TcpStream) {
        let group = self.map.entry(group_name.to_string()).or_insert(Group::new());
        group.join(&addr, stream);
    }

    pub fn exit_group(&mut self, group_name: &str, addr: &str) {
        let mut remove_group = false;
        if self.map.contains_key(group_name) {
            let group = self.map.get_mut(group_name).unwrap();
            remove_group = group.exit(&addr);
        }
        if remove_group {
            self.map.remove(group_name);
        }
    }

    pub async fn post(&mut self, group_name: &str, msg: &str) -> std::io::Result<()> {
        if self.map.contains_key(group_name) {
            let group = self.map.get_mut(group_name).unwrap();
            group.post(msg).await?;
        } else {
            todo!() // 可以返回个error啥的给客户端
        }
        Ok(())
    }
}

pub struct Group {
    map: HashMap<String, TcpStream>,
}

impl Group {
    pub fn new() -> Self {
        Group { map: HashMap::new() }
    }

    pub fn join(&mut self, addr: &str, stream: TcpStream) {
        self.map.insert(addr.to_string(), stream);
    }

    /// return true if no entry left after the exiting, return false otherwise
    pub fn exit(&mut self, addr: &str) -> bool {
        self.map.remove_entry(&addr.to_string());
        match self.map.len() {
            0 => { true }
            _ => { false }
        }
    }

    pub async fn post(&mut self, msg: &str) -> std::io::Result<()> {
        for (addr, stream) in &mut self.map {
            stream.write_all(msg.as_bytes()).await?;
            println!("[{}] {}", &addr, &msg);
        }
        Ok(())
    }
}
