use core::fmt;

use bincode::{config, Decode, Encode};
use wg_2024::network::NodeId;

use crate::ServerType;

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
    /// # Errors
    ///
    /// Will return Err if the data is not serializable
    fn serialize(&self) -> Result<Vec<u8>, SerializationError>;

    /// # Errors
    ///
    /// Will return Err if the data is not deserializable
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

pub trait WebMessage {}

#[derive(Debug, Clone, Encode, Decode)]
pub enum TextRequest {
    TextList,
    Text(String),
}
impl WebMessage for TextRequest {}

#[derive(Debug, Clone, Encode, Decode)]
pub enum MediaRequest {
    MediaList,
    Media(String),
}
impl WebMessage for MediaRequest {}

#[derive(Debug, Clone, Encode, Decode)]
pub enum TextResponse {
    TextList(Vec<String>),
    Text(String),
    NotFound,
    InvalidRequest,
}
impl WebMessage for TextResponse {}

#[derive(Debug, Clone, Encode, Decode)]
pub enum MediaResponse {
    MediaList(Vec<String>),
    Media(Vec<u8>), // should we use some other type?
    NotFound,
    InvalidRequest,
}
impl WebMessage for MediaResponse {}

#[derive(Debug, Clone, Encode, Decode)]
pub enum GenericResponse {
    Type(ServerType)
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum Request{
    Media(MediaRequest),
    Text(TextRequest),
    Type,
}
#[derive(Debug, Clone, Encode, Decode)]
pub enum Response {
    Media(MediaResponse),
    Text(TextResponse),
    Generic(GenericResponse),
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct RequestMessage {
    pub source_id: NodeId,
    pub request_id: u64,
    pub compression_type: Compression,
    pub content: Request,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ResponseMessage {
    pub source_id: NodeId,
    pub request_id: u64,
    pub compression_type: Compression,
    pub content: Response,
}