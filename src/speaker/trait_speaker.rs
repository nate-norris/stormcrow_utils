/// Trait defining speaker behavior for real or mocked implementations.
/// 
/// To be used by Rx events to notify for sound sensor edge detection
/// or errors in hardware.
pub(crate) trait SpeakerT {
    /// Initializes a new speaker instance.
    ///
    /// # Returns
    /// - `Ok(Self)` if initialization succeeds.
    /// - `Err(...)` if initialization fails.
    ///
    /// # Errors
    /// Returns an error if the port cannot be created,
    /// initialized, or configured properly.
    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

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
    async fn boom_pattern(&self) -> anyhow::Result<()>;


    /// Starts a repeating error notification pattern.
    ///
    /// # Behavior
    /// - Uses `ERROR_START.call_once` to ensure the loop is only launched once.
    /// - Spawns the loop via `tokio::spawn` so the function returns immediately.
    ///
    /// # Notes
    /// This function is intentionally non-async because it should launch
    /// its own async task rather than block the caller.
    fn spawn_error_pattern(&self);
}