fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS") == Ok("macos".to_string()) {
        println!("cargo:rustc-link-lib=framework=CoreHaptics");
    }

    if std::env::var("CARGO_CFG_TARGET_OS") == Ok("haiku".to_string()) {
        println!("cargo:rustc-link-lib=be");
        println!("cargo:rustc-link-lib=device");
        println!("cargo:rustc-link-lib=game");
        println!("cargo:rustc-link-lib=media");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
    }
}
