pub mod player_connection;
pub mod packet;
pub mod inbound;
pub mod outbound;
pub mod player;

use colored::Colorize;
use log::{info, trace};
use tokio::net::{TcpListener, TcpStream};
use crate::err::FerrumcError;
use crate::server::player_connection::Connection;

pub async fn start_server(host: &str, port: u16) -> Result<(), FerrumcError> {
    info!("Starting server on {}:{}", host.blue(), port.to_string().blue());

    let listener = TcpListener::bind(format!("{}:{}", host, port)).await.map_err(|_| FerrumcError::PortAlreadyInUse(port))?;


    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        trace!("Accepted connection from ({}:{})", addr.ip().to_string().green(), addr.port().to_string().blue());

        tokio::spawn(handle_connection(socket));
    }
}

pub async fn handle_connection(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Err(err) = connection.start_connection().await {
        trace!("Connection error: {}", err);
    }
}
