mod app;
mod load_balancer;
mod weight;
mod socket_address;
mod tests;

use tokio::signal;

use crate::app::App;
use crate::socket_address::SocketAddress;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let soc_addr = SocketAddress::new(
        String::from("127.0.0.1"), 
        String::from("6379")
    ).unwrap();

    tokio::spawn(async move {
        App::new()
            .listen_on(soc_addr)
            .run()
            .await;
    });

    match signal::ctrl_c().await {
        Ok(()) => println!("\nShutting down the server..."),
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
            println!("\nShutting down the server...")
        },
    }

    Ok(())
}