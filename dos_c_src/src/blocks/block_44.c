void __stdcall16far unk_destroy_window_op_1018_6bb6(astruct_28 *param_1,HWND16 param_2)

{
  BOOL16 BVar1;
  astruct_28 *iVar2;
  undefined2 uVar2;
  HWND16 hwnd;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_28 *)param_1;
  hwnd = param_2;
  if (iVar2->field_0xea != 0x0) {
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,iVar2->field_0xea));
  }
  PostMessage16(hwnd,0x0,0x0,0x1110079);
  if (iVar2->field_0xf0 != 0x0) {
    BVar1 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
    if (BVar1 != 0x0) {
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
      iVar2->field_0xf0 = 0x0;
    }
  }
  return;
}



void __stdcall16far pass1_1018_6c1e(ushort *param_1,byte param_2)

{
  astruct_512 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_512 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



astruct_20 * __stdcall16far struct_1018_6d02(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0xb,0x9c,0x8b,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa27e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6d38(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0xc,0x9d,0xd0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb562;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6d6e(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0xd,0x9e,0xd1,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9822;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6da4(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0xe,0x9f,0xd2,param_2,param_3,param_4);
  param_1->field_0x0 = 0xab06;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6dda(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0xf,0xa0,0xd4,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbdea;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6e10(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x10,0xa1,0xda,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa0aa;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6e46(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x11,0xa2,0xdc,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb38e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6e7c(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x12,0xa3,0xd3,param_2,param_3,param_4);
  param_1->field_0x0 = 0x964e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6eb2(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x13,0xa4,0xdb,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa932;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6ee8(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x14,0xa5,0xa5,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbc16;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6f1e(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x15,0xa7,0xb2,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9e3a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6f54(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x16,0xa8,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb11e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6f8a(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x17,0xaf,0xc0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x93de;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6fc0(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x18,0xb0,0xc1,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa6c2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_6ff6(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x19,0xb1,0x80,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb9a6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_702c(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x1ec,0x1a,0xb2,0xc3,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9c66;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7062(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x1b,0xb3,0xc4,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaf4a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7098(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x1c,0xb4,0xd8,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc22e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_70ce(astruct_20 *param_1,UINT16 param_2,ULONG param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x1d,0xb5,0x7b,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa4ee;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7104(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x1e,0xb6,0xd9,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb7d2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_713a(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x1f,0xb7,0x7d,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9a92;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7170(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x21,0xb9,0xdd,param_2,param_3,param_4);
  param_1->field_0x0 = 0xad76;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_71a6(astruct_20 *param_1,UINT16 param_2,ULONG param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x23,0xd3,0xd6,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb69a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_71dc(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x1ed,0x24,0xd4,0xd7,param_2,param_3,param_4);
  param_1->field_0x0 = 0x995a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7212(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x25,0xe9,0xee,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa452;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7248(astruct_20 *param_1,UINT16 param_2,ULONG param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x63,0xa6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc05a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_727e(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x64,0xa9,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa31a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_72b4(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x65,0xaa,0xbb,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb5fe;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_72ea(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x66,0xab,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x98be;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7320(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x67,0xac,0xbd,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaba2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7356(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x68,0xad,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbe86;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_738c(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x69,0xae,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xac3e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_73c2(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x35,0xba,0x81,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbf22;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_73f8(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x39,0xbb,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa146;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_745e(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x22,0xbc,0xd5,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb42a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7494(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x36,0xbd,0xcd,param_2,param_3,param_4);
  param_1->field_0x0 = 0x96ea;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_74ca(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x37,0xbe,0x83,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa9ce;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7500(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x38,0xbf,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbcb2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7536(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x3a,0xc0,0x85,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9f72;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_756c(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1e2,0x3b,0xc1,0x86,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb256;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far pass1_1018_75a2(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x3c,0xc2,0x87,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9516;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far pass1_1018_75d8(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x3d,0xc3,0x88,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa7fa;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_760e(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x3e,0xc4,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbade;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7644(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x3f,0xc5,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9d02;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_767a(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x40,0xc6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xafe6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_76b0(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x41,0xc7,0x8d,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc2ca;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_76e6(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x42,0xc8,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa58a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_771c(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x43,0xc9,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb86e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7752(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x44,0xcc,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9b2e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7788(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x45,0xcd,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xae12;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_77be(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x46,0xd1,0x92,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc0f6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_77f4(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x47,0xd2,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa3b6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_782a(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x48,0xd5,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xacda;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7860(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x49,0xd6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbfbe;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7896(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1f4,0x4a,0xd7,0x98,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa1e2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_78cc(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x4b,0xd8,0x99,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb4c6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7902(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x4c,0xd9,0xee,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9786;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7938(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x4d,0xda,0x9c,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaa6a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_796e(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x4e,0xdb,0x9d,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbd4e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_79a4(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x4f,0xdc,0x9e,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa00e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_79da(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x50,0xdd,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb2f2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7a10(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1d9,0x51,0xde,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x95b2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7a46(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x52,0xdf,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa896;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7a7c(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x53,0xe0,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbb7a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7ab2(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x55,0xe2,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb082;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7ae8(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x56,0xe3,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc366;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7b1e(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1da,0x57,0xe4,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa626;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7b54(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1d8,0x58,0xe5,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb90a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7b8a(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x59,0xe6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9bca;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7bc0(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1ef,0x5a,0xe7,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaeae;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7bf6(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x5b,0xe8,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc192;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7c2c(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x5c,0xea,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb736;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7c62(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x5d,0xeb,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x99f6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7c98(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1e6,0x5e,0xec,0xee,param_2,param_3,param_4);
  param_1->field_0x0 = 0xba42;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7cce(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1da,0x5f,0xed,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9ed6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7d04(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x60,0xee,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb1ba;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7d3a(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1f0,0x61,0xef,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x947a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * __stdcall16far struct_1018_7d70(astruct_20 *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  struct_op_1018_6a0e(param_1,0x1f7,0x62,0xf0,0xcc,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa75e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



void __stdcall16far pass1_1018_7da6(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



void __stdcall16far pass1_1018_7dee(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



void __stdcall16far pass1_1018_7e36(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



void __stdcall16far pass1_1018_7e7e(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



void __stdcall16far pass1_1018_7ec6(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}
