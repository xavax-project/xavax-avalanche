
use serde::{Deserialize, Serialize};
use bech32::*;


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    pub address_bytes: Vec<u8>,
    pub serialized_address: Option<String>
}

impl Address {
    /// # from_bech32_address
    /// ___
    /// `from_bech32_address` will parse a Bech32 address into an `Address` struct,
    /// which contains both the serialized address and the address in Bytes, this is
    /// useful for generating transactions that are ready to be sent to Avalanche Nodes.
    /// 
    /// ## Example
    /// ```
    /// use xavax_avalanche::primitives::address::Address;
    /// //Turn a serialized Address *(with or without the 'X-'/'P-' delimiters, it doesn't matter)* into an Address struct,
    /// //which contains both the serialized address and the bytes of the address!.
    /// let address: Address = Address::from_bech32_address("X-fuji1zcm8wjm8swx7c9hpd2mvlt9jrwyv82rpmrucwc".to_string());
    /// 
    /// ```
    /// 
    pub fn from_bech32_address(address: String) -> Self{
        let mut result: Address = Address::default();
        match address.chars().nth(1).unwrap() {
            '-' => {
                let new_address: String = address.clone()[2..].to_string();
                result.serialized_address = Some(new_address.clone());
                result.address_bytes = Vec::<u8>::from_base32(&bech32::decode(&new_address)
                .expect("Incorrect Bech32 Address!").1)
                .expect("Not Base32!")[..].try_into().expect("Slice with incorrect length!");
            }
            _ => {
                result.serialized_address = Some(address.clone());
                result.address_bytes = Vec::<u8>::from_base32(&bech32::decode(&address)
                .expect("Incorrect Bech32 Address!").1)
                .expect("Not Base32!")[..].try_into().expect("Slice with incorrect length!");
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use bech32::ToBase32;

    use super::Address;

    #[test]
    fn address_test() {
        let address: Address = Address::from_bech32_address("X-fuji1zcm8wjm8swx7c9hpd2mvlt9jrwyv82rpmrucwc".to_string());
        assert_eq!(address.serialized_address, bech32::encode("fuji", &address.address_bytes.to_base32(), bech32::Variant::Bech32).unwrap());

        let address: Address = Address::from_bech32_address("fuji1zcm8wjm8swx7c9hpd2mvlt9jrwyv82rpmrucwc".to_string());
        assert_eq!(address.serialized_address, bech32::encode("fuji", &address.address_bytes.to_base32(), bech32::Variant::Bech32).unwrap());
    }
}