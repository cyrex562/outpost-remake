use crate::globals::DAT_1050_1050;
use crate::winbase::{GLobalAlloc16, GlobalFree16};

pub unsafe fn pass1_1000_400a(mut param_1: i16) -> *mut u8 {
    let mut pu_var1: *mut u8;

    if ((param_1 < 0x0) || (PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
        PTR_LOOP_1050_5f78 = (&u16_1050_0008 + 1);
        pu_var1 = 0xffff;
    } else if (((u16_1050_61ec == 0) || (param_1 < u16_1050_5f8a && (0x2 < param_1)))
        && (0x31d < CONCAT11(DAT_1050_5f83, DAT_1050_5f82)))
    {
        pu_var1 = PTR_LOOP_1050_5f88;
        if (((*(param_1 + 0x5f90) & 1) == 0)
            || (pu_var1 = dos3_call_1000_5174(), pu_var1.is_null() == false))
        {
            PTR_LOOP_1050_5f88 = pu_var1;
            PTR_LOOP_1050_5f78 = (&u16_1050_0008 + 1);
            pu_var1 = 0xffff;
        }
    } else {
        pu_var1 = null_mut();
    }
    return pu_var1;
}

pub unsafe fn pass1_1000_422a(mut param_1: i16, mut param_2: u16) -> i16 {
    let mut pu_var1: *mut u8;
    let mut pu_var2: *mut u8;
    let mut pu_var3: *mut u8;
    let mut pu_var4: *mut u8;
    let mut pi_stack6: *mut i16;

    pi_stack6 = CONCAT22(PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    loop {
        if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= pi_stack6) {
            pu_var2 = PTR_LOOP_1050_6194 + 0x28;
            pu_var4 = PTR_LOOP_1050_6192;
            pu_var3 = pass1_1000_16aa(
                PTR_LOOP_1050_6192,
                PTR_LOOP_1050_6190,
                PTR_LOOP_1050_6192,
                pu_var2,
            );
            if ((pu_var4 | pu_var3) == 0) {
                param_1 = 0;
            } else {
                pu_var1 = pu_var3 + (PTR_LOOP_1050_6194 & 0xfffc);
                pi_stack6 = CONCAT22(pu_var4, pu_var1);
                PTR_LOOP_1050_6190 = pu_var3;
                PTR_LOOP_1050_6192 = pu_var4;
                *pi_stack6 = param_1;
                (pu_var1 + 0x2) = param_2;
                PTR_LOOP_1050_6194 = pu_var2;
                pass1_1000_4906(CONCAT22(pu_var4, pu_var1 + 0x4), NULL, 0x24);
            }
            return param_1;
        }
        if (*pi_stack6 == 0) {
            break;
        }
        pi_stack6 = (pi_stack6 & 0xffff0000 | ZEXT24(pi_stack6 + 0x4));
    }
    (pi_stack6 + 0x2) = param_2;
    *pi_stack6 = param_1;
    return param_1;
}

// WARNING: Removing unreachable block (ram,0x10004311)
pub unsafe fn dos3_call_set_struct_1000_42de(
    param_1: *mut astruct_811,
    param_2: *mut astruct_810,
    param_3: *mut u16,
) {
    let mut u_var3: u16;
    let mut pcVar4: *mut code;
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

    u_var6 = (param_1 >> 0x10);
    iVar4 = param_1;
    u_var5 = iVar4.field2_0x2;
    u_var4 = iVar4.field3_0x4;
    u_var1 = iVar4.field6_0x8;
    u_var2 = iVar4.field7_0xa;
    u_var7 = (param_3 >> 0x10);
    u_var3 = *param_3;
    u_var9 = (param_3 + 0x6);
    b_var5 = false;
    pcVar4 = swi(0x21);
    u_var12 = (*pcVar4)();
    *param_3 = u_var3;
    (param_3 + 0x6) = u_var9;
    u_var8 = (param_2 >> 0x10);
    iVar5 = param_2;
    param_2 = u_var12;
    iVar5.field2_0x2 = u_var5;
    iVar5.field3_0x4 = u_var4;
    iVar5.field4_0x6 = (u_var12 >> 0x10);
    iVar5.field5_0x8 = u_var1;
    iVar5.field6_0xa = u_var2;
    if (b_var5) {
       pass1_1000_29af(u_var12);
    }
    iVar5.field7_0xc = b_var5;
    return;
}

// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)
pub unsafe fn dos3_call_op_1000_435c(
    mut param_1: u16,
    param_2: *mut u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut pcVar1: *mut code;
    let mut u_var2: u16;
    let mut in_cx: u16;
    let mut u_var3: u16;
    let mut extraout_dx: u16;
    let mut extraout_dx_00: u16;
    let mut extraout_dx_01: u16;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u16;
    let mut c_var7: u8;
    let mut u_var5: u16;
    let mut in_stack_00000002: u16;

    pcVar1 = swi(0x21);
    (*pcVar1)(&DAT_1050_1050);
    pcVar1 = swi(0x21);
    u_var3 = in_cx;
    u_var2 = extraout_dx;
    (*pcVar1)();
    u_var6 = extraout_dx_00 >> 0x8;
    c_var7 = u_var3;
    pcVar1 = swi(0x21);
    (*pcVar1)(u_var3 >> 0x8);
    u_var4 = extraout_dx_01;
    if ((u_var2 != extraout_dx_01) && (u_var4 = extraout_dx_01, c_var7 == '\x17')) {
        u_var3 = in_cx;
        u_var4 = u_var2;
    }
    u_var2 = pass1_1000_462e(
        u_var4,
        u_var3 - 0x7bc,
        u_var4 >> 0x8,
        u_var4 & 0xff,
        u_var6,
        param_1,
        param_2,
    );
    if (param_2 != 0) {
        (param_2 + 0x2) = u_var4;
        *param_2 = u_var2;
    }
    return;
}

pub unsafe fn pass1_1000_43f0(param_1: u16) {
    if (PTR_LOOP_1050_68b4.is_null()) {
        pass1_1000_440c(param_1);
        PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 1;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1000_440c(mut param_1: u16) {
    let mut c_var1: u8;
    let mut pc_var2: *mut c_char;
    let mut u_var3: u16;
    let mut i_var4: i16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut pc_stack8: *mut c_char;

    u_var3 = pass1_1000_3ec0(0x61ca, &DAT_1050_1050);
    pc_stack8 = CONCAT22(param_1, u_var3);
    if (((param_1 | u_var3) != 0)
        && (
            _DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0, DAT_1050_61ce),
            *pc_stack8 != '\0',
        ))
    {
        str_op_1000_3dbe(
            CONCAT22(
                PTR_DAT_1050_1050_1050_61de,
                PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc,
            ),
            CONCAT22(param_1, u_var3),
            0x3,
        );
        pc_stack8 = CONCAT22(param_1, u_var3 + 0x3);
        c_var1 = *pc_stack8;
        if (c_var1 == '-') {
            pc_stack8 = CONCAT22(param_1, u_var3 + 0x4);
        }
        u_var5 = 0;
        u_var9 = 0;
        u_var8 = 0xe10;
        u_var3 = pass1_1000_3e2c(pc_stack8 & 0xffff | param_1 << 0x10);
        _DAT_1050_61ce = pass1_1000_52be(u_var3, u_var5, u_var8, u_var9);
        // for (; (pc_var2 = pc_stack8, *pc_stack8 == '+' || (('/' < *pc_stack8 && (*pc_stack8 < ':'))));
        //        pc_stack8 =  ( pc_stack8 & 0xffff0000 |  ( pc_stack8 + 1))) {
        // }
        if (*pc_stack8 == ':') {
            u_var5 = 0;
            u_var9 = 0;
            u_var8 = 0x3c;
            pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 1));
            u_var3 = pass1_1000_3e2c(pc_var2 & 0xffff0000 | (pc_stack8 + 1));
            u_var7 = pass1_1000_52be(u_var3, u_var5, u_var8, u_var9);
            u_var6 = (u_var7 >> 0x10);
            _DAT_1050_61ce = u_var7 + _DAT_1050_61ce;
            // for (; (pc_var2 = pc_stack8, '/' < *pc_stack8 && (*pc_stack8 < ':'));
            //        pc_stack8 =  ( pc_stack8 & 0xffff0000 |  ( pc_stack8 + 1))) {
            // }
            if (*pc_stack8 == ':') {
                pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 1));
                i_var4 = pass1_1000_3e2c(pc_var2 & 0xffff0000 | (pc_stack8 + 1));
                _DAT_1050_61ce += CONCAT22(u_var6, i_var4);
                // for (; ('/' < *pc_stack8 && (*pc_stack8 < ':'));
                //        pc_stack8 =  ( pc_stack8 & 0xffff0000 |  ( pc_stack8 + 1))) {
                // }
            }
        }
        PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
        if (c_var1 == '-') {
            _DAT_1050_61ce = CONCAT22(-(PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0)), -DAT_1050_61ce);
        }
        DAT_1050_61d2 = *pc_stack8;
        if (DAT_1050_61d2 == 0) {
            *_PTR_PTR_1050_61e0 = '\0';
        } else {
            str_op_1000_3dbe(_PTR_PTR_1050_61e0, pc_stack8, 0x3);
        }
    }
    PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
    return;
}

pub unsafe fn pass1_1000_455a(mut param_1: u16, mut param_2: u16) -> u16 {
    let mut pi_var1: *mut i16;
    let mut i_var2: i16;
    let mut u_var3: u16;
    let mut i_var4: i16;
    let mut uvar5: u16;
    let mut u_var6: u32;
    let mut i_stack6: i16;

    if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8))) {
        // TODO: goto LAB_1000_4623;
    }
    if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
        u_var3 = (param_1 + 0xa);
        if ((u_var3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
            i_stack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
        } else {
            i_stack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
        }
        if ((u_var3 & 0x3) == 0) {
            i_stack6 += 0x1;
        }
        u_var3 = (u_var3 - 0x46) * 0x16d + ((u_var3 - 1) >> 0x2) + i_stack6;
        u_var6 = pass1_1000_52f0(u_var3 - 0xd, (u_var3 >> 0xf) - (u_var3 < 0xd), 0x7, 0x0);
        i_stack6 = u_var6 - i_stack6;
        i_var4 = -i_stack6;
        if ((param_1 + 0x8) == 0x3) {
            i_var2 = (param_1 + 0xe);
            if ((i_var4 < i_var2) || (-i_var2 == i_stack6 && (0x1 < (param_1 + 0x4)))) {
                // TODO: goto LAB_1000_460e;
            }
        } else {
            pi_var1 = (param_1 + 0xe);
            i_var2 = *pi_var1;
            // if ((SBORROW2(*pi_var1,
            //               i_var4) != i_var2 + i_stack6 < 0x0) || ((i_var2 == i_var4 && ((param_1 + 0x4) < 1)))) {
            // // TODO: goto LAB_1000_460e;
            // }
        } //
          //        LAB_1000_4623:
        uvar5 = 0;
    } else {
        //
        //        LAB_1000_460e:
        uvar5 = 0x1;
    }
    return uvar5;
}

pub unsafe fn pass1_1000_462e(
    param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: i16,
) -> i16 {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut uvar4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_bp: i16;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut i_stack26: i16;
    // let mut local_16: [u8;0x4] = [0;0x4];
    let mut local_16: [u8; 4] = [0; 4];
    let mut u_stack18: u16;
    let mut i_stack14: i16;
    let mut i_stack12: i16;
    let mut i_stack8: i16;
    let mut local_4: u16;
    let mut i_stack2: i16;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;

    i_stack2 = unaff_bp + 1;
    local_4 = &DAT_1050_1050;
    u_var7 = (param_3 * 0x2 + 0x61ae);
    if (((param_2 & 0x3) == 0) && (0x2 < param_3)) {
        u_var7 += 0x1;
    }
    pass1_1000_43f0(param_1);
    u_var13 = 0;
    u_var12 = 0x3c;
    u_var11 = 0;
    u_var10 = 0x3c;
    u_var1 = (param_2 * 0x16d);
    u_var2 = (param_2 + 0x3) >> 0x2;
    u_var3 = u_var2 + param_4;
    u_var5 = u_var1 + u_var3;
    u_var6 = u_var5 + u_var7;
    u_var8 = pass1_1000_52be(
        u_var6 + 0xe44,
        ((param_2 * 0x16d) >> 0x10)
            + ((param_2 + 0x3) >> 0xf)
            + (param_4 >> 0xf)
            + CARRY2(u_var2, param_4)
            + CARRY2(u_var1, u_var3)
            + (u_var7 >> 0xf)
            + CARRY2(u_var5, u_var7)
            + (0xf1bb < u_var6),
        0x18,
        0x0,
    );
    u_var8 = pass1_1000_52be(
        (u_var8 + param_5),
        (u_var8 + param_5 >> 0x10),
        u_var10,
        u_var11,
    );
    u_var8 = pass1_1000_52be(
        (u_var8 + param_6),
        (u_var8 + param_6 >> 0x10),
        u_var12,
        u_var13,
    );
    i_stack26 = (u_var8 + param_7 + CONCAT22(PTR_LOOP_1050_61d0, DAT_1050_61ce));
    i_stack8 = param_4 + u_var7;
    i_stack12 = param_2 + 0x50;
    i_stack14 = param_3 -0x1;
    u_stack18 = param_5;
    if (DAT_1050_61d2 != 0) {
        uvar4 = pass1_1000_455a(local_16, &DAT_1050_1050);
        if (uvar4 != 0) {
            i_stack26 += -0xe10;
        }
    }
    return i_stack26;
}

pub unsafe fn pass1_1000_472c(mut param_1: u32, param_2: u8) -> *mut c_char {
    let mut pc_var1: *mut c_char;
    let mut u_var2: u16;
    let mut pc_var3: *mut c_char;
    let mut pc_var4: *mut c_char;
    let mut u_var5: u16;
    let mut b_var6: bool;

    u_var5 = (param_1 >> 0x10);
    pc_var3 = param_1;
    b_var6 = true;
    u_var2 = 0xffff;
    pc_var4 = pc_var3;
    loop {
        if (u_var2 == 0) {
            break;
        }
        u_var2 -= 1;
        pc_var1 = pc_var4;
        pc_var4 = pc_var4 + 1;
        b_var6 = *pc_var1 == '\0';
        if b_var6 {
            break;
        }
    }
    u_var2 = !u_var2;
    loop {
        if (u_var2 == 0) {
            break;
        }
        u_var2 -= 1;
        pc_var1 = pc_var3;
        pc_var3 = pc_var3 + 1;
        b_var6 = param_2 == *pc_var1;
        if b_var6 {
            break;
        }
    }
    if (!b_var6) {
        if (param_2 != '\0') {
            return NULL;
        }
        pc_var3 = pc_var3 + 1;
    }
    return pc_var3 -0x1;
}

pub unsafe fn pass1_1000_475e(mut param_1: u32, mut param_2: u32) -> i16 {
    let mut pc_var1: *mut c_char;
    let mut c_var2: u8;
    let mut c_var3: u8;
    let mut b_var4: u8;
    let mut b_var3: *mut astruct_235;
    let mut b_var5: i16;
    let mut pc_var5: *mut c_char;
    let mut pc_var6: *mut c_char;

    pc_var6 = param_2;
    pc_var5 = param_1;
    b_var5 = 0xff;
    loop {
        loop {
            c_var3 = b_var5;
            if (c_var3 == '\0') {
                // TODO: goto LAB_1000_479d;
            }
            pc_var1 = pc_var6;
            pc_var6 = pc_var6 + 1;
            c_var3 = *pc_var1;
            c_var2 = *pc_var5;
            b_var5 = CONCAT11(c_var2, c_var3);
            pc_var5 = pc_var5 + 1;
            if c_var2 != c_var3 {
                break;
            }
        }
        b_var4 = c_var3 + 0xbf + (-((c_var3 + 0xbf) < 0x1a) & 0x20) + 0x41;
        b_var3._0_1_ = c_var2 + 0xbf;
        b_var5._0_1_ = b_var3 + (-(b_var3 < 0x1a) & 0x20) + 0x41;
        b_var5 = CONCAT11(b_var4, b_var5);
        if b_var5 != b_var4 {
            break;
        }
    }
    c_var3 = (b_var5 < b_var4) * -0x2 + '\x01'; //
                                                //    LAB_1000_479d:
    return c_var3;
}

pub unsafe fn pass1_1000_47a4(mut param_1: u32, mut param_2: u32) -> u16 {
    let mut pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut pu_var3: *mut u16;
    let mut pb_var4: *mut u8;
    let mut i_var5: i16;
    let mut pb_var6: *mut u8;
    let mut pu_var7: *mut u16;
    let mut u_var8: u16;
    // u16 local_22[0x10];
    let mut local_22: [u16; 0x10] = [0; 0x10];

    pu_var7 = local_22;
    // for (iVar5 = 0x10; i_var5 != 0; i_var5 += -1)
    for iVar5 in 0x10..0 {
        pu_var3 = pu_var7;
        pu_var7 = pu_var7 + 1;
        *pu_var3 = 0;
    }
    pb_var6 = param_2;
    loop {
        pb_var1 = pb_var6;
        pb_var6 = pb_var6 + 1;
        b_var2 = *pb_var1;
        if (b_var2 == 0) {
            break;
        }
        pb_var1 = (local_22 + (b_var2 >> 0x3));
        *pb_var1 = *pb_var1 | '\x01' << (b_var2 & 0x7);
    }
    pb_var1 = param_1;
    if (param_1 == 0) {
        pb_var1 = pbRam105061e4;
    }
    loop {
        pbRam105061e4 = pb_var1;
        u_var8 = (pbRam105061e4 >> 0x10);
        pb_var6 = (pbRam105061e4 + 1);
        b_var2 = *pbRam105061e4;
        if (b_var2 == 0) {
            return 0x0;
        }
        pb_var1 = (pbRam105061e4 & 0xffff0000 | ZEXT24(pb_var6));
        if !(('\x01' << (b_var2 & 0x7) & *(local_22 + (b_var2 >> 0x3))) != 0) {
            break;
        }
    }
    loop {
        pb_var4 = pb_var6;
        b_var2 = *pb_var4;
        if (b_var2 == 0) {
            // TODO: goto LAB_1000_483c;
        }
        pb_var6 = pb_var4 + 1;
        if !(('\x01' << (b_var2 & 0x7) & *(local_22 + (b_var2 >> 0x3))) == 0) {
            break;
        }
    }
    *pb_var4 = 0;
    pb_var4 = pb_var4 + 1; //
                           //    LAB_1000_483c:
    pbRam105061e4 = (pbRam105061e4 & 0xffff0000 | ZEXT24(pb_var4));
    return pbRam105061e4;
}

pub unsafe fn pass1_1000_484c(mut param_1: u32, mut param_2: u32, mut param_3: u16) -> u16 {
    let mut pb_var1: *mut u8;
    let mut pb_var2: *mut u8;
    let mut i_var3: i16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut pb_var6: *mut u8;
    let mut pb_var7: *mut u8;
    let mut i_var8: i16;
    let mut b_var9: bool;
    let mut b_var10: bool;

    if (param_3 == 0) {
        return 0x0;
    }
    loop {
        i_var8 = (param_2 >> 0x10);
        pb_var7 = param_2;
        i_var3 = (param_1 >> 0x10);
        pb_var6 = param_1;
        u_var4 = !pb_var7;
        u_var4 = ((param_3 - 1) - u_var4 & -(param_3 - 0x1 < u_var4)) + u_var4;
        u_var5 = !pb_var6;
        u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 1;
        b_var9 = param_3 < u_var4;
        param_3 -= u_var4;
        b_var10 = param_3 == 0;
        loop {
            if (u_var4 == 0) {
                break;
            }
            u_var4 -= 1;
            pb_var2 = pb_var7;
            pb_var7 = pb_var7 + 1;
            pb_var1 = pb_var6;
            pb_var6 = pb_var6 + 1;
            b_var9 = *pb_var1 < *pb_var2;
            b_var10 = *pb_var1 == *pb_var2;
            if b_var10 == false {
                break;
            }
        }
        param_2 = param_2 & 0xffff0000 | ZEXT24(pb_var7);
        if (!b_var10) {
            return (0x1 - b_var9) - (b_var9 != 0);
        }
        if (param_3 == 0) {
            return u_var4;
        }
        if (pb_var6.is_null()) {
            i_var3 += 0x6c;
        }
        param_1 = CONCAT22(i_var3, pb_var6);
        if (pb_var7.is_null()) {
            param_2 = (i_var8 + 0x6c) << 0x10;
            param_1 = CONCAT22(i_var3, pb_var6);
        }
    }
}

pub unsafe fn pass1_1000_48a8(mut param_1: u32, mut param_2: u32, mut param_3: i16) -> u16 {
    let mut puVar1: *mut u16;
    let mut puVar2: *mut u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut puVar7: *mut u16;
    let mut iVar8: i16;

    if (param_3 != 0) {
        loop {
            iVar3 = (param_2 >> 0x10);
            puVar6 = param_2;
            iVar8 = (param_1 >> 0x10);
            puVar7 = param_1;
            uVar4 = !puVar7;
            uVar4 = ((param_3 - 1) - uVar4 & -(param_3 - 0x1 < uVar4)) + uVar4;
            uVar5 = !puVar6;
            uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 1;
            param_3 -= uVar4;
            // for (uVar5 = uVar4 >> 0x1; uVar5 != 0; uVar5 -= 1)
            for uVar5 in uVar4 >> 0x1..0 {
                puVar2 = puVar7;
                puVar7 = puVar7 + 1;
                puVar1 = puVar6;
                puVar6 = puVar6 + 1;
                *puVar2 = *puVar1;
            }
            // for (uVar4 =  ((uVar4 & 1) != 0); uVar4 != 0; uVar4 -= 1)
            uVar4 = uVar4 & 1 != 0;
            while uVar4 != 0 {
                puVar2 = puVar7;
                puVar7 = (puVar7 + 1);
                puVar1 = puVar6;
                puVar6 = (puVar6 + 1);
                *puVar2 = *puVar1;
                uVar4 -= 1;
            }
            if (param_3 == 0) {
                break;
            }
            if (puVar6.is_null()) {
                iVar3 += 0x6c;
            }
            param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
            param_2 = CONCAT22(iVar3, puVar6);
            if (puVar7.is_null()) {
                param_1 = (iVar8 + 0x6c) << 0x10;
                param_2 = CONCAT22(iVar3, puVar6);
            }
        }
    }
    return param_1;
}

pub unsafe fn pass1_1000_4906(
    param_1: *mut StructD,
    in_wnd_class: *mut WNDCLASS16,
    mut param_3: u16,
) -> *mut u16 {
    let mut pu_var1: *mut u16;
    let mut u_var2: u8;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut struct_1: *mut astruct_20;
    let mut u_var5: u16;
    let mut pu_var6: *mut u16;
    let mut struct_1_hi: *mut astruct_20;

    if (param_3 != 0) {
        struct_1_hi = (param_1 >> 0x10);
        struct_1 = -param_1;
        u_var5 = param_3;
        if (struct_1.is_null() == false) {
            u_var5 = (struct_1 - param_3 & -(struct_1 < param_3)) + param_3;
            struct_1 = (param_3 - u_var5);
        }
        u_var3 = in_wnd_class & 0xff | in_wnd_class << 0x8;
        pu_var6 = param_1;
        // for (uVar4 = uVar5 >> 0x1; uVar4 != 0; uVar4 -= 1)
        for uVar4 in u_var5 >> 1..0 {
            pu_var1 = pu_var6;
            pu_var6 = pu_var6 + 1;
            *pu_var1 = u_var3;
        }
        // for (uVar5 =  ((u_var5 & 1) != 0);
        //      u_var2 =  ( in_wnd_class & 0xff), u_var5 != 0;
        //      u_var5 -= 1)
        u_var5 = u_var5 & 1 != 0;
        u_var2 = in_wnd_class & 0xff;
        while u_var5 != 0 {
            pu_var1 = pu_var6;
            pu_var6 = (pu_var6 + 1);
            *pu_var1 = u_var2;
            u_var5 -= 1;
        }
        if (struct_1.is_null() == false) {
            // for (uVar5 =  struct_1 >> 0x1; u_var5 != 0; u_var5 -= 1)
            for u_var5 in struct_1 >> 1..0 {
                pu_var1 = pu_var6;
                pu_var6 = pu_var6 + 1;
                *pu_var1 = u_var3;
            }
            // for (uVar5 =  (( struct_1 & 1) != 0); u_var5 != 0; u_var5 -= 1)

            u_var5 = struct_1 & 1 != 0;
            while u_var5 != 0 {
                pu_var1 = pu_var6;
                pu_var6 = (pu_var6 + 1);
                *pu_var1 = u_var2;
                u_var5 -= 1;
            }
        }
    }
    return param_1;
}

pub unsafe fn pass1_1000_49b2(mut param_1: u16) -> i16 {
    return (param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf);
}

pub unsafe fn pass1_1000_49c6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    fn_ptr_param_7: code5,
) -> u16 {
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
    uVar7 = pass1_1000_52be(param_5 - 0x1, -(param_5 == 0), param_6, 0x0);
    uStack8 = (uVar7 + 0x8);
    uStack6 = (uVar7 + 0x8 >> 0x10) * 0x100 + param_4;
    loop {
        if (uStack6 < uStack18) {
            return 0x0;
        }
        if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
            return 0x0;
        }
        uVar1 = param_5 >> 0x1;
        if (uVar1 == 0) {
            if ((param_5 != 0) && (iVar5 = (fn_ptr_param_7)(), iVar5 == 0)) {
                return uStack20;
            }
            return 0x0;
        }
        uVar2 = uVar1;
        if ((param_5 & 1) == 0) {
            uVar2 = uVar1 - 0x1;
        }
        uVar3 = (uVar2 * param_6);
        uVar4 = uVar3 + uStack20;
        iVar6 = ((uVar2 * param_6 >> 0x10) + CARRY2(uVar3, uStack20)) * 0x100 + uStack18;
        iVar5 = fn_ptr_param_7();
        if (iVar5 == 0) {
            break;
        }
        if (iVar5 < 0x0) {
            uStack8 = -param_6 + uVar4;
            uStack6 = (CARRY2(-param_6, uVar4) - (param_6 != 0)) * 0x100 + iVar6;
            uVar2 = param_5 & 0x1;
            param_5 = uVar1;
            if (uVar2 == 0) {
                param_5 = uVar1 - 0x1;
            }
        } else {
            uStack20 = param_6 + uVar4;
            uStack18 = CARRY2(param_6, uVar4) * 0x100 + iVar6;
            param_5 = uVar1;
        }
    }
    return uVar4;
}
pub unsafe fn pass1_1000_4aea(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    fn_ptr_param_5: code5,
) {
    let mut pu_var1: *mut u16;
    // let mut ppcVar2: *mut *mut code;
    let mut ppc_var2: *mut *mut code;
    let mut l_var3: i32;
    let mut u_var4: u16;
    let mut i_var5: i16;
    let mut i_var6: i16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut pu_var11: *mut astruct_171;
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

    if ((param_4 != 0) && (param_3 != 0)) {
        u_stack_y14 = param_1;
        u_var11 = param_2;
        // for (iVar6 = param_3 -0x1; iVar6 != 0; iVar6 += -1)
        for i_var6 in param_34 - 1..0 {
            u_var9 = u_stack_y14 + param_4;
            u_var11 += -CARRY2(u_stack_y14, param_4) & 0x6c;
            u_stack_y18 = u_var9;
            u_stack_y16 = u_var11;
            i_var5 = fn_ptr_param_5();
            if (i_var5 < 0x0) {
                u_var11 = param_3 - 0x1;
                i_var6 = 0;
                loop {
                    u_var11 >>= 0x1;
                    i_var6 += -0x1;
                    if !(i_var6 != 0x0 && u_var11 != 0) {
                        break;
                    }
                }
                if (((-i_var6 * 0x8 >> 0x10) != 0)
                    || (u_var11 = pass1_1000_3bac(), u_var11 < (-i_var6 * 0x8)))
                {
                    exit_1000_25f2(-0x4, 0x4b7b, REG_CS, unaff_di);
                    return;
                }
                pu_var11 = &stack0xfff6;
                l_var3 = (param_3 - 1) * param_4;
                u_var11 = l_var3;
                u_stack_y14 = u_var11 + param_1;
                u_var11 = ((l_var3 >> 0x10) + CARRY2(u_var11, param_1)) * 0x100 + param_2;
                u_stack_y16 = param_2;
                u_stack_y18 = param_1; //
                                       //                LAB_1000_4b7d:
                if (pu_var11 <= &u_stack_y18) {
                    return;
                } //
                  //                LAB_1000_4b81:
                if ((u_stack_y16 < u_var11)
                    || (u_stack_y16 <= u_var11 && (u_stack_y18 < u_stack_y14)))
                {
                    u_stack_y22 = u_stack_y14;
                    pu_var1 = &pu_var11.field20_0x14;
                    u_var8 = u_stack_y14 + *pu_var1;
                    u_var7 = u_var11 + (-CARRY2(u_stack_y14, *pu_var1) & 0x6c);
                    u_var9 = u_stack_y16;
                    u_var10 = u_stack_y18;
                    u_stack_y26 = u_stack_y18;
                    u_stack_y24 = u_stack_y16;
                    u_var13 = u_var11; //
                                       //                    LAB_1000_4bbc:
                    loop {
                        pu_var1 = &pu_var11.field20_0x14;
                        b_var12 = CARRY2(u_var10, *pu_var1);
                        u_var10 += *pu_var1;
                        u_var9 += -b_var12 & 0x6c;
                        u_var4 = u_stack_y22;
                        if ((u_var10 != u_stack_y14) || (u_var9 != u_var11)) {
                            //                            ppcVar2 = puVar11.field21_0x16;
                            i_var6 = pu_var11.field21_0x16();
                            if (i_var6 < 1) {
                                if (i_var6 != 0) {
                                    u_stack_y26 = u_var10;
                                    u_stack_y24 = u_var9;
                                }
                                // TODO: goto LAB_1000_4bbc;
                            }
                        }
                        loop {
                            u_var14 = u_var13;
                            u_stack_y22 = u_var4;
                            pu_var1 = &pu_var11.field20_0x14;
                            b_var12 = u_var8 < *pu_var1;
                            u_var8 -= *pu_var1;
                            u_var7 -= -b_var12 & 0x6c;
                            //                            ppcVar2 =  &puVar11.field21_0x16;
                            //                            iVar6 = (**ppcVar2)();
                            i_var6 = pu_var11.field21_0x16();
                            if (0x0 < i_var6) {
                                break;
                            }
                            u_var4 = u_var8;
                            u_var13 = u_var7;
                            if !(((i_var6 != 0)
                                || (
                                    u_var4 = u_stack_y22,
                                    u_var13 = u_var14,
                                    u_var8 != u_stack_y18,
                                ))
                                || (u_var7 != u_stack_y16))
                            {
                                break;
                            }
                        }
                        if ((u_var7 < u_var9) || (u_var7 <= u_var9 && (u_var8 <= u_var10))) {
                            // TODO: goto LAB_1000_4c58;
                        }
                        pass1_1000_4ceb(pu_var11.field20_0x14);
                        u_stack_y26 = u_var10;
                        u_stack_y24 = u_var9;
                        u_var13 = u_var7;
                        u_stack_y22 = u_var8;
                    }
                }
                // TODO: goto LAB_1000_4b7d;
            }
            u_stack_y14 = u_var9;
        }
    }
    return; //
            //    LAB_1000_4c58:
    pass1_1000_4ceb(pu_var11.field20_0x14);
    u_var10 = ((u_var11 - (-(u_stack_y14 < u_stack_y22) & 0x6c)) - u_var14)
        + (-CARRY2(u_stack_y14 - u_stack_y22, u_stack_y18) & 0x6c)
        + u_stack_y16;
    u_var9 = -((u_stack_y14 - u_stack_y22) + u_stack_y18 < u_stack_y26) & 0x6c;
    if ((u_var10 < u_var9) || (u_var10 - u_var9 < u_stack_y24)) {
        u_stack_y14 = u_stack_y26;
        u_var11 = u_stack_y24;
    } else {
        u_stack_y18 = u_stack_y22;
        u_stack_y16 = u_var14;
    }
    // TODO: goto LAB_1000_4b81;
}
pub unsafe fn pass1_1000_4ceb(mut param_1: u16) {
    let mut pu_var1: *mut u8;
    let mut pu_var2: *mut u16;
    let mut u_var3: u8;
    let mut u_var4: u16;
    let mut unaff_si: i16;
    let mut unaff_di: i16;
    let mut unaff_es: u16;

    if ((param_1 & 1) != 0) {
        param_1 -= 1;
        pu_var1 = (param_1 + unaff_di);
        u_var3 = *pu_var1;
        *pu_var1 = *(param_1 + unaff_si);
        *(param_1 + unaff_si) = u_var3;
        if (param_1 == 0) {
            return;
        }
    }
    loop {
        param_1 -= 0x2;
        pu_var2 = (param_1 + unaff_di);
        u_var4 = *pu_var2;
        *pu_var2 = (param_1 + unaff_si);
        (param_1 + unaff_si) = u_var4;
        if param_1 == 0 {
            break;
        }
    }
    return;
}
pub unsafe fn pass1_1000_4d0c(mut param_1: u16) {
    DAT_1050_61e8 = param_1;
    PTR_LOOP_1050_61ea = null_mut();
    return;
}

pub unsafe fn pass1_1000_4d24() -> u16 {
    let mut u_var1: u32;

    u_var1 = pass1_1000_52be(
        DAT_1050_61e8,
        PTR_LOOP_1050_61ea,
        s_TPPOPMENU_1050_43fa + 0x3,
        0x3,
    );
    PTR_LOOP_1050_61ea = (u_var1 + 0x269ec3 >> 0x10);
    DAT_1050_61e8 = (u_var1 + 0x269ec3);
    return PTR_LOOP_1050_61ea & 0x7fff;
}
pub unsafe fn str_1000_4d58(
    in_string_1: *mut c_char,
    in_string_2: *mut c_char,
    mut param_3: u32,
    mut param_4: u32,
    param_5: *mut WNDCLASS16,
) {
    let mut u_var1: u16;
    let mut i_var2: i16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut pc_stack18: *mut c_char;
    let mut u_stack12: u16;
    let mut u_stack10: u16;
    let mut u_stack8: u16;
    let mut u_stack6: u16;

    u_stack10 = 0;
    u_stack12 = 0;
    u_var4 = (in_string_1 >> 0x10);
    i_var2 = in_string_1;
    if ((*in_string_1 == '\0') || ((i_var2 + 1) != ':')) {
        if (in_string_2.is_null() == false) {
            *in_string_2 = '\0';
        }
    } else {
        if (in_string_2.is_null() == false) {
            *in_string_2 = *in_string_1;
            *(in_string_2 + 1) = *(i_var2 + 1);
            *(in_string_2 + 0x2) = 0;
        }
        in_string_1 = (in_string_1 & 0xffff0000 | (i_var2 + 0x2));
    }
    u_stack6 = 0;
    u_stack8 = 0;
    pc_stack18 = in_string_1;
    while (true) {
        u_var5 = (pc_stack18 >> 0x10);
        u_var3 = pc_stack18;
        if (*pc_stack18 == '\0') {
            break;
        }
        if ((*pc_stack18 == '/') || (*pc_stack18 == '\\')) {
            u_stack8 = u_var3 + 1;
            u_stack6 = u_var5;
        } else if (*pc_stack18 == '.') {
            u_stack12 = u_var3;
            u_stack10 = u_var5;
        }
        pc_stack18 = (pc_stack18 & 0xffff0000 | (u_var3 + 1));
    }
    if ((u_stack6 | u_stack8) == 0) {
        if (param_3 != 0) {
            *param_3 = 0;
        }
    } else {
        if (param_3 != 0) {
            u_var1 = u_stack8 - in_string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe((param_3 & 0xffff | param_3 << 0x10), in_string_1, u_var1);
            *(param_3 + u_var1) = 0;
        }
        in_string_1 = CONCAT22(u_stack6, u_stack8);
    }
    if (((u_stack10 | u_stack12) != 0) && (in_string_1 <= u_stack12)) {
        if (param_4 != 0) {
            u_var1 = u_stack12 - in_string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe(
                (param_4 & 0xffff | param_4 << 0x10),
                (in_string_1 & 0xffff | in_string_1 << 0x10),
                u_var1,
            );
            *(param_4 + u_var1) = 0;
        }
        if (param_5.is_null()) {
            return;
        }
        u_var1 = u_var3 - u_stack12;
        if (0xff < u_var1) {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe(
            (param_5 & 0xffff | param_5 << 0x10),
            CONCAT22(u_stack10, u_stack12),
            u_var1,
        );
        *(param_5 + u_var1) = 0;
        return;
    }
    if (param_4 != 0) {
        u_var1 = u_var3 - in_string_1;
        if (0xff < u_var1) {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe(
            (param_4 & 0xffff | param_4 << 0x10),
            (in_string_1 & 0xffff | in_string_1 << 0x10),
            u_var1,
        );
        *(param_4 + u_var1) = 0;
    }
    if (param_5.is_null() == false) {
        *&param_5.style = 0;
    }
    return;
}

/*
Unable to decompile 'pass1_1000_4f1a'
Cause:
Low-level Error: Symbol $$undef00000008 extends beyond the end of the address space
*/

// WARNING: Removing unreachable block (ram,0x10004f47)

// WARNING: Removing unreachable block (ram,0x10004f47)

pub unsafe fn pass1_1000_4f2e() -> u16 {
    let mut pc_var1: *mut code;
    let mut u_var2: u16;
    let mut unaff_bp: i16;
    let mut b_var3: bool;

    b_var3 = false;
    // dos function dispatcher
    pc_var1 = swi(0x21);
    u_var2 = (*pc_var1)(&DAT_1050_1050, unaff_bp + 1);
    if (b_var3) {
        pass1_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0x0;
}
