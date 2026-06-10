use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::stdout;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

async fn scan_port(ip: IpAddr, port: u16) -> Option<u16> {
    let addr = format!("{}:{}", ip, port);
    match timeout(Duration::from_millis(500), TcpStream::connect(addr)).await {
        Ok(Ok(_)) => Some(port),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target: IpAddr = "127.0.0.1".parse().unwrap();
    
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let scan_status = Arc::new(Mutex::new(String::from("Scanning network...")));

    let ports_clone = Arc::clone(&open_ports);
    let status_clone = Arc::clone(&scan_status);

    tokio::spawn(async move {
        let mut tasks = Vec::new();
        
        for port in 1..1024 {
            let p_clone = Arc::clone(&ports_clone);
            let task = tokio::spawn(async move {
                if let Some(p) = scan_port(target, port).await {
                    let mut lock = p_clone.lock().unwrap();
                    lock.push(p);
                    lock.sort();
                }
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }

        *status_clone.lock().unwrap() = String::from("Scan Complete. Press 'q' to exit.");
    });

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let ports = open_ports.lock().unwrap();
            let status = scan_status.lock().unwrap();

            let items: Vec<ListItem> = ports
                .iter()
                .map(|p| {
                    ListItem::new(format!("Port {} is active", p))
                        .style(Style::default().fg(Color::Green))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::NONE).title(status.clone()));

            f.render_widget(list, size);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
