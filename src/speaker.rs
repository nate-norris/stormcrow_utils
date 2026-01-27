//! # speaker
//!
//! Provides an abstraction layer for interacting with speaker
//! hardware or mocks in a Tokio async context. It includes:
//!
//! - **Event types and channels** (`SpeakerNotification`, `SpeakerTx`, `SpeakerRx`)
//! - **Speaker consumer task** (`speaker_consume_task`) for handling events
//! - **Trait for speaker implementations** (`SpeakerT`)
//! - **Concrete implementations** (`Speaker` and `SpeakerMock`)
//!
//! ## Overview
//!
//! This crate is designed to work with async event streams (`tokio::sync::mpsc`)
//! and provides both real and mock speaker implementations for testing
//! and development. The consumer task listens for events and triggers
//! one-shot "boom" patterns or repeating error patterns as appropriate.
//!
//! ## Usage
//!
//! ```no_run
//! use tokio::sync::mpsc;
//! use speaker::{speaker_consume_task, SpeakerNotification};
//!
//! #[tokio::main]
//! async fn main() {
//!     let (tx, rx) = mpsc::channel(32);
//!
//!     tokio::spawn(async move {
//!         speaker_consume_task(rx).await;
//!     });
//!
//!     tx.send(SpeakerNotification::Boom).await.unwrap();
//!     tx.send(SpeakerNotification::SoundSensorError).await.unwrap();
//! }
//! ```

mod trait_speaker;

pub mod models;
pub mod consume;
pub mod speaker_mock;
pub mod speaker_real;

// re-export commmon types and functions
pub use models::{SpeakerTx, SpeakerRx, SpeakerNotification};
pub use consume::speaker_consume_task;