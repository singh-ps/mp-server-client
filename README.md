# mp-server-client

Minimal Rust UDP client/server playground for experimenting with:

- Packet sequencing + simple ACK
- (Future) RTT / loss tracking
- 128 Hz server tick using Tokio

## Quick Run

```powershell
# Terminal 1
cargo run -p server

# Terminal 2
cargo run -p client
```

## Wire Header

```rust
struct Header {
		seq: u32,      // packet sequence (wrapping)
		ack: u32,      // highest seq seen from peer
		t_send_ns: u64 // timestamp (ns) when sender built packet
}
```

Client sends a handshake (seq 0). Server ticks at 128 Hz and sends packets to known peers; client echoes back with updated `ack`.

## Roadmap (short)

- Proper RTT calculation
- Packet loss detection & basic reliability
- Shared crate for the header
- Configurable tick rate / CLI options

## License

GNU GPL 2
