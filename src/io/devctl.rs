use crate::types::SceOff;
use crate::types::SceSize;
use crate::types::SceUID;

#[repr(C)]
pub struct SceIoDevInfo {
    pub max_size: SceOff,
    pub free_size: SceOff,
    pub cluster_size: SceSize,
    pub unk: *mut crate::void,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    pub fn sceIoDevctl(
        dev: *const u8,
        cmd: u32,
        indata: *mut crate::void,
        inlen: i32,
        outdata: *mut crate::void,
        outlen: i32,
    ) -> i32;
    pub fn sceIoIoctl(
        fd: SceUID,
        cmd: u32,
        indata: *mut crate::void,
        inlen: i32,
        outdata: *mut crate::void,
        outlen: i32,
    ) -> i32;
    pub fn sceIoIoctlAsync(
        fd: SceUID,
        cmd: u32,
        indata: *mut crate::void,
        inlen: i32,
        outdata: *mut crate::void,
        outlen: i32,
    ) -> i32;
}
