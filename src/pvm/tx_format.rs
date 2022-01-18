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
/// A Validator defines information about the validator & delegator which is added to the network, this data type is
/// used in `AddValidatorTx` and `AddDelegatorTx`
/// ___
/// * `node_id` is 20 bytes which is the node ID of the validator
/// * `start_time` is the unix time in which the validator starts validating.
/// * `end_time` is the unix time in which the validator stops validating.
/// * `weight` contains the amount the validator stakes (in nAvax).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Validator {
    pub node_id: Vec<u8>,
    pub start_time: u64, 
    pub endtime: u64,
    pub weight: u64
}
/// # Stake 
/// ___
/// The `Stake` datatype contains a vector of `TransferableOutput`s which are locked for the duration of the staking period, these outputs
/// are locked for the duration of the staking period, at the end of which the outputs will be refunded to their respective addresses. 
/// ___
/// * `locked_outs` A vector of `TransferableOutput` which will be locked for the length of the staking/delegation period.
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stake {
    pub locked_outs: Vec<TransferableOutput>
}

/// # FxID 
/// ___
/// `FxID`, or 'Feature Extension ID' is the ID of the feature extensions running on new/custom chains you can add to a
/// subnet, an example of an FxID is `secp256k1fx`.
/// 
/// ### Note:
/// The documentation for this is still fairly bare-bones since subnets and custom VMs are still being fleshed-out.
/// For reference, please visit [the avax reference documentation](https://docs.avax.network/build/references/platform-transaction-serialization)
/// ___
/// * `fx_id` The bytes of the FxID.
/// 
/// ### Example:
/// for the `secp256k1fx` feature extension, you'd simply put the bytes of `secp256k1fx`.
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FxID {
    pub fx_id: Vec<u8>
}

/// # SubnetAuth 
/// ___
/// `SubnetAuth` contains `sig_indices` and a `type_id` of 0xa (10). SubnetAuth contains the information which define the addresses where
/// the signatures of which can add a validator to a subnet, this is necessary if the subnet you are adding a validator to isn't open-for-all, 
/// and requires a signature/permission of some sort.
/// 
/// ___
/// * `type_id` The typeId for `SubnetAuth` is `0xa`, or `10`.
/// * `sig_indices` sig_indices is a list of unique integers which define the **addresses signing the control signature** to add a valdiator
/// to the subnet. This array must be ordered low-to-high. 
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
/// `AddValidatorTx` adds a validator to the default network (the X-chain, P-chain, C-chain).
/// ___
/// * `base_tx` a base transaction, make sure this tx covers the fees for this type of transaction.
/// * `validator` a `Validator` which contains the information of the validator validating the network,
/// such as the start_time and end_time, the weight, and the node_id.
/// * `stake` the `Stake` which contains TransferableOutputs, which are locked for the entire period the
/// validator is validating, this is should be equivalent to the Weight in the validator.
/// * `RewardsOwner` is an `SECP256K1OutputOwners`, which defines the outputs that will contain the validator
/// rewards by the end of the staking period.
/// * `shares` The percentage of the reward taken from delegators, multiplied by 10,000.
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
/// `AddSubnetValidatorTx` adds validator to a subnetwork within avalanche. Since some subnets are permissioned (requiring signatures from special keys
/// in order do add a validator), a `SubnetAuth` may be necessary.
/// ___
/// * `base_tx` a base transaction, make sure this tx covers the fees for this type of transaction.
/// * `validator` a `Validator` which contains the information of the validator validating the network,
/// such as the start_time and end_time, the weight, and the node_id.
/// * `subnet_id` A 32 byte Subnet ID.
/// * `subnet_auth` the `SubnetAuth` which defines the addresses signing the "control signature" to add a validator to the subnet. I am unsure of the
/// exact implementation details of this, the "control signature" isn't defined anywhere specifically in the documentation as of right now.
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
/// `AddDelegatorTx` adds a delegator to a validator validating the default network (the X-chain, P-chain, C-chain).
/// ___
/// * `base_tx` a base transaction, make sure this tx covers the fees for this type of transaction.
/// * `validator` a `Validator` which contains the information of the validator the delegator will delegate to, or the *delegatee* as its
/// ever so kindly refered as in the official avalanche documentation which desperately needs a better reference regarding subnets...
/// * `stake` the `Stake` which contains TransferableOutputs, which are locked for the entire period the
/// validator is validating, this is should be equivalent to the Weight in the validator.
/// * `RewardsOwner` is an `SECP256K1OutputOwners`, which defines the outputs that will contain the validator
/// rewards by the end of the staking period.
/// * `shares` The percentage of the reward taken from delegators, multiplied by 10,000.
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
/// `CreateChainTx` creates a chain and adds it to a subnet.
/// ___
/// * `base_tx` a base transaction, make sure this tx covers the fees for this type of transaction.
/// * `subnet_id` the ID of the subnet which will validate this blockchain.
/// * `chain_name` a human readable name for the chain, which doesn't have to be unique.
/// * `vm_id` the ID of the Virtual Machine running on the new chain *this can be AVM, TimestampVM, EVM? The documentation needs
/// way more detail on this subject.*
/// * `fx_id` An array of the FxIDs running on this new chain, such as `secp256k1fx`.
/// * `genesis_data` A byte-representation of genesis state of the new chain.
/// * `SubnetAuth` Authorizes this blockchain to be added to the subnet. 
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
/// Creates a subnet which could potentially validate any arbitrary amount of chains all running different Virtual machines.
/// ___
/// * `base_tx` a base transaction with the type_id set to 16 (0x10), make sure this tx covers the fees for this type of transaction.
/// * `rewards_owner` an `SECP256K1OutputOwnersOutput`.
/// ### Note
/// The docs are extremely scarce regarding this Tx, why is there a SECP256K1OutputOwnersOutput? How do permissioned subnets work? I
/// can't write that in these docs until further experimentation.
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateSubnetTx {
    pub base_tx: BaseTx, // typeid is 16 or 0x10
    pub rewards_owner: SECP256K1OutputOwnersOutput
}

/// # StakeableLockIn 
/// ___
/// `StakeableLockIn` is a staked and locked input. The StakeableLockIn can only fund StakeableLockOuts with the same address until its locktime has passed.
/// ___
/// * `type_id` the type ID for this input is 21, or `0x15`.
/// * `locktime` is a long that contains the unix timestamp before which the input can be consumed only to stake. The unix timestamp is specific to the second.
/// * `transferable_in` is a transferable input object which contains the input which is locked & staked.
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StakeableLockIn {
    pub type_id: u32, // 21 or 0x15
    pub locktime: u64,
    pub transferable_in: TransferableInput
}

/// # StakeableLockOut 
/// ___
/// A StakeableLockOut is an output that is locked until its locktime, but can be staked in the meantime.
/// ___
/// * `type_id` the type ID for this input is 22, or `0x16`.
/// * `locktime` is a long that contains the unix timestamp before which the input can be consumed only to stake. The unix timestamp is specific to the second.
/// * `transferable_out` is a transferable output object.
///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StakeableLockOut {
    pub type_id: u32, // 21 or 0x15
    pub locktime: u64,
    pub transferable_out: TransferableOutput
}


/// # SignedTransaction
/// ___
/// A signed transaction is any transaction with the addition of an array of credentials, which contain the signatures
/// which sign the transactions.
/// ___
/// * `codec_id` currently always has to be `0`.
/// * `unsigned_tx` Any arbitrary platform-vm transaction, such as `CreateSubnetTx` or `AddValidatorTx`.
/// * `credentials` A vector of credentials, each credential will be paired with the input in the same index as the
/// credential.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SignedTransaction {
    pub codec_id: u16,
    pub unsigned_tx: Transactions,
    pub credentials: Vec<Credential>
}