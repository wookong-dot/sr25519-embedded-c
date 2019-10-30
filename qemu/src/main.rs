#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate sr25519_embedded_c;
extern crate alloc;
extern crate alloc_cortex_m;
extern crate panic_halt;

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
    let seed:[u8;32] = [0x3b, 0x70, 0x2c, 0x54, 0x31, 0x36, 0xcb, 0x64, 0xab, 0xff, 0xc0, 0x74, 0x0f, 0x0c, 0xc2, 0xfe, 0x6c, 0x6b, 0x7c, 0xd0, 0xe3, 0x6f, 0xea, 0x4e, 0xbf, 0x3a, 0x22, 0x0a, 0x38, 0x79, 0xa8, 0x3d];
    
    let mut rv = unsafe { sr_keypair_from_seed(keypair_out.as_mut_ptr(),seed.as_ptr()) };
    hprintln!("keypair {} {:#?}",rv, &keypair_out[..]).unwrap();
	let message_bytes: [u8;2] = [0x31,0x32];
	let rng_bytes: [u8;32] = [0u8;32];
    let mut sign_out:[u8;64] = [0u8;64];

    rv = unsafe {sr_sign(
                        message_bytes.as_ptr(),
                        2,
                        rng_bytes.as_ptr(),
                        keypair_out.as_ptr(),
                        sign_out.as_mut_ptr()
                ) };
    hprintln!("sign result {} {:#?}",rv, &sign_out[..]).unwrap();
    hprintln!("keypair{:#?}",&keypair_out[..]).unwrap();
    let brv = unsafe {sr_verify(sign_out.as_ptr(),message_bytes.as_ptr(),32,keypair_out[64..].as_ptr())};
    hprintln!("verify result {}",brv).unwrap();
    true
}
#[entry]
fn main() -> ! {

    test_sign_verify();
    debug::exit(debug::EXIT_SUCCESS);
    loop {
    }
}
