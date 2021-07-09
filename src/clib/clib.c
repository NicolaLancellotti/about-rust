#include "clib.h"
#include <stdarg.h>
#include <stdlib.h>

struct NLOpaqueType {
  NLValue value;
  void *target;
  NLAction action;
};

NLOpaqueType *NLOpaqueTypeCreate(NLValue value) {
  NLOpaqueType *instance = (NLOpaqueType *)malloc(sizeof(NLOpaqueType));
  instance->value = value;
  return instance;
}

void NLOpaqueTypeDelete(NLOpaqueType *instance) { free(instance); }

NLValue NLOpaqueTypeGetValue(NLOpaqueType const *instance) {
  return instance->value;
}

void NLOpaqueTypeComputeValue(NLOpaqueType *instance, 
                                       int32_t count, ...) {
  va_list ap;
  va_start(ap, count);

  double sum = 0;
  for (int i = 0; i < count; ++i) {
    sum += va_arg(ap, int32_t);
  }
  va_end(ap);
  instance->value.integer = sum;
}

void NLOpaqueTypeRegisterCallback(NLOpaqueType *instance, void *target,
                                  NLAction action) {
  instance->target = target;
  instance->action = action;
}

void NLOpaqueTypeTriggerCallback(NLOpaqueType const *instance) {
  if (instance->target && instance->action) {
    instance->action(instance->target, instance->value.integer);
  }
}

void NLInitVector(int64_t *vector, int count) {
  for (int i = 0; i < count; ++i) {
    vector[i] = i;
  }
}