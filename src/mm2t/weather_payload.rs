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
    pub altitude: i16,
    pub wind_full: f32,
    pub wind_dir: f32,
    pub temp: f32,
    pub humidity: f32,
    pub baro: f32,
}

impl WeatherPayload {
    const SIZE: usize = 23; // buffer allocation size

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
    /// | 1..3   | altitude   | f32   | 4    |
    /// | 3..7   | wind_full  | f32   | 4    |
    /// | 7..11  | wind_dir   | f32   | 4    |
    /// | 11..15 | temp       | f32   | 4    |
    /// | 15..19 | humidity   | f32   | 4    |
    /// | 19..23 | baro       | f32   | 4    |
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
    /// - If any slice cannot be converted into a its type (e.g., slice length mismatch), a
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
            altitude: i16::from_le_bytes(payload[1..3].try_into()?),
            wind_full: f32::from_le_bytes(payload[3..7].try_into()?),
            wind_dir: f32::from_le_bytes(payload[7..11].try_into()?),
            temp: f32::from_le_bytes(payload[11..15].try_into()?),
            humidity: f32::from_le_bytes(payload[15..19].try_into()?),
            baro: f32::from_le_bytes(payload[19..23].try_into()?),
        })
    }
}