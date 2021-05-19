use crate::bad_funcs::bad1;
use crate::err_funcs::error_check_1000_17ce;
use crate::struct_funcs::{process_struct_1010_20ba, process_struct_1040_7728};
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT21, SBORROW1, SBORROW2, SCARRY1, ZEXT24};

pub unsafe fn bad_fn_10d0_02c2() {
    let pb_var1: Vec<u8>;
    let mut in_al: u8;
    let mut in_bx: i32;
    let mut unaff_si: i32;
    let mut unaff_ds: u16;

    pb_var1 = (in_bx + unaff_si);
    unsafe { *pb_var1 = *pb_var1 | in_al };
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1110_029e() {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let pi_var3: *mut i32;
    let ppc_var4: *mut String;
    let pu_var5: *mut u32;
    let pu_var6: Vec<u8>;
    let pc_var7: String;
    let pu_var8: *mut u16;
    let ppc_var9: *mut String;
    let mut c_var10: u8;
    let pc_var11: *mut code;
    let mut in_al: u8;
    let mut c_var12: u8;
    let mut b_var13: u8;
    let mut c_var14: u8;
    let mut b_var15: u8;
    let mut b_var16: u8;
    let mut c_var17: u8;
    let mut c_var19: u8;
    let mut c_var20: u8;
    let mut in_cx: i32;
    let mut u_var21: i32;
    let mut extraout_dl: u8;
    let mut extraout_dl_00: u8;
    let mut c_var22: u8;
    let mut c_var23: u8;
    let mut in_bx: i32;
    let mut u_var25: i32;
    let pu_var26: *mut u32;
    let pi_var27: *mut i32;
    let mut unaff_bp: u16;
    let unaff_si: String;
    let pc_var29: String;
    let unaff_di: String;
    let mut unaff_es: u16;
    let mut unaff_ds: u16;
    let mut in_fs: u16;
    let mut u_var30: u32;
    let mut uStack000d: u16;
    let mut in_stack_0000000c: u32;
    let mut uStack0019: u16;
    let mut in_stack_00000018: u32;
    let mut in_stack_00000062: u16;
    let mut bStack0063: u8;
    let mut c_stack17: u8;
    let uStack3: u8;
    let mut u_var32: i32;
    let mut i_var18: i32;
    let mut u_var24: i32;
    let mut b_var28: u8;
    let mut u_var31: u32;
    let mut _uStack3: u16;
    let mut stack0xfffe: u16;
    let mut stack0x0015: u16;
    let mut stack0x001d: u16;
    let mut stack0x001f: u16;

    // _uStack3 = T21(unaff_bp, uStack3);
    u_var31 = _uStack3;
    pc_var1 = unaff_si + in_bx;
    unsafe {
        *pc_var1 = *pc_var1 + in_al;
        pc_var11 = swi(0);
        c_var12 = (*pc_var11)();
        pc_var1 = unaff_si;
        *pc_var1 = *pc_var1 + extraout_dl;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + in_cx;
        u_var21 = in_cx & 0xff00 | (in_cx * 0x2);
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + c_var12;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + c_var12;
        pc_var11 = swi(0);
        c_var12 = (*pc_var11)();
        pc_var1 = unaff_si;
        *pc_var1 = *pc_var1 + c_stack17;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + u_var21;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + c_var12;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + c_var12;
        pc_var11 = swi(0);
        c_var12 = (*pc_var11)();
        pc_var1 = unaff_si;
        *pc_var1 = *pc_var1 + extraout_dl_00;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + u_var21;
        u_var25 = in_bx & 0xff00 | (in_bx + u_var21);
        pc_var1 = unaff_si + u_var25;
        *pc_var1 = *pc_var1 + c_var12;
        pc_var1 = unaff_si + u_var25;
        *pc_var1 = *pc_var1 + c_var12;
        pc_var11 = swi(0);
        u_var30 = (*pc_var11)();
        pc_var7 = unaff_si + 1;
        bad1::out(*unaff_si, (u_var30 >> 0x10));
        pc_var1 = pc_var7;
        *pc_var1 = *pc_var1 + (u_var30 >> 0x10);
        pc_var1 = pc_var7 + u_var25;
        c_var20 = u_var21;
        *pc_var1 = *pc_var1 + c_var20;
        c_var19 = (u_var30 >> 8) + c_var20;
        b_var13 = u_var30;
        _uStack3 = u_var31;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 + b_var13;
        pu8_var2 = (pc_var7 + u_var25);
        b_var16 = *pu8_var2;
        *pu8_var2 = *pu8_var2 + b_var13;
        b_var13 = b_var13 + CARRY1(b_var16, b_var13);
        i_var18 = CONCAT11(c_var19, b_var13);
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 - b_var13;
        pu8_var2 = (pc_var7 + u_var25);
        *pu8_var2 = *pu8_var2 | b_var13;
        *0x17 = b_var13;
        pc_var1 = &stack0xfffe + pc_var7;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1;
        pc_var1 = pc_var7 + u_var25;
        c_var22 = (u_var21 >> 8);
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = &stack0x0015 + unaff_di;
        *pc_var1 = *pc_var1 + c_var19;
        pc_var1 = &stack0xfffe + pc_var7;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1;
        pi_var3 = (pc_var7 + u_var25);
        *pi_var3 = *pi_var3 - i_var18;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 - b_var13;
        pu8_var2 = (pc_var7 + u_var25);
        *pu8_var2 = *pu8_var2 | b_var13;
        pu_var8 = (unaff_si + 2);
        *unaff_di = *pc_var7;
        pc_var1 = (u_var25 + pu_var8);
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = (u_var25 + pu_var8);
        *pc_var1 = *pc_var1 + b_var13;
        b_var13 = b_var13 ^ *(u_var25 + pu_var8);
        pc_var1 = (u_var25 + pu_var8);
        *pc_var1 = *pc_var1 - b_var13;
        pu8_var2 = (u_var25 + pu_var8);
        *pu8_var2 = *pu8_var2 | b_var13;
        pc_var7 = unaff_si + 4;
        (unaff_di + 1) = *pu_var8;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = pc_var7 + u_var25;
        *pc_var1 = *pc_var1 - b_var13;
        pu8_var2 = (pc_var7 + u_var25);
        *pu8_var2 = *pu8_var2 | b_var13;
        pc_var7 = unaff_di + 4;
        pc_var1 = unaff_si + 5 + u_var25;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = unaff_si + 5 + u_var25;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var29 = unaff_si + 6;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = (u_var25 + 0x17);
        *pc_var1 = *pc_var1 + c_var19;
        pc_var1 = &stack0xfffe + pc_var29;
        *pc_var1 = *pc_var1 + b_var13;
        u_var32 = (u_var31 >> 8) & 0xff00 | (CONCAT11(c_var19, b_var13) >> 8);
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = pc_var29 + u_var25 + 0x17;
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = &stack0xfffe + pc_var29;
        *pc_var1 = *pc_var1 + b_var13;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var20;
        c_var14 = b_var13 * 0x2;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var14;
        c_var12 = pc_var29[u_var25 - 0x7e];
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var14;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var20;
        c_var20 = c_var20 + c_var14;
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var14;
        c_var12 = b_var13 + c_var12 + pc_var29[u_var25 - 0x7e];
        pc_var1 = pc_var29 + u_var25;
        *pc_var1 = *pc_var1 + c_var14;
        ppc_var9 = (unaff_si + 7);
        bad1::out(*pc_var29, CONCAT11(c_var19, c_var12));
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var22;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        c_var22 = c_var12 + c_var14 + *(ppc_var9 + (u_var25 - 0x7e));
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        ppc_var4 = ppc_var9;
        *ppc_var4 = *ppc_var4 + CONCAT11(c_var19, c_var22);
        ppc_var4 = ppc_var9;
        *ppc_var4 = *ppc_var4 + c_var22;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        c_var12 = *(ppc_var9 + (u_var25 - 0x7e));
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        pi_var3 = 0x1400;
        *pi_var3 = *pi_var3 + u_var25;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        c_var22 = c_var22 + c_var12 + *(ppc_var9 + (u_var25 - 0x7e));
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        pi_var3 = (pc_var7 + u_var25);
        *pi_var3 = (&stack0xfffe + *pi_var3);
        ppc_var4 = ppc_var9;
        *ppc_var4 = *ppc_var4 + c_var22;
        pc_var1 = (u_var25 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var20;
        b_var28 = (u_var25 >> 8) + c_var20;
        u_var25 = u_var25 & 0xff;
        pu_var26 = (u_var25 | b_var28 << 8);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        c_var22 = c_var22 + *(pu_var26 + ppc_var9 + -0x7e);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        pi_var3 = (&stack0xfffe + ppc_var9);
        *pi_var3 = *pi_var3 + ppc_var9;
        ppc_var4 = ppc_var9;
        *ppc_var4 = *ppc_var4 + c_var22;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var20;
        c_var14 = c_var14 + c_var22;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        c_var22 = c_var22 + *(pu_var26 + ppc_var9 + -0x7e);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        ppc_var4 = ppc_var9;
        *ppc_var4 = pc_var7 + *ppc_var4;
        ppc_var4 = ppc_var9;
        *ppc_var4 = *ppc_var4 + c_var22;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var20;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        c_var12 = *(pu_var26 + ppc_var9 + -0x7e);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var14;
        b_var15 = c_var14 + CARRY2(u_var32, CONCAT11(c_var19, c_var14));
        pu8_var2 = (pu_var26 + ppc_var9);
        *pu8_var2 = *pu8_var2 | b_var15;
        pu_var5 = pu_var26;
        b_var13 = (c_var20 + c_var22 & 0x1f) % 9;
        b_var16 = *pu_var5;
        *pu_var5 = b_var16 << b_var13 | b_var16 >> 9 - b_var13;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + b_var15;
        c_var23 = c_var22 + c_var12 + *(pu_var26 + ppc_var9 + -0x7e);
        u_var24 = CONCAT11(c_var19 + c_var20, c_var23);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + b_var15;
        pu_var5 = (pu_var26 + pc_var7);
        u_var21 = *pu_var5;
        *pu_var5 = *pu_var5 + u_var24;
        b_var15 = b_var15 + CARRY2(u_var21, u_var24);
        pu8_var2 = (pu_var26 + ppc_var9);
        *pu8_var2 = *pu8_var2 | b_var15;
        pu_var5 = pu_var26;
        b_var16 = (c_var20 + c_var22 & 0x1f) % 0x11;
        u_var21 = *pu_var5;
        *pu_var5 = u_var21 << b_var16 | u_var21 >> 0x11 - b_var16;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + b_var15;
        c_var12 = *(pu_var26 + ppc_var9 + -0x7e);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + b_var15;
        pu_var5 = (&stack0xfffe + ppc_var9);
        u_var21 = *pu_var5;
        *pu_var5 = *pu_var5 + pu_var26;
        b_var15 = b_var15 + CARRY2(u_var21, pu_var26);
        pu8_var2 = (pu_var26 + ppc_var9);
        *pu8_var2 = *pu8_var2 | b_var15;
        b_var16 = b_var15 % 0x17;
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + b_var16;
        c_var22 = *(pu_var26 + ppc_var9 + -0x7e);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + b_var16;
        ppc_var4 = ppc_var9;
        pc_var29 = *ppc_var4;
        *ppc_var4 = &stack0x001d + *ppc_var4;
        b_var16 = b_var16 + CARRY2(pc_var29, &stack0x001d);
        pu8_var2 = (pu_var26 + ppc_var9);
        *pu8_var2 = *pu8_var2 | b_var16;
        c_var17 = b_var16 + (b_var15 / 0x17) * '\x17';
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var17;
        c_var10 = *(pu_var26 + ppc_var9 + -0x7e);
        pc_var1 = (pu_var26 + ppc_var9);
        *pc_var1 = *pc_var1 + c_var17;
        pu8_var2 = (pu_var26 + ppc_var9);
        *pu8_var2 =
            *pu8_var2 | c_var17 + CARRY2(u_var32 + CONCAT11(c_var19, c_var14), &stack0xfffe);
        pu_var6 = (pu_var26 + ppc_var9);
        *pu_var6 = *pu_var6;
        c_var14 = *(pu_var26 + ppc_var9 + -0x7e);
        pu_var6 = (pu_var26 + ppc_var9);
        *pu_var6 = *pu_var6;
        pc_var1 = pc_var7;
        b_var13 = ppc_var9;
        *pc_var1 = *pc_var1 + b_var13;
        pu_var5 = pu_var26 + 0x3c00;
        *pu_var5 = *pu_var5 + u_var25;
        pi_var27 = (u_var25 | (b_var28 * 0x2) << 8);
        pi_var3 = pi_var27;
        *pi_var3 = *pi_var3 + 1;
        pi_var3 = pi_var27;
        b_var16 = *pi_var3;
        *pi_var3 = *pi_var3 + b_var13;
        bad1::out(
            0x0,
            CONCAT11(
                c_var19 + c_var20,
                c_var23 + c_var12 + c_var22 + c_var10 + c_var14,
            ),
        );
        if ((bStack0063 + 0x73 + CARRY1(b_var16, b_var13)) == '\0') {
            pu_var5 = (unaff_di + 0x77);
            *pu_var5 = *pu_var5
                + (0x8c < bStack0063 || CARRY1(bStack0063 + 0x73, CARRY1(b_var16, b_var13)))
                    * ((&stack0x001f & 3) - (*pu_var5 & 3));
            pi_var3 = pi_var27 + 1;
            *pi_var3 = *pi_var3 + b_var13;
            // WARNING: Bad instruction - Truncating control flow here
            bad1::halt_baddata();
        }
    }
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_3d38() {
    let pi_var1: *mut i32;
    let mut c_var2: u8;
    let pc_var3: *mut code;
    let pu_var4: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_si: i32;
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut in_OF: u8;
    let in_ST0: [u8; 10];
    let puStack34: Vec<u8>;
    let stack0x0000: u16;
    let stack0xfffe: u16;

    puStack34 = &stack0xfffe;
    pu_var4 = &stack0xfffe;
    c_var2 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var4 = pu_var4 + -1;
        unsafe { *pu_var4 = *unaff_bp };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    pc_var3 = swi(4);
    if (in_OF == 0x1) {
        unsafe { *pc_var3() };
    }
    (&stack0x0000 + unaff_si) = in_ST0;
    bad1::_in(0x3a);
    pi_var1 = (unaff_si + -0x51fd);
    unsafe { *pi_var1 = *pi_var1 + unaff_si + 3 };
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

fn swi(arg: i32) -> () {
    todo!()
}

pub unsafe fn bad_fn_1050_335f() {
    let pc_var1: String;
    let mut in_al: u8;
    let mut in_AH: u8;
    let mut in_bx: i32;
    let unaff_si: String;
    let mut unaff_ds: u16;

    pc_var1 = unaff_si;
    unsafe {
        *pc_var1 = *pc_var1 + in_AH;
        pc_var1 = unaff_si + in_bx;
        *pc_var1 = *pc_var1 + in_al;
    }
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_180a() {
    let pb_var1: Vec<u8>;
    let pc_var2: String;
    let pu_var3: *mut u16;
    let pu_var4: *mut u32;
    let pi_var5: *mut i32;
    let u_var6: u8;
    let mut b_var7: u8;
    let mut b_var8: u8;
    let mut b_var9: u8;
    let mut b_var10: u8;
    let mut b_var11: u8;
    let lVar12: u32;
    let mut b_var13: u8;
    let mut b_var14: u8;
    let mut b_var15: u8;
    let mut b_var16: u8;
    let mut b_var17: u8;
    let in_ax: *mut u16;
    let mut u_var18: u16;
    let mut u_var19: u16;
    let mut c_var20: u8;
    let mut in_cx: i32;
    let mut i_var21: i32;
    let mut b_var23: u8;
    let mut u_var22: u16;
    let mut b_var24: u8;
    let mut c_var25: u8;
    let mut in_dx: u16;
    let mut b_var27: u8;
    let mut u_var26: i32;
    let in_bx: Vec<u8>;
    let pu8_var28: Vec<u8>;
    let pi_var29: *mut i32;
    let mut u_var30: i32;
    let ppc_var31: *mut String;
    let ppc_var32: *mut String;
    let ppc_var33: *mut String;
    let ppi_var34: *mut *mut i32;
    let ppi_var35: *mut *mut i32;
    let ppi_var36: *mut *mut i32;
    let mut i_var37: i32;
    let ppi_var38: *mut *mut i32;
    let ppi_var39: *mut *mut i32;
    let ppi_var40: *mut *mut i32;
    let ppi_var41: *mut *mut i32;
    let unaff_bp: *mut u16;
    let mut u_var42: i32;
    let mut u_var43: i32;
    let pu_var44: *mut u32;
    let mut u_var45: i32;
    let pu_var46: *mut u32;
    let mut iVar47: i32;
    let mut u_var48: i32;
    let pi_var49: *mut i32;
    let in_esi: *mut u16;
    let unaff_di: *mut u16;
    let pu_var50: *mut u16;
    let pu_var51: Vec<u8>;
    let pu_var52: Vec<u8>;
    let pu_var53: *mut u16;
    let mut i_var54: i32;
    let pu_var55: *mut u16;
    let pc_var56: String;
    let pi_var57: *mut i32;
    let pi_var58: *mut i32;
    let pi_var59: *mut i32;
    let mut unaff_es: u16;
    let mut unaff_ss: i32;
    let mut unaff_ds: u16;
    let mut in_fs: u16;
    let mut in_gs: u16;
    let mut in_CF: u8;
    let mut b_var60: bool;
    let mut b_var61: bool;
    let mut b_var62: bool;
    let mut in_af: u8;
    let mut in_ZF: u8;
    let mut in_stack_00000070: u8;
    let mut in_stack_0000afbc: i32;
    let mut in_stack_0000afc2: i32;
    let mut in_stack_0000afc4: i32;
    let in_stack_0000afca: Vec<u8>;
    let in_stack_0000afcc: Vec<u8>;
    let mut in_stack_0000afce: i32;
    let in_stack_0000afd2: *mut u32;
    let mut in_stack_0000afd4: i32;
    let mut in_stack_0000afd6: u16;
    let mut in_stack_0000afd8: u8;
    let mut stack0xfffe: u16;

    pu_var50 = &stack0xfffe;
    c_var25 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var50 = pu_var50 + -1;
        unsafe { *pu_var50 = *unaff_bp };
        c_var25 = c_var25 + -1;
        '\0' < c_var25
    } {}
    pb_var1 = 0x1050;
    b_var60 = unsafe { *pb_var1 < '\0' };
    unsafe { *pb_var1 = *pb_var1 << 1 | in_CF };
    i_var21 = in_cx + -1;
    iVar47 = in_esi;
    b_var16 = (i_var21 >> 8);
    b_var15 = in_dx;
    b_var23 = (in_dx >> 8);
    b_var24 = (in_bx >> 8);
    if (i_var21 == 0 || in_ZF != '\0') {
        unsafe {
            b_var13 = in_ax + b_var16;
            b_var61 = CARRY1(in_ax, b_var16) || CARRY1(b_var13, b_var60);
            b_var13 = b_var13 + b_var60;
            in_stack_0000afc2 = in_ax & 0xff00 | b_var13;
            b_var60 = CARRY1(b_var13, b_var23) || CARRY1(b_var13 + b_var23, b_var61);
            b_var14 = b_var13 + b_var23 + b_var61;
            b_var13 = b_var14 + b_var24;
            b_var61 = CARRY1(b_var14, b_var24) || CARRY1(b_var13, b_var60);
            b_var13 = b_var13 + b_var60;
            in_ax = (in_ax & 0xff00 | b_var13);
            pb_var1 = in_bx + iVar47;
            b_var60 = CARRY1(*pb_var1, b_var13) || CARRY1(*pb_var1 + b_var13, b_var61);
            *pb_var1 = *pb_var1 + b_var13 + b_var61;
            pb_var1 = in_bx + iVar47;
            b_var13 = *pb_var1 + i_var21;
            b_var62 = CARRY1(*pb_var1, i_var21) || CARRY1(b_var13, b_var60);
            *pb_var1 = b_var13 + b_var60;
            pb_var1 = in_bx + iVar47;
            b_var61 = CARRY1(*pb_var1, b_var15) || CARRY1(*pb_var1 + b_var15, b_var62);
            *pb_var1 = *pb_var1 + b_var15 + b_var62;
            in_stack_0000afbc = unaff_ss;
            in_stack_0000afc4 = unaff_ss;
        }
    } else {
        unsafe {
            pb_var1 = in_bx;
            b_var61 = CARRY1(*pb_var1, b_var15) || CARRY1(*pb_var1 + b_var15, b_var60);
            *pb_var1 = *pb_var1 + b_var15 + b_var60;
        }
    }

    unsafe {
        pb_var1 = in_bx + iVar47;
        b_var15 = *pb_var1 + in_bx;
        b_var60 = CARRY1(*pb_var1, in_bx) || CARRY1(b_var15, b_var61);
        *pb_var1 = b_var15 + b_var61;
        pb_var1 = in_bx + unaff_di;
        b_var15 = *pb_var1;
        b_var14 = (in_ax >> 8);
        b_var13 = *pb_var1;
        *pb_var1 = b_var13 + b_var14 + b_var60;
        if (SCARRY1(in_stack_00000070, b_var23)
            == SCARRY1(
                in_stack_00000070 + b_var23,
                CARRY1(b_var15, b_var14) || CARRY1(b_var13 + b_var14, b_var60),
            ))
        {
            pu_var50 = (unaff_di + 1);
            u_var6 = bad1::_in(in_dx);
            *unaff_di = u_var6;
            pc_var2 = (iVar47 + 0x69);
            *pc_var2 = *pc_var2 + b_var14;
            if (-1 < *pc_var2) {
                pc_var2 = (iVar47 + 0x6f);
                *pc_var2 = *pc_var2 + b_var16;
                bad1::out(*in_esi, in_dx);
                pc_var2 = 0x3035;
                *pc_var2 = *pc_var2 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                pu_var3 = pu_var50;
                *pu_var3 = *pu_var3 + b_var23;
                unaff_di = (unaff_di + 3);
                u_var22 = bad1::_in(in_dx);
                *pu_var50 = u_var22;
                pu_var3 = unaff_di;
                *pu_var3 = *pu_var3 + b_var23;
                in_esi = in_esi + 1;
                // goto code_r0x105018a4;
            }
        } else {
            // code_r0x105018a4:
            pu_var3 = unaff_di;
            *pu_var3 = *pu_var3 + b_var23;
            pu_var3 = unaff_di;
            *pu_var3 = *pu_var3 + b_var23;
            pu_var3 = unaff_di;
            *pu_var3 = *pu_var3 + b_var23;
            pu_var3 = unaff_di;
            *pu_var3 = *pu_var3 + b_var23;
            pu_var50 = unaff_di;
        }
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var23;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var23;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var23;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var23;
        u_var22 = bad1::_in(in_dx);
        *pu_var50 = u_var22;
        pc_var2 = 0x3230;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3330;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3430;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3530;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3630;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3834;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3830;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3930;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3031;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3131;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3231;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3331;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3431;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3531;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3631;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3731;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3831;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3931;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3032;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3132;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3232;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3332;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3432;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3532;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3632;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3732;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3832;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3932;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3033;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3133;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3233;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3333;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3433;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3533;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3633;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3733;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3833;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3933;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3034;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3234;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3334;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3434;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3534;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3634;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3734;
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = 0x3934;
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = in_bx + in_esi;
        *pb_var1 = *pb_var1 + b_var24;
        pc_var2 = (in_stack_0000afc2 + in_stack_0000afbc);
        c_var25 = (in_stack_0000afc2 >> 8);
        *pc_var2 = *pc_var2 + c_var25;
        u_var22 = bad1::_in(in_stack_0000afc4);
        *in_ax = u_var22;
        pc_var2 = (in_stack_0000afc2 + in_stack_0000afbc);
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = (in_stack_0000afc2 + in_stack_0000afbc);
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = (in_stack_0000afd2 + in_stack_0000afcc);
        b_var13 = (in_stack_0000afd2 >> 8);
        *pc_var2 = *pc_var2 + b_var13;
        b_var15 = in_stack_0000afd8 ^ 0x39;
        pu_var4 = (in_stack_0000afcc + 0x69);
        *pu_var4 = *pu_var4 ^ in_stack_0000afcc;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *in_stack_0000afca = u_var6;
        pc_var2 = in_stack_0000afcc + 0x69;
        b_var24 = (in_stack_0000afd4 >> 8);
        *pc_var2 = *pc_var2 + b_var24;
        pu_var50 = (in_stack_0000afca + 2);
        u_var6 = bad1::_in(in_stack_0000afd4);
        in_stack_0000afca[1] = u_var6;
        pi_var29 = ((pu_var50 + in_stack_0000afce + 0x6f) * 0x2e6e);
        pb_var1 = (in_stack_0000afd2 + in_stack_0000afcc + 0x6c);
        b_var16 = *pb_var1;
        *pb_var1 = *pb_var1 + b_var24;
        pu_var44 = (in_stack_0000afcc + 1);
        bad1::out(*in_stack_0000afcc, in_stack_0000afd4);
        b_var23 = (in_stack_0000afd6 >> 8);
        if (*pb_var1 == 0) {
            // code_r0x10501b31:
            u_var43 = pu_var44 ^ 0x662e;
            u_var6 = bad1::_in(in_stack_0000afd4);
            *pu_var50 = u_var6;
            pu_var3 = (in_stack_0000afd2 + u_var43);
            *pu_var3 = *pu_var3;
            pu_var44 = (u_var43 ^ *in_stack_0000afd2);
            u_var6 = bad1::_in(in_stack_0000afd4);
            *(pu_var50 + 1) = u_var6;
            pu_var3 = (in_stack_0000afd2 + pu_var44);
            *pu_var3 = *pu_var3;
            pu_var51 = ((pu_var50 + 1) ^ (in_stack_0000afd2 + pu_var44));
            pu_var52 = pu_var51 + 1;
            u_var6 = bad1::_in(in_stack_0000afd4);
            *pu_var51 = u_var6;
            pu_var3 = (in_stack_0000afd2 + pu_var44);
            *pu_var3 = *pu_var3;
            pu_var52 = (pu_var52 ^ (in_stack_0000afd2 + pu_var52));
            u_var6 = bad1::_in(in_stack_0000afd4);
            *pu_var52 = u_var6;
            pu_var3 = (in_stack_0000afd2 + pu_var44);
            *pu_var3 = *pu_var3;
            u_var6 = bad1::_in(in_stack_0000afd4);
            pu_var52[1] = u_var6;
            pu_var3 = (in_stack_0000afd2 + pu_var44);
            *pu_var3 = *pu_var3;
            b_var16 = b_var15 ^ 0x32;
            pu_var53 = (pu_var52 + 3);
            u_var6 = bad1::_in(in_stack_0000afd4);
            pu_var52[2] = u_var6;
            pu_var3 = (in_stack_0000afd2 + pu_var44);
            *pu_var3 = *pu_var3;
        } else {
            pu_var4 = in_stack_0000afd2 + 0x37;
            *pu_var4 =
                *pu_var4 + CARRY1(b_var16, b_var24) * ((in_stack_0000afce & 3) - (*pu_var4 & 3));
            pc_var2 = (pu_var50 + in_stack_0000afce + 0x69);
            *pc_var2 = *pc_var2 + b_var24;
            if (*pc_var2 == '\0') {
                in_af = 9 < (b_var15 & 0xf) | in_af;
                b_var15 = b_var15 + in_af * '\x06' & 0xf;
                // goto code_r0x10501b31;
            }
            pi_var29 = ((pu_var50 + in_stack_0000afce + 0x6f) * 0x2e6e);
            pc_var2 = (in_stack_0000afd2 + pu_var50);
            *pc_var2 = *pc_var2 + b_var24;
            pb_var1 = (in_stack_0000afce + pu_var44);
            *pb_var1 = *pb_var1 ^ b_var24;
            pb_var1 = 0x6d62;
            *pb_var1 = *pb_var1 ^ b_var23;
            in_stack_0000afce = (in_stack_0000afce + 0x74) * 0x6564;
            pu_var53 = (in_stack_0000afca + 4);
            u_var22 = bad1::_in(in_stack_0000afd4);
            *pu_var50 = u_var22;
            b_var16 = in_stack_0000afd8 ^ 0x58;
            pc_var2 = (in_stack_0000afd2 + pu_var53 + 0x6e);
            *pc_var2 = *pc_var2 + b_var23;
            if (*pc_var2 != '\0') {
                u_var22 = bad1::_in(in_stack_0000afd4);
                *pu_var53 = u_var22;
                pc_var2 = (in_stack_0000afd2 + (in_stack_0000afca + 6));
                *pc_var2 = *pc_var2 + b_var24;
                pb_var1 = 0x6c66;
                *pb_var1 = *pb_var1 ^ b_var23;
                pu_var3 = (in_stack_0000afd2 + pu_var44);
                *pu_var3 = *pu_var3;
                in_stack_0000afd4 =
                    in_stack_0000afd4 & 0xff | (b_var24 ^ *(in_stack_0000afd2 + pu_var44)) << 8;
                u_var6 = bad1::_in(in_stack_0000afd4);
                in_stack_0000afca[6] = u_var6;
                pu_var3 = (in_stack_0000afd2 + pu_var44);
                *pu_var3 = *pu_var3;
                u_var43 = pu_var44 ^ (in_stack_0000afd2 + pu_var44);
                u_var6 = bad1::_in(in_stack_0000afd4);
                in_stack_0000afca[7] = u_var6;
                pu_var3 = (in_stack_0000afd2 + u_var43);
                *pu_var3 = *pu_var3;
                u_var43 = u_var43 ^ (in_stack_0000afce + u_var43);
                u_var6 = bad1::_in(in_stack_0000afd4);
                in_stack_0000afca[8] = u_var6;
                pu_var3 = (in_stack_0000afd2 + u_var43);
                *pu_var3 = *pu_var3;
                pu_var44 = (u_var43 ^ (in_stack_0000afca + 9 + in_stack_0000afce));
                u_var6 = bad1::_in(in_stack_0000afd4);
                in_stack_0000afca[9] = u_var6;
                pu_var3 = (in_stack_0000afd2 + pu_var44);
                *pu_var3 = *pu_var3;
                u_var43 = pu_var44 ^ *pu_var44;
                u_var6 = bad1::_in(in_stack_0000afd4);
                in_stack_0000afca[10] = u_var6;
                pu_var3 = (in_stack_0000afd2 + u_var43);
                *pu_var3 = *pu_var3;
                pu_var44 = (u_var43 ^ (in_stack_0000afca + 0xb));
                pu_var50 = (in_stack_0000afca + 0xc);
                u_var6 = bad1::_in(in_stack_0000afd4);
                in_stack_0000afca[0xb] = u_var6;
                pu_var3 = (in_stack_0000afd2 + pu_var44);
                *pu_var3 = *pu_var3;
                // goto code_r0x10501b31;
            }
        }
        u_var6 = bad1::_in(in_stack_0000afd4);
        *pu_var53 = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 1) = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 1) = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 3) = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 2) = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var11 = 9 < ((b_var16 ^ 5) & 0xf) | in_af;
        b_var16 = (b_var16 ^ 5) + b_var11 * '\x06' & 0xf;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var11 = 9 < b_var16 | b_var11;
        u_var43 = (b_var16 + b_var11 * '\x06' & 0xf);
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var16 = *(in_stack_0000afd2 + pu_var44);
        b_var27 = (in_stack_0000afd4 >> 8);
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 5) = u_var6;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (b_var16 < b_var27) * ((u_var43 & 3) - (*pu_var4 & 3));
        b_var16 = *(in_stack_0000afce + pu_var44);
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 3) = u_var6;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (b_var16 < b_var27) * ((u_var43 & 3) - (*pu_var4 & 3));
        b_var16 = *pu_var44;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 7) = u_var6;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (b_var16 < b_var27) * ((u_var43 & 3) - (*pu_var4 & 3));
        b_var16 = *0x662e;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 4) = u_var6;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (b_var16 < b_var27) * ((u_var43 & 3) - (*pu_var4 & 3));
        pu_var46 = *(in_stack_0000afd2 + pu_var44);
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 9) = u_var6;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (pu_var46 < pu_var44) * ((u_var43 & 3) - (*pu_var4 & 3));
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 ^ pu_var44;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + (pu_var53 + 5));
        *pu_var4 = *pu_var4 ^ pu_var44;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 5) = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + (pu_var53 + 0xb));
        *pu_var4 = *pu_var4 ^ pu_var44;
        pu_var46 = pu_var53 + 6;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *(pu_var53 + 0xb) = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var46);
        *pu_var4 = *pu_var4 ^ pu_var44;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (*0x6c66 < b_var23) * (((u_var43 ^ 0x662e) & 3) - (*pu_var4 & 3));
        pu_var4 = (in_stack_0000afce + pu_var46);
        *pu_var4 = *pu_var4 ^ pu_var44;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 ^ pu_var44;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = pu_var46;
        *pu_var4 = *pu_var4 ^ pu_var44;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = 0x2e30;
        *pu_var4 = *pu_var4 ^ pu_var44;
        pu_var52 = (pu_var53 + 0xd);
        u_var6 = bad1::_in(in_stack_0000afd4);
        *pu_var46 = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = in_stack_0000afd2;
        *pu_var4 = *pu_var4 ^ pu_var44;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 ^ pu_var52;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var52);
        *pu_var4 = *pu_var4 ^ pu_var52;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var52);
        *pu_var4 = *pu_var4 ^ pu_var52;
        pu_var4 = 0x6c66;
        *pu_var4 = *pu_var4 ^ in_stack_0000afce;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var52);
        *pu_var4 = *pu_var4 ^ pu_var52;
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var52);
        *pu_var4 = *pu_var4 ^ pu_var52;
        u_var42 = in_stack_0000afce ^ 0x6c66;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var52);
        *pu_var4 = *pu_var4 ^ pu_var52;
        b_var17 = (u_var43 ^ 0x662e);
        u_var45 = (b_var17 ^ 0x2e);
        pu_var50 = pu_var53 + 7;
        u_var6 = bad1::_in(in_stack_0000afd4);
        *pu_var52 = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        pu_var4 = (in_stack_0000afd2 + pu_var50);
        *pu_var4 = *pu_var4 ^ pu_var50;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (*0x6c66 < b_var23) * ((u_var45 & 3) - (*pu_var4 & 3));
        pu_var4 = (in_stack_0000afd2 + pu_var50);
        *pu_var4 = *pu_var4 ^ pu_var50;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (0x6c66 < u_var42) * ((u_var45 & 3) - (*pu_var4 & 3));
        b_var16 = *(in_stack_0000afd2 + pu_var44);
        u_var43 = in_stack_0000afd4 & 0xff;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var15 = *(in_stack_0000afd2 + pu_var44);
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var24 = *(in_stack_0000afd2 + pu_var44);
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (*0x6c66 < b_var23) * ((u_var45 & 3) - (*pu_var4 & 3));
        b_var14 = *(in_stack_0000afd2 + pu_var50);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var7 = *(in_stack_0000afd2 + pu_var50);
        pu_var4 = 0x6c66;
        *pu_var4 = *pu_var4 ^ u_var42;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var8 = *(u_var42 + pu_var44);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var9 = *(u_var42 + pu_var50);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var10 = *pu_var44;
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var14 = b_var27
            ^ b_var16
            ^ b_var15
            ^ b_var24
            ^ b_var14
            ^ b_var7
            ^ b_var8
            ^ b_var9
            ^ b_var10
            ^ *pu_var44;
        pu8_var28 = (pu_var53 + 0xf);
        u_var6 = bad1::_in(u_var43 | b_var14 << 8);
        *pu_var50 = u_var6;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var16 = *pu_var44;
        pu_var4 = (in_stack_0000afd2 + pu_var44);
        *pu_var4 = *pu_var4 + (*0x6c66 < b_var23) * ((u_var45 & 3) - (*pu_var4 & 3));
        b_var15 = *pu8_var28;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var24 = *pu8_var28;
        pu_var4 = 0x6c66;
        *pu_var4 = *pu_var4 ^ u_var42;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var15 = b_var14 ^ b_var16 ^ b_var15 ^ b_var24 ^ *0x2e30;
        b_var16 = bad1::_in(u_var43 | b_var15 << 8);
        *pu8_var28 = b_var16;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var15 = b_var15 ^ *in_stack_0000afd2;
        u_var43 = u_var43 | b_var15 << 8;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (in_stack_0000afd2 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var13 = b_var13 ^ *(in_stack_0000afd2 + pu_var44);
        u_var26 = in_stack_0000afd2 & 0xff;
        u_var48 = u_var26 | b_var13 << 8;
        pu_var52 = (pu_var53 + 0x11);
        u_var6 = bad1::_in(u_var43);
        *(pu_var53 + 8) = u_var6;
        pu_var3 = (u_var48 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var13 = b_var13 ^ *(u_var48 + pu_var44);
        u_var48 = u_var26 | b_var13 << 8;
        pu_var4 = (u_var48 + pu_var44);
        *pu_var4 = *pu_var4 + (*0x6c66 < b_var23) * ((u_var45 & 3) - (*pu_var4 & 3));
        b_var13 = b_var13 ^ pu_var52[u_var48];
        u_var45 = u_var26 | b_var13 << 8;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var45 + pu_var44);
        *pu_var3 = *pu_var3;
        b_var13 = b_var13 ^ pu_var52[u_var45];
        u_var45 = u_var26 | b_var13 << 8;
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (u_var45 + pu_var44);
        *pu_var3 = *pu_var3;
        u_var26 = u_var26 | (b_var13 ^ pu_var52[u_var45]) << 8;
        u_var6 = bad1::_in(u_var43);
        *pu_var52 = u_var6;
        pu_var3 = (u_var26 + pu_var44);
        *pu_var3 = *pu_var3;
        u_var45 = pu_var44 ^ (u_var26 + pu_var44);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ (u_var26 + u_var45);
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ (u_var26 + u_var45);
        u_var6 = bad1::_in(u_var43);
        *(pu_var53 + 9) = u_var6;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ (u_var26 + u_var45);
        pu_var44 = pu_var53 + 10;
        u_var6 = bad1::_in(u_var43);
        *(pu_var53 + 0x13) = u_var6;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ (u_var26 + pu_var44);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pu_var46 = (u_var45 ^ (u_var42 + pu_var44));
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + pu_var46);
        *pu_var3 = *pu_var3;
        pu_var46 = (pu_var46 ^ *pu_var46);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + pu_var46);
        *pu_var3 = *pu_var3;
        u_var45 = pu_var46 ^ *pu_var46;
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ *pu_var44;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ *pu_var44;
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var45 = u_var45 ^ 0x2e30;
        u_var6 = bad1::_in(u_var43);
        *pu_var44 = u_var6;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var43 = (pu_var53 + 0x15) ^ (u_var26 + u_var45);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var43 = u_var43 ^ (u_var26 + u_var43);
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var43 = u_var43 ^ (u_var26 + u_var43);
        b_var23 = b_var23 ^ *0x6c66;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var43 = u_var43 ^ (u_var26 + u_var43);
        pu_var4 = (u_var26 + u_var45);
        *pu_var4 = *pu_var4 + (*0x6c66 < b_var23) * (((b_var17 ^ 0x2e) & 3) - (*pu_var4 & 3));
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pb_var1 = 0x6c66;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pu_var4 = 0x6c66;
        *pu_var4 = *pu_var4 ^ u_var42;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        u_var42 = u_var42 ^ 0x6c66;
        pu_var3 = (u_var26 + u_var45);
        *pu_var3 = *pu_var3;
        pu_var4 = (u_var26 + u_var45);
        *pu_var4 = *pu_var4 ^ u_var45;
        pc_var2 = (u_var26 + u_var43);
        *pc_var2 = *pc_var2 + b_var15;
        pb_var1 = (u_var42 + u_var43);
        *pb_var1 = *pb_var1 ^ b_var15;
        pc_var2 = (u_var26 + u_var43);
        *pc_var2 = *pc_var2 + b_var15;
        pb_var1 = (u_var42 + u_var43);
        *pb_var1 = *pb_var1 ^ b_var15;
        i_var54 = *pi_var29;
        iVar47 = pi_var29[1];
        i_var21 = pi_var29[2];
        pu8_var28 = pi_var29[4];
        u_var18 = pi_var29[7];
        pb_var1 = pu8_var28 + i_var54;
        b_var23 = (pi_var29[5] >> 8);
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = (i_var21 + i_var54);
        *pb_var1 = *pb_var1 ^ b_var23;
        u_var30 = (pi_var29 + 8) ^ (pu8_var28 + i_var54 + 0x2e);
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = pu8_var28;
        *pb_var1 = *pb_var1 ^ b_var23;
        pb_var1 = pu8_var28 + i_var54 + 0x2e;
        b_var15 = (u_var18 >> 8);
        *pb_var1 = *pb_var1 ^ b_var15;
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        b_var23 = b_var23 ^ *0x6130;
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        b_var23 = b_var23 ^ *pu8_var28;
        pb_var1 = pu8_var28 + i_var54 + 0x2e;
        *pb_var1 = *pb_var1 ^ b_var15;
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = pu8_var28 + iVar47;
        b_var24 = (pu8_var28 >> 8);
        *pb_var1 = *pb_var1 ^ b_var24;
        b_var16 = pu8_var28[i_var54 + 0x2e];
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 ^ b_var24;
        pb_var1 = pu8_var28 + i_var54 + 0x2e;
        *pb_var1 = *pb_var1 ^ b_var15 ^ b_var16;
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = (i_var21 + i_var54);
        *pb_var1 = *pb_var1 ^ b_var23;
        pb_var1 = pu8_var28 + i_var54;
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = (i_var21 + i_var54);
        *pb_var1 = *pb_var1 ^ b_var23;
        pu_var55 = u_var30;
        pu_var46 = *(u_var30 + 2);
        i_var21 = (u_var30 + 4);
        u_var45 = (u_var30 + 8);
        u_var26 = (u_var30 + 10);
        u_var22 = (u_var30 + 0xc);
        u_var19 = (u_var30 + 0xe);
        pc_var2 = (u_var45 + pu_var55);
        b_var13 = (u_var26 >> 8);
        *pc_var2 = *pc_var2 + b_var13;
        pb_var1 = (i_var21 + pu_var55);
        *pb_var1 = *pb_var1 ^ b_var13;
        pc_var2 = (u_var45 + pu_var55);
        *pc_var2 = *pc_var2 + b_var13;
        pu_var4 = (u_var45 + pu_var55);
        *pu_var4 = *pu_var4 ^ pu_var46;
        pb_var1 = 0x6d62;
        b_var24 = (u_var22 >> 8);
        *pb_var1 = *pb_var1 ^ b_var24;
        pu_var4 = (u_var45 + pu_var55);
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = (u_var45 + pu_var55);
        *pu_var4 = *pu_var4 ^ pu_var55;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = (u_var45 + pu_var46);
        *pb_var1 = *pb_var1 ^ b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = (u_var45 + pu_var55);
        *pb_var1 = *pb_var1 ^ b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = (i_var21 + pu_var46);
        *pb_var1 = *pb_var1 ^ b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = (i_var21 + pu_var55);
        *pb_var1 = *pb_var1 ^ b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var4 = pu_var46;
        *pu_var4 = *pu_var4 ^ b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 ^ b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = 0x622e;
        *pb_var1 = *pb_var1 ^ b_var13;
        pu_var44 = pu_var55 + 1;
        u_var22 = bad1::_in(u_var26);
        *pu_var55 = u_var22;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pb_var1 = (u_var45 + pu_var46);
        b_var16 = (u_var45 >> 8);
        *pb_var1 = *pb_var1 ^ b_var16;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pb_var1 = (u_var45 + pu_var44);
        *pb_var1 = *pb_var1 ^ b_var16;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = (u_var45 + pu_var46);
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = (u_var45 + pu_var44);
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = (i_var21 + pu_var46);
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = (i_var21 + pu_var44);
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = pu_var46;
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + b_var13;
        pu_var4 = 0x622e;
        *pu_var4 = *pu_var4 ^ pu_var46;
        pu_var50 = pu_var55 + 2;
        u_var43 = bad1::_in(u_var26);
        *pu_var44 = u_var43;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var4 = (u_var45 + pu_var46);
        *pu_var4 = *pu_var4 ^ pu_var50;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var4 = (u_var45 + pu_var50);
        *pu_var4 = *pu_var4 ^ pu_var50;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *(u_var45 + pu_var46);
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *(u_var45 + pu_var50);
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *(i_var21 + pu_var46);
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *(i_var21 + pu_var50);
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *pu_var46;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *pu_var50;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var13 = b_var13 ^ *0x622e;
        u_var26 = u_var26 & 0xff | b_var13 << 8;
        pu_var53 = pu_var55 + 3;
        u_var22 = bad1::_in(u_var26);
        *pu_var50 = u_var22;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        u_var45 = u_var45 & 0xff | (b_var16 ^ *(u_var45 + pu_var46)) << 8;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        u_var48 = pu_var46 ^ (u_var45 + pu_var53);
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        u_var43 = (i_var21 + u_var48);
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = 0x6d62;
        *pb_var1 = *pb_var1 ^ b_var24;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        b_var16 = *0x6d62;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var53;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var50 = pu_var55 + 4;
        u_var22 = bad1::_in(u_var26);
        *pu_var53 = u_var22;
        b_var23 = u_var19 ^ 5;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var15 = (9 < (b_var23 & 0xf)) | (9 < ((u_var18 ^ 0x61) & 0xf)) | b_var11;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var23 = b_var23 + b_var15 * '\x06' & 0xf ^ 0x36;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var15 = 9 < (b_var23 & 0xf) | b_var15;
        pb_var1 = 0x6d62;
        *pb_var1 = *pb_var1 ^ b_var24 ^ b_var16;
        b_var16 = b_var23 + b_var15 * '\x06' & 0xf ^ 0x37;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var15 = 9 < (b_var16 & 0xf) | b_var15;
        b_var16 = b_var16 + b_var15 * '\x06' & 0xf ^ 0x37;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var15 = 9 < (b_var16 & 0xf) | b_var15;
        b_var16 = b_var16 + b_var15 * '\x06' & 0xf ^ 0x2e;
        pu_var3 = pu_var50;
        *pu_var3 = *pu_var3 + b_var13;
        b_var15 = 9 < (b_var16 & 0xf) | b_var15;
        pu_var55 = pu_var55 + 5;
        u_var22 = bad1::_in(u_var26);
        *pu_var50 = u_var22;
        b_var16 = b_var16 + b_var15 * '\x06' & 0xf ^ 0x19;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        b_var15 = 9 < (b_var16 & 0xf) | b_var15;
        b_var16 = b_var16 + b_var15 * '\x06' & 0xf;
        b_var15 = 9 < b_var16 | b_var15;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pu_var3 = pu_var55;
        *pu_var3 = *pu_var3 + b_var13;
        pb_var1 = (u_var45 + (u_var48 ^ u_var43));
        *pb_var1 = *pb_var1 ^ b_var13;
        pc_var56 = *(u_var30 + 0x10);
        i_var21 = (u_var30 + 0x18);
        pc_var2 = pc_var56;
        b_var23 = (*(u_var30 + 0x1a) >> 8);
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = (pc_var56 + i_var21);
        *pb_var1 = *pb_var1 ^ b_var23;
        iVar47 = (u_var30 + 0x22);
        i_var21 = (u_var30 + 0x24);
        pc_var2 = *(u_var30 + 0x20);
        b_var23 = (*(u_var30 + 0x2a) >> 8);
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = (i_var21 + iVar47);
        *pb_var1 = *pb_var1 ^ b_var23;
        pc_var56 = *(u_var30 + 0x30);
        i_var21 = (u_var30 + 0x34);
        pc_var2 = pc_var56;
        b_var23 = (*(u_var30 + 0x3a) >> 8);
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = (pc_var56 + i_var21);
        *pb_var1 = *pb_var1 ^ b_var23;
        pu8_var28 = (u_var30 + 0x42);
        pc_var2 = *(u_var30 + 0x40);
        b_var23 = (*(u_var30 + 0x4a) >> 8);
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = pu8_var28;
        *pb_var1 = *pb_var1 ^ b_var23;
        pu8_var28 = (u_var30 + 0x50);
        pb_var1 = pu8_var28;
        b_var23 = (*(u_var30 + 0x5a) >> 8);
        *pb_var1 = *pb_var1 + b_var23;
        pb_var1 = pu8_var28;
        *pb_var1 = *pb_var1 ^ b_var23;
        pc_var56 = *(u_var30 + 0x60);
        pu8_var28 = (u_var30 + 0x68);
        pc_var2 = pc_var56;
        b_var23 = (*(u_var30 + 0x6a) >> 8);
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = 0x2e61;
        *pb_var1 = *pb_var1 ^ b_var23;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + b_var23;
        pb_var1 = pu8_var28;
        *pb_var1 = *pb_var1 ^ b_var23;
        iVar47 = (u_var30 + 0x72);
        i_var21 = (u_var30 + 0x78);
        pc_var2 = *(u_var30 + 0x70);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x7a) >> 8);
        pb_var1 = (i_var21 + iVar47);
        *pb_var1 = *pb_var1 ^ (i_var21 >> 8);
        pc_var56 = *(u_var30 + 0x80);
        i_var21 = (u_var30 + 0x88);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x8a) >> 8);
        pb_var1 = (pc_var56 + i_var21);
        *pb_var1 = *pb_var1 ^ (i_var21 >> 8);
        u_var43 = (u_var30 + 0x92);
        i_var21 = (u_var30 + 0x98);
        pc_var2 = *(u_var30 + 0x90);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x9a) >> 8);
        pu_var4 = (i_var21 + u_var43);
        *pu_var4 = *pu_var4 ^ u_var43;
        pc_var56 = *(u_var30 + 0xa0);
        u_var43 = (u_var30 + 0xa2);
        i_var21 = (u_var30 + 0xa8);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + (*(u_var30 + 0xaa) >> 8);
        pu_var4 = (pc_var56 + i_var21);
        *pu_var4 = *pu_var4 ^ u_var43;
        u_var43 = (u_var30 + 0xb2);
        i_var21 = (u_var30 + 0xb4);
        pc_var2 = *(u_var30 + 0xb0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0xba) >> 8);
        pu_var4 = (i_var21 + u_var43);
        *pu_var4 = *pu_var4 ^ u_var43;
        pc_var56 = *(u_var30 + 0xc0);
        u_var43 = (u_var30 + 0xc2);
        i_var21 = (u_var30 + 0xc4);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + (*(u_var30 + 0xca) >> 8);
        pu_var4 = (pc_var56 + i_var21);
        *pu_var4 = *pu_var4 ^ u_var43;
        pu_var44 = *(u_var30 + 0xd2);
        pc_var2 = *(u_var30 + 0xd0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0xda) >> 8);
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 ^ pu_var44;
        pu_var44 = *(u_var30 + 0xe0);
        u_var43 = (u_var30 + 0xe2);
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 + (*(u_var30 + 0xea) >> 8);
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 ^ u_var43;
        pc_var56 = *(u_var30 + 0xf0);
        u_var43 = (u_var30 + 0xf2);
        pu_var44 = *(u_var30 + 0xf8);
        pc_var2 = pc_var56;
        c_var25 = (*(u_var30 + 0xfa) >> 8);
        *pc_var2 = *pc_var2 + c_var25;
        pu_var4 = 0x2e61;
        *pu_var4 = *pu_var4 ^ u_var43;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pu_var4 = pu_var44;
        *pu_var4 = *pu_var4 ^ u_var43;
        pc_var56 = *(u_var30 + 0x100);
        iVar47 = (u_var30 + 0x102);
        i_var21 = (u_var30 + 0x108);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x10a) >> 8);
        pu_var4 = (i_var21 + iVar47);
        *pu_var4 = *pu_var4 ^ pc_var56;
        pc_var56 = *(u_var30 + 0x110);
        i_var21 = (u_var30 + 0x118);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x11a) >> 8);
        pu_var4 = (pc_var56 + i_var21);
        *pu_var4 = *pu_var4 ^ pc_var56;
        pc_var2 = *(u_var30 + 0x120);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x12a) >> 8);
        pc_var2 = *(u_var30 + 0x130);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x13a) >> 8);
        pc_var2 = *(u_var30 + 0x140);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x14a) >> 8);
        pc_var2 = *(u_var30 + 0x150);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x15a) >> 8);
        pc_var2 = *(u_var30 + 0x160);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x16a) >> 8);
        pc_var2 = *(u_var30 + 0x170);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x17a) >> 8);
        pc_var56 = *(u_var30 + 0x180);
        pc_var2 = pc_var56;
        b_var23 = (*(u_var30 + 0x18a) >> 8);
        *pc_var2 = *pc_var2 + b_var23;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + (b_var23 ^ *0x2e61);
        pc_var2 = *(u_var30 + 400);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x19a) >> 8);
        pc_var2 = *(u_var30 + 0x1a0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x1aa) >> 8);
        pc_var2 = *(u_var30 + 0x1b0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x1ba) >> 8);
        pc_var2 = *(u_var30 + 0x1c0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x1ca) >> 8);
        pc_var2 = *(u_var30 + 0x1d0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x1da) >> 8);
        pc_var2 = *(u_var30 + 0x1e0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x1ea) >> 8);
        pc_var2 = *(u_var30 + 0x1f0);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x1fa) >> 8);
        pc_var2 = *(u_var30 + 0x200);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x20a) >> 8);
        pc_var2 = *(u_var30 + 0x210);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x21a) >> 8);
        pc_var2 = *(u_var30 + 0x220);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x22a) >> 8);
        pc_var2 = *(u_var30 + 0x230);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x23a) >> 8);
        pc_var2 = *(u_var30 + 0x240);
        *pc_var2 = *pc_var2 + (*(u_var30 + 0x24a) >> 8);
        pc_var56 = *(u_var30 + 0x250);
        i_var21 = (u_var30 + 600);
        u_var22 = (u_var30 + 0x25e);
        pc_var2 = pc_var56;
        c_var25 = (*(u_var30 + 0x25a) >> 8);
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pb_var1 = (pc_var56 + i_var21 + 0x2e);
        *pb_var1 = *pb_var1 ^ (u_var22 >> 8);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pu_var4 = (pc_var56 + i_var21 + 0x2e);
        *pu_var4 = *pu_var4 ^ u_var30 + 0x260;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        ppc_var31 = (u_var30 + 0x260 ^ (pc_var56 + i_var21 + 0x2e));
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        u_var22 = ppc_var31[7];
        pc_var2 = *ppc_var31;
        *pc_var2 = *pc_var2 + (ppc_var31[5] >> 8);
        pc_var56 = ppc_var31[8];
        i_var21 = ppc_var31[0xc];
        u_var18 = ppc_var31[0xf];
        pc_var2 = pc_var56;
        c_var25 = (ppc_var31[0xd] >> 8);
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        b_var23 = u_var18;
        b_var16 = (9 < (b_var23 & 0xf))
            | (9 < (u_var22 & 0xf))
            | (9 < (b_var16 + b_var15 * '\x06' & 0xf))
            | b_var15;
        b_var15 = b_var23 + b_var16 * '\x06' & 0xf;
        pb_var1 = (pc_var56 + i_var21 + 0x2e);
        *pb_var1 = *pb_var1 ^ (u_var18 >> 8) + b_var16;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        b_var16 = 9 < b_var15 | b_var16;
        b_var15 = b_var15 + b_var16 * '\x06' & 0xf;
        pu_var4 = (pc_var56 + i_var21 + 0x2e);
        *pu_var4 = *pu_var4 ^ (ppc_var31 + 0x10);
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        b_var16 = 9 < b_var15 | b_var16;
        b_var15 = b_var15 + b_var16 * '\x06' & 0xf;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        b_var16 = 9 < b_var15 | b_var16;
        b_var15 = b_var15 + b_var16 * '\x06' & 0xf;
        ppc_var32 = ((ppc_var31 + 0x10) ^ (pc_var56 + i_var21 + 0x2e));
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        b_var16 = 9 < b_var15 | b_var16;
        b_var15 = b_var15 + b_var16 * '\x06' & 0xf ^ 0x61;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        b_var16 = 9 < (b_var15 & 0xf) | b_var16;
        pc_var2 = pc_var56;
        *pc_var2 = *pc_var2 + c_var25;
        u_var22 = ppc_var32[7];
        pc_var2 = *ppc_var32;
        *pc_var2 = *pc_var2 + (ppc_var32[5] >> 8);
        b_var23 = u_var22;
        b_var16 = (9 < (b_var23 & 0xf))
            | (9 < ((b_var15 + b_var16 * '\x06' & 0xf ^ 0x61) & 0xf))
            | b_var16;
        pi_var57 = ppc_var32[8];
        pu_var52 = ppc_var32[9];
        u_var48 = ppc_var32[10];
        pu8_var28 = ppc_var32[0xc];
        u_var45 = ppc_var32[0xd];
        u_var26 = ppc_var32[0xe];
        u_var22 = ppc_var32[0xf];
        ppi_var36 = (ppc_var32 + 0x10);
        ppi_var38 = (ppc_var32 + 0x10);
        ppi_var35 = (ppc_var32 + 0x10);
        ppi_var34 = (ppc_var32 + 0x10);
        pi_var5 = pi_var57;
        b_var24 = (u_var45 >> 8);
        *pi_var5 = *pi_var5 + b_var24;
        b_var15 = u_var22;
        b_var16 = (9 < (b_var15 & 0xf)) | (9 < (b_var23 + b_var16 * '\x06' & 0xf)) | b_var16;
        b_var23 = b_var15 + b_var16 * '\x06' & 0xf;
        pi_var5 = pi_var57;
        *pi_var5 = *pi_var5 + b_var24;
        b_var15 = 9 < b_var23 | b_var16;
        u_var43 = CONCAT11(
            (u_var22 >> 8) + b_var16 + b_var15,
            b_var23 + b_var15 * '\x06',
        ) & 0xff0f;
        pb_var1 = (pu8_var28 + pu_var52 + 0x6c);
        b_var60 = CARRY1(*pb_var1, b_var24);
        *pb_var1 = *pb_var1 + b_var24;
        b_var61 = *pb_var1 == 0;
        pi_var49 = (pu_var52 + 1);
        bad1::out(*pu_var52, u_var45);
        if (b_var61) {
            // code_r0x10502336:
            pi_var5 = pi_var57;
            pi_var57 = (pi_var57 + 1);
            u_var6 = bad1::_in(u_var45);
            *pi_var5 = u_var6;
            if (b_var60 || b_var61) {
                // code_r0x1050239f:
                ppi_var38 = ppi_var34;
                if (b_var60) {}
                // goto code_r0x105023a1;
                // code_r0x10502410:
                pb_var1 = (pi_var57 + u_var48 + 0x6d);
                b_var16 = *pb_var1;
                b_var23 = (u_var45 >> 8);
                *pb_var1 = *pb_var1 + b_var23;
                pi_var5 = pi_var57;
                pi_var57 = (pi_var57 + 1);
                u_var6 = bad1::_in(u_var45);
                *pi_var5 = u_var6;
                if (!CARRY1(b_var16, b_var23)) {
                    pc_var2 = (u_var48 + pi_var49);
                    *pc_var2 = *pc_var2 + b_var23;
                    u_var45 = u_var45 & 0xff | (b_var23 ^ *pu8_var28) << 8;
                    // goto code_r0x10502495;
                }
                // code_r0x10502418:
                i_var21 = bad1::_in(u_var45);
                *pi_var57 = i_var21;
                pb_var1 = (u_var48 + (pi_var57 + 1));
                *pb_var1 = *pb_var1 ^ (u_var45 >> 8);
                pc_var2 = (pi_var57 + 0x37);
                *pc_var2 = *pc_var2 + (u_var26 >> 8);
                pi_var58 = pi_var57 + 2;
                i_var21 = bad1::_in(u_var45);
                pi_var57[1] = i_var21;
                // code_r0x1050242a:
                pi_var5 = pi_var58;
                *pi_var5 = *pi_var5 ^ (u_var45 >> 8);
                pc_var2 = (pi_var58 + 0x35);
                *pc_var2 = *pc_var2 + (u_var26 >> 8);
                b_var15 = 9 < (u_var43 & 0xf) | b_var15;
                u_var43 = CONCAT11((u_var43 >> 8) + b_var15, u_var43 + b_var15 * '\x06') & 0xff0f;
                // code_r0x10502433:
                pc_var2 = (pi_var58 + 0x35);
                *pc_var2 = *pc_var2 + (u_var26 >> 8);
                ppi_var36 = ppi_var34;
                pi_var57 = pi_var58;
                // code_r0x10502436:
                pb_var1 = pu8_var28 + pi_var49;
                b_var16 = u_var43;
                b_var60 = *pb_var1 < b_var16;
                b_var61 = *pb_var1 == b_var16;
                if (b_var61) {}
                // goto code_r0x105024a5;
                if (b_var61) {}
                // goto code_r0x10502443;
            } else {
                // code_r0x1050233a:
                u_var6 = bad1::_in(u_var45);
                *pi_var57 = u_var6;
                pb_var1 = (pi_var49 + 0x65);
                b_var16 = (u_var26 >> 8);
                b_var60 = CARRY1(*pb_var1, b_var16);
                *pb_var1 = *pb_var1 + b_var16;
                ppi_var38 = ppi_var35;
                if (b_var60 || *pb_var1 == 0) {
                    // code_r0x105023a9:
                    pu_var4 = (pi_var49 + u_var48 + 0x65);
                    *pu_var4 = *pu_var4 + b_var60 * ((pi_var49 & 3) - (*pu_var4 & 3));
                    pu_var52 = (ppi_var38 + 2);
                    u_var48 = (ppi_var38 + 4);
                    pu8_var28 = (ppi_var38 + 8);
                    u_var45 = (ppi_var38 + 10);
                    u_var26 = (ppi_var38 + 0xc);
                    u_var43 = (ppi_var38 + 0xe);
                    ppi_var35 = (ppi_var38 + 0x10);
                    ppi_var34 = (ppi_var38 + 0x10);
                    pi_var58 = *ppi_var38 + 1;
                    u_var22 = bad1::_in(u_var45);
                    **ppi_var38 = u_var22;
                    b_var15 = 9 < (u_var43 & 0xf) | b_var15;
                    b_var16 = u_var43 + b_var15 * '\x06';
                    b_var23 = (u_var43 >> 8);
                    u_var43 = u_var43 & 0xff00
                        | (b_var16
                            + (0x90 < (b_var16 & 0xf0) | b_var60 | b_var15 * (0xf9 < b_var16))
                                * '`');
                    pi_var49 = (pu_var52 + 1);
                    bad1::out(*pu_var52, u_var45);
                    pb_var1 = (pi_var58 + u_var48 + 0x68);
                    b_var16 = *pb_var1;
                    *pb_var1 = *pb_var1 + b_var23;
                    if (CARRY1(b_var16, b_var23)) {}
                    // goto code_r0x1050242a;
                    // code_r0x105023b7:
                    pb_var1 = (pu8_var28 + pi_var58 + 0x6e);
                    b_var16 = (u_var43 >> 8);
                    b_var60 = CARRY1(*pb_var1, b_var16);
                    *pb_var1 = *pb_var1 + b_var16;
                    b_var61 = *pb_var1 == 0;
                    ppi_var36 = ppi_var35;
                    pi_var57 = pi_var58;
                    if (b_var60) {}
                    // goto code_r0x10502436;
                    pi_var59 = pi_var58 + 1;
                    i_var21 = bad1::_in(u_var45);
                    *pi_var58 = i_var21;
                    bad1::out(*pi_var49, u_var45);
                    // code_r0x105023bf:
                    u_var6 = bad1::_in(u_var45);
                    *pi_var59 = u_var6;
                    pi_var58 = *ppi_var35;
                    pi_var49 = ppi_var35[1];
                    u_var48 = ppi_var35[2];
                    pu8_var28 = ppi_var35[4];
                    u_var45 = ppi_var35[5];
                    u_var26 = ppi_var35[6];
                    u_var43 = ppi_var35[7];
                    ppi_var34 = ppi_var35 + 8;
                    ppi_var40 = ppi_var35 + 8;
                    if (!b_var61) {}
                    // goto code_r0x10502433;
                    pu_var4 = (pu8_var28 + pi_var49);
                    i_var21 = (u_var48 & 3) - (*pu_var4 & 3);
                    b_var61 = 0 < i_var21;
                    *pu_var4 = *pu_var4 + b_var60 * i_var21;
                    if (!b_var60) {
                        u_var6 = bad1::_in(u_var45);
                        *pi_var58 = u_var6;
                        pi_var57 = ppi_var35[8];
                        pi_var49 = ppi_var35[9];
                        u_var48 = ppi_var35[10];
                        pu8_var28 = ppi_var35[0xc];
                        u_var45 = ppi_var35[0xd];
                        u_var26 = ppi_var35[0xe];
                        ppi_var36 = ppi_var35 + 0x10;
                        // code_r0x105023cf:
                        pu_var50 = (pi_var49 + 1);
                        bad1::out(*pi_var49, u_var45);
                        u_var43 = (pu8_var28 + pu_var50) * 0x6f62;
                        pi_var49 = (pi_var49 + 3);
                        bad1::out(*pu_var50, u_var45);
                        if (b_var61) {
                            pb_var1 = (pi_var49 + u_var48 + 0x6f);
                            b_var16 = (u_var45 >> 8);
                            b_var60 = CARRY1(*pb_var1, b_var16);
                            *pb_var1 = *pb_var1 + b_var16;
                            // goto code_r0x10502445;
                        }
                        ppi_var35 = ppi_var36;
                        if (b_var61) {}
                        // goto code_r0x105023de;
                        // goto code_r0x10502450;
                    }
                    pc_var2 = (u_var48 + 0x69);
                    *pc_var2 = *pc_var2 + (u_var43 >> 8);
                    // code_r0x10502401:
                    pb_var1 = (u_var48 + 0x69);
                    b_var16 = *pb_var1;
                    b_var23 = (u_var43 >> 8);
                    *pb_var1 = *pb_var1 + b_var23;
                    if (!CARRY1(b_var16, b_var23)) {
                        pu_var52 = ppi_var40[1];
                        u_var48 = ppi_var40[2];
                        pu8_var28 = ppi_var40[4];
                        u_var45 = ppi_var40[5];
                        u_var26 = ppi_var40[6];
                        u_var43 = ppi_var40[7];
                        ppi_var36 = ppi_var40 + 8;
                        pi_var57 = (*ppi_var40 + 1);
                        u_var6 = bad1::_in(u_var45);
                        **ppi_var40 = u_var6;
                        pb_var1 = pu_var52 + u_var48 + 0x65;
                        b_var16 = (u_var45 >> 8);
                        b_var60 = CARRY1(*pb_var1, b_var16);
                        *pb_var1 = *pb_var1 + b_var16;
                        b_var16 = *pb_var1;
                        bad1::out(*pu_var52, u_var45);
                        pi_var58 = (pu_var52 + 1);
                        // goto code_r0x1050240e;
                    }
                    // code_r0x10502469:
                    pu_var4 = (pu8_var28 + pi_var49);
                    *pu_var4 = *pu_var4 ^ u_var43;
                    // code_r0x1050246e:
                    ppi_var41 = (ppi_var40 + 2);
                    u_var43 = u_var43 & 0xff00 | (u_var43 ^ pu8_var28[pi_var49]);
                    // goto code_r0x10502474;
                }
                u_var6 = bad1::_in(u_var45);
                *(pi_var57 + 1) = u_var6;
                b_var16 = b_var16 ^ *0x6d62;
                u_var26 = u_var26 & 0xff | b_var16 << 8;
                pi_var58 = (pi_var57 + 3);
                u_var6 = bad1::_in(u_var45);
                *(pi_var57 + 1) = u_var6;
                if (b_var16 == 0) {}
                // goto code_r0x105023b7;
                u_var6 = bad1::_in(u_var45);
                *pi_var58 = u_var6;
                b_var60 = false;
                u_var48 = u_var48 ^ 0x6d62;
                b_var61 = u_var48 == 0;
                pi_var59 = (pi_var57 + 5);
                u_var6 = bad1::_in(u_var45);
                *(pi_var57 + 2) = u_var6;
                if (b_var61) {}
                // goto code_r0x105023bf;
                pi_var58 = pi_var57 + 3;
                u_var6 = bad1::_in(u_var45);
                *pi_var59 = u_var6;
                u_var43 = u_var43 ^ 0x2e;
                // code_r0x10502360:
                pb_var1 = (pi_var58 + 0x61);
                b_var16 = (u_var26 >> 8);
                b_var60 = CARRY1(*pb_var1, b_var16);
                *pb_var1 = *pb_var1 + b_var16;
                b_var16 = *pb_var1;
                if (!b_var60) {
                    pi_var57 = pi_var58 + 1;
                    i_var21 = bad1::_in(u_var45);
                    *pi_var58 = i_var21;
                    lVar12 = *pi_var49 * 0x7730;
                    ppi_var36 = lVar12;
                    if (ppi_var36 == lVar12) {
                        pc_var2 = (pi_var49 + u_var48 + 0x6f);
                        *pc_var2 = *pc_var2 + (u_var43 >> 8);
                        b_var61 = *pc_var2 == '\0';
                        // goto code_r0x105023cf;
                    }
                    i_var21 = bad1::_in(u_var45);
                    *pi_var57 = i_var21;
                    lVar12 = *pi_var49 * 0x6c61;
                    i_var37 = lVar12;
                    b_var60 = i_var37 != lVar12;
                    u_var42 = (i_var37 + 0xe);
                    if (!b_var60) {
                        u_var6 = bad1::_in((i_var37 + 0x1a));
                        *(i_var37 + 0x10) = u_var6;
                        pi_var57 = (i_var37 + 0x20);
                        pi_var49 = (i_var37 + 0x22);
                        u_var48 = (i_var37 + 0x24);
                        pu8_var28 = (i_var37 + 0x28);
                        u_var45 = (i_var37 + 0x2a);
                        u_var26 = (i_var37 + 0x2c);
                        u_var43 = (i_var37 + 0x2e);
                        if (u_var42 < 0x6d) {
                            pc_var2 = (pi_var49 + u_var48 + 0x6f);
                            *pc_var2 = *pc_var2 + (u_var45 >> 8);
                            b_var61 = *pc_var2 == '\0';
                        } else {
                            b_var60 = 99 < u_var43;
                            u_var6 = bad1::_in((i_var37 + 0x3a));
                            *(i_var37 + 0x30) = u_var6;
                            pi_var57 = (i_var37 + 0x40);
                            pi_var49 = (i_var37 + 0x42);
                            u_var48 = (i_var37 + 0x44);
                            pu8_var28 = (i_var37 + 0x48);
                            u_var45 = (i_var37 + 0x4a);
                            u_var26 = (i_var37 + 0x4c);
                            u_var43 = (i_var37 + 0x4e);
                            ppi_var40 = (i_var37 + 0x50);
                            ppi_var36 = (i_var37 + 0x50);
                            if (b_var60) {
                                b_var16 = u_var43 ^ pu8_var28[pi_var49];
                                u_var43 = u_var43 & 0xff00 | b_var16;
                                if (b_var16 < '\0') {}
                                // goto code_r0x10502401;
                                pi_var5 = pi_var57;
                                pi_var57 = (pi_var57 + 1);
                                u_var6 = bad1::_in(u_var45);
                                *pi_var5 = u_var6;
                                // goto code_r0x10502392;
                            }
                            pc_var2 = (pu8_var28 + pi_var49 + 0x6c);
                            *pc_var2 = *pc_var2 + (u_var45 >> 8);
                            b_var61 = *pc_var2 == '\0';
                        }
                        // goto code_r0x105023f4;
                    }
                    pu_var52 = (i_var37 + 0x10);
                    u_var22 = (i_var37 + 0x1a);
                    ppi_var39 = (i_var37 + 0x20);
                    ppi_var36 = (i_var37 + 0x20);
                    if (!b_var60 && b_var16 != 0) {}
                    // goto code_r0x105023ec;
                    // goto code_r0x1050244d;
                }
                pi_var57 = (pi_var58 + 1);
                u_var6 = bad1::_in(u_var45);
                *pi_var58 = u_var6;
                if (!b_var60) {}
                // goto code_r0x1050244f;
                if (b_var16 != 0) {}
                // goto code_r0x10502450;
                // code_r0x105023de:
                bad1::out(*pi_var49, u_var45);
                pb_var1 = (pi_var57 + 0x61);
                b_var16 = *pb_var1;
                b_var23 = (u_var26 >> 8);
                *pb_var1 = *pb_var1 + b_var23;
                pu_var4 = (pu8_var28 + pi_var49 + 0x78);
                *pu_var4 = *pu_var4 + CARRY1(b_var16, b_var23) * ((u_var48 & 3) - (*pu_var4 & 3));
                (ppi_var35 + -2) = 0x7269;
                pb_var1 = (pu8_var28 + pi_var57 + 0x76);
                b_var16 = (u_var43 >> 8);
                b_var60 = CARRY1(*pb_var1, b_var16);
                *pb_var1 = *pb_var1 + b_var16;
                pu_var52 = (ppi_var35 + -2);
                u_var22 = (ppi_var35 + 8);
                ppi_var39 = (ppi_var35 + 0xe);
                // code_r0x105023ec:
                u_var6 = bad1::_in(u_var22);
                *pu_var52 = u_var6;
                pi_var57 = *ppi_var39;
                u_var48 = ppi_var39[2];
                pu8_var28 = ppi_var39[4];
                u_var45 = ppi_var39[5];
                u_var26 = ppi_var39[6];
                u_var43 = ppi_var39[7];
                ppi_var35 = ppi_var39 + 8;
                pi_var49 = (ppi_var39[1] + 1);
                bad1::out(*ppi_var39[1], u_var45);
                pu_var4 = (pu8_var28 + pi_var49);
                i_var21 = (u_var48 & 3) - (*pu_var4 & 3);
                b_var61 = 0 < i_var21;
                *pu_var4 = *pu_var4 + b_var60 * i_var21;
                if (b_var60) {}
                // goto code_r0x10502465;
                // code_r0x105023f4:
                lVar12 = *pi_var57 * 0x6c70;
                ppi_var40 = lVar12;
                pi_var5 = pi_var49;
                pi_var49 = (pi_var49 + 1);
                bad1::out(*pi_var5, u_var45);
                if (!b_var61) {
                    if (ppi_var40 == lVar12) {}
                    // goto code_r0x10502401;
                    b_var15 = 9 < (u_var43 & 0xf) | b_var15;
                    u_var43 =
                        CONCAT11((u_var43 >> 8) + b_var15, u_var43 + b_var15 * '\x06') & 0xff0f;
                    // goto code_r0x1050246e;
                }
                // code_r0x10502477:
                pi_var5 = pi_var49;
                pi_var49 = pi_var49 + 1;
                bad1::out(*pi_var5, u_var45);
                if (!b_var61) {}
                // goto code_r0x105024f2;
                (ppi_var40 + -2) = 0x706c;
                (ppi_var40 + -4) = 0x706c;
                pb_var1 = pu8_var28 + pi_var49;
                b_var16 = u_var43;
                *pb_var1 = *pb_var1 + b_var16;
                u_var26 = u_var26 & 0xff | ((u_var26 >> 8) ^ *0x6d62) << 8;
                b_var23 = (u_var45 >> 8) ^ *pu8_var28 ^ *0x2e38;
                u_var45 = u_var45 & 0xff | b_var23 << 8;
                pc_var2 = (u_var48 + pi_var49);
                *pc_var2 = *pc_var2 + b_var23;
                b_var15 = 9 < (b_var16 & 0xf) | b_var15;
                u_var43 = CONCAT11((u_var43 >> 8) + b_var15, b_var16 + b_var15 * '\x06') & 0xff0f;
                // code_r0x10502495:
                pc_var2 = (u_var48 + pi_var49);
                *pc_var2 = *pc_var2 + (u_var45 >> 8);
                pb_var1 = 0x6d62;
                *pb_var1 = *pb_var1 ^ (u_var26 >> 8);
                pb_var1 = pu8_var28 + pi_var49;
                *pb_var1 = *pb_var1 + u_var43;
                // code_r0x105024a5:
                pc_var2 = (u_var48 + pi_var49);
                *pc_var2 = *pc_var2 + (u_var45 >> 8);
                b_var15 = 9 < (u_var43 & 0xf) | b_var15;
                u_var43 = CONCAT11((u_var43 >> 8) + b_var15, u_var43 + b_var15 * '\x06') & 0xff0f;
                // code_r0x105024a8:
                u_var43 = u_var43 ^ 0x2e;
            }
            pc_var2 = (u_var48 + pi_var49);
            c_var25 = (u_var45 >> 8);
            *pc_var2 = *pc_var2 + c_var25;
            pc_var2 = (u_var48 + pi_var49);
            *pc_var2 = *pc_var2 + c_var25;
            // code_r0x105024b7:
            u_var43 = u_var43 ^ 0x2e;
            // code_r0x105024ba:
            pb_var1 = (pi_var49 + u_var48 + 0x5024);
            b_var60 = CARRY1(*pb_var1, u_var43);
            *pb_var1 = *pb_var1 + u_var43;
        } else {
            pi_var5 = pi_var49;
            pi_var49 = (pu_var52 + 3);
            bad1::out(*pi_var5, u_var45);
            if (!b_var61) {
                pb_var1 = (pu8_var28 + pi_var49 + 0x6c);
                b_var60 = CARRY1(*pb_var1, b_var24);
                *pb_var1 = *pb_var1 + b_var24;
                b_var61 = *pb_var1 == 0;
                pi_var5 = pi_var49;
                pi_var49 = (pu_var52 + 4);
                bad1::out(*pi_var5, u_var45);
                ppi_var36 = (ppc_var32 + 0x10);
                if (b_var61) {
                    // code_r0x10502335:
                    ppi_var35 = ppi_var36;
                    pb_var1 = (pi_var49 + 0x65);
                    b_var16 = (u_var26 >> 8);
                    b_var60 = CARRY1(*pb_var1, b_var16);
                    *pb_var1 = *pb_var1 + b_var16;
                    ppi_var34 = ppi_var35;
                    if (b_var60 || *pb_var1 == 0) {}
                    // goto code_r0x1050239f;
                    // goto code_r0x1050233a;
                }
                if (b_var60) {}
                // goto code_r0x10502336;
                pi_var58 = ppc_var32[0x10];
                pi_var49 = ppc_var32[0x11];
                u_var48 = ppc_var32[0x12];
                pu8_var28 = ppc_var32[0x14];
                u_var45 = ppc_var32[0x15];
                u_var26 = ppc_var32[0x16];
                u_var43 = ppc_var32[0x17];
                ppc_var33 = ppc_var32 + 0x18;
                ppi_var35 = (ppc_var32 + 0x18);
                pc_var2 = (pu8_var28 + pi_var58 + 0x6e);
                c_var25 = (u_var26 >> 8);
                *pc_var2 = *pc_var2 + c_var25;
                c_var20 = (u_var45 >> 8);
                if (*pc_var2 != '\0') {
                    pu_var4 = (pi_var49 + u_var48 + 0x2e);
                    *pu_var4 = *pu_var4 ^ (ppc_var32 + 0x18);
                    pc_var2 = (pu8_var28 + pi_var58 + 0x6e);
                    *pc_var2 = *pc_var2 + c_var25;
                    ppi_var34 = (ppc_var32 + 0x18);
                    if (*pc_var2 == '\0') {}
                    // goto code_r0x10502317;
                    pu_var4 = (pi_var58 + u_var48 + 0x2e);
                    *pu_var4 = *pu_var4 ^ (ppc_var32 + 0x18);
                    pc_var2 = (pu8_var28 + pi_var58 + 0x6e);
                    *pc_var2 = *pc_var2 + c_var25;
                    if (*pc_var2 == '\0') {
                        bad1::out(*pi_var49, u_var45);
                        ppi_var34 = (ppc_var32 + 0x18);
                        // goto code_r0x10502317;
                    }
                    pu_var4 = (pi_var49 + 0x17);
                    *pu_var4 = *pu_var4 ^ (ppc_var32 + 0x18);
                    pc_var2 = (pu8_var28 + pi_var58 + 0x6e);
                    *pc_var2 = *pc_var2 + c_var25;
                    if (*pc_var2 != '\0') {
                        pi_var57 = pi_var58 + 1;
                        i_var21 = bad1::_in(u_var45);
                        *pi_var58 = i_var21;
                        pu_var4 = (pu8_var28 + pi_var57);
                        *pu_var4 = *pu_var4 ^ pi_var49;
                        pc_var2 = (u_var48 + 0x72);
                        c_var25 = *pc_var2;
                        *pc_var2 = *pc_var2 + c_var20;
                        ppc_var33 = ppc_var32 + 0x17;
                        ppi_var35 = (ppc_var32 + 0x17);
                        ppc_var32[0x17] = 0x6c65;
                        if (!SCARRY1(c_var25, c_var20)) {}
                        // goto code_r0x1050230c;
                        // goto code_r0x1050233a;
                    }
                    // goto code_r0x10502360;
                }
                // code_r0x1050230c:
                pc_var2 = (u_var48 + pi_var49);
                *pc_var2 = *pc_var2 + c_var20;
                ppi_var34 = ppc_var33;
                // code_r0x10502317:
                pc_var2 = 0x3438;
                *pc_var2 = *pc_var2 + c_var20;
                pi_var57 = *ppi_var34;
                pi_var49 = ppi_var34[1];
                u_var48 = ppi_var34[2];
                pu8_var28 = ppi_var34[4];
                u_var45 = ppi_var34[5];
                u_var26 = ppi_var34[6];
                u_var43 = ppi_var34[7];
                ppi_var36 = ppi_var34 + 8;
                ppi_var41 = ppi_var34 + 8;
                ppi_var40 = ppi_var34 + 8;
                pc_var2 = (u_var48 + 0x6f);
                *pc_var2 = *pc_var2 + (u_var26 >> 8);
                if (*pc_var2 != '\0') {
                    pb_var1 = (pi_var57 + u_var48 + 0x75);
                    b_var16 = (u_var45 >> 8);
                    b_var60 = CARRY1(*pb_var1, b_var16);
                    *pb_var1 = *pb_var1 + b_var16;
                    ppi_var36 = ppi_var34 + 8;
                    // goto code_r0x1050232c;
                }
                pi_var5 = pi_var57 + 0x3c;
                c_var25 = *pi_var5;
                c_var20 = (u_var43 >> 8);
                *pi_var5 = *pi_var5 + c_var20;
                if (!SCARRY1(c_var25, c_var20)) {}
                // goto code_r0x10502392;
                if (-1 < *pi_var5) {
                    u_var6 = bad1::_in(u_var45);
                    *pi_var57 = u_var6;
                    // goto code_r0x10502401;
                }
                pc_var2 = (u_var48 + 0x72);
                *pc_var2 = *pc_var2 + c_var20;
                b_var15 = 9 < (u_var43 & 0xf) | b_var15;
                u_var43 = CONCAT11(c_var20 + b_var15, u_var43 + b_var15 * '\x06') & 0xff0f;
                // code_r0x10502474:
                pi_var57 = *ppi_var41;
                ppi_var40 = ppi_var41 + 1;
                u_var43 = u_var43 ^ (pu8_var28 + pi_var49);
                b_var61 = u_var43 == 0;
                // goto code_r0x10502477;
            }
            pi_var5 = pi_var57;
            pi_var57 = pi_var57 + 1;
            i_var21 = bad1::_in(u_var45);
            *pi_var5 = i_var21;
            if (!b_var60) {}
            // goto code_r0x105023a1;
            // code_r0x1050232c:
            if (!b_var60) {
                pi_var57 = *ppi_var36;
                pi_var49 = ppi_var36[1];
                u_var48 = ppi_var36[2];
                pu8_var28 = ppi_var36[4];
                u_var45 = ppi_var36[5];
                u_var26 = ppi_var36[6];
                u_var43 = ppi_var36[7];
                pu_var4 = (pi_var57 + 0x17);
                *pu_var4 = *pu_var4 + b_var60 * (((ppi_var36 + 8) & 3) - (*pu_var4 & 3));
                ppi_var36 = ppi_var36 + 8;
                // goto code_r0x10502335;
            }
            pc_var2 = (pi_var57 + 0x61);
            *pc_var2 = *pc_var2 + (u_var43 >> 8);
            // code_r0x10502392:
            pi_var58 = pi_var49 + 1;
            bad1::out(*pi_var49, u_var45);
            pb_var1 = (pi_var57 + 0x61);
            b_var16 = (u_var43 >> 8);
            b_var60 = CARRY1(*pb_var1, b_var16);
            *pb_var1 = *pb_var1 + b_var16;
            b_var16 = *pb_var1;
            if (!b_var60) {
                (ppi_var36 + -2) = 0x6171;
                lVar12 = (pu8_var28 + pi_var58) * 0x73;
                u_var43 = lVar12;
                b_var60 = u_var43 != lVar12;
                pi_var49 = pi_var49 + 2;
                bad1::out(*pi_var58, u_var45);
                ppi_var38 = (ppi_var36 + -2);
                // code_r0x105023a1:
                u_var6 = bad1::_in(u_var45);
                *pi_var57 = u_var6;
                if (!b_var60) {
                    lVar12 = (u_var48 + 100) * 0x7300;
                    u_var48 = lVar12;
                    b_var60 = u_var48 != lVar12;
                    // goto code_r0x105023a9;
                }
                pi_var57 = *ppi_var38;
                ppi_var34 = ppi_var38 + 1;
                // goto code_r0x10502418;
            }
            // code_r0x1050240e:
            pi_var49 = pi_var58;
            ppi_var34 = ppi_var36;
            if (b_var16 != 0) {}
            // goto code_r0x10502410;
            // code_r0x10502443:
            if (b_var60) {
                u_var45 = u_var45 & 0xff | ((u_var45 >> 8) ^ *0x2e34) << 8;
                // goto code_r0x105024ba;
            }
            // code_r0x10502445:
            pi_var5 = pi_var49;
            pi_var49 = pi_var49 + 1;
            bad1::out(*pi_var5, u_var45);
            if (b_var60) {
                // code_r0x1050244d:
                pi_var57 = *ppi_var36;
                pi_var49 = ppi_var36[1];
                u_var48 = ppi_var36[2];
                pu8_var28 = ppi_var36[4];
                u_var45 = ppi_var36[5];
                u_var26 = ppi_var36[6];
                u_var43 = ppi_var36[7];
                ppi_var35 = ppi_var36 + 8;
                // code_r0x1050244f:
                pi_var5 = pi_var57;
                pi_var57 = pi_var57 + 1;
                i_var21 = bad1::_in(u_var45);
                *pi_var5 = i_var21;
                // code_r0x10502450:
                pb_var1 = (pi_var49 + 0x75);
                b_var16 = *pb_var1;
                b_var23 = (u_var45 >> 8);
                *pb_var1 = *pb_var1 + b_var23;
                if (!CARRY1(b_var16, b_var23)) {
                    i_var21 = (u_var48 + 0x65);
                    u_var48 = i_var21 * 0x6d00;
                    u_var43 = u_var43 ^ 0x5f39;
                    pu_var4 = (pi_var49 + i_var21 * 0x3680);
                    *pu_var4 = *pu_var4 ^ pi_var49;
                    pb_var1 = pu8_var28 + 0x6f;
                    b_var16 = (u_var43 >> 8);
                    b_var60 = CARRY1(*pb_var1, b_var16);
                    *pb_var1 = *pb_var1 + b_var16;
                    // code_r0x10502465:
                    if (!b_var60) {
                        ppi_var40 = (ppi_var35 + 2);
                        // goto code_r0x10502469;
                    }
                    pb_var1 = pu8_var28 + pi_var49;
                    *pb_var1 = *pb_var1 + u_var43;
                    pb_var1 = pu8_var28 + pi_var49;
                    *pb_var1 = *pb_var1 + u_var43;
                    u_var45 = u_var45 & 0xff | ((u_var45 >> 8) ^ *pu8_var28) << 8;
                    // goto code_r0x105024a8;
                }
                // goto code_r0x105024b7;
            }
        }
        pb_var1 = (pi_var57 + u_var48 + 0x5024);
        b_var23 = u_var43;
        b_var61 = CARRY1(*pb_var1, b_var23) || CARRY1(*pb_var1 + b_var23, b_var60);
        *pb_var1 = *pb_var1 + b_var23 + b_var60;
        pb_var1 = (pi_var57 + u_var48 + 0x5024);
        b_var16 = *pb_var1 + u_var26;
        b_var60 = CARRY1(*pb_var1, u_var26) || CARRY1(b_var16, b_var61);
        *pb_var1 = b_var16 + b_var61;
        pb_var1 = (pi_var57 + u_var48 + 0x5024);
        b_var16 = *pb_var1 + u_var45;
        b_var61 = CARRY1(*pb_var1, u_var45) || CARRY1(b_var16, b_var60);
        *pb_var1 = b_var16 + b_var60;
        pb_var1 = (pi_var57 + u_var48 + 0x5024);
        b_var16 = *pb_var1 + pu8_var28;
        b_var60 = CARRY1(*pb_var1, pu8_var28) || CARRY1(b_var16, b_var61);
        *pb_var1 = b_var16 + b_var61;
        pb_var1 = (pi_var57 + u_var48 + 0x5024);
        b_var16 = (u_var43 >> 8);
        b_var61 = CARRY1(*pb_var1, b_var16) || CARRY1(*pb_var1 + b_var16, b_var60);
        *pb_var1 = *pb_var1 + b_var16 + b_var60;
        pi_var5 = pi_var49 + 0x2812;
        b_var60 = CARRY1(*pi_var5, b_var16) || CARRY1(*pi_var5 + b_var16, b_var61);
        *pi_var5 = *pi_var5 + b_var16 + b_var61;
        pi_var5 = pi_var57 + 0x2812;
        b_var61 = CARRY1(*pi_var5, b_var16) || CARRY1(*pi_var5 + b_var16, b_var60);
        *pi_var5 = *pi_var5 + b_var16 + b_var60;
        pb_var1 = (u_var48 + 0x5024);
        b_var60 = CARRY1(*pb_var1, b_var16) || CARRY1(*pb_var1 + b_var16, b_var61);
        *pb_var1 = *pb_var1 + b_var16 + b_var61;
        pb_var1 = (u_var48 + 0x5024);
        b_var15 = (u_var26 >> 8);
        b_var16 = *pb_var1 + b_var15;
        b_var61 = CARRY1(*pb_var1, b_var15) || CARRY1(b_var16, b_var60);
        *pb_var1 = b_var16 + b_var60;
        pb_var1 = (u_var48 + 0x5024);
        b_var15 = *pb_var1;
        b_var24 = (u_var45 >> 8);
        b_var16 = *pb_var1;
        *pb_var1 = b_var16 + b_var24 + b_var61;
        pb_var1 = pu8_var28 + pi_var49;
        *pb_var1 =
            *pb_var1 + b_var23 + (CARRY1(b_var15, b_var24) || CARRY1(b_var16 + b_var24, b_var61));
        u_var45 = (b_var24 ^ pu8_var28[pi_var49]) << 8;
        u_var43 = (b_var23 ^ 0x2e);
        // code_r0x105024f2:
        pc_var2 = (u_var48 + pi_var49);
        b_var16 = (u_var45 >> 8);
        *pc_var2 = *pc_var2 + b_var16;
        pb_var1 = pu8_var28 + pi_var49;
        *pb_var1 = *pb_var1 ^ (pu8_var28 >> 8);
        pc_var2 = (u_var48 + pi_var49);
        *pc_var2 = *pc_var2 + b_var16;
        pc_var2 = (u_var48 + pi_var49);
        *pc_var2 = *pc_var2 + (b_var16 ^ pu8_var28[pi_var49]);
        pu_var4 = (pu8_var28 + pi_var57);
        *pu_var4 = *pu_var4 ^ pi_var49;
        pb_var1 = pu8_var28 + pi_var49;
        *pb_var1 = *pb_var1 + u_var43;
        pb_var1 = pu8_var28 + pi_var49;
        *pb_var1 = *pb_var1 + u_var43;
    }
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_1312() {
    let pc_var1: String;
    let mut c_var2: u8;
    let mut extraout_AH: u8;
    let mut extraout_dl: u8;
    let mut extraout_dh: u8;
    let in_bx: String;
    let pu_var3: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: String;
    let unaff_di: String;
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut stack0xfffe: u16;

    pu_var3 = &stack0xfffe;
    c_var2 = '\x1e';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_bp };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    bad1::_in(0xff);
    (**(in_bx + unaff_di))();
    pc_var1 = unaff_di;
    unsafe {
        *pc_var1 = *pc_var1 + extraout_AH;
        pc_var1 = unaff_si;
        *pc_var1 = *pc_var1 + extraout_dh;
        pc_var1 = in_bx;
        *pc_var1 = *pc_var1 + extraout_dh;
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + extraout_dh;
        pc_var1 = in_bx;
        *pc_var1 = *pc_var1 + extraout_AH;
        pc_var1 = 0x100;
        *pc_var1 = *pc_var1 + extraout_dl;
    }
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_12ee() {
    let mut c_var1: u8;
    let pu_var2: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut stack0xfffe: u16;

    pu_var2 = &stack0xfffe;
    c_var1 = '\x1e';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    } {}
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_1234() {
    let pc_var1: String;
    let ppc_var2: fn();
    let mut c_var3: u8;
    let mut in_cl: u8;
    let mut in_ch: u8;
    let mut extraout_dh: u8;
    let mut in_bx: i32;
    let pu_var4: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: String;
    let unaff_di: fn();
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut stack0xfffe: u16;

    pu_var4 = &stack0xfffe;
    c_var3 = '\x1e';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var4 = pu_var4 + -1;
        *pu_var4 = *unaff_bp;
        c_var3 = c_var3 + -1;
        '\0' < c_var3
    } {}
    c_var3 = (**unaff_di)();
    pc_var1 = 0x4000;

    *pc_var1 = *pc_var1 + in_ch;
    pc_var1 = (in_bx + unaff_di);
    *pc_var1 = *pc_var1 + in_cl;
    pc_var1 = unaff_si;
    *pc_var1 = *pc_var1 + extraout_dh;
    ppc_var2 = unaff_di;
    *ppc_var2 = *ppc_var2 + in_bx + 0x1;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + c_var3;

    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_1214() {
    let pc_var1: String;
    let ppc_var2: fn();
    let mut c_var3: u8;
    let mut in_cl: u8;
    let mut in_ch: u8;
    let mut extraout_dh: u8;
    let mut in_BL: u8;
    let pu_var4: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_si: i32;
    let unaff_di: fn();
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut stack0xfffe: u16;

    pu_var4 = &stack0xfffe;
    c_var3 = '\x1e';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var4 = pu_var4 + -1;
        unsafe { *pu_var4 = *unaff_bp };
        c_var3 = c_var3 + -1;
        '\0' < c_var3
    } {}
    c_var3 = (**unaff_di)();
    pc_var1 = 0x4000;
    unsafe {
        *pc_var1 = *pc_var1 + in_ch;
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + in_cl;
        ppc_var2 = unaff_di;
        *ppc_var2 = *ppc_var2 + extraout_dh;
        ppc_var2 = unaff_di;
        *ppc_var2 = *ppc_var2 + in_BL;
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + c_var3;
    }
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_0e8e() {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let pi_var3: *mut i32;
    let mut u_var4: i32;
    let mut c_var5: u8;
    let mut b_var6: u8;
    let mut in_ax: i32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut in_cl: u8;
    let mut in_DL: u8;
    let in_bx: String;
    let mut unaff_si: i32;
    let mut unaff_di: i32;
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut stack0xfffe: u16;

    unsafe {
        pu8_var2 = (in_bx + unaff_si);
        u_var7 = (in_ax & 0xff00 | (in_ax + *pu8_var2)) + 0xc900 + CARRY1(in_ax, *pu8_var2);
        pi_var3 = (in_bx + unaff_di);
        *pi_var3 = *pi_var3 + u_var7;
        pc_var1 = &stack0xfffe + unaff_si;
        b_var6 = u_var7;
        *pc_var1 = *pc_var1 + b_var6;
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + (u_var7 >> 8);
        pi_var3 = (in_bx + unaff_di);
        *pi_var3 = *pi_var3 + u_var7;
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + b_var6;
        pc_var1 = in_bx;
        *pc_var1 = *pc_var1 + b_var6;
        pc_var1 = in_bx + unaff_di + 0x18;
        *pc_var1 = *pc_var1 + in_DL + in_cl;
        pc_var1 = in_bx + unaff_si;
        *pc_var1 = *pc_var1 + b_var6;
        pu8_var2 = (in_bx + unaff_si);
        u_var8 = u_var7 & 0xff00 | (b_var6 + *pu8_var2);
        u_var7 = CARRY1(b_var6, *pu8_var2);
        u_var4 = u_var8 + 0x5200;
        u_var9 = u_var4 + u_var7;
        pc_var1 = in_bx + unaff_di;
        c_var5 = u_var9;
        *pc_var1 = (*pc_var1 - c_var5) - (0xadff < u_var8 || CARRY2(u_var4, u_var7));
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + c_var5;
        pc_var1 = &stack0xfffe + unaff_si;
        *pc_var1 = *pc_var1 + (u_var9 >> 8);
        pc_var1 = in_bx + unaff_di + 0x18;
        *pc_var1 = *pc_var1 + c_var5;
        pi_var3 = (in_bx + unaff_si);
        *pi_var3 = *pi_var3 + u_var9;
        pc_var1 = in_bx + unaff_si;
        *pc_var1 = *pc_var1 + c_var5;
        pi_var3 = (in_bx + unaff_si);
        *pi_var3 = *pi_var3 + u_var9;
        b_var6 = c_var5 + in_bx[unaff_si];
        pi_var3 = (in_bx + unaff_si);
        *pi_var3 = *pi_var3 + (u_var9 & 0xff00 | b_var6);
        pc_var1 = in_bx + unaff_si;
        *pc_var1 = *pc_var1 + b_var6;
        b_var6 = b_var6 + in_bx[unaff_si] + in_bx[unaff_si];
        pc_var1 = in_bx + unaff_si;
        *pc_var1 = *pc_var1 + b_var6;
        pi_var3 = (in_bx + unaff_si);
        *pi_var3 = *pi_var3 + (u_var9 & 0xff00 | b_var6);
    }
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_11f4() {
    let pc_var1: String;
    let mut c_var2: u8;
    let mut in_ch: u8;
    let in_bx: String;
    let pu_var3: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: String;
    let unaff_di: fn();
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut stack0xfffe: u16;

    pu_var3 = &stack0xfffe;
    c_var2 = '\x1e';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_bp };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    c_var2 = (**unaff_di)();
    pc_var1 = in_bx;
    *pc_var1 = *pc_var1 + in_ch;
    pc_var1 = in_bx + unaff_di;
    *pc_var1 = *pc_var1 + c_var2;
    pc_var1 = unaff_si;
    *pc_var1 = *pc_var1 + c_var2;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + c_var2;

    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1050_0daa() {
    let pc_var1: String;
    let pi_var2: *mut i32;
    let pb_var3: Vec<u8>;
    let mut b_var4: u8;
    let mut b_var5: u8;
    let mut in_ax: i32;
    let mut u_var6: i32;
    let mut b_var11: u8;
    let mut c_var12: u8;
    let mut i_var8: i32;
    let mut u_var9: i32;
    let mut u_var10: i32;
    let mut in_cl: u8;
    let mut in_DL: u8;
    let mut in_DH: u8;
    let in_bx: String;
    let pc_var13: String;
    let mut b_var14: u8;
    let mut unaff_bp: u8;
    let unaff_si: String;
    let unaff_di: *mut i32;
    let mut unaff_es: u16;
    let mut unaff_ss: u16;
    let mut unaff_ds: u16;
    let mut in_af: u8;
    let mut u_var7: i32;
    let mut stack0xfffe: u16;

    b_var4 = in_ax - *unaff_di;
    pc_var1 = in_bx + unaff_si;
    *pc_var1 = *pc_var1 + b_var4;
    u_var6 = ((in_ax & 0xff00 | (b_var4 ^ in_bx[unaff_si])) - *unaff_di) + 0x3200;
    pi_var2 = unaff_di;
    c_var12 = (in_bx >> 8);
    *pi_var2 = *pi_var2 + c_var12;
    b_var4 = u_var6 - 5;
    pc_var1 = in_bx + unaff_si;
    *pc_var1 = *pc_var1 + b_var4;
    u_var6 = (u_var6 & 0xff00 | (b_var4 ^ in_bx[unaff_si])) - 5;
    pc_var1 = in_bx + unaff_di;
    *pc_var1 = *pc_var1 + in_bx;
    pc_var1 = in_bx;
    *pc_var1 = *pc_var1 + c_var12;
    pc_var13 = in_bx + -1;
    pc_var1 = (pc_var13 + unaff_si + 0x2f00);
    b_var5 = u_var6;
    *pc_var1 = *pc_var1 + b_var5;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 - u_var6;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + 0x530;
    pb_var3 = (pc_var13 + unaff_si);
    b_var4 = *pb_var3;
    *pb_var3 = *pb_var3 + b_var5;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = (*pi_var2 - u_var6) - CARRY1(b_var4, b_var5);
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + '1';
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    b_var5 = b_var5 ^ *unaff_di;
    u_var6 = u_var6 & 0xff00;
    u_var7 = u_var6 | b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + u_var7;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + in_cl;
    pc_var1 = unaff_si;
    *pc_var1 = *pc_var1 + pc_var13;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + u_var7;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = 0x1d00;
    *pc_var1 = *pc_var1 + in_DL;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + u_var7;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_di;
    c_var12 = (u_var6 >> 8);
    *pc_var1 = *pc_var1 + c_var12;
    pc_var1 = 0x1;
    *pc_var1 = *pc_var1 + pc_var13;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + in_DH;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + c_var12;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + u_var7;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pi_var2 = unaff_di;
    b_var14 = (pc_var13 >> 8);
    *pi_var2 = *pi_var2 + b_var14;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + c_var12;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + u_var7;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + in_cl;
    b_var5 = b_var5 & pc_var13[unaff_di];
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    u_var6 = (u_var6 | b_var5) + (pc_var13 + unaff_si);
    pc_var1 = &stack0xfffe + unaff_di;
    b_var11 = (u_var6 >> 8);
    *pc_var1 = *pc_var1 + b_var11;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + u_var6;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + u_var6;
    pc_var1 = unaff_si;
    *pc_var1 = *pc_var1 + b_var11;
    pc_var1 = &stack0xfffe + unaff_di;
    b_var5 = (u_var6 & 0x101);
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13 + unaff_di;
    *pc_var1 = *pc_var1 + in_DH;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + (u_var6 & 0x101);
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + b_var5;
    b_var4 = 9 < b_var5 | in_af;
    b_var5 = b_var5 + b_var4 * '\x06';
    b_var5 = b_var5
        + (0x90 < (b_var5 & 0xf0) | CARRY1(unaff_bp, b_var14) | b_var4 * (0xf9 < b_var5)) * '`';
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + (u_var6 & 0x100 | b_var5);
    pi_var2 = unaff_di;
    *pi_var2 = *pi_var2 + b_var5;
    pc_var1 = pc_var13;
    *pc_var1 = *pc_var1 + b_var5;
    c_var12 = (b_var11 & 1) + b_var5;
    i_var8 = CONCAT11(c_var12, b_var5);
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + i_var8;
    pi_var2 = unaff_di;
    *pi_var2 = *pi_var2 + b_var5;
    pi_var2 = unaff_di;
    *pi_var2 = *pi_var2 + in_DL;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + i_var8;
    pi_var2 = unaff_di;
    *pi_var2 = *pi_var2 + b_var5;
    pc_var1 = &stack0xfffe + unaff_di;
    *pc_var1 = *pc_var1 + c_var12;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + i_var8;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13;
    *pc_var1 = *pc_var1 + b_var5;
    b_var5 = b_var5 + in_cl;
    i_var8 = CONCAT11(c_var12, b_var5);
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + i_var8;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pi_var2 = unaff_di;
    *pi_var2 = *pi_var2 + in_DL;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + i_var8;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + c_var12;
    pi_var2 = (pc_var13 + unaff_di);
    *pi_var2 = *pi_var2 + i_var8;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = pc_var13;
    *pc_var1 = *pc_var1 + b_var5;
    pc_var1 = (pc_var13 + unaff_di + 0x18);
    *pc_var1 = *pc_var1 + in_DL + in_cl * 0x2;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var5;
    pb_var3 = (pc_var13 + unaff_si);
    u_var9 = CONCAT11(c_var12, b_var5 + *pb_var3);
    u_var6 = CARRY1(b_var5, *pb_var3);
    u_var7 = u_var9 + 0x5200;
    u_var10 = u_var7 + u_var6;
    pc_var1 = pc_var13 + unaff_di;
    c_var12 = u_var10;
    *pc_var1 = (*pc_var1 - c_var12) - (0xadff < u_var9 || CARRY2(u_var7, u_var6));
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + c_var12;
    pc_var1 = &stack0xfffe + unaff_si;
    *pc_var1 = *pc_var1 + (u_var10 >> 8);
    pc_var1 = (pc_var13 + unaff_di + 0x18);
    *pc_var1 = *pc_var1 + c_var12;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + u_var10;
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + c_var12;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + u_var10;
    b_var4 = c_var12 + pc_var13[unaff_si];
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + (u_var10 & 0xff00 | b_var4);
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var4;
    b_var4 = b_var4 + pc_var13[unaff_si] + pc_var13[unaff_si];
    pc_var1 = pc_var13 + unaff_si;
    *pc_var1 = *pc_var1 + b_var4;
    pi_var2 = (pc_var13 + unaff_si);
    *pi_var2 = *pi_var2 + (u_var10 & 0xff00 | b_var4);
    // WARNING: Bad instruction - Truncating control flow here

    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1040_d920(param_1: u32) {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let pu_var3: *mut u16;
    let u_var4: u8;
    let mut b_var5: u8;
    let mut c_var6: u8;
    let mut in_ax: i32;
    let mut in_cx: u16;
    let mut b_var7: u8;
    let mut in_dx: i32;
    let mut b_var8: u8;
    let mut in_bx: i32;
    let mut b_var9: u8;
    let pu_var10: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_es: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var11: bool;
    let mut b_var12: bool;
    let in_stack_0000bf2a: Vec<u8>;
    let in_stack_0000bf2c: *mut u16;
    let mut in_stack_0000bf32: i32;
    let mut in_stack_0000bf34: u16;
    let mut in_stack_0000bf38: u8;
    let local_4e: u8;
    let local_2b: u8;
    let stack0xfffe: u16;

    pu_var10 = &stack0xfffe;
    c_var6 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_bp };
        c_var6 = c_var6 + -1;
        '\0' < c_var6
    } {}
    b_var9 = (in_bx >> 8);
    unaff_si[in_bx] = b_var9;
    b_var8 = ((in_dx + 1) >> 8);
    b_var11 = CARRY1(b_var8, b_var9) || CARRY1(b_var8 + b_var9, in_CF);
    b_var7 = (in_dx + 1);
    pu8_var2 = unaff_si + in_bx;
    unsafe {
        b_var8 = *pu8_var2;
        b_var5 = *pu8_var2 - b_var7;
        b_var12 = *pu8_var2 < b_var7 || b_var5 < b_var11;
        *pu8_var2 = b_var5 - b_var11;
        if ((*pu8_var2 != 0)
            && (SBORROW1(b_var8, b_var7) != (SBORROW1(b_var5, b_var11) == (*pu8_var2 < '\0'))))
        {
            pu8_var2 = unaff_si;
            b_var8 = *pu8_var2;
            b_var5 = *pu8_var2;
            *pu8_var2 = b_var5 + b_var9 + b_var12;
            b_var11 = CARRY1(local_4e, in_bx)
                || CARRY1(
                    local_4e + in_bx,
                    CARRY1(b_var8, b_var9) || CARRY1(b_var5 + b_var9, b_var12),
                );
            pu8_var2 = unaff_si + -0x4f;
            b_var8 = *pu8_var2;
            b_var5 = *pu8_var2;
            *pu8_var2 = b_var5 + b_var9 + b_var11;
            b_var7 = (in_cx >> 8);
            pc_var1 = &stack0xfffe + unaff_si;
            *pc_var1 = *pc_var1
                + in_cx
                + (CARRY1(local_2b, b_var7)
                    || CARRY1(
                        local_2b + b_var7,
                        CARRY1(b_var8, b_var9) || CARRY1(b_var5 + b_var9, b_var11),
                    ));
            if (!SBORROW2(in_ax, 1)) {
                bad1::out(*in_stack_0000bf2c, in_stack_0000bf34);
                u_var4 = bad1::_in(in_stack_0000bf34);
                *in_stack_0000bf2a = u_var4;
                pu_var3 = (in_stack_0000bf2a + 0x73);
                *pu_var3 = *pu_var3;
                // WARNING: Bad instruction - Truncating control flow here
                bad1::halt_baddata();
            }
            pc_var1 = (in_stack_0000bf32 + in_stack_0000bf2c);
            *pc_var1 = *pc_var1 + in_stack_0000bf38;
            // WARNING: Bad instruction - Truncating control flow here
            bad1::halt_baddata();
        }

        if (*pu8_var2 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub unsafe fn bad_fn_1040_bf16(ctx: &mut AppContext) {
    let mut c_var1: u8;
    let pu_var2: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_ss: u16;
    let mut stack0xfffe: u16;

    pu_var2 = &stack0xfffe;
    c_var1 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    } {}
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn bad_fn_1018_d46e() {
    let mut c_var1: u8;
    let pu_var2: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_ss: u16;
    let mut stack0xfffe: u16;

    pu_var2 = &stack0xfffe;
    c_var1 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    } {}
    // WARNING: Bad instruction - Truncating control flow here
    bad1::halt_baddata();
}

pub unsafe fn pass1_1038_ca75(ctx: &mut AppContext, param_1: &mut Struct44) -> &mut Struct44 {
    let pu_var1: Vec<u8>;
    let pc_var2: String;
    let pb_var3: Vec<u8>;
    let pu_var4: *mut u32;
    let mut b_var5: u8;
    let mut u_var6: i32;
    let mut in_al: u8;
    let mut in_cx: u16;
    let mut b_var7: u8;
    let mut c_var8: u8;
    let mut b_var9: u8;
    let mut b_var10: u8;
    let mut in_dx: i32;
    let mut b_var12: u8;
    let mut i_var11: i32;
    let mut in_bx: i32;
    let mut u_var13: i32;
    let mut b_var14: u8;
    let mut unaff_bp: u16;
    let unaff_si: Vec<u8>;
    let unaff_di: Vec<u8>;
    let mut unaff_es: i32;
    let mut u_var15: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var16: bool;
    let mut b_var17: bool;
    let mut in_SF: bool;
    let ppVar18: *mut pass1_struct_1;
    let in_stack_00000068: Vec<u8>;
    let mut in_stack_0000407f: u8;
    let mut in_stack_0000efc4: u32;
    let in_struct_1: *mut Struct68;
    let in_stack_0000efc8: Vec<u8>;
    let mut in_stack_0000efca: u32;
    let mut in_stack_0000efce: u32;
    let in_stack_0000efd2: Vec<u8>;
    let local_39: u8;
    let local_38: u8;
    let _uStack3: u8;
    let mut stack0x4079: u16;
    let mut stack0xefc4: u16;
    let mut stack0x002a: u16;

    _uStack3 = CONCAT21(unaff_bp, _uStack3);
    b_var9 = in_dx;
    b_var7 = (in_dx >> 8);
    b_var12 = in_bx;
    u_var13 = param_1;
    u_var15 = (param_1 >> 0x10);
    if (in_SF) {
        b_var16 = CARRY1(ctx.s__s___lu_1050_38c5[3], b_var7);
        b_var14 = ctx.s__s___lu_1050_38c5[3] + b_var7;
        ctx.s__s___lu_1050_38c5[3] = b_var14 + in_CF;
        local_39 = local_39 + b_var12 + (b_var16 || CARRY1(b_var14, in_CF));
        pb_var3 = unaff_si + in_bx;
        unsafe {
            b_var16 = *pb_var3 < b_var9;
            in_struct_1 = (in_stack_0000efc4 & 0xffff0000 | ZEXT24(unaff_si));
            b_var14 = (in_bx >> 8);
            if (*pb_var3 != b_var9 && b_var9 <= *pb_var3) {
                pb_var3 = unaff_si + in_bx;
                *pb_var3 = (*pb_var3 - (b_var9 + b_var14 + b_var16))
                    - (CARRY1(b_var9, b_var14) || CARRY1(b_var9 + b_var14, b_var16));
                *unaff_di = *unaff_si;
                //PTR_LOOP_1050_1038._0_1_ = in_al;
                ctx.PTR_LOOP_1050_1038 = in_al;
                // WARNING: Bad instruction - Truncating control flow here
                bad1::halt_baddata();
            }
            pb_var3 = &stack0x4079 + unaff_si;
            b_var17 = CARRY1(*pb_var3, b_var7) || CARRY1(*pb_var3 + b_var7, b_var16);
            *pb_var3 = *pb_var3 + b_var7 + b_var16;
            pb_var3 = unaff_si;
            b_var5 = *pb_var3;
            b_var10 = *pb_var3 + b_var12;
            b_var16 = CARRY1(*pb_var3, b_var12) || CARRY1(b_var10, b_var17);
            *pb_var3 = b_var10 + b_var17;
            if ((*pb_var3 == 0)
                || (SCARRY1(b_var5, b_var12) != (SCARRY1(b_var10, b_var17) != (*pb_var3 < '\0'))))
            {
                pu_var1 = &local_38 + unaff_si;
                *pu_var1 = *pu_var1 + in_al + b_var16;
                b_var7 = unaff_si[in_bx];
                pb_var3 = (in_bx + 0x40);
                b_var12 = *pb_var3;
                *pb_var3 = b_var14;
                in_bx = in_bx & 0xff | b_var12 << 8;
                pb_var3 = unaff_si;
                *pb_var3 = *pb_var3 + in_cx + (b_var7 < b_var9);
                pb_var3 = unaff_si + in_bx + 0x10;
                *pb_var3 = *pb_var3 + 0x54;
                pb_var3 = unaff_si + in_bx + 0x10;
                b_var16 = 0xa1 < *pb_var3;
                b_var9 = *pb_var3;
                *pb_var3 = *pb_var3 + 0x5e;
                if ((*pb_var3 != 0) && (SCARRY1(b_var9, '^') == (*pb_var3 < '\0'))) {
                    // goto code_r0x1038caa3;
                    todo!()
                }

                in_stack_00000068 = &stack0xefc4 + in_stack_00000068;
                _uStack3 = CONCAT12((unaff_bp >> 8), &stack0xefc4 + _uStack3);
                if (CARRY2(_uStack3, &stack0xefc4) || &stack0xefc4 + _uStack3 == 0x0) {
                    in_struct_1 = (in_stack_0000efc4 & 0xffff0000 | unaff_es);
                }
            } else {
                b_var17 = CARRY1(b_var7, b_var14) || CARRY1(b_var7 + b_var14, b_var16);
                pb_var3 = unaff_si + in_bx;
                b_var9 = *pb_var3;
                b_var10 = (in_dx & 0xff);
                b_var12 = *pb_var3;
                b_var5 = *pb_var3 - b_var10;
                *pb_var3 = b_var5 - b_var17;
                if ((*pb_var3 == 0)
                    || (SBORROW1(b_var12, b_var10)
                        != (SBORROW1(b_var5, b_var17) != (*pb_var3 < '\0'))))
                {
                    if (*pb_var3 != 0) {
                        error_check_1000_17ce(param_1);
                    }
                    return param_1;
                }
                pb_var3 = unaff_di + -0x75;
                *pb_var3 = *pb_var3 + b_var10 + (b_var9 < b_var10 || b_var5 < b_var17);
                bad1::_in(in_dx & 0xff | (b_var7 + b_var14 + b_var16) << 8);
            }
            process_struct_1040_7728(
                in_struct_1,
                in_stack_0000efc8,
                in_stack_0000efca,
                in_stack_0000efce,
                (in_stack_0000efce >> 0x10),
            );
            (u_var13 + 0x8e) = 0;
            param_1.offset = 0xcc9a;
            (u_var13 + 2) = &ctx.PTR_LOOP_1050_1038;
            in_bx = u_var13;
        }
    } else {
        b_var16 = CARRY1(b_var7, b_var12) || CARRY1(b_var7 + b_var12, in_CF);
        in_dx = in_dx & 0xff | (b_var7 + b_var12 + in_CF) << 8;
        // code_r0x1038caa3:
        pb_var3 = unaff_si + (in_bx - 0x7f);
        unsafe {
            b_var9 = *pb_var3;
            b_var7 = *pb_var3 + in_bx;
            *pb_var3 = b_var7 + b_var16;
            b_var12 = (in_dx >> 8);
            pu_var4 = (unaff_si + in_bx + 0x10);
            u_var6 = *pu_var4;
            *pu_var4 = *pu_var4 + 0x60ea;
            pb_var3 = unaff_si + in_bx;
            *pb_var3 = (*pb_var3 - (in_dx & 0xff)) - (0x9f15 < u_var6);
            i_var11 = (in_dx & 0xff
                | (b_var12
                    + (in_bx >> 8)
                    + (CARRY1(in_stack_0000407f, b_var12)
                        || CARRY1(
                            in_stack_0000407f + b_var12,
                            CARRY1(b_var9, in_bx) || CARRY1(b_var7, b_var16),
                        )))
                    << 8)
                - 1;
            pb_var3 = unaff_si + in_bx + 0x10;
            *pb_var3 = *pb_var3 + 0x66;
            pb_var3 = unaff_si + in_bx + 0x10;
            b_var9 = *pb_var3;
            *pb_var3 = *pb_var3 - 0x22;
            if (-1 < *pb_var3) {
                b_var7 = (in_cx >> 8);
                b_var12 = (i_var11 >> 8);
                pb_var3 = unaff_si + in_bx;
                *pb_var3 = (*pb_var3 - i_var11)
                    - (CARRY1(b_var12, b_var7) || CARRY1(b_var12 + b_var7, 0x21 < b_var9));
                loop {
                    // WARNING: Do nothing block with infinite loop
                }
            }
        }
    }
    c_var8 = (in_cx >> 8);
    func_0x47c726cc();
    pc_var2 = &stack0x002a + unaff_si;
    unsafe { *pc_var2 = *pc_var2 + c_var8 + CARRY1((in_bx >> 8), unaff_si[in_bx]) };
    ppVar18 = process_struct_1010_20ba(ctx._g_struct_372_1050_0ed0, in_stack_0000efd2);
    (u_var13 + 0x8e) = ppVar18;
    (u_var13 + 0x90) = (ppVar18 >> 0x10);
    (u_var13 + 0x74) = 0;
    return param_1;
}

fn func_0x47c726cc() -> () {
    todo!()
}
