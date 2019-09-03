use core::slice;
use schnorrkel::context::{signing_context,attach_rng}; // SigningContext,SigningTranscript
use super::*;
use core::ptr;



/// Must have for no std on embedded
///
/// ```
// #[panic_handler]
// fn panic(_info: &PanicInfo<'_>) -> ! {
// 	loop {}
// }

// /// Must have for no std on embedded
// ///
// /// ```
// #[alloc_error_handler]
// fn foo(_: core::alloc::Layout) -> ! {
// 	loop {}
// }

const PUB_KEY_LEN: usize = 32;
const PRI_KEY_LEN: usize = 64;
const SIGN_LEN: usize = 64;
/// total buffer len 96
pub const BUFFER_LEN: usize = 96;
/// fn success
pub const STATUS_OK: u32 = 0;
/// fn fail
pub const STATUS_NOK: u32 = 1;
const ERR_KEYPAIR: u32 = 2;
const ERR_PRIKEY: u32 = 3;
const ERR_SIGBYTE: u32 = 4;


/// global variable in C to indicate heap
/// must define in C again
// #[inline]
// pub fn heap_start() -> *mut u32 {
//     extern "C" {
//         static mut __sheap: u32;
//     }
//     unsafe { &mut __sheap }
// }
 
/// Alloc memory for sr lib
///
/// ```
// #[no_mangle]
// pub unsafe extern "C" fn sr_init() {
// 	let start: usize = heap_start() as usize;
// 	let size: usize = 1024; // in bytes
// 	ALLOCATOR.init(start, size);
// }

/// Free memory used, keypair ptr and signature ptr should be free
///
/// # Inputs
///
/// * `b` ptr return by Box.
/// ```
// #[no_mangle]
// pub unsafe extern "C" fn sr_free(b: *mut u8) {
// 	let u = Box::from_raw(b);
// 	drop(u);
// }
// We must make sure that this is the same as declared in the substrate source code.
const SIGNING_CTX: &'static [u8] = b"substrate";

/// ChainCode construction helper
fn create_cc(data: &[u8]) -> ChainCode {
    let mut cc = [0u8; CHAIN_CODE_LENGTH];

    cc.copy_from_slice(&data);

    ChainCode(cc)
}

/// Keypair helper function.
fn create_from_seed(seed: &[u8]) -> Keypair {
    match MiniSecretKey::from_bytes(seed) {
        Ok(mini) => return mini.expand_to_keypair(ExpansionMode::Ed25519),
        Err(_) => panic!("Provided seed is invalid."),
    }
}

/// Keypair helper function.
fn create_from_pair(pair: &[u8]) -> Keypair {
    match Keypair::from_bytes(pair) {
        Ok(pair) => return pair,
        Err(_) => panic!("Provided pair is invalid"),
    }
}

/// PublicKey helper
fn create_public(public: &[u8]) -> PublicKey {
    match PublicKey::from_bytes(public) {
        Ok(public) => return public,
        Err(_) => panic!("Provided public key is invalid."),
    }
}
/// SecretKey helper
fn create_secret(secret: &[u8]) -> SecretKey {
    match SecretKey::from_bytes(secret) {
        Ok(secret) => return secret,
        Err(_) => panic!("Provided private key is invalid."),
    }
}

/// Size of input SEED for derivation, bytes
pub const SR25519_SEED_SIZE: usize = 32;

/// Size of CHAINCODE, bytes
pub const SR25519_CHAINCODE_SIZE: usize = 32;

/// Size of SR25519 PUBLIC KEY, bytes
pub const SR25519_PUBLIC_SIZE: usize = 32;

/// Size of SR25519 PRIVATE (SECRET) KEY, which consists of [32 bytes key | 32 bytes nonce]
pub const SR25519_SECRET_SIZE: usize = 64;

/// Size of SR25519 SIGNATURE, bytes
pub const SR25519_SIGNATURE_SIZE: usize = 64;

/// Size of SR25519 KEYPAIR. [32 bytes key | 32 bytes nonce | 32 bytes public]
pub const SR25519_KEYPAIR_SIZE: usize = 96;

/// Size of VRF output, bytes
pub const SR25519_VRF_OUTPUT_SIZE: usize = 32;

/// Size of VRF proof, bytes
pub const SR25519_VRF_PROOF_SIZE: usize = 64;




#[no_mangle]
pub unsafe extern "C" fn sr25519_derive_keypair_hard(
    keypair_out: *mut u8,
    pair_ptr: *const u8,
    cc_ptr: *const u8,
) {
    let pair = slice::from_raw_parts(pair_ptr, SR25519_KEYPAIR_SIZE as usize);
    let cc = slice::from_raw_parts(cc_ptr, SR25519_CHAINCODE_SIZE as usize);
    let kp = create_from_pair(pair)
        .secret
        .hard_derive_mini_secret_key(Some(create_cc(cc)), &[])
        .0
        .expand_to_keypair(ExpansionMode::Ed25519);

    ptr::copy(kp.to_bytes().as_ptr(), keypair_out, SR25519_KEYPAIR_SIZE as usize);
}

fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
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
		match Keypair::from_bytes(slice::from_raw_parts(keypair, PUB_KEY_LEN + PRI_KEY_LEN)) {
			Ok(pair) => pair,
			Err(_) => {
				return {1};
			}
		};

	let message_bytes = slice::from_raw_parts(message, len);
	let trng_bytes = slice::from_raw_parts(random, PUB_KEY_LEN);

	let signature: Signature = keypair.sign(
        attach_rng(
            context.bytes(&message_bytes[..]), 
            exrng::ExternalRng{
                rng_bytes:copy_into_array(trng_bytes),
                len:32}
                ));

    ptr::copy(signature.to_bytes().as_ptr(), sig_out, SR25519_SIGNATURE_SIZE as usize);
    { 0 }
}