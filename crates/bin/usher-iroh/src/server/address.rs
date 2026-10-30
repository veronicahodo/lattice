use std::net::{IpAddr, SocketAddr};

pub fn get_addresses(array: Vec<&SocketAddr>) -> String {
    let mut output = Vec::new();
    let mut count: usize = 0;
    for socket in array {
        let working = match socket.ip() {
            IpAddr::V4(ip) => {
                if count != 0 {
                    ["/".to_string(), ip.to_string()].concat()
                } else {
                    ip.to_string()
                }
            }
            IpAddr::V6(ip) => {
                if count != 0 {
                    ["/".to_string(), ip.to_string()].concat()
                } else {
                    ip.to_string()
                }
            }
        };
        let working = [working, format!(":{}", socket.port())].concat();
        count += 1;
        output.push(working);
    }

    output.concat()
}
