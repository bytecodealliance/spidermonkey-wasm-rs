mod compile {
    use spidermonkey_wasm::{
        compilation_options::CompilationOptions, jsapi, root, runtime::Runtime,
        utf8_source::Utf8Source,
    };
    use std::ptr;

    #[test]
    fn compile() {
        let runtime = Runtime::new().unwrap();
        let global_class = jsapi::MakeDefaultGlobalClass();
        let context = runtime.cx();

        unsafe {
            let realm_opts = jsapi::MakeDefaultRealmOptions();
            root!(with(context);
                let global_object = jsapi::JS_NewGlobalObject(runtime.cx(), &*global_class, ptr::null_mut(), jsapi::OnNewGlobalHookOption::FireOnNewGlobalHook, &realm_opts);
            );

            let global_object_handle = global_object.handle();
            let _ar = jsapi::jsrealm::JSAutoRealm::new(context, global_object_handle.get());

            root!(with(context);
                let mut return_value = jsapi::UndefinedValue();
            );

            let return_value_handle = return_value.mut_handle();
            let mut script = Utf8Source::new(context, "41 + 1").unwrap();
            let compile_opts =
                CompilationOptions::new(context, 1, false, "eval.js".into()).unwrap();

            root!(with(context);
                let js_script = runtime.compile(&compile_opts, &mut script).unwrap();
            );
            runtime
                .execute(js_script.handle(), return_value_handle)
                .unwrap();
            let result = return_value.get().toInt32();
            assert_eq!(result, 42);
        }
    }

    #[test]
    fn compile_fail() {
        let runtime = Runtime::new().unwrap();
        let global_class = jsapi::MakeDefaultGlobalClass();
        let context = runtime.cx();

        unsafe {
            let realm_opts = jsapi::MakeDefaultRealmOptions();
            root!(with(context);
                let global_object = jsapi::JS_NewGlobalObject(runtime.cx(), &*global_class, ptr::null_mut(), jsapi::OnNewGlobalHookOption::FireOnNewGlobalHook, &realm_opts);
            );

            let global_object_handle = global_object.handle();
            let _ar = jsapi::jsrealm::JSAutoRealm::new(context, global_object_handle.get());

            let mut script = Utf8Source::new(context, "invalid syntax").unwrap();
            let compile_opts =
                CompilationOptions::new(context, 1, false, "eval.js".into()).unwrap();

            assert!(runtime.compile(&compile_opts, &mut script).is_err());
        }
    }
}
