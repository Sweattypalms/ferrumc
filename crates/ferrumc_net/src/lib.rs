mod handshake;
mod login_play;
mod login_start;
pub mod packet;
mod ping;
pub mod player;
pub mod player_connection;
mod player_position;
mod status;

use crate::player_connection::Connection;
use colored::Colorize;
use ferrumc_utils::err::FerrumcError;
use log::{info, trace};
use tokio::net::{TcpListener, TcpStream};

#[macro_export]
macro_rules! handle_packet {
    ($data:expr, $($state:pat => $id:expr => $handler:ident), *) => {
        match $data.connection.state {
                $(
                    $state => {
                        match $data.id{
                            $id => $handler($data).await,
                            _ => {
                                trace!("Unknown Packet ID {} for state {:?}", $data.id, $data.connection.state);
                                return Err(FerrumcError::InvalidPacketId);
                            }
                        }
                    }
                )*
                _ => {
                    trace!("Invalid state: {:?}", $data.connection.state);
                    return Err(FerrumcError::InvalidState);
                }
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
        .map_err(|_| FerrumcError::PortAlreadyInUse(port))?;

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        trace!(
            "Accepted connection from ({}:{})",
            addr.ip().to_string().green(),
            addr.port().to_string().blue()
        );

        tokio::spawn(handle_connection(socket));
    }
}

pub async fn handle_connection(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Err(err) = connection.start_connection().await {
        trace!("Connection error: {:?}", err);
    }
}