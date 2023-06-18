use crate::types::{SceInt32, SceSize, SceUID, SceUInt, SceUInt32, SceUInt64};

/// 64-bit system clock type
type SceKernelSysClock = SceUInt64;

// Threads

type SceKernelThreadEntry = extern "C" fn(SceSize, *mut crate::void) -> i32;

/// Additional options used when creating threads.
#[repr(C)]
pub struct SceKernelThreadOptParam {
    /// Size of the [SceKernelThreadOptParam] structure.
    pub size: SceSize,
    /// Attributes
    pub attr: SceUInt32,
}

/// Structure to hold the status information for a thread
///
/// See [sceKernelGetThreadInfo]
#[repr(C)]
pub struct SceKernelThreadInfo {
    /// Size of the structure
    pub size: SceSize,
    /// The UID of the process where the thread belongs
    pub processId: SceUID,
    /// Nul terminated name of the thread
    pub name: [u8; 32],
    /// Thread attributes
    pub attr: SceUInt32,
    /// Thread status
    pub status: SceUInt32,
    /// Thread entry point
    pub entry: SceKernelThreadEntry,
    /// Thread stack pointer
    pub stack: *mut crate::void,
    /// Thread stack size
    pub stackSize: SceInt32,
    /// Initial priority
    pub initPriority: SceInt32,
    /// Current priority
    pub currentPriority: SceInt32,
    /// Initial CPU affinity mask
    pub initCpuAffinityMask: SceInt32,
    /// Current CPU affinity mask
    pub currentCpuAffinityMask: SceInt32,
    /// Current CPU ID
    pub currentCpuId: SceInt32,
    /// Last executed CPU ID
    pub lastExecutedCpuId: SceInt32,
    /// Wait type
    pub waitType: SceUInt32,
    /// Wait id
    pub waitId: SceUID,
    /// Exit status of the thread
    pub exitStatus: SceInt32,
    /// Number of clock cycles run
    pub runClocks: SceKernelSysClock,
    /// Interrupt preemption count
    pub intrPreemptCount: SceUInt32,
    /// Thread preemption count
    pub threadPreemptCount: SceUInt32,
    /// Thread release count
    pub threadReleaseCount: SceUInt32,
    /// Number of CPUs to which the thread is moved
    pub changeCpuCount: SceInt32,
    /// Function notify callback UID
    pub fNotifyCallback: SceInt32,
    /// Reserved
    pub reserved: SceInt32,
}

/// "Nested" Struct for [SceKernelThreadRunStatus]
#[repr(C)]
pub struct SceKernelThreadCpuInfo {
    pub processId: SceUID,
    pub threadId: SceUID,
    pub priority: i32,
}

/// Statistics about a running thread.
///
/// See [sceKernelGetThreadRunStatus]
#[repr(C)]
pub struct SceKernelThreadRunStatus {
    pub size: SceSize,
    /// See [SceKernelThreadCpuInfo]
    pub cpuInfo: [SceKernelThreadCpuInfo; 4],
}

#[repr(i32)]
pub enum SceThreadStatus {
    /// SCE Thread Running
    SCE_THREAD_RUNNING = 1,
    /// SCE Thread Ready
    SCE_THREAD_READY = 2,
    /// SCE Thread Standby
    SCE_THREAD_STANDBY = 4,
    /// Sce Thread Wating
    SCE_THREAD_WAITING = 8,
    // SCE_THREAD_SUSPEND : Defined in [SceThreadStatus] Implementation
    /// SCE Thread Dormant
    SCE_THREAD_DORMANT = 16,
    // SCE_THREAD_STOPPED : Defined in [SceThreadStatus] Implementation
    /// SCE Thread Deleted - Thread manager has killed the thread (stack overflow)
    SCE_THREAD_DELETED = 32,
    // SCE_THREAD_KILLED = 32,
    /// SCE Thread Dead
    SCE_THREAD_DEAD = 64,
    /// SCE Thread Stagnant
    SCE_THREAD_STAGNANT = 128,
    /// SCE Thread Suspended
    SCE_THREAD_SUSPENDED = 256,
}

/// Implementation for [SceThreadStatus] : Fixing missing enum variant
impl SceThreadStatus {
    /// SCE Thread Suspended (Compatibility)
    pub const SCE_THREAD_SUSPEND: SceThreadStatus = SceThreadStatus::SCE_THREAD_WAITING;
    /// SCE Thread Stopped (Compatibility)
    pub const SCE_THREAD_STOPPED: SceThreadStatus = SceThreadStatus::SCE_THREAD_DORMANT;
    /// SCE Thread Killed (Compatibility)
    pub const SCE_THREAD_KILLED: SceThreadStatus = SceThreadStatus::SCE_THREAD_DELETED;
}

#[repr(i32)]
pub enum SceKernelMutexAttribute {
    SCE_KERNEL_MUTEX_ATTR_RECURSIVE = 0x02,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    //! Threads function
    /// Create a thread
    ///
    /// ### Parameters
    /// * `name` - An arbitrary thread name.
    /// * `entry` - The thread function to run when started.
    /// * `initPriority` - The initial priority of the thread. Less if higher
    /// priority.
    /// * `stackSize` - The size of the initial stack.
    /// * `attr` - The thread attributes, zero or more of ::SceThreadAttributes (???).
    /// * `cpuAffinityMask` - The CPU affinity mask.
    /// * `option` - Additional options specified by [SceKernelThreadOptParam].
    ///
    /// Returns UID of the created thread, or an error code.
    ///
    /// ### Example
    /// ```rust
    /// let thid: SceUID;
    /// thid = sceKernelCreateThread("my_thread", threadFunc, 0x10000100, 0x10000, 0, 0, 0x0);
    /// ```
    pub fn sceKernelCreateThread(
        name: *const u8,
        entry: SceKernelThreadEntry,
        initPriority: i32,
        stackSize: i32,
        attr: SceUInt,
        cpuAffinityMask: i32,
        option: *const SceKernelThreadOptParam,
    ) -> SceUID;

    /// Delete a thread
    ///
    /// ### Parameters
    /// * `thid` - UID of the thread to be deleted.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelDeleteThread(thid: SceUID) -> i32;

    /// Start a created thread
    ///
    /// ### Parameters
    /// * `thid` - Thread id from [sceKernelCreateThread].
    /// * `arglen` - Length of the data pointed to by argp, in bytes.
    /// * `argp` - Pointer to the arguments.
    pub fn sceKernelStartThread(thid: SceUID, arglen: SceSize, argp: *mut crate::void) -> i32;

    /// Exit a thread
    ///
    /// ### Parameters
    /// * `status` - Exit status.
    pub fn sceKernelExitThread(status: i32) -> i32;

    /// Exit a thread and delete itself
    ///
    /// ### Parameters
    /// * `status` - Exit status
    pub fn sceKernelExitDeleteThread(status: i32) -> i32;

    /// Wait until a thread has ended
    ///
    /// ### Parameters
    /// * `thid` - Id of the thread to wait for.
    /// * `stat` - Exit status.
    /// * `timeout` - Timeout in microseconds (assumed).
    ///
    /// Returns < 0 on error.
    pub fn sceKernelWaitThreadEnd(thid: SceUID, stat: *mut i32, timeout: *mut SceUInt) -> i32;

    /// Wait until a thread has ended and handle callbacks if necessary
    ///
    /// ### Parameters
    /// * `thid` - Id of the thread to wait for.
    /// * `stat` - Exit status.
    /// * `timeout` - Timeout in microseconds (assumed).
    ///
    /// Returns < 0 on error.
    pub fn sceKernelWaitThreadEndCB(thid: SceUID, stat: *mut i32, timeout: *mut SceUInt) -> i32;

    /// Delay the current thread by a specified number of microseconds
    ///
    /// ### Parameters
    /// * `delay` - Delay in microseconds.
    ///
    /// ### Example
    /// ```rust
    /// sceKernelDelayThread(1000000); // Delay for a second
    /// ```
    pub fn sceKernelDelayThread(delay: SceUInt) -> i32;

    /// Delay the current thread by a specified number of microseconds and
    /// handle any callbacks
    ///
    /// ### Parameters
    /// * `delay` - Delay in microseconds.
    ///
    /// ### Example
    /// ```rust
    /// sceKernelDelayThread(1000000); // Delay for a second
    /// ```
    pub fn sceKernelDelayThreadCB(delay: SceUInt) -> i32;

    /// Modify the attributes of the current thread
    ///
    /// * `unknown` - Set to 0.
    /// * `attr` - The thread attributes to modify. One of
    /// ::SceThreadAttributes (???).
    ///
    /// Returns < 0 on error.
    pub fn sceKernelChangeCurrentThreadAttr(unknown: i32, attr: SceUInt) -> i32;

    /// Change the threads current priority.
    ///
    /// ### Parameters
    /// * `thid` - The ID of the thread (from [sceKernelCreateThread] or
    /// [sceKernelGetThreadId]).
    /// * `priority` - The new priority (the lower the number the higher the
    /// priority).
    ///
    /// Returns 0 if successful, otherwise the error code.
    ///
    /// ### Example
    /// ```rust
    /// let thid: SceUID = sceKernelGetThreadId();
    /// sceKernelChangeThreadPriority(thid, 16); // Change priority of current thread to 16
    /// ```
    pub fn sceKernelChangeThreadPriority(thid: SceUID, priority: i32) -> i32;

    /// Get the current thread Id
    ///
    /// Returns the thread id of the calling thread.
    pub fn sceKernelGetThreadId() -> SceUID;

    /// Get the current priority of the thread you are in
    ///
    /// Returns The current thread priority.
    pub fn sceKernelGetThreadCurrentPriority() -> i32;

    /// Get the exit status of a thread
    ///
    /// # Parameters
    /// *  `thid` - The UID of the thread to check.
    /// *  `status` (out) - Status out pointer.
    ///
    /// Returns the exit status.
    pub fn sceKernelGetThreadExitStatus(thid: SceUID, status: *mut i32) -> i32;

    /// Check the thread stack?
    ///
    /// Returns unknown.
    pub fn sceKernelCheckThreadStack() -> i32;

    /// Get the free stack size for a thread
    ///
    /// # Parmameters
    /// * `thid` - The thread ID. Seem to take current thread if set to 0.
    ///
    /// Returns the free size.
    pub fn sceKernelGetThreadStackFreeSize(thid: SceUID) -> i32;

    /// Get the status information for the specified thread
    ///
    /// # Parameters
    /// * `thid` - Id of the thread to get status.
    /// * `info` - Pointer to the info structure to receive the data.
    /// > Note: The structures size field should be set to
    /// sizeof(SceKernelThreadInfo) before calling this function.
    ///
    /// Returns 0 if successful, otherwise the error code.
    ///
    /// ### Example
    /// ```no_run rust
    /// let status: SceKernelThreadInfo;
    /// status.size = sizeof(SceKernelThreadInfo);
    /// if(sceKernelGetThreadInfo(thid, &status) == 0) {
    ///     Do something...
    /// }
    /// ```
    pub fn sceKernelGetThreadInfo(thid: SceUID, info: *mut SceKernelThreadInfo) -> i32;

    /// Retrive the runtime status of a thread
    ///
    /// # Parameters
    /// * `thid` - UID of the thread to retrieve status.
    /// * `status` - Pointer to a [SceKernelThreadRunStatus] struct to receive
    /// the runtime status.
    ///
    /// @return 0 if successful, otherwise the error code.
    pub fn sceKernelGetThreadRunStatus(thid: SceUID, status: *mut SceKernelThreadRunStatus) -> i32;
}

// Semaphores

/// Additional options used when creating semaphores
#[repr(C)]
pub struct SceKernelSemaOptParam {
    /// Size of the [SceKernelSemaOptParam] structure
    size: SceSize,
}

/// Current state of a semaphore
///
/// See [sceKernelGetSemaInfo].
#[repr(C)]
pub struct SceKernelSemaInfo {
    /// Size of the [SceKernelSemaInfo] structure
    pub size: SceSize,
    /// The UID of the semaphore
    pub semaId: SceUID,
    /// NULL-terminated name of the semaphore
    pub name: [u8; 32],
    /// Attributes
    pub attr: SceUInt,
    /// The initial count the semaphore was created with
    pub initCount: i32,
    /// The current count
    pub currentCount: i32,
    /// The maximum count
    pub maxCount: i32,
    /// The number of threads waiting on the semaphore
    pub numWaitThreads: i32,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    //! Semaphores
    /// Creates a new semaphore
    ///
    /// ### Parameters
    /// * `name` - Specifies the name of the sema.
    /// * `attr` - Sema attribute flags (normally set to 0).
    /// * `initVal` - Sema initial value.
    /// * `maxVal` - Sema maximum value.
    /// * `option` - Sema options (normally set to 0).
    ///
    /// Returns a semaphore id.
    ///
    /// ### Example
    /// ```
    /// let semaid: SceUID;
    /// semaid = sceKernelCreateSema("MySema", 0, 1, 1, 0x0);
    /// ```
    pub fn sceKernelCreateSema(
        name: *const u8,
        attr: SceUInt,
        initVal: i32,
        maxVal: i32,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;

    /// Destroy a semaphore
    ///
    /// ### Parameters
    /// * `semaid` - The semaid returned from a previous create call.
    ///
    /// Returns the value 0 if it's successful, otherwise -1.
    pub fn sceKernelDeleteSema(semaid: SceUID) -> i32;

    /// Send a signal to a semaphore
    ///
    /// ### Parameters
    /// * `semaid` - The sema id returned from ::sceKernelCreateSema.
    /// * `signal` - The amount to signal the sema (i.e. if 2 then increment
    /// the sema by 2).
    ///
    /// Returns < 0 On error.
    ///
    /// ### Example
    /// ```rust
    /// sceKernelSignalSema(semaid, 1); // Signal the sema
    /// ```
    pub fn sceKernelSignalSema(semaid: SceUID, signal: i32) -> i32;

    /// Lock a semaphore
    ///
    /// ### Parameters
    /// * `semaid` - The sema id returned from ::sceKernelCreateSema.
    /// * `signal` - The value to wait for (i.e. if 1 then wait till reaches a
    /// signal state of 1).
    /// * `timeout` - Timeout in microseconds (assumed).
    ///
    /// Returns < 0 on error.
    ///
    /// ### Example
    /// ```rust
    /// sceKernelWaitSema(semaid, 1, 0);
    /// ```
    pub fn sceKernelWaitSema(semaid: SceUID, signal: i32, timeout: *mut SceUInt) -> i32;

    /// Lock a semaphore and handle callbacks if necessary
    ///
    /// ### Parameters
    /// * `semaid` - The sema id returned from ::sceKernelCreateSema
    /// * `signal` - The value to wait for (i.e. if 1 then wait till reaches a
    /// signal state of 1).
    /// * `timeout` - Timeout in microseconds (assumed).
    ///
    /// Returns < 0 on error.
    ///
    /// ### Example
    /// ```rust
    /// sceKernelWaitSemaCB(semaid, 1, 0);
    /// ```
    pub fn sceKernelWaitSemaCB(semaid: SceUID, signal: i32, timeout: *mut SceUInt) -> i32;

    /// Poll a semaphore
    ///
    /// * `semaid` - UID of the semaphore to poll.
    /// * `signal` - The value to test for.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelPollSema(semaid: SceUID, signal: i32) -> i32;

    /// Cancels a semaphore
    ///
    /// ### Parameters
    /// * `semaid` - The sema id returned from ::sceKernelCreateSema.
    /// * `setCount` - The new lock count of the semaphore.
    /// * `numWaitThreads` - Number of threads waiting for the semaphore.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelCancelSema(semaid: SceUID, setCount: i32, numWaitThreads: *mut i32) -> i32;

    /// Retrieve information about a semaphore
    ///
    /// ### Parameters
    /// * `semaid` - UID of the semaphore to retrieve info for.
    /// * `info` - Pointer to a ::SceKernelSemaInfo struct to receive the info.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelGetSemaInfo(semaid: SceUID, info: *mut SceKernelSemaInfo) -> i32;
}

// Mutexes

/// Additional options used when creating mutexes
#[repr(C)]
pub struct SceKernelMutexOptParam {
    /// Size of the [SceKernelMutexOptParam] structure
    pub size: SceSize,
    pub ceilingPriority: i32,
}

/// Current state of a mutex
///
/// See [sceKernelGetMutexInfo].
#[repr(C)]
pub struct SceKernelMutexInfo {
    /// Size of the ::SceKernelMutexInfo structure
    pub size: SceSize,
    /// The UID of the mutex
    pub mutexId: SceUID,
    /// NULL-terminated name of the mutex
    pub name: [u8; 32],
    /// Attributes
    pub attr: SceUInt,
    /// The initial count the mutex was created with
    pub initCount: i32,
    /// The current count
    pub currentCount: i32,
    /// The UID of the current owner of the mutex
    pub currentOwnerId: SceUID,
    /// The number of threads waiting on the mutex
    pub numWaitThreads: i32,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    //! Mutexes

    /// Creates a new mutex
    ///
    /// ### Parameters
    /// * `name` - Specifies the name of the mutex.
    /// * `attr` - Mutex attribute flags (normally set to 0).
    /// * `initCount` - Mutex initial value.
    /// * `option` - Mutex options (normally set to 0).
    ///
    /// Returns a mutex id.
    ///
    /// ### Example
    /// ```rust
    /// mutexid: SceUID;
    /// mutexid = sceKernelCreateMutex("MyMutex", 0, 1, 0x0);
    /// ```
    pub fn sceKernelCreateMutex(
        name: *const u8,
        attr: SceUInt,
        initCount: i32,
        option: *mut SceKernelMutexOptParam,
    ) -> SceUID;

    /// Destroy a mutex
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    ///
    /// Returns the value 0 if it's successful, otherwise -1.
    pub fn sceKernelDeleteMutex(mutexid: SceUID) -> i32;

    /// Open a mutex
    ///
    /// ### Parameters
    /// * `name` - The name of the mutex to open.
    ///
    /// Returns the value 0 if it's successful, otherwise -1.
    pub fn sceKernelOpenMutex(name: *const u8) -> i32;

    /// Close a mutex
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    ///
    /// Returns the value 0 if it's successful, otherwise -1.
    pub fn sceKernelCloseMutex(mutexid: SceUID) -> i32;

    /// Lock a mutex
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    /// * `lockCount` - The value to increment to the lock count of the mutex.
    /// * `timeout` - Timeout in microseconds (assumed).
    ///
    /// Returns < 0 on error.
    pub fn sceKernelLockMutex(mutexid: SceUID, lockCount: i32, timeout: *mut u32) -> i32;

    /// Lock a mutex and handle callbacks if necessary.
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    /// * `lockCount` - The value to increment to the lock count of the mutex.
    /// * `timeout` - Timeout in microseconds (assumed).
    ///
    /// Retuns < 0 on error.
    pub fn sceKernelLockMutexCB(mutexid: SceUID, lockCount: i32, timeout: *mut u32) -> i32;

    /// Try to lock a mutex (non-blocking)
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    /// * `lockCount` - The value to increment to the lock count of the mutex.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelTryLockMutex(mutexid: SceUID, lockCount: i32) -> i32;

    /// Try to unlock a mutex (non-blocking)
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    /// * `unlockCount` - The value to decrement to the lock count of the mutex.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelUnlockMutex(mutexid: SceUID, unlockCount: i32) -> i32;

    /// Cancels a mutex
    ///
    /// ### Parameters
    /// * `mutexid` - The mutex id returned from [sceKernelCreateMutex].
    /// * `newCount` - The new lock count of the mutex.
    /// * `numWaitThreads` - Number of threads waiting for the mutex.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelCancelMutex(mutexid: SceUID, lockCount: i32, numWaitThreads: *mut i32) -> i32;

    /// Retrieve information about a mutex.
    ///
    /// ### Parameters
    /// * `mutexid` - UID of the mutex to retrieve info for.
    /// * `info` - Pointer to a [SceKernelMutexInfo] struct to receive the info.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelGetMutexInfo(mutexid: SceUID, info: *mut SceKernelMutexInfo) -> i32;
}

// Event Flags

/// Structure to hold event flag information
#[repr(C)]
pub struct SceKernelEventFlagInfo {
    size: SceSize,
    /// Needs confirmation
    evfId: SceUID,
    name: [u8; 32],
    attr: SceUInt,
    initPattern: SceUInt,
    currentPattern: SceUInt,
    numWaitThreads: i32,
}

#[repr(C)]
pub struct SceKernelEventFlagOptParam {
    size: SceSize,
}

#[repr(C)]
pub enum SceEventFlagAttributes {
    /// Waiting threads queued on a FIFO basis
    SCE_EVENT_THREAD_FIFO = 0,
    /// Waiting threads queued on priority basis
    SCE_EVENT_THREAD_PRIO = 0x00002000,
    /// Event flag can only be waited upon by one thread
    /// SCE_EVENT_WAITSINGLE : Defined in [SceEventFlagAttributes] Implementation.
    /// Event flag can be waited upon by multiple threads
    SCE_EVENT_WAITMULTIPLE = 0x00001000,
    /// Event flag can be accessed by sceKernelOpenEventFlag / sceKernelCloseEventFlag
    SCE_EVENT_OPENABLE = 0x00000080,
}

impl SceEventFlagAttributes {
    pub const SCE_EVENT_WAITSINGLE: SceEventFlagAttributes =
        SceEventFlagAttributes::SCE_EVENT_THREAD_FIFO;
}

#[repr(C)]
pub enum SceEventFlagWaitTypes {
    /// Wait for all bits in the pattern to be set
    SCE_EVENT_WAITAND = 0,
    /// Wait for one or more bits in the pattern to be set
    SCE_EVENT_WAITOR = 1,
    /// Clear all the bits when it matches
    SCE_EVENT_WAITCLEAR = 2,
    /// Clear the wait pattern when it matches
    SCE_EVENT_WAITCLEAR_PAT = 4,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    //! Event Flags

    /// Create an event flag
    ///
    /// ### Parameters
    /// * `name` - The name of the event flag.
    /// * `attr` - Attributes from ::SceEventFlagAttributes.
    /// * `bits` - Initial bit pattern.
    /// * `opt` - Options, set to NULL.
    ///
    /// Returns < 0 on error. >= 0 event flag id.
    ///
    /// ### Example
    /// ```rust
    /// evid: SceUID;
    /// evid = sceKernelCreateEventFlag("wait_event", 0, 0, 0x0);
    /// ```
    pub fn sceKernelCreateEventFlag(
        name: *const u8,
        attr: i32,
        bits: i32,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUID;

    /// Set an event flag bit pattern
    ///
    /// ### Parameters
    /// * `evid` - The event id returned by [sceKernelCreateEventFlag].
    /// * `bits` - The bit pattern to set.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelSetEventFlag(evid: SceUID, bits: u32) -> i32;

    /// Clear a event flag bit pattern
    ///
    /// ### Parameters
    /// * `evid` - The event id returned by [sceKernelCreateEventFlag].
    /// * `bits` - The bits to clean.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelClearEventFlag(evid: SceUID, bits: u32) -> i32;

    /// Poll an event flag for a given bit pattern
    ///
    /// ### Parameters
    /// * `evid` - The event id returned by [sceKernelCreateEventFlag].
    /// * `bits` - The bit pattern to poll for.
    /// * `wait` - Wait type, one or more of [SceEventFlagWaitTypes] or'ed
    /// together.
    /// * `outBits` - The bit pattern that was matched.
    ///
    /// Returns < 0 on error.
    pub fn sceKernelPollEventFlag(evid: SceUID, bits: u32, wait: u32, outBits: *mut u32) -> i32;

    /// Wait for an event flag for a given bit pattern
    ///
    /// ### Parameters
    /// * `evid` - The event id returned by [sceKernelCreateEventFlag].
    /// * `bits` - The bit pattern to poll for.
    /// * `wait` - Wait type, one or more of [SceEventFlagWaitTypes] or'ed
    /// together.
    /// * `outBits` - The bit pattern that was matched.
    /// * `timeout` - Timeout in microseconds.
    /// Returns < 0 on error
    pub fn sceKernelWaitEventFlag(
        evid: SceUID,
        bits: u32,
        wait: u32,
        outBits: *mut u32,
        timeout: *mut SceUInt,
    ) -> i32;

    /// Wait for an event flag for a given bit pattern with callback
    ///
    /// ### Parameters
    /// * `evid` - The event id returned by [sceKernelCreateEventFlag].
    /// * `bits` - The bit pattern to poll for.
    /// * `wait` - Wait type, one or more of [SceEventFlagWaitTypes] or'ed
    /// together.
    /// * `outBits` - The bit pattern that was matched.
    /// * `timeout`  - Timeout in microseconds.
    /// 
    /// Returns < 0 on error
    pub fn sceKernelWaitEventFlagCB(
        evid: SceUID,
        bits: u32,
        wait: u32,
        outBits: *mut u32,
        timeout: *mut SceUInt,
    ) -> i32;
}
