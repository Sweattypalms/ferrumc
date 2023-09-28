use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FerrumcError {
    InvalidPacketID,
    InvalidState,
    NotYetImplemented,
    InvalidString,
    InvalidVarInt,
    InvalidBigEndian,
}

impl Display for FerrumcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FerrumcError::InvalidPacketID => write!(f, "The packet ID is invalid"),
            FerrumcError::InvalidState => write!(f, "The connection is in an invalid state"),
            FerrumcError::NotYetImplemented => write!(f, "This feature is not yet implemented"),
            FerrumcError::InvalidString => write!(f, "The string is invalid"),
            FerrumcError::InvalidVarInt => write!(f, "The VarInt is invalid"),
            FerrumcError::InvalidBigEndian => write!(f, "The BigEndian is invalid"),
        }
    }
}

// Implement the std::error::Error trait for your custom error type.
impl std::error::Error for FerrumcError {
    fn description(&self) -> &str {
        match self {
            FerrumcError::InvalidPacketID => "The packet ID is invalid",
            FerrumcError::InvalidState => "The connection is in an invalid state",
            FerrumcError::NotYetImplemented => "This feature is not yet implemented",
            FerrumcError::InvalidString => "The string is invalid",
            FerrumcError::InvalidVarInt => "The VarInt is invalid",
            FerrumcError::InvalidBigEndian => "The BigEndian is invalid",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}