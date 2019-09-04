use core::slice;
use core::ptr;
use schnorrkel::{
    context::{signing_context,attach_rng},
    Keypair, MiniSecretKey, PublicKey,
    Signature, ExpansionMode};
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
pub const SR_SIGN_FORMAT:u32 = 4;

pub const SIGNING_CTX: &'static [u8] = b"substrate";

#[no_mangle]
pub unsafe extern "C" fn sr_sign(
	message: *const u8,
	len: usize,
	random: *const u8,
	keypair: *const u8,
    sig_out: *mut u8,
) -> u32 {
	let context = signing_context(b"substrate");
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


#[no_mangle]
pub unsafe extern "C" fn sr_verify(
    signature_ptr: *const u8,
    message_ptr: *const u8,
    message_length: usize,
    public_ptr: *const u8,
) -> bool {
    let public = slice::from_raw_parts(public_ptr, SR_PUBLIC_LEN as usize);
    let signature = slice::from_raw_parts(signature_ptr, SR_SIGNATURE_LEN as usize);
    let message = slice::from_raw_parts(message_ptr, message_length as usize);
    let signature = match Signature::from_bytes(signature) {
        Ok(signature) => signature,
        Err(_) => return false,
    };

    let pk =  match PublicKey::from_bytes(public) {
        Ok(public) => public,
        Err(_) => return false,
    };
    pk.verify_simple(SIGNING_CTX, message, &signature).is_ok()

}

#[no_mangle]
pub unsafe extern "C" fn sr_keypair_from_seed(keypair_out: *mut u8, seed_ptr: *const u8)->u32 {
    let seed = slice::from_raw_parts(seed_ptr, SR_SEED_LEN as usize);
    let msk = match MiniSecretKey::from_bytes(seed){
         Ok(mk) => mk,
         Err(_) => return SR_SIGN_FORMAT,
    };
    let kp = msk.expand_to_keypair(ExpansionMode::Ed25519);
    let kp_bytes = kp.to_bytes();
    ptr::copy(kp_bytes.as_ptr(), keypair_out, SR_KEYPAIR_LEN as usize);
    { SR_OK }
}

#[cfg(test)]
mod test{
use super::*;
#[test]
fn test_sign_verify(){
    let mut keypair_out:[u8;96] = [0u8;96];
    let seed:[u8;32] = [0u8;32];
    let mut rv = unsafe { sr_keypair_from_seed(keypair_out.as_mut_ptr(),seed.as_ptr()) };
    assert_eq!(rv,0 );
	let message_bytes: [u8;32] = [0u8;32];
	let rng_bytes: [u8;32] = [0u8;32];
    let mut sign_out:[u8;64] = [0u8;64];
    rv = unsafe {sr_sign(
                        message_bytes.as_ptr(),
                        32,
                        rng_bytes.as_ptr(),
                        keypair_out.as_ptr(),
                        sign_out.as_mut_ptr()
                ) };
    assert_eq!(rv,0 );
    let brv = unsafe {sr_verify(sign_out.as_ptr(),message_bytes.as_ptr(),32,keypair_out[64..].as_ptr())};
    assert_eq!(brv,true);

    }
}
