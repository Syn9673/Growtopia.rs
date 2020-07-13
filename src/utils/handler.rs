use protonsdk_variant::*;
use std::collections::HashMap;

pub fn on_login(peer: &mut enet::Peer<()>, _: Option<HashMap<String, String>>) {
    let packet: crate::GamePacket = crate::GamePacket::new(0, -1);

    packet.combine(var_fn!("OnConsoleMessage", "Login coming `9soon``."))
        .send(0, peer, None);
}