use bincode::{Decode, Encode};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use wg_2024::{network::NodeId, packet::Packet};

#[derive(Debug, Clone)]
pub struct TextMediaResponse {
    html_file: (String, Vec<u8>),
    media_files: Vec<(String, Vec<u8>)>,
}

impl TextMediaResponse {
    pub fn new(html_file: (String, Vec<u8>), media_files: Vec<(String, Vec<u8>)>) -> Self {
        Self {
            html_file,
            media_files,
        }
    }
}

pub trait ClientCommand {}
pub trait ClientEvent {}

#[derive(Debug, Clone)]
pub enum WebClientCommand {
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    AskServersTypes,
    AskListOfFiles(NodeId), // chat_server_id
    RequestFile(String, NodeId), // file_name, server_id
    Shortcut(Packet),
}

impl ClientCommand for WebClientCommand {}

#[derive(Debug, Clone, PartialEq)]
pub enum WebClientEvent {
    PacketSent(Packet),
    Shortcut(Packet),
    ServersTypes(HashMap<NodeId, ServerType>), // server_id, server_type
    ListOfFiles(Vec<String>, NodeId), // list of files, chat_server_id
    FileFromClient(Vec<Vec<u8>>, NodeId), // file content (first vec is html, others are media), server_id (maybe client_id)
    UnsupportedRequest,
}

impl ClientEvent for WebClientEvent {}

#[derive(Debug, Clone)]
pub enum ChatClientCommand {
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    AskServersTypes,
    ConnectToChatServer(NodeId), // chat_server_id
    SendChatText(String, NodeId, NodeId), // text, client_id, chat_server_id
    Shortcut(Packet),
}

impl ClientCommand for ChatClientCommand {}

#[derive(Debug, Clone)]
pub enum ChatClientEvent {
    PacketSent(Packet),
    Shortcut(Packet),
    ServersTypes(HashMap<NodeId, ServerType>), // server_id, server_type
    ClientsConnectedToChatServer(Vec<NodeId>),
    NewMessageFrom(NodeId), // client_id, maybe add also chat_server_id
    UnsupportedRequest,
}

impl ClientEvent for ChatClientEvent {}

// Commands sent by the Simulation Controller to a Server
#[derive(Debug, Clone)]
pub enum ServerCommand {
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    Shortcut(Packet),
}

// Command sent by a Server to the Simulation Controller
#[derive(Debug, Clone, PartialEq)]
pub enum ServerEvent {
    PacketSent(Packet),
    ShortCut(Packet),
}

#[derive(Debug, Clone, Copy, Encode, Decode, Hash, PartialEq, Eq)]
pub enum ServerType {
    ChatServer,
    FileServer,
    MediaServer,
}
