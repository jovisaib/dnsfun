use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    // Udp socket to listen on.
    #[clap(long, short, default_value="0.0.0.0:1053",env="DNS_UDP")]
    pub udp: Vec<SocketAddr>,
    
    //TCP socket to listen on
    #[clap(long, short, env="DNS_TCP")]
    pub tcp: Vec<SocketAddr>,
    
    //Domain name
    #[clap(long, short, default_value="dnsfun.dev", env="DNS_DOMAIN")]
    pub domain: String,
}