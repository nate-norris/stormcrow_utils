use tokio::time::sleep;

use super::trait_speaker::SpeakerT;
use super::models::ERROR_START;

/// A mock speaker implementation used for testing and development.
///
/// `SpeakerMock` does not interact with real hardware. Instead, it prints
/// messages to stdout to simulate:
#[allow(dead_code)]
pub(crate) struct SpeakerMock {}

impl SpeakerT for SpeakerMock {

    /// Initializes a new speaker instance.
    ///
    /// # Returns
    /// - `Ok(Self)` if initialization succeeds.
    fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }

    /// Plays a one-time “boom” beep pattern.
    ///
    /// # Behavior
    /// - Activated upon sound sensor edge detects.
    /// - It is only triggered when no error state has occurred.
    ///
    /// Returns Ok(())
    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        println!("speaker mock: boom pattern");
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
        // call spawn for error detection pattern one time only
        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    println!("speaker mock: error pattern output");
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }
}