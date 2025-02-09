/*!
    C++Enjoyers: Shared code crate used by clients, servers and scl
*/

#![warn(clippy::pedantic)]

use crossbeam_channel::{Receiver, Sender};
use slc_commands::{ClientCommand, ClientEvent, ServerCommand, ServerEvent, ServerType};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub mod networking;
pub mod ring_buffer;
pub mod slc_commands;
pub mod web_messages;

/// Needed by a node to act as a server in the network
pub trait Server
where
    Self: Send,
{
    /// Constructor of a server
    /// * id: ID if the new server
    /// * `controller_send`: channel to send events to scl
    /// * `controller_recv`: channel to receive commands from scl
    /// * `packet_recv`: channel to receive packets from other nodes
    /// * `packet_send`: map of channels to talk to a specific neighbor ID
    fn new(
        id: NodeId,
        controller_send: Sender<ServerEvent>,
        controller_recv: Receiver<ServerCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    /// * Core function that put the server in "running mode"
    fn run(&mut self);
}

/// Needed by a node to act as a client in the network
/// * <T: ClientCommand>: type of client command that the client can accept
/// * <U: ClientEvent>: type of client events that the client can send
pub trait Client
where
    Self: Send,
{
    type T: ClientCommand;
    type U: ClientEvent;
    /// Constructor of a client
    /// * id: ID if the new server
    /// * `controller_send`: channel to send events to scl
    /// * `controller_recv`: channel to receive commands from scl
    /// * `packet_recv`: channel to receive packets from other nodes
    /// * `packet_send`: map of channels to talk to a specific neighbor ID
    fn new(
        id: NodeId,
        controller_send: Sender<Self::U>,
        controller_recv: Receiver<Self::T>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    /// * Core function that put the client in "running mode"
    fn run(&mut self);
}
