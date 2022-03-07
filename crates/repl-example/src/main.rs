use spidermonkey_wasm::{
    compilation_options::CompilationOptions,
    handle::{HandleObject, HandleString},
    jsapi, root,
    runtime::Runtime,
    utf8_source::Utf8Source,
};
use std::ptr;

fn main() {
    let runtime = Runtime::new().unwrap();
    let global_class = jsapi::MakeDefaultGlobalClass();
    let realm_opts = jsapi::MakeDefaultRealmOptions();
    let context = runtime.cx();

    root!(with(context);
        let global_object = unsafe { jsapi::JS_NewGlobalObject(context, &*global_class, ptr::null_mut(), jsapi::OnNewGlobalHookOption::FireOnNewGlobalHook, &realm_opts) };
    );

    let global_object_handle = global_object.handle();
    let _ar = jsapi::jsrealm::JSAutoRealm::new(context, global_object_handle.get());
}

fn do_loop(runtime: &Runtime, global: HandleObject) {
    let mut lineno = 1;
    let mut eof = false;

    loop {
        let startline: usize = lineno;
        let prompt = "sm-wasm > ";
        let input: String = buffer(&runtime, global, prompt, &mut eof, &mut lineno);
    }
}

fn eval(runtime: &Runtime, buffer: &str, at: usize) {
    let context = runtime.cx();
    let compilation_opts = CompilationOptions::new(context, at, false, "repl".into());
    let mut script = Utf8Source::new(context, buffer).unwrap();

    root!(with(context); let ret_val = jsapi::UndefinedValue(););

    runtime
        .eval(&compilation_opts, &mut script, ret_val)
        .unwrap();
}

fn fmt_result(runtime: &Runtime, result: jsapi::Value) -> String {
    let context = runtime.cx();

    if result.isString() {
        root!(with(context); let js_string = result.toString(););
        return fmt_string(&runtime, js_string.handle());
    }
}

fn fmt_string(runtime: &Runtime, js_string: HandleString) -> String {}

fn buffer(
    runtime: &Runtime,
    global: HandleObject,
    prompt: &str,
    eof: &mut bool,
    lineno: &mut usize,
) -> String {
    let mut buffer: String = "".into();

    loop {
        let input: String = promptly::prompt(prompt).unwrap();

        if input.is_empty() {
            *eof = true;
            break;
        }

        buffer.push_str(&input);
        *lineno += 1;

        if !unsafe { jsapi::Utf8IsCompilableUnit(runtime.cx(), global.clone().into_raw(), &buffer) }
        {
            break;
        }
    }

    return buffer;
}
