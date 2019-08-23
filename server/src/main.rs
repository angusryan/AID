#![allow(unused)]
use std::thread;

//setup function initialises connection with drone
mod setup;
use setup::setup;
use setup::listen;
use setup::connect;

fn main() {
    let locaddr = "127.0.0.1:34254";
    let addr = "127.0.0.1:4242"; //"192.168.10.1:8889";

    let (socket, socket_recv) = setup(locaddr, addr);

    let handle = thread::spawn(|| {
        listen(socket_recv);
    });

    connect(socket, addr);

    handle.join().unwrap();

}
