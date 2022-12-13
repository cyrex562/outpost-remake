
pub unsafe fn pass1_1000_5008(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    pass1_1000_5026(0x0,
                    param_1,
                    param_2,
                    param_3);
}



// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1000_5026(mut param_1: i16,
                     mut param_2: u16 ,
                     mut param_3: u16 ,
                     mut param_4: u16 )
{
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut unaff_bp: i16;
    let mut u_stack304: u32;
    // u16 local_12c[0x3];
    let mut local_12c: [u16;3] = [0;3];
    let mut u_stack294: u16;
    // local_124: *mut u8[0x6];
    let mut local_124: [*mut u8;6] = [null_mut();6];
    let mut i_stack280: i16;
    let mut local_116: u8;
    let mut uStack277: u8;
    let mut cStack272: u8;
    let mut puStack270: *mut u8;
    let mut local_108: u8;
    let mut uStack263: u8;
    let mut uStack262: u8;
    //  261[0x101];
    let mut auStack261: [u8;0x101] = [0;0x101];
    let mut local_4: u16;
    let mut i_stack2: i16;

    i_stack2 = unaff_bp + 1;
    local_4 = SUB42(&DAT_1050_1050,
                    0x0);
    u_stack304 =  CONCAT22(0x1050,
                                   &local_108);
    if (param_1 == 0) {
        param_1 = dos3_call_1000_4f94();
    }
    *u_stack304 =  param_1 + '@';
    uStack263 = 0x3a;
    puStack270 = auStack261;
    uStack262 = 0x5c;
    uStack277 = 0x47;
    cStack272 =  param_1;
    local_12c[0] = SUB42(&DAT_1050_1050,
                         0x0);
    u_stack294 = SUB42(&DAT_1050_1050,
                       0x0);
    dos3_call_set_struct_1000_42de( CONCAT22(0x1050,
                                                            &local_116),
                                    CONCAT22(0x1050,
                                                            local_124),
                                    CONCAT22(0x1050,
                                                    local_12c));
    if (i_stack280 == 0) {
        u_var1 = str_op_1000_3da4( CONCAT22(0x1050,
                                                    &local_108));
        u_var1 += 0x1;
        u_stack304 = param_2;
        u_stack304 = param_3;
        u_var2 = param_3 | param_2;
        if (u_var2 == 0) {
            if ( param_4 < u_var1) {
                param_4 = u_var1;
            }
            u_stack304 = mem_1000_167a(0x0,
                                                              param_4);
            u_stack304 = u_var2;
            if ((u_var2 | u_stack304) == 0) {
                PTR_LOOP_1050_5f78 =  &PTR_LOOP_1050_000c;
                return;
            }
        }
        if ( param_4 < u_var1) {
            PTR_LOOP_1050_5f78 =  ( s_New_failed_in_Op__Op_1050_0020 + 2);
        } else {
            unk_str_op_1000_3d3e( CONCAT22(u_stack304,
                                                   u_stack304),
                                  CONCAT22(0x1050,
                                                   &local_108));
        }
    } else {
        PTR_LOOP_1050_5f78 =  ( &PTR_LOOP_1050_000c + 1);
        PTR_LOOP_1050_5f88 = local_124[0];
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10005167)

pub unsafe fn dos3_call_1000_514e() -> u16
{
    // let mut pc_var1: *mut code;
    // let mut u_var2: u16;
    let mut unaff_bp: i16;
    // let mut b_var2: bool;

    let mut b_var2 = false;
    let pc_var1 =  swi(0x21);
    let u_var2 = (*pc_var1)( &DAT_1050_1050,
                      unaff_bp + 1);
    if (b_var2) {
        pass1_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000518c)

pub unsafe fn dos3_call_1000_5174() -> u16
{
    // let mut pc_var1: *mut code;
    // let mut u_var2: u16;
    let mut unaff_bp: i16;
    // let mut b_var2: bool;

    let mut b_var2 = false;
    let mut pc_var1 =  swi(0x21);
    let u_var2 = (*pc_var1)(unaff_bp + 1);
    if (!b_var2) {
        return 0x0;
    }
    pass1_1000_29b5(u_var2);
    return u_var2 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

pub unsafe fn dos3_calls_1000_5198(mut param_1: u16 ,
                         mut param_2: u16 ,
                         mut param_3: u16 ,
                         mut param_4: u16 ) -> u16
{
    let mut pcVar1 =  swi(0x21);
    (*pcVar1)( &DAT_1050_1050);
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0) {
        return 0x0;
    }
    pass1_1000_29b5(param_3);
    return param_3 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

pub unsafe fn dos3_call_1000_51aa(mut param_1: u16 ,
                        mut param_2: u16 ,
                        mut param_3: u16 ) -> u16
{
    // let mut pc_var1: *mut code;
    // let mut u_stack000a: u16;

    let mut pc_var1 =  swi(0x21);
    (*pc_var1)( &DAT_1050_1050);
    pc_var1 =  swi(0x21);
    (*pc_var1)();
    pc_var1 =  swi(0x21);
    (*pc_var1)();
    pc_var1 =  swi(0x21);
    (*pc_var1)();
    if ((param_2 & 0x100) == 0) {
        return 0x0;
    }
    let mut u_stack000a = param_3;
    pass1_1000_29b5(param_3);
    return u_stack000a & 0xff;
}

pub unsafe fn pass1_1000_5224(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    let mut cVar11: u8;
    let mut uVar9: u16;

    cVar11 =  param_2 < 0x0;
    if ( cVar11) {
        bVar10 = param_1 != 0;
        param_1 = -param_1;
        param_2 = - bVar10 - param_2;
    }
    if ( param_4 < 0x0) {
        cVar11 += '\x01';
        bVar10 = param_3 != 0;
        param_3 = -param_3;
        param_4 = - bVar10 - param_4;
    }
    uVar3 = param_1;
    uVar5 = param_3;
    uVar6 = param_2;
    uVar9 = param_4;
    if (param_4 == 0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (( param_2 %  param_3 << 0x10 |  param_1) /  param_3);
    } else {
        loop {
            uVar8 = uVar9 >> 0x1;
            uVar5 = uVar5 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 1) != 0) << 0xf;
            uVar6 = uVar7;
            uVar9 = uVar8;
            if uVar9 == 0 {
                break;
            }
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar5;
        iVar4 =  uVar1;
        lVar2 =  param_3 * (uVar1 & 0xffff);
        uVar3 =  ( lVar2 >> 0x10);
        uVar5 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0;
    }
    if (cVar11 == '\x01') {
        bVar10 = iVar4 != 0;
        iVar4 = -iVar4;
        uVar3 = - bVar10 - uVar3;
    }
    return CONCAT22(uVar3,
                    iVar4);
}

pub unsafe fn pass1_1000_52be(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    if ((param_4 | param_2) == 0) {
        return  param_1 *  param_3;
    }
    return  param_1 *  param_3 & 0xffff
        |  ( ( param_1 *  param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}

pub unsafe fn pass1_1000_52f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut bVar12: bool;
    let mut bVar13: bool;

    bVar13 =  param_2 < 0x0;
    if (bVar13) {
        bVar12 = param_1 != 0;
        param_1 = -param_1;
        param_2 = - bVar12 - param_2;
    }
    uVar11 =  bVar13;
    if ( param_4 < 0x0) {
        bVar13 = param_3 != 0;
        param_3 = -param_3;
        param_4 = - bVar13 - param_4;
    }
    uVar3 = param_1;
    uVar4 = param_3;
    uVar8 = param_2;
    uVar9 = param_4;
    if (param_4 == 0) {
        iVar5 =  (( param_2 %  param_3 << 0x10 |  param_1) %  param_3);
        iVar6 = 0;
        if ( (uVar11 - 1) < 0x0) {
        // TODO: goto LAB_1000_538a;
        }
    } else {
        loop {
            uVar10 = uVar9 >> 0x1;
            uVar4 = uVar4 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar7 = uVar8 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar8 & 1) != 0) << 0xf;
            uVar8 = uVar7;
            uVar9 = uVar10;
            if uVar10 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar4;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) *  param_3;
        uVar8 =  ( lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar9 = uVar8 + uVar3;
        if (((CARRY2(uVar8,
                     uVar3)) || (param_2 < uVar9)) || ((param_2 <= uVar9 && (param_1 < uVar4)))) {
            bVar13 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar9 = (uVar9 - param_4) -  bVar13;
        }
        iVar5 = uVar4 - param_1;
        iVar6 = (uVar9 - param_2) -  (uVar4 < param_1);
        if (-0x1 <  (uVar11 - 1)) {
        // TODO: goto LAB_1000_538a;
        }
    }
    bVar13 = iVar5 != 0;
    iVar5 = -iVar5;
    iVar6 = - bVar13 - iVar6;//
//    LAB_1000_538a:
    return CONCAT22(iVar6,
                    iVar5);
}

pub unsafe fn pass1_1000_5390(mut param_1: u32,
                              mut param_3: u16,
                              mut param_4: *mut u8) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;

    uVar3 = param_1;
    uVar8 = param_4;
    uVar6 = param_2;
    uVar9 = param_3;
    if (param_4 == 0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (( param_2 %  param_3 << 0x10 |  param_1) /  param_3);
    } else {
        loop {
            uVar5 = uVar8 >> 0x1;
            uVar9 = uVar9 >> 0x1 |  ((uVar8 & 1) != 0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 1) != 0) << 0xf;
            uVar8 = uVar5;
            uVar6 = uVar7;
            if uVar5 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar9;
        iVar4 =  uVar1;
        lVar2 =  param_3 * (uVar1 & 0xffff);
        uVar3 =  ( lVar2 >> 0x10);
        uVar8 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0;
    }
    return CONCAT22(uVar3,
                    iVar4);
}

pub unsafe fn pass1_1000_53f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;

    uVar3 = param_1;
    uVar4 = param_4;
    uVar9 = param_2;
    uVar10 = param_3;
    if (param_4 == 0) {
        iVar6 =  (( param_2 %  param_3 << 0x10 |  param_1) %  param_3);
        iVar7 = 0;
    } else {
        loop {
            uVar5 = uVar4 >> 0x1;
            uVar10 = uVar10 >> 0x1 |  ((uVar4 & 1) != 0) << 0xf;
            uVar8 = uVar9 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar4 = uVar5;
            uVar9 = uVar8;
            if uVar5 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar8,
                         uVar3) /  uVar10;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) *  param_3;
        uVar9 =  ( lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar10 = uVar9 + uVar3;
        if (((CARRY2(uVar9,
                     uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
            bVar11 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar10 = (uVar10 - param_4) -  bVar11;
        }
        iVar6 = -(uVar4 - param_1);
        iVar7 = - (uVar4 - param_1 != 0) - ((uVar10 - param_2) -  (uVar4 < param_1));
    }
    return CONCAT22(iVar7,
                    iVar6);
}

pub unsafe fn pass1_1000_545a(mut param_1: u32,
                    mut param_2: u32) -> i16
{
    let mut pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut pbVar5: *mut u8;
    let mut pbVar6: *mut u8;

    pbVar6 =  param_2;
    pbVar5 =  param_1;
    bVar4 = 0xff;
    loop {
        loop {
            if (bVar4 == 0) {
            // TODO: goto LAB_1000_5499;
            }
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 1;
            bVar4 = *pbVar1;
            bVar3 = *pbVar5;
            pbVar5 = pbVar5 + 1;
            if bVar3 != bVar4 {break;}
        }
        bVar2 = bVar4 + 0xbf + (-( (bVar4 + 0xbf) < 0x1a) & 0x20) + 0x41;
        bVar3 += 0xbf;
        bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20) + 0x41;
        if bVar4 != bVar2 {break;}
    }
    bVar4 = (bVar4 < bVar2) * -0x2 + 1;//
//    LAB_1000_5499:
    return   bVar4;
}

pub unsafe fn pass1_1000_54a0(mut param_1: u32,
                     mut param_2: u16 ,
                     mut param_3: u16 ) -> *mut u16
{
    let mut pu_var1: *mut u16;
    let mut u_var2: u8;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut pu_var7: *mut u16;
    let mut i_var8: i16;

    if (param_3 != 0) {
        i_var8 =  (param_1 >> 0x10);
        u_var5 = -  param_1;
        u_var6 = param_3;
        if (u_var5 != 0) {
            u_var6 = (u_var5 - param_3 & - (u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = param_2 & 0xff | param_2 << 0x8;
        pu_var7 =  param_1;
        // for (uVar4 = uVar6 >> 0x1; uVar4 != 0; uVar4 -= 1)
        for uVar4 in u_var6 >> 1 .. 0
        {
            pu_var1 = pu_var7;
            pu_var7 = pu_var7 + 1;
            *pu_var1 = u_var3;
        }
        u_var6 = u_var6 & 1 != 0;
        u_var2 = param_2 & 0xff;
        // for (uVar6 =  ((uVar6 & 1) != 0); uVar2 =  (param_2 & 0xff), uVar6 != 0; uVar6 -= 1) {
            while u_var6 !=0 {
            pu_var1 = pu_var7;
            pu_var7 =  ( pu_var7 + 1);
            * pu_var1 = u_var2;
            u_var6 -= 1;
        }
        if (u_var5 != 0) {
            // for (uVar6 = u_var5 >> 0x1; u_var6 != 0; u_var6 -= 1)
            for u_var6 in u_var5 >> 1 .. 0
            {
                pu_var1 = pu_var7;
                pu_var7 = pu_var7 + 1;
                *pu_var1 = u_var3;
            }
            // for (uVar6 =  ((u_var5 & 1) != 0); u_var6 != 0; u_var6 -= 1)
            u_var6 = u_var5 & 1 != 0;

            {
                pu_var1 = pu_var7;
                pu_var7 =  ( pu_var7 + 1);
                * pu_var1 = u_var2;
            }
        }
    }
    return  param_1;
}
pub unsafe fn pass1_1000_54e8(param_1: *mut u8,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: i16,
                     mut param_6: u16 )
{
    let mut i_var1: i16;

    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        ( param_1)();
    }
    return;
}
pub unsafe fn pass1_1000_5512(param_1: *mut u8,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: u16 )
{
    let mut b_var1: bool;
    let mut u_stack4: u16;

    pass1_1000_52be(param_3,
                    param_4,
                    param_5,
                    0x0);
    while (true) {
        b_var1 = param_3 == 0;
        param_3 += -0x1;
        param_4 -= b_var1;
        if (param_4 < 0x0) {
            break;
        }
        u_stack4 = param_2;
        ( param_1)();
    }
}

/*
Unable to decompile 'pass1_1000_55b1'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/
pub unsafe fn pass1_1000_5586(param_1: code,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: i16,
                     mut param_6: u16 )
{
    let mut i_var1: i16;

    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        ( param_1)();
    }
}

pub unsafe fn ret_op_1000_55ac() -> u32
{
    todo!()
}

pub unsafe fn exit_1000_25f2(a: i16, b: i16, c: i16, d: i16) {
    todo!()
}


//dos3_call_1000_23ea( param_4,
//                         &DAT_1050_1050,
//                        0x0,
//                         &DAT_1050_1050);
pub unsafe fn dos3_call_1000_23ea( a: *mut u8, b: u16, c: u16, d: u16) {
    todo!()
}
