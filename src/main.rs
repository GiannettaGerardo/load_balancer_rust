mod server;
mod balancers;
mod tests;

use std::path::Path;

use balancers::standard_weighted_load_balancer::load_balancer::WeightedRoundRobinLB;
use tokio::signal;
use crate::server::{
    app::Server,
    socket_address::SocketAddress
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let soc_addr = SocketAddress::new(
        String::from("127.0.0.1"), 
        String::from("6379")
    ).unwrap();

    tokio::spawn(async move {
        Server::new(soc_addr, Path::new("config.json"))
            .run::<WeightedRoundRobinLB>()
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