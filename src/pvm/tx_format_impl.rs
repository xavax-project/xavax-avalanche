use std::borrow::Borrow;
use std::convert::TryInto;

use super::tx_format::*;
use crate::avm::tx_format::{SECP256K1TransferOutput};
use crate::encoding::cb58::encode_cb58;
use crate::parser::byte_conversion::*;
use crate::parser::parser_traits::Parser;
use crate::primitives::address::Address;

/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    the parser_traits implementation for the pvm, read the note in parser_traits.rs for more info.

    The platform vm seems to be in a state of development, there is little clarity on what some things
    are and how they are used, nontheless the parser should do its job correctly unless I fked something
    up...
*/

impl Parser for SECP256K1OutputOwnersOutput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;

        self.threshold = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_num {
            self.addresses.push(Address{
                address_bytes: raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!"),
                serialized_address: "Todo".to_string(),
            });
            offset += 20;
            index += 1;
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }

    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.threshold.to_be_bytes());

        for i in &self.addresses {
            result.extend_from_slice(&i.address_bytes);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for Validator {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        self.node_id = raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!");
        offset += 20;
        self.start_time = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.endtime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.weight = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.node_id);
        result.extend_from_slice(&self.start_time.to_be_bytes());
        result.extend_from_slice(&self.endtime.to_be_bytes());
        result.extend_from_slice(&self.weight.to_be_bytes());
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for TransferableOutput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        
        self.asset_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length!");
        offset += 32;

        let output_type: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        match output_type {
            7=> {
                let mut o: SECP256K1TransferOutput = SECP256K1TransferOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::SECP256K1TransferOutput(o);
            }
            11 => {
                let mut o: SECP256K1OutputOwnersOutput = SECP256K1OutputOwnersOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::SECP256K1OutputOwnersOutput(o);

            }
            _=> {
                panic!("Incorrect Type ID!")
            }
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.asset_id);

        match &self.output {
            Outputs::SECP256K1TransferOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
            Outputs::SECP256K1OutputOwnersOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for Stake {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;


        let output_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < output_len as usize{
            let mut output: TransferableOutput = TransferableOutput::default();
            output.from_bytes(&raw_payload[offset..], Some(&mut offset));
            self.locked_outs.push(output);
            index += 1;
        }

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for l_o in &self.locked_outs {
            result.extend_from_slice(&l_o.to_bytes());
        }
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for SubnetAuth {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        self.type_id =  extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let sig_indices_len = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < sig_indices_len as usize{
            self.sig_indices.push(extract_u32(raw_payload[offset..=(offset + 3)].borrow()));
            offset += 4;
            index += 1;
        }

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());
        
        result.extend_from_slice(&self.sig_indices.len().to_be_bytes());
        for type_id in &self.sig_indices {
            result.extend_from_slice(&type_id.to_be_bytes());
        }

        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}