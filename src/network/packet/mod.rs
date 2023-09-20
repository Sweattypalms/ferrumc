use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use lazy_static::lazy_static;


use crate::network::connection::state::ConnectionState;
use crate::network::packet::inbound::{packet_play_in_handshake::PacketPlayInHandshake, packet_play_in_ping::PacketPlayInPing};
use crate::network::packet::inbound::packet_play_in_login_start::PacketPlayInLoginStart;
use crate::player::Connection;
use crate::utils::construct_async;

pub mod inbound;
pub mod outbound;

// #[async_trait]
// pub trait Packet: Send + Sync + 'static {
//     fn serialize(&self) -> Vec<u8>;
//     fn deserialize(bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized;
//     fn get_id(&self) -> u32;
//     fn get_name(&self) -> String;
//     async fn handle(&self, stream: &mut TcpStream);
//
//     fn construct_boxed(data: Vec<u8>) -> Box<dyn Packet> where Self: Sized,
//     {
//         Box::new(Self::deserialize(data).unwrap())
//     }
// }
//

#[async_trait]
pub trait InboundPacket: Send + Sync + 'static {
    async fn deserialize(bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized;
    fn get_id(&self) -> u32;
    async fn handle(&self, connection: &mut Connection);
}

#[async_trait]
pub trait OutboundPacket: Send + Sync + 'static {
    async fn serialize(&self) -> Result<Vec<u8>>;
    fn get_id(&self) -> u32;
}

type PacketConstructor = fn(Vec<u8>) -> Pin<Box<dyn Future<Output=Result<Box<dyn InboundPacket>, Error>> + Send>>;

pub struct PacketRegistry {
    pub inbound: HashMap<(ConnectionState, u32), PacketConstructor>,
}

lazy_static! {
    pub static ref REGISTRY: Arc<PacketRegistry> = {
        let mut registry = PacketRegistry::new();
        registry.initialize();
        Arc::new(registry)
    };
}
impl PacketRegistry {
    pub fn new() -> Self {
        PacketRegistry {
            inbound: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.register_packet::<PacketPlayInHandshake>(ConnectionState::Handshaking, 0x00);
        self.register_packet::<PacketPlayInLoginStart>(ConnectionState::Login, 0x00);
        self.register_packet::<PacketPlayInPing>(ConnectionState::Status, 0x01);
    }
    pub async fn deserialize_inbound(&self, state: ConnectionState, bytes: Vec<u8>) -> Option<Box<dyn InboundPacket>> {
        let id = bytes[1] as u32;
        if let Some(constructor) = self.inbound.get(&(state, id)) {
            match constructor(bytes).await {
                Ok(packet) => Some(packet),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn register_packet<T: InboundPacket + Send + 'static>(&mut self, state: ConnectionState, id: u32) {
        self.inbound.insert((state, id), |bytes| Box::pin(construct_async::<T>(bytes)));
    }
}