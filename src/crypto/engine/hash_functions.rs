use sha2::{Digest};
use sha2::Sha256;
use sha3::Keccak256;
use blake3;
use ripemd;

/// # get_sha256_hash
/// ___
/// `get_sha256_hash` takes in any arbitrary bytes, and returns said sha256 digest.
/// The sha256 hash function returns a 256bit/32byte output
/// 
/// *I'm very happy to be* **sha***ring these open-source crates with you! I hope you can make some
///***sha***ttering projects with them!*
/// 
/// *its 5 am... pls shank me*
/// 
/// ## Example
/// ```
/// use hex_literal::hex;
/// use xavax_crypto::engine::hash_fns::*;
/// 
/// //Get a sha256 Hash of "xavax".
/// let sha256_hash: Vec<u8> = get_sha256_hash(b"xavax");
/// 
/// //The "xavax" hash is: A38E80B91C64DA6AE744D3C0BBB059EDA28149E13C3170455DCE6328490CF22B (encoded in hex).
/// assert_eq!(sha256_hash, hex!("A38E80B91C64DA6AE744D3C0BBB059EDA28149E13C3170455DCE6328490CF22B"));
/// 
/// ```
pub fn get_sha256_hash(input: &[u8]) ->  Vec<u8>{    
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(input);
    hasher.finalize()[..].to_vec()
}

/// # get_keccak256_hash
/// ___
/// `get_keccak256_hash` takes in any arbitrary bytes, and returns said keccak256 digest.
/// The keccak256 hash function returns a 256bit/32byte output
/// 
/// *pronounced kek-kak btw B)*
/// 
/// ## Example
/// ```
/// use hex_literal::hex;
/// use xavax_crypto::engine::hash_fns::*;
/// 
/// //Get a keccak256 Hash of "xavax".
/// let keccak256_hash: Vec<u8> = get_keccak256_hash(b"xavax");
/// 
/// //The "xavax" hash is: f4d5f0afb5f2473efba237a409cadc1cb7e3340d049424f497ec01603cb92820 (encoded in hex).
/// assert_eq!(keccak256_hash, hex!("f4d5f0afb5f2473efba237a409cadc1cb7e3340d049424f497ec01603cb92820"));
/// 
/// ```
pub fn get_keccak256_hash(input: &[u8]) -> Vec<u8>{    
    let mut hasher: Keccak256 = Keccak256::new();
    hasher.update(input);
    hasher.finalize()[..].to_vec()
}

/// # get_ripemd160_hash
/// ___
/// `get_ripemd160_hash` takes in any arbitrary bytes, and returns said ripemd160 digest.
/// The ripemd160 hash function returns a 160bit/20byte output
/// 
/// *the md is very ripe... one sixty...*
/// 
/// ## Example
/// ```
/// use hex_literal::hex;
/// use xavax_crypto::engine::hash_fns::*;
/// 
/// //Get a ripemd160_ Hash of "xavax".
/// let ripemd160__hash: Vec<u8> = get_ripemd160_hash(b"xavax");
/// 
/// //The "xavax" hash is: 018b1e0d00933745efb282ad6ffa34eb1040a359 (encoded in hex).
/// assert_eq!(ripemd160__hash, hex!("018b1e0d00933745efb282ad6ffa34eb1040a359"));
/// 
/// ```
pub fn get_ripemd160_hash(input: &[u8]) ->  Vec<u8>{    
    let mut hasher: ripemd::Ripemd160 = ripemd::Ripemd160::new();
    hasher.update(input);
    hasher.finalize()[..].to_vec()
}

/// # get_blake3_hash
/// ___
/// `get_blake3_hash` takes in any arbitrary bytes, and returns said blake3 digest.
/// The get_blake3_hash hash function returns a 256bit/32byte output
/// 
/// *here for use in an experimental custom blockchain/subnet project.*
/// 
/// ## Example
/// ```
/// use hex_literal::hex;
/// use xavax_crypto::engine::hash_fns::*;
/// 
/// //Get a blake3_ Hash of "xavax".
/// let blake3_hash: Vec<u8> = get_blake3_hash(b"xavax");
/// 
/// //The "xavax" hash is: 9a9d675fede22bb3a1aa6b350a6e848a905cd505fe53da78cec552a7beaf308a (encoded in hex).
/// assert_eq!(blake3_hash, hex!("9a9d675fede22bb3a1aa6b350a6e848a905cd505fe53da78cec552a7beaf308a"));
/// 
/// ```
pub fn get_blake3_hash(input: &[u8]) ->  Vec<u8>{    
    blake3::hash(input).as_bytes().to_vec()
}