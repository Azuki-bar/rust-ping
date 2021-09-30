use std::fmt::{Debug, Formatter};
use crate::ip_trait::Ip;
use regex::Regex;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, IpAddr};
    use crate::my_ipv4::{Ipv4, octets2bit};
    use crate::ip_trait::Ip;

    #[test]
    fn test_ipv4_new() {
        assert_eq!(Ipv4::new("1111").unwrap_err(), "Invalid IP 1111".to_string());
        assert_eq!(Ipv4::new("255.255.255.256").unwrap_err(), "Invalid IP 255.255.255.256".to_string());
        assert_eq!(Ipv4::new("0.0.0.0").unwrap(), Ipv4 { octets: [0, 0, 0, 0], bit: 0 });
        assert_eq!(Ipv4::new("0.0.0.1").unwrap(), Ipv4 { octets: [0, 0, 0, 1], bit: 1 });
        assert_eq!(Ipv4::new("1.1.1.1").unwrap(), Ipv4 { octets: [1, 1, 1, 1], bit: 16843009 });
        assert_eq!(Ipv4::new("255.255.255.255").unwrap(), Ipv4 { octets: [255, 255, 255, 255], bit: 4294967295 });

        assert_eq!("1.1.1.1".parse::<Ipv4>().unwrap(), Ipv4 { octets: [1, 1, 1, 1], bit: 16843009 });
        assert_eq!("1".parse::<Ipv4>().unwrap_err(), "Invalid IP 1".to_string());
    }

    #[test]
    fn test_ipv4_std() {
        assert_eq!(Ipv4::new("0.0.0.0").unwrap().std(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(Ipv4::new("0.0.0.0").unwrap().std(), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), Ipv4Addr::new(0, 0, 0, 0));

        assert_eq!(Ipv4::new("1.1.1.1").unwrap().std(), Ipv4Addr::new(1, 1, 1, 1));
        assert_eq!(Ipv4::new("8.8.8.8").unwrap().std(), Ipv4Addr::new(8, 8, 8, 8));
        assert_eq!(Ipv4::new("127.127.127.127").unwrap().std(), (Ipv4Addr::new(127, 127, 127, 127)));
        assert_eq!(Ipv4::new("255.255.255.255").unwrap().std(), (Ipv4Addr::new(255, 255, 255, 255)));
    }

    #[test]
    fn test_ipv4_bits() {
        assert_eq!(Ipv4::new("0.0.0.0").unwrap().bits_string(), "00000000000000000000000000000000".to_string());
        assert_eq!(Ipv4::new("0.0.0.1").unwrap().bits_string(), "00000000000000000000000000000001".to_string());
        assert_eq!(Ipv4::new("255.255.255.254").unwrap().bits_string(), "11111111111111111111111111111110".to_string());
        assert_eq!(Ipv4::new("255.255.255.255").unwrap().bits_string(), "11111111111111111111111111111111".to_string());
    }

    #[test]
    fn test_octets2bit() {
        assert_eq!(octets2bit(&[0, 0, 0, 0]), 0);
        assert_eq!(octets2bit(&[0, 0, 0, 1]), 1);
        assert_eq!(octets2bit(&[0, 0, 1, 0]), 256);
        assert_eq!(octets2bit(&[1, 1, 1, 1]), 16843009);
        assert_eq!(octets2bit(&[255, 255, 255, 254]), 4294967294);
        assert_eq!(octets2bit(&[255, 255, 255, 255]), 4294967295);
    }
}

#[derive(PartialEq)]
pub(crate) struct Ipv4 {
    pub bit: u32,
    octets: [u8; 4],
}

impl Debug for Ipv4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IP: {}\tbit: {}", self.to_string(), self.bit)
    }
}

impl FromStr for Ipv4 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return self::Ipv4::new(s);
    }
}

impl Ip for Ipv4 {
    fn bits_string(&self) -> String {
        let mut res: String = "".to_string();
        for i in (0..32).rev() {
            let val = ((self.bit >> i) & 1).to_string();
            res = format!("{}{}", res, val);
        }
        return res;
    }

    fn new(ipaddress: &str) -> Result<Ipv4, String> {
        // https://www.oreilly.com/library/view/regular-expressions-cookbook/9780596802837/ch07s16.html
        let ipv4_reg_pattern = r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$";
        let re = Regex::new(ipv4_reg_pattern);
        let re = match re {
            Ok(re) => re,
            Err(err) => return Err(err.to_string()),
        };
        if !re.is_match(ipaddress) {
            return Err(format!("Invalid IP {}", ipaddress));
        }
        let each_reg_pattern = "(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)";
        let re = Regex::new(each_reg_pattern);
        let re = match re {
            Ok(re) => re,
            Err(err) => return Err(err.to_string())
        };

        let mut octets: [u8; 4] = [0, 0, 0, 0];
        for (i, c) in re.captures_iter(ipaddress).enumerate() {
            let octet_val = c[0].to_string().parse::<u8>();
            let octet_val = match octet_val {
                Ok(o) => o,
                Err(err) => return Err(err.to_string())
            };
            octets[i] = octet_val;
        }
        let bit = octets2bit(&octets);
        return Ok(Ipv4 { octets, bit });
    }
    fn std(self: &Self) -> Ipv4Addr {
        return Ipv4Addr::new(self.octets[0], self.octets[1], self.octets[2], self.octets[3]);
    }
    fn to_string(self: &Self) -> String {
        format!("{}.{}.{}.{}", self.octets[0], self.octets[1], self.octets[2], self.octets[3])
    }
}

fn octets2bit(octets: &[u8; 4]) -> u32 {
    let mut res: u32 = 0;
    for (i, val) in octets.into_iter().rev().enumerate() {
        let val_u32 = *val as u32;
        res += (val_u32) << (i * 8);
    }
    return res;
}

