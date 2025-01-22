use core::fmt;

use bincode::{config, Decode, Encode};
use wg_2024::network::NodeId;

#[derive(Debug, Clone, Encode, Decode)]
pub enum Compression {
    None,
    LZW,
}

#[derive(Debug)]
pub struct SerializationError;
impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serialization Error")
    }
}
impl std::error::Error for SerializationError {}

pub trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError>;
    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

impl<T: Encode + Decode> Serializable for T {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        bincode::encode_to_vec(self, config::standard()).map_err(|_| SerializationError)
    }

    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError> {
        match bincode::decode_from_slice(&data, config::standard()) {
            Ok((s, _)) => Ok(s),
            Err(_) => Err(SerializationError),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum TextRequest {
    TextList,
    Text(u64),
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum MediaRequest {
    MediaList,
    Media(u64),
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum TextResponse {
    TextList(Vec<u64>),
    Text(String),
    NotFound,
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum MediaResponse {
    MediaList(Vec<u64>),
    Media(Vec<u8>), // should we use some other type?
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Message<M> {
    pub source_id: NodeId,
    pub session_id: u64,
    pub compression_type: Compression,
    pub content: M,
}
