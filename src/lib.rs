#![feature(untagged_unions)]
#![feature(extern_types)]
#![allow(non_camel_case_types, non_snake_case, dead_code)]
#![no_std]

pub mod dialog;
pub mod display;
pub mod graphics;
pub mod kernel;
pub mod system_param;
pub mod types;

extern "C" {
    pub type void;
}
