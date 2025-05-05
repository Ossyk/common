/*!
    This module contains the types used to implement communication between
    web clietns and web servers
*/

use core::fmt;

use bincode::{config, Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};
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

/// Reflects the capability of converting an object into and from a vector of bytes
pub trait SerializableSerde {
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
use bincode::config::{self, Configuration};

impl<T: Encode + for<'a> Decode<config::WithOtherEndian<config::DefaultEncoding, config::LittleEndian>>> Serializable for T {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        bincode::encode_to_vec(self, config::standard())
            .map_err(|_| SerializationError)
    }

    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError> {
        match bincode::decode_from_slice(&data, config::standard()) {
            Ok((s, _)) => Ok(s),
            Err(_) => Err(SerializationError),
        }
    }
}

// Sadly, we can't use the same trait due to conflicts
impl<T: Serialize + DeserializeOwned> SerializableSerde for T {
    fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        bincode::serde::encode_to_vec(self, config::standard()).map_err(|_| SerializationError)
    }

    fn deserialize(data: Vec<u8>) -> Result<Self, SerializationError> {
        match bincode::serde::decode_from_slice(&data, config::standard()) {
            Ok((s, _)) => Ok(s),
            Err(_) => Err(SerializationError),
        }
    }
}

/// Identifies a message that can be exchanged between web clients/servers
pub trait WebMessage {}

/// Identifies the types of requests that can be sent to a text server
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum TextRequest {
    /// Client request a list of all the text files inside the text server
    TextList,
    /// Client request the text file identified by the String parameter
    Text(String),
}
impl WebMessage for TextRequest {}

/// Identifies the types of requests that can be sent to a media server
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum MediaRequest {
    /// Client request a list of all the text files inside the media server
    MediaList,
    /// Client request the media file identified by the String parameter
    Media(String),
}
impl WebMessage for MediaRequest {}

/// Identifies the types of response that can be sent from a text server
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum TextResponse {
    /// Server sends the list of filenames of its own text files
    TextList(Vec<String>),
    /// Server sends a serialized text file
    Text(Vec<u8>),
}
impl WebMessage for TextResponse {}

/// Identifies the types of response that can be sent from a media server
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum MediaResponse {
    /// Server sends the list of filenames of its own media files
    MediaList(Vec<String>),
    /// Server sends a serialized media file
    Media(Vec<u8>),
}
impl WebMessage for MediaResponse {}

/// Identifies the types of response that can be sent from both text and media server
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum GenericResponse {
    /// Server sends its own type (text or media)
    Type(ServerType),
    /// Server received an invalid request
    InvalidRequest,
    /// Server received a request for a file that cannot be found
    NotFound,
}

/// General type for a web request that can be sent by a web client
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum Request {
    Media(MediaRequest),
    Text(TextRequest),
    Type,
}

/// General type for a web response that can be sent by a web server
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum Response {
    Media(MediaResponse),
    Text(TextResponse),
    Generic(GenericResponse),
}

/// Contains all the details of a web request
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct RequestMessage {
    pub source_id: NodeId,
    pub compression_type: Compression,
    pub content: Request,
}

/// Contains all the details of a web response
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct ResponseMessage {
    pub source_id: NodeId,
    pub compression_type: Compression,
    pub content: Response,
}

impl RequestMessage {
    /// Constructor for a text list request
    /// * `source_id`: id of the web client that creates the request
    /// * `compression_type`: compression algorithm to be used for the response's content
    #[inline]
    #[must_use]
    pub fn new_text_list_request(
        source_id: NodeId,
        compression_type: Compression,
    ) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Text(TextRequest::TextList),
        }
    }

    /// Constructor for a text request
    /// * `source_id`: id of the web client that creates the request
    /// * `compression_type`: compression algorithm to be used for the response's content
    /// * file: name of the requested text file
    #[inline]
    #[must_use]
    pub fn new_text_request(
        source_id: NodeId,
        compression_type: Compression,
        file: String,
    ) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Text(TextRequest::Text(file)),
        }
    }

    /// Constructor for a media list request
    /// * `source_id`: id of the web client that creates the request
    /// * `compression_type`: compression algorithm to be used for the response's content
    #[inline]
    #[must_use]
    pub fn new_media_list_request(
        source_id: NodeId,
        compression_type: Compression,
    ) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Media(MediaRequest::MediaList),
        }
    }

    /// Constructor for a media request
    /// * `source_id`: id of the web client that creates the request
    /// * `compression_type`: compression algorithm to be used for the response's content
    /// * file: name of the requested media file
    #[inline]
    #[must_use]
    pub fn new_media_request(
        source_id: NodeId,
        compression_type: Compression,
        file: String,
    ) -> RequestMessage {
        Self {
            source_id,
            compression_type,
            content: Request::Media(MediaRequest::Media(file)),
        }
    }

    /// Constructor for a type request
    /// * `source_id`: id of the web client that creates the request
    /// * `compression_type`: compression algorithm to be used for the response's content
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
    /// Constructor for a type response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    /// * `server_type`: type of server that sends this response
    #[inline]
    #[must_use]
    pub fn new_type_response(
        source_id: NodeId,
        compression_type: Compression,
        server_type: ServerType,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Generic(GenericResponse::Type(server_type)),
        }
    }

    /// Constructor for a "not found" response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    #[inline]
    #[must_use]
    pub fn new_not_found_response(
        source_id: NodeId,
        compression_type: Compression,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Generic(GenericResponse::NotFound),
        }
    }

    /// Constructor for an "invalid request" response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    #[inline]
    #[must_use]
    pub fn new_invalid_request_response(
        source_id: NodeId,
        compression_type: Compression,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Generic(GenericResponse::InvalidRequest),
        }
    }

    /// Constructor for a text list response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    /// * list: list of text files available in the server
    #[inline]
    #[must_use]
    pub fn new_text_list_response(
        source_id: NodeId,
        compression_type: Compression,
        list: Vec<String>,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Text(TextResponse::TextList(list)),
        }
    }

    /// Constructor for a text file response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    /// * data: serialized text file
    #[inline]
    #[must_use]
    pub fn new_text_response(
        source_id: NodeId,
        compression_type: Compression,
        data: Vec<u8>,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Text(TextResponse::Text(data)),
        }
    }

    /// Constructor for a media list file response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    /// * list: list of media files available in the server
    #[inline]
    #[must_use]
    pub fn new_media_list_response(
        source_id: NodeId,
        compression_type: Compression,
        list: Vec<String>,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Media(MediaResponse::MediaList(list)),
        }
    }

    /// Constructor for a media file response
    /// * `source_id`: id of the web server that creates the response
    /// * `compression_type`: compression algorithm used for the response's content
    /// * data: serialized media file
    #[inline]
    #[must_use]
    pub fn new_media_response(
        source_id: NodeId,
        compression_type: Compression,
        data: Vec<u8>,
    ) -> ResponseMessage {
        Self {
            source_id,
            compression_type,
            content: Response::Media(MediaResponse::Media(data)),
        }
    }
}
