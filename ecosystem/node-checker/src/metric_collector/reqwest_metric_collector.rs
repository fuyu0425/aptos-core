use std::net::{IpAddr, Ipv4Addr};

use reqwest::Client as ReqwestClient;
use reqwest::Url;
use url::Host;

// TODO Make it possible to reject nodes unless they are a specific type.
pub struct ReqwestMetricCollector {
    client: ReqwestClient,
    node_url: Url,
}

impl ReqwestMetricCollector {
    pub fn new(node_url: Url) -> Self {
        let mut client_builder = ReqwestClient::builder();
        let mut is_localhost = false;
        if let Some(host) = node_url.host() {
            match host {
                Host::Domain(s) => if s.contains("localhost") {
                    is_localhost = true;
                }
                Host::Ipv4(ip) => if ip == Ipv4Addr::LOCALHOST {
                    is_localhost = true;
                }
                _ => {}
            }
            if is_localhost {
                client_builder = client_builder.local_address(IpAddr::from([127, 0, 0, 1]));
            }
        }
        ReqwestMetricCollector {
            client: client_builder.build().unwrap(),
            node_url,
        }
    }
}
