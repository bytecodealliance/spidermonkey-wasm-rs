mod realm_opts {
    use spidermonkey_wasm::{js, runtime::Runtime, WeakRefSpecifier};

    #[test]
    fn set() {
        let _runtime = Runtime::new().unwrap();
        let mut realm_opts = js::make_default_realm_options();
        let mut realm_creation_opts = realm_opts.pin_mut().creation_options();

        realm_creation_opts = realm_creation_opts
            .set_streams_enabled(true)
            .set_readable_byte_streams_enabled(true)
            .set_byob_stream_readers_enabled(true)
            .set_readable_stream_pipe_to_enabled(true)
            .set_writable_streams_enabled(true)
            .set_iterator_helpers_enabled(true)
            .set_weak_refs_enabled(WeakRefSpecifier::EnabledWithCleanupSome);

        assert!(realm_creation_opts.get_writable_streams_enabled());
        assert!(realm_creation_opts.get_readable_byte_streams_enabled());
        assert!(realm_creation_opts.get_byob_stream_readers_enabled());
        assert!(realm_creation_opts.get_readable_stream_pipe_to_enabled());
        assert!(realm_creation_opts.get_writable_streams_enabled());
        assert!(realm_creation_opts.get_iterator_helpers_enabled());
        assert_eq!(
            realm_creation_opts.get_weak_refs_enabled(),
            WeakRefSpecifier::EnabledWithCleanupSome
        );
    }

    #[test]
    fn unset() {
        let _runtime = Runtime::new().unwrap();
        let mut realm_opts = js::make_default_realm_options();
        let mut realm_creation_opts = realm_opts.pin_mut().creation_options();

        realm_creation_opts = realm_creation_opts
            .set_streams_enabled(false)
            .set_readable_byte_streams_enabled(false)
            .set_byob_stream_readers_enabled(false)
            .set_readable_stream_pipe_to_enabled(false)
            .set_writable_streams_enabled(false)
            .set_iterator_helpers_enabled(false);

        assert!(!realm_creation_opts.get_writable_streams_enabled());
        assert!(!realm_creation_opts.get_readable_byte_streams_enabled());
        assert!(!realm_creation_opts.get_byob_stream_readers_enabled());
        assert!(!realm_creation_opts.get_readable_stream_pipe_to_enabled());
        assert!(!realm_creation_opts.get_writable_streams_enabled());
        assert!(!realm_creation_opts.get_iterator_helpers_enabled());
    }
}
