use std::collections::HashMap;
use crossbeam_channel::Sender;
use wg_2024::{network::NodeId, packet::Packet};

// Commands sent by the Simulation Controller to a Client
#[derive(Debug, Clone)]
pub enum ClientCommand {
    AddSender(NodeId, Sender<Packet>),
    SendChatText(String, NodeId, NodeId), // text, client_id, chat_server_id
    ConnectToChatServer(NodeId),          // chat_server_id
    AskListOfFiles(NodeId),               // chat_server_id
    AskServersTypes,
    RequestFile(String, NodeId), // file_name, server_id
}

// Commands sent by the Simulation Controller to a Server
#[derive(Debug, Clone)]
pub enum ServerCommand {
    AddSender(NodeId, Sender<Packet>),
}

// Command sent by a Client to the Simulation Controller
#[derive(Debug, Clone)]
pub enum ClientEvent {
    PacketSent(Packet),
    Shortcut(Packet),
    ClientsConnectedToChatServer(Vec<NodeId>),
    ListOfFiles(Vec<String>, NodeId), // list of files, chat_server_id
    FileFromClient(String, NodeId),   // file content, server_id (maybe client_id)
    ServersTypes(HashMap<NodeId, ServerType>), // server_id, server_type
    WrongClientId,
    UnsupportedRequest,
}

// Command sent by a Server to the Simulation Controller
#[derive(Debug, Clone)]
pub enum ServerEvent {
    PacketSent(Packet),
}

#[derive(Debug, Clone)]
pub enum ServerType {
    ChatServer,
    FileServer,
    MediaServer,
}
