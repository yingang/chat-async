use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpListener;
use async_std::task;
use std::str;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1111").await?;
    
    loop {
        let (mut stream, addr) = listener.accept().await?;
        task::spawn(async move {    // thread::spawn是接收一个闭包，task::spawn是接收一个异步块
            let mut buf = vec![0; 256];
            while let Ok(n) = stream.read(&mut buf).await {
                if n == 0 {
                    // buf大小为零的时候，read也会直接返回0
                    break;
                }
                if let Ok(msg) = str::from_utf8(&buf[0..n]) {
                    println!("{}: {}", addr, msg.trim());
                    //println!("sending back...");
                    if let Err(_) = stream.write_all(&buf[0..n]).await {
                        break;
                    }
                }
                //println!("waiting...");
            }
            println!("bye {}!", addr);   // 加点输出，不然退出了都不知道 
        });
    }
}