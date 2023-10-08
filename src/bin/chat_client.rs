use async_std::net::TcpStream;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1111").await?;
    stream.write_all(b"hello world").await?;
    
    let mut buf = vec![0; 256];
    let n = stream.read(&mut buf).await?;

    println!("{:?}", &buf[0..n]);

    Ok(())
}