#include "js-confdefs.h"
#include "jsapi.h"
#include "jsfriendapi.h"
#include "js/ArrayBuffer.h"
#include "js/BuildId.h"
#include "js/CompilationAndEvaluation.h"
#include "js/ContextOptions.h"
#include "js/Conversions.h"
#include "js/Date.h"
#include "js/Equality.h"
#include "js/ForOfIterator.h"
#include "js/Id.h"
#include "js/Initialization.h"
#include "js/JSON.h"
#include "js/MemoryMetrics.h"
#include "js/Modules.h"
#include "js/Object.h"
#include "js/Promise.h"
#include "js/PropertySpec.h"
#include "js/Proxy.h"
#include "js/Realm.h"
#include "js/RegExp.h"
#include "js/SavedFrameAPI.h"
#include "js/ScalarType.h"
#include "js/SourceText.h"
#include "js/Stream.h"
#include "js/String.h"
#include "js/StructuredClone.h"
#include "js/Symbol.h"
#include "js/Utility.h"
#include "js/Warnings.h"
#include "js/WasmModule.h"
#include "js/shadow/Object.h"
#include "js/shadow/Shape.h"
#include "js/friend/DOMProxy.h"
#include "js/friend/ErrorMessages.h"
#include "js/friend/WindowProxy.h"
#include "js/experimental/JitInfo.h"
#include "js/experimental/TypedData.h"
#include "spidermonkey-wasm-sys/src/lib.rs.h"
#include "rust/cxx.h"

struct CompileOptionsParams;

typedef JS::SourceText<mozilla::Utf8Unit> Utf8UnitSourceText;
typedef JS::SourceText<char16_t> U16SourceText;

std::unique_ptr<JSClass> getDefaultGlobalClass();
std::unique_ptr<JS::OwningCompileOptions> NewOwningCompileOptions(JSContext* context, const CompileOptionsParams &opts);
JS::RealmOptions* makeDefaultRealmOptions();
std::unique_ptr<Utf8UnitSourceText> MakeUtf8UnitSourceText();

bool InitDefaultSelfHostedCode(JSContext* context);
bool Utf8SourceEvaluate(JSContext* context, const JS::OwningCompileOptions& opts, Utf8UnitSourceText& src, JS::MutableHandle<JS::Value> rval);bool InitUtf8UnitSourceText(JSContext* context, Utf8UnitSourceText& src, rust::Str units, size_t length, JS::SourceOwnership ownership);
bool InitUtf8UnitSourceText(JSContext* context, Utf8UnitSourceText& src, rust::Str units, size_t length, JS::SourceOwnership ownership);
