#![allow(unused)]
use std::{thread, time};
use std::net::UdpSocket;
use std::str::from_utf8;
use std::io;

fn main() {
    let localAddress = "192.168.10.2:8889";
    let emergencyAddress = "192.168.10.2:8888";
    let address = "192.168.10.1:8889";

    println!("Local Address: {}\nAddress: {}", localAddress, address);
//Initialise Connection
    /* Bind Socket */
    let socket = UdpSocket::bind(localAddress).expect("couldn't bind to address");

    println!("Success");

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
    /* Start Emergency Stop Thread */
    let emergencyHandle = thread::spawn(|| {
        let emergencySocket = UdpSocket::bind(emergencyAddress).expect("couldn't bind to address");

        println!("Success");

        let message = b"emergency";
        let mut input = String::new();
        while 1 {
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    println!("{} bytes read", n);
                    println!("{}", input);
                    match input {
                        //emergency landing
                        "e" => {
                            /* Send */
                            {
                                emergencySocket.send_to(message, emergencyAddress).expect("couldn't send data");
                            }

                            /* Receive */
                            {
                                let mut buffer = [0; 16];

                                let (no_bytes, src_address) = socket.recv_from(&mut buffer).expect("didn't receive data");

                                let mut filled_buffer = &mut buffer[..no_bytes];

                                println!("Received: {}", from_utf8(&filled_buffer).expect("couldn't convert to string"));
                            }
                        }
                        //exit
                        "exit" => {
                            println!("Exiting...")
                            println!("Landed?")
                            /* Send */
                            {
                                emergencySocket.send_to(message, emergencyAddress).expect("couldn't send data");
                            }


                        }
                    }
                }
                Err(error) => println!("error: {}", error),
            }

            if  {
                /* Send */
                {
                    emergencySocket.send_to(message, emergencyAddress).expect("couldn't send data");
                }

                /* Receive */
                {
                    let mut buffer = [0; 16];

                    let (no_bytes, src_address) = socket.recv_from(&mut buffer).expect("didn't receive data");

                    let mut filled_buffer = &mut buffer[..no_bytes];

                    println!("Received: {}", from_utf8(&filled_buffer).expect("couldn't convert to string"));
                }
            }
            else if  {

            }
            else {

            }
        }

    });
//Takeoff
    /* Send */
    {
        let message = b"takeoff";

        socket.send_to(message, address).expect("couldn't send data");
    }

    /* Receive */
    {
        let mut buffer = [0; 16];

        let (no_bytes, src_address) = socket.recv_from(&mut buffer).expect("didn't receive data");

        let mut filled_buffer = &mut buffer[..no_bytes];

        println!("Received: {}", from_utf8(&filled_buffer).expect("couldn't convert to string"));
    }

    /* Wait */
    thread::sleep(time::Duration::from_millis(200));
//
//Land
    /* Send */
    {
        let message = b"land";

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
}
