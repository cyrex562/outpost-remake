use crate::block_1000;
use crate::block_1000::block_1000_1000;
use crate::utils::CONCAT22;

pub fn pass1_1000_201c(mut param_1: i16,
                       mut param_2: i16)
{
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut BVar4: bool;
    let mut i_var5: i16;
    let mut u_var6: u16;

    if (param_1 == 0x0) {
        (param_2 + 0x6) = 0x0;
        (param_2 + 0x4) = 0x0;
    }
    u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    while (u_var3 != 0x0) {
        BVar4 = pass1_1000_206c((param_2 + 0x4),
                                (param_2 + 0x6));
        if (BVar4 == 0x0) {
            u_var2 = (u32) (param_2 + 0x4);
            u_var6 =  ((u32) u_var2 >> 0x10);
            i_var5 = u_var2;
            u_var1 = (i_var5 + 0x2c);
            (param_2 + 0x4) = (i_var5 + 0x2a);
            (param_2 + 0x6) = u_var1;
        } else {
            block_1000_1000::mem_op_1000_1b9a(0x1,
                                              *(u16 *) (param_2 + 0x4),
                                              (param_2 + 0x6));
        }
        u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}

pub fn pass1_1000_20a2(mut param_1: u16,
                       mut param_2: u16 )
{
    let mut i_var1: i16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_stack8: u16;
    let mut u_stack4: u16;

    i_var1 = (param_1 + 0x2e);
    u_var2 = (param_1 + 0x30);
    u_stack8 = 0x0;
    u_var3 = (i_var1 + 0x4);
    u_stack4 = (i_var1 + 0x6);
    u_var7 = 0x0;
    if ((u_stack4 | u_var3) != 0x0) {
        while ((u_var6 = u_var3, u_var4 = u_stack4, u_var6 != param_1 || (u_stack4 != param_2))) {
            u_var3 = (u_var6 + 0x2a);
            u_stack4 = (u_var6 + 0x2c);
            u_var7 = u_var6;
            u_stack8 = u_var4;
            if ((u_stack4 | u_var3) == 0x0) {
                return;
            }
        }
        if ((u_stack8 | u_var7) != 0x0) {
            u_var2 = (u_var6 + 0x2c);
            (u_var7 + 0x2a) = (u_var6 + 0x2a);
            (u_var7 + 0x2c) = u_var2;
            return;
        }
        u_var5 = (u_var6 + 0x2c);
        (i_var1 + 0x4) = (u_var6 + 0x2a);
        (i_var1 + 0x6) = u_var5;
    }
}

pub fn empty_fn_1000_214a()
{
    return;
}


// WARNING: Removing unreachable block (ram,0x1000234c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn init_1000_23be(mut param_1: *mut u8, mut param_2: u16 )
{
    init_op_1008_54aa( &DAT_1050_1050,
                      param_1,
                      param_2,
                      PTR_LOOP_1050_5f52,
                      CONCAT22(PTR_LOOP_1050_5f50,
                               PTR_LOOP_1050_5f4e),
                      PTR_LOOP_1050_5f4a,
                       HINSTANCE16_1050_5f4c);
    return;
}


// WARNING: Removing unreachable block (ram,0x10002557)
pub fn fn_ptr_op_1000_24cd(mut param_1: u16 )
{
    code *pcVar1;
    let mut i_var2: i16;
    let mut u_var2: u16;
    let mut u_var6: u16;
    let mut u_var5: u16;
    let mut u_var3: u16;
    u16_t uVar4;

    u8_1050_5fc9 = '\0';
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    block_1000::ret_op_1000_55ac();
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    dos3_op_1000_256b();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    return;
}


// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)
pub fn pass1_1000_24db(mut param_1: u16 )
{
    code *pcVar1;
    let mut unaff_bp: i16;
    let mut i_var2: i16;

    i_var2 = unaff_bp + 0x1;
    u8_1050_5fc9 = '\0';
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    dos3_op_1000_256b();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)(i_var2);
    return;
}


// WARNING: Removing unreachable block (ram,0x10002589)
pub fn dos3_op_1000_256b(void)
{
    code *pcVar1;

    if (PTR_LOOP_1050_6202 != NULL) {
        ((code) PTR_LOOP_1050_6200)();
    }
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    return;
}

pub fn fn_ptr_op_1000_2594(void)
{
    code **ppcVar1;
    code **unaff_SI;
    code **unaff_DI;
    code **ppcVar2;
    code **fn_ptr_1;

    while (unaff_SI < unaff_DI) {
        ppcVar2 = unaff_DI + -0x2;
        ppcVar1 = unaff_DI + -0x1;
        unaff_DI = ppcVar2;
        if (( *ppcVar2 |  *ppcVar1) != 0x0) {
            fn_ptr_1 = ppcVar2;
            (**fn_ptr_1)();
        }
    }
    return;
}

//
pub fn pass1_1000_25a8()
{
    pass1_1000_2913(0xfc);
    pass1_1000_2913(0xff);
    return;
}


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram
pub fn pass1_1000_262c(param_1: u32,
                       param_2: u16,
                       param_3: u16)
{
    char *pcVar1;
    char cVar2;
    let mut i_var3: i16;
    let mut i_var5: i16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut i_var4: i16;
    char **ppcVar6;
    char *pcVar7;
    char *pcVar8;
    char *pcVar9;
    let mut unaff_es: u16;
    let mut u_var12: u16;
    let mut u_var3: u16;
    u8 *puVar3;

    PTR_LOOP_1050_5fd2 = param_2;
    PTR_LOOP_1050_5fd4 = param_3;
    param_3 =  0x263d;
    param_2 =  pass1_1000_2950(0x8,
                                     param_1,
                                     0x104);
    param_3 = param_1;
    PTR_LOOP_1050_5fc2 = param_2;
    PTR_LOOP_1050_5fc4 = param_1;
    i_var5 = GetModuleFileName16(0x104,
                                 (char *) CONCAT22(param_1,
                                                  param_2),
                                 HINSTANCE16_1050_5f4c);
    param_2[i_var5] = '\0';
    i_var4 = 0x1;
    PTR_LOOP_1050_5fb8 =  ( &PTR_LOOP_1050_0000 + 0x1);
    pcVar7 = (char *) ( s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);//
//    LAB_1000_266c:
    do {
        do {
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
        } while (cVar2 == ' ');
    } while (cVar2 == '\t');
    if ((cVar2 != '\r') && (cVar2 != '\0')) {
        PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 0x1;
        do {
            pcVar7 = pcVar7 + -0x1;//
//            LAB_1000_267f:
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
            if ((cVar2 == ' ') || (cVar2 == '\t')) {
                goto LAB_1000_266c;
            }
            if ((cVar2 == '\r') || (cVar2 == '\0')) {
                break;
            }
            if (cVar2 == '\"') {//
//                LAB_1000_26b8:
                do {
                    loop {
                        loop {
                            pcVar1 = pcVar7;
                            pcVar7 = pcVar7 + 0x1;
                            cVar2 = *pcVar1;
                            if ((cVar2 == '\r') || (cVar2 == '\0')) {
                                goto LAB_1000_26e8;
                            }
                            if (cVar2 == '\"') {
                                goto LAB_1000_267f;
                            }
                            if (cVar2 == '\\') {
                                break;
                            }
                            i_var4 += 0x1;
                        }
                        u_var7 = 0x0;
                        do {
                            pcVar9 = pcVar7;
                            u_var7 += 0x1;
                            pcVar7 = pcVar9 + 0x1;
                            cVar2 = *pcVar9;
                        } while (cVar2 == '\\');
                        if (cVar2 == '\"') {
                            break;
                        }
                        i_var4 += u_var7;
                        pcVar7 = pcVar9;
                    }
                    i_var4 = i_var4 + (u_var7 >> 0x1) +  ((u_var7 & 0x1) != 0x0);
                } while ((u_var7 & 0x1) != 0x0);
                goto LAB_1000_267f;
            }
            if (cVar2 != '\\') {
                i_var4 += 0x1;
                goto LAB_1000_267f;
            }
            u_var6 = 0x0;
            do {
                u_var6 += 0x1;
                pcVar1 = pcVar7;
                pcVar7 = pcVar7 + 0x1;
                cVar2 = *pcVar1;
            } while (cVar2 == '\\');
            if (cVar2 == '\"') {
                i_var4 = i_var4 + (u_var6 >> 0x1) +  ((u_var6 & 0x1) != 0x0);
                if ((u_var6 & 0x1) == 0x0) {
                    goto LAB_1000_26b8;
                }
                goto LAB_1000_267f;
            }
            i_var4 += u_var6;
        } while (true);
    }//
//    LAB_1000_26e8:
    param_3 =  &DAT_1050_1050;
    i_var3 = -( (PTR_LOOP_1050_5fb8 +  (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var4 + 0x1) & 0xfffe);
    PTR_LOOP_1050_5fba = &stack0x0004 + i_var3;
    PTR_LOOP_1050_5fbc =  &DAT_1050_1050;
    pcVar9 = &stack0x0004 +  (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var3;
    ( &param_3 + i_var3) =  &DAT_1050_1050;
    puVar3 = PTR_LOOP_1050_5fc4;
    u_var12 = ( &param_3 + i_var3);
    *(u8 **) (&stack0x0004 + i_var3) = PTR_LOOP_1050_5fc2;
    *(u8 **) (&stack0x0006 + i_var3) = puVar3;
    ppcVar6 = (char **) (&stack0x0008 + i_var3);
    *(u8 **) ( &param_3 + i_var3) = &stack0x0004 + i_var3;
    ( &param_2 + i_var3) =  s_tile2_bmp_1050_1538;
    (&stack0xfffe + i_var3) = 0x271f;
    u_var4 = pass1_1000_29dc( &DAT_1050_1050);
    u_var3 = &PTR_LOOP_1050_5f7e;
    pcVar7 = (char *) ( s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);//
//    LAB_1000_272e:
    do {
        do {
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
        } while (cVar2 == ' ');
    } while (cVar2 == '\t');
    if ((cVar2 == '\r') || (cVar2 == '\0')) {//
//        LAB_1000_27c1:
        ( &param_3 + i_var3) =  s_tile2_bmp_1050_1538;
        ( &param_2 + i_var3) = 0x27c5;
        u_var5 = pass1_1000_29dc( &DAT_1050_1050);
        *ppcVar6 = NULL;
        ppcVar6[0x1] = NULL;
        // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
        // WARNING: Treating indirect jump as call
        ((code) (u32) &PTR_LOOP_1050_5fd2)();
        _PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4,
                                       PTR_LOOP_1050_5fc2);
        return;
    }
    *ppcVar6 = pcVar9;
    ppcVar6[0x1] = (char *) &DAT_1050_1050;
    ppcVar6 = ppcVar6 + 0x2;
    do {
        pcVar7 = pcVar7 + -0x1;//
//        LAB_1000_274f:
        pcVar1 = pcVar7;
        pcVar7 = pcVar7 + 0x1;
        cVar2 = *pcVar1;
        if ((cVar2 == ' ') || (cVar2 == '\t')) {
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\0';
            goto LAB_1000_272e;
        }
        if ((cVar2 == '\r') || (cVar2 == '\0')) {//
//            LAB_1000_27be:
            *pcVar9 = '\0';
            goto LAB_1000_27c1;
        }
        pcVar8 = pcVar7;
        if (cVar2 == '\"') {//
//            LAB_1000_278b:
            loop {
                pcVar7 = pcVar8 + 0x1;
                cVar2 = *pcVar8;
                if ((cVar2 == '\r') || (cVar2 == '\0')) {
                    goto LAB_1000_27be;
                }
                if (cVar2 == '\"') {
                    break;
                }
                if (cVar2 == '\\') {
                    u_var10 = 0x0;
                    do {
                        pcVar8 = pcVar7;
                        u_var10 += 0x1;
                        pcVar7 = pcVar8 + 0x1;
                        cVar2 = *pcVar8;
                    } while (cVar2 == '\\');
                    if (cVar2 == '\"') {
                        for (uVar11 = uVar10 >> 0x1; uVar11 != 0x0; uVar11 -= 0x1) {
                            pcVar1 = pcVar9;
                            pcVar9 = pcVar9 + 0x1;
                            *pcVar1 = '\\';
                        }
                        if ((u_var10 & 0x1) == 0x0) {
                            break;
                        }
                        pcVar1 = pcVar9;
                        pcVar9 = pcVar9 + 0x1;
                        *pcVar1 = '\"';
                        pcVar8 = pcVar7;
                    } else {
                        for (; uVar10 != 0x0; uVar10 -= 0x1) {
                            pcVar1 = pcVar9;
                            pcVar9 = pcVar9 + 0x1;
                            *pcVar1 = '\\';
                        }
                    }
                } else {
                    pcVar1 = pcVar9;
                    pcVar9 = pcVar9 + 0x1;
                    *pcVar1 = cVar2;
                    pcVar8 = pcVar7;
                }
            }
            goto LAB_1000_274f;
        }
        if (cVar2 != '\\') {
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = cVar2;
            goto LAB_1000_274f;
        }
        u_var8 = 0x0;
        do {
            u_var8 += 0x1;
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
        } while (cVar2 == '\\');
        if (cVar2 == '\"') {
            for (uVar9 = uVar8 >> 0x1; uVar9 != 0x0; uVar9 -= 0x1) {
                pcVar1 = pcVar9;
                pcVar9 = pcVar9 + 0x1;
                *pcVar1 = '\\';
            }
            pcVar8 = pcVar7;
            if ((u_var8 & 0x1) == 0x0) {
                goto LAB_1000_278b;
            }
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\"';
            goto LAB_1000_274f;
        }
        for (; uVar8 != 0x0; uVar8 -= 0x1) {
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\\';
        }
    } loop;
}

pub fn pass1_1000_27d6(mut param_1: u16 )
{
    let mut piVar2: *mut i16;
    char *pcVar3;
    char cVar4;
    let mut pu_var5: *mut u16;
    u16 **ppuVar6;
    let mut i_var7: i16;
    let mut u_var7: u16;
    let mut i_var8: i16;
    let mut pu_var7: *mut u16;
    let mut pu_var8: *mut u16;
    let mut i_var9: i16;
    let mut piVar9: *mut i16;
    let mut piVar10: *mut i16;
    char *pcVar11;
    let mut piVar12: *mut i16;
    let mut b_var13: bool;
pub fn *dos_env;
    let mut pu_var14: *mut u16;
    let mut piVar1: *mut i16;
    let mut pu_var4: *mut u16;
    let mut piVar4: *mut i16;

    dos_env = GetDOSEnvironment16();
    pu_var7 = (u16 *) ((u32) dos_env >> 0x10);
    if ( dos_env != 0x0) {
        pu_var7 = NULL;
    }
    i_var9 = 0x0;
    pcVar11 = NULL;
    i_var7 = -0x1;
    if (pu_var7 != NULL) {
        cVar4 = *NULL;
        while (cVar4 != '\0') {
            do {
                if (i_var7 == 0x0) {
                    break;
                }
                i_var7 += -0x1;
                pcVar3 = pcVar11;
                pcVar11 = pcVar11 + 0x1;
            } while (*pcVar3 != '\0');
            i_var9 += 0x1;
            pcVar3 = pcVar11;
            pcVar11 = pcVar11 + 0x1;
            cVar4 = *pcVar3;
        }
    }
    u_var7 = 0x9;
    pu_var8 = pu_var7;
    pu_var5 = pass1_1000_2950(0x9,
                              pu_var7,
                              (pcVar11 + 0x1) & 0xfffe);
    pu_var14 = pu_var8;
    ppuVar6 = (u16 **) pass1_1000_2950(u_var7,
                                       pu_var8,
                                       (i_var9 + 0x1) * 0x4);
    piVar9 = NULL;
    PTR_LOOP_1050_5fbe =  ppuVar6;
    PTR_LOOP_1050_5fc0 =  pu_var8;
    do {
        if (i_var9 == 0x0) {
            *ppuVar6 = NULL;
            ppuVar6[0x1] = NULL;
            return;
        }
        b_var13 = *piVar9 == s__C_FILE_INFO__1050_5f5c._0_2_;
        if (b_var13) {
            piVar12 =  s__C_FILE_INFO__1050_5f5c;
            i_var8 = 0x6;
            piVar10 = piVar9;
            do {
                if (i_var8 == 0x0) {
                    break;
                }
                i_var8 += -0x1;
                piVar4 = piVar12;
                piVar12 = piVar12 + 0x1;
                piVar1 = piVar10;
                piVar10 = piVar10 + 0x1;
                b_var13 = *piVar1 == *piVar4;
            } while (b_var13);
            if (!b_var13) {
                goto LAB_1000_2867;
            }
        } else {//
//            LAB_1000_2867:
            *ppuVar6 = pu_var5;
            ppuVar6[0x1] = pu_var14;
            ppuVar6 = ppuVar6 + 0x2;
        }
        do {
            piVar2 = piVar9;
            piVar9 =  ( piVar9 + 0x1);
            cVar4 = piVar2;
            pu_var4 = pu_var5;
            pu_var5 = (u16 *) ( pu_var5 + 0x1);
            pu_var4 = cVar4;
        } while (cVar4 != '\0');
        i_var9 += -0x1;
    } while (true);
}

pub fn pass1_1000_2913(mut param_1: i16)
{
    char *pcVar1;
    char *pcVar2;
    let mut i_var3: i16;
    let mut unaff_di: u16;
    let mut unaff_es: u16;
    struct astruct_825 *paVar4;
    let mut i_var5: i16;

    i_var5 = (i16) &DAT_1050_1050;
    if (u16_1050_61ec != 0x0) {
        paVar4 = (astruct_825 *) CONCAT22(unaff_di,
                                          param_1);
        pcVar2 = poss_str_op_1000_28dc(paVar4);
        if (pcVar2 != NULL) {
            i_var3 = -0x1;
            do {
                if (i_var3 == 0x0) {
                    break;
                }
                i_var3 += -0x1;
                pcVar1 = pcVar2;
                pcVar2 = pcVar2 + 0x1;
            } while (*pcVar1 != '\0');
            pass1_1000_55b1( ((u32) paVar4 >> 0x10),
            i_var5);
        }
    }
    return;
}

pub fn pass1_1000_29af(mut param_1: u16 )
{
    pass1_1000_29b5(param_1 & 0xff);
    return;
}

pub fn pass1_1000_29b5(mut param_1: u16 )
{
    char cVar1;

    PTR_LOOP_1050_5f88._0_1_ = (u8) param_1;
    cVar1 = (char) (param_1 >> 0x8);
    if (cVar1 != '\0') {
        goto LAB_1000_29d2;
    }
    if ((u8) PTR_LOOP_1050_5f88 < 0x22) {
        if ((u8) PTR_LOOP_1050_5f88 < 0x20) {
            if (0x13 < (u8) PTR_LOOP_1050_5f88) {
                goto LAB_1000_29cc;
            }
        } else {
            param_1 = 0x5;
        }
    } else {//
//        LAB_1000_29cc:
        param_1 = 0x13;
    }
    cVar1 = (u32) ((param_1 & 0xff) + 0x5fd6);//
//    LAB_1000_29d2:
    PTR_LOOP_1050_5f78 =   cVar1;
    return;
}

pub fn pass1_1000_2b3c(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16,
                       mut param_4: u16,
                       mut param_5: u16,
                       mut param_6: i16)
{
    pass1_1000_2b02(param_1,
                    param_2,
                    param_3,
                    param_4,
                    param_5,
                    0x0);
    return;
}

pub fn pass1_1000_2ba0(uchar param_1)
{
    block_1000::pass1_1000_3024();
    if (u8_1050_5fc9 != '\0') {
        pass1_1000_3f5c();
    }
    return;
}

pub fn pass1_1000_2cb0(param_1: *mut u16)
{
    let mut pu_var1: *mut u16;
    u8 bVar2;

    bVar2 = * (param_1 + 0x5);
    if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
        block_1000_1000::pass1_1000_16ee(param_1[0x3],
                                         param_1[0x4]);
        pu_var1 = param_1 + 0x5;
        * pu_var1 = * pu_var1 & 0xf7;
        param_1[0x3] = 0x0;
        param_1[0x4] = 0x0;
        *param_1 = 0x0;
        param_1[0x1] = 0x0;
        param_1[0x2] = 0x0;
    }
    return;
}

pub fn mem_1000_2ce8(mut param_1: u16,
                     i16 *param_2)
{
    let mut piVar1: *mut i16;
    let mut u_var2: u16;

    u_var2 = block_1000_1000::mem_1000_167a(param_1,
                                            0x200);
    if (param_1 == 0x0) {
        piVar1 = param_2 + 0x5;
        * piVar1 = * piVar1 | 0x4;
        param_2[0x79] = 0x1;
        param_1 =  &DAT_1050_1050;
        u_var2 =  param_2 + 0xf1;
    } else {
        piVar1 = param_2 + 0x5;
        * piVar1 = * piVar1 | 0x8;
        param_2[0x79] = 0x200;
    }
    param_2[0x1] = param_1;
    *param_2 = u_var2;
    param_2[0x4] = param_1;
    param_2[0x3] = u_var2;
    param_2[0x2] = 0x0;
    return;
}

pub fn pass1_1000_2f00(mut param_1: i16,
                       i16 *param_2)
{
    if (((* (param_2 + 0x78) & 0x10) != 0x0)
        && ((* (* ( param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)) {
        pass1_1000_2fa4(param_2);
        if (param_1 != 0x0) {
            * (param_2 + 0x78) = 0x0;
            param_2[0x79] = 0x0;
            *param_2 = 0x0;
            param_2[0x1] = 0x0;
            param_2[0x3] = 0x0;
            param_2[0x4] = 0x0;
        }
    }
    return;
}

pub fn pass1_1000_206c(mut param_1: u16,
                       mut param_2: u16 ) -> bool
{
    let mut uVar1: u16;

    uVar1 = pass1_1000_21d2(0x2,
                            0x42,
                            param_1,
                            param_2,
                            0x1);
    if ((uVar1 != 0x0) && ((param_1 + 0x14) == -0x4153)) {
        return 0x1;
    }
    return 0x0;
}

pub fn ret_true_1000_2146() -> u16
{
    return 0x1;
}

pub fn msg_box_op_1000_214c(mut param_1: u16,
                            mut param_2: i16,
                            mut param_3: u16,
                            mut param_4: u16 ) -> bool
{
    INT16 IVar1;
    let mut iVar2: i16;
    let mut type: u16;

    type = 0x2 - (param_2 == 0x0) | 0x2110;
    MessageBeep16(0x0);
    do {
        IVar1 = MessageBox16(type,
                             "SmartHeap Library",
                             (char *) CONCAT22(param_4,
                                               param_3),
                             0x0);
        iVar2 = IVar1 + -0x1;
        if (iVar2 == 0x0) {
            return 0x0;
        }
        if ((0x0 < iVar2) && (!SBORROW2(iVar2,
                                        0x1))) {
            if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
                block_1000::fatal_app_exit_1000_3e9e();
                return 0x0;
            }
            if (IVar1 == 0x4) {
                return 0x1;
            }
            if (IVar1 == 0x5) {
                return 0x0;
            }
        }
        if ((type & 0x2000) == 0x0) {
            return 0x0;
        }
        type = type & 0xdfef | 0x1010;
    } loop;
}

pub fn mem_op_1000_21b6(mut param_1: u16,
                        mut param_2: u16 ) -> bool
{
    let mut BVar1: bool;

    BVar1 = mem_op_1000_1dfa(0x0,
                             0x4,
                             param_1,
                             param_2);
    return BVar1 == 0x0;
}

pub fn pass1_1000_21d2(param_1: u8,
                       param_2: i32,
                       mut param_3: u16,
                       mut param_4: u16,
                       param_5: u8) -> u16
{
    let mut uVar1: u32;
    let mut BVar2: bool;

    BVar2 = mem_op_1000_1dfa(0x0,
                             param_1,
                             param_3,
                             param_4);
    if (BVar2 == 0x0) {
        if ((param_1 & 0x4) == 0x0) {
            uVar1 = SegmentLimit((u32) param_4);
            if ((bool) ((u8) ((u16) uVar1 >> 0x10) & 0x1)) {
                if (param_2 == 0x0) {
                    return 0x1;
                }
                if ((!CARRY4((u32) param_3,
                             param_2 - 0x1U)) && ((u32) param_3 + (param_2 - 0x1U) <= (u32)  uVar1)) {
                    return 0x1;
                }
            }
        } else {
            BVar2 = pass1_1000_22c0( param_2,
                                    _param_1,
                                    param_2,
                                    param_3,
                                    param_4);
            if (BVar2 != 0x0) {
                return 0x1;
            }
        }
    }
    return 0x0;
}

pub fn pass1_1000_2242(mut param_1: u16,
                       param_2: *mut u8,
                       mut param_3: u16,
                       mut param_4: u16,
                       mut param_5: u16,
                       mut param_6: i16) -> u32
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut bVar3: bool;

    uVar1 = param_4 | param_3;
    loop {
        if (uVar1 == 0x0) {
            return 0x0;
        }
        uVar1 = param_3;
        if (param_4 != 0x0) {
            uVar1 = 0xffff;
        }
        if (CARRY2(param_5,
                   uVar1) != false) {
            uVar1 = -param_5;
        }
        bVar3 = param_3 < uVar1;
        param_3 -= uVar1;
        param_4 -= bVar3;
        uVar2 = ((code) param_2)(uVar1,
                                 param_1,
                                 param_5,
                                 param_6);
        if (uVar2 != 0x0) {
            break;
        }
        bVar3 = CARRY2(param_5,
                       uVar1);
        param_5 += uVar1;
        param_6 +=  bVar3 * 0x100;
        uVar1 = param_4 | param_3;
    }
    return CONCAT22(param_4 + CARRY2(uVar2,
                                     param_3),
                    uVar2 + param_3);
}

pub fn pass1_1000_22c0(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16,
                       mut param_4: u16,
                       mut param_5: u16 ) -> bool
{
    let mut u32_var1: u32;

    u32_var1 = pass1_1000_2242(param_2,
                                0x1dfa,
                               param_1,
                               param_3,
                               param_4,
                               param_5);
    if (u32_var1 == 0x0) {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1000_25d2(mut param_1: i16,
                       mut param_2: i16,
                       fn_ptr_param_3: code2,
                       mut param_4: i16) -> *mut i16
{
    let mut piVar1: *mut i16;
    char *pcVar2;
    u8 *puVar3;
    StructD *pstruct_d_var4;
    let mut piVar5: *mut i16;
    char *pcVar6;
    let mut iVar7: i16;
    let mut piVar8: *mut i16;
    char *pcVar9;
    struct astruct_825 *paVar10;

    puVar3 =  (param_1 + 0x1U & 0xfffe);
    if ((puVar3 < &param_2)
        && (pstruct_d_var4 = (StructD * ) - ( puVar3 -  &param_2), PTR_LOOP_1050_000a <= pstruct_d_var4.address_offset_field_0x0)) {
        if (pstruct_d_var4.address_offset_field_0x0 < PTR_LOOP_1050_000c) {
            PTR_LOOP_1050_000c = pstruct_d_var4.address_offset_field_0x0;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
        // WARNING: Treating indirect jump as call
        piVar5 =  fn_ptr_param_3();
        return piVar5;
    }
    paVar10 = (astruct_825 *) ((u32)  param_2 << 0x10);
    iVar7 = 0x0;
    pass1_1000_25a8();
    pass1_1000_2913(iVar7);
    pcVar6 = poss_str_op_1000_28dc(paVar10);
    if (pcVar6 != NULL) {
        iVar7 = 0x9;
        if (*pcVar6 == 'M') {
            iVar7 = 0xf;
        }
        pcVar6 = pcVar6 + iVar7;
        iVar7 = 0x22;
        pcVar9 = pcVar6;
        do {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            pcVar2 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
        } while (*pcVar2 != '\r');
        pcVar9[-0x1] = '\0';
    }
    FatalAppExit16((char *) CONCAT22(0x1050,
                                     pcVar6),
                   0x0);
    FatalExit();
    piVar5 =  &PTR_LOOP_1050_63fe;
    do {
        piVar1 = piVar5;
        piVar5 = piVar5 + 0x1;
        iVar7 = *piVar1;
        piVar8 = piVar5;
        if ((iVar7 == param_4) || (piVar8 =  (iVar7 + 0x1), piVar8 == NULL)) {
            return piVar8;
        }
        iVar7 = -0x1;
        do {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            piVar1 = piVar5;
            piVar5 =  ( piVar5 + 0x1);
        } while (piVar1 != '\0');
    } loop;
}

pub fn poss_str_op_1000_28dc(param_1: *mut astruct_825) -> *mut char
{
    struct astruct_825 **ppaVar1;
    char* piVar2;
    let mut iVar3: i16;
    char* string_var3;
    struct astruct_825 *iVar2;

    string_var3 = PTR_LOOP_1050_63fe;
    do {
        ppaVar1 = (astruct_825 **) string_var3;
        string_var3 = (PCHAR)(string_var3 + 0x2);
        iVar2 = *ppaVar1;
        piVar2 = string_var3;
        if ((iVar2 == (astruct_825 *) param_1) || (piVar2 = (PCHAR)(iVar2 + 0x1), piVar2 == NULL)) {
            return (PCHAR)(astruct_825 * *)
            piVar2;
        }
        iVar3 = -0x1;
        do {
            if (iVar3 == 0x0) {
                break;
            }
            iVar3 += -0x1;
            ppaVar1 = (astruct_825 **) string_var3;
            string_var3 = (PCHAR)(string_var3 + 0x1);
        } while (ppaVar1 != '\0');
    } loop;
}

pub fn pass1_1000_2950(mut param_1: i16,
                       mut param_2: u16,
                       mut param_3: u16 ) -> *mut u16
{
    u16_t *puVar1;
    u16_t uVar2;
    char *pcVar3;
    u8 *puVar4;
    char *pcVar5;
    let mut iVar6: i16;
    u16_t *puVar7;
    u16_t *puVar8;
    u16_t unaff_BP;
    char *pcVar9;
    let mut unaff_ES: u16;
    struct astruct_825 *paVar10;

    puVar4 = PTR_LOOP_1050_6066;
    PTR_LOOP_1050_6066 =  &PTR_LOOP_1050_1000;
    puVar8 = (u16_t *) mem_1000_167a(param_2,
                                     param_3);
    PTR_LOOP_1050_6066 = puVar4;
    if ((param_2 |  puVar8) != 0x0) {
        return puVar8;
    }
    paVar10 = (astruct_825 *) CONCAT22(unaff_ES,
                                       param_1);
    pass1_1000_25a8();
    pass1_1000_2913(param_1);
    pcVar5 = poss_str_op_1000_28dc(paVar10);
    if (pcVar5 != NULL) {
        iVar6 = 0x9;
        if (*pcVar5 == 'M') {
            iVar6 = 0xf;
        }
        pcVar5 = pcVar5 + iVar6;
        iVar6 = 0x22;
        pcVar9 = pcVar5;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            pcVar3 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
        } while (*pcVar3 != '\r');
        pcVar9[-0x1] = '\0';
    }
    FatalAppExit16((char *) CONCAT22(0x1050,
                                     pcVar5),
                   0x0);
    FatalExit();
    puVar8 = (u16_t * ) & PTR_LOOP_1050_63fe;
    do {
        puVar1 = puVar8;
        puVar8 = puVar8 + 0x1;
        uVar2 = *puVar1;
        puVar7 = puVar8;
        if ((uVar2 == unaff_BP) || (puVar7 = (u16_t * )(uVar2 + 0x1), puVar7 == NULL)) {
            return puVar7;
        }
        iVar6 = -0x1;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            puVar1 = puVar8;
            puVar8 = (u16_t * )( puVar8 + 0x1);
        } while (puVar1 != '\0');
    } loop;
}

pub fn pass1_1000_29dc(mut param_1: u16 ) -> u16
{
    if (___EXPORTEDSTUB != (code) 0xb8) {
        return (u16_t) & DAT_1050_1050;
    }
    return uRam100029ed;
}

pub fn pass1_1000_2a00(param_1: *mut u16) -> u16
{
    let mut uVar1: u16;
    let mut bVar2: bool;
    let mut piVar3: *mut i16;
    let mut uVar4: u16;
    let mut unaff_BP: i16;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    u8 *puStack20;
    char local_10;
    u8 uStack15;
    u8 local_e[0x8];
    let mut uStack6: u16;
    let mut local_4: u16;
    let mut iStack2: i16;

    iStack2 = unaff_BP + 0x1;
    local_4 = SUB42(&DAT_1050_1050,
                    0x0);
    uVar5 = 0xffff;
    if ((* (param_1 + 0x5) & 0x40) != 0x0) {
        * (param_1 + 0x5) = 0x0;
        return 0xffff;
    }
    if ((* (param_1 + 0x5) & 0x83) == 0x0) {
        goto LAB_1000_2af2;
    }
    uVar5 = pass1_1000_2fa4( param_1);
    uStack6 = param_1[0x7a];
    pass1_1000_2cb0(param_1);
    uVar1 =  * ( param_1 + 0xb);
    if ( u16_1050_5f8a <  uVar1) {
        piVar3 = pass1_1000_55b1(unaff_CS,
                                 uVar1);
        if ( piVar3 < 0x0) {
            goto LAB_1000_2a6a;
        }//
//        LAB_1000_2a82:
        bVar2 = false;
    } else {
        uVar4 = dos3_call_op_1000_35fe( * ( param_1 + 0xb),
                                       (i16) &iStack2);
        if (-0x1 <  uVar4) {
            goto LAB_1000_2a82;
        }//
//        LAB_1000_2a6a:
        bVar2 = true;
    }
    if (!bVar2) {
        if (uStack6 == 0x0) {
            goto LAB_1000_2af2;
        }
        block_1000::unk_str_op_1000_3d3e((char *) CONCAT22(0x1050,
                                                           &local_10),
                                         s___1050_5fea);
        puStack20 = local_e;
        if (local_10 == '\\') {
            puStack20 = &uStack15;
        } else {
            pass1_1000_3cea(CONCAT22(0x1050,
                                     &local_10),
                            s___1050_5fec);
        }
        pass1_1000_3e82(uStack6,
                         CONCAT22(0x1050,
                                        puStack20),
                        0xa);
        uVar4 = dos3_call_1000_514e();
        if (uVar4 == 0x0) {
            goto LAB_1000_2af2;
        }
    }
    uVar5 = 0xffff;//
//    LAB_1000_2af2:
    * (param_1 + 0x5) = 0x0;
    return uVar5;
}

pub fn pass1_1000_2b02(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16,
                       mut param_4: u16,
                       mut param_5: u16,
                       param_6: u8) -> *mut u16
{
    let mut puVar1: *mut u16;

    puVar1 = pass1_1000_35aa();
    if ((param_1 |  puVar1) == 0x0) {
        puVar1 = NULL;
    } else {
        puVar1 = pass1_1000_2d34(param_2,
                                 param_3,
                                  CONCAT22(param_5,
                                                 param_4),
                                 param_6,
                                 puVar1);
    }
    return puVar1;
}

pub fn pass1_1000_2b5c(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16,
                       mut param_4: u16 ) -> u16
{
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = pass1_1000_2e74((u16 *) param_1);
    uVar2 = FUN_1000_30b4();
    pass1_1000_2f00(uVar1,
                     param_1);
    return uVar2;
}

pub fn mem_1000_2bb6(mut param_1: u16,
                     mut param_2: u16,
                     i16 *param_3) -> u16
{
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut piVar3: *mut i16;
    u8 bVar4;
    u8 *puVar5;
    u8 *puVar6;
    u8 *puVar7;

    piVar3 = param_3;
    bVar4 = * (param_3 + 0x5);
    if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
        param_3[0x2] = 0x0;
        if ((bVar4 & 0x1) != 0x0) {
            if ((bVar4 & 0x10) == 0x0) {
                goto LAB_1000_2c37;
            }
            *param_3 = param_3[0x3];
            bVar4 &= 0xfe;
        }
        * (param_3 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7 =   * ( param_3 + 0xb);
        if (((bVar4 & 0x8) == 0x0) && (((bVar4 & 0x4) != 0x0 || (((* (param_3 + 0x78) & 0x1) == 0x0 && ((
            (u16_1050_61ec != 0x0
                && (((param_3 ==  0x621c || (param_3 ==  0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0))))
                || (mem_1000_2ce8(param_1,
                                  param_3), (* (piVar3 + 0x5) & 0x8) == 0x0)))))))) {
            puVar5 = mixed_dos3_call_1000_39f2(puVar7,
                                               (char *) CONCAT22(0x1050,
                                                                 &param_2),
                                                ( &PTR_LOOP_1050_0000 + 0x1));
            puVar6 =  ( &PTR_LOOP_1050_0000 + 0x1);
        } else {
            iVar2 = piVar3[0x3];
            puVar6 =  (*piVar3 - iVar2);
            *piVar3 = iVar2 + 0x1;
            piVar3[0x2] = piVar3[0x79] + -0x1;
            if (puVar6 == NULL) {
                puVar5 = NULL;
                if ((puVar7[0x5f90] & 0x20) != 0x0) {
                    block_1000::mixed_dos3_call_1000_3636(puVar7,
                                                          0x0,
                                                          0x0,
                                                          0x2);
                    puVar5 = NULL;
                    puVar6 = puVar5;
                }
            } else {
                puVar5 = mixed_dos3_call_1000_39f2(puVar7,
                                                   (char *) CONCAT22(piVar3[0x4],
                                                                     piVar3[0x3]),
                                                   puVar6);
            }
            **(u8 **) (piVar3 + 0x3) = (char) param_2;
        }
        if (puVar5 == puVar6) {
            return param_2 & 0xff;
        }
    }//
//    LAB_1000_2c37:
    piVar1 = piVar3 + 0x5;
    * piVar1 = * piVar1 | 0x20;
    return 0xffff;
}

pub fn pass1_1000_2d34(mut param_1: u16,
                       mut param_2: u16,
                       param_3: *mut u8,
                       param_4: u8,
                       param_5: *mut u16) -> *const u16
{
    u8 bVar1;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut uVar4: u16;
    let mut in_stack_0000ffd8: u16;
    u8 uStack14;
    u8 bStack8;
    u8 uStack6;

    bStack8 = (u8) PTR_LOOP_1050_6062;
    bVar3 = false;
    bVar1 = *param_3;
    if (bVar1 == 0x77) {
        uVar4 = 0x301;
    } else {
        if (0x77 < bVar1) {
            return NULL;
        }
        if (bVar1 != 0x61) {
            if (bVar1 != 0x72) {
                return NULL;
            }
            uVar4 = 0x0;
            uStack6 = 0x1;
            goto LAB_1000_2d6c;
        }
        uVar4 = 0x109;
    }
    uStack6 = 0x2;//
//    LAB_1000_2d6c:
    bVar2 = true;//
//    LAB_1000_2d71:
    param_3 =  ((u32) param_3 & 0xffff0000 | (u32) ( param_3 + 0x1));
    if ((*param_3 == 0x0) || (!bVar2)) {
        uVar4 = mixed_dos3_call_1000_370a(in_stack_0000ffd8,
                                          param_1,
                                          param_2,
                                          uVar4,
                                          param_4,
                                          0x1a4);
        if ( uVar4 < 0x0) {
            return NULL;
        }
        PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 0x1;
        * (param_5 + 0x5) = uStack6;
        param_5[0x1] = 0x0;
        *param_5 = 0x0;
        param_5[0x4] = 0x0;
        param_5[0x3] = 0x0;
        uStack14 = (u8) uVar4;
        * ( param_5 + 0xb) = uStack14;
        * (param_5 + 0x78) = bStack8;
        param_5[0x2] = 0x0;
        param_5[0x7a] = 0x0;
        return param_5;
    }
    bVar1 = *param_3;
    if (bVar1 == 0x74) {
        if ((uVar4 & 0xc000) == 0x0) {
            uVar4 |= 0x4000;
            goto LAB_1000_2d71;
        }
    } else {
        if (0x74 < bVar1) {
            goto LAB_1000_2da4;
        }
        if (bVar1 == 0x2b) {
            if ((uVar4 & 0x2) != 0x0) {
                goto LAB_1000_2da4;
            }
            uVar4 = uVar4 & 0xfffe | 0x2;
            uStack6 = 0x80;
            goto LAB_1000_2d71;
        }
        if (bVar1 == 0x62) {
            if ((uVar4 & 0xc000) == 0x0) {
                uVar4 |= 0x8000;
                goto LAB_1000_2d71;
            }
        } else {
            if (bVar1 != 0x63) {
                if ((bVar1 != 0x6e) || (bVar3)) {
                    goto LAB_1000_2da4;
                }
                bVar3 = true;
                bStack8 &= 0xbf;
                goto LAB_1000_2d71;
            }
            if (!bVar3) {
                bVar3 = true;
                bStack8 |= 0x40;
                goto LAB_1000_2d71;
            }
        }
    }//
//    LAB_1000_2da4:
    bVar2 = false;
    goto LAB_1000_2d71;
}

pub fn pass1_1000_2e74(param_1: *mut u16) -> u16
{
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u16;

    if (u16_1050_61ec != 0x0) {
        puVar5 = param_1 + 0x78;
        puVar4 = (u16 *) 0x5ff2;
        if ((param_1 == (u16 *) 0x621c) || (puVar4 = (u16 *) &PTR_LOOP_1050_5ff6, param_1 == (u16 *) 0x6228)) {
            if (((* (param_1 + 0x5) & 0xc) == 0x0) && ((* puVar5 & 0x1) == 0x0)) {
                uVar2 = *puVar4;
                uVar3 = puVar4[0x1];
                if ((uVar2 | uVar3) == 0x0) {
                    uVar2 = mem_1000_167a(uVar3,
                                          0x200);
                    if (uVar3 == 0x0) {
                        return 0x0;
                    }
                    *puVar4 = uVar2;
                    puVar4[0x1] = uVar3;
                }
                param_1[0x3] = uVar2;
                param_1[0x4] = uVar3;
                *param_1 = uVar2;
                param_1[0x1] = uVar3;
                param_1[0x2] = 0x200;
                param_1[0x79] = 0x200;
                puVar1 = param_1 + 0x5;
                * puVar1 = * puVar1 | 0x2;
                * puVar5 = 0x11;
                return 0x1;
            }
        } else if ((u8) u16_1050_5f8a <= * ( param_1 + 0xb)) {
            puVar1 = puVar5;
            * puVar1 = * puVar1 | 0x10;
        }
    }
    return 0x0;
}

pub fn pass1_1000_2f48(param_1: i32) -> u16
{
    let mut uVar1: u16;
    u8 *puVar2;

    if (param_1 == 0x0) {
        uVar1 = pass1_1000_3038(0x0);
    } else {
        uVar1 = pass1_1000_2fa4( param_1);
        if (uVar1 == 0x0) {
            if ((* ( param_1 + 0x78) & 0x40) != 0x0) {
                puVar2 = pass1_1000_400a( * (  param_1 + 0xb));
                uVar1 = - (puVar2 != NULL);
            }
        } else {
            uVar1 = 0xffff;
        }
    }
    return uVar1;
}


u16 pass1_1000_2fa4(i16 *param_1)
{
    let mut piVar1: *mut i16;
    u8 bVar2;
    let mut iVar3: i16;
    u8 *puVar4;
    u8 *puVar5;
    let mut uVar6: u16;

    uVar6 = 0x0;
    bVar2 = * (param_1 + 0x5);
    if (((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((* (param_1 + 0x78) & 0x1) != 0x0)))) {
        puVar4 =  (*param_1 - param_1[0x3]);
        if (0x0 <  puVar4) {
            puVar5 = mixed_dos3_call_1000_39f2(  * ( param_1 + 0xb),
                                               (char *) CONCAT22(param_1[0x4],
                                                                 param_1[0x3]),
                                               puVar4);
            if (puVar5 == puVar4) {
                if ((* (param_1 + 0x5) & 0x80) != 0x0) {
                    piVar1 = param_1 + 0x5;
                    * piVar1 = * piVar1 & 0xfd;
                }
            } else {
                piVar1 = param_1 + 0x5;
                * piVar1 = * piVar1 | 0x20;
                uVar6 = 0xffff;
            }
        }
    }
    iVar3 = param_1[0x4];
    *param_1 = param_1[0x3];
    param_1[0x1] = iVar3;
    param_1[0x2] = 0x0;
    return uVar6;
}
