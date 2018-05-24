use ::types::SceUID;
use ::types::SceSize;
use ::types::SceUInt32;

#[repr(u32)]
pub enum SceKernelMemBlockType {
    SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE          = 0x0C208060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_RW                  = 0x0C20D060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_RW     = 0x0C80D060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_RW  = 0x0D808060,
    SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW            = 0x09408060
}

#[repr(C)]
pub struct SceKernelAllocMemBlockOpt {
        size: SceSize,
        attr: SceUInt32,
        alignment: SceSize,
        uidBaseBlock: SceUInt32,
        strBaseBlockName: *const u8,
        flags: i32,
        reserved: [i32; 10],
}

#[link(kind = "static", name = "SceLibKernel_stub")]
extern "C" {
    pub fn sceKernelAllocMemBlock(name: *const u8, type_: SceKernelMemBlockType, size: i32, optp: *mut SceKernelAllocMemBlockOpt) -> SceUID;
    pub fn sceKernelGetMemBlockBase(uid: SceUID, basep: *mut *mut ::void) -> i32;
    pub fn sceKernelExitProcess(res: i32) -> i32;
}
