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

    pub fn listen_on(mut self, socket_addr: SocketAddress) -> Self {
        self.listening_socket_addr = Some(socket_addr);
        self
    }

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


/// Process the request
async fn process(mut sender_socket: TcpStream, socket_address: &SocketAddress) {
    let mut buf = vec![0u8; 8193];

    println!("Socket usata con porta {}", socket_address.get_port_number()); // test

    let res = read_in_loop(&mut sender_socket, &mut buf).await;
    println!("size: {}\n", res);

    match TcpStream::connect(socket_address.get()).await {
        Ok(mut receiver_socket) => {
            receiver_socket.write_all(&buf[..]).await.unwrap();
            if read_in_loop(&mut receiver_socket, &mut buf).await != 0 {
                sender_socket.write_all(&buf[..]).await.unwrap()
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