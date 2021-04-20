//! SCE Display

use crate::types::SceSize;
use crate::types::SceUID;

/// SCE Display Error Codes
#[repr(u32)]
pub enum SceDisplayErrorCode {
    SCE_DISPLAY_ERROR_OK = 0,
    SCE_DISPLAY_ERROR_INVALID_HEAD = 0x80290000,
    SCE_DISPLAY_ERROR_INVALID_VALUE = 0x80290001,
    SCE_DISPLAY_ERROR_INVALID_ADDR = 0x80290002,
    SCE_DISPLAY_ERROR_INVALID_PIXELFORMAT = 0x80290003,
    SCE_DISPLAY_ERROR_INVALID_PITCH = 0x80290004,
    SCE_DISPLAY_ERROR_INVALID_RESOLUTION = 0x80290005,
    SCE_DISPLAY_ERROR_INVALID_UPDATETIMING = 0x80290006,
    SCE_DISPLAY_ERROR_NO_FRAME_BUFFER = 0x80290007,
    SCE_DISPLAY_ERROR_NO_PIXEL_DATA = 0x80290008,
    SCE_DISPLAY_ERROR_NO_OUTPUT_SIGNAL = 0x80290009,
}

/// SCE Display Pixel Format
#[repr(i32)]
pub enum SceDisplayPixelFormat {
    SCE_DISPLAY_PIXELFORMAT_A8B8G8R8 = 0x00000000,
}

/// SCE Display Set Buffer Sync
#[repr(i32)]
pub enum SceDisplaySetBufSync {
    /// Buffer change effective immediately
    SCE_DISPLAY_SETBUF_IMMEDIATE = 0,
    /// Buffer change effective next frame
    SCE_DISPLAY_SETBUF_NEXTFRAME = 1,
}

/// Structure used with ::sceDisplaySetFrameBuf to set/update
/// framebuffer
/// 
/// Original screen resolution is 960x544, but the following resolutions can also
/// be supplied as width and height : 480x272, 640x368, 720x408
/// 
/// > Note: This structure is returned by ::sceDisplayGetFrameBuf
#[repr(C)]
pub struct SceDisplayFrameBuf {
    /// sizeof(SceDisplayFrameBuf)
    pub size: SceSize,
    /// Pointer to framebuffer
    pub base: *mut crate::void,
    /// Pitch pixels
    pub pitch: u32,
    /// Pixel format (one of ::SceDisplayPixelFormat)
    pub pixelformat: u32,
    /// Framebuffer width
    pub width: u32,
    /// Framebuffer height
    pub height: u32,
}

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceDisplay_stub")
)]
extern "C" {
    /// Set/Update framebuffer parameters
    /// 
    /// Returns 0 on success, < 0 on error
    ///
    /// * `pParam` (out) - Pointer to a ::SceDisplayFrameBuf
    /// structure
    /// * `sync` - One of ::SceDisplaySetBufSync
    ///
    /// > Note: If NULL is provided as pParam pointer, output is blacked out
    pub fn sceDisplaySetFrameBuf(
        pParam: *const SceDisplayFrameBuf,
        sync: SceDisplaySetBufSync,
    ) -> i32;

    /// Get current framebuffer parameters
    /// 
    /// Returns 0 on success, < 0 on error
    ///     
    /// * `pParam` (out) - Pointer to a ::SceDisplayFrameBuf
    /// structure which will receive framebuffer parameters
    /// * `sync` - One of ::SceDisplaySetBufSync
    pub fn sceDisplayGetFrameBuf(
        pParam: *mut SceDisplayFrameBuf,
        sync: SceDisplaySetBufSync,
    ) -> i32;

    /// Primary display index
    pub fn sceDisplayGetPrimaryHead() -> i32;

    /// Get current number of fps for the current screen mode
    /// 
    /// Returns 0 on success, < 0 on error
    ///    
    /// * `pFps` (out) - Pointer to a float variable to store current number of
    /// fps
    ///
    /// > Note: This function returns a theoretical value, this might not be
    /// the exact frame rate
    pub fn sceDisplayGetRefreshRate(
        pFps: *mut f32
    ) -> i32;

    /// Get maximum framebuffer resolution
    ///
    /// Returns 0 on success, < 0 on error
    ///
    /// * `width` (out) - Maximum width
    /// * `height` (out) - Maximum height
    pub fn sceDisplayGetMaximumFrameBufResolution(
        width: *mut i32,
        height: *mut i32
    ) -> i32;

    /// Number of vertical blank pulses up to now
    pub fn sceDisplayGetVcount() -> i32;

    /// Number of vertical blank pulses up to now for a display
    ///
    /// * `display` - Display index.
    pub fn sceDisplayGetVcountInternal(
        display: i32
    ) -> i32;

    /// Wait for vertical blank start
    pub fn sceDisplayWaitVblankStart() -> i32;

    /// Wait for vertical blank start with callback
    pub fn sceDisplayWaitVblankStartCB() -> i32;

    /// Wait for vertical blank start after specified number of vertical
    /// periods
    ///
    /// * `vcount` - Number of vertical periods before waiting for vertical
    /// blank start
    pub fn sceDisplayWaitVblankStartMulti(
        vcount: u32
    ) -> i32;

    /// Wait for vertical blank start with callback after specified number of
    /// vertical periods
    /// 
    /// * `vcount` - Number of vertical periods before waiting for vertical
    /// blank start
    pub fn sceDisplayWaitVblankStartMultiCB(
        vcount: u32
    ) -> i32;

    /// Wait for vertical blank start since last update of framebuffer
    pub fn sceDisplayWaitSetFrameBuf() -> i32;

    /// Wait for vertical blank start with callback since last update of
    /// framebuffer
    pub fn sceDisplayWaitSetFrameBufCB() -> i32;

    /// Wait for vertical blank start after specified number of vertical
    /// periods since last update of framebuffer
    /// 
    /// * `vcount` - Number of vertical periods before waiting for vertical
    /// blank start
    pub fn sceDisplayWaitSetFrameBufMulti(
        vcount: u32
    ) -> i32;

    /// Wait for vertical blank start with callback after specified number of
    /// vertical periods since last update of framebuffer
    ///
    /// * `vcount` - Number of vertical periods before waiting for vertical
    /// blank start
    pub fn sceDisplayWaitSetFrameBufMultiCB(
        vcount: u32
    ) -> i32;

    /// Register callback to be used at each vertical blank start
    /// 
    /// * `uid` - Callback UID
    pub fn sceDisplayRegisterVblankStartCallback(
        uid: SceUID
    ) -> i32;

    /// Unregister callback used at each vertical blank start
    /// 
    /// * `uid` - Callback UID
    pub fn sceDisplayUnregisterVblankStartCallback(
        uid: SceUID
    ) -> i32;

}
