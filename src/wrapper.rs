use schnorrkel::{
    context::signing_context,
    derive::{CHAIN_CODE_LENGTH, ChainCode, Derivation}, Keypair, MiniSecretKey, PublicKey, SecretKey,
    Signature, SignatureError, vrf::{VRFOutput, VRFProof}, ExpansionMode};

// We must make sure that this is the same as declared in the substrate source code.
pub const SIGNING_CTX: &'static [u8] = b"substrate";

/// ChainCode construction helper
pub fn create_cc(data: &[u8]) -> ChainCode {
    let mut cc = [0u8; CHAIN_CODE_LENGTH];

    cc.copy_from_slice(&data);

    ChainCode(cc)
}

/// Keypair helper function.
pub fn create_from_seed(seed: &[u8]) -> Keypair {
    match MiniSecretKey::from_bytes(seed) {
        Ok(mini) => return mini.expand_to_keypair(ExpansionMode::Ed25519),
        Err(_) => panic!("Provided seed is invalid."),
    }
}

/// Keypair helper function.
pub fn create_from_pair(pair: &[u8]) -> Keypair {
    match Keypair::from_bytes(pair) {
        Ok(pair) => return pair,
        Err(_) => panic!("Provided pair is invalid"),
    }
}

/// PublicKey helper
pub fn create_public(public: &[u8]) -> PublicKey {
    match PublicKey::from_bytes(public) {
        Ok(public) => return public,
        Err(_) => panic!("Provided public key is invalid."),
    }
}
/// SecretKey helper
pub fn create_secret(secret: &[u8]) -> SecretKey {
    match SecretKey::from_bytes(secret) {
        Ok(secret) => return secret,
        Err(_) => panic!("Provided private key is invalid."),
    }
}
