# Vulnerability Scanner

A high-speed network utility designed to identify active entry points on a target system.

## How it works

This scanner uses Asynchronous I/O. Instead of scanning ports sequentially (one by one), the program initiates hundreds of connection attempts simultaneously. It waits for a response within 500ms; if it receives a successful TCP handshake, the port is marked as active.

## Capabilities

- **Port Discovery:** Rapidly identifies open services.
- **Concurrency:** Scales performance by utilizing multi-threaded tasks.
- **Timeouts:** Aggressively drops unresponsive connections to maintain scanning speed.
- **Async Scanning**: Uses `tokio` to scan ports 1-1024 concurrently without freezing the interface.
- **Shared State Architecture**: Utilizes `Arc<Mutex<T>>` to safely pass live network discovery data from background tasks to the main UI thread.
- **Borderless UI**: Built with `ratatui` and `crossterm` for an immersive, distraction-free environment.
- **Graceful Teardown**: Captures exit signals to cleanly restore the terminal to its standard state.

## Usage

1. Run `cargo run`.
2. The UI will instantly launch in an alternate screen buffer.
3. Discovered ports will populate in real-time as background tasks complete their TCP handshakes.
4. Press `q` to exit and return to your standard command prompt.

## Logic Summary

- Sequential vs. Async: Sequential scanning is slow because it waits for network latency. Async scanning effectively "fires and forgets" connection requests, allowing the OS to manage the queue.
- The TCP Handshake: An "active" port in this context means the machine is listening for a connection on that port, which is the first step an attacker or administrator takes to assess a surface area.

> [!IMPORTANT]
> ## Security Disclaimer
> This tool is for educational use only. Authorized testing requires explicit, written permission from the network owner. Unauthorized scanning is illegal and can lead to immediate blacklisting by firewalls or ISPs.
