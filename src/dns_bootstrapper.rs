use ln::msgs::NetAddress;

use secp256k1::key::PublicKey;

use crate::network_bootstrapper::{NetworkBootstrapper, NetworkFilter};
use crate::dns::message;

use rand::{thread_rng, Rng};

use std::net::{UdpSocket, ToSocketAddrs};

//TODO: tmp structures
struct Peers {}

enum DNSError {
	InvalidDNSSeed,
	SendFail,
	BindFail,
}

/// A DNS Boostrapper
pub struct DNSNetworkBootstrapper {
	/// Pool of peers
	peers: HashMap<PublicKey, NetAddress>,
	/// DNS Seed server
	seed: String,
	/// Seed root domain
	seed_root_domain: String
}

impl NetworkBootstrapper for DNSNetworkBootstrapper {
	fn get_peers(&mut self, peers: usize, _filter: NetworkFilter) {
		if self.peers.len() >= peers {
			//iter and shuffle to get a random subset of peers		
		} else {
			self.query_seed(peers - self.peers.len());
		}
	}
}

impl DNSNetworkBootstrapper {
	fn new(seed: String, seed_root_domain: String) -> DNSNetworkBootstrapper {
		DNSNetworkBootstrapper {
			peers: Vec::new(), //hashmap pubkey - addr
			seed,
			seed_root_domain,
		}
	}

	fn query_seed(&mut self, peers: usize) -> Result<(), DNSError> {
		
		let mut rng = thread_rng();

		let dns_server = self.seed.insert_str(self.seed.len(), ":53");
		if let Some(addr_server) = dns_server.to_socket_addrs().unwrap().next() {
			if let Ok(socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address")) {

				// Build a SRV query to the DNS server
				let mut header = Header::new();
				header.set_id(rng.gen())
					.set_qr(0)
					.set_opcode(0)
					.set_recursion_desired();
				let mut question = Question::new();
				question.set_qname(self.seed_root_domain); //
				 
				let mut message = Message::new();
				message.add_header(header)
					.add_query(question)
					.update_counts();

				if let Err(_) = socket.send_to(&message.serialize(), addr) {
					return DNSError::SendFail;
				}
				
				// Wait for answer
				let mut buf = [0;100]; // should allocate for 25 * Resource + message size
				socket.recv_from(&mut buf);

				let response = Message::from(buf);
				for record in resonse.records() {
					if record.rtype() == Rtype::SRV {
						//TODO send A query or AAAA query
						//
						let response = //? socket to who ?
						//TODO: LookupHost();

						peers.insert(key, addr);
					}
				}
			} else {
				return DNSError::BindFail
			}
		} else {
			return DNSError::InvalidDNSSeed;
		}
	}
}

//TODO: implement by message descent and then serializer + test
