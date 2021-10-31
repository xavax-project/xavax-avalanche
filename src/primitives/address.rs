
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    pub address_bytes: [u8; 20],
    pub serialized_address: String
}