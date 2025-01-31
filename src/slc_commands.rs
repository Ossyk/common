use bincode::{Decode, Encode};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use wg_2024::{network::NodeId, packet::Packet};

// Commands sent by the Simulation Controller to a Client
#[derive(Debug, Clone)]
pub enum ClientCommand {
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    SendChatText(String, NodeId, NodeId), // text, client_id, chat_server_id
    ConnectToChatServer(NodeId),          // chat_server_id
    AskListOfFiles(NodeId),               // chat_server_id
    AskServersTypes,
    RequestFile(String, NodeId), // file_name, server_id
    Shortcut(Packet),
}

// Commands sent by the Simulation Controller to a Server
#[derive(Debug, Clone)]
pub enum ServerCommand {
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    Shortcut(Packet),
}

// Command sent by a Client to the Simulation Controller
#[derive(Debug, Clone, PartialEq)]
pub enum ClientEvent {
    PacketSent(Packet),
    Shortcut(Packet),
    ClientsConnectedToChatServer(Vec<NodeId>),
    ListOfFiles(Vec<String>, NodeId), // list of files, chat_server_id
    FileFromClient(Vec<Vec<u8>>, NodeId),   // file content (first vec is html, others are media), server_id (maybe client_id)
    ServersTypes(HashMap<NodeId, ServerType>), // server_id, server_type
    WrongClientId,
    UnsupportedRequest,
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
