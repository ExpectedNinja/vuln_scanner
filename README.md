# Vulnerability Scanner

A high-speed network utility designed to identify active entry points on a target system.

## How it works

This scanner uses Asynchronous I/O. Instead of scanning ports sequentially (one by one), the program initiates hundreds of connection attempts simultaneously. It waits for a response within 500ms; if it receives a successful TCP handshake, the port is marked as active.
Capabilities

- Port Discovery: Rapidly identifies open services.
- Concurrency: Scales performance by utilizing multi-threaded tasks.
- Timeouts: Aggressively drops unresponsive connections to maintain scanning speed.

## Security Disclaimer

This tool is for educational use only. Authorized testing requires explicit, written permission from the network owner. Unauthorized scanning is illegal and can lead to immediate blacklisting by firewalls or ISPs.

## Getting Started

- Run `cargo run`.
- The console will output all successfully connected ports in real-time.

## Logic Summary

- Sequential vs. Async: Sequential scanning is slow because it waits for network latency. Async scanning effectively "fires and forgets" connection requests, allowing the OS to manage the queue.
- The TCP Handshake: An "active" port in this context means the machine is listening for a connection on that port, which is the first step an attacker or administrator takes to assess a surface area.
