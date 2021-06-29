use std::net::TcpListener;
use std::io::{Read, Write, BufReader, BufRead};

pub fn make_socket() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap_or_else(|_| {
        panic!("Couldn't bind to port");
    });

    for connection in server.incoming() {
        println!("New connection!");

        let mut connection = match connection {
            Err(_) => {
                println!("Couldn't connect to client.");
                continue;
            },
            _ => connection.unwrap()
        };

        let mut string = String::new();
        let mut reader = BufReader::new(&connection);
        if let Err(_) = reader.read_line(&mut string) {
            println!("Unable to read from connection.");
            continue;
        }

        println!("From client: {}", string);

        let response = "Hello!";
        if let Err(_) = connection.write_all(response.as_bytes()) {
            println!("Unable to write to connection.");
            continue;
        }

        println!("Connection closed.");
    }
}