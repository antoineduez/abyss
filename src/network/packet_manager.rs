use std::net::TcpStream;
use crate::network::packet::*;
use std::io::Read;

pub struct PacketManager {
    pub packet: Packet,
    stream: TcpStream,
    header: u16,
    buffer: Vec<u8>,
}

impl PacketManager {
    pub fn read_packet(&mut self) -> () {
        let mut header = [0 as u8; 2];
        let mut octet = [0 as u8];
        let mut i = 0;

        self.stream.read(&mut header);
        self.header = ((header[0] as u16) << 8) | header[1] as u16;
        self.packet.packet_id = (self.header >> 2) as u8;
        self.packet.length_type = (self.header & 3) as u8;

        while i < self.packet.length_type {
            self.stream.read(&mut octet);
            self.packet.length = (self.packet.length << 8) | octet[0] as u32;
            i += 1;
        }

        println!("Header: {:016b}", self.header);
        println!("Packet_ID: {}", self.packet.packet_id);
        println!("Length: {}", self.packet.length);
    }
}

pub fn init_packet_manager(stream: TcpStream) -> PacketManager
{
    let packet_manager: PacketManager = PacketManager {
        stream: stream,
        packet: init_packet(),
        header: 0,
        buffer: vec![0],
    };

    return packet_manager;
}