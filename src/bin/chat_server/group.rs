use async_std::{io::WriteExt, net::TcpStream};
use chat_async::ServerMsg;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

pub struct GroupTable {
    map: HashMap<String, Group>,
}

impl GroupTable {
    pub fn new() -> Self {
        GroupTable {
            map: HashMap::new(),
        }
    }

    pub fn join_group(&mut self, group_name: &str, addr: &SocketAddr, stream: &mut TcpStream) {
        let group = self
            .map
            .entry(group_name.to_string())
            .or_insert(Group::new());
        group.join(&addr, &stream);
    }

    pub fn exit_group(&mut self, group_name: &str, addr: &SocketAddr, _stream: &mut TcpStream) {
        let mut remove_group = false;
        if self.map.contains_key(group_name) {
            let group = self.map.get_mut(group_name).unwrap();
            remove_group = group.exit(&addr);
        }
        if remove_group {
            self.map.remove(group_name);
        }
    }

    pub async fn post(
        &mut self,
        group_name: &str,
        sender_addr: &SocketAddr,
        sender_stream: &mut TcpStream,
        msg: &str,
    ) {
        if self.map.contains_key(group_name) {
            let group = self.map.get_mut(group_name).unwrap();
            let msg = ServerMsg::GroupMessage {
                group_name: group_name.to_string(),
                sender: sender_addr.to_string(),
                message: msg.to_string(),
            };
            let mut json = serde_json::to_string(&msg).unwrap();
            json.push('\n');
            group.post(&sender_addr, &json).await;
        } else {
            let msg = ServerMsg::ErrorMessage {
                message: "The specified group does not exist!".into(),
            };
            let mut json = serde_json::to_string(&msg).unwrap();
            json.push('\n');
            let _ = sender_stream.write_all(json.as_bytes()).await;
        }
    }
}

pub struct Group {
    map: HashMap<String, TcpStream>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            map: HashMap::new(),
        }
    }

    pub fn join(&mut self, addr: &SocketAddr, stream: &TcpStream) {
        self.map.insert(addr.to_string(), stream.clone());
    }

    /// return true if no entry left after the exiting, return false otherwise
    pub fn exit(&mut self, addr: &SocketAddr) -> bool {
        self.map.remove_entry(&addr.to_string());
        match self.map.len() {
            0 => true,
            _ => false,
        }
    }

    pub async fn post(&mut self, sender: &SocketAddr, msg: &str) {
        let mut bad_peer: HashSet<String> = HashSet::new();
        for (addr, stream) in &mut self.map {
            if sender.to_string() == *addr {
                continue;
            }
            if let Err(err) = stream.write_all(msg.as_bytes()).await {
                println!("failed to send to {}: {}", &addr, err);
                bad_peer.insert(addr.clone());
            } else {
                print!("sent to {}: {}", &addr, &msg);
            }
        }
        for peer in &bad_peer {
            self.map.remove(peer);
        }
    }
}
