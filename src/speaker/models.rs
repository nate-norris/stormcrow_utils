use tokio::sync::mpsc;
use serde::{Deserialize};

/// Notification events emitted by speaker hardware or serial interrupts
#[derive(Debug, Deserialize)]
pub enum SpeakerNotification {
    Boom, // sound sensor has detected an event
    // both SoundSensorError and Radio error trigger program failure
    SoundSensorError, // notification for sound sensor hardware issues
    RadioError, // notification for radio hardware issues
    AirmarError, // notification for airmar weather hardware issues
    WeatherTimeoutError(bool), // notification for delay in receiving weather
    GeneralError, // general purpose error alert
    GeneralAlert, // general purpose non error alert
}

// mpsc channels for SpeakerNotification types
// used by producers to send edge events
pub type SpeakerTx = mpsc::Sender<SpeakerNotification>;
// used by consuemrs to receive edge events
pub type SpeakerRx = mpsc::Receiver<SpeakerNotification>;  

// global one-time initialization flag for triggering
//      speaker error pattern only once
//
// Implementations wrap calls with ERRO_START.call_once()
pub(crate) static ERROR_START: std::sync::Once = std::sync::Once::new();