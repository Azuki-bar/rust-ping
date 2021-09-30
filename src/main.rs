mod my_ipv4;
mod ip_trait;

use std::net::{IpAddr};
use crate::{ip_trait::Ip, my_ipv4::Ipv4};

#[cfg(test)]
mod tests {
    use crate::{bit_to_int};

    #[test]
    fn test_bit_to_int() {
        assert_eq!(bit_to_int("0"), 0);
        assert_eq!(bit_to_int("0000000000000000000000000"), 0);
        let bit = "00000000001";
        assert_eq!(bit_to_int(bit), 1);
        let bit = "1111111";
        assert_eq!(bit_to_int(bit), 127);
        assert_eq!(bit_to_int("1111111111111111111111111111111"), (1 << 31) - 1);
    }

    #[test]
    fn test_ping() {}
}

fn bit_to_int(bit: &str) -> u32 {
    let mut res: u32 = 0;
    for (i, b) in bit.chars().rev().enumerate() {
        res += String::from(b).parse::<u32>().unwrap() * (1 << i);
    }
    return res;
}

fn ping(addr: IpAddr) -> std::io::Result<usize> {
    let mut socket = icmp::IcmpSocket::connect(addr)?;
    let payload: &[u8] = &[1, 2];
    return socket.send(payload);
}

fn main() {
    let i = Ipv4::new("127.0.0.1").unwrap();
    let result = ping(IpAddr::from(i.std()));
    match result {
        Ok(o) => {
            println!("{}", o);
            assert_eq!(o, 2);
        }
        Err(e) => panic!("{}", e)
    };
}
