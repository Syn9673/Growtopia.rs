use std::fs;

pub struct ItemsDat {
    items_dat_hash: u32
}

impl ItemsDat {
    pub fn new() -> ItemsDat {
        ItemsDat {
            items_dat_hash: 0
        }
    }

    pub fn hash(mut self) {
        let items_dat: Vec<u8> = fs::read("./src/files/items.dat").unwrap();
        let mut h: u32 = 0x55555555;

        for i in items_dat {
            //h = ((h.wrapping_shr(27)).wrapping_add(h.wrapping_shl(5))).wrapping_add(i as u32);
            h = ((h >> 27) + (h << 5)) + i as u32;
        }

        self.items_dat_hash = h;
    }

    pub fn hash_ref(&mut self) {
        let items_dat: Vec<u8> = fs::read("./src/files/items.dat").unwrap();
        let mut h: u32 = 0x55555555;

        for i in items_dat {
            h = ((h.wrapping_shr(27)).wrapping_add(h.wrapping_shl(5))).wrapping_add(i as u32);
        }

        self.items_dat_hash = h;
    }

    pub fn get_hash(self) -> u32 {
        self.items_dat_hash
    }

    pub fn get_hash_ref(&self) -> u32 {
        self.items_dat_hash
    }
}