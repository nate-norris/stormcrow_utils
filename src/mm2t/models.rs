pub const SOP: u8 = 0xAA; // start of packet

#[cfg(feature = "mm2t-rx")]
enum DecodeState {
    WaitSOP,
    PacketType,
    Length,
    Payload,
    Checksum
}

#[cfg(feature = "mm2t-rx")]
pub struct DecodedPacket {
    packet_type: u8,
    payload: Vec<u8>,
}