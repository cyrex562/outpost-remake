use crate::app_context::AppContext;
use crate::block_1000;
use crate::unk::block_1000_1000;
use crate::block_1000::{mem_op_1000_131c, mem_op_1000_1408, mem_op_1000_14f2, pass1_1000_1e61};
use crate::globals::{
    DAT_1050_5f30, PTR_LOOP_1050_000a, PTR_LOOP_1050_000c, PTR_LOOP_1050_000e, REG_CS, u16_1050_0002,
    u16_1050_0008, u32_1050_0004, u32_1050_0006,
};
use crate::mem_ops::{alloc_mem_1000_131c, free_mem_1000_13ce, mem_op_1000_1532, realloc_1000_1408};
use crate::structs::struct_1000_07ac_1::Struct_1000_07ac_1;
use crate::utils::{CARRY2, CONCAT11, CONCAT22};
use std::os::raw::c_char;
use std::ptr;
use std::ptr::null_mut;
use crate::unk::block_1000_1000::{mem_op_1000_14f2, pass1_1000_1284, pass1_1000_1e61};
use crate::mem_container::MemContainer;
use crate::structs::struct_1000_05b4_1::Struct100005b41;
use crate::structs::struct_611::Struct611;
use crate::structs::struct_7::Struct7;
use crate::structs::struct_99::astruct_99;

use super::mem_op_1000_13ce;
use crate::unk::block_1000_5000::pass1_1000_5390;

pub unsafe fn mem_1000_0016(ctx: &mut AppContext, mut param_1: u32) -> u32 {
    let mut u_var1: u32;
    if (param_1 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx.CS_REG.clone(), 0xa, ptr::null_mut());
        return 0xffffffff;
    }
    u_var1 = mem_op_1000_0052(ptr::null_mut());
    return u_var1;
}

pub unsafe fn goto_lab_1000_00f9(pstruct7_param_1: *mut Struct7) -> u32{
    // let mut pu8_var1 = pstruct7_param_1.field_0x1e.clone();
    // let mut pu16_var2 = pstruct7_param_1.field_0x1e as *mut u16;
    // let mut i16_var3 = (*pstruct7_param_1.field_0x20).clone();
    // let a = (i16_var3 - (pstruct7_param_1.field_0x20)) - (pu8_var1 < (*pu16_var2).clone() as u8);
    // let b = pu8_var1 - (*pu16_var2).clone();
    // TODO: I'm not sure but I think all of this is concatenating the 16-bit pointer addresses for field_0x1e and field_x20 into a single address segment:offset
    return CONCAT22(a, b);
}

pub unsafe fn mem_op_1000_0052(pstruct7_param_1: *mut Struct7) -> u32 {
    let mut pu16_var1: *mut u16;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut pu8_var14: *mut u8 = ptr::null_mut();
    let mut pu8_var12: *mut u8 = ptr::null_mut();
    let mut pu8_var10: *mut u8 = ptr::null_mut();

    let mut u_stack8 = 0u16;
    loop {
        pu8_var10 = (u_stack8.clone() * 0x2 + pstruct7_param_1);
        if (!pu8_var10.is_null()) && (u_stack8 != 0x3) {
            pu8_var14 = ptr::null_mut();
            loop {
                pu8_var12 = (pu8_var10 + 0x4);
                u_var4 = (pu8_var10 + 0x8);
                if (u_var4 + 0xa) == 0x0 {
                    u_var5 = mem_op_1000_0510(0x1, ptr::null_mut());
                    if u_var5 == 0x0 {
                        // goto LAB_1000_00f9;
                        return goto_lab_1000_00f9(pstruct7_param_1);
                    }
                    if pu8_var12 == pu8_var10 {
                        pu8_var12 = ptr::null_mut();
                    }
                } else if pu8_var14.is_null() {
                    pu8_var14 = pu8_var10;
                }
                pu8_var10 = pu8_var12;
                if pu8_var12 == pu8_var14 {
                    break;
                }
            } //while i_stack12 != i_stack14;
        }
        u_stack8 += 0x1;
        if u_stack8 >= 0x5 {
            break;
        }
    } //while u_stack8 < 0x5;
    if (pstruct7_param_1.field_0x32) != 0x0 {
        (pstruct7_param_1.field_0x32)();
    }
    // TODO: LAB_1000_00f9:
    // pu16_var1 = pstruct7_param_1.field_0x1e as *mut u16;
    // let a = (i_var3 - (pstruct7_param_1.field_0x20)) - (u_var2 < (*pu16_var1).clone() as u8);
    // let b = u_var2 - (*pu16_var1).clone();
    // return CONCAT22(a, b);
    return goto_lab_1000_00f9(pstruct7_param_1);
}

pub unsafe fn pass1_1000_010c(
    ctx: &mut AppContext,
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: *mut Struct7,
    mut param_5: u16,
) -> u16 {
    let mut u_var1: u16;
    let mut uvar2: u16;
    let mut u_var2: u16;
    // let mut unaff_cs: u16;
    let mut b_var3: bool;
    let mut uvar4: u16;
    let mut u_stack8: u16;
    let mut u_stack6: u16;
    let mut u_stack4: u16;

    u_stack6 = 0;
    u_stack8 = 0;
    if (param_4 + 0x14) != -0x4153 {
        param_5 = 0;
        param_4 = ptr::null_mut();
        uvar4 = 0xa;
        // code_r0x10000128:
        pass1_1000_1e61(ctx.CS_REG.clone(), uvar4, param_4);
        return 0xffff;
    }
    DAT_1050_5f30 = 0x1;
    if param_1 == 0x1 {
        u_stack4 = 0x1;
        if (*param_4.field_0x18) == 0x0 {
            uvar4 = 0x4;
            // goto code_r0x10000128;
        }
    } else if param_1 == 0x2 {
        u_stack4 = 0x2;
    } else {
        if param_1 != 0x4 {
            DAT_1050_5f30 = 0x1;
            return 0xffff;
        }
        u_stack4 = 0;
    }
    // s_version__d__d_1050_0012 + 0x8
    uvar2 = mem_op_1000_03c6(ctx, 0x001a, 0x0, u_stack4, ptr::null_mut(), 0x0, 0);
    u_var1 = u_stack4.clone();
    while u_stack6 <= param_3 && ((u_stack6 < param_3 || (u_stack8 < param_2)) && ((uvar2.clone() | u_var1.clone()) != 0)) {
        u_var1 = (0x00a1a);
        b_var3 = CARRY2(u_stack8.clone(), u_var1);
        u_stack8 += u_var1.clone();
        u_stack6 += b_var3;
    }
    return u_stack6;
}

pub unsafe fn mem_op_1000_01b0(ctx: &mut AppContext, struct7_param_1: *mut Struct7) -> bool {
    let mut bvar3: bool = false;
    let mut uvar4: u16 = 0;
    let mut u16_var5: u16 = 0;
    let mut u32_var6: u32 = 0;
    let mut u32_var7: u32 = 0;
    let mut u_var8: u32 = 0;
    let mut u16_var9 = 0u16;
    let mut pu8_var10: *mut u8 = ptr::null_mut();
    // let mut u_stack14: u16 = 0;
    let mut u_stack12: u16 = 0;
    let mut u_stack6: u16 = 0;
    let mut i_stack4: i16 = 0;

    let mut u_stack14 = 0u16;
    if ((struct7_param_1.field_0x40.clone()) | (struct7_param_1.field_0x3e.clone())) == 0x0 {
        u16_var5 = struct7_param_1.field_0x36.clone();
        u32_var6 = mem_op_1000_1532(struct7_param_1, 0x1050);
        u32_var7 = u32_var6;
    } else {
        u32_var6 = mem_op_1000_1532(struct7_param_1, 0x1050);
        u16_var5 = u32_var6 as u16;
        if ((u32_var6.clone() >> 0x10) != 0) || (0xffef < u16_var5) {
           pass1_1000_1e61(ctx.CS_REG.clone(), 0x8, struct7_param_1);
            return false;
        }
        if 0x1fff < u16_var5 {
            u16_var5 = 0x2000;
        }
        loop {
            u16_var9 = u16_var5;
            u32_var7 = u32_var6.clone() + 0x18;
            if ((u32_var7 >> 0x10) != 0) || (0xfff0 < u32_var7) {
                u32_var7 = 0xfff0;
            }
            bvar3 = mem_op_1000_14f2((struct7_param_1.field_0x16.clone()) | 0x1000, u32_var7.clone(), ptr::null_mut(), 0);
            i_stack4 = (u32_var6.clone() >> 0x10) as i16;
            u_stack6 = u32_var6.clone() as u16;
            if bvar3 != false {
                break;
            }
            u16_var5 = u16_var9 >> 0x1;
            if u16_var5 < 0xc {
                uvar4 = pass1_1000_1e61(ctx.CS_REG.clone(), 0x2, struct7_param_1);
                if uvar4 == 0x0 {
                    return '\x01' - ((*struct7_param_1.field_0xa) == 0);
                }
                u32_var6 = mem_op_1000_1532(struct7_param_1, 0x1050);
                u16_var5 = u16_var9.clone() & 0xfffe;
            }
        }
        u_var8 = pass1_1000_5390(u32_var6.clone() - 42, 0xc, null_mut());
        u16_var5 = u_var8 * 0xc + struct7_param_1 + 0x42;
    }
    // pu_var1 =  (param_1 + 0x1e);
    let mut u16_var1 = struct7_param_1.segment_field_0x1e.clone();
    // u16_var9 = *pu_var1;
    u16_var9 = (u16_var1).clone();
    u16_var1 = (u16_var1).clone() - u32_var6.clone();
    // pi_var2 =  (param_1 + 0x20);
    let mut u16_var2 = struct7_param_1.offset_field_0x20.clone();
    u16_var2 = ((u16_var2).clone() - (u32_var6.clone() >> 0x10)) - (u16_var9 < u32_var6.clone() as u16);
    if u16_var5 != 0x0 {
        pu8_var10 = ptr::null_mut();
        u16_var9 = 0xc;
        u32_var7 = mem_op_1000_1532(struct7_param_1, 0x1050);
        u_var8 = pass1_1000_5390(
            u32_var7 - 0x42,
            u16_var9,
            pu8_var10,
        );
        u_stack14 = u_var8 * 0xc + struct7_param_1 + 0x36;
    }
    let i_stack10 = (u32_var7.clone() >> 0x10) as u16;
    u_stack12 = u32_var7.clone() as u16;
    u16_var1 = struct7_param_1.segment_field_0x1e.clone();
    u16_var9 = (u16_var1).clone();
    u16_var1 = (u16_var1).clone() + u_stack12;
    u16_var2 = struct7_param_1.offset_field_0x20.clone();
    u16_var2 = (u16_var2).clone() + i_stack10 + CARRY2(u16_var9, u_stack12.clone());
    u16_var9 = (struct7_param_1.field_0xa);
    loop {
        *pu8_var10 = u16_var5 as u8;
        *(pu8_var10 + 0x4) = u16_var9;
        u16_var9 = (*pu8_var10).clone() as u16;
        u16_var5 = *(pu8_var10 + 0xc);
        if !(*pu8_var10 < u_stack14.clone() as u8) {
            break;
        }
    } // while (*pu8_var10 < u_stack14);
    struct7_param_1.field_0xa = (*pu8_var10).clone();
    return true;
}

pub unsafe fn mem_op_1000_0308(
    ctx: &mut AppContext,
    mut param_1: i16,
    pstruct7_param_2: *mut Struct7,
) -> *mut u8 {
    let mut pu8_var1: *mut u8;
    let mut pu8_var2: *mut u8;
    let mut b_var3: bool;
    let mut pu8_var4: *mut u8;

    if (pstruct7_param_2 + 0xa) == 0x0 {
        b_var3 = mem_op_1000_01b0(ctx, pstruct7_param_2);
        if ctx.AH_REG == 0 && b_var3 == false { return null_mut(); }
    }
    pu8_var1 = (pstruct7_param_2.field_0x0a);
    (pstruct7_param_2.field_0xa) = (pu8_var1 + 0x4);
    pu8_var4 = pstruct7_param_2[param_1].field_0x02;
    if *pu8_var4 == 0x0 {
        *(pu8_var1 + 0x6) = pu8_var1;
        *(pu8_var1 + 0x4) = pu8_var1;
    } else {
        pu8_var2 = pu8_var4;
        *(pu8_var1 + 0x6) = pu8_var2;
        *(pu8_var1 + 0x4) = (pu8_var2 + 0x4);
        ((pu8_var2 + 0x4) + 0x6) = pu8_var1;
        (pu8_var2 + 0x4) = pu8_var1;
    }
    pu8_var4 = pu8_var1;
    return pu8_var1;
}

pub unsafe fn pass1_1000_0368(mut param_1: u16, mut param_2: u8, mut param_3: u16) {
    let mut pu16_var1 = 0u16;

    if (param_1 + 0x4) == param_1 {
        (param_3 + param_2 * 0x2) = 0;
    } else {
        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        pu16_var1 = (param_2 * 0x2) as u16 + param_3;
        if pu16_var1 == param_1 {
            pu16_var1 = (param_1 + 0x4);
        }
    }
    (param_1 + 0x4) = (param_3 + 0xa);
    (param_3 + 0xa) = param_1;
}

// TODO: this looks like its referring to a struct
pub unsafe fn pass1_1000_05b4(param_1: u8, param_2: *mut Struct100005b41) {
    // *(param_2 + 0xa)
    param_2.field_0xa = 0x1;
    // *(param_2 + 0x8)
    param_2.field_0x8    = 0x668;
    // *(param_2 + 0x13)
    param_2.field_0x13    = (-(param_1 & 0x2) != 0) & 0x2;
    // *(param_2 + 0x10)
    param_2.field_0x10    = 0;
    // *(param_2 + 0xe)
    param_2.field_0xe    = 0;
}



pub unsafe fn pass1_1000_07ac(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: *mut Struct_1000_07ac_1,
) {
    let mut u16_var1 = (param_3.field_0x10.clone());
    (param_3.field_0x0e) = u16_var1;
    let mut u_var3 = param_2 + (param_3.field_0x0.clone() - u16_var1.clone());
    let mut i_var2 = u16_var1c.lone() + (u_var3 - u_var3 % param_1);
    (param_3.field_0x10) = i_var2;
    while u16_var1 < (i_var2 - param_1.clone()) {
        u16_var1 = (u16_var1.clone() + param_1.clone());
        u16_var1 = (u16_var1 + param_1.clone());
    }
    u16_var1 = 0;
    return;
}

pub unsafe fn mem_op_1000_0838(ctx: &mut AppContext, mut pstruct7_param1: *mut Struct7) -> u32 {
    // let mut pu8_var1: *mut u8;
    let mut pu8_var2: *mut u8;
    let mut u16_var3: u16;
    let mut pu8_var4: *mut u8;
    let mut u16_var5: u16;
    let mut u16_var6: u16;
    let mut u16_var7: u16;
    let mut u16_var8: u16;
    let mut pu16_var9: *mut u16;
    let mut b_var10: bool;
    let mut u16_var11: u16;
    let mut pu16_var12: *mut u16;

    pu16_var9 = &mut pstruct7_param1.field_0x02;
    pu16_var12 = pu16_var9;
    if (pstruct7_param1.field_0x02) == 0x0 {
        // TODO:
        // goto LAB_1000_085b;
    }
    loop {
        loop {
            if *pu16_var9 != 0x0 {
                u16_var3 = pu16_var9[0x5];
                pu8_var4 = PTR_LOOP_1050_000e;
                if !pu8_var4.is_null() {
                    PTR_LOOP_1050_000e = pu8_var4;
                    pu8_var2 = PTR_LOOP_1050_000a;
                    *pu8_var2 = (*pu8_var2).clone() + 1;
                    pstruct7_param1.field_0x02 = (*pu16_var9).clone();
                    return CONCAT22(u16_var3, (*pu8_var4).clone() as u16);
                }
                *pu16_var9 = 0;
            }
            pu16_var9 = pu16_var9[0x2];
            if !(pu16_var9 != pu16_var12) {
                break;
            }
        } //while (piVar9 != piStack4);
        //        LAB_1000_085b:
        if (pstruct7_param1.field_0x18) == 0x0 {
            // 0x1050
            pass1_1000_1e61(ctx.CS_REG.clone(), 0x4, pstruct7_param1);
            return 0x0;
        }
        u16_var5 = (pstruct7_param1.field_0x1a.clone());
        loop {
            u16_var11 = u16_var5;
            u16_var5 = 0x1;
            u16_var8 = mem_op_1000_03c6(ctx, u16_var11, 0x0, 0x1, pstruct7_param1, 0x0, 0);
            if (u16_var8 | u16_var5) != 0x0 {
                break;
            }
            u16_var5 = (pstruct7_param1.segment_field_0x1e.clone()) as u16;
            u16_var6 = u16_var5 + u16_var11.clone();
            u16_var5 = (pstruct7_param1.offset_field_0x20.clone()) + CARRY2(u16_var5.clone(), u16_var11.clone());
            let mut u16_var1 = (pstruct7_param1.addr_part_field_0x28.clone());
            b_var10 = u16_var1 <= u16_var5;
            if b_var10 {
                if b_var10 && (u16_var5 != (u16_var1).clone() as u16) {
                    return 0x0;
                }
                u16_var1 = (pstruct7_param1.addr_part_field_0x26.clone());
                if (u16_var1 <= u16_var6) && (u16_var6 != (u16_var1).clone() as u16) {
                    return 0x0;
                }
            }
            u16_var5 = u16_var11.clone() >> 0x1;
            if (u16_var11.clone() >> 1) < (pstruct7_param1.field_0x18) + 0x14 {
                // 0x1050
                u16_var7 = pass1_1000_1e61(ctx.CS_REG.clone(), 0x2, pstruct7_param1);
                u16_var5 = u16_var11.clone() & 0xfffe;
                if u16_var7 == 0x0 {
                    return 0x0;
                }
            }
        }
        *pu16_var9 = pstruct7_param1.field_0x02.clone();
        pu16_var12 = pu16_var9[0x2];
    }
}

pub unsafe fn mem_op_1000_03c6(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut struct7_param_4: *mut Struct7,
    param_5: u8,
    mut param_6: u16,
) -> u16 {
    let mut pi16_var2: *mut i16;
    let mut pu16_var5: *mut u16;
    let mut u16_var6: u16;
    let mut u32_var9: u32;
    let mut u16_var10: u16;

    let mut u16_var7 = CONCAT11(param_6 as u8, param_5);
    let mut u16_var3 = param_1 + 0xfff & 0xf000;
    let mut u16_var1 = (struct7_param_4.segment_field_0x1e.clone());
    let mut u_var4 = u16_var3 + (*(u16_var1 as *mut u16)).clone();
    u16_var3 = param_2 + (0xf000 < param_1) + struct7_param_4.offset_field_0x20.clone() + CARRY2(u16_var3, (u16_var1).clone() as u16);
    u16_var1 = struct7_param_4.addr_part_field_0x28.clone();
    let mut b_var8 = u16_var3 < u16_var1;
    u16_var1 = struct7_param_4.addr_part_field_0x26.clone();
    if b_var8 || (b_var8.clone() || u16_var3 == u16_var1 && (
        u_var4 < u16_var1 || u_var4 == u16_var1
    )) {
        let mut pstruct7_var20: *mut Struct7 = null_mut();
        if param_3 == 0x3 {
            pstruct7_var20 = ((-((param_5.clone() & 1) != 0) >> 0x8) & 0x1 | 0x20) << 0x8;
        } else {
            pstruct7_var20.addr_offset_field_0x0 = 0x1000;
        }
        pstruct7_var20 = struct7_param_4.field_0x16.clone() | pstruct7_var20;
        alloc_mem_1000_131c(pstruct7_var20.addr_offset_field_0x0.clone(),
                            CONCAT22(param_2.clone() as u16,
                                  param_1.clone()));
        if (u16_var3 | pstruct7_var20.clone()) != 0 {
            pu16_var5 = mem_op_1000_0308(ctx, param_3 as i16, struct7_param_4) as *mut u16;
            if pu16_var5.is_null() == false {
                pu16_var5[0x4] = pstruct7_var20.clone();
                pu16_var5[0x5] = u16_var3;
                PTR_LOOP_1050_000c = param_3.clone() | 0xcad0;
                u16_1050_0002 = 0x1050;
                u32_1050_0004 = pu_var5;
                u32_1050_0006 = 0x1050;
                PTR_LOOP_1050_000a = null_mut();
                u16_var10 = 0x1050;
                // TODO: dvar9 = mem_op_1000_1532(u_stack20, u_var3);
                u16_var6 = u32_var9.clone() as u16;
                if param_3 == 1 {
                    // 0x1050
                    u16_var7 = pass1_1000_0782(ctx, struct7_param_4, u16_var6, null_mut(), 0x1050);
                } else if param_3 == 0x3 {
                    pass1_1000_05b4(param_5.clone(), null_mut());
                } else {
                    u16_var7 = pass1_1000_09ca(u16_var6 as i16, null_mut());
                }
                param_2 = (u32_var9.clone() >> 0x10) as u16;
                *pu16_var5 = u16_var7;
                pu16_var5[0x1] = 0x8000;
                u16_var1 = (struct7_param_4.segment_field_0x1e.clone());
                u_var4 = (u16_var1).clone();
                u16_var1 = (u16_var1).clone() + u16_var6.clone();
                let mut u16_var2 = (struct7_param_4.offset_field_0x20.clone());
                u16_var2 = (u16_var2).clone() + param_2 + CARRY2(u_var4, u16_var6.clone());
                return u16_var3;
            }
            free_mem_1000_13ce(pstruct7_var20.clone(), u16_var3);
        }
    } else {
        // 0x1050
        pass1_1000_1e61(ctx.CS_REG.clone(), 0x7, struct7_param_4);
    }
    return 0x0;
}

pub unsafe fn mem_op_1000_0510(mut param_1: u16, pstruct7_param_2: *mut Struct7) -> u32 {
    let mut u16_var10: u16;
    let mut b_var11: bool;
    let mut i32_var13 = 0i32;
    let mut pstruct7_var4 = pstruct7_param_2;
    let mut u16_var6 = (pstruct7_param_2.field_0x4);
    let mut u8_var3 = (pstruct7_param_2.field_0xc.clone());
    let mut u32_var12 = mem_op_1000_1532(pstruct7_param_2, 0x1050);
    let mut u16_var9 = (u32_var12 >> 0x10);
    let mut u32_var8 = u32_var12.clone();

    let mut u16_var1 = 0u16;
    let mut u16_var14 = 0x1050;
    let mut u16_var7 = 0u16;
    if param_1 != 0 {
        u16_var7 = (pstruct7_var4.segment_field_0x1e.clone());
        u16_var10 = ((pstruct7_var4.offset_field_0x20.clone()) - u16_var9) - (u16_var7 < u32_var8 as  u16);
        u16_var1 = (pstruct7_var4.addr_part_field_0x24.clone());
        b_var11 = u16_var10 < u16_var1;
        if (b_var11 || u16_var10 == u16_var1) && (b_var11.clone() || (u16_var7 - u32_var8.clone() < (pstruct7_var4 + 0x22))) {
            b_var11 = false;
            u16_var9 = u16_var10;
            // TODO
            //goto LAB_1000_0595;
            return lab_1000_0595(pstruct7_param_2, &mut b_var11, i32_var13, &mut u16_var9, u16_var14);
        }
    }
    pass1_1000_0368(u16_var6, u8_var3 & 0x7, null_mut());
    // TODO: pu16_var1 = 0x001e;
    u16_var7 = u16_var1;
    u16_var1 = (u16_var1).clone() - u32_var8.clone();
    // s_New_failed_in_Op__Op_1050_0020;
    let mut u16_var2 = 0x0020;
    u16_var2 = ((u16_var2).clone() - u16_var9) - (u16_var7 < u32_var8.clone() as u16);
    b_var11 = true; //
    //    LAB_1000_0595:
    return lab_1000_0595(pstruct7_param_2, &mut b_var11, i32_var13, &mut u16_var9, u16_var14);
}

unsafe fn lab_1000_0595(pstruct7_param_2: *mut Struct7, b_var11: &mut bool, mut i32_var13: i32, u16_var9: &mut u32, mut u16_var14: u16) -> u32{
    if b_var11 {
        pstruct7_param_2.field_0xc = 0;
        i32_var13 = free_mem_1000_13ce(pstruct7_param_2, u16_var14);
        return CONCAT22((i32_var13 >> 0x10) as u16, 1);
    }
    return u16_var9 << 0x10;
}

pub unsafe fn mem_op_1000_05e2(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    pstruct7_param_4: *mut Struct7,
) -> u32 {
    let mut u16_var1 = 0u16;
    let mut u16_var3: u16;
    let mut u16_var4: u16;
    let mut u16_var5: u16;
    let mut u32_var6: u32;
    let mut b_var5: bool;
    let mut u32_var7: u32;

    let mut u16_var2 = param_2 + (0xffeb < param_1);
    loop {
        u16_var3 = 0x3;
        u32_var6 = param_3.clone() as u32;
        u32_var6 = mem_op_1000_03c6(ctx, param_1.clone() + 0x14, u16_var2.clone(), 0x3, pstruct7_param_4, u32_var6 as u8, u32_var6.clone() as u16) as u32;
        if (u32_var6 | u16_var3) != 0 {
            return CONCAT22(u32_var6.clone() as u16, u16_var3.clone() + 0x14);
        }
        u32_var7 = mem_op_1000_0052(pstruct7_param_4);
        u16_var3 = param_1.clone() + 0x1013 & 0xf000;
        u16_var1 = pstruct7_param_4.segment_field_0x1e.clone();
        u16_var4 = u16_var3 + (u16_var1).clone();
        u16_var3 = u16_var2.clone() + (0xf000 < param_1.clone() + 0x14) + (pstruct7_param_4 + 0x20)
            + CARRY2(u16_var3.clone(), (u16_var1).clone());
        u16_var1 = (pstruct7_param_4 + 0x28);
        b_var5 = u16_var3 < u16_var1;
        // 0x1050)
         u16_var1 = pstruct7_param_4.addr_part_field_0x26.clone();
        u16_var5 = pass1_1000_1e61(ctx.CS_REG.clone(), 0x2, pstruct7_param_4);
        if (((b_var5 || u16_var3 == u16_var1) && (b_var5.clone() || (
            u16_var4 < u16_var1 || u16_var4 == u16_var1
        ))) && (u32_var7 != 0x0 || (
            u16_var5 != 0
        ))) == false {
            break;
        }
    }
    return 0x0;
}

pub unsafe fn mem_1000_0670(
    ctx: &mut AppContext,
    mut param_1: u16,
    pstruct7_param_2: *mut Struct7,
    mut param_3: i16,
    mut param_4: u16,
    param_5: *mut i16,
) -> u16 {
    let mut i16_var5: i16;
    let mut u16_var10: u16;
    let mut b_var11: bool;
    let mut u16_var12: u16;
    let mut u16_var13: u16;
    let mut u16_var16: u16;

    let mut pstruct7_var3 = pstruct7_param_2;
    let mut u16_var4 = (pstruct7_param_2.field_0x2);
    // 0x1050
    let mut u32_var14 = mem_op_1000_1532(pstruct7_param_2, 0x1050);
    let mut u16_var6: u16 = param_3 + (0xffeb < param_1);
    let mut u16_var7 = pstruct7_param_2.addr_offset_field_0x0.clone();
    let mut u16_var8 = -((param_4 & 1) != 0) & 0x100 | -((param_4.clone() & 0x4) != 0) & 0x400 | (u16_var7 + 0x16);
    if param_5.is_null() {
        //  0x1050
        b_var11 = mem_op_1000_14f2(u16_var8 | 0x2000, param_1 + 0x14, pstruct7_param_2, 0x1050);
        if b_var11 == 0 {
            return 0x0;
        }
        // 0x1050
        u16_var16 = 0x1050;
    } else {
        i16_var5 = pstruct7_param_2.field_0x01.clone() as i16;
        u16_var12 = pstruct7_param_2.field_0x06.clone();
        u16_var13 = u16_var12;
        let mut pu8_var9: *mut u8 = null_mut();
        loop {
            u16_var16 = u16_var13;
            pu8_var9 = (u16_var8.clone() | 0x2000);
            // 0x1050
            realloc_1000_1408(
                pu8_var9,
                CONCAT22(u16_var6.clone(), param_1.clone() + 0x14),
                pstruct7_param_2,
                0x1050,
            );
            u16_var13 = u16_var16 | pu8_var9;
            if u16_var13 != 0 {
                break;
            }
            u16_var10 = pass1_1000_1e61(ctx.CS_REG.clone(), 0x2, pstruct7_var3);
            if (u16_var10 != 0) == false {
                break;
            }
        }
        if (u16_var16.clone() | pu8_var9) == 0 {
            (param_5 + 0x2) = 0;
            *param_5 = 0;
            return 0x0;
        }
        (i16_var5 + 0x8) = pu8_var9;
        (i16_var5.clone() + 0xa) = u16_var16.clone();
        *param_5 = (pu8_var9 + 0x5);
        (param_5 + 0x2) = u16_var16.clone();
        // TODO: pstruct7_param_2.field_0x0 = pu8_var9;
    }
    let mut u32_var15 = mem_op_1000_1532(pstruct7_param_2, u16_var16.clone());
    let mut u16_var12 = (u32_var15 - u32_var14);
    let mut u16_var1 = pstruct7_var3.segment_field_0x1e.clone();
    u16_var8 = u16_var1.clone();
    u16_var1 = u16_var1.clone() + u16_var12;
    let mut pi16_var2 = pstruct7_var3.offset_field_0x20.clone();
    pi16_var2 = (pi16_var2).clone() + (u32_var15.clone() - u32_var14.clone() >> 0x10) + CARRY2(u16_var8, u16_var12);
    return 0x1;
}

pub unsafe fn pass1_1000_0782(
    ctx: &mut AppContext,
    mut param_1: *mut Struct7,
    mut param_2: u16,
    mut param_3: *mut Struct_1000_07ac_1,
    mut param_4: u16,
) -> u16 {
    (param_3.field_0x0e) = 0;
    (param_3.field_0x10) = param_3.field_0x14.clone();
    (param_3.field_0x08) = 0x9a0;
    pass1_1000_07ac(ctx, (param_1.field_0x18), param_2, param_3);
    return 0x1;
}

pub unsafe fn pass1_1000_07fc(ctx: &mut AppContext, mut param_1: u32) -> MemContainer {
    // let mut paVar1: *mut astruct_99;
    let mut container = MemContainer::default();
    if (param_1 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx.CS_REG.clone(), 0xa, null_mut());
        return container;
    }
    let mut u32_var1 = mem_op_1000_0838(ctx, null_mut());
    container.dword = u32_var1;
    return container;
}

pub unsafe fn pass1_1000_093a(ctx: &mut AppContext, param_1: *mut Struct611) -> bool {
    let mut piVar1: *mut u8;

    if PTR_LOOP_1050_000c != -0x352f {
        pass1_1000_1e61(ctx.CS_REG.clone(), 0xe, null_mut());
        return false;
    }
    param_1.field_0x0 = PTR_LOOP_1050_000e;
    if param_1.field_0x0 == null_mut() {
        u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000e = param_1.field_0x0;
    piVar1 = PTR_LOOP_1050_000a;
    *piVar1 = (*piVar1).clone() - 1;
    if *piVar1 == 0 {
        mem_op_1000_0510(0x1, null_mut());
    }
    return true;
}

pub unsafe fn pass1_1000_09ca(mut param_1: i16, param_2: *mut u16) -> u16 {
    let mut puVar1 = param_2 + 0xa;
    let mut puVar4 = ((param_2 + (param_1 - puVar1) -0x6 & 0xfffc) + puVar1);
    *puVar4 = 0x1;
    param_2[0x7] = puVar1;
    puVar4[0x2] = puVar4;
    puVar4[0x1] = puVar4;
    param_2[0x8] = puVar4;
    if (*(param_2 + 0x6) & 0x7) == 0x2 {
        param_2[0x9] = 0x8;
    } else {
        let uVar3 = param_2;
        let iVar2 = (uVar3 + 0x18);
        param_2[0x9] = (iVar2 - 0x5 & !-(iVar2 + 0x3 < 0x8)) + 0x8;
    }
    puVar4[-0x1] = puVar4 - puVar1;
    *puVar1 = puVar4 - puVar1 | 0x2;
    param_2[0xc] = puVar4;
    param_2[0xb] = puVar4[0x1];
    (puVar4[0x1] + 0x4) = puVar1;
    puVar4[0x1] = puVar1;
    param_2[0x4] = 0xe08;
    return *puVar1 & 0xfffc;
}

pub unsafe fn mem_op_1000_0a48(
    ctx: &mut AppContext,
    param_1: u8,
    mut param_2: u16,
    mut param_3: u16,
    pstruct7_param_4: *mut Struct7,
) -> u32 {

    let mut u16_var3: u16;
    let mut u32_var5 = 0u32;
    if (pstruct7_param_4.field_0x14) == -0x4153 {
        // (s_version__d__d_1050_0012 + 0x6)
        if (param_3 == 0) && (param_2 <= 0x0018) {
            if param_2 == 0 {
                pass1_1000_1e61(ctx.CS_REG.clone(), 0x4, pstruct7_param_4);
                u32_var5 = 0;
            } else {
                let mut pu8_var2: *mut u8 = null_mut();
                let mut pu8_var1: *mut u8 = null_mut();
                u32_var5 = mem_op_1000_0838(ctx, null_mut());
                u16_var3 = (u32_var5 >> 0x10);
                // TODO: set pointer address
                // puVar2 = uVar5.clone();
                if (u32_var5 != 0) && ((param_1 & 1) != 0) {
                    // (s_version__d__d_1050_0012 + 0x6)
                    let mut u16_var1 = 0x0018u16;
                    // for (uVar4 = uVar1 >> 0x1; uVar4 != 0; uVar4 -= 1)
                    for uVar4 in u16_var1 >> 0x1..0 {
                        pu8_var1 = pu8_var2;
                        pu8_var2 = pu8_var2 + 1;
                        *pu8_var1 = 0;
                    }
                    if (u16_var1.clone() & 1) != 0 {
                        *pu8_var2 = 0;
                    }
                }
            }
        }
        // (s_version__d__d_1050_0012 + 0xa))
        else if (param_3 == 0) && (param_2 <= 0x001c) {
            u32_var5 = mem_op_1000_0b20(ctx, param_1 & 0xfffd, null_mut(), param_2);
        } else {
            u32_var5 = mem_op_1000_05e2(ctx, param_2, param_3, param_1 & 0xfffd, null_mut());
        }
        return u32_var5.clone();
    }
    pass1_1000_1e61(ctx.CS_REG.clone(), 0xa, null_mut());
    return 0x0;
}

pub unsafe fn mem_op_1000_0b20(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut pstruct7_param_2: *mut Struct7,
    mut param_3: u16,
) -> u32 {
    // let mut pu16_var1: *mut u16;
    let mut u16_var3: u16;
    let mut u16_var8: u16;
    let mut b_var9: bool;
    let mut u32_var10: u32;

    let mut u16_var8 = 0x1050u16;
    let mut u16_var2 = param_1 & 0x2;
    let mut u16_var4 = param_3 + 0x5 & 0xfffc;
    u16_var4 = u16_var4 - 0x8 & !-(u16_var4 < 0x8);
    let mut u16_var5 = u16_var4 + 0x8;
    let mut pu8_var7 = (u16_var2 * 0x2 + pstruct7_param_2);
    let mut u16_var20 = param_1.clone();
    let mut pu8_var6 = pu8_var7;
    if pu8_var7.is_null() {
        // goto LAB_1000_0b64;
    }
    loop {
        loop {
            u32_var10 = pass1_1000_0c32(u16_var5, u16_var20.clone(), 0x0);
            if (u16_var5 <= *pu8_var7) && (u32_var10 != 0) {
                (u16_var2 * 0x2 + param_2.clone()) = pu8_var7;
                return u32_var10;
            }
            pu8_var7 = pu8_var7[0x2];
            if pu8_var7 == pu8_var6 {
                break;
            }
        }
        //        LAB_1000_0b64:
        u16_var3 = pstruct7_param_2.addr_part_field_0x32.clone();
        if ((((u16_var20.clone() & 0x2) == 0) || ((u16_var20.clone() & 0x40) != 0)) || (pstruct7_param_2.addr_part_field_0x32 == 0)) || (u16_var3 < u16_var5) {
            u16_var3 = u16_var2;
            u16_var8 = mem_op_1000_03c6(ctx, (pstruct7_param_2.field_0x1a.clone()), 0x0, u16_var2, pstruct7_param_2, 0x0, 0);
            if ((u16_var20.clone() & 0x10) != 0) || ((u16_var8 | u16_var3) == 0) {
                if (u16_var20 & 0x20) == 0 {
                    u16_var2 = u16_var4 + 0x1007 & 0xf000;
                    let mut u16_var1 = pstruct7_param_2.segment_field_0x1e.clone();
                    let mut u16_var4 = u16_var2 + u16_var1;
                    u16_var2 = pstruct7_param_2.offset_field_0x20.clone() + CARRY2(u16_var2, u16_var1.clone());
                    u16_var1 = pstruct7_param_2.addr_part_field_0x28.clone();
                    b_var9 = u16_var2 < u16_var1;
                    u16_var1 = pstruct7_param_2.addr_part_field_0x26.clone();
                    if (b_var9 || u16_var2 == u16_var1) && (b_var9.clone() || u16_var4 < u16_var1 || u16_var4 == u16_var1) {
                        u32_var10 = mem_op_1000_05e2(ctx, u16_var5, 0x0, u16_var20.clone(), pstruct7_param_2);
                        return u32_var10;
                    }
                }
                return 0x0;
            }
        } else {
            u16_var20 |= 0x40;
        }
        pu8_var7 = (u16_var2 * 0x2 + pstruct7_param_2);
        pu8_var6 = pu8_var7[0x2];
    }
}

pub unsafe fn pass1_1000_0c32(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> U32 {
    let mut puVar1: *mut u16;
    let mut pbVar2: *mut u8;
    let mut piVar3: *mut i16;
    let mut uVar4: u32;
    let mut puVar6: *mut u16;
    let mut iVar7: i16;
    let mut uVar9: u16;
    let mut uStack14: u16;
    let mut puStack8: *mut u16;

    let mut puVar8 = (param_3 + 0xe);
    let mut uStack6 = 0u16;
    puVar6 = puVar8;
    loop {
        loop {
            let mut uVar5 = *puVar6;
            if param_1 <= uVar5 {
                uVar5 = (uVar5 & 0xfffc) - param_1;
                puVar1 = (param_3 + 0x12);
                puStack8 = puVar6;
                if *puVar1 < uVar5 || *puVar1 == uVar5 {
                    uStack14 = param_1;
                    if (param_2 & 0x6) == 0 {
                        puStack8 = (uVar5 + puVar6);
                        puStack8[-0x1] = uVar5;
                        *puVar6 = uVar5 | 0x2;
                        puVar8 = puVar6[0x1];
                        pbVar2 = (puStack8 + param_1);
                        *pbVar2 = *pbVar2 | 0x2;
                        *puStack8 = param_1 | 0x1;
                    } else {
                        *puVar6 = param_1 & 0xff00 | *puVar6 & 0x2 | param_1 & 0xff | 0x1;
                        (puVar6[0x2] + 0x2) = puVar6[0x1];
                        (puVar6[0x1] + 0x4) = puVar6[0x2];
                        puVar8 = (puVar6 + param_1);
                        (puVar8 + (uVar5 - 0x2)) = uVar5;
                        *puVar8 = uVar5 | 0x2;
                        uVar5 = (param_3 + 0x10);
                        puVar8[0x2] = uVar5;
                        puVar8[0x1] = (uVar5 + 2);
                        ((uVar5 + 0x2) + 0x4) = puVar8;
                        (uVar5 + 0x2) = puVar8;
                    }
                } else {
                    puVar8 = puVar6[0x1];
                    (puVar6[0x2] + 0x2) = puVar8;
                    (puVar6[0x1] + 0x4) = puVar6[0x2];
                    puVar1 = puVar6;
                    *puVar1 = *puVar1 | 0x1;
                    uStack14 = *puVar6 & 0xfffc;
                    (puVar6 + uStack14) = (puVar6 + uStack14) | 0x2;
                }
                (param_3 + 0xe) = puVar8;
                if (param_2 & 1) != 0 {
                    puVar6 = puStack8;
                    uVar5 = uStack14 - 0x2 >> 1;
                    puVar6 = puVar6 + 1;
                    while uVar5 != 0 {
                        *puVar6 = 0;
                        uVar5 -= 1;
                    }
                    if (uStack14 - 0x2 & 1) != 0 {
                        *puVar6 = 0;
                    }
                }
                if ((param_2 & 0x2) != 0) && (puVar8[0x1] == puVar8[0x2]) {
                    (param_3 + 0x4) = ((param_3 + 0x10) + 0x2) & 0xfffc;
                    uVar4 = (param_3 + 0x4);
                    pbVar2 = (uVar4 + 0x3);
                    *pbVar2 = *pbVar2 | 0x80;
                }
                piVar3 = (param_3 + 0xa);
                *piVar3 = *piVar3 + 1;
                return CONCAT22(0x1050, puStack8 + 1);
            }
            if uStack6 < uVar5 {
                uStack6 = uVar5;
            }
            puVar6 = puVar6[0x1];
            if puVar6 == puVar8 {
                break;
            }
        }
        if ((param_2 & 0x2) == 0) || ((param_2 & 0x40) != 0) {
            break;
        }
        uVar4 = param_3 as u32;
        uVar9 = (uVar4 >> 0x10);
        iVar7 = uVar4 as i16;
        if (iVar7 + 0x34) == 0 {
            break;
        }
        uStack6 = (iVar7 + 0x34)();
        puVar6 = (param_3.clone() + 0xe);
        if (uStack6 < param_1) ||  puVar6.is_null() {
            break;
        }
    }
    (param_3.clone() + 0x4) = uStack6 & 0xfffc;
    return 0x0;
}

pub unsafe fn call_fn_ptr_1000_0dc6(ctx: &mut AppContext, str_param_1: *mut c_char) -> bool {
    if (PTR_LOOP_1050_000c.clone() & 0xfff8) != 0xcad0 {
        pass1_1000_1e61(ctx.CS_REG.clone(), 0xe, ptr::null_mut());
        return false;
    }
    // 0x1050
    // u16_1050_0008.unwrap()(0x1050);
    return true;
}

pub unsafe fn pass1_1000_0e08(mut param_1: i16) -> u16 {
    let mut puVar1: *mut u16;
    let mut pbVar2: *mut u8;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u16;
    let mut bVar6: bool;
    let mut uVar7: u32;

    puVar5 = (param_1 -0x2);
    bVar6 = (*puVar5 & 0x2) != 0;
    if (bVar6) {
        puVar1 = puVar5;
        *puVar1 = *puVar1 & 0xfe;
    } else {
        puVar4 = (puVar5 - (param_1 -0x4));
        puVar1 = puVar4;
        *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
        puVar5 = puVar4;
    }
    puVar4 = ((*puVar5 & 0xfffc) + puVar5);
    if ((*puVar4 & 1) == 0) {
        puVar1 = puVar5;
        *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
        if (puVar4 == PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e = puVar5;
        }
        (puVar4[0x2] + 0x2) = puVar4[0x1];
        (puVar4[0x1] + 0x4) = puVar4[0x2];
        puVar4 = ((*puVar5 & 0xfffc) + puVar5);
    }
    puVar4[-0x1] = *puVar5 & 0xfffc;
    uVar3 = u32_1050_0004;
    puVar1 = puVar4 -0x1;
    if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
        uVar3 = *puVar5 & 0xfffc;
        u32_1050_0004 = uVar3;
    }
    puVar1 = puVar4;
    *puVar1 = *puVar1 & 0xfd;
    if (bVar6) {
        if (PTR_LOOP_1050_0010.address_offset_field_0x2 != PTR_LOOP_1050_0010) {
            pbVar2 = (u32_1050_0004 + 0x3);
            *pbVar2 = *pbVar2 & 0x7f;
        }
        puVar5[0x2] = PTR_LOOP_1050_0010;
        uVar3 = PTR_LOOP_1050_0010.address_offset_field_0x2;
        puVar5[0x1] = uVar3;
        (PTR_LOOP_1050_0010.address_offset_field_0x2 + 0x4) = puVar5;
        PTR_LOOP_1050_0010.address_offset_field_0x2 = puVar5;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a -0x1;
    if (PTR_LOOP_1050_000a.is_null()) {
        uVar7 = mem_op_1000_0510(0x1, 0x0);
        uVar3 = uVar7;
    }
    return uVar3;
}

pub unsafe fn pass1_1000_0ed4(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut astruct_172,
    param_5: *mut astruct_172,
) -> i32 {
    let mut paVar1: *mut astruct_172;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    // let mut ppaVar4: *mut *mut struct_172;
    let mut ppaVar4: *mut *mut astruct_172;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut paVar7: *mut astruct_172;
    let mut puVar8: *mut u16;
    // let mut unaff_CS: u16;
    let mut lVar9: i32;
    let mut UVar10: u16;
    let mut UVar11: u16;
    let mut UVar12: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
        UVar11 = *NULL;
        UVar12 = &u16_1050_0002;
        if ((param_1 & 0x8) == 0) {
            ppaVar4 = &param_4;
            //  0x1050;
            uVar6 = 0x1050;
        } else {
            ppaVar4 = null_mut();
            uVar6 = 0;
        }
        uVar6 = pass1_1000_0fb8(param_2, param_4, param_3, param_1, ppaVar4, uVar6);
        if (uVar6 == 0) {
            return CONCAT22(param_5, param_4);
        }
        if ((param_1 & 0x8) == 0) {
            lVar9 = mem_op_1000_0a48(ctx, param_1, param_2, param_3, CONCAT22(UVar12, UVar11));
            uVar3 = (lVar9 >> 0x10);
            puVar8 = lVar9;
            if (lVar9 != 0) {
                paVar7 = param_4;
                // for (uVar5 = uVar6 >> 0x1; uVar5 != 0; uVar5 -= 1)
                for uVar5 in uVar6 >> 0x1..0 {
                    puVar2 = puVar8;
                    puVar8 = puVar8 + 1;
                    paVar1 = paVar7;
                    paVar7 = &paVar7.field1_0x2;
                    *puVar2 = paVar1.field0_0x0;
                }
                // for (uVar6 =  ((uVar6 & 1) != 0); uVar6 != 0; uVar6 -= 1)
                for uVar6 in uVar6 & 0x1..0 {
                    puVar2 = puVar8;
                    puVar8 = (puVar8 + 1);
                    paVar1 = paVar7;
                    paVar7 = (&paVar7.field0_0x0 + 1);
                    *puVar2 = *&paVar1.field0_0x0;
                }
                call_fn_ptr_1000_0dc6(ctx, CONCAT22(param_5, param_4));
            }
            return lVar9;
        }
        if ((param_3 | param_2) == 0) {
            return 0x0;
        }
        UVar10 = 0x5;
    } else {
        UVar11 = 0;
        UVar12 = 0;
        UVar10 = 0xe;
    }
    pass1_1000_1e61(unaff_CS, UVar10, UVar11);
    return 0x0;
}

pub unsafe fn pass1_1000_0fb8(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut u16,
    mut param_6: u16,
) -> u16 {
    let mut puVar1: *mut u16;
    let mut bVar2: u8;
    let mut uVar3: u16;
    let mut BVar4: bool;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u16;
    let mut unaff_CS: u16;
    let mut uVar9: u32;
    let mut uStack4: u16;

    if ((param_3 | param_1) == 0) {
        pass1_1000_1e61(unaff_CS, 0x4, PTR_LOOP_1050_0000);
        if ((param_6 | param_5) != 0) {
            param_5[0x1] = 0;
            *param_5 = 0;
            return 0x0;
        }
        return 0x1;
    }
    bVar2 = PTR_LOOP_1050_000c & 0x7;
    if ((PTR_LOOP_1050_000c & 0x7) != 0) {
        if (bVar2 == 1) {
            uVar3 = (PTR_LOOP_1050_0000 + 0x18);
            if (param_3 != 0) {
                return uVar3;
            }
            if (param_1 <= uVar3) {
                return 0x0;
            }
            return uVar3;
        }
        if (bVar2 != 0x2) {
            if (bVar2 != 0x3) {
                if ((param_6 | param_5) != 0) {
                    param_5[0x1] = 0;
                    *param_5 = 0;
                    return 0x0;
                }
                return 0x1;
            }
            if ((((param_6 | param_5) != 0) && (param_3 == 0)) && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c))) {
                uVar9 = pass1_1000_1284(CONCAT22(0x1050, param_2));
                if (CONCAT22(param_3, param_1) < uVar9) {
                    return param_1;
                }
                return uVar9;
            }
            iVar5 = mem_1000_0670(ctx, param_1, NULL, param_3, param_4, CONCAT22(param_6, param_5));
            if (iVar5 != 0) {
                return 0x0;
            }
            if ((param_6 | param_5) != 0) {
                return 0x0;
            }
            return 0x1;
        }
    }
    puVar8 = (param_2 -0x2);
    uVar3 = *puVar8 & 0x7ffc;
    uStack4 = uVar3 - 0x2;
    if ((*(param_2 -1) & 0x80) != 0) {
        uStack4 = uVar3 - 0x6;
    }
    if ((((param_3 == 0) && (param_1 <= uStack4)) || (param_3 == 0x0 && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c)))) && (BVar4 = pass1_1000_115c(param_1, puVar8), BVar4 != 0)) {
        if ((param_4 & 1) != 0) {
            uVar3 = (*puVar8 & 0x7ffc) - 0x2;
            if (uStack4 < param_1) {
                puVar7 = (uStack4 + param_2);
                iVar5 = -uStack4;
            } else {
                if (uVar3 <= param_1) {
                    return 0x0;
                }
                puVar7 = (param_1 + param_2);
                iVar5 = -param_1;
            }
            uVar3 += iVar5;
            // for (uVar6 = uVar3 >> 0x1; uVar6 != 0; uVar6 -= 1)
            for uVar6 in uVar3 >> 1..0 {
                puVar1 = puVar7;
                puVar7 = puVar7 + 1;
                *puVar1 = 0;
            }
            if ((uVar3 & 1) != 0) {
                *puVar7 = 0;
            }
        }
        return 0x0;
    }
    return uStack4;
}

pub unsafe fn pass1_1000_115c(mut param_1: i16, param_2: *mut u16) -> bool {
    let mut pbVar1: *mut u8;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut UVar6: u16;
    let mut uStack4: u16;

    uVar3 = *param_2 & 0x7ffc;
    uVar4 = param_1 + 0x5 & 0xfffc;
    // TODO
    // uVar4 = (uVar4 - 0x8 & ~- (uVar4 < 0x8)) + 0x8;
    if (uVar3 < uVar4) {
        puVar5 = (uVar3 + param_2);
        if (((*puVar5 & 1) != 0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
            return 0x0;
        }
        if (puVar5 == PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e = puVar5[0x1];
        }
        (puVar5[0x2] + 0x2) = puVar5[0x1];
        (puVar5[0x1] + 0x4) = puVar5[0x2];
        uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
        if (uStack4 < s_version__d__d_1050_0012) {
            puVar2 = param_2;
            *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
            pbVar1 = (puVar5 + (*puVar5 & 0xfffc));
            *pbVar1 = *pbVar1 | 0x2;
            return 0x1;
        }
    } else {
        uStack4 = uVar3 - uVar4;
        if (uStack4 < s_version__d__d_1050_0012) {
            return 0x1;
        }
        puVar5 = (uVar3 + param_2);
        if ((*puVar5 & 1) == 0) {
            uStack4 += *puVar5 & 0xfffc;
            if (puVar5 == PTR_LOOP_1050_000e) {
                PTR_LOOP_1050_000e = puVar5[0x1];
            }
            (puVar5[0x2] + 0x2) = puVar5[0x1];
            (puVar5[0x1] + 0x4) = puVar5[0x2];
        }
        if (u32_1050_0004 < uStack4) {
            u32_1050_0004 = uStack4;
        }
    }
    *param_2 = *param_2 & 0x8003 | uVar4;
    (uVar4 + param_2) = uStack4 | 0x2;
    UVar6 = uVar4 + param_2;
    (UVar6 + 0x4) = PTR_LOOP_1050_0010;
    (UVar6 + 0x2) = PTR_LOOP_1050_0010.address_offset_field_0x2;
    (PTR_LOOP_1050_0010.address_offset_field_0x2 + 0x4) = UVar6;
    PTR_LOOP_1050_0010.address_offset_field_0x2 = UVar6;
    ((UVar6 + uStack4) -0x2) = uStack4;
    pbVar1 = (UVar6 + uStack4);
    *pbVar1 = *pbVar1 & 0xfd;
    return 0x1;
}
