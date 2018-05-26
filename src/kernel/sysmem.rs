use types::SceSize;
use types::SceUID;
use types::SceUInt32;

#[repr(i32)]
pub enum SceKernelMemBlockType {
    SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE = 0x0C208060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_RW = 0x0C20D060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_RW = 0x0C80D060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_RW = 0x0D808060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW = 0x09408060,
}

#[repr(C)]
pub struct SceKernelAllocMemBlockOpt {
    pub size: SceSize,
    pub attr: SceUInt32,
    pub alignment: SceSize,
    pub uidBaseBlock: SceUInt32,
    pub strBaseBlockName: *const u8,
    pub flags: i32,
    pub reserved: [i32; 10],
}

#[repr(C)]
pub struct SceKernelFreeMemorySizeInfo {
    /// sizeof(SceKernelFreeMemorySizeInfo)
    pub size: i32,
    /// Free memory size for *_USER_RW memory
    pub size_user: i32,
    /// Free memory size for USER_CDRAM_RW memory
    pub size_cdram: i32,
    ///Free memory size for USER_MAIN_PHYCONT_*_RW memory
    pub size_phycont: i32,
}

#[repr(i32)]
pub enum SceKernelModel {
    SCE_KERNEL_MODEL_VITA = 0x10000,
    SCE_KERNEL_MODEL_VITATV = 0x20000,
}

#[repr(C)]
pub struct SceKernelMemBlockInfo {
    pub size: SceSize,
    pub mappedBase: *mut ::void,
    pub mappedSize: SceSize,
    pub memory_type: SceKernelMemoryType,
    pub access: SceUInt32,
    pub block_type: SceKernelMemBlockType,
}

#[repr(i32)]
pub enum SceKernelMemoryAccessType {
    SCE_KERNEL_MEMORY_ACCESS_X = 0x01,
    SCE_KERNEL_MEMORY_ACCESS_W = 0x02,
    SCE_KERNEL_MEMORY_ACCESS_R = 0x04,
}

#[repr(i32)]
pub enum SceKernelMemoryType {
    SCE_KERNEL_MEMORY_TYPE_NORMAL_NC = 0x80,
    SCE_KERNEL_MEMORY_TYPE_NORMAL = 0xD0,
}

#[link(kind = "static", name = "SceLibKernel_stub")]
extern "C" {
    pub fn sceKernelAllocMemBlock(
        name: *const u8,
        type_: SceKernelMemBlockType,
        size: i32,
        optp: *mut SceKernelAllocMemBlockOpt,
    ) -> SceUID;
    pub fn sceKernelFreeMemBlock(uid: SceUID) -> i32;
    pub fn sceKernelGetMemBlockBase(uid: SceUID, basep: *mut *mut ::void) -> i32;
    pub fn sceKernelFindMemBlockByAddr(addr: *const ::void, size: SceSize) -> SceUID;
    pub fn sceKernelGetMemBlockInfoByAddr(
        base: *mut ::void,
        info: *mut SceKernelMemBlockInfo,
    ) -> i32;
    pub fn sceKernelGetMemBlockInfoByRange(
        base: *mut ::void,
        size: SceSize,
        info: *mut SceKernelMemBlockInfo,
    ) -> i32;
    pub fn sceKernelAllocMemBlockForVM(name: *const u8, size: SceSize) -> SceUID;
    pub fn sceKernelSyncVMDomain(uid: SceUID, data: *const ::void, size: SceSize) -> i32;
    pub fn sceKernelOpenVMDomain() -> i32;
    pub fn sceKernelCloseVMDomain() -> i32;
    pub fn sceKernelOpenMemBlock(name: *const u8, flags: i32) -> i32;
    pub fn sceKernelCloseMemBlock(uid: SceUID) -> i32;
    pub fn sceKernelGetModelForCDialog() -> SceKernelModel;
    pub fn sceKernelGetModel() -> SceKernelModel;
    pub fn sceKernelGetFreeMemorySize(info: *mut SceKernelFreeMemorySizeInfo) -> i32;
}
