use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::api::network::node::{Node};
use crate::api::http::requests::{post_json};

/* ----\\\000111000111000111000111000111_XAVAX_TRAHENTIUM_kayowo_WE_ARE_ONE\\\ --- Non important Note
    info.rs contains api calls to the avalanche node info endpoint. This endpoint has methods that will
    give important info about the node, such as its version, tis IP, its ID, the current network name (fuji, avax, etc),
    and most importantly: the current fees.
*/ 


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TxFees {
    pub tx_fee: u64,
    pub creation_fee: u64,
    pub create_asset_fee: u64,
    pub create_subnet_fee: u64,
    pub create_blockchain_fee: u64
}


/// # get_tx_fee
/// `Calls info.getTxFee on the Avalanche node.`
/// 
/// `get_tx_fees` will (on success) will return a struct containing the current fees for each
/// transaction type, the fees will be denominated in nAvax *(nano avax: a billionth of an avax.)*
/// 
/// ## todo: example
pub async fn get_tx_fee(destionation: Node) -> Result<TxFees, String> {
    let post = json!({
        "jsonrpc": "2.0",
        "id"     : 1,
        "method" : "info.getTxFee",
        "params" : {
        }
    });
    let endpoint = "/ext/info".to_string();
    let response = post_json(post, &destionation.node_ip, &endpoint, &destionation.port)
    .await.expect("Post Request Failure: ").text().await.expect("Post request Failed!");
    match serde_json::from_str::<Value>(response.as_str()) {
        Ok(r) => {
            match r["result"].as_object() {
                Some(map) => {
                    Ok(
                        TxFees {
                            tx_fee: map["txFee"].as_str().unwrap().parse::<u64>().expect("Failed parsing Tx Fee! fix ur shit..."),
                            creation_fee: map["creationTxFee"].as_str().unwrap().parse::<u64>().expect("Failed parsing Tx Fee! fix ur shit..."),
                            create_asset_fee: map["createAssetTxFee"].as_str().unwrap().parse::<u64>().expect("Failed parsing Tx Fee! fix ur shit..."),
                            create_subnet_fee: map["createSubnetTxFee"].as_str().unwrap().parse::<u64>().expect("Failed parsing Tx Fee! fix ur shit..."),
                            create_blockchain_fee: map["createBlockchainTxFee"].as_str().unwrap().parse::<u64>().expect("Failed parsing Tx Fee! fix ur shit..."),
                        }
                    )
                },
                None => {
                    Err(format!("Error getting tx fee!: {}", &r).to_string())
                },
            }
        },
        Err(e) => {
            Err(format!("Error {0} with the response: {1}", e.to_string(), response))
        }
    }
}