use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::IpAddr;

async fn check(ip: IpAddr, port: u16) -> Option<u16> {
    let addr = format!("{}:{}", ip, port);
    match timeout(Duration::from_millis(500), TcpStream::connect(addr)).await {
        Ok(Ok(_)) => Some(port),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let target: IpAddr = "127.0.0.1".parse().unwrap();
    let mut tasks = Vec::new();

    for port in 1..1024 {
        let task = tokio::spawn(check(target, port));
        tasks.push(task);
    }

    for task in tasks {
        if let Ok(Some(port)) = task.await {
            println!("Port {} is active", port);
        }
    }
}
