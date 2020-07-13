use protonsdk_variant::*;
use bytes::{BufMut};

trait ExtraBytes {
    fn extra(&mut self, amt: usize) -> &Self;
}

impl ExtraBytes for Vec<u8> {
    fn extra(&mut self, amt: usize) -> &Self {
        let bytes: &[u8] = &b"\x00".repeat(amt);
        self.put(bytes);

        self
    }
}

pub struct GamePacket {
  data: Vec<u8>
}

impl GamePacket {
    pub fn new(delay: u32, net_id: i32) -> GamePacket {
        let mut data: Vec<u8> = vec![];
        let types: (u32, u32, u32) = (0x4, 0x1, 0x8);

        data.put_u32_le(types.0);
        data.put_u32_le(types.1);
        data.put_i32_le(net_id);
        data.extra(4);
        data.put_u32_le(types.2);
        data.extra(4);
        data.put_u32_le(delay);
        data.extra(32);

        GamePacket {
            data
        }
    }

    pub fn combine(mut self, varlist: VariantFunction) -> Self {
        for i in varlist.to_vec().unwrap() {
            self.data.push(i);
        }

        self
    }

    pub fn combine_ref(&mut self, varlist: VariantFunction) -> &Self {
        for i in varlist.to_vec().unwrap() {
            self.data.push(i);
        }

        self
    }
  
    pub fn data(self) -> Vec<u8> {
        self.data
    }

    pub fn data_mut(mut self) -> Vec<u8> {
        self.data
    }

    pub fn data_ref(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_mut_ref(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn send(self, channel: u8, peer: &mut enet::Peer<()>) {
        peer.send_packet(
            enet::Packet::new(
                self.data_ref(),
                enet::PacketMode::ReliableSequenced
            ).unwrap(),
            channel
        ).unwrap();
    }

    pub fn send_ref(&self, channel: u8, peer: &mut enet::Peer<()>) {
        peer.send_packet(
            enet::Packet::new(
                self.data_ref(),
                enet::PacketMode::ReliableSequenced
            ).unwrap(),
            channel
        ).unwrap();
    }  
}