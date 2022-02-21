
void __stdcall16far pass1_1030_1be2(ulong param_1, uint param_2, uchar *param_3)

{
    code     **ppcVar1;
    uint      *puVar2;
    uchar     *puVar3;
    uchar     *extraout_DX;
    uchar     *extraout_DX_00;
    int        iVar4;
    undefined2 uVar5;
    ulong      uVar6;
    uint       uStack4;

    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar3 = param_3;
    if(*(long *)(iVar4 + 0xc) == 0x0)
    {
        mem_op_1000_179c(0x18, param_3, 0x1000);
        puVar3 = (uchar *)((uint)param_3 | param_2);
        if(puVar3 == (uchar *)0x0)
        {
            *(undefined4 *)(iVar4 + 0xc) = 0x0;
        }
        else
        {
            struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3, param_2), 0x5, 0x5);
            *(uint *)(iVar4 + 0xc)   = param_2;
            *(uchar **)(iVar4 + 0xe) = extraout_DX;
            puVar3                   = extraout_DX;
        }
    }
    for(uStack4 = 0x0; puVar2 = *(uint **)(iVar4 + 0x10), uStack4 <= *puVar2 && *puVar2 != uStack4;
        uStack4               = uStack4 + 0x1)
    {
        uVar6   = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, (ushort)puVar3, 0x1);
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xc) + 0x8);
        (**ppcVar1)(
          (int)&USHORT_1050_1028, *(undefined4 *)(iVar4 + 0xc), (int)uVar6, (int)(uVar6 >> 0x10), uStack4, 0x0);
        puVar3 = extraout_DX_00;
    }
    return;
}


void __stdcall16far pass1_1028_ef00(
  ushort param_1, uchar *param_2, ulong param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7)

{
    ushort  uVar1;
    ushort *puVar2;

    if(param_5 == 0x4)
    {
        mem_op_1000_179c(0x16, param_2, 0x1000);
        uVar1 = (uint)param_2 | param_5;
        if(uVar1 != 0x0)
        {
            pass1_1030_b936((astruct_365 *)param_5, (ushort)param_2, 0x4, _param_6, uVar1);
            goto LAB_1028_ef8b;
        }
    }
    else
    {
        if(param_5 == 0xc)
        {
            mem_op_1000_179c(0xe, param_2, 0x1000);
            if(((uint)param_2 | param_5) != 0x0)
            {
                puVar2  = pass1_1030_bc24((uint)param_2 | param_5, param_5, (ushort)param_2, 0xc, _param_6);
                uVar1   = (ushort)((ulong)puVar2 >> 0x10);
                param_5 = (ushort)(astruct_365 *)puVar2;
                goto LAB_1028_ef8b;
            }
        }
        else
        {
            uVar1 = param_5;
            mem_op_1000_179c(0xe, param_2, 0x1000);
            if(((uint)param_2 | uVar1) != 0x0)
            {
                puVar2  = pass1_1028_b22c((ushort *)CONCAT22(param_2, uVar1), param_5, _param_6, (uint)param_2 | uVar1);
                uVar1   = (ushort)((ulong)puVar2 >> 0x10);
                param_5 = (ushort)(astruct_365 *)puVar2;
                goto LAB_1028_ef8b;
            }
        }
    }
    param_5 = 0x0;
    uVar1   = 0x0;
LAB_1028_ef8b:
    pass1_1030_1358(*(ulong *)((int)param_3 + 0x22),
                    param_5,
                    uVar1,
                    *(ulong *)(param_5 + 0x4) & 0xffff | (ulong)(*(uint *)(param_5 + 0x6) & 0xff) << 0x10,
                    param_1);
    return;
}


ushort *__stdcall16far switch_1030_07ac(ushort       param_1,
                                        ushort       param_2,
                                        ushort       param_3,
                                        ushort       param_4,
                                        ushort       param_5,
                                        ulong        param_6,
                                        uchar       *param_7,
                                        astruct_179 *param_8,
                                        ushort       param_9,
                                        ushort       param_10,
                                        ushort       param_11)

{
    ushort  uVar1;
    uchar  *puVar2;
    uint    uVar3;
    ushort *puVar4;

    puVar4 = (ushort *)CONCAT22(param_7, param_8);
    switch(param_4 - 0x1)
    {
    case 0x0:
    case 0x1:
    case 0x2:
    case 0x3:
    case 0x4:
    case 0x5:
    case 0x6:
    case 0x7:
    case 0x8:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_48c0((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x9:
        mem_op_1000_179c(0x22, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_2bfe(param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0xa:
        mem_op_1000_179c(0x26, param_7, 0x1000);
        uVar1 = (uint)param_7 | (uint)param_8;
        goto joined_r0x10300adb;
    case 0xb:
        mem_op_1000_179c(0x2c, param_7, 0x1000);
        if((uchar *)((uint)param_7 | (uint)param_8) != (uchar *)0x0)
        {
            puVar4 = pass1_1028_3692((int)param_8,
                                     (ushort)param_7,
                                     param_4,
                                     param_6,
                                     (uchar *)((uint)param_7 | (uint)param_8),
                                     param_9,
                                     param_10);
            return puVar4;
        }
        break;
    case 0xc:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_3580((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0xd:
        mem_op_1000_179c(0x26, param_7, 0x1000);
        if((uchar *)((uint)param_7 | (uint)param_8) != (uchar *)0x0)
        {
            puVar4 = pass1_1028_34a6(
              (int)param_8, (ushort)param_7, param_4, param_6, (uchar *)((uint)param_7 | (uint)param_8));
            return puVar4;
        }
        break;
    case 0xe:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_408e((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0xf:
    case 0x32:
    case 0x33:
    case 0x5f:
    case 0x60:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_0c50((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x10:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_0b64((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x11:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_4376((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x12:
    case 0x13:
    case 0x14:
    case 0x61:
    case 0x62:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_4ba6((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x15:
    case 0x16:
    case 0x17:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_1be8((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    default:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        uVar3 = (uint)param_7 | (uint)param_8;
        if(uVar3 != 0x0)
        {
            pass1_1028_b39e((ushort *)CONCAT22(param_7, param_8), param_4, param_6, uVar3);
            return (ushort *)CONCAT22(uVar3, param_8);
        }
        break;
    case 0x1a:
    case 0x1b:
    case 0x1c:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1030_be56((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x1d:
    case 0x1e:
    case 0x1f:
        mem_op_1000_179c(0x26, param_7, 0x1000);
        puVar2 = (uchar *)((uint)param_7 | (uint)param_8);
        if(puVar2 != (uchar *)0x0)
        {
            pass1_1028_00cc((int)param_8, (ushort)param_7, param_4, param_6, puVar2);
            return (ushort *)CONCAT22(puVar2, param_8);
        }
        break;
    case 0x20:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_50fa((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x21:
    case 0x22:
    case 0x23:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = (ushort *)pass1_1028_3ec8(
              (int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x24:
    case 0x25:
    case 0x26:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_d08e((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x27:
    case 0x28:
    case 0x5c:
    case 0x5d:
    case 0x5e:
        mem_op_1000_179c(0x22, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1030_c71e((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x29:
    case 0x2a:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_cd06((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x2b:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_26d6((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x2c:
    case 0x2d:
        mem_op_1000_179c(0x2a, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = (ushort *)pass1_1028_49de(
              (int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x2e:
    case 0x2f:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_e81c((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x30:
    case 0x31:
    case 0x6b:
    case 0x6c:
        mem_op_1000_179c(0x22, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_d3a4(
              (ushort *)CONCAT22(param_7, param_8), param_3, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x34:
    case 0x35:
        mem_op_1000_179c(0x2c, param_7, 0x1000);
        puVar2 = (uchar *)((uint)param_7 | (uint)param_8);
        if(puVar2 != (uchar *)0x0)
        {
            pass1_1028_3816((int)param_8, (ushort)param_7, param_4, param_6, puVar2, param_9, param_10);
            return (ushort *)CONCAT22(puVar2, param_8);
        }
        break;
    case 0x36:
        mem_op_1000_179c(0x26, param_7, 0x1000);
        uVar1 = (uint)param_7 | (uint)param_8;
    joined_r0x10300adb:
        if(uVar1 != 0x0)
        {
            pass1_1030_c09c((int)param_8, (ushort)param_7, param_4, param_6, uVar1);
            return (ushort *)CONCAT22(uVar1, param_8);
        }
        break;
    case 0x37:
    case 0x38:
        mem_op_1000_179c(0x9a, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = (ushort *)pass1_1030_c9e4(
              (int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x39:
    case 0x3a:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if((uchar *)((uint)param_7 | (uint)param_8) != (uchar *)0x0)
        {
            puVar4 = (ushort *)pass1_1028_611e(
              (int)param_8, (ushort)param_7, param_4, param_6, (uint)param_8, (uchar *)((uint)param_7 | (uint)param_8));
            return puVar4;
        }
        break;
    case 0x3b:
    case 0x3c:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_44fe((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x3d:
        mem_op_1000_179c(0x22, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_ce08(param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x3e:
        mem_op_1000_179c(0x26, param_7, 0x1000);
        puVar2 = (uchar *)((uint)param_7 | (uint)param_8);
        if(puVar2 != (uchar *)0x0)
        {
            pass1_1028_1fc8((int)param_8, (ushort)param_7, param_4, param_6, puVar2);
            return (ushort *)CONCAT22(puVar2, param_8);
        }
        break;
    case 0x3f:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_25fc((int)param_8, param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x40:
        mem_op_1000_179c(0x22, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_ca0c(param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x46:
    case 0x69:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_d5c8((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x47:
    case 0x48:
    case 0x49:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1020_d888((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x4b:
    case 0x4c:
    case 0x4d:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = (ushort *)pass1_1030_d942(
              (int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x4e:
    case 0x4f:
    case 0x50:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5c76((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x51:
    case 0x52:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5988((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x53:
    case 0x54:
    case 0x55:
        mem_op_1000_179c(0x22, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5f00((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x56:
    case 0x57:
    case 0x58:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_53e8((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x59:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_58a6((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x5a:
    case 0x5b:
        mem_op_1000_179c(0x26, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5546((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x63:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5e18((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x64:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5a6a((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x65:
    case 0x66:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_530a((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x67:
    case 0x68:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_57c8((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x6d:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_5652((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x6f:
    case 0x70:
    case 0x71:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) == 0x0)
        {
            puVar4 = (ushort *)0x0;
        }
        else
        {
            puVar4 = pass1_1020_d888((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
        }
    case 0x72:
    case 0x76:
        mem_op_1000_179c(0x26, (uchar *)((ulong)puVar4 >> 0x10), 0x1000);
        uVar3 = (uint)((ulong)puVar4 >> 0x10);
        if(puVar4 != (ushort *)0x0)
        {
            puVar4 = pass1_1020_e91e((uint)puVar4, uVar3, param_4, param_6, uVar3 | (uint)puVar4);
            return puVar4;
        }
        break;
    case 0x73:
    case 0x77:
    case 0x78:
        mem_op_1000_179c(0x2c, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = struct_1020_d99e(
              (ushort *)CONCAT22(param_7, param_8), param_3, param_4, param_6, (uint)param_7 | (uint)param_8, param_11);
            return puVar4;
        }
        break;
    case 0x74:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_17ae((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x75:
        mem_op_1000_179c(0x24, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_2b1c((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
        break;
    case 0x79:
    case 0x7a:
    case 0x7b:
    case 0x7c:
    case 0x7d:
    case 0x7e:
        mem_op_1000_179c(0x20, param_7, 0x1000);
        if(((uint)param_7 | (uint)param_8) != 0x0)
        {
            puVar4 = pass1_1028_2812((int)param_8, (ushort)param_7, param_4, param_6, (uint)param_7 | (uint)param_8);
            return puVar4;
        }
    }
    return (ushort *)0x0;
}


void __stdcall16far pass1_1028_e628(ulong  param_1,
                                    ushort param_2,
                                    ushort param_3,
                                    int    param_4,
                                    int    param_5,
                                    ushort param_6,
                                    ushort param_7,
                                    ushort param_8,
                                    ushort param_9,
                                    uchar  param_10)

{
    char        *pcVar1;
    int         *piVar2;
    char         cVar3;
    ulong        uVar4;
    ulong        uVar5;
    long         lVar6;
    code       **ppcVar7;
    undefined2  *puVar8;
    undefined2  *puVar9;
    undefined2   uVar10;
    BOOL16       BVar11;
    uint         uVar12;
    undefined4   uVar13;
    int          iVar14;
    undefined2  *extraout_DX;
    uint         extraout_DX_00;
    uchar       *extraout_DX_01;
    undefined2   uVar15;
    uint         uVar16;
    undefined2  *puVar17;
    astruct_348 *uVar18;
    astruct_349 *paVar18;
    ushort       uVar19;
    astruct_349 *uVar20;
    undefined2   uVar21;
    ushort       uVar22;
    bool         bVar23;
    bool         bVar24;
    ushort      *puVar25;
    astruct_99  *paVar26;
    undefined4  *puVar27;
    undefined2   local_154;
    undefined2   uStackY338;
    undefined2   local_14c;
    undefined2   uStackY330;
    undefined2   uStackY80;
    undefined2   uStackY78;
    undefined    uVar28;
    undefined    uVar29;
    undefined    uVar30;
    undefined    uVar31;
    undefined    uVar32;
    undefined    uVar33;
    undefined    uVar34;
    undefined    uVar35;
    undefined    uVar36;
    ushort       uVar37;
    undefined    uVar38;
    undefined    uVar39;
    int          iVar40;
    undefined2   in_stack_0000ffca;
    undefined2   in_stack_0000ffcc;
    undefined4   local_30;
    uint         uStack44;
    ushort       uStack42;
    ushort       uStack40;
    ushort       uStack38;
    undefined4  *puStack36;
    undefined2  *puStack32;
    undefined2  *puStack30;
    uint         uStack28;
    uint         uStack26;
    undefined2 **ppuStack24;
    ushort       local_16;
    undefined2  *local_14;
    int          local_12;
    undefined2  *local_10;
    undefined2  *puStack14;
    code        *pcStack12;
    ushort      *puStack10;
    undefined4  *local_6;

    uVar21 = SUB42(&USHORT_1050_1050, 0x0);
    uVar19 = param_6;
    uVar22 = param_7;
    BVar11 = read_file_1008_7dee(param_2, param_3, (ushort)&local_6, 0x0, param_9, 0x4, 0x1008);
    if(BVar11 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    puStack10 = (ushort *)0x0;
    if(((param_4 == 0x0) && ((char)(param_5 - 0x100U) == '\0'))
       && (puVar17 = (undefined2 *)(param_5 - 0x100U >> 0x7), puVar17 < (undefined2 *)((int)&PTR_LOOP_1050_000e + 0x1)))
    {
        uVar37 = (ushort)(param_1 >> 0x10);
        uVar20 = (astruct_349 *)param_1;
        uVar34 = (undefined)(param_9 >> 0x8);
        switch(puVar17)
        {
        case(undefined2 *)0x0:
            pass1_1030_145a(uVar20->field_0xe, (long)local_6);
            uStack28 = 0x0;
            uStack26 = 0x0;
            while((undefined4 *)CONCAT22(uStack26, uStack28) < local_6)
            {
                puVar27 = local_6;
                mem_op_1000_179c(0x14, (uchar *)puVar17, 0x1000);
                puStack32 = (undefined2 *)(uint)puVar27;
                puStack30 = puVar17;
                if(((uint)puVar17 | (uint)puStack32) == 0x0)
                {
                    puVar17  = (undefined2 *)0x0;
                    local_16 = 0x0;
                }
                else
                {
                    puVar25  = pass1_1030_5d0a((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
                    local_16 = (ushort)((ulong)puVar25 >> 0x10);
                    puVar17  = (undefined2 *)puVar25;
                }
                ppcVar7    = (code **)((int)*(undefined4 *)CONCAT22(local_16, puVar17) + 0x10);
                ppuStack24 = (undefined2 **)puVar17;
                (**ppcVar7)();
                if(puVar17 == (undefined2 *)0x0)
                {
                    return;
                }
                uVar5     = *(ulong *)(ppuStack24 + 0x2);
                uVar16    = (uint)ppuStack24[0x3];
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                puVar17   = (undefined2 *)(uVar16 & 0xff);
                pass1_1030_14b4(uVar20->field_0xe,
                                (ushort)ppuStack24,
                                local_16,
                                uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,
                                param_9);
                lVar6    = CONCAT22(uStack26, uStack28) + 0x1;
                uStack28 = (uint)lVar6;
                uStack26 = (uint)((ulong)lVar6 >> 0x10);
            }
            break;
        case(undefined2 *)0x1:
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        case(undefined2 *)0x2:
            pass1_1030_145a(uVar20->field_0x12, (long)local_6);
            uStack40 = 0x0;
            uStack38 = 0x0;
            while((undefined4 *)CONCAT22(uStack38, uStack40) < local_6)
            {
                puVar27 = local_6;
                mem_op_1000_179c(0x1c, (uchar *)puVar17, 0x1000);
                puStack32 = (undefined2 *)(uint)puVar27;
                uVar16    = (uint)puVar17 | (uint)puStack32;
                puStack30 = puVar17;
                if(uVar16 == 0x0)
                {
                    uVar12 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar12 = (uint)puStack32;
                    pass1_1030_2958((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
                }
                puStack36 = (undefined4 *)CONCAT22(uVar16, uVar12);
                ppcVar7   = (code **)((int)*puStack36 + 0x10);
                (**ppcVar7)();
                if(uVar12 == 0x0)
                {
                    return;
                }
                uVar19    = (ushort)((ulong)puStack36 >> 0x10);
                uVar18    = (astruct_348 *)puStack36;
                uVar5     = *(ulong *)&uVar18->field_0x4;
                uVar16    = uVar18->field_0x6;
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                puVar17   = (undefined2 *)(uVar16 & 0xff);
                pass1_1030_14b4(
                  uVar20->field_0x12, (ushort)uVar18, uVar19, uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10, param_9);
                lVar6    = CONCAT22(uStack38, uStack40) + 0x1;
                uStack40 = (ushort)lVar6;
                uStack38 = (ushort)((ulong)lVar6 >> 0x10);
            }
            break;
        case(undefined2 *)0x3:
            uStackY78 = SUB42(&USHORT_1050_1028, 0x0);
            uStackY80 = 0x970b;
            uVar19    = &uVar20->field_0x114;
            pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
            uStackY78 = 0x9728;
            local_16  = uVar19;
            local_14  = puVar17;
            pass1_1030_61fe(_PTR_LOOP_1050_5740,
                            CONCAT22(puVar17, uVar19),
                            param_1 & 0xffff0000 | (ulong)(uint)&uVar20->field_0x114,
                            *(long *)&uVar20->field_0x108,
                            uVar19,
                            (ushort)puVar17,
                            param_9);
            if((uVar20->field_0x11a == 0xa) || (uVar20->field_0x11a == 0x37))
            {
                if(uVar20->field_0x11a == 0x37)
                {
                    puVar17                                     = *(undefined2 **)((int)&uVar20->field_0x11e + 0x2);
                    uVar5                                       = uVar20->field_0x10c;
                    uStack42                                    = (ushort)uVar5;
                    uStack40                                    = (ushort)(uVar5 >> 0x10);
                    *(ulong *)((int)uVar20->field_0x11e + 0x20) = uVar5;
                }
                uVar19    = &uVar20->field_0x114;
                uStackY78 = 0x1030;
                uStackY80 = 0x9788;
                pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x400);
                *(ushort *)&uVar20->field_0x10c                   = uVar19;
                *(undefined2 **)((int)&uVar20->field_0x10c + 0x2) = puVar17;
                pass1_1018_0196((ulong)local_6,
                                CONCAT22(puVar17, *(undefined2 *)&uVar20->field_0x10c),
                                *(ulong *)&uVar20->field_0x108,
                                uVar19,
                                (uchar *)puVar17,
                                param_9);
                if(uVar20->field_0x11a == 0xa)
                {
                    pass1_1010_ed22((ulong)local_6, uVar20->field_0x10c, param_9);
                }
            }
            uVar5 = uVar20->field_0x10c;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar5, (uint)(uVar5 >> 0x10));
            *(ushort *)&uVar20->field_0x110                   = uVar19;
            *(undefined2 **)((int)&uVar20->field_0x110 + 0x2) = puVar17;
            uStack26                                          = (uint)puVar17 | *(uint *)&uVar20->field_0x110;
            if(uStack26 != 0x0)
            {
                ppcVar7 = (code **)((int)*uVar20->field_0x110 + 0x8);
                (**ppcVar7)();
                puVar17 = extraout_DX;
            }
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_16, (uint)local_14);
            ppuStack24 = (undefined2 **)puVar17;
            pass1_1030_73ee(CONCAT22(puVar17, uStack26), uVar20->field_0x10c, (ushort)puVar17);
            BVar11    = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar20->field_0x11a, 0x31);
            puStack32 = puVar17;
            if((BVar11 == 0x0) && (uVar20->field_0x122 == 0x0))
            {
                uVar21 = (undefined2)((ulong) * (undefined4 *)(uStack26 + 0xc) >> 0x10);
                if(*(int *)(uStack26 + 0x10) < 0x1)
                {
                    uVar10 = 0x5;
                }
                else
                {
                    uVar10 = 0x6;
                }
                *(undefined2 *)(uStack26 + 0x14) = uVar10;
                puStack32                        = ppuStack24;
            }
            uVar13    = *(undefined4 *)(uStack26 + 0x16);
            puStack30 = (undefined2 *)uVar13;
            uStack28  = (uint)((ulong)uVar13 >> 0x10);
            pass1_1028_e1ec(*(ulong *)&PTR_LOOP_1050_65e2, (ushort)puStack30, uStack28);
            puStack36 = (undefined4 *)CONCAT22((int)uVar13, puStack36._0_2_);
            if(CONCAT22(uStack28, puStack30) != 0x0)
            {
                struct_1030_e4fa(
                  (astruct_100 *)CONCAT22(param_9, &local_14c), CONCAT22(uStack28, puStack30), param_9, param_10);
                fn_ptr_1030_835a((ulong **)*(ulong **)&PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_9, &local_14c));
                local_14c  = 0x389a;
                uStackY330 = 0x1008;
            }
            ppcVar7 = (code **)((int)*uVar20->field_0x11e + 0x4);
            (**ppcVar7)();
            puVar27   = uVar20->field_0x11e;
            uStackY78 = 0x9902;
            pass1_1030_7e5a(CONCAT13((char)((uint)ppuStack24 >> 0x8), CONCAT12((char)ppuStack24, uStack26)),
                            *(ulong *)((int)puVar27 + 0x4),
                            extraout_DX_00);
            return;
        case(undefined2 *)0x4:
            pass1_1030_145a(uVar20->field_0x16, (long)local_6);
            uStack40 = 0x0;
            uStack38 = 0x0;
            while((undefined4 *)CONCAT22(uStack38, uStack40) < local_6)
            {
                puVar27 = local_6;
                mem_op_1000_179c(0x1e, (uchar *)puVar17, 0x1000);
                puStack32 = (undefined2 *)(uint)puVar27;
                puStack30 = puVar17;
                if(((uint)puVar17 | (uint)puStack32) == 0x0)
                {
                    iVar14 = 0x0;
                    uVar21 = 0x0;
                }
                else
                {
                    puVar25 = pass1_1030_560e((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
                    uVar21  = (undefined2)((ulong)puVar25 >> 0x10);
                    iVar14  = (int)puVar25;
                }
                puStack36 = (undefined4 *)CONCAT22(uVar21, iVar14);
                ppcVar7   = (code **)((int)*puStack36 + 0x10);
                (**ppcVar7)();
                if(iVar14 == 0x0)
                {
                    return;
                }
                uVar21    = (undefined2)((ulong)puStack36 >> 0x10);
                uVar5     = *(ulong *)((int)puStack36 + 0x4);
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                uVar4     = *(ulong *)((int)puStack36 + 0x10);
                uStack28  = (uint)uVar4;
                uStack26  = (uint)(uVar4 >> 0x10);
                pass1_1030_6222(_PTR_LOOP_1050_5740, 0x0, uVar4, uVar5, uStack28, extraout_DX_01, param_9);
                puVar17 = (undefined2 *)((uint)pcStack12 & 0xff);
                pass1_1030_14b4(uVar20->field_0x16,
                                (ushort)puStack36,
                                (ushort)((ulong)puStack36 >> 0x10),
                                CONCAT22(pcStack12, puStack14) & 0xffffff,
                                param_9);
                lVar6    = CONCAT22(uStack38, uStack40) + 0x1;
                uStack40 = (ushort)lVar6;
                uStack38 = (ushort)((ulong)lVar6 >> 0x10);
            }
            break;
        case(undefined2 *)0x5:
            *puVar17     = 0x5280;
            puVar17[0x1] = (int)&USHORT_1050_1028;
            return;
        case(undefined2 *)0x6:
            pass1_1030_145a(uVar20->field_0x1a, (long)local_6);
            for(local_30 = (undefined4 *)0x0; local_30 < local_6; local_30 = (undefined4 *)((long)local_30 + 0x1))
            {
                puVar27 = local_6;
                mem_op_1000_179c(0x21e, (uchar *)puVar17, 0x1000);
                puStack32 = (undefined2 *)(uint)puVar27;
                uVar16    = (uint)puVar17 | (uint)puStack32;
                puStack30 = puVar17;
                if(uVar16 == 0x0)
                {
                    uVar12 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar12 = (uint)puStack32;
                    pass1_1038_30aa((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10), param_9);
                }
                ppcVar7  = (code **)((int)*(undefined4 *)CONCAT22(uVar16, uVar12) + 0x10);
                uStack44 = uVar12;
                uStack42 = uVar16;
                (**ppcVar7)();
                if(uVar12 == 0x0)
                {
                    return;
                }
                uVar5     = *(ulong *)(uStack44 + 0x4);
                uVar16    = *(uint *)(uStack44 + 0x6);
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                puVar17   = (undefined2 *)(uVar16 & 0xff);
                pass1_1030_14b4(
                  uVar20->field_0x1a, uStack44, uStack42, uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10, param_9);
            }
            break;
        default:
            pass1_1030_145a(uVar20->field_0x1e, (long)local_6);
            pass1_1030_66de(_PTR_LOOP_1050_5740, (ulong)local_6, param_9);
            local_30 = (undefined4 *)0x0;
            while(true)
            {
                if(local_6 <= local_30)
                {
                    pass1_1030_154c();
                    pass1_1030_6740(_PTR_LOOP_1050_5740, param_9, param_7);
                    return;
                }
                local_14  = (undefined2 *)_PTR_LOOP_1050_5744;
                local_12  = (int)(_PTR_LOOP_1050_5744 >> 0x10);
                paVar26   = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5744);
                puStack30 = (undefined2 *)((ulong)paVar26 >> 0x10);
                puStack32 = (undefined2 *)(uint)paVar26;
                uVar16    = (uint)puStack30 | (uint)puStack32;
                if(uVar16 == 0x0)
                {
                    uVar12 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar12 = (uint)puStack32;
                    pass1_1030_67cc(&paVar26->field_0x0);
                }
                ppcVar7  = (code **)((int)*(undefined4 *)CONCAT22(uVar16, uVar12) + 0x10);
                uStack44 = uVar12;
                uStack42 = uVar16;
                (**ppcVar7)();
                if(uVar12 == 0x0)
                    break;
                uVar5     = *(ulong *)(uStack44 + 0x4);
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                lVar6     = *(long *)(uStack44 + 0x8);
                uStack40  = (ushort)lVar6;
                uStack38  = (ushort)((ulong)lVar6 >> 0x10);
                param_7   = (ushort)&local_30;
                puStack36 = (undefined4 *)((ulong)puStack36 & 0xffff0000 | ZEXT24(&stack0xffca));
                uStackY78 = 0xe977;
                pass1_1030_671c(_PTR_LOOP_1050_5740,
                                uVar5,
                                (ushort *)CONCAT22(param_9, &stack0xffca),
                                lVar6,
                                (ushort)&stack0xffca,
                                uStack42,
                                param_7,
                                param_9);
                pass1_1030_14b4(
                  uVar20->field_0x1e, uStack44, uStack42, CONCAT22(pcStack12, puStack14) & 0xffffff, param_9);
                local_30 = (undefined4 *)((long)local_30 + 0x1);
            }
            return;
        case(undefined2 *)0x9:
            local_6   = (undefined4 *)((ulong)local_6 & 0xffff);
            pcStack12 = (code *)uVar20->field_0x2e;
            puStack10 = (ushort *)(ulong)uVar20->field_0x30;
            (*pcStack12)();
            return;
        case(undefined2 *)0xa:
            pass1_1030_145a(uVar20->field_0x22, (long)local_6);
            uVar21 = 0x0;
            uVar10 = 0x0;
            while((undefined4 *)CONCAT22(uVar10, uVar21) < local_6)
            {
                puVar27 = local_6;
                mem_op_1000_179c(0xe, (uchar *)puVar17, 0x1000);
                puStack32 = (undefined2 *)(uint)puVar27;
                puStack30 = puVar17;
                if(((uint)puVar17 | (uint)puStack32) == 0x0)
                {
                    iVar14 = 0x0;
                    uVar15 = 0x0;
                }
                else
                {
                    puVar25 = pass1_1028_b204((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
                    uVar15  = (undefined2)((ulong)puVar25 >> 0x10);
                    iVar14  = (int)puVar25;
                }
                local_30 = (undefined4 *)CONCAT22(uVar15, iVar14);
                ppcVar7  = (code **)((int)*local_30 + 0x10);
                (**ppcVar7)();
                if(iVar14 == 0x0)
                {
                    return;
                }
                uVar22    = (ushort)((ulong)local_30 >> 0x10);
                uVar19    = (ushort)local_30;
                uVar5     = *(ulong *)(uVar19 + 0x4);
                uVar16    = *(uint *)(uVar19 + 0x6);
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                puVar17   = (undefined2 *)(uVar16 & 0xff);
                pass1_1030_14b4(
                  uVar20->field_0x22, uVar19, uVar22, uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10, param_9);
                lVar6  = CONCAT22(uVar10, uVar21) + 0x1;
                uVar21 = (undefined2)lVar6;
                uVar10 = (undefined2)((ulong)lVar6 >> 0x10);
            }
            break;
        case(undefined2 *)0xb:
            if(puVar17 < (undefined2 *)((int)&PTR_LOOP_1050_000e + 0x1))
            {
                pcVar1  = (char *)(param_6 + 0x23);
                cVar3   = *pcVar1;
                *pcVar1 = *pcVar1 << 0x6;
                piVar2  = (int *)((int)puVar17 + param_6);
                *piVar2 = *piVar2 + (-0x6600 - (uint)((char)(cVar3 << 0x5) < '\0'));
            }
            else
            {
                pass1_1028_780c(uVar19, uVar22, CONCAT22(in_stack_0000ffcc, in_stack_0000ffca));
                if(param_4 == 0x0)
                    goto code_r0x10287b17;
            }
            uVar29    = 0x0;
            uVar31    = 0x4;
            iVar14    = 0x1d;
            uStackY78 = 0x7b0a;
            puVar25   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_9, (uchar *)puVar17, param_7);
            puVar17   = (undefined2 *)((ulong)puVar25 >> 0x10);
            param_4   = (int)puVar25;
            pass1_1010_043a((ulong)puVar25, CONCAT13(uVar31, CONCAT12(uVar29, puVar17)), iVar14, param_9);
        code_r0x10287b17:
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x2, 0x400);
            pass1_1028_780c((ushort)uVar20, uVar37, CONCAT22(puVar17, param_4));
            puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_9, (uchar *)puVar17, param_7);
            pcStack12 = (code *)PTR_LOOP_1050_13ae;
            if(0x2 < (int)PTR_LOOP_1050_13ae)
            {
                puVar25
                  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_9, (uchar *)((ulong)puStack10 >> 0x10), param_7);
                uVar32 = (undefined)((ulong)puVar25 >> 0x10);
                uVar33 = (undefined)((ulong)puVar25 >> 0x18);
                uVar35 = 0x1;
                uVar36 = 0x0;
                uVar29 = (char)puVar25;
                uVar31 = (char)((ulong)puVar25 >> 0x8);
                while(CONCAT11(uVar36, uVar35) < 0x9)
                {
                    uVar28 = uVar29;
                    uVar30 = uVar31;
                    if((undefined4 *)*(long *)(CONCAT11(uVar31, uVar29) + 0x34 + CONCAT11(uVar36, uVar35) * 0x4)
                       == local_6)
                    {
                        puVar9   = (undefined2 *)((int)&PTR_LOOP_1050_0000 + 0x1);
                        local_30 = (undefined4 *)CONCAT22(local_30._2_2_, 0x1);
                        uVar35   = 0xd7;
                        uVar36   = 0x7b;
                        pass1_1008_612e(0x1, 0x64, 0x1);
                        puVar17 = (undefined2 *)(CONCAT11(uVar36, uVar35) - 0x7);
                        if(puVar17 == (undefined2 *)0x0)
                        {
                            bVar24 = SBORROW2((int)puVar9, 0x32);
                            puVar8 = puVar9 + -0x19;
                            bVar23 = puVar9 == (undefined2 *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x12);
                        LAB_1028_7b74:
                            if(!bVar23 && bVar24 == (int)puVar8 < 0x0)
                            {
                                local_30 = (undefined4 *)((ulong)local_30 & 0xffff0000);
                            }
                        }
                        else
                        {
                            puVar17 = (undefined2 *)(CONCAT11(uVar36, uVar35) - 0x8);
                            if(puVar17 == (undefined2 *)0x0)
                            {
                                bVar24 = SBORROW2((int)puVar9, 0x19);
                                puVar8 = (undefined2 *)((int)puVar9 + -0x19);
                                bVar23 = puVar8 == (undefined2 *)0x0;
                                goto LAB_1028_7b74;
                            }
                        }
                        puStack30 = puVar9;
                        if((int)local_30 != 0x0)
                        {
                            pass1_1028_90e6((astruct_100 *)CONCAT13(uVar34, CONCAT12((char)param_9, &local_154)),
                                            CONCAT11(uVar36, uVar35),
                                            param_9,
                                            param_10);
                            puVar17 = &local_154;
                            uVar32  = 0x8;
                            uVar33  = 0x10;
                            uVar29  = 0xc;
                            uVar31  = 0x7c;
                            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_9, puVar17));
                            local_154  = 0x389a;
                            uStackY338 = 0x1008;
                        }
                        uVar38 = 0x0;
                        uVar39 = 0x0;
                        uVar35 = 0x23;
                        uVar36 = 0x7c;
                        pass1_1008_612e(0x0, 0xa, (uint)puVar17);
                        ppuStack24 = (undefined2 **)puVar17;
                        if(CONCAT11(uVar36, uVar35) == 0x7)
                        {
                            iVar40  = 0x7;
                            puVar17 = puVar17 + 0x37;
                            iVar14  = (int)puVar17 >> 0xf;
                        }
                        else
                        {
                            uVar28 = uVar29;
                            uVar30 = uVar31;
                            if(CONCAT11(uVar36, uVar35) != 0x8)
                                goto LAB_1028_7ba0;
                            iVar40  = 0x8;
                            puVar17 = puVar17 + 0x32;
                            iVar14  = ((int)puVar17 >> 0xf) + (uint)((undefined2 *)0xff9b < puVar17);
                        }
                        uVar19 = iVar40 + iVar14 + (uint)CARRY2(CONCAT11(uVar39, uVar38), (uint)puVar17);
                        uVar28 = 0x8;
                        uVar30 = 0x10;
                        uVar35 = uVar32;
                        uVar36 = uVar33;
                        pass1_1010_ebf8(CONCAT13(uVar33, CONCAT12(uVar32, CONCAT11(uVar31, uVar29))),
                                        CONCAT11(uVar39, uVar38) + (int)puVar17,
                                        uVar19,
                                        uVar19);
                        uVar32 = uVar29;
                        uVar33 = uVar31;
                    }
                LAB_1028_7ba0:
                    iVar14 = CONCAT11(uVar36, uVar35) + 0x1;
                    uVar35 = (undefined)iVar14;
                    uVar29 = uVar28;
                    uVar31 = uVar30;
                    uVar36 = (undefined)((uint)iVar14 >> 0x8);
                }
            }
            return;
        case(undefined2 *)0xc:
            paVar18 = uVar20;
            pass1_1030_145a(uVar20->field_0x26, (long)local_6);
            uVar21 = 0x0;
            uVar10 = 0x0;
            while((undefined4 *)CONCAT22(uVar10, uVar21) < local_6)
            {
                BVar11 = read_file_1008_7dee(param_2, param_3, (ushort)&local_30, 0x0, param_9, 0x2, 0x1008);
                if(BVar11 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                    return;
                }
                uStack44 = switch_1008_73ea(param_2, param_3, (int)local_30);
                puVar27  = (undefined4 *)switch_1030_0000(
                  (ushort)uVar20, uVar37, uStack44, (uchar *)puVar17, (uint)paVar18, param_6, param_7);
                uStack38 = (ushort)((ulong)puVar27 >> 0x10);
                uVar19   = (ushort)puVar27;
                ppcVar7  = (code **)((int)*puVar27 + 0x10);
                uStack40 = uVar19;
                (**ppcVar7)();
                if(uVar19 == 0x0)
                {
                    return;
                }
                uVar5     = *(ulong *)(uStack40 + 0x4);
                uVar16    = *(uint *)(uStack40 + 0x6);
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                puVar17   = (undefined2 *)(uVar16 & 0xff);
                paVar18   = uVar20;
                pass1_1030_14b4(
                  uVar20->field_0x26, uStack40, uStack38, uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10, param_9);
                lVar6  = CONCAT22(uVar10, uVar21) + 0x1;
                uVar21 = (undefined2)lVar6;
                uVar10 = (undefined2)((ulong)lVar6 >> 0x10);
            }
            break;
        case(undefined2 *)0xd:
            puStack10  = (ushort *)(ZEXT24(puVar17) << 0x10);
            uVar13     = *(undefined4 *)&PTR_LOOP_1050_000c;
            local_10   = (undefined2 *)uVar13;
            puStack14  = (undefined2 *)((ulong)uVar13 >> 0x10);
            pcStack12  = *(code **)&PTR_LOOP_1050_0010;
            ppuStack24 = &local_10;
            uStackY78  = 0x211d;
            pass1_1008_3eb4((ushort *)CONCAT13(uVar34, CONCAT12((char)param_9, &local_10)),
                            (ushort *)CONCAT22(param_9, &local_16),
                            (ushort *)CONCAT22(param_9, &local_14),
                            (ushort *)CONCAT22(param_9, &local_12));
            ppuStack24 = (undefined2 **)((int)local_14 + -0x1);
            puStack14  = ppuStack24;
            uVar16     = pass1_1028_21ba((ushort)uVar20,
                                     uVar37,
                                     (ushort *)CONCAT22(param_9, &local_10),
                                     (long)local_6,
                                     (uint)&local_10,
                                     (uint)puVar17,
                                     param_9);
            if(uVar16 == 0x0)
            {
                ppuStack24 = (undefined2 **)((int)local_14 + 0x1);
                puStack14  = ppuStack24;
                uVar16     = pass1_1028_21ba((ushort)uVar20,
                                         uVar37,
                                         (ushort *)CONCAT22(param_9, &local_10),
                                         (long)local_6,
                                         (uint)&local_10,
                                         (uint)puVar17,
                                         param_9);
                if(uVar16 == 0x0)
                {
                    puStack14  = local_14;
                    ppuStack24 = (undefined2 **)(local_12 + -0x1);
                    local_10   = ppuStack24;
                    uVar16     = pass1_1028_21ba((ushort)uVar20,
                                             uVar37,
                                             (ushort *)CONCAT22(param_9, &local_10),
                                             (long)local_6,
                                             (uint)&local_10,
                                             (uint)puVar17,
                                             param_9);
                    if(uVar16 == 0x0)
                    {
                        ppuStack24 = (undefined2 **)(local_12 + 0x1);
                        local_10   = ppuStack24;
                        uVar16     = pass1_1028_21ba((ushort)uVar20,
                                                 uVar37,
                                                 (ushort *)CONCAT22(param_9, &local_10),
                                                 (long)local_6,
                                                 (uint)&local_10,
                                                 (uint)puVar17,
                                                 param_9);
                        if(uVar16 == 0x0)
                        {
                            return;
                        }
                    }
                }
            }
            pass1_1038_79b2(_PTR_LOOP_1050_5a64, (ulong)puStack10, uVar16, (uchar *)puVar17);
            return;
        case(undefined2 *)0xe:
            pass1_1030_145a(uVar20->field_0x2a, (long)local_6);
            uVar21 = 0x0;
            uVar10 = 0x0;
            while((undefined4 *)CONCAT22(uVar10, uVar21) < local_6)
            {
                puVar27 = local_6;
                mem_op_1000_179c(0x3b2, (uchar *)puVar17, 0x1000);
                puStack32 = (undefined2 *)(uint)puVar27;
                uVar16    = (uint)puVar17 | (uint)puStack32;
                puStack30 = puVar17;
                if(uVar16 == 0x0)
                {
                    uVar12 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar12 = (uint)puStack32;
                    pass1_1030_2068((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
                }
                local_30 = (undefined4 *)CONCAT22(uVar16, uVar12);
                ppcVar7  = (code **)((int)*local_30 + 0x10);
                (**ppcVar7)();
                if(uVar12 == 0x0)
                {
                    return;
                }
                uVar22    = (ushort)((ulong)local_30 >> 0x10);
                uVar19    = (ushort)local_30;
                uVar5     = *(ulong *)(uVar19 + 0x4);
                uVar16    = *(uint *)(uVar19 + 0x6);
                puStack14 = (undefined2 *)uVar5;
                pcStack12 = (code *)(uVar5 >> 0x10);
                puVar17   = (undefined2 *)(uVar16 & 0xff);
                pass1_1030_14b4(
                  uVar20->field_0x2a, uVar19, uVar22, uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10, param_9);
                lVar6  = CONCAT22(uVar10, uVar21) + 0x1;
                uVar21 = (undefined2)lVar6;
                uVar10 = (undefined2)((ulong)lVar6 >> 0x10);
            }
        }
        pass1_1030_154c();
    }
    return;
}


ulong __stdcall16far pass1_1028_ebee(ulong param_1, ushort param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;
    ulong      uVar3;

    mem_op_1000_179c(0x14, (uchar *)param_3, 0x1000);
    if((uchar *)(param_3 | param_2) != (uchar *)0x0)
    {
        pass1_1030_1a32((ushort *)CONCAT22(param_3, param_2), param_2, (uchar *)(param_3 | param_2));
    }
    uVar3                         = struct_1030_4574(*(ulong *)((int)param_1 + 0x52));
    uVar2                         = (undefined2)((ulong)_PTR_LOOP_1050_5166 >> 0x10);
    iVar1                         = (int)_PTR_LOOP_1050_5166;
    *(undefined2 *)(iVar1 + 0x10) = (int)uVar3;
    *(undefined2 *)(iVar1 + 0x12) = (int)(uVar3 >> 0x10);
    uVar2                         = (undefined2)((ulong)_PTR_LOOP_1050_5166 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_5166 + 0x6), *(undefined2 *)((int)_PTR_LOOP_1050_5166 + 0x4));
}


void __stdcall16far pass1_1028_ec36(ulong  param_1,
                                    ushort param_2,
                                    int    param_3,
                                    ushort param_4,
                                    ulong  param_5,
                                    ushort param_6,
                                    uchar *param_7,
                                    ushort param_8)

{
    undefined4 uVar1;
    ushort     uVar2;
    ushort     uVar3;
    uchar     *puVar4;
    uchar     *puVar5;
    undefined2 uVar6;
    ushort    *puVar7;

    mem_op_1000_179c(0x14, param_7, 0x1000);
    if((uchar *)((uint)param_7 | param_6) == (uchar *)0x0)
    {
        uVar2  = 0x0;
        puVar4 = (uchar *)0x0;
    }
    else
    {
        puVar7
          = pass1_1030_5d3c((ushort *)CONCAT22(param_7, param_6), param_5, param_6, (uchar *)((uint)param_7 | param_6));
        puVar4 = (uchar *)((ulong)puVar7 >> 0x10);
        uVar2  = (ushort)puVar7;
    }
    uVar6  = (undefined2)(param_1 >> 0x10);
    uVar1  = *(undefined4 *)((int)param_1 + 0x52);
    puVar5 = puVar4;
    uVar3  = uVar2;
    pass1_1030_4594(puVar4, (ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_3);
    pass1_1030_5fe2(CONCAT22(puVar4, uVar2), CONCAT22(puVar5, uVar3));
    pass1_1030_1358(*(ulong *)((int)param_1 + 0xe),
                    uVar2,
                    (ushort)puVar4,
                    *(ulong *)(uVar2 + 0x4) & 0xffff | ((ulong) * (uint *)(uVar2 + 0x6) & 0xff) << 0x10,
                    param_8);
    return;
}


void __stdcall16far pass1_1028_ecac(ulong  param_1,
                                    ushort param_2,
                                    int   *param_3,
                                    ushort param_4,
                                    ulong  param_5,
                                    ushort param_6,
                                    uchar *param_7,
                                    ushort param_8)

{
    undefined4 uVar1;
    int      **ppiVar2;
    uchar     *puVar3;
    uchar     *puVar4;
    undefined2 uVar5;

    mem_op_1000_179c(0x1c, param_7, 0x1000);
    puVar3 = (uchar *)((uint)param_7 | param_6);
    if(puVar3 == (uchar *)0x0)
    {
        param_6 = 0x0;
        puVar3  = (uchar *)0x0;
    }
    else
    {
        struct_1030_299a((ushort *)CONCAT22(param_7, param_6), param_5, param_6, puVar3);
    }
    uVar5   = (undefined2)(param_1 >> 0x10);
    uVar1   = *(undefined4 *)((int)param_1 + 0x52);
    puVar4  = puVar3;
    ppiVar2 = (int **)param_3;
    pass1_1030_4628(puVar3, (ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), (int)param_3);
    *ppiVar2 = param_3;
    pass1_1030_3006(CONCAT22(puVar3, param_6), CONCAT22(puVar4, ppiVar2));
    pass1_1030_1358(*(ulong *)((int)param_1 + 0x12),
                    param_6,
                    (ushort)puVar3,
                    *(ulong *)(param_6 + 0x4) & 0xffff | ((ulong) * (uint *)(param_6 + 0x6) & 0xff) << 0x10,
                    param_8);
    return;
}


// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_ed2c(ulong  param_1,
                                    ushort param_2,
                                    int    param_3,
                                    ushort param_4,
                                    ulong  param_5,
                                    ushort param_6,
                                    uchar *param_7,
                                    ushort param_8,
                                    uchar  param_9)

{
    ulong      uVar2;
    ushort     uVar3;
    ushort     uVar4;
    uchar     *puVar5;
    uchar     *puVar6;
    uchar     *puVar7;
    undefined2 uVar8;
    ushort    *puVar9;
    undefined4 uVar1;

    mem_op_1000_179c(0x1e, param_7, 0x1000);
    if((uchar *)((uint)param_7 | param_6) == (uchar *)0x0)
    {
        uVar3  = 0x0;
        puVar5 = (uchar *)0x0;
    }
    else
    {
        puVar9 = struct_1030_565a(
          (ushort *)CONCAT22(param_7, param_6), param_5, param_6, (uchar *)((uint)param_7 | param_6));
        puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
        uVar3  = (ushort)puVar9;
    }
    uVar8  = (undefined2)(param_1 >> 0x10);
    uVar1  = *(undefined4 *)((int)param_1 + 0x52);
    puVar6 = puVar5;
    uVar4  = uVar3;
    pass1_1030_4782(param_8, param_9, puVar5, (ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), 0x1, 0x1, param_3);
    puVar7 = puVar6;
    pass1_1030_5a80(CONCAT22(puVar5, uVar3), CONCAT22(puVar6, uVar4), param_8);
    uVar2 = *(ulong *)(uVar3 + 0x4);
    pass1_1030_6222(_PTR_LOOP_1050_5740, 0x1, CONCAT22(puVar6, uVar4), uVar2, (uint)uVar2, puVar7, param_8);
    pass1_1030_1358(*(ulong *)((int)param_1 + 0x16), uVar3, (ushort)puVar5, uVar2 & 0xffffff, param_8);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_edc4(ulong param_1, ushort param_2, ushort *param_3, long param_4, uchar *param_5, ushort param_6)

{
    ushort     uVar1;
    ulong      uVar2;
    uchar     *puVar3;
    uchar      in_AF;
    undefined  local_1a[0x4];
    ulong      uStack22;
    undefined4 uStack18;
    ulong      uStack14;
    ulong      uStack10;
    ushort    *puStack6;

    puStack6 = param_3;
    pass1_1030_64ce(
      param_6, (int)param_3, param_5, _PTR_LOOP_1050_5740, param_3, param_4, (ulong *)CONCAT22(param_6, local_1a));
    uVar2    = *(ulong *)param_3;
    uStack14 = uVar2;
    uStack10 = uVar2;
    mem_op_1000_179c(0x21e, param_5, 0x1000);
    uVar1  = (ushort)uVar2;
    puVar3 = (uchar *)((uint)param_5 | uVar1);
    if(puVar3 == (uchar *)0x0)
    {
        uVar1  = 0x0;
        puVar3 = (uchar *)0x0;
    }
    else
    {
        pass1_1038_3222((ushort *)(uVar2 & 0xffff | ZEXT24(param_5) << 0x10),
                        uStack14,
                        param_4,
                        uVar1,
                        puVar3,
                        in_AF,
                        (uchar *)param_6);
    }
    uStack18 = CONCAT22(puVar3, uVar1);
    uStack22 = *(ulong *)(uVar1 + 0x4);
    pass1_1030_1358(*(ulong *)((int)param_1 + 0x1a),
                    uVar1,
                    (ushort)puVar3,
                    uStack22 & 0xffff | ((ulong) * (uint *)(uVar1 + 0x6) & 0xff) << 0x10,
                    param_6);
    return;
}


void __stdcall16far struct_1028_d22e(ulong *param_1, ulong param_2, ushort param_3)

{
    uint       uVar1;
    uchar     *puVar2;
    undefined2 uVar3;

    uVar3                          = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                       = 0x0;
    *(ulong *)((int)param_1 + 0x4) = param_2;
    mem_op_1000_179c(0xc, (uchar *)param_3, 0x1000);
    uVar1  = (uint)param_2;
    puVar2 = (uchar *)(param_3 | uVar1);
    if(puVar2 == (uchar *)0x0)
    {
        *param_1 = 0x0;
    }
    else
    {
        struct_1028_d59c((ulong *)(param_2 & 0xffff | (ulong)param_3 << 0x10), puVar2);
        *(uint *)param_1                = uVar1;
        *(uchar **)((int)param_1 + 0x2) = puVar2;
    }
    return;
}


void __stdcall16far struct_1028_d59c(ulong *param_1, uchar *param_2)

{
    undefined2  *puVar1;
    uint         uVar2;
    undefined2  *puVar3;
    uchar       *puVar4;
    uchar       *extraout_DX;
    astruct_158 *iVar5;
    undefined2   uVar5;
    undefined2  *puStack14;

    uVar5            = (undefined2)((ulong)param_1 >> 0x10);
    iVar5            = (astruct_158 *)param_1;
    *param_1         = 0x0;
    iVar5->field_0x4 = (undefined2 *)0x0;
    iVar5->field_0x8 = (undefined2 *)0x0;
    puVar3           = (undefined2 *)*_PTR_LOOP_1050_5748;
    *param_1         = (ulong)puVar3;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    puVar1 = (undefined2 *)((ulong)puVar3 & 0xffff | ZEXT24(param_2) << 0x10);
    puVar4 = (uchar *)((uint)param_2 | (uint)puVar3);
    if(puVar4 == (uchar *)0x0)
    {
        iVar5->field_0x4 = (undefined2 *)0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)((ulong)puVar3 & 0xffff | ZEXT24(param_2) << 0x10));
        *puVar1                             = 0xd804;
        *(undefined2 *)((uint)puVar3 + 0x2) = (int)&USHORT_1050_1028;
        iVar5->field_0x4                    = puVar1;
        puVar3                              = puVar1;
        puVar4                              = extraout_DX;
    }
    uVar2 = (uint)puVar3;
    mem_op_1000_179c(0xc, puVar4, 0x1000);
    puStack14 = (undefined2 *)CONCAT22(puVar4, uVar2);
    if(((uint)puVar4 | uVar2) == 0x0)
    {
        iVar5->field_0x8 = (undefined2 *)0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar4, uVar2));
        *puStack14                   = 0xd804;
        *(undefined2 *)(uVar2 + 0x2) = (int)&USHORT_1050_1028;
        iVar5->field_0x8             = puStack14;
    }
    return;
}


void __stdcall16far pass1_1028_d81c(ulong *param_1, ulong param_2, uchar *param_3, ushort param_4)

{
    uint        *puVar1;
    uchar       *puVar2;
    uchar       *puVar3;
    uint         uVar4;
    astruct_136 *iVar6;
    undefined2   uVar5;

    uVar5                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar6                             = (astruct_136 *)param_1;
    *param_1                          = 0x0;
    iVar6->field_0x4                  = param_2;
    *(undefined4 *)&iVar6->field_0x52 = 0x0;
    _PTR_LOOP_1050_65e2               = param_1;
    iVar6->field_0x32                 = 0xec36;
    iVar6->field_0x34                 = (int)&USHORT_1050_1028;
    iVar6->field_0x36                 = 0xecac;
    iVar6->field_0x38                 = (int)&USHORT_1050_1028;
    iVar6->field_0x3a                 = 0xed2c;
    iVar6->field_0x3c                 = (int)&USHORT_1050_1028;
    iVar6->field_0x3e                 = 0xedc4;
    iVar6->field_0x40                 = (int)&USHORT_1050_1028;
    iVar6->field_0x42                 = 0xee54;
    iVar6->field_0x44                 = (int)&USHORT_1050_1028;
    iVar6->field_0x46                 = 0xef00;
    iVar6->field_0x48                 = (int)&USHORT_1050_1028;
    iVar6->field_0x4a                 = 0x10b0;
    iVar6->field_0x4c                 = 0x1030;
    iVar6->field_0x4e                 = 0x1120;
    iVar6->field_0x50                 = 0x1030;
    mem_op_1000_179c(0x8, param_3, 0x1000);
    uVar4  = (uint)param_2;
    puVar2 = (uchar *)((uint)param_3 | uVar4);
    if(puVar2 != (uchar *)0x0)
    {
        pass1_1030_615a((astruct_137 *)(param_2 & 0xffff | ZEXT24(param_3) << 0x10), (ushort)puVar2);
    }
    mem_op_1000_179c(0x56c, puVar2, 0x1000);
    puVar3 = (uchar *)((uint)puVar2 | uVar4);
    if(puVar3 == (uchar *)0x0)
    {
        uVar4  = 0x0;
        puVar3 = (uchar *)0x0;
    }
    else
    {
        struct_1030_44be((ulong *)CONCAT22(puVar2, uVar4), (ushort)puVar3);
    }
    iVar6->field_0x52 = uVar4;
    iVar6->field_0x54 = puVar3;
    mem_op_1000_179c(0x4, puVar3, 0x1000);
    puVar2 = (uchar *)((uint)puVar3 | uVar4);
    if(puVar2 != (uchar *)0x0)
    {
        struct_1008_bde0((ulong *)CONCAT22(puVar3, uVar4), puVar2);
    }
    puVar1 = pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar6->field_0xa), (WNDCLASS16 *)0x0, 0x24);
    mem_op_1000_179c(0x1c, puVar2, 0x1000);
    puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
    if(puVar3 == (uchar *)0x0)
    {
        *(undefined4 *)&iVar6->field_0xe = 0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar2, puVar1), 0x5, 0x15, param_4);
        iVar6->field_0xe  = puVar1;
        iVar6->field_0x10 = puVar3;
    }
    mem_op_1000_179c(0x1c, puVar3, 0x1000);
    puVar2 = (uchar *)((uint)puVar3 | (uint)puVar1);
    if(puVar2 == (uchar *)0x0)
    {
        puVar1 = (uint *)0x0;
        puVar2 = (uchar *)0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar3, puVar1), 0x5, 0xa, param_4);
    }
    iVar6->field_0x12 = puVar1;
    iVar6->field_0x14 = puVar2;
    mem_op_1000_179c(0x1c, puVar2, 0x1000);
    puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
    if(puVar3 == (uchar *)0x0)
    {
        puVar1 = (uint *)0x0;
        puVar3 = (uchar *)0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar2, puVar1), 0x5, 0x19, param_4);
    }
    iVar6->field_0x16 = puVar1;
    iVar6->field_0x18 = puVar3;
    mem_op_1000_179c(0x1c, puVar3, 0x1000);
    puVar2 = (uchar *)((uint)puVar3 | (uint)puVar1);
    if(puVar2 == (uchar *)0x0)
    {
        puVar1 = (uint *)0x0;
        puVar2 = (uchar *)0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar3, puVar1), 0x5, 0xa, param_4);
    }
    iVar6->field_0x1a = puVar1;
    iVar6->field_0x1c = puVar2;
    mem_op_1000_179c(0x1c, puVar2, 0x1000);
    puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
    if(puVar3 == (uchar *)0x0)
    {
        puVar1 = (uint *)0x0;
        puVar3 = (uchar *)0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar2, puVar1), 0x64, 0x1f4, param_4);
    }
    iVar6->field_0x1e = puVar1;
    iVar6->field_0x20 = puVar3;
    mem_op_1000_179c(0x1c, puVar3, 0x1000);
    puVar2 = (uchar *)((uint)puVar3 | (uint)puVar1);
    if(puVar2 == (uchar *)0x0)
    {
        puVar1 = (uint *)0x0;
        puVar2 = (uchar *)0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar3, puVar1), 0x19, 0x64, param_4);
    }
    iVar6->field_0x22 = puVar1;
    iVar6->field_0x24 = puVar2;
    mem_op_1000_179c(0x1c, puVar2, 0x1000);
    puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
    if(puVar3 == (uchar *)0x0)
    {
        puVar1 = (uint *)0x0;
        puVar3 = (uchar *)0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar2, puVar1), 0x64, 0x1f4, param_4);
    }
    iVar6->field_0x26 = puVar1;
    iVar6->field_0x28 = puVar3;
    mem_op_1000_179c(0x1c, puVar3, 0x1000);
    uVar4 = (uint)puVar3 | (uint)puVar1;
    if(uVar4 == 0x0)
    {
        puVar1 = (uint *)0x0;
        uVar4  = 0x0;
    }
    else
    {
        struct_1030_11aa((ushort *)CONCAT22(puVar3, puVar1), 0x5, 0x2, param_4);
    }
    iVar6->field_0x2a = puVar1;
    iVar6->field_0x2c = uVar4;
    return;
}


ulong __stdcall16far pass1_1028_e0bc(ulong param_1, int param_2, ulong *param_3, uchar *param_4, ushort param_5)

{
    ulong *puVar1;
    ulong *puVar2;
    ulong *puVar3;
    int    iVar4;
    uchar *puVar5;
    ulong *puVar6;

    mem_op_1000_179c(0x98, param_4, 0x1000);
    puVar3 = param_3;
    puVar5 = param_4;
    pass1_1030_4bbe(param_5, (ushort)param_4, *(ulong *)((int)param_1 + 0x52), param_2);
    puVar6 = param_3;
    for(iVar4 = 0x26; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
    {
        puVar2  = puVar6;
        puVar6  = puVar6 + 0x1;
        puVar1  = puVar3;
        puVar3  = puVar3 + 0x1;
        *puVar2 = *puVar1;
    }
    return CONCAT22(param_4, param_3);
}


void __stdcall16far pass1_1028_e100(ulong param_1, ushort param_2, uchar *param_3)

{
    undefined4  *puVar1;
    undefined4  *puVar2;
    astruct_311 *uVar4;
    int          iVar4;
    uint         uVar5;
    undefined4  *puVar6;
    undefined4  *puVar7;
    undefined2   uVar8;
    ushort       unaff_SS;
    ulong        uStack10;
    ulong        uStack6;
    ulong        uVar3;

    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_3, 0x1000);
        PTR_LOOP_1050_5f2e = param_3;
    }
    else
    {
    }
    uVar4
      = (astruct_311 *)fn_ptr_op_1000_1708(0xae, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    uVar3    = ZEXT24(uVar4);
    uStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    uVar5    = (uint)PTR_LOOP_1050_5f2e | (uint)uVar4;
    if(uVar5 == 0x0)
    {
        uStack6 = 0x0;
    }
    else
    {
        uVar4->field_0xa4 = 0x0;
        uVar4->field_0xa8 = 0x0;
        uVar4->field_0xac = 0x0;
        uStack6           = uStack10;
        uVar3             = uStack10;
    }
    puVar6 = (undefined4 *)uVar3;
    pass1_1030_4c06(*(ulong *)((int)param_1 + 0x52), param_2, uVar5, unaff_SS);
    uVar8  = (undefined2)(uStack6 >> 0x10);
    puVar7 = (undefined4 *)uStack6;
    for(iVar4 = 0x2b; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
    {
        puVar2  = puVar7;
        puVar7  = puVar7 + 0x1;
        puVar1  = puVar6;
        puVar6  = puVar6 + 0x1;
        *puVar2 = *puVar1;
    }
    *(undefined2 *)puVar7 = *(undefined2 *)puVar6;
    return;
}


void __stdcall16far pass1_1028_e28a(uchar *param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    undefined2  uVar3;
    ushort     *puVar4;

    puVar4  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, param_1, param_2);
    uVar3   = (undefined2)((ulong)puVar4 >> 0x10);
    puVar2  = (undefined4 *)((int)puVar4 + 0xa);
    ppcVar1 = (code **)((int)*puVar2 + 0x4);
    (**ppcVar1)(0x1010, puVar2, uVar3, 0x5);
    return;
}


void __stdcall16far
pass1_1028_c3aa(ushort param_1, ushort param_2, ushort *param_3, ulong param_4, ulong param_5, ushort param_6)

{
    code      **ppcVar1;
    int         iVar2;
    ushort      uVar3;
    uint        uVar4;
    undefined2  uVar5;
    undefined  *puVar6;
    uchar      *puVar7;
    uchar      *puVar8;
    uchar      *puVar9;
    uint        extraout_DX;
    uint        uVar10;
    int         unaff_DI;
    undefined2  uVar11;
    ulong       uVar12;
    ushort     *puVar13;
    ulong      *puVar14;
    undefined   uVar15;
    undefined   uVar16;
    undefined2  uVar17;
    uint        uVar18;
    ulong       uVar19;
    uint        uVar20;
    ulong       uStack40;
    ulong       uStack36;
    undefined4 *puStack32;
    undefined  *puStack24;
    undefined   local_4[0x2];

    uVar12 = pass1_1030_bcae((ushort)local_4, param_6);
    puVar7 = (uchar *)(uVar12 >> 0x10);
    iVar2  = (int)uVar12;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar12 = *(ulong *)(iVar2 + 0x10);
    uVar15 = SUB41(param_3, 0x0);
    uVar16 = (undefined)((ulong)param_3 >> 0x8);
    uVar11 = (undefined2)((ulong)param_3 >> 0x10);
    puVar8 = puVar7;
    uVar19 = param_5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar12, (uint)(uVar12 >> 0x10));
    puStack24 = local_4;
    pass1_1030_bcde(param_6,
                    (ushort)puStack24,
                    param_6,
                    uVar12 & 0xffff | ZEXT24(puVar8) << 0x10,
                    (ushort *)CONCAT22(uVar11, CONCAT11(uVar16, uVar15)),
                    uVar19);
    if((int)puStack24 < 0x0)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
        return;
    }
    if(0x1e < (int)puStack24)
    {
        uVar3   = 0x87;
        puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_6, puVar8, unaff_DI);
        uVar3   = pass1_1010_65d0(param_6, (ulong)puVar13, uVar3);
        if(uVar3 == 0x0)
        {
            puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x15);
            puVar9  = (uchar *)((ulong)puVar14 >> 0x10);
            uVar4   = (uint)puVar14;
            uVar11  = SUB42(&PTR_LOOP_1050_1038, 0x0);
            pass1_1038_4d6e(CONCAT22(puVar7, iVar2), puVar14, uVar4, puVar9);
            puStack32 = (undefined4 *)CONCAT22(puVar9, uVar4);
            ppcVar1   = (code **)((int)*puStack32 + 0x10);
            uVar10    = uVar4;
            uVar18    = uVar4;
            puVar8    = puVar9;
            (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar4, puVar9);
            uStack36 = CONCAT22(extraout_DX, uVar10);
            uStack40 = 0x0;
            uVar10   = extraout_DX;
            while(true)
            {
                if(uStack36 <= uStack40)
                {
                    if(puStack32 != (undefined4 *)0x0)
                    {
                        ppcVar1 = (code **)*puStack32;
                        (**ppcVar1)(uVar11, uVar4, (char)puVar9, 0x1, uVar18, puVar8, puStack32, puStack32);
                    }
                    PTR_LOOP_1050_50ca = (undefined *)0x6b6;
                    PTR_LOOP_1050_50cc = puStack24 + -0x1e;
                    return;
                }
                uVar15  = (undefined)param_5;
                uVar16  = (undefined)(param_5 >> 0x8);
                uVar12  = uStack36;
                puVar13 = param_3;
                uVar17  = (int)(param_5 >> 0x10);
                pass1_1030_1d58((ulong)puStack32);
                uVar5  = (undefined2)uVar12;
                puVar6 = local_4;
                uVar11 = 0x1030;
                uVar20 = uVar10;
                pass1_1030_bcde(param_6,
                                (ushort)puVar6,
                                param_6,
                                uVar12 & 0xffff | (ulong)uVar10 << 0x10,
                                puVar13,
                                CONCAT22(uVar17, CONCAT11(uVar16, uVar15)));
                if((0x0 < (int)puVar6) && ((int)puVar6 < 0x1f))
                    break;
                if((int)puVar6 < (int)puStack24)
                {
                    puStack24 = puVar6;
                }
                uStack40 = uStack40 + 0x1;
            }
            if(puStack32 == (undefined4 *)0x0)
            {
                return;
            }
            ppcVar1 = (code **)*puStack32;
            (**ppcVar1)(0x1030, uVar4, (char)puVar9, 0x1, uVar18, puVar8, puStack32, puStack32, uVar5, uVar20);
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1028_ccd0(uchar param_1, ushort param_2, ulong param_3, ushort *param_4)

{
    code      **ppcVar1;
    ushort     *puVar2;
    undefined  *puVar3;
    int         iVar4;
    uchar      *extraout_DX;
    uchar      *puVar5;
    uint        uVar6;
    int         iVar7;
    undefined2  extraout_DX_00;
    int         unaff_DI;
    ushort      uVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    undefined2  uVar12;
    undefined2  local_178;
    undefined2  uStack374;
    int         iStack84;
    ushort      uStack72;
    ushort      uStack64;
    int         iStack62;
    undefined4  uStack60;
    undefined4 *puStack56;
    ulong       uStack52;
    ushort     *puStack48;
    undefined   local_2c[0xc];
    int         local_20;
    int         local_1e;
    undefined4  uStack28;
    undefined4  uStack24;
    undefined4  uStack20;
    int         iStack16;
    int         iStack14;
    ushort      uStack12;
    ushort      uStack10;
    ushort      local_8;
    int         local_6;
    int         local_4;

    puVar2 = &local_8;
    pass1_1008_3eb4(param_4,
                    (ushort *)CONCAT22(param_2, puVar2),
                    (ushort *)CONCAT22(param_2, &local_6),
                    (ushort *)CONCAT22(param_2, &local_4));
    pass1_1028_b58e(param_3);
    uStack20 = CONCAT22(extraout_DX, puVar2);
    uStack24 = *(undefined4 *)(puVar2 + 0x17);
    uStack28 = *(undefined4 *)((int)uStack24 + 0x4);
    puVar5   = extraout_DX;
    pass1_1028_c1f8(param_2,
                    (int)&local_20,
                    (ushort)extraout_DX,
                    param_3,
                    (ushort *)CONCAT22(param_2, &local_20),
                    (ushort *)CONCAT22(param_2, &local_1e));
    uStack10 = local_4 - 0x1;
    iStack14 = local_4 + 0x1;
    uStack12 = local_6 - 0x1;
    iStack16 = local_6 + 0x1;
    if((int)uStack10 < 0x0)
    {
        uStack10 = 0x0;
    }
    if(local_1e <= iStack14)
    {
        iStack14 = local_1e + -0x1;
    }
    if((int)uStack12 < 0x0)
    {
        uStack12 = 0x0;
    }
    if(local_20 <= iStack16)
    {
        iStack16 = local_20 + -0x1;
    }
    pass1_1008_6c90((ushort *)CONCAT22(param_2, local_2c));
    pass1_1008_6cec((ushort *)CONCAT22(param_2, local_2c),
                    local_8,
                    CONCAT22(iStack14, iStack16),
                    local_8,
                    CONCAT22(uStack10, uStack12));
    puStack48 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_2, puVar5, unaff_DI);
    uVar6     = (uint)((ulong)puStack48 >> 0x10);
    uStack52  = *(ulong *)((int)puStack48 + 0x20);
    puVar3    = local_2c;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(param_2, puVar3), uStack52, param_2);
    puStack56 = (undefined4 *)CONCAT22(uVar6, puVar3);
    if((uVar6 | (uint)puVar3) != 0x0)
    {
        uStack60 = 0x0;
        iStack62 = 0x0;
        for(uStack64 = uStack12; (int)uStack64 <= iStack16; uStack64 = uStack64 + 0x1)
        {
            for(uStack72 = uStack10; iVar4 = iStack62, (int)uStack72 <= iStack14; uStack72 = uStack72 + 0x1)
            {
                iVar7    = iStack62 >> 0xf;
                ppcVar1  = (code **)((int)*puStack56 + 0x4);
                iStack62 = iStack62 + 0x1;
                (**ppcVar1)(0x1030, (int)puStack56, (int)((ulong)puStack56 >> 0x10), iVar4, iVar7);
                uStack60       = CONCAT22(extraout_DX_00, iVar4);
                uStack60._3_1_ = (char)((uint)extraout_DX_00 >> 0x8);
                if(uStack60._3_1_ == '\0')
                {
                    iStack84 = iVar4;
                    if(iVar4 == 0x7)
                    {
                        pass1_1008_3e76(param_4, local_8, uStack64, uStack72);
                        uVar11 = (undefined2)uStack52;
                        uVar12 = (undefined2)(uStack52 >> 0x10);
                        uVar9  = (undefined2)uStack28;
                        uVar10 = (undefined2)((ulong)uStack28 >> 0x10);
                        uVar8  = 0x6;
                    }
                    else
                    {
                        if(iVar4 == 0x8)
                        {
                            pass1_1008_3e76(param_4, local_8, uStack64, uStack72);
                            uVar11 = (undefined2)uStack52;
                            uVar12 = (undefined2)(uStack52 >> 0x10);
                            uVar9  = (undefined2)uStack28;
                            uVar10 = (undefined2)((ulong)uStack28 >> 0x10);
                            uVar8  = 0x7;
                        }
                        else
                        {
                            if(iVar4 != 0x9)
                                goto LAB_1028_ce2c;
                            pass1_1008_3e76(param_4, local_8, uStack64, uStack72);
                            uVar11 = (undefined2)uStack52;
                            uVar12 = (undefined2)(uStack52 >> 0x10);
                            uVar9  = (undefined2)uStack28;
                            uVar10 = (undefined2)((ulong)uStack28 >> 0x10);
                            uVar8  = 0x8;
                        }
                    }
                    struct_op_1028_87f0(param_2,
                                        param_1,
                                        (astruct_97 *)CONCAT22(param_2, &local_178),
                                        0x0,
                                        0x0,
                                        uVar8,
                                        (ulong *)param_4,
                                        (ushort)((ulong)param_4 >> 0x10),
                                        CONCAT22(uVar10, uVar9),
                                        CONCAT22(uVar12, uVar11));
                    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_2, &local_178));
                    local_178 = 0x389a;
                    uStack374 = 0x1008;
                }
            LAB_1028_ce2c:
            }
        }
    }
    return;
}


void __stdcall16far pass1_1028_d078(ushort param_1, ulong param_2, ulong param_3)

{
    code      **ppcVar1;
    uchar      *extraout_DX;
    uchar      *puVar2;
    int         iVar3;
    undefined2  uVar4;
    ushort     *puVar5;
    ulong       uVar6;
    undefined   local_16[0x4];
    undefined4 *puStack18;
    uchar      *puStack16;
    undefined4  uStack14;
    uint        uStack10;
    uint        uStack8;
    undefined4 *puStack6;
    uint        uStack4;

    uVar4     = (undefined2)(param_2 >> 0x10);
    iVar3     = (int)param_2;
    puStack6  = (undefined4 *)*(uint *)(iVar3 + 0x4);
    puVar2    = *(uchar **)(iVar3 + 0x6);
    uStack14  = CONCAT22(puVar2, puStack6);
    puStack18 = puStack6;
    puStack16 = puVar2;
    if(((uint)puVar2 | (uint)puStack6) != 0x0)
    {
        ppcVar1 = (code **)*puStack6;
        (**ppcVar1)();
        puVar2 = extraout_DX;
    }
    mem_op_1000_179c(0x1c, puVar2, 0x1000);
    uStack4   = (uint)puVar2 | (uint)puStack6;
    puStack18 = puStack6;
    puStack16 = puVar2;
    if(uStack4 == 0x0)
    {
        puStack6 = (undefined4 *)0x0;
        uStack4  = 0x0;
    }
    else
    {
        struct_op_1008_8e9e((astruct_78 *)CONCAT22(puVar2, puStack6), 0x6, 0x24);
    }
    *(undefined2 *)(iVar3 + 0x4) = puStack6;
    *(uint *)(iVar3 + 0x6)       = uStack4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_3, (uint)(param_3 >> 0x10));
    if((uchar *)(uStack4 | (uint)puStack6) == (uchar *)0x0)
    {
        puVar5 = pass1_1018_dcf6((ushort *)CONCAT22(param_1, local_16));
        uVar6  = pass1_1018_dd1e(
          param_1, (uint)local_16, (uchar *)((ulong)puVar5 >> 0x10), (ushort)local_16, param_1, 0x0, 0xa0000);
        pass1_1008_8faa(*(undefined4 *)(iVar3 + 0x4), uVar6);
        return;
    }
    uVar6    = pass1_1038_565e(param_1, (uchar *)(uStack4 | (uint)puStack6), CONCAT22(uStack4, puStack6));
    uStack8  = (uint)(uVar6 >> 0x10);
    uStack10 = (uint)uVar6;
    if((uStack8 | uStack10) != 0x0)
    {
        pass1_1028_d172(param_1, param_2, uVar6 & 0xffff | (ulong)uStack8 << 0x10);
    }
    return;
}


astruct_100 *__stdcall16far struct_op_1028_d1dc(ushort param_1, uchar param_2, astruct_100 *param_3, ushort param_4)

{
    astruct_101 *iVar1;
    uchar       *puVar1;
    ushort       in_stack_0000fffa;

    puVar1             = (uchar *)((ulong)param_3 >> 0x10);
    iVar1              = (astruct_101 *)param_3;
    param_3->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    iVar1->field_0x4   = param_4;
    iVar1->field_0x6   = 0x0;
    param_3->field_0x0 = 0x6ad2;
    iVar1->field_0x2   = (int)&USHORT_1050_1028;
    sys_1000_3f9c(&iVar1->field_0x8,
                  puVar1,
                  0x5160,
                  (ushort)&USHORT_1050_1050,
                  in_stack_0000fffa,
                  &stack0xfffe,
                  puVar1,
                  0x1000,
                  param_1,
                  param_2);
    return param_3;
}
