use async_std::io::prelude::BufReadExt;
use async_std::io::BufReader;
use async_std::net::TcpListener;
use async_std::sync::Mutex;
use async_std::task;
use chat_async::ClientMsg;
use std::sync::Arc;

use crate::group::GroupTable;

mod group;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    // 要在异步块中可变使用，RefCell似乎都不行，只能上Mutex？
    // 但Mutex是加在这里好，还是加在GroupTable的定义里好？
    let global_group_table = Arc::new(Mutex::new(GroupTable::new()));

    let listener = TcpListener::bind("127.0.0.1:1111").await?;
    loop {
        let (mut stream, addr) = listener.accept().await?;
        let group_table = global_group_table.clone();
        task::spawn(async move {
            // thread::spawn是接收一个闭包，task::spawn/spawn_local是接收一个异步块，spawn_local需要启用unstable feature
            let mut reader = BufReader::new(stream.clone());
            let mut buf = String::new();
            while let Ok(n) = reader.read_line(&mut buf).await {
                if n == 0 {
                    break;
                }
                if let Ok(msg) = serde_json::from_str::<ClientMsg>(&buf) {
                    println!("received from {}: {:?}", &addr, &msg);
                    match &msg {
                        ClientMsg::JoinGroup { group_name } => {
                            let mut guard = group_table.lock().await;
                            guard.join_group(group_name, &addr, &mut stream);
                        }
                        ClientMsg::ExitGroup { group_name } => {
                            let mut guard = group_table.lock().await;
                            guard.exit_group(group_name, &addr, &mut stream);
                        }
                        ClientMsg::GroupMessage {
                            group_name,
                            message,
                        } => {
                            let mut guard = group_table.lock().await;
                            guard.post(group_name, &addr, &mut stream, message).await;
                        }
                    }
                }
                buf.clear();
            }
            println!("bye {}!", addr); // 加点输出，不然退出了都不知道
        });
    }
}
