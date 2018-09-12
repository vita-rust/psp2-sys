use graphics::gxm::SceGxmColorFormat;
use graphics::gxm::SceGxmColorSurfaceType;
use graphics::gxm::SceGxmSyncObject;
use system_param::SceSystemParamEnterButtonAssign;
use system_param::SceSystemParamLang;
use types::SceInt32;
use types::ScePVoid;
use types::SceUInt32;
use types::SceUInt8;

#[repr(u32)]
pub enum SceCommonDialogErrorCode {
    SCE_COMMON_DIALOG_ERROR_BUSY = 0x80020401,
    SCE_COMMON_DIALOG_ERROR_NULL = 0x80020402,
    SCE_COMMON_DIALOG_ERROR_INVALID_ARGUMENT = 0x80020403,
    SCE_COMMON_DIALOG_ERROR_NOT_RUNNING = 0x80020404,
    SCE_COMMON_DIALOG_ERROR_NOT_SUPPORTED = 0x80020405,
    SCE_COMMON_DIALOG_ERROR_ILLEGAL_CALLER_THREAD = 0x80020406,
    SCE_COMMON_DIALOG_ERROR_NOT_CONFIGURED = 0x80020407,
    SCE_COMMON_DIALOG_ERROR_NOT_AVAILABLE = 0x80020408,
    SCE_COMMON_DIALOG_ERROR_NOT_FINISHED = 0x80020410,
    SCE_COMMON_DIALOG_ERROR_NOT_IN_USE = 0x80020411,
    SCE_COMMON_DIALOG_ERROR_INVALID_COLOR_FORMAT = 0x80020420,
    SCE_COMMON_DIALOG_ERROR_INVALID_SURFACE_RESOLUTION = 0x80020421,
    SCE_COMMON_DIALOG_ERROR_INVALID_SURFACE_STRIDE = 0x80020422,
    SCE_COMMON_DIALOG_ERROR_INVALID_SURFACE_TYPE = 0x80020423,
    SCE_COMMON_DIALOG_ERROR_WITHIN_SCENE = 0x80020424,
    SCE_COMMON_DIALOG_ERROR_IME_IN_USE = 0x80020430,
    SCE_COMMON_DIALOG_ERROR_INVALID_LANGUAGE = 0x80020431,
    SCE_COMMON_DIALOG_ERROR_INVALID_ENTER_BUTTON_ASSIGN = 0x80020432,
    SCE_COMMON_DIALOG_ERROR_INVALID_INFOBAR_PARAM = 0x80020433,
    SCE_COMMON_DIALOG_ERROR_INVALID_BG_COLOR = 0x80020434,
    SCE_COMMON_DIALOG_ERROR_INVALID_DIMMER_COLOR = 0x80020435,
    SCE_COMMON_DIALOG_ERROR_UNEXPECTED_FATAL = 0x8002047F,
}

#[repr(C)]
pub struct SceCommonDialogConfigParam {
    pub sdkVersion: SceUInt32,
    pub language: SceSystemParamLang,
    pub enterButtonAssign: SceSystemParamEnterButtonAssign,
    pub reserved: [SceUInt8; 32],
}

#[repr(C)]
pub enum SceCommonDialogStatus {
    SCE_COMMON_DIALOG_STATUS_NONE = 0,
    SCE_COMMON_DIALOG_STATUS_RUNNING = 1,
    SCE_COMMON_DIALOG_STATUS_FINISHED = 2,
}

#[repr(C)]
pub enum SceCommonDialogResult {
    SCE_COMMON_DIALOG_RESULT_OK,
    SCE_COMMON_DIALOG_RESULT_USER_CANCELED,
    SCE_COMMON_DIALOG_RESULT_ABORTED,
}

#[repr(C)]
pub struct SceCommonDialogRenderTargetInfo {
    pub depthSurfaceData: ScePVoid,
    pub colorSurfaceData: ScePVoid,
    pub surfaceType: SceGxmColorSurfaceType,
    pub colorFormat: SceGxmColorFormat,
    pub width: SceUInt32,
    pub height: SceUInt32,
    pub strideInPixels: SceUInt32,
    pub reserved: [SceUInt8; 32],
}

#[repr(C)]
pub struct SceCommonDialogUpdateParam {
    pub renderTarget: SceCommonDialogRenderTargetInfo,
    pub displaySyncObject: *mut SceGxmSyncObject,
    pub reserved: [SceUInt8; 32],
}

#[repr(C)]
pub struct SceCommonDialogInfobarParam {
    pub visibility: SceInt32,
    pub color: SceInt32,
    pub transparency: SceInt32,
    pub reserved: [SceUInt8; 32],
}

#[repr(C)]
pub struct SceCommonDialogColor {
    pub r: SceUInt8,
    pub g: SceUInt8,
    pub b: SceUInt8,
    pub a: SceUInt8,
}

pub type SceCommonDialogBgColor = SceCommonDialogColor;

#[repr(C)]
pub struct SceCommonDialogParam {
    pub infobarParam: *mut SceCommonDialogInfobarParam,
    pub bgColor: *mut SceCommonDialogColor,
    pub dimmerColor: *mut SceCommonDialogColor,
    pub reserved: [SceUInt8; 60],
    pub magic: SceUInt32,
}

#[cfg_attr(
    not(feature = "dox"),
    link(name = "SceCommonDialog_stub", kind = "static")
)]
extern "C" {
    pub fn sceCommonDialogSetConfigParam(configParam: *const SceCommonDialogConfigParam) -> i32;
    pub fn sceCommonDialogUpdate(updateParam: *const SceCommonDialogUpdateParam) -> i32;
}

pub const SCE_COMMON_DIALOG_MAGIC_NUMBER: u32 = 0xC0D1A109;

// static inline
// void _sceCommonDialogSetMagicNumber(SceCommonDialogParam *param)
// {
// 	param->magic = SCE_COMMON_DIALOG_MAGIC_NUMBER + *(SceUInt32*)&param;
// }
//
// static inline
// void sceCommonDialogConfigParamInit(SceCommonDialogConfigParam *param)
// {
// 	memset(param, 0x0, sizeof(SceCommonDialogConfigParam));
// 	param->language = SCE_SYSTEM_PARAM_LANG_MAX_VALUE;
// 	param->enterButtonAssign = SCE_SYSTEM_PARAM_ENTER_BUTTON_MAX_VALUE;
// 	param->sdkVersion = 0x03150021;
// };
