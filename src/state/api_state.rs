use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ApiState {
    pub ips: Arc<Mutex<HashMap<SocketAddr, usize>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiStateSerialized {
   pub ips: HashMap<String, usize>,
}

impl ApiStateSerialized {
    pub fn new(api_state: &ApiState) -> Self {
        let ips = api_state.get_ips();
        let serialized_ips = ips.iter()
            .map(|(k, v)| (k.to_string(), *v)) 
            .collect();

        ApiStateSerialized {
            ips: serialized_ips,
        }
    }
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            ips: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_ip(&self, ip: SocketAddr) -> usize {
        let mut ips = self.ips.lock().unwrap();
        let count = ips.entry(ip).or_insert(0);
        *count += 1;
        *count
    }

    pub fn get_ips(&self) -> HashMap<SocketAddr, usize> {
        let ips = self.ips.lock().unwrap();
        ips.clone()
    }
}