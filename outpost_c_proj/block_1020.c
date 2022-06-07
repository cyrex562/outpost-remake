//
// Created by cyrex on 2022-05-22.
//

#include "block_1020.h"


void unk_draw_op_1020_0000(astruct_840 *param_1)

{
  i16 *piVar1;
  code **ppcVar2;
  astruct_840 *iVar4;
  u16 uVar5;
  u16 uVar4;
  u8 local_c4 [0x6];
  i16 *local_be;
  i16 *piStack184;
  u16 uStack182;
  i16 local_b4;
  i16 iStack178;
  i16 aiStack176 [0x3c];
  astruct_841 *iStack56;
  i16 iStack48;
  astruct_76 *paStack46;
  i16 local_2a;
  i16 local_28;
  u32 *puStack38;
  u8 local_22 [0x20];
  u32 uVar3;
  u32 uVar2;
  astruct_841 *iVar3;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_840 *)param_1;
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar4->field4_0x4);
  uVar3 = iVar4->field19_0x14;
  puStack38 = (u32*)((int)uVar3 + 0xa);
  pass1_1008_3e94((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x18)),(u16 *)CONCAT22(0x1050,&local_2a),
                  (char *)CONCAT22(0x1050,&local_28));
  uVar4 = 0x1008;
  pass1_1008_4480((astruct_76 *)puStack38,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x18)),
                  iVar4->field32_0x24);
  paStack46 = NULL;
  for (iStack48 = 0x0; iStack48 < 0x6; iStack48 += 0x1) {
    uVar2 = iVar4->field19_0x14;
    uVar4 = 0x1010;
    pass1_1010_2b78(uVar2,(uVar2 >> 0x10),iStack48,CONCAT22(0x1050,&local_b4));
    if (local_b4 == 0x0) {
      for (iStack56 = NULL; iVar3 = iStack56, (int)iStack56 <= iStack178; iStack56 = iStack56 + 0x1) {
        piVar1 = aiStack176 + (int)iStack56 * 0x3;
        uStack182 = &DAT_1050_1050;
        piStack184 = piVar1;
        if (aiStack176[(int)iStack56 * 0x3 + 0x2] != 0x0) {
          paStack46 = pass1_1010_2b98(iVar4->field19_0x14,aiStack176[(int)iStack56 * 0x3 + 0x2]);
          pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_be),0x0,aiStack176[(int)iVar3 * 0x3 + 0x1] + local_2a,
                          *piVar1 + local_28);
          uVar4 = 0x1008;
          pass1_1008_4480((astruct_76 *)puStack38,(u16 *)CONCAT22(0x1050,&local_be),paStack46);
        }
      }
    }
    else {
      local_be = (i16 *)CONCAT22(0x1050,aiStack176 + iStack178 * 0x3);
      if (aiStack176[iStack178 * 0x3 + 0x2] != 0x0) {
        paStack46 = pass1_1010_2b98(iVar4->field19_0x14,aiStack176[iStack178 * 0x3 + 0x2]);
        pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_c4),0x0,((int)local_be + 0x2) + local_2a,
                        *local_be + local_28);
        uVar4 = 0x1008;
        pass1_1008_4480((astruct_76 *)puStack38,(u16 *)CONCAT22(0x1050,local_c4),paStack46);
      }
    }
  }
  ppcVar2 = (code **)((int)*puStack38 + 0x4);
  (**ppcVar2)(uVar4,(int)puStack38,(int)((u32)puStack38 >> 0x10),0x0,0x0,&iVar4->field_0xa,uVar5);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar4->field4_0x4);
  return;
}



StructD * pass1_1020_01a6(StructD *param_1,u8 param_2)

{
  pass1_1018_ed98(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1020_01d8(astruct_20 *param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6,
                    u32 param_7)

{
  u16 in_stack_0000ffd6;

  unk_draw_op_1008_61b2(in_stack_0000ffd6,(StructA *)param_1,param_2,param_6,param_7);
  (u32)((int)param_1 + 0xe2) = 0x0;
  (u32)((int)param_1 + 0xe6) = 0x0;
  ((int)param_1 + 0xea) = param_5;
  ((int)param_1 + 0xec) = param_4;
  ((int)param_1 + 0xee) = param_3;
  param_1->offset_0x0 = 0x45a;
  ((int)param_1 + 0x2) = 0x1020;
  return;
}



void pass1_1020_022c(astruct_29 *struct_param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_29 *iVar4;
  u16 uVar4;

  uVar4 = ((u32)struct_param_1 >> 0x10);
  iVar4 = (astruct_29 *)struct_param_1;
  struct_param_1->field0_0x0 = 0x45a;
  iVar4->field1_0x2 = 0x1020;
  puVar1 = (u32*)&iVar4->field228_0xe6;
  uVar2 = ((int)&iVar4->field228_0xe6 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1008_57c4((StructD *)((u32)struct_param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0xd2)));
  struct_param_1->field0_0x0 = 0x380a;
  iVar4->field1_0x2 = 0x1008;
  struct_param_1->field0_0x0 = 0x389a;
  iVar4->field1_0x2 = 0x1008;
  return;
}



void pass1_1020_028c(u16 param_1,u16 param_2,i16 param_3)

{
  u16 in_DX;

  pass1_1010_3c9e(*(i32 *)(param_1 + 0xe2));
  pass1_1008_68c6(in_DX,param_1,param_2,param_3);
  return;
}



void pass1_1020_02ae(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_3cd0(*(i32 *)(iVar4 + 0xe2));
  destroy_win_1008_628e(param_1);
  puVar1 = (u32 *)(iVar4 + 0xe6);
  uVar2 = (iVar4 + 0xe8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (u32)(iVar4 + 0xe6) = 0x0;
  pass1_1010_1dda((u32)(iVar4 + 0xe2));
  (u32)(iVar4 + 0xe2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_1020_0316(u16 param_1,StructA *param_2)

{
  u32 uVar1;
  astruct_666 *paVar2;
  u8 *puVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  StructA *iVar1;
  u16 uVar5;
  u32 *puVar6;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000fff2;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  create_window_ex_1008_9760(param_2);
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x1),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar6 >> 0x10);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar1 = (StructA *)param_2;
  iVar1[0x1].field11_0x16 = (i16)puVar6;
  iVar1[0x1].field12_0x18 = (i16)((u32)puVar6 >> 0x10);
  uVar1 = (u32)&iVar1[0x1].field11_0x16;
  ((int)uVar1 + 0x16) = iVar1[0x1].field15_0x1e;
  paVar2 = iVar1[0x1].field18_0x22;
  uVar1 = (u32)&iVar1[0x1].field11_0x16;
  *(astruct_666 **)((int)uVar1 + 0x12) = paVar2;
  struct_1010_3c52(paVar4,(u32)&iVar1[0x1].field11_0x16,&iVar1[0x1].field_0x20);
  mem_op_1000_179c(0x12,paVar4);
  puVar3 = (u8 *)(paVar4 | paVar2);
  if (puVar3 != NULL) {
    pass1_1020_04f6(puVar3,(astruct_662 *)CONCAT22(paVar4,paVar2),iVar1->field4_0x8);
    iVar1[0x1].field13_0x1a = paVar2;
    iVar1[0x1].field14_0x1c = puVar3;
    return;
  }
  (u32)&iVar1[0x1].field13_0x1a = 0x0;
  return;
}



void post_msg_1020_03b2(u32 param_1)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_1 + 0xe2);
  PostMessage16(0x0,*(WPARAM16 *)((int)uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}



void post_msg_1020_03d6(u32 param_1)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_1 + 0xe2);
  PostMessage16(0x0,*(WPARAM16 *)((int)uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}



void post_msg_1020_03fa(u32 param_1)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_1 + 0xe2);
  PostMessage16(0x0,*(WPARAM16 *)((int)uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}



void draw_op_1020_041e(u32 param_1)

{
  fill_rect_1020_065e(*(astruct_754 **)((int)param_1 + 0xe6));
  return;
}



astruct_29 * pass1_1020_0434(astruct_29 *param_1,u8 param_2)

{
  pass1_1020_022c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_04f6(u8 *param_1,astruct_662 *param_2,u16 param_3)

{
  code **ppcVar1;
  i16 iVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  astruct_662 *iVar4;
  u16 uVar5;
  u32 *puVar6;
  u16 in_stack_0000fe80;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ffa4;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  u16 uVar7;
  u32 in_stack_0000ffe0;
  u8 **ppuVar8;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (astruct_662 *)param_2;
  param_2 = 0x389a;
  iVar4->field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar4->field2_0x2 = 0x1008;
  iVar4->field3_0x4 = param_3;
  param_2 = 0x3ab0;
  iVar4->field2_0x2 = 0x1008;
  iVar4->field4_0x6 = NULL;
  iVar4->field5_0xa = 0x0;
  iVar4->field6_0xc = 0x0;
  iVar4->field7_0xe = 0x0;
  iVar4->field8_0x10 = 0x0;
  param_2 = 0x75a;
  iVar4->field2_0x2 = 0x1020;
  ppuVar8 = (u8 **)CONCAT22((int)((u32)in_stack_0000ffe0 >> 0x10),0x1);
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,ppuVar8,in_stack_0000fe8a,in_stack_0000ffae,in_stack_0000ffb4,
                           in_stack_0000ffb8);
  paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar6 >> 0x10);
  &iVar4->field4_0x6 = (int)puVar6;
  uVar3 = ((u32)puVar6 >> 0x10);
  ((int)&iVar4->field4_0x6 + 0x2) = uVar3;
  uVar7 = &iVar4->field4_0x6;
  ppcVar1 = (code **)((int)*iVar4->field4_0x6 + 0x4);
  (**ppcVar1)(0x1010,uVar7,uVar3,0x0,param_2,(int)((u32)ppuVar8 >> 0x10));
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(uVar7,0x48),in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  iVar2 = (int)puVar6;
  iVar4->field5_0xa = (iVar2 + 0xa);
  iVar4->field6_0xc = (iVar2 + 0xc);
  pass1_1008_3e94((u16 *)((u32)puVar6 & 0xffff0000 | (u32)(iVar2 + 0xe)),
                  (u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4->field8_0x10)),
                  (char *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4->field7_0xe)));
  return;
}



void pass1_1020_05d6(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x75a;
  iVar1->address_offset_field_0x2 = 0x1020;
  if (*(i32 *)&iVar1->field_0x6 != 0x0) {
    pass1_1010_1ea6((u32)&iVar1->field_0x6,(StructD *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  }
  param_1->address_offset_field_0x0 = 0x3ab0;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



void post_win_msg_1020_061c(u32 param_1,i16 param_2)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (u32)((int)param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  uVar1 = (u32)((int)param_1 + 0x6);
  PostMessage16(0x0,*(WPARAM16 *)((int)uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}



// WARNING: Unable to use type for symbol uVar2

void fill_rect_1020_065e(astruct_754 *astruct754_param_1)

{
  u32 uVar1;
  astruct_754 *struct754_var1;
  u16 uVar4;
  RECT16 rect_1;
  i16 iStack48;
  i16 iStack46;
  HBRUSH16 brush_handle_1;
  HDC16 *palette_handle_var42;
  u32 *puStack40;
  HDC16 hdc_var_24;
  u8 paintstruct_22 [0x20];
  u16 uVar3;
  u32 uVar2;
  code **fn_ptr_1;

  uVar4 = ((u32)astruct754_param_1 >> 0x10);
  struct754_var1 = (astruct_754 *)astruct754_param_1;
  hdc_var_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),*(HWND16 *)&struct754_var1->field_0x4);
  if (0x280 < struct754_var1->field7_0xa) {
    brush_handle_1 = CreateSolidBrush16(0x210070b);
    rect_1.x = 0x0;
    rect_1.y = 0x0;
    iStack48 = struct754_var1->field7_0xa + -0x1;
    iStack46 = struct754_var1->field8_0xc + -0x1;
    FillRect16(brush_handle_1,&rect_1,(HDC16)&DAT_1050_1050);
    DeleteObject16(brush_handle_1);
  }
  uVar2 = struct754_var1->field6_0x6;
  puStack40 = (u32*)((int)uVar2 + 0xe);
  palette_handle_var42 = &hdc_var_24;
  uVar3 = puStack40;
  uVar1 = *puStack40;
  fn_ptr_1 = (code **)((int)uVar1 + 0x8);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,uVar3,(int)((u32)puStack40 >> 0x10),palette_handle_var42,(int)&DAT_1050_1050
              );
  fn_ptr_1 = (code **)((int)uVar1 + 0x4);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)puStack40,(int)((u32)puStack40 >> 0x10),struct754_var1->field10_0x10,
               struct754_var1->field9_0xe,&hdc_var_24,(int)&DAT_1050_1050,uVar3);
  palette_handle_var42 = (HDC16 *)SelectPalette16(0x0,(HPALETTE16)palette_handle_var42,hdc_var_24);
  DeleteObject16((HGDIOBJ16)palette_handle_var42);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),*(HWND16 *)&struct754_var1->field_0x4);
  return;
}



StructD * pass1_1020_0734(StructD *param_1,u8 param_2)

{
  pass1_1020_05d6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1020_0762(astruct_20 *param_1,u32 param_2,u32 *param_3,u16 param_4,u32 param_5,u32 param_6)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;

  pass1_1020_01d8(param_1,param_2,(param_2 >> 0x10),param_4,param_5,(param_5 >> 0x10),
                  param_6);
  uVar1 = (astruct_20 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1[0x1].field6_0xe = 0x0;
  iVar1[0x1].field7_0x10 = (astruct_20 *)*param_3;
  param_1->offset_0x0 = 0x81a;
  iVar1->base_0x2 = 0x1020;
  return;
}



void pass1_1020_07aa(u16 param_1,astruct_19 *param_2)

{
  astruct_19 *iVar1;
  astruct_19 *uVar2;
  char local_16 [0x14];

  draw_op_1020_041e((u32)param_2);
  uVar2 = (astruct_19 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_19 *)param_2;
  if (iVar1[0x2].field24_0x30 == 0x0) {
    iVar1[0x2].field24_0x30 = 0x1;
    pass1_1008_5bdc((char *)CONCAT22(0x1050,local_16));
    win_1008_5c9e(local_16,param_1,CONCAT22(0x1050,local_16),
                  (u32 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar1[0x2].field25_0x32)));
    pass1_1008_5c34((char *)CONCAT22(0x1050,local_16));
  }
  return;
}



astruct_29 * pass1_1020_07f4(astruct_29 *param_1,u8 param_2)

{
  pass1_1020_022c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_08b6(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  astruct_20 *iVar1;
  u16 uVar1;
  astruct_20 *paVar2;
  u16 in_stack_0000ffd6;

  paVar2 = unk_draw_op_1008_61b2(in_stack_0000ffd6,(StructA *)param_1,0x1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  &iVar1[0x1].field2_0x4 = 0x0;
  (u32)((int)&iVar1[0x1].field2_0x4 + 0x2) = 0x0;
  param_1->offset_0x0 = 0xb0e;
  iVar1->base_0x2 = 0x1020;
  win_1008_5c5c(0x0,((u32)paVar2 >> 0x10),_u16_1050_02a0,0x1d4);
  return;
}



void send_win_msg_1020_08fe(astruct_63 *param_1)

{
  HWND16 hwnd;
  i32 lVar1;
  BOOL16 BVar2;
  astruct_63 *iVar2;
  astruct_63 *uVar2;

  uVar2 = (astruct_63 *)((u32)param_1 >> 0x10);
  iVar2 = (astruct_63 *)param_1;
  param_1->field0_0x0 = 0xb0e;
  iVar2->field1_0x2 = 0x1020;
  if (iVar2->field229_0xe8 != 0x0) {
    lVar1 = iVar2->field229_0xe8;
    hwnd = *(HWND16 *)((int)lVar1 + 0x6);
    BVar2 = IsWindow16(hwnd);
    if (BVar2 != 0x0) {
      SendMessage16(0x0,0x1,0x111,hwnd);
    }
    iVar2->field229_0xe8 = 0x0;
  }
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field208_0xd2)));
  param_1->field0_0x0 = 0x380a;
  iVar2->field1_0x2 = 0x1008;
  param_1->field0_0x0 = 0x389a;
  iVar2->field1_0x2 = 0x1008;
  return;
}



void send_msg_1020_097e(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0xea) | (iVar2 + 0xe8)) != 0x0) {
    uVar1 = (u32)(iVar2 + 0xe8);
    SendMessage16(0x0,0x1,0x111,*(HWND16 *)((int)uVar1 + 0x6));
    (u32)(iVar2 + 0xe8) = 0x0;
  }
  return;
}



void win_1020_09ba(u16 param_1,u16 param_2,StructA *param_3)

{
  u8 *puVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  StructA *iVar1;
  u16 uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  create_window_ex_1008_9760(param_3);
  mem_op_1000_179c(0xe,paVar2);
  puVar1 = (u8 *)(paVar2 | param_1);
  iVar1 = (StructA *)param_3;
  uVar3 = ((u32)param_3 >> 0x10);
  if (puVar1 != NULL) {
    struct_1020_0baa(puVar1,(u16 *)CONCAT22(paVar2,param_1),iVar1->field4_0x8);
    iVar1[0x1].field11_0x16 = param_1;
    iVar1[0x1].field12_0x18 = (i16)puVar1;
    return;
  }
  (u32)&iVar1[0x1].field11_0x16 = 0x0;
  return;
}



void pass1_1020_0a0c(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  destroy_win_1008_628e(param_1);
  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (u32)(iVar4 + 0xe2) = 0x0;
  (iVar4 + 0xe6) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_0a52(u16 param_1,u32 param_2)

{
  u16 uVar1;
  u16 uVar2;
  u32 uVar3;

  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  unk_draw_op_1020_0c3e(*(astruct_771 **)(uVar1 + 0xe2));
  if ((uVar1 + 0xe6) == 0x0) {
    (uVar1 + 0xe6) = 0x1;
    ((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
    uVar3 = pass1_1038_af40(uVar1,param_1,_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x6);
    (uVar1 + 0xe8) = (int)uVar3;
    (uVar1 + 0xea) = (int)((u32)uVar3 >> 0x10);
  }
  return;
}



void pass1_1020_0aa6(u32 param_1)

{
  win_ui_palette_op_1020_0cd2(*(astruct_775 **)((int)param_1 + 0xe2));
  return;
}



void pass1_1020_0abc(u32 param_1)

{
  code **ppcVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  if (((int)param_1 + 0xe6) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0xe8) + 0x10);
    (**ppcVar1)();
  }
  return;
}



astruct_63 * pass1_1020_0ae8(astruct_63 *param_1,u8 param_2)

{
  send_win_msg_1020_08fe(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1020_0baa(u8 *param_1,u16 *param_2,u16 param_3)

{
  u16 in_register_0000000a;
  astruct_57 *paVar1;
  astruct_276 *iVar2;
  u16 uVar2;
  u32 *puVar3;
  u16 in_stack_0000fe82;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  u16 *puVar4;
  u16 *puVar5;
  u16 uVar6;
  u16 in_stack_0000ffe2;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar2 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_276 *)param_2;
  *param_2 = 0x389a;
  iVar2->field2_0x2 = 0x1008;
  *param_2 = 0x3aa8;
  iVar2->field2_0x2 = 0x1008;
  iVar2->field3_0x4 = param_3;
  *param_2 = 0x3ab0;
  iVar2->field2_0x2 = 0x1008;
  (u32)&iVar2->field4_0x6 = 0x0;
  iVar2->field6_0xa = 0x0;
  iVar2->field7_0xc = 0x0;
  *param_2 = 0xdbc;
  iVar2->field2_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe2,0x7),in_stack_0000fe8a,
                           in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
  iVar2->field4_0x6 = (int)puVar3;
  iVar2->field5_0x8 = (u8 *)((u32)puVar3 >> 0x10);
  puVar5 = &iVar2->field6_0xa;
  puVar4 = &iVar2->field7_0xc;
  uVar6 = uVar2;
  puVar3 = mixed_1010_20ba((astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar3 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22(puVar4,0x48),in_stack_0000fe82,in_stack_0000ffa6,in_stack_0000ffac,
                           in_stack_0000ffb0);
  pass1_1008_3e94((u16 *)((u32)puVar3 & 0xffff0000 | (u32)((int)puVar3 + 0xe)),(u16 *)CONCAT22(uVar2,puVar4),
                  (char *)CONCAT22(uVar6,puVar5));
  return;
}



// WARNING: Unable to use type for symbol uVar3

void unk_draw_op_1020_0c3e(astruct_771 *param_1)

{
  u32 *puVar2;
  HDC16 *hpal;
  HPALETTE16 obj;
  astruct_771 *struct_1;
  astruct_842 *iVar5;
  u16 uVar5;
  u16 uVar6;
  u16 uStack40;
  HDC16 local_24;
  u8 paintstruct_22 [0x20];
  u32 uVar3;
  u32 *puVar1;
  u32 uVar4;
  code **fn_ptr_1;

  uVar5 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_771 *)param_1;
  local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct_1->hwnd_0x4);
  uVar3 = struct_1->field5_0x6;
  uVar6 = ((u32)uVar3 >> 0x10);
  iVar5 = (astruct_842 *)uVar3;
  puVar2 = (u32*)&iVar5->field_0xa;
  uStack40 = puVar2;
  if ((iVar5->field12_0xc | uStack40) != 0x0) {
    hpal = &local_24;
    uVar4 = *puVar2;
    fn_ptr_1 = (code **)((int)uVar4 + 0x8);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,uStack40,(int)((u32)puVar2 >> 0x10),hpal,(int)&DAT_1050_1050);
    fn_ptr_1 = (code **)((int)uVar4 + 0x4);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,puVar2,struct_1->field7_0xc,struct_1->field6_0xa,&local_24,
                 (int)&DAT_1050_1050);
    obj = SelectPalette16(0x0,(HPALETTE16)hpal,local_24);
    DeleteObject16(obj);
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct_1->hwnd_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4

void win_ui_palette_op_1020_0cd2(astruct_775 *struct_param_1)

{
  u32 *puVar2;
  code **ppcVar3;
  u16 uVar7;
  HDC16 hdc;
  HDC16 hpal;
  HPALETTE16 hpal_00;
  u16 UVar4;
  u16 DX_REG;
  u16 uVar8;
  astruct_775 *struct_1;
  astruct_774 *iVar5;
  u16 uVar5;
  u16 uVar6;
  HDC16 hdc_00;
  astruct_13 *paStack10;
  u16 uStack6;
  u32 *puVar1;
  u32 uVar4;

  uVar5 = ((u32)struct_param_1 >> 0x10);
  struct_1 = (astruct_775 *)struct_param_1;
  uVar4 = struct_1->field5_0x6;
  uVar6 = ((u32)uVar4 >> 0x10);
  iVar5 = (astruct_774 *)uVar4;
  puVar2 = (u32*)&iVar5->field_0xa;
  uStack6 = puVar2;
  uVar7 = iVar5->field12_0xc | uStack6;
  if (uVar7 != 0x0) {
    ppcVar3 = (code **)((int)*puVar2 + 0x14);
    (**ppcVar3)();
    paStack10 = (astruct_13 *)CONCAT22(DX_REG,uVar7);
    uVar8 = DX_REG | uVar7;
    if (uVar8 != 0x0) {
      hdc = GetDC16(struct_1->field4_0x4);
      hpal = hdc;
      hdc_00 = hdc;
      create_palette_1008_4e38(paStack10,uVar8);
      hpal_00 = SelectPalette16(0x0,hpal,hdc_00);
      UVar4 = RealizePalette16(hdc);
      SelectPalette16(0x1,hpal_00,hdc);
      DeleteObject16(hpal);
      if (0x0 < (int)UVar4) {
        InvalidateRect16(0x1,NULL,0x0);
      }
      ReleaseDC16(hdc,struct_1->field4_0x4);
      return;
    }
  }
  return;
}



StructD * pass1_1020_0d82(StructD *param_1,u8 param_2)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x3ab0;
  ((int)param_1 + 0x2) = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1020_0dc4(StructA *param_1,u16 param_2,u32 param_3,u32 param_4,u16 param_5,
                    u16 param_6)

{
  char **ppcVar1;
  StructA *iVar1;
  u16 uVar2;
  u16 in_stack_0000fe16;
  u16 in_stack_0000fe1a;
  u16 in_stack_0000fe59;
  u16 in_stack_0000fe64;
  u16 in_stack_0000fe68;
  u16 in_stack_0000ff40;
  u16 in_stack_0000ff44;
  u16 in_stack_0000ff48;
  u16 in_stack_0000ff8e;
  u16 in_stack_0000ff92;
  u16 in_stack_0000ff96;

  struct_1020_790e(&param_1->field0_0x0,s_PCPOPMENU_1050_4256,param_2,param_3);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (StructA *)param_1;
  (u32)&iVar1[0x1].field20_0x26 = 0x0;
  (u32)&iVar1[0x1].field22_0x2a = 0x0;
  iVar1[0x1].field25_0x2e = 0x0;
  param_1->field0_0x0 = 0x1384;
  iVar1->field1_0x2 = 0x1020;
  ppcVar1 = &iVar1->field60_0x5b;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(ppcVar1)),s_VrMode_1050_4260);
  iVar1->field140_0xac = 0x44c00000;
  window_op_1020_10a0(ppcVar1,param_4,(StructA *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10),param_5,param_6,
                      in_stack_0000fe16,in_stack_0000fe1a,in_stack_0000fe59,in_stack_0000fe68,in_stack_0000ff40,
                      in_stack_0000ff44,in_stack_0000ff48,in_stack_0000ff8e,in_stack_0000ff92,in_stack_0000ff96,
                      in_stack_0000fe64);
  return;
}



void pass1_1020_0e2c(astruct_868 *param_1)

{
  get_win_ui_info_op_1020_7a50(param_1);
  cleanup_ui_op_1020_1038(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar2

void realize_palette_1020_0e46(u32 param_1,i16 param_2)

{
  u32 uVar1;
  HGDIOBJ16 puVar4;
  astruct_800 *iVar4;
  u16 uVar4;
  u16 uVar5;
  u32 uVar2;
  u32 *puVar2;
  code **fn_ptr_1;

  if (param_2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    uVar2 = (u32)((int)param_1 + 0xf2);
    uVar5 = ((u32)uVar2 >> 0x10);
    iVar4 = (astruct_800 *)uVar2;
    puVar2 = iVar4->field102_0x66;
    fn_ptr_1 = (code **)((int)*puVar2 + 0x18);
    (**fn_ptr_1)();
    UnrealizeObject16((HGDIOBJ16)puVar2);
    uVar1 = (u32)((int)param_1 + 0xf2);
    RealizePalette16(*(HDC16 *)((int)uVar1 + 0x7c));
  }
  return;
}



void pass1_1020_0e8e(i16 param_1,u16 param_2,i16 param_3,u16 param_4,i16 param_5,i16 param_6)

{
  code **ppcVar1;

  win_ui_cursor_op_1020_1294(param_2,CONCAT22(param_4,param_3),param_5,param_6);
  if (param_1 == 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(param_3 + 0x4) + 0x5c);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_help_op_1020_0ec4(u16 param_1,u32 *param_2,u16 param_3)

{
  code **ppcVar1;
  char cVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u16 uVar7;
  u16 unaff_CS;
  astruct_477 *paVar8;
  u32 *puVar9;
  u32 uVar10;
  u16 in_stack_0000fe98;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  WPARAM16 wparam;
  HWND16 hwnd;
  i16 iVar11;
  u16 in_stack_0000fff4;
  u16 uVar12;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar7 = ((u32)param_2 >> 0x10);
  uVar3 = param_2;
  if (param_3 == 0xfb) {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,0x30),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    uVar12 = puVar9;
    pass1_1010_375e((u32)puVar9);
    ppcVar1 = (code **)((int)*param_2 + 0x14);
    (**ppcVar1)();
    uVar10 = pass1_1010_375e((u32)puVar9 & 0xffff0000 | (u32)uVar12);
    uVar12 = (uVar10 >> 0x10);
    pass1_1010_4788(uVar10,uVar12,(u32)(uVar3 + 0xf2),(char *)(uVar10 & 0xffff | (u32)uVar12 << 0x10));
    return;
  }
  if (0xfb < param_3) {
    switch(param_3) {
    default:
      return;
    case 0x12a:
      hwnd = *(HWND16 *)(uVar3 + 0x8);
      wparam = 0xf012;
      break;
    case 0x12c:
      hwnd = *(HWND16 *)(uVar3 + 0x8);
      wparam = 0xf020;
    }
    PostMessage16(0x0,wparam,0x112,hwnd);
    return;
  }
  if (param_3 == 0xf3) {
    iVar11 = 0x3;
  }
  else {
    if (0xf3 < param_3) {
      return;
    }
    cVar2 = (char)param_3;
    if ((u8)(cVar2 + 0x91U) == 0x0) {
      uVar4 = FUN_1010_830a(param_3 & 0xff00 | (u8)(cVar2 + 0x91U),paVar6,unaff_CS,_u16_1050_14cc,0x1f8);
      WinHelp16(0x28,0x1,(char *)CONCAT22((int)paVar6,uVar4),*(HWND16 *)(uVar3 + 0x8));
      return;
    }
    if (cVar2 == 'r') {
      iVar11 = uVar3 + 0xa;
      uVar4 = uVar7;
      paVar8 = (astruct_477 *)
               mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(iVar11,0x30),in_stack_0000fe98,
                               in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
      uVar5 = ((u32)paVar8 >> 0x10);
      pass1_1010_3770(uVar5,paVar8,(char *)CONCAT22(uVar4,iVar11));
      pass1_1038_af40(uVar3,uVar5,_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x3);
      return;
    }
    if (cVar2 == -0xf) {
      iVar11 = 0x1;
    }
    else {
      if (cVar2 != -0xe) {
        return;
      }
      iVar11 = 0x2;
    }
  }
  pass1_1010_4674(*(astruct_27 **)(uVar3 + 0xf2),iVar11,0x1010,&DAT_1050_1050);
  return;
}



void enable_menu_1020_1000(u16 param_1,u16 param_2,u16 param_3,i16 param_4,HMENU16 param_5)

{
  if (param_4 != 0x0) {
    return;
  }
  EnableMenuItem16(0x400,0x3,param_5);
  return;
}



void FUN_1020_101f(void)

{
  return;
}



void pass1_1020_1022(u32 param_1)

{
  draw_op_1020_15de((u32)((int)param_1 + 0xf6));
  return;
}



void cleanup_ui_op_1020_1038(astruct_868 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_868 *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_868 *)param_1;
  DestroyIcon16(iVar4->hicon_0xc2);
  iVar4->hicon_0xc2 = 0x0;
  iVar4->field8_0x8 = 0x0;
  puVar1 = iVar4->field241_0xf6;
  uVar2 = iVar4->field242_0xf8;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)((int)s_tile2_bmp_1050_1538,puVar1,uVar2,0x1);
  }
  (u32)&iVar4->field241_0xf6 = 0x0;
  pass1_1010_1dda(iVar4->field240_0xf2);
  iVar4->field240_0xf2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void window_op_1020_10a0(astruct_57 *param_1,astruct_57 *param_2,StructA *struct_param_1,u16 param_4,u16 param_5,
                        u16 param_6,u16 param_7,u16 param_8,u16 param_9,u16 param_10,
                        u16 param_11,u16 param_12,u16 param_13,u16 param_14,
                        u16 param_15,u16 param_16)

{
  u32 uVar1;
  code **ppcVar2;
  astruct_160 *paVar3;
  INT16 *pIVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  astruct_57 *paVar9;
  u16 unaff_SI;
  u32 *puVar10;
  u32 uVar11;
  u32 uVar12;
  u16 uVar13;
  u16 uVar14;
  StructA *struct_1;
  astruct_57 *paVar8;

  struct_1 = (StructA *)struct_param_1;
  uVar14 = ((u32)struct_param_1 >> 0x10);
  create_window_ex_1008_9760(struct_param_1);
  mem_op_1000_179c(0x42,param_2);
  uVar5 = (astruct_57 *)param_2 | param_1;
  paVar8 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar5);
  if (uVar5 != 0x0) {
    pass1_1008_3bd6((u32)paVar8,param_1,(astruct_57 *)param_2,0x0,0x1f009b,0x0,0x740075,
                    CONCAT22(struct_1->field4_0x8,0xf1),param_4,param_6,param_7,param_10,param_11,param_12);
  }
  mem_op_1000_179c(0x42,paVar8);
  uVar5 = (astruct_57 *)paVar8 | param_1;
  paVar9 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar5);
  if (uVar5 != 0x0) {
    pass1_1008_3bd6((u32)paVar9,param_1,(astruct_57 *)paVar8,0x0,0x31009b,0x0,0x760077,
                    CONCAT22(struct_1->field4_0x8,0xf2),param_4,param_6,param_7,param_10,param_11,param_12);
  }
  mem_op_1000_179c(0x42,paVar9);
  uVar5 = (astruct_57 *)paVar9 | param_1;
  paVar8 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar5);
  if (uVar5 != 0x0) {
    pass1_1008_3bd6((u32)paVar8,param_1,(astruct_57 *)paVar9,0x0,0x77009b,0x0,0x780079,
                    CONCAT22(struct_1->field4_0x8,0xf3),param_4,param_6,param_7,param_10,param_11,param_12);
  }
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2d),param_9,param_5,param_14,
                            param_15);
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar10 >> 0x10);
  struct_1[0x1].field20_0x26 = (astruct_243 *)puVar10;
  uVar6 = ((u32)puVar10 >> 0x10);
  struct_1[0x1].field21_0x28 = uVar6;
  struct_1[0x1].field10_0x14 = (i16)struct_1[0x1].field20_0x26;
  struct_1[0x1].field11_0x16 = uVar6;
  paVar3 = (astruct_160 *)LoadIcon16(s_PLNTICON_1050_4267,HINSTANCE16_1050_038c);
  *(astruct_160 **)&struct_1->field_0xc2 = paVar3;
  uVar1 = (u32)&struct_1[0x1].field20_0x26;
  uVar7 = uVar1;
  ppcVar2 = (code **)((int)*(u32*)&struct_1[0x1].field20_0x26 + 0x30);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,uVar7,(int)((u32)uVar1 >> 0x10),paVar3);
  mem_op_1000_179c(0x24,paVar8);
  uVar5 = paVar8 | paVar3;
  paVar9 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar5);
  if (uVar5 == 0x0) {
    (u32)&struct_1[0x1].field22_0x2a = 0x0;
  }
  else {
    unk_win_ui_op_1020_1418((astruct_40 *)CONCAT22(paVar8,paVar3),struct_param_1,param_5);
    struct_1[0x1].field22_0x2a = paVar3;
    &struct_1[0x1].field_0x2c = (int)paVar9;
  }
  (u32)&struct_1[0x1].field14_0x1c = (u32)&struct_1[0x1].field22_0x2a;
  puVar10 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(uVar7,0x2f),param_16,param_4,param_13,param_14)
  ;
  uVar12 = (u32)paVar9 & 0xffff0000;
  uVar11 = pass1_1018_04b8((u32)puVar10);
  paVar8 = (astruct_57 *)(uVar12 & 0xffff0000 | uVar11 >> 0x10);
    // WARNING: Load size is inaccurate
  pass1_1010_41d6(paVar8,struct_1[0x1].field20_0x26,uVar11);
  uVar12 = pass1_1010_451a((u8 *)paVar8,(u32)&struct_1[0x1].field20_0x26);
  uVar7 = (uVar12 >> 0x10);
  pIVar4 = (INT16 *)uVar12;
  uVar1 = (u32)struct_param_1;
  ppcVar2 = (code **)((int)uVar1 + 0x14);
  (**ppcVar2)(0x1010,struct_param_1,0x0,(char)uVar12,uVar7);
  uVar13 = 0x1;
  ppcVar2 = (code **)((int)uVar1 + 0x10);
  (**ppcVar2)();
  pass1_1010_459e(*(astruct_27 **)&struct_1[0x1].field20_0x26);
  ppcVar2 = (code **)((int)*(u32*)&struct_1[0x1].field20_0x26 + 0x10);
  (**ppcVar2)(0x1010,(u32)&struct_1[0x1].field20_0x26,struct_param_1,uVar13);
  MoveWindow16(0x1,pIVar4[0x3],pIVar4[0x2],pIVar4[0x1],*pIVar4,struct_1->field4_0x8);
  UpdateWindow16(struct_1->field4_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_cursor_op_1020_1294(u16 param_1,u32 param_2,i16 param_3,i16 param_4)

{
  u32 uVar1;
  code **ppcVar2;
  u16 in_AX;
  HCURSOR16 hcursor;
  HCURSOR16 hcursor_00;
  u16 in_register_0000000a;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 in_stack_0000ffc2;
  i16 local_12;
  i16 local_10;
  u16 *puStack14;
  u32 *puStack10;
  i16 local_6;
  i16 iStack4;

  pass1_1030_8344(_u16_1050_5748,0x4000001);
  if ((param_1 | in_AX) == 0x0) {
    local_6 = param_4;
    iStack4 = param_3;
    uVar5 = (param_2 >> 0x10);
    iVar4 = (int)param_2;
    puStack10 = (u32 *)pass1_1010_40cc(param_3,0x0,(u32)(iVar4 + 0xf2));
    uVar3 = CONCAT22(in_register_0000000a,(int)((u32)puStack10 >> 0x10));
    uVar1 = (u32)(iVar4 + 0xf2);
    puStack14 = (u16 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x76));
    pass1_1008_3e94(puStack14,(u16 *)CONCAT22(0x1050,&local_12),(char *)CONCAT22(0x1050,&local_10));
    local_6 -= local_10;
    iStack4 -= local_12;
    hcursor = pt_in_rect_1010_40f8
                        (uVar3,(u32)(iVar4 + 0xf2),(POINT16 *)CONCAT22(0x1050,&local_6),in_stack_0000ffc2);
    if (hcursor != 0xffff) {
      hcursor_00 = LoadCursor16((char *)0x7f02,0x0);
      SetCursor16(hcursor_00);
      ppcVar2 = (code **)((int)*puStack10 + 0x4);
      (**ppcVar2)();
      pass1_1008_3e0e((StructA *)param_2);
      SetCursor16(hcursor);
    }
  }
  return;
}



StructD * pass1_1020_135e(StructD *param_1,u8 param_2)

{
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1020_1418(astruct_40 *param_1,StructA *param_2,u16 param_3)

{
  u32 uVar1;
  astruct_13 *paVar2;
  code **ppcVar3;
  HDC16 *pHVar4;
  StructA *pSVar5;
  u8 *puVar6;
  u32 in_EDX;
  astruct_57 *paVar7;
  astruct_40 *iVar5;
  u16 uVar9;
  u16 *puVar10;
  u32 *puVar11;
  u16 in_stack_0000fe84;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  HDC16 local_8;
  u32 *puStack6;
  u16 uVar8;

  uVar8 = ((u32)in_EDX >> 0x10);
  get_sys_metrics_1020_7c1a(param_1,param_2);
  uVar9 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_40 *)param_1;
  (u32)&iVar5->field17_0x14 = 0x0;
  iVar5->field20_0x18 = NULL;
  puVar10 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1e)));
  paVar7 = (astruct_57 *)CONCAT22(uVar8,(int)((u32)puVar10 >> 0x10));
  param_1->field0_0x0 = 0x1730;
  iVar5->field1_0x2 = 0x1020;
  puVar11 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2d),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  iVar5->field17_0x14 = (int)puVar11;
  &iVar5->field_0x16 = (int)((u32)puVar11 >> 0x10);
  puStack6 = mixed_1010_20ba((astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)puVar11 >> 0x10),_u16_1050_0ed0,
                             (u8 **)CONCAT22(param_3,0x29),in_stack_0000fe84,in_stack_0000ffa8,in_stack_0000ffae,
                             in_stack_0000ffb2);
  puVar6 = (u8 *)((u32)puStack6 >> 0x10);
  uVar1 = (u32)&iVar5->field17_0x14;
  ppcVar3 = (code **)((int)*(u32*)&iVar5->field17_0x14 + 0x4);
  (**ppcVar3)(0x1010,(int)uVar1,(int)((u32)uVar1 >> 0x10),0x0,param_1);
  local_8 = GetDC16(iVar5->hwnd_0x4);
  uVar1 = (u32)&iVar5->field17_0x14;
  *(HDC16 *)((int)uVar1 + 0x7c) = local_8;
  uVar1 = (u32)&iVar5->field17_0x14;
  pSVar5 = *(StructA **)((int)uVar1 + 0x66);
  iVar5->field20_0x18 = pSVar5;
  ppcVar3 = (code **)((int)(u32)iVar5->field20_0x18 + 0x14);
  (**ppcVar3)();
  paVar2 = *(astruct_13 **)((int)puStack6 + 0xe);
  pass1_1008_4d84(puVar6,(astruct_90 *)((u32)pSVar5 & 0xffff | ZEXT24(puVar6) << 0x10),(u32)paVar2);
  pHVar4 = (HDC16 *)palette_op_1008_4e08((HPALETTE16)&local_8,puVar6,paVar2,(HDC16 *)CONCAT22(0x1050,&local_8));
  iVar5->field21_0x1c = pHVar4;
  return;
}



// WARNING: Unable to use type for symbol uVar1

void win_ui_op_1020_150e(StructD *param_1)

{
  HPALETTE16 obj;
  StructD *iVar1;
  u16 uVar2;
  u16 unaff_SS;
  i32 uVar1;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x1730;
  iVar1->address_offset_field_0x2 = 0x1020;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1ea6(iVar1->field12_0x14,(StructD *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  uVar1 = iVar1->field12_0x14;
  obj = SelectPalette16(0x0,*(HPALETTE16 *)&iVar1->field_0x1c,*(HDC16 *)((int)uVar1 + 0x7c));
  *(HPALETTE16 *)&iVar1->field_0x1c = obj;
  DeleteObject16(obj);
  param_1->address_offset_field_0x0 = 0x3ab0;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



void invalidate_rect_1020_157c(u32 param_1,i16 param_2)

{
  BOOL16 BVar1;
  i16 iVar2;
  u16 uVar3;
  RECT16 local_a;
  u16 uStack4;

  iVar2 = (int)param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (u32)(iVar2 + 0x14) = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    BVar1 = IsIconic16(*(HWND16 *)(iVar2 + 0x4));
    if (BVar1 == 0x0) {
      local_a.x = *(INT16 *)(iVar2 + 0x4);
      GetClientRect16(&local_a,(HWND16)&DAT_1050_1050);
      uStack4 = 0x9a;
      InvalidateRect16(0x0,&local_a,(HWND16)&DAT_1050_1050);
      return;
    }
  }
  return;
}



// WARNING: Unable to use type for symbol uVar2

void draw_op_1020_15de(astruct_779 *param_1)

{
  code **ppcVar1;
  BOOL16 BVar2;
  astruct_779 *iVar3;
  u16 uVar3;
  u16 uVar4;
  astruct_76 *paVar3;
  HDC16 local_24;
  u8 local_22 [0x20];
  u32 uVar2;
  u32 uVar1;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_779 *)param_1;
  local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),*(HWND16 *)&iVar3->field_0x4);
  BVar2 = IsIconic16(*(HWND16 *)&iVar3->field_0x4);
  if (BVar2 == 0x0) {
    uVar4 = 0x1010;
    paVar3 = (astruct_76 *)pass1_1010_454a((u32)iVar3->field20_0x14);
    if ((((u32)paVar3 >> 0x10) | paVar3) != 0x0) {
      uVar1 = (u32)iVar3->field20_0x14;
      uVar4 = 0x1008;
      pass1_1008_4480(*(astruct_76 **)(iVar3 + 0x1),(u16 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x76)),paVar3);
    }
    uVar2 = (u32)(iVar3 + 0x1);
    ppcVar1 = (code **)((int)*(u32*)(iVar3 + 0x1) + 0x4);
    (**ppcVar1)(uVar4,(int)uVar2,(int)((u32)uVar2 >> 0x10),0x0,0x0,&local_24,(int)&DAT_1050_1050);
  }
  else {
    draw_op_1020_1674(param_1);
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),*(HWND16 *)&iVar3->field_0x4);
  return;
}



// WARNING: Unable to use type for symbol puVar1

void draw_op_1020_1674(astruct_779 *param_1)

{
  HDC16 hdc;
  astruct_779 *struct_1;
  u16 uVar2;
  RECT16 rect_1a;
  i16 iStack22;
  i16 iStack20;
  i16 iStack18;
  i16 iStack16;
  RECT16 rect_e;
  i16 iStack10;
  i16 iStack8;
  HGDIOBJ16 brush_handle;
  HICON16 icon_handle;
  u32 *puVar1;
  code **fn_ptr_1;

  if (PTR_LOOP_1050_0010 == NULL) {
    uVar2 = ((u32)param_1 >> 0x10);
    struct_1 = (astruct_779 *)param_1;
    fn_ptr_1 = (code **)((int)*struct_1->field20_0x14 + 0x2c);
    icon_handle = (**fn_ptr_1)();
    if (icon_handle != 0x0) {
      brush_handle = GetStockObject16(BLACK_BRUSH);
      GetClientRect16(&rect_e,(HWND16)&DAT_1050_1050);
      rect_1a.x = 0x0;
      rect_1a.y = 0x0;
      iStack22 = (iStack10 - rect_e.x) + -0x1;
      iStack20 = (iStack8 - rect_e.y) + -0x1;
      puVar1 = struct_1->field20_0x14;
      hdc = *(HDC16 *)((int)puVar1 + 0x7c);
      iStack18 = iStack20;
      iStack16 = iStack22;
      FillRect16(brush_handle,&rect_1a,(HDC16)&DAT_1050_1050);
      DrawIcon16(icon_handle,0x2,0x2,hdc);
    }
  }
  return;
}



StructD * pass1_1020_170a(StructD *param_1,u8 param_2)

{
  win_ui_op_1020_150e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1020_1738(astruct_57 *param_1,u16 param_2,u32 param_3)

{
  astruct_57 *iVar1;
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcd,((int)param_3 + 0x8));
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_57 *)param_1;
  (u32)(iVar1 + 0x1) = 0x0;
  (u32)&iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field4_0x8 = 0x0;
  param_1->field0_0x0 = 0x1e7a;
  iVar1->field1_0x2 = 0x1020;
  return;
}



void pass1_1020_1780(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x6c);
  (**ppcVar1)();
  destroy_win_1040_8212((astruct_899 *)param_1);
  return;
}



// WARNING: Unable to use type for symbol puVar2
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_ui_op_1020_179c(u16 param_1,u16 param_2,StructB *structb_param_1)

{
  u16 uVar1;
  u16 uVar2;
  LPVOID pvVar1;
  i16 IVar1;
  i16 iVar2;
  astruct_816 *puVar11;
  u16 uVar3;
  u16 uVar4;
  i16 x;
  i16 y_offset;
  i16 IVar2;
  HWND16 HVar3;
  u16 uVar8;
  u8 *puVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  astruct_57 *paVar8;
  StructB *struct_9;
  i16 iVar9;
  StructB *uVar10;
  u16 uVar13;
  u16 uVar11;
  u16 uVar14;
  u32 *puVar10;
  u16 in_stack_0000fd2a;
  u16 in_stack_0000fd2e;
  u16 in_stack_0000fd30;
  u16 in_stack_0000fe4e;
  u16 in_stack_0000fe52;
  u16 in_stack_0000fe54;
  u16 in_stack_0000fe58;
  u16 in_stack_0000fe5a;
  u16 in_stack_0000fe5c;
  u16 in_stack_0000fe5e;
  u16 uVar16;
  char *in_buffer_4;
  u16 in_stack_0000fe88_00;
  short in_buf_len_5;
  i16 x_6e;
  i16 y;
  u16 x106;
  i16 y4;
  HWND16 hwnd_10;
  i16 cx;
  i16 cy;
  u16 uStack78;
  u16 uStack76;
  u32 uStack74;
  HWND16 HStack70;
  u32 uStack68;
  u32 uStack64;
  void *pvStack60;
  u16 uStack56;
  char **ppcStack54;
  u32 uStack50;
  astruct_92 *local_2e;
  RECT16 local_1c;
  i16 iStack22;
  i16 iStack20;
  i16 iStack18;
  u32 *puStack16;
  INT16 *pIStack12;
  u16 uStack8;
  u32 *puStack6;
  u32 *puVar1;
  u32 puVar2;
  u16 uVar12;
  u8 *in_resc_id_3;
  u16 uVar15;
  u16 in_stack_0000fe88;
  u32 uVar9;
  code **fn_ptr_1;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(structb_param_1);
  uVar15 = 0x89;
  puStack6 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x890009,in_stack_0000fd2e,in_stack_0000fe52,
                             in_stack_0000fe58,in_stack_0000fe5c);
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uVar1 = pass1_1010_65d0((u32)puStack6,uVar15);
  uStack8 = (uVar1 == 0x0);
  uVar2 = pass1_1010_65d0((u32)puStack6,0x86);
  if (uVar2 != 0x0) {
    uStack8 = 0x0;
  }
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe88_00,0x39),in_stack_0000fd30,
                            in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar10 >> 0x10);
  pvVar1 = (LPVOID)puVar10;
  uVar10 = (StructB *)((u32)structb_param_1 >> 0x10);
  struct_9 = (StructB *)structb_param_1;
  struct_9[0x7].field1_0x2 = pvVar1;
  HVar3 = (HWND16)((u32)puVar10 >> 0x10);
  struct_9[0x7].hwnd_0x6 = HVar3;
  uVar16 = struct_9[0x7].field1_0x2;
  fn_ptr_1 = (code **)((int)*(u32*)&struct_9[0x7].field1_0x2 + 0x10);
  (**fn_ptr_1)(0x1010,uVar16,HVar3,uStack8);
  mem_op_1000_179c(0x12,paVar8);
  uStack76 = paVar8;
  puVar4 = (u8 *)(uStack76 | pvVar1);
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | ZEXT24(puVar4));
  uStack78 = pvVar1;
  if (puVar4 == NULL) {
    (u32)&struct_9[0x7].lpvoid_field_0x8 = 0x0;
  }
  else {
    pass1_1020_1eea(puVar4,(astruct_663 *)CONCAT22(uStack76,pvVar1),structb_param_1,(HWND16)struct_9->lpvoid_field_0x8);
    struct_9[0x7].lpvoid_field_0x8 = pvVar1;
    struct_9[0x7].max_count_field_0x10 = paVar8;
  }
  puVar1 = (u32*)&struct_9[0x7].field1_0x2;
  pIStack12 = (INT16 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0xa));
  puStack16 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(uVar16,0x48),in_stack_0000fd2a,
                              in_stack_0000fe4e,in_stack_0000fe54,in_stack_0000fe58);
  GetClientRect16(&local_1c,(HWND16)&DAT_1050_1050);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  uVar13 = ((u32)pIStack12 >> 0x10);
  iVar9 = (i16)pIStack12;
  (iVar9 + 0x6) = IVar1 + iStack22;
  uVar11 = ((u32)puStack16 >> 0x10);
  iStack18 = ((int)puStack16 + 0xa);
  iStack20 = ((int)puStack16 + 0xc);
  (iVar9 + 0x2) = (iStack20 - (iVar9 + 0x6)) / 0x2;
  iVar2 = iStack18 - (iVar9 + 0x4);
  uVar5 = iVar2 >> 0xf;
  uVar9 = (u32)uVar5;
  *pIStack12 = iVar2 / 0x2;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_2e),0x1,0x0,0x100);
  uStack56 = 0x0;
  while( true ) {
    uVar6 = uVar9;
    puVar11 = (astruct_816 *)&local_2e;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,puVar11));
    uStack50 = CONCAT22(uVar6,puVar11);
    uVar7 = uVar6 | puVar11;
    uVar9 = (u32)uVar7;
    if (uVar7 == 0x0) break;
    ppcStack54 = (char **)puVar11->field16_0x10;
    if (ppcStack54 != NULL) {
      pass1_1000_3cea((u32)structb_param_1 & 0xffff0000 | ZEXT24(&struct_9->field8_0x10),*ppcStack54);
    }
  }
  uVar3 = pass1_1020_1da8(puVar11,0x0,structb_param_1);
  struct_9[0x7].field5_0xa = (u32 *)uVar3;
  uVar4 = pass1_1010_65d0((u32)puStack6,0x86);
  if ((uVar4 == 0x0) || (struct_9[0x7].field5_0xa != NULL)) {
    puVar2 = (u32)&struct_9[0x7].field1_0x2;
    ((int)puVar2 + 0x2c) = 0x0;
    hwnd_10 = GetDlgItem16(0x175,(HWND16)struct_9->lpvoid_field_0x8);
    if (uStack8 != 0x0) {
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,&stack0xfe88,(short)&DAT_1050_1050
                );
      SetWindowText16(CONCAT13(0x10,CONCAT12(0x50,&stack0xfe88)),hwnd_10);
    }
    pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c,destroy_win_1020_1e1e);
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&x_6e),hwnd_10);
    cx = (int)_x106 - x_6e;
    cy = y4 - y;
    x = cx - ((int)pIStack12 + 0x4);
    y_offset = GetSystemMetrics16(SM_CYCAPTION);
    MoveWindow16(0x0,cy,cx,y - y_offset,-x / 0x2,hwnd_10);
  }
  else {
    win_1008_5c7c(uVar4,uVar7,_u16_1050_02a0,0x9d0001);
    (struct_9 + 0x7)->field0_0x0 = uVar4;
    pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c,destroy_win_1020_1dea);
  }
  EnumChildWindows1(0x0,pvStack60,(HWND16)struct_9->lpvoid_field_0x8);
  FreeProcInstance16(pvStack60);
  HStack70 = GetDlgItem16(0x1,(HWND16)struct_9->lpvoid_field_0x8);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_1c),HStack70);
  uStack64 = _param_2;
  local_1c.x = (int)_param_2 - local_1c.x;
  uStack74 = CONCAT22(local_1c.x,iStack22 - local_1c.y);
  uStack68 = local_1c & 0xffff0000 | (u32)(-(local_1c.x - ((int)pIStack12 + 0x4)) / 0x2);
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  uStack68 = uStack68 & 0xffff | (u32)(uStack68 - IVar2) << 0x10;
  if (struct_9[0x7].field5_0xa == NULL) {
    if (uStack8 == 0x0) goto LAB_1020_1b24;
    in_buf_len_5 = 0x72e;
    in_resc_id_3 = &stack0xfe88;
    in_buffer_4 = (char *)&DAT_1050_1050;
  }
  else {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,&stack0xfe88,(short)&DAT_1050_1050);
    HVar3 = GetDlgItem16(0x175,(HWND16)struct_9->lpvoid_field_0x8);
    SetWindowText16(CONCAT22(0x1050,&stack0xfe88),HVar3);
    in_buffer_4 = &stack0xfe88;
    in_buf_len_5 = (short)&DAT_1050_1050;
    in_resc_id_3 = (u8 *)0x3ff;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),in_resc_id_3,in_buffer_4,
             in_buf_len_5);
  SetWindowText16(CONCAT22(0x1050,&stack0xfe88),HStack70);
LAB_1020_1b24:
  MoveWindow16(0x0,(INT16)uStack74,(INT16)(uStack74 >> 0x10),uStack68,(INT16)uStack68,HStack70);
  uVar14 = ((u32)pIStack12 >> 0x10);
  iVar2 = (int)pIStack12;
  SetWindowPos16(0x44,*(INT16 *)(iVar2 + 0x6),*(INT16 *)(iVar2 + 0x4),*(INT16 *)(iVar2 + 0x2),*pIStack12,0x0,
                 (HWND16)struct_9->lpvoid_field_0x8);
  return;
}



void pass1_1020_1b68(astruct *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct *iVar4;
  astruct *uVar4;

  uVar4 = (astruct *)((u32)param_1 >> 0x10);
  iVar4 = (astruct *)param_1;
  puVar1 = iVar4->field143_0x92;
  uVar2 = iVar4->field144_0x94;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)&iVar4->field143_0x92 = 0x0;
  pass1_1010_4f48(iVar4->field142_0x8e);
  iVar4->field142_0x8e = NULL;
  return;
}



u16 pass1_1020_1bb6(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void enable_window_1020_1bd4
               (u16 param_1,u16 param_2,astruct_901 *param_3,u16 param_4,u16 param_5,u32 param_6)

{
  code **ppcVar1;
  bool bVar2;
  HWND16 hwnd;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u16 uVar8;
  u32 *puStack12;
  u32 uVar7;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  bVar2 = false;
  pass1_1020_1d8e(CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,param_3)),CONCAT22((int)param_6,param_5));
  if (param_1 != 0x0) {
    if ((int)param_1 < 0x2) {
      bVar2 = true;
    }
    else {
      hwnd = GetDlgItem16(0x1,param_3->field6_0x6);
      pass1_1010_4e8c(param_3->field141_0x8e);
      param_1 = EnableWindow16(0x1,hwnd);
      pass1_1010_4df0(paVar6,param_3->field141_0x8e);
      if ((param_1 == 0x0) && (bVar2 = true, param_3->field146_0x96 == 0x0)) {
        param_1 = EnableWindow16(0x0,hwnd);
      }
    }
  }
  if (bVar2) {
    uVar8 = 0x1000;
    mem_op_1000_179c(0xb4,paVar6);
    uVar4 = paVar6 | param_1;
    uVar7 = (u32)paVar6 & 0xffff0000 | (u32)uVar4;
    if (uVar4 == 0x0) {
      iVar3 = 0x0;
      uVar5 = 0x0;
    }
    else {
      uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520(uVar7,(astruct_57 *)CONCAT22(paVar6,param_1),param_3->field6_0x6,0x20030,0x62a057b)
      ;
      uVar5 = uVar7;
    }
    puStack12 = (u32 *)CONCAT22(uVar5,iVar3);
    ppcVar1 = (code **)((int)*puStack12 + 0x74);
    (**ppcVar1)(uVar8,iVar3,uVar5);
  }
  return;
}



BOOL16 post_win_msg_1020_1ca4(u32 param_1,astruct_57 *param_2,u16 param_3)

{
  code **ppcVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar6;
  u32 *puStack10;
  u32 uVar5;

  uVar6 = (param_1 >> 0x10);
  if (((int)param_1 + 0x96) == 0x0) {
    pass1_1010_4df0(param_2,(u32)((int)param_1 + 0x8e));
    if (param_3 == 0x0) {
      uVar6 = 0x1000;
      mem_op_1000_179c(0xb4,param_2);
      uVar3 = param_2 | param_3;
      uVar5 = (u32)param_2 & 0xffff0000 | (u32)uVar3;
      if (uVar3 == 0x0) {
        iVar2 = 0x0;
        uVar4 = 0x0;
      }
      else {
        uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
        iVar2 = string_1040_8520(uVar5,(astruct_57 *)CONCAT22(param_2,param_3),HWND16_1050_0396,0x20030,0x62a057b)
        ;
        uVar4 = uVar5;
      }
      puStack10 = (u32 *)CONCAT22(uVar4,iVar2);
      ppcVar1 = (code **)((int)*puStack10 + 0x74);
      (**ppcVar1)(uVar6,iVar2,uVar4);
      return 0x0;
    }
    PostMessage16(0x0,0xde,0x111,HWND16_1050_0396);
  }
  return 0x1;
}



void set_win_tet_1020_1d2a(astruct_938 *param_1,u16 param_2,void *in_win_text_3,u16 param_4,INT16 in_dlg_id_5)

{
  HWND16 hwnd;

  hwnd = GetDlgItem16(param_4,param_1->field6_0x6);
  SetWindowText16((u32)in_win_text_3,hwnd);
  return;
}



void destroy_window_1020_1d4a(u32 param_1,i16 param_2)

{
  u16 in_AX;
  BOOL16 BVar1;
  u32 in_EDX;
  u16 uVar2;

  if (param_2 != 0x0) {
    BVar1 = post_win_msg_1020_1ca4(param_1,in_EDX,in_AX);
    if (BVar1 != 0x0) {
      uVar2 = (param_1 >> 0x10);
      if (((int)param_1 + 0x96) != 0x0) {
        PostMessage16(0x0,0xee,0x111,HWND16_1050_0396);
      }
      DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
    }
  }
  return;
}



void pass1_1020_1d8e(u32 param_1,u32 param_2)

{
  pt_in_rect_1010_4e08((u32)((int)param_1 + 0x8e),param_2,(param_2 >> 0x10));
  return;
}



// WARNING: Unable to use type for symbol uVar1

u16 pass1_1020_1da8(i16 param_1,u16 param_2,StructB *param_3)

{
  u32 uVar2;
  StructB *struct_b_1;
  u16 uVar3;
  u32 uVar1;

  uVar3 = ((u32)param_3 >> 0x10);
  struct_b_1 = (StructB *)param_3;
  uVar1 = (u32)&struct_b_1[0x7].field1_0x2;
  if (((int)uVar1 + 0x30) == 0x1) {
    return 0x1;
  }
  uVar2 = (u32)&struct_b_1[0x7].field1_0x2;
  if ((((int)uVar2 + 0x30) < 0x3) &&
     (pass1_1010_4df0(param_2,(u32)&struct_b_1[0x7].field1_0x2), param_1 == 0x0)) {
    return 0x1;
  }
  return 0x0;
}



BOOL16 destroy_win_1020_1dea(u16 param_1,u16 param_2,u16 param_3)

{
  BOOL16 BVar1;
  WORD WVar2;

  BVar1 = IsWindow16(param_3);
  if (BVar1 != 0x0) {
    WVar2 = GetWindowWord16(-0xc,param_3);
    if (WVar2 == 0x175) {
      DestroyWindow16(param_3);
      return 0x0;
    }
  }
  return 0x1;
}



u16 destroy_win_1020_1e1e(u16 param_1,u16 param_2,HWND16 param_3)

{
  BOOL16 BVar1;
  WORD WVar2;

  BVar1 = IsWindow16(param_3);
  if (BVar1 != 0x0) {
    WVar2 = GetWindowWord16(-0xc,param_3);
    if ((WVar2 != 0x1) && (WVar2 != 0x175)) {
      DestroyWindow16(param_3);
    }
  }
  return 0x1;
}



StructD * pass1_1020_1e54(StructD *param_1,u8 param_2)

{
  ui_cleanup_op_1040_782c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_1eea(u8 *param_1,astruct_663 *param_2,StructB *param_3,HWND16 param_4)

{
  code **ppcVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  astruct_663 *iVar3;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u32 in_stack_0000ffec;
  u8 **ppuVar5;

  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_663 *)param_2;
  param_2 = 0x389a;
  iVar3->field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar3->field2_0x2 = 0x1008;
  iVar3->field3_0x4 = param_4;
  param_2 = 0x3ab0;
  iVar3->field2_0x2 = 0x1008;
  iVar3->field4_0x6 = NULL;
  iVar3->field5_0xa = param_3;
  param_2 = 0x2518;
  iVar3->field2_0x2 = 0x1020;
  ppuVar5 = (u8 **)CONCAT22((int)((u32)in_stack_0000ffec >> 0x10),0x39);
  puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,ppuVar5,in_stack_0000fe96
                           ,in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = ((u32)puVar4 >> 0x10);
  &iVar3->field4_0x6 = (int)puVar4;
  ((int)&iVar3->field4_0x6 + 0x2) = uVar2;
  ppcVar1 = (code **)((int)*iVar3->field4_0x6 + 0x4);
  (**ppcVar1)(0x1010,&iVar3->field4_0x6,uVar2,0x0,param_2,(int)((u32)ppuVar5 >> 0x10));
  return;
}



void pass1_1020_1f74(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x2518;
  iVar1->address_offset_field_0x2 = 0x1020;
  pass1_1010_1ea6((u32)&iVar1->field_0x6,(StructD *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  param_1->address_offset_field_0x0 = 0x3ab0;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



void invalidate_rect_1020_1fb2(u32 param_1,i16 param_2)

{
  i16 iVar1;
  u16 uVar2;
  u16 local_16;
  u16 uStack20;
  i16 iStack18;
  u16 uStack16;
  i16 local_e [0x2];
  i16 iStack10;
  u16 uStack6;
  u16 uStack4;

  iVar1 = (int)param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (u32)(iVar1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_e),*(HWND16 *)(iVar1 + 0x4));
  uStack6 = 0x46;
  uStack20 = 0x46;
  iStack18 = iStack10 - local_e[0];
  uStack4 = 0x5f;
  uStack16 = 0x5f;
  local_16 = (iVar1 + 0x4);
  InvalidateRect16(0x0,(RECT16 *)&local_16,(HWND16)&DAT_1050_1050);
  return;
}



// WARNING: Inlined function: struct_1010_4d5c
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar5

void unk_draw_op_1020_2020(astruct_743 *param_1)

{
  i16 *piVar1;
  u32 uVar3;
  u32 uVar6;
  i16 iVar8;
  HDC16 *pHVar9;
  i16 iVar7;
  i16 iVar9;
  HGDIOBJ16 hgdiobj16_var8;
  HGDIOBJ16 hgdiobj16_var9;
  HGDIOBJ16 hgdiobj16_var10;
  HBRUSH16 HVar10;
  HPEN16 HVar11;
  u16 uVar11;
  HPALETTE16 obj;
  u16 uVar13;
  u16 DX_REG;
  u32 in_EDX;
  astruct_57 *paVar14;
  astruct_743 *iVar10;
  astruct_744 *iVar11;
  astruct_745 *iVar12;
  astruct_735 *puVar14;
  i16 iVar13;
  u16 uVar12;
  u16 uVar15;
  u16 uVar14;
  u16 *puVar15;
  u32 uVar16;
  i16 *piVar17;
  u8 local_38 [0x6];
  u16 local_32;
  u16 uStack48;
  u32 uStack46;
  u16 uStack42;
  u32 *puStack40;
  HDC16 hdc_24;
  u8 paintstruct16_22 [0x20];
  u32 uVar2;
  u32 *puVar4;
  u8 uVar18;
  u8 uVar19;
  u8 uVar20;
  u8 uVar21;
  u16 uVar22;
  u16 uVar23;
  u16 uVar24;
  astruct_746 *iVar14;
  astruct_748 *iVar15;
  u32 uVar1;
  u32 uVar4;
  u32 uVar5;
  i16 iVar16;
  code **fn_ptr_1;

  uVar24 = ((u32)in_EDX >> 0x10);
  puVar14 = (astruct_735 *)&stack0xfffe;
  uVar12 = ((u32)param_1 >> 0x10);
  iVar10 = (astruct_743 *)param_1;
  hdc_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct16_22),iVar10->field4_0x4);
  puStack40 = (u32 *)pass1_1010_4c2c(iVar10->field5_0x6);
  pHVar9 = &hdc_24;
  fn_ptr_1 = (code **)((int)*puStack40 + 0x8);
  (**fn_ptr_1)(0x1010,(int)puStack40,(int)((u32)puStack40 >> 0x10),pHVar9,(int)&DAT_1050_1050);
  iVar10->field12_0x10 = pHVar9;
  uVar2 = iVar10->field5_0x6;
  uStack42 = ((int)uVar2 + 0x30);
  uVar16 = iVar10->field5_0x6;
  uStack46 = *(u32*)((int)uVar16 + 0x12);
  uStack48 = 0x14;
  local_32 = 0x0;
  uVar14 = 0x1008;
  puVar15 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_38));
  paVar14 = (astruct_57 *)CONCAT22(uVar24,(int)((u32)puVar15 >> 0x10));
  while (&puVar14[-0x6].field_0x4 < (puVar14 + -0x4)) {
    iVar11 = (astruct_744 *)(&puVar14[-0x6].field_0x4 * 0x4);
    uVar6 = puVar14[-0x5].field6_0x6;
    uVar16 = pass1_1008_4772(*(astruct_76 **)(iVar11 + (int)uVar6));
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | uVar16 >> 0x10);
    &puVar14[-0x7].field_0x2 = (int)uVar16;
    uVar13 = (uVar16 >> 0x10);
    &puVar14[-0x7].field_0x4 = uVar13;
    uVar6 = puVar14->field6_0x6;
    pass1_1020_2286(uVar6,(uVar6 >> 0x10),(i16 *)CONCAT13(0x10,CONCAT12(0x50,&puVar14[-0x5].field_0x2)),
                    ((int)uVar16 + 0x8));
    uVar3 = (u32)&puVar14[-0x5].field_0x2;
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&puVar14[-0x6].field6_0x6),0x0,uVar3,((u32)uVar3 >> 0x10));
    uVar6 = puVar14[-0x5].field6_0x6;
    pass1_1008_4480(*(astruct_76 **)&puVar14[-0x4].field_0x2,(u16 *)CONCAT22(0x1050,&puVar14[-0x6].field6_0x6),
                    *(astruct_76 **)(iVar11 + (int)uVar6));
    iVar16 = &puVar14[-0x6].field_0x4;
    uVar3 = (u32)&puVar14[-0x5].field_0x2;
    uVar23 = uVar3;
    uVar20 = (u8)((u32)uVar3 >> 0x10);
    uVar21 = (u8)((u32)uVar3 >> 0x18);
    uVar3 = (u32)&puVar14[-0x7].field_0x2;
    uVar15 = ((u32)uVar3 >> 0x10);
    iVar12 = (astruct_745 *)uVar3;
    iVar7 = iVar12->field4_0x4 + &puVar14[-0x5].field_0x4;
    iVar9 = iVar12->field7_0x8 + &puVar14[-0x5].field_0x2;
    uVar6 = puVar14->field6_0x6;
    uVar3 = (u32)((int)uVar6 + 0x6);
    iVar14 = (astruct_746 *)uVar3;
    uVar22 = ((u32)uVar3 >> 0x10);
    uVar18 = '\b';
    uVar19 = '\x10';
    if (*(i32 *)&iVar14->field_0x1a == 0x0) {
      iVar8 = iVar14->field47_0x30 << 0x3;
      mem_op_1000_179c(iVar8,paVar14);
      &iVar14->field_0x1a = iVar8;
      iVar14->field28_0x1c = (u8 *)paVar14;
    }
    uVar3 = (u32)&iVar14->field_0x1a;
    iVar15 = (astruct_748 *)(iVar16 * 0x8);
    (iVar15 + (int)uVar3) = CONCAT11(uVar21,uVar20);
    uVar3 = (u32)&iVar14->field_0x1a;
    (iVar15 + (int)uVar3 + 0x2) = uVar23;
    uVar3 = (u32)&iVar14->field_0x1a;
    (iVar15 + (int)uVar3 + 0x4) = iVar7;
    uVar3 = (u32)&iVar14->field_0x1a;
    (iVar15 + (int)uVar3 + 0x6) = iVar9;
    uVar14 = CONCAT11(uVar19,uVar18);
    uVar3 = (u32)&puVar14[-0x7].field_0x2;
    piVar1 = (i16 *)&puVar14[-0x5].field_0x4;
    *piVar1 = *piVar1 + (-(&puVar14[-0x6].field_0x4 == 0x0) & 0x5) + 0x14 + ((int)uVar3 + 0x4);
    piVar1 = (i16 *)&puVar14[-0x6].field_0x4;
    *piVar1 = *piVar1 + 0x1;
  }
  puVar4 = (u32*)&puVar14[-0x4].field_0x2;
  fn_ptr_1 = (code **)((int)*puVar4 + 0x4);
  (**fn_ptr_1)(uVar14,(int)puVar4,(int)((u32)puVar4 >> 0x10),0x0,0x0,(char)puVar14 + -0x22,(int)&DAT_1050_1050);
  DX_REG = paVar14;
  hgdiobj16_var8 = CreatePen16(0x1000025,0x1,0x0);
  *(HGDIOBJ16 *)&puVar14[-0x6].field_0x2 = hgdiobj16_var8;
  hgdiobj16_var10 = SelectObject16(hgdiobj16_var8,*(HDC16 *)&puVar14[-0x4].field6_0x6);
  *(HGDIOBJ16 *)(puVar14 + -0x6) = hgdiobj16_var10;
  hgdiobj16_var9 = CreateSolidBrush16(0x1000025);
  *(HGDIOBJ16 *)((int)&puVar14[-0x7].field6_0x6 + 0x2) = hgdiobj16_var9;
  hgdiobj16_var10 = SelectObject16(hgdiobj16_var9,*(HDC16 *)&puVar14[-0x4].field6_0x6);
  *(HGDIOBJ16 *)&puVar14[-0x7].field6_0x6 = hgdiobj16_var10;
  draw_line_1020_229c(puVar14->field6_0x6,&puVar14[-0x4].field6_0x6);
  uVar1 = puVar14->field6_0x6;
  pass1_1010_4df0(DX_REG,(u32)((int)uVar1 + 0x6));
  if (hgdiobj16_var10 == 0x0) {
    hgdiobj16_var10 = SelectObject16(*(HGDIOBJ16 *)(puVar14 + -0x6),*(HDC16 *)&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    hgdiobj16_var10 = SelectObject16(*(HGDIOBJ16 *)&puVar14[-0x7].field6_0x6,*(HDC16 *)&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    HVar10 = CreateSolidBrush16(0xff);
    *(HBRUSH16 *)((int)&puVar14[-0x7].field6_0x6 + 0x2) = HVar10;
    HVar11 = CreatePen16(0xff,0x1,0x0);
    *(HPEN16 *)&puVar14[-0x6].field_0x2 = HVar11;
    SelectObject16(*(HGDIOBJ16 *)((int)&puVar14[-0x7].field6_0x6 + 0x2),*(HDC16 *)&puVar14[-0x4].field6_0x6);
    SelectObject16(*(HGDIOBJ16 *)&puVar14[-0x6].field_0x2,*(HDC16 *)&puVar14[-0x4].field6_0x6);
  }
  uVar5 = puVar14->field6_0x6;
  piVar17 = (i16 *)pass1_1010_4dc8((u32)((int)uVar5 + 0x6));
  uVar13 = ((u32)piVar17 >> 0x10);
  uVar11 = piVar17;
  pass1_1020_239c(puVar14->field6_0x6,piVar17);
  uVar6 = puVar14->field6_0x6;
  uVar4 = (u32)((int)uVar6 + 0x6);
  if (((int)uVar4 + 0x2c) != 0x0) {
    pass1_1020_2488(uVar11,uVar13,puVar14->field6_0x6);
  }
  uVar6 = puVar14->field6_0x6;
  obj = SelectPalette16(0x0,*(HPALETTE16 *)((int)uVar6 + 0x10),*(HDC16 *)&puVar14[-0x4].field6_0x6);
  DeleteObject16(obj);
  hgdiobj16_var10 = SelectObject16(*(HGDIOBJ16 *)(puVar14 + -0x6),*(HDC16 *)&puVar14[-0x4].field6_0x6);
  DeleteObject16(hgdiobj16_var10);
  hgdiobj16_var10 = SelectObject16(*(HGDIOBJ16 *)&puVar14[-0x7].field6_0x6,*(HDC16 *)&puVar14[-0x4].field6_0x6);
  DeleteObject16(hgdiobj16_var10);
  uVar6 = puVar14->field6_0x6;
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,(u8 *)((int)&puVar14[-0x4].field6_0x6 + 0x2)),
             *(HWND16 *)((int)uVar6 + 0x4));
  return;
}



void pass1_1020_2286(u16 param_1,u16 param_2,i16 *param_3,i16 param_4)

{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}



// WARNING: Unable to use type for symbol pIVar1
// WARNING: Unable to use type for symbol uVar1

void draw_line_1020_229c(u32 param_1,u16 param_2)

{
  u32 uVar2;
  i16 x;
  i16 iVar3;
  i16 *piVar4;
  u16 uVar5;
  u16 uVar6;
  u16 x_00;
  i16 iStack10;
  i16 iVar1;
  INT16 *pIVar1;
  u32 uVar1;

  uVar5 = (param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x6);
  iVar1 = ((int)uVar1 + 0x30);
  uVar2 = (u32)((int)param_1 + 0x6);
  pIVar1 = *(INT16 **)((int)uVar2 + 0x1a);
  MoveTo16(0x5,*pIVar1,param_2);
  uVar6 = ((u32)pIVar1 >> 0x10);
  iVar3 = (int)pIVar1;
  LineTo16(0x5,*(INT16 *)(iVar3 + iVar1 * 0x8 + -0x4),param_2);
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    piVar4 = (i16 *)(iStack10 * 0x8 + iVar3);
    x = (piVar4[0x2] - *piVar4 >> 0x1) + *piVar4;
    MoveTo16(0x5,x,param_2);
    LineTo16(0xa,x,param_2);
  }
  MoveTo16(0x5f,*pIVar1,param_2);
  LineTo16(0x5f,*(INT16 *)(iVar3 + iVar1 * 0x8 + -0x4),param_2);
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    piVar4 = (i16 *)(iStack10 * 0x8 + iVar3);
    x_00 = param_2;
    MoveTo16(0x5f,(piVar4[0x2] - *piVar4 >> 0x1) + *piVar4,param_2);
    LineTo16(0x5a,x_00,param_2);
  }
  return;
}



void pass1_1020_239c(u32 param_1,i16 *param_2)

{
  u16 *puVar1;
  u32 uVar2;
  u16 uVar3;
  u8 local_a [0x6];
  u16 uStack4;

  if (param_2 != NULL) {
    uStack4 = (((int)param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
    puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_a),0x0,0x57,uStack4);
    uVar3 = (param_1 >> 0x10);
    uVar2 = pass1_1020_23f2((u8 *)((u32)puVar1 >> 0x10),param_1,uVar3,(u16 *)CONCAT22(0x1050,local_a));
    draw_polygon_1020_2474(param_1,uVar3,0x3,uVar2,(uVar2 >> 0x10));
  }
  return;
}



u32 pass1_1020_23f2(u8 *param_1,u16 param_2,u16 param_3,u16 *param_4)

{
  i16 *piVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  i16 iStack18;
  i16 local_6;
  i16 local_4;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  piVar1 = &local_6;
  pass1_1008_3e94(param_4,(u16 *)CONCAT22(0x1050,piVar1),(char *)CONCAT22(0x1050,&local_4));
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = SUB42(paVar3,0x0);
  for (iStack18 = 0x0; iStack18 < 0x3; iStack18 += 0x1) {
    piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4270) + local_4;
    piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x4272) + local_6;
  }
  return CONCAT22(uVar2,piVar1);
}



void draw_polygon_1020_2474(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  Polygon16(param_3,(POINT16 *)param_4,param_5);
  return;
}



void pass1_1020_2488(u16 param_1,u16 param_2,u32 param_3)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  INT16 in_stack_0000fff2;
  i16 iStack12;
  u16 uStack10;

  uVar5 = (param_3 >> 0x10);
  iVar4 = (int)param_3;
  find_n_load_rsrc_1010_4e9e(*(astruct_812 **)(iVar4 + 0x6));
  if ((param_2 | param_1) != 0x0) {
    uStack10 = param_1;
    for (iStack12 = 0x0; iStack12 < 0x9; iStack12 += 0x1) {
      uVar1 = (u32)(iVar4 + 0x6);
      uVar2 = pass1_1010_4f20(uVar1,((u32)uVar1 >> 0x10),iStack12);
      uVar1 = (u32)(iVar4 + 0xa);
      set_win_tet_1020_1d2a
                ((astruct_938 *)uVar1,((u32)uVar1 >> 0x10),(void *)CONCAT22(param_2,uStack10),uVar2,
                 in_stack_0000fff2);
      uVar3 = str_op_1000_3da4((char *)CONCAT22(param_2,uStack10));
      uStack10 += uVar3 + 0x1;
    }
  }
  return;
}



StructD * pass1_1020_24f2(StructD *param_1,u8 param_2)

{
  pass1_1020_1f74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1020_2524(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_20 *iVar2;
  u16 unaff_BP;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  unk_draw_op_1020_7f7a(param_2,0x7,CONCAT22(param_4,param_3),param_5);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  (u32)&iVar2[0x1].field5_0xc = 0x0;
  iVar2[0x1].field7_0x10 = NULL;
  param_2->offset_0x0 = 0x270c;
  iVar2->base_0x2 = 0x1020;
  (iVar2 + 0x1)->offset_0x0 = 0x27a8;
  iVar2[0x1].base_0x2 = 0x1020;
  puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2a),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = ((u32)puVar4 >> 0x10);
  &iVar2[0x1].field7_0x10 = (int)puVar4;
  ((int)&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  ((int)&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}



void pass1_1020_2594(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x270c;
  iVar1->address_offset_field_0x2 = 0x1020;
  &iVar1->field_0xe2 = 0x27a8;
  &iVar1->field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



void pass1_1020_25c0(u16 param_1,u16 param_2,u32 param_3)

{
  i16 *piVar1;
  code **ppcVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  astruct_277 *iVar3;
  u16 uVar5;
  astruct_57 *paStack10;
  u32 *puStack6;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar5 = (param_3 >> 0x10);
  iVar3 = (astruct_277 *)param_3;
  if (iVar3->field236_0xee != NULL) {
    ppcVar2 = (code **)((int)*iVar3->field236_0xee + 0x8);
    param_1 = (**ppcVar2)();
  }
  if (iVar3->field233_0xea == 0x0) {
    iVar3->field233_0xea = 0x1;
    mem_op_1000_179c(0x98,paVar4);
    paStack10 = (astruct_57 *)CONCAT22(paVar4,param_1);
    uVar3 = paVar4 | param_1;
    if (uVar3 == 0x0) {
      puStack6 = NULL;
    }
    else {
      piVar1 = &iVar3->field204_0xcc;
      *piVar1 = *piVar1 + 0x1;
      struct_1020_1738(paStack10,iVar3->field204_0xcc,param_3 & 0xffff | (u32)uVar5 << 0x10);
      puStack6 = (u32 *)CONCAT22(uVar3,param_1);
    }
    ppcVar2 = (code **)((int)*puStack6 + 0x8);
    (**ppcVar2)(0x1000,(int)puStack6,(int)((u32)puStack6 >> 0x10));
  }
  return;
}



void window_op_1020_2642(astruct_664 *param_1,astruct_57 *param_2,StructA *param_3)

{
  u16 uVar1;
  StructA *iVar2;
  u16 uVar2;

  create_window_ex_1008_9760(param_3);
  uVar2 = ((u32)param_3 >> 0x10);
  iVar2 = (StructA *)param_3;
  get_dc_1018_4db0(*(astruct_126 **)&iVar2[0x1].field20_0x26,iVar2->field4_0x8);
  mem_op_1000_179c(0x18,param_2);
  uVar1 = param_2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1020_27b0(uVar1,param_1,param_2,iVar2->field4_0x8);
    iVar2[0x1].field18_0x22 = (astruct_666 *)param_1;
    iVar2[0x1].field19_0x24 = uVar1;
    return;
  }
  (u32)&iVar2[0x1].field18_0x22 = 0x0;
  return;
}



void pass1_1020_26a6(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 uVar4;

  uVar4 = (param_1 >> 0x10);
  puVar1 = (u32 *)((int)param_1 + 0xee);
  uVar2 = ((int)param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1);
  return;
}



StructD * pass1_1020_26d8(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1020_2594(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1020_26e6(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1020_2594(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_27b0(u16 param_1,astruct_664 *param_2,u16 param_3,u16 param_4)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u8 *puVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  u32 *puVar6;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u32 in_stack_0000ffca;
  u16 in_stack_0000fff2;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  set_struct_op_1020_921c(param_1,(StructA *)CONCAT22(param_3,param_2),param_4,in_stack_0000ffca);
  (u32)&param_2->field16_0x14 = 0x0;
  CONCAT22(param_3,param_2) = 0x288e;
  param_2->field2_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x2a),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = ((u32)puVar6 >> 0x10);
  param_2->field16_0x14 = (int)puVar6;
  param_2->field17_0x16 = uVar3;
  param_2->field5_0x6 = param_2->field16_0x14;
  param_2->field6_0x8 = uVar3;
  uVar2 = (u32)&param_2->field16_0x14;
  puVar4 = &param_2->field_0xa;
  ppcVar1 = (code **)((int)(u32)(u32)((int)uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_2->field15_0x12 = (int)puVar4;
  draw_op_1020_9364((StructA *)CONCAT22(param_3,param_2));
  return;
}



void pass1_1020_2838(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x288e;
  iVar1->address_offset_field_0x2 = 0x1020;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}



StructD * pass1_1020_2868(StructD *param_1,u8 param_2)

{
  pass1_1020_2838(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1020_289a(u16 *param_1,u16 param_2,u32 param_3)

{
  i16 iVar1;
  u16 uVar2;

  struct_1020_790e(param_1,s_SCPOPMENU_1050_427c,param_2,param_3);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  (u32)(iVar1 + 0xf2) = 0x0;
  (u32)(iVar1 + 0xf6) = 0x0;
  (iVar1 + 0xfa) = 0x0;
  (iVar1 + 0xfc) = 0x0;
  *param_1 = 0x2e4a;
  (iVar1 + 0x2) = 0x1020;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0x5b)),s_VrMode_1050_4286);
  (u32)(iVar1 + 0xac) = 0x44c00000;
  return;
}



void pass1_1020_28fc(StructD *param_1)

{
  param_1->address_offset_field_0x0 = 0x2e4a;
  ((int)param_1 + 0x2) = 0x1020;
  cleanup_menu_ui_op_1020_795c(param_1);
  return;
}



void post_win_msg_1020_291a(u32 param_1)

{
  PostMessage16(0x0,0x0,0x10,*(HWND16 *)((int)param_1 + 0x8));
  return;
}



void pass1_1020_2936(void)

{
  pass1_1020_79ae();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_294a(u8 *param_1,astruct_665 *param_2,u32 param_3,u16 param_4)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_665 *iVar3;
  u16 unaff_BP;
  astruct_665 *uVar2;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  uVar2 = (astruct_665 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_665 *)param_2;
  iVar3->field248_0xfc = param_4;
  puVar2 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_BP,param_4),in_stack_0000fea6,in_stack_0000ffca,
                           in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = ((u32)puVar2 >> 0x10);
  iVar3->field240_0xf2 = (int)puVar2;
  iVar3->field241_0xf4 = uVar1;
  iVar3->field224_0xe0 = iVar3->field240_0xf2;
  iVar3->field225_0xe2 = uVar1;
  pass1_1018_0902((u32*)&iVar3->field240_0xf2,param_3);
  return;
}



void realize_palette_1020_2992(u32 param_1,i16 param_2)

{
  HGDIOBJ16 obj;
  HDC16 hdc;
  u16 uVar1;
  u32 *puVar1;
  code **fn_ptr_1;

  if (param_2 != 0x0) {
    uVar1 = (param_1 >> 0x10);
    puVar1 = (u32 *)pass1_1018_0a50((u32)((int)param_1 + 0xf2));
    fn_ptr_1 = (code **)((int)*puVar1 + 0x18);
    obj = (**fn_ptr_1)(0x1018);
    UnrealizeObject16(obj);
    hdc = GetDC16(*(HWND16 *)((int)param_1 + 0x8));
    RealizePalette16(hdc);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 send_msg_1020_29d8(u16 param_1,astruct_57 *param_2,astruct_69 *param_3,u16 param_4,u32 param_5,
                        u16 param_6)

{
  u8 *puVar1;
  astruct_27 *paVar2;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;
  i16 iVar3;

  iVar3 = (int)(param_5 >> 0x10);
  post_win_msg_1020_79fc(param_3,param_4,param_5,iVar3);
  paVar2 = (astruct_27 *)
           mixed_1010_20ba(param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_6,0x29),in_stack_0000fe9e,
                           in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
  puVar1 = (u8 *)((u32)paVar2 >> 0x10);
  if (iVar3 == 0x0) {
    pass1_1018_270e(puVar1,paVar2,0x1,((int)param_3 + 0xfc));
  }
  else {
    pass1_1018_270e(puVar1,paVar2,0x0,((int)param_3 + 0xfc));
    SendMessage16(0x0,0x69,0x111,HWND16_1050_0396);
  }
  return CONCAT22((int)param_2,param_1);
}



void pass1_1020_2a46(u16 param_1,u16 param_2,i16 param_3)

{
  u16 in_DX;

  pass1_1018_0ae8((u32)(param_1 + 0xf2),0x1);
  pass1_1008_68c6(in_DX,param_1,param_2,param_3);
  return;
}



void pass1_1020_2a6a(u32 param_1)

{
  get_win_ui_info_op_1020_7a50((astruct_868 *)param_1);
  pass1_1018_0ae8((u32)((int)param_1 + 0xf2),0x0);
  destroy_icon_1020_2c88((astruct_869 *)param_1);
  return;
}



void pass1_1020_2a94(u32 param_1,u32 param_2)

{
  pass1_1018_1662((u32)((int)param_1 + 0xf2),(int)param_2,(int)(param_2 >> 0x10));
  return;
}



void bring_window_to_top_1020_2aae(u32 param_1,u32 param_2)

{
  u16 uVar1;
  u16 unaff_SS;

  pass1_1008_3e0e((StructA *)param_1);
  uVar1 = (param_1 >> 0x10);
  BringWindowToTop16(*(HWND16 *)((int)param_1 + 0x8));
  pass1_1018_169e((u32)((int)param_1 + 0xf2),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void invalidate_rect_1020_2ae4(u16 param_1,u32 *param_2,u16 param_3,u16 param_4)

{
  code **ppcVar1;
  u16 uVar2;
  char cVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  astruct_57 *paVar7;
  u16 uVar8;
  u16 uVar9;
  u16 unaff_CS;
  astruct_477 *paVar10;
  u32 *puVar11;
  u32 uVar12;
  u16 in_stack_0000fe98;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  WPARAM16 wparam;
  HWND16 hwnd;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_3 != 0x129) {
    uVar8 = param_2;
    uVar9 = ((u32)param_2 >> 0x10);
    if (0x129 < param_3) {
      if (param_3 == 0x12a) {
        hwnd = *(HWND16 *)(uVar8 + 0x8);
        wparam = 0xf012;
      }
      else {
        if (param_3 == 0x12b) {
          return;
        }
        if (param_3 == 0x12c) {
          hwnd = *(HWND16 *)(uVar8 + 0x8);
          wparam = 0xf020;
        }
        else {
          if (param_3 == 0x12d) {
            return;
          }
          if (param_3 != 0x12e) {
            return;
          }
          hwnd = *(HWND16 *)(uVar8 + 0x8);
          wparam = 0xf060;
        }
      }
      PostMessage16(0x0,wparam,0x112,hwnd);
      return;
    }
    if (param_3 == 0xfb) {
      puVar11 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x30),in_stack_0000fe9c,
                                in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
      pass1_1010_375e((u32)puVar11);
      ppcVar1 = (code **)((int)*param_2 + 0x14);
      (**ppcVar1)();
      uVar12 = pass1_1010_375e((u32)puVar11);
      uVar2 = (uVar12 >> 0x10);
      pass1_1018_181c((astruct_610 *)CONCAT22((int)uVar12,uVar2),(u32)(uVar8 + 0xf2),
                      (char *)(uVar12 & 0xffff | (u32)uVar2 << 0x10));
      return;
    }
    if (param_3 < 0xfc) {
      cVar3 = (char)param_3;
      if ((u8)(cVar3 + 0x91U) == 0x0) {
        uVar5 = FUN_1010_830a(param_3 & 0xff00 | (u8)(cVar3 + 0x91U),paVar7,unaff_CS,_u16_1050_14cc,0x1f8);
        WinHelp16(0x2a,0x1,(char *)CONCAT22((int)paVar7,uVar5),*(HWND16 *)(uVar8 + 0x8));
        return;
      }
      if (cVar3 == 'r') {
        iVar4 = uVar8 + 0xa;
        uVar5 = uVar9;
        paVar10 = (astruct_477 *)
                  mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(iVar4,0x30),in_stack_0000fe98,
                                  in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
        uVar6 = ((u32)paVar10 >> 0x10);
        pass1_1010_3770(uVar6,paVar10,(char *)CONCAT22(uVar5,iVar4));
        pass1_1038_af40(uVar8,uVar6,_PTR_LOOP_1050_5b7c,(uVar8 + 0x8),0x3);
        return;
      }
      if (cVar3 == 'u') {
        pass1_1018_0a76((u32)(uVar8 + 0xf2));
        InvalidateRect16(0x0,NULL,0x0);
        return;
      }
    }
  }
  return;
}



BOOL16 enable_menu_item_1020_2c2a(u16 param_1,u16 param_2,u16 param_3,u16 param_4,HMENU16 param_5)

{
  BOOL16 BVar1;
  u16 w_flags;

  if (param_4 != 0x0) {
    return param_4 - 0x1;
  }
  EnableMenuItem16(0x400,0x3,param_5);
  if ((int)PTR_LOOP_1050_3960 < 0x2) {
    w_flags = 0x401;
  }
  else {
    w_flags = 0x400;
  }
  BVar1 = EnableMenuItem16(w_flags,0x5,param_5);
  return BVar1;
}



void pass1_1020_2c72(u32 param_1)

{
  draw_op_1020_30be(*(astruct_762 **)((int)param_1 + 0xf6));
  return;
}



void destroy_icon_1020_2c88(astruct_869 *param_1)

{
  code **ppcVar1;
  astruct_869 *struct_1;
  u16 uVar3;
  u16 uVar1;
  u32 *puVar1;

  uVar3 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_869 *)param_1;
  DestroyIcon16(struct_1->field193_0xc2);
  struct_1->field193_0xc2 = 0x0;
  struct_1->field8_0x8 = 0x0;
  puVar1 = struct_1->field241_0xf6;
  uVar1 = struct_1->field242_0xf8;
  if ((uVar1 | puVar1) != 0x0) {
    ppcVar1 = (code **)*puVar1;
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,puVar1,uVar1,0x1);
  }
  (u32)&struct_1->field241_0xf6 = 0x0;
  pass1_1010_1dda(struct_1->field240_0xf2);
  struct_1->field240_0xf2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1020_2cf0(astruct_57 *param_1,StructA *param_2,u16 param_3,u16 param_4,u16 param_5,
                        u16 param_6,u16 param_7)

{
  u32 uVar1;
  code **ppcVar2;
  astruct_160 *paVar3;
  INT16 *pIVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  astruct_57 *paVar8;
  u16 unaff_SI;
  u32 *puVar9;
  u32 uVar10;
  StructA *pSVar11;
  u16 uVar12;
  StructA *iVar10;

  pSVar11 = (StructA *)param_2;
  uVar12 = ((u32)param_2 >> 0x10);
  create_window_ex_1008_9760(param_2);
  puVar9 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,pSVar11[0x1].field26_0x30),param_3,
                           param_4,param_5,param_6);
  paVar8 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puVar9 >> 0x10);
  pSVar11[0x1].field20_0x26 = (astruct_243 *)puVar9;
  uVar5 = ((u32)puVar9 >> 0x10);
  pSVar11[0x1].field21_0x28 = uVar5;
  pSVar11[0x1].field10_0x14 = (i16)pSVar11[0x1].field20_0x26;
  pSVar11[0x1].field11_0x16 = uVar5;
  paVar3 = (astruct_160 *)LoadIcon16(s_SITEICON_1050_428d,HINSTANCE16_1050_038c);
  *(astruct_160 **)&pSVar11->field_0xc2 = paVar3;
  uVar1 = (u32)&pSVar11[0x1].field20_0x26;
  ppcVar2 = (code **)((int)*(u32*)&pSVar11[0x1].field20_0x26 + 0x30);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((u32)uVar1 >> 0x10),paVar3);
  mem_op_1000_179c(0x22,paVar8);
  uVar6 = paVar8 | paVar3;
  uVar10 = (u32)paVar8 & 0xffff0000 | (u32)uVar6;
  if (uVar6 == 0x0) {
    (u32)&pSVar11[0x1].field22_0x2a = 0x0;
  }
  else {
    load_draw_op_1020_2ede(uVar10,param_7,(astruct_40 *)CONCAT22(paVar8,paVar3),pSVar11,uVar12);
    pSVar11[0x1].field22_0x2a = paVar3;
    &pSVar11[0x1].field_0x2c = (int)uVar10;
  }
  (u32)&pSVar11[0x1].field14_0x1c = (u32)&pSVar11[0x1].field22_0x2a;
  pass1_1018_0ac0((u32)&pSVar11[0x1].field20_0x26,param_2);
  uVar10 = pass1_1018_0b08((u32)&pSVar11[0x1].field20_0x26);
  uVar7 = (uVar10 >> 0x10);
  pIVar4 = (INT16 *)uVar10;
  ppcVar2 = (code **)((int)(u32)param_2 + 0x14);
  (**ppcVar2)();
  ppcVar2 = (code **)((int)*(u32*)&pSVar11[0x1].field20_0x26 + 0x10);
  (**ppcVar2)();
  MoveWindow16(0x1,pIVar4[0x3],pIVar4[0x2],pIVar4[0x1],*pIVar4,pSVar11->field4_0x8);
  pass1_1008_3e0e(param_2);
  UpdateWindow16(pSVar11->field4_0x8);
  return;
}



StructD * pass1_1020_2e24(StructD *param_1,u8 param_2)

{
  pass1_1020_28fc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol puVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void load_draw_op_1020_2ede(astruct_57 *param_1,u16 param_2,astruct_40 *param_3,StructA *param_4,u16 param_5)

{
  HDC16 hdc_dev_ctx_1;
  StructA **ppSVar1;
  HPEN16 handle;
  HDC16 *pHVar2;
  HGDIOBJ16 h_null_brush;
  u8 *in_DX;
  astruct_40 *struct_1;
  i16 unaff_DI;
  u16 uVar5;
  u16 unaff_SS;
  u16 *puVar5;
  astruct_76 *paVar3;
  DEVMODEA *devmodea_ptr_var11;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  char *device;
  char *driver;
  u16 in_stack_0000ffea;
  u32 *puVar1;
  u32 puVar2;
  COLORREF uVar10;
  LPCSTR output;
  code **fn_ptr_1;

  get_sys_metrics_1020_7c1a(param_3,(StructA *)CONCAT22(param_5,param_4));
  uVar5 = ((u32)param_3 >> 0x10);
  struct_1 = (astruct_40 *)param_3;
  (u32)&struct_1->field17_0x14 = 0x0;
  &struct_1->field20_0x18 = 0x0;
  ((int)&struct_1->field20_0x18 + 0x2) = 0x0;
  struct_1->field21_0x1c = NULL;
  &struct_1->field_0x1e = 0x0;
  ((int)&struct_1[0x1].field0_0x0 + 0x1) = 0x0;
  param_3->field0_0x0 = 0x363c;
  struct_1->field1_0x2 = 0x1020;
  puVar5 = (u16 *)mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,param_4[0x1].field26_0x30),
                                  in_stack_0000fe92,in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
  struct_1->field17_0x14 = (int)puVar5;
  &struct_1->field_0x16 = (int)((u32)puVar5 >> 0x10);
  fn_ptr_1 = (code **)((int)*(u32*)&struct_1->field17_0x14 + 0x4);
  (**fn_ptr_1)();
  driver = s_dib_1050_42c2;
  device = NULL;
  output = NULL;
  paVar3 = (astruct_76 *)pass1_1018_0a50((u32)&struct_1->field17_0x14);
  devmodea_ptr_var11 = (DEVMODEA *)pass1_1008_4772(paVar3);
  hdc_dev_ctx_1 = CreateDC16(devmodea_ptr_var11,output,device,driver);
  *(HDC16 *)&struct_1->field20_0x18 = hdc_dev_ctx_1;
  ppSVar1 = &struct_1->field20_0x18;
  fn_ptr_1 = (code **)((int)(u32)paVar3 + 0x8);
  (**fn_ptr_1)();
  ((int)&struct_1[0x1].field0_0x0 + 0x1) = (int)ppSVar1;
  puVar2 = (u32)&struct_1->field17_0x14;
  handle = CreatePen16(*(COLORREF *)((int)puVar2 + 0x64),0x1,0x0);
  *(HPEN16 *)((int)&struct_1->field20_0x18 + 0x2) = handle;
  pHVar2 = (HDC16 *)SelectObject16(handle,*(HDC16 *)&struct_1->field20_0x18);
  struct_1->field21_0x1c = pHVar2;
  h_null_brush = GetStockObject16(HOLLOW_BRUSH);
  h_null_brush = SelectObject16(h_null_brush,*(HDC16 *)&struct_1->field20_0x18);
  *(HGDIOBJ16 *)&struct_1->field_0x1e = h_null_brush;
  return;
}



void cleanup_win_ui_1020_2fea(StructD *in_struct_1)

{
  HPALETTE16 obj;
  StructD *struct_1;
  u16 var2;
  u16 unaff_SS;

  var2 = ((u32)in_struct_1 >> 0x10);
  struct_1 = (StructD *)in_struct_1;
  in_struct_1->address_offset_field_0x0 = 0x363c;
  struct_1->address_offset_field_0x2 = 0x1020;
  if (struct_1->field12_0x14 != 0x0) {
    pass1_1010_1ea6(struct_1->field12_0x14,(StructD *)((u32)in_struct_1 & 0xffff | (u32)var2 << 0x10));
  }
  SelectObject16(*(HGDIOBJ16 *)&struct_1->field_0x1c,struct_1->field13_0x18);
  SelectObject16(*(HGDIOBJ16 *)&struct_1->field_0x1e,struct_1->field13_0x18);
  DeleteObject16(struct_1->field14_0x1a);
  obj = SelectPalette16(0x0,struct_1->field19_0x20,struct_1->field13_0x18);
  DeleteObject16(obj);
  DeleteDC16(struct_1->field13_0x18);
  in_struct_1->address_offset_field_0x0 = 0x3ab0;
  struct_1->address_offset_field_0x2 = 0x1008;
  in_struct_1->address_offset_field_0x0 = 0x389a;
  struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



void invalidate_rect_1020_3080(u32 param_1,i16 param_2)

{
  if (param_2 == 0x1) {
    (u32)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if ((param_2 != 0x4) && (param_2 != 0x6)) {
    return;
  }
  InvalidateRect16(0x0,NULL,0x0);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2

void draw_op_1020_30be(astruct_762 *struct_param_1)

{
  BOOL16 is_iconic;
  astruct_762 *iVar5;
  astruct_762 *uVar5;
  HDC16 *pHVar1;
  HDC16 *pHVar2;
  RECT16 rect_30 [0x2];
  HGDIOBJ16 hbrush_40;
  HICON16 hicon_38;
  HDC16 local_24;
  u8 local_22 [0x20];
  i16 IVar4;
  u32 uVar1;
  u32 uVar2;
  code **fn_ptr_1;

  uVar5 = (astruct_762 *)((u32)struct_param_1 >> 0x10);
  iVar5 = (astruct_762 *)struct_param_1;
  local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar5->field4_0x4);
  is_iconic = IsIconic16(iVar5->field4_0x4);
  if (is_iconic == 0x0) {
    pass1_1018_0a50(iVar5->field19_0x14);
    pHVar2 = &local_24;
    IVar4 = (i16)&DAT_1050_1050;
    fn_ptr_1 = (code **)((int)(u32)CONCAT22(0x1050,pHVar2) + 0x8);
    pHVar1 = pHVar2;
    (**fn_ptr_1)(0x1018,pHVar2,(int)&DAT_1050_1050);
    uVar2 = iVar5->field19_0x14;
    if (((int)uVar2 + 0x84) == 0x1) {
      unk_draw_op_1020_320e(struct_param_1,local_24);
    }
    fn_ptr_1 = (code **)((int)(u32)CONCAT22(IVar4,pHVar2) + 0x4);
    (**fn_ptr_1)(0x1018,pHVar2,IVar4,0x0,0x0,(char)&local_24,(int)&DAT_1050_1050,pHVar1);
    uVar1 = iVar5->field19_0x14;
    if (((int)uVar1 + 0x84) != 0x1) {
      unk_draw_op_1020_320e(struct_param_1,local_24);
    }
    draw_op_1020_3488(struct_param_1);
    fn_ptr_1 = (code **)((int)(u32)CONCAT22(IVar4,pHVar2) + 0xc);
    (**fn_ptr_1)(0x1018,pHVar2,IVar4,&local_24,(int)&DAT_1050_1050);
  }
  else if (PTR_LOOP_1050_0010 == NULL) {
    fn_ptr_1 = (code **)((int)(u32)iVar5->field19_0x14 + 0x2c);
    hicon_38 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538);
    if (hicon_38 != 0x0) {
      hbrush_40 = GetStockObject16(BLACK_BRUSH);
      GetClientRect16(rect_30,(HWND16)&DAT_1050_1050);
      FillRect16(hbrush_40,(RECT16 *)&stack0xffc4,(HDC16)&DAT_1050_1050);
      DrawIcon16(hicon_38,0x2,0x2,local_24);
    }
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar5->field4_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4

void unk_draw_op_1020_320e(astruct_762 *astruct762_param_1,HDC16 hdc_param_2)

{
  u32 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  HPALETTE16 obj;
  u16 in_DX;
  u16 DX_REG;
  astruct_762 *struct_a;
  i16 iVar7;
  u16 uVar6;
  u16 uVar7;
  DEVMODEA *uVar8;
  char *device;
  char *driver;
  i16 local_c;
  u32 local_a;
  HDC16 *pHStack6;
  HDC16 hdc_var4;
  u32 *puVar2;
  u32 uVar4;
  u8 uVar9;
  u8 uVar10;
  u16 uVar11;

  hdc_var4 = hdc_param_2;
  uVar6 = ((u32)astruct762_param_1 >> 0x10);
  struct_a = (astruct_762 *)astruct762_param_1;
  uVar4 = struct_a->field19_0x14;
  if (((int)uVar4 + 0x84) == 0x1) {
    uVar3 = struct_a->field19_0x14;
    uVar7 = (uVar3 >> 0x10);
    iVar7 = (i16)uVar3;
    puVar1 = (u32 *)(u32)(iVar7 + 0x24);
    driver = s_dib_1050_42c6;
    device = NULL;
    uVar9 = '\0';
    uVar10 = '\0';
    uVar11 = 0x0;
    uVar8 = (DEVMODEA *)pass1_1008_4772((astruct_76 *)((u32)puVar1 & 0xffff | (u32)(iVar7 + 0x26) << 0x10))
    ;
    hdc_var4 = CreateDC16(uVar8,(char *)CONCAT22(uVar11,CONCAT11(uVar10,uVar9)),device,driver);
    pHStack6 = &hdc_var4;
    ppcVar2 = (code **)((int)*puVar1 + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538);
    in_DX = DX_REG;
  }
  pass1_1018_0d9a(struct_a->field19_0x14,(u16 *)CONCAT22(0x1050,&local_c),(u32 *)CONCAT22(0x1050,&local_a));
  uVar3 = struct_a->field19_0x14;
  draw_op_1020_33c0(in_DX,(u32)astruct762_param_1,*(COLORREF *)((int)uVar3 + 0x6c),local_c,local_a,0x1,hdc_var4);
  pass1_1018_1054(struct_a->field19_0x14,(u16 *)CONCAT22(0x1050,&local_c),(u32 *)CONCAT22(0x1050,&local_a));
  uVar3 = struct_a->field19_0x14;
  draw_op_1020_33c0(in_DX,(u32)astruct762_param_1,*(COLORREF *)((int)uVar3 + 0x74),local_c,local_a,0x2,hdc_var4);
  pass1_1018_1320(struct_a->field19_0x14,(u16 *)CONCAT22(0x1050,&local_c),(u32 *)CONCAT22(0x1050,&local_a));
  uVar3 = struct_a->field19_0x14;
  draw_op_1020_33c0(in_DX,(u32)astruct762_param_1,*(COLORREF *)((int)uVar3 + 0x68),local_c,local_a,0x1,hdc_var4);
  pass1_1018_15f6(struct_a->field19_0x14,(u16 *)CONCAT22(0x1050,&local_c),(u32 *)CONCAT22(0x1050,&local_a));
  if (local_c != 0x0) {
    uVar3 = struct_a->field19_0x14;
    draw_op_1020_33c0(in_DX,(u32)astruct762_param_1,*(COLORREF *)((int)uVar3 + 0x70),local_c,local_a,0x1,hdc_var4);
  }
  pass1_1018_108c(struct_a->field19_0x14,(u16 *)CONCAT22(0x1050,&local_c),(u32 *)CONCAT22(0x1050,&local_a));
  if (local_c != 0x0) {
    uVar3 = struct_a->field19_0x14;
    draw_op_1020_33c0(in_DX,(u32)astruct762_param_1,*(COLORREF *)((int)uVar3 + 0x78),local_c,local_a,0x0,hdc_var4);
  }
  uVar3 = struct_a->field19_0x14;
  if (((int)uVar3 + 0x84) == 0x1) {
    obj = SelectPalette16(0x0,(HPALETTE16)pHStack6,hdc_var4);
    DeleteObject16(obj);
    DeleteDC16(hdc_var4);
  }
  return;
}



void draw_op_1020_33c0(u16 param_1,u32 param_2,COLORREF colorref_param_2,i16 param_4,u32 param_5,i16 param_6,
                      HDC16 hdc16_param_6)

{
  HPEN16 pen_handle;
  HGDIOBJ16 object_handle;
  HBRUSH16 brush_handle;
  HGDIOBJ16 obj_handle_2;
  u16 uVar1;
  u16 uVar2;
  u16 in_DX;
  u16 uVar3;
  u16 uVar4;
  u16 unaff_SS;
  u16 uVar6;
  HDC16 HVar7;
  i16 iStack20;
  u16 *puStack14;
  u16 uVar5;

  if (param_4 != 0x0) {
    HVar7 = hdc16_param_6;
    pen_handle = CreatePen16(colorref_param_2,0x1,0x0);
    object_handle = SelectObject16(pen_handle,HVar7);
    HVar7 = hdc16_param_6;
    brush_handle = CreateSolidBrush16(colorref_param_2);
    obj_handle_2 = SelectObject16(brush_handle,HVar7);
    puStack14 = (u16 *)param_5;
    for (iStack20 = 0x0; iStack20 < param_4; iStack20 += 0x1) {
      uVar6 = (param_2 >> 0x10);
      uVar1 = param_4;
      pass1_1020_3540(param_1,param_2,uVar6,param_6,puStack14);
      if (param_6 < 0x1) {
        uVar2 = 0x3;
      }
      else {
        uVar2 = 0x4;
      }
      uVar3 = param_1;
      draw_polygon_1020_3602(param_2,uVar6,uVar2,uVar1,param_1);
      fn_ptr_1000_17ce((char *)CONCAT22(param_1,uVar1));
      puStack14 = (u16 *)((u32)puStack14 & 0xffff0000 | (u32)((int)puStack14 + 0x6));
      param_1 = uVar3;
    }
    obj_handle_2 = SelectObject16(obj_handle_2,hdc16_param_6);
    DeleteObject16(obj_handle_2);
    SelectObject16(object_handle,hdc16_param_6);
    DeleteObject16(pen_handle);
  }
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar3

void draw_op_1020_3488(astruct_762 *param_1)

{
  u32 uVar6;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  HGDIOBJ16 obj_handle_7;
  u16 uVar5;
  u16 unaff_SS;
  HDC16 in_stack_00000008;
  HDC16 hdc;
  u32 local_a;
  u16 *puStack6;
  u16 uVar1;
  u32 uVar2;
  u32 uVar3;
  u32 uVar4;
  i16 left;
  HDC16 hdc16_ffe2;

  uVar5 = ((u32)param_1 >> 0x10);
  uVar2 = (u32)((int)param_1 + 0x14);
  puStack6 = (u16 *)(uVar2 & 0xffff0000 | (u32)((int)uVar2 + 0x36));
  pass1_1008_3e94(puStack6,(u16 *)CONCAT22(0x1050,&local_a),(char *)CONCAT22(0x1050,(int)&local_a + 0x2));
  uVar4 = (u32)(local_a - 0x3U) << 0x10;
  if ((int)(local_a - 0x3U) < 0x0) {
    uVar4 = 0x0;
  }
  uVar1 = (int)local_a - 0x3;
  uVar6 = (u32)uVar1;
  if ((int)uVar1 < 0x0) {
    uVar6 = 0x0;
  }
  local_a = uVar4 | uVar6;
  uVar3 = (u32)((int)param_1 + 0x14);
  hdc = in_stack_00000008;
  handle = CreatePen16(*(COLORREF *)((int)uVar3 + 0x64),0x1,0x0);
  handle_00 = SelectObject16(handle,hdc);
  obj_handle_7 = GetStockObject16(HOLLOW_BRUSH);
  obj_handle_7 = SelectObject16(obj_handle_7,hdc16_ffe2);
  left = (i16)(local_a >> 0x10);
  Rectangle16((int)local_a + 0x6,left + 0x6,(int)local_a,left,in_stack_00000008);
  SelectObject16(handle_00,in_stack_00000008);
  SelectObject16(obj_handle_7,in_stack_00000008);
  DeleteObject16(handle);
  return;
}



void pass1_1020_3540(u16 param_1,u16 param_2,u16 param_3,i16 param_4,u16 *param_5)

{
  i16 iVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_279 *iVar2;
  i16 iStack18;
  i16 iStack12;
  i16 iStack10;
  i16 local_6;
  i16 local_4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(0x1050,&local_6),(char *)CONCAT22(0x1050,&local_4));
  if (param_4 == 0x0) {
    iStack12 = 0x3;
    iStack10 = 0x42a6;
  }
  else if (param_4 == 0x1) {
    iStack12 = 0x4;
    iStack10 = (int)s_SITEICON_1050_428d + 0x9;
  }
  else {
    if (param_4 != 0x2) {
      return;
    }
    iStack12 = 0x4;
    iStack10 = 0x42b2;
  }
  iVar1 = iStack12 << 0x2;
  mem_op_1000_179c(iVar1,paVar2);
  for (iStack18 = 0x0; iStack18 < iStack12; iStack18 += 0x1) {
    iVar2 = (astruct_279 *)(iStack18 * 0x4);
    (iVar2 + iVar1) = (iVar2 + iStack10) + local_4;
    (iVar2 + iVar1 + 0x2) = (iVar2 + iStack10 + 0x2) + local_6;
  }
  return;
}



void draw_polygon_1020_3602(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  Polygon16(param_3,(POINT16 *)param_4,param_5);
  return;
}



StructD * pass1_1020_3616(StructD *param_1,u8 param_2)

{
  cleanup_win_ui_1020_2fea(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1020_3644(u32 param_1,StructA *param_2,u16 param_3,u32 param_4,u16 param_5,
                     u16 param_6)

{
  StructA *iVar2;
  u16 in_buf_len_5;
  u16 in_stack_0000fe52;
  u16 in_stack_0000ff76;
  u16 in_stack_0000ff7c;
  u16 in_stack_0000ff80;
  astruct_270 *iVar1;

  struct_1020_790e(&param_2->field0_0x0,NULL,param_3,param_4);
  in_buf_len_5 = ((u32)param_2 >> 0x10);
  iVar2 = (StructA *)param_2;
  iVar2[0x1].field20_0x26 = (astruct_243 *)0x389a;
  iVar2[0x1].field21_0x28 = 0x1008;
  iVar2[0x1].field20_0x26 = (astruct_243 *)0x3aa8;
  iVar2[0x1].field21_0x28 = 0x1008;
  iVar2[0x1].field29_0x34 = 0x0;
  iVar2[0x1].field37_0x3e = 0x0;
  iVar2[0x1].field38_0x42 = 0x0;
  param_2->field0_0x0 = 0x3d08;
  iVar2->field1_0x2 = 0x1020;
  iVar2[0x1].field20_0x26 = (astruct_243 *)0x3d9c;
  iVar2[0x1].field21_0x28 = 0x1020;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)&iVar2->field5_0xa,
             in_buf_len_5);
  unk_str_op_1000_3d3e((char *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar2->field60_0x5b)),s_VrMode_1050_42ca);
  iVar2->field140_0xac = 0x44c00000;
  window_op_1020_38aa(param_1,(StructA *)((u32)param_2 & 0xffff | (u32)in_buf_len_5 << 0x10),param_6,param_5,
                      in_stack_0000ff7c,in_stack_0000ff80,in_stack_0000fe52,in_stack_0000ff76);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1020_36f6(u32 param_1,i16 param_2)

{
  i16 iVar1;
  code **ppcVar2;
  u32 uVar3;
  char *pcVar4;
  u16 uVar5;
  HWND16 HVar6;
  u16 uVar7;
  u16 uVar8;
  i16 iVar9;
  u16 uVar10;
  char *pcVar11;
  u8 uVar12;
  u8 uVar13;
  u8 *puStack1034;
  char local_406 [0x400];
  u32 uStack6;

  iVar9 = (int)param_1;
  uVar10 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (u32)(iVar9 + 0x8) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  uStack6 = pass1_1018_1e78((u32)(iVar9 + 0x8),-0x1);
  uVar7 = (uStack6 >> 0x10);
  uVar3 = (u32)(iVar9 + 0x18);
  GetWindowText16(0x3ff,CONCAT22(0x1050,local_406),*(HWND16 *)((int)uVar3 + 0x6));
  pcVar4 = pass1_1000_472c(CONCAT22(0x1050,local_406),':');
  puStack1034 = (u8 *)CONCAT22(uVar7,pcVar4 + 0x2);
  *puStack1034 = 0x0;
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_406,(short)&DAT_1050_1050);
  uVar3 = (u32)(iVar9 + 0x18);
  ppcVar2 = (code **)((int)(u32)(u32)(iVar9 + 0x18) + 0x18);
  (**ppcVar2)(0x1010,(int)uVar3,(int)((u32)uVar3 >> 0x10),local_406,(int)&DAT_1050_1050);
  uVar3 = (u32)(iVar9 + 0x8);
  iVar1 = ((int)uVar3 + 0x4a);
  uVar3 = (u32)(iVar9 + 0x18);
  HVar6 = *(HWND16 *)((int)uVar3 + 0x6);
  SetDlgItemText16((u32)((int)uStack6 + 0x2),0x10f,HVar6);
  SetDlgItemText16((u32)((int)uStack6 + 0xa),0x110,HVar6);
  SetDlgItemText16((u32)((int)uStack6 + 0x12),0x112,HVar6);
  SetDlgItemText16((u32)((int)uStack6 + 0xe),0x113,HVar6);
  if (iVar1 != 0x0) {
    uVar5 = pass1_1018_1f1a((u32)(iVar9 + 0x8),((int)uStack6 + 0x1a));
    if (uVar5 != 0x0) {
      uVar12 = 0x11;
      uVar13 = 0x1;
      uVar3 = (u32)((int)uStack6 + 0x16);
      uVar7 = uVar3;
      uVar8 = ((u32)uVar3 >> 0x10);
      goto LAB_1020_3846;
    }
  }
  uVar12 = 0x11;
  uVar13 = 0x1;
  pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x5d9);
  uVar8 = ((u32)pcVar11 >> 0x10);
  uVar7 = SUB42(pcVar11,0x0);
LAB_1020_3846:
  SetDlgItemText16(CONCAT22(uVar8,uVar7),CONCAT11(uVar13,uVar12),HVar6);
  if (*(i32 *)(iVar9 + 0x1c) != 0x0) {
    uVar3 = (u32)(iVar9 + 0x1c);
    HVar6 = GetDlgItem16(*(INT16 *)((int)uStack6 + 0x1a),*(HWND16 *)((int)uVar3 + 0x6));
    SetFocus16(HVar6);
    return;
  }
  InvalidateRect16(0x0,NULL,0x0);
  return;
}



void pass1_1020_3898(astruct_30 *param_1)

{
  destroy_window_1020_3b3e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void window_op_1020_38aa(astruct_57 *param_1,StructA *param_2,u16 param_3,u16 param_4,u16 param_5,
                        u16 param_6,u16 param_7,u16 param_8)

{
  HWND16 hwnd;
  code **ppcVar1;
  u32 uVar2;
  astruct_126 *paVar3;
  astruct_57 *paVar4;
  u32 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  astruct_57 *paVar9;
  u32 uVar11;
  u16 unaff_SI;
  u16 uVar12;
  u32 *puVar13;
  u16 in_stack_0000fe08;
  u16 in_stack_0000fe0c;
  u16 in_stack_0000ff32;
  u16 in_stack_0000ff36;
  u16 in_stack_0000ff3a;
  u16 uVar14;
  i16 local_12;
  i16 iStack16;
  i16 iStack14;
  i16 iStack12;
  i16 local_a [0x2];
  i16 iStack6;
  i16 iStack4;
  StructA *pstructa_hi;
  StructA *pstructa_1;
  astruct_57 *paVar10;

  pstructa_1 = (StructA *)param_2;
  uVar14 = ((u32)param_2 >> 0x10);
  create_window_ex_1008_9760(param_2);
  puVar13 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x6),param_7,param_8,param_5,param_6)
  ;
  paVar9 = (astruct_57 *)((u32)param_1 & 0xffff0000);
  pstructa_1[0x1].field25_0x2e = (i16)puVar13;
  iVar6 = (i16)((u32)puVar13 >> 0x10);
  pstructa_1[0x1].field26_0x30 = iVar6;
  pstructa_1[0x1].field10_0x14 = pstructa_1[0x1].field25_0x2e;
  pstructa_1[0x1].field11_0x16 = iVar6;
  if (param_2 == NULL) {
    paVar3 = NULL;
  }
  else {
    paVar9 = (astruct_57 *)((u32)paVar9 | (u32)uVar14);
    paVar3 = (astruct_126 *)&pstructa_1[0x1].field20_0x26;
  }
  ppcVar1 = (code **)((int)*(u32*)&pstructa_1[0x1].field25_0x2e + 0x4);
  (**ppcVar1)(0x1010,(u32)&pstructa_1[0x1].field25_0x2e,0x0,paVar3,(int)paVar9);
  pass1_1018_1a8e((u8 *)paVar9,*(astruct_653 **)&pstructa_1[0x1].field25_0x2e);
  mem_op_1000_179c(0x20,paVar9);
  uVar7 = paVar9 | paVar3;
  paVar10 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar7);
  if (uVar7 == 0x0) {
    (u32)&pstructa_1[0x1].field22_0x2a = 0x0;
  }
  else {
    unk_draw_op_1020_3da4(uVar7,param_3,(astruct_24 *)CONCAT22(paVar9,paVar3),param_2);
    pstructa_1[0x1].field22_0x2a = (astruct_160 *)paVar3;
    &pstructa_1[0x1].field_0x2c = (int)paVar10;
  }
  uVar5 = (u32)&pstructa_1[0x1].field22_0x2a;
  (u32)&pstructa_1[0x1].field14_0x1c = uVar5;
  mem_op_1000_179c(0x42,paVar10);
  paVar4 = (astruct_57 *)uVar5;
  uVar7 = (astruct_57 *)paVar10 | paVar4;
  paVar9 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar7);
  if (uVar7 == 0x0) {
    (u32)&pstructa_1[0x1].field_0x36 = 0x0;
  }
  else {
    pass1_1008_3bd6((u32)paVar9,paVar4,(astruct_57 *)paVar10,0x0,0xf1,0x0,0x1ac01ad,
                    CONCAT22(pstructa_1->field4_0x8,0xf4),param_4,in_stack_0000fe08,in_stack_0000fe0c,in_stack_0000ff32,
                    in_stack_0000ff36,in_stack_0000ff3a);
    *(astruct_57 **)&pstructa_1[0x1].field_0x36 = paVar4;
    pstructa_1[0x1].field32_0x38 = (u8 *)paVar9;
  }
  uVar12 = 0x1000;
  mem_op_1000_179c(0x42,paVar9);
  uVar7 = (astruct_57 *)paVar9 | paVar4;
  uVar11 = (u32)paVar9 & 0xffff0000 | (u32)uVar7;
  if (uVar7 == 0x0) {
    (u32)&pstructa_1[0x1].field_0x3a = 0x0;
  }
  else {
    uVar12 = 0x1008;
    pass1_1008_3bd6(uVar11,paVar4,(astruct_57 *)paVar9,0x0,0xf500f1,0x0,0x1ae01af,CONCAT22(pstructa_1->field4_0x8,0xf5),
                    param_4,in_stack_0000fe08,in_stack_0000fe0c,in_stack_0000ff32,in_stack_0000ff36,in_stack_0000ff3a);
    *(astruct_57 **)&pstructa_1[0x1].field_0x3a = paVar4;
    &pstructa_1[0x1].field_0x3c = (int)uVar11;
  }
  uVar5 = (u32)&pstructa_1[0x1].field25_0x2e;
  ppcVar1 = (code **)((int)*(u32*)&pstructa_1[0x1].field25_0x2e + 0x10);
  (**ppcVar1)(uVar12,(char)uVar5,(int)((u32)uVar5 >> 0x10));
  uVar12 = uVar11;
  uVar7 = paVar4->field1_0x2;
  paVar9 = (astruct_57 *)(uVar11 & 0xffff0000 | (u32)uVar7);
  uVar7 = MoveWindow16(0x1,paVar4->field3_0x6,paVar4->field2_0x4,uVar7,paVar4->field0_0x0,pstructa_1->field4_0x8);
  uVar12 = 0x1000;
  mem_op_1000_179c(0x8e,paVar9);
  uVar8 = paVar9 | uVar7;
  if (uVar8 == 0x0) {
    pstructa_1[0x1].field37_0x3e = 0x0;
  }
  else {
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    get_sys_metrics_1040_7728((astruct_57 *)CONCAT22(paVar9,uVar7),0x1,0x0,0xfc0,pstructa_1->field4_0x8);
    &pstructa_1[0x1].field37_0x3e = uVar7;
    ((int)&pstructa_1[0x1].field37_0x3e + 0x2) = uVar8;
  }
  uVar2 = pstructa_1[0x1].field37_0x3e;
  ((int)uVar2 + 0x74) = 0x1;
  uVar2 = pstructa_1[0x1].field37_0x3e;
  ppcVar1 = (code **)((int)(u32)pstructa_1[0x1].field37_0x3e + 0x8);
  (**ppcVar1)(uVar12,(char)uVar2,(int)(uVar2 >> 0x10));
  if ((((int)&pstructa_1[0x1].field37_0x3e + 0x2) | &pstructa_1[0x1].field37_0x3e) != 0x0) {
    uVar2 = pstructa_1[0x1].field37_0x3e;
    hwnd = *(HWND16 *)((int)uVar2 + 0x6);
    GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,local_a)),pstructa_1->field4_0x8);
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_12),hwnd);
    iStack12 -= iStack16;
    iStack14 = iStack6 - local_a[0];
    local_12 = local_a[0];
    iStack16 = iStack4 + 0x3;
    SetWindowPos16(0x44,iStack12,iStack14,iStack16,local_a[0],0x0,hwnd);
  }
  return;
}



void destroy_window_1020_3b3e(astruct_30 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i32 lVar4;
  u8 *puVar5;
  astruct_30 *paVar6;
  astruct_30 *struct_5;
  astruct_30 *struct_6;
  u16 unaff_CS;

  struct_6 = (astruct_30 *)((u32)param_1 >> 0x10);
  struct_5 = (astruct_30 *)param_1;
  struct_5->field262_0x10e = 0x0;
  if (struct_5->field261_0x10a != 0x0) {
    lVar4 = struct_5->field261_0x10a;
    // 0x1538
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    DestroyWindow16(*(HWND16 *)((int)lVar4 + 0x6));
  }
  puVar1 = struct_5->field246_0xf6;
  uVar2 = struct_5->field247_0xf8;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(unaff_CS,puVar1);
  }
  (u32)&struct_5->field246_0xf6 = 0x0;
  if (struct_5->field248_0xfa != 0x0) {
    puVar5 = (u8 *)(struct_6 | struct_5);
    if (param_1 == NULL) {
      paVar6 = NULL;
    }
    else {
      puVar5 = &struct_5->field_0xf2;
      paVar6 = struct_6;
    }
    pass1_1010_1ea6(struct_5->field248_0xfa,(StructD *)CONCAT22(paVar6,puVar5));
  }
  struct_5->field248_0xfa = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_3bd6(u16 param_1,u32 param_2)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;

  uVar3 = (param_2 >> 0x10);
  uVar2 = param_2;
  mixed_draw_op_1020_3fa0((u32)(uVar2 + 0xf6));
  if ((uVar2 + 0x100) == 0x0) {
    (uVar2 + 0x100) = 0x1;
    uVar4 = (u32)(uVar2 + 0xfa);
    if (((int)uVar4 + 0x56) == 0x0) {
      iVar1 = 0x5;
    }
    else {
      iVar1 = 0x8;
    }
    uVar4 = pass1_1038_af40(uVar2,param_1,_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),iVar1);
    (uVar2 + 0x10e) = (int)uVar4;
    (uVar2 + 0x110) = (int)((u32)uVar4 >> 0x10);
  }
  return;
}



void pass1_1020_3c32(i16 param_1,u16 param_2,u16 param_3)

{
  char cVar1;
  i16 iVar2;

  if (param_3 == 0xf5) {
    iVar2 = 0x1;
LAB_1020_3c52:
    pass1_1018_1b02(*(astruct_95 **)(param_1 + 0xfa),iVar2);
    return;
  }
  if ((param_3 < 0xf6) && (cVar1 = (char)param_3, cVar1 != '\0')) {
    if (cVar1 == '\x01' || cVar1 == '\x02') {
      return;
    }
    if (cVar1 == -0xc) {
      iVar2 = 0x0;
      goto LAB_1020_3c52;
    }
  }
  pass1_1020_3c32(param_1,param_2,param_3);
  return;
}



void pass1_1020_3c74(u16 param_1,u32 param_2,u16 param_3)

{
  pass1_1020_3c8c(CONCAT22((int)param_2,param_1),CONCAT22(param_3,(int)(param_2 >> 0x10)));
  return;
}



void pass1_1020_3c8c(u32 param_1,u32 param_2)

{
  pt_in_rect_1018_1bda((u32)((int)param_1 + 0xfa),param_2,(param_2 >> 0x10));
  return;
}



StructD * pass1_1020_3ca6(StructD *param_1,u8 param_2)

{
  u32 uVar1;
  StructD *puStack10;

  uVar1 = (u32)param_1 & 0xffff0000;
  param_1 = (StructD *)(uVar1 | (int)param_1 - 0xf2);
  param_1 = (uVar1 >> 0x10);
  if (param_1 == NULL) {
    param_1._0_2_ = 0x0;
    param_1 = 0x0;
  }
  puStack10 = (StructD *)CONCAT22(param_1,(int)param_1);
  puStack10->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void FUN_1020_3cb4(void)

{
  return;
}



StructD * FUN_1020_3cb8(u16 param_1,StructD *param_2,u8 param_3)

{
  i16 iVar1;
  u16 *puStack10;

  if (param_2 == NULL) {
    iVar1 = 0x0;
    param_2 = 0x0;
  }
  else {
    iVar1 = (int)param_2 + 0xf2;
  }
  puStack10 = (u16 *)CONCAT22(param_2,iVar1);
  *puStack10 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
//
// WARNING: Variable defined which should be unmapped: param_16
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_3d08(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6,
                    u16 param_7,u16 param_8,u16 param_9,u8 param_10,u8 param_11,u8 param_12,
                    u8 param_13,u8 param_14,u32 param_15,u16 param_16,u16 param_17,u16 param_18,
                    u16 param_19)

{
  char *pcVar1;
  u8 *pbVar2;
  bool bVar3;
  bool bVar4;
  code **ppcVar5;
  u16 *puVar6;
  u32 uVar7;
  u32 *puVar8;
  HDC16 hdc;
  StructA *pSVar9;
  u8 bVar10;
  u8 bVar12;
  i16 iVar13;
  u8 bVar17;
  char cVar18;
  HDC16 hdc_00;
  int16_t iVar14;
  HGDIOBJ16 HVar15;
  u8 *puVar16;
  u16 uVar19;
  u8 bVar20;
  u8 bVar21;
  u16 uVar22;
  u16 in_register_0000000a;
  u8 bVar24;
  char *pcVar25;
  u16 uVar26;
  u16 uVar27;
  bool bVar28;
  bool bVar29;
  u32 *puVar30;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  u16 uStack30;
  code *pcStack4;
  u8 bVar11;
  astruct_57 *paVar23;

  pSVar9 = (StructA *)CONCAT22(param_19,param_18);
  bVar20 = (char)param_2 + (char)(param_1 >> 0x8) + param_10;
  *(u8 *)param_6 = 0x3c;
  paVar23 = (astruct_57 *)
            CONCAT22(in_register_0000000a,
                     CONCAT11((char)(param_2 >> 0x8) + '<' + (*(u8 *)(param_3 + param_5) < 0x20),bVar20));
  pcStack4 = caseD_a7;
  iVar13 = 0x203d;
  pbVar2 = (u8 *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  pbVar2 = (u8 *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 & bVar20;
  pcVar25 = (char *)CONCAT11(0x79,(char)param_5 + -0x37);
  pbVar2 = (u8 *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  pbVar2 = (u8 *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  pbVar2 = (u8 *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  bVar10 = (u8)(param_6 + 0x2);
  bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
  bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
  pbVar2 = (u8 *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  bVar10 = 0x9 < bVar11 | bVar12;
  uVar19 = CONCAT11((char)(param_6 + 0x2 >> 0x8) + bVar12 + bVar10,bVar11 + bVar10 * '\x06') & 0xff0f;
  do {
    pbVar2 = (u8 *)(param_3 + iVar13);
    bVar21 = (u8)paVar23;
    *pbVar2 = *pbVar2 | bVar21;
    bVar12 = (u8)(uVar19 - 0x1);
    bVar3 = 0x9 < (bVar12 & 0xf);
    bVar20 = bVar3 | bVar10;
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar21;
    bVar4 = 0x9 < (bVar12 + bVar20 * '\x06' & 0xf);
    bVar17 = (char)(uVar19 - 0x1 >> 0x8) + bVar20 + (bVar4 | bVar20);
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar21;
    uVar19 = 0x0;
    bVar28 = &pcStack4 < (u8 *)(param_3 + iVar13);
    pbVar2 = (u8 *)(param_3 + iVar13 + 0x896);
    bVar24 = (u8)param_3;
    bVar29 = CARRY1(*pbVar2,bVar24) || CARRY1(*pbVar2 + bVar24,bVar28);
    *pbVar2 = *pbVar2 + bVar24 + bVar28;
    pbVar2 = (u8 *)(param_3 + iVar13 + 0x2038);
    bVar12 = *pbVar2;
    bVar11 = *pbVar2;
    *pbVar2 = bVar11 + bVar24 + bVar29;
    pcVar1 = (char *)(param_4 + iVar13);
    *pcVar1 = *pcVar1 + (char)((u32)paVar23 >> 0x8) + (CARRY1(bVar12,bVar24) || CARRY1(bVar11 + bVar24,bVar29));
    pcVar1 = (char *)(param_3 + iVar13 + -0x64);
    *pcVar1 = *pcVar1 + bVar17 + '\x01';
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar21;
    pcVar1 = pcVar25;
    pcVar25 = pcVar25 + 0x1;
    bVar28 = *pcVar1 != '\0';
    if (-*pcVar1 < '\0') {
      pcVar1 = (char *)(param_4 + 0x37);
      *pcVar1 = *pcVar1 + bVar24 + bVar28;
      pbVar2 = (u8 *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar21;
      iVar13 += -0x1;
      pbVar2 = (u8 *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar21;
      pbVar2 = (u8 *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar21;
      uVar22 = (int)paVar23 - 0x1;
      paVar23 = (astruct_57 *)((u32)paVar23 & 0xffff0000 | (u32)uVar22);
      pbVar2 = (u8 *)(param_3 + iVar13);
      bVar12 = (u8)uVar22;
      *pbVar2 = *pbVar2 | bVar12;
      if (*pbVar2 == 0x0) {
        pbVar2 = (u8 *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar12;
code_r0x10203d96:
        pbVar2 = (u8 *)(param_3 + iVar13);
        bVar12 = (u8)paVar23;
        *pbVar2 = *pbVar2 | bVar12;
        pbVar2 = (u8 *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar12;
        paVar23 = (astruct_57 *)
                  ((u32)paVar23 & 0xffff0000 |
                  (u32)CONCAT11((char)((u32)paVar23 >> 0x8) * '\x02' + ((u8)uVar19 < 0x20),bVar12));
        pbVar2 = (u8 *)(param_3 + iVar13 + 0x1);
        *pbVar2 = *pbVar2 & bVar12;
        param_4 = &stack0xfff6;
        param_16 = pcStack4;
        param_17 = ((u32)pcStack4 >> 0x10);
        pSVar9 = (StructA *)param_15;
code_r0x10203db1:
        get_sys_metrics_1020_7c1a((astruct_40 *)CONCAT22(param_17,param_16),pSVar9);
        puVar6 = (u16 *)(u32)(param_4 + 0x6);
        uVar26 = ((u32)puVar6 >> 0x10);
        (u32)((int)puVar6 + 0x14) = 0x0;
        *puVar6 = 0x408a;
        ((int)puVar6 + 0x2) = 0x1020;
        puVar30 = mixed_1010_20ba(paVar23,_u16_1050_0ed0,(u8 **)CONCAT22(uStack30,0x6),in_stack_0000fe8a,
                                  in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
        hdc = (HDC16)((u32)puVar30 >> 0x10);
        uVar7 = (u32)(param_4 + 0x6);
        uVar26 = ((u32)uVar7 >> 0x10);
        iVar13 = (int)uVar7;
        (iVar13 + 0x14) = (int)puVar30;
        *(HDC16 *)(iVar13 + 0x16) = hdc;
        ppcVar5 = (code **)((int)(u32)(u32)(iVar13 + 0x14) + 0x4);
        (**ppcVar5)(0x1010,(iVar13 + 0x14),hdc,0x0,iVar13,uVar26);
        uVar7 = (u32)(param_4 + 0x6);
        hdc_00 = GetDC16(*(HWND16 *)((int)uVar7 + 0x4));
        *(HDC16 *)(param_4 + -0x2) = hdc_00;
        iVar14 = SetMapMode16(0x1,hdc_00);
        uVar7 = (u32)(param_4 + 0x6);
        *(int16_t *)((int)uVar7 + 0x1e) = iVar14;
        HVar15 = GetStockObject16(0x0);
        HVar15 = SelectObject16(HVar15,hdc);
        uVar7 = (u32)(param_4 + 0x6);
        *(HGDIOBJ16 *)((int)uVar7 + 0x18) = HVar15;
        HVar15 = GetStockObject16(0x6);
        HVar15 = SelectObject16(HVar15,0x0);
        uVar7 = (u32)(param_4 + 0x6);
        *(HGDIOBJ16 *)((int)uVar7 + 0x1a) = HVar15;
        uVar7 = (u32)(param_4 + 0x6);
        uVar7 = (u32)((int)uVar7 + 0x14);
        (u32)(param_4 + -0x6) = (u32)((int)uVar7 + 0x24);
        puVar16 = (u8 *)(param_4 + -0x2);
        puVar8 = (u32 *)(u32)(param_4 + -0x6);
        ppcVar5 = (code **)((int)*puVar8 + 0x8);
        (**ppcVar5)((int)s_tile2_bmp_1050_1538,(int)puVar8,(int)((u32)puVar8 >> 0x10),puVar16,(int)&DAT_1050_1050);
        uVar7 = (u32)(param_4 + 0x6);
        uVar27 = ((u32)uVar7 >> 0x10);
        iVar13 = (int)uVar7;
        (iVar13 + 0x1c) = (int)puVar16;
        uVar26 = (param_4 + -0x2);
        (param_4 + -0x14) = uVar26;
        uVar7 = (u32)(iVar13 + 0x14);
        ((int)uVar7 + 0x4c) = uVar26;
        return;
      }
      pbVar2 = (u8 *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 & bVar12;
      pcVar1 = (char *)(param_4 + iVar13 + 0x20);
      bVar11 = (u8)param_1 & 0x1f;
      cVar18 = *pcVar1;
      *pcVar1 = *pcVar1 >> bVar11;
      pcVar1 = (char *)(param_4 + iVar13 + 0x6a);
      *pcVar1 = *pcVar1 + (u8)param_1 + ((param_1 & 0x1f) != 0x0 && (cVar18 >> bVar11 - 0x1 & 0x1U) != 0x0);
      pbVar2 = (u8 *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar12;
      uVar19 = (param_3 + iVar13) * 0x10;
      pbVar2 = (u8 *)(pcVar25 + param_4 + 0x8);
      bVar12 = (u8)(uVar19 >> 0x8);
      uVar22 = uVar19 & 0xff | (u8)(bVar12 + *pbVar2) << 0x8;
      pcVar1 = (char *)(param_4 + iVar13 + 0x37);
      *pcVar1 = *pcVar1 + (char)(param_3 >> 0x8) + CARRY1(bVar12,*pbVar2);
    }
    else {
      pcVar1 = (char *)(param_4 + iVar13);
      *pcVar1 = *pcVar1 + bVar28;
      uVar22 = (param_3 + iVar13) * 0x10;
      if ((POPCOUNT(*pcVar1) & 0x1U) == 0x0) goto code_r0x10203db1;
    }
    pbVar2 = (u8 *)(param_3 + iVar13);
    bVar11 = (u8)paVar23;
    *pbVar2 = *pbVar2 | bVar11;
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_16 = (param_14 & 0x1) * 0x4000 | (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
               ((char)*pbVar2 < '\0') * 0x80 | (*pbVar2 == 0x0) * 0x40 |
               (u8)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 | ((POPCOUNT(*pbVar2) & 0x1U) == 0x0) * 0x4;
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    bVar12 = in(0x79);
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 & bVar11;
    if (-0x1 < (char)*pbVar2) {
      pSVar9 = (StructA *)CONCAT22(param_19,param_18);
      if ((bVar17 | *(u8 *)(param_4 - 0x1)) == 0x0) {
        param_16 = param_7;
        pSVar9 = (StructA *)CONCAT22(param_19,param_18);
      }
      goto code_r0x10203db1;
    }
    pbVar2 = (u8 *)(param_4 + 0x89c);
    bVar28 = CARRY1(*pbVar2,bVar12);
    *pbVar2 = *pbVar2 + bVar12;
    bVar21 = bVar17 + bVar12;
    cVar18 = bVar21 + bVar28;
    uVar19 = CONCAT11(cVar18,bVar12);
    pcStack4._0_3_ =
         CONCAT21((param_14 & 0x1) * 0x4000 | (SCARRY1(bVar17,bVar12) != SCARRY1(bVar21,bVar28)) * 0x800 |
                  (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 | (cVar18 < '\0') * 0x80 |
                  (cVar18 == '\0') * 0x40 | (u8)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                  ((POPCOUNT(cVar18) & 0x1U) == 0x0) * 0x4 |
                  (CARRY1(bVar17,bVar12) || CARRY1(bVar21,bVar28)),pcStack4._0_1_);
    pcStack4 = (code *)((u32)pcStack4 & 0xff000000 | (u32)(u16)pcStack4);
    pbVar2 = (u8 *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_1 = uVar22 - 0x1;
    bVar10 = bVar4 | bVar20;
    if (param_1 == 0x0 || *pbVar2 == 0x0) goto code_r0x10203d96;
  } while( true );
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_draw_op_1020_3da4(u16 param_1,u16 param_2,astruct_24 *param_3,StructA *param_4)

{
  u32 *puVar2;
  u32 *pUVar3;
  i16 i16;
  HGDIOBJ16 white_pen_handle;
  HDC16 *pHVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_24 *pstruct24_1;
  u16 pstruct_24_hi;
  u32 *puVar6;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  HDC16 hdc;
  HDC16 hdc_00;
  HDC16 hdc_4;
  astruct_24 *iVar9;
  astruct_24 *uVar8;
  u32 *puVar1;
  u32 *uVar4;
  u16 in_stack_0000ffea;
  code **fn_ptr_1;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1020_7c1a((astruct_40 *)param_3,param_4);
  pstruct_24_hi = ((u32)param_3 >> 0x10);
  pstruct24_1 = (astruct_24 *)param_3;
  pstruct24_1->field17_0x14 = NULL;
  param_3->offset_0x0 = 0x408a;
  pstruct24_1->base_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x6),in_stack_0000fe92,in_stack_0000ffb6
                           ,in_stack_0000ffbc,in_stack_0000ffc0);
  hdc = (HDC16)((u32)puVar6 >> 0x10);
  &pstruct24_1->field17_0x14 = (int)puVar6;
  *(HDC16 *)((int)&pstruct24_1->field17_0x14 + 0x2) = hdc;
  hdc_00 = 0x0;
  fn_ptr_1 = (code **)((int)*pstruct24_1->field17_0x14 + 0x4);
  (**fn_ptr_1)(0x1010,&pstruct24_1->field17_0x14,hdc,0x0,param_3);
  hdc_4 = GetDC16(pstruct24_1->field2_0x4);
  i16 = SetMapMode16(0x1,hdc_4);
  pstruct24_1->field21_0x1e = i16;
  white_pen_handle = GetStockObject16(WHITE_BRUSH);
  white_pen_handle = SelectObject16(white_pen_handle,hdc);
  pstruct24_1->field18_0x18 = white_pen_handle;
  white_pen_handle = GetStockObject16(WHITE_PEN);
  white_pen_handle = SelectObject16(white_pen_handle,hdc_00);
  pstruct24_1->obj_handle_0x1a = white_pen_handle;
  uVar4 = pstruct24_1->field17_0x14;
  puVar2 = (u32 *)(u32)((int)uVar4 + 0x24);
  pHVar4 = &hdc_4;
    // just 0x1538
  fn_ptr_1 = (code **)((int)*puVar2 + 0x8);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)puVar2,(int)((u32)puVar2 >> 0x10),pHVar4,(int)&DAT_1050_1050);
  pstruct24_1->field20_0x1c = pHVar4;
  pUVar3 = pstruct24_1->field17_0x14;
  *(HDC16 *)((int)pUVar3 + 0x4c) = hdc_4;
  return;
}



// WARNING: Unable to use type for symbol uVar2

void win_ui_palette_op_1020_3e84(StructD *param_1)

{
  HDC16 hdc;
  HPALETTE16 obj;
  StructD *struct_v1;
  u16 uVar1;
  u16 unaff_SS;
  i32 uVar2;

  uVar1 = ((u32)param_1 >> 0x10);
  struct_v1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x408a;
  struct_v1->address_offset_field_0x2 = 0x1020;
  pass1_1010_1ea6(struct_v1->field12_0x14,(StructD *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  uVar2 = struct_v1->field12_0x14;
  hdc = *(HDC16 *)((int)uVar2 + 0x4c);
  SelectObject16(struct_v1->field13_0x18,hdc);
  SelectObject16(struct_v1->field14_0x1a,hdc);
  obj = SelectPalette16(0x0,*(HPALETTE16 *)&struct_v1->field_0x1c,hdc);
  DeleteObject16(obj);
  SetMapMode16(*(int16_t *)&struct_v1->field_0x1e,hdc);
  param_1->address_offset_field_0x0 = 0x3ab0;
  struct_v1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  struct_v1->address_offset_field_0x2 = 0x1008;
  return;
}



void validate_rect_1020_3f12(u32 param_1,i16 param_2)

{
  RECT16 local_a;
  u32 uStack6;

  if (param_2 == 0x1) {
    (u32)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  local_a = (RECT16)0x8000e;
  uStack6 = 0x1100116;
  InvalidateRect16(0x0,&local_a,(HWND16)&DAT_1050_1050);
  local_a = (RECT16)0xf10000;
  uStack6 = 0x1220030;
  ValidateRect16(&local_a,(HWND16)&DAT_1050_1050);
  local_a = (RECT16)0xf100f5;
  uStack6 = 0x1220127;
  ValidateRect16(&local_a,(HWND16)&DAT_1050_1050);
  return;
}



void mixed_draw_op_1020_3fa0(u32 param_1)

{
  u32 uVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  i16 iStack56;
  u32 uStack54;
  u32 local_32;
  i16 iStack46;
  u32 uStack44;
  u32 *puStack40;
  HDC16 local_24;
  u8 local_22 [0x20];

  uVar6 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),*(HWND16 *)(iVar4 + 0x4));
  uVar3 = (u32)(iVar4 + 0x14);
  local_24 = *(HDC16 *)((int)uVar3 + 0x4c);
  uVar3 = (u32)(iVar4 + 0x14);
  puStack40 = (u32 *)(u32)((int)uVar3 + 0x24);
  uVar5 = puStack40;
  ppcVar2 = (code **)((int)*puStack40 + 0x4);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,uVar5,(int)((u32)puStack40 >> 0x10),0x0,&local_24,(int)&DAT_1050_1050);
  uVar3 = (u32)(iVar4 + 0x14);
  iStack46 = ((int)uVar3 + 0x44);
  uVar3 = (u32)(iVar4 + 0x14);
  uStack44 = (u32)((int)uVar3 + 0x40);
  uVar1 = (u32)(iVar4 + 0x14);
  pass1_1008_3e94((u16 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x3a)),(u16 *)CONCAT22(0x1050,&local_32),
                  (char *)CONCAT22(0x1050,(int)&local_32 + 0x2));
  uStack54 = uStack44;
  for (iStack56 = 0x0; iStack56 < iStack46; iStack56 += 0x1) {
    draw_rect_1020_40ce(uStack54,(i16)local_32,(i16)((u32)local_32 >> 0x10),local_24,uVar5);
    uStack54 = uStack54 & 0xffff0000 | (u32)((int)uStack54 + 0x18);
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT13(0x10,CONCAT12(0x50,local_22)),*(HWND16 *)(iVar4 + 0x4));
  return;
}



StructD * pass1_1020_4064(StructD *param_1,u8 param_2)

{
  win_ui_palette_op_1020_3e84(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1020_4092(u16 *param_1)

{
  i16 iVar1;
  u16 uVar2;

  pass1_1008_3e38((astruct_19 *)param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x6) = 0x0;
  (iVar1 + 0x8) = 0x0;
  (iVar1 + 0xa) = 0x1;
  (iVar1 + 0xc) = 0x0;
  (iVar1 + 0xe) = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0x10)));
  return param_1;
}



void draw_rect_1020_40ce(u32 param_1,i16 param_2,i16 param_3,HDC16 hdc16_param_4,u16 param_5)

{
  HPEN16 pen_handle;
  HGDIOBJ16 brush_handle_1;
  i16 right;
  i16 bottom;
  u16 unaff_SS;
  HDC16 hdc;
  i16 local_6;
  i16 local_4;
  HDC16 hdc16_var_fff2;
  i16 iVar1;

  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x10)),(u16 *)CONCAT22(0x1050,&local_6),
                  (char *)CONCAT22(0x1050,&local_4));
  pass1_1008_3e94((u16 *)param_1,(u16 *)CONCAT22(0x1050,&local_6),(char *)CONCAT22(0x1050,&local_4));
  iVar1 = ((int)param_1 + 0xa);
  Ellipse16(iVar1 + local_6 + param_2,iVar1 + local_4 + param_3,(local_6 - ((int)param_1 + 0xa)) + param_2,
            (local_4 - ((int)param_1 + 0xa)) + param_3,hdc16_param_4);
  if ((*(u8 *)((int)param_1 + 0xe) & 0x1) != 0x0) {
    brush_handle_1 = GetStockObject16(HOLLOW_BRUSH);
    SelectObject16(brush_handle_1,hdc16_var_fff2);
    hdc = hdc16_param_4;
    pen_handle = CreatePen16(0x10000f9,0x1,0x0);
    SelectObject16(pen_handle,hdc);
    right = local_4 + param_3 + 0x5;
    bottom = local_6 + param_2 + 0x5;
    Rectangle16(bottom,right,local_6 + param_2 + -0x5,local_4 + param_3 + -0x5,hdc16_param_4);
    brush_handle_1 = GetStockObject16(WHITE_BRUSH);
    SelectObject16(brush_handle_1,right);
    brush_handle_1 = GetStockObject16(WHITE_PEN);
    brush_handle_1 = SelectObject16(brush_handle_1,bottom);
    DeleteObject16(brush_handle_1);
  }
  return;
}


/*
Unable to decompile 'unk_draw_op_1020_41c8'
Cause:
Low-level Error: Symbol $$undef0000000d extends beyond the end of the address space
*/


void destroy_cursor_1020_42f4(StructD *param_1)

{
  StructD *struct_1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  struct_1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x623c;
  struct_1->address_offset_field_0x2 = 0x1020;
  &struct_1->field_0xe2 = 0x62d8;
  &struct_1->field_0xe4 = 0x1020;
  if (struct_1[0x1].field13_0x18 != 0x0) {
    DestroyMenu16(struct_1[0x1].field13_0x18);
  }
  DestroyCursor16(struct_1[0x1].address_offset_field_0x2);
  DestroyCursor16(struct_1[0x1].hfile_0x4);
  pass1_1020_808e(param_1);
  return;
}



void pass1_1020_434c(u16 param_1,i16 param_2,u16 param_3,u32 *param_4,u16 param_5,u16 param_6,i16 param_7)

{
  if (param_7 == 0x1) {
    pass1_1020_6184(CONCAT22(param_3,param_2),param_6);
    return;
  }
  if (param_7 == 0x2) {
    ui_op_1020_536e(param_1,CONCAT22(param_3,param_2),CONCAT22(param_5,param_4),param_6,0x2);
    return;
  }
  pass1_1008_68ea(param_2,param_3,param_4,param_5,param_6,param_7);
  return;
}



void post_msg_1020_4394(u32 param_1,u16 param_2)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  iVar2 = (int)param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x10) {
    if ((iVar2 + 0x34) != 0x0) {
      PostMessage16(0x0,0xf6,0x111,HWND16_1050_0396);
      return;
    }
  }
  else if (param_2 < 0x11) {
    if ((char)param_2 == '\x01') {
      (u32)(iVar2 + 0x18) = 0x0;
      return;
    }
    if ((char)param_2 == '\v') {
      uVar1 = (u32)(iVar2 + 0x2c);
      ((int)uVar1 + 0xe) = (iVar2 + -0xda);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_1020_43f6(astruct_57 *param_1,StructA *param_2,u16 param_3,u16 param_4,u16 param_5,
                  u16 param_6,u16 param_7)

{
  code **ppcVar1;
  astruct_160 *paVar2;
  u16 uVar3;
  astruct_57 *paVar4;
  u32 *puVar5;
  u16 uVar6;
  StructA *struct_a_1;

  struct_a_1 = (StructA *)param_2;
  uVar6 = ((u32)param_2 >> 0x10);
  create_window_ex_1008_9760(param_2);
  get_dc_1018_4db0(*(astruct_126 **)&struct_a_1[0x1].field25_0x2e,struct_a_1->field4_0x8);
  puVar5 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x32),param_3,param_4,param_5,param_6);
  paVar4 = (astruct_57 *)((u32)param_1 & 0xffff0000);
  &struct_a_1[0x1].field38_0x42 = (int)puVar5;
  ((int)&struct_a_1[0x1].field38_0x42 + 0x2) = (int)((u32)puVar5 >> 0x10);
  if (param_2 != NULL) {
    paVar4 = (astruct_57 *)((u32)paVar4 | (u32)uVar6);
  }
  ppcVar1 = (code **)((int)(u32)struct_a_1[0x1].field38_0x42 + 0x4);
  paVar2 = (astruct_160 *)(**ppcVar1)();
  mem_op_1000_179c(0x30,paVar4);
  uVar3 = paVar4 | paVar2;
  if (uVar3 == 0x0) {
    (u32)&struct_a_1[0x1].field22_0x2a = 0x0;
  }
  else {
    pass1_1020_62e0((int)paVar2,paVar4,struct_a_1->field4_0x8);
    struct_a_1[0x1].field22_0x2a = paVar2;
    &struct_a_1[0x1].field_0x2c = uVar3;
  }
  ui_op_1020_536e(uVar3,param_2,0x0,-0x1,0x3);
  return;
}



void pass1_1020_44b0(u32 *param_1)

{
  code **ppcVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0xf6) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x98);
    (**ppcVar1)();
    (iVar2 + 0x112) = 0x0;
    ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xf6) + 0x8);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_menu_op_1020_44ec
               (astruct_850 *param_1,u16 param_2,i16 param_3,HMENU16 param_4,u16 param_5,undefined1 param_6)

{
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;
  u16 UVar4;
  BOOL16 BVar5;
  HMENU16 HVar6;
  u16 uVar7;
  u32 uVar8;
  u8 *puVar9;
  u16 uVar10;
  u16 in_register_0000000a;
  astruct_57 *paVar11;
  astruct_850 *iVar9;
  i16 iVar12;
  u16 uVar13;
  u16 uVar14;
  char *data;
  u32 *puVar15;
  u16 in_stack_0000fd70;
  u16 in_stack_0000fe94;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000fe9e;
  u16 w_flags;
  u16 in_stack_0000fec8;
  HMENU16 w_item_id;
  u16 uStack300;
  u8 bStack293;
  u16 uStack278;
  u32 uStack268;
  u32 local_108 [0x40];
  u16 uStack8;
  u32 *puStack6;

  paVar11 = (astruct_57 *)CONCAT22(in_register_0000000a,param_5);
  uVar13 = ((u32)param_1 >> 0x10);
  iVar9 = (astruct_850 *)param_1;
  if (iVar9->hmenu_0x106 != 0x0) {
    if (iVar9->hmenu_0x106 == param_4) {
      puStack6 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fec8,0x3),in_stack_0000fd70,
                                 in_stack_0000fe94,in_stack_0000fe9a,in_stack_0000fe9e);
      uVar2 = iVar9->field257_0x108;
      uStack8 = ((int)uVar2 + 0x2e);
      uVar2 = iVar9->field257_0x108;
      uVar14 = ((u32)uVar2 >> 0x10);
      iVar12 = (int)uVar2;
      uVar1 = (u32)(iVar12 + 0x42);
      puVar9 = *(u8 **)(iVar12 + 0x44);
      bStack293 = (u8)(uVar1 >> 0x18);
      uVar7 = bStack293;
      if (bStack293 == 0x0) {
        uVar3 = pass1_1020_bd80(uStack8);
        unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_108),(char *)CONCAT22(puVar9,uVar3));
      }
      else {
        pass1_1030_8344(_u16_1050_5748,uVar1 & 0xffff | ZEXT24(puVar9) << 0x10);
        pass1_1010_c3c2(puVar9,puStack6,((u32)puStack6 >> 0x10),CONCAT22(0x1050,local_108),
                        CONCAT22(puVar9,uVar7));
      }
      ModifyMenu16(CONCAT22(0x1050,local_108),0x76,0x0,0x76,iVar9->hmenu_0x106);
      UVar4 = GetMenuState16(0x0,0x13c,iVar9->hmenu_0x106);
      if (UVar4 != 0xffff) {
        DeleteMenu16(0x0,0x13c,iVar9->hmenu_0x106);
      }
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x20);
      if (BVar5 != 0x0) {
        data = load_string_1010_847e(_u16_1050_14cc,0x74b);
        InsertMenu16((u32)data,0x13c,0x400,0xffff,iVar9->hmenu_0x106);
      }
      if (((int)s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0x0) {
        HVar6 = iVar9->hmenu_0x106;
        w_flags = 0x1;
        UVar4 = 0x77;
        goto LAB_1020_464c;
      }
      HVar6 = iVar9->hmenu_0x106;
      UVar4 = 0x77;
    }
    else {
      HVar6 = GetSubMenu16(0x1,iVar9->hmenu_0x106);
      if (HVar6 != param_4) goto LAB_1020_479e;
      EnableMenuItem16(0x1,0x200,HVar6);
      uVar10 = paVar11;
      EnableMenuItem16(0x1,0x201,HVar6);
      EnableMenuItem16(0x1,0x202,HVar6);
      uVar2 = iVar9->field257_0x108;
      uVar8 = (u32)((int)uVar2 + 0x42);
      pass1_1030_8344(_u16_1050_5748,uVar8);
      uVar7 = uVar8;
      if ((uVar10 | uVar7) == 0x0) {
        return;
      }
      uVar2 = (u32)(uVar7 + 0x2e);
      uVar7 = (uVar7 + 0x30);
      uStack278 = uVar2;
      if ((uVar7 | uStack278) == 0x0) {
        return;
      }
      uStack268 = (u32)(uStack278 + 0x200);
      local_108[0] = struct_op_1030_73a8((astruct_419 *)(uVar8 & 0xffff | (u32)uVar10 << 0x10),(int)uStack268,uVar7);
      uVar13 = (local_108[0] >> 0x10);
      puStack6 = (u32*)((int)local_108[0] + 0x1c);
      uVar7 = ((int)local_108[0] + 0x1e);
      if ((uVar7 | puStack6) != 0x0) {
        uStack268 = (u32)puStack6 & 0xffff | (u32)uVar7 << 0x10;
      }
      uStack268 &= 0xff;
      if ((int)uStack268 != 0x1) {
        return;
      }
      if ((uStack268 & 0xff0000) != 0x0) {
        return;
      }
      uVar3 = pass1_1030_6fa0(uVar8 & 0xffff | (u32)uVar10 << 0x10);
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x3f);
      if (BVar5 != 0x0) {
        BVar5 = EnableMenuItem16(0x0,0x201,HVar6);
      }
      if (*(i32 *)((int)(uVar8 & 0xffff) + 0x36) != 0x0) {
        BVar5 = EnableMenuItem16(0x0,0x202,HVar6);
      }
      pass1_1030_69cc(BVar5,uStack268,uVar8 & 0xffff | (u32)uVar10 << 0x10);
      if (BVar5 == 0x0) {
        return;
      }
      UVar4 = 0x200;
    }
    w_flags = 0x0;
    goto LAB_1020_464c;
  }
LAB_1020_479e:
  iVar12 = param_3 + -0x1;
  if (iVar12 == 0x0) {
    pass1_1018_2504(0x0,(int)paVar11);
    if (iVar12 == 0x0) {
      UVar4 = 0x0;
      EnableMenuItem16(0x401,0x0,param_4);
      HVar6 = 0x1;
LAB_1020_47e3:
      w_flags = 0x401;
      goto LAB_1020_464c;
    }
    UVar4 = 0x0;
    EnableMenuItem16(0x400,0x0,param_4);
    HVar6 = 0x1;
  }
  else if (param_3 == 0x2) {
    uVar3 = pass1_1020_64d4(iVar9->field246_0xf6,0x2);
    if (uVar3 == 0x0) {
      EnableMenuItem16(0x401,0x0,param_4);
      UVar4 = 0x401;
    }
    else {
      EnableMenuItem16(0x400,0x0,param_4);
      UVar4 = 0x400;
    }
    HVar6 = 0x1;
    EnableMenuItem16(UVar4,0x1,param_4);
    if ((PTR_LOOP_1050_0010 != NULL) || (iVar9->field255_0x102 == 0x0)) goto LAB_1020_47e3;
  }
  else {
    if (param_3 == 0x3) {
      HVar6 = 0x0;
      puVar15 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)0x2f,in_stack_0000fd70,in_stack_0000fe94,
                                in_stack_0000fe9a,in_stack_0000fe9e);
      uVar13 = ((u32)puVar15 >> 0x10);
      uVar1 = (u32)((int)puVar15 + 0x20);
      uVar7 = ((int)puVar15 + 0x22);
      uStack300 = uVar1;
      if ((uVar7 | uStack300) != 0x0) {
        pass1_1030_8308(&stack0xfecc,uVar7,_u16_1050_5748,(_u16_1050_5748 >> 0x10),
                        (u16 *)CONCAT22(0x1050,&stack0xfecc),(u16 *)CONCAT22(0x1050,&stack0xfec8),
                        uVar1 & 0xffff | (u32)uVar7 << 0x10);
      }
      UVar4 = 0x0;
      do {
        CheckMenuItem16(0x400,UVar4,param_4);
        w_item_id = param_4;
        EnableMenuItem16(0x401,UVar4,param_4);
        UVar4 += 0x1;
      } while ((int)UVar4 < 0x5);
      CheckMenuItem16(0x408,w_item_id,param_4);
      for (UVar4 = 0x0; (int)UVar4 <= (int)HVar6; UVar4 += 0x1) {
        HVar6 = param_4;
        EnableMenuItem16(0x400,UVar4,param_4);
      }
      return;
    }
    if (param_3 != 0x4) {
      return;
    }
    UVar4 = 0x2;
    HVar6 = param_4;
  }
  w_flags = 0x400;
LAB_1020_464c:
  EnableMenuItem16(w_flags,UVar4,HVar6);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_sys_op_1020_493c
               (u16 param_1,StructD *param_2,StructD *param_3,u16 param_4,u16 param_5,u16 param_6,
               u32 *param_7)

{
  code **ppcVar1;
  u32 uVar2;
  i32 lVar3;
  HCURSOR16 HVar4;
  u16 uVar5;
  i16 iVar6;
  u16 *puVar7;
  u32 *puVar8;
  u16 uVar10;
  u16 uVar11;
  u16 uVar12;
  u16 uVar13;
  u16 uVar14;
  astruct_57 *paVar15;
  StructD *uVar9;
  u16 uVar16;
  u32 uVar17;
  u32 *puVar18;
  astruct_97 *paVar19;
  char *pcVar20;
  u16 in_stack_0000fb4e;
  u16 in_stack_0000fb50;
  u16 in_stack_0000fb52;
  u16 in_stack_0000fc72;
  u16 in_stack_0000fc74;
  u16 in_stack_0000fc76;
  u16 in_stack_0000fc78;
  u16 in_stack_0000fc7a;
  u16 in_stack_0000fc7c;
  u16 in_stack_0000fc7e;
  u16 in_stack_0000fc80;
  u16 uStack852;
  u16 local_24e;
  u16 uStack588;
  u32 local_144;
  u32 uStack320;
  u32 local_13c;
  u16 uStack42;
  u32 uStack38;
  u16 uStack34;
  u16 uStack32;
  u32 uStack30;
  u32 uStack26;
  u32 uStack22;
  u32 *puStack18;
  u8 *puStack14;
  u8 *puStack12;
  u16 uStack10;
  u32 uStack6;

  if (param_4 == 0xe9) {
    return;
  }
  uVar9 = (StructD *)param_3;
  uVar13 = ((u32)param_3 >> 0x10);
  if (param_4 < 0xea) {
    switch(param_4) {
    case 0x69:
      iVar6 = 0x0;
      break;
    case 0x6a:
      iVar6 = 0x1;
      break;
    case 0x6b:
      iVar6 = 0x2;
      break;
    case 0x6c:
      iVar6 = 0x3;
      break;
    case 0x6d:
      iVar6 = 0x4;
      break;
    default:
      return;
    case 0x77:
      if ((&uVar9[0x1].field_0x1c | uVar9[0x1].field14_0x1a) == 0x0) {
        return;
      }
      uVar2 = (u32)&uVar9[0x1].field14_0x1a;
      uVar11 = ((int)s_VrMode_1050_42ca + 0x8 + ((int)uVar2 + 0x2e) * 0x2);
      uStack26 = (u32 *)((u32)uStack26 & 0xffff0000 | (u32)uVar11);
      if (uVar11 == 0x0) {
        return;
      }
      uVar16 = FUN_1010_830a(uVar11,param_2,0x1020,_u16_1050_14cc,0x1f8);
      puStack18 = (u32 *)CONCAT22((int)param_2,uVar16);
      param_7 = (u32 *)uVar9->field5_0x8;
      WinHelp16(CONCAT13((u8)((int)uStack26 >> 0xf),
                         CONCAT12((u8)((int)uStack26 >> 0xf),
                                  uStack26 & 0xff | (u8)((u32)(long)(int)uStack26 >> 0x8) << 0x8))
                ,0x1,(char *)CONCAT22((int)param_2,uVar16),(HWND16)param_7);
      return;
    case 0x78:
      puVar18 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x45),
                                in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
      uStack588 = ((u32)puVar18 >> 0x10);
      local_24e = puVar18;
      enum_child_windows_1010_01be();
      return;
    }
    set_cursor_1020_5764((u32)param_3,iVar6);
    return;
  }
  if (param_4 == 0x132) {
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0xffff;
    goto LAB_1020_4ef8;
  }
  if (param_4 < 0x133) {
    if (param_4 == 0x102) {
      uVar16 = 0x1000;
      mem_op_1000_179c(0xb4,(astruct_57 *)param_2);
      uStack32 = param_2;
      uVar17 = (u32)param_2 & 0xffff0000 | (u32)(uStack32 | param_4);
      uStack34 = param_4;
      if ((uStack32 | param_4) == 0x0) {
        iVar6 = 0x0;
        uVar12 = 0x0;
      }
      else {
        uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
        iVar6 = string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,param_4),HWND16_1050_0396,0x20031,0x62b057b);
        uVar12 = uVar17;
      }
      local_144 = (u32 *)CONCAT22(uVar12,iVar6);
      ppcVar1 = (code **)((int)*local_144 + 0x74);
      (**ppcVar1)(uVar16,iVar6,uVar12);
      uStack320 = (u32 *)CONCAT22(uStack320,iVar6);
      if (iVar6 != 0x1) {
        return;
      }
      pass1_1028_837e((astruct_97 *)CONCAT22(0x1050,&local_13c));
LAB_1020_4b6c:
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_13c));
      return;
    }
    if (param_4 < 0x103) {
      switch(param_4) {
      case 0xf0:
        ui_op_1020_536e((int)param_2,param_3,0x0,-0x1,0x1);
        return;
      default:
        return;
      case 0xf6:
        if (&uVar9[0x1].field_0x28 != 0x0) {
          if (param_3 == NULL) {
            param_7 = NULL;
            uStack852 = 0x0;
          }
          else {
            param_7 = (u32 *)&uVar9->field_0xe2;
            uStack852 = uVar13;
          }
          param_2 = (StructD *)(u32)uStack852;
          pass1_1010_1ea6(_u16_1050_02a0,(StructD *)CONCAT22(uStack852,param_7));
          &uVar9[0x1].field_0x28 = 0x0;
        }
        iVar6 = 0x12;
        break;
      case 0xf7:
        unk_win_op_1010_7300(param_2,*(astruct_57 **)&uVar9[0x1].field19_0x20,0x0,0x9,0x0);
        return;
      case 0xfb:
        local_144 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x3),
                                    in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
        uStack320 = mixed_1010_20ba((astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)local_144 >> 0x10),
                                    _u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x30),in_stack_0000fb52,
                                    in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
        pcVar20 = (char *)pass1_1010_375e((u32)uStack320);
        pass1_1010_c25e(pcVar20,(u8 *)((u32)pcVar20 >> 0x10),local_144,
                        ((u32)local_144 >> 0x10),pcVar20);
        return;
      case 0xfc:
        post_msg_1020_55b0(param_2,(u32)param_3,param_5,param_6);
        return;
      case 0x101:
        uStack26 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x2f),
                                   in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
        uVar16 = ((u32)uStack26 >> 0x10);
        uStack22 = (u32)((int)uStack26 + 0x24);
        uVar11 = ((int)uStack26 + 0x26);
        paVar15 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar11);
        uStack22._0_2_ = uVar11 | uStack22;
        if (uStack22 == 0x0) {
          uVar16 = 0x1000;
          mem_op_1000_179c(0xb4,paVar15);
          uStack32 = paVar15;
          uVar17 = (u32)paVar15 & 0xffff0000 | (u32)(uStack32 | uStack22);
          uStack34 = uStack22;
          if ((uStack32 | uStack22) == 0x0) {
            puVar8 = NULL;
            uVar11 = 0x0;
          }
          else {
            uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
            puVar8 = (u32 *)
                     string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack22),HWND16_1050_0396,0x20030,
                                      0x730057b);
            uVar11 = uVar17;
          }
          uStack30 = CONCAT22(uVar11,puVar8);
LAB_1020_4c5f:
          ppcVar1 = (code **)((int)*puVar8 + 0x74);
          (**ppcVar1)(uVar16,puVar8,uVar11);
          return;
        }
        uVar17 = pass1_1038_af40(uVar9,uVar11,_PTR_LOOP_1050_5b7c,uVar9->field5_0x8,0xe);
        puStack18 = mixed_1010_20ba((astruct_57 *)((u32)paVar15 & 0xffff0000 | uVar17 >> 0x10),_u16_1050_0ed0,
                                    (u8 **)CONCAT22(param_7,0x43),in_stack_0000fb52,in_stack_0000fc76,
                                    in_stack_0000fc7c,in_stack_0000fc80);
        uVar16 = ((u32)puStack18 >> 0x10);
        iVar6 = (int)puStack18;
        puStack14 = (u8 *)(iVar6 + 0xa);
        uStack10 = (iVar6 + 0xc);
        uVar13 = (iVar6 + 0xe);
        uStack6 = CONCAT22(uStack6,uVar13);
        if ((iVar6 + 0x10) != 0x0) {
          return;
        }
        pass1_1028_84ca((astruct_97 *)CONCAT22(0x1050,&local_13c),uStack22,uVar13,uStack10,puStack14);
        goto LAB_1020_4b6c;
      }
    }
    else {
      if (param_4 != 0x106) {
        if (param_4 < 0x107) {
          if (param_4 == 0x103) {
            local_144 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x2f),
                                        in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
            uVar16 = ((u32)local_144 >> 0x10);
            uStack320 = (u32 *)*(char **)((int)local_144 + 0x24);
            uVar11 = ((int)local_144 + 0x26);
            paVar15 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar11);
            uStack34 = uVar11 | uStack320;
            if (uStack34 != 0x0) {
              uVar17 = pass1_1038_af40(uVar9,uVar11,_PTR_LOOP_1050_5b7c,uVar9->field5_0x8,0xf);
              local_13c = (astruct_477 *)
                          mixed_1010_20ba((astruct_57 *)((u32)paVar15 & 0xffff0000 | uVar17 >> 0x10),_u16_1050_0ed0,
                                          (u8 **)CONCAT22(param_7,0x42),in_stack_0000fb52,in_stack_0000fc76,
                                          in_stack_0000fc7c,in_stack_0000fc80);
              uStack42 = ((int)local_13c + 0xa);
              if (uStack42 == 0x0) {
                return;
              }
              pass1_1030_e63e((astruct_97 *)CONCAT22(0x1050,&local_24e),uStack42);
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_24e));
              return;
            }
            uVar16 = 0x1000;
            mem_op_1000_179c(0xb4,paVar15);
            uStack32 = paVar15;
            uVar17 = (u32)paVar15 & 0xffff0000 | (u32)(uStack32 | uStack34);
            if ((uStack32 | uStack34) == 0x0) {
              puVar8 = NULL;
              uVar11 = 0x0;
            }
            else {
              uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
              puVar8 = (u32 *)
                       string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack34),HWND16_1050_0396,0x20030,
                                        0x730057b);
              uVar11 = uVar17;
            }
            uStack38 = CONCAT22(uVar11,puVar8);
          }
          else {
            if (param_4 != 0x104) {
              return;
            }
            uVar16 = 0x22;
            puVar18 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)0x220003,in_stack_0000fb50,
                                      in_stack_0000fc74,in_stack_0000fc7a,in_stack_0000fc7e);
            paVar15 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)puVar18 >> 0x10);
            uStack34 = puVar18;
            uStack588 = ((u32)puVar18 >> 0x10);
            local_24e = uStack34;
            pass1_1010_af66(uStack588,(u32)puVar18,uVar16);
            local_144 = (u32 *)CONCAT22(local_144,uStack34);
            if (uStack34 != 0x0) {
              uVar16 = 0x1000;
              mem_op_1000_179c(0xb4,paVar15);
              uStack32 = paVar15;
              uVar17 = (u32)paVar15 & 0xffff0000 | (u32)(uStack32 | uStack34);
              if ((uStack32 | uStack34) == 0x0) {
                iVar6 = 0x0;
                uVar12 = 0x0;
              }
              else {
                uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
                iVar6 = string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack34),HWND16_1050_0396,0x20031,
                                         0x62c057b);
                uVar12 = uVar17;
              }
              uStack320 = (u32 *)CONCAT22(uVar12,iVar6);
              ppcVar1 = (code **)((int)*uStack320 + 0x74);
              (**ppcVar1)(uVar16,iVar6,uVar12);
              local_13c = (astruct_477 *)CONCAT22(local_13c,iVar6);
              if (iVar6 != 0x1) {
                return;
              }
              paVar19 = pass1_1030_e79a((astruct_97 *)CONCAT22(0x1050,&param_7));
              uVar13 = ((u32)paVar19 >> 0x10);
              puVar7 = &param_7;
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,puVar7));
              win_1008_5c5c(puVar7,uVar13,_u16_1050_02a0,0x1e6);
              return;
            }
            uVar16 = 0x1000;
            mem_op_1000_179c(0xb4,paVar15);
            uStack32 = paVar15;
            uVar17 = (u32)paVar15 & 0xffff0000 | (u32)(uStack32 | uStack34);
            if ((uStack32 | uStack34) == 0x0) {
              puVar8 = NULL;
              uVar11 = 0x0;
              param_7 = puVar8;
              uStack852 = uVar11;
            }
            else {
              uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
              puVar8 = (u32 *)
                       string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack34),HWND16_1050_0396,0x20030,
                                        0x731057b);
              uVar11 = uVar17;
              param_7 = puVar8;
              uStack852 = uVar11;
            }
          }
          goto LAB_1020_4c5f;
        }
        if (param_4 == 0x12f) {
          pass1_1020_61c4(uVar9,uVar13,CONCAT22(0x1050,&local_144),(u16 *)CONCAT22(0x1050,&local_24e));
          iVar6 = local_24e + 0x6a;
        }
        else {
          if (param_4 != 0x130) {
            if (param_4 != 0x131) {
              return;
            }
            uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x2);
            if (uVar10 == 0x0) {
              return;
            }
            iVar6 = 0x7;
            goto LAB_1020_49b7;
          }
          pass1_1020_61c4(uVar9,uVar13,CONCAT22(0x1050,&local_144),(u16 *)CONCAT22(0x1050,&local_24e));
          iVar6 = local_24e + 0x68;
        }
        uStack320 = (u32 *)CONCAT22(uStack320,iVar6);
        if (0x6d < iVar6) {
          return;
        }
        if (iVar6 < 0x69) {
          return;
        }
        ppcVar1 = (code **)((int)(u32)param_3 + 0x40);
        (**ppcVar1)();
        return;
      }
      iVar6 = 0x13;
    }
LAB_1020_49b7:
    pass1_1038_af40(uVar9,param_2,_PTR_LOOP_1050_5b7c,uVar9->field5_0x8,iVar6);
    return;
  }
  if (param_4 == 0x1c8) {
    lVar3 = uVar9[0x1].field12_0x14;
    SendMessage16(0x0,0x72,0x111,*(HWND16 *)((int)lVar3 + 0x8));
    return;
  }
  if (0x1c8 < param_4) {
    if (param_4 == 0x1ca) {
      local_144 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x3),
                                  in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
      uVar17 = (u32)param_2 & 0xffff0000;
      uStack320 = (u32 *)pass1_1010_c234(local_144,(u8 *)((u32)local_144 >> 0x10));
      uVar11 = uStack320;
      uVar14 = ((u32)uStack320 >> 0x10);
      if ((uVar14 | uVar11) == 0x0) {
        return;
      }
      local_13c = (astruct_477 *)
                  mixed_1010_20ba((astruct_57 *)(uVar17 & 0xffff0000 | (u32)(uVar14 | uVar11)),_u16_1050_0ed0,
                                  (u8 **)CONCAT22(uVar11,0x30),in_stack_0000fb4e,in_stack_0000fc72,
                                  in_stack_0000fc78,in_stack_0000fc7c);
      param_2 = (StructD *)((u32)local_13c >> 0x10);
      pass1_1010_3770(((u32)local_13c >> 0x10),local_13c,(char *)CONCAT22(uVar14,uVar11));
      iVar6 = 0x3;
    }
    else if (param_4 == 0x200) {
      uVar2 = (u32)&uVar9[0x1].field14_0x1a;
      uVar16 = ((u32)uVar2 >> 0x10);
      iVar6 = (int)uVar2;
      uStack26 = (u32*)(iVar6 + 0x42);
      uVar11 = (iVar6 + 0x44);
      param_2 = (StructD *)(u32)uVar11;
      uStack26._3_1_ = (u8)((u32)uStack26 >> 0x18);
      puStack14 = (u8 *)uStack26._3_1_;
      if (uStack26._3_1_ != 0x5) {
        return;
      }
      pass1_1030_8344((u32)_u16_1050_5748,(u32)uStack26 & 0xffff | (u32)uVar11 << 0x10);
      PTR_LOOP_1050_5f0e = (u8 *)param_2;
      iVar6 = 0x25;
      PTR_LOOP_1050_5f0c = puStack14;
      puStack12 = PTR_LOOP_1050_5f0e;
    }
    else if (param_4 == 0x201) {
      uVar2 = (u32)&uVar9[0x1].field14_0x1a;
      uVar16 = ((u32)uVar2 >> 0x10);
      iVar6 = (int)uVar2;
      uStack26 = (u32*)(iVar6 + 0x42);
      uVar11 = (iVar6 + 0x44);
      param_2 = (StructD *)(u32)uVar11;
      uStack26._3_1_ = (u8)((u32)uStack26 >> 0x18);
      puStack14 = (u8 *)uStack26._3_1_;
      if (uStack26._3_1_ != 0x5) {
        return;
      }
      pass1_1030_8344((u32)_u16_1050_5748,(u32)uStack26 & 0xffff | (u32)uVar11 << 0x10);
      PTR_LOOP_1050_5f18 = (u8 *)param_2;
      iVar6 = 0x26;
      PTR_LOOP_1050_5f16 = puStack14;
      puStack12 = PTR_LOOP_1050_5f18;
    }
    else {
      if (param_4 != 0x202) {
        if (param_4 != 0x203) {
          return;
        }
        if (&uVar9[0x1].field_0x6 != 0x1) {
          return;
        }
        HVar4 = SetCursor16(uVar9[0x1].hfile_0x4);
        (uVar9 + 0x1)->address_offset_field_0x0 = HVar4;
        &uVar9[0x1].field_0x6 = 0x3;
        param_7 = (u32 *)uVar9->field5_0x8;
        SetCapture16((HWND16)param_7);
        return;
      }
      uVar2 = (u32)&uVar9[0x1].field14_0x1a;
      uVar16 = ((u32)uVar2 >> 0x10);
      iVar6 = (int)uVar2;
      uStack6 = (u32)(iVar6 + 0x42);
      uVar11 = (iVar6 + 0x44);
      param_2 = (StructD *)(u32)uVar11;
      uStack6._3_1_ = (u8)(uStack6 >> 0x18);
      uVar5 = uStack6._3_1_;
      if (uStack6._3_1_ != 0x5) {
        return;
      }
      pass1_1030_8344((u32)_u16_1050_5748,uStack6 & 0xffff | (u32)uVar11 << 0x10);
      PTR_LOOP_1050_5a6a = (u8 *)param_2;
      uStack22 = CONCAT22(PTR_LOOP_1050_5a6a,uVar5);
      iVar6 = 0x27;
      u16_1050_5a68 = uVar5;
    }
    goto LAB_1020_49b7;
  }
  switch(param_4) {
  case 0x133:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0xffff;
    uVar16 = 0x0;
    break;
  case 0x134:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0x1;
    goto LAB_1020_4ef8;
  case 0x135:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0x1;
    uVar16 = 0x0;
    break;
  case 0x136:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0xfffb;
    goto LAB_1020_4ef8;
  case 0x137:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0xfffb;
    uVar16 = 0x0;
    break;
  case 0x138:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0x5;
LAB_1020_4ef8:
    uVar12 = 0x0;
    break;
  case 0x139:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0x5;
    uVar16 = 0x0;
    break;
  default:
    goto switchD_1020_518a_caseD_13a;
  case 0x13c:
    uVar10 = pass1_1020_64d4((u32)&uVar9[0x1].field5_0x8,0x2);
    if (uVar10 != 0x0) {
      iVar6 = 0x1a;
      goto LAB_1020_49b7;
    }
    goto switchD_1020_518a_caseD_13a;
  }
  pass1_1020_2a94((u32)&uVar9->field_0xce,CONCAT22(uVar16,uVar12));
switchD_1020_518a_caseD_13a:
  return;
}



void pass1_1020_51c6(u32 param_1,u16 param_2,u32 param_3)

{
  code **ppcVar1;
  i16 iVar2;
  u16 in_DX;
  u16 uVar3;
  u16 uVar4;

  uVar3 = (param_1 >> 0x10);
  iVar2 = ((int)param_1 + 0xf4);
  uVar4 = param_3;
  if (iVar2 == 0x2) {
    win_ui_op_1020_5e76(param_1 & 0xffff | (u32)uVar3 << 0x10,param_2,uVar4);
    return;
  }
  iVar2 += -0x3;
  if (iVar2 != 0x0) {
    pt_in_rect_op_1020_58ce(in_DX,param_1 & 0xffff | (u32)uVar3 << 0x10,param_2,uVar4,(u8)(param_3 >> 0x10));
    if (iVar2 == 0x0) {
      ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x5c);
      (**ppcVar1)();
    }
    return;
  }
  win_ui_op_1020_5de8(param_1 & 0xffff | (u32)uVar3 << 0x10,param_2,uVar4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_cursor_op_1020_522e(u16 param_1,astruct_52 *param_2,u16 param_3,u16 param_4)

{
  i16 iVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  astruct_52 *iVar4;
  astruct_52 *uVar4;
  u32 *puVar5;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  u8 uVar6;
  u8 uVar7;
  u16 uVar8;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar4 = (astruct_52 *)((u32)param_2 >> 0x10);
  iVar4 = (astruct_52 *)param_2;
  iVar1 = iVar4->field242_0xf4;
  if (iVar1 == 0x2) {
    SetCursor16(iVar4->field237_0xee);
    iVar4->field237_0xee = 0x0;
    iVar4->field242_0xf4 = 0x1;
    (iVar4 + 0x1) = 0x0;
    ReleaseCapture16();
    return;
  }
  if (iVar1 == 0x3) {
    SetCursor16(iVar4->field237_0xee);
    iVar4->field237_0xee = 0x0;
    iVar4->field242_0xf4 = 0x1;
    (iVar4 + 0x1) = 0x0;
    ReleaseCapture16();
    uVar6 = 0x0;
    uVar7 = 0x0;
    uVar8 = 0x0;
    puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)0x47,in_stack_0000fea0,in_stack_0000ffc4,
                             in_stack_0000ffca,in_stack_0000ffce);
    pass1_1018_57e6((u32)puVar5,CONCAT22(uVar8,CONCAT11(uVar7,uVar6)),(int)puVar5,(int)((u32)puVar5 >> 0x10));
    return;
  }
  BVar3 = menu_ui_op_1020_5bf2(param_2,param_3,param_4);
  if (BVar3 == 0x0) {
    ppcVar2 = (code **)((int)*(u32*)&iVar4->field_0x4 + 0x60);
    (**ppcVar2)();
  }
  return;
}



void pass1_1020_52de(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  puVar1 = (u32 *)(iVar6 + 0xf6);
  uVar2 = (iVar6 + 0xf8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar6 + 0xf6) = 0x0;
  if (*(i32 *)(iVar6 + 0xfa) != 0x0) {
    if (param_1 == 0x0) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((u32)(iVar6 + 0xfa),(StructD *)CONCAT22(uVar5,iVar4));
  }
  destroy_win_1008_628e(param_1);
  if (*(i32 *)(iVar6 + 0xfa) != 0x0) {
    pass1_1010_1dda((u32)(iVar6 + 0xfa));
  }
  (u32)(iVar6 + 0xfa) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void ui_op_1020_536e(u16 param_1,u32 param_2,u32 param_3,i16 param_4,i16 param_5)

{
  i16 *piVar1;
  u16 UVar2;
  u32 uVar3;
  code **ppcVar4;
  u16 uVar5;
  u16 uVar6;
  u16 UVar7;
  u16 uVar8;
  u8 *puVar9;
  u16 uVar10;
  u16 in_register_0000000a;
  astruct_57 *paVar11;
  u32 uVar12;
  astruct_57 *paVar13;
  astruct_57 *paVar14;
  u32 *puVar15;
  u16 unaff_SI;
  u16 uVar16;
  u32 *puVar17;
  StructA *pSVar18;
  astruct_27 *paVar19;
  u16 in_stack_0000fe88;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ff5c;
  u16 in_stack_0000ff60;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffb8;
  u8 uVar20;
  u8 uVar21;
  u16 uVar22;
  u16 uVar23;
  i16 iVar24;
  u32 *puStack16;

  paVar11 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar8 = param_5 - 0x1;
  uVar16 = (param_2 >> 0x10);
  iVar24 = (int)param_2;
  if (uVar8 == 0x0) {
    if (*(i32 *)(iVar24 + 0xfe) == 0x0) {
      mem_op_1000_179c(0xfc,paVar11);
      uVar10 = paVar11 | uVar8;
      uVar12 = (u32)paVar11 & 0xffff0000 | (u32)uVar10;
      if (uVar10 == 0x0) {
        (u32)(iVar24 + 0xfe) = 0x0;
      }
      else {
        piVar1 = (i16 *)(iVar24 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        unk_win_ui_op_1020_67ce
                  ((StructA *)CONCAT13((char)((u32)paVar11 >> 0x8),CONCAT12((char)paVar11,uVar8)),
                   (iVar24 + 0xcc),param_2,uVar12);
        (iVar24 + 0xfe) = uVar8;
        (iVar24 + 0x100) = (int)uVar12;
      }
      pass1_1008_6978(uVar8,(u8 *)uVar12,param_2,0x0,(u32)(iVar24 + 0xfe));
      uVar3 = (u32)(iVar24 + 0xfe);
      uVar22 = uVar3;
      uVar23 = ((u32)uVar3 >> 0x10);
      uVar3 = (u32)(iVar24 + 0xfe);
      uVar16 = ((u32)uVar3 >> 0x10);
      puVar15 = (u32 *)uVar3;
      goto LAB_1020_53f3;
    }
  }
  else {
    if (param_5 == 0x2) {
      uVar5 = param_4 + 0xc;
      puVar17 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,uVar5),in_stack_0000fe8a,
                                in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
      paVar11 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)puVar17 >> 0x10);
      uVar6 = pass1_1018_0afa((u32)puVar17);
      if (uVar6 == 0x0) {
        piVar1 = (i16 *)(iVar24 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        UVar2 = (iVar24 + 0xcc);
        UVar7 = UVar2;
        mem_op_1000_179c(0xfe,paVar11);
        uVar8 = paVar11 | UVar7;
        paVar14 = (astruct_57 *)((u32)paVar11 & 0xffff0000);
        paVar13 = (astruct_57 *)((u32)paVar14 | (u32)uVar8);
        if (uVar8 == 0x0) {
          UVar7 = 0x0;
        }
        else {
          pass1_1020_289a((u16 *)CONCAT13((char)((u32)paVar11 >> 0x8),CONCAT12((char)paVar11,UVar7)),UVar2,param_2)
          ;
          paVar14 = paVar13;
        }
        puVar9 = (u8 *)paVar14;
        puStack16 = (u32 *)CONCAT22(puVar9,UVar7);
        uVar20 = SUB41(paVar14,0x0);
        uVar21 = (u8)((u32)paVar14 >> 0x8);
        pass1_1020_294a(puVar9,(astruct_665 *)CONCAT13(uVar21,CONCAT12(uVar20,UVar7)),param_3,uVar5);
        uVar3 = *puStack16;
        ppcVar4 = (code **)((int)uVar3 + 0x8);
        uVar8 = (**ppcVar4)(0x1000,UVar7,puVar9);
        pass1_1008_3e0e((StructA *)CONCAT13(uVar21,CONCAT12(uVar20,UVar7)));
        pass1_1008_6978(uVar8,(u8 *)paVar14,param_2,UVar2,CONCAT22(puVar9,UVar7));
        ppcVar4 = (code **)((int)uVar3 + 0xc);
        (**ppcVar4)(0x1008,(char)UVar7,uVar20,0x1);
      }
      else {
        pSVar18 = (StructA *)pass1_1018_0ad4((u32)puVar17);
        paVar14 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)pSVar18 >> 0x10);
        pass1_1008_3e0e(pSVar18);
      }
      pass1_1018_1662((u32)puVar17,0x0,0x0);
      uVar3 = (u32)(iVar24 + 0xce);
      BringWindowToTop16(*(HWND16 *)((int)uVar3 + 0x8));
      uVar5 = 0x1;
      iVar24 = 0x4;
      paVar19 = (astruct_27 *)
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe88,in_stack_0000ffac,
                                in_stack_0000ffb2,in_stack_0000ffb6);
      pass1_1010_089e(paVar19,uVar5,iVar24);
      pass1_1010_089e(paVar19,0x1,0x3);
      return;
    }
    uVar8 = param_5 - 0x3;
    if ((uVar8 == 0x0) && (*(i32 *)(iVar24 + 0x102) == 0x0)) {
      mem_op_1000_179c(0xfc,paVar11);
      uVar10 = paVar11 | uVar8;
      uVar12 = (u32)paVar11 & 0xffff0000 | (u32)uVar10;
      if (uVar10 == 0x0) {
        (u32)(iVar24 + 0x102) = 0x0;
      }
      else {
        piVar1 = (i16 *)(iVar24 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        pass1_1020_0dc4((StructA *)CONCAT13((char)((u32)paVar11 >> 0x8),CONCAT12((char)paVar11,uVar8)),
                        (iVar24 + 0xcc),param_2,uVar12,in_stack_0000ff5c,in_stack_0000ff60);
        (iVar24 + 0x102) = uVar8;
        (iVar24 + 0x104) = (int)uVar12;
      }
      pass1_1008_6978(uVar8,(u8 *)uVar12,param_2,0x0,(u32)(iVar24 + 0x102));
      uVar3 = (u32)(iVar24 + 0x102);
      uVar22 = uVar3;
      uVar23 = ((u32)uVar3 >> 0x10);
      uVar3 = (u32)(iVar24 + 0x102);
      uVar16 = ((u32)uVar3 >> 0x10);
      puVar15 = (u32 *)uVar3;
LAB_1020_53f3:
      ppcVar4 = (code **)((int)*puVar15 + 0xc);
      (**ppcVar4)(0x8,uVar22,uVar23,0x5);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 post_msg_1020_55b0(StructD *param_1,u32 param_2,u16 param_3,i16 param_4)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  astruct_57 *paVar4;
  u16 uVar5;
  u16 uVar6;
  u16 *puVar7;
  char *pcVar8;
  u32 *puVar6;
  LRESULT LVar9;
  u32 *puVar10;
  u16 in_stack_0000fd80;
  u16 in_stack_0000fd82;
  u16 in_stack_0000fea4;
  u16 in_stack_0000fea6;
  u16 in_stack_0000feaa;
  u16 in_stack_0000feac;
  u16 in_stack_0000feae;
  u16 in_stack_0000feb0;
  astruct_57 *paStack288;
  u32 *puStack284;
  WPARAM16 local_114;
  u8 local_112 [0x2];
  i16 iStack272;
  i16 local_10e;
  char local_10c [0x100];
  u32 *puStack12;
  i16 iStack8;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2),in_stack_0000fd80,
                             in_stack_0000fea4,in_stack_0000feaa,in_stack_0000feae);
  paVar4 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puStack6 >> 0x10);
  iStack8 = ((int)puStack6 + 0x20);
  puStack12 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x37),in_stack_0000fd80,
                              in_stack_0000fea4,in_stack_0000feaa,in_stack_0000feae);
  uVar6 = ((u32)paVar4 >> 0x10);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_10c,(short)&DAT_1050_1050);
  puVar7 = pass1_1008_9436((u16 *)CONCAT22(0x1050,local_112));
  pcVar8 = (char *)pass1_1008_a8f4(((u32)puVar7 >> 0x10),(u32)puStack12,
                                   (u16 *)CONCAT22(0x1050,&local_114),(u16 *)CONCAT22(0x1050,local_112),
                                   (u16 *)CONCAT22(0x1050,&local_10e));
  uVar2 = pcVar8;
  paVar4 = (astruct_57 *)CONCAT22(uVar6,((u32)pcVar8 >> 0x10) | uVar2);
  uVar5 = (param_2 >> 0x10);
  if ((pcVar8 != NULL) && (*pcVar8 != '\0')) {
    uVar6 = 0x1000;
    mem_op_1000_179c(0xb4,paVar4);
    paStack288 = (astruct_57 *)CONCAT22(paVar4,uVar2);
    uVar2 = paVar4 | uVar2;
    paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000);
    if (uVar2 == 0x0) {
      puVar6._0_2_ = 0x0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      puVar6 = (u32 *)pass1_1040_8478(uVar2,paStack288,0x0,(char *)CONCAT13(0x10,CONCAT12(0x50,local_10c)),pcVar8,
                                        ((int)param_2 + 0x8));
      paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar6 >> 0x10);
      puVar6._0_2_ = SUB42(puVar6,0x0);
    }
    uVar3 = SUB42(paVar4,0x0);
    puStack284 = (u32 *)CONCAT22(uVar3,puVar6._0_2_);
    if (iStack272 == 0x0) {
      ppcVar1 = (code **)((int)*puStack284 + 0x74);
      (**ppcVar1)(uVar6,puVar6._0_2_,uVar3);
    }
    else {
      ppcVar1 = (code **)((int)*puStack284 + 0x6c);
      (**ppcVar1)(uVar6,(char)puVar6._0_2_,uVar3,local_112,(int)&DAT_1050_1050);
    }
    if ((iStack8 == 0x0) || (local_114 == 0x0)) {
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  if ((iStack8 != 0x0) && (local_114 != 0x0)) {
    LVar9 = SendMessage16(0x0,local_114,0x111,HWND16_1050_0396);
    ((int)param_2 + 0x112) = 0x1;
    return ((u32)LVar9 >> 0x10);
  }
  if (local_10e == 0x6) {
    PostMessage16(0x0,0xb0,0x111,HWND16_1050_0396);
    puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x2),in_stack_0000fd82,
                              in_stack_0000fea6,in_stack_0000feac,in_stack_0000feb0);
    paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar10 >> 0x10);
    param_4 = (int)puVar10;
    (param_4 + 0x20) = 0x1;
  }
  if (local_10e == 0x15) {
    PostMessage16(0x0,0x97,0x111,HWND16_1050_0396);
    puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x2),in_stack_0000fd82,
                              in_stack_0000fea6,in_stack_0000feac,in_stack_0000feb0);
    paVar4 = (astruct_57 *)((u32)puVar10 >> 0x10);
    ((int)puVar10 + 0x20) = 0x1;
  }
  return paVar4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void set_cursor_1020_5764(u32 param_1,i16 param_2)

{
  u16 uVar1;
  u32 uVar2;
  HCURSOR16 HVar3;
  astruct_57 *in_EDX;
  i16 iVar4;
  u16 uVar5;
  u16 in_stack_0000fe8e;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffe6;
  i16 local_e;
  u8 local_c [0x2];
  u32 uStack10;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x2f),in_stack_0000fe8e,
                             in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar5 = ((u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x20);
  uVar1 = ((int)puStack6 + 0x22);
  if ((uVar1 | uStack10) != 0x0) {
    pass1_1030_8308(&local_e,uVar1,_u16_1050_5748,((u32)_u16_1050_5748 >> 0x10),
                    (u16 *)CONCAT22(0x1050,&local_e),(u16 *)CONCAT22(0x1050,local_c),
                    CONCAT13((char)(uVar1 >> 0x8),CONCAT12((char)uVar1,uStack10)));
    if (param_2 <= local_e) {
      uVar5 = (param_1 >> 0x10);
      iVar4 = (int)param_1;
      if ((iVar4 + 0xf4) != 0x1) {
        SetCursor16(*(HCURSOR16 *)(iVar4 + 0xee));
        (iVar4 + 0xee) = 0x0;
        (iVar4 + 0xf4) = 0x1;
        (iVar4 + 0x10c) = 0x0;
        ReleaseCapture16();
      }
      HVar3 = LoadCursor16((char *)0x7f02,0x0);
      HVar3 = SetCursor16(HVar3);
      pass1_1018_017c((u32)puStack6,param_2);
      uVar2 = (u32)(iVar4 + 0xf6);
      ((int)uVar2 + 0x10) = 0x1;
      if (*(i32 *)(iVar4 + 0xfe) != 0x0) {
        pass1_1020_68de((u32)(iVar4 + 0xfe));
        uVar2 = (u32)(iVar4 + 0xfe);
        PostMessage16(0x0,0xeb,0x111,*(HWND16 *)((int)uVar2 + 0x8));
      }
      SetCursor16(HVar3);
    }
  }
  return;
}



void pt_in_rect_1020_5856(u16 param_1,u16 param_2,u32 param_3,POINT16 *param_4)

{
  u32 *puVar1;
  BOOL16 BVar2;
  u32 uVar3;
  u16 DX_REG;
  u32 uStack10;

  pass1_1018_2862(*(astruct_654 **)((int)param_3 + 0xfa));
  if ((param_2 | param_1) != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      puVar1 = (u32 *)(param_1 + 0xa);
      if (*puVar1 < uStack10 || *puVar1 == uStack10) break;
      uVar3 = uStack10;
      empty_1008_8fc4();
      if ((DX_REG | uVar3) != 0x0) {
        BVar2 = PtInRect16(*param_4,(RECT16 *)(uVar3 + 0x14));
        if (BVar2 != 0x0) {
          return;
        }
      }
      uStack10 += 0x1;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pt_in_rect_op_1020_58ce(u16 param_1,u32 param_2,u16 param_3,u16 param_4,u8 param_5)

{
  u16 uVar1;
  u32 uVar2;
  code **ppcVar3;
  u32 uVar4;
  u16 uVar5;
  BOOL16 BVar6;
  u16 *puVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  astruct_57 *paVar9;
  i16 iVar10;
  i16 iVar11;
  u16 uVar12;
  u16 uVar13;
  u32 uVar14;
  u16 *puVar15;
  u16 *puVar16;
  u32 *puVar17;
  u16 in_stack_0000fe74;
  u16 in_stack_0000ff98;
  u16 in_stack_0000ff9e;
  u16 in_stack_0000ffa2;
  WPARAM16 wparam;
  u16 in_stack_0000ffcc;
  u16 uStack46;
  i16 iStack26;
  u16 local_18 [0x2];
  u16 uStack20;
  u32 uStack18;
  RECT16 *pRStack14;
  u16 uStack12;
  u16 uStack10;
  u16 uStack8;
  u16 local_6;
  u16 uStack4;

  paVar9 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  local_6 = param_4;
  uStack4 = param_3;
  uStack8 = param_5 & 0x8;
  uStack10 = param_5 & 0x4;
  uVar12 = (param_2 >> 0x10);
  iVar10 = (int)param_2;
  uVar5 = pass1_1020_64d4((u32)(iVar10 + 0xf6),0x2);
  if (uVar5 == 0x0) {
LAB_1020_5942:
    uVar5 = pass1_1020_64d4((u32)(iVar10 + 0xf6),0x4);
    if (uVar5 == 0x0) {
LAB_1020_5a16:
      uVar5 = pass1_1020_64d4((u32)(iVar10 + 0xf6),0x1);
      if (uVar5 != 0x0) {
        uVar14 = pass1_1020_6498((u32)(iVar10 + 0xf6),0x1);
        paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | uVar14 >> 0x10);
        for (iStack26 = 0x0; iStack26 < 0x4; iStack26 += 0x1) {
          paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | uVar14 >> 0x10);
          BVar6 = PtInRect16((POINT16)CONCAT22(uStack4,local_6),(RECT16 *)(iStack26 * 0x8 + (int)uVar14));
          if (BVar6 != 0x0) {
            local_18[0] = 0x0;
            uStack20 = 0x0;
            if (iStack26 == 0x0) {
              uStack20 = (-(uStack10 == 0x0) & 0x4) - 0x5;
            }
            else if (iStack26 == 0x1) {
              uStack20 = (-(uStack10 == 0x0) & 0xfffc) + 0x5;
            }
            else if (iStack26 == 0x2) {
              local_18[0] = (-(uStack10 == 0x0) & 0x4) - 0x5;
            }
            else if (iStack26 == 0x3) {
              local_18[0] = (-(uStack10 == 0x0) & 0xfffc) + 0x5;
            }
            pass1_1020_2a94((u32)(iVar10 + 0xce),CONCAT22(local_18[0],uStack20));
            return;
          }
        }
      }
      uVar5 = pass1_1020_64d4((u32)(iVar10 + 0xf6),0x3);
      if (uVar5 != 0x0) {
        puVar7 = &local_6;
        pt_in_rect_1020_5856(puVar7,(int)paVar9,param_2,(POINT16 *)CONCAT22(0x1050,puVar7));
        uVar8 = paVar9;
        uVar14 = (u32)(uVar8 | puVar7);
        if ((uVar8 | puVar7) != 0x0) {
          uVar5 = puVar7[0x17];
          if (((uStack8 == 0x0) || (uStack10 == 0x0)) && (uStack10 == 0x0)) {
            local_18[0] = 0x1;
          }
          else {
            local_18[0] = 0x2;
          }
          uStack20 = puVar7[0x6];
          uStack18 = (u32 *)CONCAT22(uStack18,puVar7[0x7]);
          if ((uVar5 == 0xb) || (uVar5 == 0x37)) {
            uVar4 = (u32)(iVar10 + 0xfa);
            uVar13 = ((u32)uVar4 >> 0x10);
            iVar11 = (int)uVar4;
            uVar2 = (u32)(iVar11 + 0x20);
            uVar1 = (iVar11 + 0x22);
            uVar14 = (u32)paVar9 & 0xffff0000 | (u32)uVar1;
            uStack46 = uVar2;
            if ((uVar1 | uStack46) != 0x0) {
              puVar16 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&stack0xffcc));
              paVar9 = (astruct_57 *)(uVar14 & 0xffff0000 | (u32)puVar16 >> 0x10);
              pass1_1018_161c(uVar2,(u16 *)CONCAT22(0x1050,&stack0xffcc),(int)uStack18,uStack20);
              puVar17 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffcc,0x2f),
                                        in_stack_0000fe74,in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2);
              uVar14 = (u32)puVar17 >> 0x10;
              pass1_1010_ecc6(&stack0xffcc,((u32)puVar17 >> 0x10),(u32)puVar17,
                              (u16 *)CONCAT22(0x1050,&stack0xffcc),*(i32 *)(uStack46 + 0x3c));
            }
          }
          uVar13 = uVar14;
          uVar5 = pass1_1018_25d2((u32)(iVar10 + 0xfa),local_18[0],
                                  (u32)uStack18 & 0xffff | (u32)uStack20 << 0x10);
          if (uVar5 != 0x0) {
            return;
          }
          uVar5 = pass1_1020_5d56(uVar13,(u32 *)param_2,CONCAT22(uVar8,puVar7));
          if (uVar5 != 0x0) {
            return;
          }
        }
      }
      return;
    }
    uVar14 = pass1_1020_6498((u32)(iVar10 + 0xf6),0x4);
    paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | uVar14 >> 0x10);
    pRStack14 = (RECT16 *)uVar14;
    uStack12 = (uVar14 >> 0x10);
    BVar6 = PtInRect16((POINT16)CONCAT22(uStack4,local_6),pRStack14);
    if (BVar6 == 0x0) goto LAB_1020_5a16;
    uStack18 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffcc,0x2),in_stack_0000fe74,
                               in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2);
    if (((int)uStack18 + 0x72) != 0x0) {
      (iVar10 + 0x116) = 0x1;
      if (param_2 == 0x0) {
        iVar10 = 0x0;
        uVar12 = 0x0;
      }
      else {
        iVar10 += 0xe2;
      }
      ppcVar3 = (code **)((int)*_u16_1050_02a0 + 0x4);
      (**ppcVar3)(0x1010,(int)_u16_1050_02a0,(int)((u32)_u16_1050_02a0 >> 0x10),0x10,iVar10,uVar12);
      puVar15 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_18),0x1,0x86);
      puVar7 = local_18;
      win_1008_5c9e(puVar7,(int)((u32)puVar15 >> 0x10),(u32)_u16_1050_02a0,(u32 *)CONCAT22(0x1050,puVar7));
      if (puVar7 != NULL) {
        return;
      }
      wparam = 0xf6;
      goto LAB_1020_5936;
    }
    wparam = 0xf6;
  }
  else {
    uVar14 = pass1_1020_6498((u32)(iVar10 + 0xf6),0x2);
    paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | uVar14 >> 0x10);
    pRStack14 = (RECT16 *)uVar14;
    uStack12 = (uVar14 >> 0x10);
    BVar6 = PtInRect16((POINT16)CONCAT22(uStack4,local_6),pRStack14);
    if (BVar6 == 0x0) goto LAB_1020_5942;
    wparam = 0x68;
  }
  puVar7 = NULL;
LAB_1020_5936:
  PostMessage16(CONCAT22(puVar7,puVar7),wparam,0x111,HWND16_1050_0396);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 menu_ui_op_1020_5bf2(astruct_52 *param_1,INT16 param_2,INT16 param_3)

{
  u32 uVar1;
  u16 uVar2;
  BOOL16 BVar3;
  INT16 *pIVar4;
  HMENU16 HVar5;
  u16 in_DX;
  astruct_52 *iVar5;
  u16 uVar6;
  INT16 local_10;
  INT16 IStack14;
  i16 iStack12;
  u32 uStack10;
  INT16 local_6;
  INT16 IStack4;

  local_6 = param_3;
  IStack4 = param_2;
  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_52 *)param_1;
  uVar2 = pass1_1020_64d4(iVar5->field243_0xf6,0x2);
  if (uVar2 != 0x0) {
    uStack10 = pass1_1020_6498(iVar5->field243_0xf6,0x2);
    in_DX = (uStack10 >> 0x10);
    BVar3 = PtInRect16((POINT16)CONCAT22(IStack4,local_6),(RECT16 *)uStack10);
    if (BVar3 != 0x0) {
      PostMessage16(0x0,0x131,0x111,HWND16_1050_0396);
      return 0x1;
    }
  }
  uVar2 = pass1_1020_64d4(iVar5->field243_0xf6,0x3);
  if (uVar2 == 0x0) {
    return 0x0;
  }
  pIVar4 = &local_6;
  pt_in_rect_1020_5856(pIVar4,in_DX,(u32)param_1,(POINT16 *)CONCAT22(0x1050,pIVar4));
  iVar5->field257_0x108 = pIVar4;
  iVar5->field258_0x10a = in_DX;
  if ((in_DX | iVar5->field257_0x108) == 0x0) {
    return 0x0;
  }
  if (iVar5->field256_0x106 == 0x0) {
    HVar5 = LoadMenu16(s_TILEMENU_1050_43f0,HINSTANCE16_1050_038c);
    iVar5->field256_0x106 = HVar5;
    if (HVar5 == 0x0) {
      return 0x1;
    }
    HVar5 = GetSubMenu16(0x0,iVar5->field256_0x106);
    iVar5->field256_0x106 = HVar5;
    if (HVar5 == 0x0) {
      return 0x1;
    }
  }
  uVar1 = (u32)&iVar5->field257_0x108;
  uStack10._0_2_ = ((int)uVar1 + 0x2e);
  iStack12 = 0x0;
  if (uStack10 == 0x42) {
    iStack12 = 0xc9;
  }
  else if (((int)s_VrMode_1050_42ca + 0x8 + uStack10 * 0x2) == 0x0) {
    iStack12 = 0xc8;
  }
  if (iStack12 != 0x0) {
    win_1008_5c7c(uStack10,in_DX,_u16_1050_02a0,CONCAT22(iStack12,0x1));
  }
  local_10 = param_3;
  IStack14 = param_2;
  ClientToScreen16((POINT16 *)CONCAT22(0x1050,&local_10),iVar5->field8_0x8);
  TrackPopupMenu16(NULL,iVar5->field8_0x8,0x0,IStack14,local_10,0x0,iVar5->field256_0x106);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1020_5d56(u16 param_1,u32 *param_2,u32 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  i16 local_12 [0x2];
  i16 local_e;
  i16 local_c;
  i16 local_a [0x2];
  i16 iStack6;

  iStack6 = ((int)param_3 + 0x2e);
  uVar2 = param_2;
  uVar3 = ((u32)param_2 >> 0x10);
  if (iStack6 == 0x47) {
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,local_a));
    if (local_a[0] == 0x0) goto LAB_1020_5d8b;
    if (local_c <= local_a[0]) {
      return 0x1;
    }
  }
  else {
    if (iStack6 != 0x6a) {
      return 0x0;
    }
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(0x1050,&local_e),(u16 *)CONCAT22(0x1050,local_12));
    if (local_e <= local_12[0]) {
LAB_1020_5d8b:
      ppcVar1 = (code **)((int)*param_2 + 0x40);
      (**ppcVar1)();
      return 0x1;
    }
  }
  pass1_1038_af40(uVar2,param_1,_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),0x9);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1020_5de8(u32 param_1,u16 param_2,u16 param_3)

{
  u32 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  astruct_57 *in_EDX;
  i16 iVar5;
  u16 uVar6;
  u32 *puVar7;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffe4;
  u16 uStack18;
  u8 bStack15;
  u16 local_6;
  u16 uStack4;

  ReleaseCapture16();
  uVar6 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  SetCursor16(*(HCURSOR16 *)(iVar5 + 0xee));
  (iVar5 + 0xee) = 0x0;
  (iVar5 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar7 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe4,0x47),in_stack_0000fe8c,
                           in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar3 = ((u32)puVar7 >> 0x10);
  puVar2 = &local_6;
  pt_in_rect_1020_5856(puVar2,uVar3,param_1,(POINT16 *)CONCAT22(0x1050,puVar2));
  uVar4 = uVar3 | puVar2;
  if (uVar4 != 0x0) {
    uVar1 = (u32)(puVar2 + 0x21);
    uVar4 = puVar2[0x22];
    bStack15 = (u8)((u32)uVar1 >> 0x18);
    puVar2 = (u16 *)bStack15;
    if (bStack15 == 0x5) {
      uStack18 = uVar1;
      uVar3 = uVar4;
      goto LAB_1020_5e62;
    }
  }
  uStack18 = 0x0;
  uVar3 = 0x0;
LAB_1020_5e62:
  pass1_1018_57e6((u32)puVar7,CONCAT22(uVar3,uStack18),puVar2,uVar4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1020_5e76(u32 param_1,u16 param_2,u16 param_3)

{
  code **ppcVar1;
  u16 *puVar2;
  u8 *puVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u32 in_EDX;
  i16 iVar8;
  u32 *puVar9;
  u32 *puVar10;
  u16 uVar11;
  u16 uVar12;
  u32 *puVar13;
  u8 uVar14;
  u8 in_AF;
  astruct_57 *paVar15;
  u32 *puVar16;
  u16 in_stack_0000fc00;
  u16 in_stack_0000fd24;
  u16 in_stack_0000fd2a;
  u16 in_stack_0000fd2e;
  char *pcVar17;
  u16 uVar18;
  u16 in_stack_0000fd58;
  u8 *local_1aa [0x80];
  char local_aa [0x80];
  u32 uStack42;
  u32 uStack38;
  char local_22 [0x10];
  u8 *puStack18;
  u16 uStack16;
  u16 uStack14;
  u16 uStack12;
  u32 uStack10;
  u16 local_6;
  u16 uStack4;
  u32 uVar7;

  ReleaseCapture16();
  uVar11 = (param_1 >> 0x10);
  iVar8 = (int)param_1;
  SetCursor16(*(HCURSOR16 *)(iVar8 + 0xee));
  (iVar8 + 0xee) = 0x0;
  (iVar8 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar2 = &local_6;
  pt_in_rect_1020_5856(puVar2,(int)in_EDX,param_1,(POINT16 *)CONCAT22(0x1050,puVar2));
  uVar5 = in_EDX;
  uStack10 = CONCAT22(uVar5,puVar2);
  paVar15 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)(uVar5 | puVar2));
  if ((uVar5 | puVar2) == 0x0) goto LAB_1020_6176;
  uStack12 = puVar2[0x6];
  uStack14 = puVar2[0x7];
  uStack16 = 0x0;
  puVar3 = pass1_1018_2580(in_AF,(u32)(iVar8 + 0xfa),0x0,CONCAT22(uStack12,uStack14),(iVar8 + 0x10c));
  if (puVar3 == (u8 *)0x6b2) goto LAB_1020_6176;
  puStack18 = puVar3;
  if (puVar3 == (u8 *)0x6b8) {
    mem_op_1000_179c(0xb4,paVar15);
    uStack42 = CONCAT22(paVar15,puVar3);
    uVar5 = paVar15 | puVar3;
    uVar7 = (u32)paVar15 & 0xffff0000 | (u32)uVar5;
    if (uVar5 == 0x0) {
      iVar4 = 0x0;
      uVar12 = 0x0;
    }
    else {
      iVar4 = string_1040_8520(uVar7,(astruct_57 *)
                                     CONCAT13((char)((u32)paVar15 >> 0x8),CONCAT12((char)paVar15,puVar3)),
                               HWND16_1050_0396,0x20040,0x6ad06b8);
      uVar12 = uVar7;
    }
    uStack38 = CONCAT22(uVar12,iVar4);
    uVar18 = 0xa5;
LAB_1020_5f84:
    pass1_1008_941a((u16 *)CONCAT13(0x10,CONCAT12(0x50,local_22)),0x1,uVar18);
    pcVar17 = local_22;
    uVar12 = (uStack38 >> 0x10);
    puVar9 = (u32 *)uStack38;
  }
  else {
    if (puVar3 == (u8 *)0x6b4) {
      mem_op_1000_179c(0xb4,paVar15);
      uStack42 = CONCAT22(paVar15,puVar3);
      uVar5 = paVar15 | puVar3;
      uVar7 = (u32)paVar15 & 0xffff0000 | (u32)uVar5;
      if (uVar5 == 0x0) {
        iVar4 = 0x0;
        uVar12 = 0x0;
      }
      else {
        iVar4 = string_1040_8520(uVar7,(astruct_57 *)
                                       CONCAT13((char)((u32)paVar15 >> 0x8),CONCAT12((char)paVar15,puVar3)),
                                 HWND16_1050_0396,0x20040,CONCAT22(puStack18,0x57b));
        uVar12 = uVar7;
      }
      uStack38 = CONCAT22(uVar12,iVar4);
      uVar18 = 0xab;
      goto LAB_1020_5f84;
    }
    if (puVar3 == (u8 *)0x6b6) {
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_aa,(short)&DAT_1050_1050);
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)local_1aa,
                 (short)&DAT_1050_1050);
      uVar5 = sys_1000_3f9c((char *)CONCAT13(0x10,CONCAT12(0x50,&stack0xfd56)),(char *)CONCAT22(0x1050,local_1aa),
                            PTR_LOOP_1050_50cc);
      uVar14 = 0x0;
      mem_op_1000_179c(0xb4,paVar15);
      uVar6 = paVar15;
      uStack42 = CONCAT22(uVar6,uVar5);
      if ((uVar6 | uVar5) == 0x0) {
        puVar9 = NULL;
        paVar15 = NULL;
      }
      else {
        uVar14 = 0x40;
        paVar15 = pass1_1040_8478(uVar6 | uVar5,(astruct_57 *)CONCAT22(uVar6,uVar5),0x40,
                                  (char *)CONCAT13(0x10,CONCAT12(0x50,local_aa)),(char *)CONCAT22(0x1050,&stack0xfd56),
                                  HWND16_1050_0396);
        puVar9 = (u32 *)paVar15;
      }
      uStack38 = (u32)paVar15 & 0xffff0000 | ZEXT24(puVar9);
      puVar10 = puVar9;
      puVar13 = (u32 *)(((u32)paVar15 & 0xffff0000) >> 0x10);
LAB_1020_6027:
      ppcVar1 = (code **)((int)*puVar10 + 0x74);
      (**ppcVar1)(uVar14,puVar9);
      goto LAB_1020_6176;
    }
    if (puVar3 < (u8 *)0x6a7) {
      if (((iVar8 + 0x10c) == 0x78) || ((iVar8 + 0x10c) == 0x74)) {
        puVar16 = mixed_1010_20ba(paVar15,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fd58,0x5),in_stack_0000fc00
                                  ,in_stack_0000fd24,in_stack_0000fd2a,in_stack_0000fd2e);
        paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)puVar16 >> 0x10);
        in_stack_0000fd58 = ((u32)puVar16 >> 0x10);
        if (((int)puVar16 + 0xa) == 0x0) {
          return;
        }
      }
      if ((((((iVar8 + 0x10c) == 0x6c) || ((iVar8 + 0x10c) == 0x6d)) ||
           ((iVar8 + 0x10c) == 0x31)) || ((iVar8 + 0x10c) == 0x32)) &&
         (puVar16 = mixed_1010_20ba(paVar15,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fd58,0x3a),
                                    in_stack_0000fc00,in_stack_0000fd24,in_stack_0000fd2a,in_stack_0000fd2e),
         ((int)puVar16 + 0xa) == 0x0)) {
        return;
      }
      pass1_1020_68de((u32)(iVar8 + 0xfe));
      goto LAB_1020_6176;
    }
    if ((u8 *)0x6b1 < puVar3) {
      uVar14 = 0x0;
      mem_op_1000_179c(0xb4,paVar15);
      uStack42 = CONCAT22(paVar15,puVar3);
      uVar5 = paVar15 | puVar3;
      uVar7 = (u32)paVar15 & 0xffff0000 | (u32)uVar5;
      if (uVar5 == 0x0) {
        puVar9 = NULL;
        puVar10 = NULL;
        puVar13 = puVar10;
      }
      else {
        uVar14 = 0x40;
        puVar9 = (u32 *)
                 string_1040_8520(uVar7,(astruct_57 *)
                                        CONCAT13((char)((u32)paVar15 >> 0x8),CONCAT12((char)paVar15,puVar3)),
                                  HWND16_1050_0396,0x20040,CONCAT22(puStack18,0x57b));
        puVar10 = (u32 *)uVar7;
        puVar13 = puVar10;
      }
      goto LAB_1020_6027;
    }
    mem_op_1000_179c(0xb4,paVar15);
    uStack42 = CONCAT22(paVar15,puVar3);
    uVar5 = paVar15 | puVar3;
    uVar7 = (u32)paVar15 & 0xffff0000 | (u32)uVar5;
    if (uVar5 == 0x0) {
      uVar12 = 0x0;
    }
    else {
      string_1040_8520(uVar7,(astruct_57 *)CONCAT13((char)((u32)paVar15 >> 0x8),CONCAT12((char)paVar15,puVar3)),
                       HWND16_1050_0396,0x20040,CONCAT22(puStack18,0x57b));
      uVar12 = uVar7;
    }
    local_1aa[0] = puStack18 + -0x608;
    pass1_1008_941a((u16 *)CONCAT13(0x10,CONCAT12(0x50,local_aa)),0x1,local_1aa[0]);
    pcVar17 = local_aa;
    puVar9 = (u32 *)&DAT_1050_1050;
  }
  ppcVar1 = (code **)((int)*puVar9 + 0x6c);
  (**ppcVar1)(0x1008,(char)puVar9,(char)uVar12,pcVar17);
LAB_1020_6176:
  (iVar8 + 0x10c) = 0x0;
  return;
}



void pass1_1020_6184(u32 param_1,u16 param_2)

{
  HCURSOR16 HVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xf4) == 0x1) {
    HVar1 = SetCursor16(*(HCURSOR16 *)(iVar2 + 0xf0));
    *(HCURSOR16 *)(iVar2 + 0xee) = HVar1;
    (iVar2 + 0x10c) = param_2;
    SetCapture16(*(HWND16 *)(iVar2 + 0x8));
    (iVar2 + 0xf4) = 0x2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_61c4(u16 param_1,u16 param_2,u32 param_3,u16 *param_4)

{
  u32 uVar1;
  u16 uVar2;
  astruct_57 *in_EDX;
  u32 *puVar3;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000fff2;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x2f),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar2 = ((u32)puVar3 >> 0x10);
  uVar1 = (u32)((int)puVar3 + 0x20);
  pass1_1030_8308(uVar1,uVar2,_u16_1050_5748,((u32)_u16_1050_5748 >> 0x10),(u16 *)param_3,
                  param_4,uVar1);
  *param_4 = ((int)puVar3 + 0x1e);
  return;
}



StructD * pass1_1020_6208(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  destroy_cursor_1020_42f4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1020_6216(u16 param_1,StructD *param_2,u8 param_3)

{
  destroy_cursor_1020_42f4(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_62e0(i16 param_1,u16 param_2,u16 param_3)

{
  u32 *puVar1;
  code **ppcVar2;
  u16 *puVar3;
  u32 uVar4;
  u16 uVar5;
  u16 uVar6;
  astruct_57 *in_EDX;
  astruct_57 *paVar8;
  astruct_57 *paVar9;
  u32 *puVar10;
  u16 in_stack_0000fe3e;
  u16 in_stack_0000fe84;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ff62;
  u16 in_stack_0000ff68;
  u16 in_stack_0000ff6c;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u32 in_stack_0000ffc6;
  u16 uVar11;
  u16 uVar12;
  u16 uVar13;
  i16 iVar14;
  u16 uVar15;
  u16 in_stack_0000ffee;
  astruct_57 *paVar7;

  set_struct_op_1020_921c(in_EDX,(StructA *)CONCAT22(param_2,param_1),param_3,in_stack_0000ffc6);
  (u32)(param_1 + 0x14) = 0x0;
  (u32)(param_1 + 0x2c) = 0x0;
  CONCAT22(param_2,param_1) = 0x67c2;
  (param_1 + 0x2) = 0x1020;
  puVar3 = pass1_1000_4906((StructD *)CONCAT22(param_2,param_1 + 0x18),NULL,0x14);
  mem_op_1000_179c(0x3c,in_EDX);
  uVar5 = in_EDX | puVar3;
  paVar7 = (astruct_57 *)((u32)in_EDX & 0xffff0000 | (u32)uVar5);
  if (uVar5 == 0x0) {
    (u32)(param_1 + 0x1c) = 0x0;
  }
  else {
    pass1_1020_87c2((astruct_20 *)CONCAT22(in_EDX,puVar3));
    (u16*)(param_1 + 0x1c) = puVar3;
    (param_1 + 0x1e) = (int)paVar7;
  }
  mem_op_1000_179c(0x26,paVar7);
  uVar5 = paVar7 | puVar3;
  paVar9 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar9 | (u32)uVar5);
  if (uVar5 == 0x0) {
    puVar3 = NULL;
  }
  else {
    pass1_1020_8a9c((astruct_20 *)CONCAT22(paVar7,puVar3),paVar8,in_stack_0000ff68,in_stack_0000ff6c,
                    in_stack_0000ff62,in_stack_0000fe3e);
    paVar9 = paVar8;
  }
  (u16*)(param_1 + 0x20) = puVar3;
  (param_1 + 0x22) = (int)paVar9;
  mem_op_1000_179c(0xbe,paVar9);
  uVar5 = paVar9 | puVar3;
  paVar7 = (astruct_57 *)((u32)paVar9 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar7 | (u32)uVar5);
  if (uVar5 == 0x0) {
    puVar3 = NULL;
  }
  else {
    pass1_1020_8eaa((astruct_20 *)CONCAT22(paVar9,puVar3),paVar8);
    paVar7 = paVar8;
  }
  (u16*)(param_1 + 0x24) = puVar3;
  (param_1 + 0x26) = (int)paVar7;
  mem_op_1000_179c(0x20,paVar7);
  uVar5 = paVar7 | puVar3;
  paVar9 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar9 | (u32)uVar5);
  if (uVar5 == 0x0) {
    puVar3 = NULL;
  }
  else {
    pass1_1020_8360((astruct_20 *)CONCAT22(paVar7,puVar3));
    paVar9 = paVar8;
  }
  (u16*)(param_1 + 0x28) = puVar3;
  (param_1 + 0x2a) = (int)paVar9;
  pass1_1020_6746(CONCAT22(param_2,param_1),0x1,0x4);
  puVar10 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x29),in_stack_0000fe96,
                            in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  paVar7 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)puVar10 >> 0x10);
  (param_1 + 0x14) = (int)puVar10;
  uVar6 = ((u32)puVar10 >> 0x10);
  (param_1 + 0x16) = uVar6;
  uVar13 = 0x0;
  uVar12 = (param_1 + 0x14);
  ppcVar2 = (code **)((int)(u32)(u32)(param_1 + 0x14) + 0x4);
  iVar14 = param_1;
  uVar15 = param_2;
  (**ppcVar2)();
  (u32)(param_1 + 0x6) = (u32)(param_1 + 0x14);
  uVar4 = (u32)(param_1 + 0x14);
  puVar1 = (u32 *)(u32)((int)uVar4 + 0xa);
  uVar4 = CONCAT22(param_2,param_1 + 0xa);
  uVar11 = SUB42(puVar1,0x0);
  ppcVar2 = (code **)((int)*puVar1 + 0x8);
  (**ppcVar2)(0x1010,uVar11,(int)((u32)puVar1 >> 0x10),uVar4,uVar12,uVar6,uVar13,iVar14,uVar15);
  (param_1 + 0x12) = (int)uVar4;
  (param_1 + 0x10) = 0x1;
  puVar10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(uVar11,0x2),in_stack_0000fe84,in_stack_0000ffa8
                            ,in_stack_0000ffae,in_stack_0000ffb2);
  (param_1 + 0x2c) = (int)puVar10;
  (param_1 + 0x2e) = (int)((u32)puVar10 >> 0x10);
  return;
}



void pass1_1020_6466(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x67c2;
  iVar1->address_offset_field_0x2 = 0x1020;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1ea6(iVar1->field12_0x14,(StructD *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  }
  palette_op_1020_92c4(param_1);
  return;
}



u32 pass1_1020_6498(u32 param_1,i16 param_2)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x18 + param_2 * 0x4);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22((iVar2 + 0xa),(iVar2 + 0x8));
  }
  return 0x0;
}



u16 pass1_1020_64d4(u32 param_1,i16 param_2)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x18 + param_2 * 0x4);
    return ((int)uVar1 + 0x4);
  }
  return 0x0;
}



void mix_draw_op_1020_650c(StructA *param_1)

{
  code **ppcVar1;
  u32 uVar2;
  StructA *struct_1;
  u16 uVar3;
  u8 local_28 [0x20];
  i16 iStack8;
  u32 *puStack6;

  uVar3 = ((u32)param_1 >> 0x10);
  struct_1 = (StructA *)param_1;
  uVar2 = (u32)&struct_1->field10_0x14;
  puStack6 = (u32 *)(u32)((int)uVar2 + 0xa);
  if ((struct_1->field8_0x10 != 0x0) ||
     (uVar2 = (u32)&struct_1->field10_0x14, ((int)uVar2 + 0x24) != 0x0)) {
    draw_op_1020_9364(param_1);
    if (*(i32 *)&struct_1->field19_0x24 != 0x0) {
      uVar2 = (u32)&struct_1->field19_0x24;
      ppcVar1 = (code **)((int)*(u32*)&struct_1->field19_0x24 + 0x14);
      (**ppcVar1)(0x1020,(int)uVar2,(int)((u32)uVar2 >> 0x10));
    }
  }
  iStack8 = 0x0;
  do {
    if (*(i32 *)(&struct_1->field12_0x18 + iStack8 * 0x2) != 0x0) {
      uVar2 = (u32)(&struct_1->field12_0x18 + iStack8 * 0x2);
      ppcVar1 = (code **)((int)*(u32*)(&struct_1->field12_0x18 + iStack8 * 0x2) + 0x8);
      (**ppcVar1)(0x1020,(char)uVar2,(int)((u32)uVar2 >> 0x10),(int)puStack6,(int)((u32)puStack6 >> 0x10));
    }
    iStack8 += 0x1;
  } while (iStack8 < 0x5);
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_28),struct_1->field2_0x4);
  ppcVar1 = (code **)((int)*puStack6 + 0x4);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack6,(char)((u32)puStack6 >> 0x10),0x0,0x0,&struct_1->field5_0xa,
              uVar3);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_28),struct_1->field2_0x4);
  return;
}



void unk_win_op_1020_65cc(astruct_60 *param_1,i16 param_2)

{
  code **ppcVar1;
  u32 uVar2;
  BOOL16 BVar3;
  u16 uVar4;
  astruct_59 *iVar4;
  astruct_60 *iVar5;
  i16 iVar6;
  astruct_60 *uVar7;
  i16 iStack4;

  iVar5 = (astruct_60 *)param_1;
  uVar7 = (astruct_60 *)((u32)param_1 >> 0x10);
  if (param_2 == 0x1) {
    iVar5->field20_0x14 = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    for (iStack4 = 0x0; iStack4 < 0x5; iStack4 += 0x1) {
      iVar6 = iStack4 * 0x4;
      if (((&iVar5->field_0x1a + iVar6) | (&iVar5->field_0x18 + iVar6)) != 0x0) {
        ppcVar1 = (code **)((int)*(u32*)(&iVar5->field_0x18 + iVar6) + 0x4);
        (**ppcVar1)();
      }
    }
  }
  else if (((0x0 < param_2 + -0x3) && (!SBORROW2(param_2 + -0x3,0x1))) && (param_2 + -0x4 < 0x4)) {
    BVar3 = IsIconic16(HWND16_1050_0396);
    if (BVar3 == 0x0) {
      BVar3 = IsIconic16(*(HWND16 *)&iVar5->field_0x4);
      if ((BVar3 == 0x0) && (uVar2 = iVar5->field20_0x14, ((int)uVar2 + 0x24) != 0x0)) {
        InvalidateRect16(0x0,NULL,0x0);
        uVar4 = pass1_1020_64d4((u32)param_1,0x2);
        if (uVar4 == 0x0) {
          pass1_1020_6746((u32)param_1,0x1,0x2);
        }
        uVar4 = pass1_1020_64d4((u32)param_1,0x3);
        if (uVar4 == 0x0) {
          pass1_1020_6746((u32)param_1,0x1,0x3);
        }
        uVar4 = pass1_1018_255e(iVar5->field20_0x14);
        if (uVar4 == 0x0) {
          SendMessage16(0x0,0x69,0x111,*(HWND16 *)&iVar5->field_0x4);
        }
        else {
          uVar4 = pass1_1020_64d4((u32)param_1,0x1);
          if (uVar4 == 0x0) {
            pass1_1020_6746((u32)param_1,0x1,0x1);
          }
        }
        SendMessage16(0x0,0xf0,0x111,*(HWND16 *)&iVar5->field_0x4);
        uVar2 = iVar5->field41_0x2c;
        if (((int)uVar2 + 0x7a) != 0x0) {
          uVar2 = iVar5->field41_0x2c;
          ((int)uVar2 + 0x7a) = 0x0;
          SendMessage16(0x0,0x131,0x111,*(HWND16 *)&iVar5->field_0x4);
          return;
        }
      }
    }
  }
  return;
}



void pass1_1020_6746(u32 param_1,i16 param_2,i16 param_3)

{
  code **ppcVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;

  if (param_3 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = (int)param_1;
    if (*(i32 *)(iVar3 + 0x18 + param_3 * 0x4) != 0x0) {
      uVar2 = (u32)(iVar3 + 0x18 + param_3 * 0x4);
      ((int)uVar2 + 0x4) = param_2;
      (iVar3 + 0x10) = 0x1;
      if (param_2 == 0x0) {
        ppcVar1 = (code **)((int)(u32)(u32)(iVar3 + 0x18 + param_3 * 0x4) + 0x14);
        (**ppcVar1)();
      }
    }
  }
  return;
}



StructD * pass1_1020_679c(StructD *param_1,u8 param_2)

{
  pass1_1020_6466(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void unk_win_ui_op_1020_67ce(StructA *in_struct_1,u16 param_2,u32 param_3,u32 param_4)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 hcursor_2;
  StructA *struct_a_1_lo;
  StructA *struct_a_1_hi;
  u16 in_stack_0000fe10;
  u16 in_stack_0000fe14;
  u16 in_stack_0000fe66;
  u16 in_stack_0000ff3a;
  u16 in_stack_0000ff3e;
  u16 in_stack_0000ff42;
  u16 in_stack_0000ff82;
  u16 in_stack_0000ff8a;
  u16 in_stack_0000ff90;
  u16 in_stack_0000ff94;

  struct_1020_790e(&in_struct_1->field0_0x0,s_TPPOPMENU_1050_43fa,param_2,param_3);
  struct_a_1_hi = (StructA *)((u32)in_struct_1 >> 0x10);
  struct_a_1_lo = (StructA *)in_struct_1;
  (u32)&struct_a_1_lo[0x1].field20_0x26 = 0x0;
  (u32)&struct_a_1_lo[0x1].field22_0x2a = 0x0;
  in_struct_1->field0_0x0 = 0x70e6;
  struct_a_1_lo->field1_0x2 = 0x1020;
  unk_str_op_1000_3d3e
            ((char *)((u32)in_struct_1 & 0xffff0000 | ZEXT24(&struct_a_1_lo->field60_0x5b)),s_VrMode2_1050_4404);
  HVar1 = GetStockObject16(HOLLOW_BRUSH);
  struct_a_1_lo->field157_0xc6 = HVar1;
  hcursor_2 = LoadCursor16((char *)0x7f00,0x0);
  struct_a_1_lo->field156_0xc4 = hcursor_2;
  struct_a_1_lo->field140_0xac = 0x44c00000;
  struct_a_1_lo->field158_0xc8 = 0x2020;
  struct_a_1_lo->field149_0xbc = ((int)param_3 + 0x8);
  struct_a_1_lo->field159_0xca = param_2;
  win_ui_reg_class_1008_96d2(in_struct_1);
  window_op_1020_6c3a(param_4,in_struct_1,in_stack_0000fe66,in_stack_0000ff82,in_stack_0000ff8a,in_stack_0000ff90,
                      in_stack_0000ff94,in_stack_0000fe10,in_stack_0000fe14,in_stack_0000ff3a,in_stack_0000ff3e,
                      in_stack_0000ff42);
  return;
}



void pass1_1020_687c(astruct_868 *param_1)

{
  u8 unaff_BP;

  get_win_ui_info_op_1020_7a50(param_1);
  destroy_icon_1020_6bd2(param_1,unaff_BP);
  return;
}



// WARNING: Unable to use type for symbol uVar2

void realize_palette_1020_6896(u32 param_1,i16 param_2)

{
  code **ppcVar1;
  u32 uVar3;
  u32 puVar4;
  astruct_801 *iVar4;
  u16 uVar4;
  u16 uVar5;
  u32 uVar2;

  if (param_2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    uVar2 = (u32)((int)param_1 + 0xf2);
    uVar5 = ((u32)uVar2 >> 0x10);
    iVar4 = (astruct_801 *)uVar2;
    puVar4 = (u32)iVar4->field36_0x24;
    ppcVar1 = (code **)((int)(u32)puVar4 + 0x18);
    (**ppcVar1)();
    UnrealizeObject16((HGDIOBJ16)puVar4);
    uVar3 = (u32)((int)param_1 + 0xf2);
    RealizePalette16(*(HDC16 *)((int)uVar3 + 0x178));
  }
  return;
}



void pass1_1020_68de(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0xf6) != 0x0) {
    invalidate_rect_1020_735a((u32)((int)param_1 + 0xf6));
  }
  return;
}



void pt_in_rect_1020_68fc(u32 *param_1,u16 param_2,u16 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  BOOL16 BVar3;
  u16 uVar4;
  POINT16 PStack6;

  PStack6 = (POINT16)CONCAT22(param_2,param_3);
  uVar4 = ((u32)param_1 >> 0x10);
  uVar2 = pass1_1018_31d0(*(astruct_126 **)((int)param_1 + 0xf2));
  if (uVar2 != 0x0) {
    BVar3 = PtInRect16(PStack6,(RECT16 *)((int)(u32)((int)param_1 + 0xf2) + 0x16c));
    if (BVar3 != 0x0) {
      ppcVar1 = (code **)((int)*param_1 + 0x40);
      (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,0xef);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 unk_destroy_win_op_1020_694c(u16 param_1,StructA *param_2,u16 param_3)

{
  u16 uVar1;
  u16 uVar2;
  BOOL16 BVar3;
  HWND16 HVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  u32 uVar6;
  StructA *iVar5;
  StructA *uVar4;
  u16 unaff_CS;
  WPARAM16 wparam;

  uVar6 = CONCAT22(in_register_0000000a,param_1);
  uVar1 = param_3;
  if (param_3 != 0x12b) {
    iVar5 = (StructA *)param_2;
    uVar4 = (StructA *)((u32)param_2 >> 0x10);
    if (param_3 < 0x12c) {
      if (param_3 == 0x6f) {
        uVar2 = FUN_1010_830a(0x0,uVar6,unaff_CS,_u16_1050_14cc,0x1f8);
        BVar3 = WinHelp16(0x29,0x1,(char *)CONCAT22((int)uVar6,uVar2),iVar5->field4_0x8);
        return BVar3;
      }
      if (param_3 == 0xeb) {
        uVar1 = GetDlgItem16(0x1797,iVar5->field4_0x8);
        uVar5 = uVar6;
        if (uVar1 != 0x0) {
LAB_1020_6a6f:
          win_ui_fn_1020_6e98(uVar5,param_2);
          return uVar1;
        }
      }
      else {
        uVar1 = param_3 - 0xef;
        if (uVar1 == 0x0) {
          pass1_1018_2e28(*(astruct_126 **)&iVar5[0x1].field20_0x26);
          pass1_1008_3e0e(param_2);
        }
        else {
          uVar1 = param_3 - 0x129;
          if ((uVar1 != 0x0) && (uVar1 = param_3 - 0x12a, uVar1 == 0x0)) {
            HVar4 = iVar5->field4_0x8;
            wparam = 0xf012;
LAB_1020_69c3:
            BVar3 = PostMessage16(0x0,wparam,0x112,HVar4);
            return BVar3;
          }
        }
      }
    }
    else if (param_3 == 0xbb8) {
      HVar4 = GetDlgItem16(0x1797,iVar5->field4_0x8);
      if (HVar4 != 0x0) {
        DestroyWindow16(HVar4);
      }
      uVar1 = pass1_1018_31d0(*(astruct_126 **)&iVar5[0x1].field20_0x26);
      if (uVar1 != 0x0) {
        uVar1 = pass1_1018_2d9a(*(astruct_126 **)&iVar5[0x1].field20_0x26);
LAB_1020_6a0b:
        invalidate_rect_1020_735a((u32)&iVar5[0x1].field22_0x2a);
        return uVar1;
      }
    }
    else if (param_3 < 0xbb9) {
      if (param_3 == 0x12c) {
        HVar4 = iVar5->field4_0x8;
        wparam = 0xf020;
        goto LAB_1020_69c3;
      }
      uVar1 = param_3 - 0x12d;
      if (param_3 != 0x12c) {
        uVar1 = param_3 - 0x12e;
      }
    }
    else if (param_3 == 0xbb9) {
      HVar4 = GetDlgItem16(0x1797,iVar5->field4_0x8);
      if (HVar4 != 0x0) {
        DestroyWindow16(HVar4);
      }
      uVar1 = pass1_1018_31d0(*(astruct_126 **)&iVar5[0x1].field20_0x26);
      if (uVar1 != 0x0) {
        uVar1 = pass1_1018_2dde(*(astruct_126 **)&iVar5[0x1].field20_0x26);
        goto LAB_1020_6a0b;
      }
    }
    else {
      uVar1 = param_3 - 0xbba;
      if (uVar1 == 0x0) {
        uVar1 = GetDlgItem16(0x1797,iVar5->field4_0x8);
        uVar5 = uVar6;
        if (uVar1 != 0x0) {
          BVar3 = DestroyWindow16(uVar1);
          return BVar3;
        }
        goto LAB_1020_6a6f;
      }
    }
  }
  return uVar1;
}



void win_ui_op_1020_6ae6(astruct_877 *param_1,u16 param_2,i16 param_3,i16 param_4,u16 param_5,u16 param_6)

{
  code **ppcVar1;
  HWND16 hwnd;
  u8 *puVar2;
  astruct_877 *iVar3;
  u16 uVar3;
  LRESULT LVar4;

  if (param_4 == 0x1797) {
    uVar3 = ((u32)param_1 >> 0x10);
    iVar3 = (astruct_877 *)param_1;
    hwnd = GetDlgItem16(0x1797,iVar3->field8_0x8);
    if (hwnd != 0x0) {
      if (param_3 == 0x2) {
        LVar4 = SendMessage16(0x0,0x0,0x409,hwnd);
        if (LVar4 != -0x1) {
          LVar4 = SendMessage16(CONCAT13(0x10,CONCAT12(0x50,&stack0xffa8)),(WPARAM16)LVar4,0x40a,hwnd);
          puVar2 = &stack0xffa8;
          pass1_1018_30ca(((u32)LVar4 >> 0x10),(astruct_504 *)iVar3->field241_0xf2,
                          (char *)CONCAT22(0x1050,puVar2));
          pass1_1018_2fe8((astruct_126 *)iVar3->field241_0xf2,param_5,param_6);
          if (puVar2 != NULL) {
            invalidate_rect_1020_735a(iVar3->field242_0xf6);
            ppcVar1 = (code **)((int)(u32)param_1 + 0x40);
            (**ppcVar1)(0x1018,iVar3);
          }
        }
      }
      else if (param_3 != 0x3) {
        return;
      }
      DestroyWindow16(hwnd);
    }
  }
  return;
}



void enable_menu_item_1020_6b9a(u16 param_1,u16 param_2,u16 param_3,i16 param_4,HMENU16 param_5)

{
  if (param_4 != 0x0) {
    return;
  }
  EnableMenuItem16(0x400,0x0,param_5);
  return;
}



void pass1_1020_6bbc(u32 param_1)

{
  u16 in_DX;

  win_ui_op_1020_737a(in_DX,*(astruct_788 **)((int)param_1 + 0xf6));
  return;
}



void destroy_icon_1020_6bd2(astruct_868 *param_1,u8 param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_868 *struct_1;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_868 *)param_1;
  DestroyIcon16(struct_1->hicon_0xc2);
  struct_1->hicon_0xc2 = 0x0;
  struct_1->field8_0x8 = 0x0;
  puVar1 = struct_1->field241_0xf6;
  uVar2 = struct_1->field242_0xf8;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)((int)s_tile2_bmp_1050_1538,puVar1,uVar2,0x1);
  }
  (u32)&struct_1->field241_0xf6 = 0x0;
  pass1_1010_1dda(struct_1->field240_0xf2);
  struct_1->field240_0xf2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void window_op_1020_6c3a(astruct_57 *param_1,StructA *struct_param_1,u16 param_3,u16 param_4,u16 param_5,
                        u16 param_6,u16 param_7,u16 param_8,u16 param_9,u16 param_10,
                        u16 param_11,u16 param_12)

{
  u32 uVar1;
  code **ppcVar2;
  HICON16 HVar3;
  astruct_57 *paVar4;
  INT16 *pIVar5;
  u32 uVar6;
  u16 uVar7;
  u16 uVar8;
  astruct_57 *paVar9;
  u16 unaff_SI;
  u32 *puVar11;
  u8 uVar12;
  u32 local_6;
  StructA *struct_a_1;
  astruct_57 *paVar10;
  u16 struct_a_1_hi;

  struct_a_1 = (StructA *)struct_param_1;
  struct_a_1_hi = ((u32)struct_param_1 >> 0x10);
  create_window_ex_1008_9760(struct_param_1);
  puVar11 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x4),param_3,param_5,param_6,param_7)
  ;
  paVar9 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puVar11 >> 0x10);
  struct_a_1[0x1].field20_0x26 = (astruct_243 *)puVar11;
  uVar7 = ((u32)puVar11 >> 0x10);
  struct_a_1[0x1].field21_0x28 = uVar7;
  struct_a_1[0x1].field10_0x14 = (i16)struct_a_1[0x1].field20_0x26;
  struct_a_1[0x1].field11_0x16 = uVar7;
  HVar3 = LoadIcon16(s_TILEICON_1050_440c,HINSTANCE16_1050_038c);
  *(HICON16 *)&struct_a_1->field_0xc2 = HVar3;
  uVar6 = (u32)&struct_a_1[0x1].field20_0x26;
  ppcVar2 = (code **)((int)*(u32*)&struct_a_1[0x1].field20_0x26 + 0x30);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar6,(int)((u32)uVar6 >> 0x10),HVar3);
  paVar4 = (astruct_57 *)((int)&local_6 + 0x2);
  pass1_1018_2d22((u32)&struct_a_1[0x1].field20_0x26,(i16 *)CONCAT13(0x10,CONCAT12(0x50,&local_6)),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,paVar4)),0xbb8);
  mem_op_1000_179c(0x42,paVar9);
  uVar8 = (astruct_57 *)paVar9 | paVar4;
  paVar10 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar10,paVar4,(astruct_57 *)paVar9,0x0,local_6,0x0,0x7c007d,
                    CONCAT22(struct_a_1->field4_0x8,0xbb8),param_4,param_8,param_9,param_10,param_11,param_12);
  }
  paVar4 = (astruct_57 *)((int)&local_6 + 0x2);
  pass1_1018_2d22((u32)&struct_a_1[0x1].field20_0x26,(i16 *)CONCAT13(0x10,CONCAT12(0x50,&local_6)),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,paVar4)),0xbb9);
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = (astruct_57 *)paVar10 | paVar4;
  paVar9 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar9,paVar4,(astruct_57 *)paVar10,0x0,local_6,0x0,0x7e007f,
                    CONCAT22(struct_a_1->field4_0x8,0xbb9),param_4,param_8,param_9,param_10,param_11,param_12);
  }
  paVar4 = (astruct_57 *)((int)&local_6 + 0x2);
  pass1_1018_2d22((u32)&struct_a_1[0x1].field20_0x26,(i16 *)CONCAT13(0x10,CONCAT12(0x50,&local_6)),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,paVar4)),0xbba);
  mem_op_1000_179c(0x42,paVar9);
  uVar8 = (astruct_57 *)paVar9 | paVar4;
  paVar10 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6((u32)paVar10,paVar4,(astruct_57 *)paVar9,0x0,local_6,0x1b2,0x1b001b1,
                    CONCAT22(struct_a_1->field4_0x8,0xbba),param_4,param_8,param_9,param_10,param_11,param_12);
  }
  mem_op_1000_179c(0x22,paVar10);
  uVar8 = paVar10 | paVar4;
  if (uVar8 == 0x0) {
    (u32)&struct_a_1[0x1].field22_0x2a = 0x0;
  }
  else {
    unk_win_ui_op_1020_717e(uVar8,param_5,(astruct_40 *)CONCAT22(paVar10,paVar4),struct_param_1);
    struct_a_1[0x1].field22_0x2a = (astruct_160 *)paVar4;
    &struct_a_1[0x1].field_0x2c = uVar8;
  }
  uVar6 = (u32)&struct_a_1[0x1].field22_0x2a;
  (u32)&struct_a_1[0x1].field14_0x1c = uVar6;
  uVar1 = (u32)&struct_a_1[0x1].field20_0x26;
  ppcVar2 = (code **)((int)*(u32*)&struct_a_1[0x1].field20_0x26 + 0x10);
  (**ppcVar2)(0x1000,(char)uVar1,(int)((u32)uVar1 >> 0x10));
  pIVar5 = (INT16 *)uVar6;
  MoveWindow16(0x1,pIVar5[0x3],pIVar5[0x2],pIVar5[0x1],*pIVar5,struct_a_1->field4_0x8);
  uVar12 = (u8)((u32)struct_param_1 >> 0x10);
  uVar6 = (u32)struct_param_1;
  ppcVar2 = (code **)((int)uVar6 + 0x94);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,struct_a_1,uVar12,0x0);
  ppcVar2 = (code **)((int)uVar6 + 0x10);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,struct_a_1,uVar12,0x1);
  UpdateWindow16(struct_a_1->field4_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_6e52(u16 param_1,u16 param_2,i16 param_3,u16 param_4,i16 param_5)

{
  u16 uVar1;
  char *pcVar2;

  pass1_1018_2e5e(param_1,param_2,*(astruct_126 **)(param_3 + 0xf2));
  uVar1 = param_2 | param_1;
  if (uVar1 == 0x0) {
    pcVar2 = load_string_1010_847e(_u16_1050_14cc,0x5a1);
  }
  else {
    pass1_1018_2d84(param_1,*(astruct_126 **)(param_3 + 0xf2));
    pcVar2 = (char *)CONCAT22(uVar1,param_1);
  }
  string_1020_79b4(CONCAT22(param_4,param_3),param_5,pcVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_fn_1020_6e98(u16 param_1,StructA *param_2)

{
  astruct_878 **ppaVar1;
  u32 uVar2;
  HWND16 window_handle;
  u16 uVar3;
  u16 uVar4;
  StructA *iVar4;
  u16 uVar5;
  LRESULT LVar6;
  char *pcVar7;
  WPARAM16 WVar8;
  u16 UVar9;
  HWND16 HVar10;
  u32 win_style;
  RECT16 rectangle;
  HWND16 HStack6;
  i16 iStack4;
  astruct_878 *iVar9;

  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (StructA *)param_2;
  GetClientRect16(&rectangle,(HWND16)&DAT_1050_1050);
  win_style = 0x0;
  window_handle = GetDlgItem16(0x1797,iVar4->field4_0x8);
  if (window_handle != 0x0) {
    DestroyWindow16(window_handle);
  }
  pass1_1018_30fc(param_1,(u32)&iVar4[0x1].field20_0x26,(u16 **)CONCAT22(0x1050,&win_style));
  if ((win_style | win_style) != 0x0) {
    window_handle =
         CreateWindow16(win_style,(void *)CONCAT22(0x1797,HINSTANCE16_1050_038c),iVar4->field4_0x8,iStack4 - 0x19,
                        HStack6,0x0,0x0,0x103,0x40a0,s__1050_4415,s_listbox_1050_4416);
    uVar2 = win_style;
    if (window_handle == 0x0) {
      if ((win_style | win_style) != 0x0) {
        pass1_1018_2afa((u32 *)win_style);
        fn_ptr_1000_17ce((char *)uVar2);
        return;
      }
    }
    else {
      LVar6 = SendMessage16(0x0,0x0,0xb,window_handle);
      uVar4 = ((u32)LVar6 >> 0x10);
      if (((int)win_style + 0x4) == 0x0) {
        WVar8 = 0x0;
        UVar9 = 0x401;
        HVar10 = window_handle;
        pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x531);
        SendMessage16((LPARAM)pcVar7,WVar8,UVar9,HVar10);
      }
      else {
        iVar9 = NULL;
        while( true ) {
          ppaVar1 = (astruct_878 **)((int)win_style + 0x4);
          if (*ppaVar1 == iVar9 || (int)*ppaVar1 < (int)iVar9) break;
          WVar8 = 0x0;
          UVar9 = 0x401;
          HVar10 = window_handle;
          uVar3 = pass1_1020_bd80(((int)(u32)win_style + (int)iVar9 * 0x2));
          LVar6 = SendMessage16(CONCAT22(uVar4,uVar3),WVar8,UVar9,HVar10);
          uVar4 = ((u32)LVar6 >> 0x10);
          iVar9 = (astruct_878 *)(UVar9 + 0x1);
        }
      }
      LVar6 = SendMessage16(0x0,0x1,0xb,window_handle);
      uVar4 = ((u32)LVar6 >> 0x10);
      uVar3 = LVar6;
      WVar8 = 0xffff;
      UVar9 = 0x40d;
      HVar10 = window_handle;
      pass1_1018_2d84(uVar3,*(astruct_126 **)&iVar4[0x1].field20_0x26);
      LVar6 = SendMessage16(CONCAT22(uVar4,uVar3),WVar8,UVar9,HVar10);
      WVar8 = (WPARAM16)LVar6;
      if ((WVar8 != 0xffff) || ((int)((u32)LVar6 >> 0x10) != -0x1)) {
        SendMessage16(0x0,WVar8,0x407,window_handle);
        SendMessage16(0x0,WVar8,0x418,window_handle);
      }
      if (win_style != 0x0) {
        pcVar7 = (char *)win_style;
        pass1_1018_2afa((u32 *)win_style);
        fn_ptr_1000_17ce(pcVar7);
      }
      ShowWindow16(0x1,window_handle);
      SetFocus16(window_handle);
    }
  }
  return;
}



u16 draw_op_1020_7070(u16 param_1,u16 param_2,u16 param_3,u16 param_4,HDC16 hdc_param_5)

{
  u16 hgdi_obj;

  GetStockObject16(BLACK_BRUSH);
  if (COLORREF_1050_441e == 0x0) {
    COLORREF_1050_441e = 0x1000002;
  }
  if (0x6 < param_4) {
    return 0x0;
  }
  SetTextColor16(COLORREF_1050_441e,hdc_param_5);
  hgdi_obj = 0x100;
  SetBkColor16(0x1000000,hdc_param_5);
  return hgdi_obj;
}



StructD * pass1_1020_70c0(StructD *param_1,u8 param_2)

{
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1020_717e(u16 param_1,u16 param_2,astruct_40 *param_3,StructA *param_4)

{
  astruct_13 *paVar1;
  code **ppcVar2;
  u32 uVar4;
  u16 uVar5;
  HPALETTE16 HVar6;
  u32 *puVar6;
  u16 uVar6;
  u16 uVar9;
  u8 *puVar7;
  u8 *puVar10;
  u16 in_register_0000000a;
  astruct_57 *paVar11;
  astruct_40 *iVar7;
  astruct_933 *iVar8;
  astruct_40 *uVar7;
  u16 uVar8;
  u32 *puVar12;
  u16 in_stack_0000fe78;
  u16 in_stack_0000fe84;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  HDC16 local_4;
  u32 uVar3;
  astruct_41 *iVar9;
  u16 in_stack_0000ffdc;

  paVar11 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1020_7c1a(param_3,param_4);
  uVar7 = (astruct_40 *)((u32)param_3 >> 0x10);
  iVar7 = (astruct_40 *)param_3;
  (u32)&iVar7->field17_0x14 = 0x0;
  iVar7->field20_0x18 = param_4;
  (u32)&iVar7->field21_0x1c = 0x0;
  ((int)&iVar7[0x1].field0_0x0 + 0x1) = 0x0;
  param_3->field0_0x0 = 0x754c;
  iVar7->field1_0x2 = 0x1020;
  puVar12 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x4),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  uVar5 = ((u32)paVar11 >> 0x10);
  iVar7->field21_0x1c = (HDC16 *)puVar12;
  uVar9 = ((u32)puVar12 >> 0x10);
  &iVar7->field_0x1e = uVar9;
  ppcVar2 = (code **)((int)*(u32*)&iVar7->field21_0x1c + 0x4);
  (**ppcVar2)(0x1010,iVar7->field21_0x1c,uVar9,0x0,param_3);
  local_4 = GetDC16(iVar7->hwnd_0x4);
  uVar3 = (u32)&iVar7->field21_0x1c;
  *(HDC16 *)((int)uVar3 + 0x178) = local_4;
  uVar4 = (u32)&iVar7->field21_0x1c;
  uVar8 = ((u32)uVar4 >> 0x10);
  iVar8 = (astruct_933 *)uVar4;
  puVar6 = iVar8->field36_0x24;
  uVar9 = ((int)&iVar8->field36_0x24 + 0x2);
  paVar11 = (astruct_57 *)CONCAT22(uVar5,uVar9);
  uVar5 = SUB42(puVar6,0x0);
  ppcVar2 = (code **)((int)*puVar6 + 0x14);
  (**ppcVar2)(0x38,uVar5,uVar9);
  puVar12 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(uVar5,0x29),in_stack_0000fe78,
                            in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  puVar10 = (u8 *)((u32)puVar12 >> 0x10);
  paVar1 = *(astruct_13 **)((int)puVar12 + 0xe);
  pass1_1008_4d84(puVar10,(astruct_90 *)((u32)puVar6 & 0xffff | (long)paVar11 << 0x10),(u32)paVar1);
  HVar6 = palette_op_1008_4e08((HPALETTE16)&local_4,puVar10,paVar1,(HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&local_4)));
  *(HPALETTE16 *)((int)&iVar7[0x1].field0_0x0 + 0x1) = HVar6;
  return;
}



// WARNING: Unable to use type for symbol uVar3

void palette_op_1020_7270(StructD *pstruct_param_1)

{
  HPALETTE16 obj;
  StructD *struct767_var1;
  u16 uVar4;
  u16 unaff_SS;
  char *pcStack8;
  u32 uVar3;
  u16 uVar1;
  u16 uVar2;

  uVar4 = ((u32)pstruct_param_1 >> 0x10);
  struct767_var1 = (StructD *)pstruct_param_1;
  pstruct_param_1->address_offset_field_0x0 = 0x754c;
  struct767_var1->address_offset_field_0x2 = 0x1020;
  if (*(i32 *)&struct767_var1->field_0x1c != 0x0) {
    pass1_1010_1ea6((u32)&struct767_var1->field_0x1c,
                    (StructD *)((u32)pstruct_param_1 & 0xffff | (u32)uVar4 << 0x10));
  }
  uVar1 = &struct767_var1->field12_0x14;
  uVar2 = ((int)&struct767_var1->field12_0x14 + 0x2);
  pcStack8 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack8);
  }
  uVar3 = (u32)&struct767_var1->field_0x1c;
  obj = SelectPalette16(0x0,struct767_var1->field19_0x20,*(HDC16 *)((int)uVar3 + 0x178));
  struct767_var1->field19_0x20 = obj;
  DeleteObject16(obj);
  pstruct_param_1->address_offset_field_0x0 = 0x3ab0;
  struct767_var1->address_offset_field_0x2 = 0x1008;
  pstruct_param_1->address_offset_field_0x0 = 0x389a;
  struct767_var1->address_offset_field_0x2 = 0x1008;
  return;
}



void post_win_msg_1020_7308(u32 param_1,u16 param_2)

{
  char cVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar1 = (char)param_2;
      if (cVar1 == '\x01') {
        (u32)((int)param_1 + 0x1c) = 0x0;
        return;
      }
      if (('\x03' < (char)(cVar1 + -0x1)) && ((char)(cVar1 + -0x5) < '\x02')) goto LAB_1020_7310;
    }
    return;
  }
LAB_1020_7310:
  PostMessage16(0x0,0xeb,0x111,*(HWND16 *)((int)param_1 + 0x4));
  invalidate_rect_1020_735a(param_1);
  return;
}



void invalidate_rect_1020_735a(u32 param_1)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_1 + 0x1c);
  InvalidateRect16(0x0,(RECT16 *)((int)uVar1 + 0x16c),(HWND16)((u32)uVar1 >> 0x10));
  return;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol uVar4

BOOL16 win_ui_op_1020_737a(u16 param_1,astruct_788 *param_2)

{
  u16 uVar8;
  BOOL16 is_iconic;
  char *puVar3;
  RECT16 RVar9;
  u16 in_DX;
  u16 uVar5;
  u16 in_register_0000000a;
  u32 uVar10;
  astruct_788 *struct_1;
  u16 uVar6;
  char local_42 [0x6];
  RECT16 local_brush_handle;
  i16 iStack56;
  HWND16 HStack54;
  HWND16 HStack52;
  i16 iStack50;
  RECT16 rect_30;
  i16 iStack44;
  i16 iStack42;
  HGDIOBJ16 local_rect;
  BOOL16 hicon_38;
  HDC16 hdc_24;
  u8 local_paint_struct [0x20];
  astruct_126 *uVar3;
  u16 uVar1;
  u16 uVar2;
  astruct_126 *uVar4;
  u8 uVar7;
  code **fn_ptr_1;

  uVar6 = ((u32)param_2 >> 0x10);
  struct_1 = (astruct_788 *)param_2;
  hdc_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT13(0x10,CONCAT12(0x50,local_paint_struct)),struct_1->hwnd_0x4);
  is_iconic = IsIconic16(struct_1->hwnd_0x4);
  if (is_iconic == 0x0) {
    uVar4 = struct_1->field22_0x1c;
    RVar9 = *(RECT16 *)((int)uVar4 + 0x24);
    local_brush_handle = RVar9;
    pass1_1018_2e5e(SUB42(RVar9,0x0),in_DX,struct_1->field22_0x1c);
    rect_30 = (u32)RVar9 & 0xffff | (u32)in_DX << 0x10;
    pass1_1008_3e54((u16 *)CONCAT13(0x10,CONCAT12(0x50,local_42)),0x0,0x35,0xc);
    if (*(i32 *)&struct_1->field19_0x14 != 0x0) {
      pass1_1008_5236(*(astruct_109 **)&struct_1->field19_0x14);
    }
    if (rect_30 != 0x0) {
      uVar1 = struct_1->field19_0x14;
      uVar8 = struct_1->field20_0x16;
      uVar10 = CONCAT22(in_register_0000000a,uVar8);
      if ((uVar8 | uVar1) != 0x0) {
        pass1_1008_5118(CONCAT22(uVar8,uVar1));
        fn_ptr_1000_17ce((char *)CONCAT22(uVar8,uVar1));
      }
      puVar3 = local_42;
      pass1_1008_8ce4(rect_30,(u16 *)CONCAT22(0x1050,puVar3),(u32)local_brush_handle,uVar10);
      struct_1->field19_0x14 = puVar3;
      struct_1->field20_0x16 = uVar10;
    }
    fn_ptr_1 = (code **)((int)(u32)local_brush_handle + 0x4);
    (**fn_ptr_1)(0x1008,SUB42(local_brush_handle,0x0),(int)((u32)local_brush_handle >> 0x10),0x0,0x0,&hdc_24,
                 (int)&DAT_1050_1050);
    fn_ptr_1 = (code **)((int)*struct_1->field21_0x18 + 0x94);
    (**fn_ptr_1)(0x1008,struct_1->field21_0x18,0x1);
    HStack52 = GetDlgItem16(0x1797,struct_1->hwnd_0x4);
    if (HStack52 != 0x0) {
      ShowWindow16(0x1,HStack52);
    }
  }
  else if (PTR_LOOP_1050_0010 == NULL) {
    uVar3 = struct_1->field22_0x1c;
    fn_ptr_1 = (code **)((int)(u32)struct_1->field22_0x1c + 0x2c);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)uVar3,(int)((u32)uVar3 >> 0x10));
    hicon_38 = is_iconic;
    if (is_iconic != 0x0) {
      local_rect = GetStockObject16(BLACK_BRUSH);
      GetClientRect16(&rect_30,(HWND16)&DAT_1050_1050);
      local_brush_handle = (RECT16)0x0;
      iStack56 = (iStack44 - rect_30.x) + -0x1;
      HStack54 = (iStack42 - rect_30.y) - 0x1;
      HStack52 = HStack54;
      iStack50 = iStack56;
      FillRect16(local_rect,&local_brush_handle,(HDC16)&DAT_1050_1050);
      DrawIcon16(hicon_38,0x2,0x2,hdc_24);
    }
  }
  is_iconic = EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_paint_struct),struct_1->hwnd_0x4);
  return is_iconic;
}



StructD * pass1_1020_7526(StructD *param_1,u8 param_2)

{
  palette_op_1020_7270(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1020_7554(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_20 *iVar2;
  u16 unaff_BP;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  unk_draw_op_1020_7f7a(param_2,0x5,CONCAT22(param_4,param_3),param_5);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  (u32)&iVar2[0x1].field5_0xc = 0x0;
  iVar2[0x1].field7_0x10 = NULL;
  param_2->offset_0x0 = 0x7780;
  iVar2->base_0x2 = 0x1020;
  (iVar2 + 0x1)->offset_0x0 = 0x781c;
  iVar2[0x1].base_0x2 = 0x1020;
  puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x25),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = ((u32)puVar4 >> 0x10);
  &iVar2[0x1].field7_0x10 = (int)puVar4;
  ((int)&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  ((int)&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}



void pass1_1020_75c4(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x7780;
  iVar1->address_offset_field_0x2 = 0x1020;
  &iVar1->field_0xe2 = 0x781c;
  &iVar1->field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_1020_75f0(astruct_283 *param_1,u32 param_2)

{
  u16 *pUVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar8;
  astruct_283 *iVar7;
  u16 uVar9;
  u16 *puVar10;
  u16 in_stack_0000ff4c;
  u16 in_stack_0000ff62;
  u32 uStack10;
  u8 local_6 [0x4];
  astruct_57 *paVar6;
  u32 uVar7;

  uVar8 = ((u32)param_2 >> 0x10);
  uVar9 = ((u32)param_1 >> 0x10);
  iVar7 = (astruct_283 *)param_1;
  if (iVar7->field235_0xee != NULL) {
    ppcVar2 = (code **)((int)*iVar7->field235_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7->field233_0xea == 0x0) {
    iVar7->field233_0xea = 0x1;
    puVar10 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x91);
    uVar4 = ((u32)puVar10 >> 0x10);
    paVar6 = (astruct_57 *)CONCAT22(uVar8,uVar4);
    uVar3 = ZEXT24(local_6);
    win_1008_5c9e(local_6,uVar4,_u16_1050_02a0,(u32 *)CONCAT22(0x1050,local_6));
    iVar7->field234_0xec = (int)uVar3;
    mem_op_1000_179c(0x112,paVar6);
    uVar5 = paVar6 | uVar3;
    uVar7 = (u32)paVar6 & 0xffff0000 | (u32)uVar5;
    if (uVar5 == 0x0) {
      uVar9 = 0x0;
      uStack10 = NULL;
    }
    else {
      pUVar1 = &iVar7->field204_0xcc;
      *pUVar1 = *pUVar1 + 0x1;
      struct_1020_3644(uVar7,(StructA *)CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,uVar3)),
                       iVar7->field204_0xcc,(u32)param_1 & 0xffff | (u32)uVar9 << 0x10,in_stack_0000ff4c,
                       in_stack_0000ff62);
      uVar9 = uVar3;
      uStack10 = (u32 *)(uVar3 & 0xffff | uVar7 << 0x10);
    }
    pass1_1008_6978(uVar9,(u8 *)uVar7,(u32)param_1,0x0,(u32)uStack10 & 0xffff0000 | (u32)uVar9);
    ppcVar2 = (code **)((int)*uStack10 + 0xc);
    (**ppcVar2)(0x8,(int)uStack10,uStack10,0x5);
  }
  return;
}



void window_op_1020_76aa(StructA *param_1,astruct_666 *param_2)

{
  u16 uVar3;
  astruct_57 *in_EDX;
  StructA *iVar1;
  u16 uVar2;

  create_window_ex_1008_9760(param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (StructA *)param_1;
  get_dc_1018_4db0(*(astruct_126 **)&iVar1[0x1].field20_0x26,iVar1->field4_0x8);
  mem_op_1000_179c(0x18,in_EDX);
  uVar3 = in_EDX | param_2;
  if (uVar3 != 0x0) {
    pass1_1020_7824(uVar3,param_2,in_EDX,iVar1->field4_0x8);
    iVar1[0x1].field18_0x22 = param_2;
    iVar1[0x1].field19_0x24 = uVar3;
    return;
  }
  (u32)&iVar1[0x1].field18_0x22 = 0x0;
  return;
}



void pass1_1020_770e(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0xee);
  uVar2 = (iVar4 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0xee) = 0x0;
  destroy_win_1008_628e(param_1 & 0xffff | (u32)uVar5 << 0x10);
  return;
}



StructD * pass1_1020_774c(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1020_75c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_7824(u16 param_1,astruct_666 *param_2,u16 param_3,u16 param_4)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u8 *puVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  u32 *puVar6;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u32 in_stack_0000ffca;
  u16 in_stack_0000fff2;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  set_struct_op_1020_921c(param_1,(StructA *)CONCAT22(param_3,param_2),param_4,in_stack_0000ffca);
  (u32)&param_2->field16_0x14 = 0x0;
  CONCAT22(param_3,param_2) = 0x7902;
  param_2->field2_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x25),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = ((u32)puVar6 >> 0x10);
  param_2->field16_0x14 = (int)puVar6;
  param_2->field17_0x16 = uVar3;
  param_2->field5_0x6 = param_2->field16_0x14;
  param_2->field6_0x8 = uVar3;
  uVar2 = (u32)&param_2->field16_0x14;
  puVar4 = &param_2->field_0xa;
  ppcVar1 = (code **)((int)(u32)(u32)((int)uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_2->field15_0x12 = (int)puVar4;
  draw_op_1020_9364((StructA *)CONCAT22(param_3,param_2));
  return;
}



void pass1_1020_78ac(u16 *param_1)

{
  i16 iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x7902;
  (iVar1 + 0x2) = 0x1020;
  if (*(i32 *)(iVar1 + 0x14) != 0x0) {
    pass1_1010_1dda((u32)(iVar1 + 0x14));
  }
  palette_op_1020_92c4((StructD *)param_1);
  return;
}



StructD * pass1_1020_78dc(StructD *param_1,u8 param_2)

{
  pass1_1020_78ac(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1020_790e(u16 *param_1,char *param_2,u16 param_3,u32 param_4)

{
  astruct_271 *iVar1;
  u16 uVar1;

  unk_draw_op_1008_7f62(param_1,param_3,param_4);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_271 *)param_1;
  iVar1->field223_0xe0 = 0x0;
  iVar1->field224_0xe4 = 0x0;
  iVar1->field225_0xe8 = 0x0;
  iVar1->field226_0xec = 0x0;
  iVar1->field227_0xee = param_2;
  *param_1 = 0x7b86;
  iVar1->field2_0x2 = 0x1020;
  return;
}



void cleanup_menu_ui_op_1020_795c(StructD *in_struct_1)

{
  StructD *local_struct_1;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)in_struct_1 >> 0x10);
  local_struct_1 = (StructD *)in_struct_1;
  in_struct_1->address_offset_field_0x0 = 0x7b86;
  local_struct_1->address_offset_field_0x2 = 0x1020;
  if (local_struct_1->hmenu_0xec != 0x0) {
    DestroyMenu16(local_struct_1->hmenu_0xec);
  }
  pass1_1008_57c4((StructD *)((u32)in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1->field192_0xd2)));
  in_struct_1->address_offset_field_0x0 = 0x380a;
  local_struct_1->address_offset_field_0x2 = 0x1008;
  in_struct_1->address_offset_field_0x0 = 0x389a;
  local_struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



u16 pass1_1020_79ae(void)

{
  return 0x1;
}



void string_1020_79b4(u32 param_1,i16 param_2,char *param_3)

{
  u16 in_DX;

  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0xa)),param_3);
  if (param_2 != 0x0) {
    draw_op_1020_7cc8(in_DX,*(StructE **)((int)param_1 + 0xe8));
  }
  return;
}



void pass1_1020_79e4(u32 param_1)

{
  u16 in_DX;

  draw_op_1020_7cc8(in_DX,*(StructE **)((int)param_1 + 0xe8));
  return;
}



void post_win_msg_1020_79fc(astruct_69 *param_1,u16 param_2,u16 param_3,i16 param_4)

{
  u32 *puVar1;
  code **ppcVar2;
  i16 iVar3;
  astruct_69 *iVar4;
  u16 uVar4;
  u16 uVar5;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_69 *)param_1;
  uVar5 = ((u32)iVar4->field223_0xe0 >> 0x10);
  ppcVar2 = (code **)((int)*iVar4->field223_0xe0 + 0x24);
  iVar3 = (**ppcVar2)();
  if (iVar3 != param_4) {
    PostMessage16(0x0,0x0,0x85,iVar4->field8_0x8);
    puVar1 = iVar4->field223_0xe0;
    ppcVar2 = (code **)((int)*iVar4->field223_0xe0 + 0x28);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(char)puVar1,(int)((u32)puVar1 >> 0x10),param_4,uVar5);
  }
  return;
}



void get_win_ui_info_op_1020_7a50(astruct_868 *param_1)

{
  code **ppcVar1;
  BOOL16 b_var2;
  astruct_868 *iVar2;
  u16 var5;

  var5 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_868 *)param_1;
  b_var2 = IsIconic16(iVar2->field8_0x8);
  if (b_var2 == 0x0) {
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff6),iVar2->field8_0x8);
    GetSystemMetrics16(SM_CXBORDER);
    GetSystemMetrics16(SM_CYBORDER);
  }
  ppcVar1 = (code **)((int)*(u32*)&iVar2->field_0xe0 + 0x14);
  (**ppcVar1)();
  return;
}



void win_ui_menu_op_1020_7ad2(astruct_854 *param_1,HWND16 param_2,RECT16 *param_3)

{
  HMENU16 HVar1;
  astruct_854 *iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_854 *)param_1;
  if ((iVar2->field236_0xee != NULL) && (iVar2->field235_0xec == 0x0)) {
    HVar1 = LoadMenu16(iVar2->field236_0xee,HINSTANCE16_1050_038c);
    iVar2->field235_0xec = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    HVar1 = GetSubMenu16(0x0,iVar2->field235_0xec);
    iVar2->field235_0xec = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  ClientToScreen16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),iVar2->field8_0x8);
  HVar1 = iVar2->field235_0xec;
  TrackPopupMenu16(NULL,iVar2->field8_0x8,0x0,HVar1,0x0,0x0,HVar1);
  return;
}



StructD * pass1_1020_7b60(StructD *param_1,u8 param_2)

{
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void get_sys_metrics_1020_7c1a(astruct_40 *param_1,StructA *param_2)

{
  INT16 IVar1;
  astruct_40 *iVar3;
  u16 uVar3;
  astruct_40 *uVar4;
  u16 uVar1;

  uVar3 = ((u32)param_2 >> 0x10);
  uVar1 = ((int)param_2 + 0x8);
  uVar4 = (astruct_40 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_40 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  param_1->field0_0x0 = 0x3aa8;
  iVar3->field1_0x2 = 0x1008;
  iVar3->hwnd_0x4 = uVar1;
  param_1->field0_0x0 = 0x3ab0;
  iVar3->field1_0x2 = 0x1008;
  *(StructA **)&iVar3->field_0x6 = param_2;
  (u32)&iVar3->field_0xa = 0x0;
  &iVar3->field_0xe = 0x0;
  &iVar3->field_0x10 = 0x0;
  &iVar3->field_0x12 = 0x0;
  param_1->field0_0x0 = 0x7f72;
  iVar3->field1_0x2 = 0x1020;
  (u32)&iVar3->field_0xa = (u32)((int)param_2 + 0xe4);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  *(INT16 *)&iVar3->field_0xe = IVar1;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  *(INT16 *)&iVar3->field_0x10 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CYBORDER);
  *(INT16 *)&iVar3->field_0x12 = IVar1;
  return;
}



// WARNING: Unable to use type for symbol puVar2
// WARNING: Unable to use type for symbol iVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1020_7cc8(u16 param_1,StructE *struct_e_param_1)

{
  i16 y_00;
  i16 str46_len;
  i16 x;
  i16 iVar3;
  HGDIOBJ16 brush_handle_2;
  i16 width;
  i16 iVar9;
  u32 *puVar6;
  i16 iVar5;
  HPEN16 handle;
  char *string_1;
  i16 y;
  u16 DX_REG;
  i16 iVar6;
  StructE *struct_e_1;
  astruct_781 *iVar7;
  u16 uVar7;
  u16 uVar8;
  DWORD DVar1;
  RECT16 *rect;
  HDC16 hdc;
  i16 iVar2;
  char *string_46;
  i16 local_rect_1;
  i16 iStack18;
  i16 iStack16;
  i16 iStack14;
  HPALETTE16 hpalette_12;
  astruct_13 *paStack10;
  HDC16 win_hdc_1;
  BOOL16 is_iconic;
  u32 *puVar2;
  i16 style;
  i16 iVar8;
  i16 iVar1;
  code **fn_ptr_1;

  uVar7 = ((u32)struct_e_param_1 >> 0x10);
  struct_e_1 = (StructE *)struct_e_param_1;
  is_iconic = IsIconic16(struct_e_1->hwnd16_field4_0x4);
  if ((is_iconic == 0x0) || (PTR_LOOP_1050_0010 != NULL)) {
    win_hdc_1 = GetWindowDC16(struct_e_1->hwnd16_field4_0x4);
    paStack10 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
    hpalette_12 = palette_op_1008_4e08((HPALETTE16)&win_hdc_1,param_1,paStack10,(HDC16 *)CONCAT22(0x1050,&win_hdc_1));
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_rect_1),struct_e_1->hwnd16_field4_0x4);
    x = (iStack16 - local_rect_1) + -0x1;
    y = (iStack14 - iStack18) + -0x1;
    iVar1 = struct_e_1->field12_0x12;
    iVar3 = y;
    if (is_iconic == 0x0) {
      iVar3 = struct_e_1->field10_0xe - struct_e_1->field12_0x12;
    }
    rect = (RECT16 *)&DAT_1050_1050;
    hdc = win_hdc_1;
    iVar2 = y;
    brush_handle_2 = GetStockObject16(BLACK_BRUSH);
    FillRect16(brush_handle_2,rect,hdc);
    puVar2 = struct_e_1->field5_0x6;
    uVar8 = ((u32)puVar2 >> 0x10);
    iVar7 = (astruct_781 *)puVar2;
    puVar6 = (u32*)&iVar7->field_0xe0;
    style = iVar7->field226_0xe2;
    width = (i16)puVar6;
    fn_ptr_1 = (code **)((int)*puVar6 + 0x24);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,width,style,0x0,0x0,iVar2);
    iVar5 = (-((int)puVar6 == 0x0) & 0x1e) + 0x25;
    handle = CreatePen16(CONCAT22(0x100,iVar5),width,style);
    brush_handle_2 = SelectObject16(handle,win_hdc_1);
    MoveTo16(0x0,0x0,win_hdc_1);
    LineTo16(0x0,x,win_hdc_1);
    LineTo16(y,x,win_hdc_1);
    LineTo16(y,0x0,win_hdc_1);
    string_1._0_2_ = (char *)LineTo16(0x0,0x0,win_hdc_1);
    if (is_iconic == 0x0) {
      y_00 = struct_e_1->field10_0xe - struct_e_1->field12_0x12;
      MoveTo16(y_00,0x0,win_hdc_1);
      string_1._0_2_ = (char *)LineTo16(y_00,x,win_hdc_1);
    }
    fn_ptr_1 = (code **)((int)*struct_e_1->field5_0x6 + 0x18);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538);
    string_46 = (char *)CONCAT22(DX_REG,(char *)string_1);
    if (*(char *)string_1 != '\0') {
      SetBkColor16(0x0,win_hdc_1);
      SetTextColor16(CONCAT22(0x100,iVar5),win_hdc_1);
      str46_len = lstrlen16(string_46);
      DVar1 = GetTextExtent16(str46_len,
                              (LPCSTR)CONCAT13((char)(DX_REG >> 0x8),CONCAT12((char)DX_REG,(char *)string_1)),
                              win_hdc_1);
      iVar6 = (i16)(DVar1 >> 0x10);
      if (is_iconic == 0x0) {
        iVar9 = (iVar3 - iVar1) / 0x2 - iVar6 / 0x2;
        iVar8 = x / 0x2 - (int)DVar1 / 0x2;
      }
      else {
        iVar9 = y / 0x2 - iVar6 / 0x2;
        iVar8 = 0x2;
      }
      TextOut16(iVar9,(char *)CONCAT22(DX_REG,(char *)string_1),iVar9,iVar8,win_hdc_1);
    }
    hpalette_12 = SelectPalette16(0x0,hpalette_12,win_hdc_1);
    DeleteObject16(hpalette_12);
    SelectObject16(brush_handle_2,win_hdc_1);
    DeleteObject16(handle);
    ReleaseDC16(win_hdc_1,struct_e_1->hwnd16_field4_0x4);
  }
  return;
}



StructD * pass1_1020_7f38(StructD *param_1,u8 param_2)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x3ab0;
  ((int)param_1 + 0x2) = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_draw_op_1020_7f7a(astruct_20 *param_1,u16 param_2,u32 param_3,u16 param_4)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 hcursor2;
  u32 in_EDX;
  u16 uVar3;
  astruct_57 *paVar2;
  astruct_20 *struct_1;
  u16 uVar4;
  u16 unaff_SS;
  astruct_20 *paVar4;
  u32 *puVar5;
  u16 in_stack_0000fe90;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffe8;
  u16 uVar1;

  uVar3 = ((u32)in_EDX >> 0x10);
  paVar4 = unk_draw_op_1008_61b2
                     (in_stack_0000ffbc,(StructA *)param_1,param_2,param_3,CONCAT22(param_4,param_3));
  paVar2 = (astruct_57 *)CONCAT22(uVar3,(int)((u32)paVar4 >> 0x10));
  uVar4 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_20 *)param_1;
  (struct_1 + 0x1)->offset_0x0 = 0x389a;
  struct_1[0x1].base_0x2 = 0x1008;
  (struct_1 + 0x1)->offset_0x0 = 0x3aa8;
  struct_1[0x1].base_0x2 = 0x1008;
  struct_1[0x1].field2_0x4 = 0x0;
  struct_1[0x1].field3_0x8 = 0x0;
  struct_1[0x1].field4_0xa = 0x0;
  param_1->offset_0x0 = 0x82bc;
  struct_1->base_0x2 = 0x1020;
  (struct_1 + 0x1)->offset_0x0 = 0x8358;
  struct_1[0x1].base_0x2 = 0x1020;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&struct_1->field60_0x5b)),s_VrMode_1050_4422);
  HVar1 = GetStockObject16(HOLLOW_BRUSH);
  struct_1->hgdiobj_field_0xc6 = HVar1;
  hcursor2 = LoadCursor16((char *)0x7f00,0x0);
  struct_1->hcursor_field_0xc4 = hcursor2;
  struct_1->field150_0xc8 = 0x2028;
  struct_1->field139_0xac = 0x47000000;
  struct_1->field145_0xbc = (param_3 + 0x8);
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe8,0x48),in_stack_0000fe90,
                           in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
  uVar1 = ((u32)puVar5 >> 0x10);
  struct_1->field141_0xb4 = 0x0;
  struct_1->field142_0xb6 = 0x0;
  struct_1->field143_0xb8 = ((int)puVar5 + 0xa);
  struct_1->field144_0xba = ((int)puVar5 + 0xc);
  struct_1->field151_0xca = param_3;
  win_ui_reg_class_1008_96d2((StructA *)param_1);
  return;
}



void pass1_1020_808e(StructD *param_1)

{
  u8 *puVar1;
  u16 uVar2;
  StructD *iVar3;
  u16 uVar3;
  u16 *puStack6;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x82bc;
  iVar3->address_offset_field_0x2 = 0x1020;
  &iVar3->field_0xe2 = 0x8358;
  &iVar3->field_0xe4 = 0x1020;
  if (param_1 == NULL) {
    puVar1 = NULL;
    uVar2 = 0x0;
  }
  else {
    puVar1 = &iVar3->field_0xe2;
    uVar2 = uVar3;
  }
  puStack6 = (u16 *)CONCAT22(uVar2,puVar1);
  *puStack6 = 0x389a;
  (puVar1 + 0x2) = 0x1008;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field192_0xd2)));
  param_1->address_offset_field_0x0 = 0x380a;
  iVar3->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar3->address_offset_field_0x2 = 0x1008;
  return;
}



void pass1_1020_8106(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x60);
  (**ppcVar1)();
  return;
}



void realize_palette_1020_8128(u32 param_1,i16 param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u8 *puVar3;
  u32 *puVar4;
  u32 *puVar5;
  u16 DX_REG;
  i16 iVar6;
  u16 uVar7;
  u8 local_12 [0x8];
  HDC16 hdc_10;
  HGDIOBJ16 HStack8;
  u32 *puStack6;

  if (param_2 != 0x0) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = (int)param_1;
    uVar2 = (u32)(iVar6 + 0xe6);
    puVar5 = (u32 *)(u32)((int)uVar2 + 0xa);
    ppcVar1 = (code **)((int)*puVar5 + 0x18);
    puStack6 = puVar5;
    (**ppcVar1)();
    HStack8 = (HGDIOBJ16)puVar5;
    UnrealizeObject16(HStack8);
    uVar2 = (u32)(iVar6 + 0xe6);
    hdc_10 = *(HDC16 *)((int)uVar2 + 0x14);
    RealizePalette16(hdc_10);
    pass1_1008_57a4((u32 *)CONCAT22(0x1050,local_12),param_1 & 0xffff0000 | (u32)(iVar6 + 0xd2));
    while( true ) {
      puVar3 = local_12;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
      if ((DX_REG | puVar3) == 0x0) break;
      uVar2 = (u32)(puVar3 + 0x4);
      uVar7 = (puVar3 + 0x6);
      puVar4 = (u32 *)uVar2;
      ppcVar1 = (code **)((int)*puVar4 + 0x90);
      (**ppcVar1)(0x1008,puVar4,uVar7,0x1,uVar2);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_palette_op_1020_81c0(u32 param_1)

{
  astruct_13 *in_struct_1;
  HDC16 hdc;
  HDC16 hpal;
  HPALETTE16 hpal_00;
  u16 uVar1;
  u16 uVar2;
  HDC16 hdc_00;
  u16 uStack6;

  uVar2 = ((u32)_PTR_LOOP_1050_4230 >> 0x10);
  in_struct_1 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  uVar1 = ((int)_PTR_LOOP_1050_4230 + 0x10);
  uStack6 = in_struct_1;
  if ((uVar1 | uStack6) == 0x0) {
    return;
  }
  hdc = GetDC16(*(HWND16 *)((int)param_1 + 0x8));
  hpal = hdc;
  hdc_00 = hdc;
  create_palette_1008_4e38(in_struct_1,uVar1);
  hpal_00 = SelectPalette16(0x0,hpal,hdc_00);
  RealizePalette16(hdc);
  SelectPalette16(0x1,hpal_00,hdc);
  RealizePalette16(hdc);
  DeleteObject16(hpal);
  if (0x0 < (int)hpal) {
    InvalidateRect16(0x1,NULL,0x0);
  }
  return;
}



void destroy_window_1020_8250(astruct_879 *param_1)

{
  BOOL16 BVar1;
  astruct_879 *iVar2;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_879 *)param_1;
  if (iVar2->field236_0xec != 0x0) {
    BVar1 = IsWindow16(iVar2->field236_0xec);
    if (BVar1 != 0x0) {
      DestroyWindow16(iVar2->field236_0xec);
      iVar2->field236_0xec = 0x0;
    }
  }
  return;
}



StructD * pass1_1020_8288(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1020_808e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1020_8296(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1020_808e(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_8360(astruct_20 *param_1)

{
  u32 uVar1;
  u16 uVar2;
  u32 in_EDX;
  u16 uVar3;
  u16 unaff_SI;
  u16 *puVar4;
  u32 *puVar5;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 uVar6;
  astruct_20 *iVar4;

  uVar3 = ((u32)in_EDX >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  uVar6 = ((u32)param_1 >> 0x10);
  struct_1020_847a(param_1,0x1);
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field9_0x16)));
  (u32)&iVar4->field12_0x1c = 0x0;
  param_1->offset_0x0 = 0x8462;
  iVar4->base_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar3,(int)((u32)puVar4 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x29),in_stack_0000fe96,in_stack_0000ffba,in_stack_0000ffc0,
                           in_stack_0000ffc4);
  uVar2 = ((u32)puVar5 >> 0x10);
  iVar4->field12_0x1c = puVar5;
  &iVar4->field13_0x1e = uVar2;
  pass1_1018_26f8(iVar4->field12_0x1c,uVar2,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field9_0x16)));
  uVar1 = (u32)&iVar4->field12_0x1c;
  pass1_1020_8712((u32)param_1 & 0xffff | (u32)uVar6 << 0x10,*(i16 **)&iVar4->field3_0x8,
                  *(astruct_76 **)((int)uVar1 + 0x2a),(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field9_0x16))
                 );
  return;
}



void pass1_1020_83f8(u32 param_1)

{
  u32 uVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  if ((iVar3 + 0x4) != 0x0) {
    uVar1 = (u32)(iVar3 + 0x1c);
    uVar2 = (u32)(iVar3 + 0x1c);
    pass1_1008_4480(*(astruct_76 **)((int)uVar1 + 0xa),(u16 *)(param_1 & 0xffff0000 | (u32)(iVar3 + 0x16)),
                    *(astruct_76 **)((int)uVar2 + 0x2a));
  }
  return;
}



void FUN_1020_8438(void)

{
  return;
}



StructD * pass1_1020_843c(StructD *param_1,u8 param_2)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1020_847a(astruct_20 *param_1,i16 param_2)

{
  u16 uVar1;
  astruct_20 *paVar2;
  u32 in_EDX;
  u16 uVar4;
  astruct_57 *paVar3;
  astruct_20 *iVar3;
  u16 unaff_SI;
  u16 uVar5;
  u16 *puVar6;
  u32 *puVar7;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;

  uVar4 = ((u32)in_EDX >> 0x10);
  uVar5 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_20 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar3->base_0x2 = 0x1008;
  &iVar3->field2_0x4 = 0x0;
  ((int)&iVar3->field2_0x4 + 0x2) = param_2;
  (u32)&iVar3->field3_0x8 = 0x0;
  (u32)&iVar3->field5_0xc = 0x0;
  puVar6 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field7_0x10)));
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(int)((u32)puVar6 >> 0x10));
  param_1->offset_0x0 = 0x87aa;
  iVar3->base_0x2 = 0x1020;
  puVar7 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x48),in_stack_0000fe92,
                           in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
  paVar3 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)puVar7 >> 0x10);
  pass1_1008_3f62((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field7_0x10)),
                  (u16 *)((u32)puVar7 & 0xffff0000 | (u32)((int)puVar7 + 0xe)));
  uVar1 = ((int)&iVar3->field2_0x4 + 0x2) << 0x3;
  mem_op_1000_179c(uVar1,paVar3);
  iVar3->field3_0x8 = uVar1;
  iVar3->field4_0xa = paVar3;
  paVar2 = (astruct_20 *)(((int)&iVar3->field2_0x4 + 0x2) << 0x2);
  mem_op_1000_179c((i16)paVar2,paVar3);
  iVar3->field5_0xc = paVar2;
  iVar3->field6_0xe = paVar3;
  pass1_1000_4906(*(StructD **)&iVar3->field3_0x8,NULL,((int)&iVar3->field2_0x4 + 0x2) << 0x3);
  pass1_1000_4906(*(StructD **)&iVar3->field5_0xc,NULL,((int)&iVar3->field2_0x4 + 0x2) << 0x2);
  return;
}



// WARNING: Unable to use type for symbol uVar4

void pass1_1020_8556(StructD *param_1)

{
  i16 *piVar1;
  u16 uVar2;
  char *pcVar3;
  StructD *iVar5;
  astruct_589 *iVar4;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  i16 iStack12;
  u32 uVar4;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x87aa;
  iVar5->address_offset_field_0x2 = 0x1020;
  fn_ptr_1000_17ce(*(char **)&iVar5->field5_0x8);
  if ((iVar5->field8_0xe | iVar5->field7_0xc) != 0x0) {
    iStack12 = 0x0;
    while( true ) {
      piVar1 = (i16 *)&iVar5->field_0x6;
      if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
      iVar6 = iStack12 * 0x4;
      uVar4 = (u32)&iVar5->field7_0xc;
      uVar8 = ((u32)uVar4 >> 0x10);
      iVar4 = (astruct_589 *)uVar4;
      if (*(i32 *)(iVar4 + iVar6) != 0x0) {
        pcVar3 = *(char **)(iVar4 + iVar6);
        uVar2 = (iVar4 + iVar6 + 0x2);
        if ((uVar2 | pcVar3) != 0x0) {
          pass1_1008_5118((u32)pcVar3 & 0xffff | (u32)uVar2 << 0x10);
          fn_ptr_1000_17ce(pcVar3);
        }
      }
      iStack12 += 0x1;
    }
    fn_ptr_1000_17ce(*(char **)&iVar5->field7_0xc);
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar5->address_offset_field_0x2 = 0x1008;
  return;
}



void pass1_1020_85f6(astruct_590 *param_1)

{
  i16 *piVar1;
  u16 uVar2;
  char *pcVar3;
  u32 uVar4;
  i16 iVar5;
  astruct_590 *iVar6;
  u16 uVar6;
  u16 uVar7;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    uVar7 = ((u32)param_1 >> 0x10);
    iVar6 = (astruct_590 *)param_1;
    piVar1 = &iVar6->field6_0x6;
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar4 = iVar6->field11_0xc;
    uVar6 = ((u32)uVar4 >> 0x10);
    iVar5 = (int)uVar4;
    pcVar3 = *(char **)(iVar5 + iStack4 * 0x4);
    uVar2 = (iVar5 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | pcVar3) != 0x0) {
      pass1_1008_5118((u32)pcVar3 & 0xffff | (u32)uVar2 << 0x10);
      fn_ptr_1000_17ce(pcVar3);
    }
    uVar4 = iVar6->field11_0xc;
    (u32)((int)uVar4 + iStack4 * 0x4) = 0x0;
    iStack4 += 0x1;
  }
  return;
}



void pass1_1020_865a(u32 param_1)

{
  i16 *piVar1;
  u16 uVar2;
  char *pcVar3;
  u32 uVar4;
  i16 iVar5;
  astruct_592 *iVar7;
  astruct_591 *iVar6;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    uVar9 = (param_1 >> 0x10);
    iVar5 = (int)param_1;
    piVar1 = (i16 *)(iVar5 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    iVar8 = iStack4 * 0x4;
    uVar4 = (u32)(iVar5 + 0xc);
    uVar10 = ((u32)uVar4 >> 0x10);
    iVar7 = (astruct_592 *)uVar4;
    if (*(i32 *)(iVar7 + iVar8) != 0x0) {
      pass1_1008_5236(*(astruct_109 **)(iVar7 + iVar8));
      uVar4 = (u32)(iVar5 + 0xc);
      uVar10 = ((u32)uVar4 >> 0x10);
      iVar6 = (astruct_591 *)uVar4;
      pcVar3 = *(char **)(iVar6 + iVar8);
      uVar2 = (iVar6 + iVar8 + 0x2);
      if ((uVar2 | pcVar3) != 0x0) {
        pass1_1008_5118((u32)pcVar3 & 0xffff | (u32)uVar2 << 0x10);
        fn_ptr_1000_17ce(pcVar3);
      }
      uVar4 = (u32)(iVar5 + 0xc);
      (u32)((int)uVar4 + iStack4 * 0x4) = 0x0;
    }
    iStack4 += 0x1;
  }
  return;
}



void pass1_1020_86d8(u32 param_1)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    piVar1 = (i16 *)((int)param_1 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar2 = (u32)((int)param_1 + 0xc);
    uVar4 = ((u32)uVar2 >> 0x10);
    iVar3 = (int)uVar2;
    if (*(i32 *)(iVar3 + iStack4 * 0x4) != 0x0) {
      pass1_1008_5236(*(astruct_109 **)(iVar3 + iStack4 * 0x4));
    }
    iStack4 += 0x1;
  }
  return;
}



void pass1_1020_8712(u32 param_1,i16 *param_2,astruct_76 *param_3,u16 *param_4)

{
  u16 uVar1;
  u32 uVar2;

  pass1_1008_3f32((i16 *)param_4,(i16 *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x10)));
  uVar2 = pass1_1008_4772(param_3);
  uVar1 = (uVar2 >> 0x10);
  pass1_1008_3e94(param_4,(u16 *)((u32)param_2 & 0xffff0000 | ZEXT24((i16 *)((int)param_2 + 0x2))),
                  (char *)((u32)param_2 & 0xffff | (u32)param_2 << 0x10));
  ((int)param_2 + 0x4) = ((int)uVar2 + 0x4) + *param_2;
  ((int)param_2 + 0x6) = ((int)uVar2 + 0x8) + ((int)param_2 + 0x2);
  return;
}



void FUN_1020_8780(void)

{
  return;
}



StructD * pass1_1020_8784(StructD *param_1,u8 param_2)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_87c2(astruct_20 *param_1)

{
  u32 uVar1;
  astruct_20 *iVar2;
  i32 in_EDX;
  u16 uVar2;
  u16 unaff_SI;
  u16 uVar3;
  u16 *puVar4;
  u32 *puVar5;
  u16 in_stack_0000fe80;
  u16 in_stack_0000ffa4;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  u8 local_12 [0x8];
  i16 iStack10;
  astruct_19 *paStack8;
  i16 iStack4;

  struct_1020_847a(param_1,0x4);
  iStack4 = 0x4;
  iVar2 = (astruct_20 *)param_1;
  iVar2 = (astruct_20 *)&iVar2->field9_0x16;
  paStack8 = (astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar2));
  do {
    uVar2 = ((u32)in_EDX >> 0x10);
    pass1_1008_3e38(paStack8);
    in_EDX = (u32)uVar2 << 0x10;
    paStack8 = (astruct_19 *)((u32)paStack8 & 0xffff0000 | (u32)((int)paStack8 + 0x6));
    iStack4 += -0x1;
  } while (iStack4 != 0x0);
  uVar3 = ((u32)param_1 >> 0x10);
  (u32)((int)&iVar2->field17_0x2c + 0x2) = 0x0;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x32)));
  (u32)&iVar2->field_0x38 = 0x0;
  param_1->offset_0x0 = 0x8a84;
  iVar2->base_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,(int)((u32)puVar4 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x29),in_stack_0000fe80,in_stack_0000ffa4,in_stack_0000ffaa,
                           in_stack_0000ffae);
  ((int)&iVar2->field17_0x2c + 0x2) = (int)puVar5;
  iVar2->field18_0x30 = ((u32)puVar5 >> 0x10);
  iStack10 = 0x0;
  do {
    uVar1 = (u32)((int)&iVar2->field17_0x2c + 0x2);
    pass1_1018_26d8(uVar1,((u32)uVar1 >> 0x10),iStack10,
                    (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field9_0x16 + iStack10 * 0x3)));
    uVar1 = (u32)((int)&iVar2->field17_0x2c + 0x2);
    pass1_1020_8712((u32)param_1 & 0xffff | (u32)uVar3 << 0x10,
                    (i16 *)CONCAT22(iVar2->field4_0xa,iVar2->field3_0x8 + iStack10 * 0x8),
                    *(astruct_76 **)((int)uVar1 + 0x2e + iStack10 * 0x4),
                    (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field9_0x16 + iStack10 * 0x3)));
    iStack10 += 0x1;
  } while (iStack10 < 0x4);
  uVar1 = (u32)((int)&iVar2->field17_0x2c + 0x2);
  pass1_1018_2548(uVar1,((u32)uVar1 >> 0x10),
                  (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x32)));
  uVar1 = (u32)((int)&iVar2->field17_0x2c + 0x2);
  (u32)&iVar2->field_0x38 = (u32)((int)uVar1 + 0x6e);
  pass1_1020_8712((u32)param_1 & 0xffff | (u32)uVar3 << 0x10,(i16 *)CONCAT22(0x1050,local_12),
                  *(astruct_76 **)&iVar2->field_0x38,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x32)));
  return;
}



void pass1_1020_8908(astruct_284 *param_1,u32 param_2,u32 param_3)

{
  astruct_76 *paVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u8 *puVar5;
  u16 uVar6;
  u16 uVar7;
  astruct_284 *pstruct284_8;
  i16 iVar9;
  i16 iVar10;
  u16 uVar11;
  u16 uVar12;
  u32 uVar13;
  astruct_110 *paStack28;
  i16 iStack4;
  astruct_57 *paVar8;

  for (iStack4 = 0x0; uVar12 = ((u32)param_3 >> 0x10), pstruct284_8 = (astruct_284 *)param_1,
      uVar11 = ((u32)param_1 >> 0x10), iStack4 < 0x4; iStack4 += 0x1) {
    if (pstruct284_8->field4_0x4 == 0x0) {
      uVar2 = pstruct284_8->field11_0xc;
      uVar12 = ((u32)uVar2 >> 0x10);
      iVar10 = (int)uVar2;
      iVar9 = iStack4 * 0x4;
      if (((iVar10 + iVar9 + 0x2) | (iVar10 + iVar9)) != 0x0) {
        pass1_1008_5236(*(astruct_109 **)(iVar10 + iVar9));
      }
    }
    else {
      uVar2 = pstruct284_8->field42_0x2e;
      paVar1 = *(astruct_76 **)((int)uVar2 + 0x2e + iStack4 * 0x4);
      uVar13 = pass1_1008_4772(paVar1);
      uVar6 = (uVar13 >> 0x10);
      paVar8 = (astruct_57 *)CONCAT22(uVar12,uVar6);
      uVar3 = uVar13;
      uVar2 = pstruct284_8->field11_0xc;
      iVar10 = iStack4 * 0x4;
      if (*(i32 *)((int)uVar2 + iVar10) == 0x0) {
        uVar4 = uVar3;
        mem_op_1000_179c(0x14,paVar8);
        uVar7 = paVar8;
        paStack28 = (astruct_110 *)CONCAT22(uVar7,uVar4);
        paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000);
        if ((uVar7 | uVar4) == 0x0) {
          uVar2 = pstruct284_8->field11_0xc;
          (u32)((int)uVar2 + iStack4 * 0x4) = 0x0;
        }
        else {
          paVar8 = (astruct_57 *)((u32)paVar8 | (u32)uVar11);
          puVar5 = &pstruct284_8->field_0x16 + iStack4 * 0x6;
          pass1_1008_50c2(paStack28,(u32)(uVar3 + 0x8),(u32)(uVar3 + 0x4),
                          (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(puVar5)),(astruct_76 *)param_2);
          uVar2 = pstruct284_8->field11_0xc;
          uVar12 = ((u32)uVar2 >> 0x10);
          iVar9 = (int)uVar2;
          (iVar9 + iVar10) = puVar5;
          (iVar9 + iVar10 + 0x2) = (int)paVar8;
        }
        uVar2 = pstruct284_8->field11_0xc;
        pass1_1008_5134((u32)((int)uVar2 + iStack4 * 0x4));
      }
      uVar12 = ((u32)paVar8 >> 0x10);
      uVar2 = pstruct284_8->field11_0xc;
      pass1_1008_5236(*(astruct_109 **)((int)uVar2 + iStack4 * 0x4));
      param_3 = CONCAT22(uVar12,uVar11);
      pass1_1008_4480((astruct_76 *)param_2,
                      (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&pstruct284_8->field_0x16 + iStack4 * 0x6)),paVar1);
    }
  }
  if (pstruct284_8->field4_0x4 != 0x0) {
    pass1_1008_4480((astruct_76 *)param_2,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&pstruct284_8->field_0x32)),
                    pstruct284_8->field49_0x38);
  }
  return;
}



StructD * pass1_1020_8a5e(StructD *param_1,u8 param_2)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_8a9c(astruct_20 *param_1,u32 param_2,u16 param_3,u16 param_4,u16 param_5,
                    u16 param_6)

{
  astruct_287 *paVar1;
  u32 uVar2;
  astruct_19 *uVar3;
  u16 uVar4;
  u16 uVar5;
  astruct_57 *paVar6;
  u16 unaff_SI;
  u16 *puVar7;
  u32 *puVar8;
  astruct_76 *paVar9;
  u16 uVar10;
  u16 *puStack76;
  u8 local_48 [0x1e];
  u8 local_2a [0x24];
  u16 uStack6;
  u16 uStack4;
  astruct_20 *iVar9;
  astruct_20 *uVar9;

  uVar4 = ((u32)param_2 >> 0x10);
  iVar9 = (astruct_20 *)param_1;
  uVar10 = ((u32)param_1 >> 0x10);
  struct_1020_847a(param_1,0x2);
  uVar3 = (astruct_19 *)&iVar9->field9_0x16;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar3)));
  puStack76 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar9->field12_0x1c));
  puVar7 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar9->field12_0x1c)));
  paVar6 = (astruct_57 *)CONCAT22(uVar4,(int)((u32)puVar7 >> 0x10));
  iVar9->field14_0x22 = NULL;
  param_1->offset_0x0 = 0x8e92;
  iVar9->base_0x2 = 0x1020;
  puVar8 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x29),param_6,param_5,param_3,param_4);
  uVar4 = ((u32)paVar6 >> 0x10);
  &iVar9->field14_0x22 = (int)puVar8;
  uVar5 = ((u32)puVar8 >> 0x10);
  ((int)&iVar9->field14_0x22 + 0x2) = uVar5;
  pass1_1018_2678(&iVar9->field14_0x22,uVar5,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar3)));
  paVar9 = (astruct_76 *)pass1_1018_268e(iVar9->field14_0x22);
  uStack4 = ((u32)paVar9 >> 0x10);
  paVar6 = (astruct_57 *)CONCAT22(uVar4,uStack4);
  uVar4 = SUB42(paVar9,0x0);
  uStack6 = uVar4;
  pass1_1020_8712((u32)param_1,*(i16 **)&iVar9->field3_0x8,paVar9,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar3))
                 );
  paVar1 = iVar9->field14_0x22;
  pass1_1018_26c2(paVar1,((u32)paVar1 >> 0x10),puStack76);
  uVar4 = FUN_1010_830a(uVar4,paVar6,0x1018,_u16_1050_14cc,0x2);
  struct_op_1008_48fe(paVar6,(astruct_81 *)CONCAT22(0x1050,local_2a),0x1,(char *)CONCAT22((int)paVar6,uVar4));
  struct_op_1008_3f92((astruct_76 *)CONCAT22(0x1050,local_48),(char *)CONCAT22(0x1050,local_2a));
  uVar2 = (u32)&iVar9->field3_0x8;
  pass1_1020_8712((u32)param_1,(i16 *)(uVar2 & 0xffff0000 | (u32)((int)uVar2 + 0x8)),
                  (astruct_76 *)CONCAT22(0x1050,local_48),puStack76);
  pass1_1008_41bc((astruct_288 *)CONCAT22(0x1050,local_48));
  close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,local_2a));
  return;
}



void pass1_1020_8bae(u16 *param_1)

{
  *param_1 = 0x8e92;
  ((int)param_1 + 0x2) = 0x1020;
  pass1_1020_8556((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_8bcc(astruct_285 *param_1,u32 param_2,u16 param_3,u16 param_4)

{
  u32 uVar1;
  astruct_109 *paVar2;
  u16 *puVar3;
  u16 *puVar4;
  u16 uVar5;
  u16 uVar6;
  astruct_57 *paVar7;
  astruct_285 *iVar9;
  astruct_286 *iVar10;
  u16 uVar9;
  u16 uVar10;
  u8 local_58 [0x1e];
  u8 local_3a [0x26];
  u32 uStack20;
  u16 uStack12;
  astruct_76 *paStack10;
  astruct_76 *paStack6;
  astruct_57 *paVar8;

  uVar10 = ((u32)param_2 >> 0x10);
  uVar9 = ((u32)param_1 >> 0x10);
  iVar9 = (astruct_285 *)param_1;
  if (iVar9->field4_0x4 != 0x0) {
    uVar1 = iVar9->field30_0x22;
    paStack6 = *(astruct_76 **)((int)uVar1 + 0xa);
    paStack10 = (astruct_76 *)pass1_1018_268e((astruct_287 *)iVar9->field30_0x22);
    uVar6 = ((u32)paStack10 >> 0x10);
    uVar1 = iVar9->field30_0x22;
    uStack12 = ((int)uVar1 + 0x16);
    if (*(i32 *)iVar9->field11_0xc == 0x0) {
      uStack20 = pass1_1008_4772(paStack10);
      paVar7 = (astruct_57 *)CONCAT22(uVar10,(int)(uStack20 >> 0x10));
      uVar6 = uStack20;
      mem_op_1000_179c(0x14,paVar7);
      uVar5 = paVar7 | uVar6;
      paVar8 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)uVar5);
      if (uVar5 == 0x0) {
        (u32)iVar9->field11_0xc = 0x0;
      }
      else {
        puVar3 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x16));
        uVar10 = (uStack20 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(paVar7,uVar6),(u32)((int)uStack20 + 0x8),
                        (u32)((int)uStack20 + 0x4),puVar3,paStack6);
        paVar2 = iVar9->field11_0xc;
        uVar6 = puVar3;
        paVar2 = uVar6;
        ((int)paVar2 + 0x2) = (int)paVar8;
      }
      pass1_1008_5134((u32)iVar9->field11_0xc);
      uVar10 = FUN_1010_830a(uVar6,paVar8,0x1008,_u16_1050_14cc,0x2);
      struct_op_1008_48fe(paVar8,(astruct_81 *)CONCAT22(0x1050,local_3a),0x1,(char *)CONCAT22((int)paVar8,uVar10));
      uVar10 = ((u32)paVar8 >> 0x10);
      struct_op_1008_3f92((astruct_76 *)CONCAT22(0x1050,local_58),(char *)CONCAT22(0x1050,local_3a));
      uStack20 = pass1_1008_4772((astruct_76 *)CONCAT22(0x1050,local_58));
      paVar7 = (astruct_57 *)CONCAT22(uVar10,(int)(uStack20 >> 0x10));
      uVar6 = uStack20;
      mem_op_1000_179c(0x14,paVar7);
      uVar6 = paVar7 | uVar6;
      if (uVar6 == 0x0) {
        paVar2 = iVar9->field11_0xc;
        (u32)((int)paVar2 + 0x4) = 0x0;
      }
      else {
        puVar3 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x16));
        uVar10 = (uStack20 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(paVar7,(int)((u32)paStack6 >> 0x10)),
                        (u32)((int)uStack20 + 0x8),(u32)((int)uStack20 + 0x4),puVar3,paStack6);
        paVar2 = iVar9->field11_0xc;
        uVar10 = ((u32)paVar2 >> 0x10);
        iVar10 = (astruct_286 *)paVar2;
        iVar10->field4_0x4 = puVar3;
        iVar10->field5_0x6 = uVar6;
      }
      paVar2 = iVar9->field11_0xc;
      pass1_1008_5134((u32)((int)paVar2 + 0x4));
      pass1_1008_41bc((astruct_288 *)CONCAT22(0x1050,local_58));
      close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,local_3a));
    }
    paVar2 = iVar9->field11_0xc;
    pass1_1008_5236(*(astruct_109 **)((int)paVar2 + 0x4));
    pass1_1008_5236(*(astruct_109 **)iVar9->field11_0xc);
    puVar4 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x16));
    pass1_1008_4480(paStack6,puVar4,paStack10);
    invalidate_rect_1020_8d90((int)puVar4,uVar6,(u32)param_1,uStack12,(u32)paStack6,param_3,param_4);
  }
  return;
}



void invalidate_rect_1020_8d90
               (u16 param_1,u16 param_2,u32 param_3,u16 param_4,u32 param_5,u16 param_6,
               u16 param_7)

{
  u32 uVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u32 uVar6;
  RECT16 local_48;
  i16 iStack68;
  i16 iStack66;
  i16 local_40;
  i16 local_3e;
  u32 uStack60;
  astruct_288 local_38;
  u8 local_10 [0xa];
  u16 uStack6;
  u16 uStack4;

  uVar5 = (param_3 >> 0x10);
  iVar4 = (int)param_3;
  uVar2 = pass1_1018_266a((u32)(iVar4 + 0x22));
  if (uVar2 != 0x0) {
    uVar6 = pass1_1018_265c();
    uStack4 = (uVar6 >> 0x10);
    uStack6 = uVar6;
    uVar3 = CONCAT22(in_register_0000000a,uStack4 | uStack6);
    if ((uStack4 | uStack6) != 0x0) {
      sys_1000_3f9c((char *)CONCAT22(0x1050,local_10),s__03ld_1050_442a,uStack6);
      uVar1 = (u32)(iVar4 + 0x22);
      file_and_draw_op_1008_4f20
                (uVar3,(astruct_76 *)CONCAT22(0x1050,&local_38),(u32)((int)uVar1 + 0xe),0x25,
                 (char *)CONCAT22(0x1050,local_10),param_6,param_7);
      pass1_1008_4480((astruct_76 *)param_5,(u16 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x1c)),
                      (astruct_76 *)CONCAT22(0x1050,&local_38));
      uStack60 = pass1_1008_4772((astruct_76 *)CONCAT22(0x1050,&local_38));
      pass1_1008_3e94((u16 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x1c)),(u16 *)CONCAT22(0x1050,&local_40),
                      (char *)CONCAT22(0x1050,&local_3e));
      local_48.x = local_3e;
      local_48.y = local_40;
      uVar5 = (uStack60 >> 0x10);
      iStack68 = local_3e + ((int)uStack60 + 0x4);
      iStack66 = local_40 + ((int)uStack60 + 0x8);
    // just 0x0
      InvalidateRect16(0x0,&local_48,(HWND16)&DAT_1050_1050);
      pass1_1008_41bc((astruct_288 *)CONCAT22(0x1050,&local_38));
    }
  }
  return;
}



StructD * pass1_1020_8e6c(StructD *param_1,u8 param_2)

{
  pass1_1020_8bae(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_8eaa(astruct_20 *param_1,u32 param_2)

{
  u8 *puVar1;
  astruct_19 *paVar2;
  u16 uVar3;
  u16 uVar5;
  astruct_57 *paVar6;
  astruct_20 *iVar4;
  u16 unaff_SI;
  u16 unaff_DI;
  astruct_20 *uVar4;
  u16 *puVar8;
  u32 *puVar9;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffca;
  u8 local_a [0x8];
  u16 uVar7;

  uVar7 = ((u32)param_2 >> 0x10);
  struct_1020_847a(param_1,0x25);
  uVar4 = (astruct_20 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  (u32)&iVar4->field9_0x16 = 0x0;
  (u32)&iVar4->field138_0xaa = 0x0;
  puVar1 = (u8 *)((int)&iVar4->field139_0xac + 0x2);
  puVar8 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(puVar1)));
  paVar6 = (astruct_57 *)CONCAT22(uVar7,(int)((u32)puVar8 >> 0x10));
  (u32)&iVar4->field141_0xb4 = 0x0;
  iVar4->field143_0xb8 = 0xffff;
  (u32)&iVar4->field144_0xba = 0x0;
  param_1->offset_0x0 = 0x9204;
  iVar4->base_0x2 = 0x1020;
  puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x29),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)puVar9 >> 0x10);
  paVar2 = (astruct_19 *)puVar9;
  iVar4->field9_0x16 = paVar2;
  uVar5 = ((u32)puVar9 >> 0x10);
  iVar4->field10_0x18 = uVar5;
  pass1_1018_2646(iVar4->field9_0x16,uVar5,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(puVar1)));
  uVar3 = FUN_1010_830a(paVar2,paVar6,0x1018,_u16_1050_14cc,0x1ce);
  iVar4->field141_0xb4 = uVar3;
  iVar4->field142_0xb6 = (int)paVar6;
  pass1_1020_8712((u32)param_1 & 0xffff | ZEXT24(uVar4) << 0x10,(i16 *)CONCAT22(0x1050,local_a),
                  (astruct_76 *)CONCAT22((int)paVar6,iVar4->field141_0xb4),
                  (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(puVar1)));
  puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_DI,0x2),in_stack_0000fe9c,
                           in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  iVar4->field144_0xba = puVar9;
  iVar4->field145_0xbc = ((u32)puVar9 >> 0x10);
  return;
}



void pass1_1020_8f74(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  StructD *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x9204;
  iVar4->address_offset_field_0x2 = 0x1020;
  puVar1 = (u32*)&iVar4->field_0xb4;
  uVar2 = &iVar4->field_0xb6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1020_8556(param_1);
  return;
}



void invalidate_rect_1020_8fb4(u16 param_1,u16 param_2,u32 param_3)

{
  u32 uVar1;
  RECT16 *rect;
  u32 uVar2;
  u16 DX_REG;
  u16 hwnd;
  i16 iVar3;
  u16 uVar4;
  i16 iStack8;

  uVar4 = (param_3 >> 0x10);
  iVar3 = (int)param_3;
  uVar1 = (u32)(iVar3 + 0xba);
  if (((int)uVar1 + 0x1e) != 0x0) {
    pass1_1018_2862(*(astruct_654 **)(iVar3 + 0x16));
    (iVar3 + 0xaa) = param_1;
    (iVar3 + 0xac) = param_2;
    if ((param_2 | (iVar3 + 0xaa)) != 0x0) {
      uVar1 = (u32)(iVar3 + 0xaa);
      iVar3 = ((int)uVar1 + 0xa);
      for (iStack8 = 0x0; iStack8 < iVar3; iStack8 += 0x1) {
        uVar2 = (u32)iStack8;
        empty_1008_8fc4();
        rect = (RECT16 *)uVar2;
        hwnd = DX_REG | rect;
        if (((hwnd != 0x0) && (0x9 < rect[0xb].y)) &&
           (pass1_1008_8b20(uVar2 & 0xffff | (u32)DX_REG << 0x10), (hwnd | rect) != 0x0)) {
          InvalidateRect16(0x0,rect,hwnd);
        }
      }
    }
  }
  return;
}



void pass1_1020_9068(u32 param_1,u32 *param_2,u32 param_3)

{
  i16 iVar1;
  astruct_76 *paVar2;
  code **ppcVar3;
  u32 uVar4;
  u16 uVar5;
  astruct_76 *paVar6;
  u32 uVar7;
  u16 uVar8;
  i16 iVar10;
  i16 iVar11;
  u16 uVar12;
  u16 uVar13;
  i16 iStack10;
  u32 uVar9;

  uVar12 = ((u32)param_2 >> 0x10);
  iVar10 = (int)param_2;
  uVar4 = (u32)(iVar10 + 0x16);
  paVar2 = *(astruct_76 **)((int)uVar4 + 0xa);
  paVar6 = paVar2;
  pass1_1018_280c((u32)(iVar10 + 0x16));
  (iVar10 + 0xaa) = (int)paVar6;
  (iVar10 + 0xac) = param_1;
  uVar5 = param_1 | (iVar10 + 0xaa);
  if (uVar5 == 0x0) {
    pass1_1018_2862(*(astruct_654 **)(iVar10 + 0x16));
    (iVar10 + 0xaa) = uVar5;
    (iVar10 + 0xac) = (int)param_1;
  }
  if (((iVar10 + 0xac) | (iVar10 + 0xaa)) != 0x0) {
    pass1_1020_915a((u8 *)param_1,(astruct_669 *)((u32)param_2 & 0xffff | (u32)uVar12 << 0x10),param_3);
    pass1_1008_4480(paVar2,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(iVar10 + 0xae)),*(astruct_76 **)(iVar10 + 0xb4)
                   );
    ppcVar3 = (code **)((int)*param_2 + 0x10);
    (**ppcVar3)();
    uVar4 = (u32)(iVar10 + 0xaa);
    iVar1 = ((int)uVar4 + 0xa);
    for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
      uVar7 = (u32)iStack10;
      empty_1008_8fc4();
      uVar5 = uVar7;
      uVar8 = param_1 | uVar5;
      uVar9 = param_1 & 0xffff0000 | (u32)uVar8;
      if (uVar8 != 0x0) {
        pass1_1008_8c4e(uVar7 & 0xffff | param_1 << 0x10,(u32)paVar2,uVar9);
        uVar4 = (u32)(iVar10 + 0xc);
        uVar13 = ((u32)uVar4 >> 0x10);
        iVar11 = (int)uVar4;
        (iVar11 + iStack10 * 0x4) = uVar5;
        (iVar11 + iStack10 * 0x4 + 0x2) = (int)uVar9;
      }
      param_1 = uVar9;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_915a(u8 *param_1,astruct_669 *param_2,u8 **param_3)

{
  i16 iVar1;
  i16 iVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  u32 uVar5;
  astruct_669 *iVar2;
  u16 uVar6;
  u32 *puVar7;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  param_3 = (u8 **)CONCAT22(param_3,0x2f);
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,param_3,in_stack_0000fe9c,in_stack_0000ffc0,in_stack_0000ffc6,
                           in_stack_0000ffca);
  uVar5 = (u32)paVar4 & 0xffff0000 | (u32)puVar7 >> 0x10;
  iVar1 = ((int)puVar7 + 0x1e);
  uVar6 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_669 *)param_2;
  if (iVar2->field182_0xb8 != iVar1) {
    param_3 = 0x1ce;
    iVar3 = iVar1 + -0x1;
    if (iVar3 == 0x0) {
      param_3 = 0x1cf;
    }
    else {
      iVar3 = iVar1 + -0x2;
      if (iVar3 == 0x0) {
        param_3 = 0x1d0;
      }
      else {
        iVar3 = iVar1 + -0x3;
        if (iVar3 == 0x0) {
          param_3 = 0x1d1;
        }
        else {
          iVar3 = iVar1 + -0x4;
          if (iVar3 == 0x0) {
            param_3 = 0x1d2;
          }
        }
      }
    }
    iVar3 = FUN_1010_830a(iVar3,uVar5,0x1010,_u16_1050_14cc,param_3);
    iVar2->field180_0xb4 = iVar3;
    iVar2->field181_0xb6 = (int)uVar5;
    iVar2->field182_0xb8 = iVar1;
  }
  return;
}



StructD * pass1_1020_91de(StructD *param_1,u8 param_2)

{
  pass1_1020_8f74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void set_struct_op_1020_921c(u16 param_1,StructA *pstructa_param_2,u16 param_3,u8 **param_4)

{
  u16 uVar1;
  HDC16 HVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  StructA *iVar3;
  StructA *uVar3;
  u16 *pUVar3;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u32 in_stack_0000ffe2;
  u16 uVar2;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = (StructA *)((u32)pstructa_param_2 >> 0x10);
  iVar3 = (StructA *)pstructa_param_2;
  pstructa_param_2->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  pstructa_param_2->field0_0x0 = 0x3aa8;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field2_0x4 = param_3;
  pstructa_param_2->field0_0x0 = 0x3ab0;
  iVar3->field1_0x2 = 0x1008;
  (u32)&iVar3->field3_0x6 = 0x0;
  iVar3->field5_0xa = 0x0;
  iVar3->field6_0xc = 0x0;
  iVar3->field7_0xe = 0x0;
  iVar3->field8_0x10 = 0x0;
  iVar3->field9_0x12 = 0x0;
  pstructa_param_2->field0_0x0 = 0x96c8;
  iVar3->field1_0x2 = 0x1020;
  HVar2 = GetDC16(iVar3->field2_0x4);
  iVar3->field5_0xa = HVar2;
  param_4 = (u8 **)CONCAT22(param_4,0x48);
  pUVar3 = (u16 *)
           mixed_1010_20ba(paVar3,_u16_1050_0ed0,param_4,in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,
                           in_stack_0000ffba);
  uVar1 = ((u32)pUVar3 >> 0x10);
  iVar3->field6_0xc = ((int)pUVar3 + 0xa);
  iVar3->field7_0xe = ((int)pUVar3 + 0xc);
  return;
}



void palette_op_1020_92c4(StructD *struct_param_1)

{
  HPALETTE16 obj;
  StructD *struct_1;
  StructD *uVar2;

  uVar2 = (StructD *)((u32)struct_param_1 >> 0x10);
  struct_1 = (StructD *)struct_param_1;
  struct_param_1->address_offset_field_0x0 = 0x96c8;
  struct_1->address_offset_field_0x2 = 0x1020;
  if (struct_1->field11_0x12 != 0x0) {
    obj = SelectPalette16(0x0,struct_1->field11_0x12,struct_1->field6_0xa);
    DeleteObject16(obj);
  }
  struct_param_1->address_offset_field_0x0 = 0x3ab0;
  struct_1->address_offset_field_0x2 = 0x1008;
  struct_param_1->address_offset_field_0x0 = 0x389a;
  struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



void mix_draw_op_1020_9312(u32 param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u8 local_22 [0x20];

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),*(HWND16 *)(iVar4 + 0x4));
  uVar3 = (u32)(iVar4 + 0x6);
  puVar1 = (u32 *)(u32)((int)uVar3 + 0xa);
  ppcVar2 = (code **)((int)*puVar1 + 0x4);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puVar1,(int)((u32)puVar1 >> 0x10),0x0,
              param_1 & 0xffff0000 | (u32)(iVar4 + 0xa));
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),*(HWND16 *)(iVar4 + 0x4));
  return;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Could not reconcile some variable overlaps

void draw_op_1020_9364(StructA *param_1)

{
  i16 *piVar1;
  u16 *puVar2;
  HBRUSH16 brush_handle_var6;
  StructA *local_struct_1;
  u16 var7;
  u16 uVar7;
  u16 uVar3;
  u16 uVar8;
  HDC16 HVar3;
  i16 iStack62;
  astruct_737 *uStack58;
  POINT16 point16_38;
  HGDIOBJ16 hgdiobj16_var_52;
  HPEN16 HStack50;
  HDC16 hdc16_var_48;
  u32 uStack46;
  u32 uStack42;
  u32 uStack38;
  u32 uStack34;
  u32 *puStack30;
  u16 *puStack26;
  i16 iStack22;
  u16 uStack20;
  RECT16 rect16_var_12;
  u32 uStack14;
  RECT16 rect16_a;
  u32 x_var_6;
  u16 uVar2;
  i16 iVar2;
  i16 *piVar2;
  u32 uVar4;
  i16 iVar4;
  u8 uVar10;
  u8 uVar11;

  var7 = ((u32)param_1 >> 0x10);
  local_struct_1 = (StructA *)param_1;
  GetClientRect16(&rect16_a,(HWND16)&DAT_1050_1050);
  rect16_var_12 = rect16_a;
  uStack14 = x_var_6;
  uStack20 = DAT_1050_4216;
  iStack22 = DAT_1050_422c;
  puStack26 = (u16 *)PTR_u16_1050_4172_1050_4212;
  puStack30 = (u32 *)PTR_u16_1050_41b2_1050_4218;
  uStack34 = (u32)PTR_u16_1050_41da_1050_421c;
  uStack38 = (u32)PTR_u16_1050_4202_1050_4220;
  uStack42 = (u32)PTR_u16_1050_419a_1050_4224;
  uStack46 = (u32)PTR_u16_1050_41aa_1050_4228;
  uVar4 = (u32)&local_struct_1->field3_0x6;
  hdc16_var_48 = *(HDC16 *)((int)uVar4 + 0x12);
  uStack58 = (astruct_737 *)((int)&u16_1050_0008 + 0x1);
  do {
    HVar3 = hdc16_var_48;
    HStack50 = CreatePen16(*(COLORREF *)((int)uStack58 * 0x4 + (int)uStack34),0x0,0x0);
    hgdiobj16_var_52 = SelectObject16(HStack50,HVar3);
    MoveToEx16(&point16_38,(INT16)&DAT_1050_1050,*(INT16 *)((int)uStack58 * 0x2 + (int)puStack26),rect16_a.x);
    LineTo16(*(INT16 *)((int)puStack26 + (int)uStack58 * 0x2),(INT16)x_var_6,hdc16_var_48);
    iVar4 = (uStack20 - (int)uStack58) * 0x2;
    MoveToEx16(&point16_38,(INT16)&DAT_1050_1050,*(INT16 *)(iVar4 + (int)puStack26),rect16_a.x);
    LineTo16(*(INT16 *)((int)puStack26 + iVar4),(INT16)x_var_6,hdc16_var_48);
    SelectObject16(hgdiobj16_var_52,hdc16_var_48);
    DeleteObject16(HStack50);
    uStack58 = (astruct_737 *)((int)&uStack58[-0x1].field0_0x0 + 0x1);
  } while (uStack58 < (astruct_737 *)0x8000);
  brush_handle_var6 = CreateSolidBrush16(0x2000000);
  uVar7 = ((u32)puStack26 >> 0x10);
  rect16_a = (RECT16)CONCAT22(((int)puStack26 + 0x12) + 0x1,rect16_a.x);
  uVar2 = ((int)puStack26 + 0x14);
  uStack14 = uStack14 & 0xffff | (u32)uVar2 << 0x10;
  x_var_6 = CONCAT22(uVar2,(INT16)x_var_6);
  FillRect16(brush_handle_var6,&rect16_a,(HDC16)&DAT_1050_1050);
  DeleteObject16(brush_handle_var6);
  iStack62 = 0x8;
  puVar2 = (u16 *)&PTR_LOOP_1050_0000;
  while (uStack58 = (astruct_737 *)((int)puVar2 + 0x1), (int)uStack58 < 0xa) {
    brush_handle_var6 = CreateSolidBrush16(*(COLORREF *)((int)puStack30 + iStack62 * 0x4 + 0x4));
    x_var_6 = x_var_6 & 0xffff | (u32)(rect16_a.y - 0x1) << 0x10;
    rect16_var_12 = (RECT16)((u32)rect16_var_12 & 0xffff | (u32)(uStack14 + 0x1) << 0x10);
    uVar3 = ((u32)puStack26 >> 0x10);
    rect16_a = (RECT16)((u32)rect16_a & 0xffff | (u32)((iStack62 * 0x2 + (int)puStack26) + 0x1) << 0x10);
    uStack14 = uStack14 & 0xffff | (u32)((int)uStack58 * 0x2 + (int)puStack26 + 0x14) << 0x10;
    FillRect16(brush_handle_var6,&rect16_a,(HDC16)&DAT_1050_1050);
    FillRect16(brush_handle_var6,&rect16_var_12,(HDC16)&DAT_1050_1050);
    DeleteObject16(brush_handle_var6);
    iStack62 += -0x1;
    puVar2 = &uStack58->field0_0x0;
  }
  brush_handle_var6 = CreateSolidBrush16(*puStack30);
  rect16_a = (RECT16)((u32)rect16_a & 0xffff);
  x_var_6 = x_var_6 & 0xffff | (u32)*puStack26 << 0x10;
  rect16_var_12 =
       (RECT16)((u32)rect16_var_12 & 0xffff | (u32)((uStack20 * 0x2 + (int)puStack26) + 0x1) << 0x10);
  uStack14 = uStack14 & 0xffff | (u32)local_struct_1->field7_0xe << 0x10;
  FillRect16(brush_handle_var6,&rect16_a,(HDC16)&DAT_1050_1050);
  FillRect16(brush_handle_var6,&rect16_var_12,(HDC16)&DAT_1050_1050);
  DeleteObject16(brush_handle_var6);
  uStack58 = (astruct_737 *)((int)&u16_1050_0002 + 0x1);
  do {
    HVar3 = hdc16_var_48;
    HStack50 = CreatePen16(*(COLORREF *)((int)uStack58 * 0x4 + (int)uStack38),0x0,0x0);
    hgdiobj16_var_52 = SelectObject16(HStack50,HVar3);
    iVar2 = (int)uStack58 * 0x2;
    rect16_a.x = (iVar2 + (int)uStack42) + rect16_a.x;
    uVar8 = (uStack46 >> 0x10);
    piVar1 = (i16 *)(iVar2 + (int)uStack46);
    MoveToEx16(&point16_38,(INT16)&DAT_1050_1050,*(INT16 *)((iVar2 + (int)uStack46) * 0x2 + (int)puStack26),
               rect16_a.x);
    LineTo16(*(INT16 *)((uStack20 - *piVar1) * 0x2 + (int)puStack26),rect16_a.x,hdc16_var_48);
    rect16_a.x = ((iStack22 - (int)uStack58) * 0x2 + (int)uStack42) + rect16_a.x;
    MoveToEx16(&point16_38,(INT16)&DAT_1050_1050,*(INT16 *)(*piVar1 * 0x2 + (int)puStack26),rect16_a.x);
    LineTo16(*(INT16 *)((uStack20 - *piVar1) * 0x2 + (int)puStack26),rect16_a.x,hdc16_var_48);
    SelectObject16(hgdiobj16_var_52,hdc16_var_48);
    DeleteObject16(HStack50);
    uStack58 = (astruct_737 *)((int)&uStack58[-0x1].field0_0x0 + 0x1);
  } while (uStack58 < (astruct_737 *)0x8000);
  local_struct_1->field8_0x10 = 0x0;
  return;
}



StructD * pass1_1020_96a2(StructD *param_1,u8 param_2)

{
  palette_op_1020_92c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void init_globals_1020_96d4(void)

{
  u16 *puVar1;
  i16 iVar2;
  u16 *puVar3;

  PTR_LOOP_1050_4514 = 0x0;
  PTR_LOOP_1050_451a = 0x0;
  PTR_LOOP_1050_4520 = (u8 *)0x4430;
  PTR_LOOP_1050_4522 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4526 = (u8 *)0x4430;
  PTR_LOOP_1050_4528 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4524 = PTR_u16_1050_0002_1050_4434;
  PTR_LOOP_1050_452a = PTR_u16_1050_0002_1050_4434;
  PTR_LOOP_1050_452c = (u8 *)0x4430;
  PTR_LOOP_1050_452e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4530 = PTR_u16_1050_0002_1050_4434;
  PTR_LOOP_1050_4532 = (u8 *)0x4430;
  PTR_LOOP_1050_4534 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4536 = PTR_u16_1050_0002_1050_4434;
  PTR_LOOP_1050_4538 = 0x0;
  PTR_LOOP_1050_453e = 0x0;
  PTR_LOOP_1050_4544 = (u8 *)0x4436;
  PTR_LOOP_1050_4546 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_454a = (u8 *)0x4436;
  PTR_LOOP_1050_454c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4548 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_454e = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_4550 = (u8 *)0x4436;
  PTR_LOOP_1050_4552 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4554 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_4512 = DAT_1050_4462;
  PTR_LOOP_1050_455a = DAT_1050_4462;
  PTR_LOOP_1050_4556 = (u8 *)0x4454;
  PTR_LOOP_1050_4558 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_455c = (u8 *)0x4454;
  PTR_LOOP_1050_455e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4560 = DAT_1050_4462;
  PTR_LOOP_1050_4562 = (u8 *)0x4454;
  PTR_LOOP_1050_4564 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4566 = DAT_1050_4462;
  PTR_LOOP_1050_456a = NULL;
  PTR_LOOP_1050_4568 = NULL;
  PTR_LOOP_1050_456e = (u8 *)0x443c;
  PTR_LOOP_1050_4570 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4574 = (u8 *)0x443c;
  PTR_LOOP_1050_4576 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4572 = DAT_1050_4446;
  PTR_LOOP_1050_4578 = DAT_1050_4446;
  PTR_LOOP_1050_457a = (u8 *)0x443c;
  PTR_LOOP_1050_457c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_457e = DAT_1050_4446;
  PTR_LOOP_1050_4580 = (u8 *)0x443c;
  PTR_LOOP_1050_4582 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4584 = DAT_1050_4446;
  PTR_LOOP_1050_4586 = (u8 *)0x443c;
  PTR_LOOP_1050_4588 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_458a = DAT_1050_4446;
  PTR_LOOP_1050_458c = (u8 *)0x443c;
  PTR_LOOP_1050_458e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4590 = DAT_1050_4446;
  PTR_LOOP_1050_4592 = (u8 *)0x4454;
  PTR_LOOP_1050_4594 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4596 = DAT_1050_4462;
  PTR_LOOP_1050_4598 = (u8 *)0x4454;
  PTR_LOOP_1050_459a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_459c = DAT_1050_4462;
  PTR_LOOP_1050_459e = (u8 *)0x4436;
  PTR_LOOP_1050_45a0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_45a2 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_45a4 = (u8 *)0x4436;
  PTR_LOOP_1050_45a6 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_45a8 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_45aa = 0x0;
  PTR_LOOP_1050_45b0 = 0x0;
  PTR_LOOP_1050_45b6 = 0x0;
  PTR_LOOP_1050_45bc = (u8 *)0x443c;
  PTR_LOOP_1050_45be = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_45c0 = DAT_1050_4446;
  PTR_LOOP_1050_45c2 = (u8 *)0x443c;
  PTR_LOOP_1050_45c4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_45c6 = DAT_1050_4446;
  PTR_LOOP_1050_45c8 = 0x0;
  PTR_LOOP_1050_45ce = 0x0;
  PTR_LOOP_1050_45d4 = 0x0;
  PTR_LOOP_1050_45da = 0x0;
  PTR_LOOP_1050_45e0 = (u8 *)0x443c;
  PTR_LOOP_1050_45e2 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_45e4 = DAT_1050_4446;
  PTR_LOOP_1050_45e6 = (u8 *)0x443c;
  PTR_LOOP_1050_45e8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_45ea = DAT_1050_4446;
  PTR_LOOP_1050_45ec = 0x0;
  PTR_LOOP_1050_45f2 = 0x0;
  PTR_LOOP_1050_45f8 = 0x0;
  PTR_LOOP_1050_45fe = (u8 *)0x443c;
  PTR_LOOP_1050_4600 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4602 = DAT_1050_4446;
  PTR_LOOP_1050_4604 = (u8 *)0x443c;
  PTR_LOOP_1050_4606 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4608 = DAT_1050_4446;
  PTR_LOOP_1050_460a = 0x0;
  PTR_LOOP_1050_4610 = 0x0;
  PTR_LOOP_1050_451e = (u8 *)0xffff;
  PTR_LOOP_1050_45ae = (u8 *)0xffff;
  PTR_LOOP_1050_45b4 = (u8 *)0xffff;
  PTR_LOOP_1050_45ba = (u8 *)0xffff;
  PTR_LOOP_1050_45cc = (u8 *)0xffff;
  PTR_LOOP_1050_45d2 = (u8 *)0xffff;
  PTR_LOOP_1050_45f6 = (u8 *)0xffff;
  PTR_LOOP_1050_45fc = (u8 *)0xffff;
  PTR_LOOP_1050_460e = (u8 *)0xffff;
  PTR_LOOP_1050_4614 = (u8 *)0xffff;
  PTR_LOOP_1050_4616 = 0x0;
  PTR_LOOP_1050_461c = 0x0;
  PTR_LOOP_1050_4622 = 0x0;
  PTR_LOOP_1050_4628 = 0x0;
  PTR_LOOP_1050_462e = 0x0;
  PTR_LOOP_1050_4634 = 0x0;
  PTR_LOOP_1050_4518 = NULL;
  PTR_LOOP_1050_453c = NULL;
  PTR_LOOP_1050_4542 = NULL;
  PTR_LOOP_1050_456c = NULL;
  PTR_LOOP_1050_45d8 = NULL;
  PTR_LOOP_1050_45de = NULL;
  PTR_LOOP_1050_45f0 = NULL;
  PTR_LOOP_1050_461a = NULL;
  PTR_LOOP_1050_4620 = NULL;
  PTR_LOOP_1050_4626 = NULL;
  PTR_LOOP_1050_462c = NULL;
  PTR_LOOP_1050_4632 = NULL;
  PTR_LOOP_1050_4638 = NULL;
  PTR_LOOP_1050_463a = 0x0;
  PTR_LOOP_1050_4640 = 0x0;
  PTR_LOOP_1050_4646 = 0x0;
  PTR_LOOP_1050_464c = 0x0;
  PTR_LOOP_1050_4652 = 0x0;
  PTR_LOOP_1050_4658 = 0x0;
  PTR_LOOP_1050_465e = (u8 *)0x4448;
  PTR_LOOP_1050_4660 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4664 = (u8 *)0x4448;
  PTR_LOOP_1050_4666 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4662 = DAT_1050_4452;
  PTR_LOOP_1050_4668 = DAT_1050_4452;
  PTR_LOOP_1050_466a = (u8 *)0x4448;
  PTR_LOOP_1050_466c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_466e = DAT_1050_4452;
  PTR_LOOP_1050_4670 = (u8 *)0x4454;
  PTR_LOOP_1050_4672 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4676 = (u8 *)0x4454;
  PTR_LOOP_1050_4678 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4674 = DAT_1050_4462;
  PTR_LOOP_1050_467a = DAT_1050_4462;
  PTR_LOOP_1050_467c = (u8 *)0x4454;
  PTR_LOOP_1050_467e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4680 = DAT_1050_4462;
  PTR_LOOP_1050_4682 = (u8 *)0x4454;
  PTR_LOOP_1050_4684 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4686 = DAT_1050_4462;
  PTR_LOOP_1050_4688 = (u8 *)0x4454;
  PTR_LOOP_1050_468a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_468c = DAT_1050_4462;
  PTR_LOOP_1050_468e = (u8 *)0x4448;
  PTR_LOOP_1050_4690 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4692 = DAT_1050_4452;
  PTR_LOOP_1050_4694 = (u8 *)0x4448;
  PTR_LOOP_1050_4696 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4698 = DAT_1050_4452;
  PTR_LOOP_1050_469a = (u8 *)0x4448;
  PTR_LOOP_1050_469c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_469e = DAT_1050_4452;
  PTR_LOOP_1050_46a0 = (u8 *)0x4448;
  PTR_LOOP_1050_46a2 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46a4 = DAT_1050_4452;
  PTR_LOOP_1050_46a6 = (u8 *)0x4454;
  PTR_LOOP_1050_46a8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46aa = DAT_1050_4462;
  PTR_LOOP_1050_46ac = (u8 *)0x4454;
  PTR_LOOP_1050_46ae = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46b0 = DAT_1050_4462;
  PTR_LOOP_1050_46b2 = (u8 *)0x4454;
  PTR_LOOP_1050_46b4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46b6 = DAT_1050_4462;
  PTR_LOOP_1050_46b8 = (u8 *)0x4454;
  PTR_LOOP_1050_46ba = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46bc = DAT_1050_4462;
  PTR_LOOP_1050_46be = (u8 *)0x4454;
  PTR_LOOP_1050_46c0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46c2 = DAT_1050_4462;
  PTR_LOOP_1050_46c6 = NULL;
  PTR_LOOP_1050_46c4 = NULL;
  PTR_LOOP_1050_46cc = NULL;
  PTR_LOOP_1050_46ca = NULL;
  PTR_LOOP_1050_46d2 = NULL;
  PTR_LOOP_1050_46d0 = NULL;
  PTR_LOOP_1050_46d8 = NULL;
  PTR_LOOP_1050_46d6 = NULL;
  PTR_LOOP_1050_46de = NULL;
  PTR_LOOP_1050_46dc = NULL;
  PTR_LOOP_1050_46e2 = (u8 *)0x4454;
  PTR_LOOP_1050_46e4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46e6 = DAT_1050_4462;
  PTR_LOOP_1050_46e8 = (u8 *)0x4448;
  PTR_LOOP_1050_46ea = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46ec = DAT_1050_4452;
  PTR_LOOP_1050_46ee = (u8 *)0x4448;
  PTR_LOOP_1050_46f0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_46f2 = DAT_1050_4452;
  PTR_LOOP_1050_46f4 = 0x0;
  PTR_LOOP_1050_46fa = 0x0;
  PTR_LOOP_1050_46f8 = (u8 *)0xffff;
  PTR_LOOP_1050_46fe = (u8 *)0xffff;
  PTR_LOOP_1050_4700 = 0x0;
  PTR_LOOP_1050_4706 = 0x0;
  PTR_LOOP_1050_470c = (u8 *)0x4448;
  PTR_LOOP_1050_470e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4710 = DAT_1050_4452;
  PTR_LOOP_1050_4712 = (u8 *)0x4448;
  PTR_LOOP_1050_4714 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4716 = DAT_1050_4452;
  PTR_LOOP_1050_4718 = 0x0;
  PTR_LOOP_1050_471e = 0x0;
  PTR_LOOP_1050_4724 = 0x0;
  PTR_LOOP_1050_472a = 0x0;
  PTR_LOOP_1050_4730 = 0x0;
  PTR_LOOP_1050_4736 = 0x0;
  PTR_LOOP_1050_473c = 0x0;
  PTR_LOOP_1050_4742 = 0x0;
  PTR_LOOP_1050_4748 = 0x0;
  PTR_LOOP_1050_474e = 0x0;
  PTR_LOOP_1050_4754 = 0x0;
  PTR_LOOP_1050_475a = 0x0;
  PTR_LOOP_1050_4760 = 0x0;
  PTR_LOOP_1050_463e = NULL;
  PTR_LOOP_1050_4644 = NULL;
  PTR_LOOP_1050_464a = NULL;
  PTR_LOOP_1050_4650 = NULL;
  PTR_LOOP_1050_4656 = NULL;
  PTR_LOOP_1050_465c = NULL;
  PTR_LOOP_1050_46c8 = NULL;
  PTR_LOOP_1050_46ce = NULL;
  PTR_LOOP_1050_46d4 = NULL;
  PTR_LOOP_1050_46da = NULL;
  PTR_LOOP_1050_46e0 = NULL;
  PTR_LOOP_1050_4704 = NULL;
  PTR_LOOP_1050_470a = NULL;
  PTR_LOOP_1050_471c = NULL;
  PTR_LOOP_1050_4722 = NULL;
  PTR_LOOP_1050_4728 = NULL;
  PTR_LOOP_1050_472e = NULL;
  PTR_LOOP_1050_4734 = NULL;
  PTR_LOOP_1050_473a = NULL;
  PTR_LOOP_1050_4740 = NULL;
  PTR_LOOP_1050_4746 = NULL;
  PTR_LOOP_1050_474c = NULL;
  PTR_LOOP_1050_4752 = NULL;
  PTR_LOOP_1050_4758 = NULL;
  PTR_LOOP_1050_475e = NULL;
  PTR_LOOP_1050_4764 = NULL;
  PTR_LOOP_1050_4766 = 0x0;
  PTR_LOOP_1050_476c = 0x0;
  PTR_LOOP_1050_4772 = 0x0;
  PTR_LOOP_1050_4778 = 0x0;
  PTR_LOOP_1050_477e = 0x0;
  PTR_LOOP_1050_4784 = 0x0;
  PTR_LOOP_1050_478a = 0x0;
  PTR_LOOP_1050_4790 = 0x0;
  PTR_LOOP_1050_4796 = 0x0;
  PTR_LOOP_1050_479c = 0x0;
  PTR_LOOP_1050_47a2 = 0x0;
  PTR_LOOP_1050_47a8 = 0x0;
  PTR_LOOP_1050_47ae = 0x0;
  PTR_LOOP_1050_47b4 = 0x0;
  PTR_LOOP_1050_476a = NULL;
  PTR_LOOP_1050_4770 = NULL;
  PTR_LOOP_1050_4776 = NULL;
  PTR_LOOP_1050_477c = NULL;
  PTR_LOOP_1050_4782 = NULL;
  PTR_LOOP_1050_4788 = NULL;
  PTR_LOOP_1050_478e = NULL;
  PTR_LOOP_1050_4794 = NULL;
  PTR_LOOP_1050_479a = NULL;
  PTR_LOOP_1050_47a0 = NULL;
  PTR_LOOP_1050_47a6 = NULL;
  PTR_LOOP_1050_47ac = NULL;
  PTR_LOOP_1050_47b2 = NULL;
  PTR_LOOP_1050_47b8 = NULL;
  puVar3 = (u16 *)0x47ba;
  for (iVar2 = 0x1b; iVar2 != 0x0; iVar2 += -0x1) {
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar1 = 0x0;
  }
  PTR_LOOP_1050_4850 = 0x0;
  PTR_LOOP_1050_4856 = 0x0;
  PTR_LOOP_1050_484e = PTR_u16_1050_0002_1050_4468;
  PTR_LOOP_1050_4860 = PTR_u16_1050_0002_1050_4468;
  PTR_LOOP_1050_485c = (u8 *)0x4464;
  PTR_LOOP_1050_485e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4862 = (u8 *)0x4464;
  PTR_LOOP_1050_4864 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4866 = PTR_u16_1050_0002_1050_4468;
  PTR_LOOP_1050_4868 = (u8 *)0x4464;
  PTR_LOOP_1050_486a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_486c = PTR_u16_1050_0002_1050_4468;
  PTR_LOOP_1050_486e = (u8 *)0x4464;
  PTR_LOOP_1050_4870 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4872 = PTR_u16_1050_0002_1050_4468;
  PTR_LOOP_1050_4874 = 0x0;
  PTR_LOOP_1050_487a = 0x0;
  PTR_LOOP_1050_4880 = (u8 *)0x4436;
  PTR_LOOP_1050_4882 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4886 = (u8 *)0x4436;
  PTR_LOOP_1050_4888 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4884 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_488a = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_488c = (u8 *)0x4436;
  PTR_LOOP_1050_488e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4890 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_4892 = (u8 *)0x4482;
  PTR_LOOP_1050_4894 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4898 = (u8 *)0x4482;
  PTR_LOOP_1050_489a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4896 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_489c = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_489e = (u8 *)0x4482;
  PTR_LOOP_1050_48a0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48a2 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_48a6 = NULL;
  PTR_LOOP_1050_48a4 = NULL;
  PTR_LOOP_1050_48aa = (u8 *)0x4488;
  PTR_LOOP_1050_48ac = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48b0 = (u8 *)0x4488;
  PTR_LOOP_1050_48b2 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48ae = PTR_u16_1050_0002_1050_448c;
  PTR_LOOP_1050_48b4 = PTR_u16_1050_0002_1050_448c;
  PTR_LOOP_1050_48b6 = (u8 *)0x4488;
  PTR_LOOP_1050_48b8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48ba = PTR_u16_1050_0002_1050_448c;
  PTR_LOOP_1050_48bc = (u8 *)0x446a;
  PTR_LOOP_1050_48be = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48c2 = (u8 *)0x446a;
  PTR_LOOP_1050_48c4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48c0 = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_48c6 = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_48c8 = (u8 *)0x446a;
  PTR_LOOP_1050_48ca = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48cc = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_48ce = (u8 *)0x447a;
  PTR_LOOP_1050_48d0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48d4 = (u8 *)0x447a;
  PTR_LOOP_1050_48d6 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48d2 = DAT_1050_4480;
  PTR_LOOP_1050_48d8 = DAT_1050_4480;
  PTR_LOOP_1050_48da = (u8 *)0x4436;
  PTR_LOOP_1050_48dc = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48de = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_48e0 = (u8 *)0x4436;
  PTR_LOOP_1050_48e2 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48e4 = PTR_u16_1050_0002_1050_443a;
  PTR_LOOP_1050_48e6 = (u8 *)0x447a;
  PTR_LOOP_1050_48e8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48ea = DAT_1050_4480;
  PTR_LOOP_1050_48ec = 0x0;
  PTR_LOOP_1050_48f2 = 0x0;
  PTR_LOOP_1050_48f8 = (u8 *)0x447a;
  PTR_LOOP_1050_48fa = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_48fc = DAT_1050_4480;
  PTR_LOOP_1050_48fe = (u8 *)0x447a;
  PTR_LOOP_1050_4900 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4902 = DAT_1050_4480;
  PTR_LOOP_1050_4904 = 0x0;
  PTR_LOOP_1050_490a = 0x0;
  PTR_LOOP_1050_485a = (u8 *)0xffff;
  PTR_LOOP_1050_48f0 = (u8 *)0xffff;
  PTR_LOOP_1050_48f6 = (u8 *)0xffff;
  PTR_LOOP_1050_4908 = (u8 *)0xffff;
  PTR_LOOP_1050_490e = (u8 *)0xffff;
  PTR_LOOP_1050_4910 = 0x0;
  PTR_LOOP_1050_4916 = 0x0;
  PTR_LOOP_1050_4854 = NULL;
  PTR_LOOP_1050_4878 = NULL;
  PTR_LOOP_1050_487e = NULL;
  PTR_LOOP_1050_48a8 = NULL;
  PTR_LOOP_1050_4914 = NULL;
  PTR_LOOP_1050_491a = NULL;
  PTR_LOOP_1050_491c = (u8 *)0x4488;
  PTR_LOOP_1050_491e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4920 = PTR_u16_1050_0002_1050_448c;
  PTR_LOOP_1050_4922 = (u8 *)0x4488;
  PTR_LOOP_1050_4924 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4926 = PTR_u16_1050_0002_1050_448c;
  PTR_LOOP_1050_4928 = 0x0;
  PTR_LOOP_1050_492e = 0x0;
  PTR_LOOP_1050_4934 = 0x0;
  PTR_LOOP_1050_493a = (u8 *)0x446a;
  PTR_LOOP_1050_493c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4940 = (u8 *)0x446a;
  PTR_LOOP_1050_4942 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_493e = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4944 = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4946 = 0x0;
  PTR_LOOP_1050_494c = 0x0;
  PTR_LOOP_1050_4952 = 0x0;
  PTR_LOOP_1050_4958 = 0x0;
  PTR_LOOP_1050_495e = 0x0;
  PTR_LOOP_1050_4964 = 0x0;
  PTR_LOOP_1050_496a = 0x0;
  PTR_LOOP_1050_4970 = 0x0;
  PTR_LOOP_1050_4976 = 0x0;
  PTR_LOOP_1050_497c = 0x0;
  PTR_LOOP_1050_4982 = 0x0;
  PTR_LOOP_1050_4988 = 0x0;
  PTR_LOOP_1050_498e = 0x0;
  PTR_LOOP_1050_4994 = 0x0;
  PTR_LOOP_1050_499a = (u8 *)0x4448;
  PTR_LOOP_1050_499c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49a0 = (u8 *)0x4448;
  PTR_LOOP_1050_49a2 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_499e = DAT_1050_4452;
  PTR_LOOP_1050_49a4 = DAT_1050_4452;
  PTR_LOOP_1050_49a6 = (u8 *)0x4448;
  PTR_LOOP_1050_49a8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49aa = DAT_1050_4452;
  PTR_LOOP_1050_49ac = (u8 *)0x4470;
  PTR_LOOP_1050_49ae = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49b2 = (u8 *)0x4470;
  PTR_LOOP_1050_49b4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49b0 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49b6 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49b8 = (u8 *)0x4470;
  PTR_LOOP_1050_49ba = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49bc = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49be = (u8 *)0x4470;
  PTR_LOOP_1050_49c0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49c2 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49c4 = (u8 *)0x4470;
  PTR_LOOP_1050_49c6 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49c8 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49ca = (u8 *)0x4448;
  PTR_LOOP_1050_49cc = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49ce = DAT_1050_4452;
  PTR_LOOP_1050_49d0 = (u8 *)0x4448;
  PTR_LOOP_1050_49d2 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49d4 = DAT_1050_4452;
  PTR_LOOP_1050_49d6 = (u8 *)0x4448;
  PTR_LOOP_1050_49d8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49da = DAT_1050_4452;
  PTR_LOOP_1050_49dc = (u8 *)0x4448;
  PTR_LOOP_1050_49de = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49e0 = DAT_1050_4452;
  PTR_LOOP_1050_49e2 = (u8 *)0x4482;
  PTR_LOOP_1050_49e4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49e8 = (u8 *)0x4482;
  PTR_LOOP_1050_49ea = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49e6 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_49ec = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_49ee = (u8 *)0x4470;
  PTR_LOOP_1050_49f0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49f2 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49f4 = (u8 *)0x4470;
  PTR_LOOP_1050_49f6 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49f8 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_49fa = (u8 *)0x4470;
  PTR_LOOP_1050_49fc = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_49fe = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_4a02 = NULL;
  PTR_LOOP_1050_4a00 = NULL;
  PTR_LOOP_1050_4a08 = NULL;
  PTR_LOOP_1050_4a06 = NULL;
  PTR_LOOP_1050_4a0e = NULL;
  PTR_LOOP_1050_4a0c = NULL;
  PTR_LOOP_1050_4a14 = NULL;
  PTR_LOOP_1050_4a12 = NULL;
  PTR_LOOP_1050_4a1a = NULL;
  PTR_LOOP_1050_4a18 = NULL;
  PTR_LOOP_1050_4a1e = (u8 *)0x4470;
  PTR_LOOP_1050_4a20 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4a22 = PTR_u32_1050_0004_1050_4478;
  PTR_LOOP_1050_4a24 = (u8 *)0x4448;
  PTR_LOOP_1050_4a26 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4a28 = DAT_1050_4452;
  PTR_LOOP_1050_4a2a = (u8 *)0x4448;
  PTR_LOOP_1050_4a2c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4a2e = DAT_1050_4452;
  PTR_LOOP_1050_4a30 = 0x0;
  PTR_LOOP_1050_4a36 = 0x0;
  PTR_LOOP_1050_492c = (u8 *)0xffff;
  PTR_LOOP_1050_4932 = (u8 *)0xffff;
  PTR_LOOP_1050_4938 = (u8 *)0xffff;
  PTR_LOOP_1050_494a = (u8 *)0xffff;
  PTR_LOOP_1050_4950 = (u8 *)0xffff;
  PTR_LOOP_1050_4a34 = (u8 *)0xffff;
  PTR_LOOP_1050_4a3a = (u8 *)0xffff;
  PTR_LOOP_1050_4a3c = 0x0;
  PTR_LOOP_1050_4a42 = 0x0;
  PTR_LOOP_1050_4956 = NULL;
  PTR_LOOP_1050_495c = NULL;
  PTR_LOOP_1050_4962 = NULL;
  PTR_LOOP_1050_4968 = NULL;
  PTR_LOOP_1050_496e = NULL;
  PTR_LOOP_1050_4974 = NULL;
  PTR_LOOP_1050_497a = NULL;
  PTR_LOOP_1050_4980 = NULL;
  PTR_LOOP_1050_4986 = NULL;
  PTR_LOOP_1050_498c = NULL;
  PTR_LOOP_1050_4992 = NULL;
  PTR_LOOP_1050_4998 = NULL;
  PTR_LOOP_1050_4a04 = NULL;
  PTR_LOOP_1050_4a0a = NULL;
  PTR_LOOP_1050_4a10 = NULL;
  PTR_LOOP_1050_4a16 = NULL;
  PTR_LOOP_1050_4a1c = NULL;
  PTR_LOOP_1050_4a40 = NULL;
  PTR_LOOP_1050_4a46 = NULL;
  PTR_LOOP_1050_4a48 = (u8 *)0x4448;
  PTR_LOOP_1050_4a4a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4a4c = DAT_1050_4452;
  PTR_LOOP_1050_4a4e = (u8 *)0x4448;
  PTR_LOOP_1050_4a50 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4a52 = DAT_1050_4452;
  PTR_LOOP_1050_4a54 = 0x0;
  PTR_LOOP_1050_4a5a = 0x0;
  PTR_LOOP_1050_4a60 = 0x0;
  PTR_LOOP_1050_4a66 = 0x0;
  PTR_LOOP_1050_4a6c = 0x0;
  PTR_LOOP_1050_4a72 = 0x0;
  PTR_LOOP_1050_4a78 = 0x0;
  PTR_LOOP_1050_4a7e = 0x0;
  PTR_LOOP_1050_4a84 = 0x0;
  PTR_LOOP_1050_4a8a = 0x0;
  PTR_LOOP_1050_4a90 = 0x0;
  PTR_LOOP_1050_4a96 = 0x0;
  PTR_LOOP_1050_4a9c = 0x0;
  PTR_LOOP_1050_4aa2 = 0x0;
  PTR_LOOP_1050_4aa8 = 0x0;
  PTR_LOOP_1050_4aae = 0x0;
  PTR_LOOP_1050_4ab4 = 0x0;
  PTR_LOOP_1050_4aba = 0x0;
  PTR_LOOP_1050_4ac0 = 0x0;
  PTR_LOOP_1050_4ac6 = 0x0;
  PTR_LOOP_1050_4acc = 0x0;
  PTR_LOOP_1050_4ad2 = 0x0;
  PTR_LOOP_1050_4ad8 = 0x0;
  PTR_LOOP_1050_4ade = 0x0;
  PTR_LOOP_1050_4ae4 = 0x0;
  PTR_LOOP_1050_4aea = 0x0;
  PTR_LOOP_1050_4af0 = 0x0;
  PTR_LOOP_1050_4a58 = NULL;
  PTR_LOOP_1050_4a5e = NULL;
  PTR_LOOP_1050_4a64 = NULL;
  PTR_LOOP_1050_4a6a = NULL;
  PTR_LOOP_1050_4a70 = NULL;
  PTR_LOOP_1050_4a76 = NULL;
  PTR_LOOP_1050_4a7c = NULL;
  PTR_LOOP_1050_4a82 = NULL;
  PTR_LOOP_1050_4a88 = NULL;
  PTR_LOOP_1050_4a8e = NULL;
  PTR_LOOP_1050_4a94 = NULL;
  PTR_LOOP_1050_4a9a = NULL;
  PTR_LOOP_1050_4aa0 = NULL;
  PTR_LOOP_1050_4aa6 = NULL;
  PTR_LOOP_1050_4aac = NULL;
  PTR_LOOP_1050_4ab2 = NULL;
  PTR_LOOP_1050_4ab8 = NULL;
  PTR_LOOP_1050_4abe = NULL;
  PTR_LOOP_1050_4ac4 = NULL;
  PTR_LOOP_1050_4aca = NULL;
  PTR_LOOP_1050_4ad0 = NULL;
  PTR_LOOP_1050_4ad6 = NULL;
  PTR_LOOP_1050_4adc = NULL;
  PTR_LOOP_1050_4ae2 = NULL;
  PTR_LOOP_1050_4ae8 = NULL;
  PTR_LOOP_1050_4aee = NULL;
  PTR_LOOP_1050_4af4 = NULL;
  puVar3 = (u16 *)0x4af6;
  for (iVar2 = 0x1b; iVar2 != 0x0; iVar2 += -0x1) {
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar1 = 0x0;
  }
  PTR_LOOP_1050_4b9c = PTR_u16_1050_0002_1050_4434;
  PTR_LOOP_1050_4b9e = 0x0;
  PTR_LOOP_1050_4ba4 = 0x0;
  PTR_LOOP_1050_4baa = 0x0;
  PTR_LOOP_1050_4ba2 = (u8 *)0xffff;
  PTR_LOOP_1050_4ba8 = (u8 *)0xffff;
  PTR_LOOP_1050_4bae = (u8 *)0xffff;
  PTR_LOOP_1050_4bb0 = 0x0;
  PTR_LOOP_1050_4bb6 = 0x0;
  PTR_LOOP_1050_4bbc = (u8 *)0x448e;
  PTR_LOOP_1050_4bbe = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bc2 = (u8 *)0x448e;
  PTR_LOOP_1050_4bc4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bc0 = DAT_1050_4494;
  PTR_LOOP_1050_4bc6 = DAT_1050_4494;
  PTR_LOOP_1050_4bc8 = (u8 *)0x448e;
  PTR_LOOP_1050_4bca = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bcc = DAT_1050_4494;
  PTR_LOOP_1050_4bce = (u8 *)0x4482;
  PTR_LOOP_1050_4bd0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bd4 = (u8 *)0x4482;
  PTR_LOOP_1050_4bd6 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bd2 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_4bd8 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_4bda = (u8 *)0x4482;
  PTR_LOOP_1050_4bdc = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bde = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_4be2 = NULL;
  PTR_LOOP_1050_4be0 = NULL;
  PTR_LOOP_1050_4bb4 = NULL;
  PTR_LOOP_1050_4bba = NULL;
  PTR_LOOP_1050_4be4 = NULL;
  PTR_LOOP_1050_4be6 = (u8 *)0x44ac;
  PTR_LOOP_1050_4be8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bec = (u8 *)0x44ac;
  PTR_LOOP_1050_4bee = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bea = DAT_1050_44b2;
  PTR_LOOP_1050_4bf0 = DAT_1050_44b2;
  PTR_LOOP_1050_4bf2 = (u8 *)0x44ac;
  PTR_LOOP_1050_4bf4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bf6 = DAT_1050_44b2;
  PTR_LOOP_1050_4bf8 = (u8 *)0x446a;
  PTR_LOOP_1050_4bfa = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bfe = (u8 *)0x446a;
  PTR_LOOP_1050_4c00 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4bfc = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4c02 = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4c04 = (u8 *)0x446a;
  PTR_LOOP_1050_4c06 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c08 = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4c0a = (u8 *)0x448e;
  PTR_LOOP_1050_4c0c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c0e = DAT_1050_4494;
  PTR_LOOP_1050_4c10 = (u8 *)0x448e;
  PTR_LOOP_1050_4c12 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c14 = DAT_1050_4494;
  PTR_LOOP_1050_4c16 = (u8 *)0x44ac;
  PTR_LOOP_1050_4c18 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c1a = DAT_1050_44b2;
  PTR_LOOP_1050_4c22 = (u8 *)0x448e;
  PTR_LOOP_1050_4c24 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c26 = DAT_1050_4494;
  PTR_LOOP_1050_4c28 = 0x0;
  PTR_LOOP_1050_4c2e = 0x0;
  PTR_LOOP_1050_4c34 = 0x0;
  PTR_LOOP_1050_4c3a = 0x0;
  PTR_LOOP_1050_4c40 = 0x0;
  PTR_LOOP_1050_4c46 = 0x0;
  _u16_1050_4c4c = 0x0;
  PTR_LOOP_1050_4c52 = 0x0;
  PTR_LOOP_1050_4c1c = (u8 *)0x44ac;
  PTR_LOOP_1050_4c1e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c58 = (u8 *)0x44ac;
  PTR_LOOP_1050_4c5a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c20 = DAT_1050_44b2;
  PTR_LOOP_1050_4c5c = DAT_1050_44b2;
  PTR_LOOP_1050_4c5e = (u8 *)0x44ac;
  PTR_LOOP_1050_4c60 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c62 = DAT_1050_44b2;
  PTR_LOOP_1050_4c64 = 0x0;
  PTR_LOOP_1050_4c6a = 0x0;
  PTR_LOOP_1050_4c70 = 0x0;
  PTR_LOOP_1050_4c76 = (u8 *)0x446a;
  PTR_LOOP_1050_4c78 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c7c = (u8 *)0x446a;
  PTR_LOOP_1050_4c7e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4c7a = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4c80 = PTR_u16_1050_0002_1050_446e;
  PTR_LOOP_1050_4c82 = 0x0;
  PTR_LOOP_1050_4c88 = 0x0;
  PTR_LOOP_1050_4c2c = (u8 *)0xffff;
  PTR_LOOP_1050_4c32 = (u8 *)0xffff;
  PTR_LOOP_1050_4c38 = (u8 *)0xffff;
  PTR_LOOP_1050_4c3e = (u8 *)0xffff;
  PTR_LOOP_1050_4c44 = (u8 *)0xffff;
  PTR_LOOP_1050_4c4a = (u8 *)0xffff;
  PTR_LOOP_1050_4c68 = (u8 *)0xffff;
  PTR_LOOP_1050_4c6e = (u8 *)0xffff;
  PTR_LOOP_1050_4c74 = (u8 *)0xffff;
  PTR_LOOP_1050_4c86 = (u8 *)0xffff;
  PTR_LOOP_1050_4c8c = (u8 *)0xffff;
  PTR_LOOP_1050_4c8e = 0x0;
  PTR_LOOP_1050_4c94 = 0x0;
  PTR_LOOP_1050_4c9a = 0x0;
  PTR_LOOP_1050_4ca0 = 0x0;
  PTR_LOOP_1050_4ca6 = 0x0;
  PTR_LOOP_1050_4cac = 0x0;
  PTR_LOOP_1050_4cb2 = 0x0;
  PTR_LOOP_1050_4cb8 = 0x0;
  PTR_LOOP_1050_4cbe = 0x0;
  PTR_LOOP_1050_4cc4 = 0x0;
  PTR_LOOP_1050_4cca = 0x0;
  PTR_LOOP_1050_4cd0 = 0x0;
  PTR_LOOP_1050_4cd6 = (u8 *)0x4496;
  PTR_LOOP_1050_4cd8 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4cdc = (u8 *)0x4496;
  PTR_LOOP_1050_4cde = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4cda = DAT_1050_44a2;
  PTR_LOOP_1050_4ce0 = DAT_1050_44a2;
  PTR_LOOP_1050_4ce2 = (u8 *)0x4496;
  PTR_LOOP_1050_4ce4 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4ce6 = DAT_1050_44a2;
  PTR_LOOP_1050_4ce8 = (u8 *)0x4496;
  PTR_LOOP_1050_4cea = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4cec = DAT_1050_44a2;
  PTR_LOOP_1050_4cee = (u8 *)0x4496;
  PTR_LOOP_1050_4cf0 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4cf2 = DAT_1050_44a2;
  PTR_LOOP_1050_4cf4 = (u8 *)0x44a4;
  PTR_LOOP_1050_4cf6 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4cfa = (u8 *)0x44a4;
  PTR_LOOP_1050_4cfc = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4cf8 = DAT_1050_44aa;
  PTR_LOOP_1050_4cfe = DAT_1050_44aa;
  PTR_LOOP_1050_4d00 = (u8 *)0x44a4;
  PTR_LOOP_1050_4d02 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d04 = DAT_1050_44aa;
  PTR_LOOP_1050_4d06 = (u8 *)0x4496;
  PTR_LOOP_1050_4d08 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d0a = DAT_1050_44a2;
  PTR_LOOP_1050_4d0c = (u8 *)0x4496;
  PTR_LOOP_1050_4d0e = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d10 = DAT_1050_44a2;
  PTR_LOOP_1050_4d12 = (u8 *)0x4496;
  PTR_LOOP_1050_4d14 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d16 = DAT_1050_44a2;
  PTR_LOOP_1050_4d18 = (u8 *)0x4496;
  PTR_LOOP_1050_4d1a = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d1c = DAT_1050_44a2;
  PTR_LOOP_1050_4d1e = (u8 *)0x4482;
  PTR_LOOP_1050_4d20 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d24 = (u8 *)0x4482;
  PTR_LOOP_1050_4d26 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d22 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_4d28 = PTR_u16_1050_0002_1050_4486;
  PTR_LOOP_1050_4d2a = (u8 *)0x44a4;
  PTR_LOOP_1050_4d2c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d2e = DAT_1050_44aa;
  PTR_LOOP_1050_4d30 = (u8 *)0x44a4;
  PTR_LOOP_1050_4d32 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d34 = DAT_1050_44aa;
  PTR_LOOP_1050_4d36 = (u8 *)0x44a4;
  PTR_LOOP_1050_4d38 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d3a = DAT_1050_44aa;
  PTR_LOOP_1050_4d3c = 0x0;
  PTR_LOOP_1050_4d42 = 0x0;
  PTR_LOOP_1050_4c50 = NULL;
  PTR_LOOP_1050_4c56 = NULL;
  PTR_LOOP_1050_4c92 = NULL;
  PTR_LOOP_1050_4c98 = NULL;
  PTR_LOOP_1050_4c9e = NULL;
  PTR_LOOP_1050_4ca4 = NULL;
  PTR_LOOP_1050_4caa = NULL;
  PTR_LOOP_1050_4cb0 = NULL;
  PTR_LOOP_1050_4cb6 = NULL;
  PTR_LOOP_1050_4cbc = NULL;
  PTR_LOOP_1050_4cc2 = NULL;
  PTR_LOOP_1050_4cc8 = NULL;
  PTR_LOOP_1050_4cce = NULL;
  PTR_LOOP_1050_4cd4 = NULL;
  PTR_LOOP_1050_4d40 = NULL;
  PTR_LOOP_1050_4d46 = NULL;
  PTR_LOOP_1050_4d48 = 0x0;
  PTR_LOOP_1050_4d4e = 0x0;
  PTR_LOOP_1050_4d54 = 0x0;
  PTR_LOOP_1050_4d5a = (u8 *)0x44a4;
  PTR_LOOP_1050_4d5c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d5e = DAT_1050_44aa;
  PTR_LOOP_1050_4d60 = (u8 *)0x4496;
  PTR_LOOP_1050_4d62 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d66 = (u8 *)0x4496;
  PTR_LOOP_1050_4d68 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d64 = DAT_1050_44a2;
  PTR_LOOP_1050_4d6a = DAT_1050_44a2;
  PTR_LOOP_1050_4d6c = 0x0;
  PTR_LOOP_1050_4d72 = 0x0;
  PTR_LOOP_1050_4d70 = (u8 *)0xffff;
  PTR_LOOP_1050_4d76 = (u8 *)0xffff;
  PTR_LOOP_1050_4d78 = 0x0;
  PTR_LOOP_1050_4d7e = 0x0;
  PTR_LOOP_1050_4d84 = (u8 *)0x4496;
  PTR_LOOP_1050_4d86 = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d88 = DAT_1050_44a2;
  PTR_LOOP_1050_4d8a = (u8 *)0x4496;
  PTR_LOOP_1050_4d8c = (u8 *)&DAT_1050_1050;
  PTR_LOOP_1050_4d8e = DAT_1050_44a2;
  PTR_LOOP_1050_4d90 = 0x0;
  PTR_LOOP_1050_4d96 = 0x0;
  PTR_LOOP_1050_4d9c = 0x0;
  PTR_LOOP_1050_4da2 = 0x0;
  PTR_LOOP_1050_4da8 = 0x0;
  PTR_LOOP_1050_4dae = 0x0;
  PTR_LOOP_1050_4db4 = 0x0;
  PTR_LOOP_1050_4dba = 0x0;
  PTR_LOOP_1050_4dc0 = 0x0;
  PTR_LOOP_1050_4dc6 = 0x0;
  PTR_LOOP_1050_4dcc = 0x0;
  PTR_LOOP_1050_4dd2 = 0x0;
  PTR_LOOP_1050_4dd8 = 0x0;
  PTR_LOOP_1050_4dde = 0x0;
  PTR_LOOP_1050_4de4 = 0x0;
  PTR_LOOP_1050_4dea = 0x0;
  PTR_LOOP_1050_4df0 = 0x0;
  PTR_LOOP_1050_4df6 = 0x0;
  PTR_LOOP_1050_4dfc = 0x0;
  PTR_LOOP_1050_4e02 = 0x0;
  PTR_LOOP_1050_4e08 = 0x0;
  PTR_LOOP_1050_4e0e = 0x0;
  PTR_LOOP_1050_4e14 = 0x0;
  PTR_LOOP_1050_4e1a = 0x0;
  PTR_LOOP_1050_4e20 = 0x0;
  PTR_LOOP_1050_4e26 = 0x0;
  PTR_LOOP_1050_4e2c = 0x0;
  PTR_LOOP_1050_4d4c = NULL;
  PTR_LOOP_1050_4d52 = NULL;
  PTR_LOOP_1050_4d58 = NULL;
  PTR_LOOP_1050_4d7c = NULL;
  PTR_LOOP_1050_4d82 = NULL;
  PTR_LOOP_1050_4d94 = NULL;
  PTR_LOOP_1050_4d9a = NULL;
  PTR_LOOP_1050_4da0 = NULL;
  PTR_LOOP_1050_4da6 = NULL;
  PTR_LOOP_1050_4dac = NULL;
  PTR_LOOP_1050_4db2 = NULL;
  PTR_LOOP_1050_4db8 = NULL;
  PTR_LOOP_1050_4dbe = NULL;
  PTR_LOOP_1050_4dc4 = NULL;
  PTR_LOOP_1050_4dca = NULL;
  PTR_LOOP_1050_4dd0 = NULL;
  PTR_LOOP_1050_4dd6 = NULL;
  PTR_LOOP_1050_4ddc = NULL;
  PTR_LOOP_1050_4de2 = NULL;
  PTR_LOOP_1050_4de8 = NULL;
  PTR_LOOP_1050_4dee = NULL;
  PTR_LOOP_1050_4df4 = NULL;
  PTR_LOOP_1050_4dfa = NULL;
  PTR_LOOP_1050_4e00 = NULL;
  PTR_LOOP_1050_4e06 = NULL;
  PTR_LOOP_1050_4e0c = NULL;
  PTR_LOOP_1050_4e12 = NULL;
  PTR_LOOP_1050_4e18 = NULL;
  PTR_LOOP_1050_4e1e = NULL;
  PTR_LOOP_1050_4e24 = NULL;
  PTR_LOOP_1050_4e2a = NULL;
  PTR_LOOP_1050_4e30 = NULL;
  puVar3 = (u16 *)0x4e32;
  for (iVar2 = 0x1b; iVar2 != 0x0; iVar2 += -0x1) {
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar1 = 0x0;
  }
  return;
}


u16 pass1_1020_a426(void)

{
  u16 *puVar1;

  pass1_1008_3e38((astruct_19 *)&PTR_1048_4230);
  puVar1 = pass1_1008_3e38((astruct_19 *)0x10484236);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 * pass1_1020_a43e(u8 *param_1,u16 *param_2)

{
  u16 in_register_0000000a;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  u32 in_stack_0000fff2;

  *param_2 = 0xba36;
  ((int)param_2 + 0x2) = 0x1020;
  if (_PTR_LOOP_1050_4e74 != 0x0) {
    return param_2;
  }
  mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                  (u8 **)CONCAT22((int)((u32)in_stack_0000fff2 >> 0x10),0x2),in_stack_0000fe9c,
                  in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  if ((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {
    if (PTR_LOOP_1050_13ae == (u8 *)&u16_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      PTR_LOOP_1050_4e74 = (u8 *)0x44b4;
      goto LAB_1020_a482;
    }
    if (PTR_LOOP_1050_13ae == (u8 *)&u32_1050_0004) {
      PTR_LOOP_1050_4e74 = (u8 *)0x4b2c;
      goto LAB_1020_a482;
    }
  }
  PTR_LOOP_1050_4e74 = (u8 *)0x47f0;
LAB_1020_a482:
  PTR_LOOP_1050_4e74 = CONCAT22(0x1050,PTR_LOOP_1050_4e74);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_a49a(u16 param_1,u16 param_2,u32 param_3,i16 *param_4,u16 param_5)

{
  u32 uVar1;
  u16 in_register_0000000a;
  u16 uVar2;
  u16 in_stack_0000fd62;
  u16 in_stack_0000fe86;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000fe90;
  i16 iVar3;
  u8 local_136 [0x128];
  u16 uStack14;
  u16 uStack12;
  u32 uStack10;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(param_2,0x2f),in_stack_0000fd62,in_stack_0000fe86,in_stack_0000fe8c,
                             in_stack_0000fe90);
  uStack12 = ((u32)puStack6 >> 0x10);
  uVar1 = (u32)((int)puStack6 + 0x20);
  uStack10 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  uStack14 = uVar1;
  if (param_4 != NULL) {
    uVar2 = ((u32)param_4 >> 0x10);
    if (((u32 *)param_4 + 0x1) == 0x0) {
      iVar3 = (int)&PTR_LOOP_1050_4230;
    }
    else {
      iVar3 = (int)s_dib_1050_4234 + 0x2;
    }
    pass1_1008_3f32(param_4,(i16 *)CONCAT22(0x1048,iVar3));
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_136),0x0,0x0,param_5,(u32 *)param_4,uVar2,
                        (u32)((int)_PTR_LOOP_1050_4e70 + 0x4),uStack10);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_136));
    return;
  }
  pass1_1020_abc0(param_3,param_5,uVar1 & 0xffff | (u32)uStack12 << 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_a54c(u8 *param_1,u16 param_2,u16 param_3,i16 param_4)

{
  u32 uVar1;
  u32 uVar2;
  u16 in_register_0000000a;
  u16 unaff_SI;
  u16 in_stack_0000fd58;
  u16 in_stack_0000fe7c;
  u16 in_stack_0000fe82;
  u16 in_stack_0000fe86;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u8 local_140 [0x124];
  astruct_19 **ppaStack28;
  i16 local_18;
  u16 local_16;
  astruct_19 *local_14;
  u8 *puStack16;
  u16 uStack14;
  u16 uStack12;
  u32 uStack10;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd58,in_stack_0000fe7c,in_stack_0000fe82
                             ,in_stack_0000fe86);
  uStack12 = ((u32)puStack6 >> 0x10);
  uVar2 = (u32)((int)puStack6 + 0x20);
  uStack10 = uVar2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uStack14 = uVar2;
  local_14 = PTR_1048_4230;
  puStack16 = PTR_LOOP_1048_4234;
  ppaStack28 = &local_14;
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_18),(char *)CONCAT22(0x1050,&local_16)
                 );
  if ((param_4 < 0x0) || (0x5 < param_4)) {
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,local_18 - 0x9,local_16);
    uVar6 = uStack10;
    uVar7 = (uStack10 >> 0x10);
    uVar1 = (u32)((int)_PTR_LOOP_1050_4e70 + 0x4);
    uVar4 = uVar1;
    uVar5 = ((u32)uVar1 >> 0x10);
    uVar3 = 0x14;
  }
  else {
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,(local_18 - param_4) - 0x3,local_16);
    uVar6 = uStack10;
    uVar7 = (uStack10 >> 0x10);
    uVar1 = (u32)((int)_PTR_LOOP_1050_4e70 + 0x4);
    uVar4 = uVar1;
    uVar5 = ((u32)uVar1 >> 0x10);
    uVar3 = 0x7b;
  }
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_140),0x0,0x0,uVar3,(u32 *)&local_14,&DAT_1050_1050,
                      CONCAT22(uVar5,uVar4),CONCAT22(uVar7,uVar6));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_140));
  return;
}



BOOL16 pass1_1020_a644(u16 param_1,u16 param_2,u32 param_3)

{
  BOOL16 BVar1;

  BVar1 = write_to_file_1008_7cac(param_3);
  if (BVar1 != 0x0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 read_file_1020_a65e(u16 param_1,u32 param_2,HFILE16 *param_3)

{
  BOOL16 BVar1;
  u16 in_DX;
  u8 local_a [0x2];
  u8 local_8 [0x2];
  u8 local_6 [0x2];
  u8 local_4 [0x2];
  u16 uVar3;
  u16 uVar2;

  read_file_1008_7cfe((int)param_3,(int)((u32)param_3 >> 0x10),0xb);
  if (param_1 != 0x0) {
    if (0x1 < (int)u16_1050_0312) {
LAB_1020_a6dc:
      pass1_1020_b97e(param_1,in_DX,param_2,((u32)param_2 >> 0x10),0x0);
      return 0x1;
    }
    BVar1 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_4),0x2);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_8),0x2);
      if (BVar1 != 0x0) {
        BVar1 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_6),0x2);
        if (BVar1 != 0x0) {
          param_1 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,local_a),0x2);
          if (param_1 != 0x0) goto LAB_1020_a6dc;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_a6ee(u16 param_1,u16 param_2,u16 param_3,u32 param_4,u16 param_5)

{
  u32 uVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u16 *puVar3;
  u16 in_stack_0000fd48;
  u16 in_stack_0000fe6c;
  u16 in_stack_0000fe72;
  u16 in_stack_0000fe76;
  u16 uVar4;
  u8 local_13e [0x120];
  u32 uStack30;
  BOOL16 BStack26;
  u32 local_18;
  u16 uStack20;
  i16 iStack18;
  u16 uStack16;
  u32 uStack14;
  u32 *puStack10;
  u32 uStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
  uStack6 = CONCAT22(param_2,param_1);
  if (((param_2 | param_1) == 0x0) || (*(i32 *)(param_1 + 0x200) == 0x8000002)) {
    puStack10 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1),_u16_1050_0ed0,
                                (u8 **)CONCAT22(param_3,0x2f),in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
    uStack16 = ((u32)puStack10 >> 0x10);
    uVar1 = (u32)((int)puStack10 + 0x20);
    uStack14 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
    iStack18 = (int)uVar1;
    puVar3 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_18));
    uVar2 = ((u32)puVar3 >> 0x10);
    BStack26 = pass1_1008_c6ae(_u16_1050_06e0,param_5,0x28);
    if (BStack26 != 0x0) {
      uStack20 = 0x1;
    }
    uVar4 = (param_4 >> 0x10);
    pass1_1020_b2da(param_4,uVar4,(BStack26 != 0x0),(u16 *)CONCAT22(0x1050,&local_18),
                    CONCAT22(uStack16,iStack18));
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_13e),0x0,0x0,param_5,&local_18,&DAT_1050_1050,
                        (u32)((int)_PTR_LOOP_1050_4e70 + 0x4),(u32)(iStack18 + 0x4));
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13e));
    if (BStack26 != 0x0) {
      pass1_1020_ad90(uVar2,param_4,uVar4,(u16 *)CONCAT22(0x1050,&local_18),(u32)(iStack18 + 0x4));
    }
    (u32)((int)uStack30 + 0x1c) = 0x8000001;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_a80e(u16 param_1,u16 param_2,u16 param_3,u16 param_4,i16 param_5)

{
  u16 uVar1;
  u32 uVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  u32 *puVar4;
  u16 in_stack_0000fe8c;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffe4;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
  if (((param_2 | param_1) == 0x0) || (*(i32 *)(param_1 + 0x200) == 0x8000002)) {
    puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000ffe4,0x2f),in_stack_0000fe8c,in_stack_0000ffb0,
                             in_stack_0000ffb6,in_stack_0000ffba);
    uVar3 = ((u32)puVar4 >> 0x10);
    uVar2 = (u32)((int)puVar4 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
    uVar1 = uVar2;
    if (param_5 == 0xa) {
      pass1_1020_b872(CONCAT22(param_4,param_3),uVar2 & 0xffff | (u32)uVar3 << 0x10);
      return;
    }
    pass1_1020_b0aa(uVar3,param_3,param_4,param_5);
    if (uVar1 != 0x0) {
      pass1_1020_abc0(CONCAT22(param_4,param_3),uVar1,uVar2 & 0xffff | (u32)uVar3 << 0x10);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_a89e(u32 param_1,u32 *param_2,u16 param_3)

{
  i16 *piVar1;
  u8 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u32 uVar5;
  u16 uVar6;
  astruct_57 *in_EDX;
  u32 uVar7;
  astruct_57 *paVar8;
  u16 unaff_SI;
  u16 in_stack_0000f892;
  u16 in_stack_0000f9b6;
  u16 in_stack_0000f9bc;
  u16 in_stack_0000f9c0;
  u16 uVar9;
  u16 uVar10;
  u16 local_5ee;
  u16 uStack1516;
  u32 *puStack1218;
  i16 iStack1214;
  u32 uStack1212;
  u8 local_4b8 [0x8];
  u32 uStack1200;
  u32 *puStack1196;
  u8 local_4a8 [0x124];
  u8 local_384 [0x124];
  u8 local_260 [0x124];
  u8 local_13c [0x124];
  u16 local_18;
  u16 local_16;
  u32 local_14;
  u16 uStack16;
  u32 uStack14;
  u32 uStack10;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000f892,
                             in_stack_0000f9b6,in_stack_0000f9bc,in_stack_0000f9c0);
  uVar7 = (u32)in_EDX & 0xffff0000 | (u32)puStack6 >> 0x10;
  uVar5 = (u32)((int)puStack6 + 0x20);
  uStack10 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5);
  uStack14 = uVar5 & 0xffff | uVar7 << 0x10;
  local_14 = *param_2;
  uStack16 = (param_2 + 0x1);
  puStack1218 = &local_14;
  paVar8 = (astruct_57 *)(uVar7 & 0xffff0000 | ZEXT24(&local_14));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_18),(char *)CONCAT22(0x1050,&local_16)
                 );
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,local_18,local_16 + 0x2);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_13c),0x0,0x7a,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13c));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,local_18 - 0x2,local_16);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_260),0x0,0x47,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_260));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x1,local_18 - 0x2,local_16);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_384),0x0,0x6a,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_384));
  uVar9 = param_1;
  uVar10 = (param_1 >> 0x10);
  pass1_1020_ad90(paVar8,uVar9,uVar10,(u16 *)CONCAT22(0x1050,&local_14),uStack10);
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x1,local_18 - 0x2,local_16 + 0x1);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_4a8),0x0,0x7f,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_4a8));
  pass1_1020_ad90(paVar8,uVar9,uVar10,(u16 *)CONCAT22(0x1050,&local_14),uStack10);
  puStack1196 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000f892,
                                in_stack_0000f9b6,in_stack_0000f9bc,in_stack_0000f9c0);
  uVar7 = (u32)puStack1196 >> 0x10;
  uStack1200 = (u32)((int)puStack1196 + 0x12);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_4b8),uStack1200);
  iStack1214 = 0x0;
  do {
    do {
      uVar6 = uVar7;
      puVar2 = local_4b8;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
      uStack1212 = CONCAT22(uVar6,puVar2);
      uVar7 = (u32)(uVar6 | puVar2);
      if ((uVar6 | puVar2) == 0x0) {
        pass1_1010_9674((u32)puStack1196);
        return;
      }
    } while (((puVar2 + 0x4) != 0x3e) && ((puVar2 + 0x4) != 0x41));
    while (0x0 < ((int)uStack1212 + 0x6)) {
      if (iStack1214 == 0x0) {
        uVar4 = local_16 - 0x2;
LAB_1020_ab4a:
        uVar3 = local_18 - 0x2;
LAB_1020_ab51:
        iStack1214 = iStack1214 + 0x1;
        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,uVar3,uVar4);
      }
      else {
        if (iStack1214 == 0x1) {
          uVar4 = local_16 + 0x2;
          goto LAB_1020_ab4a;
        }
        if (iStack1214 == 0x2) {
          uVar4 = local_16 + 0x2;
LAB_1020_ab6e:
          uVar3 = local_18 + 0x2;
          goto LAB_1020_ab51;
        }
        if (iStack1214 == 0x3) {
          uVar4 = local_16 - 0x2;
          goto LAB_1020_ab6e;
        }
        iStack1214 = iStack1214 + 0x1;
        pass1_1020_b2da(uVar9,uVar10,0x0,(u16 *)CONCAT22(0x1050,&local_14),uStack14);
      }
      struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,&local_5ee),0x0,((int)uStack1212 + 0x4),&local_14,
                          &DAT_1050_1050,0x8000002,0x4000002,uStack10);
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_5ee));
      piVar1 = (i16 *)((int)uStack1212 + 0x6);
      *piVar1 = *piVar1 + -0x1;
      local_5ee = 0x389a;
      uStack1516 = 0x1008;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_abc0(u32 param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 uVar2;
  u16 *puVar3;
  u16 uVar4;
  u8 local_12e [0x124];
  BOOL16 BStack10;
  u32 local_8;
  u16 uStack4;

  puVar3 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_8));
  uVar1 = ((u32)puVar3 >> 0x10);
  BStack10 = pass1_1008_c6ae(_u16_1050_06e0,param_2,0x28);
  if (BStack10 != 0x0) {
    uStack4 = 0x1;
  }
  uVar4 = (param_1 >> 0x10);
  pass1_1020_b2da(param_1,uVar4,(BStack10 != 0x0),(u16 *)CONCAT22(0x1050,&local_8),param_3);
  uVar2 = (param_3 >> 0x10);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_12e),0x0,0x0,param_2,&local_8,&DAT_1050_1050,
                      (u32)((int)_PTR_LOOP_1050_4e70 + 0x4),(u32)((int)param_3 + 0x4));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_12e));
  if (BStack10 != 0x0) {
    pass1_1020_ad90(uVar1,param_1,uVar4,(u16 *)CONCAT22(0x1050,&local_8),(u32)((int)param_3 + 0x4));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ac6e(u16 param_1,u32 param_2,i16 param_3,i16 param_4,i16 param_5)

{
  u16 uVar1;
  u32 *puVar2;
  u32 uVar3;
  u16 uVar4;
  u32 in_EDX;
  u16 uVar5;
  u16 *puVar6;
  u16 in_stack_0000fd56;
  u16 in_stack_0000fe7a;
  u16 in_stack_0000fe80;
  u16 in_stack_0000fe84;
  i16 iVar7;
  u8 local_146 [0x12c];
  i16 iStack26;
  u16 uStack24;
  u32 uStack22;
  u32 *puStack18;
  u32 local_e;
  u16 local_8;
  u32 local_6;

  uVar5 = ((u32)in_EDX >> 0x10);
  if (param_3 == 0x0) {
    iVar7 = (int)&PTR_LOOP_1050_4230;
  }
  else {
    iVar7 = (int)s_dib_1050_4234 + 0x2;
  }
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1048,iVar7),(u16 *)CONCAT22(0x1050,&local_8),
                  (u16 *)CONCAT22(0x1050,&local_6),(u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  if (param_5 == 0x0) {
    local_6 = local_6 & 0xffff | (u32)(local_6 + param_4) << 0x10;
  }
  else if (param_5 == 0x1) {
    local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 + param_4);
  }
  else if (param_5 == 0x2) {
    local_6 = local_6 & 0xffff | (u32)(local_6 - param_4) << 0x10;
  }
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_e),local_8,local_6,(local_6 >> 0x10));
  puStack18 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar5,(int)((u32)puVar6 >> 0x10)),_u16_1050_0ed0,
                              (u8 **)CONCAT22(param_1,0x2f),in_stack_0000fd56,in_stack_0000fe7a,in_stack_0000fe80
                              ,in_stack_0000fe84);
  uVar4 = ((u32)puStack18 >> 0x10);
  uVar3 = (u32)((int)puStack18 + 0x20);
  uStack22 = uVar3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  iStack26 = (int)uVar3;
  uStack24 = uVar4;
  uVar1 = pass1_1020_b1ae((int)&local_e,uVar4,param_2,(param_2 >> 0x10),
                          (u16 *)CONCAT22(0x1050,&local_e),(u32)(iStack26 + 0x4));
  if (uVar1 != 0x0) {
    puVar2 = &local_e;
    pass1_1020_b240(uVar4,param_2,CONCAT22(0x1050,puVar2),CONCAT22(uStack24,iStack26));
    if (puVar2 != NULL) {
      struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_146),0x0,0x0,(-(param_3 == 0x0) & 0xfffb) + 0x7f,
                          &local_e,&DAT_1050_1050,(u32)((int)_PTR_LOOP_1050_4e70 + 0x4),uStack22);
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_146));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ad90(u16 param_1,u16 param_2,u16 param_3,u16 *param_4,u32 param_5)

{
  code **ppcVar1;
  u16 *puVar2;
  u8 *puVar3;
  i16 iVar4;
  u32 uVar5;
  u16 uVar6;
  u16 DX_REG;
  u32 *puVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 local_17e;
  u16 uStack380;
  i16 iStack90;
  u32 *puStack78;
  u16 uStack70;
  i16 iStack68;
  u32 uStack66;
  u32 *puStack62;
  u8 local_3a [0xc];
  u32 local_2e;
  u16 uStack42;
  i16 iStack40;
  u16 uStack38;
  i16 local_24;
  i16 local_22;
  u32 uStack32;
  u32 uStack28;
  u32 uStack24;
  u16 *puStack20;
  u16 uStack18;
  i16 iStack16;
  i16 iStack14;
  u32 uStack12;
  u16 local_8;
  i16 local_6;
  i16 local_4;

  puVar2 = &local_8;
  pass1_1008_3eb4((astruct_615 *)param_4,(u16 *)CONCAT22(0x1050,puVar2),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,&local_4));
  pass1_1030_627e(puVar2,param_1,(u32)_PTR_LOOP_1050_5740,param_4,param_5);
  puStack20 = puVar2;
  uStack18 = param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar2));
  uStack24 = CONCAT22(param_1,puVar2);
  uStack28 = (u32)(puVar2 + 0x17);
  uVar5 = (u32)((int)uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  iStack40 = (int)uVar5;
  uStack38 = param_1;
  puVar7 = (u32 *)pass1_1030_5b5c(iStack40,param_1);
  uVar6 = ((u32)puVar7 >> 0x10);
  local_2e = *puVar7;
  uStack42 = ((int)puVar7 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_2e),(u16 *)CONCAT22(0x1050,&local_24),(char *)CONCAT22(0x1050,&local_22)
                 );
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1U);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (u32)(local_6 - 0x1U);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if ((int)uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_3a));
  pass1_1008_6cec((u16 *)CONCAT22(0x1050,local_3a),local_8,CONCAT22(iStack14,iStack16),local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),param_5);
  puStack62 = (u32 *)CONCAT22(uVar6,puVar3);
  if ((uVar6 | puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = uStack12; (int)uStack70 <= iStack16; uStack70 += 0x1) {
      for (puStack78 = (u32 *)uStack12; (int)puStack78 <= iStack14;
          puStack78 = (u32 *)((int)puStack78 + 0x1)) {
        ppcVar1 = (code **)((int)*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(0x1030,(int)puStack62,(int)((u32)puStack62 >> 0x10));
        uStack66 = CONCAT22(DX_REG,iVar4);
        uStack66._3_1_ = (char)(DX_REG >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = ((u32)uStack32 >> 0x10);
            uVar8 = 0x6;
          }
          else if (iVar4 == 0x8) {
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = ((u32)uStack32 >> 0x10);
            uVar8 = 0x7;
          }
          else {
            if (iVar4 != 0x9) goto LAB_1020_af1c;
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = ((u32)uStack32 >> 0x10);
            uVar8 = 0x8;
          }
          struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_17e),0x0,0x0,uVar8,(u32 *)param_4,
                              ((u32)param_4 >> 0x10),CONCAT22(uVar10,uVar9),param_5);
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }
LAB_1020_af1c:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_afc4(u16 param_1,u16 param_2,u16 param_3,u16 *param_4,i32 param_5)

{
  u32 *puVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;
  u8 bStack27;
  u32 local_a;
  u32 uStack6;

  puVar1 = &local_a;
  pass1_1030_64ce(puVar1,param_1,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,puVar1));
  uStack6 = *puVar1;
  uVar3 = ((int)puVar1 + 0x2);
  bStack27 = (u8)(uStack6 >> 0x18);
  uVar2 = bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar3 << 0x10);
  uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3);
  uVar3 = (uVar4 >> 0x10);
  if ((uVar3 | uVar4) != 0x0) {
    switch((uVar4 + 0xc)) {
    case 0x1:
      break;
    case 0x2:
      break;
    case 0x3:
      break;
    case 0x4:
      break;
    case 0x5:
      break;
    case 0x6:
      break;
    case 0x7:
      return;
    case 0x8:
      return;
    case 0x9:
      return;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_b0aa(u16 param_1,u16 param_2,u16 param_3,i16 param_4)

{
  u32 *puVar1;
  code **ppcVar2;
  i16 iVar3;
  u32 *puVar4;
  u16 DX_REG;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u32 uVar8;
  u32 uStack20;

  uVar7 = ((u32)_PTR_LOOP_1050_4e74 >> 0x10);
  if (((int)_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) == 0x0) {
    return;
  }
  if (((int)_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) != -0x1) {
    if (PTR_LOOP_1050_4e78 == NULL) {
      iVar3 = param_4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
      puVar1 = (u32 *)(u32)(iVar3 + 0xc);
      ppcVar2 = (code **)((int)*puVar1 + 0x10);
      puVar4 = puVar1;
      (**ppcVar2)();
      uVar6 = DX_REG;
      for (uStack20 = 0x0; uStack20 < ((u32)puVar4 & 0xffff | (u32)DX_REG << 0x10); uStack20 += 0x1) {
        uVar8 = pass1_1030_1d7c((int)((u32)puVar4 & 0xffff),uVar6,(u32)puVar1);
        uVar5 = (uVar8 >> 0x10);
        uVar6 = uVar5 | uVar8;
        if ((uVar6 != 0x0) && ((iVar3 = (uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b)))) {
          PTR_LOOP_1050_4e78 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
          break;
        }
      }
      if (PTR_LOOP_1050_4e78 == NULL) {
        PTR_LOOP_1050_4e78 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
        return;
      }
    }
    uVar6 = ((int)_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) - 0x1;
    pass1_1008_612e(uVar6,0x0,uVar6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1020_b1ae(i16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 *param_5,u32 param_6)

{
  u32 *puVar1;
  i16 local_14;
  i16 local_12;
  i16 local_10;
  i16 local_e;
  u32 local_c;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (u32 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = ((int)puVar1 + 0x4);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(0x1050,&local_10),(char *)CONCAT22(0x1050,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_14),(char *)CONCAT22(0x1050,&local_12))
  ;
  if ((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) && (local_10 < local_14 + -0xb)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_b240(u16 param_1,u32 param_2,u32 param_3,u32 param_4)

{
  u32 *puVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u32 uVar7;
  u8 bStack31;
  u32 local_a;
  u32 uStack6;

  puVar1 = &local_a;
  uVar6 = (param_4 >> 0x10);
  pass1_1030_64ce(puVar1,param_1,_PTR_LOOP_1050_5740,(u16 *)param_3,*(i32 *)((int)param_4 + 0x4),
                  (u32 *)CONCAT22(0x1050,puVar1));
  uStack6 = *puVar1;
  uVar5 = ((int)puVar1 + 0x2);
  bStack31 = (u8)(uStack6 >> 0x18);
  uVar2 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar5 << 0x10);
    uVar7 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar2),uVar2,uVar5);
    uVar4 = (uVar7 >> 0x10);
    uVar2 = uVar7;
    uVar5 = uVar4 | uVar2;
    if ((uVar5 != 0x0) && (uVar2 = (uVar2 + 0xc), 0x9 < (int)uVar2)) {
      return;
    }
  }
  uVar3 = pass1_1020_b1ae(uVar2,uVar5,param_2,(param_2 >> 0x10),(u16 *)param_3,
                          (u32)((int)param_4 + 0x4));
  if (uVar3 == 0x0) {
    return;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1020_b2da(u16 param_1,u16 param_2,i16 param_3,u16 *param_4,u32 param_5)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  u8 *puVar4;
  u16 uVar5;
  u16 *puVar6;
  u16 in_stack_0000fe78;
  astruct_19 **ppaVar7;
  i16 iStack28;
  u8 local_1a [0x6];
  u16 uStack20;
  u16 uStack18;
  i16 *piStack16;
  i16 *piStack12;
  u16 local_8;
  u32 local_6;

  if (param_3 == 0x0) {
    uVar2 = 0x4e6a;
  }
  else {
    uVar2 = 0x4e6e;
  }
  piStack12 = (i16 *)CONCAT22(0x1050,uVar2);
  if (param_3 == 0x0) {
    uStack20 = 0x4e68;
  }
  else {
    uStack20 = 0x4e6c;
  }
  uStack18 = SUB42(&DAT_1050_1050,0x0);
  piStack16 = (i16 *)CONCAT22(0x1050,uStack20);
  do {
    if (param_3 == 0x0) {
      ppaVar7 = &PTR_1048_4230;
    }
    else {
      ppaVar7 = (astruct_19 **)0x10484236;
    }
    pass1_1008_3eb4((astruct_615 *)ppaVar7,(u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                    (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
      local_6 = CONCAT22(local_6 + *piStack16,(int)local_6 + -0x1);
    }
    else if (iVar1 == 0x1) {
      local_6 = CONCAT22(local_6 + -0x1,(int)local_6 + *piStack16);
    }
    else if (iVar1 == 0x2) {
      local_6 = CONCAT22(local_6 - *piStack16,(int)local_6 + -0x1);
    }
    puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_1a),local_8,local_6,(local_6 >> 0x10));
    uVar5 = ((u32)puVar6 >> 0x10);
    uVar2 = (param_5 >> 0x10);
    uVar3 = pass1_1020_b1ae((int)local_1a,uVar5,param_1,param_2,(u16 *)CONCAT22(0x1050,local_1a),
                            (u32)((int)param_5 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(uVar5,CONCAT22(param_2,param_1),CONCAT22(0x1050,puVar4),param_5);
      if (puVar4 != NULL) {
LAB_1020_b46e:
        pass1_1008_3e76(param_4,local_8,local_6,(local_6 >> 0x10));
        return;
      }
    }
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
LAB_1020_b45e:
      local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 + 0x2);
    }
    else if (iVar1 == 0x1) {
      local_6 = local_6 & 0xffff | (u32)(local_6 + 0x2) << 0x10;
    }
    else if (iVar1 == 0x2) goto LAB_1020_b45e;
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,local_1a),local_8,local_6,(local_6 >> 0x10));
    uVar3 = pass1_1020_b1ae((int)local_1a,uVar5,param_1,param_2,(u16 *)CONCAT22(0x1050,local_1a),
                            (u32)((int)param_5 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(uVar5,CONCAT22(param_2,param_1),CONCAT22(0x1050,puVar4),param_5);
      if (puVar4 != NULL) goto LAB_1020_b46e;
    }
    iStack28 = *piStack12 + 0x1;
    if (0x2 < iStack28) {
      iStack28 = 0x0;
      *piStack16 = *piStack16 + 0x1;
    }
    *piStack12 = iStack28;
    pass1_1020_ac6e(in_stack_0000fe78,CONCAT22(param_2,param_1),param_3,*piStack16,iStack28);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_b482(u32 param_1,u32 *param_2,u32 param_3)

{
  u8 *puVar1;
  astruct_92 **ppaVar2;
  u32 *puVar3;
  u32 uVar4;
  u16 uVar5;
  u16 uVar6;
  u32 *puVar7;
  u32 uVar8;
  u16 uVar9;
  u16 uVar10;
  u32 *puVar11;
  i16 iStack46;
  u32 local_2a;
  u16 local_26;
  u32 local_24;
  u16 uStack32;
  i32 lStack30;
  u32 uStack26;
  astruct_92 *local_16;
  u8 local_4 [0x2];

  uVar8 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar5 = (uVar8 >> 0x10);
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x400);
  while( true ) {
    ppaVar2 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar2));
    uStack26 = CONCAT22(uVar5,ppaVar2);
    uVar6 = uVar5 | ppaVar2;
    if (uVar6 == 0x0) {
      pass1_1020_b240(0x0,param_1,(u32)param_2,param_3);
      if (ppaVar2 != NULL) {
        lStack30 = *(i32 *)((int)param_3 + 0x4);
        local_24 = *param_2;
        uStack32 = ((int)param_2 + 0x4);
        puVar7 = &local_2a;
        pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_24),(u16 *)CONCAT22(0x1050,puVar7),
                        (u16 *)CONCAT22(0x1050,(int)&local_2a + 0x2),(u16 *)CONCAT22(0x1050,&local_26));
        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,local_26 - 0x1);
        puVar3 = &local_24;
        uVar9 = param_1;
        uVar10 = (param_1 >> 0x10);
        pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
        if (puVar3 != NULL) {
          pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,((u32)local_2a >> 0x10),local_26 - 0x1)
          ;
          puVar3 = &local_24;
          pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
          if (puVar3 != NULL) {
            pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x1,local_26 - 0x1);
            puVar3 = &local_24;
            pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
            if (puVar3 != NULL) {
              pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,local_26);
              puVar3 = &local_24;
              pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
              if (puVar3 != NULL) {
                pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x1,local_26);
                puVar3 = &local_24;
                pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                if (puVar3 != NULL) {
                  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x1,local_26 + 0x1);
                  puVar3 = &local_24;
                  pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                  if (puVar3 != NULL) {
                    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,((u32)local_2a >> 0x10),
                                    local_26 + 0x1);
                    puVar3 = &local_24;
                    pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                    if (puVar3 != NULL) {
                      pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,
                                      local_26 + 0x1);
                      puVar3 = &local_24;
                      pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                      if (puVar3 != NULL) {
                        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x2,
                                        local_26 - 0x2);
                        puVar3 = &local_24;
                        pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                        if (puVar3 != NULL) {
                          pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x2,
                                          local_26 - 0x2);
                          puVar3 = &local_24;
                          pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                          if (puVar3 != NULL) {
                            pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x2,
                                            local_26 + 0x2);
                            puVar3 = &local_24;
                            pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                            if (puVar3 != NULL) {
                              pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x2,
                                              local_26 + 0x2);
                              puVar3 = &local_24;
                              pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                              if (puVar3 != NULL) {
                                pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,
                                                local_26 + 0x2);
                                puVar3 = &local_24;
                                pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                                if (puVar3 != NULL) {
                                  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,
                                                  local_26 + 0x3);
                                  puVar3 = &local_24;
                                  pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30
                                                 );
                                  if (puVar3 != NULL) {
                                    iStack46 = 0x3;
                                    while( true ) {
                                      if (0x9 < iStack46) {
                                        return;
                                      }
                                      pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),0x0,local_2a - iStack46,
                                                      local_26);
                                      puVar3 = &local_24;
                                      pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),
                                                      lStack30);
                                      if (puVar3 == NULL) break;
                                      iStack46 += 0x1;
                                    }
                                    return;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      return;
    }
    uVar4 = (u32)(ppaVar2 + 0x8);
    puVar11 = param_2;
    uVar8 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4);
    puVar1 = local_4;
    pass1_1030_bcbc(puVar1,CONCAT22((int)uVar4,0x1050),CONCAT22((int)puVar11,uVar6),
                    ((u32)puVar11 >> 0x10),uVar8);
    if ((int)puVar1 < 0x0) break;
    uVar5 = uVar6;
    if ((int)puVar1 < 0x65) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_b872(u32 param_1,u32 param_2)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u32 *puVar4;
  u8 *puVar5;
  u32 *puVar6;
  u16 *puVar7;
  u16 uVar8;
  u8 local_136 [0x124];
  u32 local_12;
  i16 local_c;
  i16 local_a;
  u32 local_8;
  u16 uStack4;

  uVar8 = (param_2 >> 0x10);
  puVar6 = (u32 *)pass1_1030_5b5c((int)param_2,uVar8);
  local_8 = *puVar6;
  uStack4 = ((int)puVar6 + 0x4);
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_c),(char *)CONCAT22(0x1050,&local_a));
  uVar1 = local_a - 0xa;
  pass1_1008_612e(uVar1,0xa,uVar1);
  uVar2 = local_c - 0xa;
  pass1_1008_612e(uVar2,0xa,uVar2);
  puVar7 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_12),0x0,uVar2,uVar1);
  uVar2 = ((u32)puVar7 >> 0x10);
  while( true ) {
    puVar4 = &local_12;
    pass1_1020_b482(param_1,(u32 *)CONCAT22(0x1050,puVar4),param_2);
    if (puVar4 != NULL) break;
    uVar1 = local_a - 0xa;
    pass1_1008_612e(uVar1,0xa,uVar1);
    uVar3 = local_c - 0xa;
    pass1_1008_612e(uVar3,0xa,uVar3);
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_12),0x0,uVar3,uVar1);
  }
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_136),0x0,0xa,&local_12,&DAT_1050_1050,0x8000002,0x0,
                      (u32)((int)param_2 + 0x4));
  puVar5 = local_136;
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,puVar5));
  pass1_1020_b97e(puVar5,uVar2,param_1,(param_1 >> 0x10),0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_b97e(i16 param_1,u16 param_2,u16 param_3,u16 param_4,i16 param_5)

{
  u32 uVar1;
  i16 local_e;
  u16 local_c;
  i16 iStack10;
  u16 uStack8;
  u32 uStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
  PTR_LOOP_1050_4e70 = CONCAT22(param_2,param_1);
  uVar1 = (u32)(param_1 + 0x10);
  uStack6 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  iStack10 = (int)uVar1;
  uStack8 = param_2;
  pass1_1008_3f62((u16 *)&PTR_1048_4230,(u16 *)CONCAT22(param_2,iStack10 + 0xc));
  pass1_1008_3e94((u16 *)&PTR_1048_4230,(u16 *)CONCAT22(0x1050,&local_e),(char *)CONCAT22(0x1050,&local_c));
  if (param_5 == 0x0) {
    pass1_1008_3e76((u16 *)&PTR_1048_4230,0x0,local_e + 0x1,local_c - 0x1);
    pass1_1008_3e94((u16 *)&PTR_1048_4230,(u16 *)CONCAT22(0x1050,&local_e),(char *)CONCAT22(0x1050,&local_c));
  }
  pass1_1008_3e76((u16 *)0x10484236,0x1,local_e - 0x2,local_c);
  return;
}



void pass1_1020_ba2b(void)

{
  init_globals_1020_96d4();
  pass1_1020_a426();
  return;
}



void pass1_1020_ba3e(astruct_172 *param_1,u16 param_2,i16 param_3)

{
  astruct_172 *iVar1;
  astruct_172 *uVar1;

  uVar1 = (astruct_172 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_172 *)param_1;
  (u32)param_1 = 0x0;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x6 = param_3;
  iVar1->field4_0x8 = param_2;
  if (iVar1->field3_0x6 == 0x0) {
    iVar1->field3_0x6 = 0x5;
  }
  pass1_1020_bcc4(param_1);
  return;
}



void fn_ptr_1020_ba7e(u32 *param_1)

{
  fn_ptr_1000_17ce((char *)*param_1);
  return;
}



void pass1_1020_ba94(i32 *param_1)

{
  u16 *puVar1;
  u16 uStack8;

  if (*param_1 == 0x0) {
    return;
  }
  uStack8 = 0x0;
  while( true ) {
    puVar1 = (u16 *)((int)param_1 + 0x4);
    if (*puVar1 < uStack8 || *puVar1 == uStack8) break;
    uStack8 += 0x1;
  }
  return;
}



u32 pass1_1020_bae6(u16 param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u16 *puStack6;

  pass1_1020_bc92((u16 *)CONCAT22((int)param_4,param_3),(param_4 >> 0x10));
  puStack6 = (u16 *)CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0x0) {
    return CONCAT22((param_1 + 0x2),*puStack6);
  }
  return 0x0;
}



void pass1_1020_bb16(u32 *param_1,u32 *param_2,u16 *param_3,u16 param_4)

{
  if (((int)param_1 + 0x4) < param_4) {
    *param_3 = 0x0;
    *param_2 = 0x0;
    return;
  }
  *param_3 = (param_4 * 0x6 + (int)*param_1 + 0x4);
  *param_2 = (u32)((int)*param_1 + param_4 * 0x6);
  return;
}



void pass1_1020_bb70(i32 *param_1,u16 param_2,u32 param_3)

{
  u16 in_AX;
  u16 in_DX;

  pass1_1020_bba4((astruct_172 *)param_1,0x1,param_2,(int)param_3,((u32)param_3 >> 0x10),in_AX,in_DX);
  return;
}



void pass1_1020_bb8a(i32 *param_1,u16 param_2,u32 param_3)

{
  u16 in_AX;
  u16 in_DX;

  pass1_1020_bba4((astruct_172 *)param_1,0x0,param_2,(int)param_3,((u32)param_3 >> 0x10),in_AX,in_DX);
  return;
}



BOOL16 pass1_1020_bba4(astruct_172 *param_1,i16 param_2,u16 param_3,i16 param_4,u16 param_5,u16 *param_6,
                      u16 param_7)

{
  u16 uVar1;
  u16 uVar2;
  bool bVar3;
  u16 *puStack6;

  pass1_1020_bc92(&param_1->field0_0x0,param_5);
  puStack6 = (u16 *)CONCAT22(param_7,param_6);
  uVar1 = param_7 | param_6;
  if (uVar1 == 0x0) {
    pass1_1020_bc92(&param_1->field0_0x0,0x0);
    uVar2 = uVar1 | param_6;
    if (uVar2 == 0x0) {
      pass1_1020_bcc4(param_1);
      pass1_1020_bc92(&param_1->field0_0x0,0x0);
      if ((uVar2 | param_6) == 0x0) {
        return 0x0;
      }
      param_6[0x2] = param_5;
      uVar1 = uVar2;
    }
    else {
      param_6[0x2] = param_5;
    }
    if (param_2 != 0x0) {
      uVar2 = *param_6;
      bVar3 = CARRY2(uVar2,param_3);
      param_3 = uVar2 + param_3;
      param_4 = param_6[0x1] + param_4 + bVar3;
    }
    *param_6 = param_3;
    param_6[0x1] = param_4;
    pass1_1020_bc72(param_1);
  }
  else {
    if (param_2 != 0x0) {
      bVar3 = CARRY2(*puStack6,param_3);
      param_3 = *puStack6 + param_3;
      param_4 = param_6[0x1] + param_4 + bVar3;
    }
    *param_6 = param_3;
    param_6[0x1] = param_4;
  }
  return 0x1;
}



void pass1_1020_bc72(astruct_172 *param_1)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_1 + 0x2);
  pass1_1000_4aea(param_1->field0_0x0,uVar1,(int)((u32)uVar1 >> 0x10),0x6,(u8 *)0xbd6c);
  return;
}



void pass1_1020_bc92(u16 *param_1,u16 param_2)

{
  u32 uVar1;
  u8 local_c [0x4];
  u16 uStack8;

  uStack8 = param_2;
  uVar1 = (u32)((int)param_1 + 0x2);
  pass1_1000_49c6(local_c,&DAT_1050_1050,*param_1,uVar1,((u32)uVar1 >> 0x10),0x6,
                  (u8 *)0xbd6c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_bcc4(astruct_172 *param_1)

{
  u16 *puVar1;
  u16 uVar2;
  u16 uVar3;
  u32 in_EDX;
  StructD *pSVar4;
  astruct_172 *pstruct172_5;
  astruct_172 *pstruct172_4;
  i32 lVar5;
  u32 uStack12;

  pstruct172_4 = (astruct_172 *)((u32)param_1 >> 0x10);
  pstruct172_5 = (astruct_172 *)param_1;
  if (pstruct172_5->field2_0x4 == 0x0) {
    pSVar4 = (StructD *)(in_EDX & 0xffff0000);
    uVar2 = pstruct172_5->field3_0x6;
  }
  else {
    uVar3 = pstruct172_5->field2_0x4;
    puVar1 = &pstruct172_5->field4_0x8;
    uVar2 = uVar3 + *puVar1;
    pSVar4 = (StructD *)(in_EDX & 0xffff0000 | (u32)CARRY2(uVar3,*puVar1));
  }
  if ((int)pSVar4 == 0x0) {
    if (*(i32 *)param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar4);
        PTR_LOOP_1050_5f2e = (u8 *)pSVar4;
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(uVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    }
    else {
      lVar5 = pass1_1000_0ed4(0x1,uVar2 * 0x6,0x0,(astruct_172 *)(u32)param_1,
                              (astruct_172 *)((u32)(u32)param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (u8 *)((u32)lVar5 >> 0x10);
      uVar3 = lVar5;
    }
    uStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if ((PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      pstruct172_5->field2_0x4 = uVar2;
      (u32)param_1 = uStack12;
      pass1_1020_bc72((astruct_172 *)((u32)param_1 & 0xffff | ZEXT24(pstruct172_4) << 0x10));
    }
  }
  return;
}



i16 pass1_1020_bd6c(u32 param_1,u32 param_2)

{
  return ((int)param_1 + 0x4) - ((int)param_2 + 0x4);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1020_bd80(u16 param_1)

{
  char *pcVar1;
  u16 uVar2;
  u16 uStack6;

  switch(param_1) {
  case 0x1:
  case 0x6:
    uVar2 = 0x427;
    break;
  case 0x2:
    uVar2 = 0x428;
    break;
  case 0x3:
  case 0x7:
    uVar2 = 0x429;
    break;
  case 0x4:
  case 0x8:
    uVar2 = 0x425;
    break;
  case 0x5:
  case 0x9:
    uVar2 = 0x426;
    break;
  case 0xa:
    uVar2 = 0x402;
    break;
  case 0xb:
  case 0x37:
    uVar2 = 0x418;
    break;
  case 0xc:
  case 0x35:
  case 0x36:
    uVar2 = 0x42a;
    break;
  case 0xd:
    uVar2 = 0x5f7;
    break;
  case 0xe:
    uVar2 = 0x5f6;
    break;
  case 0xf:
    uVar2 = 0x403;
    break;
  case 0x10:
    uVar2 = 0x5f8;
    break;
  case 0x11:
    uVar2 = 0x404;
    break;
  case 0x12:
    uVar2 = 0x405;
    break;
  case 0x13:
  case 0x14:
  case 0x15:
    uVar2 = 0x406;
    break;
  case 0x16:
  case 0x19:
    uVar2 = 0x5f9;
    break;
  case 0x17:
  case 0x1a:
    uVar2 = 0x5fa;
    break;
  case 0x18:
    uVar2 = 0x5fb;
    break;
  case 0x1b:
  case 0x1c:
  case 0x1d:
    uVar2 = 0x408;
    break;
  case 0x1e:
  case 0x1f:
  case 0x20:
    uVar2 = 0x409;
    break;
  case 0x21:
    uVar2 = 0x42c;
    break;
  case 0x22:
  case 0x23:
  case 0x24:
    uVar2 = 0x40a;
    break;
  case 0x25:
  case 0x26:
  case 0x27:
    uVar2 = 0x40b;
    break;
  case 0x28:
  case 0x29:
    uVar2 = 0x40c;
    break;
  case 0x2a:
  case 0x2b:
    uVar2 = 0x40d;
    break;
  case 0x2c:
    uVar2 = 0x40e;
    break;
  case 0x2d:
  case 0x2e:
    uVar2 = 0x40f;
    break;
  case 0x2f:
  case 0x30:
    uVar2 = 0x410;
    break;
  case 0x31:
  case 0x32:
    uVar2 = 0x411;
    break;
  case 0x33:
  case 0x34:
    uVar2 = 0x416;
    break;
  case 0x38:
  case 0x39:
    uVar2 = 0x5fc;
    break;
  case 0x3a:
  case 0x3b:
    uVar2 = 0x419;
    break;
  case 0x3c:
  case 0x3d:
    uVar2 = 0x5fd;
    break;
  case 0x3e:
    uVar2 = 0x5fe;
    break;
  case 0x3f:
    uVar2 = 0x5ff;
    break;
  case 0x40:
    uVar2 = 0x600;
    break;
  case 0x41:
    uVar2 = 0x601;
    break;
  case 0x42:
  case 0x46:
  case 0x6b:
    uVar2 = 0x602;
    break;
  case 0x43:
    uStack6 = s_bidLRoadConst_1050_4e7a;
    return uStack6;
  case 0x44:
    uStack6 = s_bidRRoadConst_1050_4e88;
    return uStack6;
  case 0x45:
    uStack6 = s_bidXRoadConst_1050_4e96;
    return uStack6;
  case 0x47:
    uVar2 = 0x42b;
    break;
  case 0x48:
  case 0x49:
  case 0x4a:
  case 0x70:
  case 0x71:
  case 0x72:
    uVar2 = 0x603;
    break;
  case 0x4b:
    uVar2 = 0x42d;
    break;
  case 0x4c:
    uVar2 = 0x604;
    break;
  case 0x4d:
    uVar2 = 0x605;
    break;
  case 0x4e:
    uVar2 = 0x606;
    break;
  case 0x4f:
  case 0x50:
  case 0x51:
    uVar2 = 0x41a;
    break;
  case 0x52:
  case 0x53:
    uVar2 = 0x41b;
    break;
  case 0x54:
  case 0x55:
  case 0x56:
    uVar2 = 0x41d;
    break;
  case 0x57:
  case 0x58:
  case 0x59:
    uVar2 = 0x41e;
    break;
  case 0x5a:
    uVar2 = 0x41f;
    break;
  case 0x5b:
  case 0x5c:
    uVar2 = 0x421;
    break;
  case 0x5d:
  case 0x5e:
  case 0x5f:
    uVar2 = 0x420;
    break;
  case 0x60:
  case 0x61:
    uVar2 = 0x607;
    break;
  case 0x62:
  case 0x63:
    uVar2 = 0x608;
    break;
  case 0x64:
    uVar2 = 0x609;
    break;
  case 0x65:
    uVar2 = 0x422;
    break;
  case 0x66:
  case 0x67:
    uVar2 = 0x423;
    break;
  case 0x68:
  case 0x69:
    uVar2 = 0x60a;
    break;
  case 0x6a:
    uVar2 = 0x60b;
    break;
  case 0x6c:
  case 0x6d:
    uVar2 = 0x41c;
    break;
  case 0x6e:
    uVar2 = 0x60c;
    break;
  case 0x6f:
    uVar2 = 0x60d;
    break;
  case 0x73:
  case 0x77:
    uVar2 = 0x415;
    break;
  case 0x74:
  case 0x78:
  case 0x79:
    uVar2 = 0x412;
    break;
  case 0x75:
    uVar2 = 0x413;
    break;
  case 0x76:
    uVar2 = 0x414;
    break;
  case 0x7a:
    uVar2 = 0x60e;
    break;
  case 0x7b:
    uVar2 = 0x60f;
    break;
  case 0x7c:
    uVar2 = 0x610;
    break;
  case 0x7d:
    uVar2 = 0x611;
    break;
  case 0x7e:
    uVar2 = 0x612;
    break;
  case 0x7f:
    uVar2 = 0x613;
    break;
  case 0x80:
    uVar2 = 0x614;
    break;
  case 0x81:
    uVar2 = 0x615;
    break;
  case 0x82:
    uVar2 = 0x616;
    break;
  case 0x83:
    uVar2 = 0x617;
    break;
  case 0x84:
    uVar2 = 0x618;
    break;
  case 0x85:
    uVar2 = 0x619;
    break;
  case 0x86:
    uVar2 = 0x61a;
    break;
  case 0x87:
    uVar2 = 0x61b;
    break;
  case 0x88:
    uVar2 = 0x61c;
    break;
  case 0x89:
    uVar2 = 0x61d;
    break;
  default:
    uVar2 = 0x424;
  }
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,uVar2);
  return pcVar1;
}



void string_1020_c0ca(u16 param_1)

{
  string_1020_c0d8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * string_1020_c0d8(u16 param_1)

{
  char *pcVar1;
  u16 uVar2;

  switch(param_1) {
  case 0x1:
    uVar2 = 0x5b7;
    break;
  case 0x2:
    uVar2 = 0x5b8;
    break;
  case 0x3:
    uVar2 = 0x5b9;
    break;
  case 0x4:
    uVar2 = 0x5ba;
    break;
  case 0x5:
    uVar2 = 0x5bb;
    break;
  case 0x6:
    uVar2 = 0x5bc;
    break;
  case 0x7:
    uVar2 = 0x5bd;
    break;
  case 0x8:
    uVar2 = 0x5be;
    break;
  case 0x9:
    uVar2 = 0x5bf;
    break;
  case 0xa:
    uVar2 = 0x5c0;
    break;
  case 0xb:
    uVar2 = 0x5c1;
    break;
  case 0xc:
    uVar2 = 0x5c2;
    break;
  case 0xd:
    uVar2 = 0x5c3;
    break;
  case 0xe:
    uVar2 = 0x5c4;
    break;
  case 0xf:
    uVar2 = 0x5c5;
    break;
  case 0x10:
    uVar2 = 0x5c6;
    break;
  case 0x11:
    uVar2 = 0x5c7;
    break;
  case 0x12:
    uVar2 = 0x5c8;
    break;
  case 0x13:
    uVar2 = 0x5c9;
    break;
  case 0x14:
    uVar2 = 0x5ca;
    break;
  case 0x15:
    uVar2 = 0x5cb;
    break;
  case 0x16:
    uVar2 = 0x5cc;
    break;
  case 0x17:
    uVar2 = 0x5cd;
    break;
  case 0x18:
    uVar2 = 0x5ce;
    break;
  case 0x19:
    uVar2 = 0x5cf;
    break;
  case 0x1a:
    uVar2 = 0x5d0;
    break;
  case 0x1b:
    uVar2 = 0x5d1;
    break;
  case 0x1c:
    uVar2 = 0x5d2;
    break;
  case 0x1d:
    uVar2 = 0x5d3;
    break;
  case 0x1e:
    uVar2 = 0x5d4;
    break;
  case 0x1f:
    uVar2 = 0x5d5;
    break;
  default:
    uVar2 = 0x5d9;
    break;
  case 0x21:
    uVar2 = 0x5d6;
    break;
  case 0x23:
    uVar2 = 0x5d7;
    break;
  case 0x24:
    uVar2 = 0x5e5;
  }
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,uVar2);
  return (char *)pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * string_op_1020_c222(u16 param_1)

{
  char *pcVar1;
  u16 uVar2;

  switch(param_1) {
  case 0x1:
    uVar2 = 0x57f;
    break;
  case 0x2:
    uVar2 = 0x580;
    break;
  case 0x3:
    uVar2 = 0x581;
    break;
  case 0x4:
    uVar2 = 0x5e7;
    break;
  case 0x5:
    uVar2 = 0x57e;
    break;
  case 0x6:
    uVar2 = 0x582;
    break;
  case 0x7:
    uVar2 = 0x3e8;
    break;
  case 0x8:
    uVar2 = 0x3e9;
    break;
  case 0x9:
    uVar2 = 0x3ea;
    break;
  case 0xa:
    uVar2 = 0x3eb;
    break;
  case 0xb:
    uVar2 = 0x3ec;
    break;
  case 0xc:
    uVar2 = 0x3ed;
    break;
  case 0xd:
    uVar2 = 0x3ee;
    break;
  case 0xe:
    uVar2 = 0x3ef;
    break;
  case 0xf:
    uVar2 = 0x3f0;
    break;
  case 0x10:
    uVar2 = 0x3f1;
    break;
  case 0x11:
    uVar2 = 0x3f2;
    break;
  case 0x12:
    uVar2 = 0x3f4;
    break;
  case 0x13:
    uVar2 = 0x3f5;
    break;
  case 0x14:
    uVar2 = 0x532;
    break;
  default:
    uVar2 = 0x5d9;
  }
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,uVar2);
  return (char *)pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * string_op_1020_c2f8(u16 param_1)

{
  char *pcVar1;
  u16 uVar2;

  switch(param_1) {
  case 0x1:
    uVar2 = 0x583;
    break;
  case 0x2:
    uVar2 = 0x584;
    break;
  case 0x3:
    uVar2 = 0x585;
    break;
  case 0x4:
    uVar2 = 0x586;
    break;
  case 0x5:
    uVar2 = 0x587;
    break;
  case 0x6:
    uVar2 = 0x588;
    break;
  case 0x7:
    uVar2 = 0x589;
    break;
  case 0x8:
    uVar2 = 0x58a;
    break;
  case 0x9:
    uVar2 = 0x58b;
    break;
  case 0xa:
    uVar2 = 0x58c;
    break;
  case 0xb:
    uVar2 = 0x58d;
    break;
  case 0xc:
    uVar2 = 0x58e;
    break;
  case 0xd:
    uVar2 = 0x58f;
    break;
  case 0xe:
    uVar2 = 0x3f8;
    break;
  case 0xf:
    uVar2 = 0x3f9;
    break;
  case 0x10:
    uVar2 = 0x532;
    break;
  default:
    uVar2 = 0x5d9;
  }
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,uVar2);
  return (char *)pcVar1;
}



u16 pass1_1020_c3ae(void)

{
  return 0x1;
}



u16 switch_1020_c3b4(u16 param_1)

{
  u16 uStack6;

  uStack6 = 0x1;
  switch(param_1) {
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x5:
  case 0x8:
  case 0x9:
  case 0xa:
  case 0xb:
  case 0xc:
    uStack6 = 0x3;
    break;
  case 0x4:
    uStack6 = 0x6;
    break;
  case 0x6:
  case 0xf:
  case 0x10:
  case 0x11:
  case 0x12:
  case 0x13:
    uStack6 = 0xa;
    break;
  case 0x7:
    uStack6 = 0x2;
    break;
  case 0xd:
  case 0xe:
    uStack6 = 0x1;
  }
  return uStack6;
}



u16 pass1_1020_c42e(i16 param_1)

{
  u16 uVar1;

  if (param_1 == 0xf) {
    uVar1 = 0x1;
  }
  else {
    uVar1 = 0x3;
  }
  return uVar1;
}



void struct_1020_c444(astruct_75 *param_1,u32 param_2,u32 param_3)

{
  astruct_75 *iVar1;
  astruct_75 *uVar1;

  struct_op_1030_1cd8(param_1,param_2,param_3);
  uVar1 = (astruct_75 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_75 *)param_1;
  (u32)(iVar1 + 0x1) = 0x0;
  &iVar1[0x1].field2_0x4 = 0x0;
  param_1->field0_0x0 = 0xc834;
  iVar1->field1_0x2 = 0x1020;
  return;
}



void pass1_1020_c47a(u16 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0xc834;
  ((int)param_1 + 0x2) = 0x1020;
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x18));
  pass1_1030_1d28((StructD *)param_1);
  return;
}



void pass1_1020_c4a8(u32 param_1,u16 *param_2,u32 *param_3,i16 param_4)

{
  u32 uVar1;
  u32 *puVar2;
  u16 uVar3;
  u16 uVar4;

  uVar3 = (param_1 >> 0x10);
  if (((int)param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4((astruct_359 *)(param_1 & 0xffff | (u32)uVar3 << 0x10));
  }
  uVar1 = (u32)((int)param_1 + 0x18);
  uVar4 = ((u32)uVar1 >> 0x10);
  puVar2 = (u32 *)((int)uVar1 + param_4 * 0x6);
  *param_3 = *puVar2;
  *param_2 = (puVar2 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_c4f4(astruct_361 *param_1,u16 param_2,astruct_360 *param_3,u16 param_4,u16 param_5,u32 param_6)

{
  astruct_361 *paVar1;
  u16 uVar2;
  u16 uVar3;

  pass1_1020_c6de(param_3,param_6);
  uVar3 = param_2 | param_1;
  if (uVar3 != 0x0) {
    paVar1 = param_1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
    uVar2 = pass1_1030_6fa0(CONCAT22(uVar3,paVar1));
    param_1->field4_0x4 = (uVar2 * 0x2 + 0x4ea4);
  }
  return;
}



u32 pass1_1020_c538(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x12),((int)param_1 + 0x10));
}



void pass1_1020_c54a(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4((astruct_359 *)(param_1 & 0xffff | (u32)uVar1 << 0x10));
  }
  return;
}



u16 FUN_1020_c5ae(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void FUN_1020_c5b4(u16 param_1,u32 *param_2,u32 param_3)

{
  i32 *plVar1;
  code **ppcVar2;
  u16 in_AX;
  u16 uVar3;
  u16 uVar4;
  u16 in_DX;
  u16 DX_REG;
  u16 uVar5;
  u16 uVar6;
  u32 *puStack12;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
  uVar3 = pass1_1030_6fa0(CONCAT22(in_DX,in_AX));
  uVar4 = uVar3;
  pass1_1020_c6de((astruct_360 *)param_2,0x0);
  puStack12 = (u32 *)CONCAT22(in_DX,uVar4);
  uVar6 = ((u32)param_2 >> 0x10);
  if ((in_DX | uVar4) == 0x0) {
    ppcVar2 = (code **)((int)*param_2 + 0x20);
    (**ppcVar2)();
    uVar5 = DX_REG;
    pass1_1020_c6de((astruct_360 *)param_2,0x0);
    puStack12 = (u32 *)CONCAT22(uVar5,uVar4);
    if ((uVar5 | uVar4) == 0x0) {
      return;
    }
  }
  ((int)param_2 + 0x1c) = 0x1;
  plVar1 = (i32 *)((int)param_2 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  *puStack12 = param_3;
  ((int)puStack12 + 0x4) = (uVar3 * 0x2 + 0x4ea4);
  return;
}



void FUN_1020_c640(void)

{
  return;
}



void pass1_1020_c644(u32 *param_1,u16 param_2,u32 param_3)

{
  i32 *plVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  i16 iVar5;
  u16 uVar6;
  u32 *puStack6;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(i32 *)(iVar5 + 0x18) == 0x0) {
    ppcVar3 = (code **)((int)*param_1 + 0x20);
    (**ppcVar3)();
  }
  iVar4 = (iVar5 + 0x8) * 0x6 + (iVar5 + 0x18);
  uVar2 = (iVar5 + 0x1a);
  puStack6 = (u32 *)CONCAT22(uVar2,iVar4);
  plVar1 = (i32 *)(iVar5 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  *puStack6 = param_3;
  (iVar4 + 0x4) = param_2;
  return;
}



void pass1_1020_c694(u32 param_1)

{
  pass1_1020_c6a4((astruct_359 *)param_1);
  return;
}



void pass1_1020_c6a4(astruct_359 *param_1)

{
  i32 lVar1;
  astruct_359 *iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_359 *)param_1;
  if ((iVar2->field20_0x18 != 0x0) && (iVar2->field8_0x8 != 0x0)) {
    lVar1 = iVar2->field20_0x18;
    pass1_1000_4aea(lVar1,((u32)lVar1 >> 0x10),iVar2->field13_0x10,0x6,(u8 *)0xc7fa);
    iVar2->field21_0x1c = 0x0;
  }
  return;
}



void pass1_1020_c6de(astruct_360 *param_1,i32 param_2)

{
  u32 *puVar1;
  u32 uVar2;
  astruct_360 *iVar3;
  astruct_360 *uVar3;
  u32 uStack6;

  uVar3 = (astruct_360 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_360 *)param_1;
  if (iVar3->field22_0x1c != 0x0) {
    pass1_1020_c6a4((astruct_359 *)((u32)param_1 & 0xffff | ZEXT24(uVar3) << 0x10));
  }
  uStack6 = 0x0;
  while( true ) {
    puVar1 = &iVar3->field16_0x10;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar2 = iVar3->field21_0x18;
    if (*(i32 *)((int)uVar2 + (int)uStack6 * 0x6) == param_2) break;
    uStack6 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_c73a(u32 param_1)

{
  u16 *puVar1;
  u16 uVar2;
  u32 uVar3;
  u16 uVar4;
  u32 in_EDX;
  StructD *pSVar5;
  i16 iVar6;
  u16 uVar7;
  i32 lVar8;
  u32 uStack10;
  u32 uStack6;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(i32 *)(iVar6 + 0x10) == 0x0) {
    uVar4 = (iVar6 + 0xc);
    pSVar5 = (StructD *)(in_EDX & 0xffff0000 | (u32)(iVar6 + 0xe));
  }
  else {
    uVar2 = (iVar6 + 0x10);
    puVar1 = (u16 *)(iVar6 + 0x14);
    uVar4 = uVar2 + *puVar1;
    pSVar5 = (StructD *)
             (in_EDX & 0xffff0000 |
             (u32)((iVar6 + 0x12) + (iVar6 + 0x16) + CARRY2(uVar2,*puVar1)));
  }
  uStack6 = CONCAT22((int)pSVar5,uVar4);
  if (*(i32 *)(iVar6 + 0x18) == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar5;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  }
  else {
    uVar3 = (u32)(iVar6 + 0x18);
    lVar8 = pass1_1000_0ed4(0x1,uVar4 * 0x6,
                            ((int)pSVar5 * 0x3 + CARRY2(uVar4,uVar4) + CARRY2(uVar4 * 0x2,uVar4)) * 0x2 +
                            CARRY2(uVar4 * 0x3,uVar4 * 0x3),(astruct_172 *)uVar3,
                            (astruct_172 *)((u32)uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (u8 *)((u32)lVar8 >> 0x10);
    uVar4 = lVar8;
  }
  uStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if ((PTR_LOOP_1050_5f2e | uVar4) != 0x0) {
    (u32)(iVar6 + 0x10) = uStack6;
    (u32)(iVar6 + 0x18) = uStack10;
  }
  (iVar6 + 0x1c) = 0x1;
  return;
}



i16 pass1_1020_c7fa(u32 param_1,u32 param_2)

{
  return ((int)param_1 + 0x4) - ((int)param_2 + 0x4);
}



StructD * pass1_1020_c80e(StructD *param_1,u8 param_2)

{
  pass1_1020_c47a(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u32 pass1_1020_c860(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x6),((int)param_1 + 0x4));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_c872(u32 param_1,u32 param_2,u32 param_3)

{
  u16 *puVar1;
  u32 *puVar2;
  i16 *piVar3;
  astruct_98 *uVar4;
  u16 uVar6;
  i16 iVar7;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  bool bVar12;
  u32 uStack14;
  u32 uStack10;
  astruct_99 *puStack6;
  astruct_99 *uVar5;

  puStack6 = pass1_1000_07fc(_PTR_LOOP_1050_4fb8);
  uVar6 = ((u32)puStack6 >> 0x10);
  uVar5 = (astruct_99 *)puStack6;
  if ((uVar6 | uVar5) == 0x0) {
    puStack6 = NULL;
  }
  else {
    puStack6->field0_0x0 = 0x389a;
    uVar5->field1_0x2 = 0x1008;
    uVar5->field2_0x4 = 0x0;
    uVar5->field3_0x8 = 0x0;
    puStack6->field0_0x0 = s__s__s__1050_5bc0;
    uVar5->field1_0x2 = 0x1008;
    uVar5->field5_0xe = 0x0;
    uVar5->field4_0xc = 0x0;
    puStack6->field0_0x0 = 0xc9e6;
    uVar5->field1_0x2 = 0x1020;
  }
  if (puStack6 == NULL) {
    return;
  }
  uVar9 = ((u32)puStack6 >> 0x10);
  iVar7 = (int)puStack6;
  (u32)(iVar7 + 0x8) = param_3;
  (u32)(iVar7 + 0xc) = param_2;
  uVar10 = (param_1 >> 0x10);
  iVar8 = (int)param_1;
  uStack14 = (u32)(iVar8 + 0x4);
  uVar11 = (iVar8 + 0x6);
  if ((iVar8 + 0x8) == 0x0) {
LAB_1020_c92d:
    (u32)(iVar7 + 0x4) = (u32)(iVar8 + 0x4);
  }
  else {
    puVar1 = (u16 *)((int)uStack14 + 0xe);
    bVar12 = *puVar1 < param_2;
    if ((bVar12 || *puVar1 == param_2) &&
       ((bVar12 || (puVar1 = (u16 *)((int)uStack14 + 0xc), *puVar1 < param_2 || *puVar1 == param_2))))
    goto LAB_1020_c92d;
    bVar12 = false;
    while( true ) {
      if (uStack14 == 0x0) break;
      uVar11 = (uStack14 >> 0x10);
      puVar2 = (u32 *)((int)uStack14 + 0xc);
      if (*puVar2 < param_2 || *puVar2 == param_2) {
        bVar12 = true;
        (u32)(iVar7 + 0x4) = uStack14;
        *(astruct_99 **)((int)uStack10 + 0x4) = puStack6;
        break;
      }
      uStack10 = uStack14;
      uStack14 = (u32)((int)uStack14 + 0x4);
    }
    param_1 = uStack10;
    if (bVar12) goto LAB_1020_c9ab;
  }
  uVar11 = (param_1 >> 0x10);
  ((int)param_1 + 0x4) = iVar7;
  ((int)param_1 + 0x6) = uVar9;
LAB_1020_c9ab:
  piVar3 = (i16 *)(iVar8 + 0x8);
  *piVar3 = *piVar3 + 0x1;
  return;
}



u16 * pass1_1020_c9ba(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((astruct_611 *)param_1);
  }
  return param_1;
}



u16 * struct_1020_c9ea(u16 *param_1)

{
  struct_1028_0954((astruct_180 *)param_1);
  *param_1 = 0xcc7c;
  ((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



u16 * pass1_1020_ca0c(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xcc7c;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ca36(u16 param_1,astruct_15 *param_2,u16 param_3)

{
  u16 uVar1;
  u32 in_EDX;
  u16 uVar2;
  u32 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;

  uVar2 = ((u32)in_EDX >> 0x10);
  pass1_1028_09b8(param_2);
  uVar3 = pass1_1028_b4f2(param_2);
  uVar1 = (uVar3 >> 0x10);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,uVar1),_u16_1050_0ed0,(u8 **)CONCAT22(param_1,0x8),
                             in_stack_0000fe96,in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    pass1_1010_988c((u32)puVar4,((int)param_2 + 0xc));
  }
  return;
}



void pass1_1020_ca82(astruct_15 *param_1)

{
  u16 uVar1;
  u32 uVar3;
  u16 in_stack_0000ffb4;

  pass1_1028_be9e(param_1);
  uVar3 = pass1_1028_b4f2(param_1);
  uVar3 = (u8 *)(uVar3 >> 0x10);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    uVar1 = ((u32)param_1 >> 0x10);
    if (((int)param_1 + 0x12) == 0x5) {
      pass1_1020_cac2(uVar3,in_stack_0000ffb4,(astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_cac2(u8 *param_1,u16 param_2,astruct_15 *param_3)

{
  i16 *piVar1;
  code **ppcVar2;
  u32 uVar3;
  u32 uVar4;
  char *puVar3;
  u8 *puVar5;
  u16 uVar6;
  i16 iVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  astruct_57 *paVar9;
  u16 uVar10;
  u32 *puVar11;
  u16 in_stack_0000fe72;
  u16 in_stack_0000ff96;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ffa0;
  i16 iStack52;
  u8 *puStack36;
  char local_1c [0x4];
  u32 uStack24;
  u32 *puStack20;
  u32 *puStack16;
  u32 *puStack12;
  u8 *puStack8;
  u16 uStack6;
  u16 uStack4;

  paVar9 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar11 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x2),in_stack_0000fe72,
                            in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
  paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)puVar11 >> 0x10);
  uStack6 = SUB42(puVar11,0x0);
  uStack4 = ((u32)puVar11 >> 0x10);
  puStack8 = PTR_LOOP_1050_13ae;
  if (PTR_LOOP_1050_13ae == (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
    puStack8 = (u8 *)&u16_1050_0002;
  }
  puStack12 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x8),in_stack_0000fe72,
                              in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
  paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)puStack12 >> 0x10);
  uVar10 = ((u32)puStack12 >> 0x10);
  puStack16 = (u32 *)(u32)((int)puStack12 + 0xa);
  puStack20 = (u32 *)(u32)((int)puStack12 + 0xe);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_1c),(u32)puStack16);
  do {
    do {
      while( true ) {
        do {
          puVar3 = local_1c;
          pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
          uVar8 = paVar9;
          uVar3 = (u32)paVar9 & 0xffff0000;
          paVar9 = (astruct_57 *)(uVar3 | (uVar8 | puVar3));
          if ((uVar8 | puVar3) == 0x0) {
            return;
          }
          iVar7 = (puVar3 + 0x4);
        } while ((iVar7 < 0x12) || (SBORROW2(iVar7,0x12)));
        if (iVar7 != 0x13 && 0x0 < iVar7 + -0x12) break;
        iStack52 = 0x0;
        if (puStack8 == (u8 *)((int)&u16_1050_0002 + 0x1)) {
          iStack52 = (puVar3 + 0x6);
          uVar4 = (u32)(iStack52 >> 0xf);
          iStack52 /= 0x2;
LAB_1020_cc34:
          paVar9 = (astruct_57 *)(uVar3 | uVar4);
        }
        else if (puStack8 == (u8 *)&u32_1050_0004) {
          iVar7 = (puVar3 + 0x6) * 0x3;
          uVar4 = (u32)(iVar7 >> 0xf) & 0xffff0003;
          iStack52 = iVar7 + (int)uVar4 >> 0x2;
          goto LAB_1020_cc34;
        }
        piVar1 = (i16 *)(puVar3 + 0x6);
        *piVar1 = *piVar1 - iStack52;
        uVar10 = ((u32)puStack16 >> 0x10);
        ((int)puStack16 + 0xa) = 0x0;
        ppcVar2 = (code **)((int)*puStack16 + 0xc);
        (**ppcVar2)(0x1008,(int)puStack16,uVar10,puVar3,uVar8);
        ((int)puStack16 + 0xa) = 0x1;
        uStack24 = 0x0;
        ppcVar2 = (code **)((int)*puStack20 + 0x4);
        (**ppcVar2)(0x1008,(int)puStack20,(int)((u32)puStack20 >> 0x10),(char)puVar3,uVar8);
      }
    } while (iVar7 != 0x81);
    puStack36 = NULL;
    if (puStack8 == (u8 *)&u16_1050_0002) {
      iVar7 = (puVar3 + 0x6);
LAB_1020_cba7:
      uVar4 = (u32)(iVar7 >> 0xf) & 0xffff0003;
      puVar5 = (u8 *)(iVar7 + (int)uVar4 >> 0x2);
LAB_1020_cbb0:
      paVar9 = (astruct_57 *)(uVar3 | uVar4);
      puStack36 = puVar5;
    }
    else {
      if (puStack8 == (u8 *)((int)&u16_1050_0002 + 0x1)) {
        iVar7 = (puVar3 + 0x6);
        uVar4 = (u32)(iVar7 >> 0xf);
        puVar5 = (u8 *)(iVar7 / 0x2);
        goto LAB_1020_cbb0;
      }
      puVar5 = puStack8 + -0x4;
      if (puVar5 == NULL) {
        iVar7 = (puVar3 + 0x6) * 0x3;
        goto LAB_1020_cba7;
      }
    }
    pass1_1028_b58e(param_3);
    uVar10 = SUB42(paVar9,0x0);
    uVar6 = (puVar3 + 0x6) - (int)puStack36;
    paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)((int)uVar6 >> 0xf));
    pass1_1030_7ddc(uVar6,paVar9,CONCAT22(uVar10,puVar5),(long)(int)uVar6,0x1e);
    ppcVar2 = (code **)((int)*puStack16 + 0xc);
    (**ppcVar2)(0x1030,(int)puStack16,(int)((u32)puStack16 >> 0x10),puVar3,uVar8);
    uStack24 = 0x0;
  } while( true );
}



StructD * pass1_1020_cc56(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_cce4(astruct_180 *param_1)

{
  struct_1028_b354(param_1);
  param_1->field0_0x0 = 0xcd7e;
  ((int)param_1 + 0x2) = 0x1020;
  return &param_1->field0_0x0;
}



u16 * pass1_1020_cd06(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xcd7e;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



u16 pass1_1020_cd30(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((((iVar1 + 0x12) == 0x6) || ((iVar1 + 0x12) == 0x5)) && ((*(u8 *)(iVar1 + 0x1a) & 0x2) != 0x0))
  {
    return 0x1;
  }
  return 0x0;
}



StructD * pass1_1020_cd58(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_cde6(u16 *param_1)

{
  struct_1028_0954((astruct_180 *)param_1);
  *param_1 = 0xd004;
  ((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



u16 * pass1_1020_ce08(u16 param_1,astruct_179 *param_2,u16 param_3,i16 param_4,u32 param_5)

{
  pass1_1028_0982(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0xd004;
  param_2->field2_0x2 = 0x1020;
  return (u16 *)CONCAT22(param_3,param_2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ce32(u16 param_1,astruct_15 *param_2,u32 param_3)

{
  u16 uVar1;
  u16 uVar3;
  u32 uVar4;
  u32 *puVar5;
  astruct_67 *paVar6;
  u16 in_stack_0000fe84;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;
  i16 iVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  i16 iVar11;
  astruct_57 *paVar2;

  uVar3 = ((u32)param_3 >> 0x10);
  pass1_1028_09b8(param_2);
  uVar4 = pass1_1028_b4f2(param_2);
  uVar1 = (uVar4 >> 0x10);
  paVar2 = (astruct_57 *)CONCAT22(uVar3,uVar1);
  if (*(i32 *)((int)uVar4 + 0x200) != 0x8000002) {
    puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_1,0x8),in_stack_0000fe92,
                             in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
    paVar2 = (astruct_57 *)((u32)paVar2 & 0xffff0000 | (u32)puVar5 >> 0x10);
    pass1_1010_988c((u32)puVar5,((int)param_2 + 0xc));
    uVar10 = 0x0;
    iVar11 = 0x9;
    uVar8 = 0x1;
    uVar9 = 0x0;
    uVar3 = 0x0;
    iVar7 = 0x0;
    uVar1 = 0x0;
    paVar6 = (astruct_67 *)
             mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe84,in_stack_0000ffa8,
                             in_stack_0000ffae,in_stack_0000ffb2);
    post_win_msg_1008_a0e4(paVar6,CONCAT22(uVar3,uVar1),iVar7,uVar8,CONCAT22(uVar10,uVar9),iVar11);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ce9e(u16 param_1,astruct_15 *param_2)

{
  u16 uVar1;
  u32 in_EDX;
  u16 uVar2;
  u32 uVar3;
  astruct_27 *paVar4;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc0;

  uVar2 = ((u32)in_EDX >> 0x10);
  pass1_1028_be9e(param_2);
  if (((int)param_2 + 0x12) == 0x5) {
    pass1_1020_cefc(param_2);
    uVar3 = pass1_1028_b4f2(param_2);
    uVar1 = (uVar3 >> 0x10);
    if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
      paVar4 = (astruct_27 *)
               mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,uVar1),_u16_1050_0ed0,(u8 **)CONCAT22(param_1,0x2b),
                               in_stack_0000fe92,in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
      pass1_1010_043a(paVar4,*(i32 *)((int)uVar3 + 0x4),0x5);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_cefc(astruct_15 *param_1)

{
  u16 uVar1;
  u32 in_EDX;
  u16 uVar2;
  u32 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe94;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffec;
  u16 uStack12;

  uVar2 = ((u32)in_EDX >> 0x10);
  uVar3 = pass1_1028_b4f2(param_1);
  uVar1 = (uVar3 >> 0x10);
  if (*(i32 *)((int)uVar3 + 0x200) == 0x8000002) {
    uStack12 = 0x32;
  }
  else {
    puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,uVar1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000ffec,0x8),in_stack_0000fe94,in_stack_0000ffb8,
                             in_stack_0000ffbe,in_stack_0000ffc2);
    uStack12 = pass1_1010_96c2((u32)puVar4);
    if (0x32 < (int)uStack12) {
      uStack12 = 0x32;
    }
    pass1_1010_96a8((u32)puVar4,uStack12);
  }
  pass1_1020_cf6c(param_1,uStack12,uVar3);
  return;
}



void pass1_1020_cf6c(astruct_15 *param_1,i16 param_2,u32 param_3)

{
  u16 *puVar1;
  i16 *piVar2;
  u16 uVar3;
  u32 uVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  i16 iVar9;
  u16 uVar10;

  uVar10 = (param_3 >> 0x10);
  uVar4 = (u32)((int)param_3 + 0x1f6);
  iVar6 = (int)uVar4;
  uVar5 = ((u32)uVar4 >> 0x10);
  uVar7 = param_2 / 0x5;
  uVar8 = uVar7 * -0x4 + param_2;
  puVar1 = (u16 *)(iVar6 + 0x50);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar8;
  piVar2 = (i16 *)(iVar6 + 0x52);
  *piVar2 = *piVar2 + ((int)uVar8 >> 0xf) + CARRY2(uVar3,uVar8);
  iVar9 = (int)uVar7 >> 0xf;
  puVar1 = (u16 *)(iVar6 + 0x78);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (i16 *)(iVar6 + 0x7a);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  puVar1 = (u16 *)(iVar6 + 0xa0);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (i16 *)(iVar6 + 0xa2);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  puVar1 = (u16 *)(iVar6 + 0xc8);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (i16 *)(iVar6 + 0xca);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  puVar1 = (u16 *)(iVar6 + 0xf0);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (i16 *)(iVar6 + 0xf2);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  ((int)param_3 + 0x1fe) = 0x1;
  return;
}



StructD * pass1_1020_cfde(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_d06c(u16 *param_1)

{
  struct_1028_b354((astruct_180 *)param_1);
  *param_1 = 0xd314;
  ((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



u16 * pass1_1020_d08e(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xd314;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



void pass1_1020_d0b8(astruct_15 *param_1)

{
  u32 uVar1;
  i16 iVar2;

  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar1 = pass1_1028_b4f2(param_1);
  iVar2 = (int)uVar1;
  if (*(i32 *)(iVar2 + 0x200) != 0x8000002) {
    pass1_1028_cb04(param_1);
    if ((iVar2 == 0x0) || (pass1_1020_d194(param_1), iVar2 == 0x0)) {
      iVar2 = 0x6;
      goto LAB_1020_d10b;
    }
    pass1_1028_c952(param_1);
  }
  iVar2 = 0x5;
LAB_1020_d10b:
  pass1_1028_bdac(param_1,iVar2);
  return;
}



u16 pass1_1020_d118(i16 param_1,u16 param_2,u32 param_3,u16 *param_4,u32 param_5,u32 param_6)

{
  BOOL16 BVar1;
  u16 uVar2;
  u16 uVar3;

  uVar2 = param_3;
  uVar3 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar2,uVar3,param_4,param_6);
  if (param_1 == 0x5) {
    pass1_1028_c23e(0x5,param_2,uVar2,uVar3,param_4,param_5,param_6);
    if (param_1 != 0x0) {
      pass1_1028_c3aa(uVar2,uVar3,param_4,param_5,param_6);
      if (param_1 != 0x0) {
        BVar1 = pass1_1028_c314(param_1,param_2,uVar2,uVar3,param_4,param_5,(param_5 >> 0x10),param_6);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = (u8 *)0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_d194(astruct_15 *param_1)

{
  code **ppcVar1;
  u8 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u32 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u32 in_EDX;
  astruct_57 *paVar13;
  u16 uVar14;
  u32 uVar15;
  u32 *puVar16;
  u32 *puVar17;
  u16 in_stack_0000fe70;
  u16 in_stack_0000ff94;
  u16 in_stack_0000ff9a;
  u16 in_stack_0000ff9e;
  u8 uVar18;
  u8 uVar19;
  u16 uVar20;
  u16 uVar21;
  u32 uStack42;
  u32 uStack38;
  u32 *puStack34;
  u8 local_4 [0x2];
  astruct_57 *paVar12;

  uVar9 = ((u32)in_EDX >> 0x10);
  pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar15 = pass1_1028_b4f2(param_1);
  uVar8 = (uVar15 >> 0x10);
  paVar12 = (astruct_57 *)CONCAT22(uVar9,uVar8);
  uVar7 = (u32)((int)uVar15 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
  uVar8 = uVar7;
  paVar13 = paVar12;
  pass1_1028_b58e(param_1);
  uVar9 = SUB42(paVar13,0x0);
  puVar2 = local_4;
  pass1_1030_bd74(puVar2,&DAT_1050_1050,uVar7 & 0xffff | (long)paVar12 << 0x10,
                  (astruct_670 *)CONCAT22(uVar9,uVar8));
  if ((int)puVar2 < 0x0) {
    return;
  }
  if (0x1e < (int)puVar2) {
    uVar3 = 0x87;
    puVar16 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,(u8 **)0x870009,in_stack_0000fe70,in_stack_0000ff94,
                              in_stack_0000ff9a,in_stack_0000ff9e);
    uVar10 = ((u32)paVar13 >> 0x10);
    uVar3 = pass1_1010_65d0((u32)puVar16,uVar3);
    if (uVar3 == 0x0) {
      puVar17 = pass1_1008_c6fa(_u16_1050_06e0,0x15);
      paVar12 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)puVar17 >> 0x10));
      uVar4 = puVar17;
      uVar14 = SUB42(&u16_1050_1038,0x0);
      pass1_1038_4e78(uVar4,paVar12,uVar15,puVar17);
      uVar10 = SUB42(paVar12,0x0);
      puStack34 = (u32 *)CONCAT22(uVar10,uVar4);
      ppcVar1 = (code **)((int)*puStack34 + 0x10);
      paVar13 = paVar12;
      uVar5 = uVar4;
      uVar21 = uVar4;
      (**ppcVar1)((int)&u16_1050_1038,uVar4,uVar10);
      uStack38 = CONCAT22((int)paVar13,uVar5);
      uStack42 = 0x0;
      while( true ) {
        if (uStack38 <= uStack42) {
          if (puStack34 == NULL) {
            return;
          }
          ppcVar1 = (code **)*puStack34;
          (**ppcVar1)(uVar14,uVar4,(char)paVar12,0x1,uVar21,uVar10,puStack34,puStack34);
          return;
        }
        uVar18 = (u8)uVar8;
        uVar19 = (u8)(uVar8 >> 0x8);
        uVar15 = uStack38;
        uVar20 = uVar9;
        pass1_1030_1d58((u32)puStack34);
        uVar6 = uVar15;
        uVar11 = SUB42(paVar13,0x0);
        puVar2 = local_4;
        uVar14 = 0x1030;
        pass1_1030_bd74(puVar2,&DAT_1050_1050,uVar15 & 0xffff | (long)paVar13 << 0x10,
                        (astruct_670 *)CONCAT22(uVar20,CONCAT11(uVar19,uVar18)));
        if ((0x0 < (int)puVar2) && ((int)puVar2 < 0x1f)) break;
        uStack42 += 0x1;
      }
      if (puStack34 == NULL) {
        return;
      }
      ppcVar1 = (code **)*puStack34;
      (**ppcVar1)(0x1030,uVar4,(char)paVar12,0x1,uVar21,uVar10,puStack34,puStack34,uVar6,uVar11);
      return;
    }
  }
  return;
}



StructD * pass1_1020_d2ee(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_d37c(astruct_180 *param_1)

{
  u16 uVar1;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x20) = 0x0;
  param_1->field0_0x0 = 0xd53e;
  ((int)param_1 + 0x2) = 0x1020;
  return &param_1->field0_0x0;
}



u16 * pass1_1020_d3a4(u16 param_1,astruct_12 *param_2,u16 param_3,i16 param_4,u32 param_5)

{
  u16 uVar1;

  pass1_1028_b39e((StructD *)param_1,param_2,param_4,param_5);
  uVar1 = ((u32)param_2 >> 0x10);
  ((int)param_2 + 0x20) = param_3;
  param_2->field0_0x0 = 0xd53e;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



BOOL16 write_to_file_1020_d3d4(astruct_731 *param_1,u8 *param_2)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  BVar1 = write_to_file_1028_b5ec(param_1,(u32)param_2);
  if (BVar1 != 0x0) {
    local_c[0] = ((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 pass1_1020_d41a(BOOL16 param_1,u8 *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  BOOL16 BVar1;
  u16 local_4;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    ((int)param_3 + 0x20) = local_4;
    param_1 = 0x1;
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1020_d460(i16 param_1,u16 param_2,u32 *param_3,u16 *param_4,u32 param_5,u32 param_6)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  u32 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe90;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffe8;

  uVar1 = pass1_1028_bc90(param_1,param_2,param_3,param_4,param_5,param_6);
  if (uVar1 != 0x0) {
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_4230,param_2,_PTR_LOOP_1050_5b7c,
                            (_PTR_LOOP_1050_4230 + 0x16),0x11);
    paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,(int)((u32)uVar3 >> 0x10));
    PTR_LOOP_1050_5b80 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5b80);
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe8,0x3a),in_stack_0000fe90,
                             in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
    ((int)param_3 + 0x20) = ((int)puVar4 + 0xa);
    uVar1 = 0x1;
  }
  return uVar1;
}



void pass1_1020_d4ca(i16 param_1,astruct_15 *param_2)

{
  BOOL16 BVar1;
  u32 uVar2;
  u16 DX_REG;
  u16 uVar3;
  i16 iVar4;

  if (((int)param_2 + 0x20) == 0x2) {
    return;
  }
  pass1_1028_b58e(param_2);
  uVar2 = (u32)(param_1 + 0x2e);
  iVar4 = 0x63;
  uVar3 = DX_REG;
  pass1_1038_3fb0(uVar2);
  BVar1 = pass1_1030_25b2(uVar2 & 0xffff | (u32)uVar3 << 0x10,iVar4);
  if (BVar1 != 0x0) {
    return;
  }
  return;
}



StructD * pass1_1020_d518(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_d5a6(astruct_180 *param_1)

{
  struct_1028_b354(param_1);
  param_1->field0_0x0 = 0xd7fe;
  ((int)param_1 + 0x2) = 0x1020;
  return &param_1->field0_0x0;
}



u16 * pass1_1020_d5c8(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xd7fe;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_d5f2(i16 param_1,astruct_15 *param_2,u32 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u32 *puVar3;
  u16 DX_REG;
  u16 uVar4;
  u32 uVar5;
  u32 *puVar6;
  u32 *puVar7;
  u8 bStack55;
  u8 local_32 [0xa];
  u32 uStack40;
  u32 uStack36;
  u32 *puStack28;
  u32 local_1a;
  i16 iStack22;
  u16 uStack20;
  i16 iStack18;
  u16 uStack16;
  i16 iStack14;
  u32 local_c;
  i16 iStack8;
  i16 iStack6;
  u16 uStack4;

  pass1_1028_b58e(param_2);
  local_c = (u32)(param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = DX_REG;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_1;
  uStack4 = DX_REG;
  pass1_1028_bab6(iStack18,DX_REG,param_2);
  uVar2 = pass1_1030_2fac((astruct_598 *)CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack14 += 0x1;
  uStack20 = uVar2;
  if (iStack14 < (int)uVar2) {
    puVar7 = (u32 *)CONCAT22(0x1050,local_32);
    iStack22 = iStack14;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar3 = &local_1a;
    pass1_1030_64ce(puVar3,uVar4,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar3),
                    uVar5 & 0xffff | (u32)uVar4 << 0x10,puVar7);
    uStack40 = *puVar3;
    uVar4 = ((int)puVar3 + 0x2);
    bStack55 = (u8)(uStack40 >> 0x18);
    uVar2 = bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | (u32)uVar4 << 0x10);
      puVar6 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar4,uVar2),uVar2,uVar4);
      uVar2 = puVar6;
      ppcVar1 = (code **)((int)*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(uVar2,param_2,param_3);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_d6e6(i16 param_1,astruct_15 *param_2)

{
  code **ppcVar1;
  u32 *puVar2;
  u16 uVar3;
  u16 DX_REG;
  u16 uVar4;
  u32 uVar5;
  u32 *puVar6;
  u32 *puVar7;
  u8 bStack55;
  u8 local_32 [0xa];
  u32 uStack40;
  u32 uStack36;
  u32 *puStack28;
  u32 local_1a;
  i16 iStack22;
  u16 uStack20;
  i16 iStack18;
  u16 uStack16;
  i16 iStack14;
  u32 local_c;
  i16 iStack8;
  i16 iStack6;
  u16 uStack4;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_c = (u32)(param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = DX_REG;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_1;
  uStack4 = DX_REG;
  pass1_1028_bab6(iStack18,DX_REG,param_2);
  uStack20 = pass1_1030_2fac((astruct_598 *)CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack22 = iStack14 + 0x1;
  if (iStack22 < (int)uStack20) {
    puVar7 = (u32 *)CONCAT22(0x1050,local_32);
    iStack14 = iStack22;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(puVar2,uVar4,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                    uVar5 & 0xffff | (u32)uVar4 << 0x10,puVar7);
    uStack40 = *puVar2;
    uVar4 = ((int)puVar2 + 0x2);
    bStack55 = (u8)(uStack40 >> 0x18);
    uVar3 = bStack55;
    if (bStack55 != 0x0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | (u32)uVar4 << 0x10);
      puVar6 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar4,uVar3),uVar3,uVar4);
      if (((int)puVar6 + 0xc) == 0x6a) {
        ppcVar1 = (code **)((int)*puVar6 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



StructD * pass1_1020_d7d8(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_d866(astruct_180 *param_1)

{
  struct_1028_b354(param_1);
  param_1->field0_0x0 = 0xd8ec;
  ((int)param_1 + 0x2) = 0x1020;
  return &param_1->field0_0x0;
}



u16 * pass1_1020_d888(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xd8ec;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



void FUN_1020_d8b2(void)

{
  return;
}



void FUN_1020_d8b6(void)

{
  return;
}



void FUN_1020_d8ba(void)

{
  return;
}



void FUN_1020_d8be(void)

{
  return;
}



void FUN_1020_d8c2(void)

{
  return;
}



StructD * FUN_1020_d8c6(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2->address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1020_d954(astruct_180 *param_1)

{
  astruct_57 *in_EDX;
  astruct_180 *iVar1;
  u16 unaff_BP;
  u16 uVar1;
  u32 *puVar2;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  struct_1030_dc96(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  &iVar1[0x1].field_0x4 = 0x0;
  &iVar1[0x1].field_0x6 = 0x0;
  (u32)&iVar1[0x1].field_0x8 = 0x0;
  param_1->field0_0x0 = 0xe792;
  iVar1->field1_0x2 = 0x1020;
  puVar2 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2f),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  &iVar1[0x1].field_0x8 = (int)puVar2;
  &iVar1[0x1].field_0xa = (int)((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 * struct_1020_d99e(astruct_57 *param_1,astruct_12 *param_2,u16 param_3,i16 param_4,u32 param_5)

{
  astruct_12 *iVar1;
  u16 unaff_BP;
  u16 uVar1;
  u16 *puVar2;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  puVar2 = pass1_1030_dcc2(param_1,param_2,param_4,param_5);
  uVar1 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_12 *)param_2;
  iVar1->field23_0x24 = param_3;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  (u32)&iVar1[0x1].field1_0x2 = 0x0;
  param_2->field0_0x0 = 0xe792;
  iVar1->field1_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba((astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puVar2 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_BP,0x2f),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  iVar1[0x1].field1_0x2 = puVar3;
  &iVar1[0x1].field_0x4 = (int)((u32)puVar3 >> 0x10);
  iVar1->field12_0x10 = 0x49;
  return &param_2->field0_0x0;
}



void pass1_1020_d9fa(u16 param_1,astruct_15 *param_2)

{
  u16 DX_REG;

  if (((int)param_2 + 0xc) != 0x79) {
    pass1_1030_df0c(param_1,param_2);
    pass1_1028_b58e(param_2);
    pass1_1038_57dc((u32)(param_1 + 0x2e),0x1,0x2);
  }
  return;
}



void pass1_1020_da3c(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_da4e(u16 param_1,u32 *param_2,u16 *param_3,u32 param_4,u32 param_5)

{
  code **ppcVar1;
  u32 *puVar2;
  u16 uVar3;
  BOOL16 BVar4;
  u8 *DX_REG;
  u8 *puVar5;
  u8 *DX_REG_00;
  u16 uVar6;
  u32 uVar7;
  u32 uVar8;
  u16 uVar9;
  u16 uVar11;
  u16 *puVar10;
  u32 uVar12;
  u8 bStack31;
  u32 local_e;
  u16 uStack10;
  u16 uStack8;
  u32 uStack6;

  puVar2 = &local_e;
  pass1_1030_64ce(puVar2,param_1,_PTR_LOOP_1050_5740,param_3,param_5,(u32 *)CONCAT22(0x1050,puVar2));
  uStack6 = *puVar2;
  uVar6 = ((int)puVar2 + 0x2);
  bStack31 = (u8)(uStack6 >> 0x18);
  uVar3 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar6 << 0x10);
    uVar7 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,uVar3),uVar3,uVar6);
    uVar6 = (uVar7 >> 0x10);
    uVar3 = uVar7;
    if ((uVar3 + 0xc) == 0x10) {
      PTR_LOOP_1050_50ca = (u8 *)0x6a9;
      return;
    }
  }
  uVar9 = param_2;
  uVar11 = ((u32)param_2 >> 0x10);
  pass1_1028_c7b6(uVar6,uVar9,uVar11,param_3,param_5);
  uVar8 = (u32)param_2 & 0xffff | (u32)uVar11 << 0x10;
  ppcVar1 = (code **)((int)*param_2 + 0x60);
  puVar10 = param_3;
  uVar7 = param_4;
  uVar12 = param_5;
  uStack8 = uVar3;
  (**ppcVar1)();
  if (((uVar3 != 0x0) &&
      (puVar5 = DX_REG, pass1_1028_c23e(uVar3,DX_REG,uVar9,uVar11,param_3,param_4,param_5), uVar3 != 0x0
      )) && (BVar4 = pass1_1028_c314(uVar3,puVar5,uVar9,uVar11,param_3,param_4,(param_4 >> 0x10)
                                     ,param_5), BVar4 != 0x0)) {
    uVar6 = ((u32)param_3 >> 0x10);
    if (((((int)param_3 + 0x4) == 0x0) && (uStack8 != 0x4)) &&
       (ppcVar1 = (code **)((int)*param_2 + 0x5c),
       (**ppcVar1)(0x1028,param_2,param_3,param_4,param_5,uVar8,puVar10,uVar7,uVar12), puVar5 = DX_REG_00,
       BVar4 == 0x0)) {
      return;
    }
    uStack10 = ((int)param_3 + 0x4);
    if (uStack10 != 0x0) {
      pass1_1020_df10(uStack10,puVar5,(u32)param_2,(u16 *)((u32)param_3 & 0xffff | (u32)uVar6 << 0x10),param_5)
      ;
      return;
    }
    pass1_1020_deac(0x0,puVar5,(u32)param_2,(u16 *)((u32)param_3 & 0xffff | (u32)uVar6 << 0x10),param_5);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_db86(u16 param_1,u16 param_2,u16 *param_3,u32 param_4,i32 param_5)

{
  i16 iVar1;
  u8 *puVar2;
  u32 uVar3;
  u16 uVar4;
  u32 uVar5;
  u16 *puVar6;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (u32)(iVar1 + 0x10);
  puVar6 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,puVar6,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6af;
  }
  else {
    if (((int)puVar2 < 0x1f) || (((int)param_3 + 0x4) < 0x1)) {
      return;
    }
    PTR_LOOP_1050_50ca = (u8 *)0x6b6;
    PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_dc1c(astruct_15 *param_1,u16 *param_2)

{
  i16 iVar1;
  code **ppcVar2;
  u32 *puVar3;
  u16 uVar4;
  u16 uVar5;
  u32 uVar6;
  u32 *puVar7;
  u32 *puVar8;
  u8 bStack27;
  u8 local_a [0x4];
  u32 uStack6;

  puVar8 = (u32 *)CONCAT22(0x1050,local_a);
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uVar6 >> 0x10);
  puVar3 = (u32 *)uVar6;
  pass1_1030_64ce(puVar3,uVar5,_PTR_LOOP_1050_5740,param_2,uVar6 & 0xffff | (u32)uVar5 << 0x10,puVar8);
  uStack6 = *puVar3;
  uVar5 = ((int)puVar3 + 0x2);
  bStack27 = (u8)(uStack6 >> 0x18);
  uVar4 = bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar5 << 0x10);
    puVar7 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar4),uVar4,uVar5);
    iVar1 = ((int)puVar7 + 0xc);
    if (((iVar1 < 0x1) || (SBORROW2(iVar1,0x1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (code **)((int)*puVar7 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1020_dca8(u16 param_1,astruct_15 *param_2)

{
  u16 uVar1;
  u16 uVar2;
  u8 local_2e [0xe];
  u32 *puStack32;
  u16 uStack30;
  u16 uStack28;
  u16 uStack26;
  u16 uStack24;
  u16 uStack22;
  u16 uStack20;
  u16 uStack18;
  u32 local_10;
  u16 uStack12;
  u32 uStack10;
  u8 local_6 [0x2];
  i16 local_4;

  pass1_1028_c1f8((int)local_6,param_1,param_2,(u16 *)CONCAT22(0x1050,local_6),(u16 *)CONCAT22(0x1050,&local_4));
  uStack10 = pass1_1028_b58e(param_2);
  uVar1 = ((u32)uStack10 >> 0x10);
  local_10 = (u32)((int)uStack10 + 0xc);
  uStack12 = ((int)uStack10 + 0x10);
  puStack32 = &local_10;
  uStack18 = local_10;
  uStack20 = ((u32)local_10 >> 0x10);
  uStack24 = local_10 - 0x1;
  if ((int)uStack24 < 0x0) {
    uStack24 = 0x0;
  }
  uVar2 = local_4 - 0x1;
  uStack26 = local_10 + 0x1;
  if ((int)uVar2 < (int)(local_10 + 0x1)) {
    uStack26 = uVar2;
  }
  uStack28 = uStack20 - 0x1;
  if ((int)uStack28 < 0x0) {
    uStack28 = 0x0;
  }
  uStack30 = uStack20 + 0x1;
  if ((int)uVar2 < (int)(uStack20 + 0x1)) {
    uStack30 = uVar2;
  }
  uStack22 = uStack12;
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack12,uStack28,uStack24);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack28,uStack18);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack28,uStack26);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack20,uStack24);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack20,uStack26);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack30,uStack24);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack30,uStack18);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_2e),uStack22,uStack30,uStack26);
  pass1_1020_dc1c(param_2,(u16 *)CONCAT22(0x1050,local_2e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_de32(u8 *param_1,u32 param_2,u16 param_3)

{
  u16 uVar1;
  BOOL16 BVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  u16 uVar4;
  u32 *puVar5;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000fff0;

  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff0,0x5),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar1 = ((u32)puVar5 >> 0x10);
  ((int)puVar5 + 0x12) = param_3;
  uVar3 = uVar1;
  BVar2 = bring_win_to_top_1038_b72e(_PTR_LOOP_1050_5b7c,0x4);
  if (BVar2 == 0x0) {
    pass1_1038_af40(_PTR_LOOP_1050_4230,uVar3,_PTR_LOOP_1050_5b7c,
                    (_PTR_LOOP_1050_4230 + 0x16),0x4);
  }
  PTR_LOOP_1050_5b80 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5b80);
  uVar4 = (param_2 >> 0x10);
  ((int)param_2 + 0x24) = ((int)puVar5 + 0xa);
  if (((int)param_2 + 0x24) == 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6b2;
  }
  return;
}



BOOL16 pass1_1020_deac(i16 param_1,u8 *param_2,u32 param_3,u16 *param_4,i32 param_5)

{
  u16 uVar1;
  u16 uVar2;

  uVar1 = param_3;
  uVar2 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar1,uVar2,param_4,param_5);
  if (param_1 < 0x1) {
    return 0x0;
  }
  if (SBORROW2(param_1,0x1)) {
    return 0x0;
  }
  if (param_1 != 0x3 && 0x0 < param_1 + -0x2) {
    if (param_1 == 0x4) {
      pass1_1020_de32(param_2,param_3,0x4);
      if ((uVar1 + 0x24) == 0x6) {
        return 0x1;
      }
      return 0x0;
    }
    if (param_1 != 0x5) {
      return 0x0;
    }
  }
  (uVar1 + 0x24) = 0x1;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_df10(u16 param_1,u8 *param_2,u32 param_3,u16 *param_4,i32 param_5)

{
  u32 *puVar1;
  u16 uVar2;
  BOOL16 BVar3;
  u16 uVar4;
  u32 uVar5;
  u16 uVar6;
  u16 uVar7;
  u8 bStack31;
  u32 local_e;
  u32 uStack10;
  u16 uStack6;
  u16 uStack4;

  uStack4 = 0x0;
  uVar6 = param_3;
  uVar7 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar6,uVar7,param_4,param_5);
  uStack6 = param_1;
  if (param_1 == 0x0) {
    puVar1 = &local_e;
    pass1_1030_64ce(puVar1,param_2,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,puVar1));
    uStack10 = *puVar1;
    uVar4 = ((int)puVar1 + 0x2);
    bStack31 = (u8)(uStack10 >> 0x18);
    uVar2 = bStack31;
    if (bStack31 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10 & 0xffff | (u32)uVar4 << 0x10);
      uVar5 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar4,uVar2),uVar2,uVar4);
      if (((int)uVar5 + 0xc) == 0x6a) {
        BVar3 = pass1_1020_e044(param_3);
        if (BVar3 == 0x0) {
          (uVar6 + 0x24) = 0x1;
        }
        else {
          PTR_LOOP_1050_50ca = (u8 *)0x6ac;
        }
      }
    }
  }
  else if (((0x5 < (int)param_1) && (!SBORROW2(param_1,0x6))) && ((int)(param_1 - 0x6) < 0x4)) {
    pass1_1020_de32(param_2,param_3,param_1);
    switch((uVar6 + 0x24)) {
    case 0x1:
      BVar3 = pass1_1020_e044(param_3);
      if (BVar3 != 0x0) {
        PTR_LOOP_1050_50ca = (u8 *)0x6ac;
      }
      break;
    case 0x2:
    case 0x3:
    case 0x4:
    case 0x5:
      pass1_1020_e652(param_3,(u32 *)param_4,((u32)param_4 >> 0x10),param_5);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1020_e044(u32 param_1)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  astruct_598 *paVar4;

  uVar3 = (param_1 >> 0x10);
  paVar4 = (astruct_598 *)pass1_1018_04b8((u32)((int)param_1 + 0x28));
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)paVar4);
  uVar2 = pass1_1030_2fac(paVar4);
  uVar1 = (u32)((int)param_1 + 0x28);
  if ((int)uVar2 <= ((int)uVar1 + 0x1e)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_e08e(u16 param_1,astruct_15 *param_2)

{
  u16 uVar1;
  u32 uVar2;
  i16 iVar3;
  i16 iVar4;
  u32 uVar5;
  u32 uVar6;
  u16 DX_REG;
  u16 uVar7;
  astruct_15 *pstruct15_7;
  astruct_15 *pstruct15_7_seg;
  i16 *piVar8;
  u16 uVar9;
  u16 *puVar10;
  u16 uVar11;
  u16 uVar12;
  u16 local_158;
  u16 uStack342;
  u32 uStack50;
  u32 *puStack42;
  i16 local_22;
  u8 local_20 [0x2];
  u8 local_1e [0x2];
  u16 uStack28;
  i16 *piStack26;
  i16 local_18;
  u16 local_16;
  u32 uStack20;
  u32 local_10;
  i16 iStack12;
  u32 uStack10;
  u32 uStack6;

  pstruct15_7_seg = (astruct_15 *)((u32)param_2 >> 0x10);
  pstruct15_7 = (astruct_15 *)param_2;
  iVar3 = pstruct15_7->field10_0xc;
  if (iVar3 == 0x74) {
    uVar1 = (pstruct15_7 + 0x1)->field0_0x0;
    iVar3 = uVar1 - 0x1;
    if (iVar3 == 0x0) goto LAB_1020_e0ae;
    iVar3 = uVar1 - 0x6;
    if (iVar3 != 0x0) goto LAB_1020_e0b9;
    uVar11 = 0x1;
  }
  else if (iVar3 == 0x78) {
    uVar1 = (pstruct15_7 + 0x1)->field0_0x0;
    iVar4 = uVar1 - 0x1;
    if (iVar4 != 0x0) {
      iVar3 = uVar1 - 0x2;
      if ((0x0 < iVar4) && (!SBORROW2(iVar4,0x1))) {
        if (uVar1 - 0x5 == 0x0 || iVar3 < 0x3) {
          iVar3 = uVar1 - 0x5;
          pass1_1020_e49a(param_2);
        }
        else {
          iVar3 = uVar1 - 0x6;
          if (iVar3 == 0x0) {
            pass1_1020_e39c(0x0,param_2,0x6);
            pass1_1020_dca8(param_1,param_2);
          }
        }
      }
      goto LAB_1020_e0b9;
    }
    uVar11 = 0x6a;
    iVar3 = iVar4;
  }
  else {
    iVar3 += -0x79;
    if (iVar3 == 0x0) {
      pass1_1020_e49a(param_2);
      return;
    }
LAB_1020_e0ae:
    uVar11 = 0x47;
  }
  pass1_1020_e39c(iVar3,param_2,uVar11);
LAB_1020_e0b9:
  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(DX_REG,iVar3);
  uVar5 = (u32)(iVar3 + 0x2e);
  uVar7 = (iVar3 + 0x30);
  uStack10 = uVar5;
  if (pstruct15_7->field10_0xc != 0x79) {
    pass1_1038_5804(uVar5 & 0xffff | (u32)uVar7 << 0x10,0x1,0x2);
  }
  if ((pstruct15_7 + 0x1)->field0_0x0 == 0x6) {
    pass1_1038_43cc((int)uVar5,uVar7,(int)uStack10,(int)(uStack10 >> 0x10),0x1,0x2);
  }
  local_10 = (u32)((int)uStack6 + 0xc);
  iStack12 = ((int)uStack6 + 0x10);
  puStack42 = &local_10;
  if (((pstruct15_7 + 0x1)->field0_0x0 == 0x6) && (iStack12 == 0x0)) {
    return;
  }
  uVar2 = (u32)&pstruct15_7[0x1].field_0x4;
  uVar6 = (u32)((int)uVar2 + 0x20);
  puVar10 = &local_16;
  uVar12 = SUB42(&DAT_1050_1050,0x0);
  piStack26 = &local_18;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  piVar8 = piStack26;
  uStack20 = uVar6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
  uStack28 = uVar6;
  pass1_1030_5b1c(uVar6 & 0xffff | ZEXT24(piStack26) << 0x10,(u16 *)CONCAT22(uVar9,piVar8),
                  (u16 *)CONCAT22(uVar12,puVar10));
  pass1_1028_c8ee(param_2,(pstruct15_7 + 0x1)->field0_0x0,(u16 *)CONCAT22(0x1050,&local_10));
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,&local_22),
                  (u16 *)CONCAT22(0x1050,local_20),(u16 *)CONCAT22(0x1050,local_1e));
  if ((pstruct15_7 + 0x1)->field0_0x0 == 0x1) {
    if (local_18 < local_22) {
      pass1_1030_5b3e(CONCAT22(piStack26,uStack28),local_22,local_16);
      pass1_1030_5b1c(CONCAT22(piStack26,uStack28),(u16 *)CONCAT22(0x1050,&local_18),
                      (u16 *)CONCAT22(0x1050,&local_16));
    }
    uStack50 = (u32)((int)uStack10 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_158),0x0,0x0,0x6a,&local_10,&DAT_1050_1050,uStack50
                        ,uStack20);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_158));
    local_158 = 0x389a;
    uStack342 = 0x1008;
  }
  pass1_1028_ccd0(param_2,(u16 *)CONCAT22(0x1050,&local_10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_e294(astruct_15 *param_1)

{
  u32 uVar1;
  u32 *puVar2;
  u32 uVar3;
  u16 DX_REG;
  u16 uVar4;
  astruct_15 *pstruct15_5;
  astruct_15 *uVar6;
  char cStack347;
  u8 local_150 [0xc];
  u32 *puStack324;
  char string_140 [0x124];
  i16 iStack20;
  u32 local_10;
  u16 uStack12;
  i16 iStack10;
  u16 uStack8;
  u32 uStack6;

  uVar6 = (astruct_15 *)((u32)param_1 >> 0x10);
  pstruct15_5 = (astruct_15 *)param_1;
  if ((0x1 < (int)(pstruct15_5 + 0x1)->field0_0x0) && ((int)(pstruct15_5 + 0x1)->field0_0x0 < 0x6)) {
    uVar1 = (u32)&pstruct15_5[0x1].field_0x4;
    uVar3 = (u32)((int)uVar1 + 0x20);
    uStack6 = uVar3;
    pass1_1028_b58e(param_1);
    iStack10 = (int)uVar3;
    local_10 = (u32)(iStack10 + 0xc);
    uStack12 = (iStack10 + 0x10);
    puStack324 = &local_10;
    uVar4 = DX_REG;
    uStack8 = DX_REG;
    pass1_1028_c8ee(param_1,(pstruct15_5 + 0x1)->field0_0x0,(u16 *)CONCAT22(0x1050,puStack324));
    puVar2 = &local_10;
    pass1_1028_c89c((int)puVar2,param_1,(u16 *)CONCAT22(0x1050,puVar2),(u32 *)CONCAT22(0x1050,local_150));
    uVar3 = *puVar2;
    cStack347 = (char)(uVar3 >> 0x18);
    if ((cStack347 == '\0') && (iStack20 = (int)uVar3, iStack20 == 0x9)) {
      &pstruct15_5->field16_0x14 = 0x1;
    }
    uVar1 = (u32)(iStack10 + 0x2e);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,string_140),0x0,&pstruct15_5->field16_0x14 + 0x1,0x79,
                        &local_10,&DAT_1050_1050,(u32)((int)uVar1 + 0x4),uStack6);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,string_140));
  }
  pstruct15_5[0x1].field1_0x2 = 0x1;
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_e39c(i16 param_1,astruct_15 *param_2,u16 param_3)

{
  u16 uVar3;
  u16 DX_REG;
  char local_13c [0x124];
  u32 local_c;
  i16 iStack8;
  u32 uStack6;
  u32 uVar1;
  u32 uVar2;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(DX_REG,param_1);
  local_c = (u32)(param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  if (iStack8 < 0x1) {
    uVar3 = 0x5;
  }
  else {
    uVar3 = 0x6;
  }
  (param_1 + 0x14) = uVar3;
  uVar2 = (u32)((int)param_2 + 0x28);
  uVar1 = (u32)(param_1 + 0x2e);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_13c),0x0,0x1,param_3,&local_c,&DAT_1050_1050,
                      (u32)((int)uVar1 + 0x4),(u32)((int)uVar2 + 0x20));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13c));
  return;
}



void pass1_1020_e44c(undefined1 param_1,u16 param_2,u32 param_3)

{
  i16 *piVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_3 >> 0x10);
  iVar2 = (int)param_3;
  if ((iVar2 + 0x12) == 0x2) {
    piVar1 = (i16 *)(iVar2 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if (((iVar2 + 0x26) == 0x0) && ((iVar2 + 0xc) == 0x78)) {
      pass1_1020_e294((astruct_15 *)(param_3 & 0xffff | (u32)uVar3 << 0x10));
    }
    if ((iVar2 + 0x14) == 0x0) {
      pass1_1020_e08e(param_2,(astruct_15 *)(param_3 & 0xffff | (u32)uVar3 << 0x10));
      return;
    }
    if ((iVar2 + 0x24) == 0x6) {
      (iVar2 + 0xe) = 0x49;
    }
  }
  return;
}



void pass1_1020_e49a(astruct_15 *param_1)

{
  i16 iVar1;
  i16 iVar2;
  u32 uVar3;
  u16 uStack10;

  uVar3 = pass1_1028_b58e(param_1);
  iVar1 = ((int)uVar3 + 0x14);
  uStack10 = 0x0;
  iVar2 = iVar1 + -0x6;
  if (iVar2 == 0x0) {
    uStack10 = 0x9;
  }
  else {
    iVar2 = iVar1 + -0x7;
    if (iVar2 == 0x0) {
      uStack10 = 0x6;
    }
    else {
      iVar2 = iVar1 + -0x8;
      if (iVar2 == 0x0) {
        uStack10 = 0x7;
      }
      else {
        iVar2 = iVar1 + -0x9;
        if (iVar2 == 0x0) {
          uStack10 = 0x8;
        }
      }
    }
  }
  pass1_1020_e39c(iVar2,param_1,uStack10);
  return;
}



i16 pass1_1020_e4fa(astruct_15 *param_1,u16 param_2)

{
  u32 uVar1;
  i16 iStack4;

  switch(param_2) {
  case 0x2:
  case 0x5:
  case 0x6:
  case 0x7:
    iStack4 = 0x4;
    break;
  case 0x3:
  case 0x8:
    iStack4 = 0x5;
    break;
  default:
    uVar1 = pass1_1028_b58e(param_1);
    iStack4 = ((int)uVar1 + 0x14) + 0x2;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_e558(i16 param_1,astruct_15 *param_2)

{
  u32 *puVar1;
  u16 uVar2;
  i16 iVar3;
  u16 DX_REG;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u8 bStack45;
  u8 local_24 [0xc];
  u32 uStack24;
  u32 uStack20;
  u32 local_10;
  u16 uStack12;
  i16 iStack10;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  uVar7 = ((u32)param_2 >> 0x10);
  iVar6 = (int)param_2;
  if ((iVar6 + 0xc) == 0x79) {
    param_1 = (iVar6 + 0x24);
    (iVar6 + 0x14) = param_1;
    (iVar6 + 0x24) = 0x0;
  }
  if ((iVar6 + 0x24) != 0x6) {
    pass1_1028_b58e(param_2);
    uStack8 = (param_1 + 0x14);
    iStack6 = param_1;
    uStack4 = DX_REG;
    iStack10 = pass1_1020_e4fa(param_2,uStack8);
    local_10 = (u32)(iStack6 + 0xc);
    uStack12 = (iStack6 + 0x10);
    uStack24 = CONCAT22(uStack24,&local_10);
    uVar4 = uStack4;
    pass1_1028_c8ee(param_2,(iVar6 + 0x24),(u16 *)CONCAT22(0x1050,&local_10));
    puVar1 = &local_10;
    pass1_1028_c89c((int)puVar1,param_2,(u16 *)CONCAT22(0x1050,puVar1),(u32 *)CONCAT22(0x1050,local_24));
    uStack24 = *puVar1;
    uVar5 = ((int)puVar1 + 0x2);
    bStack45 = (u8)(uStack24 >> 0x18);
    uVar2 = bStack45;
    uStack20._0_2_ = uStack24;
    uStack20 = uStack24;
    if (bStack45 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack24 & 0xffff | (u32)uVar5 << 0x10);
      uStack20._0_2_ = (uVar2 + 0x14);
    }
    uStack8 = uStack20;
    iVar3 = pass1_1020_e4fa(param_2,uStack20);
    (iVar6 + 0x14) = iStack10 + iVar3;
    return;
  }
  (iVar6 + 0x14) = 0x1;
  return;
}



u32 * pass1_1020_e652(u32 param_1,u32 *param_2,u16 param_3,i32 param_4)

{
  u32 *puVar1;
  u16 uVar2;
  u32 local_8;
  u16 uStack4;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  uVar2 = (param_1 >> 0x10);
  pass1_1028_c8ee((astruct_15 *)param_1,(param_1 + 0x24),(u16 *)CONCAT22(0x1050,&local_8));
  puVar1 = &local_8;
  pass1_1028_c7b6(param_3,param_1,uVar2,(u16 *)CONCAT22(0x1050,puVar1),param_4);
  if (puVar1 != NULL) {
    puVar1 = (u32 *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  return puVar1;
}



BOOL16 write_to_file_1020_e6a4(u32 param_1,u8 *param_2)

{
  i16 in_AX;
  BOOL16 BVar1;
  u16 uVar2;
  HFILE16 in_stack_0000ffdc;
  u16 local_c [0x3];
  u16 local_6 [0x2];

  pass1_1030_de7c(param_1,(u32)param_2);
  if (in_AX != 0x0) {
    uVar2 = (param_1 >> 0x10);
    local_c[0] = ((int)param_1 + 0x24);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffdc);
    if (BVar1 != 0x0) {
      local_6[0] = ((int)param_1 + 0x26);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffdc);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



void pass1_1020_e70e(i16 param_1,u8 *param_2,u32 param_3,u32 param_4)

{
  BOOL16 BVar1;

  pass1_1030_dec4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)((int)param_3 + 0x24)),0x2);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)((int)param_3 + 0x26)),0x2);
      if (BVar1 != 0x0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



StructD * pass1_1020_e76c(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u16 * struct_1020_e7fa(astruct_180 *param_1)

{
  struct_1028_b354(param_1);
  param_1->field0_0x0 = 0xe88e;
  ((int)param_1 + 0x2) = 0x1020;
  return &param_1->field0_0x0;
}



u16 * pass1_1020_e81c(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xe88e;
  ((int)param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}



void pass1_1020_e846(u16 *param_1)

{
  *param_1 = 0xe88e;
  ((int)param_1 + 0x2) = 0x1020;
  pass1_1028_b418(param_1);
  return;
}



void FUN_1020_e864(void)

{
  return;
}



StructD * pass1_1020_e868(StructD *param_1,u8 param_2)

{
  pass1_1020_e846(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1020_e8f6(u16 *param_1)

{
  u16 uVar1;

  struct_1030_dc96((astruct_180 *)param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x24) = 0x0;
  *param_1 = 0xeef6;
  ((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



u16 * pass1_1020_e91e(u16 param_1,i16 param_2,u16 param_3,i16 param_4,u32 param_5)

{
  pass1_1030_dcc2(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  (param_2 + 0x24) = 0x0;
  CONCAT22(param_3,param_2) = 0xeef6;
  (param_2 + 0x2) = 0x1020;
  return (u16 *)CONCAT22(param_3,param_2);
}



BOOL16 pass1_1020_e94e(BOOL16 param_1,u32 param_2,u32 param_3)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  pass1_1030_de7c(param_2,param_3);
  if (param_1 != 0x0) {
    local_c[0] = ((int)param_2 + 0x24);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    param_1 = 0x1;
  }
  return param_1;
}



void pass1_1020_e994(i16 param_1,u8 *param_2,u32 param_3,u32 param_4)

{
  BOOL16 BVar1;

  pass1_1030_dec4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if ((param_1 != 0x0) &&
     (BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)((int)param_3 + 0x24)),0x2),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}



void pass1_1020_e9d4(u16 param_1,astruct_15 *param_2)

{
  u16 DX_REG;

  pass1_1030_df0c(param_1,param_2);
  pass1_1028_b58e(param_2);
  pass1_1038_57dc((u32)(param_1 + 0x2e),0x1,0x1);
  return;
}



void pass1_1020_ea0e(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ea20(u16 param_1,u16 param_2,u32 param_3,u16 *param_4,u32 param_5,u32 param_6)

{
  u16 uVar1;
  code **ppcVar2;
  char cVar3;
  u32 *puVar4;
  u16 uVar5;
  u16 uVar6;
  u8 *puVar7;
  u16 in_register_0000000a;
  astruct_57 *paVar8;
  u32 uVar9;
  u32 uVar10;
  u16 uVar11;
  u16 in_stack_0000fd5e;
  u16 in_stack_0000fe82;
  u16 in_stack_0000fe88;
  u16 in_stack_0000fe8c;
  u16 uVar12;
  u16 uVar13;
  i16 iVar14;
  u8 local_146 [0x10c];
  u16 uStack58;
  u16 uStack56;
  astruct_419 *paStack50;
  u32 *puStack46;
  u32 *puStack42;
  u32 uStack38;
  u32 uStack34;
  u32 uStack28;
  u32 local_12;
  i16 iStack14;
  u32 *puStack12;
  u32 uStack8;
  BOOL16 BStack4;

  uVar12 = param_3;
  uVar13 = (param_3 >> 0x10);
  pass1_1028_c3aa(uVar12,uVar13,param_4,param_5,param_6);
  if (param_1 == 0x0) {
    return;
  }
  pass1_1028_c23e(param_1,param_2,uVar12,uVar13,param_4,param_5,param_6);
  if (param_1 == 0x0) {
    return;
  }
  BStack4 = pass1_1028_c314(param_1,param_2,uVar12,uVar13,param_4,param_5,(param_5 >> 0x10),param_6);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_2,uVar12,uVar13,param_4,param_6);
  if ((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9)) {
    PTR_LOOP_1050_50ca = (u8 *)0x6a8;
    return;
  }
  if (BStack4 != 0x0) {
    return;
  }
  puVar4 = &local_12;
  pass1_1030_64ce(puVar4,param_2,_PTR_LOOP_1050_5740,param_4,param_6,(u32 *)CONCAT22(0x1050,puVar4));
  uStack38 = *puVar4;
  uVar1 = ((int)puVar4 + 0x2);
  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar1);
  uStack38._3_1_ = (u8)(uStack38 >> 0x18);
  uStack58 = uStack38._3_1_;
  uStack28 = uStack38;
  uStack8 = uStack38;
  if (uStack38._3_1_ == 0x0) goto LAB_1020_eb4e;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack38 & 0xffff | (u32)uVar1 << 0x10);
  uVar11 = SUB42(paVar8,0x0);
  uStack38 = CONCAT22(uVar11,uStack58);
  uStack34 = (u32)(uStack58 + 0x2e);
  if ((u32)((int)uStack34 + 0x4) != param_5) {
    PTR_LOOP_1050_50ca = (u8 *)0x6b7;
    return;
  }
  uStack28 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar11,uStack58),param_5,uVar11);
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | uStack28 >> 0x10);
  uVar11 = (uStack28 >> 0x10);
  uVar1 = ((int)uStack28 + 0xc);
  uStack58 = uVar1;
  if (uVar1 != 0x41) {
    if (0x41 < (int)uVar1) {
      if (uVar1 == 0x6b) {
        PTR_LOOP_1050_50ca = (u8 *)0x6b1;
        return;
      }
      if ((int)uVar1 < 0x6c) {
        if (uVar1 == 0x42) {
          PTR_LOOP_1050_50ca = (u8 *)0x6b1;
          return;
        }
        uStack58 = uVar1 - 0x4b;
        if (uStack58 == 0x0) {
          PTR_LOOP_1050_50ca = (u8 *)0x6b1;
          return;
        }
      }
      else {
        if (uVar1 == 0x6e) {
          return;
        }
        uStack58 = uVar1 - 0x73;
        if ((0x4 < (int)(uVar1 - 0x6e)) && (uStack58 = uVar1 - 0x79, uStack58 == 0x0 || (int)(uVar1 - 0x73) < 0x6)) {
          PTR_LOOP_1050_50ca = (u8 *)0x6b0;
          return;
        }
      }
      goto LAB_1020_eb4e;
    }
    if (uVar1 != 0x3e) {
      if (uVar1 < 0x3f) {
        cVar3 = (char)uVar1;
        if (cVar3 != '\v') {
          if (cVar3 == '\x10') {
            return;
          }
          uStack58 = uVar1 & 0xff00 | (u8)(cVar3 - 0x37U);
          if ((u8)(cVar3 - 0x37U) != 0x0) goto LAB_1020_eb4e;
        }
        PTR_LOOP_1050_50ca = (u8 *)0x6b4;
        return;
      }
      goto LAB_1020_eb4e;
    }
  }
  if (((int)uStack28 + 0x12) == 0x4) {
    PTR_LOOP_1050_50ca = (u8 *)0x6b1;
    return;
  }
LAB_1020_eb4e:
  uVar11 = 0x1000;
  mem_op_1000_179c(0xb4,paVar8);
  uStack56 = paVar8;
  uVar10 = (u32)paVar8 & 0xffff0000;
  uVar9 = uVar10 | (uStack56 | uStack58);
  if ((uStack56 | uStack58) == 0x0) {
    iStack14 = 0x0;
  }
  else {
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    iStack14 = string_1040_8520(uVar9,(astruct_57 *)
                                      CONCAT13((char)((u32)paVar8 >> 0x8),CONCAT12((char)paVar8,uStack58)),
                                HWND16_1050_0396,0x20024,0x5e8057b);
    uVar10 = uVar9;
  }
  puStack12 = (u32 *)CONCAT22((int)uVar10,iStack14);
  ppcVar2 = (code **)((int)*puStack12 + 0x74);
  iVar14 = iStack14;
  (**ppcVar2)(uVar11,iStack14,(int)uVar10);
  if (iStack14 != 0x7) {
    paVar8 = (astruct_57 *)(uVar10 & 0xffff0000 | uStack8 >> 0x10);
    puStack46 = (u32 *)uStack8;
    uStack34 = uStack8;
    uStack34._3_1_ = (u8)(uStack8 >> 0x18);
    uVar5 = uStack34._3_1_;
    if (uStack34._3_1_ != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack8);
      paStack50 = (astruct_419 *)CONCAT22((int)paVar8,uVar5);
      fn_ptr_1030_7296((astruct_292 *)CONCAT22((int)paVar8,uVar5));
      pass1_1030_730a(uVar5,(astruct_290 *)paStack50);
      puStack46 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(iVar14,0x2f),in_stack_0000fd5e,
                                  in_stack_0000fe82,in_stack_0000fe88,in_stack_0000fe8c);
      uVar6 = ((u32)puStack46 >> 0x10);
      uVar11 = SUB42(puStack46,0x0);
      pass1_1010_ec68((u32)puStack46,(u32)paStack50);
      puStack42 = (u32 *)struct_op_1030_73a8(paStack50,uVar11,uVar6);
      puVar7 = (u8 *)((u32)puStack42 >> 0x10);
      puVar4 = (u32 *)puStack42;
      ppcVar2 = (code **)((int)*puStack42 + 0x24);
      (**ppcVar2)(0x1030,(char)puStack42,puVar7);
      uVar5 = pass1_1028_bc4a(puVar4,puVar7,(u32)puStack42);
      (uVar12 + 0x24) = uVar5;
      struct_1030_e4fa((astruct_97 *)CONCAT22(0x1050,local_146),(u32)((int)paStack50 + 0x16));
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_146));
    }
    return;
  }
  PTR_LOOP_1050_50ca = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ecb0(i16 param_1,u16 param_2,u32 param_3)

{
  i16 iVar1;
  u16 uVar2;
  u16 uStack8;

  uVar2 = (param_3 >> 0x10);
  iVar1 = (int)param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar1 + 0x8));
  if ((iVar1 + 0x12) == 0x1) {
    switch((param_1 + 0x14)) {
    case 0x2:
    case 0x7:
      uStack8 = 0x2;
      break;
    case 0x3:
    case 0x8:
      uStack8 = 0x3;
      break;
    default:
      uStack8 = (param_1 + 0x14);
      break;
    case 0x5:
    case 0x6:
      uStack8 = 0x1;
    }
    (iVar1 + 0x14) = uStack8;
    return;
  }
  pass1_1028_bf22((u8 *)param_2,param_3 & 0xffff | (u32)uVar2 << 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ed3c(i16 param_1,astruct_15 *param_2)

{
  StructD **ppSVar1;
  u32 uVar2;
  u16 uVar3;
  i16 iVar4;
  u16 DX_REG;
  u16 DX_REG_00;
  astruct_15 *pstruct15_5_1;
  astruct_15 *pstruct15_5;
  char local_138 [0x112];
  i16 local_12;
  u8 local_10 [0x2];
  u8 local_e [0x2];
  u32 local_c;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  pstruct15_5 = (astruct_15 *)((u32)param_2 >> 0x10);
  pstruct15_5_1 = (astruct_15 *)param_2;
  ppSVar1 = &pstruct15_5_1->field16_0x14;
  ppSVar1 = ppSVar1 + -0x1;
  if (ppSVar1 == 0x0) {
    pstruct15_5_1->field15_0x12 = 0x0;
    pass1_1028_b58e(param_2);
    local_c = (u32)(param_1 + 0xc);
    uStack8 = (param_1 + 0x10);
    iStack6 = param_1;
    uStack4 = DX_REG;
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_12),
                    (u16 *)CONCAT22(0x1050,local_10),(u16 *)CONCAT22(0x1050,local_e));
    if (local_12 < 0x1) {
      uVar3 = 0x5;
    }
    else {
      uVar3 = 0x6;
    }
    (iStack6 + 0x14) = uVar3;
    if (local_12 < 0x1) {
      iVar4 = 0x5;
    }
    else {
      iVar4 = 0x9;
    }
    pass1_1020_ee3a(iVar4,param_2,iVar4);
    pass1_1028_b58e(param_2);
    uVar2 = (u32)(iVar4 + 0x2e);
    pass1_1038_5804(uVar2,0x1,0x1);
    if (0x0 < (int)(pstruct15_5_1 + 0x1)->field0_0x0) {
      pass1_1028_68de((astruct_97 *)CONCAT22(0x1050,local_138),(pstruct15_5_1 + 0x1)->field0_0x0,
                      (u32)((int)uVar2 + 0x4));
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_138));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1020_ee3a(i16 param_1,astruct_15 *param_2,u16 param_3)

{
  u32 uVar1;
  u16 DX_REG;
  u32 uVar2;
  char local_13c [0x124];
  u32 local_c;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  pass1_1028_b58e(param_2);
  local_c = (u32)(param_1 + 0xc);
  uStack8 = (param_1 + 0x10);
  iStack6 = param_1;
  uStack4 = DX_REG;
  uVar2 = pass1_1028_bb24(param_2);
  uVar1 = (u32)(iStack6 + 0x2e);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_13c),0x0,0x1,param_3,&local_c,&DAT_1050_1050,
                      (u32)((int)uVar1 + 0x4),uVar2);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13c));
  return;
}



StructD * pass1_1020_eed0(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



void pass1_1020_ef5e(u16 *param_1)

{
  *param_1 = 0x0;
  ((int)param_1 + 0x2) = 0x1028;
  pass1_1028_b418(param_1);
  return;
}



void FUN_1020_ef7c(void)

{
  return;
}



void FUN_1020_ef80(void)

{
  return;
}



void FUN_1020_ef84(void)

{
  return;
}



void FUN_1020_ef88(void)

{
  return;
}



void FUN_1020_ef8c(void)

{
  return;
}



void FUN_1020_ef90(void)

{
  return;
}



StructD * pass1_1020_ef94(StructD *param_1,u8 param_2)

{
  pass1_1020_ef5e(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}

