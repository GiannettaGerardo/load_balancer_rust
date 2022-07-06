use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt as _};
use crate::socket_address::SocketAddress;
use crate::load_balancer::WeightedRoundRobinLB;

/// Manage the app execution
pub struct App {
    listening_socket_addr: Option<SocketAddress>
}

impl App {
    pub fn new() -> Self {
        App { listening_socket_addr: None }
    }

    /// Set the socket address for listening.
    pub fn listen_on(mut self, socket_addr: SocketAddress) -> Self {
        self.listening_socket_addr = Some(socket_addr);
        self
    }

    /// Starts the server.
    pub async fn run(self) {
        let soc_addr = self.listening_socket_addr.expect("Error");
        let balancer = Arc::new(get_load_balancer());

        println!("Starting the server...");
    
        // Bind the listener to the address
        let listener = TcpListener::bind(soc_addr.get()).await.unwrap();

        println!("Listening on {}...", soc_addr.get());
    
        loop {
            let balancer = Arc::clone(&balancer);
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();
            // A new task is spawned for each inbound socket. The socket is
            // moved to the new task and processed there.
            tokio::spawn(async move {
                process(socket, balancer.next_server()).await;
            });
        }
    }
}


/// Process the request.
/// Reads the bytes of the request and redirects them to a server 
/// that will process them and send the response. Finally reads 
/// the response and redirects it back to the original sender.
/// # Arguments
///
/// * `sender_socket` - the sender socket.
/// * `socket_address` -  the socket address of the server to which 
///                       to redirect the sender's request.
async fn process(mut sender_socket: TcpStream, socket_address: &SocketAddress) {
    let mut buf = vec![0u8; 8193];

    println!("Socket usata con porta {}", socket_address.get_port_number()); // test

    let total_bytes = read_in_loop(&mut sender_socket, &mut buf).await;
    println!("size: {}\n", total_bytes);

    match TcpStream::connect(socket_address.get()).await {
        Ok(mut receiver_socket) => {
            receiver_socket.write_all(&buf[..total_bytes]).await.unwrap();
            let total_bytes_2 = read_in_loop(&mut receiver_socket, &mut buf).await;
            if total_bytes_2 == 0 {
                return
            }
            sender_socket.write_all(&buf[..total_bytes_2]).await.unwrap()
        },
        Err(_) => {
            eprintln!("failed to open socket 2");
            return
        }
    };
}

/// Reads data from socket until all data are arrived
/// # Example
/// ```
/// buf.len = 8000
/// m = 0
/// 
/// socket -> sends 8001 Byte
/// 
///  read  -> n = 8000 -> buf[m..7999] = buf[0..7999]
///           buf.len = 8000 * 2 = 16000
///           m = n + m = 8000 + 0 = 8000
/// 
///  read  -> n = 1 -> buf[m..m+1] = buf[8000..8001]
///           n < (buf.len() - m) = 1 < 16000 - 8000 = 1 < 8000 = true
///              return n + m = 1 + 8000
/// ```
/// # Arguments
///
/// * `socket` - the socket from which to read the data.
/// * `buf` - the buffer in which to save the read data.
/// 
/// # Return
///
/// * The number of bytes read.
/// 
async fn read_in_loop(socket: &mut TcpStream, buf: &mut Vec<u8>) -> usize {
    let mut m = 0;
    loop {
        m = match (*socket).read(&mut buf[m..]).await {
            Ok(n) if n <= 0 => break m,
            Ok(n) if n < (buf.len() - m) => break n + m,
            Ok(n) => {
                buf.resize(buf.len() * 2, 0u8);
                n + m
            },
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return 0;
            }
        };
    }
}

/// Only for testing purposes
fn get_load_balancer() -> WeightedRoundRobinLB {
    let mut balancer = WeightedRoundRobinLB::new(2).unwrap();
    
    balancer.insert_socket_address(
        SocketAddress::new(String::from("127.0.0.1"), String::from("7878")).unwrap(), 
        1
    ).unwrap();
    balancer.insert_socket_address(
        SocketAddress::new(String::from("127.0.0.1"), String::from("7879")).unwrap(), 
        3
    ).unwrap();

    balancer
}