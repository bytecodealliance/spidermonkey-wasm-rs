extern crate link_cplusplus;

pub mod jsgc;
pub mod jsrealm;
pub mod jsval;
pub use cxx::UniquePtr;

use cxx::{type_id, ExternType};

macro_rules! impl_extern_type {
    ($type:ty, $id:expr, $kind:ty) => {
        unsafe impl ExternType for $type {
            type Id = type_id!($id);
            type Kind = $kind;
        }
    };
}

impl_extern_type!(jsffi::JSAutoRealm, "JSAutoRealm", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootingContext, "RootingContext", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootKind, "JS::RootKind", cxx::kind::Trivial);
impl_extern_type!(jsffi::Value, "JS::Value", cxx::kind::Trivial);

impl_extern_type!(jsffi::RootedObject, "JS::RootedObject", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootedValue, "JS::RootedValue", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootedString, "JS::RootedString", cxx::kind::Opaque);
impl_extern_type!(jsffi::RootedScript, "JS::RootedScript", cxx::kind::Opaque);

impl_extern_type!(jsffi::HandleValue, "JS::HandleValue", cxx::kind::Trivial);
impl_extern_type!(jsffi::HandleObject, "JS::HandleObject", cxx::kind::Opaque);
impl_extern_type!(jsffi::HandleScript, "JS::HandleScript", cxx::kind::Trivial);

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

impl_extern_type!(jsffi::JSGCParamKey, "JSGCParamKey", cxx::kind::Trivial);
impl_extern_type!(jsffi::GCOptions, "JS::GCOptions", cxx::kind::Trivial);
impl_extern_type!(jsffi::GCReason, "JS::GCReason", cxx::kind::Trivial);

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
        type PersistentRootedObject;
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
        type HandleScript = crate::jsgc::Handle<*mut JSScript>;
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
        #[namespace = "JS"]
        unsafe fn DisableIncrementalGC(context: *mut JSContext);
        #[namespace = "JS"]
        unsafe fn PrepareForFullGC(context: *mut JSContext);
        #[namespace = "JS"]
        type GCOptions = crate::jsgc::JSGCOptions;
        #[namespace = "JS"]
        type GCReason = crate::jsgc::JSGCReason;
        #[namespace = "JS"]
        unsafe fn NonIncrementalGC(context: *mut JSContext, options: GCOptions, reason: GCReason);
        type JSGCParamKey = crate::jsgc::JSGCParamKey;
        unsafe fn JS_SetGCParameter(context: *mut JSContext, param_key: JSGCParamKey, value: u32);

        unsafe fn JS_GetRuntime(context: *mut JSContext) -> *mut JSRuntime;
        unsafe fn JS_NewContext(max_bytes: u32, parent: *mut JSRuntime) -> *mut JSContext;
        unsafe fn JS_DestroyContext(context: *mut JSContext);
        fn DefaultHeapMaxBytes() -> u32;

        fn JS_Init() -> bool;
        fn JS_ShutDown();

        fn MakeDefaultGlobalClass() -> UniquePtr<JSClass>;
        fn MakeDefaultRealmOptions() -> UniquePtr<RealmOptions>;
        unsafe fn MakeOwningCompileOptions(
            context: *mut JSContext,
            opts: &CompileOptionsParams,
        ) -> UniquePtr<OwningCompileOptions>;

        unsafe fn InitDefaultSelfHostedCode(context: *mut JSContext) -> bool;
        #[namespace = "js"]
        unsafe fn UseInternalJobQueues(context: *mut JSContext) -> bool;

        unsafe fn JS_NewPlainObject(context: *mut JSContext) -> *mut JSObject;
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

        unsafe fn MakeUtf8UnitSourceText(
            context: *mut JSContext,
            units: &str,
            length: usize,
            ownership: SourceOwnership,
        ) -> UniquePtr<Utf8UnitSourceText>;

        unsafe fn Utf8SourceEvaluate(
            context: *mut JSContext,
            compile_opts: &OwningCompileOptions,
            source: Pin<&mut Utf8UnitSourceText>,
            rval: MutableHandleValue,
        ) -> bool;

        unsafe fn Utf8SourceCompile(
            context: *mut JSContext,
            options: &OwningCompileOptions,
            source: Pin<&mut Utf8UnitSourceText>,
        ) -> *mut JSScript;

        unsafe fn JS_ExecuteScript(
            context: *mut JSContext,
            scriptArg: HandleScript,
            rval: MutableHandleValue,
        ) -> bool;

        fn MakeUninitPersistentRootedObject() -> UniquePtr<PersistentRootedObject>;
        unsafe fn InitPersistentRootedObject(
            root: Pin<&mut PersistentRootedObject>,
            context: *mut JSContext,
            initial: *mut JSObject,
        );
        fn initialized(self: &PersistentRootedObject) -> bool;

        unsafe fn Utf8IsCompilableUnit(context: *mut JSContext, global: HandleObject, source: &str) -> bool;
    }
}
