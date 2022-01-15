use serde::{Deserialize, Serialize};
use crate::encoding::cb58::decode_cb58;


/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    network.rs contains information that is necessary to send transactions on the avalanche network,
    the Ip of the node, port, and because avalanche is a multi-blockchain network we need to support
    that.
    There also exist constants for the three default blockchains: X, P, and C chain, although it is
    definitely possible to use the Node and Blockchain primitives to add more blockchains!
    ps: docs improving soon, sorry
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DefaultBlockchains {
    XChain(XChain),
    PChain(PChain),
    CChain(CChain),
    AlienBlockchain(AlienBlockchain)
}
impl Default for DefaultBlockchains {
    fn default() -> Self {
        Self::XChain(XChain::default())
    }
}

//The host + the port of the node that the api will communicate & send requests to.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Node {
    pub node_ip: String,
    pub port: u16
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Blockchain {
    pub blockchain_id: [u8; 32],
    pub network_id: u32,
    pub arbitrary_data: Option<Vec<u8>> // Some blockchains may require some custom data, just a dummy thing before a more permanent solution
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AlienBlockchain {
    pub node: Node,
    pub blockchain_data: Blockchain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XChain {
    pub node: Node,
    pub blockchain_data: Blockchain,
}
impl Default for XChain {
    fn default() -> Self {
        Self {
            node: Node {
                node_ip: "api.avax-test.network".to_string(),
                port: 443,
            },
            blockchain_data: Blockchain{
                blockchain_id: decode_cb58("2JVSBoinj9C2J33VntvzYtVJNZdN2NKiwwKjcumHUWEb5DbBrm".to_string())[..].try_into().expect("Slice with incorrect length! uwu"),
                network_id: 5,
                arbitrary_data: None
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PChain {
    pub node: Node,
    pub blockchain_data: Blockchain,
}
impl Default for PChain {
    fn default() -> Self {
        Self {
            node: Node {
                node_ip: "api.avax-test.network".to_string(),
                port: 443,
            },
            blockchain_data: Blockchain{
                blockchain_id: decode_cb58("11111111111111111111111111111111LpoYY".to_string())[..].try_into().expect("Slice with incorrect length! uwu"),
                network_id: 5,
                arbitrary_data: None
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CChain {
    pub node: Node,
    pub blockchain_data: Blockchain,
}
impl Default for CChain {
    fn default() -> Self {
        Self {
            node: Node {
                node_ip: "api.avax-test.network".to_string(),
                port: 443,
            },
            blockchain_data: Blockchain{
                blockchain_id: decode_cb58("yH8D7ThNJkxmtkuv2jgBa4P1Rn3Qpr4pPr7QYNfcdoS6k6HWp".to_string())[..].try_into().expect("Slice with incorrect length! uwu"),
                network_id: 5,
                arbitrary_data: None
            }
        }
    }
}