#![allow(dead_code, unused_imports, unused_variables, unused_macros, unused_parens)]
#![feature(lang_items, core_intrinsics, start)]
#![no_std]

extern crate psp2_sys as psp2;

mod debug;

use core::fmt::Write;
use core::intrinsics;
use core::panic::PanicInfo;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}

#[no_mangle]
pub unsafe fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut screen = debug::screen::DebugScreen::new();
    write!(screen, "This bare-metal is starting to rust!\n").ok();
    psp2::kernel::threadmgr::sceKernelDelayThread(1 * 1000000); // Wait for 1 second
    write!(screen, "See ? I told you !\n").ok();
    psp2::kernel::threadmgr::sceKernelDelayThread(3 * 1000000);
    psp2::kernel::processmgr::sceKernelExitProcess(0);
    return 0;
}

#[start]
#[no_mangle]
pub unsafe fn _start(argc: isize, argv: *const *const u8) -> isize {
    main(argc, argv)
}
