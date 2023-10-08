use async_std::io::stdin;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use chat_async::ClientMsg;
use std::str;

fn parse_command(command: &str) -> Option<ClientMsg> {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0].to_uppercase().as_str() {
        "JOIN" => {
            if words.len() < 2 {
                None
            } else {
                Some(ClientMsg::JoinGroup { group_name: String::from(words[1]) })
            }
        },
        "EXIT" => {
            if words.len() < 2 {
                None
            } else {
                Some(ClientMsg::ExitGroup { group_name: String::from(words[1]) })
            }            
        }
        "POST" => {
            if words.len() < 3 {
                None
            } else {
                Some(ClientMsg::GroupMessage { group_name: String::from(words[1]), message: String::from(words[2]) })
            }
        },
        _ => None
    }
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1111").await?;

    let mut incoming = stream.clone();
    task::spawn(async move {
        let mut buf = vec![0; 256];
        while let Ok(n) = incoming.read(&mut buf).await {
            if n == 0 {
                break;
            }
            if let Ok(msg) = str::from_utf8(&buf[0..n]) {
                println!("received: {}", msg.trim());
            }
        }
        println!("bye!");   // 加点输出，不然退出了都不知道 
    });

    let stdin = stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        if line.len() == 0 {
            break;
        }

        if let Some(json) = parse_command(&line) {
            stream.write_all(serde_json::to_string(&json).unwrap().as_bytes()).await?;
        }
    };

    Ok(())
}