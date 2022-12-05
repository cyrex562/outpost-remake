//
// Created by cyrex on 2022-06-07.
//

#include "mci_ops.h"
#include "sys_api.h"
#include "utils.h"

void mci_send_command_1008_5cb6(Struct27 *param_1,param_2: i16)

{
  Struct27 *iVar1;
  u16 uVar1;
  i16 iVar2;

  mciSendCommand16(0x0,0x0,0x804,param_2);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (Struct27 *)param_1;
  if ((iVar1->field_0xa == 0x0) || (&iVar1->field_0xa != param_2)) {
    iVar1->field18_0x12 = 0x0;
    iVar2 = 0x11;
  }
  else {
    iVar1->field_0x10 = 0x0;
    iVar2 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar2);
}


void pass1_1010_1f62(Struct27 *param_1,param_2: i16)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;
  let mut lVar4: i32;
  u8 local_a [0x8];
  code **fn_ptr_1;

  pass1_1008_5784(local_a,param_1->field4_0x4);
  while( true ) {
    lVar4 = pass1_1008_5b12(local_a);
    uVar3 = ((u32)lVar4 >> 0x10);
    iVar2 = (int)lVar4;
    if (lVar4 == 0x0) break;
    if ((((iVar2 + 0x8) == 0x0) || (param_2 == 0x0)) || ((iVar2 + 0x8) == param_2)) {
      uVar1 = (u32)(iVar2 + 0x4);
      fn_ptr_1 = (code **)((int)(u32)(u32)(iVar2 + 0x4) + 0x4);
      (**fn_ptr_1)(0x1008,(int)uVar1,(int)((u32)uVar1 >> 0x10),param_2);
    }
  }
}




i32 pass1_1008_5b12(char *param_1)

{
  u32 uVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;

  if ((*(i32 *)param_1 != 0x0) && (((int)(u32)param_1 + 0x8) != 0x0)) {
    uVar4 = ((u32)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if (*(i32 *)(iVar2 + 0x4) == 0x0) {
      uVar5 = ((u32)(u32)param_1 >> 0x10);
      iVar3 = (int)(u32)param_1;
    }
    else {
      uVar1 = (u32)(iVar2 + 0x4);
      uVar5 = ((u32)uVar1 >> 0x10);
      iVar3 = (int)uVar1;
    }
//    (u32)(iVar2 + 0x4) = (u32)(iVar3 + 0x4);
    if (*(i32 *)(iVar2 + 0x4) != 0x0) {
      return 0;
    }
  }

  return 0;
}
