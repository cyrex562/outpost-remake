// #include "file_ops_1.h"

// #include "file_ops_2.h"
// #include "file_ops_3.h"
// #include "fn_ptr_ops/fn_ptr_ops_3.h"
// #include "fn_ptr_ops/fn_ptr_ops_6.h"
// #include "fn_ptr_ops/fn_ptr_ops_7.h"
// #include "globals.h"
// #include "op_int.h"
// #include "op_winapi.h"
// #include "op_windef.h"
// #include "string_ops.h"
// #include "struct_ops/struct_ops_1.h"
// #include "structs/structs_0xx/structs_9x.h"
// #include "structs/structs_3xx/structs_30x.h"
// #include "sys_ops/sys_ops_10.h"
// #include "sys_ops/sys_ops_12.h"
// #include "sys_ops/sys_ops_2.h"
// #include "sys_ops/sys_ops_6.h"
// #include "sys_ops/sys_ops_9.h"
// #include "unk/unk_14.h"
// #include "unk/unk_18.h"
// #include "unk/unk_6.h"
// #include "utils.h"

// #include <lmcons.h>
// #include <stdbool.h>

use std::ptr::null_mut;
use crate::fn_ptr_ops::fn_ptr_ops_6_c::pass1_1008_766e;
use crate::structs::structs_3xx::structs_30x_h::Struct307;
use crate::unk::unk_18_c::pass1_1038_75ca;
use libc::c_void;

pub fn file_1038_774e(globals: &mut Globals,
                      mut param_1: *mut Struct307,
                      param_2: u32,
                      param_3: *mut u8,
                      param_4: u16)

{
    let mut u16_var1: u16 = 0;
//    Struct307 *iVar2;
    let mut BVar2: BOOL16;
    let mut iVar3: i16;
    let mut u32_var4: u32;
    let mut u16_var6: u16 = 0;
    let mut local_8: u16 = 0;
    let mut local_6: u16 = 0u16;
    let mut local_4: u16 = 0u16;
    let mut puVar5: *mut u32;

    if globals.dat_1050_0312 < 0x2
    {
        return;
    }
//    param_1  = param_1;
    param_1  = param_1.field_0x4 as *mut Struct307;
    puVar5 = param_1 | param_1;
    pass1_1008_766e(param_2,
                    puVar5,
                    param_4,
                    SEG_1008,
                    param_3);
    if puVar5 != null_mut()
    {
        // uVar1 = (param_1 >> 0x10);
        u32_var4 = param_2;
        // uVar6 = (param_2 >> 0x10);
        BVar2 = read_file_1008_7dee(u32_var4,
                                    u16_var6,
                                    &param_1.field_0x8,
                                    0x0,
                                    u16_var1,
                                    0x4,
                                    SEG_1008);

        iVar3
            = file_1008_77cc(param_2,
                             (param_1 | &param_1.field_0xe),
                             param_3,
                             SEG_1008,
                             param_4);
        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &local_4, 0x0, param_4, 0x2, SEG_1008);

        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &local_6, 0x0, param_4, 0x2, SEG_1008);

        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &local_8, 0x0, param_4, 0x2, SEG_1008);

        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &param_1.field_0x16, 0x0, u16_var1, 0x4, SEG_1008);

        BVar2 = read_file_1008_7bc8(
            param_2,
            (param_1 | &param_1.field_0x1a_addr_offset),
            SEG_1008,
            param_4);

        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &param_1.field_0x20, 0x0, u16_var1, 0x4, SEG_1008);
        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &param_1.field_0x24, 0x0, u16_var1, 0x2, SEG_1008);

        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &param_1.field_0x26, 0x0, u16_var1, 0x2, SEG_1008);

        BVar2 = read_file_1008_7dee(
            u32_var4, u16_var6, &param_1.field_0x28, 0x0, u16_var1, 0x2, SEG_1008);

        // if (((((BVar2 != 0x0)
        //        && (
        //            iVar3 != 0x0))
        //       && (
        //           BVar2 != 0x0))
        //      && (BVar2 != 0x0
        //        && (BVar2 != 0x0)))
        //     && ((
        //     BVar2 != 0x0
        //         && ((,
        //              BVar2 != 0x0
        //
        //                && (
        //                    BVar2 != 0x0))))))
        //    && ((
        //     BVar2 != 0x0
        //        && ((
        //             BVar2 != 0x0
        //               && (
        //                   BVar2 != 0x0)))))
            if BVar2 != 0x0
        {
            param_1.field_0xc  = local_4;
            u32_var4 = switch_1008_72bc(u32_var4, u16_var6, local_6);
            param_1.field_0x12 = u32_var4;
            param_1.field_0x14 = local_8;
            return;
        }
    }
    globals.dat_1050_0310 = 0x06d2;
}


pub unsafe fn pass1_1038_7b20(param_1: *mut u32, param_2: u32, param_3: u32) -> u16

{
    let mut u16_var1: u32;
    let mut b_var2: BOOL16;
    let mut u16_var3: u16;
    let mut u32_var4: u32;
    let mut u16_var5: u16;
    let mut u32_var6: u32;
    let mut u16_var7: u16;
    let mut u16_var8: u16;
    let mut u16_var9: u32;
    let mut u16_var10: [u8;8] = [0;8];
    let mut u16_var11: u16;

    b_var2 = write_to_file_1008_7cac(param_2, param_3);
    if b_var2 != 0x0
    {
        u16_var6 = (*param_1 + 0x8);
        u16_var5    = (param_2 >> 0x10) as u16;
        u16_var11  = u16_var6;
        b_var2    = write_to_file_1008_7e1c(param_2, u16_var5, &u16_var6, param_3, 0x2, SEG_1008);
        if b_var2 != 0x0
        {
            pass1_1008_5784(str_var1(param_3, u16_var10), *param_1);
            loop
            {
                u16_var9 = pass1_1008_5b12(u16_var10, param_3);
                if(u16_var9 == 0x0)
                {
                    u16_var3    = (param_1 >> 0x10);
                    u16_var1    = (param_1 + 0x4);
                    u16_var6 = (u16_var1 + 0x8);
                    u16_var11  = u16_var6;
                    b_var2    = write_to_file_1008_7e1c(param_2, u16_var5, &u16_var11, param_3, 0x2, SEG_1008);
                    if(b_var2 == 0x0)
                    {
                        return 0x0;
                    }
                    pass1_1008_5784(str_var1(param_3, u16_var10), *(param_1 + 0x4));
                    loop
                    {
                        u32_var4    = pass1_1008_5b12(u16_var10, param_3);
                        u16_var7 = u32_var4 as u16;
                        if(u32_var4 == 0x0)
                        {
                            return 0x1;
                        }
                        pass1_1030_b768(u32_var4, param_2, param_3);
                        u16_var8 = (u32_var4 >> 0x10) as u16;
                        if !(u32_var4 != 0) { break;}
                    }
                    // while(u32_var4 != 0x0);
                    return 0x0;
                }
                pass1_1038_75ca(u16_var9, param_2, u16_var9, param_3);
                u16_var9 = (u16_var9 >> 0x10);
                if !(u16_var9 != 0) {
                    break;
                }
            }
            // while(u16_var9 != 0x0);
        }
    }
    return 0x0;
}


pub unsafe fn read_file_1038_7c02(globals: &mut Globals,
                       param_1: *mut u32,
                        param_2: u32,
                        param_3: u16,
                       param_4: u16) -> u16

{
    let mut ppv_var1: *mut *mut c_void;
    let mut b_var2: BOOL16;
    let mut u16_var3: u16;
    let mut u_var4: u16;
    let mut dx_var1: *mut u8;
    let mut pu8_var5: *mut u8;
    let mut dx_var1_00: *mut u8;
    let mut ss_var1: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut local_12: [u16;0x2];
    let mut uStack14: u32;
    let mut local_4: u16;

    if(globals.dat_1050_0312 < 0x2)
    {
        return 0x1;
    }
    uVar6 = param_2;
    // uVar8 = (param_2 >> 0x10) as u16;
    read_file_1008_7cfe(uVar6, uVar8, 0x17, SEG_1008, ss_var1);
    if((param_3 != 0x0) && (b_var2 = read_file_1008_7dee(uVar6, uVar8, &local_4, 0x0, ss_var1, 0x2, SEG_1008), b_var2 != 0x0))
    {
        while(local_4 != 0x0)
        {
            uVar7   = 0x2a;
            u16_var3   = local_4;
            local_4 = local_4 - 0x1;
            uVar9 = param_2;
            mem_op_1000_179c(0x2a, param_4, 0);
            pu8_var5 = (param_4 | u16_var3);
            if(pu8_var5 == 0x0)
            {
                u16_var3  = 0x0;
                pu8_var5 = 0x0;
            }
            else
            {
                struct_1038_6520(str_var1(param_4, u16_var3));
            }
            uVar10   = (uVar9 >> 0x10);
            uStack14 = str_var1(pu8_var5, u16_var3);
            file_1038_774e(
              NULL, str_var1(pu8_var5, u16_var3), str_var1(uVar9, uVar7), pu8_var5, ss_var1);
            if(u16_var3 == 0x0)
            {
                return 0x0;
            }
            ppv_var1 = (*param_1 + 0x4);
            (**ppv_var1)(SEG_1000, *param_1, (*param_1 >> 0x10), uStack14, (uStack14 >> 0x10), uVar10);
            param_4 = dx_var1;
        }
        local_4 = local_4 - 0x1;
        b_var2   = read_file_1008_7dee(uVar6, uVar8, local_12, 0x0, ss_var1, 0x2, SEG_1008);
        if(b_var2 != 0x0)
        {
            while(true)
            {
                if(local_12[0] == 0x0)
                {
                    return 0x1;
                }
                uVar7       = 0x14;
                u16_var3       = local_12[0];
                local_12[0] = local_12[0] - 0x1;
                uVar9 = param_2;
                mem_op_1000_179c(0x14, param_4, 0);
                pu8_var5 = (param_4 | u16_var3);
                if(pu8_var5 == 0x0)
                {
                    u16_var3  = 0x0;
                    pu8_var5 = 0x0;
                }
                else
                {
                    pass1_1030_ae6c(str_var1(param_4, u16_var3));
                }
                uVar10 = (uVar9 >> 0x10);
                u_var4  = u16_var3;
                file_1030_b836(
                  str_var1(pu8_var5, u16_var3), str_var1(uVar9, uVar7), pu8_var5, ss_var1);
                if(u_var4 == 0x0){
                    break;}
                uVar7   = (param_1 >> 0x10);
                uVar9   = (param_1 + 0x4);
                ppv_var1 = ((param_1 + 0x4) + 0x4);
                (**ppv_var1)(SEG_1030, uVar9, (uVar9 >> 0x10), u16_var3, pu8_var5, uVar10);
                param_4 = dx_var1_00;
            }
            return 0x0;
        }
    }
    return 0x0;
}


pub fn pass1_1038_5e16(globals: &mut Globals,
                     param_1: u32,
                     param_2: u32,
                     param_3: i16,
                     param_4: u16,
                    param_5: u16)

{
    let mut BVar1: BOOL16;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_14: [u32;0x2];
    let mut local_c: u32;
    let mut pu_stack6: *mut u32;

    pass1_1030_16d6(param_1, param_2, param_5);
    if(param_3 != 0x0)
    {
        u_var4    = (param_1 >> 0x10);
        iVar3    = param_1;
        pu_var2   = (iVar3 + 0xc);
        pu_stack6 = pu_var2;
        pass1_1008_7898(param_2, pu_var2, pu_var2, param_4, SEG_1008, param_5);
        if(pu_var2 != 0x0)
        {
            local_14[0] = (iVar3 + 0x10);
            uVar5       = param_2;
            uVar6       = (param_2 >> 0x10);
            BVar1       = write_to_file_1008_7e1c(uVar5, uVar6, local_14, param_5, 0x4, SEG_1008);
            if(BVar1 != 0x0)
            {
                local_c = (iVar3 + 0x18);
                BVar1         = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                if(BVar1 != 0x0)
                {
                    local_c = (iVar3 + 0x1a);
                    BVar1         = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                    if(BVar1 != 0x0)
                    {
                        local_c = str_var1(local_c, (iVar3 + 0x1c));
                        BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                        if(BVar1 != 0x0)
                        {
                            local_c = *(iVar3 + 0x1e);
                            BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x4, SEG_1008);
                            if(BVar1 != 0x0)
                            {
                                local_c = local_c & 0xffff0000 | (iVar3 + 0x22);
                                BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                if(BVar1 != 0x0)
                                {
                                    local_c = local_c & 0xffff0000 | (iVar3 + 0x24);
                                    BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                    if(BVar1 != 0x0)
                                    {
                                        BVar1 = write_to_file_1008_7e1c(uVar5, uVar6, iVar3 + 0x26, u_var4, 0x94, SEG_1008);
                                        if(BVar1 != 0x0)
                                        {
                                            BVar1 = write_to_file_1008_7e1c(uVar5, uVar6, iVar3 + 0x14e, u_var4, 0x54, SEG_1008);
                                            if(BVar1 != 0x0)
                                            {
                                                BVar1 = write_to_file_1008_7e1c(uVar5, uVar6, iVar3 + 0x1a2, u_var4, 0x54, SEG_1008);
                                                if(BVar1 != 0x0)
                                                {
                                                    write_to_file_1030_32e4(*(iVar3 + 0x1f6), param_2, param_5);
                                                    BVar1 = pass1_1008_7c2a(param_2, (iVar3 + 0x1fa), SEG_1008);
                                                    if(BVar1 != 0x0)
                                                    {
                                                        local_c = local_c & 0xffff0000 | (iVar3 + 0x1fe);
                                                        BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                        if(BVar1 != 0x0)
                                                        {
                                                            local_c = *(iVar3 + 0x200);
                                                            BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x4, SEG_1008);
                                                            if(BVar1 != 0x0)
                                                            {
                                                                local_c = local_c & 0xffff0000 | (iVar3 + 0x204);
                                                                BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                if(BVar1 != 0x0)
                                                                {
                                                                    local_c = local_c & 0xffff0000 | (iVar3 + 0x206);
                                                                    BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                    if(BVar1 != 0x0)
                                                                    {
                                                                        local_c = local_c & 0xffff0000 | (iVar3 + 0x208);
                                                                        BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                        if(BVar1 != 0x0)
                                                                        {
                                                                            local_c = local_c & 0xffff0000 | (iVar3 + 0x20a);
                                                                            BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                            if(BVar1 != 0x0)
                                                                            {
                                                                                local_c = local_c & 0xffff0000 | (iVar3 + 0x20c);
                                                                                BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                                if(BVar1 != 0x0)
                                                                                {
                                                                                    local_c = local_c & 0xffff0000 | (iVar3 + 0x20e);
                                                                                    BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                                    if(BVar1 != 0x0)
                                                                                    {
                                                                                        local_c = local_c & 0xffff0000 | (iVar3 + 0x214);
                                                                                        BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x2, SEG_1008);
                                                                                        if(BVar1 != 0x0)
                                                                                        {
                                                                                            local_c = (iVar3 + 0x216);
                                                                                            BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, &local_c, param_5, 0x4, SEG_1008);
                                                                                            if(BVar1 != 0x0)
                                                                                            {
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
            }
        }
        globals.dat_1050_0310 = 0x6d0;
    }
    return;
}


pub fn file_1038_6118(globals: &mut Globals,
                    param_1: u32,
                    param_2: u32,
                    param_3: i16,
                    param_4: *mut u8,
                   param_5: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u32;
    let mut BVar3: BOOL16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
//    Struct429 *iVar9;
//    u16          uVar8;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut SVar11: SEGPTR;
   let mut paStack1046: *mut Struct18;
    let mut uStack1042: u16;
    let mut local_408: [u8;400] = [0;400];
    let mut local_8: u16;
    let mut local_6: u32;

    file_1030_1730(param_1, param_2);
    if(param_3 == 0x0)
    {
        return;
    }
    local_6 = 0x0;
    pu_var2  = &local_6;
    file_1008_7548(param_2, str_var1(param_5, pu_var2), SEG_1008, param_5);
    if(pu_var2 != 0x0)
    {
//        uVar8            = (param_1 >> 0x10);
//        iVar9            = param_1;
        param_1.field_0xc = local_6;
        uVar9            = param_2;
        uVar10           = (param_2 >> 0x10);
        BVar3            = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x10, 0x0, uVar8, 0x4, SEG_1008);
        if(((((BVar3 != 0x0) && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x18, 0x0, uVar8, 0x2, SEG_1008), BVar3 != 0x0))
             && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x1a_addr_offset, 0x0, uVar8, 0x2, SEG_1008), BVar3 != 0x0))
            && ((BVar3 = read_file_1008_7dee(uVar9, uVar10, &local_8, 0x0, param_5, 0x2, SEG_1008), BVar3 != 0x0 && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x1e, 0x0, uVar8, 0x4, SEG_1008), BVar3 != 0x0))))
           && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x22, 0x0, uVar8, 0x2, SEG_1008), BVar3 != 0x0))
        {
            param_1.field_0x1c_addr_base = local_8;
            BVar3             = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x24, 0x0, uVar8, 0x2, SEG_1008);
            if((BVar3 != 0x0) && (u_var4 = read_file_1008_7dee(uVar9, uVar10, &param_1.field_0x26, 0x0, uVar8, 0x94, SEG_1008), u_var4 != 0x0))
            {
                if(globals.dat_1050_0312 < 0x2)
                {
                    uVar5  = 0x54;
                    SVar11 = 0x54;
                    mem_op_1000_179c(0x54, param_4, 0);
                    paStack1046 = str_var1(param_4, u_var4);
                    BVar3       = read_file_1008_7dee(uVar9, uVar10, u_var4, uVar5, param_4, SVar11, SEG_1008);
                    if(BVar3 == 0x0)
                    {
                    // LAB_1038_626a:
                        globals.dat_1050_0310 = 0x6d2;
                        fn_ptr_1000_17ce(paStack1046, SEG_1000);
                        return;
                    }
                    uStack1042 = 0x0;
                    loop
                    {
                        uVar5                               = switch_1008_72bc(uVar9, uVar10, uStack1042);
                        uVar1                               = (uStack1042 * 0x4 + u_var4 + 0x2);
                        (&iVar9.field_0x14e + uVar5 * 0x4) = (uStack1042 * 0x4 + u_var4);
                        (&iVar9.field_0x150 + uVar5 * 0x4) = uVar1;
                        uStack1042                          = uStack1042 + 0x1;
                        if !(uStack1042 < 0x15){
                            break;}
                    }
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, u_var4, 0x0, param_4, 0x54, SEG_1008);
                    if(BVar3 == 0x0) {}
                        //goto LAB_1038_626a;
                    uStack1042 = 0x0;
                    loop
                    {
                        uVar5                               = switch_1008_72bc(uVar9, uVar10, uStack1042);
                        puVar7                              = (uStack1042 * 0x4 + u_var4 + 0x2);
                        (&iVar9.field_0x1a2 + uVar5 * 0x4) = (uStack1042 * 0x4 + u_var4);
                        (&iVar9.field_0x1a4 + uVar5 * 0x4) = puVar7;
                        uStack1042                          = uStack1042 + 0x1;
                        if !(uStack1042 < 0x15){
                            break;}
                    }
                    fn_ptr_1000_17ce(paStack1046, SEG_1000);
                    param_4 = puVar7;
                }
                else
                {
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x14e, 0x0, uVar8, 0x54, SEG_1008);
                    if(BVar3 == 0x0)
                    {
                        globals.dat_1050_0310 = 0x6d2;
                        return;
                    }
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x1a2, 0x0, uVar8, 0x54, SEG_1008);
                    if(BVar3 == 0x0)
                    {
                        globals.dat_1050_0310 = 0x6d2;
                        return;
                    }
                }
                read_file_1030_33f0(NULL, iVar9.field_0x1f6, param_2);
                puVar6 = local_408;
                read_file_1008_7c6e(uVar9, uVar10, str_var1(param_5, puVar6), SEG_1008);
                if(puVar6 != 0x0) {
                    u_var4 = str_op_1008_60e8(str_var1(param_5, local_408), param_4);
                    iVar9.offset_field_0x1fa = u_var4;
                    iVar9.segment_field_0x1fc = param_4;
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x1fe, 0x0, uVar8, 0x2, SEG_1008);
                    if (((((BVar3 != 0x0) &&
                           (BVar3 = read_file_1008_7dee(uVar9, uVar10, CONCAT11((param_1 >> 0x8) + '\x02', param_1),
                                                        0x0, uVar8, 0x4, SEG_1008), BVar3 != 0x0))
                          &&
                          (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x204, 0x0, uVar8, 0x2, SEG_1008),
                                  BVar3 != 0x0))
                         && ((
                            (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x206, 0x0, uVar8, 0x2, SEG_1008),
                                    BVar3 != 0x0 &&
                                    (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x208, 0x0, uVar8, 0x2,
                                                                 SEG_1008), BVar3 != 0x0))
                            && ((BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x20a, 0x0, uVar8, 0x2,
                                                             SEG_1008),
                                    BVar3 != 0x0
                                    &&
                                    ((BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x20c, 0x0, uVar8, 0x2,
                                                                  SEG_1008),
                                            BVar3 != 0x0 &&
                                            (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x20e, 0x0, uVar8,
                                                                         0x2, SEG_1008), BVar3 != 0x0))))))))
                        && ((globals.dat_1050_0312 < 0x2
                            || ((BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x214, 0x0, uVar8, 0x2, SEG_1008),
                                 BVar3 != 0x0 && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9.field_0x216, 0x0, uVar8, 0x4, SEG_1008), BVar3 != 0x0))))))
                    {
                        return;
                    }
                    globals.dat_1050_0310 = 0x6d0;
                    return;
                }
            }
        }
    }
    globals.dat_1050_0310 = 0x6d2;
}


pub fn pass1_1030_de7c(globals: &mut Globals, param_1: u32, param_2: u32, param_3: u16)

{
    let mut BVar1: BOOL16;
    let mut local_10: [u32;3];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        local_10[0] = (param_1 + 0x20);
        BVar1       = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), local_10, param_3, 0x4, SEG_1008);
        if(BVar1 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}


pub fn pass1_1030_dec4(
    globals: &mut Globals,
    param_1: u32,
                     param_2: u32,
                     param_3: i16,
                     param_4: *mut u8,
                     param_5: u16)

{
    let mut BVar1: BOOL16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(((param_3 != 0x0) && (0x1 < globals.dat_1050_0312)) && (BVar1 = read_file_1008_7dee(param_2, (param_2 >> 0x10), param_1 + 0x20, 0x0, (param_1 >> 0x10), 0x4, SEG_1008), BVar1 == 0x0))
    {
        globals.dat_1050_0310 = 0x6d2;
        return;
    }
}


pub fn pass1_1030_d61c(globals: &mut Globals, param_1: u32, param_2: u32, param_3: u16)

{
    let mut BVar1: BOOL16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut local_1a: u32;
    let mut local_16: *mut u8;
    let mut local_14: u16;
    let mut local_12: [u32;3];
    let mut iStack4: i16;

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        // for(iStack4 = 0x0; u_var4 = param_2, uVar5 = (param_2 >> 0x10), iStack4 < 0xa; iStack4 = iStack4 + 0x1)
        iStack4 = 0;
        u_var4 = param_2;
        uVar5 = param_2 >> 0x10;
        while iStack4 < 0xa
        {
            uVar3       = (param_1 >> 0x10);
            iVar2       = param_1;
            local_12[0] = (iVar2 + iStack4 * 0xc + 0x20);
            BVar1       = write_to_file_1008_7e1c(u_var4, uVar5, local_12, param_3, 0x4, SEG_1008);
            if(BVar1 == 0x0){}
                //goto LAB_1030_d701;
            local_14 = (iVar2 + iStack4 * 0xc + 0x24);
            BVar1    = write_to_file_1008_7e1c(u_var4, uVar5, &local_14, param_3, 0x2, SEG_1008);
            if(BVar1 == 0x0){}
                //goto LAB_1030_d701;
            local_16 = (iVar2 + iStack4 * 0xc + 0x26);
            BVar1    = write_to_file_1008_7e1c(u_var4, uVar5, &local_16, param_3, 0x2, SEG_1008);
            if(BVar1 == 0x0){}
                //goto LAB_1030_d701;
            local_1a = (iVar2 + iStack4 * 0xc + 0x28);
            BVar1    = write_to_file_1008_7e1c(u_var4, uVar5, &local_1a, param_3, 0x4, SEG_1008);
            if(BVar1 == 0x0){}
                //goto LAB_1030_d701;
                iStack4 = iStack4 + 0x1
        }
        local_16 = globals.dat_1050_5812;
        BVar1    = write_to_file_1008_7e1c(u_var4, uVar5, &local_16, param_3, 0x2, SEG_1008);
        if(BVar1 != 0x0)
        {
            return;
        }
    // LAB_1030_d701:
        globals.dat_1050_0310 = 0x6d0;
    }
    return;
}


pub fn  pass1_1030_d72e(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16)

{
    let mut uVar1: u16;
    let mut BVar2: BOOL16;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut iStack10: i16;
    let mut local_8: u32;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 == 0x0)
    {
        return;
    }
    iStack10 = 0x0;
    while(true)
    {
        u_var4 = param_2;
        uVar5 = (param_2 >> 0x10);
        if(0x9 < iStack10)
        {
            if((0x3 < globals.PTR_LOOP_1050_0312) && (BVar2 = read_file_1008_7dee(u_var4, uVar5, &PTR_LOOP_1050_5812, 0x0, SEG_1050, 0x2, SEG_1008), BVar2 == 0x0))
            {
                globals.dat_1050_0310 = 0x6d2;
                return;
            }
            return;
        }
        BVar2 = read_file_1008_7dee(u_var4, uVar5, &local_8, 0x0, param_5, 0x4, SEG_1008);
        if(BVar2 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d2;
            return;
        }
        BVar2 = read_file_1008_7dee(u_var4, uVar5, &local_4, 0x0, param_5, 0x2, SEG_1008);
        if(BVar2 == 0x0){
            break;}
        iVar3          = iStack10 * 0xc + param_1;
        (iVar3 + 0x20) = local_8;
        (iVar3 + 0x22) = local_8;
        uVar1          = switch_1008_72bc(u_var4, uVar5, local_4);
        (iVar3 + 0x24) = uVar1;
        if(globals.dat_1050_0312 < 0x2)
        {
            iVar3          = iStack10 * 0xc + param_1;
            (iVar3 + 0x26) = 0x3;
            (iVar3 + 0x28) = 0x0;
        }
        else
        {
            BVar2 = read_file_1008_7dee(u_var4, uVar5, &local_4, 0x0, param_5, 0x2, SEG_1008);
            if(BVar2 == 0x0)
            {
                globals.dat_1050_0310 = 0x6d2;
                return;
            }
            BVar2 = read_file_1008_7dee(u_var4, uVar5, &local_8, 0x0, param_5, 0x4, SEG_1008);
            if(BVar2 == 0x0)
            {
                globals.dat_1050_0310 = 0x6d2;
                return;
            }
            iVar3          = iStack10 * 0xc + param_1;
            (iVar3 + 0x26) = local_4;
            (iVar3 + 0x28) = local_8;
        }
        iStack10 = iStack10 + 0x1;
    }
    globals.dat_1050_0310 = 0x6d2;
    return;
}


pub fn  pass1_1030_c230(param_1: u32, param_2: u32, param_3: u16)

{
    let mut BVar1: BOOL16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut local_10: [u32;0x2];
    let mut local_8: [u16;0x3];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        u_var2       = (param_1 >> 0x10);
        local_10[0] = (param_1 + 0x20);
        uVar3       = (param_2 >> 0x10);
        BVar1       = write_to_file_1008_7e1c(param_2, uVar3, local_10, param_3, 0x4, SEG_1008);
        if(BVar1 != 0x0)
        {
            local_8[0] = (param_1 + 0x24);
            BVar1      = write_to_file_1008_7e1c(param_2, uVar3, local_8, param_3, 0x2, SEG_1008);
            if(BVar1 != 0x0)
            {
                return;
            }
        }
        globals.dat_1050_0310 = 0x6d0;
    }
    return;
}


pub fn  pass1_1030_c29c(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16)

{
    let mut uVar1: u16;
    let mut BVar2: BOOL16;
    let mut uVar3: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 != 0x0)
    {
        uVar1 = (param_1 >> 0x10);
        uVar3 = (param_2 >> 0x10);
        BVar2 = read_file_1008_7dee(param_2, uVar3, param_1 + 0x20, 0x0, uVar1, 0x4, SEG_1008);
        if(BVar2 != 0x0)
        {
            BVar2 = read_file_1008_7dee(param_2, uVar3, param_1 + 0x24, 0x0, uVar1, 0x2, SEG_1008);
            if(BVar2 != 0x0)
            {
                return;
            }
        }
        globals.dat_1050_0310 = 0x6d2;
    }
    return;
}


pub fn  pass1_1030_c84e(param_1: u32, param_2: u32, param_3: u16) -> bool

{
    let mut BVar1: BOOL16;
    let mut local_c: [u16;0x5];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        local_c[0] = (param_1 + 0x20);
        BVar1      = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), local_c, param_3, 0x2, SEG_1008);
        if(BVar1 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}


pub fn  pass1_1030_c894(param_1: u32, param_2: u32, param_3: bool, param_4: *mut u8, param_5: u16) -> bool

{
    let mut BVar1: BOOL16;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 != 0x0)
    {
        BVar1 = read_file_1008_7dee(param_2, (param_2 >> 0x10), &local_4, 0x0, param_5, 0x2, SEG_1008);
        if(BVar1 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d2;
            return BVar1;
        }
        (param_1 + 0x20) = local_4;
        param_3          = 0x1;
    }
    return param_3;
}


pub fn  pass1_1030_b768(param_1: u32, param_2: u32, param_3: u32)

{
    let mut uVar1: u32;
    let mut BVar2: BOOL16;
    let mut iVar3: i16;
    let mut pu_var4: *mut u8;
    let mut dx_var1: u16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_22: [u16;0x4];
    let mut local_1a: [u8;a] = [0;a];
    let mut local_10: u32;
    let mut puStack12: *mut u8;
    let mut uStack10: u16;
    let mut local_8: [u16;0x3];

    uVar6    = (param_1 >> 0x10);
    iVar5    = param_1;
    local_10 = *(iVar5 + 0x4);
    uVar7    = param_2;
    uVar8    = (param_2 >> 0x10);
    BVar2    = write_to_file_1008_7e1c(uVar7, uVar8, &local_10, param_3, 0x4, SEG_1008);
    if((BVar2 != 0x0) && (iVar3 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | (iVar5 + 0x8), SEG_1008, param_3), iVar3 != 0x0))
    {
        local_8[0] = (iVar5 + 0xe);
        BVar2      = write_to_file_1008_7e1c(uVar7, uVar8, local_8, param_3, 0x2, SEG_1008);
        if(BVar2 != 0x0)
        {
            uVar1       = (iVar5 + 0x10);
            local_22[0] = (uVar1 + 0x8);
            local_10    = local_10 & 0xffff0000 | local_22[0];
            BVar2       = write_to_file_1008_7e1c(uVar7, uVar8, local_22, param_3, 0x2, SEG_1008);
            if(BVar2 == 0x0)
            {
                return;
            }
            pass1_1008_5784(str_var1(param_3, local_1a), *(iVar5 + 0x10));
            loop
            {
                pu_var4 = local_1a;
                pass1_1008_5b12(pu_var4, param_3);
                if((dx_var1 | pu_var4) == 0x0)
                {
                    return;
                }
                puStack12 = pu_var4;
                uStack10  = dx_var1;
                pass1_1038_75ca(str_var1(dx_var1, pu_var4), param_2, pu_var4, param_3);
                if !(pu_var4 != 0x0){break;}
            }
            // while(pu_var4 != 0x0);
            return;
        }
    }
    globals.dat_1050_0310 = 0x6d0;
    return;
}


pub fn file_1030_b836(param_1: *mut Struct401, param_2: u32, param_3: *mut u8, param_4: u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
//    u16          uVar3;
//    Struct401 *iVar4;
    let mut BVar4: BOOL16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut puVar8: *mut u8;
    let mut dx_var1: *mut u8;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut uVar13: u16;
    let mut local_12: [u16;0x7];
    let mut local_4: u16;

//    iVar4  = param_1;
    param_1  = param_1.field_0x4;
//    uVar3  = (param_1 >> 0x10);
    uVar9  = param_2;
    uVar10 = (param_2 >> 0x10);
    BVar4  = read_file_1008_7dee(uVar9, uVar10, param_1, 0x0, uVar3, 0x4, SEG_1008);
    if(((BVar4 == 0x0) || (BVar4 = read_file_1008_7bc8(param_2, (param_1 & 0xffff0000 | &param_1.field_0x8), SEG_1008, param_4), BVar4 == 0x0)) || (BVar4 = read_file_1008_7dee(uVar9, uVar10, &local_4, 0x0, param_4, 0x2, SEG_1008), BVar4 == 0x0))
    {
        globals.dat_1050_0310 = 0x6d2;
    }
    else
    {
        param_1.field_0xe = local_4;
        BVar4            = read_file_1008_7dee(uVar9, uVar10, local_12, 0x0, param_4, 0x2, SEG_1008);
        if(BVar4 != 0x0)
        {
            while(true)
            {
                if(local_12[0] == 0x0)
                {
                    return;
                }
                uVar11      = 0x2a;
                uVar5       = local_12[0];
                local_12[0] = local_12[0] - 0x1;
                uVar12 = param_2;
                mem_op_1000_179c(0x2a, param_3, 0);
                puVar8 = (param_3 | uVar5);
                if(puVar8 == 0x0)
                {
                    uVar6  = 0x0;
                    puVar8 = 0x0;
                }
                else
                {
                    uVar6 = uVar5;
                    struct_1038_6520(str_var1(param_3, uVar5));
                }
                uVar13 = (uVar12 >> 0x10);
                uVar7  = uVar6;
                file_1038_774e(NULL,
                               str_var1(puVar8, uVar6),
                               str_var1(uVar12, uVar11),
                               puVar8,
                               param_4);
                if(uVar7 == 0x0)
                {
                    break;
                }
                puVar1  = param_1.field_0x10;
                ppcVar2 = (*param_1.field_0x10 + 0x4);
                // TODO
                // (**ppcVar2)(SEG_1038, puVar1, (puVar1 >> 0x10), uVar6, puVar8, uVar13, uVar5);
                param_3 = dx_var1;
            }
        }
    }
}


pub fn  pass1_1030_7418(param_1: *mut Struct731, param_2: u32, param_3: i16, param_4: u16)

{
    let mut uVar1: u32;
//    Struct731 *iVar2;
    let mut iVar3: i16;
    let mut BVar4: BOOL16;
    let mut puVar5: *mut u8;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut u_stack62: u16;
    let mut local_2a: [u16;0x2];
    let mut local_26: [u8;e] = [0;e];
    let mut local_18: u32;
    let mut local_14: [u32;0x2];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: [u16;0x2];

    pass1_1030_16d6(param_1, param_2, param_4);
    if(param_3 == 0x0)
    {
        return;
    }
//    iVar2 = param_1;
    param_1 = &param_1.field_0xc;
    iVar3 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | ZEXT24(param_1), SEG_1008, param_4);
    if(iVar3 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    uVar6   = (param_1 >> 0x10);
    local_c = param_1.field_0x12;
    uVar7   = param_2;
    uVar8   = (param_2 >> 0x10);
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    local_6[0] = param_1.field_0x14;
    BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    local_18 = param_1.field_0x16;
    BVar4    = write_to_file_1008_7e1c(uVar7, uVar8, &local_18, param_4, 0x4, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7954(param_2, param_1.field_0x1e, BVar4, SEG_1008, param_4);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_2, param_1.field_0x22, SEG_1008, param_4);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_2, param_1.field_0x26, SEG_1008, param_4);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    local_a = param_1.field_0x2a;
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, &local_a, param_4, 0x4, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    local_c = param_1.field_0x32;
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    local_c = param_1.field_0x34;
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_79f0(param_2, param_1.field_0x36, SEG_1008, param_4);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    if(param_1.field_0x3a == 0x0)
    {
        local_18 = local_18 & 0xffff0000;
    }
    else
    {
        uVar1    = param_1.field_0x3a;
        local_18 = local_18 & 0xffff0000 | (uVar1 + 0x8);
    }
    local_6[0] = local_18;
    BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
    if(BVar4 == 0x0)
    {
        globals.dat_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_5784(str_var1(param_4, local_26), param_1.field_0x3a);
    while(true)
    {
        puVar5 = local_26;
        pass1_1008_5b12(puVar5, param_4);
        local_14[0] = str_var1(dx_var1, puVar5);
        if((dx_var1 | puVar5) == 0x0)
        {
            if(param_1.field_0x3e == 0x0)
            {
                u_stack62 = 0x0;
            }
            else
            {
                uVar1    = param_1.field_0x3e;
                u_stack62 = (uVar1 + 0x8);
            }
            local_2a[0] = u_stack62;
            BVar4       = write_to_file_1008_7e1c(uVar7, uVar8, local_2a, param_4, 0x2, SEG_1008);
            if(BVar4 == 0x0)
            {
                globals.dat_1050_0310 = 0x6d0;
                return;
            }
            pass1_1008_5784(str_var1(param_4, local_26), param_1.field_0x3e);
            while(true)
            {
                puVar5 = local_26;
                pass1_1008_5b12(puVar5, param_4);
                if((dx_var1_00 | puVar5) == 0x0)
                {
                    return;
                }
                local_18 = local_18 & 0xffff0000 | (puVar5 + 0x4);
                BVar4    = write_to_file_1008_7e1c(uVar7, uVar8, &local_18, param_4, 0x2, SEG_1008);
                if(BVar4 == 0x0)
                {
                    globals.dat_1050_0310 = 0x6d0;
                    return;
                }
                local_14[0] = local_14[0] & 0xffff0000 | (puVar5 + 0x6);
                BVar4       = write_to_file_1008_7e1c(uVar7, uVar8, local_14, param_4, 0x2, SEG_1008);
                if(BVar4 == 0x0)
                {
                    globals.dat_1050_0310 = 0x6d0;
                    return;
                }
                local_c = (puVar5 + 0x8);
                BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, SEG_1008);
                if(BVar4 == 0x0){
                    break;}
                local_c = (puVar5 + 0xa);
                BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, SEG_1008);
                if(BVar4 == 0x0)
                {
                    globals.dat_1050_0310 = 0x6d0;
                    return;
                }
                local_6[0] = (puVar5 + 0xc);
                BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
                if(BVar4 == 0x0)
                {
                    globals.dat_1050_0310 = 0x6d0;
                    return;
                }
            }
            globals.dat_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (puVar5 + 0x4);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
        if(BVar4 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0x6);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
        if(BVar4 == 0x0){
            break;}
        local_6[0] = (local_14[0] + 0x8);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
        if(BVar4 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0xa);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
        if(BVar4 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0xc);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, SEG_1008);
        if(BVar4 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return;
        }
    }
    globals.dat_1050_0310 = 0x6d0;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn  file_1030_778c(param_1: *mut Struct387, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16)

{
    let mut lVar1 = 0i32;
    let mut ppcVar2: *mut *mut c_void;
//    Struct387 *iVar3;
    let mut BVar3: BOOL16;
    let mut iVar6: i16;
    long        *plVar7;
    let mut puVar8: *mut u32;
    let mut dx_var1: u16;
    let mut uVar9: u16;
    let mut puVar10: *mut u8;
    let mut dx_var1_00: u16;
    let mut iVar4: *mut Struct389;
    let mut iVar5: *mut Struct391;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut local_56: [u16;0x2];
    let mut uStack82: u16;
    let mut paStack74: *mut Struct99;
    let mut local_46: [u16;0x2];
    let mut local_42: [u16;0x2];
    let mut local_3e: [u32;0x3];
    let mut paStack50: *mut Struct99;
    let mut local_2e: u16;
    let mut paStack44: *mut Struct99;
    let mut local_28: [u16;0x2];
    let mut local_24: [u16;0x2];
    let mut local_20: [u16;0x9];
    let mut uStack14: u16;
    let mut local_4: u16;
    let mut uVar5: *mut Struct388;
    let mut uVar8: *mut Struct390;

    file_1030_1730(param_1, param_2);
    if(param_3 != 0x0)
    {
//        iVar3 = param_1;
        param_1 = &param_1.field_0xc;
        BVar3 = read_file_1008_7bc8(param_2, (param_1 & 0xffff0000 | ZEXT24(param_1)), SEG_1008, param_5);
        if(BVar3 != 0x0)
        {
            uVar13 = param_2;
            uVar14 = (param_2 >> 0x10);
            BVar3  = read_file_1008_7dee(uVar13, uVar14, &local_4, 0x0, param_5, 0x2, SEG_1008);
            if(BVar3 != 0x0)
            {
                uVar11            = (param_1 >> 0x10);
                param_1.field_0x12 = local_4;
                BVar3             = read_file_1008_7dee(uVar13, uVar14, &local_4, 0x0, param_5, 0x2, SEG_1008);
                if(BVar3 != 0x0)
                {
                    param_1.field_0x14 = local_4;
                    BVar3             = read_file_1008_7dee(uVar13, uVar14, &param_1.field_0x16, 0x0, uVar11, 0x4, SEG_1008);
                    if(BVar3 != 0x0)
                    {
                        plVar7 = (param_1 & 0xffff0000 | &param_1.field_0x1e);
                        file_1008_76e4(param_2, plVar7, SEG_1008, param_5, param_4);
                        if((((plVar7 != 0x0) && (iVar6 = file_1008_77cc(param_2, (param_1 & 0xffff0000 | &param_1.field_0x22), param_4, SEG_1008, param_5), iVar6 != 0x0))
                            && (iVar6 = file_1008_77cc(param_2, (param_1 & 0xffff0000 | &param_1.field_0x26), param_4, SEG_1008, param_5), iVar6 != 0x0))
                           && (BVar3 = read_file_1008_7dee(uVar13, uVar14, &param_1.field_0x2a, 0x0, uVar11, 0x4, SEG_1008), BVar3 != 0x0))
                        {
                            if(param_1.field_0x2a != 0x0)
                            {
                                lVar1 = param_1.field_0x2a;
                                pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, lVar1, (lVar1 >> 0x10));
                                param_1.field_0x2e = BVar3;
                                param_1.field_0x30 = param_4;
                            }
                            if(globals.dat_1050_0312 < 0x2)
                            {
                                return;
                            }
                            BVar3 = read_file_1008_7dee(uVar13, uVar14, &param_1.field_0x32, 0x0, uVar11, 0x2, SEG_1008);
                            if((BVar3 != 0x0) && (BVar3 = read_file_1008_7dee(uVar13, uVar14, &param_1.field_0x34, 0x0, uVar11, 0x2, SEG_1008), BVar3 != 0x0))
                            {
                                puVar8 = (param_1 & 0xffff0000 | &param_1.field_0x36);
                                pass1_1008_766e(param_2, puVar8, param_5, SEG_1008, param_4);
                                if((puVar8 != 0x0) && (BVar3 = read_file_1008_7dee(uVar13, uVar14, local_20, 0x0, param_5, 0x2, SEG_1008), BVar3 != 0x0))
                                {
                                    // for(uStack14 = 0x0; uStack14 < local_20[0]; uStack14 = uStack14 + 0x1)
                                    for loop_counter_1 in 0 .. local_20[0]
                                    {
                                        local_3e[0] = globals._PTR_LOOP_1050_68a2;
                                        paStack50 = pass1_1000_07fc(SEG_1000, globals.u32_ptr_1050_68a2);
                                        uVar9 = (paStack50 >> 0x10);
                                        uVar5       = paStack50;
                                        puVar10     = (uVar9 | uVar5);
                                        if puVar10 == 0x0
                                        {
                                            paStack44 = 0x0;
                                        }
                                        else
                                        {
                                            paStack50.fld0_addr_table
                                              = addr_table_1008_380a[36];//0x389a;
                                            uVar5.fld2_segment  = SEG_1008;
                                            uVar5.field_0x4     = 0x0;
                                            uVar5.field_0x6     = 0x0;
                                            uVar5.field_0x8     = 0x0;
                                            uVar5.field_0xa     = 0x0;
                                            uVar5.field_0xc     = 0x0;
                                            paStack50.fld0_addr_table
                                              = addr_table_1018_56ce;//0x56ce;
                                            uVar5.fld2_segment  = SEG_1018;
                                            paStack44            = paStack50;
                                        }
                                        BVar3 = read_file_1008_7dee(uVar13, uVar14, local_28, 0x0, param_5, 0x2, SEG_1008);
                                        if(((BVar3 == 0x0) || (BVar3 = read_file_1008_7dee(uVar13, uVar14, local_24, 0x0, param_5, 0x2, SEG_1008), BVar3 == 0x0))
                                           || ((BVar3 = read_file_1008_7dee(uVar13, uVar14, &local_2e, 0x0, param_5, 0x2, SEG_1008),
                                                BVar3 == 0x0
                                                  || ((BVar3 = read_file_1008_7dee(uVar13, uVar14, paStack44 + 0xa, 0x0, (paStack44 >> 0x10), 0x2, SEG_1008),
                                                       BVar3 == 0x0 || (BVar3 = read_file_1008_7dee(uVar13, uVar14, paStack44 + 0xc, 0x0, (paStack44 >> 0x10), 0x2, SEG_1008), BVar3 == 0x0)))))) {}
                                            //goto LAB_1030_77be;
                                        uVar12           = (paStack44 >> 0x10);
                                        iVar4            = paStack44;
                                        iVar4.field_0x4 = local_28[0];
                                        iVar4.field_0x6 = local_24[0];
                                        iVar4.field_0x8 = local_2e;
                                        if(iVar3.field_0x3a == 0x0)
                                        {
                                            uVar9 = local_2e;
                                            mem_op_1000_179c(0xc, puVar10, 0);
                                            paStack50 = str_var1(puVar10, uVar9);
                                            if((puVar10 | uVar9) == 0x0)
                                            {
                                                iVar3.field_0x3a = 0x0;
                                            }
                                            else
                                            {
                                                set_struct_1008_574a(
                                                  str_var1(puVar10, uVar9));
                                                iVar3.field_0x3a         = uVar9;
                                                (&iVar3.field_0x3a + 0x2) = dx_var1;
                                            }
                                        }
                                        ppcVar2 = (*iVar3.field_0x3a + 0x8);
                                        (unsafe { **ppcVar2 })();
                                    }
                                    BVar3 = read_file_1008_7dee(uVar13, uVar14, local_56, 0x0, param_5, 0x2, SEG_1008);
                                    if(BVar3 != 0x0)
                                    {
                                        uStack82 = 0x0;
                                        while(true) {
                                            if (local_56[0] <= uStack82) {
                                                return;
                                            }
                                            paStack44 =  globals.u32_ptr_1050_68a2;
                                            paStack50 = pass1_1000_07fc(SEG_1000, globals.u32_ptr_1050_68a2);
                                            uVar9 = (paStack50 >> 0x10);
                                            uVar8 =  paStack50;
                                            puVar10 = (uVar9 | uVar8);
                                            if (puVar10 == 0x0) {
                                                paStack74 =  0x0;
                                            } else {
                                                paStack50.fld0_addr_table
                                                  = addr_table_1008_380a[36];//0x389a;
                                                uVar8.fld2_segment  = SEG_1008;
                                                uVar8.field_0x4     = 0x0;
                                                uVar8.field_0x6     = 0x0;
                                                uVar8.field_0x8     = 0x0;
                                                uVar8.field_0xa     = 0x0;
                                                uVar8.field_0xc     = 0x0;
                                                paStack50.fld0_addr_table
                                                  = addr_table_1018_56ce;//0x56ce;
                                                uVar8.fld2_segment  = SEG_1018;
                                                paStack74            = paStack50;
                                            }
                                            BVar3 = read_file_1008_7dee(uVar13, uVar14, local_46, 0x0, param_5, 0x2, SEG_1008);
                                            if((((BVar3 == 0x0) || (BVar3 = read_file_1008_7dee(uVar13, uVar14, local_42, 0x0, param_5, 0x2, SEG_1008), BVar3 == 0x0))
                                                || (BVar3 = read_file_1008_7dee(uVar13, uVar14, local_3e, 0x0, param_5, 0x2, SEG_1008), BVar3 == 0x0))
                                               || ((BVar3 = read_file_1008_7dee(uVar13, uVar14, paStack74 + 0xa, 0x0, (paStack74 >> 0x10), 0x2, SEG_1008),
                                                    BVar3 == 0x0 || (BVar3 = read_file_1008_7dee(uVar13, uVar14, paStack74 + 0xc, 0x0, (paStack74 >> 0x10), 0x2, SEG_1008), BVar3 == 0x0)))){
                                                // break;
                                            }
                                            uVar12           = (paStack74 >> 0x10);
                                            iVar5            = paStack74;
                                            iVar5.field_0x4 = local_46[0];
                                            iVar5.field_0x6 = local_42[0];
                                            iVar5.field_0x8 = local_3e[0];
                                            if(iVar3.field_0x3e == 0x0)
                                            {
                                                mem_op_1000_179c(0xc, puVar10, 0);
                                                paStack50 = str_var1(puVar10, local_3e[0]);
                                                if((puVar10 | local_3e[0]) == 0x0)
                                                {
                                                    iVar3.field_0x3e = 0x0;
                                                }
                                                else
                                                {
                                                    set_struct_1008_574a(
                                                      str_var1(puVar10, local_3e[0]));
                                                    iVar3.field_0x3e         = local_3e[0];
                                                    (&iVar3.field_0x3e + 0x2) = dx_var1_00;
                                                }
                                            }
                                            ppcVar2 = (*iVar3.field_0x3e + 0x8);
                                            (unsafe { **ppcVar2 })();
                                            uStack82 = uStack82 + 0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    // LAB_1030_77be:
        globals.dat_1050_0310 = 0x6d2;
    }
    return;
}


pub fn  pass1_1030_5c1a(param_1: u32, param_2: u32, param_3: u16) -> bool

{
    let mut BVar1: BOOL16;

    BVar1 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar1 != 0x0)
    {
        BVar1 = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), param_1, (param_1 >> 0x10), 0x24, SEG_1008);
        if(BVar1 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}


pub fn  read_file_1030_5c52(param_1: u32, param_2: u32, param_3: u16, param_4: u16) -> bool

{
    let mut BVar1: BOOL16;
    let mut u_var2: u16;

    u_var2 = (param_2 >> 0x10);
    read_file_1008_7cfe(param_2, u_var2, 0x9, SEG_1008, param_4);
    if(param_3 != 0x0)
    {
        BVar1 = read_file_1008_7dee(param_2, u_var2, param_1, 0x0, (param_1 >> 0x10), 0x24, SEG_1008);
        if(BVar1 == 0x0)
        {
            globals.dat_1050_0310 = 0x6d2;
            return BVar1;
        }
        param_3 = 0x1;
    }
    return param_3;
}


pub fn  pass1_1030_5dbe(param_1: u32, param_2: u32, param_3: u16, param_4: u16)

{
    let mut uVar1: u32;
    let mut u_var2: u32;
    let mut uVar3: u16;
    let mut BVar4: BOOL16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut local_c: [u16;0x5];

    uVar3 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if(uVar3 != 0x0)
    {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        BVar4 = pass1_1008_7c2a(param_2, (iVar6 + 0x10), SEG_1008);
        if((BVar4 != 0x0) && (uVar1 = *(iVar6 + 0x10), iVar5 = write_to_file_1008_7b4c(param_2, uVar1 & 0xffff0000 | (uVar1 + 0x4), SEG_1008, param_4), iVar5 != 0x0))
        {
            u_var2      = (iVar6 + 0x10);
            local_c[0] = (u_var2 + 0xa);
            uVar3      = (param_2 >> 0x10);
            BVar4      = write_to_file_1008_7e1c(param_2, uVar3, local_c, param_4, 0x2, SEG_1008);
            if(BVar4 != 0x0)
            {
                u_var2 = (iVar6 + 0x10);
                if((u_var2 + 0xa) == 0x0)
                {
                    return;
                }
                u_var2 = (iVar6 + 0x10);
                uVar7 = (u_var2 >> 0x10);
                iVar6 = u_var2;
                u_var2 = (iVar6 + 0xc);
                BVar4 = write_to_file_1008_7e1c(param_2, uVar3, u_var2, (u_var2 >> 0x10), ((iVar6 + 0xa) * 0x2), SEG_1008);
                if(BVar4 != 0x0)
                {
                    return;
                }
            }
        }
        globals.dat_1050_0310 = 0x6d0;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn  file_1030_5e70(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16)

{
    let mut uVar1: u32;
    let mut u_var2: u32;
    let mut uVar3: u16;
    let mut pu_var4: *mut u8;
    let mut uVar5: u16;
    let mut BVar6: BOOL16;
    let mut uVar7: u16;
    let mut puVar8: *mut u8;
    let mut iVar9: i16;
    let mut unaff_DI: i16;
    let mut uVar10: u16;
    let mut puVar11: *mut u16;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut uStack1034: u32;
    let mut local_402: [u8;400] = [0;400];

    iVar12 = param_1;
    uVar13 = (param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if(param_3 != 0x0)
    {
        if(globals._PTR_LOOP_1050_5f2c == 0x0)
        {
            globals.dat_1050_5f2c      = mem_op_1000_160a(param_4, SEG_1000);
            globals.dat_1050_5f2e      = param_4;
        }
        else
        {
        }
        uVar3      = fn_ptr_op_1000_1708(0x10, 0x0, 0x1, globals.dat_1050_5f2c, globals.dat_1050_5f2e, SEG_1000);
        uStack1034 = str_var1(globals.PTR_LOOP_1050_5f2e, uVar3);
        puVar8     = (globals.PTR_LOOP_1050_5f2e | uVar3);
        if(puVar8 == 0x0)
        {
            (iVar12 + 0x10) = 0x0;
        }
        else
        {
            puVar11         = pass1_1008_3e38(str_var1(globals.PTR_LOOP_1050_5f2e, uVar3 + 0x4));
            puVar8          = (puVar11 >> 0x10);
            (iVar12 + 0x10) = uStack1034;
        }
        pu_var4 = local_402;
        uVar3  = param_2;
        uVar14 = (param_2 >> 0x10);
        read_file_1008_7c6e(uVar3, uVar14, str_var1(param_5, pu_var4), SEG_1008);
        if(pu_var4 != 0x0)
        {
            uVar5           = str_op_1008_60e8(str_var1(param_5, local_402), puVar8);
            puVar11         = (iVar12 + 0x10);
            unsafe { *puVar11        = uVar5 };
            (puVar11 + 0x2) = puVar8;
            uVar1           = *(iVar12 + 0x10);
            BVar6           = read_file_1008_7bc8(param_2, (uVar1 & 0xffff0000 | (uVar1 + 0x4)), SEG_1008, param_5);
            if(BVar6 != 0x0)
            {
                u_var2 = (iVar12 + 0x10);
                BVar6 = read_file_1008_7dee(uVar3, uVar14, u_var2 + 0xa, 0x0, (u_var2 >> 0x10), 0x2, SEG_1008);
                if(BVar6 != 0x0)
                {
                    u_var2  = (iVar12 + 0x10);
                    uVar10 = (u_var2 >> 0x10);
                    iVar9  = u_var2;
                    if((iVar9 + 0xa) == 0x0)
                    {
                    // LAB_1030_5fb7:
                        puVar11 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2f, param_5, puVar8, unaff_DI);
                        pass1_1018_04ca(puVar11, *(iVar12 + 0x4));
                        return;
                    }
                    uVar5 = (iVar9 + 0xa) * 0x2;
                    uVar7 = uVar5;
                    mem_op_1000_179c(uVar5, puVar8, 0);
                    u_var2 = (iVar12 + 0x10);
                    uVar10        = (u_var2 >> 0x10);
                    iVar9         = u_var2;
                    (iVar9 + 0xc) = uVar7;
                    (iVar9 + 0xe) = puVar8;
                    u_var2         = (iVar12 + 0x10);
                    u_var2         = (u_var2 + 0xc);
                    BVar6         = read_file_1008_7dee(uVar3, uVar14, u_var2, 0x0, (u_var2 >> 0x10), uVar5, SEG_1008);
                    if(BVar6 != 0x0) {}
                        //goto LAB_1030_5fb7;
                }
            }
        }
        globals.dat_1050_0310 = 0x6d2;
    }
    return;
}


pub fn  read_file_1030_4e70(param_1: u32, param_2: *mut u32, param_3: *mut *mut u8,
    param_4: i32, param_5: u16) -> u16

{
    let mut uVar1: u16;
    let  mut  HVar2: HFILE16;
    let mut uVar3: u16;
    let mut ss_var1: u16;
    let mut u_var4: u32;
    let mut lVar5 = 0i32;
    let mut pbStack60: *mut u8;
    let mut lStack56 = 0i32;
    let mut uStack20: u32;

    unsafe { *param_3 = 0x0 };
    unsafe { *param_2 = 0x0 };
    if(param_4 != 0x0)
    {
        u_var4   = pass1_1030_5164(param_1, param_4, ss_var1);
        param_5 = (u_var4 >> 0x10);
        uVar1   = dos_int21_find_file_1000_51aa(&stack0xfffe);
        if(uVar1 == 0x0)
        {
            unsafe { *param_2 = uStack20 };
            HVar2    = _lopen16(SEG_1000, 0x0);
            if(HVar2 != 0xffff)
            {
                lVar5           = mem_op_1000_0a48(0x1, unsafe { *param_2 }, (unsafe { *param_2 } >> 0x10), globals.dat_1050_5f2c, SEG_1000);
                param_5         = (lVar5 >> 0x10);
                param_3         = lVar5;
                (param_3 + 0x2) = param_5;
                if((param_5 | param_3) != 0x0)
                {
                    lStack56 = WIN16_hread(SEG_1000, (SEGPTR)*param_2, str_var1(unsafe { *param_3 }, (unsafe { *param_2 } >> 0x10)));
                    uVar3    = (lStack56 >> 0x10);
                    _lclose16(LAST_SEGMENT);
                    pbStack60 = unsafe { *param_3 };
                    while(lStack56 != 0x0)
                    {
                        if(((unsafe { *pbStack60 } + 0x608b) & 0x20) == 0x0)
                        {
                            unsafe { *pbStack60 = *pbStack60  + 0x80 };
                        }
                        pbStack60 = (pbStack60 & 0xffff0000 | (pbStack60 + 0x1));
                        lStack56  = lStack56 + -0x1;
                    }
                    return uVar3;
                }
            }
        }
    }
    return param_5;
}


pub fn  pass1_1030_5044(param_1: u32, param_2: u16, param_3: u16)

{
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    long *plVar3;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut pcVar8: *mut c_char;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uStack28: u32;
    let mut uStack24: u16;
    let mut uStack22: u32;
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut local_a = 0i32;
    let mut local_6: *mut c_char;
    let mut uVar9: u32;

    plVar3                      = &local_a;
    globals.dat_1050_5f2e      = read_file_1030_4e70(param_1,
                                                 str_var1(param_2, plVar3), str_var1(param_2, &local_6), s_bldgops_dat_1050_5708, param_3);
    pcVar2                      = local_6;
    if(plVar3 != 0x0)
    {
        uVar10 = param_1;
        uVar11 = (param_1 >> 0x10);
        pcVar8 = local_6;
        pass1_1030_4e34(uVar10, uVar11, local_a, local_6);
        u_var4 = pcVar8;
        if(globals._PTR_LOOP_1050_5f2c == 0x0)
        {
            globals.dat_1050_5f2c = mem_op_1000_160a(globals.PTR_LOOP_1050_5f2e, SEG_1000);
        }
        else
        {
        }
        uVar5    = fn_ptr_op_1000_1708(u_var4 * 0xae, 0x0, 0x1, globals.dat_1050_5f2c, globals.dat_1050_5f2e, SEG_1000);
        uVar9    = uVar5;
        uStack28 = str_var1(globals.PTR_LOOP_1050_5f2e, uVar5);
        if((globals.PTR_LOOP_1050_5f2e | uVar5) == 0x0)
        {
            (uVar10 + 0x15c) = 0x0;
        }
        else
        {
            pass1_1000_5586(0x51f0, SEG_1030, u_var4, 0xae, uVar5, globals.dat_1050_5f2e);
            *(uVar10 + 0x15c) = uStack28;
            uVar9             = uStack28;
        }
        uVar6 = uVar9;
        pass1_1030_4dbc(param_1, local_6, pcVar8 & 0xffff);
        uStack22 = str_var1(dx_var1, uVar6);
        // for(uStack24 = 0x0; uStack24 < u_var4; uStack24 = uStack24 + 0x1)
        for uStack24 in 0 .. u_var4
        {
            uVar1 = (uVar10 + 0x15e);
            iVar7 = (uVar10 + 0x15c) + uStack24 * 0xae;
            pass1_1030_4c52(uVar10, uVar11, str_var1(uVar1, iVar7), uStack22, uVar1, param_2);
            pass1_1030_4dbc(param_1, 0x0, 0x0);
            uStack22 = str_var1(dx_var1_00, iVar7);
        }
        uStack12 = (pcVar2 >> 0x10);
        uStack14 = pcVar2;
        if((uStack12 | uStack14) != 0x0)
        {
            call_fn_ptr_1000_0dc6(uStack14, uStack12, SEG_1000);
        }
    }
    return;
}


pub fn  pass1_1030_56f6(param_1: u32, param_2: u32, param_3: u16, param_4: u16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u32;
    let mut uVar3: u32;
    let mut u_var4: u16;
    let mut BVar5: BOOL16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_e: [u16;0x3];
    let mut local_8: [u16;0x2];
    let mut iStack4: i16;

    u_var4 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if(u_var4 != 0x0)
    {
        uVar8      = (param_1 >> 0x10);
        iVar7      = param_1;
        local_e[0] = (iVar7 + 0x10);
        u_var4      = param_2;
        uVar9      = (param_2 >> 0x10);
        BVar5      = write_to_file_1008_7e1c(u_var4, uVar9, local_e, param_4, 0x2, SEG_1008);
        if(BVar5 != 0x0)
        {
            uVar3      = (iVar7 + 0x10);
            local_8[0] = (uVar3 + 0x2);
            BVar5      = write_to_file_1008_7e1c(u_var4, uVar9, local_8, param_4, 0x2, SEG_1008);
            if((BVar5 != 0x0) && (uVar3 = (iVar7 + 0x10), BVar5 = pass1_1008_7c2a(param_2, (uVar3 + 0x4), SEG_1008), BVar5 != 0x0))
            {
                uVar3      = (iVar7 + 0x10);
                local_8[0] = (uVar3 + 0x1a);
                BVar5      = write_to_file_1008_7e1c(u_var4, uVar9, local_8, param_4, 0x2, SEG_1008);
                if(BVar5 != 0x0)
                {
                    // for(iStack4 = 0x0; uVar3 = (iVar7 + 0x10), pi_var1 = (uVar3 + 0x1a), *pi_var1 != iStack4 && iStack4 <= *pi_var1; iStack4 = iStack4 + 0x1)
                    // TODO
                    loop
                    {
                        uVar3 = (iVar7 + 0x10);
                        u_var2 = *(uVar3 + 0x16);
                        iVar6 = write_to_file_1008_7b4c(param_2, u_var2 & 0xffff0000 | (u_var2 + iStack4 * 0x6), SEG_1008, param_4);
                        if(iVar6 == 0x0) {}
                            //goto LAB_1030_5734;
                    }
                    iVar6 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | (iVar7 + 0x14), SEG_1008, param_4);
                    if(iVar6 != 0x0)
                    {
                        local_8[0] = (iVar7 + 0x1c);
                        BVar5      = write_to_file_1008_7e1c(u_var4, uVar9, local_8, param_4, 0x2, SEG_1008);
                        if(BVar5 != 0x0)
                        {
                            return;
                        }
                    }
                }
            }
        }
    // LAB_1030_5734:
        globals.dat_1050_0310 = 0x6d0;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn  file_1030_581e(param_1: *mut Struct381, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16)

{
    let mut pi_var1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut u_var4: u16;
    let mut BVar5: BOOL16;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut puVar9: *mut u8;
    let mut iVar9: *mut Struct380;
    let mut uVar10: u16;
    let mut in_AF = 0u8;
    let mut uVar11: u16;
//    u16          uVar12;
    let mut uStack1040: u32;
    let mut iStack1036: i16;
    let mut local_408: [u8;400] = [0;400];
    let mut uStack8: u32;
    let mut local_4: i16;
//    Struct381 *iVar12;

//    iVar12 = param_1;
//    uVar12 = (param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if(param_3 != 0x0)
    {
        if(globals._PTR_LOOP_1050_5f2c == 0x0)
        {
            globals.dat_1050_5f2c      = mem_op_1000_160a(param_4, SEG_1000);
            globals.dat_1050_5f2e      = param_4;
        }
        else
        {
        }
        u_var4  = fn_ptr_op_1000_1708(0x20, 0x0, 0x1, globals.dat_1050_5f2c, globals.dat_1050_5f2e, SEG_1000);
        puVar9 = (globals.PTR_LOOP_1050_5f2e | u_var4);
        if(puVar9 == 0x0)
        {
            u_var4  = 0x0;
            puVar9 = 0x0;
        }
        else
        {
            pass1_1030_84ae(str_var1(globals.PTR_LOOP_1050_5f2e, u_var4));
        }
        param_1.field_0x10 = u_var4;
        param_1.field_0x12 = puVar9;
        u_var4              = param_2;
        uVar11             = (param_2 >> 0x10);
        BVar5              = read_file_1008_7dee(u_var4, uVar11, &local_4, 0x0, param_5, 0x2, SEG_1008);
        if(BVar5 != 0x0)
        {
            uVar8   = (globals._PTR_LOOP_1050_65e2 + 0x52);
            uStack8 = uVar8;
            pass1_1030_4782(param_5, in_AF, puVar9, uVar8, (uVar8 >> 0x10), 0x0, 0x1, local_4);
            param_1.field_0x10 = uVar8;
            param_1.field_0x12 = puVar9;
            BVar5              = read_file_1008_7dee(u_var4, uVar11, iVar12.field_0x10 + 0x2, 0x0, puVar9, 0x2, SEG_1008);
            if(BVar5 != 0x0)
            {
                puVar6 = local_408;
                read_file_1008_7c6e(u_var4, uVar11, str_var1(param_5, puVar6), SEG_1008);
                if(puVar6 != 0x0)
                {
                    uVar8 = &param_1.field_0x10;
                    fn_ptr_1000_17ce((uVar8 + 0x4), SEG_1000);
                    uVar7            = str_op_1008_60e8(str_var1(param_5, local_408), puVar9);
                    uVar8            = &param_1.field_0x10;
                    uVar10           = (uVar8 >> 0x10);
                    iVar9            = uVar8;
                    iVar9.field_0x4 = uVar7;
                    iVar9.field_0x6 = puVar9;
                    uVar8            = &param_1.field_0x10;
                    BVar5            = read_file_1008_7dee(u_var4, uVar11, uVar8 + 0x1a, 0x0, (uVar8 >> 0x10), 0x2, SEG_1008);
                    if(BVar5 != 0x0)
                    {
                        uVar8 = &param_1.field_0x10;
                        iVar2 = (uVar8 + 0x1a);
                        uVar7 = iVar2 * 0x6;
                        mem_op_1000_179c(uVar7, puVar9, 0);
                        uStack1040 = str_var1(puVar9, uVar7);
                        if((puVar9 | uVar7) == 0x0)
                        {
                            uVar8          = &param_1.field_0x10;
                            (uVar8 + 0x16) = 0x0;
                        }
                        else
                        {
                            pass1_1000_5586(0x3e38, SEG_1008, iVar2, 0x6, uVar7, puVar9);
                            uVar8          = &param_1.field_0x10;
                            (uVar8 + 0x16) = uStack1040;
                        }
                        // for(iStack1036 = 0x0; uVar8 = &param_1.field_0x10, pi_var1 = (uVar8 + 0x1a), *pi_var1 != iStack1036 && iStack1036 <= *pi_var1; iStack1036 = iStack1036 + 0x1)
                        // TODO
                        loop
                        {
                            uVar8 = &param_1.field_0x10;
                            uVar3 = *(uVar8 + 0x16);
                            BVar5 = read_file_1008_7bc8(param_2, (uVar3 & 0xffff0000 | (uVar3 + iStack1036 * 0x6)), SEG_1008, param_5);
                            if(BVar5 == 0x0){}
                                //goto LAB_1030_58a7;
                        }
                        BVar5 = read_file_1008_7bc8(param_2, (param_1 & 0xffff0000 | &param_1.field_0x14), SEG_1008, param_5);
                        if((BVar5 != 0x0) && (BVar5 = read_file_1008_7dee(u_var4, uVar11, &param_1.field_0x1c_addr_base, 0x0, uVar12, 0x2, SEG_1008), BVar5 != 0x0))
                        {
                            return;
                        }
                    }
                }
            }
        }
    // LAB_1030_58a7:
        globals.dat_1050_0310 = 0x6d2;
    }
    return;
}


pub fn  write_to_file_1030_32e4(param_1: u32, param_2: u32, param_3: u16)

{
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut BVar3: BOOL16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut local_16: [u32;0x2];
    let mut local_c: u16;
    let mut local_a: [u32;0x2];

    iVar2 = param_1;
    uVar1 = (param_1 >> 0x10);
    u_var4 = param_2;
    uVar5 = (param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c(u_var4, uVar5, iVar2 + 0x4, uVar1, 0x16c, SEG_1008);
    if(BVar3 != 0x0)
    {
        BVar3 = write_to_file_1008_7e1c(u_var4, uVar5, iVar2 + 0x174, uVar1, &DAT_0000_000c, SEG_1008);
        if(BVar3 != 0x0)
        {
            BVar3 = write_to_file_1008_7e1c(u_var4, uVar5, iVar2 + 0x180, uVar1, &DAT_0000_000c, SEG_1008);
            if(BVar3 != 0x0)
            {
                BVar3 = write_to_file_1008_7e1c(u_var4, uVar5, iVar2 + 0x18c, uVar1, 0x18, SEG_1008);
                if(BVar3 != 0x0)
                {
                    local_c = (iVar2 + 0x1a8);
                    BVar3   = write_to_file_1008_7e1c(u_var4, uVar5, &local_c, param_3, 0x2, SEG_1008);
                    if(BVar3 != 0x0)
                    {
                        local_16[0] = (iVar2 + 0x1aa);
                        BVar3       = write_to_file_1008_7e1c(u_var4, uVar5, local_16, param_3, 0x4, SEG_1008);
                        if(BVar3 != 0x0)
                        {
                            local_a[0] = (iVar2 + 0x170);
                            BVar3      = write_to_file_1008_7e1c(u_var4, uVar5, local_a, param_3, 0x4, SEG_1008);
                            if(BVar3 != 0x0)
                            {
                                local_c = (iVar2 + 0x1ae);
                                BVar3   = write_to_file_1008_7e1c(u_var4, uVar5, &local_c, param_3, 0x2, SEG_1008);
                                if(BVar3 != 0x0)
                                {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    globals.dat_1050_0310 = 0x6d0;
    return;
}


pub fn read_file_1030_33f0(globals: &mut Globals, param_1: *mut Struct430, param_2: u32)

{
    let mut uVar1: u16;
//    Struct430 *iVar2;
    let mut BVar2: BOOL16;
    let mut uVar3: u16;
    let mut u_var4: u16;

//    iVar2 = param_1;
    param_1 = &param_1.field_0x4;
//    uVar1 = (param_1 >> 0x10);
    uVar3 = param_2;
    u_var4 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(uVar3, u_var4, param_1, 0x0, uVar1, 0x16c, SEG_1008);
    if(((((BVar2 != 0x0) && (BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x174, 0x0, uVar1, 0xc, SEG_1008), BVar2 != 0x0)) && (BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x180, 0x0, uVar1, 0xc, SEG_1008), BVar2 != 0x0))
        && ((BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x18c, 0x0, uVar1, 0x18, SEG_1008), BVar2 != 0x0 && (BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x1a8, 0x0, uVar1, 0x2, SEG_1008), BVar2 != 0x0))))
       && (BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x1aa, 0x0, uVar1, 0x4, SEG_1008), BVar2 != 0x0))
    {
        if(globals.dat_1050_0312 < 0x2)
        {
            return;
        }
        BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x170, 0x0, uVar1, 0x4, SEG_1008);
        if((BVar2 != 0x0) && (BVar2 = read_file_1008_7dee(uVar3, u_var4, &param_1.field_0x1ae, 0x0, uVar1, 0x2, SEG_1008), BVar2 != 0x0))
        {
            return;
        }
    }
    globals.dat_1050_0310 = 0x6d2;
}
