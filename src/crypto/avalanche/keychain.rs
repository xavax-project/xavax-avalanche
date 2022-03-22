use std::collections::HashMap;

use bech32::{FromBase32, ToBase32};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use hex::ToHex;
use tiny_hderive::bip32::ExtendedPrivKey;

use serde::{Serialize, Deserialize};

use crate::{primitives::address::Address, crypto::engine::{elliptic_curves::{SECP256K1Keypair, MnemonicKeypair, secp256k1_verify_rsv, Signing, secp256k1_sign_rsv}, hash_functions::{get_ripemd160_hash, get_sha256_hash}}};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DerivationPath {
    path: String
}
impl DerivationPath {
    /// # set_path
    /// ___
    /// set_path() will set the derivation path, which can be used to derive addresses from a seed.
    /// the parameters you pass into set_path() will define the derivation path, which derives the address derived from the seed.
    /// 
    /// * `coin_type: Option<u32>` this is the CoinType value of the derivation path, the coin_type used for avalanche X & P chain
    ///    addresses is 9000. The default coin-type will be 9000 if you choose to not manually select one.
    /// 
    /// * `account_index: u32` The account_index will create a new hardened keypair, unlike `address_index`. This can be used to create
    /// new deposit accounts every deposit, to increase annonymity. 
    /// 
    /// * `change_index: u32` this is used to define if the address will be used to receive "change" by spending a UTXO, most HD wallets do this. This
    /// value is usually either 0 or 1 depending on if the address is used as a change address or not.
    /// 
    /// * `address_index: u32` address_index will derive a new soft keypair, unlike `account_index`.
    /// ____
    /// ## Note
    /// 
    /// The way derivation-paths are used are different from wallet-to-wallet, but as long as the same seed is used and the derivation path is known,
    /// you can generate the address.
    /// 
    /// the Avalanche default wallet uses: `m/44'/9000'/index'/change/0` where `index` simply refers to the index of the address, while `change` is 1
    /// for change addresses, and 0 for non-change addresses.
    pub fn generate_path(coin_type: Option<u32>, account_index: u32, change_index: u32, address_index: u32) -> Self {
        let coin: u32;
        match coin_type {
            Some(v) => {
                coin = v;
            },
            None => {
                coin = 9000; //9000 is the coin_type value of avalanche.
            },
        }
        DerivationPath{
            path: format!("m/44'/{0}'/{1}'/{2}/{3}", coin, account_index, change_index, address_index),
        }
    }
    pub fn get_path(&self) -> String {
        return self.path.clone();
    }
}

#[derive(Debug, Clone, Default)]
pub struct Keys{
    keychain: Vec<SECP256K1Keypair>,
    human_readable_part: String
}


impl MnemonicKeypair for Keys {
    fn generate_mnemonic_phrase() -> String {
        //The entropy is from thread_rng which is cryptographically secure, (it implements CryptoRng).
        Mnemonic::new(MnemonicType::Words24, Language::English).to_string()
    }
    fn generate_seed_entropy(seed_phrase: String) -> Vec<u8> {
        todo!();
    }
    fn generate_keypair(seed_phrase: String, derivation_path: &str) -> SECP256K1Keypair {
        let seed = Seed::new(&Mnemonic::from_phrase(&seed_phrase, Language::English).expect("Failure!"), "");
        let priv_key: ExtendedPrivKey = ExtendedPrivKey::derive(seed.as_ref(), derivation_path).expect("Failure!");
    
        let public_key:  libsecp256k1::PublicKey = libsecp256k1::PublicKey::from_secret_key(&libsecp256k1::SecretKey::parse_slice(&priv_key.secret()).expect("Failure!"));
        let address: Vec<u8> = get_ripemd160_hash(&get_sha256_hash(&public_key.serialize_compressed()));
        
        
        SECP256K1Keypair {
            public_key: public_key.serialize_compressed().to_vec(),
            address: Address{ address_bytes: address, serialized_address: None },
            private_key: priv_key.secret().to_vec(),
            derivation_path: derivation_path.to_string(),
        }
        
    }
}
impl Keys {
    /// # get_key()
    /// ___
    /// get_key() will return an SECP256K1Keypair with the given parameters.
    pub fn get_key(seed_phrase: String, human_readable_part: &str, derivation_path: DerivationPath) -> SECP256K1Keypair {
        let mut key: SECP256K1Keypair = Keys::generate_keypair(seed_phrase.clone(), &derivation_path.get_path());
        key.address.serialized_address = Some(encode_bech32_address(human_readable_part, key.address.address_bytes.clone()));
        key
    }
    /* 
    pub fn insert_new_keypair(&mut self, ){
        let deriv_path: String;
        match self.is_change {
            true => {
                deriv_path = format!("m/44'/9000'/{}'/1/{}", self.account_index, self.latest_keypair.1.0);
            },
            false => {
                deriv_path = format!("m/44'/9000'/{}'/0/{}", self.account_index, self.latest_keypair.1.0);
            },
        }
        let new_kp = AvalancheKeys::generate_keypair(self.seed_phrase.clone(), &deriv_path);

        self.latest_keypair = (
            new_kp.clone().0,
            DerivationIndex(self.latest_keypair.1.0 + 1),
        );
        self.keychain.insert(new_kp.0.0, new_kp.1);
    }
    pub fn get_keypair(&self, address: Address) -> Result<SECP256K1Keypair, String> {
        match self.keychain.get(&address.0) {
            Some(kp) => {
                Ok(kp.clone())
            },
            None => {
                Err(format!("Keychaing doesn't have a key-pair with the address: {}", address.0).to_string())
            },
        }
    }
     */
}


impl Signing for Keys {
    fn sign(private_key: &Vec<u8>, data_preimage: Vec<u8>) -> Vec<u8> {
        let hashed_data = get_sha256_hash(&data_preimage);
        secp256k1_sign_rsv(private_key[..].try_into().expect("Expeded 32 bytes!"),
         &hashed_data).to_vec()
    }

    fn sign_interop_message(private_key: &Vec<u8>, data_preimage: Vec<u8>) -> Vec<u8> {
        let mut payload_to_sign: Vec<u8> = Vec::new();
        //This below is just the Message prefix used in Avalanche, if converted to a UTF8 string it would be:
        /*
            0x1aAvalanche Signed Message:\n
            {messagesize}{messagebytes}
        */
        payload_to_sign.extend_from_slice(&[0x1a_u8]);
        payload_to_sign.extend_from_slice(&[0x41_u8, 0x76_u8, 0x61_u8, 0x6c_u8, 0x61_u8, 0x6e_u8, 0x63_u8,
        0x68_u8, 0x65_u8, 0x20_u8, 0x53_u8, 0x69_u8, 0x67_u8, 0x6e_u8, 0x65_u8, 0x64_u8, 0x20_u8, 0x4d_u8,
        0x65_u8, 0x73_u8, 0x73_u8, 0x61_u8, 0x67_u8, 0x65_u8, 0x3a_u8, 0x0a_u8]);
        payload_to_sign.extend_from_slice(&(data_preimage.len() as u32).to_be_bytes());
        payload_to_sign.extend_from_slice(&data_preimage);

        Keys::sign(&private_key, payload_to_sign)
    }

    fn verify(self, signature: Vec<u8>, message_preimage: Vec<u8>) -> Result<Address, String> {
        let hashed_data = get_sha256_hash(&message_preimage);
        match secp256k1_verify_rsv(hashed_data.try_into().unwrap(),
         signature.try_into().expect("Incorrect Signature!"), true)
        {
            Ok(pk) => {
                Ok(
                    Address {
                        address_bytes: get_ripemd160_hash(&get_sha256_hash(&pk)),
                        serialized_address: None,
                    }
                )
            },
            Err(e) => {
                Err(format!("Error verifying Signature: {}", e).to_string())
            },
        }
    }

    
    fn verify_message(self, signature: Vec<u8>, message_preimage: Vec<u8>) -> Result<Address, String> {
        let mut payload_to_verify: Vec<u8> = Vec::new();
        //This below is just the Message prefix used in Avalanche, if converted to a UTF8 string it would be:
        /*
            0x1aAvalanche Signed Message:\n
            {messagesize}{messagebytes}
        */
        payload_to_verify.extend_from_slice(&[0x1a_u8]);
        payload_to_verify.extend_from_slice(&[0x41_u8, 0x76_u8, 0x61_u8, 0x6c_u8, 0x61_u8, 0x6e_u8, 0x63_u8,
        0x68_u8, 0x65_u8, 0x20_u8, 0x53_u8, 0x69_u8, 0x67_u8, 0x6e_u8, 0x65_u8, 0x64_u8, 0x20_u8, 0x4d_u8,
        0x65_u8, 0x73_u8, 0x73_u8, 0x61_u8, 0x67_u8, 0x65_u8, 0x3a_u8, 0x0a_u8]);
        payload_to_verify.extend_from_slice(&(message_preimage.len() as u32).to_be_bytes());
        payload_to_verify.extend_from_slice(&message_preimage);

        let hashed_data = get_sha256_hash(&payload_to_verify);

        
        match secp256k1_verify_rsv(hashed_data.try_into().unwrap(),
         signature.try_into().expect("Incorrect Signature!"), true)
        {
            Ok(pk) => {
                Ok(
                    Address {
                        address_bytes: get_ripemd160_hash(&get_sha256_hash(&pk)),
                        serialized_address: None,
                    }
                )
            },
            Err(e) => {
                Err(format!("Error verifying Signature: {}", e).to_string())
            },
        }
    }
}


pub fn encode_bech32_address(human_readable_part: &str, address: Vec<u8>) -> String{
    bech32::encode(human_readable_part, address.to_base32(), bech32::Variant::Bech32).expect("Failed bech32 creation!")
}
pub fn decode_bech32_address(address: String) -> (String, Vec<u8>){
    let r = bech32::decode(&address).expect("Failed bech32 creation!");
    (r.0, Vec::<u8>::from_base32(&r.1).expect("Failed converting u5 to u8!"))
}