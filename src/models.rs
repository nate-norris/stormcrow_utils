pub const SOP: u8 = 0xAA; // start of packet

enum DecodeState {
    WaitSOP,
    PacketType,
    Length,
    Payload,
    Checksum
}

pub struct DecodedPacket {
    packet_type: u8,
    payload: Vec<u8>,
}