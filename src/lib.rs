#![feature(untagged_unions)]
#![feature(extern_types)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![no_std]

pub mod ctrl;
pub mod dialog;
pub mod display;
pub mod graphics;
pub mod io;
pub mod kernel;
pub mod system_param;
pub mod types;

/// A placeholder enum for functions with `*void` arguments.
///
/// Only manipulate behind pointers !
pub enum void {}
