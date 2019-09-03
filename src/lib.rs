#![no_std]
#![warn(future_incompatible)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
//#![deny(missing_docs)] // refuse to compile if documentation is missing
#![feature(alloc_error_handler)]
#![feature(lang_items)]

extern crate alloc;
extern crate rand_core;
extern crate schnorrkel;

pub mod wrapper;
pub mod exrng;


use schnorrkel::keys::*; // {MiniSecretKey,SecretKey,PublicKey,Keypair}; + *_LENGTH
use schnorrkel::context::{signing_context,attach_rng}; // SigningContext,SigningTranscript
use schnorrkel::sign::{Signature,SIGNATURE_LENGTH};
use schnorrkel::errors::{SignatureError,SignatureResult};
use schnorrkel::derive::{ExtendedKey,ChainCode,CHAIN_CODE_LENGTH};
use core::panic::PanicInfo;
use rand_core::{CryptoRng, RngCore, SeedableRng, Error, le};


//  use alloc_cortex_m::CortexMHeap;
//  #[global_allocator]
//  pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();