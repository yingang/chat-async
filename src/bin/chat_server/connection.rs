use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::sync::Mutex;

pub struct Outcoming(Mutex<TcpStream>);

impl Outcoming {
    pub fn new(stream: TcpStream) -> Self {
        Outcoming(Mutex::new(stream))
    }

    pub async fn send(&self, msg: &str) -> std::io::Result<()> {
        let mut guard = self.0.lock().await;
        guard.write_all(msg.as_bytes()).await
    }
}