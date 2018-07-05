# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added

- Complete **userspace** bindings to:
  * the [Filesystem module]

[Filesystem]: https://docs.vitasdk.org/group__Filesystem.html


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

[Common Dialog Library]: https://docs.vitasdk.org/group__SceCommonDialog.html
[Message Dialog Library]: https://docs.vitasdk.org/group__SceMessageDialog.html
[GPU Graphics Library]: https://docs.vitasdk.org/group__SceGxm.html
[System Parameters Defines]: https://docs.vitasdk.org/group__SceSystemParam.html
[System Memory Library]: https://docs.vitasdk.org/group__SceSysmem.html
[Thread Manager Library]: https://docs.vitasdk.org/group__SceThreadMgr.html
[Type Defines]: https://docs.vitasdk.org/group__SceTypes.html


[Unreleased]: https://github.com/althonos/pruefung/compare/0.1.1...HEAD
[0.1.1]: https://github.com/althonos/pruefung/compare/0.1.0...0.1.1
