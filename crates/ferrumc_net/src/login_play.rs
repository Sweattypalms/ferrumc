// Join Game
//
// See Protocol Encryption for information on logging in.
// Packet ID 	State 	Bound To 	Field Name 	Field Type 	Notes
// 0x26 	Play 	Client 	Entity ID 	Int 	The player's Entity ID (EID).
// Is hardcore 	Boolean
// Gamemode 	Unsigned Byte 	0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
// Previous Gamemode 	Byte 	0: survival, 1: creative, 2: adventure, 3: spectator. The hardcore flag is not included. The previous gamemode. Defaults to -1 if there is no previous gamemode. (More information needed)
// World Count 	VarInt 	Size of the following array.
// World Names 	Array of Identifier 	Identifiers for all worlds on the server.
// Dimension Codec 	NBT Tag Compound 	The full extent of these is still unknown, but the tag represents a dimension and biome registry. See below for the vanilla default.
// Dimension 	NBT Tag Compound 	Valid dimensions are defined per dimension registry sent before this. The structure of this tag is a dimension type (see below).
// World Name 	Identifier 	Name of the world being spawned into.
// Hashed seed 	Long 	First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
// Max Players 	VarInt 	Was once used by the client to draw the player list, but now is ignored.
// View Distance 	VarInt 	Render distance (2-32).
// Reduced Debug Info 	Boolean 	If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
// Enable respawn screen 	Boolean 	Set to false when the doImmediateRespawn gamerule is true.
// Is Debug 	Boolean 	True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks.
// Is Flat 	Boolean 	True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63.
//
//
// The Dimension Codec NBT Tag Compound (Default value in SNBT) includes two registries: "minecraft:dimension_type" and "minecraft:worldgen/biome".
// Name 	Type 	Notes
// minecraft:dimension_type 	TAG_Compound 	The dimension type registry (see below).
// minecraft:worldgen/biome 	TAG_Compound 	The biome registry (see below).
//
// Dimension type registry:
// Name 	Type 	Notes
// type 	TAG_String 	The name of the registry. Always "minecraft:dimension_type".
// value 	TAG_List 	List of dimension types registry entries (see below).
//
// Dimension type registry entry:
// Name 	Type 	Notes
// name 	TAG_String 	The name of the dimension type (for example, "minecraft:overworld").
// id 	TAG_Int 	The protocol ID of the dimension (matches the index of the element in the registry list).
// element 	TAG_Compound 	The dimension type (see below).
//
// Dimension type:
// Name 	Type 	Meaning 	Values
// piglin_safe 	TAG_Byte 	Whether piglins shake and transform to zombified piglins. 	1: true, 0: false.
// natural 	TAG_Byte 	When false, compasses spin randomly. When true, nether portals can spawn zombified piglins. 	1: true, 0: false.
// ambient_light 	TAG_Float 	How much light the dimension has. 	0.0 to 1.0.
// fixed_time 	Optional TAG_Long 	If set, the time of the day is the specified value. 	If set, 0 to 24000.
// infiniburn 	TAG_String 	A resource location defining what block tag to use for infiniburn. 	"" or minecraft resource "minecraft:...".
// respawn_anchor_works 	TAG_Byte 	Whether players can charge and use respawn anchors. 	1: true, 0: false.
// has_skylight 	TAG_Byte 	Whether the dimension has skylight access or not. 	1: true, 0: false.
// bed_works 	TAG_Byte 	Whether players can use a bed to sleep. 	1: true, 0: false.
// effects 	TAG_String 	? 	"minecraft:overworld", "minecraft:the_nether", "minecraft:the_end" or something else.
// has_raids 	TAG_Byte 	Whether players with the Bad Omen effect can cause a raid. 	1: true, 0: false.
// min_y 	TAG_Int 	The minimum Y level.
// height 	TAG_Int 	The maximum height.
// logical_height 	TAG_Int 	The maximum height to which chorus fruits and nether portals can bring players within this dimension. 	0-256.
// coordinate_scale 	TAG_Float 	The multiplier applied to coordinates when traveling to the dimension. 	0.00001 - 30000000.0
// ultrawarm 	TAG_Byte 	Whether the dimensions behaves like the nether (water evaporates and sponges dry) or not. Also causes lava to spread thinner. 	1: true, 0: false.
// has_ceiling 	TAG_Byte 	Whether the dimension has a bedrock ceiling or not. When true, causes lava to spread faster. 	1: true, 0: false.
//
// Biome registry:
// Name 	Type 	Notes
// type 	TAG_String 	The name of the registry. Always "minecraft:worldgen/biome".
// value 	TAG_List 	List of biome registry entries (see below).
//
// Biome registry entry:
// Name 	Type 	Notes
// name 	TAG_String 	The name of the biome (for example, "minecraft:ocean").
// id 	TAG_Int 	The protocol ID of the biome (matches the index of the element in the registry list).
// element 	TAG_Compound 	The biome properties (see below).
//
// Biome properties:
// Name 	Type 	Meaning 	Values
// precipitation 	TAG_String 	The type of precipitation in the biome. 	"rain", "snow", or "none".
// depth 	TAG_Float 	The depth factor of the biome. 	The default values vary between 1.5 and -1.8.
// temperature 	TAG_Float 	The temperature factor of the biome. 	The default values vary between 2.0 and -0.5.
// scale 	TAG_Float 	? 	The default values vary between 1.225 and 0.0.
// downfall 	TAG_Float 	? 	The default values vary between 1.0 and 0.0.
// category 	TAG_String 	The category of the biome. 	Known values are "ocean", "plains", "desert", "forest", "extreme_hills", "taiga", "swamp", "river", "nether", "the_end", "icy", "mushroom", "beach", "jungle", "mesa", "savanna", and "none".
// temperature_modifier 	Optional TAG_String 	? 	The only known value is "frozen".
// effects 	sky_color 	TAG_Compound 	TAG_Int 	The color of the sky. 	Example: 8364543, which is #7FA1FF in RGB.
// water_fog_color 	TAG_Int 	Possibly the tint color when swimming. 	Example: 8364543, which is #7FA1FF in RGB.
// fog_color 	TAG_Int 	Possibly the color of the fog effect when looking past the view distance. 	Example: 8364543, which is #7FA1FF in RGB.
// water_color 	TAG_Int 	The tint color of the water blocks. 	Example: 8364543, which is #7FA1FF in RGB.
// foliage_color 	Optional TAG_Int 	The tint color of the grass. 	Example: 8364543, which is #7FA1FF in RGB.
// grass_color 	Optional TAG_Int 	? 	Example: 8364543, which is #7FA1FF in RGB.
// grass_color_modifier 	Optional TAG_String 	Unknown, likely affects foliage color. 	If set, known values are "swamp" and "dark_forest".
// music 	Optional TAG_Compound 	Music properties for the biome. 	If present, contains the fields: replace_current_music (TAG_Byte), sound (TAG_String), max_delay (TAG_Int), min_delay (TAG_Int).
// ambient_sound 	Optional TAG_String 	Ambient soundtrack. 	If present, the ID of a soundtrack. Example: "minecraft:ambient.basalt_deltas.loop".
// additions_sound 	Optional TAG_Compound 	Additional ambient sound that plays randomly. 	If present, contains the fields: sound (TAG_String), tick_chance (TAG_Double).
// mood_sound 	Optional TAG_Compound 	Additional ambient sound that plays at an interval. 	If present, contains the fields: sound (TAG_String), tick_delay (TAG_Int), offset (TAG_Double), block_search_extent (TAG_Int).
// particle 	probability 	Optional TAG_Compound 	TAG_FLOAT 	Particles that appear randomly in the biome. 	Possibly the probability of spawning the particle. 	?
// options 	TAG_COMPOUND 	The properties of the particle to spawn. 	Contains the field "type" (TAG_String), which identifies the particle type.

use crate::create_packet;
use crate::player_connection::Connection;
use byteorder::{BigEndian, WriteBytesExt};
use fastnbt::nbt;
use ferrumc_utils::err::FerrumcError;
use ferrumc_utils::utils::MinecraftWriterExt;
use ferrumc_world::nbtstructs::dimension_codec::{
    BiomeRegistry, BiomeRegistryEffects, BiomeRegistryElement, BiomeRegistryValue, DimensionCodec,
    DimensionType, DimensionTypeElement, DimensionTypeValue,
};
use log::trace;
use tokio::io::AsyncWriteExt;

pub async fn login_play(connection: &mut Connection) -> Result<(), FerrumcError> {
    let mut buffer = Vec::new();

    // Entity ID
    WriteBytesExt::write_i32::<BigEndian>(&mut buffer, 0)?;

    // Is hardcore
    WriteBytesExt::write_u8(&mut buffer, 0)?;

    // Gamemode
    WriteBytesExt::write_u8(&mut buffer, 1)?;

    // Previous Gamemode
    WriteBytesExt::write_i8(&mut buffer, -1)?;

    // World Count
    buffer.write_varint(1)?;

    // World Names
    buffer.write_varstring("minecraft:overworld")?;

    // Dimension Codec
    let dimension_codec = DimensionCodec {
        dimension_type: DimensionType {
            r#type: "minecraft:dimension_type".to_string(),
            value: vec![DimensionTypeValue {
                name: "minecraft:overworld".to_string(),
                id: 0, // Example ID, this may differ
                element: DimensionTypeElement {
                    piglin_safe: false,
                    natural: true,
                    ambient_light: 0.5, // Example value
                    infiniburn: "minecraft:some_value".to_string(), // Placeholder value
                    respawn_anchor_works: false,
                    has_skylight: true,
                    bed_works: true,
                    effects: "minecraft:overworld".to_string(),
                    has_raids: false,
                    min_y: 0,
                    height: 256,
                    logical_height: 256,
                    coordinate_scale: 1.0,
                    ultrawarm: false,
                    has_ceiling: false,
                },
            }],
        },
        biome_registry: BiomeRegistry {
            r#type: "minecraft:worldgen/biome".to_string(),
            value: vec![
                BiomeRegistryValue {
                    name: "minecraft:ocean".to_string(),
                    id: 0, // Example ID, this may differ
                    element: BiomeRegistryElement {
                        precipitation: "rain".to_string(),
                        depth: 0.5,       // Example value
                        temperature: 0.5, // Example value
                        scale: 1.0,       // Example value
                        downfall: 0.5,    // Example value
                        category: "ocean".to_string(),
                        effects: BiomeRegistryEffects {
                            sky_color: 8364543,
                            water_fog_color: 8364543,
                            fog_color: 8364543,
                            water_color: 8364543,
                        },
                    },
                },
                BiomeRegistryValue {
                    name: "minecraft:plains".to_string(),
                    id: 1, // Example ID, this may differ
                    element: BiomeRegistryElement {
                        precipitation: "rain".to_string(),
                        depth: 0.125,     // Example value for plains
                        temperature: 0.8, // Example value for plains
                        scale: 0.05,      // Example value for plains
                        downfall: 0.4,    // Example value for plains
                        category: "plains".to_string(),
                        effects: BiomeRegistryEffects {
                            sky_color: 7907327, // Placeholder values, you might need to adjust
                            water_fog_color: 7907327,
                            fog_color: 7907327,
                            water_color: 7907327,
                        },
                    },
                },
            ],
        },
    };

    let codec_value = nbt!({
        "minecraft:dimension_type": dimension_codec.dimension_type,
        "minecraft:worldgen/biome": dimension_codec.biome_registry,
    });

    let dimension_nbt = fastnbt::to_bytes(&codec_value)?;

    // trace!("Dimension NBT: {:?}", codec_value);

    buffer.extend_from_slice(&dimension_nbt);

    // Dimension

    let dimension_codec = nbt!({
        "piglin_safe": false,
        "natural": true,
        "ambient_light": 0.5,
        "infiniburn": "minecraft:some_value",
        "respawn_anchor_works": false,
        "has_skylight": true,
        "bed_works": true,
        "effects": "minecraft:overworld",
        "has_raids": false,
        "min_y": 0,
        "height": 256,
        "logical_height": 256,
        "coordinate_scale": 1.0,
        "ultrawarm": false,
        "has_ceiling": false,
    });

    let dimension = fastnbt::to_bytes(&dimension_codec)?;

    buffer.extend_from_slice(&dimension);

    // World Name
    buffer.write_varstring("minecraft:overworld")?;

    // Hashed seed
    WriteBytesExt::write_i64::<BigEndian>(&mut buffer, 0)?;

    // Max Players
    buffer.write_varint(20)?;

    // View Distance
    buffer.write_varint(10)?;

    // Reduced Debug Info
    WriteBytesExt::write_u8(&mut buffer, 0)?;

    // Enable respawn screen
    WriteBytesExt::write_u8(&mut buffer, 1)?;

    // Is Debug
    WriteBytesExt::write_u8(&mut buffer, 0)?;

    // Is Flat
    WriteBytesExt::write_u8(&mut buffer, 1)?;

    let raw = create_packet!(0x26, buffer)?;

    trace!("Login Play packet size: {}", raw.len());

    connection.stream.flush().await?;
    connection.write(&raw).await?;

    Ok(())
}