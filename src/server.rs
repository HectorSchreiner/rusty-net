use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 { return; }
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));

                    // Send a command to the client
                    let command = "write_to_file:Hello from server!";
                    stream.write(command.as_bytes()).unwrap();
                },
                Err(_) => return,
            }
        }
    }

    pub fn listener(&self) -> std::io::Result<()> {

        let listener = TcpListener::bind("0.0.0.0:8080")?;
        println!("Server listening on all interfaces on port 8080");

       
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || Self::handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

    pub fn display_local_addresses(&self) -> std::io::Result<()>{
        let local_ips: Vec<Ipv4Addr> = match local_ip_address::list_afinet_netifas() {
            Ok(interfaces) => interfaces
                .into_iter()
                .filter_map(|(_, ip)| {
                    if let std::net::IpAddr::V4(ipv4) = ip {
                        if !ipv4.is_loopback() {
                            Some(ipv4)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
            Err(_) => Vec::new(),
        };

        if local_ips.is_empty() {
            println!("Couldn't detect any local IP addresses.");
        } else {
            println!("Detected local IP addresses:");
            for ip in local_ips {
                println!("  - {}", ip);
            }
        }
        
        println!("Use one of these IP addresses for clients to connect");

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.green} {msg}")
                .unwrap()
        );

        let listener = TcpListener::bind("0.0.0.0:8080")?;
        pb.set_message("Waiting for clients");
        for stream in listener.incoming() {
            pb.tick();
            match stream {
                Ok(stream) => {
                    pb.finish_with_message("Client connected!");
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || Self::handle_client(stream));
                    
                    // Reset the progress bar for the next client
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                            .template("{spinner:.green} {msg}")
                            .unwrap()
                    );
                    pb.set_message("Waiting for more clients");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
            thread::sleep(Duration::from_millis(100)); // Small delay to control spinner speed
        }
        Ok(())
    }

    pub fn display_logo(&self) {
        println!(r#"
 _______  _______           _______           _______  _        _______ _________
 (  ____ \(  ____ \|\     /|(  ___  )|\     /|(  ____ \( (    /|(  ____ \\__   __/
 | (    \/| (    \/| )   ( || (   ) || )   ( || (    \/|  \  ( || (    \/   ) (   
 | (_____ | |      | (___) || |   | || |   | || (_____ |   \ | || (__       | |   
 (_____  )| |      |  ___  || |   | || |   | |(_____  )| (\ \) ||  __)      | |   
       ) || |      | (   ) || |   | || |   | |      ) || | \   || (         | |   
 /\____) || (____/\| )   ( || (___) || (___) |/\____) || )  \  || (____/\   | |   
 \_______)(_______/|/     \|(_______)(_______)\_______)|/    )_)(_______/   )_(  
                                                                      
        "#);
    }
}