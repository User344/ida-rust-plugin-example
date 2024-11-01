use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let ida_sdk_path = PathBuf::from(
        std::env::var("IDA_SDK_DIR").expect("IDA_SDK_DIR environment variable is not set!"),
    );

    let ida_sdk_include_path = ida_sdk_path.join("include");
    let ida_sdk_lib_path = ida_sdk_path.join("lib").join("x64_win_vc_64"); // <= hardcoded!

    let src_path = std::path::PathBuf::from("src");
    let src_ida_path = src_path.join("ida");

    println!("cargo:rustc-link-lib=static=ida");
    println!(
        "cargo:rustc-link-search=native={}",
        ida_sdk_lib_path.to_str().unwrap()
    );

    println!("cargo:rerun-if-changed=src/ida/bindgen.rs");
    autocxx_build::Builder::new("src/ida/bindgen.rs", &[&ida_sdk_include_path])
        .extra_clang_args(&["-D__NT__", "-D__EA64__", "-D__X64__"])
        .build()?
        .define("__NT__", None)
        .define("__EA64__", None)
        .define("__X64__", None)
        .compile("bindgen");

    println!("cargo:rerun-if-changed=src/ida/bridge.cpp");
    println!("cargo:rerun-if-changed=src/ida/bridge.hpp");
    println!("cargo:rerun-if-changed=src/ida/bridge.rs");
    autocxx_build::Builder::new("src/ida/bridge.rs", &[&ida_sdk_include_path, &src_ida_path])
        .extra_clang_args(&["-D__NT__", "-D__EA64__", "-D__X64__"])
        .build()?
        .cpp(true)
        .define("__NT__", None)
        .define("__EA64__", None)
        .define("__X64__", None)
        .file("src/ida/bridge.cpp")
        .compile("bridge");

    Ok(())
}
