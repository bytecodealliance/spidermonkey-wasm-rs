mod integration {
    use spidermonkey_wasm_sys::{jsffi, jsgc, jsrealm};
    use std::ptr;

    #[test]
    fn eval() {
        let global_class = jsffi::MakeDefaultGlobalClass();
        assert!(jsffi::JS_Init());

        unsafe {
            let context = jsffi::JS_NewContext(32 * 32 * 1024, ptr::null_mut());
            assert!(!context.is_null());
            assert!(jsffi::InitDefaultSelfHostedCode(context));

            let realm_opts = jsffi::MakeDefaultRealmOptions();
            let global_object = jsgc::Rooted::new(
                context,
                jsffi::JS_NewGlobalObject(
                    context,
                    &*global_class,
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

            let mut undefined_value = jsffi::UndefinedValue();
            let rval = jsgc::MutableHandle {
                ptr: &mut undefined_value,
            };

            let script = "41 + 1";
            let mut source = jsffi::MakeUtf8UnitSourceText();
            assert!(jsffi::InitUtf8UnitSourceText(context, source.pin_mut(), &script, script.len(), jsffi::SourceOwnership::Borrowed));

            jsffi::Utf8SourceEvaluate(
                context,
                &owning_compile_options,
                source.pin_mut(),
                rval,
            );

            let result = undefined_value.toInt32();
            assert_eq!(result, 42);
        }
    }
}
