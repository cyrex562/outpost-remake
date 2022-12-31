

use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708, mem_op_1000_160a, mem_op_1000_179c};
use crate::unk::block_1000_3000::{pass1_1000_3cea, sys_1000_3f9c};
use crate::unk::block_1000_4000::{pass1_1000_484c, pass1_1000_4906};
use crate::unk::block_1008_3000::{pass1_1008_3e54, pass1_1008_3e94, pass1_1008_3f62};
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::unk::block_1008_7000::switch_1008_73ea;
use crate::unk::block_1008_9000::{pass1_1008_9d36, struct_1008_9fd2};
use crate::unk::block_1008_a000::{pass1_1008_aefe, pass1_1008_af94};
use crate::unk::block_1008_c000::pass1_1008_c72a;
use crate::unk::block_1008_d000::{pass1_1008_d72e, pass1_1008_d790, pass1_1008_d99e, struct_1008_dd4e};
use crate::unk::block_1008_e000::{pass1_1008_eabc, pass1_1008_eb2a, pass1_1008_ec10, struct_1008_ecb2};
use crate::unk::block_1010_0000::{pass1_1010_0000, pass1_1010_0eac, struct_1010_02e0};
use crate::unk::block_1010_1000::{pass1_1010_1146, pass1_1010_195e, pass1_1010_1b6e, pass1_1010_1d80, pass1_1010_1f62, struct_op_1010_1d48};
use crate::unk::block_1010_3000::{pass1_1010_32f4, pass1_1010_35a4, pass1_1010_3702, pass1_1010_3d82, pass1_1010_3e3c, struct_1010_3b7a};
use crate::unk::block_1010_4000::pass1_1010_4a8a;
use crate::unk::block_1010_5000::{pass1_1010_503e, struct_1010_50b2};
use crate::unk::block_1010_6000::{pass1_1010_6700, pass1_1010_6abc, pass1_1010_6ca2, struct_1010_6326};
use crate::unk::block_1010_7000::pass1_1010_715c;
use crate::unk::block_1010_8000::{FUN_1010_830a, pass1_1010_8170, pass1_1010_8c32};
use crate::unk::block_1010_9000::{pass1_1010_9298, struct_1010_95aa};
use crate::unk::block_1010_a000::struct_1010_a1d8;
use crate::unk::block_1010_e000::{pass1_1010_e964, struct_1010_e9e4};
use crate::unk::block_1018_0000::struct_1018_0570;
use crate::unk::block_1018_1000::{pass1_1018_18b8, pass1_1018_1ff4};
use crate::unk::block_1018_2000::{struct_1018_229c, struct_1018_2b10};
use crate::unk::block_1018_3000::pass1_1018_331c;
use crate::unk::block_1018_4000::{pass1_1018_4aaa, pass1_1018_4dce, struct_op_1018_4cda};
use crate::unk::block_1018_5000::{pass1_1018_5070, pass1_1018_56e6};
use crate::dos_interrupt::swi;
use crate::draw_ops::unk_draw_op_1008_da12;
use crate::draw_ops::draw_a::palette_op_1008_4e08;
use crate::draw_ops::draw_e::clenaup_win_ui_1018_4d22;
use crate::file_ops::{read_file_1008_7dee, write_to_file_1008_7e1c};
use crate::resources::load_string_1010_84e0;
use crate::structs::struct_19::Struct19;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_638::Struct638;
use crate::structs::struct_d::StructD;
use crate::sys_ops::win_sys_op_1010_5404;
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::winapi16::{CreateDC16, DeleteDC16, DeleteObject16, DestroyWindow16, Rectangle16, SelectObject16, SelectPalette16, WritePrivateProfileString16};
use crate::windef16::{DEVMODEA, HDC16, HFILE16, HGDIOBJ16, HPALETTE16};

pub fn pass1_1010_2024(param_1: *mut StructD) -> *mut StructD {
    let mut struct_1_hi: u16;

    struct_1_hi = (param_1 >> 0x10);
    (param_1 + 0x124) = 0;
    _u16_1050_0ed0 = param_1;
    pass1_1000_4906((param_1 & 0xffff | struct_1_hi << 0x10), NULL, 0x124);
    return param_1;
}


pub fn pass1_1010_2050(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;
    let mut iStack4: i16;

    pass1_1010_2816(param_1);
    iStack4 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        puVar1 = (iStack4 * 0x4 + param_1);
        uVar2 = (iStack4 * 0x4 + param_1 + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iStack4 += 0x1;
        if iStack4 >= 0x49 {
            break;
        }
    }
    _u16_1050_0ed0 = 0;
    return;
}
pub fn pass1_1010_209e(param_1: u32, mut param_2: u16) {
    pass1_1010_2816(param_1);
    (param_1 + 0x124) = param_2;
    return;
}

pub fn mixed_1010_20ba(
    param_1: *mut Struct57,
    mut param_2: u32,
    param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) -> *mut u32 {
    let mut var1: *mut *mut code;
    let mut var2: *mut u8;
    let mut var3: u16;
    let mut var4: u16;
    let mut var5: *mut Struct638;
    // let mut unaff_SI: u16;
    let mut var6: i16;
    let mut var7: u16;
    let mut var8: u16;
    let mut var9: *mut u16;
    let mut var10: u32;
    let mut var11: *mut u16;
    let mut var12: u32;
    let mut var13: *mut Struct19;
    let mut var14: u16;
    let mut var15: u16;
    let mut var16: u16;
    let mut var17: u16;
    let mut var18: *mut u32;

    pass1_1010_2816(param_2);
    var5 = (param_3 * 0x4);
    // var7 = (param_2 >> 0x10);
    var6 = param_2;
    var18 = (&var5.field_0x0 + var6);
    if var18.is_null() == false {
        return var18;
    }
    match param_3 {
        0x1 => {
            mem_op_1000_179c(0x18, param_1);
            var2 = (param_1 | var5);
            if (var2.is_null()) {
                //
                // LAB_1010_2126:
                var5 = null_mut();
                var2 = null_mut();
            } else {
                struct_1010_3b7a(CONCAT22(param_1, var5), param_3);
            }
        }
        // break;
        0x2 => {
            mem_op_1000_179c(0x84, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            win_sys_op_1010_5404(param_4, CONCAT22(param_1, var5), param_3);
        }
        // break;
        0x3 => {
            mem_op_1000_179c(0x53c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_a1d8(CONCAT22(param_1, var5), param_3);
        }

        0x4 => {
            mem_op_1000_179c(0x18a, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1018_2b10(CONCAT22(param_1, var5), param_3);
        }

        0x5 => {
            mem_op_1000_179c(0x14, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var11 = pass1_1008_eabc(CONCAT22(param_1, var5), param_3);
            var2 = (var11 >> 0x10);
            var5 = var11;
        }

        0x6 => {
            mem_op_1000_179c(0x58, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_18b8(CONCAT22(param_1, var5), param_3);
        }

        0x7 => {
            mem_op_1000_179c(0xe, param_1);
            var4 = param_1 | var5;
            //     if (uVar4 == 0) goto LAB_1010_2126;
            var10 = pass1_1010_3d82(param_1 & 0xffff0000 | var4, var5, param_1, param_3);
            var2 = (var10 >> 0x10);
            var5 = var10;
        }

        0x8 => {
            mem_op_1000_179c(0x20, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_95aa(CONCAT22(param_1, var5), param_3);
        }

        0x9 => {
            mem_op_1000_179c(0x26, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_6326(CONCAT22(param_1, var5), param_3);
        }

        0xa => {
            mem_op_1000_179c(0x1c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            var13 = pass1_1010_0eac(var2, CONCAT22(param_1, var5), param_3);
            var2 = (var13 >> 0x10);
            var5 = var13;
        }

        0xb => {
            mem_op_1000_179c(0x1c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            var10 = pass1_1008_aefe(var2, CONCAT22(param_1, var5), param_3);
            var2 = (var10 >> 0x10);
            var5 = var10;
        }

        0xc | 0xd | 0xe | 0xf | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18
        | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 => {
            mem_op_1000_179c(0xaa, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1018_0570(CONCAT22(param_1, var5), param_3, param_5);
        }

        0x25 => {
            mem_op_1000_179c(0x1c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_4aaa(var2, CONCAT22(param_1, var5), param_3);
        }

        0x26 => {
            mem_op_1000_179c(0x1c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_d99e(var2, CONCAT22(param_1, var5), param_3);
        }

        0x27 => {
            mem_op_1000_179c(0x58, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_9d36(var5, param_1, param_3);
        }

        0x28 => {
            mem_op_1000_179c(0x2c, param_1);
            var4 = param_1 | var5;
            var10 = param_1 & 0xffff0000 | var4;
            //     if (uVar4 == 0) goto LAB_1010_2126;
            pass1_1010_28e6(var10, 0x1000, var5, param_1, param_3);
            var2 = var10;
        }

        0x29 => {
            mem_op_1000_179c(0x72, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1018_229c(var2, CONCAT22(param_1, var5), param_3);
        }

        0x2a => {
            mem_op_1000_179c(0x1c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1010_503e(var2, CONCAT22(param_1, var5), param_3);
        }

        0x2b => {
            mem_op_1000_179c(0x1a, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_02e0(CONCAT22(param_1, var5), param_3);
        }

        0x2c => {
            mem_op_1000_179c(0x10, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_eb2a(CONCAT22(param_1, var5), param_3);
        }

        0x2d => {
            mem_op_1000_179c(0x80, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1010_3e3c(CONCAT22(param_1, var5), param_3, param_6);
        }

        0x2e => {
            mem_op_1000_179c(0x806, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var10 = pass1_1018_1ff4(CONCAT22(param_1, var5), param_3);
            var2 = (var10 >> 0x10);
            var5 = var10;
        }

        0x2f => {
            mem_op_1000_179c(0x58, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_e9e4(CONCAT22(param_1, var5), param_3);
        }

        0x30 => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var9 = pass1_1010_3702(CONCAT22(param_1, var5), param_3);
            var2 = (var9 >> 0x10);
            var5 = var9;
        }

        0x31 => {
            var8 = 0x1000;
            mem_op_1000_179c(0x60, param_1);
            var4 = param_1 | var5;
            if (var4 == 0) {
                //
                // LAB_1010_2680:
                var8 = 0x1000;
                var5 = null_mut();
                var2 = null_mut();
            } else {
                var10 =
                    pass1_1010_9298(CONCAT22(var5, var4), CONCAT22(param_1, var5), param_3);
                var2 = (var10 >> 0x10);
                var5 = var10;
            }
        }
        // TODO: goto LAB_1010_2683;
        0x32 => {
            mem_op_1000_179c(0x26, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1010_6abc(CONCAT22(param_1, var5), param_3, param_6);
        }

        0x33 => {
            mem_op_1000_179c(0x72, param_1);
            var4 = param_1 | var5;
            if (var4 == 0) {
                //
                // LAB_1010_25b8:
                var8 = 0;
                var3 = 0;
            } else {
                var13 = pass1_1010_195e(param_1 & 0xffff0000 | var4, var5, param_1, param_3);
                var3 = (var13 >> 0x10);
                var8 = SUB42(var13, 0x0);
            }
        }
        // TODO: goto LAB_1010_25bb;
        0x34 => {
            mem_op_1000_179c(0x72, param_1);
            var4 = param_1 | var5;
            //     if (uVar4 == 0) goto LAB_1010_25b8;
            var13 = pass1_1010_1b6e((param_1 & 0xffff0000 | var4), var5, param_1, param_3);
            var3 = (var13 >> 0x10);
            var8 = SUB42(var13, 0x0); //
                                         // LAB_1010_25bb:
            var18 = CONCAT22(var3, var8);
            pass1_1010_1146(CONCAT22(var3, var8), 0x0, );
        }
        // TODO: goto switchD_1010_2765_caseD_38;
        0x35 => {
            mem_op_1000_179c(0x14a, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var13 = pass1_1010_6700(CONCAT22(param_1, var5), param_3);
            var2 = (var13 >> 0x10);
            var5 = var13;
        }

        0x36 => {
            mem_op_1000_179c(0x10, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_d790(var5, param_1, param_3);
        }

        0x37 => {
            mem_op_1000_179c(0x420, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1008_9fd2(CONCAT22(param_1, var5), param_3);
        }

        _ => {}
        // TODO: goto switchD_1010_2765_caseD_38;
        0x39 => {
            mem_op_1000_179c(0x36, param_1);
            var4 = param_1 | var5;
            var10 = param_1 & 0xffff0000 | var4;
            //     if (uVar4 == 0) goto LAB_1010_2126;
            pass1_1010_4a8a(
                var10,
                var5,
                param_1,
                param_3,
                param_7,
                var14,
                var15,
                var16,
                var17,
            );
            var2 = var10;
        }

        0x3a => {
            mem_op_1000_179c(0xc, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var9 = pass1_1008_d72e(CONCAT22(param_1, var5), param_3);
            var2 = (var9 >> 0x10);
            var5 = var9;
        }

        0x3b => {
            mem_op_1000_179c(0x22, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1008_dd4e(CONCAT22(param_1, var5), param_3);
        }

        0x3c => {
            mem_op_1000_179c(0x146, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_331c(var2, var5, param_1, param_3);
        }

        0x3d => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var10 = pass1_1010_8c32(CONCAT22(param_1, var5), param_3);

            var2 = (var10 >> 0x10);
            var5 = var10;
        }
        0x3e => {
            mem_op_1000_179c(0x18, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_5070(CONCAT22(param_1, var5), param_3);
        }

        0x3f => {
            mem_op_1000_179c(0x12, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_c72a(CONCAT22(param_1, var5), param_3, unaff_SI);
        }

        0x40 => {
            mem_op_1000_179c(0x24, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_af94(CONCAT22(param_1, var5), param_3, unaff_SI);
        }
        0x41 => {
            mem_op_1000_179c(0x60, param_1);
            var4 = param_1 | var5;
            //     if (uVar4 == 0) goto LAB_1010_2680;
            var8 = 0x1008;
            var12 = struct_1008_ecb2(var5, var4, CONCAT22(param_1, var5), param_3);
            var2 = (var12 >> 0x10);
            var5 = var12; //
                             // LAB_1010_2683:
            (param_3 * 0x4 + var6) = var5;
            (param_3 * 0x4 + var6 + 0x2) = var2;
            var1 = (var5 + 0x10);
            (**var1)(var8, var5, var2);
        }

        0x42 => {
            mem_op_1000_179c(0xc, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var9 = pass1_1008_ec10(var5, param_1, param_3);
            var2 = (var9 >> 0x10);
            var5 = var9;
        }

        0x43 => {
            mem_op_1000_179c(0x12, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var9 = pass1_1010_2bfc(CONCAT22(param_1, var5), param_3);
            var2 = (var9 >> 0x10);
            var5 = var9;
        }
        0x45 => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var13 = pass1_1010_0000(CONCAT22(param_1, var5), param_3);
            var2 = (var13 >> 0x10);
            var5 = var13;
        }
        0x46 => {
            mem_op_1000_179c(0x1a, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_50b2(CONCAT22(param_1, var5), param_3);
        }
        0x47 => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            var9 = pass1_1018_56e6(CONCAT22(param_1, var5), param_3);
            var2 = (var9 >> 0x10);
            var5 = var9;
        }

        0x48 => {
            mem_op_1000_179c(0x1c, param_1);
            var2 = (param_1 | var5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            unk_draw_op_1008_da12(CONCAT22(param_1, var5), param_3);
        }
    };
    var18 = CONCAT22(var2, var5);
    // switchD_1010_2765_caseD_38:
    (param_3 * 0x4 + var6) = var18;
    return var18;
}
pub fn pass1_1010_2816(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x124) != 0) {
        iVar5 = (iVar4 + 0x124) * 0x4;
        puVar1 = (iVar5 + iVar4);
        uVar2 = (iVar5 + iVar4 + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        ((iVar4 + 0x124) * 0x4 + iVar4) = 0;
        (iVar4 + 0x124) = 0;
    }
    return;
}

pub fn pass1_1010_286c() -> u16 {
    let mut puVar1: *mut u16;

    pass1_1008_3e54(&PTR_LOOP_1048_0000, 0x0, 0x5, 0x12c);
    pass1_1008_3e54((s__1050_65a0 + 0x6), 0x0, 0x9b, 0x20);
    pass1_1008_3e54(0x105065ac, 0x0, 0xf5, 0x3f);
    pass1_1008_3e54(0x105065b2, 0x0, 0x114, 0x4a);
    pass1_1008_3e54(0x105065b8, 0x0, 0x135, 0x45);
    pass1_1008_3e54(0x105065be, 0x0, 0xf5, 0x7b);
    puVar1 = pass1_1008_3e54(0x105065c4, 0x0, 0x117, 0x91);
    return puVar1;
}
pub fn pass1_1010_28e6(
    mut param_1: u32,
    mut param_2: u16,
    param_3: *mut Struct19,
    param_4: *mut Struct19,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut puVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut unaff_DS: u16;
    let mut iStack6: i16;

    struct_op_1018_4cda(CONCAT22(param_4, param_3), param_5);
    uVar5 = 0;
    param_3.field15_0x1c = 0;
    param_3.field17_0x20 = 0;
    param_3.field18_0x22 = 0;
    param_3.field19_0x24 = 0;
    param_3.field20_0x26 = 0;
    // 0x2bdc
    CONCAT22(param_4, param_3) = 0x2be4;
    param_3.segment_0x2 = 0x1010;
    PTR_LOOP_1050_4230 = param_3;
    0x4232 = param_4;
    pass1_1018_4dce(param_1, CONCAT22(param_4, param_3), 0x56);
    uVar2 = FUN_1010_830a(uVar5, param_1, 0x1018, &u32_1048_14cc, 0x4);
    param_3.field15_0x1c = uVar2;
    param_3.field16_0x1e = param_1;
    if (0x5f2c == 0) {
        puVar2 = mem_op_1000_160a(param_1);
    } else {
        puVar2 = 0x5f2c;
        param_1 = param_1 & 0xffff0000 | 0x5f2e;
    }
    uVar4 = fn_ptr_op_1000_1708(0x40, 0x0, 0x1, puVar2, param_1);
    (param_3.field20_0x26 + 0x2) = uVar4;
    param_3.field21_0x2a = param_1;
    //   for (iStack6 = 0; iStack6 < 0x10; iStack6 += 1)
    for iStack6 in 0..0x10 {
        uVar5 = FUN_1010_830a(
            iStack6 + 0x56,
            param_1,
            0x1000,
            &u32_1048_14cc,
            iStack6 + 0x56,
        );
        uVar1 = (param_3.field20_0x26 + 2);
        uVar7 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        (iVar6 + iStack6 * 0x4) = uVar5;
        (iVar6 + iStack6 * 0x4 + 0x2) = param_1;
    }
    return;
}
pub fn pass1_1010_29c6(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar5: *mut StructD;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.address_offset_field_0x0 = s_add16_wav_1050_2bdc + 0x8;
    iVar5.address_offset_field_0x2 = 0x1010;
    if (&iVar5.field_0x1c != 0) {
        puVar1 = &iVar5.field_0x1c;
        uVar2 = &iVar5.field_0x1e;
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iVar5.field_0x1c = 0;
        fn_ptr_1000_17ce(*&iVar5.field_0x28);
        iVar5.field_0x28 = 0;
    }
    clenaup_win_ui_1018_4d22(param_1);
    return;
}

// WARNING: Instruction at (ram,0x10104b2b) overlaps instruction at (ram,0x10104b2a)
//


// WARNING: Restarted to delay deadcode elimination for space: stack
pub fn pass1_1010_2b50(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, &PTR_LOOP_1048_0000);
    return;
}

pub fn pass1_1010_2b66(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1e), (param_1 + 0x1c));
}
pub fn pass1_1010_2b78(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u32,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;

    puVar3 = (param_3 * 0x7c + 0xed4);
    puVar5 = param_4;
    //   for (iVar4 = 0x1f; iVar4 != 0; iVar4 += -1)
    for iVar4 in 0x1f..0 {
        puVar2 = puVar5;
        puVar5 = puVar5 + 1;
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        *puVar2 = *puVar1;
    }
    return;
}

pub fn pass1_1010_2b98(mut param_1: u32, mut param_2: i16) -> *mut astruct_76 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar1 = (param_1 + 0x28);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22(
        (param_2 * 0x4 + iVar2 -0x156),
        (param_2 * 0x4 + iVar2 -0x158),
    );
}




pub fn pass1_1010_2bfc(param_1: *mut Struct19, mut param_2: u16) -> *mut u16 {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x10) = 0;
    param_1.offset_0x0 = 0x2cc2;
    (param_1 + 0x2) = 0x1010;
    return &param_1.offset_0x0;
}



pub fn unk_load_str_op_1010_2c34(param_1: u32) -> *mut c_char {
    let mut in_buffer_4: *mut c_char;
    let mut pUVar1: *mut u16;
    let mut in_buf_len_5: i16;
    let mut sVar2: i16;
    let mut in_EDX: *mut Struct57;
    let mut paVar3: *mut Struct57;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000fff6: u16;

    puVar4 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x3),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    paVar3 = (in_EDX & 0xffff0000 | puVar4 >> 0x10);
    in_buffer_4 = puVar4;
    mem_op_1000_179c(0x80, paVar3);
    in_buf_len_5 = paVar3;
    sVar2 = in_buf_len_5;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x80,
        in_buffer_4,
        in_buf_len_5,
    );
    pUVar1 = pass1_1000_3cea(CONCAT22(in_buf_len_5, in_buffer_4), s__1050_11c8);
    pass1_1010_e964(sVar2);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, in_buffer_4), CONCAT22(sVar2, pUVar1));
    return in_buffer_4;
}




pub fn struct_1010_2cd2(param_1: *mut Struct19, mut param_2: u16) {
    let mut in_EDX: u32;
    let mut uVar1: u16;
    let mut paVar2: *mut Struct19;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe82: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut piVar6: *mut i16;
    let mut uVar7: u16;
    let mut local_6: i16;
    let mut local_4: i16;

    uVar1 = (in_EDX >> 0x10);
    paVar2 = struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x18) = 0;
    (param_1 + 0x22) = 0;
    (param_1 + 0x24) = 0;
    (param_1 + 0x26) = 0;
    (param_1 + 0x28) = 0;
    (param_1 + 0x52) = 0;
    (param_1 + 0x56) = 0;
    (param_1 + 0x5a) = 0;
    (param_1 + 0x5e) = 0;
    (param_1 + 0x5c) = 0;
    param_1.offset_0x0 = 0x36da;
    (param_1 + 0x2) = 0x1010;
    piVar6 = &local_4;
    uVar7 = SUB42(0x1050, 0x0);
    piVar4 = &local_6;
    uVar5 = SUB42(0x1050, 0x0);
    puVar3 = mixed_1010_20ba(
        CONCAT22(uVar1, (paVar2 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(piVar4, 0x48),
        in_stack_0000fe82,
        in_stack_0000ffa6,
        in_stack_0000ffac,
        in_stack_0000ffb0,
    );
    pass1_1008_3e94(
        (puVar3 & 0xffff0000 | (puVar3 + 0xe)),
        CONCAT22(uVar5, piVar4),
        CONCAT22(uVar7, piVar6),
    );
    (param_1 + 0xe) = 0x19001db;
    (param_1 + 0xa) = 0x140 - ((param_1 + 0xe) / 0x2 - local_4);
    (param_1 + 0xc) = 0xf0 - ((param_1 + 0x10) / 0x2 - local_6);
    (param_1 + 0x1a) = 0xa006e;
    (param_1 + 0x1e) = 0xa012c;
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x2a)), NULL, 0x28);
    return;
}
pub fn pass1_1010_2db2(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar5: *mut astruct_455;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field0_0x0 = 0x36da;
    iVar5.field1_0x2 = 0x1010;
    puVar1 = iVar5[0xa].field3_0x6;
    uVar2 = (iVar5 + 0xb).field0_0x0;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*&iVar5[0xb].field2_0x4);
    pass1_1010_1d80(param_1);
    return;
}



pub fn pass1_1010_2e02(mut param_1: u32, mut param_2: i16) -> u32 {
    let mut iVar2: *mut astruct_163;
    let mut uVar2: u16;
    let mut uVar1: u32;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0) {
        uVar1 = (param_1 + 0x5c);
        uVar2 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + param_2 * 0x4 + 0x2), (iVar2 + param_2 * 0x4));
    }
    return 0x0;
}
pub fn pass1_1010_2e30(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: i16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0) {
        uVar1 = (param_1 + 0x5c);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        (iVar2 + param_4 * 0x4) = param_2;
        (iVar2 + param_4 * 0x4 + 0x2) = param_3;
    }
    return;
}
pub fn pass1_1010_2e5c(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uStack12: u32;

    uStack12 = param_3;
    if (param_3 == 0) {
        return;
    }
    //   for (; (uStack12 & 0xf) != 0; uStack12 >>= 0x4) {
    //   }
    while uStack12 & 0xf != 0 {
        uStack12 >>= 0x4;
    }
    return;
}
pub fn pass1_1010_2ee2(param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut extraout_DX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut paStack6: *mut Struct65;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x52) != 0) {
        return;
    }
    iVar2 = 0;
    (iVar3 + 0x28) = 0;
    uVar5 = *param_1;
    ppcVar1 = (uVar5 + 0x20);
    (**ppcVar1)();
    if (iVar2 == 0) {
        paStack6 = (iVar3 + 0x56);
    } else {
        ppcVar1 = (uVar5 + 0x14);
        (**ppcVar1)();
        paStack6 = CONCAT22(extraout_DX, iVar2);
        uVar5 = pass1_1010_2e02(param_1, (iVar2 + 0x12));
        pass1_1010_35a4((uVar5 >> 0x10), param_1, uVar5);
    }
    pass1_1010_32f4(param_1, paStack6);
    pass1_1010_1f62(param_1, 0x8);
    if ((iVar3 + 0x52) != 0) {
        return;
    }
    return;
}
