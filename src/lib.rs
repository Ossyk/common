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

pub trait Server {
    fn new(
        id: NodeId,
        controller_send: Sender<ServerEvent>,
        controller_recv: Receiver<ServerCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self);
}

pub trait Client {
    fn new(
        id: NodeId,
        controller_send: Sender<ClientEvent>,
        controller_recv: Receiver<ClientCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self);
}
