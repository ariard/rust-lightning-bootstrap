/// A RFC-1035 Domain Name Header
pub(crate) struct Header {
	id: u16,
	qr: u8,
	opcode: u8,
	authoritative_answer: u8,
	truncation: u8,
	recursion_desired: u8,
	recursion_available: u8,
	reserved: u8,
	rcode: u8
}

impl Header {
	fn new() -> Self {
		Header {
			id: 0,
			qr: 0,
			opcode: 0,
			authoritative_answer: 0,
			truncation: 0,
			recursion_desired: 0,
			recursion_available: 0,
			reserved: 0,
			rcode: 0,
		}
	}

	fn set_id(mut self, id: u16) -> Self {
		self.id = id;
		self
	}

	fn set_qr(mut self, qr: u8) -> Self {
		self.qr = qr;
		self
	}

	fn set_opcode(mut self, opcode: u8) -> Self {
		self.opcode = opcode;
		self
	}

	fn set_recursion_desired(mut self, rd: u8) -> Self {
		self.rd = rd;
		self
	}

	fn serialize(&mut self) -> Vec<u8> {

	}
}

/// A RFC-1035 Domain Name Question
pub(crate) struct Question {
	qname: String,
	qtype: u16,
	qlass: u16
}

impl Question {
	fn new() -> Self {
		Question {
			qname: String::new(),
			qtype: 0,
			qclass: 0
		}
	}

	fn set_qname(mut self, qname: String) -> Self {
		self.qname = qname;
		self
	}

	fn set_qtype(mut self, qtype: u16) -> Self {
		self.qtype = qtype;
		self
	}

	fn set_qclass(mut self, qclass: u16) -> Self {
		self.qclass = qclass;
		self
	}

	fn serialize(&mut self) -> Vec<u8> {

	}
}

/// A RFC-1035 Domain Name Resource
pub(crate) struct Resource {
	name: String,
	qtype: u16,
	qclass: u16,
	ttl: u32,
	rdlength: u16,
	rdata: String
}

impl Resource {
	fn new() -> Self {
		name: String::new(),
		qtype: 0,
		qclass: 0,
		ttl: 0,
		rdlength: 0,
		rdata: String::new()
	}

	fn serialize(&mut self) -> Vec<u8> {

	}
}

/// A RFC-1035 Domain Name Message
pub(crate) struct Message {
	header: Header,	
	questions: Vec<Question>,
	answers: Vec<Resource>,
	authorities: Vec<Resource>,
	additionals: Vec<Resource>,
}

impl Message {
	fn new() -> Self {
		Message {
			header: Header::new(),
			question: vec![],
			answers: vec![],
			authorities: vec![],
			additionals: vec![]
		}
	}

	fn serialize(&mut self) -> Vec<u8> {
	}
}
