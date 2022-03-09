use spidermonkey_wasm::{
    compilation_options::CompilationOptions,
    handle::{HandleObject, HandleValue},
    js, root,
    runtime::Runtime,
    utf8_source::Utf8Source,
    JSAutoRealm,
};

fn main() {
    let runtime = Runtime::new().unwrap();
    let global_class = js::make_default_global_class();
    let realm_opts = js::make_default_realm_options();
    let context = runtime.cx();

    root!(with(context);
        let global_object = js::new_global_object(context, &global_class, &realm_opts);
    );

    let global_object_handle = global_object.handle();
    let _ar = JSAutoRealm::new(context, global_object_handle.get());

    do_loop(&runtime, global_object_handle);
}

fn do_loop(runtime: &Runtime, global: HandleObject) {
    let mut lineno = 1;

    loop {
        let startline: usize = lineno;
        let input: String = buffer(&runtime, global, &mut lineno, startline);

        eval(&runtime, &input, startline);

        js::run_jobs(runtime.cx());
    }
}

fn eval(runtime: &Runtime, buffer: &str, at: usize) {
    let context = runtime.cx();
    let compilation_opts = CompilationOptions::new(context, at, false, "repl".into()).unwrap();
    let mut script = Utf8Source::new(context, buffer).unwrap();

    root!(with(context); let mut ret_val = js::undefined_value(););

    runtime
        .eval(&compilation_opts, &mut script, ret_val.mut_handle())
        .unwrap_or_else(|_| {
            js::report_exception(context).unwrap();
        });

    let result = fmt_result(&runtime, ret_val.handle());

    println!("{}", result);
}

fn fmt_result(runtime: &Runtime, result: HandleValue) -> String {
    let context = runtime.cx();

    if result.get().is_string() {
        root!(with(context); let js_string = result.get().to_string(););
        return js::to_rust_string(context, js_string.handle());
    }

    root!(with(context); let mut js_string = js::to_string(context, result) ;);

    js::to_rust_string(context, js_string.handle())
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

        if js::is_compilable_unit(runtime.cx(), global, &buffer) {
            break;
        }
    }

    return buffer;
}
