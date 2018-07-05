use types::SceDateTime;
use types::SceMode;
use types::SceOff;
use types::SceUID;

#[repr(i32)]
pub enum SceIoAccessMode {
    SCE_S_IXUSR = 0x0001, // User execute permission
    SCE_S_IWUSR = 0x0002, // User write permission
    SCE_S_IRUSR = 0x0004, // User read permission
    SCE_S_IRWXU = 0x0007, // User access rights mask

    SCE_S_IXGRP = 0x0008, // Group execute permission
    SCE_S_IWGRP = 0x0010, // Group write permission
    SCE_S_IRGRP = 0x0020, // Group read permission
    SCE_S_IRWXG = 0x0038, // Group access rights mask

    SCE_S_IXOTH = 0x0040, // Others execute permission
    SCE_S_IWOTH = 0x0080, // Others write permission
    SCE_S_IROTH = 0x0100, // Others read permission
    SCE_S_IRWXO = 0x01C0, // Others access rights mask

    SCE_S_ISVTX = 0x0200, // Sticky
    SCE_S_ISGID = 0x0400, // Set GID
    SCE_S_ISUID = 0x0800, // Set UID

    SCE_S_IFDIR = 0x1000, // Directory
    SCE_S_IFREG = 0x2000, // Regular file
    SCE_S_IFLNK = 0x4000, // Symbolic link
    SCE_S_IFMT = 0xF000,  // Format bits mask
}

#[repr(i32)]
pub enum SceIoFileMode {
    SCE_SO_IXOTH = 0x0001, // Hidden execute permission
    SCE_SO_IWOTH = 0x0002, // Hidden write permission
    SCE_SO_IROTH = 0x0004, // Hidden read permission
    SCE_SO_IFLNK = 0x0008, // Symbolic link
    SCE_SO_IFDIR = 0x0010, // Directory
    SCE_SO_IFREG = 0x0020, // Regular file
    SCE_SO_IFMT = 0x0038,  // Format mask
}

#[repr(C)]
pub struct SceIoStat {
    pub st_mode: SceMode,      // One or more ::SceIoAccessMode
    pub st_attr: u32,          // One or more ::SceIoFileMode
    pub st_size: SceOff,       // Size of the file in bytes
    pub st_ctime: SceDateTime, // Creation time
    pub st_atime: SceDateTime, // Last access time
    pub st_mtime: SceDateTime, // Last modification time
    pub st_private: [u32; 6],  // Device-specific data
}

#[link(kind = "static", name = "SceLibKernel_stub")]
extern "C" {
    pub fn sceIoMkdir(dir: *const u8, mode: SceMode) -> i32;
    pub fn sceIoRmdir(path: *const u8) -> i32;
    pub fn sceIoGetstatbyFd(fd: SceUID, stat: *mut SceIoStat) -> i32;
    pub fn sceIoChstat(file: *const u8, stat: *mut SceIoStat, bits: i32) -> i32;
    pub fn sceIoChstatByFd(fd: SceUID, buf: *const SceIoStat, cbit: u32) -> i32;
}
