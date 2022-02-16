


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1008_a1f0(ushort param_1,ushort param_2,uchar param_3,ulong param_4,ushort *param_5,ushort *param_6,
               ushort *param_7,ushort *param_8)

{
  undefined4 uVar1;
  code **ppcVar2;
  ulong uVar3;
  uint uVar4;
  ushort uVar5;
  uint extraout_DX;
  uint uVar6;
  uchar *puVar7;
  ushort uVar8;
  int iVar9;
  uchar *in_buf_len_5;
  undefined2 uVar10;
  undefined2 uVar11;
  ushort *puVar12;
  char *pcVar13;
  ushort uVar14;
  undefined uVar15;
  undefined uVar16;
  char local_106 [0x100];
  undefined4 *puStack6;
  
  uVar4 = 0x0;
  *param_8 = 0x0;
  *param_7 = 0x0;
  *param_6 = 0x0;
  *param_5 = 0x0;
  in_buf_len_5 = (uchar *)(param_4 >> 0x10);
  uVar8 = (ushort)param_4;
  *(undefined *)(uVar8 + 0xe) = 0x0;
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar8 + 0xa) + 0x10);
  (**ppcVar2)(param_1,*(undefined4 *)(uVar8 + 0xa));
  puStack6 = (undefined4 *)CONCAT22(extraout_DX,uVar4);
  uVar6 = extraout_DX | uVar4;
  if (uVar6 == 0x0) {
    return;
  }
  *param_8 = *(ushort *)(uVar4 + 0x4);
  *param_6 = *(ushort *)(uVar4 + 0xa);
  uVar5 = pass1_1008_ab80(uVar8,(ushort)in_buf_len_5,*param_8);
  *param_5 = uVar5;
  uVar10 = (undefined2)((ulong)puStack6 >> 0x10);
  iVar9 = (int)puStack6;
  uVar11 = 0x1008;
  uVar14 = (ushort)_PTR_LOOP_1050_14cc;
  uVar5 = (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  switch(*(undefined2 *)(iVar9 + 0x4)) {
  case 0x1:
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd1;
    goto LAB_1008_a2b1;
  case 0x2:
    uVar1 = *(undefined4 *)(iVar9 + 0x6);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_106,param_2);
    uVar3 = (ulong)puStack6 >> 0x10;
    pcVar13 = pass1_1038_4d28(CONCAT13((char)(uVar6 >> 0x8),CONCAT12((char)uVar6,iVar9)));
    uVar11 = 0x1000;
    sys_1000_3f9c((uchar *)(uVar8 + 0xe),in_buf_len_5,(ushort)local_106,param_2,(ushort)pcVar13,&stack0xfffe,(int)uVar3,
                  0x1000,param_2,param_3);
    break;
  case 0x5:
    goto LAB_1008_a277;
  case 0x6:
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd4;
LAB_1008_a2b1:
    uVar11 = 0x1010;
    *param_6 = 0x1;
    break;
  case 0x7:
LAB_1008_a277:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    break;
  case 0x9:
    if (*(int *)(uVar8 + 0x416) == 0x0) {
      *(undefined2 *)(uVar8 + 0x416) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0xb:
    if (*(int *)(uVar8 + 0x41a) == 0x0) {
      *(undefined2 *)(uVar8 + 0x41a) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0xe:
    if (*(int *)(uVar8 + 0x41c) == 0x0) {
      *(undefined2 *)(uVar8 + 0x41c) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0x14:
    if (*(int *)(uVar8 + 0x418) == 0x0) {
      *(undefined2 *)(uVar8 + 0x418) = 0x1;
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
                 (char *)(uVar8 + 0xe),(short)in_buf_len_5);
      pcVar13 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      puVar7 = (uchar *)((ulong)pcVar13 >> 0x10);
      pass1_1000_3cea(param_4 & 0xffff0000 | ZEXT24((char *)(uVar8 + 0xe)),(ULONG)pcVar13);
      *param_7 = 0x4c;
      uVar15 = 0x1;
      uVar16 = 0x0;
      iVar9 = 0xa;
      puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_2,puVar7,(int)in_buf_len_5);
      uVar11 = 0x1010;
      pass1_1010_089e(param_2,(ulong)puVar12,CONCAT11(uVar16,uVar15),iVar9);
      break;
    }
    goto LAB_1008_a35a;
  case 0x16:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x28;
    break;
  case 0x17:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2c;
    break;
  case 0x18:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2e;
    break;
  case 0x1b:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x30;
    break;
  case 0x1c:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x32;
    break;
  case 0x1f:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x34;
    break;
  case 0x21:
    if (*(int *)(uVar8 + 0x41e) == 0x0) {
      *(undefined2 *)(uVar8 + 0x41e) = 0x1;
      break;
    }
LAB_1008_a35a:
    *param_5 = 0x0;
    break;
  case 0x24:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2a;
    break;
  case 0x31:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x27;
    break;
  case 0x32:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x29;
    break;
  case 0x33:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2b;
    break;
  case 0x34:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2d;
    break;
  case 0x35:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2f;
    break;
  case 0x36:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x31;
    break;
  case 0x37:
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    pcVar13 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar11 = 0x1000;
    pass1_1000_3cea(param_4 & 0xffff0000 | ZEXT24((char *)(uVar8 + 0xe)),(ULONG)pcVar13);
    *param_7 = 0x33;
    break;
  case 0x38:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x35;
    break;
  case 0x39:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x36;
    break;
  case 0x3a:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x37;
    break;
  case 0x3b:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x38;
    break;
  case 0x3c:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x39;
    break;
  case 0x3d:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xce;
    break;
  case 0x3e:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xcf;
    break;
  case 0x3f:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd0;
    break;
  case 0x40:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd1;
    break;
  case 0x41:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd2;
    break;
  case 0x42:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd3;
    break;
  case 0x43:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd5;
    break;
  case 0x44:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd6;
    break;
  case 0x45:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd7;
  }
  if (puStack6 != (undefined4 *)0x0) {
    ppcVar2 = (code **)*puStack6;
    (**ppcVar2)(uVar11,(int)puStack6,(char)((ulong)puStack6 >> 0x10),0x1);
  }
  return;
}



ulong __stdcall16far
pass1_1008_a8f4(ulong param_1,ushort *param_2,ushort *param_3,ushort *param_4,ushort param_5,ushort param_6,
               ushort param_7,uchar param_8)

{
  int iVar1;
  undefined4 local_6;
  
  iVar1 = (int)&local_6 + 0x2;
  pass1_1008_a1f0(param_6,param_7,param_8,param_1,param_2,(ushort *)CONCAT22(param_7,&local_6),
                  (ushort *)CONCAT22(param_7,iVar1),param_4);
  pass1_1008_944e(param_3,(ushort)local_6,(ushort)((ulong)local_6 >> 0x10));
  return CONCAT22(param_5,iVar1);
}



void __stdcall16far pass1_1008_a930(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint extraout_DX;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 *puStack24;
  undefined2 *puStack18;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar5 + 0x410));
  do {
    puVar3 = local_a;
    pass1_1008_5b12(puVar3,param_3);
    uVar4 = extraout_DX | (uint)puVar3;
    if (uVar4 == 0x0) {
      mem_op_1000_179c(0x6,(uchar *)0x0,0x1000);
      puStack24 = (undefined2 *)CONCAT22(uVar4,puVar3);
      if ((uVar4 | (uint)puVar3) == 0x0) {
        puStack18 = (undefined2 *)0x0;
      }
      else {
        *puStack24 = 0x389a;
        *(undefined2 *)(puVar3 + 0x2) = 0x1008;
        *(int *)(puVar3 + 0x4) = param_2;
        *puStack24 = 0xad8a;
        *(undefined2 *)(puVar3 + 0x2) = 0x1008;
        puStack18 = puStack24;
      }
      uVar1 = *(undefined4 *)(iVar5 + 0x410);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x410) + 0x8);
      (**ppcVar2)(0x1000,(int)uVar1,(char)((ulong)uVar1 >> 0x10),(int)puStack18,(int)((ulong)puStack18 >> 0x10));
      return;
    }
  } while (*(int *)(puVar3 + 0x4) != param_2);
  return;
}



ushort __stdcall16far pass1_1008_a9ec(ulong param_1)

{
  undefined4 uVar1;
  uint in_AX;
  int iVar2;
  uint uVar3;
  WNDCLASS16 *unaff_SS;
  uint uStack4;
  
  uStack4 = 0x0;
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(int *)(iVar2 + 0x414) == 0x0) && (uVar1 = *(undefined4 *)(iVar2 + 0x410), *(int *)((int)uVar1 + 0x8) != 0x0)) {
    *(undefined2 *)(iVar2 + 0x414) = 0x1;
    pass1_1008_aa28(param_1 & 0xffff | (ulong)uVar3 << 0x10,in_AX,unaff_SS);
    uStack4 = in_AX;
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_aa28(ulong param_1,uint param_2,WNDCLASS16 *param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint extraout_DX;
  int iVar3;
  undefined2 uVar4;
  undefined4 *puStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(int *)(iVar3 + 0x414) != 0x0) {
    uVar2 = *(undefined4 *)(iVar3 + 0x410);
    if (*(int *)((int)uVar2 + 0x8) == 0x0) {
      *(undefined2 *)(iVar3 + 0x414) = 0x0;
      return;
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x410) + 0x10);
    (**ppcVar1)();
    puStack6 = (undefined4 *)CONCAT22(extraout_DX,param_2);
    if ((extraout_DX | param_2) != 0x0) {
      win_1008_5c5c(param_3,param_2,extraout_DX | param_2,_PTR_LOOP_1050_02a0,*(ushort *)(param_2 + 0x4));
      if (puStack6 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack6;
        (**ppcVar1)();
      }
      return;
    }
  }
  return;
}



ushort __stdcall16far pass1_1008_aaa8(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uStack4;
  
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



ushort __stdcall16far pass1_1008_ab12(ushort param_1,ushort param_2,uint param_3)

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



ushort __stdcall16far pass1_1008_ab54(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  ushort uStack4;
  
  uStack4 = 0x0;
  uVar2 = (undefined2)(param_1 >> 0x10);
  if ((*(long *)((int)param_1 + 0xa) != 0x0) &&
     (uVar1 = *(undefined4 *)((int)param_1 + 0xa), *(int *)((int)uVar1 + 0x8) != 0x0)) {
    uStack4 = 0x1;
  }
  return uStack4;
}



ushort __stdcall16far pass1_1008_ab80(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uStack4;
  
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



ushort * __stdcall16far pass1_1008_ad0c(ushort *param_1,byte param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x389a;
  ((int *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((int *)param_1,uVar1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_ad38(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_ad64(ulong param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_a086((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_ada2(ushort *param_1,int param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x0;
  *(int *)((int)param_1 + 0x4) = param_2;
  *param_1 = *(ushort *)(param_2 * 0x6 + 0x3a4);
  return param_1;
}



void __stdcall16far pass1_1008_add2(ushort *param_1)

{
  *param_1 = *(ushort *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a4);
  return;
}



ushort __stdcall16far pass1_1008_adf2(ulong param_1)

{
  return *(ushort *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a4);
}



ushort __stdcall16far pass1_1008_ae0c(ulong param_1)

{
  return *(ushort *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a6);
}



void __stdcall16far pass1_1008_ae26(int *param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = *(int *)(*(int *)(iVar3 + 0x4) * 0x6 + 0x3a8);
  if (iVar2 == 0x2) {
    if (*(int *)(iVar3 + 0x2) == 0x1) {
      *param_1 = *param_1 + -0x1;
      iVar2 = *(int *)(iVar3 + 0x4) * 0x6;
      piVar1 = (int *)(iVar2 + 0x3a4);
      if (*piVar1 != *param_1 && *param_1 <= *piVar1) {
        *param_1 = *(int *)(iVar2 + 0x3a4) + 0x1;
        *(undefined2 *)(iVar3 + 0x2) = 0x0;
        return;
      }
    }
    else {
      *param_1 = *param_1 + 0x1;
      iVar2 = *(int *)(iVar3 + 0x4) * 0x6;
      if (*(int *)(iVar2 + 0x3a6) < *param_1) {
        *param_1 = *(int *)(iVar2 + 0x3a6) + -0x1;
        *(undefined2 *)(iVar3 + 0x2) = 0x1;
        return;
      }
    }
  }
  else {
    if ((iVar2 != 0x3) && (iVar2 != 0x4)) {
      *param_1 = *param_1 + 0x1;
      iVar2 = *(int *)(iVar3 + 0x4) * 0x6;
      if (*(int *)(iVar2 + 0x3a6) < *param_1) {
        *param_1 = *(int *)(iVar2 + 0x3a4);
      }
    }
  }
  return;
}



BOOL16 __stdcall16far pass1_1008_aed8(ulong param_1)

{
  if (*(int *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a4) != 0x0) {
    return 0x1;
  }
  return 0x0;
}



ulong __stdcall16far pass1_1008_aefe(uchar *param_1,uchar *param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  struct_op_1018_4cda((int)param_1,(ushort)param_2,param_3);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xaf7c;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x1b3,param_4,param_5);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1008_af38(astruct_11 *param_1)

{
  *(undefined2 *)param_1 = 0xaf7c;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}



ulong __stdcall16far pass1_1008_af56(ulong param_1,byte param_2)

{
  pass1_1008_af38((astruct_11 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_af94(astruct_643 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x22 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xbdcc;
  param_1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_afde(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_468 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_468 *)param_1;
  *param_1 = 0xbdcc;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x12;
  uVar2 = iVar4->field_0x14;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



ushort * __stdcall16far pass1_1008_b05a(ushort *param_1)

{
  astruct_193 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_193 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  *param_1 = 0xbdc8;
  iVar1->field_0x2 = 0x1008;
  return param_1;
}



void __stdcall16far pass1_1008_b08c(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0xbdc8;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x4),0x1000);
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far set_stuct_1008_b0bc(astruct_26 *param_1)

{
  astruct_26 *iVar1;
  undefined2 uVar1;
  
  pass1_1008_b05a((ushort *)param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_26 *)param_1;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x10 = 0x0;
  *(undefined2 *)param_1 = 0xbdc4;
  iVar1->field_0x2 = 0x1008;
  return;
}



ushort * __stdcall16far pass1_1008_b0f2(ushort *param_1)

{
  undefined2 uVar1;
  
  pass1_1008_b05a(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x8) = 0x0;
  *param_1 = 0xbdc0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return param_1;
}



ushort * __stdcall16far pass1_1008_b11e(ushort *param_1)

{
  undefined2 uVar1;
  
  pass1_1008_b05a(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x8) = 0x0;
  *param_1 = 0xbddc;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_b146(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x16) != 0x0) {
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                    *(ulong *)((int)uVar1 + 0xa));
    pass1_1038_3608(CONCAT22(param_3,param_2));
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined2 *)((int)uVar1 + 0x8) = 0x0;
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined4 *)((int)uVar1 + 0xa) = 0x0;
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined2 *)((int)uVar1 + 0xe) = 0x0;
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined2 *)((int)uVar1 + 0x10) = 0x0;
  }
  return;
}



void __stdcall16far pass1_1008_b1a6(ulong param_1,char *param_2)

{
  long lVar1;
  ushort uVar2;
  ushort in_DX;
  astruct_467 *iVar3;
  astruct_466 *iVar4;
  undefined2 uVar3;
  undefined2 uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_467 *)param_1;
  if (iVar3->field_0x16 != 0x0) {
    lVar1 = iVar3->field_0x16;
    fn_ptr_1000_17ce(*(astruct_18 **)((int)lVar1 + 0x4),0x1000);
    uVar2 = str_op_1008_60e8(param_2,in_DX);
    lVar1 = iVar3->field_0x16;
    uVar4 = (undefined2)((ulong)lVar1 >> 0x10);
    iVar4 = (astruct_466 *)lVar1;
    iVar4->field_0x4 = uVar2;
    iVar4->field_0x6 = in_DX;
    iVar3->field_0x16 = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __stdcall16far load_string_1008_b1f0(void)

{
  char *pcVar1;
  
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return pcVar1;
}
