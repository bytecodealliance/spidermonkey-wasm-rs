use spidermonkey_wasm::{
    compilation_options::CompilationOptions,
    handle::{HandleObject, HandleString, HandleValue},
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

    do_loop(&runtime, global_object_handle);
}

fn do_loop(runtime: &Runtime, global: HandleObject) {
    let mut lineno = 1;

    loop {
        let startline: usize = lineno;
        let input: String = buffer(&runtime, global, &mut lineno, startline);

        eval(&runtime, &input, startline);

        unsafe { jsapi::RunJobs(runtime.cx()) };
    }
}

fn eval(runtime: &Runtime, buffer: &str, at: usize) {
    let context = runtime.cx();
    let compilation_opts = CompilationOptions::new(context, at, false, "repl".into()).unwrap();
    let mut script = Utf8Source::new(context, buffer).unwrap();

    root!(with(context); let mut ret_val = jsapi::UndefinedValue(););

    runtime
        .eval(&compilation_opts, &mut script, ret_val.mut_handle())
        .unwrap_or_else(|_| {
            let success = unsafe { jsapi::ReportException(context) };

            if !success {
                panic!("Couldn't report exception");
            }
        });

    let result = fmt_result(&runtime, ret_val.handle());

    println!("{}", result);
}

fn fmt_result(runtime: &Runtime, result: HandleValue) -> String {
    let context = runtime.cx();

    if result.get().isString() {
        root!(with(context); let js_string = result.get().toString(););
        return fmt_string(&runtime, js_string.handle());
    }

    root!(with(context); let mut js_string = unsafe { jsapi::ToString(context, result.into_raw()) };);

    return unsafe { jsapi::JSStringToRustString(context, js_string.handle().into_raw()) };
}

fn fmt_string(runtime: &Runtime, js_string: HandleString) -> String {
    return unsafe { jsapi::JSStringToRustString(runtime.cx(), js_string.into_raw()) };
}

fn buffer(runtime: &Runtime, global: HandleObject, lineno: &mut usize, startline: usize) -> String {
    let mut buffer: String = "".into();

    loop {
        let prompt = if startline == *lineno {
            "sm-wasm > "
        } else {
            ".. "
        };

        let input: String = promptly::prompt(prompt).unwrap();

        buffer.push_str(&input);
        *lineno += 1;

        if unsafe { jsapi::Utf8IsCompilableUnit(runtime.cx(), global.clone().into_raw(), &buffer) }
        {
            break;
        }
    }

    return buffer;
}
