// This shows how the standard library might define IpAddr
// (Simplified version for demonstration)

struct Ipv4Addr {
    octets: [u8; 4],
}

struct Ipv6Addr {
    segments: [u16; 8],
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr { octets: [127, 0, 0, 1] });
    let loopback = IpAddr::V6(Ipv6Addr { 
        segments: [0, 0, 0, 0, 0, 0, 0, 1] 
    });
    
    println!("Standard library style enum with structs");
}
