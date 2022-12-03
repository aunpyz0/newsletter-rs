use std::net::TcpListener;

use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let local_address = listener.local_addr().unwrap();
    println!("Serve at {}:{}", local_address.ip(), local_address.port());
    run(listener)?.await
}
