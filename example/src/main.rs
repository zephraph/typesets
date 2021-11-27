use std::convert::TryInto;
use std::net::IpAddr;

use typesets::Supertype;
use typesets::TypesetsError;

fn main() {
    #[derive(Debug, Supertype, Clone)]
    #[allow(dead_code)]
    pub enum NetworkTarget {
        #[subtype(RouteTarget, RouteDestination)]
        Vpc(String),

        #[subtype(RouteTarget, RouteDestination)]
        Subnet(String),

        #[subtype(RouteTarget)]
        Instance(String),

        Tag(String),

        #[subtype(RouteTarget, RouteDestination)]
        Ip(IpAddr),

        #[subtype(RouteTarget)]
        InternetGateway(String),

        FloatingIp(String),
    }

    let n = NetworkTarget::Vpc("test".to_string());
    let t: RouteTarget = n.clone().try_into().unwrap();
    let d: RouteDestination = n.clone().try_into().unwrap();
    let nt: NetworkTarget = t.into();
    let nd: NetworkTarget = d.into();
    println!("{:?}, {:?}", nt, nd);
}