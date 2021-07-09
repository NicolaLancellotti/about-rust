#ifndef clib_h
#define clib_h

#include <stdbool.h>
#include <stdint.h>

typedef struct NLOpaqueType NLOpaqueType;

typedef struct
{
    int32_t integer;
    bool boolean;
} NLValue;

typedef void (*NLAction)(void *, int32_t);

NLOpaqueType *NLOpaqueTypeCreate(NLValue value);

void NLOpaqueTypeDelete(NLOpaqueType *instance);

NLValue NLOpaqueTypeGetValue(NLOpaqueType const *instance);

void NLOpaqueTypeComputeValue(NLOpaqueType *instance, int32_t count, ...);

void NLOpaqueTypeRegisterCallback(NLOpaqueType *instance,
                                  void *target,
                                  NLAction action);

void NLOpaqueTypeTriggerCallback(NLOpaqueType const *instance);

void NLInitVector(int64_t *p, int count);

#endif
