    // //hprintln!("Hello, embedded sr25519!").unwrap();
    // let context = signing_context(b"good");
    // let keypair_bytes: [u8;96] = [74,83,195,251,188,89,151,14,229,248,90,248,19,135,93,255,193,58,144,74,46,83,174,126,101,250,13,234,110,98,201,1,159,7,231,190,85,81,56,122,152,186,151,124,115,45,8,13,203,15,41,160,72,227,101,105,18,198,83,62,50,238,122,237,156,102,163,57,200,52,79,146,47,195,32,108,181,218,232,20,165,148,192,23,125,211,35,92,37,77,156,64,154,101,184,8];
	// let keypair = Keypair::from_bytes(&keypair_bytes[..]).unwrap();

	// let message_bytes: [u8;32] = [0u8;32];
	// let trng_bytes: [u8;32] = [0u8;32];

	// let signature: Signature = keypair.sign_trng(context.bytes(&message_bytes[..]),&trng_bytes);
	// let signature_bytes = signature.to_bytes();
    // //hprintln!("sign result{:#?}",&signature_bytes[..]).unwrap();

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate schnorrkel;
extern crate alloc;
extern crate alloc_cortex_m;
extern crate rand_core;

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use core::slice;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use rand_core::{CryptoRng, RngCore, SeedableRng, Error, le};

use schnorrkel::keys::*; // {MiniSecretKey,SecretKey,PublicKey,Keypair}; + *_LENGTH
use schnorrkel::context::{signing_context,attach_rng}; // SigningContext,SigningTranscript
use schnorrkel::sign::{Signature,SIGNATURE_LENGTH};
use schnorrkel::errors::{SignatureError,SignatureResult};
use schnorrkel::derive::{ExtendedKey,ChainCode,CHAIN_CODE_LENGTH};


pub struct ExternalRng
{
        pub rng_bytes: [u8;32],
        pub len: usize
}
impl ExternalRng
{
    pub fn set_rng(&self, dest: &mut [u8])
    {
        let mut k = 0;
        while k<self.len
        {
            dest[k] = self.rng_bytes[k];
            k= k+1;
        }
    }
}

impl RngCore for ExternalRng {
    fn next_u32(&mut self) -> u32 {  panic!()  }
    fn next_u64(&mut self) -> u64 {  panic!()  }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.set_rng(dest); 
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
impl CryptoRng for ExternalRng {}



fn test_derive(kp:&[u8;96])->bool{
    let key = Keypair::from_bytes(&kp[..]).unwrap();
    let chaincode = ChainCode([0u8; CHAIN_CODE_LENGTH]);
    let mut extended_keypair = ExtendedKey { key, chaincode};
    let msg : &'static [u8] = b"m'/44'/0'/0'/0";
    let ctx = signing_context(b"testing testing 1 2 3");
    let extended_keypair1 = extended_keypair.derived_key_simple(msg);
    let extended_keypair1_bytes = extended_keypair1.key.to_bytes();
    //hprintln!("extended_keypair1_bytes{:#?}",&extended_keypair1_bytes[..]).unwrap();
    true
}

fn test_sign_verify(kp:&[u8;96])->bool{
    let context = signing_context(b"good");
	let keypair = Keypair::from_bytes(&kp[..]).unwrap();
	let message_bytes: [u8;32] = [0u8;32];
	let trng_bytes: [u8;32] = [0u8;32];

    	let signature: Signature = keypair.sign(
        attach_rng(
            context.bytes(&message_bytes[..]), 
            ExternalRng{
                rng_bytes:trng_bytes,
                len:32}
                ));
    
	let signature_bytes = signature.to_bytes();
    hprintln!("sign result{:#?}",&signature_bytes[..]).unwrap();

    if keypair
		.verify(context.bytes(&message_bytes), &signature)
		.is_ok()
	{
		true
	} else {
		false
	}
}

fn test_kp_from_seed(seed:&[u8;32])->[u8;96]{
    let msk = MiniSecretKey::from_bytes(seed).unwrap();
	let msk_bytes = msk.to_bytes();
    hprintln!("msk{:#?}",&msk_bytes[..]).unwrap();

    let kp = msk.expand_to_keypair(ExpansionMode::Uniform);
    let kp_bytes = kp.to_bytes();
    hprintln!("kp_bytes{:#?}",&kp_bytes[..]).unwrap();

    kp_bytes
}
use alloc_cortex_m::CortexMHeap;
 #[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
	loop {}
}

#[entry]
fn main() -> ! {
    let seed:[u8;32] = [0u8;32];

    let kp = test_kp_from_seed(&seed);
    let rv = test_sign_verify(&kp);
  //  let rv2 = test_derive(&kp);
    hprintln!("verify result {:#?}",rv).unwrap();

    debug::exit(debug::EXIT_SUCCESS);
    loop {
    }
}
