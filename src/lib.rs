#![feature(plugin)]
#![feature(ip_addr)]

extern crate curl;
extern crate regex;
extern crate xml;

// data structures
pub use self::gateway::Gateway;

// request external ip address
pub use self::external::get_external_ip;
pub use self::external::RequestError;

// request port mapping
pub use self::external::PortMappingProtocol;
pub use self::external::add_port;

// search of gateway
pub use self::search::search_gateway;
pub use self::search::SearchError;

mod gateway;
mod external;
mod search;
