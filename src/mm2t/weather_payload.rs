//! Define common struct that will be sent by weather site and received
//! by base station. 
//! 
//! Structure revolves around Vec<u8> of payload bytes.
use serde::Serialize;

#[repr(C)]
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct WeatherPayload {
    pub site_id: u8,
    pub altitude: f32,
    pub wind_full: f32,
    pub wind_dir: f32,
    pub temp: f32,
    pub humidity: f32,
    pub baro: f32,
}

impl WeatherPayload {
    const SIZE: usize = 25; // buffer allocation size

    /// Provide a byte vector given a WeatherPayload struct is set on Self
    ///
    /// Returns an error if the tone or pattern playback fails.
    /// - `Vec<u8>` properly structured payload
    pub fn encode_into(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE);

        buf.push(self.site_id);
        buf.extend(self.altitude.to_le_bytes());
        buf.extend(self.wind_full.to_le_bytes());
        buf.extend(self.wind_dir.to_le_bytes());
        buf.extend(self.temp.to_le_bytes());
        buf.extend(self.humidity.to_le_bytes());
        buf.extend(self.baro.to_le_bytes());

        buf
    }
    
    /// Decodes a `WeatherPayload` from a raw byte slice.
    ///
    /// The expected layout of `payload` is exactly `Self::SIZE` bytes:
    /// 
    /// | Offset | Field      | Type  | Size |
    /// |--------|------------|-------|------|
    /// | 0      | site_id    | u8    | 1    |
    /// | 1..5   | altitude   | f32   | 4    |
    /// | 5..9   | wind_full  | f32   | 4    |
    /// | 9..13  | wind_dir   | f32   | 4    |
    /// | 13..17 | temp       | f32   | 4    |
    /// | 17..21 | humidity   | f32   | 4    |
    /// | 21..25 | baro       | f32   | 4    |
    ///
    /// # Parameters
    ///
    /// - `payload`: A byte slice containing the serialized WeatherPayload.
    ///
    /// # Returns
    ///
    /// Returns `Ok(WeatherPayload)` if the slice has the correct length and can
    /// be converted into the appropriate numeric types. Otherwise returns an
    /// `Err` with a descriptive message.
    ///
    /// # Errors
    ///
    /// - If `payload.len() < Self::SIZE`, an error is returned.
    /// - If any slice cannot be converted into a `f32` (e.g., slice length mismatch), a
    ///   conversion error from `TryInto` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// let raw: [u8; 25] = [/* bytes from WeatherPacket payload */];
    /// let weather = WeatherPayload::decode_from(&raw)?;
    /// println!("Site ID: {}", weather.site_id);
    /// ```
    pub fn decode_from(payload: &[u8]) -> anyhow::Result<Self> {
        if payload.len() < Self::SIZE {
            anyhow::bail!("Weather payload too small");
        }
        Ok(Self {
            site_id: payload[0],
            altitude: f32::from_le_bytes(payload[1..5].try_into()?),
            wind_full: f32::from_le_bytes(payload[5..9].try_into()?),
            wind_dir: f32::from_le_bytes(payload[9..13].try_into()?),
            temp: f32::from_le_bytes(payload[13..17].try_into()?),
            humidity: f32::from_le_bytes(payload[17..21].try_into()?),
            baro: f32::from_le_bytes(payload[21..25].try_into()?),
        })
    }
}