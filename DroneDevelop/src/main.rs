#![allow(unused)]
use std::fs;
use std::{thread, time};
use std::net::UdpSocket;
use std::str::from_utf8;
use std::io;

fn main() {
    let local_address = "192.168.10.2:8889";
    let address = "192.168.10.1:8889";

//Initialise Connection
    /* Bind Socket */
    let socket = UdpSocket::bind(local_address).expect("couldn't bind to address");

    println!("Success");
//
//Start SDK Mode
    /* Send */
    {
        let message = b"command";

        socket.send_to(message, address).expect("couldn't send data");
    }

    /* Receive */
    {
        let mut buffer = [0; 16];

        let (no_bytes, src_address) = socket.recv_from(&mut buffer).expect("didn't receive data");

        let mut filled_buffer = &mut buffer[..no_bytes];

        println!("Received: {}", from_utf8(&filled_buffer).expect("couldn't convert to string"));
    }
//
//Take user input
    let mut exit = false;
    while !exit {

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                match input.as_ref() {
                    "exit\n" => {
                        println!("Exiting...");
                        exit = true;
                    }
                    "help\n" => {
                        let help = fs::read_to_string("help.txt").expect("Something went wrong reading the file");
                        print!("{}", help);
                    }
                    "takeoff\n" => {
                        /* Send */
                        socket.send_to(b"takeoff", address).expect("couldn't send data");

                        /* Receive */
                        let mut buffer = [0; 16];

                        let (no_bytes, src_address) = socket.recv_from(&mut buffer).expect("didn't receive data");

                        let mut filled_buffer = &mut buffer[..no_bytes];

                        println!("Received: {}", from_utf8(&filled_buffer).expect("couldn't convert to string"));
                    }
                    "land\n" => {
                        /* Send */
                        socket.send_to(b"land", address).expect("couldn't send data");

                        /* Receive */
                        let mut buffer = [0; 16];

                        let (no_bytes, src_address) = socket.recv_from(&mut buffer).expect("didn't receive data");

                        let mut filled_buffer = &mut buffer[..no_bytes];

                        println!("Received: {}", from_utf8(&filled_buffer).expect("couldn't convert to string"));
                    }
                    _ => println!("Type help for list of commands"),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
