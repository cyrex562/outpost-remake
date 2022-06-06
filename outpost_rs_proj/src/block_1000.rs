//
// Created by cyrex on 2022-05-22.
//

use std::borrow::BorrowMut;
use crate::utils::CONCAT22;
use crate::globals::{DAT_1050_5f30, unaff_CS};
use crate::structs_2_h::Struct7;

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
        pass1_1000_1e61(unaff_CS,
                        0xa,
                        None);
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
                              mut param_4: u16,
                              mut param_5: u16,
                              mut unaff_cs: u16) -> u16
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
        param_4 = 0x0;
        uvar4 = 0xa;
        // code_r0x10000128:
        pass1_1000_1e61(unaff_CS,
                        uvar4,
                        param_4);
        return 0xffff;
    }
    DAT_1050_5f30 = 0x1;
    if param_1 == 0x1 {
        u_stack4 = 0x1;
        if (param_4 + 0x18) == 0x0 {
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
            pass1_1000_1e61(unaff_CS,
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
                uvar4 = pass1_1000_1e61(unaff_CS,
                                        0x2,
                                        param_1);
                if uvar4 == 0x0 {
                    return (bool) ('\x01' - ((param_1.field_0xa) == 0x0));
                }
                dvar6 = mem_op_1000_1532(param_1,
                                         0x1050);
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
    // pi_var2 = (i16 *) (param_1 + 0x20);
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

pub fn mem_op_1000_0308(mut param_1: i16, pstruct7_param_2: *mut astruct_7) -> *mut u8
{
    u8 *pu8_var1;
    let mut i16_var2: i16;
    let mut b_var3: bool;
    u8 extraout_AH;
    let mut pi16_var4: *mut i16;

    if ((pstruct7_param_2 + 0xa) == 0x0) {
        b_var3 = mem_op_1000_01b0(pstruct7_param_2);
        if (CONCAT11(extraout_AH,
                     b_var3) == 0x0) {
            return 0x0;
        }
    }
    pu8_var1 = (pstruct7_param_2.field_0xa);
    (pstruct7_param_2.field_0xa) = (pu8_var1 + 0x4);
    pi16_var4 = (i16 *) (param_1 * 0x2 + pstruct7_param_2);
    if (*pi16_var4 == 0x0) {
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
pub fn pass1_1000_0368(mut param_1: u16 ,
                     mut param_2: u16 ,
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

u16 mem_op_1000_03c6(mut param_1: u16 ,
                     mut param_2: i16,
                     mut param_3: u16 ,
                     mut param_4: u16 ,
                     u8 param_5,
                     mut param_6: u16 )
{
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut UVar6: u16;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut bVar8: bool;
    let mut DVar9: u32;
    let mut uVar10: u16;
    let mut uStack20: u16;
    let mut uVar9: u16;

    uVar7 = CONCAT11(param_6,
                     param_5);
    uVar3 = param_1 + 0xfff & 0xf000;
    puVar1 = (u16 *) (param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = param_2 +  (0xf000 < param_1) + (param_4 + 0x20) +  CARRY2(uVar3,
                                                                                 *puVar1);
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
        mem_op_1000_131c(uStack20,
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
                piVar2 = (i16 *) (param_4 + 0x20);
                *piVar2 = *piVar2 + param_2 +  CARRY2(uVar4,
                                                           UVar6);
                return uVar3;
            }
            mem_op_1000_13ce(uStack20,
                             uVar3);
        }
    } else {
        // &DAT_1050_1050
        pass1_1000_1e61(unaff_CS,
                        0x7,
                        param_4);
    }
    return 0x0;
}

u32 mem_op_1000_0510(mut param_1: u16 ,
                     param_2: *mut Struct7)
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
    bVar3 = *(u8 *) (param_2 + 0xc);
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
    // (i16 *)s_New_failed_in_Op__Op_1050_0020;
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
pub fn pass1_1000_05b4(u8 param_1,
                     param_2: *mut u16)
{
    *(param_2 + 0xa) = 0x1;
    *(param_2 + 0x8) = 0x668;
    *(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    *(param_2 + 0x10) = 0x0;
    *(param_2 + 0xe) = 0x0;
}

u32 mem_op_1000_05e2(mut param_1: u16 ,
                     mut param_2: i16,
                     mut param_3: u16 ,
                     param_4: *mut Struct7)
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
        || (UVar5 = pass1_1000_1e61(unaff_CS,
                                    0x2,
                                    param_4), UVar5 != 0x0))));
    return 0x0;
}

pub fn mem_1000_0668(void) -> u32
{
    let mut u_var1: u32;

    u_var1 = mem_op_1000_0510(0x0,
                              0x0);
    return u_var1;
}

u16 mem_1000_0670(mut param_1: u16 ,
                  param_2: *mut Struct7,
                  mut param_3: i16,
                  mut param_4: u16 ,
                  i16 *param_5)
{
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut UVar3: u16;
    let mut UVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    u32 *puVar9;
    let mut UVar10: u16;
    let mut BVar11: bool;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut unaff_CS: u16;
    let mut DVar14: u32;
    let mut DVar15: u32;
    let mut uStack16: u16;

    UVar3 = param_2;
    UVar4 = ( param_2 + 0x2);
    // &DAT_1050_1050
    DVar14 = mem_op_1000_1532(param_2,
                              0x1050);
    iVar6 = param_3 +  (0xffeb < param_1);
    uVar7 = *param_2;
    uVar8 = - ((param_4 & 0x1) != 0x0) & 0x100 | - ((param_4 & 0x4) != 0x0) & 0x400 | ( uVar7 + 0x16);
    if (param_5 == NULL) {
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
        do {
            uStack16 = uVar13;
            puVar9 = (u32 *) (uVar8 | 0x2000);
            // &DAT_1050_1050
            mem_op_1000_1408(puVar9,
                             CONCAT22(iVar6,
                                      param_1 + 0x14),
                             param_2,
                             0x1050);
            uVar13 = uStack16 | puVar9;
            if (uVar13 != 0x0) {
                break;
            }
            UVar10 = pass1_1000_1e61(unaff_CS,
                                     0x2,
                                     UVar3);
        } while (UVar10 != 0x0);
        if ((uStack16 |  puVar9) == 0x0) {
            ( param_5 + 0x2) = 0x0;
            *param_5 = 0x0;
            return 0x0;
        }
        (u32*) (iVar5 + 0x8) = puVar9;
        (iVar5 + 0xa) = uStack16;
        *param_5 =  (puVar9 + 0x5);
        ( param_5 + 0x2) = uStack16;
        param_2 = puVar9;
    }
    DVar15 = mem_op_1000_1532( param_2,
                              uStack16);
    uVar12 =  (DVar15 - DVar14);
    puVar1 = (u16 *) (UVar3 + 0x1e);
    uVar8 = *puVar1;
    *puVar1 = *puVar1 + uVar12;
    piVar2 = (i16 *) (UVar3 + 0x20);
    *piVar2 = *piVar2 +  (DVar15 - DVar14 >> 0x10) +  CARRY2(uVar8,
                                                                       uVar12);
    return 0x1;
}

u16 pass1_1000_0782(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: i16,
                    mut param_4: u16 )
{
    (param_3 + 0xe) = 0x0;
    (param_3 + 0x10) = param_3 + 0x14;
    (param_3 + 0x8) = 0x9a0;
    pass1_1000_07ac((param_1 + 0x18),
                    param_2,
                    param_3);
    return 0x1;
}
pub fn pass1_1000_07ac(mut param_1: u16 ,
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

struct astruct_99 *pass1_1000_07fc(mut param_1: u32)
{
    let mut unaff_CS: u16;
    struct astruct_99 *paVar1;

    if (( param_1 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS,
                        0xa,
                        0x0);
        return NULL;
    }
    paVar1 = (astruct_99 *) mem_op_1000_0838(0x0);
    return paVar1;
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
                    piVar2 = (i16 *) &PTR_LOOP_1050_000a;
                    *piVar2 = *piVar2 + 0x1;
                    *(i16 **) (param_1 + 0x2) = piVar9;
                    return CONCAT22(i_var3,
                                    pu_var4);
                }
                *piVar9 = 0x0;
            }
            piVar9 = (i16 *) piVar9[0x2];
        } while (piVar9 != piStack4);//
//        LAB_1000_085b:
        if ((param_1 + 0x18) == 0x0) {
            // &DAT_1050_1050
            pass1_1000_1e61(unaff_CS,
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
                uvar7 = pass1_1000_1e61(unaff_CS,
                                        0x2,
                                        param_1);
                u_var5 = u_stack6 & 0xfffe;
                if (uvar7 == 0x0) {
                    return 0x0;
                }
            }
        }
        piVar9 = *(i16 **) (param_1 + 0x2);
        piStack4 = (i16 *) piVar9[0x2];
    } loop;
}

u16 pass1_1000_093a(param_1: *mut astruct_611)
{
    let mut piVar1: *mut i16;
    let mut unaff_CS: u16;

    if (&PTR_LOOP_1050_000c != -0x352f) {
        pass1_1000_1e61(unaff_CS,
                        0xe,
                        0x0);
        return 0x0;
    }
    param_1 = (astruct_611 *) PTR_LOOP_1050_000e;
    if (param_1 == 0x0) {
        (u32) &u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000e = (astruct_611 *) param_1;
    piVar1 = (i16 *) &PTR_LOOP_1050_000a;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 == 0x0) {
        mem_op_1000_0510(0x1,
                         0x0);
    }
    return 0x1;
}

u8 *pass1_1000_09a0(param_1: *mut u16)
{
    u8 *puVar1;
    let mut uVar2: u32;

    *param_1 = PTR_LOOP_1050_000e;
    if (PTR_LOOP_1050_000e == NULL) {
        u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
    puVar1 = PTR_LOOP_1050_000e;
    PTR_LOOP_1050_000e = (u8 *) param_1;
    if (PTR_LOOP_1050_000a == NULL) {
        uVar2 = mem_op_1000_0510(0x1,
                                 0x0);
        puVar1 = (u8 *) uVar2;
    }
    return puVar1;
}

u16 pass1_1000_09ca(mut param_1: i16,
                    param_2: *mut u16)
{
    let mut puVar1: *mut u16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut puVar4: *mut u16;

    puVar1 = param_2 + 0xa;
    puVar4 = (u16 *) (( param_2 + (param_1 -  puVar1) + -0x6 & 0xfffcU) +  puVar1);
    *puVar4 = 0x1;
    param_2[0x7] =  puVar1;
    puVar4[0x2] =  puVar4;
    puVar4[0x1] =  puVar4;
    param_2[0x8] =  puVar4;
    if ((*(u8 *) (param_2 + 0x6) & 0x7) == 0x2) {
        param_2[0x9] = 0x8;
    } else {
        uVar3 = (u32) param_2;
        iVar2 = ( uVar3 + 0x18);
        param_2[0x9] = (iVar2 - 0x5U & ~- (iVar2 + 0x3U < 0x8)) + 0x8;
    }
    puVar4[-0x1] =  puVar4 -  puVar1;
    *puVar1 =  puVar4 -  puVar1 | 0x2;
    param_2[0xc] =  puVar4;
    param_2[0xb] = puVar4[0x1];
    (u16 *) (puVar4[0x1] + 0x4) = puVar1;
    puVar4[0x1] =  puVar1;
    param_2[0x4] = 0xe08;
    return *puVar1 & 0xfffc;
}

i32 mem_op_1000_0a48(u8 param_1,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     param_4: *mut Struct7)
{
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar4: u16;
    let mut uVar3: u16;
    //    let mut UVar4: u16;
    let mut unaff_CS: u16;
    let mut uVar5: u32;
    u8 in_stack_00000005;
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
                puVar2 = (u16 *) uVar5;
                if ((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
                    // (s_version__d__d_1050_0012 + 0x6)
                    uVar1 = 0x0018;
                    for (uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
                        puVar1 = puVar2;
                        puVar2 = puVar2 + 0x1;
                        *puVar1 = 0x0;
                    }
                    if ((uVar1 & 0x1) != 0x0) {
                        *(u8 *) puVar2 = 0x0;
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
    pass1_1000_1e61(unaff_CS,
                    0xa,
                    0x0);
    return 0x0;
}

u32 mem_op_1000_0b20(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
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
    uVar4 = uVar4 - 0x8 & ~- (uVar4 < 0x8);
    uVar5 = uVar4 + 0x8;
    pu16_var7 = (u16 *) (uVar2 * 0x2 + param_2);
    uStack20 = param_1;
    puStack6 = pu16_var7;
    if (pu16_var7 == NULL) {
        goto LAB_1000_0b64;
    }
    do {
        do {
            if ((uVar5 <= *pu16_var7) && (uVar10 = pass1_1000_0c32(uVar5,
                                                                   uStack20,
                                                                   0x0), uVar10 != 0x0)) {
                (u16 *) (uVar2 * 0x2 + param_2) = pu16_var7;
                return uVar10;
            }
            pu16_var7 = (u16 *) pu16_var7[0x2];
        } while (pu16_var7 != puStack6);//
//        LAB_1000_0b64:
        if (((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) || ((param_2 + 0x32) == 0x0))
            || (uVar3 = ((code) (param_2 + 0x32))(), uVar3 < uVar5)) {
            if (((uStack20 & 0x10) != 0x0) || (uVar3 = uVar2, UVar6 = mem_op_1000_03c6((param_2 + 0x1a),
                                                                                       0x0,
                                                                                       uVar2,
                                                                                       param_2,
                                                                                       0x0,
                                                                                       '\0'), (UVar6 | uVar3) == 0x0)) {
                if ((uStack20 & 0x20) == 0x0) {
                    uVar2 = uVar4 + 0x1007 & 0xf000;
                    puVar1 = (u16 *) (param_2 + 0x1e);
                    uVar4 = uVar2 + *puVar1;
                    uVar2 = (param_2 + 0x20) +  CARRY2(uVar2,
                                                            *puVar1);
                    puVar1 = (u16 *) (param_2 + 0x28);
                    bVar9 = uVar2 < *puVar1;
                    if ((bVar9 || uVar2 == *puVar1)
                        && ((bVar9 || (puVar1 = (u16 *) (param_2 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
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
        pu16_var7 = (u16 *) (uVar2 * 0x2 + param_2);
        puStack6 = (u16 *) pu16_var7[0x2];
    } loop;
}

u32 pass1_1000_0c32(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 )
{
    let mut puVar1: *mut u16;
    u8 *pbVar2;
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

    puVar8 = (u16 *) (param_3 + 0xe);
    uStack6 = 0x0;
    puVar6 = puVar8;
    loop {
        do {
            uVar5 = *puVar6;
            if (param_1 <= uVar5) {
                uVar5 = (uVar5 & 0xfffc) - param_1;
                puVar1 = (u16 *) (param_3 + 0x12);
                puStack8 = puVar6;
                if (*puVar1 < uVar5 || *puVar1 == uVar5) {
                    uStack14 = param_1;
                    if ((param_2 & 0x6) == 0x0) {
                        puStack8 = (u16 *) (uVar5 +  puVar6);
                        puStack8[-0x1] = uVar5;
                        *puVar6 = uVar5 | 0x2;
                        puVar8 = (u16 *) puVar6[0x1];
                        pbVar2 = (u8 *) ( puStack8 + param_1);
                        *pbVar2 = *pbVar2 | 0x2;
                        *puStack8 = param_1 | 0x1;
                    } else {
                        *puVar6 = param_1 & 0xff00 | *(u8 *) puVar6 & 0x2 | param_1 & 0xff | 0x1;
                        (puVar6[0x2] + 0x2) = puVar6[0x1];
                        (puVar6[0x1] + 0x4) = puVar6[0x2];
                        puVar8 = (u16 *) ( puVar6 + param_1);
                        ( puVar8 + (uVar5 - 0x2)) = uVar5;
                        *puVar8 = uVar5 | 0x2;
                        uVar5 = (param_3 + 0x10);
                        puVar8[0x2] = uVar5;
                        puVar8[0x1] = (uVar5 + 0x2);
                        (u16 *) ((uVar5 + 0x2) + 0x4) = puVar8;
                        (u16 *) (uVar5 + 0x2) = puVar8;
                    }
                } else {
                    puVar8 = (u16 *) puVar6[0x1];
                    (u16 *) (puVar6[0x2] + 0x2) = puVar8;
                    (puVar6[0x1] + 0x4) = puVar6[0x2];
                    puVar1 = puVar6;
                    *(u8 *) puVar1 = *(u8 *) puVar1 | 0x1;
                    uStack14 = *puVar6 & 0xfffc;
                    ( puVar6 + uStack14) = ( puVar6 + uStack14) | 0x2;
                }
                (u16 *) (param_3 + 0xe) = puVar8;
                if ((param_2 & 0x1) != 0x0) {
                    puVar6 = puStack8;
                    for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0; uVar5 -= 0x1) {
                        *puVar6 = 0x0;
                    }
                    if ((uStack14 - 0x2 & 0x1) != 0x0) {
                        *(u8 *) puVar6 = 0x0;
                    }
                }
                if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
                    *(u16 *) (param_3 + 0x4) = *(u16 *) ((param_3 + 0x10) + 0x2) & 0xfffc;
                    uVar4 = (u32) (param_3 + 0x4);
                    pbVar2 = (u8 *) ( uVar4 + 0x3);
                    *pbVar2 = *pbVar2 | 0x80;
                }
                piVar3 = (i16 *) (param_3 + 0xa);
                *piVar3 = *piVar3 + 0x1;
                return CONCAT22(0x1050,
                                puStack8 + 0x1);
            }
            if (uStack6 < uVar5) {
                uStack6 = uVar5;
            }
            puVar6 = (u16 *) puVar6[0x1];
        } while (puVar6 != puVar8);
        if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) {
            break;
        }
        uVar4 = (u32) param_3;
        uVar9 =  ((u32) uVar4 >> 0x10);
        iVar7 =  uVar4;
        if ((iVar7 + 0x34) == 0x0) {
            break;
        }
        uStack6 = ((code) (iVar7 + 0x34))();
        if ((uStack6 < param_1) || (puVar6 = (u16 *) (param_3 + 0xe), puVar6 == NULL)) {
            break;
        }
    }
    *(u16 *) (param_3 + 0x4) = uStack6 & 0xfffc;
    return 0x0;
}

BOOL16 call_fn_ptr_1000_0dc6(char *param_1)
{
//    let mut unaff_CS: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        pass1_1000_1e61(unaff_CS,
                        0xe,
                        0x0);
        return 0x0;
    }
    // &DAT_1050_1050
    (u16_1050_0008)(0x1050);
    return 0x1;
}

u16 pass1_1000_0e08(mut param_1: i16)
{
    let mut puVar1: *mut u16;
    u8 *pbVar2;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u16;
    let mut bVar6: bool;
    let mut uVar7: u32;

    puVar5 = (u16 *) (param_1 + -0x2);
    bVar6 = (*(u8 *) puVar5 & 0x2) != 0x0;
    if (bVar6) {
        puVar1 = puVar5;
        *(u8 *) puVar1 = *(u8 *) puVar1 & 0xfe;
    } else {
        puVar4 = (u16 *) ( puVar5 - (param_1 + -0x4));
        puVar1 = puVar4;
        *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
        puVar5 = puVar4;
    }
    puVar4 = (u16 *) ((*puVar5 & 0xfffc) +  puVar5);
    if ((*(u8 *) puVar4 & 0x1) == 0x0) {
        puVar1 = puVar5;
        *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
        if (puVar4 == (u16 *) PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e = (u8 *) puVar5;
        }
        (puVar4[0x2] + 0x2) = puVar4[0x1];
        (puVar4[0x1] + 0x4) = puVar4[0x2];
        puVar4 = (u16 *) ((*puVar5 & 0xfffc) +  puVar5);
    }
    puVar4[-0x1] = *puVar5 & 0xfffc;
    uVar3 = u32_1050_0004;
    puVar1 = puVar4 + -0x1;
    if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
        uVar3 = *puVar5 & 0xfffc;
        u32_1050_0004 = uVar3;
    }
    puVar1 = puVar4;
    *(u8 *) puVar1 = *(u8 *) puVar1 & 0xfd;
    if (bVar6) {
        if ((StructD *) PTR_LOOP_1050_0010.address_offset_field_0x2 != PTR_LOOP_1050_0010) {
            pbVar2 = (u8 *) ( u32_1050_0004 + 0x3);
            *pbVar2 = *pbVar2 & 0x7f;
        }
        puVar5[0x2] =  PTR_LOOP_1050_0010;
        uVar3 = PTR_LOOP_1050_0010.address_offset_field_0x2;
        puVar5[0x1] = uVar3;
        (u16 *) (PTR_LOOP_1050_0010.address_offset_field_0x2 + 0x4) = puVar5;
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

i32 pass1_1000_0ed4(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    param_4: *mut astruct_172,
                    param_5: *mut astruct_172)
{
    struct astruct_172 *paVar1;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    struct astruct_172 **ppaVar4;
    let mut uVar5: u16;
    let mut uVar6: u16;
    struct astruct_172 *paVar7;
    let mut puVar8: *mut u16;
    let mut unaff_CS: u16;
    i32 lVar9;
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
                                (u16 *) ppaVar4,
                                uVar6);
        if (uVar6 == 0x0) {
            return CONCAT22(param_5,
                            param_4);
        }
        if ((param_1 & 0x8) == 0x0) {
            lVar9 = mem_op_1000_0a48((u8) param_1,
                                     param_2,
                                     param_3,
                                     CONCAT22(UVar12,
                                              UVar11));
            uVar3 =  ((u32) lVar9 >> 0x10);
            puVar8 = (u16 *) lVar9;
            if (lVar9 != 0x0) {
                paVar7 = param_4;
                for (uVar5 = uVar6 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                    puVar2 = puVar8;
                    puVar8 = puVar8 + 0x1;
                    paVar1 = paVar7;
                    paVar7 = (astruct_172 *) &paVar7.field1_0x2;
                    *puVar2 = paVar1.field0_0x0;
                }
                for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
                    puVar2 = puVar8;
                    puVar8 = (u16 *) ( puVar8 + 0x1);
                    paVar1 = paVar7;
                    paVar7 = (astruct_172 *) ( &paVar7.field0_0x0 + 0x1);
                    *(u8 *) puVar2 = *(u8 *) &paVar1.field0_0x0;
                }
                call_fn_ptr_1000_0dc6((char *) CONCAT22(param_5,
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
    pass1_1000_1e61(unaff_CS,
                    UVar10,
                    UVar11);
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10001126)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1000_0fb8(mut param_1: u16 ,
                    mut param_2: i16,
                    mut param_3: u16 ,
                    mut param_4: u16 ,
                    param_5: *mut u16,
                    mut param_6: u16 )
{
    let mut puVar1: *mut u16;
    u8 bVar2;
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
        pass1_1000_1e61(unaff_CS,
                        0x4,
                         PTR_LOOP_1050_0000);
        if ((param_6 |  param_5) != 0x0) {
            param_5[0x1] = 0x0;
            *param_5 = 0x0;
            return 0x0;
        }
        return 0x1;
    }
    bVar2 = (u8) PTR_LOOP_1050_000c & 0x7;
    if (((u8) PTR_LOOP_1050_000c & 0x7) != 0x0) {
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
                uVar9 = pass1_1000_1284(CONCAT22(0x1050,
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
                                  (i16 *) CONCAT22(param_6,
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
    puVar8 = (u16 *) (param_2 + -0x2);
    uVar3 = *puVar8 & 0x7ffc;
    uStack4 = uVar3 - 0x2;
    if ((*(u8 *) (param_2 + -0x1) & 0x80) != 0x0) {
        uStack4 = uVar3 - 0x6;
    }
    if ((((param_3 == 0x0) && (param_1 <= uStack4)) || ((param_3 == 0x0 && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c)))))
        && (BVar4 = pass1_1000_115c(param_1,
                                    puVar8), BVar4 != 0x0)) {
        if ((param_4 & 0x1) != 0x0) {
            uVar3 = (*puVar8 & 0x7ffc) - 0x2;
            if (uStack4 < param_1) {
                puVar7 = (u16 *) (uStack4 + param_2);
                iVar5 = -uStack4;
            } else {
                if (uVar3 <= param_1) {
                    return 0x0;
                }
                puVar7 = (u16 *) (param_1 + param_2);
                iVar5 = -param_1;
            }
            uVar3 += iVar5;
            for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
                puVar1 = puVar7;
                puVar7 = puVar7 + 0x1;
                *puVar1 = 0x0;
            }
            if ((uVar3 & 0x1) != 0x0) {
                *(u8 *) puVar7 = 0x0;
            }
        }
        return 0x0;
    }
    return uStack4;
}

BOOL16 pass1_1000_115c(mut param_1: i16,
                       param_2: *mut u16)
{
    u8 *pbVar1;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut UVar6: u16;
    let mut uStack4: u16;

    uVar3 = *param_2 & 0x7ffc;
    uVar4 = param_1 + 0x5U & 0xfffc;
    uVar4 = (uVar4 - 0x8 & ~- (uVar4 < 0x8)) + 0x8;
    if (uVar3 < uVar4) {
        puVar5 = (u16 *) (uVar3 +  param_2);
        if (((*(u8 *) puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
            return 0x0;
        }
        if (puVar5 == (u16 *) PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e = (u8 *) puVar5[0x1];
        }
        (puVar5[0x2] + 0x2) = puVar5[0x1];
        (puVar5[0x1] + 0x4) = puVar5[0x2];
        uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
        if (uStack4 < s_version__d__d_1050_0012._0_2_) {
            puVar2 = param_2;
            *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
            pbVar1 = (u8 *) ( puVar5 + (*puVar5 & 0xfffc));
            *pbVar1 = *pbVar1 | 0x2;
            return 0x1;
        }
    } else {
        uStack4 = uVar3 - uVar4;
        if (uStack4 < s_version__d__d_1050_0012._0_2_) {
            return 0x1;
        }
        puVar5 = (u16 *) (uVar3 +  param_2);
        if ((*(u8 *) puVar5 & 0x1) == 0x0) {
            uStack4 += *puVar5 & 0xfffc;
            if (puVar5 == (u16 *) PTR_LOOP_1050_000e) {
                PTR_LOOP_1050_000e = (u8 *) puVar5[0x1];
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
    *(StructD * *)(UVar6 + 0x4) = PTR_LOOP_1050_0010;
    (UVar6 + 0x2) = PTR_LOOP_1050_0010.address_offset_field_0x2;
    (PTR_LOOP_1050_0010.address_offset_field_0x2 + 0x4) = UVar6;
    PTR_LOOP_1050_0010.address_offset_field_0x2 = UVar6;
    ((u8 *) (UVar6 + uStack4) + -0x2) = uStack4;
    pbVar1 = (u8 *) (UVar6 + uStack4);
    *pbVar1 = *pbVar1 & 0xfd;
    return 0x1;
}

pub fn pass1_1000_1284(mut param_1: u32) -> u32
{
    u8 bVar1;
    let mut u_var2: u16;
    let mut u_var3: u32;
    u8 bVar4;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut b_var6: bool;
    let mut DVar7: u32;
    let mut u_stack6: u16;
    let mut i_stack4: i16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        pass1_1000_1e61(unaff_CS,
                        0xe,
                        0x0);
        return 0xffffffff;
    }
    bVar1 = *(u8 *) &PTR_LOOP_1050_000c;
    bVar4 = bVar1 & 0x7;
    if ((bVar1 & 0x7) != 0x0) {
        if (bVar4 == 0x1) {
            u_var3 = *NULL;
            return (u32) ( u_var3 + 0x18);
        }
        if (bVar4 != 0x2) {
            if (bVar4 != 0x3) {
                return 0xffffffff;
            }
            DVar7 = get_mem_sz_1000_1532(0x0,
                                         param_1);
            return CONCAT22( (DVar7 >> 0x10) -  ( DVar7 < 0x14),
                             DVar7 - 0x14);
        }
    }
    u_var2 = ( param_1 + -0x2);
    u_var5 = u_var2 & 0x7ffc;
    u_stack6 = u_var5 - 0x2;
    i_stack4 = 0x0;
    if ((u_var2 & 0x8000) != 0x0) {
        b_var6 = u_stack6 < 0x4;
        u_stack6 = u_var5 - 0x6;
        i_stack4 = -b_var6;
    }
    return CONCAT22(i_stack4,
                    u_stack6);
}
pub fn mem_op_1000_131c(mut param_1: u16 ,
                      mut param_2: u32)
{
    HGLOBAL16 handle;
    let mut flags: u16;
    let mut b_var1: bool;
    i32 lVar2;
    let mut u_stack10: u16;
    let mut u_stack8: u16;
    let mut i_stack6: i16;

    lVar2 = CONCAT22(u_stack8,
                     u_stack10);
    flags = 0x32;
    i_stack6 = 0x1;
    if (((param_1 & 0x1000) != 0x0) && ((param_2 != 0x0 || (0xfff0 <  param_2)))) {
        param_2 = 0xfff0;
    }
    if ((param_1 & 0x100) != 0x0) {
        flags = 0x72;
    }
    if ((param_1 & 0x1) != 0x0) {
        flags |= 0x2000;
    }
    if ((param_1 & 0x4) != 0x0) {
        flags &= 0xfffd;
        lVar2 = mem_op_1000_1558( param_2,
                                 param_2);
    }
    do {
        handle = GLobalAlloc16(param_2 & 0xffff | (u32) param_2 << 0x10,
                               flags);
        u_stack8 =  ((u32) lVar2 >> 0x10);
        u_stack10 =  lVar2;
        if (handle != 0x0) {
            break;
        }
        flags &= 0xffcf;
        b_var1 = i_stack6 != 0x0;
        i_stack6 = i_stack6 + -0x1;
    } while (b_var1);
    if ((param_1 & 0x4) != 0x0) {
        if (handle != 0x0) {
            GlobalPageLock16(handle);
        }
        pass1_1000_15ce((u16 *) u_stack10,
                        u_stack8);
    }
    if (handle == 0x0) {
        return;
    }
    WIN16_GlobalLock16(handle);
    return;
}

i32 mem_op_1000_13ce(mut param_1: u16 ,
                     mut param_2: u16 )
{
    HGLOBAL16 HVar1;
    let mut uVar2: u16;
    let mut DVar3: u32;

    DVar3 = GlobalHandle16(param_2);
    uVar2 =  (DVar3 >> 0x10);
    if ((HGLOBAL16) DVar3 != 0x0) {
        HVar1 = GlobalFree16((HGLOBAL16) DVar3);
        return CONCAT22(uVar2,
                         (HVar1 == 0x0));
    }
    return (u32) uVar2 << 0x10;
}
pub fn mem_op_1000_1408(mut param_1: u16 ,
                      u32 re_alloc_size,
                      mut param_3: u16 ,
                      i16 selector)
{
    HGLOBAL16 handle;
    let mut global_handle_1: u32;
    let mut realloc_flags: u16;
    HGLOBAL16 global_handle_2;

    global_handle_1 = GlobalHandle16(selector);
    //  global_handle_1._0_2_ = (HGLOBAL16)global_handle_1;
    realloc_flags = 0x32;
    // (((param_1 & 0x1000) != 0x0) && ((re_alloc_size != 0x0 || (0xfff0 < re_alloc_size))))
    if (((param_1 & 0x1000) != 0x0) && ((re_alloc_size != 0x0 || 0xfff0 < re_alloc_size))) {
        re_alloc_size = 0xfff0;
    }
    if ((param_1 & 0x100) != 0x0) {
        realloc_flags = 0x72;
    }
    if ((param_1 & 0x804) != 0x0) {
        realloc_flags &= 0xfffd;
    }
    if ((HGLOBAL16) global_handle_1 != 0x0) {
        if ((param_1 & 0x4) != 0x0) {
            GlobalPageUnlock16((HGLOBAL16) global_handle_1);
        }
        do {
            global_handle_2 = (HGLOBAL16) global_handle_1;
            handle = GlobalReAlloc16(realloc_flags,
                                     re_alloc_size,
                                     (HGLOBAL16) global_handle_1);
            if (handle != 0x0) {
                break;
            }
            realloc_flags &= 0xffcf;
        } while (global_handle_2 != 0x0);
        if ((handle != 0x0) && ((param_1 & 0x4) != 0x0)) {
            GlobalPageLock16(handle);
        }
        if (handle != 0x0) {
            WIN16_GlobalLock16(handle);
            return;
        }
    }
    return;
}

BOOL16 mem_op_1000_14f2(mut param_1: u16 ,
                        mut param_2: u32,
                        param_4: *mut Struct7,
                        mut param_5: u16 )
{
    let mut in_AX: u16;
    let mut in_DX: u16;

    if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
        mem_op_1000_1408(param_1 & 0xfdff | 0x800,
                         CONCAT22(param_3,
                                  param_2),
                         param_4,
                         param_5);
        if ((in_DX | in_AX) != 0x0) {
            return 0x1;
        }
    }
    return 0x0;
}

pub fn get_mem_sz_1000_1532(param_1: *mut Struct7, selector: i16) -> u32
{
    let mut mem_size: u32;

    // get handle to global memory
    mem_size = GlobalHandle16(selector);
    if mem_size != 0x0 {
        // get size of memory area
        mem_size = GlobalSize16(mem_size);
        return mem_size;
    }
    return 0x0;
}

i32 mem_op_1000_1558(mut param_1: u16 ,
                     mut param_2: u16 )
{
    let mut uVar1: u16;
    let mut uVar3: u32;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uVar2: u16;

    uStack12 = 0x0;
    uStack10 = 0x0;
    uStack8 = 0x8;
    if ((param_2 < 0x9) && ((param_2 < 0x8 || (param_1 == 0x0)))) {
        do {
            loop {
                uVar3 = GlobalDOSAlloc16(CONCAT22(uStack8,
                                                  uStack10));
                uVar1 =  uVar3;
                if (uVar1 == 0x0) {
                    break;
                }
                *NULL = 0x0;
                &u16_1050_0002 = uStack12;
                uStack12 = uVar1;
            }
            uVar2 = uStack8 & 0x1;
            uStack8 >>= 0x1;
            uStack10 = uStack10 >> 0x1 |  (uVar2 != 0x0) << 0xf;
        } while ((param_2 < uStack8) || ((param_2 <= uStack8 && (param_1 <= uStack10))));
    }
    return (u32) uStack12 << 0x10;
}



// WARNING: Unable to use type for symbol uVar3
pub fn pass1_1000_15ce(param_1: *mut u16,
                     mut param_2: u16 )
{
    let mut pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut pu_var2: *mut u16;
    let mut u_var3: u16;

    u_var2 = param_2 |  param_1;
    while (u_var2 != 0x0) {
        pu_var1 = (u16 *) *param_1;
        u_var3 = param_1[0x1];
        GlobalDOSFree16(param_2);
        param_1 = pu_var1;
        param_2 = u_var3;
        u_var2 = u_var3 | pu_var1;
    }
    return;
}

u8 *mem_op_1000_160a(StructD *param_1)
{
    u8 *puVar1;
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar1 =  param_1;
    puVar1 = (u8 *) ret_true_1000_2146();
    if (puVar1 == NULL) {
        return puVar1;
    }
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        DAT_1050_5f30 = 0x1;
        DAT_1050_5f32 = 0x1;
        uVar2 = mem_op_1000_18ec(DAT_1050_5f46,
                                 uVar1);
        PTR_LOOP_1050_5f2e = (u8 *) (uVar2 >> 0x10);
        PTR_LOOP_1050_5f2c = (u8 *) uVar2;
        if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) != 0x0) {
            if (PTR_LOOP_1050_5f42 != NULL) {
                pass1_1000_1a54( PTR_LOOP_1050_5f42,
                                 PTR_LOOP_1050_5f2c,
                                 PTR_LOOP_1050_5f2e);
            }
            if (DAT_1050_5f44 != 0xffff) {
                pass1_1000_1afe(DAT_1050_5f44,
                                 PTR_LOOP_1050_5f2c,
                                 PTR_LOOP_1050_5f2e);
            }
        }
    }
    empty_fn_1000_214a();
    return PTR_LOOP_1050_5f2c;
}

u16 mem_1000_167a(mut param_1: u16 ,
                  mut param_2: u16 )
{
    u8 *puVar1;
    let mut in_register_0000000a: u16;
    StructD *pSVar2;
    i32 lVar3;

    pSVar2 = (StructD *) CONCAT22(in_register_0000000a,
                                  param_1);
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        puVar1 = mem_op_1000_160a(pSVar2);
        if (( pSVar2 |  puVar1) == 0x0) {
            return 0x0;
        }
    }
    lVar3 = mem_op_1000_0a48(0x0,
                             param_2,
                             0x0,
                             CONCAT22(PTR_LOOP_1050_5f2e,
                                      PTR_LOOP_1050_5f2c));
    return  lVar3;
}

u16 pass1_1000_16aa(mut param_1: u16 ,
                    param_2: *mut u16,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u16;
    i32 lVar2;

    if ((param_3 |  param_2) == 0x0) {
        uVar1 = mem_1000_167a(param_1,
                              param_4);
        return uVar1;
    }
    if (param_4 == 0x0) {
        pass1_1000_16ee( param_2,
                        param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0,
                            param_4,
                            0x0,
                            (astruct_172 *) param_2,
                            (astruct_172 *) param_3);
    return  lVar2;
}
pub fn pass1_1000_16ee(mut param_1: u16 ,
                     mut param_2: u16 )
{
    if ((param_2 | param_1) != 0x0) {
        call_fn_ptr_1000_0dc6((char *) CONCAT22(param_2,
                                                param_1));
    }
    return;
}

u16 fn_ptr_op_1000_1708(mut param_1: u16 ,
                        mut param_2: u16 ,
                        mut param_3: u16 ,
                        mut param_4: u16 ,
                        mut param_5: u16 )
{
    let mut iVar1: i16;
    let mut bVar2: bool;
    i32 lVar3;

    if ((param_2 | param_1) == 0x0) {
        bVar2 = 0xfffe < param_1;
        param_1 += 0x1;
        param_2 += bVar2;
    }//
//    LAB_1000_1724:
    do {
        if ((param_5 | param_4) != 0x0) {
            lVar3 = mem_op_1000_0a48((u8) param_3,
                                     param_1,
                                     param_2,
                                     CONCAT22(param_5,
                                              param_4));
            if (lVar3 != 0x0) {
                return  lVar3;
            }
        }
        if (((param_3 & 0x8000) == 0x0) || (( PTR_LOOP_1050_5f3a |  PTR_LOOP_1050_5f38) == 0x0)) {
            if (( PTR_LOOP_1050_5f36 |  PTR_LOOP_1050_5f34) == 0x0) {
                if (( PTR_LOOP_1050_5f3e |  PTR_LOOP_1050_5f3c) == 0x0) {
                    return 0x0;
                }
                ((code) PTR_LOOP_1050_5f3c)();
                goto LAB_1000_1724;
            }
            iVar1 = ((code) PTR_LOOP_1050_5f34)();
        } else {
            iVar1 = ((code) PTR_LOOP_1050_5f38)();
        }
        if (iVar1 == 0x0) {
            return 0x0;
        }
    } loop;
}
pub fn mem_op_1000_179c(mut param_1: i16,
                      param_2: *mut astruct_57)
{
    u8 *puVar1;
    u8 *puVar2;

    puVar1 = PTR_LOOP_1050_5f2c;
    puVar2 = PTR_LOOP_1050_5f2e;
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        puVar1 = mem_op_1000_160a((StructD *) param_2);
        puVar2 = (u8 *) param_2;
    }
    fn_ptr_op_1000_1708(param_1,
                        0x0,
                        0x0,
                         puVar1,
                         puVar2);
    return;
}
pub fn fn_ptr_1000_17ce(char *param_1)
{
    if (param_1 != NULL) {
        call_fn_ptr_1000_0dc6(param_1);
    }
    return;
}

u8 *pass1_1000_17e8(u8 *param_1,
                    u8 *param_2)
{
    u8 *puVar1;

    puVar1 = PTR_LOOP_1050_5f34;
    PTR_LOOP_1050_5f34 = param_1;
    PTR_LOOP_1050_5f36 = param_2;
    return puVar1;
}

u16 pass1_1000_180c(mut param_1: u16 ,
                    mut param_2: u16 )
{
    u8 *puVar1;
    let mut in_register_0000000a: u16;
    StructD *pSVar2;
    i32 lVar3;

    pSVar2 = (StructD *) CONCAT22(in_register_0000000a,
                                  param_1);
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        puVar1 = mem_op_1000_160a(pSVar2);
        if (( pSVar2 |  puVar1) == 0x0) {
            return 0x0;
        }
    }
    lVar3 = mem_op_1000_0a48(0x0,
                             param_2,
                             0x0,
                             CONCAT22(PTR_LOOP_1050_5f2e,
                                      PTR_LOOP_1050_5f2c));
    return  lVar3;
}

u16 pass1_1000_183c(mut param_1: u16 ,
                    mut param_2: u16 )
{
    let mut in_EDX: u32;
    StructD *pSVar1;
    i32 lVar2;

    pSVar1 = (StructD * )(in_EDX & 0xffff0000);
    if ( ((u32) param_2 * (u32) param_1 >> 0x10) != 0x0) {
        return 0x0;
    }
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar1);
        PTR_LOOP_1050_5f2e = (u8 *) pSVar1;
        if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
            return 0x0;
        }
    }
    lVar2 = mem_op_1000_0a48(0x1,
                              ((u32) param_2 * (u32) param_1),
                             0x0,
                             CONCAT22(PTR_LOOP_1050_5f2e,
                                      PTR_LOOP_1050_5f2c));
    return  lVar2;
}

u16 pass1_1000_188e(mut param_1: u16 ,
                    param_2: *mut u16,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u16;
    i32 lVar2;

    if ((param_3 |  param_2) == 0x0) {
        uVar1 = pass1_1000_180c(param_1,
                                param_4);
        return uVar1;
    }
    if (param_4 == 0x0) {
        pass1_1000_18d2( param_2,
                        param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0,
                            param_4,
                            0x0,
                            (astruct_172 *) param_2,
                            (astruct_172 *) param_3);
    return  lVar2;
}
pub fn pass1_1000_18d2(mut param_1: u16 ,
                     mut param_2: u16 )
{
    if ((param_2 | param_1) != 0x0) {
        call_fn_ptr_1000_0dc6((char *) CONCAT22(param_2,
                                                param_1));
    }
    return;
}

u32 mem_op_1000_18ec(mut param_1: u16 ,
                     mut param_2: u16 )
{
    let mut uVar1: u32;

    uVar1 = mem_op_1000_1902(param_2,
                             param_1,
                             0x0,
                             0x0,
                             0xc);
    return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 mem_op_1000_1902(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 ,
                     mut param_4: u16 ,
                     mut param_5: u16 )
{
    let mut pUVar1: *mut u16;
    let mut UVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar3: u16;
    let mut UVar5: u16;
    let mut pUVar6: *mut u16;
    let mut unaff_CS: u16;
    let mut DVar7: u32;
    let mut uVar8: u32;
    let mut puVar1: *mut u16;

    UVar5 = param_1;
    if (((param_2 & 0x8000) != 0x0) && (UVar5 = param_1, _SHI_INVOKEERRORHANDLER1 != -0x6f70)) {
        param_2 |= 0x1;
        UVar5 = param_1;
    }
    do {
        uVar3 = UVar5;
        pUVar1 = (u16 *) (param_2 & 0xfffb | 0x1000);
        mem_op_1000_131c( pUVar1,
                         0x100);
        UVar5 = uVar3 |  pUVar1;
        if (UVar5 != 0x0) {
            break;
        }
        UVar2 = pass1_1000_1e61(unaff_CS,
                                0x2,
                                0x0);
    } while (UVar2 != 0x0);
    if ((uVar3 |  pUVar1) != 0x0) {
        pUVar1[0x17] =  &PTR_PTR_1050_5f1a;
        pUVar1[0x18] =  &DAT_1050_1050;
        pUVar1[0x15] =  PTR_LOOP_1050_5f1e;
        pUVar1[0x16] =  PTR_LOOP_1050_5f20;
        pUVar6 = pUVar1;
        PTR_LOOP_1050_5f1e = (u8 *) pUVar1;
        PTR_LOOP_1050_5f20 = (u8 *) uVar3;
        for (iVar4 = 0x5; iVar4 != 0x0; iVar4 += -0x1) {
            puVar1 = pUVar6;
            pUVar6 = pUVar6 + 0x1;
            *puVar1 = 0x0;
        }
        pUVar1[0x5] = 0x0;
        pUVar1[0x7] = 0x0;
        pUVar1[0x6] = 0x0;
        pUVar1[0x9] = 0x0;
        pUVar1[0x8] = 0x0;
        pUVar1[0xa] = 0xbead;
        pUVar1[0xb] = param_2 & 0xfffd;
        pUVar1[0xc] = 0x0;
        pUVar1[0xd] = 0x2000;
        pUVar1[0xe] = 0x800;
        DVar7 = mem_op_1000_1532( pUVar1,
                                 uVar3);
        pUVar1[0xf] =  DVar7;
        pUVar1[0x10] =  (DVar7 >> 0x10);
        pUVar1[0x12] = 0x0;
        pUVar1[0x11] = 0x0;
        pUVar1[0x13] = 0xfffe;
        pUVar1[0x14] = 0xffff;
        pUVar1[0x19] = 0x0;
        pUVar1[0x1a] = 0x0;
        pUVar1[0x20] = 0x0;
        pUVar1[0x1f] = 0x0;
        BVar3 = pass1_1000_1afe(param_5,
                                 pUVar1,
                                uVar3);
        if (BVar3 != 0x0) {
            if ((param_4 | param_3) != 0x0) {
                pUVar6 = pUVar1;
                UVar5 = uVar3;
                uVar8 = pass1_1000_52be(param_3,
                                        param_4,
                                        param_5,
                                        0x0);
                pass1_1000_010c(0x1,
                                 uVar8,
                                 (uVar8 >> 0x10),
                                 pUVar6,
                                UVar5);
            }
            return CONCAT22(uVar3,
                            pUVar1);
        }
        mem_op_1000_1b9a(0x0,
                          pUVar1,
                         uVar3);
    }
    return 0x0;
}

u16 pass1_1000_1a54(mut param_1: u16 ,
                    mut param_2: i16,
                    mut param_3: u16 )
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut unaff_CS: u16;

    if ((param_2 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS,
                        0xa,
                        0x0);
        return 0x0;
    }
    uVar1 = pass1_1000_1ab0(param_1);
    if (uVar1 < (param_2 + 0x18) + 0x14U) {
        uVar2 = 0x0;
    } else {
        uVar2 = (param_2 + 0x1a);
        (param_2 + 0x1a) = uVar1;
        (param_2 + 0x1c) = uVar1 >> 0x2;
    }
    return uVar2;
}

u16 pass1_1000_1ab0(mut param_1: u16 )
{
    let mut uVar1: u16;
    let mut uVar2: u16;

    if (param_1 == 0x2000) {
        return 0x2000;
    }
    if (param_1 < 0xfff0) {
        if (param_1 < 0x1001) {
            return 0x1000;
        }
        uVar1 = 0x2000;
        if (param_1 < 0x2001) {
            do {
                uVar2 = uVar1;
                uVar1 = uVar2 >> 0x1;
            } while (param_1 <= uVar1);
            return uVar2 & 0xfffe;
        }
        while (uVar1 *= 0x2, uVar1 != 0x0) {
            if (param_1 <= uVar1) {
                return (uVar1 + 0x10 & - (uVar1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}

BOOL16 pass1_1000_1afe(mut param_1: u16 ,
                       mut param_2: u16 ,
                       mut param_3: u16 )
{
    let mut uVar1: u16;
    let mut unaff_CS: u16;

    if (param_1 == 0x0) {
        uVar1 = 0x0;
    } else {
        uVar1 = param_1 + 0x1 & 0xfffe;
    }
    if ((param_2 + 0x14) == -0x4153) {
        if ((uVar1 < param_1) || ((param_2 + 0x1a) - 0x14U < uVar1)) {
            pass1_1000_1e61(unaff_CS,
                            0x3,
                            param_2);
        } else if ((param_2 + 0x2) == 0x0) {
            (param_2 + 0x18) = uVar1;
            return 0x1;
        }
        return 0x0;
    }
    pass1_1000_1e61(unaff_CS,
                    0xa,
                    0x0);
    return 0x0;
}

u32 mem_op_1000_1b68(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    let mut unaff_CS: u16;
    let mut uVar1: u32;

    if ((param_2 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS,
                        0xa,
                        0x0);
        return (u32) param_1 << 0x10;
    }
    uVar1 = mem_op_1000_1b9a(0x0,
                             param_2,
                             param_3);
    return uVar1;
}

u32 mem_op_1000_1b9a(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    i32 lVar7;
    let mut puStack8: *mut u16;
    let mut uStack4: u16;

    (param_2 + 0x14) = 0x0;
    uStack4 = 0x0;
    do {
        iVar6 = (uStack4 * 0x2);
        if (iVar6 != 0x0) {
            do {
                uVar3 = (u32) (iVar6 + 0x8);
                ( uVar3 + 0xc) = 0x0;
                mem_op_1000_13ce((iVar6 + 0x8),
                                 (iVar6 + 0xa));
                iVar6 = (iVar6 + 0x4);
            } while ((uStack4 * 0x2) != iVar6);
        }
        uStack4 += 0x1;
    } while (uStack4 < 0x5);
    uVar4 = (param_2 + 0x10);
    uVar5 = (param_2 + 0x12);
    loop {
        puStack8 = (u16 *) CONCAT22(uVar5,
                                    uVar4);
        if ((uVar5 | uVar4) == 0x0) {
            break;
        }
        uVar1 = *puStack8;
        uVar2 = (uVar4 + 0x2);
        mem_op_1000_13ce(uVar4,
                         uVar5);
        uVar4 = uVar1;
        uVar5 = uVar2;
    }
    pass1_1000_20a2(param_2,
                    param_3);
    lVar7 = mem_op_1000_13ce(param_2,
                             param_3);
    return CONCAT22( ((u32) lVar7 >> 0x10),
                    0x1);
}

BOOL16 mem_op_1000_1dfa(mut param_1: i16,
                        u8 param_2,
                        mut param_3: u16 ,
                        mut param_4: u16 )
{
    let mut uVar1: u32;
    let mut uVar2: u16;

    if ((param_2 & 0x4) == 0x0) {
        uVar2 =  (u8) (((u8) (- ((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
    } else {
        uVar2 = 0x1800;
    }
    if ((param_4 == 0x0)
        || ((param_4 & 0xff00 &  (u8) (((u8) (- ((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8)
            != uVar2)) {
        return 0x1;
    }
    if (param_1 != 0x0) {
        uVar1 = SegmentLimit(param_4);
        if (CARRY2(param_3,
                   param_1 - 0x1U)) {
            return 0x1;
        }
        if ( uVar1 < param_3 + (param_1 - 0x1U)) {
            return 0x1;
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1000_1e61(mut param_1: u16,
                       mut param_2: u16,
                       param_3: Option<*mut Struct7>) -> u16
{
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut UVar3: u16;
    let mut UStack64: u16;
    let mut UStack62: u16;
    let mut UStack60: u16;
    code *pcStack6;
    u8 *puStack4;
    let mut uVar3: u16;

    // &DAT_1050_1050
    uVar3 =  0x1050;
    UStack62 = param_3;
    UStack60 = param_4;
    UStack64 = param_2;
    puStack4 = 0x1050 //(u8 *) &DAT_1050_1050;
    pcStack6 = (code *) &PTR_PTR_1050_5f1a;
    if ( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0x0 {
        pcStack6 = NULL;
        puStack4 = NULL;
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pcStack6 = (code *) PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0x0) {
            PTR_PTR_1050_5f1a = (u8 *) &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c = (u8 *) &PTR_LOOP_1050_1000;
            pcStack6 = (code *) &PTR_PTR_1050_1f7e;
            puStack4 = (u8 *) &PTR_LOOP_1050_1000;
        }
    }
    if (( puStack4 |  pcStack6) == 0x0) {
        return 0x0;
    }
    BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                  &DAT_1050_1050,
                                 0x0);
    if (BVar2 == 0x0) {
        u16_var3 = (*pcStack6)(0x1000,
                               &UStack64,
                               &DAT_1050_1050,
                               uVar3);
    } else {
        puStack4 = NULL;
        pcStack6 = NULL;
        u16_var3 = 0x0;
    }
    if (( puStack4 |  pcStack6) != 0x0) {
        pass1_1000_1f68();
    }
    return u16_var3;
}

u16 _SHI_INVOKEERRORHANDLER1(void)
{
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut uVar2: u16;
    code *pcStack6;
    let mut puStack4: *mut u16;
    let mut uVar3: u16;

    puStack4 = (u16 *) &DAT_1050_1050;
    if (( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0x0) {
        pcStack6 = NULL;
        puStack4 = NULL;
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pcStack6 = (code *) PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0x0) {
            PTR_PTR_1050_5f1a = (u8 *) &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c = (u8 *) &PTR_LOOP_1050_1000;
            pcStack6 = (code *) &PTR_PTR_1050_1f7e;
            puStack4 = (u16 *) &PTR_LOOP_1050_1000;
        }
    }
    if (( puStack4 |  pcStack6) != 0x0) {
        BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                      &DAT_1050_1050,
                                     0x0);
        if (BVar2 == 0x0) {
            uVar2 = (*pcStack6)();
        } else {
            puStack4 = NULL;
            pcStack6 = NULL;
            uVar2 = 0x0;
        }
        if (( puStack4 |  pcStack6) != 0x0) {
            pass1_1000_1f68();
        }
        return uVar2;
    }
    return 0x0;
}

BOOL16 msg_box_op_1000_1f24(mut param_1: i16,
                            mut param_2: u16 ,
                            mut param_3: u16 )
{
    let mut piVar1: *mut i16;
    let mut unaff_CS: u16;

    if (param_3 < (param_1 + 0xc)) {
        msg_box_op_1000_214c(0x0,
                             0x0,
                             0xd940,
                              &PTR_LOOP_1050_1040);
        return 0x1;
    }
    piVar1 = (i16 *) (param_1 + 0xc);
    *piVar1 = *piVar1 + 0x1;
    return 0x0;
}
pub fn pass1_1000_1f68()
{
    PTR_LOOP_1050_5f26 -= 1;
    if (PTR_LOOP_1050_5f26 < 0x0) {
        PTR_LOOP_1050_5f26 = 0;
    }
}



// WARNING: Removing unreachable block (ram,0x10001f92)

BOOL16 pass1_1000_1f7e(param_1: *mut u16)
{
    char cVar1;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar4: i16;
    char *pcVar5;

    uVar3 = *param_1;
    if (uVar3 == 0xf) {//
//        LAB_1000_1fb6:
        iVar4 = 0x1;
    } else {
        if (uVar3 < 0x10) {
            cVar1 = (char) uVar3;
            if (cVar1 == '\x02') {
                goto LAB_1000_1fb6;
            }
            if (('\0' < (char) (cVar1 + -0x2)) && ((char) (cVar1 + -0x3) < '\f')) {
                iVar4 = 0x0;
                goto LAB_1000_1fbe;
            }
        }
        iVar4 = 0x0;
        uVar3 = 0x1;
    }//
//    LAB_1000_1fbe:
    pcVar5 = pass1_1000_1fd2(uVar3);
    BVar2 = msg_box_op_1000_214c(0x0,
                                 iVar4,
                                  pcVar5,
                                  ((u32) pcVar5 >> 0x10));
    return BVar2;
}

char *pass1_1000_1fd2(mut param_1: i16)
{
    if (param_1 == 0x2) {
        return "Out of memory.  Please free some memory, then choose retry.";
    }
    return (char *) CONCAT22(0x1000,
                             param_1 * 0x17 + 0x1c7a);
}



// WARNING: Removing unreachable block (ram,0x10002018)

BOOL16 pass1_1000_1fea(void)
{
    u8 *puVar1;
    let mut bVar2: bool;

    puVar1 = PTR_LOOP_1050_5f22 + 0x1;
    bVar2 = PTR_LOOP_1050_5f22 == NULL;
    PTR_LOOP_1050_5f22 = puVar1;
    if ((bVar2) && (( PTR_LOOP_1050_5f20 |  PTR_LOOP_1050_5f1e) != 0x0)) {
        PTR_LOOP_1050_5f22 = (u8 *) &u16_1050_0002;
    }
    return 0x1;
}
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
            mem_op_1000_1b9a(0x1,
                             *(u16 *) (param_2 + 0x4),
                             (param_2 + 0x6));
        }
        u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}

BOOL16 pass1_1000_206c(mut param_1: u16 ,
                       mut param_2: u16 )
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
pub fn pass1_1000_20a2(mut param_1: u16 ,
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

u16 ret_true_1000_2146()
{
    return 0x1;
}
pub fn empty_fn_1000_214a()
{
    return;
}

BOOL16 msg_box_op_1000_214c(mut param_1: u16 ,
                            mut param_2: i16,
                            mut param_3: u16 ,
                            mut param_4: u16 )
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
                fatal_app_exit_1000_3e9e();
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

bool mem_op_1000_21b6(mut param_1: u16 ,
                      mut param_2: u16 )
{
    let mut BVar1: bool;

    BVar1 = mem_op_1000_1dfa(0x0,
                             0x4,
                             param_1,
                             param_2);
    return BVar1 == 0x0;
}



// WARNING: Removing unreachable block (ram,0x100021de)

u16 pass1_1000_21d2(u8 param_1,
                    i32 param_2,
                    mut param_3: u16 ,
                    mut param_4: u16 ,
                    u8 param_5)
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

u32 pass1_1000_2242(mut param_1: u16 ,
                    u8 *param_2,
                    mut param_3: u16 ,
                    mut param_4: u16 ,
                    mut param_5: u16 ,
                    mut param_6: i16)
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

BOOL16 pass1_1000_22c0(mut param_1: u16 ,
                       mut param_2: u16 ,
                       mut param_3: u16 ,
                       mut param_4: u16 ,
                       mut param_5: u16 )
{
    let mut u32_var1: u32;

    u32_var1 = pass1_1000_2242(param_2,
                               (u8 *) 0x1dfa,
                               param_1,
                               param_3,
                               param_4,
                               param_5);
    if (u32_var1 == 0x0) {
        return 0x1;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000234c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn init_1000_23be(mut param_1: u16 ,
                    mut param_2: u16 )
{
    init_op_1008_54aa( &DAT_1050_1050,
                      param_1,
                      param_2,
                      PTR_LOOP_1050_5f52,
                      CONCAT22(PTR_LOOP_1050_5f50,
                               PTR_LOOP_1050_5f4e),
                      PTR_LOOP_1050_5f4a,
                      (u8 *) HINSTANCE16_1050_5f4c);
    return;
}


/*
Unable to decompile 'dos3_call_1000_23ea'
Cause:
Low-level Error: Symbol $$undef0000001e extends beyond the end of the address space
*/


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
    ret_op_1000_55ac();
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
pub fn pass1_1000_25a8(void)
{
    pass1_1000_2913(0xfc);
    pass1_1000_2913(0xff);
    return;
}

/*
Unable to decompile 'exit_1000_25cc'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/


i16 *pass1_1000_25d2(mut param_1: i16,
                     mut param_2: i16,
                     code2 fn_ptr_param_3,
                     mut param_4: i16)
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

    puVar3 = (u8 *) (param_1 + 0x1U & 0xfffe);
    if ((puVar3 < &param_2)
        && (pstruct_d_var4 = (StructD * ) - ( puVar3 -  &param_2), PTR_LOOP_1050_000a <= pstruct_d_var4.address_offset_field_0x0)) {
        if (pstruct_d_var4.address_offset_field_0x0 < PTR_LOOP_1050_000c) {
            PTR_LOOP_1050_000c = pstruct_d_var4.address_offset_field_0x0;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
        // WARNING: Treating indirect jump as call
        piVar5 = (i16 *) fn_ptr_param_3();
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
    piVar5 = (i16 *) &PTR_LOOP_1050_63fe;
    do {
        piVar1 = piVar5;
        piVar5 = piVar5 + 0x1;
        iVar7 = *piVar1;
        piVar8 = piVar5;
        if ((iVar7 == param_4) || (piVar8 = (i16 *) (iVar7 + 0x1), piVar8 == NULL)) {
            return piVar8;
        }
        iVar7 = -0x1;
        do {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            piVar1 = piVar5;
            piVar5 = (i16 *) ( piVar5 + 0x1);
        } while (*(char *) piVar1 != '\0');
    } loop;
}


/*
Unable to decompile 'exit_1000_25f2'
Cause:
Low-level Error: Overlapping input varnodes
*/


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram
pub fn pass1_1000_262c(u8 *param_1,
                     u8 *param_2,
                     u8 *param_3)
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
    param_3 = (u8 *) 0x263d;
    param_2 = (u8 *) pass1_1000_2950(0x8,
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
    PTR_LOOP_1050_5fb8 = (u8 *) ( &PTR_LOOP_1050_0000 + 0x1);
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
    param_3 = (u8 *) &DAT_1050_1050;
    i_var3 = -( (PTR_LOOP_1050_5fb8 +  (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var4 + 0x1) & 0xfffe);
    PTR_LOOP_1050_5fba = &stack0x0004 + i_var3;
    PTR_LOOP_1050_5fbc = (u8 *) &DAT_1050_1050;
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
    PTR_LOOP_1050_5fbe = (u8 *) ppuVar6;
    PTR_LOOP_1050_5fc0 = (u8 *) pu_var8;
    do {
        if (i_var9 == 0x0) {
            *ppuVar6 = NULL;
            ppuVar6[0x1] = NULL;
            return;
        }
        b_var13 = *piVar9 == s__C_FILE_INFO__1050_5f5c._0_2_;
        if (b_var13) {
            piVar12 = (i16 *) s__C_FILE_INFO__1050_5f5c;
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
            piVar9 = (i16 *) ( piVar9 + 0x1);
            cVar4 = *(char *) piVar2;
            pu_var4 = pu_var5;
            pu_var5 = (u16 *) ( pu_var5 + 0x1);
            *(char *) pu_var4 = cVar4;
        } while (cVar4 != '\0');
        i_var9 += -0x1;
    } while (true);
}

char * poss_str_op_1000_28dc(param_1: *mut astruct_825)
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
        } while (*(char *) ppaVar1 != '\0');
    } loop;
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

u16_t *pass1_1000_2950(mut param_1: i16,
                       mut param_2: u16 ,
                       mut param_3: u16 )
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
    PTR_LOOP_1050_6066 = (u8 *) &PTR_LOOP_1050_1000;
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
        } while (*(char *) puVar1 != '\0');
    } loop;
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
    cVar1 = *(char *) (u32) ((param_1 & 0xff) + 0x5fd6);//
//    LAB_1000_29d2:
    PTR_LOOP_1050_5f78 = (u8 *)  cVar1;
    return;
}

u16_t pass1_1000_29dc(mut param_1: u16 )
{
    if (___EXPORTEDSTUB != (code) 0xb8) {
        return (u16_t) & DAT_1050_1050;
    }
    return uRam100029ed;
}

u16_t ___EXPORTEDSTUB(void)
{
    return 0x0;
}

u16 pass1_1000_2a00(param_1: *mut u16)
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
    if ((*(u8 *) (param_1 + 0x5) & 0x40) != 0x0) {
        *(u8 *) (param_1 + 0x5) = 0x0;
        return 0xffff;
    }
    if ((*(u8 *) (param_1 + 0x5) & 0x83) == 0x0) {
        goto LAB_1000_2af2;
    }
    uVar5 = pass1_1000_2fa4((i16 *) param_1);
    uStack6 = param_1[0x7a];
    pass1_1000_2cb0(param_1);
    uVar1 =  *(u8 *) ( param_1 + 0xb);
    if ( u16_1050_5f8a <  uVar1) {
        piVar3 = pass1_1000_55b1(unaff_CS,
                                 uVar1);
        if ( piVar3 < 0x0) {
            goto LAB_1000_2a6a;
        }//
//        LAB_1000_2a82:
        bVar2 = false;
    } else {
        uVar4 = dos3_call_op_1000_35fe( *(u8 *) ( param_1 + 0xb),
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
        unk_str_op_1000_3d3e((char *) CONCAT22(0x1050,
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
                        (u8 *) CONCAT22(0x1050,
                                        puStack20),
                        0xa);
        uVar4 = dos3_call_1000_514e();
        if (uVar4 == 0x0) {
            goto LAB_1000_2af2;
        }
    }
    uVar5 = 0xffff;//
//    LAB_1000_2af2:
    *(u8 *) (param_1 + 0x5) = 0x0;
    return uVar5;
}

u16 *pass1_1000_2b02(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 ,
                     mut param_4: u16 ,
                     mut param_5: u16 ,
                     u8 param_6)
{
    let mut puVar1: *mut u16;

    puVar1 = pass1_1000_35aa();
    if ((param_1 |  puVar1) == 0x0) {
        puVar1 = NULL;
    } else {
        puVar1 = pass1_1000_2d34(param_2,
                                 param_3,
                                 (u8 *) CONCAT22(param_5,
                                                 param_4),
                                 param_6,
                                 puVar1);
    }
    return puVar1;
}
pub fn pass1_1000_2b3c(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 ,
                     mut param_4: u16 ,
                     mut param_5: u16 ,
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

u16 pass1_1000_2b5c(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = pass1_1000_2e74((u16 *) param_1);
    uVar2 = FUN_1000_30b4();
    pass1_1000_2f00(uVar1,
                    (i16 *) param_1);
    return uVar2;
}
pub fn pass1_1000_2ba0(uchar param_1)
{
    pass1_1000_3024();
    if (u8_1050_5fc9 != '\0') {
        pass1_1000_3f5c();
    }
    return;
}

u16 mem_1000_2bb6(mut param_1: u16 ,
                  mut param_2: u16 ,
                  i16 *param_3)
{
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut piVar3: *mut i16;
    u8 bVar4;
    u8 *puVar5;
    u8 *puVar6;
    u8 *puVar7;

    piVar3 = param_3;
    bVar4 = *(u8 *) (param_3 + 0x5);
    if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
        param_3[0x2] = 0x0;
        if ((bVar4 & 0x1) != 0x0) {
            if ((bVar4 & 0x10) == 0x0) {
                goto LAB_1000_2c37;
            }
            *param_3 = param_3[0x3];
            bVar4 &= 0xfe;
        }
        *(u8 *) (param_3 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7 = (u8 *)  *(u8 *) ( param_3 + 0xb);
        if (((bVar4 & 0x8) == 0x0) && (((bVar4 & 0x4) != 0x0 || (((*(u8 *) (param_3 + 0x78) & 0x1) == 0x0 && ((
            (u16_1050_61ec != 0x0
                && (((param_3 == (i16 *) 0x621c || (param_3 == (i16 *) 0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0))))
                || (mem_1000_2ce8(param_1,
                                  param_3), (*(u8 *) (piVar3 + 0x5) & 0x8) == 0x0)))))))) {
            puVar5 = mixed_dos3_call_1000_39f2(puVar7,
                                               (char *) CONCAT22(0x1050,
                                                                 &param_2),
                                               (u8 *) ( &PTR_LOOP_1050_0000 + 0x1));
            puVar6 = (u8 *) ( &PTR_LOOP_1050_0000 + 0x1);
        } else {
            iVar2 = piVar3[0x3];
            puVar6 = (u8 *) (*piVar3 - iVar2);
            *piVar3 = iVar2 + 0x1;
            piVar3[0x2] = piVar3[0x79] + -0x1;
            if (puVar6 == NULL) {
                puVar5 = NULL;
                if ((puVar7[0x5f90] & 0x20) != 0x0) {
                    mixed_dos3_call_1000_3636( puVar7,
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
    *(u8 *) piVar1 = *(u8 *) piVar1 | 0x20;
    return 0xffff;
}
pub fn pass1_1000_2cb0(param_1: *mut u16)
{
    let mut pu_var1: *mut u16;
    u8 bVar2;

    bVar2 = *(u8 *) (param_1 + 0x5);
    if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
        pass1_1000_16ee(param_1[0x3],
                        param_1[0x4]);
        pu_var1 = param_1 + 0x5;
        *(u8 *) pu_var1 = *(u8 *) pu_var1 & 0xf7;
        param_1[0x3] = 0x0;
        param_1[0x4] = 0x0;
        *param_1 = 0x0;
        param_1[0x1] = 0x0;
        param_1[0x2] = 0x0;
    }
    return;
}
pub fn mem_1000_2ce8(mut param_1: u16 ,
                   i16 *param_2)
{
    let mut piVar1: *mut i16;
    let mut u_var2: u16;

    u_var2 = mem_1000_167a(param_1,
                           0x200);
    if (param_1 == 0x0) {
        piVar1 = param_2 + 0x5;
        *(u8 *) piVar1 = *(u8 *) piVar1 | 0x4;
        param_2[0x79] = 0x1;
        param_1 =  &DAT_1050_1050;
        u_var2 =  param_2 + 0xf1;
    } else {
        piVar1 = param_2 + 0x5;
        *(u8 *) piVar1 = *(u8 *) piVar1 | 0x8;
        param_2[0x79] = 0x200;
    }
    param_2[0x1] = param_1;
    *param_2 = u_var2;
    param_2[0x4] = param_1;
    param_2[0x3] = u_var2;
    param_2[0x2] = 0x0;
    return;
}

u16 *pass1_1000_2d34(mut param_1: u16 ,
                     mut param_2: u16 ,
                     u8 *param_3,
                     u8 param_4,
                     param_5: *mut u16)
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
    param_3 = (u8 *) ((u32) param_3 & 0xffff0000 | (u32) ( param_3 + 0x1));
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
        *(u8 *) (param_5 + 0x5) = uStack6;
        param_5[0x1] = 0x0;
        *param_5 = 0x0;
        param_5[0x4] = 0x0;
        param_5[0x3] = 0x0;
        uStack14 = (u8) uVar4;
        *(u8 *) ( param_5 + 0xb) = uStack14;
        *(u8 *) (param_5 + 0x78) = bStack8;
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

u16 pass1_1000_2e74(param_1: *mut u16)
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
            if (((*(u8 *) (param_1 + 0x5) & 0xc) == 0x0) && ((*(u8 *) puVar5 & 0x1) == 0x0)) {
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
                *(u8 *) puVar1 = *(u8 *) puVar1 | 0x2;
                *(u8 *) puVar5 = 0x11;
                return 0x1;
            }
        } else if ((u8) u16_1050_5f8a <= *(u8 *) ( param_1 + 0xb)) {
            puVar1 = puVar5;
            *(u8 *) puVar1 = *(u8 *) puVar1 | 0x10;
        }
    }
    return 0x0;
}
pub fn pass1_1000_2f00(mut param_1: i16,
                     i16 *param_2)
{
    if (((*(u8 *) (param_2 + 0x78) & 0x10) != 0x0)
        && ((*(u8 *) (*(u8 *) ( param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)) {
        pass1_1000_2fa4(param_2);
        if (param_1 != 0x0) {
            *(u8 *) (param_2 + 0x78) = 0x0;
            param_2[0x79] = 0x0;
            *param_2 = 0x0;
            param_2[0x1] = 0x0;
            param_2[0x3] = 0x0;
            param_2[0x4] = 0x0;
        }
    }
    return;
}

u16 pass1_1000_2f48(i32 param_1)
{
    let mut uVar1: u16;
    u8 *puVar2;

    if (param_1 == 0x0) {
        uVar1 = pass1_1000_3038(0x0);
    } else {
        uVar1 = pass1_1000_2fa4((i16 *) param_1);
        if (uVar1 == 0x0) {
            if ((*(u8 *) ((i16 *) param_1 + 0x78) & 0x40) != 0x0) {
                puVar2 = pass1_1000_400a( *(u8 *) ( (i16 *) param_1 + 0xb));
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
    bVar2 = *(u8 *) (param_1 + 0x5);
    if (((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((*(u8 *) (param_1 + 0x78) & 0x1) != 0x0)))) {
        puVar4 = (u8 *) (*param_1 - param_1[0x3]);
        if (0x0 <  puVar4) {
            puVar5 = mixed_dos3_call_1000_39f2((u8 *)  *(u8 *) ( param_1 + 0xb),
                                               (char *) CONCAT22(param_1[0x4],
                                                                 param_1[0x3]),
                                               puVar4);
            if (puVar5 == puVar4) {
                if ((*(u8 *) (param_1 + 0x5) & 0x80) != 0x0) {
                    piVar1 = param_1 + 0x5;
                    *(u8 *) piVar1 = *(u8 *) piVar1 & 0xfd;
                }
            } else {
                piVar1 = param_1 + 0x5;
                *(u8 *) piVar1 = *(u8 *) piVar1 | 0x20;
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
pub fn pass1_1000_3024(void)
{
    pass1_1000_3038(0x1);
    return;
}

i16 pass1_1000_3038(mut param_1: i16)
{
    let mut uVar1: u16;
    u8 *puVar2;
    let mut iVar3: i16;
    let mut iStack4: i16;

    iVar3 = 0x0;
    iStack4 = 0x0;
    for (puVar2 = (u8 *) &PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
        if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
            uVar1 = pass1_1000_2f48(CONCAT22(0x1050,
                                             puVar2));
            if (uVar1 != 0xffff) {
                iVar3 += 0x1;
            }
        } else if ((param_1 == 0x0) && (((puVar2[0xa] & 0x2) != 0x0 && (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,
                                                                                                         puVar2)), uVar1
            == 0xffff)))) {
            iStack4 = -0x1;
        }
    }
    if (param_1 == 0x1) {
        iStack4 = iVar3;
    }
    return iStack4;
}

u16 FUN_1000_30b4(void)
{
    u8 bVar1;
    u8 bVar2;
    let mut unaff_BP: i16;
    let mut iVar3: i16;
    let mut unaff_SI: u16;
    let mut unaff_CS: u16;
    u8 *in_stack_00000008;
    let mut uVar4: u16;

    iVar3 = unaff_BP + 0x1;
    uVar4 = SUB42(&DAT_1050_1050,
                  0x0);
    exit_1000_25f2(0x214,
                   0x30c5,
                   unaff_CS,
                    &DAT_1050_1050);
    bVar1 = *in_stack_00000008;
    if (bVar1 == 0x0) {
        return 0x0;
    }
    if ((u8) (bVar1 - 0x20) < 0x59) {
        bVar2 = *(u8 *) (u32) ((u8) (bVar1 - 0x20) + 0x5ffe) & 0xf;
    } else {
        bVar2 = 0x0;
    }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
    uVar4 = (code) ((char) (*(u8 *) (u32) ((u8) (bVar2 * '\b') + 0x5ffe) >> 0x4) * 0x2 + 0x30a4))
    (unaff_SI & 0xff00 |  bVar1, uVar4, iVar3);
    return uVar4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3113(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    FUN_1000_3552();
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_311e(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    (unaff_BP + -0x12) = 0x0;
    (unaff_BP + -0xc) = 0x0;
    (unaff_BP + -0x14) = 0x0;
    (unaff_BP + -0x6) = 0x20;
    (unaff_BP + -0xe) = 0xffff;
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3134(void)
{
    u8 *pbVar1;
    char cVar2;
    char *pcVar3;
    u8 bVar4;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = *(char *) (unaff_BP + -0x4);
    if (cVar2 == '-') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x4;
    } else if (cVar2 == '+') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x1;
    } else if (cVar2 == ' ') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
    } else if (cVar2 == '#') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x80;
    } else {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = *(char **) (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar2 - 0x20U) < 0x59) {
            bVar4 = *(u8 *) (u32) ((u8) (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *(u8 *) (u32) ((u8) (bVar4 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (code) ((char) bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3168(void)
{
    u8 *pbVar1;
    char cVar2;
    char *pcVar3;
    u8 bVar4;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = *(char *) (unaff_BP + -0x4);
    if (cVar2 == '*') {
        uVar5 = pass1_1000_34cf();
        if ( uVar5 < 0x0) {
            uVar5 = -uVar5;
            pbVar1 = (u8 *) (unaff_BP + -0x6);
            *pbVar1 = *pbVar1 | 0x4;
        }
    } else {
        uVar5 = (unaff_BP + -0xc) * 0xa +  (u8) (cVar2 - 0x30);
    }
    (unaff_BP + -0xc) = uVar5;
    pcVar3 = *(char **) (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar2 - 0x20U) < 0x59) {
            bVar4 = *(u8 *) (u32) ((u8) (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *(u8 *) (u32) ((u8) (bVar4 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (code) ((char) bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3194(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    (unaff_BP + -0xe) = 0x0;
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_319c(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    cVar1 = *(char *) (unaff_BP + -0x4);
    if (cVar1 == '*') {
        uVar4 = pass1_1000_34cf();
        if ( uVar4 < 0x0) {
            uVar4 = 0xffff;
        }
    } else {
        uVar4 = (unaff_BP + -0xe) * 0xa +  (u8) (cVar1 - 0x30);
    }
    (unaff_BP + -0xe) = uVar4;
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_31c5(void)
{
    u8 *pbVar1;
    char cVar2;
    char *pcVar3;
    u8 bVar4;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = *(char *) (unaff_BP + -0x4);
    if (cVar2 == 'l') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'F') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x20;
    } else if (cVar2 == 'N') {
        pbVar1 = (u8 *) (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'L') {
        pbVar1 = (u8 *) (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x4;
    } else {
        pbVar1 = (u8 *) (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = *(char **) (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar2 - 0x20U) < 0x59) {
            bVar4 = *(u8 *) (u32) ((u8) (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *(u8 *) (u32) ((u8) (bVar4 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (code) ((char) bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_31f7(mut param_1: u16 )
{
    let mut piVar1: *mut i16;
    u8 *pbVar2;
    let mut puVar3: *mut u16;
    char cVar4;
    char *pcVar5;
    u8 bVar6;
    let mut uVar7: u16;
    let mut iVar8: i16;
    char *pcVar9;
    let mut uVar10: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: u16;
    let mut extraout_DX_04: u16;
    let mut unaff_BP: i16;
    let mut unaff_DI: *mut u16;
    let mut puVar11: *mut u16;
    let mut unaff_ES: i16;
    u8 in_AF;
    let mut bVar12: bool;
    let mut uVar13: u32;

    cVar4 = *(char *) (unaff_BP + -0x4);
    if ((cVar4 == 'd') || (cVar4 == 'i')) {
        pbVar2 = (u8 *) (unaff_BP + -0x6);
        *pbVar2 = *pbVar2 | 0x40;//
//        LAB_1000_3399:
        *(u8 *) (unaff_BP + -0x8) = 0xa;//
//        LAB_1000_33d4:
        if ((*(u8 *) (unaff_BP + -0x6) & 0x10) == 0x0) {
            uVar7 = pass1_1000_34cf();
            if ((*(u8 *) (unaff_BP + -0x6) & 0x40) == 0x0) {
                uVar13 = (u32) uVar7;
            } else {
                uVar13 = (u32)  uVar7;
            }
        } else {
            uVar13 = pass1_1000_34d8();
        }
        if (((*(u8 *) (unaff_BP + -0x6) & 0x40) != 0x0) && ((long) uVar13 < 0x0)) {
            pbVar2 = (u8 *) (unaff_BP + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
            uVar13 = CONCAT22(-( (uVar13 >> 0x10) +  ( uVar13 != 0x0)),
                              - uVar13);
        }
        param_1 =  (uVar13 >> 0x10);
        if ((unaff_BP + -0xe) < 0x0) {
            (unaff_BP + -0xe) = 0x1;
        } else {
            pbVar2 = (u8 *) (unaff_BP + -0x6);
            *pbVar2 = *pbVar2 & 0xf7;
        }
        if (uVar13 == 0x0) {
            (unaff_BP + -0x12) = 0x0;
        }
        unaff_DI = (u16 *) (unaff_BP + -0x17);
        unaff_ES =  &DAT_1050_1050;
        pcVar9 = (char *)  *(u8 *) (unaff_BP + -0x8);
        pass1_1000_356e( uVar13,
                         pcVar9,
                        param_1);
        if (((*(u8 *) (unaff_BP + -0x5) & 0x2) != 0x0) && ((pcVar9 == NULL || (*(char *) unaff_DI != '0')))) {
            unaff_DI = (u16 *) (unaff_BP + -0x18);
            *(char *) unaff_DI = '0';
            pcVar9 = pcVar9 + 0x1;
        }
    } else {
        if (cVar4 == 'u') {
            goto LAB_1000_3399;
        }
        if (cVar4 == 'X') {
            *(u8 *) (unaff_BP + -0x3) = 0x7;//
//            LAB_1000_33a9:
            if ((*(u8 *) (unaff_BP + -0x6) & 0x80) != 0x0) {
                (unaff_BP + -0x12) = 0x2;
                *(u8 *) (unaff_BP + -0x10) = 0x30;
                *(char *) (unaff_BP + -0xf) = *(char *) (unaff_BP + -0x3) + 'Q';
            }
            *(u8 *) (unaff_BP + -0x8) = 0x10;
            goto LAB_1000_33d4;
        }
        if (cVar4 == 'x') {
            *(u8 *) (unaff_BP + -0x3) = 0x27;
            goto LAB_1000_33a9;
        }
        if (cVar4 == 'o') {
            if ((*(u8 *) (unaff_BP + -0x6) & 0x80) != 0x0) {
                pbVar2 = (u8 *) (unaff_BP + -0x5);
                *pbVar2 = *pbVar2 | 0x2;
            }
            *(u8 *) (unaff_BP + -0x8) = 0x8;
            goto LAB_1000_33d4;
        }
        if (cVar4 == 'c') {
            uVar7 = pass1_1000_34cf();
            unaff_ES =  &DAT_1050_1050;
            *(u8 *) (unaff_BP + -0x216) = (char) uVar7;
            unaff_DI = (u16 *) (unaff_BP + -0x216);
            pcVar9 = (char *) 0x1;
        } else if (cVar4 == 's') {
            uVar13 = pass1_1000_34e6(param_1);
            param_1 =  (uVar13 >> 0x10);
            if ((unaff_DI == NULL) && (unaff_ES == 0x0)) {
                unaff_ES =  &DAT_1050_1050;
                unaff_DI = (u16 *) 0x6057;
                pcVar9 = DAT_1050_605d;
            } else {
                iVar8 = (unaff_BP + -0xe);
                puVar11 = unaff_DI;
                if (iVar8 != 0x0) {
                    bVar12 = true;
                    do {
                        if (iVar8 == 0x0) {
                            break;
                        }
                        iVar8 += -0x1;
                        puVar3 = puVar11;
                        puVar11 = (u16 *) ( puVar11 + 0x1);
                        bVar12 = *(char *) puVar3 == '\0';
                    } while (!bVar12);
                    if (bVar12) {
                        puVar11 = (u16 *) ( puVar11 + -0x1);
                    }
                }
                pcVar9 = (char *) ( puVar11 -  unaff_DI);
            }
        } else {
            if (cVar4 == 'n') {
                pass1_1000_34e6(param_1);
                *unaff_DI = (unaff_BP + -0xa);
                if ((*(u8 *) (unaff_BP + -0x6) & 0x10) != 0x0) {
                    unaff_DI[0x1] = 0x0;
                }
                goto LAB_1000_30cf;
            }
            if (cVar4 != 'p') {
                if ((cVar4 == 'E') || (cVar4 == 'G')) {
                    piVar1 = (i16 *) (unaff_BP + -0x14);
                    *piVar1 = *piVar1 + 0x1;
                }
                pbVar2 = (u8 *) (unaff_BP + -0x6);
                *pbVar2 = *pbVar2 | 0x40;
                bVar6 = *(u8 *) (unaff_BP + -0x4) | 0x20;
                iVar8 = (unaff_BP + -0xe);
                if (iVar8 < 0x1) {
                    if (iVar8 == 0x0) {
                        if (bVar6 == 0x67) {
                            (unaff_BP + -0xe) = 0x1;
                        }
                    } else {
                        (unaff_BP + -0xe) = 0x6;
                    }
                }
                unaff_DI = (u16 *) (unaff_BP + -0x216);
                if ((*(u8 *) (unaff_BP + -0x5) & 0x4) == 0x0) {
                    ((code) PTR_s_3_wav_1050_25cc_1050_6068)();
                    piVar1 = (i16 *) (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0x8;
                    param_1 = extraout_DX_00;
                } else {
                    ((code) PTR_s_3_wav_1050_25cc_1050_607c)();
                    piVar1 = (i16 *) (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0xa;
                    param_1 = extraout_DX;
                }
                if (((*(u8 *) (unaff_BP + -0x6) & 0x80) != 0x0) && ((unaff_BP + -0xe) == 0x0)) {
                    ((code) PTR_s_3_wav_1050_25cc_1050_6074)();
                    param_1 = extraout_DX_01;
                }
                if ((bVar6 == 0x67) && (((unaff_BP + -0x6) & 0x80) == 0x0)) {
                    ((code) PTR_s_3_wav_1050_25cc_1050_6070)();
                    param_1 = extraout_DX_02;
                }
                unaff_ES =  &DAT_1050_1050;
                if (*(char *) unaff_DI == '-') {
                    unaff_DI = (u16 *) (unaff_BP + -0x215);
                    pbVar2 = (u8 *) (unaff_BP + -0x5);
                    *pbVar2 = *pbVar2 | 0x1;
                }
                iVar8 = -0x1;
                puVar11 = unaff_DI;
                do {
                    if (iVar8 == 0x0) {
                        break;
                    }
                    iVar8 += -0x1;
                    puVar3 = puVar11;
                    puVar11 = (u16 *) ( puVar11 + 0x1);
                } while (*(char *) puVar3 != '\0');
                pcVar9 = (char *) ( puVar11 + (-0x1 -  unaff_DI));
                goto LAB_1000_3444;
            }
            if ((*(u8 *) (unaff_BP + -0x6) & 0x30) == 0x0) {
                uVar7 = pass1_1000_34cf();
                uVar13 = (u32) uVar7;//
//                LAB_1000_32d8:
                *(u8 *) (unaff_BP + -0x3) = 0x7;
                param_1 = 0x0;
                pass1_1000_356e( uVar13,
                                0x10,
                                0x0);
                pcVar9 = (char *) 0x4;
            } else {
                uVar13 = pass1_1000_34d8();
                uVar10 =  (uVar13 >> 0x10);
                if ((*(u8 *) (unaff_BP + -0x5) & 0x18) != 0x0) {
                    goto LAB_1000_32d8;
                }
                *(u8 *) (unaff_BP + -0x3) = 0x7;
                pass1_1000_356e( uVar13,
                                0x10,
                                0x0);
                param_1 = 0x0;
                pass1_1000_356e(uVar10,
                                0x10,
                                0x0);
                *(u8 *) (unaff_BP + -0x212) = 0x3a;
                pcVar9 = (char *) 0x9;
            }
            unaff_ES =  &DAT_1050_1050;
            unaff_DI = (u16 *) (unaff_BP + -0x216);
        }
    }//
//    LAB_1000_3444:
    if ((*(u8 *) (unaff_BP + -0x6) & 0x40) != 0x0) {
        if ((*(u8 *) (unaff_BP + -0x5) & 0x1) == 0x0) {
            if ((*(u8 *) (unaff_BP + -0x6) & 0x1) == 0x0) {
                if ((*(u8 *) (unaff_BP + -0x6) & 0x2) != 0x0) {
                    *(u8 *) (unaff_BP + -0x10) = 0x20;
                    (unaff_BP + -0x12) = 0x1;
                }
            } else {
                *(u8 *) (unaff_BP + -0x10) = 0x2b;
                (unaff_BP + -0x12) = 0x1;
            }
        } else {
            *(u8 *) (unaff_BP + -0x10) = 0x2d;
            (unaff_BP + -0x12) = 0x1;
        }
    }
    if ((*(u8 *) (unaff_BP + -0x6) & 0xc) == 0x0) {
        FUN_1000_3552(pcVar9,
                      unaff_DI,
                      unaff_ES);
        param_1 = extraout_DX_03;
    }
    pass1_1000_3534(in_AF,
                    (unaff_BP + -0x12),
                    param_1);
    if (((*(u8 *) (unaff_BP + -0x6) & 0x8) != 0x0) && ((*(u8 *) (unaff_BP + -0x6) & 0x4) == 0x0)) {
        FUN_1000_3552(pcVar9,
                      unaff_DI,
                      unaff_ES);
        param_1 = extraout_DX_04;
    }
    pass1_1000_3534(in_AF,
                    pcVar9,
                    param_1);
    if ((*(u8 *) (unaff_BP + -0x6) & 0x4) != 0x0) {
        FUN_1000_3552();
    }//
//    LAB_1000_30cf:
    pcVar5 = *(char **) (unaff_BP + 0xa);
    cVar4 = *pcVar5;
    (unaff_BP + 0xa) =  pcVar5 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar4;
    if ((cVar4 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar4 - 0x20U) < 0x59) {
            bVar6 = *(u8 *) (u32) ((u8) (cVar4 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar6 = 0x0;
        }
        bVar6 = *(u8 *) (u32) ((u8) (bVar6 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar6;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar7 = (code) ((char) bVar6 * 0x2 + 0x30a4))();
        return uVar7;
    }
    return (unaff_BP + -0xa);
}

u16 pass1_1000_34cf(void)
{
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut unaff_BP: i16;

    puVar2 = (u16 *) (unaff_BP + 0xe);
    uVar1 = *puVar2;
    (unaff_BP + 0xe) =  puVar2 + 0x2;
    return uVar1;
}

pub fn pass1_1000_34d8(void) -> u32
{
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut pu_var3: *mut u16;
    let mut unaff_bp: i16;

    pu_var3 = (u16 *) (u32) (unaff_bp + 0xe);
    u_var1 = *pu_var3;
    u_var2 = ( pu_var3 + 0x2);
    (unaff_bp + 0xe) =  pu_var3 + 0x4;
    return CONCAT22(u_var2,
                    u_var1);
}

pub fn pass1_1000_34e6(mut param_1: u16 ) -> u32
{
    let mut u_var1: u16;
    let mut unaff_bp: i16;
    let mut u_var2: u32;

    if ((*(u8 *) (unaff_bp + -0x6) & 0x20) != 0x0) {
        u_var2 = pass1_1000_34d8();
        return u_var2;
    }
    u_var1 = pass1_1000_34cf();
    if (u_var1 == 0x0) {
        return (u32) param_1 << 0x10;
    }
    return CONCAT22(param_1,
                    u_var1);
}

i16 pass1_1000_3503(mut param_1: u16 ,
                    mut param_2: u16 )
{
    let mut piVar1: *mut i16;
    char *pcVar2;
    char **ppcVar3;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut unaff_BP: i16;
    let mut uVar6: u16;

    ppcVar3 = (char **) *(i16 **) (unaff_BP + 0x6);
    uVar6 =  ((u32) ppcVar3 >> 0x10);
    piVar5 = (i16 *) ppcVar3;
    piVar1 = piVar5 + 0x2;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
        uVar4 = mem_1000_2bb6(param_2,
                               (char) param_1,
                              piVar5);
        if (uVar4 == 0xffff) {
            return -0x1;
        }
    } else {
        pcVar2 = *ppcVar3;
        *ppcVar3 = *ppcVar3 + 0x1;
        *pcVar2 = (char) param_1;
    }
    return 0x0;
}
pub fn pass1_1000_3534(undefined1 param_1,
                     mut param_2: i16,
                     mut param_3: u16 )
{
    let mut piVar1: *mut i16;
    u8 *pbVar2;
    let mut in_ax: u16;
    let mut unaff_bp: i16;
    u8 *unaff_DI;
    let mut u_var3: u16;
    let mut unaff_es: u16;

    if (param_2 != 0x0) {
        piVar1 = (i16 *) (unaff_bp + -0xa);
        *piVar1 = *piVar1 + param_2;
        u_var3 = 0x0;
        do {
            pbVar2 = unaff_DI;
            unaff_DI = unaff_DI + 0x1;
            in_ax = pass1_1000_3503(in_ax & 0xff00 |  *pbVar2,
                                    param_3);
            u_var3 |= in_ax;
            param_2 += -0x1;
        } while (param_2 != 0x0);
        if (u_var3 != 0x0) {
            (unaff_bp + -0xa) = 0xffff;
        }
    }
    return;
}
pub fn FUN_1000_3552(void)
{
    let mut piVar1: *mut i16;
    mut param_3: u16 ;
    let mut param_1: i16;
    mut param_2: u16 ;
    let mut unaff_bp: i16;
    let mut u_var2: u16;

    if (param_1 != 0x0) {
        piVar1 = (i16 *) (unaff_bp + -0xa);
        *piVar1 = *piVar1 + param_1;
        u_var2 = 0x0;
        do {
            param_3 = pass1_1000_3503(param_3 & 0xff00 | param_2 & 0xff,
                                      param_2);
            u_var2 |= param_3;
            param_1 += -0x1;
        } while (param_1 != 0x0);
        if (u_var2 != 0x0) {
            (unaff_bp + -0xa) = 0xffff;
        }
    }
    return;
}
pub fn pass1_1000_356e(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    u8 *pbVar1;
    let mut u_var2: u32;
    u8 bVar3;
    let mut unaff_bp: i16;
    let mut unaff_si: i16;
    u8 *unaff_DI;
    let mut unaff_es: u16;

    while (((0x0 < unaff_si || (param_1 != 0x0)) || (param_3 != 0x0))) {
        u_var2 = (u32) param_3;
        param_3 /= param_2;
        u_var2 = u_var2 % (u32) param_2 << 0x10 | (u32) param_1;
        param_1 =  (u_var2 / param_2);
        bVar3 = (char) (u_var2 % (u32) param_2) + 0x30;
        if (0x39 < bVar3) {
            bVar3 += *(char *) (unaff_bp + -0x3);
        }
        pbVar1 = unaff_DI;
        unaff_DI = unaff_DI + -0x1;
        *pbVar1 = bVar3;
        unaff_si += -0x1;
    }
    return;
}

u16 *pass1_1000_35aa(void)
{
    let mut puVar1: *mut u16;

    puVar1 = (u16 *) &PTR_LOOP_1050_6210;
    loop {
        if (PTR_LOOP_1050_5ff0 < puVar1) {
            return NULL;
        }
        if ((*(u8 *) (puVar1 + 0x5) & 0x83) == 0x0) {
            break;
        }
        puVar1 = puVar1 + 0x6;
    }
    *(u8 *) (puVar1 + 0x5) = 0x0;
    puVar1[0x2] = 0x0;
    puVar1[0x4] = 0x0;
    puVar1[0x3] = 0x0;
    puVar1[0x1] = 0x0;
    *puVar1 = 0x0;
    *(u8 *) ( puVar1 + 0xb) = 0xff;
    return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10003622)

u16 dos3_call_op_1000_35fe(mut param_1: u16 ,
                           mut param_2: i16)
{
//    code *pcVar1;
    let mut uVar2: u16;
    let mut u16_var2: bool;

    if (param_1 < u16_1050_5f8a) {
        u16_var2 = false;
        code7 fn_ptr_1 = (code7) swi(0x21);
        uVar2 = fn_ptr_1(param_2 + 0x1);
        if (!u16_var2) {
            *(u8 *) (param_1 + 0x5f90) = 0x0;
        }
    } else {
        uVar2 = 0x900;
        u16_var2 = true;
    }
    if (u16_var2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)
pub fn mixed_dos3_call_1000_3636(mut param_1: u16 ,
                               mut param_2: u16 ,
                               mut param_3: u16 ,
                               mut param_4: u16 )
{
    u8 *pbVar1;
    code *pcVar2;
    let mut u_var3: u16;
    let mut unaff_bp: i16;
    let mut i_var4: i16;
    let mut b_var5: bool;
    let mut u_var6: u32;

    i_var4 = unaff_bp + 0x1;
    if (((param_1 < u16_1050_5f8a) || (u16_1050_61ec == 0x0)) || (0x2 < param_1)) {
        if ((u16_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0x0)) {
            goto LAB_1000_36e3;
        }
        if (param_4 == 0x0) {
            goto LAB_1000_369b;
        }
        b_var5 = false;
        pcVar2 = (code *) swi(0x21);
        u_var6 = (*pcVar2)();
        u_var3 = u_var6;
        if (b_var5) {
            goto LAB_1000_299d;
        }
        if ((param_4 & 0x2) == 0x0) {
            if (-0x1 <  ( ((u32) u_var6 >> 0x10) + param_3 +  CARRY2(u_var3,
                                                                     param_2))) {//
//                LAB_1000_36e3:
                bVar5 = false;
                pcVar2 = (code *) swi(0x21);
                uVar3 = (*pcVar2)(iVar4);
                if (!bVar5) {
                    pbVar1 = (u8 *) (param_1 + 0x5f90);
                    bVar5 = false;
                    *pbVar1 = *pbVar1 & 0xfd;
                }
                goto LAB_1000_299d;
            }
        } else {
            pcVar2 = (code *) swi(0x21);
            u_var6 = (*pcVar2)();
            if (-0x1 <  ( ((u32) u_var6 >> 0x10) + param_3 +  CARRY2(u_var6,
                                                                     param_2))) {
                goto LAB_1000_36e3;
            }
            pcVar2 = (code *) swi(0x21);
            (*pcVar2)();
        }//
//        LAB_1000_369b:
        u_var3 =  s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    b_var5 = true;//
//    LAB_1000_299d:
    if (b_var5) {
        pass1_1000_29b5(u_var3);
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10003989)
// WARNING: Removing unreachable block (ram,0x100038a1)
// WARNING: Removing unreachable block (ram,0x10003867)
// WARNING: Removing unreachable block (ram,0x10003967)
// WARNING: Removing unreachable block (ram,0x1000391a)
// WARNING: Removing unreachable block (ram,0x10003803)
// WARNING: Removing unreachable block (ram,0x100037b7)
// WARNING: Removing unreachable block (ram,0x10003799)
// WARNING: Removing unreachable block (ram,0x10003765)
// WARNING: Removing unreachable block (ram,0x100037ec)
// WARNING: Removing unreachable block (ram,0x100038d9)
// WARNING: Removing unreachable block (ram,0x100038f2)
// WARNING: Removing unreachable block (ram,0x1000393a)
// WARNING: Removing unreachable block (ram,0x1000384b)
// WARNING: Removing unreachable block (ram,0x1000388b)
// WARNING: Removing unreachable block (ram,0x100038ba)
// WARNING: Removing unreachable block (ram,0x100039b9)
// WARNING: Removing unreachable block (ram,0x1000381c)
// WARNING: Could not reconcile some variable overlaps

u16 mixed_dos3_call_1000_370a(mut param_1: u16 ,
                              mut param_2: u16 ,
                              mut param_3: u16 ,
                              mut param_4: u16 ,
                              u8 param_5,
                              mut param_6: u16 )
{
    code *fn_ptr_1;
    let mut uVar3: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    u8 bVar2;
    let mut uVar7: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut unaff_BP: i16;
    let mut uVar6: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    let mut uStack14: u16;
    u8 bVar9;
    let mut in_stack_0000fffa: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    u8 cVar12;

    uVar6 = unaff_BP + 0x1;
    param_5 = param_6;
    uVar3 = param_1 & 0xff00;
    param_1 = uVar3 | param_5;
    uVar9 = 0x0;
    if (((param_4 & 0x8000) == 0x0) && (((param_4 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0x0)))) {
        uVar9 = 0x80;
    }
    bVar10 = false;
    fn_ptr_1 = (code *) swi(0x21);
    uVar7 = param_4;
    uVar2 = (*fn_ptr_1)();
    if (bVar10) {
        if ((uVar2 == 0x2) && ((uVar7 & 0x100) != 0x0)) {
            uVar10 = uVar9 & 0xff;
            // s_____s__lu_1050_38cd + 0x3
            param_1 =  0x38d0;
            pass1_1000_39e1();
            uVar7 = 0x0;
            _param_5 = param_6;//
//            LAB_1000_38e3:
            bVar10 = false;
            fn_ptr_1 = (code *) swi(0x21);
            uVar2 = (*fn_ptr_1)();
            if (bVar10) {
                goto LAB_1000_299d;
            }
            if (((char) param_1 != '\0') || (uVar5 = uVar2, uVar8 = uStack14, (param_4 & 0x2) == 0x0)) {
                fn_ptr_1 = (code *) swi(0x21);
                (*fn_ptr_1)();
                bVar10 = false;
                fn_ptr_1 = (code *) swi(0x21);
                uVar2 = (*fn_ptr_1)();
                if (bVar10) {
                    goto LAB_1000_299d;
                }
                uVar5 = uVar2;
                uVar8 = param_1;
                if (((uVar10 & 0x100) == 0x0) && (uVar8 = param_1, (_param_5 & 0x1) != 0x0)) {
                    uVar7 =  (u8) ((u8) uVar7 | 0x1);
                    bVar10 = false;
                    fn_ptr_1 = (code *) swi(0x21);
                    uVar4 = uVar2;
                    uVar2 = (*fn_ptr_1)();
                    uVar5 = uVar4;
                    uVar8 = uVar6;
                    if (bVar10) {
                        goto LAB_1000_299d;
                    }
                }
            }//
//            LAB_1000_3973:
            bVar9 = (u8) uVar10;
            if ((uVar10 & 0x40) == 0x0) {
                fn_ptr_1 = (code *) swi(0x21);
                (*fn_ptr_1)();
                bVar2 = 0x0;
                if ((uVar7 & 0x1) != 0x0) {
                    bVar2 = 0x10;
                }
                if ((param_4 & 0x8) != 0x0) {
                    bVar2 |= 0x20;
                }
            } else {
                bVar2 = 0x0;
            }
            if (uVar5 < &u16_1050_5f8a) {
                *(u8 *) (uVar5 + 0x5f90) = bVar2 | bVar9 | 0x1;
                return uVar5;
            }
            fn_ptr_1 = (code *) swi(0x21);
            (*fn_ptr_1)();
            uVar2 = 0x1800;
        }
    } else {
        if ((uVar7 & 0x500) != 0x500) {
            uVar10 = CONCAT11(0x1,
                              (char) uVar9);
            fn_ptr_1 = (code *) swi(0x21);
            (*fn_ptr_1)();
            if ((extraout_DX & 0x80) != 0x0) {
                uVar10 |= 0x40;
            }
            uVar5 = uVar2;
            uVar8 = param_1;
            if ((uVar10 & 0x40) == 0x0) {
                if ((param_4 & 0x200) == 0x0) {
                    if (((uVar10 & 0x80) != 0x0) && ((param_4 & 0x2) != 0x0)) {
                        code fn_ptr_1 = (code) swi(0x21);
                        (fn_ptr_1)();
                        code5 fn_ptr_2 = (code5) swi(0x21);
                        iVar3 = (fn_ptr_2)();
                        if ((iVar3 != 0x0) && (param_1 = (char) (uVar3 >> 0x8), param_1 == '\x1a')) {
                            fn_ptr_1 = (code *) swi(0x21);
                            (*fn_ptr_1)(uVar6);
                            fn_ptr_1 = (code *) swi(0x21);
                            (*fn_ptr_1)();
                        }
                        uVar7 = 0x0;
                        fn_ptr_1 = (code *) swi(0x21);
                        (*fn_ptr_1)();
                        uVar5 = uVar2;
                        uVar8 = uStack14;
                    }
                } else {
                    if ((param_4 & 0x3) == 0x0) {
                        fn_ptr_1 = (code *) swi(0x21);
                        (*fn_ptr_1)();
                        fn_ptr_1 = (code *) swi(0x21);
                        (*fn_ptr_1)();
                        goto LAB_1000_38e3;
                    }
                    uVar7 = 0x0;
                    fn_ptr_1 = (code *) swi(0x21);
                    (*fn_ptr_1)();
                    uVar5 = uVar2;
                    uVar8 = param_1;
                }
            }
            goto LAB_1000_3973;
        }
        fn_ptr_1 = (code *) swi(0x21);
        (*fn_ptr_1)();
        uVar2 = 0x1100;
    }
    bVar10 = true;//
//    LAB_1000_299d:
    if (bVar10) {
        pass1_1000_29b5(uVar2);
        uVar2 = 0xffff;
    }
    return uVar2;
}
pub fn pass1_1000_39e1()
{
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)
// WARNING: Unable to use type for symbol uVar5

u8 *mixed_dos3_call_1000_39f2(u8 *param_1,
                              char *param_2,
                              u8 *param_3)
{
    u8 *pbVar2;
    u8 *puVar3;
    let mut uVar4: u16;
    code *pcVar5;
    u8 *puVar6;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut piVar7: *mut i16;
    let mut piVar8: *mut i16;
    let mut piVar9: *mut i16;
    u8 *pbVar10;
    let mut iVar10: i16;
    u8 *puVar11;
    u8 *pbVar12;
    u8 *puVar12;
    u8 *unaff_SI;
    u8 *pbVar13;
    let mut uVar14: u16;
    let mut unaff_CS: u16;
    u8 uVar15;
    u8 bVar16;
    char cVar17;
    char in_AF;
    let mut bVar18: bool;
    char cVar19;
    char cVar20;
    let mut uVar21: u32;
    char *pcVar22;
    u8 *puVar23;
    let mut in_stack_0000fff6: i16;
    let mut piStack8: *mut i16;
    let mut piStack6: *mut i16;
    u8 *puVar2;
    let mut uVar5: u16;
    u8 *pbVar1;

    puVar6 = (u8 *) u16_1050_5f8a;
    if ((u16_1050_61ec != 0x0)
        && (puVar6 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e, param_1 < (u8 *) ( &u16_1050_0002 + 0x1U))) {
        param_1 = (u8 *) u16_1050_5f8a;
    }
    if (puVar6 <= param_1) {
        uVar15 = true;
        puVar6 = (u8 *) 0x900;
        goto LAB_1000_299d;
    }
    puVar12 = param_1;
    puVar23 = (u8 *) u16_1050_5f8a;
    if ((param_1[0x5f90] & 0x20) != 0x0) {
        uVar15 = false;
        pcVar5 = (code *) swi(0x21);
        puVar6 = (u8 *) (*pcVar5)();
        unaff_CS = 0x1000;
        if ((bool) uVar15) {
            goto LAB_1000_299d;
        }
    }
    pbVar12 = (u8 *) param_2;
    if ((puVar12[0x5f90] & 0x80) == 0x0) {//
//        LAB_1000_3acf:
        uVar15 = false;
        puVar6 = param_3;
        if (param_3 != NULL) {
            uVar15 = puVar12 < puVar23;
            if ((bool) uVar15) {
                uVar15 = 0x0;
                pcVar5 = (code *) swi(0x21);
                uVar21 = (*pcVar5)();
            } else {
                piVar8 = pass1_1000_55b1( &DAT_1050_1050,
                                         in_stack_0000fff6);
                uVar21 = CONCAT22(pbVar12,
                                  piVar8);
            }
            puVar6 = (u8 *) uVar21;
            if ((bool) uVar15) {
                puVar6 = (u8 *) CONCAT11(0x9,
                                         (char) uVar21);
            } else {
                uVar15 = false;
                if (puVar6 == NULL) {
                    if (((puVar12[0x5f90] & 0x40) == 0x0) || (*(char *) ((u32) uVar21 >> 0x10) != '\x1a')) {
                        uVar15 = true;
                        puVar6 = (u8 *) 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                }
            }
        }
    } else {
        in_stack_0000fff6 = (i16) &DAT_1050_1050;
        bVar18 = true;
        piStack6 = NULL;
        piStack8 = NULL;
        piVar9 = (i16 *) param_3;
        pbVar13 = pbVar12;
        if (param_3 != NULL) {
            do {
                if (piVar9 == NULL) {
                    break;
                }
                piVar9 = (i16 *) ( piVar9 + -0x1);
                pbVar1 = pbVar13;
                pbVar13 = pbVar13 + 0x1;
                bVar18 = *pbVar1 == '\n';
            } while (!bVar18);
            puVar23 = unaff_SI;
            if (!bVar18) {
                goto LAB_1000_3acf;
            }
            pcVar22 = param_2;
            uVar6 = pass1_1000_3bac();
            pcVar22 =  ((u32) pcVar22 >> 0x10);
            pbVar10 = (u8 *) pcVar22;
            if (uVar6 < 0xa9) {
                exit_1000_25f2(-0x4,
                               0x3ad9,
                               unaff_CS,
                               pcVar22);
                if ( pbVar13 -  pbVar10 == 0x0) {
                    return unaff_SI;
                }
                bVar16 = param_1 < unaff_SI;
                uVar4 =  param_1 -  unaff_SI;
                cVar20 =  uVar4 < 0x0;
                cVar19 = uVar4 == 0x0;
                cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1U) == 0x0;
                if ((bool) bVar16) {
                    bVar16 = 0x0;
                    cVar20 = '\0';
                    cVar19 = '\x01';
                    cVar17 = '\x01';
                    pcVar5 = (code *) swi(0x21);
                    piVar7 = (i16 *) (*pcVar5)( &DAT_1050_1050,
                                               piVar9,
                                               puVar12);
                } else {
                    piVar7 = pass1_1000_55b1( pbVar13 -  pbVar10,
                                             (i16) &DAT_1050_1050);
                }
                if (!(bool) bVar16) {
                    bVar16 = piVar9 < piVar7;
                    uVar4 =  piVar9 -  piVar7;
                    cVar20 =  uVar4 < 0x0;
                    cVar19 = uVar4 == 0x0;
                    cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1U) == 0x0;
                    piStack6 = piVar7;
                    if ((bool) bVar16 || (bool) cVar19) {
                        return unaff_SI;
                    }
                }
                uVar4 =
                     (u8) (cVar20 << 0x7 | cVar19 << 0x6 | in_AF << 0x4 | cVar17 << 0x2 | 0x2U | bVar16) << 0x8;
                puVar6 = (u8 *) ( piVar7 & 0xff | uVar4);
                if (piStack6 == NULL) {
                    uVar15 = (uVar4 & 0x100) != 0x0;
                    if ((bool) uVar15) {
                        puVar6 = (u8 *) CONCAT11(0x9,
                                                 (char) ( piVar7 & 0xff));
                    } else if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
                        uVar15 = true;
                        puVar6 = (u8 *) 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                    goto LAB_1000_299d;
                }
            } else {
                puVar6 = &stack0xfff0;
                iVar10 = 0x200;
                if (uVar6 < 0x228) {
                    iVar10 = 0x80;
                }
                iVar10 = -iVar10;
                puVar11 = &stack0xfff0 + iVar10;
                puVar12 = &stack0xfff0 + iVar10;
                (&stack0xffee + iVar10) =  &DAT_1050_1050;
                uVar14 = (&stack0xffee + iVar10);
                do {
                    pbVar2 = pbVar12;
                    pbVar12 = pbVar12 + 0x1;
                    bVar16 = *pbVar2;
                    uVar5 = uVar6 & 0xff00;
                    uVar6 = uVar5 | bVar16;
                    if (bVar16 == 0xa) {
                        uVar7 = CONCAT11((char) (uVar5 >> 0x8),
                                         0xd);
                        if (puVar12 == puVar6) {
                            (&stack0xffee + iVar10) = 0x3abd;
                            uVar7 = mixed_dos3_call_1000_3ad9(uVar7,
                                                              (i16) puVar11,
                                                              param_3,
                                                              (&stack0xfff0 + iVar10));
                        }
                        puVar3 = puVar12;
                        puVar12 = puVar12 + 0x1;
                        *puVar3 = (u8) uVar7;
                        uVar6 = CONCAT11((char) (uVar7 >> 0x8),
                                         0xa);
                        piStack8 = (i16 *) ( piStack8 + 0x1);
                    }
                    if (puVar12 == puVar6) {
                        (&stack0xffee + iVar10) = 0x3ac8;
                        uVar6 = mixed_dos3_call_1000_3ad9(uVar6,
                                                          (i16) puVar11,
                                                          param_3,
                                                          (&stack0xfff0 + iVar10));
                    }
                    puVar2 = puVar12;
                    puVar12 = puVar12 + 0x1;
                    *puVar2 = (u8) uVar6;
                    param_3 = param_3 + -0x1;
                } while (param_3 != NULL);
                (&stack0xffee + iVar10) = 0x3ab1;
                mixed_dos3_call_1000_3ad9(uVar6,
                                          (i16) puVar11,
                                          0x0,
                                          (&stack0xfff0 + iVar10));
            }
        }
        uVar15 = piStack6 < piStack8;
        puVar6 = (u8 *) ( piStack6 -  piStack8);
    }//
//    LAB_1000_299d:
    if ((bool) uVar15) {
        pass1_1000_29b5(puVar6);
        puVar6 = (u8 *) 0xffff;
    }
    return puVar6;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

u16 mixed_dos3_call_1000_3ad9(mut param_1: u16 ,
                              mut param_2: i16,
                              i16 *param_3,
                              mut param_4: u16 )
{
    let mut uVar2: u16;
    code *pcVar3;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut uVar5: u16;
    let mut unaff_BP: i16;
    let mut unaff_DI: i16;
    u8 bVar5;
    let mut bVar6: bool;
    u8 cVar7;
    char in_AF;
    u8 cVar8;
    u8 cVar9;
    let mut puVar1: *mut u16;
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut uVar1: u16;

    if (unaff_DI - param_2 == 0x0) {
        return param_4;
    }
    uVar5 = (unaff_BP + 0x6);
    puVar1 = (u16 *) (unaff_BP + -0xc);
    bVar5 = uVar5 < *puVar1;
    uVar1 = uVar5 - *puVar1;
    cVar9 =  uVar1 < 0x0;
    cVar8 = uVar1 == 0x0;
    cVar7 = (POPCOUNT(uVar1 & 0xff) & 0x1U) == 0x0;
    if ((bool) bVar5) {
        bVar5 = 0x0;
        cVar9 = '\0';
        cVar8 = '\x01';
        cVar7 = '\x01';
        pcVar3 = (code *) swi(0x21);
        piVar5 = (i16 *) (*pcVar3)();
    } else {
        piVar5 = pass1_1000_55b1(unaff_DI - param_2,
                                 (i16) &DAT_1050_1050);
    }
    if (!(bool) bVar5) {
        piVar1 = (i16 *) (unaff_BP + -0x4);
        *piVar1 = *piVar1 +  piVar5;
        bVar5 = param_3 < piVar5;
        uVar2 =  param_3 -  piVar5;
        cVar9 =  uVar2 < 0x0;
        cVar8 = uVar2 == 0x0;
        cVar7 = (POPCOUNT(uVar2 & 0xff) & 0x1U) == 0x0;
        if ((bool) bVar5 || (bool) cVar8) {
            return param_4;
        }
    }
    uVar2 =  (u8) (cVar9 << 0x7 | cVar8 << 0x6 | in_AF << 0x4 | cVar7 << 0x2 | 0x2 | bVar5) << 0x8;
    uVar4 =  piVar5 & 0xff | uVar2;
    if ((unaff_BP + -0x4) == 0x0) {
        bVar6 = (uVar2 & 0x100) != 0x0;
        if (bVar6) {
            uVar4 = CONCAT11(0x9,
                             (char) ( piVar5 & 0xff));
        } else if (((*(u8 *) (uVar5 + 0x5f90) & 0x40) == 0x0) || (**(char **) (unaff_BP + 0x8) != '\x1a')) {
            bVar6 = true;
            uVar4 = 0x1c00;
        } else {
            bVar6 = false;
        }
    } else {
        uVar2 = (unaff_BP + -0x4);
        puVar2 = (u16 *) (unaff_BP + -0x6);
        bVar6 = uVar2 < *puVar2;
        uVar4 = uVar2 - *puVar2;
    }
    if (bVar6) {
        ((unaff_BP + -0xa) + 0x2) = 0x29a2;
        pass1_1000_29b5(uVar4);
        uVar4 = 0xffff;
    }
    return uVar4;
}

i16 pass1_1000_3bac(void)
{
    let mut iVar1: i16;

    if (PTR_LOOP_1050_5f48 < &stack0x0004) {
        iVar1 = -( PTR_LOOP_1050_5f48 -  &stack0x0004);
    } else {
        iVar1 = 0x0;
    }
    return iVar1;
}
pub fn pass1_1000_3bc0(mut param_1: i16,
                     mut param_2: i16)
{
    let mut piVar1: *mut i16;
    let mut u_var2: u16;
    u16_t uVar3;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut i_var6: i16;
    let mut unaff_si: *mut u16;
    let mut pu_var7: *mut u16;
    let mut unaff_di: u16;
    let mut b_var8: bool;
    let mut u_var9: u32;

    if ((*(u8 *) (param_2 + 0x2) & 0x1) != 0x0) {
        pass1_1000_3cb7(param_2);
        u_var5 = *unaff_si;
        if ((u_var5 & 0x1) != 0x0) {
            param_1 = (param_1 - u_var5) + -0x1;
        }
        u_var5 = (param_2 + 0x4);
        if (u_var5 != 0x0) {
            u_var4 = param_1 + 0x2U + u_var5;
            if (!CARRY2(param_1 + 0x2U,
                        u_var5)) {
                uVar3 = pass1_1000_29dc( &DAT_1050_1050);
                u_var5 = &PTR_LOOP_1050_6066;
                if (u_var5 == 0x1000) {
                    goto LAB_1000_3c12;
                }
                u_var2 = 0x8000;
                while (u_var5 <= u_var2) {
                    u_var2 >>= 0x1;
                    if (u_var2 == 0x0) {
                        goto LAB_1000_3c2b;
                    }
                }
                if (u_var2 < 0x8) {
                    goto LAB_1000_3c2b;
                }
                u_var5 = u_var2 << 0x1;
                goto LAB_1000_3c12;
            }
            u_var2 = 0x0;
            u_var5 = 0xfff0;
            if (u_var4 == 0x0) {
                while (true) {
                    b_var8 = false;
                    u_var9 = mixed_mem_op_1000_3c51(u_var2,
                                                    u_var4,
                                                    param_2,
                                                    0x3c23,
                                                    unaff_di);
                    if (!b_var8) {
                        break;
                    }
                    if (u_var5 == 0xfff0) {
                        return;
                    }//
//                    LAB_1000_3c2b:
                    u_var5 = 0x10;//
//                    LAB_1000_3c12:
                    u_var5 -= 0x1;
                    u_var2 = u_var5 + u_var4;
                    if (CARRY2(u_var5,
                               u_var4)) {
                        u_var2 = 0x0;
                    }
                    u_var5 = ~u_var5;
                    u_var2 &= u_var5;
                }
                i_var6 =  u_var9 - (param_2 + 0x4);
                (param_2 + 0x4) = u_var9;
                (u16 *) (param_2 + 0xa) = unaff_si;
                piVar1 = *(i16 **) (param_2 + 0xc);
                *piVar1 = i_var6 + -0x1;
                pu_var7 = (u16 *) ( piVar1 + i_var6);
                *pu_var7 = 0xfffe;
                (u16 *) (param_2 + 0xc) = pu_var7;
            }
        }
    }
    return;
}

/*
Unable to decompile 'mixed_mem_op_1000_3c51'
Cause:
Low-level Error: Overlapping input varnodes
*/
pub fn pass1_1000_3cb7(mut param_1: i16)
{
    let mut u_var1: u16;
    let mut pu_var2: *mut u16;

    pu_var2 = (u16 *) (param_1 + 0xa);
    if (pu_var2 == (u16 *) (param_1 + 0xc)) {
        pu_var2 = (u16 *) (param_1 + 0x8);
    }
    while (true) {
        u_var1 = *pu_var2;
        if (u_var1 == 0xfffe) {
            break;
        }
        pu_var2 = (u16 *) ( pu_var2 + (u_var1 & 0xfffe) + 0x2);
    }
    return;
}
pub fn pass1_1000_3cd8(mut param_1: u16 ,
                     mut param_2: u16 )
{
    free_mem_1000_407a(param_1,
                       param_2);
    return;
}

u16 *pass1_1000_3cea(mut param_1: u32,
                     char *param_2)
{
    let mut pUVar1: *mut u16;
    char *pcVar2;
    let mut pUVar3: *mut u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut pUVar7: *mut u16;
    char *pcVar8;
    let mut pUVar9: *mut u16;
    let mut pUVar10: *mut u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut bVar13: bool;

    uVar11 =  (param_1 >> 0x10);
    bVar13 = true;
    iVar4 = -0x1;
    pUVar7 = (u16 *) param_1;
    do {
        if (iVar4 == 0x0) {
            break;
        }
        iVar4 += -0x1;
        pUVar1 = pUVar7;
        pUVar7 = (u16 *) ( pUVar7 + 0x1);
        bVar13 = *(char *) pUVar1 == '\0';
    } while (!bVar13);
    pUVar10 = (u16 *) ( pUVar7 + -0x1);
    uVar12 =  ((u32) param_2 >> 0x10);
    pcVar8 = (char *) param_2;
    uVar5 = 0xffff;
    do {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        pcVar2 = pcVar8;
        pcVar8 = pcVar8 + 0x1;
        bVar13 = *pcVar2 == '\0';
    } while (!bVar13);
    uVar5 = ~uVar5;
    if (!bVar13) {
        pcVar8 = pcVar8 + -uVar5;
        uVar5 += 0x1;
    }
    pUVar9 = (u16 *) (pcVar8 + -uVar5);
    if (uVar5 == 0x0) {
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
        *pUVar10 = *pUVar1;
        uVar5 = 0xfffe;
        pUVar10 = (u16 *) ( pUVar7 + 0x1);
    } else if (( pUVar9 & 0x1) != 0x0) {
        pUVar1 = pUVar9;
        pUVar9 = (u16 *) ( pUVar9 + 0x1);
        *(u8 *) pUVar10 = *(u8 *) pUVar1;
        uVar5 -= 0x1;
        pUVar10 = pUVar7;
    }
    for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        pUVar3 = pUVar10;
        pUVar10 = pUVar10 + 0x1;
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
        *pUVar3 = *pUVar1;
    }
    for (uVar5 =  ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
        pUVar3 = pUVar10;
        pUVar10 = (u16 *) ( pUVar10 + 0x1);
        pUVar1 = pUVar9;
        pUVar9 = (u16 *) ( pUVar9 + 0x1);
        *(u8 *) pUVar3 = *(u8 *) pUVar1;
    }
    return (u16 *) param_1;
}
pub fn unk_str_op_1000_3d3e(char *param_1,
                          char *in_string_2)
{
    let mut pu_var4: *mut u16;
    let mut pu_var5: *mut u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    char *l_string_2;
    char *puVar6;
    char *puVar7;
    let mut u_var8: u16;
    char *l_string_1;
    let mut l_b_var8: bool;
    char *puVar3;
    char *puVar2;
    char *puVar1;

    l_string_1 = (char *) ((u32) in_string_2 >> 0x10);
    l_string_2 = (char *) in_string_2;
    l_b_var8 = true;
    u_var6 = 0xffff;
    puVar6 = l_string_2;
    do {
        if (u_var6 == 0x0) {
            break;
        }
        u_var6 -= 0x1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        l_b_var8 = *puVar1 == '\0';
    } while (!l_b_var8);
    u_var6 = ~u_var6;
    u_var8 =  ((u32) param_1 >> 0x10);
    puVar7 = (char *) param_1;
    if (l_b_var8) {
        if (((u32) param_1 & 0x1) != 0x0) {
            puVar7 = puVar7 + 0x1;
            l_string_2 = l_string_2 + 0x1;
            *param_1 = *in_string_2;
            u_var6 -= 0x1;
        }
    } else {
        puVar7 = puVar7 + 0x2;
        l_string_2 = l_string_2 + 0x2;
        param_1 = in_string_2;
        u_var6 -= 0x1;
    }
    for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
        pu_var5 = (u16 *) puVar7;
        puVar7 = (char *) ( puVar7 + 0x2);
        pu_var4 = (u16 *) l_string_2;
        l_string_2 = (char *) ( l_string_2 + 0x2);
        *pu_var5 = *pu_var4;
    }
    for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
        pu_var5 = (u16 *) puVar7;
        puVar7 = (char *) ( puVar7 + 0x1);
        pu_var4 = (u16 *) l_string_2;
        l_string_2 = (char *) ( l_string_2 + 0x1);
        *(u8 *) pu_var5 = *(u8 *) pu_var4;
    }
    return;
}

i16 pass1_1000_3d7a(char *param_1,
                    char *param_2)
{
    u8 *pbVar2;
    u8 *pbVar3;
    let mut iVar4: i16;
    let mut uVar5: u16;
    char *string_4;
    char *string_1;
    char *string_2;
    char *string_6;
    let mut uVar6: u16;
    let mut bool_1: bool;
    let mut bool_2: bool;
    char *pbVar4;
    char *pbVar1;
    char *string_3;

    string_1 = (char *) param_1;
    uVar6 =  ((u32) param_2 >> 0x10);
    string_2 = (char *) param_2;
    iVar4 = 0x0;
    uVar5 = 0xffff;
    do {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        string_3 = string_2;
        string_2 = string_2 + 0x1;
    } while (*string_3 != '\0');
    string_4 = (char *) ~uVar5;
    bool_1 = string_2 < string_4;
    string_6 = string_2 + - string_4;
    bool_2 = string_6 == NULL;
    do {
        if (string_4 == NULL) {
            break;
        }
        string_4 = string_4 + -0x1;
        pbVar3 = (u8 *) string_6;
        string_6 = (char *) ((u8 *) string_6 + 0x1);
        pbVar2 = (u8 *) string_1;
        string_1 = (char *) ((u8 *) string_1 + 0x1);
        bool_1 = *pbVar2 < *pbVar3;
        bool_2 = *pbVar2 == *pbVar3;
    } while (bool_2);
    if (!bool_2) {
        iVar4 = (0x1 -  bool_1) -  (bool_1 != 0x0);
    }
    return iVar4;
}

u16 str_op_1000_3da4(char *param_1)
{
    char *pcVar1;
    let mut uVar2: u16;
    char *pcVar3;
    let mut bVar4: bool;

    pcVar3 = (char *) param_1;
    bVar4 = true;
    uVar2 = 0xffff;
    do {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar3;
        pcVar3 = pcVar3 + 0x1;
        bVar4 = *pcVar1 == '\0';
    } while (!bVar4);
    uVar2 = ~uVar2;
    if (bVar4) {
        uVar2 -= 0x1;
    }
    return uVar2;
}

uchar str_op_1000_3dbe(char *param_1,
                       char *param_2,
                       mut param_3: u16 )
{
    char *pcVar1;
    char cVar2;
    char *pcVar3;
    char *pcVar4;
    let mut uVar5: u16;

    uVar5 =  ((u32) param_1 >> 0x10);
    pcVar4 = (char *) param_1;
    pcVar3 = (char *) param_2;
    if (param_3 != 0x0) {
        do {
            pcVar1 = pcVar3;
            pcVar3 = pcVar3 + 0x1;
            cVar2 = *pcVar1;
            if (cVar2 == '\0') {
                break;
            }
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 0x1;
            *pcVar1 = cVar2;
            param_3 -= 0x1;
        } while (param_3 != 0x0);
        for (; param_3 != 0x0; param_3 -= 0x1) {
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 0x1;
            *pcVar1 = '\0';
        }
    }
    return (uchar) param_1;
}

u16 pass1_1000_3de8(char *param_1,
                    char *param_2,
                    mut param_3: u16 ,
                    u16_t param_4,
                    u16_t param_5)
{
    u8 *pbVar1;
    char *pcVar2;
    char *pcVar3;
    u8 bVar4;
    let mut uVar5: u16;
    let mut iVar6: i16;
    char *pcVar7;
    char *pcVar8;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;

    if (param_3 != 0x0) {
        uVar9 =  ((u32) param_1 >> 0x10);
        pcVar8 = (char *) param_1;
        uVar5 = param_3;
        pcVar7 = pcVar8;
        do {
            if (uVar5 == 0x0) {
                break;
            }
            uVar5 -= 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while (*pcVar2 != '\0');
        iVar6 = param_3 - uVar5;
        uVar10 =  ((u32) param_2 >> 0x10);
        pcVar7 = (char *) param_2;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while (*pcVar2 == *pcVar3);
        bVar4 = pcVar7[-0x1];
        uVar5 = 0x0;
        pbVar1 = (u8 *) (pcVar8 + -0x1);
        bVar11 = bVar4 == *pbVar1;
        if (bVar4 < *pbVar1 || bVar11) {
            if (bVar11) {
                return 0x0;
            }
            uVar5 = 0xfffe;
        }
        param_3 = ~uVar5;
    }
    return param_3;
}

i16 pass1_1000_3e2c(mut param_1: u32)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    let mut iVar4: i16;
    u8 *pbVar5;
    let mut uVar6: u16;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 = (u8 *) param_1;
    iVar4 = 0x0;
    do {
        do {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
        goto LAB_1000_3e4d;
    }
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;//
//        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +  (u8) (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

i16 pass1_1000_3e2c(mut param_1: u32)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    let mut iVar4: i16;
    u8 *pbVar5;
    let mut uVar6: u16;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 = (u8 *) param_1;
    iVar4 = 0x0;
    do {
        do {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
        goto LAB_1000_3e4d;
    }
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;//
//        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +  (u8) (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

i16 pass1_1000_3e2c(mut param_1: u32)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    let mut iVar4: i16;
    u8 *pbVar5;
    let mut uVar6: u16;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 = (u8 *) param_1;
    iVar4 = 0x0;
    do {
        do {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
        goto LAB_1000_3e4d;
    }
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;//
//        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +  (u8) (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

u8 *pass1_1000_3e82(mut param_1: u16 ,
                    u8 *param_2,
                    mut param_3: u16 )
{
    u8 *pbVar1;
    let mut uVar2: u32;
    u8 bVar3;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    u8 *pbVar8;
    u8 *pbVar9;
    u8 *pbVar10;
    u8 *pbVar11;
    let mut uVar12: u16;
    let mut bVar13: bool;
    char cVar4;

    uVar6 = 0x0;
    if (param_3 == 0xa) {
        uVar6 =  param_1 >> 0xf;
    }
    uVar12 =  ((u32) param_2 >> 0x10);
    pbVar9 = (u8 *) param_2;
    pbVar10 = pbVar9;
    pbVar8 = pbVar9;
    if ((param_3 == 0xa) && ( uVar6 < 0x0)) {
        pbVar10 = pbVar9 + 0x1;
        *param_2 = '-';
        bVar13 = param_1 != 0x0;
        param_1 = -param_1;
        uVar6 = -(uVar6 + bVar13);
        pbVar8 = pbVar10;
    }
    do {
        uVar7 = 0x0;
        uVar5 = uVar6;
        if (uVar6 != 0x0) {
            uVar5 = uVar6 / param_3;
            uVar7 = uVar6 % param_3;
        }
        uVar2 = CONCAT22(uVar7,
                         param_1);
        param_1 =  (uVar2 / param_3);
        cVar4 = (char) (uVar2 % (u32) param_3);
        bVar3 = cVar4 + 0x30;
        if (0x39 < bVar3) {
            bVar3 = cVar4 + 0x57;
        }
        pbVar11 = pbVar10 + 0x1;
        *pbVar10 = bVar3;
        uVar6 = uVar5;
        pbVar10 = pbVar11;
    } while ((uVar5 | param_1) != 0x0);
    *pbVar11 = 0x0;
    do {
        pbVar11 = pbVar11 + -0x1;
        pbVar1 = pbVar11;
        bVar3 = *pbVar1;
        *pbVar1 = *pbVar8;
        *pbVar8 = bVar3;
        pbVar10 = pbVar8 + 0x2;
        pbVar8 = pbVar8 + 0x1;
    } while (pbVar10 < pbVar11);
    return pbVar9;
}
pub fn fatal_app_exit_1000_3e9e(void)
{
    FatalAppExit16(s_ABNORMAL_PROGRAM_TERMINATION_1050_6544,
                   0x0);
    return;
}

i16 pass1_1000_3ec0(mut param_1: u16 ,
                    mut param_2: u16 )
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    u16_t uVar3;
    u16_t unaff_SI;
    u16_t uVar4;
    u32 *puVar4;

    puVar4 = (u32 *) CONCAT22(PTR_LOOP_1050_5fc0,
                              PTR_LOOP_1050_5fbe);
    if ((( PTR_LOOP_1050_5fc0 |  PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
        uVar1 = str_op_1000_3da4((char *) CONCAT22(param_2,
                                                   param_1));
        loop {
            uVar4 = (u16_t)((u32) puVar4 >> 0x10);
            uVar3 = (u16_t) puVar4;
            if (((uVar3 + 0x2) | puVar4) == 0x0) {
                break;
            }
            uVar2 = str_op_1000_3da4((char *) CONCAT22((uVar3 + 0x2),
                                                       puVar4));
            if (((uVar1 < uVar2) && (*(char *) ( *puVar4 + uVar1) == '=')) && (uVar2 =
                                                                                        pass1_1000_3de8((char *) CONCAT22((uVar3
                                                                                                                              + 0x2),
                                                                                                                          puVar4),
                                                                                                        (char *) CONCAT22(param_2,
                                                                                                                          param_1),
                                                                                                        uVar1,
                                                                                                        unaff_SI,
                                                                                                        uVar3), uVar2
                == 0x0)) {
                return puVar4 + uVar1 + 0x1;
            }
            puVar4 = (u32 *) ((u32) puVar4 & 0xffff0000 | (u32) (uVar3 + 0x4));
        }
    }
    return 0x0;
}

i16 pass1_1000_3f5c(void)
{
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut iVar3: i16;

    iVar3 = 0x0;
    if (u16_1050_61ec == 0x0) {
        puVar2 = (u16 *) &PTR_LOOP_1050_6210;
    } else {
        puVar2 = (u16 *) 0x6234;
    }
    for (; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
        uVar1 = pass1_1000_2a00(puVar2);
        if (uVar1 != 0xffff) {
            iVar3 += 0x1;
        }
    }
    return iVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 sys_1000_3f9c(char *param_1,
                  char *param_2,
                  mut param_3: u16 )
{
    char *pcVar1;
    let mut uVar2: u16;
    let mut unaff_CS: u16;
    let mut local_4: u16;

    PTR_LOOP_1050_68b2._0_1_ = 0x42;
    _u16_1050_68a8 = param_1;
    PTR_LOOP_1050_68ac = (u8 *) 0x7fff;
    _PTR_LOOP_1050_68ae = param_1;
    uVar2 = FUN_1000_30b4();
    pcVar1 = _u16_1050_68a8;
    PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
    if ( PTR_LOOP_1050_68ac < 0x0) {
        mem_1000_2bb6(param_1,
                      0x0,
                      (i16 *) &u16_1050_68a8);
    } else {
        _u16_1050_68a8 = (char *) ((u32) _u16_1050_68a8 & 0xffff0000 | (u32) (u16_1050_68a8 + 0x1));
        *pcVar1 = '\0';
    }
    PTR_LOOP_1050_68b0 = (u8 *) ((u32) _PTR_LOOP_1050_68ae >> 0x10);
    return uVar2;
}

u8 *pass1_1000_400a(mut param_1: i16)
{
    u8 *puVar1;

    if ((param_1 < 0x0) || ( PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
        PTR_LOOP_1050_5f78 = (u8 *) ( &u16_1050_0008 + 0x1);
        puVar1 = (u8 *) 0xffff;
    } else if (((u16_1050_61ec == 0x0) || ((param_1 <  u16_1050_5f8a && (0x2 < param_1)))) && (0x31d
        < CONCAT11(DAT_1050_5f83,
                   DAT_1050_5f82))) {
        puVar1 = PTR_LOOP_1050_5f88;
        if (((*(u8 *) (param_1 + 0x5f90) & 0x1) == 0x0) || (puVar1 = (u8 *) dos3_call_1000_5174(), puVar1 != NULL)) {
            PTR_LOOP_1050_5f88 = puVar1;
            PTR_LOOP_1050_5f78 = (u8 *) ( &u16_1050_0008 + 0x1);
            puVar1 = (u8 *) 0xffff;
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
    GlobalFree16((HGLOBAL16) &DAT_1050_1050);
    return;
}

i16 *mixed_sys_op_1000_40af(mut param_1: u16 ,
                            mut param_2: i16,
                            mut param_3: u16 )
{
    struct astruct_824 **ppaVar1;
    char *pcVar2;
    let mut puVar4: *mut u16;
    char *pcVar5;
    let mut iVar6: i16;
    struct astruct_824 **ppaVar7;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar8: i16;
    HGLOBAL16 hglobal_7;
pub fn *SVar8;
    struct astruct_824 **ppaVar8;
    let mut unaff_SI: i16;
    let mut piVar9: *mut i16;
    char *pcVar10;
    struct astruct_824 *hglobal_di;
    let mut puVar11: *mut u16;
    u8 unaff_CS;
    let mut unaff_SS: u16;
    let mut bVar12: bool;
pub fn *pvVar13;
    struct astruct_825 *paVar14;
    let mut puVar3: *mut u16;
    u8 uVar13;
    u8 uVar14;
    let mut iVar12: i16;
    struct astruct_824 *temp_5fa27366cb;

    do {
        uVar7 =  ((u32) param_1 * (u32) param_3);
        uVar8 = param_2 * param_3 +  ((u32) param_1 * (u32) param_3 >> 0x10);
        if ((uVar8 | uVar7) != 0x0) {
            piVar9 = NULL;
            if ((uVar8 < 0x3) && ((uVar8 < 0x2 || (uVar7 == 0x0)))) {
                if (uVar8 == 0x0) {
                    uVar7 = uVar7 + 0xfff & 0xf000;
                    if (uVar7 == 0x0) {
                        uVar8 = 0x1;
                    }
                } else if ((param_3 - 0x1 & param_3) != 0x0) {
                    piVar9 = (i16 *) (((u32) uVar8 << 0x10) % (u32) param_3);
                    bVar12 = CARRY2(uVar7,
                                     piVar9);
                    uVar7 +=  piVar9;
                    if (bVar12) {
                        goto LAB_1000_41aa;
                    }
                    uVar8 = 0x1;
                }
            } else if ((param_3 - 0x1 & param_3) != 0x0) {
                goto LAB_1000_41aa;
            }
            hglobal_7 = GLobalAlloc16(CONCAT13((char) (uVar8 >> 0x8),
                                               CONCAT12((char) uVar8,
                                                        uVar7)),
                                      0x20);
            if ((hglobal_7 != 0x0) && ((uVar7 & 0x1) != 0x0)) {
                pvVar13 = WIN16_GlobalLock16(hglobal_7);
                SVar8._0_2_ =  pvVar13;
                if (( SVar8 != 0x0) || (pvVar13 == NULL)) {
                    paVar14 = (astruct_825 *) CONCAT22(hglobal_7,
                                                       0x12);
                    uVar13 = '\x12';
                    uVar14 = '\0';
                    pass1_1000_25a8();
                    pass1_1000_2913(CONCAT11(uVar14,
                                             uVar13));
                    pcVar5 = poss_str_op_1000_28dc(paVar14);
                    if (pcVar5 == NULL) {
                        goto LAB_1000_28cb;
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
                hglobal_7 = pass1_1000_422a( ((u32) pvVar13 >> 0x10),
                                            hglobal_7);
                if (hglobal_7 == 0x0) {
                    GlobalUnlock16(uVar8);
                    GlobalFree16((HGLOBAL16) hglobal_di);
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
                        puVar11 = (u16 *) ( puVar11 + 0x1);
                        *(u8 *) puVar4 = 0x0;
                    }
                }
                return piVar9;
            }
        }//
//        LAB_1000_41aa:
        if ((u16_1050_618e |  PTR_LOOP_1050_618c) == 0x0) {
            return NULL;
        }
        iVar8 = ((code) PTR_LOOP_1050_618c)(unaff_CS,
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
    FatalAppExit16((char *) CONCAT13(0x10,
                                     CONCAT12(0x50,
                                              pcVar5)),
                   0x0);
    FatalExit();
    ppaVar8 = (astruct_824 **) &PTR_LOOP_1050_63fe;
    do {
        ppaVar1 = ppaVar8;
        ppaVar8 = ppaVar8 + 0x1;
        temp_5fa27366cb = *ppaVar1;
        ppaVar7 = ppaVar8;
        if ((temp_5fa27366cb == hglobal_di) || (ppaVar7 = (astruct_824 **) (temp_5fa27366cb + 0x1), ppaVar7 == NULL)) {
            return (i16 *) ppaVar7;
        }
        iVar6 = -0x1;
        do {
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

    piStack6 = (i16 *) CONCAT22(PTR_LOOP_1050_6192,
                                PTR_LOOP_1050_6190);
    loop {
        if (PTR_LOOP_1050_6190 + ( PTR_LOOP_1050_6194 & 0xfffc) <= (u8 *) piStack6) {
            return 0x0;
        }
        if (*piStack6 == param_1) {
            break;
        }
        piStack6 = (i16 *) ((u32) piStack6 & 0xffff0000 | ZEXT24((u8 *) piStack6 + 0x4));
    }
    *piStack6 = 0x0;
    return ((u8 *) piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1000_422a(mut param_1: i16,
                    mut param_2: u16 )
{
    u8 *puVar1;
    u8 *puVar2;
    u8 *puVar3;
    u8 *puVar4;
    let mut piStack6: *mut i16;

    piStack6 = (i16 *) CONCAT22(PTR_LOOP_1050_6192,
                                PTR_LOOP_1050_6190);
    loop {
        if (PTR_LOOP_1050_6190 + ( PTR_LOOP_1050_6194 & 0xfffc) <= (u8 *) piStack6) {
            puVar2 = PTR_LOOP_1050_6194 + 0x28;
            puVar4 = PTR_LOOP_1050_6192;
            puVar3 = (u8 *) pass1_1000_16aa( PTR_LOOP_1050_6192,
                                            (u16 *) PTR_LOOP_1050_6190,
                                             PTR_LOOP_1050_6192,
                                             puVar2);
            if (( puVar4 |  puVar3) == 0x0) {
                param_1 = 0x0;
            } else {
                puVar1 = puVar3 + ( PTR_LOOP_1050_6194 & 0xfffc);
                piStack6 = (i16 *) CONCAT22(puVar4,
                                            puVar1);
                PTR_LOOP_1050_6190 = puVar3;
                PTR_LOOP_1050_6192 = puVar4;
                *piStack6 = param_1;
                (puVar1 + 0x2) = param_2;
                PTR_LOOP_1050_6194 = puVar2;
                pass1_1000_4906((StructD *) CONCAT22(puVar4,
                                                     puVar1 + 0x4),
                                NULL,
                                0x24);
            }
            return param_1;
        }
        if (*piStack6 == 0x0) {
            break;
        }
        piStack6 = (i16 *) ((u32) piStack6 & 0xffff0000 | ZEXT24((u8 *) piStack6 + 0x4));
    }
    ((u8 *) piStack6 + 0x2) = param_2;
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
    struct astruct_811 *iVar4;
    struct astruct_810 *iVar5;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut b_var5: bool;
    let mut u_var12: u32;
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut u_var9: u16;

    u_var6 =  ((u32) param_1 >> 0x10);
    iVar4 = (astruct_811 *) param_1;
    u_var5 = iVar4.field2_0x2;
    u_var4 = iVar4.field3_0x4;
    u_var1 = iVar4.field6_0x8;
    u_var2 = iVar4.field7_0xa;
    u_var7 =  ((u32) param_3 >> 0x10);
    u_var3 = *param_3;
    u_var9 = ( param_3 + 0x6);
    b_var5 = false;
    pcVar4 = (code *) swi(0x21);
    u_var12 = (*pcVar4)();
    *param_3 = u_var3;
    ( param_3 + 0x6) = u_var9;
    u_var8 =  ((u32) param_2 >> 0x10);
    iVar5 = (astruct_810 *) param_2;
    param_2 = u_var12;
    iVar5.field2_0x2 = u_var5;
    iVar5.field3_0x4 = u_var4;
    iVar5.field4_0x6 =  (u_var12 >> 0x10);
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
    char cVar7;
    let mut u_var5: u16;
    let mut in_stack_00000002: u16;

    pcVar1 = (code *) swi(0x21);
    (*pcVar1)( &DAT_1050_1050);
    pcVar1 = (code *) swi(0x21);
    u_var3 = in_cx;
    u_var2 = extraout_dx;
    (*pcVar1)();
    u_var6 = extraout_dx_00 >> 0x8;
    cVar7 = (char) u_var3;
    pcVar1 = (code *) swi(0x21);
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
    char cVar1;
    char *pcVar2;
    let mut u_var3: u16;
    let mut i_var4: i16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    char *pcStack8;

    u_var3 = pass1_1000_3ec0(0x61ca,
                             &DAT_1050_1050);
    pcStack8 = (char *) CONCAT22(param_1,
                                 u_var3);
    if (((param_1 | u_var3) != 0x0) && (_DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0,
                                                                  DAT_1050_61ce), *pcStack8 != '\0')) {
        str_op_1000_3dbe((char *) CONCAT22(PTR_DAT_1050_1050_1050_61de,
                                           PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc),
                         (char *) CONCAT22(param_1,
                                           u_var3),
                         0x3);
        pcStack8 = (char *) CONCAT22(param_1,
                                     u_var3 + 0x3);
        cVar1 = *pcStack8;
        if (cVar1 == '-') {
            pcStack8 = (char *) CONCAT22(param_1,
                                         u_var3 + 0x4);
        }
        u_var5 = 0x0;
        u_var9 = 0x0;
        u_var8 = 0xe10;
        u_var3 = pass1_1000_3e2c((u32) pcStack8 & 0xffff | (u32) param_1 << 0x10);
        _DAT_1050_61ce = pass1_1000_52be(u_var3,
                                         u_var5,
                                         u_var8,
                                         u_var9);
        for (; (pcVar2 = pcStack8, *pcStack8 == '+' || (('/' < *pcStack8 && (*pcStack8 < ':'))));
               pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) ( pcStack8 + 0x1))) {
        }
        if (*pcStack8 == ':') {
            u_var5 = 0x0;
            u_var9 = 0x0;
            u_var8 = 0x3c;
            pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) ( pcStack8 + 0x1));
            u_var3 = pass1_1000_3e2c((u32) pcVar2 & 0xffff0000 | (u32) ( pcStack8 + 0x1));
            u_var7 = pass1_1000_52be(u_var3,
                                     u_var5,
                                     u_var8,
                                     u_var9);
            u_var6 =  (u_var7 >> 0x10);
            _DAT_1050_61ce = u_var7 + _DAT_1050_61ce;
            for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
                   pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) ( pcStack8 + 0x1))) {
            }
            if (*pcStack8 == ':') {
                pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) ( pcStack8 + 0x1));
                i_var4 = pass1_1000_3e2c((u32) pcVar2 & 0xffff0000 | (u32) ( pcStack8 + 0x1));
                _DAT_1050_61ce += CONCAT22(u_var6,
                                           i_var4);
                for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
                       pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) ( pcStack8 + 0x1))) {
                }
            }
        }
        PTR_LOOP_1050_61d0 = (u8 *) (_DAT_1050_61ce >> 0x10);
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
    PTR_LOOP_1050_61d0 = (u8 *) (_DAT_1050_61ce >> 0x10);
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
        goto LAB_1000_4623;
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
                goto LAB_1000_460e;
            }
        } else {
            piVar1 = (i16 *) (param_1 + 0xe);
            iVar2 = *piVar1;
            if ((SBORROW2(*piVar1,
                          iVar4) != iVar2 + iStack6 < 0x0) || ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1)))) {
                goto LAB_1000_460e;
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
                             ((u32) ((long)  param_2 * 0x16d) >> 0x10) + ( (param_2 + 0x3) >> 0xf)
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
    char *pcVar1;
    let mut uVar2: u16;
    char *pcVar3;
    char *pcVar4;
    let mut uVar5: u16;
    let mut bVar6: bool;

    uVar5 =  (param_1 >> 0x10);
    pcVar3 = (char *) param_1;
    bVar6 = true;
    uVar2 = 0xffff;
    pcVar4 = pcVar3;
    do {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar4;
        pcVar4 = pcVar4 + 0x1;
        bVar6 = *pcVar1 == '\0';
    } while (!bVar6);
    uVar2 = ~uVar2;
    do {
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
    char *pcVar1;
    char cVar2;
    char cVar3;
    u8 bVar4;
    struct astruct_235 *bVar3;
    let mut bVar5: i16;
    char *pcVar5;
    char *pcVar6;

    pcVar6 = (char *) param_2;
    pcVar5 = (char *) param_1;
    bVar5 = 0xff;
    do {
        do {
            cVar3 = (char) bVar5;
            if (cVar3 == '\0') {
                goto LAB_1000_479d;
            }
            pcVar1 = pcVar6;
            pcVar6 = pcVar6 + 0x1;
            cVar3 = *pcVar1;
            cVar2 = *pcVar5;
            bVar5 = CONCAT11(cVar2,
                             cVar3);
            pcVar5 = pcVar5 + 0x1;
        } while (cVar2 == cVar3);
        bVar4 = cVar3 + 0xbfU + (-((u8) (cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
        bVar3._0_1_ = cVar2 + 0xbf;
        bVar5._0_1_ = (u8) bVar3 + (-((u8) bVar3 < 0x1a) & 0x20U) + 0x41;
        bVar5 = CONCAT11(bVar4,
                         (u8) bVar5);
    } while ((u8) bVar5 == bVar4);
    cVar3 = ((u8) bVar5 < bVar4) * -0x2 + '\x01';//
//    LAB_1000_479d:
    return  cVar3;
}

u16 pass1_1000_47a4(mut param_1: u32,
                    mut param_2: u32)
{
    u8 *pbVar1;
    u8 bVar2;
    let mut puVar3: *mut u16;
    u8 *pbVar4;
    let mut iVar5: i16;
    u8 *pbVar6;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    u16 local_22[0x10];

    puVar7 = local_22;
    for (iVar5 = 0x10; iVar5 != 0x0; iVar5 += -0x1) {
        puVar3 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar3 = 0x0;
    }
    pbVar6 = (u8 *) param_2;
    loop {
        pbVar1 = pbVar6;
        pbVar6 = pbVar6 + 0x1;
        bVar2 = *pbVar1;
        if (bVar2 == 0x0) {
            break;
        }
        pbVar1 = (u8 *) ( local_22 +  (bVar2 >> 0x3));
        *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
    }
    pbVar1 = (u8 *) param_1;
    if (param_1 == 0x0) {
        pbVar1 = pbRam105061e4;
    }
    do {
        pbRam105061e4 = pbVar1;
        uVar8 =  ((u32) pbRam105061e4 >> 0x10);
        pbVar6 = (u8 *) ( pbRam105061e4 + 0x1);
        bVar2 = *pbRam105061e4;
        if (bVar2 == 0x0) {
            return 0x0;
        }
        pbVar1 = (u8 *) ((u32) pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
    } while (('\x01' << (bVar2 & 0x7) & *(u8 *) ( local_22 +  (bVar2 >> 0x3))) != 0x0);
    do {
        pbVar4 = pbVar6;
        bVar2 = *pbVar4;
        if (bVar2 == 0x0) {
            goto LAB_1000_483c;
        }
        pbVar6 = pbVar4 + 0x1;
    } while (('\x01' << (bVar2 & 0x7) & *(u8 *) ( local_22 +  (bVar2 >> 0x3))) == 0x0);
    *pbVar4 = 0x0;
    pbVar4 = pbVar4 + 0x1;//
//    LAB_1000_483c:
    pbRam105061e4 = (u8 *) ((u32) pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
    return  pbRam105061e4;
}

u16 pass1_1000_484c(mut param_1: u32,
                    mut param_2: u32,
                    mut param_3: u16 )
{
    u8 *pbVar1;
    u8 *pbVar2;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    u8 *pbVar6;
    u8 *pbVar7;
    let mut iVar8: i16;
    let mut bVar9: bool;
    let mut bVar10: bool;

    if (param_3 == 0x0) {
        return 0x0;
    }
    do {
        iVar8 =  (param_2 >> 0x10);
        pbVar7 = (u8 *) param_2;
        iVar3 =  (param_1 >> 0x10);
        pbVar6 = (u8 *) param_1;
        uVar4 = ~ pbVar7;
        uVar4 = ((param_3 - 0x1) - uVar4 & - (param_3 - 0x1 < uVar4)) + uVar4;
        uVar5 = ~ pbVar6;
        uVar4 = (uVar4 - uVar5 & - (uVar4 < uVar5)) + uVar5 + 0x1;
        bVar9 = param_3 < uVar4;
        param_3 -= uVar4;
        bVar10 = param_3 == 0x0;
        do {
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
            param_2 = (u32) (iVar8 + 0x6c) << 0x10;
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
            puVar6 = (u16 *) param_2;
            iVar8 =  (param_1 >> 0x10);
            puVar7 = (u16 *) param_1;
            uVar4 = ~ puVar7;
            uVar4 = ((param_3 - 0x1U) - uVar4 & - (param_3 - 0x1U < uVar4)) + uVar4;
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
                puVar7 = (u16 *) ( puVar7 + 0x1);
                puVar1 = puVar6;
                puVar6 = (u16 *) ( puVar6 + 0x1);
                *(u8 *) puVar2 = *(u8 *) puVar1;
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
                param_1 = (u32) (iVar8 + 0x6c) << 0x10;
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
    struct astruct_20 *struct_1;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    struct astruct_20 *struct_1_hi;

    if (param_3 != 0x0) {
        struct_1_hi = (astruct_20 *) ((u32) param_1 >> 0x10);
        struct_1 = (astruct_20 *) - (u16 *) param_1;
        uVar5 = param_3;
        if (struct_1 != NULL) {
            uVar5 = ( struct_1 - param_3 & - (struct_1 < param_3)) + param_3;
            struct_1 = (astruct_20 *) (param_3 - uVar5);
        }
        uVar3 =  in_wnd_class & 0xff |  in_wnd_class << 0x8;
        puVar6 = (u16 *) param_1;
        for (uVar4 = uVar5 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = puVar6 + 0x1;
            *puVar1 = uVar3;
        }
        for (uVar5 =  ((uVar5 & 0x1) != 0x0);
             uVar2 = (u8) ( in_wnd_class & 0xff), uVar5 != 0x0;
             uVar5 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = (u16 *) ( puVar6 + 0x1);
            *(u8 *) puVar1 = uVar2;
        }
        if (struct_1 != NULL) {
            for (uVar5 =  struct_1 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = puVar6 + 0x1;
                *puVar1 = uVar3;
            }
            for (uVar5 =  (( struct_1 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = (u16 *) ( puVar6 + 0x1);
                *(u8 *) puVar1 = uVar2;
            }
        }
    }
    return (u16 *) param_1;
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
        uVar3 =  ((u32) uVar2 * (u32) param_6);
        uVar4 = uVar3 + uStack20;
        iVar6 = ( ((u32) uVar2 * (u32) param_6 >> 0x10) +  CARRY2(uVar3,
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
    i32 lVar3;
    let mut u_var4: u16;
    let mut i_var5: i16;
    let mut iVar6: i16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    struct astruct_171 *puVar11;
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
                do {
                    u_var11 >>= 0x1;
                    iVar6 += -0x1;
                } while (iVar6 != 0x0 && u_var11 != 0x0);
                if (( ((u32)  -iVar6 * 0x8 >> 0x10) != 0x0)
                    || (u_var11 = pass1_1000_3bac(), u_var11 <  ((u32)  -iVar6 * 0x8))) {
                    exit_1000_25f2(-0x4,
                                   0x4b7b,
                                   unaff_CS,
                                   unaff_di);
                    return;
                }
                puVar11 = (astruct_171 *) &stack0xfff6;
                lVar3 = (u32) (param_3 - 0x1) * (u32) param_4;
                u_var11 =  lVar3;
                u_stack_y14 = u_var11 + param_1;
                u_var11 = ( ((u32) lVar3 >> 0x10) +  CARRY2(u_var11,
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
                    do {
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
                                goto LAB_1000_4bbc;
                            }
                        }
                        do {
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
                            goto LAB_1000_4c58;
                        }
                        pass1_1000_4ceb(puVar11.field20_0x14);
                        u_stack_y26 = u_var10;
                        u_stack_y24 = u_var9;
                        u_var13 = u_var7;
                        u_stack_y22 = u_var8;
                    } while (true);
                }
                goto LAB_1000_4b7d;
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
    goto LAB_1000_4b81;
}
pub fn pass1_1000_4ceb(mut param_1: u16 )
{
    u8 *puVar1;
    let mut pu_var2: *mut u16;
    u8 uVar3;
    let mut u_var4: u16;
    let mut unaff_si: i16;
    let mut unaff_di: i16;
    let mut unaff_es: u16;

    if ((param_1 & 0x1) != 0x0) {
        param_1 -= 0x1;
        puVar1 = (u8 *) (param_1 + unaff_di);
        uVar3 = *puVar1;
        *puVar1 = *(u8 *) (param_1 + unaff_si);
        *(u8 *) (param_1 + unaff_si) = uVar3;
        if (param_1 == 0x0) {
            return;
        }
    }
    do {
        param_1 -= 0x2;
        pu_var2 = (u16 *) (param_1 + unaff_di);
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

u16 pass1_1000_4d24(void)
{
    let mut uVar1: u32;

    uVar1 = pass1_1000_52be(DAT_1050_61e8,
                             PTR_LOOP_1050_61ea,
                             s_TPPOPMENU_1050_43fa + 0x3,
                            0x3);
    PTR_LOOP_1050_61ea = (u8 *) (uVar1 + 0x269ec3 >> 0x10);
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
    char *pcStack18;
    let mut u_stack12: u16;
    let mut u_stack10: u16;
    let mut u_stack8: u16;
    let mut u_stack6: u16;

    u_stack10 = 0x0;
    u_stack12 = 0x0;
    u_var4 =  ((u32) in_string_1 >> 0x10);
    i_var2 =  in_string_1;
    if ((*in_string_1 == '\0') || (*(char *) (i_var2 + 0x1) != ':')) {
        if (in_string_2 != NULL) {
            *in_string_2 = '\0';
        }
    } else {
        if (in_string_2 != NULL) {
            *in_string_2 = *in_string_1;
            *(u8 *) ( in_string_2 + 0x1) = *(u8 *) (i_var2 + 0x1);
            *(u8 *) ( in_string_2 + 0x2) = 0x0;
        }
        in_string_1 = (char *) ((u32) in_string_1 & 0xffff0000 | (u32) (i_var2 + 0x2));
    }
    u_stack6 = 0x0;
    u_stack8 = 0x0;
    pcStack18 = in_string_1;
    while (true) {
        u_var5 =  ((u32) pcStack18 >> 0x10);
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
        pcStack18 = (char *) ((u32) pcStack18 & 0xffff0000 | (u32) (u_var3 + 0x1));
    }
    if ((u_stack6 | u_stack8) == 0x0) {
        if (param_3 != 0x0) {
            *(u8 *) param_3 = 0x0;
        }
    } else {
        if (param_3 != 0x0) {
            u_var1 = u_stack8 -  in_string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe((char *) (param_3 & 0xffff | (u32) param_3 << 0x10),
                             in_string_1,
                             u_var1);
            *(u8 *) ( param_3 + u_var1) = 0x0;
        }
        in_string_1 = (char *) CONCAT22(u_stack6,
                                        u_stack8);
    }
    if (((u_stack10 | u_stack12) != 0x0) && ( in_string_1 <= u_stack12)) {
        if (param_4 != 0x0) {
            u_var1 = u_stack12 -  in_string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe((char *) (param_4 & 0xffff | (u32) param_4 << 0x10),
                             (char *) ((u32) in_string_1 & 0xffff | (u32) in_string_1 << 0x10),
                             u_var1);
            *(u8 *) ( param_4 + u_var1) = 0x0;
        }
        if (param_5 == NULL) {
            return;
        }
        u_var1 = u_var3 - u_stack12;
        if (0xff < u_var1) {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe((char *) ((u32) param_5 & 0xffff | (u32) param_5 << 0x10),
                         (char *) CONCAT22(u_stack10,
                                           u_stack12),
                         u_var1);
        *(u8 *) ( param_5 + u_var1) = 0x0;
        return;
    }
    if (param_4 != 0x0) {
        u_var1 = u_var3 -  in_string_1;
        if (0xff < u_var1) {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe((char *) (param_4 & 0xffff | (u32) param_4 << 0x10),
                         (char *) ((u32) in_string_1 & 0xffff | (u32) in_string_1 << 0x10),
                         u_var1);
        *(u8 *) ( param_4 + u_var1) = 0x0;
    }
    if (param_5 != NULL) {
        *(u8 *) &param_5.style = 0x0;
    }
    return;
}


/*
Unable to decompile 'pass1_1000_4f1a'
Cause:
Low-level Error: Symbol $$undef00000008 extends beyond the end of the address space
*/


// WARNING: Removing unreachable block (ram,0x10004f47)

u16 dos3_call_1000_4f20(void)
{
    code *pcVar1;
    let mut uVar2: u16;
    let mut unaff_BP: i16;
    let mut bVar2: bool;

    bVar2 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)( &DAT_1050_1050,
                      unaff_BP + 0x1);
    if (bVar2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f47)

u16 pass1_1000_4f2e(void)
{
    code *pcVar1;
    let mut uVar2: u16;
    let mut unaff_BP: i16;
    let mut bVar3: bool;

    bVar3 = false;
    pcVar1 = (code *) swi(0x21);
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
    char cVar1;
    code *pcVar2;
    let mut uVar3: u16;
    let mut unaff_BP: i16;
    let mut bVar3: bool;
    let mut uVar5: u32;

    bVar3 = false;
    pcVar2 = (code *) swi(0x21);
    uVar5 = (*pcVar2)( &DAT_1050_1050,
                      unaff_BP + 0x1);
    uVar5 = (char *) (uVar5 >> 0x10);
    uVar5._0_2_ =  uVar5;
    uVar3 =  uVar5;
    if ((bVar3) && (bVar3 =  uVar5 < 0x10,  uVar5 == 0x10)) {
        do {
            cVar1 = *uVar5;
            uVar5 = uVar5 + 0x1;
            if (cVar1 == '\0') {
                goto LAB_1000_4f90;
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

u16 dos3_call_1000_4fbe(u8 param_1)
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
pub fn pass1_1000_5008(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    pass1_1000_5026(0x0,
                    param_1,
                    param_2,
                    param_3);
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1000_5026(mut param_1: i16,
                     mut param_2: u16 ,
                     mut param_3: u16 ,
                     mut param_4: u16 )
{
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut unaff_bp: i16;
    let mut u_stack304: u32;
    u16 local_12c[0x3];
    let mut u_stack294: u16;
    u8 *local_124[0x6];
    let mut i_stack280: i16;
    u8 local_116;
    u8 uStack277;
    char cStack272;
    u8 *puStack270;
    u8 local_108;
    u8 uStack263;
    u8 uStack262;
    u8 auStack261[0x101];
    let mut local_4: u16;
    let mut i_stack2: i16;

    i_stack2 = unaff_bp + 0x1;
    local_4 = SUB42(&DAT_1050_1050,
                    0x0);
    u_stack304 = (char *) CONCAT22(0x1050,
                                   &local_108);
    if (param_1 == 0x0) {
        param_1 = dos3_call_1000_4f94();
    }
    *u_stack304 = (char) param_1 + '@';
    uStack263 = 0x3a;
    puStack270 = auStack261;
    uStack262 = 0x5c;
    uStack277 = 0x47;
    cStack272 = (char) param_1;
    local_12c[0] = SUB42(&DAT_1050_1050,
                         0x0);
    u_stack294 = SUB42(&DAT_1050_1050,
                       0x0);
    dos3_call_set_struct_1000_42de((astruct_811 *) CONCAT22(0x1050,
                                                            &local_116),
                                   (astruct_810 *) CONCAT22(0x1050,
                                                            local_124),
                                   (u16 *) CONCAT22(0x1050,
                                                    local_12c));
    if (i_stack280 == 0x0) {
        u_var1 = str_op_1000_3da4((char *) CONCAT22(0x1050,
                                                    &local_108));
        u_var1 += 0x1;
        u_stack304._0_2_ = param_2;
        u_stack304 = param_3;
        u_var2 = param_3 | param_2;
        if (u_var2 == 0x0) {
            if ( param_4 < u_var1) {
                param_4 = u_var1;
            }
            u_stack304._0_2_ = mem_1000_167a(0x0,
                                             param_4);
            u_stack304 = u_var2;
            if ((u_var2 | u_stack304) == 0x0) {
                PTR_LOOP_1050_5f78 = (u8 *) &PTR_LOOP_1050_000c;
                return;
            }
        }
        if ( param_4 < u_var1) {
            PTR_LOOP_1050_5f78 = (u8 *) ( s_New_failed_in_Op__Op_1050_0020 + 0x2);
        } else {
            unk_str_op_1000_3d3e((char *) CONCAT22(u_stack304,
                                                   u_stack304),
                                 (char *) CONCAT22(0x1050,
                                                   &local_108));
        }
    } else {
        PTR_LOOP_1050_5f78 = (u8 *) ( &PTR_LOOP_1050_000c + 0x1);
        PTR_LOOP_1050_5f88 = local_124[0];
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10005167)

u16 dos3_call_1000_514e(void)
{
    code *pcVar1;
    let mut uVar2: u16;
    let mut unaff_BP: i16;
    let mut bVar2: bool;

    bVar2 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)( &DAT_1050_1050,
                      unaff_BP + 0x1);
    if (bVar2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000518c)

u16 dos3_call_1000_5174(void)
{
    code *pcVar1;
    let mut uVar2: u16;
    let mut unaff_BP: i16;
    let mut bVar2: bool;

    bVar2 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)(unaff_BP + 0x1);
    if (!bVar2) {
        return 0x0;
    }
    pass1_1000_29b5(uVar2);
    return uVar2 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

u16 dos3_calls_1000_5198(mut param_1: u16 ,
                         mut param_2: u16 ,
                         mut param_3: u16 ,
                         mut param_4: u16 )
{
    code *pcVar1;

    pcVar1 = (code *) swi(0x21);
    (*pcVar1)( &DAT_1050_1050);
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0x0) {
        return 0x0;
    }
    pass1_1000_29b5(param_3);
    return param_3 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

u16 dos3_call_1000_51aa(mut param_1: u16 ,
                        mut param_2: u16 ,
                        mut param_3: u16 )
{
    code *pcVar1;
    let mut uStack000a: u16;

    pcVar1 = (code *) swi(0x21);
    (*pcVar1)( &DAT_1050_1050);
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0x0) {
        return 0x0;
    }
    uStack000a = param_3;
    pass1_1000_29b5(param_3);
    return uStack000a & 0xff;
}

u32 pass1_1000_5224(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u32;
    i32 lVar2;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    char cVar11;
    let mut uVar9: u16;

    cVar11 =  param_2 < 0x0;
    if ((bool) cVar11) {
        bVar10 = param_1 != 0x0;
        param_1 = -param_1;
        param_2 = - bVar10 - param_2;
    }
    if ( param_4 < 0x0) {
        cVar11 += '\x01';
        bVar10 = param_3 != 0x0;
        param_3 = -param_3;
        param_4 = - bVar10 - param_4;
    }
    uVar3 = param_1;
    uVar5 = param_3;
    uVar6 = param_2;
    uVar9 = param_4;
    if (param_4 == 0x0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) / (u32) param_3);
    } else {
        do {
            uVar8 = uVar9 >> 0x1;
            uVar5 = uVar5 >> 0x1 |  ((uVar9 & 0x1) != 0x0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 0x1) != 0x0) << 0xf;
            uVar6 = uVar7;
            uVar9 = uVar8;
        } while (uVar8 != 0x0);
        uVar1 = CONCAT22(uVar7,
                         uVar3) / (u32) uVar5;
        iVar4 =  uVar1;
        lVar2 = (u32) param_3 * (uVar1 & 0xffff);
        uVar3 =  ((u32) lVar2 >> 0x10);
        uVar5 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0x0;
    }
    if (cVar11 == '\x01') {
        bVar10 = iVar4 != 0x0;
        iVar4 = -iVar4;
        uVar3 = - bVar10 - uVar3;
    }
    return CONCAT22(uVar3,
                    iVar4);
}

u32 pass1_1000_52be(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    if ((param_4 | param_2) == 0x0) {
        return (u32) param_1 * (u32) param_3;
    }
    return (u32) param_1 * (u32) param_3 & 0xffff
        | (u32) ( ((u32) param_1 * (u32) param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}

u32 pass1_1000_52f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u32;
    i32 lVar2;
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
        bVar12 = param_1 != 0x0;
        param_1 = -param_1;
        param_2 = - bVar12 - param_2;
    }
    uVar11 =  bVar13;
    if ( param_4 < 0x0) {
        bVar13 = param_3 != 0x0;
        param_3 = -param_3;
        param_4 = - bVar13 - param_4;
    }
    uVar3 = param_1;
    uVar4 = param_3;
    uVar8 = param_2;
    uVar9 = param_4;
    if (param_4 == 0x0) {
        iVar5 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) % (u32) param_3);
        iVar6 = 0x0;
        if ( (uVar11 - 0x1) < 0x0) {
            goto LAB_1000_538a;
        }
    } else {
        do {
            uVar10 = uVar9 >> 0x1;
            uVar4 = uVar4 >> 0x1 |  ((uVar9 & 0x1) != 0x0) << 0xf;
            uVar7 = uVar8 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar8 & 0x1) != 0x0) << 0xf;
            uVar8 = uVar7;
            uVar9 = uVar10;
        } while (uVar10 != 0x0);
        uVar1 = CONCAT22(uVar7,
                         uVar3) / (u32) uVar4;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) * (u32) param_3;
        uVar8 =  ((u32) lVar2 >> 0x10);
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
        if (-0x1 <  (uVar11 - 0x1)) {
            goto LAB_1000_538a;
        }
    }
    bVar13 = iVar5 != 0x0;
    iVar5 = -iVar5;
    iVar6 = - bVar13 - iVar6;//
//    LAB_1000_538a:
    return CONCAT22(iVar6,
                    iVar5);
}

u32 pass1_1000_5390(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u32;
    i32 lVar2;
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
    if (param_4 == 0x0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) / (u32) param_3);
    } else {
        do {
            uVar5 = uVar8 >> 0x1;
            uVar9 = uVar9 >> 0x1 |  ((uVar8 & 0x1) != 0x0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 0x1) != 0x0) << 0xf;
            uVar8 = uVar5;
            uVar6 = uVar7;
        } while (uVar5 != 0x0);
        uVar1 = CONCAT22(uVar7,
                         uVar3) / (u32) uVar9;
        iVar4 =  uVar1;
        lVar2 = (u32) param_3 * (uVar1 & 0xffff);
        uVar3 =  ((u32) lVar2 >> 0x10);
        uVar8 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0x0;
    }
    return CONCAT22(uVar3,
                    iVar4);
}

u32 pass1_1000_53f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 )
{
    let mut uVar1: u32;
    i32 lVar2;
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
    if (param_4 == 0x0) {
        iVar6 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) % (u32) param_3);
        iVar7 = 0x0;
    } else {
        do {
            uVar5 = uVar4 >> 0x1;
            uVar10 = uVar10 >> 0x1 |  ((uVar4 & 0x1) != 0x0) << 0xf;
            uVar8 = uVar9 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar9 & 0x1) != 0x0) << 0xf;
            uVar4 = uVar5;
            uVar9 = uVar8;
        } while (uVar5 != 0x0);
        uVar1 = CONCAT22(uVar8,
                         uVar3) / (u32) uVar10;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) * (u32) param_3;
        uVar9 =  ((u32) lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar10 = uVar9 + uVar3;
        if (((CARRY2(uVar9,
                     uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
            bVar11 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar10 = (uVar10 - param_4) -  bVar11;
        }
        iVar6 = -(uVar4 - param_1);
        iVar7 = - (uVar4 - param_1 != 0x0) - ((uVar10 - param_2) -  (uVar4 < param_1));
    }
    return CONCAT22(iVar7,
                    iVar6);
}

i16 pass1_1000_545a(mut param_1: u32,
                    mut param_2: u32)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    u8 bVar4;
    u8 *pbVar5;
    u8 *pbVar6;

    pbVar6 = (u8 *) param_2;
    pbVar5 = (u8 *) param_1;
    bVar4 = 0xff;
    do {
        do {
            if (bVar4 == 0x0) {
                goto LAB_1000_5499;
            }
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 0x1;
            bVar4 = *pbVar1;
            bVar3 = *pbVar5;
            pbVar5 = pbVar5 + 0x1;
        } while (bVar3 == bVar4);
        bVar2 = bVar4 + 0xbf + (-((u8) (bVar4 + 0xbf) < 0x1a) & 0x20U) + 0x41;
        bVar3 += 0xbf;
        bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20U) + 0x41;
    } while (bVar4 == bVar2);
    bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;//
//    LAB_1000_5499:
    return  (char) bVar4;
}

u16 *pass1_1000_54a0(mut param_1: u32,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    let mut puVar1: *mut u16;
    u8 uVar2;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut iVar8: i16;

    if (param_3 != 0x0) {
        iVar8 =  (param_1 >> 0x10);
        uVar5 = - (u16 *) param_1;
        uVar6 = param_3;
        if (uVar5 != 0x0) {
            uVar6 = (uVar5 - param_3 & - (uVar5 < param_3)) + param_3;
            uVar5 = param_3 - uVar6;
        }
        uVar3 = param_2 & 0xff | param_2 << 0x8;
        puVar7 = (u16 *) param_1;
        for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar7;
            puVar7 = puVar7 + 0x1;
            *puVar1 = uVar3;
        }
        for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar2 = (u8) (param_2 & 0xff), uVar6 != 0x0; uVar6 -= 0x1) {
            puVar1 = puVar7;
            puVar7 = (u16 *) ( puVar7 + 0x1);
            *(u8 *) puVar1 = uVar2;
        }
        if (uVar5 != 0x0) {
            for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
                puVar1 = puVar7;
                puVar7 = puVar7 + 0x1;
                *puVar1 = uVar3;
            }
            for (uVar6 =  ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
                puVar1 = puVar7;
                puVar7 = (u16 *) ( puVar7 + 0x1);
                *(u8 *) puVar1 = uVar2;
            }
        }
    }
    return (u16 *) param_1;
}
pub fn pass1_1000_54e8(u8 *param_1,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: i16,
                     mut param_6: u16 )
{
    let mut i_var1: i16;

    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        ((code) param_1)();
    }
    return;
}
pub fn pass1_1000_5512(u8 *param_1,
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
        b_var1 = param_3 == 0x0;
        param_3 += -0x1;
        param_4 -= b_var1;
        if (param_4 < 0x0) {
            break;
        }
        u_stack4 = param_2;
        ((code) param_1)();
    }
}

/*
Unable to decompile 'pass1_1000_55b1'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/
pub fn pass1_1000_5586(code param_1,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: i16,
                     mut param_6: u16 )
{
    let mut i_var1: i16;

    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        ((code) param_1)();
    }
}

pub fn ret_op_1000_55ac(void) -> u32
{
}
pub fn exit_1000_25f2(i16 a, u16 b, u16 c, u16 d) {

}


//dos3_call_1000_23ea( param_4,
//                         &DAT_1050_1050,
//                        0x0,
//                         &DAT_1050_1050);
pub fn dos3_call_1000_23ea(u8* a, u16 b, u16 c, u16 d) {

}
