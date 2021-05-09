@ stdcall D3DRMColorGetAlpha
@ stdcall D3DRMColorGetBlue
@ stdcall D3DRMColorGetGreen
@ stdcall D3DRMColorGetRed
@ stdcall D3DRMCreateColorRGB(float float float)
@ stdcall D3DRMCreateColorRGBA(float float float float)
@ stdcall D3DRMMatrixFromQuaternion(ptr ptr)
@ stdcall D3DRMQuaternionFromRotation(ptr ptr float)
@ stdcall D3DRMQuaternionMultiply(ptr ptr ptr)
@ stdcall D3DRMQuaternionSlerp(ptr ptr ptr float)
@ stdcall D3DRMVectorAdd(ptr ptr ptr)
@ stdcall D3DRMVectorCrossProduct(ptr ptr ptr)
@ stdcall D3DRMVectorDotProduct(ptr ptr)
@ stdcall D3DRMVectorModulus(ptr)
@ stdcall D3DRMVectorNormalize(ptr)
@ stdcall D3DRMVectorRandom(ptr)
@ stdcall D3DRMVectorReflect(ptr ptr ptr)
@ stdcall D3DRMVectorRotate(ptr ptr ptr float)
@ stdcall D3DRMVectorScale(ptr ptr float)
@ stdcall D3DRMVectorSubtract(ptr ptr ptr)
@ stdcall Direct3DRMCreate(ptr)
@ stdcall -private DllCanUnloadNow()
@ stdcall -private DllGetClassObject(ptr ptr ptr)
