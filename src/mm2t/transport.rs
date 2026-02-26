#[cfg(any(feature = "mm2t-tx", feature = "mm2t-rx"))]
use tokio::sync::Mutex;
#[cfg(any(feature = "mm2t-tx", feature = "mm2t-rx"))]
use tokio_serial::{SerialStream, SerialPortBuilderExt, DataBits, Parity, 
    StopBits, FlowControl}; //SerialStream
#[cfg(feature = "mm2t-tx")]
use tokio::io::AsyncWriteExt;
#[cfg(feature = "mm2t-rx")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "mm2t-rx")]
use std::sync::Arc;

/// Represents a handle to an MM2T device over a serial connection.
///
/// This struct manages a serial port connection internally via a `Mutex` for
/// safe asynchronous access. It provides methods to initialize the device and
/// send a bytes.
///
/// # Example
/// ```rust,no_run
/// use mm2t::MM2TTransport;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Initialize the MM2T device
///     let mm2t = MM2TTransport::start("/dev/ttyUSB0").await?;
///
///     // Send a trigger packet
///     mm2t.send(&[0x01]).await?;
///
///     Ok(())
/// }
/// ```
pub struct MM2TTransport {
    #[cfg(any(feature = "mm2t-tx", feature = "mm2t-rx"))]
    port: Mutex<SerialStream>
}

impl MM2TTransport {

    /// opens a serial connection to the MM2T device
    #[cfg(any(feature = "mm2t-tx", feature = "mm2t-rx"))]
    pub async fn start(port_name: &str) -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new(port_name, 38_400)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(std::time::Duration::from_secs(3));

        let stream = port_builder
            .open_native_async()?;

        Ok(Self {
            port: Mutex::new(stream)
        })
    }

    #[cfg(not(any(feature = "mm2t-tx", feature = "mm2t-rx")))]
    pub async fn start(_port_name: &str) -> anyhow::Result<Self> {
        anyhow::bail!("MM2TTransport requires feature mm2t-tx or mm2t-rx");
    }

    /// send bytes over the port
    #[cfg(feature = "mm2t-tx")]
    pub async fn send(&self, bytes: &[u8]) -> anyhow::Result<()> {
        println!("mm2t send: {:?}", bytes);
        let mut port = self.port.lock().await;
        port.write_all(bytes).await?;
        port.flush().await?;

        Ok(())
    }

    /// read bytes over the port
    #[cfg(feature = "mm2t-rx")]
    pub async fn read(&self) -> anyhow::Result<u8> {
        let mut port = self.port.lock().await;
        let mut buf = [0u8; 1];
        
        port.read_exact(&mut buf).await?;
        println!("mm2t read: {:?}", buf[0]);
        Ok(buf[0])
    }

    #[cfg(feature = "mm2t-rx")]
    pub fn spawn_raw_read(&self) {
        let transport = Arc::new(self);
        tokio::spawn(async move {
            let transport = Arc::clone(&transport);
            let mut buf = [0u8; 64];

            loop {
                let n = {
                    let mut port = transport.port.lock().await;
                    match port.read(&mut buf).await {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Read error: {:?}", e);
                            break;
                        }
                    }
                };

                if n > 0 {
                    print!("RX: ");
                    for b in &buf[..n] {
                        print!("{:02X} ", b);
                    }
                    println!();
                }
            }
        });
    }
}