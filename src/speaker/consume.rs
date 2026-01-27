use super::models::{SpeakerRx, SpeakerNotification};
use super::speaker_mock::SpeakerMock;
use super::trait_speaker::SpeakerT;

/// Asynchronous speaker consumer task.
///
/// This task listens on an `mpsc` channel (`SpeakerRx`) for speaker-related
/// notifications SpeakerNotification. It is typically spawned using `tokio::spawn` and runs until
/// the sending side of the channel is dropped.
///
/// # Behavior
/// - **`SpeakerNotification::Boom`**  
///   Triggers a one-time *boom* beep pattern using `boom_pattern()`,  
///   **but only if no error has been triggered yet**.
///
/// - **`SpeakerNotification::SoundSensorError` or `SpeakerNotification::RadioError`**  
///   Marks the system as being in an error state and calls
///   `spawn_error_pattern()`, which begins a *repeating* error beep loop.
/// Once an error state has been triggered, subsequent `Boom` events are ignored.
///
/// # Arguments
/// * `rx` – The receiving half of an `mpsc` channel through which
///   `SpeakerNotification` events are delivered.
///
/// # Example
/// ```no_run
/// use tokio::sync::mpsc;
/// use stormcrow_utils::speaker::speaker_consume_task;
/// use stormcrow_utils::speaker::SpeakerNotification;
///
/// #[tokio::main]
/// async fn main() {
///     let (tx, rx) = mpsc::channel(32);
///
///     // Spawn the speaker task
///     tokio::spawn(async move {
///         speaker_consume_task(rx).await;
///     });
///
///     // Send events
///     tx.send(SpeakerNotification::Boom).await.unwrap();
///     tx.send(SpeakerNotification::SoundSensorError).await.unwrap();
/// }
/// ```
///
/// # Notes
/// - `SpeakerMock` is used internally; replace with a real speaker
///   implementation by updating the constructor.
/// - Task terminates when `rx` is closed and all senders are dropped.
///
/// # Errors
/// This function does not return errors; speaker failures should be handled
/// inside the `SpeakerT` implementation.
pub async fn speaker_consume_task(mut rx: SpeakerRx) {
    let mut error_triggered = false;
    // speaker for sound patterns
    let speaker = SpeakerMock::new().unwrap();

    // wait for incoming SpeakerRx
    while let Some(event) = rx.recv().await {
        match event {
            // when there is a boom with no errors create the boom pattern
            SpeakerNotification::Boom => {
                if !error_triggered {
                    let _ = speaker.boom_pattern().await;
                }
            }
            // if errors occur turn on the error pattern
            //      lockout other booms from occuring
            SpeakerNotification::SoundSensorError | SpeakerNotification::RadioError => {
                error_triggered = true;
                speaker.spawn_error_pattern();
            }
        }
    }

}