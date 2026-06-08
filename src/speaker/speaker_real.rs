
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio_serial::{SerialStream, SerialPortBuilderExt};
use tokio::time::sleep;

use super::trait_speaker::SpeakerT;
use super::models::ERROR_START;

/// A real speaker implementation using a serial port.
///
/// This struct communicates with a physical speaker device connected  
/// The device expects byte sequences to trigger boom and error patterns.
#[allow(dead_code)]
pub(crate) struct SpeakerReal {
    port: Arc<Mutex<SerialStream>>,
}
#[async_trait::async_trait]
impl SpeakerT for SpeakerReal {
    /// initialized, or configured properly.
    /// 
    /// Initializes a new speaker instance.
    ///
    /// # Returns
    /// - `Ok(Self)` if initialization succeeds.
    /// - `Err(...)` if initialization fails.
    ///
    /// # Errors
    /// Returns an error if the port cannot be created,
    /// initialized, or configured properly.
    #[allow(dead_code)]
    fn new() -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new(
            "/dev/ttyUSB2", 115_200);
        let port = port_builder.open_native_async()?;
        Ok(Self { 
            port: Arc::new(Mutex::new(port)),
        })
    }

    /// Plays a one-time “boom” beep pattern.
    ///
    /// # Behavior
    /// - Activated upon sound sensor edge detects.
    /// - It is only triggered when no error state has occurred.
    ///
    /// Returns an error if the tone or pattern playback fails.
    /// - `Ok(())` if the boom succeeds.
    /// - `Err(...)` if boom serial fails.
    /// 
    /// # Errors
    ///  Returns an error if the boom fails.
    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        let mut port = self.port.lock().await;

        port.write_all(b"1").await?;
        sleep(std::time::Duration::from_millis(500)).await;
        port.write_all(b"1").await?;
        Ok(())
    }

    /// Starts a repeating error notification pattern.
    ///
    /// # Behavior
    /// - Uses `ERROR_START.call_once` to ensure the loop is only launched once.
    /// - Spawns the loop via `tokio::spawn` so the function returns immediately.
    ///
    /// # Notes
    /// This function is intentionally non-async because it should launch
    /// its own async task rather than block the caller.
    #[allow(dead_code)]
    fn spawn_error_pattern(&self) {
        let port = self.port.clone();
        // call spawn for error detection pattern one time only
        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    let mut p = port.lock().await;
                    // send bytes for beep command
                    //      if error occurs simply log
                    let _ = p.write_all(b"1").await;

                    drop(p); // release lock before sleeping
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }

    /// Implementation of general alert pattern
    /// 
    /// # Behavior
    /// This is activated when the caller has an error or alert that is non
    /// specific.
    /// 
    /// The pattern is used with RecoverableRunner struct and the watch channel 
    /// and can be turned on or off for continuous alerts.
    #[allow(dead_code)]
    async fn perform_general_alert(&self) -> anyhow::Result<()>{
        let mut port = self.port.lock().await;
        for _ in 0..3 {
            port.write_all(b"1").await?;
            sleep(std::time::Duration::from_millis(250)).await;
        }
        Ok(())
    }
}