//Start Video Stream
    /* Send */
    {
        let msg = b"streamon";

        socket.send_to(msg, addr).expect("couldn't send data");
    }

    /* Receive */
    {
        let mut buf = [0; 16];

        let (no_bytes, src_addr) = socket.recv_from(&mut buf).expect("didn't receive data");

        let mut filled_buf = &mut buf[..no_bytes];

        println!("Received: {}", from_utf8(&filled_buf).expect("couldn't convert to string"));
    }

    /* Start Stream Thread */
    let streamaddr = "0.0.0.0:8890";
    let handle = thread::spawn(|| {
        let socket = UdpSocket::bind(streamaddr).expect("couldn't bind to address");

        println!("Success");

        /* Receive */

    });


    //Stop Stream Thread
        handle.join().unwrap();

//

//Stop Emergency Thread
    emergencyHandle.join().unwrap();

//
