use std::borrow::Borrow;
use std::convert::TryInto;

use super::tx_format::*;
use crate::encoding::cb58::encode_cb58;
use crate::parser::byte_conversion::*;
use crate::parser::parser_traits::Parser;
use crate::primitives::address::Address;

/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    the parser_traits implementation for the avm, read the note in parser_traits.rs for more info.

    This module will be a fair amount of code but don't be scared! It all does the same thing but for different
    data types...
*/



/* _________________________________________________ Outputs _________________________________________________ */

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
            6 => {
                let mut o: SECP256K1MintOutput = SECP256K1MintOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::SECP256K1MintOutput(o);
            }
            11 => {
                let mut o: NFTTransferOutput = NFTTransferOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::NFTTransferOutput(o);
            }
            10 => {
                let mut o: NFTMintOutput = NFTMintOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::NFTMintOutput(o);
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
            Outputs::SECP256K1MintOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
            Outputs::NFTTransferOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
            Outputs::NFTMintOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for SECP256K1TransferOutput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        self.amount = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;

        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        
        self.threshhold = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;


        let mut index = 0;
        while index < address_num {
            self.addresses.push(Address{
                address_bytes: raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!"),
                serialized_address: None,
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
        result.extend_from_slice(&self.amount.to_be_bytes());
        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.threshhold.to_be_bytes());

        result.extend_from_slice(&(self.addresses.len() as u32).to_be_bytes());
        for i in &self.addresses {
            result.extend_from_slice(&i.address_bytes);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for SECP256K1MintOutput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.threshhold = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_num {
            self.addresses.push(Address{
                address_bytes: raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!"),
                serialized_address: None,
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
        result.extend_from_slice(&self.threshhold.to_be_bytes());

        result.extend_from_slice(&self.addresses.len().to_be_bytes());
        for i in &self.addresses {
            result.extend_from_slice(&i.address_bytes);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for NFTMintOutput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.group_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.threshhold = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_num {
            self.addresses.push(Address{
                address_bytes: raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!"),
                serialized_address: None,
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
        result.extend_from_slice(&self.group_id.to_be_bytes());
        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.threshhold.to_be_bytes());

        result.extend_from_slice(&self.addresses.len().to_be_bytes());
        for i in &self.addresses {
            result.extend_from_slice(&i.address_bytes);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for NFTTransferOutput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.group_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let payload_len = extract_u32(raw_payload[offset..=(offset + 3)].borrow()) as usize;
        offset += 4;
        self.payload = raw_payload[offset..=(offset + payload_len - 1)].try_into().expect("Slice with incorrect length!");
        offset += payload_len;

        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.threshhold = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_num {
            self.addresses.push(Address{
                address_bytes: raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!"),
                serialized_address: None,
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
        result.extend_from_slice(&self.group_id.to_be_bytes());
        result.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.payload[..]);
        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.threshhold.to_be_bytes());

        result.extend_from_slice(&self.addresses.len().to_be_bytes());
        for i in &self.addresses {
            result.extend_from_slice(&i.address_bytes);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for Output {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.threshhold = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_num: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_num {
            self.addresses.push(Address{
                address_bytes: raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length!"),
                serialized_address: None,
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

        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.threshhold.to_be_bytes());

        result.extend_from_slice(&(self.addresses.len() as u32).to_be_bytes());
        for i in &self.addresses {
            result.extend_from_slice(&i.address_bytes);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for UTXO {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {

        let mut offset: usize = 0;

        self.codec_id = extract_u16(raw_payload[offset..=(offset + 1)].borrow());
        offset += 2;

        self.tx_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect lenght! fix ur shit");
        offset += 32;

        self.output_index = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        self.asset_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect lenght! fix ur shit");
        offset += 32;

        let output_type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());

        match output_type_id {
            7=> {
                let mut o: SECP256K1TransferOutput = SECP256K1TransferOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::SECP256K1TransferOutput(o);
            }
            6 => {
                let mut o: SECP256K1MintOutput = SECP256K1MintOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::SECP256K1MintOutput(o);
            }
            11 => {
                let mut o: NFTTransferOutput = NFTTransferOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::NFTTransferOutput(o);
            }
            10 => {
                let mut o: NFTMintOutput = NFTMintOutput::default();
                o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.output = Outputs::NFTMintOutput(o);
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

        result.extend_from_slice(&self.codec_id.to_be_bytes());
        result.extend_from_slice(&self.tx_id);
        result.extend_from_slice(&self.output_index.to_be_bytes());
        result.extend_from_slice(&self.asset_id);

        match &self.output {
            Outputs::SECP256K1TransferOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
            Outputs::SECP256K1MintOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
            Outputs::NFTTransferOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
            Outputs::NFTMintOutput(x) => {
                result.extend_from_slice(&x.to_bytes())
            },
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

/* _________________________________________________ Inputs _________________________________________________ */

impl Parser for TransferableInput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.tx_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length!");
        offset += 32;
        self.utxo_index = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.asset_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length!");
        offset += 32;
        
        let mut i: SECP256K1TransferInput = SECP256K1TransferInput::default();
        i.from_bytes(&raw_payload[offset..], Some(&mut offset));

        self.input = i;
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.tx_id);
        result.extend_from_slice(&self.utxo_index.to_be_bytes());
        result.extend_from_slice(&self.asset_id);
        result.extend_from_slice(&self.input.to_bytes());

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
    
}
impl Parser for SECP256K1TransferInput {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.amount = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;

        //[5, 230, 158, 192]
        let address_index_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_index_num {
            self.address_indices.push(extract_u32(raw_payload[offset..=(offset + 3)].borrow()));
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
        result.extend_from_slice(&self.amount.to_be_bytes());
        result.extend_from_slice(&(self.address_indices.len() as u32).to_be_bytes());
        for i in &self.address_indices {
            result.extend_from_slice(&i.to_be_bytes());
        }
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])       }
}

/* _________________________________________________ Credentials _________________________________________________ */

impl Parser for Credential {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let signature_length: u32 =  extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset +=  4;
        
        let mut sig_index: usize = 0;
        while sig_index < signature_length as usize{
            self.signatures.push(raw_payload[offset..=(offset + 64)].try_into().expect("Slice with incorrect Length!"));
            offset += 65;
            sig_index += 1;
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        //result.extend_from_slice(&(self.signatures.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&(self.signatures.len() as u32).to_be_bytes());
        for i in &self.signatures {
            result.extend_from_slice(&i[..]);
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

/* _________________________________________________ Initial State _________________________________________________ */

impl Parser for InitialState {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.fx_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let output_length: usize = extract_u32(raw_payload[offset..=(offset + 3)].borrow()) as usize;
        offset += 4;

        let mut output_index: usize = 0;
        while output_index < output_length {
            let output_type_id: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());

            match output_type_id {
                7=> {
                    let mut o: SECP256K1TransferOutput = SECP256K1TransferOutput::default();
                    o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                    self.outputs.push(Outputs::SECP256K1TransferOutput(o));
                }
                6 => {
                    let mut o: SECP256K1MintOutput = SECP256K1MintOutput::default();
                    o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                    self.outputs.push(Outputs::SECP256K1MintOutput(o));
                }
                11 => {
                    let mut o: NFTTransferOutput = NFTTransferOutput::default();
                    o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                    self.outputs.push(Outputs::NFTTransferOutput(o));
                }
                10 => {
                    let mut o: NFTMintOutput = NFTMintOutput::default();
                    o.from_bytes(&raw_payload[offset..], Some(&mut offset));
                    self.outputs.push(Outputs::NFTMintOutput(o));
                }
                _=> {
                    panic!("Incorrect Type ID!")
                }
            }

            output_index += 1;
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.fx_id.to_be_bytes());
        result.extend_from_slice(&(self.outputs.len() as u32).to_be_bytes());
        for o in &self.outputs {
            match o {
                Outputs::SECP256K1TransferOutput(x) => {
                    result.extend_from_slice(&x.to_bytes()[..]);
                },
                Outputs::SECP256K1MintOutput(x) => {
                    result.extend_from_slice(&x.to_bytes()[..]);
                },
                Outputs::NFTTransferOutput(x) => {
                    result.extend_from_slice(&x.to_bytes()[..]);
                },
                Outputs::NFTMintOutput(x) => {
                    result.extend_from_slice(&x.to_bytes()[..]);
                },
            }
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

/* _________________________________________________ Transferable Operations _________________________________________________ */

impl Parser for SECP256K1MintOp {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_index_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_index_num {
            self.address_indices.push(extract_u32(raw_payload[offset..=(offset + 3)].borrow()));
            offset += 4;
            index += 1;
        }
        let mut m_o: SECP256K1MintOutput = SECP256K1MintOutput::default();
        m_o.from_bytes(&raw_payload[offset..], Some(&mut offset));
        self.mint_output = m_o;

        let mut t_o: SECP256K1TransferOutput = SECP256K1TransferOutput::default();
        t_o.from_bytes(&raw_payload[offset..], Some(&mut offset));
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&(self.address_indices.len() as u32).to_be_bytes());
        for i in &self.address_indices {
            result.extend_from_slice(&i.to_be_bytes());
        }
        result.extend_from_slice(&self.mint_output.to_bytes()[..]);
        result.extend_from_slice(&self.transfer_output.to_bytes()[..]);

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for NFTMintOp {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_index_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_index_num {
            self.address_indices.push(extract_u32(raw_payload[offset..=(offset + 3)].borrow()));
            offset += 4;
            index += 1;
        }

        self.group_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let payload_len = extract_u32(raw_payload[offset..=(offset + 3)].borrow()) as usize;
        offset += 4;
        self.payload = raw_payload[offset..=(offset + payload_len - 1)].try_into().expect("Slice with incorrect length!");
        offset += payload_len;

        let mut output: Output = Output::default();
        output.from_bytes(&raw_payload[offset..], Some(&mut offset));
        self.output = output;
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());


        result.extend_from_slice(&(self.address_indices.len() as u32).to_be_bytes());
        for i in &self.address_indices {
            result.extend_from_slice(&i.to_be_bytes());
        }
        result.extend_from_slice(&self.group_id.to_be_bytes());
        result.extend_from_slice(&self.payload.len().to_be_bytes());
        result.extend_from_slice(&self.payload[..]);
        result.extend_from_slice(&self.output.to_bytes()[..]);

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for NFTTransferOp {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let address_index_num = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index = 0;
        while index < address_index_num {
            self.address_indices.push(extract_u32(raw_payload[offset..=(offset + 3)].borrow()));
            offset += 4;
            index += 1;
        }

        let mut output: NFTTransferOutput = NFTTransferOutput::default();
        output.from_bytes(&raw_payload[offset..], Some(&mut offset));
        self.nft_transfer_output = output;
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&(self.address_indices.len() as u32).to_be_bytes());
        for i in &self.address_indices {
            result.extend_from_slice(&i.to_be_bytes());
        }
        result.extend_from_slice(&self.nft_transfer_output.to_bytes()[..]);
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

/* _________________________________________________ Unsigned Transaction _________________________________________________ */

impl Parser for BaseTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.network_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.blockchain_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length! fix ur shit...");
        offset += 32;

        let output_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < output_len as usize{
            let mut output: TransferableOutput = TransferableOutput::default();
            output.from_bytes(&raw_payload[offset..], Some(&mut offset));
            self.outputs.push(output);
            index += 1;
        }

        let input_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < input_len as usize{
            let mut input: TransferableInput = TransferableInput::default();
            input.from_bytes(&raw_payload[offset..], Some(&mut offset));
            self.inputs.push(input);
            index += 1;
        }
        
        let memo_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        if memo_len > 0 {
            self.memo = raw_payload[offset..=(offset + memo_len as usize)].borrow().to_vec();
        }


        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&self.network_id.to_be_bytes());
        result.extend_from_slice(&self.blockchain_id);

        result.extend_from_slice(&(self.outputs.len() as u32).to_be_bytes());
        for i in &self.outputs {
            result.extend_from_slice(&i.to_bytes());
        }
        result.extend_from_slice(&(self.inputs.len() as u32).to_be_bytes());
        for i in &self.inputs {
            result.extend_from_slice(&i.to_bytes());
        }
        result.extend_from_slice(&(self.memo.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.memo[..]);
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for CreateAssetTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes()[..]);

        result.extend_from_slice(&(self.name.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.name.as_bytes());

        result.extend_from_slice(&(self.symbol.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.symbol.as_bytes());

        result.extend_from_slice(&self.denomination.to_be_bytes());

        result.extend_from_slice(&(self.initial_states.len() as u32).to_be_bytes());
        for i_s in &self.initial_states {
            result.extend_from_slice(&i_s.to_bytes()[..]);
        }

        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for OperationTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes()[..]);
        match &self.operation {
            TransferOps::SECP256K1MintOp(killme) => {
                result.extend_from_slice(&killme.to_bytes());
            },
            TransferOps::NFTTransferOp(killme) => {
                result.extend_from_slice(&killme.to_bytes());
            },
            TransferOps::NFTMintOp(killme) => {
                result.extend_from_slice(&killme.to_bytes());
            },
        }
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for ImportTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut basetx: BaseTx = BaseTx::default();
        basetx.from_bytes(&raw_payload[..], Some(&mut offset));
        self.base_tx = basetx;
        
        self.source_chain = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length!");
        offset += 32;

        let input_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < input_len as usize{
            let mut input: TransferableInput = TransferableInput::default();
            input.from_bytes(&raw_payload[offset..], Some(&mut offset));
            self.inputs.push(input);
            index += 1;
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes()[..]);
        result.extend_from_slice(&self.source_chain);

        result.extend_from_slice(&(self.inputs.len() as u32).to_be_bytes());
        for i in &self.inputs {
            result.extend_from_slice(&i.to_bytes());
        }
        result
    }

    fn to_cb58(&self) -> String {
        todo!()
    }
}
impl Parser for ExportTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut basetx: BaseTx = BaseTx::default();
        basetx.from_bytes(&raw_payload[..], None);
        offset += basetx.to_bytes().len();
        self.base_tx = basetx;
        self.destination_chain = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length!");
        offset += 32;

        let output_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < output_len as usize{
            let mut output: TransferableOutput = TransferableOutput::default();
            output.from_bytes(&raw_payload[offset..], Some(&mut offset));
            self.outputs.push(output);
            index += 1;
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes()[..]);
        result.extend_from_slice(&self.destination_chain);
        for owo in &self.outputs {
            result.extend_from_slice(&owo.to_bytes());
        }
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
/* _________________________________________________ Signed Transaction _________________________________________________ */

impl Parser for SignedTransaction {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.codec_id = extract_u16(raw_payload[offset..=(offset + 1)].borrow());
        offset += 2;

        let tx_type_id: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        match tx_type_id {
            0 => {
                let mut tx: BaseTx = BaseTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::BaseTx(tx);
            }
            1 => {
                let mut tx: CreateAssetTx = CreateAssetTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::CreateAssetTx(tx);
            }
            2 => {
                let mut tx: OperationTx = OperationTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::OperationTx(tx);
            }
            3 => {
                let mut tx: ImportTx = ImportTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::ImportTx(tx);
            }
            4 => {
                let mut tx: ExportTx = ExportTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::ExportTx(tx);
            }
            _=> {
                panic!("Incorrect tx_id!")
            }
        }
        
        let cred_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        

        let mut index: u32 = 0;
        while index < cred_len {
            let mut c = Credential::default();
            c.from_bytes(&raw_payload[offset..], Some(&mut offset));
            self.credentials.push(c);
            
            index += 1;
        }
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.codec_id.to_be_bytes());
        match &self.unsigned_tx {
            Transactions::BaseTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::CreateAssetTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::OperationTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);                
            },
            Transactions::ExportTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::ImportTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
        }

        result.extend_from_slice(&(self.credentials.len() as u32).to_be_bytes());
        for i in &self.credentials {
            result.extend_from_slice(&i.to_bytes());
        }
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

#[cfg(test)]
mod tests {
    use crate::avm::tx_format::*;
    use crate::encoding::cb58::decode_cb58;
    use crate::parser::parser_traits::Parser;

    #[test]
    fn test_signed_basetx() {
        let tx_bytes: Vec<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 171, 104, 235, 30, 225, 66, 160, 92, 254, 118, 140, 54, 225, 31, 11, 89, 109, 181, 163, 198, 199,
        122, 171, 230, 101, 218, 217, 230, 56, 202, 148, 247, 0, 0, 0, 2, 61, 155, 218, 192, 237, 29, 118, 19, 48, 207, 104, 14, 253, 235, 26, 66, 21, 158, 179,
        135, 214, 210, 149, 12, 150, 247, 210, 143, 97, 187, 226, 170, 0, 0, 0, 7, 0, 0, 0, 0, 5, 245, 225, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
        225, 192, 227, 141, 2, 91, 88, 182, 70, 18, 49, 118, 133, 237, 93, 69, 24, 98, 122, 184, 61, 155, 218, 192, 237, 29, 118, 19, 48, 207, 104, 14, 253,
        235, 26, 66, 21, 158, 179, 135, 214, 210, 149, 12, 150, 247, 210, 143, 97, 187, 226, 170, 0, 0, 0, 7, 0, 0, 0, 0, 101, 38, 42, 64, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 228, 143, 250, 126, 199, 246, 173, 35, 25, 46, 194, 48, 212, 217, 31, 142, 47, 109, 72, 27, 0, 0, 0, 1, 57, 175, 146,
        230, 146, 127, 197, 9, 6, 11, 146, 154, 195, 232, 141, 186, 169, 153, 78, 109, 63, 146, 149, 187, 9, 158, 170, 40, 58, 171, 50, 147, 0, 0, 0, 1, 61,
        155, 218, 192, 237, 29, 118, 19, 48, 207, 104, 14, 253, 235, 26, 66, 21, 158, 179, 135, 214, 210, 149, 12, 150, 247, 210, 143, 97, 187, 226, 170, 0,
        0, 0, 5, 0, 0, 0, 0, 107, 43, 77, 128, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 1, 101, 70, 212, 59, 128, 211, 99, 70, 150,
        149, 36, 50, 2, 199, 122, 9,102, 63, 73, 197, 204, 169, 142, 245, 4, 220, 103, 137, 168, 74, 198, 175, 99, 122, 134, 224, 231, 134, 131, 95, 230, 169,
        27, 4, 37, 140, 54, 226, 159, 153, 108, 53, 190, 86, 3, 238, 157, 25, 43, 139, 9, 162, 144, 31, 1].to_vec();

        let mut tx: SignedTransaction = SignedTransaction::default();
        tx.from_bytes(&tx_bytes[..], None);
        

        assert_eq!(tx.to_bytes(), tx_bytes);
        
        let cb_58_encoded_tx = "111111111UgbbpcKWRe4kWhsrY3aAUd2xUwYYrjWfeSp6g8b9uaE5J35JTCgpYStYfDZp6cGKXv5MrqGWc7urASV1WAusjRvfBwnnmS2SpKoPj6nLbRXJGU636xg3L8Z2kvrPdqaCiQmcN8jbFx1q6Utqy8bp9SQWcGXLb5oBGZspQqL7RorBAbvWmmSG21hg2ewFTd5s4oxS8BFEgQWLxYBR3PapWKR8tpB1PXcMJKaqkHhoXZiN5BDf75bVqmv8kTdFCiKXWQXF6T4f6mZF6gdLzeFuEyYzJNgqTWiVimqLNkrjUGmFEd4zttdFuWJWwomJygTMsn65bD3VgGw6S5bC769K7FqRnziTYhyPDvPb6ucsKXVkGwdZNL6hZDANBuSHfzjWSzGLEvPDcByjG47ZXSDtzrJ5zQUEyd9NwxoAwPjwFBtm8yi1doKu8dZCKPCfr3UgvAmxFc2STVud5wvBS49oZ2assBPqzEP2X4EBGWtSj8mR2F4bJFLM3U5gXhDfne".to_string();
        let a  = decode_cb58(cb_58_encoded_tx.clone());
        //assert_eq!(tx.to_cb58(), cb_58_encoded_tx);
    }


    #[test]
    fn test_signed_import_tx() {
        //Cross chain C-chain ---> X-Chain
        let cb_58_encoded_tx = "1111129nuK2FE1cuYYYcm6aZQw48K8UeDv6MZ8wMY6h77pPNnQ19UqKoSpckAdzrZFHTVS3xV8ypR3Xrvu9ZxkarccDZrWjDSHxSbR8qEdqcJGqtr4T9jXzvUYLc13AdtMsDf7Dq24d7qAxuMhBcxJeAzfKPGw6pVcGvq26eeqvtcmqNGtdXZKN9sFGccpqjKTh1BMUwsd9e5SmKMcwaC3B51WrfrhC4z5m2dctWCAhSHa2fs8zX3seQXHq5dRFKkJz2aDouL2LJw2DRh1HHdKzbPqMXnAPo3KSCLZyBaDXhomDKe2qKUoR3QKS9r1QMv3Ha8WqjNcv9e3KYQJjgcXLJJj5GjvqKW7uhi8rD5SznHFuB5QZYemk555Pb7Vz5TKLTjUPSJA8H9CEtKP3sEp9SZnmyeZp19UpjyjNFUksnaTXfs5tRUKzdsNsxhaNd8y3mgTa1BfiSs".to_string();
        let temp = decode_cb58(cb_58_encoded_tx.clone());

        let mut tx: SignedTransaction = SignedTransaction::default();
        tx.from_bytes(&temp, None);

        assert_eq!(tx.to_cb58(), cb_58_encoded_tx);
    }
}
