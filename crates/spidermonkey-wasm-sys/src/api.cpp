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

std::unique_ptr<Utf8UnitSourceText> MakeUtf8UnitSourceText() {
  return std::make_unique<Utf8UnitSourceText>();
}

bool InitUtf8UnitSourceText(JSContext* context, Utf8UnitSourceText& src, rust::Str units, size_t length, JS::SourceOwnership ownership) {
  return src.init(context, units.data(), length, ownership);
}

bool Utf8SourceEvaluate(JSContext* context, const JS::OwningCompileOptions& opts, Utf8UnitSourceText& src, JS::MutableHandle<JS::Value> rval) {
  return JS::Evaluate(context, opts, src, rval);
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
