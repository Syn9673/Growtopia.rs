#[derive(Debug)]
pub struct TankPacket {
    p_type: Option<u8>,
    tank_type: Option<u8>,
    net_id: Option<i16>
}

impl TankPacket {
    pub fn new(p_type: Option<u8>, tank_type: Option<u8>, net_id: Option<i16>) -> TankPacket {
        TankPacket {
            p_type,
            tank_type,
            net_id
        }
    }
}