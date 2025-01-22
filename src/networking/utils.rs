#![allow(unused)]

use crossbeam_channel::{select, select_biased, unbounded, Receiver, Sender};
//use log::{error, info, trace, warn, LevelFilter};
use wg_2024::config::Config;
use wg_2024::drone::Drone;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::*;


// TODO is this useful? or use another way to send packets (due to simulation controller)
pub fn send_packet(packet: Packet, c: &Sender<Packet>) {}

pub fn send_nack(mut packet: Packet, nack_type: NackType, c: &Sender<Packet>) {}
