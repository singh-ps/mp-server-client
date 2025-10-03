use crate::wire::Header;
use bincode::{config, decode_from_slice, encode_to_vec};
use std::{collections::HashMap, error::Error, net::SocketAddr, time::SystemTime};
use tokio::{
    net::UdpSocket,
    time::{interval, Duration},
};

/// Keeps what has been sent to the peer and what has been acked.
///
/// Keeps the last calculated RTT. It will be eventually used for lag compensation.
#[derive(Default)]
struct PeerState {
    pub last_sent_seq: u32,
    pub last_ack: u32,
    pub last_rtt_ms: f32,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:27015").await?;
    let mut buf = [0u8; 1200];
    let mut tick = interval(Duration::from_millis(1000 / 128));

    let mut peers: HashMap<SocketAddr, PeerState> = HashMap::new();
    let mut tick_count = 0u32;

    loop {
        tokio::select! {
            _ = tick.tick() => {
                tick_count = tick_count.wrapping_add(1);
                for (addr, peer) in peers.iter_mut() {
                    // About to send a new packet to this peer
                    // Update the last sent sequence number
                    peer.last_sent_seq = tick_count;

                    let header = Header {
                        seq: peer.last_sent_seq,
                        ack: peer.last_ack,
                        t_send_ns: now_ns(),
                    };

                    let header_enc = encode_to_vec(header, config::standard())?;
                    let _ = socket.send_to(&header_enc, addr).await;
                }
            }
            Ok((n, addr)) = socket.recv_from(&mut buf) => {
                let header: Header = decode_from_slice(&buf[..n], config::standard())?.0;
                // check if this is a known peer
                match peers.get_mut(&addr) {
                    None => {
                        peers.insert(
                            addr,
                            PeerState {
                                last_sent_seq: header.seq,
                                last_ack: header.ack,
                                last_rtt_ms: 0.0,
                            },
                        );
                    }
                    Some(peer) => {
                        // TODO: here check if the peer acks the last sent seq
                        // That determines if there has been packet loss
                        // or reordering
                        peer.last_ack = header.ack;
                        // TODO: This is not accurate,
                        // need to compare the last sent seq with seq in header
                        // there may be packet loss or reordering
                        peer.last_rtt_ms = now_ns() as f32 - header.t_send_ns as f32;
                    }
                }
            }
        }
    }
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
