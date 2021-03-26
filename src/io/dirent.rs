use crate::types::SceUID;

use super::stat::SceIoStat;

#[repr(C)]
pub struct SceIoDirent {
    pub d_stat: SceIoStat,
    pub d_name: [u8; 256],
    pub d_private: *mut crate::void,
    pub dummy: i32,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    pub fn sceIoDopen(dirname: *const u8) -> SceUID;
    pub fn sceIoDread(fd: SceUID, dir: *mut SceIoDirent) -> i32;
    pub fn sceIoDclose(fd: SceUID) -> i32;
}
