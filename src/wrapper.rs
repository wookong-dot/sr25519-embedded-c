use alloc::boxed::Box;
use core::panic::PanicInfo;
use core::slice;
use schnorrkel::context::{signing_context,attach_rng}; // SigningContext,SigningTranscript
use super::*;


/// Must have for no std on embedded
///
/// ```
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
	loop {}
}

/// Must have for no std on embedded
///
/// ```
#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
	loop {}
}

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

/// Data stuct to return to C
/// libc could not find types, so use RUST type directly
/// return as *u8 
#[repr(C)]
pub struct sr_data {
	/// status code of op
	pub status: u32,
	/// data len to indicate the useful data in buffer
	pub len: usize,
	/// data buffer for sr
	pub data: [u8; 96],
}
/// global variable in C to indicate heap
/// must define in C again
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }
    unsafe { &mut __sheap }
}
 
/// Alloc memory for sr lib
///
/// ```
#[no_mangle]
pub unsafe extern "C" fn sr_init() {
	let start: usize = heap_start() as usize;
	let size: usize = 1024; // in bytes
	ALLOCATOR.init(start, size);
}

/// Free memory used, keypair ptr and signature ptr should be free
///
/// # Inputs
///
/// * `b` ptr return by Box.
/// ```
#[no_mangle]
pub unsafe extern "C" fn sr_free(b: *mut u8) {
	let u = Box::from_raw(b);
	drop(u);
}


/// Sign message by keypairs
///
/// # Inputs
///
/// * `messages` plain text message.
/// * `keypair` is derived from seed
/// * 'random' is give by C code
///
/// # Returns
///
/// * A `*u8` ptr tp sr_data struct` value data is the signature len = 64
///
/// ```
#[no_mangle]
pub unsafe extern "C" fn sr_sign(
	message: *const u8,
	len: usize,
	random: *const u8,
	keypair: *const u8,
) -> *mut u8 {
	let context = signing_context(b"good");
	let mut sr_data = sr_data {
		status: STATUS_NOK,
		len: 0,
		data: [0u8;BUFFER_LEN],
	};
	let keypair =
		match Keypair::from_bytes(slice::from_raw_parts(keypair, PUB_KEY_LEN + PRI_KEY_LEN)) {
			Ok(pair) => pair,
			Err(_) => {
				sr_data.status = ERR_KEYPAIR;
				return Box::into_raw(Box::new(sr_data)) as *mut u8;
			}
		};

	let message_bytes: &[u8] = slice::from_raw_parts(message, len);
	let trng_bytes: &[u8] = slice::from_raw_parts(random, PUB_KEY_LEN);

	 struct ZeroFakeRng;
    impl RngCore for ZeroFakeRng {
        fn next_u32(&mut self) -> u32 {  panic!()  }
        fn next_u64(&mut self) -> u64 {  panic!()  }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for i in dest.iter_mut() {  *i = 0;  }
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }
    impl CryptoRng for ZeroFakeRng {}

	let signature: Signature = keypair.sign(attach_rng(context.bytes(&message_bytes[..]), ZeroFakeRng));
	let signature_bytes = signature.to_bytes();

	let mut i = 0;
	while i < SIGN_LEN {
		sr_data.data[i] = signature_bytes[i];
		i = i + 1;
	}

	sr_data.status = STATUS_OK;
	sr_data.len = SIGN_LEN;

	Box::into_raw(Box::new(sr_data)) as *mut u8
}