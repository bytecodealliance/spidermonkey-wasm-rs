mod context_opts {
    use spidermonkey_wasm::{js, runtime::Runtime};

    #[test]
    fn set() {
        let runtime = Runtime::new().unwrap();
        let context = runtime.cx();
        let mut opts_ref = js::context_options_ref(context);

        opts_ref = opts_ref
            .set_private_class_fields(true)
            .set_class_static_blocks(true)
            .set_private_class_methods(true)
            .set_ergonomic_brand_checks(true);

        assert!(opts_ref.private_class_fields());
        assert!(opts_ref.private_class_methods());
        assert!(opts_ref.class_static_blocks());
    }

    #[test]
    fn unset() {
        let runtime = Runtime::new().unwrap();
        let context = runtime.cx();
        let mut opts_ref = js::context_options_ref(context);

        opts_ref = opts_ref
            .set_private_class_fields(false)
            .set_class_static_blocks(false)
            .set_private_class_methods(false)
            .set_ergonomic_brand_checks(false);

        assert!(!opts_ref.private_class_fields());
        assert!(!opts_ref.private_class_methods());
        assert!(!opts_ref.class_static_blocks());
    }
}
