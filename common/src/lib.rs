use std::net::Ipv4Addr;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Database {
    //IP address for Database
    pub ip: Ipv4Addr,
    //Port for the Database
    pub port: u16,
}

impl Database {
    pub fn new(ip: &str, port: u16) -> anyhow::Result<Self> {
        //parse ip input
        let test: Ipv4Addr = ip.parse::<Ipv4Addr>()?;
        Ok(Database { ip: test, port })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
