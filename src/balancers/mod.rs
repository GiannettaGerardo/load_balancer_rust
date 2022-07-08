pub mod standard_weighted_load_balancer;

use std::path::Path;
use super::server::socket_address::SocketAddress;

// json keys
static SERVERS_KEY: &str = "Servers";
static IPV4_KEY: &str = "ipv4";
static PORT_KEY: &str = "port";
static WEIGHT_KEY: &str = "weight";

// error messages
static INCORRECT_PATH: &str = "The path of the file isn't correct";
static INCORRECT_JSON_FORMAT: &str = "The json format isn't correct";
static NO_SERVERS_KEY: &str = "The is no \"Servers\" key in the json";
static NO_IPV4_KEY: &str = "The is no \"ipv4\" key in the json";
static NO_PORT_KEY: &str = "The is no \"port\" key in the json";
static NO_WEIGHT_KEY: &str = "The is no \"weight\" key in the json";


/// An interface for all the load balancers implementation
pub trait LoadBalancer {

    /// Create and return a new instance of struct
    /// # Arguments
    ///
    /// * `servers_number` - the capacity of the servers container
    /// 
    /// # Return
    /// 
    /// * A result with Self struct in a Box or an error string  
    fn new(servers_number: usize) -> Result<Box<Self>, &'static str>;

    /// Return the socket address of the next server.
    /// The implementation of this operation must be thread safe.
    /// 
    /// # Return
    /// 
    /// * A reference to a socket address
    fn next_server(&self) -> &SocketAddress;

    /// Insert a new SocketAddress in the inner vector.
    /// Return an error if the len of vector is already 
    /// at the max capacity MAX_SERVERS.
    /// # Arguments
    ///
    /// * `socket_address` - the socket address of the new server
    /// * `weight` - the weight of server
    /// 
    /// # Return
    /// 
    /// * A result with empty Ok or an error string
    fn insert_socket_address(
        &mut self, 
        socket_address: SocketAddress, 
        weight: usize
    ) -> Result<(), &'static str>;

}


/// Factory function for every load balancer implementation.
/// # Arguments
///
/// * `servers_number` - the capacity of the servers container
/// 
/// # Generics
/// 
/// * `T` - load balancer type
/// 
/// # Return
/// 
/// * A result with Self struct or an error string  
pub fn load_balancer_factory<T>(servers_number: usize) -> Result<T, &'static str>
where T: LoadBalancer + Sync + Send + 'static {
    match T::new(servers_number) {
        Ok(t) => Ok(*t),
        Err(e) => Err(e)
    }
}


/// Open the configuration json file and extract the servers data.
/// # Arguments
/// 
/// * `file_path` - the path of the configuration json file
/// 
/// # Return
/// 
/// * A vector with tuples containing the socket address and the relative weight
pub fn configure<'a>(file_path: &'a Path) -> Vec<(SocketAddress, usize)> {
    let file = std::fs::OpenOptions::new()
            .write(false)
            .read(true)
            .open(file_path)
            .expect(INCORRECT_PATH);

    let json: serde_json::Value = serde_json::from_reader(file)
        .expect(INCORRECT_JSON_FORMAT);

    let servers_arr = json[SERVERS_KEY].as_array()
        .expect(NO_SERVERS_KEY);

    let servers: Vec<(SocketAddress, usize)> = servers_arr.iter()
    .map(|element| {
        let socket_addr = match SocketAddress::new(
            element[IPV4_KEY].as_str()
                .expect(NO_IPV4_KEY)
                .to_string(), 
            element[PORT_KEY].as_str()
                .expect(NO_PORT_KEY)
                .to_string()
        ) {
            Ok(socket_addr) => socket_addr,
            Err(e) => panic!("{e}")
        };

        let weight = element[WEIGHT_KEY].as_u64()
            .expect(NO_WEIGHT_KEY);

        (socket_addr, weight as usize)
    })
    .collect();

    servers
}


/// Create, fill and return the load balancer generic struct.
/// # Arguments
/// 
/// * `file_path` - the path of the configuration json file
/// 
/// # Generics
/// 
/// * `T` - load balancer type
/// 
/// # Return
/// 
/// * A load balancer of type T
pub fn create_and_fill_the_balancer<'a, T>(file_path: &'a Path) -> T
where T: LoadBalancer + Sync + Send + 'static {
    let servers = configure(file_path);
    let mut balancer = match load_balancer_factory::<T>(servers.len()) {
        Ok(balancer) => balancer,
        Err(e) => panic!("{e}")
    };
    for (socket_address, weight) in servers {
        match balancer.insert_socket_address(socket_address, weight) {
            Ok(_) => (),
            Err(e) => panic!("{e}")
        };
    }
    balancer
}