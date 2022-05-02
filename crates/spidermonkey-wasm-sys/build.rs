use cxx_build::bridge as cxxbridge;
use fs_extra::dir;
use std::{
    env,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

static SPIDERMONKEY_BUILD_DIR: &str = "spidermonkey-wasm-build";

fn main() {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("could not find OUT_DIR");

    let profile = derive_profile();

    let source_dir = PathBuf::from(SPIDERMONKEY_BUILD_DIR).join(profile);
    let source_include_dir = source_dir.join("include");
    let source_lib_dir = source_dir.join("lib");

    let out_include_dir = out_dir.join("include");
    let out_lib_dir = out_dir.join("lib");

    if !source_dir.exists() {
        panic!("SpiderMonkey build directory not found. Try updating git submodules via git submodule update --recursive --init");
    }

    if !source_include_dir.exists() || !source_lib_dir.exists() {
        panic!("SpiderMonkey build artifacts not found.");
    }

    if !out_include_dir.exists() {
        let copy_options = dir::CopyOptions::new();
        dir::copy(source_include_dir, &out_dir, &copy_options)
            .expect("Could not copy header files to OUT directory");
    }

    if !out_lib_dir.exists() {
        let copy_options = dir::CopyOptions::new();
        dir::copy(source_lib_dir, &out_dir, &copy_options)
            .expect("Could not copy lib directory to OUT directory");
    }

    println!("cargo:rustc-link-search={}", out_lib_dir.display());

    let libclang_path = env::var("LIBCLANG_PATH").expect("LIBCLANG_PATH to be defined");
    let libclang_rt_path = env::var("LIBCLANG_RT_PATH").expect("LIBCLANG_RT_PATH to be defined");

    println!("cargo:rustc-link-search=native={}", libclang_path);
    println!("cargo:rustc-link-search=native={}", libclang_rt_path);

    println!("cargo:rustc-link-lib=static=jsrust");
    println!("cargo:rustc-link-lib=static=js_static");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=clang_rt.builtins-wasm32");
    bridge(&out_lib_dir, &out_include_dir, &profile);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/api.h");
    println!("cargo:rerun-if-changed=src/api.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn bridge(lib_dir: impl AsRef<Path>, include_dir: impl AsRef<Path>, profile: &str) {
    let mut builder = cxxbridge("src/lib.rs");

    if profile == "debug-build" {
        builder.define("DEBUG", None);
    }

    builder
        .cpp(true)
        .cpp_link_stdlib("c++")
        .file("src/api.cpp")
        .include(include_dir)
        .include("src")
        .target("wasm32-wasi")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Werror")
        .flag_if_supported("-Qunused-arguments")
        .flag_if_supported("-fno-sized-deallocation")
        .flag_if_supported("-fno-exceptions")
        .flag_if_supported("-fno-aligned-new")
        .flag_if_supported("-mthread-model")
        .flag_if_supported("single")
        .flag_if_supported("-fPIC")
        .flag_if_supported("-fno-rtti")
        .flag_if_supported("-fno-math-errno")
        .flag_if_supported("-pipe")
        .flag_if_supported("-fno-omit-frame-pointer")
        .flag_if_supported("-funwind-tables")
        .flag_if_supported("-Wno-invalid-offsetof")
        .flag_if_supported("-std=gnu++17");

    for entry in WalkDir::new(lib_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().unwrap() == "o" {
            builder.object(entry_path);
        }
    }

    builder.opt_level(2).compile("spidermonkey-wasm");
}

fn derive_profile() -> &'static str {
    let mut profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());

    if cfg!(feature = "moz_debug") {
        profile = "debug".into();
    }

    match profile.as_str() {
        "debug" => "debug-build",
        "release" => "release-build",
        _ => panic!("Unsupported profile: {}", profile),
    }
}
