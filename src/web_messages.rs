/*! This module contains the types used to implement communication between web clients and web servers */

use core::fmt;
use bincode::config::{standard, Configuration};
use bincode::{Encode, Decode};
use serde::{Serialize, de::DeserializeOwned};

use wg_2024::network::NodeId;
use crate::ServerType;

/// Compression type to be used in a web client-server communication
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum Compression {
    None,
    LZW,
    Huffman,
}

/// Error generated when a request/response is not serializable
#[derive(Debug)]
pub struct SerializationError;
impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serialization Error")
    }
}
impl std::error::Error for SerializationError {}

/// Reflects the capability of converting an object into and from a vector of bytes
pub trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError>;
    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

pub trait SerializableSerde {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError>;
    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

impl<T> Serializable for T
where
    T: Encode + Decode<Configuration>,
{
    fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        bincode::encode_to_vec(self, standard()).map_err(|_| SerializationError)
    }

    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError> {
        match bincode::decode_from_slice::<T, Configuration>(&data, standard()) {
            Ok((s, _)) => Ok(s),
            Err(_) => Err(SerializationError),
        }
    }
}

use bincode::serde::{encode_to_vec as serde_encode_to_vec, decode_from_slice as serde_decode_from_slice};

impl<T: Serialize + DeserializeOwned> SerializableSerde for T {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        serde_encode_to_vec(self, standard()).map_err(|_| SerializationError)
    }

    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError> {
        match serde_decode_from_slice(&data, standard()) {
            Ok((s, _)) => Ok(s),
            Err(_) => Err(SerializationError),
        }
    }
}

pub trait WebMessage {}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum TextRequest {
    TextList,
    Text(String),
}
impl WebMessage for TextRequest {}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum MediaRequest {
    MediaList,
    Media(String),
}
impl WebMessage for MediaRequest {}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum TextResponse {
    TextList(Vec<String>),
    Text(Vec<u8>),
}
impl WebMessage for TextResponse {}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum MediaResponse {
    MediaList(Vec<String>),
    Media(Vec<u8>),
}
impl WebMessage for MediaResponse {}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum GenericResponse {
    Type(ServerType),
    InvalidRequest,
    NotFound,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum Request {
    Media(MediaRequest),
    Text(TextRequest),
    Type,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum Response {
    Media(MediaResponse),
    Text(TextResponse),
    Generic(GenericResponse),
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct RequestMessage {
    pub source_id: NodeId,
    pub compression_type: Compression,
    pub content: Request,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct ResponseMessage {
    pub source_id: NodeId,
    pub compression_type: Compression,
    pub content: Response,
}

impl RequestMessage {
    #[inline]
    #[must_use]
    pub fn new_text_list_request(source_id: NodeId, compression_type: Compression) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Text(TextRequest::TextList),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_text_request(source_id: NodeId, compression_type: Compression, file: String) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Text(TextRequest::Text(file)),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_media_list_request(source_id: NodeId, compression_type: Compression) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Media(MediaRequest::MediaList),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_media_request(source_id: NodeId, compression_type: Compression, file: String) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Media(MediaRequest::Media(file)),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_type_request(source_id: NodeId, compression_type: Compression) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Type,
        }
    }
}

impl ResponseMessage {
    #[inline]
    #[must_use]
    pub fn new_type_response(source_id: NodeId, compression_type: Compression, server_type: ServerType) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Generic(GenericResponse::Type(server_type)),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_not_found_response(source_id: NodeId, compression_type: Compression) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Generic(GenericResponse::NotFound),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_invalid_request_response(source_id: NodeId, compression_type: Compression) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Generic(GenericResponse::InvalidRequest),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_text_list_response(source_id: NodeId, compression_type: Compression, list: Vec<String>) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Text(TextResponse::TextList(list)),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_text_response(source_id: NodeId, compression_type: Compression, data: Vec<u8>) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Text(TextResponse::Text(data)),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_media_list_response(source_id: NodeId, compression_type: Compression, list: Vec<String>) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Media(MediaResponse::MediaList(list)),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_media_response(source_id: NodeId, compression_type: Compression, data: Vec<u8>) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Media(MediaResponse::Media(data)),
        }
    }
}
