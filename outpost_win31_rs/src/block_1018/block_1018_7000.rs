

u16 pass1_1018_6048(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}
pub fn set_window_text_1018_6066(param_1: *mut astruct_937,mut param_2: u16 ,char *in_win_text_3,INT16 dialog_id_5)

{
  let mut hwnd: HWND16;

  hwnd = GetDlgItem16(dialog_id_5,param_1->hwnd_field_0x6);
  SetWindowText16(in_win_text_3,hwnd);
  return;
}
pub fn set_window_text_1018_6086(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  let mut hwnd_1: HWND16;
  let mut uVar2: u16;

  wsprintf16((WORD *)&stack0xfff4,0x42421050,CONCAT22(param_3,0x1050));
  uVar2 = (param_1 >> 0x10);
  hwnd_1 = GetDlgItem16(0x1be,*(HWND16 *)(param_1 + 0x6));
  SetWindowText16(CONCAT22(0x1050,&stack0xfff4),hwnd_1);
  wsprintf16((WORD *)&stack0xfff4,0x42451050,CONCAT22(param_2,0x1050));
  hwnd_1 = GetDlgItem16(0x1bf,*(HWND16 *)(param_1 + 0x6));
  SetWindowText16(CONCAT22(0x1050,&stack0xfff4),hwnd_1);
  return;
}
pub fn FUN_1018_60ea(void)

{
  return;
}



u16 FUN_1018_60ee(void)

{
  return 0x0;
}



u16 FUN_1018_60f4(void)

{
  return 0x0;
}
pub fn FUN_1018_60fa(void)

{
  return;
}
pub fn FUN_1018_60fe(void)

{
  return;
}



StructD * pass1_1018_6102(StructD *param_1,param_2: u8)

{
  pass1_1018_5e5a(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_6198(u8 *param_1,param_2: *mut astruct_657,StructB *param_3,mut param_4: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_657 *iVar1;
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffec: u32;

  uVar1 = (param_2 >> 0x10);
  iVar1 = (astruct_657 *)param_2;
  param_2 = 0x389a;
  iVar1->field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar1->field2_0x2 = 0x1008;
  iVar1->field3_0x4 = param_4;
  param_2 = 0x3ab0;
  iVar1->field2_0x2 = 0x1008;
  &iVar1->field4_0x6 = 0x0;
  iVar1->field6_0xa = param_3;
  param_2 = 0x66c0;
  iVar1->field2_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((in_stack_0000ffec >> 0x10),0x39),in_stack_0000fe96,
                           in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  iVar1->field4_0x6 = puVar2;
  iVar1->field5_0x8 = (puVar2 >> 0x10);
  return;
}
pub fn pass1_1018_620c(StructD *struct_param_1)

{
  StructD *struct_1;
  let mut struct_1_lo: u16;

  struct_1_lo = (struct_param_1 >> 0x10);
  struct_1 = struct_param_1;
  struct_param_1->address_offset_field_0x0 = 0x66c0;
  struct_1->address_offset_field_0x2 = 0x1018;
  struct_param_1->address_offset_field_0x0 = 0x3ab0;
  struct_1->address_offset_field_0x2 = 0x1008;
  struct_param_1->address_offset_field_0x0 = 0x389a;
  struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



// WARNING: Inlined function: struct_1010_4d5c
// WARNING: Unable to use type for symbol uVar2_01
// WARNING: Unable to use type for symbol uVar15
// WARNING: Unable to use type for symbol uVar5
// WARNING: Unable to use type for symbol uVar8
// WARNING: Unable to use type for symbol uVar9
// WARNING: Unable to use type for symbol uVar10
// WARNING: Unable to use type for symbol uVar11
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol uVar2_00
// WARNING: Unable to use type for symbol uVar3_00
// WARNING: Unable to use type for symbol uVar23
pub fn unk_draw_op_1018_623e(param_1: *mut astruct_742)

{
  let mut

astruct_20 * struct_1018_702c(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1ec,0x1a,0xb2,0xc3,param_2,param_3);
  param_1->offset_0x0 = 0x9c66;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7062(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x1b,0xb3,0xc4,param_2,param_3);
  param_1->offset_0x0 = 0xaf4a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7098(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x1c,0xb4,0xd8,param_2,param_3);
  param_1->offset_0x0 = 0xc22e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_70ce(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x1d,0xb5,0x7b,param_2,param_3);
  param_1->offset_0x0 = 0xa4ee;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7104(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x1e,0xb6,0xd9,param_2,param_3);
  param_1->offset_0x0 = 0xb7d2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_713a(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  struct_op_1018_6a0e(param_1,0x0,0x1f,0xb7,0x7d,param_2,param_3);
  param_1->offset_0x0 = 0x9a92;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7170(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x21,0xb9,0xdd,param_2,param_3);
  param_1->offset_0x0 = 0xad76;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_71a6(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x23,0xd3,0xd6,param_2,param_3);
  param_1->offset_0x0 = 0xb69a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_71dc(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1ed,0x24,0xd4,0xd7,param_2,param_3);
  param_1->offset_0x0 = 0x995a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7212(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x25,0xe9,0xee,param_2,param_3);
  param_1->offset_0x0 = 0xa452;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7248(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x63,0xa6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xc05a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_727e(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x64,0xa9,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa31a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_72b4(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x65,0xaa,0xbb,param_2,param_3);
  param_1->offset_0x0 = 0xb5fe;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_72ea(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x66,0xab,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x98be;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7320(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x67,0xac,0xbd,param_2,param_3);
  param_1->offset_0x0 = 0xaba2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7356(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x68,0xad,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbe86;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_738c(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x69,0xae,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xac3e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_73c2(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x35,0xba,0x81,param_2,param_3);
  param_1->offset_0x0 = 0xbf22;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_73f8(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x39,0xbb,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa146;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1018_742e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut astruct_28)

{
  mixed_draw_op_1018_6a7a(param_2,param_3,param_5);
  if (PTR_LOOP_1050_4254 == NULL) {
    win_1008_5c5c(param_1,param_2,_u16_1050_02a0,0x1e9);
    if (param_1 != 0x0) {
      PTR_LOOP_1050_4254 = (&PTR_LOOP_1050_0000 + 0x1);
    }
  }
  return;
}



astruct_20 * struct_1018_745e(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x22,0xbc,0xd5,param_2,param_3);
  param_1->offset_0x0 = 0xb42a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7494(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x36,0xbd,0xcd,param_2,param_3);
  param_1->offset_0x0 = 0x96ea;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_74ca(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x37,0xbe,0x83,param_2,param_3);
  param_1->offset_0x0 = 0xa9ce;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7500(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x38,0xbf,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbcb2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7536(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x3a,0xc0,0x85,param_2,param_3);
  param_1->offset_0x0 = 0x9f72;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_756c(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1e2,0x3b,0xc1,0x86,param_2,param_3);
  param_1->offset_0x0 = 0xb256;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_75a2(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x3c,0xc2,0x87,param_2,param_3);
  param_1->offset_0x0 = 0x9516;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_75d8(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x3d,0xc3,0x88,param_2,param_3);
  param_1->offset_0x0 = 0xa7fa;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_760e(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x3e,0xc4,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbade;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7644(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x3f,0xc5,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9d02;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_767a(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x40,0xc6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xafe6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_76b0(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x41,0xc7,0x8d,param_2,param_3);
  param_1->offset_0x0 = 0xc2ca;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_76e6(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x42,0xc8,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa58a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_771c(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x43,0xc9,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb86e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7752(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x44,0xcc,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9b2e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7788(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x45,0xcd,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xae12;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_77be(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x46,0xd1,0x92,param_2,param_3);
  param_1->offset_0x0 = 0xc0f6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_77f4(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x47,0xd2,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa3b6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_782a(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x48,0xd5,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xacda;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7860(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x49,0xd6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbfbe;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7896(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1f4,0x4a,0xd7,0x98,param_2,param_3);
  param_1->offset_0x0 = 0xa1e2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_78cc(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x4b,0xd8,0x99,param_2,param_3);
  param_1->offset_0x0 = 0xb4c6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7902(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x4c,0xd9,0xee,param_2,param_3);
  param_1->offset_0x0 = 0x9786;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7938(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x4d,0xda,0x9c,param_2,param_3);
  param_1->offset_0x0 = 0xaa6a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_796e(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x4e,0xdb,0x9d,param_2,param_3);
  param_1->offset_0x0 = 0xbd4e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_79a4(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x4f,0xdc,0x9e,param_2,param_3);
  param_1->offset_0x0 = 0xa00e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_79da(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x50,0xdd,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb2f2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7a10(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1d9,0x51,0xde,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x95b2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7a46(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x52,0xdf,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa896;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7a7c(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x53,0xe0,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbb7a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7ab2(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x55,0xe2,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb082;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7ae8(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x56,0xe3,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xc366;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7b1e(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1da,0x57,0xe4,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa626;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7b54(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1d8,0x58,0xe5,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb90a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7b8a(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x59,0xe6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9bca;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7bc0(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1ef,0x5a,0xe7,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xaeae;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7bf6(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x5b,0xe8,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xc192;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7c2c(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x5c,0xea,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb736;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7c62(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x5d,0xeb,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x99f6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7c98(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1e6,0x5e,0xec,0xee,param_2,param_3);
  param_1->offset_0x0 = 0xba42;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7cce(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1da,0x5f,0xed,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9ed6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7d04(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x0,0x60,0xee,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb1ba;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7d3a(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1f0,0x61,0xef,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x947a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7d70(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  struct_op_1018_6a0e(param_1,0x1f7,0x62,0xf0,0xcc,param_2,param_3);
  param_1->offset_0x0 = 0xa75e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}
pub fn pass1_1018_7da6(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7dee(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7e36(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7e7e(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7ec6(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7f0e(StructD *param_1,param_2: u8)

{
  StructD *iVar1;
  let mut uVar1: u16;

  iVar1 = param_1;
  iVar1 = &iVar1->field192_0xd2;
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7f56(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7f9e(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1018_7fe6(StructD *param_1,param_2: u8)

{
  StructD *iVar1;
  let mut uVar1: u16;

  iVar1 = param_1;
  iVar1 = &iVar1->field192_0xd2;
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
