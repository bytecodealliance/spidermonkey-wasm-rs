extern crate link_cplusplus;

mod jsgc;
mod jsrealm;

#[cxx::bridge]
pub mod jsffi {
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[namespace = "JS"]
    enum OnNewGlobalHookOption {
        FireOnNewGlobalHook = 0,
        DontFireOnNewGlobalHook = 1,
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
        type RootedObject = crate::jsgc::Rooted<*const JSObject>;
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

        fn JS_Init() -> bool;
        fn getDefaultGlobalClass() -> UniquePtr<JSClass>;
        fn makeDefaultRealmOptions() -> *mut RealmOptions;

        unsafe fn JS_NewContext(max_bytes: u32, parent: *mut JSRuntime) -> *mut JSContext;
        unsafe fn JS_NewGlobalObject(
            context: *mut JSContext,
            klass: *const JSClass,
            principals: *mut JSPrincipals,
            hook: OnNewGlobalHookOption,
            realm_opts: &RealmOptions
        ) -> *mut JSObject;

        #[namespace = "JS"]
        unsafe fn EnterRealm(context: *mut JSContext, target: *mut JSObject) -> *mut Realm;

        #[namespace = "JS"]
        unsafe fn LeaveRealm(context: *mut JSContext, old_realm: *mut Realm);
    }
}
