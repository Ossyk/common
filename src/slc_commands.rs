/*!
    This module contains the types used to implement communication between
    clients/servers and the simulation controller
*/

use bincode::{Decode, Encode};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use wg_2024::{network::NodeId, packet::Packet};

/// Contains both a text file and the linked media files
/// Web client sends this message to the scl when all the necessary files have been retrieved from the network
#[derive(Debug, Clone, PartialEq)]
pub struct TextMediaResponse {
    /// pair with (filenme, file content)
    html_file: (String, Vec<u8>),
    /// array of pairs (filename, file content)
    media_files: Vec<(String, Vec<u8>)>,
}

impl TextMediaResponse {
    /// Constructor for `TextMediaResponse`
    /// * `html_file`: pair (text filename, text file content)
    /// * `media_files`: array of pairs (media filename, media file content)
    #[inline]
    #[must_use]
    pub fn new(html_file: (String, Vec<u8>), media_files: Vec<(String, Vec<u8>)>) -> Self {
        Self {
            html_file,
            media_files,
        }
    }

    /// Getter of the pair `html_file`
    #[inline]
    #[must_use]
    pub fn get_html_file(&self) -> &(String, Vec<u8>) {
        &self.html_file
    }

    /// Getter of the array of pairs `media_files`
    #[inline]
    #[must_use]
    pub fn get_media_files(&self) -> &Vec<(String, Vec<u8>)> {
        &self.media_files
    }
}

/// Identifies a command sent from scl to a client
pub trait ClientCommand {}
/// Identifies a command sent from scl to a server
pub trait ClientEvent {}

/// Types of commands sent to web client
#[derive(Debug, Clone)]
pub enum WebClientCommand {
    /// Ask a client to add a neighbor with the given ID and the given channel
    AddSender(NodeId, Sender<Packet>),
    /// Ask a client to remove the neighbor with the given ID
    RemoveSender(NodeId),
    /// Ask a client to discover the server types in the network
    AskServersTypes,
    /// Ask a client to retrieve the available files in the server with the given ID
    AskListOfFiles(NodeId),
    /// Ask a client to retrieve the file identified by the given string from the node identified with the given ID
    RequestFile(String, NodeId), // file_name, server_id
    /// Scl shortcuts a packet to the client
    Shortcut(Packet),
}

impl ClientCommand for WebClientCommand {}

/// Events that a web client can send to the scl
#[derive(Debug, Clone, PartialEq)]
pub enum WebClientEvent {
    /// log that the given packet has been sent
    PacketSent(Packet),
    /// inform scl that the given packet has to be shortcut
    Shortcut(Packet),
    /// communicate to scl the servers type of the servers in the network
    ServersTypes(HashMap<NodeId, ServerType>), // server_id, server_type
    /// communicate to scl the files available from the server identified by the given ID
    ListOfFiles(Vec<String>, NodeId), // list of files, chat_server_id
    /// send a `TextMediaResponse` whose text file comes from the server identified from the given ID
    FileFromClient(TextMediaResponse, NodeId), // file content (first vec is html, others are media), server_id (maybe client_id)
    /// inform that client received an unsupported request
    UnsupportedRequest,
}

impl ClientEvent for WebClientEvent {}

/// Types of commands sent to chat client
#[derive(Debug, Clone)]
pub enum ChatClientCommand {
    /// Ask a client to add a neighbor with the given ID and the given channel
    AddSender(NodeId, Sender<Packet>),
    /// Ask a client to remove the neighbor with the given ID
    RemoveSender(NodeId),
    /// Ask a client to discover the server types in the network
    AskServersTypes,
    /// Ask a client to connect to the server identified by the given ID
    ConnectToChatServer(NodeId), // chat_server_id
    /// Ask a client to send a message (first parameter) to anothe client (second parameter) through a chat server (third parameter)
    SendChatText(String, NodeId, NodeId), // text, client_id, chat_server_id
    /// Scl shortcuts a packet to the client
    Shortcut(Packet),
}

impl ClientCommand for ChatClientCommand {}

/// Events that a chat client can send to the scl
#[derive(Debug, Clone)]
pub enum ChatClientEvent {
    /// log that the given packet has been sent
    PacketSent(Packet),
    /// inform scl that the given packet has to be shortcut
    Shortcut(Packet),
    /// communicate to scl the servers type of the servers in the network
    ServersTypes(HashMap<NodeId, ServerType>), // server_id, server_type
    /// communicate to scl all the clients connected to a chat server
    ClientsConnectedToChatServer(NodeId, Vec<NodeId>),
    /// communicate to scl that a new message has arrived from the node with the given ID
    NewMessageFrom(NodeId), // client_id, maybe add also chat_server_id
    /// inform that client received an unsupported request
    UnsupportedRequest,
}

impl ClientEvent for ChatClientEvent {}

/// Commands sent by the scl to a Server
#[derive(Debug, Clone)]
pub enum ServerCommand {
    /// Ask a server to add a neighbor with the given ID and the given channel
    AddSender(NodeId, Sender<Packet>),
    /// Ask a server to remove the neighbor with the given ID
    RemoveSender(NodeId),
    /// Scl shortcuts a packet to the server
    Shortcut(Packet),
}

/// Command sent by a Server to the scl
#[derive(Debug, Clone, PartialEq)]
pub enum ServerEvent {
    /// log that the given packet has been sent
    PacketSent(Packet),
    /// inform scl that the given packet has to be shortcut
    ShortCut(Packet),
}

/// Identifies the type of a server inside the network
#[derive(Debug, Clone, Copy, Encode, Decode, Hash, PartialEq, Eq)]
pub enum ServerType {
    ChatServer,
    FileServer,
    MediaServer,
}
