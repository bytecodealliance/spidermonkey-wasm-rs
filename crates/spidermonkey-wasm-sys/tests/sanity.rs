mod sanity {
    use std::{ptr, mem};
    use spidermonkey_wasm_sys::api;

    #[test]
    fn eval() {
        let class_ops: api::JSClassOps = api::JSClassOps {
            addProperty: None,
            delProperty: None,
            enumerate: None,
            newEnumerate: None,
            resolve: None,
            mayResolve: None,
            finalize: None,
            call: None,
            hasInstance: None,
            construct: None,
            trace: Some(api::JS_GlobalObjectTraceHook),
        };

        let class: api::JSClass = api::JSClass {
            name: "global\0" as *const str as *const i8,
            flags: api::JSCLASS_GLOBAL_FLAGS,
            cOps: &class_ops,
            spec: ptr::null(),
            ext: ptr::null(),
            oOps: ptr::null(),
        };

        unsafe {
            assert!(api::exports::JS_Init());
            let context = api::JS_NewContext(32 * 32 * 1024, ptr::null_mut());
            assert!(!context.is_null());
            // TODO: Find a way to create a SelfHostedCache
            // which is the second parameter of this function
            assert!(api::JS::InitSelfHostedCode(context, [0, 0], None)); 

            // TODO: Rooted?
            let realm_opts = api::exports::JS_NewRealmOptions();
            let global = api::JS_NewGlobalObject(
                context,
                &class,
                ptr::null_mut(),
                api::JS::OnNewGlobalHookOption::FireOnNewGlobalHook,
                realm_opts
            );
            
            let realm = api::JS::EnterRealm(context, global);
            let compile_opts = api::exports::JS_NewOwningCompileOptions(context);

            let mut val: api::JS::Value = mem::zeroed();
            let mut handle: api::JS::MutableHandleValue = mem::zeroed();
            handle.ptr = &mut val;

            let script = "42".as_bytes();
            let mut source = api::JS::SourceText {
                units_: script.as_ptr() as *const _,
                length_: script.len() as u32,
                ownsUnits_: false,
                _phantom_0: std::marker::PhantomData,
            };
            assert!(api::JS::Evaluate2(
                        context,
                        &(*compile_opts)._base,
                        &mut source,
                        handle
                    )
                );

            assert!(api::exports::JS_ValueToInt32(&val) == 42);

            // TODO:
            // - LeaveRealm, Destroy context, shut down JS
        }
    }
}
