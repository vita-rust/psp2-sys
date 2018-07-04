# `hello_rust_world`

An example app that will show a Hello World message using the
[debug screen](https://github.com/vitasdk/samples/tree/master/debugscreen) from
the [VitaSDK samples](https://github.com/vitasdk/samples).

## Compiling

Make sure [`cargo-make`](https://sagiegurari.github.io/cargo-make/) is installed,
and run:
```console
$ cargo make vpk
```

## Deploying

Transfer to the `ux0:` of a PS Vita running under
[Henkaku](https://github.com/henkaku/henkaku) or
[h-encore](https://github.com/TheOfficialFloW/h-encore) and install through
VitaShell. The resulting app is flagged as *safe* and should not trigger warnings
during installation.
