mod eval {
    use spidermonkey_wasm::{
        compilation_options::CompilationOptions, js, root, runtime::Runtime,
        utf8_source::Utf8Source, JSAutoRealm,
    };

    #[test]
    fn eval() {
        let runtime = Runtime::new().unwrap();
        let global_class = js::make_default_global_class();
        let context = runtime.cx();

        let realm_opts = js::make_default_realm_options();
        root!(with(context);
            let global_object = js::new_global_object(context, &global_class, &realm_opts);
        );

        let global_object_handle = global_object.handle();
        let _ar = JSAutoRealm::new(context, global_object_handle.get());

        root!(with(context);
            let mut return_value = js::undefined_value();
        );

        let return_value_handle = return_value.mut_handle();
        let mut script = Utf8Source::new(context, "41 + 1").unwrap();
        let compile_opts = CompilationOptions::new(context, 1, false, "eval.js".into()).unwrap();

        runtime
            .eval(&compile_opts, &mut script, return_value_handle)
            .unwrap();
        let result = return_value.get().to_int32();
        assert_eq!(result, 42);
    }
}
