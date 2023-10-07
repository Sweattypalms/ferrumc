use byteorder::{BigEndian, WriteBytesExt};
use ferrumc_utils::err::FerrumcError;
use crate::create_packet;
use crate::packet::PacketData;
use crate::player_connection::Connection;
use crate::structs::{Angle, Position};
use ferrumc_utils::utils::MinecraftWriterExt;

pub async fn some_packet_received_on_join(packet_data: PacketData<'_>) -> Result<(), FerrumcError> {

    println!("Some packet received on join...");

    set_default_spawn_position(packet_data.connection).await?;
    player_position_and_look(packet_data.connection).await?;
    player_info(packet_data.connection).await?;


    Ok(())
}

pub async fn set_default_spawn_position(connection: &mut Connection) -> Result<(), FerrumcError> {
    println!("Sending spawn position...");

    let pos: Position = Position {
        x: 8,
        y: 100,
        z: 8,
    };

    let angle: Angle = Angle::from_degrees(0f32);

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&pos.to_bytes());
    bytes.extend_from_slice(&angle.rotation.to_be_bytes());

    let raw_bytes = create_packet!(0x4B, bytes)?;

    connection.write(&raw_bytes).await?;

    Ok(())
}

pub async fn player_position_and_look(connection: &mut Connection) -> Result<(), FerrumcError> {
//     Player Position And Look (clientbound)
//
// Updates the player's position on the server. This packet will also close the “Downloading Terrain” screen when joining/respawning.
//
// If the distance between the last known position of the player on the server and the new position set by this packet is greater than 100 meters, the client will be kicked for “You moved too quickly :( (Hacking?)”.
//
// Also if the fixed-point number of X or Z is set greater than 3.2E7D the client will be kicked for “Illegal position”.
//
// Yaw is measured in degrees, and does not follow classical trigonometry rules. The unit circle of yaw on the XZ-plane starts at (0, 1) and turns counterclockwise, with 90 at (-1, 0), 180 at (0, -1) and 270 at (1, 0). Additionally, yaw is not clamped to between 0 and 360 degrees; any number is valid, including negative numbers and numbers greater than 360.
//
// Pitch is measured in degrees, where 0 is looking straight ahead, -90 is looking straight up, and 90 is looking straight down.
// Packet ID 	State 	Bound To 	Field Name 	Field Type 	Notes
// 0x38 	Play 	Client 	X 	Double 	Absolute or relative position, depending on Flags.
// Y 	Double 	Absolute or relative position, depending on Flags.
// Z 	Double 	Absolute or relative position, depending on Flags.
// Yaw 	Float 	Absolute or relative rotation on the X axis, in degrees.
// Pitch 	Float 	Absolute or relative rotation on the Y axis, in degrees.
// Flags 	Byte 	Bit field, see below.
// Teleport ID 	VarInt 	Client should confirm this packet with Teleport Confirm containing the same Teleport ID.
// Dismount Vehicle 	Boolean 	True if the player should dismount their vehicle.
//
// About the Flags field:
//
// <Dinnerbone> It's a bitfield, X/Y/Z/Y_ROT/X_ROT. If X is set, the x value is relative and not absolute.
//
// Field 	Bit
// X 	0x01
// Y 	0x02
// Z 	0x04
// Y_ROT 	0x08
// X_ROT 	0x10

    let mut bytes = Vec::new();

    bytes.write_f64::<BigEndian>(0.0)?; // X
    bytes.write_f64::<BigEndian>(100.0)?; // Y
    bytes.write_f64::<BigEndian>(0.0)?; // Z

    bytes.write_f32::<BigEndian>(0.0)?; // Yaw
    bytes.write_f32::<BigEndian>(0.0)?; // Pitch

    bytes.write_u8(0x00)?; // Flags

    bytes.write_varint(0)?; // Teleport ID

    bytes.write_u8(0)?; // Dismount Vehicle

    let raw_bytes = create_packet!(0x38, bytes)?;

    connection.write(&raw_bytes).await?;

    log::trace!("Sent player position and look packet");

    Ok(())
}
pub async fn player_info(connection: &mut Connection) -> Result<(), FerrumcError> {
    println!("Sending player info...");

    let mut bytes = Vec::new();

    bytes.write_varint(0)?; // Action => Add player

    bytes.write_varint(1)?; // Number of players


    bytes.extend_from_slice(&connection.uuid.unwrap().as_bytes().to_vec()); // UUID

    bytes.write_varstring(&connection.username.clone().unwrap())?; // Username

    bytes.write_varint(0)?; // Number of properties

    bytes.write_varint(0)?; // Gamemode

    bytes.write_varint(0)?; // Ping

    bytes.write_u8(0)?; // Has display name

    let raw_bytes = create_packet!(0x36, bytes)?;

    connection.write(&raw_bytes).await?;

    log::trace!("Sent player info packet");

    Ok(())
}