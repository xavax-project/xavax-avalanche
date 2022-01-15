use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::api::network::node::{Node};
use crate::api::http::requests::{post_json};

/* ----\\\000111000111000111000111000111_XAVAX_TRAHENTIUM_kayowo_WE_ARE_ONE\\\ --- Non important Note
    ortelius.rs contains api calls to the avalanche ortelius indexer, this is useful when we want to check what addresses exist on the different chains
    so we can fetch UTXOs.
*/ 




/// # AddressChain
/// **Because subnets on avalanche can use the same cryptography, and because
/// the Xchain and Pchain use the same addresses, it is good to keep track of
/// the multiple different chains a given address currently exist in. The AddressChain
/// Struct keeps track of that.**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressChain {
    pub address: String,
    pub chains: Vec<String>
}


/// # get_address_chains
/// **get_address_chains will return a vector of AddressChain, which contains an address and all
/// the chains that said address exist on in the Avalanche network. If no address passed in was present
/// in any chain, None will be returned.**
pub async fn get_address_chains(addresses: &Vec<String>, destination: &Node) -> Result<Option<Vec<AddressChain>>, String> {
    let post = json!({
        "address": addresses,
    });
    let response = post_json(post, &destination.node_ip, &"/v2/addressChains".to_string(), &destination.port)
    .await.expect("Ree").text().await.expect("Post request Failed!");
    match serde_json::from_str::<Value>(response.as_str()) {
        Ok(r) =>
        {
            let mut result: Vec<AddressChain> = Vec::new();
            if r["addressChains"].as_object().is_some()
            {
                for a in r["addressChains"].as_object().unwrap() {
                    result.push(AddressChain{address: a.0.to_string(), chains: serde_json::from_value(json!(a.1.as_array().unwrap().to_vec())).unwrap()});
                }
                Ok(Some(result))
            } else {
                Err(format!("Error: {}", r.to_string()))
            }
        }
        Err(e) => {
            Err(format!("Error {0} with the response: {1}", e.to_string(), response))
        }
        //Ok(serde_json::from_str::<JsonAPIResponse>(response.as_str()).unwrap().result.get_key_value("txID").unwrap().1.to_string())
    }    
}

//# get_transaction
//pub async fn get_transactions(addresses: Vec<String>, endpoint: Endpoint) -> Result