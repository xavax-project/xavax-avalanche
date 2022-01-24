use crate::avm::tx_format::Credential;
use crate::crypto::engine::elliptic_curves::{SECP256K1Keypair, secp256k1_sign_rsv};
use crate::{avm, pvm};
use crate::crypto::engine::hash_functions::get_sha256_hash;
use crate::parser::parser_traits::Parser;
use tracing;
use tracing::log::trace;


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
    fn sign_tx(&self, required_keypairs: &Vec<SECP256K1Keypair>) -> Result<String, String>;
}

//AvalancheVM tx signing.
impl TxSigner for avm::tx_format::Transactions {
    /// # sign_tx
    /// ___
    /// the sign_tx function will sign the unsigned transaction, but requires you to pass in the
    /// required key-pairs that match the public keys in the UTXOs that are about to be spent in the transaction.
    /// 
    /// On successful signing, the function will return either a CB58 encoded`avm::tx_format::SignedTransaction`,
    /// you can simply decode this encoded string into a SignedTransaction by:
    /// 
    /// * Decoding into a byte-vector by inputting the CB58 encoded string into the `decode_cb58()` function.
    /// * use `SignedTransaction.from_bytes(/*byte_array_here*/)` to parse the bytes.
    fn sign_tx(&self, required_keypairs: &Vec<SECP256K1Keypair>) -> Result<String, String> {

        trace!("Attempting to sign Transaction with: {} Given Keypairs.", required_keypairs.len());

        
        let mut signed_tx: avm::tx_format::SignedTransaction = avm::tx_format::SignedTransaction::default();
        signed_tx.codec_id = 0;
        
        let mut bytes_to_sign: Vec<u8> = vec![];
        bytes_to_sign.extend_from_slice(&signed_tx.codec_id.to_be_bytes());

        match &self {
            avm::tx_format::Transactions::BaseTx(base_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&base_tx.to_bytes());

                //Go through every input in the tx.
                for input in &base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            avm::tx_format::Transactions::CreateAssetTx(create_asset_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&create_asset_tx.to_bytes());

                //Go through every input in the tx.
                for input in &create_asset_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            avm::tx_format::Transactions::OperationTx(op_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&op_tx.to_bytes());

                //Go through every input in the tx.
                for input in &op_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            avm::tx_format::Transactions::ExportTx(export_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&export_tx.to_bytes());

                //Go through every input in the tx.
                for input in &export_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            avm::tx_format::Transactions::ImportTx(import_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&import_tx.to_bytes());

                //Go through every input in the tx.
                for input in &import_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
        }
    }
}

//PlatformVM tx signing.
impl TxSigner for pvm::tx_format::Transactions {
    /// # sign_tx
    /// ___
    /// the sign_tx function will sign the unsigned transaction, but requires you to pass in the
    /// required key-pairs that match the public keys in the UTXOs that are about to be spent in the transaction.
    /// 
    /// On successful signing, the function will return either a CB58 encoded`avm::tx_format::SignedTransaction`,
    /// you can simply decode this encoded string into a SignedTransaction by:
    /// 
    /// * Decoding into a byte-vector by inputting the CB58 encoded string into the `decode_cb58()` function.
    /// * use `SignedTransaction.from_bytes(/*byte_array_here*/)` to parse the bytes.
    fn sign_tx(&self, required_keypairs: &Vec<SECP256K1Keypair>) -> Result<String, String> {

        trace!("Attempting to sign Transaction with: {} Given Keypairs.", required_keypairs.len());

        
        let mut signed_tx: avm::tx_format::SignedTransaction = avm::tx_format::SignedTransaction::default();
        signed_tx.codec_id = 0;
        
        let mut bytes_to_sign: Vec<u8> = vec![];
        bytes_to_sign.extend_from_slice(&signed_tx.codec_id.to_be_bytes());

        match &self {
            pvm::tx_format::Transactions::BaseTx(base_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&base_tx.to_bytes());

                //Go through every input in the tx.
                for input in &base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::ExportTx(export_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&export_tx.to_bytes());

                //Go through every input in the tx.
                for input in &export_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::ImportTx(import_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&import_tx.to_bytes());

                //Go through every input in the tx.
                for input in &import_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::AddValidatorTx(add_validator_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&add_validator_tx.to_bytes());

                //Go through every input in the tx.
                for input in &add_validator_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::AddSubnetValidatorTx(add_subnet_validator_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&add_subnet_validator_tx.to_bytes());

                //Go through every input in the tx.
                for input in &add_subnet_validator_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::AddDelegatorTx(add_delegator_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&add_delegator_tx.to_bytes());

                //Go through every input in the tx.
                for input in &add_delegator_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::CreateChainTx(create_chain_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&create_chain_tx.to_bytes());

                //Go through every input in the tx.
                for input in &create_chain_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
            pvm::tx_format::Transactions::CreateSubnetTx(create_subnet_tx) => {
                //Add the transaction bytes to the bytes array that we'll sign.
                bytes_to_sign.extend_from_slice(&create_subnet_tx.to_bytes());

                //Go through every input in the tx.
                for input in &create_subnet_tx.base_tx.inputs {
                    if &input.output_consumer_owners.len() <= &0 {
                        return Err("The given transaction doesn't have any specified output_consumer_owners! This is necessary
                        to use the default signing function of xavax_avalanche.".to_string());
                    }
                    //Use the output_consumer_owners "helper-datatype" to get the necessary addresses needed to sign the tx.
                    for output in &input.output_consumer_owners {
                        //Credential object that we'll push the signatures needed for the tx to.
                        let mut c: Credential = Credential {
                            type_id: 9, //type_id is alsways 9 for credentials.
                            signatures: vec![],
                        };
                        for keypair in required_keypairs {
                            if output.address_bytes == keypair.address.address_bytes {
                                //Sign the Transaction Data, and push the to the Credential.
                                c.signatures.push(
                                    secp256k1_sign_rsv(&keypair.private_key[..].try_into().expect("Incorrect Private Key!"),
                                    &get_sha256_hash(&bytes_to_sign)).to_vec()
                                );
                            }  
                        }
                        signed_tx.credentials.push(c);
                    }
                }
                return Ok(signed_tx.to_cb58());
            },
        }
    }
}