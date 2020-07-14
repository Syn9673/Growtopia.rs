use std::collections::HashMap;
pub use enet;

// import our gamepacket struct
#[path = "./structs/packets.rs"]
mod packets;

#[path = "./utils/handler.rs"]
mod handler;

#[path = "./utils/items_dat.rs"]
mod items_dat;

use packets::{GamePacket, IncomingPacket};
use handler::Handler;
use items_dat::ItemsDat;

fn main() {
    let mut items_dat: ItemsDat = ItemsDat::new();
    items_dat.hash_ref(); // calculate the hash, uses a reference of "self" to be able to use the "items_dat" variable again
    items_dat.create_ref(); // create items_dat packet

    println!("Got hash from items.dat: {}", items_dat.get_hash_ref());

    let server: enet::Enet = enet::Enet::new().unwrap();
    println!("ENet Server Created");

    // create our address
    let address: enet::Address = enet::Address::new(std::net::Ipv4Addr::new(0, 0, 0, 0), 17091);
    
    // create our host
    let mut host: enet::Host<()> = server.create_host(
        Some(&address),
        1024,
        enet::ChannelLimit::Maximum,
        enet::BandwidthLimit::Unlimited,
    enet::BandwidthLimit::Unlimited).unwrap();

    println!("Now checking for Enet Events");

    loop {
        match host.service(0).expect("failed at checking for events") {
            Some(enet::Event::Connect(ref mut peer)) => {
                println!("Peer connected {:?}", peer);
                let packet: GamePacket = GamePacket::new(0, -1);

                packet.send(0, peer, Some(b"\x01\x00\x00\x00\x00"));
            },

            Some(enet::Event::Receive { sender: ref mut peer, channel_id, ref mut packet }) => {
                let mut received: IncomingPacket = IncomingPacket::new(packet.data().to_vec(), channel_id);
                let handler: Handler = Handler::new(
                    Some(items_dat.get_hash_ref()),
                    Some(items_dat.get_items_dat_packet_ref().to_vec())
                );

                // convert the text packets to strings
                match received.p_type_ref() {
                    2 | 3 => { // handle text and action packets
                        let mut packet_map: HashMap<String, String> = HashMap::new();

                        for i in received.get_string_form_ref().split("\n") {
                            let kv: Vec<&str> = i.split("|").collect();

                            if kv.len() < 2 {
                                // no key value
                                continue;
                            } else {
                                // has key value
                                packet_map.insert(kv[0].to_owned(), kv[1].to_owned());
                            }
                        }

                        if packet_map.contains_key("requestedName") || packet_map.contains_key("tankIDName") {
                            // just logging in
                            handler.on_login(peer, Some(packet_map))
                        } else if packet_map.contains_key("action") {
                            // an action
                            let action: &str = packet_map.get("action").unwrap().as_str();

                            match action {
                                "refresh_item_data" => handler.on_request_items_dat(peer, None),
                                "enter_game" => handler.on_enter_game(peer, None),
                                _ => println!("Unhandled action: {}", action)
                            };
                        } else {};
                    },

                    4 => { // handle tank packets
                        println!("Received tank packet: {:?}", received.unpack_tank_ref());
                    },

                    _ => ()
                }
            },

            _ => ()
        }
    }
}