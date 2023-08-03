//---------------------------------------------------------------------------------------------------- Use
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use std::collections::BTreeMap;
use tokio::sync::{
	RwLock,
};
use std::net::{
	Ipv4Addr,
	SocketAddrV4,
};

//----------------------------------------------------------------------------------------------------
// Global map of seen IPs and their connection count.
pub static SEEN_IPS: RwLock<BTreeMap<Ipv4Addr, u64>> = RwLock::const_new(BTreeMap::new());

pub async fn add(addr: &std::net::SocketAddrV4) {
	SEEN_IPS.write().await.entry(*addr.ip()).and_modify(|c| { *c += 1 }).or_insert(1);
}

pub async fn seen(addr: &std::net::SocketAddrV4) -> bool {
	match SEEN_IPS.read().await.get(addr.ip()) {
		Some(k) => k > &1,
		None    => false,
	}
}

pub async fn count(addr: &std::net::SocketAddrV4) -> u64 {
	match SEEN_IPS.read().await.get(addr.ip()) {
		Some(k) => *k,
		None    => 0,
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
