mod load_balancer;
mod weight;
mod socket_address;
mod tests;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt as _};


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

/// Process the request
async fn process(mut socket: TcpStream) {
    let test_server = "127.0.0.1:7878"; // test

    let mut buf = vec![0u8; 8192 + 1];

    let res = read_in_loop(&mut socket, &mut buf).await;
    println!("size: {}\n", res);

    match TcpStream::connect(test_server).await {
        Ok(mut s2) => {
            s2.write_all(&buf[..]).await.unwrap();
            if read_in_loop(&mut s2, &mut buf).await != 0 {
                socket.write_all(&buf[..]).await.unwrap()
            }
            else {
                eprintln!("failed to read from socket 2");
                return;
            }
        },
        Err(_) => {
            eprintln!("failed to open socket 2");
            return
        }
    };
}

/// Reads data from socket until all data are arrived
async fn read_in_loop(socket: &mut TcpStream, buf: &mut Vec<u8>) -> usize {
    let mut m = 1;
    loop {
        m = match (*socket).read(&mut buf[(m-1)..]).await {
            Ok(n) if n <= 0 => break n + (m - 1),
            Ok(n) if n < (buf.len() - (m - 1)) => break n + (m - 1),
            Ok(n) => {
                buf.resize(buf.len() * 2, 0u8);
                n + (m - 1)
            },
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return 0;
            }
        };
    }
}