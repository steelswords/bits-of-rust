use std::net::{TcpListener, TcpStream};
use std::io::Read;

/*
 * tcpprint.rs
 * Author: Tristan Andrus
 *
 */

fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = String::with_capacity(200);
    stream.try_clone()?.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9123")?;

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    Ok(())
}
