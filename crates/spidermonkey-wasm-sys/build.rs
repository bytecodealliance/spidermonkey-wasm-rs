use std::{env, fs::{create_dir_all, copy}, path::{PathBuf, Path}, process::Command};
use fs_extra::{dir, remove_items};

static LIB_DEPENDENCIES: &[&str] = &[
    "js/src/build/libjs_static.a",
    "wasm32-wasi/release/libjsrust.a"
];

static OBJ_DEPENDENCIES: &[&str] = &[
    "mozglue/misc/AutoProfilerLabel.o",
    "mozglue/misc/ConditionVariable_noop.o",
    "mozglue/misc/Decimal.o",
    "mozglue/misc/MmapFaultHandler.o",
    "mozglue/Misc/Mutex_noop.o",
    "mozglue/misc/Printf.o",
    "mozglue/misc/StackWalk.o",
    "mozglue/misc/TimeStamp.o",
    "mozglue/misc/TimeStamp_posix.o",
    "memory/build/Unified_cpp_memory_build0.o",
    "memory/mozalloc/Unified_cpp_memory_mozalloc0.o",
    "mfbt/Unified_cpp_mfbt0.o",
    "mfbt/Unified_cpp_mfbt1.o",
    "mozglue/misc/Uptime.o",
    "mfbt/lz4.o",
    "mfbt/lz4frame.o",
    "mfbt/lz4hc.o",
    "mfbt/xxhash.o",
];


static GECKO_DEV_BASE_PATH: &str = "gecko-dev";
static GECKO_DEV_BUIL_DIR: &str = "obj-wasm32-unknown-wasi";
static MOZ_CONFIG: &str = "mozconfig";
const JS_CONF_DEFS: &str = "js/src/js-confdefs.h";

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

fn main() {
    let out_dir = env::var_os("OUT_DIR").map(PathBuf::from).expect("could not find OUT_DIR");
    let root = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).expect("could not find root dir");
    let gecko_dev_obj_dir = PathBuf::from(GECKO_DEV_BASE_PATH).join(GECKO_DEV_BUIL_DIR);
    let mozconfig_path = root.join(MOZ_CONFIG);

    let include_dir = out_dir.join("include");
    let lib_dir = out_dir.join("lib");


    let sysroot = "--sysroot=/opt/wasi-sdk/share/wasi-sysroot";
    let clang = "/opt/wasi-sdk/bin/clang++";
    let ar = "/opt/wasi-sdk/bin/ar";

    if !gecko_dev_obj_dir.exists() {
        build_artifacts(&mozconfig_path);

        let mut items = vec![];
        if lib_dir.exists() {
            items.push(&lib_dir);
        }
        if include_dir.exists() {
            items.push(&include_dir);
        }

        remove_items(&items).unwrap();
    }

    if !lib_dir.exists() || !include_dir.exists() {
        // Copy header files from gecko-dev
        let  copy_options = dir::CopyOptions::new();
        dir::copy(gecko_dev_obj_dir.join("dist").join("include"), &out_dir, &copy_options).unwrap();
        copy(gecko_dev_obj_dir.join(JS_CONF_DEFS), &include_dir.join("js-confdefs.h")).unwrap();

        // Copy object dependencies and static libraries from gecko-dev
        create_dir_all(&lib_dir).unwrap();
        for obj in OBJ_DEPENDENCIES {
            let obj_path = gecko_dev_obj_dir.join(obj);
            let destination = lib_dir.join(obj_path.file_name().unwrap());
            copy(obj_path, &destination).unwrap();
        }

        for lib in LIB_DEPENDENCIES {
            let lib_path = gecko_dev_obj_dir.join(lib);
            copy(&lib_path, lib_dir.join(&lib_path.file_name().unwrap())).unwrap();
        }
    }


    println!(
        "cargo:rustc-link-search={}",
        lib_dir.display()
    );
    println!("cargo:rustc-link-search=native=/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi");
    println!("cargo:rustc-link-search=native=/opt/wasi-sdk/lib/clang/11.0.0/lib/wasi");

    compile_exports(clang, ar, sysroot, lib_dir.to_str().unwrap(), include_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=jsrust");
    println!("cargo:rustc-link-lib=static=js_static");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=clang_rt.builtins-wasm32");

    generate_bindings(&out_dir);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/exports.h");
    println!("cargo:rerun-if-changed=src/exports.cpp");
}

fn build_artifacts(mozconfig_path: impl AsRef<Path>) {
    // TODO: Find a way to avoid running this command
    // if there's no need to.
    Command::new("./mach")
        .arg("bootstrap")
        .arg("--application-choice=js")
        .current_dir(GECKO_DEV_BASE_PATH)
        .status()
        .expect("Couldn't run ./mach bootstrap");

    Command::new("./mach")
        .env("MOZCONFIG", mozconfig_path.as_ref().to_str().unwrap())
        .arg("build")
        .current_dir(GECKO_DEV_BASE_PATH)
        .status()
        .expect("Couldn't run ./mach build");
}

fn compile_exports(clang: &str, ar: &str, sysroot: &str, lib_dir: impl AsRef<Path>, include_dir: impl AsRef<Path>) {
    env::set_var("CXX", clang);
    env::set_var("CXX_wasm32_wasi", clang);
    env::set_var("CXXFLAGS", sysroot);
    env::set_var("CXXFLAGS_wasm32_wasi", sysroot);
    env::set_var("AR", ar);
    env::set_var("AR_wasm32_wasi", ar);

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

    for o in OBJ_DEPENDENCIES {
        let path = PathBuf::from(o);
        let file_name = path.file_name().unwrap();
        builder.object(lib_dir.as_ref().join(file_name));
    }

    builder
        .opt_level(2)
        .compile("jsexports");
}

fn generate_bindings(dir: &Path) {

    env::set_var("CLANG_PATH", "/opt/wasi-sdk/bin/clang++");

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
        .clang_args(&["--sysroot=/opt/wasi-sdk/share/wasi-sysroot", "--target=wasm32-wasi"]);

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
