#![allow(dead_code, unused_imports, unused_variables, unused_macros, unused_parens)]
#![feature(lang_items, core_intrinsics, start, used, const_fn)]
#![no_std]

extern crate psp2_sys as psp2;

mod debug;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut screen = debug::screen::DebugScreen::new();
    screen.puts(b"Hello, world!\n\0");
    screen.puts(b"How are you?\n\0");
    // screen.puts(b"\x1b[1;34mWow, this is blue !\x1b[0m]");

    psp2::kernel::threadmgr::sceKernelDelayThread(3 * 1000000); // Wait for 3 seconds
    psp2::kernel::processmgr::sceKernelExitProcess(0);
    return 0;
}

#[start]
#[no_mangle]
pub unsafe fn _start(argc: isize, argv: *const *const u8) -> isize {
    main(argc, argv)
}
