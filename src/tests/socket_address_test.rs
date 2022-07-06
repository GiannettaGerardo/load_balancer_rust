#[cfg(test)]
mod tests {
    use crate::server::socket_address::*;

    #[test]
    fn should_not_pass_test_ptest() {
        let p = SocketAddress::new(
            String::from("test"), 
            String::from("test")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_12_t_p1t() {
        let p = SocketAddress::new(
            String::from("12.t"), 
            String::from("1t")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_1_t_t_1_p1t1t() {
        let p = SocketAddress::new(
            String::from("1.t.t.1"), 
            String::from("1t1t")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_pass_127_0_0_1_p65535() {
        let p = SocketAddress::new(
            String::from("127.0.0.1"), 
            String::from("65535")
        );

        assert!(p.is_ok());
    }

    #[test]
    fn should_not_pass_127_0000_0_1_p8080() {
        let p = SocketAddress::new(
            String::from("127.0000.0.1"), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_127_0_0_p8080() {
        let p = SocketAddress::new(
            String::from("127.0.0"), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_127_p8080() {
        let p = SocketAddress::new(
            String::from("127"), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_pass_9_0_0_1_p1() {
        let p = SocketAddress::new(
            String::from("9.0.0.1"), 
            String::from("1")
        );

        assert!(p.is_ok());
    }

    #[test]
    fn should_not_pass_127_0_0_point_p8080() {
        let p = SocketAddress::new(
            String::from("127.0.0."), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_127_0_0_1_space_p8080() {
        let p = SocketAddress::new(
            String::from("127.0.0.1 "), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_space_127_0_0_1_p5000() {
        let p = SocketAddress::new(
            String::from(" 127.0.0.1"), 
            String::from("5000")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_minus_17_0_0_1_p8080() {
        let p = SocketAddress::new(
            String::from("-17.0.0.1"), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_300_12_1_999_p8080() {
        let p = SocketAddress::new(
            String::from("300.12.1.999"), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_256_256_256_256_p8080() {
        let p = SocketAddress::new(
            String::from("256.256.256.256"), 
            String::from("8080")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_1_1_1_1_p99999() {
        let p = SocketAddress::new(
            String::from("1.1.1.1"), 
            String::from("99999")
        );

        assert!(p.is_err());
    }

    #[test]
    fn should_not_pass_1_1_1_1_p999998() {
        let p = SocketAddress::new(
            String::from("1.1.1.1"), 
            String::from("999998")
        );

        assert!(p.is_err());
    }
}