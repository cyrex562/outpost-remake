//
// Created by cyrex on 2/22/2022.
//

#include "winapi.h"

#include "types.h"

void InitTask16(struct CONTEXT *context)
{
    // TODO: implement
}

HGLOBAL16 LockSegment16(HGLOBAL16 handle)
{
    // TODO: implement
    HGLOBAL16 result = NULL;
    return result;
}

DWORD GetVersion16(void)
{
    // TODO: implement
    return 0;
}

u16 swi(u8 opcode)
{
    // TODO: implement
    return 0;
}

swi_0x21_fn_ptr swi_0x21()
{
    // TODO: implement
    swi_0x21_fn_ptr result = NULL;
    return result;
}

BOOL16 WaitEvent16(HTASK16 h_task)
{
    // TODO: implement
    return false;
}

i16 InitApp16(HINSTANCE16 h_instance)
{
    // TODO: implement
    return 0;
}

 void                        FatalAppExit16(u16 action, cstring str) {
     // TODO: implement
 }

  void                        FatalExit(void) {
      // TODO: implement
  }


  BOOL16                      DeleteObject16(HGDIOBJ16 obj) {
      // TODO: implement
      return 0;
  }
