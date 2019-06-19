pub struct Packet {
    pub packet_id: u8,
    pub length_type: u8,
    pub length: u32,
}

pub fn init_packet() -> Packet
{
    let packet: Packet = Packet {
        packet_id: 0,
        length_type: 0,
        length: 0,
    };

    return packet;
}