use std::ptr;
use crate::block_1000;
use crate::globals::{DAT_1050_5f30, REG_CS};
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
        block_1000::pass1_1000_1e61(REG_CS,
                                    0xa,
                                    ptr::null_mut());
        return 0xffffffff;
    }
    u_var1 = mem_op_1000_0052(None);
    return u_var1;
}

pub unsafe fn mem_op_1000_0052(param_1: Option<*mut Struct7>) -> u32
{
    let mut pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut i_var3: i16;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut i_stack14: i16;
    let mut i_stack12: i16;
    let mut i_stack10: i16;
    let mut u_stack8: u16;

    u_var2 = (param_1 + 0x1e);
    i_var3 = (param_1 + 0x20);
    u_stack8 = 0x0;
    loop {
        i_stack10 = (u_stack8 * 0x2 + param_1);
        if (i_stack10 != 0x0) && (u_stack8 != 0x3) {
            i_stack14 = 0x0;
            loop {
                i_stack12 = (i_stack10 + 0x4);
                u_var4 = (u32) (i_stack10 + 0x8);
                if ( u_var4 + 0xa) == 0x0 {
                    u_var5 = mem_op_1000_0510(0x1,
                                              0x0);
                    if u_var5 == 0x0 {
                        // goto LAB_1000_00f9;
                    }
                    if i_stack12 == i_stack10 {
                        i_stack12 = 0x0;
                    }
                } else if i_stack14 == 0x0 {
                    i_stack14 = i_stack10;
                }
                i_stack10 = i_stack12;
                if i_stack12 == i_stack14 {
                    break;
                }
            } //while i_stack12 != i_stack14;
        }
        u_stack8 += 0x1;
        if u_stack8 >= 0x5 {
            break;
        }
    } //while u_stack8 < 0x5;
    if (param_1 + 0x32) != 0x0 {
        ((code) (param_1 + 0x32))();
    }
//    LAB_1000_00f9:
    pu_var1 = (param_1.field_0x1e);
    let a = (i_var3 - (param_1 + 0x20)) -  (u_var2 < *pu_var1);
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
        block_1000::pass1_1000_1e61(REG_CS,
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
                             0x0,
                             0x0,
                             '\0');
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
    let mut pu8_var10: *mut u16;
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
            block_1000::pass1_1000_1e61(REG_CS,
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
                                     0x1050,
                                     0);
            i_stack4 =  (dvar6 >> 0x10) as i16;
            u_stack6 = dvar6 as u16;
            if bvar3 != false {
                break;
            }
            u16_var5 = u16_var9 >> 0x1;
            if u16_var5 < 0xc {
                uvar4 = block_1000::pass1_1000_1e61(REG_CS,
                                                    0x2,
                                                    param_1);
                if uvar4 == 0x0 {
                    return (bool) ('\x01' - ((*param_1.field_0xa) == 0x0));
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
    // pu_var1 = (u16 *) (param_1 + 0x1e);
    pu_var1 = *param_1.field_0x1e;
    // u16_var9 = *pu_var1;
    u16_var9 = *pu_var1;
    *pu_var1 = *pu_var1 - dvar6;
    // pi_var2 =  (param_1 + 0x20);
    pi_var2 = param_1.field_0x20;
    *pi_var2 = (*pi_var2 -  (dvar6 >> 0x10)) -  (u16_var9 < dvar6 as u16);
    if u16_var5 != 0x0 {
        let mut val_1: u16 = 0;
        pu8_var10 = &mut val_1;
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
    pu_var1 = param_1.field_0x1e;
    u16_var9 = *pu_var1;
    *pu_var1 = *pu_var1 + u_stack12;
    pi_var2 = param_1.field_0x20;
    *pi_var2 = *pi_var2 + i_stack10 +  CARRY2(u16_var9,
                                              u_stack12);
    u16_var9 = (param_1.field_0xa);
    loop {
        *pu8_var10 = u16_var5;
        *(pu8_var10 + 0x4) = u16_var9;
        u16_var9 = *pu8_var10;
        u16_var5 = *(pu8_var10 + 0xc);
        if !(*pu8_var10 < u_stack14) {
            break;
        }
    } // while (*pu8_var10 < u_stack14);
    param_1.field_0xa = *pu8_var10;
    return true;
}

pub unsafe fn mem_op_1000_0308(mut param_1: i16, pstruct7_param_2: *mut astruct_7) -> *mut u8
{
    u8 *pu8_var1;
    let mut i16_var2: i16;
    let mut b_var3: bool;
    // u8 extraout_AH;
    let mut pi16_var4: *mut i16;

    if ((pstruct7_param_2 + 0xa) == 0x0) {
        b_var3 = mem_op_1000_01b0(pstruct7_param_2);
        if CONCAT11(AH_REG, b_var3) == 0x0 {
            return 0x0;
        }
    }
    pu8_var1 = (pstruct7_param_2.field_0xa);
    (pstruct7_param_2.field_0xa) = (pu8_var1 + 0x4);
    pi16_var4 =  (param_1 * 0x2 + pstruct7_param_2);
    if *pi16_var4 == 0x0 {
        *(pu8_var1 + 0x6) = pu8_var1;
        *(pu8_var1 + 0x4) = pu8_var1;
    } else {
        i16_var2 = *pi16_var4;
        *(pu8_var1 + 0x6) = i16_var2;
        *(pu8_var1 + 0x4) = (i16_var2 + 0x4);
        ((i16_var2 + 0x4) + 0x6) = pu8_var1;
        (i16_var2 + 0x4) = pu8_var1;
    }
    *pi16_var4 = pu8_var1;
    return pu8_var1;
}

pub fn pass1_1000_0368(mut param_1: u16,
                       mut param_2: u16,
                       mut param_3: u16 )
{
    let mut pu_var1: *mut u16;

    if ((param_1 + 0x4) == param_1) {
        (param_3 + param_2 * 0x2) = 0x0;
    } else {
        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        pu_var1 = (u16 *) (param_2 * 0x2 + param_3);
        if (*pu_var1 == param_1) {
            *pu_var1 = (param_1 + 0x4);
        }
    }
    (param_1 + 0x4) = (param_3 + 0xa);
    (param_3 + 0xa) = param_1;
}

pub fn pass1_1000_05b4(u8 param_1,
                       param_2: *mut u16)
{
    *(param_2 + 0xa) = 0x1;
    *(param_2 + 0x8) = 0x668;
    *(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    *(param_2 + 0x10) = 0x0;
    *(param_2 + 0xe) = 0x0;
}

pub fn mem_1000_0668(void) -> u32
{
    let mut u_var1: u32;

    u_var1 = mem_op_1000_0510(0x0,
                              0x0);
    return u_var1;
}

pub fn pass1_1000_07ac(mut param_1: u16,
                       mut param_2: i16,
                       mut param_3: i16)
{
    let mut pu_var1: *mut u16;
    let mut i_var2: i16;
    let mut u_var3: u16;

    pu_var1 = (u16 *) (param_3 + 0x10);
    (u16 *) (param_3 + 0xe) = pu_var1;
    u_var3 = param_2 + (param_3 - pu_var1);
    i_var2 =  pu_var1 + (u_var3 - u_var3 % param_1);
    (param_3 + 0x10) = i_var2;
    while (pu_var1 < (u16 *) (i_var2 - param_1)) {
        *pu_var1 = (u16 *) ( pu_var1 + param_1);
        pu_var1 = (u16 *) ( pu_var1 + param_1);
    }
    *pu_var1 = 0x0;
    return;
}

pub fn mem_op_1000_0838(mut param_1: u16 ) -> u32
{
    let mut pu_var1: *mut u16;
    let mut piVar2: *mut i16;
    let mut i_var3: i16;
    let mut pu_var4: *mut u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut uvar7: u16;
    let mut uvar8: u16;
    let mut piVar9: *mut i16;
    let mut unaff_cs: u16;
    let mut b_var10: bool;
    let mut u_stack6: u16;
    let mut piStack4: *mut i16;

    piVar9 = *(i16 **) (param_1 + 0x2);
    piStack4 = piVar9;
    if ((param_1 + 0x2) == 0x0) {
        goto LAB_1000_085b;
    }
    do {
        do {
            if (*piVar9 != 0x0) {
                i_var3 = piVar9[0x5];
                pu_var4 = (u16 *) &PTR_LOOP_1050_000e;
                if (pu_var4 != NULL) {
                    &PTR_LOOP_1050_000e = *pu_var4;
                    piVar2 =  &PTR_LOOP_1050_000a;
                    *piVar2 = *piVar2 + 0x1;
                    *(i16 **) (param_1 + 0x2) = piVar9;
                    return CONCAT22(i_var3,
                                    pu_var4);
                }
                *piVar9 = 0x0;
            }
            piVar9 =  piVar9[0x2];
        } while (piVar9 != piStack4);//
//        LAB_1000_085b:
        if ((param_1 + 0x18) == 0x0) {
            // &DAT_1050_1050
            block_1000::pass1_1000_1e61(REG_CS,
                                        0x4,
                                        param_1);
            return 0x0;
        }
        u_var5 = (param_1 + 0x1a);
        while (true) {
            u_stack6 = u_var5;
            u_var5 = 0x1;
            uvar8 = mem_op_1000_03c6(u_stack6,
                                     0x0,
                                     0x1,
                                     param_1,
                                     0x0,
                                     '\0');
            if ((uvar8 | u_var5) != 0x0) {
                break;
            }
            u_var5 = (param_1 + 0x1e);
            u_var6 = u_var5 + u_stack6;
            u_var5 = (param_1 + 0x20) +  CARRY2(u_var5,
                                                u_stack6);
            pu_var1 = (u16 *) (param_1 + 0x28);
            b_var10 = *pu_var1 <= u_var5;
            if (b_var10) {
                if (b_var10 && u_var5 != *pu_var1) {
                    return 0x0;
                }
                pu_var1 = (u16 *) (param_1 + 0x26);
                if (*pu_var1 <= u_var6 && u_var6 != *pu_var1) {
                    return 0x0;
                }
            }
            u_var5 = u_stack6 >> 0x1;
            if (u_stack6 >> 0x1 < (param_1 + 0x18) + 0x14U) {
                // &DAT_1050_1050
                uvar7 = block_1000::pass1_1000_1e61(REG_CS,
                                                    0x2,
                                                    param_1);
                u_var5 = u_stack6 & 0xfffe;
                if (uvar7 == 0x0) {
                    return 0x0;
                }
            }
        }
        piVar9 = *(i16 **) (param_1 + 0x2);
        piStack4 =  piVar9[0x2];
    } loop;
}

pub fn mem_op_1000_03c6(mut param_1: u16,
                        mut param_2: i16,
                        mut param_3: u16,
                        mut param_4: u16,
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
    puVar1 = (u16 *) (param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = param_2 +  (0xf000 < param_1) + (param_4 + 0x20) +  CARRY2(uVar3, *puVar1);
    puVar1 = (u16 *) (param_4 + 0x28);
    bVar8 = uVar3 < *puVar1;
    if ((bVar8)
        || ((bVar8 || uVar3 == *puVar1 && (puVar1 = (u16 *) (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
        if (param_3 == 0x3) {
            uStack20 =  ((u8) (- ((param_5 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
        } else {
            uStack20 = 0x1000;
        }
        uStack20 = (param_4 + 0x16) | uStack20;
        block_1000::mem_op_1000_131c(uStack20,
                                     CONCAT22(param_2,
                                  param_1));
        if ((uVar3 | uStack20) != 0x0) {
            puVar5 = (u16 *) mem_op_1000_0308(param_3,
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
                puVar1 = (u16 *) (param_4 + 0x1e);
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
        block_1000::pass1_1000_1e61(unaff_CS,
                                    0x7,
                                    param_4);
    }
    return 0x0;
}

pub fn mem_op_1000_0510(mut param_1: u16,
                        param_2: *mut Struct7) -> u32
{
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    u8 bVar3;
    let mut iVar4: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;
    let mut DVar12: u32;
    i32 lVar13;
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
        puVar1 = (u16 *) (iVar4 + 0x24);
        bVar11 = uVar10 < *puVar1;
        if ((bVar11 || uVar10 == *puVar1) && ((bVar11 || (uVar7 - uVar8 < (iVar4 + 0x22))))) {
            bVar11 = false;
            uVar9 = uVar10;
            goto LAB_1000_0595;
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
        return CONCAT22( ((u32) lVar13 >> 0x10),
                        0x1);
    }
    return (u32) uVar9 << 0x10;
}

pub fn mem_op_1000_05e2(mut param_1: u16,
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
    do {
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
        puVar1 = (u16 *) (param_4 + 0x1e);
        uVar4 = uVar3 + *puVar1;
        uVar3 = iVar2 +  (0xf000 < param_1 + 0x14) + (param_4 + 0x20) +  CARRY2(uVar3,
                                                                                          *puVar1);
        puVar1 = (u16 *) (param_4 + 0x28);
        bVar5 = uVar3 < *puVar1;
        // &DAT_1050_1050)
    } while (((bVar5 || uVar3 == *puVar1)
        && ((bVar5 || (puVar1 = (u16 *) (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) && ((uVar6 != 0x0
        || (UVar5 = block_1000::pass1_1000_1e61(unaff_CS,
                                                0x2,
                                                param_4), UVar5 != 0x0))));
    return 0x0;
}
