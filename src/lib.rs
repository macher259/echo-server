use std::net::SocketAddr;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

pub struct ServerTCP {
    listener: TcpListener,
}

impl ServerTCP {
    pub async fn new(address: String, port: u16) -> io::Result<ServerTCP> {
        let listener = TcpListener::bind((&address[..], port)).await?;

        println!("Started listening on {}.", address);
        println!("Use ctrl+c to exit.");

        Ok(ServerTCP { listener })
    }

    pub async fn run(&self) {
        loop {
            let accept = self.listener.accept().await;

            if accept.is_err() {
                eprintln!("Error on accept: {}", accept.unwrap_err());
                continue;
            }

            let (socket, address) = accept.unwrap();

            tokio::spawn(async move {
                process_socket(socket, address).await;
            });
        }
    }
}

async fn process_socket(mut socket: TcpStream, address: SocketAddr) {
    println!("{}: Connected.", address);

    let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("{}: Connection with the socket closed successfully.", address);
                return;
            }
            Ok(n) => {
                let tmp = socket.write_all(&buffer[0..n]).await;
                if tmp.is_err() {
                    eprintln!("{}: Error while writing to the socket: {}.", address, tmp.unwrap_err());
                    return;
                }
            }
            Err(e) => {
                eprintln!("{}: Error while reading the socket: {}.", address, e);
                return;
            }
        }
    }
}