use std::process;
use echo_server::ServerTCP;
use tokio::signal;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 2137;

#[tokio::main]
async fn main() {
    let server = ServerTCP::new(HOSTNAME.to_string(), PORT).await.unwrap_or_else(|e| {
        eprintln!("Couldn't connect: {}.", e);
        process::exit(1);
    });

    tokio::spawn(async {
       signal::ctrl_c().await.expect("Failed to listen to ctrl-c.");
        println!("Received QUIT signal.");
        process::exit(0);
    });
    server.run().await;
}
