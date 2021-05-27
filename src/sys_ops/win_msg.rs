use std::intrinsics::offset;

use crate::{sys2, sys_ops, winapi};
use crate::app_context::AppContext;
use crate::bool_funcs::check_and_set_global_1000_1fea;
use crate::draw::rect;
use crate::err_ops::error_check_1000_17ce;
use crate::func_ptr_funcs::call_fn_ptr_1_1020_6746;
use crate::mem_funcs::alloc_mem::alloc_mem_1000_07fc;
use crate::mem_funcs::free_mem::free_mem_1000_1b68;
use crate::mem_funcs::mem_ops_1::{get_type_at_address, StructuredData};
use crate::pass::pass12_funcs::pass1_1008_c6ae;
use crate::pass::pass13_funcs::{pass1_1008_932a, pass1_1008_941a, pass1_1008_9436};
use crate::pass::pass14_funcs::{pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_fn_1008_60e8};
use crate::pass::pass15_funcs::pass1_1020_64d4;
use crate::pass::pass17_funcs::{pass1_1030_5b00, pass1_1030_73a8};
use crate::pass::pass19_funcs::{pass1_1040_4d7e, pass1_1040_4dcc};
use crate::pass::pass20_funcs::{pass1_1010_9172, pass1_1010_91cc, pass1_1010_ac92, pass1_1010_ada6, pass1_1010_ae12, pass1_1010_ae92, pass1_1018_0ad4};
use crate::pass::pass4_funcs::{pass1_1028_bc90, pass1_1028_e1ec};
use crate::pass::pass7_funcs::{pass1_1018_255e, pass1_1018_270e, pass1_1018_3a7a, pass1_1018_57d2};
use crate::pass::pass8_funcs::{pass1_1010_4df0, pass1_1010_519a};
use crate::pass::pass8_funcs;
use crate::string_ops::load::{load_string_1008_a8f4, load_string_1010_84e0};
use crate::string_ops::misc::{copy_string_1000_3d3e, load_string_1010_847e, str_fn_1010_5286};
use crate::struct_ops::process_struct_1010_20ba;
use crate::struct_ops::struct_ops_2;
use crate::struct_ops::struct_ops_2::{init_struct_1000_1902, process_struct_1000_179c, set_struct_1008_0000};
use crate::structs::prog_structs_16::Struct493;
use crate::structs::prog_structs_2::{Struct199, Struct7};
use crate::structs::prog_structs_24::{Struct103, Struct2111};
use crate::structs::prog_structs_30::Struct124;
use crate::structs::prog_structs_6::Struct672;
use crate::sys_ops::{dos_ops, proc, process_struct_1040_8478};
use crate::typedefs::{HWND16, LPARAM, LRESULT, WPARAM16};
use crate::ui_ops::{dialog, misc, msg_box, ui2, window};
use crate::ui_ops::misc::{mixed_1040_8520, pass1_1038_af40, pass1_1038_e03e};
use crate::ui_ops::msg_box::msg_box_1040_64ca;
use crate::ui_ops::window::destroy_win_1040_7b98;
use crate::util::{CONCAT12, CONCAT13, CONCAT22, SBORROW2, SUB42, ZEXT24};
use crate::winapi::{GetDlgItem16, GetWindowWord16, InvalidateRect16, IsIconic16, IsWindow16, PostMessage16, SendMessage16, SendDlgItemMessage16, GetMessage16, IsDialogMessage16, PostQuitMessage16};
use crate::structs::prog_structs_26::Struct97;

pub unsafe fn send_win_msg_1040_a3d0(param_1: u32) {
    let mut u_var1: u32;
    let mut hwnd_var2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    if (i_var3 + 0x90) != 0 {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 0x14) = (i_var3 + 6);
        hwnd_var2 = winapi::GetDlgItem16((i_var3 + 6), 0x1826);
        send_win_msg_1040_a308(param_1, 0, hwnd_var2);
    }
    return;
}

pub unsafe fn send_win_msg_1040_a308(param_1: u32, param_2: HWND16, param_3: u16, param_4: HWND16) {
    let mut pu_var1: u16;
    let mut u_var2: u32;
    let mut lparam_var3: LPARAM;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut struct_var6: Struct2111;
    let mut wparam_var7: WPARAM16;
    let mut u_var8: u16;
    let mut hwnd_var9: HWND16;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    // winapi::SendMessage16(0, 0, 0x405, param_2);
    winapi::SendMessage16(param_2, 0x405, 0, 0);
    // winapi::SendMessage16(0, 0, 0xb, param_2);
    winapi::SendMessage16(param_2, 0xb, 0, 0);
    //// _var5 = (param_1  >> 0x10);
    // i_var4 = param_1;
    u_var2 = (i_var4 + 0x90);
    if (u_var2 + 0x10) == 0 {
        wparam_var7 = 0;
        u_var8 = 0x401;
        hwnd_var9 = param_2;
        lparam_var3 = load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        // winapi::SendMessage16(lparam_var3, wparam_var7, u_var8, hwnd_var9);
        winapi::SendMessage16(hwnd_var9, u_var8, wparam_var7, lparam_var3);
    } else {
        struct_var6 =
            struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000fff2, 3));
        local_c = 0;
        while (
            u_var2 = (i_var4 + 0x90),
            pu_var1 = (u_var2 + 0x10),
            unsafe { *pu_var1 != local_c && local_c <= *pu_var1 },
        ) {
            wparam_var7 = 0;
            u_var8 = 0x401;
            u_var2 = (i_var4 + 0x90);
            u_var2 = (u_var2 + 0xc);
            hwnd_var9 = param_2;
            lparam_var3 = pass1_1010_ac92(struct_var6, (u_var2 + local_c * 2));
            // winapi::SendMessage16(lparam_var3, wparam_var7, u_var8, hwnd_var9);
            winapi::SendMessage16(hwnd_var9, u_var8, wparam_var7, lparam_var3);
            local_c = local_c + 1;
        }
    }
    // winapi::SendMessage16(0, 1, 0xb, param_2);
    winapi::SendMessage16(param_2, 0xb, 1, 0);
    return 1;
}

pub unsafe fn win_msg_fn_1020_0ae8(ctx: &mut AppContext, struct_param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    send_win_msg_1020_08fe(struct_param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, struct_param_1);
    }
    return struct_param_1;
}

pub unsafe fn send_win_msg_1020_08fe(in_struct_1: &mut Struct7) {
    let hwnd: HWND16;
    let b_var1: bool;
    let local_struct_1: Struct7;
    let local_struct_1_hi: Struct7;
    let mut local_4: u16;
    let mut temp_5f73679df1: u32;
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0xb0e;
    local_struct_1.ptr_a_hi = 0x1020;
    if (&local_struct_1.field_0xe8 != 0) {
        temp_5f73679df1 = local_struct_1.field_0xe8;
        hwnd = (temp_5f73679df1 + 6);
        b_var1 = winapi::IsWindow16(hwnd);
        if b_var1 != 0 {
            // winapi_funcs::SendMessage16(0, 1, 0x111, hwnd);
            winapi::SendMessage16(hwnd, 0x111, 1, 0);
        }
        local_struct_1.field_0xe8 = 0;
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_215_ptr_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn send_win_msg_1020_097e(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0xea) | (i_var2 + 0xe8)) != 0) {
        u_var1 = (i_var2 + 0xe8);
        winapi::SendMessage16(0, 1, 0x111, (u_var1 + 6));
        (i_var2 + 0xe8) = 0;
    }
    return;
}

pub fn post_msg_1020_03b2(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    winapi::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn post_msg_1020_03d6(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    winapi::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn post_msg_1020_03fa(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    winapi::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn post_win_msg_1020_061c(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

  // u_var2 = (param_1  >> 0x10);
    if (param_2 == 1) {
        (param_1 + 6) = 0;
        return;
    }
    if (param_2 != 2) {
        return;
    }
    u_var1 = (param_1 + 6);
    winapi::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub unsafe fn send_win_msg_1010_7c9e(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    if ((((i_var3 + 0x1e) | (i_var3 + 0x1c)) != 0) && (param_2 != 0)) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var3 + 0x1c));
        while (true) {
            lVar5 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
          // u_var4 = (lVar5  >> 0x10);
            i_var3 = lVar5;
            if (lVar5 == 0) {
                break;
            }
            if ((i_var3 + 4) != 0) {
                u_var6 = pass1_1030_73a8((i_var3 + 4));
                BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var6 + 0xc), param_2);
                if (BVar2 != 0) {
                    u_var1 = (i_var3 + 8);
                    winapi::SendMessage16(0, 0xeb, 0x111, (u_var1 + 6));
                }
            }
        }
    }
    return;
}

pub unsafe fn send_msg_1010_7c42(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x1e) | (i_var2 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            winapi::SendMessage16(0, 0xeb, 0x111, (u_var1 + 6));
        }
    }
    return;
}

pub fn send_msg_1040_c85a(param_1: u32) {
    _PTR_LOOP_1050_5efe = param_1;
    winapi::SendMessage16(0, 0xfa, 0x111, (param_1 + 0x1a));
    return;
}

pub unsafe fn window_msg_func_1008_97f2(
    param_1: &mut Vec<u8>,
    param_2: &mut i32,
    param_3: i32,
    param_4_00: Vec<u8>,
    param_4: i32,
) {
    let pp_var1: fn();
    let BVar2: bool;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let unaff_cs: u8;
    let mut u_var6: u32;
    let u_var7: u8;
    let u_var8: u8;
    let mut cVar9: u8;

    if param_4 == 0x2b {
        if *param_2 == 4 {
            sys_ops::get_prop_1040_9566(param_2, param_3);
        } else {
            unsafe {
                pp_var1 = (*param_1 + 0x70);
            }
            (**pp_var1)();
        }
        u_var5 = 1;
        // goto LAB_1008_9a95;
    }
  // u_var8 = (param_1  >> 0x10);
    if (param_4 < 0x2c) {
        unaff_cs = 8;
        match (param_4) {
            1 => {}
            2 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x3c);
                }
                (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var4, u_var3);
                SetWindowLong16(0, 0);
                BVar2 = winapi::IsWindow16((i_var4 + 0xbc));
                if (BVar2 != 0) {
                    winapi::PostMessage16(param_1, 199, 0x111, (i_var4 + 0xbc));
                }
            }
            3 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x54);
                }
                (**pp_var1)(8, u_var7, u_var3, param_3, param_2);
            }
            // default:
            // goto switchD_1008_9b30_caseD_4;
            5 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x58);
                }
                (**pp_var1)(8, u_var7, u_var8, param_3, param_2, param_4_00);
            }
            7 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x50);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            8 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x74);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            0xd => {
                unsafe {
                    pp_var1 = (*param_1 + 0x84);
                }
                i_var4 = (**pp_var1)(
                    8,
                    u_var7,
                    u_var8,
                    param_2,
                    CONCAT12(param_4_00._0_1_, param_3),
                );
                // goto LAB_1008_9ada;
            }
            0xf => {
                unsafe {
                    pp_var1 = (*param_1 + 0x34);
                }
                (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var4, u_var3);
            }
            0x10 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x38);
                }
                u_var6 = (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var4, u_var3);
                return u_var6;
            }
            0x19 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x78);
                }
                u_var3 = (**pp_var1)(
                    8,
                    u_var7,
                    u_var8,
                    param_2,
                    CONCAT12(param_4_00._0_1_, param_3),
                );
                return CONCAT22(0x1050, u_var3);
            }
            0x1c => {
                unsafe {
                    pp_var1 = (*param_1 + 0x30);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            _ => {}
        }
    } else {
        cVar9 = param_4;
        if (param_4 == 0x112) {
            if ((PTR_LOOP_1050_039a == 0x0)
                && (
                    unsafe { pp_var1 = (*param_1 + 0x48) },
                    unsafe { i_var4 = (**pp_var1)() },
                    i_var4 != 0,
                ))
            {
                proc::def_wnd_proc_func_1008_9ce6(
                    param_1,
                    CONCAT13((param_3 >> 8), CONCAT12(param_3, param_2)),
                    CONCAT13(1, CONCAT12(cVar9, param_4_00)),
                );
            }
        } else {
            if (param_4 < 0x113) {
                if (param_4 == 0x86) {
                    unsafe {
                        pp_var1 = (*param_1 + 0x80);
                    }
                    u_var6 = (**pp_var1)();
                    return u_var6;
                }
                if (param_4 < 0x87) {
                    if (param_4 == 0x85) {
                        unsafe {
                            pp_var1 = (*param_1 + 0x7c);
                        }
                        u_var6 = (**pp_var1)();
                        return u_var6;
                    }
                    if (param_4 < 0x86) {
                        if (cVar9 == '7') {
                            return *(i_var4 + 0xc2);
                        }
                        if (cVar9 == 'A') {
                            unsafe {
                                pp_var1 = (*param_1 + 0x2c);
                            }
                            (**pp_var1)();
                            // goto switchD_1008_9b30_caseD_1;
                        }
                    }
                    // switchD_1008_9b30_caseD_4:
                    if ((param_4 < 0x400) || (0x7ffe < param_4)) {
                        u_var6 = proc::def_wnd_proc_func_1008_9ce6(
                            param_1,
                            CONCAT22(param_3, param_2),
                            CONCAT22(param_4, param_4_00),
                        );
                        return u_var6;
                    }
                    unsafe {
                        pp_var1 = (*param_1 + 0x28);
                    }
                    (**pp_var1)(
                        unaff_cs,
                        u_var7,
                        u_var8,
                        param_2,
                        param_3,
                        CONCAT22(param_4, param_4_00),
                    );
                } else {
                    if (param_4 == 0x100) {
                        if (PTR_LOOP_1050_039a == 0x0) {
                            unsafe {
                                pp_var1 = (*param_1 + 0x6c);
                            }
                            (**pp_var1)();
                        }
                    } else {
                        if (param_4 == 0x102) {
                            if (PTR_LOOP_1050_039a == 0x0) {
                                unsafe {
                                    pp_var1 = (*param_1 + 0x68);
                                }
                                (**pp_var1)();
                            }
                        } else {
                            if (param_4 != 0x111) {}
                            // goto switchD_1008_9b30_caseD_4;
                            if ((param_4_00 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == 0x0)) {
                                if (param_2 == 0x0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x40);
                                    }
                                    (**pp_var1)();
                                } else {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x44);
                                    }
                                    (**pp_var1)();
                                }
                            }
                        }
                    }
                }
            } else {
                if (param_4 == 0x204) {
                    if (PTR_LOOP_1050_039a == 0x0) {
                        unsafe {
                            pp_var1 = (*param_1 + 0x60);
                        }
                        (**pp_var1)();
                    }
                } else {
                    if (param_4 < 0x205) {
                        if (param_4 == 0x113) {
                            if (_PTR_LOOP_1050_0388 != 0) {
                                pass1_1008_932a(_PTR_LOOP_1050_0388);
                            }
                        } else {
                            if (param_4 == 0x117) {
                                if (param_3 == 0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x4c);
                                    }
                                    (**pp_var1)();
                                } else {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x20);
                                    }
                                    (**pp_var1)();
                                }
                            } else {
                                if (param_4 != 0x201) {}
                                // goto switchD_1008_9b30_caseD_4;
                                if (PTR_LOOP_1050_039a == 0x0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x5c);
                                    }
                                    (**pp_var1)();
                                }
                            }
                        }
                    } else {
                        if (param_4 == 0x210) {
                            unsafe {
                                pp_var1 = (*param_1 + 100);
                            }
                            (**pp_var1)();
                        } else {
                            if (param_4 == 0x30f) {
                                // LAB_1008_9af8:
                                unsafe {
                                    pp_var1 = (*param_1 + 0x8c);
                                }
                                i_var4 = (**pp_var1)();
                                // LAB_1008_9ada:
                                return i_var4;
                            }
                            if (param_4 == 0x311) {
                                unsafe {
                                    pp_var1 = (*param_1 + 0x88);
                                }
                                i_var4 = (**pp_var1)();
                                if (i_var4 != 0) {}
                                // goto LAB_1008_9af8;
                            } else {
                                if (param_4 != 0x3b9) {}
                                // goto switchD_1008_9b30_caseD_4;
                                unsafe {
                                    pp_var1 = (*param_1 + 0x24);
                                }
                                (**pp_var1)();
                            }
                        }
                    }
                }
            }
        }
    }
    // switchD_1008_9b30_caseD_1:
    u_var5 = 0;
    // LAB_1008_9a95:
    return u_var5;
}

pub unsafe fn post_win_msg_1008_a0e4(
    param_1: &mut Struct2111,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    param_6: u16,
) -> u8 {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u8_var3: bool;
    let u_var4: u8;
    let mut u_var5: i32;
    let mut u_var7: u32;
    let puVar8: u16;


    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut local_20: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];
    let b_var6: bool;

  // u_var10 = (param_1  >> 0x10);
    i_var9 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var9 + 10));
    u8_var3 = false;
    while {
        u_var7 = ZEXT24(local_a);
        pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var5 = u_var7;
        if ((ctx.dx_reg | u_var5) == 0) {}
        // goto LAB_1008_a146;
        (u_var5 + 4) != param_6
    } {}
    (u_var5 + 0xc) = (u_var5 + 0xc) + param_3;
    u_var7 = (u_var5 + 0xe) + param_2;
    (u_var5 + 0xe) = u_var7;
    u8_var3 = true;
    // LAB_1008_a146:
    u_var4 = u_var7;
    if (!u8_var3) {
        puVar8 = _PTR_LOOP_1050_03a0;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_03a0);
        u_var5 = puVar8;
        local_e = puVar8 & 0xffff | ctx.dx_reg << 0x10;
        if ((ctx.dx_reg | u_var5) == 0) {
            local_e = 0;
        } else {
            local_e = ctx.s_1_1050_389a;
            (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var5 + 4) = param_6;
            (u_var5 + 6) = param_5;
            (u_var5 + 10) = param_4;
            (u_var5 + 0xc) = param_3;
            (u_var5 + 0xe) = param_2;
            local_e = 0xad8e;
            (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
            puVar8 = local_e;
        }
        u_var4 = puVar8;
        u_var1 = (i_var9 + 10);
        ppc_var2 = ((i_var9 + 10) + 8);
        ppc_var2(0x1000, u_var1, (u_var1 >> 0x10), local_e, (local_e >> 0x10));
    }
    if (param_6 == 0x14) {
        b_var6 = winapi::PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        u_var4 = b_var6;
    }
    return u_var4;
}

pub fn process_win_msg_1008_9498() -> WPARAM16 {
    let mut b_result: u16;
    let b_var1: bool;
    let i_var2: u16;
    let mut h_wnd_dlg: u16;
    let mut local_18: u16;
    let mut message: [u8; 18];
    let mut local_4: u16;

    // LAB_1008_949c:
    b_result = GetMessage16(0, 0, 0, message);
    if (b_result == 0) {
        return message._4_2_;
    }
    if ((ctx._PTR_LOOP_1050_1040 + 8) != 0) {}
    // goto code_r0x100894cd;
    // goto LAB_1008_94dc;
    // code_r0x100894cd:
    b_var1 = IsDialogMessage16(message, h_wnd_dlg);
    if (b_var1 == 0) {
        // LAB_1008_94dc:
        if (g_haccel_1050_0398 != 0x0) {
            i_var2 = TranslateAccelerator16(
                CONCAT22(h_wnd_dlg, message),
                g_haccel_1050_0398,
                ctx.g_h_window,
            );
            if (i_var2 != 0) {}
            // goto LAB_1008_949c;
        }
        TranslateMessage16(message);
        DispatchMessage16(message);
    }
    // goto LAB_1008_949c;
}

pub fn process_win_msg_1008_9510(param_1: &mut i32, param_2: &mut String) {
    let b_var1: bool;
    let i_var2: u16;
    let unaff_ss: HWND16;
    let mut local_16: u16;
    let mut msg: [u8; 18];

    // LAB_1008_9578:
    if (unsafe { *param_1 != 0 }) {
        b_var1 = GetMessage16(0, 0, 0, msg);
        if (b_var1 != 0) {
            if ((ctx._PTR_LOOP_1050_1040 + 8) != 0) {}
            // goto code_r0x10089538;
            // goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
    b_var1 = IsDialogMessage16(msg, unaff_ss);
    if (b_var1 == 0) {
        // LAB_1008_9547:
        if (g_haccel_1050_0398 != 0x0) {
            i_var2 =
                TranslateAccelerator16(CONCAT22(unaff_ss, msg), g_haccel_1050_0398, ctx.g_h_window);
            if (i_var2 != 0) {}
            // goto LAB_1008_9578;
        }
        TranslateMessage16(msg);
        DispatchMessage16(msg);
    }
    // goto LAB_1008_9578;
}

pub fn send_win_msg_1008_9640(param_1: u32, param_2: WPARAM16) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    if ((param_1 + 8) != 0) {
        winapi::SendMessage16(0, param_2, 0x86, (param_1 + 8));
    }
    return;
}

pub fn send_win_msg_1008_84ba(param_1: u16, param_2: &mut HWND16) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_4: u16;

  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    if ((*(i_var1 + 4) & 4) == 0) {
        if ((*(i_var1 + 4) & 8) == 0) {
            return;
        }
        local_4 = 1;
    } else {
        local_4 = 0;
    }
    winapi::SendMessage16(*(i_var1 + 2), local_4, 0x115, *param_1);
    return;
}

pub fn pos_msg_1008_3d20(param_1: u32) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    winapi::PostMessage16(0, (param_1 + 0xcc), 0x111, (param_1 + 0xbc));
    return;
}

pub unsafe fn process_win_msg_1008_54aa(
    ctx: &mut AppContext,
    struct_param_1: &mut Struct97,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    in_h_instance: u16,
) {
    let pp_var1: fn();
    // let struct_a: Struct199;
    let mut base_var2: u16;
    let mut local_1e: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    // let func_ptr_c: &mut Vec<u8>;
    let mut temp_5f75f7f23c: u32;

    if param_3_00 != 0 {
        return;
    }
    dos_ops::dos3_call_1000_435c(ctx, None);
    sys_ops::set_global_uint_1000_4d0c(ctx, None);
    check_and_set_global_1000_1fea(ctx);
    init_struct_1000_1902(None, 0x32, 0, 0x12);
    _PTR_LOOP_1050_03a0 = CONCAT22(local_DX_57, ctx.ax_reg);
    init_struct_1000_1902(None, 100, 0, 0xc);
    _PTR_LOOP_1050_029c = CONCAT22(local_DX_81, ctx.ax_reg);
    init_struct_1000_1902(None, 100, 0, 0x10);
    _PTR_LOOP_1050_4fb8 = CONCAT22(local_DX_105, ctx.ax_reg);
    init_struct_1000_1902(None, 100, 0, 0xe);
    _PTR_LOOP_1050_68a2 = CONCAT22(local_DX_129, ctx.ax_reg);
    init_struct_1000_1902(None, 500, 0, 0x42);
    _PTR_LOOP_1050_5744 = CONCAT22(local_DX_153, ctx.ax_reg);
    init_struct_1000_1902(None, 0x32, 0, 6);
    ctx.g_h_instance_1050_038c = in_h_instance;
    ctx.PTR_LOOP_1050_038e = param_3_00;
    ctx.PTR_LOOP_1050_0390 = struct_param_1;
    let mut struct_a: Struct103 = get_type_at_address::<Struct103>(ctx.dx_reg as u32);
    ctx.PTR_LOOP_1050_5768 = ctx.ax_reg;
    ctx.PTR_LOOP_1050_576a = ctx.dx_reg;
    pass1_fn_1008_60e8();
    ctx._PTR_LOOP_1050_0392 = CONCAT22(struct_a, struct_param_1);
    process_struct_1000_179c(0xc, &mut struct_a);
    if (struct_a | *struct_param_1) == 0 {
        *struct_param_1 = 0;
        base_var2 = 0;
    } else {
        set_struct_1008_0000(struct_param_1);
        base_var2 = local_DX_249;
    }
    let func_ptr_c = CONCAT22(base_var2, *struct_param_1);
    if (ctx._PTR_LOOP_1050_0392 != 0) {
        // WARNING: Load size is inaccurate
        pp_var1 = (*func_ptr_c + 4);
        pp_var1(0x1000, struct_param_1, base_var2, ctx._PTR_LOOP_1050_0392);
    }
    local_1e = CONCAT22(base_var2, struct_param_1);
    // WARNING: Load size is inaccurate
    temp_5f75f7f23c = *func_ptr_c;
    // TODO
    // pp_var1 = temp_5f75f7f23c + 4;
    // (pp_var1)();
    process_win_msg_1008_9498(local_1e);
    if (func_ptr_c != 0) {
        pp_var1 = temp_5f75f7f23c;
        (**pp_var1)(0x1000, struct_param_1, base_var2, 1);
    }
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_03a0);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_029c);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_4fb8);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_68a2);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_5744);
    return;
}

pub fn send_win_msg_1040_93e6(param_1: u32) -> LRESULT {
    let mut u_var1: u16;
    let LVar2: LRESULT;

  // u_var1 = (param_1  >> 0x10);
    LVar2 = winapi::SendMessage16(0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}

pub fn send_msg_1040_9404(param_1: u32) -> LRESULT {
    let mut u_var1: u16;
    let LVar2: LRESULT;
  // u_var1 = (param_1  >> 0x10);
    LVar2 = winapi::SendMessage16(0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}

pub fn post_win_msg_1040_7f1c(param_1: u32, param_2: i32) -> u16 {
    let mut i_var1: i32;
    let mut u_var2: u16;

  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x78) != 0) {
        return 0;
    }
    if ((i_var1 + 0x60) != param_2) {
        (i_var1 + 0x60) = param_2;
        winapi::PostMessage16(0, 0, 0x85, (i_var1 + 6));
    }
    return 1;
}

pub unsafe fn post_win_msg_1040_7f56(param_1: &mut StructuredData, param_2: &String) {
    let mut str_var1: String = param_1::get_string(0x10, None);
    let mut hwnd: HWND16 = param_1::get_u16(6) as HWND16;
    copy_string_1000_3d3e(&mut str_var1, param_2);
    // winapi_funcs::PostMessage16(0, 0, 0x85, (param_1 + 6));
    winapi::PostMessage16(hwnd, 0x85, 0, 0);
    return;
}

pub fn post_win_msg_1040_7b3c(param_1: &mut u32, param_2: u16, param_3: &mut String, param_4: &mut Struct103) -> u16 {
    let pp_var1: fn();

    if (param_2 == 1) || (param_2 == 2) {
        unsafe {
            pp_var1 = (*param_1 + 0x14);
        }
        (**pp_var1)();
    } else {
        if param_2 == 0x6f {
            unsafe {
                pp_var1 = (*param_1 + 0x2c);
            }
            (**pp_var1)();
        } else {
            if param_2 != 0x12e {
                return 0;
            }
            winapi::PostMessage16(0, 0xf060, 0x112, (param_1 + 6));
        }
    }
    return 1;
}

pub unsafe fn post_win_msg_1040_672e(ctx: &mut AppContext, param_1: &mut u32, param_2: u16, param_3: &mut String, param_4: u32) {
    let mut u_var1: u16;

    if param_3 == (ctx.s_vrpal_bmp_1050_183a + 7) {
        msg_box_1040_64ca(param_1);
    } else {
        if param_3 == (ctx.s_logo_bmp_1050_1850 + 1) {
            u_var1 = 0x2a;
        } else {
            if param_3 != (ctx.s_logo_bmp_1050_1850 + 2) {
                post_win_msg_1040_7b3c(param_1, param_2, param_3);
                return;
            }
            u_var1 = 0x29;
        }
        pass1_1038_af40(ctx.g_struct_112_001, *(param_1 + 8), u_var1);
        winapi::PostMessage16(0, 2, 0x111, (param_1 + 6));
    }
    return;
}

pub unsafe fn send_win_msg_1040_4cf4(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let h_wnd: HWND16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let LVar8: LRESULT;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    h_wnd = winapi::GetDlgItem16(6000, (i_var5 + 6));
    LVar8 = winapi::SendMessage16(0, 0, 0x407, h_wnd);
    if (LVar8 != 0xffff) {
        winapi::SendMessage16(CONCAT22(unaff_ss, local_52), LVar8, 0x408, h_wnd);
    }
    u_var3 = (i_var5 + 0x90);
    u_var1 = (i_var5 + 0x94);
    i_var4 = pass1_1010_ae12(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, local_52),
        (u_var3 + 10),
    );
    if (i_var4 != -1) {
        u_var1 = (i_var5 + 0x90);
      // u_var7 = (u_var1  >> 0x10);
        i_var4 = u_var1;
        u_var2 = (i_var4 + 6);
        local_58 = u_var2;
        pass1_1010_ae92((i_var5 + 0x94), local_58, (i_var4 + 10), u_var2);
    }
    return;
}

pub fn send_msg_1040_323c(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let LVar3: LRESULT;
    let mut local_a: u16;
    let mut local_6: u16;
  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    LVar3 = winapi::SendMessage16(0, 0, 0x407, (i_var1 + 0x92));
    winapi::SendMessage16(0, 0, 0x407, (i_var1 + 0x94));
    winapi::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x9a),
        LVar3,
        0x408,
        (i_var1 + 0x92),
    );
    LVar3 = winapi::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x19a),
        0x408,
        0x408,
        (i_var1 + 0x94),
    );
    return LVar3;
}

pub unsafe fn send_msg_1040_1696(param_1: u32, param_2: HWND16) {
    let mut u_var1: u32;
    let p_uvar2: u16;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let w_var6: WPARAM16;
    let mut u_var7: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    winapi::SendMessage16(0, 0, 0x40b, param_2);
    winapi::SendMessage16(0, 0, 0xb, param_2);
    local_4 = 0;
    pu_var2 = &local_4;
  // u_var5 = (param_1  >> 0x10);
    pass1_1010_519a((param_1 + 0x8e), CONCAT22(unaff_ss, pu_var2));
    local_12 = 0;
    loop {
        if (local_4 <= local_12) {
            break;
        }
        u_var4 = (pu_var2 + local_12 * 2);
        w_var6 = 0;
        u_var7 = 0x403;
        u_var1 = (param_1 + 0x8e);
        str_fn_1010_5286(u_var1, (u_var1 >> 0x10), u_var4);
        winapi::SendMessage16(
            u_var4 & 0xffff | ctx.dx_reg << 0x10,
            w_var6,
            u_var7,
            param_2,
        );
        error_check_1000_17ce((u_var4 & 0xffff | ctx.dx_reg << 0x10));
        local_12 = local_12 + 1;
    }
    w_var6 = 0;
    u_var7 = 0x40a;
    u_var3 = local_4;
    load_string_1010_847e(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    winapi::SendMessage16(CONCAT22(ctx.dx_reg, u_var3), w_var6, u_var7, param_2);
    winapi::SendMessage16(0, 1, 0xb, param_2);
    return;
}

pub fn post_win_msg_1040_0d5e(param_1: u32, param_2: i32) {
    if param_2 != 0 {
        winapi::PostMessage16(0, 1, 0x111, (param_1 + 8));
    }
    return;
}

pub unsafe fn send_win_msg_1038_ed8a(param_1: &mut u32, param_2: &mut String, param_3: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let paVar3: Struct493;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let ppVar6: Struct2111;
    let lVar7: u32;
    let h_wnd: HWND16;
    let mut in_stack_0000ffe2: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    h_wnd = ctx.g_h_window;
    if (param_3 != 0x1c8) {
        if (param_3 == 0x1c9) {
            ppVar6 = struct_ops_2::process_struct_1010_20ba(
                ctx.g_struct_var_1050_0ed0,
                CONCAT22(in_stack_0000ffe2, 0x2f),
            );
          // u_var2 = (ppVar6  >> 0x10);
            u_var1 = (ppVar6 + 0x20);
            u_var5 = (ppVar6 + 0x22);
            if ((u_var5 | u_var1) == 0) {
                return;
            }
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, u_var5);
            if ((u_var5 | paVar3) == 0) {
                return;
            }
            i_var4 = pass1_1030_5b00(CONCAT22(u_var5, paVar3));
            ppVar6 = struct_ops_2::process_struct_1010_20ba(
                ctx.g_struct_var_1050_0ed0,
                CONCAT22(in_stack_0000ffe2, i_var4),
            );
            if (ppVar6 == 0x0) {
                return;
            }
            lVar7 = pass1_1018_0ad4(ppVar6);
            if (lVar7 == 0) {
                return;
            }
            param_3 = 0x72;
            h_wnd = (lVar7 + 8);
        } else {
            if (param_3 != 0x1ca) {
                post_win_msg_1040_7b3c(param_1, param_2, param_2, param_3, param_3);
                return;
            }
        }
    }
    winapi::SendMessage16(0, param_3, 0x111, h_wnd);
    return;
}

pub unsafe fn post_win_msg_1038_dcb0(param_1: &mut u32) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let struct_a: Struct103;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 4];
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0xb4, ctx.dx_reg);
    u_var4 = ctx.dx_reg | in_ax;
    i_var5 = param_1;
  // u_var3 = (param_1  >> 0x10);
    if (u_var4 == 0) {
        u_var2 = 0;
        u_var4 = 0;
    } else {
        u_var2 = mixed_1040_8520(CONCAT22(ctx.dx_reg, in_ax), (i_var5 + 6), 4, 3, 0x634, 0x726);
    }
    _local_6 = CONCAT22(u_var4, u_var2);
    pass1_1008_941a(CONCAT22(unaff_ss, local_a), 1, 0x49);
    pp_var1 = (*_local_6 + 0x6c);
    _local_10 = (**pp_var1)();
  // struct_a = (_local_10  >> 0x10);
    local_c = _local_10;
    if (local_c == 6) {
        process_struct_1000_179c(0xb4, &mut struct_a);
      // u_var4 = (_local_10  >> 0x10) | _local_10;
        if (_local_10 == 0x0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var3 = mixed_1040_8520(_local_10, (i_var5 + 6), 0, 2, 0x634, 0x728);
        }
        _local_6 = CONCAT22(u_var4, u_var3);
        pass1_1008_941a(CONCAT22(unaff_ss, local_14), 1, 0x4a);
        pp_var1 = (*_local_6 + 0x6c);
        (**pp_var1)(
            &ctx.PTR_LOOP_1050_1008,
            _local_6,
            (_local_6 >> 0x10),
            local_14,
            unaff_ss,
        );
        u_var9 = 0;
        u_var10 = 0;
        u_var11 = 0x15;
        u_var7 = 1;
        u_var8 = 0;
        u_var2 = 0;
        u_var6 = 0;
        u_var3 = 0;
        _local_18 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            _local_18,
            CONCAT22(u_var2, u_var3),
            u_var6,
            u_var7,
            CONCAT13(u_var10, CONCAT12(u_var9, u_var8)),
            u_var11,
        );
        winapi::PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        return;
    }
    process_struct_1000_179c(0xb4, &mut struct_a);
  // u_var4 = (_local_10  >> 0x10) | _local_10;
    if (_local_10 == 0x0) {
        u_var3 = 0;
        u_var4 = 0;
    } else {
        u_var3 = mixed_1040_8520(_local_10, (i_var5 + 6), 0, 2, 0x634, 0x729);
    }
    _local_6 = CONCAT22(u_var4, u_var3);
    pass1_1008_941a(CONCAT22(unaff_ss, &local_18), 1, 0x4b);
    pp_var1 = (*_local_6 + 0x6c);
    (**pp_var1)(
        &ctx.PTR_LOOP_1050_1008,
        _local_6,
        (_local_6 >> 0x10),
        &local_18,
        unaff_ss,
    );
    return;
}

pub fn post_win_msg_1038_d840(param_1: Vec<u8>, param_2: i32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
  // u_var2 = (param_1  >> 0x10);
    if (param_2 == 0x10) {
        if ((i_var1 + 0x8e) != 0) {
            winapi::PostMessage16(0, (i_var1 + 0x8e), 0x111, (i_var1 + 6));
            (i_var1 + 0x8e) = 0;
            return;
        }
    } else {
        if (param_2 < 0x11) {
            if (param_2 == 0x1) {
                (i_var1 + 0x90) = 0;
                (i_var1 + 0x92) = 0;
                return;
            }
            if (param_2 == '\x03') {
                pass1_1038_e03e(param_1);
                return;
            }
        }
    }
    return;
}

pub fn send_win_msg_1038_c228(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let LVar3: LRESULT;
    let mut local_a: u16;
    let mut local_6: u16;

  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    LVar3 = winapi::SendMessage16(0, 0, 0x407, (i_var1 + 0x92));
    winapi::SendMessage16(0, 0, 0x407, (i_var1 + 0x94));
    winapi::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x9e),
        LVar3,
        0x408,
        (i_var1 + 0x92),
    );
    LVar3 = winapi::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x19e),
        0x408,
        0x408,
        (i_var1 + 0x94),
    );
    return LVar3;
}

pub fn send_msg_1038_a9fa(param_1: u32, param_2: i32) {
    let h_wnd: u16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: Struct2111;
    let LVar4: LRESULT;
    let mut in_stack_0000fff0: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        ppVar3 =
            struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000fff0, 2));
      // u_var2 = (param_1  >> 0x10);
        i_var1 = param_1;
        LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x116, (i_var1 + 6));
        if (LVar4 == 0) {
            LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x117, (i_var1 + 6));
            if (LVar4 == 0) {
                LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x118, (i_var1 + 6));
                if (LVar4 == 0) {
                    LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x119, (i_var1 + 6));
                    if (LVar4 != 0) {
                        u16_1050_13ae = 4;
                    }
                } else {
                    u16_1050_13ae = 3;
                }
            } else {
                u16_1050_13ae = 2;
            }
        } else {
            u16_1050_13ae = 1;
        }
        LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x11a, (i_var1 + 6));
        (ppVar3 + 0x82) = LVar4;
        h_wnd = winapi::GetWindowWord16(-8, (i_var1 + 6));
        winapi::PostMessage16(0, 0x105, 0x111, h_wnd);
        destroy_win_1040_7b98(i_var1, u_var2, 1);
    }
    return;
}

pub unsafe fn send_win_msg_1028_e242(ctx: &mut AppContext, param_1: u32, param_2: u32) {
    if *param_1 % 100 == 0 {
        // SendMessage16(0, 0, 0x41, ctx.g_h_window);
        let result = winapi::SendMessage16(ctx.g_h_window, 0x41, 0, 0);
    }
    *param_1 = *param_1 + 1;
    if param_2 != 0 {
        bad_1028_e28a();
    }
    return;
}

pub unsafe fn win_msg_fn_1020_d460(ctx: &mut AppContext, param_1: &mut Vec<u8>, param_2: u32, param_3: u32, param_4: u32) -> u16 {
    let mut u_var1: u16;
    let ppVar2: Struct2111;
    let mut in_stack_0000ffe8: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var1 = pass1_1028_bc90(param_1, param_2, param_3, param_4);
    if (u_var1 != 0) {
        pass1_1038_af40(ctx.g_struct_112_001, *(_PTR_LOOP_1050_4230 + 0x16), 0x11);
        ctx.PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 1);
        process_win_msg_1008_9510(&mut ctx.PTR_LOOP_1050_5b80, &mut ctx.g_alloc_addr_1050_1050);
        let struct_var2 = struct_ops_2::process_struct_1010_20ba(
            ctx.g_struct_var_1050_0ed0,
            CONCAT22(in_stack_0000ffe8, 0x3a),
        );
        (param_1 + 0x20) = (struct_var2 + 10);
        u_var1 = 1;
    }
    return u_var1;
}

pub fn post_win_msg_1020_79fc(param_1: u32, param_2: u16, param_3: u16, param_4: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
  // u_var6 = (*(i_var4 + 0xe0)  >> 0x10);
    ppc_var2 = ((i_var4 + 0xe0) + 0x24);
    i_var3 = ppc_var2();
    if (i_var3 != param_2) {
        winapi::PostMessage16(0, 0, 0x85, (i_var4 + 8));
        u_var1 = (i_var4 + 0xe0);
        ppc_var2 = ((i_var4 + 0xe0) + 0x28);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10), param_2, u_var6);
    }
    return;
}

pub fn post_msg_1020_4394(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
  // u_var3 = (param_1  >> 0x10);
    if (param_2 == 0x10) {
        if ((i_var2 + 0x34) != 0) {
            winapi::PostMessage16(0, 0xf6, 0x111, ctx.g_h_window);
            return;
        }
    } else {
        if (param_2 < 0x11) {
            if (param_2 == 0x1) {
                (i_var2 + 0x18) = 0;
                return;
            }
            if (param_2 == 0xb) {
                u_var1 = (i_var2 + 0x2c);
                (u_var1 + 0xe) = (i_var2 + -0xda);
                return;
            }
        }
    }
    return;
}

pub fn post_win_msg_1020_291a(param_1: &mut StructuredData) {
    let hwnd = param_1.get_u16(8) as HWND16;
    // winapi_funcs::PostMessage16(0, 0, 0x10, (param_1 + 8));
    winapi::PostMessage16(hwnd, 0x10, 0, 0);
    return;
}

pub unsafe fn post_win_msg_1020_1ca4(param_1: u32) -> u16 {
    let mut pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut pu_var5: Struct103;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    //// _var4 = (param_1  >> 0x10);
    if (param_1 + 0x96) == 0 {
        pu_var5 = pass1_1010_4df0((param_1 + 0x8e));
        if pu_var5 == 0 {
            u_var4 = 0x1000;
            process_struct_1000_179c(0xb4, &mut pu_var5);
            u_var3 = pu_var5;
            if pu_var5 == 0x0 {
                u_var2 = 0;
                u_var3 = 0;
            } else {
                u_var4 = &ctx.PTR_LOOP_1050_1040;
                u_var2 = mixed_1040_8520(&mut pu_var5, ctx.g_h_window, 0x30, 2, 0x57b, 0x62a);
            }
            _local_a = CONCAT22(u_var3, u_var2);
            pp_var1 = (*_local_a + 0x74);
            (**pp_var1)(u_var4, u_var2, u_var3);
            return 0;
        }
        winapi::PostMessage16(0, 0xde, 0x111, ctx.g_h_window);
    }
    return 1;
}

pub unsafe fn post_win_msg_1040_d2ac(ctx: &mut AppContext,
                                     param_1: &mut  Struct124,
                                     param_2: u16,
                                     param_3: u16,
                                     param_4: u32) {
    let LVar1: LRESULT;

    if param_2 == (ctx.s_dibtext_bmp_1050_1844 + 4) {
        // SendDlgItemMessage16(
        //     0,
        //     0,
        //     0x405,
        //     (ctx.s_dibtext_bmp_1050_1844 + 3),
        //     param_1.field_0x6,
        // );
        SendDlgItemMessage16(param_1.field_0x6, ctx.s_dibtext_bmp_1050_1844 + 3, 0x405, 0, 0);
        pass1_1010_9172(&param_1[1].field_0x8);
    } else {
        if (ctx.s_dibtext_bmp_1050_1844 + 4) < param_2 {
            if param_2 == (ctx.s_dibtext_bmp_1050_1844 + 5) {
                // LVar1 = SendDlgItemMessage16(
                //     0,
                //     0,
                //     0x40c,
                //     (ctx.s_dibtext_bmp_1050_1844 + 3),
                //     &param_1.field_0x6,
                // );
                LVar1 = SendDlgItemMessage16(
                    param_1.field_0x6,
                    ctx.s_dibtext_bmp_1050_1844 + 3,
                    0x40c,
                    0,
                    0);
                if (LVar1 != -1) || ((LVar1 >> 0x10) != -1) {
                    // SendDlgItemMessage16(
                    //     0,
                    //     LVar1 - 1,
                    //     0x403,
                    //     (ctx.s_dibtext_bmp_1050_1844 + 3),
                    //     &param_1.field_0x6,
                    // );
                    SendDlgItemMessage16(param_1.field_0x6, ctx.s_dibtext_bmp_1050_1844 + 3, 0x403, (LVar1 - 1) as u16, 0);
                    pass1_1010_91cc(&param_1[1].field_0x8);
                }
            } else {
                if param_2 == (ctx.s_dibtext_bmp_1050_1844 + 6) {
                    window::enable_window_1040_d6be(param_1, param_2_00);
                    pass1_1018_57d2((param_1 + 1), CONCAT22(param_2_00, param_1));
                    // PostMessage16(0, 0x203, 0x111, ctx.g_h_window);
                    PostMessage16(ctx.g_h_window, 0x111, 0x203, 0);
                } else {
                    if param_2 != (ctx.s_dibtext_bmp_1050_1844 + 7) {}
                    // goto LAB_1040_d3b3;
                    ctx.PTR_LOOP_1050_5a68 = &param_1[1].field_0x4;
                    misc::pass1_1038_af40(ctx.g_struct_112_001, &param_1.field_0x6, 0x27);
                }
            }
        } else {
            if param_2 == 0xeb {
                dialog::send_dialog_item_msg_1040_d79c(CONCAT22(param_2_00, param_1));
            } else {
                if param_2 != (ctx.s_vrpal_bmp_1050_183a + 7) {
                    // LAB_1040_d3b3:
                    window::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                    return;
                }
                msg_box::msg_box_1040_d3d0(param_1, param_2_00);
            }
        }
    }
    return;
}

pub fn send_window_msg_1008_0a3c(param_1: u32, param_2: i32) -> u16 {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
  // u_var3 = (param_1  >> 0x10);
    if (param_2 & 0xfff0) == 0xf140 {
        return (i_var2 + 0xde);
    }
    if (param_2 & 0xfff0) == 0xf060 {
        b_var1 = IsIconic16((i_var2 + 8));
        if b_var1 == false {
            // PostMessage16(0, 0x67, 0x111, (i_var2 + 8));
            PostMessage16(i_var2 + 8, 0x111, 0x67, 0);
        }
        return 0;
    }
    return 1;
}

pub fn get_message_1040_81b6(param_1: u32) {
    let b_var1: bool;
    let mut u_var2: u16;
    let unaff_ss: HWND16;

  // u_var2 = (param_1  >> 0x10);
    (param_1 + 0x78) = 1;
    loop {
        b_var1 = IsWindow16((param_1 + 6));
        if b_var1 == false {
            return;
        }
        // b_var1 = GetMessage16(0, 0, 0, &stack0xffec);
        b_var1 = GetMessage16(&stack0xffec, 0, 0, 0);
        if b_var1 == false {
            break;
        }
        IsDialogMessage16(unaff_ss, &stack0xffec);
    }
    return;
}

pub fn post_quit_msg_1008_3af4() {
    PostQuitMessage16(0);
    return;
}

pub fn post_msg_1038_a3d2(param_1: u32) {
    let h_wnd: HWND16;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    h_wnd = GetWindowWord16((param_1 + 6), -8);
    // PostMessage16(0, 0x105, 0x111, h_wnd);
    PostMessage16(h_wnd, 0x111, 0x105, 0);
    window::destroy_win_1040_7b98(param_1, u_var1, 1);
    return;
}

pub fn send_win_msg_1020_29d8(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u32;
    let ppVar2: &mut Struct2551;
    let mut i_var3: i32;
    let mut in_stack_0000fff6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // i_var3 = (param_3  >> 0x10);
    u_var1 = post_win_msg_1020_79fc(param_1, param_2, param_3_00, param_3, i_var3);
    ppVar2 = struct_ops_2::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x29),
    );
    if i_var3 == 0 {
        pass1_1018_270e(ppVar2, 1, (param_1 + 0xfc));
    } else {
        pass1_1018_270e(ppVar2, 0, (param_1 + 0xfc));
        // SendMessage16(0, 0x69, 0x111, ctx.g_h_window);
        SendMessage16(ctx.g_h_window, 0x111, 0x69, 0);
    }
    return u_var1;
}

pub fn send_win_msg_1040_4a0a(param_1: &mut u32) -> LRESULT {
    let pu_var1: &mut u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u32;
    let h_wnd: HWND16;
    let mut u_var5: u16;
    let w_var6: WPARAM;
    let mut u_var7: u16;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let l_var10: LRESULT;
    let l_param: LPARAM;
    let mut u_var11: u16;
    let h_var12: HWND16;
    let mut local_a: u16;
    let mut local_4: u16;

  // u_var9 = (param_1  >> 0x10);
    i_var8 = param_1;
    unsafe {
        ppc_var2 = (*param_1 + 0x74);
    }
    ppc_var2();
    // h_wnd = GetDlgItem16(6000, (i_var8 + 6));
    h_wnd = GetDlgItem16(i_var8 + 6, 6000);
    // SendMessage16(0, 0, 0x40b, h_wnd);
    SendMessage16(h_wnd, 0x40b, 0, 0);
    // LVar10 = SendMessage16(0, 0, 0xb, h_wnd);
    SendMessage16(h_wnd, 0xb, 0, 0);
    // u_var7 = (LVar10  >> 0x10);
    local_a = 0;
    while (
        u_var3 = (i_var8 + 0x90),
        pu_var1 = (u_var3 + 0x10),
        unsafe { *pu_var1 != local_a } && unsafe { local_a <= *pu_var1 },
    ) {
        w_var6 = 0;
        u_var11 = 0x403;
        u_var3 = (i_var8 + 0x90);
        u_var3 = (u_var3 + 0xc);
        u_var5 = local_a;
        h_var12 = h_wnd;
        pass1_1040_4dcc(param_1, *(u_var3 + local_a * 2));
        // LVar10 = SendMessage16(
        //     CONCAT13((u_var7 >> 8), CONCAT12(u_var7, u_var5)),
        //     w_var6,
        //     u_var11,
        //     h_var12,
        // );
        l_var10 = SendMessage16(h_var12, u_var11, w_var6, CONCAT22(u_var7, u_var5));
      // u_var7 = (LVar10  >> 0x10);
        local_a = local_a + 1;
    }
    w_var6 = pass1_1040_4d7e(param_1);
    if w_var6 == 0 {
        u_var11 = 0x40a;
        u_var4 = (i_var8 + 0x90);
        u_var3 = (i_var8 + 0x94);
        h_var12 = h_wnd;
      // l_param = pass1_1010_ada6(u_var3, (u_var3  >> 0x10), 0, (u_var4 + 10));
      //   SendMessage16(l_param, w_var6, u_var11, h_var12);
        SendMessage16(h_var12, u_var11, w_var6, l_param);
    }
    // l_var10 = SendMessage16(0, 1, 0xb, h_wnd);
    l_var10 = SendMessage(h_wnd, 0xb, 1, 0);
    return l_var10;
}

pub fn post_win_msg_1020_7308(param_1: u32, param_2: i32) {
    let mut cVar1: u8;
    let mut u_var2: u16;

  // u_var2 = (param_1  >> 0x10);
    if param_2 != 0x12 {
        if param_2 < 0x13 {
            cVar1 = param_2;
            if cVar1 == 0x1 {
                (param_1 + 0x1c) = 0;
                return;
            }
            if ('\x03' < (cVar1 - 1)) && ((cVar1 + -5) < 0x2) {}
            // goto LAB_1020_7310;
        }
        return;
    }
    // LAB_1020_7310:
    // PostMessage16(0, 0xeb, 0x111, (param_1 + 4));
    PostMessage16(param_1 + 4, 0x111, 0xeb, 0);
    rect::invalidate_rect_1020_735a(param_1);
    return;
}

pub fn send_win_msg_1020_65cc(ctx: &mut AppContext, param_1: &mut  Struct672, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let b_var3: bool;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut local_4: u16;

    i32_var6 = param_1;
  // u_var8 = (param_1  >> 0x10);
    if param_2 == 1 {
        (i32_var6 + 0x14) = 0;
        return;
    }
    if param_2 == 2 {
        local_4 = 0;
        while local_4 < 5 {
            i_var5 = i32_var6 + 0x18;
            i_var7 = local_4 * 4;
            if ((i_var5 + i_var7 + 2) | (i_var5 + i_var7)) != 0 {
                pp_var1 = ((i_var5 + i_var7) + 4);
                (**pp_var1)();
            }
            local_4 = local_4 + 1;
        }
    } else {
        if ((0 < param_2 + -3) && (!SBORROW2(param_2 - 3, 1))) && (param_2 - 4 < 4) {
            b_var3 = IsIconic16(ctx.g_h_window);
            if b_var3 == 0 {
                b_var3 = IsIconic16((i32_var6 + 4));
                if (b_var3 == 0) && (u_var2 = (i32_var6 + 0x14), (u_var2 + 0x24) != 0) {
                    // InvalidateRect16(0, 0x0, 0);
                    InvalidateRect16(0, 0, 0);
                    pass1_1020_64d4(param_1, 2);
                    if b_var3 == 0 {
                        call_fn_ptr_1_1020_6746(param_1, 1, 2);
                    }
                    pass1_1020_64d4(param_1, 3);
                    if b_var3 == 0 {
                        call_fn_ptr_1_1020_6746(param_1, 1, 3);
                    }
                    u_var4 = pass1_1018_255e((i32_var6 + 0x14));
                    if u_var4 == 0 {
                        // SendMessage16(0, 0x69, 0x111, (i32_var6 + 4));
                        SendMessage16(i32_var6 + 4, 0x111, 0x69, 0);
                    } else {
                        pass1_1020_64d4(param_1, 1);
                        if u_var4 == 0 {
                            call_fn_ptr_1_1020_6746(param_1, 1, 1);
                        }
                    }
                    // SendMessage16(0, 0xf0, 0x111, (i32_var6 + 4));
                    SendMessage16(i32_var6 + 4, 0x111, 0xf0, 0);
                    u_var2 = (i32_var6 + 0x2c);
                    if (u_var2 + 0x7a) != 0 {
                        u_var2 = (i32_var6 + 0x2c);
                        (u_var2 + 0x7a) = 0;
                        // SendMessage16(0, 0x131, 0x111, (i32_var6 + 4));
                        SendMessage16(i32_var6 + 4, 0x111, 0x131, 0);
                        return;
                    }
                }
            }
        }
    }
    return;
}

pub unsafe fn post_msg_1020_55b0(ctx: &mut AppContext, param_1: u32) {
    let pp_var1: fn();
    let pu_var2: &mut  u16;
    let pu_var3: &mut  u16;
    let mut u_var4: u16;
    let pu_var5: &mut  u16;
    let struct_a: &mut  Struct199;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let pp_var9: &mut  Struct2551;
    let mut in_stack_0000fed8: u16;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_114: u16;
    let mut local_112: [u8; 2];
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: String;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass8_funcs::process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fed8, 2));
    local_8 = (local_6 + 0x20);
    local_c = pass8_funcs::process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000fed8, 0x37),
    );
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_10c),
        0x59c,
    );
    pass1_1008_9436(CONCAT22(unaff_ss, local_112));
    pu_var2 = &local_114;
    load_string_1008_a8f4(
        local_c, pu_var2, unaff_ss, 0xee, unaff_ss, &local_10e, unaff_ss,
    );
    local_118 = CONCAT22(ctx.dx_reg, pu_var2);
    struct_a = (ctx.dx_reg | pu_var2);
  // u_var7 = (param_1  >> 0x10);
    if (struct_a != 0x0) && (*local_118 != '\0') {
        u_var8 = 0x1000;
        pu_var3 = pu_var2;
        process_struct_1000_179c(0xb4, struct_a);
        local_120 = CONCAT22(struct_a, pu_var3);
        u_var6 = struct_a | pu_var3;
        if u_var6 == 0 {
            u_var4 = 0;
            u_var6 = 0;
        } else {
            u_var8 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pu_var5 = process_struct_1040_8478(
                local_120,
                0,
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_10c)),
                CONCAT22(ctx.dx_reg, pu_var2),
                (param_1 + 8),
            );
            u_var4 = SUB42(pu_var5, 0);
        }
        local_11c = CONCAT22(u_var6, u_var4);
        if local_110 == 0 {
            pp_var1 = (*local_11c + 0x74);
            (**pp_var1)(u_var8, u_var4, u_var6);
        } else {
            pp_var1 = (*local_11c + 0x6c);
            (**pp_var1)(u_var8, u_var4, u_var6, local_112);
        }
        if (local_8 == 0) || (local_114 == 0) {
            // PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
            PostMessage16(ctx.g_h_window, 0x111, 0xfc, 0);
        }
    }
    if (local_8 != 0) && (local_114 != 0) {
        // SendMessage16(0, local_114, 0x111, ctx.g_h_window);
        SendMessage16(ctx.g_h_window, 0x111, local_114, 0);
        (param_1 + 0x112) = 1;
        return;
    }
    if local_10e == 6 {
        // PostMessage16(0, 0xb0, 0x111, ctx.g_h_window);
        PostMessage16(ctx.g_h_window, 0x111, 0xb0, 0);
        pp_var9 = pass8_funcs::process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_126, 2));
        local_126 = pp_var9;
        (local_126 + 0x20) = 1;
    }
    if local_10e == 0x15 {
        PostMessage16(0, 0x97, 0x111, ctx.g_h_window);
        pp_var9 = pass8_funcs::process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_126, 2));
        (pp_var9 + 0x20) = 1;
    }
    return;
}

pub unsafe fn win_fn_1040_3374(param_1: u32, param_2: &mut u32, hwnd_param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: Struct7;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let lresult_var6: LRESULT;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    // u_var5 = SUB42(offset, 0);
    // LVar6 = winapi::SendMessage16(0, 0, 0x40b, param_3);
    let mut lresult_var6 = winapi::SendMessage16(hwnd_param_3, 0x40b, 0, 0);
    u_var2 = lresult_var6;
    // let _var4 = (param_2  >> 0x10);
    let pp_var1 = (*param_2 + 0x10);
    (**pp_var1)(offset, param_2, u_var4);
    _local_6 = CONCAT22(ctx.dx_reg, u_var2);
    local_a = 0;
    loop {
        if _local_6 <= local_a {
            return;
        }
        pp_var1 = (*param_2 + 4);
        u_var3 = _local_6;
        (**pp_var1)(u_var5, param_2, u_var4, local_a, (local_a >> 0x10));
        u_var2 = u_var3;
        pass1_1018_3a7a(
            *(param_1 + 0x96),
            CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var2)),
        );
        // lresult_var6 = winapi::SendMessage16(CONCAT22(ctx.dx_reg, u_var2), 0, 0x403, hwnd_param_3);
        lresult_var6 = winapi::SendMessage16(hwnd_param_3, 0x403, 0, u_var2);
        u_var5 = 0x1000;
        error_check_1000_17ce(ctx, &mut u_var2);
        if lresult_var6 == -1 {
            break;
        }
        if lresult_var6 == -2 {
            return;
        }
        local_a = local_a + 1;
    }
    return;
}
