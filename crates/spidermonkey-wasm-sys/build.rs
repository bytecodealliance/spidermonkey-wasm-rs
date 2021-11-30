use std::{env, path::{PathBuf, Path}};
use fs_extra::dir;
use walkdir::WalkDir;

static SPIDERMONKEY_BUILD_DIR: &str = "spidermonkey-wasm-build";

const WHITELIST_TYPES: &'static [&'static str] = &["JS.*", "js::.*", "mozilla::.*"];

const WHITELIST_VARS: &'static [&'static str] = &[
    "JS::NullHandleValue",
    "JS::TrueHandleValue",
    "JS::UndefinedHandleValue",
    "JSCLASS_.*",
    "JSFUN_.*",
    "JSITER_.*",
    "JSPROP_.*",
    "JSREG_.*",
    "JS_.*",
    "js::Proxy.*",
    "exports::*",
];

const WHITELIST_FUNCTIONS: &'static [&'static str] = &[
    "JS_NewContext",
    "ExceptionStackOrNull",
    "JS::.*",
    "js::.*",
    "exports::.*",
    "JS_.*",
    ".*_TO_JSID",
    "JS_DeprecatedStringHasLatin1Chars",
];

const OPAQUE_TYPES: &'static [&'static str] = &[
    "JS::Auto.*Impl",
    "JS::StackGCVector.*",
    "JS::PersistentRooted.*",
    "JS::detail::CallArgsBase.*",
    "js::detail::UniqueSelector.*",
    "mozilla::BufferList",
    "mozilla::Maybe.*",
    "mozilla::UniquePtr.*",
    "mozilla::Variant",
    "mozilla::Hash.*",
    "mozilla::detail::Hash.*",
    "RefPtr_Proxy.*",
];

const BLACKLIST_TYPES: &'static [&'static str] = &[
    "JS::HandleVector",
    "JS::MutableHandleVector",
    "JS::Rooted.*Vector",
    "JS::RootedValueArray",
];

struct WasiSdk {
    clang: PathBuf,
    sysroot: PathBuf,
    ar: PathBuf
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").map(PathBuf::from).expect("could not find OUT_DIR");
    let source_dir = PathBuf::from(SPIDERMONKEY_BUILD_DIR);
    let source_include_dir = source_dir.join("include");
    let source_lib_dir = source_dir.join("lib");

    let out_include_dir = out_dir.join("include");
    let out_lib_dir = out_dir.join("lib");

    let sdk = WasiSdk {
        clang: PathBuf::from("/opt/wasi-sdk/bin/clang++"),
        sysroot: PathBuf::from("--sysroot=/opt/wasi-sdk/share/wasi-sysroot"),
        ar: PathBuf::from("/opt/wasi-sdk/bin/ar"),
    };

    if !source_dir.exists() {
        panic!("SpiderMonkey build directory not found. Try updating git submodules via git submodule update --recursive --init");
    }

    if !source_include_dir.exists() || !source_lib_dir.exists() {
        panic!("SpiderMonkey build artifacts not found.");
    }

    if !out_include_dir.exists() {
        let copy_options = dir::CopyOptions::new();
        dir::copy(source_include_dir, &out_dir, &copy_options).expect("Could not copy header files to OUT directory");
    }

    if !out_lib_dir.exists() {
        let copy_options = dir::CopyOptions::new();
        dir::copy(source_lib_dir, &out_dir, &copy_options).expect("Could not copy lib directory to OUT directory");
    }

    println!(
        "cargo:rustc-link-search={}",
        out_lib_dir.display()
    );
    println!("cargo:rustc-link-search=native=/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi");
    println!("cargo:rustc-link-search=native=/opt/wasi-sdk/lib/clang/11.0.0/lib/wasi");

    compile_exports(&sdk, &out_lib_dir, &out_include_dir);
    println!("cargo:rustc-link-lib=static=jsrust");
    println!("cargo:rustc-link-lib=static=js_static");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=clang_rt.builtins-wasm32");

    generate_bindings(&out_dir, &sdk);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/exports.h");
    println!("cargo:rerun-if-changed=src/exports.cpp");
}

fn compile_exports(wasi_sdk: &WasiSdk, lib_dir: impl AsRef<Path>, include_dir: impl AsRef<Path>) {
    env::set_var("CXX", &wasi_sdk.clang);
    env::set_var("CXX_wasm32_wasi", &wasi_sdk.clang);
    env::set_var("CXXFLAGS", &wasi_sdk.sysroot);
    env::set_var("CXXFLAGS_wasm32_wasi", &wasi_sdk.sysroot);
    env::set_var("AR", &wasi_sdk.ar);
    env::set_var("AR_wasm32_wasi", &wasi_sdk.ar);

    let mut builder = cc::Build::new();

    builder
        .cpp(true)
        .cpp_link_stdlib("c++")
        .file("src/exports.cpp")
        .include(include_dir)
        .flag("-DRUST_BINDGEN")
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

    for entry in WalkDir::new(lib_dir).sort_by_file_name().into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().unwrap() == "o" {
            builder.object(entry_path);
        }
    }

    builder
        .opt_level(2)
        .compile("spidermonkey-wasm");
}

fn generate_bindings(dir: &Path, wasi_sdk: &WasiSdk) {
    env::set_var("CLANG_PATH", &wasi_sdk.clang);

    let mut config = bindgen::CodegenConfig::all();
    config &= !bindgen::CodegenConfig::CONSTRUCTORS;
    config &= !bindgen::CodegenConfig::DESTRUCTORS;
    config &= !bindgen::CodegenConfig::METHODS;

    let bindings_file = dir.join("bindings.rs");

    let mut builder = bindgen::builder()
        .rust_target(bindgen::RustTarget::Stable_1_47)
        .header("src/exports.h")
        .rustified_enum(".*")
        .size_t_is_usize(true)
        .enable_cxx_namespaces()
        .with_codegen_config(config)
        .clang_arg("-I")
        .clang_arg(dir.join("include").to_str().expect("UTF-8"))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=gnu++17")
        .clang_arg("-Wall")
        .clang_arg("-Werror")
        .clang_arg("-Wno-invalid-offsetof")
        .clang_arg("-Qunused-arguments")
        .clang_arg("-Wno-unused-private-field")
        .clang_arg("-fno-aligned-new")
        .clang_arg("-mthread-model")
        .clang_arg("single")
        .clang_arg("-fPIC")
        .clang_arg("-fno-rtti")
        .clang_arg("-fno-exceptions")
        .clang_arg("-fno-math-errno")
        .clang_arg("-pipe")
        .clang_arg("-fno-omit-frame-pointer")
        .clang_arg("-funwind-tables")
        .clang_args(&[wasi_sdk.sysroot.to_str().unwrap(), "--target=wasm32-wasi"]);

    for ty in WHITELIST_TYPES {
        builder = builder.allowlist_type(ty);
    }

    for var in WHITELIST_VARS {
        builder = builder.allowlist_var(var);
    }

    for func in WHITELIST_FUNCTIONS {
        builder = builder.allowlist_function(func);
    }

    for ty in OPAQUE_TYPES {
        builder = builder.opaque_type(ty);
    }

    for ty in BLACKLIST_TYPES {
        builder = builder.blocklist_type(ty);
    }

    builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("could not generate bindings")
        .write_to_file(&bindings_file)
        .expect("could not write bindings file");
}
