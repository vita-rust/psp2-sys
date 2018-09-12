use types::SceChar8;
use types::SceInt32;
use types::SceUInt32;

use super::common::SceCommonDialogParam;
use super::common::SceCommonDialogStatus;

#[repr(C)]
pub enum SceMsgDialogErrorCode {
    SCE_MSG_DIALOG_ERROR_PARAM = 0x80100A01, // Illegal parameter
}

/// Max length of a user message
pub const SCE_MSG_DIALOG_USER_MSG_SIZE: usize = 512;

#[repr(C)]
pub enum SceMsgDialogMode {
    SCE_MSG_DIALOG_MODE_INVALID = 0,
    SCE_MSG_DIALOG_MODE_USER_MSG = 1,
    SCE_MSG_DIALOG_MODE_SYSTEM_MSG = 2,
    SCE_MSG_DIALOG_MODE_ERROR_CODE = 3,
    SCE_MSG_DIALOG_MODE_PROGRESS_BAR = 4,
}

#[repr(C)]
pub enum SceMsgDialogSystemMessageType {
    SCE_MSG_DIALOG_SYSMSG_TYPE_INVALID = 0,
    // Displays "Please wait."
    SCE_MSG_DIALOG_SYSMSG_TYPE_WAIT = 1,
    // Displays "There is not enough free space on the memory card."
    SCE_MSG_DIALOG_SYSMSG_TYPE_NOSPACE = 2,
    // Displays "Move away from the source of interference, or adjust the compass by moving your PS Vita system as shown below."
    SCE_MSG_DIALOG_SYSMSG_TYPE_MAGNETIC_CALIBRATION = 3,
    // Displays "Please wait." in a small message dialog
    SCE_MSG_DIALOG_SYSMSG_TYPE_WAIT_SMALL = 5,
    // Displays "Please wait..." with a cancel button
    SCE_MSG_DIALOG_SYSMSG_TYPE_WAIT_CANCEL = 6,
    // Displays "Cannot continue the application. No memory card is inserted."
    SCE_MSG_DIALOG_SYSMSG_TYPE_NEED_MC_CONTINUE = 7,
    // Displays "Cannot perform this operation. No memory card is inserted."
    SCE_MSG_DIALOG_SYSMSG_TYPE_NEED_MC_OPERATION = 8,
    // Displays "You must enable the microphone."
    SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_MIC_DISABLED = 100,
    // Displays "You must use Wi-Fi to do this."
    SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_WIFI_REQUIRED_OPERATION = 101,
    // Displays "You must use Wi-Fi to use this application."
    SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_WIFI_REQUIRED_APPLICATION = 102,
    // Displays "No content is available yet."
    SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_EMPTY_STORE = 103,
}

#[repr(C)]
pub enum SceMsgDialogButtonType {
    SCE_MSG_DIALOG_BUTTON_TYPE_OK = 0,
    SCE_MSG_DIALOG_BUTTON_TYPE_YESNO = 1,
    SCE_MSG_DIALOG_BUTTON_TYPE_NONE = 2,
    SCE_MSG_DIALOG_BUTTON_TYPE_OK_CANCEL = 3,
    SCE_MSG_DIALOG_BUTTON_TYPE_CANCEL = 4,
    SCE_MSG_DIALOG_BUTTON_TYPE_3BUTTONS = 5,
}

#[repr(C)]
pub enum SceMsgDialogButtonId {
    SCE_MSG_DIALOG_BUTTON_ID_INVALID = 0,
    // SCE_MSG_DIALOG_BUTTON_ID_OK = 1,
    SCE_MSG_DIALOG_BUTTON_ID_YES = 1,
    SCE_MSG_DIALOG_BUTTON_ID_NO = 2,
    SCE_MSG_DIALOG_BUTTON_ID_RETRY = 3,
}

#[repr(C)]
pub enum SceMsgDialogProgressBarType {
    SCE_MSG_DIALOG_PROGRESSBAR_TYPE_PERCENTAGE = 0,
}

#[repr(C)]
pub enum SceMsgDialogProgressBarTarget {
    SCE_MSG_DIALOG_PROGRESSBAR_TARGET_BAR_DEFAULT = 0,
}

#[repr(C)]
pub enum SceMsgDialogEnvFlag {
    SCE_MSG_DIALOG_ENV_FLAG_DEFAULT = 0,
}

#[repr(C)]
pub enum SceMsgDialogFontSize {
    SCE_MSG_DIALOG_FONT_SIZE_DEFAULT = 0,
    SCE_MSG_DIALOG_FONT_SIZE_SMALL = 1,
}

#[repr(C)]
pub struct SceMsgDialogButtonsParam {
    pub msg1: *const u8,
    pub fontSize1: SceMsgDialogFontSize,
    pub msg2: *const u8,
    pub fontSize2: SceMsgDialogFontSize,
    pub msg3: *const u8,
    pub fontSize3: SceMsgDialogFontSize,
    pub reserved: [SceChar8; 32],
}

#[repr(C)]
pub struct SceMsgDialogUserMessageParam {
    pub buttonType: SceMsgDialogButtonType,
    pub msg: *const SceChar8,
    pub buttonParam: *mut SceMsgDialogButtonsParam,
    pub reserved: [SceChar8; 28],
}

#[repr(C)]
pub struct SceMsgDialogSystemMessageParam {
    pub sysMsgType: SceMsgDialogSystemMessageType,
    pub value: SceInt32,
    pub reserved: [SceChar8; 32],
}

#[repr(C)]
pub struct SceMsgDialogErrorCodeParam {
    pub errorCode: SceInt32,
    pub reserved: SceChar8,
}

#[repr(C)]
pub struct SceMsgDialogProgressBarParam {
    pub barType: SceMsgDialogProgressBarType,
    pub sysMsgParam: SceMsgDialogSystemMessageParam,
    pub msg: *const SceChar8,
    pub reserved: [SceInt32; 8],
}

#[repr(C)]
pub struct SceMsgDialogParam {
    pub sdkVersion: SceUInt32,
    pub commonParam: SceCommonDialogParam,
    pub mode: SceMsgDialogMode,
    pub userMsgParam: *mut SceMsgDialogUserMessageParam,
    pub sysMsgParam: *mut SceMsgDialogSystemMessageParam,
    pub errorCodeParam: *mut SceMsgDialogErrorCodeParam,
    pub progBarParam: *mut SceMsgDialogProgressBarParam,
    pub flag: SceInt32,
    pub reserved: [SceChar8; 32],
}

#[repr(C)]
pub struct SceMsgDialogResult {
    pub mode: SceMsgDialogMode,
    pub result: SceInt32,
    pub buttonId: SceMsgDialogButtonId,
    pub reserved: [SceChar8; 32],
}

// static inline
// void sceMsgDialogParamInit(SceMsgDialogParam *param)
// {
// 	memset( param, 0x0, sizeof(SceMsgDialogParam) );
// 	_sceCommonDialogSetMagicNumber( &param->commonParam );
// 	param->sdkVersion = 0x03150021;
// }

#[cfg_attr(
    not(feature = "dox"),
    link(name = "SceCommonDialog_stub", kind = "static")
)]
extern "C" {
    pub fn sceMsgDialogInit(param: *const SceMsgDialogParam) -> i32;
    pub fn sceMsgDialogGetStatus() -> SceCommonDialogStatus;
    pub fn sceMsgDialogAbort() -> i32;
    pub fn sceMsgDialogGetResult(result: *mut SceMsgDialogResult) -> i32;
    pub fn sceMsgDialogTerm() -> i32;
    pub fn sceMsgDialogClose() -> i32;
    pub fn sceMsgDialogProgressBarInc(
        target: SceMsgDialogProgressBarTarget,
        delta: SceUInt32,
    ) -> i32;
    pub fn sceMsgDialogProgressBarSetValue(
        target: SceMsgDialogProgressBarTarget,
        rate: SceUInt32,
    ) -> i32;
    pub fn sceMsgDialogProgressBarSetMsg(
        target: SceMsgDialogProgressBarTarget,
        barMsg: *const SceChar8,
    ) -> i32;
}
