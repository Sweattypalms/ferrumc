use std::fmt::Display;

#[derive(Debug)]
pub enum FerrumcError {
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    SerdeJson(serde_json::Error),
    PortAlreadyInUse(u16),
    StreamWriteError,
    InvalidVarInt,
    InvalidString,
    InvalidBigEndian,
    InvalidState,
    InvalidPacketId,
}

impl From<std::io::Error> for FerrumcError {
    fn from(err: std::io::Error) -> FerrumcError {
        FerrumcError::Io(err)
    }
}

impl From<toml::de::Error> for FerrumcError {
    fn from(err: toml::de::Error) -> FerrumcError {
        FerrumcError::TomlDeserialize(err)
    }
}

impl From<toml::ser::Error> for FerrumcError {
    fn from(err: toml::ser::Error) -> FerrumcError {
        FerrumcError::TomlSerialize(err)
    }
}

impl From<serde_json::Error> for FerrumcError {
    fn from(err: serde_json::Error) -> FerrumcError {
        FerrumcError::SerdeJson(err)
    }
}


impl Display for FerrumcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FerrumcError::Io(ref err) => write!(f, "IO error: {}", err),
            FerrumcError::TomlSerialize(ref err) => write!(f, "TOML error: {}", err),
            FerrumcError::TomlDeserialize(ref err) => write!(f, "TOML error: {}", err),
            FerrumcError::SerdeJson(ref err) => write!(f, "Serde JSON error: {}", err),
            FerrumcError::PortAlreadyInUse(port) => write!(f, "Couldn't bind to port {}! Perhaps port is already in use?", port),
            FerrumcError::StreamWriteError => write!(f, "Failed to write to stream!"),
            FerrumcError::InvalidVarInt => write!(f, "Invalid VarInt!"),
            FerrumcError::InvalidString => write!(f, "Invalid String!"),
            FerrumcError::InvalidBigEndian => write!(f, "Invalid BigEndian!"),
            FerrumcError::InvalidState => write!(f, "Invalid state!"),
            FerrumcError::InvalidPacketId => write!(f, "Invalid packet id!"),
        }
    }
}