#![allow(unused)]
use std::net::UdpSocket;
use std::str::from_utf8;

//Initialises connection with drone
pub fn setup(locaddr: &str, addr: &str) -> (UdpSocket, UdpSocket) {

    println!("Local Address: {}\nAddress: {}", locaddr, addr);

    let socket = UdpSocket::bind(locaddr).expect("couldn't bind to address");

    println!("Success");
    let socket_clone = socket.try_clone().expect("couldn't clone the socket");
    return (socket, socket_clone);
}

pub fn listen(socket: UdpSocket) {
    loop {
        let mut buf = [0; 16];

        let (no_bytes, src_addr) = socket.recv_from(&mut buf).expect("didn't receive data");

        let mut filled_buf = &mut buf[..no_bytes];

        println!("Received: {}", from_utf8(&filled_buf).expect("couldn't convert to string"));
    }
}

pub fn connect(socket: UdpSocket, addr: &str) {

    let msg = b"command";

    socket.send_to(msg, addr).expect("couldn't send data");

}
