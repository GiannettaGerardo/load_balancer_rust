#[cfg(test)]
mod tests {
    use crate::load_balancer::*;

    fn add_soc_addr(wrrlb: &mut WeightedRoundRobinLB) -> Result<(), &'static str> {
        wrrlb.insert_socket_address(SocketAddress(String::from("127.0.0.1"), String::from("9000")))
    }

    fn create_filled_load_balancer() -> WeightedRoundRobinLB {
        let dim = 5;
        let mut b = WeightedRoundRobinLB::new(dim).unwrap();
        for _ in 0..dim {
            add_soc_addr(&mut b).unwrap();
        }
        return b;
    }

    #[test]
    fn multiple_tests() {
        let fixture = create_filled_load_balancer();
        assert_eq!(fixture.n_of_servers(), 5, "We're testing that address.len == 5");
        assert_eq!(fixture.n_of_servers(), fixture.capacity(), "We're testing that address.len == address.cap");
    }

    #[test]
    fn max_number_of_servers_has_max_capacity() {
        let fixture = WeightedRoundRobinLB::new(MAX_SERVERS);
        match fixture {
            Ok(fixture) => assert_eq!(fixture.capacity(), MAX_SERVERS),
            Err(_) => panic!("max_number_of_servers_has_max_capacity")
        }
    }

    #[test]
    fn zero_servers_return_string_error() {
        let fixture = WeightedRoundRobinLB::new(0);
        match fixture {
            Err(e) => assert_eq!(ZERO_OR_NEGATIVE_SERVERS, e),
            Ok(_) => panic!("n257_servers_return_error")
        }
    }

    #[test]
    fn max_plus_one_servers_return_string_error() {
        let fixture = WeightedRoundRobinLB::new(MAX_SERVERS + 1);
        match fixture {
            Err(e) => assert_eq!(TOO_MANY_SERVERS, e),
            Ok(_) => panic!("n257_servers_return_error")
        }
    }

    #[test]
    fn push_max_plus_one_server_should_return_error() {
        let mut fixture = WeightedRoundRobinLB::new(MAX_SERVERS).unwrap();
        for _ in 0..MAX_SERVERS {
            match add_soc_addr(&mut fixture) {
                Ok(()) => continue,
                Err(_) => panic!("push_max_plus_one_server_should_return_error -> Err arm")
            }
        }
        match add_soc_addr(&mut fixture) {
            Ok(()) => panic!("push_max_plus_one_server_should_return_error -> Ok arm"),
            Err(e) => assert_eq!(e, TOO_MANY_SERVERS)
        }
    }
}