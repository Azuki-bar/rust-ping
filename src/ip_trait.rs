pub(crate) trait Ip {
    fn bits_string(self: &Self) -> String;
    fn new(ipaddress: &str) -> Result<Self, String> where Self: Sized;
    fn std(self: &Self) -> std::net::Ipv4Addr;
    fn to_string(self: &Self) -> String;
}
