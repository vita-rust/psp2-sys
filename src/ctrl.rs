//! SCE Controls

use crate::types::SceUInt8;

/// Error codes enumeration.
#[repr(u32)]
pub enum SceCtrlErrorCode {
    SCE_CTRL_ERROR_INVALID_ARG = 0x80340001,
    SCE_CTRL_ERROR_PRIV_REQUIRED = 0x80340002,
    SCE_CTRL_ERROR_NO_DEVICE = 0x80340020,
    SCE_CTRL_ERROR_NOT_SUPPORTED = 0x80340021,
    SCE_CTRL_ERROR_INVALID_MODE = 0x80340022,
    SCE_CTRL_ERROR_FATAL = 0x803400FF,
}

/// Enumeration for the digital controller buttons.
///
/// Notes :
/// > L1/R1/L3/R3 only can bind using [sceCtrlPeekBufferPositiveExt2] and [sceCtrlReadBufferPositiveExt2]
///
/// > Values bigger than 0x00010000 can be intercepted only with shell privileges
///
/// > Vita's L Trigger and R Trigger are mapped to L1 and R1 when using [sceCtrlPeekBufferPositiveExt2] and [sceCtrlReadBufferPositiveExt2]
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
    // SCE_CTRL_L2 : defined in [SceCtrlButtons] Implementation
    /// Right trigger.
    SCE_CTRL_RTRIGGER = 0x00000200,
    // SCE_CTRL_R2 : Defined in [SceCtrlButtons] Implementation
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
    // SCE_CTRL_PSBUTTON : Defined in [SceCtrlButtons] Implementation
    /// Headphone plugged in.
    SCE_CTRL_HEADPHONE = 0x00080000,
    /// Volume up button.
    SCE_CTRL_VOLUP = 0x00100000,
    /// Volume down button.
    SCE_CTRL_VOLDOWN = 0x00200000,
    /// Power button.
    SCE_CTRL_POWER = 0x40000000,
}

/// Implementation for [SceCtrlButtons] : Fixing missing enum variant
impl SceCtrlButtons {
    /// L2 button.
    pub const SCE_CTRL_L2: SceCtrlButtons = SceCtrlButtons::SCE_CTRL_LTRIGGER;
    /// R2 button.
    pub const SCE_CTRL_R2: SceCtrlButtons = SceCtrlButtons::SCE_CTRL_RTRIGGER;
    /// Playstation (Home) button.
    pub const SCE_CTRL_PSBUTTON: SceCtrlButtons = SceCtrlButtons::SCE_CTRL_INTERCEPTED;
}

/// Enumeration for the controller types.
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

/// Controller mode.
#[repr(C)]
pub enum SceCtrlPadInputMode {
    /// Digital buttons only.
    SCE_CTRL_MODE_DIGITAL = 0,
    /// Digital buttons + Analog support.
    SCE_CTRL_MODE_ANALOG = 1,
    /// Same as [SCE_CTRL_MODE_ANALOG](crate::ctrl::SceCtrlPadInputMode::SCE_CTRL_MODE_ANALOG), but with larger range for analog sticks.
    SCE_CTRL_MODE_ANALOG_WIDE = 2,
}

/// Returned controller data
#[repr(C)]
pub struct SceCtrlData {
    /// The current read frame.
    pub timeStamp: u64,
    /// Bit mask containing zero or more of [SceCtrlButtons].
    pub buttons: u32,
    /// Left analogue stick, X axis.
    pub lx: u8,
    /// Left analogue stick, Y axis.
    pub ly: u8,
    /// Right analogue stick, X axis.
    pub rx: u8,
    /// Right analogue stick, Y axis.
    pub ry: u8,
    /// Up button
    pub up: u8,
    /// Right button
    pub right: u8,
    /// Down button
    pub down: u8,
    /// Left button
    pub left: u8,
    /// Left trigger (L2)
    pub lt: u8,
    /// Right trigger (R2)
    pub rt: u8,
    /// Left button (L1)
    pub l1: u8,
    /// Right button (R1)
    pub r1: u8,
    /// Triangle button
    pub triangle: u8,
    /// Circle button
    pub circle: u8,
    /// Cross button
    pub cross: u8,
    /// Square button
    pub square: u8,
    /// Reserved.
    _reserved: [u8; 4],
}

/// Structure to pass as argument to [sceCtrlSetRapidFire]
#[repr(C)]
pub struct SceCtrlRapidFireRule {
    pub Mask: u32,
    pub Trigger: u32,
    pub Target: u32,
    pub Delay: u32,
    pub Make: u32,
    pub Break: u32,
}

/// Structure to pass as argument to [sceCtrlSetActuator]
#[repr(C)]
pub struct SceCtrlActuator {
    /// Vibration strength of the small motor
    pub small: u8,
    /// Vibration strength of the large motor
    pub large: u8,
    /// Unknown
    unk: [u8; 6],
}

/// Structure to pass as argument to [sceCtrlGetControllerPortInfo]
#[repr(C)]
pub struct SceCtrlPortInfo {
    /// Controller type of each port (See [SceCtrlExternalInputMode])
    pub port: [u8; 5],
    /// Unknown
    unk: [u8; 11],
}

#[cfg_attr(not(feature = "dox"), link(kind = "static", name = "SceCtrl_stub"))]
extern "C" {
    /// Set the controller mode
    ///
    /// ### Parameters
    /// * `mode` - One of [SceCtrlPadInputMode].
    ///
    /// Returns he previous mode, < 0 on error.
    pub fn sceCtrlSetSamplingMode(mode: SceCtrlPadInputMode) -> i32;

    /// Set the controller extend mode
    ///
    /// ### Parameters
    /// * `mode` - One of [SceCtrlPadInputMode].
    ///
    /// Returns the previous mode, < 0 on error.
    pub fn sceCtrlSetSamplingModeExt(mode: SceCtrlPadInputMode) -> i32;

    /// Get the current controller mode
    ///
    /// ### Parameters
    /// * `pMode` (out) - Return value, see [SceCtrlPadInputMode].
    ///
    /// Returns the previous mode, < 0 on error.
    pub fn sceCtrlGetSamplingMode(pMode: *mut SceCtrlPadInputMode) -> i32;

    /// Get the controller state information (polling, positive logic)
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `*pad_data` (out) - see [SceCtrlData].
    /// * `count` - Buffers count. Up to 64 buffers can be requested.
    ///
    /// Returns buffers count, between 1 and 'count'. < 0 on error.
    pub fn sceCtrlPeekBufferPositive(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;

    /// Get the controller state information (polling, positive logic)
    ///
    /// This function will bind L/R trigger value to L1/R1 instead of LTRIGGER/RTRIGGER.
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `*pad_data` (out) - see [SceCtrlData].
    /// * `count` - Buffers count. Up to 64 buffers can be requested.
    ///
    /// Returns buffers count, between 1 and 'count'. < 0 on error.
    pub fn sceCtrlPeekBufferPositiveExt2(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;

    /// Get the controller state information (polling, negative logic)
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `*pad_data` (out) - see [SceCtrlData].
    /// * `count` - Buffers count. Up to 64 buffers can be requested.
    ///
    /// Returns buffers count, between 1 and 'count'. < 0 on error.
    pub fn sceCtrlPeekBufferNegative(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;

    /// Get the controller state information (blocking, positive logic)
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `*pad_data` (out) - see [SceCtrlData].
    /// * `count` - Buffers count. Up to 64 buffers can be requested.
    ///
    /// Returns buffers count, between 1 and 'count'. < 0 on error.
    pub fn sceCtrlReadBufferPositive(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;

    /// Get the controller extended state information (blocking, positive logic)
    ///
    /// This function will bind L/R trigger value to L1/R1 instead of LTRIGGER/RTRIGGER.
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `*pad_data` (out) - see [SceCtrlData].
    /// * `count` - Buffers count. Up to 64 buffers can be requested.
    ///
    /// Returns buffers count, between 1 and 'count'. < 0 on error.
    pub fn sceCtrlReadBufferPositiveExt2(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;

    /// Get the controller state information (blocking, negative logic)
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `*pad_data` (out) - see [SceCtrlData].
    /// * `count` - Buffers count. Up to 64 buffers can be requested.
    ///
    /// Returns buffers count, between 1 and 'count'. < 0 on error.
    pub fn sceCtrlReadBufferNegative(port: i32, pad_data: *mut SceCtrlData, count: i32) -> i32;

    /// Set rules for button rapid fire
    ///
    /// ### Parameters
    /// * `port` - use 0.
    /// * `idx` - rule index between 0-15.
    /// * `pRule` - structure [SceCtrlRapidFireRule].
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlSetRapidFire(port: i32, idx: i32, pRule: *const SceCtrlRapidFireRule);

    /// Clear rules for button rapid fire
    ///
    /// Parameters
    /// * `port` - use 0.
    /// * `idx` - rule index between 0-15.
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlClearRapidFire(port: i32, idx: i32) -> i32;

    /// Control the actuator (vibrate) on paired controllers
    ///
    /// Parameters
    /// * `port` - use 1 for the first paired controller, etc.
    /// * `state` - see [SceCtrlActuator].
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlSetActuator(port: i32, pState: *const SceCtrlActuator) -> i32;

    /// Control the light bar on paired controllers
    ///
    /// Parameters
    /// * `port` - use 1 for the first paired controller, etc.
    /// * `r` - red intensity.
    /// * `g` - green intensity.
    /// * `b` - blue intensity.
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlSetLightBar(port: i32, r: SceUInt8, g: SceUInt8, b: SceUInt8) -> i32;

    /// Get controller port information.
    ///
    /// Parameters
    /// * `info` (out) - see [SceCtrlPortInfo].
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlGetControllerPortInfo(info: *mut SceCtrlPortInfo) -> i32;

    /// Get controller battery information
    ///
    /// Parameters
    /// * `port` - use 1 for the first paired controller, etc.
    /// * `batt` (out) - battery level, between 0-5, 0xEE charging, 0xEF charged.
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlGetBatteryInfo(port: i32, batt: *mut SceUInt8) -> i32;

    /// Sets intercept
    ///
    /// If true, allows the current thread to intercept controls.
    /// The use case might be, for example, a game plugin that wishes to
    /// capture input without having the input sent to the game thread.
    ///
    /// Parameters
    /// * `intercept` - Boolean value.
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlSetButtonIntercept(intercept: i32) -> i32;

    /// Gets intercept
    ///
    /// Parameters
    /// * `intercept` (out) - Boolean value.
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceCtrlGetButtonIntercept(intercept: *mut i32) -> i32;

    /// Check if multi controller is supported
    ///
    /// Returns 1 if yes, 0 if no.
    pub fn sceCtrlIsMultiControllerSupported() -> i32;
}
