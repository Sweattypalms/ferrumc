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
/// use ferrumc::create_packet;
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
    ($id:expr, $data:expr) => {
        {
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
        }
    };
}