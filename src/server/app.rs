use core::panic;
use std::path::Path;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt as _};
use crate::{
    server::socket_address::*, 
    balancers::{
        load_balancer_factory, 
        LoadBalancer
    }
};

/// Manage the app execution
pub struct Server<'a> {
    listening_socket_addr: SocketAddress,
    file_path: &'a Path,

}

impl<'a> Server<'a> {
    pub fn new(socket_addr: SocketAddress, file_path: &'a Path) -> Self {
        Server { 
            listening_socket_addr: socket_addr, 
            file_path
        }
    }

    /// Open the configuration json file and extract the servers data.
    fn configure(&self) -> Vec<(SocketAddress, usize)> {
        let file = match std::fs::OpenOptions::new()
                .write(false)
                .read(true)
                .open(self.file_path) {
            Ok(file) => file,
            Err(_) => panic!("The path of the file isn't correct")
        };

        let json: serde_json::Value = match serde_json::from_reader(file) {
            Ok(json) => json,
            Err(_) => panic!("The json format isn't correct")
        };

        let servers_arr = match json["Servers"].as_array() {
            Some(arr) => arr,
            None => panic!("The is no \"Servers\" key in the json")
        };
        let servers: Vec<(SocketAddress, usize)> = servers_arr.iter()
        .map(|element|
            match element["ipv4"].as_str() {
                Some(ipv4) => 
                    match element["port"].as_str() {
                        Some(port) => 
                            match SocketAddress::new(ipv4.to_string(), port.to_string()) {
                                Ok(socket_addr) => 
                                    match element["weight"].as_u64() {
                                        Some(weight) => (socket_addr, weight as usize),
                                        None => panic!("The is no \"weight\" key in this json object")
                                    },
                                Err(e) => panic!("{e}")
                            },
                        None => panic!("The is no \"port\" key in this json object")
                    },
                None => panic!("The is no \"ipv4\" key in this json object")
            }
        )
        .collect();

        servers
    }

    /// Create, fill and return the load balancer generic struct.
    fn fill_the_balancer<T>(&self) -> T
    where T: LoadBalancer + Sync + Send + 'static {
        let mut balancer = match load_balancer_factory::<T>(5) {
            Ok(balancer) => balancer,
            Err(e) => panic!("{e}")
        };
        for (socket_address, weight) in self.configure() {
            match balancer.insert_socket_address(socket_address, weight) {
                Ok(_) => (),
                Err(e) => panic!("{e}")
            };
        }
        balancer
    }

    /// Starts the server.
    pub async fn run<T>(&mut self)
    where T: LoadBalancer + Sync + Send + 'static {
        let balancer = Arc::new(self.fill_the_balancer::<T>());

        println!("Starting the server...");
    
        // Bind the listener to the address
        let listener = match TcpListener::bind(self.listening_socket_addr.get())
        .await {
            Ok(listener) => listener,
            Err(e) => panic!("{e}")
        };

        println!("Listening on {}...", self.listening_socket_addr.get());
    
        loop {
            let balancer = Arc::clone(&balancer);
            // The second item contains the IP and port of the new connection.
            let socket = match listener.accept().await {
                Ok((socket, _)) => socket,
                Err(e) => {
                    println!("{e}");
                    continue
                }
            };
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