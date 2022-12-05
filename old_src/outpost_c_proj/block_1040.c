//
// Created by cyrex on 2022-05-22.
//

#include "block_1040.h"


void win_ui_op_1040_0000(Struct57*param_1,StructB *struct_b_param_1,u16 param_3)

{
  Struct57*rect;
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 unaff_SI;
  u16 uVar5;
  u16 uVar6;
  u32 uVar7;
  u32 *puVar8;
  u32 uVar9;
  u16 in_stack_0000fe16;
  u16 in_stack_0000fe1a;
  u16 in_stack_0000fe6a;
  u16 in_stack_0000ff40;
  u16 in_stack_0000ff44;
  u16 in_stack_0000ff48;
  u16 in_stack_0000ff8e;
  u16 in_stack_0000ff94;
  u16 in_stack_0000ff98;
  u16 local_24;
  u16 uStack34;
  u16 uStack32;
  u16 uStack30;
  u16 uStack28;
  u16 local_1a;
  u16 uStack24;
  u16 uStack22;
  u16 uStack18;
  u16 uStack16;
  u16 uStack14;
  i16 iStack12;
  astruct_915 *paStack10;
  Struct57*paStack8;
  u16 uStack6;
  i16 iStack4;
  u16 iVar1;
  u32 uVar4;

    // Segment:    9
    // Offset:     0006f820
    // Length:     d974
    // Min Alloc:  d974
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  iStack4 = 0x8;
  for (paStack10 = NULL; uVar5 = struct_b_param_1, uVar6 = ((u32)struct_b_param_1 >> 0x10),
      (int)paStack10 < iStack4; paStack10 = paStack10 + 0x1) {
    iVar1 = (int)paStack10 * 0xe;
    local_24 = (iVar1 + 0x5c60);
    uStack34 = (iVar1 + 0x5c62);
    uStack32 = 0x1;
    uStack30 = 0x1;
    rect = (Struct57*)&local_24;
    MapDialogRect16((RECT16 *)rect,(HWND16)0x1050);
    mem_op_1000_179c(0x42,param_1);
    uVar2 = (Struct57*)param_1 | rect;
    uVar4 = (u32)param_1 & 0xffff0000 | (u32)uVar2;
    if (uVar2 == 0x0) {
      rect = NULL;
      uVar4 = (u32)param_1 & 0xffff0000;
    }
    else {
      pass1_1008_3bd6(uVar4,rect,(Struct57*)param_1,0x1,CONCAT22(local_24,uStack34),0x104,0x1020103,
                      CONCAT22((uVar5 + 0x6),(iVar1 + 0x5c64)),param_3,in_stack_0000fe16,
                      in_stack_0000fe1a,in_stack_0000ff40,in_stack_0000ff44,in_stack_0000ff48);
    }
    uStack6 = uVar4;
    paStack8 = rect;
    uVar7 = win_ui_op_1040_0558(struct_b_param_1,paStack10);
    param_1 = (Struct57*)(uVar4 & 0xffff0000 | uVar7 >> 0x10);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  puVar8 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x48),in_stack_0000fe6a,
                           in_stack_0000ff8e,in_stack_0000ff94,in_stack_0000ff98);
  uStack16 = ((u32)puVar8 >> 0x10);
  uStack18 = puVar8;
  iStack12 = (uStack18 + 0xa);
  uStack14 = (uStack18 + 0xc);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_1a),*(HWND16 *)(uVar5 + 0x6));
  uVar3 = iStack12 >> 0xf;
  uStack28 = uStack22 - local_1a;
  local_1a = (iStack12 / 0x2 - uStack28) - 0x3;
  if ((int)local_1a < 0x0) {
    local_1a = 0x0;
  }
  SetWindowPos16(0x41,0x0,0x0,uStack24,local_1a,0x0,*(HWND16 *)(uVar5 + 0x6));
  uVar9 = pass1_1038_af40(uVar5,uVar3,_PTR_LOOP_1050_5b7c,(uVar5 + 0x6),0x17);
  uVar3 = ((u32)uVar9 >> 0x10);
  uVar1 = uVar9;
  (uVar5 + 0x96) = uVar1;
  (uVar5 + 0x98) = uVar3;
  win_1008_5c7c(uVar1,uVar3,_u16_1050_02a0,0x9e0001);
  (uVar5 + 0x8c) = uVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_0170(undefined1 param_1,u16 param_2,struct *param_3,u16 param_4,u16 param_5,param_6: i16)

{
  i16 iVar1;
  HWND16 hwnd_1;
  BOOL16 BVar2;
  astruct_915 *paVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  Struct57*paVar5;
  u16 unaff_SI;
  u32 uVar6;
  LRESULT LVar7;
  u32 *puVar8;
  char *l_param;
  u32 uVar9;
  u16 in_stack_0000fd7c;
  u16 in_stack_0000fd86;
  u16 in_stack_0000fea0;
  u16 in_stack_0000fea6;
  u16 in_stack_0000feaa;
  u16 in_stack_0000feb0;
  u16 in_stack_0000feb4;
  HCURSOR16 *pHVar10;
  u16 uVar11;
  u8 uVar12;
  u8 uVar13;
  u16 uVar14;
  WPARAM16 w_param;
  u16 msg;
  INT16 id;
  u32 in_stack_0000fedc;
  u32 uVar15;
  HCURSOR16 local_18;
  u16 local_16;
  astruct_598 *paStack20;
  astruct_915 *paStack16;
  u16 uStack14;
  u32 *puStack12;
  astruct_915 *paStack8;
  WPARAM16 WStack6;
  i16 iStack4;
  astruct_890 *iVar2;
  astruct_891 *iVar3;

  paVar5 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x8;
  WStack6 = 0x0;
  switch(param_6) {
  case 0x167:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x0;
    break;
  case 0x168:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x1;
    break;
  case 0x169:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x2;
    break;
  case 0x16a:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x3;
    break;
  case 0x16b:
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    BVar2 = EnableWindow16(0x0,hwnd_1);
    if (((int)param_3 + 0x92) != 0x3) {
      win_1008_5c5c(BVar2,paVar5,_u16_1050_02a0,0x1de);
    }
    if (((int)param_3 + 0x92) != 0x8) {
      iVar2 = (astruct_890 *)(((int)param_3 + 0x92) * 0xe);
      WStack6 = *(WPARAM16 *)(iVar2 + 0x5c6c);
      pass1_1010_6604((u32)((int)param_3 + 0x8e),(iVar2 + 0x5c66));
      ((int)param_3 + 0x92) = 0x8;
    }
    for (paStack8 = NULL; (int)paStack8 < 0x4; paStack8 = paStack8 + 0x1) {
      uVar6 = win_ui_op_1040_0558((StructB *)param_3,paStack8);
      paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000 | uVar6 >> 0x10);
    }
    goto LAB_1040_04da;
  case 0x16c:
    hwnd_1 = GetDlgItem16(0x16d,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x5;
    ((int)param_3 + 0x94) = 0x5;
    goto LAB_1040_04da;
  case 0x16d:
    hwnd_1 = GetDlgItem16(0x16d,*(HWND16 *)((int)param_3 + 0x6));
    BVar2 = EnableWindow16(0x0,hwnd_1);
    win_1008_5c5c(BVar2,paVar5,_u16_1050_02a0,0x1de);
    uVar11 = ((u32)paVar5 >> 0x10);
    if (((int)param_3 + 0x94) != 0x8) {
      iVar3 = (astruct_891 *)(((int)param_3 + 0x94) * 0xe);
      WStack6 = *(WPARAM16 *)(iVar3 + 0x5c6c);
      pass1_1010_6604((u32)((int)param_3 + 0x8e),(iVar3 + 0x5c66));
      ((int)param_3 + 0x94) = 0x8;
    }
    LVar7 = win_ui_op_1040_0558((StructB *)param_3,(astruct_915 *)((int)&u32_1050_0004 + 0x1));
    paVar5 = (Struct57*)CONCAT22(uVar11,(int)((u32)LVar7 >> 0x10));
    puStack12 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x39),in_stack_0000fd7c,
                                in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
    paVar3 = *(astruct_915 **)((int)puStack12 + 0x20);
    uVar14 = SUB42(0x1050,0x0);
    uVar12 = SUB21(&local_16,0x0);
    uVar13 = (u8)(&local_16 >> 0x8);
    pHVar10 = &local_18;
    uVar11 = SUB42(0x1050,0x0);
    uStack14 = ((int)paVar3 >> 0xf) + 0x200;
    uVar6 = (u32)paVar5 & 0xffff0000 | (u32)uStack14;
    paStack16 = paVar3;
    paStack8 = paVar3;
    pass1_1030_8344((u32)_u16_1050_5748,CONCAT22(uStack14,paVar3));
    paStack20 = (astruct_598 *)CONCAT22((int)uVar6,paVar3);
    pass1_1030_2f1a(CONCAT22((int)uVar6,paVar3),(u16 *)CONCAT22(uVar11,pHVar10),
                    (u16 *)CONCAT22(uVar14,CONCAT11(uVar13,uVar12)));
    paVar5 = (Struct57*)(uVar6 & 0xffff0000 | (u32)((int)(local_18 - local_16) >> 0xf));
    local_16 += (int)(local_18 - local_16) / 0x2;
    uVar4 = pass1_1030_2fac(paStack20);
    set_window_text_1018_6086((u32)((int)param_3 + 0x96),uVar4,local_16);
    goto LAB_1040_04da;
  case 0x16e:
    puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x39),in_stack_0000fd7c,
                             in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
    paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
    iVar1 = ((int)puVar8 + 0x20);
    local_18 = LoadCursor16((char *)0x7f02,0x0);
    local_16 = SetCursor16(local_18);
    pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,&stack0xfed6),(long)iVar1 + 0x2000000);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&stack0xfed6));
    pass1_1030_838e((u32 *)_u16_1050_5748);
    pass1_1030_8334();
    SetCursor16(local_16);
    PostMessage16(0x0,0x7e,0x111,HWND16_1050_0396);
    DestroyWindow16(*(HWND16 *)((int)param_3 + 0x6));
    goto LAB_1040_04da;
  default:
    post_win_msg_1040_7b3c((StructC *)param_3,param_4,param_5,param_6);
    return;
  }
  ((int)param_3 + 0x92) = iStack4;
LAB_1040_04da:
  uVar11 = (in_stack_0000fedc >> 0x10);
  if (iStack4 != 0x8) {
    uVar15 = in_stack_0000fedc & 0xffff0000 | (u32)((int)param_3 + 0x6);
    id = *(INT16 *)(iStack4 * 0xe + 0x5c68);
    w_param = 0x0;
    msg = 0xc;
    l_param = load_string_1010_847e(_u16_1050_14cc,(iStack4 * 0xe + 0x5c6a));
    uVar6 = (u32)paVar5 & 0xffff0000;
    uVar9 = SendDlgItemMessage16((LPARAM)l_param,w_param,msg,id,(HWND16)uVar15);
    uVar11 = (uVar15 >> 0x10);
    paVar5 = (Struct57*)(uVar6 & 0xffff0000 | uVar9 >> 0x10);
  }
  if ((WStack6 != 0x0) &&
     (puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar11,0x2),in_stack_0000fd86,
                               in_stack_0000feaa,in_stack_0000feb0,in_stack_0000feb4),
     ((int)puVar8 + 0x20) != 0x0)) {
    PostMessage16(0x0,WStack6,0x111,HWND16_1050_0396);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT win_ui_op_1040_0558(StructB *param_1,astruct_915 *param_2)

{
  HWND16 hwnd;
  i16 iVar2;
  StructB *iVar3;
  u16 uVar3;
  char *l_param;
  LRESULT LVar4;
  u16 uVar5;
  WPARAM16 w_param;
  u16 msg;
  INT16 id;
  LPVOID hwnd_00;
  astruct_913 *iVar1;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (StructB *)param_1;
  iVar1 = (astruct_913 *)((int)param_2 * 0xe);
  hwnd = GetDlgItem16(*(INT16 *)(iVar1 + 0x5c64),(HWND16)iVar3->lpvoid_field_0x8);
  iVar2 = pass1_1010_659a((u32)&iVar3[0x7].field1_0x2,(iVar1 + 0x5c66));
  if ((iVar2 == 0x0) && ((iVar1 + 0x5c66) != 0xa)) {
    EnableWindow16(0x0,hwnd);
    hwnd_00 = iVar3->lpvoid_field_0x8;
    id = *(INT16 *)((int)param_2 * 0xe + 0x5c68);
    uVar5 = 0x531;
  }
  else {
    EnableWindow16(0x1,hwnd);
    hwnd_00 = iVar3->lpvoid_field_0x8;
    id = *(INT16 *)((int)param_2 * 0xe + 0x5c68);
    uVar5 = 0x649;
  }
  msg = 0xc;
  w_param = 0x0;
  l_param = load_string_1010_847e(_u16_1050_14cc,uVar5);
  LVar4 = SendDlgItemMessage16((LPARAM)l_param,w_param,msg,id,(HWND16)hwnd_00);
  return LVar4;
}



// WARNING: Could not reconcile some variable overlaps

void enable_win_1040_060e(param_1: u32,param_2: i16)

{
  INT16 *pIVar1;
  HWND16 hwnd;
  i16 iStack10;
  INT16 *pIStack8;

  pIStack8 = (INT16 *)CONCAT22(0x1050,&stack0x000a);
  iStack10 = param_2;
  while( true ) {
    pIVar1 = pIStack8;
    if (iStack10 == 0x0) break;
    pIStack8 = (INT16 *)((u32)pIStack8 & 0xffff0000 | (u32)((int)pIStack8 + 0x2));
    hwnd = GetDlgItem16(*pIVar1,*(HWND16 *)((int)param_1 + 0x6));
    EnableWindow16(0x0,hwnd);
    iStack10 = iStack10 + -0x1;
  }
  return;
}



StructD * pass1_1040_0656(StructD *param_1,u8 param_2)

{
  destroy_win_1038_ef3a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

Struct57*
pass1_1040_06e8(StructD *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfbc,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0xb90;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x7),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_073a(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xb90;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void show_win_1040_0766(StructB *struct_b_param_1,u16 param_2)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  u32 *puVar2;
  u16 in_stack_0000fe7e;
  u16 in_stack_0000fe86;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb4;
  i16 *piVar3;
  u16 uVar4;
  i16 *piVar5;
  u16 uVar6;
  u16 in_stack_0000ffde;
  i16 local_a;
  i16 local_8;
  u16 uStack6;
  u16 uStack4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x2),in_stack_0000fe86,
                           in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
  paVar1 = (Struct57*)((u32)paVar1 & 0xffff0000 | (u32)puVar2 >> 0x10);
  uStack6 = SUB42(puVar2,0x0);
  uStack4 = ((u32)puVar2 >> 0x10);
  pass1_1010_6118((u32)puVar2);
  piVar5 = &local_8;
  uVar6 = SUB42(0x1050,0x0);
  piVar3 = &local_a;
  uVar4 = SUB42(0x1050,0x0);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(piVar3,0x48),in_stack_0000fe7e,in_stack_0000ffa2
                           ,in_stack_0000ffa8,in_stack_0000ffac);
  pass1_1008_3e94((u16 *)((u32)puVar2 & 0xffff0000 | (u32)((int)puVar2 + 0xe)),(u16 *)CONCAT22(uVar4,piVar3),
                  (char *)CONCAT22(uVar6,piVar5));
  move_win_1040_826c(struct_b_param_1,local_a + 0x8c,local_8 + 0xb9);
  ShowWindow16(0x5,*(HWND16 *)((int)struct_b_param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_07dc(u16 param_1,StructC *pstruct_c_param_2,u16 param_3,u16 param_4,u16 param_5)

{
  code **ppcVar1;
  INT16 IVar2;
  u8 *puVar3;
  u8 *puVar4;
  u8 *puVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  u32 *puVar7;
  u32 *puVar8;
  u16 in_stack_0000f69a;
  u16 in_stack_0000f7be;
  u16 in_stack_0000f7c4;
  u16 in_stack_0000f7c8;
  BOOL16 BVar9;
  u16 in_stack_0000f7f2;
  u32 uStack2060;
  char local_806 [0x400];
  u32 local_406 [0x100];
  u32 uStack6;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  uStack6 = 0x0;
  if (param_5 == 0x73) {
    enable_window_1040_0acc(pstruct_c_param_2,0x0);
    puVar4 = (u8 *)paVar6;
    puVar3 = pass1_1008_5fd8(puVar4);
    uStack2060 = CONCAT22(puVar4,puVar3);
    puVar5 = puVar4;
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_806,(short)0x1050);
    IVar2 = MessageBox16(0x34,(char *)CONCAT13(0x10,CONCAT12(0x50,local_806)),(char *)CONCAT22(puVar4,puVar3),
                         HWND16_1050_0396);
    local_406[0] = uStack2060;
    fn_ptr_1000_17ce((char *)CONCAT22(puVar4,puVar3));
    if (IVar2 == 0x6) {
      PostMessage16(0x0,0xcb,0x111,HWND16_1050_0396);
      BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2,param_3,param_4,0x1);
      uStack6 = CONCAT22(puVar5,BVar9);
    }
  }
  else {
    if (param_5 < 0x74) {
      if (param_5 == 0x6e) {
        ((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        puVar8 = (u32 *)
                 pass1_1038_af40(pstruct_c_param_2,param_1,_PTR_LOOP_1050_5b7c,
                                 (pstruct_c_param_2 + 0x6),0x2);
        ppcVar1 = (code **)((int)*puVar8 + 0x3c);
        (**ppcVar1)((int)&u16_1050_1038,(int)puVar8,(int)((u32)puVar8 >> 0x10));
        SetFocus16(*(HWND16 *)(pstruct_c_param_2 + 0x6));
        return;
      }
      if (0x6e < param_5) {
LAB_1040_09f9:
        post_win_msg_1040_7b3c(pstruct_c_param_2,param_3,param_4,param_5);
        return;
      }
      if ((char)param_5 == '\x02') {
LAB_1040_09b4:
        post_win_msg_1040_7b3c(pstruct_c_param_2,0x0,0x0,0x2);
        PostMessage16(0x0,0xee,0x111,HWND16_1050_0396);
        return;
      }
      if ((char)param_5 != 'd') goto LAB_1040_09f9;
      PostMessage16(0x0,0x64,0x111,HWND16_1050_0396);
      BVar9 = 0x0;
      goto LAB_1040_0821;
    }
    if (param_5 != 0x74) {
      if (param_5 == 0xee) goto LAB_1040_09b4;
      if (param_5 == 0x13d) {
        enable_window_1040_0acc(pstruct_c_param_2,0x1);
        return;
      }
      goto LAB_1040_09f9;
    }
    enable_window_1040_0acc(pstruct_c_param_2,0x0);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)local_406,
               (short)0x1050);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_806,(short)0x1050);
    IVar2 = MessageBox16(0x34,(char *)CONCAT13(0x10,CONCAT12(0x50,local_406)),(char *)CONCAT22(0x1050,local_806),
                         HWND16_1050_0396);
    if (IVar2 == 0x6) {
      PostMessage16(0x0,0x7a,0x111,HWND16_1050_0396);
      BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2,param_3,param_4,0x1);
      uStack6 = CONCAT22((int)paVar6,BVar9);
      puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000f7f2,0x2),in_stack_0000f69a,
                               in_stack_0000f7be,in_stack_0000f7c4,in_stack_0000f7c8);
      pass1_1010_60fa((u32)puVar7);
    }
  }
  BVar9 = 0x1;
LAB_1040_0821:
  enable_window_1040_0acc(pstruct_c_param_2,BVar9);
  return;
}



void pass1_1040_0a1a(u32 param_1)

{
  u16 uVar1;
  u32 *puVar2;
  code **ppcVar3;
  u32 uVar4;
  astruct_394 *paVar5;
  u16 uVar6;
  u8 *puVar7;
  u32 in_EDX;
  Struct57*paVar8;
  i16 iVar9;
  i16 iVar10;
  u16 uVar11;
  u16 uVar12;
  u32 uStack10;
  u16 uStack6;

  uVar11 = (param_1 >> 0x10);
  iVar9 = (int)param_1;
  uVar4 = (u32)(iVar9 + 0x8e);
  uVar12 = ((u32)uVar4 >> 0x10);
  iVar10 = (int)uVar4;
  puVar2 = (u32 *)(u32)(iVar10 + 0xa);
  uVar1 = (iVar10 + 0xc);
  paVar8 = (Struct57*)(in_EDX & 0xffff0000 | (u32)uVar1);
  uStack6 = puVar2;
  paVar5 = (astruct_394 *)(uVar1 | uStack6);
  if (paVar5 == NULL) {
    return;
  }
  ppcVar3 = (code **)((int)*puVar2 + 0x14);
  (**ppcVar3)();
  uStack10 = CONCAT22((int)paVar8,paVar5);
  if (*(i32 *)(iVar9 + 0x70) != 0x0) {
    paVar5 = *(astruct_394 **)(iVar9 + 0x70);
    uVar1 = (iVar9 + 0x72);
    uVar6 = uVar1 | paVar5;
    paVar8 = (Struct57*)((u32)paVar8 & 0xffff0000 | (u32)uVar6);
    if (uVar6 != 0x0) {
      ppcVar3 = (code **)(u32)paVar5;
      (**ppcVar3)();
    }
  }
  mem_op_1000_179c(0x14,paVar8);
  puVar7 = (u8 *)(paVar8 | paVar5);
  if (puVar7 == NULL) {
    paVar5 = NULL;
    puVar7 = NULL;
  }
  else {
    struct_1008_4c58(paVar5);
  }
  *(astruct_394 **)(iVar9 + 0x70) = paVar5;
  *(u8 **)(iVar9 + 0x72) = puVar7;
  pass1_1008_4d84(puVar7,*(astruct_90 **)(iVar9 + 0x70),uStack10);
  return;
}



void enable_window_1040_0acc(StructC *param_1,BOOL16 enable_3)

{
  BOOL16 BVar1;
  HWND16 HVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  BVar1 = IsWindow16(*(HWND16 *)(iVar3 + 0x6));
  if (BVar1 != 0x0) {
    HVar2 = GetDlgItem16(0x64,*(HWND16 *)(iVar3 + 0x6));
    BVar1 = IsWindow16(HVar2);
    if (BVar1 != 0x0) {
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0x74,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0x73,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0x6e,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0xee,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
    }
  }
  return;
}



StructD * pass1_1040_0b6a(StructD *param_1,u8 param_2)

{
  pass1_1040_073a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

Struct57*
pass1_1040_0bfc(StructD *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcd,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0xdb0;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x39),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  iVar1->field86_0x74 = 0x1;
  return param_2;
}



void pass1_1040_0c54(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xdb0;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  (u32)((int)param_1 + 0x8e) = 0x0;
  ui_cleanup_op_1040_782c(param_1);
  return;
}



void show_win_1040_0c7c(StructB *param_1)

{
  u32 uVar1;
  u16 uVar2;
  u32 local_6;

  dialog_ui_fn_1040_78e2(param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x8e);
  pass1_1010_4f30(uVar1,((u32)uVar1 >> 0x10),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  move_win_1040_826c(param_1,(INT16)local_6,(BOOL16)((u32)local_6 >> 0x10));
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 set_text_bk_color_1040_0cc0(u32 *param_1,u16 param_2,u16 param_3,HWND16 hwnd_param_4)

{
  astruct_783 *iVar1;
  u16 uVar3;
  u32 uVar1;
  HGDIOBJ16 hobject;
  code **fn_ptr_1;

  hobject = GetStockObject16(BLACK_BRUSH);
  if (_u16_1050_5cd0 == 0x0) {
    fn_ptr_1 = (code **)((int)*param_1 + 0x68);
    uVar1 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,param_1,((int)param_1 + 0x6e));
    uVar1 = pass1_1008_4d72(uVar1);
    uVar3 = (uVar1 >> 0x10);
    iVar1 = (astruct_783 *)uVar1;
    _u16_1050_5cd0 = CONCAT22(CONCAT11(0x2,iVar1->field_0x94),CONCAT11(iVar1->field_0x95,iVar1->field_0x96));
  }
  if (0x3 < param_3) {
    if (param_3 == 0x4) {
      hobject = GetStockObject16(HOLLOW_BRUSH);
    }
    else if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
      return 0x0;
    }
  }
  SetTextColor16(_u16_1050_5cd0,hwnd_param_4);
  SetBkColor16(0x1000000,hwnd_param_4);
  return CONCAT22(0x1050,hobject);
}



void post_win_msg_1040_0d5e(u16 param_1,u16 param_2,param_3: i16)

{
  if (param_3 != 0x0) {
    PostMessage16(0x0,0x1,0x111,*(HWND16 *)(param_1 + 0x8));
  }
  return;
}



u16 pass1_1040_0d80(void)

{
  return 0x1;
}



void FUN_1040_0d86(void)

{
  return;
}



StructD * pass1_1040_0d8a(StructD *param_1,u8 param_2)

{
  pass1_1040_0c54(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_0e1c(StructD *param_1,Struct57*param_2,u16 param_3,param_4: u32,u16 param_5)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1c0,param_5);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  (u32)&iVar1[0x1].field2_0x4 = param_4;
  iVar1[0x1].field4_0x8 = 0x0;
  iVar1[0x1].field5_0xa = param_3;
    // just 0x11d2
  param_2->field0_0x0 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
    // just 0x1040
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3a),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_0e86(StructD *param_1)

{
  u16 uVar1;
  char *pcVar2;
  u16 uVar3;
  u32 in_EDX;
  i16 iVar5;
  u16 uVar6;
  u32 *puVar7;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffee;
  Struct57*paVar4;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  param_1->address_offset_field_0x0 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
  (iVar5 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pcVar2 = *(char **)(iVar5 + 0x92);
  uVar1 = (iVar5 + 0x94);
  uVar3 = uVar1 | pcVar2;
  paVar4 = (Struct57*)(in_EDX & 0xffff0000 | (u32)uVar3);
  if (uVar3 != 0x0) {
    pass1_1040_a5d0((StructD *)((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  PTR_LOOP_1050_5b82 = (u8 *)(iVar5 + 0x96);
  if (*(i32 *)(iVar5 + 0x92) == 0x0) {
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar5 + 0x6));
  }
  else {
    puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x32),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    pass1_1010_7b8c((u32)puVar7,(iVar5 + 0x6));
  }
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void FUN_1040_0f0c(u16 param_1,StructB *param_2)

{
  u32 uVar1;
  u16 in_AX;
  HWND16 HVar2;
  u32 in_EDX;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u32 uVar6;
  u32 *puVar7;
  u16 in_stack_0000fe6e;
  u16 in_stack_0000ff92;
  u16 in_stack_0000ff98;
  u16 in_stack_0000ff9c;
  INT16 id;
  u16 in_stack_0000ffc6;
  u8 local_2e [0x2];
  i16 iStack44;
  i16 local_26;
  i16 iStack36;
  i16 iStack34;
  i16 iStack32;
  i16 iStack30;
  u16 uStack28;
  i16 iStack26;
  i16 iStack24;
  i16 iStack22;
  i16 iStack20;
  i16 iStack18;
  i16 iStack16;
  i16 local_e;
  i16 iStack12;
  i16 iStack10;
  i16 iStack8;
  POINT16 local_6;

  dialog_ui_fn_1040_78e2(param_2);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (int)param_2;
  if ((iVar4 + 0x98) == 0x0) {
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_26),*(HWND16 *)(iVar4 + 0x6));
    uVar3 = ((u32)in_EDX >> 0x10);
    HVar2 = GetDlgItem16(0x1830,*(HWND16 *)(iVar4 + 0x6));
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_2e),HVar2);
    iStack34 -= local_26;
    iStack32 = (iStack44 - iStack36) + -0x2;
    SetWindowPos16(0x6,iStack32,iStack34,0x0,0x0,0x0,*(HWND16 *)(iVar4 + 0x6));
    CheckDlgButton16(0x1,0x1c1,*(HWND16 *)(iVar4 + 0x6));
    uVar1 = (u32)(iVar4 + 0x8e);
    ((int)uVar1 + 0xa) = 0x2;
    HVar2 = GetDlgItem16(0x1830,*(HWND16 *)(iVar4 + 0x6));
    in_stack_0000ffc6 = 0x0;
    EnableWindow16(0x0,HVar2);
  }
  else {
    uVar1 = (u32)(iVar4 + 0x92);
    uVar6 = struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x6),in_AX,(int)in_EDX);
    uVar3 = ((u32)in_EDX >> 0x10);
    if (((int)uVar6 + 0x20) == 0x2) {
      HVar2 = *(HWND16 *)(iVar4 + 0x6);
      id = 0x1c1;
    }
    else {
      HVar2 = *(HWND16 *)(iVar4 + 0x6);
      id = 0x1c2;
    }
    CheckDlgButton16(0x1,id,HVar2);
  }
  GetCursorPos16(&local_6);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_e),*(HWND16 *)(iVar4 + 0x6));
  iStack20 = iStack10 - local_e;
  iStack16 = -(iStack20 / 0x2 - local_6.x);
  iStack22 = iStack8 - iStack12;
  iStack18 = -(iStack22 / 0x2 - local_6.y);
  puVar7 = mixed_1010_20ba((Struct57*)CONCAT22(uVar3,iStack22 >> 0xf),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffc6,0x48),in_stack_0000fe6e,in_stack_0000ff92,
                           in_stack_0000ff98,in_stack_0000ff9c);
  uStack28 = ((u32)puVar7 >> 0x10);
  iStack30 = (int)puVar7;
  iStack24 = (iStack30 + 0xa);
  iStack26 = (iStack30 + 0xc);
  if (iStack24 < iStack20 + iStack16) {
    iStack16 = iStack24 - iStack20;
  }
  if (iStack26 < iStack22 + iStack18) {
    iStack18 = iStack26 - iStack22;
  }
  SetWindowPos16(0x45,0x0,0x0,iStack18,iStack16,0x0,*(HWND16 *)(iVar4 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_109c(u8 *param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  u32 uVar1;
  bool bVar2;
  i16 iVar3;
  u16 in_register_0000000a;
  Struct57*paVar4;
  u32 uVar5;
  Struct57*paVar6;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000fff2;

  paVar4 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  bVar2 = false;
  if (param_5 == 0x1c1) {
    (param_2 + 0x96) = 0x2;
    bVar2 = true;
  }
  else if (param_5 == 0x1c2) {
    (param_2 + 0x96) = 0x1;
    bVar2 = true;
  }
  else {
    if (param_5 != 0x1830) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    paVar6 = (Struct57*)
             mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x32),in_stack_0000fe9a,
                             in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
    uVar5 = (u32)paVar4 & 0xffff0000 | (u32)paVar6 >> 0x10;
    iVar3 = (int)paVar6;
    uVar1 = (u32)(param_2 + 0x92);
    ui_op_1010_79aa(paVar6,0xfb6,*(i32 *)((int)uVar1 + 0x6));
    if (iVar3 == 0x0) {
      uVar1 = (u32)(param_2 + 0x92);
      unk_win_op_1010_7300(uVar5,paVar6,0x0,0xc,(u32)((int)uVar1 + 0x6));
    }
  }
  if (bVar2) {
    uVar1 = (u32)(param_2 + 0x8e);
    ((int)uVar1 + 0xa) = (param_2 + 0x96);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_1152(u8 *param_1,i16 param_2,u16 param_3)

{
  u16 uVar1;
  u32 uVar2;
  u16 in_register_0000000a;
  Struct57*paVar3;
  i16 iVar4;
  u16 uVar5;
  u32 *puVar6;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000fff4;

  paVar3 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (*(i32 *)(param_2 + 0x92) != 0x0) {
    uVar2 = (u32)(param_2 + 0x8e);
    uVar1 = ((int)uVar2 + 0xa);
    puVar6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,0x3),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    uVar2 = (u32)(param_2 + 0x92);
    uVar5 = ((u32)uVar2 >> 0x10);
    iVar4 = (int)uVar2;
    pass1_1010_ae92((u32)puVar6,uVar1,(iVar4 + 0xa),(u32)(iVar4 + 0x6),
                    (u32)paVar3 & 0xffff0000 | (u32)puVar6 >> 0x10);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  PTR_LOOP_1050_5b80 = NULL;
  return;
}



StructD * pass1_1040_11ac(StructD *param_1,u8 param_2)

{
  pass1_1040_0e86(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

Struct57*
pass1_1040_123e(StructD *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfd1,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0x17b0;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x46),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_1290(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x17b0;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_12bc(undefined1 param_1,StructB *struct_b_param_1)

{
  u32 uVar1;
  HWND16 HVar2;
  StructB *struct_b_3;
  u16 uVar3;
  char *lparam;
  u8 local_54 [0x52];

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_3 = (StructB *)struct_b_param_1;
  uVar1 = (u32)&struct_b_3[0x7].field1_0x2;
  sys_1000_3f9c((char *)CONCAT22(0x1050,local_54),s__u_1050_5cd4,((int)uVar1 + 0xa));
  HVar2 = GetDlgItem16(0xfd2,(HWND16)struct_b_3->lpvoid_field_0x8);
  SendMessage16(CONCAT22(0x1050,local_54),0x0,0xc,HVar2);
  SetFocus16(HVar2);
  SendMessage16(-0x10000,0x0,0x401,HVar2);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  HVar2 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x5,(HWND16)struct_b_3->lpvoid_field_0x8);
  send_msg_1040_1696(struct_b_param_1,HVar2);
  SendMessage16((LPARAM)lparam,0xffff,0x40d,HVar2);
  HVar2 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x4,(HWND16)struct_b_3->lpvoid_field_0x8);
  send_msg_1040_1696(struct_b_param_1,HVar2);
  SendMessage16((LPARAM)lparam,0xffff,0x40d,HVar2);
  ShowWindow16(0x5,(HWND16)struct_b_3->lpvoid_field_0x8);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_msg_op_1040_13b2(astruct_892 *param_1,param_2: i16)

{
  HWND16 HVar1;
  u16 uVar4;
  i16 iVar4;
  u8 *puVar5;
  i16 iVar5;
  char *puVar4;
  u8 *puVar6;
  u8 *puVar7;
  u32 in_EDX;
  u16 uVar5;
  Struct57*paVar2;
  astruct_892 *struct_7;
  i16 iVar6;
  u16 struct_5_lo;
  u16 uVar6;
  LRESULT lresult_4;
  char *pcVar6;
  u32 *puStack266;
  char local_100 [0x52];
  u8 local_aa [0x52];
  u16 uStack88;
  HWND16 handle_86;
  u8 local_54 [0x52];
  u32 uVar1;
  u32 uVar2;
  u32 uVar3;
  code **fn_ptr_1;

  uVar5 = ((u32)in_EDX >> 0x10);
  struct_7 = (astruct_892 *)param_1;
  struct_5_lo = ((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    handle_86 = GetDlgItem16(0xfd2,struct_7->hwnd_0x6);
    SendMessage16(CONCAT22(0x1050,local_54),0x51,0xd,handle_86);
    uStack88 = pass1_1000_3e2c(CONCAT22(0x1050,local_54));
    HVar1 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x4,struct_7->hwnd_0x6);
    lresult_4 = SendMessage16(0x0,0x0,0x407,HVar1);
    if ((WPARAM16)lresult_4 != 0xffff) {
      SendMessage16(CONCAT22(0x1050,local_aa),(WPARAM16)lresult_4,0x408,HVar1);
    }
    HVar1 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x5,struct_7->hwnd_0x6);
    lresult_4 = SendMessage16(0x0,0x0,0x407,HVar1);
    if ((WPARAM16)lresult_4 != 0xffff) {
      SendMessage16(CONCAT22(0x1050,local_100),(WPARAM16)lresult_4,0x408,HVar1);
    }
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x531);
    paVar2 = (Struct57*)CONCAT22(uVar5,local_aa);
    uVar4 = pass1_1000_3d7a((char *)CONCAT22(0x1050,local_aa),(char *)CONCAT22(0x1050,local_100));
    if (uVar4 != 0x0) {
      uVar4 = pass1_1000_3d7a((char *)CONCAT22(0x1050,local_aa),pcVar6);
      if (uVar4 != 0x0) {
        uVar4 = pass1_1000_3d7a((char *)CONCAT22(0x1050,local_100),pcVar6);
        if (uVar4 != 0x0) {
          pass1_1010_531c(local_aa,(u8 *)paVar2,struct_7->field141_0x8e,CONCAT22(0x1050,local_aa));
          puVar5 = (u8 *)local_100;
          pass1_1010_52fc(puVar5,(u8 *)paVar2,struct_7->field141_0x8e,CONCAT22(0x1050,puVar5));
          pass1_1010_5120(puVar5,paVar2,struct_7->field141_0x8e,uStack88);
          if (puVar5 == NULL) {
            mem_op_1000_179c(0xb4,paVar2);
            puVar7 = (u8 *)(paVar2 | puVar5);
            uVar3 = (u32)paVar2 & 0xffff0000 | ZEXT24(puVar7);
            if (puVar7 == NULL) {
              iVar5 = 0x0;
              uVar5 = 0x0;
            }
            else {
              iVar5 = string_1040_8520(uVar3,(Struct57*)CONCAT22(paVar2,puVar5),HWND16_1050_0396,0x20030,
                                       0x7d2057b);
              uVar5 = uVar3;
            }
            fn_ptr_1 = (code **)((int)(u32)CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,iVar5)) +
                                0x74);
            (**fn_ptr_1)();
            return;
          }
          uVar1 = struct_7->field141_0x8e;
          uVar2 = struct_7->field141_0x8e;
          uVar6 = (uVar2 >> 0x10);
          iVar6 = (i16)uVar2;
          uVar3 = struct_7->field141_0x8e;
          pass1_1028_8d9e((astruct_97 *)CONCAT22(0x1050,&stack0xfdd2),(u32)((int)uVar3 + 0xa),
                          (u32)((int)uVar1 + 0x12),
                          (u32)(iVar6 + 0x16) & 0xffff | (u32)(iVar6 + 0x18) << 0x10);
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&stack0xfdd2));
          pass1_1028_8dec((u16 *)CONCAT22(0x1050,&stack0xfdd2));
          goto LAB_1040_1619;
        }
      }
    }
    mem_op_1000_179c(0xb4,paVar2);
    puVar6 = (u8 *)(paVar2 | uVar4);
    uVar3 = (u32)paVar2 & 0xffff0000 | ZEXT24(puVar6);
    if (puVar6 == NULL) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = string_1040_8520(uVar3,(Struct57*)CONCAT22(paVar2,uVar4),HWND16_1050_0396,0x20030,0x755057b);
      uVar5 = uVar3;
    }
    puStack266 = (u32 *)CONCAT22(uVar5,iVar4);
    fn_ptr_1 = (code **)((int)*puStack266 + 0x74);
    (**fn_ptr_1)();
  }
LAB_1040_1619:
  DestroyWindow16(struct_7->hwnd_0x6);
  return;
}



u32 set_win_pos_1040_162a(u16 param_1,u16 param_2,param_3: u32,u32 param_4)

{
  u16 uVar1;
  BOOL16 BVar2;
  i16 iStack6;

  if ((param_4 != (int)s_vrpal_bmp_1050_183a + 0x5) && (param_4 != (int)s_vrpal_bmp_1050_183a + 0x4)) {
    BVar2 = post_win_msg_1040_7b3c
                      ((StructC *)CONCAT22((int)param_3,param_2),param_3,param_4,param_4);
    return CONCAT22(param_1,BVar2);
  }
  if (param_4 == 0x7) {
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff6),param_3);
    SetWindowPos16(0x2,0x50,iStack6 - param_3,0x0,0x0,0x0,param_3);
  }
  else if ((param_4 != 0x9) && (param_4 != 0xa)) {
    uVar1 = 0x0;
    goto LAB_1040_164d;
  }
  uVar1 = 0x1;
LAB_1040_164d:
  return (u32)uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void send_msg_1040_1696(StructB *param_1,u16 param_2)

{
  u32 uVar1;
  u32 uVar2;
  u16 *puVar3;
  u8 *puVar4;
  u8 *puVar5;
  u16 uVar6;
  LRESULT LVar7;
  char *pcVar8;
  WPARAM16 WVar9;
  u16 UVar10;
  u16 uVar11;
  u16 uStack18;
  u16 local_4;

  SendMessage16(0x0,0x0,0x40b,param_2);
  LVar7 = SendMessage16(0x0,0x0,0xb,param_2);
  puVar4 = (u8 *)((u32)LVar7 >> 0x10);
  local_4 = 0x0;
  puVar3 = &local_4;
  uVar6 = ((u32)param_1 >> 0x10);
  pass1_1010_519a(puVar4,(u32)((int)param_1 + 0x8e),(char *)CONCAT22(0x1050,puVar3));
  puVar5 = puVar4;
  for (uStack18 = 0x0; uStack18 < local_4; uStack18 += 0x1) {
    uVar1 = (u32)(puVar3 + uStack18 * 0x2);
    WVar9 = 0x0;
    UVar10 = 0x403;
    uVar2 = (u32)((int)param_1 + 0x8e);
    uVar11 = param_2;
    pcVar8 = (char *)string_1010_5286((char *)uVar1,puVar5,uVar2,((u32)uVar2 >> 0x10),uVar1);
    LVar7 = SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
    puVar5 = (u8 *)((u32)LVar7 >> 0x10);
    fn_ptr_1000_17ce(pcVar8);
  }
  WVar9 = 0x0;
  UVar10 = 0x40a;
  uVar11 = param_2;
  pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
  SendMessage16(0x0,0x1,0xb,param_2);
  return;
}



void FUN_1040_1786(void)

{
  return;
}



StructD * pass1_1040_178a(StructD *param_1,u8 param_2)

{
  pass1_1040_1290(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_181c(u16 param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfbb,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x1c48;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_1876(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x1c48;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void show_win_1040_18a2(StructB *struct_b_param_1)

{
  u32 uVar1;
  StructB *struct_b_2;
  u16 uVar2;
  WORD local_304 [0x80];
  char local_204 [0x100];
  char local_104 [0x100];
  u16 uStack4;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  check_dialog_btn_1040_1afe(struct_b_param_1);
  struct_b_2 = (StructB *)struct_b_param_1;
  uVar2 = ((u32)struct_b_param_1 >> 0x10);
  if (PTR_LOOP_1050_13ae != NULL) {
    if (PTR_LOOP_1050_13ae == (u8 *)&u16_1050_0002) {
      uStack4 = 0x621;
    }
    else if (PTR_LOOP_1050_13ae == (u8 *)((int)&u16_1050_0002 + 0x1)) {
      uStack4 = 0x622;
    }
    else if (PTR_LOOP_1050_13ae == (u8 *)&u32_1050_0004) {
      uStack4 = 0x623;
    }
    else {
      uStack4 = 0x620;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_204,(short)0x1050);
    wsprintf16(local_304,(char *)0x5cda1050,(char *)CONCAT22(local_204,0x1050),(int)0x1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfe0,(HWND16)struct_b_2->lpvoid_field_0x8);
    uVar1 = (u32)&struct_b_2[0x7].field1_0x2;
    if (((int)uVar1 + 0x82) == 0x0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_204,(short)0x1050);
    wsprintf16(local_304,(char *)0x5cdf1050,(char *)CONCAT22(local_204,0x1050),(int)0x1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfdf,(HWND16)struct_b_2->lpvoid_field_0x8);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_b_2->lpvoid_field_0x8);
  return;
}



void unk_win_ui_op_1040_19ea(astruct_32 *param_1,i16 param_2,u8 *param_3)

{
  StructD *pSVar1;
  u16 UVar2;
  astruct_32 *pstruct32_1;
  astruct_32 *pstruct_32_hi;

  pstruct32_1 = (astruct_32 *)param_1;
  pstruct_32_hi = (astruct_32 *)((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    UVar2 = IsDlgButtonChecked(0xfdb,pstruct32_1->field6_0x6);
    pass1_1010_5d9c(param_3,(u32)pstruct32_1->pstructd_0x8e,UVar2);
    UVar2 = IsDlgButtonChecked(0xfdc,pstruct32_1->field6_0x6);
    pSVar1 = pstruct32_1->pstructd_0x8e;
    ((int)pSVar1 + 0x20) = UVar2;
    UVar2 = IsDlgButtonChecked(0xfdd,pstruct32_1->field6_0x6);
    pSVar1 = pstruct32_1->pstructd_0x8e;
    ((int)pSVar1 + 0x74) = UVar2;
    UVar2 = IsDlgButtonChecked(0xfde,pstruct32_1->field6_0x6);
    pSVar1 = pstruct32_1->pstructd_0x8e;
    ((int)pSVar1 + 0x72) = UVar2;
    if (pstruct32_1->field142_0x92 != 0x0) {
      pSVar1 = pstruct32_1->pstructd_0x8e;
      pass1_1000_4906((StructD *)((u32)pSVar1 & 0xffff0000 | (u32)((int)pSVar1 + 0x22)),NULL,0x40);
    }
    if (pstruct32_1->field143_0x94 != 0x0) {
      pass1_1010_60a0((astruct_19 *)pstruct32_1->pstructd_0x8e);
    }
  }
  DestroyWindow16(pstruct32_1->field6_0x6);
  return;
}



u32 pass1_1040_1ab0(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  BOOL16 BStack6;
  u16 uStack4;

  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_5 == 0x1831) {
    (param_2 + 0x92) = 0x1;
    (param_2 + 0x94) = 0x1;
    check_dialog_btn_1040_1b8a((StructC *)CONCAT22(param_3,param_2));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
    uStack4 = param_1;
  }
  return CONCAT22(uStack4,BStack6);
}



void check_dialog_btn_1040_1afe(StructB *param_1)

{
  u16 check;
  u16 check_00;
  u32 uVar1;
  u32 uVar2;
  StructB *iVar3;
  u16 uVar3;
  u16 check_01;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (StructB *)param_1;
  uVar1 = (u32)&iVar3[0x7].field1_0x2;
  uVar2 = (u32)&iVar3[0x7].field1_0x2;
  check = ((int)uVar2 + 0x20);
  uVar2 = (u32)&iVar3[0x7].field1_0x2;
  check_00 = ((int)uVar2 + 0x74);
  uVar2 = (u32)&iVar3[0x7].field1_0x2;
  check_01 = ((int)uVar2 + 0x72);
  CheckDlgButton16(((int)uVar1 + 0x1e),0xfdb,(HWND16)iVar3->lpvoid_field_0x8);
  CheckDlgButton16(check_00,0xfdd,(HWND16)iVar3->lpvoid_field_0x8);
  CheckDlgButton16(check_01,0xfde,(HWND16)iVar3->lpvoid_field_0x8);
  CheckDlgButton16(check,0xfdc,(HWND16)iVar3->lpvoid_field_0x8);
  return;
}



void check_dialog_btn_1040_1b8a(StructC *param_1)

{
  u16 check;
  u16 check_00;
  u16 check_01;
  StructC *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructC *)param_1;
  check_00 = pass1_1010_60b4();
  pass1_1010_60c6();
  check_01 = pass1_1010_60c0();
  pass1_1010_60ba();
  CheckDlgButton16(check_00,0xfdb,iVar1->field6_0x6);
  CheckDlgButton16(check_01,0xfdd,iVar1->field6_0x6);
  CheckDlgButton16(0xfde,0xfde,iVar1->field6_0x6);
  check = iVar1->field6_0x6;
  CheckDlgButton16(check,0xfdc,check);
  return;
}



void FUN_1040_1c1e(void)

{
  return;
}



StructD * pass1_1040_1c22(StructD *param_1,u8 param_2)

{
  pass1_1040_1876(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_1cb4(StructD *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar2;
  u16 unaff_BP;
  Struct57*uVar2;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;
  u8 **ppuVar3;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xe8,param_6);
  uVar2 = (Struct57*)((u32)param_2 >> 0x10);
  iVar2 = (Struct57*)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  param_2->field0_0x0 = 0x1eee;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  ppuVar3 = (u8 **)CONCAT22(unaff_BP,0x2);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 0x1)->field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((Struct57*)((u32)paVar1 & 0xffff0000 | (u32)puVar2 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)ppuVar3 >> 0x10),0x37),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_1d24(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x1eee;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



void show_win_1040_1d50(StructB *param_1)

{
  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



void unk_win_ui_op_1040_1d7a(astruct_33 *param_1,param_2: i16)

{
  u16 UVar2;
  u16 UVar1;
  astruct_33 *iVar3;
  astruct_33 *uVar3;
  u32 uVar1;

  iVar3 = (astruct_33 *)param_1;
  uVar3 = (astruct_33 *)((u32)param_1 >> 0x10);
  if ((param_2 != 0x0) && (uVar1 = iVar3->field141_0x8e, ((int)uVar1 + 0x72) != 0x0)) {
    UVar2 = IsDlgButtonChecked(0xe1,iVar3->hwnd_0x6);
    if (UVar2 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d5);
    }
    UVar1 = IsDlgButtonChecked(0xe2,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d6);
    }
    UVar1 = IsDlgButtonChecked(0xe3,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d7);
    }
    UVar1 = IsDlgButtonChecked(0xe5,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d8);
    }
    UVar1 = IsDlgButtonChecked(0xe6,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1e2);
    }
    UVar1 = IsDlgButtonChecked(0xe7,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1dc);
    }
    return;
  }
  DestroyWindow16(iVar3->hwnd_0x6);
  return;
}



u32 pass1_1040_1e80(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  BOOL16 BStack6;
  u16 uStack4;

  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_5 == 0xe4) {
    pass1_1008_a9ec((u32)(param_2 + 0x92));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
    uStack4 = param_1;
  }
  return CONCAT22(uStack4,BStack6);
}



void FUN_1040_1ec4(void)

{
  return;
}



StructD * pass1_1040_1ec8(StructD *param_1,u8 param_2)

{
  pass1_1040_1d24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_1f5a(Struct57*param_1,u16 param_2,u32 param_3)

{
  u16 *puVar1;
  u16 uVar2;
  Struct57*paVar3;
  u16 unaff_DI;
  u16 unaff_CS;
  u32 uVar4;
  u32 *puVar5;
  Struct27 *paVar6;
  u16 in_stack_0000fe70;
  u16 in_stack_0000fe78;
  u16 in_stack_0000ff94;
  u16 in_stack_0000ff9a;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ff9e;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa6;
  u32 *puVar7;
  u32 *puVar8;
  u16 uVar9;
  u32 local_16;
  u32 uStack18;
  Struct57*iVar6;
  u16 uVar6;

  iVar6 = (Struct57*)param_1;
  uVar6 = ((u32)param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  (u32)(iVar6 + 0x1) = 0x0;
  iVar6[0x1].field10_0x14 = 0x0;
  iVar6[0x1].field11_0x18 = 0x0;
  param_1->field0_0x0 = 0x237e;
  iVar6->field1_0x2 = &PTR_LOOP_1050_1040;
  uVar2 = FUN_1010_830a(0x0,param_3,unaff_CS,_u16_1050_14cc,0x1cc);
  (iVar6 + 0x1)->field0_0x0 = uVar2;
  iVar6[0x1].field1_0x2 = param_3;
  uVar4 = pass1_1008_4772((Struct76 *)CONCAT22(param_3,(iVar6 + 0x1)->field0_0x0));
  paVar3 = (Struct57*)(param_3 & 0xffff0000 | uVar4 >> 0x10);
  uVar9 = (uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_DI,0x48),in_stack_0000fe78,
                           in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  local_16 = CONCAT22(((int)uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,((int)uVar4 + 0x4) + -0xa);
  (u32)&iVar6[0x1].field2_0x4 = local_16;
  (u32)&iVar6[0x1].field4_0x8 = uStack18;
  (u32)&iVar6[0x1].field6_0xc = local_16;
  (u32)&iVar6[0x1].field8_0x10 = uStack18;
  puVar1 = &iVar6[0x1].field7_0xe;
  *puVar1 = *puVar1 + 0x14;
  puVar8 = &iVar6[0x1].field10_0x14;
  puVar7 = &iVar6[0x1].field11_0x18;
  uVar9 = uVar6;
  paVar6 = (Struct27 *)
           mixed_1010_20ba((Struct57*)((u32)paVar3 & 0xffff0000 | (u32)puVar5 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22(puVar7,0x2b),in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  pass1_1010_0538(paVar6,(char **)CONCAT22(uVar6,puVar7),(char **)CONCAT22(uVar9,puVar8));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_205e(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  StructD *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x237e;
  iVar4->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  puVar1 = &iVar4->field_0x8e;
  uVar2 = &iVar4->field_0x90;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar4->field_0xa2);
  fn_ptr_1000_17ce(*(char **)&iVar4->field_0xa6);
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar4->field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address




// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mix_draw_op_1040_21d6(astruct_763 *param_1)

{
  astruct_13 *paVar1;
  code **ppcVar2;
  u8 uVar4;
  HPALETTE16 hpalette_7;
  u16 uVar7;
  HANDLE16 handle;
  u32 extraout_var;
  u16 in_DX;
  u16 DX_REG;
  astruct_763 *iVar10;
  i16 count;
  u32 uVar5;
  COLORREF color;
  HGDIOBJ16 handle_00;
  HDC16 hdc_24;
  PAINTSTRUCT16 *paintstruct_22;
  u8 uVar1;
  u32 *uVar2;
  u16 uVar3;
  astruct_764 *iVar5;
  u16 uVar11;

  count = (i16)((u32)param_1 >> 0x10);
  iVar10 = (astruct_763 *)param_1;
  hdc_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->field6_0x6);
  paVar1 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  hpalette_7 = palette_op_1008_4e08((HPALETTE16)&hdc_24,in_DX,paVar1,(HDC16 *)CONCAT22(0x1050,&hdc_24));
  uVar2 = iVar10->field141_0x8e;
  ppcVar2 = (code **)((int)*iVar10->field141_0x8e + 0x4);
  (**ppcVar2)(0x1008,(int)uVar2,(int)((u32)uVar2 >> 0x10),0xffff,0xffff,&hdc_24,(int)0x1050);
  uVar5 = pass1_1008_4d72((u32)paVar1);
  uVar3 = (uVar5 >> 0x10);
  iVar5 = (astruct_764 *)uVar5;
  uVar7 = CONCAT11(iVar5->field_0x3e5,iVar5->field_0x3e6);
  uVar1 = iVar5->field996_0x3e4;
  color = SetBkColor16(0x0,hdc_24);
  DX_REG = (color >> 0x10);
  uVar4 = SetTextColor16(CONCAT22(CONCAT11(0x2,uVar1),uVar7),hdc_24);
  handle_00 = 0x0;
  handle = GetProp16(s_hfont_1050_5ced,iVar10->field6_0x6);
  if (handle != 0x0) {
    handle_00 = SelectObject16(handle,hdc_24);
  }
  DrawText16(0x10,(RECT16 *)
                  ((u32)param_1 & 0xff000000 | (u32)CONCAT12((char)((u32)param_1 >> 0x10),&iVar10->rect_0x92)),
             -0x1,(LPCSTR)iVar10->field152_0xa2,hdc_24);
  SetTextColor16(CONCAT22(CONCAT11(0x2,iVar5->field_0x94),CONCAT11(iVar5->field_0x95,iVar5->field_0x96)),hdc_24);
  DrawText16(0x10,(RECT16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar10->field147_0x9a)),-0x1,
             (LPCSTR)iVar10->field153_0xa6,hdc_24);
  if (handle != 0x0) {
    SelectObject16(handle_00,hdc_24);
  }
  SetBkColor16(color,hdc_24);
  SetTextColor16(CONCAT31((u32)extraout_var,uVar4) & 0xffff | (u32)DX_REG << 0x10,hdc_24);
  hpalette_7 = SelectPalette16(0x0,hpalette_7,hdc_24);
  DeleteObject16(hpalette_7);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->field6_0x6);
  return;
}



StructD * pass1_1040_2358(StructD *param_1,u8 param_2)

{
  pass1_1040_205e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_23ea(u16 param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6,
                    u8 **param_7)

{
  u32 uVar1;
  u16 in_register_0000000a;
  Struct57*paVar2;
  Struct57*iVar2;
  Struct57*uVar2;
  u32 *puVar3;
  u16 in_stack_0000fea2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd0;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,param_3,0xfbd,param_6);
  uVar2 = (Struct57*)((u32)param_2 >> 0x10);
  iVar2 = (Struct57*)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  iVar2[0x1].field2_0x4 = 0x0;
  iVar2[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x2956;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2->field105_0x8a = 0x26;
  param_7 = (u8 **)CONCAT22((int)((u32)param_7 >> 0x10),0x6);
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,param_7,in_stack_0000fea2,in_stack_0000ffc6,in_stack_0000ffcc,
                           in_stack_0000ffd0);
  (iVar2 + 0x1)->field0_0x0 = puVar3;
  iVar2[0x1].field1_0x2 = ((u32)puVar3 >> 0x10);
  uVar1 = (u32)(iVar2 + 0x1);
  iVar2[0x1].field2_0x4 = ((int)uVar1 + 0x28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_2464(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x2956;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



void show_win_1040_2490(StructB *struct_b_param_1)

{
  code **ppcVar1;
  HWND16 hwnd;
  StructB *struct_b_4;
  u16 uVar3;
  i16 *piVar2;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_4 = (StructB *)struct_b_param_1;
  hwnd = GetDlgItem16(0xfb1,(HWND16)struct_b_4->lpvoid_field_0x8);
  EnableWindow16(0x0,hwnd);
  ppcVar1 = (code **)((int)*&struct_b_4[0x7].field1_0x2 + 0x10);
  piVar2 = (i16 *)(**ppcVar1)((int)s_tile2_bmp_1050_1538,(u32)&struct_b_4[0x7].field1_0x2);
  piVar2 = ((u32)piVar2 >> 0x10);
  move_win_1040_826c(struct_b_param_1,((int)piVar2 + 0x2) + -0x2,((int)piVar2 + 0x4) + *piVar2 + 0x3);
  ShowWindow16(0x5,(HWND16)struct_b_4->lpvoid_field_0x8);
  pass1_1018_1c9a(*(astruct_263 **)&struct_b_4[0x7].field1_0x2,0x1a0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 win_ui_op_1040_2512(Struct57*param_1,u16 param_2,StructC *param_3,param_4: u32,u16 param_5)

{
  u32 *puVar1;
  u16 uVar2;
  astruct_263 *paVar3;
  u16 uVar4;
  u16 UVar4;
  HWND16 HVar5;
  BOOL16 BVar6;
  i16 iVar6;
  i16 iVar7;
  u16 UVar6;
  u16 uVar7;
  u16 uVar11;
  u8 *puVar8;
  u16 uVar12;
  u8 *puVar9;
  StructC *iVar8;
  i16 iVar9;
  u16 uVar8;
  u16 uVar9;
  u16 *puVar15;
  u32 uVar10;
  u16 in_stack_0000fe84;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  u8 local_1e [0x4];
  u16 uStack26;
  u16 uStack24;
  u32 *local_16 [0x2];
  u16 uStack12;
  u32 *puStack10;
  BOOL16 BStack6;
  u16 uStack4;
  i16 *piVar1;
  u16 in_stack_0000ffdc;
  u32 uVar13;
  u32 uVar14;
  code **fn_ptr_21;

  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_5 == 0x2) {
LAB_1040_266d:
    BStack6 = 0x1;
    uStack4 = 0x0;
  }
  else {
    iVar8 = (StructC *)param_3;
    if (param_5 - 0x2 < 0x19e) {
LAB_1040_2539:
      param_2 = param_5;
    }
    else {
      uVar8 = ((u32)param_3 >> 0x10);
      if (param_5 - 0x1a0 < 0x14 || param_5 == 0x1b4) {
        UVar4 = IsDlgButtonChecked(param_5,iVar8->field6_0x6);
        if (UVar4 == 0x0) {
          puVar1 = &iVar8->field142_0x92;
          puVar1 = puVar1 + 0x1;
          if (0x0 < &iVar8->field142_0x92) {
            ((int)&iVar8->field142_0x92 + 0x2) = 0x0;
          }
          paVar3 = iVar8->field141_0x8e;
          if (((int)paVar3 + 0x28) == &iVar8->field142_0x92) {
            HVar5 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
            EnableWindow16(0x0,HVar5);
          }
        }
        else {
          puVar1 = &iVar8->field142_0x92;
          puVar1 = puVar1 + -0x1;
          HVar5 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
          BVar6 = IsWindowEnabled16(HVar5);
          if (BVar6 == 0x0) {
            HVar5 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
            EnableWindow16(0x1,HVar5);
          }
          if (&iVar8->field142_0x92 < 0x1) {
            ((int)&iVar8->field142_0x92 + 0x2) = 0x1;
          }
          pass1_1018_1c9a(iVar8->field141_0x8e,param_5);
          puStack10 = (u32 *)pass1_1018_1e78((u32)iVar8->field141_0x8e,-0x1);
          uVar2 = ((u32)puStack10 >> 0x10);
          uVar11 = uVar2 | puStack10;
          if (uVar11 == 0x0) {
            uStack12 = 0x0;
          }
          else {
            uStack12 = (puStack10 + 0x1c);
          }
          win_1008_5c7c(uStack12,uVar11,_u16_1050_02a0,CONCAT22(uStack12,0x1));
        }
        if ((-0x1 < &iVar8->field142_0x92) &&
           (paVar3 = iVar8->field141_0x8e, &iVar8->field142_0x92 <= ((int)paVar3 + 0x28))) {
          sys_1000_3f9c((char *)CONCAT13(0x10,CONCAT12(0x50,local_16)),s__d_1050_5cf4,&iVar8->field142_0x92);
          SetDlgItemText16(CONCAT22(0x1050,local_16),0xfb2,iVar8->field6_0x6);
        }
        goto LAB_1040_266d;
      }
      uVar4 = param_5 - 0xfb1;
      if (uVar4 != 0x0) goto LAB_1040_2539;
      if (&iVar8->field142_0x92 < 0x0) {
        mem_op_1000_179c(0xb4,param_1);
        uStack24 = param_1;
        puVar8 = (u8 *)(uStack24 | uVar4);
        uVar13 = (u32)param_1 & 0xffff0000 | ZEXT24(puVar8);
        uStack26 = uVar4;
        if (puVar8 == NULL) {
          iVar6 = 0x0;
          uVar12 = 0x0;
        }
        else {
          iVar6 = string_1040_8520(uVar13,(Struct57*)
                                          CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,uVar4)),
                                   HWND16_1050_0396,0x20030,0x57c057b);
          uVar12 = uVar13;
        }
        puStack10 = (u32 *)CONCAT22(uVar12,iVar6);
        fn_ptr_21 = (code **)((int)*puStack10 + 0x74);
        (**fn_ptr_21)(0x1000,iVar6,uVar12);
        goto LAB_1040_27c0;
      }
      if (0x0 < &iVar8->field142_0x92) {
        mem_op_1000_179c(0xb4,param_1);
        uStack24 = param_1;
        puVar9 = (u8 *)(uStack24 | uVar4);
        uVar13 = (u32)param_1 & 0xffff0000;
        uVar14 = uVar13 | ZEXT24(puVar9);
        uStack26 = uVar4;
        if (puVar9 == NULL) {
          iVar7 = 0x0;
        }
        else {
          iVar7 = string_1040_8520(uVar14,(Struct57*)
                                          CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,uVar4)),
                                   HWND16_1050_0396,0x20021,0x57d057b);
          uVar13 = uVar14;
        }
        puStack10 = (u32 *)CONCAT22((int)uVar13,iVar7);
        puVar15 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_1e),0x1,0xc2);
        param_1 = (Struct57*)(uVar13 & 0xffff0000 | (u32)puVar15 >> 0x10);
        param_2 = 0x1050;
        fn_ptr_21 = (code **)((int)*puStack10 + 0x6c);
        uVar10 = (**fn_ptr_21)(0x1008,(char)puStack10,(int)((u32)puStack10 >> 0x10),local_1e);
        if ((int)uVar10 == 0x2) goto LAB_1040_27c0;
      }
      local_16[0] = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x6),in_stack_0000fe84,
                                    in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      param_1 = (Struct57*)((u32)local_16[0] >> 0x10);
      uStack12 = 0x1a0;
      do {
        UVar6 = IsDlgButtonChecked(uStack12,iVar8->field6_0x6);
        if (UVar6 == 0x1) {
          uVar9 = ((u32)local_16[0] >> 0x10);
          iVar9 = (i16)local_16[0];
          (iVar9 + (iVar9 + 0x56) * 0x2 + 0x4e) = uStack12;
          piVar1 = (i16 *)(iVar9 + 0x56);
          *piVar1 = *piVar1 + 0x1;
        }
        uStack12 += 0x1;
      } while ((int)uStack12 < 0x1b5);
      uVar2 = &iVar8->field142_0x92;
      puStack10 = (u32 *)((u32)puStack10 & 0xffff0000 | (u32)uVar2);
      paVar3 = iVar8->field141_0x8e;
      ((int)paVar3 + 0x28) = uVar2;
      PostMessage16(0x0,0xc8,0x111,HWND16_1050_0396);
      param_2 = 0x1;
    }
    uVar12 = SUB42(param_1,0x0);
    BStack6 = post_win_msg_1040_7b3c(param_3,param_4,(param_4 >> 0x10),param_2);
    uStack4 = uVar12;
  }
LAB_1040_27c0:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Unable to use type for symbol uVar6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 draw_ui_op_1040_27cc(astruct_752 *param_1,HWND16 hwnd16_param_2,u16 param_3,HDC16 hdc_param_4)

{
  u32 uVar1;
  HBRUSH16 brush_handle_var8;
  i16 IVar3;
  astruct_752 *iVar3;
  u16 uVar3;
  u16 unaff_CS;
  u16 uVar7;
  u32 uVar4;
  HDC16 hdc;
  astruct_753 *iVar2;
  u16 uVar2;
  u16 uVar5;
  u16 uVar6;
  code **fn_ptr_1;

  uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_752 *)param_1;
  if (iVar3->brush_handle_field4_0x4 == 0x0) {
    uVar7 = SUB42(s_tile2_bmp_1050_1538,0x0);
    brush_handle_var8 = CreateSolidBrush16(WHITE_BRUSH);
    iVar3->brush_handle_field4_0x4 = brush_handle_var8;
  }
  if (_u16_1050_5cf8 == 0x0) {
    fn_ptr_1 = (code **)((int)(u32)param_1 + 0x68);
    uVar1 = (**fn_ptr_1)(uVar7,param_1,iVar3->field109_0x6e);
    uVar4 = pass1_1008_4d72(uVar1);
    uVar2 = (uVar4 >> 0x10);
    iVar2 = (astruct_753 *)uVar4;
    _u16_1050_5cf8 = CONCAT22(CONCAT11(0x2,iVar2->field_0x94),CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    uVar5 = 0x284a;
    IVar3 = GetDlgCtrlID16(hwnd16_param_2);
    if ((iVar3->field146_0x94 != 0x0) && (IVar3 == 0xfb2)) {
      uVar6 = 0xff;
      hdc = 0x0;
      goto LAB_1040_286e;
    }
  }
  uVar5 = _u16_1050_5cf8;
  uVar6 = ((u32)_u16_1050_5cf8 >> 0x10);
  hdc = hdc_param_4;
LAB_1040_286e:
  SetTextColor16(CONCAT22(uVar6,uVar5),hdc);
  SetBkColor16(0x1000000,hdc_param_4);
  return CONCAT22(0x1050,iVar3->brush_handle_field4_0x4);
}



void pass1_1040_288e(u32 param_1)

{
  u16 uVar1;
  code **ppcVar2;
  u32 uVar3;
  u32 uVar4;
  astruct_394 *paVar5;
  u32 *puVar6;
  u16 uVar7;
  u8 *puVar8;
  u32 in_EDX;
  Struct57*paVar9;
  i16 iVar10;
  i16 iVar11;
  u16 uVar12;
  u16 uVar13;

  uVar12 = (param_1 >> 0x10);
  iVar10 = (int)param_1;
  uVar3 = (u32)(iVar10 + 0x8e);
  uVar13 = ((u32)uVar3 >> 0x10);
  iVar11 = (int)uVar3;
  puVar6 = (u32 *)(u32)(iVar11 + 0x24);
  paVar9 = (Struct57*)(in_EDX & 0xffff0000 | (u32)(iVar11 + 0x26));
  ppcVar2 = (code **)((int)*puVar6 + 0x14);
  (**ppcVar2)();
  paVar5 = (astruct_394 *)puVar6;
  uVar4 = (long)paVar9 << 0x10;
  if (*(i32 *)(iVar10 + 0x70) != 0x0) {
    paVar5 = *(astruct_394 **)(iVar10 + 0x70);
    uVar1 = (iVar10 + 0x72);
    uVar7 = uVar1 | paVar5;
    paVar9 = (Struct57*)((u32)paVar9 & 0xffff0000 | (u32)uVar7);
    if (uVar7 != 0x0) {
      ppcVar2 = (code **)(u32)paVar5;
      (**ppcVar2)();
    }
  }
  mem_op_1000_179c(0x14,paVar9);
  puVar8 = (u8 *)(paVar9 | paVar5);
  if (puVar8 == NULL) {
    paVar5 = NULL;
    puVar8 = NULL;
  }
  else {
    struct_1008_4c58(paVar5);
  }
  *(astruct_394 **)(iVar10 + 0x70) = paVar5;
  *(u8 **)(iVar10 + 0x72) = puVar8;
  pass1_1008_4d84(puVar8,*(astruct_90 **)(iVar10 + 0x70),(u32)puVar6 & 0xffff | uVar4);
  return;
}



StructD * pass1_1040_2930(StructD *param_1,u8 param_2)

{
  pass1_1040_2464(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

Struct57* pas1_1040_29c2(u16 param_1,u16 param_2,Struct57*param_3,param_4: u32,u16 param_5)

{
  Struct57*iVar1;
  Struct57*uVar1;

  pass1_1040_b0bc(param_3,param_4,CONCAT22(param_5,0x157));
  uVar1 = (Struct57*)((u32)param_3 >> 0x10);
  iVar1 = (Struct57*)param_3;
  param_3->field0_0x0 = 0x2e26;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x64b);
  iVar1[0x1].field3_0x6 = param_1;
  iVar1[0x1].field4_0x8 = param_2;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x64a);
  iVar1[0x1].field5_0xa = param_1;
  iVar1[0x1].field6_0xc = param_2;
  return param_3;
}



void pass1_1040_2a22(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;
  u16 in_stack_0000ffd2;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x2e26;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x94);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x98);
  unk_draw_op_1040_b0f8(in_stack_0000ffd2,param_1);
  return;
}



void dlg_ui_op_1040_2a64(u16 param_1,StructB *struct_b_param_1)

{
  u32 uVar1;
  u16 in_AX;
  Struct57*paVar2;
  u16 uVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  Struct57*paVar5;
  StructB *struct_b_6;
  astruct_918 *iVar8;
  u16 uVar7;
  u16 in_stack_0000fe30;
  u16 in_stack_0000fe34;
  u16 in_stack_0000ff5a;
  u16 in_stack_0000ff5e;
  u16 in_stack_0000ff62;
  u16 in_stack_0000ffa2;
  i16 iVar9;
  RECT16 local_16;
  u16 uStack18;
  u16 uStack16;
  i16 iStack14;
  u32 uStack12;
  u32 uStack8;
  i16 iStack4;
  Struct57*paVar6;

  unk_win_ui_op_1040_b230(param_1,struct_b_param_1);
  iStack4 = 0x5;
  iVar9 = 0x0;
  uVar7 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_6 = (StructB *)struct_b_param_1;
  uVar1 = (u32)&struct_b_6[0x7].hwnd_0x6;
  uStack12 = struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x6),in_AX,param_1);
  paVar5 = (Struct57*)CONCAT22(in_register_0000000a,(int)(uStack12 >> 0x10));
  PTR_LOOP_1050_5d04 = (u8 *)pass1_1028_4a9a(uStack12,iVar9);
  for (iStack14 = 0x0; iStack14 < iStack4; iStack14 += 0x1) {
    if (iStack14 != 0x0) {
      ((int)&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0x0;
    }
    iVar9 = iStack14 * 0xc;
    local_16.x = *(INT16 *)(iVar9 + 0x5cfc);
    local_16.y = *(INT16 *)(iVar9 + 0x5cfe);
    paVar2 = (Struct57*)((int)&PTR_LOOP_1050_0000 + 0x1);
    uStack18 = 0x1;
    uStack16 = 0x1;
    MapDialogRect16(&local_16,(HWND16)0x1050);
    mem_op_1000_179c(0x42,paVar5);
    uVar4 = (Struct57*)paVar5 | paVar2;
    paVar6 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      paVar2 = NULL;
      paVar5 = (Struct57*)((u32)paVar5 & 0xffff0000);
    }
    else {
      pass1_1008_3bd6((u32)paVar6,paVar2,(Struct57*)paVar5,0x1,CONCAT22(local_16.x,local_16.y),0x101,0xff0100,
                      CONCAT22(struct_b_6->lpvoid_field_0x8,(iVar9 + 0x5d00)),in_stack_0000ffa2,
                      in_stack_0000fe30,in_stack_0000fe34,in_stack_0000ff5a,in_stack_0000ff5e,in_stack_0000ff62);
      paVar5 = paVar6;
    }
    uVar4 = paVar5;
    uStack8 = CONCAT22(uVar4,paVar2);
    if (PTR_LOOP_1050_5d04 == NULL) {
      if ((iStack14 != 0x0) && ((uVar4 | paVar2) != 0x0)) {
        EnableWindow16(0x0,*(HWND16 *)&paVar2->field11_0x18);
      }
    }
    else {
      iVar8 = (astruct_918 *)(iStack14 * 0xc);
      uVar3 = pass1_1028_4a9a(uStack12,(iVar8 + 0x5d02));
      if (uVar3 != 0x0) {
        (iVar8 + 0x5d04) = 0x1;
        SetDlgItemText16((u32)&struct_b_6[0x7].field6_0xc,*(INT16 *)(iVar8 + 0x5d06),
                         (HWND16)struct_b_6->lpvoid_field_0x8);
      }
    }
  }
  return;
}



void win_ui_op_1040_2bb2(u8 *param_1,astruct_903 *pstruct_903_param_2,u16 param_3,u32 param_4)

{
  u32 uVar1;
  u16 uVar2;
  HWND16 HVar3;
  astruct_922 *iVar4;
  i16 iVar5;
  astruct_920 *iVar3;
  u16 uVar6;
  u16 uVar7;
  u8 *id;
  i16 iStack8;
  i16 iStack4;

  if (param_4 == 0x158) {
    PTR_LOOP_1050_5d04 = (u8 *)(PTR_LOOP_1050_5d04 == NULL);
    if (PTR_LOOP_1050_5d04 == NULL) {
      for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
        iVar5 = iStack8 * 0xc;
        HVar3 = GetDlgItem16(*(INT16 *)(iVar5 + 0x5d00),*(HWND16 *)((int)pstruct_903_param_2 + 0x6));
        EnableWindow16(0x0,HVar3);
        ((int)&PTR_LOOP_1050_5d04 + iVar5) = 0x0;
        SetDlgItemText16((u32)((int)pstruct_903_param_2 + 0x94),
                         *(INT16 *)((int)&PTR_s_post_1050_015d_1050_5d06 + iVar5),
                         *(HWND16 *)((int)pstruct_903_param_2 + 0x6));
      }
      HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
      uVar1 = (u32)((int)pstruct_903_param_2 + 0x94);
      uVar6 = uVar1;
      uVar7 = ((u32)uVar1 >> 0x10);
      id = PTR_s_post_1050_015d_1050_5d06;
      goto LAB_1040_2ccc;
    }
    for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
      iVar3 = (astruct_920 *)(iStack8 * 0xc);
      HVar3 = GetDlgItem16(*(INT16 *)(iVar3 + 0x5d00),*(HWND16 *)((int)pstruct_903_param_2 + 0x6));
      EnableWindow16(0x1,HVar3);
      (iVar3 + 0x5d04) = 0x0;
      SetDlgItemText16((u32)((int)pstruct_903_param_2 + 0x94),*(INT16 *)(iVar3 + 0x5d06),
                       *(HWND16 *)((int)pstruct_903_param_2 + 0x6));
    }
    HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
    id = PTR_s_post_1050_015d_1050_5d06;
  }
  else {
    if (param_4 == 0x159) {
      iStack4 = 0x1;
    }
    else if (param_4 == 0x15a) {
      iStack4 = 0x2;
    }
    else if (param_4 == 0x15b) {
      iStack4 = 0x3;
    }
    else {
      if (param_4 != 0x15c) {
        pass1_1040_b54a(param_1,pstruct_903_param_2,param_3,param_4);
        return;
      }
      iStack4 = 0x4;
    }
    if (iStack4 == 0x0) {
      return;
    }
    iVar4 = (astruct_922 *)(iStack4 * 0xc);
    uVar2 = ((iVar4 + 0x5d04) == 0x0);
    (iVar4 + 0x5d04) = uVar2;
    if (uVar2 == 0x0) {
      HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
      uVar1 = (u32)((int)pstruct_903_param_2 + 0x94);
      uVar6 = uVar1;
      uVar7 = ((u32)uVar1 >> 0x10);
      id = *(u8 **)(iVar4 + 0x5d06);
      goto LAB_1040_2ccc;
    }
    HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
    id = (u8 *)((int)&PTR_s_post_1050_015d_1050_5d06 + iStack4 * 0xc);
  }
  uVar1 = (u32)((int)pstruct_903_param_2 + 0x98);
  uVar6 = uVar1;
  uVar7 = ((u32)uVar1 >> 0x10);
LAB_1040_2ccc:
  SetDlgItemText16(CONCAT22(uVar7,uVar6),(INT16)id,HVar3);
  return;
}



void win_dlg_item_1040_2d48(u32 param_1)

{
  u16 UVar1;
  u16 value;
  BOOL16 local_4;

  pass1_1040_b45e(param_1);
  UVar1 = GetDlgItemInt16(0x1,&local_4,(INT16)0x1050,0x163);
  value = GetDlgItemInt16(0x1,&local_4,(INT16)0x1050,0x165);
  if (UVar1 != 0x0) {
    value /= UVar1;
  }
  SetDlgItemInt16(0x1,value,0x165,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



void pass1_1040_2dac(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  u32 uVar2;
  i16 iStack10;

  uVar1 = (u32)((int)param_1 + 0x90);
  uVar2 = struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x6),in_AX,in_DX);
  for (iStack10 = 0x0; iStack10 < 0x5; iStack10 += 0x1) {
    pass1_1028_4ab2(uVar2,((int)&PTR_LOOP_1050_5d04 + iStack10 * 0xc),(iStack10 * 0xc + 0x5d02));
  }
  return;
}



StructD * pass1_1040_2e00(StructD *param_1,u8 param_2)

{
  pass1_1040_2a22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_2ea2(StructD *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x180,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  iVar1[0x1].field1_0x2 = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field4_0x8 = 0x0;
  param_2->field0_0x0 = 0x3436;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field4_0x8 = puVar2;
  iVar1[0x1].field5_0xa = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_2f06(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x3436;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_2f32(u16 param_1,u16 param_2,u16 param_3)

{
  u16 in_register_0000000a;
  Struct27 *paVar1;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  i16 iVar2;

  iVar2 = 0x0;
  paVar1 = (Struct27 *)
           mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}



void show_win_1040_2f5a(StructB *param_1)

{
  u16 uVar1;

  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  uVar1 = ((u32)param_1 >> 0x10);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  SetFocus16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_dlg_op_1040_2f90(u16 param_1,u32 param_2)

{
  u16 uVar1;
  HWND16 HVar2;
  u16 in_register_0000000a;
  Struct57*paVar3;
  astruct_943 *iVar4;
  u16 uVar4;
  u32 *puVar5;
  u32 uVar6;
  char *l_param;
  u16 in_stack_0000fd7a;
  u16 in_stack_0000fd7c;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000fea0;
  u16 in_stack_0000fea4;
  u16 in_stack_0000fea6;
  u16 in_stack_0000fea8;
  u16 in_stack_0000feaa;
  u16 in_stack_0000fed2;
  u16 in_stack_0000fed4;
  u32 *local_116;
  u32 *local_112;
  WORD local_10e [0x41];
  u8 local_8c [0x82];
  u32 uStack10;
  u32 *puStack6;

  paVar3 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed2,0x2),in_stack_0000fd7a,
                             in_stack_0000fe9e,in_stack_0000fea4,in_stack_0000fea8);
  paVar3 = (Struct57*)((u32)paVar3 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x68);
  uVar4 = (param_2 >> 0x10);
  iVar4 = (astruct_943 *)param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),iVar4->field6_0x6);
  wsprintf16(local_10e,(char *)CONCAT22(local_8c,0x1050),(char *)CONCAT22((int)uStack10,0x1050),
             (int)((u32)uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),iVar4->field6_0x6);
  HVar2 = GetDlgItem16(0x182,iVar4->field6_0x6);
  iVar4->field143_0x92 = HVar2;
  pass1_1018_3a94(iVar4->field145_0x96,(u32 *)CONCAT22(0x1050,&local_116),(u32 *)CONCAT22(0x1050,&local_112));
  send_msg_1040_3374(param_2,local_112,iVar4->field143_0x92);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed4,0x2f),in_stack_0000fd7c,
                           in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
  uVar1 = ((u32)puVar5 >> 0x10);
  uVar6 = (u32)((int)puVar5 + 0x24);
  uVar6 = pass1_1018_3a7a(uVar6,uVar1,iVar4->field145_0x96,uVar6);
  SendMessage16(uVar6,0xffff,0x40d,iVar4->field143_0x92);
  HVar2 = GetDlgItem16(0x183,iVar4->field6_0x6);
  iVar4->field144_0x94 = HVar2;
  send_msg_1040_3374(param_2,local_116,HVar2);
  l_param = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendDlgItemMessage16((LPARAM)l_param,0x0,0x403,0x183,iVar4->field6_0x6);
  SendDlgItemMessage16((LPARAM)l_param,0xffff,0x40d,0x183,iVar4->field6_0x6);
  HVar2 = GetDlgItem16(0x181,iVar4->field6_0x6);
  iVar4->field141_0x8e = HVar2;
  HVar2 = GetDlgItem16(0x184,iVar4->field6_0x6);
  iVar4->field142_0x90 = HVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_311a(i16 param_1,u16 param_2,u16 param_3,u32 param_4)

{
  i16 iVar1;
  u32 uVar2;
  u32 in_EDX;
  u16 uVar4;
  u32 uVar3;
  LRESULT LVar5;
  Struct27 *paVar6;
  u16 in_stack_0000fe8e;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbc;
  i16 iVar7;

  uVar4 = ((u32)in_EDX >> 0x10);
  send_msg_1040_323c(CONCAT22(param_2,param_1));
  load_string_1010_847e(_u16_1050_14cc,0x531);
  if (param_4 == 0x181) {
    uVar3 = CONCAT22(uVar4,param_2);
    iVar1 = param_1 + 0x9a;
    iVar7 = iVar1;
    pass1_1018_3cda(*(astruct_506 **)(param_1 + 0x96),(char *)CONCAT22(param_2,param_1 + 0x19a),
                    (char *)CONCAT22(param_2,iVar1));
    pass1_1018_3424(iVar7,uVar3,(u32)(param_1 + 0x96));
    if (iVar7 == 0x0) {
      iVar7 = 0x21;
    }
    else {
      pass1_1018_3a42((int)uVar3,(u32)(param_1 + 0x96),CONCAT22(param_2,iVar1));
      pass1_1030_8344(_u16_1050_5748,CONCAT22((int)uVar3,iVar7));
      uVar2 = (u32)(iVar7 + 0x10);
      pass1_1030_8344(_u16_1050_5748,uVar2);
      PTR_LOOP_1050_5f0c = (u8 *)uVar2;
      PTR_LOOP_1050_5f0e = (u8 *)uVar3;
      PTR_LOOP_1050_5f10 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
      iVar7 = 0x25;
    }
    pass1_1038_af40(param_1,uVar3,_PTR_LOOP_1050_5b7c,(param_1 + 0x8),iVar7);
    uVar4 = ((u32)uVar3 >> 0x10);
    LVar5 = SendMessage16(0x0,0x2,0x111,*(HWND16 *)(param_1 + 0x6));
    iVar7 = 0x1;
    paVar6 = (Struct27 *)
             mixed_1010_20ba((Struct57*)CONCAT22(uVar4,(int)((u32)LVar5 >> 0x10)),_u16_1050_0ed0,
                             (u8 **)0x1002b,in_stack_0000fe8e,in_stack_0000ffb2,in_stack_0000ffb8,
                             in_stack_0000ffbc);
    pass1_1010_038e(paVar6,iVar7);
  }
  else {
    if ((param_4 == 0x181) || (0x1 < param_4 - 0x182U)) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_2,param_1),param_3,param_4,param_4);
      return;
    }
    set_win_pos_1040_331a(CONCAT22(param_2,param_1),param_3,param_4);
  }
  return;
}



LRESULT send_msg_1040_323c(u32 param_1)

{
  WPARAM16 wparam;
  i16 iVar1;
  u16 uVar2;
  LRESULT LVar3;
  WPARAM16 wparam_00;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,*(HWND16 *)(iVar1 + 0x92));
  wparam = (WPARAM16)LVar3;
  SendMessage16(0x0,0x0,0x407,*(HWND16 *)(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (u32)(iVar1 + 0x9a),wparam,0x408,*(HWND16 *)(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (u32)(iVar1 + 0x19a),wparam_00,0x408,*(HWND16 *)(iVar1 + 0x94));
  return LVar3;
}



void enable_win_1040_32a8(u32 param_1)

{
  u16 uVar1;
  BOOL16 BVar1;
  u16 uVar2;
  u32 uStack12;

  uVar1 = (int)param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | (u32)uVar1;
  uVar2 = param_1;
  pass1_1018_3a5c((u32)((int)param_1 + 0x96),(char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x9aU)),
                  (char *)(param_1 & 0xffff0000 | (u32)uVar1));
  SetWindowText16(CONCAT22(uVar2,uVar1),*(HWND16 *)((int)param_1 + 0x90));
  BVar1 = string_1018_39d8((u32)((int)param_1 + 0x96),
                           (char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x9aU)),(char *)uStack12);
  EnableWindow16(BVar1 & 0x1,*(HWND16 *)((int)param_1 + 0x8e));
  return;
}



BOOL16 set_win_pos_1040_331a(param_1: u32,u16 param_2,param_3: i16)

{
  i16 iStack10;

  if (param_3 == 0x1) {
    enable_win_1040_32a8(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff2),param_2);
    SetWindowPos16(0x2,0x50,iStack10 - param_2,0x0,0x0,0x0,param_2);
  }
  return 0x1;
}



void send_msg_1040_3374(param_1: u32,u32 *param_2,u16 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u32 uVar3;
  u16 DX_REG;
  u16 DX_REG_00;
  u16 uVar4;
  LRESULT LVar5;
  char *lparam;
  u32 uStack10;
  u32 uStack6;

  uVar4 = SUB42(s_tile2_bmp_1050_1538,0x0);
  LVar5 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar2 = LVar5;
  ppcVar1 = (code **)((int)*param_2 + 0x10);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_2);
  uStack6 = CONCAT22(DX_REG,uVar2);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)(uVar4,param_2,(char)uStack10,(int)(uStack10 >> 0x10));
    lparam = (char *)pass1_1018_3a7a(uVar3,DX_REG_00,(u32)((int)param_1 + 0x96),
                                     CONCAT13((char)(DX_REG_00 >> 0x8),CONCAT12((char)DX_REG_00,uVar3)))
    ;
    LVar5 = SendMessage16((LPARAM)lparam,0x0,0x403,param_3);
    uVar4 = 0x1000;
    fn_ptr_1000_17ce(lparam);
    if (LVar5 == -0x1) break;
    if (LVar5 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}



StructD * pass1_1040_3410(StructD *param_1,u8 param_2)

{
  pass1_1040_2f06(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_34a2(StructD *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x192,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  iVar1[0x1].field4_0x8 = 0x0;
  iVar1[0x1].field5_0xa = 0x0;
  param_2->field0_0x0 = (int)s_Null_Ptr_1050_38f3 + 0x7;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_3506(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = (int)s_Null_Ptr_1050_38f3 + 0x7;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_3532(u8 *param_1,u16 param_2,u16 param_3)

{
  u16 in_register_0000000a;
  Struct27 *paVar1;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  i16 iVar2;

  iVar2 = 0x0;
  paVar1 = (Struct27 *)
           mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}



void show_win_1040_355a(StructB *param_1)

{
  u16 uVar1;

  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  uVar1 = ((u32)param_1 >> 0x10);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  SetFocus16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void set_win_text_1040_3590(u16 param_1,astruct_923 *param_2)

{
  HWND16 HVar1;
  HWND16 HVar2;
  u16 uVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_923 *iVar5;
  u16 uVar5;
  u16 in_stack_0000f8f8;
  u16 in_stack_0000fa1c;
  u16 in_stack_0000fa22;
  u16 in_stack_0000fa26;
  u8 uVar6;
  u16 in_stack_0000fa50;
  u32 local_59a;
  u32 local_596;
  BOOL16 BStack1426;
  u16 uStack1424;
  WORD local_58e [0x41];
  WORD local_50c [0x80];
  u32 uStack1036;
  u32 *puStack1032;
  char local_404 [0x402];

  puStack1032 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                                (u8 **)CONCAT22(in_stack_0000fa50,0x2),in_stack_0000f8f8,in_stack_0000fa1c,
                                in_stack_0000fa22,in_stack_0000fa26);
  uVar4 = ((u32)puStack1032 >> 0x10);
  uStack1036 = (u32)((int)puStack1032 + 0x68);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar5 = (astruct_923 *)param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_50c),iVar5->field6_0x6);
  uVar6 = SUB21(local_50c,0x0);
  wsprintf16(local_58e,(char *)CONCAT13((char)(local_50c >> 0x8),CONCAT12(uVar6,0x1050)),uVar6,
             CONCAT22((int)uStack1036,0x1050),(int)((u32)uStack1036 >> 0x10));
  BStack1426 = SetWindowText16(CONCAT22(0x1050,local_58e),iVar5->field6_0x6);
  sprintf_op_1018_34b6((u8)BStack1426,uVar4,(astruct_263 *)iVar5->field141_0x8e);
  uStack1424 = uVar4;
  pass1_1018_3d44(iVar5->field141_0x8e,(u32 *)CONCAT22(0x1050,&local_59a),(u32 *)CONCAT22(0x1050,&local_596));
  HVar1 = GetDlgItem16(0x193,iVar5->field6_0x6);
  iVar5->field148_0x98 = HVar1;
  EnableWindow16(0x1,HVar1);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_404,(short)0x1050);
  wsprintf16(local_50c,0x50,(char *)CONCAT22(local_404,0x1050),CONCAT22((int)local_596,0x1050),
             (int)((u32)local_596 >> 0x10));
  HVar1 = GetDlgItem16(0x195,iVar5->field6_0x6);
  SetWindowText16(CONCAT22(0x1050,local_50c),HVar1);
  HVar2 = GetDlgItem16(0x196,iVar5->field6_0x6);
  HVar1 = HVar2;
  sprintf_op_1018_34b6((u8)HVar2,uVar4,(astruct_263 *)iVar5->field141_0x8e);
  SetWindowText16(CONCAT22(uVar4,HVar2),HVar1);
  HVar1 = GetDlgItem16(0x197,iVar5->field6_0x6);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_404,(short)0x1050);
  SetWindowText16(CONCAT22(0x1050,local_404),HVar1);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_404,(short)0x1050);
  wsprintf16(local_50c,(char *)CONCAT22(local_404,0x1050),(char *)CONCAT22((int)local_59a,0x1050),
             (int)((u32)local_59a >> 0x10));
  HVar1 = GetDlgItem16(0x198,iVar5->field6_0x6);
  SetWindowText16(CONCAT22(0x1050,local_50c),HVar1);
  uVar3 = GetDlgItem16(0x199,iVar5->field6_0x6);
  HVar1 = uVar3;
  unk_str_op_1018_35b0(uVar3,(astruct_263 *)iVar5->field141_0x8e);
  if ((uVar4 | uVar3) == 0x0) {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_404,(short)0x1050);
    SetWindowText16(CONCAT22(0x1050,local_404),HVar1);
    GetDlgItem16(0x19a,iVar5->field6_0x6);
    HVar1 = (HWND16)_u16_1050_14cc;
    load_string_1010_84e0(HVar1,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_404,(short)0x1050);
    SetWindowText16(CONCAT22(0x1050,local_404),HVar1);
    EnableWindow16(0x0,iVar5->field148_0x98);
    return;
  }
  SetWindowText16(CONCAT22(uVar4,uVar3),HVar1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void message_box_op_1040_37f0(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  u16 uVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  Struct57*paVar3;
  u16 uVar4;
  LRESULT LVar5;
  u16 in_stack_0000fa94;
  u16 in_stack_0000fbb8;
  u16 in_stack_0000fbbe;
  u16 in_stack_0000fbc2;
  u16 in_stack_0000fbec;
  i16 iVar6;
  char local_40c [0x402];
  char *pcStack10;
  Struct27 *paStack6;

  paVar3 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0x193) {
    paStack6 = (Struct27 *)
               mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fbec,0x2),in_stack_0000fa94,
                               in_stack_0000fbb8,in_stack_0000fbbe,in_stack_0000fbc2);
    uVar2 = ((u32)paStack6 >> 0x10);
    pcStack10 = *(char **)((int)paStack6 + 0x68);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_40c,(short)0x1050);
    uVar1 = MessageBox16(0x30,pcStack10,(char *)CONCAT22(0x1050,local_40c),*(HWND16 *)(param_2 + 0x6));
    pass1_1018_3710(uVar1,uVar2,*(astruct_263 **)(param_2 + 0x8e));
    PostMessage16(0x0,0x2,0x111,*(HWND16 *)(param_2 + 0x6));
  }
  else {
    if (param_5 != 0x194) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),0x21);
    uVar4 = ((u32)paVar3 >> 0x10);
    LVar5 = SendMessage16(0x0,0x2,0x111,*(HWND16 *)(param_2 + 0x6));
    iVar6 = 0x1;
    paStack6 = (Struct27 *)
               mixed_1010_20ba((Struct57*)CONCAT22(uVar4,(int)((u32)LVar5 >> 0x10)),_u16_1050_0ed0,
                               (u8 **)0x1002b,in_stack_0000fa94,in_stack_0000fbb8,in_stack_0000fbbe,
                               in_stack_0000fbc2);
    pass1_1010_038e(paStack6,iVar6);
  }
  return;
}



StructD * pass1_1040_38d4(StructD *param_1,u8 param_2)

{
  pass1_1040_3506(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_3966(u8 *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x185,param_6);
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  (u32)&iVar1[0x1].field2_0x4 = 0x0;
  (u32)&iVar1[0x1].field4_0x8 = 0x0;
  iVar1[0x1].field6_0xc = 0x0;
  iVar1[0x1].field7_0xe = 0x0;
  iVar1[0x1].field8_0x10 = 0x0;
  iVar1[0x1].field9_0x12 = 0x0;
  &iVar1[0x1].field10_0x14 = 0x0;
  ((int)&iVar1[0x1].field10_0x14 + 0x2) = 0x5;
  param_2->field0_0x0 = 0x3ffc;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_39e2(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x3ffc;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_3a0e(u8 *param_1,u16 param_2,u16 param_3)

{
  u16 in_register_0000000a;
  Struct27 *paVar1;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  i16 iVar2;

  iVar2 = 0x0;
  paVar1 = (Struct27 *)
           mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}



u16 enable_win_1040_3a36(astruct_924 *param_1,u16 param_2,u16 param_3,param_4: i16)

{
  u16 *puVar1;
  bool bVar2;
  astruct_924 *iVar3;
  u16 uVar3;

  bVar2 = false;
  iVar3 = (astruct_924 *)param_1;
  uVar3 = ((u32)param_1 >> 0x10);
  if (param_4 == 0x0) {
    if (iVar3->field155_0x9e <= iVar3->field154_0x9c) goto LAB_1040_3a79;
    puVar1 = &iVar3->field154_0x9c;
    *puVar1 = *puVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_3a79;
    if (iVar3->field154_0x9c == 0x0) goto LAB_1040_3a79;
    puVar1 = &iVar3->field154_0x9c;
    *puVar1 = *puVar1 - 0x1;
  }
  bVar2 = true;
LAB_1040_3a79:
  if (bVar2) {
    SetDlgItemInt16(0x0,iVar3->field154_0x9c,0x18e,iVar3->field6_0x6);
  }
  if ((iVar3->field154_0x9c != 0x0) && (iVar3->field158_0xa2 == 0x0)) {
    iVar3->field158_0xa2 = 0x1;
    EnableWindow16(0x1,iVar3->field153_0x9a);
  }
  if ((iVar3->field154_0x9c == 0x0) && (iVar3->field158_0xa2 != 0x0)) {
    iVar3->field158_0xa2 = 0x0;
    EnableWindow16(0x0,iVar3->field153_0x9a);
  }
  return 0x0;
}



void show_win_1040_3ae8(StructB *param_1)

{
  u16 uVar1;

  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  uVar1 = ((u32)param_1 >> 0x10);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  SetFocus16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_3b1e(u16 param_1,StructC *struct_c_param_1)

{
  u32 uVar1;
  BOOL16 BVar2;
  HWND16 HVar3;
  StructC *pSVar4;
  u16 in_register_0000000a;
  StructC *struct_c_4;
  u16 unaff_SI;
  StructC *struct_c_param_2;
  u32 uVar5;
  u16 in_stack_0000fd8a;
  u16 in_stack_0000feae;
  u16 in_stack_0000feb4;
  u16 in_stack_0000feb8;
  u32 *puStack282;
  WORD local_10e [0x41];
  WORD local_8c [0x41];
  u32 uStack10;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd8a,in_stack_0000feae,in_stack_0000feb4,
                             in_stack_0000feb8);
  uStack10 = (u32)((int)puStack6 + 0x68);
  struct_c_param_2 = (StructC *)((u32)struct_c_param_1 >> 0x10);
  struct_c_4 = (StructC *)struct_c_param_1;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),struct_c_4->field6_0x6);
  wsprintf16(local_10e,(char *)CONCAT22(local_8c,0x1050),(char *)CONCAT22((int)uStack10,0x1050),
             (int)((u32)uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),struct_c_4->field6_0x6);
  puStack282 = (u32 *)((u32)struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4->field_0x96));
  pSVar4 = struct_c_param_2;
  pass1_1018_3d44((u32)struct_c_4->field141_0x8e,
                  (u32 *)((u32)struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4->field142_0x92)),
                  (u32 *)((u32)struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4->field_0x96)));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x80,(char *)local_10e,(short)0x1050
            );
  uVar1 = struct_c_4->field142_0x92;
  wsprintf16(local_8c,(char *)CONCAT22(local_10e,0x1050),(char *)CONCAT22((int)*puStack282,0x1050),
             (int)((u32)*puStack282 >> 0x10),(int)uVar1,(int)(uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_8c),0x187,struct_c_4->field6_0x6);
  BVar2 = CheckRadioButton16(0x188,0x18d,0x188,struct_c_4->field6_0x6);
  struct_c_4->field149_0xa0 = 0x188;
  uVar5 = switch_1018_3b9e(BVar2,pSVar4,struct_c_4->field141_0x8e,struct_c_4->field149_0xa0);
  send_dlg_item_msg_1040_3f12(struct_c_4,struct_c_param_2,uVar5);
  dialog_item_ui_op_1040_3e08(struct_c_param_1);
  HVar3 = GetDlgItem16(0x186,struct_c_4->field6_0x6);
  struct_c_4->field146_0x9a = HVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1040_3c64
               (u16 param_1,StructC *struct_c_param_1,StructC *struct_c_param_2,u16 param_4,u32 param_5)

{
  u16 UVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 uVar3;
  LRESULT LVar4;
  Struct27 *paVar5;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  i16 iVar6;

  if (param_5 == 0x186) {
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x409,0x190,struct_c_param_1->field6_0x6);
    uVar2 = ((u32)LVar4 >> 0x10);
    UVar1 = GetDlgItemInt16(0x0,NULL,0x0,0x18e);
    pass1_1018_36e6((u32)struct_c_param_1->field141_0x8e,UVar1,LVar4,struct_c_param_1->field149_0xa0);
    pass1_1038_af40(struct_c_param_1,uVar2,_PTR_LOOP_1050_5b7c,&struct_c_param_1->field_0x8,0x22);
    LVar4 = SendMessage16(0x0,0x2,0x111,struct_c_param_1->field6_0x6);
    iVar6 = 0x1;
    paVar5 = (Struct27 *)
             mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,(int)((u32)LVar4 >> 0x10)),_u16_1050_0ed0,
                             (u8 **)0x1002b,in_stack_0000fe9a,in_stack_0000ffbe,in_stack_0000ffc4,
                             in_stack_0000ffc8);
    pass1_1010_038e(paVar5,iVar6);
  }
  else {
    if (param_5 - 0x186 < 0x2) {
LAB_1040_3c7f:
      post_win_msg_1040_7b3c
                ((StructC *)CONCAT22(struct_c_param_2,struct_c_param_1),param_4,param_5,param_5);
      return;
    }
    if (param_5 - 0x188 < 0x5 || param_5 == 0x18d) {
      struct_c_param_1->field149_0xa0 = param_5;
      uVar3 = switch_1018_3b9e(param_5,param_1,struct_c_param_1->field141_0x8e,param_5);
      send_dlg_item_msg_1040_3f12(struct_c_param_1,struct_c_param_2,uVar3);
    }
    else {
      if (param_5 - 0x188 != 0x8) goto LAB_1040_3c7f;
      if (param_5 != 0x1) {
        return;
      }
    }
    dialog_item_ui_op_1040_3e08((StructC *)CONCAT22(struct_c_param_2,struct_c_param_1));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 get_dc_op_1040_3d5e(astruct_1 *param_1)

{
  code **ppcVar1;
  u16 uVar2;
  u32 in_EDX;
  astruct_1 *iVar4;
  astruct_934 *iVar3;
  u16 uVar3;
  u32 *puStack8;
  HDC16 local_4;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_1 *)param_1;
  local_4 = GetDC16(iVar4->field6_0x6);
  uVar2 = FUN_1010_830a(local_4,in_EDX,(int)s_tile2_bmp_1050_1538,_u16_1050_14cc,iVar4->field163_0xa4);
  puStack8 = (u32 *)CONCAT22((int)in_EDX,uVar2);
  iVar3 = (astruct_934 *)*puStack8;
  ppcVar1 = (code **)&iVar3->field6_0x8;
  (**ppcVar1)(0x1010,uVar2,(int)in_EDX,&local_4,(int)0x1050);
  ppcVar1 = (code **)&iVar3->field4_0x4;
  (**ppcVar1)(0x1010,puStack8,0x50078,&local_4,(int)0x1050);
  ppcVar1 = (code **)&iVar3->field8_0xc;
  (**ppcVar1)(0x1010,puStack8,&local_4,(int)0x1050);
  ReleaseDC16(local_4,iVar4->field6_0x6);
  return 0x0;
}



void invalidate_rect_1040_3ddc(StructC *in_struct_1)

{
  RECT16 rect;
  u32 uStack6;

  rect = (RECT16)0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(0x0,&rect,(HWND16)0x1050);
  return;
}



void dialog_item_ui_op_1040_3e08(StructC *struct_c_param_1)

{
  u16 uVar1;
  StructC *struct_c_1;
  u16 var3;
  LRESULT LVar2;

  var3 = ((u32)struct_c_param_1 >> 0x10);
  struct_c_1 = (StructC *)struct_c_param_1;
  CheckRadioButton16(struct_c_1->field149_0xa0,0x18d,0x188,struct_c_1->field6_0x6);
  struct_c_1->field147_0x9c = 0x0;
  struct_c_1->field148_0x9e = 0x0;
  LVar2 = SendDlgItemMessage16(0x0,0x0,0x409,0x190,struct_c_1->field6_0x6);
  if (LVar2 != -0x1) {
    uVar1 = pass1_1018_3ab2((u32)struct_c_1->field141_0x8e,(int)LVar2,struct_c_1->field149_0xa0);
    struct_c_1->field148_0x9e = uVar1;
  }
  SetDlgItemInt16(0x0,struct_c_1->field147_0x9c,0x18e,struct_c_1->field6_0x6);
  SetDlgItemInt16(0x0,struct_c_1->field148_0x9e,0x191,struct_c_1->field6_0x6);
  switch(struct_c_1->field149_0xa0) {
  case 0x188:
    struct_c_1->field152_0xa4 = 0x5;
    break;
  case 0x189:
    struct_c_1->field152_0xa4 = 0x6;
    break;
  case 0x18a:
    struct_c_1->field152_0xa4 = 0x7;
    break;
  case 0x18b:
    struct_c_1->field152_0xa4 = 0x8;
    break;
  case 0x18c:
    struct_c_1->field152_0xa4 = 0x9;
    break;
  case 0x18d:
    struct_c_1->field152_0xa4 = 0xa;
  }
  invalidate_rect_1040_3ddc(struct_c_param_1);
  return;
}



void send_dlg_item_msg_1040_3f12(StructC *struct_c_param_1,StructC *struct_c_param_2,u32 param_3)

{
  u8 *puVar1;
  u16 DX_REG;
  i16 iVar2;
  LRESULT LVar3;
  u8 local_a [0x8];

  SendDlgItemMessage16(0x0,0x0,0xb,0x190,struct_c_param_1->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x190,struct_c_param_1->field6_0x6);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),param_3);
  while( true ) {
    puVar1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar1));
    if ((DX_REG | puVar1) == 0x0) break;
    LVar3 = SendDlgItemMessage16(*(LPARAM *)(puVar1 + 0x4),0x0,0x401,0x190,struct_c_param_1->field6_0x6);
    iVar2 = (int)((u32)LVar3 >> 0x10);
    if ((((int)LVar3 == -0x1) && (iVar2 == -0x1)) || (((int)LVar3 == -0x2 && (iVar2 == -0x1)))) break;
  }
  SendDlgItemMessage16(0x0,0x0,0x407,0x190,struct_c_param_1->field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x190,struct_c_param_1->field6_0x6);
  return;
}



StructD * pass1_1040_3fd6(StructD *param_1,u8 param_2)

{
  pass1_1040_39e2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_4068(u8 *param_1,Struct57*param_2,param_3: u32,u16 param_4,u16 param_5,u16 param_6)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar2;
  u16 unaff_BP;
  u16 uVar2;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;
  u8 **ppuVar4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb7,param_6);
  uVar2 = ((u32)param_2 >> 0x10);
  iVar2 = (Struct57*)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  iVar2[0x1].field6_0xc = 0x0;
  param_2->field0_0x0 = 0x4466;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2->field87_0x76 = 0x1;
  ppuVar4 = (u8 **)CONCAT22(unaff_BP,0x2);
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar4,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 0x1)->field0_0x0 = puVar3;
  iVar2[0x1].field1_0x2 = ((u32)puVar3 >> 0x10);
  puVar3 = mixed_1010_20ba((Struct57*)((u32)paVar1 & 0xffff0000 | (u32)puVar3 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)ppuVar4 >> 0x10),0x29),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field4_0x8 = puVar3;
  iVar2[0x1].field5_0xa = ((u32)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_40e2(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x4466;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_410e(undefined1 param_1,StructB *struct_b_param_1)

{
  u32 uVar1;
  u32 in_EDX;
  Struct57*paVar2;
  StructB *struct_b_3;
  u16 uVar3;
  u16 *puVar4;
  u32 *puVar5;
  u16 in_stack_0000fe52;
  u16 in_stack_0000ff76;
  u16 in_stack_0000ff7c;
  u16 in_stack_0000ff80;
  i16 *piVar6;
  u16 uVar7;
  char *pcVar8;
  i16 local_36;
  i16 local_34;
  i16 local_32;
  u8 local_30 [0x6];
  i16 local_2a [0x4];
  u32 uStack34;
  u32 local_1e;
  u32 uStack26;
  i16 local_16;
  i16 iStack20;
  i16 iStack18;
  i16 iStack16;
  HWND16 HStack14;
  u8 local_c [0xa];

  uVar7 = ((u32)in_EDX >> 0x10);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  pass1_1000_4906((StructD *)CONCAT22(0x1050,local_c),NULL,0xa);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_3 = (StructB *)struct_b_param_1;
  uVar1 = (u32)&struct_b_3[0x7].field1_0x2;
  sys_1000_3f9c((char *)CONCAT22(0x1050,local_c),s__lu_1050_5d38,(u32)((int)uVar1 + 0x76));
  HStack14 = GetDlgItem16(0xfb5,(HWND16)struct_b_3->lpvoid_field_0x8);
  SendMessage16(CONCAT22(0x1050,local_c),0x0,0xc,HStack14);
  SetFocus16(HStack14);
  SendMessage16(-0x10000,0x0,0x401,HStack14);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_16),(HWND16)struct_b_3->lpvoid_field_0x8);
  pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_1e),NULL,0x8);
  uVar1 = (u32)&struct_b_3[0x7].field1_0x2;
  uStack34 = (u32 *)pass1_1010_5f7a((int)uVar1,((u32)uVar1 >> 0x10),0x0,0x7);
  if (uStack34 != NULL) {
    local_1e = *uStack34;
    uStack26 = (u32)((int)uStack34 + 0x4);
  }
  if ((local_1e == 0x0) && ((int)local_1e == 0x0)) {
    puVar4 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_30));
    paVar2 = (Struct57*)CONCAT22(uVar7,(int)((u32)puVar4 >> 0x10));
    uVar1 = (u32)&struct_b_3[0x7].field5_0xa;
    pass1_1018_2678(uVar1,((u32)uVar1 >> 0x10),(u16 *)CONCAT22(0x1050,local_30));
    pass1_1008_3e94((u16 *)CONCAT22(0x1050,local_30),(u16 *)CONCAT22(0x1050,&local_32),(char *)CONCAT22(0x1050,local_2a)
                   );
    pcVar8 = (char *)CONCAT22(0x1050,&local_34);
    piVar6 = &local_36;
    uVar7 = SUB42(0x1050,0x0);
    puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(piVar6,0x48),in_stack_0000fe52,
                             in_stack_0000ff76,in_stack_0000ff7c,in_stack_0000ff80);
    pass1_1008_3e94((u16 *)((u32)puVar5 & 0xffff0000 | (u32)((int)puVar5 + 0xe)),(u16 *)CONCAT22(uVar7,piVar6),
                    pcVar8);
    uStack26 = CONCAT22(iStack16 - iStack20,iStack18 - local_16);
    local_1e = CONCAT22(((((int)puVar5 + 0xc) * -0x14) / 0x258 - (iStack16 - iStack20)) + local_36 + local_32,
                        local_34 + local_2a[0]);
  }
  move_win_1040_826c(struct_b_param_1,local_1e,(BOOL16)local_1e);
  ShowWindow16(0x5,(HWND16)struct_b_3->lpvoid_field_0x8);
  return;
}



void win_ui_op_1040_42b2(param_1: u32,param_2: i16)

{
  u32 uVar1;
  HWND16 HVar2;
  u16 uVar3;
  u16 uVar4;
  astruct_893 *iVar5;
  u16 uVar5;
  LRESULT LVar6;
  WORD local_54 [0x29];

  iVar5 = (astruct_893 *)param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    iVar5->field147_0x9a = 0x1;
    DestroyWindow16(iVar5->field6_0x6);
    return;
  }
  pass1_1000_4906((StructD *)CONCAT22(0x1050,local_54),NULL,0x51);
  HVar2 = GetDlgItem16(0xfb5,iVar5->field6_0x6);
  LVar6 = SendMessage16(CONCAT22(0x1050,local_54),0x51,0xd,HVar2);
  uVar4 = ((u32)LVar6 >> 0x10);
  uVar3 = pass1_1000_3e2c(CONCAT22(0x1050,local_54));
  if ((uVar4 | uVar3) != 0x0) {
    &iVar5->field142_0x92 = uVar3;
    ((int)&iVar5->field142_0x92 + 0x2) = uVar4;
  }
  if ((int)uVar4 < 0x0) {
    uVar1 = iVar5->field141_0x8e;
    uVar1 = (u32)((int)uVar1 + 0x76);
    wsprintf16(local_54,(char *)0x5d3c1050,(char *)CONCAT22((int)uVar1,0x1050),(int)((u32)uVar1 >> 0x10));
    SendMessage16(CONCAT22(0x1050,local_54),0x0,0xc,HVar2);
    SetFocus16(HVar2);
    SendMessage16(-0x10000,0x0,0x401,HVar2);
    return;
  }
  HVar2 = GetDlgItem16(0x1,iVar5->field6_0x6);
  EnableWindow16(0x0,HVar2);
  uVar1 = iVar5->field141_0x8e;
  *(LPARAM *)((int)uVar1 + 0x76) = iVar5->field142_0x92;
  PostMessage16(iVar5->field142_0x92,0x0,0x400,HWND16_1050_0396);
  HVar2 = GetDlgItem16(0x1,iVar5->field6_0x6);
  EnableWindow16(0x1,HVar2);
  return;
}



void get_win_rect_1040_43ea(i16 param_1,u16 param_2)

{
  u32 uVar1;
  u32 local_a;
  i16 iStack6;
  i16 iStack4;

  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_a),*(HWND16 *)(param_1 + 0x6));
  iStack6 -= (int)local_a;
  iStack4 -= local_a;
  pass1_1010_5fb0((u32)(param_1 + 0x8e),0x0,&local_a,0x1050,0x7);
  uVar1 = (u32)(param_1 + 0x8e);
  ((int)uVar1 + 0x7a) = ((param_1 + 0x9a) == 0x0);
  return;
}



StructD * pass1_1040_4440(StructD *param_1,u8 param_2)

{
  pass1_1040_40e2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2

void pass1_1040_44d2(u16 param_1,u8 *param_2,Struct57*param_3,param_4: u32,u16 param_5)

{
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  Struct57*iVar6;
  i16 iVar7;
  Struct57*uVar8;
  u16 uVar9;
  i16 *piStack8;
  u32 uVar1;
  u32 uVar2;
  Struct57*paVar6;

  struct_1040_b082(param_3,CONCAT22(param_5,0xfa2));
  uVar8 = (Struct57*)((u32)param_3 >> 0x10);
  iVar6 = (Struct57*)param_3;
  param_3->field0_0x0 = 0x4824;
  iVar6->field1_0x2 = &PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,(Struct57*)param_2);
  uVar4 = param_2 | param_1;
  paVar6 = (Struct57*)((u32)param_2 & 0xffff0000 | (u32)uVar4);
  if (uVar4 == 0x0) {
    (u32)&iVar6[0x1].field1_0x2 = 0x0;
  }
  else {
    struct_1040_a598((astruct_259 *)CONCAT22(param_2,param_1));
    iVar6[0x1].field1_0x2 = param_1;
    iVar6[0x1].field2_0x4 = paVar6;
  }
  *(u16*)&iVar6[0x1].field1_0x2 = 0x14;
  iVar7 = **(i16 **)&iVar6[0x1].field1_0x2;
  uVar4 = iVar7 * 0xa + 0x2;
  mem_op_1000_179c(uVar4,paVar6);
  uVar5 = paVar6;
  piStack8 = (i16 *)CONCAT22(uVar5,uVar4);
  if ((uVar5 | uVar4) == 0x0) {
    uVar3 = (u32)&iVar6[0x1].field1_0x2;
    (u32)((int)uVar3 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar7;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar7,0xa,uVar4 + 0x2,uVar5);
    uVar3 = (u32)&iVar6[0x1].field1_0x2;
    uVar9 = ((u32)uVar3 >> 0x10);
    iVar7 = (int)uVar3;
    (iVar7 + 0x2) = uVar4 + 0x2;
    (iVar7 + 0x4) = uVar5;
  }
  uVar1 = (u32)&iVar6[0x1].field1_0x2;
  (u32)((int)uVar1 + 0x6) = param_4;
  uVar2 = (u32)&iVar6[0x1].field1_0x2;
  ((int)uVar2 + 0xa) = 0x1;
  uVar3 = (u32)&iVar6[0x1].field1_0x2;
  ((int)uVar3 + 0x12) = iVar6->field5_0xa;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_45e8(u8 *param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  StructD *pSVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  StructD *pSVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  Struct57*paVar7;
  Struct57*paVar9;
  i16 iVar10;
  u16 unaff_SI;
  u16 uVar11;
  astruct_20 *paVar12;
  u16 in_stack_0000fe88;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb6;
  i16 *piStack16;
  Struct57*paVar8;

  paVar7 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (param_5 != 0xeb) {
    pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                    param_5);
    return;
  }
  paVar12 = (astruct_20 *)
            mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe88,
                            in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
  paVar7 = (Struct57*)((u32)paVar7 & 0xffff0000 | (u32)paVar12 >> 0x10);
  pSVar1 = *(StructD **)(param_2 + 0x90);
  if (pSVar1 != NULL) {
    pSVar5 = pSVar1;
    mem_op_1000_179c(0x18,paVar7);
    uVar4 = pSVar5;
    uVar6 = paVar7 | uVar4;
    paVar9 = (Struct57*)((u32)paVar7 & 0xffff0000);
    paVar8 = (Struct57*)((u32)paVar9 | (u32)uVar6);
    if (uVar6 == 0x0) {
      uVar4 = 0x0;
    }
    else {
      struct_1040_a598((astruct_259 *)((u32)pSVar5 & 0xffff | (long)paVar7 << 0x10));
      paVar9 = paVar8;
    }
    (param_2 + 0x90) = uVar4;
    (param_2 + 0x92) = (int)paVar9;
    (u32)(param_2 + 0x90) = 0x14;
    iVar10 = **(i16 **)(param_2 + 0x90);
    uVar4 = iVar10 * 0xa + 0x2;
    mem_op_1000_179c(uVar4,paVar9);
    uVar6 = paVar9;
    piStack16 = (i16 *)CONCAT22(uVar6,uVar4);
    if ((uVar6 | uVar4) == 0x0) {
      uVar3 = (u32)(param_2 + 0x90);
      (u32)((int)uVar3 + 0x2) = 0x0;
    }
    else {
      *piStack16 = iVar10;
      pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar10,0xa,uVar4 + 0x2,uVar6);
      uVar3 = (u32)(param_2 + 0x90);
      uVar11 = ((u32)uVar3 >> 0x10);
      iVar10 = (int)uVar3;
      (iVar10 + 0x2) = uVar4 + 0x2;
      (iVar10 + 0x4) = uVar6;
    }
    uVar3 = (u32)(param_2 + 0x90);
    (u32)((int)uVar3 + 0x6) = (u32)((int)pSVar1 + 0x6);
    uVar3 = (u32)(param_2 + 0x90);
    ((int)uVar3 + 0xa) = 0x1;
    uVar3 = (u32)(param_2 + 0x90);
    ((int)uVar3 + 0x12) = (param_2 + 0xa);
    uVar11 = 0x1010;
    pass1_1010_a50c(paVar12,(u8 **)0x10505d40,*(StructD **)(param_2 + 0x90));
    if (pSVar1 != NULL) {
      pass1_1040_a5d0(pSVar1);
      uVar11 = 0x1000;
      fn_ptr_1000_17ce((char *)pSVar1);
    }
    ppcVar2 = (code **)((int)(u32)CONCAT22(param_3,param_2) + 0x70);
    (**ppcVar2)(uVar11,param_2,param_3);
  }
  return;
}



void pass1_1040_4766(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_477e(u16 param_1,StructB *param_2)

{
  u8 *puVar1;
  u16 *pUVar2;
  u8 *puVar3;
  u8 *puVar4;
  u16 in_register_0000000a;
  Struct57*paVar5;
  u32 *puVar6;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 uVar7;
  u16 uVar8;
  u16 in_stack_0000ffee;

  paVar5 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  unk_win_ui_op_1040_b230(param_1,param_2);
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x3),in_stack_0000fe96,
                           in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  puVar3 = (u8 *)((u32)puVar6 >> 0x10);
  uVar8 = SUB42(0x1050,0x0);
  uVar7 = 0x5d68;
  puVar1 = pass1_1008_5fd8(puVar3);
  puVar4 = puVar3;
  pUVar2 = pass1_1000_3cea(CONCAT22(puVar3,puVar1),(char *)CONCAT22(uVar8,uVar7));
  pass1_1010_e964(puVar4);
  pass1_1000_3cea(CONCAT22(puVar3,puVar1),(char *)CONCAT22(puVar4,pUVar2));
  unk_str_op_1000_3d3e
            ((char *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x10)),(char *)CONCAT22(puVar3,puVar1));
  fn_ptr_1000_17ce((char *)CONCAT22(puVar3,puVar1));
  return;
}



StructD * pass1_1040_47fe(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffda;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_48a0(Struct57*param_1,Struct57*param_2,u16 param_3,param_4: u32,u16 param_5)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;
  u16 uVar5;
  u16 uVar6;
  u8 *puVar7;
  Struct57*paVar8;
  Struct57*iVar5;
  astruct_445 *iVar6;
  u16 unaff_SI;
  Struct57*uVar10;
  u16 uVar11;
  u32 *puVar12;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  i16 *piStack8;
  Struct57*paVar9;

  struct_1040_b082(param_2,CONCAT22(param_5,0xfa1));
  uVar10 = (Struct57*)((u32)param_2 >> 0x10);
  iVar5 = (Struct57*)param_2;
  (u32)&iVar5[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = &PTR_LOOP_1050_4e18;
  iVar5->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar12 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe9c,
                            in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  paVar8 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)puVar12 >> 0x10);
  uVar5 = puVar12;
  iVar5[0x1].field3_0x6 = uVar5;
  iVar5[0x1].field4_0x8 = ((u32)puVar12 >> 0x10);
  mem_op_1000_179c(0x18,paVar8);
  uVar6 = paVar8 | uVar5;
  paVar9 = (Struct57*)((u32)paVar8 & 0xffff0000 | (u32)uVar6);
  if (uVar6 == 0x0) {
    (u32)&iVar5[0x1].field1_0x2 = 0x0;
  }
  else {
    struct_1040_a598((astruct_259 *)CONCAT22(paVar8,uVar5));
    iVar5[0x1].field1_0x2 = uVar5;
    iVar5[0x1].field2_0x4 = paVar9;
  }
  *(u16*)&iVar5[0x1].field1_0x2 = 0x7;
  iVar1 = **(i16 **)&iVar5[0x1].field1_0x2;
  uVar5 = iVar1 * 0xa + 0x2;
  mem_op_1000_179c(uVar5,paVar9);
  puVar7 = (u8 *)paVar9;
  piStack8 = (i16 *)CONCAT22(puVar7,uVar5);
  if ((puVar7 | uVar5) == 0x0) {
    uVar4 = (u32)&iVar5[0x1].field1_0x2;
    (u32)((int)uVar4 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar1;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar1,0xa,uVar5 + 0x2,puVar7);
    uVar4 = (u32)&iVar5[0x1].field1_0x2;
    uVar11 = ((u32)uVar4 >> 0x10);
    iVar6 = (astruct_445 *)uVar4;
    iVar6->field2_0x2 = uVar5 + 0x2;
    iVar6->field3_0x4 = puVar7;
  }
  uVar4 = (u32)&iVar5[0x1].field1_0x2;
  (u32)((int)uVar4 + 0x6) = param_4;
  uVar4 = (u32)&iVar5[0x1].field1_0x2;
  ((int)uVar4 + 0xa) = param_3;
  uVar4 = (u32)&iVar5[0x1].field1_0x2;
  ((int)uVar4 + 0x12) = iVar5->field5_0xa;
  uVar2 = iVar5[0x1].field1_0x2;
  uVar3 = iVar5[0x1].field2_0x4;
  pass1_1010_debe((u32)&iVar5[0x1].field3_0x6,(uVar2 + 0xa),(u16 *)CONCAT22(uVar3,uVar2 + 0x10),
                  (u32 *)CONCAT22(uVar3,uVar2 + 0xc),param_4);
  return;
}



LRESULT send_win_msg_1040_4a0a(astruct_48 *struct_param_1)

{
  WPARAM16 *pWVar1;
  code **ppcVar2;
  u32 uVar3;
  u32 uVar4;
  HWND16 dlg_item;
  u16 uVar5;
  astruct_48 *iVar5;
  astruct_48 *uVar6;
  LRESULT lresult_6;
  char *pcVar6;
  WPARAM16 wparam;
  u16 UVar7;
  HWND16 HVar8;
  WPARAM16 WStack10;

  uVar6 = (astruct_48 *)((u32)struct_param_1 >> 0x10);
  iVar5 = (astruct_48 *)struct_param_1;
  ppcVar2 = (code **)((int)(u32)struct_param_1 + 0x74);
  (**ppcVar2)();
  dlg_item = GetDlgItem16(0x1770,iVar5->hwnd_0x6);
  SendMessage16(0x0,0x0,0x40b,dlg_item);
  lresult_6 = SendMessage16(0x0,0x0,0xb,dlg_item);
  uVar5 = ((u32)lresult_6 >> 0x10);
  for (WStack10 = 0x0; uVar3 = iVar5->field143_0x90, pWVar1 = (WPARAM16 *)((int)uVar3 + 0x10),
      *pWVar1 != WStack10 && (int)WStack10 <= (int)*pWVar1; WStack10 += 0x1) {
    wparam = 0x0;
    UVar7 = 0x403;
    uVar3 = iVar5->field143_0x90;
    uVar3 = (u32)((int)uVar3 + 0xc);
    HVar8 = dlg_item;
    pcVar6 = pass1_1040_4dcc(uVar5,struct_param_1,((int)uVar3 + WStack10 * 0x2));
    lresult_6 = SendMessage16((LPARAM)pcVar6,wparam,UVar7,HVar8);
    uVar5 = ((u32)lresult_6 >> 0x10);
  }
  pass1_1040_4d7e(struct_param_1);
  if (WStack10 == 0x0) {
    UVar7 = 0x40a;
    uVar4 = iVar5->field143_0x90;
    uVar3 = iVar5->field144_0x94;
    HVar8 = dlg_item;
    pcVar6 = string_op_1010_ada6(uVar5,uVar3,((u32)uVar3 >> 0x10),0x0,((int)uVar4 + 0xa));
    SendMessage16((LPARAM)pcVar6,WStack10,UVar7,HVar8);
  }
  lresult_6 = SendMessage16(0x0,0x1,0xb,dlg_item);
  return lresult_6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void set_win_pos_1040_4ae4(i16 param_1,u16 param_2,u16 param_3,u32 param_4)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  StructD *pSVar4;
  u16 uVar5;
  Struct57*in_EDX;
  Struct57*paVar6;
  Struct57*paVar8;
  i16 iVar9;
  u16 unaff_SI;
  u16 uVar10;
  u16 in_stack_0000fe80;
  u16 in_stack_0000ffa4;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  i16 local_24 [0x2];
  i16 iStack32;
  StructD *pSStack20;
  StructD *pSStack16;
  i16 iStack12;
  StructD *pSStack10;
  astruct_20 *paStack6;
  Struct57*paVar7;

  if (param_4 == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe80,
                               in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
    paVar6 = (Struct57*)((u32)in_EDX & 0xffff0000 | (u32)paStack6 >> 0x10);
    pSVar4 = *(StructD **)(param_1 + 0x90);
    if (pSVar4 != NULL) {
      pSStack10 = pSVar4;
      mem_op_1000_179c(0x18,paVar6);
      uVar3 = pSVar4;
      pSStack16 = (StructD *)((u32)pSVar4 & 0xffff | (long)paVar6 << 0x10);
      uVar5 = paVar6 | uVar3;
      paVar8 = (Struct57*)((u32)paVar6 & 0xffff0000);
      paVar7 = (Struct57*)((u32)paVar8 | (u32)uVar5);
      if (uVar5 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1040_a598((astruct_259 *)((u32)pSVar4 & 0xffff | (long)paVar6 << 0x10));
        paVar8 = paVar7;
      }
      (param_1 + 0x90) = uVar3;
      (param_1 + 0x92) = (int)paVar8;
      (u32)(param_1 + 0x90) = 0x7;
      iStack12 = **(i16 **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,paVar8);
      uVar5 = paVar8;
      pSStack16 = (StructD *)CONCAT22(uVar5,uVar3);
      if ((uVar5 | uVar3) == 0x0) {
        uVar2 = (u32)(param_1 + 0x90);
        (u32)((int)uVar2 + 0x2) = 0x0;
      }
      else {
        pSStack16 = iStack12;
        pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,uVar5);
        uVar2 = (u32)(param_1 + 0x90);
        uVar10 = ((u32)uVar2 >> 0x10);
        iVar9 = (int)uVar2;
        (iVar9 + 0x2) = uVar3 + 0x2;
        (iVar9 + 0x4) = uVar5;
      }
      uVar10 = ((u32)pSStack10 >> 0x10);
      iVar9 = (int)pSStack10;
      uVar2 = (u32)(param_1 + 0x90);
      (u32)((int)uVar2 + 0x6) = (u32)(iVar9 + 0x6);
      uVar2 = (u32)(param_1 + 0x90);
      ((int)uVar2 + 0xa) = (iVar9 + 0xa);
      uVar2 = (u32)(param_1 + 0x90);
      ((int)uVar2 + 0x12) = (iVar9 + 0x12);
      uVar10 = 0x1010;
      pass1_1010_a50c(paStack6,(u8 **)0x10505d6a,*(StructD **)(param_1 + 0x90));
      pSStack20 = pSStack10;
      pSStack16 = pSStack10;
      if (pSStack10 != NULL) {
        pass1_1040_a5d0(pSStack10);
        uVar10 = 0x1000;
        fn_ptr_1000_17ce((char *)pSStack10);
      }
      ppcVar1 = (code **)((int)(u32)CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar1)(uVar10,param_1,param_2);
    }
  }
  else {
    if (param_4 != 0x1770) {
      pass1_1040_b54a((u8 *)in_EDX,(astruct_903 *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)),
                      param_3,param_4);
      return;
    }
    if ((int)param_4 == 0x7) {
      GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_24),param_3);
      iStack32 -= local_24[0];
      SetWindowPos16(0x2,0x50,iStack32,0x0,0x0,0x0,param_3);
    }
  }
  return;
}



LRESULT send_msg_1040_4cb2(u16 param_1,u32 param_2)

{
  u8 uVar1;
  HWND16 HVar1;
  u32 uVar2;
  LRESULT LVar2;
  HWND16 hwnd;
  u16 uVar3;
  u16 uVar4;

  pass1_1040_b45e(param_2);
  HVar1 = GetDlgItem16(0x1770,*(HWND16 *)((int)param_2 + 0x6));
  uVar3 = 0xffff;
  uVar4 = 0x40d;
  hwnd = HVar1;
  pass1_1040_4d7e((astruct_48 *)param_2);
  uVar2 = (u32)pass1_1040_4dcc(param_1,(astruct_48 *)param_2,HVar1);
  LVar2 = SendMessage16(uVar2,uVar3,uVar4,hwnd);
  return LVar2;
}



void pass1_1040_4cf4(u32 param_1)

{
  u32 uVar1;
  u32 uVar2;
  HWND16 hwnd;
  u16 uVar3;
  u32 in_EDX;
  u32 uVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  LRESULT LVar9;
  u32 uVar10;
  u8 local_52 [0x50];

  uVar8 = ((u32)in_EDX >> 0x10);
  uVar7 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  hwnd = GetDlgItem16(0x1770,*(HWND16 *)(iVar5 + 0x6));
  LVar9 = SendMessage16(0x0,0x0,0x407,hwnd);
  uVar4 = CONCAT22(uVar8,(int)((u32)LVar9 >> 0x10));
  if ((WPARAM16)LVar9 != 0xffff) {
    uVar10 = SendMessage16(CONCAT22(0x1050,local_52),(WPARAM16)LVar9,0x408,hwnd);
    uVar4 = uVar4 & 0xffff0000 | uVar10 >> 0x10;
  }
  uVar2 = (u32)(iVar5 + 0x90);
  uVar1 = (u32)(iVar5 + 0x94);
  uVar3 = pass1_1010_ae12((char *)uVar4,uVar1,((u32)uVar1 >> 0x10),(char *)CONCAT22(0x1050,local_52),
                          ((int)uVar2 + 0xa));
  if (uVar3 != 0xffff) {
    uVar1 = (u32)(iVar5 + 0x90);
    uVar8 = ((u32)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    uVar10 = (u32)(iVar6 + 0x6);
    pass1_1010_ae92((u32)(iVar5 + 0x94),uVar10,(iVar6 + 0xa),uVar10,uVar4);
  }
  return;
}



void pass1_1040_4d7e(astruct_48 *param_1)

{
  u32 uVar1;
  i16 *piVar2;
  u16 uVar3;
  i16 iStack8;
  u32 *puStack6;

  uVar3 = ((u32)param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x90);
  puStack6 = ((int)uVar1 + 0x2);
  iStack8 = 0x0;
  while ((piVar2 = *(i16 **)((int)param_1 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2 &&
         (((int)puStack6 + 0x4) != 0x1770))) {
    iStack8 += 0x1;
    puStack6 = (u32 *)((u32)puStack6 & 0xffff0000 | (u32)((int)puStack6 + 0xa));
  }
  pass1_1000_3e2c(*puStack6);
  return;
}



char * pass1_1040_4dcc(u16 param_1,astruct_48 *param_2,param_3: i16)

{
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;
  char *pcVar4;

  uVar3 = ((u32)param_2 >> 0x10);
  uVar2 = (u32)((int)param_2 + 0x90);
  uVar1 = (u32)((int)param_2 + 0x94);
  pcVar4 = string_op_1010_ada6(param_1,uVar1,((u32)uVar1 >> 0x10),param_3,((int)uVar2 + 0xa));
  return pcVar4;
}



StructD * pass1_1040_4df2(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffda;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_4e94(Struct57*param_1,i32 param_2,u16 param_3)

{
  Struct57*iVar1;
  Struct57*uVar1;
  u16 uVar2;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  (u32)&iVar1[0x1].field18_0x22 = 0x0;
  iVar1[0x1].field21_0x26 = 0x0;
  &iVar1[0x1].field_0x28 = 0x0;
  param_1->field0_0x0 = 0x55a2;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar2 = ((u32)param_2 >> 0x10);
    (u32)&iVar1[0x1].field18_0x22 = (u32)((int)param_2 + 0x6);
    iVar1[0x1].field21_0x26 = ((int)param_2 + 0x14);
  }
  return;
}



void pass1_1040_4f0a(StructD *param_1)

{
  u16 in_stack_0000ffde;

  param_1->address_offset_field_0x0 = 0x55a2;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



u16 pass1_1040_4f28(u32 *param_1,i16 *param_2,u16 param_3,u16 param_4,i16 param_5,u16 param_6)

{
  code **ppcVar1;
  u16 uVar2;

  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((i16 *)CONCAT22(param_3,param_2));
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x7c);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



void pass1_1040_4f82(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x80);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void set_win_pos_1040_4f96
               (StructD *param_1,StructB *struct_b_param_1,u16 param_3,u16 param_4,u16 param_5)

{
  LPVOID pvVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  Struct57*paVar5;
  u16 uVar6;
  u16 uVar7;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  Struct57*paVar12;
  Struct57*paVar14;
  StructB *struct_b_11;
  u16 uVar15;
  u16 uVar16;
  u32 *puVar17;
  u16 *puVar18;
  u16 in_stack_0000fe34;
  u16 in_stack_0000fe36;
  u16 in_stack_0000fe38;
  u16 in_stack_0000fe3a;
  u16 in_stack_0000fe88;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ff5e;
  u16 in_stack_0000ff60;
  u16 in_stack_0000ff62;
  u16 in_stack_0000ff64;
  u16 in_stack_0000ff66;
  u16 in_stack_0000ff68;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u8 uVar19;
  u8 uVar20;
  BOOL16 BVar21;
  u8 *puVar22;
  Struct57*paVar13;
  code **fn_ptr_1;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar17 = mixed_1010_20ba((Struct57*)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,0x41),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  paVar12 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)puVar17 >> 0x10);
  paVar5 = (Struct57*)puVar17;
  uVar15 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_11 = (StructB *)struct_b_param_1;
  struct_b_11[0x7].field6_0xc = (u8 *)paVar5;
  uVar9 = ((u32)puVar17 >> 0x10);
  struct_b_11[0x7].field7_0xe = uVar9;
  puVar22 = struct_b_11[0x7].field6_0xc;
  fn_ptr_1 = (code **)((int)*&struct_b_11[0x7].field6_0xc + 0x10);
  (**fn_ptr_1)(0x1010,puVar22,uVar9);
  mem_op_1000_179c(0xa,paVar12);
  uVar10 = paVar12 | paVar5;
  paVar13 = (Struct57*)((u32)paVar12 & 0xffff0000 | (u32)uVar10);
  if (uVar10 == 0x0) {
    (u32)&struct_b_11[0x7].max_count_field_0x10 = 0x0;
  }
  else {
    puVar18 = struct_1040_bf3e((astruct_442 *)CONCAT13((char)((u32)paVar12 >> 0x8),CONCAT12((char)paVar12,paVar5)),
                               struct_b_11->lpvoid_field_0x8);
    paVar13 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)puVar18 >> 0x10);
    paVar5 = (Struct57*)puVar18;
    struct_b_11[0x7].max_count_field_0x10 = paVar5;
    struct_b_11[0x7].field5_0xa = (u32 *)((u32)puVar18 >> 0x10);
  }
  pass1_1040_bfde(*(void **)&struct_b_11[0x7].max_count_field_0x10,&struct_b_11[0x7].field6_0xc);
  mem_op_1000_179c(0x42,paVar13);
  uVar10 = (Struct57*)paVar13 | paVar5;
  paVar12 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar12,paVar5,(Struct57*)paVar13,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_11->lpvoid_field_0x8,0x10a),param_4,in_stack_0000fe36,in_stack_0000fe3a,
                    in_stack_0000ff60,in_stack_0000ff64,in_stack_0000ff68);
  }
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = (Struct57*)paVar12 | paVar5;
  paVar13 = (Struct57*)((u32)paVar12 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar13,paVar5,(Struct57*)paVar12,0x1,0xa0028,0x0,0x840085,
                    CONCAT22(struct_b_11->lpvoid_field_0x8,0x10b),param_4,in_stack_0000fe36,in_stack_0000fe3a,
                    in_stack_0000ff60,in_stack_0000ff64,in_stack_0000ff68);
  }
  mem_op_1000_179c(0x42,paVar13);
  uVar10 = (Struct57*)paVar13 | paVar5;
  paVar12 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar12,paVar5,(Struct57*)paVar13,0x1,0xa0046,0x0,0x860087,
                    CONCAT22(struct_b_11->lpvoid_field_0x8,0x10d),param_4,in_stack_0000fe36,in_stack_0000fe3a,
                    in_stack_0000ff60,in_stack_0000ff64,in_stack_0000ff68);
  }
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = (Struct57*)paVar12 | paVar5;
  paVar13 = (Struct57*)((u32)paVar12 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar13,paVar5,(Struct57*)paVar12,0x1,0xa0064,0x0,0x880089,
                    CONCAT22(struct_b_11->lpvoid_field_0x8,0x10e),param_4,in_stack_0000fe36,in_stack_0000fe3a,
                    in_stack_0000ff60,in_stack_0000ff64,in_stack_0000ff68);
  }
  mem_op_1000_179c(0x42,paVar13);
  uVar10 = (Struct57*)paVar13 | paVar5;
  paVar12 = (Struct57*)((u32)paVar13 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar12,paVar5,(Struct57*)paVar13,0x1,0xa0082,0x0,0x820083,
                    CONCAT22(struct_b_11->lpvoid_field_0x8,0x10c),param_4,in_stack_0000fe36,in_stack_0000fe3a,
                    in_stack_0000ff60,in_stack_0000ff64,in_stack_0000ff68);
  }
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = (Struct57*)paVar12 | paVar5;
  paVar13 = (Struct57*)((u32)paVar12 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar13,paVar5,(Struct57*)paVar12,0x1,0xa00d2,0x0,0x8a008b,
                    CONCAT22(struct_b_11->lpvoid_field_0x8,0xbbb),param_4,in_stack_0000fe36,in_stack_0000fe3a,
                    in_stack_0000ff60,in_stack_0000ff64,in_stack_0000ff68);
  }
  BVar21 = 0x0;
  mem_op_1000_179c(0x42,paVar13);
  uVar10 = (Struct57*)paVar13 | paVar5;
  paVar12 = (Struct57*)((u32)paVar13 & 0xffff0000);
  paVar14 = (Struct57*)((u32)paVar12 | (u32)uVar10);
  if (uVar10 == 0x0) {
    paVar5 = NULL;
  }
  else {
    pvVar1 = struct_b_11->lpvoid_field_0x8;
    pass1_1008_3bd6((u32)paVar14,paVar5,(Struct57*)paVar13,0x1,0xa00a0,0x8e,0x8c008d,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0xbbc)),param_3,in_stack_0000fe34,
                    in_stack_0000fe38,in_stack_0000ff5e,in_stack_0000ff62,in_stack_0000ff66);
    paVar12 = paVar14;
  }
  uVar19 = SUB41(paVar12,0x0);
  uVar20 = (u8)((u32)paVar12 >> 0x8);
  enable_win_1040_9234(CONCAT13(uVar20,CONCAT12(uVar19,paVar5)),BVar21);
  puVar17 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)CONCAT22(puVar22,0x3),in_stack_0000fe88,
                            in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
  uVar4 = ((u32)puVar17 >> 0x10);
  uVar3 = puVar17;
  uVar11 = uVar4;
  uVar6 = pass1_1010_a5ac(uVar3,uVar4,(u32)struct_b_11[0x8].field8_0x10);
  uVar7 = pass1_1010_ac62(uVar6,uVar11,uVar3,uVar4,0x1e);
  if (uVar7 != 0x0) {
    pass1_1010_a5ca(uVar7,uVar11,uVar3,uVar4,uVar6);
    if (0x0 < (int)uVar7) {
      pass1_1010_a58a(uVar7,uVar11,uVar3,uVar4,uVar6);
      if (uVar7 == 0x0) {
        enable_win_1040_9234(CONCAT13(uVar20,CONCAT12(uVar19,paVar5)),0x1);
      }
    }
  }
  uVar2 = (u32)&struct_b_11[0x7].field6_0xc;
  iVar8 = (int)uVar2;
  uVar2 &= 0xffff0000;
  uVar16 = (uVar2 >> 0x10);
  SetWindowPos16(0x40,*(INT16 *)(iVar8 + 0x10),*(INT16 *)(iVar8 + 0xe),*(INT16 *)(iVar8 + 0xc),
                 *(INT16 *)(uVar2 | iVar8 + 0xa),0x0,(HWND16)struct_b_11->lpvoid_field_0x8);
  return;
}



u16 pass1_1040_5238(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void destroy_win_1040_5256(astruct_34 *param_1)

{
  u32 *pUVar1;
  BOOL16 bool3;
  astruct_34 *pstruct34_5;
  astruct_34 *pstruct34_hi;
  u16 unaff_CS;
  u16 uVar2;
  code **fn_ptr_1;

  pstruct34_hi = (astruct_34 *)((u32)param_1 >> 0x10);
  pstruct34_5 = (astruct_34 *)param_1;
  if (pstruct34_5->hwnd_0xb6 != 0x0) {
    // 0x1538
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    bool3 = IsWindow16(pstruct34_5->hwnd_0xb6);
    if (bool3 != 0x0) {
    // 0x1538
      unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
      DestroyWindow16(pstruct34_5->hwnd_0xb6);
    }
  }
  pstruct34_5->hwnd_0xb6 = 0x0;
  pUVar1 = pstruct34_5->field148_0x94;
  uVar2 = pstruct34_5->field149_0x96;
  if ((uVar2 | pUVar1) != 0x0) {
    fn_ptr_1 = (code **)*pUVar1;
    (**fn_ptr_1)(unaff_CS,pUVar1,uVar2,0x1);
  }
  (u32)&pstruct34_5->field148_0x94 = 0x0;
  pstruct34_5->field150_0x98 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_52c0(u8 *param_1,astruct_894 *param_2,u16 param_3,u16 param_4,u32 param_5)

{
  code **ppcVar1;
  u32 *puVar2;
  BOOL16 is_window;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  Struct57*paVar8;
  u32 uVar9;
  u32 *puVar10;
  astruct_940 *paVar11;
  u16 in_stack_0000fe84;
  u16 in_stack_0000fe86;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb4;
  u16 uVar12;
  u16 uVar13;
  u16 uVar14;
  u16 in_stack_0000ffde;
  u16 uVar15;
  u16 uStack30;
  RECT16 local_a;
  i16 iStack6;
  i16 iStack4;
  HWND16 hwnd_8;

  paVar8 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (param_5 != 0x10c) {
    if (param_5 < 0x10d) {
      if (param_5 == 0xfa) {
        ppcVar1 = (code **)((int)*param_2->field148_0x98 + 0x18);
        (**ppcVar1)();
        return;
      }
      if (param_5 == 0x10a) {
        GetClientRect16(&local_a,(HWND16)0x1050);
        puVar2 = param_2->field148_0x98;
        local_a.y += 0x3;
        local_a.x = ((int)puVar2 + 0x1a) + -0x9;
        iStack6 += -0x3;
        iStack4 += -0x3;
        InvalidateRect16(0x1,&local_a,(HWND16)0x1050);
        unk_destroy_win_op_1010_2fa0((astruct_873 *)param_2->field148_0x98);
        pass1_1010_32c0((u32)param_2->field148_0x98,0x0);
        pass1_1010_2ee2(param_2->field148_0x98);
        return;
      }
      if (param_5 != 0x10b) {
LAB_1040_5560:
        pass1_1040_b54a(param_1,(astruct_903 *)CONCAT22(param_3,param_2),param_4,param_5);
        return;
      }
      puVar2 = param_2->field148_0x98;
      uVar12 = ((int)puVar2 + 0x12);
      uVar5 = uVar12;
      puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(uVar12,0x3),in_stack_0000fe84,
                                in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      uVar7 = ((u32)puVar10 >> 0x10);
      uStack30 = puVar10;
      uVar4 = uStack30;
      uVar6 = uVar7;
      pass1_1010_a5ca(uStack30,uVar7,uStack30,uVar7,uVar12);
      if ((uVar5 != 0x70) && (uVar4 == 0x0)) {
        return;
      }
      uVar9 = param_2->field169_0xb0;
      uVar13 = uVar9;
      uVar14 = (uVar9 >> 0x10);
      puVar2 = param_2->field148_0x98;
      uVar12 = ((int)puVar2 + 0x12);
    }
    else {
      if (param_5 != 0x10d) {
        if (param_5 == 0x10e) {
          puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x32),
                                    in_stack_0000fe86,in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
          uVar9 = (u32)paVar8 & 0xffff0000 | (u32)puVar10 >> 0x10;
          uVar3 = puVar10;
          uVar15 = uVar3;
          ui_op_1010_79aa(puVar10,0xfc6,param_2->field169_0xb0);
          if (uVar3 != 0x0) {
            return;
          }
          unk_win_op_1010_7300
                    (uVar9,(Struct57*)((u32)puVar10 & 0xffff0000 | (u32)uVar15),0x0,0x13,param_2->field169_0xb0);
          return;
        }
        if (param_5 != 0xbbb) {
          if (param_5 == 0xbbc) {
            puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x3),
                                      in_stack_0000fe86,in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
            uVar12 = ((u32)puVar10 >> 0x10);
            uVar4 = puVar10;
            uVar7 = uVar12;
            uVar5 = pass1_1010_a5ac(uVar4,uVar12,param_2->field169_0xb0);
            uVar6 = uVar5;
            pass1_1010_a58a(uVar5,uVar7,uVar4,uVar12,uVar5);
            if (uVar6 == 0x0) {
              pass1_1010_a568(0x0,uVar7,uVar4,uVar12,uVar5);
            }
            hwnd_8 = GetDlgItem16(0xbbc,param_2->hwnd_0x6);
            EnableWindow16(0x0,hwnd_8);
            return;
          }
          goto LAB_1040_5560;
        }
        if ((param_2->field171_0xb6 == 0x0) || (is_window = IsWindow16(param_2->field171_0xb6), is_window == 0x0)) {
          paVar11 = (astruct_940 *)
                    pass1_1038_af40(param_2,paVar8,_PTR_LOOP_1050_5b7c,param_2->hwnd_0x6,0x1b);
          param_2->field171_0xb6 = *(HWND16 *)((int)paVar11 + 0x6);
          set_win_pos_1038_abdc(paVar11);
          ShowWindow16(0x1,param_2->field171_0xb6);
          return;
        }
        hwnd_8 = param_2->field171_0xb6;
        goto LAB_1040_5417;
      }
      puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x3),in_stack_0000fe86,
                                in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
      uVar6 = ((u32)puVar10 >> 0x10);
      uStack30 = puVar10;
      uVar9 = param_2->field169_0xb0;
      uVar13 = uVar9;
      uVar14 = (uVar9 >> 0x10);
      uVar12 = 0x71;
      uVar7 = uVar6;
    }
    pass1_1010_a5ec(uVar6,uStack30,uVar7,uVar12,CONCAT22(uVar14,uVar13));
    if ((param_2->hwnd_0xb4 != 0x0) && (is_window = IsWindow16(param_2->hwnd_0xb4), is_window != 0x0)) {
      SendMessage16(0x0,0xeb,0x111,param_2->hwnd_0xb4);
    }
  }
  hwnd_8 = param_2->hwnd_0x6;
LAB_1040_5417:
  DestroyWindow16(hwnd_8);
  return;
}



StructD * pass1_1040_557c(StructD *param_1,u8 param_2)

{
  pass1_1040_4f0a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar1

void pass1_1040_5626(Struct57*param_1,Struct57*param_2,param_3: u32,u16 param_4)

{
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  Struct57*iVar9;
  i16 iVar6;
  Struct57*uVar11;
  u16 uVar7;
  u32 uVar8;
  i16 *piStack12;
  u32 uVar1;
  Struct57*paVar5;

  struct_1040_b082(param_2,CONCAT22(param_4,0xfa3));
  uVar11 = (Struct57*)((u32)param_2 >> 0x10);
  iVar9 = (Struct57*)param_2;
  uVar3 = 0x0;
  iVar9[0x1].field3_0x6 = 0x0;
  iVar9[0x1].field4_0x8 = 0x0;
  iVar9[0x1].field5_0xa = 0x0;
  iVar9[0x1].field7_0xe = 0x0;
  param_2->field0_0x0 = 0x6386;
  iVar9->field1_0x2 = &PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_1);
  uVar4 = param_1 | uVar3;
  paVar5 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)uVar4);
  if (uVar4 == 0x0) {
    (u32)&iVar9[0x1].field1_0x2 = 0x0;
  }
  else {
    struct_1040_a598((astruct_259 *)CONCAT22(param_1,uVar3));
    iVar9[0x1].field1_0x2 = uVar3;
    iVar9[0x1].field2_0x4 = paVar5;
  }
  *(u16*)&iVar9[0x1].field1_0x2 = 0x6;
  iVar6 = **(i16 **)&iVar9[0x1].field1_0x2;
  uVar3 = iVar6 * 0xa + 0x2;
  mem_op_1000_179c(uVar3,paVar5);
  uVar4 = paVar5;
  piStack12 = (i16 *)CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0x0) {
    uVar2 = (u32)&iVar9[0x1].field1_0x2;
    (u32)((int)uVar2 + 0x2) = 0x0;
  }
  else {
    *piStack12 = iVar6;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar6,0xa,uVar3 + 0x2,uVar4);
    uVar2 = (u32)&iVar9[0x1].field1_0x2;
    uVar7 = ((u32)uVar2 >> 0x10);
    iVar6 = (int)uVar2;
    (iVar6 + 0x2) = uVar3 + 0x2;
    (iVar6 + 0x4) = uVar4;
  }
  uVar1 = (u32)&iVar9[0x1].field1_0x2;
  (u32)((int)uVar1 + 0x6) = param_3;
  uVar2 = (u32)&iVar9[0x1].field1_0x2;
  ((int)uVar2 + 0xa) = 0x4;
  uVar2 = (u32)&iVar9[0x1].field1_0x2;
  ((int)uVar2 + 0x12) = iVar9->field5_0xa;
  uVar8 = pass1_1040_5d12((u32)param_2);
  uVar3 = (uVar8 >> 0x10);
  if ((uVar3 | uVar8) == 0x0) {
    iVar9[0x1].field6_0xc = 0x0;
  }
  else {
    iVar9[0x1].field6_0xc = (uVar8 + 0x20);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void enable_win_1040_5780(u32 *param_1)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  HWND16 hwnd;
  Struct57*in_EDX;
  astruct_945 *iVar4;
  u16 uVar4;
  u32 *puVar5;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  astruct_945 *paVar6;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_945 *)param_1;
  ppcVar1 = (code **)((int)*param_1 + 0x74);
  paVar6 = iVar4;
  (**ppcVar1)();
  puVar5 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(paVar6,0x3),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar2 = iVar4->field143_0x90;
  uVar3 = pass1_1010_acc0(puVar5,((u32)puVar5 >> 0x10),(u32)((int)uVar2 + 0x6));
  if (uVar3 != 0x0) {
    hwnd = GetDlgItem16(0x1790,iVar4->field6_0x6);
    EnableWindow16(0x1,hwnd);
  }
  return;
}



void pass1_1040_57d4(u8 *param_1,StructB *param_2)

{
  pass1_1040_5d42(param_2);
  pass1_1040_5eaa(param_2);
  pass1_1040_5dc4(param_1,param_2);
  unk_win_ui_op_1040_b230(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_5800(u8 *param_1,astruct_18 *param_2,u16 param_3,u16 param_4,u32 param_5)

{
  code **ppcVar1;
  u32 uVar2;
  astruct_18 *paVar5;
  u16 uVar3;
  u16 uVar4;
  HWND16 hwnd;
  u16 uVar6;
  u8 *puVar7;
  u16 in_register_0000000a;
  Struct57*paVar8;
  Struct57*paVar10;
  i16 iVar11;
  u16 unaff_SI;
  u16 uVar12;
  u16 in_stack_0000fe80;
  u16 in_stack_0000ffa4;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  i16 *piStack24;
  u8 local_14 [0x8];
  i16 iStack12;
  StructD *pSStack10;
  astruct_20 *paStack6;
  StructD *pSVar5;
  Struct57*paVar9;

  paVar8 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe80,
                               in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
    paVar8 = (Struct57*)((u32)paVar8 & 0xffff0000 | (u32)paStack6 >> 0x10);
    pSVar5 = *(StructD **)&param_2->field138_0x90;
    if (pSVar5 != NULL) {
      pSStack10 = pSVar5;
    // 0x0018
      mem_op_1000_179c(0x18,paVar8);
      uVar3 = pSVar5;
      uVar6 = paVar8 | uVar3;
      paVar10 = (Struct57*)((u32)paVar8 & 0xffff0000);
      paVar9 = (Struct57*)((u32)paVar10 | (u32)uVar6);
      if (uVar6 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1040_a598((astruct_259 *)((u32)pSVar5 & 0xffff | (long)paVar8 << 0x10));
        paVar10 = paVar9;
      }
      param_2->field138_0x90 = uVar3;
      param_2->field139_0x92 = paVar10;
      *(u16*)&param_2->field138_0x90 = 0x6;
      iStack12 = **(i16 **)&param_2->field138_0x90;
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,paVar10);
      uVar6 = paVar10;
      piStack24 = (i16 *)CONCAT22(uVar6,uVar3);
      puVar7 = (u8 *)(uVar6 | uVar3);
      if (puVar7 == NULL) {
        uVar2 = (u32)&param_2->field138_0x90;
        (u32)((int)uVar2 + 0x2) = 0x0;
      }
      else {
        *piStack24 = iStack12;
    // &PTR_LOOP_1050_1040 actually 0x1040
        pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,uVar6);
        uVar2 = (u32)&param_2->field138_0x90;
        uVar12 = ((u32)uVar2 >> 0x10);
        iVar11 = (int)uVar2;
        (iVar11 + 0x2) = uVar3 + 0x2;
        (iVar11 + 0x4) = uVar6;
      }
      uVar2 = (u32)&param_2->field138_0x90;
      (u32)((int)uVar2 + 0x6) = (u32)((int)pSStack10 + 0x6);
      uVar2 = (u32)&param_2->field138_0x90;
      ((int)uVar2 + 0xa) = 0x4;
      uVar2 = (u32)&param_2->field138_0x90;
      ((int)uVar2 + 0x12) = &param_2->field_0xa;
      uVar12 = 0x1010;
      pass1_1010_a50c(paStack6,(u8 **)&u32_1050_5d78,*(StructD **)&param_2->field138_0x90);
      if (pSStack10 != NULL) {
        pass1_1040_a5d0(pSStack10);
        uVar12 = 0x1000;
        fn_ptr_1000_17ce((char *)pSStack10);
      }
      ppcVar1 = (code **)((int)(u32)CONCAT22(param_3,param_2) + 0x70);
      (**ppcVar1)(uVar12,param_2,param_3);
      uVar4 = pass1_1040_5cd6(CONCAT22(param_3,param_2));
      if (uVar4 != 0x0) {
        pass1_1040_5eaa((StructB *)CONCAT22(param_3,param_2));
        &param_2->field_0x94 = 0x0;
      }
      pass1_1040_5dc4(puVar7,(StructB *)CONCAT22(param_3,param_2));
      GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,local_14)),param_2->hwnd_0x6);
      InvalidateRect16(param_2[0x1].base_0x0,NULL,0x0);
      if (param_2[0x1].base_0x0 != 0x0) {
        param_2[0x1].base_0x0 = 0x0;
      }
    }
  }
  else {
    if (param_5 != 0x13b) {
      pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                      param_5);
      return;
    }
    hwnd = GetDlgItem16(0x1790,param_2->hwnd_0x6);
    EnableWindow16(0x1,hwnd);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1040_5a06(param_1: u32,astruct_741 *struct741_param_1)

{
  u16 uVar1;
  i16 caption_height_px;
  i16 IVar2;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  i16 IVar3;
  i16 y;
  i16 IVar4;
  i16 y_00;
  i16 x;
  i16 IVar5;
  u16 in_DX;
  u16 uVar6;
  HPALETTE16 palette_handle_7;
  u32 *puVar2;
  u32 uVar8;
  astruct_741 *struct_1;
  u16 uVar9;
  u16 base_addr;
  u32 uVar11;
  u32 uVar12;
  u8 uVar10;
  u16 uVar14;
  u16 uStack82;
  i16 iStack72;
  i16 iStack68;
  u32 *puStack54;
  HDC16 hdc16_2c;
  u8 paint_struct_2a [0x20];
  u8 rect_array_local_a [0x8];
  u16 uVar13;
  u16 uVar15;
  u16 uVar16;
  u32 uVar4;
  u16 *puVar1;
  u16 uVar5;
  u32 uVar2;
  u32 uVar3;
  u32 uVar7;
  code **fn_ptr_1;

  uVar9 = ((u32)struct741_param_1 >> 0x10);
  struct_1 = (astruct_741 *)struct741_param_1;
  GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,rect_array_local_a)),struct_1->field6_0x6);
  hdc16_2c = BeginPaint16((PAINTSTRUCT16 *)CONCAT13(0x10,CONCAT12(0x50,paint_struct_2a)),struct_1->field6_0x6);
  base_addr = 0x1008;
  palette_handle_7 =
       palette_op_1008_4e08
                 ((HPALETTE16)&hdc16_2c,(int)param_1,*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),
                  (HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&hdc16_2c)));
  puVar2 = NULL;
  puStack54 = NULL;
  uVar7 = param_1;
  if (struct_1->field149_0x98 != 0x0) {
    uVar1 = FUN_1010_830a(0x0,param_1,0x1008,_u16_1050_14cc,struct_1->field149_0x98);
    uVar14 = param_1;
    puStack54 = (u32 *)CONCAT22(uVar14,uVar1);
    uVar7 = param_1;
    uVar11 = pass1_1008_4772((Struct76 *)CONCAT22(uVar14,uVar1));
    uVar6 = (uVar11 >> 0x10) | (u32 *)(uVar11 & 0xffff);
    uVar7 = uVar7 & 0xffff0000 | (u32)uVar6;
    if (uVar6 == 0x0) {
      puVar2 = (u32 *)(uVar11 & 0xffff);
      if (puStack54 != NULL) {
        puVar2 = puStack54;
        if (puStack54 != NULL) {
          fn_ptr_1 = (code **)*puStack54;
          (**fn_ptr_1)(0x8,uVar1,(char)param_1,0x1,uVar14);
          puVar2 = puStack54;
        }
      }
      uVar1 = FUN_1010_830a((int)puVar2,uVar7,0x1008,_u16_1050_14cc,0x4d);
      puStack54 = (u32 *)CONCAT22((int)uVar7,uVar1);
    }
    uVar13 = 0x1050;
    uVar10 = SUB21(&hdc16_2c,0x0);
    base_addr = s_tile2_bmp_1050_1538;
    caption_height_px = GetSystemMetrics16(SM_CYCAPTION);
    puVar2 = (u32 *)(u32)-(caption_height_px + -0x23);
    fn_ptr_1 = (code **)((int)*puStack54 + 0x4);
    (**fn_ptr_1)(0x38,(char)puStack54,(char)((u32)puStack54 >> 0x10),-(caption_height_px + -0x23),uVar10,uVar13);
  }
  if (puStack54 != NULL) {
    uVar1 = ((u32)puStack54 >> 0x10);
    puVar2 = puStack54;
    if (puStack54 != NULL) {
      fn_ptr_1 = (code **)*puStack54;
      (**fn_ptr_1)((char)base_addr,(char)puStack54,uVar1,0x1,(int)puStack54,uVar1);
      puVar2 = puStack54;
    }
  }
  uVar1 = FUN_1010_830a((int)puVar2,uVar7,base_addr,_u16_1050_14cc,struct_1->field148_0x96);
  puStack54 = (u32 *)CONCAT22((int)uVar7,uVar1);
  uVar14 = SUB42(0x1050,0x0);
  uVar10 = SUB21(&hdc16_2c,0x0);
  uVar8 = uVar7;
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  uVar3 = *puStack54;
  fn_ptr_1 = (code **)uVar3 + 0x2;
  (**fn_ptr_1)(0x38,(char)uVar1,(char)uVar7,-(IVar2 + -0x23),uVar10,uVar14);
  if (puStack54 != NULL) {
    if (puStack54 != NULL) {
      fn_ptr_1 = (code **)uVar3;
      (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,uVar1,(char)uVar7,0x1);
    }
  }
  handle = CreatePen16(0x1000025,0x0,0x0);
  handle_00 = SelectObject16(handle,hdc16_2c);
  uVar14 = FUN_1010_830a(handle_00,uVar8,(int)s_tile2_bmp_1050_1538,_u16_1050_14cc,0x4f);
  puStack54 = (u32 *)CONCAT22((int)uVar8,uVar14);
  uVar12 = pass1_1008_4772((Struct76 *)CONCAT13((char)(uVar8 >> 0x8),CONCAT12((char)uVar8,uVar14)));
  uVar1 = (uVar12 >> 0x10);
  uVar4 = (u32)((int)uVar12 + 0x4);
  uVar2 = (u32)((int)uVar12 + 0x8);
  IVar3 = GetSystemMetrics16(SM_CYCAPTION);
  y = -(IVar3 + -0xc1);
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  iStack72 = (i16)uVar2;
  y_00 = 0xc5 - (IVar4 - iStack72);
  MoveTo16(y,0x82,hdc16_2c);
  iStack68 = (i16)uVar4;
  x = iStack68 * 0xa + 0x85;
  LineTo16(y,x,hdc16_2c);
  LineTo16(y_00,x,hdc16_2c);
  LineTo16(y_00,0x82,hdc16_2c);
  LineTo16(y,0x82,hdc16_2c);
  for (uStack82 = 0x0; puVar1 = &struct_1->field147_0x94, uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 += 0x1) {
    IVar5 = GetSystemMetrics16(SM_CYCAPTION);
    fn_ptr_1 = (code **)((int)*puStack54 + 0x4);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(char)uVar14,(int)uVar8,-(IVar5 + -0xc4));
  }
  if (puStack54 != NULL) {
    if (puStack54 != NULL) {
      fn_ptr_1 = (code **)*puStack54;
      (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,uVar14);
    }
  }
  SelectObject16(handle_00,hdc16_2c);
  DeleteObject16(handle);
  palette_handle_7 = SelectPalette16(0x0,palette_handle_7,hdc16_2c);
  DeleteObject16(palette_handle_7);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paint_struct_2a),struct_1->field6_0x6);
  return;
}



u16 pass1_1040_5cd6(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;
  u32 uVar3;

  uVar3 = pass1_1040_5d12(param_1);
  if (uVar3 != 0x0) {
    iVar1 = ((int)uVar3 + 0x20);
    uVar2 = (param_1 >> 0x10);
    if (((int)param_1 + 0x9a) != iVar1) {
      ((int)param_1 + 0x9a) = iVar1;
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Unable to use type for symbol uVar3

u32 pass1_1040_5d12(u32 param_1)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar4;
  astruct_440 *iVar4;
  u16 uVar5;
  u32 uVar6;
  u32 uVar3;

  uVar3 = (u32)((int)param_1 + 0x90);
  uVar5 = ((u32)uVar3 >> 0x10);
  iVar4 = (astruct_440 *)uVar3;
  uVar1 = iVar4->field6_0x6;
  uVar2 = iVar4->field7_0x8;
  uVar4 = uVar2 | uVar1;
  if (uVar4 != 0x0) {
    uVar6 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,uVar1),uVar1,uVar4);
    return uVar6;
  }
  return 0x0;
}



void pass1_1040_5d42(StructB *param_1)

{
  u16 uVar1;
  char cVar2;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;

  uVar5 = pass1_1040_5d12((u32)param_1);
  if (uVar5 != 0x0) {
    uVar1 = ((int)uVar5 + 0xc);
    iVar3 = (int)param_1;
    uVar4 = ((u32)param_1 >> 0x10);
    if (uVar1 == 0x5f) {
      (iVar3 + 0x96) = 0x53;
      return;
    }
    if (uVar1 < 0x60) {
      cVar2 = (char)uVar1;
      if (cVar2 == '(') {
        (iVar3 + 0x96) = 0x54;
        return;
      }
      if (cVar2 == ')') {
        (iVar3 + 0x96) = 0x55;
        return;
      }
      if (cVar2 == ']') {
        (iVar3 + 0x96) = 0x51;
        return;
      }
      if (cVar2 == '^') {
        (iVar3 + 0x96) = 0x52;
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_5dc4(u8 *param_1,StructB *param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  Struct57*paVar7;
  StructB *iVar7;
  u16 unaff_SI;
  u16 uVar8;
  u32 *puVar9;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 uVar10;
  i16 iStack18;

  paVar7 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  puVar9 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe8c,
                           in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  paVar7 = (Struct57*)((u32)paVar7 & 0xffff0000 | (u32)puVar9 >> 0x10);
  uVar3 = puVar9;
  uVar8 = ((u32)param_2 >> 0x10);
  iVar7 = (StructB *)param_2;
  uVar5 = ((u32)puVar9 >> 0x10);
  pass1_1010_a5ca(uVar3,uVar5,uVar3,uVar5,iVar7[0x7].field7_0xe);
  if (uVar3 == 0x0) {
    iVar7[0x7].max_count_field_0x10 = 0x0;
    &iVar7[0x7].field8_0x10 = 0x1;
  }
  if (-0x1 < (int)uVar3) {
    if ((int)iVar7[0x7].field7_0xe < 0x72) {
      uVar10 = 0x31;
    }
    else {
      uVar10 = 0x41;
    }
    puVar9 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,uVar10),in_stack_0000fe8c,
                             in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
    uVar6 = ((u32)puVar9 >> 0x10);
    uVar4 = iVar7[0x7].field7_0xe;
    ppcVar1 = (code **)((int)*puVar9 + 0x14);
    (**ppcVar1)(0x1010,(int)puVar9,uVar6,uVar4,(int)uVar4 >> 0xf);
    if ((uVar6 | uVar4) == 0x0) {
      iStack18 = 0x0;
    }
    else {
      uVar2 = (u32)(uVar4 + 0x16);
      iStack18 = ((int)uVar2 + 0x4);
    }
    if ((iStack18 != 0x0) && (uVar3 != 0x0)) {
      uVar4 = (int)((iStack18 - uVar3) * 0x64) / iStack18;
      uVar6 = uVar4 / 0xa;
      iVar7[0x7].max_count_field_0x10 = uVar6;
      if (0x4 < uVar4 % 0xa) {
        iVar7[0x7].max_count_field_0x10 = uVar6 + 0x1;
      }
    }
  }
  return;
}



i16 pass1_1040_5eaa(StructB *param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  switch((iVar1 + 0x9a)) {
  case 0x0:
  case 0x70:
  case 0x71:
    (iVar1 + 0x98) = 0x0;
    return iVar1;
  case 0x1:
  case 0x2:
    (iVar1 + 0x98) = 0xd;
    return iVar1;
  case 0x3:
    (iVar1 + 0x98) = 0xe;
    return iVar1;
  case 0x4:
  case 0x4b:
    (iVar1 + 0x98) = 0xf;
    break;
  case 0x5:
    (iVar1 + 0x98) = 0x10;
    return iVar1;
  case 0x6:
    (iVar1 + 0x98) = 0x11;
    return iVar1;
  case 0x7:
    (iVar1 + 0x98) = 0x12;
    break;
  case 0x8:
    (iVar1 + 0x98) = 0x13;
    break;
  case 0x9:
  case 0xa:
  case 0xb:
    (iVar1 + 0x98) = 0x14;
    break;
  case 0xc:
    (iVar1 + 0x98) = 0x18;
    break;
  case 0xd:
    (iVar1 + 0x98) = 0x19;
    break;
  case 0xe:
  case 0x76:
    (iVar1 + 0x98) = 0x17;
    break;
  case 0xf:
  case 0x10:
  case 0x11:
    (iVar1 + 0x98) = 0x1a;
    break;
  case 0x12:
    (iVar1 + 0x98) = 0x1b;
    break;
  case 0x13:
    (iVar1 + 0x98) = 0x1c;
    break;
  case 0x14:
    (iVar1 + 0x98) = 0x1d;
    break;
  case 0x15:
  case 0x16:
  case 0x17:
  case 0x18:
  case 0x19:
    (iVar1 + 0x98) = 0x1e;
    break;
  case 0x1a:
    (iVar1 + 0x98) = 0x1f;
    break;
  case 0x1b:
    (iVar1 + 0x98) = 0x20;
    break;
  case 0x1c:
  case 0x1d:
  case 0x1e:
    (iVar1 + 0x98) = 0x21;
    break;
  case 0x1f:
    (iVar1 + 0x98) = 0x22;
    break;
  case 0x20:
    (iVar1 + 0x98) = 0x23;
    break;
  case 0x21:
    (iVar1 + 0x98) = 0x24;
    break;
  case 0x22:
    (iVar1 + 0x98) = 0x25;
    break;
  case 0x23:
  case 0x24:
  case 0x25:
  case 0x26:
  case 0x27:
  case 0x28:
  case 0x29:
  case 0x2a:
  case 0x2b:
    (iVar1 + 0x98) = 0x26;
    break;
  case 0x2c:
    (iVar1 + 0x98) = 0x27;
    break;
  case 0x2d:
    (iVar1 + 0x98) = 0x28;
    break;
  case 0x2e:
  case 0x2f:
  case 0x30:
  case 0x31:
    (iVar1 + 0x98) = 0x29;
    break;
  case 0x32:
  case 0x33:
  case 0x34:
  case 0x35:
  case 0x4d:
    (iVar1 + 0x98) = 0x2a;
    break;
  case 0x36:
    (iVar1 + 0x98) = 0x2b;
    break;
  case 0x37:
  case 0x38:
  case 0x39:
    (iVar1 + 0x98) = 0x2c;
    break;
  case 0x3a:
    (iVar1 + 0x98) = 0x2d;
    break;
  case 0x3b:
  case 0x3c:
    (iVar1 + 0x98) = 0x2e;
    break;
  case 0x3d:
    (iVar1 + 0x98) = 0x2f;
    break;
  case 0x3e:
    (iVar1 + 0x98) = 0x30;
    break;
  case 0x3f:
    (iVar1 + 0x98) = 0x31;
    break;
  case 0x40:
    (iVar1 + 0x98) = 0x32;
    break;
  case 0x41:
    (iVar1 + 0x98) = 0x33;
    break;
  case 0x42:
    (iVar1 + 0x98) = 0x34;
    break;
  case 0x43:
    (iVar1 + 0x98) = 0x35;
    break;
  case 0x44:
    (iVar1 + 0x98) = 0x36;
    break;
  case 0x45:
    (iVar1 + 0x98) = 0x37;
    break;
  case 0x46:
    (iVar1 + 0x98) = 0x38;
    break;
  case 0x47:
    (iVar1 + 0x98) = 0x39;
    break;
  case 0x48:
  case 0x49:
  case 0x4a:
    (iVar1 + 0x98) = 0x3a;
    break;
  case 0x4c:
    (iVar1 + 0x98) = 0x3b;
    break;
  case 0x4e:
    (iVar1 + 0x98) = 0x3c;
    break;
  case 0x4f:
  case 0x50:
    (iVar1 + 0x98) = 0x3d;
    break;
  case 0x51:
  case 0x52:
  case 0x53:
  case 0x54:
  case 0x55:
    (iVar1 + 0x98) = 0x3e;
    break;
  case 0x56:
  case 0x57:
  case 0x58:
  case 0x59:
  case 0x5a:
    (iVar1 + 0x98) = 0x3f;
    break;
  case 0x5b:
    (iVar1 + 0x98) = 0x40;
    break;
  case 0x5c:
  case 0x5d:
  case 0x5e:
    (iVar1 + 0x98) = 0x41;
    break;
  case 0x5f:
  case 0x60:
  case 0x61:
    (iVar1 + 0x98) = 0x42;
    break;
  case 0x62:
  case 0x63:
  case 0x64:
  case 0x65:
  case 0x66:
    (iVar1 + 0x98) = 0x43;
    break;
  case 0x67:
  case 0x68:
    (iVar1 + 0x98) = 0x44;
    break;
  case 0x69:
    (iVar1 + 0x98) = 0x45;
    break;
  case 0x6a:
    (iVar1 + 0x98) = 0x46;
    break;
  case 0x6b:
    (iVar1 + 0x98) = 0x47;
    break;
  case 0x6c:
    (iVar1 + 0x98) = 0x48;
    break;
  case 0x6d:
    (iVar1 + 0x98) = 0x49;
    break;
  case 0x6e:
    (iVar1 + 0x98) = 0x4a;
    break;
  case 0x6f:
    (iVar1 + 0x98) = 0x4b;
    break;
  case 0x74:
    (iVar1 + 0x98) = 0x15;
    break;
  case 0x75:
    (iVar1 + 0x98) = 0x16;
    break;
  case 0x78:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
  case 0x7f:
  case 0x80:
  case 0x81:
  case 0x82:
    (iVar1 + 0x98) = 0x4c;
  }
  return iVar1;
}



StructD * pass1_1040_6360(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffda;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_6402(StructD *param_1,Struct57*param_2,u16 param_3)

{
  code **ppcVar1;
  u16 in_register_0000000a;
  Struct57*paVar2;
  Struct57*iVar2;
  u16 unaff_BP;
  Struct57*uVar2;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1850,param_3);
  uVar2 = (Struct57*)((u32)param_2 >> 0x10);
  iVar2 = (Struct57*)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  param_2->field0_0x0 = 0x67ba;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar3;
  iVar2[0x1].field3_0x6 = ((u32)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*&iVar2[0x1].field2_0x4 + 0x4);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_6470(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x67ba;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (*(i32 *)&iVar1->field_0x92 != 0x0) {
    pass1_1010_1ea6((u32)&iVar1->field_0x92,param_1);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1->field_0x6);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x8e);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_ui_op_1040_64ca(char *param_1,u16 param_2,u32 param_3)

{
  short in_buf_len_5;
  u16 in_register_0000000a;
  Struct57*paVar1;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)0x1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void show_win_1040_65ba(StructD *param_1,StructB *struct_b_param_1,u16 param_3)

{
  LPVOID pvVar1;
  u32 uVar2;
  u16 uVar3;
  Struct57*rect;
  u16 uVar4;
  StructD *uVar5;
  Struct57*paVar5;
  StructB *struct_b_4;
  i16 iVar6;
  u16 unaff_SI;
  i16 unaff_DI;
  u16 uVar7;
  u16 uVar8;
  u16 in_stack_0000fe2a;
  u16 in_stack_0000fe2e;
  u16 in_stack_0000fe7e;
  u16 in_stack_0000ff54;
  u16 in_stack_0000ff58;
  u16 in_stack_0000ff5c;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffac;
  u16 local_22;
  u16 uStack32;
  u16 uStack30;
  u16 uStack28;
  u16 *puStack26;
  i16 iStack10;
  u16 uStack8;
  u32 *puStack6;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar5 = param_1;
  puStack6 = mixed_1010_20ba((Struct57*)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),
                             in_stack_0000fe7e,in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  uVar5 = (StructD *)((u32)uVar5 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack8 = pass1_1010_0898();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar5);
  }
  else {
    uVar5 = (StructD *)((u32)uVar5 & 0xffff0000 | PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack26 = (u16 *)CONCAT22(uVar5,PTR_LOOP_1050_5f2c);
  uVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,uVar5);
  uVar7 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_4 = (StructB *)struct_b_param_1;
  struct_b_4[0x7].field1_0x2 = uVar3;
  struct_b_4[0x7].hwnd_0x6 = (HWND16)uVar5;
  for (iStack10 = 0x1; iStack10 <= (int)uStack8; iStack10 += 0x1) {
    puStack26 = (u16 *)pass1_1010_0946((int)puStack6,(int)((u32)puStack6 >> 0x10),iStack10,(u8 *)uVar5,unaff_DI,
                                       0x1050);
    paVar5 = (Struct57*)((u32)uVar5 & 0xffff0000 | (u32)puStack26 >> 0x10);
    local_22 = *puStack26;
    uStack32 = ((int)puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (Struct57*)&local_22;
    MapDialogRect16((RECT16 *)rect,(HWND16)0x1050);
    mem_op_1000_179c(0x42,paVar5);
    uVar4 = (Struct57*)paVar5 | rect;
    uVar5 = (StructD *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      uVar2 = (u32)&struct_b_4[0x7].field1_0x2;
      (u32)((int)uVar2 + iStack10 * 0x4) = 0x0;
    }
    else {
      pvVar1 = struct_b_4->lpvoid_field_0x8;
      pass1_1008_3bd6((u32)uVar5,rect,(Struct57*)paVar5,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,((int)puStack26 + 0x4)))
                      ,param_3,in_stack_0000fe2a,in_stack_0000fe2e,in_stack_0000ff54,in_stack_0000ff58,in_stack_0000ff5c
                     );
      uVar2 = (u32)&struct_b_4[0x7].field1_0x2;
      uVar8 = ((u32)uVar2 >> 0x10);
      iVar6 = (int)uVar2;
      *(astruct_57 **)(iVar6 + iStack10 * 0x4) = rect;
      (iVar6 + iStack10 * 0x4 + 0x2) = (int)uVar5;
    }
    uVar2 = (u32)&struct_b_4[0x7].field1_0x2;
    uVar8 = ((u32)uVar2 >> 0x10);
    iVar6 = (int)uVar2;
    if (*(i32 *)(iVar6 + iStack10 * 0x4) != 0x0) {
      unaff_DI = (int)puStack26;
      enable_win_1040_9234((u32)(iVar6 + iStack10 * 0x4),*(BOOL16 *)(unaff_DI + 0x6));
    }
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_b_4->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void post_win_msg_1040_672e(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  i16 iVar1;

  if (param_5 == (int)s_vrpal_bmp_1050_183a + 0x7) {
    msg_box_ui_op_1040_64ca(0x0,param_1,CONCAT22(param_3,param_2));
  }
  else {
    if (param_5 == 0x1851) {
      iVar1 = 0x2a;
    }
    else {
      if (param_5 != 0x1852) {
        post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
        return;
      }
      iVar1 = 0x29;
    }
    pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),iVar1);
    PostMessage16(0x0,0x2,0x111,*(HWND16 *)(param_2 + 0x6));
  }
  return;
}



StructD * pass1_1040_6794(StructD *param_1,u8 param_2)

{
  pass1_1040_6470(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_6826(Struct57*param_1,u16 param_2)

{
  Struct57*iVar1;
  Struct57*uVar1;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfda));
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  param_1->field0_0x0 = 0x6f32;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



void pass1_1040_6862(StructD *param_1)

{
  u16 in_stack_0000ffde;

  param_1->address_offset_field_0x0 = 0x6f32;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1

void enable_win_1040_6880(astruct_925 *param_1,param_2: i16)

{
  u32 uVar2;
  HWND16 HVar3;
  astruct_925 *iVar3;
  u16 uVar4;
  u32 uVar1;

  if (param_2 == 0x8) {
    uVar4 = ((u32)param_1 >> 0x10);
    iVar3 = (astruct_925 *)param_1;
    HVar3 = GetDlgItem16(0x107,iVar3->field6_0x6);
    uVar1 = iVar3->field147_0x94;
    EnableWindow16(*(BOOL16 *)((int)uVar1 + 0x24),HVar3);
    HVar3 = GetDlgItem16(0x108,iVar3->field6_0x6);
    uVar2 = iVar3->field147_0x94;
    EnableWindow16(*(BOOL16 *)((int)uVar2 + 0x26),HVar3);
  }
  return;
}



u16 pass1_1040_68d2(u32 *param_1,i16 *param_2,u16 param_3,u16 param_4,param_5: i16)

{
  code **ppcVar1;
  u16 uVar2;

  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((i16 *)CONCAT22(param_3,param_2));
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x80);
    (**ppcVar1)();
  }
  return 0x1;
}



void pass1_1040_692e(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x7c);
  (**ppcVar1)();
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar18
// WARNING: Unable to use type for symbol uVar19
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_win_ui_op_1040_6942
               (Struct57*param_1,u16 param_2,StructB *struct_b_param_1,u16 param_4,u16 param_5)

{
  LPVOID pvVar1;
  code **ppcVar2;
  Struct57*paVar3;
  u32 *hwnd;
  astruct_790 *iVar3;
  u16 uVar4;
  u16 uVar10;
  u16 uVar5;
  Struct57*paVar13;
  StructB *struct_b_6;
  u16 uVar6;
  u16 uVar9;
  u16 uVar7;
  u16 uVar8;
  u32 *puVar15;
  u16 *puVar14;
  DWORD DVar16;
  u16 in_stack_0000fdd4;
  u16 in_stack_0000fdd6;
  u16 in_stack_0000fdd8;
  u16 in_stack_0000fdda;
  u16 in_stack_0000fe32;
  u16 in_stack_0000fefe;
  u16 in_stack_0000ff00;
  u16 in_stack_0000ff02;
  u16 in_stack_0000ff04;
  u16 in_stack_0000ff06;
  u16 in_stack_0000ff08;
  u16 in_stack_0000ff56;
  u16 in_stack_0000ff5c;
  u16 in_stack_0000ff60;
  u8 uVar17;
  u8 uVar20;
  BOOL16 BVar21;
  u16 uVar22;
  char *pcVar23;
  HDC16 hdc;
  u32 local_64;
  u32 uStack96;
  HWND16 HStack92;
  HMENU16 HStack90;
  char local_58 [0x50];
  HDC16 hdc_8;
  Struct57*paStack6;
  u16 uStack4;
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;
  u8 uVar18;
  u8 uVar19;
  u16 in_stack_0000ff8a;
  Struct57*paVar11;
  Struct57*paVar12;
  u32 uVar14;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar15 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x33),in_stack_0000fe32,
                            in_stack_0000ff56,in_stack_0000ff5c,in_stack_0000ff60);
  paVar11 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)puVar15 >> 0x10);
  paVar3 = (Struct57*)puVar15;
  uVar6 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_6 = (StructB *)struct_b_param_1;
  struct_b_6[0x7].max_count_field_0x10 = paVar3;
  struct_b_6[0x7].field5_0xa = (u32 *)((u32)puVar15 >> 0x10);
  ppcVar2 = (code **)((int)*&struct_b_6[0x7].max_count_field_0x10 + 0x4);
  (**ppcVar2)(0x1010,struct_b_6[0x7].max_count_field_0x10,(char)((u32)puVar15 >> 0x10),0x0,struct_b_param_1);
  mem_op_1000_179c(0xa,paVar11);
  uVar10 = paVar11 | paVar3;
  paVar12 = (Struct57*)((u32)paVar11 & 0xffff0000 | (u32)uVar10);
  if (uVar10 == 0x0) {
    (u32)&struct_b_6[0x7].field6_0xc = 0x0;
  }
  else {
    puVar14 = struct_1040_bf3e((astruct_442 *)CONCAT13((char)((u32)paVar11 >> 0x8),CONCAT12((char)paVar11,paVar3)),
                               struct_b_6->lpvoid_field_0x8);
    paVar12 = (Struct57*)((u32)paVar12 & 0xffff0000 | (u32)puVar14 >> 0x10);
    paVar3 = (Struct57*)puVar14;
    struct_b_6[0x7].field6_0xc = (u8 *)paVar3;
    struct_b_6[0x7].field7_0xe = ((u32)puVar14 >> 0x10);
  }
    // WARNING: Load size is inaccurate
  pass1_1040_bfde(struct_b_6[0x7].field6_0xc,&struct_b_6[0x7].max_count_field_0x10);
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = (Struct57*)paVar12 | paVar3;
  paVar11 = (Struct57*)((u32)paVar12 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar11,paVar3,(Struct57*)paVar12,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_6->lpvoid_field_0x8,0x10a),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar10 = (Struct57*)paVar11 | paVar3;
  paVar12 = (Struct57*)((u32)paVar11 & 0xffff0000 | (u32)uVar10);
  if (uVar10 != 0x0) {
    pass1_1008_3bd6((u32)paVar12,paVar3,(Struct57*)paVar11,0x1,0xa0028,0x0,0x820083,
                    CONCAT22(struct_b_6->lpvoid_field_0x8,0x10c),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  BVar21 = 0x0;
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = (Struct57*)paVar12 | paVar3;
  paVar11 = (Struct57*)((u32)paVar12 & 0xffff0000);
  paVar13 = (Struct57*)((u32)paVar11 | (u32)uVar10);
  if (uVar10 == 0x0) {
    paVar3 = NULL;
  }
  else {
    pvVar1 = struct_b_6->lpvoid_field_0x8;
    pass1_1008_3bd6((u32)paVar13,paVar3,(Struct57*)paVar12,0x1,0xa00aa,0x101,0xff0100,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0x107)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    paVar11 = paVar13;
  }
  uStack4 = SUB42(paVar11,0x0);
  paStack6 = paVar3;
  enable_win_1040_9234(CONCAT13((char)((u32)paVar11 >> 0x8),CONCAT12((char)paVar11,paVar3)),BVar21);
  BVar21 = 0x0;
  mem_op_1000_179c(0x42,paVar11);
  uVar5 = (Struct57*)paVar11 | paVar3;
  uVar14 = (u32)paVar11 & 0xffff0000 | (u32)uVar5;
  if (uVar5 == 0x0) {
    paVar3 = NULL;
    uStack4 = 0x0;
  }
  else {
    pvVar1 = struct_b_6->lpvoid_field_0x8;
    pass1_1008_3bd6(uVar14,paVar3,(Struct57*)paVar11,0x1,0xa00c2,0x101,0xff0100,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0x108)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    uStack4 = uVar14;
  }
  paStack6 = paVar3;
  enable_win_1040_9234(CONCAT13((char)(uStack4 >> 0x8),CONCAT12((char)uStack4,paVar3)),BVar21);
  hdc_8 = GetDC16((HWND16)struct_b_6->lpvoid_field_0x8);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x50,local_58,(short)0x1050);
  uVar22 = SUB42(0x1050,0x0);
  uVar17 = SUB21(local_58,0x0);
  uVar20 = (u8)(local_58 >> 0x8);
  hdc = hdc_8;
  uVar10 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_58));
  DVar16 = GetTextExtent16(uVar10,(LPCSTR)CONCAT22(uVar22,CONCAT11(uVar20,uVar17)),hdc);
  HStack90 = (HMENU16)(DVar16 >> 0x10);
  HStack92 = (HWND16)DVar16;
  CreateWindow16(0x0,(void *)CONCAT22(0x7cd,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_6->lpvoid_field_0x8,HStack90,
                 HStack92,0xad,0x22,0x0,(int)s_Rebel_1050_4ffc + 0x4,(char *)CONCAT13(0x10,CONCAT12(0x50,local_58)),
                 s_static_1050_5d84);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x50,local_58,(short)0x1050);
  uVar18 = (u8)hdc_8;
  uVar19 = (u8)(hdc_8 >> 0x8);
  pcVar23 = local_58;
  uVar22 = SUB42(0x1050,0x0);
  uVar10 = str_op_1000_3da4((char *)CONCAT13(0x10,CONCAT12(0x50,pcVar23)));
  DVar16 = GetTextExtent16(uVar10,(LPCSTR)CONCAT22(uVar22,pcVar23),CONCAT11(uVar19,uVar18));
  HStack90 = (HMENU16)(DVar16 >> 0x10);
  HStack92 = (HWND16)DVar16;
  ReleaseDC16(hdc_8,(HWND16)struct_b_6->lpvoid_field_0x8);
  CreateWindow16(0x0,(void *)CONCAT22(0x7ce,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_6->lpvoid_field_0x8,HStack90,
                 HStack92,0xc5,0x22,0x0,(int)s_Rebel_1050_4ffc + 0x4,(char *)CONCAT22(0x1050,local_58),
                 s_static_1050_5d8b);
  local_64 = 0x5a000a;
  uStack96 = 0x140050;
  hwnd = &local_64;
  create_window_1040_6eae((u32)struct_b_param_1,0x1,(astruct_859 *)CONCAT22(0x1050,hwnd),0x5eb,0xfd);
  local_64 = local_64 & 0xffff | (u32)(local_64 + 0x14) << 0x10;
  create_window_1040_6eae((u32)struct_b_param_1,0x0,(astruct_859 *)CONCAT22(0x1050,&local_64),0x5ec,0xfe);
  local_64 = local_64 & 0xffff | (u32)(local_64 + 0x14) << 0x10;
  create_window_1040_6eae((u32)struct_b_param_1,0x0,(astruct_859 *)CONCAT22(0x1050,&local_64),0x5ee,0xff);
  SendMessage16(0x0,0x1,0x401,(HWND16)hwnd);
  uVar1 = (u32)&struct_b_6[0x7].max_count_field_0x10;
  iVar3 = (astruct_790 *)uVar1;
  iVar3 = (astruct_790 *)&iVar3->field_0xa;
  uVar9 = ((uVar1 & 0xffff0000) >> 0x10);
  SetWindowPos16(0x40,iVar3->field14_0x10,iVar3->field13_0xe,iVar3->field12_0xc,
                 *(INT16 *)(uVar1 & 0xffff0000 | ZEXT24(iVar3)),0x0,(HWND16)struct_b_6->lpvoid_field_0x8);
  DAT_1050_0ecc = 0x0;
  uVar2 = (u32)&struct_b_6[0x7].max_count_field_0x10;
  ppcVar2 = (code **)((int)*&struct_b_6[0x7].max_count_field_0x10 + 0x10);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  pass1_1010_2ee2(&struct_b_6[0x7].max_count_field_0x10);
  PostMessage16(0x0,0x10a,0x111,(HWND16)struct_b_6->lpvoid_field_0x8);
  return;
}



void pass1_1040_6cac(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_1ea6((u32)(iVar4 + 0x94),(StructD *)(param_1 & 0xffff | (u32)uVar5 << 0x10));
  puVar1 = (u32 *)(iVar4 + 0x98);
  uVar2 = (iVar4 + 0x9a);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  (u32)(iVar4 + 0x98) = 0x0;
  (u32)(iVar4 + 0x94) = 0x0;
  return;
}



u16 pass1_1040_6cfa(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_6d1a(astruct_897 *param_1,u16 param_2,u16 param_3,u32 param_4)

{
  code **ppcVar1;
  Struct27 *paVar2;
  u8 *in_DX;
  RECT16 local_a;
  i16 iStack6;
  astruct_896 *iStack4;
  astruct_895 *iVar3;

  switch(param_4) {
  case 0xfa:
    ppcVar1 = (code **)((int)(u32)param_1->field144_0x94 + 0x18);
    (**ppcVar1)();
    break;
  default:
    pass1_1040_b54a(in_DX,(astruct_903 *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)),param_3,
                    param_4);
    return;
  case 0xfd:
    if (DAT_1050_0ecc == 0x0) {
      return;
    }
    DAT_1050_0ecc = 0x0;
    goto LAB_1040_6deb;
  case 0xfe:
    if (DAT_1050_0ecc == 0x1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
    goto LAB_1040_6deb;
  case 0xff:
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;
LAB_1040_6deb:
    paVar2 = param_1->field144_0x94;
    ppcVar1 = (code **)((int)(u32)param_1->field144_0x94 + 0x10);
    (**ppcVar1)((int)&PTR_LOOP_1050_1040,(int)paVar2,(int)((u32)paVar2 >> 0x10));
    pass1_1010_2ee2((u32 *)param_1->field144_0x94);
    PostMessage16(0x0,0x10a,0x111,param_1->field6_0x6);
    break;
  case 0x107:
    iVar3 = NULL;
    goto LAB_1040_6e48;
  case 0x108:
    iVar3 = (astruct_895 *)((int)&PTR_LOOP_1050_0000 + 0x1);
LAB_1040_6e48:
    win_ui_op_1010_3202(param_1->field144_0x94,(int)iVar3);
    break;
  case 0x10a:
    GetClientRect16(&local_a,(HWND16)0x1050);
    paVar2 = param_1->field144_0x94;
    local_a.y += 0x3;
    local_a.x = ((int)paVar2 + 0x1a) + -0x9;
    iStack6 += -0x3;
    iStack4 = iStack4 + -0x3;
    InvalidateRect16(0x1,&local_a,(HWND16)0x1050);
    unk_destroy_win_op_1010_2fa0((astruct_873 *)param_1->field144_0x94);
    pass1_1010_32c0((u32)param_1->field144_0x94,0x0);
    pass1_1010_2ee2((u32 *)param_1->field144_0x94);
    break;
  case 0x10c:
    DestroyWindow16(param_1->field6_0x6);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void create_window_1040_6eae(param_1: u32,i16 param_2,astruct_859 *pstruct_param_3,u16 param_4,u16 param_5)

{
  astruct_859 *pstruct_1;
  u16 uVar1;
  char *window_name;
  HINSTANCE16 h_instance;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar1 = ((u32)pstruct_param_3 >> 0x10);
  pstruct_1 = (astruct_859 *)pstruct_param_3;
  CreateWindow16(0x0,(void *)CONCAT22(param_5,HINSTANCE16_1050_038c),*(HINSTANCE16 *)((int)param_1 + 0x6),
                 pstruct_1->field4_0x6,pstruct_1->field3_0x4,pstruct_1->field2_0x2,*(INT16 *)pstruct_param_3,
                 (INT16)_h_instance,(INT16)((u32)_h_instance >> 0x10),window_name,s_button_1050_5d92);
  return;
}



StructD * pass1_1040_6f0c(StructD *param_1,u8 param_2)

{
  pass1_1040_6862(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_6fb6(Struct57*param_1,u16 param_2)

{
  Struct57*iVar1;
  Struct57*uVar1;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfd9));
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  param_1->field0_0x0 = 0x76a4;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



void enable_win_1040_6ff2(param_1: u32,param_2: i16)

{
  u32 uVar1;
  HWND16 HVar2;
  astruct_926 *iVar3;
  u16 uVar3;

  if (param_2 == 0x8) {
    uVar3 = (param_1 >> 0x10);
    iVar3 = (astruct_926 *)param_1;
    HVar2 = GetDlgItem16(0x107,iVar3->field6_0x6);
    uVar1 = iVar3->field147_0x94;
    EnableWindow16(*(BOOL16 *)((int)uVar1 + 0x24),HVar2);
    HVar2 = GetDlgItem16(0x108,iVar3->field6_0x6);
    uVar1 = iVar3->field147_0x94;
    EnableWindow16(*(BOOL16 *)((int)uVar1 + 0x26),HVar2);
  }
  return;
}



u16 pass1_1040_7044(u32 *param_1,i16 *param_2,u16 param_3,u16 param_4,param_5: i16)

{
  code **ppcVar1;
  u16 uVar2;

  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((i16 *)CONCAT22(param_3,param_2));
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x80);
    (**ppcVar1)();
  }
  return 0x1;
}



void pass1_1040_70a0(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x7c);
  (**ppcVar1)();
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar14
// WARNING: Unable to use type for symbol uVar15
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_win_ui_op_1040_70b4
               (Struct57*param_1,u16 param_2,StructB *struct_b_param_1,u16 param_4,u16 param_5)

{
  LPVOID pvVar1;
  Struct57*paVar2;
  u16 uVar3;
  u16 count;
  u32 *hwnd;
  astruct_792 *iVar3;
  u16 uVar4;
  u16 uVar5;
  Struct57*paVar5;
  Struct57*paVar7;
  StructB *struct_b_5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u32 *puVar10;
  u16 *puVar11;
  u32 DVar11;
  u32 DVar12;
  u16 in_stack_0000fdd4;
  u16 in_stack_0000fdd6;
  u16 in_stack_0000fdd8;
  u16 in_stack_0000fdda;
  u16 in_stack_0000fe32;
  u16 in_stack_0000fefe;
  u16 in_stack_0000ff00;
  u16 in_stack_0000ff02;
  u16 in_stack_0000ff04;
  u16 in_stack_0000ff06;
  u16 in_stack_0000ff08;
  u16 in_stack_0000ff56;
  u16 in_stack_0000ff5c;
  u16 in_stack_0000ff60;
  u8 uVar11;
  u8 uVar12;
  BOOL16 BVar13;
  u16 uVar16;
  char *pcVar17;
  HDC16 hdc;
  u32 local_64;
  u32 uStack96;
  HWND16 HStack92;
  HMENU16 HStack90;
  char local_58 [0x50];
  HDC16 HStack8;
  Struct57*paStack6;
  u16 uStack4;
  u32 uVar1;
  u32 uVar2;
  u8 uVar14;
  u8 uVar15;
  u16 in_stack_0000ff8a;
  Struct57*paVar6;
  u32 uVar9;
  code **fn_ptr_1;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar10 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x34),in_stack_0000fe32,
                            in_stack_0000ff56,in_stack_0000ff5c,in_stack_0000ff60);
  paVar5 = (Struct57*)((u32)param_1 & 0xffff0000 | (u32)puVar10 >> 0x10);
  paVar2 = (Struct57*)puVar10;
  uVar6 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_5 = (StructB *)struct_b_param_1;
  struct_b_5[0x7].max_count_field_0x10 = paVar2;
  struct_b_5[0x7].field5_0xa = (u32 *)((u32)puVar10 >> 0x10);
  fn_ptr_1 = (code **)((int)*&struct_b_5[0x7].max_count_field_0x10 + 0x4);
  (**fn_ptr_1)(0x1010,struct_b_5[0x7].max_count_field_0x10,(char)((u32)puVar10 >> 0x10),0x0,struct_b_param_1);
  mem_op_1000_179c(0xa,paVar5);
  uVar4 = paVar5 | paVar2;
  paVar6 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
  if (uVar4 == 0x0) {
    (u32)&struct_b_5[0x7].field6_0xc = 0x0;
  }
  else {
    puVar11 = struct_1040_bf3e((astruct_442 *)CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,paVar2)),
                               struct_b_5->lpvoid_field_0x8);
    paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)puVar11 >> 0x10);
    paVar2 = (Struct57*)puVar11;
    struct_b_5[0x7].field6_0xc = (u8 *)paVar2;
    struct_b_5[0x7].field7_0xe = ((u32)puVar11 >> 0x10);
  }
    // WARNING: Load size is inaccurate
  pass1_1040_bfde(struct_b_5[0x7].field6_0xc,&struct_b_5[0x7].max_count_field_0x10);
  mem_op_1000_179c(0x42,paVar6);
  uVar4 = (Struct57*)paVar6 | paVar2;
  paVar5 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
  if (uVar4 != 0x0) {
    pass1_1008_3bd6((u32)paVar5,paVar2,(Struct57*)paVar6,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_5->lpvoid_field_0x8,0x10a),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  mem_op_1000_179c(0x42,paVar5);
  uVar4 = (Struct57*)paVar5 | paVar2;
  paVar6 = (Struct57*)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
  if (uVar4 != 0x0) {
    pass1_1008_3bd6((u32)paVar6,paVar2,(Struct57*)paVar5,0x1,0xa0028,0x0,0x820083,
                    CONCAT22(struct_b_5->lpvoid_field_0x8,0x10c),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  BVar13 = 0x0;
  mem_op_1000_179c(0x42,paVar6);
  uVar4 = (Struct57*)paVar6 | paVar2;
  paVar5 = (Struct57*)((u32)paVar6 & 0xffff0000);
  paVar7 = (Struct57*)((u32)paVar5 | (u32)uVar4);
  if (uVar4 == 0x0) {
    paVar2 = NULL;
  }
  else {
    pvVar1 = struct_b_5->lpvoid_field_0x8;
    pass1_1008_3bd6((u32)paVar7,paVar2,(Struct57*)paVar6,0x1,0xa00aa,0x101,0xff0100,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0x107)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    paVar5 = paVar7;
  }
  uStack4 = SUB42(paVar5,0x0);
  paStack6 = paVar2;
  enable_win_1040_9234(CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,paVar2)),BVar13);
  BVar13 = 0x0;
  mem_op_1000_179c(0x42,paVar5);
  uVar5 = (Struct57*)paVar5 | paVar2;
  uVar9 = (u32)paVar5 & 0xffff0000 | (u32)uVar5;
  if (uVar5 == 0x0) {
    paVar2 = NULL;
    uStack4 = 0x0;
  }
  else {
    pvVar1 = struct_b_5->lpvoid_field_0x8;
    pass1_1008_3bd6(uVar9,paVar2,(Struct57*)paVar5,0x1,0xa00c2,0x101,0xff0100,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0x108)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    uStack4 = uVar9;
  }
  paStack6 = paVar2;
  enable_win_1040_9234(CONCAT13((char)(uStack4 >> 0x8),CONCAT12((char)uStack4,paVar2)),BVar13);
  HStack8 = GetDC16((HWND16)struct_b_5->lpvoid_field_0x8);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x50,local_58,(short)0x1050);
  uVar16 = SUB42(0x1050,0x0);
  uVar11 = SUB21(local_58,0x0);
  uVar12 = (u8)(local_58 >> 0x8);
  hdc = HStack8;
  uVar3 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_58));
  DVar11 = GetTextExtent16(uVar3,(LPCSTR)CONCAT22(uVar16,CONCAT11(uVar12,uVar11)),hdc);
  HStack90 = (HMENU16)(DVar11 >> 0x10);
  HStack92 = (HWND16)DVar11;
  CreateWindow16(0x0,(void *)CONCAT22(0x7cd,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_5->lpvoid_field_0x8,HStack90,
                 HStack92,0xad,0x22,0x0,(int)s_Rebel_1050_4ffc + 0x4,(char *)CONCAT13(0x10,CONCAT12(0x50,local_58)),
                 s_static_1050_5d9a);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x50,local_58,(short)0x1050);
  uVar14 = (u8)HStack8;
  uVar15 = (u8)(HStack8 >> 0x8);
  pcVar17 = local_58;
  uVar16 = SUB42(0x1050,0x0);
  count = str_op_1000_3da4((char *)CONCAT13(0x10,CONCAT12(0x50,pcVar17)));
  DVar12 = GetTextExtent16(count,(LPCSTR)CONCAT22(uVar16,pcVar17),CONCAT11(uVar15,uVar14));
  HStack90 = (HMENU16)(DVar12 >> 0x10);
  HStack92 = (HWND16)DVar12;
  ReleaseDC16(HStack8,(HWND16)struct_b_5->lpvoid_field_0x8);
  CreateWindow16(0x0,(void *)CONCAT22(0x7ce,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_5->lpvoid_field_0x8,HStack90,
                 HStack92,0xc5,0x22,0x0,(int)s_Rebel_1050_4ffc + 0x4,(char *)CONCAT22(0x1050,local_58),
                 s_static_1050_5da1);
  local_64 = 0x5a000a;
  uStack96 = 0x140050;
  hwnd = &local_64;
  create_window_1040_7620(struct_b_param_1,0x1,(astruct_860 *)CONCAT22(0x1050,hwnd),0x5eb,0xfd);
  local_64 = local_64 & 0xffff | (u32)(local_64 + 0x14) << 0x10;
  create_window_1040_7620(struct_b_param_1,0x0,(astruct_860 *)CONCAT22(0x1050,&local_64),0x5ed,0xfe);
  local_64 = local_64 & 0xffff | (u32)(local_64 + 0x14) << 0x10;
  create_window_1040_7620(struct_b_param_1,0x0,(astruct_860 *)CONCAT22(0x1050,&local_64),0x5ef,0xff);
  SendMessage16(0x0,0x1,0x401,(HWND16)hwnd);
  uVar1 = (u32)&struct_b_5[0x7].max_count_field_0x10;
  iVar3 = (astruct_792 *)uVar1;
  iVar3 = (astruct_792 *)&iVar3->field_0xa;
  uVar16 = ((uVar1 & 0xffff0000) >> 0x10);
  SetWindowPos16(0x40,iVar3->field14_0x10,iVar3->field13_0xe,iVar3->field12_0xc,
                 *(INT16 *)(uVar1 & 0xffff0000 | ZEXT24(iVar3)),0x0,(HWND16)struct_b_5->lpvoid_field_0x8);
  DAT_1050_0ecc = 0x0;
  uVar2 = (u32)&struct_b_5[0x7].max_count_field_0x10;
  fn_ptr_1 = (code **)((int)*&struct_b_5[0x7].max_count_field_0x10 + 0x10);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  pass1_1010_2ee2(&struct_b_5[0x7].max_count_field_0x10);
  PostMessage16(0x0,0x10a,0x111,(HWND16)struct_b_5->lpvoid_field_0x8);
  return;
}



void pass1_1040_741e(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_1ea6((u32)(iVar4 + 0x94),(StructD *)(param_1 & 0xffff | (u32)uVar5 << 0x10));
  puVar1 = (u32 *)(iVar4 + 0x98);
  uVar2 = (iVar4 + 0x9a);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  (u32)(iVar4 + 0x98) = 0x0;
  (u32)(iVar4 + 0x94) = 0x0;
  return;
}



u16 pass1_1040_746c(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_cleanup_op_1040_748c(u8 *param_1,astruct_898 *param_2,u16 param_3,u16 param_4,u32 param_5)

{
  u32 *puVar1;
  i16 iVar2;
  RECT16 local_a;
  i16 iStack6;
  i16 iStack4;
  code **fn_ptr_1;

  switch(param_5) {
  case 0xfa:
    fn_ptr_1 = (code **)((int)*param_2->field144_0x94 + 0x18);
    (**fn_ptr_1)();
    break;
  default:
    pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                    param_5);
    return;
  case 0xfd:
    if (DAT_1050_0ecc == 0x0) {
      return;
    }
    DAT_1050_0ecc = 0x0;
    goto LAB_1040_755d;
  case 0xfe:
    if (DAT_1050_0ecc == 0x1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
    goto LAB_1040_755d;
  case 0xff:
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;
LAB_1040_755d:
    puVar1 = param_2->field144_0x94;
    fn_ptr_1 = (code **)((int)*param_2->field144_0x94 + 0x10);
    (**fn_ptr_1)((int)&PTR_LOOP_1050_1040,(int)puVar1,(int)((u32)puVar1 >> 0x10));
    pass1_1010_2ee2(param_2->field144_0x94);
    PostMessage16(0x0,0x10a,0x111,param_2->field6_0x6);
    break;
  case 0x107:
    iVar2 = 0x0;
    goto LAB_1040_75ba;
  case 0x108:
    iVar2 = 0x1;
LAB_1040_75ba:
    win_ui_op_1010_3202((Struct27 *)param_2->field144_0x94,iVar2);
    break;
  case 0x10a:
    GetClientRect16(&local_a,(HWND16)0x1050);
    puVar1 = param_2->field144_0x94;
    local_a.y += 0x3;
    local_a.x = ((int)puVar1 + 0x1a) + -0x9;
    iStack6 += -0x3;
    iStack4 += -0x3;
    InvalidateRect16(0x1,&local_a,(HWND16)0x1050);
    unk_destroy_win_op_1010_2fa0((astruct_873 *)param_2->field144_0x94);
    pass1_1010_32c0((u32)param_2->field144_0x94,0x0);
    pass1_1010_2ee2(param_2->field144_0x94);
    break;
  case 0x10c:
    DestroyWindow16(param_2->field6_0x6);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void create_window_1040_7620(param_1: u32,i16 param_2,astruct_860 *pstruct_param_3,u16 param_4,u16 param_5)

{
  astruct_860 *iVar1;
  u16 uVar1;
  char *window_name;
  HINSTANCE16 h_instance;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar1 = ((u32)pstruct_param_3 >> 0x10);
  iVar1 = (astruct_860 *)pstruct_param_3;
  CreateWindow16(0x0,(void *)CONCAT22(param_5,HINSTANCE16_1050_038c),*(HINSTANCE16 *)((int)param_1 + 0x6),
                 iVar1->field4_0x6,iVar1->field3_0x4,iVar1->field2_0x2,*(INT16 *)pstruct_param_3,(INT16)_h_instance,
                 (INT16)((u32)_h_instance >> 0x10),window_name,s_button_1050_5da8);
  return;
}



StructD * pass1_1040_767e(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffda;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void get_sys_metrics_1040_7728(Struct57*param_1,u16 param_2,param_3: u32,u16 param_4,u16 param_5)

{
  INT16 IVar1;
  Struct57*iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (Struct57*)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar2->field1_0x2 = 0x1008;
  param_1->field0_0x0 = 0x3aa8;
  iVar2->field1_0x2 = 0x1008;
  iVar2->field2_0x4 = 0x0;
  iVar2->field3_0x6 = 0x0;
  iVar2->field4_0x8 = param_5;
  iVar2->field5_0xa = param_4;
  (u32)&iVar2->field6_0xc = 0x0;
  iVar2->field78_0x60 = 0x0;
  iVar2->field79_0x62 = 0x0;
  iVar2->field80_0x64 = 0x0;
  iVar2->field81_0x66 = 0x0;
  iVar2->field82_0x68 = 0x0;
  iVar2->field83_0x6a = param_3;
  iVar2->field84_0x6e = param_2;
  iVar2->field85_0x70 = 0x0;
  iVar2->field86_0x74 = 0x0;
  iVar2->field87_0x76 = 0x0;
  iVar2->field88_0x78 = 0x0;
  iVar2->field105_0x8a = 0x0;
  iVar2->field106_0x8c = 0x0;
  param_1->field0_0x0 = 0x840c;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field8_0x10)),(char *)0x10505db0);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x7a)),NULL,0x8);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x82)),NULL,0x8);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  iVar2->field79_0x62 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  iVar2->field80_0x64 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CYBORDER);
  iVar2->field81_0x66 = IVar1;
  return;
}



void ui_cleanup_op_1040_782c(StructD *param_1)

{
  u32 *puVar2;
  u16 uVar3;
  StructD *struct_1;
  u16 uVar2;
  u16 uVar1;
  u32 *puVar1;
  code **fn_ptr_1;

  uVar2 = ((u32)param_1 >> 0x10);
  struct_1 = (int)param_1;
  param_1->address_offset_field_0x0 = 0x840c;
  ((int)struct_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  puVar2 = (u32 *)((int)struct_1 + 0x70);
  uVar3 = ((int)struct_1 + 0x72);
  if ((uVar3 | puVar2) != 0x0) {
    fn_ptr_1 = (code **)*puVar2;
    (**fn_ptr_1)();
  }
  if (((int)struct_1 + 0x4) != 0x0) {
    DeleteObject16(*(HGDIOBJ16 *)((int)struct_1 + 0x4));
    ((int)struct_1 + 0x4) = 0x0;
  }
  if (((int)struct_1 + 0x68) != 0x0) {
    DestroyMenu16(*(HMENU16 *)((int)struct_1 + 0x68));
  }
  RemoveProp16(s_thisLo_1050_5db1,*(HWND16 *)((int)struct_1 + 0x6));
  RemoveProp16(s_thisHi_1050_5db8,*(HWND16 *)((int)struct_1 + 0x6));
  RemoveProp16(s_procLo_1050_5dbf,*(HWND16 *)((int)struct_1 + 0x6));
  RemoveProp16(s_procHi_1050_5dc6,*(HWND16 *)((int)struct_1 + 0x6));
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)struct_1 + 0x2) = 0x1008;
  return;
}



void pass1_1040_78de(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address



u16 pass1_1040_79c0(u32 *param_1,i16 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  code **ppcVar1;
  char cVar2;
  u16 uVar3;

  if (param_5 == 0xa1) {
    ppcVar1 = (code **)((int)*param_1 + 0x38);
    uVar3 = (**ppcVar1)();
    return uVar3;
  }
  if (param_5 < 0xa2) {
    if (param_5 == 0x85) {
      ppcVar1 = (code **)((int)*param_1 + 0x1c);
      (**ppcVar1)();
      return 0x1;
    }
    if (param_5 < 0x86) {
      cVar2 = (char)param_5;
      if (cVar2 == '\x02') {
        ppcVar1 = (code **)((int)*param_1 + 0x24);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\f') {
        ppcVar1 = (code **)((int)*param_1 + 0x18);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (code **)((int)*param_1 + 0x60);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (cVar2 == '+') {
        if (*param_2 != 0x4) {
          return 0x1;
        }
        win_ui_get_prop_op_1040_9566((i16 *)CONCAT22(param_3,param_2));
        return 0x1;
      }
    }
  }
  else {
    if (param_5 == 0x114) {
      ppcVar1 = (code **)((int)*param_1 + 0x58);
      uVar3 = (**ppcVar1)();
      return uVar3;
    }
    if (param_5 < 0x115) {
      if (param_5 == 0x104) {
        ppcVar1 = (code **)((int)*param_1 + 0x30);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x111) {
        ppcVar1 = (code **)((int)*param_1 + 0x10);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
    }
    else {
      if (param_5 == 0x115) {
        ppcVar1 = (code **)((int)*param_1 + 0x54);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x201) {
        ppcVar1 = (code **)((int)*param_1 + 0x44);
        (**ppcVar1)();
        return 0x1;
      }
      if (param_5 == 0x204) {
        ppcVar1 = (code **)((int)*param_1 + 0x28);
        (**ppcVar1)();
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 post_win_msg_1040_7b3c(StructC *param_1,u16 param_2,u16 param_3,param_4: i16)

{
  code **ppcVar1;

  if ((param_4 == 0x1) || (param_4 == 0x2)) {
    ppcVar1 = (code **)((int)(u32)param_1 + 0x14);
    (**ppcVar1)();
  }
  else if (param_4 == 0x6f) {
    ppcVar1 = (code **)((int)(u32)param_1 + 0x2c);
    (**ppcVar1)();
  }
  else {
    if (param_4 != 0x12e) {
      return 0x0;
    }
    PostMessage16(0x0,0xf060,0x112,*(HWND16 *)((int)param_1 + 0x6));
  }
  return 0x1;
}



void destroy_win_1040_7b98(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x74) == 0x0) {
    DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void draw_op_1040_7bb2(astruct_14 *in_struct_1)

{
  BOOL16 is_iconic;
  i16 x;
  i16 y1;
  i16 iVar5;
  HPEN16 pen_handle;
  HGDIOBJ16 obj_handle;
  i16 y;
  HGDIOBJ16 brush_handle;
  HANDLE16 handle;
  u16 count;
  i16 count_00;
  astruct_14 *struct_1;
  RECT16 *pRVar1;
  let mut win_long: i32;
  DWORD DVar2;
  RECT16 *rect;
  i16 count_01;
  i16 local_rect_12;
  i16 iStack16;
  i16 iStack14;
  i16 iStack12;
  HPALETTE16 HStack10;
  u32 uStack8;
  HDC16 win_hdc16_4;
  u8 uVar5;
  u8 uVar6;
  u32 uVar7;
  u8 *pcVar3;
  u16 uVar3;
  code **func_ptr_1;
  HDC16 hdc16_dev_ctx_1;

  pRVar1 = (RECT16 *)((u32)in_struct_1 >> 0x10);
  struct_1 = (astruct_14 *)in_struct_1;
  is_iconic = IsIconic16(struct_1->hwnd16_field6_0x6);
  if (is_iconic == 0x0) {
    win_hdc16_4 = GetWindowDC16(struct_1->hwnd16_field6_0x6);
    func_ptr_1 = (code **)((int)(u32)in_struct_1 + 0x68);
    uStack8 = (**func_ptr_1)((int)s_tile2_bmp_1050_1538,in_struct_1);
    if ((astruct_13 *)uStack8 != NULL) {
      HStack10 = palette_op_1008_4e08
                           ((HPALETTE16)&win_hdc16_4,(uStack8 >> 0x10) | uStack8,(astruct_13 *)uStack8,
                            (HDC16 *)CONCAT22(0x1050,&win_hdc16_4));
      GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,&local_rect_12)),struct_1->hwnd16_field6_0x6);
      x = (iStack14 - local_rect_12) + -0x1;
      y1 = (iStack12 - iStack16) + -0x1;
      iVar5 = (-(struct_1->field95_0x60 == 0x0) & 0x1e) + 0x25;
      pen_handle = CreatePen16(CONCAT13(0x1,(u16)iVar5),0x0,0x0);
      obj_handle = SelectObject16(pen_handle,win_hdc16_4);
      MoveTo16(0x0,0x0,win_hdc16_4);
      LineTo16(0x0,x,win_hdc16_4);
      LineTo16(y1,x,win_hdc16_4);
      LineTo16(y1,0x0,win_hdc16_4);
      LineTo16(0x0,0x0,win_hdc16_4);
      win_i32 = GetWindowLong16(-0x10,struct_1->hwnd16_field6_0x6);
      if (((win_i32 & 0x800000U) != 0x0) && ((win_i32 & 0x400000U) != 0x0)) {
        y = struct_1->field96_0x62 - struct_1->field98_0x66;
        MoveTo16(y,0x0,win_hdc16_4);
        LineTo16(y,x,win_hdc16_4);
        struct_1->field115_0x7a = struct_1->field97_0x64;
        struct_1->field116_0x7c = struct_1->field98_0x66;
        struct_1->field117_0x7e = x;
        struct_1->field118_0x80 = struct_1->field96_0x62 - struct_1->field98_0x66;
        rect = pRVar1;
        hdc16_dev_ctx_1 = win_hdc16_4;
        brush_handle = GetStockObject16(BLACK_BRUSH);
        FillRect16(brush_handle,rect,hdc16_dev_ctx_1);
        if (struct_1->field112_0x76 != 0x0) {
          draw_op_1040_82ee(in_struct_1);
        }
        if (struct_1->field15_0x10 != '\0') {
          handle = GetProp16(s_hfont_1050_5de9,struct_1->hwnd16_field6_0x6);
          if (handle != 0x0) {
            SelectObject16(handle,win_hdc16_4);
          }
          SetBkColor16(0x0,win_hdc16_4);
          count_01 = 0x100;
          hdc16_dev_ctx_1 = win_hdc16_4;
          SetTextColor16(CONCAT22(0x100,iVar5),win_hdc16_4);
          count = lstrlen16((char *)CONCAT22(hdc16_dev_ctx_1,count_01));
          DVar2 = GetTextExtent16(count,(LPCSTR)CONCAT22(count_01,win_hdc16_4),win_hdc16_4);
          count_00 = ((struct_1->field117_0x7e - struct_1->field115_0x7a) / 0x2 - (int)DVar2 / 0x2) +
                     struct_1->field115_0x7a;
          brush_handle = (struct_1->field118_0x80 - struct_1->field116_0x7c) / 0x2 - (int)(DVar2 >> 0x10) / 0x2;
          TextOut16(count_01,(char *)CONCAT22(count_01,win_hdc16_4),brush_handle,count_00,win_hdc16_4);
          if (count_00 != 0x0) {
            SelectObject16(brush_handle,win_hdc16_4);
          }
        }
      }
      func_ptr_1 = (code **)((int)(u32)in_struct_1 + 0x64);
      (**func_ptr_1)((int)s_tile2_bmp_1050_1538,in_struct_1,uStack8,win_hdc16_4);
      HStack10 = SelectPalette16(0x0,HStack10,win_hdc16_4);
      DeleteObject16(HStack10);
      SelectObject16(obj_handle,win_hdc16_4);
      DeleteObject16(pen_handle);
    }
    ReleaseDC16(win_hdc16_4,struct_1->hwnd16_field6_0x6);
    return;
  }
  return;
}



u32 set_text_bk_color_1040_7e5e(u32 *param_1,u16 param_2,u16 param_3,HDC16 param_4)

{
  HGDIOBJ16 HVar1;
  astruct_936 *iVar2;
  INT16 IVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  HDC16 hdc;
  code **fn_ptr_1;

  HVar1 = GetStockObject16(BLACK_BRUSH);
  if (COLORREF_1050_5df0 == 0x0) {
    fn_ptr_1 = (code **)((int)*param_1 + 0x68);
    uVar3 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,param_1,((int)param_1 + 0x6e));
    if (uVar3 == 0x0) {
      return 0x0;
    }
    uVar3 = pass1_1008_4d72(uVar3);
    uVar4 = (uVar3 >> 0x10);
    iVar2 = (astruct_936 *)uVar3;
    COLORREF_1050_5df0 = CONCAT22(CONCAT11(0x2,iVar2->field_0x94),CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    uVar4 = 0x7ed5;
    IVar2 = GetDlgCtrlID16(param_2);
    if (IVar2 == 0x14c) {
      uVar5 = 0xffff;
      hdc = 0x0;
      goto LAB_1040_7f00;
    }
    if (IVar2 == 0x175) {
      uVar5 = 0xff;
      hdc = 0x0;
      goto LAB_1040_7f00;
    }
  }
  uVar4 = COLORREF_1050_5df0;
  uVar5 = (COLORREF_1050_5df0 >> 0x10);
  hdc = param_4;
LAB_1040_7f00:
  SetTextColor16(CONCAT22(uVar5,uVar4),hdc);
  SetBkColor16(0x1000000,param_4);
  return CONCAT22(0x1050,HVar1);
}



BOOL16 post_win_msg_1040_7f1c(param_1: u32,param_2: i16)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x78) != 0x0) {
    return 0x0;
  }
  if ((iVar1 + 0x60) != param_2) {
    (iVar1 + 0x60) = param_2;
    PostMessage16(0x0,0x0,0x85,*(HWND16 *)(iVar1 + 0x6));
  }
  return 0x1;
}



void post_win_msg_1040_7f56(param_1: u32,char *param_2)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x10)),param_2);
  PostMessage16(0x0,0x0,0x85,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



void menu_ui_op_1040_7f86(astruct_855 *param_1)

{
  HMENU16 HVar1;
  astruct_855 *iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_855 *)param_1;
  if ((iVar2->field104_0x6a != NULL) && (iVar2->field103_0x68 == 0x0)) {
    HVar1 = LoadMenu16(iVar2->field104_0x6a,HINSTANCE16_1050_038c);
    iVar2->field103_0x68 = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    HVar1 = GetSubMenu16(0x0,iVar2->field103_0x68);
    iVar2->field103_0x68 = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  ClientToScreen16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),iVar2->field6_0x6);
  HVar1 = iVar2->field103_0x68;
  TrackPopupMenu16(NULL,iVar2->field6_0x6,0x0,HVar1,0x0,0x0,HVar1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_help_1040_800c(u32 param_1)

{
  u16 in_AX;
  u16 uVar1;
  u32 in_EDX;
  i16 iVar2;
  u16 uVar3;
  u16 unaff_CS;
  i16 iVar4;
  u16 w_command;
  HWND16 hwnd;

  uVar1 = FUN_1010_830a(in_AX,in_EDX,unaff_CS,_u16_1050_14cc,0x1f8);
  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0x8a) == 0x0) {
    hwnd = *(HWND16 *)(iVar2 + 0x6);
    iVar4 = 0x0;
    w_command = 0x3;
    iVar2 = 0x0;
  }
  else {
    hwnd = *(HWND16 *)(iVar2 + 0x6);
    w_command = 0x1;
    iVar2 = (iVar2 + 0x8a);
    iVar4 = iVar2 >> 0xf;
  }
  WinHelp16(CONCAT22(iVar4,iVar2),w_command,(char *)CONCAT22((int)in_EDX,uVar1),hwnd);
  return;
}



u16 pass1_1040_8054(void)

{
  return 0x0;
}


/*
Unable to decompile 'pass1_1040_805a'
Cause:
Low-level Error: Symbol $$undef00000006 extends beyond the end of the address space
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_807e(astruct_395 *param_1,u16 param_2)

{
  code **ppcVar1;
  u16 in_AX;
  astruct_394 *paVar2;
  astruct_394 *paVar3;
  astruct_394 *paVar4;
  u16 uVar5;
  u16 uVar6;
  u8 *puVar7;
  u32 in_EDX;
  Struct57*paVar8;
  astruct_395 *iVar9;
  astruct_395 *uVar9;
  u16 unaff_CS;
  u32 *puStack6;

  if (param_2 == 0x1) {
    pass1_1040_805a((u8 *)in_EDX);
    return;
  }
  paVar2 = (astruct_394 *)FUN_1010_830a(in_AX,in_EDX,unaff_CS,_u16_1050_14cc,param_2);
  uVar5 = in_EDX;
  puStack6 = (u32 *)CONCAT22(uVar5,paVar2);
  paVar8 = (Struct57*)(in_EDX & 0xffff0000 | (u32)(uVar5 | paVar2));
  if ((uVar5 | paVar2) != 0x0) {
    ppcVar1 = (code **)((int)*puStack6 + 0x14);
    paVar3 = paVar2;
    (**ppcVar1)(0x1010,paVar2,uVar5);
    uVar6 = SUB42(paVar8,0x0);
    uVar9 = (astruct_395 *)((u32)param_1 >> 0x10);
    iVar9 = (astruct_395 *)param_1;
    paVar4 = paVar3;
    if (iVar9->field112_0x70 != NULL) {
      paVar4 = *(astruct_394 **)&iVar9->field112_0x70;
      uVar5 = ((int)&iVar9->field112_0x70 + 0x2);
      paVar8 = (Struct57*)((u32)paVar8 & 0xffff0000 | (u32)(uVar5 | paVar4));
      if ((uVar5 | paVar4) != 0x0) {
        ppcVar1 = (code **)(u32)paVar4;
        (**ppcVar1)(0x1010,paVar4,(char)uVar5,0x1);
      }
    }
    mem_op_1000_179c(0x14,paVar8);
    puVar7 = (u8 *)(paVar8 | paVar4);
    if (puVar7 == NULL) {
      paVar4 = NULL;
      puVar7 = NULL;
    }
    else {
      struct_1008_4c58(paVar4);
    }
    *(astruct_394 **)&iVar9->field112_0x70 = paVar4;
    *(u8 **)((int)&iVar9->field112_0x70 + 0x2) = puVar7;
    pass1_1008_4d84(puVar7,iVar9->field112_0x70,CONCAT22(uVar6,paVar3));
    if (puStack6 != NULL) {
      ppcVar1 = (code **)*puStack6;
      (**ppcVar1)(0x1008,paVar2,(char)in_EDX,0x1);
    }
    return;
  }
  return;
}



void unk_win_ui_op_1040_8158(u32 *param_1,POINT16 param_2,param_3: i16)

{
  code **ppcVar1;
  BOOL16 BVar2;
  i16 iVar3;
  u16 uVar4;

  if (param_3 == 0x2) {
    uVar4 = ((u32)param_1 >> 0x10);
    iVar3 = (int)param_1;
    if ((iVar3 + 0x76) != 0x0) {
      ScreenToClient16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),*(HWND16 *)(iVar3 + 0x6));
      GetSystemMetrics16(SM_CYCAPTION);
      BVar2 = PtInRect16((POINT16)((u32)param_1 & 0xffff0000 | ZEXT24((RECT16 *)(iVar3 + 0x82))),
                         (RECT16 *)(iVar3 + 0x82));
      if (BVar2 != 0x0) {
        ppcVar1 = (code **)((int)*param_1 + 0x14);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538,iVar3);
      }
    }
  }
  return;
}



void check_dialog_msg_1040_81b6(u32 param_1)

{
  BOOL16 BVar1;
  u16 uVar2;
  MSG16 local_14;

  uVar2 = (param_1 >> 0x10);
  ((int)param_1 + 0x78) = 0x1;
  while( true ) {
    BVar1 = IsWindow16(*(HWND16 *)((int)param_1 + 0x6));
    if (BVar1 == 0x0) {
      return;
    }
    local_14.hwnd = (HWND16)0x1050;
    BVar1 = GetMessage16(0x0,0x0,0x0,&local_14);
    if (BVar1 == 0x0) break;
    IsDialogMessage16(&local_14,(HWND16)0x1050);
  }
  return;
}



void win_ui_op_1040_81fe(u32 param_1)

{
  SetSysModalWindow(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



void destroy_win_1040_8212(astruct_899 *param_1)

{
  BOOL16 is_window;
  astruct_899 *struct_1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_899 *)param_1;
  if (struct_1->field140_0x8c != 0x0) {
    is_window = IsWindow16(struct_1->field140_0x8c);
    if (is_window != 0x0) {
      DestroyWindow16(struct_1->field140_0x8c);
      struct_1->field140_0x8c = 0x0;
    }
  }
  return;
}



u16 pass1_1040_824a(param_1: u32,param_2: i16)

{
  if (((int)param_1 + 0x6) != param_2) {
    return 0x1;
  }
  return 0x0;
}



u16 FUN_1040_8260(void)

{
  return 0x0;
}



u16 FUN_1040_8266(void)

{
  return 0x0;
}



void move_win_1040_826c(StructB *param_1,INT16 param_2,BOOL16 param_3)

{
  INT16 IVar1;
  u16 struct_b_1_hi;
  i16 local_e;
  i16 iStack12;
  i16 iStack10;
  i16 iStack8;
  INT16 IStack6;
  BOOL16 BStack4;

  struct_b_1_hi = ((u32)param_1 >> 0x10);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_e),*(HWND16 *)((int)param_1 + 0x6));
  if ((param_3 == 0xffff) || (param_2 == -0x1)) {
    IVar1 = GetSystemMetrics16(SM_CXSCREEN);
    BStack4 = (IVar1 - (iStack10 - local_e)) / 0x2;
    IVar1 = GetSystemMetrics16(SM_CYSCREEN);
    param_2 = (IVar1 - (iStack8 - iStack12)) / 0x2;
  }
  else {
    BStack4 = param_3;
  }
  IVar1 = *(INT16 *)((int)param_1 + 0x6);
  IStack6 = param_2;
  MoveWindow16(0x0,IVar1,iStack10 - local_e,param_2,BStack4,IVar1);
  return;
}



void draw_op_1040_82ee(astruct_14 *astruct14_param_1)

{
  astruct_14 *struct_1;
  HDC16 struct_1_hi;
  u32 local_1a;
  u32 uStack22;
  RECT16 rect_var_12;
  i16 iStack14;
  i16 iStack12;
  HBRUSH16 brush_handle_1;
  i16 iStack8;
  i16 iStack6;
  i16 iStack4;

  struct_1_hi = (HDC16)((u32)astruct14_param_1 >> 0x10);
  struct_1 = (astruct_14 *)astruct14_param_1;
  iStack6 = (struct_1->field118_0x80 - struct_1->field116_0x7c) + -0x2;
  iStack8 = (-(struct_1->field95_0x60 == 0x0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  brush_handle_1 = CreateSolidBrush16(CONCAT22(0x100,iStack8));
  if (&struct_1[0x1].field_0x4 == 0x0) {
    local_1a = CONCAT22(struct_1->field98_0x66 + 0x2,struct_1->field97_0x64 + 0x2);
    uStack22 = CONCAT22(iStack4,iStack6);
    (u32)(struct_1 + 0x1) = local_1a;
    (u32)&struct_1[0x1].field_0x4 = uStack22;
  }
  rect_var_12.x = (struct_1 + 0x1) + 0x2;
  rect_var_12.y =
       (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 + &struct_1[0x1].field_0x2 +
       -0x2;
  iStack14 = &struct_1[0x1].field_0x4 + -0x2;
  iStack12 = (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 +
             &struct_1[0x1].field_0x2 + 0x2;
  FrameRect16(brush_handle_1,(RECT16 *)(struct_1 + 0x1),struct_1_hi);
  FrameRect16(brush_handle_1,&rect_var_12,(HDC16)0x1050);
  DeleteObject16(brush_handle_1);
  struct_1->field115_0x7a = &struct_1[0x1].field_0x4 + 0x2;
  return;
}



StructD * pass1_1040_83e6(StructD *param_1,u8 param_2)

{
  ui_cleanup_op_1040_782c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



Struct57*
pass1_1040_8478(u16 param_1,Struct57*param_2,u16 param_3,char *param_4,char *param_5,u16 param_6)

{
  u16 uVar1;
  Struct57*iVar2;
  Struct57*uVar2;

  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_6);
  uVar2 = (Struct57*)((u32)param_2 >> 0x10);
  iVar2 = (Struct57*)param_2;
  (iVar2 + 0x1)->field0_0x0 = 0x0;
  iVar2[0x1].field5_0xa = param_3;
  iVar2[0x1].field6_0xc = 0x0;
  &iVar2[0x1].field_0x24 = 0x0;
  param_2->field0_0x0 = 0x8ddc;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  (u32)&iVar2[0x1].field8_0x10 = 0x0;
  iVar2[0x1].field10_0x14 = 0x12c;
  uVar1 = str_op_1008_60e8(param_1,param_5);
  iVar2[0x1].field1_0x2 = uVar1;
  iVar2[0x1].field2_0x4 = param_1;
  uVar1 = str_op_1008_60e8(param_1,param_4);
  iVar2[0x1].field3_0x6 = uVar1;
  iVar2[0x1].field4_0x8 = param_1;
  load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = NULL;
  return param_2;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 string_1040_8520(param_1: u32,Struct57*param_2,u16 param_3,param_4: u32,u32 param_5)

{
  u32 uVar1;
  u32 *puVar2;
  u16 *puVar3;
  u16 uVar4;
  u16 uVar5;
  char *pcVar6;
  u16 uVar7;
  i16 iStack22;
  i16 iStack16;
  u16 *puStack14;
  Struct57*iVar7;

  iVar7 = (Struct57*)param_2;
  uVar7 = ((u32)param_2 >> 0x10);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_3);
  (iVar7 + 0x1)->field0_0x0 = 0x0;
  iVar7[0x1].field5_0xa = param_4;
  iVar7[0x1].field6_0xc = 0x0;
  &iVar7[0x1].field_0x24 = 0x0;
  param_2->field0_0x0 = 0x8ddc;
  iVar7->field1_0x2 = &PTR_LOOP_1050_1040;
  (u32)&iVar7[0x1].field8_0x10 = 0x0;
  iVar7[0x1].field10_0x14 = 0x12c;
  puVar2 = &param_5;
  iStack16 = param_4;
  if (param_4 != 0x0) {
    puVar2 = (u32 *)((int)&param_5 + 0x2);
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),param_5);
    iVar7[0x1].field3_0x6 = param_5;
    iVar7[0x1].field4_0x8 = param_1;
    iStack16 = param_4 + -0x1;
  }
  puStack14 = (u16 *)CONCAT22(0x1050,puVar2);
  iStack22 = 0x0;
  while (puVar3 = puStack14, iStack16 != 0x0) {
    puStack14 = (u16 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    param_1 = param_1 & 0xffff0000 | (u32)pcVar6 >> 0x10;
    uVar4 = str_op_1000_3da4(pcVar6);
    iStack22 += uVar4;
    iStack16 = iStack16 + -0x1;
  }
  uVar5 = iStack22 + 0x1;
  mem_op_1000_179c(uVar5,(Struct57*)param_1);
  iVar7[0x1].field1_0x2 = uVar5;
  iVar7[0x1].field2_0x4 = param_1;
  puStack14 = (u16 *)CONCAT22(0x1050,(int)&param_5 + 0x2);
  iStack16 = param_4 + -0x1;
  if (param_4 + -0x1 != 0x0) {
    puStack14 = (u16 *)CONCAT22(0x1050,&stack0x0012);
    uVar1 = (u32)&iVar7[0x1].field1_0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(char *)uVar1,(short)((u32)uVar1 >> 0x10)
              );
    iStack16 = param_4 + -0x2;
  }
  while (puVar3 = puStack14, iStack16 != 0x0) {
    puStack14 = (u16 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    pass1_1000_3cea((u32)&iVar7[0x1].field1_0x2,pcVar6);
    iStack16 = iStack16 + -0x1;
  }
  load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = NULL;
  return (int)iVar7;
}



void pass1_1040_869a(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x8ddc;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x90);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x94);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



void enable_win_1040_86dc(u32 param_1)

{
  HWND16 HVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  HVar1 = GetDlgItem16(0x1,*(HWND16 *)((int)param_1 + 0x6));
  if (HVar1 != 0x0) {
    EnableWindow16(0x1,HVar1);
    HVar1 = GetDlgItem16(0x2,*(HWND16 *)((int)param_1 + 0x6));
    if (HVar1 != 0x0) {
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 * win_ui_op_1040_8718(astruct_37 *param_1)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  u32 in_EDX;
  Struct57*paVar6;
  u32 *puVar7;
  u16 in_stack_0000fd88;
  u16 in_stack_0000fd8a;
  u16 in_stack_0000feac;
  u16 in_stack_0000feae;
  u16 in_stack_0000feb2;
  u16 in_stack_0000feb4;
  u16 in_stack_0000feb6;
  u16 in_stack_0000feb8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u16 in_stack_0000fee0;
  u16 in_stack_0000fee2;
  i16 local_104 [0x80];
  u16 uStack4;
  u16 uVar8;
  astruct_37 *paVar12;
  astruct_37 *uVar12;

  uVar5 = ((u32)in_EDX >> 0x10);
  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5df4);
  paVar12 = (astruct_37 *)param_1;
  uVar12 = (astruct_37 *)((u32)param_1 >> 0x10);
  dialog_ui_fn_1040_78e2((StructB *)param_1);
  PTR_LOOP_1050_5df6 = (u8 *)((int)&paVar12->field1_0x4 + 0x2);
  if (*(i32 *)&paVar12->field_0x94 != 0x0) {
    unk_str_op_1000_3d3e
              ((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&paVar12->field_0x10)),*(char **)&paVar12->field_0x94);
  }
  get_sys_metrics_1040_8c66(param_1);
  uStack4 = paVar12->field138_0x98 & 0xf;
  if (uStack4 == 0x1) {
    iVar3 = &paVar12[0x1].field_0x8 + -0xc4;
    paVar6 = (Struct57*)CONCAT22(uVar5,iVar3 >> 0xf);
    &paVar12[0x1].field_0xc = iVar3 / 0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0xff,(char *)local_104,
               (short)0x1050);
    uVar2 = (u32)&paVar12[0x1].field_0xc;
    create_window_1040_8bea
              (paVar12,uVar12,0x1,0x1,(int)uVar2,(int)((u32)uVar2 >> 0x10),CONCAT22(0x1050,local_104));
    piVar1 = (i16 *)&paVar12[0x1].field_0xc;
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0xff,(char *)local_104,
               (short)0x1050);
    uVar2 = (u32)&paVar12[0x1].field_0xc;
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = 0x2;
  }
  else {
    if (uStack4 != 0x4) {
      iVar3 = &paVar12[0x1].field_0x8 + -0x58;
      paVar6 = (Struct57*)CONCAT22(uVar5,iVar3 >> 0xf);
      &paVar12[0x1].field_0xc = iVar3 / 0x2;
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0xff,(char *)local_104,
                 (short)0x1050);
      uVar2 = (u32)&paVar12[0x1].field_0xc;
      uVar10 = uVar2;
      uVar11 = ((u32)uVar2 >> 0x10);
      uVar5 = 0x1;
      uVar9 = 0x1;
      goto LAB_1040_88a5;
    }
    iVar3 = &paVar12[0x1].field_0x8 + -0xc4;
    paVar6 = (Struct57*)CONCAT22(uVar5,iVar3 >> 0xf);
    &paVar12[0x1].field_0xc = iVar3 / 0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0xff,(char *)local_104,
               (short)0x1050);
    uVar2 = (u32)&paVar12[0x1].field_0xc;
    create_window_1040_8bea
              (paVar12,uVar12,0x1,0x6,(int)uVar2,(int)((u32)uVar2 >> 0x10),CONCAT22(0x1050,local_104));
    piVar1 = (i16 *)&paVar12[0x1].field_0xc;
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0xff,(char *)local_104,
               (short)0x1050);
    uVar2 = (u32)&paVar12[0x1].field_0xc;
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = 0x7;
  }
  uVar5 = 0x0;
LAB_1040_88a5:
  create_window_1040_8bea(paVar12,uVar12,uVar5,uVar9,uVar10,uVar11,CONCAT22(0x1050,local_104));
  puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fee0,0x48),in_stack_0000fd88,
                           in_stack_0000feac,in_stack_0000feb2,in_stack_0000feb6);
  uVar5 = ((u32)puVar7 >> 0x10);
  local_104[0] = ((int)puVar7 + 0xa);
  uStack4 = ((int)puVar7 + 0xc);
  iVar3 = uStack4 - &paVar12[0x1].field_0xa;
  paVar6 = (Struct57*)((u32)paVar6 & 0xffff0000 | (u32)(iVar3 >> 0xf));
  SetWindowPos16(0x40,*(INT16 *)&paVar12[0x1].field_0xa,*(INT16 *)&paVar12[0x1].field_0x8,iVar3 / 0x2,
                 (local_104[0] - &paVar12[0x1].field_0x8) / 0x2,0x0,*(HWND16 *)((int)&paVar12->field1_0x4 + 0x2)
                );
  PTR_LOOP_1050_5df4 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5df4);
  destroy_win_1040_8b7e((u32)param_1);
  PTR_LOOP_1050_5df6 = NULL;
  if (&paVar12[0x1].field_0x10 != 0x0) {
    puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fee2,0x37),in_stack_0000fd8a,
                             in_stack_0000feae,in_stack_0000feb4,in_stack_0000feb8);
    uVar4 = pass1_1008_ab54((u32)puVar7);
    if (uVar4 != 0x0) {
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  return PTR_LOOP_1050_5df8;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_8978(u16 param_1,u16 param_2,u32 *param_3,u16 param_4)

{
  code **ppcVar1;

  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5df4);
  win_1008_5c5c(param_1,param_2,_u16_1050_02a0,param_4);
  ppcVar1 = (code **)((int)*param_3 + 0x74);
  (**ppcVar1)(0x1008,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_89a4(u8 *param_1,u32 *param_2,u16 *param_3)

{
  u16 uVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  Struct57*paVar6;
  u16 uVar7;
  u32 *puVar8;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000fff0;

  paVar6 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5df4);
  uVar1 = ((int)param_3 + 0x2);
  uVar2 = *param_3;
  uVar7 = 0x1010;
  puVar8 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff0,0x2),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  uVar4 = ((u32)puVar8 >> 0x10);
  uVar5 = puVar8;
  if ((uVar5 + 0x72) != 0x0) {
    uVar7 = 0x1008;
    win_1008_5c7c(uVar5,uVar4,_u16_1050_02a0,CONCAT22(uVar1,uVar2));
    ((int)param_2 + 0x8c) = uVar5;
  }
  ppcVar3 = (code **)((int)*param_2 + 0x74);
  (**ppcVar3)(uVar7,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_draw_op_1040_8a06(u16 param_1,astruct_765 *param_2)

{
  astruct_13 *paVar1;
  u8 uVar6;
  HPALETTE16 HVar7;
  HANDLE16 handle;
  u32 extraout_var;
  u16 DX_REG;
  astruct_765 *iVar10;
  i16 count;
  u32 uVar8;
  COLORREF color;
  u32 color_00;
  HDC16 hdc_local_24;
  PAINTSTRUCT16 paintstruct_22;
  u8 uVar1;
  u8 uVar2;
  u8 uVar3;
  LPCSTR uVar4;
  u16 uVar5;
  astruct_766 *iVar2;

  count = (i16)((u32)param_2 >> 0x10);
  iVar10 = (astruct_765 *)param_2;
  hdc_local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->hwnd_field6_0x6);
  paVar1 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  HVar7 = palette_op_1008_4e08((HPALETTE16)&hdc_local_24,param_1,paVar1,(HDC16 *)CONCAT22(0x1050,&hdc_local_24));
  uVar8 = pass1_1008_4d72((u32)paVar1);
  uVar5 = (uVar8 >> 0x10);
  iVar2 = (astruct_766 *)uVar8;
  uVar1 = iVar2->field149_0x95;
  uVar2 = iVar2->field150_0x96;
  uVar3 = iVar2->field148_0x94;
  DrawIcon16(iVar10->field141_0x8e,0xa,0x14,hdc_local_24);
  color = SetBkColor16(0x0,hdc_local_24);
  DX_REG = (color >> 0x10);
  uVar6 = SetTextColor16(CONCAT22(CONCAT11(0x2,uVar3),CONCAT11(uVar1,uVar2)),hdc_local_24);
  color_00 = CONCAT31(extraout_var,uVar6) & 0xffff | (u32)DX_REG << 0x10;
  handle = GetProp16(s_hfont_1050_5dfa,iVar10->hwnd_field6_0x6);
  if (handle != 0x0) {
    SelectObject16(handle,hdc_local_24);
  }
  DrawText16(0x10,(RECT16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar10->rect_0x9e)),-0x1,
             (LPCSTR)iVar10->field142_0x90,hdc_local_24);
  if (handle != 0x0) {
    SelectObject16(hdc_local_24,hdc_local_24);
  }
  SetBkColor16(color,hdc_local_24);
  SetTextColor16(color_00,hdc_local_24);
  HVar7 = SelectPalette16(0x0,HVar7,hdc_local_24);
  DeleteObject16(HVar7);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->hwnd_field6_0x6);
  return;
}



void pass1_1040_8b3c(u16 param_1,param_2: u32,u32 param_3)

{
  if ((param_3 != NULL) &&
     ((param_3 == (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1) || param_3 == (u8 *)&u16_1050_0002 ||
      (((u8 *)((int)&u16_1050_0002 + 0x1U) < param_3 + -0x2 &&
       (param_3 + -0x6 < (u8 *)&u16_1050_0002)))))) {
    PTR_LOOP_1050_5df4 = NULL;
    PTR_LOOP_1050_5df8 = param_3;
    return;
  }
  post_win_msg_1040_7b3c
            ((StructC *)CONCAT22((int)param_2,param_1),(param_2 >> 0x10),param_3,(int)param_3);
  return;
}



void destroy_win_1040_8b7e(u32 param_1)

{
  DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



void load_icon_1040_8b92(Struct57*param_1)

{
  u8 bVar1;
  HICON16 HVar2;
  u16 uVar3;
  u16 uVar4;

  uVar3 = ((u32)param_1 >> 0x10);
  bVar1 = *(u8 *)((int)param_1 + 0x98) & 0xf0;
  if (bVar1 == 0x30) {
    uVar4 = 0x7f03;
  }
  else if ((bVar1 == 0x10) || (bVar1 == 0x10)) {
    uVar4 = 0x7f01;
  }
  else if ((bVar1 == 0x40) || (bVar1 == 0x40)) {
    uVar4 = 0x7f04;
  }
  else {
    if (bVar1 != 0x20) {
      return;
    }
    uVar4 = 0x7f02;
  }
  HVar2 = LoadIcon16((char *)(u32)uVar4,0x0);
  *(HICON16 *)((int)param_1 + 0x8e) = HVar2;
  return;
}



HANDLE16 create_window_1040_8bea
                   (astruct_37 *param_1,u16 param_2,i16 param_3,u16 param_4,INT16 param_5,INT16 param_6,char *param_7)

{
  HWND16 hwnd;
  HANDLE16 wparam;
  LRESULT LVar1;
  u32 uStack6;

  uStack6 = 0x50010000;
  if (param_3 != 0x0) {
    uStack6 = 0x50010001;
  }
  if (&param_1->field_0x74 != 0x0) {
    uStack6 |= 0x8000000;
  }
  hwnd = CreateWindow16(0x0,(void *)CONCAT22(param_4,HINSTANCE16_1050_038c),
                        *(HINSTANCE16 *)((int)&param_1->field1_0x4 + 0x2),0x17,0x58,param_6,param_5,(INT16)uStack6,
                        (INT16)(uStack6 >> 0x10),param_7,s_OPButton_1050_5e00);
  wparam = GetProp16(s_hfont_1050_5e09,*(HWND16 *)((int)&param_1->field1_0x4 + 0x2));
  if (wparam != 0x0) {
    LVar1 = SendMessage16(0x1,wparam,0x30,hwnd);
    wparam = (HANDLE16)LVar1;
  }
  return wparam;
}



void get_sys_metrics_1040_8c66(astruct_37 *param_1)

{
  i16 *piVar1;
  u8 bVar2;
  HDC16 HVar3;
  INT16 IVar4;
  astruct_37 *struct_1;
  u16 uVar5;

  uVar5 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_37 *)param_1;
  HVar3 = GetDC16(*(HWND16 *)((int)&struct_1->field1_0x4 + 0x2));
  draw_text_1040_8d14(param_1,HVar3);
  struct_1[0x1].field1_0x4 = *(char **)&struct_1->field144_0x9e;
  *(RECT16 **)&struct_1[0x1].field_0x8 = (struct_1 + 0x1)->field0_0x0;
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  piVar1 = (i16 *)&struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + IVar4;
  bVar2 = struct_1->field138_0x98 & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar4 = GetSystemMetrics16(SM_CYICON);
    if (&struct_1[0x1].field_0xa < IVar4) {
      IVar4 = GetSystemMetrics16(SM_CYICON);
      *(INT16 *)&struct_1[0x1].field_0xa = IVar4;
    }
  }
  piVar1 = (i16 *)&struct_1[0x1].field_0x8;
  *piVar1 = *piVar1 + 0x14;
  piVar1 = (i16 *)&struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0xa;
  &struct_1[0x1].field_0xe = &struct_1[0x1].field_0xa;
  piVar1 = (i16 *)&struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0x30;
  HVar3 = *(HDC16 *)((int)&struct_1->field1_0x4 + 0x2);
  ReleaseDC16(HVar3,HVar3);
  return;
}



void draw_text_1040_8d14(astruct_37 *param_1,HDC16 hdc_param_2)

{
  LPCSTR in_string;
  u8 bVar1;
  INT16 IVar2;
  HANDLE16 handle;
  astruct_37 *struct_1;
  u16 uVar3;
  HGDIOBJ16 obj_handle_1;

  uVar3 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_37 *)param_1;
  bVar1 = struct_1->field138_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    struct_1->field145_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(SM_CXICON);
    struct_1->field144_0x9e = IVar2 + 0x28;
  }
  else {
    struct_1->field145_0xa0 = 0xa;
    struct_1->field144_0x9e = 0x14;
  }
  handle = GetProp16(s_hfont_1050_5e0f,*(HWND16 *)((int)&struct_1->field1_0x4 + 0x2));
  if (handle != 0x0) {
    SelectObject16(handle,hdc_param_2);
  }
  in_string = struct_1->field133_0x90;
  obj_handle_1 = (HGDIOBJ16)((u32)in_string >> 0x10);
  DrawText16(0x410,(RECT16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&struct_1->field144_0x9e)),-0x1,in_string,hdc_param_2
            );
  if (obj_handle_1 != 0x0) {
    SelectObject16(obj_handle_1,hdc_param_2);
  }
  return;
}



StructD * pass1_1040_8db6(StructD *param_1,u8 param_2)

{
  pass1_1040_869a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1040_8e58(i16 param_1,u16 param_2,u16 param_3,u32 param_4)

{
  pass1_1040_b040((Struct57*)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x8f3c;
  (param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return (u16 *)CONCAT22(param_2,param_1);
}



void pass1_1040_8e82(StructD *param_1)

{
  u16 in_stack_0000ffde;

  param_1->address_offset_field_0x0 = 0x8f3c;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



void enable_window_1040_8ea0(u8 *param_1,astruct_903 *param_2,u16 param_3,u32 param_4)

{
  BOOL16 enable;
  HWND16 hwnd;

  if (param_4 == 0xf8) {
    hwnd = GetDlgItem16(0x17d8,*(HWND16 *)((int)param_2 + 0x6));
    enable = 0x1;
  }
  else {
    if (param_4 != 0x17d8) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4);
      return;
    }
    SetWindowPos16(0x6,0xf6,0x269,0x0,0x0,0x0,*(HWND16 *)((int)param_2 + 0x6));
    enable = (BOOL16)s_tile2_bmp_1050_1538;
    GetDlgItem16(0x17d8,*(HWND16 *)((int)param_2 + 0x6));
    hwnd = 0x0;
  }
  EnableWindow16(enable,hwnd);
  return;
}



StructD * pass1_1040_8f16(StructD *param_1,u8 param_2)

{
  pass1_1040_8e82(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_struct_op_1040_8fb8
               (param_1: u32,astruct_65 *param_2,u16 param_3,char *param_4,u16 param_5,u16 param_6,u16 param_7,
               u16 param_8,u16 param_9,u16 param_10)

{
  u16 uVar5;
  u16 uVar3;
  u16 uVar6;
  astruct_65 *iVar3;
  astruct_65 *uVar4;
  u16 unaff_CS;
  u16 uVar1;
  u16 uVar2;

  uVar4 = (astruct_65 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_65 *)param_2;
  param_2->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  (u32)&iVar3->field2_0x4 = 0x0;
  (u32)&iVar3->field4_0x8 = 0x0;
  (u32)&iVar3->field6_0xc = 0x0;
  (u32)&iVar3->field8_0x10 = 0x0;
  iVar3->field10_0x14 = 0x0;
  iVar3->field11_0x18 = 0x0;
  iVar3->field12_0x1a = param_9;
  iVar3->field13_0x1c = param_8;
  iVar3[0x1].field8_0x10 = 0x5;
  iVar3[0x1].field9_0x12 = 0x0;
  &iVar3[0x1].field10_0x14 = 0x0;
  ((int)&iVar3[0x1].field10_0x14 + 0x2) = 0x2;
  iVar3[0x1].field11_0x18 = 0x0;
  iVar3[0x1].field12_0x1a = param_3;
  param_2->field0_0x0 = 0x9800;
    // this is probably just the value 0x1040
  iVar3->field1_0x2 = &PTR_LOOP_1050_1040;
  uVar1 = iVar3[0x1].field8_0x10;
  iVar3[0x1].field1_0x2 = uVar1;
  (iVar3 + 0x1)->field0_0x0 = uVar1;
  iVar3[0x1].field3_0x6 = 0x0;
  iVar3[0x1].field2_0x4 = 0x0;
  if ((param_7 != 0x0) && (param_6 != 0x0)) {
    iVar3[0x1].field9_0x12 = 0x1;
    uVar5 = FUN_1010_830a(0x0,param_1,unaff_CS,_u16_1050_14cc,param_7);
    iVar3->field4_0x8 = uVar5;
    iVar3->field5_0xa = param_1;
    uVar5 = FUN_1010_830a(uVar5,param_1,0x1010,_u16_1050_14cc,param_6);
    iVar3->field6_0xc = uVar5;
    iVar3->field7_0xe = param_1;
    if (param_5 == 0x0) {
      (u32)&iVar3->field8_0x10 = 0x0;
    }
    else {
      uVar5 = FUN_1010_830a(uVar5,param_1,0x1010,_u16_1050_14cc,param_5);
      iVar3->field8_0x10 = uVar5;
      iVar3->field9_0x12 = param_1;
    }
  }
  uVar6 = param_1;
  uVar2 = iVar3[0x1].field8_0x10;
  iVar3[0x1].field5_0xa = uVar2;
  iVar3[0x1].field4_0x8 = uVar2;
  (u32)&iVar3[0x1].field6_0xc = 0x0;
  if (param_4 != NULL) {
    uVar3 = str_op_1008_60e8(uVar6,param_4);
    iVar3->field2_0x4 = uVar3;
    iVar3->field3_0x6 = uVar6;
  }
  (u32)&iVar3->field16_0x22 = 0x0;
  iVar3->field14_0x1e = 0x0;
  iVar3->field15_0x20 = 0x0;
  if (_u16_1050_5e18 == NULL) {
    _u16_1050_5e18 = MakeProcInstance16(HINSTANCE16_1050_038c,call_win_proc_1040_9684);
  }
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mix_win_ui_op_1040_911e(StructD *param_1)

{
  u32 *puVar3;
  StructD *struct_1;
  u16 uVar5;
  u32 *puVar1;
  u32 *puVar2;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  code **fn_ptr_1;

  uVar5 = ((u32)param_1 >> 0x10);
  struct_1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x9800;
  struct_1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&struct_1->field_0x38 != 0x0) {
    puVar1 = (u32 *)struct_1->field5_0x8;
    uVar2 = struct_1->field6_0xa;
    if ((uVar2 | puVar1) != 0x0) {
      fn_ptr_1 = (code **)*puVar1;
      (**fn_ptr_1)();
    }
    puVar3 = (u32 *)struct_1->field7_0xc;
    uVar3 = struct_1->field8_0xe;
    if ((uVar3 | puVar3) != 0x0) {
      fn_ptr_1 = (code **)*puVar3;
      (**fn_ptr_1)();
    }
    puVar3 = &struct_1->field_0x10;
    uVar4 = struct_1->field11_0x12;
    if ((uVar4 | puVar3) != 0x0) {
      fn_ptr_1 = (code **)*puVar3;
      (**fn_ptr_1)();
    }
  }
  fn_ptr_1000_17ce(*(char **)&struct_1->hfile_0x4);
  SetWindowLong16(struct_1->field12_0x14,-0x4,struct_1->field13_0x18);
  RemoveProp16(s_thisLo_1050_5e1c,struct_1->field13_0x18);
  RemoveProp16(s_thisHi_1050_5e23,struct_1->field13_0x18);
  RemoveProp16(s_procLo_1050_5e2a,struct_1->field13_0x18);
  RemoveProp16(s_procHi_1050_5e31,struct_1->field13_0x18);
  RemoveProp16(s_IsDlg_1050_5e38,struct_1->field13_0x18);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -0x1;
  if (PTR_LOOP_1050_5e16 == NULL) {
    FreeProcInstance16(_u16_1050_5e18);
    _u16_1050_5e18 = NULL;
  }
  param_1->address_offset_field_0x0 = 0x389a;
  struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



void enable_win_1040_9234(param_1: u32,BOOL16 param_2)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x18) != 0x0) {
    EnableWindow16(param_2,*(HWND16 *)((int)param_1 + 0x18));
  }
  return;
}



void pass1_1040_9252(astruct_65 *param_1)

{
  i16 *piVar1;
  u16 uVar2;
  i16 iVar4;
  astruct_65 *iVar3;
  astruct_65 *uVar5;

  uVar5 = (astruct_65 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_65 *)param_1;
  if (*(i32 *)&iVar3->field2_0x4 != 0x0) {
    draw_text_1040_9650((astruct_65 *)((u32)param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  if (*(i32 *)&iVar3->field4_0x8 != 0x0) {
    pass1_1040_9618((astruct_65 *)((u32)param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  uVar2 = iVar3[0x1].field6_0xc;
  if (iVar3->field16_0x22 < (int)uVar2) {
    iVar3->field16_0x22 = uVar2;
  }
  uVar2 = iVar3[0x1].field7_0xe;
  if (iVar3->field17_0x24 < (int)uVar2) {
    iVar3->field17_0x24 = uVar2;
  }
  iVar4 = (iVar3 + 0x1)->field0_0x0 + iVar3[0x1].field2_0x4;
  if (iVar3->field16_0x22 < iVar4) {
    iVar3->field16_0x22 = iVar4;
  }
  iVar4 = iVar3[0x1].field1_0x2 + iVar3[0x1].field3_0x6;
  if (iVar3->field17_0x24 < iVar4) {
    iVar3->field17_0x24 = iVar4;
  }
  piVar1 = &iVar3->field16_0x22;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  piVar1 = &iVar3->field17_0x24;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void create_window_1040_92dc(astruct_65 *param_1)

{
  HWND16 hwnd;
  astruct_65 *pstruct65_2;
  astruct_65 *pstruct65_1;
  let mut lVar1: i32;

  pstruct65_1 = (astruct_65 *)((u32)param_1 >> 0x10);
  pstruct65_2 = (astruct_65 *)param_1;
  if (pstruct65_2->field11_0x18 == 0x0) {
    hwnd = CreateWindow16(0x0,(void *)CONCAT22(pstruct65_2->field13_0x1c,HINSTANCE16_1050_038c),
                          pstruct65_2->field12_0x1a,0x0,0x0,pstruct65_2->field15_0x20,pstruct65_2->field14_0x1e,0xb,
                          0x4000,s__1050_5e3e,s_button_1050_5e3f);
    pstruct65_2->field11_0x18 = hwnd;
    lVar1 = SetWindowLong16(_u16_1050_5e18,-0x4,hwnd);
    lVar1 = (HANDLE16)((u32)lVar1 >> 0x10);
    &pstruct65_2->field10_0x14 = (int)lVar1;
    *(HANDLE16 *)((int)&pstruct65_2->field10_0x14 + 0x2) = lVar1;
    SetProp16(lVar1,s_procHi_1050_5e46,pstruct65_2->field11_0x18);
    SetProp16(*(HANDLE16 *)&pstruct65_2->field10_0x14,s_procLo_1050_5e4d,pstruct65_2->field11_0x18);
    SetProp16((HANDLE16)pstruct65_1,s_thisHi_1050_5e54,pstruct65_2->field11_0x18);
    SetProp16((HANDLE16)pstruct65_2,s_thisLo_1050_5e5b,pstruct65_2->field11_0x18);
    if (pstruct65_2[0x1].field12_0x1a != 0x0) {
      SetProp16(0x1,s_IsDlg_1050_5e62,pstruct65_2->field11_0x18);
    }
    ShowWindow16(0x5,pstruct65_2->field11_0x18);
  }
  return;
}



void mov_update_win_1040_93aa(astruct_65 *param_1,INT16 param_2,u16 param_3)

{
  astruct_65 *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_65 *)param_1;
  iVar1->field14_0x1e = param_3;
  iVar1->field15_0x20 = param_2;
  MoveWindow16(0x1,iVar1->field17_0x24,iVar1->field16_0x22,param_2,iVar1->field14_0x1e,iVar1->field11_0x18);
  UpdateWindow16(iVar1->field11_0x18);
  return;
}



LRESULT pass1_1040_93e6(u32 param_1)

{
  u16 uVar1;
  LRESULT LVar2;

  uVar1 = (param_1 >> 0x10);
  LVar2 = SendMessage16(0x0,*(WPARAM16 *)((int)param_1 + 0x1c),0x111,*(HWND16 *)((int)param_1 + 0x1a));
  return LVar2;
}



LRESULT send_msg_1040_9404(u32 param_1)

{
  u16 uVar1;
  LRESULT LVar2;

  uVar1 = (param_1 >> 0x10);
  LVar2 = SendMessage16(0x0,*(WPARAM16 *)((int)param_1 + 0x1c),0x111,*(HWND16 *)((int)param_1 + 0x1a));
  return LVar2;
}



void pass1_1040_9422(u32 *param_1)

{
  code **ppcVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x8) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x10);
    (**ppcVar1)();
  }
  if (*(i32 *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
  }
  return;
}



void unk_draw_op_1040_9458(astruct_17 *param_1,u8 param_2,u16 param_3)

{
  code **ppcVar1;
  u16 *hpal;
  HPALETTE16 obj;
  u16 uVar3;
  astruct_17 *iVar4;
  u16 uVar4;
  u16 *puStack8;
  u32 *puStack6;
  u32 UVar2;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_17 *)param_1;
  if (iVar4->field8_0x8 != NULL) {
    puStack6 = iVar4->field8_0x8;
    if (((((int)&iVar4->field9_0xc + 0x2) | &iVar4->field9_0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack6 = iVar4->field9_0xc;
    }
    if ((iVar4->field10_0x10 != NULL) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = iVar4->field10_0x10;
    }
    hpal = &param_3;
    UVar2 = *puStack6;
    ppcVar1 = (code **)((int)UVar2 + 0x8);
    (**ppcVar1)();
    ppcVar1 = (code **)((int)UVar2 + 0x4);
    (**ppcVar1)();
    obj = SelectPalette16(0x0,(HPALETTE16)hpal,param_3);
    DeleteObject16(obj);
  }
  return;
}



void draw_text_1040_94fc(astruct_37 *struct_param_1,HDC16 hdc_param_2)

{
  astruct_37 *struct_1;
  u16 struct_1_lo;
  COLORREF colorref_2;
  u16 u16_var_1;
  u16 u16_var3;

  struct_1_lo = ((u32)struct_param_1 >> 0x10);
  struct_1 = (astruct_37 *)struct_param_1;
  colorref_2 = SetBkColor16(CONCAT22(0x100,struct_1->field49_0x3a),hdc_param_2);
  SetTextColor16(CONCAT22(0x100,struct_1->field50_0x3c),hdc_param_2);
  DrawText16(0x10,(RECT16 *)((u32)struct_param_1 & 0xffff0000 | ZEXT24(&struct_1->field40_0x2e)),-0x1,
             struct_1->field1_0x4,hdc_param_2);
  u16_var_1 = ((colorref_2 & 0xffff0000) >> 0x10);
  u16_var3 = hdc_param_2;
  SetBkColor16(colorref_2 & 0xffff0000 | (u32)hdc_param_2,hdc_param_2);
  SetTextColor16(CONCAT22(u16_var_1,u16_var3),hdc_param_2);
  return;
}



void win_ui_get_prop_op_1040_9566(i16 *param_1)

{
  u16 uVar1;
  i16 iVar2;
  code **ppcVar3;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  i16 iVar6;
  u16 uVar7;
  u32 *puStack12;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*param_1 == 0x4) {
    uVar1 = (iVar6 + 0xc);
    HVar4 = GetProp16(s_thisHi_1050_5e6f,*(HWND16 *)(iVar6 + 0xa));
    HVar5 = GetProp16(s_thisLo_1050_5e68,*(HWND16 *)(iVar6 + 0xa));
    if ((HVar4 | HVar5) != 0x0) {
      iVar2 = (iVar6 + 0x6);
      if (iVar2 == 0x1) {
        puStack12 = (u32 *)CONCAT22(HVar4,uVar1);
        ppcVar3 = (code **)((int)*puStack12 + 0xc);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x2) {
        puStack12 = (u32 *)CONCAT22(HVar4,uVar1);
        ppcVar3 = (code **)((int)*puStack12 + 0x10);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x4) {
        puStack12 = (u32 *)CONCAT22(HVar4,uVar1);
        ppcVar3 = (code **)((int)*puStack12 + 0x18);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,uVar1,HVar4,*(u8 *)(iVar6 + 0x8) & 0x10);
        return;
      }
    }
  }
  return;
}



void pass1_1040_9618(astruct_65 *param_1)

{
  u16 uVar1;
  astruct_65 *iVar2;
  u16 uVar2;
  u32 uVar3;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_65 *)param_1;
  uVar3 = pass1_1008_4772(*(Struct76 **)&iVar2->field4_0x8);
  uVar1 = (uVar3 >> 0x10);
  iVar2[0x1].field2_0x4 = ((int)uVar3 + 0x4);
  iVar2[0x1].field3_0x6 = ((int)uVar3 + 0x8);
  return;
}



void draw_text_1040_9650(astruct_65 *param_1)

{
  HDC16 hdc;

  hdc = GetDC16(0x0);
  DrawText16(0x410,(RECT16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x2e)),-0x1,
             *(LPCSTR *)((int)param_1 + 0x4),hdc);
  ReleaseDC16(hdc,0x0);
  return;
}



void call_win_proc_1040_9684(HWND16 win_handle_1,u16 param_2,WPARAM16 w_param_1,LPARAM l_param_1)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  BOOL16 bool_1;
  u16 uVar2;
  RECT16 local_1a [0x2];
  u32 *var18;
  u32 *var14;
  u32 *var10;
  let mut var6: i32;
  u32 var2;
  u16 var4;
  u16 var11;
  u16 var7;
  u16 var8;
  u16 var9;
  u16 uVar1;
  code **fn_ptr_1;

  handle_1 = GetProp16(s_procHi_1050_5e7d,l_param_1);
  handle_2 = GetProp16(s_procLo_1050_5e76,l_param_1);
  var6 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16(s_thisHi_1050_5e8b,l_param_1);
  handle_2 = GetProp16(s_thisLo_1050_5e84,l_param_1);
  var10 = (u32 *)CONCAT22(handle_1,handle_2);
  if ((handle_1 | handle_2) != 0x0) {
    if (l_param_1 == 0x2) {
      var18 = var10;
      var14 = var10;
      if (var10 != NULL) {
        fn_ptr_1 = (code **)*var10;
        (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,handle_2,handle_1,0x1);
      }
    }
    else if (l_param_1 == 0x201) {
      handle_1 = GetProp16(s_IsDlg_1050_5e92,l_param_1);
      if (handle_1 == 0x0) {
        uVar2 = ((int)var10 + 0x18);
        GetClientRect16(local_1a,(HWND16)0x1050);
        bool_1 = PtInRect16((POINT16)CONCAT22(param_2,win_handle_1),local_1a);
        if (bool_1 == 0x0) {
          return;
        }
        debug_print_1008_6048(uVar1,s_button__08lx_ldown_1050_5e98);
        fn_ptr_1 = (code **)((int)*var10 + 0x1c);
        (**fn_ptr_1)(0x1008,(char)var10,(int)((u32)var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
        return;
      }
    }
    else if (l_param_1 == 0x204) {
      uVar2 = (handle_2 + 0x18);
      var4 = uVar1;
      GetClientRect16(local_1a,(HWND16)0x1050);
      bool_1 = PtInRect16((POINT16)CONCAT22(param_2,win_handle_1),local_1a);
      if (bool_1 == 0x0) {
        return;
      }
      debug_print_1008_6048(var4,s_button__08lx_rdown_1050_5eab);
      fn_ptr_1 = (code **)((int)*var10 + 0x20);
      (**fn_ptr_1)(0x8,(int)var10,(char)((u32)var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
      return;
    }
  }
  if (var6 != 0x0) {
    CallWindowProc16(CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,win_handle_1)),w_param_1,l_param_1,
                     l_param_1,(LPVOID)var6);
  }
  return;
}



StructD * pass1_1040_97da(StructD *param_1,u8 param_2)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_9824(u32 *param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x0;
  (iVar1 + 0x4) = 0x0;
  (u32)(iVar1 + 0x56) = 0x0;
  (iVar1 + 0x5a) = 0x0;
  (iVar1 + 0x5c) = 0x0;
  *(u8 *)(iVar1 + 0x6) = 0x0;
  return;
}



u16 * unk_win_ui_op_1040_9854(astruct_787 *param_1)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  astruct_787 *struct_1;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_787 *)param_1;
  param_1->offset = 0x389a;
  struct_1->base = 0x1008;
  param_1->offset = 0xa230;
  struct_1->base = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&struct_1->field_0x4)),s_OPButton_1050_5ece);
  struct_1->field82_0x54 = 0x3;
  HVar1 = LoadCursor16((char *)0x7f00,0x0);
  struct_1->field84_0x58 = HVar1;
  HVar2 = GetStockObject16(BLACK_BRUSH);
  struct_1->field83_0x56 = HVar2;
  reg_class_1040_98c0((u32)param_1 & 0xffff | (u32)uVar3 << 0x10);
  return &param_1->offset;
}



void reg_class_1040_98c0(u32 param_1)

{
  BOOL16 BVar1;
  ATOM AVar2;
  WNDCLASS16 wndclass;

  wndclass.lpsz_class_name = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(&wndclass,(char *)CONCAT22((int)wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0x0) {
    wndclass.style = ((int)param_1 + 0x54);
    wndclass.lpfn_wnd_proc = 0x9cde;
    wndclass.lpfn_wnd_proc = SUB42(&PTR_LOOP_1050_1040,0x0);
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



// WARNING: Variable defined which should be unmapped: iStack88
// WARNING: Variable defined which should be unmapped: x1
// WARNING: Variable defined which should be unmapped: y2
// WARNING: Variable defined which should be unmapped: x2
// WARNING: Variable defined which should be unmapped: y1
// WARNING: Could not reconcile some variable overlaps

void draw_op_1040_9948(u16 param_1,astruct_71 *param_2)

{
  HDC16 hdc16_dev_ctx_1;
  i16 mode;
  u16 uVar3;
  HPEN16 handle;
  HPEN16 hgdiobj16_00;
  HGDIOBJ16 hgdiobj_2;
  HGDIOBJ16 hdc_lt_gray_brush_1;
  u16 cch_1;
  u16 puVar1;
  u16 uVar5;
  u16 uVar6;
  u16 uVar4;
  u16 DX_REG;
  u16 DX_REG_00;
  astruct_71 *struct71_var4;
  u16 uVar7;
  char *pcVar1;
  u16 uVar2;
  HDC16 HVar3;
  i16 iStack88;
  i16 x1;
  i16 y2;
  i16 x2;
  i16 y1;
  i16 iStack78;
  u8 paintstruct_42 [0x20];
  u16 uStack34;
  u16 uStack32;
  HGDIOBJ16 hgdiobj_1;
  i16 iStack28;
  i16 x4;
  i16 y6;
  i16 x5;
  i16 y4;
  RECT16 rect_12;
  i16 x3;
  i16 y3;
  i16 rect_a;
  i16 iStack8;
  i16 x6;
  i16 y7;
  i16 iVar1;
  astruct_782 *iVar2;
  u8 uVar8;
  u8 uVar9;
  u16 uVar14;
  u8 uVar10;
  u8 uVar11;
  u16 uVar12;
  u16 uVar13;
  u16 uVar4_00;

  x6 = 0x1;
  y7 = 0x1;
  rect_a = 0x0;
  iStack8 = 0x0;
  iStack28 = 0x0;
  hgdiobj_1 = 0x0;
  uVar7 = ((u32)param_2 >> 0x10);
  struct71_var4 = (astruct_71 *)param_2;
  uStack32 = struct71_var4->field4_0x4 & 0x8;
  uStack34 = struct71_var4->field86_0x56 & 0x1;
  hdc16_dev_ctx_1 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_42),param_1);
  mode = SetMapMode16(0x1,hdc16_dev_ctx_1);
  GetClientRect16(&rect_12,(HWND16)0x1050);
  iVar2 = (astruct_782 *)((u32)_x3 >> 0x10);
  _x3 = CONCAT22(iVar2 + -0x1,x3 + -0x1);
  if (uStack34 != 0x0) {
    x4 = (i16)rect_12;
    y6 = (i16)((u32)rect_12 >> 0x10);
    rect_12 = CONCAT22(y6 + 0x2,x4 + 0x2);
    _x3 = CONCAT22(iVar2 + -0x3,x3 + -0x3);
    x5 = x3 + -0x1;
    y4 = (i16)(iVar2 + -0x1);
  }
  if (struct71_var4->field6_0x6 != '\0') {
    iStack28 = 0x1;
    if (struct71_var4->hgdiobj_field90_0x5a != 0x0) {
      hgdiobj_1 = SelectObject16(struct71_var4->hgdiobj_field90_0x5a,hdc16_dev_ctx_1);
    }
    pcVar1 = &struct71_var4->field6_0x6;
    uVar2 = uVar7;
    HVar3 = hdc16_dev_ctx_1;
    uVar3 = str_op_1000_3da4((char *)((u32)param_2 & 0xffff0000 | ZEXT24(pcVar1)));
    DrawText16(0x400,(RECT16 *)CONCAT22(0x1050,&rect_a),uVar3,(LPCSTR)CONCAT22(uVar2,pcVar1),HVar3);
    iStack8 = ((y3 - y7) + iStack8) / 0x2 + rect_12.y;
    y7 += iStack8;
    rect_a = ((x3 - x6) + rect_a) / 0x2 + rect_12.x;
    x6 += rect_a;
  }
    // 1050:5ec2
  handle = CreatePen16(COLORREF_1050_5ec2,0x1,0x0);
    // 1050:5ebe
  hgdiobj16_00 = CreatePen16(COLORREF_1050_5ebe,0x1,0x0);
  hgdiobj_2 = SelectObject16(handle,hdc16_dev_ctx_1);
  if (uStack34 != 0x0) {
    iStack78 = 0x0;
    do {
      MoveTo16(y4,x4,hdc16_dev_ctx_1);
      LineTo16(y4,x5,hdc16_dev_ctx_1);
      LineTo16(y6,x5,hdc16_dev_ctx_1);
      LineTo16(y6,x4,hdc16_dev_ctx_1);
      LineTo16(y4,x4,hdc16_dev_ctx_1);
      y6 += 0x1;
      x4 += 0x1;
      x5 += -0x1;
      y4 += -0x1;
      iStack78 += 0x1;
    } while (iStack78 < 0x1);
  }
  if ((struct71_var4->field4_0x4 & 0x2) == 0x0) {
    y2 = (i16)((u32)rect_12 >> 0x10);
    x2 = (i16)_x3;
    iStack78 = 0x0;
    x1 = rect_12.x;
    y1 = y3;
    do {
      SelectObject16(handle,hdc16_dev_ctx_1);
      MoveTo16(y1,x1,hdc16_dev_ctx_1);
      LineTo16(y1,x2,hdc16_dev_ctx_1);
      LineTo16(y2,x2,hdc16_dev_ctx_1);
      do {
        SelectObject16(hgdiobj16_00,hdc16_dev_ctx_1);
        LineTo16(y2,x1,hdc16_dev_ctx_1);
        LineTo16(y1,x1,hdc16_dev_ctx_1);
      } while ((int)(hdc16_dev_ctx_1 + 0x1) < 0x2);
      y2 += 0x1;
      x1 += 0x1;
      x2 += -0x1;
      y1 += -0x1;
      iStack78 += 0x1;
    } while (iStack78 < 0x2);
  }
  else {
    MoveTo16(y3,rect_12.x,hdc16_dev_ctx_1);
    LineTo16(rect_12.y,rect_12.x,hdc16_dev_ctx_1);
    LineTo16(rect_12.y,x3 + 0x1,hdc16_dev_ctx_1);
    if (iStack28 != 0x0) {
      iStack8 += 0x2;
      rect_a += 0x2;
      x6 += 0x2;
      y7 += 0x2;
    }
  }
  MoveTo16(0x0,0x0,hdc16_dev_ctx_1);
  if (iStack28 != 0x0) {
    if ((struct71_var4->field4_0x4 & 0x4) == 0x0) {
      uVar4 = u16_1050_5ec8;
      puVar1 = u16_1050_5ec6;
      if (uStack32 != 0x0) {
        uVar4 = u16_1050_5ecc;
        puVar1 = u16_1050_5eca;
      }
      SetTextColor16(CONCAT22(uVar4,puVar1),hdc16_dev_ctx_1);
      SetBkColor16(0x1000000,hdc16_dev_ctx_1);
      pcVar1 = &struct71_var4->field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      uVar6 = str_op_1000_3da4((char *)((u32)param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      DrawText16(0x1,(RECT16 *)CONCAT22(0x1050,&rect_a),uVar6,(LPCSTR)CONCAT22(uVar7,pcVar1),HVar3);
      uVar13 = s_tile2_bmp_1050_1538;
      uVar14 = 0x9c8d;
      SetTextColor16(CONCAT22(HVar3,uVar7),hdc16_dev_ctx_1);
      SetBkColor16(CONCAT22(uVar13,uVar14),hdc16_dev_ctx_1);
    }
    else {
      hdc_lt_gray_brush_1 = GetStockObject16(LTGRAY_BRUSH);
      uVar4_00 = 0x0;
      uVar12 = 0x0;
      pcVar1 = &struct71_var4->field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      cch_1 = str_op_1000_3da4((char *)((u32)param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      GrayString16(y7 - iStack8,x6 - rect_a,iStack8,rect_a,cch_1,CONCAT22(uVar7,pcVar1),
                   (void *)CONCAT22(uVar12,uVar4_00),hdc_lt_gray_brush_1,HVar3);
    }
    if (hgdiobj_1 != 0x0) {
      SelectObject16(hgdiobj_1,hdc16_dev_ctx_1);
    }
  }
  SelectObject16(hgdiobj_2,hdc16_dev_ctx_1);
  SetMapMode16(mode,hdc16_dev_ctx_1);
  DeleteObject16(handle);
  DeleteObject16(hgdiobj16_00);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_42),param_1);
  return;
}



void win_op_1040_9cde(LPARAM lparam_param_1,WPARAM16 wparam_param_2,u16 msg_param_3,HWND16 hwnd_param_4,
                     u16 param_5,u16 param_6,u32 param_7)

{
  u8 *pbVar1;
  i16 iVar2;
  u8 bVar3;
  WPARAM16 WVar4;
  BOOL16 BVar5;
  u32 uVar9;
  u16 uVar6;
  i16 uVar8;
  u16 uVar10;
  let mut win_long_11: i32;
  WPARAM16 *pWVar11;
  LRESULT LVar12;
  u32 uVar13;
  u8 uVar14;
  u8 uVar15;
  RECT16 rect_a [0x2];
  i16 iVar3;
  Struct57*paVar7;
  HWND16 hwnd_dlg_8;

  uVar10 = ((u32)param_7 >> 0x10);
  win_long_11 = GetWindowLong16(0x0,hwnd_param_4);
  paVar7 = (Struct57*)CONCAT22(uVar10,(int)((u32)win_long_11 >> 0x10));
  iVar3 = (i16)win_long_11;
  uVar8 = (i16)((win_long_11 & 0xffff0000U) >> 0x10);
  if (msg_param_3 == 0x30) {
    *(WPARAM16 *)(iVar3 + 0x5a) = wparam_param_2;
  }
  else {
    if (msg_param_3 < 0x31) {
      if (msg_param_3 == 0x1f) {
        (iVar3 + 0x4) = 0x0;
        ReleaseCapture16();
        return;
      }
      if (0x1f < msg_param_3) goto LAB_1040_a1ae;
      bVar3 = (u8)msg_param_3;
      if (bVar3 == 0x8) {
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xf7;
        uVar6 = 0x0;
        BVar5 = IsWindow16(wparam_param_2);
        if (BVar5 != 0x0) {
          uVar13 = SendMessage16(0x0,0x0,0x87,wparam_param_2);
          uVar6 = ((uVar13 & 0x20) == 0x0);
        }
        (u32)(iVar3 + 0x56) = 0x0;
        SendMessage16(0x0,*(WPARAM16 *)(iVar3 + 0x5c),0x401,*(HWND16 *)(iVar3 + 0x2));
        if (((iVar3 + 0x5c) != 0x0) && ((iVar3 + 0x5c) != win_long_11)) {
          SendDlgItemMessage16((u32)uVar6,0x1,0x404,*(INT16 *)(iVar3 + 0x5c),*(HWND16 *)(iVar3 + 0x2));
        }
        (iVar3 + 0x5c) = 0x0;
      }
      else if (bVar3 < 0x9) {
        if (bVar3 == 0x1) {
          pWVar11 = (WPARAM16 *)GetWindowLong16(0x0,hwnd_param_4);
          iVar2 = (int)pWVar11;
          uVar10 = (((u32)pWVar11 & 0xffff0000) >> 0x10);
          (iVar2 + 0x2) = ((int)lparam_param_1 + 0x8);
          WVar4 = GetDlgCtrlID16(hwnd_param_4);
          *pWVar11 = WVar4;
          (u32)(iVar2 + 0x56) = (u32)((int)lparam_param_1 + 0x12);
          unk_str_op_1000_3d3e
                    ((char *)((u32)pWVar11 & 0xffff0000 | (u32)(iVar2 + 0x6)),*(char **)((int)lparam_param_1 + 0x16)
                    );
          if ((*(u8 *)((int)lparam_param_1 + 0x12) & 0x1) != 0x0) {
            SendMessage16(0x0,*pWVar11,0x401,*(HWND16 *)((int)lparam_param_1 + 0x8));
          }
          if ((((int)lparam_param_1 + 0x14) & 0x800) == 0x0) {
            return;
          }
          pbVar1 = (u8 *)(iVar2 + 0x4);
          *pbVar1 = *pbVar1 | 0x4;
          return;
        }
        if (bVar3 == 0x2) {
          fn_ptr_1000_17ce((char *)win_long_11);
          SetWindowLong16(0x0,0x0,hwnd_param_4);
          return;
        }
        if (bVar3 != 0x7) goto LAB_1040_a1ae;
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x8;
        LVar12 = SendMessage16(0x0,0x0,0x400,*(HWND16 *)(iVar3 + 0x2));
        iVar2 = (int)LVar12;
        if (((int)((u32)LVar12 >> 0x10) == 0x534b) && ((iVar3 + 0x5c) = iVar2, iVar2 != win_long_11))
        {
          SendDlgItemMessage16(0x1,0x0,0x404,iVar2,*(HWND16 *)(iVar3 + 0x2));
        }
        SendMessage16(0x0,*(WPARAM16 *)win_long_11,0x401,*(HWND16 *)(iVar3 + 0x2));
        (u32)(iVar3 + 0x56) = 0x1;
      }
      else if (bVar3 == 0xa) {
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfb;
        if (wparam_param_2 == 0x0) {
          pbVar1 = (u8 *)(iVar3 + 0x4);
          *pbVar1 = *pbVar1 | 0x4;
        }
      }
      else {
        if (bVar3 != 0xc) {
          if (bVar3 == 0xf) {
            draw_op_1040_9948(hwnd_param_4,(astruct_71 *)win_long_11);
            return;
          }
          goto LAB_1040_a1ae;
        }
        if (lparam_param_1 != 0x0) {
          unk_str_op_1000_3d3e((char *)(win_long_11 & 0xffff0000U | (u32)(iVar3 + 0x6)),(char *)lparam_param_1);
        }
      }
      goto LAB_1040_9e20;
    }
    if (msg_param_3 == 0x200) {
      if ((*(u8 *)(iVar3 + 0x4) & 0x1) == 0x0) {
        return;
      }
      GetClientRect16(rect_a,(HWND16)0x1050);
      iVar2 = (iVar3 + 0x4);
      BVar5 = PtInRect16((POINT16)lparam_param_1,rect_a);
      if (BVar5 == 0x0) {
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfd;
      }
      else {
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x2;
      }
      lparam_param_1 = (iVar3 + 0x4) - iVar2;
    }
    else {
      if (msg_param_3 < 0x201) {
        uVar9 = msg_param_3 - 0x81;
        if (uVar9 == 0x0) {
          uVar14 = 0x0;
          uVar15 = 0x0;
          mem_op_1000_179c(0x5e,paVar7);
          uVar6 = paVar7 | uVar9;
          if (uVar6 == 0x0) {
            uVar9 = 0x0;
            uVar6 = 0x0;
          }
          else {
            pass1_1040_9824((u32 *)CONCAT22(paVar7,uVar9));
          }
          SetWindowLong16(CONCAT22(uVar6,uVar9),CONCAT11(uVar15,uVar14),hwnd_param_4);
          return;
        }
        if (msg_param_3 == 0x87) {
          return;
        }
        if (msg_param_3 == 0x100) {
          if ((wparam_param_2 == 0x26) || (wparam_param_2 == 0x25)) {
            hwnd_dlg_8 = *(HWND16 *)(iVar3 + 0x2);
            BVar5 = 0x1;
          }
          else {
            if ((wparam_param_2 != 0x28) && (wparam_param_2 != 0x27)) {
              if (((wparam_param_2 == 0x20) || (wparam_param_2 == 0xd)) && (PTR_LOOP_1050_5ed8 == NULL)) {
                PTR_LOOP_1050_5ed8 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
                pbVar1 = (u8 *)(iVar3 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
                goto LAB_1040_9e20;
              }
              goto LAB_1040_a1ae;
            }
            hwnd_dlg_8 = *(HWND16 *)(iVar3 + 0x2);
            BVar5 = 0x0;
          }
          hwnd_dlg_8 = GetNextDlgTabItem16(BVar5,hwnd_param_4,hwnd_dlg_8);
          SetFocus16(hwnd_dlg_8);
          return;
        }
        if ((msg_param_3 == 0x101) && (PTR_LOOP_1050_5ed8 != NULL)) {
          PTR_LOOP_1050_5ed8 = NULL;
          pbVar1 = (u8 *)(iVar3 + 0x4);
          *pbVar1 = *pbVar1 & 0xfd;
          InvalidateRect16(0x1,NULL,0x0);
          UpdateWindow16(hwnd_param_4);
          SendMessage16(0x0,*(WPARAM16 *)win_long_11,0x111,*(HWND16 *)(iVar3 + 0x2));
          return;
        }
LAB_1040_a1ae:
        DefWindowProc16(lparam_param_1,wparam_param_2,msg_param_3,hwnd_param_4);
        return;
      }
      if (msg_param_3 == 0x201) {
LAB_1040_9e74:
        SetFocus16(hwnd_param_4);
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x3;
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
        SetCapture16(hwnd_param_4);
        return;
      }
      if (msg_param_3 == 0x202) {
        ReleaseCapture16();
        GetClientRect16(rect_a,(HWND16)0x1050);
        if ((*(u8 *)(iVar3 + 0x4) & 0x1) == 0x0) {
          return;
        }
        pbVar1 = (u8 *)(iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfc;
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
        BVar5 = PtInRect16((POINT16)lparam_param_1,rect_a);
        if (BVar5 == 0x0) {
          return;
        }
        PostMessage16(0x0,*(WPARAM16 *)win_long_11,0x111,*(HWND16 *)(iVar3 + 0x2));
        return;
      }
      if (msg_param_3 == 0x203) goto LAB_1040_9e74;
      if (msg_param_3 != 0x404) goto LAB_1040_a1ae;
      if (wparam_param_2 == 0x1) {
        (u32)(iVar3 + 0x56) = 0x1;
      }
      else {
        (u32)(iVar3 + 0x56) = 0x0;
      }
    }
  }
  if ((int)lparam_param_1 == 0x0) {
    return;
  }
LAB_1040_9e20:
  InvalidateRect16(0x1,NULL,0x0);
  UpdateWindow16(hwnd_param_4);
  return;
}



u16 * pass1_1040_a204(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void make_proc_inst_1040_a234(u8 *param_1,u8 *param_2,u16 param_3,u32 param_4)

{
  pass1_1040_b040((Struct57*)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0xa4e8;
  (param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (_PTR_LOOP_1050_5edc == NULL) {
    PTR_LOOP_1050_5edc = MakeProcInstance16(HINSTANCE16_1050_038c,call_win_proc_1040_a40e);
  }
  *(void **)(param_1 + 0xc) = PTR_LOOP_1050_5edc;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 0x1;
  PTR_LOOP_1050_5ee0 = param_1;
  PTR_LOOP_1050_5ee2 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void free_proc_inst_1040_a294(StructD *param_1)

{
  u16 in_stack_0000ffde;

  param_1->address_offset_field_0x0 = 0xa4e8;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -0x1;
  if (PTR_LOOP_1050_5eda == NULL) {
    FreeProcInstance16(_PTR_LOOP_1050_5edc);
    PTR_LOOP_1050_5edc = NULL;
  }
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



u32 pass1_1040_a2cc(u16 param_1,u8 *param_2,i16 param_3,param_4: u32,u32 param_5)

{
  u16 uVar1;

  if (param_5 == 0x1826) {
    if (((int)param_5 == 0x1) || ((0x1 < (int)param_5 - 0x1U && ((int)param_5 - 0x3U < 0x2)))) {
      uVar1 = 0x1;
    }
    else {
      uVar1 = 0x0;
    }
    return (u32)uVar1;
  }
  pass1_1040_b54a(param_2,(astruct_903 *)CONCAT22((int)param_4,param_3),(param_4 >> 0x10),param_5);
  return CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 win_msg_1040_a308(astruct_49 *param_1,u16 param_2,u16 param_3,u16 param_4)

{
  i16 *piVar1;
  u32 uVar2;
  u32 in_EDX;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  LRESULT LVar6;
  u32 *puVar7;
  char *pcVar8;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  WPARAM16 WVar9;
  u16 UVar10;
  u16 uVar11;
  u16 in_stack_0000fff2;
  i16 iStack12;

  uVar3 = ((u32)in_EDX >> 0x10);
  SendMessage16(0x0,0x0,0x405,param_4);
  LVar6 = SendMessage16(0x0,0x0,0xb,param_4);
  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar2 = (u32)(iVar4 + 0x90);
  if (((int)uVar2 + 0x10) == 0x0) {
    WVar9 = 0x0;
    UVar10 = 0x401;
    uVar11 = param_4;
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
    SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
  }
  else {
    puVar7 = mixed_1010_20ba((Struct57*)CONCAT22(uVar3,(int)((u32)LVar6 >> 0x10)),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fff2,0x3),in_stack_0000fe9a,in_stack_0000ffbe,
                             in_stack_0000ffc4,in_stack_0000ffc8);
    for (iStack12 = 0x0; uVar2 = (u32)(iVar4 + 0x90), piVar1 = (i16 *)((int)uVar2 + 0x10),
        *piVar1 != iStack12 && iStack12 <= *piVar1; iStack12 += 0x1) {
      WVar9 = 0x0;
      UVar10 = 0x401;
      uVar2 = (u32)(iVar4 + 0x90);
      uVar2 = (u32)((int)uVar2 + 0xc);
      uVar11 = param_4;
      pcVar8 = load_string_1010_ac92
                         (puVar7,((u32)puVar7 >> 0x10),((int)uVar2 + iStack12 * 0x2));
      SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
    }
  }
  LVar6 = SendMessage16(0x0,0x1,0xb,param_4);
  return CONCAT22((int)((u32)LVar6 >> 0x10),0x1);
}



void get_dlg_item_1040_a3d0(astruct_49 *param_1)

{
  let mut lVar1: i32;
  HWND16 dlg_item;
  astruct_49 *iVar3;
  astruct_49 *uVar2;

  uVar2 = (astruct_49 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_49 *)param_1;
  if (iVar3->field143_0x90 != 0x0) {
    lVar1 = iVar3->field143_0x90;
    *(HWND16 *)((int)lVar1 + 0x14) = iVar3->field6_0x6;
    dlg_item = GetDlgItem16(0x1826,iVar3->field6_0x6);
    win_msg_1040_a308(param_1,0x0,0x0,dlg_item);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 call_win_proc_1040_a40e(u16 param_1,HWND16 param_2,param_3: u32,LPARAM param_4)

{
  LPVOID func;
  u16 uVar1;
  code **ppcVar2;
  WPARAM16 wparam;
  u16 uVar6;
  u32 uVar3;
  u32 uStack6;
  u32 *puVar3;
  u32 uVar5;

  uStack6 = 0x0;
  wparam = (WPARAM16)(param_3 >> 0x10);
  if (param_4 == 0x19) {
    ppcVar2 = (code **)((int)(u32)_PTR_LOOP_1050_5ee0 + 0x34);
    uStack6 = (**ppcVar2)();
    param_1 = (uStack6 >> 0x10);
  }
  else {
    if (param_4 == 0x86) {
      ppcVar2 = (code **)((int)(u32)_PTR_LOOP_1050_5ee0 + 0x20);
      uVar3 = (**ppcVar2)();
      return uVar3;
    }
    if (param_4 == 0x110) {
      uVar3 = win_msg_1040_a308(_PTR_LOOP_1050_5ee0,param_2,param_3,wparam);
      return uVar3;
    }
  }
  if (uStack6 != 0x0) {
    return uStack6 & 0xffff | (u32)param_1 << 0x10;
  }
  uVar6 = ((u32)_u16_1050_5bc8 >> 0x10);
  func = *(LPVOID *)((int)_u16_1050_5bc8 + 0x4);
  uVar1 = ((int)_u16_1050_5bc8 + 0x6);
  if ((uVar1 | func) == 0x0) {
    return (u32)uVar1 << 0x10;
  }
  uVar3 = CallWindowProc16(CONCAT22(param_3,param_2),wparam,param_4,(HWND16)((u32)param_4 >> 0x10),func);
  return uVar3;
}



StructD * pass1_1040_a4c2(StructD *param_1,u8 param_2)

{
  free_proc_inst_1040_a294(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_a564(u32 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x0;
  ((int)param_1 + 0x4) = 0x0;
  (u32)((int)param_1 + 0x6) = 0x0;
  return;
}



void pass1_1040_a582(u32 *param_1)

{
  fn_ptr_1000_17ce((char *)*param_1);
  return;
}



void struct_1040_a598(astruct_259 *param_1)

{
  astruct_259 *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_259 *)param_1;
  param_1->field0_0x0 = 0x0;
  iVar1->field1_0x2 = 0x0;
  iVar1->field2_0x6 = 0x0;
  iVar1->field3_0xa = 0x0;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0x10 = 0x0;
  iVar1->field6_0x12 = 0x0;
  iVar1->field7_0x14 = 0x0;
  iVar1->field8_0x16 = 0x0;
  return;
}



void pass1_1040_a5d0(StructD *param_1)

{
  u16 uVar1;
  u16 uVar2;
  StructD *iVar4;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  uVar1 = iVar4->address_offset_field_0x2;
  uVar2 = iVar4->hfile_0x4;
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1000_54e8((u8 *)0xa582,&PTR_LOOP_1050_1040,(uVar1 - 0x2),0xa,uVar1,uVar2);
    fn_ptr_1000_17ce((char *)CONCAT22(uVar2,uVar1 - 0x2));
  }
  fn_ptr_1000_17ce(*(char **)&iVar4->field7_0xc);
  return;
}



void string_1040_a626(u16 param_1,u16 *param_2,char *param_3)

{
  u16 uVar1;

  uVar1 = str_op_1008_60e8(param_1,param_3);
  *param_2 = uVar1;
  ((int)param_2 + 0x2) = param_1;
  return;
}



void pass1_1040_a640(Struct57*param_1,param_2: u32,u16 param_3)

{
  Struct57*iVar1;
  Struct57*uVar1;

  struct_1040_b082(param_1,CONCAT22(param_3,0x1f1));
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (u32)&iVar1[0x1].field3_0x6 = param_2;
  iVar1[0x1].field5_0xa = 0x0;
  &iVar1[0x1].field_0x5c = 0x0;
  param_1->field0_0x0 = 0xac08;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1040_a67e(astruct_750 *struct750_param_1,i16 param_2,u16 param_3,u16 param_4)

{
  bool bVar1;
  HBRUSH16 brush_handle_var2;
  astruct_750 *struct750_var4;
  u16 uVar3;
  u32 uVar4;
  i16 iStack14;
  i16 *piVar1;
  u16 uVar2;
  astruct_751 *iVar2;

  uVar3 = ((u32)struct750_param_1 >> 0x10);
  struct750_var4 = (astruct_750 *)struct750_param_1;
  if (struct750_var4->hbrush16_field142_0x8e == 0x0) {
    brush_handle_var2 = CreateSolidBrush16(WHITE_BRUSH);
    struct750_var4->hbrush16_field142_0x8e = brush_handle_var2;
  }
  if (_u16_1050_5ee8 == 0x0) {
    uVar4 = pass1_1008_4d72((u32)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar2 = (uVar4 >> 0x10);
    iVar2 = (astruct_751 *)uVar4;
    _u16_1050_5ee8 = (u32)CONCAT12(iVar2->field_0x94,CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
    u16_1050_5eec = CONCAT11(iVar2->field_0x3e5,iVar2->field_0x3e6);
    u16_1050_5eee = iVar2->field996_0x3e4;
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar1 = false;
    for (iStack14 = 0x0; piVar1 = &struct750_var4->field233_0xea, *piVar1 != iStack14 && iStack14 <= *piVar1;
        iStack14 += 0x1) {
      if ((&struct750_var4->field_0x9a + iStack14 * 0x2) == param_2) {
        bVar1 = true;
        break;
      }
    }
    if (bVar1) {
      u16_1050_5ee8 = u16_1050_5eec;
      u16_1050_5eea = u16_1050_5eee;
    }
  }
  SetTextColor16(CONCAT22(u16_1050_5eea,u16_1050_5ee8),param_4);
  SetBkColor16(0x1000000,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_a784(undefined1 param_1,u16 param_2,i16 param_3,i16 param_4,u16 param_5,u32 param_6)

{
  HWND16 hwnd;
  i16 iVar1;

  iVar1 = param_3;
  if (param_6 != 0xeb) {
    if (param_6 == 0x1f4) {
      msg_box_op_1040_a85a(0x0,param_2,CONCAT22(param_4,param_3));
      return;
    }
    if (param_6 == 0x1f7) {
      PTR_LOOP_1050_5ef0 = (u32)(param_3 + 0x94);
      pass1_1038_af40(param_3,param_2,_PTR_LOOP_1050_5b7c,(param_3 + 0x8),0x23);
      PostMessage16(0x0,0x2,0x111,*(HWND16 *)(param_3 + 0x6));
      return;
    }
    if (param_6 != 0x17d8) {
      pass1_1040_b54a((u8 *)param_2,(astruct_903 *)CONCAT22(param_4,param_3),param_5,param_6);
      return;
    }
    SetWindowPos16(0x6,0xed,0x237,0x0,0x0,0x0,*(HWND16 *)(param_3 + 0x6));
    hwnd = GetDlgItem16(0x17d8,*(HWND16 *)(param_3 + 0x6));
    iVar1 = (int)s_tile2_bmp_1050_1538;
    EnableWindow16(0x0,hwnd);
    (param_3 + 0x98) = 0x1;
    param_4 = param_3;
  }
  win_ui_dlg_op_1040_a94a(param_2,CONCAT22(param_4,iVar1));
  return;
}



void pass1_1040_a84a(undefined1 param_1,u16 param_2,u32 param_3)

{
  win_ui_dlg_op_1040_a94a(param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_op_1040_a85a(char *param_1,u16 param_2,u32 param_3)

{
  short in_buf_len_5;
  u16 in_register_0000000a;
  Struct57*paVar1;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)0x1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_dlg_op_1040_a94a(u16 param_1,u32 param_2)

{
  i16 *piVar1;
  u32 uVar2;
  u32 uVar3;
  u8 *puVar4;
  u16 uVar5;
  u8 *value;
  char *pcVar6;
  u16 uVar7;
  HWND16 HVar8;
  u16 value_00;
  u8 *puVar9;
  u16 in_register_0000000a;
  u32 uVar10;
  i16 iVar11;
  i16 iVar12;
  u16 unaff_SI;
  u16 uVar13;
  u16 uVar14;
  bool bVar15;
  u32 *puVar16;
  u32 uVar17;
  u16 in_stack_0000fd7c;
  u16 in_stack_0000fea0;
  u16 in_stack_0000fea6;
  u16 in_stack_0000feaa;
  u16 uStack288;
  i16 iStack278;
  i16 iStack276;
  u8 local_102 [0x100];

  puVar16 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            (u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fd7c,in_stack_0000fea0,in_stack_0000fea6,
                            in_stack_0000feaa);
  puVar4 = (u8 *)((u32)puVar16 >> 0x10);
  uVar5 = puVar16;
  uVar13 = (param_2 >> 0x10);
  iVar11 = (int)param_2;
  puVar9 = puVar4;
  pass1_1010_c3c2(puVar4,uVar5,puVar4,CONCAT22(0x1050,local_102),(u32)(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1f2,*(HWND16 *)(iVar11 + 0x6));
  pass1_1010_c320((char *)puVar9,uVar5,puVar4,(u8 *)CONCAT22(0x1050,local_102),(u32)(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1774,*(HWND16 *)(iVar11 + 0x6));
  string_op_1010_c446(puVar9,(u32)puVar16,CONCAT22(0x1050,local_102),(u32)(iVar11 + 0x94));
  value = local_102;
  SetDlgItemText16(CONCAT22(0x1050,value),0x1773,*(HWND16 *)(iVar11 + 0x6));
  pass1_1030_6ddc((u32)(iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f5,*(HWND16 *)(iVar11 + 0x6));
  pass1_1030_6e14((u32)(iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f6,*(HWND16 *)(iVar11 + 0x6));
  if ((iVar11 + 0x98) != 0x0) {
    struct_1010_dd5e(uVar5,puVar4,(u32)(iVar11 + 0x94));
    if ((puVar9 | value) != 0x0) {
      uVar3 = (u32)(iVar11 + 0x94);
      uVar14 = ((u32)uVar3 >> 0x10);
      iVar12 = (int)uVar3;
      uVar2 = (u32)(iVar12 + 0x26);
      uVar10 = (u32)(iVar12 + 0x28);
      iStack276 = 0x1798;
      iStack278 = 0x17c3;
      (iVar11 + 0xea) = 0x0;
      uVar17 = uVar2;
      for (uStack288 = 0x1; (int)uStack288 < 0x25; uStack288 += 0x1) {
        if (uVar2 == 0x0) {
          value_00 = 0x0;
          uVar10 = 0x0;
        }
        else {
          uVar17 = pass1_1020_bae6(uVar17,uVar10,uVar2,CONCAT22(uStack288,(int)(uVar2 >> 0x10)));
          uVar10 = uVar17 >> 0x10;
          value_00 = uVar17;
        }
        uVar7 = uVar10;
        bVar15 = *(i32 *)(value + uStack288 * 0x4) != 0x0;
        if (bVar15) {
          pcVar6 = string_1020_c0d8(uStack288);
          SetDlgItemText16(CONCAT22((int)uVar10,pcVar6),iStack276,*(HWND16 *)(iVar11 + 0x6));
          SetDlgItemInt16(0x0,(value + uStack288 * 0x4),iStack278,*(HWND16 *)(iVar11 + 0x6));
        }
        uVar7 |= value_00;
        if (uVar7 != 0x0) {
          if (!bVar15) {
            pcVar6 = string_1020_c0d8(uStack288);
            SetDlgItemText16(CONCAT22((int)uVar10,pcVar6),iStack276,*(HWND16 *)(iVar11 + 0x6));
          }
          SetDlgItemInt16(0x0,value_00,iStack278,*(HWND16 *)(iVar11 + 0x6));
          iVar12 = (iVar11 + 0xea);
          HVar8 = GetDlgItem16(iStack276,*(HWND16 *)(iVar11 + 0x6));
          *(HWND16 *)(iVar11 + iVar12 * 0x2 + 0x9a) = HVar8;
          piVar1 = (i16 *)(iVar11 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          iVar12 = (iVar11 + 0xea);
          uVar7 = GetDlgItem16(iStack278,*(HWND16 *)(iVar11 + 0x6));
          *(HWND16 *)(iVar11 + iVar12 * 0x2 + 0x9a) = uVar7;
          piVar1 = (i16 *)(iVar11 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          bVar15 = true;
        }
        uVar17 = (u32)uVar7;
        if (bVar15) {
          iStack276 += 0x1;
          iStack278 += 0x1;
        }
      }
    }
  }
  return;
}



StructD * pass1_1040_abe2(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffda;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_ac84(u8 *param_1,Struct57*param_2,u16 param_3)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  Struct57*uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1f3));
  uVar1 = (Struct57*)((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  param_2->field0_0x0 = 0xafc4;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  (u32)&iVar1[0x1].field3_0x6 = PTR_LOOP_1050_5ef0;
  PTR_LOOP_1050_5ef0 = 0x0;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3d),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field5_0xa = puVar2;
  iVar1[0x1].field6_0xc = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_ace8(StructD *param_1)

{
  u16 uVar1;
  u16 in_stack_0000ffde;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xafc4;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



void pass1_1040_ad14(u32 param_1)

{
  u16 in_DX;

  win_ui_op_1040_ae04(in_DX,param_1);
  return;
}



void pass1_1040_ad24(u8 *param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  if (param_5 == 0xeb) {
    win_ui_op_1040_ae04(param_1,CONCAT22(param_3,param_2));
  }
  else {
    if (param_5 != 0x1f0) {
      pass1_1040_b54a(param_1,(astruct_903 *)CONCAT22(param_3,param_2),param_4,param_5);
      return;
    }
    msg_box_ui_op_1040_ad66(0x0,param_1,CONCAT22(param_3,param_2));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_ui_op_1040_ad66(char *param_1,u16 param_2,u32 param_3)

{
  short in_buf_len_5;
  u16 in_register_0000000a;
  Struct57*paVar1;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)0x1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_ae04(u16 param_1,u32 param_2)

{
  i16 iVar1;
  bool bVar2;
  WORD *pWVar3;
  i16 iVar4;
  char *pcVar5;
  let mut lVar6: i32;
  u8 *puVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  i16 iVar9;
  i32 *plVar10;
  u16 unaff_SI;
  u16 uVar11;
  u16 uVar12;
  u32 *puVar13;
  u32 uVar14;
  u32 uVar15;
  char *lp_string;
  u32 uVar16;
  u16 in_stack_0000fd84;
  u16 in_stack_0000fea8;
  u16 in_stack_0000feae;
  u16 in_stack_0000feb2;
  i16 iStack280;
  WORD local_102 [0x80];

  puVar13 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            (u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fd84,in_stack_0000fea8,in_stack_0000feae,
                            in_stack_0000feb2);
  puVar7 = (u8 *)((u32)puVar13 >> 0x10);
  uVar11 = (param_2 >> 0x10);
  iVar9 = (int)param_2;
  pass1_1010_c3c2(puVar7,puVar13,puVar7,CONCAT22(0x1050,local_102),(u32)(iVar9 + 0x94));
  pWVar3 = local_102;
  SetDlgItemText16(CONCAT22(0x1050,pWVar3),0x1778,*(HWND16 *)(iVar9 + 0x6));
  uVar14 = struct_op_1030_73a8(*(astruct_419 **)(iVar9 + 0x94),pWVar3,puVar7);
  iVar4 = (int)uVar14 + 0x20;
  uVar15 = pass1_1030_8326();
  uVar16 = uVar15 >> 0x10;
  iVar1 = 0x0;
  bVar2 = false;
  for (iStack280 = 0x0; uVar8 = uVar16, iStack280 < 0xa; iStack280 += 0x1) {
    uVar12 = ((uVar14 & 0xffff0000) >> 0x10);
    if (((iStack280 * 0xc + iVar4 + 0x2) | (iStack280 * 0xc + iVar4)) != 0x0) {
      plVar10 = (i32 *)(iStack280 * 0xc + iVar4);
      pcVar5 = string_op_1020_c222((plVar10 + 0x1));
      SetDlgItemText16(CONCAT22(uVar8,pcVar5),iVar1 + 0x1d2,*(HWND16 *)(iVar9 + 0x6));
      lVar6 = *plVar10 - uVar15;
      wsprintf16(local_102,(char *)0x5ef41050,0xf4,CONCAT22((int)lVar6,0x1050),(int)((u32)lVar6 >> 0x10));
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1dc,*(HWND16 *)(iVar9 + 0x6));
      uVar16 = unk_load_str_op_1010_8c96
                         (uVar8,(u32)(iVar9 + 0x98),CONCAT22(0x1050,local_102),
                          uVar14 & 0xffff0000 | ZEXT24(plVar10));
      uVar16 &= 0xffff;
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1e6,*(HWND16 *)(iVar9 + 0x6));
      iVar1 += 0x1;
      bVar2 = true;
    }
  }
  if (!bVar2) {
    lp_string = load_string_1010_847e(_u16_1050_14cc,0x531);
    SetDlgItemText16((u32)lp_string,0x1d2,*(HWND16 *)(iVar9 + 0x6));
  }
  return;
}



StructD * pass1_1040_af9e(StructD *param_1,u8 param_2)

{
  pass1_1040_ace8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_b040(Struct57*param_1,param_2: u32,u16 param_3)

{
  Struct57*iVar1;
  Struct57*uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,((int)param_2 + 0x12),param_3);
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  (u32)&iVar1[0x1].field1_0x2 = param_2;
  param_1->field0_0x0 = 0xb772;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



void struct_1040_b082(Struct57*param_1,u32 param_2)

{
  Struct57*iVar1;
  Struct57*uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_2,(param_2 >> 0x10));
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  (u32)&iVar1[0x1].field1_0x2 = 0x0;
  param_1->field0_0x0 = 0xb772;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



void pass1_1040_b0bc(Struct57*param_1,param_2: u32,u32 param_3)

{
  Struct57*iVar1;
  u16 uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_3,(param_3 >> 0x10));
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  (u32)&iVar1[0x1].field1_0x2 = param_2;
  param_1->field0_0x0 = 0xb772;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_draw_op_1040_b0f8(u16 param_1,StructD *param_2)

{
  u16 uVar1;
  u16 uVar2;
  Struct57*in_EDX;
  StructD *iVar3;
  u16 uVar3;
  u32 *puVar3;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  u16 uStack22;
  char *pcStack10;
  u32 in_stack_0000ffe8;

  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (StructD *)param_2;
  param_2->address_offset_field_0x0 = 0xb772;
  iVar3->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  _param_1 = (StructD *)CONCAT22(uStack22,0x32);
  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)_param_1,in_stack_0000fe92,in_stack_0000ffb6,
                           in_stack_0000ffbc,in_stack_0000ffc0);
  pass1_1010_7b8c((u32)puVar3,&iVar3->field_0x6);
  if (&iVar3->field_0x8e != 0x0) {
    DeleteObject16(*(HGDIOBJ16 *)&iVar3->field_0x8e);
    &iVar3->field_0x8e = 0x0;
  }
  uVar1 = &iVar3->field_0x90;
  uVar2 = &iVar3->field_0x92;
  pcStack10 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    _param_1 = (StructD *)CONCAT22(uVar2,uVar1);
    pass1_1040_a5d0(_param_1);
    fn_ptr_1000_17ce(pcStack10);
  }
  ui_cleanup_op_1040_782c(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_b17c(u8 *param_1,param_2: u32,u32 param_3)

{
  i16 *piVar1;
  u32 uVar2;
  char *pcVar3;
  u16 in_register_0000000a;
  Struct57*paVar4;
  u32 uVar5;
  i16 iVar6;
  u16 unaff_SI;
  u16 uVar7;
  u32 *puVar8;
  u16 in_stack_0000fe94;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc2;
  u8 **ppuVar9;
  u16 *puStack12;
  i16 iStack4;

  paVar4 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  iStack4 = 0x0;
  while( true ) {
    uVar7 = (param_2 >> 0x10);
    iVar6 = (int)param_2;
    piVar1 = *(i16 **)(iVar6 + 0x90);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    paVar4 = (Struct57*)((u32)paVar4 & 0xffff0000 | (u32)piVar1 >> 0x10);
    uVar2 = (u32)((int)piVar1 + 0x2);
    (iStack4 * 0xa + (int)uVar2 + 0x4) = (iStack4 * 0x2 + (int)param_3);
    iStack4 += 0x1;
  }
  ppuVar9 = (u8 **)CONCAT22(unaff_SI,0x3);
  puVar8 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,ppuVar9,in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,
                           in_stack_0000ffc2);
  uVar5 = (u32)puVar8 >> 0x10;
  uVar2 = (u32)(iVar6 + 0x90);
  puStack12 = (u16*)((int)uVar2 + 0x2);
  for (iStack4 = 0x0; piVar1 = *(i16 **)(iVar6 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 0x1) {
    ppuVar9 = (u8 **)((u32)ppuVar9 & 0xffff0000);
    uVar2 = (u32)(iVar6 + 0x90);
    uVar2 = (u32)((int)uVar2 + 0x6);
    pcVar3 = pass1_1010_b038((u8 *)puVar8,uVar2,((u32)uVar2 >> 0x10),
                             *(u8 **)((int)puStack12 + 0x4),(int)((u32)ppuVar9 >> 0x10));
    string_1040_a626(uVar5,puStack12,(char *)CONCAT22(uVar5,pcVar3));
    puStack12 = (u16 *)((u32)puStack12 & 0xffff0000 | (u32)((int)puStack12 + 0xa));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1040_b230(u16 param_1,StructB *param_2)

{
  code **ppcVar1;
  INT16 cy_caption_1;
  u16 in_register_0000000a;
  Struct57*paVar2;
  u16 uVar3;
  u16 in_stack_0000fe6e;
  u16 in_stack_0000ff92;
  u16 in_stack_0000ff98;
  u16 in_stack_0000ff9c;
  i16 *piVar3;
  u16 uVar4;
  char *pcVar5;
  i16 local_1a;
  i16 iStack24;
  i16 iStack22;
  i16 iStack20;
  i16 iStack18;
  i16 iStack16;
  i16 iStack14;
  i16 iStack12;
  u32 *puStack10;
  i16 local_6;
  i16 local_4;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_2);
  if (PTR_LOOP_1050_5ef8 == (u8 *)((int)&u32_1050_0004 + 0x1)) {
    PTR_LOOP_1050_5ef8 = NULL;
  }
  pcVar5 = (char *)CONCAT22(0x1050,&local_4);
  piVar3 = &local_6;
  uVar4 = SUB42(0x1050,0x0);
  puStack10 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(piVar3,0x48),in_stack_0000fe6e,
                              in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
  pass1_1008_3e94((u16 *)((u32)puStack10 & 0xffff0000 | (u32)((int)puStack10 + 0xe)),(u16 *)CONCAT22(uVar4,piVar3),
                  pcVar5);
  uVar3 = ((u32)puStack10 >> 0x10);
  iStack12 = ((int)puStack10 + 0xa);
  iStack14 = ((int)puStack10 + 0xc);
  cy_caption_1 = GetSystemMetrics16(SM_CYCAPTION);
  iStack16 = cy_caption_1 * (int)PTR_LOOP_1050_5ef8 + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 0x1;
  iStack18 = iStack16 + local_6;
  iStack16 += local_4;
  uVar4 = ((u32)param_2 >> 0x10);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_1a),*(HWND16 *)((int)param_2 + 0x6));
  if (iStack14 < (iStack20 - iStack24) + iStack18) {
    iStack18 = -0x2 - ((iStack20 - iStack24) - iStack14);
  }
  if (iStack12 < (iStack22 - local_1a) + iStack16) {
    iStack16 = -0x2 - ((iStack22 - local_1a) - iStack12);
  }
  SetWindowPos16(0x1,0x0,0x0,iStack18,iStack16,0x0,*(HWND16 *)((int)param_2 + 0x6));
  ppcVar1 = (code **)((int)(u32)param_2 + 0x6c);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_2);
  return;
}



u16 pass1_1040_b316(u32 *param_1,u16 param_2,u16 param_3,u16 param_4,param_5: i16)

{
  code **ppcVar1;
  u16 uStack4;

  if (param_5 == 0xf) {
    ppcVar1 = (code **)((int)*param_1 + 0x60);
    uStack4 = (**ppcVar1)();
  }
  else if (param_5 == 0x111) {
    ppcVar1 = (code **)((int)*param_1 + 0x10);
    (**ppcVar1)();
    uStack4 = 0x1;
  }
  else {
    uStack4 = pass1_1040_79c0(param_1,(i16 *)param_2,param_3,param_4,param_5);
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_b372(param_1: u32,HWND16 hwnd_param_2,u16 param_3,HDC16 hdc_param_4)

{
  u16 uVar1;
  i16 dlg_ctrl_id;
  HBRUSH16 local_brush_handle;
  u32 uVar4;
  u16 DX_REG;
  u16 uVar5;
  u32 uVar2;
  astruct_798 *iVar1;
  u16 uVar3;
  u16 uVar6;

  uVar5 = (param_1 >> 0x10);
  if (((int)param_1 + 0x8e) == 0x0) {
    local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
    *(HBRUSH16 *)((int)param_1 + 0x8e) = local_brush_handle;
  }
  if (_PTR_LOOP_1050_5efa == 0x0) {
    uVar2 = pass1_1008_4d72((u32)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (uVar2 >> 0x10);
    iVar1 = (astruct_798 *)uVar2;
    PTR_LOOP_1050_5efa = (u32)CONCAT12(iVar1->field_0x94,CONCAT11(iVar1->field_0x95,iVar1->field_0x96));
  }
  if (param_3 < 0x4) {
LAB_1040_b3ea:
    dlg_ctrl_id = GetDlgCtrlID16(hwnd_param_2);
    if (dlg_ctrl_id == 0x14c) {
      uVar3 = 0xffff;
      uVar6 = 0x0;
      goto LAB_1040_b41a;
    }
    if (dlg_ctrl_id == 0x175) {
      uVar3 = 0xff;
      uVar6 = 0x0;
      goto LAB_1040_b41a;
    }
  }
  else if (param_3 != 0x4) {
    if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
      return;
    }
    goto LAB_1040_b3ea;
  }
  uVar3 = PTR_LOOP_1050_5efa;
  uVar6 = (_PTR_LOOP_1050_5efa >> 0x10);
LAB_1040_b41a:
  SetTextColor16(CONCAT22(uVar6,uVar3),hdc_param_4);
  SetBkColor16(0x1000000,hdc_param_4);
  return;
}



void show_win_1040_b43c(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x70);
  (**ppcVar1)();
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



void pass1_1040_b45e(u32 param_1)

{
  u32 uVar1;
  i16 *piVar2;
  i16 iVar3;
  u16 uVar4;
  i16 iStack8;
  u16 *puStack6;

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x90) != 0x0) {
    uVar1 = (u32)(iVar3 + 0x90);
    ((int)uVar1 + 0x14) = (iVar3 + 0x6);
    uVar1 = (u32)(iVar3 + 0x90);
    puStack6 = (u16 *)(u32)((int)uVar1 + 0x2);
    for (iStack8 = 0x0; piVar2 = *(i16 **)(iVar3 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 0x1) {
      uVar1 = (u32)((int)puStack6 + 0x2);
      SetDlgItemText16(CONCAT22((int)uVar1,*puStack6),(INT16)((u32)uVar1 >> 0x10),*(HWND16 *)(iVar3 + 0x6));
      puStack6 = (u16 *)((u32)puStack6 & 0xffff0000 | (u32)((int)puStack6 + 0xa));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_b4c8(u8 *param_1,u32 param_2)

{
  i16 iVar1;
  u32 uVar2;
  u16 uVar3;
  i16 iVar4;
  u16 in_register_0000000a;
  Struct57*paVar5;
  u32 uVar6;
  u16 uVar7;
  u32 *puVar8;
  u16 in_stack_0000fea2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000fffa;

  paVar5 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  uVar7 = (param_2 >> 0x10);
  if (*(i32 *)((int)param_2 + 0x90) != 0x0) {
    puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fffa,0x32),in_stack_0000fea2,
                             in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
    uVar6 = (u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10;
    uVar3 = puVar8;
    uVar2 = (u32)((int)param_2 + 0x90);
    iVar1 = ((int)uVar2 + 0xa);
    iVar4 = iVar1 + -0x4;
    if (iVar4 == 0x0) {
      ui_op_1010_79aa(puVar8,0xfd9,0x0);
      if (iVar4 == 0x0) {
        uVar7 = 0xe;
LAB_1040_b50f:
        unk_win_op_1010_7300
                  (uVar6,(Struct57*)((u32)puVar8 & 0xffff0000 | (u32)uVar3),CONCAT22(iVar4,iVar4),uVar7,
                   CONCAT22(iVar4,iVar4));
        return;
      }
    }
    else if (((0x0 < iVar1 + -0x5) && (!SBORROW2(iVar1 + -0x5,0x1))) &&
            (iVar4 = iVar1 + -0x7, iVar4 == 0x0 || iVar1 + -0x6 < 0x1)) {
      ui_op_1010_79aa(puVar8,0xfda,0x0);
      if (iVar4 == 0x0) {
        uVar7 = 0xd;
        goto LAB_1040_b50f;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_b54a(u8 *param_1,astruct_903 *param_2,u16 param_3,u32 param_4)

{
  StructD *pSVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  i16 iVar5;
  StructD *pSVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  Struct57*paVar8;
  u32 uVar9;
  astruct_515 *iVar6;
  u16 uVar10;
  u32 *puVar11;
  Struct57*paVar12;
  u16 in_stack_0000fe8e;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbc;
  u8 uVar13;
  u8 uVar14;
  u16 uVar15;
  u16 uVar16;
  u16 uVar17;
  u16 in_stack_0000ffe6;

  paVar8 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (param_4 == 0xea) {
    ppcVar2 = (code **)((int)(u32)param_2 + 0x5c);
    (**ppcVar2)();
  }
  else if (param_4 == 0xeb) {
    puVar11 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x3),in_stack_0000fe8e,
                              in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
    uVar7 = ((u32)puVar11 >> 0x10);
    pSVar1 = *(StructD **)((int)param_2 + 0x90);
    if (pSVar1 != NULL) {
      uVar10 = ((u32)pSVar1 >> 0x10);
      uVar15 = 0x1010;
      pSVar6 = pSVar1;
      pass1_1010_ad64((u32)pSVar1,uVar7,puVar11,CONCAT22(((int)pSVar1 + 0xa),uVar7),
                      (u32)((int)pSVar1 + 0x6));
      ((int)param_2 + 0x90) = (int)pSVar6;
      ((int)param_2 + 0x92) = uVar7;
      if ((uVar7 | ((int)param_2 + 0x90)) == 0x0) {
        *(StructD **)((int)param_2 + 0x90) = pSVar1;
      }
      else {
        if (pSVar1 != NULL) {
          pass1_1040_a5d0(pSVar1);
          uVar15 = 0x1000;
          fn_ptr_1000_17ce((char *)pSVar1);
        }
        ppcVar2 = (code **)((int)(u32)param_2 + 0x70);
        (**ppcVar2)(uVar15,param_2);
      }
    }
  }
  else {
    if (param_4 == 0x1790) {
      paVar12 = (Struct57*)
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = (u32)paVar8 & 0xffff0000 | (u32)paVar12 >> 0x10;
      uVar3 = (u32)((int)param_2 + 0x90);
      uVar3 = (u32)((int)uVar3 + 0x6);
      iVar4 = pass1_1010_7d38(paVar12,((u32)paVar12 >> 0x10),(int)uVar3,((u32)uVar3 >> 0x10)
                             );
      iVar5 = iVar4;
      ui_op_1010_79aa(paVar12,0xfab,0x0);
      if (iVar5 != 0x0) {
        return;
      }
      if (iVar4 == 0x0) {
        uVar3 = (u32)((int)param_2 + 0x90);
        uVar10 = ((u32)uVar3 >> 0x10);
        iVar6 = (astruct_515 *)uVar3;
        uVar3 = iVar6->field6_0x6;
        uVar16 = uVar3;
        uVar17 = ((u32)uVar3 >> 0x10);
        uVar15 = 0x14;
      }
      else {
        uVar3 = (u32)((int)param_2 + 0x90);
        uVar10 = ((u32)uVar3 >> 0x10);
        iVar6 = (astruct_515 *)uVar3;
        uVar3 = iVar6->field6_0x6;
        uVar16 = uVar3;
        uVar17 = ((u32)uVar3 >> 0x10);
        uVar15 = 0x9;
      }
      uVar13 = (u8)uVar10;
      uVar14 = (u8)(uVar10 >> 0x8);
    }
    else if (param_4 == 0x1824) {
      paVar12 = (Struct57*)
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = (u32)paVar8 & 0xffff0000 | (u32)paVar12 >> 0x10;
      iVar6 = (astruct_515 *)paVar12;
      uVar3 = (u32)((int)param_2 + 0x90);
      ui_op_1010_79aa(paVar12,0xfc5,*(i32 *)((int)uVar3 + 0x6));
      if (iVar6 != NULL) {
        return;
      }
      uVar3 = (u32)((int)param_2 + 0x90);
      uVar3 = (u32)((int)uVar3 + 0x6);
      uVar16 = uVar3;
      uVar17 = ((u32)uVar3 >> 0x10);
      uVar15 = 0x12;
      uVar13 = 0x0;
      uVar14 = 0x0;
    }
    else {
      if (param_4 != 0x1830) {
        post_win_msg_1040_7b3c((StructC *)param_2,param_3,param_4,param_4);
        return;
      }
      paVar12 = (Struct57*)
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = (u32)paVar8 & 0xffff0000 | (u32)paVar12 >> 0x10;
      iVar6 = (astruct_515 *)paVar12;
      uVar3 = (u32)((int)param_2 + 0x90);
      ui_op_1010_79aa(paVar12,0xfb6,*(i32 *)((int)uVar3 + 0x6));
      if (iVar6 != NULL) {
        return;
      }
      uVar3 = (u32)((int)param_2 + 0x90);
      uVar3 = (u32)((int)uVar3 + 0x6);
      uVar16 = uVar3;
      uVar17 = ((u32)uVar3 >> 0x10);
      uVar15 = 0xc;
      uVar13 = 0x0;
      uVar14 = 0x0;
    }
    unk_win_op_1010_7300(uVar9,paVar12,CONCAT13(uVar14,CONCAT12(uVar13,iVar6)),uVar15,CONCAT22(uVar17,uVar16));
  }
  return;
}



void destroy_window_1040_b726(param_1: u32,param_2: i16)

{
  code **fn_ptr_1;

  if (param_2 != 0x0) {
    fn_ptr_1 = (code **)((int)(u32)param_1 + 0x78);
    (**fn_ptr_1)();
  }
  DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



StructD * pass1_1040_b74c(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffde;

  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_b7ee(Struct57*param_1,i32 param_2,u16 param_3)

{
  Struct57*iVar1;
  Struct57*uVar1;
  u16 uVar2;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar1 = (Struct57*)((u32)param_1 >> 0x10);
  iVar1 = (Struct57*)param_1;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  (u32)&iVar1[0x1].field18_0x22 = 0x0;
  iVar1[0x1].field21_0x26 = 0x0;
  &iVar1[0x1].field_0x28 = 0x0;
  param_1->field0_0x0 = 0xbeba;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar2 = ((u32)param_2 >> 0x10);
    (u32)&iVar1[0x1].field18_0x22 = (u32)((int)param_2 + 0x6);
    iVar1[0x1].field21_0x26 = ((int)param_2 + 0x14);
  }
  return;
}



u16 pass1_1040_b864(u32 *param_1,i16 *param_2,u16 param_3,u16 param_4,param_5: i16)

{
  code **ppcVar1;
  u16 uVar2;

  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((i16 *)CONCAT22(param_3,param_2));
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x7c);
    (**ppcVar1)();
  }
  return 0x1;
}



void pass1_1040_b8be(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x80);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_1040_b8d2(u16 param_1,StructB *param_2)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  Struct57*paVar4;
  u16 uVar5;
  u16 uVar6;
  i16 iVar7;
  u16 uVar8;
  u16 uVar9;
  u16 in_register_0000000a;
  Struct57*paVar10;
  Struct57*paVar12;
  StructB *struct_b_10;
  StructB *struct_b_10_hi;
  u16 uVar13;
  u32 *puVar14;
  u16 *puVar15;
  u16 in_stack_0000fe3a;
  u16 in_stack_0000fe3e;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ff64;
  u16 in_stack_0000ff68;
  u16 in_stack_0000ff6c;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffe4;
  Struct57*paVar11;

  paVar10 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_2);
  puVar14 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe4,0x31),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  paVar10 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)puVar14 >> 0x10);
  paVar4 = (Struct57*)puVar14;
  struct_b_10_hi = (StructB *)((u32)param_2 >> 0x10);
  struct_b_10 = (StructB *)param_2;
  struct_b_10[0x7].field6_0xc = (u8 *)paVar4;
  struct_b_10[0x7].field7_0xe = ((u32)puVar14 >> 0x10);
  mem_op_1000_179c(0xa,paVar10);
  uVar8 = paVar10 | paVar4;
  paVar11 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
  if (uVar8 == 0x0) {
    (u32)&struct_b_10[0x7].max_count_field_0x10 = 0x0;
  }
  else {
    puVar15 = struct_1040_bf3e((astruct_442 *)CONCAT22(paVar10,paVar4),struct_b_10->lpvoid_field_0x8);
    paVar11 = (Struct57*)((u32)paVar11 & 0xffff0000 | (u32)puVar15 >> 0x10);
    paVar4 = (Struct57*)puVar15;
    struct_b_10[0x7].max_count_field_0x10 = paVar4;
    struct_b_10[0x7].field5_0xa = (u32 *)((u32)puVar15 >> 0x10);
  }
  pass1_1040_bfde(*(void **)&struct_b_10[0x7].max_count_field_0x10,&struct_b_10[0x7].field6_0xc);
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = (Struct57*)paVar11 | paVar4;
  paVar10 = (Struct57*)((u32)paVar11 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar10,paVar4,(Struct57*)paVar11,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0x10a),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = (Struct57*)paVar10 | paVar4;
  paVar11 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar11,paVar4,(Struct57*)paVar10,0x1,0xa0028,0x0,0x840085,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0x10b),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = (Struct57*)paVar11 | paVar4;
  paVar10 = (Struct57*)((u32)paVar11 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar10,paVar4,(Struct57*)paVar11,0x1,0xa0046,0x0,0x860087,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0x10d),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = (Struct57*)paVar10 | paVar4;
  paVar11 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar11,paVar4,(Struct57*)paVar10,0x1,0xa0064,0x0,0x880089,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0x10e),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = (Struct57*)paVar11 | paVar4;
  paVar10 = (Struct57*)((u32)paVar11 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar10,paVar4,(Struct57*)paVar11,0x1,0xa0082,0x0,0x820083,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0x10c),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = (Struct57*)paVar10 | paVar4;
  paVar11 = (Struct57*)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar11,paVar4,(Struct57*)paVar10,0x1,0xa00d2,0x0,0x8a008b,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0xbbb),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = (Struct57*)paVar11 | paVar4;
  paVar10 = (Struct57*)((u32)paVar11 & 0xffff0000);
  paVar12 = (Struct57*)((u32)paVar10 | (u32)uVar8);
  if (uVar8 == 0x0) {
    paVar4 = NULL;
  }
  else {
    pass1_1008_3bd6((u32)paVar12,paVar4,(Struct57*)paVar11,0x1,0xa00a0,0x8e,0x8c008d,
                    CONCAT22(struct_b_10->lpvoid_field_0x8,0xbbc),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
    paVar10 = paVar12;
  }
  puVar14 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe4,0x3),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar3 = ((u32)puVar14 >> 0x10);
  uVar2 = puVar14;
  uVar9 = uVar3;
  uVar5 = pass1_1010_a5ac(uVar2,uVar3,(u32)struct_b_10[0x8].field8_0x10);
  uVar6 = pass1_1010_ac62(uVar5,uVar9,uVar2,uVar3,0x1e);
  if (uVar6 != 0x0) {
    pass1_1010_a5ca(uVar6,uVar9,uVar2,uVar3,uVar5);
    if (0x0 < (int)uVar6) {
      pass1_1010_a58a(uVar6,uVar9,uVar2,uVar3,uVar5);
      if (uVar6 == 0x0) goto LAB_1040_bb26;
    }
  }
  enable_win_1040_9234(CONCAT22((int)paVar10,paVar4),0x0);
LAB_1040_bb26:
  uVar1 = (u32)&struct_b_10[0x7].field6_0xc;
  iVar7 = (int)uVar1;
  uVar1 &= 0xffff0000;
  uVar13 = (uVar1 >> 0x10);
  SetWindowPos16(0x40,*(INT16 *)(iVar7 + 0x10),*(INT16 *)(iVar7 + 0xe),*(INT16 *)(iVar7 + 0xc),
                 *(INT16 *)(uVar1 | iVar7 + 0xa),0x0,(HWND16)struct_b_10->lpvoid_field_0x8);
  return;
}



u16 pass1_1040_bb5a(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void destroy_win_1040_bb78(astruct_35 *param_1)

{
  u16 uVar1;
  BOOL16 is_window;
  astruct_35 *pstruct35_5;
  astruct_35 *pstruct35_hi;
  u16 unaff_CS;
  u32 *puVar1;
  code **fn_ptr_1;

  pstruct35_hi = (astruct_35 *)((u32)param_1 >> 0x10);
  pstruct35_5 = (astruct_35 *)param_1;
  if (pstruct35_5->hwnd_0xb6 != 0x0) {
    // 0x1538
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    is_window = IsWindow16(pstruct35_5->hwnd_0xb6);
    if (is_window != 0x0) {
    // 0x1538
      unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
      DestroyWindow16(pstruct35_5->hwnd_0xb6);
    }
  }
  pstruct35_5->hwnd_0xb6 = 0x0;
  puVar1 = pstruct35_5->field148_0x94;
  uVar1 = pstruct35_5->field149_0x96;
  if ((uVar1 | puVar1) != 0x0) {
    fn_ptr_1 = (code **)*puVar1;
    (**fn_ptr_1)(unaff_CS,puVar1,uVar1,0x1);
  }
  (u32)&pstruct35_5->field148_0x94 = 0x0;
  pstruct35_5->field150_0x98 = 0x0;
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar18
// WARNING: Unable to use type for symbol uVar19
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_bbe2(u8 *param_1,HWND16 param_2,astruct_900 *param_3,u16 param_4,u16 param_5,u32 param_6)

{
  u16 uVar2;
  code **ppcVar3;
  u32 *puVar5;
  u16 uVar6;
  BOOL16 BVar7;
  i16 iVar7;
  u16 uVar8;
  u16 uVar7;
  u16 uVar9;
  HWND16 hwnd;
  u16 uVar10;
  u16 uVar13;
  u16 uVar11;
  u16 uVar12;
  u16 in_register_0000000a;
  Struct57*paVar14;
  u32 uVar15;
  u32 *puVar16;
  Struct57*paVar17;
  u32 uVar16;
  u16 in_stack_0000fe84;
  u16 in_stack_0000fe86;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb4;
  u16 uVar21;
  u16 uStack30;
  RECT16 local_a;
  i16 iStack6;
  i16 iStack4;
  u32 uVar1;
  u32 *puVar4;
  u16 uVar17;
  u16 uVar18;
  u16 uVar19;
  u16 in_stack_0000ffde;
  u16 uVar20;

  paVar14 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  if (param_6 != 0x10c) {
    if (param_6 < 0x10d) {
      if (param_6 == 0xfa) {
        ppcVar3 = (code **)((int)*param_3->field148_0x98 + 0x18);
        (**ppcVar3)();
        return;
      }
      if (param_6 == 0x10a) {
        GetClientRect16(&local_a,(HWND16)0x1050);
        puVar5 = param_3->field148_0x98;
        local_a.y += 0x3;
        local_a.x = ((int)puVar5 + 0x1a) + -0x9;
        iStack6 += -0x3;
        iStack4 += -0x3;
        InvalidateRect16(0x1,&local_a,(HWND16)0x1050);
        unk_destroy_win_op_1010_2fa0((astruct_873 *)param_3->field148_0x98);
        pass1_1010_32c0((u32)param_3->field148_0x98,0x0);
        pass1_1010_2ee2(param_3->field148_0x98);
        return;
      }
      if (param_6 != 0x10b) {
LAB_1040_be78:
        pass1_1040_b54a(param_1,(astruct_903 *)CONCAT22(param_4,param_3),param_5,param_6);
        return;
      }
      puVar4 = param_3->field148_0x98;
      uVar2 = ((int)puVar4 + 0x12);
      uVar21 = uVar2;
      puVar16 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(uVar2,0x3),in_stack_0000fe84,
                                in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      uVar8 = ((u32)puVar16 >> 0x10);
      uStack30 = puVar16;
      uVar6 = uStack30;
      uVar13 = uVar8;
      pass1_1010_a5ca(uStack30,uVar8,uStack30,uVar8,uVar21);
      if ((uVar2 != 0x70) && (uVar6 == 0x0)) {
        return;
      }
      uVar1 = param_3->field169_0xb0;
      uVar18 = uVar1;
      uVar19 = (uVar1 >> 0x10);
      puVar5 = param_3->field148_0x98;
      uVar17 = ((int)puVar5 + 0x12);
    }
    else {
      if (param_6 != 0x10d) {
        if (param_6 == 0x10e) {
          paVar17 = (Struct57*)
                    mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x32),in_stack_0000fe86,
                                    in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
          uVar15 = (u32)paVar14 & 0xffff0000 | (u32)paVar17 >> 0x10;
          iVar7 = (i16)paVar17;
          ui_op_1010_79aa(paVar17,0xfc6,param_3->field169_0xb0);
          if (iVar7 != 0x0) {
            return;
          }
          unk_win_op_1010_7300(uVar15,paVar17,0x0,0x13,param_3->field169_0xb0);
          return;
        }
        if (param_6 != 0xbbb) {
          if (param_6 == 0xbbc) {
            puVar16 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x3),in_stack_0000fe86,
                                      in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
            uVar2 = ((u32)puVar16 >> 0x10);
            uVar8 = puVar16;
            uVar13 = uVar2;
            uVar7 = pass1_1010_a5ac(uVar8,uVar2,param_3->field169_0xb0);
            uVar9 = uVar7;
            pass1_1010_a58a(uVar7,uVar13,uVar8,uVar2,uVar7);
            if (uVar9 == 0x0) {
              pass1_1010_a568(0x0,uVar13,uVar8,uVar2,uVar7);
            }
            hwnd = GetDlgItem16(0xbbc,param_3->field6_0x6);
            EnableWindow16(0x0,hwnd);
            return;
          }
          goto LAB_1040_be78;
        }
        if ((param_3->field171_0xb6 == 0x0) || (BVar7 = IsWindow16(param_3->field171_0xb6), BVar7 == 0x0)) {
          uVar16 = pass1_1038_af40(param_3,paVar14,_PTR_LOOP_1050_5b7c,param_3->field6_0x6,0x1b);
          param_3->field171_0xb6 = *(HWND16 *)((int)uVar16 + 0x6);
          ShowWindow16(0x1,param_3->field171_0xb6);
          return;
        }
        param_2 = param_3->field171_0xb6;
        goto LAB_1040_bd39;
      }
      puVar16 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x3),in_stack_0000fe86,
                                in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
      uVar13 = ((u32)puVar16 >> 0x10);
      uStack30 = puVar16;
      uVar15 = param_3->field169_0xb0;
      uVar18 = uVar15;
      uVar19 = (uVar15 >> 0x10);
      uVar17 = 0x71;
      uVar8 = uVar13;
    }
    pass1_1010_a5ec(uVar13,uStack30,uVar8,uVar17,CONCAT22(uVar19,uVar18));
    if ((param_3->field170_0xb4 != 0x0) && (BVar7 = IsWindow16(param_3->field170_0xb4), BVar7 != 0x0)) {
      SendMessage16(0x0,0xeb,0x111,param_3->field170_0xb4);
    }
  }
  param_2 = param_3->field6_0x6;
LAB_1040_bd39:
  DestroyWindow16(param_2);
  return;
}



StructD * pass1_1040_be94(StructD *param_1,u8 param_2)

{
  u16 in_stack_0000ffda;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1040_bf3e(astruct_442 *param_1,u16 param_2)

{
  astruct_442 *iVar1;
  astruct_442 *uVar1;

  uVar1 = (astruct_442 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_442 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  param_1->field0_0x0 = 0x3aa8;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = param_2;
  param_1->field0_0x0 = 0x3ab0;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field3_0x6 = 0x0;
  param_1->field0_0x0 = 0xc53e;
  iVar1->field1_0x2 = (int)&PTR_LOOP_1050_1040;
  return &param_1->field0_0x0;
}



void pass1_1040_bf92(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xc53e;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  pass1_1010_1ea6((u32)&iVar1->field_0x6,(StructD *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  unk_destroy_win_op_1010_2fa0(*(astruct_873 **)&iVar1->field_0x6);
  param_1->address_offset_field_0x0 = 0x3ab0;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



void pass1_1040_bfde(void *param_1,u32 *param_2)

{
  code **ppcVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  (iVar3 + 0x6) = param_2;
  ppcVar1 = (code **)((int)*param_2 + 0x4);
  (**ppcVar1)();
  uVar2 = (u32)(iVar3 + 0x6);
  ((int)uVar2 + 0x22) = (iVar3 + 0x4);
  pass1_1010_2ee2((iVar3 + 0x6));
  return;
}



void invalidate_rect_1040_c028(param_1: u32,param_2: i16)

{
  u32 uVar1;
  u32 uVar2;
  u32 uVar3;
  u16 uVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  i16 iVar8;
  u16 uVar10;
  RECT16 *erase;
  RECT16 *rect;
  HWND16 hwnd;
  RECT16 local_a;
  i16 iStack6;
  i16 iStack4;
  i16 *piVar9;

  iVar8 = (int)param_1;
  uVar10 = (param_1 >> 0x10);
  if (param_2 == 0x8) {
    GetClientRect16(&local_a,(HWND16)0x1050);
    uVar1 = (u32)(iVar8 + 0x6);
    uVar3 = (u32)(iVar8 + 0x6);
    iVar6 = ((int)uVar3 + 0x16);
    uVar3 = (u32)(iVar8 + 0x6);
    local_a.x = ((int)uVar3 + 0x1a);
    uVar3 = (u32)(iVar8 + 0x6);
    local_a.y = ((int)uVar3 + 0x1c);
    if (iVar6 != 0x0) {
      if (iVar6 < 0x2) {
        iVar5 = 0x1;
      }
      else {
        iVar5 = 0x2;
      }
      uVar2 = (u32)((iVar6 - iVar5) * 0x4 + (int)uVar1 + 0x2a);
      iVar6 = (int)uVar2;
      uVar2 &= 0xffff0000;
      local_a.x = (iVar6 + 0x22) + (uVar2 | iVar6 + 0x1e);
    }
    uVar1 = (u32)(iVar8 + 0x6);
    iStack6 = ((int)uVar1 + 0x1e);
    iStack4 += -0x5;
  }
  else {
    if (param_2 != 0x9) {
      if (param_2 != 0xa) {
        return;
      }
      uVar1 = (u32)(iVar8 + 0x6);
      uVar7 = (int)uVar1 + 0x2a;
      if (((iVar8 + 0x8) | uVar7) == 0x0) {
        return;
      }
      uVar3 = (u32)(iVar8 + 0x6);
      uVar2 = (u32)((((int)uVar3 + 0x16) + -0x1) * 0x4 + uVar7);
      iVar8 = (int)uVar2;
      uVar2 &= 0xffff0000;
      piVar9 = (i16 *)(uVar2 | iVar8 + 0x1e);
      uVar10 = (uVar2 >> 0x10);
      local_a.y = (iVar8 + 0x20) + -0x8;
      local_a.x = *piVar9;
      iStack6 = (iVar8 + 0x22) + *piVar9;
      iStack4 = (iVar8 + 0x20);
      rect = &local_a;
      hwnd = (HWND16)0x1050;
      erase = NULL;
      goto LAB_1040_c19d;
    }
    local_a.x = 0x0;
    local_a.y = 0x0;
    iStack6 = 0x0;
    iStack4 = 0x0;
    GetClientRect16(&local_a,(HWND16)0x1050);
    uVar1 = (u32)(iVar8 + 0x6);
    local_a.x = ((int)uVar1 + 0x1a);
    uVar1 = (u32)(iVar8 + 0x6);
    iStack6 = ((int)uVar1 + 0x1e);
    iStack4 += -0x5;
    uVar1 = (u32)(iVar8 + 0x6);
    uVar3 = (u32)(iVar8 + 0x6);
    iVar6 = ((int)uVar3 + 0x16);
    if (0x0 < iVar6) {
      uVar1 = (u32)((int)uVar1 + iVar6 * 0x4 + 0x26);
      iVar6 = (int)uVar1;
      uVar4 = ((u32)uVar1 >> 0x10);
      local_a.y = (iVar6 + 0x20) + (iVar6 + 0x24);
    }
  }
  hwnd = *(HWND16 *)(iVar8 + 0x4);
  erase = &local_a;
  rect = (RECT16 *)0x1050;
LAB_1040_c19d:
  InvalidateRect16((BOOL16)erase,rect,hwnd);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2

void unk_draw_op_1040_c226(astruct_772 *struct_param_1)

{
  HPEN16 handle;
  HGDIOBJ16 obj_handle_var3;
  astruct_772 *iVar3;
  u16 uVar4;
  HDC16 hdc;
  RECT16 rect_var_32;
  i16 iStack46;
  i16 iStack44;
  u16 uStack42;
  i16 iStack40;
  HBRUSH16 hbrush_var38;
  HDC16 hdc16_var36;
  PAINTSTRUCT16 *paintstruct_22;
  u32 uVar1;
  u32 uVar2;

  uVar4 = ((u32)struct_param_1 >> 0x10);
  iVar3 = (astruct_772 *)struct_param_1;
  hdc16_var36 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar3->hwnd_0x4);
  hbrush_var38 = CreateSolidBrush16(0x8000);
  GetClientRect16(&rect_var_32,(HWND16)0x1050);
  uVar1 = iVar3->field5_0x6;
  iStack40 = ((int)uVar1 + 0x1a);
  uVar2 = iVar3->field5_0x6;
  uStack42 = ((int)uVar2 + 0x1c);
  rect_var_32.y += 0x2;
  rect_var_32.x = iStack40 + -0xa;
  iStack46 += -0x2;
  iStack44 += -0x2;
  FrameRect16(hbrush_var38,&rect_var_32,(HDC16)0x1050);
  DeleteObject16(hbrush_var38);
  hdc = hdc16_var36;
  handle = CreatePen16(0x8080,0x2,0x0);
  obj_handle_var3 = SelectObject16(handle,hdc);
  draw_line_1040_c302(struct_param_1,hdc16_var36);
  draw_op_1040_c38e(struct_param_1);
  obj_handle_var3 = SelectObject16(obj_handle_var3,hdc16_var36);
  DeleteObject16(obj_handle_var3);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar3->hwnd_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4

void draw_line_1040_c302(astruct_772 *param_1,HDC16 param_2)

{
  u32 uVar3;
  u16 uVar5;
  astruct_794 *iVar7;
  astruct_793 *iVar6;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u32 uVar2;
  u32 uVar4;
  i16 iVar1;
  u32 uVar1;

  uVar6 = ((u32)param_1 >> 0x10);
  uVar4 = (u32)((int)param_1 + 0x6);
  iVar1 = ((int)uVar4 + 0x16);
  if (0x1 < iVar1) {
    uVar2 = (u32)((int)param_1 + 0x6);
    uVar5 = uVar2;
    uVar5 = uVar5 + 0x2a;
    uVar1 = (u32)(uVar2 & 0xffff0000 | (u32)uVar5);
    iVar7 = (astruct_794 *)uVar1;
    iVar7 = (astruct_794 *)&iVar7->field_0x1e;
    uVar7 = ((uVar1 & 0xffff0000) >> 0x10);
    MoveTo16(iVar7->field32_0x20 + iVar7->field34_0x24,
             iVar7->field33_0x22 / 0x2 + (uVar1 & 0xffff0000 | ZEXT24(iVar7)),param_2);
    uVar3 = (u32)(uVar5 + iVar1 * 0x4 + -0x4);
    iVar6 = (astruct_793 *)uVar3;
    iVar6 = (astruct_793 *)&iVar6->field_0x1e;
    uVar3 &= 0xffff0000;
    uVar8 = (uVar3 >> 0x10);
    LineTo16(iVar6->field32_0x20,iVar6->field33_0x22 / 0x2 + (uVar3 | ZEXT24(iVar6)),param_2);
  }
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar6
// WARNING: Unable to use type for symbol uVar7

void draw_op_1040_c38e(astruct_772 *param_1)

{
  i16 iVar1;
  u32 uVar8;
  i16 iVar5;
  i16 iVar11;
  i16 y1;
  i16 iVar12;
  u16 in_DX;
  astruct_772 *iVar10;
  u16 uVar10;
  u16 uVar9;
  u16 uVar11;
  u16 unaff_SS;
  DWORD DVar10;
  u32 DVar9;
  HDC16 in_stack_00000008;
  i16 iStack26;
  i16 x3;
  i16 y5;
  i16 x2;
  i16 y4;
  u32 uVar2;
  u32 uVar1;
  u32 uVar5;
  i16 *x1;
  u32 uVar4;
  u32 uVar3;
  u32 uVar6;
  u32 uVar7;

  uVar10 = ((u32)param_1 >> 0x10);
  iVar10 = (astruct_772 *)param_1;
  uVar2 = iVar10->field5_0x6;
  iVar1 = ((int)uVar2 + 0x18);
  if ((iVar1 != 0x0) && (uVar4 = iVar10->field5_0x6, ((int)uVar4 + 0x16) != 0x0)) {
    iVar5 = iVar1;
    pass1_1010_2ee2((u32 *)iVar10->field5_0x6);
    for (iStack26 = 0x0; iStack26 < iVar1; iStack26 += 0x1) {
      uVar3 = (u32)(iStack26 * 0x4 + iVar5);
      iVar11 = (i16)uVar3;
      iVar11 = iVar11 + 0x1e;
      x1 = (i16 *)(uVar3 & 0xffff0000 | (u32)iVar11);
      uVar9 = ((uVar3 & 0xffff0000) >> 0x10);
      y1 = (iVar11 + 0x24) / 0x2 + (iVar11 + 0x20);
      MoveTo16(y1,*x1,in_stack_00000008);
      LineTo16(y1,*x1 + -0xf,in_stack_00000008);
      DVar10 = GetCurrentPosition16(in_stack_00000008);
      y5 = (i16)(DVar10 >> 0x10);
      x3 = (i16)DVar10;
      if (iStack26 == 0x0) {
        x2 = x3;
        y4 = y5;
      }
    }
    uVar6 = iVar10->field5_0x6;
    if (((int)uVar6 + 0x24) != 0x0) {
      y4 += -0xd;
    }
    uVar7 = iVar10->field5_0x6;
    if (((int)uVar7 + 0x26) != 0x0) {
      y5 += 0xd;
    }
    uVar8 = iVar10->field5_0x6;
    uVar5 = iVar10->field5_0x6;
    uVar1 = (u32)((int)uVar8 + ((int)uVar5 + 0x16) * 0x4 + 0x26);
    iVar12 = (i16)uVar1;
    iVar12 = iVar12 + 0x1e;
    uVar11 = ((uVar1 & 0xffff0000) >> 0x10);
    MoveTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),
             (iVar12 + 0x22) + (uVar1 & 0xffff0000 | (u32)iVar12),in_stack_00000008);
    LineTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),x2,in_stack_00000008);
    DVar9 = GetCurrentPosition16(in_stack_00000008);
    DVar9 = (int)(DVar9 >> 0x10);
    if (DVar9 < y4) {
      y4 = DVar9;
    }
    if (y5 < DVar9) {
      y5 = DVar9;
    }
    MoveTo16(y4,x2,in_stack_00000008);
    LineTo16(y5,x3,in_stack_00000008);
  }
  return;
}



u32 pass1_1040_c518(param_1: u32,u8 param_2)

{
  pass1_1040_bf92((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1040_c54a(astruct_65 *param_1,u16 param_2,u32 *param_3,u16 param_4,u32 param_5)

{
  code **ppcVar1;
  i16 iVar3;
  astruct_65 *iVar2;
  astruct_65 *uVar4;
  u32 *puVar4;
  u16 uVar5;
  u32 uVar6;

  iVar3 = ((int)param_3 + 0x12) + 0xc8;
  uVar6 = 0x0;
  uVar5 = 0x0;
  ppcVar1 = (code **)((int)*param_3 + 0x14);
  puVar4 = param_3;
  (**ppcVar1)();
  mixed_struct_op_1040_8fb8
            (param_5,param_1,0x0,(char *)CONCAT22((int)param_5,iVar3),puVar4,((u32)puVar4 >> 0x10),uVar5,
             uVar6,((u32)uVar6 >> 0x10),param_4);
  uVar4 = (astruct_65 *)((u32)param_1 >> 0x10);
  iVar2 = (astruct_65 *)param_1;
  &iVar2[0x1].field13_0x1c = param_3;
  iVar2[0x1].field15_0x20 = 0x0;
  iVar2[0x1].field16_0x22 = param_2;
  param_1->field0_0x0 = 0xc9f2;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  pass1_1040_c630((astruct_65 *)((u32)param_1 & 0xffff | ZEXT24(uVar4) << 0x10),param_5);
  return;
}



void pass1_1040_c5ac(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  StructD *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xc9f2;
  iVar4->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + -0x1;
  if (PTR_LOOP_1050_5f02 == NULL) {
    puVar1 = (u32 *)iVar4->field5_0x8;
    uVar2 = iVar4->field6_0xa;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = (u32 *)iVar4->field7_0xc;
    uVar2 = iVar4->field8_0xe;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
  }
  mix_win_ui_op_1040_911e(param_1);
  return;
}



u16 pass1_1040_c60e(astruct_65 *param_1)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x42) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x42);
    return ((int)uVar1 + 0x12);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_c630(astruct_65 *param_1,u32 param_2)

{
  u16 uVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  astruct_65 *iVar4;
  u16 uVar5;
  u16 unaff_CS;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_65 *)param_1;
  uVar3 = (u32)&iVar4[0x1].field13_0x1c;
  if (((int)uVar3 + 0x12) != 0x71) {
    iVar4[0x1].field8_0x10 = 0x5;
    (iVar4 + 0x1)->field0_0x0 = 0x5;
    iVar4[0x1].field1_0x2 = 0x5;
    uVar1 = iVar4[0x1].field8_0x10;
    iVar4[0x1].field5_0xa = uVar1;
    iVar4[0x1].field4_0x8 = uVar1;
    if (PTR_LOOP_1050_5f02 == NULL) {
      uVar4 = FUN_1010_830a(uVar1,param_2,unaff_CS,_u16_1050_14cc,0xff);
      PTR_LOOP_1050_5f04 = CONCAT22((int)param_2,uVar4);
      unaff_CS = 0x1010;
      uVar4 = FUN_1010_830a(uVar4,param_2,0x1010,_u16_1050_14cc,0x100);
      PTR_LOOP_1050_5f08 = CONCAT22((int)param_2,uVar4);
    }
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 0x1;
    (u32)&iVar4->field4_0x8 = PTR_LOOP_1050_5f04;
    (u32)&iVar4->field6_0xc = PTR_LOOP_1050_5f08;
    pass1_1040_9618((astruct_65 *)((u32)param_1 & 0xffff | (u32)uVar5 << 0x10));
    iVar4->field15_0x20 = 0x0;
    iVar4->field14_0x1e = 0xc8;
    iVar4->field16_0x22 = 0xa0;
    iVar4->field17_0x24 = iVar4[0x1].field3_0x6 + iVar4[0x1].field8_0x10;
    iVar4[0x1].field4_0x8 = iVar4[0x1].field8_0x10 * 0x3 + iVar4[0x1].field2_0x4;
    iVar4[0x1].field5_0xa = iVar4[0x1].field8_0x10;
    iVar4[0x1].field6_0xc = iVar4->field16_0x22 - iVar4[0x1].field8_0x10;
    ((int)&iVar4[0x1].field10_0x14 + 0x2) = 0x25;
    uVar3 = (u32)param_1;
    ppcVar2 = (code **)((int)uVar3 + 0x4);
    (**ppcVar2)(unaff_CS,param_1);
    ppcVar2 = (code **)((int)uVar3 + 0x8);
    (**ppcVar2)(unaff_CS,(char)param_1,uVar5);
  }
  return;
}



void pass1_1040_c71e(astruct_65 *param_1)

{
  astruct_65 *iVar1;
  astruct_65 *uVar1;

  pass1_1040_9252(param_1);
  uVar1 = (astruct_65 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_65 *)param_1;
  iVar1[0x1].field1_0x2 = iVar1->field17_0x24 / 0x2 - (int)iVar1[0x1].field3_0x6 / 0x2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1040_c74c(astruct_738 *param_1,u16 param_2,HDC16 hdc16_param_3,u16 param_4)

{
  u16 uVar2;
  HGDIOBJ16 hdc_black_brush_1;
  HPEN16 pen_handle_1;
  HGDIOBJ16 handle;
  HPALETTE16 hpalette_1;
  astruct_738 *struct_1;
  u16 uVar4;
  u16 uVar5;
  u32 uVar3;
  u16 uVar1;
  code **func_ptr_1;

  uVar4 = ((u32)_PTR_LOOP_1050_4230 >> 0x10);
  uVar2 = ((int)_PTR_LOOP_1050_4230 + 0x10);
  hpalette_1 = palette_op_1008_4e08
                         ((HPALETTE16)&hdc16_param_3,uVar2,
                          (astruct_13 *)CONCAT22(uVar2,((int)_PTR_LOOP_1050_4230 + 0xe)),
                          (HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&hdc16_param_3)));
  uVar5 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_738 *)param_1;
  struct_1->field66_0x46 = 0x1;
  hdc_black_brush_1 = GetStockObject16(BLACK_BRUSH);
  pen_handle_1 = CreatePen16(0x1000002,0x1,0x0);
  hdc_black_brush_1 = SelectObject16(hdc_black_brush_1,hdc16_param_3);
  handle = SelectObject16(pen_handle_1,hdc16_param_3);
  Rectangle16(struct_1->field35_0x24,struct_1->field34_0x22,0x0,0x0,hdc16_param_3);
  MoveTo16(0x0,struct_1->field51_0x36 * 0x2 + struct_1->field40_0x2a,hdc16_param_3);
  LineTo16(struct_1->field35_0x24,struct_1->field51_0x36 * 0x2 + struct_1->field40_0x2a,hdc16_param_3);
  SelectObject16(hdc_black_brush_1,hdc16_param_3);
  hdc_black_brush_1 = SelectObject16(handle,hdc16_param_3);
  DeleteObject16(hdc_black_brush_1);
  uVar3 = (u32)param_1;
  func_ptr_1 = (code **)((int)uVar3 + 0x10);
  (**func_ptr_1)((int)s_tile2_bmp_1050_1538,param_1,_param_2);
  func_ptr_1 = (code **)((int)uVar3 + 0x14);
  (**func_ptr_1)((int)s_tile2_bmp_1050_1538,struct_1,(char)((u32)param_1 >> 0x10),hdc16_param_3);
  struct_1->field66_0x46 = 0x0;
  hpalette_1 = SelectPalette16(0x0,hpalette_1,hdc16_param_3);
  DeleteObject16(hpalette_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void send_msg_1040_c85a(u32 param_1)

{
  PTR_LOOP_1050_5efe = param_1;
  SendMessage16(0x0,0xfa,0x111,*(HWND16 *)((int)param_1 + 0x1a));
  return;
}



void FUN_1040_c882(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void palette_op_1040_c886(astruct_769 *struct_param_1,u8 param_2,HDC16 hdc_param_3)

{
  u16 uVar1;
  HPALETTE16 palette_2;
  u16 uVar4;
  astruct_769 *struct_2;
  u16 uVar2;
  u16 uVar3;
  u16 unaff_CS;
  u32 *puStack8;
  HPALETTE16 palette;
  code **fn_ptr_1;

  uVar2 = ((u32)struct_param_1 >> 0x10);
  struct_2 = (astruct_769 *)struct_param_1;
  if ((((int)&struct_2->field8_0x8 + 0x2) | &struct_2->field8_0x8) != 0x0) {
    palette = 0x0;
    if (struct_2->field59_0x46 == 0x0) {
      uVar3 = ((u32)_PTR_LOOP_1050_4230 >> 0x10);
      uVar1 = ((int)_PTR_LOOP_1050_4230 + 0x10);
      unaff_CS = 0x1008;
      palette = palette_op_1008_4e08
                          ((HPALETTE16)&hdc_param_3,uVar1,
                           (astruct_13 *)CONCAT22(uVar1,((int)_PTR_LOOP_1050_4230 + 0xe)),
                           (HDC16 *)CONCAT22(0x1050,&hdc_param_3));
    }
    puStack8 = struct_2->field8_0x8;
    uVar4 = ((int)&struct_2->field8_0x8 + 0x2);
    if (((((int)&struct_2->field9_0xc + 0x2) | &struct_2->field9_0xc) != 0x0) &&
       ((param_2 & 0x1) != 0x0)) {
      puStack8 = struct_2->field9_0xc;
      uVar4 = ((int)&struct_2->field9_0xc + 0x2);
    }
    if ((struct_2->field10_0x10 != NULL) && ((param_2 & 0x4) != 0x0)) {
      puStack8 = struct_2->field10_0x10;
      uVar4 = ((int)&struct_2->field10_0x10 + 0x2);
    }
    fn_ptr_1 = (code **)((int)*puStack8 + 0x4);
    (**fn_ptr_1)(unaff_CS,(int)puStack8,uVar4,struct_2->field30_0x28,struct_2->field29_0x26,&hdc_param_3,
                 (int)0x1050);
    if (struct_2->field59_0x46 == 0x0) {
      palette_2 = SelectPalette16(0x0,palette,hdc_param_3);
      DeleteObject16(palette_2);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_c94a(u8 *param_1,astruct_37 *param_2,HDC16 param_3,u16 param_4)

{
  u16 uVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  u32 *puVar5;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;

  if (((int)param_2 + 0x48) != 0x0) {
    puVar5 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(param_4,0x3),in_stack_0000fe98,in_stack_0000ffbc,in_stack_0000ffc2,
                             in_stack_0000ffc6);
    uVar3 = ((u32)puVar5 >> 0x10);
    uVar2 = (u32)((int)param_2 + 0x42);
    uVar1 = ((int)uVar2 + 0x12);
    uVar4 = uVar1;
    pass1_1010_a5ca(uVar1,uVar3,puVar5,uVar3,uVar1);
    if (uVar4 == 0xffff) {
      ((int)param_2 + 0x3c) = 0xf9;
    }
    else if (uVar4 == 0x0) {
      ((int)param_2 + 0x3c) = 0x25;
      if ((uVar1 == 0x5b) || (uVar1 == 0x9)) {
        ((int)param_2 + 0x3c) = 0xfe;
      }
    }
    else {
      ((int)param_2 + 0x3c) = 0xfb;
    }
  }
  draw_text_1040_94fc(param_2,param_3);
  return;
}



StructD * pass1_1040_c9cc(StructD *param_1,u8 param_2)

{
  pass1_1040_c5ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_ca16(u8 *param_1,Struct57*param_2,u16 param_3)

{
  u16 in_register_0000000a;
  Struct57*paVar1;
  Struct57*iVar1;
  u16 unaff_BP;
  u16 uVar2;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1840));
  uVar2 = ((u32)param_2 >> 0x10);
  iVar1 = (Struct57*)param_2;
  (u32)&iVar1[0x1].field3_0x6 = PTR_LOOP_1050_5f0c;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  iVar1[0x1].field7_0xe = 0x0;
  iVar1[0x1].field8_0x10 = 0x0;
  param_2->field0_0x0 = 0xd07c;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3e),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field5_0xa = puVar3;
  iVar1[0x1].field6_0xc = ((u32)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_ca74(StructD *param_1)

{
  u16 uVar1;
  u16 in_stack_0000ffde;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xd07c;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  PTR_LOOP_1050_5f10 = NULL;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_caa6(u8 *param_1,u16 param_2,u32 param_3)

{
  u16 in_register_0000000a;
  Struct27 *paVar1;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  i16 iVar2;

  iVar2 = 0x0;
  paVar1 = (Struct27 *)
           mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_window_1040_b726(CONCAT22((int)param_3,param_2),(int)(param_3 >> 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_cace(u16 param_1,u32 param_2)

{
  u32 uVar1;
  bool bVar2;
  i16 iVar3;
  INT16 IVar4;
  u16 uVar5;
  u16 uVar6;
  bool bVar7;
  u16 uVar8;
  char local_208 [0x100];
  char local_108 [0x100];
  u16 UStack8;
  u16 uStack6;
  BOOL16 local_4;

  uVar6 = (param_2 >> 0x10);
  uVar5 = param_2;
  uStack6 = GetDlgItemInt16(0x0,&local_4,(INT16)0x1050,0x18e);
  UStack8 = GetDlgItemInt16(0x0,&local_4,(INT16)0x1050,0x191);
  if (uStack6 == 0x0) {
    return;
  }
  pass1_1018_50ea((u32)(uVar5 + 0x98),uStack6,(u32)(uVar5 + 0x94));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_208,(short)0x1050);
  uVar1 = (u32)(uVar5 + 0x94);
  uVar8 = ((u32)_u16_1050_14cc >> 0x10);
  if (*(i32 *)((int)uVar1 + 0x36) == 0x0) {
    load_string_1010_84e0(_u16_1050_14cc,uVar8,0x3ff,local_108,(short)0x1050);
    iVar3 = MessageBox16(0x34,(char *)CONCAT22(0x1050,local_208),(char *)CONCAT22(0x1050,local_108),
                         *(HWND16 *)(uVar5 + 0x8));
  }
  else {
    load_string_1010_84e0(_u16_1050_14cc,uVar8,0x3ff,local_108,(short)0x1050);
    iVar3 = MessageBox16(0x34,(char *)CONCAT22(0x1050,local_208),(char *)CONCAT22(0x1050,local_108),
                         *(HWND16 *)(uVar5 + 0x8));
  }
  bVar2 = iVar3 == 0x6;
  bVar7 = false;
  if ((!bVar2) && (uVar1 = (u32)(uVar5 + 0x94), ((int)uVar1 + 0x34) < 0x1)) {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_108,(short)0x1050);
    IVar4 = MessageBox16(0x34,(char *)CONCAT22(0x1050,local_208),(char *)CONCAT22(0x1050,local_108),
                         *(HWND16 *)(uVar5 + 0x8));
    bVar7 = IVar4 == 0x6;
    bVar2 = false;
  }
  if (bVar2) {
    PTR_LOOP_1050_5f16 = (u32)(uVar5 + 0x94);
    iVar3 = 0x26;
  }
  else {
    if (!bVar7) {
      return;
    }
    _u16_1050_5a68 = (u32)(uVar5 + 0x94);
    iVar3 = 0x27;
  }
  pass1_1038_af40(uVar5,param_1,_PTR_LOOP_1050_5b7c,(uVar5 + 0x8),iVar3);
  return;
}



LRESULT pass1_1040_cc66(u32 param_1)

{
  code **ppcVar1;
  u16 DX_REG;
  LRESULT LVar2;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x98) + 0x10);
  (**ppcVar1)();
  LVar2 = send_dlg_msg_1040_cf1c(DX_REG,(astruct_903 *)param_1);
  return LVar2;
}



void pass1_1040_cc8c(u8 *param_1,astruct_903 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  if (param_5 == 0xeb) {
    send_dlg_msg_1040_cf1c(param_1,param_2);
  }
  else {
    // just 0x1756
    if (param_5 == (int)s_vrpal_bmp_1050_183a + 0x7U) {
      msg_box_op_1040_cce4(0x0,param_1,param_2);
    }
    else {
      if (param_5 != (int)s_vrpal_bmp_1050_183a + 0x8U) {
        pass1_1040_b54a(param_1,param_2,param_3,_param_4);
        return;
      }
      if (param_4 == 0x1) {
        send_dlg_item_1040_ce76(param_2);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_op_1040_cce4(char *param_1,u16 param_2,astruct_903 *param_3)

{
  short in_buf_len_5;
  u16 in_register_0000000a;
  Struct57*paVar1;
  u32 uStack522;
  char local_206 [0x102];
  char local_104 [0x102];
  u16 uVar2;
  u16 uVar3;

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)0x1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



u16 pass1_1040_cdac(param_1: u32,u16 param_2,u16 param_3,param_4: i16)

{
  i16 *piVar1;
  i16 iVar2;
  bool bVar3;
  i16 iVar4;
  u16 uVar5;

  bVar3 = false;
  iVar4 = (int)param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_4 == 0x0) {
    iVar2 = (iVar4 + 0x9e);
    piVar1 = (i16 *)(iVar4 + 0x9c);
    if (*piVar1 == iVar2 || *piVar1 < iVar2) goto LAB_1040_cdef;
    piVar1 = (i16 *)(iVar4 + 0x9e);
    *piVar1 = *piVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_cdef;
    if ((iVar4 + 0x9e) < 0x1) goto LAB_1040_cdef;
    piVar1 = (i16 *)(iVar4 + 0x9e);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar3 = true;
LAB_1040_cdef:
  if (bVar3) {
    SetDlgItemInt16(0x0,(iVar4 + 0x9e),0x18e,*(HWND16 *)(iVar4 + 0x6));
  }
  return 0x0;
}



void send_dlg_item_msg_1040_ce12(u16 param_1,u16 param_2,param_3: u32,u16 param_4)

{
  u32 uVar1;
  let mut lVar2: i32;
  WORD local_10a [0x80];
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),param_3);
  while( true ) {
    lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar2 == 0x0) break;
    uVar1 = (u32)((int)lVar2 + 0x4);
    wsprintf16(local_10a,(char *)0x5f121050,(char *)CONCAT22((int)uVar1,0x1050),(int)((u32)uVar1 >> 0x10));
    SendDlgItemMessage16(CONCAT22(0x1050,local_10a),0x0,0x401,param_4,*(HWND16 *)(param_1 + 0x6));
  }
  return;
}



void send_dlg_item_1040_ce76(astruct_903 *param_1)

{
  i16 iVar1;
  u16 uVar2;
  LRESULT LVar3;
  u32 uVar4;
  u8 local_106 [0x100];
  WPARAM16 WStack6;
  i16 iStack4;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  LVar3 = SendDlgItemMessage16(0x0,0x0,0x409,(int)s_vrpal_bmp_1050_183a + 0x8,*(HWND16 *)(iVar1 + 0x6));
  WStack6 = (WPARAM16)LVar3;
  iStack4 = (int)WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16
              (CONCAT22(0x1050,local_106),WStack6,0x40a,(int)s_vrpal_bmp_1050_183a + 0x8,*(HWND16 *)(iVar1 + 0x6));
    uVar4 = pass1_1018_5206((u32)(iVar1 + 0x98),(char *)CONCAT22(0x1050,local_106));
    if (uVar4 != 0x0) {
      (iVar1 + 0x9c) = ((int)uVar4 + 0x8);
      (iVar1 + 0x9e) = 0x0;
      SetDlgItemInt16(0x0,0x0,0x18e,*(HWND16 *)(iVar1 + 0x6));
      SetDlgItemInt16(0x0,(iVar1 + 0x9c),0x191,*(HWND16 *)(iVar1 + 0x6));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_msg_1040_cf1c(u16 param_1,astruct_903 *param_2)

{
  u8 *puVar1;
  HWND16 hwnd;
  u16 in_register_0000000a;
  astruct_928 *uVar1;
  u16 uVar2;
  u32 *puVar3;
  u32 uVar4;
  LRESULT LVar5;
  u16 in_stack_0000f99c;
  u16 in_stack_0000fac0;
  u16 in_stack_0000fac6;
  u16 in_stack_0000faca;
  BOOL16 enable;
  u16 uVar6;
  u16 in_stack_0000faf4;
  char local_106 [0x100];

  puVar3 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000faf4,0x3),in_stack_0000f99c,in_stack_0000fac0,
                           in_stack_0000fac6,in_stack_0000faca);
  puVar1 = (u8 *)((u32)puVar3 >> 0x10);
  uVar2 = ((u32)param_2 >> 0x10);
  uVar1 = (astruct_928 *)param_2;
  pass1_1010_c3c2(puVar1,puVar3,puVar1,CONCAT22(0x1050,local_106),uVar1->field147_0x94);
  SendDlgItemMessage16(CONCAT22(0x1050,local_106),0x0,0xc,(int)s_dibtext_bmp_1050_1844 + 0x2,uVar1->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,(int)s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,(int)s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  uVar6 = (int)s_vrpal_bmp_1050_183a + 0x8;
  uVar4 = pass1_1018_526a(uVar1->field148_0x98,uVar1->field147_0x94);
  send_dlg_item_msg_1040_ce12(uVar1,uVar2,uVar4,uVar6);
  LVar5 = SendDlgItemMessage16(0x0,0x0,0x40c,(int)s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  if (((int)((u32)LVar5 >> 0x10) < 0x1) && ((LVar5 < 0x0 || ((int)LVar5 == 0x0)))) {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,&stack0xfaf4,(short)0x1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&stack0xfaf4),0x0,0x401,(int)s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
    hwnd = GetDlgItem16(0x1,uVar1->field6_0x6);
    enable = 0x0;
  }
  else {
    hwnd = GetDlgItem16(0x1,uVar1->field6_0x6);
    enable = 0x1;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,(int)s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  return LVar5;
}



StructD * pass1_1040_d056(StructD *param_1,u8 param_2)

{
  pass1_1040_ca74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_d0f8(Struct57*param_1,u16 param_2,u16 param_3,StructD *param_4,u16 param_5,
                    u16 param_6,u16 param_7,u16 param_8,u16 param_9,u16 param_10,
                    u16 param_11,u16 param_12,u16 param_13)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  Struct57*paVar4;
  Struct57*iVar5;
  u16 uVar7;
  u32 *puVar8;
  u32 uVar9;
  u32 uVar5;
  Struct57*paVar6;

  struct_1040_b082(param_1,CONCAT22(param_2,0x1845));
  uVar7 = ((u32)param_1 >> 0x10);
  iVar5 = (Struct57*)param_1;
  (u32)&iVar5[0x1].field3_0x6 = 0x0;
  (u32)&iVar5[0x1].field5_0xa = PTR_LOOP_1050_5f16;
  (u32)&iVar5[0x1].field7_0xe = 0x0;
  iVar5[0x1].field9_0x12 = 0x0;
  param_1->field0_0x0 = 0xd8c4;
  iVar5->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar8 = mixed_1010_20ba((Struct57*)param_4,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x47),param_6,param_10,
                           param_11,param_12);
  uVar5 = (u32)param_4 & 0xffff0000;
  iVar5[0x1].field3_0x6 = puVar8;
  uVar2 = ((u32)puVar8 >> 0x10);
  iVar5[0x1].field4_0x8 = uVar2;
  uVar9 = pass1_1018_5732(puVar8,uVar2,iVar5[0x1].field3_0x6,uVar2,(u32)&iVar5[0x1].field5_0xa);
  paVar4 = (Struct57*)(uVar5 & 0xffff0000 | uVar9 >> 0x10);
  iVar5[0x1].field7_0xe = uVar9;
  uVar1 = (uVar9 >> 0x10);
  iVar5[0x1].field8_0x10 = uVar1;
  uVar1 |= iVar5[0x1].field7_0xe;
  if (uVar1 == 0x0) {
    mem_op_1000_179c(0xc,paVar4);
    uVar3 = paVar4 | uVar1;
    paVar6 = (Struct57*)((u32)paVar4 & 0xffff0000 | (u32)uVar3);
    if (uVar3 == 0x0) {
      (u32)&iVar5[0x1].field7_0xe = 0x0;
    }
    else {
      pass1_1010_8ef2(paVar6,(astruct_170 *)CONCAT22(paVar4,uVar1),param_13,param_5,param_7,param_8,param_9);
      iVar5[0x1].field7_0xe = uVar1;
      iVar5[0x1].field8_0x10 = paVar6;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_d1bc(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  StructD *iVar4;
  u16 uVar4;
  u16 in_stack_0000ffd4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xd8c4;
  iVar4->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar4->field_0x6);
  puVar1 = &iVar4->field_0x9c;
  uVar2 = &iVar4->field_0x9e;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)((int)&u16_1050_1038,puVar1,uVar2,0x1);
  }
  unk_draw_op_1040_b0f8(in_stack_0000ffd4,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void send_dlg_item_msg_1040_d20c(u16 param_1,u16 param_2,astruct_929 *param_3,i32 param_4)

{
  u8 *puVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  Struct57*paVar3;
  i16 iVar4;
  u16 uVar5;
  u32 *puVar6;
  u16 in_stack_0000fd96;
  u16 in_stack_0000feba;
  u16 in_stack_0000fec0;
  u16 in_stack_0000fec4;
  u8 *puVar7;
  u16 uVar8;
  u8 local_106 [0x100];
  u16 uStack6;
  u16 uStack4;

  paVar3 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  if (param_4 == 0x0) {
    enable_win_1040_d60e(param_3);
    return;
  }
  uVar5 = ((u32)param_3 >> 0x10);
  iVar4 = (int)param_3;
  if ((iVar4 + 0xa0) != 0x0) {
    pass1_1010_9210((u32)(iVar4 + 0x9c));
    enable_win_1040_d60e(param_3);
    pass1_1030_8344(_u16_1050_5748,param_4);
    uVar2 = SUB42(paVar3,0x0);
    puVar7 = local_106;
    uVar8 = SUB42(0x1050,0x0);
    uStack6 = param_1;
    uStack4 = uVar2;
    puVar6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(puVar7,0x3),in_stack_0000fd96,
                             in_stack_0000feba,in_stack_0000fec0,in_stack_0000fec4);
    puVar1 = (u8 *)((u32)puVar6 >> 0x10);
    pass1_1010_c3c2(puVar1,puVar6,puVar1,CONCAT22(uVar8,puVar7),CONCAT22(uVar2,param_1));
    SendDlgItemMessage16
              (CONCAT22(0x1050,local_106),0x0,0x401,(int)s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  }
  return;
}



void pass1_1040_d29c(u32 param_1)

{
  u16 in_DX;

  send_ldg_item_msg_1040_d79c(in_DX,(astruct_903 *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_d2ac(u16 param_1,astruct_903 *pstruct_param_2,u16 param_3,u32 param_4)

{
  u16 in_register_0000000a;
  LRESULT LVar1;

  if (param_4 == (int)s_dibtext_bmp_1050_1844 + 0x4U) {
    LVar1 = SendDlgItemMessage16
                      (0x0,0x0,0x405,(int)s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(pstruct_param_2 + 0x6));
    struct_1010_9172(*(astruct_249 **)(pstruct_param_2 + 0x9c),
                     CONCAT22(in_register_0000000a,(int)((u32)LVar1 >> 0x10)));
  }
  else if ((int)s_dibtext_bmp_1050_1844 + 0x4U < param_4) {
    if (param_4 == (int)s_dibtext_bmp_1050_1844 + 0x5U) {
      LVar1 = SendDlgItemMessage16
                        (0x0,0x0,0x40c,(int)s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(pstruct_param_2 + 0x6));
      if (((int)LVar1 != -0x1) || ((int)((u32)LVar1 >> 0x10) != -0x1)) {
        SendDlgItemMessage16
                  (0x0,(int)LVar1 - 0x1,0x403,(int)s_dibtext_bmp_1050_1844 + 0x3,
                   *(HWND16 *)(pstruct_param_2 + 0x6));
        pass1_1010_91cc((u32)(pstruct_param_2 + 0x9c));
      }
    }
    else if (param_4 == (int)s_dibtext_bmp_1050_1844 + 0x6U) {
      enable_win_1040_d6be(pstruct_param_2);
      pass1_1018_57d2((u32)(pstruct_param_2 + 0x94),(u32)pstruct_param_2);
      PostMessage16(0x0,0x203,0x111,HWND16_1050_0396);
    }
    else {
      if (param_4 != (int)s_dibtext_bmp_1050_1844 + 0x7U) goto LAB_1040_d3b3;
      _u16_1050_5a68 = (u32)(pstruct_param_2 + 0x98);
      pass1_1038_af40(pstruct_param_2,param_1,_PTR_LOOP_1050_5b7c,(pstruct_param_2 + 0x6),
                      0x27);
    }
  }
  else if (param_4 == 0xeb) {
    send_ldg_item_msg_1040_d79c(param_1,pstruct_param_2);
  }
  else {
    if (param_4 != (int)s_vrpal_bmp_1050_183a + 0x7U) {
LAB_1040_d3b3:
      pass1_1040_b54a((u8 *)param_1,pstruct_param_2,param_3,param_4);
      return;
    }
    msg_box_op_1040_d3d0(0x0,param_1,pstruct_param_2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_op_1040_d3d0(char *param_1,u16 param_2,u32 param_3)

{
  short in_buf_len_5;
  u16 in_register_0000000a;
  Struct57*paVar1;
  u16 uVar2;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (Struct57*)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)0x1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  uVar2 = ((u32)param_3 >> 0x10);
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



void enable_win_1040_d60e(astruct_929 *param_1)

{
  HWND16 HVar1;
  astruct_929 *iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_929 *)param_1;
  HVar1 = GetDlgItem16(0x1,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(0x2,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x4,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x5,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x6,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  iVar2->field159_0xa0 = 0x0;
  return;
}



void enable_win_1040_d6be(astruct_903 *param_1)

{
  HWND16 HVar1;
  astruct_903 *iVar2;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_903 *)param_1;
  HVar1 = GetDlgItem16(0x1,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x2,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x4,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x5,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x6,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16((int)s_dibtext_bmp_1050_1844 + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  &iVar2[0x1].field_0x8 = 0x1;
  return;
}



void pass1_1040_d76e(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = (u32)(iVar2 + 0x94);
  pass1_1018_5742(uVar1,((u32)uVar1 >> 0x10),(iVar2 + 0x9c),*(astruct_299 **)(iVar2 + 0x98)
                 );
  (u32)(iVar2 + 0x9c) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void send_ldg_item_msg_1040_d79c(u16 param_1,astruct_903 *param_2)

{
  let mut lVar1: i32;
  u8 *puVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  i16 iVar4;
  u16 uVar5;
  u32 uVar6;
  u32 uVar7;
  u16 in_stack_0000fd9a;
  u16 in_stack_0000febe;
  u16 in_stack_0000fec4;
  u16 in_stack_0000fec8;
  u16 in_stack_0000fef2;
  u16 uStack268;
  u32 uStack266;
  char local_106 [0x100];
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((Struct57*)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fef2,0x3),in_stack_0000fd9a,in_stack_0000febe,
                             in_stack_0000fec4,in_stack_0000fec8);
  puVar2 = (u8 *)((u32)puStack6 >> 0x10);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (int)param_2;
  pass1_1010_c3c2(puVar2,puStack6,puVar2,CONCAT22(0x1050,local_106),(u32)(iVar4 + 0x98));
  SendDlgItemMessage16(CONCAT22(0x1050,local_106),0x0,0xc,(int)s_dibtext_bmp_1050_1844 + 0x2,*(HWND16 *)(iVar4 + 0x6));
  SendDlgItemMessage16(0x0,0x0,0xb,(int)s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  uVar6 = SendDlgItemMessage16(0x0,0x0,0x405,(int)s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  uVar7 = uVar6 >> 0x10;
  uVar3 = uVar6;
  pass1_1010_9044((u32)(iVar4 + 0x9c));
  uStack266 = CONCAT22((int)uVar7,uVar3);
  uVar3 = 0x0;
  uStack268 = 0x0;
  while (CONCAT22(uStack268,uVar3) < uStack266) {
    pass1_1010_9130(local_106,uVar7,(u32)(iVar4 + 0x9c),(u8 *)CONCAT22(0x1050,local_106));
    if (local_106[0] != '\0') {
      uVar7 = SendDlgItemMessage16
                        (CONCAT22(0x1050,local_106),0x0,0x401,(int)s_dibtext_bmp_1050_1844 + 0x3,
                         *(HWND16 *)(iVar4 + 0x6));
      uVar7 >>= 0x10;
    }
    lVar1 = CONCAT22(uStack268,uVar3) + 0x1;
    uVar3 = lVar1;
    uStack268 = ((u32)lVar1 >> 0x10);
  }
  SendDlgItemMessage16(0x0,0x1,0xb,(int)s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  return;
}



StructD * pass1_1040_d89e(StructD *param_1,u8 param_2)

{
  pass1_1040_d1bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
