pub const SOP: u8 = 0x24; // start of packet $

#[cfg(feature = "mm2t-rx")]
pub(crate) enum DecodeState {
    WaitSOP,
    PacketType,
    Length,
    Payload,
    Checksum
}

#[cfg(feature = "mm2t-rx")]
pub struct DecodedPacket {
    pub packet_type: u8,
    pub payload: Vec<u8>,
}