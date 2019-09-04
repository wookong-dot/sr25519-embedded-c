use core::slice;
use crate::wrapper::{create_from_pair,create_cc,create_public,SIGNING_CTX,create_from_seed};
use core::ptr;
use schnorrkel::{
    context::signing_context,
    derive::{CHAIN_CODE_LENGTH, ChainCode, Derivation}, Keypair, MiniSecretKey, PublicKey, SecretKey,
    Signature, SignatureError, vrf::{VRFOutput, VRFProof}, ExpansionMode,context::attach_rng};
use rand_core::{CryptoRng, RngCore};
use exrng::ExternalRng;


pub const SR_SEED_LEN: usize = 32;
pub const SR_CHAINCODE_LEN: usize = 32;
pub const SR_PUBLIC_LEN: usize = 32;
pub const SR_SECRET_LEN: usize = 64;
pub const SR_SIGNATURE_LEN: usize = 64;
pub const SR_KEYPAIR_LEN: usize = 96;
pub const SR_VRF_OUTPUT_LEN: usize = 32;
pub const SR_VRF_PROOF_LEN: usize = 64;

pub const SR_OK:u32 = 0;
pub const SR_FAIL:u32 = 1;
pub const SR_PAIR_FAIL:u32 = 2;
pub const SR_VERIFY_FAIL:u32 = 3;
pub const SR_SIGN_FORMET:u32 = 4;

#[no_mangle]
pub unsafe extern "C" fn sr_derive_keypair_hard(
    keypair_out: *mut u8,
    pair_ptr: *const u8,
    cc_ptr: *const u8,
) {
    let pair = slice::from_raw_parts(pair_ptr, SR_KEYPAIR_LEN as usize);
    let cc = slice::from_raw_parts(cc_ptr, SR_CHAINCODE_LEN as usize);
    let kp = create_from_pair(pair)
        .secret
        .hard_derive_mini_secret_key(Some(create_cc(cc)), &[])
        .0
        .expand_to_keypair(ExpansionMode::Ed25519);

    ptr::copy(kp.to_bytes().as_ptr(), keypair_out, SR_KEYPAIR_LEN as usize);
}

#[no_mangle]
pub unsafe extern "C" fn sr_sign(
	message: *const u8,
	len: usize,
	random: *const u8,
	keypair: *const u8,
    sig_out: *mut u8,
) -> u32 {
	let context = signing_context(b"good");
	let keypair =
		match Keypair::from_bytes(slice::from_raw_parts(keypair, SR_PUBLIC_LEN + SR_SECRET_LEN)) {
			Ok(pair) => pair,
			Err(_) => {
				return {SR_PAIR_FAIL};
			}
		};
	let message_bytes = slice::from_raw_parts(message, len);
	let trng_bytes = slice::from_raw_parts(random, SR_PUBLIC_LEN);
    let signature: Signature = keypair.sign(
        attach_rng(
            context.bytes(&message_bytes[..]), 
            ExternalRng{
                rng_bytes:ExternalRng::copy_into_array(trng_bytes),
                len:32}
                ));
     ptr::copy(signature.to_bytes().as_ptr(), sig_out, SR_SIGNATURE_LEN as usize);  
    { SR_OK }
}

#[allow(unused_attributes)]
#[no_mangle]
pub unsafe extern "C" fn sr_verify(
    signature_ptr: *const u8,
    message_ptr: *const u8,
    message_length: usize,
    public_ptr: *const u8,
) -> u32 {
    let public = slice::from_raw_parts(public_ptr, SR_PUBLIC_LEN as usize);
    let signature = slice::from_raw_parts(signature_ptr, SR_SIGNATURE_LEN as usize);
    let message = slice::from_raw_parts(message_ptr, message_length as usize);
    let signature = match Signature::from_bytes(signature) {
        Ok(signature) => signature,
        Err(_) => return SR_SIGN_FORMET,
    };
    if create_public(public).verify_simple(SIGNING_CTX, message, &signature).is_ok()
        { SR_OK }
    else
        { SR_VERIFY_FAIL }
}

#[no_mangle]
pub unsafe extern "C" fn sr_keypair_from_seed(keypair_out: *mut u8, seed_ptr: *const u8)->u32 {
    let seed = slice::from_raw_parts(seed_ptr, SR_SEED_LEN as usize);
    let kp = create_from_seed(seed);
    ptr::copy(kp.to_bytes().as_ptr(), keypair_out, SR_KEYPAIR_LEN as usize);
    { SR_OK }
}