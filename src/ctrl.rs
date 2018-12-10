use crate::types::SceUChar8;
use crate::types::SceUInt;
use crate::types::SceUInt64;
use crate::types::SceUInt8;

#[repr(C)]
pub enum SceCtrlErrorCode {
    SCE_CTRL_ERROR_INVALID_ARG = 0x80340001,
    SCE_CTRL_ERROR_PRIV_REQUIRED = 0x80340002,
    SCE_CTRL_ERROR_NO_DEVICE = 0x80340020,
    SCE_CTRL_ERROR_NOT_SUPPORTED = 0x80340021,
    SCE_CTRL_ERROR_INVALID_MODE = 0x80340022,
    SCE_CTRL_ERROR_FATAL = 0x803400FF,
}

#[repr(C)]
pub enum SceCtrlButtons {
    /// Select button.
    SCE_CTRL_SELECT = 0x00000001,
    /// L3 button.
    SCE_CTRL_L3 = 0x00000002,
    /// R3 button.
    SCE_CTRL_R3 = 0x00000004,
    /// Start button.
    SCE_CTRL_START = 0x00000008,
    /// Up D-Pad button.
    SCE_CTRL_UP = 0x00000010,
    /// Right D-Pad button.
    SCE_CTRL_RIGHT = 0x00000020,
    /// Down D-Pad button.
    SCE_CTRL_DOWN = 0x00000040,
    /// Left D-Pad button.
    SCE_CTRL_LEFT = 0x00000080,
    /// Left trigger.
    SCE_CTRL_LTRIGGER = 0x00000100,
    // /// L2 button.
    // SCE_CTRL_L2 = SCE_CTRL_LTRIGGER,
    /// Right trigger.
    SCE_CTRL_RTRIGGER = 0x00000200,
    // /// R2 button.
    // SCE_CTRL_R2 = SCE_CTRL_RTRIGGER,
    /// L1 button.
    SCE_CTRL_L1 = 0x00000400,
    /// R1 button.
    SCE_CTRL_R1 = 0x00000800,
    /// Triangle button.
    SCE_CTRL_TRIANGLE = 0x00001000,
    /// Circle button.
    SCE_CTRL_CIRCLE = 0x00002000,
    /// Cross button.
    SCE_CTRL_CROSS = 0x00004000,
    /// Square button.
    SCE_CTRL_SQUARE = 0x00008000,
    /// Input not available because intercercepted by another application
    SCE_CTRL_INTERCEPTED = 0x00010000,
    // /// Playstation (Home) button.
    // SCE_CTRL_PSBUTTON = SCE_CTRL_INTERCEPTED,
    /// Headphone plugged in.
    SCE_CTRL_HEADPHONE = 0x00080000,
    /// Volume up button.
    SCE_CTRL_VOLUP = 0x00100000,
    /// Volume down button.
    SCE_CTRL_VOLDOWN = 0x00200000,
    /// Power button.
    SCE_CTRL_POWER = 0x40000000,
}

#[repr(u8)]
pub enum SceCtrlExternalInputMode {
    /// Unpaired controller
    SCE_CTRL_TYPE_UNPAIRED = 0,
    /// Physical controller for VITA
    SCE_CTRL_TYPE_PHY = 1,
    /// Virtual controller for PSTV
    SCE_CTRL_TYPE_VIRT = 2,
    /// DualShock 3
    SCE_CTRL_TYPE_DS3 = 4,
    /// DualShock 4
    SCE_CTRL_TYPE_DS4 = 8,
}

#[repr(C)]
pub enum SceCtrlPadInputMode {
    /// Digital buttons only.
    SCE_CTRL_MODE_DIGITAL = 0,
    /// Digital buttons + Analog support.
    SCE_CTRL_MODE_ANALOG = 1,
    /// Same as ::SCE_CTRL_MODE_ANALOG, but with larger range for analog sticks.
    SCE_CTRL_MODE_ANALOG_WIDE = 2,
}

#[repr(C)]
pub struct SceCtrlData {
    /// The current read frame.
    pub timeStamp: u64,
    /** Bit mask containing zero or more of ::SceCtrlButtons. */
    pub buttons: u32,
    /** Left analogue stick, X axis. */
    pub lx: u8,
    /** Left analogue stick, Y axis. */
    pub ly: u8,
    /** Right analogue stick, X axis. */
    pub rx: u8,
    /** Right analogue stick, Y axis. */
    pub ry: u8,
    /** Up button */
    pub up: u8,
    /** Right button */
    pub right: u8,
    /** Down button */
    pub down: u8,
    /** Left button */
    pub left: u8,
    /** Left trigger (L2) */
    pub lt: u8,
    /** Right trigger (R2) */
    pub rt: u8,
    /** Left button (L1) */
    pub l1: u8,
    /** Right button (R1) */
    pub r1: u8,
    /** Triangle button */
    pub triangle: u8,
    /// Circle button
    pub circle: u8,
    /** Cross button */
    pub cross: u8,
    /** Square button */
    pub square: u8,
    /** Reserved. */
    _reserved: [u8; 4],
}

#[repr(C)]
pub struct SceCtrlRapidFireRule {
    pub Mask: u32,
    pub Trigger: u32,
    pub Target: u32,
    pub Delay: u32,
    pub Make: u32,
    pub Break: u32,
}

#[repr(C)]
pub struct SceCtrlActuator {
    pub small: u8,
    pub large: u8,
    unk: [u8; 6],
}

#[repr(C)]
pub struct SceCtrlPortInfo {
    pub port: [SceCtrlExternalInputMode; 5],
    unk: [u8; 11],
}

#[cfg_attr(not(feature = "dox"), link(kind = "static", name = "SceCtrl_stub"))]
extern "C" {
    pub fn sceCtrlSetSamplingMode(mode: SceCtrlPadInputMode) -> i32;
    pub fn sceCtrlSetSamplingModeExt(mode: SceCtrlPadInputMode) -> i32;
    pub fn sceCtrlGetSamplingMode(pMode: *mut SceCtrlPadInputMode) -> i32;
    pub fn sceCtrlPeekBufferPositive(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;
    pub fn sceCtrlPeekBufferPositiveExt2(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;
    pub fn sceCtrlPeekBufferNegative(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;
    pub fn sceCtrlReadBufferPositive(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;
    pub fn sceCtrlReadBufferPositiveExt2(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;
    pub fn sceCtrlReadBufferNegative(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;
    pub fn sceCtrlSetRapidFire(port: i32, idx: i32, pRule: *const SceCtrlRapidFireRule);
    pub fn sceCtrlClearRapidFire(port: i32, idx: i32) -> i32;
    pub fn sceCtrlSetActuator(port: i32, pState: *const SceCtrlActuator) -> i32;
    pub fn sceCtrlSetLightBar(port: i32, r: u8, g: u8, b: u8) -> i32;
    pub fn sceCtrlGetControllerPortInfo(info: *mut SceCtrlPortInfo) -> i32;
    pub fn sceCtrlGetBatteryInfo(port: i32, batt: *mut u8) -> i32;
    pub fn sceCtrlSetButtonIntercept(intercept: i32) -> i32;
    pub fn sceCtrlGetButtonIntercept(intercept: *mut i32) -> i32;
    pub fn sceCtrlIsMultiControllerSupported() -> i32;
}
