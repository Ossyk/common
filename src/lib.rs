#![warn(clippy::pedantic)]

use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub mod networking;
pub mod ring_buffer;
pub mod slc_commands;
pub mod web_messages;

// TODO add correct types to parameters
pub trait Server {
    fn new(
        id: NodeId,
        controller_send: Sender<u8>,
        controller_recv: Receiver<u8>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self);
}

// TODO add correct types to parameters
pub trait Client {
    fn new(
        id: NodeId,
        controller_send: Sender<u8>,
        controller_recv: Receiver<u8>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self);
}
