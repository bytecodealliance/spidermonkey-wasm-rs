extern crate link_cplusplus;

pub mod jsgc;
pub mod jsrealm;
pub mod jsval;

use cxx::{type_id, ExternType};

macro_rules! impl_extern_type {
    ($type:ty, $id:expr, $kind:ty) => {
        unsafe impl ExternType for $type {
            type Id = type_id!($id);
            type Kind = $kind;
        }
    };
}

impl_extern_type!(jsffi::RootingContext, "RootingContext", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootKind, "JS::RootKind", cxx::kind::Trivial);
impl_extern_type!(jsffi::Value, "JS::Value", cxx::kind::Trivial);

impl_extern_type!(jsffi::RootedObject, "JS::RootedObject", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootedValue, "JS::RootedValue", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootedString, "JS::RootedString", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootedScript, "JS::RootedScript", cxx::kind::Opaque);

impl_extern_type!(jsffi::HandleValue, "JS::HandleValue", cxx::kind::Trivial);
impl_extern_type!(jsffi::HandleObject, "JS::HandleObject", cxx::kind::Opaque);

impl_extern_type!(
    jsffi::MutableHandleObject,
    "JS::MutableHandleObject",
    cxx::kind::Opaque
);
impl_extern_type!(
    jsffi::MutableHandleValue,
    "JS::MutableHandleValue",
    cxx::kind::Trivial
);

#[cxx::bridge]
pub mod jsffi {

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[namespace = "JS"]
    enum OnNewGlobalHookOption {
        FireOnNewGlobalHook = 0,
        DontFireOnNewGlobalHook = 1,
    }

    #[repr(u32)]
    #[namespace = "JS"]
    enum SourceOwnership {
        Borrowed = 0,
        TakeOwnership = 1,
    }

    struct CompileOptionsParams {
        force_full_parse: bool,
        file: String,
        lineno: usize,
    }

    unsafe extern "C++" {
        include!("api.h");

        type JSAutoRealm = crate::jsrealm::JSAutoRealm;
        type JSObject;
        type JSRuntime;
        type JSContext;
        type JSClass;
        type JSScript;
        type JSPrincipals;
        type RootingContext = crate::jsgc::RootingContext;
        type Utf8UnitSourceText;
        type JSString;

        #[namespace = "JS"]
        type SourceOwnership;
        #[namespace = "JS"]
        type Value = crate::jsval::Value;
        #[namespace = "JS"]
        type RootedObject = crate::jsgc::Rooted<*mut JSObject>;
        #[namespace = "JS"]
        type RootedValue = crate::jsgc::Rooted<Value>;
        #[namespace = "JS"]
        type RootedString = crate::jsgc::Rooted<*mut JSString>;
        #[namespace = "JS"]
        type RootedScript = crate::jsgc::Rooted<*mut JSScript>;
        #[namespace = "JS"]
        type HandleObject = crate::jsgc::Handle<*mut JSObject>;
        #[namespace = "JS"]
        type HandleValue = crate::jsgc::Handle<Value>;
        #[namespace = "JS"]
        type MutableHandleObject = crate::jsgc::MutableHandle<*mut JSObject>;
        #[namespace = "JS"]
        type MutableHandleValue = crate::jsgc::MutableHandle<Value>;
        #[namespace = "JS"]
        type RootKind = crate::jsgc::RootKind;
        #[namespace = "JS"]
        type AutoRooterListHeads;
        #[namespace = "js"]
        type GeckoProfilerThread;
        #[namespace = "JS"]
        type Realm;
        #[namespace = "JS"]
        type Zone;
        #[namespace = "JS"]
        type RealmOptions;
        #[namespace = "JS"]
        type OnNewGlobalHookOption;
        #[namespace = "JS"]
        type OwningCompileOptions;
        #[namespace = "JS"]
        type ReadOnlyCompileOptions;

        unsafe fn JS_NewContext(max_bytes: u32, parent: *mut JSRuntime) -> *mut JSContext;
        fn JS_Init() -> bool;

        fn MakeDefaultGlobalClass() -> UniquePtr<JSClass>;
        fn MakeDefaultRealmOptions() -> UniquePtr<RealmOptions>;
        unsafe fn MakeOwningCompileOptions(
            context: *mut JSContext,
            opts: &CompileOptionsParams,
        ) -> UniquePtr<OwningCompileOptions>;

        unsafe fn InitDefaultSelfHostedCode(context: *mut JSContext) -> bool;

        unsafe fn JS_NewGlobalObject(
            context: *mut JSContext,
            klass: *const JSClass,
            principals: *mut JSPrincipals,
            hook: OnNewGlobalHookOption,
            realm_opts: &RealmOptions,
        ) -> *mut JSObject;

        #[namespace = "JS"]
        unsafe fn EnterRealm(context: *mut JSContext, target: *mut JSObject) -> *mut Realm;
        #[namespace = "JS"]
        unsafe fn LeaveRealm(context: *mut JSContext, old_realm: *mut Realm);

        #[namespace = "JS"]
        fn UndefinedValue() -> Value;
        fn toInt32(self: &Value) -> i32;

        unsafe fn MakeUtf8UnitSourceText() -> UniquePtr<Utf8UnitSourceText>;

        unsafe fn InitUtf8UnitSourceText(
            context: *mut JSContext,
            src: Pin<&mut Utf8UnitSourceText>,
            units: &str,
            length: usize,
            ownership: SourceOwnership,
        ) -> bool;

        unsafe fn Utf8SourceEvaluate(
            context: *mut JSContext,
            compile_opts: &OwningCompileOptions,
            source: Pin<&mut Utf8UnitSourceText>,
            rval: MutableHandleValue,
        ) -> bool;
    }
}
