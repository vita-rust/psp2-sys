#![feature(untagged_unions)]
#![allow(non_camel_case_types, non_snake_case, dead_code)]
#![no_std]

#[cfg(feature = "kernel")]
pub mod kernel;
pub mod types;
pub mod display;

pub enum void {}
