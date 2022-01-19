use crate::crypto::engine::elliptic_curves::SECP256K1Keypair;
use crate::avm;


/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    the tx_signer.rs module has traits and implementations for signing avm, pvm, and atomic c-chain Txs.
*/

pub trait TxSigner {
    /// # sign_tx
    /// ___
    /// the sign_tx function will sign the unsigned transaction, but requires you to pass in the
    /// required key-pairs that match the public keys in the UTXOs that are about to be spent in the transaction.
    /// 
    /// On successful signing, the function will return either a `avm::tx_format::SignedTransaction` or
    /// `pvm::tx_format::SignedTransaction` depending on if you are signing a pvm or avm tx.
    fn sign_tx(&self, required_keypairs: SECP256K1Keypair) -> Result<String, String>;
}


impl TxSigner for avm::tx_format::Transactions {
    fn sign_tx(&self, required_keypairs: SECP256K1Keypair) -> Result<String, String> {
        todo!()
    }
}