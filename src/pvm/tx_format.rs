use serde::{Deserialize, Serialize};

use crate::avm::tx_format::{
SECP256K1TransferOutput, BaseTx, ExportTx, ImportTx, TransferableInput, Credential};
use crate::primitives::address;

/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    the parser_traits implementation for the pvm (platform virtual machine, the P-chain). The P-chain has some
    data types that are identical to the ones on the AVM, so I'm probably going to skip adding those.

    Thinking about how I maybe should've split every datatype + its parser into its seperate file, I might
    do that in the future, we'll see...

    ps: subnets, subnet-chains... this is where the fun stuff happens B)
*/



/// # Outputs
/// ___
/// `Outputs` is an enum that can hold any of the PVM output types. The current output
/// types the PVM Has are:
/// ___
/// * SECP256K1TransferOutput() [see reference](https://docs.avax.network/build/references/avm-transaction-serialization#secp256k1-transfer-output)
/// 
/// * SECP256K1OutputOwnersOutput() [see reference](https://docs.avax.network/build/references/platform-transaction-serialization/#secp256k1-output-owners-output)
/// 
/// 
/// Outputs also implements the Default trait, which will simply return an empty SECP256K1TransferOutput.
/// 
/// ps: the Outputs enum is not to be confused with the regular Output struct, which is another data-type used
/// by the AVM which exists here as well...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Outputs {
    SECP256K1TransferOutput(SECP256K1TransferOutput),
    SECP256K1OutputOwnersOutput(SECP256K1OutputOwnersOutput)
}
impl Default for Outputs {
    fn default() -> Self {
        Self::SECP256K1TransferOutput(SECP256K1TransferOutput::default())
    }
}



/// # TransferableOutput
/// ___
/// TransferableOutput is a wrapper for all Outputs, such as SECP256K1TransferOutput, or
/// SECP256K1OutputOwnersOutput, etc. It essentially contains the data of the specific output along with
/// the asset_id of the output, which is a 32 byte array.
/// 
/// Because of the fact that there are multiple output-types and each transferable-output contains
/// one, we use the `Outputs` Enum which can contain any possible output.
/// ___
/// 
/// * `asset_id` is a 32 byte array which references what asset the `Output` is acting upon.
/// 
/// * `output` is an enum which can contain an arbitrary PVM Output.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransferableOutput {
    pub asset_id: [u8; 32],
    pub output: Outputs
}


/// # SECP256K1OutputOwnersOutput
/// ___
/// An SECP256K1OutputOwnersOutput defines the output that will receive the staking rewards when the lock-up period ends.
/// ___
/// 
///  * `type_id` is the ID for this output type, for SECP256K1OutputOwnersOutput its always 11 (or 0xb).
/// 
///  * `locktime` is the specified unix time of which this output can be spend after.
/// 
///  * `threshold` is an int that specifies the amount of unique signatures needed for this output to be spent.
/// 
///  * `addresses` is a list of addresses that correspond to the Private keys that can be used to spend this output.
/// These addresses must be sorted in lexical order.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SECP256K1OutputOwnersOutput {
    pub type_id: u32,
    pub locktime: u64,
    pub threshold: u32,
    pub addresses: Vec<address::Address>
}


/// # Validator 
/// ___
/// ## TODO: Docs
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Validator {
    pub node_id: Vec<u8>,
    pub start_time: u64, 
    pub endtime: u64,
    pub weight: u64
}
/// # Stake 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stake {
    pub locked_outs: Vec<TransferableOutput>
}

/// # FxID 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FxID {
    pub fx_id: Vec<u8>
}

/// # SubnetAuth 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubnetAuth {
    pub type_id: u32, // type id for this is 10 or 0xa
    pub sig_indices: Vec<u32>,
}

/// # Transactions
/// ___
/// `Transactions` is an enum that can hold any of the PVM Transaction types. The current Transaction Types
///  the PVM Has are:
/// ___
/// * BaseTx
/// * AddValidatorTx
/// * AddSubnetValidatorTx
/// * AddDelegatorTx
/// * CreateChainTx
/// * CreateSubnetTx
/// * ImportTx
/// * ExportTx
/// 
/// Transactions also implement the Default trait, which will simply return an empty BaseTx.
/// 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Transactions {
    BaseTx(BaseTx),
    ExportTx(ExportTx),
    ImportTx(ImportTx),
    AddValidatorTx(AddValidatorTx),
    AddSubnetValidatorTx(AddSubnetValidatorTx),
    AddDelegatorTx(AddDelegatorTx),
    CreateChainTx(CreateChainTx),
    CreateSubnetTx(CreateSubnetTx)
}
impl Default for Transactions {
    fn default() -> Self {
        Self::BaseTx(BaseTx::default())
    }
}

/// # AddValidatorTx 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AddValidatorTx {
    pub base_tx: BaseTx, // TypeID is 0xc or 12
    pub validator: Validator,
    pub stake: Stake,
    pub rewards_owner: SECP256K1OutputOwnersOutput,
    pub shares: u32,
}

/// # AddSubnetValidatorTx 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AddSubnetValidatorTx {
    pub base_tx: BaseTx, // typeID is 0xd or 13
    pub validator: Validator,
    pub subnet_id: Vec<u8>,
    pub subnet_auth: SubnetAuth
}

/// # AddDelegatorTx 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AddDelegatorTx {
    pub base_tx: BaseTx, // typeId is 0xe or 14
    pub validator: Validator,
    pub stake: Stake,
    pub rewards_owner: SECP256K1OutputOwnersOutput,
}

/// # CreateChainTx 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateChainTx {
    pub base_tx: BaseTx, // typeId is 0xf or 15
    pub subnet_id: Vec<u8>,
    pub chain_name: Vec<u8>,
    pub vm_id: Vec<u8>, // How do you get a vm id?
    pub fx_id: Vec<FxID>, // Need better docs on this one
    pub genesis_data: Vec<u8>, // not sure what the implementation details for this is, needs better docs.
    pub subnet_auth: SubnetAuth,
}

/// # CreateSubnetTx 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateSubnetTx {
    pub base_tx: BaseTx, // typeid is 16 or 0x10
    pub rewards_owner: SECP256K1OutputOwnersOutput
}

/// # StakeableLockIn 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StakeableLockIn {
    pub type_id: u32, // 21 or 0x15
    pub locktime: u64,
    pub transferable_in: TransferableInput
}

/// # StakeableLockOut 
/// ___
/// ## TODO: Docs
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StakeableLockOut {
    pub type_id: u32, // 21 or 0x15
    pub locktime: u64,
    pub transferable_out: TransferableOutput
}


/// # SignedTransaction
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SignedTransaction {
    pub codec_id: u16,
    pub unsigned_tx: Transactions,
    pub credentials: Vec<Credential>
}


