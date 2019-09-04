#![no_std]
#![warn(future_incompatible)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
//#![deny(missing_docs)] // refuse to compile if documentation is missing
#![feature(alloc_error_handler)]
#![feature(lang_items)]

extern crate rand_core;
extern crate schnorrkel;
extern crate exrng;

pub mod wrapper;
pub mod api;

