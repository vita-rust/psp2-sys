use ::types::SceSize;
use ::types::SceUID;

#[repr(u32)]
pub enum SceDisplayErrorCode {
    SCE_DISPLAY_ERROR_OK                    = 0,
    SCE_DISPLAY_ERROR_INVALID_HEAD          = 0x80290000,
    SCE_DISPLAY_ERROR_INVALID_VALUE         = 0x80290001,
    SCE_DISPLAY_ERROR_INVALID_ADDR          = 0x80290002,
    SCE_DISPLAY_ERROR_INVALID_PIXELFORMAT   = 0x80290003,
    SCE_DISPLAY_ERROR_INVALID_PITCH         = 0x80290004,
    SCE_DISPLAY_ERROR_INVALID_RESOLUTION    = 0x80290005,
    SCE_DISPLAY_ERROR_INVALID_UPDATETIMING  = 0x80290006,
    SCE_DISPLAY_ERROR_NO_FRAME_BUFFER       = 0x80290007,
    SCE_DISPLAY_ERROR_NO_PIXEL_DATA         = 0x80290008,
    SCE_DISPLAY_ERROR_NO_OUTPUT_SIGNAL      = 0x80290009
}

#[repr(i32)]
pub enum SceDisplayPixelFormat {
    SCE_DISPLAY_PIXELFORMAT_A8B8G8R8        = 0x00000000,
}

#[repr(i32)]
pub enum SceDisplaySetBufSync {
        /** Buffer change effective immediately */
        SCE_DISPLAY_SETBUF_IMMEDIATE = 0,
        /** Buffer change effective next frame */
        SCE_DISPLAY_SETBUF_NEXTFRAME = 1
}

#[repr(C)]
pub struct SceDisplayFrameBuf {
        pub size: SceSize,      //  sizeof(SceDisplayFrameBuf)
        pub base: *mut ::void,  //  Pointer to framebuffer
        pub pitch: u32,         //  pitch pixels
        pub pixelformat: u32,   //  pixel format (one of ::SceDisplayPixelFormat)
        pub width: u32,         //  framebuffer width
        pub height: u32         //  framebuffer height
}

#[link(kind = "static", name = "SceDisplay_stub")]
extern "C" {
    pub fn sceDisplaySetFrameBuf(pParam: *const SceDisplayFrameBuf, sync: SceDisplaySetBufSync) -> i32;
    pub fn sceDisplayGetFrameBuf(pParam: *mut SceDisplayFrameBuf, sync: SceDisplaySetBufSync) -> i32;
    pub fn sceDisplayGetPrimaryHead() -> i32;
    pub fn sceDisplayGetRefreshRate(pFps: *mut f32) -> i32;
    pub fn sceDisplayGetMaximumFrameBufResolution(width: *mut i32, height: *mut i32) -> i32;
    pub fn sceDisplayGetVcount() -> i32;
    pub fn sceDisplayGetVcountInternal(display: i32) -> i32;
    pub fn sceDisplayWaitVblankStart() -> i32;
    pub fn sceDisplayWaitVblankStartCB() -> i32;
    pub fn sceDisplayWaitVblankStartMulti(vcount: u32) -> i32;
    pub fn sceDisplayWaitVblankStartMultiCB(vcount: u32) -> i32;
    pub fn sceDisplayWaitSetFrameBuf() -> i32;
    pub fn sceDisplayWaitSetFrameBufCB() -> i32;
    pub fn sceDisplayWaitSetFrameBufMulti(vcount: u32) -> i32;
    pub fn sceDisplayWaitSetFrameBufMultiCB(vcount: u32) -> i32;
    pub fn sceDisplayRegisterVblankStartCallback(uid: SceUID) -> i32;
    pub fn sceDisplayUnregisterVblankStartCallback(uid: SceUID) -> i32;
}
