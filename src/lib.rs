#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate rmp_serde as msgpack;
extern crate sodiumoxide;

pub mod scp;
