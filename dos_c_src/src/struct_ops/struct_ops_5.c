
ushort *__stdcall16far pass1_1010_3702(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)(param_1 + 0xa)        = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x37c4;
    *(undefined2 *)(param_1 + 0x2)        = 0x1010;
    return (ushort *)CONCAT22(param_2, param_1);
}

ushort *__stdcall16far pass1_1010_37d4(ushort *param_1)

{
    undefined2 uVar1;

    struct_1010_383a(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x16) = 0x0;
    *param_1                             = 0x3b3e;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1010;
    return param_1;
}

void __stdcall16far struct_1010_383a(ushort *param_1)

{
    astruct_223 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_223 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = 0x0;
    iVar1->field_0x8  = 0x0;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    *param_1          = 0x3b5e;
    iVar1->field_0x2  = 0x1010;
    return;
}

void __stdcall16far struct_1010_3b7a(astruct_648 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x389a;
    param_1->field_0xc                        = 0x1008;
    param_1->field_0xa                        = 0x3aa8;
    param_1->field_0xc                        = 0x1008;
    param_1->field_0xe                        = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x14                       = 0x0;
    param_1->field_0x16                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x3d6a;
    param_1->field_0x2                        = 0x1010;
    param_1->field_0xa                        = 0x3d7a;
    param_1->field_0xc                        = 0x1010;
    return;
}

ushort *__stdcall16far pass1_1010_2bfc(astruct_644 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                    = 0x0;
    param_1->field_0xc                    = 0x0;
    param_1->field_0xe                    = 0x0;
    param_1->field_0x10                   = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x2cc2;
    param_1->field_0x2                    = 0x1010;
    return (ushort *)CONCAT22(param_2, param_1);
}

astruct_79 *__stdcall16far struct_op_1010_1d48(astruct_79 *param_1, ushort param_2)

{
    astruct_79 *iVar1;
    astruct_79 *uVar1;

    uVar1              = (astruct_79 *)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_79 *)param_1;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    iVar1->field_0x4   = 0x0;
    iVar1->field_0x8   = param_2;
    param_1->field_0x0 = 0x2014;
    iVar1->field_0x2   = 0x1010;
    return param_1;
}

ulong __stdcall16far pass1_1010_0eac(uchar *param_1, uchar *param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    struct_op_1018_4cda((int)param_1, (ushort)param_2, param_3);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xf0c;
    *(undefined2 *)(param_1 + 0x2)            = 0x1010;
    PTR_LOOP_1050_4230                        = param_1;
    PTR_LOOP_1050_4232                        = param_2;
    pass1_1018_4dce((ulong *)CONCAT22(param_2, param_1), 0xff, param_4, param_5);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1010_0f24(astruct_79 *param_1, astruct_79 *param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    int     unaff_DI;
    ushort *puVar1;

    struct_1010_2cd2(param_1, param_2, param_3, param_5);
    *(undefined4 *)((int)&param_1[0x9].field_0x4 + 0x2) = 0x0;
    *(undefined4 *)(param_1 + 0xa)                      = 0x0;
    *(undefined2 *)&param_1[0xa].field_0x4              = 0x0;
    *(undefined4 *)((int)&param_1[0xa].field_0x4 + 0x2) = 0x0;
    *(int *)CONCAT22(param_2, param_1)                  = (int)s_648_bmp_1050_1919 + 0x1;
    param_1->field_0x2                                  = 0x1010;
    puVar1                                              = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_5, param_4, unaff_DI);
    *(undefined2 *)((int)&param_1[0xa].field_0x4 + 0x2) = (int)puVar1;
    param_1[0xa].field_0x8                              = (ushort)((ulong)puVar1 >> 0x10);
    return;
}


void __stdcall16far struct_1010_0f9c(ulong *param_1, ushort param_2, ushort param_3)

{
    code       **ppcVar1;
    ushort       uVar2;
    undefined   *puVar3;
    undefined   *puVar4;
    undefined4   uVar5;
    uchar       *extraout_DX;
    uchar       *puVar6;
    uchar       *puVar7;
    uchar       *extraout_DX_00;
    undefined2   extraout_DX_01;
    undefined2   extraout_DX_02;
    undefined2   extraout_DX_03;
    uchar       *extraout_DX_04;
    astruct_232 *iVar8;
    astruct_231 *iVar9;
    astruct_230 *iVar10;
    astruct_233 *iVar11;
    undefined2   uVar8;
    undefined2   uVar9;
    ulong       *puVar10;
    undefined4   uVar11;
    ulong       *puVar12;
    undefined    uVar13;
    undefined4   uStack36;
    int          iStack32;
    ushort       uStack30;
    uint        *puStack28;
    undefined4   uStack24;
    undefined    local_14[0x12];

    uVar8   = (undefined2)((ulong)param_1 >> 0x10);
    iVar8   = (astruct_232 *)param_1;
    ppcVar1 = (code **)((int)*param_1 + 0x38);
    (**ppcVar1)();
    iVar8->field_0x68 = param_2;
    if((*(long *)&iVar8->field_0x60 != 0x0) && (iVar8->field_0x68 == 0x1))
    {
        return;
    }
    if(iVar8->field_0x68 == 0x0)
    {
        return;
    }
    puVar7 = extraout_DX;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    uVar2 = iVar8->field_0x68 * 0x18;
    mem_op_1000_179c(uVar2, puVar7, 0x1000);
    iVar8->field_0x60 = uVar2;
    iVar8->field_0x62 = puVar7;
    puStack28         = (uint *)CONCAT22(puVar7, iVar8->field_0x60);
    uStack30          = iVar8->field_0x68;
    do
    {
        do
        {
            puVar6 = puVar7;
            puVar3 = local_14;
            pass1_1028_e4ec(CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, puVar3)));
            uStack24 = CONCAT22(puVar6, puVar3);
            puVar7   = (uchar *)((uint)puVar6 | (uint)puVar3);
            if(puVar7 == (uchar *)0x0)
                goto LAB_1010_10ca;
            iVar9   = (astruct_231 *)*param_1;
            ppcVar1 = (code **)&iVar9->field_0x40;
            puVar4  = puVar3;
            (**ppcVar1)();
            puVar7 = extraout_DX_00;
        } while(puVar4 == (undefined *)0x0);
        uVar13 = SUB21(puVar6, 0x0);
        pass1_1028_b58e(CONCAT13((char)((uint)puVar6 >> 0x8), CONCAT12(uVar13, puVar3)));
        uStack36 = CONCAT22(extraout_DX_01, puVar4);
        ppcVar1  = (code **)&iVar9->field_0x2c;
        puVar12  = param_1;
        (**ppcVar1)();
        uVar9             = (undefined2)((ulong)puStack28 >> 0x10);
        iVar10            = (astruct_230 *)puStack28;
        *puStack28        = (uint)puVar4;
        iVar10->field_0x2 = extraout_DX_02;
        ppcVar1           = (code **)&iVar9->field_0x30;
        puVar10           = param_1;
        uVar11            = uStack24;
        (**ppcVar1)();
        iVar10->field_0x8 = puVar4;
        iVar10->field_0xa = extraout_DX_03;
        iVar10->field_0xc = uStack36;
        ppcVar1           = (code **)&iVar9->field_0x3c;
        uVar5             = uStack36;
        (**ppcVar1)((int)&USHORT_1050_1028, param_1, uStack24, puVar10, uVar11, puVar12, puVar3, uVar13);
        iVar10->field_0x10 = (int)uVar5;
        iVar10->field_0x12 = extraout_DX_04;
        iVar10->field_0x14 = uStack36;
        puStack28          = (uint *)((ulong)puStack28 & 0xffff0000 | ZEXT24(iVar10 + 0x1));
        uStack30           = uStack30 - 0x1;
        puVar7             = extraout_DX_04;
    } while(uStack30 != 0x0);
LAB_1010_10ca:
    uVar2 = iVar8->field_0x68 << 0x2;
    mem_op_1000_179c(uVar2, puVar7, 0x1000);
    iVar8->field_0x64 = uVar2;
    iVar8->field_0x66 = puVar7;
    iStack32          = 0x0;
    uStack30          = 0x0;
    while(true)
    {
        if((int)(iVar8->field_0x68 * 0x3) <= (int)uStack30)
            break;
        puVar7                                     = iVar8->field_0x62;
        uVar5                                      = *(undefined4 *)&iVar8->field_0x64;
        uVar9                                      = (undefined2)((ulong)uVar5 >> 0x10);
        iVar11                                     = (astruct_233 *)uVar5;
        *(ushort *)(iVar11 + iStack32 * 0x4)       = iVar8->field_0x60 + uStack30 * 0x8;
        *(uchar **)(iVar11 + iStack32 * 0x4 + 0x2) = puVar7;
        uStack30                                   = uStack30 + 0x3;
        iStack32                                   = iStack32 + 0x1;
    }
    return;
}

undefined2 *__stdcall16far pass1_1008_eabc(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined2 *)(param_1 + 0xa) = 0x0;
    pass1_1008_3e38((ushort *)CONCAT22(param_2, param_1 + 0xc));
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xeb1a;
    *(undefined2 *)(param_1 + 0x2)            = 0x1008;
    return (undefined2 *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1008_eb2a(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined2 *)(param_1 + 0xa)            = 0x0;
    *(undefined4 *)(param_1 + 0xc)            = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xec00;
    *(undefined2 *)(param_1 + 0x2)            = 0x1008;
    return;
}

ushort *__stdcall16far pass1_1008_ec10(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined2 *)(param_1 + 0xa)        = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xec62;
    *(undefined2 *)(param_1 + 0x2)        = 0x1008;
    return (ushort *)CONCAT22(param_2, param_1);
}

ushort *__stdcall16far struct_1008_ec72(ushort *param_1)

{
    struct_1010_383a(param_1);
    *param_1                            = 0xefc4;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    return param_1;
}

void __stdcall16far pass1_1008_ee14(ulong param_1, ushort param_2)

{
    undefined *puVar1;
    undefined2 uVar2;
    int        iVar3;
    undefined2 uVar4;
    ushort    *puVar5;
    undefined  local_1c[0x1a];

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0x56) == 0x0)
    {
        puVar5 = struct_1008_ec72((ushort *)CONCAT22(param_2, local_1c));
        uVar2  = (undefined2)((ulong)puVar5 >> 0x10);
        puVar1 = local_1c;
        pass1_1010_398e((ulong *)CONCAT22(param_2, puVar1), 0x0, 0x0, 0x0, (ushort)puVar1);
        *(ushort *)(iVar3 + 0x56)     = (ushort)puVar1;
        *(undefined2 *)(iVar3 + 0x58) = uVar2;
        pass1_1008_ec94((ushort *)CONCAT22(param_2, local_1c));
    }
    return;
}


ushort * __stdcall16far pass1_1008_d72e(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined2 *)(param_1 + 0xa) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xd780;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  return (ushort *)CONCAT22(param_2,param_1);
}


void __stdcall16far pass1_1008_d818(ulong param_1,int param_2)

{
  astruct_732 *iVar1;
  undefined2 uVar1;
  
  if (param_2 - 0x1a0U < 0x15) {
    iVar1 = (astruct_732 *)param_1;
    uVar1 = (undefined2)(param_1 >> 0x10);
    switch(param_2) {
    case 0x1a0:
      iVar1->field_0xe = 0x14;
      break;
    case 0x1a1:
      iVar1->field_0xe = 0x3;
      break;
    case 0x1a2:
      iVar1->field_0xe = 0x2;
      break;
    case 0x1a3:
      iVar1->field_0xe = 0xe;
      break;
    case 0x1a4:
      iVar1->field_0xe = 0xc;
      break;
    case 0x1a5:
      iVar1->field_0xe = 0x4;
      break;
    case 0x1a6:
      iVar1->field_0xe = 0xb;
      break;
    case 0x1a7:
      iVar1->field_0xe = 0x6;
      break;
    case 0x1a8:
      iVar1->field_0xe = 0xa;
      break;
    case 0x1a9:
      iVar1->field_0xe = 0xd;
      break;
    case 0x1aa:
      iVar1->field_0xe = 0x13;
      break;
    case 0x1ab:
      iVar1->field_0xe = 0x5;
      break;
    case 0x1ac:
      iVar1->field_0xe = 0x9;
      break;
    case 0x1ad:
      iVar1->field_0xe = 0x8;
      break;
    case 0x1ae:
      iVar1->field_0xe = 0x12;
      break;
    case 0x1af:
      iVar1->field_0xe = 0x11;
      break;
    case 0x1b0:
      iVar1->field_0xe = 0x7;
      return;
    case 0x1b1:
      iVar1->field_0xe = 0x10;
      return;
    case 0x1b2:
      iVar1->field_0xe = 0x1;
      return;
    case 0x1b3:
      iVar1->field_0xe = 0xf;
      return;
    case 0x1b4:
      iVar1->field_0xe = 0x15;
      return;
    }
  }
  return;
}


void __stdcall16far pass1_1008_d99e(int param_1,ushort param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  struct_op_1018_4cda(param_1,param_2,param_3);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xd9fa;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x9a,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}

void __stdcall16far struct_1008_dc90(ushort *param_1,ulong param_2,ulong param_3)

{
  astruct_212 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_212 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x0;
  *param_1 = 0xdd4a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far struct_1008_dcdc(ushort *param_1)

{
  astruct_220 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_220 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x0;
  *param_1 = 0xdd4a;
  iVar1->field_0x2 = 0x1008;
  return;
}

void __stdcall16far
pass1_1008_e05e(ulong param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  undefined local_122 [0x112];
  int iStack16;
  undefined local_e [0x8];
  long lStack6;
  
  lStack6 = pass1_1008_e8cc(param_5,param_1,param_3,param_4);
  if (lStack6 != 0x0) {
    uVar3 = pass1_1030_8326();
    uVar2 = (undefined2)((ulong)lStack6 >> 0x10);
    iVar1 = (int)lStack6;
    *(undefined2 *)(iVar1 + 0xe) = (int)uVar3;
    *(undefined2 *)(iVar1 + 0x10) = (int)(uVar3 >> 0x10);
    *(ushort *)(iVar1 + 0xc) = param_2;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_e),*(ulong *)((int)param_1 + 0xa));
  iStack16 = 0x0;
  do {
    lStack6 = pass1_1008_5b12(local_e,param_5);
    if (lStack6 == 0x0) goto LAB_1008_e0d3;
  } while (*(int *)((int)lStack6 + 0xc) != 0x1);
  iStack16 = 0x1;
LAB_1008_e0d3:
  if (iStack16 == 0x0) {
    struct_1030_e2be((astruct_100 *)CONCAT22(param_5,local_122),0x0,0x0,0x0,param_5,param_6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_5,local_122));
  }
  return;
}

void __stdcall16far pass1_1008_e164(ulong param_1,ushort param_2,uchar param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  astruct_215 *uVar5;
  astruct_215 *paVar3;
  astruct_216 *paVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  uint uVar8;
  astruct_214 *uVar11;
  astruct_215 *paVar9;
  astruct_213 *iVar12;
  ushort uVar10;
  undefined2 uVar12;
  ulong uVar13;
  undefined local_118 [0x112];
  long lStack6;
  astruct_216 *iVar1;
  
  uVar10 = (ushort)(param_1 >> 0x10);
  uVar11 = (astruct_214 *)param_1;
  lStack6 = pass1_1008_e8cc(param_2,param_1,uVar11->field_0x1a,uVar11->field_0x16);
  uVar8 = (uint)((ulong)lStack6 >> 0x10);
  uVar5 = (astruct_215 *)lStack6;
  puVar5 = (uchar *)(uVar8 | (uint)uVar5);
  if (lStack6 == 0x0) {
    pass1_1008_e852((ushort)uVar11,uVar10,uVar11->field_0x16,param_2,(uint)puVar5);
    paVar3 = uVar5;
    puVar6 = puVar5;
    pass1_1008_e852((ushort)uVar11,uVar10,uVar11->field_0x1a,param_2,(uint)puVar5);
    paVar9 = paVar3;
    puVar7 = puVar6;
    mem_op_1000_179c(0x14,puVar6,0x1000);
    uVar8 = (uint)puVar7 | (uint)paVar9;
    if (uVar8 == 0x0) {
      paVar9 = (astruct_215 *)0x0;
      uVar8 = 0x0;
    }
    else {
      struct_1008_dc90((ushort *)CONCAT22(puVar7,paVar9),
                       CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,paVar3)),CONCAT22(puVar5,uVar5));
    }
    lStack6 = CONCAT22(uVar8,paVar9);
    paVar9->field_0xc = 0x1;
    uVar13 = pass1_1030_8326();
    uVar12 = (undefined2)((ulong)lStack6 >> 0x10);
    iVar12 = (astruct_213 *)lStack6;
    iVar12->field_0xe = (int)uVar13;
    iVar12->field_0x10 = (int)(uVar13 >> 0x10);
    puVar1 = uVar11->field_0xa;
    ppcVar2 = (code **)((int)*uVar11->field_0xa + 0x4);
    (**ppcVar2)(0x1030,(char)puVar1,(char)((ulong)puVar1 >> 0x10),iVar12,uVar12);
  }
  else {
    iVar1 = (astruct_216 *)uVar5->field_0xc;
    paVar4 = iVar1 + -0x1;
    if (paVar4 == (astruct_216 *)0x0) {
      return;
    }
    if (((0x0 < (int)paVar4) && (!SBORROW2((int)paVar4,0x1))) && ((int)(iVar1 + -0x2) < 0x2)) {
      uVar5->field_0x12 = 0x1;
    }
    uVar5->field_0xc = 0x1;
  }
  uVar12 = (undefined2)((ulong)lStack6 >> 0x10);
  struct_1030_e2be((astruct_100 *)CONCAT22(param_2,local_118),0x1,*(ulong *)((int)lStack6 + 0x8),
                   *(ulong *)((int)lStack6 + 0x4),param_2,param_3);
  uVar13 = pass1_1030_8326();
  pass1_1030_8372(_PTR_LOOP_1050_5748,uVar13 + 0x1,(ulong *)CONCAT22(param_2,local_118));
  return;
}



void __stdcall16far pass1_1008_c72a(astruct_642 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xca4a;
  param_1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_ca5a(astruct_639 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xd71a;
  param_1->field_0x2 = 0x1008;
  return;
}


void __stdcall16far struct_1008_bde0(ulong *param_1,uchar *param_2)

{
  ushort uVar1;
  astruct_139 *iVar2;
  astruct_140 *iVar3;
  astruct_141 *iVar4;
  astruct_142 *iVar5;
  astruct_143 *iVar6;
  astruct_144 *iVar7;
  astruct_145 *iVar8;
  astruct_146 *iVar9;
  astruct_147 *iVar10;
  astruct_148 *iVar11;
  astruct_149 *iVar12;
  astruct_150 *iVar2_00;
  astruct_151 *iVar2_01;
  astruct_152 *iVar2_02;
  astruct_153 *iVar2_03;
  astruct_154 *iVar2_04;
  astruct_155 *iVar2_05;
  int iVar2_06;
  ushort uVar3;
  undefined2 uVar13;
  
  _PTR_LOOP_1050_06e0 = param_1;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_2,0x1000);
    PTR_LOOP_1050_5f2e = param_2;
  }
  else {
  }
  uVar1 = fn_ptr_op_1000_1708(0x1aa,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  *(ushort *)param_1 = uVar1;
  *(uchar **)((int)param_1 + 0x2) = PTR_LOOP_1050_5f2e;
  uVar3 = (ushort)(*param_1 >> 0x10);
  iVar2 = (astruct_139 *)*param_1;
  iVar2->field_0x6 = 0x6e4;
  iVar2->field_0x8 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xa) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar3 = (astruct_140 *)*param_1;
  iVar3->field_0xc = 0x6ea;
  iVar3->field_0xe = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x10) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar4 = (astruct_141 *)*param_1;
  iVar4->field_0x12 = 0x6ee;
  iVar4->field_0x14 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x16) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar5 = (astruct_142 *)*param_1;
  iVar5->field_0x18 = 0x6f2;
  iVar5->field_0x1a = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x1c) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar6 = (astruct_143 *)*param_1;
  iVar6->field_0x1e = 0x6f6;
  iVar6->field_0x20 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x22) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar7 = (astruct_144 *)*param_1;
  iVar7->field_0x24 = 0x6fe;
  iVar7->field_0x26 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x28) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar8 = (astruct_145 *)*param_1;
  iVar8->field_0x2a = 0x702;
  iVar8->field_0x2c = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x2e) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar9 = (astruct_146 *)*param_1;
  iVar9->field_0x30 = 0x708;
  iVar9->field_0x32 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x34) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar10 = (astruct_147 *)*param_1;
  iVar10->field_0x36 = 0x70e;
  iVar10->field_0x38 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x3a) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar11 = (astruct_148 *)*param_1;
  iVar11->field_0x3c = 0x714;
  iVar11->field_0x3e = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x40) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar12 = (astruct_149 *)*param_1;
  iVar12->field_0x42 = 0x71a;
  iVar12->field_0x44 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x46) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_00 = (astruct_150 *)*param_1;
  iVar2_00->field_0x48 = 0x71e;
  iVar2_00->field_0x4a = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x4c) = 0x7;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_01 = (astruct_151 *)*param_1;
  iVar2_01->field_0x4e = 0x72c;
  iVar2_01->field_0x50 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x52) = 0x6;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_02 = (astruct_152 *)*param_1;
  iVar2_02->field_0x54 = 0x738;
  iVar2_02->field_0x56 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x58) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_03 = (astruct_153 *)*param_1;
  iVar2_03->field_0x5a = 0x73e;
  iVar2_03->field_0x5c = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x5e) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_04 = (astruct_154 *)*param_1;
  iVar2_04->field_0x60 = 0x744;
  iVar2_04->field_0x62 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x64) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_05 = (astruct_155 *)*param_1;
  iVar2_05->field_0x66 = 0x74c;
  iVar2_05->field_0x68 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x6a) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x6c) = 0x750;
  *(undefined2 *)(iVar2_06 + 0x6e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x70) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x72) = 0x756;
  *(undefined2 *)(iVar2_06 + 0x74) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x76) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x78) = 0x75a;
  *(undefined2 *)(iVar2_06 + 0x7a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x7c) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x7e) = 0x75e;
  *(undefined2 *)(iVar2_06 + 0x80) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x82) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x84) = 0x764;
  *(undefined2 *)(iVar2_06 + 0x86) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x88) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x8a) = 0x76a;
  *(undefined2 *)(iVar2_06 + 0x8c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x8e) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x90) = 0x770;
  *(undefined2 *)(iVar2_06 + 0x92) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x94) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x96) = 0x774;
  *(undefined2 *)(iVar2_06 + 0x98) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x9a) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x9c) = 0x77c;
  *(undefined2 *)(iVar2_06 + 0x9e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xa0) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xa2) = 0x780;
  *(undefined2 *)(iVar2_06 + 0xa4) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xa6) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xa8) = 0x782;
  *(undefined2 *)(iVar2_06 + 0xaa) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xac) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xae) = 0x786;
  *(undefined2 *)(iVar2_06 + 0xb0) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xb2) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xb4) = 0x78a;
  *(undefined2 *)(iVar2_06 + 0xb6) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xb8) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xba) = 0x78e;
  *(undefined2 *)(iVar2_06 + 0xbc) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xbe) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xc0) = 0x792;
  *(undefined2 *)(iVar2_06 + 0xc2) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xc4) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xc6) = 0x796;
  *(undefined2 *)(iVar2_06 + 0xc8) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xca) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xcc) = 0x79e;
  *(undefined2 *)(iVar2_06 + 0xce) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xd0) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xd2) = 0x7a0;
  *(undefined2 *)(iVar2_06 + 0xd4) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xd6) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xd8) = 0x7a4;
  *(undefined2 *)(iVar2_06 + 0xda) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xdc) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xde) = 0x7a6;
  *(undefined2 *)(iVar2_06 + 0xe0) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xe2) = 0x6;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xe4) = 0x7b2;
  *(undefined2 *)(iVar2_06 + 0xe6) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xe8) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xea) = 0x7b4;
  *(undefined2 *)(iVar2_06 + 0xec) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xee) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xf0) = 0x7ba;
  *(undefined2 *)(iVar2_06 + 0xf2) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xf4) = 0x2d;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xf6) = 0x814;
  *(undefined2 *)(iVar2_06 + 0xf8) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xfa) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xfc) = 0x81a;
  *(undefined2 *)(iVar2_06 + 0xfe) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x100) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x102) = 0x81c;
  *(undefined2 *)(iVar2_06 + 0x104) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x106) = 0x4b;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x108) = 0x8b2;
  *(undefined2 *)(iVar2_06 + 0x10a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x10c) = 0x6;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x10e) = 0x8be;
  *(undefined2 *)(iVar2_06 + 0x110) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x112) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x11a) = 0x8c6;
  *(undefined2 *)(iVar2_06 + 0x11c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x11e) = 0x35;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x120) = 0x930;
  *(undefined2 *)(iVar2_06 + 0x122) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x124) = 0x2e;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x114) = 0x98c;
  *(undefined2 *)(iVar2_06 + 0x116) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x118) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x126) = 0x98e;
  *(undefined2 *)(iVar2_06 + 0x128) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x12a) = 0x9;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x12c) = 0x9a0;
  *(undefined2 *)(iVar2_06 + 0x12e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x130) = 0x1a;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x132) = 0x9d4;
  *(undefined2 *)(iVar2_06 + 0x134) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x136) = 0x8;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x138) = 0x9e4;
  *(undefined2 *)(iVar2_06 + 0x13a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x13c) = 0x4a;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x144) = 0xa78;
  *(undefined2 *)(iVar2_06 + 0x146) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x148) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x14a) = 0xa7c;
  *(undefined2 *)(iVar2_06 + 0x14c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x14e) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x150) = 0xa7e;
  *(undefined2 *)(iVar2_06 + 0x152) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x154) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x156) = 0xa80;
  *(undefined2 *)(iVar2_06 + 0x158) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x15a) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x15c) = 0xa86;
  *(undefined2 *)(iVar2_06 + 0x15e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x160) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x168) = 0xa8a;
  *(undefined2 *)(iVar2_06 + 0x16a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x16c) = 0x1b;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x16e) = 0xac0;
  *(undefined2 *)(iVar2_06 + 0x170) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x172) = 0x16;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x174) = 0xaec;
  *(undefined2 *)(iVar2_06 + 0x176) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x178) = 0x3e;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x17a) = 0xb68;
  *(undefined2 *)(iVar2_06 + 0x17c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x17e) = 0x46;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x180) = 0xbf4;
  *(undefined2 *)(iVar2_06 + 0x182) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x184) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x186) = 0xbf6;
  *(undefined2 *)(iVar2_06 + 0x188) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x18a) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x18c) = 0xbfc;
  *(undefined2 *)(iVar2_06 + 0x18e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x190) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x192) = 0xc02;
  *(undefined2 *)(iVar2_06 + 0x194) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x196) = 0xa;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x198) = 0xc16;
  *(undefined2 *)(iVar2_06 + 0x19a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x19c) = 0x24;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x19e) = 0xc5e;
  *(undefined2 *)(iVar2_06 + 0x1a0) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x1a2) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x1a4) = 0xc62;
  *(undefined2 *)(iVar2_06 + 0x1a6) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x1a8) = 0x44;
  return;
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


void __stdcall16far set_struct_op_1008_9584(astruct_20 *param_1,ULONG param_2)

{
  astruct_20 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_2;
  param_1->field_0x0 = 0x9d2e;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xac = 0x2000000;
  iVar1->field_0xb0 = 0x0;
  iVar1->field_0xb4 = 0x8000;
  iVar1->field_0xb6 = 0x8000;
  iVar1->field_0xb8 = 0x8000;
  iVar1->field_0xba = 0x8000;
  iVar1->field_0xbc = 0x0;
  iVar1->field_0xbe = 0x0;
  iVar1->field_0xc2 = 0x0;
  iVar1->hcursor_field_0xc4 = 0x0;
  iVar1->hgdiobj_field_0xc6 = 0x0;
  iVar1->field_0xc8 = 0x2008;
  iVar1->field_0xca = 0x0;
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x5b = '\0';
  *(undefined *)&iVar1->field_0xa = 0x0;
  return;
}




void __stdcall16far struct_op_1008_8e9e(astruct_78 *param_1,ulong param_2,ulong param_3)

{
  astruct_78 *iVar1;
  astruct_78 *uVar1;
  ushort unaff_SS;
  
  uVar1 = (astruct_78 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_78 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = (ulong *)0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xe = param_3;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x16 = param_2;
  iVar1->field_0x1a = 0x1;
  param_1->field_0x0 = 0x9170;
  iVar1->field_0x2 = 0x1008;
  if (iVar1->field_0xe < 0x7) {
    iVar1->field_0xe = 0x6;
  }
  pass1_1008_909c((ulong)param_1,unaff_SS);
  *iVar1->field_0x6 = 0x0;
  return;
}


void __stdcall16far struct_op_1008_9174(astruct_88 *param_1,ulong param_2,ulong param_3)

{
  astruct_88 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_88 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = param_2;
  iVar1->field_0x10 = 0x0;
  *(undefined2 *)param_1 = 0x9412;
  iVar1->field_0x2 = 0x1008;
  return;
}

void __stdcall16far set_struct_1008_687a(astruct_20 *param_1,ULONG param_2)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;
  
  set_struct_op_1008_9584(param_1,param_2);
  uVar1 = (astruct_20 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1->field_0xcc = 0x0;
  iVar1->field_0xce = 0x0;
  set_struct_1008_574a((astruct_21 *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0xd2)));
  param_1->field_0x0 = 0x6bfc;
  iVar1->field_0x2 = 0x1008;
  (iVar1->field_0xd2).field_0xa = 0x0;
  return;
}

ushort __cdecl16far str_op_1008_60e8(char *param_1,ushort param_2)

{
  uint uVar1;
  
  if (param_1 != (char *)0x0) {
    uVar1 = str_op_1000_3da4(param_1);
    uVar1 = uVar1 + 0x1;
    mem_op_1000_179c(uVar1,(uchar *)param_2,0x1000);
    if ((param_2 | uVar1) != 0x0) {
      unk_str_op_1000_3d3e((char *)CONCAT22(param_2,uVar1),param_1);
      return uVar1;
    }
  }
  return 0x0;
}


void __stdcall16far struct_1008_4c58(ushort *param_1)

{
  astruct_394 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_394 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x1;
  *param_1 = 0x4f1c;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far struct_op_1008_4c98(astruct_76 *param_1,ulong param_2,ushort param_3)

{
  astruct_76 *iVar1;
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *(ulong *)&iVar1->field_0x4 = param_2;
  iVar1->field_0xc = param_3;
  iVar1->field_0xe = (undefined4 *)0x0;
  iVar1->field_0x12 = 0x0;
  param_1->field_0x0 = 0x4f1c;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_5068(astruct_76 *param_1,astruct_83 *param_2)

{
  struct_op_1008_4214(param_1,param_2);
  return;
}



ushort * __stdcall16far struct_op_1008_56b4(astruct_76 *param_1)

{
  astruct_82 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_82 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  param_1->field_0x0 = (ushort)s__s__d_1050_573a;
  iVar1->field_0x2 = 0x1008;
  return &param_1->field_0x0;
}


void __stdcall16far set_struct_1008_574a(astruct_21 *param_1)

{
  astruct_21 *iVar1;
  astruct_21 *uVar1;
  
  uVar1 = (astruct_21 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_21 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xa = 0x1;
  param_1->field_0x0 = 0x5bc4;
  iVar1->field_0x2 = 0x1008;
  return;
}

void __stdcall16far struct_op_1008_3f92(astruct_76 *param_1,astruct_83 *param_2)

{
  code **ppcVar1;
  astruct_76 *iVar2;
  undefined2 uVar2;
  
  struct_op_1008_56b4(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  *(undefined4 *)&iVar2->field_0x6 = 0x0;
  *(undefined4 *)((int)&iVar2->field_0x8 + 0x2) = 0x0;
  *(undefined2 *)&iVar2->field_0xe = 0x0;
  *(undefined4 *)((int)&iVar2->field_0xe + 0x2) = 0x0;
  iVar2->field_0x14 = 0x0;
  *(undefined4 *)&iVar2->field_0x18 = 0x0;
  iVar2->field_0x1c = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_48de;
  iVar2->field_0x2 = 0x1008;
  if (param_2 == (astruct_83 *)0x0) {
    return;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)param_2 + 0x8);
  (**ppcVar1)();
  struct_op_1008_4214(param_1,param_2);
  pass1_1008_47cc(param_1);
  pass1_1008_4834(param_1);
  return;
}



void __stdcall16far pass1_1008_4016(astruct_76 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_op_1008_56b4(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x6) = 0x0;
  *(undefined4 *)(iVar1 + 0xa) = 0x0;
  *(undefined2 *)(iVar1 + 0xe) = 0x0;
  *(undefined4 *)(iVar1 + 0x10) = 0x0;
  *(undefined4 *)(iVar1 + 0x14) = 0x0;
  *(undefined4 *)(iVar1 + 0x18) = 0x0;
  *(undefined2 *)(iVar1 + 0x1c) = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_48de;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far struct_op_1008_4214(astruct_76 *param_1,astruct_83 *param_2)

{
  ulong *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_83 *iVar4;
  astruct_83 *uVar4;
  
  uVar4 = (astruct_83 *)((ulong)param_2 >> 0x10);
  iVar4 = (astruct_83 *)param_2;
  *(undefined4 *)((int)param_1 + 0x6) = iVar4->field_0x1a;
  iVar4->field_0x1a = 0x0;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x4 = 0x0;
  iVar4->field_0xe = 0x0;
  iVar4->field_0x12 = 0x0;
  iVar4->field_0x16 = 0x0;
  iVar4->field_0x1e = 0x0;
  return;
}



astruct_20 * __stdcall16far pass1_1008_3ab8(astruct_20 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  set_struct_1008_687a(param_1,0x0);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0xde) = 0x0;
  param_1->field_0x0 = 0x3b46;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x5b)),s_SOLDefaultWindowClass_1050_01fe);
  return param_1;
}


void __stdcall16far struct_op_1008_0000(ushort *param_1)

{
  int iVar1;
  ushort uVar2;
  
                    // Segment:    2
                    // Offset:     000060e0
                    // Length:     efe0
                    // Min Alloc:  efe0
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x52a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *(undefined4 *)(iVar1 + 0x4) = 0x0;
  *(undefined4 *)(iVar1 + 0x8) = 0x0;
  *param_1 = 0x51e;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}

