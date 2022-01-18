use sha2::{ Sha256, Digest };
use bs58;


///Encodes given bytes to CB58 Encoding.
pub fn encode_cb58(input: &[u8]) -> String {

    //Get Sha256 Checksum
    let input_sha256 = get_sha256_digest(input.clone());
    let checksum: [u8; 4] = 
    [
        input_sha256[input_sha256.len() - 4],
        input_sha256[input_sha256.len() - 3],
        input_sha256[input_sha256.len() - 2],
        input_sha256[input_sha256.len() - 1],
    ];

    let mut check: Vec<u8> = input.clone().to_vec();
    check.append(&mut checksum.to_vec());
    let cb58 = bs58::encode(check).into_string();
    cb58
}
///Decodes given CB58 to a byte payload.
pub fn decode_cb58(input: String) -> Vec<u8> {
    let result = bs58::decode(input).into_vec().expect("Failed");
    let check_body = result.split_at(result.len() - 4);
    check_body.0.to_vec()
}


///Take arbitrary byte vector as input, return Sha256 digest of input.
fn get_sha256_digest(input: &[u8]) ->  Vec<u8>{    
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(input);
    hasher.finalize()[..].to_vec()
}