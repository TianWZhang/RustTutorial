use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

fn ip_addr() {
    let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(v4));
    assert_eq!("::1".parse(), Ok(v6));

    assert!(v4.is_loopback());
    assert!(v6.is_loopback());
    assert!(!IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)).is_loopback());

    assert!(!v4.is_multicast());
    assert!(!v6.is_multicast());
    assert!(IpAddr::V4(Ipv4Addr::new(224, 254, 0, 0)).is_multicast());

    assert!(v4.is_ipv4());
    assert!(v6.is_ipv6());
}

fn socket_addr() {
    let mut v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);
    
    assert_eq!(v4.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    v4.set_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    assert_eq!(v4.ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    v6.set_ip(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 1, 1)));

    assert_eq!(v4.port(), 8080);
    v4.set_port(1025);
    v6.set_port(1025);

    assert!(v4.is_ipv4());
    assert!(v6.is_ipv6());
}

fn main() {
    ip_addr();
    socket_addr();
}
