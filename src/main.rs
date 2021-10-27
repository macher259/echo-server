use std::process;
use echo_server::ServerTCP;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 2137;

#[tokio::main]
async fn main() {
    let server = ServerTCP::new(HOSTNAME.to_string(), PORT).await.unwrap_or_else(|e| {
        eprintln!("Couldn't connect: {}.", e);
        process::exit(1);
    });

    server.run().await;
}
