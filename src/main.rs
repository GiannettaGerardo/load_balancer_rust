mod server;
mod balancers;
mod tests;

use std::path::Path;
use balancers::{
    standard_weighted_load_balancer::load_balancer::WeightedRoundRobinLB, 
    configure
};
use tokio::signal;
use server::{
    app::Server
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Configuring the server...");
    let (server_soc, servers_vec) = configure(Path::new("config.json"));
    println!("Configuration completed...");

    tokio::spawn(async move {
        Server::new(server_soc)
            .run::<WeightedRoundRobinLB>(servers_vec)
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