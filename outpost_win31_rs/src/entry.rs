//
// Created by cyrex on 2022-05-24.
//

// #include "types.h"
// #include "structs_2.h"
// #include "func_ptrs.h"
// #include "entry.h"
// #include "utils.h"
// #include "globals.h"
// #include "sys_api.h"
// #include "block_1000.h"

use crate::block_1000::block_1000_2000::{
    fn_ptr_op_1000_24cd, init_1000_23be, pass1_1000_24db, pass1_1000_25a8, pass1_1000_262c,
    pass1_1000_27d6, pass1_1000_2913, poss_str_op_1000_28dc,
};
use crate::block_1000::block_1000_5000::{dos3_call_1000_23ea, ret_op_1000_55ac};
use crate::globals::{
    DAT_1050_5f82, DAT_1050_5f87, HINSTANCE16_1050_5f4c, PTR_LOOP_1050_5f48, PTR_LOOP_1050_5f4a,
    PTR_LOOP_1050_5f4e, PTR_LOOP_1050_5f50, PTR_LOOP_1050_5f7e, PTR_LOOP_1050_5f84,
    PTR_LOOP_1050_63fe, WIN_VERSION_1050_5f80, REG_DI, REG_SI,
};
use crate::sys_api::{
    FatalAppExit16, FatalExit, GetVersion16, InitApp16, InitTask16, LockSegment16, WaitEvent16,
};
use crate::utils::CONCAT22;
use std::ptr;

pub unsafe fn entry(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    param_4: *mut u8,
    param_5: *mut u8,
    param_6: u16,
) -> *mut c_char {
    // paVar1: *mut astruct_822;
    let mut paVar1: *mut astruct_822;
    let mut piVar2: *mut c_char;
    let mut uVar4: u16;
    // string_var4: *mut c_char;
    let mut string_var4: *mut c_char;
    let mut iVar5: i16;
    let mut string_var5: *mut c_char;
    let mut iVar6: i16;
    //    let mut unaff_SI: *mut u8;
    let mut string_var7: *mut c_char;
    // let mut unaff_DI: HISTANCE16;
    // paVar8: *mut c_char;
    let mut string_var8: *mut c_char;
    let mut bVar9: bool;
    let mut win_version: u32 = 0u32;
    let mut u32_var11: u32;
    let mut uVar10: u32 = 0u32;
    let mut u32_var12: u32;
    // paVar13: *mut astruct_825;
    let mut pstruct_825_var13: *mut astruct_825 = ptr::null_mut();
    let mut string_var1: *mut c_char;

    u32_var11 = CONCAT22(param_6, PTR_LOOP_1050_5f84 as u16);
    loop {
        InitTask16(NULL);
        PTR_LOOP_1050_5f84 = u32_var11;
        if param_3 != 0x0 {
            PTR_LOOP_1050_5f7e = 0x1050;
            // bVar9 = param_5 <  0xff00;
            // param_5 = param_5 + 0x100;
            if bVar9 {
                //  &DAT_1050_1050
                PTR_LOOP_1050_5f50 = 0x1050;
                PTR_LOOP_1050_5f48 = param_5;
                PTR_LOOP_1050_5f4a = REG_SI;
                HINSTANCE16_1050_5f4c = REG_DI;
                PTR_LOOP_1050_5f4e = param_4;
                LockSegment16(0xffff);
                //                PTR_LOOP_1050_5f52 =  (u32_var11 >> 0x10);
                PTR_LOOP_1050_5f84 = u32_var11;
                win_version = GetVersion16();
                //                PTR_LOOP_1050_5f52 =  (u32_var11 >> 0x10);
                PTR_LOOP_1050_5f84 = u32_var11;
                //                PTR_LOOP_1050_5f80 =  CONCAT11( u32_var10,
                //                                                      (u32_var10 >> 0x8));
                WIN_VERSION_1050_5f80 = win_version;
                let mut func_ptr_3: code8 = swi(0x21);
                u32_var12 = u32_var11;
                u32_var11 = func_ptr_3();
                //                PTR_LOOP_1050_5f52 =  (u32_var12 >> 0x10);
                PTR_LOOP_1050_5f84 = u32_var12;
                //                DAT_1050_5f82 = CONCAT11( u32_var11,
                //                                           (u32_var11 >> 0x8));
                DAT_1050_5f82 = u32_var11 as u16;
                DAT_1050_5f87 = 0;
                WaitEvent16(0x0);
                PTR_LOOP_1050_5f84 = u32_var11;
                param_3 = InitApp16(HINSTANCE16_1050_5f4c);
                PTR_LOOP_1050_5f84 = u32_var11;
                if param_3 != 0x0 {
                    break;
                }
            }
        }
        //        param_3 = CONCAT11( ( param_3 >> 0x8),
        //                           0xff);
        param_3 = ((param_3 >> 0x8) << 0x8) | 0xff;
        pass1_1000_24db(param_3 as u16);
        PTR_LOOP_1050_5f84 = u32_var11;
    }
    // &DAT_1050_1050
    dos3_call_1000_23ea(param_4, 0x1050, 0x0, 0x1050);
    PTR_LOOP_1050_5f84 = u32_var11;
    //  s_tile2_bmp_1050_1538
    pass1_1000_262c(u32_var11, 0x238d, 0x1538);
    PTR_LOOP_1050_5f84 = u32_var11;
    pass1_1000_27d6(u32_var11 as u16);
    win_version = ret_op_1000_55ac();
    uVar4 = win_version as u16;
    init_1000_23be(param_5, (uVar10 >> 0x10) as u16);
    fn_ptr_op_1000_24cd(uVar4);
    // TODO
    // paVar13 =  CONCAT22(uVar4,
    //                                    0x15);
    pass1_1000_25a8();
    pass1_1000_2913(0x15);
    string_var4 = poss_str_op_1000_28dc(pstruct_825_var13);
    if string_var4.is_null() == false {
        iVar5 = 0x9;
        if string_var4[0] == 'M' {
            iVar5 = 0xf;
        }
        string_var4 = string_var4 + iVar5;
        iVar6 = 0x22;
        string_var8 = string_var4;
        loop {
            if iVar6 == 0x0 {
                break;
            }
            iVar6 += -0x1;
            paVar1 = string_var8;
            string_var8 = string_var8 + 1;
            if !(paVar1.field0_0x0 != '\r') {
                break;
            }
        } //while (paVar1.field0_0x0 != '\r');
        (string_var8 -1) = '\0';
    }
    FatalAppExit16(0x0, string_var4);
    FatalExit();
    string_var7 = PTR_LOOP_1050_63fe;
    loop {
        string_var1 = string_var7;
        string_var7 = string_var7 + 1;
        string_var5 = string_var7;
        string_var5 = string_var1 + 1;
        if (*string_var1 == param_2) || (string_var5.is_null()) {
            return string_var5;
        }
        iVar6 = -0x1;
        loop {
            if iVar6 == 0x0 {
                break;
            }
            iVar6 += -0x1;
            piVar2 = string_var7;
            string_var7 = (string_var7 + 1);
            if *piVar2 == '\0' {
                break;
            }
        }
        // } while piVar2 != '\0';
    }
}
