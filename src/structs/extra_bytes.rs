use bytes::{BufMut};

pub trait ExtraBytes {
    fn extra(&mut self, amt: usize) -> &Self;
}

impl ExtraBytes for Vec<u8> {
    fn extra(&mut self, amt: usize) -> &Self {
        let bytes: &[u8] = &b"\x00".repeat(amt);
        self.put(bytes);

        self
    }
}