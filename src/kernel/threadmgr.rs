use ::types::SceUInt;
use ::types::SceUID;
use ::types::SceSize;

#[repr(C)]
pub struct SceKernelMutexOptParam {
    pub size: SceSize,
    pub ceilingPriority: i32,
}

#[link(kind = "static", name = "SceLibKernel_stub")]
extern "C" {
    pub fn sceKernelDelayThread(delay: SceUInt) -> i32;
    pub fn sceKernelCreateMutex(name: *const u8, attr: SceUInt, initCount: i32, option: *mut SceKernelMutexOptParam) -> SceUID;
    pub fn sceKernelLockMutex(mutexid: SceUID, lockCount: i32, timeout: *mut u32) -> i32;
    pub fn sceKernelUnlockMutex(mutexid: SceUID, unlockCount: i32) -> i32;
}
