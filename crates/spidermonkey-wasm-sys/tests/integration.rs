mod integration {
    use spidermonkey_wasm_sys::jsffi::JS_NewPlainObject;
    use spidermonkey_wasm_sys::{jsclass, jsffi, jsgc, jsrealm};
    use std::marker::PhantomData;
    use std::ptr;

    fn init_engine() -> *mut jsffi::JSContext {
        assert!(jsffi::JS_Init());
        unsafe {
            let context = jsffi::JS_NewContext(jsffi::DefaultHeapMaxBytes(), ptr::null_mut());
            assert!(!context.is_null());
            context
        }
    }

    fn shutdown_engine(context: *mut jsffi::JSContext) {
        unsafe {
            jsffi::JS_DestroyContext(context);
        }
        jsffi::JS_ShutDown();
    }

    #[test]
    fn eval() {
        let global_class: jsclass::JSClass = jsclass::JSClass {
            name: "global\0".as_ptr() as *const i8,
            flags: jsffi::js_class_global_flags(),
            c_ops: jsffi::default_global_class_ops(),
            spec: std::ptr::null(),
            ext: std::ptr::null(),
            o_ops: std::ptr::null(),
        };

        let context = init_engine();

        unsafe {
            assert!(jsffi::InitDefaultSelfHostedCode(context));

            let realm_opts = jsffi::make_default_realm_options();
            let mut global_object = jsgc::Rooted::default();
            global_object.init(
                context,
                jsffi::JS_NewGlobalObject(
                    context,
                    &global_class,
                    ptr::null_mut(),
                    jsffi::OnNewGlobalHookOption::FireOnNewGlobalHook,
                    &realm_opts,
                ),
            );

            let _ar = jsrealm::JSAutoRealm::new(context, global_object.ptr);
            let owning_compile_options = jsffi::MakeOwningCompileOptions(
                context,
                &jsffi::CompileOptionsParams {
                    force_full_parse: false,
                    lineno: 1,
                    file: "eval.js".into(),
                },
            );

            let mut undefined_value = jsffi::undefined_value();
            let rval = jsgc::MutableHandle {
                ptr: &mut undefined_value,
                _marker: PhantomData,
            };

            let script = "41 + 1";
            let mut source = jsffi::MakeUtf8UnitSourceText(
                context,
                &script,
                script.len(),
                jsffi::SourceOwnership::Borrowed,
            );

            jsffi::Utf8SourceEvaluate(context, &owning_compile_options, source.pin_mut(), rval);

            let result = undefined_value.to_int32();
            assert_eq!(result, 42);
            global_object.remove_from_root_stack();

            let mut persistent = jsffi::MakeUninitPersistentRootedObject();
            assert!(!persistent.initialized());
            jsffi::InitPersistentRootedObject(
                persistent.pin_mut(),
                context,
                JS_NewPlainObject(context),
            );
            assert!(persistent.initialized());
        }

        shutdown_engine(context);
    }
}
