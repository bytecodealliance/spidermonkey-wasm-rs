#include "api.h"

std::unique_ptr<JSClass> MakeDefaultGlobalClass() {
  const JSClass defaultGlobal = { 
    "Global",
    JSCLASS_GLOBAL_FLAGS,
    &JS::DefaultGlobalClassOps
  };

  return std::make_unique<JSClass>(defaultGlobal);
}

std::unique_ptr<JS::RealmOptions> MakeDefaultRealmOptions() {
  return std::make_unique<JS::RealmOptions>();
}

std::unique_ptr<JS::OwningCompileOptions> MakeOwningCompileOptions(JSContext* context, const CompileOptionsParams &opts) {
  JS::CompileOptions jsOpts(context);

  if (opts.force_full_parse) {
    jsOpts.setForceFullParse();
  }

  jsOpts.setFileAndLine(opts.file.data(), opts.lineno);

  auto owningOpts = std::make_unique<JS::OwningCompileOptions>(context);

  if (!owningOpts->copy(context, jsOpts)) {
    owningOpts.reset(nullptr);
  }

  return owningOpts;
}

bool InitDefaultSelfHostedCode(JSContext* context) {
  return JS::InitSelfHostedCode(context);
}

std::unique_ptr<Utf8UnitSourceText> MakeUtf8UnitSourceText(JSContext* context, rust::Str units, size_t length, JS::SourceOwnership ownership) {
  auto src = std::make_unique<Utf8UnitSourceText>();
  if (!src->init(context, units.data(), length, ownership)) {
      src.reset(nullptr);
  }

  return src;
}

bool Utf8SourceEvaluate(JSContext* context, const JS::OwningCompileOptions& opts, Utf8UnitSourceText& src, JS::MutableHandle<JS::Value> rval) {
  return JS::Evaluate(context, opts, src, rval);
}

JSScript* Utf8SourceCompile(JSContext* context, const JS::OwningCompileOptions& opts, Utf8UnitSourceText& src) {
  return JS::Compile(context, opts, src);
}

std::unique_ptr<JS::PersistentRootedObject> MakeUninitPersistentRootedObject() {
  return std::make_unique<JS::PersistentRootedObject>();
}

void InitPersistentRootedObject(JS::PersistentRootedObject& obj, JSContext* context, JSObject* initial) {
  obj.init(context, initial);
}

uint32_t DefaultHeapMaxBytes() {
  return JS::DefaultHeapMaxBytes;
}

bool Utf8IsCompilableUnit(JSContext* context, JS::HandleObject global, rust::Str source) {
  return JS_Utf8BufferIsCompilableUnit(context, global, source.data(), source.length());
}

rust::String JSStringToRustString(JSContext* context, JS::HandleString str) {
  JS::UniqueChars chars = JS_EncodeStringToUTF8(context, str);
  return rust::String(chars.get());
}

bool ReportException(JSContext* context) {
  JS::ExceptionStack stack(context);

  if (!JS::StealPendingExceptionStack(context, &stack)) {
    return false;
  }

  JS::ErrorReportBuilder report(context);
  if (!report.init(context, stack, JS::ErrorReportBuilder::WithSideEffects)) {
    return false;
  }

  JS::PrintError(stderr, report, false);

  return true;
}

void JS_SetGCCallbackWrapper(JSContext* context, JSGCCallback callback) {
  JS_SetGCCallback(context, callback, nullptr);
}

const JSClassOps* DefaultGlobalClassOps() {
  return &JS::DefaultGlobalClassOps;
}

uint32_t JSClassGlobalFlags() {
  return JSCLASS_GLOBAL_FLAGS;
}


