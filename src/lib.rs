// -*- mode: rust; -*-
//
// This file is part of sr25519-embedded-c.
// Copyright (c) 2017-2019 Chester Li and extropies.com
// See LICENSE for licensing information.
//
// Authors:
// - Chester Li<chester@lichester.com>

//! # sr25519-embedded-c
//! C bindings of sr25519 for embedded device. 
//! Do not generate random number inside. 
//! so that it could be run on platforms without rust rng crate. 
//! no std, no libc, no heap memory,
//! bin size < 100 KB, ram size < 32 KB. 
//! could be compiled by arm-gcc. 
//! # Example
//! ```
//! # fn main(){
//!     use sr25519_embedded_c::{
//!     api::{sr_keypair_from_seed,sr_sign,sr_verify}
//!     };
//!     let mut keypair_out:[u8;96] = [0u8;96];
//!     let seed:[u8;32] = [0u8;32];
//!     let mut rv = unsafe { sr_keypair_from_seed(keypair_out.as_mut_ptr(),seed.as_ptr()) };
//!     assert_eq!(rv,0 );
//! 	let message_bytes: [u8;32] = [0u8;32];
//! 	let rng_bytes: [u8;32] = [0u8;32];
//!     let mut sign_out:[u8;64] = [0u8;64];
//!     rv = unsafe {sr_sign(
//!                         message_bytes.as_ptr(),
//!                         32,
//!                         rng_bytes.as_ptr(),
//!                         keypair_out.as_ptr(),
//!                         sign_out.as_mut_ptr()
//!                 ) };
//! #    assert_eq!(rv,0 );
//!     let brv = unsafe {sr_verify(sign_out.as_ptr(),message_bytes.as_ptr(),32,keypair_out[64..].as_ptr())};
//! #    assert_eq!(brv,true);
//! #
//! #     }
//! 
//! ```
//! 

#![no_std]
#![deny(missing_docs)] // refuse to compile if documentation is missing
#![feature(alloc_error_handler)]
#![feature(lang_items)]

extern crate rand_core;
extern crate schnorrkel;
extern crate exrng;

/// api fns
pub mod api;
/// panic handler for embedded
pub mod device;

