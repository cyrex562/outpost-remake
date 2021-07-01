use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT31, LocalDescriptorTableRegister, POPCOUNT, SCARRY1, SCARRY2, ZEXT24};


// pub unsafe fn bad_fn_1138_0034(param1: u8, param2: u8, param3: u16, param4: u16, param5: u8) {
//     let register0x00000010: u16;
//     let stack0x0063: u16;
//     let stack0x834e: u16;
//     let stack0x005f: u16;
//     let stack0x006f: u16;
//     let stack0x08ff: u16;
//     let pu_var1: Vec<u8>;
//     let pc_var2: Vec<u8>;
//     let pb_var3: Vec<u8>;
//     let pi_var4: &mut  i32;
//     let pu_var5: Vec<u8>;
//     let mut u_var6: u16;
//     let pu_var7: Vec<u8>;
//     let pu_var8: Vec<u8>;
//     let pu_var9: &mut  u32;
//     let pc_var10: Vec<u8>;
//     let mut b_var11: u8;
//     let mut c_var12: u8;
//     let mut i_var13: i32;
//     let lVar14;
//     let mut in_al: u8 = param5;
//     let mut b_var15: u8;
//     let mut b_var16: u8;
//     let mut b_var17: u8;
//     let mut c_var18: u8;
//     let mut b_var19: u16;
//     let mut c_var20: u8;
//     let mut in_cl: u8 = param1;
//     let mut in_ch: u8 = param2;
//     let mut in_dx: u16 = param3;
//     let mut u_var21: u16;
//     let in_bx: Vec<u8>;
//     let mut u_var22: u16;
//     let pc_var23: Vec<u8>;
//     let pi_var24: &mut  i32;
//     let mut i_var25: i32;
//     let unaff_si: Vec<u8>;
//     let pu_var26: Vec<u8>;
//     let unaff_di: Vec<u8>;
//     let pc_var27: Vec<u8>;
//     let mut unaff_es: u16;
//     let mut unaff_cs: u16;
//     let mut unaff_ss: u16;
//     let mut unaff_ds: u16;
//     let mut in_gs: u16 = param4;
//     let mut b_var28: u8;
//     let ac_stack3: Vec<u8>;
//     let local_1: u8;

//     pc_var2 = unaff_di.offset(*ac_stack3.offset(1) as isize);
//     *pc_var2 = *pc_var2 + in_al;

//     pc_var27 = unaff_di.offset(-1);
//     u_var21 = (in_dx + 1);
//     if (u_var21 == 0) {
//         if (u_var21 == 0) {
//             pc_var2 = unaff_di.offset(0x2fff);
//             unsafe {
//                 *pc_var2 = *pc_var2 + in_ch as u8;
//             }
//         } else {
//             unsafe {
//                 pu_var26 = unaff_si.offset(1);

//                 out(*unaff_si as u32, 0);
//                 pb_var3 = in_bx.offset(0x66);
//                 b_var28 = (CARRY1(*pb_var3, in_cl) == 0).into();
//                 *pb_var3 = *pb_var3 + in_cl;
//                 b_var15 = *pb_var3;
//                 if (b_var28 > 0) {
//                     // code_r0x1138006f:
//                     pu_var8 = pu_var26.offset(1);
//                     out((*pu_var26) as u32, u_var21 as u32);
//                     pu_var7 = pc_var27.offset(0x6c);
//                     *pu_var7 = *pu_var7 + b_var28 * (*pu_var7 & 3);
//                     // pc_var2 = in_bx + pu_var8;
//                     *pc_var2 = *(pc_var2.offset(unaff_cs as isize));
//                     b_var19 = (unaff_cs + 0x400 >> 8) + (u_var21 >> 8);
//                     u_var22 = unaff_cs + 0x400 & 0xff;
//                     unaff_cs = u_var22 | b_var19 << 8;
//                     pc_var2 = in_bx;
//                     (*pc_var2) = (*pc_var2) + in_bx;
//                     pc_var2 = in_bx.offset(1);
//                     b_var16 = u_var22;
//                     *pc_var2 = *pc_var2 + b_var16;
//                     pc_var2 = (register0x00000010 + -2 + pu_var8);
//                     *pc_var2 = *pc_var2 + b_var16;
//                     pc_var2 = in_bx + pu_var8;
//                     *pc_var2 = *pc_var2;
//                     pb_var3 = (in_bx + pu_var8);
//                     *pb_var3 = *pb_var3 ^ b_var16;
//                     pc_var2 = (register0x00000010 + -2 + pu_var8);
//                     *pc_var2 = *pc_var2 + in_cl;
//                     pc_var2 = (in_bx + pu_var8 + 0x901);
//                     *pc_var2 = *pc_var2 + in_cl;
//                     pb_var3 = (register0x00000010 + -2 + pu_var8);
//                     b_var15 = *pb_var3;
//                     *pb_var3 = *pb_var3 + b_var16;
//                     pb_var3 = &stack0x0063 + pu_var8;
//                     b_var17 = *pb_var3;
//                     b_var11 = *pb_var3;
//                     *pb_var3 = b_var11 + 0x73 + CARRY1(b_var15, b_var16);
//                     pu_var9 = (pu_var26 + 3);
//                     out(*pu_var8, u_var21);
//                     if (*pb_var3 != 0) {
//                         return;
//                     }
//                     pu_var7 = (pc_var27 + 0x73);
//                     *pu_var7 = *pu_var7
//                         + (0x8c < b_var17 || CARRY1(b_var11 + 0x73, CARRY1(b_var15, b_var16)))
//                             * -(*pu_var7 & 3);
//                     pc_var2 = in_bx + pu_var9;
//                     *pc_var2 = *pc_var2 + b_var16;
//                     pc_var2 = in_bx + pu_var9;
//                     *pc_var2 = *pc_var2 + b_var19;
//                     pc_var2 = (register0x00000010 + -2 + pu_var9);
//                     *pc_var2 = *pc_var2 + b_var19;
//                     pc_var2 = (register0x00000010 + -2 + pu_var9);
//                     *pc_var2 = *pc_var2 + in_cl;
//                     pu_var7 = pu_var26 + 0x482;
//                     *pu_var7 = *pu_var7 + in_cl;
//                     pc_var2 = in_bx + pu_var9;
//                     *pc_var2 = *pc_var2 + b_var16;
//                     pb_var3 = (in_bx + pc_var27 + 0x73);
//                     *pb_var3 = *pb_var3 + 0x73;
//                     b_var15 = *pb_var3;
//                     u_var21 = unaff_cs;
//                     pu_var26 = pu_var9;
//                 } else {
//                     if (!b_var28) {
//                         pc_var2 = in_bx + pu_var26;
//                         *pc_var2 = *pc_var2 + unaff_cs;
//                         unaff_cs = unaff_cs & (in_bx + pu_var26);
//                         pc_var2 = 0x200;
//                         *pc_var2 = *pc_var2 + in_cl;
//                         pc_var2 = in_bx + (unaff_di + 1);
//                         b_var15 = unaff_cs;
//                         *pc_var2 = *pc_var2 + b_var15;
//                         pc_var2 = unaff_di + 1 + (ac_stack3 + 1);
//                         *pc_var2 = *pc_var2 + b_var15;
//                         u_var21 = in_dx + 2;
//                         if (u_var21 != 0) {
//                             // goto code_r0x113800dd;
//                         }

//                         if (u_var21 == 0) {
//                             0x8350 = unaff_cs;
//                             // goto code_r0x113800dd;
//                         }
//                         pu_var7 = pu_var26;
//                         pu_var26 = unaff_si + 1;
//                         out(*pu_var7, 0);
//                         pb_var3 = &stack0x005f + unaff_di;
//                         b_var28 = CARRY1(*pb_var3, b_var15);
//                         *pb_var3 = *pb_var3 + b_var15;
//                         pc_var27 = unaff_di;
//                         // goto code_r0x1138006f;
//                     }
//                 }
//                 unaff_si = pu_var26;
//                 c_var18 = unaff_cs;
//                 if (b_var15 == 0) {
//                     pc_var2 = (register0x00000010 + -2 + unaff_si);
//                     *pc_var2 = *pc_var2 + c_var18;
//                     pc_var2 = in_bx + unaff_si;
//                     *pc_var2 = *pc_var2;
//                     pc_var2 = 0x900;
//                     *pc_var2 = *pc_var2 + c_var18;
//                     // goto code_r0x1138013a;
//                 }
//                 pc_var2 = in_bx + unaff_si;
//                 *pc_var2 = *pc_var2 + c_var18;
//                 pu_var7 = unaff_si;
//                 unaff_si = unaff_si + 1;
//                 unaff_cs = *pu_var7;
//                 pc_var2 = in_bx + unaff_si;
//                 *pc_var2 = *pc_var2 + (u_var21 >> 8);
//             }
//         }
//     } else {
//         unsafe {
//             pb_var3 = (in_bx + unaff_si);
//             *pb_var3 = *pb_var3 ^ unaff_cs;
//             unaff_cs = (in_bx + pc_var27);
//             pu_var7 = (in_bx + unaff_si);
//             *pu_var7 = *pu_var7 | unaff_cs;
//             pc_var2 = (in_bx + unaff_si + -0x80);
//             *pc_var2 = *pc_var2 + u_var21;
//             pu_var7 = unaff_si;
//             unaff_si = unaff_si + 1;
//             out(*pu_var7, u_var21);
//         }
//     }
//     pc_var2 = (register0x00000010 + -2 + unaff_si);
//     c_var20 = (unaff_cs >> 8);
//     unsafe {
//         *pc_var2 = *pc_var2 + c_var20;
//         pc_var2 = (register0x00000010 + -2 + unaff_si);
//         *pc_var2 = *pc_var2 + in_cl;
//         pc_var2 = &stack0x08ff + pc_var27;
//         *pc_var2 = *pc_var2 + in_cl;
//         pb_var3 = (in_bx + unaff_si);
//         b_var15 = *pb_var3;
//         b_var17 = unaff_cs;
//         *pb_var3 = *pb_var3 + b_var17;
//         pc_var2 = (in_bx + unaff_si + 0x65);
//         c_var18 = *pc_var2;
//         c_var12 = *pc_var2;
//         *pc_var2 = c_var12 + 'o' + CARRY1(b_var15, b_var17);
//     }
//     if (SCARRY1(c_var18, 'o') == SCARRY1(c_var12 + 'o', CARRY1(b_var15, b_var17))) {
//         pc_var2 = in_bx + unaff_si;
//         unsafe {
//             *pc_var2 = *pc_var2 + b_var17;
//             in_bx = in_bx + -1;
//             pc_var2 = in_bx + pc_var27;
//             *pc_var2 = *pc_var2 + in_cl;
//             unaff_di = pc_var27 + 2;
//             pc_var2 = unaff_di + (ac_stack3 + 1);
//             *pc_var2 = *pc_var2 + in_cl;
//             pi_var4 = (in_bx + unaff_di);
//             *pi_var4 = *pi_var4 + unaff_cs;
//             pc_var2 = &stack0x834e + unaff_di;
//             *pc_var2 = *pc_var2 + c_var20;
//             pu_var26 = unaff_si;
//             // code_r0x113800dd:
//             pc_var2 = in_bx + pu_var26;
//             c_var18 = unaff_cs;
//             *pc_var2 = *pc_var2 + c_var18;
//             pc_var2 = (in_bx + unaff_di + 0x1f00);
//             *pc_var2 = *pc_var2 + (unaff_cs >> 8);
//             pc_var2 = in_bx + pu_var26;
//             *pc_var2 = *pc_var2 + in_cl;
//             u_var22 = in_bx & 0xff | ((in_bx >> 8) * 0x2) << 8;
//             pi_var4 = (u_var22 + pu_var26);
//             *pi_var4 = *pi_var4 + 1;
//             pu_var1 = &local_1 + pu_var26;
//             *pu_var1 = *pu_var1 + c_var18;
//             pc_var2 = unaff_di + u_var22 + 0x76;
//             *pc_var2 = *pc_var2 + 'a';
//             i_var13 = (pu_var26 + 0x61);
//             pc_var2 = (u_var22 + pu_var26);
//             *pc_var2 = *pc_var2 + c_var18;
//             u_var6 = 0x3000;
//             pc_var2 = (u_var22 + pu_var26);
//             *pc_var2 = *pc_var2 + in_cl;
//             pc_var2 = unaff_di + u_var22 + 0x201;
//             *pc_var2 = *pc_var2 + u_var21;
//             pu_var7 = pu_var26 + i_var13 * 0x3631;
//             c_var18 = u_var6;
//             *pu_var7 = *pu_var7 + c_var18;
//             pu_var5 = (u_var22 + pu_var26);
//             *pu_var5 = *pu_var5;
//             pc_var2 = unaff_di;
//             *pc_var2 = *pc_var2 + c_var18;
//         }
//         // WARNING: Bad instruction - Truncating control flow here
//         halt_baddata();
//     }
//     pc_var2 = in_bx + pc_var27;
//     unsafe {
//         *pc_var2 = *pc_var2 + in_cl;
//     }
//     // code_r0x1138013a:
//     pc_var2 = in_bx + 1;
//     unsafe {
//         *pc_var2 = *pc_var2 + in_cl;
//         pc_var2 = in_bx + unaff_si;
//         b_var15 = unaff_cs;
//         *pc_var2 = *pc_var2 + b_var15;
//     }
//     lVar14 = (&stack0x006f + pc_var27) * 0x536f;
//     i_var25 = lVar14;
//     if (i_var25 != lVar14) {
//         // WARNING: Bad instruction - Truncating control flow here
//         halt_baddata();
//     }
//     unsafe { out(*unaff_si, u_var21) };
//     pc_var23 = in_bx + 1;
//     out((unaff_si + 1), u_var21);
//     out(*(unaff_si + 3), u_var21);
//     if (pc_var23 == 0x0) {
//         // WARNING: Bad instruction - Truncating control flow here
//         halt_baddata();
//     }
//     pu_var26 = unaff_si + 3;
//     out(unaff_si[2], u_var21);
//     pc_var10 = pc_var27 + 1;
//     c_var18 = _in(u_var21);
//     unsafe {
//         *pc_var27 = c_var18;
//         pc_var2 = pc_var23 + pu_var26;
//         *pc_var2 = *pc_var2 + b_var15;
//         pc_var2 = ac_stack3 + pc_var10;
//         c_var18 = (pc_var23 >> 8);
//         *pc_var2 = *pc_var2 + c_var18;
//         pc_var2 = (pc_var23 + pc_var10 + -0x7900);
//         *pc_var2 = *pc_var2 + u_var21;
//         pc_var2 = ac_stack3 + pu_var26;
//         *pc_var2 = *pc_var2 + (u_var21 >> 8);
//         pi_var24 = (pc_var23 & 0xff | (c_var18 * 0x2) << 8);
//         pi_var4 = pi_var24;
//         *pi_var4 = *pi_var4 + 1;
//         pb_var3 = (pi_var24 + pu_var26);
//         b_var17 = *pb_var3;
//         *pb_var3 = *pb_var3 + b_var15;
//         (i_var25 + -2) = unaff_cs;
//         pc_var2 = (pi_var24 + pc_var10 + 0x75);
//         *pc_var2 = *pc_var2 + 'a' + CARRY1(b_var17, b_var15);
//         out(*pu_var26, u_var21);
//         if (*pc_var2 == '\0') {
//             // WARNING: Bad instruction - Truncating control flow here
//             halt_baddata();
//         }
//         if (*pc_var2 == '\0') {
//             // WARNING: Bad instruction - Truncating control flow here
//             halt_baddata();
//         }
//         pc_var2 = (pi_var24 + (unaff_si + 7));
//         *pc_var2 = *pc_var2 + b_var15;
//     }
//     // WARNING: Bad instruction - Truncating control flow here
//     halt_baddata();
// }

// WARNING: Unable to track spacebase fully for stack

// baddata();
//}

pub unsafe fn halt_baddata() {
    todo!()
}

pub fn _in(u_var21: i32) -> u8 {
    todo!()
}

pub fn out(unaff_s_i: u32, arg: u32) -> () {
    todo!()
}

pub unsafe fn bad_fn_1050_525e() {
    let pb_var1: u16;
    let pu_var2: Vec<u8>;
    let pu_var3: &mut  u32;
    let pi_var4: u16;
    let pu_var5: u16;
    let pc_var6: String;
    let pu_var7: &mut  u32;
    let u_var8: u8;
    let lVar9;
    let mut u_var10: u32;
    let mut b_var11: u8;
    let mut c_var12: u8;
    let mut b_var13: u8;
    let mut b_var14: u8;
    let mut i_var15: u16;
    let mut u_var16: u16;
    let mut c_var23: u8;
    let mut u_var17: u16;
    let mut b_var24: u8;
    let mut u_var18: i32;
    let mut u_var19: i32;
    let mut in_eax: u32;
    let mut u_var20: u32;
    let mut u_var21: u32;
    let pc_var22: u16;
    let mut in_cx: u16;
    let mut i_var25: u16;
    let pi_var26: &mut  u16;
    let mut u_var27: i32;
    let mut u_var28: i32;
    let mut c_var29: u8;
    let mut b_var30: u8;
    let mut u_var31: i32;
    let mut u_var32: u16;
    let mut in_edx: u32;
    let mut c_var36: u8;
    let mut u_var33: u32;
    let mut b_var37: u8;
    let mut u_var34: u32;
    let mut u_var35: u32;
    let mut b_var38: u8;
    let mut b_var39: u8;
    let in_bx: u16;
    let pi_var40: u16;
    let mut b_var43: u8;
    let mut pi_var41: u16;
    let mut u_var42: i32;
    let pu_var44: Vec<u8>;
    let pu_var45: &mut  u16;
    let ppu_var46: &mut  &mut  u32;
    let ppu_var47: &mut  &mut  u32;
    let ppu_var48: &mut  &mut  u32;
    let mut u_var49: u16;
    let mut in_ebp: u32;
    let mut u_var50: u32;
    let mut i_var51: u16;
    let pi_var52: u16;
    let pi_var53: &mut  i32;
    let pc_var54: String;
    let pu_var55: &mut  u16;
    let pu_var56: Vec<u8>;
    let pu_var57: &mut  u16;
    let pu_var58: &mut  u32;
    let pu_var59: Vec<u8>;
    let mut in_esi: u32;
    let pu_var60: &mut  u16;
    let mut unaff_di: u16;
    let pi_var61: u16;
    let pi_var62: u16;
    let mut u_var63: i32;
    let pu_var64: &mut  u32;
    let pu_var65: &mut  u32;
    let mut unaff_es: u16;
    let mut u_var66: u16;
    let mut u_var67: u16;
    let mut unaff_ss: u16;
    let unaff_ds: &mut  i32;
    let mut in_fs: u16;
    let mut in_gs: u16;
    let mut in_af: u8;
    let mut b_var68: bool;
    let mut b_var69: bool;
    let mut b_var70: bool;
    let mut in_stack_0000a0fe: u16;
    let mut in_stack_0000a100: i32;
    let in_stack_0000a104: &mut  i32;
    let mut in_stack_0000a106: i32;
    let mut in_stack_0000a108: u16;
    let mut in_stack_0000a10a: i32;
    let in_stack_0000a10c: &mut  i32;
    let mut in_stack_0000a10e: i32;
    let mut in_stack_0000a110: i32;
    let in_stack_0000a114: &mut  i32;
    let mut in_stack_0000a116: i32;
    let mut in_stack_0000a118: u16;
    let mut in_stack_0000a11a: i32;
    let in_stack_0000a11c: &mut  i32;
    let in_stack_0000a11e: &mut  i32;
    let mut in_stack_0000a120: i32;
    let in_stack_0000a124: &mut  i32;
    let mut in_stack_0000a126: i32;
    let mut in_stack_0000a128: u16;
    let in_stack_0000a12a: &mut  i32;
    let mut uStack2: u16;
    let stack0x0103: u16;
    let stack0x006b: u16;
    let stack0xa11e: u16;

    // pi_var26 = &uStack2 as &mut  u16;
    u_var50 = in_ebp & 0xffff0000;
    u_var35 = u_var50 | ZEXT24(uStack2);
    i_var25 = in_cx - 1;
    if (i_var25 != 0) {
        pi_var4 = in_bx;
        todo!();
        //*pi_var4 = *pi_var4 + 1;

        in_stack_0000a0fe = unaff_es;
    }
    i_var51 = in_esi as u16;
    pc_var22 = (in_bx + i_var51 as u16);
    c_var12 = in_eax as u8;

    todo!();
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (in_bx + i_var51 as u16);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (in_bx + i_var51 as u16);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (unaff_di + 0x205);
    // *pc_var22 = *pc_var22 + (i_var25 >> 8);
    pc_var22 = (uStack2 + i_var51);
    // maybe an address of a char?
    c_var36 = (in_edx >> 8) as u8;
    // *pc_var22 = *pc_var22 + c_var36;
    pb_var1 = (&uStack2 + i_var51);
    // todo: de-reference pointer pb_var1
    b_var11 = pb_var1 as u8;
    b_var38 = in_bx as u8;
    // *pb_var1 = *pb_var1 + b_var38;
    b_var11 = c_var12 + CARRY1(b_var11, b_var38) as u8;
    pb_var1 = (in_bx + i_var51);
    // *pb_var1 = *pb_var1 | b_var11;
    pc_var22 = (in_bx + i_var51);
    // *pc_var22 = *pc_var22 + b_var11;
    pc_var22 = (in_bx + i_var51);
    // *pc_var22 = *pc_var22 + b_var11;
    u_var10 = in_eax & 0xffff0000;
    // TODO: fix this
    // i_var15 = (in_bx + i_var51) * -10 + (in_bx + i_var51);
    pc_var22 = (in_bx + i_var51);
    c_var12 = i_var15 as u8;
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (in_bx + i_var51);
    // *pc_var22 = *pc_var22 + c_var12;
    // TODO: set pointer
    //pi_var61 = (unaff_di + 3);
    u_var16 = i_var15 + 2;
    // set one pointer to another
    pi_var4 = pi_var61;
    c_var12 = u_var16 as u8;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (in_bx + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (in_bx + 0x7405);
    // *pc_var22 = *pc_var22 + c_var36;
    pc_var22 = 0x7300;
    // *pc_var22 = *pc_var22 + (in_bx >> 8);
    c_var29 = in_edx as u8;
    u_var34 = in_edx & 0xffff0000;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + 1;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (in_bx + pi_var61 + 0x105);
    // todo: port value
    pc_var22 = pc_var22 + c_var36 as u16 * 0x2;
    pc_var22 = 0xaf00;
    b_var24 = i_var25 as u8;
    //*pc_var22 = *pc_var22 + b_var24;
    pc_var22 = 0x7300;
    // *pc_var22 = *pc_var22 + c_var29;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    c_var23 = (u_var16 >> 8) as u8;
    c_var36 = c_var36 * 0x2 + c_var23;
    pc_var22 = (&uStack2 + pi_var61);
    // *pc_var22 = *pc_var22 + b_var24;
    u_var67 = CONCAT11(0xff, b_var24);
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = &stack0x0103 + pi_var61;
    // *pc_var22 = *pc_var22 + c_var36;
    pc_var22 = (&uStack2 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var29;
    pi_var4 = in_bx;
    // *pi_var4 = *pi_var4 + 0x7300;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = 0x7405;
    // *pc_var22 = *pc_var22 + c_var36;
    pi_var4 = in_bx;
    // *pi_var4 = *pi_var4 + b_var38;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + -1;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (unaff_di + 0x108);
    // *pc_var22 = *pc_var22 + c_var36;
    c_var23 = c_var23 + c_var29;
    pi_var4 = in_bx;
    // *pi_var4 = *pi_var4 + c_var36;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + 1;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = in_bx + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = 0x6401;
    // *pc_var22 = *pc_var22 + -1;
    b_var38 = b_var38 + c_var29;
    pi_var40 = (in_bx & 0xff00 | b_var38 as u16);
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + 1;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + -1;
    c_var12 = c_var12 - 1;
    u_var33 = u_var34 | in_stack_0000a0fe as u32;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + b_var38;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (&uStack2 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = 0x7300;
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = 0x100;
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = 0x100;
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = 0x700;
    // *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var24;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + b_var24;
    c_var12 = c_var12 + in_stack_0000a0fe as u8;
    pi_var4 = (pi_var40 + pi_var61);
    // *pi_var4 = *pi_var4 + CONCAT11(c_var23, c_var12);
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    // *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + b_var38;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = 0x7300;
    //*pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var40;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40;
    // *pi_var4 = *pi_var4 + c_var12;
    b_var14 = (in_stack_0000a0fe >> 8) as u8;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + c_var12 + b_var14;
    // TODOL
    // u_var16 = CONCAT11(c_var23, c_var12 + b_var14) + pi_var40[0x3980];
    // u_var16 = u_var16 & 0xff00 | (u_var16 + *(pi_var40 + 0x3980));
    pu_var5 = (pi_var40 + 0x3980);
    //u_var17 = u_var16 + *pu_var5;
    // b_var11 = (u_var17 - *(pi_var40 + 0x3980)) - CARRY2(u_var16, *pu_var5);
    u_var16 = u_var17 & 0xff00 | b_var11 as u16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var11;
    u_var16 = u_var16 + 0x700;
    b_var13 = u_var16 as u8;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = 0x0;
    //*pc_var22 = *pc_var22 + b_var38;
    pc_var22 = 0x7300;
    //*pc_var22 = *pc_var22 + in_stack_0000a0fe;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var24;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + b_var24;
    pi_var4 = pi_var40;
    // *pi_var4 = *pi_var4 + b_var38;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = 0x7300;
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = 0x7300;
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    //b_var11 = *pi_var4;
    // *pi_var4 = *pi_var4 + b_var14;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16 + CARRY1(b_var11, b_var14);
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var38;
    pc_var22 = 0x0;
    //*pc_var22 = *pc_var22 + b_var24;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var40 + 0x3980;
    // *pi_var4 = *pi_var4 + b_var13;
    //b_var13 = b_var13 | *(pi_var40 + 0x3980);
    b_var11 = (u_var16 >> 8) as u8;
    pi_var52 = 0x7301;
    pi_var41 = 0x7301;
    //out(*0x7300, in_stack_0000a0fe);
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var40;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var24;
    pc_var22 = 0x0;
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var24;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + ((in_bx & 0xff00) >> 8);
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = 0x300;
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = 0x7301;
    //*pc_var22 = *pc_var22 + b_var13;
    i_var25 = CONCAT11(b_var11 + b_var24, b_var13);
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var24);
    pc_var22 = 0x7301;
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = (pi_var40 + 0x7301);
    // *pi_var4 = *pi_var4 + i_var25;
    pi_var4 = (pi_var40 + 0x7301);
    // *pi_var4 = *pi_var4 + i_var25;
    pi_var4 = (pi_var40 + 0x7301);
    // *pi_var4 = *pi_var4 + i_var25;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = (pi_var40 + 0x7301);
    // *pi_var4 = *pi_var4 + i_var25;
    u_var16 = i_var25 + 0x500;
    pc_var22 = (pi_var40 + pi_var61);
    c_var12 = u_var16 as u8;
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    b_var11 = (u_var16 >> 8) as u8;
    u_var21 = (u_var16 & 0xff | (b_var11 + b_var24) << 8) as u16;
    u_var20 = u_var10 | u_var21;
    pc_var22 = (pi_var40 + 0x7301);
    c_var12 = (u_var16 & 0xff);
    //*pc_var22 = *pc_var22 + c_var12 + CARRY1(b_var11, b_var24);
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + c_var12;
    pi_var4 = pi_var61;
    // *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (&uStack2 + pi_var61);
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var40 + 0x7301);
    //*pc_var22 = *pc_var22 + c_var12;
    pb_var1 = (pi_var40 + 0x65);
    b_var11 = *pb_var1;
    b_var13 = (u_var21 >> 8);
    b_var70 = SCARRY1(*pb_var1, b_var13);
    *pb_var1 = *pb_var1 + b_var13;
    if (*pb_var1 == 0) {
        pu_var60 = (in_esi & 0xffff0000 | ZEXT24(&uStack2));
        in_stack_0000a114 = in_stack_0000a104;
        // code_r0x10505706:
        u_var35 = u_var50 | in_stack_0000a100;
        u_var33 = u_var34 | in_stack_0000a106;
        u_var20 = u_var10 | in_stack_0000a10a;
        pi_var41 = (pu_var60 + 2);
        out(*pu_var60, in_stack_0000a106);
        if (b_var70) {
            // code_r0x10505782:
            in_stack_0000a124 = in_stack_0000a114;
            in_stack_0000a128 = in_stack_0000a108;
            in_stack_0000a11c = (pi_var26 + 1);
            u_var8 = _in(u_var33);
            *pi_var26 = u_var8;
            pi_var4 = in_stack_0000a124 + 0x33;
            // *pi_var4 = *pi_var4 & (in_stack_0000a128 >> 8);
            pi_var4 = in_stack_0000a11c;
            // *pi_var4 = *pi_var4 & (u_var20 >> 8);
            pi_var52 = pi_var41;
            // code_r0x105057ab:
            pi_var4 = in_stack_0000a11c;
            b_var11 = (u_var20 >> 8);
            // *pi_var4 = *pi_var4 & b_var11;
            u_var8 = _in(u_var33);
            *in_stack_0000a11c = u_var8;
            pi_var4 = in_stack_0000a124 + 0x33;
            // *pi_var4 = *pi_var4 & (in_stack_0000a128 >> 8);
            pi_var4 = (in_stack_0000a11c + 1);
            // *pi_var4 = *pi_var4 & b_var11;
            u_var67 = in_stack_0000a128;
            pi_var40 = in_stack_0000a124;
            pi_var62 = (in_stack_0000a11c + 1);
            // goto code_r0x105057d9;
        }
        u_var35 = u_var50 | in_stack_0000a120;
        u_var33 = u_var34 | in_stack_0000a126;
        u_var50 = ZEXT24(in_stack_0000a12a);
        pi_var61 = (in_stack_0000a11c + 1);
        pi_var26 = (in_stack_0000a11e + 1);
        out(*in_stack_0000a11e, in_stack_0000a126);
        pi_var40 = in_stack_0000a124;
        if (-1 < in_stack_0000a124) {
            pb_var1 = (in_stack_0000a124 + pi_var26);
            *pb_var1 = *pb_var1 ^ (in_stack_0000a124 >> 8);
            u_var8 = _in(in_stack_0000a126);
            *pi_var61 = u_var8;
            pi_var40 = (in_stack_0000a124 + 1);
            pi_var61 = (in_stack_0000a11c + 3);
            u_var8 = _in(in_stack_0000a126);
            *(in_stack_0000a11c + 1) = u_var8;
            pu_var60 = (in_stack_0000a11e + 0x21);
            *pu_var60 = *pu_var60;
            b_var11 = in_stack_0000a12a & 100;
            pc_var22 = (pi_var40 + pi_var26);
            //*pc_var22 = *pc_var22 + b_var11;
            pc_var22 = (pi_var40 + pi_var26);
            //*pc_var22 = *pc_var22 + b_var11;
            u_var50 = u_var50 & 0xffff0060;
            pc_var22 = (pi_var40 + pi_var26);
            c_var12 = u_var50;
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var26);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var26);
            //*pc_var22 = *pc_var22 + c_var12;
            in_stack_0000a11c = pi_var40;
            in_stack_0000a11e = in_stack_0000a124;
        }
        u_var20 = u_var10 | u_var50;
        pc_var22 = (pi_var40 + pi_var26);
        c_var12 = u_var50;
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + pi_var26);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + pi_var26);
        //*pc_var22 = *pc_var22 + c_var12;
        u_var67 = in_stack_0000a128;
        in_stack_0000a12a = in_stack_0000a124;
        // PTR_LOOP_1050_574c:
        pi_var41 = pi_var26;
        pc_var22 = (pi_var40 + pi_var41);
        b_var11 = u_var20;
        //*pc_var22 = *pc_var22 + b_var11;
        pc_var22 = (pi_var40 + pi_var41);
        //*pc_var22 = *pc_var22 + b_var11;
        pc_var22 = (pi_var40 + pi_var41);
        //*pc_var22 = *pc_var22 + b_var11;
        pc_var22 = (pi_var40 + pi_var41);
        //*pc_var22 = *pc_var22 + b_var11;
        pb_var1 = (pi_var40 + pi_var41);
        b_var70 = CARRY1(*pb_var1, b_var11);
        *pb_var1 = *pb_var1 + b_var11;
        b_var68 = *pb_var1 == 0;
        // code_r0x10505758:
        c_var12 = u_var20;
        pi_var52 = pi_var41;
        if (!b_var70) {
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            // goto code_r0x105057d3;
        }
        pi_var62 = pi_var61 + 1;
        u_var66 = u_var33;
        i_var25 = _in(u_var66);
        *pi_var61 = i_var25;
        if (b_var70) {
            // code_r0x105057c2:
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            // goto code_r0x105057c6;
        }
        if (b_var70) {
            pi_var52 = pi_var41 + 1;
            out(*pi_var41, u_var66);
            if (!b_var68) {
                // goto code_r0x105057d6;
            }
            pi_var4 = pi_var62;
            b_var11 = *pi_var4;
            b_var13 = (u_var20 >> 8);
            // *pi_var4 = *pi_var4 + b_var13;
            c_var23 = *pi_var4;
            b_var70 = *pi_var4 == '\0';
            pi_var26 = in_stack_0000a11c;
            pi_var41 = in_stack_0000a11e;
            if (!CARRY1(b_var11, b_var13)) {
                // goto code_r0x10505770;
            }
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            c_var23 = *pc_var22;
            b_var70 = *pc_var22 == '\0';
            if (!b_var70) {
                // goto code_r0x1050576e;
            }
            // goto code_r0x105057e0;
        }
        pi_var52 = pi_var41 + 1;
        out(*pi_var41, u_var66);
        pi_var4 = pi_var62;
        pi_var62 = pi_var61 + 2;
        i_var25 = _in(u_var66);
        // *pi_var4 = i_var25;
        pb_var1 = (pi_var40 + pi_var52);
        b_var11 = (u_var33 >> 8);
        *pb_var1 = *pb_var1 & b_var11;
        b_var13 = (pi_var40 >> 8);
        if (-1 < *pb_var1) {
            pb_var1 = (pi_var40 + pi_var52);
            *pb_var1 = *pb_var1 ^ b_var13;
            b_var68 = *pb_var1 < '\0';
            b_var70 = *pb_var1 == 0;
            pi_var4 = pi_var62;
            pi_var62 = (pi_var61 + 5);
            u_var8 = _in(u_var66);
            // *pi_var4 = u_var8;
            if (!b_var70) {
                // goto code_r0x1050579a;
            }

            // goto code_r0x1050580c;
        }
        pb_var1 = (pi_var41 + 0x71);
        *pb_var1 = *pb_var1 & b_var11;
        pb_var1 = (pi_var40 + pi_var52);
        *pb_var1 = *pb_var1 & b_var11;
        if (-1 < *pb_var1) {
            pb_var1 = (pi_var40 + pi_var52);
            *pb_var1 = *pb_var1 ^ b_var13;
            pi_var4 = pi_var62;
            pi_var62 = (pi_var61 + 5);
            u_var8 = _in(u_var66);
            // *pi_var4 = u_var8;
            // goto code_r0x105057c2;
        }
        // code_r0x105057e4:
        pc_var22 = (pi_var40 + pi_var52);
        c_var12 = u_var20;
        //*pc_var22 = *pc_var22 + c_var12;
        pi_var4 = pi_var52;
        c_var23 = (u_var20 >> 8);
        // *pi_var4 = *pi_var4 + c_var23;
        pi_var4 = pi_var52;
        // *pi_var4 = *pi_var4 + c_var23;
        pc_var22 = (pi_var40 + pi_var52);
        //*pc_var22 = *pc_var22 - c_var12;
        pc_var22 = (pi_var40 + pi_var52);
        //*pc_var22 = *pc_var22 - c_var12;
        pi_var52 = (pi_var52 + 1);
        pc_var22 = (u_var35 + pi_var52);
        c_var12 = (u_var33 >> 8);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (u_var35 + pi_var52);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = 0x1e00;
        //*pc_var22 = *pc_var22 + pi_var40;
        pc_var22 = 0x0;
        //*pc_var22 = *pc_var22 + pi_var40;
        // code_r0x105057ff:
        pi_var4 = pi_var52;
        // *pi_var4 = *pi_var4 + (u_var20 >> 8);
        pc_var22 = (pi_var40 + pi_var52);
        c_var12 = u_var20;
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + pi_var52);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + pi_var52);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + pi_var52);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + pi_var52);
        //*pc_var22 = *pc_var22 + c_var12;
    } else {
        pi_var26 = 0x7301;
        if (!CARRY1(b_var11, b_var13)) {
            // code_r0x1050574d:
            pi_var41 = pi_var26;
            pc_var22 = (pi_var40 + pi_var41);
            c_var12 = u_var20;
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var41);
            //*pc_var22 = *pc_var22 + c_var12;
            pi_var4 = pi_var40;
            b_var11 = (u_var33 >> 8);
            b_var70 = CARRY1(*pi_var4, b_var11);
            // *pi_var4 = *pi_var4 + b_var11;
            b_var68 = *pi_var4 == '\0';
            // goto code_r0x10505758;
        }
        if (*pb_var1 == 0) {
            // code_r0x1050574b:
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + u_var20;
            pi_var26 = pi_var52;
            // goto code_r0x1050574d;
        }
        pb_var1 = 0x7362;
        *pb_var1 = *pb_var1 & b_var13;
        if (*pb_var1 == 0) {
            // goto PTR_LOOP_1050_574c;
        }
        pb_var1 = (pi_var40 + 0x69);
        b_var70 = false;
        *pb_var1 = *pb_var1 & b_var14;
        b_var68 = *pb_var1 == 0;
        if (b_var68) {
            // goto code_r0x10505758;
        }
        pi_var52 = 0x7303;
        out(0x7301, in_stack_0000a0fe);
        if (b_var68) {
            pb_var1 = (pi_var40 + 0x736f);
            *pb_var1 = *pb_var1 & b_var14;
            u_var35 = u_var50 | in_stack_0000a100;
            u_var33 = u_var34 | in_stack_0000a106;
            u_var20 = u_var10 | in_stack_0000a10a;
            pi_var52 = (&uStack2 + 1);
            out(uStack2, in_stack_0000a106);
            if (*pb_var1 != 0) {
                pc_var22 = &stack0x006b + in_stack_0000a100;
                c_var12 = (in_stack_0000a10a >> 8);
                b_var70 = SCARRY1(*pc_var22, c_var12);
                //*pc_var22 = *pc_var22 + c_var12;
                pu_var60 = (in_esi & 0xffff0000 | in_stack_0000a10e);
                in_stack_0000a108 = in_stack_0000a118;
                pi_var26 = in_stack_0000a10c;
                in_stack_0000a10a = in_stack_0000a11a;
                in_stack_0000a106 = in_stack_0000a116;
                in_stack_0000a100 = in_stack_0000a110;
                // goto code_r0x10505706;
            }
            pc_var22 = (in_stack_0000a104 + pi_var52);
            c_var12 = in_stack_0000a10a;
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (in_stack_0000a104 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (in_stack_0000a104 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (in_stack_0000a104 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (in_stack_0000a104 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (in_stack_0000a104 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            u_var67 = in_stack_0000a108;
            pi_var40 = in_stack_0000a104;
            pi_var61 = &uStack2;
            // goto code_r0x1050574b;
        }
        pc_var22 = (pi_var40 + 0x7303);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var40 + 0x7303);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = 0x7375;
        //*pc_var22 = *pc_var22 + b_var14;
        c_var23 = *pc_var22;
        b_var70 = *pc_var22 == '\0';
        pi_var62 = pi_var61;
        // code_r0x1050576e:
        pi_var61 = pi_var62;
        pi_var26 = in_stack_0000a11c;
        pi_var41 = in_stack_0000a11e;
        if (!b_var70) {
            // code_r0x105057d3:
            pc_var22 = (pi_var40 + pi_var52);
            c_var12 = u_var20;
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pi_var62 = pi_var61;
            // code_r0x105057d9:
            pc_var22 = (pi_var40 + pi_var52);
            b_var11 = u_var20;
            //*pc_var22 = *pc_var22 + b_var11;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + b_var11;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + b_var11;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + b_var11;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + b_var11;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + b_var11;
            pi_var4 = pi_var52;
            c_var12 = (u_var20 >> 8);
            // *pi_var4 = *pi_var4 + c_var12;
            pi_var4 = pi_var52;
            // *pi_var4 = *pi_var4 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            c_var12 = (u_var67 >> 8);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = u_var35;
            //*pc_var22 = *pc_var22 + b_var11;
            b_var11 = b_var11 ^ *(pi_var40 + pi_var52) ^ *(pi_var40 + pi_var52);
            u_var20 = u_var20 & 0xffffff00 | b_var11;
            pc_var22 = 0x1e00;
            //*pc_var22 = *pc_var22 + pi_var40;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + b_var11;
            in_stack_0000a11c = unaff_ds;
            // goto code_r0x105057ff;
        }
        // code_r0x10505770:
        b_var68 = c_var23 < '\0';
        in_stack_0000a11c = pi_var26;
        in_stack_0000a11e = pi_var41;
        if (!b_var68) {
            pb_var1 = (pi_var40 + pi_var52);
            *pb_var1 = *pb_var1 ^ (pi_var40 >> 8);
            b_var68 = *pb_var1 < '\0';
            b_var70 = *pb_var1 == 0;
            pi_var4 = pi_var62;
            pi_var62 = (pi_var62 + 1);
            u_var66 = u_var33;
            u_var8 = _in(u_var66);
            // *pi_var4 = u_var8;
            if (!b_var68) {
                u_var8 = _in(u_var66);
                *pi_var62 = u_var8;
                out(*pi_var52, u_var66);
                u_var35 = u_var35 & 0xffff0000 | in_stack_0000a120;
                u_var33 = u_var33 & 0xffff0000 | in_stack_0000a126;
                u_var20 = u_var20 & 0xffff0000 | ZEXT24(in_stack_0000a12a);
                pi_var4 = pi_var26;
                // *pi_var4 = *pi_var4 & (ZEXT24(in_stack_0000a12a) >> 8);
                in_stack_0000a108 = in_stack_0000a128;
                in_stack_0000a114 = in_stack_0000a124;
                // goto code_r0x10505782;
            }
        }
        // code_r0x1050579a:
        if (!b_var70) {
            // goto code_r0x105057ff;
        }
        if (b_var68) {
            // code_r0x105057c6:
            pc_var22 = (pi_var40 + pi_var52);
            c_var12 = u_var20;
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            // code_r0x105057d6:
            pc_var22 = (pi_var40 + pi_var52);
            c_var12 = u_var20;
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + c_var12;
            // code_r0x105057e0:
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + u_var20;
            pc_var22 = (pi_var40 + pi_var52);
            //*pc_var22 = *pc_var22 + u_var20;
            // goto code_r0x105057e4;
        }
        pb_var1 = (pi_var40 + pi_var52);
        *pb_var1 = *pb_var1 ^ (pi_var40 >> 8);
        b_var11 = *pb_var1;
        b_var13 = *pb_var1;
        pi_var4 = pi_var62;
        pi_var62 = (pi_var62 + 1);
        u_var66 = u_var33;
        u_var8 = _in(u_var66);
        // *pi_var4 = u_var8;
        if (b_var11 < '\0') {
            // goto code_r0x105057c6;
        }
        if (b_var13 == 0) {
            u_var8 = _in(u_var66);
            *pi_var62 = u_var8;
            out(*pi_var52, u_var66);
            u_var35 = u_var35 & 0xffff0000 | in_stack_0000a120;
            u_var33 = u_var33 & 0xffff0000 | in_stack_0000a126;
            u_var20 = u_var20 & 0xffff0000 | ZEXT24(in_stack_0000a12a);
            pi_var52 = in_stack_0000a11e;
            // goto code_r0x105057ab;
        }
        in_af = 9 < (u_var20 & 0xf) | in_af;
        b_var11 = u_var20 + in_af * '\x06';
        u_var20 = u_var20 & 0xffffff00
            | (b_var11 + (0x90 < (b_var11 & 0xf0) | in_af * (0xf9 < b_var11)) * '`');
    }
    // code_r0x1050580c:
    pb_var1 = (pi_var40 + pi_var52);
    b_var11 = *pb_var1;
    b_var13 = u_var20;
    *pb_var1 = *pb_var1 + b_var13;
    u_var50 = (pi_var40 + 0x28);
  // u_var66 = (u_var50  >> 0x10);
    u_var50 = u_var50 & 0xffff;
    u_var34 = u_var33 & 0xffff0000 | u_var50;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var13);
    pi_var4 = pi_var40;
    c_var12 = (u_var20 >> 8);
    *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var62;
    b_var30 = u_var50;
    *pi_var4 = *pi_var4 + b_var30;
    i_var25 = u_var35;
    pc_var22 = (i_var25 + pi_var52);
    b_var14 = u_var67;
    //*pc_var22 = *pc_var22 + b_var14;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var14;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (i_var25 + pi_var52);
    b_var43 = (pi_var40 >> 8);
    //*pc_var22 = *pc_var22 + b_var43;
    pc_var22 = (pi_var40 + pi_var62);
    b_var38 = (u_var67 >> 8);
    //*pc_var22 = *pc_var22 + b_var38;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var38;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var14;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + c_var12;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var14;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = 0x600;
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var52;
    b_var39 = pi_var40;
    *pi_var4 = *pi_var4 + b_var39;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = 0xb00;
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var30;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var30;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var30;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var39;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var30;
    pc_var22 = 0x2600;
    //*pc_var22 = *pc_var22 + c_var12;
    pc_var22 = 0xa00;
    b_var37 = (u_var50 >> 8);
    //*pc_var22 = *pc_var22 + b_var37;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var14;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var14;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var39;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var39;
    pi_var4 = pi_var62;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + b_var37;
    pc_var22 = 0x2600;
    //*pc_var22 = *pc_var22 + b_var37;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + b_var37;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var30;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pc_var22 = 0xe00;
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = 0x3700;
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = 0x1a00;
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pi_var4 = pi_var40;
    *pi_var4 = *pi_var4 + b_var30;
    pc_var22 = 0x1700;
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var39;
    pc_var22 = 0x1200;
    //*pc_var22 = *pc_var22 + b_var14;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var40 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var52;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var30;
    u_var20 = u_var20 & 0xffff0000;
    pc_var22 = (pi_var40 + pi_var62);
    b_var13 = in_stack_0000a11c;
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var30);
    pb_var1 = 0x5058;
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var30;
    pc_var22 = (i_var25 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var30);
    pi_var4 = pi_var52;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var39;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var39);
    pb_var1 = 0x5058;
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var39;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var39);
    pb_var1 = 0x5058;
    b_var11 = *pb_var1;
    b_var24 = (ZEXT24(in_stack_0000a11c) >> 8);
    *pb_var1 = *pb_var1 + b_var24;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var24);
    pb_var1 = (pi_var40 + pi_var52);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var38;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var38);
    pb_var1 = (pi_var40 + pi_var52);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var37;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var37);
    pb_var1 = (i_var25 + pi_var52);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var37;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var37);
    pb_var1 = (i_var25 + pi_var52);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var43;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var43);
    pb_var1 = (pi_var52 + i_var25 + 0x58);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var13);
    pb_var1 = (pi_var52 + i_var25 + 0x58);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var14;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var14);
    pi_var4 = pi_var52 + 0x2c;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var14;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var14);
    pi_var4 = pi_var52 + 0x2c;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var30;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var30);
    pi_var4 = pi_var52 + 0x2c;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var39;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var39);
    pi_var4 = pi_var52 + 0x2c;
    b_var11 = *pi_var4;
    b_var24 = (ZEXT24(in_stack_0000a11c) >> 8);
    *pi_var4 = *pi_var4 + b_var24;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var24);
    pb_var1 = (i_var25 + 0x58);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var24;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var24);
    pb_var1 = (i_var25 + 0x58);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var38;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var38);
    pb_var1 = (pi_var40 + pi_var52 + 0x58);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var37;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var37);
    pb_var1 = (pi_var40 + pi_var52 + 0x58);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var43;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var43);
    pb_var1 = (pi_var40 + pi_var52 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var13);
    pb_var1 = (pi_var40 + pi_var52 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var14;
    pc_var22 = (pi_var40 + pi_var62);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var14);
    pb_var1 = (pi_var52 + i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var14;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var14);
    pb_var1 = (pi_var52 + i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var30;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var30);
    pb_var1 = (pi_var52 + i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var39;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var39);
    pb_var1 = (pi_var52 + i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var24;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var24);
    pb_var1 = (pi_var52 + i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var38;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var38);
    pb_var1 = (pi_var52 + i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var37;
    pc_var22 = (i_var25 + pi_var52);
    //*pc_var22 = *pc_var22 + b_var13 + CARRY1(b_var11, b_var37);
    pb_var1 = (i_var25 + 0x5058);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var37;
    pi_var4 = pi_var52;
    *pi_var4 = *pi_var4 + b_var13 + CARRY1(b_var11, b_var37);
    pc_var22 = (pi_var62 + i_var25 + 0x43);
    //*pc_var22 = *pc_var22 + b_var30;
    pi_var26 = ((pi_var40 + pi_var62 + 0x6e) * 0x7570);
    i_var25 = pi_var52[0x36] * 0x6c42;
    u_var35 = u_var35 & 0xffff0000;
    b_var70 = in_stack_0000a11c < 0x303e;
    i_var15 = u_var50;
    if (-1 < (in_stack_0000a11c + -0x181f)) {
        pb_var1 = (pi_var40 + pi_var52);
        *pb_var1 = *pb_var1 ^ b_var43;
        u_var8 = _in(i_var15);
        *pi_var62 = u_var8;
        i_var25 = pi_var52[0x36];
        pi_var61 = pi_var62 + 1;
        u_var8 = _in(i_var15);
        *(pi_var62 + 1) = u_var8;
        out(*pi_var52, i_var15);
        out(*(pi_var52 + 1), i_var15);
        pc_var22 = (pi_var61 + i_var25 * 0x6f43 + 0x43);
        //*pc_var22 = *pc_var22 + b_var30;
        lVar9 = (pi_var52 + 0x6f) * 0x6552;
        i_var25 = lVar9;
        b_var70 = i_var25 != lVar9;
        out((pi_var52 + 3), i_var15);
        pi_var62 = (pi_var62 + 3);
        u_var8 = _in(i_var15);
        *pi_var61 = u_var8;
        pu_var59 = (pi_var52 + 7);
        out((pi_var52 + 5), i_var15);
        pi_var52 = pi_var52 + 4;
        out(*pu_var59, i_var15);
        in_stack_0000a104 = pi_var40;
    }
    pi_var61 = (pi_var40 + 1);
    u_var50 = u_var35 | i_var25 - 1;
    out(*pi_var52, i_var15);
    b_var11 = (in_stack_0000a11c >> 8);
    pi_var41 = in_stack_0000a11c;
    if (b_var70 || i_var25 - 1 == 0) {
        u_var21 = (u_var20 | ZEXT24(in_stack_0000a11c + -0x181f)) ^ 0x756c;
        pb_var1 = (pi_var62 + i_var25 + 0x6b);
        b_var13 = (u_var21 >> 8);
        b_var70 = CARRY1(*pb_var1, b_var13);
        *pb_var1 = *pb_var1 + b_var13;
        b_var68 = *pb_var1 < '\0';
        pi_var53 = (pi_var52 + 3);
        out(*(pi_var52 + 1), i_var15);
        // code_r0x10505a1f:
        pu_var5 = (pi_var53 + 0x37);
        *pu_var5 = *pu_var5 + b_var70 * ((u_var50 & 3) - (*pu_var5 & 3));
        if (!b_var68) {
            // code_r0x10505a44:
            pb_var1 = (pi_var61 + pi_var53);
            *pb_var1 = *pb_var1 & u_var21;
            // code_r0x10505a48:
            u_var21 = u_var21 & 0xffff2460;
            pb_var1 = (pi_var61 + pi_var53);
            *pb_var1 = *pb_var1 & u_var21;
            pi_var62 = pi_var41;
            // code_r0x10505a52:
            u_var50 = u_var50 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
            u_var34 = u_var34 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
            u_var21 = u_var21 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
            pi_var61 = in_stack_0000a11c;
            pi_var53 = in_stack_0000a11c;
            // goto code_r0x10505a76;
        }
        pb_var1 = (pi_var61 + pi_var53 + 0x25);
        b_var13 = (pi_var61 >> 8);
        *pb_var1 = *pb_var1 ^ b_var13;
        pb_var1 = (pi_var61 + pi_var53);
        *pb_var1 = *pb_var1 ^ b_var13;
        b_var13 = *pb_var1;
        pi_var40 = (pi_var62 + 1);
        u_var8 = _in(u_var34);
        *pi_var62 = u_var8;
        if (b_var13 < '\0') {
            pb_var1 = (pi_var61 + pi_var40 + 0x6c);
            b_var70 = CARRY1(*pb_var1, u_var21);
            *pb_var1 = *pb_var1 + u_var21;
        } else {
            pb_var1 = (pi_var53 + 0x75);
            *pb_var1 = *pb_var1 & (u_var21 >> 8);
            // code_r0x10505a2f:
            i_var25 = _in(u_var34);
            *pi_var40 = i_var25;
            u_var50 = u_var50 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
            u_var34 = u_var34 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
            pb_var1 = (in_stack_0000a11c * 2);
            *pb_var1 = *pb_var1 & b_var11;
            b_var70 = false;
            u_var21 = u_var21 & 0xffff0000 | ZEXT24(in_stack_0000a11c) & 0xffff0073;
            pi_var26 = (in_stack_0000a11c + 1);
            pi_var40 = (pi_var41 + 1);
            u_var8 = _in(in_stack_0000a11c);
            *pi_var41 = u_var8;
            pi_var61 = in_stack_0000a11c;
            pi_var53 = in_stack_0000a11c;
        }
        pi_var62 = (pi_var40 + 1);
        u_var8 = _in(u_var34);
        *pi_var40 = u_var8;
        pi_var4 = pi_var53;
        pi_var53 = pi_var53 + 1;
        out(*pi_var4, u_var34);
        pu_var5 = (u_var50 + pi_var53);
        *pu_var5 = *pu_var5 + b_var70 * ((pi_var62 & 3) - (*pu_var5 & 3));
        u_var21 = u_var21 & 0xffff0073;
        pc_var22 = (pi_var61 + pi_var53);
        c_var12 = u_var21;
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var61 + pi_var53);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var61 + pi_var53);
        //*pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var61 + pi_var53);
        //*pc_var22 = *pc_var22 + c_var12;
        in_stack_0000a11c = pi_var26;
    } else {
        u_var50 = u_var35 | ZEXT24(pi_var40);
        u_var34 = u_var33 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
        u_var10 = ZEXT24(in_stack_0000a11c);
        u_var21 = u_var20 | u_var10;
        b_var24 = (u_var10 >> 8);
        pi_var61 = in_stack_0000a11c;
        pi_var26 = in_stack_0000a104;
        if (!b_var70) {
            // code_r0x105059da:
            u_var16 = u_var50 - 1;
            u_var50 = u_var50 & 0xffff0000 | u_var16;
            pi_var53 = pi_var26 + 1;
            out(*pi_var26, in_stack_0000a11c);
            if (b_var70 || u_var16 == 0) {
                u_var8 = _in(in_stack_0000a11c);
                *pi_var40 = u_var8;
                // goto code_r0x10505a44;
            }
            pb_var1 = (pi_var40 + 0x69);
            b_var70 = CARRY1(*pb_var1, b_var11);
            *pb_var1 = *pb_var1 + b_var11;
            b_var14 = *pb_var1;
            out(*pi_var53, in_stack_0000a11c);
            pi_var53 = (pi_var26 + 5);
            out((pi_var26 + 3), in_stack_0000a11c);
            // code_r0x105059e5:
            i_var25 = _in(in_stack_0000a11c);
            *pi_var40 = i_var25;
            pi_var62 = (pi_var40 + 3);
            u_var8 = _in(in_stack_0000a11c);
            *(pi_var40 + 1) = u_var8;
            if (b_var14 != 0) {
                if (!b_var70) {
                    out(*pi_var53, in_stack_0000a11c);
                    out((pi_var53 + 1), in_stack_0000a11c);
                    pi_var4 = pi_var40 + 0x36;
                    *pi_var4 = *pi_var4 & b_var11;
                    out(*(pi_var53 + 3), in_stack_0000a11c);
                    pb_var1 = (pi_var62 + u_var50 + 0x43);
                    b_var14 = *pb_var1;
                    *pb_var1 = *pb_var1 + b_var13;
                    pi_var26 = pi_var53 + 3;
                    out(pi_var53[2], in_stack_0000a11c);
                    pi_var53 = pi_var53 + 4;
                    out(*pi_var26, in_stack_0000a11c);
                    if (CARRY1(b_var14, b_var13) || *pb_var1 == 0) {
                        pc_var22 = (pi_var61 + pi_var53);
                        //*pc_var22 = *pc_var22 + b_var13;
                        pc_var22 = (pi_var61 + pi_var53);
                        //*pc_var22 = *pc_var22 + b_var13;
                        pi_var4 = pi_var62;
                        *pi_var4 = *pi_var4 + b_var24;
                        // goto code_r0x10505a6f;
                    }
                    pc_var22 = (pi_var61 + pi_var53);
                    //*pc_var22 = *pc_var22 + b_var13;
                    u_var21 = u_var20
                        | CONCAT11(
                            ((ZEXT24(in_stack_0000a11c) | 0xa0d) >> 8) | *pi_var62,
                            (ZEXT24(in_stack_0000a11c) | 0xa0d),
                        );
                }
                b_var70 = false;
                u_var21 = u_var21 & 0xffff6c35;
                b_var68 = false;
                pi_var26 = in_stack_0000a11c;
                pi_var41 = pi_var61;
                // goto code_r0x10505a1f;
            }
            pb_var1 = (pi_var61 + pi_var53);
            *pb_var1 = *pb_var1 & b_var13;
            pi_var62 = pi_var61;
            // goto code_r0x10505a52;
        }
        if (i_var15 == -1) {
            pb_var1 = (in_stack_0000a104 + 0x75);
            *pb_var1 = *pb_var1 & b_var24;
            // goto code_r0x10505a2f;
        }
        pb_var1 = (in_stack_0000a11c + in_stack_0000a104);
        b_var70 = false;
        *pb_var1 = *pb_var1 & b_var11;
        b_var14 = *pb_var1;
        pi_var53 = in_stack_0000a104;
        if (*pb_var1 < '\0') {
            //goto code_r0x105059e5;
        }
        pb_var1 = (in_stack_0000a11c + in_stack_0000a104);
        *pb_var1 = *pb_var1 ^ b_var11;
        pu_var59 = (pi_var40 + 1);
        u_var8 = _in(in_stack_0000a11c);
        *pi_var40 = u_var8;
        pc_var22 = (pi_var40 + pu_var59 + 0x43);
        *pc_var22 = *pc_var22 + b_var13;
        lVar9 = pi_var40[0x32] * 0x73;
        i_var25 = lVar9;
        pi_var41 = (in_stack_0000a11c + 1);
        u_var16 = i_var25 - 1;
        u_var50 = u_var35 | u_var16;
        pi_var26 = in_stack_0000a104 + 1;
        out(*in_stack_0000a104, in_stack_0000a11c);
        if (i_var25 == lVar9) {
            pi_var40 = pi_var40 + 1;
            u_var8 = _in(in_stack_0000a11c);
            *pu_var59 = u_var8;
            pb_var1 = (pi_var41 + pi_var26);
            b_var70 = CARRY1(*pb_var1, b_var13);
            *pb_var1 = *pb_var1 + b_var13;
            pi_var61 = in_stack_0000a11c + 1;
            // goto code_r0x105059da;
        }
        pb_var1 = (i_var25 + 0x6e);
        *pb_var1 = *pb_var1 & b_var24;
        b_var11 = *pb_var1;
        u_var8 = _in(in_stack_0000a11c);
        *pu_var59 = u_var8;
        pi_var62 = (pi_var40 + 3);
        u_var8 = _in(in_stack_0000a11c);
        *(pi_var40 + 1) = u_var8;
        pi_var53 = in_stack_0000a104 + 2;
        out(*pi_var26, in_stack_0000a11c);
        if (b_var11 != 0) {
            c_var12 = *(pi_var41 + pi_var53);
            pi_var4 = in_stack_0000a11c + -0x30f6;
            b_var11 = *pi_var4;
            *pi_var4 = *pi_var4 + pi_var41;
            pu_var5 = (in_stack_0000a104 + -0x62e9);
            u_var17 = CARRY1(b_var11, pi_var41);
            u_var18 = pi_var41 + *pu_var5;
            pi_var61 = (u_var18 + u_var17);
            u_var21 = u_var20
                | (b_var13 + c_var12)
                    + (u_var16 + pi_var53)
                    + (CARRY2(pi_var41, *pu_var5) || CARRY2(u_var18, u_var17));
            in_stack_0000a11c = (in_stack_0000a11c + 1);
            // goto code_r0x10505a6f;
        }
        pi_var4 = pi_var62;
        b_var11 = *pi_var4;
        *pi_var4 = *pi_var4 + b_var24;
        pi_var61 = pi_var41;
        if (CARRY1(b_var11, b_var24)) {
            u_var21 = u_var20 | u_var10 & 0xffff646c;
            pb_var1 = (pi_var41 + pi_var53);
            *pb_var1 = *pb_var1 & (u_var10 & 0xffff646c);
            pi_var41 = in_stack_0000a11c;
            // goto code_r0x10505a48;
        }
        pc_var22 = (pi_var41 + pi_var53);
        *pc_var22 = *pc_var22 + b_var13;
        u_var8 = *(u_var16 + pi_var62);
        u_var34 = CONCAT31(
            (u_var34 >> 8) & 0xffff00 | (*(u_var16 + pi_var62) >> 8),
            u_var8,
        );
        *(u_var16 + pi_var62) = u_var8;
        // code_r0x10505a76:
        (u_var50 + pi_var62) = u_var34;
        u_var21 = u_var21 & 0xffffff00 | (u_var21 + *(pi_var61 + pi_var53));
        pc_var22 = (pi_var61 + pi_var53);
        *pc_var22 = *pc_var22 + -0x71;
    }
    // code_r0x10505a6f:
    u_var19 = u_var50;
    pb_var1 = (pi_var62 + u_var19 + 0x8a13);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + in_stack_0000a11c;
    pu_var5 = (pi_var61 + pi_var53 + -0x76ed);
    u_var16 = CARRY1(b_var11, in_stack_0000a11c);
    u_var17 = in_stack_0000a11c + *pu_var5;
    u_var27 = u_var17 + u_var16;
    u_var18 = u_var21
        + (u_var19 + pi_var53)
        + (CARRY2(in_stack_0000a11c, *pu_var5) || CARRY2(u_var17, u_var16));
    pi_var4 = pi_var53;
    b_var13 = u_var18;
    *pi_var4 = *pi_var4 + b_var13;
    pb_var1 = (pi_var61 + pi_var53 + -0x7100);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (u_var19 + 0x8c13);
    u_var16 = u_var27 + *pu_var5;
    u_var28 = u_var16
        + CARRY1(b_var11, b_var13)
        + (pi_var62 + 0x113)
        + (CARRY2(u_var27, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var13)));
    pi_var4 = pi_var53;
    *pi_var4 = *pi_var4 + u_var34;
    pb_var1 = (pi_var61 + pi_var62 + -0x6d00);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (pi_var53 + u_var19 + 0x9013);
    u_var16 = u_var34 + *pu_var5;
    u_var17 = u_var16
        + CARRY1(b_var11, b_var13)
        + (pi_var61 + pi_var62 + 0x113)
        + (CARRY2(u_var34, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var13)));
    pi_var4 = pi_var53;
    *pi_var4 = *pi_var4 + u_var17;
    pb_var1 = (pi_var53 + u_var19 + 0x9700);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (u_var19 + 0x9413);
    u_var16 = u_var17 + *pu_var5;
    u_var31 = u_var16
        + CARRY1(b_var11, b_var13)
        + (pi_var62 + 0x213)
        + (CARRY2(u_var17, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var13)));
    pi_var4 = pi_var53;
    *pi_var4 = *pi_var4 + b_var13;
    pb_var1 = 0x9b00;
    b_var11 = *pb_var1;
    b_var24 = (pi_var61 >> 8);
    *pb_var1 = *pb_var1 + b_var24;
    pu_var5 = (pi_var53 + u_var19 + 0x9813);
    u_var16 = CARRY1(b_var11, b_var24);
    u_var17 = pi_var61 + *pu_var5;
    i_var25 = u_var17 + u_var16;
    u_var27 = i_var25
        + (pi_var62 + i_var25 + 0x213)
        + (CARRY2(pi_var61, *pu_var5) || CARRY2(u_var17, u_var16));
    pi_var4 = pi_var53;
    *pi_var4 = *pi_var4 + b_var13;
    pb_var1 = (u_var27 + pi_var62);
    b_var70 = CARRY1(*pb_var1, b_var13);
    *pb_var1 = *pb_var1 + b_var13;
    u_var18 = u_var18 & 0xff;
    u_var17 = u_var18
        | ((*pb_var1 < '\0') << 7
            | (*pb_var1 == 0) << 6
            | in_af << 4
            | ((POPCOUNT(*pb_var1) & 1) == 0) << 2
            | 2
            | b_var70)
            << 8;
    pu_var5 = (u_var19 + 0x9c13);
    u_var16 = u_var27 + *pu_var5;
    u_var42 = u_var16
        + b_var70
        + (pi_var62 + 0x213)
        + (CARRY2(u_var27, *pu_var5) || CARRY2(u_var16, b_var70));
    pc_var22 = (u_var19 + pi_var53);
    *pc_var22 = *pc_var22 + u_var18;
    pc_var22 = (u_var19 + pi_var53);
    b_var24 = u_var28;
    *pc_var22 = *pc_var22 + b_var24;
    pb_var1 = (pi_var62 + u_var19 + 0xa213);
    b_var11 = *pb_var1;
    b_var13 = (u_var17 >> 8);
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (pi_var53 + u_var42 + 0xa113);
    u_var16 = CARRY1(b_var11, b_var13);
    pu_var59 = &stack0xa11e + *pu_var5;
    u_var18 = u_var17
        + (u_var42 + pi_var62)
        + (CARRY2(&stack0xa11e, *pu_var5) || CARRY2(pu_var59, u_var16));
    pc_var22 = (u_var19 + pi_var53);
    *pc_var22 = *pc_var22 + u_var18;
    pc_var22 = (u_var19 + pi_var62);
    c_var12 = u_var31;
    *pc_var22 = *pc_var22 + c_var12;
    pb_var1 = (u_var42 + 0xa613);
    b_var11 = *pb_var1;
    b_var13 = (u_var18 >> 8);
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (pi_var53 + -0x5aed);
    u_var17 = CARRY1(b_var11, b_var13);
    pu_var56 = pu_var59 + *pu_var5 + u_var16;
    i_var25 = u_var18
        + (u_var42 + pi_var53)
        + (CARRY2((pu_var59 + u_var16), *pu_var5) || CARRY2(pu_var56, u_var17));
    pc_var22 = (u_var19 + pi_var53);
    *pc_var22 = *pc_var22 + b_var24;
    pb_var1 = (pi_var62 + u_var19 + 0xab00);
    b_var11 = *pb_var1;
    b_var13 = i_var25;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (pi_var53 + u_var19 + 0xa813);
    u_var16 = CARRY1(b_var11, b_var13);
    u_var18 = u_var19 + *pu_var5;
    u_var27 = u_var18
        + u_var16
        + (pi_var62 + u_var42 + 0x13)
        + (CARRY2(u_var19, *pu_var5) || CARRY2(u_var18, u_var16));
    pi_var4 = pi_var53;
    *pi_var4 = *pi_var4 + c_var12;
    pc_var22 = (u_var27 + pi_var53);
    *pc_var22 = *pc_var22 + c_var12;
    pb_var1 = (u_var42 + 0xae13);
    b_var11 = *pb_var1;
    b_var13 = (u_var28 >> 8);
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (pi_var53 + -0x52ed);
    u_var16 = CARRY1(b_var11, b_var13);
    u_var18 = u_var27 + *pu_var5;
    u_var49 = u_var18 + u_var16;
    u_var18 =
        i_var25 + (u_var42 + pi_var53) + (CARRY2(u_var27, *pu_var5) || CARRY2(u_var18, u_var16));
    pi_var4 = pi_var53;
    b_var13 = u_var18;
    *pi_var4 = *pi_var4 + b_var13;
    pi_var4 = pi_var53 + -0x2680;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var13;
    pu_var5 = (pi_var53 + u_var49 + 0xb013);
    u_var16 = pi_var53 + *pu_var5;
    pc_var54 = (u_var16
        + CARRY1(b_var11, b_var13)
        + (pi_var62 + u_var42 + 0x13)
        + (CARRY2(pi_var53, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var13))));
    pc_var22 = pc_var54;
    *pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var62 + -0x2480;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + b_var13;
    pu_var5 = (u_var49 + 0xb413);
    pc_var54 = pc_var54
        + *pu_var5
        + (CARRY2(pc_var54, *pu_var5) || CARRY2((pc_var54 + *pu_var5), CARRY1(b_var11, b_var13)))
        + (pi_var62 + 0x13)
        + CARRY1(b_var11, b_var13);
    pc_var22 = pc_var54 + u_var49;
    *pc_var22 = *pc_var22 + b_var13;
    pb_var1 = (u_var49 + 0xbb00);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (pc_var54 + u_var49 + 0xb813);
    u_var16 = pi_var62 + *pu_var5;
    i_var25 = u_var16 + CARRY1(b_var11, b_var13);
    u_var27 = i_var25
        + (u_var42 + i_var25 + 0x13)
        + (CARRY2(pi_var62, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var13)));
    pc_var22 = pc_var54 + u_var49;
    *pc_var22 = *pc_var22 + b_var24;
    pb_var1 = (u_var42 + 0xbf00);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var5 = (u_var49 + 0xbc13);
    u_var16 = u_var27 + *pu_var5;
    i_var25 = u_var16 + CARRY1(b_var11, b_var13);
    u_var63 = i_var25
        + (i_var25 + 0x13)
        + (CARRY2(u_var27, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var13)));
    pc_var22 = pc_var54;
    *pc_var22 = *pc_var22 + b_var13;
    pb_var1 = (pc_var54 + u_var42 + 0xc300);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var24;
    u_var27 = u_var18 + u_var31 + CARRY1(b_var11, b_var24);
    u_var16 = (CARRY2(u_var18, u_var31) || CARRY2(u_var18 + u_var31, CARRY1(b_var11, b_var24)));
    u_var19 = u_var27 * 2 + u_var16;
    u_var16 = (CARRY2(u_var27, u_var27) || CARRY2(u_var27 * 2, u_var16));
    u_var18 = u_var19 + u_var28;
    u_var27 = u_var18
        + u_var16
        + (pc_var54 + u_var42)
        + (CARRY2(u_var19, u_var28) || CARRY2(u_var18, u_var16));
    pc_var22 = pc_var54;
    c_var12 = u_var27;
    *pc_var22 = *pc_var22 + c_var12;
    pb_var1 = (u_var42 + u_var63 + -0x7600);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var24;
    pu_var5 = (u_var49 + 0x9213);
    u_var16 = u_var28 + *pu_var5;
    u_var19 = u_var16 + CARRY1(b_var11, b_var24);
    pu_var3 = (u_var49 + 0x9a13);
    u_var16 = (CARRY2(u_var28, *pu_var5) || CARRY2(u_var16, CARRY1(b_var11, b_var24)));
    u_var18 = u_var31 + *pu_var3;
    u_var28 = u_var18 + u_var16;
    u_var35 = u_var34 & 0xffff0000 | u_var28;
    pu_var5 = (u_var49 + 0xa213);
    u_var16 = (CARRY2(u_var31, *pu_var3) || CARRY2(u_var18, u_var16));
    u_var18 = u_var42 + *pu_var5;
    pi_var26 = (u_var18 + u_var16);
    pu_var3 = (u_var49 + 0xaa13);
    u_var18 = (CARRY2(u_var42, *pu_var5) || CARRY2(u_var18, u_var16));
    pu_var59 = pu_var56 + *pu_var3 + u_var17;
    pu_var5 = (u_var49 + 0xb213);
    u_var16 = (CARRY2((pu_var56 + u_var17), *pu_var3) || CARRY2(pu_var59, u_var18));
    u_var17 = u_var49 + *pu_var5;
    i_var25 = u_var17 + u_var16;
    pu_var3 = (i_var25 + -0x45ed);
    u_var16 = (CARRY2(u_var49, *pu_var5) || CARRY2(u_var17, u_var16));
    pu_var55 = (pc_var54 + *pu_var3 + u_var16);
    pu_var5 = (i_var25 + -0x3ded);
    u_var16 = (CARRY2(pc_var54, *pu_var3) || CARRY2((pc_var54 + *pu_var3), u_var16));
    u_var17 = u_var63 + *pu_var5;
    pu_var64 = (u_var17 + u_var16);
    u_var17 = i_var25
        + (pi_var26 + pu_var55 + -0x56f1)
        + (CARRY2(u_var63, *pu_var5) || CARRY2(u_var17, u_var16));
    u_var20 = u_var50 & 0xffff0000 | u_var17;
    u_var67 = LocalDescriptorTableRegister();
    (pi_var26 + pu_var55) = u_var67;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pu_var5 = (pi_var26 + pu_var55);
    u_var16 = *pu_var5;
    *pu_var5 = *pu_var5 + u_var27;
    b_var11 = c_var12 + CARRY2(u_var16, u_var27);
    u_var21 = u_var21 & 0xffff0000 | u_var27 & 0xffffff00 | b_var11;
    pu_var44 = pu_var59 + (u_var18 - 2);
    pu_var45 = (pu_var59 + (u_var18 - 2));
    (pu_var59 + (u_var18 - 2)) = u_var66;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + b_var11;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + b_var11;
    pu_var5 = pu_var64;
    *pu_var5 = *pu_var5 + b_var11;
    pc_var22 = (u_var17 + pu_var55);
    *pc_var22 = *pc_var22 + b_var11;
    pc_var22 = (u_var17 + pu_var64);
    c_var12 = u_var19;
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (u_var17 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pu_var5 = pu_var64;
    *pu_var5 = *pu_var5 + b_var11;
    pu_var60 = pu_var55;
    *pu_var60 = *pu_var60 + b_var11;
    pu_var60 = pu_var55;
    *pu_var60 = *pu_var60 + b_var11;
    pc_var22 = (u_var17 + pu_var64);
    *pc_var22 = *pc_var22 + b_var11;
    pu_var60 = pu_var55;
    *pu_var60 = *pu_var60 + c_var12;
    pi_var4 = pi_var26;
    *pi_var4 = *pi_var4 + b_var11;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55);
    *pc_var22 = *pc_var22 + b_var11;
    pu_var5 = pu_var64;
    b_var11 = *pu_var5;
    b_var13 = ((u_var27 & 0xffffff00) >> 8);
    *pu_var5 = *pu_var5 + b_var13;
    b_var24 = (u_var28 >> 8);
    if (CARRY1(b_var11, b_var13)) {
        u_var21 = u_var21 & 0xffff2173;
        pc_var22 = (pi_var26 + pu_var55);
        b_var14 = u_var21;
        *pc_var22 = *pc_var22 + b_var14;
        pc_var22 = (pi_var26 + pu_var55);
        *pc_var22 = *pc_var22 + b_var14;
        pc_var22 = (pi_var26 + pu_var55);
        *pc_var22 = *pc_var22 + b_var14;
        pc_var22 = (pi_var26 + pu_var55);
        *pc_var22 = *pc_var22 + b_var14;
        pb_var1 = (pi_var26 + pu_var55);
        b_var11 = *pb_var1;
        b_var13 = *pb_var1;
        *pb_var1 = *pb_var1 + b_var14;
        if (!SCARRY1(b_var13, b_var14)) {
            out(*pu_var55, u_var28);
            pu_var5 = (pu_var55 + 0x71);
            *pu_var5 = *pu_var5 + CARRY1(b_var11, b_var14) * ((u_var19 & 3) - (*pu_var5 & 3));
            pb_var1 = (pi_var26 + (pu_var55 + 1) + 0x72);
            b_var11 = *pb_var1;
            *pb_var1 = *pb_var1 + b_var24;
            out(pu_var55[1], u_var28);
            pu_var5 = (pi_var26 + pu_var55 + 0x6d);
            *pu_var5 = *pu_var5 + CARRY1(b_var11, b_var24) * ((u_var19 & 3) - (*pu_var5 & 3));
            pu_var60 = pu_var55 + 0x36;
            *pu_var60 = *pu_var60 + b_var24;
            b_var70 = *pu_var60 == '\0';
            pu_var55 = ((pu_var64 + u_var17 + 0x4c) * 0x6f);
            if (!b_var70) {
                // goto code_r0x10505be7;
            }
            pc_var22 = (pu_var55 + u_var17 + 1);
            *pc_var22 = *pc_var22 + b_var14;
            pu_var64 = (pu_var64 + 1);
            pu_var44 = pu_var59 + (u_var18 - 4);
            (pu_var59 + (u_var18 - 4)) = u_var66;
        }
        // code_r0x10505c44:
        u_var19 = u_var19 + 1;
        pi_var4 = pi_var26 + 3;
        i_var25 = u_var21;
        *pi_var4 = *pi_var4 + i_var25;
        pc_var22 = (pi_var26 + pu_var55 + 1);
        *pc_var22 = *pc_var22 + u_var19;
        pc_var22 = (pi_var26 + pu_var55);
        *pc_var22 = *pc_var22 + (i_var25 + 0x7400);
        u_var35 = u_var34 & 0xffff0000 | (u_var28 + 1);
        pi_var4 = pi_var26 + 3;
        *pi_var4 = *pi_var4 + i_var25 + 0x7400;
        u_var21 = u_var21 & 0xffff0000 | (i_var25 - 0xb00);
        pc_var22 = (pi_var26 + pu_var64 + 1);
        *pc_var22 = *pc_var22 + u_var19;
        pu_var45 = pu_var44;
        // code_r0x10505c5a:
        pc_var22 = (pi_var26 + pu_var55);
        *pc_var22 = *pc_var22 + u_var21;
        pi_var26 = (pi_var26 + 1);
    } else {
        out(*pu_var55, u_var28);
        pu_var60 = pu_var55 + 0x35;
        *pu_var60 = *pu_var60 + b_var24;
        b_var70 = *pu_var60 == '\0';
        // code_r0x10505be7:
        pu_var55 = ((pu_var64 + u_var17 + 0x48) * 0x69);
        if (b_var70) {
            pi_var4 = (pi_var26 + pu_var55);
            *pi_var4 = *pi_var4 + 0x149;
            // goto code_r0x10505c5a;
        }
        pu_var55 = ((pu_var64 + u_var17 + 0x4c) * 0x6f);
        pu_var45 = (pu_var59 + (u_var18 - 2));
        if (!b_var70) {
            pu_var56 = ((pu_var64 + u_var17 + 0x48) * 0x69);
            (pu_var59 + (u_var18 - 4)) = 0x6f66;
            out(*pu_var56, u_var28);
            (pu_var59 + (u_var18 - 6)) = 0x6f66;
            out(pu_var56[1], u_var28);
            (pu_var59 + (u_var18 - 8)) = 0x6f66;
            pu_var55 = (pu_var56 + 3);
            out(pu_var56[2], u_var28);
            i_var25 = u_var21;
            pc_var22 = (pu_var55 + u_var17 + 1);
            *pc_var22 = *pc_var22 + c_var12;
            pi_var4 = (pi_var26 + pu_var55);
            *pi_var4 = *pi_var4 + i_var25 + 0xe00;
            pu_var44 = pu_var59 + (u_var18 - 7);
            pi_var4 = (u_var17 + 6);
            *pi_var4 = *pi_var4 + i_var25 + 0xe00;
            pc_var22 = (pu_var64 + u_var17 + 1);
            *pc_var22 = *pc_var22 + c_var12;
            pi_var4 = (pi_var26 + pu_var55);
            *pi_var4 = *pi_var4 + i_var25 + 0x2900;
            pc_var22 = (pi_var26 + pu_var55);
            c_var12 = (i_var25 + 0x2900);
            *pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var26 + pu_var55);
            *pc_var22 = *pc_var22 + c_var12;
            pb_var1 = (pu_var64 + 1);
            b_var13 = (i_var25 + 0x6600);
            *pb_var1 = *pb_var1 + b_var13;
            pc_var22 = (pi_var26 + pu_var55);
            *pc_var22 = *pc_var22 + b_var13;
            b_var11 = 9 < (b_var13 & 0xf) | in_af;
            u_var16 =
                CONCAT11(((i_var25 + 0x6600) >> 8) - b_var11, b_var13 + b_var11 * -6) & 0xff0f;
            pi_var4 = pi_var26 + 3;
            *pi_var4 = *pi_var4 + u_var16;
            pc_var22 = (u_var17 + 1);
            *pc_var22 = *pc_var22 + u_var16;
            pc_var22 = (pi_var26 + pu_var55);
            *pc_var22 = *pc_var22 + u_var16;
            pi_var4 = (pi_var26 + pu_var55 + 6);
            *pi_var4 = *pi_var4 + u_var19;
            u_var21 = u_var21 & 0xffff0000 | (u_var16 + 0xb101);
            pc_var22 = (pi_var26 + 1);
            c_var12 = (u_var16 + 0xb101);
            *pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var26 + pu_var55);
            *pc_var22 = *pc_var22 + c_var12;
            // goto code_r0x10505c44;
        }
    }
    pi_var4 = (pi_var26 + pu_var55 + 6);
    *pi_var4 = *pi_var4 + u_var19;
    u_var67 = *pu_var45;
    pu_var60 = pu_var55;
    *pu_var60 = *pu_var60 + u_var35;
    pc_var22 = (pi_var26 + 1);
    c_var12 = (u_var21 >> 8);
    *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = (pi_var26 + pu_var55);
    *pi_var4 = *pi_var4 + 0x6f;
    pi_var4 = (u_var17 + 6);
    u_var18 = u_var21;
    *pi_var4 = *pi_var4 + u_var18;
    pc_var22 = (u_var17 + pu_var64);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var55 + 1);
    c_var23 = (u_var19 >> 8);
    *pc_var22 = *pc_var22 + c_var23;
    pu_var57 = (pu_var55 + 1);
    *pu_var45 = u_var67;
    pu_var65 = pu_var64 + 1;
    *pu_var64 = u_var18;
    pi_var4 = pi_var26;
    *pi_var4 = *pi_var4 + u_var21;
    pc_var22 = (u_var17 + pu_var57);
    *pc_var22 = *pc_var22 + 0x7;
    pc_var22 = (pi_var26 + pu_var65 + 1);
    c_var12 = *pc_var22;
    *pc_var22 = *pc_var22 + c_var23;
    pu_var5 = (pi_var26 + pu_var57);
    u_var16 = *pu_var5;
    *pu_var5 = u_var18;
    if (SCARRY1(c_var12, c_var23)) {
        pu_var57 = pu_var55 + 1;
    }
    pu_var45[-1] = u_var67;
    u_var18 = 0x700;
    pb_var1 = (pi_var26 + pu_var65);
    b_var11 = *pb_var1;
    b_var13 = u_var16;
    *pb_var1 = *pb_var1 + b_var13;
    pu_var45[-2] = 1;
    *(pi_var26 + pu_var57) = b_var13;
    if (!CARRY1(b_var11, b_var13)) {
        pu_var57 = (pu_var57 + 1);
    }
    pu_var45[-3] = u_var67;
    u_var35 = u_var35 & 0xffff0000;
    pu_var45[-4] = 0;
    pc_var22 = (pu_var65 + u_var17 + 1);
    *pc_var22 = *pc_var22 + 0x7;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    u_var67 = pu_var45[-4];
    pu_var2 = (u_var17 + pu_var65);
    *pu_var2 = *pu_var2;
    b_var11 = _in(0);
    *pu_var65 = b_var11;
    pi_var4 = pu_var57 + 0x3980;
    *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = (u_var17 + 6);
    *pi_var4 = *pi_var4 + u_var16;
    pb_var1 = (pu_var64 + 0x6d03);
    *pb_var1 = *pb_var1 + b_var13;
    pi_var4 = (pi_var26 + pu_var57);
    *pi_var4 = *pi_var4 + u_var16;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var26 + pu_var57);
    *pc_var22 = *pc_var22 + b_var13;
    pi_var4 = pi_var26;
    *pi_var4 = *pi_var4 + b_var13;
    pu_var60 = pu_var57 + 0x3700;
    *pu_var60 = *pu_var60 + pi_var26;
    pi_var4 = (u_var17 + pu_var57);
    *pi_var4 = *pi_var4 + 0x700;
    pu_var2 = (pu_var57 + 1);
    *pu_var2 = *pu_var2;
    pu_var58 = (pu_var57 + 1);
    0x2afe = u_var67;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + b_var13;
    pc_var22 = (u_var21 & 0xffff0000 | u_var16 & 0xffff0071);
    pu_var64 = pu_var64 + 1;
    0x2afc = (u_var16 & 0xffff0071);
    u_var34 = u_var35 | 1;
    u_var66 = 0x8217;
    pi_var4 = pi_var26;
    b_var11 = *pi_var4;
    *pi_var4 = *pi_var4 + -0x7f;
    u_var66 = 0x8219;
    pi_var4 = pi_var26;
    *pi_var4 = *pi_var4 + 0x20 + (0x7e < b_var11);
    pb_var1 = (pi_var26 + pu_var58 + 0x17);
    b_var24 = (pi_var26 >> 8);
    b_var70 = CARRY1(*pb_var1, b_var24);
    b_var11 = *pb_var1;
    *pb_var1 = *pb_var1 + b_var24;
    b_var13 = *pb_var1;
    b_var69 = *pb_var1 == 0;
    b_var68 = (POPCOUNT(*pb_var1) & 1) != 0;
    ppu_var46 = 0x821b;
    if (SCARRY1(b_var11, b_var24)) {
        if (!b_var69) {
            // goto code_r0x10505d87;
        }
        // code_r0x10505de8:
        pc_var6 = (pi_var26 + pu_var58 + 0x66);
        *pc_var6 = *pc_var6 + (u_var18 >> 8);
        pu_var55 = (pu_var58 + 2);
        out(pu_var58, u_var34);
        pu_var58 = (pu_var58 + 3);
        out(*pu_var55, u_var34);
        pc_var6 = (pi_var26 + pu_var58);
        c_var12 = pc_var22;
        *pc_var6 = *pc_var6 + c_var12;
        pc_var6 = (pi_var26 + pu_var58);
        *pc_var6 = *pc_var6 + c_var12;
        pc_var6 = (pi_var26 + pu_var58);
        *pc_var6 = *pc_var6 + c_var12;
        pc_var6 = (pi_var26 + pu_var58);
        *pc_var6 = *pc_var6 + c_var12;
        pc_var6 = (pi_var26 + pu_var58);
        *pc_var6 = *pc_var6 + c_var12;
        // code_r0x10505df9:
        pb_var1 = (pi_var26 + pu_var58 + 0x66);
        b_var11 = (u_var18 >> 8);
        b_var70 = CARRY1(*pb_var1, b_var11);
        *pb_var1 = *pb_var1 + b_var11;
        pu_var7 = pu_var58;
        pu_var58 = (pu_var58 + 2);
        out(pu_var7, u_var34);
        // code_r0x10505dfd:
        pu_var7 = pu_var58;
        pu_var58 = (pu_var58 + 1);
        out(*pu_var7, u_var34);
        // code_r0x10505dfe:
        pu_var64 = (pu_var64 + -1);
        // code_r0x10505e01:
        ppu_var48 = (ppu_var46 + -2);
        (ppu_var46 + -2) = pc_var22;
        i_var25 = u_var34;
        u_var16 = i_var25 + 1;
        u_var34 = u_var34 & 0xffff0000 | u_var16;
        if (u_var16 == 0) {
            b_var68 = SCARRY2(i_var25, 1);
            if (u_var16 != 0) {
                // code_r0x10505e07:
                pu_var7 = pu_var58;
                pu_var58 = (pu_var58 + 1);
                out(*pu_var7, u_var34);
                // code_r0x10505e08:
                pc_var6 = (pi_var26 + pu_var58 + 0x66);
                *pc_var6 = *pc_var6 + (u_var18 >> 8);
                pu_var57 = (pu_var58 + 2);
                out(pu_var58, u_var34);
                // goto code_r0x10505e0c;
            }
            // code_r0x10505e76:
            i_var25 = u_var20;
            if (!b_var68) {
                pu_var7 = pu_var58;
                pu_var58 = (pu_var58 + 2);
                out(pu_var7, u_var34);
                // goto code_r0x10505e79;
            }
            pc_var6 = (pi_var26 + pu_var58);
            c_var12 = pc_var22;
            *pc_var6 = *pc_var6 + c_var12;
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + c_var12;
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + c_var12;
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + c_var12;
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + c_var12;
            u_var16 = pc_var22 & 0x646c;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + u_var16;
            // goto code_r0x10505ef5;
        }
        // code_r0x10505e79:
        pu_var5 = (pu_var58 + 0x6f);
        *pu_var5 = *pu_var5 + b_var70 * ((u_var18 & 3) - (*pu_var5 & 3));
        pb_var1 = (pi_var26 + pu_var58 + 0x72);
        b_var11 = *pb_var1;
        b_var13 = (u_var34 >> 8);
        *pb_var1 = *pb_var1 + b_var13;
        out(pu_var58, u_var34);
        pu_var5 = (pi_var26 + pu_var58 + 0x6b);
        *pu_var5 = *pu_var5 + CARRY1(b_var11, b_var13) * ((u_var18 & 3) - (*pu_var5 & 3));
        pu_var60 = (pu_var58 + 0x6a);
        *pu_var60 = *pu_var60 + b_var13;
        b_var68 = *pu_var60 == '\0';
        // code_r0x10505e86:
        u_var16 = pc_var22;
        i_var25 = u_var20;
        pu_var58 = ((pu_var64 + i_var25 + 0x4c) * 0x6f);
        if (b_var68) {
            // goto code_r0x10505ef5;
        }
        // code_r0x10505e8d:
        u_var16 = pc_var22;
        i_var25 = u_var20;
        lVar9 = (pu_var64 + i_var25 + 0x48) * 0x69;
        pu_var58 = lVar9;
        u_var18 = u_var18 - 1;
        u_var32 = u_var34;
        c_var23 = (pc_var22 >> 8);
        c_var12 = pc_var22;
        if (pu_var58 == lVar9) {
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + c_var12;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + c_var12;
            pu_var5 = pu_var64;
            *pu_var5 = *pu_var5 + c_var23;
            pu_var5 = pu_var64;
            pu_var64 = (pu_var64 + 1);
            b_var11 = _in(u_var32);
            *pu_var5 = b_var11;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + c_var12;
            // goto code_r0x10505ee9;
        }
        pu_var5 = pu_var64;
        pu_var64 = (pu_var64 + 1);
        b_var11 = _in(u_var32);
        *pu_var5 = b_var11;
        pc_var6 = (u_var34 + 0x75);
        *pc_var6 = *pc_var6 + c_var23;
        if (*pc_var6 != '\0') {
            pu_var55 = (pu_var58 + 2);
            out(pu_var58, u_var32);
            pu_var58 = (pu_var58 + 3);
            out(*pu_var55, u_var32);
            // goto code_r0x10505e9e;
        }
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        u_var16 = u_var16 & 0x73;
    } else {
        ppu_var46 = 0x821b;
        if (b_var70) {
            // code_r0x10505d87:
            if (b_var69) {
                pc_var6 = (pi_var26 + pu_var58);
                b_var11 = pc_var22;
                *pc_var6 = *pc_var6 + b_var11;
                pc_var6 = (pi_var26 + pu_var58);
                *pc_var6 = *pc_var6 + b_var11;
                pc_var6 = (pi_var26 + pu_var58);
                *pc_var6 = *pc_var6 + b_var11;
                pb_var1 = (pi_var26 + pu_var58);
                b_var70 = CARRY1(*pb_var1, b_var11);
                *pb_var1 = *pb_var1 + b_var11;
                pu_var59 = ppu_var46;
                // code_r0x10505dfa:
                ppu_var46 = (pu_var59 + -2);
                (pu_var59 + -2) = 0x6f66;
                // goto code_r0x10505dfd;
            }
            // code_r0x10505d89:
            pu_var5 = (pi_var26 + pu_var58);
            i_var25 = (pc_var22 & 3) - (*pu_var5 & 3);
            b_var69 = 0 < i_var25;
            *pu_var5 = *pu_var5 + b_var70 * i_var25;
            // code_r0x10505d8b:
            if (!b_var70) {
                // goto code_r0x10505e01;
            }
            // code_r0x10505d8d:
            pu_var64 = *ppu_var46;
            pu_var58 = ppu_var46[1];
            u_var20 = u_var20 & 0xffff0000 | ZEXT24(ppu_var46[2]);
            pi_var26 = ppu_var46[4];
            u_var34 = u_var34 & 0xffff0000 | ZEXT24(ppu_var46[5]);
            u_var18 = ppu_var46[6];
            ppu_var47 = ppu_var46 + 7;
            pc_var22 = (pc_var22 & 0xffff0000 | ZEXT24(*ppu_var47));
            ppu_var48 = ppu_var46 + 8;
            ppu_var46 = ppu_var46 + 8;
            if (b_var69) {
                // goto code_r0x10505df9;
            }
            pu_var5 = (pi_var26 + pu_var58);
            i_var25 = (*ppu_var47 & 3) - (*pu_var5 & 3);
            b_var69 = 0 < i_var25;
            *pu_var5 = *pu_var5 + b_var70 * i_var25;
            // code_r0x10505d95:
            if (b_var69) {
                pu_var7 = pu_var58;
                pu_var58 = (pu_var58 + 2);
                out(pu_var7, u_var34);
                // goto code_r0x10505e07;
            }
            // code_r0x10505d97:
            out(*pu_var58, u_var34);
            pb_var1 = (pi_var26 + (pu_var58 + 1));
            b_var70 = CARRY1(*pb_var1, pc_var22);
            *pb_var1 = *pb_var1 + pc_var22;
            b_var11 = *pb_var1;
            ppu_var46 = ppu_var48;
            if (!b_var70) {
                pu_var59 = (pu_var58 + 5);
                out((pu_var58 + 1), u_var34);
                // goto code_r0x10505e0f;
            }
            // code_r0x10505d9c:
            b_var69 = b_var11 == 0;
            pu_var64 = *ppu_var46;
            pu_var58 = ppu_var46[1];
            u_var20 = u_var20 & 0xffff0000 | ZEXT24(ppu_var46[2]);
            pi_var26 = ppu_var46[4];
            u_var34 = u_var34 & 0xffff0000 | ZEXT24(ppu_var46[5]);
            u_var18 = ppu_var46[6];
            pc_var22 = (pc_var22 & 0xffff0000 | ZEXT24(ppu_var46[7]));
            ppu_var48 = ppu_var46 + 8;
            // code_r0x10505d9d:
            if (b_var69) {
                // goto code_r0x10505e08;
            }
            pu_var5 = (pi_var26 + pu_var58);
            i_var25 = (pc_var22 & 3) - (*pu_var5 & 3);
            *pu_var5 = *pu_var5 + b_var70 * i_var25;
            if (!b_var70) {
                // goto code_r0x10505e17;
            }
            pu_var64 = *ppu_var48;
            pu_var59 = ppu_var48[1];
            u_var27 = ppu_var48[2];
            u_var20 = u_var20 & 0xffff0000 | u_var27;
            pi_var26 = ppu_var48[4];
            u_var17 = ppu_var48[5];
            u_var34 = u_var34 & 0xffff0000 | u_var17;
            u_var18 = ppu_var48[6];
            u_var16 = ppu_var48[7];
            pc_var22 = (pc_var22 & 0xffff0000 | u_var16);
            ppu_var46 = ppu_var48 + 8;
            ppu_var48 = ppu_var48 + 8;
            if (0 < i_var25) {
                // goto code_r0x10505e0f;
            }
            pu_var5 = (pi_var26 + pu_var59);
            i_var25 = (u_var16 & 3) - (*pu_var5 & 3);
            b_var68 = 0 < i_var25;
            *pu_var5 = *pu_var5 + b_var70 * i_var25;
            if (b_var68) {
                if (!b_var68) {
                    // goto code_r0x10505e1e;
                }
                // goto code_r0x10505e86;
            }
            out(*pu_var59, u_var17);
            pc_var6 = (pi_var26 + (pu_var59 + 1));
            *pc_var6 = *pc_var6 + u_var16;
            pc_var6 = pu_var59 + 0x69;
            b_var11 = (u_var17 >> 8);
            *pc_var6 = *pc_var6 + b_var11;
            if (*pc_var6 == '\0') {
                pc_var6 = ((pu_var64 + u_var27 + 0x4c) * 0x6f + 0x68);
                *pc_var6 = *pc_var6 + b_var11;
                // goto code_r0x10505e25;
            }
            lVar9 = (pu_var64 + u_var27 + 0x48) * 0x69;
            pu_var55 = lVar9;
            b_var70 = pu_var55 != lVar9;
            if (b_var70) {
                // goto code_r0x10505e33;
            }
            out(*pu_var55, u_var17);
            pu_var5 = (pu_var55 + 0x71);
            *pu_var5 = *pu_var5 + b_var70 * ((u_var18 & 3) - (*pu_var5 & 3));
            pb_var1 = (pi_var26 + (pu_var55 + 1) + 0x72);
            b_var13 = *pb_var1;
            *pb_var1 = *pb_var1 + b_var11;
            out(pu_var55[1], u_var17);
            pu_var5 = (pi_var26 + pu_var55 + 0x6d);
            *pu_var5 = *pu_var5 + CARRY1(b_var13, b_var11) * ((u_var18 & 3) - (*pu_var5 & 3));
            pu_var60 = pu_var55 + 0x36;
            *pu_var60 = *pu_var60 + b_var11;
            pu_var58 = ((pu_var64 + u_var27 + 0x4c) * 0x6f);
            if (*pu_var60 != '\0') {
                lVar9 = (pu_var64 + u_var27 + 0x48) * 0x69;
                pu_var55 = lVar9;
                b_var70 = pu_var55 != lVar9;
                if (!b_var70) {
                    out(*pu_var55, u_var17);
                    pu_var5 = (pu_var55 + 0x71);
                    *pu_var5 = *pu_var5 + b_var70 * ((u_var18 & 3) - (*pu_var5 & 3));
                    pb_var1 = (pi_var26 + (pu_var55 + 1) + 0x72);
                    b_var13 = *pb_var1;
                    *pb_var1 = *pb_var1 + b_var11;
                    pu_var58 = (pu_var55 + 2);
                    out(pu_var55[1], u_var17);
                    pu_var5 = (pi_var26 + pu_var58 + 0x69);
                    *pu_var5 =
                        *pu_var5 + CARRY1(b_var13, b_var11) * ((u_var18 & 3) - (*pu_var5 & 3));
                    // goto code_r0x10505de8;
                }
                // goto code_r0x10505e4f;
            }
            pc_var6 = (pu_var58 + u_var27 + 0x75);
            *pc_var6 = *pc_var6 + (u_var16 >> 8);
            if (*pc_var6 != '\0') {
                pu_var7 = pu_var58;
                pu_var58 = (pu_var58 + 2);
                out(pu_var7, u_var17);
                // goto code_r0x10505e44;
            }
            pb_var1 = (pu_var58 + u_var27 + 100);
            *pb_var1 = *pb_var1 & b_var11;
            b_var11 = *pb_var1;
            // code_r0x10505eba:
            c_var12 = u_var18;
            b_var70 = false;
            out(pu_var58, u_var34);
            // code_r0x10505ebb:
            i_var25 = u_var20;
            u_var16 = pc_var22;
            pu_var58 = (pu_var58 + 2);
            i_var15 = u_var34;
            pu_var65 = pu_var64;
            if (!b_var70 && b_var11 != 0) {
                // code_r0x10505f2b:
                pc_var22 = (pi_var26 + pu_var58);
                c_var12 = u_var16;
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var64);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var64);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                // code_r0x10505f3b:
                pc_var22 = (pi_var26 + pu_var58);
                c_var12 = u_var16;
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var64);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pi_var26 = (pi_var26 & 0xff | ((pi_var26 >> 8) * 0x2) << 8);
                pi_var4 = (pi_var26 + pu_var58);
                *pi_var4 = *pi_var4 + 1;
                // goto code_r0x10505f47;
            }
            // code_r0x10505ebd:
            b_var11 = u_var16 * 0x2;
            u_var16 = u_var16 & 0xff00 | b_var11;
            pc_var22 = (pi_var26 + pu_var58 + 0x80);
            *pc_var22 = *pc_var22 + 0x2;
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + b_var11;
            pu_var64 = (pu_var65 * 2);
            pi_var4 = (pi_var26 + pu_var58);
            *pi_var4 = *pi_var4 + 1;
            pi_var4 = pi_var26 + 0x28;
            *pi_var4 = *pi_var4 + c_var12;
            u_var17 = i_var15 + 1;
            u_var34 = u_var17;
            if (u_var17 != 0) {
                // code_r0x10505f47:
                pu_var2 = (pi_var26 + pu_var58);
                *pu_var2 = *pu_var2;
                pc_var22 = (pi_var26 + pu_var58);
                c_var12 = u_var16;
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pc_var22 = (pi_var26 + pu_var58);
                *pc_var22 = *pc_var22 + c_var12;
                pi_var4 = (pi_var26 + pu_var58);
                *pi_var4 = *pi_var4 + u_var16;
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            if (u_var17 == 0) {
                return;
            }
            pu_var7 = pu_var58;
            pu_var58 = (pu_var58 + 1);
            out(*pu_var7, 0);
            pc_var22 = (pi_var26 + pu_var58);
            *pc_var22 = *pc_var22 + b_var11;
        } else {
            ppu_var46 = 0x821b;
            if (!b_var70) {
                // goto code_r0x10505d89;
            }
            ppu_var46 = 0x821b;
            if (b_var69) {
                // goto code_r0x10505d8b;
            }
            if (-1 < b_var13) {
                // goto code_r0x10505d8d;
            }
            ppu_var46 = 0x821b;
            if (b_var68) {
                *pi_var26 = 1;
                u_var34 = u_var35 | *pi_var26;
                ppu_var48 = ppu_var46;
                if (!b_var70) {
                    // goto code_r0x10505d95;
                }
                if (b_var69) {
                    // goto code_r0x10505d97;
                }
                if (-1 < b_var13) {
                    pb_var1 = (pu_var64 + u_var17 + 0x74);
                    b_var70 = false;
                    *pb_var1 = *pb_var1;
                    b_var11 = *pb_var1;
                    // goto code_r0x10505d9c;
                }
                if (b_var68) {
                    pu_var59 = 0x821b;
                    if (b_var70) {
                        pu_var64 = *0x821b;
                        pu_var58 = 0x821d;
                        u_var20 = u_var50 & 0xffff0000 | *0x821f;
                        pi_var26 = 0x8223;
                        u_var34 = u_var35 | 0x8225;
                        u_var18 = 0x8227;
                        pc_var22 = (u_var21 & 0xffff0000 | *0x8229);
                        ppu_var46 = 0x822b;
                        // goto code_r0x10505d87;
                    }
                    // goto code_r0x10505dfa;
                }
                if (!b_var69) {
                    // goto code_r0x10505d9d;
                }
                // goto code_r0x10505dfe;
            }
            ppu_var48 = ((u_var17 + pu_var64) * 0x7562);
            if (!b_var69) {
                pu_var7 = pu_var58;
                pu_var58 = (pu_var57 + 3);
                out(pu_var7, 1);
                // goto code_r0x10505d97;
            }
            pu_var57 = (pu_var57 + 5);
            out(*pu_var58, 1);
            // code_r0x10505e0c:
            pu_var59 = (pu_var57 + 1);
            out(*pu_var57, u_var34);
            // code_r0x10505e0f:
            (ppu_var48 + -2) = 0x6f66;
            pu_var58 = (pu_var59 + 1);
            out(*pu_var59, u_var34);
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + pc_var22;
            // code_r0x10505e17:
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + pc_var22;
            pc_var6 = (pi_var26 + pu_var58);
            *pc_var6 = *pc_var6 + pc_var22;
            pu_var7 = pu_var58 + 0x1a;
            *pu_var7 = *pu_var7 + (u_var34 >> 8);
            b_var68 = *pu_var7 == '\0';
            // code_r0x10505e1e:
            if (b_var68) {
                // goto code_r0x10505e8d;
            }
            // code_r0x10505e25:
            lVar9 = (pu_var64 + u_var20 + 0x48) * 0x69;
            pu_var58 = lVar9;
            if (pu_var58 != lVar9) {
                // code_r0x10505e9e:
                u_var16 = pc_var22;
                c_var12 = u_var18;
                pu_var5 = pu_var64;
                *pu_var5 = *pu_var5 & (pc_var22 >> 8);
                pb_var1 = (pi_var26 + pu_var58);
                *pb_var1 = *pb_var1 ^ (pi_var26 >> 8);
                b_var24 = *pb_var1;
                b_var13 = *pb_var1;
                pu_var65 = (pu_var64 + 1);
                u_var32 = u_var34;
                b_var11 = _in(u_var32);
                *pu_var64 = b_var11;
                if (-1 < b_var24) {
                    pu_var64 = pu_var64 + 1;
                    b_var11 = _in(u_var32);
                    *pu_var65 = b_var11;
                    pu_var7 = pu_var58;
                    pu_var58 = (pu_var58 + 2);
                    out(pu_var7, u_var32);
                    if (b_var13 == 0) {
                        // code_r0x10505eaa:
                        u_var16 = pc_var22;
                        pc_var6 = (pu_var58 + u_var20 + 0x75);
                        b_var11 = (pc_var22 >> 8);
                        *pc_var6 = *pc_var6 + b_var11;
                        if (*pc_var6 != '\0') {
                            pu_var55 = (pu_var58 + 2);
                            out(pu_var58, u_var34);
                            pu_var58 = (pu_var58 + 3);
                            out(*pu_var55, u_var34);
                            pu_var5 = pu_var64;
                            *pu_var5 = *pu_var5 & b_var11;
                            // goto code_r0x10505eb3;
                        }
                        pc_var6 = (pi_var26 + pu_var58);
                        c_var12 = pc_var22;
                        *pc_var6 = *pc_var6 + c_var12;
                        pc_var22 = (pi_var26 + pu_var58);
                        *pc_var22 = *pc_var22 + c_var12;
                        pc_var22 = (pi_var26 + pu_var58);
                        *pc_var22 = *pc_var22 + c_var12;
                        pc_var22 = (pi_var26 + pu_var58);
                        *pc_var22 = *pc_var22 + c_var12;
                        // goto code_r0x10505f2b;
                    }
                    pc_var6 = (pi_var26 + pu_var58);
                    c_var12 = *pc_var6;
                    *pc_var6 = *pc_var6 + pc_var22;
                    if (*pc_var6 != '\0' && SCARRY1(c_var12, pc_var22) == (*pc_var6 < '\0')) {
                        // goto ctx.PTR_LOOP_1050_5f1c;
                    }

                    // goto code_r0x10505f3b;
                }
                // code_r0x10505ec5:
                u_var16 = pc_var22;
                i_var15 = u_var34;
                i_var25 = u_var20;
                pc_var6 = (i_var25 + pu_var58);
                *pc_var6 = *pc_var6 + pc_var22;
                // goto code_r0x10505ebd;
            }
            pu_var55 = (pu_var58 + 2);
            out(pu_var58, u_var34);
            pu_var5 = (pu_var58 + 0x71);
            *pu_var5 = *pu_var5 + (pu_var58 != lVar9) * ((u_var18 & 3) - (*pu_var5 & 3));
            pb_var1 = (pi_var26 + pu_var55 + 0x72);
            b_var11 = (u_var34 >> 8);
            b_var70 = CARRY1(*pb_var1, b_var11);
            *pb_var1 = *pb_var1 + b_var11;
            // code_r0x10505e33:
            pu_var58 = (pu_var55 + 1);
            out(*pu_var55, u_var34);
            pu_var5 = (pi_var26 + pu_var58 + 0x69);
            *pu_var5 = *pu_var5 + b_var70 * ((u_var18 & 3) - (*pu_var5 & 3));
            pc_var6 = (pi_var26 + pu_var64 + 0x73);
            *pc_var6 = *pc_var6 + u_var18;
            pu_var5 = pu_var64;
            pu_var64 = (pu_var64 + 1);
            b_var11 = _in(u_var34);
            *pu_var5 = b_var11;
            *pc_var22 = *pc_var22 + pc_var22;
            if (*pc_var22 != '\0') {
                // code_r0x10505e44:
                out(*pu_var58, u_var34);
                pb_var1 = (pi_var26 + (pu_var58 + 1) + 0x72);
                b_var11 = *pb_var1;
                b_var13 = (u_var34 >> 8);
                *pb_var1 = *pb_var1 + b_var13;
                pu_var55 = (pu_var58 + 3);
                out((pu_var58 + 1), u_var34);
                pu_var5 = (pi_var26 + pu_var55 + 0x69);
                *pu_var5 = *pu_var5 + CARRY1(b_var11, b_var13) * ((u_var18 & 3) - (*pu_var5 & 3));
                pb_var1 = (pi_var26 + pu_var55 + 0x72);
                b_var70 = CARRY1(*pb_var1, b_var13);
                *pb_var1 = *pb_var1 + b_var13;
                // code_r0x10505e4f:
                c_var12 = u_var18;
                u_var32 = u_var34;
                out(*pu_var55, u_var32);
                pu_var5 = (pu_var55 + 0x71);
                *pu_var5 = *pu_var5 + b_var70 * ((u_var18 & 3) - (*pu_var5 & 3));
                pu_var60 = pu_var55 + 0x35;
                b_var13 = (u_var34 >> 8);
                *pu_var60 = *pu_var60 + b_var13;
                i_var25 = u_var20;
                pu_var58 = ((pu_var64 + i_var25 + 0x48) * 0x69);
                pu_var65 = pu_var64;
                if (*pu_var60 != '\0') {
                    lVar9 = (pu_var64 + i_var25 + 0x4c) * 0x6f;
                    pu_var58 = lVar9;
                    u_var18 = u_var18 - 1;
                    c_var12 = u_var18;
                    if (pu_var58 == lVar9) {
                        pu_var7 = pu_var58;
                        pu_var58 = (pu_var58 + 1);
                        out(*pu_var7, u_var32);
                        // goto code_r0x10505eaa;
                    }
                    pu_var5 = pu_var64;
                    pu_var64 = (pu_var64 + 1);
                    b_var11 = _in(u_var32);
                    *pu_var5 = b_var11;
                    pb_var1 = (pc_var22 + u_var20 * 2 + 0x69);
                    b_var70 = CARRY1(*pb_var1, b_var13);
                    *pb_var1 = *pb_var1 + b_var13;
                    b_var11 = *pb_var1;
                    if (b_var70) {
                        out(pu_var58, u_var32);
                        pu_var60 = (pu_var58 + 0x6a);
                        *pu_var60 = *pu_var60 + b_var13;
                        lVar9 = (pu_var64 + i_var25 + 0x48) * 0x69;
                        pu_var58 = lVar9;
                        b_var70 = pu_var58 != lVar9;
                        b_var68 = b_var70;
                        // goto code_r0x10505e76;
                    }
                    out(pu_var58, u_var32);
                    // goto code_r0x10505ebb;
                }
                // goto code_r0x10505ec5;
            }
            // code_r0x10505eb3:
            i_var25 = u_var20;
            u_var16 = pc_var22;
            pb_var1 = (pi_var26 + pu_var58);
            *pb_var1 = *pb_var1 ^ (pi_var26 >> 8);
            b_var24 = *pb_var1;
            b_var11 = *pb_var1;
            pu_var5 = pu_var64;
            pu_var64 = (pu_var64 + 1);
            b_var13 = _in(u_var34);
            *pu_var5 = b_var13;
            if (-1 < b_var24) {}
            // goto code_r0x10505eba;
        }
        pc_var22 = (pi_var26 + pu_var58);
        c_var12 = u_var16;
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        u_var16 = u_var16 & 0x646c;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + u_var16;
        // code_r0x10505ee9:
        pc_var22 = (pi_var26 + pu_var58);
        c_var12 = u_var16;
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pu_var5 = pu_var64;
        *pu_var5 = *pu_var5 + (u_var16 >> 8);
        // code_r0x10505ef5:
        b_var11 = _in(u_var34);
        *pu_var64 = b_var11;
        pc_var22 = (pi_var26 + pu_var58);
        c_var12 = u_var16;
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pc_var22 = (pi_var26 + pu_var58);
        *pc_var22 = *pc_var22 + c_var12;
        pb_var1 = (pu_var64 + 1);
        *pb_var1 = *pb_var1 + (u_var16 >> 8);
    }
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + u_var16;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + u_var16;
    pc_var22 = (i_var25 + 0x1f);
    *pc_var22 = *pc_var22 + (pi_var26 >> 8);
    // ctx.PTR_LOOP_1050_5f1c:
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + u_var34;
    pc_var22 = (pi_var26 + pu_var58);
    c_var12 = u_var16;
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = (pi_var26 + pu_var58);
    *pi_var4 = *pi_var4 + u_var16;
    pi_var4 = (pi_var26 + pu_var58);
    *pi_var4 = *pi_var4 + u_var16;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;
    pi_var4 = (pi_var26 + pu_var58);
    *pi_var4 = *pi_var4 + u_var16;
    pc_var22 = (pi_var26 + pu_var58);
    *pc_var22 = *pc_var22 + c_var12;

    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}
