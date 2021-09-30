pub(crate) trait Ip {
    type Err;
    fn bits_string(self: &Self) -> String;
    fn new(ipaddress: &str) -> Result<Self, Self::Err> where Self: Sized;
    fn std(self: &Self) -> std::net::Ipv4Addr;
    fn to_string(self: &Self) -> String;
}
