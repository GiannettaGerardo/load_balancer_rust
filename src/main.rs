mod load_balancer;
mod weight;
mod socket_address;
mod tests;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt as _};

use std::str::from_utf8;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting the server...");

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }

    // first incomplete version
    Ok(())
}

// Test
async fn response(mut socket: TcpStream) {
    let http_res = "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nCiao";
    if let Err(e) = socket.write_all(http_res.as_bytes()).await {
        eprintln!("failed to write to socket; err = {:?}", e);
        return;
    }
}

// Test
async fn process(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    // In a loop, read data from the socket and write the data back.
    loop {
        match socket.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        println!("{}", from_utf8(&buf).unwrap_or_else(|_| "FINEEE"));
    }
}