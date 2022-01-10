use std::borrow::Borrow;
use std::convert::TryInto;

use super::tx_format::*;
use crate::avm::tx_format::{SECP256K1TransferOutput, BaseTx, TransferableInput, ImportTx, ExportTx, Credential};
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

impl Parser for FxID {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;
        self.fx_id = raw_payload[offset..(offset + 31)].to_vec();
        offset += 32;
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.fx_id);
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
        
        result.extend_from_slice(&(self.sig_indices.len() as u32).to_be_bytes());
        for type_id in &self.sig_indices {
            result.extend_from_slice(&type_id.to_be_bytes());
        }

        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for AddValidatorTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut base_tx: BaseTx = BaseTx::default();
        base_tx.from_bytes(raw_payload, Some(&mut offset));
        self.base_tx = base_tx;

        let mut validator: Validator = Validator::default();
        validator.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        let mut stake: Stake = Stake::default();
        stake.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        let mut rewards_owner: SECP256K1OutputOwnersOutput = SECP256K1OutputOwnersOutput::default();
        rewards_owner.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        self.shares = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes());
        result.extend_from_slice(&self.validator.to_bytes());
        result.extend_from_slice(&self.stake.to_bytes());
        result.extend_from_slice(&self.rewards_owner.to_bytes());
        result.extend_from_slice(&self.shares.to_be_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for AddSubnetValidatorTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut base_tx: BaseTx = BaseTx::default();
        base_tx.from_bytes(raw_payload, Some(&mut offset));
        self.base_tx = base_tx;

        let mut validator: Validator = Validator::default();
        validator.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        self.subnet_id = raw_payload[offset..=(offset + 31)].borrow().to_vec();
        offset += 32;

        let mut subnet_auth: SubnetAuth = SubnetAuth::default();
        subnet_auth.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        self.subnet_auth = subnet_auth;

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes());
        result.extend_from_slice(&self.validator.to_bytes());
        result.extend_from_slice(&self.subnet_id[..]);
        result.extend_from_slice(&self.subnet_auth.to_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for AddDelegatorTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut base_tx: BaseTx = BaseTx::default();
        base_tx.from_bytes(raw_payload, Some(&mut offset));
        self.base_tx = base_tx;

        let mut validator: Validator = Validator::default();
        validator.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        let mut stake: Stake = Stake::default();
        stake.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));

        let mut rewards_owner: SECP256K1OutputOwnersOutput = SECP256K1OutputOwnersOutput::default();
        rewards_owner.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));


        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes());
        result.extend_from_slice(&self.validator.to_bytes());
        result.extend_from_slice(&self.stake.to_bytes());
        result.extend_from_slice(&self.rewards_owner.to_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for CreateChainTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut base_tx: BaseTx = BaseTx::default();
        base_tx.from_bytes(raw_payload, Some(&mut offset));
        self.base_tx = base_tx;

        self.subnet_id = raw_payload[offset..=(offset + 31)].to_vec();
        offset += 32;

        let chain_name_len: u16 = extract_u16(raw_payload[offset..=(offset + 1)].borrow());
        offset += 2;

        let mut index: u16 = 0;
        while index < chain_name_len {
            self.chain_name.push(*raw_payload[offset].borrow());
            offset += 1;
            index += 1;
        }

        self.vm_id = raw_payload[offset..=(offset + 31)].to_vec();
        offset += 32;

        let fx_ids_len: u32 = extract_u32(raw_payload[offset..=(offset + 1)].borrow());
        offset += 4;

        let mut index: u32 = 0;
        while index < fx_ids_len {
            self.fx_id.push(FxID { fx_id: raw_payload[offset..=(offset + 31)].to_vec() });
            offset += 32;
            index += 1;
        }

        let genesis_data_len: u32 = extract_u32(raw_payload[offset..=(offset + 1)].borrow());
        offset += 4;

        let mut index: u32 = 0;
        while index < genesis_data_len {
            self.chain_name.push(*raw_payload[offset].borrow());
            offset += 1;
            index += 1;
        }

        let mut subnet_auth: SubnetAuth = SubnetAuth::default();
        subnet_auth.from_bytes(&raw_payload[offset..], Some(&mut offset));

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes());
        result.extend_from_slice(&self.subnet_id);

        result.extend_from_slice(&(self.chain_name.len() as u16).to_be_bytes());
        result.extend_from_slice(&self.chain_name);

        result.extend_from_slice(&self.vm_id);
        result.extend_from_slice(&(self.fx_id.len() as u32).to_be_bytes());

        result.extend_from_slice(&(self.genesis_data.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.genesis_data);

        result.extend_from_slice(&self.subnet_auth.to_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for CreateSubnetTx {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        let mut base_tx: BaseTx = BaseTx::default();
        base_tx.from_bytes(raw_payload, Some(&mut offset));
        self.base_tx = base_tx;

        let mut rewards_owner: SECP256K1OutputOwnersOutput = SECP256K1OutputOwnersOutput::default();
        rewards_owner.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));
        
        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.base_tx.to_bytes());
        result.extend_from_slice(&self.rewards_owner.to_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for StakeableLockIn {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        
        let mut trans_in: TransferableInput = TransferableInput::default();
        trans_in.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));  
        self.transferable_in = trans_in;

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.transferable_in.to_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for StakeableLockOut {
    fn from_bytes(&mut self, raw_payload: &[u8], offset_to_change: Option<&mut usize>) {
        let mut offset: usize = 0;

        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.locktime = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        
        let mut trans_out: TransferableOutput = TransferableOutput::default();
        trans_out.from_bytes(raw_payload[offset..].borrow(), Some(&mut offset));  
        self.transferable_out = trans_out;

        match offset_to_change {
            Some(v) => { *v += offset},
            None => {}
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&self.locktime.to_be_bytes());
        result.extend_from_slice(&self.transferable_out.to_bytes());
        result
    }
    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
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
            12 => {
                let mut tx: AddValidatorTx = AddValidatorTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::AddValidatorTx(tx);
            }
            13 => {
                let mut tx: AddSubnetValidatorTx = AddSubnetValidatorTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::AddSubnetValidatorTx(tx);
            }
            14 => {
                let mut tx: AddDelegatorTx = AddDelegatorTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::AddDelegatorTx(tx);
            }
            15 => {
                let mut tx: CreateChainTx = CreateChainTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::CreateChainTx(tx);
            }
            16 => {
                let mut tx: CreateChainTx = CreateChainTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::CreateChainTx(tx);
            }
            17 => {
                let mut tx: ImportTx = ImportTx::default();
                tx.from_bytes(&raw_payload[offset..], Some(&mut offset));
                self.unsigned_tx = Transactions::ImportTx(tx);
            }
            18 => {
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
            Transactions::ExportTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::ImportTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::CreateChainTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::AddDelegatorTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::AddSubnetValidatorTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::AddValidatorTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            Transactions::CreateSubnetTx(tx) => {
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


