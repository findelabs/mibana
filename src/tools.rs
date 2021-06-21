use http::request::Parts;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};

pub type Queries = HashMap<String, String>;

pub fn queries(req: &Parts) -> Option<Queries> {
    let params: HashMap<String, String> = req
        .uri
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);
    Some(params)
}

pub fn get_client_ip(socket: SocketAddr) -> Option<Ipv4Addr> {
    // Get client ip, or return 255.255.255.255
    let client_ip = match socket.ip().to_string().parse::<Ipv4Addr>() {
        Ok(ipv4) => ipv4,
        Err(_) => Ipv4Addr::new(255, 255, 255, 255),
    };

    Some(client_ip)
}
