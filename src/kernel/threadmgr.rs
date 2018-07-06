use types::SceSize;
use types::SceUID;
use types::SceUInt;
use types::SceUInt32;
use types::SceUInt64;

type SceKernelSysClock = SceUInt64;

// Mutexes

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

#[repr(i32)]
pub enum SceKernelMutexAttribute {
    SCE_KERNEL_MUTEX_ATTR_RECURSIVE = 0x02,
}

// Threads

type SceKernelThreadEntry = extern "C" fn(SceSize, *mut ::void) -> i32;

#[repr(C)]
pub struct SceKernelThreadOptParam {
    pub size: SceSize,
    pub attr: SceUInt32,
}

#[repr(C)]
pub struct SceKernelThreadInfo {
    pub size: SceSize,
    pub processId: SceUID,
    pub name: [u8; 32],
    pub attr: SceUInt,
    pub status: i32,
    pub entry: SceKernelThreadEntry,
    pub stack: *mut ::void,
    pub stackSize: i32,
    pub initPriority: i32,
    pub currentPriority: i32,
    pub initCpuAffinityMask: i32,
    pub currentCpuAffinityMask: i32,
    pub currentCpuId: i32,
    pub lastExecutedCpuId: i32,
    pub waitType: i32,
    pub waitId: SceUID,
    pub exitStatus: i32,
    pub runClocks: SceKernelSysClock,
    pub intrPreemptCount: SceUInt,
    pub threadPreemptCount: SceUInt,
    pub threadReleaseCount: SceUInt,
    pub fNotifyCallback: SceUID,
    pub reserved: i32,
}

#[repr(C)]
pub struct SceKernelThreadCpuInfo {
    pub processId: SceUID,
    pub threadId: SceUID,
    pub priority: i32,
}

#[repr(C)]
pub struct SceKernelThreadRunStatus {
    pub size: SceSize,
    pub cpuInfo: [SceKernelThreadCpuInfo; 4],
}

#[repr(i32)]
pub enum SceThreadStatus {
    SCE_THREAD_RUNNING = 1,
    SCE_THREAD_READY = 2,
    SCE_THREAD_WAITING = 4,
    SCE_THREAD_SUSPEND = 8,
    SCE_THREAD_STOPPED = 16,
    SCE_THREAD_KILLED = 32,
}

// Semaphores

#[repr(C)]
pub struct SceKernelSemaOptParam {
    size: SceSize,
}

#[repr(C)]
pub struct SceKernelSemaInfo {
    pub size: SceSize,
    pub semaId: SceUID,
    pub name: [u8; 32],
    pub attr: SceUInt,
    pub initCount: i32,
    pub currentCount: i32,
    pub maxCount: i32,
    pub numWaitThreads: i32,
}

#[link(kind = "static", name = "SceLibKernel_stub")]
extern "C" {

    // Mutexes

    pub fn sceKernelCreateMutex(
        name: *const u8,
        attr: SceUInt,
        initCount: i32,
        option: *mut SceKernelMutexOptParam,
    ) -> SceUID;
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

    pub fn sceKernelCreateThread(
        name: *const u8,
        entry: SceKernelThreadEntry,
        initPriority: i32,
        stackSize: i32,
        attr: SceUInt,
        cpuAffinityMask: i32,
        option: *const SceKernelThreadOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteThread(thid: SceUID) -> i32;
    pub fn sceKernelStartThread(thid: SceUID, arglen: SceSize, argp: *mut ::void) -> i32;
    pub fn sceKernelWaitThreadEnd(thid: SceUID, stat: *mut i32, timeout: *mut SceUInt) -> i32;
    pub fn sceKernelWaitThreadEndCB(thid: SceUID, stat: *mut i32, timeout: *mut SceUInt) -> i32;
    pub fn sceKernelDelayThread(delay: SceUInt) -> i32;
    pub fn sceKernelDelayThreadCB(delay: SceUInt) -> i32;
    pub fn sceKernelChangeCurrentThreadAttr(unknown: i32, attr: SceUInt) -> i32;
    pub fn sceKernelChangeThreadPriority(thid: SceUID, priority: i32) -> i32;
    pub fn sceKernelGetThreadId() -> SceUID;
    pub fn sceKernelGetThreadCurrentPriority() -> i32;
    pub fn sceKernelGetThreadExitStatus(thid: SceUID) -> i32;
    pub fn sceKernelCheckThreadStack() -> i32;
    pub fn sceKernelGetThreadStackFreeSize(thid: SceUID) -> i32;
    pub fn sceKernelGetThreadInfo(thid: SceUID, info: *mut SceKernelThreadInfo) -> i32;
    pub fn sceKernelGetThreadRunStatus(thid: SceUID, status: *mut SceKernelThreadRunStatus) -> i32;

    // Semaphores

    pub fn sceKernelCreateSema(
        name: *const u8,
        attr: SceUInt,
        initVal: i32,
        maxVal: i32,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteSema(semaid: SceUID) -> i32;
    pub fn sceKernelSignalSema(semaid: SceUID, signal: i32) -> i32;
    pub fn sceKernelWaitSema(semaid: SceUID, signal: i32, timeout: *mut SceUInt) -> i32;
    pub fn sceKernelWaitSemaCB(semaid: SceUID, signal: i32, timeout: *mut SceUInt) -> i32;
    pub fn sceKernelPollSema(semaid: SceUID, signal: i32) -> i32;
    pub fn sceKernelCancelSema(semaid: SceUID, setCount: i32, numWaitThreads: *mut i32) -> i32;
    pub fn sceKernelGetSemaInfo(semaid: SceUID, info: *mut SceKernelSemaInfo) -> i32;

}
