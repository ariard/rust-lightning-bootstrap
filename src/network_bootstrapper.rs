pub enum NetworkFilter {
	Ipv4,
	Ipv6,
	All,
}

/// A trait to describe an object which can provide a lightning node with peers
pub trait NetworkBootstrapper : Send + Sync {
	fn get_peers(&mut self, peers: usize, filter: NetworkFilter);
}
