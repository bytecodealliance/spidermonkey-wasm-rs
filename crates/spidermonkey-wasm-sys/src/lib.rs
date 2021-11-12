pub use js_bindings::root as api;

pub mod js_bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
