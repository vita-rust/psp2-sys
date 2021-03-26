use crate::types::SceSize;

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {

    pub type va_list;

    pub fn sceClibStrcmp(s1: *const u8, s2: *const u8) -> i32;
    pub fn sceClibStrncmp(s1: *const u8, s2: *const u8, n: SceSize) -> *mut crate::void;
    pub fn sceClibStrncasecmp(s1: *const u8, s2: *const u8) -> i32;
    pub fn sceClibStrncpy(dest: *mut u8, src: *const u8, n: SceSize) -> *mut u8;
    pub fn sceClibStrncat(dest: *mut u8, src: *const u8, n: SceSize) -> *mut u8;
    pub fn sceClibStrnlen(s: *const u8, maxlen: SceSize) -> SceSize;
    pub fn sceClibStrrchr(s: *const u8, c: i32) -> *mut u8;

    pub fn sceClibPrintf(format: *const u8, ...) -> i32;
    pub fn sceClibSnprintf(s: *mut u8, size: SceSize, format: *const u8, ...) -> i32;
    pub fn sceClibVsnprintf(s: *mut u8, size: SceSize, format: *const u8, ap: va_list) -> i32;

    pub fn sceClibMemset(s: *mut crate::void, c: i32, n: SceSize) -> *mut crate::void;
    pub fn sceClibMemcpy(dest: *mut crate::void, src: *const crate::void, n: SceSize) -> *mut crate::void;
    pub fn sceClibMemmove(dest: *mut crate::void, src: *const crate::void, n: SceSize) -> *mut crate::void;
}
