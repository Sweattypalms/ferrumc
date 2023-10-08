pub mod chat;
pub mod chunk_data;
pub mod entity_action;
pub mod handshake;
pub mod join_settings;
pub mod login_play;
pub mod login_start;
pub mod packet;
pub mod ping;
pub mod player;
pub mod player_connection;
pub mod player_position;
pub mod status;
pub mod structs;

use crate::player_connection::Connection;
use colored::Colorize;
use ferrumc_utils::err::FerrumcError;
use lazy_static::lazy_static;
use log::{info, trace};
use priomutex;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

lazy_static! {
    static ref CONN_POOL: Arc<tokio::sync::Mutex<Vec<Connection>>> =
        Arc::new(tokio::sync::Mutex::new(Vec::new()));
}

#[macro_export]
macro_rules! handle_packet {
    ($data:expr, $($state:pat => { $($id:expr => $handler:ident),+ }), *) => {
        match $data.connection.state {
            $(
                $state => {
                    match $data.id {
                        $($id => $handler(&mut $data).await,)+
                        _ => {
                            trace!("Unknown Packet ID {} ({:#02X}) for state {:?}", $data.id, $data.id, $data.connection.state);
                            return Err(FerrumcError::InvalidPacketId);
                        }
                    }
                }
            )*
        }
    };
}

/// Creates a packet handler for the given state and packet id.<br>
///
/// # Example
/// ```
/// use ferrumc_net::create_packet;
/// use ferrumc_utils::utils::MinecraftWriterExt;
///
/// let mut  somedatabytes: Vec<u8> = vec![0x00, 0x01, 0x02];
/// let mut buffer = Vec::new();
/// buffer.write_varint(somedatabytes.len() as i32)?;
/// buffer.append(&mut somedatabytes);
/// let bytes = create_packet!(0x00, buffer);
/// ```
///
/// @return Returns raw bytes to be sent to the client.
#[macro_export]
macro_rules! create_packet {
    ($id:expr, $data:expr) => {{
        let out: Result<Vec<u8>, FerrumcError> = {
            let mut temp_buffer = vec![];

            temp_buffer.write_varint($id)?;

            let mut data_bytes = $data;

            // temp_buffer.write_varint(data_bytes.len() as i32)?;

            temp_buffer.append(&mut data_bytes);

            let packet_length = temp_buffer.len() as i32;

            let mut buffer = Vec::new();
            // let  buffer.write_varint(packet_length)?;
            buffer.write_varint(packet_length)?;

            buffer.append(&mut temp_buffer);

            Ok(buffer)
        };
        out
    }};
}

pub async fn start_server(host: &str, port: u16) -> Result<(), FerrumcError> {
    info!(
        "Starting server on {}:{}",
        host.blue(),
        port.to_string().blue()
    );

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .map_err(|_| FerrumcError::FailedPortBind(port))?;

    tokio::spawn(iterate_pool());

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        trace!(
            "Accepted connection from ({}:{})",
            addr.ip().to_string().green(),
            addr.port().to_string().blue()
        );

        CONN_POOL.lock().await.push(Connection::new(socket));
    }
}

pub async fn handle_connection(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Err(err) = connection.start_connection().await {
        trace!("Connection error: {:?}", err);
    }
}

async fn iterate_pool() -> Result<(), FerrumcError> {
    loop {
        let startms = std::time::Instant::now();
        let mut pool = CONN_POOL.lock().await;

        for connection in pool.iter_mut() {
            if let Err(err) = connection.tick().await {
                trace!("Tick error: {:?}", err);
            }
            connection.read_packet().await?;
            connection.stream.flush().await.unwrap();
        }
        drop(pool);
        let endms = std::time::Instant::now();
        let elapsed = endms - startms;
        if elapsed.as_millis() < 50 {
            tokio::time::sleep(std::time::Duration::from_millis(
                50 - elapsed.as_millis() as u64,
            ))
            .await;
        }
    }
}