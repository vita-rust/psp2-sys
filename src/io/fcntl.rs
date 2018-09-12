use types::SceOff;
use types::SceSize;
use types::SceUID;

#[repr(i32)]
pub enum SceIoMode {
    SCE_O_RDONLY = 0x0001,        // Read-only
    SCE_O_WRONLY = 0x0002,        // Write-only
    SCE_O_RDWR = 0x0003,          // Read/Write
    SCE_O_NBLOCK = 0x0004,        // Non blocking
    SCE_O_DIROPEN = 0x0008,       // Internal use for ::sceIoDopen
    SCE_O_RDLOCK = 0x0010,        // Read locked (non-shared)
    SCE_O_WRLOCK = 0x0020,        // Write locked (non-shared)
    SCE_O_APPEND = 0x0100,        // Append
    SCE_O_CREAT = 0x0200,         // Create
    SCE_O_TRUNC = 0x0400,         // Truncate
    SCE_O_EXCL = 0x0800,          // Exclusive create
    SCE_O_SCAN = 0x1000,          // Scan type
    SCE_O_RCOM = 0x2000,          // Remote command entry
    SCE_O_NOBUF = 0x4000,         // Number device buffer
    SCE_O_NOWAIT = 0x8000,        // Asynchronous I/O
    SCE_O_FDEXCL = 0x01000000,    // Exclusive access
    SCE_O_PWLOCK = 0x02000000,    // Power control lock
    SCE_O_FGAMEDATA = 0x40000000, // Gamedata access
}

#[repr(i32)]
pub enum SceIoSeekMode {
    SCE_SEEK_SET,
    SCE_SEEK_CUR,
    SCE_SEEk_END,
}

#[repr(i32)]
pub enum SceIoDevType {
    SCE_DEV_TYPE_NULL = 0x00,    // Dummy device
    SCE_DEV_TYPE_CHAR = 0x01,    // Character device
    SCE_DEV_TYPE_BLOCK = 0x04,   // Block device
    SCE_DEV_TYPE_FS = 0x10,      // File system device
    SCE_DEV_TYPE_ALIAS = 0x20,   // Alias name
    SCE_DEV_TYPE_MOUNTPT = 0x40, // Mount point
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceIofilemgr_stub")
)]
extern "C" {
    pub fn sceIoOpen(file: *const u8, flags: i32, mode: SceIoMode) -> SceUID;
    pub fn sceIoOpenAsync(file: *const u8, flags: i32, mode: SceIoMode) -> SceUID;
    pub fn sceIoClose(fd: SceUID) -> i32;
    pub fn sceIoCloseAsync(fd: SceUID) -> i32;
    pub fn sceIoRead(fd: SceUID, data: *mut ::void, size: SceSize) -> i32;
    pub fn sceIoReadAsync(fd: SceUID, data: *mut ::void, size: SceSize) -> i32;
    pub fn sceIoPread(fd: SceUID, data: *mut ::void, size: SceSize, offset: SceOff) -> i32;
    pub fn sceIoWrite(fd: SceUID, data: *const ::void, size: SceSize) -> i32;
    pub fn sceIoWriteAsync(fd: SceUID, data: *const ::void, size: SceSize) -> i32;
    pub fn sceIoPwrite(fd: SceUID, data: *const ::void, size: SceSize, offset: SceOff) -> i32;
    pub fn sceIoLseek(fd: SceUID, offset: SceOff, whence: SceIoSeekMode) -> SceOff;
    pub fn sceIoLseekAsync(fd: SceUID, offset: SceOff, whence: SceIoSeekMode) -> SceOff;
    pub fn sceIoLseek32(fd: SceUID, offset: i32, whence: SceIoSeekMode) -> SceOff;
    pub fn sceIoRemove(file: *const u8) -> i32;
    pub fn sceIoSync(device: *const u8, unk: u32) -> i32;
    pub fn sceIoSyncByFd(fd: SceUID) -> i32;
    pub fn sceIoCancel(fd: SceUID) -> i32;
}
