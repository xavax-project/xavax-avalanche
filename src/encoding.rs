use sha2::{ Sha256, Digest };

pub mod cb58;


///Generates a transaction ID from the given Tx bytes
pub fn get_tx_id(input: &[u8]) -> String {
    let mut tx_id: Vec<u8> = Vec::new();

    let tx_hash: Vec<u8> = get_sha256_digest(&input);

    tx_id.extend(tx_hash.iter().cloned());
    
    let tx_hash_hash: Vec<u8> = get_sha256_digest(&tx_hash[..]);

    let (_, right) = tx_hash_hash.split_at(28);
    tx_id.extend(right.iter().cloned());
    bs58::encode(&tx_id[..]).into_string()
    //encode_cb58(&tx_hash_hash)
}  

///Take arbitrary byte vector as input, return Sha256 digest of input.
fn get_sha256_digest(input: &[u8]) ->  Vec<u8>{    
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(input);
    hasher.finalize()[..].to_vec()
}