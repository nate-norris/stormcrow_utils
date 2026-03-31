//! # lib_mm2t
//!
//! Provides an abstraction layer for interacting with the 900 MHz
//! serial radio in a Tokio async contexts.
//!
//! This library exposes:
//! - [`PacketT`]: the trait defining a packet to be sent as bytes over the MM2T port
//! - [`MM2TTransport`]: a real hardware implementation using a serial port.

// internal package imports
pub(crate) mod checksum;
pub(crate) mod models;
pub(crate) mod weather_payload;

// packets always included regardless of features
mod packet;
mod transport;

// optional mm2t-rx feature
#[cfg(feature = "mm2t-rx")]
pub mod packet_decoder;

// re-export commmon types and functions
pub use transport::MM2TTransport;
pub use packet::PacketT;
pub use weather_payload::WeatherPayload;
#[cfg(feature = "mm2t-rx")]
pub use packet_decoder::PacketDecoder;
#[cfg(feature = "mm2t-rx")]
pub use models::DecodedPacket;