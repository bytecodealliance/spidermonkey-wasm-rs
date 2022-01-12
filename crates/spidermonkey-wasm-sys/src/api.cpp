#include "api.h"

std::unique_ptr<JSClass> getDefaultGlobalClass() {
  const JSClass defaultGlobal = { 
    "Global",
    JSCLASS_GLOBAL_FLAGS,
    &JS::DefaultGlobalClassOps
  };

  return std::make_unique<JSClass>(defaultGlobal);
}

JS::RealmOptions* makeDefaultRealmOptions() {
  return new JS::RealmOptions();
}
