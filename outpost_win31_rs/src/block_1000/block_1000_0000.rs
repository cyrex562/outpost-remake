use std::os::raw::c_char;
use std::ptr;
use std::ptr::null_mut;
use crate::block_1000;
use crate::block_1000::block_1000_1000;
use crate::block_1000::block_1000_1000::{mem_op_1000_14f2, pass1_1000_1e61};
use crate::globals::{DAT_1050_5f30, PTR_LOOP_1050_000a, PTR_LOOP_1050_000c, PTR_LOOP_1050_000e, REG_CS, u16_1050_0008};
use crate::structs::structs_2::{Struct7, Struct_1000_07ac_1};
use crate::structs_2_h::Struct7;
use crate::utils::CONCAT22;

// #include "types.h"
// #include "structs.h"
// #include "globals.h"
// #include "sys_api.h"
// #include "block_1000.h"
// #include "utils.h"
// #include "structs_2.h"
// #include "func_ptrs.h"
// #include "entry.h"
// #include <stdbool.h>
// #include <stdio.h>
pub unsafe fn pass1_1000_0000(mut param_1: *mut u16,
                     mut param_2: *mut u16,
                     mut param_3: u16 )
{
    let mut pu_var1: *mut u16;
    let mut pu_var2: *mut u16;
    let mut u_var3: u16;

    // for (u_var3 = param_3 >> 0x1; u_var3 != 0x0; u_var3 -= 0x1) {
    u_var3 = param_3 >> 0x1;
    while u_var3 != 0 {
        pu_var2 = param_2;
        param_2 = param_2 + 0x1;
        pu_var1 = param_1;
        param_1 = param_1 + 0x1;
        *pu_var2 = *pu_var1;
        u_var3 -= 1;
    }
}

pub unsafe fn mem_1000_0016(mut param_1: u32) -> u32
{
    // let mut unaff_CS: u16;
    let mut u_var1: u32;

    if ( param_1 + 0x14) != -0x4153 {
        block_1000_1000::pass1_1000_1e61(REG_CS,
                                         0xa,
                                         ptr::null_mut());
        return 0xffffffff;
    }
    u_var1 = mem_op_1000_0052(ptr::null_mut());
    return u_var1;
}

pub unsafe fn mem_op_1000_0052(pstruct7_param_1: *mut Struct7) -> u32
{
    let mut pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut i_var3: i16;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut pu8_var14: *mut u8 = ptr::null_mut();
    let mut pu8_var12: *mut u8 = ptr::null_mut();
    let mut pu8_var10: *mut u8 = ptr::null_mut();
    let mut u_stack8: u16 = 0u16;

    u_var2 = (pstruct7_param_1.field_0x1e);
    i_var3 = *(pstruct7_param_1.field_0x20);
    u_stack8 = 0x0;
    loop {
        pu8_var10 = (u_stack8 * 0x2 + pstruct7_param_1);
        if (!pu8_var10.is_null()) && (u_stack8 != 0x3) {
            pu8_var14 = ptr::null_mut();
            loop {
                pu8_var12 = (pu8_var10 + 0x4);
                u_var4 =  (pu8_var10 + 0x8);
                if ( u_var4 + 0xa) == 0x0 {
                    u_var5 = mem_op_1000_0510(0x1,
                                              ptr::null_mut());
                    if u_var5 == 0x0 {
                        // goto LAB_1000_00f9;
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
        ( (pstruct7_param_1.field_0x32))();
    }
    // TODO:
//    LAB_1000_00f9:
    pu_var1 = &mut pstruct7_param_1.field_0x1e;
    let a = (i_var3 - (pstruct7_param_1 + 0x20)) -  (u_var2 < *pu_var1);
    let b = u_var2 - *pu_var1;
    return CONCAT22(a, b           );
}

pub unsafe fn pass1_1000_010c(mut param_1: i16,
                              mut param_2: u16,
                              mut param_3: u16,
                              mut param_4: *mut Struct7,
                              mut param_5: u16) -> u16
{
    let mut u_var1: u16;
    let mut uvar2: u16;
    let mut u_var2: u16;
    // let mut unaff_cs: u16;
    let mut b_var3: bool;
    let mut uvar4: u16;
    let mut u_stack8: u16;
    let mut u_stack6: u16;
    let mut u_stack4: u16;

    u_stack6 = 0x0;
    u_stack8 = 0x0;
    if (param_4 + 0x14) != -0x4153 {
        param_5 = 0x0;
        param_4 = ptr::null_mut();
        uvar4 = 0xa;
        // code_r0x10000128:
        block_1000_1000::pass1_1000_1e61(REG_CS,
                                         uvar4,
                                         param_4);
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
        u_stack4 = 0x0;
    }
    // s_version__d__d_1050_0012 + 0x8
    uvar2 = mem_op_1000_03c6(0x001a,
                             0x0,
                             u_stack4,
                             ptr::null_mut(),
                             0x0,
                             0);
    u_var1 = u_stack4;
    while u_stack6 <= param_3 && ((u_stack6 < param_3 || (u_stack8 < param_2)) && (
        (uvar2 | u_var1) != 0x0)) {
        u_var1 = ( 0x00a1a);
        b_var3 = CARRY2(u_stack8,
                        u_var1);
        u_stack8 += u_var1;
        u_stack6 += b_var3;
    }
    return u_stack6;
}

pub unsafe fn mem_op_1000_01b0(param_1: *mut Struct7) -> bool
{
    let mut pu_var1: *mut u16;
    let mut pi_var2: *mut i16;
    let mut bvar3: bool;
    let mut uvar4: u16;
    let mut u16_var5: u16;
    // let mut unaff_CS: u16;
    let mut dvar6: u32;
    let mut dvar7: u32;
    let mut u_var8: u32;
    let mut u16_var9: u16;
    let mut pu8_var10: *mut u8 = ptr::null_mut();
    let mut u_stack14: u16;
    let mut u_stack12: u16;
    let mut i_stack10: i16;
    let mut u_stack6: u16;
    let mut i_stack4: i16;

    u_stack14 = 0x0;
    if ((param_1.field_0x40) | (param_1.field_0x3e)) == 0x0 {
        u16_var5 = param_1.field_0x36;
        dvar6 = mem_op_1000_1532(param_1,
                                 0x1050);
        dvar7 = dvar6;
    } else {
        dvar6 = mem_op_1000_1532(param_1,
                                 0x1050);
        u16_var5 = dvar6 as u16;
        if ( (dvar6 >> 0x10) != 0x0) || (0xffef < u16_var5) {
            block_1000_1000::pass1_1000_1e61(REG_CS,
                                             0x8,
                                             param_1);
            return false;
        }
        if 0x1fff < u16_var5 {
            u16_var5 = 0x2000;
        }
        loop {
            u16_var9 = u16_var5;
            dvar7 = dvar6 + 0x18;
            if ( (dvar7 >> 0x10) != 0x0) || (0xfff0 < dvar7) {
                dvar7 = 0xfff0;
            }
            bvar3 = mem_op_1000_14f2((param_1.field_0x16) | 0x1000,
                                     dvar7,
                                     ptr::null_mut(),
                                     0);
            i_stack4 =  (dvar6 >> 0x10) as i16;
            u_stack6 = dvar6 as u16;
            if bvar3 != false {
                break;
            }
            u16_var5 = u16_var9 >> 0x1;
            if u16_var5 < 0xc {
                uvar4 = block_1000_1000::pass1_1000_1e61(REG_CS,
                                                         0x2,
                                                         param_1);
                if uvar4 == 0x0 {
                    return  ('\x01' - ((*param_1.field_0xa) == 0x0));
                }
                dvar6 = mem_op_1000_1532(param_1, 0x1050);
                u16_var5 = u16_var9 & 0xfffe;
            }
        }
        u_var8 = pass1_1000_5390(u_stack6 - 0x42,
                                 i_stack4 -  (u_stack6 < 0x42),
                                 0xc,
                                 0x0);
        u16_var5 =  u_var8 * 0xc + param_1 + 0x42;
    }
    // pu_var1 =  (param_1 + 0x1e);
    pu_var1 = *param_1.field_0x1e;
    // u16_var9 = *pu_var1;
    u16_var9 = *pu_var1;
    *pu_var1 = *pu_var1 - dvar6;
    // pi_var2 =  (param_1 + 0x20);
    pi_var2 = param_1.field_0x20;
    *pi_var2 = (*pi_var2 -  (dvar6 >> 0x10)) -  (u16_var9 < dvar6 as u16);
    if u16_var5 != 0x0 {
        pu8_var10 = ptr::null_mut();
        u16_var9 = 0xc;
        dvar7 = mem_op_1000_1532(param_1,
                                 0x1050);
        u_var8 = pass1_1000_5390(dvar7 - 0x42,
                                 (dvar7 >> 0x10) -  ( dvar7 < 0x42),
                                 u16_var9,
                                 pu8_var10);
        u_stack14 =  u_var8 * 0xc + param_1 + 0x36;
    }
    i_stack10 =  (dvar7 >> 0x10) as i16;
    u_stack12 = dvar7 as u16;
    pu_var1 = &mut param_1.field_0x1e;
    u16_var9 = *pu_var1;
    *pu_var1 = *pu_var1 + u_stack12;
    pi_var2 = param_1.field_0x20;
    *pi_var2 = *pi_var2 + i_stack10 +  CARRY2(u16_var9,
                                              u_stack12);
    u16_var9 = (param_1.field_0xa);
    loop {
        *pu8_var10 = u16_var5 as u8;
        *(pu8_var10 + 0x4) = u16_var9;
        u16_var9 = *pu8_var10 as u16;
        u16_var5 = *(pu8_var10 + 0xc);
        if !(*pu8_var10 < u_stack14 as u8) {
            break;
        }
    } // while (*pu8_var10 < u_stack14);
    param_1.field_0xa = *pu8_var10;
    return true;
}

pub unsafe fn mem_op_1000_0308(mut param_1: i16,
                               pstruct7_param_2: *mut Struct7) -> *mut u8
{
    let mut pu8_var1: *mut u8;
    let mut pu8_var2: *mut u8;
    let mut b_var3: bool;
    let mut pu8_var4: *mut u8;

    if (pstruct7_param_2 + 0xa) == 0x0 {
        b_var3 = mem_op_1000_01b0(pstruct7_param_2);
        if CONCAT11(AH_REG, b_var3) == 0x0 {
            return ptr::null_mut();
        }
    }
    pu8_var1 = (pstruct7_param_2.field_0x0a);
    (pstruct7_param_2.field_0xa) = (pu8_var1 + 0x4);
    pu8_var4 =  pstruct7_param_2[param_1].field_0x02;
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

pub unsafe fn pass1_1000_0368(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16 )
{
    let mut pu16_var1: *mut u16 = ptr::null_mut();

    if ((param_1 + 0x4) == param_1) {
        (param_3 + param_2 * 0x2) = 0x0;
    } else {
        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        *pu16_var1 =  (param_2 * 0x2 + param_3);
        if *pu16_var1 == param_1 {
            *pu16_var1 = (param_1 + 0x4);
        }
    }
    (param_1 + 0x4) = (param_3 + 0xa);
    (param_3 + 0xa) = param_1;
}

pub fn pass1_1000_05b4(param_1: u8,
                       param_2: *mut u16)
{
    *(param_2 + 0xa) = 0x1;
    *(param_2 + 0x8) = 0x668;
    *(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    *(param_2 + 0x10) = 0x0;
    *(param_2 + 0xe) = 0x0;
}

pub unsafe fn mem_1000_0668() -> u32
{
    let mut u_var1: u32;

    u_var1 = mem_op_1000_0510(0x0, ptr::null_mut());
    return u_var1;
}

pub unsafe fn pass1_1000_07ac(mut param_1: u16,
                              mut param_2: i16,
                              mut param_3: *mut Struct_1000_07ac_1) {
    let mut pu_var1: *mut u16;
    let mut i_var2: u16;
    let mut u_var3: u16;

    pu_var1 = (param_3.field_0x10);
    (param_3.field_0x0e) = pu_var1;
    u_var3 = param_2 + (param_3 - pu_var1);
    i_var2 = pu_var1 + (u_var3 - u_var3 % param_1);
    *(param_3.field_0x10) = i_var2;
    while *pu_var1 < (i_var2 - param_1) {
        *pu_var1 = (pu_var1 + param_1);
        pu_var1 = (pu_var1 + param_1);
    }
    *pu_var1 = 0x0;
    return;
}

pub unsafe fn mem_op_1000_0838(mut pstruct7_param1: *mut Struct7) -> u32
{
    let mut pu8_var1: *mut u8;
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

    pu16_var9 =  &mut pstruct7_param1.field_0x02;
    pu16_var12 = pu16_var9;
    if (pstruct7_param1.field_0x02) == 0x0 {
        // TODO:
        // goto LAB_1000_085b;
    }
    loop {
        loop {
            if *pu16_var9 != 0x0 {
                u16_var3 = pu16_var9[0x5];
                pu8_var4 =  PTR_LOOP_1050_000e;
                if !pu8_var4.is_null() {
                    PTR_LOOP_1050_000e = pu8_var4;
                    pu8_var2 =  PTR_LOOP_1050_000a;
                    *pu8_var2 = *pu8_var2 + 0x1;
                     pstruct7_param1.field_0x02 = *pu16_var9;
                    return CONCAT22(u16_var3, *pu8_var4 as u16);
                }
                *pu16_var9 = 0x0;
            }
            pu16_var9 =  pu16_var9[0x2];
            if !(pu16_var9 != pu16_var12) {
                break;
            }
        } //while (piVar9 != piStack4);
//        LAB_1000_085b:
        if (pstruct7_param1.field_0x18) == 0x0 {
            // &DAT_1050_1050
            pass1_1000_1e61(REG_CS, 0x4, pstruct7_param1);
            return 0x0;
        }
        u16_var5 = (pstruct7_param1.field_0x1a);
        loop {
            u16_var11 = u16_var5;
            u16_var5 = 0x1;
            u16_var8 = mem_op_1000_03c6(u16_var11,
                                        0x0,
                                        0x1,
                                        pstruct7_param1,
                                        0x0,
                                        0);
            if (u16_var8 | u16_var5) != 0x0 {
                break;
            }
            u16_var5 = (pstruct7_param1.field_0x1e);
            u16_var6 = u16_var5 + u16_var11;
            u16_var5 = (pstruct7_param1.field_0x20) +  CARRY2(u16_var5,
                                                          u16_var11);
            pu8_var1 =  (pstruct7_param1.field_0x28);
            b_var10 = *pu8_var1 <= u16_var5 as u8;
            if (b_var10) {
                if b_var10 && (u16_var5 != *pu8_var1 as u16) {
                    return 0x0;
                }
                pu8_var1 =  (pstruct7_param1.field_0x26);
                if (*pu8_var1 <= u16_var6 as u8) && (u16_var6 != *pu8_var1 as u16) {
                    return 0x0;
                }
            }
            u16_var5 = u16_var11 >> 0x1;
            if (u16_var11 >> 0x1) < (pstruct7_param1.field_0x18) + 0x14 {
                // &DAT_1050_1050
                u16_var7 = pass1_1000_1e61(REG_CS, 0x2,
                                                            pstruct7_param1);
                u16_var5 = u16_var11 & 0xfffe;
                if u16_var7 == 0x0 {
                    return 0x0;
                }
            }
        }
        *pu16_var9 =  pstruct7_param1.field_0x02;
        pu16_var12 =  pu16_var9[0x2];
    }
}

pub unsafe fn mem_op_1000_03c6(mut param_1: u16,
                               mut param_2: i16,
                               mut param_3: u16,
                               mut param_4: *mut Struct7,
                               param_5: u8,
                               mut param_6: u16 ) -> u16
{
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut UVar6: u16;
    let mut uVar7: u16;
    // let mut unaff_CS: u16;
    let mut bVar8: bool;
    let mut DVar9: u32;
    let mut uVar10: u16;
    let mut uStack20: u16;
    let mut uVar9: u16;

    uVar7 = CONCAT11(param_6, param_5);
    uVar3 = param_1 + 0xfff & 0xf000;
    puVar1 =  (param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = param_2 +  (0xf000 < param_1) + (param_4 + 0x20) +  CARRY2(uVar3, *puVar1);
    puVar1 =  (param_4 + 0x28);
    bVar8 = uVar3 < *puVar1;
    if ((bVar8)
        || ((bVar8 || uVar3 == *puVar1 && (puVar1 =  (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
        if (param_3 == 0x3) {
            uStack20 =  ( (- ((param_5 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
        } else {
            uStack20 = 0x1000;
        }
        uStack20 = (param_4 + 0x16) | uStack20;
        block_1000_1000::mem_op_1000_131c(uStack20,
                                          CONCAT22(param_2,
                                  param_1));
        if ((uVar3 | uStack20) != 0x0) {
            puVar5 =  mem_op_1000_0308(param_3,
                                              param_4);
            if (puVar5 != NULL) {
                puVar5[0x4] = uStack20;
                puVar5[0x5] = uVar3;
                PTR_LOOP_1050_000c = param_3 | 0xcad0;
                //        *NULL = param_4;
                //        &u16_1050_0002 = &DAT_1050_1050;
                u16_1050_0002 = 0x1050;
                u32_1050_0004 = puVar5;
                // (&u32_1050_0004 + 0x2) = &DAT_1050_1050;
                u32_1050_0006 = 0x1050;
                PTR_LOOP_1050_000a = 0x0;
                // SUB42(&DAT_1050_1050,0x0);
                uVar10 = 0x1050;
                DVar9 = mem_op_1000_1532(uStack20,
                                         uVar3);
                UVar6 =  DVar9;
                if (param_3 == 0x1) {
                    // &DAT_1050_1050
                    uVar7 = pass1_1000_0782(param_4,
                                            UVar6,
                                            0x0,
                                            0x1050);
                } else if (param_3 == 0x3) {
                    pass1_1000_05b4(param_5,
                                    0x0);
                } else {
                    uVar7 = pass1_1000_09ca(UVar6,
                                            NULL);
                }
                param_2 =  (DVar9 >> 0x10);
                *puVar5 = uVar7;
                puVar5[0x1] = 0x8000;
                puVar1 =  (param_4 + 0x1e);
                uVar4 = *puVar1;
                *puVar1 = *puVar1 + UVar6;
                piVar2 =  (param_4 + 0x20);
                *piVar2 = *piVar2 + param_2 +  CARRY2(uVar4,
                                                           UVar6);
                return uVar3;
            }
            mem_op_1000_13ce(uStack20,
                             uVar3);
        }
    } else {
        // &DAT_1050_1050
        block_1000_1000::pass1_1000_1e61(unaff_CS,
                                         0x7,
                                         param_4);
    }
    return 0x0;
}

pub unsafe fn mem_op_1000_0510(mut param_1: u16,
                        param_2: *mut Struct7) -> u32
{
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut bVar3: u8;
    let mut iVar4: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;
    let mut DVar12: u32;
    let mut lVar13: i32;
    let mut uVar14: u16;
    let mut uVar5: u16;

    iVar4 = param_2;
    uVar5 = (param_2.field_0x2);
    uVar6 = (param_2.field_0x4);
    bVar3 = * (param_2 + 0xc);
    // &DAT_1050_1050
    DVar12 = mem_op_1000_1532(param_2,
                              0x1050);
    uVar9 =  (DVar12 >> 0x10);
    uVar8 =  DVar12;
    //  uVar14 = &DAT_1050_1050;
    uVar14 = 0x1050;
    if (param_1 != 0x0) {
        uVar7 = (iVar4 + 0x1e);
        uVar10 = ((iVar4 + 0x20) - uVar9) -  (uVar7 < uVar8);
        puVar1 =  (iVar4 + 0x24);
        bVar11 = uVar10 < *puVar1;
        if ((bVar11 || uVar10 == *puVar1) && ((bVar11 || (uVar7 - uVar8 < (iVar4 + 0x22))))) {
            bVar11 = false;
            uVar9 = uVar10;
            // TODO
            //goto LAB_1000_0595;
        }
    }
    pass1_1000_0368(uVar6,
                    bVar3 & 0x7,
                    0x0);
    puVar1 = 0x001e;
    uVar7 = *puVar1;
    *puVar1 = *puVar1 - uVar8;
    // s_New_failed_in_Op__Op_1050_0020;
    piVar2 = 0x0020;
    *piVar2 = (*piVar2 - uVar9) -  (uVar7 < uVar8);
    bVar11 = true;//
//    LAB_1000_0595:
    if (bVar11) {
        (param_2 + 0xc) = 0x0;
        lVar13 = mem_op_1000_13ce(param_2,
                                  uVar14);
        return CONCAT22( ( lVar13 >> 0x10),
                        0x1);
    }
    return  uVar9 << 0x10;
}

pub unsafe fn mem_op_1000_05e2(mut param_1: u16,
                        mut param_2: i16,
                        mut param_3: u16,
                        param_4: *mut Struct7) -> u32
{
    let mut puVar1: *mut u16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut UVar5: u16;
    let mut UVar6: u32;
    let mut unaff_CS: u16;
    let mut bVar5: bool;
    let mut uVar6: u32;

    iVar2 = param_2 +  (0xffeb < param_1);
    loop {
        uVar3 = 0x3;
        UVar6 = param_3;
        UVar6 = mem_op_1000_03c6(param_1 + 0x14,
                                 iVar2,
                                 0x3,
                                 param_4,
                                 UVar6,
                                 UVar6);
        if ((UVar6 | uVar3) != 0x0) {
            return CONCAT22(UVar6,
                            uVar3 + 0x14);
        }
        uVar6 = mem_op_1000_0052(param_4);
        uVar3 = param_1 + 0x1013 & 0xf000;
        puVar1 =  (param_4 + 0x1e);
        uVar4 = uVar3 + *puVar1;
        uVar3 = iVar2 +  (0xf000 < param_1 + 0x14) + (param_4 + 0x20) +  CARRY2(uVar3,
                                                                                          *puVar1);
        puVar1 =  (param_4 + 0x28);
        bVar5 = uVar3 < *puVar1;
        // &DAT_1050_1050)
        if  (((bVar5 || uVar3 == *puVar1)
        && ((bVar5 || (puVar1 =  (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) && ((uVar6 != 0x0
        || (UVar5 = block_1000_1000::pass1_1000_1e61(unaff_CS,
                                                     0x2,
                                                     param_4), UVar5 != 0x0)))) == false {
                                                        break;
                                                     }
    }
    return 0x0;
}

pub unsafe fn mem_1000_0670(mut param_1: u16,
                     param_2: *mut Struct7,
                     mut param_3: i16,
                     mut param_4: u16,
                     param_5: *mut i16) -> u16
{
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut UVar3: u16;
    let mut UVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uVar9: *mut u32;
    let mut UVar10: u16;
    let mut BVar11: bool;
    let mut uVar12: u16;
    let mut uVar13: u16;
    // let mut unaff_CS: u16;
    let mut DVar14: u32;
    let mut DVar15: u32;
    let mut uStack16: u16;

    UVar3 = param_2;
    UVar4 = ( param_2 + 0x2);
    // &DAT_1050_1050
    DVar14 = mem_op_1000_1532(param_2, 0x1050);
    iVar6 = param_3 +  (0xffeb < param_1);
    uVar7 = *param_2;
    uVar8 = - ((param_4 & 0x1) != 0x0) & 0x100 | - ((param_4 & 0x4) != 0x0) & 0x400 | ( uVar7 + 0x16);
    if param_5 == NULL {
        //  &DAT_1050_1050
        BVar11 = mem_op_1000_14f2(uVar8 | 0x2000,
                                  param_1 + 0x14,
                                  param_2,
                                  0x1050);
        if (BVar11 == 0x0) {
            return 0x0;
        }
        // &DAT_1050_1050
        uStack16 = 0x1050;
    } else {
        iVar5 = (param_2 + 0x1);
        uVar12 = ( param_2 + 0x6);
        uVar13 = uVar12;
        loop {
            uStack16 = uVar13;
            puVar9 =  (uVar8 | 0x2000);
            // &DAT_1050_1050
            block_1000_1000::mem_op_1000_1408(puVar9,
                                              CONCAT22(iVar6,
                                      param_1 + 0x14),
                                              param_2,
                                              0x1050);
            uVar13 = uStack16 | puVar9;
            if (uVar13 != 0x0) {
                break;
            }
            UVar10 = block_1000_1000::pass1_1000_1e61(unaff_CS,
                                                      0x2,
                                                      UVar3);
                                                      if (UVar10 != 0x0) == false {
                                                        break;
                                                      }
        }
        if ((uStack16 |  puVar9) == 0x0) {
            ( param_5 + 0x2) = 0x0;
            *param_5 = 0x0;
            return 0x0;
        }
         (iVar5 + 0x8) = puVar9;
        (iVar5 + 0xa) = uStack16;
        *param_5 =  (puVar9 + 0x5);
        ( param_5 + 0x2) = uStack16;
        param_2 = puVar9;
    }
    DVar15 = mem_op_1000_1532( param_2,
                              uStack16);
    uVar12 =  (DVar15 - DVar14);
    puVar1 =  (UVar3 + 0x1e);
    uVar8 = *puVar1;
    *puVar1 = *puVar1 + uVar12;
    piVar2 =  (UVar3 + 0x20);
    *piVar2 = *piVar2 +  (DVar15 - DVar14 >> 0x10) +  CARRY2(uVar8,
                                                                       uVar12);
    return 0x1;
}

pub unsafe fn pass1_1000_0782(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: i16,
                       mut param_4: u16 ) -> u16
{
    (param_3 + 0xe) = 0x0;
    (param_3 + 0x10) = param_3 + 0x14;
    (param_3 + 0x8) = 0x9a0;
    pass1_1000_07ac((param_1 + 0x18),
                    param_2,
                    param_3);
    return 0x1;
}

pub unsafe fn pass1_1000_07fc(mut param_1: u32) -> *mut astruct_99
{
    // let mut unaff_CS: u16;
    let mut paVar1: *mut astruct_99;

    if (( param_1 + 0x14) != -0x4153) {
        block_1000_1000::pass1_1000_1e61(unaff_CS,
                                         0xa,
                                         0x0);
        return NULL;
    }
    paVar1 =  mem_op_1000_0838(0x0);
    return paVar1;
}

pub unsafe  fn pass1_1000_093a(param_1: *mut astruct_611) -> u16
{
    let mut piVar1: *mut i16;
    let mut unaff_CS: u16;

    if (&PTR_LOOP_1050_000c != -0x352f) {
        block_1000_1000::pass1_1000_1e61(unaff_CS,
                                         0xe,
                                         0x0);
        return 0x0;
    }
    param_1 =  PTR_LOOP_1050_000e;
    if (param_1 == 0x0) {
         u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000e =  param_1;
    piVar1 =  &PTR_LOOP_1050_000a;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 == 0x0) {
        mem_op_1000_0510(0x1,
                         0x0);
    }
    return 0x1;
}

pub unsafe fn pass1_1000_09a0(param_1: *mut u16) -> *mut u8
{
    let mut puVar1: *mut u8 = null_mut();
    let mut uVar2: u32;

    *param_1 = PTR_LOOP_1050_000e;
    if (PTR_LOOP_1050_000e == NULL) {
        u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
    puVar1 = PTR_LOOP_1050_000e;
    PTR_LOOP_1050_000e =  param_1;
    if (PTR_LOOP_1050_000a == NULL) {
        uVar2 = mem_op_1000_0510(0x1,
                                 0x0);
        puVar1 =  uVar2;
    }
    return puVar1;
}

pub unsafe fn pass1_1000_09ca(mut param_1: i16,
                       param_2: *mut u16) -> u16
{
    let mut puVar1: *mut u16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut puVar4: *mut u16;

    puVar1 = param_2 + 0xa;
    puVar4 =  (( param_2 + (param_1 -  puVar1) + -0x6 & 0xfffc) +  puVar1);
    *puVar4 = 0x1;
    param_2[0x7] =  puVar1;
    puVar4[0x2] =  puVar4;
    puVar4[0x1] =  puVar4;
    param_2[0x8] =  puVar4;
    if ((* (param_2 + 0x6) & 0x7) == 0x2) {
        param_2[0x9] = 0x8;
    } else {
        uVar3 =  param_2;
        iVar2 = ( uVar3 + 0x18);
        // param_2[0x9] = (iVar2 - 0x5 & ~- (iVar2 + 0x3 < 0x8)) + 0x8;
    }
    puVar4[-0x1] =  puVar4 -  puVar1;
    *puVar1 =  puVar4 -  puVar1 | 0x2;
    param_2[0xc] =  puVar4;
    param_2[0xb] = puVar4[0x1];
     (puVar4[0x1] + 0x4) = puVar1;
    puVar4[0x1] =  puVar1;
    param_2[0x4] = 0xe08;
    return *puVar1 & 0xfffc;
}

pub unsafe fn mem_op_1000_0a48(param_1: u8,
                        mut param_2: u16,
                        mut param_3: i16,
                        param_4: *mut Struct7) -> i32
{
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar4: u16;
    let mut uVar3: u16;
    //    let mut UVar4: u16;
    let mut unaff_CS: u16;
    let mut uVar5: u32;
    let mut in_stack_00000005: u8;
    let mut puVar1: *mut u16;

    //    UVar4 =  (param_4 >> 0x10);
    if ((param_4.field_0x14) == -0x4153) {
        // (s_version__d__d_1050_0012 + 0x6)
        if ((param_3 == 0x0) && (param_2 <= 0x0018)) {
            if (param_2 == 0x0) {
                pass1_1000_1e61(unaff_CS,
                                                 0x4,
                                                 param_4);
                uVar5 = 0x0;
            } else {
                uVar5 = mem_op_1000_0838(0x0);
                uVar3 =  (uVar5 >> 0x10);
                puVar2 =  uVar5;
                if ((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
                    // (s_version__d__d_1050_0012 + 0x6)
                    uVar1 = 0x0018;
                    // for (uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1)
                    for   uVar4 in uVar1 >> 0x1 .. 0
                    {
                        puVar1 = puVar2;
                        puVar2 = puVar2 + 0x1;
                        *puVar1 = 0x0;
                    }
                    if ((uVar1 & 0x1) != 0x0) {
                        * puVar2 = 0x0;
                    }
                }
            }
        }
            // (s_version__d__d_1050_0012 + 0xa))
        else if ((param_3 == 0x0) && (param_2 <= 0x001c)) {
            uVar5 = mem_op_1000_0b20(param_1 & 0xfffd,
                                     0x0,
                                     param_2);
        } else {
            uVar5 = mem_op_1000_05e2(param_2,
                                     param_3,
                                     param_1 & 0xfffd,
                                     0x0);
        }
        return uVar5;
    }
    block_1000_1000::pass1_1000_1e61(unaff_CS,
                                     0xa,
                                     0x0);
    return 0x0;
}

pub unsafe fn mem_op_1000_0b20(mut param_1: u16,
                        mut param_2: u16,
                        mut param_3: u16 ) -> u32
{
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut UVar6: u16;
    let mut pu16_var7: *mut u16;
    let mut uVar8: u16;
    let mut bVar9: bool;
    let mut uVar10: u32;
    let mut uStack20: u16;
    let mut puStack6: *mut u16;

    uVar8 = 0x1050;
    uVar2 = param_1 & 0x2;
    uVar4 = param_3 + 0x5 & 0xfffc;
    // uVar4 = uVar4 - 0x8 & ~- (uVar4 < 0x8);
    uVar5 = uVar4 + 0x8;
    pu16_var7 =  (uVar2 * 0x2 + param_2);
    uStack20 = param_1;
    puStack6 = pu16_var7;
    if (pu16_var7 == NULL) {
        // goto LAB_1000_0b64;
    }
    loop {
        loop {
            if ((uVar5 <= *pu16_var7) && (uVar10 = pass1_1000_0c32(uVar5,
                                                                   uStack20,
                                                                   0x0), uVar10 != 0x0)) {
                 (uVar2 * 0x2 + param_2) = pu16_var7;
                return uVar10;
            }
            pu16_var7 =  pu16_var7[0x2];
        if (pu16_var7 == puStack6) {
            break;
        }
        }
//        LAB_1000_0b64:
        if (((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) || ((param_2 + 0x32) == 0x0))
            || (uVar3 = ( (param_2 + 0x32))(), uVar3 < uVar5)) {
            if (((uStack20 & 0x10) != 0x0) || (uVar3 = uVar2, UVar6 = mem_op_1000_03c6((param_2 + 0x1a),
                                                                                       0x0,
                                                                                       uVar2,
                                                                                       param_2,
                                                                                       0x0,
                                                                                       '\0'), (UVar6 | uVar3) == 0x0)) {
                if ((uStack20 & 0x20) == 0x0) {
                    uVar2 = uVar4 + 0x1007 & 0xf000;
                    puVar1 =  (param_2 + 0x1e);
                    uVar4 = uVar2 + *puVar1;
                    uVar2 = (param_2 + 0x20) +  CARRY2(uVar2,
                                                            *puVar1);
                    puVar1 =  (param_2 + 0x28);
                    bVar9 = uVar2 < *puVar1;
                    if ((bVar9 || uVar2 == *puVar1)
                        && ((bVar9 || (puVar1 =  (param_2 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
                        uVar10 = mem_op_1000_05e2(uVar5,
                                                  0x0,
                                                  uStack20,
                                                  param_2);
                        return uVar10;
                    }
                }
                return 0x0;
            }
        } else {
            uStack20 |= 0x40;
        }
        pu16_var7 =  (uVar2 * 0x2 + param_2);
        puStack6 =  pu16_var7[0x2];
    }
}

pub unsafe fn pass1_1000_0c32(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16 ) -> U32
{
    let mut puVar1: *mut u16;
    let mut pbVar2: *mut u8;
    let mut piVar3: *mut i16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut iVar7: i16;
    let mut puVar8: *mut u16;
    let mut uVar9: u16;
    let mut uStack14: u16;
    let mut puStack8: *mut u16;
    let mut uStack6: u16;

    puVar8 =  (param_3 + 0xe);
    uStack6 = 0x0;
    puVar6 = puVar8;
    loop {
        loop {
            uVar5 = *puVar6;
            if (param_1 <= uVar5) {
                uVar5 = (uVar5 & 0xfffc) - param_1;
                puVar1 =  (param_3 + 0x12);
                puStack8 = puVar6;
                if (*puVar1 < uVar5 || *puVar1 == uVar5) {
                    uStack14 = param_1;
                    if ((param_2 & 0x6) == 0x0) {
                        puStack8 =  (uVar5 +  puVar6);
                        puStack8[-0x1] = uVar5;
                        *puVar6 = uVar5 | 0x2;
                        puVar8 =  puVar6[0x1];
                        pbVar2 =  ( puStack8 + param_1);
                        *pbVar2 = *pbVar2 | 0x2;
                        *puStack8 = param_1 | 0x1;
                    } else {
                        *puVar6 = param_1 & 0xff00 | * puVar6 & 0x2 | param_1 & 0xff | 0x1;
                        (puVar6[0x2] + 0x2) = puVar6[0x1];
                        (puVar6[0x1] + 0x4) = puVar6[0x2];
                        puVar8 =  ( puVar6 + param_1);
                        ( puVar8 + (uVar5 - 0x2)) = uVar5;
                        *puVar8 = uVar5 | 0x2;
                        uVar5 = (param_3 + 0x10);
                        puVar8[0x2] = uVar5;
                        puVar8[0x1] = (uVar5 + 0x2);
                         ((uVar5 + 0x2) + 0x4) = puVar8;
                         (uVar5 + 0x2) = puVar8;
                    }
                } else {
                    puVar8 =  puVar6[0x1];
                     (puVar6[0x2] + 0x2) = puVar8;
                    (puVar6[0x1] + 0x4) = puVar6[0x2];
                    puVar1 = puVar6;
                    * puVar1 = * puVar1 | 0x1;
                    uStack14 = *puVar6 & 0xfffc;
                    ( puVar6 + uStack14) = ( puVar6 + uStack14) | 0x2;
                }
                 (param_3 + 0xe) = puVar8;
                if ((param_2 & 0x1) != 0x0) {
                    puVar6 = puStack8;
                    uVar5 = uStack14 - 0x2 >> 1;
                    puVar6 = puVar6 + 0x1;
                    while uVar5 != 0
                    {
                        *puVar6 = 0x0;
                        uVar5 -= 1;
                    }
                    if ((uStack14 - 0x2 & 0x1) != 0x0) {
                        * puVar6 = 0x0;
                    }
                }
                if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
                     (param_3 + 0x4) =  ((param_3 + 0x10) + 0x2) & 0xfffc;
                    uVar4 =  (param_3 + 0x4);
                    pbVar2 =  ( uVar4 + 0x3);
                    *pbVar2 = *pbVar2 | 0x80;
                }
                piVar3 =  (param_3 + 0xa);
                *piVar3 = *piVar3 + 0x1;
                return CONCAT22(0x1050,
                                puStack8 + 0x1);
            }
            if (uStack6 < uVar5) {
                uStack6 = uVar5;
            }
            puVar6 =  puVar6[0x1];
            if puVar6 == puVar8 { break; }
        }
        if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) {
            break;
        }
        uVar4 =  param_3;
        uVar9 =  ( uVar4 >> 0x10);
        iVar7 =  uVar4;
        if ((iVar7 + 0x34) == 0x0) {
            break;
        }
        uStack6 = ( (iVar7 + 0x34))();
        if ((uStack6 < param_1) || (puVar6 =  (param_3 + 0xe), puVar6 == NULL)) {
            break;
        }
    }
     (param_3 + 0x4) = uStack6 & 0xfffc;
    return 0x0;
}

pub unsafe fn call_fn_ptr_1000_0dc6(param_1: *mut c_char) -> bool {
    if (PTR_LOOP_1050_000c & 0xfff8) != 0xcad0 {
        pass1_1000_1e61(REG_CS, 0xe, ptr::null_mut());
        return false;
    }
    // &DAT_1050_1050
    u16_1050_0008.unwrap()(0x1050);
    return true;
}

pub unsafe fn pass1_1000_0e08(mut param_1: i16) -> u16
{
    let mut puVar1: *mut u16;
    let mut pbVar2: *mut u8;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u16;
    let mut bVar6: bool;
    let mut uVar7: u32;

    puVar5 =  (param_1 + -0x2);
    bVar6 = (* puVar5 & 0x2) != 0x0;
    if (bVar6) {
        puVar1 = puVar5;
        * puVar1 = * puVar1 & 0xfe;
    } else {
        puVar4 =  ( puVar5 - (param_1 + -0x4));
        puVar1 = puVar4;
        *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
        puVar5 = puVar4;
    }
    puVar4 =  ((*puVar5 & 0xfffc) +  puVar5);
    if ((* puVar4 & 0x1) == 0x0) {
        puVar1 = puVar5;
        *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
        if (puVar4 ==  PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e =  puVar5;
        }
        (puVar4[0x2] + 0x2) = puVar4[0x1];
        (puVar4[0x1] + 0x4) = puVar4[0x2];
        puVar4 =  ((*puVar5 & 0xfffc) +  puVar5);
    }
    puVar4[-0x1] = *puVar5 & 0xfffc;
    uVar3 = u32_1050_0004;
    puVar1 = puVar4 + -0x1;
    if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
        uVar3 = *puVar5 & 0xfffc;
        u32_1050_0004 = uVar3;
    }
    puVar1 = puVar4;
    * puVar1 = * puVar1 & 0xfd;
    if (bVar6) {
        if ( PTR_LOOP_1050_0010.address_offset_field_0x2 != PTR_LOOP_1050_0010) {
            pbVar2 =  ( u32_1050_0004 + 0x3);
            *pbVar2 = *pbVar2 & 0x7f;
        }
        puVar5[0x2] =  PTR_LOOP_1050_0010;
        uVar3 = PTR_LOOP_1050_0010.address_offset_field_0x2;
        puVar5[0x1] = uVar3;
         (PTR_LOOP_1050_0010.address_offset_field_0x2 + 0x4) = puVar5;
        PTR_LOOP_1050_0010.address_offset_field_0x2 =  puVar5;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
    if (PTR_LOOP_1050_000a == NULL) {
        uVar7 = mem_op_1000_0510(0x1,
                                 0x0);
        uVar3 =  uVar7;
    }
    return uVar3;
}

pub unsafe fn pass1_1000_0ed4(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16,
                       param_4: *mut astruct_172,
                       param_5: *mut astruct_172) -> i32
{
    let mut paVar1: *mut astruct_172;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
        // struct astruct_172 **ppaVar4;
    let mut ppaVar4: *mut *mut astruct_172;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut paVar7: *mut astruct_172;
    let mut puVar8: *mut u16;
    let mut unaff_CS: u16;
    let mut lVar9: i32;
    let mut UVar10: u16;
    let mut UVar11: u16;
    let mut UVar12: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
        UVar11 = *NULL;
        UVar12 = &u16_1050_0002;
        if ((param_1 & 0x8) == 0x0) {
            ppaVar4 = &param_4;
            //  &DAT_1050_1050;
            uVar6 = 0x1050;
        } else {
            ppaVar4 = NULL;
            uVar6 = 0x0;
        }
        uVar6 = pass1_1000_0fb8(param_2,
                                 param_4,
                                param_3,
                                param_1,
                                 ppaVar4,
                                uVar6);
        if (uVar6 == 0x0) {
            return CONCAT22(param_5,
                            param_4);
        }
        if ((param_1 & 0x8) == 0x0) {
            lVar9 = mem_op_1000_0a48( param_1,
                                     param_2,
                                     param_3,
                                     CONCAT22(UVar12,
                                              UVar11));
            uVar3 =  ( lVar9 >> 0x10);
            puVar8 =  lVar9;
            if (lVar9 != 0x0) {
                paVar7 = param_4;
                // for (uVar5 = uVar6 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1)
                for uVar5 in uVar6 >> 0x1 .. 0
                {
                    puVar2 = puVar8;
                    puVar8 = puVar8 + 0x1;
                    paVar1 = paVar7;
                    paVar7 =  &paVar7.field1_0x2;
                    *puVar2 = paVar1.field0_0x0;
                }
                // for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1)
                for uVar6 in uVar6 & 0x1 .. 0
                {
                    puVar2 = puVar8;
                    puVar8 =  ( puVar8 + 0x1);
                    paVar1 = paVar7;
                    paVar7 =  ( &paVar7.field0_0x0 + 0x1);
                    * puVar2 = * &paVar1.field0_0x0;
                }
                call_fn_ptr_1000_0dc6( CONCAT22(param_5,
                                                        param_4));
            }
            return lVar9;
        }
        if ((param_3 | param_2) == 0x0) {
            return 0x0;
        }
        UVar10 = 0x5;
    } else {
        UVar11 = 0x0;
        UVar12 = 0x0;
        UVar10 = 0xe;
    }
    block_1000_1000::pass1_1000_1e61(unaff_CS,
                                     UVar10,
                                     UVar11);
    return 0x0;
}

pub unsafe fn pass1_1000_0fb8(mut param_1: u16,
                       mut param_2: i16,
                       mut param_3: u16,
                       mut param_4: u16,
                       param_5: *mut u16,
                       mut param_6: u16 ) -> u16
{
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

    if ((param_3 | param_1) == 0x0) {
        block_1000_1000::pass1_1000_1e61(unaff_CS,
                                         0x4,
                                         PTR_LOOP_1050_0000);
        if ((param_6 |  param_5) != 0x0) {
            param_5[0x1] = 0x0;
            *param_5 = 0x0;
            return 0x0;
        }
        return 0x1;
    }
    bVar2 =  PTR_LOOP_1050_000c & 0x7;
    if (( PTR_LOOP_1050_000c & 0x7) != 0x0) {
        if (bVar2 == 0x1) {
            uVar3 = (PTR_LOOP_1050_0000 + 0x18);
            if (param_3 != 0x0) {
                return uVar3;
            }
            if (param_1 <= uVar3) {
                return 0x0;
            }
            return uVar3;
        }
        if (bVar2 != 0x2) {
            if (bVar2 != 0x3) {
                if ((param_6 |  param_5) != 0x0) {
                    param_5[0x1] = 0x0;
                    *param_5 = 0x0;
                    return 0x0;
                }
                return 0x1;
            }
            if ((((param_6 |  param_5) != 0x0) && (param_3 == 0x0)) && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c))) {
                uVar9 = block_1000_1000::pass1_1000_1284(CONCAT22(0x1050,
                                                                  param_2));
                if (CONCAT22(param_3,
                             param_1) < uVar9) {
                    return param_1;
                }
                return  uVar9;
            }
            iVar5 = mem_1000_0670(param_1,
                                  NULL,
                                  param_3,
                                  param_4,
                                   CONCAT22(param_6,
                                                   param_5));
            if (iVar5 != 0x0) {
                return 0x0;
            }
            if ((param_6 |  param_5) != 0x0) {
                return 0x0;
            }
            return 0x1;
        }
    }
    puVar8 =  (param_2 + -0x2);
    uVar3 = *puVar8 & 0x7ffc;
    uStack4 = uVar3 - 0x2;
    if ((* (param_2 + -0x1) & 0x80) != 0x0) {
        uStack4 = uVar3 - 0x6;
    }
    if ((((param_3 == 0x0) && (param_1 <= uStack4)) || ((param_3 == 0x0 && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c)))))
        && (BVar4 = pass1_1000_115c(param_1,
                                    puVar8), BVar4 != 0x0)) {
        if ((param_4 & 0x1) != 0x0) {
            uVar3 = (*puVar8 & 0x7ffc) - 0x2;
            if (uStack4 < param_1) {
                puVar7 =  (uStack4 + param_2);
                iVar5 = -uStack4;
            } else {
                if (uVar3 <= param_1) {
                    return 0x0;
                }
                puVar7 =  (param_1 + param_2);
                iVar5 = -param_1;
            }
            uVar3 += iVar5;
            // for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1)
            for uVar6 in uVar3 >> 1 .. 0
            {
                puVar1 = puVar7;
                puVar7 = puVar7 + 0x1;
                *puVar1 = 0x0;
            }
            if ((uVar3 & 0x1) != 0x0) {
                * puVar7 = 0x0;
            }
        }
        return 0x0;
    }
    return uStack4;
}

pub unsafe fn pass1_1000_115c(mut param_1: i16,
                       param_2: *mut u16) -> bool
{
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
        puVar5 =  (uVar3 +  param_2);
        if (((* puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
            return 0x0;
        }
        if (puVar5 ==  PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e =  puVar5[0x1];
        }
        (puVar5[0x2] + 0x2) = puVar5[0x1];
        (puVar5[0x1] + 0x4) = puVar5[0x2];
        uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
        if (uStack4 < s_version__d__d_1050_0012) {
            puVar2 = param_2;
            *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
            pbVar1 =  ( puVar5 + (*puVar5 & 0xfffc));
            *pbVar1 = *pbVar1 | 0x2;
            return 0x1;
        }
    } else {
        uStack4 = uVar3 - uVar4;
        if (uStack4 < s_version__d__d_1050_0012) {
            return 0x1;
        }
        puVar5 =  (uVar3 +  param_2);
        if ((* puVar5 & 0x1) == 0x0) {
            uStack4 += *puVar5 & 0xfffc;
            if (puVar5 ==  PTR_LOOP_1050_000e) {
                PTR_LOOP_1050_000e =  puVar5[0x1];
            }
            (puVar5[0x2] + 0x2) = puVar5[0x1];
            (puVar5[0x1] + 0x4) = puVar5[0x2];
        }
        if (u32_1050_0004 < uStack4) {
            u32_1050_0004 = uStack4;
        }
    }
    *param_2 = *param_2 & 0x8003 | uVar4;
    (uVar4 +  param_2) = uStack4 | 0x2;
    UVar6 = uVar4 +  param_2;
    (UVar6 + 0x4) = PTR_LOOP_1050_0010;
    (UVar6 + 0x2) = PTR_LOOP_1050_0010.address_offset_field_0x2;
    (PTR_LOOP_1050_0010.address_offset_field_0x2 + 0x4) = UVar6;
    PTR_LOOP_1050_0010.address_offset_field_0x2 = UVar6;
    ( (UVar6 + uStack4) + -0x2) = uStack4;
    pbVar1 =  (UVar6 + uStack4);
    *pbVar1 = *pbVar1 & 0xfd;
    return 0x1;
}
