use std::net::UdpSocket;
use std::str::from_utf8;

fn main() {
    let locaddr = "127.0.0.1:4242";
    let addr = "127.0.0.1:34254";

    println!("Local Address: {}\nAddress: {}", locaddr, addr);

    let socket = UdpSocket::bind(locaddr).expect("couldn't bind to address");

    let mut buf = [0; 16];

    let (no_bytes, _src_addr) = socket.recv_from(&mut buf).expect("didn't receive data");

    let filled_buf = &mut buf[..no_bytes];

    let recv_msg = from_utf8(filled_buf).expect("couldn't convert to string");

    println!("{}", recv_msg);

    let send_msg = b"ok";

    socket.send_to(send_msg, addr).expect("couldn't send data");
}
