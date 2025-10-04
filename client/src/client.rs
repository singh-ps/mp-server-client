use crate::wire::Header;
use bincode::{config, decode_from_slice, encode_to_vec};
use std::error::Error;
use tokio::net::UdpSocket;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("127.0.0.1:27015").await?;
    let mut buf = [0u8; 1200];

    let mut seq: u32 = 0;
    let mut large_rcv_seq: u32 = 0;

    let sending_header = Header {
        seq,
        ack: 0,
        t_send_ns: 0,
    };

    println!("Sending handshake to server: {:?}", sending_header);
    let encoded = encode_to_vec(&sending_header, config::standard())?;
    socket.send(&encoded).await?;
    seq = seq.wrapping_add(1);

    loop {
        println!("Waiting for server...");
        let (n, _) = socket.recv_from(&mut buf).await?;
        let header: Header = decode_from_slice(&buf[..n], config::standard())?.0;
        println!("Received from server: {:?}", header);

        if header.seq > large_rcv_seq {
            large_rcv_seq = header.seq;
        }

        let sending_header = Header {
            seq,
            ack: large_rcv_seq,
            t_send_ns: header.t_send_ns,
        };

        let encoded = encode_to_vec(&sending_header, config::standard())?;
        println!("Sending to server: {:?}", sending_header);
        socket.send(&encoded).await?;
        seq = seq.wrapping_add(1);
    }
}
