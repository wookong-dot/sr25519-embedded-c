#![no_std]
//#![deny(missing_docs)] // refuse to compile if documentation is missing
#![feature(alloc_error_handler)]
#![feature(lang_items)]

extern crate rand_core;
extern crate schnorrkel;
extern crate exrng;

pub mod api;
pub mod device;

