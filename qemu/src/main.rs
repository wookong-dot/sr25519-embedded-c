#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate sr25519_embedded_c;
extern crate alloc;
extern crate alloc_cortex_m;
extern crate panic_halt;

use core::slice;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use sr25519_embedded_c::{
    api::{sr_keypair_from_seed,sr_sign,sr_verify}
};


use alloc_cortex_m::CortexMHeap;
 #[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
	loop {}
}

fn test_sign_verify()->bool{
    let mut keypair_out:[u8;96] = [0u8;96];
    let seed:[u8;32] = [0u8;32];
    let mut rv = unsafe { sr_keypair_from_seed(keypair_out.as_mut_ptr(),seed.as_ptr()) };
    hprintln!("keypair{:#?}",&keypair_out[..]).unwrap();
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
    hprintln!("sign result{:#?}",&sign_out[..]).unwrap();
    hprintln!("keypair{:#?}",&keypair_out[..]).unwrap();
    rv = unsafe {sr_verify(sign_out.as_ptr(),message_bytes.as_ptr(),32,keypair_out[64..].as_ptr())};
    hprintln!("verify result{}",rv).unwrap();
    true
}
// pub unsafe extern "C" fn sr_verify(
//     signature_ptr: *const u8,
//     message_ptr: *const u8,
//     message_length: usize,
//     public_ptr: *const u8,
// ) -> u32 {

#[entry]
fn main() -> ! {

    let rv = test_sign_verify();
    debug::exit(debug::EXIT_SUCCESS);
    loop {
    }
}
