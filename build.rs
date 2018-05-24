use std::path::PathBuf;
use std::env;

fn main() {
    let sdk = PathBuf::from(env::var("VITASDK").expect("$VITASDK is not set !"));
    let lib = sdk.join("arm-vita-eabi").join("lib");
    println!("cargo:rustc-link-search={}", lib.to_str().unwrap());
}
