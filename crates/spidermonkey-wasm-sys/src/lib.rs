extern crate link_cplusplus;

pub mod jsgc;
pub mod jsrealm;
pub mod jsval;
pub mod jssourcetext;

#[cxx::bridge]
pub mod jsffi {

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[namespace = "JS"]
    enum OnNewGlobalHookOption {
        FireOnNewGlobalHook = 0,
        DontFireOnNewGlobalHook = 1,
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
        type JSPrincipals;
        type RootingContext = crate::jsgc::RootingContext;
        type Utf8UnitSourceText = crate::jssourcetext::SourceText<u8>;
        type U16SourceText = crate::jssourcetext::SourceText<u16>;

        #[namespace = "JS"]
        type SourceOwnership;
        #[namespace = "JS"]
        type Value = crate::jsval::Value;
        #[namespace = "JS"]
        type RootedObject = crate::jsgc::Rooted<*const JSObject>;
        #[namespace = "JS"]
        type HandleObject = crate::jsgc::Handle<*const JSObject>;
        #[namespace = "JS"]
        type MutableHandleObject = crate::jsgc::MutableHandle<*const JSObject>;
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

        fn JS_Init() -> bool;
        fn getDefaultGlobalClass() -> UniquePtr<JSClass>;
        fn makeDefaultRealmOptions() -> *mut RealmOptions;

        unsafe fn InitDefaultSelfHostedCode(context: *mut JSContext) -> bool;
        unsafe fn NewOwningCompileOptions(context: *mut JSContext, opts: &CompileOptionsParams) -> UniquePtr<OwningCompileOptions>;
        unsafe fn JS_NewContext(max_bytes: u32, parent: *mut JSRuntime) -> *mut JSContext;
        unsafe fn JS_NewGlobalObject(
            context: *mut JSContext,
            klass: *const JSClass,
            principals: *mut JSPrincipals,
            hook: OnNewGlobalHookOption,
            // TODO: verify this signature
            realm_opts: &RealmOptions
        ) -> *mut JSObject;

        #[namespace = "JS"]
        unsafe fn EnterRealm(context: *mut JSContext, target: *mut JSObject) -> *mut Realm;
        #[namespace = "JS"]
        unsafe fn LeaveRealm(context: *mut JSContext, old_realm: *mut Realm);

        #[namespace = "JS"]
        fn UndefinedValue() -> Value;
        fn toInt32(self: &Value) -> i32;

        unsafe fn Utf8SourceEvaluate(
            context: *mut JSContext,
            compile_opts: &OwningCompileOptions,
            source: Pin<&mut Utf8UnitSourceText>,
            rval: MutableHandleValue,
        ) -> bool;
    }
}
