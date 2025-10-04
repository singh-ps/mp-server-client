use bincode::{Decode, Encode};

/// Will be used by both client and server
///
/// seq: packet sequence number
///
/// ack: last received sequence number from the other side, usually set by client for server to send the next snapshot
///
/// t_send_ns: timestamp when the packet was sent, used to calculate RTT
#[repr(C)]
#[derive(Clone, Copy, Debug, Decode, Encode)]
pub struct Header {
    pub seq: u32,
    pub ack: u32,
    pub t_send_ns: u64,
}
