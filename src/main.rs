use protonsdk_variant::*;
use enet;

// import our gamepacket struct
#[path = "./structs/packets.rs"]
mod packets;

use packets::{GamePacket, IncomingPacket};

fn main() {
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
                println!("Peer connected: {:?}", std::time::SystemTime::now());
                let packet: GamePacket = GamePacket::new(0, -1);

                packet.send(0, peer, Some(b"\x01\x00\x00\x00\x00"));
            },

            Some(enet::Event::Receive { ref mut sender, channel_id, ref mut packet }) => {
                let received: IncomingPacket = IncomingPacket::new(packet.data().to_vec(), channel_id);
                println!("Received packet type: {}", received.p_type());
            },

            _ => ()
        }
    }
}