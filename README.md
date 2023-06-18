# `psp2-sys`

*Unsafe Rust FFI bindings to the [`psp2` headers](https://github.com/vitasdk/vita-headers/)*.

[![TravisCI](https://img.shields.io/travis/vita-rust/psp2-sys/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/vita-rust/psp2-sys/builds)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/vita-rust/psp2-sys)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
[![Crate](https://img.shields.io/crates/v/psp2-sys.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/psp2-sys)
[![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/psp2-sys)
[![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)

## Deprecation notice

Deprecated in favor of [vitasdk-sys](https://github.com/vita-rust/vitasdk-sys)

## Usage

This crate will link statically to the required library stubs, so you'll need the
[`vitasdk`](https://vitasdk.org) set up and the `$VITASDK` environment variable set.

See the [examples](https://github.com/vita-rust/psp2-sys/tree/master/examples)
to see how to setup a project using the raw bindings, or use the
[`vita`](https://github.com/vita-rust/vita) crate for a safe API closer to the
Rust [`std`](http://doc.rust-lang.org/nightly/std/) library.

The submodules in this crate follow the file hierarchy of the `psp2` headers:
so the FFI in the `psp2_sys::kernel::threadmgr` are the ones you would get in C
after including `psp2/kernel/threadmgr.h`.

## Credits

* [**VitaSDK team**](http://vitasdk.org/) for the `arm-vita-eabi` toolchain, `psp2` headers, ...
* [**Team Molecule**](http://henkaku.xyz/) for the `Henkaku` hard work.


## Disclaimer

*`psp2-sys` is not affiliated, sponsored, or otherwise endorsed by Sony
Interactive Entertainment, LLC. PlayStation and PS Vita are trademarks or
registered trademarks of Sony Interactive Entertainment, LLC. This software is
provided "as is" without warranty of any kind under the MIT License.*
