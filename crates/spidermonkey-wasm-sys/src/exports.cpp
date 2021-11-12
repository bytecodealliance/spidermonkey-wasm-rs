#include "exports.h"

namespace exports {
  bool JS_Init() {
    return ::JS_Init();
  }

  JS::RealmOptions* JS_NewRealmOptions() {
    JS::RealmOptions* opts = new JS::RealmOptions;
    return opts;
  }

  JS::OwningCompileOptions* JS_NewOwningCompileOptions(JSContext* cx) {
    JS::OwningCompileOptions* opts = new JS::OwningCompileOptions(cx);
    return opts;
  }

  int32_t JS_ValueToInt32(const JS::Value* value) {
    return value->toInt32();
  }
}
