#![allow(unused)]
/*!
    This module contains the Flooder trait which is common to both clients and servers in the network
*/

use crossbeam_channel::Sender;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{FloodRequest, NodeType, Packet};

/// Error that is generated during the handling of a flood request
#[derive(Debug)]
pub struct FloodingError;
impl std::fmt::Display for FloodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flooding Error")
    }
}
impl std::error::Error for FloodingError {}

/// Gives the ability to handle a flood request in the network
// ! NOTE we can have the function take a packet but then we would need another enum match
pub trait Flooder {
    /// specifies the node's type that is implementing the trait
    // associated constants (looks like a good idea)
    const NODE_TYPE: NodeType;

    /// retrieves the ID of the node
    fn get_id(&self) -> NodeId;
    /// retrieves the neighbors of the node
    fn get_neighbours(&self) -> impl ExactSizeIterator<Item = (&NodeId, &Sender<Packet>)>;
    /// checks if the node has already seen a flood
    /// * flood_id: ID of the flood to check
    fn has_seen_flood(&self, flood_id: (NodeId, u64)) -> bool;
    /// insert the flood_id insde the history of floods that have been already seen
    /// * flood_id: ID of the flood to store
    fn insert_flood(&mut self, flood_id: (NodeId, u64));
    /// logs to scl that the packet p has been sent
    /// * p: packet to be logged
    fn send_to_controller(&self, p: Packet);

    /// Provided method that handles an incoming flood request
    /// # Errors
    ///
    /// Will return Err if the flood reponse cannot be sent
    fn handle_flood_request(
        &mut self,
        routing_header: &SourceRoutingHeader,
        sid: u64,
        flood_r: &mut FloodRequest,
    ) -> Result<(), FloodingError> {
        let sender_id: NodeId = flood_r
            .path_trace
            .last()
            .map_or(flood_r.initiator_id, |(id, t)| *id);
        let flood_tuple_id = (flood_r.initiator_id, flood_r.flood_id);

        flood_r.increment(self.get_id(), Self::NODE_TYPE);

        let mut it = self.get_neighbours();
        if self.has_seen_flood(flood_tuple_id) || it.len() <= 1 {
            let mut new_packet: Packet = flood_r.generate_response(sid);
            new_packet.routing_header.increase_hop_index();
            let next_hop: NodeId = new_packet
                .routing_header
                .current_hop()
                .expect("If this panics the wg code is borken");
            match it.find(|(id, c)| **id == next_hop) {
                Some((_, c)) => {
                    c.send(new_packet.clone());
                    self.send_to_controller(new_packet);
                    Ok(())
                }
                None => Err(FloodingError),
            }
        } else {
            it.for_each(|(id, c)| {
                if *id != sender_id {
                    let new_packet =
                        Packet::new_flood_request(routing_header.clone(), sid, flood_r.clone());
                    c.send(new_packet.clone());
                    self.send_to_controller(new_packet);
                }
            });
            self.insert_flood(flood_tuple_id);
            Ok(())
        }
    }
}
