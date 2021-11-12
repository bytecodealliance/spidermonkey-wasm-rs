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
#include <jsapi.h>
#include <js/Initialization.h>

namespace exports {
  __attribute__((visibility("default"))) bool JS_Init();
  __attribute__((visibility("default"))) JS::RealmOptions* JS_NewRealmOptions();
  __attribute__((visibility("default"))) JS::OwningCompileOptions* JS_NewOwningCompileOptions(JSContext* cx);
  __attribute__((visibility("default"))) int32_t JS_ValueToInt32(const JS::Value* value);
}
