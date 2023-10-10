use async_std::io::stdin;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use chat_async::ClientMsg;
use chat_async::ServerMsg;
use std::str;

fn parse_command(command: &str) -> Option<ClientMsg> {
    let words: Vec<&str> = command.split_whitespace().collect(); // 暂且不支持消息内容中带空格类字符
    match words[0].to_uppercase().as_str() {
        "JOIN" => {
            if words.len() < 2 {
                None
            } else {
                Some(ClientMsg::JoinGroup {
                    group_name: String::from(words[1]),
                })
            }
        }
        "EXIT" => {
            if words.len() < 2 {
                None
            } else {
                Some(ClientMsg::ExitGroup {
                    group_name: String::from(words[1]),
                })
            }
        }
        "POST" => {
            if words.len() < 3 {
                None
            } else {
                Some(ClientMsg::GroupMessage {
                    group_name: String::from(words[1]),
                    message: String::from(words[2]),
                })
            }
        }
        _ => None,
    }
}

fn parse_message(msg: &str) {
    if let Ok(msg) = serde_json::from_str::<ServerMsg>(msg) {
        match &msg {
            ServerMsg::GroupMessage {
                group_name,
                sender,
                message,
            } => {
                println!("received from {} at {}: {}", sender, &group_name, message);
            }
            ServerMsg::ErrorMessage { message } => {
                println!("ERROR: {}", &message);
            }
        }
    }
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1111").await?;

    let incoming = stream.clone();
    task::spawn_local(async move {
        let mut reader = BufReader::new(incoming);
        let mut buf = String::new();
        while let Ok(n) = reader.read_line(&mut buf).await {
            if n == 0 {
                break;
            }
            parse_message(&buf);
            buf.clear();
        }
        println!("bye!"); // 加点输出，不然退出了都不知道
    });

    let stdin = stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        if line.trim().len() == 0 {
            break;
        }

        if let Some(msg) = parse_command(&line) {
            let mut buf = serde_json::to_string(&msg).unwrap();
            buf.push('\n'); // 用换行来区分不同的消息，服务端对应处理
            stream.write_all(buf.as_bytes()).await?;
        }
    }

    Ok(())
}
