

u8 *pass1_1000_400a(mut param_1: i16)
{
    let mut puVar1: *mut u8;

    if ((param_1 < 0x0) || ( PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
        PTR_LOOP_1050_5f78 =  ( &u16_1050_0008 + 0x1);
        puVar1 =  0xffff;
    } else if (((u16_1050_61ec == 0x0) || ((param_1 <  u16_1050_5f8a && (0x2 < param_1)))) && (0x31d
        < CONCAT11(DAT_1050_5f83,
                   DAT_1050_5f82))) {
        puVar1 = PTR_LOOP_1050_5f88;
        if (((* (param_1 + 0x5f90) & 0x1) == 0x0) || (puVar1 =  dos3_call_1000_5174(), puVar1 != NULL)) {
            PTR_LOOP_1050_5f88 = puVar1;
            PTR_LOOP_1050_5f78 =  ( &u16_1050_0008 + 0x1);
            puVar1 =  0xffff;
        }
    } else {
        puVar1 = NULL;
    }
    return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)
pub fn free_mem_1000_407a(mut param_1: u16 ,
                        mut param_2: u16 )
{
    GlobalFree16( &DAT_1050_1050);
    return;
}

mixed_sys_op_1000_40af: *mut i16(mut param_1: u16 ,
                            mut param_2: i16,
                            mut param_3: u16 )
{
    struct astruct_824 **ppaVar1;
    let mut pcVar2: *mut c_char;
    let mut puVar4: *mut u16;
    let mut pcVar5: *mut c_char;
    let mut iVar6: i16;
    struct astruct_824 **ppaVar7;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar8: i16;
    let mut hglobal_7: HGLOBAL16;
pub fn *SVar8;
    struct astruct_824 **ppaVar8;
    let mut unaff_SI: i16;
    let mut piVar9: *mut i16;
    let mut pcVar10: *mut c_char;
    let mut hglobal_di: *mut astruct_824;
    let mut puVar11: *mut u16;
    u8 unaff_CS;
    let mut unaff_SS: u16;
    let mut bVar12: bool;
pub fn *pvVar13;
    let mut paVar14: *mut astruct_825;
    let mut puVar3: *mut u16;
    u8 uVar13;
    u8 uVar14;
    let mut iVar12: i16;
    let mut temp_5fa27366cb: *mut astruct_824;

    loop {
        uVar7 =  ( param_1 *  param_3);
        uVar8 = param_2 * param_3 +  ( param_1 *  param_3 >> 0x10);
        if ((uVar8 | uVar7) != 0x0) {
            piVar9 = NULL;
            if ((uVar8 < 0x3) && ((uVar8 < 0x2 || (uVar7 == 0x0)))) {
                if (uVar8 == 0x0) {
                    uVar7 = uVar7 + 0xfff & 0xf000;
                    if (uVar7 == 0x0) {
                        uVar8 = 0x1;
                    }
                } else if ((param_3 - 0x1 & param_3) != 0x0) {
                    piVar9 =  (( uVar8 << 0x10) %  param_3);
                    bVar12 = CARRY2(uVar7,
                                     piVar9);
                    uVar7 +=  piVar9;
                    if (bVar12) {
                    // TODO: goto LAB_1000_41aa;
                    }
                    uVar8 = 0x1;
                }
            } else if ((param_3 - 0x1 & param_3) != 0x0) {
            // TODO: goto LAB_1000_41aa;
            }
            hglobal_7 = GLobalAlloc16(CONCAT13( (uVar8 >> 0x8),
                                               CONCAT12( uVar8,
                                                        uVar7)),
                                      0x20);
            if ((hglobal_7 != 0x0) && ((uVar7 & 0x1) != 0x0)) {
                pvVar13 = WIN16_GlobalLock16(hglobal_7);
                SVar8 =  pvVar13;
                if (( SVar8 != 0x0) || (pvVar13 == NULL)) {
                    paVar14 =  CONCAT22(hglobal_7,
                                                       0x12);
                    uVar13 = '\x12';
                    uVar14 = '\0';
                    pass1_1000_25a8();
                    pass1_1000_2913(CONCAT11(uVar14,
                                             uVar13));
                    pcVar5 = poss_str_op_1000_28dc(paVar14);
                    if (pcVar5 == NULL) {
                    // TODO: goto LAB_1000_28cb;
                    }
                    iVar6 = 0x9;
                    if (*pcVar5 == 'M') {
                        iVar6 = 0xf;
                    }
                    pcVar5 = pcVar5 + iVar6;
                    iVar6 = 0x22;
                    pcVar10 = pcVar5;
                    break;
                }
                hglobal_7 = pass1_1000_422a( ( pvVar13 >> 0x10),
                                            hglobal_7);
                if (hglobal_7 == 0x0) {
                    GlobalUnlock16(uVar8);
                    GlobalFree16( hglobal_di);
                    hglobal_7 = 0x0;
                }
            }
            unaff_CS = 0x38;
            if (hglobal_7 != 0x0) {
                puVar11 = NULL;
                for (; unaff_SI != 0x0; unaff_SI += -0x1) {
                    for (iVar6 = -0x8000; iVar6 != 0x0; iVar6 += -0x1) {
                        puVar3 = puVar11;
                        puVar11 = puVar11 + 0x1;
                        *puVar3 = 0x0;
                    }
                    hglobal_7 += 0x100;
                }
                if (hglobal_di != NULL) {
                    for (; hglobal_di != NULL; hglobal_di = hglobal_di + -0x1) {
                        puVar4 = puVar11;
                        puVar11 =  ( puVar11 + 0x1);
                        * puVar4 = 0x0;
                    }
                }
                return piVar9;
            }
        }//
//        LAB_1000_41aa:
        if ((u16_1050_618e |  PTR_LOOP_1050_618c) == 0x0) {
            return NULL;
        }
        iVar8 = ( PTR_LOOP_1050_618c)(unaff_CS,
                                            param_3,
                                            param_1,
                                            param_2);
        if (iVar8 == 0x0) {
            return NULL;
        }
    } loop;
    loop {
        iVar6 += -0x1;
        pcVar2 = pcVar10;
        pcVar10 = pcVar10 + 0x1;
        if (*pcVar2 == '\r') {
            break;
        }
        if (iVar6 == 0x0) {
            break;
        }
    }
    pcVar10[-0x1] = '\0';//
//    LAB_1000_28cb:
    FatalAppExit16( CONCAT13(0x10,
                                     CONCAT12(0x50,
                                              pcVar5)),
                   0x0);
    FatalExit();
    ppaVar8 = (astruct_824 **) &PTR_LOOP_1050_63fe;
    loop {
        ppaVar1 = ppaVar8;
        ppaVar8 = ppaVar8 + 0x1;
        temp_5fa27366cb = *ppaVar1;
        ppaVar7 = ppaVar8;
        if ((temp_5fa27366cb == hglobal_di) || (ppaVar7 = (astruct_824 **) (temp_5fa27366cb + 0x1), ppaVar7 == NULL)) {
            return  ppaVar7;
        }
        iVar6 = -0x1;
        loop {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            ppaVar1 = ppaVar8;
            ppaVar8 = (astruct_824 **) ( ppaVar8 + 0x1);
        } while (ppaVar1 != NULL);
    } loop;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1000_41e0(mut param_1: i16)
{
    let mut piStack6: *mut i16;

    piStack6 =  CONCAT22(PTR_LOOP_1050_6192,
                                PTR_LOOP_1050_6190);
    loop {
        if (PTR_LOOP_1050_6190 + ( PTR_LOOP_1050_6194 & 0xfffc) <=  piStack6) {
            return 0x0;
        }
        if (*piStack6 == param_1) {
            break;
        }
        piStack6 =  ( piStack6 & 0xffff0000 | ZEXT24( piStack6 + 0x4));
    }
    *piStack6 = 0x0;
    return ( piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1000_422a(mut param_1: i16,
                    mut param_2: u16 )
{
    let mut puVar1: *mut u8;
    let mut puVar2: *mut u8;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut piStack6: *mut i16;

    piStack6 =  CONCAT22(PTR_LOOP_1050_6192,
                                PTR_LOOP_1050_6190);
    loop {
        if (PTR_LOOP_1050_6190 + ( PTR_LOOP_1050_6194 & 0xfffc) <=  piStack6) {
            puVar2 = PTR_LOOP_1050_6194 + 0x28;
            puVar4 = PTR_LOOP_1050_6192;
            puVar3 =  pass1_1000_16aa( PTR_LOOP_1050_6192,
                                             PTR_LOOP_1050_6190,
                                             PTR_LOOP_1050_6192,
                                             puVar2);
            if (( puVar4 |  puVar3) == 0x0) {
                param_1 = 0x0;
            } else {
                puVar1 = puVar3 + ( PTR_LOOP_1050_6194 & 0xfffc);
                piStack6 =  CONCAT22(puVar4,
                                            puVar1);
                PTR_LOOP_1050_6190 = puVar3;
                PTR_LOOP_1050_6192 = puVar4;
                *piStack6 = param_1;
                (puVar1 + 0x2) = param_2;
                PTR_LOOP_1050_6194 = puVar2;
                pass1_1000_4906( CONCAT22(puVar4,
                                                     puVar1 + 0x4),
                                NULL,
                                0x24);
            }
            return param_1;
        }
        if (*piStack6 == 0x0) {
            break;
        }
        piStack6 =  ( piStack6 & 0xffff0000 | ZEXT24( piStack6 + 0x4));
    }
    ( piStack6 + 0x2) = param_2;
    *piStack6 = param_1;
    return param_1;
}



// WARNING: Removing unreachable block (ram,0x10004311)
pub fn dos3_call_set_struct_1000_42de(param_1: *mut astruct_811,
                                    param_2: *mut astruct_810,
                                    param_3: *mut u16)
{
    let mut u_var3: u16;
    code *pcVar4;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut iVar4: *mut astruct_811;
    let mut iVar5: *mut astruct_810;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut b_var5: bool;
    let mut u_var12: u32;
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut u_var9: u16;

    u_var6 =  ( param_1 >> 0x10);
    iVar4 = (astruct_811 *) param_1;
    u_var5 = iVar4.field2_0x2;
    u_var4 = iVar4.field3_0x4;
    u_var1 = iVar4.field6_0x8;
    u_var2 = iVar4.field7_0xa;
    u_var7 =  ( param_3 >> 0x10);
    u_var3 = *param_3;
    u_var9 = ( param_3 + 0x6);
    b_var5 = false;
    pcVar4 =  swi(0x21);
    u_var12 = (*pcVar4)();
    *param_3 = u_var3;
    ( param_3 + 0x6) = u_var9;
    u_var8 =  ( param_2 >> 0x10);
    iVar5 = (astruct_810 *) param_2;
    param_2 = u_var12;
    iVar5.field2_0x2 = u_var5;
    iVar5.field3_0x4 = u_var4;
    iVar5.field4_0x6 =  (u_var12 >> 0x10);
    iVar5.field5_0x8 = u_var1;
    iVar5.field6_0xa = u_var2;
    if (b_var5) {
        block_1000_2000::pass1_1000_29af(u_var12);
    }
    iVar5.field7_0xc = b_var5;
    return;
}



// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)
pub fn dos3_call_op_1000_435c(mut param_1: u16 ,
                            param_2: *mut u16,
                            mut param_3: u16 ,
                            mut param_4: u16 ,
                            mut param_5: u16 ,
                            mut param_6: u16 )
{
    code *pcVar1;
    let mut u_var2: u16;
    let mut in_cx: u16;
    let mut u_var3: u16;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u16;
    let mut cVar7: u8;
    let mut u_var5: u16;
    let mut in_stack_00000002: u16;

    pcVar1 =  swi(0x21);
    (*pcVar1)( &DAT_1050_1050);
    pcVar1 =  swi(0x21);
    u_var3 = in_cx;
    u_var2 = extraout_dx;
    (*pcVar1)();
    u_var6 = extraout_dx_00 >> 0x8;
    cVar7 =  u_var3;
    pcVar1 =  swi(0x21);
    (*pcVar1)(u_var3 >> 0x8);
    u_var4 = extraout_dx_01;
    if ((u_var2 != extraout_dx_01) && (u_var4 = extraout_dx_01, cVar7 == '\x17')) {
        u_var3 = in_cx;
        u_var4 = u_var2;
    }
    u_var2 = pass1_1000_462e(u_var4,
                             u_var3 - 0x7bc,
                             u_var4 >> 0x8,
                             u_var4 & 0xff,
                             u_var6,
                             param_1,
                             param_2);
    if (param_2 != 0x0) {
        ( param_2 + 0x2) = u_var4;
        *param_2 = u_var2;
    }
    return;
}
pub fn pass1_1000_43f0(u16_t param_1)
{
    if (PTR_LOOP_1050_68b4 == NULL) {
        pass1_1000_440c(param_1);
        PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 0x1;
    }
    return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1000_440c(mut param_1: u16 )
{
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut u_var3: u16;
    let mut i_var4: i16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut pcStack8: *mut c_char;

    u_var3 = pass1_1000_3ec0(0x61ca,
                             &DAT_1050_1050);
    pcStack8 =  CONCAT22(param_1,
                                 u_var3);
    if (((param_1 | u_var3) != 0x0) && (_DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0,
                                                                  DAT_1050_61ce), *pcStack8 != '\0')) {
        str_op_1000_3dbe( CONCAT22(PTR_DAT_1050_1050_1050_61de,
                                           PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc),
                          CONCAT22(param_1,
                                           u_var3),
                         0x3);
        pcStack8 =  CONCAT22(param_1,
                                     u_var3 + 0x3);
        cVar1 = *pcStack8;
        if (cVar1 == '-') {
            pcStack8 =  CONCAT22(param_1,
                                         u_var3 + 0x4);
        }
        u_var5 = 0x0;
        u_var9 = 0x0;
        u_var8 = 0xe10;
        u_var3 = pass1_1000_3e2c( pcStack8 & 0xffff |  param_1 << 0x10);
        _DAT_1050_61ce = pass1_1000_52be(u_var3,
                                         u_var5,
                                         u_var8,
                                         u_var9);
        for (; (pcVar2 = pcStack8, *pcStack8 == '+' || (('/' < *pcStack8 && (*pcStack8 < ':'))));
               pcStack8 =  ( pcStack8 & 0xffff0000 |  ( pcStack8 + 0x1))) {
        }
        if (*pcStack8 == ':') {
            u_var5 = 0x0;
            u_var9 = 0x0;
            u_var8 = 0x3c;
            pcStack8 =  ( pcStack8 & 0xffff0000 |  ( pcStack8 + 0x1));
            u_var3 = pass1_1000_3e2c( pcVar2 & 0xffff0000 |  ( pcStack8 + 0x1));
            u_var7 = pass1_1000_52be(u_var3,
                                     u_var5,
                                     u_var8,
                                     u_var9);
            u_var6 =  (u_var7 >> 0x10);
            _DAT_1050_61ce = u_var7 + _DAT_1050_61ce;
            for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
                   pcStack8 =  ( pcStack8 & 0xffff0000 |  ( pcStack8 + 0x1))) {
            }
            if (*pcStack8 == ':') {
                pcStack8 =  ( pcStack8 & 0xffff0000 |  ( pcStack8 + 0x1));
                i_var4 = pass1_1000_3e2c( pcVar2 & 0xffff0000 |  ( pcStack8 + 0x1));
                _DAT_1050_61ce += CONCAT22(u_var6,
                                           i_var4);
                for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
                       pcStack8 =  ( pcStack8 & 0xffff0000 |  ( pcStack8 + 0x1))) {
                }
            }
        }
        PTR_LOOP_1050_61d0 =  (_DAT_1050_61ce >> 0x10);
        if (cVar1 == '-') {
            _DAT_1050_61ce = CONCAT22(- (PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0x0)),
                                      -DAT_1050_61ce);
        }
        DAT_1050_61d2 =  *pcStack8;
        if (DAT_1050_61d2 == 0x0) {
            *_PTR_PTR_1050_61e0 = '\0';
        } else {
            str_op_1000_3dbe(_PTR_PTR_1050_61e0,
                             pcStack8,
                             0x3);
        }
    }
    PTR_LOOP_1050_61d0 =  (_DAT_1050_61ce >> 0x10);
    return;
}

u16 pass1_1000_455a(mut param_1: u16 ,
                    mut param_2: u16 )
{
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut UVar5: u16;
    let mut uVar6: u32;
    let mut iStack6: i16;

    if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8))) {
    // TODO: goto LAB_1000_4623;
    }
    if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
        uVar3 = (param_1 + 0xa);
        if (( uVar3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
            iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
        } else {
            iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
        }
        if ((uVar3 & 0x3) == 0x0) {
            iStack6 += 0x1;
        }
        uVar3 = (uVar3 - 0x46) * 0x16d + ( (uVar3 - 0x1) >> 0x2) + iStack6;
        uVar6 = pass1_1000_52f0(uVar3 - 0xd,
                                ( uVar3 >> 0xf) -  (uVar3 < 0xd),
                                0x7,
                                0x0);
        iStack6 =  uVar6 - iStack6;
        iVar4 = -iStack6;
        if ((param_1 + 0x8) == 0x3) {
            iVar2 = (param_1 + 0xe);
            if ((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < (param_1 + 0x4))))) {
            // TODO: goto LAB_1000_460e;
            }
        } else {
            piVar1 =  (param_1 + 0xe);
            iVar2 = *piVar1;
            if ((SBORROW2(*piVar1,
                          iVar4) != iVar2 + iStack6 < 0x0) || ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1)))) {
            // TODO: goto LAB_1000_460e;
            }
        }//
//        LAB_1000_4623:
        UVar5 = 0x0;
    } else {//
//        LAB_1000_460e:
        UVar5 = 0x1;
    }
    return UVar5;
}

i16 pass1_1000_462e(u16_t param_1,
                    mut param_2: u16 ,
                    mut param_3: i16,
                    mut param_4: u16 ,
                    mut param_5: u16 ,
                    mut param_6: u16 ,
                    mut param_7: i16)
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut UVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut unaff_BP: i16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut iStack26: i16;
    u8 local_16[0x4];
    let mut uStack18: u16;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut iStack8: i16;
    let mut local_4: u16;
    let mut iStack2: i16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;

    iStack2 = unaff_BP + 0x1;
    local_4 =  &DAT_1050_1050;
    uVar7 = (param_3 * 0x2 + 0x61ae);
    if (((param_2 & 0x3) == 0x0) && (0x2 < param_3)) {
        uVar7 += 0x1;
    }
    pass1_1000_43f0(param_1);
    uVar13 = 0x0;
    uVar12 = 0x3c;
    uVar11 = 0x0;
    uVar10 = 0x3c;
    uVar1 =  ((long)  param_2 * 0x16d);
    uVar2 =  (param_2 + 0x3) >> 0x2;
    uVar3 = uVar2 + param_4;
    uVar5 = uVar1 + uVar3;
    uVar6 = uVar5 + uVar7;
    uVar8 = pass1_1000_52be(uVar6 + 0xe44,
                             ( ((long)  param_2 * 0x16d) >> 0x10) + ( (param_2 + 0x3) >> 0xf)
                                + ( param_4 >> 0xf) +  CARRY2(uVar2,
                                                                        param_4) +  CARRY2(uVar1,
                                                                                                uVar3)
                                + ( uVar7 >> 0xf) +  CARRY2(uVar5,
                                                                      uVar7) +  (0xf1bb < uVar6),
                            0x18,
                            0x0);
    uVar8 = pass1_1000_52be( (uVar8 + (long)  param_5),
                             (uVar8 + (long)  param_5 >> 0x10),
                            uVar10,
                            uVar11);
    uVar8 = pass1_1000_52be( (uVar8 + (long)  param_6),
                             (uVar8 + (long)  param_6 >> 0x10),
                            uVar12,
                            uVar13);
    iStack26 =  (uVar8 + (long) param_7 + CONCAT22(PTR_LOOP_1050_61d0,
                                                        DAT_1050_61ce));
    iStack8 = param_4 + uVar7;
    iStack12 = param_2 + 0x50;
    iStack14 = param_3 + -0x1;
    uStack18 = param_5;
    if (DAT_1050_61d2 != 0x0) {
        UVar4 = pass1_1000_455a( local_16,
                                 &DAT_1050_1050);
        if (UVar4 != 0x0) {
            iStack26 += -0xe10;
        }
    }
    return iStack26;
}

char *pass1_1000_472c(mut param_1: u32,
                      char param_2)
{
    let mut pcVar1: *mut c_char;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut pcVar4: *mut c_char;
    let mut uVar5: u16;
    let mut bVar6: bool;

    uVar5 =  (param_1 >> 0x10);
    pcVar3 =  param_1;
    bVar6 = true;
    uVar2 = 0xffff;
    pcVar4 = pcVar3;
    loop {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar4;
        pcVar4 = pcVar4 + 0x1;
        bVar6 = *pcVar1 == '\0';
    } while (!bVar6);
    uVar2 = ~uVar2;
    loop {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar3;
        pcVar3 = pcVar3 + 0x1;
        bVar6 = param_2 == *pcVar1;
    } while (!bVar6);
    if (!bVar6) {
        if (param_2 != '\0') {
            return NULL;
        }
        pcVar3 = pcVar3 + 0x1;
    }
    return pcVar3 + -0x1;
}

i16 pass1_1000_475e(mut param_1: u32,
                    mut param_2: u32)
{
    let mut pcVar1: *mut c_char;
    let mut cVar2: u8;
    let mut cVar3: u8;
    u8 bVar4;
    let mut bVar3: *mut astruct_235;
    let mut bVar5: i16;
    let mut pcVar5: *mut c_char;
    let mut pcVar6: *mut c_char;

    pcVar6 =  param_2;
    pcVar5 =  param_1;
    bVar5 = 0xff;
    loop {
        loop {
            cVar3 =  bVar5;
            if (cVar3 == '\0') {
            // TODO: goto LAB_1000_479d;
            }
            pcVar1 = pcVar6;
            pcVar6 = pcVar6 + 0x1;
            cVar3 = *pcVar1;
            cVar2 = *pcVar5;
            bVar5 = CONCAT11(cVar2,
                             cVar3);
            pcVar5 = pcVar5 + 0x1;
        } while (cVar2 == cVar3);
        bVar4 = cVar3 + 0xbfU + (-( (cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
        bVar3._0_1_ = cVar2 + 0xbf;
        bVar5._0_1_ =  bVar3 + (-( bVar3 < 0x1a) & 0x20U) + 0x41;
        bVar5 = CONCAT11(bVar4,
                          bVar5);
    } while ( bVar5 == bVar4);
    cVar3 = ( bVar5 < bVar4) * -0x2 + '\x01';//
//    LAB_1000_479d:
    return  cVar3;
}

u16 pass1_1000_47a4(mut param_1: u32,
                    mut param_2: u32)
{
    let mut pbVar1: *mut u8;
    u8 bVar2;
    let mut puVar3: *mut u16;
    let mut pbVar4: *mut u8;
    let mut iVar5: i16;
    let mut pbVar6: *mut u8;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    u16 local_22[0x10];

    puVar7 = local_22;
    for (iVar5 = 0x10; iVar5 != 0x0; iVar5 += -0x1) {
        puVar3 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar3 = 0x0;
    }
    pbVar6 =  param_2;
    loop {
        pbVar1 = pbVar6;
        pbVar6 = pbVar6 + 0x1;
        bVar2 = *pbVar1;
        if (bVar2 == 0x0) {
            break;
        }
        pbVar1 =  ( local_22 +  (bVar2 >> 0x3));
        *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
    }
    pbVar1 =  param_1;
    if (param_1 == 0x0) {
        pbVar1 = pbRam105061e4;
    }
    loop {
        pbRam105061e4 = pbVar1;
        uVar8 =  ( pbRam105061e4 >> 0x10);
        pbVar6 =  ( pbRam105061e4 + 0x1);
        bVar2 = *pbRam105061e4;
        if (bVar2 == 0x0) {
            return 0x0;
        }
        pbVar1 =  ( pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
    } while (('\x01' << (bVar2 & 0x7) & * ( local_22 +  (bVar2 >> 0x3))) != 0x0);
    loop {
        pbVar4 = pbVar6;
        bVar2 = *pbVar4;
        if (bVar2 == 0x0) {
        // TODO: goto LAB_1000_483c;
        }
        pbVar6 = pbVar4 + 0x1;
    } while (('\x01' << (bVar2 & 0x7) & * ( local_22 +  (bVar2 >> 0x3))) == 0x0);
    *pbVar4 = 0x0;
    pbVar4 = pbVar4 + 0x1;//
//    LAB_1000_483c:
    pbRam105061e4 =  ( pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
    return  pbRam105061e4;
}

u16 pass1_1000_484c(mut param_1: u32,
                    mut param_2: u32,
                    mut param_3: u16 )
{
    let mut pbVar1: *mut u8;
    let mut pbVar2: *mut u8;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut pbVar6: *mut u8;
    let mut pbVar7: *mut u8;
    let mut iVar8: i16;
    let mut bVar9: bool;
    let mut bVar10: bool;

    if (param_3 == 0x0) {
        return 0x0;
    }
    loop {
        iVar8 =  (param_2 >> 0x10);
        pbVar7 =  param_2;
        iVar3 =  (param_1 >> 0x10);
        pbVar6 =  param_1;
        uVar4 = ~ pbVar7;
        uVar4 = ((param_3 - 0x1) - uVar4 & - (param_3 - 0x1 < uVar4)) + uVar4;
        uVar5 = ~ pbVar6;
        uVar4 = (uVar4 - uVar5 & - (uVar4 < uVar5)) + uVar5 + 0x1;
        bVar9 = param_3 < uVar4;
        param_3 -= uVar4;
        bVar10 = param_3 == 0x0;
        loop {
            if (uVar4 == 0x0) {
                break;
            }
            uVar4 -= 0x1;
            pbVar2 = pbVar7;
            pbVar7 = pbVar7 + 0x1;
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 0x1;
            bVar9 = *pbVar1 < *pbVar2;
            bVar10 = *pbVar1 == *pbVar2;
        } while (bVar10);
        param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
        if (!bVar10) {
            return (0x1 -  bVar9) -  (bVar9 != 0x0);
        }
        if (param_3 == 0x0) {
            return uVar4;
        }
        if (pbVar6 == NULL) {
            iVar3 += 0x6c;
        }
        param_1 = CONCAT22(iVar3,
                           pbVar6);
        if (pbVar7 == NULL) {
            param_2 =  (iVar8 + 0x6c) << 0x10;
            param_1 = CONCAT22(iVar3,
                               pbVar6);
        }
    } loop;
}

u16 pass1_1000_48a8(mut param_1: u32,
                    mut param_2: u32,
                    mut param_3: i16)
{
    let mut puVar1: *mut u16;
    let mut puVar2: *mut u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut puVar7: *mut u16;
    let mut iVar8: i16;

    if (param_3 != 0x0) {
        loop {
            iVar3 =  (param_2 >> 0x10);
            puVar6 =  param_2;
            iVar8 =  (param_1 >> 0x10);
            puVar7 =  param_1;
            uVar4 = ~ puVar7;
            uVar4 = ((param_3 - 0x1) - uVar4 & - (param_3 - 0x1 < uVar4)) + uVar4;
            uVar5 = ~ puVar6;
            uVar4 = (uVar4 - uVar5 & - (uVar4 < uVar5)) + uVar5 + 0x1;
            param_3 -= uVar4;
            for (uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                puVar2 = puVar7;
                puVar7 = puVar7 + 0x1;
                puVar1 = puVar6;
                puVar6 = puVar6 + 0x1;
                *puVar2 = *puVar1;
            }
            for (uVar4 =  ((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 -= 0x1) {
                puVar2 = puVar7;
                puVar7 =  ( puVar7 + 0x1);
                puVar1 = puVar6;
                puVar6 =  ( puVar6 + 0x1);
                * puVar2 = * puVar1;
            }
            if (param_3 == 0x0) {
                break;
            }
            if (puVar6 == NULL) {
                iVar3 += 0x6c;
            }
            param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
            param_2 = CONCAT22(iVar3,
                               puVar6);
            if (puVar7 == NULL) {
                param_1 =  (iVar8 + 0x6c) << 0x10;
                param_2 = CONCAT22(iVar3,
                                   puVar6);
            }
        }
    }
    return  param_1;
}

u16 *pass1_1000_4906(StructD *param_1,
                     WNDCLASS16 *in_wnd_class,
                     mut param_3: u16 )
{
    let mut puVar1: *mut u16;
    u8 uVar2;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut struct_1: *mut astruct_20;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut struct_1_hi: *mut astruct_20;

    if (param_3 != 0x0) {
        struct_1_hi = (astruct_20 *) ( param_1 >> 0x10);
        struct_1 = (astruct_20 *) -  param_1;
        uVar5 = param_3;
        if (struct_1 != NULL) {
            uVar5 = ( struct_1 - param_3 & - (struct_1 < param_3)) + param_3;
            struct_1 = (astruct_20 *) (param_3 - uVar5);
        }
        uVar3 =  in_wnd_class & 0xff |  in_wnd_class << 0x8;
        puVar6 =  param_1;
        for (uVar4 = uVar5 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = puVar6 + 0x1;
            *puVar1 = uVar3;
        }
        for (uVar5 =  ((uVar5 & 0x1) != 0x0);
             uVar2 =  ( in_wnd_class & 0xff), uVar5 != 0x0;
             uVar5 -= 0x1) {
            puVar1 = puVar6;
            puVar6 =  ( puVar6 + 0x1);
            * puVar1 = uVar2;
        }
        if (struct_1 != NULL) {
            for (uVar5 =  struct_1 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = puVar6 + 0x1;
                *puVar1 = uVar3;
            }
            for (uVar5 =  (( struct_1 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 =  ( puVar6 + 0x1);
                * puVar1 = uVar2;
            }
        }
    }
    return  param_1;
}

i16 pass1_1000_49b2(mut param_1: u16 )
{
    return (param_1 ^  param_1 >> 0xf) - ( param_1 >> 0xf);
}

u16 pass1_1000_49c6(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ,
                    mut param_5: u16 ,
                    mut param_6: u16 ,
                    code5 fn_ptr_param_7)
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u32;
    let mut uStack20: u16;
    let mut uStack18: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;

    uStack20 = param_3;
    uStack18 = param_4;
    uVar7 = pass1_1000_52be(param_5 - 0x1,
                            - (param_5 == 0x0),
                            param_6,
                            0x0);
    uStack8 =  (uVar7 + 0x8);
    uStack6 =  (uVar7 + 0x8 >> 0x10) * 0x100 + param_4;
    loop {
        if (uStack6 < uStack18) {
            return 0x0;
        }
        if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
            return 0x0;
        }
        uVar1 = param_5 >> 0x1;
        if (uVar1 == 0x0) {
            if ((param_5 != 0x0) && (iVar5 = ((code5) fn_ptr_param_7)(), iVar5 == 0x0)) {
                return uStack20;
            }
            return 0x0;
        }
        uVar2 = uVar1;
        if ((param_5 & 0x1) == 0x0) {
            uVar2 = uVar1 - 0x1;
        }
        uVar3 =  ( uVar2 *  param_6);
        uVar4 = uVar3 + uStack20;
        iVar6 = ( ( uVar2 *  param_6 >> 0x10) +  CARRY2(uVar3,
                                                                            uStack20)) * 0x100 + uStack18;
        iVar5 = fn_ptr_param_7();
        if (iVar5 == 0x0) {
            break;
        }
        if (iVar5 < 0x0) {
            uStack8 = -param_6 + uVar4;
            uStack6 = ( CARRY2(-param_6,
                                    uVar4) -  (param_6 != 0x0)) * 0x100 + iVar6;
            uVar2 = param_5 & 0x1;
            param_5 = uVar1;
            if (uVar2 == 0x0) {
                param_5 = uVar1 - 0x1;
            }
        } else {
            uStack20 = param_6 + uVar4;
            uStack18 =  CARRY2(param_6,
                                    uVar4) * 0x100 + iVar6;
            param_5 = uVar1;
        }
    }
    return uVar4;
}
pub fn pass1_1000_4aea(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: u16 ,
                     code5 fn_ptr_param_5)
{
    let mut pu_var1: *mut u16;
    code **ppcVar2;
    let mut lVar3: i32;
    let mut u_var4: u16;
    let mut i_var5: i16;
    let mut iVar6: i16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut puVar11: *mut astruct_171;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_di: i16;
    let mut u_var11: u16;
    let mut unaff_cs: u16;
    let mut b_var12: bool;
    let mut u_stack_y26: u16;
    let mut u_stack_y24: u16;
    let mut u_stack_y22: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut u_stack_y18: u16;
    let mut u_stack_y16: u16;
    let mut u_stack_y14: u16;

    if ((param_4 != 0x0) && (param_3 != 0x0)) {
        u_stack_y14 = param_1;
        u_var11 = param_2;
        for (iVar6 = param_3 + -0x1; iVar6 != 0x0; iVar6 += -0x1) {
            u_var9 = u_stack_y14 + param_4;
            u_var11 += - CARRY2(u_stack_y14,
                                param_4) & 0x6c;
            u_stack_y18 = u_var9;
            u_stack_y16 = u_var11;
            i_var5 = fn_ptr_param_5();
            if (i_var5 < 0x0) {
                u_var11 = param_3 - 0x1;
                iVar6 = 0x0;
                loop {
                    u_var11 >>= 0x1;
                    iVar6 += -0x1;
                } while (iVar6 != 0x0 && u_var11 != 0x0);
                if (( (  -iVar6 * 0x8 >> 0x10) != 0x0)
                    || (u_var11 = pass1_1000_3bac(), u_var11 <  (  -iVar6 * 0x8))) {
                    exit_1000_25f2(-0x4,
                                   0x4b7b,
                                   REG_CS,
                                   unaff_di);
                    return;
                }
                puVar11 = (astruct_171 *) &stack0xfff6;
                lVar3 =  (param_3 - 0x1) *  param_4;
                u_var11 =  lVar3;
                u_stack_y14 = u_var11 + param_1;
                u_var11 = ( ( lVar3 >> 0x10) +  CARRY2(u_var11,
                                                            param_1)) * 0x100 + param_2;
                u_stack_y16 = param_2;
                u_stack_y18 = param_1;//
//                LAB_1000_4b7d:
                if (puVar11 <= (astruct_171 *) & u_stack_y18) {
                    return;
                }//
//                LAB_1000_4b81:
                if ((u_stack_y16 < u_var11) || ((u_stack_y16 <= u_var11 && (u_stack_y18 < u_stack_y14)))) {
                    u_stack_y22 = u_stack_y14;
                    pu_var1 = &puVar11.field20_0x14;
                    u_var8 = u_stack_y14 + *pu_var1;
                    u_var7 = u_var11 + (- CARRY2(u_stack_y14,
                                                 *pu_var1) & 0x6c);
                    u_var9 = u_stack_y16;
                    u_var10 = u_stack_y18;
                    u_stack_y26 = u_stack_y18;
                    u_stack_y24 = u_stack_y16;
                    u_var13 = u_var11;//
//                    LAB_1000_4bbc:
                    loop {
                        pu_var1 = &puVar11.field20_0x14;
                        b_var12 = CARRY2(u_var10,
                                         *pu_var1);
                        u_var10 += *pu_var1;
                        u_var9 += -b_var12 & 0x6c;
                        u_var4 = u_stack_y22;
                        if ((u_var10 != u_stack_y14) || (u_var9 != u_var11)) {
//                            ppcVar2 = puVar11.field21_0x16;
                            iVar6 = puVar11.field21_0x16();
                            if (iVar6 < 0x1) {
                                if (iVar6 != 0x0) {
                                    u_stack_y26 = u_var10;
                                    u_stack_y24 = u_var9;
                                }
                            // TODO: goto LAB_1000_4bbc;
                            }
                        }
                        loop {
                            u_var14 = u_var13;
                            u_stack_y22 = u_var4;
                            pu_var1 = &puVar11.field20_0x14;
                            b_var12 = u_var8 < *pu_var1;
                            u_var8 -= *pu_var1;
                            u_var7 -= -b_var12 & 0x6c;
//                            ppcVar2 = (code **) &puVar11.field21_0x16;
//                            iVar6 = (**ppcVar2)();
                            iVar6 = puVar11.field21_0x16();
                            if (0x0 < iVar6) {
                                break;
                            }
                            u_var4 = u_var8;
                            u_var13 = u_var7;
                        } while (((iVar6 != 0x0) || (u_var4 = u_stack_y22, u_var13 = u_var14, u_var8 != u_stack_y18))
                            || (u_var7 != u_stack_y16));
                        if ((u_var7 < u_var9) || ((u_var7 <= u_var9 && (u_var8 <= u_var10)))) {
                        // TODO: goto LAB_1000_4c58;
                        }
                        pass1_1000_4ceb(puVar11.field20_0x14);
                        u_stack_y26 = u_var10;
                        u_stack_y24 = u_var9;
                        u_var13 = u_var7;
                        u_stack_y22 = u_var8;
                    } while (true);
                }
            // TODO: goto LAB_1000_4b7d;
            }
            u_stack_y14 = u_var9;
        }
    }
    return;//
//    LAB_1000_4c58:
    pass1_1000_4ceb(puVar11.field20_0x14);
    u_var10 = ((u_var11 - (- (u_stack_y14 < u_stack_y22) & 0x6c)) - u_var14) + (- CARRY2(u_stack_y14 - u_stack_y22,
                                                                                         u_stack_y18) & 0x6c)
        + u_stack_y16;
    u_var9 = - ((u_stack_y14 - u_stack_y22) + u_stack_y18 < u_stack_y26) & 0x6c;
    if ((u_var10 < u_var9) || (u_var10 - u_var9 < u_stack_y24)) {
        u_stack_y14 = u_stack_y26;
        u_var11 = u_stack_y24;
    } else {
        u_stack_y18 = u_stack_y22;
        u_stack_y16 = u_var14;
    }
// TODO: goto LAB_1000_4b81;
}
pub fn pass1_1000_4ceb(mut param_1: u16 )
{
    let mut puVar1: *mut u8;
    let mut pu_var2: *mut u16;
    u8 uVar3;
    let mut u_var4: u16;
    let mut unaff_si: i16;
    let mut unaff_di: i16;
    let mut unaff_es: u16;

    if ((param_1 & 0x1) != 0x0) {
        param_1 -= 0x1;
        puVar1 =  (param_1 + unaff_di);
        uVar3 = *puVar1;
        *puVar1 = * (param_1 + unaff_si);
        * (param_1 + unaff_si) = uVar3;
        if (param_1 == 0x0) {
            return;
        }
    }
    loop {
        param_1 -= 0x2;
        pu_var2 =  (param_1 + unaff_di);
        u_var4 = *pu_var2;
        *pu_var2 = (param_1 + unaff_si);
        (param_1 + unaff_si) = u_var4;
    } while (param_1 != 0x0);
    return;
}
pub fn pass1_1000_4d0c(mut param_1: u16 )
{
    DAT_1050_61e8 = param_1;
    PTR_LOOP_1050_61ea = NULL;
    return;
}

u16 pass1_1000_4d24()
{
    let mut uVar1: u32;

    uVar1 = pass1_1000_52be(DAT_1050_61e8,
                             PTR_LOOP_1050_61ea,
                             s_TPPOPMENU_1050_43fa + 0x3,
                            0x3);
    PTR_LOOP_1050_61ea =  (uVar1 + 0x269ec3 >> 0x10);
    DAT_1050_61e8 =  (uVar1 + 0x269ec3);
    return  PTR_LOOP_1050_61ea & 0x7fff;
}
pub fn str_1000_4d58(char *in_string_1,
                   char *in_string_2,
                   mut param_3: u32,
                   mut param_4: u32,
                   WNDCLASS16 *param_5)
{
    let mut u_var1: u16;
    let mut i_var2: i16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut pcStack18: *mut c_char;
    let mut u_stack12: u16;
    let mut u_stack10: u16;
    let mut u_stack8: u16;
    let mut u_stack6: u16;

    u_stack10 = 0x0;
    u_stack12 = 0x0;
    u_var4 =  ( in_string_1 >> 0x10);
    i_var2 =  in_string_1;
    if ((*in_string_1 == '\0') || ((i_var2 + 0x1) != ':')) {
        if (in_string_2 != NULL) {
            *in_string_2 = '\0';
        }
    } else {
        if (in_string_2 != NULL) {
            *in_string_2 = *in_string_1;
            * ( in_string_2 + 0x1) = * (i_var2 + 0x1);
            * ( in_string_2 + 0x2) = 0x0;
        }
        in_string_1 =  ( in_string_1 & 0xffff0000 |  (i_var2 + 0x2));
    }
    u_stack6 = 0x0;
    u_stack8 = 0x0;
    pcStack18 = in_string_1;
    while (true) {
        u_var5 =  ( pcStack18 >> 0x10);
        u_var3 =  pcStack18;
        if (*pcStack18 == '\0') {
            break;
        }
        if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
            u_stack8 = u_var3 + 0x1;
            u_stack6 = u_var5;
        } else if (*pcStack18 == '.') {
            u_stack12 = u_var3;
            u_stack10 = u_var5;
        }
        pcStack18 =  ( pcStack18 & 0xffff0000 |  (u_var3 + 0x1));
    }
    if ((u_stack6 | u_stack8) == 0x0) {
        if (param_3 != 0x0) {
            * param_3 = 0x0;
        }
    } else {
        if (param_3 != 0x0) {
            u_var1 = u_stack8 -  in_string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe( (param_3 & 0xffff |  param_3 << 0x10),
                             in_string_1,
                             u_var1);
            * ( param_3 + u_var1) = 0x0;
        }
        in_string_1 =  CONCAT22(u_stack6,
                                        u_stack8);
    }
    if (((u_stack10 | u_stack12) != 0x0) && ( in_string_1 <= u_stack12)) {
        if (param_4 != 0x0) {
            u_var1 = u_stack12 -  in_string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe( (param_4 & 0xffff |  param_4 << 0x10),
                              ( in_string_1 & 0xffff |  in_string_1 << 0x10),
                             u_var1);
            * ( param_4 + u_var1) = 0x0;
        }
        if (param_5 == NULL) {
            return;
        }
        u_var1 = u_var3 - u_stack12;
        if (0xff < u_var1) {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe( ( param_5 & 0xffff |  param_5 << 0x10),
                          CONCAT22(u_stack10,
                                           u_stack12),
                         u_var1);
        * ( param_5 + u_var1) = 0x0;
        return;
    }
    if (param_4 != 0x0) {
        u_var1 = u_var3 -  in_string_1;
        if (0xff < u_var1) {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe( (param_4 & 0xffff |  param_4 << 0x10),
                          ( in_string_1 & 0xffff |  in_string_1 << 0x10),
                         u_var1);
        * ( param_4 + u_var1) = 0x0;
    }
    if (param_5 != NULL) {
        * &param_5.style = 0x0;
    }
    return;
}


/*
Unable to decompile 'pass1_1000_4f1a'
Cause:
Low-level Error: Symbol $$undef00000008 extends beyond the end of the address space
*/


// WARNING: Removing unreachable block (ram,0x10004f47)

u16 dos3_call_1000_4f20()
{
    code *pcVar1;
    let mut uVar2: u16;
    let mut unaff_BP: i16;
    let mut bVar2: bool;

    bVar2 = false;
    pcVar1 =  swi(0x21);
    uVar2 = (*pcVar1)( &DAT_1050_1050,
                      unaff_BP + 0x1);
    if (bVar2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f47)

u16 pass1_1000_4f2e()
{
    code *pcVar1;
    let mut uVar2: u16;
    let mut unaff_BP: i16;
    let mut bVar3: bool;

    bVar3 = false;
    pcVar1 =  swi(0x21);
    uVar2 = (*pcVar1)( &DAT_1050_1050,
                      unaff_BP + 0x1);
    if (bVar3) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f6d)

u16 dos3call_1000_4f54(mut param_1: u32)
{
    let mut cVar1: u8;
    code *pcVar2;
    let mut uVar3: u16;
    let mut unaff_BP: i16;
    let mut bVar3: bool;
    let mut uVar5: u32;

    bVar3 = false;
    pcVar2 =  swi(0x21);
    uVar5 = (*pcVar2)( &DAT_1050_1050,
                      unaff_BP + 0x1);
    uVar5 =  (uVar5 >> 0x10);
    uVar5 =  uVar5;
    uVar3 =  uVar5;
    if ((bVar3) && (bVar3 =  uVar5 < 0x10,  uVar5 == 0x10)) {
        loop {
            cVar1 = *uVar5;
            uVar5 = uVar5 + 0x1;
            if (cVar1 == '\0') {
            // TODO: goto LAB_1000_4f90;
            }
        } while ((cVar1 != '?') && (cVar1 != '*'));
        uVar3 = 0x3;//
//        LAB_1000_4f90:
        bVar3 = true;
    }
    if (!bVar3) {
        return 0x0;
    }
    pass1_1000_29b5(uVar3);
    return 0xffff;
}



// WARNING: Removing unreachable block (ram,0x10004fa9)

i16 dos3_call_1000_4f94()
{
    code6 fn_ptr_1 = (code6)swi(0x21);
//    bVar2 = (*pcVar1)(unaff_BP + 0x1);
    i16 bVar2 = fn_ptr_1(unaff_BP + 1);
    return bVar2 + 0x1;
}

// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)

u16 dos3_call_1000_4fbe(param_1: u8)
{
    u8 cVar2;
    let mut uVar3: u16;
    //    i16 unaff_BP;

    code6 fn_ptr_var1 = (code6) swi(0x21);
    (fn_ptr_var1)(unaff_BP + 0x1);
    code4 fn_ptr_var2 = (code4) swi(0x21);
    cVar2 = fn_ptr_var2();
    uVar3 = 0xffff;
    if (cVar2 + '\x01' == param_1) {
        uVar3 = 0x0;
    }
    return uVar3;
}
