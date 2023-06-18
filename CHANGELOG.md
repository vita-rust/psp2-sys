# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added
- Complete **userspace** bindings to:
  * the [Control Library]
  * the [Display Library]
  * the *threading* part of the [Thread Manager Library]
  * the *semaphore* part of the [Thread Manager Library]
- Partial **userspace** bindings to:
  * the *event flags* part of the [Thread Manager Library]
- Complete documentation for:
  * the [Type Defines]
  * the [System Parameters Defines]
  * the [Display Library]
  * the [Control Library]
  * the [Random Number Library]
- Partial documentation for:
  * the *event flags* part of the [Thread Manager Library]

### Removed
- `SceUVector4` & `SceUMatrix4` structures from `types` (not present in types.h).
- `SceUnion32` & `SceUnion64` & `SceUnion128` union from `types` (not present in types.h)

### Fixed
- Fix `i32` being used instead of `u32` as `psp2_sys::types::SceUInt`

### Changed
- Type of variable used by `size` parameter of `ctrl::sceKernelGetRandomNumber` function. (`SceSize` instead of `u32`)
- Type of variable used by `port` variable in struct `SceCtrlPortInfo` (`u8` instead of `SceCtrlExternalInputMode`)
- Type of variable used by `r`, `g`, `b` parameters of `ctrl::sceCtrlSetLightBar` function. (`SceUInt8` instead of `u8`)
- Type of variable used by `batt` parameters of `ctrl::sceCtrlGetBatteryInfo` function. (`SceUInt8` instead of `u8`)


## [v0.2.2] - 2018-09-12
### Fixed
- Fix missing `#[cfg_attr(...)]` in `psp_sys::kernel`

## [v0.2.1] - 2018-09-12
### Added
- `dox` feature to disable linking to `vitasdk` stubs during compilation (use in  `docs.rs`)

### Fixed
- Links in `CHANGELOG.md` file.


## [v0.2.0] - 2018-07-06
### Added

- Complete **userspace** bindings to:
  * the [Device Control Library]
  * the [Directory Entries Library]
  * the [File Control Library]
  * the [File Status Library]
  * the [C Standard Library]
  * the [Random Number Library]
- Partial **userspace** bindings to:
  * the *threading* part of the [Thread Manager Library]
  * the *semaphore* part of the [Thread Manager Library]


## [v0.1.1] - 2018-07-04
### Changed

- `build.rs` only to warn about missing `$VITASDK` environment variable
  (allows `cargo doc` without `$VITASDK` set)


## [v0.1.0] - 2018-07-04
### Added

- Complete **userspace** bindings to:
  * the [System Memory Library]
  * the [System Parameters Defines]
  * the [Type Defines]
  * the [Common Dialog Library]
  * the [Message Dialog Library]
- Partial **userspace** bindings to:
  * the *mutex* part of the [Thread Manager Library]
  * the *color formats* of the [GPU Graphics Library]
- A basic README with links to other `vita-rust` projects.
- This CHANGELOG file.


[Unreleased]: https://github.com/vita-rust/psp2-sys/compare/v0.2.2...HEAD
[v0.2.2]: https://github.com/vita-rust/psp2-sys/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/vita-rust/psp2-sys/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/vita-rust/psp2-sys/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/vita-rust/psp2-sys/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/vita-rust/psp2-sys/compare/147a58f...v0.1.0


[Control Library]: https://docs.vitasdk.org/group__SceCtrlUser.html
[Display Library]: https://docs.vitasdk.org/group__SceDisplayUser.html
[Random Number Library]: https://docs.vitasdk.org/group__SceRngUser.html
[Device Control Library]: https://docs.vitasdk.org/group__SceDevCtlUser.html
[Directory Entries Library]: https://docs.vitasdk.org/group__SceDirEntUser.html
[File Control Library]: https://docs.vitasdk.org/group__SceFcntlUser.html
[File Status Library]: https://docs.vitasdk.org/group__SceStatUser.html
[C Standard Library]: https://docs.vitasdk.org/group__SceCLibUser.html
[Common Dialog Library]: https://docs.vitasdk.org/group__SceCommonDialogUser.html
[Message Dialog Library]: https://docs.vitasdk.org/group__SceMessageDialogUser.html
[GPU Graphics Library]: https://docs.vitasdk.org/group__SceGxmUser.html
[System Parameters Defines]: https://docs.vitasdk.org/group__SceSystemParamUser.html
[System Memory Library]: https://docs.vitasdk.org/group__SceSysmemUser.html
[Thread Manager Library]: https://docs.vitasdk.org/group__SceThreadMgrUser.html
[Type Defines]: https://docs.vitasdk.org/group__SceTypesUser.html
