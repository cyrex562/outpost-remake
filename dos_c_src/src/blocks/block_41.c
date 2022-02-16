
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined4 __stdcall16far switch_1018_3b9e(ulong param_1,ushort param_2,ushort param_3,uint param_4,ushort param_5)

{
  undefined4 uVar1;
  astruct_263 *iVar2;
  uint uVar2;
  ulong uStack14;
  undefined2 uStack6;
  undefined2 uStack4;
  
  uStack6 = 0x0;
  uStack4 = 0x0;
  uVar2 = (uint)(param_1 >> 0x10);
  iVar2 = (astruct_263 *)param_1;
  uVar1 = iVar2->field_0x122;
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),iVar2->field_0x126,param_5,param_4);
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),CONCAT22(param_4,param_3));
  uStack14 = CONCAT22(param_4,param_3);
  switch(param_2) {
  case 0x188:
    if (iVar2->field_0xa == 0x0) {
      pass1_1008_d3ae(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    }
    uStack6 = *(undefined2 *)&iVar2->field_0xa;
    uStack4 = *(undefined2 *)((int)&iVar2->field_0xa + 0x2);
    break;
  case 0x189:
    if (iVar2->field_0xe == 0x0) {
      unk_str_op_1008_d4f6(param_1 & 0xffff | (ulong)uVar2 << 0x10,uStack14);
    }
    uStack6 = *(undefined2 *)&iVar2->field_0xe;
    uStack4 = *(undefined2 *)((int)&iVar2->field_0xe + 0x2);
    break;
  case 0x18a:
    if (iVar2->field_0x12 == 0x0) {
      unk_str_op_1008_d1c6(param_1 & 0xffff | (ulong)uVar2 << 0x10,uStack14);
    }
    uStack6 = *(undefined2 *)&iVar2->field_0x12;
    uStack4 = *(undefined2 *)((int)&iVar2->field_0x12 + 0x2);
    break;
  case 0x18b:
    if (iVar2->field_0x16 == 0x0) {
      pass1_1008_cfa0(param_1 & 0xffff | (ulong)uVar2 << 0x10,uStack14);
    }
    uStack6 = *(undefined2 *)&iVar2->field_0x16;
    uStack4 = *(undefined2 *)((int)&iVar2->field_0x16 + 0x2);
    break;
  case 0x18c:
    if (iVar2->field_0x1a == 0x0) {
      pass1_1008_cda2(param_1 & 0xffff | (ulong)uVar2 << 0x10,uStack14,param_5);
    }
    uStack6 = *(undefined2 *)&iVar2->field_0x1a;
    uStack4 = *(undefined2 *)((int)&iVar2->field_0x1a + 0x2);
    break;
  case 0x18d:
    if (iVar2->field_0x1e == 0x0) {
      pass1_1008_cbc4(param_1 & 0xffff | (ulong)uVar2 << 0x10,uStack14,param_5);
    }
    uStack6 = *(undefined2 *)&iVar2->field_0x1e;
    uStack4 = *(undefined2 *)((int)&iVar2->field_0x1e + 0x2);
  }
  return CONCAT22(uStack4,uStack6);
}



void __stdcall16far pass1_1018_3cda(ulong *param_1,char *param_2,char *param_3)

{
  code **ppcVar1;
  ushort uVar2;
  ushort extraout_DX;
  ushort uVar3;
  astruct_506 *iVar5;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_506 *)param_1;
  ppcVar1 = (code **)((int)*param_1 + 0x10);
  (**ppcVar1)();
  uVar3 = extraout_DX;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x126,0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x12a,0x1000);
  uVar2 = str_op_1008_60e8(param_3,uVar3);
  iVar5->field_0x126 = uVar2;
  iVar5->field_0x128 = uVar3;
  uVar2 = str_op_1008_60e8(param_2,uVar3);
  iVar5->field_0x12a = uVar2;
  iVar5->field_0x12c = uVar3;
  return;
}



void __stdcall16far pass1_1018_3d44(ulong param_1,ulong *param_2,ulong *param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *param_3 = *(ulong *)((int)param_1 + 0x126);
  *param_2 = *(ulong *)((int)param_1 + 0x12a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_3d6c(ulong param_1)

{
  byte bVar1;
  uint uVar2;
  undefined *puVar3;
  uint uVar4;
  astruct_679 *iVar6;
  undefined2 uVar5;
  ulong uVar6;
  ulong uVar7;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_679 *)param_1;
  uVar4 = iVar6->field_0x142;
  uVar2 = uVar4 + 0x1e;
  if (iVar6->field_0x144 + 0x1U == (uint)(uVar4 < 0xffe2)) {
    if (uVar2 != 0x3c) {
      if (0x3c < uVar2) {
        return;
      }
      bVar1 = (byte)uVar2;
      if (bVar1 == 0x14) {
        iVar6->field_0x142 = 0xffec;
LAB_1018_3e3d:
        iVar6->field_0x144 = -0x1;
        return;
      }
      if (0x14 < bVar1) {
        if (bVar1 == 0x1e) {
          if ((int)PTR_LOOP_1050_13ae < 0x1) {
            return;
          }
          if (SBORROW2((int)PTR_LOOP_1050_13ae,0x1)) {
            return;
          }
          if (PTR_LOOP_1050_13ae != (undefined *)&PTR_LOOP_1050_0002 && 0x0 < (int)(PTR_LOOP_1050_13ae + -0x1)) {
            puVar3 = PTR_LOOP_1050_13ae + -0x3;
            if (puVar3 == (undefined *)0x0) {
              pass1_1008_612e(0x1,0x64,0x0);
              if ((int)puVar3 < 0x32) {
                uVar4 = 0xa;
              }
              else {
                uVar4 = 0xfff6;
              }
              iVar6->field_0x142 = uVar4;
              iVar6->field_0x144 = (int)uVar4 >> 0xf;
              return;
            }
            if (puVar3 != (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
              return;
            }
            iVar6->field_0x142 = 0xfff6;
            goto LAB_1018_3e3d;
          }
          iVar6->field_0x142 = 0xa;
        }
        else {
          if (bVar1 == 0x28) {
            iVar6->field_0x142 = 0x14;
          }
          else {
            if (bVar1 != 0x32) {
              return;
            }
            iVar6->field_0x142 = 0x1e;
          }
        }
        iVar6->field_0x144 = 0x0;
        return;
      }
      if (bVar1 != 0x0) {
        if (bVar1 != 0xa) {
          return;
        }
        iVar6->field_0x142 = 0xffe2;
        goto LAB_1018_3e3d;
      }
    }
    uVar7 = 0x5;
    uVar6 = pass1_1030_8326();
    if (uVar6 % uVar7 == 0x0) {
      *(undefined4 *)&iVar6->field_0x142 = 0x0;
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1018_3e8c(ushort param_1,ushort param_2,ushort *param_3,ushort *param_4)

{
  *param_4 = 0x1;
  *param_3 = 0x19;
  return;
}



void __stdcall16far pass1_1018_3ea4(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  pass1_1008_cac6(param_1);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x136);
  uVar2 = *(uint *)(iVar4 + 0x138);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar4 + 0x136) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far switch_1018_3ee6(ulong param_1,long param_2,int param_3,ushort param_4,uchar *param_5)

{
  int iVar1;
  char *pcVar2;
  ushort uVar3;
  ushort uVar4;
  uint uVar5;
  uint uVar6;
  ulong uVar7;
  uchar *puVar8;
  ushort unaff_SS;
  ushort *puVar9;
  long lVar10;
  int iVar11;
  INT16 IVar12;
  ushort uVar13;
  ulong uStack26;
  ushort *puStack22;
  long lStack18;
  long lStack14;
  int iStack10;
  undefined2 uStack8;
  int *piStack6;
  
  switch(param_4) {
  case 0x1:
    iVar1 = param_3 * 0x4 + 0x40b6;
    break;
  default:
    iVar1 = param_3 * 0x4 + 0x40ce;
    break;
  case 0x3:
    iVar1 = param_3 * 0x4 + 0x40e2;
    break;
  case 0x4:
    iVar1 = param_3 * 0x4 + 0x40ee;
    break;
  case 0x8:
    iVar1 = param_3 * 0x4 + 0x40f2;
    break;
  case 0x9:
    iVar1 = param_3 * 0x4 + 0x4106;
    break;
  case 0xa:
    iVar1 = param_3 * 0x4 + 0x410a;
    break;
  case 0x14:
    iVar1 = param_3 * 0x4 + 0x410e;
    break;
  case 0x16:
    iVar1 = param_3 * 0x4 + 0x4112;
    break;
  case 0x17:
    iVar1 = param_3 * 0x4 + 0x4116;
    break;
  case 0x19:
    iVar1 = param_3 * 0x4 + 0x411a;
  }
  piStack6 = (int *)CONCAT22(0x1050,iVar1);
  if (piStack6 == (int *)0x0) {
    return;
  }
  iStack10 = 0x0;
  uStack8 = 0x0;
  iVar11 = *piStack6;
  uVar13 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  if (iVar11 == 0x1) {
    uVar13 = pass1_1018_456a(uVar13,uVar3,*(ushort *)(iVar1 + 0x2));
    lStack14 = CONCAT22(param_5,uVar13);
    pcVar2 = string_1020_c0d8(*(ushort *)(iVar1 + 0x2));
    uVar3 = str_op_1008_60e8((char *)CONCAT22(param_5,pcVar2),(ushort)param_5);
    puVar8 = param_5;
    uVar13 = uVar3;
    mem_op_1000_179c(0x10,param_5,0x1000);
    puStack22 = (ushort *)CONCAT22(puVar8,uVar13);
    if (((uint)puVar8 | uVar13) != 0x0) {
      lVar10 = param_2 / lStack14;
      uStack8 = (undefined2)(param_2 % lStack14);
      struct_1018_4790(puStack22,lVar10,CONCAT22(param_5,uVar3),*(ushort *)(iVar1 + 0x2));
      iStack10 = (int)lVar10;
      goto LAB_1018_425e;
    }
  }
  else {
    if (iVar11 == 0x2) {
      uVar13 = pass1_1018_451e(uVar13,uVar3,*(int *)(iVar1 + 0x2));
      lStack18 = CONCAT22(param_5,uVar13);
      pcVar2 = string_op_1020_c222(*(ushort *)(iVar1 + 0x2));
      uVar3 = str_op_1008_60e8((char *)CONCAT22(param_5,pcVar2),(ushort)param_5);
      puVar8 = param_5;
      uVar13 = uVar3;
      mem_op_1000_179c(0x10,param_5,0x1000);
      puStack22 = (ushort *)CONCAT22(puVar8,uVar13);
      if (((uint)puVar8 | uVar13) != 0x0) {
        puVar9 = struct_1018_48b0(puStack22,param_2 / lStack18,CONCAT22(param_5,uVar3),*(ushort *)(iVar1 + 0x2));
        uStack8 = (undefined2)((ulong)puVar9 >> 0x10);
        iStack10 = (int)puVar9;
        goto LAB_1018_425e;
      }
    }
    else {
      if (iVar11 == 0x3) {
        uVar4 = pass1_1008_c646((ushort)_PTR_LOOP_1050_06e0,
                                CONCAT22(*(undefined2 *)(iVar1 + 0x2),(int)((ulong)_PTR_LOOP_1050_06e0 >> 0x10)),
                                unaff_SS);
        if (uVar4 == 0x0) {
          uVar4 = 0x4f;
        }
        uVar13 = switch_1018_43ec(uVar13,uVar3,uVar4);
        lStack14 = CONCAT22(param_5,uVar13);
        uVar13 = pass1_1020_bd80(uVar4);
        uVar5 = str_op_1008_60e8((char *)CONCAT22(param_5,uVar13),(ushort)param_5);
        uStack26 = CONCAT22(param_5,uVar5);
        mem_op_1000_179c(0x14,param_5,0x1000);
        puStack22 = (ushort *)CONCAT22(param_5,uVar5);
        if (((uint)param_5 | uVar5) != 0x0) {
          uVar7 = param_2 / lStack14;
          uStack8 = (undefined2)(param_2 % lStack14);
          struct_1018_47c8(puStack22,uVar7,uStack26,uVar4,0x0);
          iStack10 = (int)uVar7;
          goto LAB_1018_425e;
        }
      }
      else {
        if (iVar11 != 0x4) goto LAB_1018_425e;
        iVar1 = *(int *)(iVar1 + 0x2);
        uVar5 = iVar1 - 0x1;
        iVar11 = (int)_PTR_LOOP_1050_14cc;
        IVar12 = (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
        if (uVar5 == 0x0) {
          load_string_1010_84ac(iVar11,IVar12,0x1010);
          uVar6 = uVar5;
          puVar8 = param_5;
          mem_op_1000_179c(0x14,param_5,0x1000);
          puStack22 = (ushort *)CONCAT22(puVar8,uVar6);
          if (((uint)puVar8 | uVar6) != 0x0) {
            uVar13 = 0x2;
            lVar10 = 0x14;
LAB_1018_4230:
            puVar9 = struct_1018_4842(puStack22,param_2 / lVar10,CONCAT22(param_5,uVar5),uVar13);
            uStack8 = (undefined2)((ulong)puVar9 >> 0x10);
            iStack10 = (int)puVar9;
            goto LAB_1018_425e;
          }
        }
        else {
          uVar5 = iVar1 - 0x2;
          if (uVar5 == 0x0) {
            load_string_1010_84ac(iVar11,IVar12,0x1010);
            uVar6 = uVar5;
            puVar8 = param_5;
            mem_op_1000_179c(0x14,param_5,0x1000);
            puStack22 = (ushort *)CONCAT22(puVar8,uVar6);
            if (((uint)puVar8 | uVar6) != 0x0) {
              uVar13 = 0x3;
              lVar10 = 0x16;
              goto LAB_1018_4230;
            }
          }
          else {
            uVar5 = iVar1 - 0x3;
            if (uVar5 == 0x0) {
              load_string_1010_84ac(iVar11,IVar12,0x1010);
              uVar6 = uVar5;
              puVar8 = param_5;
              mem_op_1000_179c(0x14,param_5,0x1000);
              puStack22 = (ushort *)CONCAT22(puVar8,uVar6);
              if (((uint)puVar8 | uVar6) != 0x0) {
                uVar13 = 0x4;
                lVar10 = 0x17;
                goto LAB_1018_4230;
              }
            }
            else {
              uVar5 = iVar1 - 0x4;
              if (uVar5 != 0x0) goto LAB_1018_425e;
              load_string_1010_84ac(iVar11,IVar12,0x1010);
              uVar6 = uVar5;
              puVar8 = param_5;
              mem_op_1000_179c(0x14,param_5,0x1000);
              puStack22 = (ushort *)CONCAT22(puVar8,uVar6);
              if (((uint)puVar8 | uVar6) != 0x0) {
                uVar13 = 0x4;
                lVar10 = 0xa;
                goto LAB_1018_4230;
              }
            }
          }
        }
      }
    }
  }
  iStack10 = 0x0;
  uStack8 = 0x0;
LAB_1018_425e:
  if (*(long *)(iStack10 + 0x8) == 0x0) {
    *(undefined4 *)(iStack10 + 0x8) = 0x1;
  }
  return;
}



void __stdcall16far pass1_1018_427c(ulong param_1)

{
  int iVar1;
  ushort in_AX;
  undefined2 in_DX;
  ushort uVar2;
  ushort uVar3;
  ushort unaff_SS;
  ulong uVar4;
  long lVar5;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  uVar4 = switch_1018_3b9e(param_1,*(ushort *)(uVar2 + 0x12e),in_AX,in_DX,unaff_SS);
  iVar1 = *(int *)(uVar2 + 0x12e);
  if (iVar1 == 0x188) {
    lVar5 = pass1_1008_57f0(uVar4,*(int *)(uVar2 + 0x130),unaff_SS);
    pass1_1018_456a(uVar2,uVar3,*(ushort *)((int)lVar5 + 0xe));
  }
  else {
    if (iVar1 == 0x18b) {
      lVar5 = pass1_1008_57f0(uVar4,*(int *)(uVar2 + 0x130),unaff_SS);
      pass1_1018_45d4(uVar2,uVar3,*(int *)((int)lVar5 + 0xe));
    }
    else {
      if (iVar1 == 0x18c) {
        lVar5 = pass1_1008_57f0(uVar4,*(int *)(uVar2 + 0x130),unaff_SS);
        pass1_1018_451e(uVar2,uVar3,*(int *)((int)lVar5 + 0xe));
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_435e(ulong param_1,long param_2,int param_3,int param_4,uint param_5,ushort param_6)

{
  undefined4 uVar1;
  uint uVar2;
  uint uVar3;
  undefined2 uVar4;
  
  if (param_3 < param_4) {
    param_4 = param_3;
  }
  uVar2 = 0x0;
  uVar4 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x122);
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong *)((int)param_1 + 0x126),param_6,param_5);
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),CONCAT22(param_5,uVar2));
  do {
    do {
      uVar3 = uVar2;
      pass1_1008_612e(param_4,param_3,uVar3);
      uVar2 = *(uint *)(uVar3 * 0x2 + 0x411c);
    } while (uVar2 == 0x0);
    if (uVar2 != 0x1) {
      pass1_1008_612e(0x1,uVar2,uVar2);
    }
    uVar2 = uVar2 - 0x1;
    switch_1018_3ee6(param_1,param_2,uVar2,uVar3,param_5);
    param_5 = param_5 | uVar2;
  } while (param_5 == 0x0);
  return;
}



ushort __stdcall16far switch_1018_43ec(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uStack6;
  
  switch(param_3) {
  case 0xf:
  case 0x35:
  case 0x36:
    uStack6 = 0x7;
    break;
  default:
    uStack6 = 0x1;
    break;
  case 0x11:
  case 0x13:
  case 0x14:
  case 0x15:
  case 0x2d:
  case 0x2e:
  case 0x6e:
    uStack6 = 0x9;
    break;
  case 0x12:
  case 0x31:
  case 0x32:
  case 0x52:
  case 0x53:
  case 0x54:
  case 0x55:
  case 0x56:
  case 0x5a:
  case 0x5b:
  case 0x5c:
  case 0x5d:
  case 0x5e:
  case 0x5f:
    uStack6 = 0x4;
    break;
  case 0x1b:
  case 0x1c:
  case 0x1d:
  case 0x28:
  case 0x29:
  case 0x2c:
  case 0x2f:
  case 0x30:
  case 0x68:
  case 0x69:
    uStack6 = 0x5;
    break;
  case 0x1e:
  case 0x1f:
  case 0x20:
  case 0x33:
  case 0x34:
    uStack6 = 0x6;
    break;
  case 0x22:
  case 0x23:
  case 0x24:
    uStack6 = 0x8;
    break;
  case 0x25:
  case 0x26:
  case 0x27:
    uStack6 = 0x2;
    break;
  case 0x38:
  case 0x39:
  case 0x4f:
  case 0x50:
  case 0x51:
  case 0x57:
  case 0x58:
  case 0x59:
  case 0x66:
  case 0x67:
  case 0x6c:
  case 0x6d:
    uStack6 = 0x3;
  }
  return uStack6;
}



ushort __stdcall16far pass1_1018_451e(ushort param_1,ushort param_2,int param_3)

{
  ushort uStack6;
  
  if (param_3 == 0x7) {
    uStack6 = 0x9;
  }
  else {
    if (param_3 == 0x8) {
      uStack6 = 0xa;
    }
    else {
      if (param_3 == 0xc) {
        uStack6 = 0x19;
      }
      else {
        if (param_3 == 0xd) {
          uStack6 = 0x3;
        }
        else {
          uStack6 = 0x8;
        }
      }
    }
  }
  return uStack6;
}



ushort __stdcall16far pass1_1018_456a(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uStack6;
  
  switch(param_3) {
  case 0x11:
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x15:
    uStack6 = 0x2;
    break;
  case 0x16:
  case 0x1e:
    uStack6 = 0x3;
    break;
  default:
    uStack6 = 0x1;
    break;
  case 0x1d:
  case 0x21:
    uStack6 = 0x4;
  }
  return uStack6;
}



ushort __stdcall16far pass1_1018_45d4(ushort param_1,ushort param_2,int param_3)

{
  ushort uStack6;
  
  if (param_3 == 0x3) {
    uStack6 = 0x16;
  }
  else {
    if (param_3 == 0x4) {
      uStack6 = 0x17;
    }
    else {
      uStack6 = 0x14;
    }
  }
  return uStack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

long __stdcall16far pass1_1018_4608(ushort param_1,ulong param_2,ulong param_3,ulong param_4)

{
  undefined4 uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  long lVar7;
  char *pcVar8;
  char *pcVar9;
  ulong uStack26;
  ulong uStack22;
  undefined local_a [0x8];
  
  uVar1 = *(undefined4 *)((int)param_2 + 0x122);
  pass1_1008_5784((ulong *)CONCAT22(param_1,local_a),*(ulong *)((int)uVar1 + 0xa));
  while( true ) {
    lVar7 = pass1_1008_5b12(local_a,param_1);
    uVar5 = (uint)((ulong)lVar7 >> 0x10);
    uVar2 = (uint)lVar7;
    uVar6 = uVar5 | uVar2;
    if (lVar7 == 0x0) {
      return 0x0;
    }
    uVar1 = *(undefined4 *)(uVar2 + 0x4);
    uVar3 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    uStack22 = CONCAT22(uVar6,uVar3);
    uVar1 = *(undefined4 *)(uVar2 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    uStack26 = CONCAT22(uVar6,uVar3);
    pcVar8 = pass1_1038_4d28(uStack22);
    pcVar9 = pass1_1038_4d28(uStack26);
    iVar4 = pass1_1000_3d7a(param_4,(ulong)pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3,(ulong)pcVar9), iVar4 == 0x0)) break;
    iVar4 = pass1_1000_3d7a(param_3,(ulong)pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4,(ulong)pcVar9), iVar4 == 0x0)) {
      return lVar7;
    }
  }
  return lVar7;
}



ushort * __stdcall16far pass1_1018_46e6(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1018_33b4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1018_4720(ushort *param_1,ulong param_2,ulong param_3)

{
  astruct_204 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_204 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = 0x0;
  *param_1 = (ushort)&PTR_LOOP_1050_4aa6;
  iVar1->field_0x2 = 0x1018;
  return;
}



void __stdcall16far pass1_1018_4760(ushort *param_1)

{
  astruct_507 *iVar2;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_507 *)param_1;
  *param_1 = (ushort)&PTR_LOOP_1050_4aa6;
  iVar2->field_0x2 = 0x1018;
  fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0x4,0x1000);
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}



ushort * __stdcall16far struct_1018_4790(ushort *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  astruct_266 *iVar1;
  undefined2 uVar1;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_266 *)param_1;
  iVar1->field_0xe = param_4;
  *param_1 = 0x4a92;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x1;
  return param_1;
}



void __stdcall16far struct_1018_47c8(ushort *param_1,ulong param_2,ulong param_3,ushort param_4,ulong param_5)

{
  astruct_264 *iVar1;
  undefined2 uVar1;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_264 *)param_1;
  iVar1->field_0xe = param_5;
  iVar1->field_0x12 = param_4;
  *param_1 = (ushort)&PTR_LOOP_1050_4a9a;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x2;
  return;
}



void __stdcall16far pass1_1018_4808(ushort *param_1,ulong param_2,ulong param_3,ulong param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0xe) = param_4;
  *param_1 = (ushort)&PTR_LOOP_1050_4aa2;
  *(undefined2 *)(iVar1 + 0x2) = 0x1018;
  *(undefined2 *)(iVar1 + 0xc) = 0x3;
  return;
}



ushort * __stdcall16far struct_1018_4842(ushort *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  astruct_265 *iVar1;
  undefined2 uVar1;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_265 *)param_1;
  iVar1->field_0xe = param_4;
  iVar1->field_0x10 = 0x0;
  *param_1 = (ushort)&PTR_LOOP_1050_4a8e;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x4;
  return param_1;
}



void __stdcall16far pass1_1018_4882(ushort *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = (ushort)&PTR_LOOP_1050_4a8e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x10),0x1000);
  pass1_1018_4760(param_1);
  return;
}



ushort * __stdcall16far struct_1018_48b0(ushort *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  astruct_207 *iVar1;
  undefined2 uVar1;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_207 *)param_1;
  iVar1->field_0xe = param_4;
  *param_1 = (ushort)&PTR_LOOP_1050_4a96;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x5;
  return param_1;
}



ushort * __stdcall16far struct_1018_48e8(ushort *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ushort *)(iVar1 + 0xe) = param_4;
  *param_1 = 0x4a9e;
  *(undefined2 *)(iVar1 + 0x2) = 0x1018;
  *(undefined2 *)(iVar1 + 0xc) = 0x6;
  return param_1;
}



void __stdcall16far struct_1018_4920(ushort *param_1,ulong param_2,ulong param_3,ulong param_4)

{
  astruct_203 *iVar1;
  undefined2 uVar1;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  iVar1->field_0xe = param_4;
  *param_1 = (ushort)&PTR_LOOP_1050_4a8a;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x7;
  return;
}



ushort * __stdcall16far pass1_1018_495a(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_4980(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_49a6(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_49cc(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_49f2(ushort *param_1,byte param_2)

{
  pass1_1018_4882(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_4a18(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_4a3e(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}
