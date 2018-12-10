pub type SceChar8 = i8;
pub type SceUChar8 = u8;

pub type SceInt8 = i8;
pub type SceUInt8 = u8;

pub type SceShort16 = i16;
pub type SceUShort16 = u16;

pub type SceInt32 = i32;
pub type SceUInt32 = u32;

pub type SceInt = i32;
pub type SceUInt = u32;

pub type SceInt64 = i64;
pub type SceUInt64 = u64;

pub type SceLong64 = i64;
pub type SceULong64 = u64;

pub type SceSize = u32;
pub type SceSSize = i32;

#[repr(i32)]
pub enum SceBool {
    SCE_FALSE = 0,
    SCE_TRUE = 1,
}

pub type SceFloat = f32;
pub type SceFloat32 = f32;

pub type SceDouble = f64;
pub type SceDouble64 = f64;

pub type SceSByte = i8;
pub type SceSByte8 = i8;

pub type SceByte = u8;
pub type SceByte8 = u8;

pub type SceWChar16 = u16;
pub type SceWChar32 = u32;

pub type SceVoid = ::void;
pub type ScePVoid = *mut ::void;

pub type SceIntPtr = i32;
pub type SceUIntPtr = u32;
pub type SceUIntVAddr = SceUIntPtr;

#[repr(C)]
pub struct SceIVector2 {
    x: SceInt,
    y: SceInt,
}

#[repr(C)]
pub struct SceFVector2 {
    x: SceFloat,
    y: SceFloat,
}

#[repr(C)]
pub struct SceIVector3 {
    x: SceInt,
    y: SceInt,
    z: SceInt,
}

#[repr(C)]
pub struct SceFVector3 {
    x: SceFloat,
    y: SceFloat,
    z: SceFloat,
}

#[repr(C)]
pub struct SceIVector4 {
    x: SceInt,
    y: SceInt,
    z: SceInt,
    w: SceInt,
}

#[repr(C)]
pub struct SceUVector4 {
    x: SceUInt,
    y: SceUInt,
    z: SceUInt,
    w: SceUInt,
}

#[repr(C)]
pub struct SceFVector4 {
    x: SceFloat,
    y: SceFloat,
    z: SceFloat,
    w: SceFloat,
}

#[repr(C)]
pub struct SceIMatrix2 {
    x: SceIVector2,
    y: SceIVector2,
}

#[repr(C)]
pub struct SceFMatrix2 {
    x: SceFVector2,
    y: SceFVector2,
}

#[repr(C)]
pub struct SceIMatrix3 {
    x: SceIVector3,
    y: SceIVector3,
    z: SceIVector3,
}

#[repr(C)]
pub struct SceFMatrix3 {
    x: SceFVector3,
    y: SceFVector3,
    z: SceFVector3,
}

#[repr(C)]
pub struct SceIMatrix4 {
    x: SceIVector4,
    y: SceIVector4,
    z: SceIVector4,
    w: SceIVector4,
}

#[repr(C)]
pub struct SceUMatrix4 {
    x: SceUVector4,
    y: SceUVector4,
    z: SceUVector4,
    w: SceUVector4,
}

#[repr(C)]
pub struct SceFMatrix4 {
    x: SceFVector4,
    y: SceFVector4,
    z: SceFVector4,
    w: SceFVector4,
}

#[repr(C)]
pub struct SceFQuaternion {
    x: SceFloat,
    y: SceFloat,
    z: SceFloat,
    w: SceFloat,
}

#[repr(C)]
pub struct SceFColor {
    r: SceFloat,
    g: SceFloat,
    b: SceFloat,
    a: SceFloat,
}

#[repr(C)]
pub struct SceFPlane {
    a: SceFloat,
    b: SceFloat,
    c: SceFloat,
    d: SceFloat,
}

#[repr(C)]
pub union SceUnion32 {
    ui: u32,
    i: u32,
    us: [u16; 2],
    s: [i16; 2],
    uc: [u8; 4],
    c: [i8; 4],
    f: f32,
    p: *mut ::void,
}

#[repr(C)]
pub union SceUnion64 {
    ull: SceULong64,
    ll: SceLong64,
    ui: [u32; 2],
    i: [i32; 2],
    us: [u16; 4],
    s: [i16; 4],
    uc: [u8; 8],
    c: [i8; 8],
    f: [f32; 2],
    fv: SceFVector2,
    iv: SceIVector2,
}

#[repr(C)]
pub union SceUnion128 {
    ull: [SceULong64; 2],
    ll: [SceLong64; 2],
    ui: [u32; 4],
    i: [i32; 4],
    us: [u16; 8],
    s: [i16; 8],
    uc: [u8; 16],
    c: [u8; 16],
    f: [f32; 4],
    fv: SceFVector4,
    fq: SceFQuaternion,
    fp: SceFPlane,
    fc: SceFColor,
    iv: SceIVector4,
}

#[repr(C)]
pub struct SceDateTime {
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    microsecond: u32,
}

pub type SceMode = i32;
pub type SceOff = SceInt64;

pub type SceUID = i32;

pub type ScePID = i32;

pub type SceName = *mut u8;
