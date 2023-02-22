use clap::Parser;
use std::net::UdpSocket;
use chrono::Local;

/// Extremely simple command line to create a single listen-only UDP socket.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// The local address to bind the UDP socket to, e.g., "127.0.0.1:0" or "0.0.0.0:1234"
    bind_addr: String,
}

fn main() {
    let args = Args::parse();
    let udp_socket = UdpSocket::bind(args.bind_addr).expect("couldn't bind to address");
    loop {
        let mut buf = [0u8; 256];
        let (number_of_bytes, src_addr) = udp_socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf = &mut buf[..number_of_bytes];
        println!("[{}] Received {} bytes from {}: {:?}",
                 Local::now(), number_of_bytes, src_addr, filled_buf.iter().map(|byte| *byte as char).collect::<Vec<char>>());
    }
}
