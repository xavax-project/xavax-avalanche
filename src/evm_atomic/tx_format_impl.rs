use std::borrow::Borrow;

use super::tx_format::*;
use crate::avm::tx_format::{Credential, TransferableInput, TransferableOutput};
use crate::encoding::cb58::encode_cb58;
use crate::parser::parser_traits::Parser;
use crate::parser::byte_conversion::*;


/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    Working really hard but this stuff is a lot, no docs for this yet! Sowwy (╥﹏╥)
*/


impl Parser for EVMOutput {
    fn from_bytes(&mut self, raw_payload: &[u8]) {
        let mut offset: usize = 0;
        self.address.address_bytes = raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length! diinki pls fix");
        offset += 20;
        self.amount = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.asset_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length! diinki pls fix");
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.address.address_bytes);
        result.extend_from_slice(&self.amount.to_be_bytes());
        result.extend_from_slice(&self.asset_id);
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}
impl Parser for EVMInput {
    fn from_bytes(&mut self, raw_payload: &[u8]) {
        let mut offset: usize = 0;
        self.address.address_bytes = raw_payload[offset..=(offset + 19)].try_into().expect("Slice with incorrect length! diinki pls fix");
        offset += 20;
        self.amount = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
        offset += 8;
        self.asset_id = raw_payload[offset..=(offset + 31)].try_into().expect("Slice with incorrect length! diinki pls fix");
        offset += 32;
        self.nonce = extract_u64(raw_payload[offset..=(offset + 7)].borrow());
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.address.address_bytes);
        result.extend_from_slice(&self.amount.to_be_bytes());
        result.extend_from_slice(&self.asset_id);
        result.extend_from_slice(&self.nonce.to_be_bytes());
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for ExportTx {
    fn from_bytes(&mut self, raw_payload: &[u8]) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.network_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.blockchain_id = raw_payload[offset..=(offset + 31)].try_into().expect("Incorrect slice length!");
        offset += 32;
        self.destination_chain = raw_payload[offset..=(offset + 31)].try_into().expect("Incorrect slice length!");
        offset += 32;

        let inputs_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < inputs_len as usize {
            let mut input: EVMInput = EVMInput::default();
            input.from_bytes(&raw_payload[offset..]);
            self.inputs.push(input.clone());
            offset += input.to_bytes().len();
            index += 1;
        }

        let output_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < output_len as usize {
            let mut output: TransferableOutput = TransferableOutput::default();
            output.from_bytes(&raw_payload[offset..]);
            self.exported_outputs.push(output.clone());
            offset += output.to_bytes().len();
            index += 1;
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&self.network_id.to_be_bytes());
        result.extend_from_slice(&self.blockchain_id);
        result.extend_from_slice(&self.destination_chain);
        
        result.extend_from_slice(&(self.inputs.len() as u32).to_be_bytes());
        for i in &self.inputs {
            result.extend_from_slice(&i.to_bytes()[..]);
        }

        result.extend_from_slice(&(self.exported_outputs.len() as u32).to_be_bytes());
        for t_o in &self.exported_outputs {
            result.extend_from_slice(&t_o.to_bytes()[..]);
        }
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}

impl Parser for ImportTx {
    fn from_bytes(&mut self, raw_payload: &[u8]) {
        let mut offset: usize = 0;
        self.type_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.network_id = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;
        self.blockchain_id = raw_payload[offset..=(offset + 31)].try_into().expect("Incorrect slice length!");
        offset += 32;
        self.source_chain = raw_payload[offset..=(offset + 31)].try_into().expect("Incorrect slice length!");
        offset += 32;

        let input_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < input_len as usize {
            let mut input: TransferableInput = TransferableInput::default();
            input.from_bytes(&raw_payload[offset..]);
            self.imported_inputs.push(input.clone());
            offset += input.to_bytes().len();
            index += 1;
        }

        let imported_inputs_len: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        offset += 4;

        let mut index: usize = 0;
        while index < imported_inputs_len as usize {
            let mut output: EVMOutput = EVMOutput::default();
            output.from_bytes(&raw_payload[offset..]);
            self.outputs.push(output.clone());
            offset += output.to_bytes().len();
            index += 1;
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.type_id.to_be_bytes());
        result.extend_from_slice(&self.network_id.to_be_bytes());
        result.extend_from_slice(&self.blockchain_id);
        result.extend_from_slice(&self.source_chain);
        
        result.extend_from_slice(&(self.imported_inputs.len() as u32).to_be_bytes());
        for i in &self.imported_inputs {
            result.extend_from_slice(&i.to_bytes()[..]);
        }

        result.extend_from_slice(&(self.outputs.len() as u32).to_be_bytes());
        for o in &self.outputs {
            result.extend_from_slice(&o.to_bytes()[..]);
        }
        result
    }

    fn to_cb58(&self) -> String {
        encode_cb58(&self.to_bytes()[..])
    }
}


impl Parser for SignedTransaction {
    fn from_bytes(&mut self, raw_payload: &[u8]) {
        let mut offset: usize = 0;
        self.codec_id = extract_u16(raw_payload[offset..=(offset + 1)].borrow());
        offset += 2;

        let tx_type_id: u32 = extract_u32(raw_payload[offset..=(offset + 3)].borrow());
        match tx_type_id {
            0 => {
                let mut tx: ImportTx = ImportTx::default();
                tx.from_bytes(&raw_payload[offset..]);
                self.atomic_tx = AtomicTx::ImportTx(tx.clone());
                offset += tx.to_bytes().len();
            }
            1 => {
                let mut tx: ExportTx = ExportTx::default();
                tx.from_bytes(&raw_payload[offset..]);
                self.atomic_tx = AtomicTx::ExportTx(tx.clone());
                offset += tx.to_bytes().len();
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
            c.from_bytes(&raw_payload[offset..]);
            self.credentials.push(c.clone());
            offset += c.to_bytes().len();
            
            index += 1;
        }
        
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.codec_id.to_be_bytes());
        match &self.atomic_tx {
            AtomicTx::ImportTx(tx) => {
                result.extend_from_slice(&tx.to_bytes()[..]);
            },
            AtomicTx::ExportTx(tx) => {
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
    use crate::encoding::cb58::decode_cb58;
    use crate::evm_atomic::tx_format::*;
    use crate::parser::parser_traits::Parser;


    /* damn the test worked firs try, I didn't expect it but I did really need it  (ಥ﹏ಥ)*/

    //ps: the cb_58_encoded_tx is a transaction generated by the default avalanche web wallet that I packed-sniffed and got,
    //used it here as a bench-mark.
    #[test]
    fn test_signed_transactions() {

        //Cross chain X-chain ---> C-Chain
        let cb_58_encoded_tx = "111111111879MAjcAYPQY7BPt8JDK9oJrKf88WfnUdX7dPpKTZRb9MxnwzRmG9MiCPndDfJMKpU3xjSLoboA15Y1no9J3vdc9YV3WAVmamjDMGYxffQ3jBsEB2yUEa5mvhwkAHpZMtnWRvnU3EimZY377FCCFfvAc3RVesPJoe5cuZTjAk1MH3hz23xXeURUYSEBWHsNd4ByxLFNejRtRyV9AN3bmWTrWN4F6fXvKwd1uicCw5FSjJUfQDZpBD84htGu4q7KUCXHBT4ze3caembBiJ1BMHRgG4Wa4kvfaWwrBpBMvv3eKxWoHgja4pJ8jckq64N8wCEt3bS5xBZrBbrsg7xCmr6vHG7P95ahepcQDGwu9ANCWfyxeoBCHTeXiHDzR9PAXk4fzuRZ4J196k1i6NDrZxHm9ohRa".to_string();
        let temp = decode_cb58(cb_58_encoded_tx.clone());

        let mut tx: SignedTransaction = SignedTransaction::default();
        tx.from_bytes(&temp);

        assert_eq!(tx.to_cb58(), cb_58_encoded_tx);

        //Cross chain C-chain ---> X-Chain
        let cb_58_encoded_tx = "111119TRcmX2yWov6MZAQjGpUwLsrQkofTZg9FjNiX2vryYzsKbvwVu8EVobeZ9NqJo9AcYDrLiB8u1QQduF7Gpu6ktiij117A5PNjRinDMMm77VDaT7ZG8CFEjNSQT3TiQ28eyBZ5rWKsTd74phC6zS7TRZtqXecsie5sgUxF5hSZfPNMcZpTpRPtvauuFx6F85bpV8HPBfEXzZYafczqGn1S8SzCKa5QodPsR9y5KX25rYb2xbBVLDeRA8fV2NPyxHwA6kKbJ7vNdyV9w4Gv1NAT5HRRVrPoDC2SE3SPAzytVmSUPgjwLaq3zUQ1iD6z8hER9E9idGT2dbvdyWx2YiC46YFH1R4wkx6kEWZURSKc54vqv9y13pe5tyYJFRuZ3wqWiPV7qcpq1M1GfyZUubjY8323TcATRjNJ".to_string();
        let temp = decode_cb58(cb_58_encoded_tx.clone());

        let mut tx: SignedTransaction = SignedTransaction::default();
        tx.from_bytes(&temp);

        assert_eq!(tx.to_cb58(), cb_58_encoded_tx);
    }
}