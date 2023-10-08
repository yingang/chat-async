use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpListener;
use async_std::task;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1111").await?;
    
    loop {
        let (mut stream, _addr) = listener.accept().await?;
        task::spawn(async move {
            let mut buf = vec![0; 256];
            while let Ok(n) = stream.read(&mut buf).await {
                if n == 0 {
                    break;
                }
                println!("{:?}", &buf[0..n]);
                if let Err(_) = stream.write(&buf[0..n]).await {
                    break;
                }
                buf.clear();
            }
        });
    }
}