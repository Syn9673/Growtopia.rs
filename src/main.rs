use protonsdk_variant::*;
use enet;

// import our gamepacket struct
#[path = "./structs/gamepacket.rs"]
mod packet;

use packet::GamePacket;

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

    loop {
        match host.service(0).expect("failed at checking for events") {
            Some(enet::Event::Connect(ref mut peer)) => {
                println!("Peer connected: {:?}", std::time::SystemTime::now());
                let mut packet: GamePacket = GamePacket::new(10000, -1);

                packet.combine(var_fn!("OnConsoleMessage", "This is a test packet that is showed after 10 seconds."))
                    .send(0, peer);

                packet = GamePacket::new(5000, -1);
                packet.combine_ref(var_fn!("OnConsoleMessage", "This is a test packet that is showed after 5 seconds but sent after the 10 second packet."));

                println!("{:?}", packet.data_ref());

                packet.send(0, peer);
            },

            _ => ()
        }
    }
}