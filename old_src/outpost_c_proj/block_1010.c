//
// Created by cyrex on 2022-05-22.
//

#include "block_1010.h"


astruct_19 * pass1_1010_0000(astruct_19 *param_1,u16 param_2)

{
  u32 in_EDX;
  u16 uVar1;
  astruct_19 *paVar2;
  u32 *puVar3;
  u16 in_stack_0000fe86;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb4;
  i16 iVar4;
  i16 iVar5;
  u16 uVar6;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  uVar1 = ((u32)in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  ((int)param_1 + 0xc) = 0x0;
  param_1->offset_0x0 = 0x2c8;
  ((int)param_1 + 0x2) = 0x1010;
  iVar5 = (int)param_1 + 0xa;
  iVar4 = (int)param_1 + 0xc;
  uVar6 = param_1;
  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(uVar1,(int)((u32)paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(iVar4,0x48),in_stack_0000fe86,in_stack_0000ffaa,in_stack_0000ffb0,
                           in_stack_0000ffb4);
  pass1_1008_3e94((u16 *)((u32)puVar3 & 0xffff0000 | (u32)((int)puVar3 + 0xe)),(u16 *)CONCAT22(param_1,iVar4),
                  (char *)CONCAT22(uVar6,iVar5));
  return param_1;
}



void pass1_1010_0052(u16 *param_1,u16 param_2)

{
  *param_1 = 0x2c8;
  ((int)param_1 + 0x2) = 0x1010;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



void set_window_placement_1010_0070(param_1: u32,i16 param_2,u16 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u32 *puVar3;
  let mut lVar4: i32;
  u16 uVar5;
  u8 local_18 [0x6];
  INT16 IStack18;
  i16 iStack16;
  INT16 IStack14;
  INT16 IStack12;
  INT16 IStack10;
  INT16 IStack8;
  u16 uStack6;
  u16 uStack4;

  local_18 = 0x16;
  local_18._2_4_ = 0x0;
  IStack18 = 0x0;
  iStack16 = 0x0;
  IStack14 = 0x0;
  IStack12 = 0x0;
  IStack10 = 0x0;
  IStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uVar5 = param_3;
  GetWindowPlacement16((WINDOWPLACEMENT16 *)local_18,(HWND16)0x1050);
  if ((iStack16 == -0x1) || (param_2 != 0x0)) {
    local_18._2_4_ = 0x50001;
    lVar4 = GetWindowLong16(0x0,param_3);
    uVar2 = ((u32)lVar4 >> 0x10);
    puVar3 = (u32 *)(u32)((int)lVar4 + 0xe0);
    ppcVar1 = (code **)((int)*puVar3 + 0x38);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puVar3,((int)lVar4 + 0xe2),uVar5);
    pass1_1010_01f8(param_1,CONCAT22(0x1050,local_18),(int)puVar3);
    SetWindowPlacement16((WINDOWPLACEMENT16 *)local_18,(HWND16)0x1050);
  }
  return;
}



void set_win_placement_1010_010e(u16 param_1,u16 param_2,u16 param_3)

{
  code **ppcVar1;
  i16 iVar2;
  i16 *piVar3;
  u16 uVar4;
  u32 *puVar5;
  u16 DX_REG;
  let mut lVar6: i32;
  u16 uVar7;
  WINDOWPLACEMENT16 local_18;
  i16 iStack6;
  i16 iStack4;

  local_18.length = 0x16;
  local_18.flags = 0x0;
  local_18.show_cmd = 0x0;
  local_18.pt_min_position.x = 0x0;
  local_18.pt_min_position.y = 0x0;
  local_18.pt_max_position.x = 0x0;
  local_18.pt_max_position.y = 0x0;
  local_18.rc_normal_position.x = 0x0;
  local_18.rc_normal_position.y = 0x0;
  iStack6 = 0x0;
  iStack4 = 0x0;
  uVar7 = param_3;
  GetWindowPlacement16(&local_18,(HWND16)0x1050);
  if (local_18.rc_normal_position.x == -0x1) {
    lVar6 = GetWindowLong16(0x0,param_3);
    uVar4 = ((u32)lVar6 >> 0x10);
    puVar5 = (u32 *)(u32)((int)lVar6 + 0xe0);
    ppcVar1 = (code **)((int)*puVar5 + 0x1c);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puVar5,((int)lVar6 + 0xe2),uVar7);
    iVar2 = (int)puVar5;
    piVar3 = (i16 *)((u32)puVar5 & 0xffff | (u32)DX_REG << 0x10);
    local_18.show_cmd = 0x9;
    local_18.rc_normal_position.x = *piVar3;
    local_18.rc_normal_position.y = *(INT16 *)(iVar2 + 0x2);
    iStack6 = (iVar2 + 0x4) + *piVar3;
    iStack4 = (iVar2 + 0x2) + (iVar2 + 0x6);
    SetWindowPlacement16(&local_18,(HWND16)0x1050);
  }
  return;
}



void enum_child_windows_1010_01be(void)

{
  void *func;

  if (PTR_LOOP_1050_0010 == NULL) {
    func = MakeProcInstance16(HINSTANCE16_1050_038c,win_ui_op_1010_0240);
    EnumChildWindows1(0x0,func,(HWND16)func);
    FreeProcInstance16(func);
  }
  return;
}



void pass1_1010_01f8(param_1: u32,param_2: u32,param_3: i16)

{
  i16 iVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;

  iVar2 = (param_3 * 0x4 + 0xe02) * 0x4;
  iVar1 = (iVar2 + 0xdfc);
  uVar3 = (param_1 >> 0x10);
  uVar4 = (param_2 >> 0x10);
  ((int)param_2 + 0x6) =
       (param_3 * 0x4 + 0xe04) * 0x28 + (iVar2 + 0xdfa) + ((int)param_1 + 0xa);
  ((int)param_2 + 0x8) = ((int)param_1 + 0xc) + iVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 win_ui_op_1010_0240(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  code **ppcVar1;
  BOOL16 BVar2;
  WORD WVar3;
  u16 in_register_0000000a;
  Struct57*paVar4;
  u32 *puVar5;
  u16 in_stack_0000fea2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd0;
  u32 in_stack_0000fff8;
  u16 uVar6;

  uVar6 = ((u32)in_stack_0000fff8 >> 0x10);
  paVar4 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  BVar2 = IsWindow16(param_4);
  if (BVar2 != 0x0) {
    WVar3 = GetWindowWord16(-0x6,param_4);
    if (WVar3 == HINSTANCE16_1050_038c) {
      BVar2 = IsIconic16(param_4);
      if (BVar2 != 0x0) {
        puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x45),in_stack_0000fea2,
                                 in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
        ppcVar1 = (code **)((int)(u32)((u32)puVar5 & 0xffff0000 | (u32)param_4) + 0x10);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puVar5,(int)((u32)puVar5 >> 0x10),0x1);
      }
    }
  }
  return 0x1;
}



u16 * pass1_1010_02a2(u16 *param_1,u8 param_2)

{
  pass1_1010_0052(param_1,0x1050);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1010_02e0(astruct_19 *param_1,u16 param_2)

{
  u16 uVar1;
  u16 uVar2;
  u32 in_EDX;
  u16 uVar4;
  Struct57*paVar3;
  astruct_19 *paVar5;

  uVar4 = ((u32)in_EDX >> 0x10);
  paVar5 = struct_op_1010_1d48(param_1,param_2);
  paVar3 = (Struct57*)CONCAT22(uVar4,(int)((u32)paVar5 >> 0x10));
  uVar1 = 0x0;
  (u32)((int)param_1 + 0xa) = 0x0;
  ((int)param_1 + 0xe) = 0x0;
  ((int)param_1 + 0x10) = 0x0;
  ((int)param_1 + 0x18) = 0x0;
  param_1->offset_0x0 = 0xe98;
  ((int)param_1 + 0x2) = 0x1010;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  if (uVar2 == 0x0) {
    (u32)((int)param_1 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((Struct57*)CONCAT22(paVar3,uVar1));
    ((int)param_1 + 0xa) = uVar1;
    ((int)param_1 + 0xc) = uVar2;
  }
  return;
}



void pass1_1010_0350(astruct_455 *param_1)

{
  u32 *puVar1;
  u32 *puVar2;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xe98;
  iVar4->field1_0x2 = 0x1010;
  puVar1 = (u32 *)iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}



void pass1_1010_038e(Struct27 *param_1,param_2: i16)

{
  bool bVar1;
  Struct27 *iVar2;
  u16 uVar2;

  bVar1 = false;
  iVar2 = (Struct27 *)param_1;
  uVar2 = ((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (&iVar2->field_0x18 == 0x0) {
      iVar2->field18_0x12 = DAT_1050_0e28;
      iVar2->field19_0x14 = PTR_LOOP_1050_0e30;
      iVar2->field20_0x16 = (int)PTR_LOOP_1050_0ea8;
      DAT_1050_0e28 = 0x0;
      PTR_LOOP_1050_0e30 = NULL;
      PTR_LOOP_1050_0ea8 = NULL;
      &iVar2->field_0x18 = 0x1;
      bVar1 = true;
      goto LAB_1010_0404;
    }
  }
  if (param_2 == 0x0) {
    if (&iVar2->field_0x18 != 0x0) {
      DAT_1050_0e28 = iVar2->field18_0x12;
      PTR_LOOP_1050_0e30 = (u8 *)iVar2->field19_0x14;
      PTR_LOOP_1050_0ea8 = (u8 *)iVar2->field20_0x16;
      &iVar2->field_0x18 = 0x0;
      bVar1 = true;
    }
  }
LAB_1010_0404:
  if (bVar1) {
    pass1_1010_1f62(param_1,0x3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1010_041a(void)

{
  u32 uVar1;

  uVar1 = pass1_1030_8326();
  if (((int)(uVar1 >> 0x10) == 0x0) && (uVar1 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}



void pass1_1010_043a(Struct27 *param_1,i32 param_2,param_3: i16)

{
  code **ppcVar1;
  char *pchar_1;
  u16 uVar2;
  Struct57*in_EDX;
  Struct27 *iVar4;
  astruct_227 *iVar5;
  Struct27 *uVar3;
  u16 uVar4;
  u16 *puStack18;
  u16 *puStack14;
  char local_a [0x8];

  iVar4 = (Struct27 *)param_1;
  uVar3 = (Struct27 *)((u32)param_1 >> 0x10);
  if (param_3 == 0xc) {
    if (&iVar4->field_0xe != 0x0) {
      return;
    }
    &iVar4->field_0xe = 0x1;
  }
  else if (param_3 == 0xd) {
    if (&iVar4->field_0x10 != 0x0) {
      return;
    }
    &iVar4->field_0x10 = 0x1;
  }
  else if (param_3 == 0x12) {
    return;
  }
  pass1_1010_089e(param_1,0x1,0x6);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)&iVar4->field_0xa);
  do {
    pchar_1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,pchar_1));
    uVar2 = in_EDX;
    in_EDX = (Struct57*)((u32)in_EDX & 0xffff0000 | (u32)(uVar2 | pchar_1));
    if ((uVar2 | pchar_1) == 0x0) {
      mem_op_1000_179c(0xa,in_EDX);
      uVar2 = in_EDX;
      puStack18 = (u16 *)CONCAT22(uVar2,pchar_1);
      if ((uVar2 | pchar_1) == 0x0) {
        puStack14 = NULL;
      }
      else {
        *puStack18 = 0x389a;
        (pchar_1 + 0x2) = 0x1008;
        *puStack18 = 0xea8;
        (pchar_1 + 0x2) = 0x1010;
        puStack14 = puStack18;
      }
      uVar4 = ((u32)puStack14 >> 0x10);
      iVar5 = (astruct_227 *)puStack14;
      iVar5->field4_0x4 = param_3;
      iVar5->field5_0x6 = param_2;
      ppcVar1 = (code **)((int)*&iVar4->field_0xa + 0x8);
      (**ppcVar1)(0x1000,(u32)&iVar4->field_0xa,iVar5,uVar4);
      return;
    }
  } while (((pchar_1 + 0x4) != param_3) || (*(i32 *)(pchar_1 + 0x6) != param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_0538(Struct27 *param_1,char **param_2,char **param_3)

{
  i16 iVar1;
  u32 uVar2;
  code **ppcVar3;
  u32 uVar4;
  u16 uVar5;
  u16 uVar6;
  char *pcVar7;
  u8 *puVar8;
  u16 DX_REG;
  u8 *puVar9;
  u8 *DX_REG_00;
  Struct27 *uVar10;
  Struct27 *uVar11;
  u16 uVar12;
  u32 *puStack6;

  uVar5 = 0x0;
  *param_3 = NULL;
  *param_2 = NULL;
  uVar11 = (Struct27 *)((u32)param_1 >> 0x10);
  uVar10 = (Struct27 *)param_1;
  ppcVar3 = (code **)((int)*&uVar10->field_0xa + 0x10);
  (**ppcVar3)();
  puStack6 = (u32 *)CONCAT22(DX_REG,uVar5);
  puVar9 = (u8 *)(DX_REG | uVar5);
  if (puVar9 == NULL) {
    return;
  }
  iVar1 = (uVar5 + 0x4);
  uVar2 = (u32)(uVar5 + 0x6);
  if ((DX_REG | uVar5) != 0x0) {
    ppcVar3 = (code **)*puStack6;
    (**ppcVar3)();
    puVar9 = DX_REG_00;
  }
  uVar4 = (u32)&uVar10->field_0xa;
  if (((int)uVar4 + 0x8) == 0x0) {
    pass1_1010_089e(param_1,0x0,0x6);
    pass1_1010_1f62(param_1,0x3);
  }
  uVar6 = iVar1 + 0x757;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),uVar6);
  param_3 = uVar6;
  *(u8 **)((int)param_3 + 0x2) = puVar9;
  while (pcVar7 = pass1_1000_472c((u32)*param_3,'%'), (puVar9 | pcVar7) != 0x0) {
    pass1_1010_09b4(puVar9,uVar10,uVar11,(u8 *)CONCAT22(puVar9,pcVar7),param_3,uVar2);
  }
  if (0x1e < iVar1 - 0x1U) goto LAB_1010_0850;
  uVar12 = ((u32)param_2 >> 0x10);
  switch(iVar1) {
  case 0x1:
    goto LAB_1010_0619;
  case 0x2:
    goto LAB_1010_0619;
  case 0x3:
    break;
  case 0x4:
    goto LAB_1010_0619;
  case 0x5:
    goto LAB_1010_0619;
  case 0x6:
    break;
  case 0x7:
    goto LAB_1010_0619;
  case 0x8:
    goto LAB_1010_0619;
  case 0x9:
    break;
  case 0xa:
    goto LAB_1010_0619;
  case 0xb:
    goto LAB_1010_0619;
  case 0xc:
    break;
  case 0xd:
    goto LAB_1010_0619;
  case 0xe:
    break;
  case 0xf:
    goto LAB_1010_0619;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
    break;
  case 0x15:
    break;
  case 0x16:
LAB_1010_0619:
    puVar8 = pass1_1008_5fd8(puVar9);
    goto LAB_1010_0621;
  case 0x17:
    break;
  case 0x18:
    break;
  case 0x19:
  case 0x1f:
LAB_1010_0785:
    puVar8 = pass1_1008_5fd8(puVar9);
    goto LAB_1010_0621;
  case 0x1a:
    goto LAB_1010_0785;
  case 0x1b:
    goto LAB_1010_0785;
  case 0x1c:
    break;
  case 0x1d:
    break;
  case 0x1e:
    puVar8 = pass1_1008_5fd8(puVar9);
    *(u8 **)param_2 = puVar8;
    *(u8 **)((int)param_2 + 0x2) = puVar9;
    goto LAB_1010_0785;
  }
  puVar8 = pass1_1008_5fd8(puVar9);
LAB_1010_0621:
  *(u8 **)param_2 = puVar8;
  *(u8 **)((int)param_2 + 0x2) = puVar9;
LAB_1010_0850:
  while (pcVar7 = pass1_1000_472c((u32)*param_2,'%'), (puVar9 | pcVar7) != 0x0) {
    pass1_1010_09b4(puVar9,uVar10,uVar11,(u8 *)CONCAT22(puVar9,pcVar7),param_2,uVar2);
  }
  return;
}



u16 pass1_1010_0886(void)

{
  return 0xa;
}



u16 pass1_1010_088c(void)

{
  return 0x3;
}



u16 pass1_1010_0892(void)

{
  return 0x3;
}



u16 pass1_1010_0898(void)

{
  return 0x3;
}



void pass1_1010_089e(Struct27 *param_1,u16 param_2,param_3: i16)

{
  ((param_3 + -0x1) * 0x8 + 0xe28) = param_2;
  pass1_1010_1f62(param_1,0x3);
  return;
}



void pass1_1010_08c0(param_1: u32,u16 param_2,param_3: i16)

{
  ((param_3 + -0x1) * 0x8 + 0xea8) = param_2;
  pass1_1010_1f62((Struct27 *)param_1,0x3);
  return;
}



u32 pass1_1010_08e2(u16 param_1,u16 param_2,param_3: i16)

{
  if (PTR_LOOP_1050_4fe8 != NULL) {
    DAT_1050_0e28 = 0x0;
    PTR_LOOP_1050_0e30 = NULL;
    PTR_LOOP_1050_0e38 = NULL;
    PTR_LOOP_1050_0e40 = NULL;
    PTR_LOOP_1050_0e48 = NULL;
    DAT_1050_0e58 = 0x0;
    DAT_1050_0e60 = 0x0;
    PTR_LOOP_1050_0e70 = NULL;
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe22);
}



u32 pass1_1010_091e(u16 param_1,u16 param_2,param_3: i16)

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe72);
}



u32 pass1_1010_0932(u16 param_1,u16 param_2,param_3: i16)

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe8a);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_0946(u16 param_1,u16 param_2,i16 param_3,u8 *param_4,i16 param_5,u16 param_6)

{
  i16 iVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 *puVar3;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 uVar4;
  u16 uVar5;
  let mut lVar6: i32;

  PTR_LOOP_1050_0ea8 = NULL;
  lVar6 = 0x4000001;
  uVar4 = 0x2;
  uVar5 = 0x400;
  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_4),_u16_1050_0ed0,(u8 **)0x2003b,
                           in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar2 = ((u32)puVar3 >> 0x10);
  iVar1 = (int)puVar3;
  pass1_1008_dfa6((u32)puVar3,CONCAT22(uVar5,uVar4),lVar6);
  if (iVar1 != 0x0) {
    pass1_1030_8344(_u16_1050_5748,0x4000002);
    if (*(i32 *)(iVar1 + 0x200) == 0x8000002) {
      PTR_LOOP_1050_0ea8 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xea2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_09b4(u16 param_1,u16 param_2,u16 param_3,u8 *param_4,char **param_5,u32 param_6)

{
  u8 bVar1;
  bool bVar2;
  bool bVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  Struct57*paVar8;
  u32 *puVar9;
  char *pcVar10;
  u16 in_stack_0000fe86;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffde;
  char *pcStack18;
  u16 uStack10;
  u16 uStack8;

  paVar8 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  bVar3 = false;
  bVar2 = false;
  bVar1 = *(u8 *)((int)param_4 + 0x1);
  if (bVar1 == 0x70) {
LAB_1010_0a06:
    bVar3 = false;
    bVar2 = true;
  }
  else if (bVar1 < 0x71) {
    if (bVar1 != 0x43) {
      if (bVar1 == 0x50) goto LAB_1010_0a06;
      if (bVar1 != 0x63) goto LAB_1010_09db;
    }
    bVar3 = true;
    bVar2 = false;
  }
LAB_1010_09db:
  uVar7 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x0;
  if (bVar2) {
    puVar9 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x2),in_stack_0000fe86,
                             in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
    uVar7 = ((u32)puVar9 >> 0x10);
    uStack10 = ((int)puVar9 + 0x6c);
    paVar8 = (Struct57*)((u32)paVar8 & 0xffff0000 | (u32)((int)puVar9 + 0x6e));
  }
  else {
    if (!bVar3) goto LAB_1010_0a36;
    pass1_1030_8344(_u16_1050_5748,param_6);
    pcVar10 = pass1_1038_4d28((char *)CONCAT22((int)paVar8,uVar7));
    paVar8 = (Struct57*)((u32)paVar8 & 0xffff0000 | (u32)pcVar10 >> 0x10);
    uStack10 = pcVar10;
  }
  uStack8 = paVar8;
LAB_1010_0a36:
  if ((uStack8 | uStack10) != 0x0) {
    uVar4 = str_op_1000_3da4(*param_5);
    uVar5 = str_op_1000_3da4((char *)CONCAT22(uStack8,uStack10));
    iVar6 = uVar5 + 0xa + uVar4;
    mem_op_1000_179c(iVar6,paVar8);
    uVar7 = SUB42(paVar8,0x0);
    pcStack18 = (char *)CONCAT22(uVar7,iVar6);
    *param_4 = '\0';
    unk_str_op_1000_3d3e((char *)CONCAT22(uVar7,iVar6),*param_5);
    pass1_1000_3cea(CONCAT22(uVar7,iVar6),(char *)CONCAT22(uStack8,uStack10));
    pass1_1000_3cea(CONCAT22(uVar7,iVar6),(char *)((u32)param_4 & 0xffff0000 | (u32)((int)param_4 + 0x2)));
    fn_ptr_1000_17ce(*param_5);
    *param_5 = pcStack18;
  }
  return;
}



void pass1_1010_0ad2(param_1: u32,u32 param_2)

{
  u32 uVar1;
  BOOL16 BVar2;
  u8 *puVar3;
  u16 DX_REG;
  i16 iVar4;
  u16 uVar5;
  HFILE16 in_stack_0000ffbc;
  u32 local_2a [0x2];
  u16 local_22 [0x2];
  u16 local_1e [0x3];
  u16 local_18 [0x3];
  u32 uStack18;
  u8 local_e [0x8];
  u16 uStack6;
  i16 iStack4;

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 == 0x0) {
    return;
  }
  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0xa) == 0x0) {
    uStack6 = 0x0;
  }
  else {
    uVar1 = (u32)(iVar4 + 0xa);
    uStack6 = ((int)uVar1 + 0x8);
  }
  local_1e[0] = uStack6;
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1e),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_e),(u32)(iVar4 + 0xa));
    do {
      puVar3 = local_e;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
      uStack18 = CONCAT22(DX_REG,puVar3);
      if ((DX_REG | puVar3) == 0x0) {
        local_22[0] = (iVar4 + 0xe);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_22),(char *)0x2,in_stack_0000ffbc);
        if (BVar2 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_22[0] = (iVar4 + 0x10);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_22),(char *)0x2,in_stack_0000ffbc);
        if (BVar2 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        if ((iVar4 + 0x18) != 0x0) {
          DAT_1050_0e28 = (iVar4 + 0x12);
          PTR_LOOP_1050_0e30 = (u8 *)(iVar4 + 0x14);
          PTR_LOOP_1050_0ea8 = (u8 *)(iVar4 + 0x16);
        }
        iStack4 = 0x0;
        while( true ) {
          if (0x9 < iStack4) {
            iStack4 = 0x0;
            while( true ) {
              if (0x2 < iStack4) {
                if ((iVar4 + 0x18) != 0x0) {
                  DAT_1050_0e28 = 0x0;
                  PTR_LOOP_1050_0e30 = NULL;
                  PTR_LOOP_1050_0ea8 = NULL;
                }
                return;
              }
              local_1e[0] = (iStack4 * 0x8 + 0xea8);
              BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1e),(char *)0x2,in_stack_0000ffbc);
              if (BVar2 == 0x0) break;
              iStack4 += 0x1;
            }
            u16_1050_0310 = 0x6d0;
            return;
          }
          local_1e[0] = (iStack4 * 0x8 + 0xe28);
          BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1e),(char *)0x2,in_stack_0000ffbc);
          if (BVar2 == 0x0) break;
          iStack4 += 0x1;
        }
        u16_1050_0310 = 0x6d0;
        return;
      }
      local_18[0] = (puVar3 + 0x4);
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_18),(char *)0x2,in_stack_0000ffbc);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return;
      }
      local_2a[0] = (u32)((int)uStack18 + 0x6);
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_2a),(char *)0x4,in_stack_0000ffbc);
    } while (BVar2 != 0x0);
  }
  u16_1050_0310 = 0x6d0;
  return;
}



void file_1010_0c7c(i16 param_1,u8 *param_2,astruct_228 *param_3,u32 param_4)

{
  code **ppcVar1;
  BOOL16 BVar2;
  astruct_229 *uVar4;
  u16 uVar3;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  astruct_228 *iVar6;
  u16 uVar7;
  u16 local_2a [0x2];
  u16 uStack38;
  u32 *puStack26;
  u32 *puStack22;
  u16 local_12 [0x5];
  astruct_229 *paStack8;
  astruct_229 *local_6;
  u16 uStack4;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  uVar7 = (param_4 >> 0x10);
  read_file_1008_7cfe(param_4,uVar7,0x6);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,&local_6),0x2);
    if (BVar2 != 0x0) {
      for (paStack8 = NULL; iVar6 = (astruct_228 *)param_3, paStack8 < local_6;
          paStack8 = (astruct_229 *)&paStack8->field_0x1) {
        uVar4 = local_6;
        mem_op_1000_179c(0xa,paVar6);
        uVar5 = paVar6;
        puStack26 = (u32 *)CONCAT22(uVar5,uVar4);
        paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar4));
        if ((uVar5 | uVar4) == 0x0) {
          puStack22 = NULL;
        }
        else {
          puStack26 = 0x389a;
          uVar4->field2_0x2 = 0x1008;
          puStack26 = 0xea8;
          uVar4->field2_0x2 = 0x1010;
          puStack22 = puStack26;
        }
        BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_12),0x2);
        if ((BVar2 == 0x0) ||
           (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                        (u8 *)((u32)puStack22 & 0xffff0000 | (u32)((int)puStack22 + 0x6)),0x4),
           BVar2 == 0x0)) {
          puStack26 = puStack22;
          if (puStack22 != NULL) {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(0x1008,(int)puStack22,(int)((u32)puStack22 >> 0x10),0x1);
          }
          goto LAB_1010_0cb1;
        }
        ((int)puStack22 + 0x4) = local_12[0];
        ppcVar1 = (code **)((int)*iVar6->field10_0xa + 0x8);
        (**ppcVar1)();
      }
      BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar6->field_0xe)),0x2
                                 );
      if ((BVar2 != 0x0) &&
         (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                      (u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar6->field_0x10)),0x2),
         BVar2 != 0x0)) {
        for (uStack4 = 0x0; (int)uStack4 < 0xa; uStack4 += 0x1) {
          BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_2a),0x2);
          if (BVar2 == 0x0) goto LAB_1010_0cb1;
          uVar3 = uStack4;
          if ((int)u16_1050_0312 < 0x2) {
            uVar3 = pass1_1008_738c(param_4,uVar7,uStack4);
          }
          (uVar3 * 0x8 + 0xe28) = local_2a[0];
          uStack38 = uVar3;
        }
        if (0x2 < (int)u16_1050_0312) {
          uStack4 = 0x0;
          do {
            BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_2a),0x2);
            if (BVar2 == 0x0) goto LAB_1010_0cb1;
            (uStack4 * 0x8 + 0xea8) = local_2a[0];
            uStack4 += 0x1;
          } while ((int)uStack4 < 0x3);
        }
        return;
      }
    }
LAB_1010_0cb1:
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 * pass1_1010_0e46(u16 *param_1,u8 param_2,u16 param_3)

{
  pass1_1010_0350((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_0e6c(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_19 * pass1_1010_0eac(u8 *param_1,astruct_19 *param_2,u16 param_3)

{
  struct_op_1018_4cda(param_2,param_3);
  param_2->offset_0x0 = 0xf0c;
  ((int)param_2 + 0x2) = 0x1010;
  PTR_LOOP_1050_4230 = param_2;
  pass1_1018_4dce(param_1,param_2,0xff);
  return param_2;
}



StructD * pass1_1010_0ee6(StructD *param_1,u8 param_2)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_0f24(u8 *param_1,astruct_19 *param_2,u16 param_3)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  u16 unaff_BP;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  struct_1010_2cd2(param_2,param_3);
  (u32)((int)param_2 + 0x60) = 0x0;
  (u32)((int)param_2 + 0x64) = 0x0;
  ((int)param_2 + 0x68) = 0x0;
  (u32)((int)param_2 + 0x6a) = 0x0;
    // 0x191a
  param_2->offset_0x0 = (int)s_648_bmp_1050_1919 + 0x1;
  ((int)param_2 + 0x2) = 0x1010;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  ((int)param_2 + 0x6a) = (int)puVar2;
  ((int)param_2 + 0x6c) = (int)((u32)puVar2 >> 0x10);
  return;
}



void pass1_1010_0f76(astruct_455 *param_1)

{
  astruct_455 *uVar1;

  uVar1 = (astruct_455 *)((u32)param_1 >> 0x10);
  param_1->field0_0x0 = (int)s_648_bmp_1050_1919 + 0x1;
  ((int)param_1 + 0x2) = 0x1010;
  pass1_1010_17c0((astruct_455 *)((u32)param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
  pass1_1010_2db2(param_1);
  return;
}



void struct_1010_0f9c(u16 param_1,astruct_232 *param_2,Struct57*param_3)

{
  u8 *puVar1;
  code **ppcVar2;
  i16 iVar3;
  astruct_92 *paVar4;
  astruct_92 *paVar5;
  u16 uVar6;
  u32 uVar7;
  u16 uVar8;
  astruct_232 *iVar8;
  astruct_231 *iVar9;
  astruct_230 *iVar10;
  astruct_233 *iVar11;
  astruct_232 *uVar9;
  u16 uVar10;
  astruct_232 *paVar11;
  u8 uVar12;
  u32 uStack36;
  i16 iStack32;
  u16 uStack30;
  astruct_92 **ppaStack28;
  astruct_15 *paStack24;
  astruct_92 local_14;

  uVar9 = (astruct_232 *)((u32)param_2 >> 0x10);
  iVar8 = (astruct_232 *)param_2;
  ppcVar2 = (code **)((int)(u32)param_2 + 0x38);
  (**ppcVar2)();
  iVar8->field100_0x68 = param_1;
  if ((*(i32 *)&iVar8->field96_0x60 != 0x0) && (iVar8->field100_0x68 == 0x1)) {
    return;
  }
  if (iVar8->field100_0x68 == 0x0) {
    return;
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  iVar3 = iVar8->field100_0x68 * 0x18;
  mem_op_1000_179c(iVar3,param_3);
  iVar8->field96_0x60 = iVar3;
  iVar8->field97_0x62 = (u8 *)param_3;
  ppaStack28 = (astruct_92 **)CONCAT22((u8 *)param_3,iVar8->field96_0x60);
  uStack30 = iVar8->field100_0x68;
  do {
    do {
      paVar4 = &local_14;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar8 = param_3;
      paStack24 = (astruct_15 *)CONCAT22(uVar8,paVar4);
      param_3 = (Struct57*)((u32)param_3 & 0xffff0000 | (u32)(uVar8 | paVar4));
      if ((uVar8 | paVar4) == 0x0) goto LAB_1010_10ca;
      iVar9 = (astruct_231 *)(u32)param_2;
      ppcVar2 = (code **)&iVar9->field58_0x40;
      paVar5 = paVar4;
      (**ppcVar2)(0x1028,param_2);
    } while (paVar5 == NULL);
    pass1_1028_b58e(paStack24);
    uStack36 = CONCAT22((int)param_3,paVar5);
    ppcVar2 = (code **)&iVar9->field44_0x2c;
    (**ppcVar2)(0x1028,param_2);
    uVar10 = ((u32)ppaStack28 >> 0x10);
    iVar10 = (astruct_230 *)ppaStack28;
    *ppaStack28 = paVar5;
    iVar10->field2_0x2 = (int)param_3;
    uVar12 = SUB21(paVar4,0x0);
    ppcVar2 = (code **)&iVar9->field46_0x30;
    paVar11 = param_2;
    (**ppcVar2)();
    iVar10->field7_0x8 = (u8 *)paVar5;
    iVar10->field8_0xa = (int)param_3;
    iVar10->field9_0xc = uStack36;
    ppcVar2 = (code **)&iVar9->field56_0x3c;
    uVar7 = uStack36;
    (**ppcVar2)(0x1028,param_2,paStack24,paVar11,uVar12,uVar8);
    iVar10->field10_0x10 = (int)uVar7;
    iVar10->field11_0x12 = (u8 *)param_3;
    iVar10->field12_0x14 = uStack36;
    ppaStack28 = (astruct_92 **)((u32)ppaStack28 & 0xffff0000 | ZEXT24(iVar10 + 0x1));
    uStack30 -= 0x1;
  } while (uStack30 != 0x0);
LAB_1010_10ca:
  uVar6 = iVar8->field100_0x68 << 0x2;
  mem_op_1000_179c(uVar6,param_3);
  iVar8->field98_0x64 = uVar6;
  iVar8->field99_0x66 = (u8 *)param_3;
  iStack32 = 0x0;
  uStack30 = 0x0;
  while( true ) {
    if ((int)(iVar8->field100_0x68 * 0x3) <= (int)uStack30) break;
    puVar1 = iVar8->field97_0x62;
    uVar7 = (u32)&iVar8->field98_0x64;
    uVar10 = ((u32)uVar7 >> 0x10);
    iVar11 = (astruct_233 *)uVar7;
    (iVar11 + iStack32 * 0x4) = iVar8->field96_0x60 + uStack30 * 0x8;
    *(u8 **)(iVar11 + iStack32 * 0x4 + 0x2) = puVar1;
    uStack30 += 0x3;
    iStack32 += 0x1;
  }
  return;
}



void pass1_1010_1146(param_1: u32,u16 param_2)

{
  u32 uVar1;
  u16 uVar2;

  DAT_1050_0ecc = param_2;
  uVar2 = (param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x66);
  pass1_1000_4aea(((int)param_1 + 0x64),uVar1,(int)((u32)uVar1 >> 0x10),0x4,
                  (u8 *)((int)s_dibtext_bmp_1050_1844 + 0x6));
  return;
}



void pass1_1010_116c(u32 *param_1,i16 param_2,u16 param_3)

{
  code **ppcVar1;
  i16 iVar2;
  u32 in_EDX;
  i16 iVar3;
  u16 uVar4;
  u16 uStack4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x56) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x34);
    (**ppcVar1)();
  }
  ppcVar1 = (code **)((int)*param_1 + 0x28);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    uStack4 = DAT_1050_0ecc;
    iVar2 = DAT_1050_0ecc + 0x1;
    if (iVar2 == 0x0) {
      uStack4 = 0x0;
    }
    pass1_1010_1146((u32)param_1,uStack4);
    pass1_1010_11c6(iVar2,in_EDX,(astruct_234 *)param_1);
    (iVar3 + 0x56) = iVar2;
    (iVar3 + 0x58) = (int)in_EDX;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_11c6(u16 param_1,Struct57*param_2,astruct_234 *param_3)

{
  i16 *piVar1;
  u16 *puVar2;
  code **ppcVar3;
  u32 uVar4;
  astruct_239 *iVar6;
  i16 iVar5;
  i16 iVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u16 uVar12;
  u16 uVar13;
  u16 uVar14;
  Struct57*paVar15;
  u32 uVar16;
  u32 *puVar17;
  astruct_234 *iVar18;
  i16 iVar19;
  i16 iVar21;
  astruct_238 *iVar20;
  u16 uVar22;
  u16 uVar23;
  astruct_223 *paVar24;
  Struct57*paVar25;
  u32 *puStack50;
  i16 iStack42;
  i16 iStack40;
  StructD *pSStack38;
  i16 iStack28;
  u32 *puStack26;
  u32 *puStack22;
  u32 uStack14;
  u32 uStack10;

  if (DAT_1050_0ecc == -0x1) {
    return;
  }
  mem_op_1000_179c(0x1a,param_2);
  paVar15 = (Struct57*)((u32)param_2 & 0xffff0000);
  if ((param_2 | param_1) == 0x0) {
    iVar6 = NULL;
  }
  else {
    paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(param_2,param_1));
    paVar15 = (Struct57*)((u32)paVar15 & 0xffff0000 | (u32)paVar24 >> 0x10);
    iVar6 = (astruct_239 *)paVar24;
  }
  uVar11 = SUB42(paVar15,0x0);
  uStack10 = 0x10500ece;
  uStack14 = 0x0;
  while( true ) {
    uVar22 = ((u32)param_3 >> 0x10);
    iVar18 = (astruct_234 *)param_3;
    piVar1 = &iVar18->field101_0x68;
    if (*piVar1 == (int)uStack14 || *piVar1 < (int)uStack14) break;
    uVar4 = iVar18->field100_0x64;
    uVar16 = (u32)((int)uVar4 + (int)uStack14 * 0x4);
    puVar17 = (u32 *)((int)uVar16 + DAT_1050_0ecc * 0x8);
    puStack50 = (u32 *)(uVar16 & 0xffff0000 | ZEXT24(puVar17));
    iVar5 = pass1_1000_475e(uStack10,*puVar17);
    if (iVar5 != 0x0) {
      uStack10 = *puStack50;
      uStack14 = uStack14 & 0xffff | (u32)(uStack14 + 0x1) << 0x10;
    }
    uStack14 = uStack14 & 0xffff0000 | (u32)((int)uStack14 + 0x1);
  }
  iVar6->field13_0x10 = uStack14;
  paVar25 = struct_1010_38f8(uStack14,paVar15,(astruct_240 *)CONCAT22(uVar11,iVar6),uStack14);
  paVar15 = (Struct57*)((u32)paVar15 & 0xffff0000 | (u32)paVar25 >> 0x10);
  iVar7 = 0x0;
  mem_op_1000_179c(0x400,paVar15);
  uVar12 = SUB42(paVar15,0x0);
  iVar5 = iVar7;
  mem_op_1000_179c(0x400,paVar15);
  uVar13 = SUB42(paVar15,0x0);
  pSStack38 = (StructD *)CONCAT22(uVar13,iVar5);
  iStack28 = 0x0;
  pass1_1000_4906((StructD *)CONCAT22(uVar12,iVar7),NULL,0x400);
  pass1_1000_4906((StructD *)CONCAT22(uVar13,iVar5),NULL,0x400);
  iStack42 = 0x0;
  uVar10 = 0x0;
  do {
    puVar2 = &iVar6->field13_0x10;
    if (*puVar2 == uVar10 || (int)*puVar2 < (int)uVar10) {
      return;
    }
    uVar4 = iVar18->field100_0x64;
    uVar23 = ((u32)uVar4 >> 0x10);
    iVar19 = (int)uVar4;
    iVar21 = (iVar19 + iStack28 * 0x4);
    uVar9 = (iVar19 + iStack28 * 0x4 + 0x2);
    paVar25 = (Struct57*)((u32)paVar15 & 0xffff0000 | (u32)uVar9);
    iVar19 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    puStack22 = (u32 *)CONCAT22(uVar9,iVar19);
    uVar8 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
    mem_op_1000_179c(0x1a,paVar25);
    uVar14 = paVar25 | uVar8;
    paVar15 = (Struct57*)((u32)paVar25 & 0xffff0000 | (u32)uVar14);
    if (uVar14 == 0x0) {
      uVar4 = iVar6->field8_0x8;
      (u32)((int)uVar4 + uVar10 * 0x4) = 0x0;
    }
    else {
      paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(paVar25,uVar8));
      paVar15 = (Struct57*)((u32)paVar15 & 0xffff0000 | (u32)paVar24 >> 0x10);
      uVar4 = iVar6->field8_0x8;
      uVar23 = ((u32)uVar4 >> 0x10);
      iVar21 = (int)uVar4;
      (iVar21 + uVar10 * 0x4) = (int)paVar24;
      (iVar21 + uVar10 * 0x4 + 0x2) = (int)((u32)paVar24 >> 0x10);
    }
    iStack42 += 0x1;
    uVar4 = iVar6->field8_0x8;
    uVar23 = ((u32)uVar4 >> 0x10);
    iVar21 = (int)uVar4;
    uVar4 = (u32)(iVar21 + uVar10 * 0x4);
    ppcVar3 = (code **)((int)(u32)(u32)(iVar21 + uVar10 * 0x4) + 0x1c);
    (**ppcVar3)(0x1000,(int)uVar4,(int)((u32)uVar4 >> 0x10),iStack42,iVar19,uVar9);
    uStack14 = (u32)uVar10;
    while( true ) {
      piVar1 = &iVar18->field101_0x68;
      if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
      iVar19 = iStack28 * 0x4;
      uVar4 = iVar18->field100_0x64;
      uVar4 = (u32)((int)uVar4 + iVar19);
      iVar21 = pass1_1000_475e(*puStack22,(u32)((int)uVar4 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
      if (iVar21 != 0x0) break;
      uVar4 = iVar18->field100_0x64;
      uVar23 = ((u32)uVar4 >> 0x10);
      iVar21 = (int)uVar4;
      uVar10 = (iVar21 + iVar19 + 0x2);
      paVar15 = (Struct57*)((u32)paVar15 & 0xffff0000 | (u32)uVar10);
      uVar9 = (iVar21 + iVar19) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
      puStack26 = (u32 *)CONCAT22(uVar10,uVar9);
      mem_op_1000_179c(0x1a,paVar15);
      uVar16 = (u32)paVar15 & 0xffff0000;
      if ((paVar15 | uVar9) == 0x0) {
        uVar23 = 0x0;
      }
      else {
        paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(paVar15,uVar9));
        uVar16 = uVar16 & 0xffff0000 | (u32)paVar24 >> 0x10;
        uVar23 = SUB42(paVar24,0x0);
      }
      (uStack14 * 0x4 + iVar7) = uVar23;
      (uStack14 * 0x4 + iVar7 + 0x2) = (int)uVar16;
      uVar4 = iVar18->field100_0x64;
      uVar23 = ((u32)uVar4 >> 0x10);
      iVar21 = (int)uVar4;
      uVar10 = (iVar21 + iStack28 * 0x4 + 0x2);
      paVar15 = (Struct57*)(uVar16 & 0xffff0000 | (u32)uVar10);
      iStack42 += 0x1;
      uVar4 = (u32)(uStack14 * 0x4 + iVar7);
      ppcVar3 = (code **)((int)(u32)(u32)(uStack14 * 0x4 + iVar7) + 0x1c);
      (**ppcVar3)(0x1000,(int)uVar4,(int)((u32)uVar4 >> 0x10),iStack42,
                  (iVar21 + iStack28 * 0x4) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8,uVar10);
      iStack40 = 0x0;
      while( true ) {
        piVar1 = &iVar18->field101_0x68;
        if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
        uVar4 = iVar18->field100_0x64;
        uVar4 = (u32)((int)uVar4 + iStack28 * 0x4);
        iVar21 = pass1_1000_475e(*puStack26,(u32)((int)uVar4 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
        if (iVar21 != 0x0) break;
        uVar4 = iVar18->field100_0x64;
        uVar4 = (u32)((int)uVar4 + iStack28 * 0x4);
        uVar10 = pass1_1000_475e(*puStack22,(u32)((int)uVar4 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
        if (uVar10 != 0x0) break;
        mem_op_1000_179c(0x1a,paVar15);
        uVar16 = (u32)paVar15 & 0xffff0000;
        if ((paVar15 | uVar10) == 0x0) {
          uVar23 = 0x0;
        }
        else {
          paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(paVar15,uVar10));
          uVar16 = uVar16 & 0xffff0000 | (u32)paVar24 >> 0x10;
          uVar23 = SUB42(paVar24,0x0);
        }
        (iStack40 * 0x4 + iVar5) = uVar23;
        (iStack40 * 0x4 + iVar5 + 0x2) = (int)uVar16;
        uVar4 = iVar18->field100_0x64;
        uVar23 = ((u32)uVar4 >> 0x10);
        iVar20 = (astruct_238 *)uVar4;
        uVar10 = (iVar20 + iStack28 * 0x4 + 0x2);
        paVar15 = (Struct57*)(uVar16 & 0xffff0000 | (u32)uVar10);
        iStack42 += 0x1;
        uVar4 = (u32)(iStack40 * 0x4 + iVar5);
        ppcVar3 = (code **)((int)(u32)(u32)(iStack40 * 0x4 + iVar5) + 0x1c);
        (**ppcVar3)(0x1000,(int)uVar4,(int)((u32)uVar4 >> 0x10),iStack42,
                    (iVar20 + iStack28 * 0x4) + (DAT_1050_0ecc * 0x6 + 0xebe) * 0x8,uVar10);
        iStack28 += 0x1;
        iStack40 += 0x1;
      }
      uVar4 = (u32)(uStack14 * 0x4 + iVar7);
      ((int)uVar4 + 0x10) = iStack40;
      uVar10 = iStack40 << 0x2;
      iVar21 = iVar5;
      uVar23 = uVar13;
      paVar25 = struct_1010_38f8(uVar10,paVar15,*(astruct_240 **)(uStack14 * 0x4 + iVar7),iStack40);
      paVar15 = (Struct57*)((u32)paVar15 & 0xffff0000 | (u32)paVar25 >> 0x10);
      pass1_1000_48a8((u32)paVar25,CONCAT22(uVar23,iVar21),uVar10);
      pass1_1000_4906(pSStack38,NULL,0x400);
      uStack14 = uStack14 & 0xffff | (u32)(uStack14 + 0x1) << 0x10;
    }
    uVar4 = iVar6->field8_0x8;
    uVar4 = (u32)((int)uVar4 + (int)uStack14 * 0x4);
    ((int)uVar4 + 0x10) = uStack14;
    uVar10 = uStack14 << 0x2;
    uVar4 = iVar6->field8_0x8;
    iVar21 = iVar7;
    uVar23 = uVar12;
    paVar25 = struct_1010_38f8(uVar10,paVar15,*(astruct_240 **)((int)uVar4 + (int)uStack14 * 0x4),uStack14);
    paVar15 = (Struct57*)((u32)paVar15 & 0xffff0000);
    pass1_1000_48a8((u32)paVar25,CONCAT22(uVar23,iVar21),uVar10);
    pass1_1000_4906((StructD *)CONCAT22(uVar12,iVar7),NULL,0x400);
    uVar10 = (int)uStack14 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_1656(u16 param_1,u8 *param_2,u16 param_3,Struct27 *param_4,u16 param_5,u32 param_6)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  Struct57*paVar4;
  u32 uVar5;
  i16 iVar6;
  u16 uVar7;
  Struct57*paVar8;
  astruct_15 *paVar9;
  u16 in_stack_0000fe88;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb6;

  paVar4 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  unk_destroy_win_op_1010_305a(param_1,param_4,param_5,param_6);
  if (((int)param_4 + 0x16) == 0x3) {
    paVar8 = (Struct57*)
             mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x32),in_stack_0000fe88,
                             in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
    uVar5 = (u32)paVar4 & 0xffff0000;
    uVar1 = (u32)((int)param_4 + 0x32);
    uVar1 = (u32)((int)uVar1 + 0x42);
    uVar7 = ((u32)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    uVar1 = (u32)(iVar6 + 0x16);
    paVar9 = (astruct_15 *)struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x4),iVar6,(int)((u32)paVar8 >> 0x10));
    uVar5 = uVar5 & 0xffff0000 | (u32)paVar9 >> 0x10;
    uVar2 = pass1_1010_7818((u32)paVar8,paVar9);
    uVar1 = (u32)(iVar6 + 0x16);
    uVar3 = uVar2;
    ui_op_1010_79aa(paVar8,0x0,*(i32 *)((int)uVar1 + 0x4));
    if (uVar3 == 0x0) {
      uVar1 = (u32)(iVar6 + 0x16);
      unk_win_op_1010_7300(uVar5,paVar8,0x0,uVar2,(u32)((int)uVar1 + 0x4));
    }
  }
  return;
}



void pass1_1010_16ee(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6)

{
  u16 uVar1;
  u16 in_register_0000000a;
  Struct57*paVar2;
  u16 in_stack_0000ffc0;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_6);
  mem_op_1000_179c(0x4a,paVar2);
  uVar1 = paVar2 | param_5;
  if (uVar1 != 0x0) {
    pass1_1040_c54a((astruct_65 *)CONCAT22(paVar2,param_5),0x0,(u32 *)CONCAT22(param_4,param_3),
                    in_stack_0000ffc0,(u32)paVar2 & 0xffff0000 | (u32)uVar1);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void string_1010_1722(u16 param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u16 DX_REG;
  u16 uVar1;
  char *pcVar2;
  u16 uVar3;
  u8 local_52 [0x50];

  pass1_1028_b58e((astruct_15 *)param_4);
  if ((DX_REG | param_1) == 0x0) {
    pcVar2 = load_string_1010_847e(_u16_1050_14cc,0x424);
    uVar1 = ((u32)pcVar2 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_52),pcVar2);
    pcVar2 = (char *)CONCAT22(uVar1,local_52);
    uVar3 = 0x1050;
  }
  else {
    pcVar2 = pass1_1038_4d28(*(char **)(param_1 + 0x2e));
    uVar3 = ((u32)pcVar2 >> 0x10);
  }
  str_op_1008_60e8(((u32)pcVar2 >> 0x10),(char *)((u32)pcVar2 & 0xffff | (u32)uVar3 << 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_1788(char *param_1,u16 param_2,u16 param_3,astruct_15 *param_4)

{
  u16 uVar1;
  char *pcVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  u32 *puVar4;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;
  u8 *puVar5;
  u32 in_stack_0000fff4;
  u8 **ppuVar6;
  i16 iVar7;

  ppuVar6 = (u8 **)CONCAT22((int)((u32)in_stack_0000fff4 >> 0x10),0x3);
  puVar4 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,ppuVar6,in_stack_0000fe9e
                           ,in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
  uVar3 = ((u32)puVar4 >> 0x10);
  iVar7 = (int)((u32)ppuVar6 >> 0x10);
  puVar5 = (u8 *)0x1778;
  uVar1 = pass1_1028_b58e(param_4);
  pcVar2 = pass1_1010_b038((u8 *)puVar4,uVar1,uVar3,puVar5,iVar7);
  str_op_1008_60e8(uVar3,(char *)CONCAT22(uVar3,pcVar2));
  return;
}



void pass1_1010_17c0(astruct_455 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  astruct_455 *struct_1;
  astruct_455 *struct_1_hi;
  code **fn_ptr_1;

  unk_destroy_win_op_1010_2fa0((astruct_873 *)param_1);
  struct_1_hi = (astruct_455 *)((u32)param_1 >> 0x10);
  struct_1 = (astruct_455 *)param_1;
  puVar1 = (u32 *)struct_1[0xa].field3_0x6;
  uVar2 = (struct_1 + 0xb)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    fn_ptr_1 = (code **)*puVar1;
    (**fn_ptr_1)();
  }
  (u32)&struct_1[0xa].field3_0x6 = 0x0;
  fn_ptr_1000_17ce(*(char **)(struct_1 + 0xc));
  pass1_1000_4906(*(StructD **)&struct_1[0xc].field2_0x4,NULL,(struct_1 + 0xd)->field0_0x0 << 0x2);
  fn_ptr_1000_17ce(*(char **)&struct_1[0xc].field2_0x4);
  (u32)(struct_1 + 0xc) = 0x0;
  (u32)&struct_1[0xc].field2_0x4 = 0x0;
  return;
}



void pass1_1010_184a(u32 *param_1,u32 *param_2)

{
  i16 iVar1;
  i16 iVar2;

  iVar2 = DAT_1050_0ecc;
  iVar1 = (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
  iVar1 = pass1_1000_475e((u32)(iVar1 + (int)*param_1),(u32)(iVar1 + (int)*param_2));
  if (iVar1 == 0x0) {
    iVar1 = (iVar2 * 0x6 + 0xebc) * 0x8;
    iVar1 = pass1_1000_475e((u32)(iVar1 + (int)*param_1),(u32)(iVar1 + (int)*param_2));
    if (iVar1 == 0x0) {
      iVar2 = (iVar2 * 0x6 + 0xebe) * 0x8;
      pass1_1000_475e((u32)(iVar2 + (int)*param_1),(u32)(iVar2 + (int)*param_2));
    }
  }
  return;
}



u16 FUN_1010_18e8(void)

{
  return 0x0;
}



u16 FUN_1010_18ee(void)

{
  return 0x1;
}



u16 * pass1_1010_18f4(u16 *param_1,u8 param_2,u16 param_3)

{
  pass1_1010_0f76((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_19 * pass1_1010_195e(Struct57*param_1,astruct_19 *param_2,astruct_19 *param_3,u16 param_4)

{
  u16 unaff_BP;
  u32 *puVar1;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  pass1_1010_0f24((u8 *)param_1,(astruct_19 *)CONCAT22(param_3,param_2),param_4);
  (u32)&param_2[0x1].field_0xe = 0x0;
  CONCAT22(param_3,param_2) = 0x1b2a;
  param_2->segment_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  &param_2[0x1].field_0xe = (int)puVar1;
  param_2[0x1].field8_0x10 = ((u32)puVar1 >> 0x10);
  return (astruct_19 *)CONCAT22(param_3,param_2);
}



void pass1_1010_19a4(u16 param_1,u32 *param_2)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  u16 DX_REG;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    if ((param_1 | paVar2) == 0x0) break;
    ppcVar1 = (code **)((int)*param_2 + 0x40);
    (**ppcVar1)(0x1028,param_2);
    param_1 = DX_REG;
  }
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_1a06(param_1: u32,astruct_15 *param_2,i16 param_3,u16 param_4)

{
  char *pcVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar2;
  Struct57*in_EDX;
  u16 uVar5;
  u32 uVar6;
  u32 *puVar3;
  char *pcVar4;
  u16 in_stack_0000fe8e;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbc;
  i16 in_stack_0000ffee;
  u32 uVar1;

  uVar6 = pass1_1028_b58e(param_2);
  uVar5 = (param_1 >> 0x10);
  pcVar1 = pass1_1010_b038(*(u8 **)((int)param_1 + 0x6e),uVar6,in_EDX,(u8 *)0x1770,
                           in_stack_0000ffee);
  iVar2 = pass1_1000_3e2c(CONCAT22((int)in_EDX,pcVar1));
  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22((int)param_2,0x32),in_stack_0000fe8e,
                           in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = ((u32)puVar3 >> 0x10);
  uVar3 = pass1_1010_7818((u32)puVar3,param_2);
  uVar1 = (u32)((int)param_1 + 0x6e);
  pcVar4 = string_op_1010_ada6(uVar2,uVar1,((u32)uVar1 >> 0x10),iVar2,uVar3);
  str_op_1008_60e8(((u32)pcVar4 >> 0x10),pcVar4);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 pass1_1010_1a66(param_1: u32,astruct_15 *param_2)

{
  u32 uVar2;
  u8 uVar3;
  u16 uVar4;
  BOOL16 BVar4;
  astruct_15 *uVar5;
  u16 uVar6;
  u32 uVar7;
  u32 uVar1;

  uVar5 = (astruct_15 *)((u32)param_2 >> 0x10);
  if ((((int)param_2 + 0x1c) != 0x2) || ((((int)param_2 + 0x1e) & 0xff) != 0x0)) {
    uVar7 = pass1_1028_b58e((astruct_15 *)((u32)param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
    uVar6 = (param_1 >> 0x10);
    uVar1 = (u32)((int)param_1 + 0x6e);
    pass1_1010_c2d8(uVar1,((u32)uVar1 >> 0x10),uVar7);
    if (((int)uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0)) {
      uVar2 = (u32)((int)param_1 + 0x6e);
      uVar4 = pass1_1010_b028(uVar2,((u32)uVar2 >> 0x10),param_2);
      BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uVar4,0x5);
      if ((BVar4 == 0x0) && (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uVar4,0x6), BVar4 == 0x0)) {
        uVar3 = '\0';
      }
      else {
        uVar3 = '\x01';
      }
      return uVar3;
    }
  }
  return '\0';
}



u16 * pass1_1010_1b04(u16 *param_1,u8 param_2)

{
  pass1_1010_0f76((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_19 * pass1_1010_1b6e(StructD *param_1,astruct_19 *param_2,astruct_19 *param_3,u16 param_4)

{
  u16 unaff_BP;
  u32 *puVar1;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  pass1_1010_0f24((u8 *)param_1,(astruct_19 *)CONCAT22(param_3,param_2),param_4);
  (u32)&param_2[0x1].field_0xe = 0x0;
  CONCAT22(param_3,param_2) = 0x1d04;
  param_2->segment_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba((Struct57*)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  &param_2[0x1].field_0xe = (int)puVar1;
  param_2[0x1].field8_0x10 = ((u32)puVar1 >> 0x10);
  return (astruct_19 *)CONCAT22(param_3,param_2);
}



void pass1_1010_1bb4(u16 param_1,u32 *param_2)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  u16 DX_REG;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    if ((param_1 | paVar2) == 0x0) break;
    ppcVar1 = (code **)((int)*param_2 + 0x40);
    (**ppcVar1)(0x1028,param_2);
    param_1 = DX_REG;
  }
  return;
}



void pass1_1010_1c16(param_1: u32,astruct_15 *param_2,param_3: i16)

{
  char *pcVar1;
  astruct_15 *uVar3;
  astruct_15 *uVar2;
  u32 uVar4;

  uVar4 = pass1_1028_b58e(param_2);
  uVar3 = (astruct_15 *)((u32)uVar4 >> 0x10);
  uVar2 = uVar3;
  pcVar1 = pass1_1010_b038(*(u8 **)((int)param_1 + 0x6e),uVar4,uVar3,(u8 *)0x178a,param_3);
  str_op_1008_60e8(uVar2,(char *)CONCAT22(uVar2,pcVar1));
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 pass1_1010_1c40(param_1: u32,astruct_15 *param_2)

{
  u32 uVar4;
  u16 uVar3;
  BOOL16 BVar5;
  astruct_15 *uVar5;
  u16 uVar6;
  u32 uVar7;
  u8 uVar2;
  u32 uVar1;

  uVar5 = (astruct_15 *)((u32)param_2 >> 0x10);
  if ((((int)param_2 + 0x1c) != 0x2) || ((((int)param_2 + 0x1e) & 0xff) != 0x0)) {
    uVar7 = pass1_1028_b58e((astruct_15 *)((u32)param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
    uVar6 = (param_1 >> 0x10);
    uVar1 = (u32)((int)param_1 + 0x6e);
    pass1_1010_c2d8(uVar1,((u32)uVar1 >> 0x10),uVar7);
    if (((int)uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0)) {
      uVar4 = (u32)((int)param_1 + 0x6e);
      uVar3 = pass1_1010_b028(uVar4,((u32)uVar4 >> 0x10),param_2);
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x11);
      if ((BVar5 == 0x0) && (BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x12), BVar5 == 0x0)) {
        uVar2 = '\0';
      }
      else {
        uVar2 = '\x01';
      }
      return uVar2;
    }
  }
  return '\0';
}



u16 * pass1_1010_1cde(u16 *param_1,u8 param_2)

{
  pass1_1010_0f76((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_19 * struct_op_1010_1d48(astruct_19 *param_1,u16 param_2)

{
  astruct_19 *iVar1;
  astruct_19 *uVar1;

  uVar1 = (astruct_19 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_19 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->segment_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x8 = param_2;
  param_1->offset_0x0 = 0x2014;
  iVar1->segment_0x2 = 0x1010;
  return param_1;
}



void pass1_1010_1d80(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->address_offset_field_0x0 = 0x2014;
  iVar4->field1_0x2 = 0x1010;
  pass1_1010_1f62((Struct27 *)((u32)param_1 & 0xffff | ZEXT24(uVar4) << 0x10),0x1);
  puVar1 = iVar4->field2_0x4;
  uVar2 = iVar4->field3_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar4->field1_0x2 = 0x1008;
  return;
}



u16 pass1_1010_1dce(void)

{
  return 0x0;
}



u16 pass1_1010_1dd4(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_1dda(u32 param_1)

{
  pass1_1010_209e(_u16_1050_0ed0,((int)param_1 + 0x8));
  return;
}



void pass1_1010_1df2(astruct_242 *param_1,u16 param_2,param_3: u32,u16 param_4,Struct57*param_5)

{
  code **ppcVar1;
  astruct_241 *in_AX;
  u16 uVar2;
  u16 uVar3;
  astruct_242 *iVar3;
  u16 uVar4;
  u16 *puStack10;
  u16 *puStack6;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_242 *)param_1;
  if (iVar3->field4_0x4 == NULL) {
    mem_op_1000_179c(0xc,param_5);
    uVar2 = param_5;
    uVar3 = uVar2 | in_AX;
    param_5 = (Struct57*)((u32)param_5 & 0xffff0000 | (u32)uVar3);
    if (uVar3 == 0x0) {
      iVar3->field4_0x4 = NULL;
    }
    else {
      set_struct_1008_574a((Struct57*)CONCAT22(uVar2,in_AX));
      *(astruct_241 **)&iVar3->field4_0x4 = in_AX;
      ((int)&iVar3->field4_0x4 + 0x2) = (int)param_5;
    }
  }
  mem_op_1000_179c(0xa,param_5);
  uVar2 = param_5;
  puStack10 = (u16 *)CONCAT22(uVar2,in_AX);
  if ((uVar2 | in_AX) == 0x0) {
    puStack6 = NULL;
  }
  else {
    *puStack10 = 0x389a;
    in_AX->field2_0x2 = 0x1008;
    in_AX->field3_0x4 = param_3;
    in_AX->field4_0x8 = param_2;
    *puStack10 = 0x2010;
    in_AX->field2_0x2 = 0x1010;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)((int)*iVar3->field4_0x4 + 0x4);
  (**ppcVar1)(0x1000,iVar3->field4_0x4,puStack6);
  return;
}



void pass1_1010_1ea6(param_1: u32,StructD *param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u32 *puVar4;
  u8 *puVar5;
  u16 DX_REG;
  astruct_498 *iVar6;
  u16 uVar6;
  u8 local_c [0x4];
  u32 uStack8;
  u16 uStack4;

  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_498 *)param_1;
  if (iVar6->field4_0x4 == NULL) {
    return;
  }
  uStack4 = 0x0;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)iVar6->field4_0x4);
  while( true ) {
    puVar5 = local_c;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
    if ((DX_REG | puVar5) == 0x0) break;
    if (*(StructD **)(puVar5 + 0x4) == param_2) {
      uStack4 = 0x1;
      ppcVar3 = (code **)((int)*iVar6->field4_0x4 + 0xc);
      (**ppcVar3)(0x1008);
      uStack8 = 0x0;
    }
  }
  puVar4 = iVar6->field4_0x4;
  if (((int)puVar4 + 0x8) == 0x0) {
    // WARNING: Load size is inaccurate
    puVar1 = iVar6->field4_0x4;
    uVar2 = ((int)&iVar6->field4_0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(0x1008,puVar1,uVar2,0x1,puVar1,uVar2,puVar1,uVar2);
    }
    iVar6->field4_0x4 = NULL;
  }
  return;
}




u16 * pass1_1010_1fbe(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_1fea(u16 *param_1,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

StructD * pass1_1010_2024(StructD *param_1)

{
  u16 struct_1_hi;

  struct_1_hi = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x124) = 0x0;
  _u16_1050_0ed0 = param_1;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff | (u32)struct_1_hi << 0x10),NULL,0x124);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_2050(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 uVar4;
  i16 iStack4;

  pass1_1010_2816(param_1);
  iStack4 = 0x0;
  do {
    uVar4 = (param_1 >> 0x10);
    puVar1 = (u32 *)(iStack4 * 0x4 + (int)param_1);
    uVar2 = (iStack4 * 0x4 + (int)param_1 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x49);
  _u16_1050_0ed0 = 0x0;
  return;
}



void pass1_1010_209e(param_1: u32,u16 param_2)

{
  pass1_1010_2816(param_1);
  ((int)param_1 + 0x124) = param_2;
  return;
}






u16 pass1_1010_286c(void)

{
  u16 *puVar1;

  pass1_1008_3e54((u16 *)&PTR_LOOP_1048_0000,0x0,0x5,0x12c);
  pass1_1008_3e54((u16 *)(s__1050_65a0 + 0x6),0x0,0x9b,0x20);
  pass1_1008_3e54((u16 *)0x105065ac,0x0,0xf5,0x3f);
  pass1_1008_3e54((u16 *)0x105065b2,0x0,0x114,0x4a);
  pass1_1008_3e54((u16 *)0x105065b8,0x0,0x135,0x45);
  pass1_1008_3e54((u16 *)0x105065be,0x0,0xf5,0x7b);
  puVar1 = pass1_1008_3e54((u16 *)0x105065c4,0x0,0x117,0x91);
  return puVar1;
}



void pass1_1010_28e6(param_1: u32,u16 param_2,astruct_19 *param_3,astruct_19 *param_4,u16 param_5)

{
  u32 uVar1;
  u16 uVar2;
  u16 puVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 unaff_DS;
  i16 iStack6;

  struct_op_1018_4cda((astruct_19 *)CONCAT22(param_4,param_3),param_5);
  uVar5 = 0x0;
  (u32)&param_3->field15_0x1c = 0x0;
  param_3->field17_0x20 = 0x0;
  param_3->field18_0x22 = 0x0;
  param_3->field19_0x24 = 0x0;
  &param_3->field20_0x26 = 0x0;
    // 0x2bdc
  CONCAT22(param_4,param_3) = 0x2be4;
  param_3->segment_0x2 = 0x1010;
  *(astruct_19 **)&PTR_LOOP_1050_4230 = param_3;
  *(astruct_19 **)0x4232 = param_4;
  pass1_1018_4dce((u8 *)param_1,(astruct_19 *)CONCAT22(param_4,param_3),0x56);
  uVar2 = FUN_1010_830a(uVar5,param_1,0x1018,*(astruct_87 **)&u32_1048_14cc,0x4);
  param_3->field15_0x1c = uVar2;
  param_3->field16_0x1e = param_1;
  if (*(i32 *)0x5f2c == 0x0) {
    puVar2 = mem_op_1000_160a((StructD *)param_1);
  }
  else {
    puVar2 = 0x5f2c;
    param_1 = param_1 & 0xffff0000 | (u32)0x5f2e;
  }
  uVar4 = fn_ptr_op_1000_1708(0x40,0x0,0x1,puVar2,param_1);
  ((int)&param_3->field20_0x26 + 0x2) = uVar4;
  param_3->field21_0x2a = param_1;
  for (iStack6 = 0x0; iStack6 < 0x10; iStack6 += 0x1) {
    uVar5 = FUN_1010_830a(iStack6 + 0x56,param_1,0x1000,*(astruct_87 **)&u32_1048_14cc,iStack6 + 0x56);
    uVar1 = (u32)((int)&param_3->field20_0x26 + 0x2);
    uVar7 = ((u32)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    (iVar6 + iStack6 * 0x4) = uVar5;
    (iVar6 + iStack6 * 0x4 + 0x2) = (int)param_1;
  }
  return;
}



void pass1_1010_29c6(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  StructD *iVar5;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = (int)s_add16_wav_1050_2bdc + 0x8;
  iVar5->address_offset_field_0x2 = 0x1010;
  if (*(i32 *)&iVar5->field_0x1c != 0x0) {
    puVar1 = &iVar5->field_0x1c;
    uVar2 = &iVar5->field_0x1e;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (u32)&iVar5->field_0x1c = 0x0;
    fn_ptr_1000_17ce(*(char **)&iVar5->field_0x28);
    (u32)&iVar5->field_0x28 = 0x0;
  }
  clenaup_win_ui_1018_4d22(param_1);
  return;
}



// WARNING: Instruction at (ram,0x10104b2b) overlaps instruction at (ram,0x10104b2a)
//
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

u16 FUN_1010_2a32(u8 *buffer_param_2,param_2: u32,HFILE16 *hfile_param,u16 param_4)

{
  i16 *piVar1;
  char *pcVar2;
  u8 *pbVar3;
  u8 bVar4;
  u32 *puVar5;
  u32 *puVar6;
  code **ppcVar7;
  code *pcVar8;
  u32 uVar9;
  u16 uVar10;
  HDC16 HVar11;
  HPALETTE16 HVar12;
  u16 uVar13;
  u16 uVar14;
  BOOL16 BVar15;
  i16 iVar16;
  u8 bVar17;
  u8 *puVar18;
  u8 *puVar19;
  u16 uVar20;
  u16 uVar21;
  u32 in_EDX;
  u32 *in_BX;
  u16 unaff_BP;
  i16 unaff_SI;
  i16 iVar23;
  i16 iVar24;
  u16 unaff_ES;
  u16 uVar25;
  u8 bVar26;
  bool bVar27;
  DEVMODEA *init_data;
  i16 in_stack_00000000;
  u16 in_stack_00000002;
  u16 uStack54;
  u16 *puStack46;
  u16 uStack42;
  u8 *read_buffer_38;
  u8 *read_buffer_22;
  HGDIOBJ16 HStack30;
  HGDIOBJ16 HStack28;
  u8 in_stack_0000ffec;
  u8 uVar28;
  u8 in_stack_0000ffed;
  u8 uVar29;
  u8 uVar30;
  u8 uVar31;
  u8 uVar32;
  u8 uVar33;
  u8 uVar34;
  u8 uVar35;
  u8 uVar36;
  u8 uVar37;
  u8 uVar38;
  Struct57*paVar22;

  uVar28 = (u8)unaff_BP;
  uVar29 = (u8)(unaff_BP >> 0x8);
  bVar26 = 0x0;
  uVar38 = 0x0;
  if ((param_2 + 0xec76 & 0x3) != 0x0) goto LAB_1010_2ad8;
  uVar10 = param_2 + 0xec76 >> 0x1;
  if (0x1c < uVar10) goto LAB_1010_2ad8;
  uVar14 = in_EDX;
  switch(uVar10) {
  default:
    goto switchD_1010_2ab5_caseD_0;
  case 0x1:
  case 0x3:
  case 0xb:
    (uVar10 + 0xa) = in_BX;
  case 0x9:
  case 0xf:
  case 0x15:
  case 0x1b:
    (uVar10 + 0xa) = in_BX;
    (uVar10 + 0x10) = in_BX;
    (uVar10 + 0xc) = in_BX;
    return uVar10;
  case 0x5:
    BVar15 = write_to_file_1008_7e1c
                       ((u8 *)param_2,ZEXT24(in_BX),
                        (char *)CONCAT13((char)(in_stack_00000000 >> 0x8),
                                         CONCAT12((char)in_stack_00000000,unaff_BP)),
                        CONCAT11(in_stack_0000ffed,in_stack_0000ffec));
    if (BVar15 != 0x0) {
      return 0x1;
    }
    u16_1050_0310 = 0x6d0;
    return 0x0;
  case 0x6:
    bVar26 = 0x0;
    goto LAB_1010_2ad8;
  case 0x7:
    ppcVar7 = (code **)*in_BX;
    (**ppcVar7)();
    iVar16 = (int)(u32 *)param_2 + 0x105;
    puVar18 = (u8 *)in_EDX;
    pass1_1010_8170(puVar18,_u16_1050_14cc,iVar16);
    iVar23 = (int)(u32 *)param_2 * 0x4;
    ((int)buffer_param_2 + iVar23 + 0x26) = iVar16;
    *(u8 **)((int)buffer_param_2 + iVar23 + 0x28) = puVar18;
    uVar36 = 0x50;
    uVar37 = 0x10;
    uVar34 = 0x80;
    uVar35 = 0x13;
    uVar30 = 0x0;
    uVar31 = 0x0;
    uVar32 = 0x0;
    uVar33 = 0x0;
    uVar28 = 0x0;
    uVar29 = 0x0;
    init_data = (DEVMODEA *)pass1_1008_4772(*(Struct76 **)((int)buffer_param_2 + 0x26 + iVar23));
    uVar25 = ((u32)init_data >> 0x10);
    HVar11 = CreateDC16(init_data,(char *)CONCAT13(uVar29,CONCAT12(uVar28,uVar25)),
                        (char *)CONCAT13(uVar33,CONCAT12(uVar32,CONCAT11(uVar31,uVar30))),
                        (char *)CONCAT13(uVar37,CONCAT12(uVar36,CONCAT11(uVar35,uVar34))));
    uVar28 = (u8)HVar11;
    uVar29 = (u8)(HVar11 >> 0x8);
    HVar12 = palette_op_1008_4e08
                       ((HPALETTE16)&stack0xffec,uVar25,*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),
                        (HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&stack0xffec)));
    HStack28 = SelectObject16(CONCAT11(uVar38,bVar26),CONCAT11(uVar29,uVar28));
    HStack30 = SelectObject16(CONCAT11(uVar29,uVar28),CONCAT11(uVar29,uVar28));
    for (read_buffer_22 = 0x0; piVar1 = (i16 *)((int)buffer_param_2 + 0x74),
        *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1; read_buffer_22 += 0x1) {
      iVar16 = (read_buffer_22 * 0x10 + (int)(u32 *)param_2) * 0x8;
      uVar25 = ((int)buffer_param_2 + 0x72);
      uVar13 = pass1_1000_484c(CONCAT13(0x10,CONCAT12(0x50,&stack0xfff2)),
                               CONCAT13((char)(uVar25 >> 0x8),
                                        CONCAT12((char)uVar25,iVar16 + ((int)buffer_param_2 + 0x70))),0x8);
      if (uVar13 != 0x0) {
        uVar9 = (u32)((int)buffer_param_2 + 0x70);
        uVar25 = ((u32)uVar9 >> 0x10);
        iVar23 = (int)uVar9;
        iVar24 = iVar16 + iVar23;
        Rectangle16(*(INT16 *)(iVar24 + 0x6),*(INT16 *)(iVar24 + 0x4),*(INT16 *)(iVar24 + 0x2),
                    *(INT16 *)(iVar23 + iVar16),CONCAT11(uVar29,uVar28));
      }
    }
    HVar12 = SelectPalette16(0x0,HVar12,CONCAT11(uVar29,uVar28));
    DeleteObject16(HVar12);
    SelectObject16(HStack28,CONCAT11(uVar29,uVar28));
    SelectObject16(HStack30,CONCAT11(uVar29,uVar28));
    uVar38 = 0x38;
    uVar30 = 0x15;
    DeleteDC16(CONCAT11(uVar29,uVar28));
    BVar15 = DeleteObject16(CONCAT11(uVar30,uVar38));
    return BVar15;
  case 0x8:
    bVar26 = 0x3;
    goto LAB_1010_2ad8;
  case 0xd:
    pbVar3 = (u8 *)(uVar10 + unaff_SI);
    bVar26 = *pbVar3;
    bVar4 = *pbVar3 + (u8)in_EDX;
    *pbVar3 = bVar4 + (uVar10 < 0x1c);
    puVar5 = (u32 *)(CARRY1(bVar26,(u8)in_EDX) || CARRY1(bVar4,uVar10 < 0x1c));
    puVar6 = in_BX + -0x404;
    bVar26 = in_BX < (u32 *)0x1010 || puVar6 < puVar5;
    uVar21 = (int)puVar6 - (int)puVar5;
    pcVar8 = (code *)swi(0x4);
    if (SBORROW2((int)in_BX,0x1010) != SBORROW2((int)puVar6,(int)puVar5)) {
      (*pcVar8)();
    }
    puVar19 = (u8 *)in_EDX;
    bVar27 = uVar21 < 0x1010 || uVar21 + 0xeff0 < bVar26;
    pbVar3 = (u8 *)(uVar10 + unaff_SI);
    bVar26 = *pbVar3;
    bVar17 = (u8)in_EDX;
    bVar4 = *pbVar3;
    *pbVar3 = bVar4 + bVar17 + bVar27;
    pcVar2 = (char *)(uVar10 + unaff_SI);
    *pcVar2 = *pcVar2 + bVar17 + (CARRY1(bVar26,bVar17) || CARRY1(bVar4 + bVar17,bVar27));
    struct_op_1018_4cda((astruct_19 *)CONCAT13((char)in_stack_00000000,CONCAT12(uVar29,CONCAT11(uVar28,uVar38))),
                        CONCAT11((char)in_stack_00000002,(char)(in_stack_00000000 >> 0x8)));
    iVar16 = CONCAT11(uVar28,uVar38);
    uVar30 = (u8)in_stack_00000000;
    piVar1 = (i16 *)CONCAT13(uVar30,CONCAT12(uVar29,iVar16));
    *piVar1 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
    (iVar16 + 0x2) = 0x1010;
    pass1_1018_4dce(puVar19,(astruct_19 *)CONCAT13(uVar30,CONCAT12(uVar29,iVar16)),0x1b3);
    PTR_LOOP_1050_4230 = CONCAT13(uVar30,CONCAT12(uVar29,CONCAT11(uVar28,uVar38)));
    return CONCAT11(uVar28,uVar38);
  case 0xe:
    ((int)buffer_param_2 + 0x20) = (u32 *)param_2;
    break;
  case 0x11:
    do {
    // WARNING: Do nothing block with infinite loop
    } while( true );
  case 0x12:
    PTR_LOOP_1050_10c6 = (u8 *)(0x0 < (int)(u32 *)param_2);
    PTR_LOOP_1050_1142 = (u8 *)(0x2 < (int)(u32 *)param_2);
    break;
  case 0x13:
    iVar16 = (int)buffer_param_2 * 0x8 + in_stack_00000000;
    if ((((iVar16 + 0x22) != 0x0) || ((iVar16 + 0x24) != 0x0)) ||
       (((iVar16 + 0x26) != 0x0 || ((iVar16 + 0x28) != 0x0)))) {
      HStack28 = 0x1010;
      HStack30 = 0x627c;
      sys_1000_3f9c(*(char **)(in_stack_00000000 + 0xe),s__d__d__d__d_1050_14ae,
                    (u32)((int)buffer_param_2 * 0x8 + in_stack_00000000 + 0x22));
      HStack28 = 0x1000;
      HStack30 = 0x62a0;
      in_BX = (u32 *)
              WritePrivateProfileString16
                        (*(char **)(in_stack_00000000 + 0xa),*(char **)(in_stack_00000000 + 0xe),
                         *(char **)((int)buffer_param_2 * 0x4 + 0x1446),s_windows_1050_13b8);
    }
    return (u16)in_BX;
  case 0x14:
    ((int)buffer_param_2 + 0x24) = (u32 *)param_2;
    break;
  case 0x17:
    uVar21 = uVar14 - 0x1;
    paVar22 = (Struct57*)(in_EDX & 0xffff0000 | (u32)uVar21);
    pbVar3 = (u8 *)(uVar10 + unaff_SI);
    *pbVar3 = *pbVar3 | (u8)uVar21;
    ((int)buffer_param_2 + 0x12) = in_BX;
    ((int)buffer_param_2 + 0x14) = uVar21;
    uStack42 = 0x0;
    while( true ) {
      if (uStack54 <= uStack42) {
        uVar38 = (u8)((u32)buffer_param_2 >> 0x10);
        BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                     (u8 *)(((u32)buffer_param_2 & 0xff00) << 0x10 |
                                           (u32)CONCAT12(uVar38,(int)buffer_param_2 + 0x1a)),0x2);
        if (((BVar15 != 0x0) &&
            (BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                          (u8 *)(((u32)buffer_param_2 & 0xff00) << 0x10 |
                                                (u32)CONCAT12(uVar38,(int)buffer_param_2 + 0x1c)),0x2), BVar15 != 0x0)
            ) && (BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                               (u8 *)(((u32)buffer_param_2 & 0xff00) << 0x10 |
                                                     (u32)CONCAT12(uVar38,(int)buffer_param_2 + 0x1e)),0x2),
                 BVar15 != 0x0)) {
          return 0x1;
        }
        u16_1050_0310 = 0x6d2;
        return 0x0;
      }
      uVar10 = uStack54;
      mem_op_1000_179c(0x8,paVar22);
      uVar21 = paVar22;
      puStack46 = (u16 *)CONCAT22(uVar21,uVar10);
      paVar22 = (Struct57*)((u32)paVar22 & 0xffff0000 | (u32)(uVar21 | uVar10));
      if ((uVar21 | uVar10) == 0x0) {
        read_buffer_38 = NULL;
      }
      else {
        *puStack46 = 0x389a;
        (uVar10 + 0x2) = 0x1008;
        *puStack46 = 0xa1c4;
        (uVar10 + 0x2) = 0x1010;
        read_buffer_38 = (u8 *)puStack46;
      }
      BVar15 = read_file_1008_7dee((HFILE16 *)param_2,(u8 *)CONCAT13(0x10,CONCAT12(0x50,&read_buffer_22)),0x2);
      if ((BVar15 == 0x0) ||
         (BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                       (u8 *)((u32)read_buffer_38 & 0xff000000 |
                                             (u32)CONCAT12((char)((u32)read_buffer_38 >> 0x10),
                                                             (int)read_buffer_38 + 0x6)),0x2), BVar15 == 0x0)) break;
      iVar16 = switch_1008_73ea((u32 *)param_2,param_2,(int)read_buffer_22);
      ((int)read_buffer_38 + 0x4) = iVar16;
      ppcVar7 = (code **)((int)(u32)(u32)((int)buffer_param_2 + 0x12) + 0x4);
      (**ppcVar7)();
      uStack42 += 0x1;
    }
    if (read_buffer_38 == NULL) {
      u16_1050_0310 = 0x6d2;
      return 0x0;
    }
    ppcVar7 = (code **)(u32)read_buffer_38;
    (**ppcVar7)();
    u16_1050_0310 = 0x6d2;
    return 0x0;
  case 0x18:
    bVar26 = 0x2;
    goto LAB_1010_2ad8;
  case 0x19:
    uVar14 = pass1_1010_6ca2(uVar14,(u32)buffer_param_2,0x8);
    uVar20 = in_EDX;
    if (uVar14 != 0x0) {
      pass1_1010_715c(uVar14,uVar20,CONCAT22(0x1a,(int)buffer_param_2),0x1a);
      buffer_param_2 = (u8 *)0x1a001a;
    }
    if ((u32 *)param_2 == (u32 *)0x2c) {
      pass1_1010_715c(uVar14,uVar20,CONCAT22(0x1d,(int)buffer_param_2),0x1d);
    }
    uVar14 = pass1_1010_6ca2(uVar20,0x5a,0x2);
    if (uVar14 != 0x0) {
      pass1_1010_715c(uVar14,uVar20,0x1c005a,0x1c);
    }
    return 0x1;
  case 0x1a:
    ((int)buffer_param_2 + 0x26) = (u32 *)param_2;
  }
  bVar26 = 0x1;
LAB_1010_2ad8:
  if ((bVar26 == 0x1) || (bVar26 == 0x2)) {
    if (bVar26 == 0x1) {
      param_2 = (u32)(((int)buffer_param_2 + 0x20) + ((int)buffer_param_2 + 0x22) +
                            ((int)buffer_param_2 + 0x24) + ((int)buffer_param_2 + 0x26));
    }
    if ((u32 *)param_2 != NULL) {
      uVar10 = (int)(u32 *)param_2 / 0x2 + 0x1;
      if (0x5 < (int)uVar10) {
        uVar10 = 0x5;
      }
      param_2 = (u32)uVar10;
      if ((bVar26 == 0x1) && (PTR_LOOP_1050_10c6 != NULL)) {
        if (0x4 < (int)uVar10) {
          uVar10 = 0x4;
        }
        param_2 = (u32)uVar10;
      }
    }
  }
  (bVar26 * 0x7c + 0xed6) = (u32 *)param_2;
  in_BX = (u32 *)param_2;
  pass1_1010_1f62((Struct27 *)buffer_param_2,0xc);
switchD_1010_2ab5_caseD_0:
  return (u16)in_BX;
}



void pass1_1010_2b50(u16 param_1,u16 param_2,u16 *param_3)

{
  pass1_1008_3f62(param_3,(u16 *)&PTR_LOOP_1048_0000);
  return;
}



u32 pass1_1010_2b66(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x1e),((int)param_1 + 0x1c));
}



void pass1_1010_2b78(u16 param_1,u16 param_2,i16 param_3,u32 param_4)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  i16 iVar4;
  u32 *puVar5;

  puVar3 = (u32 *)(param_3 * 0x7c + 0xed4);
  puVar5 = (u32 *)param_4;
  for (iVar4 = 0x1f; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



Struct76 * pass1_1010_2b98(param_1: u32,param_2: i16)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar1 = (u32)((int)param_1 + 0x28);
  uVar3 = ((u32)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  return (Struct76 *)
         CONCAT22((param_2 * 0x4 + iVar2 + -0x156),(param_2 * 0x4 + iVar2 + -0x158));
}



void pass1_1010_2bb9(void)

{
  pass1_1010_286c();
  return;
}



StructD * pass1_1010_2bbe(StructD *param_1,u8 param_2)

{
  pass1_1010_29c6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_2bfc(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  ((int)param_1 + 0xc) = 0x0;
  ((int)param_1 + 0xe) = 0x0;
  ((int)param_1 + 0x10) = 0x0;
  param_1->offset_0x0 = 0x2cc2;
  ((int)param_1 + 0x2) = 0x1010;
  return &param_1->offset_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * unk_load_str_op_1010_2c34(u32 *param_1)

{
  char *in_buffer_4;
  u16 *pUVar1;
  short in_buf_len_5;
  short sVar2;
  Struct57*in_EDX;
  Struct57*paVar3;
  u32 *puVar4;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000fff6;

  puVar4 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff6,0x3),in_stack_0000fe9e,
                           in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
  paVar3 = (Struct57*)((u32)in_EDX & 0xffff0000 | (u32)puVar4 >> 0x10);
  in_buffer_4 = (char *)puVar4;
  mem_op_1000_179c(0x80,paVar3);
  in_buf_len_5 = (short)paVar3;
  sVar2 = in_buf_len_5;
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x80,in_buffer_4,in_buf_len_5);
  pUVar1 = pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),s__1050_11c8);
  pass1_1010_e964(sVar2);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),(char *)CONCAT22(sVar2,pUVar1));
  return in_buffer_4;
}



u16 * pass1_1010_2c9c(u16 *param_1,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1010_2cd2(astruct_19 *param_1,u16 param_2)

{
  u32 in_EDX;
  u16 uVar1;
  astruct_19 *paVar2;
  u32 *puVar3;
  u16 in_stack_0000fe82;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb0;
  i16 *piVar4;
  u16 uVar5;
  i16 *piVar6;
  u16 uVar7;
  i16 local_6;
  i16 local_4;

  uVar1 = ((u32)in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0x12) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  ((int)param_1 + 0x18) = 0x0;
  ((int)param_1 + 0x22) = 0x0;
  ((int)param_1 + 0x24) = 0x0;
  ((int)param_1 + 0x26) = 0x0;
  ((int)param_1 + 0x28) = 0x0;
  (u32)((int)param_1 + 0x52) = 0x0;
  (u32)((int)param_1 + 0x56) = 0x0;
  ((int)param_1 + 0x5a) = 0x0;
  ((int)param_1 + 0x5e) = 0x0;
  ((int)param_1 + 0x5c) = 0x0;
  param_1->offset_0x0 = 0x36da;
  ((int)param_1 + 0x2) = 0x1010;
  piVar6 = &local_4;
  uVar7 = SUB42(0x1050,0x0);
  piVar4 = &local_6;
  uVar5 = SUB42(0x1050,0x0);
  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(uVar1,(int)((u32)paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(piVar4,0x48),in_stack_0000fe82,in_stack_0000ffa6,in_stack_0000ffac,
                           in_stack_0000ffb0);
  pass1_1008_3e94((u16 *)((u32)puVar3 & 0xffff0000 | (u32)((int)puVar3 + 0xe)),(u16 *)CONCAT22(uVar5,piVar4),
                  (char *)CONCAT22(uVar7,piVar6));
  (u32)((int)param_1 + 0xe) = 0x19001db;
  ((int)param_1 + 0xa) = 0x140 - (((int)param_1 + 0xe) / 0x2 - local_4);
  ((int)param_1 + 0xc) = 0xf0 - (((int)param_1 + 0x10) / 0x2 - local_6);
  (u32)((int)param_1 + 0x1a) = 0xa006e;
  (u32)((int)param_1 + 0x1e) = 0xa012c;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x2a)),NULL,0x28);
  return;
}



void pass1_1010_2db2(astruct_455 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_455 *iVar5;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0x36da;
  iVar5->field1_0x2 = 0x1010;
  puVar1 = (u32 *)iVar5[0xa].field3_0x6;
  uVar2 = (iVar5 + 0xb)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar5[0xb].field2_0x4);
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1

u32 pass1_1010_2e02(param_1: u32,param_2: i16)

{
  astruct_163 *iVar2;
  u16 uVar2;
  u32 uVar1;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x5c) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x5c);
    uVar2 = ((u32)uVar1 >> 0x10);
    iVar2 = (astruct_163 *)uVar1;
    return CONCAT22((iVar2 + param_2 * 0x4 + 0x2),(iVar2 + param_2 * 0x4));
  }
  return 0x0;
}



void pass1_1010_2e30(param_1: u32,u16 param_2,u16 param_3,param_4: i16)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x5c) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x5c);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    (iVar2 + param_4 * 0x4) = param_2;
    (iVar2 + param_4 * 0x4 + 0x2) = param_3;
  }
  return;
}



void pass1_1010_2e5c(u16 param_1,u16 param_2,u32 param_3)

{
  u32 uStack12;

  uStack12 = param_3;
  if (param_3 == 0x0) {
    return;
  }
  for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
  }
  return;
}



void pass1_1010_2ee2(u32 *param_1)

{
  code **ppcVar1;
  i16 iVar2;
  u16 DX_REG;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;
  astruct_65 *paStack6;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x52) != 0x0) {
    return;
  }
  iVar2 = 0x0;
  (iVar3 + 0x28) = 0x0;
  uVar5 = *param_1;
  ppcVar1 = (code **)((int)uVar5 + 0x20);
  (**ppcVar1)();
  if (iVar2 == 0x0) {
    paStack6 = *(astruct_65 **)(iVar3 + 0x56);
  }
  else {
    ppcVar1 = (code **)((int)uVar5 + 0x14);
    (**ppcVar1)();
    paStack6 = (astruct_65 *)CONCAT22(DX_REG,iVar2);
    uVar5 = pass1_1010_2e02((u32)param_1,(iVar2 + 0x12));
    pass1_1010_35a4((int)(uVar5 >> 0x10),param_1,uVar5);
  }
  pass1_1010_32f4((Struct27 *)param_1,paStack6);
  pass1_1010_1f62((Struct27 *)param_1,0x8);
  if (*(i32 *)(iVar3 + 0x52) != 0x0) {
    return;
  }
  return;
}



// WARNING: Unable to use type for symbol uVar3

void unk_destroy_win_op_1010_2fa0(astruct_873 *param_1)

{
  astruct_872 **ppaVar1;
  u32 uVar2;
  astruct_873 *iVar3;
  u16 uVar4;
  astruct_872 *iStack4;
  u32 uVar3;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_873 *)param_1;
  iVar3->field39_0x28 = 0x0;
  iStack4 = NULL;
  while( true ) {
    ppaVar1 = &iVar3->field22_0x16;
    if (*ppaVar1 == iStack4 || (int)*ppaVar1 < (int)iStack4) break;
    uVar3 = (u32)(&iVar3->field_0x2a + (int)iStack4 * 0x4);
    DestroyWindow16(*(HWND16 *)((int)uVar3 + 0x18));
    iStack4 = iStack4 + 0x1;
  }
  iVar3->field22_0x16 = NULL;
  if ((iVar3->field82_0x54 | &iVar3->field_0x52) != 0x0) {
    iStack4 = NULL;
    do {
      uVar2 = (u32)&iVar3->field_0x52;
      if (*(i32 *)((int)uVar2 + (int)iStack4 * 0x4) != 0x0) {
        uVar2 = (u32)&iVar3->field_0x52;
        uVar2 = (u32)((int)uVar2 + (int)iStack4 * 0x4);
        DestroyWindow16(*(HWND16 *)((int)uVar2 + 0x18));
        uVar2 = (u32)&iVar3->field_0x52;
        (u32)((int)uVar2 + (int)iStack4 * 0x4) = 0x0;
      }
      iStack4 = iStack4 + 0x1;
    } while ((int)iStack4 < 0xa);
    fn_ptr_1000_17ce(*(char **)&iVar3->field_0x52);
    (u32)&iVar3->field_0x52 = 0x0;
  }
  return;
}



void unk_destroy_win_op_1010_305a(u16 param_1,Struct27 *param_2,i16 param_3,astruct_65 *param_4)

{
  astruct_874 **ppaVar1;
  i16 *piVar2;
  u32 UVar3;
  let mut lVar4: i32;
  u32 uVar5;
  bool bVar6;
  u16 uVar8;
  Struct27 *iVar4;
  i16 iVar9;
  Struct27 *uVar7;
  u16 uVar10;
  i16 iStack10;
  astruct_874 *paStack8;
  astruct_874 *paStack6;
  astruct_874 *iVar7;

  uVar8 = pass1_1040_c60e(param_4);
  uVar7 = (Struct27 *)((u32)param_2 >> 0x10);
  iVar4 = (Struct27 *)param_2;
  iVar4->field18_0x12 = uVar8;
  iVar4->field19_0x14 = 0x0;
  paStack6 = NULL;
  bVar6 = false;
  iVar4->field33_0x28 = 0x0;
  paStack8 = NULL;
  do {
    ppaVar1 = (astruct_874 **)&iVar4->field20_0x16;
    if (*ppaVar1 == paStack8 || (int)*ppaVar1 < (int)paStack8) {
LAB_1010_30ad:
      iVar7 = paStack6;
      if (bVar6) {
        while (paStack8 = iVar7 + 0x1, ppaVar1 = (astruct_874 **)&iVar4->field20_0x16,
              *ppaVar1 != paStack8 && (int)paStack8 <= (int)*ppaVar1) {
          UVar3 = (&iVar4->field36_0x2e)[(int)iVar7];
          DestroyWindow16(*(HWND16 *)((int)UVar3 + 0x18));
          (&iVar4->field36_0x2e)[(int)iVar7] = 0x0;
          iVar7 = paStack8;
        }
        iVar4->field20_0x16 = (int)(paStack6 + 0x1);
        pass1_1010_1f62(param_2,0x9);
      }
      else {
        iVar9 = iVar4->field20_0x16;
        (&iVar4->field34_0x2a)[iVar9 * 0x2] = param_4;
        (&iVar4->field35_0x2c)[iVar9 * 0x2] = ((u32)param_4 >> 0x10);
        iStack10 = 0xa;
        piVar2 = &iVar4->field20_0x16;
        *piVar2 = *piVar2 + 0x1;
        if (0x1 < iVar4->field20_0x16) {
          UVar3 = (&iVar4->field30_0x22)[iVar4->field20_0x16];
          iVar9 = (int)UVar3;
          uVar10 = (UVar3 >> 0x10);
          iStack10 = (iVar9 + 0x20) + (iVar9 + 0x24) + 0x8;
        }
        mov_update_win_1040_93aa(param_4,iStack10,iVar4->field23_0x1a);
      }
      if (!bVar6) {
        pass1_1010_1f62(param_2,0xa);
      }
      if (param_3 == 0x0) {
        if (iVar4->field69_0x52 != 0x0) {
          paStack8 = NULL;
          do {
            lVar4 = iVar4->field69_0x52;
            uVar10 = ((u32)lVar4 >> 0x10);
            iVar9 = (int)lVar4;
            if ((*(i32 *)(iVar9 + (int)paStack8 * 0x4) != 0x0) &&
               (*(astruct_65 **)(iVar9 + (int)paStack8 * 0x4) != param_4)) {
              lVar4 = iVar4->field69_0x52;
              uVar5 = (u32)((int)lVar4 + (int)paStack8 * 0x4);
              DestroyWindow16(*(HWND16 *)((int)uVar5 + 0x18));
            }
            lVar4 = iVar4->field69_0x52;
            (u32)((int)lVar4 + (int)paStack8 * 0x4) = 0x0;
            paStack8 = (astruct_874 *)((int)paStack8 + 0x1);
          } while ((int)paStack8 < 0xa);
        }
        pass1_1010_32da(param_2,param_4);
        pass1_1010_1f62(param_2,0x8);
      }
      return;
    }
    if (*(astruct_65 **)(&iVar4->field34_0x2a + (int)paStack8 * 0x2) == param_4) {
      bVar6 = true;
      paStack6 = paStack8;
      goto LAB_1010_30ad;
    }
    paStack8 = paStack8 + 0x1;
  } while( true );
}



void win_ui_op_1010_3202(Struct27 *param_1,param_2: i16)

{
  i16 *piVar1;
  let mut lVar2: i32;
  u32 uVar3;
  Struct27 *iVar3;
  u16 uVar4;
  i16 iStack4;

  iVar3 = (Struct27 *)param_1;
  uVar4 = ((u32)param_1 >> 0x10);
  if (param_2 == 0x0) {
    piVar1 = &iVar3->field33_0x28;
    *piVar1 = *piVar1 + -0xa;
    if (*piVar1 < 0x0) {
      iVar3->field33_0x28 = 0x0;
    }
  }
  else {
    piVar1 = &iVar3->field33_0x28;
    *piVar1 = *piVar1 + &iVar3->field_0x18;
  }
  if (iVar3->field69_0x52 != 0x0) {
    iStack4 = 0x0;
    do {
      lVar2 = iVar3->field69_0x52;
      if (*(i32 *)((int)lVar2 + iStack4 * 0x4) != 0x0) {
        lVar2 = iVar3->field69_0x52;
        uVar3 = (u32)((int)lVar2 + iStack4 * 0x4);
        DestroyWindow16(*(HWND16 *)((int)uVar3 + 0x18));
        lVar2 = iVar3->field69_0x52;
        (u32)((int)lVar2 + iStack4 * 0x4) = 0x0;
      }
      iStack4 += 0x1;
    } while (iStack4 < 0xa);
  }
  if (iVar3->field20_0x16 == 0x0) {
    pass1_1010_32f4(param_1,*(astruct_65 **)(iVar3 + 0x1));
  }
  else {
    pass1_1010_32da(param_1,*(astruct_65 **)(&iVar3->field_0x26 + iVar3->field20_0x16 * 0x4));
  }
  pass1_1010_1f62(param_1,0x8);
  return;
}



void pass1_1010_32c0(param_1: u32,u32 param_2)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  ((int)param_1 + 0x28) = 0x0;
  (u32)((int)param_1 + 0x12) = param_2;
  return;
}



void pass1_1010_32da(Struct27 *param_1,astruct_65 *param_2)

{
  pass1_1010_32f4(param_1,*(astruct_65 **)((int)param_2 + 0x42));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_32f4(Struct27 *param_1,astruct_65 *param_2)

{
  i16 *piVar1;
  u32 *puVar2;
  u32 uVar3;
  u32 uVar4;
  code **ppcVar5;
  u32 uVar6;
  let mut lVar7: i32;
  u16 uVar8;
  u16 uVar9;
  i16 iVar11;
  StructD *in_EDX;
  Struct27 *iVar10;
  i16 iVar12;
  i16 iVar13;
  u16 uVar14;
  u16 uVar15;
  u16 unaff_CS;
  u16 uVar16;
  i16 *piStack48;
  i16 iStack16;
  i16 iStack12;

  uVar14 = ((u32)param_1 >> 0x10);
  iVar10 = (Struct27 *)param_1;
  if (iVar10->field69_0x52 != 0x0) {
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce((char *)iVar10->field69_0x52);
    iVar10->field69_0x52 = 0x0;
    &iVar10->field_0x18 = 0x0;
  }
  uVar8 = param_2 | param_2;
  if ((param_2 != NULL) &&
     (ppcVar5 = (code **)((int)(u32)param_1 + 0x24), (**ppcVar5)(unaff_CS,param_1,param_2), uVar8 != 0x0)) {
    ppcVar5 = (code **)((int)(u32)param_2 + 0x4);
    (**ppcVar5)(unaff_CS,param_2);
    &iVar10->field_0x18 = uVar8;
    if (uVar8 != 0x0) {
      ((int)&iVar10->field30_0x22 + 0x2) = 0x0;
      &iVar10->field_0x26 = 0x0;
      piVar1 = (i16 *)&iVar10->field_0x18;
      *piVar1 = *piVar1 - iVar10->field33_0x28;
      if (0xa < &iVar10->field_0x18) {
        &iVar10->field_0x26 = 0x1;
        &iVar10->field_0x18 = 0xa;
      }
      if (0x1 < (int)iVar10->field33_0x28) {
        ((int)&iVar10->field30_0x22 + 0x2) = 0x1;
      }
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(in_EDX);
      }
      else {
        in_EDX = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
      }
      uVar16 = 0x1000;
      uVar9 = fn_ptr_op_1000_1708(0x28,0x0,0x1,PTR_LOOP_1050_5f2c,in_EDX);
      &iVar10->field69_0x52 = uVar9;
      ((int)&iVar10->field69_0x52 + 0x2) = (int)in_EDX;
      if ((((int)&iVar10->field69_0x52 + 0x2) | &iVar10->field69_0x52) != 0x0) {
        uVar3 = (u32)(param_2 + 0x8);
        iVar11 = &iVar10->field_0x10;
        iStack12 = 0x0;
        for (iStack16 = 0x0; piVar1 = (i16 *)&iVar10->field_0x18, *piVar1 != iStack16 && iStack16 <= *piVar1;
            iStack16 += 0x1) {
          uVar6 = iVar10->field69_0x52;
          uVar8 = (int)uVar6 + iStack16 * 0x4;
          uVar6 &= 0xffff0000;
          piStack48 = (i16 *)(uVar6 | uVar8);
          uVar4 = (u32)((iVar10->field33_0x28 + iStack16) * 0x4 + (int)uVar3);
          ppcVar5 = (code **)((int)(u32)param_1 + 0x1c);
          iVar13 = iStack16;
          (**ppcVar5)(uVar16,param_1,(int)uVar4,(char)((u32)uVar4 >> 0x10),&iVar10->field30_0x22);
          *piStack48 = iVar13;
          (uVar8 + 0x2) = in_EDX;
          lVar7 = iVar10->field69_0x52;
          uVar4 = (u32)((int)lVar7 + iStack16 * 0x4);
          iStack12 += ((int)uVar4 + 0x24) + 0x8;
          if (iVar11 + -0xa < iStack12) {
            uVar16 = 0x1008;
            debug_print_1008_6048(in_EDX,s_overflow_on_node__d_1050_11ca);
            &iVar10->field_0x18 = iStack16 + -0x1;
            &iVar10->field_0x26 = 0x1;
            lVar7 = iVar10->field69_0x52;
            uVar15 = ((u32)lVar7 >> 0x10);
            iVar13 = (int)lVar7;
            puVar2 = (u32 *)(iVar13 + iStack16 * 0x4);
            uVar8 = (iVar13 + iStack16 * 0x4 + 0x2);
            in_EDX = (StructD *)(u32)(uVar8 | puVar2);
            if ((uVar8 | puVar2) != 0x0) {
              ppcVar5 = (code **)*puVar2;
              (**ppcVar5)(0x1008,puVar2,(char)uVar8,0x1);
            }
            lVar7 = iVar10->field69_0x52;
            iVar13 = iStack16 * 0x4;
            (u32)((int)lVar7 + iVar13) = 0x0;
            if (0x0 < iStack16) {
              lVar7 = iVar10->field69_0x52;
              uVar15 = ((u32)lVar7 >> 0x10);
              iVar12 = (int)lVar7;
              puVar2 = (u32 *)(iVar12 + iVar13 + -0x4);
              uVar8 = (iVar12 + iVar13 + -0x2);
              in_EDX = (StructD *)(u32)(uVar8 | puVar2);
              if ((uVar8 | puVar2) != 0x0) {
                ppcVar5 = (code **)*puVar2;
                (**ppcVar5)(0x1008,puVar2,(char)uVar8,0x1);
              }
              lVar7 = iVar10->field69_0x52;
              (u32)(iStack16 * 0x4 + (int)lVar7 + -0x4) = 0x0;
            }
          }
        }
        &iVar10->field_0x20 = 0xa;
        uVar9 = &iVar10->field_0x1e;
        mov_update_win_1040_93aa(*(astruct_65 **)iVar10->field69_0x52,0xa,uVar9);
        for (iStack16 = 0x1; piVar1 = (i16 *)&iVar10->field_0x18, *piVar1 != iStack16 && iStack16 <= *piVar1;
            iStack16 += 0x1) {
          lVar7 = iVar10->field69_0x52;
          uVar3 = (u32)(iStack16 * 0x4 + (int)lVar7 + -0x4);
          iVar11 = (int)uVar3;
          uVar16 = ((u32)uVar3 >> 0x10);
          lVar7 = iVar10->field69_0x52;
          mov_update_win_1040_93aa
                    (*(astruct_65 **)((int)lVar7 + iStack16 * 0x4),
                     (iVar11 + 0x20) + (iVar11 + 0x24) + 0x8,uVar9);
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_35a4(u16 param_1,u32 *param_2,u32 param_3)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u32 *puVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  Struct57*paVar8;
  u16 uVar9;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffb0;
  u32 uStack12;
  u32 *puStack8;
  Struct57*paVar7;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  uVar9 = ((u32)param_2 >> 0x10);
  uVar2 = (u32)((int)param_2 + 0x56);
  uVar2 = (u32)((int)uVar2 + 0x8);
  puStack8 = ((int)uVar2 + ((int)param_2 + 0x5a) * 0x4);
  uStack12 = param_3;
  if (param_3 != 0x0) {
    uVar9 = 0x1000;
    mem_op_1000_179c(0x4a,paVar6);
    uVar3 = param_3;
    uVar5 = paVar6 | uVar3;
    paVar8 = (Struct57*)((u32)paVar6 & 0xffff0000);
    paVar7 = (Struct57*)((u32)paVar8 | (u32)uVar5);
    if (uVar5 == 0x0) {
      uVar3 = 0x0;
    }
    else {
      uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
      pass1_1040_c54a((astruct_65 *)(param_3 & 0xffff | (long)paVar6 << 0x10),0x1,puStack8,in_stack_0000ffb0,paVar7);
      paVar8 = paVar7;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x18);
    (**ppcVar1)(uVar9,param_2,0x1,uVar3,(int)paVar8);
    for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
      uVar2 = (u32)((int)puStack8 + 0x8);
      puStack8 = ((((u8)uStack12 & 0xf) - 0x1) * 0x4 + (int)uVar2);
      uVar9 = 0x1000;
      puVar4 = puStack8;
      mem_op_1000_179c(0x4a,paVar8);
      uVar3 = puVar4;
      uVar5 = paVar8 | uVar3;
      paVar6 = (Struct57*)((u32)(Struct57*)((u32)paVar8 & 0xffff0000) | (u32)uVar5);
      if (uVar5 == 0x0) {
        uVar3 = 0x0;
        paVar8 = (Struct57*)((u32)paVar8 & 0xffff0000);
      }
      else {
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        pass1_1040_c54a((astruct_65 *)((u32)puVar4 & 0xffff | (long)paVar8 << 0x10),0x1,puStack8,in_stack_0000ffa6,
                        paVar6);
        paVar8 = paVar6;
      }
      ppcVar1 = (code **)((int)*param_2 + 0x18);
      (**ppcVar1)(uVar9,param_2,0x1,uVar3);
    }
  }
  return;
}



void pass1_1010_3680(u16 param_1,u8 *param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6)

{
  u16 uVar1;
  u16 in_register_0000000a;
  Struct57*paVar2;
  u16 in_stack_0000ffc0;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x4a,paVar2);
  uVar1 = paVar2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1040_c54a((astruct_65 *)CONCAT22(paVar2,param_1),0x1,(u32 *)CONCAT22(param_6,param_5),
                    in_stack_0000ffc0,(u32)paVar2 & 0xffff0000 | (u32)uVar1);
    return;
  }
  return;
}



StructD * pass1_1010_36b4(StructD *param_1,u8 param_2)

{
  pass1_1010_2db2((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_3702(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  param_1->offset_0x0 = 0x37c4;
  ((int)param_1 + 0x2) = 0x1010;
  return &param_1->offset_0x0;
}



void pass1_1010_3730(u16 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x37c4;
  ((int)param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0xa));
  pass1_1010_1d80((StructD *)param_1);
  return;
}



u32 pass1_1010_375e(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0xc),((int)param_1 + 0xa));
}



void pass1_1010_3770(u16 param_1,astruct_477 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_477 *iVar3;
  astruct_477 *uVar2;

  uVar2 = (astruct_477 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_477 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field10_0xa);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field10_0xa = uVar1;
  iVar3->field11_0xc = param_1;
  return;
}



u16 * pass1_1010_379e(u16 *param_1,u8 param_2,u16 param_3)

{
  pass1_1010_3730(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_223 * pass1_1010_37d4(astruct_223 *param_1)

{
  u16 uVar1;

  struct_1010_383a(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x16) = 0x0;
  param_1->field0_0x0 = 0x3b3e;
  ((int)param_1 + 0x2) = 0x1010;
  return param_1;
}



void pass1_1010_3800(StructD *param_1)

{
  StructD *iVar2;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar2 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x3b3e;
  iVar2->address_offset_field_0x2 = 0x1010;
  if (*(i32 *)((int)&iVar2->field12_0x14 + 0x2) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(u32)((int)&iVar2->field12_0x14 + 0x2));
  }
  pass1_1010_3880(param_1);
  return;
}



void struct_1010_383a(astruct_223 *param_1)

{
  astruct_223 *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_223 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x8 = 0x0;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0x10 = 0x0;
  iVar1->field6_0x12 = 0x0;
  iVar1->field7_0x14 = 0x0;
  param_1->field0_0x0 = 0x3b5e;
  iVar1->field1_0x2 = 0x1010;
  return;
}



void pass1_1010_3880(StructD *param_1)

{
  i16 *piVar1;
  u32 *puVar2;
  u16 uVar3;
  code **ppcVar4;
  u32 uVar5;
  StructD *iVar6;
  i16 iVar7;
  StructD *uVar8;
  u16 uVar9;
  i16 iStack4;

  uVar8 = (StructD *)((u32)param_1 >> 0x10);
  iVar6 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x3b5e;
  iVar6->address_offset_field_0x2 = 0x1010;
  if (*(i32 *)&iVar6->field5_0x8 != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      piVar1 = (i16 *)&iVar6->field_0x10;
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      uVar5 = (u32)&iVar6->field5_0x8;
      uVar9 = ((u32)uVar5 >> 0x10);
      iVar7 = (int)uVar5;
      puVar2 = (u32 *)(iVar7 + iStack4 * 0x4);
      uVar3 = (iVar7 + iStack4 * 0x4 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      iStack4 += 0x1;
    }
    fn_ptr_1000_17ce(*(char **)&iVar6->field5_0x8);
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar6->address_offset_field_0x2 = 0x1008;
  return;
}



Struct57* struct_1010_38f8(u16 param_1,Struct57*param_2,astruct_240 *param_3,param_4: i16)

{
  u16 uVar1;
  astruct_240 *iVar2;
  astruct_240 *uVar2;
  Struct57*paVar2;

  if (param_4 != 0x0) {
    uVar1 = param_4 << 0x2;
    mem_op_1000_179c(uVar1,param_2);
    uVar2 = (astruct_240 *)((u32)param_3 >> 0x10);
    iVar2 = (astruct_240 *)param_3;
    iVar2->field8_0x8 = uVar1;
    iVar2->field9_0xa = (Struct57*)param_2;
    return (Struct57*)CONCAT22((Struct57*)param_2,iVar2->field8_0x8);
  }
  mem_op_1000_179c(0x1a,param_2);
  if ((param_2 | param_1) != 0x0) {
    paVar2 = (Struct57*)pass1_1010_37d4((astruct_223 *)CONCAT22(param_2,param_1));
    return paVar2;
  }
  return NULL;
}



void pass1_1010_394a(u16 param_1,u16 param_2,u16 param_3,u16 param_4,param_5: i16)

{
  u16 in_register_0000000a;
  Struct57*paVar1;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  if (param_5 != 0x0) {
    mem_op_1000_179c(param_5 << 0x2,paVar1);
    return;
  }
  mem_op_1000_179c(0x16,paVar1);
  if ((paVar1 | param_1) != 0x0) {
    struct_1010_383a((astruct_223 *)CONCAT22(paVar1,param_1));
    return;
  }
  return;
}



void pass1_1010_398e(u16 param_1,u32 *param_2,u16 param_3,u16 param_4,u32 param_5)

{
  i16 *piVar1;
  code **ppcVar2;
  u32 uVar3;
  u32 uVar4;
  i16 iVar5;
  u16 uVar6;
  u16 DX_REG;
  u16 DX_REG_00;
  i16 iVar7;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uStack12;
  u32 *puStack6;

  uVar9 = ((u32)param_2 >> 0x10);
  uVar3 = *param_2;
  ppcVar2 = (code **)((int)uVar3 + 0x8);
  (**ppcVar2)();
  puStack6 = (u32 *)CONCAT22(DX_REG,param_1);
  if ((DX_REG | param_1) == 0x0) {
    return;
  }
  (u32)(param_1 + 0xc) = param_5;
  iVar7 = (int)*puStack6;
  ppcVar2 = (code **)(iVar7 + 0xc);
  (**ppcVar2)();
  iVar5 = ((int)param_2 + 0x14);
  piVar1 = (i16 *)((int)param_2 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  ppcVar2 = (code **)(iVar7 + 0x10);
  (**ppcVar2)();
  ppcVar2 = (code **)(iVar7 + 0x4);
  (**ppcVar2)();
  if (iVar5 != 0x0) {
    ppcVar2 = (code **)((int)uVar3 + 0x8);
    iVar7 = iVar5;
    (**ppcVar2)();
    (param_1 + 0x8) = iVar7;
    (param_1 + 0xa) = DX_REG_00;
    PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + 0x1;
    uVar9 = DX_REG_00;
    for (uStack12 = 0x0; (int)uStack12 < iVar5; uStack12 += 0x1) {
      uVar6 = uStack12;
      pass1_1010_398e(uStack12,param_2,uStack12,(int)uStack12 >> 0xf,(u32)puStack6);
      uVar4 = (u32)(param_1 + 0x8);
      uVar10 = ((u32)uVar4 >> 0x10);
      iVar7 = (int)uVar4;
      iVar8 = uStack12 * 0x4;
      (iVar7 + iVar8) = uVar6;
      (iVar7 + iVar8 + 0x2) = uVar9;
      uVar4 = (u32)(param_1 + 0x8);
      if (*(i32 *)((int)uVar4 + iVar8) == 0x0) break;
    }
    PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + -0x1;
  }
  return;
}



u16 pass1_1010_3a86(u32 param_1)

{
  return ((int)param_1 + 0x10);
}



void pass1_1010_3a94(param_1: u32,u16 param_2)

{
  ((int)param_1 + 0x12) = param_2;
  return;
}



void FUN_1010_3aa6(void)

{
  return;
}



u32 pass1_1010_3aaa(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x6),((int)param_1 + 0x4));
}



u16 FUN_1010_3abc(void)

{
  return 0x0;
}



void pass1_1010_3ac2(param_1: u32,u16 param_2,u32 param_3)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  (u32)((int)param_1 + 0x16) = param_3;
  ((int)param_1 + 0x12) = param_2;
  return;
}



u32 pass1_1010_3adc(u32 param_1)

{
  u16 *puVar1;

  puVar1 = (u16 *)(u32)((int)param_1 + 0x16);
  return CONCAT22(((int)puVar1 + 0x2),*puVar1);
}



StructD * pass1_1010_3af2(StructD *param_1,u8 param_2)

{
  pass1_1010_3800(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1010_3b18(StructD *param_1,u8 param_2)

{
  pass1_1010_3880(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1010_3b7a(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x389a;
  ((int)param_1 + 0xc) = 0x1008;
  ((int)param_1 + 0xa) = 0x3aa8;
  ((int)param_1 + 0xc) = 0x1008;
  (u32)((int)param_1 + 0xe) = 0x0;
  ((int)param_1 + 0x12) = 0x0;
  ((int)param_1 + 0x14) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  param_1->offset_0x0 = 0x3d6a;
  ((int)param_1 + 0x2) = 0x1010;
  ((int)param_1 + 0xa) = 0x3d7a;
  ((int)param_1 + 0xc) = 0x1010;
  return;
}



void pass1_1010_3bde(astruct_455 *param_1,u16 param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 *puVar4;
  astruct_455 *iVar4;
  u16 uVar5;
  u16 *puStack14;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0x3d6a;
  iVar4->field1_0x2 = 0x1010;
  iVar4[0x1].field1_0x2 = 0x3d7a;
  iVar4[0x1].field2_0x4 = (u32 *)0x1010;
  puVar1 = (u32 *)iVar4[0x1].field3_0x6;
  uVar2 = (iVar4 + 0x2)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar4[0x1].field1_0x2;
  }
  puStack14 = (u16 *)CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1010_3c52(param_1: u32,param_2: u32,u16 param_3)

{
  u16 uVar1;
  code **ppcVar2;
  u32 *puVar3;
  u16 uVar4;
  astruct_274 *iVar4;
  u16 uVar6;
  u16 unaff_CS;
  u32 uVar5;

  uVar6 = (param_2 >> 0x10);
  iVar4 = (astruct_274 *)param_2;
  iVar4->field18_0x14 = param_3;
  puVar3 = iVar4->field14_0xe;
  uVar1 = iVar4->field15_0x10;
  uVar4 = uVar1 | puVar3;
  uVar5 = param_1 & 0xffff0000 | (u32)uVar4;
  if (uVar4 != 0x0) {
    ppcVar2 = (code **)*puVar3;
    puVar3 = (u32 *)(**ppcVar2)();
  }
  puVar3 = (u32 *)FUN_1010_830a(puVar3,uVar5,unaff_CS,_u16_1050_14cc,iVar4->field18_0x14);
  iVar4->field14_0xe = puVar3;
  iVar4->field15_0x10 = uVar5;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_3c9e(i32 param_1)

{
  Struct57*paVar1;
  u32 in_EDX;
  Struct57*paVar2;

  if (param_1 == 0x0) {
    paVar1 = NULL;
    paVar2 = (Struct57*)(in_EDX & 0xffff0000);
  }
  else {
    paVar2 = (Struct57*)(in_EDX & 0xffff0000 | (u32)param_1);
    paVar1 = (Struct57*)((int)param_1 + 0xa);
  }
  pass1_1008_9262(paVar1,paVar2,(int)_PTR_LOOP_1050_0388,((u32)_PTR_LOOP_1050_0388 >> 0x10),
                  (u32)((int)param_1 + 0x12),CONCAT22((int)paVar2,paVar1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_3cd0(i32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == 0x0) {
      iVar1 = 0x0;
      uVar2 = 0x0;
    }
    else {
      iVar1 = (int)param_1 + 0xa;
      uVar2 = param_1;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,(u32)((int)param_1 + 0x12),CONCAT22(uVar2,iVar1));
  }
  return;
}



void pass1_1010_3d0a(i16 param_1,u16 param_2,i16 param_3,u16 param_4)

{
  if (param_3 == 0x2) {
    pass1_1010_3cd0(CONCAT22(param_2,param_1 + -0xa));
    pass1_1010_1f62((Struct27 *)CONCAT22(param_2,param_1 + -0xa),0x2);
  }
  return;
}



StructD * pass1_1010_3d38(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xa));
  pass1_1010_3bde((astruct_455 *)param_1,0x1050);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1010_3d44(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1010_3bde((astruct_455 *)param_2,0x1050);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_3d82(param_1: u32,astruct_19 *param_2,astruct_19 *param_3,u16 param_4)

{
  INT16 IVar1;
  u16 uVar3;
  u32 uVar2;
  u16 unaff_CS;
  astruct_19 *paVar4;

  uVar3 = ((u32)param_1 >> 0x10);
  paVar4 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_3,param_2),param_4);
  uVar2 = CONCAT22(uVar3,(int)((u32)paVar4 >> 0x10));
  (u32)&param_2->horiz_res_0xa = 0x0;
  CONCAT22(param_3,param_2) = 0x3e2c;
  param_2->segment_0x2 = 0x1010;
  IVar1 = FUN_1010_830a((int)paVar4,uVar2,unaff_CS,_u16_1050_14cc,0x99);
  param_2->horiz_res_0xa = IVar1;
  param_2->ver_res_0xc = uVar2;
  return CONCAT22(param_3,param_2);
}



void pass1_1010_3dc8(astruct_455 *param_1,u16 param_2)

{
  u32 *puVar1;
  u32 *puVar2;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0x3e2c;
  iVar4->field1_0x2 = 0x1010;
  puVar1 = (u32 *)iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}



StructD * pass1_1010_3e06(StructD *param_1,u8 param_2)

{
  pass1_1010_3dc8((astruct_455 *)param_1,0x1050);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_3e3c(astruct_19 *param_1,u16 param_2,u16 param_3)

{
  u16 *puVar1;
  u16 uVar2;
  u32 in_EDX;
  u32 uVar3;
  astruct_19 *iVar1;
  astruct_19 *uVar1;
  u16 *puVar4;

  uVar2 = ((u32)in_EDX >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x6,param_2);
  uVar1 = (astruct_19 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_19 *)param_1;
  iVar1->field17_0x20 = 0x389a;
  iVar1->field18_0x22 = 0x1008;
  iVar1->field17_0x20 = 0x3aa8;
  iVar1->field18_0x22 = 0x1008;
  iVar1->field19_0x24 = 0x0;
  (u32)((int)&iVar1[0x1].field2_0x4 + 0x2) = 0x0;
  iVar1[0x1].horiz_res_0xa = 0x4;
  (u32)&iVar1[0x1].ver_res_0xc = 0x0;
  (u32)&iVar1[0x1].field8_0x10 = 0x0;
  iVar1[0x1].field10_0x14 = 0x0;
  puVar4 = pass1_1008_3e54((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1[0x1].field11_0x16)),0x0,0x3,0x5);
  uVar3 = CONCAT22(uVar2,(int)((u32)puVar4 >> 0x10));
  (u32)&iVar1[0x1].field15_0x1c = 0x0;
    // 0x4a46
  param_1->offset_0x0 = &PTR_LOOP_1050_4a46;
  iVar1->segment_0x2 = 0x1010;
    // just 0x4a82
  iVar1->field17_0x20 = &PTR_LOOP_1050_4a82;
  iVar1->field18_0x22 = 0x1010;
  puVar1 = pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->field20_0x26)),NULL,0x40);
  uVar2 = FUN_1010_830a(puVar1,uVar3,0x1000,_u16_1050_14cc,0x1a1);
  ((int)&iVar1[0x1].field2_0x4 + 0x2) = uVar2;
  iVar1[0x1].field3_0x8 = uVar3;
  pass1_1018_4b78(param_1);
  return;
}



void pass1_1010_3f00(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 *puVar4;
  StructD *iVar5;
  StructD *uVar5;
  u16 *puStack16;
  i16 iStack4;

  uVar5 = (StructD *)((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = &PTR_LOOP_1050_4a46;
  iVar5->address_offset_field_0x2 = 0x1010;
  iVar5->field19_0x20 = &PTR_LOOP_1050_4a82;
  iVar5->field20_0x22 = 0x1010;
  iStack4 = 0x0;
  do {
    puVar1 = (&iVar5->field_0x26 + iStack4 * 0x4);
    uVar2 = (&iVar5->field_0x28 + iStack4 * 0x4);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x10);
  puVar1 = &iVar5->field_0x66;
  uVar2 = &iVar5->field_0x68;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar5->field_0x70);
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar5 = NULL;
  }
  else {
    puVar4 = &iVar5->field19_0x20;
  }
  puStack16 = (u16 *)CONCAT22(uVar5,puVar4);
  *puStack16 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



u16 FUN_1010_3fc2(u16 param_1,param_2: u32,u8 *param_3)

{
  BOOL16 BVar1;
  i16 iVar2;
  u16 uVar3;
  HFILE16 in_stack_0000ffda;
  u16 local_c [0x3];
  u16 local_6 [0x2];

  BVar1 = write_to_file_1008_7cac(param_3);
  if (BVar1 != 0x0) {
    uVar3 = ((u32)param_2 >> 0x10);
    iVar2 = (int)param_2;
    local_c[0] = (iVar2 + 0x24);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffda);
    if (BVar1 != 0x0) {
      local_6[0] = (iVar2 + 0x6a);
      BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffda);
      if (BVar1 != 0x0) {
        local_6[0] = (iVar2 + 0x7e);
        BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffda);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



void pass1_1010_404a(i16 param_1,param_2: u32,u32 param_3)

{
  i16 iVar1;
  BOOL16 BVar2;
  u16 local_4;

  read_file_1008_7cfe((int)param_3,(int)(param_3 >> 0x10),0x5);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    iVar1 = (int)param_2;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x24)),0x2);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 != 0x0) {
        BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x7e)),0x2);
        if (BVar2 != 0x0) {
          (iVar1 + 0x6a) = local_4;
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_40cc(i16 param_1,u16 param_2,u32 param_3)

{
  pass1_1030_8344(_u16_1050_5748,(u32)((int)param_3 + 0x6c));
  return CONCAT22((param_1 + 0xe),(param_1 + 0xc));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pt_in_rect_1010_40f8(Struct57*param_1,param_2: u32,POINT16 *param_3,u16 param_4)

{
  i16 *piVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  Struct57*paVar8;
  i16 iVar10;
  u16 uVar11;
  u32 *puVar12;
  u16 in_stack_0000fe94;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc2;
  u32 *puStack16;
  i16 iStack6;
  u16 uStack4;
  u32 uVar9;

  iStack6 = 0x0;
  uStack4 = 0x0;
  do {
    uVar11 = (param_2 >> 0x10);
    iVar10 = (int)param_2;
    piVar1 = (i16 *)(iVar10 + 0x74);
    if (*piVar1 == iStack6 || *piVar1 < iStack6) {
LAB_1010_413e:
      if ((uStack4 != 0x0) && (0x3 < (int)PTR_LOOP_1050_3960)) {
        puVar12 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,iStack6 + 0xcU),
                                  in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
        paVar8 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)puVar12 >> 0x10);
        uVar4 = pass1_1018_0afa((u32)puVar12);
        if (uVar4 == 0x0) {
          uVar11 = 0x1000;
          uVar5 = uVar4;
          mem_op_1000_179c(0xb4,paVar8);
          uVar6 = paVar8 | uVar5;
          uVar9 = (u32)paVar8 & 0xffff0000 | (u32)uVar6;
          if (uVar6 == 0x0) {
            iVar10 = 0x0;
            uVar7 = 0x0;
          }
          else {
            uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
            iVar10 = string_1040_8520(uVar9,(Struct57*)CONCAT22(paVar8,uVar5),HWND16_1050_0396,0x20030,
                                      0x6450643);
            uVar7 = uVar9;
          }
          puStack16 = (u32 *)CONCAT22(uVar7,iVar10);
          ppcVar2 = (code **)((int)*puStack16 + 0x74);
          (**ppcVar2)(uVar11,iVar10,uVar7);
          pass1_1010_209e(_u16_1050_0ed0,iStack6 + 0xcU);
          uStack4 = uVar4;
        }
      }
      if (uStack4 != 0x0) {
        return iStack6;
      }
      return -0x1;
    }
    param_1 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)(iVar10 + 0x72));
    BVar3 = PtInRect16(*param_3,(RECT16 *)((iStack6 * 0x10 + (iVar10 + 0x24)) * 0x8 + (iVar10 + 0x70)));
    if (BVar3 != 0x0) {
      uStack4 = 0x1;
      goto LAB_1010_413e;
    }
    iStack6 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_41d6(Struct57*param_1,astruct_243 *param_2,u32 param_3)

{
  u16 *puVar1;
  i16 *piVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u8 *puVar6;
  u16 uVar7;
  Struct57*paVar8;
  u32 uVar9;
  astruct_243 *iVar9;
  i16 iVar11;
  astruct_244 *iVar10;
  u16 unaff_SI;
  u16 uVar12;
  u16 uVar13;
  u32 *puVar14;
  u16 in_stack_0000fe6e;
  u16 in_stack_0000ff92;
  u16 in_stack_0000ff98;
  u16 in_stack_0000ff9c;
  i16 iStack50;
  i16 local_30;
  char *local_2e;
  i16 iStack42;
  char *pcStack40;
  char *pcStack34;
  char *pcStack30;
  i16 iStack26;
  u16 uStack24;
  i16 iStack22;
  char *pcStack20;
  u16 uStack16;
  u32 uStack14;
  u32 uStack10;
  i16 iStack6;
  u16 uStack4;

  uVar12 = ((u32)param_2 >> 0x10);
  iVar9 = (astruct_243 *)param_2;
  iVar9->field108_0x6c = param_3;
  puVar14 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe6e,
                            in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
  uVar9 = (u32)param_1 & 0xffff0000;
  iStack6 = (int)puVar14;
  uStack4 = ((u32)puVar14 >> 0x10);
  uStack10 = pass1_1010_ec40(iStack6,uStack4,iStack6,uStack4,iVar9->field108_0x6c);
  paVar8 = (Struct57*)(uVar9 & 0xffff0000 | uStack10 >> 0x10);
  iVar9->field112_0x74 = ((int)uStack10 + 0x22);
  if (*(i32 *)&iVar9->field_0x70 != 0x0) {
    pcStack34 = *(char **)&iVar9->field_0x70;
    pcStack30 = pcStack34;
    fn_ptr_1000_17ce(pcStack34);
    (u32)&iVar9->field_0x70 = 0x0;
  }
  iVar4 = iVar9->field112_0x74 << 0x7;
  mem_op_1000_179c(iVar4,paVar8);
  &iVar9->field_0x70 = iVar4;
  iVar9->field111_0x72 = (u8 *)paVar8;
  pass1_1030_8344(_u16_1050_5748,iVar9->field108_0x6c);
  uStack14 = CONCAT22((int)paVar8,iVar4);
  uStack16 = (**(i16 **)(iVar4 + 0x10) == 0x9);
  iStack22 = ((int)uStack10 + 0x22);
  uVar7 = iStack22 * 0x6;
  mem_op_1000_179c(uVar7,paVar8);
  uVar5 = paVar8;
  pcStack30 = (char *)CONCAT22(uVar5,uVar7);
  uVar9 = (u32)(uVar5 | uVar7);
  if ((uVar5 | uVar7) == 0x0) {
    pcStack20 = NULL;
  }
  else {
    pass1_1000_5586(0x3e38,0x1008,iStack22,0x6,uVar7,uVar5);
    pcStack20 = pcStack30;
  }
  uStack24 = 0x0;
  while( true ) {
    puVar6 = (u8 *)uVar9;
    uVar13 = (uStack10 >> 0x10);
    puVar1 = (u16 *)((int)uStack10 + 0x22);
    if (*puVar1 < uStack24 || *puVar1 == uStack24) break;
    uVar3 = (u32)((int)uStack10 + 0x24);
    uVar7 = uStack24;
    pass1_1028_e0a0(puVar6,_PTR_LOOP_1050_65e2,(u32)((int)uVar3 + uStack24 * 0x2) << 0x10);
    pcStack34 = (char *)CONCAT22(puVar6,uVar7);
    uVar9 = (u32)pcStack20 >> 0x10;
    pass1_1008_3f62((u16 *)((u32)pcStack20 & 0xffff0000 | (u32)(uStack24 * 0x6 + (int)pcStack20)),
                    (u16 *)CONCAT22(puVar6,uVar7 + 0x8));
    pcStack40 = pcStack34;
    pcStack30 = pcStack34;
    if (pcStack34 != NULL) {
      fn_ptr_1030_84d0((u32)pcStack34);
      fn_ptr_1000_17ce(pcStack34);
    }
    uStack24 += 0x1;
  }
  for (iStack26 = 0x0; piVar2 = &iVar9->field112_0x74, *piVar2 != iStack26 && iStack26 <= *piVar2; iStack26 += 0x1) {
    pass1_1008_3e94((u16 *)((u32)pcStack20 & 0xffff0000 | (u32)(iStack26 * 0x6 + (int)pcStack20)),
                    (u16 *)CONCAT22(0x1050,&local_2e),(char *)CONCAT22(0x1050,&local_30));
    iStack42 = pass1_1000_49b2(local_2e);
    iStack42 /= 0x5;
    if (0xc < iStack42) {
      iStack42 = 0xc;
      iVar4 = pass1_1000_49b2(local_2e);
      local_2e = (char *)((u32)local_2e & 0xffff0000 | (u32)(((int)local_2e / iVar4) * 0x3c));
    }
    iVar4 = pass1_1000_49b2(local_2e);
    uVar7 = iVar4 % 0x5;
    pcStack34 = (char *)((u32)pcStack34 & 0xffff0000 | (u32)uVar7);
    if ((int)local_2e < 0x0) {
      if (0x2 < (int)uVar7) {
        uVar7 -= 0x5;
      }
      local_2e = (char *)((u32)local_2e & 0xffff0000 | (u32)(local_2e + uVar7));
    }
    else if ((int)uVar7 < 0x3) {
      local_2e = (char *)((u32)local_2e & 0xffff0000 | (u32)(local_2e - uVar7));
    }
    else {
      local_2e = (char *)((u32)local_2e & 0xffff0000 | (u32)(local_2e + (0x5 - uVar7)));
    }
    iStack50 = local_30 / 0x16;
    for (iVar4 = 0x0; iVar4 < 0x10; iVar4 += 0x1) {
      if (0xf < iStack50) {
        iStack50 = 0x0;
      }
      if (((int)(uStack16 != 0x0) < iStack50) && (iStack50 < 0x8)) {
        iVar11 = ((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
        iVar10 = (astruct_244 *)((iStack26 * 0x10 + iVar4) * 0x8);
        uVar3 = (u32)&iVar9->field_0x70;
        (iVar10 + (int)uVar3) = iVar11 + 0x49;
        uVar3 = (u32)&iVar9->field_0x70;
        (iVar10 + (int)uVar3 + 0x2) = local_2e + 0x49;
        uVar3 = (u32)&iVar9->field_0x70;
        (iVar10 + (int)uVar3 + 0x4) = iVar11 + 0x4e;
        uVar3 = (u32)&iVar9->field_0x70;
        (iVar10 + (int)uVar3 + 0x6) = local_2e + 0x4e;
      }
      else {
        iVar11 = (iStack26 * 0x10 + iVar4) * 0x8;
        uVar3 = (u32)&iVar9->field_0x70;
        (iVar11 + (int)uVar3) = 0x0;
        uVar3 = (u32)&iVar9->field_0x70;
        ((int)uVar3 + iVar11 + 0x2) = 0x0;
        uVar3 = (u32)&iVar9->field_0x70;
        ((int)uVar3 + iVar11 + 0x4) = 0x1;
        uVar3 = (u32)&iVar9->field_0x70;
        ((int)uVar3 + iVar11 + 0x6) = 0x1;
      }
      iStack50 += 0x1;
    }
  }
  pcStack40 = pcStack20;
  local_2e = pcStack20;
  fn_ptr_1000_17ce(pcStack20);
  draw_1010_47ae((u32)param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_451a(u8 *param_1,u32 param_2)

{
  u16 uVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 *puVar3;
  u32 uVar4;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000fff6;

  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff6,0x2f),in_stack_0000fe9e,in_stack_0000ffc2,
                           in_stack_0000ffc8,in_stack_0000ffcc);
  uVar1 = ((u32)puVar3 >> 0x10);
  uVar4 = pass1_1010_ec40((int)puVar3,uVar1,(int)puVar3,uVar1,(u32)((int)param_2 + 0x6c));
  uVar2 = (uVar4 >> 0x10);
  return CONCAT22(((int)uVar4 + 0x4),((int)uVar4 + 0x2));
}



u32 pass1_1010_454a(u32 param_1)

{
  i16 iVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  iVar2 = (iVar1 + 0x24) * 0x4;
  return CONCAT22((iVar1 + iVar2 + 0x28),(iVar1 + iVar2 + 0x26));
}



void pass1_1010_4566(i16 param_1,u16 param_2,param_3: i16)

{
  if (param_3 != 0x2) {
    return;
  }
  pass1_1010_4956(CONCAT22(param_2,param_1 + -0x20));
  pass1_1010_1f62((Struct27 *)CONCAT22(param_2,param_1 + -0x20),0x2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_459e(Struct27 *param_1)

{
  Struct57*paVar1;
  u32 in_EDX;
  Struct57*paVar2;

  if (param_1 == NULL) {
    paVar1 = NULL;
    paVar2 = (Struct57*)(in_EDX & 0xffff0000);
  }
  else {
    paVar2 = (Struct57*)(in_EDX & 0xffff0000 | (u32)param_1);
    paVar1 = (Struct57*)((int)param_1 + 0x20);
  }
  pass1_1008_9262(paVar1,paVar2,(int)_PTR_LOOP_1050_0388,((u32)_PTR_LOOP_1050_0388 >> 0x10),0x1f4,
                  CONCAT22((int)paVar2,paVar1));
  ((int)param_1 + 0x7e) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_45d6(i32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 unaff_CS;
  i16 iStack4;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if ((iVar6 + 0x7e) != 0x0) {
    if (_PTR_LOOP_1050_0388 != 0x0) {
      if (param_1 == 0x0) {
        iVar4 = 0x0;
        uVar5 = 0x0;
      }
      else {
        iVar4 = iVar6 + 0x20;
        uVar5 = uVar7;
      }
      unaff_CS = 0x1008;
      pass1_1008_92b2(_PTR_LOOP_1050_0388,0x1f4,CONCAT22(uVar5,iVar4));
    }
    for (iStack4 = 0x0; iStack4 < 0x10; iStack4 += 0x1) {
      if ((iVar6 + 0x24) != iStack4) {
        puVar1 = (u32 *)(iVar6 + 0x26 + iStack4 * 0x4);
        uVar2 = (iVar6 + 0x26 + iStack4 * 0x4 + 0x2);
        if ((uVar2 | puVar1) != 0x0) {
          ppcVar3 = (code **)*puVar1;
          (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1);
        }
        (u32)(iVar6 + iStack4 * 0x4 + 0x26) = 0x0;
      }
    }
    (iVar6 + 0x7e) = 0x0;
  }
  return;
}



void pass1_1010_4674(Struct27 *param_1,i16 param_2,u16 param_3,u16 param_4)

{
  i16 *piVar1;
  Struct27 *paVar2;
  u16 uVar2;

  paVar2 = (Struct27 *)param_1;
  uVar2 = ((u32)param_1 >> 0x10);
  if (param_2 == 0x1) {
    piVar1 = (i16 *)((int)&paVar2->field30_0x22 + 0x2);
    *piVar1 = *piVar1 + 0x1;
    if (0xf < ((int)&paVar2->field30_0x22 + 0x2)) {
      ((int)&paVar2->field30_0x22 + 0x2) = 0x0;
    }
LAB_1010_469a:
    draw_op_1010_47d0(paVar2,uVar2,((int)&paVar2->field30_0x22 + 0x2));
  }
  else if (param_2 != 0x2) {
    if (param_2 != 0x3) {
      if ((paVar2[0x1].field19_0x14 != 0x0) && (paVar2[0x1].field19_0x14 != 0x4)) {
        pass1_1010_459e(param_1);
      }
      goto LAB_1010_46e8;
    }
    piVar1 = (i16 *)((int)&paVar2->field30_0x22 + 0x2);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      ((int)&paVar2->field30_0x22 + 0x2) = 0xf;
    }
    goto LAB_1010_469a;
  }
  pass1_1010_1f62(param_1,0x2);
  pass1_1010_45d6(param_1);
LAB_1010_46e8:
  paVar2[0x1].field19_0x14 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void get_sys_metrics_1010_46f6(param_1: u32,Struct57*param_2)

{
  INT16 IVar1;
  INT16 IVar2;
  i16 iVar3;
  u16 uVar4;
  u32 *puVar5;
  u32 uVar6;
  u16 in_stack_0000fe7c;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffaa;
  i16 *piVar7;
  u16 uVar8;
  char *pcVar9;
  i16 local_6;
  i16 local_4;

  pcVar9 = (char *)CONCAT22(0x1050,&local_4);
  piVar7 = &local_6;
  uVar8 = SUB42(0x1050,0x0);
  puVar5 = mixed_1010_20ba(param_2,_u16_1050_0ed0,(u8 **)CONCAT22(piVar7,0x48),in_stack_0000fe7c,
                           in_stack_0000ffa0,in_stack_0000ffa6,in_stack_0000ffaa);
  pass1_1008_3e94((u16 *)((u32)puVar5 & 0xffff0000 | (u32)((int)puVar5 + 0xe)),(u16 *)CONCAT22(uVar8,piVar7),pcVar9)
  ;
  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar6 = pass1_1008_4772(*(Struct76 **)(iVar3 + 0x66));
  uVar8 = (uVar6 >> 0x10);
  (iVar3 + 0x18) = local_4 + 0x8;
  (iVar3 + 0x1a) = local_6 + 0x9;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  (iVar3 + 0x1c) = IVar1 * 0x2 + ((int)uVar6 + 0x4);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  IVar2 = GetSystemMetrics16(SM_CYBORDER);
  (iVar3 + 0x1e) = IVar2 + IVar1 + ((int)uVar6 + 0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_4788(u16 param_1,u16 param_2,param_3: u32,char *param_4)

{
  pass1_1030_8344(_u16_1050_5748,(u32)((int)param_3 + 0x6c));
  pass1_1030_301a(param_2,CONCAT22(param_2,param_1),param_4);
  return;
}



void draw_1010_47ae(u32 param_1)

{
  u16 uStack4;

  uStack4 = 0x0;
  do {
    draw_op_1010_47d0((Struct27 *)param_1,(param_1 >> 0x10),uStack4);
    uStack4 += 0x1;
  } while ((int)uStack4 < 0x10);
  return;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1010_47d0(Struct27 *param_1,u16 param_2,u16 param_3)

{
  i16 *piVar1;
  u32 *puVar3;
  i16 iVar5;
  HPALETTE16 hpalette16_var6;
  HGDIOBJ16 handle;
  HGDIOBJ16 hgdiobj16_00;
  astruct_797 *iVar4;
  u16 uVar5;
  u8 *DX_REG;
  u8 *puVar5;
  i16 iVar7;
  astruct_739 *iVar8;
  u16 uVar11;
  u32 uVar12;
  i16 iStack32;
  HDC16 hdc16_var_1;
  u16 devmodea_init_data;
  u16 uStack16;
  u16 local_e;
  u16 uStack12;
  u16 uStack10;
  u16 uStack8;
  HGDIOBJ16 stock_obj_handle;
  HPEN16 pen_handle;
  u32 uVar4;
  u32 *puVar2;
  u16 uVar13;
  u8 uVar14;
  u8 uVar15;
  u16 uVar16;
  u16 uVar17;
  u16 offset_1;
  u16 base_addr_1;
  code **fn_ptr_1;

  pen_handle = CreatePen16(0x77d7fb,0x1,0x0);
  stock_obj_handle = GetStockObject16(HOLLOW_BRUSH);
  local_e = 0x0;
  uStack12 = 0x0;
  uStack10 = 0x1;
  uStack8 = 0x1;
  puVar3 = (&param_1->field_0x26 + param_3 * 0x4);
  puVar5 = (u8 *)(&param_1->field33_0x28)[param_3 * 0x2];
  if ((puVar5 | puVar3) != 0x0) {
    fn_ptr_1 = (code **)*puVar3;
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,puVar3,puVar5,0x1);
    puVar5 = DX_REG;
  }
  iVar5 = param_3 + 0x105;
  pass1_1010_8170(puVar5,_u16_1050_14cc,iVar5);
  iVar8 = (astruct_739 *)(param_3 * 0x4);
  (iVar8 + (int)(&param_1->field_0x0 + 0x26)) = iVar5;
  *(u8 **)(iVar8 + (int)(&param_1->field_0x0 + 0x28)) = puVar5;
  base_addr_1 = 0x1050;
  offset_1 = 0x1380;
  uVar16 = 0x0;
  uVar17 = 0x0;
  uVar13 = 0x0;
  uVar14 = '\0';
  uVar15 = '\0';
  uVar12 = pass1_1008_4772(*(Struct76 **)(&param_1->field_0x26 + (int)iVar8));
  uVar12 = (uVar12 >> 0x10);
  devmodea_init_data = uVar12;
  uStack16 = uVar12;
  hdc16_var_1 = CreateDC16((DEVMODEA *)(uVar12 & 0xffff | (u32)uVar12 << 0x10),
                           (char *)CONCAT13(uVar15,CONCAT12(uVar14,uVar13)),(char *)CONCAT22(uVar17,uVar16),
                           (char *)CONCAT22(base_addr_1,offset_1));
  hpalette16_var6 =
       palette_op_1008_4e08
                 ((HPALETTE16)&hdc16_var_1,uVar12,*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),
                  (HDC16 *)CONCAT22(0x1050,&hdc16_var_1));
  handle = SelectObject16(pen_handle,hdc16_var_1);
  hgdiobj16_00 = SelectObject16(stock_obj_handle,hdc16_var_1);
  iStack32 = 0x0;
  while( true ) {
    piVar1 = (i16 *)&param_1[0x1].field_0x1e;
    if (*piVar1 == iStack32 || *piVar1 < iStack32) break;
    iVar4 = (astruct_797 *)((iStack32 * 0x10 + param_3) * 0x8);
    uVar5 = pass1_1000_484c(CONCAT22(0x1050,&local_e),
                            CONCAT22(&param_1[0x1].field_0x1c,iVar4 + param_1[0x1].field23_0x1a),0x8);
    if (uVar5 != 0x0) {
      uVar4 = (u32)&param_1[0x1].field23_0x1a;
      uVar11 = ((u32)uVar4 >> 0x10);
      iVar7 = (i16)uVar4;
      Rectangle16(*(INT16 *)(iVar4 + iVar7 + 0x6),*(INT16 *)(iVar4 + iVar7 + 0x4),*(INT16 *)(iVar4 + iVar7 + 0x2),
                  *(INT16 *)(iVar4 + iVar7),hdc16_var_1);
    }
    iStack32 += 0x1;
  }
  hpalette16_var6 = SelectPalette16(0x0,hpalette16_var6,hdc16_var_1);
  DeleteObject16(hpalette16_var6);
  SelectObject16(handle,hdc16_var_1);
  SelectObject16(hgdiobj16_00,hdc16_var_1);
  DeleteDC16(hdc16_var_1);
  DeleteObject16(pen_handle);
  return;
}



void pass1_1010_4956(u32 param_1)

{
  i16 *piVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = (iVar3 + 0x6a);
  if (iVar2 == 0x0) {
    piVar1 = (i16 *)(iVar3 + 0x24);
    *piVar1 = *piVar1 + 0x1;
    if (0xf < (iVar3 + 0x24)) {
      (iVar3 + 0x24) = 0x0;
      return;
    }
  }
  else {
    if (iVar2 != 0x4) {
      return;
    }
    piVar1 = (i16 *)(iVar3 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      (iVar3 + 0x24) = 0xf;
    }
  }
  return;
}



StructD * pass1_1010_4994(u16 param_1,StructD *param_2,u8 param_3)

{
  param_2 = (StructD *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 - 0x20));
  pass1_1010_3f00(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u32 pass1_1010_49a0(i16 param_1,u16 param_2)

{
  return CONCAT22(param_2,param_1 + 0xa);
}



u32 pass1_1010_49b0(i16 param_1,u16 param_2)

{
  return CONCAT22(param_2,param_1 + 0x18);
}



u16 pass1_1010_49c0(u32 param_1)

{
  return ((int)param_1 + 0x14);
}



void pass1_1010_49ce(param_1: u32,u16 param_2)

{
  ((int)param_1 + 0x14) = param_2;
  return;
}



u16 pass1_1010_49e0(u32 param_1)

{
  return ((int)param_1 + 0x16);
}



void pass1_1010_49ee(param_1: u32,u16 param_2)

{
  ((int)param_1 + 0x16) = param_2;
  return;
}



void pass1_1010_4a00(param_1: u32,u16 param_2)

{
  ((int)param_1 + 0x12) = param_2;
  return;
}



u16 pass1_1010_4a12(u32 param_1)

{
  return ((int)param_1 + 0x12);
}



u16 * pass1_1010_4a20(u16 *param_1,u8 param_2)

{
  pass1_1010_3f00((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_4a8a(param_1: u32,astruct_19 *param_2,astruct_19 *param_3,u16 param_4,u16 param_5,
                    u16 param_6,u16 param_7,u16 param_8,u16 param_9)

{
  INT16 IVar1;
  u16 uVar3;
  Struct57*paVar2;
  u16 unaff_CS;
  astruct_19 *paVar4;
  u32 *puVar5;
  u16 uStack4;

  uVar3 = ((u32)param_1 >> 0x10);
  paVar4 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_3,param_2),param_4);
  paVar2 = (Struct57*)CONCAT22(uVar3,(int)((u32)paVar4 >> 0x10));
  (u32)&param_2->field11_0x16 = 0x0;
  (u32)&param_2->field_0x1a = 0x0;
  param_2->field16_0x1e = 0x0;
  param_2->field17_0x20 = 0x1;
  param_2->field18_0x22 = 0x0;
  param_2->field19_0x24 = 0x0;
  param_2->field20_0x26 = NULL;
  param_2->field21_0x2a = 0x0;
  param_2->field22_0x2c = 0x1;
  param_2->field23_0x2e = 0x0;
  param_2->field24_0x30 = 0x0;
    // just 0x5024
  param_2->field25_0x32 = 0x0;
    // just 0x5024
  CONCAT22(param_3,param_2) = (int)s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
  param_2->segment_0x2 = 0x1010;
  IVar1 = FUN_1010_830a(0x0,paVar2,unaff_CS,_u16_1050_14cc,0x1b3);
  param_2->field11_0x16 = IVar1;
  param_2->field12_0x18 = paVar2;
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(uStack4,0x3),param_6,param_7,param_8,param_9);
  &param_2->field20_0x26 = (int)puVar5;
  ((int)&param_2->field20_0x26 + 0x2) = (int)((u32)puVar5 >> 0x10);
  pass1_1008_4772(*(Struct76 **)&param_2->field11_0x16);
  &param_2->field_0xe = 0x13c;
  param_2->horiz_res_0xa = 0x0;
  param_2->field8_0x10 = 0x0;
  param_2->ver_res_0xc = 0x0;
  return;
}



void free_rsrc_1010_4b3e(StructD *param_1)

{
  u32 *puVar3;
  u32 uVar5;
  BOOL16 BVar6;
  StructD *pstructd_1;
  astruct_818 *iVar7;
  StructD *pstructd_1_hi;
  u16 uVar4;
  u16 unaff_CS;
  i16 iStack4;
  u32 *puVar2;
  i16 *piVar1;
  u16 uVar1;
  u32 *puVar1;
  u16 uVar2;
  code **fn_ptr_1;

  pstructd_1_hi = (StructD *)((u32)param_1 >> 0x10);
  pstructd_1 = (StructD *)param_1;
    // really just 0x5024
  param_1->address_offset_field_0x0 = (int)s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
  pstructd_1->address_offset_field_0x2 = 0x1010;
  if (&pstructd_1->field_0x2a != 0x0) {
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    BVar6 = GlobalUnlock16(*(HGLOBAL16 *)&pstructd_1->field_0x2a);
    if (BVar6 == 0x0) {
      unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
      FreeResource16(*(HGLOBAL16 *)&pstructd_1->field_0x2a);
    }
  }
  &pstructd_1->field_0x2a = 0x0;
  if (**(i32 **)&pstructd_1->field11_0x12 != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      puVar3 = &pstructd_1->field11_0x12;
      piVar1 = (i16 *)((int)puVar3 + 0x8);
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      uVar4 = ((u32)*puVar3 >> 0x10);
      iVar7 = (astruct_818 *)*puVar3;
      puVar2 = (iVar7 + iStack4 * 0x4);
      uVar1 = (iVar7 + iStack4 * 0x4 + 0x2);
      if ((uVar1 | puVar2) != 0x0) {
        fn_ptr_1 = (code **)*puVar2;
        (**fn_ptr_1)(unaff_CS,puVar2,uVar1,0x1);
      }
      iStack4 += 0x1;
    }
  }
  uVar5 = (u32)&pstructd_1->field11_0x12;
  fn_ptr_1000_17ce(*(char **)((int)uVar5 + 0x4));
  fn_ptr_1000_17ce(*(char **)&pstructd_1->field11_0x12);
  puVar1 = ((int)&pstructd_1->field12_0x14 + 0x2);
  uVar2 = pstructd_1->field13_0x18;
  if ((uVar2 | puVar1) != 0x0) {
    fn_ptr_1 = (code **)*puVar1;
    (**fn_ptr_1)(0x1000,puVar1,uVar2,0x1);
  }
  fn_ptr_1000_17ce(*(char **)&pstructd_1->field14_0x1a);
  pass1_1010_1d80(param_1);
  return;
}



u32 pass1_1010_4c2c(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x18),((int)param_1 + 0x16));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_4c3e(param_1: u32,i16 param_2,i16 param_3,u8 *param_4,u16 param_5)

{
  i16 *piVar1;
  u32 uVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  u32 uVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  u16 unaff_CS;
  u32 uVar9;
  i16 iStack14;
  u8 local_c [0x6];
  u16 uStack6;
  i16 iStack4;

  uVar4 = CONCAT22(in_register_0000000a,param_4);
  uVar7 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  pass1_1010_bffa(param_3,param_4,(u32)(iVar5 + 0x26));
  (iVar5 + 0x12) = param_3;
  (iVar5 + 0x14) = uVar4;
  if ((uVar4 | (iVar5 + 0x12)) != 0x0) {
    if (param_2 == 0x0) {
      uVar4 = (u32)(iVar5 + 0x12);
      (iVar5 + 0x30) = ((int)uVar4 + 0x8);
    }
    else {
      (iVar5 + 0x2e) = 0x1;
      uVar2 = (u32)(iVar5 + 0x12);
      uVar2 = (u32)((int)uVar2 + 0x4);
      iVar6 = ((int)uVar2 + 0x2);
      if ((iVar6 == 0x5) || (iVar6 == 0x6)) {
        (iVar5 + 0x30) = 0x1;
        (iVar5 + 0x20) = 0x0;
      }
      else {
        (iVar5 + 0x30) = 0x2;
        uVar2 = (u32)(u32)(iVar5 + 0x12);
        uVar2 = (u32)((int)uVar2 + 0x4);
        (u32)(iVar5 + 0x32) = uVar2;
        uVar3 = FUN_1010_830a((int)uVar2,uVar4,unaff_CS,_u16_1050_14cc,0x1bf);
        uVar2 = (u32)(u32)(iVar5 + 0x12);
        uVar8 = ((u32)uVar2 >> 0x10);
        iVar6 = (int)uVar2;
        (iVar6 + 0x4) = uVar3;
        (iVar6 + 0x6) = (int)uVar4;
      }
    }
    iStack4 = 0x14;
    pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
    uStack6 = 0x0;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = (i16 *)(iVar5 + 0x30);
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar4 = (u32)(u32)(iVar5 + 0x12);
      uVar9 = pass1_1008_4772(*(Struct76 **)((int)uVar4 + iStack14 * 0x4));
      iStack4 += (-(iStack14 == 0x0) & 0x5) + 0x14 + ((int)uVar9 + 0x4);
      iStack14 += 0x1;
    }
    if ((iVar5 + 0xe) < iStack4) {
      (iVar5 + 0xe) = iStack4;
    }
  }
  return;
}



// WARNING: This is an inlined function
// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar1

void struct_1010_4d5c(u8 *param_1,astruct_245 *param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6,param_7: i16)

{
  u32 uVar3;
  i16 iVar4;
  u16 in_register_0000000a;
  Struct57*paVar5;
  astruct_245 *iVar3;
  astruct_747 *iVar5;
  u16 uVar6;
  u32 uVar2;
  u32 uVar1;

  paVar5 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  uVar6 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_245 *)param_2;
  if (*(i32 *)&iVar3->field_0x1a == 0x0) {
    iVar4 = iVar3->field47_0x30 << 0x3;
    mem_op_1000_179c(iVar4,paVar5);
    &iVar3->field_0x1a = iVar4;
    iVar3->field28_0x1c = (u8 *)paVar5;
  }
  uVar2 = (u32)&iVar3->field_0x1a;
  iVar5 = (astruct_747 *)(param_7 * 0x8);
  (iVar5 + (int)uVar2) = param_6;
  uVar3 = (u32)&iVar3->field_0x1a;
  (iVar5 + (int)uVar3 + 0x2) = param_5;
  uVar1 = (u32)&iVar3->field_0x1a;
  (iVar5 + (int)uVar1 + 0x4) = param_4;
  uVar3 = (u32)&iVar3->field_0x1a;
  (iVar5 + (int)uVar3 + 0x6) = param_3;
  return;
}



u32 pass1_1010_4dc8(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x20) == 0x0) {
    return 0x0;
  }
  return CONCAT22((iVar1 + 0x1c),(iVar1 + 0x20) * 0x8 + (iVar1 + 0x1a));
}



void pass1_1010_4df0(u16 param_1,u32 param_2)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_2 >> 0x10);
  uVar1 = (u32)((int)param_2 + 0x26);
  pass1_1010_c1ba(param_1,uVar1,((u32)uVar1 >> 0x10),((int)param_2 + 0x20));
  return;
}



void pt_in_rect_1010_4e08(param_1: u32,u16 param_2,u16 param_3)

{
  i16 *piVar1;
  bool bVar2;
  BOOL16 BVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iStack12;
  i16 iStack10;
  POINT16 PStack8;

  PStack8 = (POINT16)CONCAT22(param_2,param_3);
  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  (iVar4 + 0x22) = (iVar4 + 0x20);
  bVar2 = false;
  (iVar4 + 0x24) = 0x0;
  iStack12 = 0x0;
  iStack10 = 0x0;
  do {
    piVar1 = (i16 *)(iVar4 + 0x30);
    if (*piVar1 == iStack12 || *piVar1 < iStack12) {
LAB_1010_4e67:
      if (iStack10 != 0x0) {
        (iVar4 + 0x20) = iStack10;
      }
      if (bVar2) {
        return;
      }
      return;
    }
    BVar3 = PtInRect16(PStack8,(RECT16 *)((int)(u32)(iVar4 + 0x1a) + iStack12 * 0x8));
    if (BVar3 != 0x0) {
      iStack10 = iStack12;
      bVar2 = true;
      goto LAB_1010_4e67;
    }
    iStack12 += 0x1;
  } while( true );
}



void pass1_1010_4e8c(u32 param_1)

{
  pass1_1010_1f62((Struct27 *)param_1,0xd);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2

void find_n_load_rsrc_1010_4e9e(astruct_812 *struct_param_1)

{
  BOOL16 BVar1;
  HRSRC16 h_rsrc;
  HGLOBAL16 handle;
  astruct_812 *struct_1;
  u16 uVar3;
  u32 uVar1;
  u32 uVar2;

  uVar3 = ((u32)struct_param_1 >> 0x10);
  struct_1 = (astruct_812 *)struct_param_1;
  if (struct_1->field29_0x20 != 0x0) {
    if (struct_1->hglobal_0x2a != 0x0) {
      BVar1 = GlobalUnlock16(struct_1->hglobal_0x2a);
      if (BVar1 == 0x0) {
        FreeResource16(struct_1->hglobal_0x2a);
      }
    }
    uVar1 = struct_1->field18_0x12;
    uVar2 = (u32)((int)uVar1 + 0x4);
    h_rsrc = FindResource16((char *)0xa,
                            (char *)(u32)(((int)uVar2 + struct_1->field29_0x20 * 0x2) * 0x2 + 0x1384)
                            ,HINSTANCE16_1050_038c);
    handle = LoadResource16(h_rsrc,HINSTANCE16_1050_038c);
    struct_1->hglobal_0x2a = handle;
    if (handle != 0x0) {
      WIN16_LockResource16(handle);
      return;
    }
  }
  return;
}



u16 pass1_1010_4f20(u16 param_1,u16 param_2,param_3: i16)

{
  return (param_3 * 0x2 + 0x139a);
}



void pass1_1010_4f30(u16 param_1,u16 param_2,u16 *param_3,u16 *param_4)

{
  *param_4 = 0xa;
  *param_3 = 0x73;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_4f48(astruct_482 *param_1)

{
  u16 uVar1;
  code **ppcVar2;
  u32 *puVar3;
  u32 *puVar4;
  u32 in_EDX;
  u16 uVar6;
  u32 uVar5;
  astruct_482 *iVar6;
  astruct_483 *iVar7;
  astruct_482 *uVar7;
  u16 uVar8;
  u16 unaff_CS;

  uVar6 = ((u32)in_EDX >> 0x10);
  uVar7 = (astruct_482 *)((u32)param_1 >> 0x10);
  iVar6 = (astruct_482 *)param_1;
  puVar3 = iVar6->field18_0x12;
  iVar6->field39_0x30 = ((int)puVar3 + 0x8);
  if (iVar6->field40_0x32 != 0x0) {
    uVar5 = *iVar6->field18_0x12;
    uVar8 = ((u32)uVar5 >> 0x10);
    iVar7 = (astruct_483 *)uVar5;
    puVar3 = iVar7->field4_0x4;
    iVar7->field4_0x4 = (u32 *)iVar6->field40_0x32;
    if (puVar3 != NULL) {
      ppcVar2 = (code **)*puVar3;
      (**ppcVar2)();
    }
    iVar6->field40_0x32 = 0x0;
  }
  puVar4 = iVar6->field19_0x16;
  uVar1 = iVar6->field20_0x18;
  uVar5 = CONCAT22(uVar6,uVar1);
  if ((uVar1 | puVar4) != 0x0) {
    ppcVar2 = (code **)*puVar4;
    (**ppcVar2)();
  }
  puVar4 = (u32 *)FUN_1010_830a(puVar4,uVar5,unaff_CS,_u16_1050_14cc,0x1b3);
  iVar6->field19_0x16 = puVar4;
  iVar6->field20_0x18 = uVar5;
  fn_ptr_1000_17ce((char *)iVar6->field21_0x1a);
  iVar6->field21_0x1a = 0x0;
  iVar6->field38_0x2e = 0x0;
  return;
}



u16 * pass1_1010_5004(StructD *param_1,u8 param_2,u16 param_3)

{
  free_rsrc_1010_4b3e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return &param_1->address_offset_field_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_503e(u8 *param_1,astruct_19 *param_2,u16 param_3)

{
  struct_op_1018_4cda(param_2,param_3);
    // just 0x5099
    // 0x1010:509a = ptr to fn ptr in table
  param_2->offset_0x0 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
  ((int)param_2 + 0x2) = 0x1010;
  pass1_1018_4dce(param_1,param_2,0x1b3);
  PTR_LOOP_1050_4230 = param_2;
  return;
}



StructD * pass1_1010_5074(StructD *param_1,u8 param_2)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1010_50b2(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xc) = 0x0;
  ((int)param_1 + 0x10) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  (u32)((int)param_1 + 0x16) = 0x0;
  param_1->offset_0x0 = 0x53f4;
  ((int)param_1 + 0x2) = 0x1010;
  return;
}



void pass1_1010_50f2(u16 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x53f4;
  ((int)param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0xc));
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_5120(u16 param_1,u16 param_2,param_3: u32,u16 param_4)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;
  u16 uVar5;
  u16 uVar6;
  i16 iVar7;
  i16 iVar8;
  u16 uVar9;

  uVar9 = (param_3 >> 0x10);
  iVar8 = (int)param_3;
  if (*(i32 *)(iVar8 + 0x16) != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar8 + 0x16));
    uVar5 = param_2 | param_1;
    if (uVar5 != 0x0) {
      uVar1 = (u32)(param_1 + 0x1f6);
      uVar4 = uVar1;
      pass1_1030_38f2(uVar1,0x3);
      uVar2 = uVar4;
      uVar6 = uVar5;
      uVar3 = uVar2;
      pass1_1030_38f2(uVar1,0x4);
      iVar7 = uVar6 + uVar5 + CARRY2(uVar3,uVar2);
      if ((0x0 < iVar7) || ((-0x1 < iVar7 && (param_4 <= uVar3 + uVar2)))) {
        (iVar8 + 0xa) = param_4;
        return;
      }
    }
  }
  return;
}



void pass1_1010_519a(u8 *param_1,param_2: u32,char *param_3)

{
  u16 uVar1;
  u32 uVar2;
  u16 uVar3;
  astruct_92 *paVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  astruct_246 *iVar5;
  astruct_247 *iVar6;
  i16 iVar7;
  u16 uVar8;
  u16 uVar9;
  i16 *piStack44;
  astruct_92 local_18;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x400);
  uVar8 = (param_2 >> 0x10);
  iVar5 = (astruct_246 *)param_2;
  iVar5->field14_0x10 = (int)local_18.field5_0xc;
  fn_ptr_1000_17ce(*(char **)&iVar5->field12_0xc);
  uVar3 = iVar5->field14_0x10 << 0x2;
  mem_op_1000_179c(uVar3,paVar6);
  iVar5->field12_0xc = uVar3;
  iVar5->field13_0xe = (u8 *)paVar6;
  iVar5->field14_0x10 = 0x0;
  while( true ) {
    uVar5 = paVar6;
    paVar4 = &local_18;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
    paVar6 = (Struct57*)(u32)(uVar5 | paVar4);
    if ((uVar5 | paVar4) == 0x0) break;
    if (paVar4[0x1c].field4_0x8 != 0x8000002) {
      uVar1 = ((int)&paVar4->field3_0x4 + 0x2);
      paVar6 = (Struct57*)(u32)uVar1;
      uVar2 = (u32)&iVar5->field12_0xc;
      uVar9 = ((u32)uVar2 >> 0x10);
      iVar7 = (int)uVar2;
      iVar6 = (astruct_247 *)(iVar5->field14_0x10 * 0x4);
      piStack44 = (i16 *)(param_2 & 0xffff0000 | ZEXT24(&iVar5->field14_0x10));
      (iVar6 + iVar7) = &paVar4->field3_0x4;
      (iVar6 + iVar7 + 0x2) = uVar1;
      *piStack44 = *piStack44 + 0x1;
    }
  }
  param_3 = iVar5->field14_0x10;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 string_1010_5286(char *param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  short in_buf_len_5;
  u16 in_register_0000000a;
  Struct57*paVar1;
  char *pcVar2;
  u32 UStack10;
  char *pcStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  pcStack6 = (char *)CONCAT22(param_2,param_1);
  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2 | param_1);
  if ((param_2 | param_1) == 0x0) {
    return 0x0;
  }
  mem_op_1000_179c(0x80,paVar1);
  in_buf_len_5 = (short)paVar1;
  UStack10 = CONCAT22(in_buf_len_5,param_1);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x80,param_1,in_buf_len_5);
  pass1_1000_3cea(UStack10,(char *)0x105013ac);
  pcVar2 = pass1_1038_4d28(pcStack6);
  pass1_1000_3cea(UStack10,pcVar2);
  return CONCAT22(in_buf_len_5,param_1);
}



void pass1_1010_52fc(u16 param_1,u8 *param_2,param_3: u32,u32 param_4)

{
  u16 uVar1;

  pass1_1010_533c(param_2,param_3,(char *)param_4);
  uVar1 = (param_3 >> 0x10);
  ((int)param_3 + 0x12) = param_1;
  *(u8 **)((int)param_3 + 0x14) = param_2;
  return;
}



void pass1_1010_531c(u16 param_1,u8 *param_2,param_3: u32,u32 param_4)

{
  u16 uVar1;

  pass1_1010_533c(param_2,param_3,(char *)param_4);
  uVar1 = (param_3 >> 0x10);
  ((int)param_3 + 0x16) = param_1;
  *(u8 **)((int)param_3 + 0x18) = param_2;
  return;
}



void pass1_1010_533c(u8 *param_1,param_2: u32,char *param_3)

{
  u16 *puVar1;
  u32 uVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  char *pcVar7;
  u16 uStack6;
  char local_4 [0x2];

  pass1_1010_519a(param_1,param_2,(char *)CONCAT22(0x1050,local_4));
  uStack6 = 0x0;
  while( true ) {
    uVar6 = (param_2 >> 0x10);
    uVar5 = param_2;
    puVar1 = (u16 *)(uVar5 + 0x10);
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar3 = (u32)(uVar5 + 0xc);
    uVar2 = (u32)((int)uVar3 + uStack6 * 0x4);
    pcVar7 = (char *)string_1010_5286((char *)uVar2,param_1,uVar5,uVar6,uVar2);
    param_1 = (u8 *)((u32)pcVar7 >> 0x10);
    iVar4 = pass1_1000_3d7a(param_3,(char *)((u32)pcVar7 & 0xffff | ZEXT24(param_1) << 0x10));
    if (iVar4 == 0x0) break;
    fn_ptr_1000_17ce(pcVar7);
    uStack6 += 0x1;
  }
  return;
}



u16 * pass1_1010_53ce(u16 *param_1,u8 param_2)

{
  pass1_1010_50f2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_sys_op_1010_5404(i16 param_1,astruct_19 *param_2,u16 param_3)

{
  i16 *piVar1;
  u16 **ppuVar2;
  u32 uVar3;
  code **ppcVar4;
  u32 uVar5;
  u16 uVar6;
  u16 uVar7;
  i16 iVar8;
  u16 *puVar9;
  INT16 IVar10;
  astruct_821 *uVar4;
  u16 uVar11;
  u16 uVar12;
  u32 in_EDX;
  Struct57*paVar14;
  Struct57*paVar15;
  u8 *puVar16;
  u16 uVar17;
  u16 uVar18;
  astruct_19 *paVar19;
  char *pcVar20;
  u32 *puVar21;
  u16 in_stack_0000fd74;
  u16 in_stack_0000fe98;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000fea2;
  u8 uVar22;
  u8 uVar23;
  u16 *puStack50;
  i16 iStack42;
  u32 uStack16;
  i16 iStack12;
  i16 iStack10;
  Struct57*paVar13;

  uVar18 = ((u32)in_EDX >> 0x10);
  paVar19 = struct_op_1010_1d48(param_2,param_3);
  uVar11 = ((u32)paVar19 >> 0x10);
  paVar13 = (Struct57*)CONCAT22(uVar18,uVar11);
  uVar18 = 0x0;
  (u32)((int)param_2 + 0xa) = 0x0;
  (u32)((int)param_2 + 0xe) = 0x0;
  (u32)((int)param_2 + 0x12) = 0x0;
  (u32)((int)param_2 + 0x16) = 0x0;
  (u32)((int)param_2 + 0x1a) = 0x0;
  ((int)param_2 + 0x62) = 0x0;
  (u32)((int)param_2 + 0x64) = 0x0;
  (u32)((int)param_2 + 0x68) = 0x0;
  (u32)((int)param_2 + 0x6c) = 0x0;
  ((int)param_2 + 0x70) = 0x1;
  ((int)param_2 + 0x7a) = 0x0;
  ((int)param_2 + 0x7c) = 0x0;
  ((int)param_2 + 0x7e) = 0x0;
  ((int)param_2 + 0x80) = 0x0;
  ((int)param_2 + 0x82) = 0x1;
  param_2->offset_0x0 = 0x6312;
  ((int)param_2 + 0x2) = 0x1010;
  pass1_1010_6034(uVar11,param_2);
  mem_op_1000_179c(0x101,paVar13);
  ((int)param_2 + 0xe) = uVar18;
  ((int)param_2 + 0x10) = paVar13;
  pass1_1000_5008(((int)param_2 + 0xe),paVar13,0x100);
  uVar18 = ((u32)paVar13 >> 0x10);
  uVar6 = str_op_1000_3da4(*(char **)((int)param_2 + 0xe));
  uVar5 = (u32)((int)param_2 + 0xe);
  uVar17 = ((u32)uVar5 >> 0x10);
  puVar16 = (u8 *)((int)uVar5 + uVar6);
  if (puVar16[-0x1] != '\\') {
    *puVar16 = 0x5c;
    uVar5 = (u32)((int)param_2 + 0xe);
    *(u8 *)((int)uVar5 + uVar6 + 0x1) = 0x0;
  }
  pcVar20 = load_string_1010_847e(_u16_1050_14cc,0x578);
  paVar13 = (Struct57*)CONCAT22(uVar18,(int)((u32)pcVar20 >> 0x10));
  pass1_1000_3cea((u32)((int)param_2 + 0xe),pcVar20);
  uVar7 = str_op_1008_60e8(paVar13,*(char **)((int)param_2 + 0xe));
  ((int)param_2 + 0xa) = uVar7;
  ((int)param_2 + 0xc) = (int)paVar13;
  GetPrivateProfileString16
            ((char *)CONCAT22((int)paVar13,((int)param_2 + 0xa)),0x100,*(char **)((int)param_2 + 0xe),
             s_playerName_1050_148e + 0xc,s_rez_1050_13c0,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    iVar8 = pass1_1000_3e2c((u32)((int)param_2 + 0xe));
    puVar21 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,(u8 **)CONCAT22(param_1,0x48),in_stack_0000fd74,
                              in_stack_0000fe98,in_stack_0000fe9e,in_stack_0000fea2);
    paVar13 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)puVar21 >> 0x10);
    uVar18 = ((u32)puVar21 >> 0x10);
    iStack10 = ((int)puVar21 + 0xa);
    iStack12 = ((int)puVar21 + 0xc);
    ((int)param_2 + 0x62) = (iVar8 != iStack10);
  }
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_falseMap_1050_13de,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    param_1 = (int)0x1050;
    iVar8 = pass1_1000_475e((u32)((int)param_2 + 0xe),(u32)s_on_1050_13c4);
    if (iVar8 == 0x0) {
      ((int)param_2 + 0x80) = 0x1;
    }
  }
  param_1 = (int)0x1050;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_music_1050_13d2,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    param_1 = (int)s_off_1050_13c8;
    iVar8 = pass1_1000_475e((u32)((int)param_2 + 0xe),(u32)s_off_1050_13c8);
    if (iVar8 == 0x0) {
      ((int)param_2 + 0x74) = 0x0;
    }
  }
  param_1 = (int)s_general_1050_13b0;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_sound_1050_13d8,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    uVar3 = (u32)((int)param_2 + 0xe);
    param_1 = (int)(uVar3 >> 0x10);
    iVar8 = pass1_1000_475e(uVar3,(u32)s_off_1050_13c8);
    if (iVar8 == 0x0) {
      ((int)param_2 + 0x72) = 0x0;
    }
  }
  param_1 = (int)0x1050;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_anims_1050_13cc,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    uVar3 = (u32)((int)param_2 + 0xe);
    param_1 = (int)uVar3;
    iVar8 = pass1_1000_475e(uVar3,(u32)s_off_1050_13c8);
    if (iVar8 == 0x0) {
      ((int)param_2 + 0x1e) = 0x0;
    }
  }
  param_1 = (int)s_movies_1050_13e8;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_movies_1050_13e8,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    param_1 = (int)s_tile2_bmp_1050_1538;
    iVar8 = pass1_1000_475e((u32)((int)param_2 + 0xe),(u32)s_off_1050_13c8);
    if (iVar8 == 0x0) {
      ((int)param_2 + 0x20) = 0x0;
    }
  }
  param_1 = (int)0x1050;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_turns_1050_1466,s_general_1050_13b0);
  paVar14 = paVar13;
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    uVar6 = pass1_1000_3e2c((u32)((int)param_2 + 0xe));
    uVar12 = paVar13 | uVar6;
    paVar14 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)uVar12);
    if (uVar12 != 0x0) {
      ((int)param_2 + 0x76) = uVar6;
      ((int)param_2 + 0x78) = paVar13;
      paVar14 = paVar13;
    }
  }
  param_1 = (int)s_playerName_1050_148e + 0xc;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_turnsDlgStatus_1050_146c,s_general_1050_13b0);
  if ((**(char **)((int)param_2 + 0xe) != '\0') &&
     (iVar8 = pass1_1000_475e((u32)((int)param_2 + 0xe),(u32)s_on_1050_13c4), iVar8 == 0x0)) {
    ((int)param_2 + 0x7a) = 0x1;
  }
  pcVar20 = *(char **)((int)param_2 + 0xe);
  param_1 = (int)((u32)pcVar20 >> 0x10);
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,pcVar20,s_playerName_1050_148e + 0xc,s_savePath_1050_147c,
             s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    uVar7 = str_op_1008_60e8(paVar14,*(char **)((int)param_2 + 0xe));
    ((int)param_2 + 0x1a) = uVar7;
    ((int)param_2 + 0x1c) = (int)paVar14;
  }
  pcVar20 = *(char **)((int)param_2 + 0xe);
  param_1 = (int)pcVar20;
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,pcVar20,s_playerName_1050_148e + 0xc,s_aiName_1050_1486,
             s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    uVar7 = str_op_1008_60e8(paVar14,*(char **)((int)param_2 + 0xe));
    ((int)param_2 + 0x68) = uVar7;
    ((int)param_2 + 0x6a) = (int)paVar14;
  }
  param_1 = 0x100;
  puVar9 = (u16 *)GetPrivateProfileString16
                             (*(char **)((int)param_2 + 0xa),0x100,*(char **)((int)param_2 + 0xe),
                              s_playerName_1050_148e + 0xc,s_playerName_1050_148e,s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    puVar9 = (u16 *)str_op_1008_60e8(paVar14,*(char **)((int)param_2 + 0xe));
    (u16*)((int)param_2 + 0x6c) = puVar9;
    ((int)param_2 + 0x6e) = (int)paVar14;
  }
  if (((int)param_2 + 0x62) == 0x0) {
    IVar10 = GetSystemMetrics16(SM_CYCAPTION);
    iStack42 = 0x1;
    do {
      get_private_profile_string_1010_6132(param_2,iStack42);
      iVar8 = iStack42 * 0x8 + (int)param_2;
      if (((((iVar8 + 0x22) < 0x0) || ((iVar8 + 0x24) < 0x0)) ||
          (piVar1 = (i16 *)(iVar8 + 0x22), *piVar1 != iStack10 - IVar10 && iStack10 - IVar10 <= *piVar1)) ||
         (puVar9 = (u16 *)(iStack12 - IVar10), ppuVar2 = (u16 **)(iVar8 + 0x24),
         *ppuVar2 != puVar9 && (int)puVar9 <= (int)*ppuVar2)) {
        puVar9 = pass1_1000_4906((StructD *)
                                 ((u32)param_2 & 0xffff0000 | (u32)(iStack42 * 0x8 + (int)param_2 + 0x22)),NULL,0x8)
        ;
      }
      iStack42 += 0x1;
    } while (iStack42 < 0x8);
  }
  mem_op_1000_179c(0xc,paVar14);
  uVar6 = paVar14 | puVar9;
  paVar13 = (Struct57*)((u32)paVar14 & 0xffff0000);
  paVar15 = (Struct57*)((u32)paVar13 | (u32)uVar6);
  if (uVar6 == 0x0) {
    puVar9 = NULL;
  }
  else {
    set_struct_1008_574a((Struct57*)CONCAT22(paVar14,puVar9));
    paVar13 = paVar15;
  }
  (u16*)((int)param_2 + 0x64) = puVar9;
  ((int)param_2 + 0x66) = (int)paVar13;
  uVar5 = (u32)((int)param_2 + 0xe);
  pass1_1000_5008(uVar5,((u32)uVar5 >> 0x10),0x100);
  uVar6 = str_op_1000_3da4(*(char **)((int)param_2 + 0xe));
  uVar5 = (u32)((int)param_2 + 0xe);
  uVar18 = ((u32)uVar5 >> 0x10);
  puVar16 = (u8 *)((int)uVar5 + uVar6);
  if (puVar16[-0x1] != '\\') {
    *puVar16 = 0x5c;
    uVar5 = (u32)((int)param_2 + 0xe);
    *(u8 *)((int)uVar5 + uVar6 + 0x1) = 0x0;
  }
  uVar4 = (astruct_821 *)str_op_1008_60e8(paVar13,*(char **)((int)param_2 + 0xe));
  uStack16 = CONCAT22((int)paVar13,uVar4);
  mem_op_1000_179c(0x8,paVar13);
  uVar6 = paVar13;
  puStack50 = (u16 *)CONCAT22(uVar6,uVar4);
  paVar13 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)(uVar6 | uVar4));
  if ((uVar6 | uVar4) != 0x0) {
    *puStack50 = 0x389a;
    uVar4->field2_0x2 = 0x1008;
    uVar4->field3_0x4 = uStack16;
    *puStack50 = 0x6322;
    uVar4->field2_0x2 = 0x1010;
  }
  ppcVar4 = (code **)((int)(u32)(u32)((int)param_2 + 0x64) + 0x4);
  (**ppcVar4)();
  pcVar20 = *(char **)((int)param_2 + 0xe);
  param_1 = (int)((u32)pcVar20 >> 0x10);
  GetPrivateProfileString16
            (*(char **)((int)param_2 + 0xa),0x100,pcVar20,s_playerName_1050_148e + 0xc,s_opimage_1050_13f0,
             s_general_1050_13b0);
  if (**(char **)((int)param_2 + 0xe) != '\0') {
    uVar5 = (u32)((int)param_2 + 0xe);
    uVar18 = uVar5;
    uVar22 = (u8)((u32)uVar5 >> 0x10);
    uVar23 = (u8)((u32)uVar5 >> 0x18);
    while( true ) {
      uVar7 = pass1_1000_47a4(CONCAT13(uVar23,CONCAT12(uVar22,uVar18)),(u32)s___1050_13f8);
      if ((paVar13 | uVar7) == 0x0) break;
      unk_str_op_1000_3d3e((char *)CONCAT13(0x10,CONCAT12(0x50,&param_1)),(char *)CONCAT22(paVar13,uVar7));
      uVar6 = str_op_1000_3da4((char *)CONCAT22(0x1050,&param_1));
      if ((&stack0xfecb)[uVar6] != '\\') {
        *(u8 *)((int)&param_1 + uVar6) = 0x5c;
        *(u8 *)((int)&param_1 + uVar6 + 0x1) = 0x0;
      }
      uVar6 = str_op_1008_60e8(paVar13,(char *)CONCAT22(0x1050,&param_1));
      uStack16 = CONCAT22((int)paVar13,uVar6);
      mem_op_1000_179c(0x8,paVar13);
      uVar12 = paVar13;
      puStack50 = (u16 *)CONCAT22(uVar12,uVar6);
      paVar13 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)(uVar12 | uVar6));
      if ((uVar12 | uVar6) != 0x0) {
        *puStack50 = 0x389a;
        (uVar6 + 0x2) = 0x1008;
        (u32)(uVar6 + 0x4) = uStack16;
        *puStack50 = 0x6322;
        (uVar6 + 0x2) = 0x1010;
      }
      ppcVar4 = (code **)((int)(u32)(u32)((int)param_2 + 0x64) + 0x8);
      (**ppcVar4)();
      uVar18 = 0x0;
      uVar22 = 0x0;
      uVar23 = 0x0;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void write_private_profile_str_1010_5b10(u16 param_1,StructD *param_2)

{
  u32 *puVar2;
  code **ppcVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 in_register_0000000a;
  StructD *iVar10;
  u16 unaff_SI;
  u16 uVar10;
  u32 *puVar10;
  u16 in_stack_0000fe76;
  u16 in_stack_0000ff9a;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa4;
  i16 iStack12;
  u32 *puVar1;
  u16 uVar3;

  uVar10 = ((u32)param_2 >> 0x10);
  iVar10 = (StructD *)param_2;
  param_2->address_offset_field_0x0 = 0x6312;
  iVar10->address_offset_field_0x2 = 0x1010;
  puVar10 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            (u8 **)CONCAT22(unaff_SI,0x48),in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,
                            in_stack_0000ffa4);
  sys_1000_3f9c(*(char **)&iVar10->field8_0xe,s__d__d_1050_149c,((int)puVar10 + 0xa));
  if (&iVar10->field_0x80 == 0x0) {
    // actualy just 0x13c8
    uVar4 = s_off_1050_13c8;
  }
  else {
    // actually just 0x13c4
    uVar4 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,(char *)CONCAT22(0x1050,uVar4),s_falseMap_1050_13de,s_general_1050_13b0);
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,*(char **)&iVar10->field8_0xe,s_rez_1050_13c0,s_general_1050_13b0);
  if (&iVar10->field_0x1e == 0x0) {
    // actually just 0x13c8
    uVar5 = s_off_1050_13c8;
  }
  else {
    // actually just 0x13c4
    uVar5 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,(char *)CONCAT22(0x1050,uVar5),s_anims_1050_13cc,s_general_1050_13b0);
  if (&iVar10->field_0x74 == 0x0) {
    // 0x13c8
    uVar6 = s_off_1050_13c8;
  }
  else {
    // 0x13c4
    uVar6 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,(char *)CONCAT22(0x1050,uVar6),s_music_1050_13d2,s_general_1050_13b0);
  if (&iVar10->field_0x72 == 0x0) {
    // 0x13c8
    uVar7 = s_off_1050_13c8;
  }
  else {
    // 0x13c4
    uVar7 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,(char *)CONCAT22(0x1050,uVar7),s_sound_1050_13d8,s_general_1050_13b0);
  if (iVar10->field19_0x20 == 0x0) {
    // 0x13c8
    uVar8 = s_off_1050_13c8;
  }
  else {
    // 0x13c4
    uVar8 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,(char *)CONCAT22(0x1050,uVar8),s_movies_1050_13e8,s_general_1050_13b0);
  sys_1000_3f9c(*(char **)&iVar10->field8_0xe,s__lu_1050_14a2,(u32)&iVar10->field_0x76);
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,*(char **)&iVar10->field8_0xe,s_turns_1050_1466,s_general_1050_13b0);
  if (&iVar10->field_0x7a == 0x0) {
    // 0x13c8
    uVar9 = SUB42(s_off_1050_13c8,0x0);
  }
  else {
    // 0x13c4
    uVar9 = SUB42(s_on_1050_13c4,0x0);
  }
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,(char *)CONCAT22(0x1050,uVar9),s_turnsDlgStatus_1050_146c,s_general_1050_13b0
            );
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,*(char **)&iVar10->field14_0x1a,s_savePath_1050_147c,s_general_1050_13b0);
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,*(char **)&iVar10->field_0x68,s_aiName_1050_1486,s_general_1050_13b0);
  WritePrivateProfileString16
            (*(char **)&iVar10->field6_0xa,*(char **)&iVar10->field_0x6c,s_playerName_1050_148e,s_general_1050_13b0);
  iStack12 = 0x1;
  do {
    caseD_13((u32)param_2,iStack12);
    iStack12 += 0x1;
  } while (iStack12 < 0x8);
  fn_ptr_1000_17ce(*(char **)&iVar10->field6_0xa);
  fn_ptr_1000_17ce(*(char **)&iVar10->field8_0xe);
  fn_ptr_1000_17ce(*(char **)&iVar10->field11_0x12);
  fn_ptr_1000_17ce(*(char **)((int)&iVar10->field12_0x14 + 0x2));
  fn_ptr_1000_17ce(*(char **)&iVar10->field14_0x1a);
  puVar2 = &iVar10->field_0x64;
  uVar3 = &iVar10->field_0x66;
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)(0x1000,puVar2,uVar3,0x1);
  }
  fn_ptr_1000_17ce(*(char **)&iVar10->field_0x68);
  fn_ptr_1000_17ce(*(char **)&iVar10->field_0x6c);
  pass1_1010_1d80(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_5d9c(u8 *param_1,param_2: u32,param_3: i16)

{
  u16 in_register_0000000a;
  u32 *puVar1;
  u16 in_stack_0000fea2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000fffa;

  ((int)param_2 + 0x1e) = param_3;
  if (param_3 == 0x0) {
    puVar1 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fffa,0x2e),in_stack_0000fea2,in_stack_0000ffc6,
                             in_stack_0000ffcc,in_stack_0000ffd0);
    pass1_1018_209c((u32)puVar1);
  }
  return;
}



u16 pass1_1010_5dc6(param_1: u32,u32 param_2)

{
  BOOL16 BVar1;
  i16 iVar2;
  u16 uVar3;
  HFILE16 in_stack_0000ffdc;
  u8 *local_c [0x3];
  u16 local_6 [0x2];

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    uVar3 = (param_1 >> 0x10);
    iVar2 = (int)param_1;
    BVar1 = pass1_1008_7c2a(param_2,*(char **)(iVar2 + 0x68));
    if (BVar1 != 0x0) {
      BVar1 = pass1_1008_7c2a(param_2,*(char **)(iVar2 + 0x6c));
      if (BVar1 != 0x0) {
        local_c[0] = PTR_LOOP_1050_13ae;
        BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffdc);
        if (BVar1 != 0x0) {
          local_6[0] = (iVar2 + 0x82);
          BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffdc);
          if (BVar1 != 0x0) {
            return 0x1;
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



void pass1_1010_5e56(i16 param_1,u16 param_2,param_3: u32,u32 param_4)

{
  u8 *puVar1;
  u16 uVar2;
  BOOL16 BVar3;
  i16 iVar4;
  u16 uVar5;
  HFILE16 HVar6;
  u16 uVar7;
  u8 *local_404;
  u8 local_402 [0x400];

  HVar6 = (HFILE16)param_4;
  uVar7 = (param_4 >> 0x10);
  read_file_1008_7cfe(HVar6,uVar7,0x4);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    puVar1 = local_402;
    read_file_1008_7c6e(HVar6,uVar7,(char *)CONCAT22(0x1050,puVar1));
    if (puVar1 != NULL) {
      uVar2 = str_op_1008_60e8(param_2,(char *)CONCAT22(0x1050,local_402));
      uVar5 = (param_3 >> 0x10);
      iVar4 = (int)param_3;
      (iVar4 + 0x68) = uVar2;
      (iVar4 + 0x6a) = param_2;
      puVar1 = local_402;
      read_file_1008_7c6e(HVar6,uVar7,(char *)CONCAT22(0x1050,puVar1));
      if (puVar1 != NULL) {
        uVar2 = str_op_1008_60e8(param_2,(char *)CONCAT22(0x1050,local_402));
        (iVar4 + 0x6c) = uVar2;
        (iVar4 + 0x6e) = param_2;
        BVar3 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,&local_404),0x2);
        if (BVar3 != 0x0) {
          PTR_LOOP_1050_13ae = local_404;
          if ((int)u16_1050_0312 < 0x2) {
            return;
          }
          BVar3 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x82)),0x2);
          if (BVar3 != 0x0) {
            return;
          }
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



void struct_1010_5f1e(u16 param_1,astruct_73 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_73 *iVar3;
  astruct_73 *uVar3;

  uVar3 = (astruct_73 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_73 *)param_2;
  fn_ptr_1000_17ce(iVar3->field22_0x16);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  &iVar3->field22_0x16 = uVar1;
  ((int)&iVar3->field22_0x16 + 0x2) = param_1;
  return;
}



void pass1_1010_5f4c(u16 param_1,astruct_484 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_484 *iVar3;
  astruct_484 *uVar2;

  uVar2 = (astruct_484 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_484 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field18_0x12);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field18_0x12 = uVar1;
  iVar3->field19_0x14 = param_1;
  return;
}



u32 pass1_1010_5f7a(i16 param_1,u16 param_2,u16 param_3,param_4: i16)

{
  i16 iVar1;

  iVar1 = param_4 * 0x8 + param_1;
  if (((iVar1 + 0x26) == 0x0) && ((iVar1 + 0x28) == 0x0)) {
    return 0x0;
  }
  return CONCAT22(param_2,param_4 * 0x8 + param_1 + 0x22);
}



void pass1_1010_5fb0(param_1: u32,u16 param_2,u32 *param_3,u16 param_4,param_5: i16)

{
  u16 uVar1;
  astruct_656 *iVar1;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_656 *)((int)param_1 + param_5 * 0x8);
  iVar1->field34_0x22 = *param_3;
  iVar1->field35_0x26 = param_3[0x1];
  return;
}



void pass1_1010_5fd8(u16 param_1,astruct_485 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_485 *iVar3;
  astruct_485 *uVar2;

  uVar2 = (astruct_485 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_485 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field104_0x68);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field104_0x68 = uVar1;
  iVar3->field105_0x6a = param_1;
  return;
}



void pass1_1010_6006(u16 param_1,astruct_486 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_486 *iVar3;
  astruct_486 *uVar2;

  uVar2 = (astruct_486 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_486 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field108_0x6c);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field108_0x6c = uVar1;
  iVar3->field109_0x6e = param_1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_6034(u16 param_1,astruct_19 *param_2)

{
  u16 *puVar1;
  astruct_19 *struct_1;
  astruct_19 *struct_1_hi;

  struct_1_hi = (astruct_19 *)((u32)param_2 >> 0x10);
  struct_1 = (astruct_19 *)param_2;
  struct_1->field16_0x1e = 0x1;
  struct_1->field17_0x20 = 0x1;
  struct_1[0x1].field9_0x12 = 0x1;
  struct_1[0x1].field10_0x14 = 0x1;
  pass1_1010_60a0(param_2);
  puVar1 = pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1->field18_0x22)),NULL,0x40);
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x630);
  struct_1[0x1].field3_0x8 = puVar1;
  struct_1[0x1].horiz_res_0xa = param_1;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x62f);
  struct_1[0x1].ver_res_0xc = puVar1;
  &struct_1[0x1].field_0xe = param_1;
  return;
}



void pass1_1010_60a0(astruct_19 *param_1)

{
  (u32)((int)param_1 + 0x76) = 0x5;
  return;
}



u16 pass1_1010_60b4(void)

{
  return 0x1;
}



u16 pass1_1010_60ba(void)

{
  return 0x1;
}



u16 pass1_1010_60c0(void)

{
  return 0x1;
}



u16 pass1_1010_60c6(void)

{
  return 0x1;
}



void pass1_1010_60cc(u16 param_1,astruct_487 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_487 *iVar3;
  astruct_487 *uVar2;

  uVar2 = (astruct_487 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_487 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field26_0x1a);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field26_0x1a = uVar1;
  iVar3->field27_0x1c = param_1;
  return;
}



void pass1_1010_60fa(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x7e) = 0x1;
  (iVar1 + 0x7c) = (iVar1 + 0x20);
  (iVar1 + 0x20) = 0x1;
  return;
}



void pass1_1010_6118(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x7e) != 0x0) {
    (iVar1 + 0x20) = (iVar1 + 0x7c);
  }
  return;
}



void get_private_profile_string_1010_6132(astruct_19 *param_1,param_2: i16)

{
  u16 uVar2;
  i16 iVar1;
  u16 uVar3;
  i16 iVar2;
  u16 uVar4;
  i16 iVar3;
  u16 uVar5;
  i16 iVar4;
  u16 in_DX;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  astruct_19 *iVar5;
  i16 iVar6;
  u16 uVar9;
  u16 unaff_SS;
  u32 uVar1;

  uVar9 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_19 *)param_1;
  GetPrivateProfileString16
            (*(char **)&iVar5->horiz_res_0xa,0x100,*(char **)&iVar5->field_0xe,s_playerName_1050_148e + 0xc,
             *(char **)(param_2 * 0x4 + 0x1446),s_windows_1050_13b8);
  if (**(char **)&iVar5->field_0xe != '\0') {
    uVar2 = pass1_1000_47a4((u32)&iVar5->field_0xe,(u32)s___1050_14a6);
    uVar6 = in_DX | uVar2;
    if (uVar6 != 0x0) {
      iVar1 = pass1_1000_3e2c(CONCAT22(in_DX,uVar2));
      (&iVar5->field18_0x22)[param_2 * 0x4] = iVar1;
      uVar3 = pass1_1000_47a4(0x0,(u32)s___1050_14a8);
      uVar7 = uVar6 | uVar3;
      if (uVar7 != 0x0) {
        iVar2 = pass1_1000_3e2c(CONCAT22(uVar6,uVar3));
        (&iVar5->field19_0x24)[param_2 * 0x4] = iVar2;
        uVar4 = pass1_1000_47a4(0x0,(u32)s___1050_14aa);
        uVar8 = uVar7 | uVar4;
        if (uVar8 != 0x0) {
          iVar3 = pass1_1000_3e2c(CONCAT22(uVar7,uVar4));
          (&iVar5->field20_0x26 + param_2 * 0x2) = iVar3;
          uVar5 = pass1_1000_47a4(0x0,(u32)s___1050_14ac);
          if ((uVar8 | uVar5) != 0x0) {
            iVar4 = pass1_1000_3e2c(CONCAT22(uVar8,uVar5));
            ((int)&iVar5->field20_0x26 + param_2 * 0x8 + 0x2) = iVar4;
          }
        }
      }
    }
  }
  return;
}



void caseD_13(param_1: u32,param_2: i16)

{
  astruct_833 *iVar1;

  iVar1 = (astruct_833 *)(param_2 * 0x8 + (int)param_1);
  if ((((iVar1->field34_0x22 != 0x0) || (iVar1->field35_0x24 != 0x0)) || (iVar1->field36_0x26 != 0x0)) ||
     (iVar1->field37_0x28 != 0x0)) {
    sys_1000_3f9c(*(char **)((int)param_1 + 0xe),s__d__d__d__d_1050_14ae,
                  (u32)(param_2 * 0x8 + (int)param_1 + 0x22));
    WritePrivateProfileString16
              (*(char **)((int)param_1 + 0xa),*(char **)((int)param_1 + 0xe),*(char **)(param_2 * 0x4 + 0x1446),
               s_windows_1050_13b8);
  }
  return;
}



void pass1_1010_62a4(astruct_488 *param_1,u8 param_2)

{
  astruct_488 *uVar2;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  uVar2 = (astruct_488 *)param_1;
  param_1 = 0x6322;
  uVar2->field2_0x2 = 0x1010;
  fn_ptr_1000_17ce((char *)uVar2->field3_0x4);
  param_1 = 0x389a;
  uVar2->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



StructD * pass1_1010_62ec(undefined1 param_1,u16 param_2,StructD *param_3,u8 param_4)

{
  write_private_profile_str_1010_5b10(param_2,param_3);
  if ((param_4 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_3);
  }
  return param_3;
}



void struct_1010_6326(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  (u32)((int)param_1 + 0x16) = 0x0;
  (u32)((int)param_1 + 0x1a) = 0x0;
  (u32)((int)param_1 + 0x1e) = 0x0;
  (u32)((int)param_1 + 0x22) = 0x0;
  param_1->offset_0x0 = 0x66f0;
  ((int)param_1 + 0x2) = 0x1010;
  return;
}



void write_to_file_1010_6372(astruct_729 *param_1,u32 param_2)

{
  BOOL16 BVar1;
  astruct_729 *iVar2;
  u16 uVar2;
  HFILE16 in_stack_0000ffce;
  u32 local_10 [0x2];
  u32 local_8;

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    uVar2 = ((u32)param_1 >> 0x10);
    iVar2 = (astruct_729 *)param_1;
    local_10[0] = iVar2->field10_0xa;
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffce);
    if (BVar1 != 0x0) {
      local_8 = iVar2->field11_0xe;
      BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffce);
      if (BVar1 != 0x0) {
        local_8 = iVar2->field12_0x12;
        BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffce);
        if (BVar1 != 0x0) {
          local_8 = iVar2->field13_0x16;
          BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffce);
          if (BVar1 != 0x0) {
            local_8 = iVar2->field14_0x1a;
            BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffce);
            if (BVar1 != 0x0) {
              local_8 = iVar2->field15_0x1e;
              BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffce);
              if (BVar1 != 0x0) {
                local_8 = iVar2->field16_0x22;
                BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffce);
                if (BVar1 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



void pass1_1010_648a(i16 param_1,param_2: u32,u32 param_3)

{
  i16 iVar1;
  BOOL16 BVar2;

  read_file_1008_7cfe((int)param_3,(int)(param_3 >> 0x10),0x7);
  if (param_1 != 0x0) {
    iVar1 = (int)param_2;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0xa)),0x4);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0xe)),0x4);
      if (BVar2 != 0x0) {
        BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x12)),0x4);
        if (BVar2 != 0x0) {
          BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x16)),0x4);
          if (BVar2 != 0x0) {
            BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x1a)),0x4);
            if (BVar2 != 0x0) {
              BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x1e)),0x4);
              if (BVar2 != 0x0) {
                BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar1 + 0x22)),0x4)
                ;
                if (BVar2 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



void pass1_1010_6566(param_1: u32,u16 param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;
  u16 uVar2;
  i16 local_4;

  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,(u16 *)CONCAT22(0x1050,&local_4),param_4);
  if (local_4 != 0x0) {
    (uVar1 + local_4) = param_3;
    (uVar1 + local_4 + 0x2) = param_2;
  }
  return;
}



i16 pass1_1010_659a(param_1: u32,u16 param_2)

{
  u16 uVar1;
  u16 uVar2;
  i16 local_4;

  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,(u16 *)CONCAT22(0x1050,&local_4),param_2);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return (uVar1 + local_4) - (uVar1 + local_4 + 0x2);
}



u16 pass1_1010_65d0(param_1: u32,u16 param_2)

{
  u16 uVar1;
  i16 local_4;

  uVar1 = (param_1 >> 0x10);
  switch_1010_6646(param_1,uVar1,(u16 *)CONCAT22(0x1050,&local_4),param_2);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return (param_1 + local_4 + 0x2);
}



void pass1_1010_6604(param_1: u32,u16 param_2)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  i16 local_4;

  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  switch_1010_6646(uVar2,uVar3,(u16 *)CONCAT22(0x1050,&local_4),param_2);
  if (local_4 != 0x0) {
    iVar1 = (uVar2 + local_4 + 0x2);
    (uVar2 + local_4) = (uVar2 + local_4);
    (uVar2 + local_4 + 0x2) = iVar1 + 0x1;
    pass1_1010_1f62((Struct27 *)(param_1 & 0xffff | (u32)uVar3 << 0x10),0x15);
  }
  return;
}



void switch_1010_6646(u16 param_1,u16 param_2,u16 *param_3,u16 param_4)

{
  switch(param_4) {
  case 0x83:
    *param_3 = 0xa;
    break;
  case 0x84:
    *param_3 = 0xe;
    break;
  case 0x85:
    *param_3 = 0x12;
    break;
  case 0x86:
    *param_3 = 0x16;
    return;
  case 0x87:
    *param_3 = 0x1a;
    return;
  case 0x88:
    *param_3 = 0x1e;
    return;
  case 0x89:
    *param_3 = 0x22;
    return;
  default:
    *param_3 = 0x0;
    return;
  }
  return;
}



u16 * pass1_1010_66ca(u16 *param_1,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_19 * pass1_1010_6700(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0x148) = 0x33;
    //        1010:6aac  86  6a  10  10      addr         pass1_1010_6a86
    //
  param_1->offset_0x0 = 0x6aac;
  ((int)param_1 + 0x2) = 0x1010;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0xa)),NULL,0x114);
  ((int)param_1 + 0x32) = 0x1;
  ((int)param_1 + 0x40) = 0x1;
  ((int)param_1 + 0x46) = 0x1;
  ((int)param_1 + 0x4e) = 0x1;
  ((int)param_1 + 0x54) = 0x1;
  ((int)param_1 + 0x5e) = 0x1;
  ((int)param_1 + 0x68) = 0x1;
  ((int)param_1 + 0x6c) = 0x1;
  ((int)param_1 + 0x74) = 0x1;
  ((int)param_1 + 0x78) = 0x1;
  ((int)param_1 + 0x7a) = 0x1;
  ((int)param_1 + 0x7e) = 0x1;
  ((int)param_1 + 0x82) = 0x1;
  ((int)param_1 + 0xa2) = 0x1;
  ((int)param_1 + 0xa4) = 0x1;
  ((int)param_1 + 0xa6) = 0x1;
  ((int)param_1 + 0xa8) = 0x1;
  ((int)param_1 + 0xae) = 0x1;
  ((int)param_1 + 0xb2) = 0x1;
  ((int)param_1 + 0xb8) = 0x1;
  ((int)param_1 + 0xbe) = 0x1;
  ((int)param_1 + 0xc0) = 0x1;
  ((int)param_1 + 0xc4) = 0x1;
  ((int)param_1 + 0xd4) = 0x1;
  ((int)param_1 + 0xda) = 0x1;
  ((int)param_1 + 0xe2) = 0x1;
  ((int)param_1 + 0xfe) = 0x1;
  ((int)param_1 + 0x100) = 0x1;
  ((int)param_1 + 0x102) = 0x1;
  ((int)param_1 + 0x104) = 0x1;
  ((int)param_1 + 0x106) = 0x1;
  ((int)param_1 + 0x108) = 0x1;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x11e)),NULL,0x2a);
  ((int)param_1 + 0x120) = 0x1;
  ((int)param_1 + 0x122) = 0x1;
  ((int)param_1 + 0x124) = 0x1;
  ((int)param_1 + 0x126) = 0x1;
  ((int)param_1 + 0x128) = 0x1;
  ((int)param_1 + 0x12c) = 0x1;
  ((int)param_1 + 0x138) = 0x1;
  return param_1;
}



void pass1_1010_6814(param_1: u32,u16 param_2,param_3: i16)

{
  ((int)param_1 + param_3 * 0x2 + 0x11e) = param_2;
  return;
}



void pass1_1010_682e(param_1: u32,u16 param_2,param_3: i16)

{
  ((int)param_1 + param_3 * 0x2 + 0xa) = param_2;
  return;
}



void write_to_file_1010_6846(param_1: u32,u8 *param_2)

{
  BOOL16 BVar1;
  i16 iVar2;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    iVar2 = (int)param_1;
    BVar1 = write_to_file_1008_7e1c(param_2,param_1 & 0xffff0000 | (u32)(iVar2 + 0xa),(char *)0x114,in_stack_0000ffde)
    ;
    if (BVar1 != 0x0) {
      BVar1 = write_to_file_1008_7e1c
                        (param_2,param_1 & 0xffff0000 | (u32)(iVar2 + 0x11e),(char *)0x2a,in_stack_0000ffde);
      if (BVar1 != 0x0) {
        local_c[0] = (iVar2 + 0x148);
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
        if (BVar1 != 0x0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



void pass1_1010_68c6(u16 param_1,u8 *param_2,param_3: u32,u32 param_4)

{
  astruct_248 *iVar2;
  BOOL16 BVar1;
  i16 iVar3;
  BOOL16 BVar4;
  u16 uVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  Struct57*paVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u16 uVar12;
  char *pcStack18;
  char *pcStack10;
  i16 local_6;
  u16 uStack4;

  paVar7 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  uVar9 = param_4;
  uVar10 = (param_4 >> 0x10);
  read_file_1008_7cfe(uVar9,uVar10,0x3);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
    return;
  }
  iVar2 = (astruct_248 *)param_3;
  uVar8 = (param_3 >> 0x10);
  if ((int)u16_1050_0312 < 0x2) {
    uVar11 = 0x102;
    uVar12 = 0x0;
    mem_op_1000_179c(0x102,paVar7);
    uVar6 = paVar7;
    pcStack10 = (char *)CONCAT22(uVar6,param_1);
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(uVar6,param_1),CONCAT22(uVar12,uVar11));
    pcStack18 = pcStack10;
    if (BVar1 == 0x0) goto LAB_1010_692c;
    uStack4 = 0x1;
    do {
      iVar3 = switch_1008_73ea(uVar9,uVar10,uStack4);
      (&iVar2->field_0xa + iVar3 * 0x2) = (uStack4 * 0x2 + param_1);
      uStack4 += 0x1;
    } while (uStack4 < 0x81);
    fn_ptr_1000_17ce(pcStack10);
    BVar1 = (BOOL16)pcStack10;
  }
  else {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | ZEXT24(&iVar2->field_0xa)),0x114);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  if ((int)u16_1050_0312 < 0x2) {
    uVar11 = 0x2a;
    uVar12 = 0x0;
    mem_op_1000_179c(0x2a,paVar7);
    uVar6 = paVar7;
    pcStack18 = (char *)CONCAT22(uVar6,BVar1);
    BVar4 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(uVar6,BVar1),CONCAT22(uVar12,uVar11));
    if (BVar4 == 0x0) {
LAB_1010_692c:
      u16_1050_0310 = 0x6d2;
      fn_ptr_1000_17ce((char *)((u32)pcStack18 & 0xffff | (u32)uVar6 << 0x10));
      return;
    }
    uStack4 = 0x0;
    do {
      uVar5 = switch_1008_72bc((HFILE16 *)param_4,uStack4);
      (&iVar2->field_0x11e + uVar5 * 0x2) = (uStack4 * 0x2 + BVar1);
      uStack4 += 0x1;
    } while (uStack4 < 0x15);
    fn_ptr_1000_17ce(pcStack18);
  }
  else {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | ZEXT24(&iVar2->field_0x11e)),0x2a);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = switch_1008_73ea(uVar9,uVar10,local_6);
  iVar2->field328_0x148 = BVar1;
  return;
}



u16 * pass1_1010_6a86(u16 *param_1,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_6abc(astruct_19 *param_1,u16 param_2,u16 param_3)

{
  code **ppcVar1;
  u32 in_EDX;
  u16 uVar3;
  Struct57*paVar2;
  astruct_19 *paVar4;
  u32 *puVar5;
  u16 in_stack_0000fe94;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;

  uVar3 = ((u32)in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48(param_1,param_2);
  paVar2 = (Struct57*)CONCAT22(uVar3,(int)((u32)paVar4 >> 0x10));
    //        1008:389a  7e  37  08  10      addr         pass1_1008_377e
    //
  ((int)param_1 + 0xa) = 0x389a;
  ((int)param_1 + 0xc) = 0x1008;
    //        1008:3aa8  14  3a  08  10      addr *       pass1_1008_3a14
  ((int)param_1 + 0xa) = 0x3aa8;
  ((int)param_1 + 0xc) = 0x1008;
  ((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x10) = 0x0;
  (u32)((int)param_1 + 0x14) = 0x0;
  (u32)((int)param_1 + 0x1c) = 0x0;
  ((int)param_1 + 0x20) = 0x0;
    //        1010:7e28  fe  7d  10  10      addr         FUN_1010_7dfe
    //
  (u32)((int)param_1 + 0x22) = 0x0;
  param_1->offset_0x0 = 0x7e28;
  ((int)param_1 + 0x2) = 0x1010;
    //        1010:7e38  c6  7d  10  10      addr         pass1_1010_7dc6
    //
  ((int)param_1 + 0xa) = 0x7e38;
  ((int)param_1 + 0xc) = 0x1010;
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3),in_stack_0000fe9e,in_stack_0000ffc2
                           ,in_stack_0000ffc8,in_stack_0000ffcc);
  paVar2 = (Struct57*)((u32)paVar2 & 0xffff0000);
  ((int)param_1 + 0x14) = (int)puVar5;
  ((int)param_1 + 0x16) = (int)((u32)puVar5 >> 0x10);
  if (param_1 != NULL) {
    paVar2 = (Struct57*)((u32)paVar2 | (u32)param_1);
  }
  uVar3 = (u32)((int)param_1 + 0x14);
  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x14) + 0x4);
  (**ppcVar1)();
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(uVar3,0x2f),in_stack_0000fe94,in_stack_0000ffb8,
                           in_stack_0000ffbe,in_stack_0000ffc2);
  ((int)param_1 + 0x22) = (int)puVar5;
  ((int)param_1 + 0x24) = (int)((u32)puVar5 >> 0x10);
  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x22) + 0x4);
  (**ppcVar1)();
  return;
}



void pass1_1010_6bb2(u16 *param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 *puStack14;

  uVar7 = ((u32)param_1 >> 0x10);
  uVar6 = param_1;
  *param_1 = 0x7e28;
  (uVar6 + 0x2) = 0x1010;
  (uVar6 + 0xa) = 0x7e38;
  (uVar6 + 0xc) = 0x1010;
  puVar1 = (u32 *)(uVar6 + 0x1c);
  uVar3 = (uVar6 + 0x1e);
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)();
  }
  (u32)(uVar6 + 0x1c) = 0x0;
  if (*(i32 *)(uVar6 + 0x14) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == NULL) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((u32)(uVar6 + 0x14),(StructD *)CONCAT22(uVar5,uVar3));
  }
  if (*(i32 *)(uVar6 + 0x22) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == NULL) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((u32)(uVar6 + 0x22),(StructD *)CONCAT22(uVar5,uVar3));
  }
  (u32)(uVar6 + 0x14) = 0x0;
  (u32)(uVar6 + 0x22) = 0x0;
  if (param_1 == NULL) {
    iVar4 = 0x0;
    uVar7 = 0x0;
  }
  else {
    iVar4 = uVar6 + 0xa;
  }
  puStack14 = (u16 *)CONCAT22(uVar7,iVar4);
  *puStack14 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1010_6ca2(u16 param_1,param_2: u32,param_3: i16)

{
  u32 uVar1;
  u16 *puVar2;
  u16 uVar3;
  i16 iStack10;
  u16 *puStack8;

  puStack8 = (u16 *)CONCAT22(0x1050,&stack0x000a);
  iStack10 = param_3;
  do {
    puVar2 = puStack8;
    if (iStack10 == 0x0) {
      return 0x1;
    }
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0x2));
    uVar3 = *puVar2;
    uVar1 = (u32)((int)param_2 + 0x14);
    pass1_1010_a5ca(uVar3,param_1,uVar1,((u32)uVar1 >> 0x10),uVar3);
    iStack10 = iStack10 + -0x1;
  } while (uVar3 == 0x0);
  return 0x0;
}



u16 pass1_1010_6cf8(u16 param_1,u16 param_2,u16 param_3,param_4: u32,param_5: i16)

{
  u16 uVar1;

  switch(param_5) {
  case 0x1:
    pass1_1010_715c(param_1,param_2,param_4,0x1);
    send_msg_1010_7c9e(param_4,0x12);
    return 0x1;
  default:
    return 0x0;
  case 0x4:
    uVar1 = 0x2;
    break;
  case 0x5:
    uVar1 = 0x3;
    break;
  case 0x6:
    uVar1 = 0x4;
    break;
  case 0x7:
    uVar1 = 0x5;
    break;
  case 0x9:
    pass1_1010_715c(param_1,param_2,param_4,0x6);
  case 0x2e:
    uVar1 = 0x38;
    break;
  case 0xa:
  case 0x80:
    uVar1 = 0x2d;
    break;
  case 0xb:
    uVar1 = 0x7;
    break;
  case 0xc:
  case 0x17:
  case 0x18:
  case 0x19:
  case 0x21:
  case 0x75:
  case 0x81:
    if (param_5 == 0x75) {
      pass1_1010_715c(param_1,param_2,param_4,0x8);
      pass1_1010_715c(param_1,param_2,param_4,0x9);
    }
    uVar1 = pass1_1010_6ca2(param_2,param_4,0x7);
    if (uVar1 != 0x0) {
      pass1_1010_715c(uVar1,param_2,param_4,0x10);
    }
    param_1 = pass1_1010_6ca2(param_2,param_4,0x3);
    if (param_1 != 0x0) {
      pass1_1010_715c(param_1,param_2,param_4,0x11);
    }
    if (param_5 == 0x21) {
      pass1_1010_715c(param_1,param_2,param_4,0x14);
    }
    if (param_5 != 0xc) {
      return 0x1;
    }
    uVar1 = 0x9;
    goto code_r0x10106d4c;
  case 0xe:
    uVar1 = 0xc;
    goto code_r0x10106d4c;
  case 0x10:
  case 0x11:
  case 0x13:
    uVar1 = 0xd;
    break;
  case 0x12:
    uVar1 = 0xe;
    break;
  case 0x1b:
  case 0x1f:
  case 0x5b:
  case 0x78:
  case 0x7e:
  case 0x7f:
    if (param_5 == 0x7e) {
      pass1_1010_715c(param_1,param_2,param_4,0x2c);
    }
    if (param_5 == 0x5b) {
      pass1_1010_715c(param_1,param_2,param_4,0x38);
    }
    if (param_5 == 0x1f) {
      pass1_1010_715c(param_1,param_2,param_4,0x3f);
    }
    if (param_5 == 0x7f) {
      pass1_1010_715c(param_1,param_2,param_4,0x42);
    }
    param_1 = pass1_1010_6ca2(param_2,param_4,0x5);
    if ((param_1 == 0x0) && (param_1 = pass1_1010_6ca2(param_2,param_4,0x5), param_1 == 0x0)) {
      return 0x1;
    }
    uVar1 = 0x37;
    break;
  case 0x1d:
  case 0x2a:
  case 0x2c:
  case 0x3c:
  case 0x3d:
  case 0x4b:
  case 0x53:
  case 0x54:
  case 0x55:
  case 0x5a:
    uVar1 = pass1_1010_6ca2(param_2,param_4,0x2);
    if (uVar1 != 0x0) {
      pass1_1010_715c(uVar1,param_2,param_4,0x12);
    }
    uVar1 = pass1_1010_6ca2(param_2,param_4,0x8);
    if (uVar1 != 0x0) {
      pass1_1010_715c(uVar1,param_2,param_4,0x1a);
    }
    if (param_5 == 0x2c) {
      pass1_1010_715c(uVar1,param_2,param_4,0x1d);
    }
    param_1 = pass1_1010_6ca2(param_2,param_4,0x2);
    if (param_1 == 0x0) {
      return 0x1;
    }
    uVar1 = 0x1c;
    break;
  case 0x22:
    uVar1 = 0x15;
    break;
  case 0x25:
    uVar1 = 0x16;
    break;
  case 0x26:
    pass1_1010_715c(param_1,param_2,param_4,0x17);
  case 0x1e:
    uVar1 = 0x13;
    break;
  case 0x27:
    uVar1 = 0x18;
    break;
  case 0x29:
    uVar1 = 0x19;
    break;
  case 0x2b:
    uVar1 = 0x1b;
    break;
  case 0x2f:
  case 0x36:
    param_1 = pass1_1010_6ca2(param_2,param_4,0x2);
    if (param_1 == 0x0) {
      return 0x0;
    }
    uVar1 = 0x1e;
    break;
  case 0x30:
    uVar1 = 0x1f;
    break;
  case 0x31:
    uVar1 = 0x35;
    break;
  case 0x33:
    uVar1 = 0x21;
    break;
  case 0x34:
    uVar1 = 0x22;
    break;
  case 0x35:
    pass1_1010_715c(param_1,param_2,param_4,0x23);
  case 0x65:
  case 0x66:
  case 0x6b:
  case 0x6c:
  case 0x6d:
  case 0x6e:
  case 0x6f:
    uVar1 = 0x34;
    break;
  case 0x38:
    pass1_1010_715c(param_1,param_2,param_4,0x24);
    uVar1 = 0x3d;
    break;
  case 0x39:
    uVar1 = 0x25;
    break;
  case 0x3e:
    pass1_1010_715c(param_1,param_2,param_4,0x26);
    pass1_1010_715c(param_1,param_2,param_4,0x28);
    uVar1 = 0x27;
    break;
  case 0x40:
    uVar1 = 0x2a;
    break;
  case 0x41:
    uVar1 = 0x39;
    break;
  case 0x42:
    uVar1 = 0x3a;
    break;
  case 0x44:
    uVar1 = 0x36;
    break;
  case 0x45:
    uVar1 = 0x3b;
    break;
  case 0x49:
    uVar1 = 0x29;
    break;
  case 0x50:
    uVar1 = 0x2b;
    break;
  case 0x56:
    pass1_1010_715c(param_1,param_2,param_4,0x3c);
    uVar1 = 0x3e;
    break;
  case 0x5d:
    pass1_1010_715c(param_1,param_2,param_4,0x2f);
    uVar1 = 0x40;
    break;
  case 0x5e:
  case 0x60:
    uVar1 = 0x2f;
    break;
  case 0x5f:
    pass1_1010_715c(param_1,param_2,param_4,0x34);
    uVar1 = 0x41;
    break;
  case 0x61:
    uVar1 = 0x30;
    break;
  case 0x63:
    uVar1 = 0x31;
    break;
  case 0x64:
    uVar1 = 0x24;
    break;
  case 0x68:
    uVar1 = 0x32;
    break;
  case 0x69:
    uVar1 = 0x33;
    break;
  case 0x76:
    uVar1 = 0xa;
code_r0x10106d4c:
    pass1_1010_715c(param_1,param_2,param_4,uVar1);
    uVar1 = 0xb;
  }
  pass1_1010_715c(param_1,param_2,param_4,uVar1);
  return 0x1;
}



u16 FUN_1010_702e(void)

{
  u16 in_AX;
  u16 in_DX;
  i16 unaff_BP;

  pass1_1010_715c(in_AX,in_DX,(u32)(unaff_BP + 0x6),0x3c);
  pass1_1010_715c(in_AX,in_DX,(u32)(unaff_BP + 0x6),0x3e);
  return 0x1;
}



u16 FUN_1010_703e(void)

{
  return 0x0;
}



void FUN_1010_7041(void)

{
  return;
}



void pass1_1010_715c(u16 param_1,u16 param_2,param_3: u32,u16 param_4)

{
  pass1_1010_a69c(param_1,param_2,(u32)((int)param_3 + 0x14),param_4);
  return;
}



void FUN_1010_7174(u16 param_1,param_2: u32,u16 param_3)

{
  u32 uVar1;
  u16 in_DX;
  i16 iVar2;
  u16 uVar3;

  iVar2 = (int)param_2;
  uVar3 = (param_2 >> 0x10);
  if (param_3 == 0x13) {
    uVar1 = (u32)(iVar2 + 0x18);
    destroy_window_1010_7b26(in_DX,param_2 & 0xffff0000 | (u32)(iVar2 - 0xa),*(i32 *)((int)uVar1 + 0x28));
    return;
  }
  if (param_3 < 0x14) {
    if ((char)param_3 == '\x01') {
      (u32)(iVar2 + 0xa) = 0x0;
      (u32)(iVar2 + 0x18) = 0x0;
      return;
    }
    if ((char)param_3 == '\x05') {
      send_msg_1010_7c42(param_2 & 0xffff0000 | (u32)(iVar2 - 0xa));
      return;
    }
  }
  return;
}



void pass1_1010_71b0(void)

{
  u32 uVar1;
  i16 unaff_BP;

  uVar1 = (u32)(unaff_BP + 0x6);
  send_msg_1010_7c42(uVar1 & 0xffff0000 | (u32)((int)uVar1 - 0xa));
  return;
}



void pass1_1010_71c2(u16 param_1,u16 param_2)

{
  u32 uVar1;
  u32 uVar2;
  i16 iVar3;
  i16 unaff_BP;
  u16 uVar4;

  if (param_1 == 0x13) {
    uVar2 = (u32)(unaff_BP + 0x6);
    uVar2 = (u32)((int)uVar2 + 0x18);
    uVar1 = (u32)(unaff_BP + 0x6);
    destroy_window_1010_7b26(param_2,uVar1 & 0xffff0000 | (u32)((int)uVar1 - 0xa),*(i32 *)((int)uVar2 + 0x28));
    return;
  }
  if (param_1 < 0x14) {
    if ((char)param_1 == '\x01') {
      uVar2 = (u32)(unaff_BP + 0x6);
      uVar4 = ((u32)uVar2 >> 0x10);
      iVar3 = (int)uVar2;
      (u32)(iVar3 + 0xa) = 0x0;
      (u32)(iVar3 + 0x18) = 0x0;
      return;
    }
    if ((char)param_1 == '\x05') {
      uVar1 = (u32)(unaff_BP + 0x6);
      send_msg_1010_7c42(uVar1 & 0xffff0000 | (u32)((int)uVar1 - 0xa));
      return;
    }
  }
  return;
}



void pass1_1010_71d6(u16 param_1,u16 param_2,param_3: u32,i16 param_4,u16 *param_5,u16 param_6)

{
  u16 uVar1;
  i16 iVar2;
  u32 uVar3;
  u16 uVar4;
  i16 iVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  u32 uVar9;
  u16 uVar10;
  astruct_15 *paVar11;
  u16 uStack20;
  u16 uStack14;
  u32 uStack6;

  uVar10 = (param_3 >> 0x10);
  uVar3 = (u32)((int)param_3 + 0x14);
  pass1_1010_ad22(param_2,uVar3,((u32)uVar3 >> 0x10),*param_5);
  uStack6 = CONCAT22(param_2,param_1);
  if ((param_2 | param_1) == 0x0) {
    return;
  }
  paVar11 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(param_2,param_1),param_1,param_2 | param_1);
  uVar8 = ((u32)paVar11 >> 0x10);
  uVar4 = paVar11;
  if (((uVar8 | uVar4) != 0x0) && (*(i32 *)(uVar4 + 0x1c) == 0x8000002)) {
    return;
  }
  uVar3 = (u32)(param_1 + 0x2e);
  uVar1 = (param_1 + 0x30);
  uVar9 = CONCAT22(in_register_0000000a,uVar1);
  uStack14 = uVar3;
  if (((uVar1 | uStack14) != 0x0) && (*(i32 *)(uStack14 + 0x200) == 0x8000002)) {
    return;
  }
  uVar3 = (u32)((int)param_3 + 0x14);
  uVar6 = pass1_1010_b028(uVar3,((u32)uVar3 >> 0x10),paVar11);
  iVar2 = (uVar4 + 0x12);
  iVar5 = iVar2;
  if ((iVar2 != 0x4) && (iVar5 = param_4, iVar2 == 0x7)) {
    param_4 = 0x5;
    iVar5 = param_4;
  }
  param_4 = iVar5;
  uVar7 = param_4 - 0x2;
  if (uVar7 != 0x0) {
    if (param_4 == 0x3) {
      uVar7 = uVar6 - 0xb;
      if ((uVar7 == 0x0) || (uVar7 = uVar6 - 0x37, uVar7 == 0x0)) {
        uStack20 = 0xb;
      }
      else {
        uStack20 = 0xa;
      }
      goto LAB_1010_72a7;
    }
    uVar7 = param_4 - 0x4;
    if (uVar7 == 0x0) {
      uStack20 = 0x17;
      goto LAB_1010_72a7;
    }
    uVar7 = param_4 - 0x5;
    if (uVar7 != 0x0) {
      uVar7 = pass1_1010_7818(param_3,paVar11);
      uStack20 = uVar7;
      goto LAB_1010_72a7;
    }
  }
  uStack20 = 0xc;
LAB_1010_72a7:
  if (uStack20 == 0x0) {
    return;
  }
  ui_op_1010_79aa(param_3,0x0,uStack6);
  if (uVar7 == 0x0) {
    unk_win_op_1010_7300(uVar9,(Struct57*)param_3,0x0,uStack20,uStack6);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void unk_win_op_1010_7300(Struct57*param_1,Struct57*param_2,param_3: u32,u16 param_4,u32 param_5)

{
  u32 uVar1;
  u32 uVar2;
  code **ppcVar3;
  char cVar4;
  Struct57*paVar5;
  u16 uVar6;
  StructD *pSVar7;
  u16 uVar8;
  u16 uVar9;
  Struct57*paVar10;
  Struct57*paVar11;
  Struct57*uVar13;
  u16 uVar12;
  Struct57*paVar13;
  u16 *puVar14;
  u32 uVar15;
  u16 *puStack20;
  u16 *puStack14;
  u32 *puStack10;
  u32 uStack6;

  if (param_4 == 0x0) {
    return;
  }
  uStack6 = param_3;
  paVar11 = (Struct57*)param_2;
  uVar13 = (Struct57*)((u32)param_2 >> 0x10);
  if (param_3 == 0x0) {
    uVar1 = paVar11->field10_0x14;
    pass1_1010_ad64(0x0,param_1,uVar1,CONCAT22(param_4,(int)(uVar1 >> 0x10)),param_5);
    uStack6 = param_3 & 0xffff | (long)param_1 << 0x10;
  }
  switch(param_4) {
  case 0x1:
  case 0x4:
  case 0x6:
  case 0x7:
  case 0x8:
  case 0x9:
  case 0xd:
  case 0xe:
  case 0x14:
  case 0x18:
    break;
  default:
    if ((uStack6 | uStack6) == 0x0) {
      return;
    }
  }
  pass1_1010_1f62((Struct27 *)param_2,0xb);
  if (paVar11->field7_0xe == 0x0) {
    return;
  }
  paVar5 = paVar11;
  switch(param_4 - 0x1) {
  case 0x0:
    mem_op_1000_179c(0x94,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) {
LAB_1010_73fe:
      uVar12 = 0x1000;
      paVar5 = NULL;
      paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000);
    }
    else {
      uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
      pass1_1040_44d2(paVar5,(u8 *)paVar10,
                      (Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),param_5,
                      paVar11->field7_0xe);
    }
    break;
  default:
    mem_op_1000_179c(0x94,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_b040((Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),uStack6,
                    paVar11->field7_0xe);
    break;
  case 0x3:
    mem_op_1000_179c(0x9e,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_5626(paVar10,(Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),param_5
                    ,paVar11->field7_0xe);
    break;
  case 0x4:
    mem_op_1000_179c(0x94,param_1);
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    puVar14 = pass1_1040_8e58((int)paVar5,param_1,uStack6,CONCAT22(paVar11->field7_0xe,uStack6));
    paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)puVar14 >> 0x10);
    paVar5 = (Struct57*)puVar14;
    break;
  case 0x5:
  case 0x6:
    mem_op_1000_179c(0x98,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_48a0(paVar10,CONCAT22(param_1,paVar5),param_4,param_5,paVar11->field7_0xe);
    break;
  case 0x7:
    mem_op_1000_179c(0x9c,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&u16_1050_1038,0x0);
    pass1_1038_9144(uVar8,(u16 *)CONCAT22(param_1,paVar5),paVar11->field7_0xe);
    break;
  case 0x8:
    mem_op_1000_179c(0xb8,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_b7ee((Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),uStack6,
                    paVar11->field7_0xe);
    break;
  case 0x9:
  case 0xa:
    mem_op_1000_179c(0x94,param_1);
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000);
    if (((Struct57*)param_1 | paVar5) == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&u16_1050_1038,0x0);
    puVar14 = pass1_1038_9a1e(paVar5,(Struct57*)param_1,uStack6,CONCAT22(paVar11->field7_0xe,uStack6));
    paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)puVar14 >> 0x10);
    paVar5 = (Struct57*)puVar14;
    break;
  case 0xb:
    mem_op_1000_179c(0x12a,param_1);
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000);
    if (((Struct57*)param_1 | paVar5) == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&u16_1050_1038,0x0);
    uVar15 = pass1_1038_9b72(paVar5,(Struct57*)param_1,uStack6,CONCAT22(paVar11->field7_0xe,uStack6));
    paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000 | uVar15 >> 0x10);
    paVar5 = (Struct57*)uVar15;
    break;
  case 0xc:
    mem_op_1000_179c(0x9c,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_6826((Struct57*)CONCAT22(param_1,paVar5),paVar11->field7_0xe);
    break;
  case 0xd:
    mem_op_1000_179c(0x9c,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_6fb6((Struct57*)CONCAT22(param_1,paVar5),paVar11->field7_0xe);
    break;
  case 0x12:
    mem_op_1000_179c(0x94,param_1);
    uVar8 = (u8 *)param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    make_proc_inst_1040_a234((u8 *)paVar5,(u8 *)param_1,uStack6,CONCAT22(paVar11->field7_0xe,uStack6));
    break;
  case 0x13:
    mem_op_1000_179c(0xb8,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_4e94((Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),uStack6,
                    paVar11->field7_0xe);
    break;
  case 0x14:
    mem_op_1000_179c(0x9a,param_1);
    pSVar7 = (StructD *)(param_1 | paVar5);
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | ZEXT24(pSVar7));
    if (pSVar7 == NULL) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_0e1c(pSVar7,(Struct57*)CONCAT22(param_1,paVar5),0x1,uStack6,paVar11->field7_0xe);
    break;
  case 0x15:
    mem_op_1000_179c(0x9c,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar13 = pas1_1040_29c2(paVar5,uVar8,
                             (Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),
                             uStack6,paVar11->field7_0xe);
    paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)paVar13 >> 0x10);
    paVar5 = (Struct57*)paVar13;
    break;
  case 0x16:
    mem_op_1000_179c(0x12a,param_1);
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&u16_1050_1038,0x0);
    puVar14 = pass1_1038_adde((int)paVar5,param_1,uStack6,CONCAT22(paVar11->field7_0xe,uStack6));
    paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)puVar14 >> 0x10);
    paVar5 = (Struct57*)puVar14;
    break;
  case 0x17:
    mem_op_1000_179c(0xec,param_1);
    uVar8 = param_1 | paVar5;
    paVar10 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar8);
    if (uVar8 == 0x0) goto LAB_1010_73fe;
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_a640((Struct57*)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,paVar5)),param_5,
                    paVar11->field7_0xe);
  }
  uVar8 = paVar10;
  puStack10 = (u32 *)CONCAT22(uVar8,paVar5);
  ppcVar3 = (code **)((int)*puStack10 + 0x8);
  paVar13 = paVar10;
  (**ppcVar3)(uVar12,paVar5,uVar8);
  if (param_4 != 0x17) {
    if (0x17 < param_4) goto LAB_1010_7710;
    cVar4 = (char)param_4;
    if ((cVar4 != '\x05') && (((char)(cVar4 + -0x5) < '\x05' || ('\x02' < (char)(cVar4 + -0xa))))) goto LAB_1010_7710;
  }
  if ((uStack6 + 0x16) != 0x0) {
    uVar12 = SUB42(s_tile2_bmp_1050_1538,0x0);
    uVar15 = SendMessage16(0x0,0xf8,0x111,*(HWND16 *)(uStack6 + 0x14));
    paVar13 = (Struct57*)((u32)paVar13 & 0xffff0000 | uVar15 >> 0x10);
  }
LAB_1010_7710:
  if (puStack10 != NULL) {
    uVar12 = SUB42(s_tile2_bmp_1050_1538,0x0);
    uVar6 = IsWindow16(paVar5->field3_0x6);
    if (uVar6 != 0x0) {
      if (*(i32 *)&paVar11->field_0x1c == 0x0) {
        mem_op_1000_179c(0xc,paVar13);
        uVar8 = paVar13;
        uVar9 = uVar8 | uVar6;
        paVar13 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)uVar9);
        if (uVar9 == 0x0) {
          (u32)&paVar11->field_0x1c = 0x0;
        }
        else {
          set_struct_1008_574a((Struct57*)CONCAT22(uVar8,uVar6));
          &paVar11->field_0x1c = uVar6;
          &paVar11->field_0x1e = (int)paVar13;
        }
      }
      mem_op_1000_179c(0xc,paVar13);
      uVar8 = paVar13;
      puStack14 = (u16 *)CONCAT22(uVar8,uVar6);
      if ((uVar8 | uVar6) == 0x0) {
        puStack20 = NULL;
      }
      else {
        *puStack14 = 0x389a;
        (uVar6 + 0x2) = 0x1008;
        (u32)(uVar6 + 0x4) = param_5;
        (u32)(uVar6 + 0x8) = puStack10;
        *puStack14 = 0x7e24;
        (uVar6 + 0x2) = 0x1010;
        puStack20 = puStack14;
      }
      uVar2 = (u32)&paVar11->field_0x1c;
      ppcVar3 = (code **)((int)*&paVar11->field_0x1c + 0x4);
      (**ppcVar3)(0x1000,(int)uVar2,(char)((u32)uVar2 >> 0x10),(int)puStack20,(int)((u32)puStack20 >> 0x10));
      return;
    }
  }
  if ((uVar8 | paVar5) != 0x0) {
    ppcVar3 = (code **)*puStack10;
    (**ppcVar3)(uVar12,paVar5,(char)paVar10,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1010_7818(param_1: u32,astruct_15 *param_2)

{
  u32 uVar1;
  u16 uVar2;
  BOOL16 BVar3;
  u16 uVar4;
  u16 uStack6;

  uVar4 = (param_1 >> 0x10);
  uVar1 = (u32)(param_1 + 0x14);
  uVar2 = pass1_1010_b028(uVar1,((u32)uVar1 >> 0x10),param_2);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x1e);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0xb);
    if (((BVar3 == 0x0) && (BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x20), BVar3 == 0x0)) &&
       (BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x1c), BVar3 == 0x0)) {
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x2);
      if ((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x13), BVar3 != 0x0)) {
        return 0x5;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x11);
      if ((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x12), BVar3 != 0x0)) {
        return 0x4;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x5);
      if (BVar3 != 0x0) {
        return 0x6;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x6);
      if (BVar3 != 0x0) {
        return 0x7;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x4);
      if (BVar3 != 0x0) {
        return 0x10;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x3);
      if (BVar3 != 0x0) {
        return 0x11;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x19);
      if (BVar3 != 0x0) {
        return 0x15;
      }
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x1d);
      if (BVar3 != 0x0) {
        return 0x16;
      }
      uVar2 = pass1_1010_7d7e(param_1,uVar4,0x1,uVar2);
      if (uVar2 == 0x0) {
        return 0x0;
      }
      return 0xc;
    }
    uStack6 = 0x1;
  }
  else {
    uStack6 = 0x18;
  }
  return uStack6;
}



void ui_op_1010_79aa(param_1: u32,i16 param_2,i32 param_3)

{
  HWND16 hwnd;
  u32 uVar1;
  u8 *puVar2;
  u16 DX_REG;
  u16 uVar3;
  let mut lStack18: i32;
  let mut lStack14: i32;
  u8 local_a [0x8];

  uVar3 = ((u32)param_1 >> 0x10);
  if ((*(i32 *)((int)param_1 + 0x1c) != 0x0) && ((param_3 != 0x0 || (param_2 != 0x0)))) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x1c));
    lStack18 = 0x0;
    do {
      puVar2 = local_a;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
      lStack14 = CONCAT22(DX_REG,puVar2);
      if ((DX_REG | puVar2) == 0x0) goto LAB_1010_7a49;
      if (((param_2 == 0x0) && (*(i32 *)(puVar2 + 0x4) == param_3)) ||
         ((param_3 == 0x0 && (uVar1 = (u32)(puVar2 + 0x8), ((int)uVar1 + 0xa) == param_2)))) break;
    } while ((*(i32 *)(puVar2 + 0x4) != param_3) ||
            (uVar1 = (u32)(puVar2 + 0x8), ((int)uVar1 + 0xa) != param_2));
    lStack18 = lStack14;
LAB_1010_7a49:
    if (lStack18 != 0x0) {
      uVar1 = (u32)((int)lStack18 + 0x8);
      hwnd = *(HWND16 *)((int)uVar1 + 0x6);
      SetFocus16(hwnd);
      BringWindowToTop16(hwnd);
      return;
    }
  }
  return;
}



void show_win_1010_7a76(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;
  let mut lVar4: i32;
  u8 local_a [0x8];

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0x20) == 0x0) {
    (iVar2 + 0x20) = 0x1;
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar2 + 0x1c));
    while( true ) {
      lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      if (lVar4 == 0x0) break;
      uVar1 = (u32)((int)lVar4 + 0x8);
      ShowWindow16(0x0,*(HWND16 *)((int)uVar1 + 0x6));
    }
  }
  return;
}



void show_window_1010_7ace(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;
  let mut lVar4: i32;
  u8 local_a [0x8];

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0x20) != 0x0) {
    (iVar2 + 0x20) = 0x0;
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar2 + 0x1c));
    while( true ) {
      lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      if (lVar4 == 0x0) break;
      uVar1 = (u32)((int)lVar4 + 0x8);
      ShowWindow16(0x1,*(HWND16 *)((int)uVar1 + 0x6));
    }
  }
  return;
}



u32 destroy_window_1010_7b26(u16 param_1,param_2: u32,i32 param_3)

{
  u32 uVar1;
  u16 uVar2;
  UCHAR *puVar2;
  u16 DX_REG;
  i16 iVar3;
  u16 uVar4;
  UCHAR local_a [0x8];

  uVar4 = (param_2 >> 0x10);
  iVar3 = (int)param_2;
  uVar2 = (iVar3 + 0x1e) | (iVar3 + 0x1c);
  if (uVar2 != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar3 + 0x1c));
    do {
      puVar2 = local_a;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
      param_1 = DX_REG | puVar2;
      if (param_1 == 0x0) break;
    } while (*(i32 *)(puVar2 + 0x4) != param_3);
    uVar2 = DX_REG | puVar2;
    if (uVar2 != 0x0) {
      uVar1 = (u32)(puVar2 + 0x8);
      uVar2 = DestroyWindow16(*(HWND16 *)((int)uVar1 + 0x6));
    }
  }
  return CONCAT22(uVar2,param_1);
}



void pass1_1010_7b8c(param_1: u32,param_2: i16)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u32 uVar4;
  u8 *puVar5;
  u16 DX_REG;
  i16 iVar6;
  u16 uVar7;
  u32 uStack14;
  u8 local_a [0x8];

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (((iVar6 + 0x1e) | (iVar6 + 0x1c)) != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar6 + 0x1c));
    do {
      puVar5 = local_a;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
      uStack14 = CONCAT22(DX_REG,puVar5);
      if ((DX_REG | puVar5) == 0x0) break;
      uVar4 = (u32)(puVar5 + 0x8);
    } while (((int)uVar4 + 0x6) != param_2);
    if ((DX_REG | puVar5) != 0x0) {
      ppcVar3 = (code **)((int)(u32)(u32)(iVar6 + 0x1c) + 0xc);
      (**ppcVar3)(0x1008,(u32)(iVar6 + 0x1c),uStack14);
    }
    uVar4 = (u32)(iVar6 + 0x1c);
    if (((int)uVar4 + 0x8) == 0x0) {
      puVar1 = (u32 *)(iVar6 + 0x1c);
      uVar2 = (iVar6 + 0x1e);
      if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1008,puVar1,uVar2,0x1,puVar1,uVar2,puVar1,uVar2);
      }
      (u32)(iVar6 + 0x1c) = 0x0;
    }
  }
  return;
}



void send_msg_1010_7c42(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;
  let mut lVar4: i32;
  u8 local_a [0x8];

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0x1e) | (iVar2 + 0x1c)) != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar2 + 0x1c));
    while( true ) {
      lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      if (lVar4 == 0x0) break;
      uVar1 = (u32)((int)lVar4 + 0x8);
      SendMessage16(0x0,0xeb,0x111,*(HWND16 *)((int)uVar1 + 0x6));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void send_msg_1010_7c9e(param_1: u32,param_2: i16)

{
  u32 uVar1;
  u16 uVar2;
  BOOL16 BVar3;
  u16 uVar4;
  i16 iVar5;
  u16 uVar6;
  let mut lVar7: i32;
  u32 uVar8;
  u8 local_a [0x8];

  uVar6 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  if ((((iVar5 + 0x1e) | (iVar5 + 0x1c)) != 0x0) && (param_2 != 0x0)) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar5 + 0x1c));
    while( true ) {
      lVar7 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      uVar4 = ((u32)lVar7 >> 0x10);
      uVar2 = lVar7;
      if (lVar7 == 0x0) break;
      if (*(i32 *)(uVar2 + 0x4) != 0x0) {
        uVar8 = struct_op_1030_73a8(*(astruct_419 **)(uVar2 + 0x4),uVar2,uVar4 | uVar2);
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar8 + 0xc),param_2);
        if (BVar3 != 0x0) {
          uVar1 = (u32)(uVar2 + 0x8);
          SendMessage16(0x0,0xeb,0x111,*(HWND16 *)((int)uVar1 + 0x6));
        }
      }
    }
  }
  return;
}



u16 pass1_1010_7d38(u16 param_1,u16 param_2,i16 param_3,u16 param_4)

{
  u32 local_e;
  u16 uStack10;
  u16 local_8;
  u8 local_6 [0x2];
  u8 local_4 [0x2];

  local_e = (u32)(param_3 + 0xc);
  uStack10 = (param_3 + 0x10);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_e),(u16 *)CONCAT22(0x1050,&local_8),
                  (u16 *)CONCAT22(0x1050,local_6),(u16 *)CONCAT22(0x1050,local_4));
  return local_8;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1010_7d7e(u16 param_1,u16 param_2,i16 param_3,param_4: i16)

{
  BOOL16 BVar1;

  if (param_3 != 0x3) {
    if ((param_4 < 0xa) || (0x7f < param_4)) {
      return 0x0;
    }
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,param_4,0x3c);
    if (BVar1 != 0x0) {
      return 0x0;
    }
    if (((param_4 == 0x6a) && (param_3 != 0x4)) && (param_3 != 0x5)) {
      return 0x0;
    }
  }
  return 0x1;
}



StructD * pass1_1010_7dc6(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xa));
  pass1_1010_6bb2((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_7dd2(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1010_7dfe(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1010_6bb2(&param_2->address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_7e40(u8 *param_1,astruct_652 *param_2)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  astruct_652 *struct_1;
  u16 uVar2;
  u32 *puVar3;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000fff6;
  u32 uVar1;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  uVar2 = ((u32)param_2 >> 0x10);
  struct_1 = (astruct_652 *)param_2;
  param_2->field0_0x0 = 0x0;
  struct_1->field1648_0x67c = 0x0;
  struct_1->field1649_0x680 = 0x0;
  struct_1->field3698_0xe82 = 0x0;
  struct_1->field3699_0xe84 = 0x0;
  (u32)&struct_1->field3700_0xe88 = 0x0;
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1->field1_0x4)),NULL,0x228);
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1->field550_0x22c)),NULL,0x228);
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1->field1099_0x454)),NULL,0x228);
  struct_1->field_0x682 = 0x0;
  struct_1->field_0xa82 = 0x0;
  _u16_1050_14cc = param_2;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff6,0x2),in_stack_0000fe9e,
                           in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
  struct_1->field3700_0xe88 = (i16)puVar3;
  struct_1->field3701_0xe8a = (i16)((u32)puVar3 >> 0x10);
  uVar1 = (u32)&struct_1->field3700_0xe88;
  struct_1->field3699_0xe84 = (u32)((int)uVar1 + 0x64);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_7efc(u32 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  u32 *puVar3;
  code **ppcVar4;
  astruct_448 *iVar5;
  u16 uVar5;
  u16 unaff_CS;
  char *pcStack8;
  i16 iStack4;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_448 *)param_1;
  uVar1 = iVar5->field1660_0x67c;
  uVar2 = iVar5->field1661_0x67e;
  pcStack8 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(uVar2,uVar1));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcStack8);
  }
  for (iStack4 = 0x0; iStack4 < 0x8a; iStack4 += 0x1) {
    puVar3 = (&iVar5->field_0x4 + iStack4 * 0x4);
    uVar1 = (&iVar5->field_0x6 + iStack4 * 0x4);
    if ((uVar1 | puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
    }
    puVar3 = (&iVar5->field_0x22c + iStack4 * 0x4);
    uVar1 = (&iVar5->field_0x22e + iStack4 * 0x4);
    if ((uVar1 | puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(unaff_CS,puVar3);
    }
    puVar3 = (&iVar5->field_0x454 + iStack4 * 0x4);
    uVar1 = (&iVar5->field_0x456 + iStack4 * 0x4);
    if ((uVar1 | puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(unaff_CS,puVar3);
    }
  }
  fn_ptr_1000_17ce((char *)*param_1);
  _u16_1050_14cc = 0x0;
  return;
}



void pass1_1010_7fd6(astruct_489 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  astruct_489 *iVar3;
  astruct_489 *uVar3;
  char *pcStack6;

  uVar3 = (astruct_489 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_489 *)param_1;
  uVar1 = iVar3->field1660_0x67c;
  uVar2 = iVar3->field1661_0x67e;
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  (u32)&iVar3->field1660_0x67c = 0x0;
  iVar3->field1662_0x680 = 0x0;
  return;
}



void pass1_1010_8018(u8 *param_1,param_2: u32,u16 param_3)

{
  i16 iVar1;
  u16 *uVar2;

  if ((param_3 * 0xa + 0x1fa0) != 0x0) {
    pass1_1010_878c((astruct_87 **)param_2,(param_3 * 0xa + 0x1fa0));
    uVar2 = (u16 *)(param_2 >> 0x10);
    if (*(i32 *)((int)param_2 + 0x67c) != 0x0) {
      iVar1 = param_3 * 0xa;
      pass1_1008_64c8(param_3,param_1,((int)param_2 + 0x67c),
                      CONCAT22((iVar1 + 0x1fa6),(iVar1 + 0x1fa8)),(iVar1 + 0x1fa4)
                     );
      return;
    }
  }
  return;
}



void pass1_1010_8096(u32 *param_1,param_2: i16)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  char *pcVar5;
  u16 *puVar6;
  u8 local_306 [0x100];
  u8 local_206 [0x100];
  u8 local_106 [0x104];

  uVar4 = ((u32)param_1 >> 0x10);
  uVar3 = param_1;
  str_1000_4d58(*(char **)((uVar3 + 0xe82) * 0x4 + 0x2526),NULL,0x0,CONCAT22(0x1050,local_206),
                (WNDCLASS16 *)CONCAT22(0x1050,local_306));
  unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_106),(char *)CONCAT22(0x1050,local_206));
  if (param_2 == 0x2) {
    puVar6 = &u16_1050_3194;
  }
  else {
    puVar6 = &u16_1050_3196;
  }
  pass1_1000_3cea(CONCAT22(0x1050,local_106),(char *)puVar6);
  pass1_1000_3cea(CONCAT22(0x1050,local_106),(char *)CONCAT22(0x1050,local_306));
  pcVar5 = (char *)set_err_mode_1010_8b14((u32)param_1,CONCAT22(0x1050,local_106));
  uVar2 = ((u32)pcVar5 >> 0x10);
  if (((u8 *)pcVar5 == local_106) && (uVar2 == 0x1050)) {
    msg_box_op_1010_8bb4(uVar3,uVar4,(u32)pcVar5 & 0xffff | 0x10500000);
  }
  fn_ptr_1000_17ce((char *)*param_1);
  uVar1 = str_op_1008_60e8(uVar2,pcVar5);
  param_1 = uVar1;
  (uVar3 + 0x2) = uVar2;
  return;
}



void pass1_1010_8170(u8 *param_1,astruct_87 *param_2,param_3: i16)

{
  i16 iVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = ((u32)param_2 >> 0x10);
  iVar2 = (int)param_2;
  iVar1 = (iVar2 + 0x680);
  iVar3 = param_3 * 0x10;
  if ((iVar3 + 0x16) != iVar1) {
    pass1_1010_8096((u32 *)param_2,(iVar3 + 0x16));
    pass1_1010_878c((astruct_87 **)param_2,(iVar3 + 0x16));
    if (*(i32 *)(iVar2 + 0x67c) == 0x0) {
      return;
    }
  }
  iVar3 = param_3 * 0x10;
  pass1_1008_6562((Struct57*)CONCAT22(iVar1,param_1),*(Struct76 **)(iVar2 + 0x67c),
                  CONCAT22((iVar3 + 0x1c),(iVar3 + 0x1e)),(iVar3 + 0x1a));
  return;
}



void pass1_1010_81f6(astruct_87 **param_1,i32 param_2,param_3: i16)

{
  u32 in_EDX;
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uStack12;
  u32 uStack10;

  if (param_2 == 0x8000001) {
    uVar1 = in_EDX & 0xffff0000 | (u32)param_1;
    uStack10 = (u32)param_1 & 0xffff0000 | (u32)(param_1 + 0x22c);
    uStack12 = 0xfa;
  }
  else if (param_2 == 0x8000002) {
    uVar1 = in_EDX & 0xffff0000 | (u32)param_1;
    uStack10 = (u32)param_1 & 0xffff0000 | (u32)(param_1 + 0x454);
    uStack12 = 0xfc;
  }
  else {
    uVar1 = in_EDX & 0xffff0000 | (u32)param_1;
    uStack10 = (u32)param_1 & 0xffff0000 | (u32)(param_1 + 0x4);
    uStack12 = 0x2;
  }
  uVar2 = (uStack10 >> 0x10);
  uVar3 = uVar1;
  if (*(i32 *)(param_3 * 0x4 + (int)uStack10) == 0x0) {
    if ((0x0 < param_3) && (param_3 < 0xa)) {
      pass1_1010_89f0(param_1,uVar3,uStack12,uStack10);
      return;
    }
    if (param_3 < 0xa) {
      return;
    }
    if (0x7f < param_3) {
      return;
    }
    if (*(i32 *)((int)uStack10 + 0x14) == 0x0) {
      pass1_1010_89f0(param_1,uVar3,uStack12,uStack10);
    }
    pass1_1010_887a(param_1,uStack10,param_3,uVar1);
    uVar3 = param_1;
  }
  pass1_1010_866c(uVar1,param_1,uVar3,uStack10,param_3);
  return;
}



void pass1_1010_82f8(param_1: u32,u16 param_2)

{
  ((int)param_1 + 0xe82) = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps

u16 FUN_1010_830a(u16 param_1,param_2: u32,u16 param_3,astruct_87 *param_4,param_5: i16)

{
  u16 uVar2;
  astruct_81 *puVar2;
  astruct_81 *paVar3;
  u16 uVar4;
  Struct57*paVar5;
  i16 iVar6;
  u16 unaff_SS;
  u16 unaff_DS;
  char *pcVar7;
  u32 uVar8;
  astruct_81 local_2e;
  i16 iStack10;
  i16 iStack8;
  u32 uStack6;
  u16 uVar1;
  astruct_87 *uVar9;
  astruct_87 *uVar10;

  uStack6 = 0x0;
  uVar1 = 0x63f0;
  iVar6 = param_5 * 0x10;
  uVar9 = (astruct_87 *)param_4;
  uVar10 = (astruct_87 *)((u32)param_4 >> 0x10);
  if ((iVar6 + 0x10) == 0x1) {
    pcVar7 = (char *)set_err_mode_1010_8b14((u32)param_4,(u32)(iVar6 + 0x12));
    paVar5 = (Struct57*)(param_2 & 0xffff0000 | (u32)pcVar7 >> 0x10);
    iStack10 = (int)pcVar7;
    iStack8 = (int)((u32)pcVar7 >> 0x10);
    uVar2 = 0x63f0;
    if (((iVar6 + 0x12) == iStack10) && ((iVar6 + 0x14) == iStack8)) {
      msg_box_op_1010_8bb4(uVar9,uVar10,(u32)pcVar7);
      return 0x0;
    }
    puVar2 = &local_2e;
    struct_op_1008_48fe(paVar5,(astruct_81 *)CONCAT22(unaff_SS,puVar2),0x1,pcVar7);
    mem_op_1000_179c(0x1e,paVar5);
    uVar4 = paVar5 | puVar2;
    if (uVar4 == 0x0) {
      paVar3 = NULL;
      uVar4 = 0x0;
    }
    else {
      paVar3 = &local_2e;
      struct_op_1008_3f92((Struct76 *)CONCAT22(paVar5,puVar2),(char *)CONCAT22(unaff_SS,paVar3));
    }
    uStack6 = CONCAT22(uVar4,paVar3);
    close_file_1008_496c((astruct_803 *)CONCAT22(unaff_SS,&local_2e));
  }
  else {
    if ((param_5 * 0x10 + 0x10) == 0x2) {
      pass1_1010_878c((astruct_87 **)param_4,(param_5 * 0x10 + 0x16));
      if (*(i32 *)&uVar9->field1660_0x67c == 0x0) {
        return 0x0;
      }
      uVar2 = 0x63f0;
      iVar6 = param_5 * 0x10;
      pass1_1008_6562((Struct57*)(param_2 & 0xffff | (u32)param_1 << 0x10),*(Struct76 **)&uVar9->field1660_0x67c,
                      CONCAT22((iVar6 + 0x1c),(iVar6 + 0x1e)),(iVar6 + 0x1a));
    }
    else {
      iVar6 = param_5 * 0x10;
      if ((iVar6 + 0x10) == 0x3) {
        uVar8 = set_err_mode_1010_8b14((u32)param_4,(u32)(iVar6 + 0x12));
        local_2e.field1_0x2 = (int)(uVar8 >> 0x10);
        param_1 = uVar8;
        uVar2 = 0x63f0;
        if (((iVar6 + 0x12) == param_1) && ((iVar6 + 0x14) == local_2e.field1_0x2)) {
          local_2e.field0_0x0 = param_1;
          msg_box_op_1010_8bb4(uVar9,uVar10,uVar8);
          param_1 = local_2e.field0_0x0;
        }
      }
      else {
        if ((param_5 * 0x10 + 0x10) != 0x4) goto LAB_1010_8473;
        uVar8 = set_err_mode_1010_8b14((u32)param_4,(u32)(param_5 * 0x10 + 0x12));
        param_1 = uVar8;
      }
    }
    uStack6 = (u32)param_1;
  }
LAB_1010_8473:
  return uStack6;
}



char * load_string_1010_847e(param_1: u32,u16 param_2)

{
  LoadString16(0x3ff,(char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x682U)),param_2,HINSTANCE16_1050_038c);
  return (char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x682U));
}



void load_string_1010_84ac(i16 param_1,INT16 param_2,u16 param_3)

{
  u16 uVar1;

  uVar1 = param_2;
  LoadString16(0x3ff,(char *)CONCAT22(param_2,param_1 + 0x682),param_3,HINSTANCE16_1050_038c);
  str_op_1008_60e8(uVar1,(char *)CONCAT22(param_2,param_1 + 0x682));
  return;
}



void load_string_1010_84e0(u16 param_1,u16 param_2,u16 in_resc_id_3,char *in_buffer_4,short in_buf_len_5)

{
  u16 in_stack_0000000e;

  LoadString16(in_resc_id_3,(char *)CONCAT22(in_buf_len_5,in_buffer_4),in_stack_0000000e,HINSTANCE16_1050_038c);
  return;
}



// WARNING: Could not reconcile some variable overlaps



void pass1_1010_85be(param_1: u32,i16 param_2,param_3: i16)

{
  u32 uVar1;
  u8 local_30a [0x100];
  u8 local_20a [0x100];
  u8 local_10a [0x108];

  if (param_2 == 0x2) {
    uVar1 = (u32)(param_3 * 0x4 + 0x2e34);
    str_1000_4d58((char *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x3)),NULL,0x0,CONCAT22(0x1050,local_20a),
                  (WNDCLASS16 *)CONCAT22(0x1050,local_30a));
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_10a),s_male_1050_14c6);
    pass1_1000_3cea(CONCAT22(0x1050,local_10a),(char *)CONCAT22(0x1050,local_20a));
    pass1_1000_3cea(CONCAT22(0x1050,local_10a),(char *)CONCAT22(0x1050,local_30a));
    set_err_mode_1010_8b14(param_1,CONCAT22(0x1050,local_10a));
    return;
  }
  set_err_mode_1010_8b14(param_1,(u32)(param_3 * 0x4 + 0x2e34));
  return;
}



void pass1_1010_866c(u16 param_1,u16 param_2,u16 param_3,param_4: u32,u16 param_5)

{
  astruct_828 *paVar1;
  char cVar2;
  i16 iVar3;
  bool bVar4;

  if ((int)param_5 < 0x28) {
    if (((int)param_5 < 0x25) && (param_5 != 0x23)) {
      if (0x23 < param_5) {
        return;
      }
      cVar2 = (char)param_5;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else if (param_5 != 0x37) {
    if ((int)param_5 < 0x38) {
      if ((int)param_5 < 0x33) {
        return;
      }
      bVar4 = SBORROW2(param_5 - 0x33,0x1);
      iVar3 = param_5 - 0x34;
    }
    else {
      if (param_5 == 0x49) goto LAB_1010_8691;
      if ((int)(param_5 - 0x49) < 0x2a) {
        return;
      }
      bVar4 = SBORROW2(param_5 - 0x73,0x5);
      iVar3 = param_5 - 0x78;
    }
    if (iVar3 != 0x0 && bVar4 == iVar3 < 0x0) {
      return;
    }
  }
LAB_1010_8691:
  paVar1 = *(astruct_828 **)(param_5 * 0x4 + (int)param_4);
  memcpy_op_1008_676e((astruct_830 *)paVar1,param_1,paVar1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_86de(u16 param_1,u16 param_2,u8 param_3,Struct76 *param_4)

{
  i32 *plVar1;
  i16 iVar2;
  bool bVar3;
  u16 uVar4;
  let mut lVar5: i32;
  u32 uVar6;
  let mut lStack20: i32;
  u32 uStack10;

  uVar6 = pass1_1008_4772(param_4);
  uVar4 = (uVar6 >> 0x10);
  uStack10 = 0x0;
  do {
    plVar1 = (i32 *)((int)uVar6 + 0x8);
    if (*plVar1 == uStack10 || *plVar1 < uStack10) {
      return;
    }
    lVar5 = uStack10;
    pass1_1008_4544(param_4);
    iVar2 = (int)lVar5;
    bVar3 = false;
    for (lStack20 = 0x0; plVar1 = (i32 *)((int)uVar6 + 0x4), *plVar1 != lStack20 && lStack20 <= *plVar1;
        lStack20 += 0x1) {
      if (bVar3) {
LAB_1010_86fc:
        if (bVar3) {
          if (*(char *)((int)lStack20 + iVar2) == -0x1) {
            *(u8 *)((int)lStack20 + iVar2) = param_3;
            break;
          }
        }
      }
      else {
        if (*(char *)((int)lStack20 + iVar2) == -0x1) goto LAB_1010_86fc;
        *(u8 *)((int)lStack20 + iVar2 + -0x1) = param_3;
        bVar3 = true;
      }
    }
    uStack10 += 0x1;
  } while( true );
}



void pass1_1010_878c(astruct_87 **param_1,param_2: i16)

{
  u16 uVar4;
  u16 uVar1;
  u8 *puVar2;
  u32 in_EDX;
  Struct57*paVar3;
  astruct_87 *uVar6;
  i16 iVar4;
  u16 uVar5;
  astruct_87 *paVar6;
  char *pcStack6;

  uVar5 = ((u32)param_1 >> 0x10);
  uVar6 = (astruct_87 *)param_1;
  if (uVar6->field1662_0x680 == param_2) {
    return;
  }
  uVar4 = uVar6->field1660_0x67c;
  puVar2 = uVar6->field1661_0x67e;
  pcStack6 = (char *)CONCAT22(puVar2,uVar4);
  paVar3 = (Struct57*)(in_EDX & 0xffff0000 | (u32)(puVar2 | uVar4));
  if ((puVar2 | uVar4) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(puVar2,uVar4));
    fn_ptr_1000_17ce(pcStack6);
  }
  if ((param_2 == 0x1) || (param_2 == 0x2)) {
    mem_op_1000_179c(0x8,paVar3);
    uVar1 = paVar3;
    paVar3 = (Struct57*)((u32)paVar3 & 0xffff0000 | (u32)(uVar1 | uVar4));
    if ((uVar1 | uVar4) == 0x0) {
      (u32)&uVar6->field1660_0x67c = 0x0;
      goto LAB_1010_8869;
    }
    paVar6 = *param_1;
LAB_1010_8853:
    file_1008_6414(paVar3,(u32 *)CONCAT22(uVar1,uVar4),(char *)paVar6);
    puVar2 = (u8 *)paVar3;
  }
  else {
    iVar4 = param_2 * 0x4;
    paVar6 = (astruct_87 *)set_err_mode_1010_8b14((u32)param_1,(u32)(iVar4 + 0x172a));
    paVar3 = (Struct57*)((u32)paVar3 & 0xffff0000 | (u32)paVar6 >> 0x10);
    uVar4 = paVar6;
    uVar1 = ((u32)paVar6 >> 0x10);
    if (((iVar4 + 0x172a) == uVar4) && ((iVar4 + 0x172c) == uVar1)) {
      msg_box_op_1010_8bb4(uVar6,uVar5,(u32)paVar6 & 0xffff | (u32)uVar1 << 0x10);
    }
    mem_op_1000_179c(0x8,paVar3);
    uVar1 = paVar3;
    paVar3 = (Struct57*)((u32)paVar3 & 0xffff0000 | (u32)(uVar1 | uVar4));
    if ((uVar1 | uVar4) != 0x0) goto LAB_1010_8853;
    uVar4 = 0x0;
    puVar2 = NULL;
  }
  uVar6->field1660_0x67c = uVar4;
  uVar6->field1661_0x67e = puVar2;
LAB_1010_8869:
  uVar6->field1662_0x680 = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_887a(astruct_87 **param_1,param_2: u32,i16 param_3,u32 param_4)

{
  u16 uVar1;
  u32 uVar2;
  u32 uVar3;
  u16 uVar4;
  Struct57*paVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u8 bVar10;
  u8 uVar11;
  u8 local_26 [0x6];
  u16 uStack32;
  u16 uStack30;
  Struct76 *paStack28;
  u32 uStack24;
  u32 uStack20;
  u32 uStack16;
  Struct76 *paStack12;
  Struct76 *paStack8;
  u16 uStack4;

  uStack4 = param_3 - 0xa;
  pass1_1010_878c(param_1,(uStack4 * 0xa + 0x3382));
  uVar7 = ((u32)param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x67c) != 0x0) {
    iVar6 = uStack4 * 0xa;
    uVar1 = uStack4;
    pass1_1008_6562((Struct57*)(param_4 & 0xffff | (u32)uStack4 << 0x10),*(Struct76 **)((int)param_1 + 0x67c),
                    CONCAT22((iVar6 + 0x3388),(iVar6 + 0x338a)),(iVar6 + 0x3386));
    paStack8 = (Struct76 *)CONCAT22((int)param_4,uVar1);
    uVar8 = ((u32)param_2 >> 0x10);
    paStack12 = *(Struct76 **)((int)param_2 + 0x14);
    uStack16 = pass1_1008_4772(paStack12);
    uVar2 = param_4 & 0xffff0000;
    uStack20 = pass1_1008_4772(paStack8);
    paVar5 = (Struct57*)(uVar2 & 0xffff0000 | uStack20 >> 0x10);
    uVar7 = (uStack20 >> 0x10);
    uVar2 = (u32)((int)uStack20 + 0x4);
    uVar9 = (uStack16 >> 0x10);
    iVar6 = (int)uStack16;
    if ((long)uVar2 < *(i32 *)(iVar6 + 0x4)) {
      uVar2 = (u32)(iVar6 + 0x4);
    }
    uVar3 = (u32)((int)uStack20 + 0x8);
    if ((long)uVar3 < *(i32 *)(iVar6 + 0x8)) {
      uVar3 = (u32)(iVar6 + 0x8);
    }
    uVar1 = uVar3;
    uStack24 = uVar3 & 0xffff | uVar2 << 0x10;
    bVar10 = 0xff;
    uVar11 = 0x0;
    mem_op_1000_179c(0x1e,paVar5);
    uVar4 = paVar5 | uVar1;
    if (uVar4 == 0x0) {
      uVar1 = 0x0;
      uVar4 = 0x0;
    }
    else {
      struct_op_1008_6604((Struct76 *)CONCAT22(paVar5,uVar1),(int)uStack24,
                          CONCAT13(uVar11,CONCAT12(bVar10,(int)(uStack24 >> 0x10))));
    }
    paStack28 = (Struct76 *)CONCAT22(uVar4,uVar1);
    pass1_1008_431c((Struct76 *)CONCAT22(uVar4,uVar1),bVar10);
    uVar7 = (uStack16 >> 0x10);
    uStack30 = (uStack24 - ((int)uStack16 + 0x4)) / 0x2;
    uStack32 = (int)uStack24 - ((int)uStack16 + 0x8);
    pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_26),0x0,uStack32,uStack30);
    pass1_1008_4480(paStack28,(u16 *)CONCAT22(0x1050,local_26),paStack12);
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,local_26),0x0,0x0,0x7);
    pass1_1008_4480(paStack28,(u16 *)CONCAT22(0x1050,local_26),paStack8);
    *(Struct76 **)(param_3 * 0x4 + (int)param_2) = paStack28;
  }
  return;
}



void pass1_1010_89f0(u16 param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u16 uVar1;
  u16 uVar2;
  u8 *puVar3;
  u32 in_EDX;
  Struct57*paVar4;
  i16 iVar7;
  char *pcVar8;
  u32 uStack22;
  char *pcStack12;
  u16 uStack8;
  Struct57*paVar5;
  u16 uVar6;

  uVar1 = (param_1 + 0x67c);
  uVar2 = (param_1 + 0x67e);
  uVar6 = ((u32)in_EDX >> 0x10);
  pcStack12 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack12);
  }
  pcVar8 = (char *)set_err_mode_1010_8b14(CONCAT22(param_2,param_1),(u32)((param_1 + 0xe82) * 0x4 + 0x24be));
  uVar2 = ((u32)pcVar8 >> 0x10);
  paVar4 = (Struct57*)CONCAT22(uVar6,uVar2);
  uVar1 = pcVar8;
  iVar7 = (param_1 + 0xe82) * 0x4;
  if (((iVar7 + 0x24be) == uVar1) && ((iVar7 + 0x24c0) == uVar2)) {
    msg_box_op_1010_8bb4(param_1,param_2,(u32)pcVar8 & 0xffff | (u32)uVar2 << 0x10);
  }
  mem_op_1000_179c(0x8,paVar4);
  uVar2 = paVar4 | uVar1;
  paVar5 = (Struct57*)((u32)paVar4 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    uVar1 = 0x0;
    paVar5 = NULL;
  }
  else {
    file_1008_6414(paVar5,(u32 *)CONCAT22(paVar4,uVar1),pcVar8);
  }
  (param_1 + 0x67c) = uVar1;
  (param_1 + 0x67e) = (int)paVar5;
  (param_1 + 0x680) = 0x0;
  if (((param_1 + 0x67e) | (param_1 + 0x67c)) != 0x0) {
    for (uStack8 = 0x1; puVar3 = (u8 *)paVar5, (int)uStack8 < 0xa; uStack8 += 0x1) {
      iVar7 = uStack8 * 0xa;
      uVar1 = uStack8;
      pass1_1008_64c8(uStack8,puVar3,(param_1 + 0x67c),
                      CONCAT22((iVar7 + 0x2558),(iVar7 + 0x255a)),(iVar7 + 0x2556)
                     );
      uStack22 = CONCAT22(puVar3,uVar1);
      pass1_1010_86de(param_1,param_2,(u8)param_3,(Struct76 *)CONCAT22(puVar3,uVar1));
      paVar5 = (Struct57*)ZEXT24(puVar3);
      (u32)(uStack8 * 0x4 + (int)param_4) = uStack22;
    }
  }
  return;
}





// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_op_1010_8bb4(u16 param_1,u16 param_2,u32 param_3)

{
  char *pcVar1;
  u8 local_402 [0x400];

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,0x3fa);
  unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_402),pcVar1);
  pass1_1000_3cea(CONCAT22(0x1050,local_402),(char *)param_3);
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,0x57b);
  MessageBox16(0x1010,pcVar1,(char *)CONCAT22(0x1050,local_402),HWND16_1050_0396);
  PostMessage16(0x0,0xee,0x111,HWND16_1050_0396);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_8c32(astruct_19 *param_1,u16 param_2)

{
  u32 in_EDX;
  u16 uVar1;
  u16 unaff_BP;
  astruct_19 *paVar2;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  uVar1 = ((u32)in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
    //        1010:8ee2  bc  8e  10  10      addr         pass1_1010_8ebc
  param_1->offset_0x0 = 0x8ee2;
  ((int)param_1 + 0x2) = 0x1010;
  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(uVar1,(int)((u32)paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  ((int)param_1 + 0xa) = (int)puVar3;
  ((int)param_1 + 0xc) = (int)((u32)puVar3 >> 0x10);
  return (u32)param_1;
}



void pass1_1010_8c78(u16 *param_1)

{
  *param_1 = 0x8ee2;
  ((int)param_1 + 0x2) = 0x1010;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 unk_load_str_op_1010_8c96(u8 *param_1,param_2: u32,param_3: u32,u32 param_4)

{
  u32 uVar1;
  INT16 IVar2;
  u32 *puVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  astruct_15 *paVar9;
  u32 uVar10;
  char *pcVar11;
  WORD *valist;
  short in_buf_len_5;
  u8 *puVar12;
  u32 uStack46;
  u32 local_10;
  i16 iStack12;
  i16 iStack10;
  u8 *puStack8;
  u16 uStack6;
  u16 uStack4;

  uVar7 = (param_4 >> 0x10);
  iVar6 = (int)param_4;
  uVar5 = (iVar6 + 0x6);
  uVar10 = (u32)uVar5;
  valist = (WORD *)param_3;
  in_buf_len_5 = (short)(param_3 >> 0x10);
  if (uVar5 != 0x0) {
    uVar8 = (param_2 >> 0x10);
    if (uVar5 == 0x1) {
      uVar10 = param_4 & 0xffff;
      iVar4 = (int)uVar10;
      switch((iVar6 + 0x4) + -0x1) {
      case 0x0:
      case 0x1:
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar6 + 0x8));
        local_10 = (u32)(iVar4 + 0xc);
        iStack12 = (iVar4 + 0x10);
        iStack10 = iVar4;
        puStack8 = param_1;
        if (0x0 < iStack12) {
          pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x437);
          uStack4 = ((u32)pcVar11 >> 0x10);
          uStack6 = SUB42(pcVar11,0x0);
          IVar2 = wsprintf16(valist,(char *)CONCAT22(uStack6,in_buf_len_5),(char *)CONCAT22(iStack12,uStack4));
          return CONCAT22(IVar2,uStack4);
        }
        break;
      case 0x2:
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar6 + 0x8));
        local_10 = (u32)(iVar4 + 0xc);
        iStack12 = (iVar4 + 0x10);
        if (0x0 < iStack12) {
          iStack12 = 0x0;
          paVar9 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(param_1,iVar4),&local_10,param_1);
          uVar10 = pass1_1028_bb24(paVar9);
          param_1 = (u8 *)(uVar10 >> 0x10);
          iStack10 = (int)uVar10;
          puVar3 = &local_10;
          puStack8 = param_1;
          pass1_1030_627e(puVar3,param_1,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar3),uVar10);
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar3));
          uVar1 = (u32)((int)param_2 + 0xa);
          pass1_1010_c3c2(param_1,uVar1,((u32)uVar1 >> 0x10),0x0,CONCAT22(param_1,puVar3));
          uStack46 = CONCAT22(param_1,puVar3);
          puVar12 = param_1;
          pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x439);
          uStack4 = ((u32)pcVar11 >> 0x10);
          uStack6 = SUB42(pcVar11,0x0);
          wsprintf16(valist,(char *)CONCAT22(uStack6,in_buf_len_5),(char *)CONCAT22(puVar3,uStack4),puVar12);
          goto LAB_1010_8def;
        }
        break;
      default:
        goto switchD_1010_8e11_caseD_4;
      case 0x4:
      case 0x7:
      case 0x8:
      case 0xa:
        goto LAB_1010_8ea5;
      }
      uVar10 = ZEXT24(&local_10);
    }
    else {
      uVar10 = (u32)(uVar5 - 0x2);
      if (uVar5 - 0x2 == 0x0) {
        iVar4 = (iVar6 + 0x4);
        uVar5 = iVar4 - 0x4;
        if (uVar5 != 0x0) {
          uVar5 = iVar4 - 0xc;
          uVar10 = (u32)uVar5;
          if (uVar5 != 0x0) goto LAB_1010_8ea5;
        }
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar6 + 0x8));
        uVar1 = (u32)((int)param_2 + 0xa);
        pass1_1010_c3c2(param_1,uVar1,((u32)uVar1 >> 0x10),0x0,CONCAT22(param_1,uVar5));
        uStack46 = CONCAT22(param_1,uVar5);
        puVar12 = param_1;
        pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x43b);
        uStack4 = ((u32)pcVar11 >> 0x10);
        uStack6 = SUB42(pcVar11,0x0);
        wsprintf16(valist,(char *)CONCAT22(uStack6,in_buf_len_5),(char *)CONCAT22(uVar5,uStack4),puVar12);
LAB_1010_8def:
        fn_ptr_1000_17ce((char *)(uStack46 & 0xffff | ZEXT24(param_1) << 0x10));
        return CONCAT22((int)uStack46,param_1);
      }
    }
  }
LAB_1010_8ea5:
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(char *)valist,in_buf_len_5);
switchD_1010_8e11_caseD_4:
  return CONCAT22((int)uVar10,param_1);
}



u32 pass1_1010_8ebc(param_1: u32,u8 param_2)

{
  pass1_1010_8c78((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_8ef2(Struct57*param_1,astruct_170 *param_2,u16 param_3,u16 param_4,u16 param_5,
                    u16 param_6,u16 param_7)

{
  u16 uVar1;
  u16 uVar2;
  astruct_170 *iVar3;
  u16 uVar4;
  u32 *puVar5;
  Struct57*paVar3;

  uVar4 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_170 *)param_2;
  param_2->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  uVar1 = 0x0;
  (u32)&iVar3->field2_0x4 = 0x0;
  (u32)&iVar3->field4_0x8 = 0x0;
  param_2->field0_0x0 = 0x9254;
  iVar3->field1_0x2 = 0x1010;
  mem_op_1000_179c(0x18,param_1);
  uVar2 = param_1 | uVar1;
  paVar3 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    (u32)&iVar3->field2_0x4 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_1,uVar1),0x5,0x5);
    iVar3->field2_0x4 = uVar1;
    iVar3->field3_0x6 = (u8 *)paVar3;
  }
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3),param_4,param_5,param_6,param_7);
  iVar3->field4_0x8 = (int)puVar5;
  iVar3->field5_0xa = (int)((u32)puVar5 >> 0x10);
  return;
}



void pass1_1010_8f78(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  StructD *iVar4;
  StructD *uVar4;

  uVar4 = (StructD *)((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x9254;
  iVar4->address_offset_field_0x2 = 0x1010;
  puVar1 = (u32 *)iVar4->hfile_0x4;
  uVar2 = &iVar4->field_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar4->address_offset_field_0x2 = 0x1008;
  return;
}



void pass1_1010_8fba(u16 param_1,astruct_411 *param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u16 DX_REG;
  u16 DX_REG_00;
  astruct_411 *iVar3;
  u16 uVar3;
  u32 uStack14;
  u32 uStack10;

  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_411 *)param_2;
  ppcVar1 = (code **)((int)*iVar3->field4_0x4 + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(DX_REG,param_1);
  uStack14 = 0x0;
  while( true ) {
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar1 = (code **)((int)*iVar3->field4_0x4 + 0x4);
    uVar2 = uStack10;
    (**ppcVar1)();
    if ((DX_REG_00 | uVar2) != 0x0) break;
    uStack14 += 0x1;
  }
  ppcVar1 = (code **)((int)*iVar3->field4_0x4 + 0x8);
  (**ppcVar1)();
  return;
}



void pass1_1010_9044(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}



void fn_ptr_1010_905e(astruct_169 *param_1,u32 param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_169 *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_169 *)param_1;
  puVar1 = &iVar4->field4_0x4;
  uVar2 = ((int)&iVar4->field4_0x4 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4->field4_0x4 = param_2;
  return;
}



void pass1_1010_9092(u16 param_1,u32 param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  Struct57*in_EDX;
  i16 iVar7;
  u16 uVar8;
  u16 in_stack_0000fe7c;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffd4;
  u32 uVar9;
  u32 uStack14;
  u32 uStack6;
  Struct57*paVar6;

  uVar8 = (param_2 >> 0x10);
  iVar7 = (int)param_2;
  uVar9 = (u32)(iVar7 + 0x4);
  ppcVar1 = (code **)((int)(u32)(u32)(iVar7 + 0x4) + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)in_EDX,param_1);
  mem_op_1000_179c(0xc,in_EDX);
  uVar3 = in_EDX | param_1;
  paVar6 = (Struct57*)((u32)in_EDX & 0xffff0000 | (u32)uVar3);
  if (uVar3 == 0x0) {
    param_1 = 0x0;
    paVar6 = NULL;
  }
  else {
    pass1_1010_8ef2(paVar6,(astruct_170 *)CONCAT22(in_EDX,param_1),in_stack_0000ffd4,in_stack_0000fe7c,
                    in_stack_0000ffa0,in_stack_0000ffa6,in_stack_0000ffaa);
  }
  uVar4 = SUB42(paVar6,0x0);
  uStack14 = 0x0;
  while( true ) {
    uVar3 = paVar6;
    if (uStack6 <= uStack14) break;
    ppcVar1 = (code **)((int)(u32)(u32)(iVar7 + 0x4) + 0x4);
    uVar2 = uStack6;
    (**ppcVar1)(0x1000,(u32)(iVar7 + 0x4),uStack14,uVar9);
    uVar5 = uVar3 | uVar2;
    paVar6 = (Struct57*)(u32)uVar5;
    if (uVar5 != 0x0) {
      ppcVar1 = (code **)((int)(u32)(u32)(param_1 + 0x4) + 0xc);
      (**ppcVar1)(0x1000,(u32)(param_1 + 0x4),uVar2,uVar3);
    }
    uStack14 += 0x1;
  }
  return;
}



void pass1_1010_9130(u16 param_1,u16 param_2,param_3: u32,u8 *param_4)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_3 >> 0x10);
  pass1_1030_1d58((u32)((int)param_3 + 0x4));
  if ((u8 *)(param_2 | param_1) != NULL) {
    uVar1 = (u32)((int)param_3 + 0x8);
    pass1_1010_c3c2((u8 *)(param_2 | param_1),uVar1,((u32)uVar1 >> 0x10),(u32)param_4,
                    CONCAT22(param_2,param_1));
    return;
  }
  *param_4 = '\0';
  return;
}



void struct_1010_9172(astruct_249 *param_1,u32 param_2)

{
  code **ppcVar1;
  u32 *puVar2;
  u16 uVar3;
  Struct57*paVar4;
  astruct_249 *iVar4;
  u16 uVar5;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_249 *)param_1;
  puVar2 = iVar4->field4_0x4;
  uVar3 = iVar4->field5_0x6;
  paVar4 = (Struct57*)(param_2 & 0xffff0000 | (u32)uVar3);
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    puVar2 = (u32 *)(**ppcVar1)();
  }
  mem_op_1000_179c(0x18,paVar4);
  uVar3 = paVar4 | puVar2;
  if (uVar3 == 0x0) {
    puVar2 = NULL;
    uVar3 = 0x0;
  }
  else {
    puVar2 = (u32 *)struct_op_1030_1cd8((astruct_75 *)CONCAT22(paVar4,puVar2),0x5,0x5);
  }
  iVar4->field4_0x4 = puVar2;
  iVar4->field5_0x6 = uVar3;
  return;
}



void pass1_1010_91cc(u32 param_1)

{
  code **ppcVar1;
  u16 uVar2;
  let mut lVar3: i32;

  uVar2 = (param_1 >> 0x10);
  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x10);
  lVar3 = (**ppcVar1)();
  if (lVar3 != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x8);
    (**ppcVar1)();
  }
  return;
}



void pass1_1010_9210(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0xc);
  (**ppcVar1)();
  return;
}



StructD * pass1_1010_922e(StructD *param_1,u8 param_2)

{
  pass1_1010_8f78(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_223 * pass1_1010_9258(astruct_223 *param_1)

{
  struct_1010_383a(param_1);
  param_1->field0_0x0 = 0x958e;
  ((int)param_1 + 0x2) = 0x1010;
  return param_1;
}



void pass1_1010_927a(u16 *param_1)

{
  *param_1 = 0x958e;
  ((int)param_1 + 0x2) = 0x1010;
  pass1_1010_3880((StructD *)param_1);
  return;
}



u32 pass1_1010_9298(StructD *param_1,astruct_19 *param_2,u16 param_3)

{
  u16 uVar1;
  Struct57*paVar2;

  uVar1 = ((u32)param_1 >> 0x10);
  paVar2 = (Struct57*)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  struct_1010_2cd2(param_2,param_3);
    //        1010:9566  40  95  10  10      addr         pass1_1010_9540
  param_2->offset_0x0 = 0x9566;
  ((int)param_2 + 0x2) = 0x1010;
  mem_op_1000_179c(0x20c,paVar2);
  ((int)param_2 + 0x5c) = uVar1;
  ((int)param_2 + 0x5e) = (int)paVar2;
  pass1_1000_4906((StructD *)CONCAT22((int)paVar2,((int)param_2 + 0x5c)),NULL,0x20c);
  return (u32)param_2;
}



void pass1_1010_92e6(u16 *param_1)

{
  *param_1 = 0x9566;
  ((int)param_1 + 0x2) = 0x1010;
  pass1_1010_2db2((astruct_455 *)param_1);
  return;
}



void pass1_1010_9304(u16 param_1,u8 *param_2,u16 param_3,u16 param_4,param_5: i16)

{
  u16 in_register_0000000a;
  Struct57*paVar1;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  if (param_5 != 0x0) {
    mem_op_1000_179c(param_5 << 0x2,paVar1);
    return;
  }
  mem_op_1000_179c(0x1a,paVar1);
  if ((paVar1 | param_1) != 0x0) {
    pass1_1010_9258((astruct_223 *)CONCAT22(paVar1,param_1));
    return;
  }
  return;
}



void pass1_1010_9348(param_1: u32,param_2: i16)

{
  i16 iVar1;
  u16 uVar2;

  (param_2 * 0x8 + 0x319e) = param_2;
  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x16) = param_2 * 0x8 + 0x3198;
  (iVar1 + 0x18) = (int)0x1050;
  (iVar1 + 0x12) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_9372(u32 *param_1,u16 param_2,i16 param_3,i16 param_4,param_5: i16)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  char cVar4;
  Struct57*in_EDX;
  u16 uVar5;
  u16 unaff_SI;
  u16 uVar6;
  bool bVar7;
  u32 uVar8;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;

  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_3528 == NULL) {
      ppcVar1 = (code **)((int)*param_1 + 0x18);
      uVar3 = (**ppcVar1)();
      PTR_LOOP_1050_3528 =
           mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,uVar3),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
    }
    uVar2 = (u32)((int)param_1 + 0xc);
    uVar8 = pass1_1010_2e02((u32)_PTR_LOOP_1050_3528,((int)uVar2 + 0x12));
    uVar5 = param_2 + 0x1;
    uVar6 = param_3 + (0xfffe < param_2);
    for (cVar4 = ((char)param_4 + -0x1) * '\x04'; cVar4 != '\0'; cVar4 += -0x1) {
      bVar7 = CARRY2(uVar5,uVar5);
      uVar5 *= 0x2;
      uVar6 = uVar6 * 0x2 + bVar7;
    }
    pass1_1010_2e30((u32)_PTR_LOOP_1050_3528,uVar5 | uVar8,uVar6 | (uVar8 >> 0x10),param_5);
  }
  return;
}



void pass1_1010_93f0(u32 param_1)

{
  u8 *puVar1;
  u16 uVar2;
  i16 iVar3;
  u16 uVar4;
  astruct_223 *paVar5;
  u8 local_1c [0x1a];

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x56) == 0x0) {
    paVar5 = pass1_1010_9258((astruct_223 *)CONCAT22(0x1050,local_1c));
    uVar2 = ((u32)paVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e(puVar1,(u32 *)CONCAT22(0x1050,puVar1),0x0,0x0,0x0);
    (iVar3 + 0x56) = puVar1;
    (iVar3 + 0x58) = uVar2;
    pass1_1010_927a((u16 *)CONCAT22(0x1050,local_1c));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1010_9432(void)

{
  char *pcVar1;
  u32 in_stack_00000004;

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,*(u16*)((int)in_stack_00000004 + 0x16));
  return pcVar1;
}



void pass1_1010_944e(u16 param_1,u16 param_2,param_3: i16)

{
  code **ppcVar1;
  u32 uVar2;

  if (*(i32 *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)((int)(u32)CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



u16 FUN_1010_9482(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool pass1_1010_9488(u8 *param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  u32 *puVar5;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 uVar6;
  u32 in_stack_0000ffee;

  uVar6 = ((int)param_4 + 0x12);
  puVar5 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)in_stack_0000ffee >> 0x10),0x3),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  uVar1 = ((u32)puVar5 >> 0x10);
  uVar2 = puVar5;
  uVar3 = uVar6 - 0x32;
  uVar4 = uVar1;
  if (uVar3 == 0x0) {
    pass1_1010_a5ca(0x0,uVar1,uVar2,uVar1,0x32);
    if (uVar3 != 0x0) {
      return false;
    }
    uVar6 = 0x4d;
  }
  else {
    uVar3 = uVar6 - 0x3f;
    if (uVar3 == 0x0) {
      pass1_1010_a5ca(0x0,uVar1,uVar2,uVar1,0x3f);
      if (uVar3 != 0x0) {
        return false;
      }
      uVar6 = 0x4e;
    }
  }
  pass1_1010_a5ca(uVar3,uVar4,uVar2,uVar1,uVar6);
  return uVar3 == 0x0;
}



u16 pass1_1010_9502(u32 param_1)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_1 + 0x16);
  return ((int)uVar1 + 0x2);
}



u16 pass1_1010_9514(void)

{
  return 0x31;
}



StructD * pass1_1010_951a(StructD *param_1,u8 param_2)

{
  pass1_1010_927a(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_9540(u16 *param_1,u8 param_2)

{
  pass1_1010_92e6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1010_95aa(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  ((int)param_1 + 0x18) = 0x0;
  ((int)param_1 + 0x1a) = 0x0;
  ((int)param_1 + 0x1c) = 0xa;
  ((int)param_1 + 0x1e) = 0x0;
  param_1->offset_0x0 = 0xa1c8;
  ((int)param_1 + 0x2) = 0x1010;
  return;
}



void pass1_1010_95f8(astruct_455 *param_1)

{
  u32 *puVar1;
  u32 *puVar2;
  u16 uVar3;
  code **ppcVar4;
  astruct_455 *iVar4;
  u16 uVar5;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xa1c8;
  iVar4->field1_0x2 = 0x1010;
  puVar1 = (u32 *)iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = (u32 *)iVar4[0x1].field3_0x6;
  uVar3 = (iVar4 + 0x2)->field0_0x0;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = (u32 *)iVar4[0x2].field1_0x2;
  puVar2 = iVar4[0x2].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}



void pass1_1010_9674(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0x12);
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0x12) = 0x0;
  return;
}



void pass1_1010_96a8(param_1: u32,param_2: i16)

{
  i16 *piVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  piVar1 = (i16 *)((int)param_1 + 0x1e);
  *piVar1 = *piVar1 - param_2;
  if (*piVar1 < 0x0) {
    ((int)param_1 + 0x1e) = 0x0;
  }
  return;
}



u16 pass1_1010_96c2(u32 param_1)

{
  return ((int)param_1 + 0x1e);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1010_96d0(astruct_690 *param_1)

{
  i16 *piVar1;
  i16 iVar2;
  astruct_690 *iVar3;
  u16 uVar3;
  u32 uVar4;
  i16 iStack8;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_690 *)param_1;
  if (iVar3->field26_0x1a != 0x0) {
    if (0x0 < iVar3->field27_0x1c) {
      piVar1 = &iVar3->field27_0x1c;
      *piVar1 = *piVar1 + -0x1;
    }
    if ((iVar3->field27_0x1c == 0x0) && (iVar3->field28_0x1e != 0x0)) {
      iStack8 = 0x1;
      uVar4 = pass1_1030_8326();
      iVar2 = (int)(uVar4 >> 0x10);
      if ((iVar2 != 0x0) || (0x32 < uVar4)) {
        iStack8 = 0x5;
      }
      if ((iVar2 != 0x0) || (0x3c < uVar4)) {
        iStack8 = 0xa;
      }
      if (iVar3->field28_0x1e < iStack8) {
        iStack8 = iVar3->field28_0x1e;
      }
      piVar1 = &iVar3->field28_0x1e;
      *piVar1 = *piVar1 - iStack8;
      if (*piVar1 < 0x0) {
        iVar3->field28_0x1e = 0x0;
      }
      if (0x0 < iVar3->field28_0x1e) {
        return iStack8;
      }
      return -0x1;
    }
  }
  return 0x0;
}



void pass1_1010_9766(u16 param_1,u32 param_2)

{
  i16 in_AX;
  u16 uVar1;

  uVar1 = (param_2 >> 0x10);
  ((int)param_2 + 0x1a) = 0x1;
  pass1_1010_a0a0(param_1,(astruct_252 *)param_2);
  pass1_1010_9f8c(param_2,0x80);
  ((int)param_2 + 0x1e) = in_AX * 0x32;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_9794(astruct_250 *param_1)

{
  i16 iVar1;
  code **ppcVar2;
  char *pcVar3;
  u16 uVar4;
  char *pchar_5;
  u32 *puVar5;
  u16 uVar6;
  u32 in_EDX;
  u32 uVar8;
  astruct_250 *iVar9;
  u16 uVar9;
  char *pcVar10;
  char local_a [0x8];
  Struct57*paVar7;

  uVar9 = ((u32)param_1 >> 0x10);
  iVar9 = (astruct_250 *)param_1;
  if (iVar9->field18_0x18 == 0x0) {
    iVar9->field18_0x18 = 0x1;
    puVar5 = iVar9->field11_0xe;
    uVar4 = ((int)&iVar9->field11_0xe + 0x2);
    uVar6 = uVar4 | (u32 *)puVar5;
    paVar7 = (Struct57*)(in_EDX & 0xffff0000 | (u32)uVar6);
    if (uVar6 != 0x0) {
      ppcVar2 = (code **)(u32)puVar5;
      (**ppcVar2)();
    }
    mem_op_1000_179c(0xc,paVar7);
    uVar4 = puVar5;
    uVar6 = paVar7 | uVar4;
    uVar8 = (u32)uVar6;
    if (uVar6 == 0x0) {
      uVar4 = 0x0;
      uVar8 = 0x0;
    }
    else {
      set_struct_1008_574a((Struct57*)((u32)puVar5 & 0xffff | (long)paVar7 << 0x10));
    }
    &iVar9->field11_0xe = uVar4;
    ((int)&iVar9->field11_0xe + 0x2) = (int)uVar8;
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)iVar9->field10_0xa);
    while( true ) {
      uVar4 = uVar8;
      pchar_5 = local_a;
      pass1_1008_5b12((char *)CONCAT22(0x1050,pchar_5));
      uVar8 = (u32)(uVar4 | pchar_5);
      if ((uVar4 | pchar_5) == 0x0) break;
      iVar1 = (pchar_5 + 0x4);
      if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
        pcVar10 = iVar9->field10_0xa;
        ((int)pcVar10 + 0xa) = 0x0;
        pcVar10 = iVar9->field10_0xa;
        ppcVar2 = (code **)((int)(u32)iVar9->field10_0xa + 0xc);
        (**ppcVar2)();
        pcVar3 = iVar9->field10_0xa;
        ((int)pcVar3 + 0xa) = 0x1;
        local_a._4_4_ = 0x0;
        ppcVar2 = (code **)((int)*iVar9->field11_0xe + 0x4);
        (**ppcVar2)(0x1008,iVar9->field11_0xe,CONCAT22(uVar4,pchar_5),pcVar10);
      }
    }
  }
  return;
}



void pass1_1010_988c(param_1: u32,param_2: i16)

{
  code **ppcVar1;
  u32 uVar2;
  i16 iVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  let mut lVar8: i32;
  u8 local_a [0x8];

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar6 + 0xe));
  do {
    lVar8 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar5 = ((u32)lVar8 >> 0x10);
    iVar3 = (int)lVar8;
    if (lVar8 == 0x0) {
      return;
    }
  } while ((iVar3 + 0x4) != param_2);
  iVar4 = (iVar3 + 0x6) + -0x1;
  (iVar3 + 0x6) = iVar4;
  if ((iVar4 < 0x1) &&
     (ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xe) + 0xc),
     (**ppcVar1)(0x1008,(u32)(iVar6 + 0xe),lVar8), uVar2 = (u32)(iVar6 + 0xe),
     ((int)uVar2 + 0x8) == 0x0)) {
    (iVar6 + 0x16) = 0x1;
  }
  return;
}



u16 FUN_1010_9900(u16 param_1,param_2: u32,u8 *param_3)

{
  u32 uVar1;
  BOOL16 BVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  let mut lVar6: i32;
  HFILE16 in_stack_0000ffc0;
  u16 uStack36;
  u16 local_1c [0x3];
  u16 local_16 [0x3];
  u32 local_10;
  u8 local_c [0x8];
  u16 local_4;

  BVar2 = write_to_file_1008_7cac(param_3);
  if (BVar2 == 0x0) {
    return 0x0;
  }
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (int)param_2;
  if (*(i32 *)(iVar4 + 0xa) == 0x0) {
    local_4 = 0x0;
  }
  else {
    uVar1 = (u32)(iVar4 + 0xa);
    local_4 = ((int)uVar1 + 0x8);
  }
  local_1c[0] = local_4;
  BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffc0);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)(iVar4 + 0xa));
  do {
    local_10 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_c));
    if (local_10 == 0x0) {
      if (*(i32 *)(iVar4 + 0xe) == 0x0) {
        uStack36 = 0x0;
      }
      else {
        uVar1 = (u32)(iVar4 + 0xe);
        uStack36 = ((int)uVar1 + 0x8);
      }
      local_16[0] = uStack36;
      BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),(char *)0x2,in_stack_0000ffc0);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return 0x0;
      }
      pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)(iVar4 + 0xe));
      do {
        lVar6 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_c));
        uVar3 = ((u32)lVar6 >> 0x10);
        if (lVar6 == 0x0) {
          if (*(i32 *)(iVar4 + 0x12) == 0x0) {
            uStack36 = 0x0;
          }
          else {
            uVar1 = (u32)(iVar4 + 0x12);
            uStack36 = ((int)uVar1 + 0x8);
          }
          local_16[0] = uStack36;
          BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),(char *)0x2,in_stack_0000ffc0);
          if (BVar2 == 0x0) {
            u16_1050_0310 = 0x6d0;
            return 0x0;
          }
          pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)(iVar4 + 0x12));
          do {
            lVar6 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_c));
            uVar3 = ((u32)lVar6 >> 0x10);
            if (lVar6 == 0x0) {
              local_1c[0] = (iVar4 + 0x1a);
              BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffc0);
              if (BVar2 == 0x0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
              }
              local_1c[0] = (iVar4 + 0x1c);
              BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffc0);
              if (BVar2 == 0x0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
              }
              local_1c[0] = (iVar4 + 0x1e);
              BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffc0);
              if (BVar2 == 0x0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
              }
              return 0x1;
            }
            local_10 = local_10 & 0xffff0000 | (u32)((int)lVar6 + 0x4);
            BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_10),(char *)0x2,in_stack_0000ffc0);
            if (BVar2 == 0x0) {
              u16_1050_0310 = 0x6d0;
              return 0x0;
            }
            local_4 = ((int)lVar6 + 0x6);
            BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_4),(char *)0x2,in_stack_0000ffc0);
          } while (BVar2 != 0x0);
          u16_1050_0310 = 0x6d0;
          return 0x0;
        }
        local_10 = local_10 & 0xffff0000 | (u32)((int)lVar6 + 0x4);
        BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_10),(char *)0x2,in_stack_0000ffc0);
        if (BVar2 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return 0x0;
        }
        local_4 = ((int)lVar6 + 0x6);
        BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_4),(char *)0x2,in_stack_0000ffc0);
      } while (BVar2 != 0x0);
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    local_16[0] = ((int)local_10 + 0x4);
    BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),(char *)0x2,in_stack_0000ffc0);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    local_16[0] = ((int)local_10 + 0x6);
    BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),(char *)0x2,in_stack_0000ffc0);
  } while (BVar2 != 0x0);
  u16_1050_0310 = 0x6d0;
  return 0x0;
}



void FUN_1010_9b72(u16 param_1,param_2: u32,HFILE16 *param_3,i16 param_4,u16 param_5)

{
  code **ppcVar1;
  u16 uVar2;
  BOOL16 BVar3;
  i16 iVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  i16 iVar7;
  u32 *puVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u16 local_36 [0x4];
  u16 *puStack46;
  u16 local_2a [0x2];
  u16 *puStack38;
  u16 auStack34 [0x2];
  u16 *puStack30;
  i16 local_1a [0x2];
  u16 *puStack22;
  u16 uStack18;
  u16 *puStack14;
  i16 local_a [0x3];
  u16 local_4;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_5);
  uVar10 = param_3;
  uVar11 = ((u32)param_3 >> 0x10);
  read_file_1008_7cfe(uVar10,uVar11,0x1);
  if (param_4 != 0x0) {
    uVar2 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (uVar2 != 0x0) {
      iVar7 = (int)param_2;
      uVar9 = (param_2 >> 0x10);
      if (local_4 != 0x0) {
        if (*(i32 *)(iVar7 + 0xa) == 0x0) {
          mem_op_1000_179c(0xc,paVar6);
          uVar5 = paVar6;
          puStack22 = (u16 *)CONCAT22(uVar5,uVar2);
          paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar2));
          if ((uVar5 | uVar2) == 0x0) {
            (u32)(iVar7 + 0xa) = 0x0;
          }
          else {
            set_struct_1008_574a((Struct57*)CONCAT22(uVar5,uVar2));
            (iVar7 + 0xa) = uVar2;
            (iVar7 + 0xc) = (int)paVar6;
          }
        }
        for (uStack18 = 0x0; uStack18 < local_4; uStack18 += 0x1) {
          uVar2 = local_4;
          mem_op_1000_179c(0x8,paVar6);
          uVar5 = paVar6;
          puStack22 = (u16 *)CONCAT22(uVar5,uVar2);
          paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar2));
          if ((uVar5 | uVar2) == 0x0) {
            puStack14 = NULL;
          }
          else {
            *puStack22 = 0x389a;
            (uVar2 + 0x2) = 0x1008;
            *puStack22 = 0xa1c4;
            (uVar2 + 0x2) = 0x1010;
            puStack14 = puStack22;
          }
          BVar3 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_a),0x2);
          if (BVar3 == 0x0) {
LAB_1010_9c32:
            puStack22 = puStack14;
            if (puStack14 == NULL) goto LAB_1010_9ba1;
            uVar9 = ((u32)puStack14 >> 0x10);
            puVar8 = (u32 *)puStack14;
            goto LAB_1010_9e9e;
          }
          BVar3 = read_file_1008_7dee(param_3,(u8 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0x6)),0x2)
          ;
          if (BVar3 == 0x0) goto LAB_1010_9c32;
          iVar4 = switch_1008_73ea(uVar10,uVar11,local_a[0]);
          ((int)puStack14 + 0x4) = iVar4;
          ppcVar1 = (code **)((int)(u32)(u32)(iVar7 + 0xa) + 0x4);
          (**ppcVar1)();
        }
      }
      uVar2 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_2a),0x2);
      if (uVar2 != 0x0) {
        if (local_2a[0] != 0x0) {
          if (*(i32 *)(iVar7 + 0xe) == 0x0) {
            mem_op_1000_179c(0xc,paVar6);
            uVar5 = paVar6;
            puStack46 = (u16 *)CONCAT22(uVar5,uVar2);
            paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar2));
            if ((uVar5 | uVar2) == 0x0) {
              (u32)(iVar7 + 0xe) = 0x0;
            }
            else {
              set_struct_1008_574a((Struct57*)CONCAT22(uVar5,uVar2));
              (iVar7 + 0xe) = uVar2;
              (iVar7 + 0x10) = (int)paVar6;
            }
          }
          for (auStack34[0] = 0x0; auStack34[0] < local_2a[0]; auStack34[0] += 0x1) {
            uVar2 = local_2a[0];
            mem_op_1000_179c(0x8,paVar6);
            uVar5 = paVar6;
            puStack22 = (u16 *)CONCAT22(uVar5,uVar2);
            paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar2));
            if ((uVar5 | uVar2) == 0x0) {
              puStack30 = NULL;
            }
            else {
              *puStack22 = 0x389a;
              (uVar2 + 0x2) = 0x1008;
              *puStack22 = 0xa1c4;
              (uVar2 + 0x2) = 0x1010;
              puStack30 = puStack22;
            }
            BVar3 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_1a),0x2);
            if (BVar3 == 0x0) {
LAB_1010_9d5c:
              puStack22 = puStack30;
              if (puStack30 == NULL) goto LAB_1010_9ba1;
              uVar9 = ((u32)puStack30 >> 0x10);
              puVar8 = (u32 *)puStack30;
              goto LAB_1010_9e9e;
            }
            BVar3 = read_file_1008_7dee(param_3,(u8 *)((u32)puStack30 & 0xffff0000 | (u32)((int)puStack30 + 0x6)),
                                        0x2);
            if (BVar3 == 0x0) goto LAB_1010_9d5c;
            iVar4 = switch_1008_73ea(uVar10,uVar11,local_1a[0]);
            ((int)puStack30 + 0x4) = iVar4;
            ppcVar1 = (code **)((int)(u32)(u32)(iVar7 + 0xe) + 0x4);
            (**ppcVar1)();
          }
        }
        uVar2 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_36),0x2);
        if (uVar2 != 0x0) {
          if (local_36[0] != 0x0) {
            if (*(i32 *)(iVar7 + 0x12) == 0x0) {
              mem_op_1000_179c(0xc,paVar6);
              uVar5 = paVar6;
              puStack22 = (u16 *)CONCAT22(uVar5,uVar2);
              paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar2));
              if ((uVar5 | uVar2) == 0x0) {
                (u32)(iVar7 + 0x12) = 0x0;
              }
              else {
                set_struct_1008_574a((Struct57*)CONCAT22(uVar5,uVar2));
                (iVar7 + 0x12) = uVar2;
                (iVar7 + 0x14) = (int)paVar6;
              }
            }
            for (local_2a[0] = 0x0; local_2a[0] < local_36[0]; local_2a[0] += 0x1) {
              uVar2 = local_36[0];
              mem_op_1000_179c(0x8,paVar6);
              uVar5 = paVar6;
              puStack46 = (u16 *)CONCAT22(uVar5,uVar2);
              paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar5 | uVar2));
              if ((uVar5 | uVar2) == 0x0) {
                puStack38 = NULL;
              }
              else {
                *puStack46 = 0x389a;
                (uVar2 + 0x2) = 0x1008;
                *puStack46 = 0xa1c4;
                (uVar2 + 0x2) = 0x1010;
                puStack38 = puStack46;
              }
              BVar3 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,auStack34),0x2);
              if (BVar3 == 0x0) {
LAB_1010_9e86:
                puStack22 = puStack38;
                if (puStack38 != NULL) {
                  uVar9 = ((u32)puStack38 >> 0x10);
                  puVar8 = (u32 *)puStack38;
LAB_1010_9e9e:
                  ppcVar1 = (code **)*puVar8;
                  puStack46 = puStack22;
                  (**ppcVar1)(0x1008,(int)puStack22,uVar9,0x1);
                }
                goto LAB_1010_9ba1;
              }
              BVar3 = read_file_1008_7dee(param_3,(u8 *)((u32)puStack38 & 0xffff0000 | (u32)((int)puStack38 + 0x6)),
                                          0x2);
              if (BVar3 == 0x0) goto LAB_1010_9e86;
              iVar4 = switch_1008_73ea(uVar10,uVar11,auStack34[0]);
              ((int)puStack38 + 0x4) = iVar4;
              ppcVar1 = (code **)((int)(u32)(u32)(iVar7 + 0x12) + 0x4);
              (**ppcVar1)();
            }
          }
          BVar3 = read_file_1008_7dee(param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar7 + 0x1a)),0x2);
          if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar7 + 0x1c)),0x2);
            if (BVar3 != 0x0) {
              BVar3 = read_file_1008_7dee(param_3,(u8 *)(param_2 & 0xffff0000 | (u32)(iVar7 + 0x1e)),0x2);
              if (BVar3 != 0x0) {
                return;
              }
            }
          }
        }
      }
    }
LAB_1010_9ba1:
    u16_1050_0310 = 0x6d2;
  }
  return;
}



void pass1_1010_9f72(param_1: u32,param_2: i16)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  pass1_1010_9fa6(param_1,uVar1,(u32)(param_1 + 0xe),param_2);
  return;
}



void pass1_1010_9f8c(param_1: u32,param_2: i16)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  pass1_1010_9fa6(param_1,uVar1,(u32)(param_1 + 0xa),param_2);
  return;
}



u16 pass1_1010_9fa6(u16 param_1,u16 param_2,param_3: u32,param_4: i16)

{
  u16 uVar1;
  let mut lVar2: i32;
  u8 local_a [0x8];

  if (param_3 != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),param_3);
    while( true ) {
      lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      uVar1 = ((u32)lVar2 >> 0x10);
      if (lVar2 == 0x0) break;
      if (((int)lVar2 + 0x4) == param_4) {
        return ((int)lVar2 + 0x6);
      }
    }
  }
  return 0x0;
}



void pass1_1010_9fee(StructD *param_1,astruct_252 *param_2,u16 param_3,u16 param_4)

{
  StructD *iVar5;
  u16 uVar1;
  u16 uVar2;
  Struct57*paVar3;
  astruct_252 *iVar3;
  astruct_253 *iVar4;
  u16 uVar4;
  u16 uVar5;
  StructD *puStack10;
  StructD *pSStack6;
  code **fn_ptr_1;

  iVar5 = (StructD *)((u32)param_1 >> 0x10);
  paVar3 = (Struct57*)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  uVar4 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_252 *)param_2;
  if (iVar3->field18_0x12 == NULL) {
    mem_op_1000_179c(0xc,paVar3);
    uVar1 = paVar3;
    uVar2 = uVar1 | iVar5;
    paVar3 = (Struct57*)((u32)paVar3 & 0xffff0000 | (u32)uVar2);
    if (uVar2 == 0x0) {
      iVar3->field18_0x12 = NULL;
    }
    else {
      set_struct_1008_574a((Struct57*)CONCAT22(uVar1,iVar5));
      *(StructD **)&iVar3->field18_0x12 = iVar5;
      ((int)&iVar3->field18_0x12 + 0x2) = (int)paVar3;
    }
  }
  mem_op_1000_179c(0x8,paVar3);
  uVar1 = paVar3;
  puStack10 = (StructD *)CONCAT22(uVar1,iVar5);
  if ((uVar1 | iVar5) == 0x0) {
    pSStack6 = NULL;
  }
  else {
    puStack10->address_offset_field_0x0 = 0x389a;
    iVar5->address_offset_field_0x2 = 0x1008;
    puStack10->address_offset_field_0x0 = 0xa1c4;
    iVar5->address_offset_field_0x2 = 0x1010;
    pSStack6 = puStack10;
  }
  uVar5 = ((u32)pSStack6 >> 0x10);
  iVar4 = (astruct_253 *)pSStack6;
  iVar4->field4_0x4 = param_4;
  iVar4->field5_0x6 = param_3;
  fn_ptr_1 = (code **)((int)*iVar3->field18_0x12 + 0x4);
  (**fn_ptr_1)(0x1000,iVar3->field18_0x12,iVar4,uVar5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_a0a0(u16 param_1,astruct_252 *param_2)

{
  i16 *piVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  Struct57*paVar8;
  u32 uVar9;
  bool bVar10;
  bool bVar11;
  u32 *puVar12;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffe4;
  i16 iStack12;
  u8 local_a [0x8];

  paVar8 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_2 + 0xa));
  iStack12 = 0x4;
  puVar12 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe4,0x2),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar9 = (u32)puVar12 >> 0x10;
  if ((PTR_LOOP_1050_13ae != (u8 *)&u16_1050_0002) &&
     (PTR_LOOP_1050_13ae != (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1))) {
    iStack12 = 0x2;
  }
  do {
    while( true ) {
      uVar6 = uVar9;
      uVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      uVar9 = (u32)(uVar6 | uVar4);
      if ((uVar6 | uVar4) == 0x0) {
        return;
      }
      iVar2 = (uVar4 + 0x4);
      if (iVar2 != 0x12) break;
      piVar1 = (i16 *)(uVar4 + 0x6);
      bVar11 = SBORROW2(*piVar1,0x2);
      iVar3 = *piVar1 + -0x2;
      bVar10 = iVar3 == 0x0;
LAB_1010_a151:
      if (!bVar10 && bVar11 == iVar3 < 0x0) {
LAB_1010_a153:
        iVar2 = (uVar4 + 0x6);
        uVar9 = (u32)(iVar2 % iStack12);
        piVar1 = (i16 *)(uVar4 + 0x6);
        *piVar1 = *piVar1 - iVar2 / iStack12;
      }
    }
    if (((iVar2 != 0x3e) && (iVar2 != 0x41)) && (iVar2 != 0x80)) {
      if (iVar2 == 0x83) {
        piVar1 = (i16 *)(uVar4 + 0x6);
        bVar11 = SBORROW2(*piVar1,0x1);
        iVar2 = *piVar1;
        iVar3 = iVar2 + -0x1;
        bVar10 = iVar2 == 0x1;
        goto LAB_1010_a151;
      }
      goto LAB_1010_a153;
    }
    iVar2 = (uVar4 + 0x6);
    uVar7 = iVar2 >> 0xf;
    uVar5 = iVar2 / 0x2;
    piVar1 = (i16 *)(uVar4 + 0x6);
    *piVar1 = *piVar1 - uVar5;
    if (uVar5 == 0x0) {
      uVar5 = 0x1;
    }
    uVar9 = (u32)uVar7;
    pass1_1010_9fee((StructD *)CONCAT22(uVar5,uVar7),param_2,uVar5,(uVar4 + 0x4));
  } while( true );
}



u16 * pass1_1010_a172(u16 *param_1,u8 param_2)

{
  pass1_1010_95f8((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_a198(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1010_a1d8(astruct_19 *param_1,u16 param_2)

{
  i16 iVar1;
  code **ppcVar2;
  u32 in_EDX;
  u16 uVar3;
  u16 unaff_SI;
  astruct_19 *paVar4;
  u32 *puVar5;
  u16 in_stack_0000fe4e;
  u16 in_stack_0000ff72;
  u16 in_stack_0000ff78;
  u16 in_stack_0000ff7c;
  u16 uStack4;

  uVar3 = ((u32)in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x389a;
  ((int)param_1 + 0xc) = 0x1008;
  ((int)param_1 + 0xa) = 0x3aa8;
  ((int)param_1 + 0xc) = 0x1008;
  (u32)((int)param_1 + 0x138) = 0x0;
  param_1->offset_0x0 = 0xe9cc;
  ((int)param_1 + 0x2) = 0x1010;
  ((int)param_1 + 0xa) = 0xe9dc;
  ((int)param_1 + 0xc) = 0x1010;
  puVar5 = mixed_1010_20ba((Struct57*)CONCAT22(uVar3,(int)((u32)paVar4 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe4e,in_stack_0000ff72,in_stack_0000ff78,
                           in_stack_0000ff7c);
  ((int)param_1 + 0x138) = (int)puVar5;
  ((int)param_1 + 0x13a) = (int)((u32)puVar5 >> 0x10);
  ppcVar2 = (code **)((int)(u32)(u32)((int)param_1 + 0x138) + 0x4);
  (**ppcVar2)();
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0xa4)),NULL,0x94);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0xe)),NULL,0x96);
  uStack4 = 0x0;
  do {
    iVar1 = (int)param_1 + uStack4 * 0x6;
    *(code **)(iVar1 + 0xe) = pass1_1010_c7e2;
    (iVar1 + 0x12) = 0x0;
    uStack4 += 0x1;
  } while (uStack4 < 0x19);
  *(code **)((int)param_1 + 0x4a) = pass1_1010_c864;
  ((int)param_1 + 0x4e) = 0x0;
  *(code **)((int)param_1 + 0x50) = pass1_1010_cc56;
  ((int)param_1 + 0x54) = 0x0;
  *(code **)((int)param_1 + 0x56) = pass1_1010_cf36;
  ((int)param_1 + 0x5a) = 0x0;
  *(code **)((int)param_1 + 0x2c) = pass1_1010_d24a;
  ((int)param_1 + 0x30) = 0x0;
  *(code **)((int)param_1 + 0x6e) = pass1_1010_d448;
  ((int)param_1 + 0x72) = 0x0;
  *(code **)((int)param_1 + 0x74) = pass1_1010_d5ae;
  ((int)param_1 + 0x78) = 0x0;
  *(code **)((int)param_1 + 0x98) = pass1_1010_d710;
  ((int)param_1 + 0x9c) = 0x0;
  return;
}



void pass1_1010_a478(StructD *param_1)

{
  u16 *puVar1;
  u16 uVar2;
  StructD *uVar3;
  u16 uVar4;
  u16 *puStack6;

  uVar4 = ((u32)param_1 >> 0x10);
  uVar3 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe9cc;
  uVar3->address_offset_field_0x2 = 0x1010;
  uVar3->field6_0xa = 0xe9dc;
  uVar3->field7_0xc = (astruct_589 *)0x1010;
  if (*(i32 *)&uVar3[0x1].field_0x4a != 0x0) {
    if (param_1 == NULL) {
      puVar1 = NULL;
      uVar2 = 0x0;
    }
    else {
      puVar1 = &uVar3->field6_0xa;
      uVar2 = uVar4;
    }
    pass1_1010_1ea6((u32)&uVar3[0x1].field_0x4a,(StructD *)CONCAT22(uVar2,puVar1));
  }
  (u32)&uVar3[0x1].field_0x4a = 0x0;
  if (param_1 == NULL) {
    puVar1 = NULL;
    uVar4 = 0x0;
  }
  else {
    puVar1 = &uVar3->field6_0xa;
  }
  puStack6 = (u16 *)CONCAT22(uVar4,puVar1);
  *puStack6 = 0x389a;
  puVar1[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_a50c(astruct_20 *param_1,u8 **param_2,StructD *param_3)

{
  i16 iVar1;
  astruct_20 *struct_1;
  u32 local_8;
  i16 iStack4;

  struct_1 = (astruct_20 *)param_1;
  struct_1 = (astruct_20 *)&struct_1->field133_0xa4;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(struct_1)),NULL,0x94);
  iVar1 = ((int)param_3 + 0xa);
  local_8 = (u32)(&struct_1->field6_0xe + iVar1 * 0x3);
  iStack4 = ((int)&struct_1->field7_0x10 + iVar1 * 0x6 + 0x2);
  ((code)local_8)(0x1000,(int)&struct_1->offset_0x0 + iStack4,param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_a568(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  pass1_1030_2622(CONCAT22(param_2,param_1),param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_a58a(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  pass1_1030_266c(param_1,CONCAT22(param_5,param_2));
  return;
}



u16 pass1_1010_a5ac(u16 param_1,u16 param_2,u32 param_3)

{
  u16 in_AX;
  u16 in_DX;
  u32 uVar1;

  uVar1 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,in_DX);
  return ((int)uVar1 + 0x20);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_a5ca(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  pass1_1030_2242((astruct_168 *)CONCAT22(param_2,param_1),param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_a5ec(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 DX_REG;
  u32 *puVar6;
  u32 uStack6;

  uVar2 = param_5 | param_5;
  if (param_5 != 0x0) {
    pass1_1030_8344(_u16_1050_5748,0x8000001);
    uStack6 = CONCAT22(param_1,uVar2);
    puVar6 = (u32 *)struct_op_1030_73a8((astruct_419 *)param_5,uVar2,param_1);
    uVar5 = ((u32)puVar6 >> 0x10);
    uVar4 = ((int)puVar6 + 0x20);
    if (uVar4 != param_4) {
      uVar3 = param_4;
      pass1_1010_a5ca(param_4,uVar5,param_2,param_3,uVar4);
      if ((uVar4 != 0x70) && ((int)uVar3 < 0x0)) {
        pass1_1030_25d8(CONCAT22(param_1,uVar2),uVar3 + 0x1,uVar4);
      }
      ppcVar1 = (code **)((int)*puVar6 + 0x8);
      uVar4 = param_4;
      (**ppcVar1)();
      if (param_4 != 0x70) {
        pass1_1010_a5ca(uVar4,DX_REG,param_2,param_3,param_4);
        if ((int)uVar4 < 0x0) {
          pass1_1030_25d8(uStack6,uVar4 - 0x1,param_4);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_a69c(i16 param_1,u16 param_2,param_3: u32,u16 param_4)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  u8 *puVar4;
  u16 in_register_0000000a;
  Struct57*paVar5;
  u32 *puVar6;
  astruct_25 *paVar7;
  astruct_67 *paVar8;
  Struct27 *paVar9;
  u16 in_stack_0000fe74;
  u16 in_stack_0000fe76;
  u16 in_stack_0000fe78;
  u16 in_stack_0000fe7a;
  u16 in_stack_0000ff98;
  u16 in_stack_0000ff9a;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ff9e;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa4;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffa8;
  u16 uVar10;
  u16 uVar11;
  u16 in_stack_0000ffd2;
  u16 uStack22;
  i16 iStack20;

  paVar5 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  uVar3 = SUB42(paVar5,0x0);
  if (param_4 == 0x1) {
    for (iStack20 = 0x0; iStack20 < 0x83; iStack20 += 0x1) {
      iVar1 = pass1_1030_2242((astruct_168 *)CONCAT22(uVar3,param_1),iStack20);
      if (0x19 < iVar1) {
        uStack22 = iVar1 - 0x5;
        if ((int)uStack22 < 0x19) {
          uStack22 = 0x19;
        }
        pass1_1030_25d8(CONCAT22(uVar3,param_1),uStack22,iStack20);
      }
    }
    goto switchD_1010_aaef_caseD_b;
  }
  pass1_1030_25f0(CONCAT22(uVar3,param_1),0x0,param_4);
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd2,0x35),in_stack_0000fe7a,
                           in_stack_0000ff9e,in_stack_0000ffa4,in_stack_0000ffa8);
  paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)puVar6 >> 0x10);
  puVar4 = (u8 *)((u32)puVar6 >> 0x10);
  uVar2 = param_3;
  uVar10 = (param_3 >> 0x10);
  switch(param_4) {
  case 0xa:
  case 0xc:
    iVar1 = 0x1b;
    break;
  default:
    goto switchD_1010_aaef_caseD_b;
  case 0x10:
    pass1_1010_682e((u32)puVar6,0x1,0x2d);
    if ((param_1 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x2d;
    goto LAB_1010_a91f;
  case 0x12:
    pass1_1010_682e((u32)puVar6,0x1,0x16);
    pass1_1010_682e((u32)puVar6,0x1,0x17);
    pass1_1010_682e((u32)puVar6,0x1,0x18);
    pass1_1010_682e((u32)puVar6,0x1,0x40);
    iVar1 = 0x3f;
    goto LAB_1010_a96c;
  case 0x13:
    iVar1 = 0x35;
    goto LAB_1010_a91f;
  case 0x19:
    goto switchD_1010_aaef_caseD_19;
  case 0x1a:
    iVar1 = 0xf;
    goto LAB_1010_a96c;
  case 0x1c:
    iVar1 = 0x11;
    goto LAB_1010_a96c;
  case 0x1d:
  case 0x24:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x1e);
    iVar1 = 0x5b;
    goto LAB_1010_a91f;
  case 0x1e:
    uVar2 = 0x1;
    iVar1 = 0x2;
    puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe76,in_stack_0000ff9a,
                             in_stack_0000ffa0,in_stack_0000ffa4);
    paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)puVar6 >> 0x10);
    pass1_1010_08c0((u32)puVar6,uVar2,iVar1);
    paVar7 = (astruct_25 *)
             mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd2,0x40),in_stack_0000fe7a,
                             in_stack_0000ff9e,in_stack_0000ffa4,in_stack_0000ffa8);
    paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)paVar7 >> 0x10);
    load_str_and_sprintf_1008_b69c((int)((u32)paVar7 >> 0x10),paVar7);
    goto switchD_1010_aaef_caseD_b;
  case 0x22:
    iVar1 = 0x8;
    goto LAB_1010_aabe;
  case 0x23:
    iVar1 = 0xc;
    goto LAB_1010_aabe;
  case 0x25:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x14);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x1b);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x1e);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x22);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x25);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x28);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x2a);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x2d);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x2f);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x31);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x35);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x38);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x3a);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x3c);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x48);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x4f);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x52);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x54);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x57);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x5b);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x5d);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x62);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x66);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x68);
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,0x6c);
    goto switchD_1010_aaef_caseD_19;
  case 0x29:
    iVar1 = 0x25;
    break;
  case 0x2a:
    iVar1 = 0xf;
    goto LAB_1010_aabe;
  case 0x2b:
    iVar1 = 0x6e;
    goto LAB_1010_a96c;
  case 0x30:
    iVar1 = 0x54;
    break;
  case 0x33:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x31);
    iVar1 = 0x6c;
    goto LAB_1010_a91f;
  case 0x36:
    iVar1 = 0x13;
    goto LAB_1010_aabe;
  case 0x37:
    iVar1 = 0x2c;
LAB_1010_a96c:
    pass1_1010_682e((u32)puVar6,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x38:
    pass1_1010_682e((u32)puVar6,0x1,0x28);
    if ((param_1 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x28;
    goto LAB_1010_a91f;
  case 0x39:
    iVar1 = 0x10;
    goto LAB_1010_aabe;
  case 0x3a:
    iVar1 = 0x11;
    goto LAB_1010_aabe;
  case 0x3b:
    iVar1 = 0x12;
LAB_1010_aabe:
    pass1_1010_6814((u32)puVar6,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x3c:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x14);
    iVar1 = 0x62;
    goto LAB_1010_a91f;
  case 0x3d:
    pass1_1010_682e((u32)puVar6,0x1,0x66);
    if ((param_1 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x66;
LAB_1010_a91f:
    pass1_1010_abd2((u8 *)paVar5,uVar2,uVar10,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x3e:
    iVar1 = 0x5d;
    break;
  case 0x3f:
    iVar1 = 0x22;
    break;
  case 0x40:
    iVar1 = 0x57;
    break;
  case 0x41:
    iVar1 = 0x4f;
  }
  pass1_1010_abd2(puVar4,uVar2,uVar10,iVar1);
switchD_1010_aaef_caseD_b:
  paVar8 = (astruct_67 *)
           mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x37),in_stack_0000fe78,
                           in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)paVar8 >> 0x10);
  uVar2 = pass1_1008_ab12(paVar8,((u32)paVar8 >> 0x10),param_4);
  if (uVar2 != 0x0) {
    post_win_msg_1008_a0e4(paVar8,0x0,0x0,0x1,0x0,uVar2);
  }
  post_win_msg_1008_a0e4(paVar8,0x0,0x0,0x1,0x0,0x3d);
  uVar11 = 0x400;
  iVar1 = 0x6;
  uVar3 = 0x1;
  paVar9 = (Struct27 *)
           mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe74,in_stack_0000ff98,
                           in_stack_0000ff9e,in_stack_0000ffa2);
  pass1_1010_043a(paVar9,CONCAT22(uVar11,uVar3),iVar1);
  return;
switchD_1010_aaef_caseD_19:
  ((int)puVar6 + 0x148) = 0x34;
  goto switchD_1010_aaef_caseD_b;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_abd2(u8 *param_1,u16 param_2,u16 param_3,param_4: i16)

{
  bool bVar1;
  i16 *piVar2;
  u16 in_register_0000000a;
  u16 unaff_SI;
  u32 *puVar3;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  i16 *piStack20;
  i16 iStack16;
  i16 iStack14;

  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe8a,in_stack_0000ffae,in_stack_0000ffb4,
                           in_stack_0000ffb8);
  bVar1 = false;
  iStack16 = param_4;
  piStack20 = (i16 *)CONCAT22(0x1050,&stack0x000a);
  while( true ) {
    piVar2 = piStack20;
    if (iStack16 == 0x0) {
      return;
    }
    if (bVar1) break;
    if ((iStack16 * 0x2 + (int)puVar3 + 0xa) != 0x0) {
      bVar1 = true;
      iStack14 = iStack16;
    }
    piStack20 = (i16 *)((u32)piStack20 & 0xffff0000 | (u32)((int)piStack20 + 0x2));
    iStack16 = *piVar2;
  }
  pass1_1010_682e((u32)puVar3,0x0,iStack14);
  pass1_1010_682e((u32)puVar3,0x1,iStack16);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1010_ac62(u16 param_1,u16 param_2,u16 param_3,u16 param_4,param_5: i16)

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  return (param_1 + 0x116 + param_5 * 0x2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1010_ac92(u16 param_1,u16 param_2,param_3: i16)

{
  char *pcVar1;

  if ((0x0 < param_3) && (param_3 < 0x43)) {
    pcVar1 = load_string_1010_847e(_u16_1050_14cc,param_3 + 0x664);
    return pcVar1;
  }
  return NULL;
}



u16 pass1_1010_acc0(u16 param_1,u16 param_2,u32 param_3)

{
  u16 in_AX;
  u16 in_DX;
  u32 uVar1;

  uVar1 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,in_DX);
  if (((int)uVar1 + 0x12) != 0x4) {
    return 0x1;
  }
  return 0x0;
}



void pass1_1010_acec(param_1: u32,param_2: i16)

{
  if (param_2 == 0x1) {
    (u32)((int)param_1 + 0x12e) = 0x0;
  }
  else if (param_2 != 0x5) {
    return;
  }
  pass1_1010_1f62((Struct27 *)(param_1 & 0xffff0000 | (u32)((int)param_1 - 0xa)),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_ad22(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u32 uVar1;
  u16 *puVar2;

  uVar1 = (u32)(param_2 + 0x138);
  puVar2 = &param_4;
  pass1_1030_627e(puVar2,param_1,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  *(i32 *)((int)uVar1 + 0x20));
  if ((param_1 | puVar2) == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar2));
  return;
}



void pass1_1010_ad64(param_1: u32,u16 param_2,u16 param_3,param_4: u32,u32 param_5)

{
  if (param_5 != 0x0) {
    param_1 = (u32)((int)param_5 + 0x2e);
    if (*(i32 *)((int)param_1 + 0x200) == 0x8000002) {
      return;
    }
  }
  pass1_1010_c58as(param_1,param_2,(astruct_20 *)CONCAT22((int)param_4,param_3),(param_4 >> 0x10),
                   param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * string_op_1010_ada6(u16 param_1,u16 param_2,u16 param_3,i16 param_4,param_5: i16)

{
  char *pcVar1;
  char *pcStack6;

  pcStack6 = NULL;
  if (param_5 == 0x6) {
    if (param_4 == 0x0) goto LAB_1010_adee;
    pcVar1 = string_op_1020_c222(param_4);
  }
  else {
    if (param_5 != 0x7) {
      return NULL;
    }
    if (param_4 == 0x0) goto LAB_1010_adee;
    pcVar1 = string_op_1020_c2f8(param_4);
  }
  pcStack6 = (char *)CONCAT22(param_1,pcVar1);
LAB_1010_adee:
  if (pcStack6 == NULL) {
    pcStack6 = load_string_1010_847e(_u16_1050_14cc,0x531);
  }
  return pcStack6;
}



u16 pass1_1010_ae12(char *param_1,u16 param_2,u16 param_3,char *param_4,param_5: i16)

{
  char *pcVar1;
  i16 iVar2;
  u16 uStack4;

  if (param_5 == 0x6) {
    for (uStack4 = 0x0; (int)uStack4 < 0x15; uStack4 += 0x1) {
      pcVar1 = string_op_1020_c222(uStack4);
      iVar2 = pass1_1000_3d7a(param_4,(char *)CONCAT22(param_1,pcVar1));
      if (iVar2 == 0x0) {
        return uStack4;
      }
    }
  }
  else if (param_5 == 0x7) {
    for (uStack4 = 0x0; (int)uStack4 < 0x11; uStack4 += 0x1) {
      pcVar1 = string_op_1020_c2f8(uStack4);
      iVar2 = pass1_1000_3d7a(param_4,(char *)CONCAT22(param_1,pcVar1));
      if (iVar2 == 0x0) {
        return uStack4;
      }
    }
  }
  return 0xffff;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_ae92(param_1: u32,u16 param_2,u16 param_3,param_4: u32,u32 param_5)

{
  u8 bVar1;
  u16 uVar2;
  i16 iVar4;
  u16 uVar5;
  astruct_15 *paVar6;
  u32 uVar7;
  astruct_67 *paVar8;
  u16 in_stack_0000fe88;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb6;
  u16 uVar9;
  u16 uVar10;
  i16 iVar11;
  u8 uVar12;
  u8 uVar13;
  u16 uVar14;
  u16 uVar15;
  i16 iVar16;
  u16 uVar3;

  if (param_3 == 0x15) {
    uVar7 = struct_op_1030_73a8((astruct_419 *)param_4,0x15,(int)param_5);
    uVar3 = (uVar7 >> 0x10);
    if ((uVar3 | uVar7) != 0x0) {
      (uVar7 + 0x20) = param_2;
      return;
    }
  }
  else if (param_3 < 0x16) {
    bVar1 = (char)param_3 - 0x6;
    uVar3 = param_3 & 0xff00 | bVar1;
    if (bVar1 == 0x0) {
      pass1_1030_7f1a(param_4,param_2);
      paVar6 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)param_4,uVar3,(int)param_5);
      uVar5 = ((u32)param_5 >> 0x10);
      uVar2 = pass1_1010_b028(param_1,(param_1 >> 0x10),paVar6);
      uVar7 = pass1_1030_8326();
      iVar4 = (int)(uVar7 >> 0x10);
      if (((uVar2 == 0xe) && ((iVar4 != 0x0 || (0x32 < uVar7)))) &&
         ((param_2 == 0x1 || (((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3)))))) {
        uVar15 = 0x0;
        iVar16 = 0xb;
        uVar12 = 0x1;
        uVar13 = 0x0;
        uVar14 = 0x0;
        uVar10 = 0x0;
        iVar11 = 0x0;
        uVar9 = 0x0;
        paVar8 = (astruct_67 *)
                 mixed_1010_20ba((Struct57*)CONCAT22(uVar5,iVar4),_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe88
                                 ,in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
        post_win_msg_1008_a0e4
                  (paVar8,CONCAT22(uVar10,uVar9),iVar11,CONCAT11(uVar13,uVar12),CONCAT22(uVar15,uVar14),iVar16);
        return;
      }
    }
    else if ((char)param_3 == '\a') {
      pass1_1030_7eda(param_4,param_2);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_af66(u16 param_1,param_2: u32,u16 param_3)

{
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;

  uVar1 = (u32)((int)param_2 + 0x138);
  uVar2 = (u32)((int)uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uVar3 = param_1 | uVar2;
  if (uVar3 == 0x0) {
    return;
  }
  pass1_1038_5050(uVar2,uVar3,uVar2 & 0xffff | (u32)param_1 << 0x10,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_afa2(u16 param_1,param_2: u32,u16 param_3)

{
  u32 uVar1;
  u32 uVar2;

  uVar1 = (u32)((int)param_2 + 0x138);
  uVar2 = (u32)((int)uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  if ((param_1 | uVar2) == 0x0) {
    return;
  }
  pass1_1038_50e0(uVar2,uVar2 & 0xffff | (u32)param_1 << 0x10,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_afde(param_1: u32,param_2: i16)

{
  u32 uVar1;
  u32 uVar2;
  u32 in_EDX;
  u32 uVar3;
  u32 *puVar4;

  uVar1 = (u32)((int)param_1 + 0x138);
  uVar2 = (u32)((int)uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uVar3 = in_EDX & 0xffff0000;
  if ((in_EDX | uVar2) == 0x0) {
    return;
  }
  puVar4 = pass1_1008_c6fa(_u16_1050_06e0,param_2);
  pass1_1038_4e78(puVar4,(Struct57*)(uVar3 & 0xffff0000 | (u32)puVar4 >> 0x10),
                  uVar2 & 0xffff | in_EDX << 0x10,puVar4);
  return;
}



u16 pass1_1010_b028(u16 param_1,u16 param_2,astruct_15 *param_3)

{
  return ((int)param_3 + 0xc);
}


/*
Unable to decompile 'pass1_1010_b038'
Cause:
Low-level Error: Overlapping input varnodes
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void bad_1010_bf08(u16 param_1,u16 param_2,u32 param_3)

{
  bad_1028_e1bc();
  return;
}



void pass1_1010_bf1e(i16 param_1,u8 *param_2,param_3: u32,i16 *param_4)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;
  astruct_92 *paVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  u32 uVar7;
  i16 iVar8;
  u16 uVar9;
  u32 uStack40;
  i16 iStack36;
  u16 *puStack26;
  astruct_92 local_16;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  bad_1010_bf08(param_3,(param_3 >> 0x10),0x1000000);
  iVar2 = param_1 + -0x1;
  *param_4 = iVar2;
  uVar3 = iVar2 * 0x18;
  mem_op_1000_179c(uVar3,paVar6);
  uVar5 = paVar6;
  uStack40 = CONCAT22(uVar5,uVar3);
  uVar7 = (u32)(uVar5 | uVar3);
  iVar8 = (int)param_4;
  uVar9 = ((u32)param_4 >> 0x10);
  if ((uVar5 | uVar3) == 0x0) {
    (u32)(iVar8 + 0x2) = 0x0;
  }
  else {
    pass1_1000_5586(0x4092,0x1020,iVar2,0x18,uVar3,uVar5);
    (u32)(iVar8 + 0x2) = uStack40;
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x100);
  puStack26 = (u16*)(iVar8 + 0x2);
  iStack36 = 0x0;
  while( true ) {
    uVar3 = uVar7;
    paVar4 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
    uVar7 = (u32)(uVar3 | paVar4);
    if ((uVar3 | paVar4) == 0x0) break;
    uVar1 = (u32)&paVar4->field6_0x10;
    if (uVar1 != 0x0) {
      uVar7 = uVar1 >> 0x10;
      pass1_1008_3f62(puStack26,(u16 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x4)));
      ((int)puStack26 + 0xc) = iStack36;
      iStack36 += 0x1;
      puStack26 = (u16 *)((u32)puStack26 & 0xffff0000 | (u32)((int)puStack26 + 0x18));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_bffa(i16 param_1,u8 *param_2,u32 param_3)

{
  u16 *puVar1;
  i16 *piVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  astruct_257 *puVar5;
  u16 uVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  Struct57*paVar9;
  astruct_254 *iVar6;
  astruct_255 *iVar7;
  astruct_256 *iVar8;
  i16 iVar10;
  u16 uVar11;
  i16 iStack34;
  i16 iStack30;
  astruct_92 local_18;
  u32 *puStack6;

  paVar9 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0xa,paVar9);
  uVar7 = SUB42(paVar9,0x0);
  puStack6 = (u32 *)CONCAT22(uVar7,param_1);
  iVar4 = param_1;
  bad_1010_bf08(param_3,(param_3 >> 0x10),0x2000000);
  (param_1 + 0x8) = iVar4;
  if (iVar4 == 0x0) {
    (param_1 + 0x8) = 0x1;
  }
  uVar5 = (param_1 + 0x8) << 0x2;
  mem_op_1000_179c(uVar5,paVar9);
  puStack6 = uVar5;
  (param_1 + 0x2) = paVar9;
  if ((paVar9 | puStack6) == 0x0) {
    (param_1 + 0x8) = 0x0;
  }
  else {
    iVar4 = (param_1 + 0x8) * 0x2;
    mem_op_1000_179c(iVar4,paVar9);
    (param_1 + 0x4) = iVar4;
    (param_1 + 0x6) = paVar9;
    uVar5 = paVar9 | (param_1 + 0x4);
    if (uVar5 != 0x0) {
      uVar6 = FUN_1010_830a(uVar5,paVar9,0x1000,_u16_1050_14cc,0x1b4);
      puVar1 = (u16 *)*puStack6;
      *puVar1 = uVar6;
      ((int)puVar1 + 0x2) = (int)paVar9;
      (u32)(param_1 + 0x4) = 0x0;
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x200);
      iStack30 = 0x1;
      while( true ) {
        puVar5 = (astruct_257 *)&local_18;
        uVar6 = 0x1028;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,puVar5));
        uVar5 = paVar9;
        uVar8 = uVar5 | puVar5;
        paVar9 = (Struct57*)((u32)paVar9 & 0xffff0000 | (u32)uVar8);
        if (uVar8 == 0x0) break;
        piVar2 = puVar5->field16_0x10;
        uVar8 = piVar2;
        iVar4 = *piVar2;
        iStack34 = 0x0;
        uVar5 = iVar4 - 0x1;
        if (uVar5 < 0xa) {
          uVar6 = 0x1010;
          switch(uVar5) {
          case 0x0:
            iStack34 = 0x1b5;
            uVar5 = uVar8;
            break;
          case 0x1:
            iStack34 = 0x1b6;
            uVar5 = uVar8;
            break;
          case 0x2:
            iStack34 = 0x1b7;
            uVar5 = uVar8;
            break;
          case 0x3:
            iStack34 = 0x1b8;
            uVar5 = uVar8;
            break;
          case 0x4:
            iStack34 = 0x1b9;
            uVar5 = uVar8;
            break;
          case 0x5:
            iStack34 = 0x1ba;
            uVar5 = uVar8;
            break;
          case 0x6:
            iStack34 = 0x1bb;
            uVar5 = uVar8;
            break;
          case 0x7:
            iStack34 = 0x1bc;
            uVar5 = uVar8;
            break;
          case 0x8:
            iStack34 = 0x1bd;
            uVar5 = uVar8;
            break;
          case 0x9:
            iStack34 = 0x1be;
            uVar5 = uVar8;
          }
        }
        uVar6 = FUN_1010_830a(uVar5,paVar9,uVar6,_u16_1050_14cc,iStack34);
        uVar11 = ((u32)*puStack6 >> 0x10);
        iVar10 = (int)*puStack6;
        (iVar10 + iStack30 * 0x4) = uVar6;
        (iVar10 + iStack30 * 0x4 + 0x2) = (int)paVar9;
        uVar3 = (u32)(param_1 + 0x4);
        ((int)uVar3 + iStack30 * 0x2) = iVar4;
        iStack30 += 0x1;
      }
      return;
    }
  }
  return;
}



void pass1_1010_c1ba(u16 param_1,u16 param_2,u16 param_3,param_4: i16)

{
  astruct_92 *paVar1;
  i16 iStack28;
  astruct_92 local_16;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x200);
  iStack28 = 0x1;
  while( true ) {
    paVar1 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    param_1 |= paVar1;
    if ((param_1 == 0x0) || (iStack28 == param_4)) break;
    iStack28 += 0x1;
  }
  return;
}



char * pass1_1010_c234(u16 param_1,u8 *param_2)

{
  char *pcVar1;

  pass1_1010_c28a(param_2);
  if ((param_2 | param_1) == 0x0) {
    return NULL;
  }
  pcVar1 = pass1_1038_4d28((char *)CONCAT22(param_2,param_1));
  return pcVar1;
}



void pass1_1010_c25e(u16 param_1,u8 *param_2,u16 param_3,u16 param_4,char *param_5)

{
  if (param_5 != NULL) {
    pass1_1010_c28a(param_2);
    if ((param_2 | param_1) != 0x0) {
      pass1_1038_4d3c(CONCAT22(param_2,param_1),param_5,param_2 | param_1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_c28a(u8 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  u32 *puVar5;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffee;

  puVar5 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffee,0x2f),in_stack_0000fe96,in_stack_0000ffba,
                           in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = ((u32)puVar5 >> 0x10);
  uVar1 = ((int)puVar5 + 0x24);
  uVar4 = ((int)puVar5 + 0x26);
  uVar3 = uVar4 | uVar1;
  if (uVar3 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar4,uVar1));
    if ((uVar4 | uVar3) != 0x0) {
      return;
    }
  }
  return;
}



void pass1_1010_c2d8(u16 param_1,u16 param_2,i32 param_3)

{
  u16 uVar1;
  u16 uStack6;

  if ((param_3 != 0x0) &&
     (uVar1 = ((u32)param_3 >> 0x10), uStack6 = (u32)((int)param_3 + 0x2e),
     (((int)param_3 + 0x30) | uStack6) != 0x0)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_c312(void)

{
  return CONCAT22(((int)_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_c320(char *param_1,u16 param_2,u16 param_3,u8 *param_4,u32 param_5)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  u32 uVar2;
  u8 *puStack6;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  puStack6 = param_4;
  if (param_4 == NULL) {
    mem_op_1000_179c(0x100,paVar1);
    puStack6 = (u8 *)((u32)param_4 & 0xffff | (long)paVar1 << 0x10);
  }
  uVar2 = struct_op_1030_73a8((astruct_419 *)param_5,(int)param_4,(int)paVar1);
  switch(((int)uVar2 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  default:
    *puStack6 = '\0';
    return;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)puStack6,
             (short)((u32)puStack6 >> 0x10));
  return;
}



void pass1_1010_c3c2(u8 *param_1,u16 param_2,u16 param_3,param_4: u32,u32 param_5)

{
  u16 uVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  Struct57*paVar3;
  u8 local_40c [0x400];
  u16 uStack12;
  u32 uStack10;
  char *pcStack6;

  paVar3 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  pcStack6 = (char *)param_4;
  if (param_4 == 0x0) {
    mem_op_1000_179c(0x100,paVar3);
    pcStack6 = (char *)(param_4 & 0xffff | (long)paVar3 << 0x10);
  }
  uStack10 = struct_op_1030_73a8((astruct_419 *)param_5,(int)param_4,(int)paVar3);
  uVar2 = (uStack10 >> 0x10);
  uStack12 = ((int)uStack10 + 0xc);
  uVar1 = pass1_1020_bd80(uStack12);
  unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_40c),(char *)CONCAT22(uVar2,uVar1));
  pass1_1030_8086(param_5);
  sys_1000_3f9c(pcStack6,(char *)0x105038c5,local_40c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void string_op_1010_c446(u16 param_1,param_2: u32,char *param_3,astruct_419 *param_4)

{
  i16 iVar1;
  u16 in_register_0000000a;
  Struct57*paVar2;
  u32 uVar3;
  char *pcVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  char *in_buffer_4;
  short in_buf_len_5;
  u16 uStack22;
  char *pcStack6;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  pcStack6 = param_3;
  if (param_3 == NULL) {
    mem_op_1000_179c(0x100,paVar2);
    pcStack6 = (char *)((u32)param_3 & 0xffff | (long)paVar2 << 0x10);
  }
  uVar3 = struct_op_1030_73a8(param_4,(int)param_3,(int)paVar2);
  struct_1010_dd5e(param_2,(param_2 >> 0x10),(u32)param_4);
  iVar1 = ((int)uVar3 + 0x12);
  if (0x6 < iVar1 - 0x3U) {
    return;
  }
  in_buffer_4 = (char *)pcStack6;
  in_buf_len_5 = (short)((u32)pcStack6 >> 0x10);
  uVar6 = _u16_1050_14cc;
  uVar7 = (_u16_1050_14cc >> 0x10);
  switch(iVar1) {
  default:
    break;
  case 0x6:
    load_string_1010_84e0(uVar6,uVar7,0x3ff,in_buffer_4,in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,0x61e);
    uVar8 = pcVar4;
    uVar5 = SUB42(s_____s__lu_1050_38d7,0x0);
    goto LAB_1010_c4f9;
  case 0x7:
  case 0x9:
    break;
  case 0x8:
    load_string_1010_84e0(uVar6,uVar7,0x3ff,in_buffer_4,in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,0x61e);
    uVar8 = pcVar4;
    uVar5 = SUB42(s_____s__lu_1050_38cd,0x0);
LAB_1010_c4f9:
    sys_1000_3f9c((char *)((u32)pcStack6 & 0xffff0000 | ZEXT24(in_buffer_4 + uStack22)),(char *)CONCAT22(0x1050,uVar5)
                  ,uVar8);
    return;
  }
  load_string_1010_84e0(uVar6,uVar7,0x3ff,in_buffer_4,in_buf_len_5);
  return;
}



void pass1_1010_c58as(u16 param_1,u16 param_2,astruct_20 *param_3,u16 param_4,u32 param_5)

{
  StructD *pstructd_1_lo;
  u16 pstructd_lo_11;
  u32 uVar1;
  StructD *pstructd_1_hi;
  u16 uVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  Struct57*paVar4;
  Struct57*paVar6;
  StructD *pUStack26;
  HDC16 HStack18;
  u16 UStack16;
  u8 **ppuStack14;
  u16 *pstructd32_1;
  Struct57*paVar5;

  paVar4 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  uVar1 = (u32)param_1;
  mem_op_1000_179c(0x18,paVar4);
  pstructd_1_lo = (StructD *)uVar1;
  pstructd_1_hi = (StructD *)(paVar4 | pstructd_1_lo);
  paVar6 = (Struct57*)((u32)paVar4 & 0xffff0000);
  paVar5 = (Struct57*)((u32)paVar6 | ZEXT24(pstructd_1_hi));
  if (pstructd_1_hi == NULL) {
    pstructd_1_lo = NULL;
  }
  else {
    struct_1040_a598((astruct_259 *)(uVar1 & 0xffff | (long)paVar4 << 0x10));
    paVar6 = paVar5;
  }
  uVar2 = paVar6;
  pstructd32_1 = (u16 *)CONCAT22(uVar2,pstructd_1_lo);
  paVar4 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(uVar2 | pstructd_1_lo));
  if ((uVar2 | pstructd_1_lo) == 0x0) {
    return;
  }
  ppuStack14 = NULL;
  HStack18 = 0x0;
  UStack16 = 0x0;
  switch(param_4) {
  case 0x5:
    ppuStack14 = (u8 **)&u16_1050_352c;
    HStack18 = 0xfa4;
    UStack16 = 0x30;
    break;
  default:
    if (pstructd32_1 == NULL) {
      return;
    }
    pass1_1040_a5d0((StructD *)CONCAT22(uVar2,pstructd_1_lo));
    fn_ptr_1000_17ce((char *)pstructd32_1);
    return;
  case 0xa:
    ppuStack14 = (u8 **)&u16_1050_358c;
    HStack18 = 0xfb3;
    UStack16 = 0x51;
    break;
  case 0xb:
    ppuStack14 = (u8 **)&u16_1050_358c;
    HStack18 = 0xfb4;
    UStack16 = 0x51;
    break;
  case 0xc:
    ppuStack14 = (u8 **)&u16_1050_362e;
    HStack18 = 0xfb6;
    UStack16 = 0x30;
    break;
  case 0x10:
    ppuStack14 = &PTR_DAT_1050_1805_1050_368e;
    HStack18 = 0xfc4;
    UStack16 = 0x3c;
    break;
  case 0x11:
    ppuStack14 = &PTR_DAT_1050_1805_1050_3706;
    HStack18 = 0xfc1;
    UStack16 = 0x4b;
    break;
  case 0x12:
    ppuStack14 = (u8 **)&u16_1050_379c;
    HStack18 = 0xfc5;
    UStack16 = 0x8;
    break;
  case 0x13:
    pass1_1010_debe((u32)param_3,param_4,(u16 *)CONCAT22(uVar2,&pstructd_1_lo->field_0x10),
                    (u32 *)CONCAT22(uVar2,&pstructd_1_lo->field7_0xc),param_5);
    ppuStack14 = (u8 **)&u16_1050_37ac;
    HStack18 = 0xfc6;
    UStack16 = 0x1;
    paVar4 = paVar6;
    break;
  case 0x15:
    (u32)&pstructd_1_lo->field_0x6 = param_5;
    pstructd_1_lo->field6_0xa = param_4;
    break;
  case 0x16:
    ppuStack14 = (u8 **)&u16_1050_37ae;
    HStack18 = 0x157;
    UStack16 = 0x4;
    break;
  case 0x17:
    ppuStack14 = (u8 **)&u16_1050_37b6;
    UStack16 = 0x2c;
    HStack18 = 0xfd8;
  }
  if (ppuStack14 != NULL) {
    *pstructd32_1 = UStack16;
    pstructd_lo_11 = UStack16 * 0xa + 0x2;
    mem_op_1000_179c(pstructd_lo_11,paVar4);
    uVar3 = paVar4;
    pUStack26 = (StructD *)CONCAT22(uVar3,pstructd_lo_11);
    if ((uVar3 | pstructd_lo_11) == 0x0) {
      (u32)&pstructd_1_lo->address_offset_field_0x2 = 0x0;
    }
    else {
      pUStack26->address_offset_field_0x0 = UStack16;
      pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,UStack16,0xa,pstructd_lo_11 + 0x2,uVar3);
      pstructd_1_lo->address_offset_field_0x2 = pstructd_lo_11 + 0x2;
      pstructd_1_lo->hfile_0x4 = uVar3;
    }
    (u32)&pstructd_1_lo->field_0x6 = param_5;
    pstructd_1_lo->field6_0xa = param_4;
    pstructd_1_lo->field11_0x12 = HStack18;
    pass1_1010_a50c(param_3,ppuStack14,(StructD *)CONCAT22(uVar2,pstructd_1_lo));
  }
  return;
}



void pass1_1010_c7e2(param_1: u32,param_2: u32,i16 *param_3)

{
  u32 uVar1;
  char *pcVar2;
  u16 in_DX;
  i16 iVar3;
  i16 unaff_SI;
  u16 uVar4;
  u16 *puStack8;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    uVar4 = ((u32)param_3 >> 0x10);
    iVar3 = (int)param_3;
    if (*param_3 == iStack4 || *param_3 < iStack4) break;
    uVar1 = (u32)(iVar3 + 0x2);
    (iStack4 * 0xa + (int)uVar1 + 0x4) = (iStack4 * 0x2 + (int)param_2);
    iStack4 += 0x1;
  }
  puStack8 = (u16*)(iVar3 + 0x2);
  for (iStack4 = 0x0; *param_3 != iStack4 && iStack4 <= *param_3; iStack4 += 0x1) {
    uVar1 = (u32)(iVar3 + 0x6);
    pcVar2 = pass1_1010_b038((u8 *)param_1,uVar1,((u32)uVar1 >> 0x10),
                             *(u8 **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(in_DX,puStack8,(char *)CONCAT22(in_DX,pcVar2));
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0xa));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_c864(char *param_1,param_2: u32,u16 *param_3,astruct_104 *param_4)

{
  StructD **ppSVar1;
  let mut lVar2: i32;
  code **ppcVar3;
  u32 uVar4;
  char *uVar12;
  char *pcVar5;
  u16 uVar6;
  u32 uVar7;
  u16 uVar8;
  u16 DX_REG;
  u16 DX_REG_00;
  u16 uVar9;
  astruct_104 *struct_1;
  i16 iVar10;
  astruct_104 *struct_1_seg;
  u16 uVar11;
  u16 uVar13;
  u16 uVar14;
  u16 uVar15;
  u32 local_1f0;
  char *pcStack412;
  u32 uStack408;
  char *uStack404;
  u16 uStack402;
  u16 uStack400;
  u32 uStack398;
  StructD *local_18a;
  u32 local_f6;
  u16 *puStack98;
  i16 iStack94;
  u32 uStack92;
  astruct_104 *struct_2;
  u8 *puStack86;
  char string_1 [0x52];

  puStack86 = NULL;
  struct_1_seg = (astruct_104 *)((u32)param_4 >> 0x10);
  struct_1 = (astruct_104 *)param_4;
  struct_2 = (astruct_104 *)param_4->field0_0x0;
  uVar12 = NULL;
  uStack92 = 0x0;
  uVar14 = param_2;
  uVar15 = (param_2 >> 0x10);
  pass1_1010_c320(param_1,uVar14,uVar15,NULL,(u32)(struct_1 + 0x1));
  unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,string_1),(char *)CONCAT22(param_1,uVar12));
  puStack98 = (u16 *)struct_1->field1_0x2;
  uStack402 = ((int)&struct_1->field1_0x2 + 0x2);
  ((int)puStack98 + 0x4) = *param_3;
  string_1040_a626(uStack402,puStack98,(char *)CONCAT22(0x1050,string_1));
  iStack94 = 0x0;
  pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_f6),NULL,0x94);
  uStack404 = (char *)pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_18a),NULL,0x94);
  uStack398 = 0x0;
  for (uStack400 = 0x1; (int)uStack400 < 0x25; uStack400 += 0x1) {
    _uStack404 = (u32 *)pass1_1030_7c28(uStack404,uStack402,(u32)(struct_1 + 0x1),uStack400);
    uStack402 = ((u32)_uStack404 >> 0x10) | uStack404;
    if (_uStack404 != NULL) {
      pcVar5 = string_1020_c0d8(uStack400);
      uStack408 = CONCAT22(uStack402,pcVar5);
      uVar8 = uStack402 | pcVar5;
      if (uVar8 == 0x0) {
        unk_str_op_1000_3d3e((char *)(&local_f6)[uStack398],s_Null_Ptr_1050_38e1);
      }
      else {
        uVar6 = str_op_1008_60e8(uVar8,(char *)CONCAT22(uStack402,pcVar5));
        (&local_f6 + uStack398) = uVar6;
        ((int)&local_f6 + uStack398 * 0x4 + 0x2) = uVar8;
      }
      *(char **)(&local_18a + uStack398) = uStack404;
      ((int)&local_18a + uStack398 * 0x4 + 0x2) = uStack402;
      uStack398 += 0x1;
    }
  }
  uStack92 = uStack398;
  if (0x13 < uStack398) {
    iStack94 = 0x1;
  }
  puStack86 = (u8 *)
              pass1_1010_db2e(uVar14,uVar15,0x1,CONCAT22(0x1050,&local_f6),CONCAT22(0x1050,&local_18a),(u32)param_3,
                              param_4);
  while (uVar7 = uStack398 - 0x1, uStack398 != 0x0) {
    uStack398 = uVar7;
    pcStack412 = (char *)(&local_f6)[uStack398];
    _uStack404 = (u32 *)pcStack412;
    uStack398 = uVar7;
    fn_ptr_1000_17ce(pcStack412);
  }
  uVar13 = 0x1000;
  uStack398 = uVar7;
  pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_18a),NULL,0x54);
  uVar4 = (u32)(struct_1 + 0x1);
  uVar11 = ((u32)uVar4 >> 0x10);
  iVar10 = (int)uVar4;
  _uStack404 = (u32 *)(u32)(iVar10 + 0x1e);
  uVar8 = (iVar10 + 0x20) | uStack404;
  uVar7 = (u32)uVar8;
  if (uVar8 != 0x0) {
    uStack398 = 0x0;
    while( true ) {
      uVar4 = *_uStack404;
      ppcVar3 = (code **)((int)uVar4 + 0x10);
      (**ppcVar3)(uVar13,(int)_uStack404,(int)((u32)_uStack404 >> 0x10));
      if ((DX_REG < uStack398) || ((DX_REG <= uStack398 && (uVar7 <= uStack398))))
      break;
      ppcVar3 = (code **)((int)uVar4 + 0x4);
      (**ppcVar3)(uVar13,_uStack404,(char)uStack398,uStack398);
      uVar8 = uVar7;
      uVar9 = DX_REG_00 | uVar8;
      if (uVar9 != 0x0) {
        uVar13 = 0x1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | (u32)DX_REG_00 << 0x10);
        uStack408 = CONCAT22(uVar9,uVar8);
        if ((uVar9 | uVar8) == 0x0) {
          return;
        }
        iVar10 = (uVar8 + 0xc);
        uVar7 = (u32)(iVar10 - 0x1U);
        if (((0x0 < iVar10) && (!SBORROW2(iVar10,0x1))) &&
           (uVar7 = (u32)(iVar10 - 0x6U), iVar10 - 0x6U == 0x0 || (int)(iVar10 - 0x1U) < 0x5)) {
          ppSVar1 = &local_18a + iVar10;
          *ppSVar1 = (StructD *)((long)&(*ppSVar1)->address_offset_field_0x0 + 0x1);
        }
      }
      uStack398 += 0x1;
    }
    uVar8 = DX_REG;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_f6),NULL,0x54);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_1f0),NULL,0x54);
    uStack398 = 0x0;
    for (uStack400 = 0x1; (int)uStack400 < 0x15; uStack400 += 0x1) {
      if ((&local_18a)[uStack400] != NULL) {
        pcVar5 = string_op_1020_c222(uStack400);
        uVar9 = uVar8 | pcVar5;
        if (uVar9 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_f6)[uStack398],s_Null_Ptr_1050_38ea);
        }
        else {
          uVar6 = str_op_1008_60e8(uVar9,(char *)CONCAT22(uVar8,pcVar5));
          (&local_f6 + uStack398) = uVar6;
          ((int)&local_f6 + uStack398 * 0x4 + 0x2) = uVar9;
        }
        uVar8 = ((int)&local_18a + uStack400 * 0x4 + 0x2);
        (&local_1f0 + uStack398) = (&local_18a + uStack400);
        ((int)&local_1f0 + uStack398 * 0x4 + 0x2) = uVar8;
        uStack398 += 0x1;
      }
    }
    if (iStack94 == 0x0) {
      iVar10 = (int)(uStack92 >> 0x10) + CARRY2(uStack92,uStack398);
      uStack92 = CONCAT22(iVar10,uStack92 + uStack398);
      if ((iVar10 != 0x0) || (0x13 < uStack92 + uStack398)) {
        iStack94 = 0x1;
      }
    }
    if ((puStack86 < (u8 *)((int)&struct_2[-0x1].field1_0x2 + 0x2U)) && (local_1f0 != 0x0)) {
      iVar10 = string_1010_dcac(uVar14,uVar15,(int)puStack86,(u32)param_3,param_4);
      puStack86 = (u8 *)(iVar10 + 0x1);
      puStack86 = (u8 *)
                  pass1_1010_db2e(uVar14,uVar15,puStack86,CONCAT22(0x1050,&local_f6),CONCAT22(0x1050,&local_1f0),
                                  (u32)param_3,param_4);
    }
    while (lVar2 = uStack398 + -0x1, uStack398 != 0x0) {
      uStack398 = lVar2;
      pcStack412 = (char *)(&local_f6)[uStack398];
      uStack398 = lVar2;
      fn_ptr_1000_17ce(pcStack412);
    }
    if (iStack94 != 0x0) {
      ((int)&struct_1[0x3].field1_0x2 + 0x2) = 0x1;
    }
    uStack398 = lVar2;
    pass1_1010_dc36(uVar14,uVar15,puStack86,(u32)param_3,param_4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_cc56(u16 param_1,param_2: u32,param_3: u32,astruct_104 *param_4)

{
  StructD **ppSVar1;
  u32 uVar2;
  char *pcVar3;
  u16 uVar4;
  i16 iVar5;
  u32 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  StructD *local_1a0;
  u32 uStack332;
  u16 uStack328;
  u16 uStack326;
  u32 uStack324;
  u16 uStack320;
  StructD *local_13e;
  StructD *local_aa;
  u16 uStack22;
  i16 iStack18;
  u16 uStack16;
  u16 uStack14;
  u16 uStack12;
  u32 uStack10;
  u32 uStack6;

  uVar9 = (param_2 >> 0x10);
  uVar8 = param_2;
  uVar2 = (u32)(uVar8 + 0x138);
  uVar6 = (u32)((int)uVar2 + 0x24);
  uStack6 = uVar6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
  uStack10 = uVar6 & 0xffff | (u32)param_1 << 0x10;
  uStack324 = param_1 | uVar6;
  if (uStack324 != 0x0) {
    uStack14 = param_4->field0_0x0;
    iStack18 = 0x0;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_aa),NULL,0x94);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_13e),NULL,0x94);
    uStack12 = 0x0;
    uStack16 = 0x0;
    uStack22 = 0x0;
    for (uStack320 = 0x1; (int)uStack320 < 0x25; uStack320 += 0x1) {
      uStack324 = *(i32 *)((int)uStack10 + 0x26 + uStack320 * 0x4);
      if (uStack324 != 0x0) {
        pcVar3 = string_1020_c0d8(uStack320);
        uStack332 = uStack332 & 0xffff | ZEXT24(pcVar3) << 0x10;
        uVar7 = uStack324 | pcVar3;
        uStack328 = uStack324;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_aa)[uStack22],s_Null_Ptr_1050_38f3);
        }
        else {
          uVar4 = str_op_1008_60e8(uVar7,(char *)CONCAT22(uStack324,pcVar3));
          (&local_aa + uStack22) = uVar4;
          ((int)&local_aa + uStack22 * 0x4 + 0x2) = uVar7;
        }
        (&local_13e + uStack22) = uStack324;
        ((int)&local_13e + uStack22 * 0x4 + 0x2) = uStack324;
        uStack22 += 0x1;
      }
    }
    uStack16 = uStack22;
    if (0x13 < uStack22) {
      iStack18 = 0x1;
    }
    uStack12 = pass1_1010_db2e(uVar8,uVar9,0x1,CONCAT22(0x1050,&local_aa),CONCAT22(0x1050,&local_13e),param_3,param_4);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_13e),NULL,0x54);
    for (uStack332 = 0x1; uStack332 < 0x15; uStack332 += 0x1) {
      uStack326 = uStack332;
      if (*(i32 *)((int)uStack10 + 0x14e + uStack332 * 0x4) != 0x0) {
        if (((0x0 < (int)uStack332) && (!SBORROW2(uStack332,0x1))) && ((int)(uStack332 - 0x1) < 0x6))
        {
          ppSVar1 = &local_13e + uStack332;
          *ppSVar1 = (StructD *)((long)&(*ppSVar1)->address_offset_field_0x0 + 0x1);
        }
      }
    }
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_aa),NULL,0x54);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_1a0),NULL,0x54);
    for (uStack332 = 0x10000; (int)uStack332 < 0x15;
        uStack332 = uStack332 & 0xffff | (u32)(uStack332 + 0x1) << 0x10) {
      if ((&local_13e)[uStack332] != NULL) {
        pcVar3 = string_op_1020_c222(uStack332);
        uStack324 = CONCAT22(uStack324,pcVar3);
        uVar7 = uStack324 | pcVar3;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_aa)[uStack332],s_Null_Ptr_1050_38fc);
        }
        else {
          uVar4 = str_op_1008_60e8(uVar7,(char *)CONCAT22(uStack324,pcVar3));
          (&local_aa + uStack332) = uVar4;
          ((int)&local_aa + uStack332 * 0x4 + 0x2) = uVar7;
        }
        uStack324 = ((int)&local_13e + uStack332 * 0x4 + 0x2);
        (&local_1a0 + uStack332) = (&local_13e + uStack332);
        ((int)&local_1a0 + uStack332 * 0x4 + 0x2) = uStack324;
        uStack332 = uStack332 & 0xffff0000 | (u32)(uStack332 + 0x1);
      }
    }
    if (iStack18 == 0x0) {
      uStack16 += uStack332;
      if (0x13 < uStack16) {
        iStack18 = 0x1;
      }
    }
    if ((uStack12 < uStack14 - 0x2) && (local_1a0 != NULL)) {
      iVar5 = string_1010_dcac(uVar8,uVar9,uStack12,param_3,param_4);
      uStack12 = iVar5 + 0x1;
      uStack12 = pass1_1010_db2e(uVar8,uVar9,uStack12,CONCAT22(0x1050,&local_aa),CONCAT22(0x1050,&local_1a0),param_3,
                                 param_4);
    }
    if (iStack18 != 0x0) {
      ((int)param_4 + 0x16) = 0x1;
    }
    pass1_1010_dc36(uVar8,uVar9,uStack12,param_3,param_4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_cf36(u16 param_1,param_2: u32,param_3: u32,astruct_104 *param_4)

{
  u32 uVar1;
  char *pcVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  i16 unaff_SI;
  i16 iVar7;
  astruct_104 *struct_104_1;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u32 uVar11;
  u32 uVar12;
  u16 uVar13;
  u16 uVar14;
  u16 uVar15;
  u16 uStack326;
  i16 iStack316;
  u16 uStack314;
  i16 iStack312;
  u16 local_136 [0x4a];
  u32 local_a2;
  i16 iStack14;
  u32 uStack12;
  u16 *puStack8;
  i16 iStack4;

  iStack4 = 0x0;
  do {
    uVar8 = (param_3 >> 0x10);
    iVar7 = (int)param_3;
    uVar9 = ((u32)param_4 >> 0x10);
    struct_104_1 = (astruct_104 *)param_4;
    uVar1 = struct_104_1->field1_0x2;
    (iStack4 * 0xa + (int)uVar1 + 0x4) = (iStack4 * 0x2 + iVar7);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = (u16 *)struct_104_1->field1_0x2;
  iStack4 = 0x0;
  do {
    uVar11 = (u32)(struct_104_1 + 0x1);
    pcVar2 = pass1_1010_b038((u8 *)param_2,uVar11,((u32)uVar11 >> 0x10),
                             *(u8 **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_1,puStack8,(char *)CONCAT22(param_1,pcVar2));
    iStack4 += 0x1;
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar13 = param_2;
  uVar14 = (param_2 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,(u32)(struct_104_1 + 0x1));
  uStack12 = CONCAT22(param_1,pcVar2);
  uVar4 = param_1 | pcVar2;
  if (uVar4 != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_a2),NULL,0x94);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,local_136),NULL,0x94);
    uStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar11 = (u32)(struct_104_1 + 0x1);
    uVar1 = (u32)((int)uVar11 + 0x26);
    uVar12 = uVar1;
    for (uStack326 = 0x1; (int)uStack326 < 0x25; uStack326 += 0x1) {
      uVar15 = (uVar1 >> 0x10);
      if (*(i32 *)(uStack326 * 0x4 + (int)uStack12) == 0x0) {
        if (uVar1 != 0x0) {
          uVar12 = pass1_1020_bae6(uVar12,uVar4,uVar1,CONCAT22(uStack326,uVar15));
          uVar6 = (uVar12 >> 0x10);
          uVar12 &= 0xffff;
          uVar4 = uVar6 | uVar12;
          if (uVar4 != 0x0) {
            if (iStack14 == 0x0) {
              iStack14 = 0x1;
            }
            pcVar2 = string_1020_c0d8(uStack326);
            uVar5 = uVar4 | pcVar2;
            if (uVar5 == 0x0) {
              unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_390e);
            }
            else {
              uVar3 = str_op_1008_60e8(uVar5,(char *)CONCAT22(uVar4,pcVar2));
              (&local_a2 + iStack312) = uVar3;
              ((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar5;
            }
            local_136[iStack312 * 0x2] = uVar12;
            local_136[iStack312 * 0x2 + 0x1] = uVar6;
            goto LAB_1010_d11d;
          }
        }
      }
      else {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar2 = string_1020_c0d8(uStack326);
        uVar6 = uVar4 | pcVar2;
        if (uVar6 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_3905);
        }
        else {
          uVar3 = str_op_1008_60e8(uVar6,(char *)CONCAT22(uVar4,pcVar2));
          (&local_a2 + iStack312) = uVar3;
          ((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar6;
        }
        uVar10 = ((u32)uStack12 >> 0x10);
        uVar4 = (uStack326 * 0x4 + (int)uStack12);
        uVar6 = (uStack326 * 0x4 + (int)uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar4;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        if (uVar1 == 0x0) {
          uVar4 = 0x0;
        }
        else {
          uVar11 = pass1_1020_bae6(uVar4,uVar6,uVar1,CONCAT22(uStack326,uVar15));
          uVar6 = ((u32)uVar11 >> 0x10);
          uVar4 = uVar11;
        }
        uVar12 = (u32)uVar4;
        if (uVar4 == 0x0) {
          iStack316 += 0x2;
          uVar4 = uVar6;
          iStack312 = iStack312 + 0x1;
        }
        else {
LAB_1010_d11d:
          iStack312 += 0x1;
          (uVar13 + uStack314 * 0x2 + 0xa4) = (iVar7 + iStack316 * 0x2 + 0x10);
          (uVar13 + (uStack314 + 0x1) * 0x2 + 0xa4) =
               (iVar7 + (iStack316 + 0x1) * 0x2 + 0x10);
          iStack316 += 0x2;
          uStack314 += 0x2;
          uVar12 = (u32)uStack314;
          uVar4 = uVar6;
        }
      }
    }
    uVar4 = pass1_1010_db2e(uVar13,uVar14,0x8,CONCAT22(0x1050,&local_a2),CONCAT22(0x1050,local_136),param_3,param_4);
    if (iStack14 != 0x0) {
      ((int)&struct_104_1[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((char *)(&local_a2)[iStack312 + -0x1]);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar4,param_3,param_4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_d24a(u8 param_1,u16 param_2,param_3: u32,param_4: u32,astruct_104 *param_5)

{
  u32 uVar1;
  u32 uVar2;
  char *pcVar3;
  u16 *puVar4;
  u16 uVar5;
  u16 *puVar6;
  u16 *puVar7;
  u16 uVar8;
  i16 unaff_SI;
  astruct_104 *iVar9;
  u16 uVar9;
  let mut lVar10: i32;
  u16 uVar11;
  u16 uVar12;
  u16 uStack320;
  let mut lStack318: i32;
  u16 *local_13a [0x4a];
  u32 local_a6;
  i16 iStack18;
  u32 uStack16;
  char *pcStack12;
  u16 uStack10;
  u16 *puStack8;
  i16 iStack4;

  iStack4 = 0x0;
  do {
    uVar9 = ((u32)param_5 >> 0x10);
    iVar9 = (astruct_104 *)param_5;
    uVar2 = iVar9->field1_0x2;
    (iStack4 * 0xa + (int)uVar2 + 0x4) = (iStack4 * 0x2 + (int)param_4);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = (u16 *)iVar9->field1_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = (u32)(iVar9 + 0x1);
    pcVar3 = pass1_1010_b038((u8 *)param_3,uVar1,((u32)uVar1 >> 0x10),
                             *(u8 **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_2,puStack8,(char *)CONCAT22(param_2,pcVar3));
    iStack4 += 0x1;
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar11 = param_3;
  uVar12 = (param_3 >> 0x10);
  struct_1010_dd5e(uVar11,uVar12,(u32)(iVar9 + 0x1));
  puVar6 = (u16 *)(param_2 | pcVar3);
  if (puVar6 != NULL) {
    pcStack12 = pcVar3;
    uStack10 = param_2;
    pass1_1010_e8f6(pcVar3,puVar6,uVar11,uVar12,(u32)(iVar9 + 0x1));
    uStack16 = CONCAT22(puVar6,pcVar3);
    iStack18 = 0x0;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_a6),NULL,0x94);
    puVar4 = pass1_1000_4906((StructD *)CONCAT22(0x1050,local_13a),NULL,0x94);
    lStack318 = 0x0;
    for (uStack320 = 0x1; (int)uStack320 < 0x25; uStack320 += 0x1) {
      lVar10 = pass1_1030_7c28(puVar4,puVar6,uStack16,uStack320);
      puVar7 = (u16 *)((u32)lVar10 >> 0x10);
      puVar4 = (u16 *)lVar10;
      puVar6 = (u16 *)(puVar7 | puVar4);
      if (lVar10 != 0x0) {
        if (iStack18 == 0x0) {
          iStack18 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack320);
        uVar8 = puVar6 | pcVar3;
        if (uVar8 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a6)[(int)lStack318],s_Null_Ptr_1050_3917);
        }
        else {
          uVar5 = str_op_1008_60e8(uVar8,(char *)CONCAT22(puVar6,pcVar3));
          (&local_a6 + (int)lStack318) = uVar5;
          ((int)&local_a6 + (int)lStack318 * 0x4 + 0x2) = uVar8;
        }
        local_13a[(int)lStack318 * 0x2] = puVar4;
        local_13a[(int)lStack318 * 0x2 + 0x1] = puVar7;
        lStack318 += 0x1;
        puVar6 = puVar7;
      }
    }
    uVar8 = pass1_1010_db2e(uVar11,uVar12,0x8,CONCAT22(0x1050,&local_a6),CONCAT22(0x1050,local_13a),param_4,param_5);
    if (iStack18 != 0x0) {
      ((int)&iVar9[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (lStack318 != 0x0) {
      lStack318 = (int)(lStack318 + -0x1);
      fn_ptr_1000_17ce((char *)(&local_a6)[(int)lStack318]);
      lStack318 = lStack318 + -0x1;
    }
    pass1_1010_dc36(uVar11,uVar12,uVar8,param_4,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_d448(u16 param_1,u16 param_2,u8 *param_3,param_4: u32,astruct_104 *param_5,i16 param_6
                    )

{
  u16 uVar1;
  u16 *puVar2;
  u32 uVar3;
  u32 uVar4;
  u16 *puVar5;
  char *pcVar6;
  i16 iVar7;
  u16 uVar8;
  u16 uVar9;
  astruct_104 *iVar10;
  astruct_104 *uVar10;
  u32 uVar11;
  u16 uVar12;
  u16 local_40c;
  u32 uStack1034;
  u32 uStack1030;
  u8 local_402 [0x400];

  uVar10 = (astruct_104 *)((u32)param_5 >> 0x10);
  iVar10 = (astruct_104 *)param_5;
  uStack1030 = struct_op_1030_73a8(*(astruct_419 **)(iVar10 + 0x1),param_1,param_2);
  uVar8 = (uStack1030 >> 0x10);
  uVar1 = uStack1030;
  if ((uVar8 | uVar1) != 0x0) {
    uStack1034 = (u32)(uVar1 + 0x20);
    uVar1 = (uVar1 + 0x22);
    if ((uVar1 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar5 = &local_40c;
      uVar12 = ((u32)param_3 >> 0x10);
      pass1_1010_d984(param_3,uVar12,(i16 *)CONCAT22(0x1050,puVar5),0x3,
                      uStack1034 & 0xffff | (u32)uVar1 << 0x10,(u32)&PTR_DAT_1050_1805_1050_368e,(u32)param_5);
      puVar2 = (u16 *)iVar10->field1_0x2;
      uVar9 = ((int)&iVar10->field1_0x2 + 0x2);
      ((int)puVar2 + 0x4) = PTR_DAT_1050_1805_1050_368e;
      uVar3 = (u32)(iVar10 + 0x1);
      pcVar6 = pass1_1010_b038(param_3,uVar3,((u32)uVar3 >> 0x10),*(u8 **)((int)puVar2 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_402),(char *)CONCAT22(uVar9,pcVar6));
      string_1040_a626(uVar9,puVar2,(char *)CONCAT22(0x1050,local_402));
      uVar4 = iVar10->field1_0x2;
      uVar9 = ((int)&iVar10->field1_0x2 + 0x2);
      iVar7 = (int)uVar4;
      (iVar7 + 0xe) = PTR_DAT_1050_1822_1050_3690;
      sys_1000_3f9c((char *)CONCAT22(0x1050,local_402),(char *)0x10503920,local_40c);
      string_1040_a626(uVar9,(u16 *)(uVar4 & 0xffff0000 | (u32)(iVar7 + 0xa)),(char *)CONCAT22(0x1050,local_402));
      uVar4 = iVar10->field1_0x2;
      iVar7 = (int)uVar4;
      (iVar7 + 0x18) = PTR_DAT_1050_1823_1050_3692;
      uVar11 = pass1_1028_62c8(uStack1030);
      uVar9 = (uVar11 >> 0x10);
      sys_1000_3f9c((char *)CONCAT22(0x1050,local_402),(char *)0x10503923,uVar11);
      string_1040_a626(uVar9,(u16 *)(uVar4 & 0xffff0000 | (u32)(iVar7 + 0x14)),(char *)CONCAT22(0x1050,local_402));
      pass1_1010_dc36(param_3,uVar12,puVar5,param_4,param_5);
    }
  }
  return;
}



void pass1_1010_d5ae(u16 param_1,u16 param_2,u8 *param_3,param_4: u32,astruct_104 *param_5,i16 param_6
                    )

{
  u16 *puVar1;
  u32 uVar2;
  u8 *puVar3;
  u16 *puVar4;
  char *pcVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  i16 iVar10;
  u16 uVar11;
  u32 uVar12;
  u16 uVar13;
  u16 local_40c;
  u16 uStack1034;
  u16 uStack1032;
  astruct_15 *paStack1030;
  char string_402 [0x400];
  u32 uVar9;

  uVar11 = ((u32)param_5 >> 0x10);
  iVar10 = (int)param_5;
  paStack1030 = (astruct_15 *)struct_op_1030_73a8(*(astruct_419 **)(iVar10 + 0x6),param_1,param_2);
  uStack1034 = paStack1030;
  uVar7 = ((u32)paStack1030 >> 0x10) | uStack1034;
  uVar9 = CONCAT22(in_register_0000000a,uVar7);
  if (uVar7 != 0x0) {
    pass1_1028_45fe(uStack1034,paStack1030,uVar9);
    uStack1032 = uVar9;
    if ((uStack1032 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar4 = &local_40c;
      uVar13 = ((u32)param_3 >> 0x10);
      pass1_1010_d984(param_3,uVar13,(i16 *)CONCAT22(0x1050,puVar4),0x3,CONCAT22(uStack1032,uStack1034),
                      (u32)&PTR_DAT_1050_1805_1050_3706,(u32)param_5);
      puVar1 = (u16*)(iVar10 + 0x2);
      uVar8 = (iVar10 + 0x4);
      ((int)puVar1 + 0x4) = PTR_DAT_1050_1805_1050_3706;
      uVar9 = (u32)(iVar10 + 0x6);
      pcVar5 = pass1_1010_b038(param_3,uVar9,((u32)uVar9 >> 0x10),*(u8 **)((int)puVar1 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,string_402),(char *)CONCAT22(uVar8,pcVar5));
      string_1040_a626(uVar8,puVar1,(char *)CONCAT22(0x1050,string_402));
      uVar2 = (u32)(iVar10 + 0x2);
      uVar8 = (iVar10 + 0x4);
      iVar6 = (int)uVar2;
      (iVar6 + 0xe) = PTR_DAT_1050_1822_1050_3708;
      sys_1000_3f9c((char *)CONCAT22(0x1050,string_402),s__u_1050_3926,local_40c);
      string_1040_a626(uVar8,(u16 *)(uVar2 & 0xffff0000 | (u32)(iVar6 + 0xa)),(char *)CONCAT22(0x1050,string_402));
      puVar3 = PTR_DAT_1050_1823_1050_370a;
      uVar2 = (u32)(iVar10 + 0x2);
      iVar10 = (iVar10 + 0x4);
      iVar6 = (int)uVar2;
      (iVar6 + 0x18) = PTR_DAT_1050_1823_1050_370a;
      uVar12 = pass1_1028_45e2(puVar3,iVar10,(u32)paStack1030);
      uVar8 = (uVar12 >> 0x10);
      sys_1000_3f9c((char *)CONCAT22(0x1050,string_402),(char *)0x10503929,uVar12);
      string_1040_a626(uVar8,(u16 *)(uVar2 & 0xffff0000 | (u32)(iVar6 + 0x14)),(char *)CONCAT22(0x1050,string_402))
      ;
      pass1_1010_dc36(param_3,uVar13,puVar4,param_4,param_5);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1010_d710(u8 param_1,u16 param_2,param_3: u32,param_4: u32,astruct_104 *param_5)

{
  let mut lVar1: i32;
  u32 uVar2;
  char *pcVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  i16 unaff_SI;
  i16 iVar8;
  astruct_104 *iVar9;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u32 uVar12;
  u16 uVar13;
  u16 uVar14;
  u16 uStack322;
  i16 iStack316;
  i16 iStack314;
  i16 iStack312;
  u16 local_136 [0x4a];
  u32 local_a2;
  i16 iStack14;
  u32 uStack12;
  u16 *puStack8;
  i16 iStack4;

  iStack4 = 0x0;
  do {
    uVar9 = (param_4 >> 0x10);
    iVar8 = (int)param_4;
    uVar10 = ((u32)param_5 >> 0x10);
    iVar9 = (astruct_104 *)param_5;
    uVar2 = iVar9->field1_0x2;
    (iStack4 * 0xa + (int)uVar2 + 0x4) = (iStack4 * 0x2 + iVar8);
    iStack4 += 0x1;
  } while (iStack4 < 0x4);
  puStack8 = (u16 *)iVar9->field1_0x2;
  iStack4 = 0x0;
  do {
    uVar12 = (u32)(iVar9 + 0x1);
    pcVar3 = pass1_1010_b038((u8 *)param_3,uVar12,((u32)uVar12 >> 0x10),
                             *(u8 **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_2,puStack8,(char *)CONCAT22(param_2,pcVar3));
    iStack4 += 0x1;
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0xa));
  } while (iStack4 < 0x4);
  uVar13 = param_3;
  uVar14 = (param_3 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,(u32)(iVar9 + 0x1));
  uStack12 = CONCAT22(param_2,pcVar3);
  uVar6 = param_2 | pcVar3;
  if (uVar6 != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_a2),NULL,0x94);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,local_136),NULL,0x94);
    iStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar12 = (u32)(iVar9 + 0x1);
    lVar1 = *(i32 *)((int)uVar12 + 0x26);
    for (uStack322 = 0x1; (int)uStack322 < 0x25; uStack322 += 0x1) {
      if (*(i32 *)(uStack322 * 0x4 + (int)uStack12) != 0x0) {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack322);
        uVar7 = uVar6 | pcVar3;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_392c);
        }
        else {
          uVar5 = str_op_1008_60e8(uVar7,(char *)CONCAT22(uVar6,pcVar3));
          (&local_a2 + iStack312) = uVar5;
          ((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
        }
        uVar11 = ((u32)uStack12 >> 0x10);
        uVar7 = (uStack322 * 0x4 + (int)uStack12);
        uVar6 = (uStack322 * 0x4 + (int)uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar7;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        iStack312 += 0x1;
        if (lVar1 == 0x0) {
          iVar4 = 0x0;
        }
        else {
          uVar12 = pass1_1020_bae6(uVar7,uVar6,lVar1,CONCAT22(uStack322,(int)((u32)lVar1 >> 0x10)));
          uVar6 = ((u32)uVar12 >> 0x10);
          iVar4 = (int)uVar12;
        }
        if (iVar4 == 0x0) {
          iStack316 += 0x2;
        }
        else {
          (uVar13 + iStack314 * 0x2 + 0xa4) = (iVar8 + iStack316 * 0x2 + 0x8);
          (uVar13 + (iStack314 + 0x1) * 0x2 + 0xa4) =
               (iVar8 + (iStack316 + 0x1) * 0x2 + 0x8);
          iStack316 += 0x2;
          iStack314 += 0x2;
        }
      }
    }
    uVar6 = pass1_1010_db2e(uVar13,uVar14,0x4,CONCAT22(0x1050,&local_a2),CONCAT22(0x1050,local_136),param_4,param_5);
    if (iStack14 != 0x0) {
      ((int)&iVar9[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((char *)(&local_a2)[iStack312 + -0x1]);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar6,param_4,param_5);
  }
  return;
}



void pass1_1010_d984(u16 param_1,u16 param_2,i16 *param_3,i16 param_4,param_5: u32,param_6: u32,u32 param_7)

{
  u16 uVar1;
  u8 *puVar2;
  char *pcVar3;
  i16 iVar4;
  u16 DX_REG;
  u16 uVar5;
  u16 uVar6;
  i16 iVar7;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  u8 local_418 [0x400];
  u16 uStack24;
  char *pcStack22;
  u16 uStack18;
  u32 uStack16;
  u8 local_c [0x8];
  i16 iStack4;

  iStack4 = param_4;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_c),param_5);
  do {
    puVar2 = local_c;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
    uStack16 = CONCAT22(DX_REG,puVar2);
    uVar5 = DX_REG | puVar2;
    if (uVar5 == 0x0) {
      return;
    }
    uStack18 = (puVar2 + 0xa);
    pcStack22 = NULL;
    if ((puVar2 + 0x4) == 0x0) {
      if ((puVar2 + 0x6) == 0x0) {
        if ((puVar2 + 0x8) == 0x0) {
          return;
        }
        pcVar3 = string_op_1020_c2f8((puVar2 + 0x8));
      }
      else {
        pcVar3 = string_op_1020_c222((puVar2 + 0x6));
      }
    }
    else {
      pcVar3 = string_1020_c0d8((puVar2 + 0x4));
    }
    pcStack22 = (char *)CONCAT22(uVar5,pcVar3);
    uStack24 = ((int)uStack16 + 0xc);
    *param_3 = *param_3 + uStack24;
    uVar9 = (param_7 >> 0x10);
    iVar7 = (int)param_7;
    uVar1 = (iVar7 + 0x4);
    iVar4 = (iVar7 + 0x2) + iStack4 * 0xa;
    uVar10 = (param_6 >> 0x10);
    iVar8 = (int)param_6;
    (iVar4 + 0x4) = (iStack4 * 0x2 + iVar8);
    uVar6 = uVar1;
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_418),(char *)0x10503935,uStack18);
    string_1040_a626(uVar6,(u16 *)CONCAT22(uVar1,iVar4),(char *)CONCAT22(0x1050,local_418));
    uVar1 = (iVar7 + 0x4);
    iStack4 += 0x1;
    iVar4 = (iVar7 + 0x2) + iStack4 * 0xa;
    (iVar4 + 0x4) = (iStack4 * 0x2 + iVar8);
    uVar6 = uVar1;
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_418),pcStack22);
    string_1040_a626(uVar6,(u16 *)CONCAT22(uVar1,iVar4),(char *)CONCAT22(0x1050,local_418));
    iVar4 = (iStack4 + 0x1) * 0xa + (iVar7 + 0x2);
    uVar1 = (iVar7 + 0x4);
    (iVar4 + 0x4) = ((iStack4 + 0x1) * 0x2 + iVar8);
    iStack4 += 0x2;
    uVar6 = uVar1;
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_418),(char *)0x10503938,uStack24);
    string_1040_a626(uVar6,(u16 *)CONCAT22(uVar1,iVar4),(char *)CONCAT22(0x1050,local_418));
  } while( true );
}



u16 pass1_1010_db2e(u16 param_1,u16 param_2,u16 param_3,param_4: u32,param_5: u32,param_6: u32,
                    astruct_104 *param_7)

{
  u16 uVar1;
  astruct_493 *iVar2;
  i16 iVar3;
  u16 uVar4;
  astruct_104 *iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uStack94;
  i16 iStack92;
  u16 uStack90;
  u16 *puStack88;
  u8 local_54 [0x52];

  uStack94 = param_3;
  uStack90 = param_3;
  iStack92 = 0x0;
  while( true ) {
    uVar7 = ((u32)param_7 >> 0x10);
    iVar4 = (astruct_104 *)param_7;
    if (param_7->field0_0x0 - 0x1 < uStack94) {
      return uStack94;
    }
    uVar1 = ((int)&iVar4->field1_0x2 + 0x2);
    iVar2 = (astruct_493 *)(&iVar4->field1_0x2 + uStack94 * 0xa);
    uVar5 = (param_5 >> 0x10);
    uVar6 = (param_4 >> 0x10);
    if ((*(i32 *)(iStack92 * 0x4 + (int)param_5) == 0x0) && (*(i32 *)(iStack92 * 0x4 + (int)param_4) == 0x0)) break;
    uVar4 = uVar1;
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_54),*(char **)(iStack92 * 0x4 + (int)param_4));
    uVar6 = (param_6 >> 0x10);
    iVar2->field4_0x4 = (uStack90 * 0x2 + (int)param_6);
    string_1040_a626(uVar4,(u16 *)CONCAT22(uVar1,iVar2),(char *)CONCAT22(0x1050,local_54));
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_54),(char *)0x1050393b,
                  (u32)((int)param_5 + iStack92 * 0x4));
    uVar1 = ((int)&iVar4->field1_0x2 + 0x2);
    iVar3 = &iVar4->field1_0x2 + (uStack94 + 0x1) * 0xa;
    puStack88 = (u16 *)CONCAT22(uVar1,iVar3);
    (iVar3 + 0x4) = ((uStack90 + 0x1) * 0x2 + (int)param_6);
    string_1040_a626(uVar1,puStack88,(char *)CONCAT22(0x1050,local_54));
    uStack94 += 0x2;
    uStack90 += 0x2;
    iStack92 += 0x1;
  }
  return uStack94;
}



void pass1_1010_dc36(u16 param_1,u16 param_2,u16 param_3,param_4: u32,astruct_104 *param_5)

{
  u32 *puVar1;
  u16 uVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u32 *puVar6;
  u16 uVar7;
  u16 uStack90;
  char *string_54;
  u32 local_52 [0x14];

  string_54 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
  puVar6 = local_52;
  for (iVar4 = 0x13; iVar4 != 0x0; iVar4 += -0x1) {
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar1 = 0x0;
  }
  puVar6 = 0x0;
  *(u8 *)(u16 *)((int)puVar6 + 0x2) = 0x0;
  uStack90 = param_3;
  while( true ) {
    uVar7 = ((u32)param_5 >> 0x10);
    if (param_5->field0_0x0 < uStack90 || param_5->field0_0x0 == uStack90) break;
    uVar3 = (u32)((int)param_5 + 0x2);
    uVar2 = ((int)param_5 + 0x4);
    uVar5 = (int)uVar3 + uStack90 * 0xa;
    (uVar5 + 0x4) = (uStack90 * 0x2 + (int)param_4);
    uStack90 += 0x1;
    string_1040_a626(uVar2,(u16 *)(uVar3 & 0xffff0000 | (u32)uVar5),(char *)CONCAT22(0x1050,&string_54));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 string_1010_dcac(u16 param_1,u16 param_2,i16 param_3,param_4: u32,astruct_104 *param_5)

{
  u32 uVar1;
  astruct_105 *iVar2;
  u16 uVar3;
  u16 uVar4;
  astruct_104 *struct_104_5;
  astruct_104 *param_5_seg;
  u16 uVar7;
  char *string_4;
  u16 uVar2;

  string_4 = load_string_1010_847e(_u16_1050_14cc,0x590);
  param_5_seg = (astruct_104 *)((u32)param_5 >> 0x10);
  struct_104_5 = (astruct_104 *)param_5;
  uVar2 = ((int)&struct_104_5->field1_0x2 + 0x2);
  iVar2 = (astruct_105 *)(&struct_104_5->field1_0x2 + param_3 * 0xa);
  uVar7 = (param_4 >> 0x10);
  iVar2->field4_0x4 = (param_3 * 0x2 + (int)param_4);
  string_1040_a626(uVar2,(u16 *)CONCAT22(uVar2,iVar2),string_4);
  unk_str_op_1000_3d3e(string_4,s__1050_3941);
  uVar3 = param_3 + 0x1;
  uVar1 = struct_104_5->field1_0x2;
  uVar4 = (int)uVar1 + uVar3 * 0xa;
  (uVar4 + 0x4) = (uVar3 * 0x2 + (int)param_4);
  string_1040_a626(uVar3,(u16 *)(uVar1 & 0xffff0000 | (u32)uVar4),string_4);
  return uVar3;
}



void struct_1010_dd5e(u16 param_1,u16 param_2,u32 param_3)

{
  i16 iVar1;
  i16 iVar2;
  u16 in_AX;
  u16 in_DX;
  u16 uVar3;
  u32 uVar4;
  i32 *plStack16;

  if (param_3 != 0x0) {
    uVar4 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,in_DX);
    uVar3 = (uVar4 >> 0x10);
    iVar2 = (int)uVar4;
    plStack16 = (i32 *)(uVar4 & 0xffff0000 | (u32)(iVar2 + 0x14U));
    if ((uVar3 | iVar2 + 0x14U) != 0x0) {
      iVar1 = (iVar2 + 0x12);
      iVar2 = (iVar2 + 0x18);
      if (((((iVar1 == 0x4) ||
            ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) || ((iVar1 == 0x6 && (iVar2 == 0x5)))))) ||
           (iVar1 == 0x8)) || ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0x0)) {
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void load_str_1010_ddf6(param_1: u32,u32 param_2)

{
  u16 in_AX;
  u16 in_DX;
  short in_buf_len_5;
  u32 uVar1;

  in_buf_len_5 = (short)(param_1 >> 0x10);
  *(u8 *)((int)param_1 + 0x13c) = 0x0;
  uVar1 = struct_op_1030_73a8((astruct_419 *)param_2,in_AX,in_DX);
  switch(((int)uVar1 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
  case 0x7:
  case 0x9:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x8:
    break;
  default:
    goto switchD_1010_de53_caseD_9;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)((int)param_1 + 0x13c),
             in_buf_len_5);
switchD_1010_de53_caseD_9:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_de78(param_1: u32,u32 param_2)

{
  short in_buf_len_5;

  in_buf_len_5 = (short)(param_1 >> 0x10);
  *(u8 *)((int)param_1 + 0x13c) = 0x0;
  pass1_1030_809c(param_2);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)((int)param_1 + 0x13c),
             in_buf_len_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_debe(param_1: u32,u16 param_2,u16 *param_3,u32 *param_4,u32 param_5)

{
  u8 bVar1;
  u16 uVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u32 in_EDX;
  Struct57*paVar6;
  i16 iVar7;
  u16 unaff_SI;
  u16 uVar8;
  astruct_15 *paVar9;
  u32 *puVar10;
  u16 in_stack_0000fe84;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  u16 uVar11;
  i16 iStack34;
  u16 uStack30;
  i16 iStack26;
  i16 iStack24;
  i16 iStack22;
  i16 iStack20;

  *param_4 = 0x0;
  *param_3 = 0x0;
  paVar9 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)param_5,0x0,(int)in_EDX);
  paVar6 = (Struct57*)(in_EDX & 0xffff0000 | (u32)paVar9 >> 0x10);
  iVar4 = ((int)paVar9 + 0x12);
  uVar5 = param_1;
  uVar11 = (param_1 >> 0x10);
  uVar2 = pass1_1010_b028(uVar5,uVar11,paVar9);
  puVar10 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)puVar10 >> 0x10);
  iVar7 = (int)param_4;
  uVar8 = ((u32)param_4 >> 0x10);
  if (param_2 == 0x13) {
    iStack34 = 0x0;
    while (iStack34 += 0x1, iStack34 < 0x43) {
      param_2 = pass1_1010_ac62(param_2,paVar6,uVar5,uVar11,iStack34);
      if (param_2 != 0x0) {
        *param_3 = *param_3 + 0x1;
      }
    }
    iVar4 = *param_3 * 0x2;
    mem_op_1000_179c(iVar4,paVar6);
    param_4 = iVar4;
    (iVar7 + 0x2) = paVar6;
    if ((paVar6 | param_4) != 0x0) {
      iStack34 = 0x0;
      for (uStack30 = 0x0; uVar2 = uStack30, *param_3 != uStack30 && (int)uStack30 <= (int)*param_3; uStack30 += 0x1) {
        do {
          iStack34 += 0x1;
          if (0x42 < iStack34) goto LAB_1010_e0d4;
          uVar2 = pass1_1010_ac62(uVar2,paVar6,uVar5,uVar11,iStack34);
        } while (uVar2 == 0x0);
        (uStack30 * 0x2 + (int)*param_4) = iStack34;
LAB_1010_e0d4:
      }
    }
  }
  else if (param_2 < 0x14) {
    if ((char)param_2 == '\x06') {
      if (((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8)) {
        uVar3 = (int)puVar10 + 0x11e;
        if (uVar2 == 0xf) {
          iStack20 = 0xf;
          iStack22 = 0x13;
        }
        else if (uVar2 == 0xe) {
          iStack22 = 0x4;
          iStack20 = 0x1;
        }
        else {
          iStack22 = 0xe;
          iStack20 = 0x1;
        }
        iVar4 = pass1_1010_e128(uVar5,uVar11,iStack22,iStack20,(u32)puVar10 & 0xffff0000 | (u32)uVar3);
        *param_3 = iVar4 + 0x1U;
        if (iVar4 + 0x1U != 0x0) {
          iVar4 = *param_3 * 0x2;
          mem_op_1000_179c(iVar4,paVar6);
          param_4 = iVar4;
          (iVar7 + 0x2) = (int)paVar6;
          iStack24 = 0x0;
          for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 += 0x1) {
            if ((iStack26 * 0x2 + uVar3) != 0x0) {
              ((int)*param_4 + iStack24 * 0x2) = iStack26;
              iStack24 += 0x1;
            }
          }
          ((int)*param_4 + iStack24 * 0x2) = 0x14;
          return;
        }
      }
    }
    else {
      bVar1 = (char)param_2 - 0x7;
      if ((bVar1 == 0x0) && (((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8)))) {
        uVar2 = pass1_1010_ac62(param_2 & 0xff00 | bVar1,((u32)puVar10 >> 0x10),uVar5,uVar11,0x7);
        uVar5 = -(uVar2 == 0x0) + 0x10;
        *param_3 = uVar5;
        iVar4 = uVar5 * 0x2;
        mem_op_1000_179c(iVar4,paVar6);
        param_4 = iVar4;
        (iVar7 + 0x2) = paVar6;
        if ((paVar6 | param_4) == 0x0) {
          *param_3 = 0x0;
          return;
        }
        for (iStack26 = 0x0; iStack26 < (int)(-(uVar2 == 0x0) + 0xf); iStack26 += 0x1) {
          (iStack26 * 0x2 + (int)*param_4) = iStack26 + 0x1;
        }
        (iStack26 * 0x2 + (int)*param_4) = 0x10;
        return;
      }
    }
  }
  return;
}



i16 pass1_1010_e128(u16 param_1,u16 param_2,i16 param_3,i16 param_4,u32 param_5)

{
  i16 iStack6;
  i16 iStack4;

  iStack4 = 0x0;
  for (iStack6 = param_4; iStack6 <= param_3; iStack6 += 0x1) {
    if ((iStack6 * 0x2 + (int)param_5) != 0x0) {
      iStack4 += 0x1;
    }
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_e15e(u16 param_1,u16 param_2,u32 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;
  u16 DX_REG;
  u16 DX_REG_00;
  u16 uVar5;
  u16 unaff_CS;
  u16 uVar6;
  u16 uVar7;
  u32 uStack18;
  u32 uStack14;
  u32 *puStack10;

  pass1_1010_afde(param_3,0xc);
  puStack10 = (u32 *)CONCAT22(param_2,param_1);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar2 = param_1;
  uVar6 = param_1;
  uVar7 = param_2;
  (**ppcVar1)();
  uStack14 = CONCAT22(DX_REG,uVar2);
  for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
    ppcVar1 = (code **)((int)*puStack10 + 0x4);
    uVar4 = uStack14;
    (**ppcVar1)(unaff_CS,param_1,param_2,(char)uStack18,(int)(uStack18 >> 0x10));
    uVar3 = uVar4;
    uVar5 = DX_REG_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)DX_REG_00 << 0x10);
    unaff_CS = 0x1030;
    pass1_1030_7c28(uVar3,uVar5,CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,uVar3)),0x23);
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(unaff_CS,param_1,(char)param_2,0x1,uVar6,uVar7);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_e1f4(u16 param_1,param_2: u32,u32 param_3)

{
  u16 uVar1;
  u16 in_AX;
  BOOL16 BVar2;
  char *pcVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  short in_buf_len_5;
  u32 uVar7;

  in_buf_len_5 = (short)(param_2 >> 0x10);
  iVar6 = (int)param_2;
  *(u8 *)(iVar6 + 0x13c) = 0x0;
  uVar7 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,param_1);
  uVar5 = (uVar7 >> 0x10);
  uVar1 = ((int)uVar7 + 0xc);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xc);
  if ((((((((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x14), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xa), BVar2 == 0x0)) &&
         ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x15), BVar2 == 0x0 &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xb), BVar2 == 0x0)))) &&
        (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x16), BVar2 == 0x0)) &&
       (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x17), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x21), BVar2 == 0x0)) &&
        ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1c), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1d), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x18), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x19), BVar2 == 0x0)))))))) &&
      ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x4), BVar2 == 0x0 &&
       (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x3), BVar2 == 0x0)))) &&
     (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1e), BVar2 == 0x0 &&
       (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x23), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1b), BVar2 == 0x0)) &&
        ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1f), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x2), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x13), BVar2 == 0x0)))))))) &&
      (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x20), BVar2 == 0x0 &&
        (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xe), BVar2 == 0x0)) &&
       (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x10), BVar2 == 0x0)))))) {
    pcVar3 = (char *)pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x12);
    if ((pcVar3 == NULL) && (pcVar3 = (char *)pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x11), pcVar3 == NULL)) {
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x6);
      if (BVar2 == 0x0) {
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x5);
        if (BVar2 == 0x0) {
          pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x40);
          goto LAB_1010_e241;
        }
        uVar4 = pass1_1030_7f98(param_3);
        pcVar3 = string_op_1020_c222(uVar4);
      }
      else {
        uVar4 = pass1_1030_7f5a(param_3);
        pcVar3 = string_op_1020_c2f8(uVar4);
      }
    }
    else {
      pass1_1010_e58a(uVar5,param_2,uVar7);
    }
    unk_str_op_1000_3d3e((char *)(param_2 & 0xffff0000 | (u32)(iVar6 + 0x13c)),(char *)CONCAT22(uVar5,pcVar3));
  }
  else {
LAB_1010_e241:
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)(iVar6 + 0x13c),in_buf_len_5
              );
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_e58a(u16 param_1,param_2: u32,u32 param_3)

{
  code **ppcVar1;
  BOOL16 BVar2;
  u32 *puVar3;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  i16 iVar7;
  short in_buf_len_5;
  u16 uVar8;
  u32 *puVar9;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffee;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  in_buf_len_5 = (short)(param_2 >> 0x10);
  iVar7 = (int)param_2;
  *(u8 *)(iVar7 + 0x13c) = 0x0;
  uVar8 = (param_3 >> 0x10);
  puVar3 = (u32 *)((int)param_3 + 0x20);
  uVar8 = ((int)param_3 + 0xc);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar8,0x11);
  if (BVar2 == 0x0) {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar8,0x12);
    if (BVar2 == 0x0) {
      return;
    }
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x31),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    uVar4 = ((u32)puVar9 >> 0x10);
    ppcVar1 = (code **)((int)*puVar9 + 0x14);
    (**ppcVar1)(0x1008,(int)puVar9,uVar4,puVar3,(int)puVar3 >> 0xf);
    uVar5 = uVar4 | puVar3;
  }
  else {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x41),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    uVar4 = ((u32)puVar9 >> 0x10);
    ppcVar1 = (code **)((int)*puVar9 + 0x14);
    (**ppcVar1)(0x1008,(int)puVar9,uVar4,puVar3,(int)puVar3 >> 0xf);
    uVar5 = uVar4 | puVar3;
  }
  if (uVar5 == 0x0) {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)(iVar7 + 0x13c),in_buf_len_5
              );
  }
  else {
    ppcVar1 = (code **)((int)*puVar3 + 0x14);
    (**ppcVar1)(0x1008,(char)puVar3,uVar4);
    unk_str_op_1000_3d3e((char *)(param_2 & 0xffff0000 | (u32)(iVar7 + 0x13c)),(char *)CONCAT22(uVar5,puVar3));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_e682(u16 param_1,u16 param_2,param_3: u32,u32 param_4)

{
  u16 uVar1;
  BOOL16 BVar2;
  u16 uVar3;
  u32 uVar4;
  u16 uVar5;
  u16 in_buf_len_5;
  u32 uVar6;
  u16 uVar7;
  u16 local_1e;
  u16 uStack28;
  u16 local_1a;
  u16 uStack24;
  u16 uStack22;
  u32 uStack20;
  u32 uStack16;
  u32 uStack12;
  u16 uStack8;
  u32 uStack6;

  in_buf_len_5 = (param_3 >> 0x10);
  uVar5 = param_3;
  *(u8 *)(uVar5 + 0x13c) = 0x0;
  uStack6 = struct_op_1030_73a8((astruct_419 *)param_4,param_1,param_2);
  uVar3 = (uStack6 >> 0x10);
  uStack8 = ((int)uStack6 + 0xc);
  uVar1 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x1);
  if (((uVar1 == 0x0) && (uVar1 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x13), uVar1 == 0x0)) &&
     (uVar1 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x2), uVar1 == 0x0)) {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0xe);
    if (BVar2 != 0x0) {
      uVar6 = (u32)(uVar5 + 0x138);
      uVar4 = (u32)((int)uVar6 + 0x24);
      uStack16 = uVar4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4);
      uStack12 = uVar4 & 0xffff | (u32)uVar3 << 0x10;
      uStack20 = (u32)((int)uVar4 + 0x1f6);
      uVar3 = ((int)uStack20 + 0x1a8);
      uVar7 = 0x3947;
      uStack22 = uVar3;
LAB_1010_e76d:
      sys_1000_3f9c((char *)(param_3 & 0xffff0000 | (u32)(uVar5 + 0x13c)),(char *)CONCAT22(0x1050,uVar7),uVar3);
      return;
    }
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x5);
    if ((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x6), BVar2 == 0x0)) {
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x10);
      if (BVar2 == 0x0) {
        local_1e = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0xc);
        if ((local_1e == 0x0) && (local_1e = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x14), local_1e == 0x0)) {
          BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0xa);
          if (BVar2 == 0x0) {
            uVar3 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x1e);
            if (uVar3 == 0x0) {
              load_string_1010_84e0
                        (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)(uVar5 + 0x13c),
                         in_buf_len_5);
              return;
            }
            pass1_1030_6ddc(param_4);
            uVar7 = 0x395c;
            local_1e = uVar3;
            goto LAB_1010_e76d;
          }
          uVar6 = pass1_1030_7c28(BVar2,uVar3,param_4,0x21);
          uStack28 = ((u32)uVar6 >> 0x10);
          uVar1 = uVar6;
          uVar7 = 0x3958;
          local_1e = uVar1;
        }
        else {
          pass1_1010_e8f6(local_1e,uVar3,uVar5,in_buf_len_5,param_4);
          uStack28 = uVar3;
          uVar6 = pass1_1030_7c28(local_1e,uVar3,CONCAT22(uVar3,local_1e),0x23);
          uStack24 = ((u32)uVar6 >> 0x10);
          uVar1 = uVar6;
          uVar7 = 0x3954;
          local_1a = uVar1;
        }
      }
      else {
        uVar6 = pass1_1030_7c28(BVar2,uVar3,param_4,0x1e);
        uStack28 = ((u32)uVar6 >> 0x10);
        uVar1 = uVar6;
        uVar7 = 0x3950;
        local_1e = uVar1;
      }
    }
    else {
      local_1e = 0x0;
      local_1a = 0x0;
      pass1_1010_e8d0(&local_1e,uVar5,in_buf_len_5,(u16 *)CONCAT22(0x1050,&local_1a),
                      (u16 *)CONCAT22(0x1050,&local_1e),param_4);
      uVar7 = 0x394a;
      uVar1 = local_1e;
    }
  }
  else {
    pass1_1010_e8f6(uVar1,uVar3,uVar5,in_buf_len_5,param_4);
    uStack12 = CONCAT22(uVar3,uVar1);
    pass1_1030_70f4(CONCAT22(uVar3,uVar1));
    uStack16 = CONCAT22(uVar3,uVar1);
    uVar7 = 0x3943;
  }
  sys_1000_3f9c((char *)(param_3 & 0xffff0000 | (u32)(uVar5 + 0x13c)),(char *)CONCAT22(0x1050,uVar7),uVar1);
  return;
}



void pass1_1010_e8d0(u16 param_1,u16 param_2,u16 param_3,u16 *param_4,u16 *param_5,u32 param_6)

{
  pass1_1030_7064(param_6);
  *param_5 = param_1;
  pass1_1030_70ac(param_6);
  *param_4 = param_1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_e8f6(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  u16 uVar2;
  BOOL16 BVar3;
  u16 uVar6;
  u32 uVar5;
  astruct_15 *paVar4;
  u32 uVar7;
  u16 uVar1;

  uVar5 = struct_op_1030_73a8((astruct_419 *)param_5,param_1,param_2);
  uVar1 = ((int)uVar5 + 0xc);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x13);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x14);
    if (BVar3 == 0x0) {
      return;
    }
    uVar7 = pass1_1028_4faa((astruct_15 *)uVar5,0x1050);
    uVar6 = (uVar7 >> 0x10);
    uVar2 = uVar7;
  }
  else {
    paVar4 = pass1_1028_121e(0x1050,(astruct_15 *)uVar5);
    uVar6 = ((u32)paVar4 >> 0x10);
    uVar2 = SUB42(paVar4,0x0);
  }
  pass1_1028_b58e((astruct_15 *)CONCAT22(uVar6,uVar2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_e964(u16 param_1)

{
  u32 uVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 *puVar3;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffea;

  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffea,0x2f),in_stack_0000fe92,in_stack_0000ffb6,
                           in_stack_0000ffbc,in_stack_0000ffc0);
  uVar2 = ((u32)puVar3 >> 0x10);
  uVar1 = (u32)((int)puVar3 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  pass1_1038_4d28((char *)(uVar1 & 0xffff | (u32)uVar2 << 0x10));
  return;
}



StructD * pass1_1010_e99a(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xa));
  pass1_1010_a478(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1010_e9a6(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1010_a478(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



void struct_1010_e9e4(astruct_19 *param_1,u16 param_2)

{
  u16 *puVar1;
  u16 uVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  u32 uVar6;
  u32 in_EDX;
  u16 uVar8;
  Struct57*paVar7;
  i16 iVar9;
  astruct_19 *paVar10;
  u16 *puVar11;
  i16 iStack4;

  uVar8 = ((u32)in_EDX >> 0x10);
  paVar10 = struct_op_1010_1d48(param_1,param_2);
  paVar7 = (Struct57*)CONCAT22(uVar8,(int)((u32)paVar10 >> 0x10));
  ((int)param_1 + 0xa) = 0x389a;
  ((int)param_1 + 0xc) = 0x1008;
  ((int)param_1 + 0xa) = 0x3aa8;
  ((int)param_1 + 0xc) = 0x1008;
  uVar5 = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  (u32)((int)param_1 + 0x16) = 0x0;
  (u32)((int)param_1 + 0x1a) = 0x0;
  ((int)param_1 + 0x1e) = 0x0;
  (u32)((int)param_1 + 0x20) = 0x0;
  (u32)((int)param_1 + 0x24) = 0x0;
  (u32)((int)param_1 + 0x28) = 0x0;
  (u32)((int)param_1 + 0x2c) = 0x0;
  ((int)param_1 + 0x30) = 0x0;
  ((int)param_1 + 0x32) = 0x0;
  param_1->offset_0x0 = 0x558;
  ((int)param_1 + 0x2) = 0x1018;
  ((int)param_1 + 0xa) = 0x568;
  ((int)param_1 + 0xc) = 0x1018;
  mem_op_1000_179c(0x4,paVar7);
  if ((paVar7 | uVar5) == 0x0) {
    (u32)((int)param_1 + 0xe) = 0x0;
  }
  else {
    puVar11 = pass1_1018_dcf6((u16 *)CONCAT22(paVar7,uVar5));
    ((int)param_1 + 0xe) = (int)puVar11;
    ((int)param_1 + 0x10) = (int)((u32)puVar11 >> 0x10);
  }
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x34)),NULL,0x24);
  (u32)((int)param_1 + 0x38) = 0xfa;
  (u32)((int)param_1 + 0x3c) = 0x15e;
  uVar6 = 0x1c2;
  (u32)((int)param_1 + 0x40) = 0x1c2;
  ((int)param_1 + 0x44) = 0x1c2;
  (u32)((int)param_1 + 0x46) = 0x2260000;
  (u32)((int)param_1 + 0x4a) = 0x28a0000;
  (u32)((int)param_1 + 0x4e) = 0x730000;
  (u32)((int)param_1 + 0x52) = 0x960000;
  ((int)param_1 + 0x56) = 0x0;
  for (iStack4 = 0x1; iStack4 < 0x9; iStack4 += 0x1) {
    pass1_1008_612e(uVar6,0x0,0x1d);
    uVar5 = uVar6;
    pass1_1008_612e(uVar5,0x1,0x2);
    if ((uVar6 & 0x1) != 0x0) {
      uVar5 = -uVar5;
    }
    iVar9 = iStack4 * 0x4;
    puVar1 = (u16 *)((int)param_1 + 0x34 + iVar9);
    uVar2 = *puVar1;
    uVar4 = uVar5 + *puVar1;
    uVar6 = (u32)uVar4;
    iVar3 = ((int)param_1 + 0x34 + iVar9 + 0x2);
    ((int)param_1 + iVar9 + 0x34) = uVar4;
    ((int)param_1 + iVar9 + 0x36) = ((int)uVar5 >> 0xf) + iVar3 + CARRY2(uVar5,uVar2);
  }
  return;
}



void pass1_1010_eb66(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 *puVar4;
  StructD *iVar5;
  u16 uVar5;
  u16 *puStack14;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x558;
  iVar5->address_offset_field_0x2 = 0x1018;
  iVar5->field6_0xa = 0x568;
  iVar5->field7_0xc = (astruct_589 *)0x1018;
  puVar1 = (u32 *)iVar5->field8_0xe;
  uVar2 = &iVar5->field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1018_04f2(param_1);
  fn_ptr_1000_17ce(iVar5->field29_0x2c);
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar5->field6_0xa;
  }
  puStack14 = (u16 *)CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



void pass1_1010_ebf8(param_1: u32,u16 param_2,u16 param_3,param_4: i16)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  ((int)param_1 + param_4 * 0x4 + 0x34) = param_2;
  ((int)param_1 + param_4 * 0x4 + 0x36) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_ec18(i16 param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  return CONCAT22((param_1 + 0x12),(param_1 + 0x10));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1010_ec40(i16 param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  return CONCAT22((param_1 + 0x12),(param_1 + 0x10));
}



void pass1_1010_ec68(param_1: u32,u32 param_2)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  (u32)((int)param_1 + 0x28) = param_2;
  pass1_1010_1f62((Struct27 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),0x13);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_ec84(u32 param_1)

{
  u8 local_10e [0x10c];

  pass1_1010_1f62((Struct27 *)param_1,0x14);
  pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,local_10e),(u32)((int)param_1 + 0x20));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_10e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_ecc6(u16 param_1,u16 param_2,param_3: u32,u16 *param_4,i32 param_5)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_4,param_5);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
  uVar1 = (u32)(param_1 + 0x2e);
  uVar3 = ((u32)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  if (*(i32 *)(iVar2 + 0x200) == 0x8000001) {
    pass1_1010_ed22(param_3,(u32)(iVar2 + 0x4));
  }
  return;
}



void pass1_1010_ed22(param_1: u32,u32 param_2)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  (u32)((int)param_1 + 0x24) = param_2;
  pass1_1010_1f62((Struct27 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),0x12);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1010_ed3e(u32 param_1)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_1 + 0x16));
  return;
}



void write_to_file_1010_ed58(param_1: u32,u32 param_2)

{
  i16 *piVar1;
  u16 uVar2;
  BOOL16 BVar3;
  i16 iVar4;
  u32 *puVar5;
  i16 iVar6;
  u16 uVar7;
  HFILE16 in_stack_0000ffc4;
  u32 local_22;
  u16 uStack30;
  u32 local_12 [0x2];
  u32 local_a;
  i16 iStack4;

  BVar3 = write_to_file_1008_7cac(param_2);
  if (BVar3 != 0x0) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = (int)param_1;
    local_12[0] = (u32)(iVar6 + 0x16);
    BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_12),(char *)0x4,in_stack_0000ffc4);
    if (BVar3 != 0x0) {
      local_a = (u32)(iVar6 + 0x1a);
      BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_a),(char *)0x4,in_stack_0000ffc4);
      if (BVar3 != 0x0) {
        local_a = (u32)(iVar6 + 0x20);
        BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_a),(char *)0x4,in_stack_0000ffc4);
        if (BVar3 != 0x0) {
          local_a = (u32)(iVar6 + 0x24);
          BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_a),(char *)0x4,in_stack_0000ffc4);
          if (BVar3 != 0x0) {
            local_a = local_a & 0xffff0000 | (u32)(iVar6 + 0x30);
            BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_a),(char *)0x2,in_stack_0000ffc4);
            if (BVar3 != 0x0) {
              local_a = local_a & 0xffff0000 | (u32)(iVar6 + 0x32);
              BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_a),(char *)0x2,in_stack_0000ffc4);
              if (BVar3 != 0x0) {
                iStack4 = 0x0;
                while( true ) {
                  piVar1 = (i16 *)(iVar6 + 0x30);
                  if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                    return;
                  }
                  uVar2 = (iVar6 + 0x2e);
                  puVar5 = (u32 *)((iVar6 + 0x2c) + iStack4 * 0x6);
                  local_22 = *puVar5;
                  uStack30 = (puVar5 + 0x1);
                  local_12[0] = local_12[0] & 0xffff0000 | ZEXT24(&local_22);
                  iVar4 = write_to_file_1008_7b4c(param_2,(astruct_615 *)CONCAT22(0x1050,&local_22));
                  if (iVar4 == 0x0) break;
                  iStack4 += 0x1;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
