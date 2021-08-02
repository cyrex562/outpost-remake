use std::default::default;

use crate::defines::U32Ptr;
use crate::file::file_1008::{read_file_1008_7cfe, read_file_1008_7dee, write_to_file_1008_7a22, write_to_file_1008_7cac};
use crate::file::file_1030::file_1030_1730;
use crate::pass::pass_1008::{pass1_1008_5784, pass1_1008_5b12};
use crate::pass::pass_1028::{pass1_1028_dc52, pass1_1028_e0bc, pass1_1028_e100, pass1_1028_e4ec, pass1_1028_e628};
use crate::pass::pass_1030::pass1_1030_16d6;
use crate::pass::pass_1038::pass1_1038_3ba0;
use crate::switch_ops::switch_1008::{switch_1008_72bc, switch_1008_73ea};
use crate::util::{CONCAT22, ZEXT24};
use crate::global::AppContext;

pub fn write_to_file_1028_0234(param_1: u32, param_2: u32, param_3: u16) -> u16

{
    let u_var1: u32;
    let BVar2: bool;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let local_1a: [u16; 0x3];
    let local_14: [u16; 0x2];
    let uStack16: u16;
    let lStack14: i32;
    let local_a: [u8; 8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (BVar2 != 0x0) {
        // u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        local_1a[0] = (i_var3 + 0x20);
        u_var5 = param_2;
        // u_var6 = (param_2 >> 0x10);
        BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_1a, param_3, 0x2, 0x1008);
        if (BVar2 != 0x0) {
            pass1_1008_5784(CONCAT22(param_3, local_a), (i_var3 + 0x22));
            u_var1 = (i_var3 + 0x22);
            local_14[0] = (u_var1 + 0x8);
            uStack16 = local_14[0];
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_3, 0x2, 0x1008);
            while (BVar2 != 0x0) {
                lStack14 = pass1_1008_5b12(local_a, param_3);
                if (lStack14 == 0x0) {
                    return 0x1;
                }
                local_14[0] = (lStack14 + 0x4);
                BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_3, 0x2, 0x1008);
                if (BVar2 == 0x0) { break; }
                local_14[0] = (lStack14 + 0x6);
                BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_3, 0x2, 0x1008);
                if (BVar2 == 0x0) { break; }
                local_14[0] = (lStack14 + 0x8);
                BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_3, 0x2, 0x1008);
                if (BVar2 == 0x0) { break; }
                local_14[0] = (lStack14 + 0xa);
                BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_3, 0x2, 0x1008);
                if (BVar2 == 0x0) { break; }
                local_14[0] = (lStack14 + 0xc);
                BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_3, 0x2, 0x1008);
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn write_to_file_1028_1452(param_1: u32, param_2: u32, param_3: u16) -> u16

{
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let local_c: [u16; 0x3];
    u8 * local_6[0x2];

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (b_var1 != 0x0) {
        // u_var2 = (param_1 >> 0x10);
        local_c[0] = (param_1 + 0x22);
        u_var3 = param_2;
        // u_var4 = (param_2 >> 0x10);
        b_var1 = write_to_file_1008_7e1c(u_var3, u_var4, local_c, param_3, 0x2, 0x1008);
        if (b_var1 != 0x0) {
            local_6[0] = (param_1 + 0x20);
            b_var1 = write_to_file_1008_7e1c(u_var3, u_var4, local_6, param_3, 0x2, 0x1008);
            if (b_var1 != 0x0) {
                local_6[0] = ctx.PTR_LOOP_1050_4fbc;
                b_var1 = write_to_file_1008_7e1c(u_var3, u_var4, local_6, param_3, 0x2, 0x1008);
                if (b_var1 != 0x0) {
                    return 0x1;
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn file_1028_24a2(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) -> bool

{
    let u_var1: u32;
    let ppcVar2: u32;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let extraout_dx: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let u_var10: u16;
    let uVar9: u32;
    let uStack6: u16;
    let local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 == 0x0) {
        return 0x0;
    }
    if (0x1 < ctx.PTR_LOOP_1050_0312) {
        BVar3 = read_file_1008_7dee(param_2, (param_2 >> 0x10), &local_4, 0x0, param_5, 0x2, 0x1008);
        if (BVar3 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return 0x0;
        }
        // TODO: refactor for loop
        // for (uStack6 = 0x0; uStack6 < local_4; uStack6 += 0x1) {
        //   uVar8 = 0x2a;
        //   u_var6 = local_4;
        //   uVar9 = param_2;
        //   mem_op_1000_179c(0x2a,param_4,0x1000);
        //   puVar7 = (param_4 | u_var6);
        //   if (puVar7 == 0x0) {
        //     u_var4 = 0x0;
        //     puVar7 = 0x0;
        //   }
        //   else {
        //     u_var5 = u_var6;
        //     struct_1038_6520(CONCAT22(param_4,u_var6));
        //     u_var4 = u_var6;
        //     u_var6 = u_var5;
        //   }
        //   u_var10 = (uVar9 >> 0x10);
        //   u_var5 = u_var4;
        //   file_1038_774e(CONCAT22(puVar7,u_var4),CONCAT22(uVar9,uVar8),puVar7,param_5);
        //   if (u_var5 == 0x0) {
        //     return 0x0;
        //   }
        //   uVar8 = (param_1 >> 0x10);
        //   u_var1 = (param_1 + 0x20);
        //   ppcVar2 = ((param_1 + 0x20) + 0x8);
        //   (**ppcVar2)(&ctx.PTR_LOOP_1050_1038,u_var1,(u_var1 >> 0x10),u_var4,
        //               puVar7,u_var10,u_var6);
        //   param_4 = extraout_dx;
        // }
    }
    return 0x1;
}


pub fn write_to_file_1028_3d0e(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    let local_10: [u32; 0x2];
    let local_8: u32;

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (b_var1 != 0x0) {
        // u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        local_10[0] = (i_var2 + 0x20);
        // u_var4 = (param_2 >> 0x10);
        b_var1 = write_to_file_1008_7e1c(param_2, u_var4, local_10, param_3, 0x4, 0x1008);
        if (b_var1 != 0x0) {
            local_8 = (i_var2 + 0x24);
            b_var1 = write_to_file_1008_7e1c(param_2, u_var4, &local_8, param_3, 0x4, 0x1008,
            );
            if (b_var1 != 0x0) {
                write_to_file_1008_7a22(param_2, (i_var2 + 0x28), 0x1008, param_3);
                if (b_var1 != 0x0) {
                    return;
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


pub fn write_to_file_1028_5f82(param_1: u32, param_2: u32, param_3: u16) -> bool

{
    let b_var1: bool;
    let local_c: [u16; 0x5];

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (b_var1 != 0x0) {
        local_c[0] = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), local_c, param_3,
                                         0x2, 0x1008);
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 0x1;
    }
    return b_var1;
}


pub fn pass1_1028_5fcc(param_1: i16, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let BVar3: bool;

    file_1028_b81a((param_3 + 0x6), (param_3 + 0xa), param_1, param_4,
                   param_2);
    if ((param_1 != 0x0) && (u_var1 = (param_3 + 0x6), u_var2 = (param_3 + 0xa),
                             BVar3 = read_file_1008_7dee(u_var2, (u_var2 >> 0x10),
                                                         u_var1 + 0x20, 0x0, (u_var1 >> 0x10), 0x2,
                                                         0x1008), BVar3 == 0x0)) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    return;
}


pub fn write_to_file_1028_b286(param_1: i16, param_2: u16) -> bool

{
    let u_var1: u32;
    let in_AX: bool;
    let BVar2: bool;

    pass1_1030_16d6((param_1 + 0x6), (param_1 + 0xa), param_2);
    if (in_AX != 0x0) {
        u_var1 = (param_1 + 0x6);
        (param_1 + -0xa) = (u_var1 + 0xc);
        u_var1 = (param_1 + 0xa);
        BVar2 = write_to_file_1008_7e1c(u_var1, (u_var1 >> 0x10), param_1 - 0xa, param_2,
                                        0x2, 0x1008);
        if (BVar2 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return BVar2;
        }
        in_AX = 0x1;
    }
    return in_AX;
}


pub fn write_to_file_1028_b5ec(param_1: u32, param_2: u32, param_3: u16) -> bool

{
    let u_var1: u32;
    let BVar2: bool;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let local_e: [u16; 0x3];
    let local_8: [u16; 0x2];
    let i_stack4: i16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    local_e[0] = (i_var3 + 0xc);
    u_var5 = param_2;
    // u_var6 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_e, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    pass1_1030_16d6(param_1, param_2, param_3);
    if (BVar2 == 0x0) {
        return 0x0;
    }
    local_8[0] = (i_var3 + 0xc);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    local_8[0] = (i_var3 + 0xe);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    local_8[0] = (i_var3 + 0x10);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    local_8[0] = (i_var3 + 0x12);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    local_8[0] = (i_var3 + 0x18);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    local_8[0] = (i_var3 + 0x1a);
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    i_stack4 = (i_var3 + 0x12);
    if (i_stack4 == 0x6) {
        i_stack4 = (i_var3 + 0x18);
    }
    if (i_stack4 < 0x1) {
        return 0x1;
    }
    if (SBORROW2(i_stack4, 0x1)) {
        return 0x1;
    }
    if (i_stack4 == 0x3 || i_stack4 + -0x2 < 0x1) {
        local_8[0] = (i_var3 + 0x14);
    } else {
        if (i_stack4 == 0x4) {
            if ((i_var3 + 0x14) == 0x0) {
                local_8[0] = 0x0;
                BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
//         TODO: goto joined_r0x1028b766;
            }
            u_var1 = (i_var3 + 0x14);
            local_8[0] = (u_var1 + 0x94);
        } else {
            if (i_stack4 != 0x5) {
                return 0x1;
            }
            u_var1 = (i_var3 + 0x14);
            local_8[0] = (u_var1 + 0xa4);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return 0x0;
            }
            u_var1 = (i_var3 + 0x14);
            local_8[0] = (u_var1 + 0xa6);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return 0x0;
            }
            u_var1 = (i_var3 + 0x14);
            local_8[0] = (u_var1 + 0xa8);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return 0x0;
            }
            u_var1 = (i_var3 + 0x14);
            local_8[0] = (u_var1 + 0xaa);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return 0x0;
            }
            u_var1 = (i_var3 + 0x14);
            local_8[0] = (u_var1 + 0xac);
        }
    }
    BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, local_8, param_3, 0x2, 0x1008);
// joined_r0x1028b766:
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return 0x0;
    }
    return 0x1;
}


// WARNING: Unable to use type for symbol pu_var3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn file_1028_b81a(param_1: u32, param_2: u32, param_3: i16, param_4: u16, param_5: U32Ptr) {
    let b_var1: bool;
    let i_var2: i16;
    let puVar4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let uVar8: u16;
    let local_2a: [u16; 0x2];
    let local_26: [u8; 16];
    let puStack16: u32;
    let puStack14: U32Ptr;
    let uStack10: i16;
    let local_8: i16;
    let local_6: i16;
    let local_4: i16;
    let pu_var3: u32;

    pu_var3 = param_1;
    // u_var6 = (param_1 >> 0x10);
    file_1030_1730(param_1, param_2);
    if (param_3 == 0x0) {
        return;
    }
    u_var5 = param_2;
    // uVar8 = (param_2 >> 0x10);
    b_var1 = read_file_1008_7dee(u_var5, uVar8, &local_4, 0x0, param_4, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    b_var1 = read_file_1008_7dee(u_var5, uVar8, &local_6, 0x0, param_4, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    b_var1 = read_file_1008_7dee(u_var5, uVar8, &local_8, 0x0, param_4, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    i_var2 = switch_1008_73ea(u_var5, uVar8, local_4);
    (pu_var3 + 0x3) = i_var2;
    i_var2 = switch_1008_73ea(u_var5, uVar8, local_6);
    (pu_var3 + 0xe) = i_var2;
    i_var2 = switch_1008_73ea(u_var5, uVar8, local_8);
    (pu_var3 + 0x4) = i_var2;
    b_var1 = read_file_1008_7dee(u_var5, uVar8, &local_4, 0x0, param_4, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    b_var1 = read_file_1008_7dee(u_var5, uVar8, &local_6, 0x0, param_4, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    b_var1 = read_file_1008_7dee(u_var5, uVar8, pu_var3 + 0x1a, 0x0, u_var6, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    (pu_var3 + 0x12) = local_4;
    (pu_var3 + 0x6) = local_6;
    iStack10 = (pu_var3 + 0x12);
    if (iStack10 == 0x6) {
        iStack10 = (pu_var3 + 0x6);
    }
    if (false) {
        return;
    }
    match (iStack10) {
        0x1 | 0x2 | 0x3 => {
            puVar4 = pu_var3 + 0x5;
//LAB_1028_b968:
            b_var1 = read_file_1008_7dee(u_var5, uVar8, puVar4, 0x0, u_var6, 0x2, 0x1008);
        }
        0x4 => {
            uVar7 = pass1_1028_e0bc(ctx.PTR_LOOP_1050_65e2, (pu_var3 + 0x3), pu_var3, param_5,
                                    param_4);
            // puStack14 = (uVar7 >> 0x10);
            (pu_var3 + 0x5) = uVar7;
            (pu_var3 + 0x16) = puStack14;
            if ((puStack14 | (pu_var3 + 0x5)) != 0x0) {
                puVar4 = ((pu_var3 + 0x5) + 0x94);
                u_var6 = puStack14;
                puStack16 = puVar4;
//       TODO: goto LAB_1028_b968;
            }
            b_var1 = read_file_1008_7dee(u_var5, uVar8, local_26, 0x0, param_4, 0x2, 0x1008);
        }

        0x5 => {
            puVar4 = pu_var3;
            pass1_1028_e100(ctx.PTR_LOOP_1050_65e2, (pu_var3 + 0x3), param_5);
            (pu_var3 + 0x5) = puVar4;
            (pu_var3 + 0x16) = param_5;
            puStack16 = ((pu_var3 + 0x5) + 0xa4);
            puStack14 = param_5;
            b_var1 = read_file_1008_7dee(u_var5, uVar8, puStack16, 0x0, param_5, 0x2,
                                         0x1008);
            if (b_var1 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            b_var1 = read_file_1008_7dee(u_var5, uVar8, local_2a, 0x0, param_4, 0x2, 0x1008);
            if (b_var1 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            uVar7 = pu_var3[0x5];
            b_var1 = read_file_1008_7dee(u_var5, uVar8, uVar7 + 0xa8, 0x0, (uVar7 >> 0x10),
                                         0x2, 0x1008);
            if (b_var1 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            uVar7 = pu_var3[0x5];
            b_var1 = read_file_1008_7dee(u_var5, uVar8, uVar7 + 0xaa, 0x0, (uVar7 >> 0x10),
                                         0x2, 0x1008);
            if (b_var1 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            uVar7 = pu_var3[0x5];
            b_var1 = read_file_1008_7dee(u_var5, uVar8, uVar7 + 0xac, 0x0, (uVar7 >> 0x10),
                                         0x2, 0x1008);
            if (b_var1 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            u_var5 = switch_1008_72bc(u_var5, uVar8, local_2a[0]);
            uVar7 = pu_var3[0x5];
            (uVar7 + 0xa6) = u_var5;
            return;
        }
        // default:
//     TODO: goto switchD_1028_ba97_caseD_6;
        0x9 => {
            puVar4 = pu_var3;
            pass1_1028_e100(ctx.PTR_LOOP_1050_65e2, (pu_var3 + 0x3), param_5);
            (pu_var3 + 0x5) = puVar4;
            (pu_var3 + 0x16) = param_5;
        }
//     TODO: goto switchD_1028_ba97_caseD_6;
    }
    if b_var1 == 0x0 {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
// switchD_1028_ba97_caseD_6:
    return;
}


pub fn read_file_1028_d7ba(
    ctx: &mut AppContext,
    param_3: u32,
    param_4: u16,
    param_5: u16
) -> u16

{
    read_file_1008_7cfe(param_3, 0x8, 0x1008, param_4);
    if param_5 == 0x0 {
        ctx.PTR_LOOP_1050_0310 = 0x6d4;
        return param_5;
    }
    return 0x1;
}


pub fn write_to_file_1028_dce2(
    param_1: U32Ptr,
    param_2: u32,
    param_3: u16,
    in_dx: u16,
    extraout_dx: u16,
) -> u32

{
    let ppc_var1: u32;
    let bvar2: bool;
    let pu_var3: U32Ptr;
    // let in_dx: u16;
    // let extraout_dx: u16;
    let u_var4: u16;
    let i_var5: i16;
    let u_var6: u16;
    let u_var7: u16;
    let local_26: [u32; 0x2];
    let local_1e: [u16; 0x3];
    let u_stack24: u32;
    let local_14: [u8; 12];

    // u_var7 = (param_2 >> 0x10);
    bvar2 = write_to_file_1008_7cac(param_2, param_3);
    if bvar2 != 0x0 {
        local_26[0] = *param_1;
        bvar2 = write_to_file_1008_7e1c(param_2, u_var7, local_26, param_3, 0x4, 0x1008);
        if bvar2 != 0x0 {
            // u_var6 = (param_1 >> 0x10);
            i_var5 = param_1;
            local_1e[0] = (i_var5 + 0x8);
            bvar2 = write_to_file_1008_7e1c(param_2, u_var7, local_1e, param_3, 0x2, 0x1008,
            );
            if bvar2 != 0x0 {
                ppc_var1 = (*_PTR_LOOP_1050_5166 + 0xc);
                (**ppc_var1)(0x1008, _PTR_LOOP_1050_5166,
                             (ctx.PTR_LOOP_1050_5166 >> 0x10), param_2);
                in_dx = extraout_dx;
                if bvar2 != 0x0 {
                    bvar2 = write_to_file_1008_7cac(param_2, param_3);
                    if bvar2 != 0x0 {
                        in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                        if (bvar2 != 0x0) {
                            bvar2 = write_to_file_1008_7cac(param_2, param_3);
                            if (bvar2 != 0x0) {
                                in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                if (bvar2 != 0x0) {
                                    bvar2 = write_to_file_1008_7cac(param_2, param_3);
                                    if (bvar2 != 0x0) {
                                        in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                        if (bvar2 != 0x0) {
                                            bvar2 = write_to_file_1008_7cac(param_2, param_3);
                                            if (bvar2 != 0x0) {
                                                in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                                if (bvar2 != 0x0) {
                                                    bvar2 = write_to_file_1008_7cac(param_2, param_3);
                                                    if (bvar2 != 0x0) {
                                                        in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                                        if (bvar2 != 0x0) {
                                                            bvar2 = write_to_file_1008_7cac(param_2, param_3);
                                                            if (bvar2 != 0x0) {
                                                                in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                                                if (bvar2 != 0x0) {
                                                                    bvar2 = write_to_file_1008_7cac(param_2, param_3);
                                                                    if (bvar2 != 0x0) {
                                                                        in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                                                        if (bvar2 != 0x0) {
                                                                            bvar2 = write_to_file_1008_7cac(param_2, param_3);
                                                                            if (bvar2 != 0x0) {
                                                                                in_dx = write_file_fn_1028_e56c(i_var5, u_var6, param_2, param_3);
                                                                                if (bvar2 != 0x0) {
                                                                                    pass1_1028_dc52(
                                                                                        CONCAT22(param_3, local_14), 0x1,
                                                                                        0x0, 0x400);
                                                                                    loop {
                                                                                        u_var4 = in_dx;
                                                                                        pu_var3 = local_14;
                                                                                        pass1_1028_e4ec(CONCAT22(param_3, pu_var3));
                                                                                        u_stack24 = CONCAT22(u_var4, pu_var3);
                                                                                        in_dx = u_var4 | pu_var3;
                                                                                        if (in_dx == 0x0) { break; }
                                                                                        if ((pu_var3 + 0x200) != 0x8000002) {
                                                                                            pass1_1038_3ba0(CONCAT22(u_var4, pu_var3));
                                                                                        }
                                                                                    }
                                                                                    return 0x10000;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return in_dx;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1028_def2(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let ppcVar1: u32;
    let BVar2: bool;
    let unaff_SI: u16;
    let unaff_DI: u16;
    let u_var3: u16;
    let in_AF: u8;
    let u_var4: u16;
    let u_var5: u16;

    u_var4 = param_2;
    // u_var5 = (param_2 >> 0x10);
    read_file_1008_7cfe(u_var4, 0xa, 0x1008, param_3);
    if (param_4 != 0x0) {
        // u_var3 = (param_1 >> 0x10);
        BVar2 = read_file_1008_7dee(u_var4, u_var5, param_1, 0x0, u_var3, 0x4, 0x1008);
        if (BVar2 != 0x0) {
            BVar2 = read_file_1008_7dee(u_var4, u_var5, param_1 + 0x8, 0x0, u_var3, 0x2, 0x1008);
            if (BVar2 != 0x0) {
                u_var3 = (*_PTR_LOOP_1050_5166 >> 0x10);
                ppcVar1 = (*_PTR_LOOP_1050_5166 + 0x10);
                (**ppcVar1)(0x1008, _PTR_LOOP_1050_5166,
                            (ctx.PTR_LOOP_1050_5166 >> 0x10), param_2);
                if (BVar2 != 0x0) {
                    read_file_1008_7cfe(u_var4, 0xc, 0x1008, param_3);
                    if (BVar2 != 0x0) {
                        pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x100, unaff_SI, unaff_DI, u_var3, param_3,
                                        in_AF);
                        if (BVar2 != 0x0) {
                            read_file_1008_7cfe(u_var4, 0xd, 0x1008, param_3);
                            if (BVar2 != 0x0) {
                                pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x200, unaff_SI, unaff_DI, u_var3,
                                                param_3, in_AF);
                                if (BVar2 != 0x0) {
                                    read_file_1008_7cfe(u_var4, 0xe, 0x1008, param_3);
                                    if (BVar2 != 0x0) {
                                        pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x300, unaff_SI, unaff_DI, u_var3,
                                                        param_3, in_AF);
                                        if (BVar2 != 0x0) {
                                            read_file_1008_7cfe(u_var4, 0xf, 0x1008, param_3);
                                            if (BVar2 != 0x0) {
                                                pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x400, unaff_SI, unaff_DI,
                                                                u_var3, param_3, in_AF);
                                                if (BVar2 != 0x0) {
                                                    read_file_1008_7cfe(u_var4, 0x10, 0x1008, param_3);
                                                    if (BVar2 != 0x0) {
                                                        pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x500, unaff_SI,
                                                                        unaff_DI, u_var3, param_3, in_AF);
                                                        if (BVar2 != 0x0) {
                                                            read_file_1008_7cfe(u_var4, 0x11, 0x1008, param_3);
                                                            if (BVar2 != 0x0) {
                                                                pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x600, unaff_SI,
                                                                                unaff_DI, u_var3, param_3, in_AF);
                                                                if (BVar2 != 0x0) {
                                                                    read_file_1008_7cfe(u_var4, 0x12, 0x1008, param_3);
                                                                    if (BVar2 != 0x0) {
                                                                        pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x700, unaff_SI, unaff_DI, u_var3, param_3, in_AF);
                                                                        if (BVar2 != 0x0) {
                                                                            read_file_1008_7cfe(u_var4, 0x13, 0x1008, param_3);
                                                                            if (BVar2 != 0x0) {
                                                                                pass1_1028_e628(param_1, u_var4, u_var5, 0x0, 0x800,
                                                                                                unaff_SI, unaff_DI, u_var3, param_3,
                                                                                                in_AF);
                                                                                if (BVar2 != 0x0) {
                                                                                    return;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}


pub fn write_file_fn_1028_e56c(param_1: u16, param_2: u16, param_3: u32, param_4: u16) -> u16

{
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let BVar3: bool;
    let in_DX: u16;
    let extraout_dx: u16;
    let in_stack_0000000c: u32;
    let local_2a: [u32; 0x3];
    let puStack28: u32;
    let uStack24: u32;
    let local_14: [u8; 8];
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let i_stack4: i16;

    pass1_1028_dc52(CONCAT22(param_4, local_14), 0x1, in_stack_0000000c,
                    (in_stack_0000000c >> 0x10));
    uStack24 = 0x0;
    loop {
        pu_var2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
        puStack28 = CONCAT22(in_DX, pu_var2);
        in_DX |= pu_var2;
        if (in_DX == 0x0) { break; }
        uStack24 += 0x1;
    }
    local_2a[0] = uStack24;
    BVar3 = write_to_file_1008_7e1c(param_3, (param_3 >> 0x10), local_2a,
                                    param_4, 0x4, 0x1008);
    if (BVar3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    } else {
        uStack12 = uStack8;
        uStack10 = uStack6;
        if (i_stack4 != 0x0) {
            uStack12 = 0x1;
            uStack6 = 0x0;
            uStack10 = uStack6;
        }
        loop {
            pu_var2 = local_14;
            pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
            puStack28 = CONCAT22(uStack6, pu_var2);
            if ((uStack6 | pu_var2) == 0x0) {
                return 0x0;
            }
            ppcVar1 = (*puStack28 + 0xc);
            (**ppcVar1)(0x1008, pu_var2, uStack6);
            local_2a[0] = local_2a[0] & 0xffff0000 | ZEXT24(pu_var2);
            uStack6 = extraout_dx;
            in_DX = extraout_dx;
            if pu_var2 == 0 { break; }
        }
    }
    return in_DX;
}

