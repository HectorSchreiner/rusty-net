use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use server::Server;

mod server;

fn main() {
    let mut server = Server::new();
    server.display_logo();
    server.display_local_addresses();
    server.listener();
}
