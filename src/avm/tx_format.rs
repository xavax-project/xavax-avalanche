use serde::{Serialize, Deserialize};

use crate::primitives::{address};


/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    tx_format in x_chain contains all Transcations in native Rust data struct form. These data-types
    are used to interact with the X-chain (AVM) in the avalanche network.

    See https://docs.avax.network/build/references/avm-transaction-serialization for more information.

    Note: The p-chain/platform vm/pvm contains a lot of the same primitives that the avm has, for that reason
    only the pvm unique data-structs will exist in the pvm transaction format module, meanwhile the avm transaction format
    will contain all of the avm things... I hope that makes sense? Maybe I should mention that in the pvm module and not here
    but whatever leave me alone...

    Also, thx ava-labs for creating really good documentation for the transaction format! <3
*/



/* _________________________________________________ Outputs _________________________________________________ */


/// # TransferableOutput
/// ___
/// TransferableOutput is a wrapper for all Outputs, such as SECP256K1TransferOutput, or
/// NFTMintOutput, etc. It essentially contains the data of the specific output along with
/// the asset_id of the output, which is a 32 byte array.
/// 
/// Because of the fact that there are multiple output-types and each transferable-output contains
/// one, we use the `Outputs` Enum which can contain any possible output.
/// ___
/// 
/// * `asset_id` is a 32 byte array which references what asset the `Output` is acting upon.
/// 
/// * `output` is an enum which can contain an arbitrary AVM Output.
/// 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TransferableOutput {
    pub asset_id: [u8; 32],
    pub output: Outputs
}

/// # Outputs
/// ___
/// `Outputs` is an enum that can hold any of the AVM output types. The current output
/// types the AVM Has are:
/// ___
/// * SECP256K1TransferOutput() [see reference](https://docs.avax.network/build/references/avm-transaction-serialization#secp256k1-transfer-output)
/// 
/// * SECP256K1MintOutput()     [see reference](https://docs.avax.network/build/references/avm-transaction-serialization#secp256k1-mint-output)
/// 
/// * NFTTransferOutput()       [see reference](https://docs.avax.network/build/references/avm-transaction-serialization#nft-transfer-output)
/// 
/// * NFTMintOutput()           [see reference](https://docs.avax.network/build/references/avm-transaction-serialization#nft-mint-output)
/// 
/// Outputs also implements the Default trait, which will simply return an empty SECP256K1TransferOutput.
/// 
/// ps: the Outputs enum is not to be confused with the regular Output struct, which is another data-type used
/// by the AVM which exists here as well...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Outputs {
    SECP256K1TransferOutput(SECP256K1TransferOutput),
    SECP256K1MintOutput(SECP256K1MintOutput),
    NFTTransferOutput(NFTTransferOutput),
    NFTMintOutput(NFTMintOutput)
}
impl Default for Outputs {
    fn default() -> Self {
        Self::SECP256K1TransferOutput(SECP256K1TransferOutput::default())
    }
}

/// # SECP256K1TransferOutput
/// ___
/// An SECP256K1TransferOutput is the most basic output, it allows for sending any specified Asset, such as AVAX, to any
/// collection of Addresses. It also allows a locking-period, i.e the addresses cannot spend the output before said time passes.
/// This time is specified in unix seconds. The addresses correspond to the addresses that need to sign the output to C o n s u m e i t
/// ___
/// 
///  * `type_id` is the ID for this output type, for SECP256K1TransferOutput its always 7.
/// 
///  * `amount` is the amount of the asset that this output has.
/// 
///  * `locktime` is the specified unix time of which this output can be spend after.
/// 
///  * `threshold` is an int that specifies the amount of unique signatures needed for this output to be spent.
/// 
///  * `addresses` is a list of addresses that correspond to the Private keys that can be used to spend this output.
/// These addresses must be sorted in lexical order.

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SECP256K1TransferOutput {
    pub type_id: u32,
    pub amount: u64,
    pub locktime: u64,
    pub threshhold: u32,
    pub addresses: Vec<address::Address>
}

/// # SECP256K1MintOutput
/// ___
///  A SECP256K1MintOutput is a minted output that is owned by an arbitrary amount of addresses.
/// 
///  Now, I'm gonna be honest:
///  I'm not entirely sure on how this output is specifically used, but with it you can mint Avalanche native tokens
/// *(ANTs?)* 
/// 
///  First you would create the ANT with a CreateAssetTx and InitialState, then use SECP256K1MintOperation to Mint
///  the ANT And then send an arbitrary amount to any address with SECP256K1TransferOutputs.
/// ___
/// 
///  * `type_id` is the ID for this output type, for SECP256K1TransferOutput its always 6.
/// 
///  * `locktime` is the specified unix time of which this output can be spend after.
/// 
///  * `threshold` is an int that specifies the amount of unique signatures needed for this output to be spent.
/// 
///  * `addresses` is a list of addresses that correspond to the Private keys that can be used to spend this output.
/// 
/// These addresses must be sorted in lexical order.
///
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SECP256K1MintOutput {
    pub type_id: u32,
    pub locktime: u64,
    pub threshhold: u32,
    pub addresses: Vec<address::Address>
}

/// # NFTMintOutput
/// ___
///  A NFTMintOutput is an NFT MINT output that is owned by an arbitrary amount of addresses *(the addresses need to sign the
/// output in order to mint the NFTs?).*
/// ___
///
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NFTMintOutput {
    pub type_id: u32,
    pub group_id: u32,
    pub locktime: u64,
    pub threshhold: u32,
    pub addresses: Vec<address::Address>
}

/// # NFTTransferOutput
/// ___
///  A NFTTransferOutput is an output that is an AvalancheNativeToken (ANT) NFT,
///  which is owned by an arbitrary amount of addresses.
/// ___
/// 
/// * `type_id` is the ID for this output type, for NFTTransferOutputs its always 11.
/// 
/// * `group_id` is an integer that specifies the group this NFT was issued to.
/// 
/// * `payload` is an arbitrary amount of bytes no longer than 1024 bytes. This can be
/// a json payload, or just any string...
/// 
/// * `locktime` is the specified unix time of which this output can be spend after.
/// 
/// * `addresses`is a list of addresses that correspond to the Private keys that can be used to spend this output.
/// These addresses must be sorted in lexical order.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NFTTransferOutput {
    pub type_id: u32,
    pub group_id: u32,
    pub payload: Vec<u8>,
    pub locktime: u64,
    pub threshhold: u32,
    pub addresses: Vec<address::Address>
}


/// # Output
/// ___
/// Output is a minimal Output object which only contains the basics. This is used by the NFTMintOp.
/// ___
/// 
/// * `locktime` is the specified unix time of which this output can be spend after.
/// * `threshold` is an int that specifies the amount of unique signatures needed for this output to be spent.
/// * `addresses`is a list of addresses that correspond to the Private keys that can be used to spend this output.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Output {
    pub locktime: u64,
    pub threshhold: u32,
    pub addresses: Vec<address::Address>
}

/// # UTXO Datatype
/// A UTXO is a standalone representation of a transaction output.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UTXO {
    pub codec_id: u16,
    pub tx_id: [u8; 32],
    pub output_index: u32,
    pub asset_id: [u8; 32],

    pub output: Outputs,
}

/* _________________________________________________ Inputs _________________________________________________ */

/// # TransferableInput
/// ___
/// A TransferableInput, much like TransferableOutputs is a wrapper for all inputs, Although
/// there is currently only **One** input that can be used with TransferableOutputs which is the
/// `SECP256K1TransferInput`.
/// ___
/// 
/// * `output_consumer_owners` is a list of addresses that correspond to the addresses that own the Output that this
/// Input consumes! This is purely a helper data-type for the XAVAX-API and NOT Part of the message payload!
/// 
/// * `tx_id` is a 32 byte array that defines which transaction this Input is consuming an Output from.
/// TxId's are the SHA256 Hash of the bytes of the Signed transaction.
/// 
/// * `utxo_index` is an integer that defines whcih UTXO This input is consuming in the specified transaction (specified
/// by the tx_id). This is obviously needed because a transaction can have many different outputs!
/// 
/// * `asset_id` is a 32 byte array that specifies which asset this input references.
/// 
/// * `input` is an SECP256K1TransferInput, the AVM currently only supports this input for transferable inputs,
/// so no fancy enums for different input types here...

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TransferableInput {
    pub output_consumer_owners: Vec<address::Address>, //Not part of the message payload!

    pub tx_id: [u8; 32],
    pub utxo_index: u32,
    pub asset_id: [u8; 32],
    pub input: SECP256K1TransferInput
}

/// # SECP256K1TransferInput
/// ___
/// A SECP256K1TransferInput Gives us the power to spend an unspent SECP256K1TransferOutput.
/// ___
/// 
/// * `type_id` is the ID for this input type, which for SECP256K1TransferInputs is always 5.
/// 
/// * `amount` is the amount of an asset that this Input spends, this must be equal to the amount
/// in the UTXO This output consumes!
/// 
/// * `address_indices` is a list of Unique integers that define the Private keys that are being used to 
/// spend the UTXO, this is because a UTXO can have multiple addresses that can spend the UTXO, each integer here
/// defines the index in the list of addresses that can spend the UTXO in the UTXO Adress list. This list must
/// be sorted from low to high.
/// 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SECP256K1TransferInput {
    pub type_id: u32,
    pub amount: u64,
    pub address_indices: Vec<u32>,
}

/* _________________________________________________ Credentials _________________________________________________ */


/// # Credential
/// ___
/// A Credential is a type_id and a list of signatures, there are two type IDs for Credentials:
/// ___
/// * SECP256K1 Credential, where the `type_id` is **9**
/// 
/// * NFT Credential, where the `type_id` is **14**
/// 
/// * `signatures` is a list of signatures.

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Credential {
    pub type_id: u32,
    pub signatures: Vec<Vec<u8>>
}

/* _________________________________________________ Initial State _________________________________________________ */

/// # InitialState
/// ___
/// InitialState describes the initial sate of an ANT asset when its created, in other words the genesis of the asset.
/// Different avanache native tokens can have different features, such as Fungible assets and NFTS. This is what the fx_id
/// describes.
/// 
/// The InitialState can for instance be used to describe the creation of an NFT, where the Outputs specified dictate the
/// amount of the NFT and who to send it to! (and the payload of the NFT).
/// 
/// All that is bundled together in a `CreateAssetTx`
/// ___
/// 
/// * `fx_id` is an integer that defines what type of ANT this is. For SECP256K1 Assets its `0`, for NFTs its `1`
/// * `outputs` is a list of `Outputs`, which is an enum of all kind of outputs.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct InitialState {
    pub fx_id: u32,
    pub outputs: Vec<Outputs>
}

/* _________________________________________________ Transferable Operations _________________________________________________ */


/// # TransferOps
/// ___
/// `TransferOps` is an enum that can hold any of the AVM Transfer operations types. The current Operations
///  the AVM Has are:
/// ___
/// * SECP256K1MintOperation()
/// * NFTMintOp()
/// * NFTTransferOk()
/// 
/// TransferOps also implement the Default trait, which will simply return an empty SECP256K1TransferOutput.
/// 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransferOps {
    SECP256K1MintOp(SECP256K1MintOp),
    NFTTransferOp(NFTTransferOp),
    NFTMintOp(NFTMintOp),
}
impl Default for TransferOps {
    fn default() -> Self {
        Self::SECP256K1MintOp(SECP256K1MintOp::default())
    }
}

/// # SECP256K1MintOp
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SECP256K1MintOp{
    pub type_id: u32,
    pub address_indices: Vec<u32>,
    pub mint_output: SECP256K1MintOutput,
    pub transfer_output: SECP256K1TransferOutput
}

/// # NFTMintOp
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NFTMintOp {
    pub type_id: u32,
    pub address_indices: Vec<u32>,
    pub group_id: u32,
    pub payload: Vec<u8>,
    pub output: Output,
}

/// # NFTTransferOp
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NFTTransferOp {
    pub type_id: u32,
    pub address_indices: Vec<u32>,
    pub nft_transfer_output: NFTTransferOutput,
}

/* _________________________________________________ Unsigned Transactions _________________________________________________ */


/// # Transactions
/// ___
/// `Transactions` is an enum that can hold any of the AVM Transaction types. The current Transaction Types
///  the AVM Has are:
/// ___
/// * BaseTx
/// * CreateAssetTx
/// * OperationTx
/// * ExportTx
/// * ImportTx
/// 
/// Transactions also implement the Default trait, which will simply return an empty BaseTx.
/// 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Transactions {
    BaseTx(BaseTx),
    CreateAssetTx(CreateAssetTx),
    OperationTx(OperationTx),
    ExportTx(ExportTx),
    ImportTx(ImportTx)
}
impl Default for Transactions {
    fn default() -> Self {
        Self::BaseTx(BaseTx::default())
    }
}


/// # BaseTx
/// ___
/// A BaseTx (Base transaction) is a primitive for all other transaction types, and also just
/// a basic transaction that consumes outputs and creates input. Ps: The `type_id` changes when
/// you use a basetx as a primitive for another transaction.
/// ___
/// * `type_id` is the ID for the Transaction Types. For a BaseTx the ID is 0.
/// 
/// * `network_id` is an int that defines which network this transaction is meant to be
/// 
/// issued to. This value is meant to support transaction routing (whatever that is) and is not
/// designed for replay attack prevention. *(tf is transaction routing? I'm just quoting the docs for this one...)*
/// 
/// * `blockchain_id` is a 32 byte array which defines the blockchain this transaction is issued to. This is used for
/// replay prevention for transactions that could be potentially valid across different networks and blockchains!
/// 
/// * `outputs` is an list of transferable output objects. These outputs must be sorted in lexical order by their 
/// serialized representation in hex, and the total output quantity must be less than OR equal to the total quantity of
/// each asset consumed in the inputs MINUS the transaction fees. (The value of the outputs the inputs consume cannot be
/// more than themselves :p)
/// 
/// * `inputs` is a list of transferable input objects. Inputs are first sorted in lexical order, then by the utxo_index from
/// low to high (in case there are two inputs that with the same tx_id).
/// 
/// * `memo` memo is just an arbitrary payload of up to 256 bytes, could be a note to mother, or a payment reference...

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BaseTx {
    pub type_id: u32,
    pub network_id: u32,
    pub blockchain_id: [u8; 32],
    pub outputs: Vec<TransferableOutput>,
    pub inputs: Vec<TransferableInput>,
    pub memo: Vec<u8>
}

/// # CreateAssetTx
/// ___
/// CreateAssetTx contains the data necessary to create an entirely new ANT (Avalanche Native Token).
/// These ANTs can be NFTs, for Fungible tokens. Tokens can have multiple different features, the vector of
/// `IntialState` describes those feature sets this asset supports. The IntialState also describes the Genesis
/// of the asset: i.e the first set of Outputs of the Assets.
/// ___
/// * `base_tx` is a BaseTx with its `type_id` set to `1`
/// 
/// * `name` is a human readable name which defines the name of the asset. This string must only consist of ASCII characters
/// 
/// and not be longer than 128 Characters.
/// * `symbol` is a human readable symbol which has to consist a maximum of `4` ASCII characters
/// 
/// * `denomination` is a byte which defines the divisibility of the asset this transaction will Create. The denomination
/// cannot be more than 32. (the AVAX Asset has a denomination of 9, which means its divisible into billionths).
/// 
/// * `initial_states` is a vector of `InitialState`. These define the feature sets the asset will have as well as the Genesis
/// of the asset.
/// 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CreateAssetTx {
    pub base_tx: BaseTx,
    pub name: String,
    pub symbol: String,
    pub denomination: u8,
    pub initial_states: Vec<InitialState>
}

/// # OperationTx
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OperationTx {
    pub base_tx: BaseTx,
    pub operation: TransferOps
}

/// # ImportTx
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ImportTx {
    pub base_tx: BaseTx,
    pub source_chain: [u8; 32],
    pub inputs: Vec<TransferableInput>
}

/// # ExportTx
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ExportTx {
    pub base_tx: BaseTx,
    pub destination_chain: [u8; 32],
    pub outputs: Vec<TransferableOutput>
}

/* _________________________________________________ Signed Transactions _________________________________________________ */

/// # SignedTransaction
/// ### Todo: Docs, i'm currently busy doing other things docs take too m uch time 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SignedTransaction {
    pub codec_id: u16,
    pub unsigned_tx: Transactions,
    pub credentials: Vec<Credential>
}


/* _________________________________________________ Genesis Asset _________________________________________________ */

/* I might add GenesisAsset later, its practically useless right now and I need to write a lot of code so I'll skip it for now... */
//save me
