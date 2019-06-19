mod network;

use std::net::TcpStream;
use std::process::exit;
use crate::network::packet_manager::{PacketManager, init_packet_manager};

fn create_stream(remote_host: &str) -> TcpStream
{
    let stream = TcpStream::connect(remote_host);

    match stream {
        Ok(stream) => {
            println!("Connection successful");
            return stream;
        }
        Err(_) => {
            println!("Error trying to connect remote host");
            exit(84);
        }
    }
}

fn main()
{
    let stream = create_stream("34.252.21.81:5555");
    let mut packet_manager: PacketManager = init_packet_manager(stream);

    packet_manager.read_packet();
}