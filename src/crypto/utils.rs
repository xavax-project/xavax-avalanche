use bip39::{Language, Mnemonic, MnemonicType, Seed};


/// # validate_mnemonic
/// ___
/// This function returns true if the passed in mnemonic-phrase is valid, and
/// false if invalid! Simple as that!
/// 
/// * `mnemonic_phrase` A string containing the seed-phrase, seperated by spaces.
/// ___
/// 
/// ### Example:
/// ```
/// use xavax_avalanche::crypto::utils::{validate_mnemonic};
/// 
/// let mnemonic_phrase = "hello world".to_string();
/// let is_valid = validate_mnemonic(&mnemonic_phrase);
/// 
/// if is_valid {
///   //eliminate tyranny
/// }else {
///   //try again until death
/// }
/// 
/// ```
pub fn validate_mnemonic(mnemonic_phrase: &String) -> bool {
    let res = Mnemonic::validate(mnemonic_phrase, Language::English);
    match res {
        Ok(v) => {
            true
        },
        Err(e) => {
            false
        },
    }
}