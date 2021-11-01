use crate::{avm::tx_format::{Credential, TransferableInput, TransferableOutput}, primitives::address};
use serde::{Serialize, Deserialize};

/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    Docs for this will come later on due to me trying to get this stuff done so I can start
    working on the wallet, its a bit troublesome but I'm trying my best! Sorry.
*/




/// # EVMInput
/// ___
/// ## Todo: Docs
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct EVMInput {
    pub address: address::Address,
    pub amount: u64,
    pub asset_id: [u8; 32],
    pub nonce: u64,
}
/// # EVMOutput
/// ___
/// ## Todo: Docs
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct EVMOutput {
    pub address: address::Address,
    pub amount: u64,
    pub asset_id: [u8; 32],
}
/// # AtomicTx
/// ___
/// ## Todo: Docs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AtomicTx {
    ExportTx(ExportTx),
    ImportTx(ImportTx)
}
impl Default for AtomicTx {
    fn default() -> Self {
        Self::ExportTx(ExportTx::default())
    }
}

/// # ExportTx
/// ___
/// ## Todo: Docs
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ExportTx {
    pub type_id: u32,
    pub network_id: u32,
    pub blockchain_id: [u8; 32],
    pub destination_chain: [u8; 32],
    pub inputs: Vec<EVMInput>,
    pub exported_outputs: Vec<TransferableOutput>,
}
/// # ImportTx
/// ___
/// ## Todo: Docs
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ImportTx {
    pub type_id: u32,
    pub network_id: u32,
    pub blockchain_id: [u8; 32],
    pub source_chain: [u8; 32],
    pub imported_inputs: Vec<TransferableInput>,
    pub outputs: Vec<EVMOutput>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SignedTransaction {
    pub codec_id: u16,
    pub atomic_tx: AtomicTx,
    pub credentials: Vec<Credential>
}
