# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

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

[Random Number Library]: https://docs.vitasdk.org/group__SceRngUser.html
[Device Control Library]: https://docs.vitasdk.org/group__SceDevCtlUser.html
[Directory Entries Library]: https://docs.vitasdk.org/group__SceDirEntUser.html
[File Control Library]: https://docs.vitasdk.org/group__SceFcntlUser.html
[File Status Library]: https://docs.vitasdk.org/group__SceStatUser.html
[C Standard Library]: https://docs.vitasdk.org/group__SceCLibUser.html
[Thread Manager Library]: https://docs.vitasdk.org/group__SceThreadMgrUser.html


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

[Common Dialog Library]: https://docs.vitasdk.org/group__SceCommonDialogUser.html
[Message Dialog Library]: https://docs.vitasdk.org/group__SceMessageDialogUser.html
[GPU Graphics Library]: https://docs.vitasdk.org/group__SceGxmUser.html
[System Parameters Defines]: https://docs.vitasdk.org/group__SceSystemParamUser.html
[System Memory Library]: https://docs.vitasdk.org/group__SceSysmemUser.html
[Thread Manager Library]: https://docs.vitasdk.org/group__SceThreadMgrUser.html
[Type Defines]: https://docs.vitasdk.org/group__SceTypesUser.html


[Unreleased]: https://github.com/vita-rust/psp2-sys/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/vita-rust/psp2-sys/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/vita-rust/psp2-sys/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/vita-rust/psp2-sys/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/vita-rust/psp2-sys/compare/147a58f...v0.1.0
