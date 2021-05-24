use std::intrinsics::offset;

use crate::app_context::AppContext;
use crate::cleanup::{win_cleanup_1010_305a, win_cleanup_func_1040_782c, window_msg_func_1010_7300};
use crate::err_ops::error_check_1000_17ce;
use crate::func_ptr_funcs::call_fn_ptr_1000_24cd;
use crate::mem_funcs::mem_ops_1::StructuredData;
use crate::other_funcs::{mixed_fn_1010_830a, switch_stmt_1008_aaa8};
use crate::pass::pass13_funcs::pass1_1008_941a;
use crate::pass::pass16_funcs::{pass1_1028_8d9e, pass1_1028_8dec};
use crate::pass::pass17_funcs::{pass1_1030_73a8, pass1_1030_835a};
use crate::pass::pass19_funcs::{pass1_1038_df5c, pass1_1040_a5d0};
use crate::pass::pass6_funcs::pass1_1038_4f54;
use crate::pass::pass7_funcs::pass1_1018_3a7a;
use crate::pass::pass8_funcs::{pass1_1008_eb74, pass1_1010_1ea6, pass1_1010_375e, pass1_1010_3770, pass1_1010_4674, pass1_1010_4788, pass1_1010_5120, pass1_1010_52fc, pass1_1010_531c, pass1_1010_7818, pass1_1010_7b8c};
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_fn_1000_3e2c};
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e};
use crate::struct_ops::{process_struct_1010_20ba, struct_ops_2};
use crate::struct_ops::struct_ops_2::{pass1_1038_df86, process_struct_1000_179c, process_struct_1008_e586};
use crate::structs::prog_structs_2::{Struct199, Struct318, Struct7};
use crate::structs::prog_structs_23::{Struct1090, WinStruct42};
use crate::structs::prog_structs_24::{Struct103, Struct2111, Struct369};
use crate::structs::prog_structs_9::Struct637;
use crate::sys_ops::win_msg;
use crate::typedefs::{HANDLE16, HWND16, LRESULT, WPARAM16};
use crate::ui_ops::misc::{mixed_1040_8520, win_fn_1020_1294, win_gui_fn_1010_79aa};
use crate::ui_ops::misc::pass1_1038_af40;
use crate::ui_ops::window::win_fn_1020_7270;
use crate::util::{CONCAT12, CONCAT13, CONCAT22, CONCAT31, SUB21, SUB42};
use crate::winapi;
use crate::winapi::CreateWindowEx16;

pub unsafe fn destroy_win_1040_13b2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let paVar4: Struct199;

    let struct_a: Struct103;
    let mut u_var5: i32;
    let struct_a_00: Struct103;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let LVar10: LRESULT;
    let mut local_232: u16;
    let mut local_230: u16;
    let paStack556: Struct199;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: u16;
    let local_100: [Struct199; 6];
    let mut local_ae: u16;
    let mut local_ac: u16;
    let local_aa: [Struct199; 6];
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    i32_var6 = param_1;
  // u_var8 = (param_1  >> 0x10);
    if (param_2 != 0) {
        local_56 = winapi::GetDlgItem16(0xfd2, (i32_var6 + 6));
        winapi::SendMessage16(CONCAT22(unaff_ss, local_54), 0x51, 0xd, local_56);
        local_58 = local_54;
        local_232._0_1_ = unaff_ss;
        local_232._1_1_ = (unaff_ss >> 8);
        pass1_fn_1000_3e2c(CONCAT13(local_232._1_1_, CONCAT12(local_232, local_58)));
        local_ac = winapi::GetDlgItem16((s_vrpal_bmp_1050_183a + 4), (i32_var6 + 6));
        LVar10 = winapi::SendMessage16(0, 0, 0x407, local_ac);
        local_ae = LVar10;
        if (local_ae != 0xffff) {
            winapi::SendMessage16(CONCAT22(unaff_ss, local_aa), local_ae, 0x408, local_ac);
        }
        local_ac = winapi::GetDlgItem16((s_vrpal_bmp_1050_183a + 5), (i32_var6 + 6));
        LVar10 = winapi::SendMessage16(0, 0, 0x407, local_ac);
        paVar4 = LVar10;
        local_ae = paVar4;
        if (paVar4 != 0xffff) {
            paStack556 = local_ac;
            LVar10 = winapi::SendMessage16(
                CONCAT13(local_232._1_1_, CONCAT12(local_232, local_100)),
                paVar4,
                0x408,
                local_ac,
            );
            paVar4 = LVar10;
        }
      // paStack556 = (ctx._g_struct_73_1050_14cc  >> 0x10);
        load_string_1010_847e();
        _local_104 = CONCAT22(ctx.dx_reg, paVar4);
        paStack556 = local_100;
        struct_a = local_aa;
        local_230._0_1_ = SUB21(struct_a, 0);
        pass1_1000_3d7a(local_230);
        if (paVar4 != 0x0) {
            paStack556 = _local_104;
            paVar4 = local_aa;
            local_230._0_1_ = SUB21(paVar4, 0);
            pass1_1000_3d7a(local_230);
            if (paVar4 != 0x0) {
                paStack556 = _local_104;
                paVar4 = local_100;
                pass1_1000_3d7a(0);
                if (paVar4 != 0x0) {
                    paStack556 = local_aa;
                    pass1_1010_531c((i32_var6 + 0x8e), CONCAT22(unaff_ss, paStack556));
                    paVar4 = local_100;
                    paStack556 = paVar4;
                    pass1_1010_52fc((i32_var6 + 0x8e), CONCAT22(unaff_ss, paVar4));
                    u_var1 = (i32_var6 + 0x8e);
                  // paStack556 = (u_var1  >> 0x10);
                    pass1_1010_5120(u_var1, local_58);
                    _local_10a = CONCAT22(local_108, paVar4);
                    if (paVar4 == 0x0) {
                        paStack556 = 0x1010;
                        local_10c = struct_a_00;
                        process_struct_1000_179c(0xb4, &mut struct_a_00);
                        u_var5 = local_10c | paVar4;
                        local_10e = paVar4;
                        if (u_var5 == 0) {
                            paVar4 = 0x0;
                            u_var5 = 0;
                        } else {
                            paStack556 = 0x57b;
                            mixed_1040_8520(
                                CONCAT13((local_10c >> 8), CONCAT12(local_10c, paVar4)),
                                ctx.g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x7d2,
                            );
                        }
                        local_230._0_1_ = u_var5;
                        local_230._1_1_ = (u_var5 >> 8);
                        ppc_var2 = (CONCAT13(local_230._1_1_, CONCAT12(local_230, paVar4)) + 0x74);
                        paStack556 = paVar4;
                        ppc_var2();
                        return;
                    }
                    u_var3 = (i32_var6 + 0x8e);
                    local_112 = (u_var3 + 0x12);
                    u_var3 = (i32_var6 + 0x8e);
                  // u_var9 = (u_var3  >> 0x10);
                    i_var7 = u_var3;
                    local_116 = (i_var7 + 0x16);
                    u_var3 = (i32_var6 + 0x8e);
                    local_106 = (u_var3 + 10);
                    paStack556 = local_116;
                    pass1_1028_8d9e(
                        CONCAT22(unaff_ss, &stack0xfdd2),
                        local_106,
                        local_112,
                        local_116 & 0xffff | *(i_var7 + 0x18) << 0x10,
                    );
                    paStack556 = &stack0xfdd2;
                    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, paStack556));
                    paStack556 = &stack0xfdd2;
                    unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
                    pass1_1028_8dec(CONCAT22(unaff_ss, paStack556));
                    // goto LAB_1040_1619;
                }
            }
        }
        paStack556 = 0x1000;
        unaff_cs = 0x1000;
        process_struct_1000_179c(0xb4, &mut struct_a);
        u_var5 = struct_a | paVar4;
        local_10c = struct_a;
        local_10e = paVar4;
        if (u_var5 == 0) {
            paVar4 = 0x0;
            u_var5 = 0;
        } else {
            paStack556 = 0x57b;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, paVar4)),
                ctx.g_h_window,
                0x30,
                2,
                0x57b,
                0x755,
            );
        }
        _local_10a = CONCAT22(u_var5, paVar4);
        ppc_var2 = (*_local_10a + 0x74);
        paStack556 = paVar4;
        ppc_var2();
    }
    // LAB_1040_1619:
    paStack556 = unaff_cs;
    winapi::DestroyWindow16((i32_var6 + 6));
    return;
}

pub unsafe fn win_fn_1038_da68(ctx: &mut AppContext, param_1: &mut u32, param_2: u16, param_3_00: &mut String, param_3: &mut Struct103) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: Vec<u8>;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let pu_var6: Vec<u8>;
    let mut u_var7: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if param_3_00 == 0x204 {
        bad1_1038_de20(
            CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
            (ctx.s_SOLDefaultWindowClass_1050_01fe + 6),
            param_3,
            param_3._2_2_,
        );
        return;
    }
    local_6 = 0;
    local_8 = 0;
    if param_3._2_2_ == (ctx.s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x11) {
        local_a = 1;
        local_6 = 0x6ec0000;
        local_8 = 0x15;
    } else {
        if (ctx.s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x11) < param_3._2_2_ {
            if param_3._2_2_ == (ctx.s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x12) {
                u_var7 = 0x14;
            } else {
                if (param_3._2_2_ != (ctx.s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x13)) {
                    if param_3._2_2_ + -0x125 == 0x0 {
                        pp_var1 = (ctx.g_struct_1050_02a0 + 4);
                        param_3._2_2_ = param_3._2_2_ + -0x125;
                        (**pp_var1)();
                        (param_1 + 0x90) = 1;
                        mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, 0x1db);
                        (param_1 + 0x8e) = 0x100;
                        ctx.dx_reg = ctx.dx_reg;
                    } else {
                        pu_var3 = param_3._2_2_ + -0x126;
                        if pu_var3 == 0x0 {
                            (param_1 + 0x8e) = 0;
                            mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, 0xcb0001);
                            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
                            local_e = CONCAT22(ctx.dx_reg, pu_var3);
                            ctx.dx_reg = ctx.dx_reg;
                            param_3._2_2_ =
                                // winapi_funcs::WinHelp16(0, 3, CONCAT22(ctx.dx_reg, pu_var3), (param_1 + 6));
                            winapi::WinHelp(param_1 + 6, pu_var3, CONCAT22(ctx.dx_reg, pu_var3), 3, 0);
                        } else {
                            if (param_3._2_2_ + -0x127 != 0x0) {}
                            // goto LAB_1038_dc20;
                            param_3._2_2_ = param_3._2_2_ + -0x127;
                            win_msg::post_win_msg_1038_dcb0(param_1, param_2);
                            ctx.dx_reg = ctx.dx_reg;
                        }
                    }
                    // goto LAB_1038_dac3;
                }
                u_var7 = 0x28;
            }
            // LAB_1038_db1c:
            pu_var6 = pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), u_var7);
            ctx.dx_reg = (pu_var6 >> 0x10);
            param_3._2_2_ = pu_var6;
        } else {
            if (param_3._2_2_ + -0x100 == 0x0) {
                param_3._2_2_ = param_3._2_2_ + -0x100;
                if ((param_1 + 0x8e) == 0) {
                    u_var2 =
                        pass1_1010_1ea6(ctx.g_struct_1050_02a0, CONCAT22(param_2, param_1));
                    param_3._2_2_ = CONCAT31(extraout_var, u_var2);
                    (param_1 + 0x90) = 0;
                }
                local_6 = CONCAT22(0x72c, local_6);
                local_8 = 0x48;
            } else {
                if (param_3._2_2_ + -0x11c == 0x0) {
                    param_3._2_2_ = param_3._2_2_ + -0x11c;
                    pass1_1038_df86(CONCAT22(param_2, param_1));
                    ctx.dx_reg = ctx.dx_reg;
                } else {
                    if (param_3._2_2_ + -0x11d != 0x0) {
                        if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0xe)) {
                            u_var7 = 0x1d;
                        } else {
                            if (param_3._2_2_
                                != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x10))
                            {
                                // LAB_1038_dc20:
                                win_msg::post_win_msg_1040_7b3c(
                                    &mut param_1,
                                    param_2,
                                    param_3_00,
                                    param_3,
                                    param_3._2_2_,
                                );
                                return;
                            }
                            u_var7 = 0x1c;
                        }
                        // goto LAB_1038_db1c;
                    }
                    param_3._2_2_ = param_3._2_2_ + -0x11d;
                    pass1_1038_df5c(CONCAT22(param_2, param_1));
                    ctx.dx_reg = ctx.dx_reg;
                }
            }
        }
    }
    // LAB_1038_dac3:
    if (local_6._2_2_ == 0) {
        return;
    }
    if local_6 == 0 {
        process_struct_1000_179c(0xb4, ctx.dx_reg);
        u_var4 = ctx.dx_reg | param_3._2_2_;
        local_12 = param_3._2_2_;
        local_10 = ctx.dx_reg;
        if u_var4 != 0 {
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, param_3)),
                (param_1 + 6),
                0,
                2,
                0x634,
                local_6,
            );
            // goto LAB_1038_dc37;
        }
    } else {
        process_struct_1000_179c(0xb4, ctx.dx_reg);
        u_var4 = ctx.dx_reg | param_3._2_2_;
        local_12 = param_3._2_2_;
        local_10 = ctx.dx_reg;
        if u_var4 != 0 {
            u_var5 = &ctx.PTR_LOOP_1050_1040;
            mixed_1040_8520(
                param_3,
                (param_1 + 6),
                0,
                3,
                0x634,
                local_6._2_2_,
            );
            // goto LAB_1038_dc37;
        }
    }
    u_var5 = 0x1000;
    param_3._2_2_ = 0x0;
    u_var4 = 0;
    // LAB_1038_dc37:
    local_e = CONCAT22(u_var4, param_3._2_2_);
    if (local_8 == 0) {
        pp_var1 = (*local_e + 0x74);
        (**pp_var1)(u_var5, param_3._2_2_, u_var4);
    } else {
        pass1_1008_941a(CONCAT22(unaff_ss, local_16), 1, local_8);
        pp_var1 = (*local_e + 0x6c);
        (**pp_var1)(
            &ctx.PTR_LOOP_1050_1008,
            local_e,
            (local_e >> 0x10),
            local_16,
        );
    }
    return;
}

pub fn win_fn_1038_d118(param_1: u32, param_2: u32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut cVar3: u8;

    let h_var4: HANDLE16;
    let HVar5: HANDLE16;
    let mut u_var6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    h_var4 = winapi::GetProp16(CONCAT22(in_ax, 0x5bf3), param_3);
    HVar5 = winapi::GetProp16(CONCAT22(in_ax, 0x5bec), param_3);
    _local_6 = CONCAT22(h_var4, HVar5);
    if (param_2._2_2_ == 0x30) {
        if (param_2 == 0) {
            return;
        }
        SetProp16(param_2, CONCAT22(in_ax, 0x5c06), param_3);
        return;
    }
  // u_var6 = (param_1  >> 0x10);
    if (param_2._2_2_ < 0x31) {
      // cVar3 = (param_2  >> 0x10);
        if (cVar3 == 0x2) {
            if ((h_var4 | HVar5) != 0) {
                u_var2 = *_local_6;
                pp_var1 = u_var2 + 6;
                (**pp_var1)(
                    offset,
                    HVar5,
                    h_var4,
                    param_1,
                    u_var6,
                    param_2,
                    param_2._2_2_,
                );
                if (_local_6 != 0x0) {
                    pp_var1 = u_var2;
                    (**pp_var1)(offset, HVar5, h_var4, 1);
                }
            }
            h_var4 = winapi::GetProp16(CONCAT22(in_ax, 0x5bfa), param_3);
            if (h_var4 == 0) {
                return;
            }
            winapi::DeleteObject16(h_var4);
            winapi::RemoveProp16(CONCAT22(in_ax, 0x5c00), param_3);
            return;
        }
        if (cVar3 == '\x06') {
            if ((param_2 != 1) && (param_2 != 2)) {
                u_var2 = &PTR_LOOP_1050_5bc8;
                (u_var2 + 8) = 0;
                return;
            }
            u_var2 = &PTR_LOOP_1050_5bc8;
            (u_var2 + 8) = param_3;
            return;
        }
    }
    if ((h_var4 | HVar5) != 0) {
        pp_var1 = (*_local_6 + 0xc);
        (**pp_var1)(
            offset,
            HVar5,
            h_var4,
            param_1,
            u_var6,
            param_2,
            param_2._2_2_,
        );
    }
    return;
}

pub fn call_win_proc_1038_d020(param_1: u16, param_2: u32, param_3: u32) {
    let pp_var1: fn();

    let HVar2: HANDLE16;
    let win_proc: &mut Vec<u8>;
    let HVar3: HANDLE16;
    let mut u_var4: i32;
    let LVar5: LRESULT;
    let mut u_var6: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    HVar2 = winapi::GetProp16(CONCAT22(in_ax, 0x5bd7), param_3._2_2_);
    win_proc = winapi::GetProp16(
        CONCAT13((in_ax >> 8), CONCAT12(in_ax, 0x5bd0)),
        param_3._2_2_,
    );
    _local_6 = CONCAT22(HVar2, win_proc);
    HVar2 = winapi::GetProp16(CONCAT22(in_ax, 0x5be5), param_3._2_2_);
    HVar3 = winapi::GetProp16(CONCAT22(in_ax, 0x5bde), param_3._2_2_);
    _local_a = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0) {
        local_e = 0;
        if (param_3 == 0x19) {
            pp_var1 = (*_local_a + 0x34);
            local_e = (**pp_var1)(offset, HVar3, HVar2, param_1, param_2);
        } else {
            if (param_3 == 0x86) {
                pp_var1 = (*_local_a + 0x20);
                u_var4 = (**pp_var1)(offset, HVar3, HVar2, param_2._2_2_);
                // goto LAB_1038_d10e;
            }
            if ((param_3 == 0x112) && ((param_2._2_2_ & 0xfff0) == 0xf140)) {
                LVar5 = winapi::SendMessage16(0, 0xf140, 0x112, &ctx.g_h_window);
                u_var4 = (LVar5 == 0);
                // goto LAB_1038_d10e;
            }
        }
        if (local_e != 0) {
            return local_e;
        }
    }
    if (_local_6 != 0) {
        u_var6 = CallWindowProc16(
            CONCAT22(param_2, param_1),
            param_2._2_2_,
            param_3,
            param_3._2_2_,
            win_proc,
        );
        return u_var6;
    }
    u_var4 = 0;
    // LAB_1038_d10e:
    return u_var4;
}

pub unsafe fn destroy_win_1038_cc00(param_1: &mut u32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    if (param_3._2_2_ == 0x1cd) {
        u_var2 = 1;
    } else {
        if (param_3._2_2_ == 0x1ce) {
            u_var2 = 2;
        } else {
            if (param_3._2_2_ == 0x1cf) {
                u_var2 = 3;
            } else {
                if (param_3._2_2_ == 0x1d0) {
                    u_var2 = 4;
                } else {
                    if (param_3._2_2_ != 0x1d1) {
                        win_msg::post_win_msg_1040_7b3c(
                            param_1,
                            param_2,
                            param_3_00,
                            param_3,
                            param_3._2_2_,
                        );
                        return;
                    }
                    u_var2 = 5;
                }
            }
        }
    }
    i_var1 = pass1_1008_eb74((param_1 + 0x8e), u_var2);
    if (i_var1 != 0) {
        mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, CONCAT22(i_var1, 1));
        winapi::DestroyWindow16((param_1 + 6));
    }
    return;
}

pub unsafe fn destroy_win_1038_c836(param_1: &mut u32, param_2: &mut String, param_3: u16, param_4: u32) {
    let mut u_var1: u32;
    let mut local_6: [u8; 4];

    if (param_2._2_2_ == 0xfce) {
        pass1_1008_941a(CONCAT22(unaff_ss, local_6), 1, 0xac);
        mci_send_command_1008_5c9e(ctx.g_struct_1050_02a0, CONCAT22(unaff_ss, local_6));
        u_var1 = (param_1 + 0x8e);
        (u_var1 + 10) = 6;
        winapi::DestroyWindow16((param_1 + 6));
        PTR_LOOP_1050_5b80 = 0x0;
        return;
    }
    win_msg::post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2_00), param_2);
    return;
}

pub unsafe fn win_fn_1038_c374(ctx: &mut AppContext, param_1: u32, param_2: &mut u32, param_3: HWND16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let LVar7: LRESULT;
    let struct_var8: &mut Struct7;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = SUB42(offset, 0);
    LVar7 = winapi::SendMessage16(0, 0, 0x40b, param_3);
    u_var3 = LVar7;
  // u_var5 = (param_2  >> 0x10);
    unsafe {
        ppc_var2 = (*param_2 + 0x10);
    }
    ppc_var2(offset, param_2, u_var5);
    _local_6 = CONCAT22(ctx.dx_reg, u_var3);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        unsafe {
            ppc_var2 = (*param_2 + 4);
        }
        u_var4 = _local_6;
        ppc_var2(u_var6, param_2, u_var5, local_a, (local_a >> 0x10));
        u_var1 = (param_1 + 0x8e);
        struct_var8 = process_struct_1008_e586(
            u_var1,
            (u_var1 >> 0x10),
            CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var4)),
        );
        LVar7 = winapi::SendMessage16(struct_var8, 0, 0x403, param_3);
        u_var6 = 0x1000;
        error_check_1000_17ce(ctx, struct_var8);
        if (LVar7 == -1) {
            break;
        }
        if (LVar7 == -2) {
            return;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn win_fn_1038_362e(param_1: &mut Struct1090) {
    let mut i_var1: i32;
    let mut u_var2: i32;
    let mut ppVar3: Struct2111;
    let mut in_stack_0000fff8: u32;
    let mut u_var4: u16;

  // u_var2 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if local_bx_4.field_0x214 == 0 {
      // u_var4 = (in_stack_0000fff8  >> 0x10);
        i_var1 = pass1_1038_4f54((param_1 & 0xffff | u_var2 << 0x10), 0x1f);
        if i_var1 == 0 {
            local_bx_4.field_0x214 = 0x14;
        } else {
            local_bx_4.field_0x214 = 0x28;
        }
       let mut struct_var3 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(u_var4, 0x37));
        win_msg::post_win_msg_1008_a0e4(&mut struct_var3, 0, 0, 1, local_bx_4.field_0x4, 0x38);
        local_bx_4.field_0x216 = 0;
    }
    return;
}

pub unsafe fn win_fn_1030_e67c(ctx: &mut AppContext, param_1: u32) -> u16 {
    let mut u_var1: u16;
    let ppVar2: Struct2111;
    let mut in_stack_0000fff6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = struct_ops_2::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22((in_stack_0000fff6 >> 0x10), 0x37),
    );
  // u_var1 = switch_stmt_1008_aaa8(ppVar2, (ppVar2  >> 0x10), (param_1 + 0x108));
    if (u_var1 != 0) {
        win_msg::post_win_msg_1008_a0e4(&mut ppVar2, 0, 0, 1, 0, u_var1);
    }
    return 1;
}

pub unsafe fn call_win_fn_1020_7526(param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    win_fn_1020_7270(ctx, param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
    return param_1;
}

pub unsafe fn win_fn_1020_0ec4(param_1: &mut u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut cVar3: u8;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;



    let mut u_var7: u16;
    let pp_var8: Struct2111;
    let w_param: WPARAM16;
    let h_wnd: HWND16;
    let mut u_var9: u16;
    let mut local_c: u16;
    let mut local_a: u16;

  // u_var7 = (param_1  >> 0x10);
    i_var4 = param_1;
    if (param_2 == 0xfb) {
        pp_var8 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_c, 0x30));
        u_var9 = SUB42(pp_var8, 0);
        pass1_1010_375e(pp_var8);
        unsafe {
            ppc_var2 = (*param_1 + 0x14);
        }
        ppc_var2(0x1010, param_1, u_var7, 1, u_var9, ctx.dx_reg);
        pass1_1010_375e(pp_var8);
        pass1_1010_4788((i_var4 + 0xf2), CONCAT22(ctx.dx_reg, u_var9));
        return;
    }
    if (0xfb < param_2) {
        match (param_2) {
            _ => {
                return;
            }
            0x12a => {
                h_wnd = (i_var4 + 8);
                w_param = 0xf012;
            }
            300 => {
                h_wnd = (i_var4 + 8);
                w_param = 0xf020;
            }
        }
        winapi::PostMessage16(0, w_param, 0x112, h_wnd);
        return;
    }
    if (param_2 == 0xf3) {
        u_var9 = 3;
    } else {
        if (0xf3 < param_2) {
            return;
        }
        cVar3 = param_2;
        u_var6 = param_2 & 0xff00 | (cVar3 + 0x91);
        if ((cVar3 + 0x91) == 0) {
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
            // winapi_funcs::WinHelp16(0x28, 1, CONCAT22(ctx.dx_reg, u_var6), (i_var4 + 8));
            winapi::WinHelp16(i_var4 + 8, CONCAT22(ctx.dx_reg, u_var6), 1, 0x28);
            return;
        }
        if (cVar3 == 'r') {
            i_var5 = i_var4 + 10;
            u_var9 = u_var7;
            pp_var8 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(i_var5, 0x30));
            pass1_1010_3770(pp_var8, CONCAT22(u_var9, i_var5));
            pass1_1038_af40(ctx._g_Struct112_a, *(i_var4 + 8), 3);
            return;
        }
        if (cVar3 == -0xf) {
            u_var9 = 1;
        } else {
            if (cVar3 != -0xe) {
                return;
            }
            u_var9 = 2;
        }
    }
    u_var1 = (i_var4 + 0xf2);
    pass1_1010_4674(u_var1, (u_var1 >> 0x10), u_var9);
    return;
}

pub fn call_win_fn_1020_0e8e(in_struct_1: &mut Struct637, param_2: u32, param_3: u32) {
    let mut i_var1: i32;
    // fn_ptr_1: &mut Vec<u8>;

  // i_var1 = win_fn_1020_1294(CONCAT22(param_2, in_struct_1), (param_2  >> 0x10), param_3);
    if (i_var1 == 0) {
        fn_ptr_1 = (in_struct_1.field_0x4 + 0x5c);
        (**fn_ptr_1)();
    }
    return;
}

pub fn win_func_1018_6bb6(param_1: u32) {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    if (i_var2 + 0xea) != 0 {
        winapi::PostMessage16(0, (i_var2 + 0xea), 0x111, ctx.g_h_window);
    }
    winapi::PostMessage16(0, 0x79, 0x111, ctx.g_h_window);
    if (i_var2 + 0xf0) != 0 {
        b_var1 = winapi::IsWindow16((i_var2 + 0xf0));
        if (b_var1 != 0) {
            winapi::DestroyWindow16((i_var2 + 0xf0));
            (i_var2 + 0xf0) = 0;
        }
    }
    return;
}

pub unsafe fn win_cleanup_func_1040_b0f8(param_1: &mut Struct7) -> u8 {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u8;
    let mut u_var4: u16;
    let mut ppVar5: Struct2111;
    let mut in_stack_0000ffe8: u32;
    let mut local_a: u16;
    let mut local_8: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0xb772;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    ppVar5 = struct_ops_2::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22((in_stack_0000ffe8 >> 0x10), 0x32),
    );
    pass1_1010_7b8c(&mut ppVar5, local_bx_4.field_0x6);
    if local_bx_4[0x11].field_0x6 != 0 {
        winapi::DeleteObject16(local_bx_4[0x11].field_0x6);
        local_bx_4[0x11].field_0x6 = 0;
    }
    u_var1 = (local_bx_4 + 0x12).field_0x0;
    u_var2 = local_bx_4[0x12].field_0x2;
    _local_a = CONCAT22(u_var2, u_var1);
    if (u_var2 | u_var1) != 0 {
        pass1_1040_a5d0(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(ctx, _local_a);
    }
    u_var3 = win_cleanup_func_1040_782c(ctx, param_1);
    return u_var3;
}

pub fn win_fn_1010_0242(param_1: u16, param_2: u16, param_3: HWND16) -> u16 {
    let pp_var1: fn();

    let BVar2: bool;
    let wVar3: u16;
    let ppVar4: Struct2111;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar2 = winapi::IsWindow16(param_1);
    if (BVar2 != 0) {
        wVar3 = winapi::GetWindowWord16(-6, param_1);
        if (wVar3 == &ctx.g_h_instance_1050_038c) {
            BVar2 = winapi::IsIconic16(param_1);
            if (BVar2 != 0) {
                _local_8 = CONCAT22(local_6, 0x45);
                ppVar4 = struct_ops_2::process_struct_1010_20ba(&g_Struct372_1050_0ed0, _local_8);
                _local_8 = (ppVar4 & 0xffff0000 | param_1);
                pp_var1 = (*_local_8 + 0x10);
                (**pp_var1)(offset, ppVar4, 1);
            }
        }
    }
    return 1;
}

pub unsafe fn win_fn_1010_1656(param_1: &mut Struct318, param_2: u16, param_3: &mut StructuredData, param_4: u16) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let struct_var6: Struct2111;
    let mut u_var7: u32;
    let mut in_stack_0000000c: u16;
    let mut in_stack_0000ffe0: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff6e820591: Struct369;

    win_cleanup_1010_305a(param_1, param_2, param_3);
    if param_1.field_0x16 == 3 {
        struct_var6 = struct_ops_2::process_struct_1010_20ba(
            ctx.g_struct_var_1050_0ed0,
            CONCAT22(param_4, 0x32),
        );
        u_var1 = param_1.field_0x32;
        u_var1 = (u_var1 + 0x42);
        //// _var5 = (u_var1  >> 0x10);
        i_var4 = u_var1;
        u_var1 = (i_var4 + 0x16);
        u_var7 = pass1_1030_73a8((u_var1 + 4));
        i_var2 = u_var7;
        pass1_1010_7818(struct_var6, u_var7);
        u_var1 = (i_var4 + 0x16);
        i_var3 = i_var2;
        win_gui_fn_1010_79aa(&mut struct_var6, 0, (u_var1 + 4));
        if i_var3 == 0 {
            u_var1 = (i_var4 + 0x16);
            u_var1 = (u_var1 + 4);
            window_msg_func_1010_7300(struct_var6, 0, 0, i_var2, u_var1, (u_var1 >> 0x10));
        }
    }
    return;
}

pub fn create_win_1008_9760(ctx: &mut AppContext, struct_param_1: &mut WinStruct42) -> Struct199 {
    let mut win_handle_1: HWND16 = 0;
    let win_struct: WinStruct42;
    let mut u_var1: i32;
    let mut out: Struct199 = Struct199::new();

    if win_struct.win_handle_0x8 == 0 {
        let data: Vec<u8> = in_win_struct::to_le_bytes_vec();
        win_handle_1 = CreateWindowEx16(struct_param_1.style_0xb0, &struct_param_1.class_name_0x5b, &ctx.g_string_1050_039e, struct_param_1.style_0xac, struct_param_1.x_0xb4, struct_param_1.y_0xb6, struct_param_1.width_0xb8, struct_param_1.height_0xba, struct_param_1.parent_window_0xbc, struct_param_1.menu_handle_0xca, ctx.g_h_instance_1050_038c, &data);
        // win_handle_1 = CreateWindowEx16(&data, ctx.g_h_instance_1050_038c, struct_param_1.menu_handle_0xca, struct_param_1.parent_window_0xbc, struct_param_1.height_0xba, struct_param_1.width_0xb8, struct_param_1.y_0xb6, struct_param_1.x_0xb4, struct_param_1.style_0xac, &ctx.g_string_1050_039e, &struct_param_1.class_name_0x5b, struct_param_1.style_0xb0)'

        struct_param_1.win_handle_0x8 = win_handle_1;
    }
    // if in_win_struct.win_handle_0x8 == 0 {
    //     win_handle_1 = call_fn_ptr_1000_24cd(0);
    // }
    struct_param_1.win_handle_0x8 == 0 {
        win_handle_1 = call_fn_ptr_1000_24cd(ctx, None);
    }
    // let mut out: StructuredData = StructuredData::new();
    out.

    out.field_0x2 = ctx.dx_reg;
    out.field_0x16 = win_handle_1;
    return out;
}
