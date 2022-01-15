use libsecp256k1::{RecoveryId, Signature, curve::Affine};
use arrayref::array_mut_ref;
use serde::{Serialize, Deserialize};

use crate::primitives::address::Address;




#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SECP256K1Keypair {
    pub public_key: Vec<u8>,    //Pub-key
    pub address: Address,       //Address
    pub private_key: Vec<u8>,   //Private Key
    pub derivation_path: String, //Derivation path
}


pub trait Signing {
    /// # sign
    /// ___
    /// ## Todo: Docs
    fn sign(private_key: &Vec<u8>, data_preimage: Vec<u8>) -> Vec<u8>;
    /// # verify
    /// ___
    /// ## Todo: Docs
    fn verify(self, signature: Vec<u8>, message_preimage: Vec<u8>) -> Result<Address, String>;
    /// # sign_message
    /// ___
    /// ## Todo: Docs
    fn sign_interop_message(private_key: &Vec<u8>, data_preimage: Vec<u8>) -> Vec<u8>;
    /// # verify_message
    /// ___
    /// ## Todo: Docs
    fn verify_message(self, signature: Vec<u8>, message_preimage: Vec<u8>) -> Result<Address, String>;
}
pub trait MnemonicKeypair {
    /// # generate_mnemonic_phrase
    /// ___
    /// `generate_mnemonic_phrase()` is a trait function will generate a new mnemonic phrase that can be used to derive
    /// private keys. The entropy source is Cryptographically secure as it uses ThreadRng.
    /// 
    /// This trait may me implemented
    /// for various different cryptographic primitives, i.e to generate pub-keys with the SECP256K1 curve or Edwards curve.
    /// 
    /// ## Example
    /// ```
    /// use xavax_crypto::avm::keys::*;
    /// use xavax_crypto::engine::elliptic_curves::*;
    /// 
    /// let seed_phrase = AvalancheKeys::generate_mnemonic_phrase();
    /// ```
    /// ## Note
    /// Generally, as `xavax-crypto` is a very high-level library that enables instant support for *currently* Avalanche and Ethereum
    /// cryptography, it is better to use the more high-level functions that automatically generate keychains and keeps track of them,
    /// 
    /// i.e
    /// `AvalancheKeys::new(/*--bunch of shit---*/);`
    /// 
    /// or
    /// 
    /// `EthereumKeys::(/*--more shit---*/);`
    fn generate_mnemonic_phrase() -> String;
    /// # generate_keypair
    /// ___
    /// `generate_keypair()` is a trait function that takes in two arguments, and returns a tuple that contains an
    /// `Address` and `SECP256K1Keypair`, what those are is fairly self-explanatory.
    /// 
    /// Because of the fact that you can generate many keypairs that use different derivation paths, its good to keep track of
    /// every address -> Private key pair, this can be done using a `std::collections::HashMap`, and this is how the more high-level
    /// keychain functions do it that exist in `xavax_crypto::avm::keys` or `xavax_crypto::eth::keys`, etc you get the point.
    /// 
    /// This trait may me implemented
    /// for various different primitives, Ethereum for instance doesn't use [Bech32](https://en.bitcoin.it/wiki/Bech32), but the
    /// [AVM](https://docs.avax.network/build/references/cryptographic-primitives#cryptography-in-the-avalanche-virtual-machine) does,
    /// this will mean the `Address` will potentially have a completely different format for different networks.
    /// 
    /// Derivation paths will also be different for all networks...
    /// 
    /// ## Example
    /// ```
    /// use xavax_crypto::avm::keys::*;
    /// use xavax_crypto::engine::elliptic_curves::*;
    /// 
    /// let seed_phrase: String = "bench small pioneer home chunk
    /// job flavor crucial tree twenty vapor guide punch
    /// shield laundry horn pencil grunt
    /// gravity save shell
    /// floor region model".to_string();
    /// 
    /// let derivation_path = "m/44'/9000'/0'/0/0";
    /// 
    /// let keypair: (Address, SECP256K1Keypair) = AvalancheKeys::generate_keypair(seed_phrase, derivation_path);
    /// ```
    /// ## Note
    /// Much like the `generate_mnemonic_phrase()`, there exist higher-level abstractions that take care of the
    /// keychains and keep track of the latest derivation index used, for instance `xavax_crypto::avm::AvalancheKeys::insert_new_keypair();`.
    fn generate_keypair(seed_phrase: String, derivation_path: &str) -> SECP256K1Keypair;

    fn generate_seed_entropy(seed_phrase: String) -> Vec<u8>;
}

/// # secp256k1_sign_rsv
/// ___
/// Generates an SECP256K1 signatures with `[R | S | V]` format to enable quick
/// public-key recovery. Avalanche virtual machine compatible.
/// 
/// Make sure to SHA256 the message before passing it into this function!
pub fn secp256k1_sign_rsv(private_key_bytes: [u8; 32], message: &[u8]) -> [u8; 65] {
    let private_key= libsecp256k1::SecretKey::parse_slice(&private_key_bytes).expect("Incorrect private key!");

    let mut signature: (Signature, RecoveryId) = libsecp256k1::sign(&libsecp256k1::Message::parse_slice(message).expect("EXPECTED SHA256 OF MESSAGE PAYLOAD! WRONG MESSAGE SIZE..."), &private_key);
    
    signature.0.normalize_s();

    let r = signature.0.r;
    let s = signature.0.s;
    let v: u8 = signature.1.serialize();
    
    let mut sig: [u8; 65] = [0u8; 65];
    
    r.fill_b32(array_mut_ref!(sig, 0, 32));
    s.fill_b32(array_mut_ref!(sig, 32, 32));
    sig[64] = v;

    sig
}

/// # secp256k1_sign_vsr
/// ___
/// Generates an SECP256K1 signatures with `[V | S | R]` format to enable quick
/// public-key recovery. For some reason, many ethereum clients use VSR signatures instead of
/// RSV, for that reason I decided to add this function here as well.
/// 
/// The V value is also not 0 or 1, but not 27 or 28, same principle: It allows for quick
/// public key recovery.
pub fn secp256k1_sign_vsr(private_key_bytes: [u8; 32], message: &[u8]) -> [u8; 65] {
    let private_key= libsecp256k1::SecretKey::parse_slice(&private_key_bytes).expect("Incorrect private key!");

    let mut signature: (Signature, RecoveryId) = libsecp256k1::sign(&libsecp256k1::Message::parse_slice(message).expect("EXPECTED SHA256 OF MESSAGE PAYLOAD! WRONG MESSAGE SIZE..."), &private_key);
    
    signature.0.normalize_s();

    let r = signature.0.r;
    let s = signature.0.s;
    let v: u8 = 27 + signature.1.serialize();
    
    let mut sig: [u8; 65] = [0u8; 65];
    
    sig[0] = v;
    s.fill_b32(array_mut_ref!(sig, 1, 32));
    r.fill_b32(array_mut_ref!(sig, 32, 32));

    sig
}

/// # secp256k1_verify_rsv
/// ___
/// ## Todo: Docs
pub fn secp256k1_verify_rsv(message: [u8; 32], signature: [u8; 65], serialize_compressed: bool) -> Result<Vec<u8>, String> {

    match libsecp256k1::Signature::parse_standard_slice(&signature[..64]) {
        Ok(s) => {
            match libsecp256k1::RecoveryId::parse(signature[64]) 
            {
                Ok(rid) => {
                    match libsecp256k1::recover(&libsecp256k1::Message::parse_slice(&message)
                    .expect("Wrong message size!"),
                     &s,
                     &rid)
                     {
                        Ok(pk) => {
                            if serialize_compressed {
                                Ok(pk.serialize_compressed().to_vec())
                            } else {

                                let mut pk_coords: Affine = pk.into();
                                let mut pubkey_concat: Vec<u8> = Vec::new();
                        
                                pk_coords.x.normalize();
                                pk_coords.y.normalize();
                        
                                let x = pk_coords.x.b32();
                                let y = pk_coords.y.b32();
                            
                                pubkey_concat.extend_from_slice(&x);
                                pubkey_concat.extend_from_slice(&y);

                                Ok(
                                    pubkey_concat
                                )
                            }
                        },
                        Err(e) => {
                            Err(format!("Recovery Error: {}", e).to_string())
                        },
                    }
                },
                Err(e) => {
                    return Err(format!("V value failure: {}", e).to_string());
                },
            }
        },
        Err(e) => {
            return Err(format!("Signature Error: {}", e).to_string());
        },
    }
}