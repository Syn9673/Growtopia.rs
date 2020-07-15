use bytes::{BufMut};

#[path = "../structs/extra_bytes.rs"]
mod extra_bytes;

use extra_bytes::ExtraBytes;

#[allow(dead_code)]
pub struct World {
    width: u16,
    height: u16,
    tiles: Vec<u8>,
    name: String
}

#[allow(dead_code)]
impl World {
    pub fn packet(self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let x: u16 = self.width;
        let y: u16 = self.height;
        let square: u16 = x * y;

        data.put_u32_le(4);
        data.put_u32_le(4);
        data.extra(8);
        data.put_uint_le(8, 1);
        data.extra(49);
        data.put_uint_le(self.name.len() as u64, 1);
        data.extra(1);
        data.put(self.name.as_bytes());
        data.put_uint_le(x as u64, 1);
        data.extra(3);
        data.put_uint_le(y as u64, 1);
        data.extra(3);
        data.put_uint_le(square as u64, 2);
        data.extra(2);

        let mut index: usize = 0;

        while index < square as usize {
            if index >= 3000 {
                data.put_uint_le(2, 2);
                data.put_uint_le(14, 2);
            } else {
                data.extra(4);
            }

            data.put_uint_le(0x00000000, 4);

            index += 1;
        }
        
        data
    }

    pub fn packet_ref(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let x: u16 = self.width;
        let y: u16 = self.height;
        let square: u16 = x * y;

        data.put_u32_le(4);
        data.put_u32_le(4);
        data.extra(8);
        data.put_uint_le(8, 1);
        data.extra(49);
        data.put_uint_le(self.name.len() as u64, 1);
        data.extra(1);
        data.put(self.name.as_bytes());
        data.put_uint_le(x as u64, 1);
        data.extra(3);
        data.put_uint_le(y as u64, 1);
        data.extra(3);
        data.put_uint_le(square as u64, 2);
        data.extra(2);

        let mut index: usize = 0;

        while index < square as usize {
            if index >= 3000 {
                data.put_uint_le(2, 2);
                data.put_uint_le(14, 2);
            } else {
                data.extra(4);
            }

            data.put_uint_le(0x00000000, 4);

            index += 1;
        }
        
        data
    }

    pub fn new(name: &str, width: u16, height: u16) -> World {
        World {
            width,
            height,
            tiles: vec![],
            name: String::from(name)
        }
    }
}