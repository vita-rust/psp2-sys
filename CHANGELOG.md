# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added

- Complete **userspace** bindings to:
  * the [Device Control Library]
  * the [Directory Entries Library]
  * the [File Control Library]
  * the [File Status Library]
  * the [C Standard Library]


[Device Control Library]: https://docs.vitasdk.org/group__SceDevCtlUser.html
[Directory Entries Library]: https://docs.vitasdk.org/group__SceDirEntUser.html
[File Control Library]: https://docs.vitasdk.org/group__SceFcntlUser.html
[File Status Library]: https://docs.vitasdk.org/group__SceStatUser.html
[C Standard Library]: https://docs.vitasdk.org/group__SceCLibUser.html


## [0.1.1] - 2018-07-04
### Changed

- Change `build.rs` only to warn about missing `$VITASDK` environment variable
  (allows `cargo doc` without `$VITASDK` set)


## [0.1.0] - 2018-07-04
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


[Unreleased]: https://github.com/althonos/pruefung/compare/0.1.1...HEAD
[0.1.1]: https://github.com/althonos/pruefung/compare/0.1.0...0.1.1
