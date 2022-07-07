pub mod standard_weighted_load_balancer;

use super::server::socket_address::SocketAddress;

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