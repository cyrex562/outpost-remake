use std::default::default;

use crate::bad::{bad_1030_1312, bad_1030_8cd2};
use crate::defines::{Struct18, U32Ptr, Struct99};
use crate::file::file_1008::{read_file_1008_7bc8, read_file_1008_7c6e, read_file_1008_7dee, write_to_file_1008_7a22, write_to_file_1008_7cac};
use crate::file::file_1028::{file_1028_b81a, write_to_file_1028_b5ec};
use crate::file::file_1030::file_1030_19b4;
use crate::file::file_1038::file_1038_774e;
use crate::fn_ptr::fn_ptr_1000::{call_fn_ptr_1000_0dc6, fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1020::fn_ptr_1020_ba7e;
use crate::fn_ptr::fn_ptr_1028;
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1028_d566;
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_07fc, pass1_1000_093a, pass1_1000_0ed4, pass1_1000_47a4, pass1_1000_4906, pass1_1000_49b2, pass1_1000_49c6, pass1_1000_4aea, pass1_1000_54a0, pass1_1000_5586, pass1_1000_3e2c};
use crate::pass::pass_1008::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_3f62, pass1_1008_4772, pass1_1008_5784, pass1_1008_5b12, pass1_1008_612e, pass1_1008_6c90, pass1_1008_6d18, pass1_1008_6d3e, pass1_1008_6d64, pass1_1008_79f0, pass1_1008_7c2a, pass1_1008_b820, pass1_1008_6cec, pass1_1008_4544, pass1_1008_6cb4};
use crate::pass::pass_1010::{pass1_1010_043a, pass1_1010_089e, pass1_1010_82f8, pass1_1010_9092, pass1_1010_9794, pass1_1010_ed22, pass1_1010_ed3e, pass1_1010_9f8c, pass1_1010_65d0};
use crate::pass::pass_1018::{pass1_1018_04a4, pass1_1018_04ca, pass1_1018_04b8};
use crate::pass::pass_1020::{pass1_1020_a43e, pass1_1020_ba94, pass1_1020_bae6, pass1_1020_bb16, pass1_1020_a49a, pass1_1020_ba3e, pass1_1020_bb8a};
use crate::pass::pass_1028::{pass1_1028_1c1c, pass1_1028_20b0, pass1_1028_45e2, pass1_1028_6228, pass1_1028_6302, pass1_1028_6408, pass1_1028_6744, pass1_1028_67d4, pass1_1028_b22c, pass1_1028_b260, pass1_1028_b39e, pass1_1028_b418, pass1_1028_b46e, pass1_1028_b4f2, pass1_1028_b58e, pass1_1028_bb6a, pass1_1028_bb96, pass1_1028_bd38, pass1_1028_bdac, pass1_1028_bf22, pass1_1028_c952, pass1_1028_cb04, pass1_1028_cfd2, pass1_1028_cff2, pass1_1028_d01a, pass1_1028_d078, pass1_1028_d282, pass1_1028_d52c, pass1_1028_d81c, pass1_1028_daba, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e2ac, pass1_1028_e2e0, pass1_1028_e332, pass1_1028_e372, pass1_1028_e44a, pass1_1028_e4ec, struct_1028_b354, pass1_1028_be9e, pass1_1028_be2a, pass1_1028_c7b6, pass1_1028_e198, pass1_1028_6356};
use crate::pass::pass_1038::{empty_1038_540a, pass1_1038_01c0, pass1_1038_354a, pass1_1038_35a8, pass1_1038_3fb0, pass1_1038_4760, pass1_1038_48e0, pass1_1038_4900, pass1_1038_4b20, pass1_1038_4d0e, pass1_1038_4d6e, pass1_1038_4e78, pass1_1038_4fd8, pass1_1038_53ba, pass1_1038_5694, pass1_1038_6984, pass1_1038_69fe, pass1_1038_75ca, pass1_1038_78e2, pass1_1038_7964, struct_1038_6520, pass1_1038_095e, pass1_1038_08d4, pass1_1038_008e, pass1_1038_44d8, pass1_1038_52b8};
use crate::string::string_1000::{str_op_1000_3da4, string_1000_3cea, string_1000_3d3e};
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::{load_string_1010_84ac, load_string_1010_847e};
use crate::string::string_1030::vsprintf_op_1030_840a;
use crate::struct_ops::struct_1008::{clear_struct_1008_3e38, pass1_1008_c6ae, pass1_1008_c6fa, set_struct_1008_574a};
use crate::struct_ops::struct_1028::{struct_1028_d22e, struct_1028_d2b0, struct_op_1028_87f0};
use crate::struct_ops::struct_1030::{struct_1030_1550, struct_1030_1628, struct_1030_17ce, struct_1030_2112, struct_1030_8544, struct_op_1030_1cd8, struct_op_1030_73a8};
use crate::switch_ops::switch_1008::switch_1008_72bc;
use crate::switch_ops::switch_1020::switch_1020_c3b4;
use crate::sys_api::{dos3_call_1000_51aa, sys_1000_3f9c};
use crate::ui::ui_1008::{pass1_1008_aaa8, post_win_msg_1008_a0e4};
use crate::ui::ui_1028::send_msg_1028_e242;
use crate::util::{CARRY2, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24, SBORROW2, SEXT24};
use crate::debug::debug_print_1008_6048;
use crate::file::file_1010::unk_io_op_1010_830a;

pub fn pass1_1030_10b0(param_1: u16, param_2: u16, param_3: u16, param_4: u32,
                       param_5: u32, param_6: U32Ptr, param_7: &mut Struct179, param_8: u16,
                       param_9: u16, param_10: u16)

{
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uStack8: u16;

    puVar6 = switch_1030_07ac(param_1, param_2, param_3, param_4,
                              (param_4 >> 0x10), param_5, param_6, param_7,
                              param_8, param_9, param_10);
    // u_var3 = (puVar6 >> 0x10);
    u_var1 = (puVar6 + 0x4);
    u_var2 = u_var1;
    u_var4 = u_var3;
    pass1_1028_e1ec(CONCAT22(param_2, param_1), param_5);
    u_var5 = u_var4 | u_var2;
    if (u_var5 != 0x0) {
        pass1_1030_7e5a(u_var2 & 0xffff | u_var4 << 0x10, u_var1, u_var5);
    }
    // uStack8 = (u_var1 >> 0x10);
    pass1_1030_1358((param_1 + 0x26), puVar6, u_var3,
                    u_var1 & 0xffff | (uStack8 & 0xff) << 0x10, param_10);
    return;
}


pub fn pass1_1030_1120(param_1: u32, param_2: u16, param_3: U32Ptr, param_4: u16) {
    let pu_var1: U32Ptr;

    mem_op_1000_179c(0x3b2, param_3, 0x1000);
    pu_var1 = (param_3 | param_2);
    if (pu_var1 == 0x0) {
        param_2 = 0x0;
        pu_var1 = 0x0;
    } else {
        struct_1030_2112(CONCAT22(param_3, param_2), 0x0, param_2, pu_var1);
    }
    pass1_1030_1358((param_1 + 0x2a), param_2, pu_var1,
                    (param_2 + 0x4) & 0xffff | ((param_2 + 0x6) & 0xff) << 0x10, param_4);
    return;
}



astruct_18 *  pass1_1030_117a(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


pub fn pass1_1030_1244(param_1: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let ppc_var4: u32;
    let paVar5: &mut Struct18;
    let iVar6: &mut Struct606;
    let iVar7: i16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let uStack6: u32;

    // uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = (s_462_bmp_1050_1620 + 0x4);
    iVar6.field_0x2 = 0x1030;
    if (iVar6.field_0x1a != 0x0) {
        uStack6 = 0x1;
        loop {
            pu_var1 = &iVar6.field_0xa;
            if (*pu_var1 < uStack6 || *pu_var1 == uStack6) { break; }
            i_var8 = uStack6 * 0x4;
            paVar5 = iVar6.field_0x6;
            // u_var10 = (paVar5 >> 0x10);
            iVar7 = paVar5;
            pu_var2 = (iVar7 + i_var8);
            u_var3 = (iVar7 + i_var8 + 0x2);
            if ((u_var3 | pu_var2) != 0x0) {
                ppc_var4 = *pu_var2;
                (**ppc_var4)();
            }
            uStack6 += 0x1;
        }
    }
    fn_ptr_1000_17ce(ctx, iVar6.field_0x6, 0x1000);
    *param_1 = 0x389a;
    iVar6.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1030_12ca(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u32;
    let i_var3: &mut Struct176;
    let u_var3: u16;
    let uStack6: u32;

    uStack6 = 0x1;
    loop {
        // u_var3 = (param_1 >> 0x10);
        i_var3 = param_1;
        pu_var1 = &i_var3.field_0xa;
        if (*pu_var1 < uStack6 || *pu_var1 == uStack6) {
            i_var3.field_0x4 = 0x0;
            return;
        }
        u_var2 = i_var3.field_0x6;
        if ((u_var2 + uStack6 * 0x4) == 0x0) { break; }
        uStack6 += 0x1;
    }
    return;
}


pub fn pass1_1030_1358(param_1: u32, param_2: u16, param_3: u16, param_4: u32, param_5: u16) {
    let pu_var1: u32;
    let pu_var2: U32Ptr;
    let lVar3: i32;
    let i_var4: &mut Struct291;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let bVar8: bool;

    if (param_4 == 0x0) {
        return;
    }
    // u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = &i_var4.field_0xa;
    if ((*pu_var1 < param_4 || *pu_var1 == param_4) || (i_var4.field_0x6 == 0x0)) {
        pu_var2 = (&i_var4.field_0x12 + 0x2);
        bVar8 = *pu_var2 < param_4._2_2_;
        if ((bVar8 || *pu_var2 == param_4._2_2_) && ((bVar8 || (pu_var1 = &i_var4.field_0x12,
                                                               pu_var1 < param_4 || pu_var1 == param_4)))) {
            struct_1030_1550(param_1 & 0xffff | u_var6 << 0x10, param_5);
        }
        pu_var1 = &i_var4.field_0x12;
        if (*pu_var1 < param_4 || *pu_var1 == param_4) {
            return;
        }
        if (i_var4.field_0x6 == 0x0) {
            return;
        }
        pu_var2 = &i_var4.field_0xc;
        bVar8 = *pu_var2 < param_4._2_2_;
        if ((bVar8 || *pu_var2 == param_4._2_2_) && ((bVar8 || (pu_var2 = &i_var4.field_0xa,
                                                               *pu_var2 < param_4 || *pu_var2 == param_4)))) {
            i_var4.field_0xa = (param_4 + 0x1);
            i_var4.field_0xc = (param_4 + 0x1 >> 0x10);
        }
    }
    lVar3 = i_var4.field_0x6;
    // uVar7 = (lVar3 >> 0x10);
    iVar5 = lVar3;
    (iVar5 + param_4 * 0x4) = param_2;
    (iVar5 + param_4 * 0x4 + 0x2) = param_3;
    return;
}


pub fn pass1_1030_13f6(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16) -> u16

{
    let ppcVar1: u32;
    let u_var2: u16;
    let puStack8: u32;
    let uStack4: u16;

    uStack4 = 0x0;
    bad_1030_1312();
    puStack8 = CONCAT22(param_4, param_3);
    if ((param_4 | param_3) != 0x0) {
        uStack4 = 0x1;
        // u_var2 = (param_1 >> 0x10);
        if (((param_1 + 0x1a) != 0x0) && ((param_4 | param_3) != 0x0)) {
            ppcVar1 = *puStack8;
            (**ppcVar1)();
        }
        pass1_1030_1358(param_1, 0x0, 0x0, param_2, param_5);
        (param_1 + 0x4) = 0x1;
    }
    return uStack4;
}


pub fn pass1_1030_145a(param_1: u32, param_2: i32) {
    let u_var1: u32;
    let u_var2: u16;
    let i_var4: &mut Struct346;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    fn_ptr_1000_17ce(ctx, i_var4.field_0x6, 0x1000);
    i_var4.field_0x6 = 0x0;
    i_var4.field_0xa = 0x0;
    u_var1 = i_var4.field_0x16 + param_2;
    // u_var2 = (u_var1 >> 0x10);
    if (u_var1 < i_var4.field_0xe) {
        u_var1 = &i_var4.field_0xe;
        u_var2 = (&i_var4.field_0xe + 0x2);
    }
    &i_var4.field_0xe = u_var1;
    (&i_var4.field_0xe + 0x2) = u_var2;
    i_var4.field_0x12 = 0x0;
    return;
}


pub fn pass1_1030_14b4(param_1: u32, param_2: u16, param_3: u16, param_4: u32, param_5: u16) {
    let pu_var1: u32;
    let pu_var2: U32Ptr;
    let lVar3: i32;
    let iVar5: &mut Struct345;
    let i_var4: &mut Struct344;
    let u_var4: u16;
    let u_var5: u16;
    let bVar6: bool;

    // u_var4 = (param_1 >> 0x10);
    iVar5 = param_1;
    pu_var1 = &iVar5.field_0xa;
    if ((*pu_var1 < param_4 || *pu_var1 == param_4) || (iVar5.field_0x6 == 0x0)) {
        pu_var2 = (&iVar5.field_0x12 + 0x2);
        bVar6 = *pu_var2 < param_4._2_2_;
        if ((bVar6 || *pu_var2 == param_4._2_2_) && ((bVar6 || (pu_var1 = &iVar5.field_0x12,
                                                               pu_var1 < param_4 || pu_var1 == param_4)))) {
            struct_1030_1550(param_1 & 0xffff | u_var4 << 0x10, param_5);
        }
        pu_var1 = &iVar5.field_0x12;
        if ((*pu_var1 < param_4 || *pu_var1 == param_4) || (iVar5.field_0x6 == 0x0)) {
            return;
        }
        pu_var2 = &iVar5.field_0xc;
        bVar6 = *pu_var2 < param_4._2_2_;
        if ((bVar6 || *pu_var2 == param_4._2_2_) && ((bVar6 || (pu_var2 = &iVar5.field_0xa,
                                                               *pu_var2 < param_4 || *pu_var2 == param_4)))) {
            iVar5.field_0xa = (param_4 + 0x1);
            iVar5.field_0xc = (param_4 + 0x1 >> 0x10);
        }
    }
    lVar3 = iVar5.field_0x6;
    // u_var5 = (lVar3 >> 0x10);
    i_var4 = lVar3;
    (i_var4 + param_4 * 0x4) = param_2;
    (i_var4 + param_4 * 0x4 + 0x2) = param_3;
    return;
}


pub fn pass1_1030_154c() {
    return;
}


astruct_18 *  pass1_1030_15fe(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_1244(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_165e(param_1: U32Ptr, param_2: u32, param_3: u32, param_4: u16) {
    let i_var1: &mut Struct175;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    &i_var1.field_0x4 = 0x0;
    i_var1.field_0x8 = param_3;
    *param_1 = 0x17ba;
    i_var1.field_0x2 = 0x1030;
    pass1_1030_5c8a(ctx.PTR_LOOP_1050_5736, param_2);
    i_var1.field_0x4 = param_3;
    i_var1.field_0x6 = param_4;
    return;
}


pub fn pass1_1030_16b2(param_1: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x17ba;
    (param_1 + 0x2) = 0x1030;
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    return;
}


pub fn pass1_1030_16d6(param_1: u32, param_2: u32, param_3: u16) {
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let local_10: [u32; 0x2];
    let local_8: u32;

    // u_var2 = (param_1 >> 0x10);
    local_10[0] = (param_1 + 0x4);
    // u_var3 = (param_2 >> 0x10);
    b_var1 = write_to_file_1008_7e1c(param_2, u_var3, local_10, param_3, 0x4, 0x1008);
    if (b_var1 != 0x0) {
        local_8 = (param_1 + 0x8);
        b_var1 = write_to_file_1008_7e1c(param_2, u_var3, &local_8, param_3, 0x4, 0x1008);
        if (b_var1 != 0x0) {
            return;
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}


pub fn pass1_1030_177a(param_1: u32, param_2: u32) {
    (param_1 + 0x8) = param_2;
    return;
}



astruct_18 *  pass1_1030_1794(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_16b2(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


u16 *
pass1_1030_183c(param_1: U32Ptr,param_2: u32,param_3: u32,param_4: u32,param_5: u32,
param_6: u16,param_7: U32Ptr)

{
let u_var1: u32; let u_var2: u16;
let i_var2: & mut Struct351;

i_var2 = param_1;
// u_var2 = (param_1 >> 0x10);
pass1_1030_165e(param_1, param_4, param_5, param_7); & i_var2.field_0xc = 0x0; *param_1 = 0x1a16; i_var2.field_0x2 = 0x1030;
if ((param_3 != 0x0) | | (param_2 != 0x0)) {
mem_op_1000_179c(0x18, param_7, 0x1000); if ((param_7 | param_6) == 0x0) {
u_var1 = 0x0;
}
else {
u_var1 = struct_op_1030_1cd8(CONCAT22(param_7, param_6), param_2, param_3);
}
i_var2.field_0xc = u_var1; i_var2.field_0xe = (u_var1 > > 0x10);
}
return param_1;
}



pub fn pass1_1030_18b2(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x1a16;
    (i_var4 + 0x2) = 0x1030;
    pu_var1 = (i_var4 + 0xc);
    u_var2 = (i_var4 + 0xe);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1030_16b2(param_1);
    return;
}


pub fn pass1_1030_18f0(param_1: u32, param_2: i16, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let extraout_dx: u16;
    let extraout_DX_00: i16;
    let i_var3: i16;
    let u_var4: u16;
    let uStack10: u32;
    let uStack6: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (i_var3 + 0xc) != 0x0 {
        ppcVar1 = ((i_var3 + 0xc) + 0x10);
        (**ppcVar1)();
        uStack6 = CONCAT22(extraout_dx, param_4);
        // TODO: refactor for loop
        // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
        //   ppcVar1 = ((i_var3 + 0xc) + 0x4);
        //   u_var2 = uStack6;
        //   (**ppcVar1)();
        //   if (u_var2 == param_2) && (extraout_DX_00 == param_3) {
        //     ppcVar1 = ((i_var3 + 0xc) + 0x8);
        //     (**ppcVar1)();
        //   }
        // }
    }
    return;
}


pub fn pass1_1030_1972() -> u16

{
    return 0x1;
}


pub fn pass1_1030_1978(param_1: u32, param_2: u32, param_3: u16, param_4: u16) -> u16

{
    pass1_1030_16d6(param_1, param_2, param_4);
    if (param_3 != 0x0) {
        write_to_file_1008_7954(param_2, (param_1 + 0xc), param_3, 0x1008,
                                param_4);
        if (param_3 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return param_3;
        }
        param_3 = 0x1;
    }
    return param_3;
}


astruct_18 *  pass1_1030_19f0(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_18b2(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_1a32(param_1: U32Ptr, param_2: u16, param_3: U32Ptr) -> u16

{
    pass1_1030_183c(param_1, 0x1, 0x16, 0xff000000, 0x0, param_2, param_3);
    ctx.PTR_LOOP_1050_5168 = (param_1 >> 0x10);
    ctx.PTR_LOOP_1050_5166 = param_1;
    (ctx.PTR_LOOP_1050_5166 + 0x10) = 0x0;
    *param_1 = 0x1cbc;
    (ctx.PTR_LOOP_1050_5166 + 0x2) = 0x1030;
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_1a74(param_1: U32Ptr) {
    *param_1 = 0x1cbc;
    (param_1 + 0x2) = 0x1030;
    ctx._PTR_LOOP_1050_5166 = 0x0;
    pass1_1030_18b2(param_1);
    return;
}


pub fn pass1_1030_1a9c(param_1: u32, param_2: u32, param_3: u16) -> u16

{
    let u_var1: u32;
    let piVar2: U32Ptr;
    let in_AX: u16;
    let u_var3: u16;
    let Bvar4: bool;
    let iVar5: i16;
    let u_var6: u16;
    let local_c: [u16; 0x5];

    u_var3 = pass1_1030_1978(param_1, param_2, in_AX, param_3);
    if (u_var3 != 0x0) {
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        local_c[0] = (iVar5 + 0x10);
        // u_var3 = (param_2 >> 0x10);
        BVar4 = write_to_file_1008_7e1c(param_2, u_var3, local_c, param_3, 0x2, 0x1008);
        if (BVar4 != 0x0) {
            if (*(iVar5 + 0x10) == 0x0) {
                return 0x1;
            }
            piVar2 = (iVar5 + 0x10);
            u_var1 = (piVar2 + 0x2);
            BVar4 = write_to_file_1008_7e1c(param_2, u_var3, u_var1,
                                            (u_var1 >> 0x10),
                                            (*piVar2 * 0x2), 0x1008);
            if (BVar4 != 0x0) {
                return 0x1;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn pass1_1030_1be2(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;
    let uStack4: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var3 = param_3;
    if ((i_var4 + 0xc) == 0x0) {
        mem_op_1000_179c(0x18, param_3, 0x1000);
        pu_var3 = (param_3 | param_2);
        if (pu_var3 == 0x0) {
            (i_var4 + 0xc) = 0x0;
        } else {
            struct_op_1030_1cd8(CONCAT22(param_3, param_2), 0x5, 0x5);
            (i_var4 + 0xc) = param_2;
            (i_var4 + 0xe) = extraout_dx;
            pu_var3 = extraout_dx;
        }
    }
    // TODO: refactor for loop
    // for (uStack4 = 0x0; pu_var2 = (i_var4 + 0x10),
    //     uStack4 <= *pu_var2 && *pu_var2 != uStack4; uStack4 += 0x1) {
    //   u_var6 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2,pu_var3,0x1);
    //   ppcVar1 = ((i_var4 + 0xc) + 0x8);
    //   (**ppcVar1)(&USHORT_1050_1028,(i_var4 + 0xc),u_var6,
    //               (u_var6 >> 0x10),uStack4,0x0);
    //   pu_var3 = extraout_DX_00;
    // }
    return;
}



astruct_18 *  pass1_1030_1c96(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_1a74(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


pub fn pass1_1030_1d28(param_1: U32Ptr) {
    let i_var1: &mut Struct594;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x2044;
    i_var1.field_0x2 = 0x1030;
    fn_ptr_1000_17ce(ctx, i_var1.field_0x4, 0x1000);
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_1d58(param_1: u32) {
    let ppcVar1: u32;
    let u_var2: u32;

    ppcVar1 = (param_1 + 0x4);
    u_var2 = (**ppcVar1)();
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    return;
}


pub fn pass1_1030_1d7c(param_1: u16, param_2: u16, param_3: u32) -> u32

{
    let u_var1: u32;

    pass1_1030_1d58(param_3);
    if ((param_2 | param_1) != 0x0) {
        u_var1 = struct_op_1030_73a8(CONCAT22(param_2, param_1));
        return u_var1;
    }
    return 0x0;
}


pub fn pass1_1030_1daa(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xa), (param_1 + 0x8));
}


pub fn pass1_1030_1dbc() {
    return;
}


pub fn pass1_1030_1dfc(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u32) {
    let pu_var1: u32;
    let pu_var2: U32Ptr;
    let ppc_var3: u32;
    let u_var4: u32;
    let iVar5: i16;
    let u_var6: u16;
    let bVar7: bool;

    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    pu_var1 = (iVar5 + 0x8);
    if ((*pu_var1 < param_4 || *pu_var1 == param_4) || ((iVar5 + 0x4) == 0x0)) {
        pu_var2 = (iVar5 + 0x12);
        bVar7 = *pu_var2 < param_4._2_2_;
        if ((bVar7 || *pu_var2 == param_4._2_2_) && ((bVar7 || (pu_var2 = (iVar5 + 0x10),
                                                               *pu_var2 < param_4 || *pu_var2 == param_4)))) {
            ppc_var3 = (*param_1 + 0x20);
            (**ppc_var3)();
        }
        pu_var1 = (iVar5 + 0x10);
        if ((*pu_var1 < param_4 || *pu_var1 == param_4) || ((iVar5 + 0x4) == 0x0)) {
            return;
        }
        pu_var2 = (iVar5 + 0xa);
        bVar7 = *pu_var2 < param_4._2_2_;
        if ((bVar7 || *pu_var2 == param_4._2_2_) && ((bVar7 || (pu_var2 = (iVar5 + 0x8),
                                                               *pu_var2 < param_4 || *pu_var2 == param_4)))) {
            (iVar5 + 0x8) = (param_4 + 0x1);
            (iVar5 + 0xa) = (param_4 + 0x1 >> 0x10);
        }
    }
    u_var4 = (iVar5 + 0x4);
    // u_var6 = (u_var4 >> 0x10);
    iVar5 = u_var4;
    (iVar5 + param_4 * 0x4) = param_2;
    (iVar5 + param_4 * 0x4 + 0x2) = param_3;
    return;
}


pub fn pass1_1030_1e96(param_1: U32Ptr) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let uStack6: u32;

    uStack6 = 0x0;
    loop {
        // u_var4 = (param_1 >> 0x10);
        pu_var1 = (param_1 + 0x8);
        if ((*pu_var1 < uStack6 || *pu_var1 == uStack6) || (u_var3 = (param_1 + 0x4),
                                                          (u_var3 + uStack6 * 0x4) == 0x0)) { break; }
        uStack6 += 0x1;
    }
    ppcVar2 = (*param_1 + 0x8);
    (**ppcVar2)();
    return;
}


pub fn pass1_1030_1eee(param_1: u32, param_2: u32) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0xc);
    param_2._2_2_ = (i_var2 + 0xe);
    if (u_var1 < param_2) {
        u_var1 = param_2 & 0xffff;
    }
    (i_var2 + 0xc) = u_var1;
    (i_var2 + 0xe) = param_2._2_2_;
    return;
}


pub fn pass1_1030_1f16(param_1: U32Ptr, param_2: u32) {
    plVar1: &i32;
    let ppcVar2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (((i_var4 + 0x4) == 0x0) || ((i_var4 + 0x10) <= (i_var4 + 0x8))) {
        ppcVar2 = (*param_1 + 0x20);
        (**ppcVar2)();
    }
    u_var3 = (i_var4 + 0x4);
    ((i_var4 + 0x8) * 0x4 + u_var3) = param_2;
    plVar1 = (i_var4 + 0x8);
    *plVar1 = *plVar1 + 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_1f77(param_1: i16, param_2: i16, param_3: u16, param_4: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let lVar7: i32;

    if ((param_1 + 0x10) == 0x0) {
        i_var4 = (param_1 + 0xc);
        ctx.PTR_LOOP_1050_5f2e = (param_1 + 0xe);
    } else {
        u_var2 = (param_1 + 0x10);
        pu_var1 = (param_1 + 0x14);
        i_var4 = u_var2 + *pu_var1;
        ctx.PTR_LOOP_1050_5f2e =

            ((param_1 + 0x12) + (param_1 + 0x16) + CARRY2(u_var2, *pu_var1));
    }
    (param_2 + -0x4) = i_var4;
    (param_2 + -0x2) = ctx.PTR_LOOP_1050_5f2e;
    (param_2 + -0x8) = 0x0;
    if ((param_1 + 0x4) == 0x0) {
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {}
        u_var5 = fn_ptr_op_1000_1708((param_2 + -0x4) << 0x2, 0x0, 0x1,
                                    ctx.PTR_LOOP_1050_5f2c, PTR_LOOP_1050_5f2e, 0x1000);
    } else {
        u_var3 = (param_1 + 0x4);
        u_var2 = (param_2 + -0x4);
        lVar7 = pass1_1000_0ed4(
            ctx, 0x1000, param_4, 0x1, u_var2 * 0x4,
            (ctx.PTR_LOOP_1050_5f2e * 0x2 + CARRY2(u_var2, u_var2)) * 0x2 + CARRY2(u_var2 * 0x2, u_var2 * 0x2), u_var3,
            (u_var3 >> 0x10));
        ctx.PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
        u_var5 = lVar7;
    }
    (param_2 + -0x8) = u_var5;
    (param_2 + -0x6) = ctx.PTR_LOOP_1050_5f2e;
    if ((ctx.PTR_LOOP_1050_5f2e | (param_2 + -0x8)) != 0x0) {
        u_var3 = (param_2 + 0x6);
        // u_var6 = (u_var3 >> 0x10);
        i_var4 = u_var3;
        (i_var4 + 0x10) = (param_2 + -0x4);
        (i_var4 + 0x4) = (param_2 + -0x8);
    }
    return;
}



astruct_18 *  pass1_1030_201e(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_1d28(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_2068(param_1: U32Ptr) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;
    let i_stack4: i16;

    struct_1030_17ce(param_1, 0x0, 0x0);
    // u_var3 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x293c;
    (i_var1 + 0x2) = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | (i_var1 + 0x10)),
                    0x0, 0x106);
    pass1_1000_4906((param_1 & 0xffff0000 | (i_var1 + 0x116)),
                    0x0, 0x86);
    pass1_1000_4906((param_1 & 0xffff0000 | (i_var1 + 0x19c)),
                    0x0, 0xa);
    pass1_1000_4906((param_1 & 0xffff0000 | (i_var1 + 0x2ac)),
                    0x0, 0x106);
    i_stack4 = 0x0;
    loop {
        i_var2 = i_stack4 * 0x2 + i_var1;
        (i_var2 + 0x10) = 0xffff;
        (i_var2 + 0x1a6) = 0x19;
        i_stack4 += 0x1;
        if (i_stack4 < 0x83) == false { break; }
    }
    return;
}


pub fn pass1_1030_2242(param_1: u32, param_2: i16) -> i16

{
    let i_var1: i16;
    let i_var2: &mut Struct168;
    let paVar2: &mut Struct168;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    paVar2 = &i_var2.field_0x10;
    if (-0x1 < (&paVar2.field_0x0 + param_2 * 0x2)) {
        i_var1 = (&i_var2.field_0x10 + param_2 * 0x2);
        paVar2 = i_var2 + 0x1;
        if ((&paVar2.field_0x0 + param_2 * 0x2) <= i_var1) {
            return i_var1;
        }
    }
    return (&paVar2.field_0x0 + param_2 * 0x2);
}


pub fn pass1_1030_227a(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;

    u_var1 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if (u_var1 != 0x0) {
        i_var2 = param_1;
        // u_var1 = (param_1 >> 0x10);
        u_var4 = param_2;
        // u_var5 = (param_2 >> 0x10);
        BVar3 = write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x10, u_var1, 0x106, 0x1008);
        if (BVar3 != 0x0) {
            BVar3 = write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x116, u_var1, 0x86, 0x1008);
            if (BVar3 != 0x0) {
                BVar3 = write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x19c, u_var1, 0xa, 0x1008);
                if (BVar3 != 0x0) {
                    BVar3 = write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x1a6, u_var1, 0x106, 0x1008);
                    if (BVar3 != 0x0) {
                        BVar3 = write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x2ac, u_var1, 0x106, 0x1008);
                        if (BVar3 != 0x0) {
                            return;
                        }
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


pub fn pass1_1030_232e(param_1: u32, param_2: u32, param_3: i16, param_4: u16,
                       param_5: u16)

{
    let u_var1: u16;
    let i_var2: i16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;

    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if (param_3 != 0x0) {
        i_var2 = param_1;
        // u_var1 = (param_1 >> 0x10);
        u_var4 = param_2;
        // u_var5 = (param_2 >> 0x10);
        BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x10, 0x0, u_var1, 0x106, 0x1008);
        if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x116, 0x0, u_var1, 0x86, 0x1008);
            if (BVar3 != 0x0) {
                BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x19c, 0x0, u_var1, 0xa, 0x1008);
                if (BVar3 != 0x0) {
                    BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x1a6, 0x0, u_var1, 0x106, 0x1008);
                    if (BVar3 != 0x0) {
                        BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x2ac, 0x0, u_var1, 0x106, 0x1008);
                        if (BVar3 != 0x0) {
                            return;
                        }
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_23e2(param_1: u32, param_2: i16, param_3: u16, param_4: i16, param_5: U32Ptr,
                       param_6: u16, param_7: u16)

{
    let ppcVar1: u32;
    let u_var2: u32;
    let bVar3: bool;
    let bVar4: bool;
    undefined3
    extraout_var;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;
    let uVar9: u16;
    let puVar10: u32;
    let puVar11: U32Ptr;
    let uVar12: u16;
    let iStack8: i16;

    // uVar9 = (param_1 >> 0x10);
    u_var6 = param_1;
    if ((u_var6 + 0x10 + param_3 * 0x2) < 0x0) {
        uVar12 = param_3;
        if (param_2 == 0x0) {
            puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x31, param_7, param_5, param_6);
            ppcVar1 = (*puVar10 + 0x14);
            (**ppcVar1)(0x1010, puVar10, (puVar10 >> 0x10), param_3,
                        param_3 >> 0xf);
            param_5 = extraout_DX_00;
        } else {
            puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x41, param_7, param_5, param_6);
            ppcVar1 = (*puVar10 + 0x14);
            (**ppcVar1)(0x1010, puVar10, (puVar10 >> 0x10), param_3,
                        param_3 >> 0xf);
            param_5 = extraout_dx;
        }
        u_var2 = (uVar12 + 0x16);
        param_4 = (u_var2 + 0x4);
        (u_var6 + param_3 * 0x2 + 0x10) = param_4;
    }
    if ((u_var6 + 0x10 + param_3 * 0x2) != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
        pass1_1030_2852();
        bVar3 = false;
        iStack8 = param_4;
        if ((u_var6 + 0x152) != 0x0) {
            bVar4 = pass1_1030_266c(u_var6, CONCAT22(param_3, uVar9));
            if (CONCAT31(extraout_var, bVar4) != 0x0) {
                iStack8 = param_4 + 0x1;
                bVar3 = true;
            }
        }
        i_var8 = param_3 * 0x2;
        iStack8 = (u_var6 + i_var8 + 0x10) - iStack8;
        (u_var6 + i_var8 + 0x10) = iStack8;
        if (iStack8 < 0x0) {
            (u_var6 + i_var8 + 0x10) = 0x0;
        }
        uVar7 = param_3 * 0x2;
        if ((u_var6 + 0x2ac + uVar7) == 0x0) {
            i_var8 = uVar7 + u_var6;
            (i_var8 + 0x2ac) = 0x1;
            param_5 = ((u_var6 + uVar7 + 0x1a6) + -0x1);
            (i_var8 + 0x1a6) = param_5;
            param_6 = uVar7;
            if ((u_var6 + uVar7 + 0x1a6) < 0x0) {
                (i_var8 + 0x1a6) = 0x0;
            }
        }
        if (((u_var6 + 0x10 + param_3 * 0x2) != 0x0) || (uVar7 = u_var6 + 0x1a6, (uVar7 + param_3 * 0x2) != 0x0)) {
            if ((u_var6 + 0x10 + param_3 * 0x2) == 0x0) {
                (u_var6 + param_3 * 0x2 + 0x10) = 0x1;
            }
            return;
        }
        uVar12 = param_3;
        puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, param_6);
        // u_var5 = (puVar11 >> 0x10);
        pass1_1010_6cf8(0x1010, puVar11, uVar12, param_7, u_var5, uVar7, param_6);
        pass1_1030_26ac(param_1, param_3, u_var5, param_7);
        if (bVar3) {
            i_var8 = pass1_1030_28dc(param_1, param_3);
            (u_var6 + i_var8 * 0x2 + 0x19c) = 0x0;
        }
    }
    return;
}


pub fn pass1_1030_25b2(param_1: u32, param_2: i16) -> bool

{
    if ((param_1 + 0x10 + param_2 * 0x2) == 0x0) {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1030_25d8(param_1: u32, param_2: u16, param_3: i16) {
    (param_1 + param_3 * 0x2 + 0x10) = param_2;
    return;
}


pub fn pass1_1030_25f0(param_1: u32, param_2: i16, param_3: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if (param_2 == 0x0) {
        param_2 = (param_1 + 0x116 + param_3 * 0x2) + 0x1;
    }
    (param_1 + param_3 * 0x2 + 0x116) = param_2;
    return;
}


pub fn pass1_1030_2622(param_1: u32, param_2: i16) -> bool

{
    let i_var1: i16;

    if ((param_2 != 0x70) && (param_2 != 0x1)) {
        i_var1 = pass1_1030_28dc(param_1, 0x0);
        if (-0x1 < i_var1) {
            (param_1 + i_var1 * 0x2 + 0x19c) = param_2;
        }
        return -0x1 < i_var1;
    }
    return false;
}


pub fn pass1_1030_266c(param_1: u16, param_2: u32) -> bool

{
    let i_var1: i16;

    i_var1 = pass1_1030_28dc(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    return i_var1 != -0x1;
}


pub fn pass1_1030_2690(param_1: u32) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x2ac)),
                    0x0, 0x106);
    return;
}


pub fn pass1_1030_26ac(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let cVar5: u8;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let i_var8: i16;
    let i_var9: i16;
    let u_var10: u16;
    let iVar11: i16;
    let uVar12: u16;
    let iStack38: i16;
    let local_14: [u8; 12];

    iVar11 = param_1;
    // uVar12 = (param_1 >> 0x10);
    if (param_2 != 0x13) {
        if (0x13 < param_2) {
            if (param_2 != 0x5f) {
                if ((param_2 - 0x5f) < 0x6) {
                    return;
                }
                if (param_2 != 0x66 && 0x0 < (param_2 - 0x65)) {
                    if ((param_2 - 0x66) < 0x5) {
                        return;
                    }
                    if (0x4 < (param_2 - 0x6b)) {
                        return;
                    }
                }
            }
            pass1_1028_dc52(CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
            loop {
                u_var10 = param_3;
                puVar6 = local_14;
                pass1_1028_e4ec(CONCAT22(param_4, puVar6));
                param_3 = u_var10 | puVar6;
                if (param_3 == 0x0) { break; }
                if ((iVar11 + 0x4) == (puVar6 + 0x200)) {
                    uVar7 = (puVar6 + 0x18) + 0x19;
                    if (0x3e8 < uVar7) {
                        uVar7 = 0x3e8;
                    }
                    pass1_1038_4d0e(CONCAT22(u_var10, puVar6), uVar7);
                }
            }
            return;
        }
        if (param_2 == 0x12) {
            pass1_1028_dc52(CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
            loop {
                u_var10 = param_3;
                puVar6 = local_14;
                pass1_1028_e4ec(CONCAT22(param_4, puVar6));
                param_3 = u_var10 | puVar6;
                if (param_3 == 0x0) { break; }
                if ((iVar11 + 0x4) == (puVar6 + 0x200)) {
                    u_var2 = (puVar6 + 0x1f6);
                    i_var9 = u_var2;
                    // u_var4 = (u_var2 >> 0x10);
                    pi_var1 = (i_var9 + 0x182);
                    *pi_var1 = *pi_var1 + -0x19;
                    i_var8 = (i_var9 + 0x182);
                    if (i_var8 < 0x1) {
                        i_var8 = 0x1;
                    }
                    (i_var9 + 0x182) = i_var8;
                }
            }
            return;
        }
        if (0x12 < param_2) {
            return;
        }
        cVar5 = param_2;
        if (cVar5 != '\n') {
            if ((cVar5 + -0xa) < '\x06') {
                return;
            }
            if ('\x01' < (cVar5 + -0x10)) {
                return;
            }
        }
    }
    pass1_1028_dc52(CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
    loop {
        u_var10 = param_3;
        puVar6 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar6));
        param_3 = u_var10 | puVar6;
        if (param_3 == 0x0) { break; }
        if ((iVar11 + 0x4) == (puVar6 + 0x200)) {
            u_var2 = (puVar6 + 0x1f6);
            i_var8 = u_var2 + 0x180;
            // u_var4 = (u_var2 >> 0x10);
            iStack38 = 0x1;
            loop {
                i_var3 = iStack38 * 0x2;
                pi_var1 = (i_var3 + i_var8);
                *pi_var1 = *pi_var1 + -0x1;
                i_var9 = (i_var3 + i_var8);
                if (i_var9 < 0x1) {
                    i_var9 = 0x1;
                }
                (i_var3 + i_var8) = i_var9;
                iStack38 += 0x1;
                if (iStack38 < 0x6) == false { break; }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_2852() {
    return;
}



i16  pass1_1030_28dc(param_1: u32,param_2: i16)

{
let i_stack4: i16;

i_stack4 = 0x0; loop {
if (0x4 < i_stack4) {
return - 0x1;
}
if ((param_1 + 0x19c + i_stack4 * 0x2) == param_2) { break; }
i_stack4 += 0x1;
}
return i_stack4;
}



astruct_18 *  pass1_1030_2916(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_18b2(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_2958(param_1: U32Ptr) {
    let i_var1: &mut Struct347;
    let u_var1: u16;

    struct_1030_17ce(param_1, 0x5, 0xf);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x10 = 0x0;
    i_var1.field_0x14 = 0x0;
    i_var1.field_0x16 = 0x0;
    i_var1.field_0x18 = 0x2710;
    i_var1.field_0x1a = 0x0;
    *param_1 = 0x3130;
    i_var1.field_0x2 = 0x1030;
    return;
}


pub fn pass1_1030_29e6(param_1: U32Ptr) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var4: &mut Struct607;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x3130;
    i_var4.field_0x2 = 0x1030;
    paVar2 = &i_var4.field_0x10;
    u_var1 = i_var4.field_0x12;
    if ((u_var1 | paVar2) != 0x0) {
        pass1_1030_8496(paVar2 & 0xffff | u_var1 << 0x10);
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    }
    pass1_1030_18b2(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_2a2c(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let pi_var1: U32Ptr;
    let i_var2: &mut Struct678;
    let u_var2: u16;
    let paVar3: &mut Struct67;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let i_var9: i16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (0x0 < i_var2.field_0x18) {
        pi_var1 = &i_var2.field_0x18;
        *pi_var1 = *pi_var1 + -0x1;
    }
    if (i_var2.field_0x16 == 0x0) {
        i_var2.field_0x16 = 0x1;
    }
    if (i_var2.field_0x1a == 0x0) {
        i_var2.field_0x1a = 0x2;
    }
    if (i_var2.field_0x18 < 0x1) {
        i_var2.field_0x16 = 0x2;
        i_var2.field_0x1a = 0x1;
        uVar8 = 0x0;
        i_var9 = 0x21;
        u_var6 = 0x1;
        uVar7 = 0x0;
        u_var4 = 0x0;
        iVar5 = 0x0;
        u_var2 = 0x0;
        paVar3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
        post_win_msg_1008_a0e4(paVar3, CONCAT22(u_var4, u_var2), iVar5, u_var6, CONCAT22(uVar8, uVar7), i_var9, 0x1008, param_4);
    }
    return;
}


pub fn pass1_1030_2a98(param_1: u32) -> u16

{
    let pi_var1: U32Ptr;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    pi_var1 = (param_1 + 0x14);
    *pi_var1 = *pi_var1 + 0x1;
    return (param_1 + 0x14);
}


pub fn pass1_1030_2aaa(param_1: u32) -> u16

{
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x10) == 0x0) {
        return 0x0;
    }
    u_var1 = (param_1 + 0x10);
    return (u_var1 + 0xc);
}


pub fn pass1_1030_2aca(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let Bvar4: bool;
    let iVar5: i16;
    let iVar6: &mut Struct730;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let local_18: [u32; 0x3];
    let local_c: [u16; 0x3];
    let local_6: [u16; 0x2];

    u_var3 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if (u_var3 == 0x0) {
        return;
    }
    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    local_c[0] = *iVar6.field_0x10;
    u_var3 = param_2;
    // uVar8 = (param_2 >> 0x10);
    BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_c, param_4, 0x2, 0x1008);
    if (((BVar4 != 0x0) && (pu_var2 = iVar6.field_0x10,
                            BVar4 = pass1_1008_7c2a(param_2, (pu_var2 + 0x2), 0x1008), BVar4 != 0x0)
    ) && (pu_var2 = iVar6.field_0x10,
          iVar5 = write_to_file_1008_7b4c(param_2, pu_var2 & 0xffff0000 | (pu_var2 + 0x6), 0x1008, param_4),
          iVar5 != 0x0)) {
        pu_var2 = iVar6.field_0x10;
        local_6[0] = (pu_var2 + 0xc);
        BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_6, param_4, 0x2, 0x1008);
        if (BVar4 != 0x0) {
            pu_var2 = iVar6.field_0x10;
            local_18[0] = (pu_var2 + 0xe);
            BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_18, param_4, 0x4, 0x1008);
            if ((BVar4 != 0x0) && (pu_var2 = iVar6.field_0x10,
                                   BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, pu_var2 + 0x12, (pu_var2 >> 0x10), 0x10, 0x1008), BVar4 != 0x0)) {
                pu_var2 = iVar6.field_0x10;
                local_c[0] = (pu_var2 + 0x22);
                BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_c, param_4, 0x2, 0x1008);
                if ((BVar4 != 0x0) && ((pu_var2 = iVar6.field_0x10, (pu_var2 + 0x22) == 0x0 || (pu_var2 = iVar6.field_0x10, uVar7 = (pu_var2 >> 0x10),
                                                                                              iVar5 = pu_var2, u_var1 = (iVar5 + 0x24),
                                                                                              BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, u_var1, (u_var1 >> 0x10),
                                                                                                                              ((iVar5 + 0x22) * 0x2), 0x1008), BVar4 != 0x0)))) {
                    local_c[0] = iVar6.field_0x14;
                    BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_c, param_4, 0x2, 0x1008);
                    if (BVar4 != 0x0) {
                        local_c[0] = iVar6.field_0x16;
                        BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_c, param_4, 0x2, 0x1008);
                        if (BVar4 != 0x0) {
                            local_c[0] = iVar6.field_0x18;
                            BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_c, param_4, 0x2, 0x1008);
                            if (BVar4 != 0x0) {
                                local_c[0] = iVar6.field_0x1a;
                                BVar4 = write_to_file_1008_7e1c(u_var3, uVar8, local_c, param_4, 0x2, 0x1008);
                                if (BVar4 != 0x0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_2c8a(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let iVar7: &mut Struct374;
    let i_var8: &mut Struct371;
    let i_var9: &mut Struct372;
    let unaff_DI: i16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let puStack1038: U32Ptr;
    let local_406: u16;
    let local_404: u16;
    let local_402: [u8; 400];
    let iVar14: &mut Struct373;

    iVar14 = param_1;
    // u_var10 = (param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if (param_3 == 0x0) {
        return;
    }
    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4, 0x1000);
        ctx.PTR_LOOP_1050_5f2e = param_4;
    } else {}
    u_var2 = fn_ptr_op_1000_1708(0x28, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e, 0x1000);
    puStack1038 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var2);
    puVar6 = (ctx.PTR_LOOP_1050_5f2e | u_var2);
    if (puVar6 == 0x0) {
        iVar14.field_0x10 = 0x0;
    } else {
        puVar8 = clear_struct_1008_3e38(CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var2 + 0x6));
        // puVar6 = (puVar8 >> 0x10);
        iVar14.field_0x10 = puStack1038;
    }
    puVar8 = iVar14.field_0x10;
    u_var2 = param_2;
    // uVar9 = (param_2 >> 0x10);
    BVar3 = read_file_1008_7dee(u_var2, uVar9, puVar8, 0x0,
                                (puVar8 >> 0x10), 0x2, 0x1008);
    if (BVar3 != 0x0) {
        puVar4 = local_402;
        read_file_1008_7c6e(u_var2, uVar9, CONCAT22(param_5, puVar4), 0x1008);
        if (puVar4 != 0x0) {
            u_var5 = str_op_1008_60e8(CONCAT22(param_5, local_402), puVar6);
            puVar8 = iVar14.field_0x10;
            // uVar7 = (puVar8 >> 0x10);
            iVar7 = puVar8;
            iVar7.field_0x2 = u_var5;
            iVar7.field_0x4 = puVar6;
            puVar8 = iVar14.field_0x10;
            BVar3 = read_file_1008_7bc8(param_2,
                                        (puVar8 & 0xffff0000 | (puVar8 + 0x6)), 0x1008, param_5);
            if ((((BVar3 != 0x0) && (puVar8 = iVar14.field_0x10,
                                     BVar3 = read_file_1008_7dee(u_var2, uVar9, puVar8 + 0xc, 0x0,
                                                                 (puVar8 >> 0x10), 0x2, 0x1008),
                                     BVar3 != 0x0)) && (puVar8 = iVar14.field_0x10,
                                                        BVar3 = read_file_1008_7dee(u_var2, uVar9, puVar8 + 0xe, 0x0,
                                                                                    (puVar8 >> 0x10), 0x4, 0x1008),
                                                        BVar3 != 0x0)) && ((puVar8 = iVar14.field_0x10,
                                                                            BVar3 = read_file_1008_7dee(u_var2, uVar9, puVar8 + 0x12, 0x0,
                                                                                                        (puVar8 >> 0x10), 0x10, 0x1008),
                                                                            BVar3 != 0x0 && (puVar8 = iVar14.field_0x10,
                                                                                             BVar3 = read_file_1008_7dee(u_var2, uVar9, puVar8 + 0x22, 0x0,
                                                                                                                         (puVar8 >> 0x10), 0x2, 0x1008),
                                                                                             BVar3 != 0x0)))) {
                puVar8 = iVar14.field_0x10;
                if ((puVar8 + 0x22) != 0x0) {
                    puVar8 = iVar14.field_0x10;
                    // unaff_DI = (puVar8 >> 0x10);
                    i_var8 = puVar8;
                    u_var5 = i_var8.field_0x22 * 0x2;
                    mem_op_1000_179c(u_var5, puVar6, 0x1000);
                    i_var8.field_0x24 = u_var5;
                    i_var8.field_0x26 = puVar6;
                    puVar8 = iVar14.field_0x10;
                    // uVar7 = (puVar8 >> 0x10);
                    i_var9 = puVar8;
                    u_var1 = i_var9.field_0x24;
                    BVar3 = read_file_1008_7dee(u_var2, uVar9, u_var1, 0x0,
                                                (u_var1 >> 0x10),
                                                i_var9.field_0x22 * 0x2, 0x1008);
                    if (BVar3 == 0x0) {
                        ctx.PTR_LOOP_1050_0310 = 0x6d2;
                        return;
                    }
                }
                BVar3 = read_file_1008_7dee(u_var2, uVar9, &iVar14.field_0x14, 0x0, u_var10, 0x2, 0x1008);
                if (((BVar3 != 0x0) && (BVar3 = read_file_1008_7dee(u_var2, uVar9, &local_404, 0x0, param_5, 0x2,
                                                                    0x1008), BVar3 != 0x0)) && ((BVar3 = read_file_1008_7dee(u_var2, uVar9, &iVar14.field_0x18, 0x0, u_var10, 0x2,
                                                                                                                             0x1008), BVar3 != 0x0 && (BVar3 = read_file_1008_7dee(u_var2, uVar9, &local_406, 0x0, param_5, 0x2,
                                                                                                                                                                                   0x1008), BVar3 != 0x0)))) {
                    iVar14.field_0x16 = local_404;
                    iVar14.field_0x1a = local_406;
                    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_5, puVar6, unaff_DI);
                    pass1_1018_04a4(puVar8, iVar14.field_0x4);
                    pass1_1010_82f8(ctx.PTR_LOOP_1050_14cc, *iVar14.field_0x10);
                    return;
                }
            }
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
}



i16  pass1_1030_2f1a(param_1: u32,param_2: U32Ptr,param_3: U32Ptr)

{
let i_var1: i16; let u_var2: u32;
let i_var3: i16;

u_var2 = (param_1 + 0x10); i_var3 = u_var2; i_var1 = (i_var3 + 0xc); if (i_var1 - 0x1 < 0x9) {
switch(i_var1) {
default: * param_3 = 0x19; * param_2 = 0x2d; return i_var3; 0x3 => 0x4 => 0x5 => * param_3 = 0xa; * param_2 = 0xf; return i_var3; 0x6 => * param_3 = 0xa; * param_2 = 0x19; return i_var3; 0x7 => * param_3 = 0x19; * param_2 = 0x37; return i_var3;
}
}
* param_3 = 0x0; * param_2 = 0x0;
return 0x0;
}



pub fn pass1_1030_2fac(param_1: u32) -> u16

{
    let lVar1: i32;
    let i_var2: &mut Struct598;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2.field_0x10 == 0x0) {
        return 0x0;
    }
    lVar1 = i_var2.field_0x10;
    if ((lVar1 + 0xc) < 0x2) {
        return 0x4;
    }
    lVar1 = i_var2.field_0x10;
    if ((lVar1 + 0xc) < 0x5) {
        return 0x3;
    }
    lVar1 = i_var2.field_0x10;
    if ((lVar1 + 0xc) < 0x8) {
        return 0x2;
    }
    return 0x1;
}


pub fn pass1_1030_3006(param_1: u32, param_2: u32) {
    (param_1 + 0x10) = param_2;
    return;
}


pub fn pass1_1030_301a(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u32;
    let u_var2: u16;
    let i_var4: i16;
    let i_var3: &mut Struct608;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x10) != 0x0) {
        u_var1 = (i_var4 + 0x10);
        fn_ptr_1000_17ce(ctx, (u_var1 + 0x2), 0x1000);
        u_var2 = str_op_1008_60e8(param_2, param_3);
        u_var1 = (i_var4 + 0x10);
        // u_var5 = (u_var1 >> 0x10);
        i_var3 = u_var1;
        i_var3.field_0x2 = u_var2;
        i_var3.field_0x4 = param_3;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_3058(param_1: u32, param_2: u16, param_3: U32Ptr) -> u16

{
    let pu_var1: U32Ptr;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let extraout_dx: U32Ptr;
    let i_var4: &mut Struct375;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u32;
    let uStack4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var3 = param_3;
    if (i_var4.field_0xc == 0x0) {
        mem_op_1000_179c(0x18, param_3, 0x1000);
        pu_var3 = (param_3 | param_2);
        if (pu_var3 == 0x0) {
            i_var4.field_0xc = 0x0;
        } else {
            u_var5 = struct_op_1030_1cd8(CONCAT22(param_3, param_2), 0x5, 0x5);
            // pu_var3 = (u_var5 >> 0x10);
            &i_var4.field_0xc = u_var5;
            (&i_var4.field_0xc + 0x2) = pu_var3;
        }
    }
    // TODO: refactor for loop
    // for (uStack4 = 0x0; u_var5 = i_var4.field_0x10, pu_var1 = (u_var5 + 0x22),
    //     uStack4 <= *pu_var1 && *pu_var1 != uStack4; uStack4 += 0x1) {
    //   u_var6 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2,pu_var3,0x3);
    //   ppcVar2 = (*i_var4.field_0xc + 0x8);
    //   (**ppcVar2)(&USHORT_1050_1028,i_var4.field_0xc,u_var6,(u_var6 >> 0x10),
    //               uStack4,0x0);
    //   pu_var3 = extraout_dx;
    // }
    return 0x1;
}



astruct_18 *  pass1_1030_310a(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_29e6(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_314c(param_1: U32Ptr, param_2: u32, param_3: U32Ptr, param_4: u16) {
    let i_var1: &mut Struct364;
    let unaff_DI: i16;
    let u_var1: u16;
    let iStack12: i16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x170 = 0x0;
    i_var1.field_0x1a4 = param_2;
    i_var1.field_0x1a8 = 0x5;
    i_var1.field_0x1aa = 0x0;
    i_var1.field_0x1ae = 0x10;
    *param_1 = 0x3af2;
    i_var1.field_0x2 = 0x1030;
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var1.field_0x4),
        0x0, 0x16c);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var1.field_0x18c),
        0x0, 0x18);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var1.field_0x174),
        0x0, 0xc);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var1.field_0x180),
        0x0, 0xc);
    mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_3, unaff_DI);
    if (ctx.PTR_LOOP_1050_13ae < 0x2) {
        pass1_1030_34da(param_1);
    } else {
        i_var1.field_0x176 = 0x1;
        i_var1.field_0x178 = 0x2;
        i_var1.field_0x17a = 0x2;
        i_var1.field_0x17c = 0x60001;
        // TODO: refactor for loop
        // for (iStack12 = 0x1; iStack12 < 0x6; iStack12 += 0x1) {
        //   (&i_var1.field_0x180 + iStack12 * 0x2) = 0x64;
        // }
    }
    return;
}


pub fn pass1_1030_3258(param_1: u32, param_2: u16) {
    (param_1 + 0x1ae) = param_2;
    return;
}


pub fn pass1_1030_326a(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let i_var4: &mut Struct692;
    let u_var4: u16;
    let lStack6: i32;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (i_var4.field_0x1aa == 0x0) {
        i_var4.field_0x1aa = 0x1;
    } else {
        param_2 = i_var4.field_0x1aa * 0x2;
        i_var4.field_0x1aa = param_2;
    }
    u_var1 = param_2;
    pass1_1030_38b8();
    lStack6 = CONCAT22(param_3, u_var1);
    u_var2 = i_var4.field_0x1aa;
    u_var3 = (&i_var4.field_0x1aa + 0x2);
    if (lStack6 < u_var2) {
        u_var2 = u_var1;
        u_var3 = param_3;
    }
    &i_var4.field_0x1aa = u_var2;
    (&i_var4.field_0x1aa + 0x2) = u_var3;
    pass1_1030_375a(param_1 & 0xffff | u_var4 << 0x10, 0x0,
                    u_var2 & 0xffff | u_var3 << 0x10, param_4);
    return;
}


pub fn pass1_1030_34da(param_1: u32) {
    let i_var1: &mut Struct682;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x176 = 0x1;
    i_var1.field_0x178 = 0x1;
    i_var1.field_0x17a = 0x1;
    i_var1.field_0x17c = 0x1;
    i_var1.field_0x17e = 0x4;
    i_var1.field_0x182 = 0x32;
    i_var1.field_0x184 = 0xa;
    i_var1.field_0x186 = 0xa;
    i_var1.field_0x188 = 0xa;
    i_var1.field_0x18a = 0x4b;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(i_var1 + 0x1)),
                    0x0, 0x18);
    return;
}


pub fn pass1_1030_3534(param_1: u32, param_2: u32) {
    (param_1 + 0x4) = param_2;
    return;
}


pub fn pass1_1030_3548(param_1: u32, param_2: i32) {
    plVar1: &i32;

    plVar1 = (param_1 + 0x4);
    *plVar1 = *plVar1 + param_2;
    return;
}


pub fn pass1_1030_355c(param_1: u32, param_2: u32) {
    let i_var1: i16;
    let u_var2: u16;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        i_var1 = i_stack4 * 0x4;
        // u_var2 = (param_1 >> 0x10);
        (param_1 + i_var1 + 0x4) = (i_var1 + param_2) + (param_1 + 0x4 + i_var1);
        i_stack4 += 0x1;
        if (i_stack4 < 0x5b) == false { break; }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_35a4(param_1: u32, param_2: i32, param_3: U32Ptr, param_4: u16, param_5: u16) {
    let pu_var1: U32Ptr;
    uchar * *ppu_var2;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let uVar9: u8;
    let u_var10: u8;
    let puStack24: U32Ptr;
    let puStack22: U32Ptr;
    let local_c: [u8; 2];
    let local_a: u32;
    let uStack6: u32;

    vsprintf_op_1030_840a(s_Pop_Leaving__ld_1050_516a, param_4, param_5, param_3);
    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_3, 0x1000);
        ctx.PTR_LOOP_1050_5f2e = param_3;
    } else {}
    puStack24 = ctx.PTR_LOOP_1050_5f2c;
    puStack22 = ctx.PTR_LOOP_1050_5f2e;
    u_var4 = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e, 0x1000);
    uStack6 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var4);
    uVar9 = param_5;
    u_var10 = (param_5 >> 0x8);
    pass1_1030_3948(param_1, CONCAT22(param_5, local_c),
                    CONCAT13(u_var10, CONCAT12(uVar9, &local_a)), 0x3);
    u_var6 = (&local_a + 0x2);
    pass1_1030_3948(param_1, CONCAT22(param_5, &local_a + 0x2),
                    CONCAT13(u_var10, CONCAT12(uVar9, local_c)), 0x4);
    loop {
        puVar7 = u_var6;
        if (param_2 < 0x1) { break; }
        pass1_1008_612e(local_a, (local_a >> 0x10), puVar7);
        u_var6 = ZEXT24(&param_2);
        puStack24 = puVar7;
        pass1_1030_3a3a(param_1, CONCAT13(u_var10, CONCAT12(uVar9, &param_2)), puVar7);
        // uVar8 = (uStack6 >> 0x10);
        pu_var1 = (puStack24 * 0x4 + uStack6);
        u_var5 = *pu_var1;
        *pu_var1 = *pu_var1 + u_var6;
        ppu_var2 = (uchar * *)(puStack24 * 0x4 + uStack6 + 0x2);
        *ppu_var2 = ctx.PTR_LOOP_1050_5f2e + (*ppu_var2 + CARRY2(u_var5, u_var6));
        pass1_1030_38f2(param_1, 0x3, param_5);
        u_var5 = u_var6;
        puVar7 = ctx.PTR_LOOP_1050_5f2e;
        pass1_1030_38f2(param_1, 0x4, param_5);
        i_var3 = ctx.PTR_LOOP_1050_5f2e + puVar7;
        ctx.PTR_LOOP_1050_5f2e = puVar7;
        if ((i_var3 + CARRY2(u_var5, u_var6) | u_var5 + u_var6) != 0x0);
        pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18c)),
                        0x0, 0x18) == false
        { break; }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_3694(param_1: u32, param_2: i16, param_3: i32, param_4: U32Ptr, param_5: u16,
                       param_6: u16)

{
    let pu_var1: U32Ptr;
    uchar * *ppu_var2;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;
    let puStack18: U32Ptr;
    let uStack6: u16;
    let puStack4: U32Ptr;

    vsprintf_op_1030_840a(s_Pop_Leaving__ld_1050_517a, param_5, param_6, param_4);
    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4, 0x1000);
        ctx.PTR_LOOP_1050_5f2e = param_4;
    } else {}
    puStack18 = ctx.PTR_LOOP_1050_5f2c;
    uStack6 = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                  ctx.PTR_LOOP_1050_5f2e, 0x1000);
    u_var5 = (param_2 - 0x1);
    puStack4 = ctx.PTR_LOOP_1050_5f2e;
    if (((param_2 < 0x1) || (SBORROW2(param_2, 0x1))) || (u_var5 = (param_2 - 0x5),
                                                          param_2 - 0x5 != 0x0 && 0x3 < (param_2 - 0x1))) {
        while (puVar4 = u_var5, 0x0 < param_3) {
            pass1_1008_612e(0x0, 0x5a, puVar4);
            u_var5 = ZEXT24(&param_3);
            puStack18 = puVar4;
            pass1_1030_3a3a(param_1, CONCAT13((param_6 >> 0x8),
                                              CONCAT12(param_6, &param_3)),
                            puVar4);
            pu_var1 = (puStack18 * 0x4 + uStack6);
            u_var3 = *pu_var1;
            *pu_var1 = *pu_var1 + u_var5;
            ppu_var2 = (uchar * *)(puStack18 * 0x4 + uStack6 + 0x2);
            *ppu_var2 = ctx.PTR_LOOP_1050_5f2e + (*ppu_var2 + CARRY2(u_var3, u_var5));
        }
    } else {
        pass1_1030_39dc(param_1, CONCAT22(param_6, &param_3),
                        CONCAT13((ctx.PTR_LOOP_1050_5f2e >> 0x8),
                                 CONCAT12(ctx.PTR_LOOP_1050_5f2e, uStack6)), param_2);
    }
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18c)),
                    0x0, 0x18);
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_375a(param_1: u32, param_2: i16, param_3: i32, param_4: u16) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;
    let i_var4: i16;
    let iVar5: i16;
    let u_var6: u16;
    let iStack20: i16;
    let uStack18: u32;
    let lStack14: i32;
    let uStack10: i16;
    let iStack8: i16;
    let local_6: i16;
    let local_4: i16;

    i_var4 = param_1;
    if (param_2 == 0x0) {
        local_4 = 0x5a;
        while ((-0x1 < local_4 && (pass1_1030_3a3a(param_1, CONCAT22(param_4, &param_3), local_4),
                                   param_3 != 0x0))) {
            local_4 += -0x1;
        }
    } else {
        pass1_1030_3948(param_1, CONCAT22(param_4, &local_4),
                        CONCAT22(param_4, &local_6), param_2);
        iStack10 = (local_4 - local_6) + 0x1;
        iStack8 = iStack10 >> 0xf;
        lStack14 = param_3 / iStack10;
        u_var3 = (lStack14 * iStack10);
        uStack18 = CONCAT22(((param_3 >> 0x10) - ((lStack14 * iStack10) >> 0x10)) - (param_3 < u_var3), param_3 - u_var3);
        // TODO: refactor for loop
        // for (iStack20 = local_6; iStack20 <= local_4; iStack20 += 0x1) {
        //   iVar5 = iStack20 * 0x4;
        //   u_var6 = (param_1 >> 0x10);
        //   (i_var4 + iVar5 + 0x4) = (i_var4 + iVar5 + 0x4) - lStack14;
        //   i_var1 = (i_var4 + iVar5 + 0x6);
        //   if ((uStack18._2_2_ | uStack18) != 0x0) {
        //     i_var2 = (i_var4 + iVar5 + 0x4);
        //     (i_var4 + iVar5 + 0x4) = i_var2 + -0x1;
        //     (i_var4 + iVar5 + 0x6) = i_var1 - (i_var2 == 0x0);
        //     uStack18 += -0x1;
        //   }
        //   if ((i_var4 + iStack20 * 0x4 + 0x6) < 0x0) {
        //     (i_var4 + iStack20 * 0x4 + 0x4) = 0x0;
        //   }
        // }
    }
    pass1_1000_4906((param_1 & 0xffff0000 | (i_var4 + 0x18c)),
                    0x0, 0x18);
    return;
}


pub fn pass1_1030_387c(param_1: u32) {
    let i_stack4: i16;

    i_stack4 = 0x5a;
    loop {
        (i_stack4 * 0x4 + param_1 + 0x4) = (i_stack4 * 0x4 + param_1);
        i_stack4 += -0x1;
        if (0x0 < i_stack4) == false { break; }
    }
    (param_1 + 0x4) = 0x0;
    return;
}


pub fn pass1_1030_38b8() {
    let iStack8: i16;

    iStack8 = 0x0;
    {
        iStack8 += 0x1;
        if (iStack8 < 0x5b) == false { break; }
    }
    return;
}


pub fn pass1_1030_38f2(param_1: u32, param_2: i16, param_3: u16) {
    let iStack12: i16;
    let local_a: i16;
    let local_8: i16;
    let uStack6: u32;

    uStack6 = 0x0;
    pass1_1030_3948(param_1, CONCAT22(param_3, &local_a),
                    CONCAT22(param_3, &local_8), param_2);
    // TODO: refactor for loop
    // for (iStack12 = local_8; iStack12 <= local_a; iStack12 += 0x1) {
    // }
    return;
}


pub fn pass1_1030_3948(param_1: u32, param_2: U32Ptr, param_3: &mut i16, param_4: i16) {
    let u_var1: u16;

    if (param_4 == 0x1) {
        *param_3 = 0x0;
        *param_2 = 0x3;
        return;
    }
    // u_var1 = (param_1 >> 0x10);
    if (param_4 == 0x2) {
        *param_3 = 0x4;
        *param_2 = (param_1 + 0x1ae);
        return;
    }
    if (param_4 == 0x3) {
        *param_3 = (param_1 + 0x1ae) + 0x1;
        *param_2 = 0x27;
        return;
    }
    if (param_4 != 0x4) {
        if (param_4 == 0x5) {
            *param_3 = 0x4c;
        } else {
            *param_3 = 0x0;
        }
        *param_2 = 0x5a;
        return;
    }
    *param_3 = 0x28;
    *param_2 = 0x4b;
    return;
}


pub fn pass1_1030_39dc(param_1: u32, param_2: &i32, param_3: u32, param_4: i16) {
    let i_var1: i16;
    let in_DX: u16;
    let u_var2: u16;
    let unaff_SS: u16;
    let iStack8: i16;
    let local_6: i16;
    let local_4: i16;

    pass1_1030_3948(param_1, CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_4), param_4);
    iStack8 = local_6;
    loop {
        if (iStack8 < local_4) {
            return;
        }
        i_var1 = local_4;
        pass1_1030_3a3a(param_1, param_2, iStack8);
        // u_var2 = (param_3 >> 0x10);
        (iStack8 * 0x4 + param_3) = i_var1;
        (iStack8 * 0x4 + param_3 + 0x2) = in_DX;
        if (*param_2 == 0x0) { break; }
        iStack8 += -0x1;
    }
    return;
}


pub fn pass1_1030_3a3a(param_1: u32, param_2: &i32, param_3: i16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let iVar7: i16;
    let i_var8: i16;
    let uVar9: u16;

    i_var2 = (param_2 + 0x2);
    // uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    iVar7 = iVar6 + 0x4;
    i_var8 = param_3 * 0x4;
    pi_var1 = (iVar7 + i_var8 + 0x2);
    i_var3 = *pi_var1;
    if ((i_var3 < i_var2) || ((u_var5 = *param_2, *pi_var1 == i_var2 || i_var3 < i_var2 && ((iVar7 + i_var8) < u_var5)))) {
        *param_2 = *param_2 - (iVar6 + 0x4 + param_3 * 0x4);
        (iVar6 + param_3 * 0x4 + 0x4) = 0x0;
    } else {
        u_var4 = (iVar7 + i_var8);
        i_var3 = (iVar7 + i_var8 + 0x2);
        (iVar6 + i_var8 + 0x4) = u_var4 - u_var5;
        (iVar6 + i_var8 + 0x6) = (i_var3 - i_var2) - (u_var4 < u_var5);
        *param_2 = 0x0;
    }
    return;
}



astruct_18 *  pass1_1030_3ac6(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



u32 *
pass1_1030_3af6(param_1: U32Ptr,param_2: u16,param_3: u16,param_4: U32Ptr,param_5: u16
)

{
let i_var1: i16; let u_var2: u16;

// u_var2 = (param_1 >> 0x10);
i_var1 = param_1; * param_1 = * param_4;
(i_var1 + 0x4) = (param_4 + 0x1); (i_var1 + 0x6) = param_3;
(i_var1 + 0x8) = param_2; return param_1;
}



pub fn pass1_1030_3b28(param_1: u16) -> u16

{
    let pu_var1: U32Ptr;
    let pu_var2: u32;
    let local_8: [u8; 6];

    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffc4, 0x0);
    pass1_1030_3af6(&USHORT_1050_65e6, 0x115, 0x15b, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6(&USHORT_1050_65f0, 0x116, 0x15c, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffdd, 0x32);
    pass1_1030_3af6(&USHORT_1050_65fa, 0x117, 0x15d, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x4b);
    pass1_1030_3af6(&USHORT_1050_6604, 0x118, 0x15e, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xf, 0x64);
    pass1_1030_3af6(&USHORT_1050_660e, 0x119, 0x15f, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x28, 0x7d);
    pass1_1030_3af6(&USHORT_1050_6618, 0x11a, 0x160, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffec, 0x96);
    pass1_1030_3af6(&USHORT_1050_6622, 0x11b, 0x161, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x14, 0xaf);
    pass1_1030_3af6(&USHORT_1050_662c, 0x11c, 0x162, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0xc8);
    pass1_1030_3af6(&USHORT_1050_6636, 0x11d, 0x163, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xfffb, 0xe1);
    pass1_1030_3af6(&USHORT_1050_6640, 0x11e, 0x164, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x32, 0xfa);
    pass1_1030_3af6(&USHORT_1050_664a, 0x11f, 0x165, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0xe1);
    pass1_1030_3af6(&USHORT_1050_6654, 0x120, 0x166, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffe7, 0xfa);
    pass1_1030_3af6(&USHORT_1050_665e, 0x121, 0x167, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x113);
    pass1_1030_3af6(&USHORT_1050_6668, 0x122, 0x168, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x28, 0x12c);
    pass1_1030_3af6(&USHORT_1050_6672, 0x123, 0x169, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xf, 0x145);
    pass1_1030_3af6(&USHORT_1050_667c, 0x124, 0x16a, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffec, 0x15e);
    pass1_1030_3af6(&USHORT_1050_6686, 0x125, 0x16b, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x0);
    pass1_1030_3af6(&USHORT_1050_6690, 0x126, 0x16c, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x2d, 0x19);
    pass1_1030_3af6(&USHORT_1050_669a, 0x127, 0x16d, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xa, 0x32);
    pass1_1030_3af6(&USHORT_1050_66a4, 0x128, 0x16e, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffe2, 0x4b);
    pass1_1030_3af6(&USHORT_1050_66ae, 0x129, 0x16f, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x5, 0x64);
    pass1_1030_3af6(&USHORT_1050_66b8, 0x12a, 0x170, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x32, 0x7d);
    pass1_1030_3af6(&USHORT_1050_66c2, 0x12b, 0x171, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffc9, 0x96);
    pass1_1030_3af6(&USHORT_1050_66cc, 0x12c, 0x172, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xfffb, 0xaf);
    pass1_1030_3af6(&USHORT_1050_66d6, 0x12d, 0x173, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffe7, 0xc8);
    pass1_1030_3af6(&USHORT_1050_66e0, 0x12e, 0x174, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x32, 0x32);
    pass1_1030_3af6(&USHORT_1050_66ea, 0x12f, 0x175, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x3c, 0x64);
    pass1_1030_3af6(&USHORT_1050_66f4, 0x130, 0x176, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffc4, 0xe1);
    pass1_1030_3af6(&USHORT_1050_66fe, 0x131, 0x177, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6(&USHORT_1050_6708, 0x132, 0x178, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x5, 0xaf);
    pass1_1030_3af6(&USHORT_1050_6712, 0x133, 0x179, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6(&USHORT_1050_671c, 0x134, 0x17a, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x23, 0x19);
    pass1_1030_3af6(&USHORT_1050_6726, 0x135, 0x17b, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xfffb, 0x32);
    pass1_1030_3af6(&USHORT_1050_6730, 0x136, 0x17c, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xf, 0x32);
    pass1_1030_3af6(&USHORT_1050_673a, 0x137, 0x17d, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x2d, 0x4b);
    pass1_1030_3af6(&USHORT_1050_6744, 0x138, 0x17e, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0x4b);
    pass1_1030_3af6(&USHORT_1050_674e, 0x139, 0x17f, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x2d, 0x64);
    pass1_1030_3af6(&USHORT_1050_6758, 0x13a, 0x180, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffe7, 0x7d);
    pass1_1030_3af6(&USHORT_1050_6762, 0x13b, 0x181, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x5, 0xaf);
    pass1_1030_3af6(&USHORT_1050_676c, 0x13c, 0x182, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0xc8);
    pass1_1030_3af6(&USHORT_1050_6776, 0x13d, 0x183, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffce, 0xc8);
    pass1_1030_3af6(&USHORT_1050_6780, 0x13e, 0x184, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xf, 0xfa);
    pass1_1030_3af6(&USHORT_1050_678a, 0x13f, 0x185, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0x113);
    pass1_1030_3af6(&USHORT_1050_6794, 0x140, 0x186, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffe2, 0x12c);
    pass1_1030_3af6(&USHORT_1050_679e, 0x141, 0x187, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x64, 0x12c);
    pass1_1030_3af6(&USHORT_1050_67a8, 0x142, 0x188, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x32, 0x145);
    pass1_1030_3af6(&USHORT_1050_67b2, 0x143, 0x189, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x64, 0x145);
    pass1_1030_3af6(&USHORT_1050_67bc, 0x144, 0x18a, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0x15e);
    pass1_1030_3af6(&USHORT_1050_67c6, 0x145, 0x18b, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffd3, 0x15e);
    pass1_1030_3af6(&USHORT_1050_67d0, 0x146, 0x18c, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x32, 0xfa);
    pass1_1030_3af6(&USHORT_1050_67da, 0x147, 0x18d, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xf, 0x19);
    pass1_1030_3af6(&USHORT_1050_67e4, 0x148, 0x18e, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x32);
    pass1_1030_3af6(&USHORT_1050_67ee, 0x149, 0x18f, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0xaf);
    pass1_1030_3af6(&USHORT_1050_67f8, 0x14a, 0x190, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xfffb, 0xe1);
    pass1_1030_3af6(&USHORT_1050_6802, 0x14b, 0x191, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xa, 0x15e);
    pass1_1030_3af6(&USHORT_1050_680c, 0x14c, 0x192, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6(&USHORT_1050_6816, 0x14d, 0x193, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0x32);
    pass1_1030_3af6(&USHORT_1050_6820, 0x14e, 0x194, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xfffb, 0x64);
    pass1_1030_3af6(&USHORT_1050_682a, 0x14f, 0x195, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xf, 0x64);
    pass1_1030_3af6(&USHORT_1050_6834, 0x150, 0x196, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x1e, 0x7d);
    pass1_1030_3af6(&USHORT_1050_683e, 0x151, 0x197, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffdd, 0xe1);
    pass1_1030_3af6(&USHORT_1050_6848, 0x152, 0x198, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x113);
    pass1_1030_3af6(&USHORT_1050_6852, 0x153, 0x199, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x2d, 0x12c);
    pass1_1030_3af6(&USHORT_1050_685c, 0x154, 0x19a, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffe7, 0x145);
    pass1_1030_3af6(&USHORT_1050_6866, 0x155, 0x19b, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xa, 0x15e);
    pass1_1030_3af6(&USHORT_1050_6870, 0x156, 0x19c, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x4b);
    pass1_1030_3af6(&USHORT_1050_687a, 0x157, 0x19d, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x5, 0x64);
    pass1_1030_3af6(&USHORT_1050_6884, 0x158, 0x19e, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0xffec, 0x96);
    pass1_1030_3af6(&USHORT_1050_688e, 0x159, 0x19f, pu_var1,
                    (pu_var1 >> 0x10));
    pu_var1 = pass1_1008_3e54(CONCAT22(param_1, local_8), 0x0, 0x0, 0x113);
    pu_var2 = pass1_1030_3af6(&USHORT_1050_6898, 0x15a, 0x1a0, pu_var1,
                             (pu_var1 >> 0x10));
    return pu_var2;
}


pub fn pass1_1030_4538(param_1: U32Ptr) {
    let u_var1: u16;

    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    // u_var1 = (param_1 >> 0x10);
    fn_ptr_1000_17ce(ctx, (param_1 + 0x12), 0x1000);
    fn_ptr_1000_17ce(ctx, (param_1 + 0x15c), 0x1000);
    return;
}


pub fn pass1_1030_4594(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: i16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let i_var4: i16;
    let iVar5: i16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let puStack8: U32Ptr;

    pu_var2 = (param_4 - 0x1);
    mem_op_1000_179c(0x10, param_1, 0x1000);
    puStack8 = (pu_var2 & 0xffff | ZEXT24(param_1) << 0x10);
    u_var3 = param_1 | pu_var2;
    if (u_var3 == 0x0) {
        puStack8 = 0x0;
    } else {
        puVar7 = clear_struct_1008_3e38(CONCAT22(param_1, pu_var2 + 0x4));
        // u_var3 = (puVar7 >> 0x10);
        pu_var2 = puStack8;
    }
    u_var1 = SUB42(pu_var2, 0x0);
    i_var4 = (param_4 - 0x1) * 0x12;
    load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1010);
    // u_var6 = (puStack8 >> 0x10);
    iVar5 = puStack8;
    *puStack8 = u_var1;
    (iVar5 + 0x2) = u_var3;
    (iVar5 + 0xa) = (i_var4 + 0x51ba);
    pass1_1008_3e76((puStack8 & 0xffff0000 | (iVar5 + 0x4)),
                    (i_var4 + 0x51c0), (i_var4 + 0x51be),
                    (i_var4 + 0x51bc));
    (iVar5 + 0xc) = i_var4 + 0x51c2;
    (iVar5 + 0xe) = ctx.data_seg;
    return;
}


pub fn pass1_1030_4628(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: i16) {
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let iVar5: i16;
    let iVar6: i16;
    let piVar7: U32Ptr;
    let uVar8: u16;
    let iStack24: i16;
    let piStack20: U32Ptr;
    let uStack10: i16;
    let piStack8: U32Ptr;

    u_var2 = param_4 - 0x1;
    u_var3 = u_var2;
    mem_op_1000_179c(0x28, param_1, 0x1000);
    piStack20 = CONCAT22(param_1, u_var3);
    if ((param_1 | u_var3) == 0x0) {
        piStack8 = 0x0;
    } else {
        clear_struct_1008_3e38(CONCAT22(param_1, u_var3 + 0x6));
        piStack8 = piStack20;
    }
    // uVar8 = (piStack8 >> 0x10);
    iVar5 = piStack8;
    (iVar5 + 0x2) = 0x0;
    iVar6 = u_var2 * 0x5e;
    pass1_1008_3e76((piStack8 & 0xffff0000 | (iVar5 + 0x6)),
                    (iVar6 + 0x5336), (iVar6 + 0x5334),
                    (iVar6 + 0x5332));
    (iVar5 + 0xc) = (iVar6 + 0x5348);
    *piStack8 = param_4;
    (iVar5 + 0xe) = (iVar6 + 0x534a);
    iStack10 = 0x0;
    loop {
        u_var3 = ((u_var2 * 0x2f + iStack10) * 0x2 + 0x5338);
        (iVar5 + iStack10 * 0x2 + 0x12) = u_var3;
        iStack10 += 0x1;
        if (iStack10 < 0x8) == false { break; }
    }
    u_var1 = (&DAT_1050_5350 + u_var2 * 0x5e);
    pass1_1008_612e(u_var1, (u_var1 >> 0x10), u_var3);
    (iVar5 + 0x22) = u_var3;
    piVar7 = (u_var2 * 0x5e + 0x5354);
    (iVar5 + 0x24) = piVar7;
    (iVar5 + 0x26) = ctx.data_seg;
    iVar6 = *piVar7;
    pass1_1000_4906(CONCAT22(0x1050, piVar7), 0x0, 0x1e);
    iStack10 = 0x0;
//LAB_1030_474c:
    if (u_var3 <= iStack10) {
        return;
    }
    loop {
        i_var4 = (u_var2 * 0x5e + 0x534e) + iVar6 + -0x1;
        pass1_1008_612e(iVar6, i_var4, i_var4);
        iStack24 = 0x0;
        loop {
            if (iStack10 < iStack24) {
                u_var1 = (iVar5 + 0x24);
                (u_var1 + iStack10 * 0x2) = i_var4;
                iStack10 += 0x1;
//         TODO: goto LAB_1030_474c;
            }
            u_var1 = (iVar5 + 0x24);
            if ((u_var1 + iStack24 * 0x2) == i_var4) { break; }
            iStack24 += 0x1;
        }
    }
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_4782(param_1: U32Ptr, param_2: u8, param_3: U32Ptr, param_4: u16, param_5: u16,
                       param_6: i16, param_7: i16, param_8: i16)

{
    let i_var1: i16;
    let u_var2: u16;
    uchar * *ppu_var3;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let unaff_DI: i16;
    let uVar8: u16;
    let uVar9: u16;
    let puVar10: U32Ptr;
    let paVar11: &mut Struct43;
    let uVar12: u32;
    let iStack220: i16;
    let local_c4: U32Ptr;
    let puStack194: U32Ptr;
    let local_c0: U32Ptr;
    let uStack190: u16;
    let iStack188: i16;
    let paStack184: &mut Struct18;
    let iStack180: i16;
    let paStack178: &mut Struct18;
    let paStack174: &mut Struct18;
    let uStack170: u16;
    let uStack168: u16;
    let uStack166: u16;
    let uStack164: u16;
    let uStack162: u16;
    uchar * *ppuStack160;
    let iStack158: i16;
    let iStack156: i16;
    let iStack154: i16;
    let uStack152: u16;
    let mut pcStack150: String;
    ulocal_92: u8[0x80];
    let uStack18: u32;
    let uStack14: u32;
    let uStack10: u16;
    let uStack8: u16;
    let pi_stack6: U32Ptr;

    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_3, 0x1000);
        ctx.PTR_LOOP_1050_5f2e = param_3;
    } else {}
    local_c4 = ctx.PTR_LOOP_1050_5f2c;
    puStack194 = ctx.PTR_LOOP_1050_5f2e;
    u_var2 = fn_ptr_op_1000_1708(0x20, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e, 0x1000);
    paStack184 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var2);
    puVar4 = (ctx.PTR_LOOP_1050_5f2e | u_var2);
    if (puVar4 == 0x0) {
        u_var2 = 0x0;
        puVar4 = 0x0;
    } else {
        pass1_1030_84ae(CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var2));
    }
    pi_stack6 = CONCAT22(puVar4, u_var2);
    *pi_stack6 = param_8;
    pass1_1008_3f62(
        CONCAT13((puVar4 >> 0x8), CONCAT12(puVar4, u_var2 + 0x8)), CONCAT22(0x1050, &USHORT_1050_65e6 + param_8 * 0xa));
    if (param_7 != 0x0) {
        puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_1, puVar4, unaff_DI);
        // uStack8 = (puVar10 >> 0x10);
        uStack10 = SUB42(puVar10, 0x0);
        uStack14 = pass1_1018_04b8(puVar10);
        // u_var5 = (uStack14 >> 0x10);
        u_var2 = uStack14;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        uStack18 = CONCAT22(u_var5, u_var2);
        pcStack150 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc, (ctx.PTR_LOOP_1050_14cc >> 0x10),
                                           0x1010);
        // u_var6 = (pcStack150 >> 0x10);
        u_var2 = pass1_1030_2a98(uStack18);
        // uVar8 = (pi_stack6 >> 0x10);
        (pi_stack6 + 0x2) = u_var2;
        sys_1000_3f9c(local_92, param_1, pcStack150, (pcStack150 >> 0x10),
                      u_var2, &stack0xfffe, uVar8, 0x1000, param_1, param_2);
        u_var2 = str_op_1008_60e8(CONCAT22(param_1, local_92), u_var6);
        // uVar8 = (pi_stack6 >> 0x10);
        (pi_stack6 + 0x4) = u_var2;
        (pi_stack6 + 0x6) = u_var6;
        paVar11 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, (param_8 * 0xa + 0x65ec),
                                      param_1);
        // uVar8 = (pi_stack6 >> 0x10);
        (pi_stack6 + 0xe) = paVar11;
        (pi_stack6 + 0x10) = (paVar11 >> 0x10);
        paVar11 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, (param_8 * 0xa + 0x65ee),
                                      param_1);
        // uVar8 = (pi_stack6 >> 0x10);
        iVar7 = pi_stack6;
        (iVar7 + 0x12) = paVar11;
        (iVar7 + 0x14) = (paVar11 >> 0x10);
        uVar12 = pass1_1008_4772((iVar7 + 0xe));
        // uStack152 = (uVar12 >> 0x10);
        iStack154 = uVar12;
        iStack156 = (iStack154 + 0x4) + -0x1;
        iStack158 = (iStack154 + 0x8) + -0x1;
        if (param_6 != 0x0) {
            ppuStack160 = (uchar * *)(&ctx.PTR_LOOP_1050_000e + 0x1);
            if (uStack14 == 0x0) {
                debug_print_1008_6048(s_get_site_data_without_planet__1050_56de, 0x1008, param_1);
            } else {
                ppu_var3 = &local_c4;
                pass1_1030_2f1a(uStack18,
                                CONCAT13((param_1 >> 0x8),
                                         CONCAT12(param_1, &local_c0)),
                                CONCAT22(param_1, ppu_var3));
                pass1_1008_612e(local_c4, local_c0, ppu_var3);
                ppuStack160 = ppu_var3;
            }
            iVar7 = ppuStack160 * 0xa;
            // uVar8 = (pi_stack6 >> 0x10);
            (pi_stack6 + 0x1c) = iVar7;
            (pi_stack6 + 0x1c) = iVar7 / 0x64;
            puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_1,
                                      (iVar7 % 0x64), unaff_DI);
            // puStack194 = (puVar10 >> 0x10);
            local_c4 = puVar10;
            local_c0 = ctx.PTR_LOOP_1050_13ae;
            u_var2 = 0x84;
            puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x9, param_1, puStack194,
                                      unaff_DI);
            uStack190 = pass1_1010_65d0(param_1, puVar10, u_var2);
            iStack188 = 0x3c;
            if (local_c0 < 0x3) {
                if (0x0 < uStack190) {
                    iStack188 = 0x5a;
                }
            } else {
                if (uStack190 == 0x1) {
                    iStack188 = 0x44;
                } else {
                    if (uStack190 == 0x2) {
                        iStack188 = 0x4b;
                    } else {
                        if (uStack190 == 0x3) {
                            iStack188 = 0x53;
                        } else {
                            if (uStack190 == 0x4) {
                                iStack188 = 0x5a;
                            }
                        }
                    }
                }
            }
            iVar7 = iStack188 * ppuStack160;
            ppuStack160 = (uchar * *)(iVar7 / 0x64);
            puVar4 = (iVar7 % 0x64);
            // uVar8 = (pi_stack6 >> 0x10);
            (pi_stack6 + 0x1a) = ppuStack160;
            uStack164 = ppuStack160 + (pi_stack6 + 0x1c);
            u_var2 = uStack164 * 0x6;
            uStack162 = uStack164;
            mem_op_1000_179c(u_var2, puVar4, 0x1000);
            paStack184 = CONCAT22(puVar4, u_var2);
            ctx.PTR_LOOP_1050_5f2e = (puVar4 | u_var2);
            if (ctx.PTR_LOOP_1050_5f2e == 0x0) {
                (pi_stack6 + 0x16) = 0x0;
            } else {
                pass1_1000_5586(0x3e38, 0x1008, uStack164, 0x6, u_var2, puVar4);
                (pi_stack6 + 0x16) = paStack184;
            }
            uStack170 = uStack162 * 0x2;
            if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
                ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
            } else {}
            u_var2 = fn_ptr_op_1000_1708(uStack170, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                        ctx.PTR_LOOP_1050_5f2e, 0x1000);
            paStack174 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var2);
            if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
                ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
            } else {}
            u_var2 = fn_ptr_op_1000_1708(uStack170, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                        ctx.PTR_LOOP_1050_5f2e, 0x1000);
            paStack178 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var2);
            iStack180 = 0x0;
//LAB_1030_4b57:
            u_var5 = uStack162;
            if (iStack180 < uStack162) {
                loop {
                    pass1_1008_612e(0x0, iStack156, u_var5);
                    uStack166 = u_var5;
                    pass1_1008_612e(0x0, iStack158, u_var5);
                    iStack220 = 0x0;
                    loop {
                        iVar7 = paStack174;
                        // uVar8 = (paStack174 >> 0x10);
                        // uVar9 = (paStack178 >> 0x10);
                        uStack168 = u_var5;
                        if (iStack180 <= iStack220) {
                            i_var1 = iStack180 * 0x2;
                            (i_var1 + iVar7) = uStack166;
                            (i_var1 + paStack178) = u_var5;
                            uVar12 = (pi_stack6 + 0x16);
                            pass1_1008_3e76(
                                (uVar12 & 0xffff0000 | (uVar12 + iStack180 * 0x6)), 0x0, u_var5,
                                (i_var1 + iVar7));
                            iStack180 += 0x1;
//               TODO: goto LAB_1030_4b57;
                        }
                        if (((iStack220 * 0x2 + iVar7) == uStack166) && ((iStack220 * 0x2 + paStack178) == u_var5)) { break; }
                        iStack220 += 0x1;
                    }
                }
            }
            fn_ptr_1000_17ce(ctx, paStack174, 0x1000);
            paStack184 = paStack178;
            fn_ptr_1000_17ce(ctx, paStack178, 0x1000);
        }
    }
    return;
}


pub fn pass1_1030_4bbe(param_1: u16, param_2: u16, param_3: u32, param_4: i16) {
    let pu_var1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let i_var4: i16;
    Struct117 * iVar5;
    let pu_var5: u32;
    let puVar6: u32;
    let uVar7: u16;

    // uVar7 = (param_3 >> 0x10);
    iVar5 = (Struct117 *)
    param_3;
    if (iVar5.field_0x12 == 0x0) {
        pass1_1030_4f5a(param_1, param_2, param_3 & 0xffff | uVar7 << 0x10);
    }
    puVar6 = &iVar5.field_0x16;
    u_var3 = (&iVar5.field_0x12 + 0x2);
    pu_var5 = (&iVar5.field_0x12 + param_4 * 0x98);
    // TODO: refactor for loop
    // for (i_var4 = 0x26; i_var4 != 0x0; i_var4 += -0x1) {
    //   pu_var2 = puVar6;
    //   puVar6 = puVar6 + 0x1;
    //   pu_var1 = pu_var5;
    //   pu_var5 = pu_var5 + 0x1;
    //   *pu_var2 = *pu_var1;
    // }
    return;
}


pub fn pass1_1030_4c06(param_1: u32, param_2: i16, param_3: u16, param_4: u16) {
    let pu_var1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let puVar4: u32;
    let iVar5: i16;
    let puVar6: u32;
    let uVar7: u16;

    // uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x15c) == 0x0) {
        pass1_1030_5044(param_1 & 0xffff | uVar7 << 0x10, param_4, param_3);
    }
    puVar4 = (iVar5 + 0xae);
    u_var3 = (iVar5 + 0x15e);
    puVar6 = ((iVar5 + 0x15c) + param_2 * 0xae);
    // for (iVar5 = 0x2b; iVar5 != 0x0; iVar5 += -0x1) {
    //   pu_var2 = puVar4;
    //   puVar4 = puVar4 + 0x1;
    //   pu_var1 = puVar6;
    //   puVar6 = puVar6 + 0x1;
    //   *pu_var2 = *pu_var1;
    // }
    puVar4 = puVar6;
    return;
}


pub fn pass1_1030_4c52(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16,
                       param_6: u16)

{
    let u_var1: u16;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let mut pcStack8: String;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        u_var1 = pass1_1000_47a4(param_4, 0x1050518a, param_6);
        pcStack8 = CONCAT22(param_5, u_var1);
        if ((param_5 | u_var1) == 0x0) { break; }
        if (*pcStack8 != '\"') {
            i_var2 = pass1_1000_3e2c(CONCAT22(param_5, u_var1));
            i_var3 = param_3;
            // u_var4 = (param_3 >> 0x10);
            if (i_stack4 < 0x25) {
                (i_stack4 * 0x4 + i_var3) = i_var2;
                (i_stack4 * 0x4 + i_var3 + 0x2) = param_5;
            } else {
                if (i_stack4 == 0x25) {
                    (i_var3 + 0x94) = i_var2;
                } else {
                    if (i_stack4 == 0x26) {
                        (i_var3 + 0x96) = i_var2;
                    } else {
                        if (i_stack4 == 0x27) {
                            (i_var3 + 0x98) = i_var2;
                        } else {
                            if (i_stack4 == 0x28) {
                                (i_var3 + 0x9a) = i_var2;
                            } else {
                                if (i_stack4 == 0x29) {
                                    (i_var3 + 0x9c) = i_var2;
                                } else {
                                    if (i_stack4 == 0x2a) {
                                        (i_var3 + 0x9e) = i_var2;
                                    } else {
                                        if (i_stack4 == 0x2b) {
                                            (i_var3 + 0xa0) = i_var2;
                                        } else {
                                            if (i_stack4 == 0x2c) {
                                                (i_var3 + 0xa2) = i_var2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i_stack4 += 0x1;
        }
        param_4 = 0x0;
    }
    return;
}


pub fn pass1_1030_4d3a(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u32) {
    let u_var1: u16;
    let i_var2: i16;
    Struct118 * i_var3;
    let u_var3: u16;
    let unaff_SS: u16;
    let mut pcStack8: String;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        u_var1 = pass1_1000_47a4(param_5, 0x1050518a, unaff_SS);
        pcStack8 = CONCAT22(param_1, u_var1);
        if ((param_1 | u_var1) == 0x0) { break; }
        if (*pcStack8 != '\"') {
            i_var2 = pass1_1000_3e2c(CONCAT22(param_1, u_var1));
            i_var3 = (Struct118 *)
            param_4;
            // u_var3 = (param_4 >> 0x10);
            if (i_stack4 < 0x25) {
                (&i_var3.field_0x0 + i_stack4 * 0x4) = i_var2;
                (&i_var3.field_0x2 + i_stack4 * 0x4) = param_1;
            } else {
                if (i_stack4 == 0x25) {
                    i_var3.field_0x94 = i_var2;
                } else {
                    if (i_stack4 == 0x26) {
                        i_var3.field_0x96 = i_var2;
                    }
                }
            }
            i_stack4 += 0x1;
        }
        param_5 = 0x0;
    }
    return;
}


pub fn pass1_1030_4dbc(param_1: u32, param_2: u32, param_3: i32) {
    plVar1: &i32;
    let piVar2: U32Ptr;
    let lVar3: i32;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u16;

    iVar5 = param_1;
    // u_var6 = (param_1 >> 0x10);
    if (0x0 < param_3) {
        (iVar5 + 0x160) = param_2;
        (iVar5 + 0x164) = param_3;
    }
    if (((iVar5 + 0x160) == 0x0) || (lVar3 = (iVar5 + 0x164), plVar1 = (iVar5 + 0x164),
                                     *plVar1 = *plVar1 + -0x1, lVar3 == 0x0)) {
        (iVar5 + 0x160) = 0x0;
    } else {
        u_var4 = str_op_1000_3da4((iVar5 + 0x160));
        piVar2 = (iVar5 + 0x160);
        *piVar2 = *piVar2 + u_var4 + 0x2;
    }
    return;
}


pub fn pass1_1030_4e34(param_1: u16, param_2: u16, param_3: i32, param_4: &mut String) {
    while (param_3 != 0x0) {
        if ((*param_4 == '\r') || (*param_4 == '\n')) {
            *param_4 = '\0';
        }
        param_4 = (param_4 & 0xffff0000 | (param_4 + 0x1));
        param_3 = param_3 + -0x1;
    }
    return;
}


pub fn pass1_1030_4f5a(param_1: u16, param_2: u16, param_3: u32) {
    let u_var1: u16;
    let mut pcVar2: String;
    plVar3: &i32;
    let u_var4: u16;
    let iVar5: i16;
    let mut pcVar6: String;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uStack22: u16;
    let uStack20: u32;
    let uStack14: u16;
    let uStack12: u16;
    let local_a: i32;
    let mut local_6: String;

    plVar3 = &local_a;
    ctx.PTR_LOOP_1050_5f2e =

        read_file_1030_4e70(param_3, CONCAT22(param_1, plVar3),
                            (byte * *)CONCAT22(param_1, &local_6),
                            s_bldgbld_dat_1050_56fc, param_2);
    pcVar2 = local_6;
    if (plVar3 != 0x0) {
        uVar7 = param_3;
        // uVar8 = (param_3 >> 0x10);
        pcVar6 = local_6;
        pass1_1030_4e34(uVar7, uVar8, local_a, local_6);
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {}
        u_var4 = fn_ptr_op_1000_1708(pcVar6 * 0x98, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                    ctx.PTR_LOOP_1050_5f2e, 0x1000);
        (uVar7 + 0x12) = u_var4;
        (uVar7 + 0x14) = ctx.PTR_LOOP_1050_5f2e;
        pass1_1030_4dbc(param_3, local_6, pcVar6 & 0xffff);
        uStack20 = CONCAT22(extraout_dx, u_var4);
        // for (uStack22 = 0x0; uStack22 < pcVar6; uStack22 += 0x1) {
        //   u_var1 = (uVar7 + 0x14);
        //   iVar5 = (uVar7 + 0x12) + uStack22 * 0x98;
        //   pass1_1030_4d3a(u_var1,uVar7,uVar8,CONCAT22(u_var1,iVar5),uStack20);
        //   pass1_1030_4dbc(param_3,0x0,0x0);
        //   uStack20 = CONCAT22(extraout_DX_00,iVar5);
        // }
        // uStack12 = (pcVar2 >> 0x10);
        uStack14 = pcVar2;
        if ((uStack12 | uStack14) != 0x0) {
            call_fn_ptr_1000_0dc6(uStack14, uStack12, 0x1000);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_5044(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let mut pcVar2: String;
    plVar3: &i32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let mut pcVar8: String;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uStack28: u32;
    let uStack24: u16;
    let uStack22: u32;
    let uStack14: u16;
    let uStack12: u16;
    let local_a: i32;
    let mut local_6: String;
    let uVar9: u32;

    plVar3 = &local_a;
    ctx.PTR_LOOP_1050_5f2e =

        read_file_1030_4e70(param_1, CONCAT22(param_2, plVar3),
                            (byte * *)CONCAT22(param_2, &local_6),
                            s_bldgops_dat_1050_5708, param_3);
    pcVar2 = local_6;
    if (plVar3 != 0x0) {
        u_var10 = param_1;
        // u_var11 = (param_1 >> 0x10);
        pcVar8 = local_6;
        pass1_1030_4e34(u_var10, u_var11, local_a, local_6);
        u_var4 = pcVar8;
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {}
        u_var5 = fn_ptr_op_1000_1708(u_var4 * 0xae, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                    ctx.PTR_LOOP_1050_5f2e, 0x1000);
        uVar9 = u_var5;
        uStack28 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var5);
        if ((ctx.PTR_LOOP_1050_5f2e | u_var5) == 0x0) {
            (u_var10 + 0x15c) = 0x0;
        } else {
            pass1_1000_5586(0x51f0, 0x1030, u_var4, 0xae, u_var5, PTR_LOOP_1050_5f2e);
            (u_var10 + 0x15c) = uStack28;
            uVar9 = uStack28;
        }
        u_var6 = uVar9;
        pass1_1030_4dbc(param_1, local_6, pcVar8 & 0xffff);
        uStack22 = CONCAT22(extraout_dx, u_var6);
        // for (uStack24 = 0x0; uStack24 < u_var4; uStack24 += 0x1) {
        //   u_var1 = (u_var10 + 0x15e);
        //   iVar7 = (u_var10 + 0x15c) + uStack24 * 0xae;
        //   pass1_1030_4c52(u_var10,u_var11,CONCAT22(u_var1,iVar7),uStack22,u_var1,param_2);
        //   pass1_1030_4dbc(param_1,0x0,0x0);
        //   uStack22 = CONCAT22(extraout_DX_00,iVar7);
        // }
        // uStack12 = (pcVar2 >> 0x10);
        uStack14 = pcVar2;
        if ((uStack12 | uStack14) != 0x0) {
            call_fn_ptr_1000_0dc6(uStack14, uStack12, 0x1000);
        }
    }
    return;
}


pub fn pass1_1030_5164(param_1: u32, param_2: i32, param_3: u16) -> u32

{
    let u_var1: u16;
    let u_var2: u16;
    let lVar3: i32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x568));
    loop {
        lVar3 = pass1_1008_5b12(local_a, param_3);
        if (lVar3 == 0x0) {
            return param_2;
        }
        u_var1 = param_1 + 0x168;
        string_1000_3d3e((param_1 & 0xffff0000 | u_var1), (lVar3 + 0x4));
        string_1000_3cea(param_1 & 0xffff0000 | u_var1, param_2);
        u_var2 = dos3_call_1000_51aa(&stack0xfffe);
        if (u_var2 != 0x0) == false { break; }
    }
    return param_1 & 0xffff0000 | u_var1;
}


pub fn pass1_1030_51eb() {
    let unaff_SS: u16;

    pass1_1030_3b28(unaff_SS);
    return;
}


pub fn pass1_1030_51f0(param_1: u32) -> u32

{
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xa4) = 0x0;
    (i_var1 + 0xa6) = 0x0;
    (i_var1 + 0xa8) = 0x0;
    (i_var1 + 0xaa) = 0x0;
    (i_var1 + 0xac) = 0x0;
    return param_1;
}


pub fn pass1_1030_521c(param_1: &mut Struct100, param_2: u32, param_3: u16, param_4: u8) {
    let i_var1: i16;
    let pu_var2: U32Ptr;

    struct_op_1028_d1dc(param_3, param_4, param_1, 0x32c7);
    // pu_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x108) = param_2;
    param_1.field_0x0 = 0x55fe;
    (i_var1 + 0x2) = 0x1030;
    sys_1000_3f9c((i_var1 + 0x8), pu_var2, s_SCGenKids_0x_08lx_1050_5714,
                  ctx.data_seg, param_2, &stack0xfffe, pu_var2, 0x1000,
                  param_3, param_4);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_5260(param_1: u32, param_2: u16, param_3: u16) -> u16

{
    let u_var1: u32;
    let ppcVar2: u32;
    let puStack6: u32;

    u_var1 = (param_1 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    puStack6 = CONCAT22(param_3, param_2);
    ppcVar2 = (*puStack6 + 0x14);
    (**ppcVar2)();
    return 0x1;
}


pub fn pass1_1030_5290(param_1: u32, param_2: &mut Struct376, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct377;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10c, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        *puStack10 = 0x55fe;
        param_2.field_0x2 = 0x1030;
    }
    return;
}


pub fn pass1_1030_532e(param_1: &mut Struct100, param_2: u32, param_3: u16, param_4: u8) {
    let i_var1: i16;
    let pu_var2: U32Ptr;

    struct_op_1028_d1dc(param_3, param_4, param_1, 0x32c7);
    // pu_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x108) = param_2;
    param_1.field_0x0 = 0x55ee;
    (i_var1 + 0x2) = 0x1030;
    sys_1000_3f9c(ctx, (i_var1 + 0x8), pu_var2, s_SCSelect__u___d_1050_5726,
                  ctx.data_seg, (i_var1 + 0x4),
                  &stack0xfffe, pu_var2, 0x1000, param_3, param_4);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_538a(param_1: u32, param_2: i16, param_3: u16) -> u16

{
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let i_var4: &mut Struct694;
    let u_var3: u16;
    let puVar4: U32Ptr;

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (&i_var4.field_0x108 + 0x2);
    u_var2 = pu_var1 >> 0x8;
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_3, pu_var1, param_2);
    if (u_var2 == 0x1) {
        pass1_1018_04ca(puVar4, i_var4.field_0x108);
    } else {
        if (u_var2 == 0x2) {
            pass1_1018_04a4(puVar4, i_var4.field_0x108);
        }
    }
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_53f4(param_1: u32, param_2: u16, param_3: u16, param_4: u8) {
    let u_var1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;
    let bStack291: u8;
    u8
    local_11e[0x10e];
    let uStack16: u32;
    let uStack12: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    uStack12 = (i_var3 + 0x108);
    uStack12._3_1_ = (uStack12 >> 0x18);
    if (uStack12._3_1_ == -0x1) {
        u_var5 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2, param_2,
                                ((i_var3 + 0x108) >> 0x18));
        // param_2 = (u_var5 >> 0x10);
    } else {
        uStack16 = (i_var3 + 0x108);
        uStack16._3_1_ = (uStack16 >> 0x18);
        if (uStack16._3_1_ == '\x03') {
            pass1_1028_e44a(ctx.PTR_LOOP_1050_65e2, (i_var3 + 0x108), param_3);
        } else {
            u_var1 = (i_var3 + 0x108);
            pass1_1028_e372(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10),
                            param_3);
        }
    }
    uStack12 = (i_var3 + 0x108);
    uStack12._3_1_ = (uStack12 >> 0x18);
    if (uStack12._3_1_ != '\x03') {
        pass1_1030_521c(
            CONCAT13((param_3 >> 0x8), CONCAT12(param_3, local_11e)),
            (i_var3 + 0x108), param_3, param_4);
        uStack16 = *_PTR_LOOP_1050_5748;
        fn_ptr_1028_d566(uStack16, CONCAT22(param_3, local_11e));
        bStack291 = ((i_var3 + 0x108) >> 0x18);
        u_var2 = bStack291;
        if (bStack291 == 0x2) {
            u_var1 = (i_var3 + 0x108);
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
            pass1_1010_82f8(ctx.PTR_LOOP_1050_14cc, *(u_var2 + 0x10));
        }
    }
    return;
}


pub fn pass1_1030_54f8(param_1: &mut Struct378, param_2: U32Ptr, param_3: u32) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct379;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10c, param_2, 0x1000);
    puStack10 = CONCAT22(param_2, param_1);
    if ((param_2 | param_1) != 0x0) {
        *puStack10 = 0x389a;
        param_1.field_0x2 = 0x1008;
        // u_var6 = (param_3 >> 0x10);
        iVar5 = param_3;
        param_1.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_1.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_1.field_0x2 = &USHORT_1050_1028;
        param_1.field_0x108 = iVar5.field_0x108;
        *puStack10 = 0x55ee;
        param_1.field_0x2 = 0x1030;
    }
    return;
}


pub fn pass1_1030_5596(param_1: &mut Struct18, param_2: u8) -> *mut astruct_18

{
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1030_55c2(param_1: &mut Struct18, param_2: u8) -> *mut astruct_18

{
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1030_560e(param_1: U32Ptr) -> U32Ptr

{
    let i_var1: i16;
    let u_var2: u16;

    struct_1030_17ce(param_1, 0x64, 0x1f4);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x10) = 0x0;
    clear_struct_1008_3e38((param_1 & 0xffff0000 | (i_var1 + 0x14)));
    (i_var1 + 0x1a) = 0x0;
    (i_var1 + 0x1c) = 0x0;
    *param_1 = s_procLo_1050_5bd0;
    (i_var1 + 0x2) = 0x1030;
    return param_1;
}


pub fn pass1_1030_56b0(param_1: U32Ptr) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = s_procLo_1050_5bd0;
    (i_var3 + 0x2) = 0x1030;
    paVar2 = (i_var3 + 0x10);
    u_var1 = (i_var3 + 0x12);
    if ((u_var1 | paVar2) != 0x0) {
        fn_ptr_1030_84d0(paVar2 & 0xffff | u_var1 << 0x10);
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    }
    pass1_1030_18b2(param_1);
    return;
}


pub fn pass1_1030_56f6(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let BVar5: bool;
    let iVar6: i16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let local_e: [u16; 0x3];
    let local_8: [u16; 0x2];
    let i_stack4: i16;

    u_var4 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if (u_var4 != 0x0) {
        // uVar8 = (param_1 >> 0x10);
        iVar7 = param_1;
        local_e[0] = (iVar7 + 0x10);
        u_var4 = param_2;
        // uVar9 = (param_2 >> 0x10);
        BVar5 = write_to_file_1008_7e1c(u_var4, uVar9, local_e, param_4, 0x2, 0x1008);
        if (BVar5 != 0x0) {
            u_var3 = (iVar7 + 0x10);
            local_8[0] = (u_var3 + 0x2);
            BVar5 = write_to_file_1008_7e1c(u_var4, uVar9, local_8, param_4, 0x2, 0x1008);
            if ((BVar5 != 0x0) && (u_var3 = (iVar7 + 0x10),
                                   BVar5 = pass1_1008_7c2a(param_2, (u_var3 + 0x4), 0x1008),
                                   BVar5 != 0x0)) {
                u_var3 = (iVar7 + 0x10);
                local_8[0] = (u_var3 + 0x1a);
                BVar5 = write_to_file_1008_7e1c(u_var4, uVar9, local_8, param_4, 0x2, 0x1008);
                if (BVar5 != 0x0) {
                    // for (i_stack4 = 0x0; u_var3 = (iVar7 + 0x10),
                    //     pi_var1 = (u_var3 + 0x1a),
                    //     *pi_var1 != i_stack4 && i_stack4 <= *pi_var1; i_stack4 += 0x1) {
                    //   u_var3 = (iVar7 + 0x10);
                    //   u_var2 = (u_var3 + 0x16);
                    //   iVar6 = write_to_file_1008_7b4c
                    //                     (param_2,u_var2 & 0xffff0000 |
                    //                              (u_var2 + i_stack4 * 0x6),0x1008,
                    //                      param_4);
                    //   if (iVar6 == 0x0) goto LAB_1030_5734;
                    // }
                    iVar6 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | (iVar7 + 0x14), 0x1008,
                                                    param_4);
                    if (iVar6 != 0x0) {
                        local_8[0] = (iVar7 + 0x1c);
                        BVar5 = write_to_file_1008_7e1c(u_var4, uVar9, local_8, param_4, 0x2, 0x1008);
                        if (BVar5 != 0x0) {
                            return;
                        }
                    }
                }
            }
        }
//LAB_1030_5734:
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


pub fn pass1_1030_5a52(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x10);
    *param_3 = (u_var1 + 0xe);
    u_var1 = (param_1 + 0x10);
    *param_2 = (u_var1 + 0x12);
    return;
}


pub fn pass1_1030_5a80(param_1: u32, param_2: u32, param_3: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let u_var3: u32;
    u8
    local_20[0xc];
    let local_14: u32;
    let uStack14: u32;
    let uStack10: u32;
    let i_stack6: i16;
    let uStack4: u16;

    // u_var2 = (param_1 >> 0x10);
    (param_1 + 0x10) = param_2;
    u_var3 = pass1_1008_4772((param_2 + 0xe));
    // uStack4 = (u_var3 >> 0x10);
    i_stack6 = u_var3;
    uStack10 = (i_stack6 + 0x4);
    uStack14 = (i_stack6 + 0x8);
    pass1_1008_3e54(CONCAT22(param_3, &local_14), 0x0, uStack14 - 0x1,
                    uStack10 - 0x1);
    pu_var1 = (param_1 + 0x14);
    pass1_1008_6cb4(CONCAT22(param_3, local_20), &local_14, param_3, pu_var1, u_var2);
    pass1_1008_6d64(CONCAT22(param_3, local_20),
                    (param_1 & 0xffff0000 | ZEXT24(pu_var1)));
    return;
}



i16  pass1_1030_5b00(param_1: u32)

{
return (param_1 + 0x4) + 0xb;
}



pub fn pass1_1030_5b1c(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x1a);
    *param_2 = (param_1 + 0x1c);
    return;
}


pub fn pass1_1030_5b3e(param_1: u32, param_2: i16, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x1a) = param_3;
    if ((i_var1 + 0x1c) < param_2) {
        (i_var1 + 0x1c) = param_2;
    }
    return;
}


pub fn pass1_1030_5b5c(param_1: i16, param_2: u16) -> u32

{
    return CONCAT22(param_2, param_1 + 0x14);
}


pub fn pass1_1030_5b6c(param_1: u32, param_2: &mut String, param_3: u16) {
    let lVar1: i32;
    let u_var2: u16;
    let i_var4: &mut Struct610;
    let i_var3: &mut Struct609;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (i_var4.field_0x10 != 0x0) {
        lVar1 = i_var4.field_0x10;
        fn_ptr_1000_17ce(ctx, (lVar1 + 0x4), 0x1000);
        u_var2 = str_op_1008_60e8(param_2, param_3);
        lVar1 = i_var4.field_0x10;
        // u_var3 = (lVar1 >> 0x10);
        i_var3 = lVar1;
        i_var3.field_0x4 = u_var2;
        i_var3.field_0x6 = param_3;
    }
    return;
}



astruct_18 *  pass1_1030_5baa(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_56b0(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_5bec(param_1: u32) {
    ctx._PTR_LOOP_1050_5736 = param_1;
    pass1_1000_54a0(param_1, 0x0, 0x24);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_5c0e() {
    ctx._PTR_LOOP_1050_5736 = 0x0;
    return;
}


pub fn pass1_1030_5c1a(param_1: u32, param_2: u32, param_3: u16) -> bool

{
    let b_var1: bool;

    b_var1 = write_to_file_1008_7cac(param_2, param_3);
    if (b_var1 != 0x0) {
        b_var1 = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), param_1,
                                        (param_1 >> 0x10), 0x24, 0x1008);
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 0x1;
    }
    return b_var1;
}


pub fn pass1_1030_5c8a(param_1: u32, param_2: u32) {
    plVar1: &i32;
    let u_var2: u16;
    let u_var3: u32;
    let u_var4: u16;
    let iVar5: &mut Struct177;
    let u_var5: u16;
    let uStack6: u32;

    uStack6 = 0x0;
    u_var2 = param_2._3_1_;
    if (u_var2 == 0xff) {
        return;
    }
    u_var5 = (ctx.PTR_LOOP_1050_65e2 >> 0x10);
    iVar5 = (ctx.PTR_LOOP_1050_65e2 + 0xa);
    u_var3 = (iVar5 + u_var2 * 0x4);
    u_var4 = (iVar5 + u_var2 * 0x4 + 0x2);
    if ((u_var3 + 0x4) != 0x0) {
        pass1_1030_12ca(u_var3 & 0xffff | u_var4 << 0x10);
        uStack6 = u_var3 & 0xffff | u_var4 << 0x10;
    }
    if (uStack6 == 0x0) {
        plVar1 = (u_var2 * 0x4 + param_1);
        *plVar1 = *plVar1 + 0x1;
    }
    return;
}


pub fn pass1_1030_5d0a(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    struct_1030_17ce(param_1, 0x1, 0x4);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x10) = 0x0;
    *param_1 = 0x613e;
    (param_1 + 0x2) = 0x1030;
    return param_1;
}


pub fn pass1_1030_5d3c(param_1: U32Ptr, param_2: u32, param_3: u16, param_4: U32Ptr) -> u16

{
    let u_var1: u16;

    pass1_1030_183c(param_1, 0x1, 0x4, 0x1000000, param_2, param_3, param_4);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x10) = 0x0;
    *param_1 = 0x613e;
    (param_1 + 0x2) = 0x1030;
    return param_1;
}


pub fn pass1_1030_5d78(param_1: U32Ptr) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x613e;
    (i_var3 + 0x2) = 0x1030;
    paVar2 = (i_var3 + 0x10);
    u_var1 = (i_var3 + 0x12);
    if ((u_var1 | paVar2) != 0x0) {
        pass1_1030_8480((astruct_18 * *)(paVar2 & 0xffff | u_var1 << 0x10));
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    }
    pass1_1030_18b2(param_1);
    return;
}


pub fn pass1_1030_5dbe(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let iVar5: i16;
    let iVar6: i16;
    let uVar7: u16;
    let local_c: [u16; 0x5];

    u_var3 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if (u_var3 != 0x0) {
        // uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        BVar4 = pass1_1008_7c2a(param_2, (iVar6 + 0x10), 0x1008);
        if ((BVar4 != 0x0) && (u_var1 = (iVar6 + 0x10),
                               iVar5 = write_to_file_1008_7b4c(param_2, u_var1 & 0xffff0000 | (u_var1 + 0x4), 0x1008,
                                                               param_4), iVar5 != 0x0)) {
            u_var2 = (iVar6 + 0x10);
            local_c[0] = (u_var2 + 0xa);
            // u_var3 = (param_2 >> 0x10);
            BVar4 = write_to_file_1008_7e1c(param_2, u_var3, local_c, param_4, 0x2, 0x1008);
            if (BVar4 != 0x0) {
                u_var2 = (iVar6 + 0x10);
                if ((u_var2 + 0xa) == 0x0) {
                    return;
                }
                u_var2 = (iVar6 + 0x10);
                // uVar7 = (u_var2 >> 0x10);
                iVar6 = u_var2;
                u_var2 = (iVar6 + 0xc);
                BVar4 = write_to_file_1008_7e1c(param_2, u_var3, u_var2,
                                                (u_var2 >> 0x10),
                                                ((iVar6 + 0xa) * 0x2), 0x1008);
                if (BVar4 != 0x0) {
                    return;
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn file_1030_5e70(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let BVar6: bool;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let i_var9: i16;
    let unaff_DI: i16;
    let u_var10: u16;
    let puVar11: U32Ptr;
    let iVar12: i16;
    let uVar13: u16;
    let uVar14: u16;
    let uStack1034: u32;
    let local_402: [u8; 400];

    iVar12 = param_1;
    // uVar13 = (param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if (param_3 != 0x0) {
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4, 0x1000);
            ctx.PTR_LOOP_1050_5f2e = param_4;
        } else {}
        u_var3 = fn_ptr_op_1000_1708(0x10, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                    ctx.PTR_LOOP_1050_5f2e, 0x1000);
        uStack1034 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var3);
        puVar8 = (ctx.PTR_LOOP_1050_5f2e | u_var3);
        if (puVar8 == 0x0) {
            (iVar12 + 0x10) = 0x0;
        } else {
            puVar11 = clear_struct_1008_3e38(CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var3 + 0x4));
            // puVar8 = (puVar11 >> 0x10);
            (iVar12 + 0x10) = uStack1034;
        }
        puVar4 = local_402;
        u_var3 = param_2;
        // uVar14 = (param_2 >> 0x10);
        read_file_1008_7c6e(u_var3, uVar14, CONCAT22(param_5, puVar4), 0x1008);
        if (puVar4 != 0x0) {
            u_var5 = str_op_1008_60e8(CONCAT22(param_5, local_402), puVar8);
            puVar11 = (iVar12 + 0x10);
            *puVar11 = u_var5;
            (puVar11 + 0x2) = puVar8;
            u_var1 = (iVar12 + 0x10);
            BVar6 = read_file_1008_7bc8(param_2,
                                        (u_var1 & 0xffff0000 | (u_var1 + 0x4)), 0x1008, param_5);
            if (BVar6 != 0x0) {
                u_var2 = (iVar12 + 0x10);
                BVar6 = read_file_1008_7dee(u_var3, uVar14, u_var2 + 0xa, 0x0,
                                            (u_var2 >> 0x10), 0x2, 0x1008);
                if (BVar6 != 0x0) {
                    u_var2 = (iVar12 + 0x10);
                    // u_var10 = (u_var2 >> 0x10);
                    i_var9 = u_var2;
                    if ((i_var9 + 0xa) == 0x0) {
//LAB_1030_5fb7:
                        puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_5, puVar8, unaff_DI);
                        pass1_1018_04ca(puVar11, (iVar12 + 0x4));
                        return;
                    }
                    u_var5 = (i_var9 + 0xa) * 0x2;
                    uVar7 = u_var5;
                    mem_op_1000_179c(u_var5, puVar8, 0x1000);
                    u_var2 = (iVar12 + 0x10);
                    // u_var10 = (u_var2 >> 0x10);
                    i_var9 = u_var2;
                    (i_var9 + 0xc) = uVar7;
                    (i_var9 + 0xe) = puVar8;
                    u_var2 = (iVar12 + 0x10);
                    u_var2 = (u_var2 + 0xc);
                    BVar6 = read_file_1008_7dee(u_var3, uVar14, u_var2, 0x0,
                                                (u_var2 >> 0x10), u_var5, 0x1008);
                    if (BVar6 != 0x0) {
                        // goto
                        // LAB_1030_5fb7;
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}


pub fn pass1_1030_5fe2(param_1: u32, param_2: u32) {
    (param_1 + 0x10) = param_2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_5ff6(param_1: u32, param_2: u16, param_3: U32Ptr, param_4: U32Ptr, param_5: u8) {
    let pu_var1: U32Ptr;
    let ppcVar2: u32;
    let pu_var3: u32;
    let u_var4: u32;
    let iVar5: i16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    ulocal_6c: u8[0x58];
    let uStack20: u32;
    let uStack16: u32;
    let uStack12: u32;
    let uStack8: u16;
    let puStack6: U32Ptr;
    let uStack4: u16;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    puVar7 = param_3;
    if ((i_var8 + 0xc) == 0x0) {
        mem_op_1000_179c(0x18, param_3, 0x1000);
        puVar7 = (param_3 | param_2);
        uStack8 = param_2;
        puStack6 = param_3;
        if (puVar7 == 0x0) {
            (i_var8 + 0xc) = 0x0;
        } else {
            struct_op_1030_1cd8(CONCAT22(param_3, param_2), 0x5, 0x5);
            (i_var8 + 0xc) = param_2;
            (i_var8 + 0xe) = extraout_dx;
            puVar7 = extraout_dx;
        }
    }
    // for (uStack4 = 0x0; u_var4 = (i_var8 + 0x10),
    //     pu_var1 = (u_var4 + 0xa), uStack4 <= *pu_var1 && *pu_var1 != uStack4;
    //     uStack4 += 0x1) {
    //   uStack12 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2,puVar7,0x2);
    //   iVar5 = uStack12;
    //   ppcVar2 = ((i_var8 + 0xc) + 0x8);
    //   (**ppcVar2)(&USHORT_1050_1028,(i_var8 + 0xc),iVar5,
    //               (uStack12 >> 0x10),uStack4,0x0);
    //   puVar7 = extraout_DX_00;
    //   pass1_1030_8344(ctx.PTR_LOOP_1050_5748,
    //                   (ctx.PTR_LOOP_1050_5748 >> 0x10),uStack12);
    //   uStack16 = CONCAT22(puVar7,iVar5);
    //   uStack20 = (iVar5 + 0x10);
    //   if ((uStack20 + 0x2) == 0x0) {
    //     pu_var3 = (i_var8 + 0x10);
    //     sys_1000_3f9c(local_6c,param_4,s__s__d_1050_573a,ctx.data_seg,
    //                   *pu_var3,&stack0xfffe,(pu_var3 >> 0x10),0x1000,
    //                   param_4,param_5);
    //     u_var6 = str_op_1008_60e8(CONCAT22(param_4,local_6c),puVar7);
    //     u_var10 = (uStack20 >> 0x10);
    //     (uStack20 + 0x2) = u_var6;
    //     (uStack20 + 0x4) = puVar7;
    //   }
    // }
    return;
}



astruct_18 *  pass1_1030_6118(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_5d78(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_615a(param_1: &mut Struct137, param_2: u16) {
    let u_var1: u16;
    let extraout_dx: u16;
    let i_var2: &mut Struct137;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = 0x0;
    param_1 = 0x0;
    &i_var2.field_0x4 = 0x0;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    if ((param_2 | u_var1) == 0x0) {
        &i_var2.field_0x4 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(param_2, u_var1));
        i_var2.field_0x4 = u_var1;
        i_var2.field_0x6 = extraout_dx;
    }
    ctx._PTR_LOOP_1050_5740 = param_1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_61b0(param_1: U32Ptr) {
    let u_var1: u16;
    let pu_var2: u32;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = (i_var4 + 0x2);
    if ((u_var1 | *param_1) != 0x0) {
        ppc_var3 = *param_1;
        (**ppc_var3)();
    }
    pu_var2 = (i_var4 + 0x4);
    u_var1 = (i_var4 + 0x6);
    if ((u_var1 | pu_var2) != 0x0) {
        ppc_var3 = *pu_var2;
        (**ppc_var3)();
    }
    ctx._PTR_LOOP_1050_5740 = 0x0;
    return;
}


pub fn pass1_1030_61fe(param_1: u32, param_2: u32, param_3: u32, param_4: i32, param_5: u16,
                       param_6: u16, param_7: u16)

{
    pass1_1030_677a(param_1, param_4, param_7);
    pass1_1030_8aa0(CONCAT22(param_6, param_5), param_2, param_3, param_6, param_7);
    return;
}



u16
pass1_1030_6222(param_1: u32,param_2: i16,param_3: u32,param_4: u32,param_5: u16,
param_6: U32Ptr,param_7: u16)

{
let ppcVar1: u32; let u_var2: u16;
let extraout_dx: u16; let uStack6: u32;

mem_op_1000_179c(0x4c, param_6, 0x1000); u_var2 = param_6 | param_5; if (u_var2 == 0x0) {
param_5 = 0x0; u_var2 = 0x0;
}
else {
pass1_1030_88ce(CONCAT22(param_6, param_5), param_3, param_4, param_7);
}
uStack6 = CONCAT22(u_var2, param_5); ppcVar1 = ((param_1 + 0x4) + 0x4); ( * * ppcVar1)(); if (param_2 != 0x0) {
pass1_1030_8d08(uStack6, extraout_dx);
}
return 0x0;
}



pub fn pass1_1030_627e(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: U32Ptr,
                       param_6: i32)

{
    let local_12: [u32; 0x2];
    let uStack10: u32;
    let uStack6: u32;

    uStack6 = 0x0;
    pass1_1030_677a(param_4, param_6, param_1);
    uStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        pass1_1030_8b00(uStack10, param_5, CONCAT22(param_1, local_12), param_1);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_62e4(param_1: U32Ptr, param_2: U32Ptr, param_3: i32, param_4: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let extraout_dx: U32Ptr;
    let puVar4: U32Ptr;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let u_var5: u16;
    i16
    local_64[0x3];
    let uStack94: u32;
    let uStack88: u16;
    let uStack78: u16;
    let uStack76: u16;
    let local_40: u32;
    let uStack60: u32;
    let uStack56: u16;
    let puStack54: u32;
    let puStack52: U32Ptr;
    let puStack50: u32;
    let puStack48: U32Ptr;
    let uStack46: u16;
    let iStack44: i16;
    let local_2a: [u8; 2];
    let local_28: i16;
    let local_26: i16;
    let local_24: u16;
    let local_22: [u8; 2];
    let local_20: [u8; 2];
    let local_1e: u16;
    let local_1c: u16;
    let local_1a: u16;
    let local_18: [u8; 6];
    let local_12: [u8; 6];
    let local_c: [u8; 6];
    let uStack6: u32;

    // u_var5 = (param_1 >> 0x10);
    pu_var2 = param_1;
    puVar4 = (param_1 + 0x2);
    puStack54 = pu_var2;
    puStack52 = puVar4;
    puStack50 = pu_var2;
    puStack48 = puVar4;
    if ((puVar4 | pu_var2) != 0x0) {
        ppcVar1 = *pu_var2;
        (**ppcVar1)();
        puVar4 = extraout_dx;
    }
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puStack54 = pu_var2;
    puStack52 = puVar4;
    if ((puVar4 | pu_var2) == 0x0) {
        pu_var2 = 0x0;
        u_var3 = 0x0;
    } else {
        struct_op_1030_1cd8(CONCAT22(puVar4, pu_var2), 0x5, 0x5);
        u_var3 = extraout_DX_00;
    }
    param_1 = pu_var2;
    (param_1 + 0x2) = u_var3;
    pass1_1030_677a(param_1, param_3, param_4);
    uStack6 = CONCAT22(u_var3, pu_var2);
    if ((u_var3 | pu_var2) != 0x0) {
        clear_struct_1008_3e38(CONCAT22(param_4, local_c));
        clear_struct_1008_3e38(CONCAT22(param_4, local_12));
        clear_struct_1008_3e38(CONCAT22(param_4, local_18));
        pass1_1008_6d3e(param_2, CONCAT22(param_4, local_12),
                        CONCAT22(param_4, local_c));
        pass1_1008_3eb4(CONCAT22(param_4, local_c),
                        CONCAT22(param_4, &local_1e),
                        CONCAT22(param_4, &local_1c),
                        CONCAT22(param_4, &local_1a));
        pass1_1008_3eb4(CONCAT22(param_4, local_12),
                        CONCAT22(param_4, &local_24),
                        CONCAT22(param_4, local_22),
                        CONCAT22(param_4, local_20));
        pass1_1008_6d64(param_2, CONCAT22(param_4, local_18));
        pass1_1008_3eb4(CONCAT22(param_4, local_18),
                        CONCAT22(param_4, local_2a),
                        CONCAT22(param_4, &local_28),
                        CONCAT22(param_4, &local_26));
        if (local_24 == local_1e) {
            iStack44 = 0x0;
            // for (uStack46 = local_1c; u_var3 = local_28 + local_1c, uStack46 < u_var3;
            //     uStack46 += 0x1) {
            //   for (uStack56 = local_1a; uStack56 < (local_26 + local_1a);
            //       uStack56 += 0x1) {
            //     uStack88 = local_1e;
            //     pass1_1008_3e54(
            //                     CONCAT13((param_4 >> 0x8),CONCAT12(param_4,local_64)
            //                             ),local_1e,uStack46,uStack56);
            //     pass1_1030_8b00(uStack6,CONCAT22(param_4,local_64),
            //                     CONCAT22(param_4,&local_40),param_4);
            //     uStack60 = local_40;
            //     local_64[0] = iStack44;
            //     uStack60._0_2_ = local_40;
            //     uStack78 = uStack60;
            //     uStack76 = local_40._2_2_;
            //     uStack76._1_1_ = (local_40 >> 0x18);
            //     if (uStack76._1_1_ == '\0') {
            //       uStack60._0_2_ = 0x0;
            //       local_40._2_2_ = 0x0;
            //     }
            //     uStack94 = CONCAT22(local_40._2_2_,uStack60);
            //     ppcVar1 = (*param_1 + 0x8);
            //     iStack44 = iStack44 + 0x1;
            //     (**ppcVar1)();
            //   }
            // }
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)(0x1008, *param_1);
            if ((extraout_DX_01 | u_var3) != 0x0) {
                return;
            }
        }
    }
    return;
}


pub fn pass1_1030_64ce(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: U32Ptr, param_6: i32, param_7: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let local_e: u32;
    let uStack10: u32;
    let uStack6: u32;

    uStack6 = 0x0;
    pass1_1030_677a(param_4, param_6, param_1);
    uStack10 = CONCAT22(param_3, param_2);
    u_var2 = param_3 | param_2;
    if (u_var2 != 0x0) {
        pu_var1 = &local_e;
        pass1_1030_8b00(uStack10, param_5, CONCAT22(param_1, pu_var1), param_1);
        uStack6 = *pu_var1;
    }
    *param_7 = uStack6;
    return;
}


pub fn pass1_1030_6522(param_1: U32Ptr, param_2: u32, param_3: u32, param_4: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let extraout_dx: U32Ptr;
    let puVar4: U32Ptr;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let u_var5: u16;
    u8
    local_64[0xc];
    let uStack88: u16;
    let local_40: u32;
    let uStack60: u32;
    let uStack56: u16;
    let puStack54: u32;
    let puStack52: U32Ptr;
    let puStack50: u32;
    let puStack48: U32Ptr;
    let uStack46: u16;
    let iStack44: i16;
    let local_2a: [u8; 2];
    let local_28: i16;
    let local_26: i16;
    let local_24: u16;
    let local_22: [u8; 2];
    let local_20: [u8; 2];
    let local_1e: u16;
    let local_1c: u16;
    let local_1a: u16;
    let local_18: [u8; 6];
    let local_12: [u8; 6];
    let local_c: [u8; 6];
    let uStack6: u32;

    // u_var5 = (param_1 >> 0x10);
    pu_var2 = param_1;
    puVar4 = (param_1 + 0x2);
    puStack54 = pu_var2;
    puStack52 = puVar4;
    puStack50 = pu_var2;
    puStack48 = puVar4;
    if ((puVar4 | pu_var2) != 0x0) {
        ppcVar1 = *pu_var2;
        (**ppcVar1)();
        puVar4 = extraout_dx;
    }
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puStack54 = pu_var2;
    puStack52 = puVar4;
    if ((puVar4 | pu_var2) == 0x0) {
        pu_var2 = 0x0;
        u_var3 = 0x0;
    } else {
        struct_op_1030_1cd8(CONCAT22(puVar4, pu_var2), 0x5, 0x5);
        u_var3 = extraout_DX_00;
    }
    param_1 = pu_var2;
    (param_1 + 0x2) = u_var3;
    pass1_1030_677a(param_1, param_3, param_4);
    uStack6 = CONCAT22(u_var3, pu_var2);
    if ((u_var3 | pu_var2) != 0x0) {
        clear_struct_1008_3e38(CONCAT22(param_4, local_c));
        clear_struct_1008_3e38(CONCAT22(param_4, local_12));
        clear_struct_1008_3e38(CONCAT22(param_4, local_18));
        pass1_1008_6d3e(param_2, CONCAT22(param_4, local_12),
                        CONCAT22(param_4, local_c));
        pass1_1008_3eb4(CONCAT22(param_4, local_c),
                        CONCAT22(param_4, &local_1e),
                        CONCAT22(param_4, &local_1c),
                        CONCAT22(param_4, &local_1a));
        pass1_1008_3eb4(CONCAT22(param_4, local_12),
                        CONCAT22(param_4, &local_24),
                        CONCAT22(param_4, local_22),
                        CONCAT22(param_4, local_20));
        pass1_1008_6d64(param_2, CONCAT22(param_4, local_18));
        pass1_1008_3eb4(CONCAT22(param_4, local_18),
                        CONCAT22(param_4, local_2a),
                        CONCAT22(param_4, &local_28),
                        CONCAT22(param_4, &local_26));
        if (local_24 == local_1e) {
            iStack44 = 0x0;
            // for (uStack46 = local_1c; u_var3 = local_28 + local_1c, uStack46 < u_var3;
            //     uStack46 += 0x1) {
            //   for (uStack56 = local_1a; uStack56 < (local_26 + local_1a);
            //       uStack56 += 0x1) {
            //     uStack88 = local_1e;
            //     pass1_1008_3e54(
            //                     CONCAT13((param_4 >> 0x8),CONCAT12(param_4,local_64)
            //                             ),local_1e,uStack46,uStack56);
            //     pass1_1030_8b00(uStack6,CONCAT22(param_4,local_64),
            //                     CONCAT22(param_4,&local_40),param_4);
            //     uStack60 = local_40;
            //     iStack44 += 0x1;
            //     ppcVar1 = (*param_1 + 0x8);
            //     (**ppcVar1)();
            //   }
            // }
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)(0x1008, *param_1);
            if ((extraout_DX_01 | u_var3) != 0x0) {
                return;
            }
        }
    }
    return;
}


pub fn pass1_1030_66de(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x4));
    loop {
        u_var1 = pass1_1008_5b12(local_a, param_3);
        if (u_var1 == 0x0) { break; }
        pass1_1030_8bac(u_var1, param_2);
    }
    return;
}


pub fn pass1_1030_671c(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: i32, param_5: u16, param_6: u16, param_7: i16, param_8: u16)

{
    pass1_1030_677a(param_1, param_4, param_8);
    pass1_1030_8bdc(CONCAT22(param_6, param_5), param_2, param_3, param_7, param_8);
    return;
}


pub fn pass1_1030_6740(param_1: u32, param_2: u16, param_3: i16) {
    let u_var1: u32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_2, local_a), (param_1 + 0x4));
    loop {
        u_var1 = pass1_1008_5b12(local_a, param_2);
        if (u_var1 == 0x0) { break; }
        pass1_1030_8c38(u_var1, param_3, param_2);
    }
    return;
}


pub fn pass1_1030_677a(param_1: u32, param_2: i32, param_3: u16) {
    let pu_var1: U32Ptr;
    let extraout_dx: u16;
    let u_var2: u16;
    let local_a: [u8; 8];

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x4) == 0x0) {
        return;
    }
    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x4));
    loop {
        pu_var1 = local_a;
        pass1_1008_5b12(pu_var1, param_3);
        if ((extraout_dx | pu_var1) == 0x0) {
            return;
        }
        if ((pu_var1 + 0x24) != param_2) == false { break; }
    }
    return;
}


pub fn pass1_1030_67cc(param_1: U32Ptr) {
    let i_var1: &mut Struct687;
    let u_var1: u16;

    struct_1030_1628(param_1);
    i_var1 = param_1;
    i_var1 = &i_var1.field_0xc;
    clear_struct_1008_3e38((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    i_var1.field_0x12 = 0x0;
    i_var1.field_0x14 = 0x0;
    i_var1.field_0x16 = 0x0;
    i_var1.field_0x1a = 0x0;
    i_var1.field_0x1e = 0x0;
    i_var1.field_0x22 = 0x0;
    i_var1.field_0x26 = 0x0;
    i_var1.field_0x2a = 0x0;
    i_var1.field_0x2e = 0x0;
    i_var1.field_0x32 = 0x0;
    i_var1.field_0x34 = 0x0;
    i_var1.field_0x38 = 0x0;
    i_var1.field_0x36 = 0x0;
    i_var1.field_0x3c = 0x0;
    i_var1.field_0x3a = 0x0;
    i_var1.field_0x40 = 0x0;
    i_var1.field_0x3e = 0x0;
    *param_1 = 0x8114;
    i_var1.field_0x2 = 0x1030;
    return;
}


pub fn pass1_1030_684c(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: u16,
                       param_5: u16, param_6: u32, param_7: u16)

{
    let i_var1: i16;
    let u_var2: u16;

    pass1_1030_165e(param_1, 0x5000000, param_6, param_7);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xc) = *param_2;
    (i_var1 + 0x10) = (param_2 + 0x1);
    (i_var1 + 0x12) = param_4;
    (i_var1 + 0x14) = param_4;
    (i_var1 + 0x16) = 0x0;
    (i_var1 + 0x1a) = 0x0;
    (i_var1 + 0x1e) = 0x0;
    (i_var1 + 0x22) = 0x0;
    (i_var1 + 0x26) = 0x0;
    (i_var1 + 0x2a) = 0x0;
    (i_var1 + 0x2e) = 0x0;
    (i_var1 + 0x32) = 0x0;
    (i_var1 + 0x34) = 0x0;
    (i_var1 + 0x36) = 0x0;
    (i_var1 + 0x3a) = 0x0;
    (i_var1 + 0x3e) = 0x0;
    *param_1 = 0x8114;
    (i_var1 + 0x2) = 0x1030;
    return;
}


pub fn pass1_1030_68dc(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: u32;
    let paVar4: &mut Struct18;
    let ppcVar5: u32;
    let iVar6: &mut Struct611;
    let u_var6: u16;
    let paStack10: &mut Struct18;

    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x8114;
    iVar6.field_0x2 = 0x1030;
    paVar4 = &iVar6.field_0x22;
    u_var1 = iVar6.field_0x24;
    if ((u_var1 | paVar4) != 0x0) {
        fn_ptr_1020_ba7e((paVar4 & 0xffff | u_var1 << 0x10));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, paVar4, 0x1000);
    }
    u_var1 = iVar6.field_0x26;
    u_var2 = iVar6.field_0x28;
    paStack10 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        fn_ptr_1020_ba7e(CONCAT22(u_var2, u_var1));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
    }
    pu_var3 = iVar6.field_0x1e;
    u_var1 = iVar6.field_0x20;
    if ((u_var1 | pu_var3) != 0x0) {
        ppcVar5 = *pu_var3;
        (**ppcVar5)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar6.field_0x36;
    u_var1 = iVar6.field_0x38;
    if ((u_var1 | pu_var3) != 0x0) {
        ppcVar5 = *pu_var3;
        (**ppcVar5)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar6.field_0x3a;
    u_var1 = iVar6.field_0x3c;
    if ((u_var1 | pu_var3) != 0x0) {
        ppcVar5 = *pu_var3;
        (**ppcVar5)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar6.field_0x3e;
    u_var1 = iVar6.field_0x40;
    if ((u_var1 | pu_var3) != 0x0) {
        ppcVar5 = *pu_var3;
        (**ppcVar5)(param_2, pu_var3, u_var1, 0x1);
    }
    pass1_1030_16b2(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_69cc(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x3e) != 0x0) {
        return;
    }
    if (((i_var3 + 0x22) != 0x0) && (pass1_1020_ba94((i_var3 + 0x22)), (param_3 | param_2) != 0x0)) {
        return;
    }
    u_var1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x4);
    if ((BVar2 != 0x0) && (u_var5 = pass1_1028_67d4((i_var3 + 0x1a), param_4),
                           ((u_var5 >> 0x10) | u_var5) != 0x0)) {
        return;
    }
    return;
}


pub fn pass1_1030_6a2c(param_1: u32, param_2: i32, param_3: u16, param_4: U32Ptr, param_5: u16) {
    let ppcVar1: u32;
    let i_var2: &mut Struct384;
    let u_var2: u16;
    let i_var4: &mut Struct382;
    let iVar5: &mut Struct383;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let lVar6: i32;
    let local_a: [u8; 8];

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (i_var4.field_0x3e == 0x0) {
        mem_op_1000_179c(0xc, param_4, 0x1000);
        if ((param_4 | param_3) == 0x0) {
            i_var4.field_0x3e = 0x0;
        } else {
            u_var5 = set_struct_1008_574a(CONCAT22(param_4, param_3));
            &i_var4.field_0x3e = u_var5;
            (&i_var4.field_0x3e + 0x2) = (u_var5 >> 0x10);
        }
    }
    pass1_1008_5784(CONCAT22(param_5, local_a), i_var4.field_0x3e);
    loop {
        loop {
            lVar6 = pass1_1008_5b12(local_a, param_5);
            // u_var2 = (lVar6 >> 0x10);
            i_var2 = lVar6;
            if (lVar6 == 0x0) {
                // goto
                // LAB_1030_6af4;
            }
            // u_var4 = (param_2 >> 0x10);
            iVar5 = param_2;
            if ((i_var2.field_0x6 != iVar5.field_0x6) || (i_var2.field_0x4 != iVar5.field_0x4)) == false { break; }
        }
        if (i_var2.field_0x8 != iVar5.field_0x8) == false { break; }
    }
    i_var2.field_0xa = i_var2.field_0xa + iVar5.field_0xa;
    i_var2.field_0xc = i_var2.field_0xc + iVar5.field_0xc;
    param_2 = 0x0;
//LAB_1030_6af4:
    if (param_2 != 0x0) {
        ppcVar1 = (*i_var4.field_0x3e + 0x8);
        (**ppcVar1)(0x1008, i_var4.field_0x3e, param_2);
    }
    return;
}


pub fn pass1_1030_6b16(param_1: u32) -> u32

{
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u32;
    let iVar5: &mut Struct412;
    let u_var5: u16;
    let u_var6: u32;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (&iVar5.field_0x3a == 0x0) {
        return 0x0;
    }
    ppc_var3 = (&iVar5.field_0x3a + 0x10);
    u_var6 = (**ppc_var3)();
    u_var4 = &iVar5.field_0x3a;
    if ((u_var4 + 0x8) == 0x0) {
        pu_var1 = &iVar5.field_0x3a;
        u_var2 = iVar5.field_0x3c;
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        &iVar5.field_0x3a = 0x0;
    }
    return u_var6;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6b86(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let extraout_dx: u16;
    let u_var3: u16;
    let extraout_DX_00: u16;
    let i_var4: i16;
    let u_var5: u16;
    let uStack12: u32;
    let uStack8: u32;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x1e) == 0x0) {
        param_2 = 0x0;
        u_var3 = 0x0;
    } else {
        ppcVar1 = ((i_var4 + 0x1e) + 0x10);
        (**ppcVar1)();
        u_var3 = extraout_dx;
    }
    uStack8 = CONCAT22(u_var3, param_2);
    // for (uStack12 = 0x0; uStack12 < uStack8; uStack12 += 0x1) {
    //   ppcVar1 = ((i_var4 + 0x1e) + 0x4);
    //   u_var2 = uStack8;
    //   (**ppcVar1)(param_3,(i_var4 + 0x1e));
    //   if ((extraout_DX_00 | u_var2) != 0x0) {
    //     param_3 = SUB42(&USHORT_1050_1028,0x0);
    //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var2,extraout_DX_00);
    //   }
    // }
    return;
}


pub fn pass1_1030_6c1a(param_1: u32, param_2: i16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    i_var2 = (i_var3 + 0x32);
    (i_var3 + 0x32) = param_2;
    pi_var1 = (i_var3 + 0x34);
    *pi_var1 = *pi_var1 + (param_2 - i_var2);
    i_var2 = (i_var3 + 0x32);
    if (i_var2 < 0x0) {
        i_var2 = 0x0;
    }
    (i_var3 + 0x32) = i_var2;
    return;
}


pub fn pass1_1030_6c4c(param_1: u32, param_2: i16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = (param_1 + 0x32);
    if (param_2 < i_var1) {
        i_var1 = param_2;
    }
    (param_1 + 0x34) = i_var1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6c66(param_1: u32, param_2: i16, param_3: u32, param_4: u16, param_5: U32Ptr,
                       param_6: u16)

{
    let u_var1: u16;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let BVar5: bool;
    let extraout_dx: u16;
    let puVar6: U32Ptr;
    let iVar7: &mut Struct386;
    let iVar6: &mut Struct385;
    let unaff_SI: u16;
    let unaff_DI: u16;
    let uVar7: u16;
    let uVar8: u16;
    let unaff_SS: u16;

    // uVar7 = (param_1 >> 0x10);
    iVar7 = param_1;
    if (iVar7.field_0x3a == 0x0) {
        param_6 = 0x1000;
        mem_op_1000_179c(0xc, param_5, 0x1000);
        if ((param_5 | param_4) == 0x0) {
            iVar7.field_0x3a = 0x0;
        } else {
            param_6 = 0x1008;
            set_struct_1008_574a(CONCAT22(param_5, param_4));
            &iVar7.field_0x3a = param_4;
            (&iVar7.field_0x3a + 0x2) = extraout_dx;
        }
    }
    ppcVar2 = (*iVar7.field_0x3a + 0x8);
    (**ppcVar2)(param_6, iVar7.field_0x3a, param_3);
    if (param_2 != 0x0) {
        // uVar8 = (param_3 >> 0x10);
        iVar6 = param_3;
        if (iVar6.field_0x6 != 0x0) {
            pass1_1030_6e9c(param_1, iVar6.field_0xa, iVar6.field_0x6);
            return;
        }
        if (iVar6.field_0x4 != 0x0) {
            u_var1 = iVar6.field_0xa;
            u_var3 = -u_var1;
            puVar6 = -(u_var1 != 0x0);
            pass1_1030_7ddc(param_1, CONCAT13((puVar6 >> 0x8),
                                              CONCAT12(puVar6, u_var3)), iVar6.field_0x4,
                            u_var3, puVar6, unaff_SI, unaff_DI, unaff_SS);
            return;
        }
        if (iVar6.field_0x8 != 0x0) {
            u_var4 = pass1_1030_6fa0(param_1);
            BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var4, 0x4);
            if (BVar5 != 0x0) {
                pass1_1028_6356(iVar7.field_0x1a, 0x0, iVar6.field_0xa, 0x0, unaff_SS);
            }
        }
    }
    return;
}


pub fn pass1_1030_6d4e(param_1: u32, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    let u_var1: u16;
    let uStack6: u16;
    let uStack4: u16;

    uStack6 = 0x0;
    uStack4 = 0x0;
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x36) != 0x0) {
        pass1_1010_9092((param_1 + 0x36), param_2, param_4);
        uStack6 = param_2;
        uStack4 = param_3;
    }
    return CONCAT22(uStack4, uStack6);
}


pub fn pass1_1030_6d80(param_1: u32, param_2: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct299;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = &i_var4.field_0x36;
    u_var2 = (&i_var4.field_0x36 + 0x2);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    i_var4.field_0x36 = param_2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6db4(param_1: U32Ptr, param_2: i16, param_3: u16) -> u16

{
    let pu_var1: U32Ptr;

    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_3, param_1, param_2);
    pass1_1010_ed3e(pu_var1);
    return (pu_var1 + 0x18);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6ddc(param_1: u32) {
    let u_var1: u16;
    let BVar2: bool;

    u_var1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1e);
    if (BVar2 != 0x0) {
        pass1_1030_d0c6((param_1 + 0x1a));
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6e14(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;
    let BVar3: bool;

    u_var2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x1e);
    if (BVar3 != 0x0) {
        u_var1 = (param_1 + 0x1a);
        pass1_1030_d102(u_var1, (u_var1 >> 0x10));
        return;
    }
    return;
}


pub fn pass1_1030_6e4c(param_1: u32) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var3 << 0x10);
    }
    if (((i_var2 + 0x1a) != 0x0) && (u_var1 = (i_var2 + 0x1a), (u_var1 + 0x12) == 0x4)) {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6e9c(param_1: u32, param_2: i32, param_3: i16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let iVar6: &mut Struct301;
    let u_var6: u16;
    let unaff_SS: u16;
    let uStack10: u32;
    let uStack6: u32;

    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    u_var2 = (&iVar6.field_0x1e + 0x2) | &iVar6.field_0x1e;
    if (u_var2 != 0x0) {
        ppcVar1 = (*iVar6.field_0x1e + 0x10);
        (**ppcVar1)();
        uStack6 = CONCAT22(extraout_dx, u_var2);
        // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
        //   ppcVar1 = (*iVar6.field_0x1e + 0x4);
        //   u_var4 = uStack6;
        //   (**ppcVar1)();
        //   u_var2 = u_var4;
        //   u_var5 = extraout_DX_00 | u_var2;
        //   if (u_var5 != 0x0) {
        //     u_var3 = u_var2;
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var2,extraout_DX_00);
        //     if ((u_var3 + 0xc) == param_3) {
        //       param_2 += -0x1;
        //       pass1_1028_e332(ctx.PTR_LOOP_1050_65e2,u_var2,extraout_DX_00,unaff_SS);
        //       ppcVar1 = (*iVar6.field_0x1e + 0x8);
        //       (**ppcVar1)(&USHORT_1050_1028,iVar6.field_0x1e,0x0,uStack10);
        //     }
        //     if ((param_2._2_2_ | param_2) == 0x0) {
        //       return;
        //     }
        //   }
        // }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_6f5a(param_1: u32, param_2: u16) {
    let u_var1: u16;
    let BVar2: bool;

    u_var1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x4);
    if (BVar2 != 0x0) {
        pass1_1028_6302((param_1 + 0x1a), param_2);
    }
    return;
}


pub fn pass1_1030_6fa0(param_1: u32) -> u16

{
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var3 << 0x10);
    }
    if ((i_var2 + 0x1a) != 0x0) {
        u_var1 = (i_var2 + 0x1a);
        return (u_var1 + 0xc);
    }
    return 0x0;
}


pub fn pass1_1030_6fd4(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    u_var1 = (param_1 + 0x1a);
    if ((u_var1 + 0x12) == 0x5) {
        return;
    }
    return;
}


pub fn pass1_1030_701c(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    u_var1 = (param_1 + 0x1a);
    if ((u_var1 + 0x12) == 0x5) {
        return;
    }
    return;
}


pub fn pass1_1030_7064(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    u_var1 = (param_1 + 0x1a);
    if ((u_var1 + 0x12) == 0x5) {
        return;
    }
    return;
}


pub fn pass1_1030_70ac(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    u_var1 = (param_1 + 0x1a);
    if ((u_var1 + 0x12) == 0x5) {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_70f4(param_1: u32) {
    let u_var1: u16;
    let u_var2: u32;
    let BVar3: bool;
    let i_var4: i16;
    let u_var5: u16;
    plVar6: &i32;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var5 << 0x10);
    }
    u_var2 = (i_var4 + 0x1a);
    u_var1 = (u_var2 + 0xc);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1);
    if (BVar3 == 0x0) {
        BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x2);
        if ((BVar3 == 0x0) || ((i_var4 + 0x22) == 0x0)) {
            return;
        }
        plVar6 = (i_var4 + 0x22);
    } else {
        u_var2 = (i_var4 + 0x1a);
        plVar6 = (u_var2 + 0x28);
    }
    pass1_1020_ba94(plVar6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_7176(param_1: u32, param_2: u16) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let local_1a: i32;
    i16
    local_16[0x2];
    let uStack18: u16;
    let uStack14: u16;
    let BStack10: bool;
    let uStack8: u16;
    let lStack6: i32;

    lStack6 = 0x0;
    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x22) == 0x0) {
        return;
    }
    if ((i_var2 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1);
    }
    u_var1 = (i_var2 + 0x1a);
    uStack8 = (u_var1 + 0xc);
    BStack10 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x3);
    if ((BStack10 != 0x0) && (u_var1 = (i_var2 + 0x1a), (u_var1 + 0x12) == 0x5)) {
        u_var1 = (i_var2 + 0x22);
        uStack14 = (u_var1 + 0x4);
        // for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
        //   pass1_1020_bb16(*(u32 **)(i_var2 + 0x22),CONCAT22(param_2,&local_1a),
        //                   CONCAT22(param_2,local_16),uStack18);
        //   if (0x0 < local_16[0]) {
        //     lStack6 += local_1a;
        //   }
        // }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_7226(param_1: u32) {
    let u_var1: u32;
    let u_var2: u32;
    let BVar3: bool;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var5 << 0x10);
    }
    u_var2 = (i_var4 + 0x1a);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var2 + 0xc), 0x10);
    if (((BVar3 != 0x0) && (u_var2 = (i_var4 + 0x1a), (u_var2 + 0x12) == 0x5)) && (u_var1 = (i_var4 + 0x1a),
                                                                                u_var2 = (u_var1 & 0xffff0000 | (u_var1 + 0x14)),
                                                                                (u_var2 + 0xa4) == 0x1e)) {
        return;
    }
    return;
}


pub fn fn_ptr_1030_7296(param_1: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct292;
    let u_var3: u16;
    let paStack6: &mut Struct18;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = i_var3.field_0x22;
    u_var2 = i_var3.field_0x24;
    paStack6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        fn_ptr_1020_ba7e(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack6, 0x1000);
    }
    &i_var3.field_0x22 = 0x0;
    return;
}


pub fn pass1_1030_72d0(param_1: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct605;
    let u_var3: u16;
    let paStack6: &mut Struct18;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = i_var3.field_0x26;
    u_var2 = i_var3.field_0x28;
    paStack6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        fn_ptr_1020_ba7e(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack6, 0x1000);
    }
    &i_var3.field_0x26 = 0x0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_730a(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let iVar5: &mut Struct290;
    let u_var5: u16;
    let puVar6: u32;
    let uStack10: u32;
    let uStack6: u32;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (iVar5.field_0x1e != 0x0) {
        puVar6 = iVar5.field_0x1e;
        ppc_var3 = (*iVar5.field_0x1e + 0x10);
        (**ppc_var3)();
        uStack6 = CONCAT22(extraout_dx, param_2);
        // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
        //   ppc_var3 = (*iVar5.field_0x1e + 0x4);
        //   u_var4 = uStack6;
        //   (**ppc_var3)(param_3);
        //   if ((extraout_DX_00 | u_var4) != 0x0) {
        //     param_3 = &USHORT_1050_1028;
        //     pass1_1028_e332(ctx.PTR_LOOP_1050_65e2,u_var4,extraout_DX_00,param_4);
        //   }
        // }
        // WARNING: Load size is inaccurate
        pu_var1 = iVar5.field_0x1e;
        u_var2 = (&iVar5.field_0x1e + 0x2);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)(param_3, pu_var1, u_var2, 0x1, puVar6);
        }
        iVar5.field_0x1e = 0x0;
    }
    return;
}


pub fn pass1_1030_73ee(param_1: u32, param_2: u32, param_3: u16) {
    let i_var1: &mut Struct294;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x2a = param_2;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_2);
    i_var1.field_0x2e = param_2;
    i_var1.field_0x30 = param_3;
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_7418(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let i_var2: &mut Struct731;
    let i_var3: i16;
    let Bvar4: bool;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uStack62: u16;
    let local_2a: [u16; 0x2];
    u8
    local_26[0xe];
    let local_18: u32;
    let local_14: [u32; 0x2];
    let local_c: u16;
    let local_a: u32;
    let local_6: [u16; 0x2];

    pass1_1030_16d6(param_1, param_2, param_4);
    if (param_3 == 0x0) {
        return;
    }
    i_var2 = param_1;
    i_var2 = &i_var2.field_0xc;
    i_var3 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | ZEXT24(i_var2), 0x1008, param_4);
    if (i_var3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    // u_var6 = (param_1 >> 0x10);
    local_c = i_var2.field_0x12;
    uVar7 = param_2;
    // uVar8 = (param_2 >> 0x10);
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    local_6[0] = i_var2.field_0x14;
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    local_18 = i_var2.field_0x16;
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_18, param_4, 0x4, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7954(param_2, i_var2.field_0x1e, BVar4, 0x1008, param_4);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_2, i_var2.field_0x22, 0x1008, param_4);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_2, i_var2.field_0x26, 0x1008, param_4);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    local_a = i_var2.field_0x2a;
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_a, param_4, 0x4, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    local_c = i_var2.field_0x32;
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    local_c = i_var2.field_0x34;
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_79f0(param_2, i_var2.field_0x36, 0x1008, param_4);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    if (i_var2.field_0x3a == 0x0) {
        local_18 &= 0xffff0000;
    } else {
        u_var1 = i_var2.field_0x3a;
        local_18 = local_18 & 0xffff0000 | (u_var1 + 0x8);
    }
    local_6[0] = local_18;
    BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
    if (BVar4 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_5784(CONCAT22(param_4, local_26), i_var2.field_0x3a);
    loop {
        pu_var5 = local_26;
        pass1_1008_5b12(pu_var5, param_4);
        local_14[0] = CONCAT22(extraout_dx, pu_var5);
        if ((extraout_dx | pu_var5) == 0x0) {
            if (i_var2.field_0x3e == 0x0) {
                uStack62 = 0x0;
            } else {
                u_var1 = i_var2.field_0x3e;
                uStack62 = (u_var1 + 0x8);
            }
            local_2a[0] = uStack62;
            BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_2a, param_4, 0x2, 0x1008);
            if (BVar4 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return;
            }
            pass1_1008_5784(CONCAT22(param_4, local_26), i_var2.field_0x3e);
            loop {
                pu_var5 = local_26;
                pass1_1008_5b12(pu_var5, param_4);
                if ((extraout_DX_00 | pu_var5) == 0x0) {
                    return;
                }
                local_18 = local_18 & 0xffff0000 | (pu_var5 + 0x4);
                BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_18, param_4, 0x2, 0x1008);
                if (BVar4 == 0x0) {
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
                local_14[0] = local_14[0] & 0xffff0000 | (pu_var5 + 0x6);
                BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_14, param_4, 0x2, 0x1008);
                if (BVar4 == 0x0) {
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
                local_c = (pu_var5 + 0x8);
                BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, 0x1008);
                if (BVar4 == 0x0) { break; }
                local_c = (pu_var5 + 0xa);
                BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, &local_c, param_4, 0x2, 0x1008);
                if (BVar4 == 0x0) {
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
                local_6[0] = (pu_var5 + 0xc);
                BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
                if (BVar4 == 0x0) {
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
            }
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (pu_var5 + 0x4);
        BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
        if (BVar4 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0x6);
        BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
        if (BVar4 == 0x0) { break; }
        local_6[0] = (local_14[0] + 0x8);
        BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
        if (BVar4 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0xa);
        BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
        if (BVar4 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0xc);
        BVar4 = write_to_file_1008_7e1c(uVar7, uVar8, local_6, param_4, 0x2, 0x1008);
        if (BVar4 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}


pub fn pass1_1030_7bee(param_1: u32) -> u16

{
    let ppcVar1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x16) == 0x0) {
        return 0x0;
    }
    if ((i_var3 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var4 << 0x10);
    }
    ppcVar1 = ((i_var3 + 0x1a) + 0x44);
    u_var2 = (**ppcVar1)();
    return u_var2;
}


pub fn pass1_1030_7c28(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16) -> u32

{
    let u_var1: u16;
    let u_var2: u32;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x22) == 0x0) {
        return 0x0;
    }
    u_var2 = (param_1 + 0x22);
    u_var2 = pass1_1020_bae6(u_var2, CONCAT22(param_2, (u_var2 >> 0x10)),
                            param_3, param_4, param_5);
    return u_var2;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_7c50(param_1: u32, param_2: i32, param_3: i16, param_4: u16, param_5: U32Ptr) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let uVar7: u16;
    let extraout_DX_01: U32Ptr;
    let i_var8: &mut Struct305;
    let uVar8: u16;
    let uVar9: u32;
    let puVar10: u32;
    let puStack18: u32;

    // uVar8 = (param_1 >> 0x10);
    i_var8 = param_1;
    puVar6 = param_5;
    if (i_var8.field_0x1e == 0x0) {
        mem_op_1000_179c(0x18, param_5, 0x1000);
        puVar6 = (param_5 | param_4);
        if (puVar6 == 0x0) {
            i_var8.field_0x1e = 0x0;
        } else {
            struct_op_1030_1cd8(CONCAT22(param_5, param_4), 0x5, 0x5);
            &i_var8.field_0x1e = param_4;
            (&i_var8.field_0x1e + 0x2) = extraout_dx;
            puVar6 = extraout_dx;
        }
    }
    if (param_3 == 0x4) {
        pi_var1 = &i_var8.field_0x34;
        *pi_var1 = *pi_var1 + param_2;
    }
    while (param_2 != 0x0) {
        uVar9 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2, puVar6, 0x6);
        u_var3 = uVar9;
        u_var4 = uVar9 >> 0x10;
        puVar10 = i_var8.field_0x1e;
        ppcVar2 = (*i_var8.field_0x1e + 0xc);
        u_var5 = u_var3;
        (**ppcVar2)();
        uVar7 = extraout_DX_00;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
        puStack18 = CONCAT22(uVar7, u_var5);
        ppcVar2 = (*puStack18 + 0x14);
        (**ppcVar2)(&USHORT_1050_1028, u_var5, uVar7, param_1, puVar10, uVar9);
        puVar6 = extraout_DX_01;
        param_2 = param_2 + -0x1;
    }
    return;
}


pub fn pass1_1030_7d1c(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: U32Ptr,
                       param_6: u16, param_7: u16, param_8: u16)

{
    let u_var1: u16;
    let i_var2: &mut Struct397;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2.field_0x22 == 0x0) {
        mem_op_1000_179c(0xa, param_5, 0x1000);
        u_var1 = param_5 | param_4;
        if (u_var1 == 0x0) {
            i_var2.field_0x22 = 0x0;
        } else {
            pass1_1020_ba3e(CONCAT22(param_5, param_4), 0xa, 0x2, param_7, param_6);
            &i_var2.field_0x22 = param_4;
            (&i_var2.field_0x22 + 0x2) = u_var1;
        }
    }
    pass1_1020_bb8a(i_var2.field_0x22, param_2, param_3, param_7, param_8);
    return;
}


pub fn pass1_1030_7d7c(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: U32Ptr,
                       param_6: u16, param_7: u16, param_8: u16)

{
    let u_var1: u16;
    let i_var2: &mut Struct398;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2.field_0x26 == 0x0) {
        mem_op_1000_179c(0xa, param_5, 0x1000);
        u_var1 = param_5 | param_4;
        if (u_var1 == 0x0) {
            i_var2.field_0x26 = 0x0;
        } else {
            pass1_1020_ba3e(CONCAT22(param_5, param_4), 0xa, 0x2, param_7, param_6);
            &i_var2.field_0x26 = param_4;
            (&i_var2.field_0x26 + 0x2) = u_var1;
        }
    }
    pass1_1020_bb8a(i_var2.field_0x26, param_2, param_3, param_7, param_8);
    return;
}


pub fn pass1_1030_7ddc(param_1: u32, param_2: i32, param_3: u16, param_4: u16, param_5: U32Ptr,
                       param_6: u16, param_7: u16, param_8: u16)

{
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let u_var4: u16;
    let lVar5: i32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pu_var2 = param_5;
    if ((i_var3 + 0x22) == 0x0) {
        mem_op_1000_179c(0xa, param_5, 0x1000);
        pu_var2 = (param_5 | param_4);
        if (pu_var2 == 0x0) {
            (i_var3 + 0x22) = 0x0;
        } else {
            pass1_1020_ba3e(CONCAT22(param_5, param_4), 0xa, 0x2, param_7, param_6);
            (i_var3 + 0x22) = param_4;
            (i_var3 + 0x24) = pu_var2;
        }
    }
    u_var1 = (i_var3 + 0x22);
    lVar5 = pass1_1020_bae6(u_var1, CONCAT22(param_3, (u_var1 >> 0x10)),
                            param_4, pu_var2, param_8);
    pass1_1020_bb8a((i_var3 + 0x22), (lVar5 + param_2),
                    CONCAT22(param_3, ((lVar5 + param_2) >> 0x10)), param_7,
                    param_8);
    return;
}


pub fn pass1_1030_7e5a(param_1: u32, param_2: u32, param_3: u16) {
    let i_var1: &mut Struct358;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x16 = param_2;
    i_var1.field_0x1a = 0x0;
    pass1_1030_6fa0(param_1 & 0xffff | u_var1 << 0x10);
    if (i_var1.field_0x2e != 0x0) {
        pass1_1038_4b20(i_var1.field_0x2e, i_var1.field_0x16, i_var1.field_0x4, param_3);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_7ea0(param_1: u32) -> bool

{
    let u_var1: u32;
    let u_var2: u16;
    let BVar3: bool;

    u_var2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0xb);
    if (BVar3 != 0x0) {
        u_var1 = (param_1 + 0x1a);
        if ((u_var1 + 0x12) == 0x5) {
            return 0x1;
        }
        BVar3 = 0x0;
    }
    return BVar3;
}


pub fn pass1_1030_7eda(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let local_c: u16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let uStack4: u16;

    local_c = 0x0;
    uStack10 = 0x0;
    uStack6 = 0x0;
    uStack4 = 0x0;
    uStack8 = param_2;
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var1 << 0x10);
    }
    pass1_1028_bb96((param_1 + 0x1a), &local_c, param_3);
    return;
}


pub fn pass1_1030_7f1a(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let local_c: u16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let uStack4: u16;

    local_c = 0x0;
    uStack8 = 0x0;
    uStack6 = 0x0;
    uStack4 = 0x0;
    uStack10 = param_2;
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var1 << 0x10);
    }
    pass1_1028_bb96((param_1 + 0x1a), &local_c, param_3);
    return;
}


pub fn pass1_1030_7f5a(param_1: u32) -> u16

{
    let u_var1: u16;
    let u_var2: u32;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var1 << 0x10);
    }
    u_var2 = pass1_1028_bb6a((param_1 + 0x1a));
    if (u_var2 != 0x0) {
        return (u_var2 + 0x4);
    }
    return 0x0;
}


pub fn pass1_1030_7f98(param_1: u32) -> u16

{
    let u_var1: u16;
    let u_var2: u32;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var1 << 0x10);
    }
    u_var2 = pass1_1028_bb6a((param_1 + 0x1a));
    if (u_var2 != 0x0) {
        return (u_var2 + 0x2);
    }
    return 0x0;
}


pub fn pass1_1030_7fd6(param_1: u32, param_2: u16, param_3: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x1a) == 0x0) {
        u_var5 = struct_op_1030_73a8(param_1 & 0xffff | u_var4 << 0x10);
        // param_2 = (u_var5 >> 0x10);
    }
    u_var2 = (i_var3 + 0x1a);
    i_var1 = (u_var2 + 0xc);
    if (((0x32 < i_var1) && (!SBORROW2(i_var1, 0x33))) && ((i_var1 == 0x34 || i_var1 + -0x33 < 0x1 || ((0x2b < i_var1 + -0x34 && (i_var1 + -0x60 < 0x2)))))) {
        pass1_1028_1416((i_var3 + 0x1a), param_2, param_3);
    }
    return;
}


pub fn pass1_1030_8030(param_1: u32, param_2: u16, param_3: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x1a) == 0x0) {
        u_var5 = struct_op_1030_73a8(param_1 & 0xffff | u_var4 << 0x10);
        // param_2 = (u_var5 >> 0x10);
    }
    u_var2 = (i_var3 + 0x1a);
    i_var1 = (u_var2 + 0xc);
    if (((0x32 < i_var1) && (!SBORROW2(i_var1, 0x33))) && ((i_var1 == 0x34 || i_var1 + -0x33 < 0x1 || ((0x2b < i_var1 + -0x34 && (i_var1 + -0x60 < 0x2)))))) {
        u_var5 = (i_var3 + 0x1a);
        pass1_1028_1106(u_var5, u_var5, param_2, param_3);
    }
    return;
}


pub fn pass1_1030_8086(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18),
                    (param_1 + 0x16)) & 0xffffff;
}


pub fn pass1_1030_809c(param_1: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0x0) {
        struct_op_1030_73a8(param_1 & 0xffff | u_var1 << 0x10);
    }
    return;
}


pub fn pass1_1030_80ee(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1030_68dc(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        pass1_1000_093a(param_1, (param_1 >> 0x10), 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn struct_1030_8128(param_1: &mut Struct18, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let extraout_dx: U32Ptr;
    let i_var4: &mut Struct135;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = 0x0;
    *param_1 = 0x0;
    &i_var4.field_0x4 = 0x0;
    i_var4.field_0x8 = 0x0;
    ctx._PTR_LOOP_1050_5748 = param_1;
    mem_op_1000_179c(0x56, param_2, 0x1000);
    pu_var2 = (param_2 | u_var1);
    if (pu_var2 != 0x0) {
        pass1_1028_d81c(CONCAT22(param_2, u_var1), param_1, pu_var2, param_3);
    }
    mem_op_1000_179c(0x8, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | u_var1);
    if (pu_var3 == 0x0) {
        *param_1 = 0x0;
    } else {
        struct_1028_d22e(CONCAT22(pu_var2, u_var1), param_1, pu_var3);
        param_1 = u_var1;
        i_var4.field_0x2 = pu_var3;
    }
    mem_op_1000_179c(0x8, pu_var3, 0x1000);
    pu_var2 = (pu_var3 | u_var1);
    if (pu_var2 == 0x0) {
        &i_var4.field_0x4 = 0x0;
    } else {
        pass1_1028_cfd2(CONCAT22(pu_var3, u_var1), param_1);
        i_var4.field_0x4 = u_var1;
        i_var4.field_0x6 = extraout_dx;
        pu_var2 = extraout_dx;
    }
    mem_op_1000_179c(0x24, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | u_var1);
    if (pu_var3 != 0x0) {
        pass1_1030_5bec(CONCAT22(pu_var2, u_var1));
    }
    mem_op_1000_179c(0x8, pu_var3, 0x1000);
    if ((pu_var3 | u_var1) != 0x0) {
        pass1_1038_78e2(CONCAT22(pu_var3, u_var1), (pu_var3 | u_var1));
    }
    ctx.PTR_LOOP_1050_574a = (ctx.PTR_LOOP_1050_5748 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8210(param_1: U32Ptr) {
    let u_var1: u16;
    let u_var2: u16;
    let paVar3: &mut Struct18;
    let i_var4: i16;
    let u_var5: u16;
    let paStack10: &mut Struct18;
    let paStack6: &mut Struct18;

    paVar3 = ctx._PTR_LOOP_1050_65e2;
    if (ctx.PTR_LOOP_1050_65e2 != 0x0) {
        pass1_1028_daba(ctx.PTR_LOOP_1050_65e2, &USHORT_1050_1028);
        fn_ptr_1000_17ce(ctx, paVar3, 0x1000);
    }
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = *param_1;
    u_var2 = (i_var4 + 0x2);
    paStack10 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1028_d282(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
    }
    u_var1 = (i_var4 + 0x4);
    u_var2 = (i_var4 + 0x6);
    paStack6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1028_cff2(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack6, 0x1000);
    }
    paVar3 = ctx._PTR_LOOP_1050_5736;
    if (ctx.PTR_LOOP_1050_5736 != 0x0) {
        pass1_1030_5c0e();
        fn_ptr_1000_17ce(ctx, paVar3, 0x1000);
    }
    paVar3 = ctx._PTR_LOOP_1050_5a64;
    if ((ctx.PTR_LOOP_1050_5a66 | ctx._PTR_LOOP_1050_5a64) != 0x0) {
        pass1_1038_7964((ctx.PTR_LOOP_1050_5a64 & 0xffff | ZEXT24(ctx.PTR_LOOP_1050_5a66) << 0x10));
        fn_ptr_1000_17ce(ctx, paVar3, 0x1000);
    }
    ctx._PTR_LOOP_1050_5748 = 0x0;
    return;
}


pub fn pass1_1030_82f0(param_1: u16, param_2: u32, param_3: u32) {
    pass1_1028_d078(param_1, (param_2 + 0x4), param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8308(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: U32Ptr,
                       param_5: u32, param_6: u16, param_7: u16)

{
    pass1_1028_e198(ctx.PTR_LOOP_1050_65e2, param_3, param_4, param_5, param_6, param_7);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8326(ctx: &mut AppContext) -> u32

{
    return CONCAT22((ctx.PTR_LOOP_1050_65e2 + 0x2), ctx._PTR_LOOP_1050_65e2 as u16);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8334(ctx: &mut AppContext, ptr_1: U32Ptr) -> bool {
    ctx._PTR_LOOP_1050_65e2 = 0x0;
    true
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8344(ctx: &mut AppContext, param_1: u32, param_3: u32) {
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3);
    return;
}


pub fn pass1_1030_8372(param_1: U32Ptr, param_2: u32, param_3: U32Ptr) {
    pass1_1028_d52c(*param_1, param_2, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_838e(param_1: U32Ptr, param_2: u16, param_3: u8) {
    struct_1028_d2b0(*param_1, param_2, param_3);
    pass1_1028_d01a((param_1 + 0x4));
    send_msg_1028_e242(ctx.PTR_LOOP_1050_65e2, 0x1, &USHORT_1050_1028);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


pub fn pass1_1030_8480(astruct_18 * * param_1) {
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    return;
}


pub fn pass1_1030_8496(param_1: u32) {
    fn_ptr_1000_17ce(ctx, (param_1 + 0x2), 0x1000);
    return;
}


pub fn pass1_1030_84ae(param_1: u32) {
    clear_struct_1008_3e38((param_1 & 0xffff0000 | (param_1 + 0x8)));
    (param_1 + 0x1e) = 0x1;
    return;
}


pub fn fn_ptr_1030_84d0(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x1e) != 0x0) {
        pu_var1 = (i_var4 + 0xe);
        u_var2 = (i_var4 + 0x10);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        pu_var1 = (i_var4 + 0x12);
        u_var2 = (i_var4 + 0x14);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        fn_ptr_1000_17ce(ctx, (i_var4 + 0x4), 0x1000);
        fn_ptr_1000_17ce(ctx, (i_var4 + 0x16), 0x1000);
    }
    return;
}


pub fn pass1_1030_85be(param_1: &i32, param_2: u16, param_3: i16, param_4: i16, param_5: u16) {
    let i_var1: &mut Struct357;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x0;
    i_var1.field_0x4 = 0x0;
    i_var1.field_0x6 = param_3;
    i_var1.field_0x8 = param_2;
    i_var1.field_0xe = 0x0;
    if (i_var1.field_0x6 == 0x0) {
        i_var1.field_0x6 = 0x5;
    }
    pass1_1030_878c(param_1, param_4, param_5);
    return;
}


pub fn pass1_1030_8604(astruct_18 * * param_1) {
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    return;
}


pub fn pass1_1030_861a(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let puStack6: u32;

    pass1_1030_8854(param_1, param_2, param_3, param_6);
    puStack6 = CONCAT22(param_5, param_4);
    if ((param_5 | param_4) == 0x0) {
        (param_1 + 0xa) = 0x0;
    } else {
        (param_1 + 0xa) = *puStack6;
    }
    return;
}


pub fn pass1_1030_8660(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u16, param_7: i16)

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let puStack6: u32;

    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    pass1_1030_8854(u_var2, u_var3, param_3, param_6);
    puStack6 = CONCAT22(param_5, param_4);
    u_var1 = param_5 | param_4;
    if (u_var1 == 0x0) {
        pass1_1030_8854(u_var2, u_var3, 0x0, param_6);
        puStack6 = CONCAT22(u_var1, param_4);
        u_var1 |= param_4;
        if (u_var1 == 0x0) {
            pass1_1030_878c(param_1, param_7, param_6);
            pass1_1030_8854(u_var2, u_var3, 0x0, param_6);
            puStack6 = CONCAT22(u_var1, param_4);
            if ((u_var1 | param_4) == 0x0) {
                return;
            }
        }
        (puStack6 + 0x4) = param_3;
        *puStack6 = *param_2;
        pass1_1030_8834(param_1, param_7, param_6);
    } else {
        *puStack6 = *param_2;
    }
    return;
}


pub fn pass1_1030_86ec(astruct_18 * * param_1, param_2: u16) {
    let i_var1: &mut Struct612;
    let u_var1: u16;

    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x0;
    i_var1.field_0x4 = 0x0;
    i_var1.field_0x6 = param_2;
    i_var1.field_0xe = 0x0;
    return;
}


pub fn pass1_1030_871e(param_1: &i32, param_2: U32Ptr, param_3: u16, param_4: i16, param_5: u16) {
    let pi_var1: U32Ptr;
    let i_var2: &mut Struct681;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (*param_1 == 0x0) {
        pass1_1030_878c((param_1 & 0xffff | u_var2 << 0x10), param_4,
                        param_5);
    }
    pi_var1 = &i_var2.field_0xe;
    *pi_var1 = *pi_var1 + 0x1;
    (*param_1 + i_var2.field_0xe * 0x6 + 0x4) = param_3;
    (i_var2.field_0xe * 0x6 + *param_1) = *param_2;
    return;
}


pub fn pass1_1030_877c(param_1: U32Ptr, param_2: i16, param_3: u16) {
    pass1_1030_8834(param_1, param_2, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_878c(param_1: &i32, param_2: i16, param_3: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: &mut Struct350;
    let u_var4: u16;
    let lVar5: i32;
    let lStack12: i32;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (i_var4.field_0x4 == 0x0) {
        ctx.PTR_LOOP_1050_5f2e = 0x0;
        u_var2 = i_var4.field_0x6;
    } else {
        u_var3 = i_var4.field_0x4;
        pu_var1 = &i_var4.field_0x8;
        u_var2 = u_var3 + *pu_var1;
        ctx.PTR_LOOP_1050_5f2e = CARRY2(u_var3, *pu_var1);
    }
    if ((false) || (ctx.PTR_LOOP_1050_5f2e == 0x0)) {
        if (*param_1 == 0x0) {
            if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
                ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
            } else {}
            u_var3 = fn_ptr_op_1000_1708(u_var2 * 0x6, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                        ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {
            lVar5 = pass1_1000_0ed4(
                ctx, 0x1000, param_3, 0x1, u_var2 * 0x6, 0x0, *param_1,
                (*param_1 >> 0x10));
            ctx.PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
            u_var3 = lVar5;
        }
        lStack12 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var3);
        if ((ctx.PTR_LOOP_1050_5f2e | u_var3) != 0x0) {
            i_var4.field_0x4 = u_var2;
            *param_1 = lStack12;
            pass1_1030_8834((param_1 & 0xffff | u_var4 << 0x10), param_2,
                            param_3);
        }
    }
    return;
}


pub fn pass1_1030_8834(param_1: U32Ptr, param_2: i16, param_3: u16) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x2);
    pass1_1000_4aea(*param_1, u_var1, (u_var1 >> 0x10), 0x6, 0x888e,
                    &stack0xfffe, param_2, u_var2, 0x1000, param_3);
    return;
}


pub fn pass1_1030_8854(param_1: u16, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let local_c: u32;
    let uStack8: u16;

    uStack8 = param_3;
    local_c = 0x0;
    u_var1 = (param_1 + 0x2);
    pass1_1000_49c6(&local_c, param_4, *_param_1, u_var1,
                    (u_var1 >> 0x10), 0x6, 0x888e, &stack0xfffe);
    return;
}


pub fn pass1_1030_888e(param_1: u32, param_2: u32) -> u16

{
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = (param_1 + 0x4);
    // u_var4 = (param_2 >> 0x10);
    pi_var1 = (param_2 + 0x4);
    if (*pi_var1 != i_var2 && i_var2 <= *pi_var1) {
        return 0xffff;
    }
    if ((param_2 + 0x4) < (param_1 + 0x4)) {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1030_88ce(param_1: U32Ptr, param_2: u32, param_3: u32, param_4: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let i_var4: &mut Struct354;
    let u_var3: u16;
    let u_var4: u32;
    let puStack38: U32Ptr;
    let iStack34: i16;
    let local_20: [u8; 2];
    let local_1e: i16;
    let local_1c: i16;
    let local_1a: [u8; 6];
    let local_14: [u8; 6];
    let uStack14: u32;
    let uStack10: u32;
    let i_stack6: i16;
    let uStack4: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x389a;
    i_var4.field_0x2 = 0x1008;
    pass1_1030_84ae(param_1 & 0xffff0000 | &i_var4.field_0x4);
    i_var4.field_0x24 = param_3;
    puStack38 = (param_1 & 0xffff0000 | &i_var4.field_0x28);
    pass1_1008_6c90(
        (param_1 & 0xffff0000 | &i_var4.field_0x28));
    &i_var4.field_0x34 = 0x0;
    *param_1 = 0x8e38;
    i_var4.field_0x2 = 0x1030;
    struct_1030_8544(
        (param_1 & 0xffff0000 | &i_var4.field_0x4),
        param_2);
    u_var4 = pass1_1008_4772(i_var4.field_0x12);
    // uStack4 = (u_var4 >> 0x10);
    i_stack6 = u_var4;
    uStack10 = (i_stack6 + 0x4);
    uStack14 = (i_stack6 + 0x8);
    pass1_1008_3e54(CONCAT22(param_4, local_14), 0x0, uStack14 - 0x1,
                    uStack10 - 0x1);
    pass1_1008_3e54(CONCAT22(param_4, local_1a), 0x0, 0x0, 0x0);
    pass1_1008_6d18(puStack38, CONCAT22(param_4, local_14),
                    CONCAT22(param_4, local_1a));
    pass1_1008_6d64(puStack38, CONCAT22(param_4, local_1a));
    pass1_1008_3eb4(CONCAT22(param_4, local_1a),
                    CONCAT22(param_4, local_20),
                    CONCAT22(param_4, &local_1e),
                    CONCAT22(param_4, &local_1c));
    pu_var1 = ((local_1e * local_1c) >> 0x10);
    u_var4 = local_1e * local_1c & 0xffff;
    i_var4.field_0x34 = u_var4;
    i_var4.field_0x36 = pu_var1;
    // for (iStack34 = 0x0; iStack34 < 0x5; iStack34 += 0x1) {
    //   mem_op_1000_179c(0x10,pu_var1,0x1000);
    //   pu_var2 = (pu_var1 | u_var4);
    //   if (pu_var2 == 0x0) {
    //     (&i_var4[0x1].field_0x0 + iStack34 * 0x4) = 0x0;
    //   }
    //   else {
    //     pass1_1030_85be((u_var4 & 0xffff | ZEXT24(pu_var1) << 0x10),0x19,0x64,u_var3,
    //                     param_4);
    //     (&i_var4[0x1].field_0x0 + iStack34 * 0x4) = u_var4;
    //     (&i_var4[0x1].field_0x2)[iStack34 * 0x2] = pu_var2;
    //   }
    //   pu_var1 = pu_var2;
    // }
    return;
}


pub fn pass1_1030_8a2c(param_1: U32Ptr) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var3: &mut Struct613;
    let u_var3: u16;
    let i_stack4: i16;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x8e38;
    i_var3.field_0x2 = 0x1030;
    i_stack4 = 0x0;
    loop {
        paVar2 = (&i_var3[0x1].field_0x0 + i_stack4 * 0x4);
        u_var1 = (&i_var3[0x1].field_0x2)[i_stack4 * 0x2];
        if ((u_var1 | paVar2) != 0x0) {
            pass1_1030_8604((astruct_18 * *)(paVar2 & 0xffff | u_var1 << 0x10));
            fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
        }
        i_stack4 += 0x1;
        if (i_stack4 < 0x5) == false { break; }
    }
    fn_ptr_1030_84d0(param_1 & 0xffff0000 | &i_var3.field_0x4);
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1030_8aa0(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let unaff_DI: i16;
    let local_12: u32;
    let puStack14: U32Ptr;
    let uStack12: u32;
    let local_8: [u8; 2];
    let local_6: [u8; 2];
    let local_4: [u8; 2];

    puStack14 = local_8;
    pass1_1008_3eb4(param_3,
                    CONCAT13((param_5 >> 0x8),
                             CONCAT12(param_5, puStack14)),
                    CONCAT22(param_5, local_6), CONCAT22(param_5, local_4));
    bad_1030_8cd2();
    uStack12 = CONCAT22(param_4, puStack14);
    u_var1 = param_4 | puStack14;
    if (u_var1 != 0x0) {
        pass1_1030_8d9e(param_1, param_5);
        local_12 = param_2;
        pass1_1030_8660(uStack12, CONCAT22(param_5, &local_12), puStack14,
                        &local_12, u_var1, param_5, unaff_DI);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_8b00(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: u16) {
    let pu_var1: u32;
    let piVar2: U32Ptr;
    let u_var3: u16;
    let local_2a: u32;
    let uStack38: u32;
    let uStack28: u32;
    let puStack18: u32;
    let puStack16: u32;
    let piStack14: U32Ptr;
    let local_c: i16;
    let local_a: [u8; 4];
    let uStack6: u32;

    uStack6 = 0x0;
    pu_var1 = (local_a + 0x2);
    piVar2 = &local_c;
    pass1_1008_3eb4(param_2,
                    CONCAT13((param_4 >> 0x8), CONCAT12(param_4, piVar2)),
                    CONCAT22(param_4, local_a), CONCAT22(param_4, pu_var1));
    bad_1030_8cd2();
    puStack16 = pu_var1;
    piStack14 = piVar2;
    pass1_1030_8d9e(param_1, param_4);
    puStack18 = pu_var1;
    pass1_1030_861a(puStack16, piStack14, pu_var1, pu_var1,
                    piVar2, param_4);
    uStack38 = *pu_var1;
    u_var3 = (pu_var1 + 0x2);
    uStack38._3_1_ = (uStack38 >> 0x18);
    uStack6 = uStack38;
    if (uStack38._3_1_ == '\0') {
        pu_var1 = &local_2a;
        uStack28 = uStack38;
        pass1_1030_8c66(param_1, local_c, local_a, (local_a >> 0x10),
                        CONCAT22(param_4, pu_var1), u_var3);
        uStack6 = *pu_var1;
        u_var3 = (pu_var1 + 0x2);
    }
    *param_3 = uStack6;
    (param_3 + 0x2) = u_var3;
    return;
}


pub fn pass1_1030_8bac(param_1: u32, param_2: u16) {
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        pass1_1030_86ec((param_1 + 0x38 + i_stack4 * 0x4), param_2);
        i_stack4 += 0x1;
        if (i_stack4 < 0x5) == false { break; }
    }
    return;
}


pub fn pass1_1030_8bdc(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: U32Ptr;
    let local_12: u32;
    let puStack14: U32Ptr;
    plStack12: &i32;
    let local_8: [u8; 2];
    let local_6: [u8; 2];
    let local_4: [u8; 2];

    puStack14 = local_4;
    pu_var1 = local_8;
    pass1_1008_3eb4(param_3,
                    CONCAT13((param_5 >> 0x8), CONCAT12(param_5, pu_var1)),
                    CONCAT22(param_5, local_6),
                    CONCAT22(param_5, puStack14));
    bad_1030_8cd2();
    plStack12 = CONCAT22(pu_var1, puStack14);
    pass1_1030_8d9e(param_1, param_5);
    local_12 = param_2;
    pass1_1030_871e(plStack12, CONCAT22(param_5, &local_12), puStack14, param_4, param_5);
    return;
}


pub fn pass1_1030_8c38(param_1: u32, param_2: i16, param_3: u16) {
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        pass1_1030_877c((param_1 + 0x38 + i_stack4 * 0x4), param_2, param_3);
        i_stack4 += 0x1;
        if (i_stack4 < 0x5) == false { break; }
    }
    return;
}


pub fn pass1_1030_8c66(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: u16, param_5: U32Ptr,
                       param_6: u16)

{
    let bVar1: u8;
    let u_var2: u16;
    let uStack6: u32;

    pass1_1008_4544((param_1 + 0x12));
    bVar1 = *param_3;
    u_var2 = bVar1;
    uStack6 = (u_var2 + 0x1);
    if (0x0 < param_2) {
        if (u_var2 == 0x0) {
            uStack6 = 0x7;
        } else {
            if (((bVar1 == 0x0) || (SBORROW2(u_var2, 0x1))) || (0x1 < (u_var2 - 0x1))) {
                uStack6 = 0x9;
            } else {
                uStack6 = 0x8;
            }
        }
    }
    *param_5 = uStack6;
    return;
}


pub fn pass1_1030_8d08(param_1: u32, param_2: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let uStack16: u32;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // u_var4 = (param_1 >> 0x10);
        pi_var1 = (param_1 + 0x1e);
        if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) { break; }
        u_var3 = i_stack4 * 0x6;
        u_var2 = (param_1 + 0x1a);
        (u_var2 + u_var3 + 0x4) = 0x0;
        pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x500);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
        uStack16 = CONCAT22(param_2, u_var3);
        u_var5 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2, param_2, 0x7);
        // param_2 = (u_var5 >> 0x10);
        pass1_1030_7e5a(uStack16, u_var5 & 0xffff | param_2 << 0x10, param_2);
        i_stack4 += 0x1;
    }
    return;
}


pub fn pass1_1030_8d9e(param_1: u32, param_2: u16) {
    let local_c: [u8; 2];
    let local_a: [u8; 2];
    let local_8: [u8; 6];

    clear_struct_1008_3e38(CONCAT22(param_2, local_8));
    pass1_1008_6d64((param_1 & 0xffff0000 | (param_1 + 0x28)),
                    CONCAT22(param_2, local_8));
    pass1_1008_3e94(CONCAT22(param_2, local_8), CONCAT22(param_2, local_c),
                    CONCAT22(param_2, local_a));
    return;
}



astruct_18 *  pass1_1030_8e12(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_8a2c(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8e3c(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: u32, param_5: u32) -> u32

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let unaff_DI: i16;
    let pu_var5: u32;
    let puVar6: U32Ptr;
    let uVar7: u16;

    mem_op_1000_179c(0xc, param_3, 0x1000);
    if ((param_3 | param_2) == 0x0) {
        pu_var5 = 0x0;
    } else {
        pu_var5 = set_struct_1008_574a(CONCAT22(param_3, param_2));
    }
    if (param_5._3_1_ == '\x04') {
        puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_1,
                                 (pu_var5 >> 0x10), unaff_DI);
        // u_var3 = (puVar6 >> 0x10);
        u_var1 = (puVar6 + 0x1e);
        u_var2 = u_var1;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5);
        // uVar7 = (param_4 >> 0x10);
        u_var4 = u_var3;
        if (u_var1 < 0x1) {
            pass1_1030_9296(param_4, pu_var5, CONCAT22(u_var3, u_var2), param_1, u_var2, u_var3);
            pass1_1030_951a(param_1, u_var4, param_4, pu_var5, CONCAT22(u_var3, u_var2));
        } else {
            pass1_1030_9adc(param_4, uVar7, pu_var5, CONCAT22(u_var3, u_var2), u_var2, u_var3);
            pass1_1030_9c1c(param_4, pu_var5, CONCAT22(u_var3, u_var2));
        }
        pass1_1030_9d42(param_1, u_var4, param_4, uVar7, pu_var5, CONCAT22(u_var3, u_var2));
    }
    return pu_var5;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_8f04(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u16;
    let iStack8: i16;
    let uStack6: u32;

    pass1_1038_53ba(param_3, 0x1);
    if ((((param_5 != 0x0) || (0x1 < param_4)) && ((pass1_1038_53ba(param_3, 0x2), param_5 != 0x0 || (0x1 < param_4)))) && ((pass1_1038_53ba(param_3, 0x3), param_5 != 0x0 || (0x1 < param_4)))) {
        pass1_1038_53ba(param_3, 0x4);
        u_var5 = param_5;
        if ((param_5 != 0x0) || (0x1 < param_4)) {
            empty_1038_540a();
            uStack6 = param_4 & 0xffff | u_var5 << 0x10;
            iStack8 = 0x0;
            loop {
                u_var3 = u_var5;
                u_var2 = param_4;
                if (0x0 < (iStack8 * 0x2 + ctx._PTR_LOOP_1050_580e)) {
                    empty_1038_540a();
                    u_var6 = (ctx.PTR_LOOP_1050_580e >> 0x10);
                    u_var1 = (iStack8 * 0x2 + ctx._PTR_LOOP_1050_580e);
                    param_4 = u_var1;
                    u_var4 = u_var1 >> 0xf;
                    u_var5 = u_var4;
                    if ((u_var3 <= u_var4) && ((u_var3 < u_var4 || (u_var2 < u_var1)))) {
                        if (0x1c < iStack8) {
                            return;
                        }
                        u_var2 = (iStack8 * 0x2 + ctx._PTR_LOOP_1050_580e);
                        param_4 = SEXT24(u_var2);
                        u_var5 = param_4 >> 0x10;
                        if (uStack6 < param_4) {
                            return;
                        }
                        uStack6 = CONCAT22(((uStack6 >> 0x10) - (u_var2 >> 0xf)) - (uStack6 < u_var2), uStack6 - u_var2);
                    }
                }
                iStack8 += 0x1;
                if (0x24 < iStack8) {
                    return;
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool
pass1_1030_8fe4(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
param_6: U32Ptr,param_7: i32)

{
let i_var1: i16; let u_var2: u16;
let u_var3: u32;

pass1_1030_627e(param_1, param_2, param_3, _PTR_LOOP_1050_5740, param_6, param_7);
u_var2 = param_3 | param_2; if (u_var2 != 0x0) {
pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_2, param_3); if ((u_var2 | param_2) != 0x0) {
u_var3 = struct_op_1030_73a8(CONCAT22(u_var2, param_2)); if ((u_var3 != 0x0) && ((i_var1 = (u_var3 + 0xc), i_var1 == 0x5 | | (i_var1 == 0x9)))) {
return 0x1;
}
}
}
return 0x0;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9048(param_1: u16, param_2: u32, param_3: i16, param_4: u32) {
    let i_var1: i16;
    let u_var2: u32;
    let ppc_var3: u32;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let uVar8: u16;
    let uVar9: u16;
    let puVar10: u32;
    let u_var11: u16;
    let uVar12: u16;
    let puVar13: u32;
    let uVar14: u32;
    let uVar15: u16;
    let uVar16: u8;
    let uStack36: u32;
    let local_18: [u8; 2];
    let local_16: i16;
    let local_14: i16;
    let local_12: i16;
    let iStack16: i16;
    let uStack12: u32;
    let uStack8: u16;
    let puStack6: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x8 - (param_3 == 0x0);
    puVar13 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, i_stack4);
    // puVar7 = (puVar13 >> 0x10);
    uVar8 = puVar13;
    uStack8 = uVar8;
    puStack6 = puVar7;
    pass1_1038_4e78(uVar8, puVar7, param_4, puVar13);
    uStack12 = CONCAT22(puVar7, uVar8);
    uVar12 = 0x1008;
    clear_struct_1008_3e38(CONCAT22(param_1, &local_12));
    u_var2 = (param_4 + 0x8);
    // u_var11 = (uStack12 >> 0x10);
    uVar9 = SUB42(uStack12, 0x0);
    ppc_var3 = (*uStack12 + 0x10);
    u_var6 = u_var2;
    (**ppc_var3)(0x1008, uVar9, u_var11);
    u_var6 = u_var6 & 0xffff | extraout_dx << 0x10;
    uStack36 = 0x0;
    loop {
        if (u_var6 <= uStack36) {
            if (uStack12 != 0x0) {
                ppc_var3 = *uStack12;
                (**ppc_var3)(uVar12, uStack12, (uStack12 >> 0x10), 0x1, uVar9, u_var11,
                            uStack12, uStack12);
            }
            return;
        }
        ppc_var3 = (*uStack12 + 0x4);
        uVar14 = u_var6;
        (**ppc_var3)();
        u_var5 = uVar14;
        uVar8 = extraout_DX_00;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
        pass1_1008_3f62(CONCAT22(param_1, &local_12),
                        CONCAT22(uVar8, u_var5 + 0xc));
        uVar12 = 0x1008;
        pass1_1008_3eb4(CONCAT22(param_1, &local_12),
                        CONCAT22(param_1, local_18),
                        CONCAT22(param_1, &local_16),
                        CONCAT22(param_1, &local_14));
        uVar14 = struct_op_1030_73a8(CONCAT22(uVar8, u_var5));
        // uVar8 = (uVar14 >> 0x10);
        i_var1 = (uVar14 + 0xc);
        if (i_var1 - 0x7a < 0x6) { break; }
//LAB_1030_91fa:
        uStack36 += 0x1;
    }
    uVar12 = 0x1030;
    u_var5 = param_2;
    // uVar15 = (param_2 >> 0x10);
    switch(i_var1)
    {
        default: iStack16 = local_16 + -0x1;
        BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, u_var5, uVar15,
                                CONCAT22(param_1, &local_12), u_var2);
        if (BVar4 != 0x0) {
            // goto
            // LAB_1030_91cb;
        }
        iStack16 = local_16 + 0x1;
        BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, u_var5, uVar15,
                                CONCAT22(param_1, &local_12), u_var2);
        if (BVar4 == 0x0) {
            iStack16 = local_16;
            local_12 = local_14 + -0x1;
            BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, u_var5, uVar15,
                                    CONCAT22(param_1, &local_12), u_var2);
//       TODO: goto joined_r0x1030911e;
        }
//LAB_1030_9144:
        break;
        0x7b => 0x7e => iStack16 = local_16 + -0x1;
        BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, u_var5, uVar15,
                                CONCAT22(param_1, &local_12), u_var2);
        if (BVar4 == 0x0) {
            iStack16 = local_16 + 0x1;
//       TODO: goto LAB_1030_912c;
        }
        if (uStack12 == 0x0) {
            return;
        }
        // uVar12 = (uStack12 >> 0x10);
        puVar10 = uStack12;
        // uVar16 = (uStack12 >> 0x10);
//     TODO: goto LAB_1030_90e6;
        0x7c => 0x7d => local_12 = local_14 + -0x1;
        BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, u_var5, uVar15,
                                CONCAT22(param_1, &local_12), u_var2);
        joined_r0x1030911e: if (BVar4 == 0x0) {
        local_12 = local_14 + 0x1;
//LAB_1030_912c:
        BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, u_var5, uVar15,
                                CONCAT22(param_1, &local_12), u_var2);
        if (BVar4 != 0x0) {
            // goto
            // LAB_1030_9144;
        }
//       TODO: goto LAB_1030_91fa;
    }
//LAB_1030_91cb:
    }
    puVar10 = uStack12;
    if ((uStack12._2_2_ | puVar10) != 0x0) {
        // uVar12 = (uStack12 >> 0x10);
        // uVar16 = (uStack12 >> 0x10);
//LAB_1030_90e6:
        ppc_var3 = *puVar10;
        (**ppc_var3)(0x1030, puVar10, uVar16, 0x1, uVar9, u_var11, uStack12, uStack12);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9296(param_1: u32, param_2: U32Ptr, param_3: u32, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let ppcVar1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let in_register_00000002: u16;
    let paVar4: &mut Struct99;
    let u_var6: u32;
    let uVar7: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let puVar8: U32Ptr;
    let extraout_DX_01: U32Ptr;
    let extraout_DX_02: U32Ptr;
    let extraout_DX_03: u16;
    let uVar9: u16;
    Struct116 * iVar11;
    Struct115 * iVar10;
    Struct114 * i_var9;
    let unaff_DI: i16;
    let u_var10: u16;
    let u_var11: u8;
    u8
    local_3a[0xc];
    let uStack46: u32;
    let uStack36: u32;
    let uStack30: u32;
    let uStack26: u16;
    let paStack18: &mut Struct99;
    let uStack14: u32;
    let puStack10: U32Ptr;
    let paStack6: &mut Struct99;
    Struct113 * u_var5;

    paVar4 = CONCAT22(in_register_00000002, param_5);
    pass1_1038_53ba(param_3, 0x1);
    uVar7 = param_6 | paVar4;
    // u_var10 = (param_2 >> 0x10);
    u_var11 = SUB41(param_2, 0x0);
    if (uVar7 != 0x0) {
        uStack30 = ctx._PTR_LOOP_1050_5768;
        u_var6 = ctx._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // uVar7 = (paStack18 >> 0x10);
        paVar4 = (u_var6 & 0xffff0000 | paStack18 & 0xffff);
        if ((uVar7 | (paStack18 & 0xffff)) == 0x0) {
            paStack6 = 0x0;
        } else {
            iVar11 = (Struct116 *)
            paStack18;
            paStack18.field_0x0 = 0x389a;
            iVar11.field_0x2 = 0x1008;
            iVar11.field_0x4 = 0x73;
            paStack18.field_0x0 = 0x9ec8;
            iVar11.field_0x2 = 0x1030;
            paVar4 = paStack18;
            paStack6 = paStack18;
        }
        ppcVar1 = (*param_2 + 0x4);
        (**ppcVar1)(0x1000, u_var11, u_var10, paStack6, (paStack6 >> 0x10));
        uVar7 = extraout_dx;
    }
    pass1_1038_53ba(param_3, 0x2);
    uVar7 |= paVar4;
    if (uVar7 != 0x0) {
        uStack30 = ctx._PTR_LOOP_1050_5768;
        u_var6 = ctx._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // uVar7 = (paStack18 >> 0x10);
        paVar4 = (u_var6 & 0xffff0000 | paStack18 & 0xffff);
        if ((uVar7 | (paStack18 & 0xffff)) == 0x0) {
            paStack6 = 0x0;
        } else {
            iVar10 = (Struct115 *)
            paStack18;
            paStack18.field_0x0 = 0x389a;
            iVar10.field_0x2 = 0x1008;
            iVar10.field_0x4 = 0x74;
            paStack18.field_0x0 = 0x9ec8;
            iVar10.field_0x2 = 0x1030;
            paVar4 = paStack18;
            paStack6 = paStack18;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(0x1000, u_var11, u_var10, paStack6, (paStack6 >> 0x10));
        uVar7 = extraout_DX_00;
    }
    pass1_1038_53ba(param_3, 0x3);
    puVar8 = (uVar7 | paVar4);
    if (puVar8 != 0x0) {
        uStack36 = ctx._PTR_LOOP_1050_5768;
        u_var6 = ctx._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // uVar7 = (paStack18 >> 0x10);
        paVar4 = (u_var6 & 0xffff0000 | paStack18 & 0xffff);
        if ((uVar7 | (paStack18 & 0xffff)) == 0x0) {
            paStack6 = 0x0;
        } else {
            i_var9 = (Struct114 *)
            paStack18;
            paStack18.field_0x0 = 0x389a;
            i_var9.field_0x2 = 0x1008;
            i_var9.field_0x4 = 0x75;
            paStack18.field_0x0 = 0x9ec8;
            i_var9.field_0x2 = 0x1030;
            paVar4 = paStack18;
            paStack6 = paStack18;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(0x1000, u_var11, u_var10, paStack6, (paStack6 >> 0x10));
        puVar8 = extraout_DX_01;
    }
    pass1_1030_8f04(param_1, (param_1 >> 0x10), param_3, paVar4,
                    puVar8);
    if (paVar4 != 0x0) {
        uStack36 = ctx._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // uVar7 = (paStack18 >> 0x10);
        u_var5 = (Struct113 *)
        paStack18;
        if ((uVar7 | u_var5) == 0x0) {
            paStack6 = 0x0;
        } else {
            paStack18.field_0x0 = 0x389a;
            u_var5.field_0x2 = 0x1008;
            u_var5.field_0x4 = 0x37;
            paStack18.field_0x0 = 0x9ec8;
            u_var5.field_0x2 = 0x1030;
            paStack6 = paStack18;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(0x1000, u_var11, u_var10, paStack6, (paStack6 >> 0x10));
        puVar8 = extraout_DX_02;
    }
    puStack10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_4, puVar8, unaff_DI);
    // u_var2 = (puStack10 >> 0x10);
    uStack14 = (puStack10 + 0xe);
    uVar7 = (puStack10 + 0x10);
    if ((uVar7 | uStack14) != 0x0) {
        pass1_1008_5784(CONCAT22(param_4, local_3a),
                        uStack14 & 0xffff | uVar7 << 0x10);
        loop {
            pu_var3 = local_3a;
            pass1_1008_5b12(pu_var3, param_4);
            uStack46 = CONCAT22(extraout_DX_03, pu_var3);
            if ((extraout_DX_03 | pu_var3) == 0x0) { break; }
            if (((pu_var3 + 0x4) == 0x3e) || ((pu_var3 + 0x4) == 0x41)) {
                uStack30 = ctx._PTR_LOOP_1050_5768;
                paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
                // uVar9 = (paStack18 >> 0x10);
                uVar7 = paStack18;
                if ((uVar9 | uVar7) == 0x0) {
                    paStack6 = 0x0;
                } else {
                    uStack26 = (uStack46 + 0x4);
                    paStack18.field_0x0 = 0x389a;
                    (uVar7 + 0x2) = 0x1008;
                    (uVar7 + 0x4) = uStack26;
                    paStack18.field_0x0 = 0x9ec8;
                    (uVar7 + 0x2) = 0x1030;
                    paStack6 = paStack18;
                }
                ppcVar1 = (*param_2 + 0x8);
                (**ppcVar1)(0x1000, u_var11, u_var10, paStack6, (paStack6 >> 0x10));
            }
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_951a(param_1: u16, param_2: u16, param_3: u32, param_4: U32Ptr, param_5: u32) {
    let ppcVar1: u32;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let extraout_dx: u16;
    let u_var10: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let iVar11: i16;
    let puVar12: U32Ptr;
    let unaff_DI: i16;
    let uVar13: u16;
    let uVar14: u16;
    let uVar15: u8;
    let puVar16: u32;
    let uVar17: u32;
    let uVar18: u8;
    let uVar19: u8;
    let uVar20: u8;
    let puStack76: u32;
    let uStack70: u32;
    let uStack62: u32;
    let paStack58: &mut Struct99;
    let local_36: u16;
    let uStack52: u16;
    let uStack46: u32;
    let uStack42: u16;
    let uStack40: u16;
    let iStack38: i16;
    let puStack36: U32Ptr;
    let puStack32: U32Ptr;
    let iStack28: i16;
    let iStack20: i16;
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: U32Ptr;
    let paStack6: &mut Struct99;
    let u_var2: &mut Struct122;
    let u_var3: &mut Struct123;
    let u_var4: &mut Struct124;
    let u_var5: &mut Struct125;

    puStack10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x35, param_1, param_2, unaff_DI);
    // puVar9 = (puStack10 >> 0x10);
    u_var6 = puStack10 + 0xa;
    uStack14 = puStack10 & 0xffff0000 | u_var6;
    pass1_1030_9048(param_1, param_3, 0x0, param_5);
    // uVar13 = (param_4 >> 0x10);
    uVar20 = SUB41(param_4, 0x0);
    if (u_var6 != 0x0) {
        iStack28 = 0x0;
        puStack32 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_1, puVar9, unaff_DI);
        // uVar14 = (puStack32 >> 0x10);
        puStack36 = (puStack32 + 0xe);
        u_var6 = (puStack32 + 0x10);
        if ((u_var6 | puStack36) != 0x0) {
            pass1_1008_5784(CONCAT22(param_1, &local_36),
                            puStack36 & 0xffff | u_var6 << 0x10);
            loop {
                puVar7 = &local_36;
                pass1_1008_5b12(puVar7, param_1);
                uStack46 = CONCAT22(extraout_dx, puVar7);
                if ((extraout_dx | puVar7) == 0x0) { break; }
                if ((puVar7[0x2] != 0x3e) && (puVar7[0x2] != 0x41)) {
                    paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
                    // u_var10 = (paStack6 >> 0x10);
                    u_var6 = paStack6;
                    if ((u_var10 | u_var6) == 0x0) {
                        paStack6 = 0x0;
                    } else {
                        uVar14 = (uStack46 + 0x4);
                        paStack6.field_0x0 = 0x389a;
                        (u_var6 + 0x2) = 0x1008;
                        (u_var6 + 0x4) = uVar14;
                        paStack6.field_0x0 = 0x9ec8;
                        (u_var6 + 0x2) = 0x1030;
                    }
                    ppcVar1 = (*param_4 + 0x8);
                    (**ppcVar1)(0x0, uVar20, uVar13, paStack6, (paStack6 >> 0x10));
                    if ((uStack46 + 0x4) == 0x13) {
                        iStack28 = 0x1;
                    }
                }
            }
        }
        // for (iStack38 = 0xa; iStack38 < 0x41; iStack38 += 0x1) {
        //   if ((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36)) &&
        //        ((iStack38 != 0x25 && (iStack38 != 0x26)))) &&
        //       ((iStack38 != 0x27 &&
        //        (((iStack38 * 0x2 + uStack14) != 0x0 && (iStack38 != 0x13)))))) &&
        //      ((iStack38 != 0x14 || (iStack28 == 0x0)))) {
        //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        //     u_var10 = (paStack6 >> 0x10);
        //     u_var6 = paStack6;
        //     if ((u_var10 | u_var6) == 0x0) {
        //       paStack6 = 0x0;
        //     }
        //     else {
        //       paStack6.field_0x0 = 0x389a;
        //       (u_var6 + 0x2) = 0x1008;
        //       (u_var6 + 0x4) = iStack38;
        //       paStack6.field_0x0 = 0x9ec8;
        //       (u_var6 + 0x2) = 0x1030;
        //     }
        //     ppcVar1 = (*param_4 + 0x8);
        //     (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
        //   }
        // }
    }
    // uVar14 = (uStack14 >> 0x10);
    if ((uStack14 + 0x6a) == 0x0) {
        if ((uStack14 + 0x6c) != 0x0) {
            paStack58 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
            // u_var6 = (paStack58 >> 0x10);
            puVar12 = paStack58;
            if ((u_var6 | puVar12) == 0x0) {
                // goto
                // LAB_1030_973e;
            }
            paStack58.field_0x0 = 0x389a;
            puVar12[0x1] = 0x1008;
            puVar12[0x2] = 0x36;
//       TODO: goto LAB_1030_9728;
        }
    } else {
        paStack58 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // u_var6 = (paStack58 >> 0x10);
        puVar12 = paStack58;
        if ((u_var6 | puVar12) == 0x0) {
//LAB_1030_973e:
            paStack6 = 0x0;
        } else {
            paStack58.field_0x0 = 0x389a;
            puVar12[0x1] = 0x1008;
            puVar12[0x2] = 0x35;
//LAB_1030_9728:
            *puVar12 = 0x9ec8;
            puVar12[0x1] = 0x1030;
            paStack6 = paStack58;
        }
        ppcVar1 = (*param_4 + 0x8);
        (**ppcVar1)(0x0, uVar20, uVar13, paStack6, (paStack6 >> 0x10));
    }
    // uVar14 = (uStack14 >> 0x10);
    iVar11 = uStack14;
    if ((iVar11 + 0x4a) == 0x0) {
        if ((iVar11 + 0x4c) == 0x0) {
            if ((iVar11 + 0x4e) == 0x0) {
                // goto
                // LAB_1030_97e8;
            }
            paStack58 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
            // u_var6 = (paStack58 >> 0x10);
            puVar12 = paStack58;
            if ((u_var6 | puVar12) != 0x0) {
                paStack58.field_0x0 = 0x389a;
                puVar12[0x1] = 0x1008;
                puVar12[0x2] = 0x27;
//         TODO: goto LAB_1030_9879;
            }
        } else {
            paStack58 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
            // u_var6 = (paStack58 >> 0x10);
            puVar12 = paStack58;
            if ((u_var6 | puVar12) != 0x0) {
                paStack58.field_0x0 = 0x389a;
                puVar12[0x1] = 0x1008;
                puVar12[0x2] = 0x26;
//         TODO: goto LAB_1030_9879;
            }
        }
//LAB_1030_97d0:
        paStack6 = 0x0;
    } else {
        paStack58 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // u_var6 = (paStack58 >> 0x10);
        puVar12 = paStack58;
        if ((u_var6 | puVar12) == 0x0) {
            // goto
            // LAB_1030_97d0;
        }
        paStack58.field_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x25;
//LAB_1030_9879:
        *puVar12 = 0x9ec8;
        puVar12[0x1] = 0x1030;
        paStack6 = paStack58;
    }
    ppcVar1 = (*param_4 + 0x8);
    (**ppcVar1)(0x0, uVar20, uVar13, paStack6, (paStack6 >> 0x10));
//LAB_1030_97e8:
    uStack18 = puStack10 & 0xffff0000 | (puStack10 + 0x11e);
    if ((puStack10 + 0x138) != 0x0) {
        puVar16 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
        // puVar9 = (puVar16 >> 0x10);
        u_var6 = puVar16;
        uVar15 = 0x38;
        pass1_1038_4d6e(param_5, puVar16, u_var6, puVar9);
        puStack76 = CONCAT22(puVar9, u_var6);
        ppcVar1 = (*puStack76 + 0x10);
        u_var10 = u_var6;
        (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var6, puVar9);
        uStack70 = CONCAT22(extraout_DX_00, u_var10);
        // for (uStack62 = 0x0; uStack62 < uStack70; uStack62 += 0x1) {
        //   ppcVar1 = (*puStack76 + 0x4);
        //   uVar17 = uStack70;
        //   (**ppcVar1)(uVar15,u_var6,puVar9,uStack62,(uStack62 >> 0x10));
        //   uVar8 = uVar17;
        //   iVar11 = 0xd;
        //   u_var10 = extraout_DX_01;
        //   local_36 = uVar8;
        //   uStack52 = extraout_DX_01;
        //   pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,uVar8,extraout_DX_01);
        //   uStack46 = CONCAT22(u_var10,uVar8);
        //   uVar17 = struct_op_1030_73a8(CONCAT22(u_var10,uVar8));
        //   u_var10 = (uVar17 >> 0x10);
        //   uStack42 = uVar17;
        //   uVar15 = 0x28;
        //   uStack40 = u_var10;
        //   uVar8 = pass1_1028_6744(param_1,uVar17,iVar11);
        //   if ((u_var10 | uVar8) != 0x0) {
        //     puStack32 = ctx._PTR_LOOP_1050_5768;
        //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        //     u_var10 = (paStack6 >> 0x10);
        //     u_var5 = paStack6;
        //     if ((u_var10 | u_var5) == 0x0) {
        //       paStack6 = 0x0;
        //     }
        //     else {
        //       paStack6.field_0x0 = 0x389a;
        //       u_var5.field_0x2 = 0x1008;
        //       u_var5.field_0x4 = 0x4c;
        //       paStack6.field_0x0 = 0x9ec8;
        //       u_var5.field_0x2 = 0x1030;
        //     }
        //     ppcVar1 = (*param_4 + 0x8);
        //     (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
        //     puStack36 = ctx._PTR_LOOP_1050_5768;
        //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        //     u_var10 = (paStack6 >> 0x10);
        //     u_var4 = paStack6;
        //     if ((u_var10 | u_var4) == 0x0) {
        //       paStack6 = 0x0;
        //     }
        //     else {
        //       paStack6.field_0x0 = 0x389a;
        //       u_var4.field_0x2 = 0x1008;
        //       u_var4.field_0x4 = 0x4d;
        //       paStack6.field_0x0 = 0x9ec8;
        //       u_var4.field_0x2 = 0x1030;
        //     }
        //     uVar18 = SUB41(paStack6,0x0);
        //     uVar19 = (paStack6 >> 0x10);
        //     ppcVar1 = (*param_4 + 0x8);
        //     puVar16 = param_4;
        //     (**ppcVar1)();
        //     puStack36 = ctx._PTR_LOOP_1050_5768;
        //     uVar15 = 0x0;
        //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        //     u_var10 = (paStack6 >> 0x10);
        //     u_var3 = paStack6;
        //     if ((u_var10 | u_var3) == 0x0) {
        //       paStack6 = 0x0;
        //     }
        //     else {
        //       paStack6.field_0x0 = 0x389a;
        //       u_var3.field_0x2 = 0x1008;
        //       u_var3.field_0x4 = 0x4e;
        //       paStack6.field_0x0 = 0x9ec8;
        //       u_var3.field_0x2 = 0x1030;
        //     }
        //     ppcVar1 = (*param_4 + 0x8);
        //     (**ppcVar1)(0x1000,param_4,paStack6,puVar16,uVar18,uVar19);
        //     break;
        //   }
        // }
        if (puStack76 != 0x0) {
            ppcVar1 = *puStack76;
            (**ppcVar1)(uVar15, u_var6, puVar9, 0x1);
        }
    }
    // for (iStack20 = 0x7a; iStack20 < 0x7d; iStack20 += 0x1) {
    //   if ((iStack20 * 0x2 + uStack14) != 0x0) {
    //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    //     u_var6 = (paStack6 >> 0x10);
    //     u_var2 = paStack6;
    //     if ((u_var6 | u_var2) == 0x0) {
    //       paStack6 = 0x0;
    //     }
    //     else {
    //       paStack6.field_0x0 = 0x389a;
    //       u_var2.field_0x2 = 0x1008;
    //       u_var2.field_0x4 = iStack20;
    //       paStack6.field_0x0 = 0x9ec8;
    //       u_var2.field_0x2 = 0x1030;
    //     }
    //     ppcVar1 = (*param_4 + 0x8);
    //     (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
    //   }
    // }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9adc(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: u32, param_5: u16,
                       param_6: u16)

{
    let ppcVar1: u32;
    let paVar2: &mut Struct99;
    let u_var4: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let iVar7: &mut Struct121;
    Struct119 * iVar6;
    let paStack6: &mut Struct99;
    let u_var3: &mut Struct120;

    pass1_1038_53ba(param_4, 0x1);
    u_var4 = param_6 | param_5;
    if (u_var4 != 0x0) {
        paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // u_var4 = (paStack6 >> 0x10);
        paVar2 = (paStack6 & 0xffff);
        if ((u_var4 | paVar2) == 0x0) {
            paStack6 = 0x0;
        } else {
            iVar7 = paStack6;
            paStack6.field_0x0 = 0x389a;
            iVar7.field_0x2 = 0x1008;
            iVar7.field_0x4 = 0x77;
            paStack6.field_0x0 = 0x9ec8;
            iVar7.field_0x2 = 0x1030;
            paVar2 = paStack6;
        }
        param_5 = paVar2;
        ppcVar1 = (*param_3 + 0x4);
        (**ppcVar1)(0x1000, param_3, paStack6, (paStack6 >> 0x10));
        u_var4 = extraout_dx;
    }
    pass1_1038_53ba(param_4, 0x2);
    u_var4 |= param_5;
    if (u_var4 != 0x0) {
        paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // u_var4 = (paStack6 >> 0x10);
        paVar2 = (paStack6 & 0xffff);
        if ((u_var4 | paVar2) == 0x0) {
            paStack6 = 0x0;
        } else {
            iVar6 = (Struct119 *)
            paStack6;
            paStack6.field_0x0 = 0x389a;
            iVar6.field_0x2 = 0x1008;
            iVar6.field_0x4 = 0x78;
            paStack6.field_0x0 = 0x9ec8;
            iVar6.field_0x2 = 0x1030;
            paVar2 = paStack6;
        }
        param_5 = paVar2;
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(0x1000, param_3, paStack6, (paStack6 >> 0x10));
        u_var4 = extraout_DX_00;
    }
    pass1_1038_53ba(param_4, 0x3);
    if ((u_var4 | param_5) != 0x0) {
        paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        // u_var4 = (paStack6 >> 0x10);
        u_var3 = paStack6;
        if ((u_var4 | u_var3) == 0x0) {
            paStack6 = 0x0;
        } else {
            paStack6.field_0x0 = 0x389a;
            u_var3.field_0x2 = 0x1008;
            u_var3.field_0x4 = 0x75;
            paStack6.field_0x0 = 0x9ec8;
            u_var3.field_0x2 = 0x1030;
        }
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(0x1000, param_3, paStack6, (paStack6 >> 0x10));
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9c1c(param_1: u32, param_2: U32Ptr, param_3: u32) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let iVar5: i16;
    let in_DX: U32Ptr;
    let u_var6: u16;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let puVar7: U32Ptr;
    let iStack24: i16;
    let iStack16: i16;
    let paStack6: &mut Struct99;

    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x35, unaff_SS, in_DX, unaff_DI);
    i_var4 = puVar7 + 0xa;
    // u_var3 = (puVar7 >> 0x10);
    iVar5 = i_var4;
    pass1_1030_9048(unaff_SS, param_1, 0x1, param_3);
    if (iVar5 != 0x0) {
        // for (iStack24 = 0x4f; iStack24 < 0x70; iStack24 += 0x1) {
        //   if ((iStack24 * 0x2 + i_var4) != 0x0) {
        //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        //     u_var6 = (paStack6 >> 0x10);
        //     u_var2 = paStack6;
        //     if ((u_var6 | u_var2) == 0x0) {
        //       paStack6 = 0x0;
        //     }
        //     else {
        //       paStack6.field_0x0 = 0x389a;
        //       (u_var2 + 0x2) = 0x1008;
        //       (u_var2 + 0x4) = iStack24;
        //       paStack6.field_0x0 = 0x9ec8;
        //       (u_var2 + 0x2) = 0x1030;
        //     }
        //     ppcVar1 = (*param_2 + 0x8);
        //     (**ppcVar1)(0x1000,param_2,paStack6,(paStack6 >> 0x10));
        //   }
        // }
    }
    // for (iStack16 = 0x7d; iStack16 < 0x80; iStack16 += 0x1) {
    //   if ((iStack16 * 0x2 + i_var4) != 0x0) {
    //     paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    //     u_var6 = (paStack6 >> 0x10);
    //     u_var2 = paStack6;
    //     if ((u_var6 | u_var2) == 0x0) {
    //       paStack6 = 0x0;
    //     }
    //     else {
    //       paStack6.field_0x0 = 0x389a;
    //       (u_var2 + 0x2) = 0x1008;
    //       (u_var2 + 0x4) = iStack16;
    //       paStack6.field_0x0 = 0x9ec8;
    //       (u_var2 + 0x2) = 0x1030;
    //     }
    //     ppcVar1 = (*param_2 + 0x8);
    //     (**ppcVar1)(0x1000,param_2,paStack6,(paStack6 >> 0x10));
    //   }
    // }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9d42(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: U32Ptr, param_6: u32)

{
    let pu_var1: u32;
    let u_var2: u32;
    let ppc_var3: u32;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let uVar8: u16;
    let i_var9: i16;
    let u_var10: u16;
    let local_a6: [u8; 4];
    let uStack162: u32;
    let uStack158: u32;
    let iStack154: i16;
    let local_98: u32;
    let uStack12: u32;
    let uStack8: u32;
    let i_stack4: i16;
    let uVar7: u32;

    // u_var10 = (param_6 >> 0x10);
    if ((param_6 + 0x206) == 0x0) {
        i_stack4 = (param_6 + 0x204);
        puVar4 = pass1_1000_4906(CONCAT22(param_1, &local_98), 0x0,
                                 0x94);
        uVar7 = ZEXT24(puVar4);
        iStack154 = 0x11;
        loop {
            empty_1038_540a();
            u_var10 = uVar7;
            (&local_98 + iStack154) = u_var10;
            (&local_98 + iStack154 * 0x4 + 0x2) = param_2;
            iStack154 += 0x1;
            if (iStack154 < 0x25) == false { break; }
        }
        empty_1038_540a();
        uStack158 = CONCAT22(param_2, u_var10);
        pass1_1008_5784(CONCAT22(param_1, local_a6), param_5);
        uVar7 = (ctx.PTR_LOOP_1050_65e2 + 0x52);
        loop {
            pu_var5 = local_a6;
            pass1_1008_5b12(pu_var5, param_1);
            uVar8 = extraout_dx | pu_var5;
            if (uVar8 == 0x0) { break; }
            puVar6 = pu_var5;
            pass1_1030_4bbe(param_1, uVar8, uVar7, (pu_var5 + 0x4));
            if (i_stack4 == 0x0) {
                // for (iStack154 = 0x11; iStack154 < 0x25; iStack154 += 0x1) {
                //   i_var9 = iStack154 * 0x4;
                //   if (((puVar6 + i_var9) != 0x0) &&
                //      (u_var2 = (&local_98)[iStack154], pu_var1 = (puVar6 + i_var9),
                //      u_var2 <= *pu_var1 && *pu_var1 != u_var2)) {
                //     pu_var1 = (puVar6 + i_var9);
                //     if (uStack158 <= *pu_var1 && *pu_var1 != uStack158) goto LAB_1030_9e17;
                //     uStack158 -= (puVar6 + i_var9);
                //   }
                // }
            } else {
                pu_var1 = (puVar6 + 0x8c);
                if ((uStack12 <= *pu_var1 && *pu_var1 != uStack12) || (pu_var1 = (puVar6 + 0x90), uStack8 <= *pu_var1 && *pu_var1 != uStack8)) {
//LAB_1030_9e17:
                    ppc_var3 = (*param_5 + 0xc);
                    (**ppc_var3)(0x1008, param_5, (param_5 >> 0x10), pu_var5,
                                extraout_dx);
                    uStack162 = 0x0;
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_9e9c(param_1: U32Ptr, param_2: u8) -> u16

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x389a;
    (param_1)[0x1] = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        pass1_1000_093a(param_1, u_var1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1030_9ecc(param_1: U32Ptr, param_2: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x0;
    (param_1 + 0x4) = param_2;
    (param_1 + 0x8) = 0x0;
    return;
}


pub fn pass1_1030_9ef2(param_1: U32Ptr) -> u16

{
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;

    if (*param_1 != 0x0) {
        u_var3 = struct_op_1030_73a8(*param_1);
        // u_var2 = (u_var3 >> 0x10);
        i_var1 = (u_var3 + 0xc);
        if (((i_var1 != 0x5) && (i_var1 != 0x9)) && ((u_var3 + 0x12) < 0x5)) {
            return 0x0;
        }
        pass1_1030_9f64(param_1);
    }
    return 0x1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9f40(param_1: u32, param_2: u16, param_3: u16, param_4: u8) {
    let u_var1: u16;

    u_var1 = pass1_1008_c646(ctx.PTR_LOOP_1050_06e0,
                            CONCAT22(param_2, (ctx.PTR_LOOP_1050_06e0 >> 0x10)),
                            param_3);
    (param_1 + 0x8) = u_var1;
    pass1_1030_9f7a(param_1, u_var1, param_3, param_4);
    return;
}


pub fn pass1_1030_9f64(param_1: U32Ptr) {
    (param_1 + 0x8) = 0x0;
    *param_1 = 0x0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_9f7a(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u8) {
    let u_var1: u32;
    let BVar2: bool;
    let pu_var3: u32;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u16;
    let local_130: [u8; 120];
    let uStack16: u32;
    let uStack12: u32;
    let local_8: u32;
    let i_stack4: i16;

    clear_struct_1008_3e38(CONCAT22(param_3, &local_8));
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, param_2, 0x28);
    if (BVar2 != 0x0) {
        i_stack4 = 0x1;
    }
    pu_var3 = &local_8;
    pass1_1030_a278(param_1, CONCAT22(param_3, pu_var3), pu_var3, param_3,
                    param_4);
    if (pu_var3 != 0x0) {
        // u_var5 = (param_1 >> 0x10);
        u_var4 = param_1;
        u_var1 = (u_var4 + 0x4);
        uStack12 = (u_var1 + 0x8);
        u_var1 = (u_var4 + 0x4);
        struct_op_1028_87f0(param_3, param_4, CONCAT22(param_3, local_130), 0x0, 0x0,
                            param_2, &local_8, param_3, (u_var1 + 0x4), uStack12);
        fn_ptr_1028::fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_3, local_130));
        pass1_1028_b58e(uStack16);
        *param_1 = uStack16;
        (u_var4 + 0x2) = extraout_dx;
        if (0x0 < i_stack4) {
            pass1_1030_a044(param_3, extraout_dx, param_4, u_var4, u_var5,
                            CONCAT22(param_3, &local_8), uStack12);
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_a044(param_1: u16, param_2: u16, param_3: u8, param_4: u16, param_5: u16,
                       param_6: U32Ptr, param_7: u32)

{
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let i_var4: i16;
    let u_var5: u32;
    let u_var6: u16;
    let extraout_dx: u16;
    let uVar7: u16;
    let puVar8: u32;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let local_17e: u16;
    let uStack380: u16;
    let iStack90: i16;
    let puStack78: u32;
    let uStack70: u16;
    let i_stack68: i16;
    let uStack66: u32;
    let puStack62: u32;
    u8
    local_3a[0xc];
    let local_2e: u32;
    let uStack42: u16;
    let iStack40: i16;
    let uStack38: u16;
    let local_24: i16;
    let local_22: i16;
    let uStack32: u32;
    let uStack28: u32;
    let uStack24: u32;
    let puStack20: U32Ptr;
    let uStack18: u16;
    let iStack16: i16;
    let iStack14: i16;
    let uStack12: u32;
    let local_8: u16;
    let local_6: i16;
    let local_4: i16;

    pu_var2 = &local_8;
    pass1_1008_3eb4(param_6, CONCAT22(param_1, pu_var2),
                    CONCAT22(param_1, &local_6),
                    CONCAT22(param_1, &local_4));
    pass1_1030_627e(param_1, pu_var2, param_2, _PTR_LOOP_1050_5740, param_6, param_7);
    puStack20 = pu_var2;
    uStack18 = param_2;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2);
    uStack24 = CONCAT22(param_2, pu_var2);
    uStack28 = (pu_var2 + 0x17);
    u_var5 = (uStack28 + 0x4);
    uStack32 = u_var5;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_7);
    iStack40 = u_var5;
    uStack38 = param_2;
    puVar8 = pass1_1030_5b5c(iStack40, param_2);
    // u_var6 = (puVar8 >> 0x10);
    local_2e = *puVar8;
    uStack42 = (puVar8 + 0x4);
    puStack78 = &local_2e;
    pass1_1008_3e94(CONCAT22(param_1, &local_2e),
                    CONCAT22(param_1, &local_24),
                    CONCAT22(param_1, &local_22));
    iStack14 = local_4 + 0x1;
    uStack12 = CONCAT22(local_4 + -0x1, local_6 - 0x1);
    iStack16 = local_6 + 0x1;
    if (local_4 + -0x1 < 0x0) {
        uStack12 = (local_6 - 0x1);
    }
    if (local_22 <= iStack14) {
        iStack14 = local_22 + -0x1;
    }
    if (uStack12 < 0x0) {
        uStack12 &= 0xffff0000;
    }
    if (local_24 <= iStack16) {
        iStack16 = local_24 + -0x1;
    }
    pass1_1008_6c90(CONCAT22(param_1, local_3a));
    uVar7 = 0x1008;
    pass1_1008_6cec(CONCAT22(param_1, local_3a), local_8, CONCAT22(iStack14, iStack16), local_8, uStack12);
    pu_var3 = local_3a;
    pass1_1030_6522(ctx.PTR_LOOP_1050_5740, CONCAT22(param_1, pu_var3), param_7, param_1);
    puStack62 = CONCAT22(u_var6, pu_var3);
    if ((u_var6 | pu_var3) != 0x0) {
        uStack66 = 0x0;
        i_stack68 = 0x0;
//     for (uStack70 = uStack12; uStack70 <= iStack16; uStack70 += 0x1) {
//       for (puStack78 = uStack12._2_2_; puStack78 <= iStack14;
//           puStack78 = (puStack78 + 0x1)) {
//         ppcVar1 = (*puStack62 + 0x4);
//         i_var4 = i_stack68;
//         i_stack68 = i_stack68 + 0x1;
//         (**ppcVar1)(uVar7,puStack62,(puStack62 >> 0x10));
//         uStack66 = CONCAT22(extraout_dx,i_var4);
//         uStack66._3_1_ = (extraout_dx >> 0x8);
//         if (uStack66._3_1_ == '\0') {
//           iStack90 = i_var4;
//           if (i_var4 == 0x7) {
//             pass1_1008_3e76(param_6,local_8,uStack70,puStack78);
//             u_var10 = uStack32;
//             u_var11 = (uStack32 >> 0x10);
//             uVar9 = 0x6;
//           }
//           else {
//             if (i_var4 == 0x8) {
//               pass1_1008_3e76(param_6,local_8,uStack70,puStack78);
//               u_var10 = uStack32;
//               u_var11 = (uStack32 >> 0x10);
//               uVar9 = 0x7;
//             }
//             else {
//               if (i_var4 != 0x9) goto LAB_1030_a1d0;
//               pass1_1008_3e76(param_6,local_8,uStack70,puStack78);
//               u_var10 = uStack32;
//               u_var11 = (uStack32 >> 0x10);
//               uVar9 = 0x8;
//             }
//           }
//           uVar7 = SUB42(&USHORT_1050_1028,0x0);
//           struct_op_1028_87f0(param_1,param_3,CONCAT22(param_1,&local_17e),
//                               0x0,0x0,uVar9,param_6,
//                               (param_6 >> 0x10),CONCAT22(u_var11,u_var10),
//                               param_7);
//           fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,&local_17e));
//           local_17e = 0x389a;
//           uStack380 = 0x1008;
//         }
// //LAB_1030_a1d0:
//       }
//     }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_a278(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: u16,
                       param_5: u8)

{
    let i_var1: i16;
    let u_var2: u32;
    let in_DX: i16;
    let extraout_dx: u16;
    let pu_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let local_134: [u8; 120];
    let uStack20: u32;
    let uStack16: u32;
    let uStack12: u32;
    let uStack6: u16;
    let uStack4: u16;

    uStack4 = 0x1;
    pass1_1030_a39a(param_1, param_2, param_4);
    if (param_3 != 0x0) {
        return;
    }
    uStack6 = param_3;
    pass1_1030_a3ae(param_1, param_2, param_4);
    pu_var3 = param_2;
    // u_var5 = (param_2 >> 0x10);
    if (param_3 == 0x0) {
        pass1_1030_a57e(param_1, param_2, 0x0, in_DX, param_4);
        if (param_3 == 0x0) {
            pass1_1030_a844(param_1, param_2, 0x0, in_DX, param_4);
            if (param_3 == 0x0) {
                uStack4 = 0x0;
//         TODO: goto LAB_1030_a305;
            }
            i_var1 = (pu_var3 + 0x1);
        } else {
            i_var1 = (pu_var3 + 0x1);
        }
        if (i_var1 < 0x1) {
            uStack6 = 0x73;
        } else {
            uStack6 = 0x77;
        }
    } else {
        if ((pu_var3 + 0x1) < 0x1) {
            uStack6 = 0x7a;
        } else {
            uStack6 = 0x7f;
        }
    }
//LAB_1030_a305:
    if (uStack6 != 0x0) {
        // u_var6 = (param_1 >> 0x10);
        u_var4 = param_1;
        u_var2 = (u_var4 + 0x4);
        uStack16 = (u_var2 + 0x8);
        u_var2 = (u_var4 + 0x4);
        struct_op_1028_87f0(param_4, param_5, CONCAT22(param_4, local_134), 0x0, 0x0,
                            uStack6, pu_var3, u_var5, (u_var2 + 0x4), uStack16);
        fn_ptr_1028::fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_4, local_134));
        uStack12 = uStack20;
        pass1_1028_b58e(uStack20);
        *param_1 = uStack20;
        (u_var4 + 0x2) = extraout_dx;
        if (0x0 < (pu_var3 + 0x1)) {
            pass1_1030_a044(param_4, extraout_dx, param_5, u_var4, u_var6,
                            (param_2 & 0xffff | u_var5 << 0x10), uStack16);
        }
    }
    return;
}


pub fn pass1_1030_a39a(param_1: u32, param_2: U32Ptr, param_3: u16) {
    pass1_1030_aa18(param_1, param_2, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_a3ae(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let BVar5: bool;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let iVar11: i16;
    let uVar12: u16;
    let uVar13: u16;
    let puVar14: u32;
    let puVar15: U32Ptr;
    let uVar16: u16;
    let uStack44: u32;
    let local_28: i16;
    let local_26: i16;
    let local_24: i16;
    let local_22: [u8; 6];
    let local_1c: i16;
    let iStack26: i16;
    let lStack22: i32;
    let uStack18: u32;
    let puStack14: u32;
    let uStack10: u16;
    let puStack8: U32Ptr;
    let i_stack6: i16;
    let uStack4: u16;

    uStack4 = 0x0;
    i_stack6 = (param_2 + 0x4);
    puVar14 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x45);
    // puVar7 = (puVar14 >> 0x10);
    u_var3 = puVar14;
    // uVar12 = (param_1 >> 0x10);
    u_var10 = param_1;
    uStack10 = u_var3;
    puStack8 = puVar7;
    pass1_1038_4e78(u_var3, puVar7, (u_var10 + 0x4), puVar14);
    puStack14 = CONCAT22(puVar7, u_var3);
    ppcVar1 = (*puStack14 + 0x10);
    uVar16 = u_var3;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var3, puVar7);
    uStack18 = CONCAT22(extraout_dx, u_var3);
    u_var2 = (u_var10 + 0x4);
    lStack22 = (u_var2 + 0x8);
    clear_struct_1008_3e38(CONCAT22(param_3, &local_1c));
    puVar15 = clear_struct_1008_3e38(CONCAT22(param_3, local_22));
    uStack44 = 0x0;
    // uVar8 = (puVar15 >> 0x10);
    loop {
        if (uStack18 <= uStack44) {
//LAB_1030_a4e7:
            if (puStack14 != 0x0) {
                ppcVar1 = *puStack14;
                (**ppcVar1)(0x1008, puStack14, (puStack14 >> 0x10), 0x1, uVar16,
                            puVar7, puStack14, puStack14);
            }
            return;
        }
        u_var6 = uStack18;
        pass1_1030_1d58(puStack14);
        uVar9 = uVar8 | u_var6;
        if (uVar9 != 0x0) {
            pass1_1008_3f62(CONCAT22(param_3, &local_1c),
                            CONCAT22(uVar8, u_var6 + 0xc));
            pass1_1008_3eb4(CONCAT22(param_3, &local_1c),
                            CONCAT22(param_3, &local_28),
                            CONCAT22(param_3, &local_26),
                            CONCAT22(param_3, &local_24));
            uVar9 = uVar8;
            if ((local_28 == i_stack6) && (u_var2 = (u_var10 + 0x4),
                                          uVar13 = (u_var2 >> 0x10), iVar11 = u_var2,
                                          u_var2 = (iVar11 + 0x4),
                                          u_var4 = pass1_1030_addc(u_var10, uVar12, CONCAT22(param_3, &local_1c),
                                                                  u_var2, (u_var2 >> 0x10),
                                                                  (iVar11 + 0x8), &local_1c, uVar8, param_3),
                                          uVar9 = uVar8, u_var4 != 0x0)) {
                pass1_1008_3f62(CONCAT22(param_3, local_22),
                                CONCAT22(param_3, &local_1c));
                iStack26 = local_26 + -0x1;
                BVar5 = pass1_1030_ad22(u_var10, uVar12, CONCAT22(param_3, &local_1c),
                                        lStack22, &local_1c, uVar8, param_3);
                if (BVar5 == 0x0) {
                    iStack26 = local_26 + 0x1;
                    BVar5 = pass1_1030_ad22(u_var10, uVar12, CONCAT22(param_3, &local_1c),
                                            lStack22, &local_1c, uVar8, param_3);
                    if (BVar5 == 0x0) {
                        iStack26 = local_26;
                        local_1c = local_24 + -0x1;
                        BVar5 = pass1_1030_ad22(u_var10, uVar12, CONCAT22(param_3, &local_1c),
                                                lStack22, &local_1c, uVar8, param_3);
                        if (BVar5 == 0x0) {
                            local_1c = local_24 + 0x1;
                            BVar5 = pass1_1030_ad22(u_var10, uVar12, CONCAT22(param_3, &local_1c),
                                                    lStack22, &local_1c, uVar8, param_3);
                            uVar9 = uVar8;
                            if (BVar5 == 0x0) {
                                // goto
                                // LAB_1030_a45b;
                            }
                        }
                    }
                }
                pass1_1008_3f62(param_2, CONCAT22(param_3, local_22));
                uStack4 = 0x1;
//         TODO: goto LAB_1030_a4e7;
            }
        }
//LAB_1030_a45b:
        uStack44 += 0x1;
        uVar8 = uVar9;
    }
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_a57e(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: i16, param_5: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let piVar5: U32Ptr;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let iVar12: i16;
    let puVar13: u32;
    let uVar14: u16;
    let uVar15: u16;
    let uVar16: u16;
    let uVar17: u16;
    let puVar18: u32;
    let uVar19: u32;
    let uVar20: u8;
    let uStack40: u32;
    let local_1c: [u8; 2];
    let local_1a: i16;
    let local_18: i16;
    let local_16: i16;
    let iStack20: i16;
    let uStack16: u32;
    let uStack12: u16;
    let puStack10: U32Ptr;
    let iStack8: i16;
    let i_stack6: i16;
    let uStack4: u16;

    uStack4 = 0x0;
    // uVar14 = (param_1 >> 0x10);
    u_var10 = param_1;
    pass1_1038_53ba((u_var10 + 0x4), 0x1);
    if ((param_4 != 0x0) || (param_3 != 0x0)) {
        i_stack6 = (param_2 + 0x4);
        iStack8 = 0x8 - (i_stack6 == 0x0);
        puVar18 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, iStack8);
        // puVar7 = (puVar18 >> 0x10);
        uVar8 = puVar18;
        uStack12 = uVar8;
        puStack10 = puVar7;
        pass1_1038_4e78(uVar8, puVar7, (u_var10 + 0x4), puVar18);
        uStack16 = CONCAT22(puVar7, uVar8);
        uVar17 = 0x1008;
        clear_struct_1008_3e38(CONCAT22(param_5, &local_16));
        u_var3 = (u_var10 + 0x4);
        u_var1 = (u_var3 + 0x8);
        // uVar15 = (uStack16 >> 0x10);
        u_var11 = SUB42(uStack16, 0x0);
        ppcVar2 = (*uStack16 + 0x10);
        u_var6 = u_var1;
        (**ppcVar2)(0x1008, u_var11, uVar15);
        u_var6 = u_var6 & 0xffff | extraout_dx << 0x10;
        uVar8 = extraout_dx;
//     for (uStack40 = 0x0; uStack40 < u_var6; uStack40 += 0x1) {
//       uVar19 = u_var6;
//       pass1_1030_1d58(uStack16);
//       uVar9 = uVar8 | uVar19;
//       if (uVar9 != 0x0) {
//         uVar9 = uVar8;
//         pass1_1008_3f62(CONCAT22(param_5,&local_16),
//                         CONCAT22(uVar8,uVar19 + 0xc));
//         uVar17 = 0x1008;
//         pass1_1008_3eb4(CONCAT22(param_5,&local_16),
//                         CONCAT22(param_5,local_1c),
//                         CONCAT22(param_5,&local_1a),
//                         CONCAT22(param_5,&local_18));
//         u_var3 = (u_var10 + 0x4);
//         uVar16 = (u_var3 >> 0x10);
//         iVar12 = u_var3;
//         u_var3 = (iVar12 + 0x4);
//         u_var4 = pass1_1030_addc(u_var10,uVar14,CONCAT22(param_5,&local_16),
//                                 u_var3,(u_var3 >> 0x10),
//                                 (iVar12 + 0x8),&local_16,uVar9,param_5);
//         if (u_var4 == 0x0) goto LAB_1030_a660;
//         uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | uVar8 << 0x10);
//         uVar9 = (uVar19 >> 0x10);
//         iVar12 = (uVar19 + 0xc);
//         if (0x5 < iVar12 - 0x7a) goto LAB_1030_a660;
//         uVar17 = 0x1030;
//         switch(iVar12) {
//         default:
//           iStack20 = local_1a + -0x1;
//           piVar5 = &local_16;
//           pass1_1030_ad86(u_var10,uVar14,CONCAT22(param_5,piVar5),u_var1,param_5,
//                           uVar9);
//           if (piVar5 != 0x0) goto LAB_1030_a7df;
//           iStack20 = local_1a + 0x1;
//           piVar5 = &local_16;
//           pass1_1030_ad86(u_var10,uVar14,CONCAT22(param_5,piVar5),u_var1,param_5,
//                           uVar9);
//           if (piVar5 == 0x0) {
//             iStack20 = local_1a;
//             local_16 = local_18 + -0x1;
//             piVar5 = &local_16;
//             pass1_1030_ad86(u_var10,uVar14,CONCAT22(param_5,piVar5),u_var1,param_5
//                             ,uVar9);
// //             TODO: goto joined_r0x1030a722;
//           }
// //LAB_1030_a748:
//           pass1_1008_3f62(param_2,CONCAT22(param_5,&local_16));
//           break;
//         0x7b =>
//         0x7e =>
//           iStack20 = local_1a + -0x1;
//           piVar5 = &local_16;
//           pass1_1030_ad86(u_var10,uVar14,CONCAT22(param_5,piVar5),u_var1,param_5,
//                           uVar9);
//           if (piVar5 == 0x0) {
//             iStack20 = local_1a + 0x1;
// //             TODO: goto LAB_1030_a730;
//           }
//           pass1_1008_3f62(param_2,CONCAT22(param_5,&local_16));
//           if (uStack16 == 0x0) {
//             return;
//           }
//           uVar17 = (uStack16 >> 0x10);
//           puVar13 = uStack16;
//           uVar20 = (uStack16 >> 0x10);
// //           TODO: goto LAB_1030_a6ea;
//         0x7c =>
//         0x7d =>
//           local_16 = local_18 + -0x1;
//           piVar5 = &local_16;
//           pass1_1030_ad86(u_var10,uVar14,CONCAT22(param_5,piVar5),u_var1,param_5,
//                           uVar9);
// joined_r0x1030a722:
//           if (piVar5 == 0x0) {
//             local_16 = local_18 + 0x1;
// //LAB_1030_a730:
//             piVar5 = &local_16;
//             pass1_1030_ad86(u_var10,uVar14,CONCAT22(param_5,piVar5),u_var1,param_5
//                             ,uVar9);
//             if (piVar5 != 0x0) goto LAB_1030_a748;
// //             TODO: goto LAB_1030_a660;
//           }
// //LAB_1030_a7df:
//           pass1_1008_3f62(param_2,CONCAT22(param_5,&local_16));
//         }
//         puVar13 = uStack16;
//         if ((uStack16._2_2_ | puVar13) != 0x0) {
//           uVar17 = (uStack16 >> 0x10);
//           uVar20 = (uStack16 >> 0x10);
// //LAB_1030_a6ea:
//           ppcVar2 = *puVar13;
//           (**ppcVar2)(0x1008,puVar13,uVar20,0x1,u_var11,uVar15,uStack16,uStack16);
//         }
//         return;
//       }
// //LAB_1030_a660:
//       uVar8 = uVar9;
//     }
        if (uStack16 != 0x0) {
            ppcVar2 = *uStack16;
            (**ppcVar2)(uVar17, uStack16, (uStack16 >> 0x10), 0x1, u_var11, uVar15,
                        uStack16, uStack16);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_a844(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: i16, param_5: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let piVar6: U32Ptr;
    let puVar7: u32;
    let extraout_dx: u16;
    let uVar9: u16;
    let u_var10: u16;
    let uVar8: &mut Struct426;
    let i_var8: &mut Struct427;
    let iVar11: i16;
    let uVar12: u16;
    let uVar13: u16;
    let puVar14: U32Ptr;
    let uVar15: u32;
    let uStack34: u32;
    let local_1c: i16;
    let local_1a: i16;
    let local_18: i16;
    let local_16: i16;
    let iStack20: i16;
    let uStack16: u16;
    let lStack14: i32;
    let uStack10: u32;
    let puStack6: u32;

    // uVar12 = (param_1 >> 0x10);
    uVar8 = param_1;
    pass1_1038_53ba(uVar8.field_0x4, 0x1);
    if ((param_4 != 0x0) || (param_3 != 0x0)) {
        uVar15 = uVar8.field_0x4;
        // uVar13 = (uVar15 >> 0x10);
        i_var8 = uVar15;
        puVar7 = i_var8.field_0xc;
        ppc_var3 = (*puVar7 + 0x10);
        puStack6 = puVar7;
        (**ppc_var3)(&ctx.PTR_LOOP_1050_1038, puVar7,
                    (&i_var8.field_0xc + 0x2));
        uStack10 = puVar7 & 0xffff | extraout_dx << 0x10;
        uVar15 = uVar8.field_0x4;
        lStack14 = (uVar15 + 0x8);
        uStack16 = 0x0;
        puVar14 = clear_struct_1008_3e38(CONCAT22(param_5, &local_16));
        // uVar9 = (puVar14 >> 0x10);
        i_var1 = (param_2 + 0x4);
//     for (uStack34 = 0x0; uStack34 < uStack10; uStack34 += 0x1) {
//       uVar15 = pass1_1030_1d7c(uStack10,uVar9,puStack6);
//       u_var4 = (uVar15 >> 0x10);
//       u_var10 = u_var4 | uVar15;
//       uVar9 = u_var10;
//       if ((u_var10 != 0x0) &&
//          (u_var4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,(uVar15 + 0xc),
//                                   0x46), uVar9 = u_var10, u_var4 != 0x0)) {
//         pass1_1030_1d58(puStack6);
//         uVar9 = u_var10 | u_var4;
//         if ((u_var10 | u_var4) != 0x0) {
//           pass1_1008_3f62(CONCAT22(param_5,&local_16),
//                           CONCAT22(u_var10,u_var4 + 0xc));
//           pass1_1008_3eb4(CONCAT22(param_5,&local_16),
//                           CONCAT22(param_5,&local_1c),
//                           CONCAT22(param_5,&local_1a),
//                           CONCAT22(param_5,&local_18));
//           uVar9 = u_var10;
//           if ((i_var1 == local_1c) &&
//              (uVar15 = uVar8.field_0x4, uVar13 = (uVar15 >> 0x10),
//              iVar11 = uVar15, u_var2 = (iVar11 + 0x4),
//              u_var5 = pass1_1030_addc(uVar8,uVar12,
//                                      CONCAT22(param_5,&local_16),u_var2,
//                                      (u_var2 >> 0x10),
//                                      (iVar11 + 0x8),&local_16,u_var10,
//                                      param_5), uVar9 = u_var10, u_var5 != 0x0)) {
//             iStack20 = local_1a + -0x1;
//             piVar6 = &local_16;
//             pass1_1030_ad86(uVar8,uVar12,CONCAT22(param_5,piVar6),
//                             lStack14,param_5,u_var10);
//             if (piVar6 != 0x0) {
// //LAB_1030_a98e:
//               pass1_1008_3f62(param_2,CONCAT22(param_5,&local_16));
//               return;
//             }
//             iStack20 = local_1a + 0x1;
//             piVar6 = &local_16;
//             pass1_1030_ad86(uVar8,uVar12,CONCAT22(param_5,piVar6),
//                             lStack14,param_5,u_var10);
//             if (piVar6 != 0x0) goto LAB_1030_a98e;
//             iStack20 = local_1a;
//             local_16 = local_18 + -0x1;
//             piVar6 = &local_16;
//             pass1_1030_ad86(uVar8,uVar12,CONCAT22(param_5,piVar6),
//                             lStack14,param_5,u_var10);
//             if (piVar6 != 0x0) goto LAB_1030_a98e;
//             local_16 = local_18 + 0x1;
//             piVar6 = &local_16;
//             pass1_1030_ad86(uVar8,uVar12,CONCAT22(param_5,piVar6),
//                             lStack14,param_5,u_var10);
//             uVar9 = u_var10;
//             if (piVar6 != 0x0) goto LAB_1030_a98e;
//           }
//         }
//       }
//     }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_aa18(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let BVar5: bool;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let iVar12: i16;
    let puVar13: u32;
    let uVar14: u16;
    let uVar15: u16;
    let uVar16: u16;
    let uVar17: u16;
    let puVar18: u32;
    let uVar19: u32;
    let uVar20: u8;
    let uStack38: u32;
    let local_1a: [u8; 2];
    let local_18: i16;
    let local_16: i16;
    let local_14: i16;
    let iStack18: i16;
    let uStack14: u32;
    let uStack10: u16;
    let puStack8: U32Ptr;
    let i_stack6: i16;
    let i_stack4: i16;

    i_stack4 = (param_2 + 0x4);
    i_stack6 = 0x8 - (i_stack4 == 0x0);
    puVar18 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, i_stack6);
    // puVar7 = (puVar18 >> 0x10);
    uVar8 = puVar18;
    // uVar14 = (param_1 >> 0x10);
    u_var10 = param_1;
    uStack10 = uVar8;
    puStack8 = puVar7;
    pass1_1038_4e78(uVar8, puVar7, (u_var10 + 0x4), puVar18);
    uStack14 = CONCAT22(puVar7, uVar8);
    uVar17 = 0x1008;
    clear_struct_1008_3e38(CONCAT22(param_3, &local_14));
    u_var3 = (u_var10 + 0x4);
    u_var1 = (u_var3 + 0x8);
    // uVar15 = (uStack14 >> 0x10);
    u_var11 = SUB42(uStack14, 0x0);
    ppcVar2 = (*uStack14 + 0x10);
    u_var6 = u_var1;
    (**ppcVar2)(0x1008, u_var11, uVar15);
    u_var6 = u_var6 & 0xffff | extraout_dx << 0x10;
    uStack38 = 0x0;
    uVar8 = extraout_dx;
    loop {
        if (u_var6 <= uStack38) {
            if (uStack14 != 0x0) {
                ppcVar2 = *uStack14;
                (**ppcVar2)(uVar17, uStack14, (uStack14 >> 0x10), 0x1, u_var11, uVar15, uStack14, uStack14);
            }
            return;
        }
        uVar19 = u_var6;
        pass1_1030_1d58(uStack14);
        uVar9 = uVar8 | uVar19;
        if (uVar9 != 0x0) { break; }
//LAB_1030_aadc:
        uStack38 += 0x1;
        uVar8 = uVar9;
    }
    uVar9 = uVar8;
    pass1_1008_3f62(CONCAT22(param_3, &local_14),
                    CONCAT22(uVar8, uVar19 + 0xc));
    uVar17 = 0x1008;
    pass1_1008_3eb4(CONCAT22(param_3, &local_14),
                    CONCAT22(param_3, local_1a),
                    CONCAT22(param_3, &local_18),
                    CONCAT22(param_3, &local_16));
    u_var3 = (u_var10 + 0x4);
    // uVar16 = (u_var3 >> 0x10);
    iVar12 = u_var3;
    u_var3 = (iVar12 + 0x4);
    u_var4 = pass1_1030_addc(u_var10, uVar14, CONCAT22(param_3, &local_14),
                            u_var3, (u_var3 >> 0x10),
                            (iVar12 + 0x8), &local_14, uVar9, param_3);
    if (u_var4 == 0x0) {
        // goto
        // LAB_1030_aadc;
    }
    uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | uVar8 << 0x10);
    // uVar9 = (uVar19 >> 0x10);
    iVar12 = (uVar19 + 0xc);
    if (0x5 < iVar12 - 0x7a) {
        // goto
        // LAB_1030_aadc;
    }
    uVar17 = 0x1030;
    switch(iVar12)
    {
        default: iStack18 = local_18 + -0x1;
        BVar5 = pass1_1030_acbe(u_var10, uVar14, CONCAT22(param_3, &local_14), u_var1,
                                &local_14, uVar9, param_3);
        if (BVar5 != 0x0) {
            // goto
            // LAB_1030_ac5b;
        }
        iStack18 = local_18 + 0x1;
        BVar5 = pass1_1030_acbe(u_var10, uVar14, CONCAT22(param_3, &local_14), u_var1,
                                &local_14, uVar9, param_3);
        if (BVar5 == 0x0) {
            iStack18 = local_18;
            local_14 = local_16 + -0x1;
            BVar5 = pass1_1030_acbe(u_var10, uVar14, CONCAT22(param_3, &local_14), u_var1,
                                    &local_14, uVar9, param_3);
//       TODO: goto joined_r0x1030ab9e;
        }
//LAB_1030_abc4:
        pass1_1008_3f62(param_2, CONCAT22(param_3, &local_14));
        break;
        0x7b => 0x7e => iStack18 = local_18 + -0x1;
        BVar5 = pass1_1030_acbe(u_var10, uVar14, CONCAT22(param_3, &local_14), u_var1,
                                &local_14, uVar9, param_3);
        if (BVar5 == 0x0) {
            iStack18 = local_18 + 0x1;
//       TODO: goto LAB_1030_abac;
        }
        pass1_1008_3f62(param_2, CONCAT22(param_3, &local_14));
        if (uStack14 == 0x0) {
            return;
        }
        // uVar17 = (uStack14 >> 0x10);
        puVar13 = uStack14;
        // uVar20 = (uStack14 >> 0x10);
//     TODO: goto LAB_1030_ab66;
        0x7c => 0x7d => local_14 = local_16 + -0x1;
        BVar5 = pass1_1030_acbe(u_var10, uVar14, CONCAT22(param_3, &local_14), u_var1,
                                &local_14, uVar9, param_3);
        joined_r0x1030ab9e: if (BVar5 == 0x0) {
        local_14 = local_16 + 0x1;
//LAB_1030_abac:
        BVar5 = pass1_1030_acbe(u_var10, uVar14, CONCAT22(param_3, &local_14), u_var1,
                                &local_14, uVar9, param_3);
        if (BVar5 != 0x0) {
            // goto
            // LAB_1030_abc4;
        }
//       TODO: goto LAB_1030_aadc;
    }
//LAB_1030_ac5b:
        pass1_1008_3f62(param_2, CONCAT22(param_3, &local_14));
    }
    puVar13 = uStack14;
    if ((uStack14._2_2_ | puVar13) != 0x0) {
        // uVar17 = (uStack14 >> 0x10);
        // uVar20 = (uStack14 >> 0x10);
//LAB_1030_ab66:
        ppcVar2 = *puVar13;
        (**ppcVar2)(0x1008, puVar13, uVar20, 0x1, u_var11, uVar15, uStack14, uStack14);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool
pass1_1030_acbe(param_1: u16,param_2: u16,param_3: U32Ptr,param_4: i32,param_5: u16,
param_6: u16,param_7: u16)

{
let i_var1: i16; let u_var2: u16;
let u_var3: u32;

pass1_1030_627e(param_7, param_5, param_6, _PTR_LOOP_1050_5740, param_3, param_4);
u_var2 = param_6 | param_5; if (u_var2 != 0x0) {
pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5, param_6); if ((u_var2 | param_5) != 0x0) {
u_var3 = struct_op_1030_73a8(CONCAT22(u_var2, param_5)); if ((u_var3 != 0x0) && ((i_var1 = (u_var3 + 0xc), i_var1 == 0x5 | | (i_var1 == 0x9)))) {
return 0x1;
}
}
}
return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool
pass1_1030_ad22(param_1: u16,param_2: u16,param_3: U32Ptr,param_4: i32,param_5: u16,
param_6: u16,param_7: u16)

{
let b_var1: bool; let u_var2: u16;
let u_var3: u32;

pass1_1030_627e(param_7, param_5, param_6, _PTR_LOOP_1050_5740, param_3, param_4);
u_var2 = param_6 | param_5; if (u_var2 != 0x0) {
pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5, param_6); if ((u_var2 | param_5) != 0x0) {
u_var3 = struct_op_1030_73a8(CONCAT22(u_var2, param_5)); if (u_var3 != 0x0) {
b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var3 + 0xc), 0x46
); return b_var1;
}
}
}
return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_ad86(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: i32, param_5: u16,
                       param_6: u16)

{
    let u_var1: u32;
    let pu_var2: u32;
    let cStack17: u8;
    let local_a: u32;
    let i_stack6: i16;

    pu_var2 = &local_a;
    pass1_1030_64ce(param_5, pu_var2, param_6, _PTR_LOOP_1050_5740, param_3, param_4,
                    CONCAT22(param_5, pu_var2));
    u_var1 = *pu_var2;
    cStack17 = (u_var1 >> 0x18);
    if (cStack17 == '\0') {
        i_stack6 = u_var1;
        if (((0x0 < i_stack6) && (!SBORROW2(i_stack6, 0x1))) && ((i_stack6 == 0x3 || i_stack6 + -0x2 < 0x1 || ((0x3 < i_stack6 + -0x3 && (i_stack6 + -0x7 < 0x2)))))) {
            return;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16
pass1_1030_addc(param_1: u16,param_2: u16,param_3: U32Ptr,param_4: u16,
param_5: u16,param_6: u32,param_7: i16,param_8: u16,param_9: u16)

{
let pu_var1: u32; let local_14: i16;
let local_12: i16; let local_10: i16;
let local_e: i16; let local_c: u32;
let uStack8: u16; let i_stack6: i16;
let uStack4: u16;

pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6, (param_6 > > 0x10)); i_stack6 = param_7; uStack4 = param_8; pu_var1 = pass1_1030_5b5c(param_7, param_8); local_c = * pu_var1; uStack8 = (pu_var1 + 0x4); pass1_1008_3e94(param_3, CONCAT22(param_9, &local_10),
CONCAT22(param_9, & local_e)); pass1_1008_3e94(CONCAT22(param_9, & local_c),
CONCAT22(param_9, & local_14),
CONCAT22(param_9, & local_12)); if ((((0x1 < local_e) & & (0x1 < local_10)) & & (local_e < local_12 + - 0x1)) & & (local_10 < local_14 + - 0x1)) {
return 0x1;
}
return 0x0;
}



pub fn pass1_1030_ae6c(param_1: U32Ptr) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let extraout_dx: u16;
    let i_var4: &mut Struct399;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x389a;
    i_var4.field_0x2 = 0x1008;
    i_var4.field_0x4 = 0x0;
    pu_var5 = clear_struct_1008_3e38(
        (param_1 & 0xffff0000 | &i_var4.field_0x8));
    // pu_var3 = (pu_var5 >> 0x10);
    u_var2 = 0x0;
    i_var4.field_0xe = 0x0;
    &i_var4.field_0x10 = 0x0;
    *param_1 = 0xb932;
    i_var4.field_0x2 = 0x1030;
    mem_op_1000_179c(0xc, pu_var3, 0x1000);
    if ((pu_var3 | u_var2) == 0x0) {
        &i_var4.field_0x10 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(pu_var3, u_var2));
        i_var4.field_0x10 = u_var2;
        i_var4.field_0x12 = extraout_dx;
    }
    u_var1 = &i_var4.field_0x10;
    (u_var1 + 0xa) = 0x0;
    return;
}


pub fn pass1_1030_aefa(param_1: U32Ptr, param_2: u32) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let extraout_dx: u16;
    let u_var4: u16;
    let iVar5: &mut Struct400;
    let u_var5: u16;
    let puVar6: U32Ptr;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x389a;
    iVar5.field_0x2 = 0x1008;
    iVar5.field_0x4 = 0x0;
    puVar6 = clear_struct_1008_3e38(
        (param_1 & 0xffff0000 | &iVar5.field_0x8));
    // pu_var3 = (puVar6 >> 0x10);
    iVar5.field_0xe = 0x0;
    &iVar5.field_0x10 = 0x0;
    *param_1 = 0xb932;
    iVar5.field_0x2 = 0x1030;
    iVar5.field_0x4 = (param_2 + 0x4);
    puVar6 = (param_1 & 0xffff0000 | &iVar5.field_0x8);
    pass1_1008_3f62(puVar6, (param_2 & 0xffff0000 | (param_2 + 0xc)));
    u_var2 = puVar6;
    mem_op_1000_179c(0xc, pu_var3, 0x1000);
    if ((pu_var3 | u_var2) == 0x0) {
        u_var2 = 0x0;
        u_var4 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(pu_var3, u_var2));
        u_var4 = extraout_dx;
    }
    iVar5.field_0x10 = u_var2;
    iVar5.field_0x12 = u_var4;
    u_var1 = &iVar5.field_0x10;
    (u_var1 + 0xa) = 0x0;
    return;
}


pub fn pass1_1030_afa6(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u32;
    let iVar5: &mut Struct614;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0xb932;
    iVar5.field_0x2 = 0x1030;
    if (&iVar5.field_0x10 != 0x0) {
        u_var4 = &iVar5.field_0x10;
        (u_var4 + 0xa) = 0x1;
    }
    pu_var1 = &iVar5.field_0x10;
    u_var2 = iVar5.field_0x12;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    *param_1 = 0x389a;
    iVar5.field_0x2 = 0x1008;
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_affc(param_1: u32, param_2: i16, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let uVar8: u32;
    let iStack12: i16;
    let uStack10: u32;
    let local_6: u32;

    uVar8 = ZEXT24(&local_6);
    pass1_1030_b718(param_1, param_1._2_2_,
                    (param_1 & 0xffff0000 | (param_1 + 0x8)),
                    CONCAT22(param_3, &local_6), param_1._2_2_, param_2,
                    param_3);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, local_6);
    uStack10 = uVar8 & 0xffff | param_1._2_2_ << 0x10;
    u_var5 = param_1._2_2_ | uVar8;
    if (u_var5 != 0x0) {
        uVar7 = struct_op_1030_73a8(uVar8 & 0xffff | param_1._2_2_ << 0x10);
        // u_var5 = (uVar7 >> 0x10);
        i_var1 = (uVar7 + 0xc);
        uVar8 = (i_var1 - 0x16);
        if ((0x15 < i_var1) && (!SBORROW2(i_var1, 0x16))) {
            uVar8 = (i_var1 - 0x17);
            if (i_var1 - 0x17 != 0x0 && 0x0 < (i_var1 - 0x16)) {
                uVar8 = (i_var1 - 0x19);
                if ((i_var1 + -0x18 < 0x1) || (uVar8 = (i_var1 - 0x1a),
                                              i_var1 - 0x1a != 0x0 && 0x0 < (i_var1 - 0x19))) {
                    // goto
                    // LAB_1030_b064;
                }
            }
            (uVar7 + 0x20) = 0x0;
        }
    }
//LAB_1030_b064:
    iStack12 = 0x6;
    loop {
        u_var3 = uVar8;
        if (iStack12 == 0x0) {
//LAB_1030_b0fc:
            if ((uStack10._2_2_ | uStack10) != 0x0) {
                uVar8 = struct_op_1030_73a8(uStack10);
                // u_var2 = (uVar8 >> 0x10);
                i_var1 = (uVar8 + 0xc);
                if (((0x15 < i_var1) && (!SBORROW2(i_var1, 0x16))) && ((i_var1 == 0x17 || i_var1 + -0x16 < 0x1 || ((0x0 < i_var1 + -0x18 && (i_var1 + -0x19 < 0x2)))))) {
                    (uVar8 + 0x20) = 0x1;
                }
            }
            return;
        }
        pass1_1030_b578(param_1, param_2, param_3);
        if ((u_var5 | u_var3) == 0x0) {
            // goto
            // LAB_1030_b0fc;
        }
        uStack10 = CONCAT22(u_var5, u_var3);
        uVar8 = struct_op_1030_73a8(CONCAT22(u_var5, u_var3));
        // u_var6 = (uVar8 >> 0x10);
        i_var1 = (uVar8 + 0xc);
        pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                        CONCAT22(u_var5, u_var3 + 0xc));
        if ((i_var1 == 0x18) || (u_var5 = u_var6, i_var1 == 0x3f)) {
            pass1_1030_b142(param_1, uStack10);
            u_var5 = u_var6;
        }
        BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var1, 0x40);
        uVar8 = BVar4;
        if (BVar4 != 0x0) {
            pass1_1030_b454(param_1, uStack10, param_3);
//       TODO: goto LAB_1030_b0fc;
        }
        iStack12 += -0x1;
    }
}


pub fn pass1_1030_b13c() {
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_b142(param_1: u32, param_2: u32) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;
    let bVar4: bool;
    let u_var5: u32;
    let uStack12: u32;

    u_var5 = struct_op_1030_73a8(param_2);
    // u_var3 = (u_var5 >> 0x10);
    i_var1 = u_var5;
    i_var2 = (i_var1 + 0xc);
    uStack12 = 0x0;
    if (i_var2 == 0x18) {
        uStack12._2_2_ = pass1_1028_1c1c();
        u_var3 = (i_var1 + 0x22);
    } else {
        if (i_var2 != 0x3f) {
            // goto
            // LAB_1030_b1a6;
        }
        uStack12._2_2_ = pass1_1028_20b0();
        u_var3 = (i_var1 + 0x24);
    }
    uStack12 = CONCAT22(uStack12._2_2_, u_var3);
//LAB_1030_b1a6:
    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xe) == 0x1) {
        bVar4 = (uStack12 & 0x10000) == 0x0;
    } else {
        if ((i_var2 + 0xe) == 0x2) {
            bVar4 = (uStack12 & 0x20000) == 0x0;
        } else {
            if ((i_var2 + 0xe) == 0x3) {
                bVar4 = (uStack12 & 0x40000) == 0x0;
            } else {
                bVar4 = (uStack12 & 0x80000) == 0x0;
            }
        }
    }
    if ((bVar4) || (uStack12 != 0x0)) {
        bVar4 = false;
        loop {
            if (((uStack12 & 0x10000) != 0x0) && (uStack12 == 0x0)) {
                // goto
                // LAB_1030_b239;
            }
            if (((uStack12 & 0x20000) != 0x0) && (uStack12 == 0x0)) {
                // goto
                // LAB_1030_b247;
            }
            if (((uStack12 & 0x40000) != 0x0) && (uStack12 == 0x0)) {
                // goto
                // LAB_1030_b255;
            }
            if (((uStack12 & 0x80000) != 0x0) && (uStack12 == 0x0)) {
                // goto
                // LAB_1030_b263;
            }
            if (bVar4) { break; }
            uStack12._1_3_ = (uStack12 >> 0x8) & 0xffff00;
            i_var1 = (i_var2 + 0xe);
            if (i_var1 == 0x1) {
                uStack12 = CONCAT31(uStack12._1_3_, 0x4);
            } else {
                if (i_var1 == 0x2) {
                    uStack12 = CONCAT31(uStack12._1_3_, 0x8);
                } else {
                    if (i_var1 == 0x3) {
                        uStack12 = CONCAT31(uStack12._1_3_, 0x1);
                    } else {
                        uStack12 = CONCAT31(uStack12._1_3_, 0x2);
                    }
                }
            }
            bVar4 = true;
        }
        if ((i_var2 + 0xe) == 0x1) {
//LAB_1030_b255:
            (i_var2 + 0xe) = 0x3;
            return;
        }
        if ((i_var2 + 0xe) == 0x2) {
//LAB_1030_b263:
            (i_var2 + 0xe) = 0x4;
            return;
        }
        if ((i_var2 + 0xe) == 0x3) {
//LAB_1030_b239:
            (i_var2 + 0xe) = 0x1;
            return;
        }
        if ((i_var2 + 0xe) == 0x4) {
//LAB_1030_b247:
            (i_var2 + 0xe) = 0x2;
            return;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_b2aa(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u32;
    let bStack23: u8;
    let local_6: u32;

    pass1_1030_b718(param_1, (param_1 >> 0x10), param_2,
                    CONCAT22(param_5, &local_6), param_3, param_4, param_5);
    bStack23 = (local_6 >> 0x18);
    u_var1 = bStack23;
    if (bStack23 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, local_6);
        if ((local_6._2_2_ | u_var1) != 0x0) {
            u_var3 = struct_op_1030_73a8(CONCAT22(local_6._2_2_, u_var1));
            BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var3 + 0xc), 0x42);
            if (BVar2 != 0x0) {
                pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                                CONCAT22(local_6._2_2_, u_var1 + 0xc));
                return;
            }
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_b344(param_1: u32, param_2: u16) -> u32

{
    let pu_var1: U32Ptr;
    let puStack18: u32;
    let puStack16: U32Ptr;
    let local_e: [u8; 2];
    let local_c: i16;
    let local_a: i16;
    let local_8: u32;
    let uStack4: u16;

    local_8 = (param_1 + 0x8);
    uStack4 = (param_1 + 0xc);
    pu_var1 = param_1._2_2_;
    pass1_1008_3eb4(CONCAT22(param_2, &local_8), CONCAT22(param_2, local_e), CONCAT22(param_2, &local_c),
                    CONCAT22(param_2, &local_a));
    local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(param_1, CONCAT22(param_2, puStack18), pu_var1, &stack0xfffe,
                    param_2);
    puStack16 = (pu_var1 | puStack18);
    if (puStack16 == 0x0) {
        local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
        puStack18 = &local_8;
        pass1_1030_b2aa(param_1, CONCAT22(param_2, puStack18), 0x0,
                        &stack0xfffe, param_2);
        pu_var1 = (puStack16 | puStack18);
        if (pu_var1 == 0x0) {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            puStack18 = &local_8;
            pass1_1030_b2aa(param_1, CONCAT22(param_2, puStack18), 0x0,
                            &stack0xfffe, param_2);
            puStack16 = (pu_var1 | puStack18);
            if (puStack16 == 0x0) {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                puStack18 = &local_8;
                pass1_1030_b2aa(param_1, CONCAT22(param_2, puStack18), 0x0,
                                &stack0xfffe, param_2);
                if ((puStack16 | puStack18) == 0x0) {
                    return 0x0;
                }
                (param_1 + 0xe) = 0x2;
            } else {
                (param_1 + 0xe) = 0x4;
                puStack16 = pu_var1;
            }
        } else {
            (param_1 + 0xe) = 0x3;
        }
    } else {
        (param_1 + 0xe) = 0x1;
        puStack16 = pu_var1;
    }
    return CONCAT22(puStack16, puStack18);
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_b454(param_1: u32, param_2: u32, param_3: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let extraout_dx: u16;
    let i_var4: i16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u32;
    let uVar9: u32;
    let lStack38: i32;
    let uStack30: u32;
    let local_12: [u8; 4];
    let uStack14: u32;
    let uStack10: u32;
    let lStack6: i32;

    lStack6 = (param_2 + 0x4);
    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(param_3, local_12), (iVar6 + 0x10));
    loop {
        pu_var3 = local_12;
        pass1_1008_5b12(pu_var3, param_3);
        uStack10 = CONCAT22(extraout_dx, pu_var3);
        if ((extraout_dx | pu_var3) == 0x0) { break; }
        if ((pu_var3 + 0x20) == lStack6) {
            ppcVar2 = ((iVar6 + 0x10) + 0xc);
            (**ppcVar2)();
            uStack14 = 0x0;
            pass1_1038_69fe(uStack10);
        }
    }
    uVar8 = struct_op_1030_73a8(param_2);
    // i_var4 = (uVar8 >> 0x10);
    pu_var1 = (uVar8 + 0x20);
    pu_var3 = local_12;
    pass1_1008_5784(CONCAT22(param_3, pu_var3), pu_var1);
    pass1_1030_b13c();
    uStack30 = CONCAT22(-(
        (s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2a) < pu_var3) - i_var4, 0x1f4 - pu_var3);
    loop {
        pu_var3 = local_12;
        pass1_1008_5b12(pu_var3, param_3);
        uStack10 = CONCAT22(extraout_DX_00, pu_var3);
        u_var5 = extraout_DX_00 | pu_var3;
        if (u_var5 == 0x0) {
            return;
        }
        pass1_1038_6984(CONCAT22(extraout_DX_00, pu_var3));
        lStack38 = CONCAT22(u_var5, pu_var3);
        if ((u_var5 <= uStack30._2_2_) && ((u_var5 < uStack30._2_2_ || (pu_var3 <= uStack30)))) {
            uVar9 = (iVar6 + 0x10);
            ppcVar2 = ((iVar6 + 0x10) + 0x8);
            (**ppcVar2)();
            uStack30 -= lStack38;
            ppcVar2 = (*pu_var1 + 0xc);
            (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, pu_var1, (pu_var1 >> 0x10),
                        uStack10, uVar9);
            uStack14 = 0x0;
        }
        if (0x0 < uStack30) == false { break; }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_b578(param_1: u32, param_2: i16, param_3: u16) {
    let i_var1: i16;
    let pu_var2: u32;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let bVar5: bool;
    let u_var6: u32;
    let uStack48: u32;
    let local_1c: [u8; 2];
    let local_1a: i16;
    let local_18: i16;
    let local_16: u32;
    let uStack18: u16;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u16;
    let uStack8: u16;
    let local_6: u32;

    pass1_1030_b718(param_1, param_1._2_2_,
                    (param_1 & 0xffff0000 | (param_1 + 0x8)),
                    CONCAT22(param_3, &local_6), param_1._2_2_, param_2, param_3);
    uStack48._3_1_ = (local_6 >> 0x18);
    uStack10 = uStack48._3_1_;
    if (uStack48._3_1_ == 0x0) {
        return;
    }
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, local_6);
    uStack8 = local_6._2_2_;
    uStack14 = struct_op_1030_73a8(CONCAT22(local_6._2_2_, uStack10));
    uStack16 = (uStack14 + 0xc);
    local_16 = (param_1 + 0x8);
    uStack18 = (param_1 + 0xc);
    puVar4 = param_1._2_2_;
    pass1_1008_3eb4(CONCAT22(param_3, &local_16),
                    CONCAT22(param_3, local_1c),
                    CONCAT22(param_3, &local_1a),
                    CONCAT22(param_3, &local_18));
    i_var1 = (param_1 + 0xe);
    if (i_var1 == 0x0) {
        pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10, param_3);
        return;
    }
    if (i_var1 == 0x1) {
        u_var3 = local_1a - 0x1;
//LAB_1030_b63e:
        local_16 = local_16 & 0xffff | u_var3 << 0x10;
        pu_var2 = &local_16;
        pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,
                        CONCAT22(param_3, pu_var2), puVar4, &uStack16, param_3);
        uStack48 = CONCAT22(puVar4, pu_var2);
        if ((puVar4 | pu_var2) == 0x0) {
            return;
        }
        u_var6 = struct_op_1030_73a8(CONCAT22(puVar4, pu_var2));
        u_var3 = (u_var6 + 0xc);
        if (u_var3 == 0x3f) {
            // goto
            // LAB_1030_b6e0;
        }
        if (0x3f < u_var3) {
            return;
        }
        if (u_var3 == '\x16') {
            // goto
            // LAB_1030_b6e0;
        }
        bVar5 = u_var3 == '\x18';
    } else {
        if (i_var1 == 0x2) {
            u_var3 = local_18 + 0x1;
        } else {
            if (i_var1 == 0x3) {
                u_var3 = local_1a + 0x1;
//         TODO: goto LAB_1030_b63e;
            }
            if (i_var1 != 0x4) {
                return;
            }
            u_var3 = local_18 - 0x1;
        }
        local_16 = local_16 & 0xffff0000 | u_var3;
        pu_var2 = &local_16;
        pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,
                        CONCAT22(param_3, pu_var2), puVar4, &uStack16, param_3);
        uStack48 = CONCAT22(puVar4, pu_var2);
        if ((puVar4 | pu_var2) == 0x0) {
            return;
        }
        u_var6 = struct_op_1030_73a8(CONCAT22(puVar4, pu_var2));
        i_var1 = (u_var6 + 0xc);
        if (i_var1 < 0x17) {
            return;
        }
        if (SBORROW2(i_var1, 0x17)) {
            return;
        }
        if (i_var1 == 0x18 || i_var1 + -0x17 < 0x1) {
            // goto
            // LAB_1030_b6e0;
        }
        bVar5 = i_var1 == 0x3f;
    }
    if (!bVar5) {
        return;
    }
//LAB_1030_b6e0:
    pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                    (uStack48 & 0xffff0000 | (uStack48 + 0xc)));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_b718(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: U32Ptr,
                       param_5: U32Ptr, param_6: i16, param_7: u16)

{
    let pu_var1: u32;
    let u_var2: u16;
    let local_12: [u32; 0x2];
    let lStack10: i32;
    let puStack6: U32Ptr;

    puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    // u_var2 = (puStack6 >> 0x10);
    lStack10 = (puStack6 + 0x20);
    pu_var1 = local_12;
    pass1_1030_64ce(param_7, pu_var1, u_var2, _PTR_LOOP_1050_5740, param_3, lStack10,
                    CONCAT22(param_7, pu_var1));
    *param_4 = *pu_var1;
    return;
}


pub fn pass1_1030_b768(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u32;
    let BVar2: bool;
    let i_var3: i16;
    let puVar4: U32Ptr;
    let extraout_dx: u16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let local_22: [u16; 0x4];
    u8
    local_1a[0xa];
    let local_10: u32;
    let puStack12: U32Ptr;
    let uStack10: u16;
    let local_8: [u16; 0x3];

    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    local_10 = (iVar5 + 0x4);
    uVar7 = param_2;
    // uVar8 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar7, uVar8, &local_10, param_3, 0x4, 0x1008);
    if ((BVar2 != 0x0) && (i_var3 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | (iVar5 + 0x8), 0x1008,
                                                           param_3), i_var3 != 0x0)) {
        local_8[0] = (iVar5 + 0xe);
        BVar2 = write_to_file_1008_7e1c(uVar7, uVar8, local_8, param_3, 0x2, 0x1008);
        if (BVar2 != 0x0) {
            u_var1 = (iVar5 + 0x10);
            local_22[0] = (u_var1 + 0x8);
            local_10 = local_10 & 0xffff0000 | local_22[0];
            BVar2 = write_to_file_1008_7e1c(uVar7, uVar8, local_22, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                return;
            }
            pass1_1008_5784(CONCAT22(param_3, local_1a), (iVar5 + 0x10));
            loop {
                puVar4 = local_1a;
                pass1_1008_5b12(puVar4, param_3);
                if ((extraout_dx | puVar4) == 0x0) {
                    return;
                }
                puStack12 = puVar4;
                uStack10 = extraout_dx;
                pass1_1038_75ca(CONCAT22(extraout_dx, puVar4), param_2, puVar4, param_3);
                if (puVar4 != 0x0) == false { break; }
            }
            return;
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}


pub fn file_1030_b836(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let i_var4: &mut Struct401;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let extraout_dx: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u32;
    let uVar13: u16;
    let local_12: [u16; 0x7];
    let local_4: u16;

    i_var4 = param_1;
    i_var4 = &i_var4.field_0x4;
    // u_var3 = (param_1 >> 0x10);
    uVar9 = param_2;
    // u_var10 = (param_2 >> 0x10);
    BVar4 = read_file_1008_7dee(uVar9, u_var10, i_var4, 0x0, u_var3, 0x4, 0x1008);
    if (((BVar4 == 0x0) || (BVar4 = read_file_1008_7bc8(param_2,
                                                        (param_1 & 0xffff0000 | &i_var4.field_0x8), 0x1008, param_4), BVar4 == 0x0)) || (BVar4 = read_file_1008_7dee(uVar9, u_var10, &local_4, 0x0, param_4, 0x2, 0x1008),
                                                                                                                                        BVar4 == 0x0)) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    } else {
        i_var4.field_0xe = local_4;
        BVar4 = read_file_1008_7dee(uVar9, u_var10, local_12, 0x0, param_4, 0x2, 0x1008);
        if (BVar4 != 0x0) {
            loop {
                if (local_12[0] == 0x0) {
                    return;
                }
                u_var11 = 0x2a;
                u_var5 = local_12[0];
                local_12[0] = local_12[0] - 0x1;
                uVar12 = param_2;
                mem_op_1000_179c(0x2a, param_3, 0x1000);
                puVar8 = (param_3 | u_var5);
                if (puVar8 == 0x0) {
                    u_var6 = 0x0;
                    puVar8 = 0x0;
                } else {
                    u_var6 = u_var5;
                    struct_1038_6520(CONCAT22(param_3, u_var5));
                }
                // uVar13 = (uVar12 >> 0x10);
                uVar7 = u_var6;
                file_1038_774e(CONCAT22(puVar8, u_var6), CONCAT22(uVar12, u_var11), puVar8, param_4);
                if (uVar7 == 0x0) { break; }
                pu_var1 = i_var4.field_0x10;
                ppcVar2 = (*i_var4.field_0x10 + 0x4);
                (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, pu_var1, (pu_var1 >> 0x10),
                            u_var6, puVar8, uVar13, u_var5);
                param_3 = extraout_dx;
            }
        }
    }
    return;
}



astruct_18 *  pass1_1030_b90c(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_afa6(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_b936(param_1: &mut Struct365, param_2: u16, param_3: u16, param_4: u32,
                       param_5: u16)

{
    pass1_1028_b22c(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    CONCAT22(param_2, param_1) = 0xbc0c;
    param_1.field_0x2 = 0x1030;
    return;
}


pub fn pass1_1030_b96c(param_1: U32Ptr) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0xbc0c;
    (i_var3 + 0x2) = 0x1030;
    paVar2 = (i_var3 + 0xe);
    u_var1 = (i_var3 + 0x10);
    if ((u_var1 | paVar2) != 0x0) {
        fn_ptr_1020_ba7e((paVar2 & 0xffff | u_var1 << 0x10));
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    }
    pass1_1028_b260(param_1);
    return;
}


pub fn pass1_1030_b9b2(param_1: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0xe) = 0x0;
    (param_1 + 0x12) = 0x0;
    return;
}


pub fn pass1_1030_b9da(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: u16,
                       param_6: u16, param_7: u16)

{
    let pu_var1: u32;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let iVar7: &mut Struct402;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u32;
    let uStack12: u16;
    let uStack4: u16;

    pu_var3 = param_3;
    // uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    if (iVar7.field_0xe == 0x0) {
        mem_op_1000_179c(0xa, pu_var3, 0x1000);
        u_var4 = pu_var3 | param_4;
        param_3 = u_var4;
        if (u_var4 == 0x0) {
            iVar7.field_0xe = 0x0;
        } else {
            pass1_1020_ba3e((param_4 & 0xffff | ZEXT24(pu_var3) << 0x10), 0x5, 0x5, param_6,
                            param_5);
            &iVar7.field_0xe = param_4;
            (&iVar7.field_0xe + 0x2) = param_3;
        }
        iVar7.field_0x12 = 0x0;
    }
    // for (uStack4 = 0x4; uStack4 < 0xe; uStack4 += 0x1) {
    //   u_var10 = pass1_1030_7c28(param_2,uStack4,param_4,param_3,param_7);
    //   u_var4 = (u_var10 >> 0x10);
    //   param_4 = u_var10 & 0xffff;
    //   u_var5 = u_var4 | param_4;
    //   param_3 = u_var5;
    //   if (u_var5 != 0x0) {
    //     u_var2 = 0x64 - iVar7.field_0x12;
    //     u_var6 = u_var2 >> 0x10;
    //     uStack12 = u_var10;
    //     if (u_var10 < u_var2) {
    //       u_var2 = u_var10 & 0xffff;
    //       u_var6 = u_var4;
    //     }
    //     u_var5 = u_var2;
    //     param_4 = u_var2 & 0xffff | u_var6 << 0x10;
    //     i_var8 = (u_var4 - u_var6) - (uStack12 < u_var5);
    //     param_3 = u_var6;
    //     pass1_1030_7d1c(param_2,uStack12 - u_var5,CONCAT22(uStack4,i_var8),u_var5,u_var6,
    //                     i_var8,param_6,param_7);
    //     pass1_1020_bb8a(iVar7.field_0xe,u_var5,u_var6 | uStack4 << 0x10,param_6,
    //                     param_7);
    //     pu_var1 = &iVar7.field_0x12;
    //     *pu_var1 = *pu_var1 + param_4;
    //     string_1020_c0ca(uStack4);
    //     vsprintf_op_1030_840a
    //               (s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c,0x1020,param_7,
    //                param_3);
    //     if (0x63 < iVar7.field_0x12) break;
    //   }
    // }
    if (iVar7.field_0x12 != 0x0) {
        return;
    }
    return;
}


pub fn pass1_1030_bb0e(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let paVar1: &mut Struct18;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let in_DX: u16;
    let pu_var5: U32Ptr;
    let u_var6: u32;
    let uStack8: u16;

    u_var3 = pass1_1030_7bee(param_2);
    u_var4 = u_var3;
    if (u_var3 != 0x0) {
        return;
    }
    pass1_1030_b9b2(param_1);
    u_var2 = u_var4 & 0xffff;
    paVar1 = (u_var2 | in_DX << 0x10);
    pu_var5 = (in_DX | u_var4);
    if (pu_var5 != 0x0) {
        // for (uStack8 = 0x4; uStack8 < 0x25; uStack8 += 0x1) {
        //   u_var6 = pass1_1020_bae6(u_var2,CONCAT22(uStack8,in_DX),u_var4,
        //                           pu_var5,param_6);
        //   u_var4 = u_var6 & 0xffff;
        //   pu_var5 = ((u_var6 >> 0x10) | u_var4);
        //   if (pu_var5 != 0x0) {
        //     pass1_1030_7ddc(param_2,u_var6,uStack8,u_var4,pu_var5,param_4,param_5,param_6);
        //     u_var3 = pass1_1030_7bee(param_2);
        //     u_var4 = u_var3;
        //     if (u_var3 != 0x0) {
        //       return;
        //     }
        //     string_1020_c0ca(uStack8);
        //     vsprintf_op_1030_840a
        //               (s_truck_0x_08lx_unloaded__ld_of__s_1050_5798,0x1020,param_6,
        //                pu_var5);
        //     pass1_1020_bb8a(paVar1,0x0,uStack8 << 0x10,param_5,param_6);
        //   }
        // }
        if (paVar1 != 0x0) {
            fn_ptr_1020_ba7e(paVar1);
            fn_ptr_1000_17ce(ctx, paVar1, 0x1000);
        }
    }
    return;
}



astruct_18 *  pass1_1030_bbe6(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_b96c(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_bc24(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u32) -> u16

{
    pass1_1028_b22c(CONCAT22(param_3, param_2), param_4, param_5, param_1);
    CONCAT22(param_3, param_2) = 0xbc96;
    (param_2 + 0x2) = 0x1030;
    return CONCAT22(param_3, param_2);
}


pub fn pass1_1030_bc4e(param_1: U32Ptr) {
    *param_1 = 0xbc96;
    (param_1 + 0x2) = 0x1030;
    pass1_1028_b260(param_1);
    return;
}



astruct_18 *  pass1_1030_bc70(param_1: & mut Struct18,param_2: u8)

{
pass1_1030_bc4e(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_bcae(param_1: u16, param_2: u16) -> u32

{
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1030_bcbc(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16,
                       param_6: u32)

{
    pass1_1030_bcde(param_1, param_2, param_3,
                    CONCAT22(param_4, param_3._2_2_),
                    CONCAT22(param_5, param_4._2_2_), (param_6 + 0x4));
    return;
}


pub fn pass1_1030_bcde(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: U32Ptr, param_6: i32)

{
    let i_var1: i16;
    let u_var2: u16;
    let local_14: i16;
    let local_12: i16;
    let local_10: i16;
    let local_e: i16;
    let local_c: u32;
    let uStack8: u16;
    let lStack6: i32;

    // u_var2 = (param_4 >> 0x10);
    i_var1 = param_4;
    lStack6 = (i_var1 + 0x8);
    if (lStack6 != param_6) {
        return;
    }
    local_c = (i_var1 + 0xc);
    uStack8 = (i_var1 + 0x10);
    pass1_1008_3e94(param_5, CONCAT22(param_1, &local_10),
                    CONCAT22(param_1, &local_e));
    pass1_1008_3e94(CONCAT22(param_1, &local_c),
                    CONCAT22(param_1, &local_14),
                    CONCAT22(param_1, &local_12));
    pass1_1000_49b2(local_e - local_12);
    pass1_1000_49b2(local_10 - local_14);
    return;
}


pub fn pass1_1030_bd74(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16) {
    let i_var1: &mut Struct670;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    let local_1e: i16;
    let local_1c: i16;
    let local_1a: i16;
    let local_18: i16;
    let local_16: u32;
    let uStack18: u16;
    let local_10: u32;
    let uStack12: u16;
    let lStack10: i32;
    let lStack6: i32;

    // u_var3 = (param_4 >> 0x10);
    i_var1 = param_4;
    lStack6 = i_var1.field_0x8;
    // u_var4 = (param_3 >> 0x10);
    i_var2 = param_3;
    lStack10 = (i_var2 + 0x8);
    if (lStack10 != lStack6) {
        return;
    }
    local_10 = i_var1.field_0xc;
    uStack12 = i_var1.field_0x10;
    local_16 = (i_var2 + 0xc);
    uStack18 = (i_var2 + 0x10);
    pass1_1008_3e94(CONCAT22(param_5, &local_10),
                    CONCAT22(param_5, &local_1a),
                    CONCAT22(param_5, &local_18));
    pass1_1008_3e94(CONCAT22(param_5, &local_16),
                    CONCAT22(param_5, &local_1e),
                    CONCAT22(param_5, &local_1c));
    pass1_1000_49b2(local_18 - local_1c);
    pass1_1000_49b2(local_1a - local_1e);
    return;
}


pub fn struct_1030_be34(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0xc006;
    (param_1 + 0x2) = 0x1030;
    return param_1;
}


pub fn pass1_1030_be56(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) -> u16

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0xc006;
    (param_1 + 0x2) = 0x1030;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1030_be80(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let Bvar4: bool;
    let u_var5: u32;
    let extraout_dx: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let i_var9: i16;

    pass1_1028_bf22(param_1, param_2, param_3);
    // uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if ((iVar7 + 0x12) == 0x5) {
        u_var2 = (iVar7 + 0x14);
        (u_var2 + 0xa4) = 0x1e;
        u_var2 = (iVar7 + 0x14);
        (u_var2 + 0xac) = 0x1;
        i_var9 = (iVar7 + 0xc);
        i_var3 = i_var9 + -0x1b;
        if (i_var3 == 0x0) {
            u_var2 = (iVar7 + 0x14);
            (u_var2 + 0xaa) = 0xa;
        } else {
            i_var3 = i_var9 + -0x1c;
            if (i_var3 == 0x0) {
                u_var2 = (iVar7 + 0x14);
                (u_var2 + 0xaa) = 0xb;
            } else {
                i_var3 = i_var9 + -0x1d;
                if (i_var3 == 0x0) {
                    u_var2 = (iVar7 + 0x14);
                    (u_var2 + 0xaa) = 0xc;
                }
            }
        }
        pass1_1028_b58e(param_1);
        u_var5 = (i_var3 + 0x2e);
        i_var9 = 0xc;
        u_var6 = extraout_dx;
        pass1_1038_3fb0(u_var5);
        BVar4 = pass1_1030_25b2(u_var5 & 0xffff | u_var6 << 0x10, i_var9);
        if (BVar4 != 0x0) {
            u_var2 = (iVar7 + 0x14);
            pi_var1 = (u_var2 + 0xaa);
            *pi_var1 = *pi_var1 + 0x1;
        }
        BVar4 = pass1_1030_25b2(u_var5 & 0xffff | u_var6 << 0x10, 0xe);
        if (BVar4 != 0x0) {
            u_var2 = (iVar7 + 0x14);
            pi_var1 = (u_var2 + 0xaa);
            *pi_var1 = *pi_var1 + 0x1;
        }
        BVar4 = pass1_1030_25b2(u_var5 & 0xffff | u_var6 << 0x10, 0x76);
        if (BVar4 != 0x0) {
            u_var2 = (iVar7 + 0x14);
            pi_var1 = (u_var2 + 0xaa);
            *pi_var1 = *pi_var1 + 0x1;
        }
    }
    return;
}


pub fn pass1_1030_bf6e(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u32;
    let uVar8: u32;
    let uVar9: u16;

    uVar9 = 0x1e;
    uVar7 = pass1_1028_b58e(param_1);
    uVar8 = pass1_1030_7c28(uVar7, uVar9, uVar7, (uVar7 >> 0x10), param_4);
    u_var4 = 0x3e8 - uVar8;
    u_var2 = (param_1 + 0x14);
    // u_var6 = (u_var2 >> 0x10);
    iVar5 = u_var2;
    pu_var1 = (iVar5 + 0xaa);
    u_var3 = -(u_var4 < *pu_var1);
    pass1_1030_7ddc(uVar7, ((u_var4 - *pu_var1 & u_var3) + (iVar5 + 0xaa)), 0x1e,
                    u_var3, (uVar8 >> 0x10), param_2, param_3, param_4);
    return;
}


pub fn pass1_1030_bfb8(param_1: u32, param_2: u16) -> u16

{
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;

    u_var3 = 0x1e;
    u_var1 = pass1_1028_b58e(param_1);
    u_var2 = pass1_1030_7c28(u_var1, u_var3, u_var1, (u_var1 >> 0x10), param_2);
    return 0x3e8 - u_var2;
}



astruct_18 *  pass1_1030_bfe0(param_1: & mut Struct18,param_2: u8)

{
pass1_1028_b418(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_c09c(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    (param_1 + 0x24) = 0x0;
    CONCAT22(param_2, param_1) = 0xc68e;
    (param_1 + 0x2) = 0x1030;
    return;
}


pub fn pass1_1030_c0d2(param_1: u32) -> u16

{
    if (0x0 < (param_1 + 0x24)) {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1030_c0ec(param_1: u32) -> u16

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if (((param_1 + 0xc) != 0xb) && ((param_1 + 0x24) < 0x1)) {
        return 0x0;
    }
    return 0x1;
}


pub fn pass1_1030_c10e(param_1: u32) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (0x0 < (i_var2 + 0x24)) {
        pi_var1 = (i_var2 + 0x24);
        *pi_var1 = *pi_var1 + -0x1;
        return;
    }
    (i_var2 + 0xc) = 0x37;
    return;
}


pub fn pass1_1030_c12e(param_1: u32, param_2: i16, param_3: i16, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let extraout_dx: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uStack6: u32;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_3);
    u_var2 = (param_3 + 0x2e);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    i_var3 = u_var2;
    if ((i_var4 + 0x24) < 0x1) {
        pass1_1030_7d1c(uStack6, 0x0, 0x230000, i_var3, extraout_dx, param_4, param_5, param_6);
    } else {
        if (param_2 == 0x0) {
            u_var6 = 0x0;
        } else {
            u_var6 = 0x32;
        }
        pass1_1030_7d1c(uStack6, u_var6, 0x230000, i_var3, extraout_dx, param_4, param_5, param_6);
        pi_var1 = (i_var4 + 0x24);
        *pi_var1 = *pi_var1 + -0x1;
    }
    if ((0x0 < (i_var4 + 0x24)) && ((i_var4 + 0x24) < 0x19)) {
        (i_var3 + 0x1fe) = 0x1;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_c1b2(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: u16, param_5: u16,
)

{
    let i_var1: i16;
    let i_var2: &mut Struct695;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    pass1_1028_be9e(param_1, param_3, param_4, &USHORT_1050_1028, param_5);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2.field_0x12 == 0x5) {
        if (i_var2.field_0xc == 0xb) {
            pass1_1030_c652(param_2, param_4, param_5);
            i_var1 = 0x82;
            pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_5, param_2, param_4);
            pass1_1010_9f8c(pu_var3, i_var1, param_5);
            i_var2.field_0x24 = pu_var3 * 0x3;
            mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_5, (pu_var3 >> 0x10),
                            param_4);
            if (ctx.PTR_LOOP_1050_13ae < 0x3) {
                i_var1 = i_var2.field_0x24;
                if (i_var1 < 0x32) {
                    i_var1 = 0x32;
                }
                i_var2.field_0x24 = i_var1;
                return;
            }
        } else {
            i_var2.field_0x24 = 0x64;
        }
    }
    return;
}


pub fn pass1_1030_c230(param_1: u32, param_2: u32, param_3: u16) {
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let local_10: [u32; 0x2];
    let local_8: [u16; 0x3];

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (b_var1 != 0x0) {
        // u_var2 = (param_1 >> 0x10);
        local_10[0] = (param_1 + 0x20);
        // u_var3 = (param_2 >> 0x10);
        b_var1 = write_to_file_1008_7e1c(param_2, u_var3, local_10, param_3, 0x4, 0x1008);
        if (b_var1 != 0x0) {
            local_8[0] = (param_1 + 0x24);
            b_var1 = write_to_file_1008_7e1c(param_2, u_var3, local_8, param_3, 0x2, 0x1008);
            if (b_var1 != 0x0) {
                return;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


pub fn pass1_1030_c29c(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 != 0x0) {
        // u_var1 = (param_1 >> 0x10);
        // u_var3 = (param_2 >> 0x10);
        BVar2 = read_file_1008_7dee(param_2, u_var3, param_1 + 0x20, 0x0, u_var1, 0x4,
                                    0x1008);
        if (BVar2 != 0x0) {
            BVar2 = read_file_1008_7dee(param_2, u_var3, param_1 + 0x24, 0x0, u_var1, 0x2,
                                        0x1008);
            if (BVar2 != 0x0) {
                return;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_c2fa(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: u32;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u32;
    let puVar9: U32Ptr;
    let u_var10: u32;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let puVar14: U32Ptr;
    let uVar15: u16;
    let uStack84: u16;
    let lStack80: i32;
    let iStack56: i16;
    let uStack10: u32;
    let uStack6: u32;
    let iVar5: &mut Struct698;

    // uVar12 = (param_1 >> 0x10);
    if ((param_1 + 0xc) != 0xb) {
        pass1_1028_bd38(param_1, param_3, param_5);
        u_var1 = (param_1 + 0x20);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
        uStack6 = CONCAT22(param_3, param_2);
        iVar6 = param_2;
        puVar9 = param_3;
        pass1_1028_b58e(param_1);
        uStack10 = CONCAT22(puVar9, iVar6);
        u_var2 = (iVar6 + 0x2e);
        puVar14 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_5, puVar9, param_4);
        u_var10 = puVar14 >> 0x10;
        // uVar13 = (u_var2 >> 0x10);
        pass1_1010_ed22(puVar14, (u_var2 + 0x4), param_5);
        u_var3 = (param_2 + 0x1f6);
        uVar8 = u_var3;
        pass1_1030_3694(u_var3, 0x3, 0x2, u_var10, 0x1010, param_5);
        // uVar12 = (u_var3 >> 0x10);
        u_var4 = (u_var2 + 0x1f6);
        pass1_1030_355c(u_var4, uVar8 & 0xffff | u_var10 << 0x10);
        // uVar13 = (u_var4 >> 0x10);
        iStack56 = 0x0;
        loop {
            iVar5 = (iStack56 * 0x2);
            (iVar5 + u_var4 + 0x174) = (iVar5 + u_var3 + 0x174);
            uVar7 = (iVar5 + u_var3 + 0x180);
            uVar8 = uVar7;
            (iVar5 + u_var4 + 0x180) = uVar7;
            iStack56 += 0x1;
            if (iStack56 < 0x6) == false { break; }
        }
        while;
        uStack84 = 0x11;
        loop {
            puVar9 = u_var10;
            uVar7 = uVar8;
            if (0x24 < uStack84) { break; }
            if (0x0 < (uStack84 * 0x2 + ctx._PTR_LOOP_1050_580e)) {
                empty_1038_540a();
                lStack80 = CONCAT22(puVar9, uVar7);
                uVar12 = (ctx.PTR_LOOP_1050_580e >> 0x10);
                u_var11 = ctx._PTR_LOOP_1050_580e;
                iVar6 = (uStack84 * 0x2 + u_var11);
                u_var10 = iVar6 >> 0x10;
                uVar15 = uStack84;
                if (lStack80 < iVar6) {
                    iVar6 = (uStack84 * 0x2 + u_var11);
                    u_var10 = (iVar6 >> 0xf);
                    uVar15 = 0x21;
                }
                pass1_1038_52b8(uStack6, CONCAT22(u_var10, iVar6), uVar15, u_var11, param_4,
                                &ctx.PTR_LOOP_1050_1038, param_5);
                uVar15 = uStack84 * 0x2;
                uVar7 = (uVar15 + ctx._PTR_LOOP_1050_580e);
                pass1_1030_7ddc(uStack10, uVar7, uStack84, uVar7, u_var10, uVar15,
                                param_4, param_5);
                iVar6 = (ctx.PTR_LOOP_1050_580e + uVar15);
                uVar8 = SEXT24(iVar6);
                pass1_1038_5694(u_var2, iVar6, uStack84);
            }
            uStack84 += 0x1;
        }
        pass1_1030_7c50(uStack10, 0x2, 0x1, uVar7, puVar9);
        pass1_1030_7c50(uStack10, 0x2, 0x2, uVar7, puVar9);
        pass1_1030_7c50(uStack10, 0x2, 0x3, uVar7, puVar9);
        pass1_1030_7c50(uStack10, 0x2, 0x4, uVar7, puVar9);
        pass1_1038_44d8(param_2, param_3, 0x2, 0x1, uVar7, puVar9);
        pass1_1038_44d8(param_2, param_3, 0x2, 0x2, uVar7, puVar9);
        pass1_1038_44d8(param_2, param_3, 0x2, 0x3, uVar7, puVar9);
        pass1_1038_44d8(param_2, param_3, 0x2, 0x4, uVar7, puVar9);
        puVar14 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, puVar9, param_4);
        pass1_1010_043a(puVar14, (param_2 + 0x4), 0x7, param_5);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_c52e(param_1: u32, param_2: U32Ptr, param_3: u32, param_4: u32, param_5: i16,
                       param_6: u16, param_7: u16)

{
    let b_var1: bool;
    let pu_var2: u32;
    let pu_var3: U32Ptr;
    let puVar4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let u_var10: u32;
    let local_32: [u8; 12];
    let local_20: u32;
    let uStack28: u32;
    let puStack24: u32;
    let uStack22: u32;
    let uStack18: u16;
    let uStack16: u16;
    let local_c: u32;
    let uStack8: u16;
    let uStack6: u32;

    // uVar8 = (param_1 >> 0x10);
    b_var1 = pass1_1028_c314(param_7, param_5, param_6, param_1, uVar8, param_2,
                            param_3, (param_3 >> 0x10), param_4);
    if (b_var1 != 0x0) {
        pu_var2 = &local_c;
        pass1_1030_64ce(param_7, pu_var2, param_6, _PTR_LOOP_1050_5740, param_2, param_4,
                        CONCAT22(param_7, pu_var2));
        local_20 = *pu_var2;
        local_20._3_1_ = (local_20 >> 0x18);
        uStack8 = local_20._3_1_;
        if (local_20._3_1_ == 0x0) {
            uStack22 = local_20;
            uStack6 = local_20;
            pass1_1028_c7b6(param_7, param_6, param_1, uVar8, param_2, param_4);
            if ((uStack8 != 0x4) && (uStack8 != 0x0)) {
                uVar7 = pass1_1030_bcae(&local_20, param_7);
                // u_var5 = (uVar7 >> 0x10);
                pass1_1028_dc52(CONCAT22(param_7, local_32), 0x1, 0x0, 0x400);
                loop {
                    pu_var3 = local_32;
                    pass1_1028_e4ec(CONCAT22(param_7, pu_var3));
                    uStack28 = CONCAT22(u_var5, pu_var3);
                    u_var6 = u_var5 | pu_var3;
                    if (u_var6 == 0x0) {
                        return;
                    }
                    uVar7 = (pu_var3 + 0x10);
                    u_var10 = param_4;
                    uStack22 = uVar7;
                    puVar9 = param_2;
                    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar7);
                    uStack18 = uVar7;
                    puVar4 = &local_20;
                    uStack16 = u_var6;
                    pass1_1030_bcde(param_7, puVar4, param_7,
                                    uVar7 & 0xffff | u_var6 << 0x10, puVar9, u_var10);
                    if (puVar4 < 0x0) { break; }
                    u_var5 = u_var6;
                    puStack24 = puVar4;
                    if (puVar4 < 0x1f) {
                        ctx.PTR_LOOP_1050_50ca = 0x6ae;
                        return;
                    }
                }
                ctx.PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            ctx.PTR_LOOP_1050_50ca = 0x6a8;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_c652(param_1: U32Ptr, param_2: i16, param_3: u16) {
    let pu_var1: U32Ptr;

    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_3, param_1, param_2);
    pass1_1010_9794(pu_var1, param_3);
    return;
}



astruct_18 *  pass1_1030_c668(param_1: & mut Struct18,param_2: u8)

{
pass1_1028_b418(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_c71e(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) -> u16

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    CONCAT22(param_2, param_1) = 0xc940;
    (param_1 + 0x2) = 0x1030;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1030_c74e(param_1: u32, param_2: u32, param_3: u16) {
    pass1_1028_b46e(param_1, param_2, param_3);
    (param_1 + 0x20) = 0x70;
    return;
}


pub fn pass1_1030_c76c(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (((i_var1 + 0x12) != 0x6) && ((i_var1 + 0x12) != 0x5)) {
        return;
    }
    i_var1 = (i_var1 + 0x20);
    if (i_var1 != 0x0) {
        if (((i_var1 < 0x70) || (SBORROW2(i_var1, 0x70))) || (0x1 < i_var1 + -0x70)) {
            pass1_1028_be2a(param_1, param_2, param_3, &USHORT_1050_1028, param_4);
            return;
        }
    }
    pass1_1028_bdac(param_1, 0x6, &USHORT_1050_1028);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_c7b0(param_1: u32, param_2: u16, param_3: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let BVar5: bool;
    let u_var6: u32;
    let extraout_dx: U32Ptr;
    let puVar7: U32Ptr;
    let i_var8: i16;
    let uVar9: u16;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    i_var1 = (i_var8 + 0x20);
    if (i_var1 != 0x0) {
        i_var3 = i_var1 + -0x70;
        i_var4 = i_var3;
        if (((i_var1 < 0x70) || (SBORROW2(i_var1, 0x70))) || (i_var4 = i_var1 + -0x71, i_var4 != 0x0 && 0x0 < i_var3)) {
            pass1_1028_b58e(param_1 & 0xffff | uVar9 << 0x10);
            u_var2 = (i_var4 + 0x2e);
            u_var6 = (u_var2 + 0x200);
            puVar7 = extraout_dx;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var6);
            u_var6 = u_var6 & 0xffff | ZEXT24(puVar7) << 0x10;
            BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (i_var8 + 0xc), 0x11);
            pass1_1030_23e2(u_var6, BVar5, (i_var8 + 0x20), BVar5, puVar7, param_2, param_3);
            if (BVar5 != 0x0) {
                if ((i_var8 + 0x20) == 0x1) {
                    pass1_1030_25d8(u_var6, 0x64, (i_var8 + 0x20));
                    return;
                }
                (i_var8 + 0x20) = 0x70;
            }
        }
    }
    return;
}


pub fn pass1_1030_c84e(param_1: u32, param_2: u32, param_3: u16) -> bool

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


pub fn pass1_1030_c894(param_1: u32, param_2: u32, param_3: bool, param_4: U32Ptr, param_5: u16) -> bool

{
    let b_var1: bool;
    let local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 != 0x0) {
        b_var1 = read_file_1008_7dee(param_2, (param_2 >> 0x10), &local_4, 0x0, param_5, 0x2, 0x1008);
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return b_var1;
        }
        (param_1 + 0x20) = local_4;
        param_3 = 0x1;
    }
    return param_3;
}


pub fn pass1_1030_c8da(param_1: u32, param_2: u32, param_3: u32) -> u32

{
    let u_var1: u32;

    u_var1 = 0x0;
    if (param_3._2_2_ == 0x1) {
        (param_1 + 0x20) = param_2._2_2_;
    } else {
        u_var1 = func_0x1030178e();
    }
    return u_var1;
}



astruct_18 *  pass1_1030_c91a(param_1: & mut Struct18,param_2: u8)

{
pass1_1028_b418(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


pub fn pass1_1030_c9e4(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) -> u32

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x98) = 0x1;
    CONCAT22(param_2, param_1) = 0xd88e;
    (param_1 + 0x2) = 0x1030;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x20), 0x0, 0x78);
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1030_ca26(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;
    let extraout_dx: u16;
    let i_var2: i16;
    let u_var3: u16;
    let uStack4: u16;

    // for (uStack4 = 0x0; i_var2 = param_1, u_var3 = (param_1 >> 0x10),
    //     uStack4 < 0xa; uStack4 += 0x1) {
    //   if (((i_var2 + uStack4 * 0xc + 0x26) == 0x2) ||
    //      ((i_var2 + uStack4 * 0xc + 0x26) == 0x1)) {
    //     (i_var2 + uStack4 * 0xc + 0x26) = 0x4;
    //     param_3 = uStack4;
    //   }
    //   else {
    //     u_var1 = uStack4;
    //     pass1_1028_b58e(param_1);
    //     i_var2 = uStack4 * 0xc + i_var2;
    //     pass1_1030_6e9c(CONCAT22(extraout_dx,u_var1),0x1,(i_var2 + 0x24));
    //     param_3 = 0x0;
    //     (i_var2 + 0x20) = 0x0;
    //     (i_var2 + 0x24) = 0x0;
    //     (i_var2 + 0x26) = 0x0;
    //   }
    // }
    pass1_1028_b46e(param_1, param_2, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_cac2(param_1: U32Ptr, param_2: i16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let pu_var2: u32;
    let ppc_var3: u32;
    let u_var4: u32;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: u32;
    let uVar8: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let uVar9: u16;
    let u_var10: u16;
    let uStack34: u32;
    let iStack30: i16;
    let iStack28: i16;

    pass1_1028_be9e(param_1, param_3, param_4, &USHORT_1050_1028, param_5);
    // u_var10 = (param_1 >> 0x10);
    if (((param_1 + 0x12) == 0x5) && (ctx.PTR_LOOP_1050_5812 == 0x0)) {
        ctx.PTR_LOOP_1050_5812 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        pass1_1028_b58e(param_1 & 0xffff | u_var10 << 0x10);
        u_var1 = (param_2 + 0x2e);
        u_var6 = (u_var1 + 0x10);
        u_var10 = extraout_dx;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var6);
        pu_var2 = (u_var6 + 0x1e);
        ppc_var3 = (*pu_var2 + 0x10);
        puVar7 = pu_var2;
        (**ppc_var3)(&USHORT_1050_1028, pu_var2, (u_var6 + 0x20));
        u_var4 = puVar7 & 0xffff | extraout_DX_00 << 0x10;
        iStack28 = 0x0;
        iStack30 = pass1_1030_d144(param_1);
        uStack34 = 0x0;
        while ((uStack34 < u_var4 && (iStack30 != 0x0))) {
            ppc_var3 = (*pu_var2 + 0x4);
            uVar8 = u_var4;
            (**ppc_var3)(&USHORT_1050_1028, pu_var2, (pu_var2 >> 0x10),
                        uStack34, (uStack34 >> 0x10));
            u_var5 = uVar8;
            uVar9 = extraout_DX_01 | u_var5;
            if (uVar9 != 0x0) {
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
                u_var5 = (u_var5 + 0xc);
                if ((0x0 < u_var5) && (!SBORROW2(u_var5, 0x1))) {
                    if (u_var5 != 0x3 && 0x0 < (u_var5 - 0x2)) {
                        if (u_var5 != 0x4) {
                            // goto
                            // LAB_1030_cbbc;
                        }
                        iStack28 += 0x1;
                    }
                    pass1_1030_6e9c(u_var6 & 0xffff | u_var10 << 0x10, 0x1, u_var5);
                    pass1_1030_d180(param_1, u_var5);
                    iStack30 += -0x1;
                }
            }
//LAB_1030_cbbc:
            uStack34 += 0x1;
        }
        while (iStack28 < 0x4) {
            pass1_1030_d180(param_1, 0x4);
            iStack28 = iStack28 + 0x1;
        }
    }
    return;
}


pub fn pass1_1030_cbf0(param_1: i16, param_2: u16, param_3: i16) -> u16

{
    let i_var1: &mut Struct595;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return 0x0;
        }
        i_var1 = (i_stack4 * 0xc + param_1);
        if ((i_var1.field_0x24 == param_3) && (i_var1.field_0x26 == 0x3)) { break; }
        i_stack4 += 0x1;
    }
    i_var1.field_0x26 = 0x0;
    i_var1.field_0x28 = 0x0;
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_cc44(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: i16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let puVar7: U32Ptr;
    let extraout_DX_01: u16;
    let iVar7: &mut Struct304;
    let i_var8: &mut Struct303;
    let uVar8: u8;
    let unaff_SS: u16;
    let puVar9: u32;
    let puVar10: u32;
    let puVar11: U32Ptr;
    let local_32: [u8; 8];
    let puStack42: u32;
    let uStack38: u32;
    let uStack34: u32;
    let puStack30: u32;
    let uStack26: u16;
    let puStack24: U32Ptr;
    let uStack22: u16;
    let puStack20: U32Ptr;
    let puStack18: u32;
    let iStack14: i16;
    let uStack12: u16;
    let uStack10: i16;
    let uStack8: u32;
    let i_stack4: i16;

    i_stack4 = 0x0;
    uStack8 = (param_4 + 0x4);
    iStack10 = 0x0;
    loop {
        if (0x9 < iStack10) {
            return;
        }
        i_var8 = (iStack10 * 0xc + param_1);
        if (((i_var8.field_0x28 == uStack8) && (i_var8.field_0x2a == uStack8._2_2_)) && (i_var8.field_0x24 == param_5)) {
            if (i_var8.field_0x26 == 0x4) {
                i_var2 = param_5;
                pass1_1028_b58e(CONCAT22(param_2, param_1));
                iStack14 = i_var2;
                uStack12 = extraout_DX_00;
                pass1_1030_6e9c(CONCAT13((extraout_DX_00 >> 0x8),
                                         CONCAT12(extraout_DX_00, iStack14)), 0x1,
                                i_var8.field_0x24);
                i_var8.field_0x20 = 0x0;
                i_var8.field_0x24 = 0x0;
                i_var8.field_0x26 = 0x0;
                puStack42 = 0x0;
                puStack18 = 0x0;
                _DAT_0000_0006 = param_5;
                uRam0000000a = 0x1;
                u_var4 = switch_1020_c3b4(param_5);
                (puStack18 + 0xc) = u_var4;
                puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
                // puVar7 = (puVar10 >> 0x10);
                u_var6 = puVar10;
                u_var5 = u_var6;
                puVar11 = puVar7;
                uStack22 = u_var6;
                puStack20 = puVar7;
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
                uVar8 = 0x38;
                uStack26 = u_var6;
                puStack24 = puVar7;
                pass1_1038_4d6e(CONCAT22(puVar7, u_var6), CONCAT22(puVar11, u_var5), u_var6,
                                puVar7);
                puStack30 = CONCAT22(puVar7, u_var6);
                ppcVar1 = (*puStack30 + 0x10);
                (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var6, puVar7);
                uStack34 = CONCAT22(extraout_DX_01, u_var6);
                u_var6 = extraout_DX_01;
                // for (uStack38 = 0x0; uStack38 < uStack34; uStack38 += 0x1) {
                //   puVar9 = pass1_1030_1d7c(uStack34,u_var6,puStack30);
                //   u_var5 = (puVar9 >> 0x10);
                //   u_var6 = u_var5 | puVar9;
                //   if (u_var6 != 0x0) {
                //     pu_var3 = local_32;
                //     ppcVar1 = (*puVar9 + 0x40);
                //     (**ppcVar1)(0x38,puVar9,u_var5,pu_var3);
                //     u_var6 = extraout_dx;
                //     if (pu_var3 == 0x0) {
                //       uVar8 = 0x28;
                //       pass1_1028_6408(puVar9,puStack18,unaff_SS);
                //       break;
                //     }
                //   }
                // }
                puStack42 = puStack30;
                if (puStack30 != 0x0) {
                    ppcVar1 = *puStack30;
                    (**ppcVar1)(uVar8, puStack30, (puStack30 >> 0x10), 0x1);
                }
            } else {
                iVar7 = (iStack10 * 0xc + param_1);
                iVar7.field_0x26 = 0x0;
                iVar7.field_0x28 = 0x0;
            }
            i_stack4 += 0x1;
            param_3 += -0x1;
            if (param_3 == 0x0) {
                return;
            }
        }
        iStack10 += 0x1;
    }
}



i16  pass1_1030_cde8(param_1: i16,param_2: u16,param_3: i16)

{
let i_var1: i16; let i_stack4: i16;

i_stack4 = 0x0; loop {
if (0x9 < i_stack4) {
return - 0x1;
}
i_var1 = i_stack4 * 0xc + param_1; if (((i_var1 + 0x24) == param_3) && ((i_var1 + 0x26) == 0x0)) { break; }
i_stack4 += 0x1;
}
return i_stack4;
}


// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_ce2e(param_1: i16,param_2: u16,param_3: i16)

{
let i_var1: i16; let uStack6: u32;

// for (uStack6 = 0x0; uStack6 < 0xa;
//     uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
//   i_var1 = uStack6 * 0xc + param_1;
//   if (((i_var1 + 0x24) == param_3) && ((i_var1 + 0x26) == 0x0)) {
//     uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
//   }
// } return uStack6._2_2_;
}



pub fn pass1_1030_ce72(param_1: u32, param_2: i16, param_3: u32, param_4: i16) {
    let lVar1: i32;
    let i_var2: &mut Struct300;
    let uStack10: i16;

    lVar1 = (param_3 + 0x4);
    iStack10 = 0x0;
    loop {
        if (0x9 < iStack10) {
            return;
        }
        i_var2 = (iStack10 * 0xc + param_1);
        if ((i_var2.field_0x24 == param_4) && (i_var2.field_0x28 == 0x0)) {
            i_var2.field_0x28 = lVar1;
            if (param_4 == 0x4) {
                i_var2.field_0x26 = 0x2;
            } else {
                (param_1 + iStack10 * 0xc + 0x26) = 0x1;
            }
            param_2 += -0x1;
            if (param_2 == 0x0) {
                return;
            }
        }
        iStack10 += 0x1;
    }
}


pub fn pass1_1030_cef8(param_1: u32, param_2: u32, param_3: u16, param_4: i16) {
    let u_var1: u16;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    (i_var2 + param_4 * 0xc + 0x26) = param_3;
    // u_var4 = (param_2 >> 0x10);
    u_var1 = (param_2 + 0x6);
    (i_var2 + param_4 * 0xc + 0x28) = (param_2 + 0x4);
    (i_var2 + param_4 * 0xc + 0x2a) = u_var1;
    return;
}


pub fn pass1_1030_cf3a(param_1: u32, param_2: i16) -> u16

{
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return 0x0;
        }
        if ((param_1 + i_stack4 * 0xc + 0x24) == param_2) { break; }
        i_stack4 += 0x1;
    }
    return 0x1;
}


pub fn pass1_1030_cf78(param_1: u32, param_2: u16) {
    let u_var1: u32;
    let extraout_dx: u16;
    let i_var3: &mut Struct680;
    let u_var2: u16;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return;
        }
        u_var1 = param_2;
        // u_var2 = (param_1 >> 0x10);
        if ((param_1 + i_stack4 * 0xc + 0x24) == param_2) { break; }
        i_stack4 += 0x1;
    }
    pass1_1028_b58e(param_1);
    if (param_2 == 0x5) {
        pass1_1038_4900((u_var1 + 0x2e));
    } else {
        pass1_1030_6e9c(u_var1 & 0xffff | extraout_dx << 0x10, 0x1, param_2);
    }
    i_var3 = (i_stack4 * 0xc + param_1);
    i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    i_var3.field_0x26 = 0x0;
    return;
}


pub fn pass1_1030_d00c(param_1: i16, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let extraout_dx: u16;
    let local_BX_40: &mut Struct696;
    let i_var2: i16;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return;
        }
        i_var2 = i_stack4 * 0xc + param_1;
        if (((i_var2 + 0x26) == 0x0) && (u_var1 = param_3, (i_var2 + 0x24) == param_3)) { break; }
        i_stack4 += 0x1;
    }
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    if (param_3 == 0x5) {
        pass1_1038_4900((u_var1 + 0x2e));
    } else {
        pass1_1030_6e9c(u_var1 & 0xffff | extraout_dx << 0x10, 0x1, param_3);
    }
    local_BX_40 = (i_stack4 * 0xc + param_1);
    local_BX_40.field_0x20 = 0x0;
    local_BX_40.field_0x24 = 0x0;
    local_BX_40.field_0x26 = 0x0;
    return;
}


pub fn pass1_1030_d0a8(param_1: u32) -> u16

{
    let u_var1: u16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x98);
    pass1_1030_d56a(param_1 & 0xffff | u_var2 << 0x10);
    return u_var1;
}


// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_d0c6(param_1: u32)

{
let uStack6: u32;

// for (uStack6 = 0x0; uStack6 < 0xa;
//     uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
//   if ((param_1 + uStack6 * 0xc + 0x20) != 0x0) {
//     uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
//   }
// } return uStack6._2_2_;
}


// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_d102(param_1: i16,param_2: u16)

{
let i_var1: i16; let uStack6: u32;

// for (uStack6 = 0x0; uStack6 < 0xa;
//     uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
//   i_var1 = uStack6 * 0xc + param_1;
//   if (((i_var1 + 0x20) != 0x0) && ((i_var1 + 0x26) != 0x0)) {
//     uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
//   }
// } return uStack6._2_2_;
}


// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_d144(param_1: u32)

{
let uStack6: u32;

// for (uStack6 = 0x0; uStack6 < 0xa;
//     uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
//   if ((param_1 + uStack6 * 0xc + 0x20) == 0x0) {
//     uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
//   }
// } return uStack6._2_2_;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_d180(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let extraout_dx: U32Ptr;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return;
        }
        // u_var5 = (param_1 >> 0x10);
        u_var3 = param_1;
        if (((u_var3 + i_stack4 * 0xc + 0x22) | (u_var3 + i_stack4 * 0xc + 0x20)
        ) == 0x0) { break; }
        i_stack4 += 0x1;
    }
    u_var2 = *_PTR_LOOP_1050_65e2;
    i_var1 = (ctx.PTR_LOOP_1050_65e2 + 0x2);
    i_var4 = i_stack4 * 0xc + u_var3;
    (i_var4 + 0x20) = u_var2 + 0xc8;
    (i_var4 + 0x22) = i_var1 + (0xff37 < u_var2);
    (i_var4 + 0x24) = param_2;
    u_var2 = param_2;
    pass1_1030_d340(u_var3, u_var5, param_1 & 0xffff0000 | (i_var4 + 0x20));
    pass1_1028_b58e(param_1);
    if (param_2 == 0x5) {
        pass1_1038_48e0((u_var2 + 0x2e), 0x1);
        return;
    }
    pass1_1030_7c50(CONCAT22(extraout_dx, u_var2), 0x1, param_2, u_var2, extraout_dx);
    return;
}


pub fn pass1_1030_d230(param_1: u32) -> u16

{
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return 0x1;
        }
        if ((param_1 + i_stack4 * 0xc + 0x20) == 0x0) { break; }
        i_stack4 += 0x1;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_d26c(param_1: u32, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u32;
    let extraout_dx: u16;
    let iVar5: i16;
    let iStack8: i16;

    u_var2 = *_PTR_LOOP_1050_65e2;
    // for (iStack8 = 0x0; iStack8 < 0xa; iStack8 += 0x1) {
    //   iVar5 = iStack8 * 0xc + param_1;
    //   if ((((iVar5 + 0x22) | (iVar5 + 0x20)) != 0x0) &&
    //      (pu_var1 = (iVar5 + 0x20), *pu_var1 < u_var2 || *pu_var1 == u_var2)) {
    //     u_var4 = u_var2;
    //     pass1_1030_d3b2(param_1,param_1._2_2_,iStack8,u_var2,param_2);
    //     i_var3 = u_var4;
    //     if (i_var3 == 0x0) {
    //       pass1_1028_b58e(param_1);
    //       if ((iVar5 + 0x24) == 0x5) {
    //         pass1_1038_4900((i_var3 + 0x2e));
    //       }
    //       else {
    //         pass1_1030_6e9c(CONCAT22(extraout_dx,i_var3),0x1,
    //                         (param_1 + iStack8 * 0xc + 0x24));
    //       }
    //       iVar5 = iStack8 * 0xc + param_1;
    //       (iVar5 + 0x20) = 0x0;
    //       (iVar5 + 0x24) = 0x0;
    //       (iVar5 + 0x26) = 0x0;
    //     }
    //   }
    // }
    return;
}


pub fn pass1_1030_d340(param_1: u16, param_2: u16, param_3: u32) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_3 >> 0x10);
    i_var2 = param_3;
    i_var1 = (i_var2 + 0x4);
    if (((0x0 < i_var1) && (!SBORROW2(i_var1, 0x1))) && ((i_var1 == 0x4 || i_var1 + -0x1 < 0x3 || (i_var1 == 0xc)))) {
        (i_var2 + 0x6) = 0x0;
        return;
    }
    (i_var2 + 0x6) = 0x1;
    return;
}


pub fn pass1_1030_d36e(param_1: u32, param_2: i16) -> u16

{
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        if (0x9 < i_stack4) {
            return 0x0;
        }
        if ((i_stack4 != param_2) && ((param_1 + i_stack4 * 0xc + 0x24) == 0x8))
        break;
        i_stack4 += 0x1;
    }
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_d3b2(param_1: i16, param_2: u16, param_3: i16, param_4: i16, param_5: u16) {
    let i_var1: i16;
    let ppcVar2: u32;
    let bVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let extraout_dx: u16;
    let puVar6: U32Ptr;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let uVar7: u16;
    let uVar8: u16;
    let puVar9: u32;
    let u_var10: u32;
    let u_var11: u32;
    let puStack26: u32;
    let uStack18: u32;
    let uStack14: u32;

    pass1_1028_b58e(CONCAT22(param_2, param_1));
    u_var11 = (param_4 + 0x2e);
    u_var4 = pass1_1030_d36e(CONCAT22(param_2, param_1), param_3);
    if (u_var4 == 0x0) {
        puVar9 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1e);
        // puVar6 = (puVar9 >> 0x10);
        u_var5 = puVar9;
        pass1_1038_4d6e(u_var11, puVar9, u_var5, puVar6);
        puStack26 = CONCAT22(puVar6, u_var5);
        ppcVar2 = (*puStack26 + 0x10);
        uVar7 = u_var5;
        (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, u_var5, puVar6);
        uStack18 = CONCAT22(extraout_DX_00, uVar7);
        bVar3 = false;
        // for (uStack14 = 0x0; uStack14 < uStack18; uStack14 += 0x1) {
        //   u_var10 = pass1_1030_1d7c(uStack14,uStack14._2_2_,puStack26);
        //   uVar7 = (u_var10 >> 0x10);
        //   if ((((uVar7 | u_var10) != 0x0) &&
        //       ((u_var10 + 0x4) != (param_1 + 0x4))) &&
        //      (u_var4 = pass1_1030_cf3a(u_var10,0x8), u_var4 != 0x0)) {
        //     bVar3 = true;
        //     break;
        //   }
        // }
        if (puStack26 != 0x0) {
            ppcVar2 = *puStack26;
            (**ppcVar2)(0x38, u_var5, puVar6, 0x1);
        }
        if (!bVar3) {
            return;
        }
    }
    puVar9 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
    // puVar6 = (puVar9 >> 0x10);
    u_var5 = puVar9;
    uVar8 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(u_var11, puVar9, u_var5, puVar6);
    puStack26 = CONCAT22(puVar6, u_var5);
    ppcVar2 = (*puStack26 + 0x10);
    uVar7 = u_var5;
    (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, u_var5, puVar6);
    uStack18 = CONCAT22(extraout_DX_01, uVar7);
    bVar3 = false;
    uStack14 = 0x0;
    loop {
        if (uStack18 <= uStack14) {
//LAB_1030_d51b:
            if (puStack26 != 0x0) {
                ppcVar2 = *puStack26;
                (**ppcVar2)(uVar8, u_var5, puVar6, 0x1);
            }
            if (!bVar3) {
                return;
            }
            u_var5 = *_PTR_LOOP_1050_65e2;
            i_var1 = (ctx.PTR_LOOP_1050_65e2 + 0x2);
            (param_1 + param_3 * 0xc + 0x20) = u_var5 + 0xc8;
            (param_1 + param_3 * 0xc + 0x22) = i_var1 + (0xff37 < u_var5);
            return;
        }
        u_var11 = pass1_1030_1d7c(uStack14, uStack14._2_2_, puStack26);
        uVar7 = (u_var11 >> 0x10) | u_var11;
        if (uVar7 != 0x0) {
            uVar8 = SUB42(&USHORT_1050_1028, 0x0);
            u_var4 = pass1_1028_6744(param_5, u_var11, 0x7);
            if ((uVar7 | u_var4) != 0x0) {
                uVar8 = SUB42(&USHORT_1050_1028, 0x0);
                pass1_1028_6228(u_var11, 0x1, 0x0, 0x7, param_5);
                bVar3 = true;
//         TODO: goto LAB_1030_d51b;
            }
        }
        uStack14 += 0x1;
    }
}



i16  pass1_1030_d56a(param_1: u32)

{
let i_var1: i16; let i_var2: i16;
let i_var3: i16; let u_var4: u16;

// u_var4 = (param_1 >> 0x10);
i_var3 = param_1; i_var2 = (i_var3 + 0x98) + - 0x1; i_var1 = i_var2; if (false) {
switchD_1030_d5fb_caseD_4: (i_var3 + 0x98) = 0x1;
return i_var1;
}
i_var1 = i_var3; switch(i_var2) {
0x0 => (i_var3 + 0x98) = 0x2;
break; 0x1 => (i_var3 + 0x98) = 0x3; break; 0x2 => (i_var3 + 0x98) = 0x4; break; 0x3 => (i_var3 + 0x98) = 0xc; break; default:
//     TODO: goto switchD_1030_d5fb_caseD_4;
0x7 => (i_var3 + 0x98) = 0x9;
return i_var3; 0x8 => (i_var3 + 0x98) = 0xb; return i_var3; 0xa => (i_var3 + 0x98) = 0x5; return i_var3; 0xb => (i_var3 + 0x98) = 0x8; return i_var3;
}
return i_var3;
}



pub fn pass1_1030_d61c(param_1: u32, param_2: u32, param_3: u16) {
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let local_1a: u32;
    let local_16: U32Ptr;
    let local_14: u16;
    let local_12: [u32; 0x3];
    let i_stack4: i16;

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (b_var1 != 0x0) {
        // for (i_stack4 = 0x0; u_var4 = param_2, u_var5 = (param_2 >> 0x10),
        //     i_stack4 < 0xa; i_stack4 += 0x1) {
        //   u_var3 = (param_1 >> 0x10);
        //   i_var2 = param_1;
        //   local_12[0] = (i_var2 + i_stack4 * 0xc + 0x20);
        //   b_var1 = write_to_file_1008_7e1c
        //                     (u_var4,u_var5,local_12,param_3,0x4,0x1008);
        //   if (b_var1 == 0x0) goto LAB_1030_d701;
        //   local_14 = (i_var2 + i_stack4 * 0xc + 0x24);
        //   b_var1 = write_to_file_1008_7e1c
        //                     (u_var4,u_var5,&local_14,param_3,0x2,0x1008);
        //   if (b_var1 == 0x0) goto LAB_1030_d701;
        //   local_16 = (i_var2 + i_stack4 * 0xc + 0x26);
        //   b_var1 = write_to_file_1008_7e1c
        //                     (u_var4,u_var5,&local_16,param_3,0x2,0x1008);
        //   if (b_var1 == 0x0) goto LAB_1030_d701;
        //   local_1a = (i_var2 + i_stack4 * 0xc + 0x28);
        //   b_var1 = write_to_file_1008_7e1c
        //                     (u_var4,u_var5,&local_1a,param_3,0x4,0x1008);
        //   if (b_var1 == 0x0) goto LAB_1030_d701;
        // }
        local_16 = ctx.PTR_LOOP_1050_5812;
        b_var1 = write_to_file_1008_7e1c(u_var4, u_var5, &local_16, param_3, 0x2, 0x1008);
        if (b_var1 != 0x0) {
            return;
        }
//LAB_1030_d701:
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_d72e(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let uStack10: i16;
    let local_8: u32;
    let local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 == 0x0) {
        return;
    }
    iStack10 = 0x0;
    loop {
        u_var4 = param_2;
        // u_var5 = (param_2 >> 0x10);
        if (0x9 < iStack10) {
            if ((0x3 < ctx.PTR_LOOP_1050_0312) && (BVar2 = read_file_1008_7dee(u_var4, u_var5, &ctx.PTR_LOOP_1050_5812, 0x0,
                                                                               ctx.data_seg, 0x2, 0x1008), BVar2 == 0x0)
            ) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            return;
        }
        BVar2 = read_file_1008_7dee(u_var4, u_var5, &local_8, 0x0, param_5, 0x4, 0x1008);
        if (BVar2 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return;
        }
        BVar2 = read_file_1008_7dee(u_var4, u_var5, &local_4, 0x0, param_5, 0x2, 0x1008);
        if (BVar2 == 0x0) { break; }
        i_var3 = iStack10 * 0xc + param_1;
        (i_var3 + 0x20) = local_8;
        (i_var3 + 0x22) = local_8._2_2_;
        u_var1 = switch_1008_72bc(u_var4, u_var5, local_4);
        (i_var3 + 0x24) = u_var1;
        if (ctx.PTR_LOOP_1050_0312 < 0x2) {
            i_var3 = iStack10 * 0xc + param_1;
            (i_var3 + 0x26) = 0x3;
            (i_var3 + 0x28) = 0x0;
        } else {
            BVar2 = read_file_1008_7dee(u_var4, u_var5, &local_4, 0x0, param_5, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            BVar2 = read_file_1008_7dee(u_var4, u_var5, &local_8, 0x0, param_5, 0x4, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return;
            }
            i_var3 = iStack10 * 0xc + param_1;
            (i_var3 + 0x26) = local_4;
            (i_var3 + 0x28) = local_8;
        }
        iStack10 += 0x1;
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
}



astruct_18 *  pass1_1030_d868(param_1: & mut Struct18,param_2: u8)

{
pass1_1028_b418(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


pub fn pass1_1030_d942(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) -> u32

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0xdc2e;
    (param_1 + 0x2) = 0x1030;
    if ((param_1 + 0xc) == 0x4c) {
        (param_1 + 0xe) = 0x43;
    } else {
        if ((param_1 + 0xc) == 0x4d) {
            (param_1 + 0xe) = 0x44;
        } else {
            (param_1 + 0xe) = 0x45;
        }
    }
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1030_d994(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x12) != 0x4) {
        return;
    }
    u_var6 = pass1_1028_b4f2(param_1);
    i_var3 = u_var6;
    if ((i_var3 + 0x200) == 0x8000002) {
        u_var2 = (i_var4 + 0x14);
        pi_var1 = (u_var2 + 0x94);
        *pi_var1 = *pi_var1 + -0x1;
    } else {
        pass1_1028_cb04(param_1, param_2, param_3, param_4);
        if (i_var3 == 0x0) {
            return;
        }
        pass1_1030_dace(param_1, param_4);
        if (i_var3 == 0x0) {
            return;
        }
        u_var2 = (i_var4 + 0x14);
        pi_var1 = (u_var2 + 0x94);
        *pi_var1 = *pi_var1 + -0x1;
        pass1_1028_c952(param_1, param_2, param_3, param_4);
        pass1_1030_da22(param_1, param_4);
    }
    u_var2 = (i_var4 + 0x14);
    if ((u_var2 + 0x94) < 0x1) {
        pass1_1028_bdac(param_1, 0x5, &USHORT_1050_1028);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_da22(param_1: u32, param_2: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let puVar6: u32;
    let extraout_dx: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u32;
    let uStack18: u32;

    uVar9 = pass1_1028_b4f2(param_1);
    // u_var3 = (uVar9 >> 0x10);
    pu_var1 = (uVar9 + 0xc);
    ppcVar2 = (*pu_var1 + 0x10);
    puVar6 = pu_var1;
    (**ppcVar2)(&USHORT_1050_1028, pu_var1, (uVar9 + 0xe));
    uStack18 = 0x0;
    loop {
        if ((puVar6 & 0xffff | extraout_dx << 0x10) <= uStack18) {
            return;
        }
        uVar9 = pass1_1030_1d7c((puVar6 & 0xffff), extraout_dx, pu_var1);
        // uVar7 = (uVar9 >> 0x10);
        uVar8 = uVar7 | uVar9;
        if (((uVar8 != 0x0) && (BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (uVar9 + 0xc),
                                                        0x4), BVar4 != 0x0)) && (u_var5 = pass1_1028_6744(param_2, uVar9, 0xd), (uVar8 | u_var5) != 0x0)) { break; }
        uStack18 += 0x1;
    }
    pass1_1028_6228(uVar9, 0x1, 0x0, 0xd, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_dace(param_1: u32, param_2: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let puVar6: u32;
    let extraout_dx: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u32;
    let uStack20: u32;

    uVar9 = pass1_1028_b4f2(param_1);
    // u_var3 = (uVar9 >> 0x10);
    pu_var1 = (uVar9 + 0xc);
    ppcVar2 = (*pu_var1 + 0x10);
    puVar6 = pu_var1;
    (**ppcVar2)(&USHORT_1050_1028, pu_var1, (uVar9 + 0xe));
    uStack20 = 0x0;
    uVar8 = extraout_dx;
    loop {
        if ((puVar6 & 0xffff | extraout_dx << 0x10) <= uStack20) {
            return;
        }
        uVar9 = pass1_1030_1d7c((puVar6 & 0xffff), uVar8, pu_var1);
        // uVar7 = (uVar9 >> 0x10);
        uVar8 = uVar7 | uVar9;
        if ((uVar8 != 0x0) && (BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (uVar9 + 0xc), 0x4,
        ), BVar4 != 0x0)) {
            u_var5 = pass1_1028_6744(param_2, uVar9, 0xd);
            uVar8 |= u_var5;
            if (uVar8 != 0x0) {
                return;
            }
        }
        uStack20 += 0x1;
    }
}


pub fn pass1_1030_db72() -> u16

{
    return 0x1;
}


pub fn pass1_1030_db78(param_1: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x6) {
        pass1_1028_bdac((param_1 & 0xffff | u_var1 << 0x10), 0x5,
                        &USHORT_1050_1028);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_db92(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: u32, param_5: i32,
                       param_6: u16)

{
    let i_var1: i16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u32;
    let local_4: [u8; 2];

    u_var4 = pass1_1030_bcae(local_4, param_6);
    // u_var3 = (u_var4 >> 0x10);
    i_var1 = u_var4;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var4 = (i_var1 + 0x10);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
    pu_var2 = local_4;
    pass1_1030_bcde(param_6, pu_var2, param_6, u_var4 & 0xffff | u_var3 << 0x10,
                    param_3, param_5);
    if (pu_var2 < 0x0) {
        ctx.PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}


pub fn pass1_1030_dc02() -> u16

{
    return 0x1;
}



astruct_18 *  pass1_1030_dc08(param_1: & mut Struct18,param_2: u8)

{
pass1_1028_b418(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_dcc2(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) -> u16

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    CONCAT22(param_2, param_1) = 0xe036;
    (param_1 + 0x2) = 0x1030;
    return CONCAT22(param_2, param_1);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_dcf4(param_1: U32Ptr, param_2: u16) {
    let lVar1: i32;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let extraout_dx: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let extraout_DX_00: u16;
    let uVar8: u16;
    let i_var9: &mut Struct596;
    let uVar9: u16;
    let puVar10: u32;
    let u_var11: u32;
    let uStack28: u32;
    let uStack24: u32;
    let puStack20: u32;
    let iStack12: i16;

    // uVar9 = (param_1 >> 0x10);
    i_var9 = param_1;
    *param_1 = 0xe036;
    i_var9.field_0x2 = 0x1030;
    if (ctx.PTR_LOOP_1050_65e2 != 0x0) {
        pass1_1028_b58e(param_1);
        if (i_var9.field_0x20 == 0x0) {
            u_var3 = extraout_dx | param_2;
            if (u_var3 == 0x0) {
                u_var6 = extraout_dx;
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
            } else {
                u_var3 = (param_2 + 0x2e);
                u_var6 = (param_2 + 0x30);
            }
            puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1e);
            // puVar7 = (puVar10 >> 0x10);
            u_var4 = puVar10;
            pass1_1038_4d6e(CONCAT22(u_var6, u_var3), puVar10, u_var4, puVar7);
            puStack20 = CONCAT22(puVar7, u_var4);
            ppcVar2 = (*puStack20 + 0x10);
            u_var6 = u_var4;
            (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, u_var4, puVar7);
            uStack24 = CONCAT22(extraout_DX_00, u_var6);
            u_var3 = extraout_DX_00;
            // for (uStack28 = 0x0; uStack28 < uStack24; uStack28 += 0x1) {
            //   u_var11 = pass1_1030_1d7c(u_var6,u_var3,puStack20);
            //   uVar8 = (u_var11 >> 0x10);
            //   u_var3 = uVar8 | u_var11;
            //   if (u_var3 != 0x0) {
            //     u_var5 = pass1_1030_dfcc(param_1);
            //     u_var5 = pass1_1030_cbf0(u_var11,uVar8,u_var5);
            //     if (u_var5 != 0x0) break;
            //   }
            // }
            if (puStack20 != 0x0) {
                ppcVar2 = *puStack20;
                (**ppcVar2)(0x38, u_var4, puVar7, 0x1);
            }
        } else {
            lVar1 = i_var9.field_0x20;
            u_var3 = extraout_dx;
            u_var6 = param_2;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, lVar1);
            if ((u_var3 | u_var6) != 0x0) {
                iStack12 = 0x0;
                switch(i_var9.field_0xc)
                {
                    0x73 => 0x77 => iStack12 = 0x1;
                    break;
                    0x74 => 0x78 => iStack12 = 0x2;
                    break;
                    0x75 => iStack12 = 0x3;
                    break;
                    0x76 => iStack12 = 0x5;
                }
                if (iStack12 != 0x0) {
                    pass1_1030_cc44(u_var6, u_var3, 0x1, CONCAT22(extraout_dx, param_2), iStack12);
                }
            }
        }
    }
    pass1_1028_b418(param_1);
    return;
}


pub fn pass1_1030_de7c(param_1: u32, param_2: u32, param_3: u16) {
    let b_var1: bool;
    let local_10: [u32; 0x3];

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (b_var1 != 0x0) {
        local_10[0] = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), local_10, param_3,
                                        0x4, 0x1008);
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}


pub fn pass1_1030_dec4(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let b_var1: bool;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (((param_3 != 0x0) && (0x1 < ctx.PTR_LOOP_1050_0312)) && (b_var1 = read_file_1008_7dee(param_2, (param_2 >> 0x10),
                                                                                             param_1 + 0x20, 0x0, (param_1 >> 0x10), 0x4,
                                                                                             0x1008), b_var1 == 0x0)) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    return;
}


pub fn pass1_1030_df0c(param_1: u32, param_2: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let lVar3: i32;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u32;
    let extraout_dx: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let uStack24: u16;
    let uStack22: u16;
    let uStack14: u16;
    let uStack10: u16;

    pass1_1028_b58e(param_1);
    u_var1 = (param_2 + 0x2e);
    uStack10 = u_var1;
    if (((param_2 + 0x30) | uStack10) != 0x0) {
        // uVar9 = (u_var1 >> 0x10);
        u_var1 = (uStack10 + 0x210);
        uVar7 = (uStack10 + 0x212);
        uStack14 = u_var1;
        if ((uVar7 | uStack14) != 0x0) {
            u_var2 = (uStack14 + 0xa);
            u_var4 = pass1_1030_dfcc(param_1);
            if (u_var4 != 0x0) {
                uStack24 = 0x1;
                uStack22 = 0x0;
                while (CONCAT22(uStack22, uStack24) < u_var2) {
                    u_var6 = u_var2;
                    u_var10 = u_var4;
                    bad_1030_1312();
                    uVar8 = uVar7;
                    iVar5 = pass1_1030_cde8(u_var6, uVar7, u_var10);
                    if (-0x1 < iVar5) {
                        pass1_1030_cef8(u_var6 & 0xffff | uVar7 << 0x10,
                                        CONCAT22(extraout_dx, param_2), 0x1, iVar5);
                        (param_1 + 0x20) = (u_var6 + 0x4);
                        return;
                    }
                    lVar3 = CONCAT22(uStack22, uStack24) + 0x1;
                    uStack24 = lVar3;
                    uVar7 = uVar8;
                    // uStack22 = (lVar3 >> 0x10);
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_dfcc(param_1: u32) -> u16

{
    let i_var1: i16;
    let uStack4: u16;

    i_var1 = (param_1 + 0xc);
    if (i_var1 == 0x73) {
//LAB_1030_dfde:
        uStack4 = 0x1;
    } else {
        if (i_var1 != 0x74) {
            if (i_var1 == 0x75) {
                return 0x3;
            }
            if (i_var1 == 0x77) {
                // goto
                // LAB_1030_dfde;
            }
            if (i_var1 != 0x78) {
                return 0x0;
            }
        }
        uStack4 = 0x2;
    }
    return uStack4;
}



astruct_18 *  pass1_1030_e010(param_1: & mut Struct18,param_2: u8)

{
let in_AX: u16;

pass1_1030_dcf4(param_1, in_AX); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



astruct_100 *
pass1_1030_e09e(param_1: & mut Struct100,param_2: u16,param_3: u8)

{
struct_op_1028_d1dc(param_2, param_3, param_1, 0x2af7); param_1.field_0x0 = 0xe2ae; (param_1 + 0x2) = 0x1030; unk_str_op_1000_3d3e
((param_1 & 0xffff0000 | (param_1 + 0x8)),
s_SCAiInput_1050_5972); return param_1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_e0d4(param_1: U32Ptr, param_2: u16, param_3: i16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let extraout_dx: u16;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let i_var9: i16;
    let u_var10: u16;
    let puVar11: U32Ptr;
    let uStack42: u32;
    let local_1c: [u8; 8];
    let uStack20: u32;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u32;
    let i_stack6: i16;
    let uStack4: u16;

    puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x40, param_2, param_1, param_3);
    // uStack4 = (puVar11 >> 0x10);
    i_stack6 = puVar11;
    uStack10 = pass1_1008_b820(puVar11, i_stack6, uStack4);
    u_var3 = uStack10;
    u_var6 = (uStack10 >> 0x10) | u_var3;
    if (u_var6 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
        uStack14 = CONCAT22(u_var6, u_var3);
        uStack16 = ((u_var3 + 0x154) != 0x0);
        pass1_1008_5784(CONCAT22(param_2, local_1c), uStack10);
        loop {
            puVar4 = local_1c;
            pass1_1008_5b12(puVar4, param_2);
            uStack20 = CONCAT22(extraout_dx, puVar4);
            puVar7 = (extraout_dx | puVar4);
            if (puVar7 == 0x0) { break; }
            if ((puVar4 + 0x8) != 0x0) {
                u_var2 = (puVar4 + 0xa);
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
                puVar8 = puVar7;
                pu_var5 = puVar4;
                pass1_1038_354a(CONCAT22(puVar7, puVar4), puVar4, puVar7);
                if (pu_var5 != 0x0) {
                    // u_var10 = (uStack20 >> 0x10);
                    if (uStack16 == 0x0) {
                        i_var9 = (uStack20 + 0xe) * 0xc;
                        uStack42 = (i_var9 + 0x58c4);
                        u_var3 = (i_var9 + 0x58c8);
                    } else {
                        i_var9 = (uStack20 + 0xe) * 0xc;
                        uStack42 = (i_var9 + 0x58be);
                        u_var3 = (i_var9 + 0x58c2);
                    }
                    u_var6 = u_var3;
                    pass1_1038_35a8(CONCAT22(puVar7, puVar4),
                                    ((uStack20 + 0x10) * 0x2 + uStack42), u_var3,
                                    puVar8);
                    if (u_var6 != 0x0) {
                        // u_var10 = (uStack20 >> 0x10);
                        i_var9 = uStack20;
                        pi_var1 = (i_var9 + 0x10);
                        *pi_var1 = *pi_var1 + 0x1;
                        if (u_var3 <= (i_var9 + 0x10)) {
                            (i_var9 + 0x10) = 0x0;
                        }
                    }
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_e1f4(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0xe2ae;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}



astruct_18 *  pass1_1030_e282(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn struct_1030_e2be(param_1: &mut Struct100, param_2: u16, param_3: u32, param_4: u32,
                        param_5: u16, param_6: u8)

{
    let i_var1: &mut Struct217;
    let u_var1: u16;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x2af7);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x108 = param_4;
    i_var1.field_0x10c = param_3;
    i_var1.field_0x110 = param_2;
    param_1.field_0x0 = 0xe4ea;
    i_var1.field_0x2 = 0x1030;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_e300(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) -> u16

{
    let pu_var1: U32Ptr;

    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, param_2, param_3);
    pass1_1010_089e(param_4, pu_var1, (param_1 + 0x110), 0x2);
    return 0x1;
}


pub fn pass1_1030_e328(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u8) -> u16

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x110) == 0x0) {
        pass1_1030_e4ba(param_1);
    } else {
        pass1_1030_e410(param_4, param_2, param_5, param_3,
                        param_1 & 0xffff | u_var1 << 0x10);
    }
    return 0x1;
}


pub fn pass1_1030_e34e(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let in_AX: &mut Struct404;
    let i_var3: i16;
    let iVar5: &mut Struct403;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x112, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        (param_2 + 0x4) = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = (param_2 + 0x8);
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        (param_2 + 0x108) = iVar5.field_0x108;
        (param_2 + 0x10c) = iVar5.field_0x10c;
        (param_2 + 0x110) = iVar5.field_0x110;
        *puStack10 = 0xe4ea;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_e410(param_1: u16, param_2: u16, param_3: u8, param_4: u16, param_5: u32) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let local_10: [u8; 6];
    let local_a: [u8; 4];
    let uStack6: u16;
    let uStack4: u16;

    u_var1 = (param_5 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    pu_var2 = (param_4 | param_2);
    if (pu_var2 != 0x0) {
        uStack6 = param_2;
        uStack4 = param_4;
        pass1_1038_4fd8(param_2, CONCAT22(param_4, param_2), 0x21);
        if (param_2 == 0x0) {
            pass1_1020_a43e(param_1, pu_var2, CONCAT22(param_1, local_a));
            puVar4 = pass1_1008_3e54(CONCAT22(param_1, local_10), 0x0, 0x2, 0xfffd);
            // u_var3 = (puVar4 >> 0x10);
            pass1_1020_a49a(param_1, param_3, u_var3, CONCAT22(param_1, local_a),
                            CONCAT22(param_1, local_10), 0x7a);
            pass1_1008_3e76(CONCAT22(param_1, local_10), 0x0, 0x3, 0xfffe);
            pass1_1020_a49a(param_1, param_3, u_var3, CONCAT22(param_1, local_a),
                            CONCAT22(param_1, local_10), 0x7a);
            pass1_1008_3e76(CONCAT22(param_1, local_10), 0x0, 0x3, 0xfffd);
            pass1_1020_a49a(param_1, param_3, u_var3, CONCAT22(param_1, local_a),
                            CONCAT22(param_1, local_10), 0x21);
        }
    }
    return;
}


pub fn pass1_1030_e4ba() {
    return;
}


pub fn pass1_1030_e540() -> u16

{
    return 0x1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_e546(param_1: u32, param_2: u16) -> u16

{
    let u_var1: u32;

    u_var1 = (param_1 + 0x108);
    pass1_1028_e332(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10), param_2);
    return 0x1;
}


pub fn pass1_1030_e564(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct405;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10c, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        (param_2 + 0x4) = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = (param_2 + 0x8);
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        (param_2 + 0x108) = iVar5.field_0x108;
        *puStack10 = 0xe62e;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}



astruct_18 *  pass1_1030_e602(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



astruct_100 *
pass1_1030_e63e(param_1: & mut Struct100,param_2: u16,param_3: u16,param_4: u8)

{
let i_var1: i16; let u_var2: u16;

i_var1 = param_1;
// u_var2 = (param_1 >> 0x10);
struct_op_1028_d1dc(param_3, param_4,param_1, 0xf9f); (i_var1 + 0x108) = param_2; param_1.field_0x0 = 0xe78a; (i_var1 + 0x2) = 0x1030; unk_str_op_1000_3d3e
((param_1 & 0xffff0000 | (i_var1 + 0x8)),
s_SCKillColony_1050_5990); return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_e67c(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) -> u16

{
    let u_var1: u16;
    let paVar2: &mut Struct67;

    paVar2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
    u_var1 = pass1_1008_aaa8(paVar2, (paVar2 >> 0x10),
                            (param_1 + 0x108));
    if (u_var1 != 0x0) {
        post_win_msg_1008_a0e4(paVar2, 0x0, 0x0, 0x1, 0x0, u_var1, 0x1008, param_4);
    }
    return 0x1;
}


pub fn pass1_1030_e6c2(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct406;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10a, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        (param_2 + 0x4) = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = (param_2 + 0x8);
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        (param_2 + 0x108) = iVar5.field_0x108;
        *puStack10 = 0xe78a;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}



astruct_18 *  pass1_1030_e75e(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



astruct_100 *
pass1_1030_e79a(param_1: & mut Struct100,param_2: u16,param_3: u8)

{
struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f); param_1.field_0x0 = 0xe890; (param_1 + 0x2) = 0x1030; unk_str_op_1000_3d3e
((param_1 & 0xffff0000 | (param_1 + 0x8)),
s_SCKillRebelColony_1050_599e); return param_1;
}



pub fn pass1_1030_e7d0() -> u16

{
    return 0x1;
}


pub fn pass1_1030_e7d6(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0xe890;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}



astruct_18 *  pass1_1030_e864(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1030_e8a0(param_1: &mut Struct100, param_2: u32, param_3: u16, param_4: u32,
                       param_5: u16, param_6: u8)

{
    let i_var1: &mut Struct408;
    let pu_var1: U32Ptr;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x2710);
    // pu_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x108 = param_2;
    i_var1.field_0x10c = param_4;
    i_var1.field_0x110 = param_3;
    param_1.field_0x0 = 0xeb40;
    i_var1.field_0x2 = 0x1030;
    sys_1000_3f9c(&i_var1.field_0x8, pu_var1, s_SCMoveBas_to_0x_08lx_1050_59b0,
                  ctx.data_seg, i_var1.field_0x10c, &stack0xfffe, pu_var1,
                  0x1000, param_5, param_6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16
pass1_1030_e8f8(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
param_6: u16)

{
let u_var1: u16; let u_var2: u16;
let u_var3: u32; let i_var4: i16;
let u_var5: u16; let u_var6: u32;
let paStack20: & mut Struct18; let uStack6: u32;

// u_var5 = (param_1 >> 0x10);
i_var4 = param_1; if ((i_var4 + 0x108) != 0x0) {
u_var3 = (i_var4 + 0x10c); pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3, (u_var3 > > 0x10)); uStack6 = CONCAT22(param_3,param_2); u_var6 = struct_op_1030_73a8(CONCAT22(param_3, param_2)); if ((u_var6 + 0xc) == (i_var4 + 0x110)) {
pass1_1030_ea50(param_1, uStack6, param_4, param_5,param_6);
}
u_var1 = (i_var4 + 0x108); u_var2 = (i_var4 + 0x10a); paStack20 = CONCAT22(u_var2, u_var1); if ((u_var2 | u_var1) != 0x0) {
fn_ptr_1020_ba7e(CONCAT22(u_var2, u_var1));
fn_ptr_1000_17ce(paStack20, 0x1000);
}
(i_var4 + 0x108) = 0x0;
}
return 0x1;
}



pub fn pass1_1030_e98e(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct407;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x112, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        (param_2 + 0x4) = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = (param_2 + 0x8);
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        (param_2 + 0x108) = iVar5.field_0x108;
        (param_2 + 0x10c) = iVar5.field_0x10c;
        (param_2 + 0x110) = iVar5.field_0x110;
        *puStack10 = 0xeb40;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_ea50(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let BVar2: bool;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;
    let local_12: u32;
    let local_e: u16;
    let iStack12: i16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u32;

    uStack6 = 0x1869f;
    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (i_var3 + 0x110), 0x3);
    if (BVar2 != 0x0) {
        u_var5 = struct_op_1030_73a8(param_2);
        // iStack12 = (u_var5 >> 0x10);
        local_e = u_var5;
        uStack6 = pass1_1028_45e2(u_var5, local_e, iStack12, param_5);
    }
    u_var1 = (i_var3 + 0x108);
    uStack8 = (u_var1 + 0x4);
    uStack10 = 0x0;
    loop {
        if (uStack8 <= uStack10) {
            return;
        }
        pass1_1020_bb16((i_var3 + 0x108), CONCAT22(param_5, &local_12),
                        CONCAT22(param_5, &local_e), uStack10);
        if (uStack6 < local_12) {
            pass1_1030_7ddc(param_2, uStack6, local_e, uStack6, uStack6._2_2_, param_3, param_4,
                            param_5);
            uStack6 = 0x0;
        } else {
            uStack6 -= local_12;
            pass1_1030_7ddc(param_2, local_12, local_e, local_12, uStack6._2_2_, param_3,
                            param_4, param_5);
        }
        if ((uStack6._2_2_ | uStack6) == 0x0) { break; }
        uStack10 += 0x1;
    }
    return;
}



astruct_18 *  pass1_1030_eb14(param_1: & mut Struct18,param_2: u8)

{
param_1.field_0x0 = 0x389a; (param_1 + 0x2) = 0x1008; if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



astruct_100 *
pass1_1030_eb50(param_1: & mut Struct100,param_2: u16,param_3: u8)

{
struct_op_1028_d1dc(param_2, param_3, param_1, 0x1f3f); param_1.field_0x0 = 0xecb2; (param_1 + 0x2) = 0x1030; unk_str_op_1000_3d3e
((param_1 & 0xffff0000 | (param_1 + 0x8)),
s_SCMines_1050_59c6); return param_1;
}



pub fn pass1_1030_eb86(param_1: u16, param_2: u16) -> u16

{
    let i_var1: i16;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let extraout_dx: u16;
    let puStack24: u32;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x700);
    loop {
        u_var4 = param_1;
        pu_var3 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var3));
        puStack24 = CONCAT22(u_var4, pu_var3);
        param_1 = u_var4 | pu_var3;
        if (param_1 == 0x0) { break; }
        if ((pu_var3 + 0x12) == 0x5) {
            i_var1 = (pu_var3 + 0xc);
            if (((0x32 < i_var1) && (!SBORROW2(i_var1, 0x33))) && ((i_var1 == 0x34 || i_var1 + -0x33 < 0x1 || ((0x2b < i_var1 + -0x34 && (i_var1 + -0x60 < 0x2)))))) {
                ppcVar2 = (*puStack24 + 0x2c);
                (**ppcVar2)(&USHORT_1050_1028);
                param_1 = extraout_dx;
            }
        }
    }
    return 0x1;
}


pub fn pass1_1030_ebf8(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0xecb2;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}


pub fn pass1_1030_ec86(param_1: &mut Struct18, param_2: u8)

{
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}



astruct_100 *
pass1_1030_ecc2(param_1: & mut Struct100,param_2: u16,param_3: u8)

{
struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f); param_1.field_0x0 = 0xb96; (param_1 + 0x2) = & ctx.PTR_LOOP_1050_1038; unk_str_op_1000_3d3e
((param_1 & 0xffff0000 | (param_1 + 0x8)),
s_SCMorale_1050_59ce); return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1030_ecf8(param_1: u32, param_2: u32, param_3: i16, param_4: u16, param_5: u8) {
    let i_var1: i16;
    let pu_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let u_var5: u32;
    let puVar6: U32Ptr;
    let iVar7: i16;
    let uVar8: u32;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u32;
    let uVar14: u16;
    let bVar15: bool;
    let puVar16: U32Ptr;
    let puVar17: u32;
    let uVar18: u16;
    let uStack64: u32;
    let iStack56: i16;
    let uStack54: u16;
    let uStack38: u32;
    let local_22: [u8; 12];
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let uStack4: u16;

    uStack12 = 0x0;
    puVar16 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, param_2, param_3);
    uVar13 = param_2 & 0xffff0000 | puVar16 >> 0x10;
    uStack10 = puVar16;
    // uStack4 = (puVar16 >> 0x10);
    uStack6 = uStack10;
    pass1_1010_ed3e(puVar16);
    uStack8 = uVar13;
    uVar13 = uVar13 & 0xffff0000 | (uStack8 | uStack10);
    if ((uStack8 | uStack10) != 0x0) {
        uStack12 = pass1_1030_2aaa(CONCAT22(uStack8, uStack10));
    }
    if (uStack12 < 0x2) {
        uStack12 = 0x0;
    }
    puVar16 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, uVar13, param_3);
    uVar13 = uVar13 & 0xffff0000 | puVar16 >> 0x10;
    uStack16 = SUB42(puVar16, 0x0);
    // uStack14 = (puVar16 >> 0x10);
    if ((0x0 < ctx.PTR_LOOP_1050_13ae) && (!SBORROW2(ctx.PTR_LOOP_1050_13ae, 0x1))) {
        if (ctx.PTR_LOOP_1050_13ae == &ctx.PTR_LOOP_1050_0002 || (ctx.PTR_LOOP_1050_13ae + -0x1) < 0x1) {
            if (0x6 < uStack12) {
                uStack12 -= 0x2;
//         TODO: goto LAB_1030_ed5b;
            }
            bVar15 = SBORROW2(uStack12, 0x4);
            i_var1 = uStack12 - 0x4;
        } else {
            if (ctx.PTR_LOOP_1050_13ae != (&ctx.PTR_LOOP_1050_0002 + 0x1))
//       TODO: goto LAB_1030_ed5b;
            bVar15 = SBORROW2(uStack12, 0x7);
            i_var1 = uStack12 - 0x7;
        }
        if (bVar15 == i_var1 < 0x0) {
            uStack12 -= 0x1;
        }
    }
//LAB_1030_ed5b:
    pass1_1028_dc52(
        CONCAT13((param_4 >> 0x8), CONCAT12(param_4, local_22)), 0x1,
        0x0, 0x400);
    loop {
        puVar6 = local_22;
        pass1_1028_e4ec(CONCAT22(param_4, puVar6));
        uVar9 = uVar13;
        uStack38 = CONCAT22(uVar9, puVar6);
        if ((uVar9 | puVar6) == 0x0) { break; }
        u_var10 = (puVar6 + 0x1f6);
        uVar13 = uVar13 & 0xffff0000 | (puVar6 + 0x1f8);
        if (((puVar6 + 0x1fe) != 0x0) && ((puVar6 + 0x200) != 0x8000002)) {
            pass1_1030_38b8();
            u_var10 = uVar13 | u_var10;
            uVar8 = uVar13 & 0xffff0000;
            uVar13 = uVar8 | u_var10;
            if (u_var10 != 0x0) {
                pu_var2 = (puVar6 + 0xc);
                u_var10 = (puVar6 + 0xe);
                uVar8 |= u_var10;
                ppc_var3 = (*pu_var2 + 0x10);
                puVar17 = pu_var2;
                (**ppc_var3)(&USHORT_1050_1028, pu_var2, u_var10);
                u_var5 = puVar17 & 0xffff | uVar8 << 0x10;
                uStack54 = (puVar6 + 0x18);
                uVar14 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
                pass1_1038_4760(CONCAT22(uVar9, puVar6));
                i_var1 = (puVar6 + 0x22);
                iStack56 = i_var1 / 0xa;
                uVar13 = uVar8 & 0xffff0000 | i_var1 % 0xa & 0xffff;
                i_var1 = (puVar6 + 0x24);
                if (i_var1 < 0x33) {
                    if (i_var1 < 0x32) {
                        iStack56 += -0x1;
                    }
                } else {
                    uStack54 += 0x1;
                }
                // for (uStack64 = 0x0; uStack64 < u_var5; uStack64 += 0x1) {
                //   ppc_var3 = (*pu_var2 + 0x4);
                //   uVar8 = u_var5;
                //   (**ppc_var3)(uVar14,pu_var2,(pu_var2 >> 0x10),uStack64,
                //               (uStack64 >> 0x10));
                //   u_var10 = uVar8;
                //   u_var11 = uVar13;
                //   uVar12 = u_var11 | u_var10;
                //   uVar13 = uVar13 & 0xffff0000 | uVar12;
                //   if (uVar12 != 0x0) {
                //     uVar14 = SUB42(&USHORT_1050_1028,0x0);
                //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var10,u_var11);
                //     puVar17 = struct_op_1030_73a8(CONCAT22(uVar13,u_var10));
                //     u_var10 = puVar17;
                //     u_var11 = (puVar17 >> 0x10);
                //     uVar13 = uVar13 & 0xffff0000 | (u_var11 | u_var10);
                //     if (((u_var11 | u_var10) != 0x0) && ((u_var10 + 0x12) == 0x5)) {
                //       ppc_var3 = (*puVar17 + 0x48);
                //       (**ppc_var3)(&USHORT_1050_1028,u_var10,u_var11);
                //       if (u_var10 < 0x0) {
                //         iStack56 += u_var10;
                //       }
                //       else {
                //         uStack54 += u_var10;
                //       }
                //     }
                //   }
                // }
                iStack56 -= uStack12;
                i_var1 = (puVar6 + 0x20a);
                // uVar18 = (param_1 >> 0x10);
                u_var4 = param_1;
                iVar7 = i_var1;
                pass1_1038_01c0(u_var4, uVar18, uStack38, param_4);
                iVar7 -= i_var1;
                iStack56 -= iVar7;
                pass1_1038_008e(u_var4, uVar18, uStack38, uVar13, param_3, param_4);
                if (iVar7 < 0x0) {
                    iStack56 += iVar7;
                } else {
                    uStack54 += iVar7;
                }
                if (0x3e8 < uStack54) {
                    uStack54 = 0x3e8;
                }
                if (uStack54 < 0x0) {
                    uStack54 = 0x0;
                }
                uStack54 += iStack56;
                if (0x3e8 < uStack54) {
                    uStack54 = 0x3e8;
                }
                if (uStack54 < 0x0) {
                    uStack54 = 0x0;
                }
                pass1_1038_4d0e(uStack38, uStack54);
                if ((puVar6 + 0x4) == 0x4000001) {
                    pass1_1038_08d4(u_var4, CONCAT22(uStack54, uVar18), uStack38, uVar13, param_4, param_5);
                }
                pass1_1038_095e(u_var4, uVar18, uStack54, uStack38, uVar13, param_3, param_4);
            }
        }
    }
    return;
}



