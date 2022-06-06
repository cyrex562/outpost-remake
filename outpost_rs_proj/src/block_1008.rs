//
// Created by cyrex on 2022-05-22.
//

#include "types.h"
#include "structs_2.h"
#include "block_1008.h"
#include "func_ptrs.h"
pub fn struct_op_1008_0000(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;


  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x52a;
  (iVar1 + 0x2) = 0x1008;
  (u32)(iVar1 + 0x4) = 0x0;
  (u32)(iVar1 + 0x8) = 0x0;
  *param_1 = 0x51e;
  (iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_0036(param_1: *mut u16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  char *pcVar3;
  code **ppcVar4;
  u32 *puVar5;
  let mut puVar6: *mut u16;
  astruct_449 *iVar6;
  let mut uVar7: u16;
  let mut unaff_CS: u16;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar6 = (astruct_449 *)param_1;
  *param_1 = 0x51e;
  iVar6.field2_0x2 = 0x1008;
  pcVar3 = *(char **)&iVar6.field_0x8;
  if ((iVar6.field7_0xa | pcVar3) != 0x0) {
    pass1_1008_53aa();
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar3);
  }
  puVar6 = _u16_1050_5748;
  _PTR_LOOP_1050_0298 = 0x0;
  if (_u16_1050_5748 != NULL) {
    pass1_1030_8210(_u16_1050_5748);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce((char *)puVar6);
  }
  pcVar3 = _u16_1050_0ed0;
  if (_u16_1050_0ed0 != NULL) {
    pass1_1010_2050((u32)_u16_1050_0ed0);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar3);
  }
  puVar5 = _u16_1050_14cc;
  if (_u16_1050_14cc != NULL) {
    pass1_1010_7efc(_u16_1050_14cc);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce((char *)puVar5);
  }
  pcVar3 = _PTR_LOOP_1050_5b7c;
  if (_PTR_LOOP_1050_5b7c != NULL) {
    pass1_1038_af34();
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar3);
  }
  if (_u16_1050_5bc8 != NULL) {
    ppcVar4 = (code **)*_u16_1050_5bc8;
    (**ppcVar4)(unaff_CS,(int)_u16_1050_5bc8,(int)((u32)_u16_1050_5bc8 >> 0x10),0x1);
  }
  if (_u16_1050_02a0 != NULL) {
    ppcVar4 = (code **)*_u16_1050_02a0;
    (**ppcVar4)(unaff_CS,(int)_u16_1050_02a0,(int)((u32)_u16_1050_02a0 >> 0x10),0x1);
  }
  puVar1 = iVar6.field3_0x4;
  uVar2 = iVar6.field4_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)(unaff_CS,puVar1,uVar2,0x1);
  }
  pass1_1008_9466(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_win_sys_op_1008_016e(param_1: *mut astruct_823)

{
  let mut puVar1: *mut u16;
  let mut uVar6: u16;
  let mut iVar3: i16;
  let mut uVar5: u16;
  let mut uVar9: u16;
  let mut uVar11: u16;
  let mut uVar8: u32;
  let mut DVar10: u16;
  u8 *puVar4;
  let mut puVar14: u16;
  let mut uVar13: u16;
  u8 *puVar12;
  u8 *puVar13;
  let mut uVar7: u16;
  let mut in_EDX: u32;
  astruct_823 *struct_1;
  let mut unaff_DI: i16;
  let mut uVar10: u16;
  let mut uVar12: u16;
  let mut DVar16:u32;
  u32 *puVar17;
  StructD *pSVar18;
  let mut in_stack_0000fe46: u16;
  u8 local_13e [0xac];
  u8 local_92 [0x80];
  let mut uStack18: u16;
  let mut uStack16: u16;
  u32 *puStack14;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;
  let mut uVar1: u32;
  let mut puVar2: *mut u16;
  astruct_20 *uVar4;
  astruct_20 *uVar2;
  astruct_20 *uVar3;
  astruct_57 *paVar14;
  astruct_57 *paVar15;
  code **fn_ptr;

  uVar9 = ((u32)in_EDX >> 0x10);
  DVar16 = GetVersion16();
  DVar10 = (DVar16 >> 0x10);
  paVar14 = (astruct_57 *)CONCAT22(uVar9,DVar10);
  uStack6 = (DVar16 & 0xffff);
  uVar6 = DVar16 & 0xff;
  uStack10 = (u8)((DVar16 & 0xffff) >> 0x8);
  uStack8 = uVar6;
  if ((uVar6 < 0x3) || ((uVar6 == 0x3 && (uStack10 < 0xa)))) {
    // 0x97
    uVar12 = 0x1000;
    mem_op_1000_179c(0xb4,paVar14);
    uStack16 = paVar14;
    puVar4 = (u8 *)(uStack16 | uVar6);
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000);
    paVar15 = (astruct_57 *)((u32)paVar14 | ZEXT24(puVar4));
    uStack18 = uVar6;
    if (puVar4 == NULL) {
      iVar3 = 0x0;
    }
    else {
      uVar12 = &PTR_LOOP_1050_1040;
      iVar3 = string_1040_8520((u32)paVar15,(astruct_57 *)CONCAT22(uStack16,uVar6),0x0,0x20010,0x5dd05de);
      paVar14 = paVar15;
    }
    puStack14 = (u32 *)CONCAT22((int)paVar14,iVar3);
    fn_ptr = (code **)((int)*puStack14 + 0x74);
    (**fn_ptr)(uVar12,(char)iVar3,(char)paVar14);
    fn_ptr_op_1000_24cd(0x1);
  }
  debug_print_1008_6048(paVar14,s_version__d__d_1050_0012);
  if ((uStack8 == 0x3) && (0xb < (int)uStack10)) {
    PTR_LOOP_1050_0010 = (StructD *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  LoadString16(0x80,(char *)CONCAT22(0x1050,local_92),0x578,HINSTANCE16_1050_038c);
  uVar5 = dos3_call_1000_51aa(local_92,&DAT_1050_1050,0x1);
  if (uVar5 != 0x0) {
    LoadString16(0x80,(char *)CONCAT13(0x10,CONCAT12(0x50,local_13e)),0x57b,HINSTANCE16_1050_038c);
    LoadString16(0x80,(char *)CONCAT13(0x10,CONCAT12(0x50,&stack0xfe42)),0x62e,HINSTANCE16_1050_038c);
    uVar5 = MessageBox16(0x10,(char *)CONCAT13(0x10,CONCAT12(0x50,local_13e)),(char *)CONCAT22(0x1050,&stack0xfe42),0x0)
    ;
    fn_ptr_op_1000_24cd(0x1);
  }
  mem_op_1000_179c(0x4,paVar14);
  uStack16 = paVar14;
  paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000);
  if ((uStack16 | uVar5) == 0x0) {
    uVar9 = 0x0;
    uStack18 = uVar5;
  }
  else {
    uStack18 = uVar5;
    puVar17 = pass1_1008_5394((u32 *)CONCAT22(uStack16,uVar5));
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)puVar17 >> 0x10);
    uVar9 = SUB42(puVar17,0x0);
  }
  uVar10 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_823 *)param_1;
  &struct_1.field5_0x8 = uVar9;
  ((int)&struct_1.field5_0x8 + 0x2) = (int)paVar14;
  uVar8 = (u32)struct_1.field5_0x8;
  puVar1 = struct_1.field5_0x8;
  _PTR_LOOP_1050_0298 = (u16 *)uVar8;
  *puVar1 = 0x70;
    // 0x1538
  ((int)puVar1 + 0x2) = (int)s_tile2_bmp_1050_1538;
  mem_op_1000_179c(0x126,paVar14);
  uVar11 = uVar8;
  uStack16 = paVar14;
  paVar15 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)(uStack16 | uVar11));
  uStack18 = uVar11;
  if ((uStack16 | uVar11) != 0x0) {
    pSVar18 = pass1_1010_2024((StructD *)(uVar8 & 0xffff | (long)paVar14 << 0x10));
    paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)pSVar18 >> 0x10);
    uVar11 = pSVar18;
  }
  if (_u16_1050_0ed0 == 0x0) {
    debug_print_1008_6048(paVar15,s_New_failed_in_Op__Op_1050_0020);
    fn_ptr_op_1000_24cd(0x1);
  }
  mem_op_1000_179c(0xe8c,paVar15);
  uStack16 = paVar15;
  puVar12 = (u8 *)(uStack16 | uVar11);
  paVar14 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | ZEXT24(puVar12));
  uStack18 = uVar11;
  if (puVar12 != NULL) {
    pass1_1010_7e40(puVar12,(astruct_652 *)CONCAT22(uStack16,uVar11));
  }
  if (_u16_1050_14cc == 0x0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__ResLibr_1050_0035);
    fn_ptr_op_1000_24cd(0x1);
  }
  mem_op_1000_179c(0xb0,paVar14);
  uStack16 = paVar14;
  paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)(uStack16 | uVar11));
  uStack18 = uVar11;
  if ((uStack16 | uVar11) != 0x0) {
    pSVar18 = pass1_1038_aeca((StructD *)CONCAT22(uStack16,uVar11));
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)pSVar18 >> 0x10);
    uVar11 = pSVar18;
  }
  if (_PTR_LOOP_1050_5b7c == 0x0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__DialogCtr_1050_0053);
    fn_ptr_op_1000_24cd(0x1);
  }
  mem_op_1000_179c(0xa,paVar14);
  uStack16 = paVar14;
  puVar13 = (u8 *)(uStack16 | uVar11);
  paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | ZEXT24(puVar13));
  uStack18 = uVar11;
  if (puVar13 != NULL) {
    make_proc_inst_1038_cf6c(puVar13,(astruct_831 *)CONCAT22(uStack16,uVar11));
  }
  if (_u16_1050_5bc8 == 0x0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__DialogHand_1050_0073);
    fn_ptr_op_1000_24cd(0x1);
  }
  mem_op_1000_179c(0x14,paVar14);
  uStack16 = paVar14;
  paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)(uStack16 | uVar11));
  uStack18 = uVar11;
  if ((uStack16 | uVar11) != 0x0) {
    pass1_1008_5bdc((char *)CONCAT22(uStack16,uVar11));
  }
  if (_u16_1050_02a0 == 0x0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__Simulator_1050_0097);
    fn_ptr_op_1000_24cd(0x1);
  }
  mem_op_1000_179c(0xfc,paVar14);
  uStack16 = paVar14;
  uVar7 = uStack16 | uVar11;
  uStack18 = uVar11;
  if (uVar7 == 0x0) {
    uVar11 = 0x0;
    uVar7 = 0x0;
  }
  else {
    set_struct_op_1008_0536((astruct_20 *)CONCAT22(uStack16,uVar11),in_stack_0000fe46);
  }
  &struct_1.field4_0x4 = uVar11;
  ((int)&struct_1.field4_0x4 + 0x2) = uVar7;
  if (struct_1.field4_0x4 == NULL) {
    debug_print_1008_6048(uVar7,s_New_failed_in_Op__Op_1050_00b7);
    fn_ptr_op_1000_24cd(0x1);
  }
  win_ui_reg_class_1008_96d2((StructA *)struct_1.field4_0x4);
  fn_ptr = (code **)((int)(u32)struct_1.field4_0x4 + 0x8);
  (**fn_ptr)(0x1000);
  uVar4 = struct_1.field4_0x4;
  HWND16_1050_0396 = *(HWND16 *)((int)uVar4 + 0x8);
  uVar2 = struct_1.field4_0x4;
  fn_ptr = (code **)((int)(u32)struct_1.field4_0x4 + 0xc);
  (**fn_ptr)(0x1000,(char)uVar2,(char)((u32)uVar2 >> 0x10),0x3);
  uVar3 = struct_1.field4_0x4;
  UpdateWindow16(*(HWND16 *)((int)uVar3 + 0x8));
  return;
}
pub fn pass1_1008_049c(mut param_1: u16 ,mut param_2: u16 ,char *param_3)

{
  let mut uVar1: u16;
  u8 *puVar2;

  if (param_3 != NULL) {
    uVar1 = str_op_1000_3da4(param_3);
    if (uVar1 != 0x0) {
      puVar2 = (u8 *)
               pass1_1000_545a((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x1),(u32)s_nomono2_1050_00cc);
      if (puVar2 == NULL) {
        PTR_LOOP_1050_02ec = puVar2;
      }
    }
  }
  return;
}



StructD * pass1_1008_04d2(StructD *param_1,u8 param_2)

{
  pass1_1008_9466(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_04f8(param_1: *mut u16,u8 param_2)

{
  pass1_1008_0036(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack
pub fn set_struct_op_1008_0536(param_1: *mut astruct_20,mut param_2: u16 )

{
  HICON16 hicon_1;
  HCURSOR16 hcursor_1;
  HGDIOBJ16 hbrush_1;
  let mut in_EDX: u32;
  let mut uVar2: u16;
  astruct_57 *paVar1;
  astruct_20 *paVar3;
  u32 *puVar4;
  let mut in_stack_0000feac: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd6: u16;
  let mut in_stack_0000ffda: u16;

  uVar2 = ((u32)in_EDX >> 0x10);
  paVar3 = pass1_1008_3ab8(param_1);
  paVar1 = (astruct_57 *)CONCAT22(uVar2,(int)((u32)paVar3 >> 0x10));
  (u32)((int)param_1 + 0xe0) = 0x0;
  (u32)((int)param_1 + 0xe4) = 0x0;
  (u32)((int)param_1 + 0xe8) = 0x0;
  ((int)param_1 + 0xec) = 0x0;
  (u32)((int)param_1 + 0xee) = 0x0;
  ((int)param_1 + 0xf2) = 0x0;
  (u32)((int)param_1 + 0xf4) = 0x0;
  (u32)((int)param_1 + 0xf8) = 0x0;
  param_1.offset_0x0 = 0x389e;
  ((int)param_1 + 0x2) = 0x1008;
  ((int)param_1 + 0xc8) = 0x2008;
  ((int)param_1 + 0xac) = 0x0;
  ((int)param_1 + 0xae) = 0x8700;
  hicon_1 = LoadIcon16(s_Op_1050_00d4,HINSTANCE16_1050_038c);
  *(HICON16 *)((int)param_1 + 0xc2) = hicon_1;
  hcursor_1 = LoadCursor16((char *)0x7f00,0x0);
  *(HCURSOR16 *)((int)param_1 + 0xc4) = hcursor_1;
  hbrush_1 = GetStockObject16(BLACK_BRUSH);
  *(HGDIOBJ16 *)((int)param_1 + 0xc6) = hbrush_1;
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0 & 0xffff,(u8 **)CONCAT22((int)param_1,0x48),in_stack_0000feac,
                           in_stack_0000ffd0,in_stack_0000ffd6,in_stack_0000ffda);
  paVar1 = (astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar4 >> 0x10);
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0xa)),s_Outpost_1050_00d7);
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0 & 0xffff,(u8 **)CONCAT22((int)param_1,0x32),in_stack_0000feac,
                           in_stack_0000ffd0,in_stack_0000ffd6,in_stack_0000ffda);
  ((int)param_1 + 0xf4) = (int)puVar4;
  ((int)param_1 + 0xf6) = (int)((u32)puVar4 >> 0x10);
  set_sys_color_1008_357e((astruct_53 *)param_1,0x1,(u32)paVar1 & 0xffff0000 | (u32)puVar4 >> 0x10);
  return;
}
pub fn cleanup_ui_op_1008_0618(param_1: *mut astruct_53)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut in_EDX: u32;
  astruct_53 *iVar4;
  let mut uVar3: u16;
  u32 *puVar1;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_53 *)param_1;
  param_1 = 0x389e;
  &iVar4.field_0x2 = 0x1008;
  set_sys_color_1008_357e(param_1,0x0,in_EDX);
  fn_ptr_1000_17ce(*(char **)&iVar4.field248_0xf8);
  if (&iVar4.field_0xec != 0x0) {
    DestroyMenu16(*(HMENU16 *)&iVar4.field_0xec);
  }
  DestroyIcon16(*(HICON16 *)&iVar4.field_0xc2);
  &iVar4.field_0xc2 = 0x0;
  puVar1 = (u32*)&iVar4.field_0xe0;
  uVar1 = &iVar4.field_0xe2;
  if ((uVar1 | puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,puVar1,uVar1,0x1);
  }
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
  param_1 = 0x380a;
  &iVar4.field_0x2 = 0x1008;
  param_1 = 0x389a;
  &iVar4.field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_cursor_op_1008_06c0(u32 *param_1,mut param_2: u32,mut param_3: u16 ,mut param_4: i16)

{
  code **ppcVar1;
  let mut in_AX: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar2;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  u8 in_AF;
  char *pcVar6;
  u32 *puVar7;
  let mut in_stack_0000fe3a: u16;
  let mut in_stack_0000fe44: u16;
  let mut in_stack_0000ff5e: u16;
  let mut in_stack_0000ff64: u16;
  let mut in_stack_0000ff68: u16;
  let mut in_stack_0000ff6e: u16;
  let mut in_stack_0000ff72: u16;
  let mut iVar8: i16;
  let mut in_stack_0000ff9c: u16;
  u8 local_5a [0x50];
  let mut uStack10: u16;
  let mut uStack8: u16;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  if (param_4 == 0x400) {
    pass1_1030_8344((u32)_u16_1050_5748,0x4000001);
    in_AX = in_EDX | in_AX;
    paVar2 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)in_AX);
    if (in_AX != 0x0) {
      iVar4 = (int)param_1;
      uVar5 = ((u32)param_1 >> 0x10);
      if (PTR_LOOP_1050_4fe8 != NULL) {
        pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x57b);
        MessageBox16(0x10,pcVar6,s_You_may_not_run_a_turn__The_game_1050_00df,*(HWND16 *)(iVar4 + 0x8));
        return;
      }
      HStack4 = LoadCursor16((char *)0x7f02,0x0);
      HStack6 = SetCursor16(HStack4);
      pass1_1030_83ba(in_AF,_u16_1050_5748,param_2);
      ((int)_u16_1050_5748 + 0x8) = 0x1;
      puVar7 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ff9c,0x29),in_stack_0000fe44,
                               in_stack_0000ff68,in_stack_0000ff6e,in_stack_0000ff72);
      uVar3 = ((u32)paVar2 >> 0x10);
      uStack10 = SUB42(puVar7,0x0);
      uStack8 = ((u32)puVar7 >> 0x10);
      pass1_1018_262e((u32)puVar7);
      pass1_1030_8326();
      pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x5dc);
      paVar2 = (astruct_57 *)CONCAT22(uVar3,(int)((u32)pcVar6 >> 0x10));
      sys_1000_3f9c((char *)CONCAT13(0x10,CONCAT12(0x50,local_5a)),s__s__ld_1050_0109,pcVar6);
      ppcVar1 = (code **)((int)*param_1 + 0x14);
      iVar8 = iVar4;
      (**ppcVar1)(0x1000,iVar4,(char)((u32)param_1 >> 0x10),0x0,local_5a,(int)&DAT_1050_1050);
      puVar7 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(iVar8,0x37),in_stack_0000fe3a,
                               in_stack_0000ff5e,in_stack_0000ff64,in_stack_0000ff68);
      pass1_1008_a9ec((u32)puVar7);
      SetCursor16(HStack6);
      PostMessage16(0x0,0xfc,0x111,*(HWND16 *)(iVar4 + 0x8));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1008_07d8(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_72)

{
  let mut uVar2: u16;
  let mut uVar1: u16;
  let mut uVar4: u32;
  astruct_57 *paVar3;

  if (_u16_1050_5748 == NULL) {
    mem_op_1000_179c(0xa,param_2);
    uVar2 = param_2 | param_1;
    paVar3 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar2);
    if (uVar2 != 0x0) {
      struct_1030_8128(paVar3,(astruct_135 *)CONCAT22(param_2,param_1));
    }
    if (_u16_1050_5748 == NULL) {
      debug_print_1008_6048(paVar3,s_New_failed_in_Op__Op__Simulator_1050_0110);
      fn_ptr_op_1000_24cd(0x1);
    }
    uVar4 = pass1_1028_e2e0(paVar3,_PTR_LOOP_1050_65e2,0x8);
    paVar3 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | uVar4 >> 0x10);
    uVar4 = pass1_1028_e2e0(paVar3,_PTR_LOOP_1050_65e2,0x8);
    pass1_1028_e2e0((astruct_57 *)((u32)paVar3 & 0xffff0000 | uVar4 >> 0x10),_PTR_LOOP_1050_65e2,0xff);
    pass1_1030_838e(_u16_1050_5748);
    param_1 = pass1_1030_8334();
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_087e(uchar param_1,mut param_2: u16 ,StructD *param_3)

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  let mut uVar2: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_3);
  mem_op_1000_179c(0xa,paVar1);
  uStack4 = paVar1;
  paVar1 = (astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)(uStack4 | param_2));
  uStack6 = param_2;
  if ((uStack4 | param_2) != 0x0) {
    struct_1030_8128(paVar1,(astruct_135 *)CONCAT22(uStack4,param_2));
  }
  if (_u16_1050_5748 == NULL) {
    debug_print_1008_6048(paVar1,s_New_failed_in_Op__Op__Simulator_1050_0130);
    fn_ptr_op_1000_24cd(0x1);
  }
  uVar2 = pass1_1028_e2e0(paVar1,_PTR_LOOP_1050_65e2,0x8);
  pass1_1028_e2e0((astruct_57 *)((u32)paVar1 & 0xffff0000 | uVar2 >> 0x10),_PTR_LOOP_1050_65e2,0x8);
  pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,&local_112),0xff000000);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_112));
  pass1_1030_838e((u32 *)_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_0932(void) -> u32

{
  let mut uVar1: u32;

  if (_u16_1050_14cc != NULL) {
    pass1_1010_7fd6(_u16_1050_14cc);
  }
  mem_1000_0016(_PTR_LOOP_1050_03a0);
  mem_1000_0016(_PTR_LOOP_1050_029c);
  mem_1000_0016(_PTR_LOOP_1050_4fb8);
  mem_1000_0016(_PTR_LOOP_1050_68a2);
  mem_1000_0016(_PTR_LOOP_1050_5744);
  uVar1 = mem_1000_0016(_PTR_LOOP_1050_5f2c);
  return uVar1;
}
pub fn pass1_1008_0984(mut param_1: i16,mut param_2: u16 ,mut param_3: i16)

{
  code **ppcVar1;
  let mut in_EDX: u32;

  set_sys_color_1008_357e((astruct_53 *)CONCAT22(param_2,param_1),param_3,in_EDX);
  if (*(i32 *)(param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(param_1 + 0xe8) + 0x98);
    (**ppcVar1)();
  }
  return;
}
pub fn menu_ui_op_1008_09ba(param_1: *mut astruct_853,HWND16 param_2,RECT16 *param_3)

{
  HMENlet mut HVar1: u16;
  astruct_853 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_853 *)param_1;
  if (iVar2.field235_0xec == 0x0) {
    HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150,HINSTANCE16_1050_038c);
    iVar2.field235_0xec = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    HVar1 = GetSubMenu16(0x0,iVar2.field235_0xec);
    iVar2.field235_0xec = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  ClientToScreen16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),iVar2.field8_0x8);
  HVar1 = iVar2.field235_0xec;
  TrackPopupMenu16(NULL,HWND16_1050_0396,0x0,HVar1,0x0,0x0,HVar1);
  return;
}



u16 unk_win_msg_op_1008_0a3c(mut param_1: u32,mut param_2: u16 )

{
  let mut BVar1:bool;
  let mut iVar2: i16;
  let mut uVar3: u16;

  iVar2 = (int)param_1;
  uVar3 = (param_1 >> 0x10);
  if ((param_2 & 0xfff0) == 0xf140) {
    return (iVar2 + 0xde);
  }
  if ((param_2 & 0xfff0) == 0xf060) {
    BVar1 = IsIconic16(*(HWND16 *)(iVar2 + 0x8));
    if (BVar1 == 0x0) {
      PostMessage16(0x0,0x67,0x111,*(HWND16 *)(iVar2 + 0x8));
    }
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_0a92(mut param_1: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0xee) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xee) + 0x90);
    (**ppcVar1)();
  }
  if (*(i32 *)(iVar2 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xe8) + 0x90);
    (**ppcVar1)();
  }
  if (_PTR_LOOP_1050_0388 != NULL) {
    ppcVar1 = (code **)*_PTR_LOOP_1050_0388;
    (**ppcVar1)();
  }
  post_quit_msg_1008_3af4();
  return;
}
pub fn window_op_1008_0af8(mut param_1: u16 ,StructA *struct_param_1)

{
  StructA *pSVar1;
  HWND16 HVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  u8 *puVar5;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  StructA *iVar8;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut puVar10: *mut u16;
  let mut uVar11: u16;
  u8 uVar12;
  let mut uVar13: u16;
  astruct_20 *struct_20_v6;
  astruct_57 *paVar7;
  code **fn_ptr_1;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  create_window_ex_1008_9760(struct_param_1);
  uVar8 = ((u32)struct_param_1 >> 0x10);
  iVar8 = (StructA *)struct_param_1;
  HVar2 = iVar8.field4_0x8;
  HWND16_1050_0396 = HVar2;
  mem_op_1000_179c(0x12,paVar6);
  uVar4 = paVar6 | HVar2;
  paVar7 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
  if (uVar4 != 0x0) {
    puVar10 = pass1_1008_91ba((astruct_3 *)CONCAT22(paVar6,HVar2));
    paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)puVar10 >> 0x10);
    HVar2 = (HWND16)puVar10;
  }
  mem_op_1000_179c(0x6,paVar7);
  uVar4 = paVar7 | HVar2;
  paVar6 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)uVar4);
  if (uVar4 == 0x0) {
    (u32)&iVar8[0x1].field10_0x14 = 0x0;
  }
  else {
    pass1_1008_392e((u16 *)CONCAT22(paVar7,HVar2),iVar8.field4_0x8);
    iVar8[0x1].field10_0x14 = HVar2;
    iVar8[0x1].field11_0x16 = (i16)paVar6;
  }
  fn_ptr_1 = (code **)((int)(u32)struct_param_1 + 0x14);
  (**fn_ptr_1)(0x1000,struct_param_1,0x0,0x15a,(int)&DAT_1050_1050);
  uVar9 = 0x1000;
  mem_op_1000_179c(0xec,paVar6);
  struct_20_v6 = (astruct_20 *)CONCAT22(paVar6,HVar2);
  uVar4 = paVar6 | HVar2;
  if (uVar4 == 0x0) {
    (u32)&iVar8[0x1].field12_0x18 = 0x0;
  }
  else {
    pSVar1 = iVar8 + 0x1;
    pSVar1.field0_0x0 = pSVar1.field0_0x0 + 0x1;
    uVar9 = 0x1020;
    pass1_1020_08b6(struct_20_v6,(iVar8 + 0x1)->field0_0x0,(u32)struct_param_1);
    iVar8[0x1].field12_0x18 = HVar2;
    iVar8[0x1].field13_0x1a = uVar4;
  }
  if (*(i32 *)&iVar8[0x1].field1_0x2 != 0x0) {
    fn_ptr_1 = (code **)((int)*(u32*)&iVar8[0x1].field1_0x2 + 0x10);
    (**fn_ptr_1)();
  }
  (u32)&iVar8[0x1].field1_0x2 = (u32)&iVar8[0x1].field12_0x18;
  uVar13 = 0x1;
  uVar3 = (u32)&iVar8[0x1].field12_0x18;
  uVar11 = uVar3;
  uVar12 = (u8)((u32)uVar3 >> 0x10);
  fn_ptr_1 = (code **)((int)*(u32*)&iVar8[0x1].field12_0x18 + 0x10);
  (**fn_ptr_1)();
  uVar3 = (u32)&iVar8[0x1].field12_0x18;
  puVar5 = (u8 *)iVar8[0x1].field13_0x1a;
  (u32)&iVar8[0x1].field14_0x1c = uVar3;
  fn_ptr_1 = (code **)((int)*(u32*)&iVar8[0x1].field14_0x1c + 0x8);
  (**fn_ptr_1)(uVar9,iVar8[0x1].field14_0x1c,puVar5,uVar11,uVar12,uVar13);
  uVar4 = uVar3;
  fn_ptr_1 = (code **)((int)*(u32*)&iVar8[0x1].field14_0x1c + 0xc);
  (**fn_ptr_1)();
  pass1_1008_6978(uVar4,puVar5,(u32)struct_param_1 & 0xffff | (u32)uVar8 << 0x10,0x0,
                  (u32)&iVar8[0x1].field14_0x1c);
  return;
}



// WARNING: Unable to use type for symbol puVar1
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 mixed_win_op_1008_0c60
                 (mut param_1: u16 ,StructD *param_2,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut astruct_72,mut param_6: u16 ,
                 mut param_7: u16 ,mut param_8: u16 )

{
  code **ppcVar1;
  HINSTANCE16 HVar2;
  let mut iVar3: i16;
  let mut BVar4:bool;
  let mut uVar7: u16;
  StructD *pSVar8;
  let mut uVar15: u16;
  astruct_72 *struct_var5;
  let mut uVar6: u16;
  uchar in_AF;
  let mut uVar5: u32;
  LRESULT lresult_6;
  char *pcVar16;
  u32 *puVar17;
  let mut in_stack_0000fcd2: u16;
  let mut in_stack_0000fce4: u16;
  let mut in_stack_0000fcf8: u16;
  WNDCLASS16 *in_stack_0000fd58;
  let mut in_stack_0000fe34: u16;
  let mut in_stack_0000fe3a: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5e: u16;
  let mut in_stack_0000ff62: u16;
  let mut in_stack_0000ff64: u16;
  let mut in_stack_0000ff68: u16;
  WPARAM16 WVar18;
  let mut uVar19: u16;
  u8 local_64 [0x50];
  let mut uStack20: u32;
  HCURSOR16 HStack16;
  HCURSOR16 HStack14;
  let mut uStack6: u32;
  u32 *puVar1;
  u8 uVar9;
  u8 uVar10;
  let mut iVar12: i16;
  let mut uVar13: u16;
  astruct_72 *struct_var15;
  let mut uVar14: u16;
  let mut in_stack_0000ff92: u16;
  u8 uVar11;
  let mut uVar12: u16;
  astruct_57 *paVar9;

  struct_var5 = (astruct_72 *)param_5;
  uVar6 = ((u32)param_5 >> 0x10);
  pSVar8 = (StructD *)param_2;
  switch(param_6) {
  case 0x64:
    BVar4 = pass1_1008_07d8(param_1,(astruct_57 *)param_2,struct_var5);
    win_ui_cursor_op_1008_2e9a(param_2,param_5,in_stack_0000fd58,in_stack_0000fcd2,in_stack_0000fce4,in_stack_0000fcf8);
    return BVar4;
  case 0x65:
    pass1_1008_3018((u8 *)pSVar8,(u32)param_5);
    return param_1;
  case 0x66:
    pass1_1008_30cc(param_1,pSVar8,param_5);
    return param_1;
  case 0x67:
    iVar3 = win_ui_op_1008_2b54(param_1,pSVar8,param_5);
    if (iVar3 == 0x0) {
      return 0x0;
    }
  case 0xee:
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0x0;
    uVar9 = '\x10';
    uVar10 = '\0';
    goto LAB_1008_0d18;
  case 0x68:
    pass1_1030_8344((u32)_u16_1050_5748,0x4000001);
    uVar7 = param_2 | param_1;
    paVar9 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar7);
    if (uVar7 == 0x0) {
      return param_1;
    }
    if (PTR_LOOP_1050_4fe8 != NULL) {
      pcVar16 = load_string_1010_847e((u32)_u16_1050_14cc,0x57b);
      BVar4 = MessageBox16(0x10,pcVar16,s_You_may_not_run_a_turn__The_game_1050_0172,struct_var5.field7_0x8);
      return BVar4;
    }
    HStack14 = LoadCursor16((char *)0x7f02,0x0);
    HStack16 = SetCursor16(HStack14);
    puVar17 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x29),in_stack_0000fe3a,
                              in_stack_0000ff5e,in_stack_0000ff64,in_stack_0000ff68);
    uVar15 = ((u32)paVar9 >> 0x10);
    uStack20._0_2_ = SUB42(puVar17,0x0);
    uStack20 = ((u32)puVar17 >> 0x10);
    pass1_1018_262e((u32)puVar17);
    pass1_1030_838e(_u16_1050_5748);
    ((int)_u16_1050_5748 + 0x8) = 0x1;
    pass1_1030_8326();
    pcVar16 = load_string_1010_847e((u32)_u16_1050_14cc,0x5dc);
    paVar9 = (astruct_57 *)CONCAT22(uVar15,(int)((u32)pcVar16 >> 0x10));
    sys_1000_3f9c((char *)CONCAT13(0x10,CONCAT12(0x50,local_64)),s__s__ld_1050_019c,pcVar16);
    uVar12 = 0x0;
    ppcVar1 = (code **)((int)(u32)param_5 + 0x14);
    (**ppcVar1)(0x0,struct_var5,(char)((u32)param_5 >> 0x10),0x0,local_64,(int)&DAT_1050_1050);
    puVar17 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(uVar12,0x37),in_stack_0000fe34,
                              in_stack_0000ff58,in_stack_0000ff5e,in_stack_0000ff62);
    pass1_1008_a9ec((u32)puVar17);
    SetCursor16(HStack16);
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0xfc;
    uVar11 = '\x11';
    goto LAB_1008_0e3d;
  default:
    if ((((int)&struct_var5.field227_0xe8 + 0x2) | &struct_var5.field227_0xe8) == 0x0) {
      return 0x0;
    }
    puVar1 = struct_var5.field227_0xe8;
    ppcVar1 = (code **)((int)*struct_var5.field227_0xe8 + 0x40);
    BVar4 = (**ppcVar1)(0x8,(int)puVar1,(char)((u32)puVar1 >> 0x10),param_6);
    return BVar4;
  case 0x6e:
    iVar12 = 0x2;
    goto LAB_1008_0cba;
  case 0x6f:
    uStack6._0_2_ = FUN_1010_830a(param_1,param_2,0x1008,_u16_1050_14cc,0x1f8);
    uStack6 = SUB42(param_2,0x0);
    BVar4 = WinHelp16(0x0,0x3,(char *)CONCAT22(uStack6,uStack6),struct_var5.field7_0x8);
    return BVar4;
  case 0x70:
    iVar12 = 0x1;//
LAB_1008_0cba:
    uVar5 = pass1_1038_af40(struct_var5,pSVar8,_PTR_LOOP_1050_5b7c,struct_var5.field7_0x8,iVar12);
    return (BOOL16)uVar5;
  case 0x71:
    HVar2 = WinExec16(0x3,s_notepad_read_me_1050_0162);
    return HVar2;
  case 0x79:
    BVar4 = post_msg_1008_2d22(param_5);
    return BVar4;
  case 0x7a:
    uVar13 = 0xb;
    goto LAB_1008_0f3e;
  case 0x7b:
    uVar13 = 0x1e;
    goto LAB_1008_0f3e;
  case 0x7c:
    uVar13 = 0x1f;
    goto LAB_1008_0f3e;
  case 0x7d:
    uVar13 = 0x21;
    goto LAB_1008_0f3e;
  case 0x7e:
    uVar13 = 0x35;
    goto LAB_1008_0f3e;
  case 0x7f:
    uVar14 = 0x39;
    break;
  case 0x80:
    uVar13 = 0x22;
    goto LAB_1008_0f3e;
  case 0x81:
    uVar14 = 0x36;
    break;
  case 0x82:
    uVar14 = 0x37;
    break;
  case 0x83:
    uVar14 = 0x38;
    break;
  case 0x84:
    uVar14 = 0x3a;
    break;
  case 0x85:
    uVar14 = 0x3b;
    break;
  case 0x86:
    uVar14 = 0x3c;
    break;
  case 0x87:
    uVar14 = 0x3d;
    break;
  case 0x88:
    uVar14 = 0x3e;
    break;
  case 0x89:
    uVar14 = 0x3f;
    break;
  case 0x8a:
    uVar14 = 0x40;
    break;
  case 0x8b:
    uVar13 = 0xc;
    goto LAB_1008_0f3e;
  case 0x8c:
    uVar14 = 0x41;
    break;
  case 0x8d:
    uVar14 = 0x42;
    break;
  case 0x8e:
    uVar14 = 0x43;
    break;
  case 0x8f:
    uVar14 = 0x44;
    break;
  case 0x90:
    uVar14 = 0x45;
    break;
  case 0x91:
    uVar14 = 0x46;
    break;
  case 0x92:
    uVar14 = 0x47;
    break;
  case 0x93:
    uVar13 = 0x23;
    goto LAB_1008_0f3e;
  case 0x94:
    uVar13 = 0x24;
    goto LAB_1008_0f3e;
  case 0x95:
    uVar14 = 0x48;
    break;
  case 0x96:
    uVar14 = 0x49;
    break;
  case 0x97:
    uVar14 = 0x4a;
    break;
  case 0x98:
    uVar14 = 0x4b;
    break;
  case 0x99:
    uVar14 = 0x4c;
    break;
  case 0x9a:
    uVar13 = 0xd;
    goto LAB_1008_0f3e;
  case 0x9b:
    uVar14 = 0x4d;
    break;
  case 0x9c:
    uVar14 = 0x4e;
    break;
  case 0x9d:
    uVar14 = 0x4f;
    break;
  case 0x9e:
    uVar14 = 0x50;
    break;
  case 0x9f:
    uVar14 = 0x51;
    break;
  case 0xa0:
    uVar13 = 0xe;
    goto LAB_1008_0f3e;
  case 0xa1:
    uVar13 = 0xf;
    goto LAB_1008_0f3e;
  case 0xa2:
    uVar14 = 0x52;
    break;
  case 0xa3:
    uVar13 = 0x10;
    goto LAB_1008_0f3e;
  case 0xa4:
    uVar14 = 0x53;
    break;
  case 0xa5:
    uVar13 = 0x11;
    goto LAB_1008_0f3e;
  case 0xa6:
    uVar13 = 0x12;
    goto LAB_1008_0f3e;
  case 0xa7:
    uVar14 = 0x57;
    break;
  case 0xa8:
    uVar13 = 0x13;
    goto LAB_1008_0f3e;
  case 0xa9:
    uVar13 = 0x14;
    goto LAB_1008_0f3e;
  case 0xaa:
    uVar14 = 0x58;
    break;
  case 0xab:
    uVar14 = 0x63;
    break;
  case 0xac:
    uVar14 = 0x59;
    break;
  case 0xad:
    uVar14 = 0x5a;
    break;
  case 0xae:
    uVar14 = 0x5b;
    break;
  case 0xaf:
    uVar14 = 0x15;
    break;
  case 0xb0:
    uVar13 = 0x25;
    goto LAB_1008_0f3e;
  case 0xb1:
    uVar14 = 0x5c;
    break;
  case 0xb2:
    uVar14 = 0x16;
    break;
  case 0xb3:
    uVar14 = 0x5d;
    break;
  case 0xb4:
    uVar13 = 0x5e;
    goto LAB_1008_0f3e;
  case 0xb5:
    uVar14 = 0x5f;
    break;
  case 0xb6:
    uVar14 = 0x60;
    break;
  case 0xb7:
    uVar14 = 0x61;
    break;
  case 0xb8:
    uVar14 = 0x62;
    break;
  case 0xb9:
    uVar14 = 0x64;
    break;
  case 0xba:
    uVar14 = 0x65;
    break;
  case 0xbb:
    uVar14 = 0x66;
    break;
  case 0xbc:
    uVar14 = 0x67;
    break;
  case 0xbd:
    uVar14 = 0x68;
    break;
  case 0xbe:
    uVar14 = 0x69;
    break;
  case 0xbf:
    uVar13 = 0x17;
    goto LAB_1008_0f3e;
  case 0xc0:
    uVar13 = 0x18;
    goto LAB_1008_0f3e;
  case 0xc1:
    uVar13 = 0x19;
    goto LAB_1008_0f3e;
  case 0xc2:
    uVar13 = 0x1a;
    goto LAB_1008_0f3e;
  case 0xc3:
    uVar13 = 0x1b;
    goto LAB_1008_0f3e;
  case 0xc4:
    uVar13 = 0x1c;
    goto LAB_1008_0f3e;
  case 0xc5:
    uVar13 = 0x1d;
    goto LAB_1008_0f3e;
  case 0xc6:
    uVar13 = 0x4;
    goto LAB_1008_0f3e;
  case 0xc8:
    uVar13 = 0x3;
    goto LAB_1008_0f3e;
  case 0xc9:
    uVar13 = 0x1;
    goto LAB_1008_0f3e;
  case 0xca:
    uVar13 = 0x5;
    goto LAB_1008_0f3e;
  case 0xcb:
    pass1_1008_087e(in_AF,param_1,pSVar8);
    uVar13 = 0x6;
    goto LAB_1008_0f3e;
  case 0xcc:
    uVar13 = 0x7;
    goto LAB_1008_0f3e;
  case 0xcd:
    uVar13 = 0x8;
    goto LAB_1008_0f3e;
  case 0xce:
    uVar13 = 0x9;
    goto LAB_1008_0f3e;
  case 0xcf:
    uVar13 = 0xa;
    goto LAB_1008_0f3e;
  case 0xd0:
    uVar13 = 0x26;
    goto LAB_1008_0f3e;
  case 0xd1:
    uVar13 = 0x27;
    goto LAB_1008_0f3e;
  case 0xd2:
    uVar13 = 0x28;
    goto LAB_1008_0f3e;
  case 0xd3:
    uVar13 = 0x29;
    goto LAB_1008_0f3e;
  case 0xd4:
    uVar13 = 0x2a;
    goto LAB_1008_0f3e;
  case 0xd5:
    uVar13 = 0x2b;
    goto LAB_1008_0f3e;
  case 0xd6:
    uVar13 = 0x2c;
    goto LAB_1008_0f3e;
  case 0xd7:
    uVar13 = 0x2d;
    goto LAB_1008_0f3e;
  case 0xd8:
    uVar13 = 0x2e;
    goto LAB_1008_0f3e;
  case 0xd9:
    uVar13 = 0x2f;
    goto LAB_1008_0f3e;
  case 0xda:
    uVar13 = 0x30;
    goto LAB_1008_0f3e;
  case 0xdb:
    uVar13 = 0x31;
    goto LAB_1008_0f3e;
  case 0xdc:
    uVar13 = 0x32;
    goto LAB_1008_0f3e;
  case 0xdd:
    uVar13 = 0x33;
    goto LAB_1008_0f3e;
  case 0xde:
    uVar13 = 0x34;//
LAB_1008_0f3e:
    cursor_op_1008_2dcc(param_2,(astruct_20 *)param_5,uVar13,param_7,param_8);
    return param_1;
  case 0xdf:
    uVar14 = 0x55;
    break;
  case 0xe0:
    uVar14 = 0x56;
    break;
  case 0x100:
    win_1008_5c5c(param_1,pSVar8,_u16_1050_02a0,0x1dc);
    return param_1;
  case 0x12c:
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0xf020;
    uVar9 = '\x12';
    uVar10 = '\x01';//
LAB_1008_0d18:
    lresult_6 = SendMessage16(0x0,WVar18,CONCAT11(uVar10,uVar9),uVar19);
    return (BOOL16)lresult_6;
  case 0x12e:
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0xf060;
    uVar11 = '\x12';//
LAB_1008_0e3d:
    BVar4 = PostMessage16(0x0,WVar18,CONCAT11(0x1,uVar11),uVar19);
    return BVar4;
  }
  ui_op_1008_2c4e(pSVar8,param_8,param_5,uVar14);
  return param_1;
}
pub fn caseD_a7(mut param_1: u16 ,mut param_2: u16 )

{
  let mut unaff_BP: i16;
  astruct_72 *uVar1;

  ui_op_1008_2c4e(param_1,param_2,*(astruct_72 **)(unaff_BP + 0x6),0x57);
  return;
}
pub fn caseD_aa(void)

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x58);
  return;
}
pub fn caseD_ac(void)

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x59);
  return;
}
pub fn caseD_ad(void)

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5a);
  return;
}
pub fn caseD_ae(void)

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5b);
  return;
}
pub fn caseD_b1(void)

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5c);
  return;
}
pub fn caseD_b3(void)

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5d);
  return;
}
pub fn draw_op_1008_1230(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (u32)((int)param_1 + 0xe0);
  fill_rect_1008_39ac((astruct_930 *)uVar1,((u32)uVar1 >> 0x10));
  return;
}
pub fn pass1_1008_1246(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0xe8) + 0x4c);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_1272(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0xe8) + 0x88);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9cc4(param_1 & 0xffff | (u32)uVar2 << 0x10,param_2);
  return;
}
pub fn pass1_1008_12aa(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0xe8) + 0x8c);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9ce0();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn message_box_op_1008_12dc(param_1: *mut astruct_72,mut param_2: u32)

{
  let mut BVar1:bool;
  let mut uVar2: u16;
  let mut in_DX: u16;
  HWND16 HVar3;
  let mut uVar4: u16;
  char *pcVar5;
  let mut uVar6: u16;
  HWND16 hwnd;
  let mut uStack16: u32;
  u8 local_c [0x6];
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  HStack4 = LoadCursor16((char *)0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  str_1008_6d8a(in_DX,(u32 *)CONCAT22(0x1050,local_c),(char *)param_2);
  BVar1 = file_fn_1008_6e02((astruct_802 *)CONCAT22(0x1050,local_c));
  uVar4 = ((u32)param_1 >> 0x10);
  if (BVar1 == 0x0) {
    SetCursor16(HStack6);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,u16_1050_0310);
    HVar3 = (HWND16)((u32)pcVar5 >> 0x10);
    uVar2 = str_op_1008_60e8(HVar3,pcVar5);
    uStack16 = CONCAT22(HVar3,uVar2);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    MessageBeep16(0x10);
    MessageBox16(0x10,pcVar5,(char *)CONCAT22(HVar3,uVar2),*(HWND16 *)((int)param_1 + 0x8));
  }
  else {
    ((int)_u16_1050_5748 + 0x8) = 0x0;
    SetCursor16(HStack6);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,0x6d3);
    str_op_1008_60e8(((u32)pcVar5 >> 0x10),pcVar5);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    uVar6 = 0x0;
    MessageBeep16(0x0);
    hwnd = *(HWND16 *)((int)param_1 + 0x8);
    HVar3 = hwnd;
    MessageBox16(0x40,pcVar5,(char *)CONCAT22(hwnd,uVar6),hwnd);
    uStack16 = CONCAT22(HVar3,hwnd);
  }
  fn_ptr_1000_17ce((char *)(uStack16 & 0xffff | (u32)HVar3 << 0x10));
  close_file_1008_6dd0((StructD *)CONCAT22(0x1050,local_c));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1008_1414(mut param_1: u32,param_2: *mut astruct_20,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,
                        mut param_6: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut BVar3:bool;
  let mut uVar4: u16;
  let mut iVar5: i16;
  u32 *puVar5;
  let mut extraout_DX: u16;
  let mut uVar6: u32;
  astruct_57 *paVar7;
  let mut uVar8: u32;
  astruct_20 *iVar17;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  astruct_72 *uVar16;
  u32 *puVar9;
  char *pcVar10;
  u32 *puVar11;
  astruct_27 *paVar12;
  let mut in_stack_0000fe3c: u16;
  let mut in_stack_0000fe3e: u16;
  let mut in_stack_0000fe4e: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ff62: u16;
  let mut in_stack_0000ff66: u16;
  let mut in_stack_0000ff68: u16;
  let mut in_stack_0000ff6a: u16;
  let mut in_stack_0000ff6c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff78: u16;
  let mut in_stack_0000ff7c: u16;
  u8 uVar13;
  u8 uVar14;
  let mut uVar15: u16;
  let mut local_2a: u32;
  let mut uStack38: u16;
  let mut iStack36: i16;
  let mut uStack34: u16;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut uStack16: u32;
  u32 *puStack12;
  u8 local_8 [0x6];
  let mut uVar10: u16;

  puVar9 = str_1008_6d8a(param_1,(u32 *)CONCAT22(0x1050,local_8),(char *)param_3);
  paVar7 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)puVar9 >> 0x10);
  BVar3 = read_file_1008_6e78((astruct_806 *)CONCAT22(0x1050,local_8),unaff_SI,unaff_DI,param_4);
  iVar17 = (astruct_20 *)param_2;
  uVar16 = (astruct_72 *)((u32)param_2 >> 0x10);
  if (BVar3 == 0x0) {
    if (u16_1050_0310 == 0x0) {
      u16_1050_0310 = 0x6d4;
    }
    pcVar10 = load_string_1010_847e(_u16_1050_14cc,u16_1050_0310);
    uVar8 = (u32)paVar7 & 0xffff0000 | (u32)pcVar10 >> 0x10;
    uVar4 = str_op_1008_60e8(((u32)pcVar10 >> 0x10),pcVar10);
    uVar15 = uVar8;
    pcVar10 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    paVar7 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)pcVar10 >> 0x10);
    MessageBeep16(0x10);
    MessageBox16(0x10,pcVar10,(char *)CONCAT22(uVar15,uVar4),iVar17.field3_0x8);
    fn_ptr_1000_17ce((char *)CONCAT22(uVar15,uVar4));
    fn_ptr_op_1000_24cd(0x1);
  }
  cursor_op_1008_2dcc(paVar7,param_2,0x8,param_6,param_5);
  puStack12 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_DI,0x2f),in_stack_0000fe4e,
                              in_stack_0000ff72,in_stack_0000ff78,in_stack_0000ff7c);
  paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)puStack12 >> 0x10);
  uVar6 = (u32)((int)puStack12 + 0x20);
  uStack16 = uVar6;
  pass1_1030_8344(_u16_1050_5748,uVar6);
  uStack20 = uVar6 & 0xffff | (long)paVar7 << 0x10;
  uStack24 = (u32)((int)uVar6 + 0x10);
  iVar5 = ((int)uStack24 + 0x2) + -0x1;
  uVar1 = (u32)((int)&iVar17[0x1].field2_0x4 + 0x2);
  ppcVar2 = (code **)((int)(u32)(u32)((int)&iVar17[0x1].field2_0x4 + 0x2) + 0x4);
  (**ppcVar2)(0x1030,(int)uVar1,(int)((u32)uVar1 >> 0x10),(int)uStack16,(int)(uStack16 >> 0x10),iVar5,0x2);
  pass1_1030_8344(_u16_1050_5748,0x4000001);
  uStack28 = CONCAT22((int)paVar7,iVar5);
  uVar6 = (u32)(iVar5 + 0x10);
  uStack32 = uVar6;
  pass1_1030_8344(_u16_1050_5748,uVar6);
  iStack36 = (int)uVar6;
  uStack34 = SUB42(paVar7,0x0);
  local_2a = (u32)(iStack36 + 0xc);
  uStack38 = (iStack36 + 0x10);
  puVar5 = (u32 *)pass1_1030_5b00(uStack20);
  uVar15 = SUB42(&DAT_1050_1050,0x0);
  uVar13 = SUB21(&local_2a,0x0);
  uVar14 = (u8)(&local_2a >> 0x8);
  puVar11 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT13(uVar14,CONCAT12(uVar13,puVar5)),
                            in_stack_0000fe3e,in_stack_0000ff62,in_stack_0000ff68,in_stack_0000ff6c);
  paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)puVar11 >> 0x10);
  pass1_1018_179e((u32)puVar11,CONCAT22(uVar15,CONCAT11(uVar14,uVar13)));
  uVar13 = 0x0;
  uVar14 = 0x4;
  iVar5 = 0x1b;
  paVar12 = (astruct_27 *)
            mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe3c,in_stack_0000ff60,
                            in_stack_0000ff66,in_stack_0000ff6a);
  pass1_1010_043a(paVar12,CONCAT13(uVar14,CONCAT12(uVar13,0x1)),iVar5);
  close_file_1008_6dd0((StructD *)CONCAT22(0x1050,local_8));
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn big_switch_1008_15d4(param_1: *mut astruct_20,param_2: *mut astruct_72,i32 param_3)

{
  let mut var3: u16;
  u8 *var2;
  let mut var4: u16;
  astruct_20 *var5;
  u8 *puVar1;
  let mut uVar3: u16;
  astruct_57 *in_EDX;
  let mut uVar4: u32;
  let mut var6: u16;
  astruct_20 *paVar2;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut uVar5: u16;
  astruct_20 *paStack32;
  u8 local_e [0x8];
  let mut uStack6: u32;
  let mut uVar2: u32;
  i32 *var_1;
  let mut piVar1: *mut i16;

  uStack6 = 0x0;
  var3 = param_2;
  var3 = var3 + 0xd2;
  pass1_1008_57a4((u32 *)CONCAT22(0x1050,local_e),(u32)param_2 & 0xffff0000 | (u32)var3);
  do {
    var2 = local_e;
    pass1_1008_5b12((char *)CONCAT22(0x1050,var2));
    uVar3 = in_EDX;
    var5 = (astruct_20 *)(uVar3 | var2);
    uVar4 = (u32)in_EDX & 0xffff0000;
    in_EDX = (astruct_57 *)(uVar4 | ZEXT24(var5));
    if (var5 == NULL) goto LAB_1008_162a;
    uVar2 = (u32)(var2 + 0x4);
    uVar3 = (var2 + 0x6);
    in_EDX = (astruct_57 *)(uVar4 | uVar3);
    param_1 = (astruct_20 *)uVar2;
  } while (param_1.field164_0xde != param_3);
  uStack6 = uVar2 & 0xffff | (u32)uVar3 << 0x10;//
LAB_1008_162a:
  if (uStack6 != 0x0) {
    return;
  }
  uVar5 = ((u32)param_2 >> 0x10);
  switch(param_3 - 0x1) {
  case 0x0:
    mem_op_1000_179c(0xec,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) {//
LAB_1008_169a:
      puVar1 = (u8 *)uVar4;
      uStack6 = 0x0;
      goto LAB_1008_2b3a;
    }
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1020_08b6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    break;
  default:
    debug_print_1008_6048(in_EDX,s_OpWnd__getKid__Unknown_target_mo_1050_01a3);
    puVar1 = (u8 *)in_EDX;
    fn_ptr_op_1000_24cd(0x1);
    goto LAB_1008_2b3a;
  case 0x2:
    mem_op_1000_179c(0xfa,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1018_e91e(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x3:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1018_e230(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x4:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1020_7554(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x5:
    mem_op_1000_179c(0xf8,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)in_EDX & 0xffff0000 | (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1018_5840(uVar4,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5,
                     in_stack_0000fe78,in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
    puVar1 = (u8 *)uVar4;
    break;
  case 0x6:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1020_2524(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x7:
    mem_op_1000_179c(0x118,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    unk_draw_op_1020_41c8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,0x1020);
    break;
  case 0x8:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1018_e5dc(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x9:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (u8 *)(in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1018_66cc(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0xa:
    win_1008_5c5c(param_1,in_EDX,_u16_1050_02a0,0x1d3);
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6d02((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xb:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6d38((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xc:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6d6e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xd:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6da4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xe:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6dda((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xf:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6e10((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x10:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6e46((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x11:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6e7c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x12:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6eb2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x13:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6ee8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x14:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6f1e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x15:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6f54((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x16:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6f8a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x17:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6fc0((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x18:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6ff6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x19:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_702c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7062((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7098((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_70ce((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7104((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_713a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2,
                              &DAT_1050_1050);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x20:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7170((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x21:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_745e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x22:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_71a6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x23:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_71dc((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x24:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7212((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x25:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_c958((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x26:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_c9a6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x27:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_c9f4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x28:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_ca48((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x29:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_ca96((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2a:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_caea((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2b:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cb38((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2c:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cb86((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2d:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cbda((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2e:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cc28((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2f:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cc76((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x30:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_ccc4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x31:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cd12((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x32:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cd60((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x33:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cf74((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x34:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_73c2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x35:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7494((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x36:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_74ca((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x37:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7500((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x38:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_73f8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x39:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7536((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_756c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_75a2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_75d8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_760e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7644((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3f:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_767a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x40:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_76b0((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x41:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_76e6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x42:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_771c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x43:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7752((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x44:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7788((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x45:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_77be((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x46:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_77f4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x47:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_782a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x48:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7860((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x49:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7896((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_78cc((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7902((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7938((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_796e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_79a4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4f:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_79da((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x50:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7a10((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x51:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7a46((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x52:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7a7c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x54:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7ab2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x55:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7ae8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x56:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7b1e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x57:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7b54((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x58:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7b8a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x59:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7bc0((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7bf6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7c2c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7c62((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7c98((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7cce((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5f:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7d04((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x60:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7d3a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x61:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7d70((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x62:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7248((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x63:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_727e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x64:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_72b4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x65:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_72ea((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x66:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7320((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x67:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    piVar1 = (i16 *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7356((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x68:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = (u32)uVar3;
    if (uVar3 == 0x0) goto LAB_1008_169a;
    var_1 = (i32 *)(var3 + 0xcc);
    var_1 = var_1 + 0x1;
    paVar2 = struct_1018_738c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),(u32)param_2);
    puVar1 = (u8 *)((u32)paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
  }
  uStack6 = CONCAT22(puVar1,param_1);//
LAB_1008_2b3a:
  pass1_1008_6978(param_1,puVar1,(u32)param_2,0x0,uStack6);
  return;
}


/*
Unable to decompile 'FUN_1008_1df2'
Cause:
Low-level Error: Symbol $$undef00000008 extends beyond the end of the address space
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 win_ui_op_1008_2b54(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  let mut uVar7: u16;
  char *pcVar8;
  u32 *local_a6 [0x14];
  u8 local_56 [0x50];
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut uVar6: u32;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x0;
  if (_PTR_LOOP_1050_4230 == 0x0) {
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x5f2);
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_56),pcVar8);
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_a6),pcVar8);
    iStack4 = MessageBox16(0x21,(char *)CONCAT13(0x10,CONCAT12(0x50,local_a6)),(char *)CONCAT22(0x1050,local_56),
                           HWND16_1050_0396);
  }
  else {
    uVar7 = 0x1000;
    mem_op_1000_179c(0xb4,paVar5);
    uVar3 = paVar5 | param_1;
    uVar6 = (u32)paVar5 & 0xffff0000 | (u32)uVar3;
    if (uVar3 == 0x0) {
      iVar2 = 0x0;
      uVar4 = 0x0;
    }
    else {
      uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar6,(astruct_57 *)CONCAT22(paVar5,param_1),HWND16_1050_0396,0x20021,0x5f2057b);
      uVar4 = uVar6;
    }
    local_a6[0] = (u32 *)CONCAT22(uVar4,iVar2);
    ppcVar1 = (code **)((int)*local_a6[0] + 0x74);
    iStack4 = (**ppcVar1)(uVar7,iVar2,uVar4,param_1);
  }
  iStack6 = iStack4;
  if (iStack4 != 0x1) {
    iStack6 = 0x0;
  }
  if (((iStack6 != 0x0) && (_u16_1050_5748 != 0x0)) &&
     (uVar3 = ((int)_u16_1050_5748 + 0x8),
     local_a6[0] = (u32 *)((u32)local_a6[0] & 0xffff0000 | (u32)uVar3), uVar3 != 0x0)) {
    PostMessage16(0x0,0xb4,0x111,*(HWND16 *)((int)param_3 + 0x8));
    iStack6 = 0x0;
  }
  return iStack6;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn ui_op_1008_2c4e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_72,mut param_4: u16 )

{
  let mut uVar1: u32;
  HCURSOR16 hcursor_5;
  let mut uVar3: u16;
  astruct_20 *paVar2;
  astruct_20 *uVar5;
  let mut in_stack_0000fff8: u16;
  let mut uVar2: u32;
  astruct_20 *ppaVar1;
  code **fn_ptr_1;

  hcursor_5 = LoadCursor16((char *)0x7f02,0x0);
  hcursor_5 = SetCursor16(hcursor_5);
  uVar5 = (astruct_20 *)CONCAT22(param_1,hcursor_5);
  ppaVar1 = (astruct_20 *)&((astruct_20 *)param_3)[0x1].field7_0x10;
  ppaVar1.offset_0x0 = ppaVar1.offset_0x0 + 0x1;
  paVar2 = (astruct_20 *)param_3;
  if (*(i32 *)&((astruct_20 *)param_3)[0x1].field5_0xc != 0x0) {
    uVar2 = (u32)&((astruct_20 *)param_3)[0x1].field5_0xc;
    paVar2 = (astruct_20 *)*(u32*)&((astruct_20 *)param_3)[0x1].field5_0xc;
    fn_ptr_1 = (code **)&paVar2.field_0x90;
    uVar5 = (astruct_20 *)(**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  }
  big_switch_1008_15d4(paVar2,param_3,CONCAT22(param_2,param_4));
  uVar3 = ((u32)uVar5 >> 0x10);
  ((astruct_20 *)param_3)[0x1].field5_0xc = (astruct_20 *)uVar5;
  ((astruct_20 *)param_3)[0x1].field6_0xe = uVar3;
  fn_ptr_1 = (code **)((int)*(u32*)&((astruct_20 *)param_3)[0x1].field5_0xc + 0x8);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,((astruct_20 *)param_3)[0x1].field5_0xc,uVar3);
  if (*(i32 *)((int)&((astruct_20 *)param_3)[0x1].field2_0x4 + 0x2) != 0x0) {
    uVar1 = (u32)((int)&((astruct_20 *)param_3)[0x1].field2_0x4 + 0x2);
    fn_ptr_1 = (code **)((int)(u32)(u32)((int)&((astruct_20 *)param_3)[0x1].field2_0x4 + 0x2) + 0xc)
    ;
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)uVar1,(char)((u32)uVar1 >> 0x10),0x0);
  }
  show_win_1038_b634(_PTR_LOOP_1050_5b7c);
  show_win_1010_7a76((u32)((int)&((astruct_20 *)param_3)[0x1].field7_0x10 + 0x2));
  uVar1 = (u32)&((astruct_20 *)param_3)[0x1].field5_0xc;
  fn_ptr_1 = (code **)((int)*(u32*)&((astruct_20 *)param_3)[0x1].field5_0xc + 0xc);
  (**fn_ptr_1)(0x1010,(int)uVar1,(char)((u32)uVar1 >> 0x10),0x5);
  uVar1 = (u32)&((astruct_20 *)param_3)[0x1].field5_0xc;
  BringWindowToTop16(*(HWND16 *)((int)uVar1 + 0x8));
  SetCursor16(hcursor_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn post_msg_1008_2d22(param_1: *mut astruct_72)

{
  let mut piVar1: *mut i16;
  u32 *puVar2;
  code **ppcVar3;
  astruct_72 *iVar4;
  let mut uVar4: u16;
  u32 *puVar5;
  let mut uVar6: u16;
  u32 *puVar7;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_72 *)param_1;
  if ((iVar4.field230_0xee != NULL) &&
     (piVar1 = &iVar4.field231_0xf2, *piVar1 = *piVar1 + -0x1, iVar4.field231_0xf2 < 0x1)) {
    puVar7 = iVar4.field230_0xee;
    ppcVar3 = (code **)((int)*iVar4.field230_0xee + 0x90);
    (**ppcVar3)();
    iVar4.field230_0xee = NULL;
    iVar4.field231_0xf2 = 0x0;
    if (iVar4.field227_0xe8 != NULL) {
      uVar6 = 0x3;
      puVar5 = iVar4.field227_0xe8;
      ppcVar3 = (code **)((int)*iVar4.field227_0xe8 + 0xc);
      (**ppcVar3)();
      show_win_1038_b68a(_PTR_LOOP_1050_5b7c);
      show_window_1010_7ace(iVar4.field232_0xf4);
      puVar2 = iVar4.field227_0xe8;
      ppcVar3 = (code **)((int)*iVar4.field227_0xe8 + 0x98);
      (**ppcVar3)(0x1010,(int)puVar2,(char)((u32)puVar2 >> 0x10),0x1,puVar5,uVar6,puVar7);
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  return;
}
pub fn cursor_op_1008_2dcc(mut param_1: u16 ,param_2: *mut astruct_20,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  HCURSOR16 cursor_handle;
  HCURSOR16 hcursor;
  let mut extraout_DX: u16;
  astruct_20 *paVar3;

  cursor_handle = LoadCursor16((char *)0x7f02,0x0);
  hcursor = SetCursor16(cursor_handle);
  paVar3 = (astruct_20 *)param_2;
  cursor_handle = hcursor;
  if (*(i32 *)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2) != 0x0) {
    uVar1 = (u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2);
    paVar3 = (astruct_20 *)(u32)(u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2);
    ppcVar2 = (code **)&paVar3.field_0x90;
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((u32)uVar1 >> 0x10));
    param_1 = extraout_DX;
  }
  big_switch_1008_15d4(paVar3,(astruct_72 *)param_2,CONCAT22(param_5,param_3));
  *(HCURSOR16 *)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2) = cursor_handle;
  ((astruct_20 *)param_2)[0x1].field3_0x8 = param_1;
  uVar1 = (u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2);
  if (((int)uVar1 + 0xe0) == 0x0) {
    uVar1 = (u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2);
    ppcVar2 = (code **)((int)(u32)(u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2) + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((u32)uVar1 >> 0x10));
    uVar1 = (u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2);
    ppcVar2 = (code **)((int)(u32)(u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2) + 0xc);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(char)((u32)uVar1 >> 0x10),0x3);
    ((astruct_20 *)param_2)->field153_0xce = (u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2);
  }
  else {
    (u32)((int)&((astruct_20 *)param_2)[0x1].field2_0x4 + 0x2) = 0x0;
    ui_op_1008_2c4e(param_1,param_4,(astruct_72 *)param_2,param_3);
    ((astruct_20 *)param_2)->field153_0xce = 0x0;
  }
  SetCursor16(hcursor);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_cursor_op_1008_2e9a
               (param_1: *mut astruct_57,param_2: *mut astruct_72,WNDCLASS16 *param_3,mut param_4: u16 ,mut param_5: u16 ,
               mut param_6: u16 )

{
  char *pcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  HCURSOR16 HVar6;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut unaff_SI: u16;
  astruct_72 *uVar7;
  let mut in_stack_0000fc78: u16;
  let mut in_stack_0000fd9c: u16;
  let mut in_stack_0000fda2: u16;
  let mut in_stack_0000fda6: u16;
  char local_224 [0x108];
  let mut uStack266: u16;
  let mut uStack262: u32;
  char local_102 [0x100];

  local_102[0] = '\0';
  uStack262 = (astruct_73 *)
              mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fc78,
                              in_stack_0000fd9c,in_stack_0000fda2,in_stack_0000fda6);
  uVar3 = ((u32)uStack262 >> 0x10);
  iVar4 = (int)uStack262;
  pcVar1 = *(char **)(iVar4 + 0x16);
  uVar5 = (iVar4 + 0x18);
  uVar9 = (u32)param_1 & 0xffff0000 | (u32)uVar5;
  uStack266 = pcVar1;
  uStack266 = uVar5 | uStack266;
  if (uStack266 == 0x0) {
    save_file_1008_3178(uVar5,param_2,0x1);
    uVar5 = uVar9;
    uVar8 = uVar5 | uStack266;
    uVar9 = uVar9 & 0xffff0000 | (u32)uVar8;
    if (uVar8 == 0x0) {
      PostMessage16(0x0,0x13d,0x111,HWND16_1050_0396);
      return;
    }
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_102),(char *)CONCAT22(uVar5,uStack266));
    str_1000_4d58((char *)CONCAT22(0x1050,local_102),NULL,0x0,CONCAT22(0x1050,local_224),
                  (WNDCLASS16 *)CONCAT22(0x1050,&param_3));
    if ((char)param_3 != '\0') {
      pass1_1000_3cea(CONCAT22(0x1050,local_224),(char *)CONCAT22(0x1050,&param_3));
    }
    struct_1010_5f1e(uVar9,uStack262,CONCAT22(0x1050,local_224));
  }
  else {
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_102),*(char **)(iVar4 + 0x1a));
    uVar5 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_102));
    if (local_102[uVar5 - 0x1] != '\\') {
      local_102[uVar5] = '\\';
      local_102[uVar5 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(0x1050,local_102),pcVar1);
  }
  if (local_102[0] != '\0') {
    uVar7 = (astruct_72 *)((u32)param_2 >> 0x10);
    send_msg_1020_097e((u32)((int)param_2 + 0xe8));
    uVar2 = (u32)((int)param_2 + 0xe8);
    UpdateWindow16(*(HWND16 *)((int)uVar2 + 0x8));
    HVar6 = LoadCursor16((char *)0x7f02,0x0);
    param_3 = (WNDCLASS16 *)((u32)param_3 & 0xffff0000 | (u32)HVar6);
    HVar6 = SetCursor16(HVar6);
    param_3 = (WNDCLASS16 *)CONCAT22(0x1050,local_102);
    win_ui_op_1008_1414(uVar9,(astruct_20 *)param_2,(u32)param_3,param_6,param_5,param_4);
    param_3 = (WNDCLASS16 *)CONCAT22(HVar6,0x1538);
    SetCursor16(HVar6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_3018(u8 *param_1,mut param_2: u32)

{
  char *pcVar1;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd8a: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb4: u16;
  let mut in_stack_0000feb8: u16;
  let mut uStack266: u16;
  let mut uStack262: u32;
  char local_102 [0x100];

  local_102[0] = '\0';
  uStack262 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                              (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd8a,in_stack_0000feae,in_stack_0000feb4
                              ,in_stack_0000feb8);
  uVar2 = ((u32)uStack262 >> 0x10);
  iVar3 = (int)uStack262;
  pcVar1 = *(char **)(iVar3 + 0x12);
  uVar4 = (iVar3 + 0x14);
  uStack266 = pcVar1;
  if ((uVar4 | uStack266) == 0x0) {
    pass1_1008_30cc(0x0,uVar4,(astruct_72 *)param_2);
  }
  else {
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_102),*(char **)(iVar3 + 0x1a));
    uVar4 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_102));
    if (local_102[uVar4 - 0x1] != '\\') {
      local_102[uVar4] = '\\';
      local_102[uVar4 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(0x1050,local_102),pcVar1);
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc((astruct_72 *)param_2,CONCAT22(0x1050,local_102));
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_30cc(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_72)

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_484 *paVar2;
  let mut in_stack_0000fc90: u16;
  let mut in_stack_0000fdb4: u16;
  let mut in_stack_0000fdba: u16;
  let mut in_stack_0000fdbe: u16;
  u8 *puVar3;
  let mut uVar4: u16;
  char local_210 [0xa];
  u8 local_206 [0x100];
  let mut uStack262: u16;
  let mut uStack260: u16;
  char local_102 [0x100];

  local_102[0] = '\0';
  save_file_1008_3178(param_2,param_3,0x2);
  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1);
  if ((param_2 | param_1) != 0x0) {
    uStack262 = param_1;
    uStack260 = param_2;
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_102),(char *)CONCAT22(param_2,param_1));
    str_1000_4d58((char *)CONCAT22(0x1050,local_102),NULL,0x0,CONCAT22(0x1050,local_206),
                  (WNDCLASS16 *)CONCAT22(0x1050,local_210));
    if (local_210[0] != '\0') {
      pass1_1000_3cea(CONCAT22(0x1050,local_206),(char *)CONCAT22(0x1050,local_210));
    }
    puVar3 = local_206;
    uVar4 = SUB42(&DAT_1050_1050,0x0);
    paVar2 = (astruct_484 *)
             mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(puVar3,0x2),in_stack_0000fc90,
                             in_stack_0000fdb4,in_stack_0000fdba,in_stack_0000fdbe);
    pass1_1010_5f4c(((u32)paVar2 >> 0x10),paVar2,(char *)CONCAT22(uVar4,puVar3));
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc(param_3,CONCAT22(0x1050,local_102));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn save_file_1008_3178(mut param_1: u16 ,param_2: *mut astruct_72,mut param_3: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut success:bool;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  char *pcVar9;
  let mut in_stack_0000f720: u16;
  let mut in_stack_0000f844: u16;
  let mut in_stack_0000f84a: u16;
  let mut in_stack_0000f84e: u16;
  char local_782 [0x104];
  u16 local_67e [0x4];
  char *pcStack1654;
  let mut type: u16;
  let mut text: u16;
  char *pcStack1646;
  u8 local_666 [0x100];
  char *pcStack1382;
  SEGPTR ofn;
  let mut uStack1374: u16;
  char *pcStack1370;
  let mut uStack1368: u16;
  char *pcStack1354;
  let mut uStack1350: u32;
  u8 *puStack1346;
  let mut uStack1344: u16;
  let mut uStack1342: u32;
  char *pcStack1338;
  let mut uStack1336: u16;
  u8 *puStack1334;
  let mut uStack1332: u16;
  let mut uStack1330: u32;
  let mut uStack1326: u16;
  char *pcStack1322;
  let mut uStack1320: u16;
  char cStack1306;
  char acStack1305 [0x101];
  let mut uStack1048: u16;
  char local_416 [0x8];
  let mut uStack1038: u16;
  u8 local_40c [0x102];
  let mut uStack778: u32;
  astruct_487 *paStack774;
  u8 local_302 [0x100];
  u8 local_202 [0xff];
  char acStack259 [0x101];

  acStack259[1] = 0x0;
  local_302[0] = 0x0;
  local_202[0] = 0x0;
  paStack774 = (astruct_487 *)
               mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                               (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000f720,in_stack_0000f844,
                               in_stack_0000f84a,in_stack_0000f84e);
  uVar8 = ((u32)paStack774 >> 0x10);
  iVar2 = (int)paStack774;
  uStack778 = (u32)(iVar2 + 0x1a);
  uVar5 = (iVar2 + 0x1c);
  if ((uVar5 | uStack778) == 0x0) {
    pcStack1646 = *(char **)(iVar2 + 0x64);
    uVar5 = (iVar2 + 0x66);
    if ((uVar5 | pcStack1646) != 0x0) {
      pass1_1008_5784((char *)CONCAT22(0x1050,local_67e),(u32)pcStack1646 & 0xffff | (u32)uVar5 << 0x10);
      puVar3 = local_67e;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
      pcStack1654 = (char *)CONCAT22(uVar5,puVar3);
      if ((uVar5 | puVar3) != 0x0) {
        uVar1 = (u32)(puVar3 + 0x2);
        uStack778._0_2_ = uVar1;
        uVar5 = ((u32)uVar1 >> 0x10);
        goto LAB_1008_3206;
      }
    }
  }
  else {//
LAB_1008_3206:
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,acStack259 + 0x1),(char *)CONCAT22(uVar5,uStack778));
  }
  pass1_1000_5008(local_40c,&DAT_1050_1050,0x100);
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_40c));
  if (local_40c[uStack1038 - 0x1] == '\\') {
    local_40c[uStack1038 - 0x1] = 0x0;
  }
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(0x1050,acStack259 + 0x1));
  if (acStack259[uStack1038] == '\\') {
    acStack259[uStack1038] = '\0';
  }
  pass1_1000_4f2e();
  uVar8 = ((u32)paStack774 >> 0x10);
  uStack778 = (u32)((int)paStack774 + 0x12);
  uVar5 = ((int)paStack774 + 0x14);
  if ((uVar5 | uStack778) != 0x0) {
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_202),(char *)(uStack778 & 0xffff | (u32)uVar5 << 0x10));
  }
  local_416[0] = '\0';
  pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x579);
  unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_416),pcVar9);
  uStack1048 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_416));
  uStack1038 = uStack1048;
  for (; -0x1 < (int)uStack1048; uStack1048 -= 0x1) {
    if (local_416[uStack1048] == '.') {
      unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_67e),(char *)CONCAT22(0x1050,local_416 + uStack1048 + 0x1));
      unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_416),(char *)CONCAT22(0x1050,local_67e));
    }
  }
  acStack1305[1] = 0x0;
  pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x74c);
  uVar7 = ((u32)pcVar9 >> 0x10);
  unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,acStack1305 + 0x1),pcVar9);
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(0x1050,acStack1305 + 0x1));
  cStack1306 = acStack1305[uStack1038];
  uStack1048 = 0x0;
  while (acStack1305[uStack1048 + 0x1] != '\0') {
    if (acStack1305[uStack1048 + 0x1] == cStack1306) {
      acStack1305[uStack1048 + 0x1] = '\0';
    }
    uStack1048 += 0x1;
  }
  pass1_1000_4906((StructD *)CONCAT22(0x1050,&ofn),NULL,0x48);
  _ofn = 0x48;
  uVar8 = ((u32)param_2 >> 0x10);
  uStack1374 = ((int)param_2 + 0x8);
  pcStack1370 = acStack1305 + 0x1;
  uStack1368 = SUB42(&DAT_1050_1050,0x0);
  pcStack1354 = (char *)CONCAT22(0x1050,local_202);
  puStack1346 = local_302;
  uStack1344 = SUB42(&DAT_1050_1050,0x0);
  uStack1350 = 0x100;
  uStack1342 = 0x100;
  pcStack1338 = acStack259 + 0x1;
  uStack1336 = SUB42(&DAT_1050_1050,0x0);
  pcStack1322 = local_416;
  uStack1320 = SUB42(&DAT_1050_1050,0x0);
  pcStack1382 = NULL;
  local_666[0] = 0x0;
  if (param_3 == 0x1) {
    uStack1330 = 0x1804;
    pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x74d);
    uVar7 = ((u32)pcVar9 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_666),pcVar9);
    puStack1334 = local_666;
    uStack1332 = SUB42(&DAT_1050_1050,0x0);
    success = GetOpenFileName16(CONCAT22(0x1050,&ofn));
  }
  else {
    if (param_3 != 0x2) {
      debug_print_1008_6048(uVar7,s_Unsupported_FileStructType_in_Op_1050_01ca);
      goto LAB_1008_3461;
    }
    uStack1330 = 0x6;
    pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x74e);
    uVar7 = ((u32)pcVar9 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_666),pcVar9);
    puStack1334 = local_666;
    uStack1332 = SUB42(&DAT_1050_1050,0x0);
    success = GetSaveFileName16(CONCAT22(0x1050,&ofn));
  }
  if (success != 0x0) {
    pcStack1382 = pcStack1354;
  }//
LAB_1008_3461:
  if (pcStack1382 != NULL) {
    local_67e[0] = uStack1326;
    if ((int)uStack1326 < 0x0) {
      pcStack1654 = load_string_1010_847e(_u16_1050_14cc,0x3fd);
      uVar6 = ((u32)pcStack1654 >> 0x10);
      uVar4 = str_op_1008_60e8(uVar6,pcStack1654);
      pcStack1654 = (char *)CONCAT22(uVar6,uVar4);
      pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x57b);
      text = ((u32)pcVar9 >> 0x10);
      type = SUB42(pcVar9,0x0);
      MessageBox16(0x10,pcVar9,pcStack1654,*(HWND16 *)((int)param_2 + 0x8));
      pcStack1382 = NULL;
      pcStack1646 = pcStack1654;
      fn_ptr_1000_17ce(pcStack1654);
    }
    else {
      str_op_1000_3dbe((char *)CONCAT13(0x10,CONCAT12(0x50,local_782)),pcStack1354,uStack1326);
      local_782[uStack1326] = '\0';
      if (local_782[0] != '\0') {
        pass1_1010_60cc(uVar7,paStack774,(char *)CONCAT22(0x1050,local_782));
      }
    }
  }
  pass1_1000_4f2e();
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn set_sys_color_1008_357e(param_1: *mut astruct_53,mut param_2: i16,mut param_3: u32)

{
  let mut uVar1: u16;
  astruct_57 *paVar2;
  astruct_53 *iVar3;
  let mut iVar4: i16;
  astruct_53 *uVar6;
  let mut uVar5: u16;
  COLORREF CVar6;
  let mut uVar7: u32;
  let mut iStack132: i16;
  let mut local_80: u32;
  let mut uStack124: u16;
  let mut uStack122: u16;
  let mut uStack120: u16;
  let mut uStack118: u16;
  let mut uStack116: u16;
  let mut uStack114: u16;
  let mut uStack112: u32;
  let mut uStack108: u32;
  let mut uStack104: u16;
  let mut uStack102: u16;
  let mut uStack100: u16;
  let mut uStack98: u16;
  let mut uStack96: u16;
  let mut uStack94: u16;
  let mut uStack92: u16;
  let mut uStack90: u16;
  let mut uStack88: u32;
  let mut uStack84: u32;
  let mut uStack80: u16;
  let mut uStack78: u16;
  let mut uStack76: u32;
  let mut uStack72: u32;
  let mut uStack68: u32;
  let mut uStack64: u32;
  let mut uStack60: u32;
  let mut uStack56: u32;
  let mut uStack52: u32;
  let mut uStack48: u32;
  let mut local_2c: u32;
  let mut uStack40: u32;
  let mut uStack36: u32;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u32;
  let mut uStack4: u16;

  local_2c = 0x70004;
  uStack40 = 0xf0000;
  uStack36 = 0x100014;
  uStack32 = 0xd0012;
  uStack28 = 0x6000e;
  uStack24 = 0x80005;
  uStack20 = 0x10011;
  uStack16 = 0x30002;
  uStack12 = 0xa0009;
  uStack8 = 0xc000b;
  uStack4 = 0x13;
  local_80 = 0x0;
  uStack108 = 0x808080;
  paVar2 = (astruct_57 *)CONCAT22((int)((u32)param_3 >> 0x10),0x100);
  uStack116 = 0x0;
  uStack114 = 0x100;
  uStack100 = 0x0;
  uStack98 = 0x100;
  uStack96 = 0xffff;
  uStack94 = 0x0;
  uStack124 = 0x2;
  uStack122 = 0x100;
  uStack120 = 0x2;
  uStack118 = 0x100;
  uStack104 = 0x2;
  uStack102 = 0x100;
  uStack92 = 0x2;
  uStack90 = 0x100;
  uStack88 = 0x0;
  uStack80 = 0xc0c0;
  uStack78 = 0x0;
  uStack76 = 0x0;
  uStack72 = 0x0;
  uStack68 = 0x0;
  uStack52 = 0x0;
  uVar1 = 0x8000;
  uStack112 = 0x8000;
  uStack84 = 0x8000;
  uStack64 = 0x8000;
  uStack60 = 0x8000;
  uStack56 = 0x8000;
  uStack48 = 0x8000;
  uVar6 = (astruct_53 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_53 *)param_1;
  if (*(i32 *)&iVar3.field248_0xf8 == 0x0) {
    mem_op_1000_179c(0x54,paVar2);
    iVar3.field248_0xf8 = uVar1;
    iVar3.field249_0xfa = (int)paVar2;
    for (iStack132 = 0x0; iStack132 < 0x15; iStack132 += 0x1) {
      CVar6 = GetSysColor16(*(INT16 *)((int)&local_2c + iStack132 * 0x2));
      uVar7 = (u32)&iVar3.field248_0xf8;
      uVar5 = ((u32)uVar7 >> 0x10);
      iVar4 = (int)uVar7;
      (iVar4 + iStack132 * 0x4) = (int)CVar6;
      (iVar4 + iStack132 * 0x4 + 0x2) = (int)(CVar6 >> 0x10);
    }
  }
  if (param_2 != 0x0) {
    CVar6 = GetSysColor16((INT16)local_2c);
    if (((int)CVar6 == (int)local_80) && ((int)(CVar6 >> 0x10) == local_80)) {
      return;
    }
  }
  if (PTR_LOOP_1050_0010 == NULL) {
    uStack112 = 0xc0c0c0;
  }
  if (param_2 == 0x0) {
    uVar7 = (u32)&iVar3.field248_0xf8;
  }
  else {
    uVar7 = CONCAT22(0x1050,&local_80);
  }
  SetSysColors16((COLORREF *)uVar7,(INT16 *)((u32)uVar7 >> 0x10),(INT16)&local_2c);
  return;
}
pub fn pass1_1008_3714(StructA *param_1)

{
  pass1_1008_3e0e(param_1);
  return;
}



pub fn pass1_1008_372c(mut param_1: i16,mut param_2: u16 ) -> u32

{
  return CONCAT22(param_2,param_1 + 0xa);
}
pub fn pass1_1008_373c(void)

{
  return;
}
pub fn pass1_1008_3740(void)

{
  return;
}
pub fn pass1_1008_3744(void)

{
  return;
}
pub fn pass1_1008_3748(void)

{
  return;
}
pub fn pass1_1008_374c(void)

{
  return;
}
pub fn pass1_1008_3750(void)

{
  return;
}
pub fn pass1_1008_3754(void)

{
  return;
}



u16 pass1_1008_3758(void)

{
  return 0x1;
}
pub fn pass1_1008_375e(void)

{
  return;
}
pub fn pass1_1008_3762(void)

{
  return;
}
pub fn pass1_1008_3766(void)

{
  return;
}
pub fn FUN_1008_376a(void)

{
  return;
}
pub fn FUN_1008_376e(void)

{
  return;
}
pub fn FUN_1008_3772(void)

{
  return;
}
pub fn FUN_1008_3776(void)

{
  return;
}
pub fn pass1_1008_377a(void)

{
  return;
}



u16 * pass1_1008_377e(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_450 * pass1_1008_37aa(param_1: *mut astruct_450,u8 param_2)

{
  astruct_450 *uVar1;
  astruct_450 *uVar2;

  uVar2 = (astruct_450 *)((u32)param_1 >> 0x10);
  uVar1 = (astruct_450 *)param_1;
  param_1.field0_0x0 = 0x380a;
  uVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  uVar1.field1_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



pub fn pass1_1008_37e4(mut param_1: u32,u8 param_2) -> u32

{
  cleanup_ui_op_1008_0618((astruct_53 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_392e(param_1: *mut u16,mut param_2: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa8;
  (iVar1 + 0x2) = 0x1008;
  (iVar1 + 0x4) = param_2;
  *param_1 = 0x3ab0;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa0;
  (iVar1 + 0x2) = 0x1008;
  return param_1;
}
pub fn pass1_1008_397a(param_1: *mut astruct_452)

{
  astruct_452 *iVar1;
  astruct_452 *uVar1;

  uVar1 = (astruct_452 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_452 *)param_1;
  param_1.field0_0x0 = 0x3aa0;
  iVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3ab0;
  iVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  return;
}



// l
pub fn fill_rect_1008_39ac(astruct_930 *in_win_handle_1,mut param_2: u16 )

{
  HBRUSH16 hbrush;
  u8 local_paint_struct [0x20];

  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_paint_struct),in_win_handle_1.field4_0x4);
  hbrush = CreateSolidBrush16(0x210070b);
  GetClientRect16((RECT16 *)&stack0xffd2,(HWND16)&DAT_1050_1050);
  FillRect16(hbrush,(RECT16 *)&stack0xffd2,(HDC16)&DAT_1050_1050);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_paint_struct),in_win_handle_1.field4_0x4);
  DeleteObject16(hbrush);
  return;
}
pub fn pass1_1008_3a10(void)

{
  return;
}



u16 * pass1_1008_3a14(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_3a40(param_1: *mut astruct_451,u8 param_2)

{
  astruct_451 *uVar1;
  astruct_451 *uVar2;

  uVar2 = (astruct_451 *)((u32)param_1 >> 0x10);
  uVar1 = (astruct_451 *)param_1;
  param_1 = 0x3ab0;
  uVar1.field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1.field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return (u16 *)param_1;
}



pub fn pass1_1008_3a7a(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_397a((astruct_452 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_20 * pass1_1008_3ab8(param_1: *mut astruct_20)

{
  astruct_20 *iVar1;
  let mut uVar1: u16;

  set_struct_1008_687a(param_1,0x0);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1.field164_0xde = 0x0;
  param_1.offset_0x0 = 0x3b46;
  iVar1.base_0x2 = 0x1008;
  unk_str_op_1000_3d3e
            ((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1.field60_0x5b)),s_SOLDefaultWindowClass_1050_01fe);
  return param_1;
}
pub fn post_quit_msg_1008_3af4(void)

{
  PostQuitMessage16(0x0);
  return;
}
pub fn pass1_1008_3afe(param_1: *mut u16,u8 param_2)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}
pub fn pass1_1008_3bd6(mut param_1: u32,param_2: *mut astruct_57,param_3: *mut astruct_57,mut param_4: u16 ,mut param_5: u32,mut param_6: u16 ,
                    mut param_7: u32,mut param_8: u32,mut param_9: u16 ,mut param_10: u16 ,mut param_11: u16 ,
                    mut param_12: u16 ,mut param_13: u16 ,mut param_14: u16 )

{
  mixed_struct_op_1040_8fb8
            (param_1,(astruct_65 *)CONCAT22(param_3,param_2),param_4,NULL,param_6,param_7,(param_7 >> 0x10),
             param_8,(param_8 >> 0x10),param_9);
  CONCAT22(param_3,param_2) = 0x3cfc;
  param_2.field1_0x2 = 0x1008;
  &param_2.field_0x36 = 0x0;
  (u32)&param_2.field21_0x26 = 0x0;
  pass1_1040_9252((astruct_65 *)CONCAT22(param_3,param_2));
  create_window_1040_92dc((astruct_65 *)CONCAT22(param_3,param_2));
  mov_update_win_1040_93aa((astruct_65 *)CONCAT22(param_3,param_2),(INT16)param_5,(param_5 >> 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1008_3c34(mut param_1: u32,u8 param_2,HDC16 hdc_param_3)

{
  let mut uVar1: u16;
  code **ppcVar2;
  HPALETTE16 HVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u32 *puStack6;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (((iVar4 + 0xa) | (iVar4 + 0x8)) != 0x0) {
    puStack6 = (u32*)(iVar4 + 0x8);
    if ((*(i32 *)(iVar4 + 0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack6 = (u32*)(iVar4 + 0xc);
    }
    if ((*(i32 *)(iVar4 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = (u32*)(iVar4 + 0x10);
    }
    uVar6 = ((u32)_PTR_LOOP_1050_4230 >> 0x10);
    uVar1 = ((int)_PTR_LOOP_1050_4230 + 0x10);
    HVar3 = palette_op_1008_4e08
                      ((HPALETTE16)&hdc_param_3,uVar1,
                       (astruct_13 *)CONCAT22(uVar1,((int)_PTR_LOOP_1050_4230 + 0xe)),
                       (HDC16 *)CONCAT22(0x1050,&hdc_param_3));
    ppcVar2 = (code **)((int)*puStack6 + 0x4);
    (**ppcVar2)();
    HVar3 = SelectPalette16(0x0,HVar3,hdc_param_3);
    DeleteObject16(HVar3);
  }
  return;
}
pub fn FUN_1008_3cd2(void)

{
  return;
}



StructD * pass1_1008_3cd6(StructD *param_1,u8 param_2)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn post_msg_1008_3d20(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  PostMessage16(0x0,*(WPARAM16 *)((int)param_1 + 0xcc),0x111,*(HWND16 *)((int)param_1 + 0xbc));
  return;
}
pub fn FUN_1008_3d40(void)

{
  return;
}



u16 * pass_1008_3d44(param_1: *mut astruct_453,u8 param_2)

{
  astruct_453 *uVar1;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  uVar1 = (astruct_453 *)param_1;
  param_1.field0_0x0 = 0x380a;
  uVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  uVar1.field1_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return &param_1.field0_0x0;
}
pub fn pass1_1008_3e0e(StructA *param_1)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x4);
    (**ppcVar1)();
  }
  return;
}



u16 * pass1_1008_3e38(param_1: *mut astruct_19)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1.offset_0x0 = 0x0;
  ((int)param_1 + 0x2) = 0x0;
  ((int)param_1 + 0x4) = 0x0;
  return &param_1.offset_0x0;
}



u16 * pass1_1008_3e54(param_1: *mut u16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = param_4;
  ((int)param_1 + 0x2) = param_3;
  ((int)param_1 + 0x4) = param_2;
  return param_1;
}
pub fn pass1_1008_3e76(param_1: *mut u16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = param_4;
  ((int)param_1 + 0x2) = param_3;
  ((int)param_1 + 0x4) = param_2;
  return;
}
pub fn pass1_1008_3e94(param_1: *mut u16,param_2: *mut u16,char *param_3)

{
  param_3 = *param_1;
  *param_2 = ((int)param_1 + 0x2);
  return;
}
pub fn pass1_1008_3eb4(param_1: *mut astruct_615,param_2: *mut u16,param_3: *mut u16,param_4: *mut u16)

{
  let mut uVar1: u16;

  *param_4 = param_1;
  uVar1 = ((u32)param_1 >> 0x10);
  *param_3 = ((int)param_1 + 0x2);
  *param_2 = ((int)param_1 + 0x4);
  return;
}
pub fn pass1_1008_3ee2(i16 *param_1,i16 *param_2)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  iVar1 = *param_2 - *param_1;
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *param_1 = iVar1 + 0x1;
  uVar3 = ((u32)param_2 >> 0x10);
  uVar4 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  iVar1 = ((int)param_2 + 0x2) - (iVar2 + 0x2);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  (iVar2 + 0x2) = iVar1 + 0x1;
  iVar1 = ((int)param_2 + 0x4) - (iVar2 + 0x4);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  (iVar2 + 0x4) = iVar1 + 0x1;
  return;
}
pub fn pass1_1008_3f32(i16 *param_1,i16 *param_2)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;

  *param_1 = *param_1 + *param_2;
  uVar2 = ((u32)param_2 >> 0x10);
  uVar3 = ((u32)param_1 >> 0x10);
  piVar1 = (i16 *)((int)param_1 + 0x2);
  *piVar1 = *piVar1 + ((int)param_2 + 0x2);
  piVar1 = (i16 *)((int)param_1 + 0x4);
  *piVar1 = *piVar1 + ((int)param_2 + 0x4);
  return;
}
pub fn pass1_1008_3f62(param_1: *mut u16,param_2: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  *param_1 = *param_2;
  uVar1 = ((u32)param_2 >> 0x10);
  uVar2 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x2) = ((int)param_2 + 0x2);
  ((int)param_1 + 0x4) = ((int)param_2 + 0x4);
  return;
}
pub fn struct_op_1008_3f92(param_1: *mut astruct_76,char *param_2)

{
  code **ppcVar1;
  astruct_76 *iVar2;
  let mut uVar2: u16;

  struct_op_1008_56b4(param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  (u32)&iVar2.field3_0x6 = 0x0;
  (u32)&iVar2.field5_0xa = 0x0;
  iVar2.field7_0xe = 0x0;
  iVar2.field8_0x10 = 0x0;
  (u32)&iVar2.field9_0x14 = 0x0;
  (u32)&iVar2.field11_0x18 = 0x0;
  iVar2.field13_0x1c = 0x0;
  param_1.offset_0x0 = &PTR_LOOP_1050_48de;
  iVar2.base_0x2 = 0x1008;
  if (param_2 == NULL) {
    return;
  }
  ppcVar1 = (code **)((int)(u32)param_2 + 0x8);
  (**ppcVar1)();
  struct_op_1008_4214(param_1,(astruct_81 *)param_2);
  pass1_1008_47cc(param_1);
  pass1_1008_4834(param_1);
  return;
}
pub fn pass1_1008_4016(param_1: *mut astruct_76)

{
  astruct_76 *iVar1;
  let mut uVar1: u16;

  struct_op_1008_56b4(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  (u32)&iVar1.field3_0x6 = 0x0;
  (u32)&iVar1.field5_0xa = 0x0;
  iVar1.field7_0xe = 0x0;
  iVar1.field8_0x10 = 0x0;
  (u32)&iVar1.field9_0x14 = 0x0;
  (u32)&iVar1.field11_0x18 = 0x0;
  iVar1.field13_0x1c = 0x0;
    // just 0x48de
  param_1.offset_0x0 = &PTR_LOOP_1050_48de;
  iVar1.base_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_405c(param_1: *mut astruct_76,mut param_2: u32,mut param_3: i16,mut param_4: i16)

{
  let mut uVar1: u32;
  sqword sVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  i32 lVar5;
  u8 *puVar6;
  astruct_76 *iVar4;
  let mut uVar7: u16;
  let mut uStack10: u32;

  struct_op_1008_56b4(param_1);
  uVar7 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_76 *)param_1;
  (u32)&iVar4.field3_0x6 = 0x0;
  (u32)&iVar4.field5_0xa = 0x0;
  iVar4.field7_0xe = 0x0;
  iVar4.field8_0x10 = 0x0;
  (u32)&iVar4.field9_0x14 = 0x0;
  (u32)&iVar4.field11_0x18 = 0x0;
  iVar4.field13_0x1c = 0x0;
  param_1.offset_0x0 = &PTR_LOOP_1050_48de;
  iVar4.base_0x2 = 0x1008;
  iVar3 = param_4 * 0x8 + 0x1f;
  uVar4 = ((int)(iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  uStack10 = (u32)param_3;
  lVar5 = (long)(int)uVar4 * (long)param_3 + 0x436;
  lVar5 = mem_op_1000_0a48(0x1,lVar5,(int)((u32)lVar5 >> 0x10),_PTR_LOOP_1050_5f2c);
  iVar4.field3_0x6 = (int)lVar5;
  iVar4.field4_0x8 = ((u32)lVar5 >> 0x10);
  pass1_1008_47cc((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar7 << 0x10));
  iVar4.field11_0x18 = uVar4;
  iVar4.field12_0x1a = (int)uVar4 >> 0xf;
  (u32)iVar4.field8_0x10 = 0x28;
  uVar1 = iVar4.field8_0x10;
  *(i32 *)((int)uVar1 + 0x4) = (long)param_4;
  uVar1 = iVar4.field8_0x10;
  (u32)((int)uVar1 + 0x8) = uStack10;
  uVar1 = iVar4.field8_0x10;
  ((int)uVar1 + 0xc) = 0x1;
  uVar1 = iVar4.field8_0x10;
  ((int)uVar1 + 0xe) = 0x8;
  uVar1 = iVar4.field8_0x10;
  (u32)((int)uVar1 + 0x10) = 0x0;
  sVar2 = (qword)(u32)&iVar4.field11_0x18 * (qword)uStack10;
  puVar6 = (u8 *)((qword)sVar2 >> 0x20);
  uVar1 = iVar4.field8_0x10;
  (u32)((int)uVar1 + 0x14) = (long)sVar2;
  uVar1 = iVar4.field8_0x10;
  (u32)((int)uVar1 + 0x20) = 0x100;
  uVar1 = iVar4.field8_0x10;
  (u32)((int)uVar1 + 0x24) = 0x100;
  pass1_1008_4834(param_1);
  pass1_1008_4d84(puVar6,*(astruct_90 **)&iVar4.field5_0xa,param_2);
  return;
}
pub fn pass1_1008_41bc(param_1: *mut astruct_288)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_288 *iVar5;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_288 *)param_1;
  param_1 = (int)&PTR_LOOP_1050_48de;
  iVar5.field2_0x2 = 0x1008;
  puVar1 = iVar5.field6_0xa;
  uVar2 = iVar5.field7_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (iVar5.field5_0x6 != 0x0) {
    call_fn_ptr_1000_0dc6((char *)iVar5.field5_0x6);
  }
  param_1 = 0x389a;
  iVar5.field2_0x2 = 0x1008;
  return;
}
pub fn struct_op_1008_4214(param_1: *mut astruct_76,param_2: *mut astruct_81)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_81 *iVar4;
  astruct_81 *uVar4;

  uVar4 = (astruct_81 *)((u32)param_2 >> 0x10);
  iVar4 = (astruct_81 *)param_2;
  (u32)((int)param_1 + 0x6) = iVar4.buffer_0x1a;
  iVar4.buffer_0x1a = 0x0;
  puVar1 = (u32*)&iVar4.field2_0x4;
  uVar2 = ((int)&iVar4.field2_0x4 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4.field2_0x4 = 0x0;
  iVar4.field6_0xe = 0x0;
  iVar4.field7_0x12 = 0x0;
  iVar4.field8_0x16 = 0x0;
  iVar4.field10_0x1e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn memcpy_op_1008_4274(mut param_1: u16 ,param_2: *mut astruct_826)

{
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut uVar2: u32;
  astruct_826 *iVar3;
  astruct_827 *uVar3;
  let mut uVar4: u16;
  let mut count: u32;
pub fn *dst;
  astruct_76 *paStack14;
  astruct_57 *paVar2;

  uVar5 = ((u32)in_EDX >> 0x10);
  uVar4 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_826 *)param_2;
  if (iVar3.pvoid32_0x6 != NULL) {
    count = pass1_1000_1284((u32)iVar3.pvoid32_0x6);
    dst = (void *)mem_op_1000_0a48(0x1,count,(int)(count >> 0x10),_PTR_LOOP_1050_5f2c);
    uVar3 = (astruct_827 *)dst;
    uVar1 = ((u32)dst >> 0x10) | uVar3;
    paVar2 = (astruct_57 *)CONCAT22(uVar5,uVar1);
    if (uVar1 != 0x0) {
      hmemcpy16(count,iVar3.pvoid32_0x6,dst);
      mem_op_1000_179c(0x1e,paVar2);
      uVar2._0_2_ = paVar2 | uVar3;
      if (uVar2 == 0x0) {
        uVar3 = NULL;
        uVar2._0_2_ = 0x0;
      }
      else {
        pass1_1008_4016((astruct_76 *)CONCAT22(paVar2,uVar3));
      }
      paStack14 = (astruct_76 *)CONCAT22(uVar2,uVar3);
      uVar3.field6_0x6 = dst;
      pass1_1008_47cc((astruct_76 *)CONCAT22(uVar2,uVar3));
      pass1_1008_4834(paStack14);
      uVar3.field25_0x1c = 0x1;
      return;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1008_431c(param_1: *mut astruct_76,u8 param_2)

{
  u32 *puVar1;
  let mut uVar2: u32;
  let mut bVar3: bool;
  let mut uVar4: u32;
  astruct_76 *iVar5;
  astruct_76 *uVar5;
  let mut uStack6: u32;

  uVar5 = (astruct_76 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar5.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)((u32)param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  if ((iVar5.field4_0x8 | iVar5.field3_0x6) == 0x0) {
    bVar3 = false;
  }
  else {
    if ((iVar5.field6_0xc | iVar5.field5_0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)((u32)param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
    }
    bVar3 = true;
  }
  if (bVar3) {
    if ((iVar5.field10_0x16 | iVar5.field9_0x14) == 0x0) {
      return;
    }
    uStack6 = 0x0;
    while( true ) {
      uVar2 = iVar5.field8_0x10;
      puVar1 = (u32 *)((int)uVar2 + 0x8);
      if (*puVar1 == uStack6 || (long)*puVar1 < (long)uStack6) break;
      uVar4 = uStack6;
      pass1_1008_4544(param_1);
      uVar2 = iVar5.field8_0x10;
      pass1_1000_4906((StructD *)(uVar4 & 0xffff | (u32)uStack6 << 0x10),(WNDCLASS16 *)param_2,
                      ((int)uVar2 + 0x4));
      uStack6 += 0x1;
    }
  }
  return;
}



pub fn pass1_1008_43cc(param_1: *mut astruct_76) -> u32

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2.field5_0xa == 0x0) {
      pass1_1008_4834((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2.field10_0x16,iVar2.field9_0x14);
}



pub fn pass1_1008_4426(param_1: *mut astruct_76) -> u32

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  astruct_76 *uVar2;

  uVar2 = (astruct_76 *)((u32)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)((u32)param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
  }
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2.field5_0xa == 0x0) {
      pass1_1008_4834((astruct_76 *)((u32)param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2.field6_0xc,iVar2.field5_0xa);
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1008_4480(param_1: *mut astruct_76,param_2: *mut u16,param_3: *mut astruct_76)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iStack26: i16;
  char *pcStack24;
  char *pcStack20;
  let mut iStack16: i16;
  let mut local_6: i16;
  char local_4 [0x2];

  pass1_1008_3e94(param_2,(u16 *)CONCAT22(0x1050,&local_6),(char *)CONCAT22(0x1050,local_4));
  uVar7 = pass1_1008_4772(param_3);
  uVar5 = (uVar7 >> 0x10);
  iVar1 = ((int)uVar7 + 0x4);
  iVar2 = ((int)uVar7 + 0x8);
  for (iStack16 = 0x0; iStack16 < iVar2; iStack16 += 0x1) {
    uVar6 = local_6 >> 0xf;
    iVar3 = local_6;
    local_6 = local_6 + 0x1;
    pass1_1008_4544(param_1);
    pcStack20 = (char *)CONCAT22(uVar6,iVar3);
    uVar4 = (u32)iStack16;
    pass1_1008_4544(param_3);
    pcStack24 = (char *)(uVar4 & 0xffff | (u32)uVar6 << 0x10);
    iStack26 = iVar1;
    while (iStack26 != 0x0) {
      if (*pcStack24 != -0x1) {
        *pcStack20 = *pcStack24;
      }
      pcStack24 = (char *)CONCAT22((int)((u32)pcStack24 >> 0x10) + (-(0xfffe < pcStack24) & 0x6c),
                                   pcStack24 + 0x1);
      pcStack20 = (char *)CONCAT22((int)((u32)pcStack20 >> 0x10) + (-(0xfffe < pcStack20) & 0x6c),
                                   pcStack20 + 0x1);
      iStack26 = iStack26 + -0x1;
    }
  }
  return;
}
pub fn pass1_1008_4544(param_1: *mut astruct_76)

{
  let mut bVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar3 << 0x10));
  }
  if (*(i32 *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn set_di_bits_to_device_1008_45d6(param_1: *mut astruct_76,INT16 param_2,HDC16 param_3)

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  BITMAPINFO *info;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut startscan: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc(param_1);
  }
  if ((iVar2.field4_0x8 | iVar2.field3_0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if ((iVar2.field6_0xc | iVar2.field5_0xa) == 0x0) {
      pass1_1008_4834(param_1);
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  uVar1 = iVar2.field8_0x10;
  uVar4 = (uVar1 >> 0x10);
  info = (BITMAPINFO *)uVar1;
  startscan = &(info.bim_header).biHeight;
  uVar2 = (u32)&iVar2.field9_0x14;
  SetDIBitsToDevice(0x0,info,(void *)CONCAT22((int)uVar2,uVar4),((u32)uVar2 >> 0x10),startscan,0x0,0x0,0x0,
                    startscan,*(INT16 *)&(info.bim_header).biWidth,param_2,param_3);
  return;
}



// WARNING: Unable to use type for symbol uVar1
pub fn stretch_di_bits_1008_465a(param_1: *mut astruct_76,HDC16 hdc_param_2)

{
  INT16 x_src;
  INT16 y_src;
  let mut uVar2: u32;
  let mut bVar3: bool;
  astruct_76 *iVar3;
  BITMAPINFO *info;
  let mut uVar4: u16;
  PVOID bits;
  let mut uVar1: u32;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar3.field3_0x6 == 0x0) {
    pass1_1008_47cc(param_1);
  }
  if ((iVar3.field4_0x8 | iVar3.field3_0x6) == 0x0) {
    bVar3 = false;
  }
  else {
    if ((iVar3.field6_0xc | iVar3.field5_0xa) == 0x0) {
      pass1_1008_4834(param_1);
    }
    bVar3 = true;
  }
  if (!bVar3) {
    return;
  }
  uVar1 = iVar3.field8_0x10;
  bits = (PVOID)(uVar1 >> 0x10);
  info = (BITMAPINFO *)uVar1;
  x_src = *(INT16 *)&(info.bim_header).biWidth;
  y_src = *(INT16 *)&(info.bim_header).biHeight;
  uVar2 = (u32)&iVar3.field9_0x14;
  StretchDIBits16(0xcc0020,0x0,info,bits,(INT16)uVar2,(INT16)((u32)uVar2 >> 0x10),y_src,x_src,0x0,0x0,y_src,x_src,
                  hdc_param_2);
  return;
}



u16 palette_op_1008_46e4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_76,HDC16 *param_4)

{
  let mut bVar1: bool;
  let mut uVar2: u16;
  HPALETTE16 HVar2;
  let mut uVar5: u16;
  astruct_76 *struct_var3;
  let mut uVar4: u16;
  let mut uVar3: u32;

  uVar4 = ((u32)param_3 >> 0x10);
  struct_var3 = (astruct_76 *)param_3;
  if (*(i32 *)&struct_var3.field3_0x6 == 0x0) {
    uVar5 = param_2;
    pass1_1008_47cc((astruct_76 *)((u32)param_3 & 0xffff | (u32)uVar4 << 0x10));
    param_2 = uVar5;
  }
  uVar3 = CONCAT22(param_2,param_1);
  if (*(i32 *)&struct_var3.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&struct_var3.field5_0xa == 0x0) {
      uVar3 = pass1_1008_4834((astruct_76 *)((u32)param_3 & 0xffff | (u32)uVar4 << 0x10));
    }
    bVar1 = true;
  }
  uVar2 = uVar3;
  if (!bVar1) {
    return 0x0;
  }
  create_palette_1008_4e38(*(astruct_13 **)&struct_var3.field5_0xa,(uVar3 >> 0x10));
  struct_var3.field7_0xe = uVar2;
  HVar2 = SelectPalette16(0x0,struct_var3.field7_0xe,*param_4);
  struct_var3.field2_0x4 = HVar2;
  RealizePalette16(*param_4);
  return struct_var3.field2_0x4;
}



pub fn pass1_1008_4772(param_1: *mut astruct_76) -> u32

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2.field5_0xa == 0x0) {
      pass1_1008_4834((astruct_76 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(((int)&iVar2.field8_0x10 + 0x2),&iVar2.field8_0x10);
}
pub fn pass1_1008_47cc(param_1: *mut astruct_76)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u16;
  astruct_76 *iVar5;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uStack14: u32;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar5.field3_0x6 != 0x0) {
    uVar2 = (u32)&iVar5.field3_0x6;
    uVar1 = iVar5.field4_0x8;
    iVar6 = (int)uVar2;
    uVar4 = iVar6 + 0xe;
    iVar5.field8_0x10 = uVar2 & 0xffff0000 | (u32)uVar4;
    iVar5.field9_0x14 = iVar6 + 0x436;
    iVar5.field10_0x16 = uVar1 + (-(0xfbd7 < uVar4) & 0x6c);
    uVar3 = iVar5->field8_0x10;
    uVar8 = (uVar3 >> 0x10);
    iVar6 = (int)uVar3;
    uStack14 = (u32)(iVar6 + 0xe);
    *(i32 *)&iVar5->field11_0x18 = (long)(uStack14 * *(i32 *)(iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
  }
  return;
}
pub fn pass1_1008_4834(param_1: *mut astruct_76)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar5;
  astruct_76 *struct_var5_1;
  astruct_76 *struct_var5;
  astruct_76 *paStack10;

  struct_var5 = (astruct_76 *)((u32)param_1 >> 0x10);
  struct_var5_1 = (astruct_76 *)param_1;
  puVar2 = (u32 *)struct_var5_1->field5_0xa;
  uVar4 = struct_var5_1->field6_0xc;
  paVar5 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar4);
  if ((uVar4 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x14,paVar5);
  paStack10 = (astruct_76 *)CONCAT22(paVar5,puVar2);
  uVar4 = paVar5 | puVar2;
  if (uVar4 != 0x0) {
    uVar3 = struct_var5_1->field8_0x10;
    uVar3 = uVar3 & 0xffff0000 | (u32)((int)uVar3 + 0x28);
    struct_op_1008_4c98(paStack10,uVar3,0x100);
    struct_var5_1->field5_0xa = uVar3;
    struct_var5_1->field6_0xc = uVar4;
    return;
  }
  (u32)&struct_var5_1->field5_0xa = 0x0;
  return;
}



u16 pass1_1008_48aa(mut param_1: u32)

{
  return ((int)param_1 + 0xe);
}



StructD * pass1_1008_48b8(StructD *param_1,u8 param_2)

{
  pass1_1008_41bc((astruct_288 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10084942) overlaps instruction at (ram,0x10084941)
//
pub fn pass1_1008_48de(param_1: *mut u16,mut param_2: i16,char param_3,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32,mut param_7: u16 ,
                    uchar param_8)

{
  u8 *pbVar1;
  let mut uVar2: u32;
  u8 bVar3;
  let mut uVar4: u16;
  u8 bVar5;
  let mut uVar6: u16;
  let mut unaff_BP: i16;
  u8 *puVar7;
  let mut unaff_SI: i16;
  let mut iVar8: i16;
  u8 *unaff_DI;
  let mut unaff_ES: u16;
  let mut uVar9: u16;

  uVar6 = param_4 & 0xff | (u8)((char)(param_4 >> 0x8) + (char)param_4 + param_3) << 0x8;
  puVar7 = (u8 *)(unaff_BP + 0x1);
  pbVar1 = (u8 *)((int)param_1 + unaff_SI);
  bVar5 = (u8)(param_4 & 0xff);
  *pbVar1 = *pbVar1 | bVar5;
  bVar3 = in(0x46);
  pbVar1 = (u8 *)((int)param_1 + unaff_SI);
  *pbVar1 = *pbVar1 | bVar5;
  if (param_2 == 0x1) {
    pbVar1 = (u8 *)((int)param_1 + unaff_SI);
    *pbVar1 = *pbVar1 | bVar5;
    iVar8 = unaff_SI + 0x1;
    pbVar1 = (u8 *)((int)param_1 + iVar8);
    bVar5 = (u8)param_7;
    *pbVar1 = *pbVar1 | bVar5;
    pbVar1 = (u8 *)((int)param_1 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    *unaff_DI = bVar3;
    pbVar1 = (u8 *)((int)param_1 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    uVar6 = param_7;
    if (*pbVar1 != 0x0) {
      pbVar1 = (u8 *)((int)param_1 + iVar8);
      *pbVar1 = *pbVar1 | bVar5;
      puVar7 = (u8 *)((int)&param_7 + 0x1);
      param_1 = (u16 *)(param_6 >> 0x8);
      CONCAT13(param_8,param_6._1_3_) = 0x389a;
      param_1[0x1] = 0x1008;
      unaff_ES = (CONCAT13(param_8,param_6._1_3_) >> 0x10);
      (u32)(param_1 + 0x2) = 0x0;
      (u32)(param_1 + 0x4) = 0x0;
      param_1[0x6] = 0xffff;
      (u32)(param_1 + 0x7) = 0x0;
      (u32)(param_1 + 0x9) = 0x0;
      (u32)(param_1 + 0xb) = 0x0;
      (u32)(param_1 + 0xd) = 0x0;
      param_1[0xf] = 0x0;
    }
  }
  else {
    param_1[0x11] = bVar3 | 0x800;
  }
  param_1[0x11] = (puVar7 + 0xa);
  *param_1 = &u16_1050_4c4c;
  param_1[0x1] = 0x1008;
  uVar4 = str_op_1008_60e8(uVar6,*(char **)(puVar7 + 0xc));
  uVar2 = (u32)(puVar7 + 0x6);
  uVar9 = ((u32)uVar2 >> 0x10);
  iVar8 = (int)uVar2;
  (iVar8 + 0x8) = uVar4;
  (iVar8 + 0xa) = uVar6;
  return;
}
pub fn struct_op_1008_48fe(param_1: *mut astruct_57,param_2: *mut astruct_81,mut param_3: u16 ,char *param_4)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_81 *pstruct81_2;
  let mut uVar3: u16;

  uVar2 = param_1;
  uVar3 = ((u32)param_2 >> 0x10);
  pstruct81_2 = (astruct_81 *)param_2;
  param_2->field0_0x0 = 0x389a;
  pstruct81_2->field1_0x2 = 0x1008;
  pstruct81_2->field2_0x4 = 0x0;
  (u32)&pstruct81_2->field3_0x8 = 0x0;
  pstruct81_2->hfile_0xc = 0xffff;
  pstruct81_2->field6_0xe = 0x0;
  pstruct81_2->field7_0x12 = 0x0;
  pstruct81_2->field8_0x16 = 0x0;
  pstruct81_2->buffer_0x1a = 0x0;
  pstruct81_2->field10_0x1e = 0x0;
  pstruct81_2->field13_0x22 = param_3;
    // just 0x4c4c
  param_2->field0_0x0 = &u16_1050_4c4c;
  pstruct81_2->field1_0x2 = 0x1008;
  uVar1 = str_op_1008_60e8(uVar2,param_4);
  pstruct81_2->field3_0x8 = uVar1;
  pstruct81_2->field4_0xa = uVar2;
  return;
}
pub fn close_file_1008_496c(param_1: *mut astruct_803)

{
  code **ppcVar1;
  astruct_803 *iVar5;
  let mut uVar2: u16;
  u32 *puVar1;
  let mut uVar1: u16;
  i32 lVar1;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_803 *)param_1;
  param_1->offset_0x0 = &u16_1050_4c4c;
  iVar5->base_0x2 = 0x1008;
  puVar1 = iVar5->field2_0x4;
  uVar1 = iVar5->field3_0x6;
  if ((uVar1 | puVar1) != 0x0) {
    ppcVar1 = (code **)*puVar1;
    (**ppcVar1)();
  }
  fn_ptr_1000_17ce((char *)iVar5->field4_0x8);
  if (iVar5->field18_0x1a != 0x0) {
    call_fn_ptr_1000_0dc6((char *)iVar5->field18_0x1a);
  }
  if (iVar5->field5_0xc != 0xffff) {
    _lclose16(iVar5->field5_0xc);
  }
  param_1->offset_0x0 = 0x389a;
  iVar5->base_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 read_file_1008_49e8(HFILE16 param_1,mut param_2: u16 ,astruct_81 *struct_param_1,mut param_4: u32)

{
  let mut uVar4: u32;
  HFILE16 hfile_1;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar1: u32;
  let mut lVar5: i16;
  u8 *puVar5;
  u8 *extraout_DX;
  let mut uVar8: u16;
  astruct_57 *paVar7;
  astruct_81 *struct_1;
  let mut unaff_DI: i16;
  let mut uVar5: u16;
  let mut unaff_SS: u16;
  i32 iVar9;
  i32 lVar10;
  let mut uStack20: u16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  i32 iStack6;
  let mut in_stack_0000ffe8: i16;
  let mut uVar3: u32;
  let mut in_stack_0000ffea: u16;
  let mut in_stack_0000ffe8_00: u16;
  let mut uVar6: u32;

  uVar8 = ((u32)param_4 >> 0x10);
  uVar5 = ((u32)struct_param_1 >> 0x10);
  struct_1 = (astruct_81 *)struct_param_1;
  if (*(i32 *)&struct_1->field3_0x8 != 0x0) {
    if (struct_1->field10_0x1e != 0x0) {
      return 0x1;
    }
    if (struct_1->hfile_0xc == -0x1) {
      hfile_1 = _lopen16(0x0,*(char **)&struct_1->field3_0x8);
      struct_1->hfile_0xc = hfile_1;
      if (hfile_1 == 0xffff) {
        return 0x0;
      }
    }
    iStack6 = 0x0;
    iVar9 = WIN16_hread(0xe,(void *)CONCAT22(0x1050,&param_1),struct_1->hfile_0xc);
    if ((((int)iVar9 == 0xe) && ((int)((u32)iVar9 >> 0x10) == 0x0)) &&
       (iStack6 = CONCAT22(uStack20,param_2), param_1 == (int)&PTR_LOOP_1050_4d42)) {
      _llseek16(0x0,0x0,struct_1->hfile_0xc);
      lVar10 = mem_op_1000_0a48(0x1,iStack6,(int)((u32)iStack6 >> 0x10),_PTR_LOOP_1050_5f2c);
      lVar5 = (i16)((u32)lVar10 >> 0x10);
      &struct_1->buffer_0x1a = (int)lVar10;
      ((int)&struct_1->buffer_0x1a + 0x2) = lVar5;
      if ((lVar5 | &struct_1->buffer_0x1a) != 0x0) {
        iVar9 = WIN16_hread(iStack6,(void *)struct_1->buffer_0x1a,struct_1->hfile_0xc);
        uStack8 = ((u32)iVar9 >> 0x10);
        paVar7 = (astruct_57 *)CONCAT22(uVar8,uStack8);
        uStack10 = iVar9;
        param_1 = struct_1->hfile_0xc;
        _lclose16(param_1);
        struct_1->hfile_0xc = 0xffff;
        struct_1->field10_0x1e = 0x1;
        struct_1->field6_0xe = struct_1->buffer_0x1a;
        uVar3 = struct_1->buffer_0x1a;
        iVar1 = (i16)uVar3;
        iVar1 = iVar1 + 0xe;
        struct_1->field7_0x12 = uVar3 & 0xffff0000 | (u32)iVar1;
        uVar1._0_2_ = iVar1 + 0x436;
        uVar1 = uVar3 & 0xffff0000 | (u32)uVar1;
        struct_1->field8_0x16 = uVar1;
        param_2 = 0x14;
        param_1 = (HFILE16)s_tile2_bmp_1050_1538;
        mem_op_1000_179c(0x14,paVar7);
        puVar5 = (u8 *)(paVar7 | uVar1);
        extraout_DX = puVar5;
        if (puVar5 == NULL) {
          struct_1->field2_0x4 = 0x0;
        }
        else {
          param_2 = 0x100;
          uVar4 = struct_1->field7_0x12;
          uVar2 = uVar4;
          uVar2 = uVar2 + 0x28;
          uVar4 &= 0xffff0000;
          uVar6 = uVar4 | uVar2;
          param_1 = (HFILE16)(uVar4 >> 0x10);
          struct_op_1008_4c98((astruct_76 *)(uVar1 & 0xffff | (u32)uVar2 << 0x10),uVar6,0x100);
          &struct_1->field2_0x4 = (int)uVar6;
          *(u8 **)((int)&struct_1->field2_0x4 + 0x2) = extraout_DX;
        }
        if (struct_1->field13_0x22 == 0x0) {
          return 0x1;
        }
        _param_1 = struct_param_1;
        pass1_1008_4b8e(extraout_DX,(astruct_807 *)struct_param_1);
        return 0x1;
      }
    }
    else {
      _lclose16(struct_1->hfile_0xc);
      struct_1->hfile_0xc = 0xffff;
    }
  }
  return 0x0;
}



pub fn pass1_1008_4b5e(u32 *param_1) -> u32

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if ((iVar3 + 0x1e) == 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x8);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0x0) {
      return 0x0;
    }
  }
  return CONCAT22((iVar3 + 0x6),(iVar3 + 0x4));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_4b8e(u8 *param_1,param_2: *mut astruct_807)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut uVar2: u16;
  u32 *puVar3;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe4: u32;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut iStack10: i16;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)in_stack_0000ffe4 >> 0x10),0x48),in_stack_0000fe8e,
                           in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = ((u32)puVar3 >> 0x10);
  uVar1 = (u32)((int)puVar3 + 0x18);
  iStack18 = ((int)puVar3 + 0x16) / 0x2;
  for (iStack16 = 0x0; iStack10 = (int)uVar1, uVar2 = ((u32)param_2 >> 0x10), iStack16 < iStack18;
      iStack16 += 0x1) {
    pass1_1008_4d26(*(astruct_650 **)((int)param_2 + 0x4),
                    (u16 *)(uVar1 & 0xffff0000 | (u32)(iStack16 * 0x4 + iStack10)),iStack16);
  }
  for (iStack18 = 0x100 - iStack18; iStack18 < 0x100; iStack18 += 0x1) {
    pass1_1008_4d26(*(astruct_650 **)((int)param_2 + 0x4),
                    (u16 *)(uVar1 & 0xffff0000 | (u32)(iStack16 * 0x4 + iStack10)),iStack18);
    iStack16 += 0x1;
  }
  return;
}



astruct_803 * file_1008_4c26(param_1: *mut astruct_803,u8 param_2)

{
  close_file_1008_496c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_1008_4c58(param_1: *mut astruct_394)

{
  astruct_394 *iVar1;
  let mut in_stack_00000006: u16;

  *_param_1 = 0x389a;
  param_1->field2_0x2 = 0x1008;
  param_1->field3_0x4 = 0x0;
  param_1->field8_0xc = 0x0;
  param_1->field9_0xe = 0x0;
  param_1->field10_0x12 = 0x1;
  *_param_1 = 0x4f1c;
  param_1->field2_0x2 = 0x1008;
  return;
}
pub fn struct_op_1008_4c98(param_1: *mut astruct_76,mut param_2: u32,mut param_3: u16 )

{
  astruct_76 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->base_0x2 = 0x1008;
  (u32)&iVar1->field2_0x4 = param_2;
  iVar1->field6_0xc = param_3;
  (u32)&iVar1->field7_0xe = 0x0;
  ((int)&iVar1->field8_0x10 + 0x2) = 0x0;
  param_1->offset_0x0 = 0x4f1c;
  iVar1->base_0x2 = 0x1008;
  return;
}
pub fn pass1_1008_4cdc(param_1: *mut astruct_454)

{
  astruct_454 *iVar2;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_454 *)param_1;
  param_1 = 0x4f1c;
  iVar2->field2_0x2 = 0x1008;
  fn_ptr_1000_17ce((char *)iVar2->field10_0xe);
  if (iVar2->field11_0x12 != 0x0) {
    fn_ptr_1000_17ce((char *)iVar2->field3_0x4);
  }
  param_1 = 0x389a;
  iVar2->field2_0x2 = 0x1008;
  return;
}



u16 pass1_1008_4d26(param_1: *mut astruct_650,param_2: *mut u16,mut param_3: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  i32 lVar3;
  astruct_650 *iVar5;
  astruct_649 *iVar4;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_650 *)param_1;
  if (((iVar5->field4_0x4 != 0x0) && (-0x1 < param_3)) &&
     (piVar1 = &iVar5->field9_0xc, *piVar1 != param_3 && param_3 <= *piVar1)) {
    uVar2 = ((int)param_2 + 0x2);
    lVar3 = iVar5->field4_0x4;
    uVar4 = ((u32)lVar3 >> 0x10);
    iVar4 = (astruct_649 *)lVar3;
    (iVar4 + param_3 * 0x4) = *param_2;
    (iVar4 + param_3 * 0x4 + 0x2) = uVar2;
    return 0x1;
  }
  return 0x0;
}



pub fn pass1_1008_4d72(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x6),((int)param_1 + 0x4));
}
pub fn pass1_1008_4d84(u8 *param_1,param_2: *mut astruct_90,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_90 *iVar3;
  let mut uVar3: u16;
  let mut uVar4: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_90 *)param_2;
  uVar4 = (param_3 >> 0x10);
  if (iVar3->field14_0x12 != 0x0) {
    iVar3->field9_0xc = ((int)param_3 + 0xc);
    fn_ptr_1000_17ce((char *)iVar3->field4_0x4);
    iVar3->field4_0x4 = 0x0;
    iVar1 = iVar3->field9_0xc << 0x2;
    mem_op_1000_179c(iVar1,paVar2);
    &iVar3->field4_0x4 = iVar1;
    ((int)&iVar3->field4_0x4 + 0x2) = (int)paVar2;
  }
  if (iVar3->field9_0xc != 0x100) {
    return;
  }
  pass1_1000_48a8(iVar3->field4_0x4,(u32)((int)param_3 + 0x4),0x400);
  return;
}



HPALETTE16 palette_op_1008_4e08(HPALETTE16 hpal_param_2,mut param_2: u16 ,param_3: *mut astruct_13,HDC16 *phdc_param_2)

{
  HDC16 hdc_1;

  hdc_1 = *phdc_param_2;
  create_palette_1008_4e38(param_3,param_2);
  SelectPalette16(0x0,hpal_param_2,hdc_1);
  hdc_1 = *phdc_param_2;
  RealizePalette16(hdc_1);
  return hdc_1;
}



// WARNING: Unable to use type for symbol uVar3
pub fn create_palette_1008_4e38(astruct_13 *in_struct_1,mut param_2: u16 )

{
  let mut piVar1: *mut i16;
  LOGPALETTE *pLVar2;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  astruct_13 *local_struct_1;
  let mut iVar5: i16;
  astruct_13 *uVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut iStack14: i16;
  UCHAR *puStack12;
  UCHAR *puStack8;
  LOGPALETTE *uVar3;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar8 = (astruct_13 *)((u32)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_13 *)in_struct_1;
  iVar3 = (local_struct_1->field9_0xc + 0x2) * 0x4;
  if (local_struct_1->field10_0xe == NULL) {
    mem_op_1000_179c(iVar3,paVar4);
    &local_struct_1->field10_0xe = iVar3;
    ((int)&local_struct_1->field10_0xe + 0x2) = (int)paVar4;
    local_struct_1->field10_0xe->pal_version = 0x300;
    uVar3 = local_struct_1->field10_0xe;
    ((int)uVar3 + 0x2) = local_struct_1->field9_0xc;
    pLVar2 = local_struct_1->field10_0xe;
    puStack8 = (UCHAR *)((u32)pLVar2 & 0xffff0000 | (u32)((int)pLVar2 + 0x4));
    puStack12 = local_struct_1->field4_0x4;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = &local_struct_1->field9_0xc;
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar9 = ((u32)puStack12 >> 0x10);
      iVar3 = (int)puStack12;
      *puStack8 = *(UCHAR *)(iVar3 + 0x2);
      uVar10 = ((u32)puStack8 >> 0x10);
      iVar5 = (int)puStack8;
      *(u8 *)(iVar5 + 0x1) = *(u8 *)(iVar3 + 0x1);
      *(UCHAR *)(iVar5 + 0x2) = *puStack12;
      *(u8 *)(iVar5 + 0x3) = 0x0;
      iStack14 += 0x1;
      puStack8 = (UCHAR *)((u32)puStack8 & 0xffff0000 | (u32)(iVar5 + 0x4));
      puStack12 = (UCHAR *)((u32)puStack12 & 0xffff0000 | (u32)(iVar3 + 0x4));
    }
  }
  CreatePalette16((LOGPALETTE *)local_struct_1->field10_0xe);
  return;
}



StructD * pass1_1008_4ef6(StructD *param_1,u8 param_2)

{
  pass1_1008_4cdc((astruct_454 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_and_draw_op_1008_4f20
               (param_1: *mut astruct_57,param_2: *mut astruct_76,mut param_3: u32,mut param_4: u16 ,char *param_5,mut param_6: u16 ,
               mut param_7: u16 )

{
  let mut uVar1: u16;
  HDC16 hdc;
  let mut uVar2: u16;
  let mut count: i16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  HPALETTE16 hpalette_a;
  astruct_76 *struct_1;
  let mut uVar6: u16;
  let mut unaff_CS: u16;
  let mut uVar3: u32;
  COLORREF color;
  astruct_81 struct81_26;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut offset: u16;
  let mut segment: u16;
  let mut hdc_b: u16;
  u8 uVar14;
  u8 uVar15;
  struct uVar17;
  HDC16 hdc_a;

  pass1_1008_4016(param_2);
  uVar6 = ((u32)param_2 >> 0x10);
  struct_1 = (astruct_76 *)param_2;
  struct_1->lpcstr_field13_0x1e = param_5;
  struct_1->field15_0x22 = param_4;
  struct_1->field16_0x24 = param_3;
    // 0x50a2
  param_2->offset_0x0 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x9;
  struct_1->base_0x2 = 0x1008;
  uVar1 = FUN_1010_830a((int)param_3,param_1,unaff_CS,_u16_1050_14cc,0x2);
  struct_op_1008_48fe(param_1,(astruct_81 *)CONCAT22(0x1050,&struct81_26),0x1,(char *)CONCAT22((int)param_1,uVar1));
  read_file_1008_49e8(param_6,param_7,(astruct_81 *)CONCAT22(0x1050,&struct81_26),param_1);
  pass1_1008_5068(param_2,(astruct_81 *)CONCAT22(0x1050,&struct81_26));
  pass1_1008_47cc(param_2);
  pass1_1008_4834(param_2);
  segment = &DAT_1050_1050;
  offset = 0x27e;
  uVar10 = 0x0;
  uVar11 = 0x0;
  uVar8 = 0x0;
  uVar9 = 0x0;
  uVar3 = pass1_1008_4772(param_2);
  uVar4 = (uVar3 >> 0x10);
  hdc = CreateDC16((DEVMODEA *)(uVar3 & 0xffff | (u32)uVar4 << 0x10),(char *)CONCAT22(uVar9,uVar8),
                   (char *)CONCAT22(uVar11,uVar10),(char *)CONCAT22(segment,offset));
  uVar2 = palette_op_1008_46e4(&stack0xffd4,uVar4,param_2,(HDC16 *)CONCAT22(0x1050,&stack0xffd4));
  color = SetBkColor16(0xffffff,hdc);
  SetTextColor16(CONCAT22(0x100,struct_1->field15_0x22),hdc);
  count = str_op_1000_3da4(struct_1->lpcstr_field13_0x1e);
  TextOut16(count,struct_1->lpcstr_field13_0x1e,0x0,0x0,hdc);
  uVar1 = (color >> 0x10);
  hdc_a = hdc;
  SetBkColor16(color,hdc);
  SetTextColor16(CONCAT22(hdc,uVar1),hdc_a);
  hpalette_a = SelectPalette16(0x0,uVar2,hdc_a);
  DeleteObject16(hpalette_a);
  DeleteDC16(hdc_a);
  close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,&struct81_26));
  return;
}
pub fn pass1_1008_5068(param_1: *mut astruct_76,param_2: *mut astruct_81)

{
  struct_op_1008_4214(param_1,param_2);
  return;
}



StructD * pass1_1008_507c(StructD *param_1,u8 param_2)

{
  pass1_1008_41bc((astruct_288 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1008_50c2(param_1: *mut astruct_110,mut param_2: u32,mut param_3: u32,param_4: *mut u16,param_5: *mut astruct_76)

{
  astruct_110 *pstruct110_1;
  mut param_1: let mut _seg: u16;

  param_1->field0_0x0 = *param_4;
  param_1_seg = ((u32)param_1 >> 0x10);
  pstruct110_1 = (astruct_110 *)param_1;
  pstruct110_1->field1_0x2 = ((int)param_4 + 0x2);
  pstruct110_1->field2_0x4 = param_3;
  pstruct110_1->field3_0x8 = param_2;
  pstruct110_1->field4_0xc = param_5;
  pstruct110_1->field5_0x10 = 0x0;
  pass1_1008_52fc((astruct_110 *)((u32)param_1 & 0xffff | (u32)param_1_seg << 0x10));
  return;
}
pub fn pass1_1008_5118(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x10) != 0x0) {
    call_fn_ptr_1000_0dc6(*(char **)((int)param_1 + 0x10));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_5134(mut param_1: u32)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut iStack16: i16;
  i32 lStack14;
  let mut uStack10: u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  lVar4 = *(i32 *)(iVar6 + 0x4) * *(i32 *)(iVar6 + 0x8);
  lVar4 = mem_op_1000_0a48(0x1,lVar4,(int)((u32)lVar4 >> 0x10),_PTR_LOOP_1050_5f2c);
  uVar3 = ((u32)lVar4 >> 0x10);
  (iVar6 + 0x10) = (int)lVar4;
  (iVar6 + 0x12) = uVar3;
  if ((uVar3 | (iVar6 + 0x10)) == 0x0) {
    return;
  }
  iVar5 = (iVar6 + 0x8);
  iVar2 = (iVar6 + 0xa);
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0x0),iVar5 + -0x1) * *(i32 *)(iVar6 + 0x4);
  puVar1 = (u16 *)(iVar6 + 0x10);
  uVar3 = lVar4;
  uStack10 = CONCAT22(((int)((u32)lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 + (iVar6 + 0x12),
                      uVar3 + *puVar1);
  lStack14 = CONCAT22(iVar2,iVar5);
  iStack16 = (iVar6 + 0x2);
  while (lStack14 != 0x0) {
    iVar2 = iStack16 + 0x1;
    iVar5 = iStack16 >> 0xf;
    pass1_1008_4544(*(astruct_76 **)(iVar6 + 0xc));
    pass1_1000_48a8(uStack10,CONCAT22(iVar5,iStack16),(iVar6 + 0x4));
    iVar5 = (iVar6 + 0x4);
    uVar3 = -iVar5;
    uStack10 = CONCAT22((int)(uStack10 >> 0x10) +
                        (CARRY2(uStack10,uVar3) - ((iVar6 + 0x6) + (iVar5 != 0x0))) * 0x100,
                        uStack10 + uVar3);
    iStack16 = iVar2;
    lStack14 = lStack14 + -0x1;
  }
  return;
}
pub fn pass1_1008_5236(param_1: *mut astruct_109)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;
  let mut iVar5: i16;
  astruct_109 *pstruct109_6;
  let mut uVar6: u16;
  let mut bVar7: bool;
  let mut iStack12: i16;
  i32 lStack10;
  let mut uStack6: u16;
  let mut iStack4: i16;

  uVar6 = ((u32)param_1 >> 0x10);
  pstruct109_6 = (astruct_109 *)param_1;
  iVar5 = pstruct109_6->field6_0x8;
  iVar2 = pstruct109_6->field7_0xa;
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0x0),iVar5 + -0x1) * *(i32 *)&pstruct109_6->field_0x4;
  puVar1 = &pstruct109_6->field9_0x10;
  uVar3 = lVar4;
  uStack6 = uVar3 + *puVar1;
  iStack4 = ((int)((u32)lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 + pstruct109_6->field10_0x12;
  lStack10 = CONCAT22(iVar2,iVar5);
  iStack12 = pstruct109_6->field2_0x2;
  while (lStack10 != 0x0) {
    iVar2 = iStack12 + 0x1;
    iVar5 = iStack12 >> 0xf;
    pass1_1008_4544((astruct_76 *)pstruct109_6->field8_0xc);
    pass1_1000_48a8(CONCAT22(iVar5,iStack12),CONCAT22(iStack4,uStack6),&pstruct109_6->field_0x4);
    iVar5 = &pstruct109_6->field_0x4;
    uVar3 = -iVar5;
    bVar7 = CARRY2(uStack6,uVar3);
    uStack6 += uVar3;
    iStack4 += (bVar7 - (pstruct109_6->field5_0x6 + (iVar5 != 0x0))) * 0x100;
    iStack12 = iVar2;
    lStack10 = lStack10 + -0x1;
  }
  return;
}
pub fn pass1_1008_52fc(param_1: *mut astruct_110)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  i32 lVar4;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  astruct_110 *pstruct110_8;
  astruct_110 *uVar8;
  let mut uVar9: u32;
  let mut uStack14: u16;
  let mut iStack12: i16;

  uVar8 = (astruct_110 *)((u32)param_1 >> 0x10);
  pstruct110_8 = (astruct_110 *)param_1;
  uVar9 = pass1_1008_4772(pstruct110_8->field4_0xc);
  uVar5 = (uVar9 >> 0x10);
  iVar7 = (int)uVar9;
  iVar6 = (iVar7 + 0x4);
  uVar3 = iVar6 - 0x1;
  iVar6 = (iVar7 + 0x6) - (iVar6 == 0x0);
  lVar4 = *(i32 *)(iVar7 + 0x8) + -0x1;
  uVar2 = param_1->field0_0x0;
  puVar1 = &pstruct110_8->field2_0x4;
  iVar7 = ((int)uVar2 >> 0xf) + ((int)&pstruct110_8->field2_0x4 + 0x2) + CARRY2(uVar2,puVar1);
  if ((iVar6 <= iVar7) && ((iVar6 < iVar7 || (uVar3 < uVar2 + puVar1)))) {
    &pstruct110_8->field2_0x4 = uVar3 - uVar2;
    ((int)&pstruct110_8->field2_0x4 + 0x2) = (iVar6 - ((int)uVar2 >> 0xf)) - (uVar3 < uVar2);
  }
  uVar2 = pstruct110_8->field1_0x2;
  puVar1 = &pstruct110_8->field3_0x8;
  iVar6 = ((int)uVar2 >> 0xf) + ((int)&pstruct110_8->field3_0x8 + 0x2) + CARRY2(uVar2,puVar1);
  iStack12 = (int)((u32)lVar4 >> 0x10);
  if ((iStack12 <= iVar6) && ((uStack14 = lVar4, iStack12 < iVar6 || (uStack14 < uVar2 + puVar1)))) {
    &pstruct110_8->field3_0x8 = uStack14 - uVar2;
    ((int)&pstruct110_8->field3_0x8 + 0x2) = (iStack12 - ((int)uVar2 >> 0xf)) - (uStack14 < uVar2);
  }
  return;
}



u32 * pass1_1008_5394(u32 *param_1)

{
  *param_1 = 0x0;
  return param_1;
}
pub fn pass1_1008_53aa(void)

{
  return;
}
pub fn mci_send_command_1008_53ae(mut param_1: u32,mut param_2: u16 )

{
  let mut DVar1: u32;
  let mut DVar2: u32;
  let mut w_error:u32;
  let mut local_32: u16;
  let mut uStack48: u16;
  let mut local_2e: u16;
  let mut uStack44: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut local_1e: u32;
  let mut UStack26: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;

  local_1e = 0x0;
  uStack22 = 0x28c;
  uStack20 = &DAT_1050_1050;
  uStack18 = param_1;
  uStack14 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(CONCAT22(0x1050,&local_1e),0x30200,0x803,0x0);
  DVar1 = (DVar1 >> 0x10);
  uStack34 = DVar1;
  if (UStack26 != 0x0) {
    if ((DVar1 | uStack34) != 0x0) {
      mciGetErrorString16(0x400,(char *)CONCAT22(0x1050,&stack0xfbce),DVar1);
    }
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_2e),NULL,0xc);
    local_2e = param_2;
    uStack44 = 0x0;
    DVar2 = mciSendCommand16(CONCAT22(0x1050,&local_2e),0x2,0x806,UStack26);
    DVar2 = (DVar2 >> 0x10);
    uStack34 = DVar2;
    DVar1 = DVar2;
    if ((DVar2 | uStack34) != 0x0) {
      mciGetErrorString16(0x400,(char *)CONCAT22(0x1050,&stack0xfbce),DVar2);
    }
    local_32 = param_2;
    uStack48 = 0x0;
    w_error = mciSendCommand16(CONCAT22(0x1050,&local_32),0x1,0x804,UStack26);
    DVar1 = (w_error >> 0x10);
    uStack34 = w_error;
    if ((DVar1 | uStack34) != 0x0) {
      mciGetErrorString16(0x400,(char *)CONCAT22(0x1050,&stack0xfbce),w_error);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn init_op_1008_54aa(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u8 *param_4,char *param_5,u8 *param_6,
                      u8 *param_7)

{
  code **ppcVar1;
  let mut uVar3: u16;
  let mut in_CX: u16;
  let mut in_DX: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut in_stack_0000ffea: u16;
  let mut in_stack_0000ffec: u16;
  u32 *puStack12;
  let mut uVar2: u32;

  if (param_6 != NULL) {
    return;
  }
  dos3_call_op_1000_435c(unaff_CS,NULL,unaff_SI,unaff_DI,in_stack_0000ffea,in_stack_0000ffec);
  pass1_1000_4d0c(param_1);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(in_DX,0x0,0x32,0x0,0x12);
  _PTR_LOOP_1050_029c = mem_op_1000_1902((int)(_PTR_LOOP_1050_03a0 >> 0x10),0x0,0x64,0x0,0xc);
  _PTR_LOOP_1050_4fb8 = mem_op_1000_1902((int)(_PTR_LOOP_1050_029c >> 0x10),0x0,0x64,0x0,0x10);
  _PTR_LOOP_1050_68a2 = mem_op_1000_1902((int)(_PTR_LOOP_1050_4fb8 >> 0x10),0x0,0x64,0x0,0xe);
  _PTR_LOOP_1050_5744 = mem_op_1000_1902((int)(_PTR_LOOP_1050_68a2 >> 0x10),0x0,0x1f4,0x0,0x42);
  uVar6 = mem_op_1000_1902((int)(_PTR_LOOP_1050_5744 >> 0x10),0x0,0x32,0x0,0x6);
  PTR_LOOP_1050_576a = (u8 *)(uVar6 >> 0x10);
  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,PTR_LOOP_1050_576a);
  PTR_LOOP_1050_5768 = (u8 *)uVar6;
  HINSTANCE16_1050_038c = (HINSTANCE16)param_7;
  PTR_LOOP_1050_038e = param_6;
  PTR_LOOP_1050_0390 = param_4;
  uVar3 = str_op_1008_60e8(PTR_LOOP_1050_576a,param_5);
  _PTR_LOOP_1050_0392 = CONCAT22((int)paVar5,uVar3);
  mem_op_1000_179c(0xc,paVar5);
  extraout_DX = paVar5 | uVar3;
  if (extraout_DX == 0x0) {
    uVar3 = 0x0;
    extraout_DX = 0x0;
  }
  else {
    struct_op_1008_0000((u16 *)CONCAT22(paVar5,uVar3));
  }
  puStack12 = (u32 *)CONCAT22(extraout_DX,uVar3);
  uVar4 = extraout_DX;
  if (_PTR_LOOP_1050_0392 != 0x0) {
    ppcVar1 = (code **)((int)*puStack12 + 0x4);
    (**ppcVar1)(0x1000,uVar3,extraout_DX,_PTR_LOOP_1050_0392);
  }
  uVar7 = CONCAT22(extraout_DX,uVar3);
  uVar2 = *puStack12;
  ppcVar1 = (code **)uVar2 + 0x4;
  (**ppcVar1)();
  win_msg_op_1008_9498();
  if (puStack12 != NULL) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)(0x1000,uVar3,extraout_DX,0x1,uVar7);
  }
  uVar6 = mem_op_1000_1b68(uVar4,_PTR_LOOP_1050_03a0,(_PTR_LOOP_1050_03a0 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_029c,(_PTR_LOOP_1050_029c >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_4fb8,(_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_68a2,(_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_5744,(_PTR_LOOP_1050_5744 >> 0x10));
  return;
}
pub fn def_win_proc_1008_5632(LPARAM param_1,WPARAM param_2,mut param_3: u16 ,HWND16 param_4)

{
  code **ppcVar1;
  u32 *puStack6;
  let mut uVar2: u16;

  uVar2 = &DAT_1050_1050;
  puStack6 = (u32 *)GetWindowLong16(0x0,param_4);
  if ((((u32)puStack6 >> 0x10) | puStack6) == 0x0) {
    if (param_3 != 0x1) {
      DefWindowProc16(param_1,param_2,param_3,param_4);
      return;
    }
    puStack6 = (u32*)param_1;
    SetWindowLong16((long)puStack6,0x0,param_4);
    pass1_1008_9628(puStack6,param_4);
  }
  ppcVar1 = (code **)((int)*puStack6 + 0x1c);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack6,(int)((u32)puStack6 >> 0x10),param_1,param_2,param_3,uVar2);
  return;
}



u16 * struct_op_1008_56b4(param_1: *mut astruct_76)

{
  astruct_76 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->base_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  param_1->offset_0x0 = s__s__d_1050_573a;
  iVar1->base_0x2 = 0x1008;
  return &param_1->offset_0x0;
}



BOOL16 cleanup_palette_1008_56e2(mut param_1: u32,mut param_2: u32)

{
  HPALETTE16 hpalette_a;
  let mut u16_a: u16;

  u16_a = (param_1 >> 0x10);
  hpalette_a = SelectPalette16(0x0,*(HPALETTE16 *)((int)param_1 + 0x4),*(HDC16 *)param_2);
  *(HPALETTE16 *)((int)param_1 + 0x4) = hpalette_a;
  DeleteObject16(hpalette_a);
  return 0x1;
}



u16 * pass1_1008_570e(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn set_struct_1008_574a(param_1: *mut astruct_57)

{
  StructD *iVar1;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  (u32)&iVar1->hfile_0x4 = 0x0;
  iVar1->field5_0x8 = 0x0;
  iVar1->field6_0xa = 0x1;
    // just 0x5bc4
  param_1->field0_0x0 = (int)s__s__s__1050_5bc0 + 0x4;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1008_5784(char *param_1,mut param_2: u32)

{
  (u32)param_1 = param_2;
  (u32)((int)param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1008_57a4(u32 *param_1,mut param_2: u32)

{
  *param_1 = param_2;
  (u32)((int)param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1008_57c4(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = (int)s__s__s__1050_5bc0 + 0x4;
  ((int)param_1 + 0x2) = 0x1008;
  pass1_1008_5830((u32)param_1 & 0xffff | (u32)uVar1 << 0x10);
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}



i32 pass1_1008_57f0(mut param_1: u32,mut param_2: i16)

{
  let mut bVar1: bool;
  i32 lVar2;
  let mut iStack12: i16;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),param_1);
  iStack12 = 0x0;
  do {
    lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar2 == 0x0) {
      return 0x0;
    }
    bVar1 = iStack12 != param_2;
    iStack12 = iStack12 + 0x1;
  } while (bVar1);
  return lVar2;
}
pub fn pass1_1008_5830(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  u32 *puVar5;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;

  while( true ) {
    uVar8 = (param_1 >> 0x10);
    iVar6 = (int)param_1;
    if (*(i32 *)(iVar6 + 0x4) == 0x0) break;
    if ((iVar6 + 0xa) != 0x0) {
      uVar4 = (u32)(iVar6 + 0x4);
      uVar9 = ((u32)uVar4 >> 0x10);
      iVar7 = (int)uVar4;
      puVar1 = (u32 *)(iVar7 + 0x8);
      uVar2 = (iVar7 + 0xa);
      if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
      }
    }
    puVar5 = (u32 *)*(i32 *)(iVar6 + 0x4);
    (u32)(iVar6 + 0x4) = (u32)((int)puVar5 + 0x4);
    if (puVar5 != NULL) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
    }
  }
  (iVar6 + 0x8) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_58a6(mut param_1: u32,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_99 *paStack6;

  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_029c);
  uVar3 = ((u32)paStack6 >> 0x10);
  uVar2 = paStack6;
  if ((uVar3 | uVar2) == 0x0) {
    paStack6 = NULL;
  }
  else {
    paStack6->field0_0x0 = 0x389a;
    (uVar2 + 0x2) = 0x1008;
    (u32)(uVar2 + 0x4) = 0x0;
    (u32)(uVar2 + 0x8) = 0x0;
    paStack6->field0_0x0 = s__s__s__1050_5bc0;
    (uVar2 + 0x2) = 0x1008;
  }
  if (paStack6 == NULL) {
    return;
  }
  uVar5 = ((u32)paStack6 >> 0x10);
  (u32)((int)paStack6 + 0x8) = param_2;
  uVar6 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  (u32)((int)paStack6 + 0x4) = (u32)(iVar4 + 0x4);
  *(astruct_99 **)(iVar4 + 0x4) = paStack6;
  piVar1 = (i16 *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_593c(u32 *param_1,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  astruct_99 *paStack6;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if ((iVar5 + 0x8) == 0x0) {
    ppcVar2 = (code **)((int)*param_1 + 0x4);
    (**ppcVar2)();
    return;
  }
  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_029c);
  uVar4 = ((u32)paStack6 >> 0x10);
  uVar3 = paStack6;
  if ((uVar4 | uVar3) == 0x0) {
    paStack6 = NULL;
  }
  else {
    paStack6->field0_0x0 = 0x389a;
    (uVar3 + 0x2) = 0x1008;
    (u32)(uVar3 + 0x4) = 0x0;
    (u32)(uVar3 + 0x8) = 0x0;
    paStack6->field0_0x0 = s__s__s__1050_5bc0;
    (uVar3 + 0x2) = 0x1008;
  }
  if (paStack6 == NULL) {
    return;
  }
  (u32)((int)paStack6 + 0x8) = param_2;
  do {
    param_1 = (u32*)((int)param_1 + 0x4);
    uVar7 = ((u32)param_1 >> 0x10);
  } while (*(i32 *)((int)param_1 + 0x4) != 0x0);
  *(astruct_99 **)((int)param_1 + 0x4) = paStack6;
  piVar1 = (i16 *)(iVar5 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}
pub fn pass1_1008_59f4(mut param_1: u32,i32 param_2)

{
  let mut piVar1: *mut i16;
  u32 *puVar2;
  let mut uVar3: u16;
  u32 *puVar4;
  code **ppcVar5;
  u32 *puVar6;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack10: u16;
  u32 *puStack6;

  puStack6 = NULL;
  uVar9 = (param_1 >> 0x10);
  puVar6 = puStack6;
  puVar4 = (u32 *)param_1;
  do {
    puStack6 = puVar6;
    uVar10 = ((u32)puVar4 >> 0x10);
    iVar8 = (int)puVar4;
    puVar4 = (u32 *)*(i32 *)(iVar8 + 0x4);
    uStack10 = puVar4;
    uVar11 = ((u32)puVar4 >> 0x10);
    if (((iVar8 + 0x6) | uStack10) == 0x0) break;
    puVar6 = puVar4;
  } while (*(i32 *)(uStack10 + 0x8) != param_2);
  if (puVar4 != NULL) {
    if (puStack6 == NULL) {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
      puStack6 = (u32 *)param_1;
    }
    else {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
    }
    uVar12 = ((u32)puStack6 >> 0x10);
    ((int)puStack6 + 0x4) = uVar10;
    ((int)puStack6 + 0x6) = uVar7;
    if (((int)param_1 + 0xa) != 0x0) {
      puVar2 = (u32 *)(uStack10 + 0x8);
      uVar3 = (uStack10 + 0xa);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar5 = (code **)*puVar2;
        (**ppcVar5)();
      }
    }
    if (puVar4 != NULL) {
      ppcVar5 = (code **)*puVar4;
      (**ppcVar5)();
    }
    piVar1 = (i16 *)((int)param_1 + 0x8);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  return;
}
pub fn pass1_1008_5ab8(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0x4) == 0x0) {
    return;
  }
  puVar3 = (u32 *)(u32)(iVar4 + 0x4);
  uVar6 = ((u32)puVar3 >> 0x10);
  (u32)(iVar4 + 0x4) = (u32)(puVar3 + 0x4);
  if ((uVar6 | puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
  }
  piVar1 = (i16 *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + -0x1;
  return;
}
pub fn pass1_1008_5b12(char *param_1)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;

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
    (u32)(iVar2 + 0x4) = (u32)(iVar3 + 0x4);
    if (*(i32 *)(iVar2 + 0x4) != 0x0) {
      return;
    }
  }
  return;
}



u16 * pass1_1008_5b6e(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((astruct_611 *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_5b9a(StructD *param_1,u8 param_2)

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_5bdc(char *param_1)

{
  let mut in_EDX: u32;
  let mut uVar1: u16;
  astruct_19 *pstruct19_1;
  let mut unaff_BP: u16;
  astruct_19 *pstruct19_param_1;
  astruct_19 *paVar2;
  u32 *puVar3;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  uVar1 = ((u32)in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48((astruct_19 *)param_1,0x44);
  pstruct19_param_1 = (astruct_19 *)((u32)param_1 >> 0x10);
  pstruct19_1 = (astruct_19 *)param_1;
  pstruct19_1->horiz_res_0xa = 0x0;
  (u32)&pstruct19_1->ver_res_0xc = 0x0;
  pstruct19_1->field8_0x10 = 0x0;
  pstruct19_1->field9_0x12 = 0x0;
  param_1 = 0x5fc8;
  pstruct19_1->segment_0x2 = 0x1008;
  _u16_1050_02a0 = param_1;
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar1,(int)((u32)paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_BP,0x2),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  pstruct19_1->ver_res_0xc = puVar3;
  &pstruct19_1->field_0xe = (int)((u32)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_5c34(char *param_1)

{
  param_1 = 0x5fc8;
  ((int)param_1 + 0x2) = 0x1008;
  _u16_1050_02a0 = 0x0;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1008_5c5c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  pass1_1010_84f8(_u16_1050_14cc,param_4);
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),(WNDCLASS16 *)&DAT_1050_1050);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1008_5c7c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  pass1_1010_85be(_u16_1050_14cc,(int)param_4,(int)(param_4 >> 0x10));
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),(WNDCLASS16 *)&DAT_1050_1050);
  return;
}
pub fn win_1008_5c9e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u32 *param_4)

{
  win_1008_5c7c(param_1,param_2,param_3,*param_4);
  return;
}
pub fn mci_send_command_1008_5cb6(param_1: *mut astruct_27,mut param_2: i16)

{
  astruct_27 *iVar1;
  let mut uVar1: u16;
  let mut iVar2: i16;

  mciSendCommand16(0x0,0x0,0x804,param_2);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_27 *)param_1;
  if ((&iVar1->field_0xa == 0x0) || (&iVar1->field_0xa != param_2)) {
    iVar1->field18_0x12 = 0x0;
    iVar2 = 0x11;
  }
  else {
    &iVar1->field_0x10 = 0x0;
    iVar2 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar2);
  return;
}
pub fn win_ui_op_1008_5cfe(param_1: *mut astruct_27,char *param_2,WNDCLASS16 *in_wnd_class)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  astruct_27 *iVar3;
  let mut uVar3: u16;
  let mut DVar4:u32;
  let mut iVar5: i16;
  HWND16 message_1;
  let mut uStack298: u16;
  HWND16 window_handle_1;
  u8 local_11e [0x100];
  char *string_1;
  let mut iStack26: i16;
  let mut iStack24: i16;
  u8 local_16 [0x4];
  let mut offset_val: u16;
  char *pcStack14;
  char *pcStack10;

  pass1_1000_4906((StructD *)CONCAT22(0x1050,local_16),NULL,0x14);
  pcStack10 = param_2;
  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_27 *)param_1;
  uVar1 = (u32)&iVar3->field_0xc;
  iStack24 = ((int)uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,NULL,0x0,0x0,(WNDCLASS16 *)CONCAT22(0x1050,local_11e));
  iVar2 = pass1_1000_475e(CONCAT22(0x1050,local_11e),(u32)s__mid_1050_02ae);
  if (iVar2 == 0x0) {
    uVar1 = (u32)&iVar3->field_0xc;
    iStack24 = ((int)uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0x0;
  }
  if (iStack24 != 0x0) {
    if ((iStack26 != 0x0) && (&iVar3->field_0x10 != 0x0)) {
      return;
    }
    if ((iStack26 == 0x0) && (iVar3->field18_0x12 != 0x0)) {
      return;
    }
    pcStack14 = string_1;
    DVar4 = mciSendCommand16(CONCAT22(0x1050,local_16),0x2200,0x803,0x0);
    if (((DVar4 >> 0x10) | DVar4) == 0x0) {
      if (iStack26 == 0x0) {
        iVar3->field18_0x12 = 0x1;
      }
      else {
        &iVar3->field_0xa = offset_val;
        &iVar3->field_0x10 = 0x1;
      }
      window_handle_1 = create_window_1008_5e7e();
      if (window_handle_1 == 0x0) {
        mci_send_command_1008_5cb6(param_1,offset_val);
        return;
      }
      pass1_1000_4906((StructD *)CONCAT22(0x1050,&message_1),NULL,0xc);
      message_1 = window_handle_1;
      uStack298 = 0x0;
      mciSendCommand16(CONCAT22(0x1050,&message_1),0x1,0x806,offset_val);
      SetWindowWord16(offset_val,0x0,window_handle_1);
      return;
    }
  }
  if (iStack26 == 0x0) {
    iVar5 = 0x11;
  }
  else {
    iVar5 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar5);
  return;
}



HWND16 create_window_1008_5e7e(void)

{
  u32 *puVar2;
  let mut BVar3:bool;
  ATOM AVar4;
  HWND16 window_handle_1;
  let mut iVar5: i16;
  char *string_1;
  u32 *puVar5;
  WNDCLASS16 wndclass_44;
  u32 local_12 [0x4];
  char *puVar1;

  puVar5 = local_12;
  string_1 = (char *)s_MciSoundWindow_1050_02bd;
  for (iVar5 = 0x3; iVar5 != 0x0; iVar5 += -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = string_1;
    string_1 = (char *)((int)string_1 + 0x4);
    *puVar2 = (u32)puVar1;
  }
  puVar5 = string_1;
  *(u8 *)(u16 *)((int)puVar5 + 0x2) = *(u8 *)(u16 *)((int)string_1 + 0x2);
  wndclass_44.style = 0x2000;
  wndclass_44.lpfn_wnd_proc._0_2_ = SUB42(&DAT_1050_5f44,0x0);
  wndclass_44.lpfn_wnd_proc = 0x1008;
  wndclass_44.cb_wnd_extra = 0x2;
  wndclass_44.h_instance = HINSTANCE16_1050_038c;
  wndclass_44.h_icon = 0x0;
  wndclass_44.h_cursor = 0x0;
  wndclass_44.cb_cls_extra = 0x0;
  wndclass_44.hbr_background = GetStockObject16(WHITE_BRUSH);
  wndclass_44.lpsz_menu_name = 0x0;
  wndclass_44.lpsz_class_name._0_2_ = local_12;
  wndclass_44.lpsz_class_name = SUB42(&DAT_1050_1050,0x0);
  BVar3 = GetClassInfo16(&wndclass_44,(char *)CONCAT22((u32 *)wndclass_44.lpsz_class_name,0x1050),
                         (HINSTANCE16)&DAT_1050_1050);
  if (BVar3 == 0x0) {
    AVar4 = RegisterClass16(&wndclass_44);
    if (AVar4 == 0x0) {
      OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc);
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16(0x0,(void *)(u32)HINSTANCE16_1050_038c,HWND16_1050_0396,0x1,0x1,-0x8000,-0x8000,0x0,0xcf,
                      s_MciSound_registerClass_failed_1050_02cc + 0x1e,(char *)CONCAT22(0x1050,local_12));
  return window_handle_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT make_def_win_proc_1008_5f44
                  (mut param_1: u16 ,mut param_2: u16 ,LPARAM param_3,WPARAM16 in_wparam_2,mut param_5: u16 ,HWND16 param_6)

{
  WORD WVar1;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  LRESULT LVar3;
  u32 *puVar4;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0x2) {
    WVar1 = GetWindowWord16(0x0,param_6);
    mci_send_command_1008_5cb6(_u16_1050_02a0,WVar1);
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x37),in_stack_0000fea0,
                             in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1008_aa28(puVar4,(u32)puVar4);
  }
  else {
    if (param_5 != 0x3b9) {
      LVar3 = DefWindowProc16(param_3,in_wparam_2,param_5,param_6);
      return LVar3;
    }
    DestroyWindow16(param_6);
  }
  return 0x0;
}



pub fn pass1_1008_5fa2(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_5c34((char *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 * pass1_1008_5fd8(u8 *param_1)

{
  let mut puVar1: *mut u16;
  u8 *puVar2;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  char *pcVar4;
  let mut in_stack_00000004: u16;
  let mut puStack6: *mut u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar2 = &stack0x0006;
  puStack6 = (u16 *)CONCAT22(0x1050,puVar2);
  mem_op_1000_179c(0x1000,paVar3);
  pcVar4 = load_string_1010_847e(_u16_1050_14cc,in_stack_00000004);
  unk_str_op_1000_3d3e((char *)CONCAT22((int)paVar3,puVar2),pcVar4);
  while( true ) {
    puVar1 = puStack6;
    puStack6 = (u16 *)((u32)puStack6 & 0xffff0000 | (u32)((int)puStack6 + 0x2));
    if (*puVar1 == 0x0) break;
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,*puVar1);
    pass1_1000_3cea(CONCAT22((int)paVar3,puVar2),pcVar4);
  }
  return puVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn debug_print_1008_6048(mut param_1: u16 ,char *param_2)

{
  let mut uVar1: u16;
  char local_106 [0x100];

  if (PTR_LOOP_1050_02ec != NULL) {
    if (DAT_1050_02ee == 0xffff) {
      uVar1 = pass1_1000_3ec0(0x2f4,&DAT_1050_1050);
      DAT_1050_02ee = ((param_1 | uVar1) != 0x0);
    }
    if (DAT_1050_02ee != 0x0) {
      wvsprintf16((WORD *)&stack0x0008,(char *)CONCAT22((int)param_2,0x1050),
                  (char *)CONCAT22(local_106,(int)((u32)param_2 >> 0x10)));
      OutputDebugString16((char *)CONCAT22(0x1050,local_106));
      OutputDebugString16(s__1050_02fa);
      if (_PTR_LOOP_1050_02f0 != 0x0) {
        pass1_1000_2b5c(_PTR_LOOP_1050_02f0,((u32)_PTR_LOOP_1050_02f0 >> 0x10),0x2fd,
                        &DAT_1050_1050);
        pass1_1000_2f48(_PTR_LOOP_1050_02f0);
      }
    }
  }
  return;
}



u16 str_op_1008_60e8(mut param_1: u16 ,char *param_2)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_2 != NULL) {
    uVar1 = str_op_1000_3da4(param_2);
    uVar1 += 0x1;
    mem_op_1000_179c(uVar1,paVar2);
    if ((paVar2 | uVar1) != 0x0) {
      unk_str_op_1000_3d3e((char *)CONCAT22(paVar2,uVar1),param_2);
      return uVar1;
    }
  }
  return 0x0;
}
pub fn pass1_1008_612e(mut param_1: u16 ,mut param_2: i16,mut param_3: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  i32 lVar3;
  let mut iVar4: i16;
  let mut iStack18: i16;
  let mut iStack16: i16;

  uVar1 = pass1_1000_4d24();
  uVar2 = (param_3 - param_2) + 0x1;
  if (((int)uVar2 >> 0xf | uVar2) == 0x0) {
    return;
  }
  iStack16 = 0x1;
  iStack18 = param_2;
  do {
    if (param_3 < iStack18) {
      return;
    }
    lVar3 = (long)iStack16 * (long)(0x7fff / (sqword)(long)(int)uVar2);
    iVar4 = (int)((u32)lVar3 >> 0x10);
    if ((int)uVar1 >> 0xf <= iVar4) {
      if ((int)uVar1 >> 0xf < iVar4) {
        return;
      }
      if (uVar1 <= lVar3) {
        return;
      }
    }
    iStack18 += 0x1;
    iStack16 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_20 * unk_draw_op_1008_61b2(mut param_1: u16 ,StructA *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  HGDIOBJ16 l_hgdiobj_1;
  HCURSOR16 l_hcursor_1;
  astruct_57 *in_EDX;
  astruct_20 *iVar2;
  let mut uVar2: u16;
  let mut l_struct_2: *mut u16;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  astruct_20 *uVar5;
  let mut in_stack_0000ffe8: u16;
  astruct_20 *iVar1;
  astruct_20 *iVar4;
  let mut uVar1: *mut u16;

  set_struct_1008_687a((astruct_20 *)param_2,param_5);
  uVar2 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  iVar2->field164_0xde = param_3;
  iVar2->field165_0xe0 = 0x0;
  param_2->field0_0x0 = 0x6378;
  iVar2->base_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar2->field60_0x5b)),s_DanBrotherton_1050_0302);
  l_hgdiobj_1 = GetStockObject16(BLACK_BRUSH);
  iVar2->hgdiobj_field_0xc6 = l_hgdiobj_1;
  l_hcursor_1 = LoadCursor16((char *)0x7f00,0x0);
  iVar2->hcursor_field_0xc4 = l_hcursor_1;
  iVar2->field150_0xc8 = 0x200b;
  iVar2->field139_0xac = 0x45000000;
  iVar2->field145_0xbc = ((int)param_5 + 0x8);
  l_struct_2 = (u16 *)
               mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(param_1,0x48),in_stack_0000fe90,
                               in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
  uVar1 = (u16 *)((u32)l_struct_2 >> 0x10);
  iVar2->field141_0xb4 = 0x0;
  iVar2->field142_0xb6 = 0x0;
  iVar2->field143_0xb8 = ((int)l_struct_2 + 0xa);
  iVar2->field144_0xba = ((int)l_struct_2 + 0xc);
  iVar2->field151_0xca = param_4;
  win_ui_reg_class_1008_96d2(param_2);
  return (astruct_20 *)param_2;
}
pub fn destroy_win_1008_628e(mut param_1: u32)

{
  code **fn_ptr_1;

  fn_ptr_1 = (code **)((int)(u32)((int)param_1 + 0xd2) + 0x14);
  (**fn_ptr_1)();
  DestroyWindow16(*(HWND16 *)((int)param_1 + 0x8));
  ((int)param_1 + 0x8) = 0x0;
  return;
}
pub fn fill_rect_1008_62c0(param_1: *mut astruct_838,mut param_2: u16 )

{
  RECT16 rect_2e [0x2];
  HBRUSH16 hbrush_var38;
  HDC16 hbrush_var36;
  u8 paintstruct_22 [0x20];

  hbrush_var36 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),param_1->field8_0x8);
  hbrush_var38 = CreateSolidBrush16(0x210070b);
  GetClientRect16(rect_2e,(HWND16)&DAT_1050_1050);
  FillRect16(hbrush_var38,rect_2e,(HDC16)&DAT_1050_1050);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),param_1->field8_0x8);
  DeleteObject16(hbrush_var38);
  return;
}
pub fn FUN_1008_6324(void)

{
  return;
}
pub fn FUN_1008_6328(void)

{
  return;
}
pub fn FUN_1008_632c(void)

{
  return;
}
pub fn pass1_1008_6330(param_1: *mut astruct_456,u8 param_2)

{
  astruct_456 *uVar1;
  let mut uVar2: u16;

  uVar1 = (astruct_456 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = ((u32)param_1 >> 0x10);
  param_1 = 0x380a;
  uVar1->field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}
pub fn file_1008_6414(param_1: *mut astruct_57,u32 *param_2,char *param_3)

{
  code **ppcVar1;
  astruct_76 *paVar2;
  astruct_81 *pstruct81_3;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  astruct_81 string_26;

  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (int)param_2;
  *param_2 = 0x0;
  (u32)(iVar4 + 0x4) = 0x0;
  pstruct81_3 = &string_26;
  struct_op_1008_48fe(param_1,(astruct_81 *)CONCAT13(0x10,CONCAT12(0x50,pstruct81_3)),0x1,param_3);
  mem_op_1000_179c(0x1e,param_1);
  uVar3 = param_1 | pstruct81_3;
  if (uVar3 == 0x0) {
    *param_2 = 0x0;
  }
  else {
    paVar2 = (astruct_76 *)CONCAT22(param_1,pstruct81_3);
    pstruct81_3 = &string_26;
    struct_op_1008_3f92(paVar2,(char *)CONCAT22(0x1050,&string_26));
    *(astruct_81 **)param_2 = pstruct81_3;
    (iVar4 + 0x2) = uVar3;
  }
  ppcVar1 = (code **)((int)(u32)*param_2 + 0x14);
  (**ppcVar1)(0x1000,*param_2);
  *(astruct_81 **)(iVar4 + 0x4) = pstruct81_3;
  (iVar4 + 0x6) = uVar3;
  close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,&string_26));
  return;
}
pub fn pass1_1008_64a2(param_1: *mut u16)

{
  let mut uVar1: u16;
  code **ppcVar2;

  uVar1 = ((int)param_1 + 0x2);
  if ((uVar1 | (u32 *)*param_1) != 0x0) {
    ppcVar2 = (code **)(u32)*param_1;
    (**ppcVar2)();
  }
  return;
}
pub fn pass1_1008_64c8(mut param_1: u16 ,u8 *param_2,u32 *param_3,mut param_4: u32,mut param_5: i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  let mut in_stack_0000ffe6: u16;
  let mut iStack8: i16;
  astruct_76 *paStack6;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (*param_3 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,paVar6);
  uVar3 = paVar6 | param_1;
  if (uVar3 == 0x0) {
    param_1 = 0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1008_6604((astruct_76 *)CONCAT22(paVar6,param_1),(int)param_4,
                        CONCAT22(in_stack_0000ffe6,(int)(param_4 >> 0x10)));
  }
  paStack6 = (astruct_76 *)CONCAT22(uVar3,param_1);
  iStack8 = 0x0;
  while (param_4 = param_4 & 0xffff0000 | (u32)((int)param_4 - 0x1), (int)param_4 != 0x0) {
    iVar1 = param_5 + 0x1;
    iVar4 = param_5 >> 0xf;
    pass1_1008_4544((astruct_76 *)*param_3);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(paStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_5),param_4);
    param_5 = iVar1;
    iStack8 = iVar2;
  }
  return;
}
pub fn pass1_1008_6562(param_1: *mut astruct_57,param_2: *mut astruct_76,mut param_3: u32,mut param_4: i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut pstruct57_hi: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  astruct_57 *paVar6;
  let mut iStack8: i16;
  astruct_76 *paStack6;

  pstruct57_hi = ((u32)param_1 >> 0x10);
  paVar6 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  if (*(i32 *)param_2 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,paVar6);
  uVar3 = paVar6 | pstruct57_hi;
  if (uVar3 == 0x0) {
    pstruct57_hi = 0x0;
    uVar3 = 0x0;
  }
  else {
    pass1_1008_405c((astruct_76 *)CONCAT22(paVar6,pstruct57_hi),(u32)((int)param_2 + 0x4),(int)param_3,
                    (int)(param_3 >> 0x10));
  }
  paStack6 = (astruct_76 *)CONCAT22(uVar3,pstruct57_hi);
  iStack8 = 0x0;
  while (param_3 = param_3 & 0xffff0000 | (u32)((int)param_3 - 0x1), (int)param_3 != 0x0) {
    iVar1 = param_4 + 0x1;
    iVar4 = param_4 >> 0xf;
    pass1_1008_4544(*(astruct_76 **)param_2);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(paStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_4),param_3);
    param_4 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_op_1008_6604(param_1: *mut astruct_76,mut param_2: i16,i32 param_3)

{
  let mut uVar1: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_76 *pstruct76_4;
  astruct_84 *iVar2;
  astruct_76 *uVar5;
  let mut uVar6: u16;
  i32 lVar7;

  pass1_1008_4016(param_1);
  uVar5 = (astruct_76 *)((u32)param_1 >> 0x10);
  pstruct76_4 = (astruct_76 *)param_1;
  param_1->offset_0x0 = 0x685a;
  pstruct76_4->base_0x2 = 0x1008;
  lVar7 = mem_op_1000_0a48(0x1,0x28,0x0,_PTR_LOOP_1050_5f2c);
  &pstruct76_4->field8_0x10 = (int)lVar7;
  ((int)&pstruct76_4->field8_0x10 + 0x2) = (int)((u32)lVar7 >> 0x10);
  iVar3 = (int)param_3 * 0x8 + 0x1f;
  uVar4 = ((int)(iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  pstruct76_4->field11_0x18 = uVar4;
  pstruct76_4->field12_0x1a = (int)uVar4 >> 0xf;
  lVar7 = mem_op_1000_0a48(0x1,((long)(int)uVar4 * (long)param_2),
                           (int)((u32)((long)(int)uVar4 * (long)param_2) >> 0x10),_PTR_LOOP_1050_5f2c);
  uVar4 = ((u32)lVar7 >> 0x10);
  pstruct76_4->field3_0x6 = (int)lVar7;
  pstruct76_4->field4_0x8 = uVar4;
  pstruct76_4->field9_0x14 = pstruct76_4->field3_0x6;
  pstruct76_4->field10_0x16 = uVar4;
  (u32)pstruct76_4->field8_0x10 = 0x28;
  uVar1 = pstruct76_4->field8_0x10;
  *(i32 *)((int)uVar1 + 0x4) = (long)(int)param_3;
  uVar1 = pstruct76_4->field8_0x10;
  uVar6 = (uVar1 >> 0x10);
  iVar2 = (astruct_84 *)uVar1;
  iVar2->field8_0x8 = param_2;
  iVar2->field9_0xa = param_2 >> 0xf;
  uVar1 = pstruct76_4->field8_0x10;
  ((int)uVar1 + 0xc) = 0x1;
  uVar1 = pstruct76_4->field8_0x10;
  ((int)uVar1 + 0xe) = 0x8;
  uVar1 = pstruct76_4->field8_0x10;
  (u32)((int)uVar1 + 0x10) = 0x0;
  uVar1 = pstruct76_4->field8_0x10;
  *(i32 *)((int)uVar1 + 0x14) = *(i32 *)&pstruct76_4->field11_0x18 * (long)param_2;
  uVar1 = pstruct76_4->field8_0x10;
  (u32)((int)uVar1 + 0x20) = 0x100;
  uVar1 = pstruct76_4->field8_0x10;
  (u32)((int)uVar1 + 0x24) = 0x100;
  return;
}
pub fn pass1_1008_6732(param_1: *mut astruct_288)

{
  astruct_288 *iVar2;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_288 *)param_1;
  param_1 = 0x685a;
  iVar2->field2_0x2 = 0x1008;
  if (*(i32 *)&iVar2[0x1].field2_0x2 != 0x0) {
    call_fn_ptr_1000_0dc6(*(char **)&iVar2[0x1].field2_0x2);
  }
  (u32)&iVar2[0x1].field2_0x2 = 0x0;
  pass1_1008_41bc(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub fn memcpy_op_1008_676e(param_1: *mut astruct_830,mut param_2: u16 ,param_3: *mut astruct_828)

{
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_828 *iVar1;
  astruct_829 *iVar2;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_stack_0000fff2: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar4 = ((u32)param_3 >> 0x10);
  iVar1 = (astruct_828 *)param_3;
  if (iVar1->field6_0x6 == NULL) {
    return;
  }
  mem_op_1000_179c(0x1e,paVar1);
  uVar3 = paVar1 | param_1;
  if (uVar3 == 0x0) {
    param_1 = NULL;
    uVar3 = 0x0;
  }
  else {
    uVar1 = iVar1->field13_0x10;
    uVar5 = (uVar1 >> 0x10);
    iVar2 = (astruct_829 *)uVar1;
    struct_op_1008_6604((astruct_76 *)CONCAT22(paVar1,param_1),iVar2->field7_0x8,
                        CONCAT22(in_stack_0000fff2,iVar2->field4_0x4));
  }
  pass1_1000_48a8(param_1->field13_0x10,iVar1->field13_0x10,0x28);
  uVar2 = param_1->field13_0x10;
  hmemcpy16(*(i32 *)((int)uVar2 + 0x8) * iVar1->field18_0x18,iVar1->field6_0x6,param_1->field6_0x6);
  param_1->field22_0x1c = 0x1;
  return;
}
pub fn FUN_1008_6814(void)

{
  return;
}
pub fn FUN_1008_681a(void)

{
  return;
}
pub fn FUN_1008_681e(void)

{
  return;
}



u16 FUN_1008_6822(void)

{
  return 0x0;
}
pub fn FUN_1008_6824(void)

{
  return;
}
pub fn FUN_1008_6828(void)

{
  return;
}



u16 FUN_1008_682e(void)

{
  return 0x0;
}



pub fn pass1_1008_6834(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_6732((astruct_288 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn set_struct_1008_687a(param_1: *mut astruct_20,mut param_2: u32)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;

  set_struct_op_1008_9584(param_1,param_2);
  uVar1 = (astruct_20 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1->field152_0xcc = 0x0;
  iVar1->field153_0xce = 0x0;
  set_struct_1008_574a((astruct_57 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->field154_0xd2)));
  param_1->offset_0x0 = 0x6bfc;
  iVar1->base_0x2 = 0x1008;
  iVar1->field163_0xdc = 0x0;
  return;
}



BOOL16 pass1_1008_68c6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut BVar1:bool;

  BVar1 = show_win_1008_96ae(CONCAT22(param_3,param_2),param_4);
  pass1_1008_6a04(param_1,CONCAT22(param_3,param_2));
  return BVar1;
}
pub fn pass1_1008_68ea(mut param_1: i16,mut param_2: u16 ,u32 *param_3,mut param_4: u16 ,mut param_5: u16 ,mut param_6: i16)

{
  code **ppcVar1;

  if (param_6 == 0x0) {
    if (*(i32 *)(param_1 + 0xce) != CONCAT22(param_4,param_3)) {
      if (*(i32 *)(param_1 + 0xce) != 0x0) {
        ppcVar1 = (code **)((int)(u32)(u32)(param_1 + 0xce) + 0x10);
        (**ppcVar1)();
      }
      *(i32 *)(param_1 + 0xce) = CONCAT22(param_4,param_3);
      ppcVar1 = (code **)((int)*param_3 + 0x10);
      (**ppcVar1)();
      ppcVar1 = (code **)((int)(u32)(u32)(param_1 + 0xce) + 0xc);
      (**ppcVar1)();
      return;
    }
  }
  else {
    pass1_1008_3e0e((StructA *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)));
  }
  return;
}
pub fn pass1_1008_6978(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: i16,mut param_5: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut puStack10: *mut u16;
  let mut puStack6: *mut u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0xa,paVar3);
  uVar2 = paVar3;
  puStack10 = (u16 *)CONCAT22(uVar2,param_1);
  if ((uVar2 | param_1) == 0x0) {
    puStack6 = NULL;
  }
  else {
    if (param_4 == 0x0) {
      param_4 = ((int)param_3 + 0xcc);
    }
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    (u32)(param_1 + 0x4) = param_5;
    (param_1 + 0x8) = param_4;
    *puStack10 = 0x6c8c;
    (param_1 + 0x2) = 0x1008;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)((int)(u32)((int)param_3 + 0xd2) + 0x4);
  (**ppcVar1)(0x1000,(u32 *)((int)param_3 + 0xd2),param_3,puStack6);
  return;
}
pub fn pass1_1008_6a04(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  u8 *puVar2;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  pass1_1008_57a4((u32 *)CONCAT22(0x1050,local_a),param_2 & 0xffff0000 | (u32)((int)param_2 + 0xd2));
  while( true ) {
    puVar2 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
    if ((extraout_DX | puVar2) == 0x0) break;
    ppcVar1 = (code **)((int)*(u32*)(puVar2 + 0x4) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1008_6a4a(mut param_1: u32,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  code **ppcVar1;
  let mut iVar2: i16;
  u8 *puVar3;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  u8 local_e [0x4];
  let mut uStack10: u32;
  let mut uStack6: u32;

  if (param_4 == 0x2) {
    iVar2 = (int)param_1;
    pass1_1008_57a4((u32 *)CONCAT22(0x1050,local_e),param_1 & 0xffff0000 | (u32)(iVar2 + 0xd2));
    do {
      puVar3 = local_e;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
      uStack6 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | puVar3) == 0x0) break;
    } while ((puVar3 + 0x8) != param_2);
    if (uStack6 != 0x0) {
      ppcVar1 = (code **)((int)(u32)(iVar2 + 0xd2) + 0xc);
      (**ppcVar1)();
      uStack10 = 0x0;
      uStack6._0_2_ = local_e;
      pass1_1008_5b12((char *)CONCAT22(0x1050,(u8 *)uStack6));
      if ((extraout_DX_00 | (u8 *)uStack6) != 0x0) {
        ppcVar1 = (code **)((int)*(u32*)((u8 *)uStack6 + 0x4) + 0x10);
        uStack6 = extraout_DX_00;
        (**ppcVar1)();
        (u32)(iVar2 + 0xce) = (u32)((u8 *)uStack6 + 0x4);
        return;
      }
      (u32)(iVar2 + 0xce) = 0x0;
    }
  }
  return;
}
pub fn pass1_1008_6b02(mut param_1: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xce) + 0x6c);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_6b2e(mut param_1: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xce) + 0x68);
    (**ppcVar1)();
  }
  return;
}



u16 * pass1_1008_6b5a(param_1: *mut astruct_458,u8 param_2)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_458 *uVar4;
  let mut uVar5: u16;

  uVar5 = ((u32)param_1 >> 0x10);
  uVar4 = (astruct_458 *)param_1;
  param_1 = 0x6c8c;
  uVar4->field2_0x2 = 0x1008;
  puVar1 = uVar4->field3_0x4;
  uVar2 = uVar4->field4_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1 = 0x389a;
  uVar4->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return (u16 *)param_1;
}
pub fn pass1_1008_6bb4(param_1: *mut astruct_459,u8 param_2)

{
  astruct_459 *uVar1;
  let mut uVar2: u16;

  uVar1 = (astruct_459 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = ((u32)param_1 >> 0x10);
  param_1 = 0x380a;
  uVar1->field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}
pub fn pass1_1008_6c90(param_1: *mut u16)

{
  pass1_1008_3e38((astruct_19 *)param_1);
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x6)));
  return;
}



u32 * pass1_1008_6cb4(param_1: *mut astruct_362,u32 *param_2,mut param_3: u16 ,u32 *param_4,mut param_5: u16 )

{
  astruct_362 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_362 *)param_1;
  (u32)param_1 = *param_4;
  iVar1->field4_0x4 = (param_4 + 0x1);
  iVar1->field5_0x6 = *param_2;
  iVar1->field6_0xa = (param_2 + 0x1);
  return (u32 *)param_1;
}
pub fn pass1_1008_6cec(param_1: *mut u16,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u32)

{
  pass1_1008_3e76(param_1,param_4,param_5,(param_5 >> 0x10));
  pass1_1008_3e76((u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x6)),param_2,param_3,
                  (param_3 >> 0x10));
  return;
}
pub fn pass1_1008_6d18(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)

{
  pass1_1008_3f62(param_1,param_3);
  pass1_1008_3f62((u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x6)),param_2);
  return;
}
pub fn pass1_1008_6d3e(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,param_1);
  pass1_1008_3f62(param_2,(u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x6)));
  return;
}
pub fn pass1_1008_6d64(param_1: *mut u16,param_2: *mut u16)

{
  pass1_1008_3f62(param_2,param_1);
  pass1_1008_3ee2((i16 *)param_2,(i16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x6)));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 * str_1008_6d8a(mut param_1: u16 ,u32 *param_2,char *param_3)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_2 >> 0x10);
  *param_2 = 0x0;
  ((int)param_2 + 0x4) = 0xffff;
  u16_1050_0312 = 0x4;
  sys_1000_3f9c((char *)s__1050_65a0,_PTR_s_SC_03d_1050_0314_1050_031c,0x4);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  param_2 = uVar1;
  ((int)param_2 + 0x2) = param_1;
  return param_2;
}
pub fn close_file_1008_6dd0(StructD *param_1)

{
  StructD *struct_1;
  let mut struct_1_hi: u16;

  struct_1_hi = ((u32)param_1 >> 0x10);
  struct_1 = (StructD *)param_1;
  if (struct_1->hfile_0x4 != 0xffff) {
    _lclose16(struct_1->hfile_0x4);
    struct_1->hfile_0x4 = 0xffff;
  }
  fn_ptr_1000_17ce(*(char **)param_1);
  return;
}



BOOL16 file_fn_1008_6e02(param_1: *mut astruct_802)

{
  let mut var1: u16;
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut var2:bool;
  u8 *extraout_DX;
  astruct_802 *struct_1;
  let mut unaff_DI: i16;
  let mut uVar1: u16;
  let mut puVar1: *mut u16;
  u8 local_4 [0x2];

  u16_1050_0310 = 0x0;
  var1 = write_to_file_1008_70a6(param_1);
  if (var1 != 0x0) {
    uVar1 = ((u32)param_1 >> 0x10);
    struct_1 = (astruct_802 *)param_1;
    puVar1 = pass1_1008_72a8((u16 *)CONCAT22(0x1050,local_4),struct_1->hfile_0x4);
    extraout_DX = (u8 *)((u32)puVar1 >> 0x10);
    iVar1 = pass1_1008_7006(extraout_DX,struct_1,uVar1,CONCAT22(0x1050,local_4),&DAT_1050_1050);
    if ((iVar1 != 0x0) && (iVar2 = file_fn_1008_6eee(struct_1,uVar1,CONCAT22(0x1050,local_4)), iVar2 != 0x0)) {
      var2 = file_fn_1008_726c(struct_1,uVar1);
      if (var2 == 0x0) {
        return 0x0;
      }
      return 0x1;
    }
    _lclose16(struct_1->hfile_0x4);
  }
  return 0x0;
}



BOOL16 read_file_1008_6e78(param_1: *mut astruct_806,mut param_2: u16 ,u16 in_string,mut param_4: u16 )

{
  let mut b_var1:bool;
  let mut i_var2: i16;
  u8 *var3;
  u8 *puVar1;
  astruct_802 *paVar2;
  let mut unaff_BP: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut puVar4: *mut u16;
  let mut in_stack_00000000: u16;
  let mut in_stack_0000fffc: u16;

  u16_1050_0310 = 0x0;
  b_var1 = read_file_1008_7146(unaff_CS,param_1,in_stack_0000fffc,unaff_BP,in_stack_00000000);
  if (b_var1 != 0x0) {
    uVar3 = ((u32)param_1 >> 0x10);
    paVar2 = (astruct_802 *)param_1;
    puVar4 = pass1_1008_72a8((u16 *)CONCAT22(0x1050,&stack0xfffc),paVar2->hfile_0x4);
    puVar1 = (u8 *)((u32)puVar4 >> 0x10);
    i_var2 = pass1_1008_7056(puVar1,paVar2,uVar3,CONCAT22(0x1050,&stack0xfffc));
    if (i_var2 != 0x0) {
      var3 = &stack0xfffc;
      read_file_1008_6f7a(paVar2,uVar3,CONCAT22(0x1050,var3),puVar1);
      if (var3 != NULL) {
        b_var1 = file_fn_1008_726c(paVar2,uVar3);
        if (b_var1 == 0x0) {
          return 0x0;
        }
        return 0x1;
      }
    }
    _lclose16(paVar2->hfile_0x4);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_fn_1008_6eee(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut BVar1:bool;
  let mut uVar2: u16;
  u8 *in_DX;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;
  u8 local_e [0x4];
  let mut uStack10: u32;
  u32 *puStack6;

  puStack6 = (u32 *)*_u16_1050_5748;
  uStack10 = *puStack6;
  puVar4 = pass1_1020_a43e(in_DX,(u16 *)CONCAT22(0x1050,local_e));
  uVar3 = ((u32)puVar4 >> 0x10);
  BVar1 = pass1_1028_d7a0(uStack10,((u32)uStack10 >> 0x10),param_3);
  if (BVar1 != 0x0) {
    BVar1 = pass1_1030_5c1a(_PTR_LOOP_1050_5736,param_3);
    if (BVar1 != 0x0) {
      uVar5 = write_to_file_1028_dce2(uVar3,_PTR_LOOP_1050_65e2,param_3);
      if ((int)((u32)uVar5 >> 0x10) != 0x0) {
        uVar2 = pass1_1038_7b20(_PTR_LOOP_1050_5a64,param_3);
        if (uVar2 != 0x0) {
          BVar1 = pass1_1020_a644(local_e,&DAT_1050_1050,param_3);
          if (BVar1 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn read_file_1008_6f7a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u8 *param_4)

{
  u16_t var5;
  let mut i_var3: i16;
  let mut b_var4:bool;
  u16_t uVar1;
  let mut puVar2: *mut u16;
  u8 local_e [0x4];
  let mut uStack10: u32;
  u32 *puStack6;

  puStack6 = (u32 *)*_u16_1050_5748;
  uStack10 = *puStack6;
  puVar2 = pass1_1020_a43e(param_4,(u16 *)CONCAT22(0x1050,local_e));
  uVar1 = (u16_t)((u32)puVar2 >> 0x10);
  var5 = read_file_1028_d7ba((u16_t)puVar2,(int)uStack10,(int)((u32)uStack10 >> 0x10),param_3);
  if (var5 != 0x0) {
    var5 = read_file_1030_5c52(var5,_PTR_LOOP_1050_5736,param_3);
    if (var5 != 0x0) {
      read_file_1028_def2(var5,_PTR_LOOP_1050_65e2,param_3);
      if (var5 != 0x0) {
        i_var3 = read_file_1038_7c02(var5,uVar1,_PTR_LOOP_1050_5a64,param_3);
        if (i_var3 != 0x0) {
          b_var4 = read_file_1020_a65e((u16_t)local_e,CONCAT22(0x1050,local_e),param_3);
          if (b_var4 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1008_7006(u8 *param_1,param_2: *mut astruct_802,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  u32 *puVar4;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;
  let mut iStack4: i16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iStack4 = 0x0;
  while( true ) {
    if ((int)PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fff8,(iStack4 * 0x2 + 0x320)),
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    paVar3 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)puVar4 >> 0x10);
    in_stack_0000fff8 = SUB42(puVar4,0x0);
    ppcVar1 = (code **)((int)*puVar4 + 0x8);
    iVar2 = (**ppcVar1)(0x1010,in_stack_0000fff8,(int)((u32)puVar4 >> 0x10),param_4);
    if (iVar2 == 0x0) break;
    iStack4 += 0x1;
  }
  return iVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1008_7056(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  u32 *puVar4;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;
  let mut iStack4: i16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iStack4 = 0x0;
  while( true ) {
    if ((int)PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fff8,(iStack4 * 0x2 + 0x320)),
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    paVar3 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)puVar4 >> 0x10);
    in_stack_0000fff8 = SUB42(puVar4,0x0);
    ppcVar1 = (code **)((int)*puVar4 + 0xc);
    iVar2 = (**ppcVar1)(0x1010,in_stack_0000fff8,(int)((u32)puVar4 >> 0x10),param_4);
    if (iVar2 == 0x0) break;
    iStack4 += 0x1;
  }
  return iVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 write_to_file_1008_70a6(astruct_802 *struct802_param_1)

{
  HFILE16 hfile_1;
  let mut uVar1: u16;
  let mut i16_var2: i16;
  astruct_802 *i16_var4;
  let mut i16_var5: u16;
  let mut i16_var6: u16;
  u8 in_AF;
  let mut uVar2: u32;
  HFILE16 hfile_2;

  i16_var5 = ((u32)struct802_param_1 >> 0x10);
  i16_var4 = (astruct_802 *)struct802_param_1;
  if (i16_var4->hfile_0x4 != 0xffff) {
    _lclose16(i16_var4->hfile_0x4);
    i16_var4->hfile_0x4 = 0xffff;
  }
  hfile_2 = 0x0;
  hfile_1 = _lcreat16(0x0,struct802_param_1->filename_0x0);
  i16_var4->hfile_0x4 = hfile_1;
  if (hfile_1 == 0xffff) {
    u16_1050_0310 = 0x6cf;
  }
  else {
    u16_1050_0312 = 0x4;
    sys_1000_3f9c((char *)s__1050_65a0,_PTR_s_SC_03d_1050_0314_1050_031c,0x4);
    uVar1 = str_op_1000_3da4((char *)s__1050_65a0);
    i16_var2 = (int)uVar1 >> 0xf;
    uVar2 = _hwrite16(CONCAT22(0x65a0,i16_var2),(u8 *)CONCAT22(i16_var4->hfile_0x4,0x1050),hfile_2);
    if (uVar2 == CONCAT22(uVar1,i16_var2)) {
      return 0x1;
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Restarted to delay deadcode elimination for space: stack

BOOL16 read_file_1008_7146(mut param_1: u16 ,astruct_806 *struct_param_1,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  HFILE16 hfile_1;
  let mut uVar1: u16;
  astruct_806 *struct_1;
  let mut uVar2: u16;

  uVar2 = ((u32)struct_param_1 >> 0x10);
  struct_1 = (astruct_806 *)struct_param_1;
  if (struct_1->hfile_0x4 != 0xffff) {
    _lclose16(struct_1->hfile_0x4);
    struct_1->hfile_0x4 = 0xffff;
  }
  hfile_1 = _lopen16(0x0,struct_param_1->path_0x0);
  struct_1->hfile_0x4 = hfile_1;
  if (hfile_1 == 0xffff) {
    u16_1050_0310 = 0x6cf;
  }
  else {
    uVar1 = read_file_1008_71a0((astruct_806 *)((u32)struct_param_1 & 0xffff),param_1);
    if (uVar1 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 read_file_1008_71a0(param_1: *mut astruct_806,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  i32 iVar3;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut iStack22: i16;
  char local_e [0x9];
  u8 uStack5;
  let mut uStack4: u16;

  uStack4 = 0x1;
  uVar1 = str_op_1000_3da4((char *)s__1050_65a0);
  iStack22 = 0x0;
  iVar3 = WIN16_hread((long)(int)uVar1,(void *)CONCAT22(0x1050,local_e),*(HFILE16 *)((int)param_1 + 0x4));
  uVar2 = iVar3;
  if ((int)uVar1 < iVar3) {
    uVar2 = uVar1;
  }
  iStack24 = uVar2 - 0x2;
  if (iStack24 < 0x0) {
    iStack24 = 0x0;
  }
  iStack26 = 0x2;
  while (iStack24 != 0x0) {
    iStack22 = iStack22 * 0xa + local_e[iStack26] + -0x30;
    iStack26 += 0x1;
    iStack24 = iStack24 + -0x1;
  }
  if (iStack22 == 0x1) {
    u16_1050_0312 = 0x1;
  }
  else if (iStack22 == 0x4) {
    u16_1050_0312 = 0x4;
  }
  else {
    uStack5 = '\0';
    u16_1050_0312 = 0x1;
    uStack4 = 0x0;
  }
  sys_1000_3f9c((char *)s__1050_65a0,_PTR_s_SC_03d_1050_0314_1050_031c,u16_1050_0312);
  return uStack4;
}



BOOL16 file_fn_1008_726c(param_1: *mut astruct_802,mut param_2: u16 )

{
  HFILE16 hfile_1;

  if (param_1->hfile_0x4 != 0xffff) {
    hfile_1 = _lclose16(param_1->hfile_0x4);
    if (hfile_1 == 0xffff) {
      u16_1050_0310 = 0x6d1;
      return 0x0;
    }
    param_1->hfile_0x4 = 0xffff;
    u16_1050_0310 = 0x0;
  }
  return 0x1;
}



u16 * pass1_1008_72a8(param_1: *mut u16,mut param_2: u16 )

{
  *param_1 = param_2;
  return param_1;
}



u16 switch_1008_72bc(HFILE16 *param_1,mut param_2: u16 )

{
  if ((int)u16_1050_0312 < 0x2) {
    switch(param_2) {
    case 0x1:
      param_2 = 0x1;
      break;
    case 0x2:
      param_2 = 0x2;
      break;
    case 0x3:
      param_2 = 0x3;
      break;
    case 0x4:
      param_2 = 0x5;
      break;
    case 0x5:
      param_2 = 0x4;
      break;
    case 0x6:
      param_2 = 0x6;
      break;
    case 0x7:
      param_2 = 0x7;
      break;
    case 0x8:
      param_2 = 0x8;
      break;
    case 0x9:
      param_2 = 0x9;
      break;
    case 0xa:
      param_2 = 0xa;
      break;
    case 0xb:
      param_2 = 0xb;
      break;
    case 0xc:
      param_2 = 0xc;
      break;
    case 0xd:
      param_2 = 0xd;
      break;
    case 0xe:
      param_2 = 0xe;
      break;
    case 0xf:
      param_2 = 0xf;
      break;
    case 0x10:
      return 0x10;
    case 0x11:
      return 0x11;
    case 0x12:
      return 0x12;
    case 0x13:
      return 0x13;
    default:
      return 0x0;
    }
  }
  return param_2;
}



u16 pass1_1008_738c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u16;

  switch(param_3) {
  case 0x1:
    uVar1 = 0x3;
    break;
  case 0x2:
    uVar1 = 0x4;
    break;
  case 0x3:
    return 0x5;
  case 0x4:
    return 0x6;
  case 0x5:
    return 0x8;
  case 0x6:
    return 0x9;
  case 0x7:
    return 0xa;
  default:
    uVar1 = 0x0;
  }
  return uVar1;
}



i16 switch_1008_73ea(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut iStack4: i16;

  iStack4 = param_3;
  if ((int)u16_1050_0312 < 0x2) {
    switch(param_3) {
    case 0x18:
    case 0x19:
    case 0x1a:
    case 0x1b:
    case 0x1c:
    case 0x1d:
    case 0x1e:
    case 0x1f:
    case 0x20:
    case 0x21:
    case 0x22:
    case 0x23:
    case 0x24:
    case 0x25:
    case 0x26:
    case 0x27:
    case 0x28:
    case 0x29:
    case 0x2a:
    case 0x2b:
    case 0x2c:
    case 0x2d:
    case 0x2e:
    case 0x2f:
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
    case 0x3a:
    case 0x3b:
    case 0x3c:
      iStack4 = param_3 + 0x3;
      break;
    case 0x3d:
    case 0x3e:
      iStack4 = param_3 + 0x4;
      break;
    case 0x3f:
    case 0x40:
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x48:
    case 0x49:
    case 0x4a:
    case 0x4b:
    case 0x4c:
    case 0x4d:
    case 0x4e:
    case 0x4f:
    case 0x50:
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
    case 0x5b:
    case 0x5c:
    case 0x5d:
    case 0x5e:
    case 0x5f:
    case 0x60:
    case 0x61:
    case 0x62:
    case 0x63:
    case 0x64:
    case 0x65:
    case 0x66:
      iStack4 = param_3 + 0x8;
      break;
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6a:
    case 0x6b:
    case 0x6c:
    case 0x6d:
    case 0x6e:
    case 0x6f:
    case 0x70:
    case 0x71:
    case 0x72:
    case 0x73:
    case 0x74:
    case 0x75:
    case 0x76:
    case 0x77:
    case 0x78:
    case 0x79:
    case 0x7a:
    case 0x7b:
    case 0x7c:
    case 0x7d:
    case 0x7e:
    case 0x7f:
    case 0x80:
      iStack4 = param_3 + 0x9;
    }
  }
  return iStack4;
}
pub fn file_1008_7548(HFILE16 *hfile_param,i32 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  let mut file_read_ok:bool;
  let mut uVar2: u16;
  u8 *buffer_3;
  let mut uVar3: u16;
  let mut uVar4: u16;
  astruct_57 *buffer_4;
  let mut unaff_CS: u16;
  i32 lVar5;
  u8 *read_buffer_1c;
  u16 local_18 [0x5];
  u8 *puStack14;
  let mut uStack10: u32;
  u8 *read_buffer;

  uVar4 = ((u32)param_3 >> 0x10);
  read_buffer = NULL;
  file_read_ok = read_file_1008_7dee(hfile_param,(u8 *)CONCAT22(0x1050,&read_buffer),0x4);
  if (file_read_ok == 0x0) {
    return;
  }
  if (read_buffer != NULL) {
    buffer_3 = read_buffer;
    if (read_buffer < (u8 *)0xc8) {
      read_buffer = 0x0;
      buffer_3 = (u8 *)0xc8;
    }
    buffer_4 = (astruct_57 *)CONCAT22(uVar4,read_buffer);
    uVar2 = buffer_3;
    uStack10 = (u32)buffer_3 & 0xffff | (u32)read_buffer << 0x10;
    if (*param_2 == 0x0) {
      unaff_CS = 0x1000;
      mem_op_1000_179c(0x1e,buffer_4);
      uVar3 = buffer_4 | uVar2;
      if (uVar3 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        unaff_CS = 0x1020;
        struct_1020_c444((astruct_75 *)CONCAT22(buffer_4,uVar2),0x64,uStack10);
        param_2 = uVar2;
        ((int)param_2 + 0x2) = uVar3;
      }
    }
    lVar5 = *param_2;
    ppcVar1 = (code **)((int)(u32)*param_2 + 0x24);
    (**ppcVar1)();
    for (puStack14 = NULL; puStack14 < read_buffer; puStack14 = puStack14 + 0x1) {
      file_read_ok = read_file_1008_7dee(hfile_param,(u8 *)CONCAT22(0x1050,&read_buffer_1c),0x4);
      if ((file_read_ok == 0x0) ||
         (file_read_ok = read_file_1008_7dee(hfile_param,(u8 *)CONCAT22(0x1050,local_18),0x2), file_read_ok == 0x0)) {
        ppcVar1 = (code **)((int)(u32)*param_2 + 0x1c);
        (**ppcVar1)(unaff_CS,*param_2);
        return;
      }
      ppcVar1 = (code **)((int)(u32)*param_2 + 0x28);
      (**ppcVar1)(unaff_CS,(int)*param_2,(char)((u32)*param_2 >> 0x10),local_18[0],read_buffer_1c);
    }
    ppcVar1 = (code **)((int)(u32)*param_2 + 0x1c);
    (**ppcVar1)(unaff_CS,*param_2,lVar5);
  }
  return;
}
pub fn pass1_1008_766e(u8 *param_1,mut param_2: u32,param_3: *mut astruct_169)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffe4: u16;
  let mut local_6: u32;
  astruct_57 *paVar5;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  (u32)param_3 = 0x0;
  local_6 = 0x0;
  puVar1 = &local_6;
  file_1008_76e4(paVar4,(HFILE16 *)param_2,(i32 *)CONCAT22(0x1050,puVar1));
  if (puVar1 != NULL) {
    if (local_6 != 0x0) {
      mem_op_1000_179c(0xc,paVar4);
      uVar2 = paVar4 | puVar1;
      paVar5 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)uVar2);
      if (uVar2 == 0x0) {
        puVar1 = NULL;
        uVar3 = 0x0;
      }
      else {
        pass1_1010_8ef2(paVar5,(astruct_170 *)CONCAT22(paVar4,puVar1),in_stack_0000ffe4,in_stack_0000fe8c,
                        in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
        uVar3 = SUB42(paVar5,0x0);
      }
      (u32*)param_3 = puVar1;
      ((int)param_3 + 0x2) = uVar3;
      fn_ptr_1010_905e(*(astruct_169 **)param_3,local_6);
    }
    return;
  }
  return;
}
pub fn file_1008_76e4(param_1: *mut astruct_57,HFILE16 *param_2,i32 *param_3)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3:bool;
  let mut uVar4: u16;
  u8 local_18 [0xe];
  u8 *puStack10;
  u8 *buffer_6;

  buffer_6 = NULL;
  uVar2 = read_file_1008_7dee(param_2,(u8 *)CONCAT22(0x1050,&buffer_6),0x4);
  if (uVar2 == 0x0) {
    return;
  }
  if (buffer_6 != NULL) {
    if (*param_3 == 0x0) {
      mem_op_1000_179c(0x18,param_1);
      uVar4 = param_1 | uVar2;
      if (uVar4 == 0x0) {
        *param_3 = 0x0;
      }
      else {
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_1,uVar2),0x5,(u32)buffer_6);
        param_3 = uVar2;
        ((int)param_3 + 0x2) = uVar4;
      }
    }
    ppcVar1 = (code **)((int)(u32)*param_3 + 0x14);
    (**ppcVar1)();
    for (puStack10 = NULL; puStack10 < buffer_6; puStack10 = puStack10 + 0x1) {
      BVar3 = read_file_1008_7dee(param_2,(u8 *)CONCAT22(0x1050,local_18),0x4);
      if (BVar3 == 0x0) {
        return;
      }
      ppcVar1 = (code **)((int)(u32)*param_3 + 0x18);
      (**ppcVar1)();
    }
    ppcVar1 = (code **)((int)(u32)*param_3 + 0x1c);
    (**ppcVar1)();
  }
  return;
}



u16 file_1008_77cc(mut param_1: u16 ,mut param_2: u32,i32 *param_3)

{
  let mut uVar1: u16;
  let mut BVar2:bool;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  u16 local_14 [0x2];
  u32 local_10 [0x2];
  let mut uStack6: u16;
  let mut local_4: u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  local_4 = 0x0;
  uVar1 = read_file_1008_7dee((HFILE16 *)param_2,(u8 *)CONCAT22(0x1050,&local_4),0x2);
  if (uVar1 == 0x0) {
    return 0x0;
  }
  if (local_4 != 0x0) {
    if (*param_3 == 0x0) {
      mem_op_1000_179c(0xa,paVar4);
      uVar3 = paVar4 | uVar1;
      if (uVar3 == 0x0) {
        *param_3 = 0x0;
      }
      else {
        pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar4,uVar1),0x5,0x5);
        param_3 = uVar1;
        ((int)param_3 + 0x2) = uVar3;
      }
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 += 0x1) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_2,(u8 *)CONCAT22(0x1050,local_14),0x2);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      BVar2 = read_file_1008_7dee((HFILE16 *)param_2,(u8 *)CONCAT22(0x1050,local_10),0x4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      pass1_1020_bb8a((i32 *)*param_3,local_10[0],CONCAT22(local_14[0],(int)((u32)local_10[0] >> 0x10)));
    }
  }
  return 0x1;
}
pub fn pass1_1008_7898(mut param_1: u16 ,mut param_2: u32,u32 *param_3)

{
  code **ppcVar1;
  let mut BVar2:bool;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  HFILE16 in_stack_0000ffc4;
  let mut local_26: u16;
  u32 local_24 [0x3];
  let mut local_18: u32;
  u16 local_14 [0x5];
  let mut uStack10: u32;
  let mut uStack6: u32;

  if (param_3 == NULL) {
    param_1 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*param_3 + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_1);
  local_18 = CONCAT22(uVar3,param_1);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_18),(char *)0x4,in_stack_0000ffc4);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return;
      }
      pass1_1020_c4a8((u32)param_3,(u16 *)CONCAT22(0x1050,local_14),(u32 *)CONCAT22(0x1050,&local_18),
                      (int)uStack10);
      local_24[0] = local_18;
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_24),(char *)0x4,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_26 = local_14[0];
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_26),(char *)0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return;
      }
      uStack10 += 0x1;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



u16_t write_to_file_1008_7954(u16_t param_1,u8 *param_2,u32 *param_3)

{
  code **ppcVar1;
  let mut BVar2:bool;
  let mut uVar3: u32;
  u16_t extraout_DX;
  u16_t uVar4;
  u16_t extraout_DX_00;
  HFILE16 in_stack_0000ffca;
  u16_t local_20;
  u16_t uStack30;
  u16_t local_18;
  u16_t uStack22;
  let mut uStack10: u32;
  let mut uStack6: u32;

  if (param_3 == NULL) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*param_3 + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_1);
  local_18 = param_1;
  uStack22 = uVar4;
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_18),(char *)0x4,in_stack_0000ffca);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return uVar4;
      }
      ppcVar1 = (code **)((int)*param_3 + 0x4);
      uVar3 = uStack6;
      (**ppcVar1)();
      local_20 = (u16_t)uVar3;
      uVar4 = extraout_DX_00;
      uStack30 = extraout_DX_00;
      local_18 = local_20;
      uStack22 = extraout_DX_00;
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_20),(char *)0x4,in_stack_0000ffca);
      if (BVar2 == 0x0) break;
      uStack10 += 0x1;
    }
  }
  u16_1050_0310 = 0x6d0;
  return uVar4;
}
pub fn pass1_1008_79f0(mut param_1: u32,i32 param_2)

{
  u16_t uVar1;
  let mut uVar2: u16;
  let mut uStack4: u16;

  if (param_2 == 0x0) {
    uVar1 = 0x0;
    uStack4 = 0x0;
  }
  else {
    uVar2 = ((u32)param_2 >> 0x10);
    uVar1 = *(u16_t *)((int)param_2 + 0x4);
    uStack4 = ((int)param_2 + 0x6);
  }
  write_to_file_1008_7954(uVar1,param_1,(u32 *)CONCAT22(uStack4,uVar1));
  return;
}
pub fn write_to_file_1008_7a22(u8 *param_1,i32 param_2)

{
  let mut BVar1:bool;
  HFILE16 in_stack_0000ffc6;
  u32 local_24 [0x2];
  u16 local_1c [0x5];
  let mut local_12: u16;
  let mut local_10: u32;
  let mut uStack10: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  if (param_2 == 0x0) {
    uStack4 = 0x0;
  }
  else {
    uStack4 = ((int)param_2 + 0x4);
  }
  local_12 = uStack4;
  BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,&local_12),(char *)0x2,in_stack_0000ffc6);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d0;
  }
  else {
    uStack6 = 0x0;
    while( true ) {
      if (uStack4 <= uStack6) {
        return;
      }
      pass1_1020_bb16((u32 *)param_2,(u32 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,&local_12),uStack6)
      ;
      uStack10 = local_12;
      local_1c[0] = local_12;
      BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffc6);
      if (BVar1 == 0x0) break;
      local_24[0] = local_10;
      BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_24),(char *)0x4,in_stack_0000ffc6);
      if (BVar1 == 0x0) {
        return;
      }
      uStack6 += 0x1;
    }
  }
  return;
}



u16 pass1_1008_7ad4(mut param_1: u32,i32 *param_2)

{
  let mut BVar1:bool;
  u16 local_14 [0x2];
  u32 local_10 [0x2];
  let mut uStack6: u16;
  let mut local_4: u16;

  BVar1 = read_file_1008_7dee((HFILE16 *)param_1,(u8 *)CONCAT22(0x1050,&local_4),0x2);
  if (BVar1 != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      if (local_4 <= uStack6) {
        return 0x1;
      }
      BVar1 = read_file_1008_7dee((HFILE16 *)param_1,(u8 *)CONCAT22(0x1050,local_14),0x2);
      if ((BVar1 == 0x0) ||
         (BVar1 = read_file_1008_7dee((HFILE16 *)param_1,(u8 *)CONCAT22(0x1050,local_10),0x4), BVar1 == 0x0)) break;
      pass1_1020_bb8a(param_2,local_10[0],CONCAT22(local_14[0],(int)((u32)local_10[0] >> 0x10)));
      uStack6 += 0x1;
    }
  }
  return 0x0;
}



u16 write_to_file_1008_7b4c(u8 *param_1,param_2: *mut astruct_615)

{
  let mut BVar1:bool;
  HFILE16 in_stack_0000ffd4;
  u16 local_12 [0x3];
  u16 local_c [0x2];
  let mut local_8: u16;
  let mut local_6: u16;
  let mut local_4: u16;

  pass1_1008_3eb4(param_2,(u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,&local_4));
  local_12[0] = local_4;
  BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_12),(char *)0x2,in_stack_0000ffd4);
  if (BVar1 != 0x0) {
    local_c[0] = local_6;
    BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffd4);
    if (BVar1 != 0x0) {
      local_c[0] = local_8;
      BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffd4);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 read_file_1008_7bc8(mut param_1: u32,param_2: *mut u16)

{
  let mut BVar1:bool;
  let mut local_8: u16;
  let mut local_6: u32;

  BVar1 = read_file_1008_7dee((HFILE16 *)param_1,(u8 *)CONCAT22(0x1050,(int)&local_6 + 0x2),0x2);
  if (BVar1 != 0x0) {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_1,(u8 *)CONCAT22(0x1050,&local_6),0x2);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee((HFILE16 *)param_1,(u8 *)CONCAT22(0x1050,&local_8),0x2);
      if (BVar1 != 0x0) {
        pass1_1008_3e76(param_2,local_8,local_6,((u32)local_6 >> 0x10));
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 pass1_1008_7c2a(mut param_1: u32,char *param_2)

{
  let mut uVar1: u16;
  let mut BVar2:bool;
  HFILE16 in_stack_0000ffe6;

  if (param_2 != NULL) {
    uVar1 = str_op_1000_3da4(param_2);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_1,(u32)param_2,(char *)(long)(int)(uVar1 + 0x1),in_stack_0000ffe6);
    return BVar2;
  }
  write_to_file_1008_7e1c((u8 *)param_1,(u32)(s_playerName_1050_148e + 0xc),(char *)0x1,in_stack_0000ffe6);
  return 0x1;
}
pub fn read_file_1008_7c6e(HFILE16 param_1,mut param_2: u16 ,char *param_3)

{
  char *pcVar1;
  char local_c [0xa];

  while( true ) {
    pcVar1 = param_3;
    WIN16_hread(0x1,(void *)CONCAT22(0x1050,local_c),*_param_1);
    if (local_c[0] == '\0') break;
    param_3 = (char *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x1));
    *pcVar1 = local_c[0];
  }
  *param_3 = '\0';
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 write_to_file_1008_7cac(u8 *param_1)

{
  let mut uVar1: u16;
  let mut BVar2:bool;
  HFILE16 in_stack_0000ffde;
  u8 local_c [0xa];

  sys_1000_3f9c((char *)CONCAT22(0x1050,local_c),s__s_02x_1050_0340,_PTR_s_dcbSC_1050_0336_1050_033c);
  uVar1 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_c));
  BVar2 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_c),(char *)(u32)uVar1,in_stack_0000ffde);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return BVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn read_file_1008_7cfe(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut bVar1: bool;
  let mut uVar2: u16;
  i32 iVar3;
  let mut in_stack_0000fbd6: u16;
  u16_t in_stack_0000fbd8;
  let mut uStack1040: u32;
  char local_406 [0x400];
  let mut uStack6: u32;

  uStack6 = 0x0;
  bVar1 = false;
  do {
    _llseek16(0x0,uStack6,*(HFILE16 *)CONCAT22(param_2,param_1));
    iVar3 = WIN16_hread(0x400,(void *)CONCAT22(0x1050,local_406),*(HFILE16 *)CONCAT22(param_2,param_1));
    for (uStack1040 = 0x0; uStack1040 < iVar3; uStack1040 += 0x1) {
      if (local_406[uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
        if (!bVar1) {
          bVar1 = true;
          uStack6 = CONCAT22((int)((u32)uStack6 >> 0x10) + uStack1040 +
                             CARRY2(uStack6,uStack1040),uStack6 + uStack1040);
          break;
        }
        bVar1 = false;
        uVar2 = pass1_1008_7e4a((char *)CONCAT22(0x1050,local_406 + uStack1040),in_stack_0000fbd6,
                                in_stack_0000fbd8);
        if (uVar2 != 0x0) {
          _llseek16(0x0,uStack1040 + uStack6 + 0x7,*(HFILE16 *)CONCAT22(param_2,param_1));
          return;
        }
      }
    }
    if (!bVar1) {
      if (iVar3 < 0x400) {
        return;
      }
      uStack6._0_2_ = CONCAT11(uStack6._1_1_ + 0x4,(u8)uStack6);
      uStack6 = CONCAT22((int)((u32)uStack6 >> 0x10) + (0xfb < uStack6._1_1_),uStack6);
    }
  } while( true );
}



BOOL16 read_file_1008_7dee(HFILE16 *hfile_param_1,u8 *buffer_param_2,u32 count_param_3)

{
  i32 read_count;

  read_count = WIN16_hread(count_param_3,buffer_param_2,*hfile_param_1);
  if (((int)read_count == (int)count_param_3) && ((int)((u32)read_count >> 0x10) == count_param_3)) {
    return 0x1;
  }
  return 0x0;
}



BOOL16 write_to_file_1008_7e1c(u8 *buffer,u32 count,char *buf_to_write,HFILE16 param_4)

{
  let mut uVar1: u32;
  let mut uStackY16: u16;
  let mut uVar2: u16;

  uStackY16 = SUB42(buf_to_write,0x0);
  uVar2 = ((u32)buf_to_write >> 0x10);
  uVar1 = _hwrite16(CONCAT22((int)count,uVar2),(u8 *)CONCAT22(buffer,(int)(count >> 0x10)),param_4);
  if (uVar1 != CONCAT22(uStackY16,uVar2)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1008_7e4a(char *param_1,mut param_2: u16 ,u16_t param_3)

{
  let mut uVar1: u16;

  sys_1000_3f9c((char *)CONCAT22(0x1050,&param_2),s__s_02x_1050_0347,_PTR_s_dcbSC_1050_0336_1050_033c);
  uVar1 = str_op_1000_3da4((char *)CONCAT22(0x1050,&param_2));
  uVar1 = pass1_1000_3de8(param_1,(char *)CONCAT22(0x1050,&param_2),uVar1,param_2,param_3);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}



u16 * pass1_1008_7e98(param_1: *mut astruct_460,u8 param_2)

{
  astruct_460 *uVar1;
  astruct_460 *uVar2;

  uVar2 = (astruct_460 *)((u32)param_1 >> 0x10);
  uVar1 = (astruct_460 *)param_1;
  param_1 = 0x380a;
  uVar1->field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return (u16 *)param_1;
}



astruct_20 * unk_draw_op_1008_7f62(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 HVar2;
  astruct_20 *iVar4;
  let mut uVar3: u16;
  astruct_20 *uVar4;
  astruct_20 *iVar3;

  set_struct_1008_687a(param_1,param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  iVar4->field164_0xde = param_2;
  param_1->offset_0x0 = 0x8042;
  iVar4->base_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field60_0x5b)),s_SOLChildPar_1050_0358);
  HVar1 = GetStockObject16(HOLLOW_BRUSH);
  iVar4->hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16((char *)0x7f00,0x0);
  iVar4->hcursor_field_0xc4 = HVar2;
  iVar4->field150_0xc8 = 0x2008;
  iVar4->field139_0xac = 0x44000000;
  iVar4->field145_0xbc = ((int)param_3 + 0x8);
  iVar4->field151_0xca = iVar4->field164_0xde;
  win_ui_reg_class_1008_96d2((StructA *)param_1);
  return param_1;
}
pub fn pass1_1008_7ffa(param_1: *mut astruct_461,u8 param_2)

{
  astruct_461 *uVar1;
  let mut uVar2: u16;

  uVar1 = (astruct_461 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = ((u32)param_1 >> 0x10);
  param_1 = 0x380a;
  uVar1->field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



u32 * pass1_1008_80d2(u32 *param_1)

{
  *param_1 = 0x0;
  ((int)param_1 + 0x4) = 0x0;
  return param_1;
}



astruct_23 * unk_draw_op_1008_80ee(param_1: *mut astruct_23)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  astruct_23 *iVar3;
  astruct_23 *uVar3;

  uVar3 = (astruct_23 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_23 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field79_0x54 = 0x0;
  iVar3->field80_0x56 = 0x0;
  iVar3->field81_0x58 = 0x0;
  param_1->field0_0x0 = 0x87c8;
  iVar3->field1_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field2_0x4)),s_MicroSpinControl_1050_0370);
  iVar3->field79_0x54 = 0x3;
  HVar1 = LoadCursor16((char *)0x7f00,0x0);
  iVar3->field81_0x58 = HVar1;
  HVar2 = GetStockObject16(0x4);
  iVar3->field80_0x56 = HVar2;
  pass1_1008_818c((astruct_23 *)((u32)param_1 & 0xffff | ZEXT24(uVar3) << 0x10));
  return param_1;
}
pub fn pass1_1008_8168(char *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1 = 0x87c8;
  ((int)param_1 + 0x2) = 0x1008;
  param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1008_818c(param_1: *mut astruct_23)

{
  let mut BVar1:bool;
  ATOM AVar2;
  WNDCLASS16 wndclass;

  wndclass.lpsz_class_name._0_2_ = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(&wndclass,(char *)CONCAT22((int)wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0x0) {
    wndclass.style = ((int)param_1 + 0x54);
    wndclass.lpfn_wnd_proc._0_2_ = 0x84f2;
    wndclass.lpfn_wnd_proc = 0x1008;
    wndclass._6_4_ = 0x40000;
    wndclass.h_instance = HINSTANCE16_1050_038c;
    wndclass.h_icon = 0x0;
    wndclass.h_cursor = *(HCURSOR16 *)((int)param_1 + 0x58);
    wndclass.hbr_background = *(HBRUSH16 *)((int)param_1 + 0x56);
    wndclass.lpsz_menu_name = 0x0;
    wndclass.lpsz_class_name = param_1;
    AVar2 = RegisterClass16(&wndclass);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0);
    }
  }
  return;
}



u16 win_ui_op_1008_8214(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u16 ,mut param_6: u32)

{
  let mut uVar1: u16;
  INT16 IVar2;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  u32 *puVar4;
  let mut puVar5: *mut u16;
  let mut offset: i16;
  let mut hwnd: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (param_4 == 0x81) {
    offset = 0x0;
    hwnd = param_3;
    mem_op_1000_179c(0x6,paVar3);
    if ((paVar3 | param_1) == 0x0) {
      uVar1 = 0x0;
      puVar4 = NULL;
    }
    else {
      puVar4 = pass1_1008_80d2((u32 *)CONCAT22(paVar3,param_1));
      uVar1 = puVar4;
    }
    SetWindowLong16((u32)puVar4 & 0xffff0000 | (u32)uVar1,offset,hwnd);
  }
  if (param_4 == 0x1) {
    puVar5 = (u16 *)GetWindowLong16(0x0,param_3);
    *puVar5 = ((int)param_6 + 0x8);
    IVar2 = GetDlgCtrlID16(param_3);
    *(INT16 *)((int)puVar5 + 0x2) = IVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
pub fn draw_op_1008_8288(mut param_1: u16 ,mut param_2: u32,HDC16 hdc16_param_1,mut param_4: u32)

{
  HDC16 paint_handle_1;
  HPEN16 pen_handle_1;
  HPEN16 pen_handle_3;
  HBRUSH16 brush_handle_1;
  HGDIOBJ16 hgdiobj16_var1;
  let mut y: u16;
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  HDC16 hdc;
  let mut right_00: u16;
  HGDIOBJ16 obj;
  u8 paintstruct_3c [0x20];
  POINT16 point_1c;
  let mut iStack24: i16;
  let mut iStack22: i16;
  let mut iStack20: i16;
  let mut iStack18: i16;
  POINT16 local_10;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;
  let mut in_stack_0000ffa6: u32;
  let mut left_01: i16;
  let mut top: i16;
  let mut left: i16;
  let mut x2: u16;
  let mut uVar1: u32;
  let mut right: u16;
  let mut in_stack_0000ffae: u16;
  let mut bottom: u16;

  paint_handle_1 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_3c),hdc16_param_1);
  uStack4 = 0x0;
  pen_handle_1 = CreatePen16(COLORREF_1050_0368,0x1,0x0);
  pen_handle_3 = CreatePen16(COLORREF_1050_0364,0x1,0x0);
  brush_handle_1 = CreateSolidBrush16(COLORREF_1050_0364);
  uVar1 = CONCAT22(pen_handle_3,brush_handle_1);
  hdc = hdc16_param_1;
  GetClientRect16((RECT16 *)((int)&param_2 + 0x2),(HWND16)&DAT_1050_1050);
  y = param_1 >> 0x1;
  right_00 = x2;
  hgdiobj16_var1 = GetStockObject16(BLACK_PEN);
  hgdiobj16_var1 = SelectObject16(hgdiobj16_var1,hdc);
  param_2 = param_2 & 0xffff0000 | (u32)hgdiobj16_var1;
  hgdiobj16_var1 = GetStockObject16(BLACK_BRUSH);
  SelectObject16(hgdiobj16_var1,left);
  Rectangle16(param_1,right_00,top,(INT16)(param_2 >> 0x10),paint_handle_1);
  MoveTo16(y,0x0,paint_handle_1);
  param_2 = param_2 & 0xffff0000 | (u32)paint_handle_1;
  LineTo16(y,x2,paint_handle_1);
  uVar3 = (param_4 >> 0x10);
  if ((*(u8 *)((int)param_4 + 0x4) & 0x4) != 0x0) {
    uStack4 = 0x1;
  }
  local_10.x = (x2 >> 0x1) + uStack4;
  iVar1 = (param_1 >> 0x2) + uStack4;
  local_10.y = iVar1 + -0x2;
  iStack12 = local_10.x + -0x3;
  iStack10 = iVar1 + 0x1;
  iStack8 = local_10.x + 0x3;
  iStack6 = iStack10;
  param_2._0_2_ = pen_handle_1;
  param_2 = paint_handle_1;
  SelectObject16(pen_handle_1,paint_handle_1);
  if (uStack4 == 0x0) {
    param_2 = CONCAT22(paint_handle_1,0x1);
    MoveTo16(y - 0x2,0x1,paint_handle_1);
    param_2 = 0x10001;
    LineTo16(0x1,0x1,paint_handle_1);
    param_2 = 0x1;
    param_2._0_2_ = (HPEN16)s_tile2_bmp_1050_1538;
    LineTo16(0x1,x2 - 0x1,paint_handle_1);
  }
  uStack4 = ((*(u8 *)((int)param_4 + 0x4) & 0x8) != 0x0);
  point_1c.x = (x2 >> 0x1) + uStack4;
  iVar2 = (param_1 - (param_1 >> 0x2)) + uStack4;
  point_1c.y = iVar2 + 0x1;
  iStack24 = point_1c.x + -0x3;
  iStack22 = iVar2 + -0x2;
  iStack20 = point_1c.x + 0x3;
  iStack18 = iStack22;
  if (uStack4 == 0x0) {
    param_2 = 0x15388429;
    MoveTo16(paint_handle_1 - 0x2,0x1,paint_handle_1);
    param_2 = 0x843a;
    LineTo16(y + 0x1,0x1,paint_handle_1);
    uVar1 = CONCAT22(paint_handle_1,x2 - 0x1);
    LineTo16(y + 0x1,x2 - 0x1,paint_handle_1);
  }
  param_2 = CONCAT22(0x8453,(HPEN16)param_2);
  SelectObject16((HGDIOBJ16)(uVar1 >> 0x10),paint_handle_1);
  param_2 = CONCAT22(0x845e,(HPEN16)param_2);
  SelectObject16((HGDIOBJ16)uVar1,paint_handle_1);
  obj = (HGDIOBJ16)(uVar1 >> 0x10);
  param_2 = 0x31538;
  Polygon16(0x3,&local_10,(HDC16)&DAT_1050_1050);
  param_2 = 0x31538;
  hgdiobj16_var1 = 0x847c;
  Polygon16(0x3,&point_1c,(HDC16)&DAT_1050_1050);
  param_2 = CONCAT22(0x8487,(HPEN16)param_2);
  SelectObject16(hgdiobj16_var1,paint_handle_1);
  param_2 = CONCAT22(0x8492,(HPEN16)param_2);
  SelectObject16((HPEN16)param_2,paint_handle_1);
  DeleteObject16(pen_handle_1);
  DeleteObject16(obj);
  DeleteObject16(obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_3c),hdc16_param_1);
  return;
}
pub fn send_msg_1008_84ba(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  WPARAM16 WStack4;

  uVar2 = (param_2 >> 0x10);
  iVar1 = (int)param_2;
  if ((*(u8 *)(iVar1 + 0x4) & 0x4) == 0x0) {
    if ((*(u8 *)(iVar1 + 0x4) & 0x8) == 0x0) {
      return;
    }
    WStack4 = 0x1;
  }
  else {
    WStack4 = 0x0;
  }
  SendMessage16((u32)(iVar1 + 0x2),WStack4,0x115,*(HWND16 *)param_2);
  return;
}
pub fn win_sys_op_1008_84f2(LPARAM lparam_param_1,WPARAM16 wparam_param_2,u16 msg_param_3,HWND16 hwnd_param_4)

{
  let mut puVar1: *mut u16;
  char cVar2;
  let mut BVar3:bool;
  let mut uVar4: u16;
  i32 win_long_1;
  let mut in_stack_0000ff7c: u32;
  let mut in_stack_0000ff84: u16;
  RECT16 rect_a;
  let mut iStack4: i16;
  astruct_863 *iVar3;

  win_long_1 = GetWindowLong16(0x0,hwnd_param_4);
  win_long_1 = ((u32)win_long_1 >> 0x10);
  iVar3 = (astruct_863 *)win_long_1;
  if (msg_param_3 == 0x1f) {
    iVar3->field3_0x4 = 0x0;
    KillTimer16(0xfa6,hwnd_param_4);
    KillTimer16(0xfa7,hwnd_param_4);
    ReleaseCapture16();
  }
  else if (msg_param_3 < 0x20) {
    if (msg_param_3 != 0x14) {
      if (0x14 < msg_param_3) goto LAB_1008_8771;
      cVar2 = (char)msg_param_3;
      uVar4 = msg_param_3 & 0xff00 | (u8)(cVar2 - 0x1U);
      if ((u8)(cVar2 - 0x1U) == 0x0) {//
LAB_1008_8560:
        win_ui_op_1008_8214(uVar4,win_long_1,hwnd_param_4,msg_param_3,wparam_param_2,lparam_param_1);
        return;
      }
      if (cVar2 == '\x02') {
        fn_ptr_1000_17ce((char *)win_long_1);
      }
      else if (cVar2 != '\f') {
        if (cVar2 != '\x0f') goto LAB_1008_8771;
        draw_op_1008_8288(in_stack_0000ff84,in_stack_0000ff7c,hwnd_param_4,win_long_1);
      }
    }
  }
  else if (msg_param_3 == 0x200) {
    if ((*(u8 *)&iVar3->field3_0x4 & 0x1) != 0x0) {
      GetClientRect16(&rect_a,(HWND16)&DAT_1050_1050);
      uVar4 = iVar3->field3_0x4;
      puVar1 = &iVar3->field3_0x4;
      *(u8 *)puVar1 = *(u8 *)puVar1 & 0xf3;
      BVar3 = PtInRect16((POINT16)lparam_param_1,&rect_a);
      if (BVar3 == 0x0) {
        puVar1 = &iVar3->field3_0x4;
        *(u8 *)puVar1 = *(u8 *)puVar1 | 0x2;
      }
      else {
        if ((int)lparam_param_1 < iStack4 >> 0x1) {
          puVar1 = &iVar3->field3_0x4;
          *(u8 *)puVar1 = *(u8 *)puVar1 | 0x4;
        }
        else {
          puVar1 = &iVar3->field3_0x4;
          *(u8 *)puVar1 = *(u8 *)puVar1 | 0x8;
        }
        puVar1 = &iVar3->field3_0x4;
        *(u8 *)puVar1 = *(u8 *)puVar1 & 0xfd;
      }
      if (iVar3->field3_0x4 != uVar4) {
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
      }
    }
  }
  else if (msg_param_3 < 0x201) {
    uVar4 = msg_param_3 - 0x81;
    if (uVar4 == 0x0) goto LAB_1008_8560;
    if (msg_param_3 != 0x113) {//
LAB_1008_8771:
      DefWindowProc16(lparam_param_1,wparam_param_2,msg_param_3,hwnd_param_4);
      return;
    }
    if (wparam_param_2 == 0xfa6) {
      KillTimer16(0xfa6,hwnd_param_4);
      SetTimer16(NULL,0x1,0xfa7,hwnd_param_4);
    }
    if ((*(u8 *)&iVar3->field3_0x4 & 0x2) == 0x0) {
      send_msg_1008_84ba(hwnd_param_4,win_long_1);
    }
  }
  else {
    if (msg_param_3 != 0x201) {
      if (msg_param_3 == 0x202) {
        KillTimer16(0xfa6,hwnd_param_4);
        KillTimer16(0xfa7,hwnd_param_4);
        ReleaseCapture16();
        uVar4 = iVar3->field3_0x4;
        if (((uVar4 & 0x1) != 0x0) && ((uVar4 & 0xfffd) != 0x0)) {
          puVar1 = &iVar3->field3_0x4;
          *(u8 *)puVar1 = *(u8 *)puVar1 & 0xf2;
          InvalidateRect16(0x1,NULL,0x0);
          UpdateWindow16(hwnd_param_4);
        }
        SendMessage16((u32)iVar3->field2_0x2,0xf9,0x111,*(HWND16 *)win_long_1);
        return;
      }
      if (msg_param_3 != 0x203) goto LAB_1008_8771;
    }
    puVar1 = &iVar3->field3_0x4;
    *(u8 *)puVar1 = *(u8 *)puVar1 | 0x1;
    GetClientRect16(&rect_a,(HWND16)&DAT_1050_1050);
    if (lparam_param_1 < (iStack4 >> 0x1)) {
      puVar1 = &iVar3->field3_0x4;
      *(u8 *)puVar1 = *(u8 *)puVar1 | 0x4;
    }
    else {
      puVar1 = &iVar3->field3_0x4;
      *(u8 *)puVar1 = *(u8 *)puVar1 | 0x8;
    }
    send_msg_1008_84ba(hwnd_param_4,win_long_1);
    SetTimer16(NULL,0x12c,0xfa6,hwnd_param_4);
    InvalidateRect16(0x1,NULL,0x0);
    UpdateWindow16(hwnd_param_4);
    SetCapture16(hwnd_param_4);
  }
  return;
}



pub fn pass1_1008_87a2(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_8168((char *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_87cc(param_1: *mut astruct_86,mut param_2: i16,mut param_3: i16,mut param_4: u16 ,param_5: *mut astruct_76,mut param_6: u32,
                    mut param_7: u32)

{
  i32 lVar1;
  let mut uVar2: u16;
  let mut BVar3:bool;
  let mut piVar4: *mut i16;
  let mut uVar5: u16;
  astruct_57 *paVar6;
  astruct_86 *iVar5;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u16;
  let mut puVar12: *mut u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut piStack48: *mut i16;
  astruct_19 *local_24;
  let mut uStack32: u16;
  let mut uStack30: u32;
  char *pcStack26;
  let mut uStack18: u32;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut uStack6: u32;

  uVar10 = ((u32)param_7 >> 0x10);
  uVar8 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_86 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar5->field1_0x2 = 0x1008;
  iVar5->field2_0x4 = param_5;
  iVar5->field3_0x8 = 0x0;
  iVar5->field4_0xc = param_3;
  iVar5->field5_0xe = param_2;
  iVar5->field6_0x10 = 0x0;
  iVar5->field7_0x12 = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field12_0x1c)));
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field14_0x22)));
  puVar11 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field17_0x28)));
  paVar6 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)puVar11 >> 0x10));
  iVar5->field20_0x2e = param_4;
  iVar5->field21_0x30 = 0xffff;
  iVar5->field27_0x3a = 0x0;
  iVar5->field28_0x3e = 0x1;
  iVar5->field29_0x40 = 0x1;
  iVar5->field30_0x42 = param_6;
  param_1->field0_0x0 = 0x8e9a;
  iVar5->field1_0x2 = 0x1008;
  if (_PTR_LOOP_1050_0382 == NULL) {
    _PTR_LOOP_1050_0382 =
         mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2e),in_stack_0000fe70,in_stack_0000ff94
                         ,in_stack_0000ff9a,in_stack_0000ff9e);
    paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  }
  uVar10 = ((u32)paVar6 >> 0x10);
  uStack6 = pass1_1008_4772(iVar5->field2_0x4);
  iVar5->field7_0x12 = 0x2f - ((int)uStack6 + 0x8);
  uVar9 = ((u32)_PTR_LOOP_1050_0382 >> 0x10);
  iVar7 = (int)_PTR_LOOP_1050_0382;
  iStack8 = (iVar7 + 0xa);
  iStack10 = (iVar7 + 0xc);
  iStack12 = (iVar7 + 0xe);
  iStack14 = (iVar7 + 0x10);
  iVar7 = iVar5->field4_0xc;
  lVar1 = (long)(iVar7 + iVar5->field5_0xe) * (long)iStack14;
  paVar6 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)lVar1 >> 0x10));
  pass1_1008_3e76((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field12_0x1c)),0x0,
                  (int)lVar1 + iVar5->field7_0x12 + iStack10,
                  (iVar7 - iVar5->field5_0xe) * iStack12 + iVar5->field6_0x10 + iStack8);
  iVar5->field8_0x14 = &iVar5->field12_0x1c + 0x20;
  iVar5->field9_0x16 = ((int)uStack6 + 0x8) + ((int)&iVar5->field12_0x1c + 0x2) + -0x25;
  iVar5->field10_0x18 = iVar5->field8_0x14 + 0x32;
  uVar2 = iVar5->field9_0x16 + 0x19;
  iVar5->field11_0x1a = uVar2;
  mem_op_1000_179c(0x6,paVar6);
  uVar5 = paVar6;
  pcStack26 = (char *)CONCAT22(uVar5,uVar2);
  uStack18 = uVar5 | uVar2;
  if (uStack18 == 0x0) {
    iVar5->field3_0x8 = 0x0;
  }
  else {
    puVar12 = pass1_1008_ada2((u16 *)CONCAT22(uVar5,uVar2),iVar5->field20_0x2e);
    uStack18 = ((u32)puVar12 >> 0x10);
    &iVar5->field3_0x8 = (int)puVar12;
    ((int)&iVar5->field3_0x8 + 0x2) = uStack18;
  }
  BVar3 = pass1_1008_aed8(iVar5->field3_0x8);
  if (BVar3 == 0x0) {
    pcStack26 = (char *)iVar5->field3_0x8;
    uStack18 = pcStack26;
    fn_ptr_1000_17ce(pcStack26);
    iVar5->field3_0x8 = 0x0;
  }
  else {
    piVar4 = (i16 *)iVar5->field3_0x8;
    pass1_1018_20ee((u32)_PTR_LOOP_1050_0382,piVar4);
    uStack18._0_2_ = SUB42(piVar4,0x0);
    pass1_1008_add2((u16 *)iVar5->field3_0x8);
    uStack30 = pass1_1008_4772((astruct_76 *)CONCAT22(uStack18,uStack18));
    pass1_1018_214e(_PTR_LOOP_1050_0382,((u32)_PTR_LOOP_1050_0382 >> 0x10),
                    (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field17_0x28)),iVar5->field20_0x2e);
    local_24 = iVar5->field12_0x1c;
    uStack32 = iVar5->field13_0x20;
    pass1_1008_3f32((i16 *)CONCAT22(0x1050,&local_24),
                    (i16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field17_0x28)));
    piStack48 = (i16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x32));
    pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_24),
                    (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field24_0x34)),
                    (char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x32)));
    uVar10 = (uStack30 >> 0x10);
    iVar5->field25_0x36 = ((int)uStack30 + 0x4) + *piStack48;
    uVar2 = ((int)uStack30 + 0x8) + iVar5->field24_0x34;
    iVar5->field26_0x38 = uVar2;
    pass1_1008_612e(uVar2,0x2,0x5);
    iVar5->field28_0x3e = uVar2;
  }
  return;
}
pub fn pass1_1008_8aa2(param_1: *mut astruct_462)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut uVar5: u32;
  astruct_462 *iVar6;
  let mut uVar6: u16;
  char *pcStack16;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar6 = (astruct_462 *)param_1;
  param_1 = 0x8e9a;
  iVar6->field2_0x2 = 0x1008;
  uVar5 = (u32)&iVar6->field3_0x4;
  if (((int)uVar5 + 0x1c) != 0x0) {
    puVar1 = iVar6->field3_0x4;
    uVar2 = iVar6->field4_0x6;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  uVar2 = iVar6->field55_0x3a;
  uVar3 = iVar6->field56_0x3c;
  pcStack16 = (char *)CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar3,uVar2));
    fn_ptr_1000_17ce(pcStack16);
  }
  param_1 = 0x389a;
  iVar6->field2_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_8b20(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut piVar2: *mut i16;
  let mut in_EDX: u32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u8 local_a [0x2];
  u8 local_8 [0x2];
  astruct_76 *paStack6;

  uVar6 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(i32 *)(iVar5 + 0x8) != 0x0) {
    iVar1 = (iVar5 + 0x40);
    piVar2 = (i16 *)(iVar5 + 0x40);
    *piVar2 = *piVar2 + 0x1;
    uVar4 = (long)iVar1 % (long)(iVar5 + 0x3e) & 0xffff;
    uVar3 = in_EDX & 0xffff0000 | uVar4;
    if ((int)uVar4 == 0x0) {
      (iVar5 + 0x40) = 0x1;
      piVar2 = *(i16 **)(iVar5 + 0x8);
      pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar2);
      paStack6 = (astruct_76 *)((u32)piVar2 & 0xffff | uVar3 << 0x10);
      uVar4 = uVar3 & 0xffff0000 | (u32)uVar6;
      pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)(iVar5 + 0x28U)),(u16 *)CONCAT22(0x1050,local_a),
                      (char *)CONCAT22(0x1050,local_8));
      pass1_1008_8d8a((astruct_76 *)(param_1 & 0xffff | (u32)uVar6 << 0x10),paStack6,(u32)(iVar5 + 0x4),uVar4);
      pass1_1008_4480(*(astruct_76 **)(iVar5 + 0x4),(u16 *)(param_1 & 0xffff0000 | (u32)(iVar5 + 0x28U)),paStack6);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_8bc6(mut param_1: u16 ,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut in_register_0000000a: u16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  u8 local_a [0x2];
  u8 local_8 [0x2];
  astruct_76 *paStack6;

  uVar4 = (param_2 >> 0x10);
  iVar3 = (int)param_2;
  if (*(i32 *)(iVar3 + 0x8) == 0x0) {
    return;
  }
  piVar1 = *(i16 **)(iVar3 + 0x8);
  pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar1);
  paStack6 = (astruct_76 *)((u32)piVar1 & 0xffff | (u32)param_1 << 0x10);
  uVar2 = CONCAT22(in_register_0000000a,uVar4);
  pass1_1008_3e94((u16 *)(param_2 & 0xffff0000 | (u32)(iVar3 + 0x28U)),(u16 *)CONCAT22(0x1050,local_a),
                  (char *)CONCAT22(0x1050,local_8));
  pass1_1008_8d8a((astruct_76 *)(param_2 & 0xffff | (u32)uVar4 << 0x10),paStack6,(u32)(iVar3 + 0x4),uVar2);
  pass1_1008_4480(*(astruct_76 **)(iVar3 + 0x4),(u16 *)(param_2 & 0xffff0000 | (u32)(iVar3 + 0x28U)),paStack6);
  return;
}
pub fn pass1_1008_8c4e(mut param_1: u32,mut param_2: u32,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  astruct_110 *paStack14;
  astruct_57 *paVar4;

  uVar5 = ((u32)param_3 >> 0x10);
  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar8 = pass1_1008_4772(*(astruct_76 **)(iVar6 + 0x4));
  uVar2 = (uVar8 >> 0x10);
  paVar4 = (astruct_57 *)CONCAT22(uVar5,uVar2);
  uVar3 = 0x0;
  if (((iVar6 + 0xc) == 0x0) || ((iVar6 + 0xe) == 0x0)) {
    mem_op_1000_179c(0x14,paVar4);
    paStack14 = (astruct_110 *)CONCAT22(paVar4,uVar3);
    uVar3 = paVar4 | uVar3;
    if (uVar3 == 0x0) {
      uVar2 = 0x0;
      uVar3 = 0x0;
    }
    else {
      puVar1 = (u16 *)(param_1 & 0xffff0000 | (u32)(iVar6 + 0x1c));
      pass1_1008_50c2(paStack14,(u32)((int)uVar8 + 0x8),(u32)((int)uVar8 + 0x4),puVar1,(astruct_76 *)param_2);
      uVar2 = SUB42(puVar1,0x0);
    }
    pass1_1008_5134(CONCAT22(uVar3,uVar2));
  }
  pass1_1008_4480((astruct_76 *)param_2,(u16 *)(param_1 & 0xffff0000 | (u32)(iVar6 + 0x1c)),
                  *(astruct_76 **)(iVar6 + 0x4));
  return;
}
pub fn pass1_1008_8ce4(mut param_1: u32,param_2: *mut u16,mut param_3: u32,mut param_4: u32)

{
  u8 *puVar1;
  let mut uVar2: u16;
  astruct_57 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;
  u8 local_10 [0x6];
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar6 = ((u32)param_4 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  uStack6 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x4));
  uStack10 = 0x0;
  puVar7 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_10),0x0,(iVar4 + 0x12),(iVar4 + 0x10));
  paVar3 = (astruct_57 *)CONCAT22(uVar6,(int)((u32)puVar7 >> 0x10));
  puVar1 = local_10;
  pass1_1008_3f32((i16 *)param_2,(i16 *)CONCAT22(0x1050,puVar1));
  mem_op_1000_179c(0x14,paVar3);
  uVar2 = paVar3 | puVar1;
  if (uVar2 == 0x0) {
    puVar1 = NULL;
    uVar2 = 0x0;
  }
  else {
    uVar6 = (uStack6 >> 0x10);
    pass1_1008_50c2((astruct_110 *)CONCAT22(paVar3,puVar1),(u32)((int)uStack6 + 0x8),
                    (u32)((int)uStack6 + 0x4),param_2,(astruct_76 *)param_3);
  }
  uStack10 = CONCAT22(uVar2,puVar1);
  pass1_1008_5134(CONCAT22(uVar2,puVar1));
  pass1_1008_4480((astruct_76 *)param_3,param_2,*(astruct_76 **)(iVar4 + 0x4));
  return;
}
pub fn pass1_1008_8d8a(param_1: *mut astruct_76,param_2: *mut astruct_76,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  char cVar2;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar8: u16;
  astruct_76 *pstruct76_var7;
  astruct_76 *uVar7;
  let mut uVar9: u32;
  astruct_110 *paStack10;
  astruct_57 *paVar6;

  uVar8 = ((u32)param_4 >> 0x10);
  uVar7 = (astruct_76 *)((u32)param_1 >> 0x10);
  pstruct76_var7 = (astruct_76 *)param_1;
  uVar1 = pstruct76_var7[0x1].field3_0x6;
  if ((int)uVar1 < 0x28) {
    if (((int)uVar1 < 0x25) && (uVar1 != 0x23)) {
      if (0x23 < uVar1) {
        return;
      }
      cVar2 = (char)uVar1;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else if ((int)uVar1 < 0x46) {
    if ((int)uVar1 < 0x43) {
      if ((int)uVar1 < 0x33) {
        return;
      }
      if ((uVar1 != 0x34 && 0x0 < (int)(uVar1 - 0x33)) && (uVar1 != 0x37)) {
        return;
      }
    }
  }
  else if (uVar1 != 0x49) {
    if ((int)(uVar1 - 0x49) < 0x2a) {
      return;
    }
    if (0x5 < (int)(uVar1 - 0x73)) {
      return;
    }
  }
  if (*(i32 *)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2) == 0x0) {
    uVar9 = pass1_1008_4772(param_2);
    uVar4 = (uVar9 >> 0x10);
    paVar6 = (astruct_57 *)CONCAT22(uVar8,uVar4);
    uVar1 = uVar9;
    uVar5 = uVar1;
    mem_op_1000_179c(0x14,paVar6);
    paStack10 = (astruct_110 *)CONCAT22(paVar6,uVar5);
    uVar5 = paVar6 | uVar5;
    if (uVar5 == 0x0) {
      (u32)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2) = 0x0;
    }
    else {
      puVar3 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(pstruct76_var7 + 0x1));
      pass1_1008_50c2(paStack10,(u32)(uVar1 + 0x8),(u32)(uVar1 + 0x4),puVar3,(astruct_76 *)param_3);
      ((int)&pstruct76_var7[0x1].field8_0x10 + 0x2) = (int)puVar3;
      pstruct76_var7[0x1].field9_0x14 = uVar5;
    }
    pass1_1008_5134((u32)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2));
    return;
  }
  pass1_1008_5236(*(astruct_109 **)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2));
  return;
}



StructD * pass1_1008_8e74(StructD *param_1,u8 param_2)

{
  pass1_1008_8aa2((astruct_462 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_op_1008_8e9e(param_1: *mut astruct_78,mut param_2: u32,mut param_3: u32)

{
  astruct_78 *iVar1;
  astruct_78 *uVar1;

  uVar1 = (astruct_78 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_78 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x6 = NULL;
  iVar1->field4_0xa = 0x0;
  iVar1->field5_0xe = param_3;
  iVar1->field6_0x12 = 0x0;
  iVar1->field7_0x16 = param_2;
  iVar1->field8_0x1a = 0x1;
  param_1->field0_0x0 = 0x9170;
  iVar1->field1_0x2 = 0x1008;
  if (iVar1->field5_0xe < 0x7) {
    iVar1->field5_0xe = 0x6;
  }
  pass1_1008_909c(param_1);
  (u32)iVar1->field3_0x6 = 0x0;
  return;
}
pub fn pass1_1008_8f24(param_1: *mut astruct_463)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut uVar5: u32;
  astruct_463 *iVar6;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack6: u32;

  uVar9 = ((u32)param_1 >> 0x10);
  iVar6 = (astruct_463 *)param_1;
  param_1 = 0x9170;
  iVar6->field2_0x2 = 0x1008;
  if (iVar6->field19_0x1a != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      puVar1 = &iVar6->field6_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = (int)uStack6 * 0x4;
      uVar5 = iVar6->field5_0x6;
      uVar10 = ((u32)uVar5 >> 0x10);
      iVar7 = (int)uVar5;
      puVar2 = (u32 *)(iVar7 + iVar8);
      uVar3 = (iVar7 + iVar8 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 += 0x1;
    }
  }
  fn_ptr_1000_17ce((char *)iVar6->field5_0x6);
  param_1 = 0x389a;
  iVar6->field2_0x2 = 0x1008;
  return;
}
pub fn pass1_1008_8faa(param_1: *mut astruct_78,mut param_2: u32)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  pass1_1008_9004((astruct_78 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10),param_2,
                  (param_2 >> 0x10),(u32)((int)param_1 + 0xa));
  return;
}
pub fn empty_1008_8fc4(void)

{
  return;
}
pub fn pass1_1008_9004(param_1: *mut astruct_78,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  u32 *puVar3;
  let mut puVar4: *mut u16;
  astruct_78 *pstruct78_var4;
  astruct_108 *pstruct108_5_1;
  astruct_78 *uVar4;
  let mut uVar3: u16;
  let mut bVar5: bool;
  let mut puVar2: *mut u16;
  u32 *puVar1;
  astruct_108 *pstruct108_5;

  uVar4 = (astruct_78 *)((u32)param_1 >> 0x10);
  pstruct78_var4 = (astruct_78 *)param_1;
  puVar1 = &pstruct78_var4->field4_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct78_var4->field3_0x6 == NULL)) {
    puVar2 = (u16 *)((int)&pstruct78_var4->field6_0x12 + 0x2);
    if ((*puVar2 < param_4 || *puVar2 == param_4) &&
       ((*puVar2 < param_4 ||
        (puVar3 = &pstruct78_var4->field6_0x12, puVar3 < param_4 || puVar3 == param_4))))
    {
      pass1_1008_909c((astruct_78 *)((u32)param_1 & 0xffff | ZEXT24(uVar4) << 0x10));
    }
    puVar3 = &pstruct78_var4->field6_0x12;
    if ((*puVar3 < param_4 || *puVar3 == param_4) || (pstruct78_var4->field3_0x6 == NULL)) {
      return;
    }
    puVar4 = (u16 *)((int)&pstruct78_var4->field4_0xa + 0x2);
    bVar5 = *puVar4 < param_4;
    if ((bVar5 || *puVar4 == param_4) &&
       ((bVar5 || (puVar3 = &pstruct78_var4->field4_0xa,
                  puVar3 < param_4 || puVar3 == param_4)))) {
      &pstruct78_var4->field4_0xa = (int)(param_4 + 0x1);
      ((int)&pstruct78_var4->field4_0xa + 0x2) = (int)(param_4 + 0x1 >> 0x10);
    }
  }
  pstruct108_5 = pstruct78_var4->field3_0x6;
  uVar3 = ((u32)pstruct108_5 >> 0x10);
  pstruct108_5_1 = (astruct_108 *)pstruct108_5;
  (pstruct108_5_1 + param_4 * 0x4) = param_2;
  (pstruct108_5_1 + param_4 * 0x4 + 0x2) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_909c(param_1: *mut astruct_78)

{
  u32 *puVar1;
  let mut uVar2: u16;
  astruct_108 *paVar3;
  let mut uVar6: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  StructD *pSVar7;
  astruct_78 *iVar5;
  astruct_78 *uVar4;
  i32 lVar8;
  astruct_108 *paStack10;
  let mut uStack6: u32;

  uVar4 = (astruct_78 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_78 *)param_1;
  if (iVar5->field6_0x12 == 0x0) {
    uVar6 = &iVar5->field5_0xe;
    pSVar7 = (StructD *)(in_EDX & 0xffff0000 | (u32)((int)&iVar5->field5_0xe + 0x2));
  }
  else {
    uVar2 = &iVar5->field6_0x12;
    puVar1 = &iVar5->field7_0x16;
    uVar6 = uVar2 + puVar1;
    pSVar7 = (StructD *)
             (in_EDX & 0xffff0000 |
             (u32)(((int)&iVar5->field6_0x12 + 0x2) + ((int)&iVar5->field7_0x16 + 0x2) +
                    CARRY2(uVar2,puVar1)));
  }
  uStack6 = CONCAT22((int)pSVar7,uVar6);
  if (iVar5->field3_0x6 == NULL) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar7;
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708(uVar6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  }
  else {
    paVar3 = iVar5->field3_0x6;
    lVar8 = pass1_1000_0ed4(0x1,uVar6 * 0x4,
                            ((int)pSVar7 * 0x2 + CARRY2(uVar6,uVar6)) * 0x2 +
                            CARRY2(uVar6 * 0x2,uVar6 * 0x2),(astruct_172 *)paVar3,
                            (astruct_172 *)((u32)paVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (u8 *)((u32)lVar8 >> 0x10);
    uVar5 = lVar8;
  }
  paStack10 = (astruct_108 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar5);
  if ((PTR_LOOP_1050_5f2e | uVar5) != 0x0) {
    iVar5->field6_0x12 = uStack6;
    iVar5->field3_0x6 = paStack10;
  }
  return;
}



StructD * pass1_1008_914a(param_1: *mut astruct_463,u8 param_2)

{
  pass1_1008_8f24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return (StructD *)param_1;
}
pub fn struct_op_1008_9174(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u32)

{
  astruct_88 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_88 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field2_0x2 = 0x1008;
  iVar1->field3_0x4 = param_3;
  iVar1->field4_0x8 = param_2;
  iVar1->field5_0xc = param_2;
  iVar1->field6_0x10 = 0x0;
  param_1->field0_0x0 = 0x9412;
  iVar1->field2_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 * pass1_1008_91ba(param_1: *mut astruct_3)

{
  let mut UVar1: u16;
  astruct_3 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_3 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar2->field1_0x2 = 0x1008;
  iVar2->field2_0x4 = 0x0;
  set_struct_1008_574a((astruct_57 *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar2 + 0x1)));
  param_1->field0_0x0 = 0x9416;
  iVar2->field1_0x2 = 0x1008;
  _PTR_LOOP_1050_0388 = param_1;
  UVar1 = SetTimer16(NULL,0x1,0x1,HWND16_1050_0396);
  if (UVar1 == 0x0) {
    fn_ptr_op_1000_24cd(0x1);
  }
  PTR_LOOP_1050_038a = (u8 *)((u32)_PTR_LOOP_1050_0388 >> 0x10);
  return &param_1->field0_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn kill_timer_1008_921c(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x9416;
  (iVar1 + 0x2) = 0x1008;
  KillTimer16(0x1,HWND16_1050_0396);
  _PTR_LOOP_1050_0388 = 0x0;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0x6)));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1008_9262(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: i16,mut param_4: u16 ,mut param_5: u32,mut param_6: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;

  mem_op_1000_179c(0x12,param_2);
  uVar2 = param_2 | param_1;
  if (uVar2 == 0x0) {
    param_1 = NULL;
    uVar2 = 0x0;
  }
  else {
    struct_op_1008_9174((astruct_57 *)CONCAT22(param_2,param_1),param_5,param_6);
  }
  if ((uVar2 | param_1) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(param_3 + 0x6) + 0x4);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_92b2(mut param_1: u32,i32 param_2,i32 param_3)

{
  code **ppcVar1;
  u8 *puVar2;
  let mut extraout_DX: u16;
  u8 local_c [0x4];
  let mut uStack8: u32;
  let mut uStack4: u16;

  uStack4 = 0x0;
  pass1_1008_57a4((u32 *)CONCAT22(0x1050,local_c),param_1 & 0xffff0000 | (u32)((int)param_1 + 0x6));
  while( true ) {
    puVar2 = local_c;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
    if ((extraout_DX | puVar2) == 0x0) break;
    if ((*(i32 *)(puVar2 + 0x4) == param_3) && (*(i32 *)(puVar2 + 0x8) == param_2)) {
      uStack4 = 0x1;
      ppcVar1 = (code **)((int)(u32)((int)param_1 + 0x6) + 0xc);
      (**ppcVar1)();
      uStack8 = 0x0;
    }
  }
  return;
}
pub fn pass1_1008_932a(mut param_1: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  u8 *puVar3;
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u8 local_a [0x8];

  uVar6 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  if ((iVar5 + 0x4) == 0x0) {
    (iVar5 + 0x4) = 0x1;
    pass1_1008_57a4((u32 *)CONCAT22(0x1050,local_a),param_1 & 0xffff0000 | (u32)(iVar5 + 0x6));
    while( true ) {
      puVar3 = local_a;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
      if ((extraout_DX | puVar3) == 0x0) break;
      uVar1 = (puVar3 + 0xc);
      iVar4 = (puVar3 + 0xe) - (uVar1 < 0x37);
      (puVar3 + 0xc) = uVar1 - 0x37;
      (puVar3 + 0xe) = iVar4;
      if ((iVar4 < 0x1) && (((iVar4 < 0x0 || ((puVar3 + 0xc) == 0x0)) && ((puVar3 + 0x10) == 0x0)))) {
        ppcVar2 = (code **)((int)*(u32*)(puVar3 + 0x4) + 0x4);
        (**ppcVar2)();
        (u32)(puVar3 + 0xc) = (u32)(puVar3 + 0x8);
      }
    }
    (iVar5 + 0x4) = 0x0;
  }
  return;
}



u16 * pass1_1008_93c0(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_93ec(StructD *param_1,u8 param_2)

{
  kill_timer_1008_921c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_941a(param_1: *mut u16,mut param_2: u16 ,mut param_3: u16 )

{
  *param_1 = param_2;
  ((int)param_1 + 0x2) = param_3;
  return param_1;
}



u16 * pass1_1008_9436(param_1: *mut u16)

{
  *param_1 = 0x0;
  ((int)param_1 + 0x2) = 0x0;
  return param_1;
}
pub fn pass1_1008_944e(param_1: *mut u16,mut param_2: u16 ,mut param_3: u16 )

{
  ((int)param_1 + 0x2) = param_3;
  *param_1 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_9466(param_1: *mut u16)

{
  *param_1 = 0x52a;
  ((int)param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(_PTR_LOOP_1050_0392);
  _PTR_LOOP_1050_0392 = NULL;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

WPARAM16 win_msg_op_1008_9498(void)

{
  let mut BVar1:bool;
  INT16 IVar2;
  MSG16 local_msg_1;//
//
LAB_1008_949c:
  BVar1 = GetMessage16(0x0,0x0,0x0,&local_msg_1);
  if (BVar1 == 0x0) {
    return local_msg_1.wparam;
  }
  if (((int)_u16_1050_5bc8 + 0x8) != 0x0) goto code_r0x100894cd;
  goto LAB_1008_94dc;
code_r0x100894cd:
  BVar1 = IsDialogMessage16(&local_msg_1,(HWND16)&DAT_1050_1050);
  if (BVar1 == 0x0) {//
LAB_1008_94dc:
    if (PTR_LOOP_1050_0398 != NULL) {
      IVar2 = TranslateAccelerator16(&local_msg_1,(HACCEL16)&DAT_1050_1050,(HWND16)PTR_LOOP_1050_0398);
      if (IVar2 != 0x0) goto LAB_1008_949c;
    }
    TranslateMessage16(&local_msg_1);
    DispatchMessage16(&local_msg_1);
  }
  goto LAB_1008_949c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_msg_op_1008_9510(i16 *param_1)

{
  let mut has_message:bool;
  INT16 IVar1;
  MSG16 local_14;//
//
LAB_1008_9578:
  if (*param_1 != 0x0) {
    has_message = GetMessage16(0x0,0x0,0x0,&local_14);
    if (has_message != 0x0) {
      if (((int)_u16_1050_5bc8 + 0x8) != 0x0) goto code_r0x10089538;
      goto LAB_1008_9547;
    }
  }
  return;
code_r0x10089538:
  has_message = IsDialogMessage16(&local_14,(HWND16)&DAT_1050_1050);
  if (has_message == 0x0) {//
LAB_1008_9547:
    if (PTR_LOOP_1050_0398 != NULL) {
      IVar1 = TranslateAccelerator16(&local_14,(HACCEL16)&DAT_1050_1050,(HWND16)PTR_LOOP_1050_0398);
      if (IVar1 != 0x0) goto LAB_1008_9578;
    }
    TranslateMessage16(&local_14);
    DispatchMessage16(&local_14);
  }
  goto LAB_1008_9578;
}
pub fn set_struct_op_1008_9584(param_1: *mut astruct_20,mut param_2: u32)

{
  astruct_20 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->base_0x2 = 0x1008;
  iVar1->field2_0x4 = param_2;
  param_1->offset_0x0 = 0x9d2e;
  iVar1->base_0x2 = 0x1008;
  iVar1->field3_0x8 = 0x0;
  iVar1->field139_0xac = 0x2000000;
  iVar1->field140_0xb0 = 0x0;
  iVar1->field141_0xb4 = 0x8000;
  iVar1->field142_0xb6 = 0x8000;
  iVar1->field143_0xb8 = 0x8000;
  iVar1->field144_0xba = 0x8000;
  iVar1->field145_0xbc = 0x0;
  iVar1->field146_0xbe = 0x0;
  iVar1->field147_0xc2 = 0x0;
  iVar1->hcursor_field_0xc4 = 0x0;
  iVar1->hgdiobj_field_0xc6 = 0x0;
  iVar1->field150_0xc8 = 0x2008;
  iVar1->field151_0xca = 0x0;
  param_1->offset_0x0 = 0x380a;
  iVar1->base_0x2 = 0x1008;
  iVar1->field60_0x5b = '\0';
  *(u8 *)&iVar1->field4_0xa = 0x0;
  return;
}
pub fn pass1_1008_9628(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x8) == 0x0) {
    ((int)param_1 + 0x8) = param_2;
  }
  return;
}
pub fn send_msg_1008_9640(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x8) != 0x0) {
    SendMessage16(0x0,param_2,0x86,*(HWND16 *)((int)param_1 + 0x8));
  }
  return;
}
pub fn set_win_text_1008_9664(mut param_1: u32,mut param_2: u16 ,char *param_3)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0xaU)),param_3);
  SetWindowText16(param_1 & 0xffff0000 | (u32)((int)param_1 + 0xaU),*(HWND16 *)((int)param_1 + 0x8));
  return;
}
pub fn destroy_win_1008_9698(param_1: *mut astruct_871,mut param_2: u16 )

{
  DestroyWindow16(param_1->hwnd_0x8);
  return;
}



BOOL16 show_win_1008_96ae(mut param_1: u32,INT16 param_2)

{
  let mut BVar1:bool;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (((int)param_1 + 0x8) != 0x0) {
    BVar1 = ShowWindow16(param_2,*(HWND16 *)((int)param_1 + 0x8));
    return BVar1;
  }
  return 0x0;
}
pub fn win_ui_reg_class_1008_96d2(StructA *param_1)

{
  let mut BVar1:bool;
  ATOM AVar2;
  WNDCLASS16 wndclass;

  wndclass.lpsz_class_name._0_2_ = (int)param_1 + 0x5b;
  BVar1 = GetClassInfo16(&wndclass,(char *)CONCAT22((int)wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0x0) {
    wndclass.style = ((int)param_1 + 0xc8);
    wndclass.lpfn_wnd_proc._0_2_ = 0x5632;
    wndclass.lpfn_wnd_proc = 0x1008;
    wndclass._6_4_ = 0x40000;
    wndclass.h_instance = HINSTANCE16_1050_038c;
    wndclass.h_icon = *(HICON16 *)((int)param_1 + 0xc2);
    wndclass.h_cursor = *(HCURSOR16 *)((int)param_1 + 0xc4);
    wndclass.hbr_background = *(HBRUSH16 *)((int)param_1 + 0xc6);
    wndclass.lpsz_menu_name = 0x0;
    wndclass.lpsz_class_name = param_1;
    AVar2 = RegisterClass16(&wndclass);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0);
    }
  }
  return;
}
pub fn create_window_ex_1008_9760(StructA *in_struct_1)

{
  HWND16 window_handle;
  StructA *struct_1;
  let mut uVar1: u16;

  uVar1 = ((u32)in_struct_1 >> 0x10);
  struct_1 = (StructA *)in_struct_1;
  if (struct_1->field4_0x8 == 0x0) {
    window_handle =
         CreateWIndowEx16((void *)((u32)in_struct_1 & 0xffff | (u32)uVar1 << 0x10),HINSTANCE16_1050_038c,
                          struct_1->field159_0xca,struct_1->field149_0xbc,struct_1->field148_0xba,
                          struct_1->field147_0xb8,struct_1->field146_0xb6,struct_1->field145_0xb4,
                          struct_1->field140_0xac,(char *)0x1050039e,
                          (char *)((u32)in_struct_1 & 0xffff0000 | ZEXT24(&struct_1->field60_0x5b)),
                          *(DWORD *)&struct_1->field_0xb0);
    struct_1->field4_0x8 = window_handle;
  }
  if (struct_1->field4_0x8 == 0x0) {
    fn_ptr_op_1000_24cd(0x0);
  }
  return;
}
pub fn begin_end_paint_1008_97c8(param_1: *mut astruct_837,mut param_2: u16 )

{
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&stack0xffde),param_1->field8_0x8);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&stack0xffde),param_1->field8_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_win_op_1008_97f2(u32 *param_1,i16 *param_2,WPARAM16 param_3,u8 *param_4,mut param_5: u16 ) -> u32

{
  code **ppcVar1;
  let mut BVar2:bool;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  astruct_864 *paVar6;
  let mut UVar7: u16;
  u8 unaff_CS;
  let mut uVar8: u32;
  u8 uVar9;
  u8 uVar10;

  paVar6 = (astruct_864 *)param_1;
  UVar7 = ((u32)param_1 >> 0x10);
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((i16 *)CONCAT22(param_3,param_2));
    }
    else {
      ppcVar1 = (code **)((int)*param_1 + 0x70);
      (**ppcVar1)();
    }
    uVar5 = 0x1;
    goto LAB_1008_9a95;
  }
  uVar10 = (u8)((u32)param_1 >> 0x10);
  uVar9 = SUB41(param_1,0x0);
  if (param_5 < 0x2c) {
    unaff_CS = 0x8;
    switch(param_5) {
    case 0x1:
      break;
    case 0x2:
      ppcVar1 = (code **)((int)*param_1 + 0x3c);
      (**ppcVar1)(0x1008);
      SetWindowLong16(0x0,0x0,paVar6->hwnd_0x8);
      BVar2 = IsWindow16(paVar6[0x12].hwnd_0x8);
      if (BVar2 != 0x0) {
        PostMessage16((LPARAM)param_1,0xc7,0x111,paVar6[0x12].hwnd_0x8);
      }
      break;
    case 0x3:
      ppcVar1 = (code **)((int)*param_1 + 0x54);
      (**ppcVar1)(0x8,uVar9,UVar7,param_3,param_2);
      break;
    default:
      goto switchD_1008_9b30_caseD_4;
    case 0x5:
      ppcVar1 = (code **)((int)*param_1 + 0x58);
      (**ppcVar1)(0x8,uVar9,uVar10,param_3,param_2,param_4);
      break;
    case 0x7:
      ppcVar1 = (code **)((int)*param_1 + 0x50);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0x8:
      ppcVar1 = (code **)((int)*param_1 + 0x74);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0xd:
      ppcVar1 = (code **)((int)*param_1 + 0x84);
      iVar4 = (**ppcVar1)(0x8,uVar9,uVar10,param_2,CONCAT12(param_4._0_1_,param_3));
      goto LAB_1008_9ada;
    case 0xf:
      ppcVar1 = (code **)((int)*param_1 + 0x34);
      (**ppcVar1)(0x1008,param_1);
      break;
    case 0x10:
      ppcVar1 = (code **)((int)*param_1 + 0x38);
      uVar8 = (**ppcVar1)(0x1008,param_1);
      return uVar8;
    case 0x19:
      ppcVar1 = (code **)((int)*param_1 + 0x78);
      uVar3 = (**ppcVar1)(0x8,uVar9,uVar10,param_2,CONCAT12(param_4._0_1_,param_3));
      return CONCAT22(0x1050,uVar3);
    case 0x1c:
      ppcVar1 = (code **)((int)*param_1 + 0x30);
      (**ppcVar1)(0x8,param_1,param_4);
    }
  }
  else if (param_5 == 0x112) {
    if ((PTR_LOOP_1050_039a == NULL) && (ppcVar1 = (code **)((int)*param_1 + 0x48), iVar4 = (**ppcVar1)(), iVar4 != 0x0)
       ) {
      make_def_wnd_proc_1008_9ce6
                (paVar6,UVar7,CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),(WPARAM16)param_4,0x112);
    }
  }
  else if (param_5 < 0x113) {
    if (param_5 == 0x86) {
      ppcVar1 = (code **)((int)*param_1 + 0x80);
      uVar8 = (**ppcVar1)();
      return uVar8;
    }
    if (param_5 < 0x87) {
      if (param_5 == 0x85) {
        ppcVar1 = (code **)((int)*param_1 + 0x7c);
        uVar8 = (**ppcVar1)();
        return uVar8;
      }
      if (param_5 < 0x86) {
        if ((char)param_5 == '7') {
          return (u32)&paVar6[0x13].field_0x4;
        }
        if ((char)param_5 == 'A') {
          ppcVar1 = (code **)((int)*param_1 + 0x2c);
          (**ppcVar1)();
          goto switchD_1008_9b30_caseD_1;
        }
      }
switchD_1008_9b30_caseD_4:
      if ((param_5 < 0x400) || (0x7ffe < param_5)) {
        uVar8 = make_def_wnd_proc_1008_9ce6(paVar6,UVar7,CONCAT22(param_3,param_2),(WPARAM16)param_4,param_5);
        return uVar8;
      }
      ppcVar1 = (code **)((int)*param_1 + 0x28);
      (**ppcVar1)(unaff_CS,uVar9,uVar10,(char)param_2,param_3,CONCAT22(param_5,param_4));
    }
    else if (param_5 == 0x100) {
      if (PTR_LOOP_1050_039a == NULL) {
        ppcVar1 = (code **)((int)*param_1 + 0x6c);
        (**ppcVar1)();
      }
    }
    else if (param_5 == 0x102) {
      if (PTR_LOOP_1050_039a == NULL) {
        ppcVar1 = (code **)((int)*param_1 + 0x68);
        (**ppcVar1)();
      }
    }
    else {
      if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
      if ((param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == NULL)) {
        if (param_2 == NULL) {
          ppcVar1 = (code **)((int)*param_1 + 0x40);
          (**ppcVar1)();
        }
        else {
          ppcVar1 = (code **)((int)*param_1 + 0x44);
          (**ppcVar1)();
        }
      }
    }
  }
  else if (param_5 == 0x204) {
    if (PTR_LOOP_1050_039a == NULL) {
      ppcVar1 = (code **)((int)*param_1 + 0x60);
      (**ppcVar1)();
    }
  }
  else if (param_5 < 0x205) {
    if (param_5 == 0x113) {
      if (_PTR_LOOP_1050_0388 != 0x0) {
        pass1_1008_932a(_PTR_LOOP_1050_0388);
      }
    }
    else if (param_5 == 0x117) {
      if (param_3 == 0x0) {
        ppcVar1 = (code **)((int)*param_1 + 0x4c);
        (**ppcVar1)();
      }
      else {
        ppcVar1 = (code **)((int)*param_1 + 0x20);
        (**ppcVar1)();
      }
    }
    else {
      if (param_5 != 0x201) goto switchD_1008_9b30_caseD_4;
      if (PTR_LOOP_1050_039a == NULL) {
        ppcVar1 = (code **)((int)*param_1 + 0x5c);
        (**ppcVar1)();
      }
    }
  }
  else if (param_5 == 0x210) {
    ppcVar1 = (code **)((int)*param_1 + 0x64);
    (**ppcVar1)();
  }
  else {
    if (param_5 == 0x30f) {//
LAB_1008_9af8:
      ppcVar1 = (code **)((int)*param_1 + 0x8c);
      iVar4 = (**ppcVar1)();//
LAB_1008_9ada:
      return (u32)iVar4;
    }
    if (param_5 == 0x311) {
      ppcVar1 = (code **)((int)*param_1 + 0x88);
      iVar4 = (**ppcVar1)();
      if (iVar4 != 0x0) goto LAB_1008_9af8;
    }
    else {
      if (param_5 != 0x3b9) goto switchD_1008_9b30_caseD_4;
      ppcVar1 = (code **)((int)*param_1 + 0x24);
      (**ppcVar1)();
    }
  }
switchD_1008_9b30_caseD_1:
  uVar5 = 0x0;//
LAB_1008_9a95:
  return (u32)uVar5;
}



LRESULT pass1_1008_9c16(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  LRESULT LVar1;

  LVar1 = make_def_wnd_proc_1008_9ce6
                    ((astruct_864 *)param_1,param_2,CONCAT22((int)param_3,(int)(param_2 >> 0x10)),
                     (WPARAM16)(param_3 >> 0x10),0x85);
  return LVar1;
}



LRESULT pass1_1008_9c30(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  LRESULT LVar1;

  LVar1 = make_def_wnd_proc_1008_9ce6
                    ((astruct_864 *)param_1,param_2,CONCAT22((int)param_3,(int)(param_2 >> 0x10)),
                     (WPARAM16)(param_3 >> 0x10),0x86);
  return LVar1;
}
pub fn pass1_1008_9c4a(void)

{
  return;
}
pub fn pass1_1008_9c4e(void)

{
  return;
}
pub fn pass1_1008_9c52(void)

{
  return;
}
pub fn get_stock_obj_1008_9c56(void)

{
  GetStockObject16(HOLLOW_BRUSH);
  return;
}
pub fn pass1_1008_9c60(mut param_1: u16 ,mut param_2: u16 ,u32 *param_3,mut param_4: i16)

{
  code **ppcVar1;

  if ((param_4 == 0xc7) && (param_3 != NULL)) {
    ppcVar1 = (code **)*param_3;
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_9c86(mut param_1: u32,char *param_2,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = str_op_1000_3da4((char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0xa)));
  if (param_3 < (int)uVar1) {
    uVar1 = param_3 - 0x1;
  }
  str_op_1000_3dbe(param_2,(char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0xa)),uVar1);
  return;
}



BOOL16 pass1_1008_9cc4(mut param_1: u32,mut param_2: i16)

{
  if (((int)param_1 + 0x8) != param_2) {
    return 0x1;
  }
  return 0x0;
}



u16 pass1_1008_9ce0(void)

{
  return 0x0;
}



LRESULT make_def_wnd_proc_1008_9ce6
                  (param_1: *mut astruct_864,mut param_2: u16 ,LPARAM lparam_param_3,WPARAM16 wparam_param_4,u16 msg_param_5)

{
  LRESULT LVar1;

  LVar1 = DefWindowProc16(lparam_param_3,wparam_param_4,msg_param_5,param_1->hwnd_0x8);
  return LVar1;
}



u16 * pass1_1008_9d02(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_9d36(param_1: *mut astruct_19,param_2: *mut astruct_19,mut param_3: u16 )

{
  let mut uVar1: u16;
  astruct_57 *paVar2;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut uVar4: u32;
  astruct_57 *paVar5;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;
  let mut iStack4: i16;

  uVar3 = ((u32)in_EDX >> 0x10);
  struct_op_1018_4cda((astruct_19 *)CONCAT22(param_2,param_1),param_3);
  param_1->field15_0x1c = 0x389a;
  param_1->field16_0x1e = 0x1008;
  param_1->field15_0x1c = 0x3aa8;
  param_1->field16_0x1e = 0x1008;
  param_1->field17_0x20 = 0x0;
  puVar6 = pass1_1008_3e38((astruct_19 *)CONCAT22(param_2,&param_1->field44_0x52));
  uVar4 = CONCAT22(uVar3,(int)((u32)puVar6 >> 0x10));
  CONCAT22(param_2,param_1) = 0x9fb2;
  param_1->segment_0x2 = 0x1008;
  param_1->field15_0x1c = 0x9fca;
  param_1->field16_0x1e = 0x1008;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1000_4906((StructD *)CONCAT22(param_2,&param_1->field18_0x22),NULL,0x30);
  pass1_1018_4dce((u8 *)uVar4,(astruct_19 *)CONCAT22(param_2,param_1),0x1c0);
  iStack4 = 0x0;
  uVar3 = 0x1018;
  do {
    uVar1 = FUN_1010_830a(iStack4 + 0x1c0,uVar4,uVar3,_u16_1050_14cc,iStack4 + 0x1c0);
    (&param_1->field18_0x22)[iStack4 * 0x2] = uVar1;
    (&param_1->field19_0x24)[iStack4 * 0x2] = uVar4;
    iStack4 += 0x1;
    uVar3 = 0x1010;
  } while (iStack4 < 0xc);
  uVar7 = pass1_1008_4772(*(astruct_76 **)&param_1->field18_0x22);
  uVar4 &= 0xffff0000;
  uVar3 = (uVar7 >> 0x10);
  pass1_1008_3e76((u16 *)CONCAT22(param_2,&param_1->field44_0x52),0x0,(0x1e0 - ((int)uVar7 + 0x8)) / 0x2 - 0x32,
                  (0x280 - ((int)uVar7 + 0x4)) / 0x2);
  if (CONCAT22(param_2,param_1) == 0x0) {
    paVar2 = NULL;
    paVar5 = (astruct_57 *)(uVar4 & 0xffff0000);
  }
  else {
    paVar5 = (astruct_57 *)(uVar4 & 0xffff0000 | ZEXT24(param_2));
    paVar2 = (astruct_57 *)&param_1->field15_0x1c;
  }
  pass1_1008_9262(paVar2,paVar5,(int)_PTR_LOOP_1050_0388,((u32)_PTR_LOOP_1050_0388 >> 0x10),0x50,
                  CONCAT22((int)paVar5,paVar2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_9e5a(StructD *structd_param_1)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  u8 *puVar4;
  let mut uVar5: u16;
  StructD *structd_5;
  let mut uVar6: u16;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  uVar6 = ((u32)structd_param_1 >> 0x10);
  structd_5 = (StructD *)structd_param_1;
  structd_param_1->address_offset_field_0x0 = 0x9fb2;
  structd_5->address_offset_field_0x2 = 0x1008;
  &structd_5->field_0x1c = 0x9fca;
  &structd_5->field_0x1e = 0x1008;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (structd_param_1 == NULL) {
      puVar4 = NULL;
      uVar5 = 0x0;
    }
    else {
      puVar4 = &structd_5->field_0x1c;
      uVar5 = uVar6;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x50,CONCAT22(uVar5,puVar4));
  }
  iStack4 = 0x0;
  do {
    puVar1 = (u32 *)(&structd_5->field20_0x22)[iStack4 * 0x2];
    uVar2 = (&structd_5->field_0x24 + iStack4 * 0x4);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0xc);
  if (structd_param_1 == NULL) {
    puVar4 = NULL;
    uVar6 = 0x0;
  }
  else {
    puVar4 = &structd_5->field_0x1c;
  }
  puStack8 = (u16 *)CONCAT22(uVar6,puVar4);
  *puStack8 = 0x389a;
  (puVar4 + 0x2) = 0x1008;
  clenaup_win_ui_1018_4d22(structd_param_1);
  return;
}
pub fn pass1_1008_9f18(mut param_1: i16,mut param_2: u16 ,mut param_3: i16)

{
  if (param_3 == 0x2) {
    pass1_1008_9f64(CONCAT22(param_2,param_1 + -0x1c));
    pass1_1010_1f62((astruct_27 *)CONCAT22(param_2,param_1 + -0x1c),0x2);
  }
  return;
}



astruct_76 * pass1_1008_9f48(param_1: *mut astruct_134)

{
  astruct_134 *iVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_134 *)param_1;
  iVar2 = iVar1->field32_0x20 * 0x4;
  return (astruct_76 *)
         CONCAT22((&iVar1[0x1].field_0x2 + iVar2),(&iVar1[0x1].field_0x0 + iVar2));
}
pub fn pass1_1008_9f64(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  piVar1 = (i16 *)(iVar2 + 0x20);
  *piVar1 = *piVar1 + 0x1;
  if (0xb < (iVar2 + 0x20)) {
    (iVar2 + 0x20) = 0x0;
  }
  return;
}



StructD * pass1_1008_9f80(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0x1c));
  pass1_1008_9e5a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1008_9fb2(mut param_1: u16 ,mut param_2: i16,u8 param_3,u8 param_4,u8 param_5,mut param_6: u16 ,mut param_7: i16,
                    mut param_8: u16 ,mut param_9: u16 )

{
  char *pcVar1;
  u8 *pbVar2;
  u8 bVar3;
  code *pcVar4;
  u8 bVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u8 bVar8;
  let mut uVar9: u16;
  let mut in_register_00000009: u32;
  let mut uVar10: u32;
  astruct_57 *paVar11;
  let mut unaff_SI: i16;
  u8 bVar13;
  let mut bVar14: bool;
  let mut bVar15: bool;
  astruct_19 *paVar16;
  astruct_57 *paVar12;

  uVar10 = CONCAT31(in_register_00000009,param_5);
  (param_2 + 0x1008) = (int)&DAT_1050_1050;
  uVar7 = param_4;
  uVar9 = param_1 + 0xeff0;
  bVar13 = param_1 < 0x1010 || uVar9 < uVar7;
  uVar6 = uVar9 - uVar7;
  pcVar4 = (code *)swi(0x4);
  if (SBORROW2(param_1,0x1010) != SBORROW2(uVar9,uVar7)) {
    (*pcVar4)();
  }
  bVar5 = (u8)((char)(uVar6 + 0xeff0) - bVar13) % 0x1d;
  pcVar1 = (char *)(param_2 + unaff_SI);
  bVar8 = (u8)uVar10;
  *pcVar1 = *pcVar1 + bVar8 + (uVar6 < 0x1010 || uVar6 + 0xeff0 < bVar13);
  pbVar2 = (u8 *)(param_2 + unaff_SI);
  bVar14 = *pbVar2 < bVar8 || (u8)(*pbVar2 - bVar8) < (0xb1 < bVar5);
  *pbVar2 = (*pbVar2 - bVar8) - (0xb1 < bVar5);
  pbVar2 = (u8 *)(param_2 + 0x18);
  bVar15 = *pbVar2 < param_3 || (u8)(*pbVar2 - param_3) < bVar14;
  *pbVar2 = (*pbVar2 - param_3) - bVar14;
  pbVar2 = (u8 *)(param_2 + unaff_SI + 0x89f);
  bVar13 = *pbVar2;
  bVar3 = *pbVar2 + bVar5 + 0x4e;
  *pbVar2 = bVar3 + bVar15;
  pcVar1 = (char *)(param_2 + unaff_SI);
  *pcVar1 = *pcVar1 + (char)param_2 + (CARRY1(bVar13,bVar5 + 0x4e) || CARRY1(bVar3,bVar15));
  pbVar2 = (u8 *)(param_2 + unaff_SI);
  *pbVar2 = *pbVar2 | bVar8;
  paVar16 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_8,param_7),param_9);
  paVar11 = (astruct_57 *)(uVar10 & 0xffff0000 | (u32)paVar16 >> 0x10);
  uVar7 = 0x0;
  (u32)(param_7 + 0xa) = 0x0;
  (u32)(param_7 + 0x410) = 0x0;
  (param_7 + 0x414) = 0x0;
  (param_7 + 0x416) = 0x0;
  (param_7 + 0x418) = 0x0;
  (param_7 + 0x41a) = 0x0;
  (param_7 + 0x41c) = 0x0;
  (param_7 + 0x41e) = 0x0;
  CONCAT22(param_8,param_7) = 0xad92;
  (param_7 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,paVar11);
  uVar9 = paVar11 | uVar7;
  paVar12 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)uVar9);
  if (uVar9 == 0x0) {
    (u32)(param_7 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar11,uVar7));
    (param_7 + 0xa) = uVar7;
    (param_7 + 0xc) = (int)paVar12;
  }
  mem_op_1000_179c(0xc,paVar12);
  uVar9 = paVar12 | uVar7;
  if (uVar9 == 0x0) {
    uVar7 = 0x0;
    uVar9 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar12,uVar7));
  }
  (param_7 + 0x410) = uVar7;
  (param_7 + 0x412) = uVar9;
  return;
}
pub fn struct_1008_9fd2(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  astruct_57 *paVar3;
  astruct_19 *paVar6;
  astruct_57 *paVar4;

  uVar5 = ((u32)in_EDX >> 0x10);
  paVar6 = struct_op_1010_1d48(param_1,param_2);
  paVar3 = (astruct_57 *)CONCAT22(uVar5,(int)((u32)paVar6 >> 0x10));
  uVar1 = 0x0;
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0x410) = 0x0;
  ((int)param_1 + 0x414) = 0x0;
  ((int)param_1 + 0x416) = 0x0;
  ((int)param_1 + 0x418) = 0x0;
  ((int)param_1 + 0x41a) = 0x0;
  ((int)param_1 + 0x41c) = 0x0;
  ((int)param_1 + 0x41e) = 0x0;
  param_1->offset_0x0 = 0xad92;
  ((int)param_1 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  paVar4 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    (u32)((int)param_1 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    ((int)param_1 + 0xa) = uVar1;
    ((int)param_1 + 0xc) = (int)paVar4;
  }
  mem_op_1000_179c(0xc,paVar4);
  uVar2 = paVar4 | uVar1;
  if (uVar2 == 0x0) {
    uVar1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar1));
  }
  ((int)param_1 + 0x410) = uVar1;
  ((int)param_1 + 0x412) = uVar2;
  return;
}
pub fn pass1_1008_a086(param_1: *mut astruct_455)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  code **ppcVar4;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xad92;
  iVar4->field1_0x2 = 0x1008;
  puVar1 = (u32 *)iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = (u32 *)(iVar4 + 0x82)->field0_0x0;
  uVar3 = iVar4[0x82].field1_0x2;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn post_win_msg_1008_a0e4(param_1: *mut astruct_67,i32 param_2,mut param_3: i16,mut param_4: u16 ,mut param_5: u32,mut param_6: i16)

{
  u32 *puVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut bVar4: bool;
  char *puVar4;
  astruct_66 *uVar5;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  astruct_67 *iVar7;
  astruct_67 *uVar6;
  astruct_99 *paStack14;
  char local_a [0x8];

  uVar6 = (astruct_67 *)((u32)param_1 >> 0x10);
  iVar7 = (astruct_67 *)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)iVar7->field10_0xa);
  bVar4 = false;
  do {
    puVar4 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar4));
    if ((extraout_DX | puVar4) == 0x0) goto LAB_1008_a146;
  } while ((puVar4 + 0x4) != param_6);
  (puVar4 + 0xc) = (puVar4 + 0xc) + param_3;
  *(i32 *)(puVar4 + 0xe) = *(i32 *)(puVar4 + 0xe) + param_2;
  bVar4 = true;//
LAB_1008_a146:
  if (!bVar4) {
    paStack14 = pass1_1000_07fc(_PTR_LOOP_1050_03a0);
    uVar7 = ((u32)paStack14 >> 0x10);
    uVar3 = paStack14;
    if ((uVar7 | uVar3) == 0x0) {
      paStack14 = NULL;
    }
    else {
      paStack14->field0_0x0 = 0x389a;
      (uVar3 + 0x2) = 0x1008;
      (uVar3 + 0x4) = param_6;
      (u32)(uVar3 + 0x6) = param_5;
      (uVar3 + 0xa) = param_4;
      (uVar3 + 0xc) = param_3;
      *(i32 *)(uVar3 + 0xe) = param_2;
      paStack14->field0_0x0 = 0xad8e;
      (uVar3 + 0x2) = 0x1008;
    }
    puVar1 = iVar7->field10_0xa;
    ppcVar2 = (code **)((int)*iVar7->field10_0xa + 0x8);
    (**ppcVar2)(0x1000,(int)puVar1,(int)((u32)puVar1 >> 0x10),(int)paStack14,(int)((u32)paStack14 >> 0x10));
  }
  if (param_6 == 0x14) {
    PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_a1f0(uchar param_1,mut param_2: u32,param_3: *mut u16,param_4: *mut u16,param_5: *mut u16,param_6: *mut u16)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut uVar5: u32;
  astruct_57 *paVar6;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut in_buf_len_5: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  astruct_27 *paVar11;
  char *pcVar12;
  let mut in_stack_0000fd8a: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb4: u16;
  let mut in_stack_0000feb8: u16;
  let mut uVar13: u16;
  u8 uVar14;
  u8 uVar15;
  let mut uVar16: u32;
  char local_106 [0x100];
  u32 *puStack6;

  uVar2 = 0x0;
  *param_6 = 0x0;
  *param_5 = 0x0;
  *param_4 = 0x0;
  *param_3 = 0x0;
  in_buf_len_5 = (param_2 >> 0x10);
  uVar7 = param_2;
  *(u8 *)(uVar7 + 0xe) = 0x0;
  uVar16 = (u32)(uVar7 + 0xa);
  ppcVar1 = (code **)((int)(u32)(u32)(uVar7 + 0xa) + 0x10);
  (**ppcVar1)();
  uVar4 = in_EDX;
  puStack6 = (u32 *)CONCAT22(uVar4,uVar2);
  uVar5 = in_EDX & 0xffff0000 | (u32)(uVar4 | uVar2);
  if ((uVar4 | uVar2) == 0x0) {
    return;
  }
  *param_6 = (uVar2 + 0x4);
  *param_4 = (uVar2 + 0xa);
  uVar3 = pass1_1008_ab80(uVar7,in_buf_len_5,*param_6);
  *param_3 = uVar3;
  uVar9 = ((u32)puStack6 >> 0x10);
  iVar8 = (int)puStack6;
  uVar10 = 0x1008;
  uVar13 = _u16_1050_14cc;
  uVar3 = (_u16_1050_14cc >> 0x10);
  switch((iVar8 + 0x4)) {
  case 0x1:
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd1;
    goto LAB_1008_a2b1;
  case 0x2:
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar8 + 0x6));
    load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_106,(short)&DAT_1050_1050)
    ;
    pcVar12 = pass1_1038_4d28((char *)CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,iVar8)));
    uVar10 = 0x1000;
    sys_1000_3f9c((char *)(param_2 & 0xffff0000 | (u32)(uVar7 + 0xe)),(char *)CONCAT22(0x1050,local_106),pcVar12)
    ;
    break;
  case 0x5:
    goto LAB_1008_a277;
  case 0x6:
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd4;//
LAB_1008_a2b1:
    uVar10 = 0x1010;
    *param_4 = 0x1;
    break;
  case 0x7://
LAB_1008_a277:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    break;
  case 0x9:
    if ((uVar7 + 0x416) == 0x0) {
      (uVar7 + 0x416) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0xb:
    if ((uVar7 + 0x41a) == 0x0) {
      (uVar7 + 0x41a) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0xe:
    if ((uVar7 + 0x41c) == 0x0) {
      (uVar7 + 0x41c) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0x14:
    if ((uVar7 + 0x418) == 0x0) {
      (uVar7 + 0x418) = 0x1;
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
      uVar9 = (uVar5 >> 0x10);
      pcVar12 = load_string_1010_847e(_u16_1050_14cc,0x72b);
      paVar6 = (astruct_57 *)CONCAT22(uVar9,(int)((u32)pcVar12 >> 0x10));
      pass1_1000_3cea(param_2 & 0xffff0000 | ZEXT24((char *)(uVar7 + 0xe)),pcVar12);
      *param_5 = 0x4c;
      uVar14 = 0x1;
      uVar15 = 0x0;
      iVar8 = 0xa;
      paVar11 = (astruct_27 *)
                mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd8a,in_stack_0000feae,
                                in_stack_0000feb4,in_stack_0000feb8);
      uVar10 = 0x1010;
      pass1_1010_089e(paVar11,CONCAT11(uVar15,uVar14),iVar8);
      break;
    }
    goto LAB_1008_a35a;
  case 0x16:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x28;
    break;
  case 0x17:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x2c;
    break;
  case 0x18:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x2e;
    break;
  case 0x1b:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x30;
    break;
  case 0x1c:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x32;
    break;
  case 0x1f:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x34;
    break;
  case 0x21:
    if ((uVar7 + 0x41e) == 0x0) {
      (uVar7 + 0x41e) = 0x1;
      break;
    }//
LAB_1008_a35a:
    *param_3 = 0x0;
    break;
  case 0x24:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x2a;
    break;
  case 0x31:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x27;
    break;
  case 0x32:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x29;
    break;
  case 0x33:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x2b;
    break;
  case 0x34:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x2d;
    break;
  case 0x35:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x2f;
    break;
  case 0x36:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x31;
    break;
  case 0x37:
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    pcVar12 = load_string_1010_847e(_u16_1050_14cc,0x71f);
    uVar10 = 0x1000;
    pass1_1000_3cea(param_2 & 0xffff0000 | ZEXT24((char *)(uVar7 + 0xe)),pcVar12);
    *param_5 = 0x33;
    break;
  case 0x38:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x35;
    break;
  case 0x39:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x36;
    break;
  case 0x3a:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x37;
    break;
  case 0x3b:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x38;
    break;
  case 0x3c:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0x39;
    break;
  case 0x3d:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xce;
    break;
  case 0x3e:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xcf;
    break;
  case 0x3f:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd0;
    break;
  case 0x40:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd1;
    break;
  case 0x41:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd2;
    break;
  case 0x42:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd3;
    break;
  case 0x43:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd5;
    break;
  case 0x44:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd6;
    break;
  case 0x45:
    uVar10 = 0x1010;
    load_string_1010_84e0(uVar13,uVar3,0x3ff,(char *)(uVar7 + 0xe),in_buf_len_5);
    *param_5 = 0xd7;
  }
  if (puStack6 != NULL) {
    ppcVar1 = (code **)*puStack6;
    (**ppcVar1)(uVar10,(int)puStack6,(char)((u32)puStack6 >> 0x10),0x1,uVar16);
  }
  return;
}



pub fn pass1_1008_a8f4(mut param_1: u16 ,mut param_2: u32,param_3: *mut u16,param_4: *mut u16,param_5: *mut u16) -> u32

{
  let mut iVar1: i16;
  uchar in_AF;
  let mut local_6: u32;

  iVar1 = (int)&local_6 + 0x2;
  pass1_1008_a1f0(in_AF,param_2,param_3,(u16 *)CONCAT22(0x1050,&local_6),(u16 *)CONCAT22(0x1050,iVar1),param_5);
  pass1_1008_944e(param_4,local_6,((u32)local_6 >> 0x10));
  return CONCAT22(param_1,iVar1);
}
pub fn pass1_1008_a930(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  astruct_57 *in_EDX;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut puStack24: *mut u16;
  let mut puStack18: *mut u16;
  u8 local_a [0x8];

  if (param_2 == 0x0) {
    return;
  }
  uVar6 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar5 + 0x410));
  do {
    puVar2 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
    uVar3 = in_EDX;
    uVar4 = uVar3 | puVar2;
    in_EDX = (astruct_57 *)((u32)in_EDX & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      mem_op_1000_179c(0x6,in_EDX);
      uVar3 = in_EDX;
      puStack24 = (u16 *)CONCAT22(uVar3,puVar2);
      if ((uVar3 | puVar2) == 0x0) {
        puStack18 = NULL;
      }
      else {
        *puStack24 = 0x389a;
        (puVar2 + 0x2) = 0x1008;
        (puVar2 + 0x4) = param_2;
        *puStack24 = 0xad8a;
        (puVar2 + 0x2) = 0x1008;
        puStack18 = puStack24;
      }
      ppcVar1 = (code **)((int)(u32)(u32)(iVar5 + 0x410) + 0x8);
      (**ppcVar1)(0x1000,(u32)(iVar5 + 0x410),puStack18);
      return;
    }
  } while ((puVar2 + 0x4) != param_2);
  return;
}



u16 pass1_1008_a9ec(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uStack4 = 0x0;
  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0x414) == 0x0) && (uVar1 = (u32)(iVar2 + 0x410), ((int)uVar1 + 0x8) != 0x0)) {
    (iVar2 + 0x414) = 0x1;
    pass1_1008_aa28(in_AX,param_1 & 0xffff | (u32)uVar3 << 0x10);
    uStack4 = in_AX;
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_aa28(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  u32 *puStack6;

  uVar4 = (param_2 >> 0x10);
  iVar3 = (int)param_2;
  if ((iVar3 + 0x414) != 0x0) {
    uVar2 = (u32)(iVar3 + 0x410);
    if (((int)uVar2 + 0x8) == 0x0) {
      (iVar3 + 0x414) = 0x0;
      return;
    }
    ppcVar1 = (code **)((int)(u32)(u32)(iVar3 + 0x410) + 0x10);
    (**ppcVar1)();
    puStack6 = (u32 *)CONCAT22(extraout_DX,param_1);
    if ((extraout_DX | param_1) != 0x0) {
      win_1008_5c5c(param_1,extraout_DX | param_1,_u16_1050_02a0,(param_1 + 0x4));
      if (puStack6 != NULL) {
        ppcVar1 = (code **)*puStack6;
        (**ppcVar1)();
      }
      return;
    }
  }
  return;
}



u16 pass1_1008_aaa8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uStack4: u16;

  uStack4 = 0x0;
  switch(param_3) {
  case 0x1:
    uStack4 = 0x24;
    break;
  case 0x2:
    uStack4 = 0x16;
    break;
  case 0x3:
    uStack4 = 0x17;
    break;
  case 0x4:
    uStack4 = 0x18;
    break;
  case 0x5:
    uStack4 = 0x1b;
    break;
  case 0x6:
    uStack4 = 0x1c;
    break;
  case 0x7:
    uStack4 = 0x1f;
  }
  return uStack4;
}



u16 pass1_1008_ab12(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  if (param_3 == 0x37) {
    return 0x22;
  }
  if (param_3 < 0x38) {
    if ((char)param_3 == '\r') {
      return 0xf;
    }
    if ((char)param_3 == '*') {
      return 0x2b;
    }
  }
  return 0x0;
}



u16 pass1_1008_ab54(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uStack4: u16;

  uStack4 = 0x0;
  uVar2 = (param_1 >> 0x10);
  if ((*(i32 *)((int)param_1 + 0xa) != 0x0) &&
     (uVar1 = (u32)((int)param_1 + 0xa), ((int)uVar1 + 0x8) != 0x0)) {
    uStack4 = 0x1;
  }
  return uStack4;
}



u16 pass1_1008_ab80(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uStack4: u16;

  uStack4 = 0x0;
  switch(param_3) {
  case 0x8:
    uStack4 = 0x82;
    break;
  case 0x9:
    uStack4 = 0x7f;
    break;
  case 0xa:
    uStack4 = 0x80;
    break;
  case 0xb:
    uStack4 = 0x84;
    break;
  case 0xc:
    uStack4 = 0x89;
    break;
  case 0xd:
    uStack4 = 0x8a;
    break;
  case 0xe:
    uStack4 = 0x8c;
    break;
  case 0xf:
    uStack4 = 0x8e;
    break;
  case 0x10:
    uStack4 = 0x8f;
    break;
  case 0x11:
    uStack4 = 0x90;
    break;
  case 0x12:
    uStack4 = 0x91;
    break;
  case 0x13:
    uStack4 = 0x95;
    break;
  case 0x14:
    uStack4 = 0x96;
    break;
  case 0x16:
    uStack4 = 0x9b;
    break;
  case 0x17:
    uStack4 = 0x9f;
    break;
  case 0x18:
    uStack4 = 0xa2;
    break;
  case 0x19:
    uStack4 = 0xa4;
    break;
  case 0x1b:
  case 0x1c:
    uStack4 = 0xa7;
    break;
  case 0x1d:
    uStack4 = 0xaa;
    break;
  case 0x1e:
    uStack4 = 0xac;
    break;
  case 0x1f:
    uStack4 = 0xad;
    break;
  case 0x20:
    uStack4 = 0xae;
    break;
  case 0x21:
    uStack4 = 0xb1;
    break;
  case 0x22:
    uStack4 = 0xb3;
    break;
  case 0x23:
    uStack4 = 0xb4;
    break;
  case 0x24:
    uStack4 = 0xb5;
    break;
  case 0x25:
    uStack4 = 0xb6;
    break;
  case 0x26:
    uStack4 = 0xb7;
    break;
  case 0x27:
    uStack4 = 0xab;
    break;
  case 0x28:
    uStack4 = 0xb9;
    break;
  case 0x29:
    uStack4 = 0xba;
    break;
  case 0x2a:
    uStack4 = 0xbc;
    break;
  case 0x2b:
    uStack4 = 0xbe;
    break;
  case 0x2c:
    uStack4 = 0xdf;
    break;
  case 0x2d:
    uStack4 = 0xe0;
  }
  return uStack4;
}



u16 * pass1_1008_ad0c(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((astruct_611 *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_ad38(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



pub fn pass1_1008_ad64(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_a086((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_ada2(param_1: *mut u16,mut param_2: i16)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x0;
  ((int)param_1 + 0x2) = 0x0;
  ((int)param_1 + 0x4) = param_2;
  *param_1 = (param_2 * 0x6 + 0x3a4);
  return param_1;
}
pub fn pass1_1008_add2(param_1: *mut u16)

{
  *param_1 = (((int)param_1 + 0x4) * 0x6 + 0x3a4);
  return;
}



u16 pass1_1008_adf2(mut param_1: u32)

{
  return (((int)param_1 + 0x4) * 0x6 + 0x3a4);
}



u16 pass1_1008_ae0c(mut param_1: u32)

{
  return (((int)param_1 + 0x4) * 0x6 + 0x3a6);
}
pub fn pass1_1008_ae26(i16 *param_1)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = ((iVar3 + 0x4) * 0x6 + 0x3a8);
  if (iVar2 == 0x2) {
    if ((iVar3 + 0x2) == 0x1) {
      *param_1 = *param_1 + -0x1;
      iVar2 = (iVar3 + 0x4) * 0x6;
      piVar1 = (i16 *)(iVar2 + 0x3a4);
      if (*piVar1 != *param_1 && *param_1 <= *piVar1) {
        *param_1 = (iVar2 + 0x3a4) + 0x1;
        (iVar3 + 0x2) = 0x0;
        return;
      }
    }
    else {
      *param_1 = *param_1 + 0x1;
      iVar2 = (iVar3 + 0x4) * 0x6;
      if ((iVar2 + 0x3a6) < *param_1) {
        *param_1 = (iVar2 + 0x3a6) + -0x1;
        (iVar3 + 0x2) = 0x1;
        return;
      }
    }
  }
  else if ((iVar2 != 0x3) && (iVar2 != 0x4)) {
    *param_1 = *param_1 + 0x1;
    iVar2 = (iVar3 + 0x4) * 0x6;
    if ((iVar2 + 0x3a6) < *param_1) {
      *param_1 = (iVar2 + 0x3a4);
    }
  }
  return;
}



BOOL16 pass1_1008_aed8(mut param_1: u32)

{
  if ((((int)param_1 + 0x4) * 0x6 + 0x3a4) != 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_aefe(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 ) -> u32

{
  struct_op_1018_4cda(param_2,param_3);
  param_2->offset_0x0 = 0xaf7c;
  ((int)param_2 + 0x2) = 0x1008;
  _PTR_LOOP_1050_4230 = param_2;
  pass1_1018_4dce(param_1,param_2,0x1b3);
  return (u32)param_2;
}
pub fn pass1_1008_af38(StructD *param_1)

{
  param_1->address_offset_field_0x0 = 0xaf7c;
  ((int)param_1 + 0x2) = 0x1008;
  clenaup_win_ui_1018_4d22(param_1);
  return;
}



StructD * pass1_1008_af56(StructD *param_1,u8 param_2)

{
  pass1_1008_af38(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1008_af94(param_1: *mut astruct_19,mut param_2: u16 ,mut param_3: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  (u32)((int)param_1 + 0x16) = 0x0;
  (u32)((int)param_1 + 0x1a) = 0x0;
  (u32)((int)param_1 + 0x1e) = 0x0;
  ((int)param_1 + 0x22) = 0x0;
  param_1->offset_0x0 = 0xbdcc;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1008_afde(param_1: *mut astruct_455)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  code **ppcVar4;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xbdcc;
  iVar4->field1_0x2 = 0x1008;
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



u16 * pass1_1008_b05a(StructD *param_1)

{
  StructD *iVar1;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  (u32)&iVar1->hfile_0x4 = 0x0;
  param_1->address_offset_field_0x0 = 0xbdc8;
  iVar1->address_offset_field_0x2 = 0x1008;
  return &param_1->address_offset_field_0x0;
}
pub fn pass1_1008_b08c(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0xbdc8;
  (iVar1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(char **)(iVar1 + 0x4));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}
pub fn set_stuct_1008_b0bc(StructD *param_1)

{
  StructD *iVar1;
  StructD *uVar1;

  pass1_1008_b05a(param_1);
  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  iVar1->field5_0x8 = 0x0;
  (u32)&iVar1->field6_0xa = 0x0;
  iVar1->field8_0xe = 0x0;
  &iVar1->field_0x10 = 0x0;
  param_1->address_offset_field_0x0 = 0xbdc4;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



u16 * pass1_1008_b0f2(StructD *param_1)

{
  StructD *uVar1;

  pass1_1008_b05a(param_1);
  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x8) = 0x0;
  param_1->address_offset_field_0x0 = 0xbdc0;
  ((int)param_1 + 0x2) = 0x1008;
  return &param_1->address_offset_field_0x0;
}



u16 * pass1_1008_b11e(StructD *param_1)

{
  StructD *uVar1;

  pass1_1008_b05a(param_1);
  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  ((int)param_1 + 0x8) = 0x0;
  param_1->address_offset_field_0x0 = 0xbddc;
  ((int)param_1 + 0x2) = 0x1008;
  return &param_1->address_offset_field_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_b146(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_3 >> 0x10);
  iVar2 = (int)param_3;
  if (*(i32 *)(iVar2 + 0x16) != 0x0) {
    uVar1 = (u32)(iVar2 + 0x16);
    pass1_1030_8344(_u16_1050_5748,(u32)((int)uVar1 + 0xa));
    pass1_1038_3608(CONCAT22(param_2,param_1));
    uVar1 = (u32)(iVar2 + 0x16);
    ((int)uVar1 + 0x8) = 0x0;
    uVar1 = (u32)(iVar2 + 0x16);
    (u32)((int)uVar1 + 0xa) = 0x0;
    uVar1 = (u32)(iVar2 + 0x16);
    ((int)uVar1 + 0xe) = 0x0;
    uVar1 = (u32)(iVar2 + 0x16);
    ((int)uVar1 + 0x10) = 0x0;
  }
  return;
}
pub fn pass1_1008_b1a6(mut param_1: u32,char *param_2)

{
  i32 lVar1;
  let mut uVar2: u16;
  let mut in_DX: u16;
  astruct_467 *iVar3;
  astruct_466 *iVar4;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_467 *)param_1;
  if (iVar3->field22_0x16 != 0x0) {
    lVar1 = iVar3->field22_0x16;
    fn_ptr_1000_17ce(*(char **)((int)lVar1 + 0x4));
    uVar2 = str_op_1008_60e8(in_DX,param_2);
    lVar1 = iVar3->field22_0x16;
    uVar4 = ((u32)lVar1 >> 0x10);
    iVar4 = (astruct_466 *)lVar1;
    iVar4->field4_0x4 = uVar2;
    iVar4->field5_0x6 = in_DX;
    iVar3->field22_0x16 = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1008_b1f0(void)

{
  char *pcVar1;

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,0x531);
  return pcVar1;
}
pub fn pass1_1008_b200(param_1: *mut astruct_194)

{
  let mut uVar1: u32;
  u32 *puVar2;
  code **ppcVar3;
  u32 *puVar4;
  astruct_92 *paVar5;
  astruct_195 *uVar5;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar11;
  astruct_57 *paVar13;
  let mut uVar14: u32;
  astruct_194 *iVar12;
  let mut uVar15: u16;
  let mut puVar16: *mut u16;
  char *pcVar17;
  char *pcStack24;
  astruct_92 local_14;
  astruct_57 *paVar12;

  uVar15 = ((u32)param_1 >> 0x10);
  iVar12 = (astruct_194 *)param_1;
  if (iVar12->field14_0xe != NULL) {
    return;
  }
    // WARNING: Load size is inaccurate
  puVar4 = iVar12->field14_0xe;
  uVar9 = ((int)&iVar12->field14_0xe + 0x2);
  paVar11 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar9);
  if ((uVar9 | puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar11);
  uVar9 = paVar11 | puVar4;
  paVar13 = (astruct_57 *)((u32)paVar11 & 0xffff0000);
  paVar12 = (astruct_57 *)((u32)paVar13 | (u32)uVar9);
  if (uVar9 == 0x0) {
    puVar4 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar11,puVar4));
    paVar13 = paVar12;
  }
  (u32*)&iVar12->field14_0xe = puVar4;
  ((int)&iVar12->field14_0xe + 0x2) = (int)paVar13;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    paVar5 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar5));
    uVar9 = paVar13;
    pcStack24 = (char *)CONCAT22(uVar9,paVar5);
    paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)(uVar9 | paVar5));
    if ((uVar9 | paVar5) == 0x0) break;
    uVar1 = paVar5->field3_0x4;
    if (paVar5[0x1c].field4_0x8 == 0x8000001) {
      uVar8 = uVar1;
      mem_op_1000_179c(0xc,paVar13);
      uVar5 = (astruct_195 *)uVar8;
      uVar14 = (u32)paVar13 & 0xffff0000;
      if ((paVar13 | uVar5) == 0x0) {
        iVar6 = 0x0;
      }
      else {
        puVar16 = pass1_1008_b0f2((StructD *)(uVar8 & 0xffff | (long)paVar13 << 0x10));
        uVar14 = uVar14 & 0xffff0000 | (u32)puVar16 >> 0x10;
        iVar6 = (int)puVar16;
      }
      uVar10 = uVar14;
      pcVar17 = pass1_1038_4d28(pcStack24);
      paVar13 = (astruct_57 *)(uVar14 & 0xffff0000 | (u32)pcVar17 >> 0x10);
      uVar7 = ((u32)pcVar17 >> 0x10);
      uVar7 = str_op_1008_60e8(uVar7,(char *)((u32)pcVar17 & 0xffff | (u32)uVar7 << 0x10));
      (iVar6 + 0x4) = uVar7;
      (iVar6 + 0x6) = (int)paVar13;
      (u32)(iVar6 + 0x8) = uVar1;
      puVar2 = iVar12->field14_0xe;
      ppcVar3 = (code **)((int)*iVar12->field14_0xe + 0x8);
      (**ppcVar3)(0x38,(int)puVar2,(int)((u32)puVar2 >> 0x10));
    }
  }
  return;
}



pub fn pass1_1008_b340(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x16) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x16);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



pub fn pass1_1008_b366(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x1a);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b38c(StructD *param_1,param_2: *mut astruct_196) -> u32

{
  u32 *puVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  astruct_197 *iVar3;
  astruct_197 *paVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_57 *paVar7;
  let mut uVar9: u32;
  astruct_196 *iVar4;
  astruct_196 *uVar4;
  let mut puVar10: *mut u16;
  let mut uStack4: u16;
  astruct_57 *paVar8;

  uVar3 = ((u32)param_1 >> 0x10);
  paVar7 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  uVar4 = (astruct_196 *)((u32)param_2 >> 0x10);
  iVar4 = (astruct_196 *)param_2;
  if (iVar4->field18_0x12 == NULL) {
    mem_op_1000_179c(0xc,paVar7);
    uVar5 = paVar7 | uVar3;
    paVar8 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)uVar5);
    if (uVar5 == 0x0) {
      iVar4->field18_0x12 = NULL;
    }
    else {
      uVar3 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar7,uVar3));
      &iVar4->field18_0x12 = uVar3;
      ((int)&iVar4->field18_0x12 + 0x2) = (int)paVar8;
    }
    for (uStack4 = 0x6d9; (int)uStack4 < 0x6e7; uStack4 += 0x1) {
      if (uStack4 == 0x6e3) {
        pass1_1030_8344(_u16_1050_5748,0x8000001);
        if ((uVar3 + 0x136) != 0x0) goto LAB_1008_b44a;
      }
      else {//
LAB_1008_b44a:
        mem_op_1000_179c(0xa,paVar8);
        uVar9 = (u32)paVar8 & 0xffff0000;
        if ((paVar8 | uVar3) == 0x0) {
          iVar3 = NULL;
          paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000);
        }
        else {
          puVar10 = pass1_1008_b11e((StructD *)CONCAT22(paVar8,uVar3));
          paVar8 = (astruct_57 *)(uVar9 & 0xffff0000 | (u32)puVar10 >> 0x10);
          iVar3 = (astruct_197 *)puVar10;
        }
        uVar6 = SUB42(paVar8,0x0);
        paVar4 = iVar3;
        load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),uStack4);
        iVar3->field4_0x4 = (int)paVar4;
        iVar3->field5_0x6 = (int)paVar8;
        iVar3->field6_0x8 = uStack4 - 0x6d8;
        puVar1 = iVar4->field18_0x12;
        ppcVar2 = (code **)((int)*iVar4->field18_0x12 + 0x8);
        uVar3 = (**ppcVar2)(0x1010,(int)puVar1,(int)((u32)puVar1 >> 0x10),iVar3,uVar6);
      }
    }
  }
  return CONCAT22(((int)&iVar4->field18_0x12 + 0x2),&iVar4->field18_0x12);
}



pub fn pass1_1008_b47a(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1e) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x1e);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}
pub fn pass1_1008_b4a0(mut param_1: u16 ,u8 *param_2,mut param_3: u32,i32 param_4)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  i32 lVar7;

  uVar6 = (u32)param_1;
  iVar4 = (int)param_3;
  uVar5 = (param_3 >> 0x10);
  if (param_4 == 0x0) {
    (u32)(iVar4 + 0x16) = 0x0;
  }
  else {
    pass1_1008_b9ce(param_3,(char *)param_4);
    (iVar4 + 0x16) = (int)uVar6;
    *(u8 **)(iVar4 + 0x18) = param_2;
  }
  uVar1 = (u32)(iVar4 + 0x16);
  if (((int)uVar1 + 0x8) != 0x0) {
    pass1_1008_b200((astruct_194 *)param_3);
    uVar6 = pass1_1008_b38c((StructD *)CONCAT22((int)uVar6,param_2),(astruct_196 *)param_3);
    uVar3 = (uVar6 >> 0x10);
    uVar2 = uVar6;
    uVar1 = (u32)(iVar4 + 0x16);
    pass1_1008_b85c(param_3,*(i32 *)((int)uVar1 + 0xa));
    (iVar4 + 0x1a) = uVar2;
    (iVar4 + 0x1c) = uVar3;
    uVar1 = (u32)(iVar4 + 0x16);
    lVar7 = pass1_1008_b8ac(param_3,((int)uVar1 + 0xe));
    (iVar4 + 0x1e) = (int)lVar7;
    (iVar4 + 0x20) = (int)((u32)lVar7 >> 0x10);
    return;
  }
  (u32)(iVar4 + 0x1a) = 0x0;
  (u32)(iVar4 + 0x1e) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_b544(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut unaff_CS: u16;

  iVar7 = (int)param_2;
  uVar8 = (param_2 >> 0x10);
  if (param_3 != 0x0) {
    if (*(i32 *)(iVar7 + 0x1a) != 0x0) {
      uVar4 = (u32)(iVar7 + 0x16);
      ((int)uVar4 + 0x8) = 0x1;
      uVar4 = (u32)(iVar7 + 0x1a);
      uVar5 = (u32)(iVar7 + 0x16);
      (u32)((int)uVar5 + 0xa) = (u32)((int)uVar4 + 0x8);
      uVar4 = (u32)(iVar7 + 0x1e);
      uVar6 = ((int)uVar4 + 0x8);
      uVar4 = (u32)(iVar7 + 0x16);
      ((int)uVar4 + 0xe) = uVar6;
      uVar4 = (u32)(iVar7 + 0x16);
      pass1_1030_8344(_u16_1050_5748,(u32)((int)uVar4 + 0xa));
      unaff_CS = SUB42(&u16_1050_1038,0x0);
      pass1_1038_3608(CONCAT22(param_1,uVar6));
    }
  }
  (u32)(iVar7 + 0x1e) = 0x0;
  (u32)(iVar7 + 0x1a) = 0x0;
  (u32)(iVar7 + 0x16) = 0x0;
  puVar1 = (u32 *)(iVar7 + 0xe);
  uVar2 = (iVar7 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1);
  }
  (u32)(iVar7 + 0xe) = 0x0;
  puVar1 = (u32 *)(iVar7 + 0x12);
  uVar2 = (iVar7 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1);
  }
  (u32)(iVar7 + 0x12) = 0x0;
  return;
}
pub fn pass1_1008_b61a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;

  pass1_1008_b8fa(param_2,param_3,(char *)param_4);
  uVar1 = (param_3 >> 0x10);
  ((int)param_3 + 0x1a) = param_1;
  ((int)param_3 + 0x1c) = param_2;
  return;
}
pub fn pass1_1008_b63a(mut param_1: u32,mut param_2: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u16;

  pass1_1008_b964(param_1,(char *)param_2);
  uVar1 = (param_1 >> 0x10);
  ((int)param_1 + 0x1e) = in_AX;
  ((int)param_1 + 0x20) = in_DX;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_string_1008_b65a(mut param_1: u32,LPSTR in_string_2,mut param_3: u32,mut param_4: u16 )

{
  pass1_1008_b9ce(param_1,(char *)CONCAT22(param_4,param_3));
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,in_string_2,(short)param_3)
  ;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_str_and_sprintf_1008_b69c(mut param_1: u16 ,param_2: *mut astruct_25)

{
  code **ppcVar1;
  char *in_buffer_4;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  astruct_57 *paVar10;
  astruct_25 *iVar5;
  astruct_25 *uVar5;
  let mut iStack516: i16;
  char local_202 [0x100];
  WORD local_102 [0x80];
  astruct_57 *paVar9;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  in_buffer_4 = local_202;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,in_buffer_4,(short)&DAT_1050_1050);
  uVar5 = (astruct_25 *)((u32)param_2 >> 0x10);
  iVar5 = (astruct_25 *)param_2;
  if (iVar5->field10_0xa == NULL) {
    mem_op_1000_179c(0xc,paVar8);
    uVar4 = paVar8 | in_buffer_4;
    paVar10 = (astruct_57 *)((u32)paVar8 & 0xffff0000);
    paVar9 = (astruct_57 *)((u32)paVar10 | (u32)uVar4);
    if (uVar4 == 0x0) {
      uVar4 = 0x0;
    }
    else {
      uVar4 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar8,in_buffer_4));
      paVar10 = paVar9;
    }
    &iVar5->field10_0xa = uVar4;
    ((int)&iVar5->field10_0xa + 0x2) = (int)paVar10;
    for (iStack516 = 0x1; iStack516 < 0x6; iStack516 += 0x1) {
      mem_op_1000_179c(0x12,paVar10);
      uVar7 = paVar10 | uVar4;
      paVar8 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar7);
      if (uVar7 == 0x0) {
        iVar3 = 0x0;
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000);
      }
      else {
        iVar3 = set_stuct_1008_b0bc((StructD *)CONCAT22(paVar10,uVar4));
        paVar10 = paVar8;
      }
      uVar6 = SUB42(paVar10,0x0);
      wsprintf16(local_102,(char *)CONCAT22(local_202,0x1050),(char *)CONCAT22(iStack516,0x1050),iVar3,uVar6,uVar4);
      uVar2 = str_op_1008_60e8(paVar10,(char *)CONCAT22(0x1050,local_102));
      (iVar3 + 0x4) = uVar2;
      (iVar3 + 0x6) = (int)paVar10;
      ppcVar1 = (code **)((int)*iVar5->field10_0xa + 0x8);
      uVar4 = (**ppcVar1)();
    }
    iVar5->field31_0x22 = 0x5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_str_and_sprintf_1008_b78a(StructD *param_1,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  astruct_57 *paVar6;
  let mut iVar7: i16;
  let mut uVar8: u16;
  char local_206 [0x100];
  WORD local_106 [0x80];
  let mut iStack6: i16;
  let mut uStack4: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  paVar6 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  mem_op_1000_179c(0x12,paVar6);
  uVar5 = paVar6 | uVar3;
  if (uVar5 == 0x0) {
    iStack6 = 0x0;
    uVar5 = 0x0;
  }
  else {
    iStack6 = set_stuct_1008_b0bc((StructD *)CONCAT22(paVar6,uVar3));
  }
  uStack4 = uVar5;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  uVar8 = (param_2 >> 0x10);
  iVar7 = (int)param_2;
  piVar1 = (i16 *)(iVar7 + 0x22);
  *piVar1 = *piVar1 + 0x1;
  wsprintf16(local_106,(char *)CONCAT22(local_206,0x1050),(char *)CONCAT22((iVar7 + 0x22),0x1050),uVar3);
  uVar4 = str_op_1008_60e8(uVar5,(char *)CONCAT22(0x1050,local_106));
  (iStack6 + 0x4) = uVar4;
  (iStack6 + 0x6) = uVar5;
  ppcVar2 = (code **)((int)(u32)(u32)(iVar7 + 0xa) + 0x8);
  (**ppcVar2)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b820(mut param_1: i16,mut param_2: u16 ,mut param_3: u32) -> u32

{
  let mut uVar1: u16;

  pass1_1030_8344(_u16_1050_5748,0x8000001);
  if ((param_1 + 0x152) == 0x0) {
    return 0x0;
  }
  uVar1 = (param_3 >> 0x10);
  return CONCAT22(((int)param_3 + 0xc),((int)param_3 + 0xa));
}
pub fn pass1_1008_b85c(mut param_1: u32,i32 param_2)

{
  u8 *puVar1;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (*(i32 *)(puVar1 + 0x8) != param_2);
  return;
}



i32 pass1_1008_b8ac(mut param_1: u32,mut param_2: i16)

{
  i32 lVar1;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x12));
  do {
    lVar1 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar1 == 0x0) {
      return 0x0;
    }
  } while (((int)lVar1 + 0x8) != param_2);
  return lVar1;
}
pub fn pass1_1008_b8fa(mut param_1: u16 ,mut param_2: u32,char *param_3)

{
  u8 *puVar1;
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  if (param_3 == NULL) {
    return;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_2 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(char **)(puVar1 + 0x4),param_3);
  } while (iVar2 != 0x0);
  return;
}
pub fn pass1_1008_b964(mut param_1: u32,char *param_2)

{
  char *string_1;
  let mut iVar1: i16;
  let mut extraout_DX: u16;
  char local_a [0x8];

  if (param_2 == NULL) {
    return;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x12));
  do {
    string_1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,string_1));
    if ((extraout_DX | string_1) == 0x0) {
      return;
    }
    iVar1 = pass1_1000_3d7a(*(char **)(string_1 + 0x4),param_2);
  } while (iVar1 != 0x0);
  return;
}
pub fn pass1_1008_b9ce(mut param_1: u32,char *param_2)

{
  u8 *puVar1;
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  if (param_2 == NULL) {
    return;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(char **)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}
pub fn pass1_1008_ba38(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2:bool;
  u8 *puVar3;
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  HFILE16 in_stack_0000ffc0;
  u32 local_2a [0x3];
  u16 local_1e [0x5];
  u8 local_14 [0x8];
  let mut local_c: u16;
  let mut uStack10: u32;
  u16 local_6 [0x2];

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 != 0x0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = (int)param_1;
    local_c = (iVar4 + 0x22);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffc0);
    if (BVar2 != 0x0) {
      if (*(i32 *)(iVar4 + 0xa) == 0x0) {
        local_c = 0x0;
      }
      else {
        uVar1 = (u32)(iVar4 + 0xa);
        local_c = ((int)uVar1 + 0x8);
      }
      local_1e[0] = local_c;
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1e),(char *)0x2,in_stack_0000ffc0);
      if (BVar2 != 0x0) {
        pass1_1008_5784((char *)CONCAT22(0x1050,local_14),(u32)(iVar4 + 0xa));
        do {
          puVar3 = local_14;
          pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
          uStack10 = CONCAT22(extraout_DX,puVar3);
          if ((extraout_DX | puVar3) == 0x0) {
            return;
          }
          BVar2 = pass1_1008_7c2a(param_2,*(char **)(puVar3 + 0x4));
          if (BVar2 == 0x0) break;
          local_6[0] = ((int)uStack10 + 0x8);
          BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffc0);
          if (BVar2 == 0x0) break;
          local_2a[0] = (u32)((int)uStack10 + 0xa);
          BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_2a),(char *)0x4,in_stack_0000ffc0);
          if (BVar2 == 0x0) break;
          local_6[0] = ((int)uStack10 + 0xe);
          BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffc0);
        } while (BVar2 != 0x0);
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn file_1008_bb5e(mut param_1: i16,StructD *param_2,param_3: *mut astruct_199,mut param_4: u32)

{
  code **ppcVar1;
  astruct_199 *iVar3;
  let mut BVar2:bool;
  StructD *uVar3;
  astruct_200 *uVar4;
  u8 *puVar3;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  astruct_57 *paVar10;
  let mut uVar13: u16;
  let mut uVar14: u16;
  astruct_200 *paStack286;
  u32 *puStack284;
  u8 local_118 [0x100];
  u16 local_18 [0x2];
  u16 local_14 [0x2];
  astruct_200 *local_10 [0x4];
  let mut local_8: u32;
  astruct_199 *uVar12;
  astruct_199 *uVar11;
  astruct_199 *uVar2;
  astruct_57 *paVar9;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if ((int)u16_1050_0312 < 0x2) {
    return;
  }
  uVar14 = (param_4 >> 0x10);
  read_file_1008_7cfe((HFILE16)param_4,uVar14,0x16);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    iVar3 = (astruct_199 *)param_3;
    iVar3 = (astruct_199 *)&iVar3->field31_0x22;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(iVar3)),0x2);
    if ((BVar2 != 0x0) &&
       (uVar3 = (StructD *)read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_10),0x2), uVar3 != NULL))
    {
      if (local_10[0] == NULL) {
        return;
      }
      mem_op_1000_179c(0xc,paVar8);
      uVar6 = paVar8 | uVar3;
      paVar10 = (astruct_57 *)((u32)paVar8 & 0xffff0000);
      paVar9 = (astruct_57 *)((u32)paVar10 | (u32)uVar6);
      if (uVar6 == 0x0) {
        uVar3 = NULL;
      }
      else {
        set_struct_1008_574a((astruct_57 *)CONCAT22(paVar8,uVar3));
        paVar10 = paVar9;
      }
      uVar13 = ((u32)param_3 >> 0x10);
      *(StructD **)&iVar3->field10_0xa = uVar3;
      ((int)&iVar3->field10_0xa + 0x2) = (int)paVar10;
      paStack286 = NULL;
      while( true ) {
        if (local_10[0] <= paStack286) {
          return;
        }
        uVar4 = local_10[0];
        mem_op_1000_179c(0x12,paVar10);
        uVar6 = paVar10 | uVar4;
        paVar8 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar6);
        if (uVar6 == 0x0) {
          uVar4 = NULL;
          paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000);
        }
        else {
          set_stuct_1008_b0bc((StructD *)CONCAT22(paVar10,uVar4));
          paVar10 = paVar8;
        }
        uVar7 = SUB42(paVar10,0x0);
        puStack284 = (u32 *)CONCAT22(uVar7,uVar4);
        puVar3 = local_118;
        read_file_1008_7c6e((HFILE16)param_4,uVar14,(char *)CONCAT22(0x1050,puVar3));
        if ((((puVar3 == NULL) ||
             (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_14),0x2), BVar2 == 0x0)) ||
            (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,&local_8),0x4), BVar2 == 0x0)) ||
           (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_18),0x2), BVar2 == 0x0)) break;
        uVar5 = str_op_1008_60e8(paVar10,(char *)CONCAT22(0x1050,local_118));
        uVar4->field3_0x4 = uVar5;
        uVar4->field4_0x6 = paVar10;
        uVar4->field5_0x8 = local_14[0];
        uVar4->field6_0xa = local_8;
        uVar4->field7_0xe = local_18[0];
        ppcVar1 = (code **)((int)(u32)iVar3->field10_0xa + 0x8);
        (**ppcVar1)();
        paStack286 = (astruct_200 *)&paStack286->field1_0x1;
      }
      if (puStack284 != NULL) {
        ppcVar1 = (code **)*puStack284;
        (**ppcVar1)(0x1000,uVar4,uVar7,0x1,puStack284);
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



pub fn pass1_1008_bd02(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_afde((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd28(StructD *param_1,u8 param_2)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd4e(StructD *param_1,u8 param_2)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd74(StructD *param_1,u8 param_2)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd9a(StructD *param_1,u8 param_2)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1008_bde0(StructD *param_1,param_2: *mut astruct_139)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  StructD *pSVar2;
  astruct_139 *iVar2;
  astruct_139 *iVar3;
  astruct_139 *iVar4;
  astruct_139 *iVar5;
  astruct_139 *iVar6;
  astruct_139 *iVar7;
  astruct_139 *iVar8;
  astruct_139 *iVar9;
  astruct_139 *iVar10;
  astruct_139 *iVar11;
  astruct_139 *iVar12;
  astruct_139 *iVar2_00;
  astruct_139 *iVar2_01;
  astruct_139 *iVar2_02;
  astruct_139 *iVar2_03;
  astruct_139 *iVar2_04;
  astruct_139 *iVar2_05;
  astruct_139 *iVar2_06;
  astruct_139 *iVar2_07;
  astruct_139 *iVar2_08;
  astruct_139 *iVar2_09;
  let mut iVar13: i16;
  astruct_139 *iVar2_10;
  astruct_139 *iVar2_11;
  astruct_139 *iVar2_12;
  astruct_139 *iVar2_13;
  astruct_139 *iVar2_14;
  astruct_139 *iVar2_15;
  astruct_139 *iVar2_16;
  astruct_139 *iVar2_17;
  astruct_139 *iVar2_18;
  astruct_139 *iVar2_19;
  astruct_139 *iVar2_20;
  astruct_139 *iVar2_21;
  astruct_139 *iVar2_22;
  astruct_139 *iVar2_23;
  astruct_139 *iVar2_24;
  astruct_139 *iVar2_25;
  astruct_139 *iVar2_26;
  astruct_139 *iVar2_27;
  astruct_139 *iVar2_28;
  astruct_139 *iVar2_29;
  astruct_139 *iVar2_30;
  astruct_139 *iVar2_31;
  astruct_139 *iVar2_32;
  astruct_139 *iVar2_33;
  astruct_139 *iVar2_34;
  astruct_139 *iVar2_35;
  astruct_139 *iVar2_36;
  astruct_139 *iVar2_37;
  astruct_139 *iVar2_38;
  astruct_139 *iVar2_39;
  astruct_139 *iVar2_40;
  astruct_139 *iVar2_41;
  astruct_139 *iVar2_42;
  astruct_139 *iVar2_43;
  astruct_139 *iVar2_44;
  astruct_139 *iVar2_45;
  astruct_139 *iVar2_46;
  astruct_139 *iVar2_47;
  astruct_139 *iVar2_48;
  astruct_139 *iVar2_49;
  astruct_139 *iVar2_50;
  let mut uVar3: u16;
  let mut uVar14: u16;

  pSVar2 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  _u16_1050_06e0 = param_2;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
    PTR_LOOP_1050_5f2e = (u8 *)pSVar2;
  }
  else {
  }
  uVar1 = fn_ptr_op_1000_1708(0x1aa,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  param_2 = uVar1;
  ((int)param_2 + 0x2) = PTR_LOOP_1050_5f2e;
  uVar3 = ((u32)(u32)param_2 >> 0x10);
  iVar2 = (astruct_139 *)(u32)param_2;
  iVar2->field5_0x6 = 0x6e4;
  iVar2->field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xa) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar3 = (astruct_139 *)(u32)param_2;
  iVar3[0x1].field2_0x2 = 0x6ea;
  &iVar3[0x1].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x10) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar4 = (astruct_139 *)(u32)param_2;
  iVar4[0x1].field6_0x8 = 0x6ee;
  (iVar4 + 0x2) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x16) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar5 = (astruct_139 *)(u32)param_2;
  &iVar5[0x2].field_0x4 = 0x6f2;
  iVar5[0x2].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x1c) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar6 = (astruct_139 *)(u32)param_2;
  (iVar6 + 0x3) = 0x6f6;
  iVar6[0x3].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x22) = 0x4;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar7 = (astruct_139 *)(u32)param_2;
  iVar7[0x3].field5_0x6 = 0x6fe;
  iVar7[0x3].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x28) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar8 = (astruct_139 *)(u32)param_2;
  iVar8[0x4].field2_0x2 = 0x702;
  &iVar8[0x4].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x2e) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar9 = (astruct_139 *)(u32)param_2;
  iVar9[0x4].field6_0x8 = 0x708;
  (iVar9 + 0x5) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x34) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar10 = (astruct_139 *)(u32)param_2;
  &iVar10[0x5].field_0x4 = 0x70e;
  iVar10[0x5].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x3a) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar11 = (astruct_139 *)(u32)param_2;
  (iVar11 + 0x6) = 0x714;
  iVar11[0x6].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x40) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar12 = (astruct_139 *)(u32)param_2;
  iVar12[0x6].field5_0x6 = 0x71a;
  iVar12[0x6].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x46) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_00 = (astruct_139 *)(u32)param_2;
  iVar2_00[0x7].field2_0x2 = 0x71e;
  &iVar2_00[0x7].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x4c) = 0x7;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_01 = (astruct_139 *)(u32)param_2;
  iVar2_01[0x7].field6_0x8 = 0x72c;
  (iVar2_01 + 0x8) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x52) = 0x6;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_02 = (astruct_139 *)(u32)param_2;
  &iVar2_02[0x8].field_0x4 = 0x738;
  iVar2_02[0x8].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x58) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_03 = (astruct_139 *)(u32)param_2;
  (iVar2_03 + 0x9) = 0x73e;
  iVar2_03[0x9].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x5e) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_04 = (astruct_139 *)(u32)param_2;
  iVar2_04[0x9].field5_0x6 = 0x744;
  iVar2_04[0x9].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x64) = 0x4;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_05 = (astruct_139 *)(u32)param_2;
  iVar2_05[0xa].field2_0x2 = 0x74c;
  &iVar2_05[0xa].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x6a) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_06 = (astruct_139 *)(u32)param_2;
  iVar2_06[0xa].field6_0x8 = 0x750;
  (iVar2_06 + 0xb) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x70) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_07 = (astruct_139 *)(u32)param_2;
  &iVar2_07[0xb].field_0x4 = 0x756;
  iVar2_07[0xb].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x76) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_08 = (astruct_139 *)(u32)param_2;
  (iVar2_08 + 0xc) = 0x75a;
  iVar2_08[0xc].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x7c) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_09 = (astruct_139 *)(u32)param_2;
  iVar2_09[0xc].field5_0x6 = 0x75e;
  iVar2_09[0xc].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x82) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar13 = (int)(u32)param_2;
  (iVar13 + 0x84) = 0x764;
  (iVar13 + 0x86) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x88) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_10 = (astruct_139 *)(u32)param_2;
  iVar2_10[0xd].field6_0x8 = 0x76a;
  (iVar2_10 + 0xe) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x8e) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_11 = (astruct_139 *)(u32)param_2;
  &iVar2_11[0xe].field_0x4 = 0x770;
  iVar2_11[0xe].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x94) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar13 = (int)(u32)param_2;
  (iVar13 + 0x96) = 0x774;
  (iVar13 + 0x98) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x9a) = 0x4;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_12 = (astruct_139 *)(u32)param_2;
  iVar2_12[0xf].field5_0x6 = 0x77c;
  iVar2_12[0xf].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xa0) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_13 = (astruct_139 *)(u32)param_2;
  iVar2_13[0x10].field2_0x2 = 0x780;
  &iVar2_13[0x10].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xa6) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_14 = (astruct_139 *)(u32)param_2;
  iVar2_14[0x10].field6_0x8 = 0x782;
  (iVar2_14 + 0x11) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xac) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar13 = (int)(u32)param_2;
  (iVar13 + 0xae) = 0x786;
  (iVar13 + 0xb0) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xb2) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_15 = (astruct_139 *)(u32)param_2;
  (iVar2_15 + 0x12) = 0x78a;
  iVar2_15[0x12].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0xb8) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_16 = (astruct_139 *)(u32)param_2;
  iVar2_16[0x12].field5_0x6 = 0x78e;
  iVar2_16[0x12].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xbe) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_17 = (astruct_139 *)(u32)param_2;
  iVar2_17[0x13].field2_0x2 = 0x792;
  &iVar2_17[0x13].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xc4) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar13 = (int)(u32)param_2;
  (iVar13 + 0xc6) = 0x796;
  (iVar13 + 0xc8) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xca) = 0x4;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_18 = (astruct_139 *)(u32)param_2;
  &iVar2_18[0x14].field_0x4 = 0x79e;
  iVar2_18[0x14].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0xd0) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar13 = (int)(u32)param_2;
  (iVar13 + 0xd2) = 0x7a0;
  (iVar13 + 0xd4) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xd6) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_19 = (astruct_139 *)(u32)param_2;
  iVar2_19[0x15].field5_0x6 = 0x7a4;
  iVar2_19[0x15].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xdc) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_20 = (astruct_139 *)(u32)param_2;
  iVar2_20[0x16].field2_0x2 = 0x7a6;
  &iVar2_20[0x16].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xe2) = 0x6;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_21 = (astruct_139 *)(u32)param_2;
  iVar2_21[0x16].field6_0x8 = 0x7b2;
  (iVar2_21 + 0x17) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xe8) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_22 = (astruct_139 *)(u32)param_2;
  &iVar2_22[0x17].field_0x4 = 0x7b4;
  iVar2_22[0x17].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0xee) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_23 = (astruct_139 *)(u32)param_2;
  (iVar2_23 + 0x18) = 0x7ba;
  iVar2_23[0x18].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0xf4) = 0x2d;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_24 = (astruct_139 *)(u32)param_2;
  iVar2_24[0x18].field5_0x6 = 0x814;
  iVar2_24[0x18].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0xfa) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_25 = (astruct_139 *)(u32)param_2;
  iVar2_25[0x19].field2_0x2 = 0x81a;
  &iVar2_25[0x19].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x100) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_26 = (astruct_139 *)(u32)param_2;
  iVar2_26[0x19].field6_0x8 = 0x81c;
  (iVar2_26 + 0x1a) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x106) = 0x4b;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_27 = (astruct_139 *)(u32)param_2;
  &iVar2_27[0x1a].field_0x4 = 0x8b2;
  iVar2_27[0x1a].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x10c) = 0x6;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_28 = (astruct_139 *)(u32)param_2;
  (iVar2_28 + 0x1b) = 0x8be;
  iVar2_28[0x1b].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x112) = 0x4;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_29 = (astruct_139 *)(u32)param_2;
  iVar2_29[0x1c].field2_0x2 = 0x8c6;
  &iVar2_29[0x1c].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x11e) = 0x35;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_30 = (astruct_139 *)(u32)param_2;
  iVar2_30[0x1c].field6_0x8 = 0x930;
  (iVar2_30 + 0x1d) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x124) = 0x2e;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_31 = (astruct_139 *)(u32)param_2;
  iVar2_31[0x1b].field5_0x6 = 0x98c;
  iVar2_31[0x1b].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x118) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_32 = (astruct_139 *)(u32)param_2;
  &iVar2_32[0x1d].field_0x4 = 0x98e;
  iVar2_32[0x1d].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x12a) = 0x9;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_33 = (astruct_139 *)(u32)param_2;
  (iVar2_33 + 0x1e) = 0x9a0;
  iVar2_33[0x1e].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x130) = 0x1a;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar13 = (int)(u32)param_2;
  (iVar13 + 0x132) = 0x9d4;
  (iVar13 + 0x134) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x136) = 0x8;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_34 = (astruct_139 *)(u32)param_2;
  iVar2_34[0x1f].field2_0x2 = 0x9e4;
  &iVar2_34[0x1f].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x13c) = 0x4a;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_35 = (astruct_139 *)(u32)param_2;
  &iVar2_35[0x20].field_0x4 = 0xa78;
  iVar2_35[0x20].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x148) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_36 = (astruct_139 *)(u32)param_2;
  (iVar2_36 + 0x21) = 0xa7c;
  iVar2_36[0x21].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x14e) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_37 = (astruct_139 *)(u32)param_2;
  iVar2_37[0x21].field5_0x6 = 0xa7e;
  iVar2_37[0x21].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x154) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_38 = (astruct_139 *)(u32)param_2;
  iVar2_38[0x22].field2_0x2 = 0xa80;
  &iVar2_38[0x22].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x15a) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_39 = (astruct_139 *)(u32)param_2;
  iVar2_39[0x22].field6_0x8 = 0xa86;
  (iVar2_39 + 0x23) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x160) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_40 = (astruct_139 *)(u32)param_2;
  (iVar2_40 + 0x24) = 0xa8a;
  iVar2_40[0x24].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x16c) = 0x1b;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_41 = (astruct_139 *)(u32)param_2;
  iVar2_41[0x24].field5_0x6 = 0xac0;
  iVar2_41[0x24].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x172) = 0x16;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_42 = (astruct_139 *)(u32)param_2;
  iVar2_42[0x25].field2_0x2 = 0xaec;
  &iVar2_42[0x25].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x178) = 0x3e;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_43 = (astruct_139 *)(u32)param_2;
  iVar2_43[0x25].field6_0x8 = 0xb68;
  (iVar2_43 + 0x26) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x17e) = 0x46;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_44 = (astruct_139 *)(u32)param_2;
  &iVar2_44[0x26].field_0x4 = 0xbf4;
  iVar2_44[0x26].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x184) = 0x1;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_45 = (astruct_139 *)(u32)param_2;
  (iVar2_45 + 0x27) = 0xbf6;
  iVar2_45[0x27].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x18a) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_46 = (astruct_139 *)(u32)param_2;
  iVar2_46[0x27].field5_0x6 = 0xbfc;
  iVar2_46[0x27].field6_0x8 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x190) = 0x3;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_47 = (astruct_139 *)(u32)param_2;
  iVar2_47[0x28].field2_0x2 = 0xc02;
  &iVar2_47[0x28].field_0x4 = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x196) = 0xa;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_48 = (astruct_139 *)(u32)param_2;
  iVar2_48[0x28].field6_0x8 = 0xc16;
  (iVar2_48 + 0x29) = (int)&DAT_1050_1050;
  ((int)(u32)param_2 + 0x19c) = 0x24;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_49 = (astruct_139 *)(u32)param_2;
  &iVar2_49[0x29].field_0x4 = 0xc5e;
  iVar2_49[0x29].field5_0x6 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x1a2) = 0x2;
  uVar14 = ((u32)(u32)param_2 >> 0x10);
  iVar2_50 = (astruct_139 *)(u32)param_2;
  (iVar2_50 + 0x2a) = 0xc62;
  iVar2_50[0x2a].field2_0x2 = &DAT_1050_1050;
  ((int)(u32)param_2 + 0x1a8) = 0x44;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_c626(u32 *param_1)

{
  _u16_1050_06e0 = 0x0;
  fn_ptr_1000_17ce((char *)*param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1008_c646(mut param_1: u16 ,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut uVar4: u16;
  let mut unaff_SI: u16;
  u32 *puVar5;
  u32 *puVar6;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut iStack18: i16;
  let mut iStack16: i16;

  uVar4 = ((u32)in_EDX >> 0x10);
  puVar5 = pass1_1008_c6fa((i16 *)CONCAT22((int)param_2,param_1),(int)(param_2 >> 0x10));
  uVar3 = ((u32)puVar5 >> 0x10);
  puVar6 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar4,uVar3),_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x35),
                           in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  iStack18 = 0x0;
  iStack16 = 0x0;
  while ((piVar1 = (i16 *)((int)puVar5 + 0x4), iVar2 = iStack16, *piVar1 != iStack18 && iStack18 <= *piVar1 &&
         (iVar2 = ((int)*puVar5 + iStack18 * 0x2), (iVar2 * 0x2 + (int)puVar6 + 0xa) == 0x0))) {
    iStack18 += 0x1;
  }
  iStack16 = iVar2;
  return iStack16;
}



BOOL16 pass1_1008_c6ae(mut param_1: u32,mut param_2: i16,mut param_3: i16)

{
  let mut piVar1: *mut i16;
  u32 *puVar2;
  let mut iStack8: i16;

  puVar2 = pass1_1008_c6fa((i16 *)param_1,param_3);
  iStack8 = 0x0;
  while( true ) {
    piVar1 = (i16 *)((int)puVar2 + 0x4);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return 0x0;
    }
    if (((int)*puVar2 + iStack8 * 0x2) == param_2) break;
    iStack8 += 0x1;
  }
  return 0x1;
}



u32 * pass1_1008_c6fa(i16 *param_1,mut param_2: i16)

{
  if ((0x0 < param_2) && (param_2 < 0x47)) {
    return (u32 *)CONCAT22(((int)param_1 + 0x2),param_2 * 0x6 + *param_1);
  }
  return NULL;
}
pub fn pass1_1008_c72a(param_1: *mut astruct_19,mut param_2: u16 ,mut param_3: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  param_1->offset_0x0 = 0xca4a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1008_c75c(param_1: *mut astruct_455)

{
  u32 *puVar1;
  u32 *puVar2;
  code **ppcVar3;
  astruct_455 *iVar4;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xca4a;
  iVar4->field1_0x2 = 0x1008;
  puVar1 = (u32 *)iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_c79a(mut param_1: u32,char *param_2)

{
  char *string_1;
  let mut iVar1: i16;
  astruct_117 *pstruct117_2;
  let mut extraout_DX: u16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut in_stack_0000fe70: u16;
  u8 local_12 [0x4];
  let mut uStack14: u32;
  char string_a [0x8];

  uVar4 = (param_1 >> 0x10);
  pass1_1008_5784((char *)CONCAT22(0x1050,string_a),(u32)((int)param_1 + 0xa));
  while( true ) {
    string_1 = string_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,string_1));
    uStack14 = CONCAT22(extraout_DX,string_1);
    puVar2 = (u8 *)(extraout_DX | string_1);
    if (puVar2 == NULL) break;
    iVar1 = pass1_1000_3d7a(*(char **)(string_1 + 0x4),param_2);
    if (iVar1 == 0x0) {
      puVar5 = pass1_1020_a43e(puVar2,(u16 *)CONCAT22(0x1050,local_12));
      uVar3 = ((u32)puVar5 >> 0x10);
      pass1_1020_a6ee(local_12,uVar3,in_stack_0000fe70,CONCAT22(0x1050,local_12),((int)uStack14 + 0x12)
                     );
      pstruct117_2 = *(astruct_117 **)((int)_PTR_LOOP_1050_65e2 + 0x52);
      pass1_1030_4bbe(uVar3,pstruct117_2,((int)uStack14 + 0x12));
      *(i32 *)((int)param_1 + 0xe) = (long)((int)pstruct117_2 + 0x94) + *_PTR_LOOP_1050_65e2;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_c83a(param_1: *mut astruct_201)

{
  if (*_PTR_LOOP_1050_65e2 <= (u32)((int)param_1 + 0xe)) {
    return;
  }
  return;
}



pub fn pass1_1008_c85e(param_1: *mut astruct_201) -> u32

{
  astruct_201 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_201 *)param_1;
  if (iVar1->field10_0xa == NULL) {
    pass1_1008_c882((astruct_201 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  }
  return CONCAT22(((int)&iVar1->field10_0xa + 0x2),&iVar1->field10_0xa);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_c882(param_1: *mut astruct_201)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  u32 *puVar3;
  code **ppcVar4;
  u32 *puVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar10;
  let mut uVar12: u32;
  astruct_57 *paVar13;
  astruct_201 *iVar9;
  let mut unaff_SI: u16;
  let mut uVar14: u16;
  u8 uVar15;
  u32 *puVar16;
  u32 *puVar17;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffac: u16;
  let mut iStack16: i16;
  astruct_57 *paVar11;

  uVar14 = ((u32)param_1 >> 0x10);
  iVar9 = (astruct_201 *)param_1;
    // WARNING: Load size is inaccurate
  puVar5 = iVar9->field10_0xa;
  uVar9 = ((int)&iVar9->field10_0xa + 0x2);
  paVar10 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar9);
  if ((uVar9 | puVar5) != 0x0) {
    ppcVar4 = (code **)*puVar5;
    puVar5 = (u32 *)(**ppcVar4)();
  }
  mem_op_1000_179c(0xc,paVar10);
  uVar9 = paVar10 | puVar5;
  paVar13 = (astruct_57 *)((u32)paVar10 & 0xffff0000);
  paVar11 = (astruct_57 *)((u32)paVar13 | (u32)uVar9);
  if (uVar9 == 0x0) {
    uVar6 = 0x0;
  }
  else {
    uVar6 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar10,puVar5));
    paVar13 = paVar11;
  }
  &iVar9->field10_0xa = uVar6;
  ((int)&iVar9->field10_0xa + 0x2) = (int)paVar13;
  puVar16 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe7e,
                            in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  uVar12 = (u32)paVar13 & 0xffff0000;
  puVar17 = pass1_1008_c6fa(_u16_1050_06e0,0x44);
  paVar10 = (astruct_57 *)(uVar12 & 0xffff0000 | (u32)puVar17 >> 0x10);
  iStack16 = 0x0;
  while( true ) {
    piVar1 = (i16 *)((int)puVar17 + 0x4);
    if (*piVar1 == iStack16 || *piVar1 < iStack16) break;
    uVar2 = ((int)*puVar17 + iStack16 * 0x2);
    if ((uVar2 * 0x2 + (int)puVar16 + 0xa) != 0x0) {
      uVar7 = pass1_1020_bd80(uVar2);
      uVar8 = str_op_1008_60e8(paVar10,(char *)CONCAT22(paVar10,uVar7));
      uVar6 = SUB42(paVar10,0x0);
      uVar15 = 0x0;
      uVar7 = uVar8;
      mem_op_1000_179c(0x14,paVar10);
      uVar9 = paVar10 | uVar7;
      paVar13 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar9);
      if (uVar9 == 0x0) {
        uVar7 = 0x0;
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000);
      }
      else {
        uVar15 = 0x18;
        struct_1018_47c8((astruct_203 *)CONCAT22(paVar10,uVar7),0x1,CONCAT22(uVar6,uVar8),uVar2,0x0);
        paVar10 = paVar13;
      }
      puVar3 = iVar9->field10_0xa;
      ppcVar4 = (code **)((int)*iVar9->field10_0xa + 0x4);
      (**ppcVar4)(uVar15,(int)puVar3,(int)((u32)puVar3 >> 0x10),uVar7,(int)paVar10);
    }
    iStack16 += 0x1;
  }
  return;
}
pub fn pass1_1008_c98e(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1:bool;
  HFILE16 in_stack_0000ffda;
  u32 local_10 [0x3];

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    local_10[0] = (u32)((int)param_1 + 0xe);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffda);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
  }
  return;
}
pub fn pass1_1008_c9d4(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1:bool;

  if (0x1 < (int)u16_1050_0312) {
    read_file_1008_7cfe((int)param_3,(int)(param_3 >> 0x10),0x15);
    if (param_1 == 0x0) {
      u16_1050_0310 = 0x6d4;
      return;
    }
    BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)(param_2 & 0xffff0000 | (u32)((int)param_2 + 0xe)),0x4);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  return;
}



pub fn pass1_1008_ca24(mut param_1: u32,u8 param_2) -> u32

{
  pass1_1008_c75c((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1008_ca5a(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  (u32)((int)param_1 + 0x16) = 0x0;
  (u32)((int)param_1 + 0x1a) = 0x0;
  (u32)((int)param_1 + 0x1e) = 0x0;
  param_1->offset_0x0 = 0xd71a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1008_caa0(param_1: *mut astruct_455)

{
  astruct_455 *uVar1;

  uVar1 = (astruct_455 *)((u32)param_1 >> 0x10);
  param_1->field0_0x0 = 0xd71a;
  ((int)param_1 + 0x2) = 0x1008;
  pass1_1008_cac6((astruct_455 *)((u32)param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn pass1_1008_cac6(param_1: *mut astruct_455)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  code **ppcVar4;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  puVar1 = (u32 *)iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  (u32)&iVar4[0x1].field1_0x2 = 0x0;
  puVar1 = (u32 *)iVar4[0x1].field3_0x6;
  uVar3 = (iVar4 + 0x2)->field0_0x0;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  (u32)&iVar4[0x1].field3_0x6 = 0x0;
  puVar1 = (u32 *)iVar4[0x2].field1_0x2;
  puVar2 = iVar4[0x2].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  (u32)&iVar4[0x2].field1_0x2 = 0x0;
  puVar1 = (u32 *)iVar4[0x2].field3_0x6;
  uVar3 = (iVar4 + 0x3)->field0_0x0;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  (u32)&iVar4[0x2].field3_0x6 = 0x0;
  puVar1 = (u32 *)iVar4[0x3].field1_0x2;
  puVar2 = iVar4[0x3].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  (u32)&iVar4[0x3].field1_0x2 = 0x0;
  puVar1 = (u32 *)iVar4[0x3].field3_0x6;
  uVar3 = (iVar4 + 0x4)->field0_0x0;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  (u32)&iVar4[0x3].field3_0x6 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_cbc4(param_1: *mut astruct_263,mut param_2: u32)

{
  i32 lVar1;
  i32 lVar2;
  code **ppcVar3;
  let mut bVar4: bool;
  u32 *puVar5;
  let mut uVar6: u16;
  astruct_92 *paVar7;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar13;
  astruct_263 *iVar10;
  astruct_263 *uVar16;
  u8 uVar17;
  char *pcVar18;
  let mut uStack54: u16;
  let mut iStack30: i16;
  astruct_92 local_18;
  let mut uVar8: u32;
  astruct_57 *paVar14;
  astruct_57 *paVar15;

  uVar16 = (astruct_263 *)((u32)param_1 >> 0x10);
  iVar10 = (astruct_263 *)param_1;
  puVar5 = (u32*)&iVar10->field17_0x1e;
  uVar9 = ((int)&iVar10->field17_0x1e + 0x2);
  paVar13 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar9);
  if ((uVar9 | puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar13);
  uVar9 = paVar13 | puVar5;
  paVar15 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
  paVar14 = (astruct_57 *)((u32)paVar15 | (u32)uVar9);
  if (uVar9 == 0x0) {
    puVar5 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar13,puVar5));
    paVar15 = paVar14;
  }
  (u32*)&iVar10->field17_0x1e = puVar5;
  ((int)&iVar10->field17_0x1e + 0x2) = (int)paVar15;
  lVar1 = *(i32 *)((int)param_2 + 0x200);
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x400);
  iStack30 = 0x0;
  while( true ) {
    paVar7 = &local_18;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar7));
    uVar9 = paVar15;
    uVar10 = uVar9 | paVar7;
    paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000);
    paVar15 = (astruct_57 *)((u32)paVar13 | (u32)uVar10);
    if (uVar10 == 0x0) break;
    if (paVar7[0x1c].field4_0x8 == lVar1) {
      iStack30 += 0x1;
    }
  }
  bVar4 = false;
  if (0x1 < iStack30) {
    if (local_18.field6_0x10 == 0x0) {
      paVar15 = (astruct_57 *)((u32)paVar13 | (u32)local_18.field5_0xc);
    }
    else {
      local_18.field5_0xc._0_2_ = 0x1;
      paVar15 = paVar13;
    }
    local_18.field4_0x8 = SUB42(paVar15,0x0);
    local_18.field4_0x8._0_2_ = local_18.field5_0xc;
    while( true ) {
      paVar7 = &local_18;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar7));
      uVar9 = paVar15;
      paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)(uVar9 | paVar7));
      if ((uVar9 | paVar7) == 0x0) break;
      if ((paVar7[0x1c].field4_0x8 == lVar1) && (paVar7->field3_0x4 != 0x4000001)) {
        pcVar18 = pass1_1038_4d28((char *)CONCAT22(uVar9,paVar7));
        paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)pcVar18 >> 0x10);
        uVar6 = str_op_1008_60e8(((u32)pcVar18 >> 0x10),pcVar18);
        uVar8 = (u32)uVar6;
        uVar11 = SUB42(paVar13,0x0);
        uVar17 = 0x0;
        mem_op_1000_179c(0x12,paVar13);
        uVar10 = uVar8;
        uVar12 = paVar13 | uVar10;
        paVar15 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar12);
        if (uVar12 == 0x0) {
          uVar10 = 0x0;
          uStack54 = 0x0;
        }
        else {
          uVar17 = 0x18;
          struct_1018_4920((astruct_203 *)(uVar8 & 0xffff | (long)paVar13 << 0x10),0x1,CONCAT22(uVar11,uVar6),
                           paVar7->field3_0x4);
          uStack54 = SUB42(paVar15,0x0);
        }
        lVar2 = iVar10->field17_0x1e;
        ppcVar3 = (code **)((int)(u32)iVar10->field17_0x1e + 0x4);
        (**ppcVar3)(uVar17,(int)lVar2,(int)((u32)lVar2 >> 0x10),uVar10,uStack54);
        bVar4 = true;
      }
    }
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x43d);
    uVar8 = CONCAT22((int)paVar15,paVar7);
    uVar17 = 0x0;
    mem_op_1000_179c(0x12,paVar15);
    uVar9 = paVar15 | paVar7;
    if (uVar9 == 0x0) {
      paVar7 = NULL;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x18;
      struct_1018_4920((astruct_203 *)CONCAT22(paVar15,paVar7),0x0,uVar8,0x0);
    }
    lVar1 = iVar10->field17_0x1e;
    ppcVar3 = (code **)((int)(u32)iVar10->field17_0x1e + 0x4);
    (**ppcVar3)(uVar17,(int)lVar1,(int)((u32)lVar1 >> 0x10),paVar7,uVar9,uVar8,paVar7,uVar9);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_cda2(param_1: *mut astruct_263,mut param_2: u32)

{
  i32 *plVar1;
  i32 lVar2;
  code **ppcVar3;
  u32 *puVar4;
  let mut uVar5: u16;
  char *pcVar6;
  astruct_206 *puVar9;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar10;
  let mut uVar11: u32;
  astruct_263 *iVar15;
  let mut uVar12: u16;
  let mut uVar13: u16;
  u8 uVar14;
  astruct_203 *paVar15;
  u8 local_2e [0xa];
  let mut uStack36: u16;
  let mut uStack34: u32;
  let mut uStack30: u32;
  let mut uStack26: u32;
  u32 *puStack18;
  let mut uStack16: u16;
  astruct_203 *paStack14;
  let mut uStack10: u16;
  let mut uStack8: u32;
  let mut iStack4: i16;

  uVar12 = ((u32)param_1 >> 0x10);
  iVar15 = (astruct_263 *)param_1;
  puVar4 = (u32*)&iVar15->field16_0x1a;
  uStack16 = ((int)&iVar15->field16_0x1a + 0x2);
  paVar10 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack16);
  paStack14 = (astruct_203 *)CONCAT22(uStack16,puVar4);
  puStack18 = puVar4;
  if ((uStack16 | puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar10);
  uStack16 = paVar10;
  uVar7 = (u32)paVar10 & 0xffff0000;
  uVar11 = uVar7 | (uStack16 | puVar4);
  puStack18 = puVar4;
  if ((uStack16 | puVar4) == 0x0) {
    puVar4 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(uStack16,puVar4));
    uVar7 = uVar11;
  }
  (u32*)&iVar15->field16_0x1a = puVar4;
  ((int)&iVar15->field16_0x1a + 0x2) = (int)uVar7;
  iStack4 = 0x0;
  uVar13 = (param_2 >> 0x10);
  uStack8 = (u32)((int)param_2 + 0x210);
  uVar5 = ((int)param_2 + 0x212);
  paVar10 = (astruct_57 *)(uVar7 & 0xffff0000 | (u32)uVar5);
  uVar5 |= uStack8;
  uVar7 = (u32)uVar5;
  if (uVar5 != 0x0) {
    uStack26 = (u32)(uStack8 + 0xa);
    uStack30 = 0x0;
    while( true ) {
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | uStack26 >> 0x10);
      uVar7 = uStack26;
      if (uStack26 <= uStack30) break;
      bad_1030_1312();
      uStack34 = uVar7 & 0xffff | (long)paVar10 << 0x10;
      uVar5 = paVar10 | uVar7;
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar5);
      if (uVar5 != 0x0) {
        for (uStack36 = 0x1; (int)uStack36 < 0x15; uStack36 += 0x1) {
          local_2e._8_2_ = pass1_1030_ce2e((int)uStack34,(uStack34 >> 0x10),uStack36);
          if (local_2e._8_2_ != 0x0) {
            pass1_1008_5784((char *)CONCAT22(0x1050,local_2e),iVar15->field16_0x1a);
            do {
              puVar9 = (astruct_206 *)local_2e;
              pass1_1008_5b12((char *)CONCAT22(0x1050,puVar9));
              uVar5 = paVar10;
              paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)(uVar5 | puVar9));
              if ((uVar5 | puVar9) == 0x0) break;
            } while (puVar9->field8_0xe != uStack36);
            if (CONCAT22(uVar5,puVar9) == 0x0) {
              pcVar6 = string_op_1020_c222(uStack36);
              uVar5 = str_op_1008_60e8(paVar10,(char *)CONCAT22(paVar10,pcVar6));
              uVar7 = CONCAT22((int)paVar10,uVar5);
              uVar14 = 0x0;
              mem_op_1000_179c(0x10,paVar10);
              uVar8 = paVar10;
              paStack14 = (astruct_203 *)CONCAT22(uVar8,uVar5);
              paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000);
              if ((uVar8 | uVar5) == 0x0) {
                uVar13 = 0x0;
              }
              else {
                uVar14 = 0x18;
                paVar15 = struct_1018_48b0(paStack14,
                                           CONCAT22((int)local_2e._8_2_ >> 0xf,
                                                    local_2e._8_2_ & 0xff |
                                                    (u8)((u32)(long)(int)local_2e._8_2_ >> 0x8) << 0x8),uVar7,
                                           uStack36);
                paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)paVar15 >> 0x10);
                uVar13 = SUB42(paVar15,0x0);
              }
              lVar2 = iVar15->field16_0x1a;
              ppcVar3 = (code **)((int)(u32)iVar15->field16_0x1a + 0x4);
              (**ppcVar3)(uVar14,(int)lVar2,(int)((u32)lVar2 >> 0x10),uVar13,(int)paVar10);
            }
            else {
              plVar1 = &puVar9->field5_0x8;
              *plVar1 = *plVar1 + (long)(int)local_2e._8_2_;
            }
            iStack4 = 0x1;
          }
        }
      }
      uStack30 += 0x1;
    }
  }
  uVar5 = uVar7;
  uStack10 = 0x0;
  if (iStack4 == 0x0) {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x43f);
    uVar7 = CONCAT22((int)paVar10,uVar5);
    uVar14 = 0x0;
    mem_op_1000_179c(0x10,paVar10);
    uStack16 = paVar10;
    puStack18 = (u32 *)uVar5;
    if ((uStack16 | uVar5) == 0x0) {
      uVar13 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar14 = 0x18;
      paVar15 = struct_1018_48b0((astruct_203 *)CONCAT22(uStack16,uVar5),0x0,uVar7,0x0);
      uVar9 = ((u32)paVar15 >> 0x10);
      uVar13 = SUB42(paVar15,0x0);
    }
    lVar2 = iVar15->field16_0x1a;
    ppcVar3 = (code **)((int)(u32)iVar15->field16_0x1a + 0x4);
    (**ppcVar3)(uVar14,(int)lVar2,(int)((u32)lVar2 >> 0x10),uVar13,uVar9,uVar7,uVar13,uVar9);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_cfa0(param_1: *mut astruct_263,mut param_2: u32)

{
  let mut uVar1: u32;
  u32 *puVar2;
  code **ppcVar3;
  let mut bVar4: bool;
  u32 *puVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar14;
  astruct_57 *paVar16;
  astruct_4 *iVar15;
  let mut uVar17: u16;
  u8 uVar18;
  astruct_203 *paVar19;
  astruct_57 *paVar15;

  uVar17 = ((u32)param_1 >> 0x10);
  iVar15 = (astruct_4 *)param_1;
    // WARNING: Load size is inaccurate
  puVar5 = iVar15->field22_0x16;
  uVar10 = ((int)&iVar15->field22_0x16 + 0x2);
  paVar14 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar10);
  if ((uVar10 | puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar14);
  uVar10 = paVar14 | puVar5;
  paVar16 = (astruct_57 *)((u32)paVar14 & 0xffff0000);
  paVar15 = (astruct_57 *)((u32)paVar16 | (u32)uVar10);
  if (uVar10 == 0x0) {
    puVar5 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar14,puVar5));
    paVar16 = paVar15;
  }
  (u32*)&iVar15->field22_0x16 = puVar5;
  ((int)&iVar15->field22_0x16 + 0x2) = (int)paVar16;
  bVar4 = false;
  uVar1 = (u32)((int)param_2 + 0x1f6);
  uVar9 = uVar1;
  pass1_1030_38f2(uVar1,0x2);
  uVar10 = uVar9;
  if ((-0x1 < (int)paVar16) && ((0x0 < (int)paVar16 || (uVar10 != 0x0)))) {
    paVar15 = paVar16;
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x430);
    uVar13 = SUB42(paVar15,0x0);
    uVar18 = 0x0;
    uVar6 = uVar10;
    mem_op_1000_179c(0x14,paVar15);
    paVar14 = (astruct_57 *)((u32)paVar15 & 0xffff0000);
    if ((paVar15 | uVar6) == 0x0) {
      uVar10 = 0x0;
      paVar16 = paVar14;
    }
    else {
      uVar18 = 0x18;
      paVar19 = struct_1018_4842((astruct_203 *)CONCAT22(paVar15,uVar6),uVar9 & 0xffff | (long)paVar16 << 0x10,
                                 CONCAT22(uVar13,uVar10),0x2);
      uVar10 = paVar19;
      paVar16 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
    }
    puVar2 = iVar15->field22_0x16;
    ppcVar3 = (code **)((int)*iVar15->field22_0x16 + 0x4);
    (**ppcVar3)(uVar18,(int)puVar2,(int)((u32)puVar2 >> 0x10),uVar10,(int)paVar16);
    bVar4 = true;
  }
  pass1_1030_38f2(uVar1,0x3);
  iVar11 = (int)paVar16;
  if ((-0x1 < iVar11) && ((0x0 < iVar11 || (uVar10 != 0x0)))) {
    uVar7 = uVar10;
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x431);
    uVar13 = SUB42(paVar16,0x0);
    uVar18 = 0x0;
    uVar6 = uVar7;
    mem_op_1000_179c(0x14,paVar16);
    uVar12 = paVar16;
    paVar16 = (astruct_57 *)((u32)paVar16 & 0xffff0000);
    if ((uVar12 | uVar6) == 0x0) {
      uVar10 = 0x0;
    }
    else {
      uVar18 = 0x18;
      paVar19 = struct_1018_4842((astruct_203 *)CONCAT22(uVar12,uVar6),CONCAT22(iVar11,uVar10),CONCAT22(uVar13,uVar7),
                                 0x3);
      paVar16 = (astruct_57 *)((u32)paVar16 & 0xffff0000 | (u32)paVar19 >> 0x10);
      uVar10 = paVar19;
    }
    puVar2 = iVar15->field22_0x16;
    ppcVar3 = (code **)((int)*iVar15->field22_0x16 + 0x4);
    (**ppcVar3)(uVar18,(int)puVar2,(int)((u32)puVar2 >> 0x10),uVar10,(int)paVar16);
    bVar4 = true;
  }
  pass1_1030_38f2(uVar1,0x4);
  iVar11 = (int)paVar16;
  if ((-0x1 < iVar11) && ((0x0 < iVar11 || (uVar10 != 0x0)))) {
    uVar7 = uVar10;
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x432);
    uVar13 = SUB42(paVar16,0x0);
    uVar18 = 0x0;
    uVar6 = uVar7;
    mem_op_1000_179c(0x14,paVar16);
    uVar12 = paVar16;
    paVar16 = (astruct_57 *)((u32)paVar16 & 0xffff0000);
    if ((uVar12 | uVar6) == 0x0) {
      uVar10 = 0x0;
    }
    else {
      uVar18 = 0x18;
      paVar19 = struct_1018_4842((astruct_203 *)CONCAT22(uVar12,uVar6),CONCAT22(iVar11,uVar10),CONCAT22(uVar13,uVar7),
                                 0x4);
      paVar16 = (astruct_57 *)((u32)paVar16 & 0xffff0000 | (u32)paVar19 >> 0x10);
      uVar10 = paVar19;
    }
    puVar2 = iVar15->field22_0x16;
    ppcVar3 = (code **)((int)*iVar15->field22_0x16 + 0x4);
    (**ppcVar3)(uVar18,(int)puVar2,(int)((u32)puVar2 >> 0x10),uVar10,(int)paVar16);
    bVar4 = true;
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x440);
    uVar13 = SUB42(paVar16,0x0);
    uVar18 = 0x0;
    uVar6 = uVar10;
    mem_op_1000_179c(0x14,paVar16);
    if ((paVar16 | uVar6) == 0x0) {
      uVar8 = 0x0;
      uVar13 = 0x0;
    }
    else {
      uVar18 = 0x18;
      paVar19 = struct_1018_4842((astruct_203 *)CONCAT22(paVar16,uVar6),0x0,CONCAT22(uVar13,uVar10),0x0);
      uVar13 = ((u32)paVar19 >> 0x10);
      uVar8 = SUB42(paVar19,0x0);
    }
    puVar2 = iVar15->field22_0x16;
    ppcVar3 = (code **)((int)*iVar15->field22_0x16 + 0x4);
    (**ppcVar3)(uVar18,(int)puVar2,(int)((u32)puVar2 >> 0x10),uVar8,uVar13);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_str_op_1008_d1c6(param_1: *mut astruct_263,mut param_2: u32)

{
  let mut iVar1: i16;
  u32 *puVar2;
  code **ppcVar3;
  let mut bVar4: bool;
  u32 *puVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar14;
  astruct_57 *paVar15;
  astruct_57 *paVar17;
  astruct_57 *paVar18;
  astruct_5 *iVar15;
  let mut uVar19: u16;
  let mut uVar20: u16;
  u32 *puVar21;
  let mut uVar22: u32;
  u8 uVar23;
  let mut uStack52: u16;
  let mut uStack20: u32;
  let mut uStack14: u32;
  u32 *puStack10;
  let mut uVar16: u32;

  uVar19 = ((u32)param_1 >> 0x10);
  iVar15 = (astruct_5 *)param_1;
    // WARNING: Load size is inaccurate
  puVar5 = iVar15->field18_0x12;
  uVar9 = ((int)&iVar15->field18_0x12 + 0x2);
  paVar14 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar9);
  if ((uVar9 | puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar14);
  uVar9 = paVar14 | puVar5;
  uVar16 = (u32)paVar14 & 0xffff0000;
  uVar22 = uVar16 | uVar9;
  if (uVar9 == 0x0) {
    puVar5 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar14,puVar5));
    uVar16 = uVar22;
  }
  (u32*)&iVar15->field18_0x12 = puVar5;
  ((int)&iVar15->field18_0x12 + 0x2) = (int)uVar16;
  puVar21 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
  paVar15 = (astruct_57 *)(uVar16 & 0xffff0000 | (u32)puVar21 >> 0x10);
  uVar9 = puVar21;
  uVar20 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4e78(uVar9,paVar15,param_2,puVar21);
  uVar10 = paVar15;
  puStack10 = (u32 *)CONCAT22(uVar10,uVar9);
  ppcVar3 = (code **)((int)*puStack10 + 0x10);
  paVar14 = paVar15;
  uVar6 = uVar9;
  (**ppcVar3)((int)&u16_1050_1038,uVar9,uVar10);
  uStack14 = CONCAT22((int)paVar14,uVar6);
  bVar4 = false;
  for (uStack20 = 0x0; uStack20 < uStack14; uStack20 += 0x1) {
    uVar20 = 0x1030;
    uVar22 = pass1_1030_1d7c(uVar6,(int)paVar14,(u32)puStack10);
    uVar16 = (u32)paVar14 & 0xffff0000;
    uVar13 = uVar22;
    uVar11 = (uVar22 >> 0x10);
    paVar14 = (astruct_57 *)(uVar16 | (uVar11 | uVar13));
    if ((((uVar11 | uVar13) != 0x0) && (*(i32 *)(uVar13 + 0x1c) != 0x8000002)) &&
       ((iVar1 = (uVar13 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      uVar22 = (u32)(uVar13 + 0x6) & 0xffff00ff;
      paVar17 = (astruct_57 *)(uVar16 | uVar22);
      uVar20 = uVar22;
      uVar23 = (u8)(uVar13 + 0x4);
      uVar7 = pass1_1020_bd80((uVar13 + 0xc));
      wsprintf16(&iVar15->field31_0x22,(char *)CONCAT13(0xc,CONCAT12(0xea,uVar19)),0xea,0x50,CONCAT22(uVar7,0x1050),
                 (char)paVar17,uVar23,uVar20);
      uVar8 = str_op_1008_60e8(paVar17,(char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar15->field31_0x22)));
      uVar20 = 0x1000;
      paVar18 = paVar17;
      uVar7 = uVar8;
      mem_op_1000_179c(0x12,paVar17);
      uVar12 = paVar18 | uVar7;
      paVar14 = (astruct_57 *)((u32)paVar18 & 0xffff0000 | (u32)uVar12);
      if (uVar12 == 0x0) {
        uVar23 = 0x0;
        uStack52 = 0x0;
      }
      else {
        uVar20 = 0x1018;
        pass1_1018_4808((astruct_203 *)CONCAT22(paVar18,uVar7),0x1,
                        CONCAT13((char)((u32)paVar17 >> 0x8),CONCAT12((char)paVar17,uVar8)),(u32)(uVar13 + 0x4));
        uVar23 = (u8)uVar7;
        uStack52 = SUB42(paVar14,0x0);
      }
      puVar2 = iVar15->field18_0x12;
      ppcVar3 = (code **)((int)*iVar15->field18_0x12 + 0x4);
      (**ppcVar3)((char)uVar20,(int)puVar2,(char)((u32)puVar2 >> 0x10),uVar23,uStack52);
      bVar4 = true;
    }
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x441);
    uVar20 = 0x1000;
    paVar18 = paVar14;
    uVar6 = uStack14;
    mem_op_1000_179c(0x12,paVar14);
    uVar13 = paVar18 | uVar6;
    if (uVar13 == 0x0) {
      uVar23 = 0x0;
      uVar13 = 0x0;
    }
    else {
      uVar20 = 0x1018;
      pass1_1018_4808((astruct_203 *)CONCAT22(paVar18,uVar6),0x0,
                      CONCAT13((char)((u32)paVar14 >> 0x8),CONCAT12((char)paVar14,uStack14)),0x0);
      uVar23 = (u8)uVar6;
    }
    puVar2 = iVar15->field18_0x12;
    ppcVar3 = (code **)((int)*iVar15->field18_0x12 + 0x4);
    (**ppcVar3)((char)uVar20,(int)puVar2,(char)((u32)puVar2 >> 0x10),uVar23,uVar13);
  }
  if ((uVar10 | uVar9) != 0x0) {
    ppcVar3 = (code **)*puStack10;
    (**ppcVar3)(uVar20,(char)uVar9,(char)paVar15,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_d3ae(param_1: *mut astruct_263)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut bVar3: bool;
  u32 *puVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar11;
  astruct_57 *paVar13;
  astruct_263 *iVar13;
  let mut uVar14: u16;
  u8 uVar15;
  let mut uStack6: u16;
  astruct_57 *paVar12;

  uVar14 = ((u32)param_1 >> 0x10);
  iVar13 = (astruct_263 *)param_1;
  puVar4 = (u32 *)iVar13->field8_0xa;
  uVar9 = iVar13->field9_0xc;
  paVar11 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar9);
  if ((uVar9 | puVar4) != 0x0) {
    ppcVar2 = (code **)*puVar4;
    puVar4 = (u32 *)(**ppcVar2)();
  }
  mem_op_1000_179c(0xc,paVar11);
  uVar9 = paVar11 | puVar4;
  paVar13 = (astruct_57 *)((u32)paVar11 & 0xffff0000);
  paVar12 = (astruct_57 *)((u32)paVar13 | (u32)uVar9);
  if (uVar9 == 0x0) {
    uVar9 = 0x0;
  }
  else {
    uVar9 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar11,puVar4));
    paVar13 = paVar12;
  }
  iVar13->field8_0xa = uVar9;
  iVar13->field9_0xc = paVar13;
  bVar3 = false;
  for (uStack6 = 0x21; 0x10 < (int)uStack6; uStack6 -= 0x1) {
    empty_1038_540a();
    uVar10 = paVar13;
    uVar5 = uVar10 | uVar9;
    if (uVar5 != 0x0) {
      bVar3 = true;
      string_1020_c0ca(uStack6);
      uVar6 = str_op_1008_60e8(paVar13,(char *)CONCAT22(paVar13,uVar5));
      uVar8 = SUB42(paVar13,0x0);
      uVar15 = 0x0;
      uVar7 = uVar6;
      mem_op_1000_179c(0x10,paVar13);
      uVar5 = paVar13 | uVar7;
      paVar11 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar5);
      if (uVar5 == 0x0) {
        uVar8 = 0x0;
        paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
      }
      else {
        uVar15 = 0x18;
        uVar8 = struct_1018_4790(CONCAT22(paVar13,uVar7),CONCAT22(uVar10,uVar9),CONCAT22(uVar8,uVar6),uStack6);
        paVar13 = paVar11;
      }
      uVar1 = (u32)&iVar13->field8_0xa;
      ppcVar2 = (code **)((int)*(u32*)&iVar13->field8_0xa + 0x4);
      uVar5 = (**ppcVar2)(uVar15,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar8,(int)paVar13);
    }
    uVar9 = uVar5;
  }
  if (!bVar3) {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x43e);
    uVar8 = SUB42(paVar13,0x0);
    uVar15 = 0x0;
    uVar5 = uVar9;
    mem_op_1000_179c(0x10,paVar13);
    uVar10 = paVar13 | uVar5;
    if (uVar10 == 0x0) {
      uVar8 = 0x0;
      uVar10 = 0x0;
    }
    else {
      uVar15 = 0x18;
      uVar8 = struct_1018_4790(CONCAT22(paVar13,uVar5),0x0,CONCAT22(uVar8,uVar9),0x0);
    }
    uVar1 = (u32)&iVar13->field8_0xa;
    ppcVar2 = (code **)((int)*(u32*)&iVar13->field8_0xa + 0x4);
    (**ppcVar2)(uVar15,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar8,uVar10);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_str_op_1008_d4f6(param_1: *mut astruct_263,param_2: *mut astruct_6)

{
  let mut iVar1: i16;
  i32 lVar2;
  u32 *puVar3;
  let mut uVar4: u32;
  let mut bVar5: bool;
  u32 *puVar6;
  let mut BVar7:bool;
  let mut uVar8: u16;
  let mut uVar9: u16;
  u32 *puVar10;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar15;
  let mut uVar16: u32;
  astruct_57 *paVar18;
  astruct_6 *iVar18;
  StructD *structd_v20;
  let mut uVar19: u16;
  let mut structd_v20_hi: u16;
  let mut uVar20: u16;
  let mut uVar21: u32;
  u8 uVar22;
  let mut uStack58: u16;
  let mut uStack20: u32;
  let mut uStack12: u16;
  let mut uVar17: u32;
  code **fn_ptr_v5;

  uVar19 = ((u32)param_2 >> 0x10);
  iVar18 = (astruct_6 *)param_2;
  lVar2 = iVar18->field509_0x200;
  structd_v20_hi = ((u32)param_1 >> 0x10);
  structd_v20 = (StructD *)param_1;
  puVar6 = (u32 *)structd_v20->field8_0xe;
  uVar12 = &structd_v20->field_0x10;
  paVar15 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar12);
  if ((uVar12 | puVar6) != 0x0) {
    fn_ptr_v5 = (code **)*puVar6;
    (**fn_ptr_v5)();
  }
  mem_op_1000_179c(0xc,paVar15);
  uVar12 = paVar15 | puVar6;
  uVar17 = (u32)paVar15 & 0xffff0000;
  uVar11 = uVar17 | uVar12;
  if (uVar12 == 0x0) {
    puVar6 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar15,puVar6));
    uVar17 = uVar11;
  }
  structd_v20->field8_0xe = puVar6;
  &structd_v20->field_0x10 = (int)uVar17;
  puVar3 = iVar18->field12_0xc;
  uVar12 = ((int)&iVar18->field12_0xc + 0x2);
  uVar16 = uVar17 & 0xffff0000 | (u32)uVar12;
  fn_ptr_v5 = (code **)((int)*puVar3 + 0x10);
  puVar10 = puVar3;
  (**fn_ptr_v5)(0x1000,(int)puVar3,uVar12);
  uVar11 = (u32)puVar10 & 0xffff | uVar16 << 0x10;
  bVar5 = false;
  uStack20 = 0x0;
  uVar17 = uVar16;
  while( true ) {
    uStack12 = uVar16;
    paVar15 = (astruct_57 *)(uVar17 & 0xffff0000 | uVar16 & 0xffff);
    if (uVar11 <= uStack20) break;
    uVar21 = pass1_1030_1d7c((int)((u32)puVar10 & 0xffff),uStack12,(u32)puVar3);
    uVar17 = (u32)paVar15 & 0xffff0000;
    uVar12 = uVar21;
    uVar14 = (uVar21 >> 0x10);
    if ((((uVar14 | uVar12) != 0x0) && (*(i32 *)(uVar12 + 0x1c) != 0x8000002)) &&
       ((iVar1 = (uVar12 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      uVar8 = (uVar12 + 0xc);
      BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar8,0x34);
      if ((BVar7 == 0x0) && (*(i32 *)(uVar12 + 0x1c) != lVar2)) {
        uVar21 = (u32)(uVar12 + 0x6) & 0xffff00ff;
        uVar17 = uVar17 & 0xffff0000 | uVar21;
        uVar19 = uVar21;
        uVar22 = (u8)(uVar12 + 0x4);
        uVar8 = pass1_1020_bd80(uVar8);
        paVar15 = (astruct_57 *)(uVar17 & 0xffff0000 | (u32)structd_v20_hi);
        wsprintf16(&structd_v20->field20_0x22,(char *)CONCAT22(0xcf4,structd_v20_hi),(char *)CONCAT22(uVar8,0x1050),
                   (char)uVar17,uVar22,uVar19);
        uVar9 = str_op_1008_60e8(paVar15,
                                 (char *)((u32)param_1 & 0xffff0000 | ZEXT24(&structd_v20->field20_0x22)));
        uVar19 = SUB42(paVar15,0x0);
        uVar20 = 0x1000;
        uVar8 = uVar9;
        mem_op_1000_179c(0x14,paVar15);
        uVar13 = paVar15 | uVar8;
        uVar17 = (u32)paVar15 & 0xffff0000 | (u32)uVar13;
        if (uVar13 == 0x0) {
          uVar22 = 0x0;
          uStack58 = 0x0;
        }
        else {
          uVar20 = 0x1018;
          struct_1018_47c8((astruct_203 *)CONCAT22(paVar15,uVar8),0x1,CONCAT22(uVar19,uVar9),
                           (uVar12 + 0xc),(u32)(uVar12 + 0x4));
          uVar22 = (u8)uVar8;
          uStack58 = uVar17;
        }
        uVar4 = (u32)&structd_v20->field8_0xe;
        fn_ptr_v5 = (code **)((int)*(u32*)&structd_v20->field8_0xe + 0x4);
        (**fn_ptr_v5)(uVar20,(int)uVar4,(char)((u32)uVar4 >> 0x10),uVar22,uStack58);
        bVar5 = true;
      }
    }
    uStack20 += 0x1;
  }
  if (!bVar5) {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x442);
    uVar12 = uVar11;
    uVar19 = 0x1000;
    paVar18 = paVar15;
    mem_op_1000_179c(0x14,paVar15);
    uVar14 = paVar18 | uVar12;
    if (uVar14 == 0x0) {
      uVar22 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar19 = 0x1018;
      struct_1018_47c8((astruct_203 *)CONCAT22(paVar18,uVar12),0x0,uVar11 & 0xffff | (long)paVar15 << 0x10,0x0,0x0
                      );
      uVar22 = (u8)uVar12;
    }
    uVar4 = (u32)&structd_v20->field8_0xe;
    fn_ptr_v5 = (code **)((int)*(u32*)&structd_v20->field8_0xe + 0x4);
    (**fn_ptr_v5)(uVar19,(int)uVar4,(char)((u32)uVar4 >> 0x10),uVar22,uVar14);
  }
  return;
}



u16 * pass1_1008_d6f4(param_1: *mut u16,u8 param_2)

{
  pass1_1008_caa0((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_d72e(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  param_1->offset_0x0 = 0xd780;
  ((int)param_1 + 0x2) = 0x1008;
  return &param_1->offset_0x0;
}



u16 * pass1_1008_d75a(param_1: *mut u16,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_d790(param_1: *mut astruct_19,param_2: *mut astruct_19,mut param_3: u16 )

{
  INT16 IVar1;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
  astruct_19 *paVar4;

  uVar3 = ((u32)in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_2,param_1),param_3);
  uVar2 = CONCAT22(uVar3,(int)((u32)paVar4 >> 0x10));
  (u32)&param_1->horiz_res_0xa = 0x0;
  &param_1->field_0xe = 0x0;
  CONCAT22(param_2,param_1) = 0xd98e;
  param_1->segment_0x2 = 0x1008;
  IVar1 = FUN_1010_830a(0x0,uVar2,0x1010,_u16_1050_14cc,0x9b);
  param_1->horiz_res_0xa = IVar1;
  param_1->ver_res_0xc = uVar2;
  return;
}
pub fn pass1_1008_d7da(param_1: *mut u16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0xd98e;
  (iVar4 + 0x2) = 0x1008;
  puVar1 = (u32 *)(iVar4 + 0xa);
  uVar2 = (iVar4 + 0xc);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn pass1_1008_d818(param_1: *mut astruct_263,mut param_2: i16)

{
  astruct_263 *iVar1;
  let mut uVar1: u16;

  if (param_2 - 0x1a0U < 0x15) {
    iVar1 = (astruct_263 *)param_1;
    uVar1 = ((u32)param_1 >> 0x10);
    switch(param_2) {
    case 0x1a0:
      iVar1->field10_0xe = 0x14;
      break;
    case 0x1a1:
      iVar1->field10_0xe = 0x3;
      break;
    case 0x1a2:
      iVar1->field10_0xe = 0x2;
      break;
    case 0x1a3:
      iVar1->field10_0xe = 0xe;
      break;
    case 0x1a4:
      iVar1->field10_0xe = 0xc;
      break;
    case 0x1a5:
      iVar1->field10_0xe = 0x4;
      break;
    case 0x1a6:
      iVar1->field10_0xe = 0xb;
      break;
    case 0x1a7:
      iVar1->field10_0xe = 0x6;
      break;
    case 0x1a8:
      iVar1->field10_0xe = 0xa;
      break;
    case 0x1a9:
      iVar1->field10_0xe = 0xd;
      break;
    case 0x1aa:
      iVar1->field10_0xe = 0x13;
      break;
    case 0x1ab:
      iVar1->field10_0xe = 0x5;
      break;
    case 0x1ac:
      iVar1->field10_0xe = 0x9;
      break;
    case 0x1ad:
      iVar1->field10_0xe = 0x8;
      break;
    case 0x1ae:
      iVar1->field10_0xe = 0x12;
      break;
    case 0x1af:
      iVar1->field10_0xe = 0x11;
      break;
    case 0x1b0:
      iVar1->field10_0xe = 0x7;
      return;
    case 0x1b1:
      iVar1->field10_0xe = 0x10;
      return;
    case 0x1b2:
      iVar1->field10_0xe = 0x1;
      return;
    case 0x1b3:
      iVar1->field10_0xe = 0xf;
      return;
    case 0x1b4:
      iVar1->field10_0xe = 0x15;
      return;
    }
  }
  return;
}



StructD * pass1_1008_d968(StructD *param_1,u8 param_2)

{
  pass1_1008_d7da(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_d99e(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 )

{
  struct_op_1018_4cda(param_2,param_3);
  param_2->offset_0x0 = 0xd9fa;
  ((int)param_2 + 0x2) = 0x1008;
  pass1_1018_4dce(param_1,param_2,0x9a);
  _PTR_LOOP_1050_4230 = param_2;
  return;
}



StructD * pass1_1008_d9d4(StructD *param_1,u8 param_2)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1008_da12(param_1: *mut astruct_19,mut param_2: u16 )

{
  u8 bVar1;
  HDC16 hdc;
  let mut horiz_res: i16;
  let mut vert_res: i16;
  let mut iVar2: i16;
  let mut raster_caps: u16;
  let mut sz_palette: i16;
  let mut num_reserved: i16;
  PALETTEENTRY *entries;
  let mut uVar4: u16;
  let mut start: u16;
  u8 *count;
  let mut count_00: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut puStack32: *mut u16;
  let mut iStack16: i16;
  i32 lStack8;
  let mut uVar3: u32;
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u16;
  StructD *pSVar2;

  uVar5 = ((u32)in_EDX >> 0x10);
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  ((int)param_1 + 0xc) = 0x0;
  pass1_1008_3e38((astruct_19 *)
                  ((u32)param_1 & 0xff000000 | (u32)CONCAT12((char)((u32)param_1 >> 0x10),(int)param_1 + 0xeU)));
  ((int)param_1 + 0x14) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  (u32)((int)param_1 + 0x18) = 0x0;
  param_1->offset_0x0 = 0xdc80;
  ((int)param_1 + 0x2) = 0x1008;
  hdc = GetDC16(0x0);
  horiz_res = GetDeviceCaps16(HORZRES,hdc);
  ((int)param_1 + 0xa) = horiz_res;
  vert_res = GetDeviceCaps16(VERTRES,hdc);
  ((int)param_1 + 0xc) = vert_res;
  iVar2 = ((int)param_1 + 0xc) + -0x1e0;
  count = (u8 *)(iVar2 >> 0xf);
  pSVar2 = (StructD *)CONCAT22(uVar5,count);
  pass1_1008_3e76((u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0xeU)),0x0,iVar2 / 0x2,
                  (((int)param_1 + 0xa) + -0x280) / 0x2);
  raster_caps = GetDeviceCaps16(RASTERCAPS,hdc);
  if ((raster_caps & 0x100) != 0x0) {
    sz_palette = GetDeviceCaps16(SIZEPALETTE,hdc);
    ((int)param_1 + 0x14) = sz_palette;
    num_reserved = GetDeviceCaps16(NUMRESERVED,hdc);
    ((int)param_1 + 0x16) = num_reserved;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
    }
    else {
      pSVar2 = (StructD *)((u32)pSVar2 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    entries = (PALETTEENTRY *)
              fn_ptr_op_1000_1708((num_reserved + 0x1) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar2);
    count_00 = pSVar2;
    lStack8 = CONCAT22(count_00,entries);
    iVar6 = ((int)param_1 + 0x16);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar2;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708((iVar6 + 0x1) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    ((int)param_1 + 0x18) = uVar4;
    ((int)param_1 + 0x1a) = PTR_LOOP_1050_5f2e;
    if (lStack8 != 0x0) {
      if (*(i32 *)((int)param_1 + 0x18) != 0x0) {
        start = ((int)param_1 + 0x16) / 0x2;
        GetSystemPaletteEntries(entries,count_00,start,0x0);
        GetSystemPaletteEntries(entries + start,count_00,start,((int)param_1 + 0x14) - start);
        puStack32 = (u16*)((int)param_1 + 0x18);
        for (iStack16 = 0x0; puVar2 = puStack32, piVar1 = (i16 *)((int)param_1 + 0x16),
            *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1) {
          bVar1 = (entries + iStack16)->pe_red;
          uVar3 = (u32)puStack32 >> 0x10;
          iVar6 = (int)puStack32;
          puStack32 = (u16 *)((u32)puStack32 & 0xffff0000 | (u32)(iVar6 + 0x4));
          *puVar2 = CONCAT11(entries[iStack16].pe_green,entries[iStack16].pe_blue);
          (iVar6 + 0x2) = bVar1;
        }
      }
    }
    fn_ptr_1000_17ce((char *)CONCAT22(count_00,entries));
  }
  ReleaseDC16(hdc,0x0);
  return;
}
pub fn pass1_1008_dc2c(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0xdc80;
  ((int)param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x18));
  pass1_1010_1d80((StructD *)param_1);
  return;
}



StructD * pass1_1008_dc5a(StructD *param_1,u8 param_2)

{
  pass1_1008_dc2c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Variable defined which should be unmapped: param_9
pub fn pass1_1008_dc80(mut param_1: u16 ,mut param_2: i16,u8 param_3,uchar param_4,mut param_5: u16 ,param_6: *mut u16,mut param_7: u32,
                    mut param_8: u32,mut param_9: u16 )

{
  char *pcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  code *pcVar4;
  let mut uVar5: u16;
  char cVar6;
  char extraout_DL;
  u8 bVar7;
  let mut iVar8: i16;
  let mut unaff_SI: i16;
  let mut uVar9: u16;
  u8 bVar10;

  bVar7 = (u8)(param_9 >> 0x8);
  bVar10 = (u8)param_9 + bVar7;
  cVar6 = bVar10 + param_3;
  uVar2 = (CARRY1((u8)param_9,bVar7) || CARRY1(bVar10,param_3));
  uVar3 = param_1 + 0xeff0;
  bVar10 = param_1 < 0x1010 || uVar3 < uVar2;
  uVar5 = uVar3 - uVar2;
  pcVar4 = (code *)swi(0x4);
  if (SBORROW2(param_1,0x1010) != SBORROW2(uVar3,uVar2)) {
    (*pcVar4)();
    cVar6 = extraout_DL;
  }
  pcVar1 = (char *)(param_2 + unaff_SI);
  *pcVar1 = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < bVar10);
  uVar9 = ((u32)param_6 >> 0x10);
  iVar8 = (int)param_6;
  *param_6 = 0x389a;
  (iVar8 + 0x2) = 0x1008;
  (u32)(iVar8 + 0x4) = param_8;
  (u32)(iVar8 + 0x8) = param_7;
  (iVar8 + 0xc) = 0x0;
  (u32)(iVar8 + 0xe) = 0x0;
  (iVar8 + 0x12) = 0x0;
  *param_6 = 0xdd4a;
  (iVar8 + 0x2) = 0x1008;
  return;
}
pub fn struct_1008_dc90(param_1: *mut astruct_212,mut param_2: u32,mut param_3: u32)

{
  astruct_212 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_212 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = param_3;
  iVar1->field3_0x8 = param_2;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0xe = 0x0;
  iVar1->field6_0x12 = 0x0;
  param_1->field0_0x0 = 0xdd4a;
  iVar1->field1_0x2 = 0x1008;
  return;
}
pub fn struct_1008_dcdc(param_1: *mut astruct_220)

{
  astruct_220 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_220 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x8 = 0x0;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0xe = 0x0;
  iVar1->field6_0x12 = 0x0;
  param_1->field0_0x0 = 0xdd4a;
  iVar1->field1_0x2 = 0x1008;
  return;
}



u16 * pass1_1008_dd1e(param_1: *mut u16,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_1008_dd4e(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar4: u16;
  astruct_57 *paVar3;
  astruct_19 *paVar5;

  uVar4 = ((u32)in_EDX >> 0x10);
  paVar5 = struct_op_1010_1d48(param_1,param_2);
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(int)((u32)paVar5 >> 0x10));
  uVar1 = 0x0;
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  (u32)((int)param_1 + 0x16) = 0x0;
  (u32)((int)param_1 + 0x1a) = 0x0;
  (u32)((int)param_1 + 0x1e) = 0x0;
  param_1->offset_0x0 = 0xeaac;
  ((int)param_1 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  if (uVar2 == 0x0) {
    (u32)((int)param_1 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    ((int)param_1 + 0xa) = uVar1;
    ((int)param_1 + 0xc) = uVar2;
  }
  return;
}
pub fn pass1_1008_ddca(param_1: *mut astruct_455)

{
  u32 *puVar1;
  let mut uVar2: u16;
  u32 *puVar3;
  code **ppcVar4;
  astruct_455 *iVar5;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xeaac;
  iVar5->field1_0x2 = 0x1008;
  puVar1 = (u32 *)iVar5[0x1].field3_0x6;
  uVar2 = (iVar5 + 0x2)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = (u32 *)iVar5[0x2].field1_0x2;
  puVar3 = iVar5[0x2].field2_0x4;
  if ((puVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = (u32 *)iVar5[0x1].field1_0x2;
  puVar3 = iVar5[0x1].field2_0x4;
  if ((puVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar5[0x3].field3_0x6);
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_de58(param_1: *mut astruct_211,i32 param_2,i32 param_3)

{
  code **ppcVar1;
  let mut bVar2: bool;
  char *pstring_1;
  let mut uVar3: u16;
  astruct_57 *in_EDX;
  astruct_211 *pstruct211_1;
  char *pcVar4;
  astruct_211 *pstruct211_2;
  let mut uVar5: u32;
  char local_a [0x8];

  pstruct211_2 = (astruct_211 *)((u32)param_1 >> 0x10);
  pstruct211_1 = (astruct_211 *)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)pstruct211_1->field10_0xa);
  bVar2 = false;
  do {
    pstring_1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,pstring_1));
    uVar3 = in_EDX;
    in_EDX = (astruct_57 *)((u32)in_EDX & 0xffff0000 | (u32)(uVar3 | pstring_1));
    pcVar4 = pstring_1;
    if ((uVar3 | pstring_1) == 0x0) goto LAB_1008_dedb;
  } while (((*(i32 *)(pstring_1 + 0x4) != param_3) || (*(i32 *)(pstring_1 + 0x8) != param_2)) &&
          ((*(i32 *)(pstring_1 + 0x8) != param_3 || (*(i32 *)(pstring_1 + 0x4) != param_2))));
  (pstring_1 + 0xc) = 0x1;
  uVar5 = pass1_1030_8326();
  in_EDX = (astruct_57 *)((u32)in_EDX & 0xffff0000 | uVar5 >> 0x10);
  pcVar4 = (char *)uVar5;
  *(char **)(pstring_1 + 0xe) = pcVar4;
  (pstring_1 + 0x10) = (int)(uVar5 >> 0x10);
  bVar2 = true;//
LAB_1008_dedb:
  if (!bVar2) {
    mem_op_1000_179c(0x14,in_EDX);
    uVar3 = in_EDX | pcVar4;
    if (uVar3 == 0x0) {
      pcVar4 = NULL;
      uVar3 = 0x0;
    }
    else {
      struct_1008_dc90((astruct_212 *)CONCAT22(in_EDX,pcVar4),param_2,param_3);
    }
    (pcVar4 + 0xc) = 0x1;
    uVar5 = pass1_1030_8326();
    (pcVar4 + 0xe) = (int)uVar5;
    (pcVar4 + 0x10) = (int)(uVar5 >> 0x10);
    ppcVar1 = (code **)((int)*pstruct211_1->field10_0xa + 0x4);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_df4a(param_1: *mut astruct_102,mut param_2: i16,mut param_3: u16 )

{
  let mut uVar1: u16;
  astruct_102 *paVar2;
  let mut uVar3: u32;
  char local_a [0x8];

  paVar2 = (astruct_102 *)((u32)param_1 >> 0x10);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)&((astruct_102 *)param_1)->field_0xa);
  while( true ) {
    uVar3 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar1 = (uVar3 >> 0x10);
    if (uVar3 == 0x0) break;
    if ((((int)uVar3 + 0xc) == 0x2) || (((int)uVar3 + 0xc) == 0x3)) {
      pass1_1008_e9a4((astruct_102 *)param_1,paVar2,uVar3);
    }
  }
  return;
}
pub fn pass1_1008_dfa6(mut param_1: u32,i32 param_2,i32 param_3)

{
  u8 *puVar1;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (((*(i32 *)(puVar1 + 0x4) != param_3) || (*(i32 *)(puVar1 + 0x8) != param_2)) &&
          ((*(i32 *)(puVar1 + 0x8) != param_3 || (*(i32 *)(puVar1 + 0x4) != param_2))));
  if ((puVar1 + 0xc) != 0x1) {
    return;
  }
  return;
}
pub fn pass1_1008_e01c(mut param_1: u32,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (u32)((int)param_1 + 0x16) = param_3;
  (u32)((int)param_1 + 0x1a) = param_2;
  return;
}
pub fn pass1_1008_e038(mut param_1: u32,u32 *param_2,u32 *param_3)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (u32)((int)param_1 + 0x16);
  *param_2 = (u32)((int)param_1 + 0x1a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_e05e(param_1: *mut astruct_102,mut param_2: u16 ,char *param_3,char *param_4)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  i32 lVar3;
  let mut uVar4: u32;
  u8 local_122 [0x112];
  let mut iStack16: i16;
  char local_e [0x8];

  lVar3 = pass1_1008_e8cc(param_1,param_3,param_4);
  uVar1 = ((u32)lVar3 >> 0x10);
  iVar2 = (int)lVar3;
  if (lVar3 != 0x0) {
    uVar4 = pass1_1030_8326();
    (iVar2 + 0xe) = (int)uVar4;
    (iVar2 + 0x10) = (int)(uVar4 >> 0x10);
    (iVar2 + 0xc) = param_2;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_e),(u32)((int)param_1 + 0xa));
  iStack16 = 0x0;
  do {
    lVar3 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_e));
    if (lVar3 == 0x0) goto LAB_1008_e0d3;
  } while (((int)lVar3 + 0xc) != 0x1);
  iStack16 = 0x1;//
LAB_1008_e0d3:
  if (iStack16 == 0x0) {
    struct_1030_e2be((astruct_97 *)CONCAT22(0x1050,local_122),0x0,0x0,0x0);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_122));
  }
  return;
}



i16 pass1_1008_e10c(param_1: *mut astruct_102,char *param_2,char *param_3,mut param_4: i16,mut param_5: u16 )

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u32;

  uVar3 = pass1_1008_e8cc(param_1,param_2,param_3);
  if (uVar3 == 0x0) {
    return 0x1;
  }
  iVar1 = ((int)uVar3 + 0xc);
  if (-0x1 < iVar1) {
    if (iVar1 < 0x2) {
      return 0x1;
    }
    if ((0x0 < iVar1 + -0x1) && (iVar2 = iVar1 + -0x3, iVar2 == 0x0 || iVar1 + -0x2 < 0x1)) {
      pass1_1008_e9a4((astruct_102 *)param_1,(astruct_102 *)((u32)param_1 >> 0x10),uVar3);
      return iVar2;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_e164(param_1: *mut astruct_102)

{
  code **ppcVar1;
  let mut uVar5: u16;
  astruct_216 *paVar2;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar4;
  astruct_102 *uVar11;
  astruct_213 *iVar12;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uStack292: u32;
  let mut uStack288: u32;
  let mut uStack284: u32;
  u8 local_118 [0x112];
  i32 lStack6;
  astruct_216 *iVar1;

  uVar7 = ((u32)in_EDX >> 0x10);
  uVar6 = ((u32)param_1 >> 0x10);
  uVar11 = (astruct_102 *)param_1;
    // WARNING: Load size is inaccurate
    // WARNING: Load size is inaccurate
  lStack6 = pass1_1008_e8cc(param_1,uVar11->field25_0x1a,uVar11->field22_0x16);
  uVar3 = ((u32)lStack6 >> 0x10);
  uVar5 = lStack6;
  paVar4 = (astruct_57 *)CONCAT22(uVar7,uVar3 | uVar5);
  if (lStack6 == 0x0) {
    // WARNING: Load size is inaccurate
    pass1_1008_e852(uVar3 | uVar5,uVar11,uVar6,uVar11->field22_0x16);
    uStack288 = CONCAT22(paVar4,uVar5);
    // WARNING: Load size is inaccurate
    pass1_1008_e852(paVar4,uVar11,uVar6,uVar11->field25_0x1a);
    uStack292 = CONCAT22((int)paVar4,uVar5);
    mem_op_1000_179c(0x14,paVar4);
    uVar3 = paVar4 | uVar5;
    if (uVar3 == 0x0) {
      uVar5 = 0x0;
      uVar3 = 0x0;
    }
    else {
      struct_1008_dc90((astruct_212 *)CONCAT22(paVar4,uVar5),uStack292,uStack288);
    }
    lStack6 = CONCAT22(uVar3,uVar5);
    (uVar5 + 0xc) = 0x1;
    uVar8 = pass1_1030_8326();
    uVar7 = ((u32)lStack6 >> 0x10);
    iVar12 = (astruct_213 *)lStack6;
    iVar12->field14_0xe = (int)uVar8;
    iVar12->field15_0x10 = (int)(uVar8 >> 0x10);
    ppcVar1 = (code **)((int)*(u32*)&uVar11->field_0xa + 0x4);
    (**ppcVar1)(0x1030,(u32)&uVar11->field_0xa,iVar12,uVar7);
  }
  else {
    iVar1 = *(astruct_216 **)(uVar5 + 0xc);
    paVar2 = iVar1 + -0x1;
    if (paVar2 == NULL) {
      return;
    }
    if (((0x0 < (int)paVar2) && (!SBORROW2((int)paVar2,0x1))) && ((int)(iVar1 + -0x2) < 0x2)) {
      (uVar5 + 0x12) = 0x1;
    }
    (uVar5 + 0xc) = 0x1;
  }
  uVar7 = ((u32)lStack6 >> 0x10);
  struct_1030_e2be((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,local_118)),0x1,(u32)((int)lStack6 + 0x8),
                   (u32)((int)lStack6 + 0x4));
  uVar8 = pass1_1030_8326();
  uStack284 = CONCAT22((int)(uVar8 >> 0x10) + (0xfffe < uVar8),uVar8 + 0x1);
  pass1_1030_8372(_u16_1050_5748,uStack284,(u32 *)CONCAT22(0x1050,local_118));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1008_e2a4(param_1: *mut astruct_102,char *param_2,char *param_3)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  char *string_2;
  i32 lVar3;
  char *string_1;

  string_1 = param_2;
  string_2 = load_string_1010_847e(_u16_1050_14cc,0x531);
  iVar1 = pass1_1000_3d7a(string_2,string_1);
  if ((iVar1 == 0x0) || (iVar1 = pass1_1000_3d7a(param_3,param_2), iVar1 == 0x0)) {
    return 0x0;
  }
  lVar3 = pass1_1008_e8cc(param_1,param_2,param_3);
  if (lVar3 != 0x0) {
    iVar1 = ((int)lVar3 + 0xc);
    iVar2 = iVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x2;
    }
    if (iVar2 < 0x1) {
      return 0x0;
    }
    if (SBORROW2(iVar2,0x1)) {
      return 0x0;
    }
    if (0x1 < iVar1 + -0x2) {
      return 0x0;
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_e320(param_1: *mut astruct_102,char *param_2,char *param_3)

{
  let mut uVar2: i16;
  let mut uVar1: u16;
  let mut uVar3: u16;
  astruct_102 *struct_1;
  astruct_102 *struct_1_hi;
  char *string_1;
  i32 lVar4;
  let mut uStack12: u16;
  char *string_2;

  struct_1_hi = (astruct_102 *)((u32)param_1 >> 0x10);
  struct_1 = (astruct_102 *)param_1;
  fn_ptr_1000_17ce(*(char **)&struct_1->field28_0x1e);
  (u32)&struct_1->field28_0x1e = 0x0;
  string_2 = param_2;
  string_1 = load_string_1010_847e(_u16_1050_14cc,0x531);
  uVar3 = ((u32)string_1 >> 0x10);
  uVar2 = pass1_1000_3d7a(string_1,string_2);
  if ((uVar2 == 0x0) || (uVar2 = pass1_1000_3d7a(param_3,param_2), uVar2 == 0x0)) {
    uStack12 = 0x443;
  }
  else {
    lVar4 = pass1_1008_e8cc(param_1,param_2,param_3);
    uVar1 = ((u32)lVar4 >> 0x10);
    uVar2 = (i16)lVar4;
    uVar3 = uVar1 | uVar2;
    if (uVar3 == 0x0) {
      uStack12 = 0x444;
    }
    else {
      uStack12 = 0x443;
      uVar1 = (uVar2 + 0xc);
      uVar2 = uVar1;
      if (uVar1 != 0x0) {
        uVar2 = uVar1 - 0x1;
        if (uVar2 == 0x0) {
          uStack12 = 0x445;
          goto LAB_1008_e378;
        }
        uVar2 = uVar1 - 0x2;
        if (uVar2 != 0x0) {
          uVar2 = uVar1 - 0x3;
          if (uVar2 == 0x0) {
            uStack12 = 0x446;
          }
          goto LAB_1008_e378;
        }
      }
      uStack12 = 0x444;
    }
  }//
LAB_1008_e378:
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),uStack12);
  struct_1->field28_0x1e = uVar2;
  struct_1->field29_0x20 = uVar3;
  return;
}
pub fn pass1_1008_e3ec(param_1: *mut astruct_218,u32 *param_2,u32 *param_3)

{
  u32 *puVar1;
  code **ppcVar2;
  astruct_92 *pstruct92_1;
  u32 *puVar3;
  astruct_92 *puVar4;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar6;
  astruct_218 *iVar10;
  let mut uVar9: u16;
  astruct_92 struct92_1;
  let mut uVar7: u32;
  let mut uVar8: u32;

  uVar9 = ((u32)param_1 >> 0x10);
  iVar10 = (astruct_218 *)param_1;
    // WARNING: Load size is inaccurate
  puVar3 = iVar10->field14_0xe;
  uVar4 = ((int)&iVar10->field14_0xe + 0x2);
  paVar6 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar4);
  if ((uVar4 | puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
  }
  mem_op_1000_179c(0x18,paVar6);
  uVar4 = paVar6 | puVar3;
  uVar8 = (u32)paVar6 & 0xffff0000;
  uVar7 = uVar8 | uVar4;
  if (uVar4 == 0x0) {
    puVar3 = NULL;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,puVar3)),0x5,0x5);
    uVar8 = uVar7;
  }
  (u32*)&iVar10->field14_0xe = puVar3;
  ((int)&iVar10->field14_0xe + 0x2) = (int)uVar8;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&struct92_1),0x1,0x0,0x400);
  while( true ) {
    pstruct92_1 = &struct92_1;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,pstruct92_1));
    uVar4 = uVar8;
    uVar5 = uVar4 | pstruct92_1;
    uVar7 = uVar8 & 0xffff0000;
    uVar8 = uVar7 | uVar5;
    if (uVar5 == 0x0) break;
    if (pstruct92_1[0x1c].field4_0x8 != 0x8000002) {
      puVar1 = iVar10->field14_0xe;
      ppcVar2 = (code **)((int)*iVar10->field14_0xe + 0xc);
      (**ppcVar2)(0x28,(int)puVar1,(int)((u32)puVar1 >> 0x10));
    }
  }
  *param_3 = (u32)iVar10->field14_0xe;
  uVar4 = ((int)&iVar10->field15_0x12 + 0x2);
  puVar3 = (u32 *)iVar10->field15_0x12;
  uVar5 = uVar4 | puVar3;
  paVar6 = (astruct_57 *)(uVar7 | uVar5);
  if (uVar5 != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)(0x1028,puVar3);
  }
  mem_op_1000_179c(0x18,paVar6);
  uVar4 = paVar6 | puVar3;
  if (uVar4 == 0x0) {
    puVar3 = NULL;
    uVar4 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,puVar3)),0x5,0x5);
  }
  (u32*)&iVar10->field15_0x12 = puVar3;
  ((int)&iVar10->field15_0x12 + 0x2) = uVar4;
  if (struct92_1.field6_0x10 != 0x0) {
    struct92_1.field5_0xc._0_2_ = 0x1;
    struct92_1.field5_0xc = 0x0;
  }
  uVar8 = (u32)struct92_1.field5_0xc;
  struct92_1.field4_0x8._0_2_ = struct92_1.field5_0xc;
  struct92_1.field4_0x8 = struct92_1.field5_0xc;
  while( true ) {
    uVar4 = uVar8;
    puVar4 = &struct92_1;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,puVar4));
    uVar8 = (u32)(uVar4 | puVar4);
    if ((uVar4 | puVar4) == 0x0) break;
    puVar1 = iVar10->field15_0x12;
    ppcVar2 = (code **)((int)*iVar10->field15_0x12 + 0xc);
    (**ppcVar2)(0x28,(int)puVar1,(int)((u32)puVar1 >> 0x10));
  }
  *param_2 = (u32)iVar10->field15_0x12;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn string_1008_e586(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ) -> u32

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  char *in_string_2;
  char *pcStack10;
  char *pcStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
  pcStack6 = (char *)CONCAT22(param_5,param_4);
  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_5 | param_4);
  if ((param_5 | param_4) == 0x0) {
    return 0x0;
  }
  mem_op_1000_179c(0x80,paVar1);
  pcStack10 = (char *)CONCAT22((int)paVar1,param_4);
  in_string_2 = pass1_1038_4d28(pcStack6);
  unk_str_op_1000_3d3e(pcStack10,in_string_2);
  return CONCAT22((int)paVar1,param_4);
}
pub fn pass1_1008_e5da(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2:bool;
  u8 *puVar3;
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  HFILE16 in_stack_0000ffba;
  u32 local_30 [0x2];
  let mut local_28: u32;
  u32 local_24 [0x2];
  u16 local_1c [0x3];
  u16 local_16 [0x3];
  let mut uStack16: u32;
  u8 local_c [0x8];
  let mut uStack4: u16;

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 != 0x0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = (int)param_1;
    if (*(i32 *)(iVar4 + 0xa) == 0x0) {
      uStack4 = 0x0;
    }
    else {
      uVar1 = (u32)(iVar4 + 0xa);
      uStack4 = ((int)uVar1 + 0x8);
    }
    local_1c[0] = uStack4;
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffba);
    if (BVar2 != 0x0) {
      pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)(iVar4 + 0xa));
      do {
        puVar3 = local_c;
        pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
        uStack16 = CONCAT22(extraout_DX,puVar3);
        if ((extraout_DX | puVar3) == 0x0) {
          return;
        }
        local_24[0] = (u32)(puVar3 + 0x4);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_24),(char *)0x4,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_28 = (u32)((int)uStack16 + 0x8);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_28),(char *)0x4,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_16[0] = ((int)uStack16 + 0xc);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_16),(char *)0x2,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_30[0] = (u32)((int)uStack16 + 0xe);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_30),(char *)0x4,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_16[0] = ((int)uStack16 + 0x12);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_16),(char *)0x2,in_stack_0000ffba);
      } while (BVar2 != 0x0);
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn file_1008_e70e(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut BVar2:bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  u16 local_12 [0x2];
  u32 *puStack14;
  let mut uStack10: u16;
  let mut local_4: u16;
  astruct_57 *paVar6;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if ((int)u16_1050_0312 < 0x2) {
    return;
  }
  read_file_1008_7cfe((int)param_4,(int)(param_4 >> 0x10),0x14);
  if (param_1 != 0x0) {
    BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar2 != 0x0) {
      if (local_4 == 0x0) {
        return;
      }
      uStack10 = 0x0;
      while( true ) {
        if (local_4 <= uStack10) {
          return;
        }
        uVar3 = local_4;
        mem_op_1000_179c(0x14,paVar5);
        uVar4 = paVar5 | uVar3;
        paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
        if (uVar4 == 0x0) {
          uVar3 = 0x0;
          paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000);
        }
        else {
          struct_1008_dcdc((astruct_220 *)CONCAT22(paVar5,uVar3));
          paVar5 = paVar6;
        }
        puStack14 = (u32 *)CONCAT22((int)paVar5,uVar3);
        BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22((int)paVar5,uVar3 + 0x4),0x4);
        if ((((BVar2 == 0x0) ||
             (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                          (u8 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0x8)),0x4),
             BVar2 == 0x0)) ||
            (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,local_12),0x2), BVar2 == 0x0)) ||
           ((BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                         (u8 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0xe)),0x4),
            BVar2 == 0x0 ||
            (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                         (u8 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0x12)),0x2),
            BVar2 == 0x0)))) break;
        ((int)puStack14 + 0xc) = local_12[0];
        ppcVar1 = (code **)((int)(u32)(u32)((int)param_3 + 0xa) + 0x4);
        (**ppcVar1)();
        uStack10 += 0x1;
      }
      if (puStack14 != NULL) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1000,(int)puStack14,(int)((u32)puStack14 >> 0x10),0x1,puStack14);
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn pass1_1008_e852(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,char *param_4)

{
  astruct_92 *pstruct92_1;
  let mut iVar1: i16;
  char *pcVar2;
  astruct_92 struct92_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&struct92_14),0x1,0x0,0x400);
  do {
    pstruct92_1 = &struct92_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,pstruct92_1));
    if ((param_1 | pstruct92_1) == 0x0) {
      return;
    }
    pcVar2 = pass1_1038_4d28((char *)CONCAT22(param_1,pstruct92_1));
    param_1 = ((u32)pcVar2 >> 0x10);
    iVar1 = pass1_1000_3d7a(param_4,(char *)((u32)pcVar2 & 0xffff | (u32)param_1 << 0x10));
  } while (iVar1 != 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 pass1_1008_e8cc(param_1: *mut astruct_102,char *param_2,char *param_3)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  i32 lVar6;
  char *pcVar7;
  char *pcVar8;
  char *pcStack22;
  char *pcStack18;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0xa));
  while( true ) {
    lVar6 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar4 = ((u32)lVar6 >> 0x10);
    uVar1 = lVar6;
    uVar5 = uVar4 | uVar1;
    if (lVar6 == 0x0) {
      return 0x0;
    }
    uVar2 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar1 + 0x4));
    pcStack18 = (char *)CONCAT22(uVar5,uVar2);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar1 + 0x8));
    pcStack22 = (char *)CONCAT22(uVar5,uVar2);
    pcVar7 = pass1_1038_4d28(pcStack18);
    pcVar8 = pass1_1038_4d28(pcStack22);
    iVar3 = pass1_1000_3d7a(param_3,pcVar7);
    if ((iVar3 == 0x0) && (iVar3 = pass1_1000_3d7a(param_2,pcVar8), iVar3 == 0x0)) break;
    iVar3 = pass1_1000_3d7a(param_2,pcVar7);
    if ((iVar3 == 0x0) && (iVar3 = pass1_1000_3d7a(param_3,pcVar8), iVar3 == 0x0)) {
      return lVar6;
    }
  }
  return lVar6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_e9a4(param_1: *mut astruct_102,param_2: *mut astruct_102,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffe8: u16;
  let mut iStack20: i16;
  let mut uStack16: u32;
  let mut uStack6: u32;

  uVar5 = ((u32)in_EDX >> 0x10);
  uVar8 = pass1_1030_8326();
  uVar7 = (param_3 >> 0x10);
  iVar6 = (int)param_3;
  puVar1 = (u16 *)(iVar6 + 0xe);
  uVar2 = uVar8 - *puVar1;
  iVar4 = ((int)(uVar8 >> 0x10) - (iVar6 + 0x10)) - (uVar8 < *puVar1);
  uStack6 = CONCAT22(iVar4,uVar2);
  mixed_1010_20ba((astruct_57 *)CONCAT22(uVar5,iVar4),_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe8,0x2),
                  in_stack_0000fe90,in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
  uStack16 = 0x0;
  if (((int)PTR_LOOP_1050_13ae < 0x1) || (SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) goto LAB_1008_ea2b;
  if (PTR_LOOP_1050_13ae == (u8 *)&u16_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
    if ((iVar6 + 0x12) == 0x0) {//
LAB_1008_ea20:
      uVar3 = 0x1e;
    }
    else {
      uVar3 = 0xa;
    }
  }
  else if (PTR_LOOP_1050_13ae == (u8 *)((int)&u16_1050_0002 + 0x1)) {
    if ((iVar6 + 0x12) == 0x0) {
      uVar3 = 0x28;
    }
    else {
      uVar3 = 0x14;
    }
  }
  else {
    if (PTR_LOOP_1050_13ae != (u8 *)&u32_1050_0004) goto LAB_1008_ea2b;
    if ((iVar6 + 0x12) != 0x0) goto LAB_1008_ea20;
    uVar3 = 0x32;
  }
  uStack16 = (u32)uVar3;//
LAB_1008_ea2b:
  if (uStack16 < uStack6) {
    pass1_1008_612e(uVar2,0x1,0x64);
    iStack20 = 0x0;
    iVar4 = (iVar6 + 0xc);
    if (iVar4 == 0x2) {
      iStack20 = 0x32;
    }
    else if (iVar4 == 0x3) {
      iStack20 = 0x19;
    }
    if ((int)uStack6 < iStack20) {
      return;
    }
  }
  return;
}



u16 * pass1_1008_ea86(param_1: *mut u16,u8 param_2,mut param_3: u16 )

{
  pass1_1008_ddca((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_eabc(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0xc)));
  param_1->offset_0x0 = 0xeb1a;
  ((int)param_1 + 0x2) = 0x1008;
  return &param_1->offset_0x0;
}



u16 * pass1_1008_eaf4(param_1: *mut u16,u8 param_2,mut param_3: u16 )

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1008_eb2a(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xc) = 0x0;
  param_1->offset_0x0 = 0xec00;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}



pub fn pass1_1008_eb5c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16) -> u32

{
  return CONCAT22(0x1050,param_3 * 0x10 + 0xd0e);
}



u16 pass1_1008_eb6e(void)

{
  return 0x5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_eb74(u8 *param_1,mut param_2: u32,mut param_3: i16)

{
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;

  ((int)param_2 + 0xa) = param_3;
  if (param_3 != 0x0) {
    mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                    (u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe98,in_stack_0000ffbc,in_stack_0000ffc2,
                    in_stack_0000ffc6);
    pass1_1010_c312();
  }
  return;
}



u16 * pass1_1008_ebda(param_1: *mut u16,u8 param_2,mut param_3: u16 )

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1008_ec10(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 )

{
  struct_op_1010_1d48((astruct_19 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  CONCAT22(param_2,param_1) = 0xec62;
  (param_1 + 0x2) = 0x1008;
  return (u16 *)CONCAT22(param_2,param_1);
}



u16 * pass1_1008_ec3c(param_1: *mut u16,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_223 * struct_1008_ec72(param_1: *mut astruct_223)

{
  struct_1010_383a(param_1);
  param_1->field0_0x0 = 0xefc4;
  ((int)param_1 + 0x2) = 0x1008;
  return param_1;
}
pub fn pass1_1008_ec94(StructD *param_1)

{
  param_1->address_offset_field_0x0 = 0xefc4;
  ((int)param_1 + 0x2) = 0x1008;
  pass1_1010_3880(param_1);
  return;
}



astruct_19 * struct_1008_ecb2(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_19,mut param_4: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  struct_1010_2cd2(param_3,param_4);
  param_3->offset_0x0 = 0xef9c;
  ((int)param_3 + 0x2) = 0x1008;
  mem_op_1000_179c(0x20c,paVar1);
  ((int)param_3 + 0x5c) = param_1;
  ((int)param_3 + 0x5e) = (int)paVar1;
  pass1_1000_4906((StructD *)CONCAT22((int)paVar1,((int)param_3 + 0x5c)),NULL,0x20c);
  return param_3;
}
pub fn pass1_1008_ed00(param_1: *mut u16)

{
  *param_1 = 0xef9c;
  ((int)param_1 + 0x2) = 0x1008;
  pass1_1010_2db2((astruct_455 *)param_1);
  return;
}
pub fn mem_1008_ed1e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,mut param_5: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_5);
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,paVar1);
    return;
  }
  mem_op_1000_179c(0x1a,paVar1);
  if ((paVar1 | param_4) != 0x0) {
    struct_1008_ec72((astruct_223 *)CONCAT22(paVar1,param_4));
    return;
  }
  return;
}
pub fn pass1_1008_ed62(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x16) = param_2 * 0x8 + 0xd5e;
  (iVar1 + 0x18) = (int)&DAT_1050_1050;
  (iVar1 + 0x12) = (param_2 * 0x8 + 0xd64);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_ed8a(u32 *param_1,mut param_2: u16 ,mut param_3: i16,mut param_4: i16,mut param_5: i16,mut param_6: u16 ,
                    mut param_7: u16 )

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  char cVar4;
  astruct_57 *in_EDX;
  let mut uVar5: u16;
  let mut unaff_SI: u16;
  let mut uVar6: u16;
  let mut bVar7: bool;
  let mut uVar8: u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;

  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_0df6 == NULL) {
      ppcVar1 = (code **)((int)*param_1 + 0x18);
      uVar3 = (**ppcVar1)();
      _PTR_LOOP_1050_0df6 =
           mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,uVar3),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
    }
    uVar2 = (u32)((int)param_1 + 0xc);
    uVar8 = pass1_1010_2e02((u32)_PTR_LOOP_1050_0df6,((int)uVar2 + 0x12));
    uVar5 = param_2 + 0x1;
    uVar6 = param_3 + (0xfffe < param_2);
    for (cVar4 = ((char)param_4 + -0x1) * '\x04'; cVar4 != '\0'; cVar4 += -0x1) {
      bVar7 = CARRY2(uVar5,uVar5);
      uVar5 *= 0x2;
      uVar6 = uVar6 * 0x2 + bVar7;
    }
    pass1_1010_2e30((u32)_PTR_LOOP_1050_0df6,uVar5 | uVar8,uVar6 | (uVar8 >> 0x10),
                    (param_5 * 0x8 + 0xd64));
  }
  return;
}
pub fn pass1_1008_ee14(mut param_1: u32,mut param_2: u16 )

{
  u8 *puVar1;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_223 *paVar5;
  u8 local_1c [0x1a];

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x56) == 0x0) {
    paVar5 = struct_1008_ec72((astruct_223 *)CONCAT22(0x1050,local_1c));
    uVar2 = ((u32)paVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e(puVar1,(u32 *)CONCAT22(0x1050,puVar1),0x0,0x0,0x0);
    (iVar3 + 0x56) = puVar1;
    (iVar3 + 0x58) = uVar2;
    pass1_1008_ec94((StructD *)CONCAT22(0x1050,local_1c));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1008_ee56(void)

{
  char *pcVar1;
  let mut in_stack_00000004: u32;

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,*(u16*)((int)in_stack_00000004 + 0x16));
  return pcVar1;
}
pub fn pass1_1008_ee72(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;

  if (*(i32 *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)((int)(u32)CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



u16 pass1_1008_eea6(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool pass1_1008_eeac(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  char cVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  u32 *puVar7;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffee: u32;

  uVar5 = ((int)param_4 + 0x12);
  puVar7 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)in_stack_0000ffee >> 0x10),0x3),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  uVar1 = ((u32)puVar7 >> 0x10);
  uVar2 = puVar7;
  uVar6 = uVar1;
  if (uVar5 == 0x7d) {
    pass1_1010_a5ca(0x7d,uVar1,uVar2,uVar1,0x7c);
    if (uVar5 != 0x0) {
      return false;
    }
    pass1_1010_a5ca(0x0,uVar6,uVar2,uVar1,0x7d);
    if (uVar5 != 0x0) {
      return false;
    }
    uVar4 = uVar5;
    uVar5 = 0x78;
  }
  else {
    uVar4 = uVar5;
    if (uVar5 < 0x7e) {
      cVar3 = (char)uVar5;
      uVar4 = uVar5 & 0xff00;
      if ((u8)(cVar3 + 0x8dU) == 0x0) {
        uVar5 = 0x9;
        uVar4 = uVar4 | (u8)(cVar3 + 0x8dU);
      }
      else if ((u8)(cVar3 + 0x89U) == 0x0) {
        uVar5 = 0x2e;
        uVar4 = uVar4 | (u8)(cVar3 + 0x89U);
      }
      else {
        uVar4 |= (u8)(cVar3 + 0x87U);
        if ((u8)(cVar3 + 0x87U) == 0x0) {
          uVar5 = 0x5b;
        }
      }
    }
  }
  pass1_1010_a5ca(uVar4,uVar6,uVar2,uVar1,uVar5);
  return uVar4 == 0x0;
}



u16 pass1_1008_ef38(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (u32)((int)param_1 + 0x16);
  return ((int)uVar1 + 0x2);
}



u16 pass1_1008_ef4a(void)

{
  return 0x41;
}



StructD * pass1_1008_ef50(StructD *param_1,u8 param_2)

{
  pass1_1008_ec94(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_ef76(StructD *param_1,u8 param_2)

{
  pass1_1008_ed00(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
