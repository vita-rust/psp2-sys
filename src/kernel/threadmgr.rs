use ::types::SceUInt;
use ::types::SceUInt64;
use ::types::SceUID;
use ::types::SceSize;

type SceKernelSysClock = SceUInt64;

#[repr(C)]
pub struct SceKernelMutexOptParam {
    pub size: SceSize,
    pub ceilingPriority: i32,
}

#[repr(C)]
pub struct SceKernelMutexInfo {
	pub size: SceSize,
	pub mutexId: SceUID,
	pub name: [u8; 32],
	pub attr: SceUInt,
	pub initCount: i32,
	pub currentCount: i32,
	pub currentOwnerId: SceUID,
	pub numWaitThreads: i32,
}

#[link(kind = "static", name = "SceLibKernel_stub")]
extern "C" {

    // Mutexes
    pub fn sceKernelCreateMutex(name: *const u8, attr: SceUInt, initCount: i32, option: *mut SceKernelMutexOptParam) -> SceUID;
    pub fn sceKernelOpenMutex(name: *const u8) -> i32;
    pub fn sceKernelCloseMutex(mutexid: SceUID) -> i32;
    pub fn sceKernelDeleteMutex(mutexid: SceUID) -> i32;
    pub fn sceKernelLockMutex(mutexid: SceUID, lockCount: i32, timeout: *mut u32) -> i32;
    pub fn sceKernelLockMutexCB(mutexid: SceUID, lockCount: i32, timeout: *mut u32) -> i32;
    pub fn sceKernelTryLockMutex(mutexid: SceUID, lockCount: i32) -> i32;
    pub fn sceKernelUnlockMutex(mutexid: SceUID, unlockCount: i32) -> i32;
    pub fn sceKernelCancelMutex(mutexid: SceUID, lockCount: i32, numWaitThreads: *mut i32) -> i32;
    pub fn sceKernelGetMutexInfo(mutexid: SceUID, info: *mut SceKernelMutexInfo) -> i32;

    // Threads
    pub fn sceKernelDelayThread(delay: SceUInt) -> i32;
    
}
