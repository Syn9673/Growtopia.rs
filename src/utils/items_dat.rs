use std::fs;
use bytes::{BufMut};

#[path = "../structs/extra_bytes.rs"]
mod extra_bytes;

use extra_bytes::ExtraBytes;

pub struct ItemsDat {
    items_dat_hash: u32,
    items_dat: Vec<u8>,
    items_dat_file: Vec<u8>
}

#[allow(dead_code)]
impl ItemsDat {
    pub fn new() -> ItemsDat {
        ItemsDat {
            items_dat_hash: 0,
            items_dat: vec![],
            items_dat_file: fs::read("./src/files/items.dat").unwrap()
        }
    }

    pub fn create(mut self) {
        let mut data: Vec<u8> = vec![]; // create an empty vector of unisgned 8 bit integers
        let types: (u32, u32, u32) = (0x4, 0x10, 0x8);

        data.put_u32_le(types.0);
        data.put_u32_le(types.1);
        data.put_i32_le(-1); // net id
        data.extra(4); // four extra bytes
        data.put_u32_le(types.2);
        data.extra(36); // 36 extra bytes
        data.put_u32_le(self.items_dat_file.len() as u32);
        
        let mut index: usize = 0;

        while index < self.items_dat_file.len() {
            data.push(self.items_dat_file[index]);
            index += 1;
        }

        self.items_dat = data;
    }

    pub fn create_ref(&mut self) {
        let mut data: Vec<u8> = vec![]; // create an empty vector of unisgned 8 bit integers
        let types: (u32, u32, u32) = (0x4, 0x10, 0x8);

        data.put_u32_le(types.0);
        data.put_u32_le(types.1);
        data.put_i32_le(-1); // net id
        data.extra(4); // four extra bytes
        data.put_u32_le(types.2);
        data.extra(36); // 36 extra bytes
        data.put_u32_le(self.items_dat_file.len() as u32);
        
        let mut index: usize = 0;

        while index < self.items_dat_file.len() {
            data.push(self.items_dat_file[index]);
            index += 1;
        }

        self.items_dat = data;
    }

    pub fn hash(mut self) {
        let mut h: u32 = 0x55555555;
        let mut index: usize = 0;

        while index < self.items_dat_file.len() {
            h = (h >> 27) + (h << 5) + self.items_dat_file[index] as u32;
            index += 1;
        };

        self.items_dat_hash = h;
    }

    pub fn hash_ref(&mut self) {
        let mut h: u32 = 0x55555555;
        let mut index: usize = 0;

        while index < self.items_dat_file.len() {
            h = (h >> 27) + (h << 5) + self.items_dat_file[index] as u32;
            index += 1;
        };

        self.items_dat_hash = h;
    }

    pub fn get_items_dat_packet(self) -> Vec<u8> {
        self.items_dat
    }

    pub fn get_items_dat_packet_ref(&self) -> &Vec<u8> {
        &self.items_dat
    }

    pub fn get_hash(self) -> u32 {
        self.items_dat_hash
    }

    pub fn get_hash_ref(&self) -> u32 {
        self.items_dat_hash
    }
}