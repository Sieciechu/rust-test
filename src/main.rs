use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    eprintln!("stream = {:#?}", stream);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}