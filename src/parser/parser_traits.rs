
/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    parser_traits.rs contains traits for turning avalanche virtual machine transaction format data-types
    to a byte-payload that the AvalancheGo node can read and understand. The traits, Data-types, and Implementation
    of the traits are all seperated in their seperate modules for c l e a n l i n e s s and o r g a n i z a t i o n purposes.
    implementations for the Parser trait are found in the same folder as each respective transaction format module, for instance:

    |-src
       |-avm
           |-tx_format.rs
           |-tx_format_impl.rs   <----- This is the Parser implementation for the avm tx_format.

    
    The same applies for the p_chain(pvm) and c_chain_atomic. The coreeth implementation uses a completely different
    encoding and therefore isn't even included in this entire Crate. For that use the xavax_eth(I think that's what I'm naming it) crate which implements
    data-structs for eth transactions and encoding/decoding/parsing for coreeth as well. 
*/


/// # Parser
/// Parser is a trait that has functions which parse raw byte payloads to avalanche transaction format
/// data structs as well as parse the structs to raw-payloads that are ready to be sent to avalanche nodes.
/// Parser also has a function that encodes the raw bytes to CB58 encoding, which is the encoding used by
/// the avalanche network.
/// 
/// **Parser has three functions:**
/// ```ignore,
/// //Takes in a raw_payload and attempt to parse it into the data-structure of self that implements the trait.
/// fn from_bytes(&mut self, raw_payload: &[u8]); 
/// 
/// //Returns self into a serialized byte-payload form using the avalanche serialization format
/// fn to_bytes(&self) -> &[u8]; 
/// 
/// //Returns the from_bytes() payload serialized in CB58.
/// fn to_cb58(&self) -> String; 
/// ```
/// ___
/// * The byte-payload is serialized using [the Avalanche serialization format](https://docs.avax.network/build/references/serialization-primitives)
/// 
/// 
/// * CB58 encoding is practically the same as Base 58 Check but with a different checksum algorithm, you can find a CB58 implementation in the cb58.rs module
/// that this crate has.
/// ___
/// 
/// ## Example:
/// 
/// ```ignore,
/// //Some arbitrary data-type that has the Parser trait implemented.
/// pub struct Output {
///     type_id: u32,
///     some_data: [u8; 32]
/// }
/// 
/// //Use the Parser trait to get the Output datatype serialized in bytes.
/// let mut bababooey: Output = Output::default();
/// let bytes: Vec<u8> = bababooey.to_bytes();
/// 
/// ```
pub trait Parser {
    fn from_bytes(&mut self, raw_payload: &[u8]);
    fn to_bytes(&self) -> Vec<u8>;
    fn to_cb58(&self) -> String;
}