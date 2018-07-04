use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(sdk) = env::var("VITASDK").map(PathBuf::from) {
        let lib = sdk.join("arm-vita-eabi").join("lib");
        println!("cargo:rustc-link-search={}", lib.to_str().unwrap());
    } else {
        println!("cargo:warning=$VITASDK not set!");
    }
}
