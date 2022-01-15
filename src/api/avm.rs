use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::api::network::node::{Node};
use crate::api::http::requests::{post_json};



/* ________________________________________ X-Chain Api Json Responses ________________________________________ */


//Some response calls are quite complicated, so I decided to just write specific structs for them. In the end it might
//be more efficient to write complete structs for every POST as well but this "procedural way" is easier. 


///A UTXOResponse is returned on a successfull get_utxos call.
#[derive(Serialize, Deserialize, Debug)]
pub struct UTXOResponse {
    pub num_fetched: String,
    pub utxos: Vec<String>,
    pub end_address: String,
    pub end_utxo: String
}

/* ________________________________________ AVM api methods ________________________________________ */

/// # issue_tx
/// `Calls avm.issueTx on the Avalanche node.`
/// 
/// **Sends a signed CB58 Encoded transaction to the network, returns a transaction ID on success.**
/// 
/// 
/// **Make sure that the signed transaction is encoded in CB58, to do that you can simply call the
/// .to_cb58() method on the Sign**
/// 
pub async fn issue_tx(tx: String, destination: Node) -> Result<String, String> {
    let post = json!({
        "jsonrpc": "2.0",
        "id"     : 1,
        "method" : "avm.issueTx",
        "params" : {
            "tx": tx,
            "encoding": "cb58",
        }
    });
    let endpoint = "/ext/bc/X".to_string();
    let response = post_json(post, &destination.node_ip, &endpoint, &destination.port)
    .await.expect("Ree").text().await.expect("Post request Failed!");

    match serde_json::from_str::<Value>(response.as_str()) {
        Ok(r) => {
            match r["result"].as_object() {
                Some(s) => {
                    Ok(s["txID"].as_str().unwrap().to_string())
                },
                None => {
                    Err(r.to_string())
                },
            }
        },
        Err(e) => {
            Err(format!("Error {0} with the response: {1}", e.to_string(), response))
        }
    }
}

/// # get_utxos
/// `Calls avm.getUTXOs on the Avalanche node.`
/// 
/// Gets the UTXOs that reference the given *addresses*. If *source_chain* is specified, then it will also
/// get the atomic UTXOs exported from that chain to this *(X)* chain. The
/// 
/// *limit* specifies how many UTXOs we will try and fetch, fetching the same amount of UTXOs as the limit hints
/// that there are more to be found.
///  
/// **Find all the mysterious UTXOs that might be hidden on the avalanche blockchain or DAG...
/// Or get delirious about the true balance that might be forever ridden from your bag.**
pub async fn get_utxos(addresses: Vec<String>, limit: u32, source_chain: Option<char>, destination: Node) -> Result<UTXOResponse, String> {
    let mut addresses_prefixed: Vec<String> = Vec::new();
    let endpoint: String;
    endpoint = "/ext/bc/X".to_string();    
    for a in addresses {
        if !(a.chars().nth(0).unwrap() == 'X') {
            addresses_prefixed.push(format!("{}{}", "X-", a));
        }
    }

    let post = json!({
        "jsonrpc": "2.0",
        "id"     : 1,
        "method" : "avm.getUTXOs",
        "params" : {
            "addresses": addresses_prefixed,
            "limit"    : limit,
            "sourceChain": source_chain
        }
    });
    let response = post_json(post, &destination.node_ip, &endpoint, &destination.port)
    .await.expect("Ree").text().await.expect("Post request Failed!");
    match serde_json::from_str::<Value>(response.as_str()) {
        Ok(r) => {
            match r["result"].as_object() {
                Some(v) => {
                    
                    Ok(UTXOResponse {
                        //Will optimize this later
                        num_fetched: v["numFetched"].to_string(),
                        utxos: serde_json::from_value(json!(v["utxos"].as_array().unwrap().to_vec())).unwrap(),
                        end_address: json!(v["endIndex"].as_object().unwrap())["address"].to_string(),
                        end_utxo: json!(v["endIndex"].as_object().unwrap())["utxo"].to_string(),
                     })
                },
                None => {
                    Err(r.to_string())
                },
            }
    
        },
        Err(e) => {
            Err(format!("Error {0} with the response: {1}", e.to_string(), response))
        }
    }
}
pub fn get_tx_status() -> Result<String, String> {
    todo!();
}