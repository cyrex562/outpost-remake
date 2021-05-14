use crate::{defines::{Struct126, Struct145}, string_funcs::{process_string_1000_28dc, process_string_1000_55b1}, util::CONCAT22, mixed_fn_1010_830a};
use crate::app_context::AppContext;
use crate::prog_structs::prog_structs_2::{Struct306, Struct199, Struct293, Struct223, Struct131, Struct7, Struct163};
use crate::pass8_funcs::{pass1_1008_e9a4, pass1_1010_2ee2, pass1_1010_8ef2, pass1_1010_1d80, pass1_1010_1f62, pass1_1010_905e};
use crate::pass5_funcs::{pass1_1030_8326, pass1_1030_38f2, pass1_1030_4bbe, pass1_1030_8344, pass1_1030_8334, pass1_1030_8128};
use crate::struct_funcs::{process_struct_1000_179c, process_struct_1040_b082, process_struct_1040_7728, process_struct_1040_b0bc, process_struct_1040_c630, process_struct_1010_1d48, process_struct_1008_574a, process_struct_1008_c882, struct_fn_1000_160a, process_struct_1008_50c2, process_struct_1008_4772, process_struct_1008_8d8a, set_struct_1008_9584, process_struct_1008_41bc, set_struct_1008_4016, process_struct_1008_4544, process_struct_1008_4834, process_struct_1008_47cc, set_struct_1008_56b4, process_struct_1000_2f00, process_struct_1000_2e74};
use crate::prog_structs::prog_structs_31::{Struct305, Struct348, Struct113, Struct335, Struct329, Struct289, Struct288, Struct282, Struct281, Struct280, Struct278, Struct277, Struct276, Struct275, Struct264, Struct263, Struct262, Struct259, Struct258, Struct251, Struct250, Struct175, Struct173, Struct169, Struct162, Struct161, Struct160, Struct158, Struct156, Struct157, Struct145, Struct126};
use crate::sys_funcs::{win_cleanup_func_1040_b0f8, get_prop_1040_9566, make_proc_inst_1040_8fb8, reg_class_1008_96d2, LoadCursor16, GetStockObject16, pass1_1030_838e, dos3_call_1000_42de, dos3_call_1000_4f94, dos3_call_1000_5174, dos3_call_1000_2bb6, dos3_call_1000_39f2, dos3_call_1000_370a, swi, process_win_msg_1008_54aa};
use crate::pass6_funcs::{pass1_1038_b6e0, pass1_1038_3608, pass1_1038_4d28};
use crate::prog_structs::prog_structs_24::{pass1_struct_1, Struct249, Struct236, Struct182, Struct103};
use crate::prog_structs::prog_structs_21::{Struct350, Struct343, Struct74, Struct240, Struct164};
use crate::err_funcs::{error_check_1000_17ce, error_check_1000_0dc6, handle_error_1008_9466};
use crate::util::{CARRY1, SBORROW1, CONCAT11, SUB42, CONCAT31, CONCAT12, CONCAT13, ZEXT24, POPCOUNT, SBORROW2, CARRY2, SUB21};
use crate::prog_structs::prog_structs_8::{Struct68, Struct265};
use crate::pass7_funcs::{pass1_1040_79c0, pass1_1018_5732, pass1_1018_5742, pass1_1018_4dce, pass1_1018_4cda, pass1_1018_4842, pass1_1018_20ee};
use crate::prog_structs::prog_structs_28::{Struct346, Struct331, Struct327, Struct290, Struct273, Struct260, Struct255, Struct237, Struct224, Struct207, Struct170, Struct148};
use crate::prog_structs::prog_structs_18::{Struct338, Struct195};
use crate::mem_funcs::{Address, alloc_mem_1000_1708, alloc_mem_1008_909c, alloc_mem_1000_0a48, alloc_mem_1000_07fc, alloc_mem_1000_167a, alloc_mem_1000_16aa};
use crate::ui_funcs::{free_proc_inst_1040_911e, win_cleanup_1040_d1bc, win_cleanup_1018_4d22, win_cleanup_1008_0618, post_quit_msg_1008_3af4};
use crate::prog_structs::prog_structs_15::{Struct337, Struct194, Struct181};
use crate::prog_structs::prog_structs_29::{Struct114, Struct332, Struct328, Struct299, Struct291, Struct286, Struct269, Struct256, Struct243, Struct242, Struct238, Struct229, Struct228, Struct227, Struct226, Struct225, Struct221, Struct220, Struct213, Struct212, Struct192, Struct149, Struct153};
use crate::draw::process_struct_1040_9252;
use crate::prog_structs::prog_structs_30::{Struct333, Struct295, Struct294, Struct292, Struct287, Struct274, Struct261, Struct257, Struct247, Struct246, Struct245, Struct244, Struct239, Struct234, Struct233, Struct232, Struct231, Struct230, Struct203, Struct201, Struct200, Struct417, Struct172, Struct168};
use crate::prog_structs::prog_structs_23::{Struct330, Struct267, Struct193, Struct184};
use crate::bad_funcs::{halt_baddata, bad_fn_1050_335f};
use crate::prog_structs::prog_structs_27::{Struct301, Struct298, Struct285, Struct268, pass1_struct_2, Struct165};
use crate::prog_structs::prog_structs_7::{Struct44, Struct376, Struct189};
use crate::prog_structs::prog_structs_17::{Struct534, Struct284};
use crate::string_funcs::{load_str_1010_84ac, copy_string_1000_3d3e, string_fn_1000_3f9c, get_string_index_1000_3da4, fn_1008_6048, process_string_1000_440c, process_string_1000_2a00};
use crate::pass3_funcs::{pass1_1020_a6ee, pass1_1020_a43e};
use crate::prog_structs::prog_structs_1::{Struct283, Struct104, Struct197};
use crate::prog_structs::prog_structs_13::{Struct279, Struct146};
use crate::prog_structs::prog_structs_25::{Struct272, Struct219, Struct211, Struct65};
use crate::prog_structs::prog_structs_19::Struct271;
use crate::prog_structs::prog_structs_4::Struct270;
use crate::prog_structs::prog_structs_12::{Struct266, Struct235, Struct102};
use crate::prog_structs::prog_structs_26::{Struct254, Struct241, Struct206, Struct196, Struct183};
use crate::prog_structs::prog_structs_20::Struct253;
use crate::prog_structs::prog_structs_10::Struct252;
use crate::prog_structs::prog_structs_16::{Struct248, Struct174};
use crate::file_funcs::{write_to_file_1008_7e1c, write_to_file_1008_7c2a, write_to_file_1008_7cac, read_file_1008_76e4, call_read_file_1020_a65e, read_from_file_1038_7c02, read_file_1028_def2, read_file_1030_5c52, read_file_1028_d7ba, call_write_to_file_1020_a644, write_to_file_1038_7b20, write_to_file_1028_dce2, write_to_file_1030_5c1a, write_to_file_1028_d7a0};
use crate::pass4_funcs::{pass1_1028_e4ec, pass1_1028_dc52, pass1_1028_e2e0};
use crate::prog_structs::prog_structs_11::{Struct222, Struct218, Struct159};
use crate::sound_funcs::mci_send_cmd_1008_5c5c;
use crate::typedefs::{HCURSOR16, HGDIOBJ16};
use crate::other_funcs::{zero_list_1008_3e38, empty_fn_1000_55ac};
use crate::prog_structs::prog_structs_5::{Struct1, Struct150};
use crate::prog_structs::prog_structs_6::Struct675;
use crate::func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_256b, call_fn_ptr_1000_2594};
use crate::prog_structs::prog_structs_22::Struct147;

pub unsafe fn pass1_fn_1000_0c32(param_1: u16, Struct126_: *mut Struct126, param_3: u16) -> u16 {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct145;
    let pb_var3: *mut u8;
    let piVar4: *mut i32;
    let mut u_var5: u32;
    let mut u_var6: i32;
    let local_BX__1: *mut Struct126;
    let pa_var7: *mut Struct145;
    let mut i_var8: i32;
    let pu_var9: *mut u32;
    let pu_var10: *mut u32;
    let pa_var11: *mut Struct145;
    let mut u_var12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let local_c: u8;
    let mut local_a: u16;
    let local_8: *mut Struct145;
    let mut local_6: u16;

    pa_var11 = local_BX__1.field_0xe;
    local_6 = 0;
    pa_var7 = pa_var11;
    while (true) {
        while {
            u_var6 = pa_var7;
            if (param_1 <= u_var6) {
                u_var6 = (u_var6 & 0xfffc) - param_1;
                pu_var1 = &local_BX__1.field_0x12;
                local_8 = pa_var7;
                let pu_var1_val = unsafe { *pu_var1 };
                if (pu_var1_val < u_var6 || pu_var1_val == u_var6) {
                    local_e = param_1;
                    if ((param_3 & 6) == 0) {
                        local_8 = (u_var6 + pa_var7);
                        local_8[-1].field_0x4 = u_var6;
                        pa_var7 = u_var6 | 2;
                        pu_var9 = pa_var7.field_0x2;
                        pb_var3 = (&local_8.field_0x0 + param_1);
                        unsafe {
                            unsafe { *pb_var3 = *pb_var3 | 2 };
                            unsafe { *local_8 = param_1 | 1 };
                        }
                    } else {
                        unsafe {
                            unsafe {
                                *pa_var7 = param_1 & 0xff00 | *pa_var7 & 2 | param_1 & 0xff | 1
                            }
                        };
                        (pa_var7.field_0x4 + 2) = pa_var7.field_0x2;
                        (pa_var7.field_0x2 + 4) = pa_var7.field_0x4;
                        pu_var9 = (&pa_var7.field_0x0 + param_1);
                        (pu_var9 + (u_var6 - 2)) = u_var6;
                        unsafe { unsafe { *pu_var9 = u_var6 | 2 } };
                        u_var6 = local_BX__1.field_0x10;
                        pu_var9[2] = u_var6;
                        pu_var9[1] = (u_var6 + 2);
                        *((u_var6 + 2) + 4) = pu_var9;
                        *(u_var6 + 2) = pu_var9;
                    }
                } else {
                    pu_var9 = pa_var7.field_0x2;
                    *(pa_var7.field_0x4 + 2) = pu_var9;
                    (pa_var7.field_0x2 + 4) = pa_var7.field_0x4;
                    paVar2 = pa_var7;
                    unsafe { unsafe { *paVar2 = *paVar2 | 1 } };
                    local_e = pa_var7 & 0xfffc;
                    pu_var10 = (&pa_var7.field_0x0 + local_e);
                    unsafe { unsafe { *pu_var10 = *pu_var10 | 2 } };
                }
                local_BX__1.field_0xe = pu_var9;
                if ((param_3 & 1) != 0) {
                    u_var6 = local_e - 2 >> 1;
                    pa_var11 = local_8;
                    while (pa_var11 = &pa_var11.field_0x2, u_var6 != 0) {
                        u_var6 = u_var6 - 1;
                        pa_var11 = 0;
                    }
                    if ((local_e - 2 & 1) != 0) {
                        unsafe { unsafe { *pa_var11 = 0 } };
                    }
                }
                if (((param_3 & 2) != 0) && (pu_var9[1] == pu_var9[2])) {
                    local_BX__1.field_0x4 = *(local_BX__1.field_0x10 + 2) & 0xfffc;
                    u_var5 = local_BX__1.field_0x4;
                    pb_var3 = (u_var5 + 3);
                    unsafe { unsafe { *pb_var3 = *pb_var3 | 0x80 } };
                }
                piVar4 = &local_BX__1.field_0xa;
                unsafe { unsafe { *piVar4 = *piVar4 + 1 } };
                return CONCAT22(0x1050, &local_8.field_0x2);
            }
            if (local_6 < u_var6) {
                local_6 = u_var6;
            }
            pa_var7 = pa_var7.field_0x2;
            (pa_var7 != pa_var11)
        } {}
        if (((param_3 & 2) == 0) || ((param_3 & 0x40) != 0)) {
            break;
        }
        u_var5 = (local_BX__1).field_0x0;
        u_var12 = (u_var5 >> 0x10);
        i_var8 = u_var5;
        if ((i_var8 + 0x34) == 0) {
            break;
        }
        local_6 = (**(i_var8 + 0x34))();
        if ((local_6 < param_1) || (pa_var7 = local_BX__1.field_0xe, pa_var7 == 0x0)) {
            break;
        }
    }
    local_BX__1.field_0x4 = local_6 & 0xfffc;
    return 0;
}

pub unsafe fn pass1_fn_1000_1dfa(param_1: i32, param_2: u8, uparam_3: i32, param_4: i32) -> bool {
    let u_var1: u32;
    let mut u_var2: i32;

    if ((param_2 & 4) == 0) {
        u_var2 = (((-((param_2 & 2) == 0) >> 8) & 0xfe) + 0x92) << 8;
    } else {
        u_var2 = 0x1800;
    }
    if ((param_4 == 0)
        || ((param_4 & 0xff00 & (((-((param_2 & 4) == 0) >> 8) & 0x82) + 0x18) << 8) != u_var2))
    {
        return 1;
    }
    if (param_1 != 0) {
        u_var1 = SegmentLimit(param_4);
        if (CARRY2(param_3, param_1 - 1)) {
            return 1;
        }
        if (u_var1 < param_3 + (param_1 - 1)) {
            return 1;
        }
    }
    return 0;
}

pub unsafe fn pass1_fn_1000_206c(param_1: *mut Struct148, param_2: u16) -> u16 {
    let mut i_var1: i32;

    i_var1 = pass1_fn_1000_21d2(0x102, 0x42, 0, param_1, param_2);
    if ((i_var1 != 0) && (param_1.field_0x14 == -0x4153)) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_fn_1000_20a2(param_1: *mut Struct147, param_2: u16) {
    let mut u_var1: u16;
    let paVar2: *mut Struct147;
    let mut u_var3: u16;
    let paVar4: *mut Struct147;
    let local_DI_58: *mut Struct147;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_5fbe0711d8: *mut Struct146;

    temp_5fbe0711d8 = (param_1 + 1);
    u_var1 = &param_1[1].field_0x2;
    local_8 = 0;
    paVar2 = temp_5fbe0711d8.field_0x4;
    local_4 = temp_5fbe0711d8.field_0x6;
    local_DI_58 = 0x0;
    if ((local_4 | paVar2) != 0) {
        while ((
            paVar4 = paVar2,
            u_var3 = local_4,
            paVar4 != param_1 || (local_4 != param_2),
        )) {
            paVar2 = paVar4.field_0x2a;
            local_4 = paVar4.field_0x2c;
            local_DI_58 = paVar4;
            local_8 = u_var3;
            if ((local_4 | paVar2) == 0) {
                return;
            }
        }
        if ((local_8 | local_DI_58) != 0) {
            u_var3 = paVar4.field_0x2c;
            local_DI_58.field_0x2a = paVar4.field_0x2a;
            local_DI_58.field_0x2c = u_var3;
            return;
        }
        u_var3 = paVar4.field_0x2c;
        temp_5fbe0711d8.field_0x4 = paVar4.field_0x2a;
        temp_5fbe0711d8.field_0x6 = u_var3;
    }
    return;
}

pub unsafe fn pass1_fn_1000_21b6(param_1: i32, param_2: i32) -> u32 {
    let b_var1: bool;

    b_var1 = pass1_fn_1000_1dfa(0, 4, param_1, param_2);
    return (b_var1 == 0);
}

// WARNING: Removing unreachable block (ram,0x100021de)

pub unsafe fn pass1_fn_1000_21d2(
    param_1: i32,
    param_2: libc::c_long,
    uparam_3: i32,
    param_4: i32,
) -> u16 {
    let u_var1: u32;
    let BVar2: bool;

    BVar2 = pass1_fn_1000_1dfa(0, param_1, param_3, param_4);
    if (BVar2 == 0) {
        if ((param_1 & 4) == 0) {
            u_var1 = SegmentLimit(param_4);
            if ((u_var1 >> 0x10) & 1) {
                if (param_2 == 0) {
                    return 1;
                }
                if ((!CARRY4(param_3, param_2 - 1)) && (param_3 + (param_2 - 1) <= u_var1)) {
                    return 1;
                }
            }
        } else {
            BVar2 = pass1_fn_1000_22c0(param_3, param_4, param_2, param_1, param_2._2_2_);
            if (BVar2 != 0) {
                return 1;
            }
        }
    }
    return 0;
}

pub unsafe fn pass1_fn_1000_2242(
    param_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: i32,
    param_5: u16,
    param_6: &mut Vec<u8>,
) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut b_var3: bool;
    let mut local_c: u16;
    let mut local_a: u16;

    u_var1 = param_2 | param_1;
    while (true) {
        if (u_var1 == 0) {
            return 0;
        }
        u_var1 = param_1;
        if (param_2 != 0) {
            u_var1 = 0xffff;
        }
        if (CARRY2(param_3, u_var1) != false) {
            u_var1 = -param_3;
        }
        b_var3 = param_1 < u_var1;
        param_1 = param_1 - u_var1;
        param_2 = param_2 - b_var3;
        u_var2 = (u_var1, param_5, param_3, param_4);
        if (u_var2 != 0) {
            break;
        }
        b_var3 = CARRY2(param_3, u_var1);
        param_3 = param_3 + u_var1;
        param_4 = param_4 + b_var3 * 0x100;
        u_var1 = param_2 | param_1;
    }
    return CONCAT22(param_2 + CARRY2(u_var2, param_1), u_var2 + param_1);
}

pub unsafe fn pass1_fn_1000_22c0(
    param_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: u16,
    uparam_5: i32,
) -> bool {
    let mut u_var1: u32;

    u_var1 = pass1_fn_1000_2242(
        param_3,
        param_5,
        param_1,
        param_2,
        param_4,
        (s_1037a_bmp_1050_1df1 + 9),
    );
    if (u_var1 == 0) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_fn_1000_23c1(param_1: i32) {
    process_win_msg_1008_54aa(
        &PTR_LOOP_1050_5f52,
        &PTR_LOOP_1050_5f4e,
        &PTR_LOOP_1050_5f50,
        &PTR_LOOP_1050_5f4a,
        &g_h_instance,
    );
    return;
}

// WARNING: Removing unreachable block (ram,0x10002557)

pub unsafe fn pass1_fn_1000_24ee(param_1: u16, param_2: i32) {
    let pc_var1: *mut code;
    let mut c_var2: u8;

    *&PTR_LOOP_1050_5fc9 = 1;
    c_var2 = 0x1;
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    empty_fn_1000_55ac(param_1);
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if (c_var2 == '\0') {
        unsafe {
            pc_var1 = swi(0x21);
            (*pc_var1)();
        }
    }
    return;
}

pub unsafe fn pass1_fn_1000_25a8(ctx: &mut AppContext) {
    pass1_fn_1000_2913(ctx, 0xfc);
    pass1_fn_1000_2913(ctx, 0xff);
    return;
}

pub unsafe fn pass1_fn_1000_25ac(ctx: &mut AppContext) {
    pass1_fn_1000_2913(ctx, 0xfc);
    pass1_fn_1000_2913(ctx, 0xff);
    return;
}

pub unsafe fn pass1_fn_1000_2913(ctx: &mut AppContext, param_1: u8) {
    let pc_var1: String;
    let pc_var2: String;
    let mut i_var3: i32;
    let mut unaff_es: u16;

    if (ctx.PTR_LOOP_1050_61ec != 0x0) {
        pc_var2 = process_string_1000_28dc(ctx, param_1);
        if (pc_var2 != 0x0) {
            i_var3 = -1;
            while {
                if (i_var3 == 0) {
                    break;
                }
                i_var3 = i_var3 + -1;
                pc_var1 = pc_var2;
                pc_var2 = pc_var2 + 1;
                let pc_var1_val = *pc_var1;
                pv_var1_val != '\0'
            } {}
            process_string_1000_55b1();
        }
    }
    return;
}

pub unsafe fn pass1_fn_1000_29af() {
    let mut in_ax: i32;

    pass1_fn_1000_29b5(in_ax & 0xff);
    return;
}

pub unsafe fn pass1_fn_1000_29b5(param_1: u16) {
    let char1: u8;

    PTR_LOOP_1050_5f88._0_1_ = param_1;
    char1 = (param_1 >> 8);
    if (char1 != '\0') {}
    // goto LAB_1000_29d2;
    if (PTR_LOOP_1050_5f88 < 0x22) {
        if (PTR_LOOP_1050_5f88 < 0x20) {
            if (0x13 < PTR_LOOP_1050_5f88) {}
            // goto LAB_1000_29cc;
        } else {
            param_1 = 5;
        }
    } else {
        // LAB_1000_29cc:
        param_1 = 0x13;
    }
    unsafe { char1 = *((param_1 & 0xff) + 0x5fd6) };
    // LAB_1000_29d2:
    PTR_LOOP_1050_5f78 = char1;
    return;
}

pub unsafe fn pass1_fn_1000_2b02(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> u16 {
    let paVar1: *mut Struct153;
    let pi_var2: *mut i32;
    let mut local_6: u16;

    paVar1 = pass1_fn_1000_35aa();
    if ((param_6 | paVar1) == 0) {
        pi_var2 = 0x0;
    } else {
        pi_var2 = pass1_fn_1000_2d34(
            param_1,
            param_2,
            CONCAT22(param_4, param_3),
            param_5,
            paVar1,
        );
    }
    return pi_var2;
}

pub unsafe fn pass1_fn_1000_2b3c(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) {
    pass1_fn_1000_2b02(param_1, param_2, param_3, param_4, 0, param_5);
    return;
}

pub unsafe fn pass1_fn_1000_2b5c(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u16 {
    let mut local_AX_15: u16;
    let mut u_var1: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_AX_15 = process_struct_1000_2e74(param_1);
    u_var1 = pass1_fn_1000_30b4(
        param_1,
        &ctx.g_alloc_addr_1050_1050,
        CONCAT22(param_4, param_2_00),
    );
    process_struct_1000_2f00(local_AX_15, param_1);
    return u_var1;
}

pub unsafe fn pass1_fn_1000_2d34(
    param_1: i32,
    param_2: i32,
    param_3: *mut byte,
    param_4: i32,
    param_5: *mut Struct153,
) -> *mut i32 {
    let mut b_var1: u8;
    let mut b_var2: bool;
    let mut b_var3: bool;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut local_e: u8;
    let mut local_c: u16;
    let mut local_a: u16;
    let local_8: u8;
    let mut local_6: u8;

    local_8 = PTR_LOOP_1050_6062;
    b_var3 = false;
    unsafe { unsafe { b_var1 = *param_3 } };
    if (b_var1 == 0x77) {
        u_var5 = 0x301;
    } else {
        if (0x77 < b_var1) {
            return 0x0;
        }
        if (b_var1 != 0x61) {
            if (b_var1 != 0x72) {
                return 0x0;
            }
            u_var5 = 0;
            local_6 = 1;
            // goto LAB_1000_2d6c;
        }
        u_var5 = 0x109;
    }
    local_6 = 2;
    // LAB_1000_2d6c:
    b_var2 = true;
    // LAB_1000_2d71:
    param_3 = (param_3 & 0xffff0000 | (param_3 + 1));
    let param_3_val = unsafe { *param_3 };
    if ((param_3_val == 0) || (!b_var2)) {
        i_var4 = dos3_call_1000_370a(param_1, param_2, u_var5, param_4, 0x1a4);
        if (i_var4 < 0) {
            return 0x0;
        }
        PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 1;
        *&param_5.field_0xa = local_6;
        param_5.field_0x2 = 0;
        (param_5).field_0x0 = 0;
        param_5.field_0x8 = 0;
        param_5.field_0x6 = 0;
        local_e = i_var4;
        *&param_5.field_0xb = local_e;
        param_5.field_0xf0 = local_8;
        param_5.field_0x4 = 0;
        param_5.field_0xf4 = 0;
        return param_5;
    }
    unsafe { unsafe { b_var1 = *param_3 } };
    if (b_var1 == 0x74) {
        if ((u_var5 & 0xc000) == 0) {
            u_var5 = u_var5 | 0x4000;
            // goto LAB_1000_2d71;
        }
    } else {
        if (0x74 < b_var1) {}
        // goto LAB_1000_2da4;
        if (b_var1 == 0x2b) {
            if ((u_var5 & 2) != 0) {}
            // goto LAB_1000_2da4;
            u_var5 = u_var5 & 0xfffe | 2;
            local_6 = 0x80;
            // goto LAB_1000_2d71;
        }
        if (b_var1 == 0x62) {
            if ((u_var5 & 0xc000) == 0) {
                u_var5 = u_var5 | 0x8000;
                // goto LAB_1000_2d71;
            }
        } else {
            if (b_var1 != 99) {
                if ((b_var1 != 0x6e) || (b_var3)) {}
                // goto LAB_1000_2da4;
                b_var3 = true;
                local_8 = local_8 & 0xbf;
                // goto LAB_1000_2d71;
            }
            if (!b_var3) {
                b_var3 = true;
                local_8 = local_8 | 0x40;
                // goto LAB_1000_2d71;
            }
        }
    }
    // LAB_1000_2da4:
    b_var2 = false;
    // goto LAB_1000_2d71;
}

pub unsafe fn pass1_fn_1000_2f48(param_1: *mut Struct156, param_2: u16) -> i32 {
    let mut i_var1: i32;

    if ((param_2 | param_1) == 0) {
        i_var1 = pass1_fn_1000_3038(0);
    } else {
        i_var1 = pass1_fn_1000_2fa4(param_1);
        if (i_var1 == 0) {
            if ((param_1.field_0xf0 & 0x40) != 0) {
                i_var1 = pass1_fn_1000_400a(param_1.field_0xb);
                i_var1 = -(i_var1 != 0);
            }
        } else {
            i_var1 = -1;
        }
    }
    return i_var1;
}

pub unsafe fn pass1_fn_1000_2fa4(param_1: *mut Struct157) -> u16 {
    let pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut local_DI_8: u16;
    let mut local_4: u16;

    local_DI_8 = 0;
    b_var2 = param_1.field_0xa;
    if (((b_var2 & 3) == 2) && ((b_var2 & 8) != 0 || ((param_1.field_0xf0 & 1) != 0))) {
        i_var3 = (param_1).field_0x0 - param_1.field_0x6;
        if (0 < i_var3) {
            i_var4 = dos3_call_1000_39f2(
                param_1.field_0xb,
                param_1.field_0x6,
                param_1.field_0x8,
                i_var3,
            );
            if (i_var4 == i_var3) {
                if ((param_1.field_0xa & 0x80) != 0) {
                    pb_var1 = &param_1.field_0xa;
                    unsafe { *pb_var1 = *pb_var1 & 0xfd };
                }
            } else {
                pb_var1 = &param_1.field_0xa;
                unsafe { *pb_var1 = *pb_var1 | 0x20 };
                local_DI_8 = 0xffff;
            }
        }
    }
    i_var3 = param_1.field_0x8;
    (param_1).field_0x0 = param_1.field_0x6;
    param_1.field_0x2 = i_var3;
    param_1.field_0x4 = 0;
    return local_DI_8;
}

pub unsafe fn pass1_fn_1000_3024() {
    let mut unaff_bp: i32;

    pass1_fn_1000_3038(1, &ctx.g_alloc_addr_1050_1050, unaff_bp + 1);
    return;
}

pub unsafe fn pass1_fn_1000_3038(param_1: i32, param_2: u32, param_3: u32) -> u16 {
    let mut i_var1: i32;
    let paVar2: *mut Struct156;
    let mut u_var3: u16;
    let mut local_4: u16;

    paVar2 = &PTR_LOOP_1050_6210;
    u_var3 = 0;
    local_4 = 0;
    while (paVar2 <= PTR_LOOP_1050_5ff0) {
        if ((param_1 == 1) && ((paVar2.field_0xa & 0x83) != 0)) {
            i_var1 = pass1_fn_1000_2f48(paVar2, &ctx.g_alloc_addr_1050_1050);
            if (i_var1 != -1) {
                u_var3 = u_var3 + 1;
            }
        } else {
            if ((param_1 == 0)
                && ((paVar2.field_0xa & 2) != 0
                    && (
                        i_var1 = pass1_fn_1000_2f48(paVar2, &ctx.g_alloc_addr_1050_1050),
                        i_var1 == -1,
                    )))
            {
                local_4 = 0xffff;
            }
        }
        paVar2 = &paVar2.field_0xc;
    }
    if (param_1 == 1) {
        local_4 = u_var3;
    }
    return local_4;
}

// WARNING: Variable defined which should be unmapped: local_c
// WARNING: Variable defined which should be unmapped: local_6
// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_fn_1000_30b4(param_1: u16, param_2: u16, param_1_00: *mut byte) -> u16 {
    let mut b_var1: u8;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut unaff_si: i32;
    let mut local_c: u16;
    let local_9: u8;
    let local_6: u8;

    bad_1000_25f2(&ctx.g_alloc_addr_1050_1050, unaff_bp + 1);
    unsafe { b_var1 = *param_1_00 };
    _local_6 = unaff_si & 0xff00 | b_var1;
    if (b_var1 == 0) {
        return 0;
    }
    if ((b_var1 - 0x20) < 0x59) {
        unsafe { b_var1 = *((b_var1 - 0x20) + 0x5ffe) & 0xf };
    } else {
        b_var1 = 0;
    }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
    u_var2 = (**((*((b_var1 * 0x8) + 0x5ffe) >> 4) * 2 + 0x30a4))(_local_6);
    return u_var2;
}

pub unsafe fn pass1_fn_1000_34cf() -> u16 {
    let mut u_var1: u16;
    let p_uvar2: *mut u16;
    let local_BP__1: *mut Struct158;
    let mut unaff_ss: u16;

    pu_var2 = &local_BP__1.field_0xe;
    unsafe { u_var1 = *pu_var2 };
    local_BP__1.field_0xe = pu_var2 + 2;
    return u_var1;
}

pub unsafe fn pass1_fn_1000_34d8() -> u32 {
    let u_var1: u32;
    let pu_var2: *mut u3232_t;
    let local_BP__1: *mut Struct159;
    let mut unaff_ss: u16;

    unsafe { pu_var2 = &local_BP__1.field_0xe };
    unsafe { u_var1 = *pu_var2 };
    local_BP__1.field_0xe = pu_var2 + 4;
    return u_var1;
}

pub unsafe fn pass1_fn_1000_34e6() {
    let mut i_var1: i32;
    let local_BP__1: *mut Struct160;
    let mut unaff_ss: u16;

    if ((*(local_BP__1 + -6) & 0x20) != 0) {
        pass1_fn_1000_34d8();
        return;
    }
    i_var1 = pass1_fn_1000_34cf();
    if (i_var1 == 0) {
        return;
    }
    return;
}

pub unsafe fn pass1_fn_1000_3503(param_1: u32) -> u16 {
    let pi_var1: *mut i32;
    let pu_var2: *mut u328_t;
    **ppu_var3;
    let mut i_var4: i32;
    let local_bx_3: *mut Struct161;
    let local_BP__1: *mut Struct162;
    let mut u_var5: u16;
    let mut unaff_ss: u16;

    ppu_var3 = local_BP__1.field_0x6;
    u_var5 = (ppu_var3 >> 0x10);
    local_bx_3 = ppu_var3;
    pi_var1 = &local_bx_3.field_0x4;
    unsafe { *pi_var1 = *pi_var1 + -1 };
    let pi_var1_val = unsafe { *pi_var1 };
    if (pi_var1_val < 0) {
        i_var4 = dos3_call_1000_2bb6(param_1, local_bx_3, u_var5);
        if (i_var4 == -1) {
            return 0xffff;
        }
    } else {
        unsafe { pu_var2 = *ppu_var3 };
        unsafe { *ppu_var3 = *ppu_var3 + 1 };
        unsafe { *pu_var2 = param_1 };
    }
    return 0;
}

pub unsafe fn pass1_fn_1000_3534(param1: u16) {
    let pi_var1: *mut i32;
    let pu_var2: *mut u328_t;
    let mut u_var3: u16;
    let local_BP__1: *mut Struct163;
    let unaff_DI: *mut u328_t;
    let mut u_var4: i32;
    let mut unaff_es: u16;
    let mut unaff_ss: u16;

    if (param1 != 0) {
        pi_var1 = (local_BP__1 + -10);
        unsafe { *pi_var1 = *pi_var1 + param1 };
        u_var4 = 0;
        while {
            pu_var2 = unaff_DI;
            unaff_DI = unaff_DI + 1;
            let pu_var2_val = unsafe { *pu_var2 };
            u_var3 = pass1_fn_1000_3503(pu_var2_val);
            u_var4 = u_var4 | u_var3;
            param1 = param1 - 1;
            param1 != 0
        } {}
        if (u_var4 != 0) {
            (local_BP__1 + -10) = 0xffff;
        }
    }
    return;
}

pub unsafe fn pass1_fn_1000_3552(param_1: u16, param_2: u32) {
    let pi_var1: *mut i32;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u_var3: i32;
    let mut unaff_ss: u16;

    if (param_1 != 0) {
        pi_var1 = (unaff_bp + -10);
        unsafe { *pi_var1 = *pi_var1 + param_1 };
        u_var3 = 0;
        while {
            u_var2 = pass1_fn_1000_3503(param_2);
            u_var3 = u_var3 | u_var2;
            param_1 = param_1 - 1;
            param_1 != 0
        } {}
        if (u_var3 != 0) {
            (unaff_bp + -10) = 0xffff;
        }
    }
    return;
}

pub unsafe fn pass1_fn_1000_356e(param_1: u16, param_2: u16, param_3: u16) {
    let pb_var1: *mut u8;
    let mut u_var2: u32;
    let mut b_var3: u8;
    let mut unaff_bp: i32;
    let mut unaff_si: i32;
    let unaff_DI: *mut u8;
    let mut unaff_es: u16;
    let mut unaff_ss: u16;

    while ((0 < unaff_si || (param_1 != 0)) || (param_3 != 0)) {
        u_var2 = param_3;
        param_3 = param_3 / param_2;
        u_var2 = u_var2 % param_2 << 0x10 | param_1;
        param_1 = (u_var2 / param_2);
        b_var3 = (u_var2 % param_2) + 0x30;
        if (0x39 < b_var3) {
            b_var3 = b_var3 + *(unaff_bp + -3);
        }
        pb_var1 = unaff_DI;
        unaff_DI = unaff_DI + -1;
        unsafe { *pb_var1 = b_var3 };
        unaff_si = unaff_si + -1;
    }
    return;
}

pub unsafe fn pass1_fn_1000_35aa() -> *mut Struct164 {
    let local_SI_9: *mut Struct164;
    let mut local_8: u16;
    let mut local_6: u16;

    local_SI_9 = &PTR_LOOP_1050_6210;
    while (true) {
        if (PTR_LOOP_1050_5ff0 < local_SI_9) {
            return 0x0;
        }
        if ((*&local_SI_9.field_0xa & 0x83) == 0) {
            break;
        }
        local_SI_9 = &local_SI_9.field_0xc;
    }
    *&local_SI_9.field_0xa = 0;
    local_SI_9.field_0x4 = 0;
    local_SI_9.field_0x8 = 0;
    local_SI_9.field_0x6 = 0;
    local_SI_9.field_0x2 = 0;
    (local_SI_9).field_0x0 = 0;
    *&local_SI_9.field_0xb = 0xff;
    return local_SI_9;
}

pub unsafe fn pass1_fn_1000_3bac() -> i32 {
    let mut i_var1: i32;

    if (PTR_LOOP_1050_5f48 < &stack0x0004) {
        i_var1 = -(PTR_LOOP_1050_5f48 + -&stack0x0004);
    } else {
        i_var1 = 0;
    }
    return i_var1;
}

pub unsafe fn pass1_fn_1000_3cb7(param_1: *mut Struct165) {
    let mut u_var1: i32;
    let pu_var2: *mut u32;

    pu_var2 = param_1.field_0xa;
    if (pu_var2 == param_1.field_0xc) {
        pu_var2 = param_1.field_0x8;
    }
    while (true) {
        unsafe { u_var1 = *pu_var2 };
        if (u_var1 == 0xfffe) {
            break;
        }
        pu_var2 = (pu_var2 + (u_var1 & 0xfffe) + 2);
    }
    return;
}

pub unsafe fn pass1_1000_3d7a(param_1: u32, param_2: u32) -> i32 {
    let pb_var1: *mut u8;
    let paVar2: *mut Struct168;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pa_var5: *mut Struct168;
    let pb_var6: *mut u8;
    let local_DI_11: *mut Struct168;
    let pa_var7: *mut Struct168;
    let mut u_var8: u16;
    let mut b_var9: bool;
    let mut b_var10: bool;

    pb_var6 = param_1;
    u_var8 = (param_2 >> 0x10);
    local_DI_11 = param_2;
    i_var3 = 0;
    u_var4 = 0xffff;
    while {
        if (u_var4 == 0) {
            break;
        }
        u_var4 = u_var4 - 1;
        paVar2 = local_DI_11;
        local_DI_11 = &local_DI_11.field_0x1;
        paVar2.field_0x0 != '\0'
    } {}
    // pa_var5 = ~u_var4;
    b_var9 = local_DI_11 < pa_var5;
    pa_var7 = (local_DI_11 - pa_var5);
    b_var10 = pa_var7 == 0x0;
    while {
        if (pa_var5 == 0x0) {
            break;
        }
        pa_var5 = &pa_var5[-1].field_0x1;
        paVar2 = pa_var7;
        pa_var7 = &pa_var7.field_0x1;
        pb_var1 = pb_var6;
        pb_var6 = pb_var6 + 1;
        unsafe { b_var9 = *pb_var1 < paVar2.field_0x0 };
        unsafe { b_var10 = *pb_var1 == paVar2.field_0x0 };
        b_var10
    } {}
    if (!b_var10) {
        i_var3 = (1 - b_var9) - (b_var9 != 0);
    }
    return i_var3;
}

pub unsafe fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
    let pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut b_var3: u8;
    let mut i_var4: i32;
    let pb_var5: *mut u8;
    let mut u_var6: u16;

    u_var6 = (long_byte_ptr >> 0x10);
    pb_var5 = long_byte_ptr;
    i_var4 = 0;
    while {
        while {
            pb_var1 = pb_var5;
            pb_var5 = pb_var5 + 1;
            unsafe { b_var2 = *pb_var1 };
            b_var2 == 0x20
        } {}
        b_var2 == 9
    } {}
    if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {}
    // goto LAB_1000_3e4d;
    while (true) {
        pb_var1 = pb_var5;
        pb_var5 = pb_var5 + 1;
        unsafe { b_var3 = *pb_var1 };
        // LAB_1000_3e4d:
        if ((0x39 < b_var3) || (b_var3 < 0x30)) {
            break;
        }
        i_var4 = i_var4 * 10 + (b_var3 - 0x30);
    }
    if (b_var2 == 0x2d) {
        i_var4 = -i_var4;
    }
    return i_var4;
}

// pub unsafe fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
//     let pb_var1: *mut u8;
//     let mut b_var2: u8;
//     let mut b_var3: u8;
//     let mut i_var4: i32;
//     let pb_var5: *mut u8;
//     let mut u_var6: u16;

//     u_var6 = (long_byte_ptr >> 0x10);
//     pb_var5 = long_byte_ptr;
//     i_var4 = 0;
//     while {
//         while {
//             pb_var1 = pb_var5;
//             pb_var5 = pb_var5 + 1;
//             unsafe { b_var2 = *pb_var1 };
//             b_var2 == 0x20
//         } {}
//         b_var2 == 9
//     } {}
//     if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {}
//     // goto LAB_1000_3e4d;
//     loop {
//         pb_var1 = pb_var5;
//         pb_var5 = pb_var5 + 1;
//         unsafe { b_var3 = *pb_var1 };
//         // LAB_1000_3e4d:
//         if ((0x39 < b_var3) || (b_var3 < 0x30)) {
//             break;
//         }
//         i_var4 = i_var4 * 10 + (b_var3 - 0x30);
//     }
//     if (b_var2 == 0x2d) {
//         i_var4 = -i_var4;
//     }
//     return i_var4;
// }

// pub unsafe fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
//     let pb_var1: *mut u8;
//     let mut b_var2: u8;
//     let mut b_var3: u8;
//     let mut i_var4: i32;
//     let pb_var5: *mut u8;
//     let mut u_var6: u16;

//     u_var6 = (long_byte_ptr >> 0x10);
//     pb_var5 = long_byte_ptr;
//     i_var4 = 0;
//     while {
//         while {
//             pb_var1 = pb_var5;
//             pb_var5 = pb_var5 + 1;
//             unsafe { b_var2 = *pb_var1 };
//             b_var2 == 0x20
//         } {}
//         b_var2 == 9
//     } {}
//     if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {}
//     // goto LAB_1000_3e4d;
//     loop {
//         pb_var1 = pb_var5;
//         pb_var5 = pb_var5 + 1;
//         unsafe { b_var3 = *pb_var1 };
//         // LAB_1000_3e4d:
//         if ((0x39 < b_var3) || (b_var3 < 0x30)) {
//             break;
//         }
//         i_var4 = i_var4 * 10 + (b_var3 - 0x30);
//     }
//     if (b_var2 == 0x2d) {
//         i_var4 = -i_var4;
//     }
//     return i_var4;
// }

pub unsafe fn pass1_fn_1000_3e82(param_1: i32, param_2: *mut u8, uparam_3: i32) -> *mut Struct169 {
    let paVar1: *mut Struct169;
    let mut u_var2: u32;
    let mut b_var3: u8;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let local_SI_2839: *mut Struct169;
    let pa_var8: *mut Struct169;
    let paVar9: *mut Struct169;
    let pa_var10: *mut Struct169;
    let mut u_var11: u16;
    let mut b_var12: bool;
    let mut cVar4: u8;

    u_var6 = 0;
    if (param_3 == 10) {
        u_var6 = param_1 >> 0xf;
    }
    u_var11 = (param_2 >> 0x10);
    pa_var8 = param_2;
    paVar9 = pa_var8;
    local_SI_2839 = pa_var8;
    if ((param_3 == 10) && (u_var6 < 0)) {
        paVar9 = &pa_var8.field_0x1;
        unsafe { *param_2 = 0x2d };
        b_var12 = param_1 != 0;
        param_1 = -param_1;
        u_var6 = -(u_var6 + b_var12);
        local_SI_2839 = paVar9;
    }
    while {
        u_var7 = 0;
        u_var5 = u_var6;
        if (u_var6 != 0) {
            u_var5 = u_var6 / param_3;
            u_var7 = u_var6 % param_3;
        }
        u_var2 = CONCAT22(u_var7, param_1);
        param_1 = (u_var2 / param_3);
        cVar4 = (u_var2 % param_3);
        b_var3 = cVar4 + 0x30;
        if (0x39 < b_var3) {
            b_var3 = cVar4 + 0x57;
        }
        pa_var10 = &paVar9.field_0x1;
        (paVar9).field_0x0 = b_var3;
        u_var6 = u_var5;
        paVar9 = pa_var10;
        (u_var5 | param_1) != 0
    } {}
    (pa_var10).field_0x0 = 0;
    while {
        pa_var10 = &pa_var10[-1].field_0x3;
        paVar1 = pa_var10;
        b_var3 = paVar1.field_0x0;
        paVar1.field_0x0 = (local_SI_2839).field_0x0;
        (local_SI_2839).field_0x0 = b_var3;
        paVar9 = &local_SI_2839.field_0x2;
        local_SI_2839 = &local_SI_2839.field_0x1;
        paVar9 < pa_var10
    } {}
    return pa_var8;
}

pub unsafe fn pass1_fn_1000_3f5c() -> i32 {
    let mut i_var1: i32;
    let paVar2: *mut Struct150;
    let mut i_var3: i32;

    i_var3 = 0;
    if (PTR_LOOP_1050_61ec == 0x0) {
        paVar2 = &PTR_LOOP_1050_6210;
    } else {
        paVar2 = 0x6234;
    }
    while (paVar2 <= PTR_LOOP_1050_5ff0) {
        i_var1 = process_string_1000_2a00(paVar2);
        if (i_var1 != -1) {
            i_var3 = i_var3 + 1;
        }
        paVar2 = &paVar2.field_0xc;
    }
    return i_var3;
}

pub unsafe fn pass1_fn_1000_400a(param_1: i32) -> u16 {
    let paVar1: *mut Struct149;
    let mut Struct149_1: u16;

    if ((param_1 < 0) || (PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
        PTR_LOOP_1050_5f78 = &BYTE_1050_0009;
        Struct149_1 = 0xffff;
    } else {
        if (((PTR_LOOP_1050_61ec == 0x0) || (param_1 < u16_1050_5f8a && (2 < param_1)))
            && (0x31d < CONCAT11(u8_1050_5f83, u8_1050_5f82)))
        {
            paVar1 = PTR_LOOP_1050_5f88;
            if (((*(param_1 + 0x5f90) & 1) == 0)
                || (
                    Struct149_1 = dos3_call_1000_5174(param_1),
                    paVar1 = Struct149_1,
                    Struct149_1 != 0x0,
                ))
            {
                PTR_LOOP_1050_5f88 = paVar1;
                PTR_LOOP_1050_5f78 = &BYTE_1050_0009;
                Struct149_1 = 0xffff;
            }
        } else {
            Struct149_1 = 0;
        }
    }
    return Struct149_1;
}

pub unsafe fn pass1_fn_1000_41e0(param_1: i32) -> u16 {
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = CONCAT22(PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    while (true) {
        if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= local_6) {
            return 0;
        }
        if (*_local_6 == param_1) {
            break;
        }
        _local_6 = (_local_6 & 0xffff0000 | (local_6 + 4));
    }
    unsafe { *_local_6 = 0 };
    return (local_6 + 2);
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_fn_1000_422a(param_1: i32, param_2: u16) -> i32 {
    let pu_var1: *mut u8;
    let pu_var2: *mut u8;
    let pu_var3: *mut u8;
    let pu_var4: *mut u8;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = CONCAT22(PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    while (true) {
        if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= local_6) {
            pu_var2 = PTR_LOOP_1050_6194 + 0x28;
            pu_var4 = PTR_LOOP_1050_6192;
            pu_var3 = alloc_mem_1000_16aa();
            if ((pu_var4 | pu_var3) == 0) {
                param_1 = 0;
            } else {
                pu_var1 = pu_var3 + (PTR_LOOP_1050_6194 & 0xfffc);
                _local_6 = CONCAT22(pu_var4, pu_var1);
                PTR_LOOP_1050_6190 = pu_var3;
                PTR_LOOP_1050_6192 = pu_var4;
                unsafe { *_local_6 = param_1 };
                (pu_var1 + 2) = param_2;
                PTR_LOOP_1050_6194 = pu_var2;
                pass1_1000_4906(CONCAT22(pu_var4, pu_var1 + 4), 0, 0x24);
            }
            return param_1;
        }
        if (*_local_6 == 0) {
            break;
        }
        _local_6 = (_local_6 & 0xffff0000 | (local_6 + 4));
    }
    (local_6 + 2) = param_2;
    unsafe { *_local_6 = param_1 };
    return param_1;
}

pub unsafe fn pass1_fn_1000_43f0() {
    let mut in_dx: u16;

    if (PTR_LOOP_1050_68b4 == 0x0) {
        process_string_1000_440c(in_dx);
        PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 1;
    }
    return;
}

pub unsafe fn pass1_fn_1000_455a(param_1: *mut Struct170) -> u16 {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let local_bx_9: *mut Struct170;
    let mut u_var6: u16;
    let mut local_6: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_9 = param_1;
    if (((local_bx_9.field_0xa < 0x43) || (local_bx_9.field_0x8 < 3)) || (9 < local_bx_9.field_0x8))
    {
    }
    // goto LAB_1000_4623;
    if ((local_bx_9.field_0x8 < 4) || (8 < local_bx_9.field_0x8)) {
        u_var3 = local_bx_9.field_0xa;
        if ((u_var3 < 0x57) || (local_bx_9.field_0x8 != 3)) {
            local_6 = (local_bx_9.field_0x8 * 2 + 0x61b2);
        } else {
            local_6 = (local_bx_9.field_0x8 * 2 + 0x61b0) + 7;
        }
        if ((u_var3 & 3) == 0) {
            local_6 = local_6 + 1;
        }
        u_var3 = (u_var3 - 0x46) * 0x16d + ((u_var3 - 1) >> 2) + local_6;
        i_var4 = pass1_fn_1000_52f0(u_var3 - 0xd, (u_var3 >> 0xf) - (u_var3 < 0xd), 7, 0);
        i_var4 = i_var4 - local_6;
        i_var5 = -i_var4;
        if (local_bx_9.field_0x8 == 3) {
            i_var2 = local_bx_9.field_0xe;
            if ((i_var5 < i_var2) || (-i_var2 == i_var4 && (1 < local_bx_9.field_0x4))) {}
            // goto LAB_1000_460e;
        } else {
            pi_var1 = &local_bx_9.field_0xe;
            unsafe { i_var2 = *pi_var1 };
            if ((SBORROW2(i_var2, i_var5) != i_var2 + (i_var4 < 0))
                || (i_var2 == i_var5 && (local_bx_9.field_0x4 < 1)))
            {}
            // goto LAB_1000_460e;
        }
        // LAB_1000_4623:
        u_var6 = 0;
    } else {
        // LAB_1000_460e:
        u_var6 = 1;
    }
    return u_var6;
}

pub unsafe fn pass1_fn_1000_462e(
    param_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: i32,
    uparam_5: i32,
    param_6: i32,
) {
    u_var1;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let extraout_AH: u8;
    let extraout_AH_00: u8;
    let extraout_AH_01: u8;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut unaff_bp: i32;
    let mut u_var9: i32;
    let mut unaff_ss: u16;
    let mut in_u16_3: u16;
    let mut in_u16_4: u16;
    let mut in_u16_3_00: u16;
    let mut in_u16_4_00: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_bp + 1;
    local_4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var9 = (param_2 * 2 + 0x61ae);
    if (((param_1 & 3) == 0) && (2 < param_2)) {
        u_var9 = u_var9 + 1;
    }
    pass1_fn_1000_43f0();
    in_u16_4_00 = 0;
    in_u16_3_00 = 0x3c;
    in_u16_4 = 0;
    in_u16_3 = 0x3c;
    u_var2 = (param_1 * 0x16d);
    u_var3 = (param_1 + 3) >> 2;
    u_var4 = u_var3 + param_3;
    u_var5 = u_var2 + u_var4;
    i_var7 = u_var9 >> 0xf;
    u_var6 = u_var5 + u_var9;
    u_var1 = pass1_fn_1000_52be(
        u_var6 + 0xe44,
        ((param_1 * 0x16d) >> 0x10)
            + ((param_1 + 3) >> 0xf)
            + (param_3 >> 0xf)
            + CARRY2(u_var3, param_3)
            + CARRY2(u_var2, u_var4)
            + i_var7
            + CARRY2(u_var5, u_var9)
            + (0xf1bb < u_var6),
        0x18,
        0,
    );
    i_var8 = param_4 >> 0xf;
    u_var1 = pass1_fn_1000_52be(
        CONCAT11(extraout_AH, u_var1) + param_4,
        i_var7 + i_var8 + CARRY2(CONCAT11(extraout_AH, u_var1), param_4),
        in_u16_3,
        in_u16_4,
    );
    u_var1 = pass1_fn_1000_52be(
        CONCAT11(extraout_AH_00, u_var1) + param_5,
        i_var8 + (param_5 >> 0xf) + CARRY2(CONCAT11(extraout_AH_00, u_var1), param_5),
        in_u16_3_00,
        in_u16_4_00,
    );
    local_1a = CONCAT11(extraout_AH_01, u_var1) + param_6 + u16_1050_61ce;
    local_8 = param_3 + u_var9;
    local_c = param_1 + 0x50;
    local_e = param_2 - 1;
    local_12 = param_4;
    if (u16_1050_61d2 != 0) {
        i_var7 = pass1_fn_1000_455a(CONCAT22(unaff_ss, local_16));
        if (i_var7 != 0) {
            local_1a = local_1a - 0xe10;
        }
    }
    return local_1a;
}

pub unsafe fn pass1_fn_1000_47a4(param_1: u32, param_2: *mut byte) -> uint {
    let pb_var1: *mut u8;
    let pu8_var2: *mut u8;
    let mut b_var3: u8;
    let mut byte_ptr_1: u16;
    let mut i_var4: i32;
    let mut byte_ptr_3: u16;
    let pu_var5: *mut u16;
    let mut unaff_ss: u16;
    let mut u_var6: u16;
    let mut u16_array_1: [u8; 32];
    let mut byte_ptr_2: u32;
    let byte_1: u8;
    let temp_87f577f7f21: *mut u16;

    i_var4 = 0x10;
    pu_var5 = u16_array_1;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        temp_87f577f7f21 = pu_var5;
        pu_var5 = pu_var5 + 1;
        unsafe { *temp_87f577f7f21 = 0 };
    }
    byte_ptr_3 = param_2;
    while (true) {
        byte_ptr_2 = byte_ptr_3;
        byte_ptr_3 = byte_ptr_3 + 1;
        unsafe { byte_1 = *byte_ptr_2 };
        if (byte_1 == '\0') {
            break;
        }
        pb_var1 = u16_array_1 + (byte_1 >> 3);
        unsafe { *pb_var1 = *pb_var1 | 0x1 << (byte_1 & 7) };
    }
    pb_var1 = param_1;
    if (param_1 == 0) {
        pb_var1 = _UINT_1050_61e4;
    }
    while {
        _UINT_1050_61e4 = pb_var1;
        u_var6 = (_UINT_1050_61e4 >> 0x10);
        pu8_var2 = (_UINT_1050_61e4 + 1);
        unsafe { b_var3 = *_UINT_1050_61e4 };
        if (b_var3 == 0) {
            return 0;
        }
        pb_var1 = (_UINT_1050_61e4 & 0xffff0000 | ZEXT24(pu8_var2));
        (0x1 << (b_var3 & 7) & u16_array_1[b_var3 >> 3]) != 0
    } {}
    while {
        byte_ptr_1 = pu8_var2;
        unsafe { b_var3 = *byte_ptr_1 };
        if (b_var3 == 0) {}
        // goto LAB_1000_483c;
        pu8_var2 = (byte_ptr_1 + 1);
        (0x1 << (b_var3 & 7) & u16_array_1[b_var3 >> 3]) == 0
    } {}
    unsafe { *byte_ptr_1 = 0 };
    byte_ptr_1 = (byte_ptr_1 + 1);
    // LAB_1000_483c:
    _UINT_1050_61e4 = (_UINT_1050_61e4 & 0xffff0000 | byte_ptr_1);
    return UINT_1050_61e4;
}

pub unsafe fn pass1_fn_1000_484c(param_1: *mut byte, param_2: *mut byte, param_3: u16) -> uint {
    let pb_var1: *mut u8;
    let pu8_var2: *mut u8;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let pb_var6: *mut u8;
    let pb_var7: *mut u8;
    let local_es_16: *mut Struct172;
    let mut i_var8: i32;
    let local_DS_13: *mut Struct173;
    let mut b_var9: bool;
    let mut b_var10: bool;

    if (param_3 == 0) {
        return 0;
    }
    loop {
        i_var8 = (param_2 >> 0x10);
        pb_var7 = param_2;
        i_var3 = (param_1 >> 0x10);
        pb_var6 = param_1;
        // u_var4 = ~pb_var7;
        u_var4 = ((param_3 - 1) - u_var4 & -(param_3 - 1 < u_var4)) + u_var4;
        // u_var5 = ~pb_var6;
        u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 1;
        b_var9 = param_3 < u_var4;
        param_3 = param_3 - u_var4;
        b_var10 = param_3 == 0;
        while {
            if (u_var4 == 0) {
                break;
            }
            u_var4 = u_var4 - 1;
            pu8_var2 = pb_var7;
            pb_var7 = pb_var7 + 1;
            pb_var1 = pb_var6;
            pb_var6 = pb_var6 + 1;
            unsafe { b_var9 = *pb_var1 < *pu8_var2 };
            unsafe { b_var10 = *pb_var1 == *pu8_var2 };
            b_var10
        } {}
        param_2 = (param_2 & 0xffff0000 | ZEXT24(pb_var7));
        if (!b_var10) {
            return (1 - b_var9) - (b_var9 != 0);
        }
        if (param_3 == 0) {
            return u_var4;
        }
        if (pb_var6 == 0x0) {
            i_var3 = i_var3 + 0x6c;
        }
        param_1 = CONCAT22(i_var3, pb_var6);
        if (pb_var7 == 0x0) {
            param_2 = ((i_var8 + 0x6c) << 0x10);
            param_1 = CONCAT22(i_var3, pb_var6);
        }
    }
}

pub unsafe fn pass1_fn_1000_48a8(param_1: *mut Struct174, param_2: u32, param_3: u16) -> u16 {
    let pu_var1: *mut u16;
    let p_uvar2: *mut u16;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let pu_var6: *mut u16;
    let pu_var7: *mut u16;
    let local_es_14: *mut Struct174;
    let mut i_var8: i32;
    let local_DS_11: *mut Struct175;

    if (param_3 != 0) {
        while (true) {
            i_var3 = (param_2 >> 0x10);
            pu_var6 = param_2;
            i_var8 = (param_1 >> 0x10);
            pu_var7 = param_1;
            // u_var4 = ~pu_var7;
            u_var4 = ((param_3 - 1) - u_var4 & -(param_3 - 1 < u_var4)) + u_var4;
            // u_var5 = ~pu_var6;
            u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 1;
            param_3 = param_3 - u_var4;
            u_var5 = u_var4 >> 1;
            while (u_var5 != 0) {
                u_var5 = u_var5 - 1;
                pu_var2 = pu_var7;
                pu_var7 = pu_var7 + 1;
                pu_var1 = pu_var6;
                pu_var6 = pu_var6 + 1;
                unsafe { *pu_var2 = *pu_var1 };
            }
            u_var4 = ((u_var4 & 1) != 0);
            while (u_var4 != 0) {
                u_var4 = u_var4 - 1;
                pu_var2 = pu_var7;
                pu_var7 = (pu_var7 + 1);
                pu_var1 = pu_var6;
                pu_var6 = (pu_var6 + 1);
                unsafe { *pu_var2 = *pu_var1 };
            }
            if (param_3 == 0) {
                break;
            }
            if (pu_var6 == 0x0) {
                i_var3 = i_var3 + 0x6c;
            }
            param_1 = (param_1 & 0xffff0000 | ZEXT24(pu_var7));
            param_2 = CONCAT22(i_var3, pu_var6);
            if (pu_var7 == 0x0) {
                param_1 = ((i_var8 + 0x6c) << 0x10);
                param_2 = CONCAT22(i_var3, pu_var6);
            }
        }
    }
    return param_1._0_2_;
}

pub unsafe fn pass1_1000_4906(param_1: *mut Struct65, param_2: u16, param_3: u16) {
    let pu_var1: *mut u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: u16;
    let mut i_var7: i32;

    if (param_3 != 0) {
        i_var7 = (param_1 >> 0x10);
        u_var5 = -param_1;
        u_var6 = param_3;
        if (u_var5 != 0) {
            u_var6 = (u_var5 - param_3 & -(u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = param_2 & 0xff | param_2 << 8;
        u_var4 = u_var6 >> 1;
        while (u_var4 != 0) {
            u_var4 = u_var4 - 1;
            pu_var1 = param_1;
            param_1._0_2_ = param_1 + 1;
            unsafe { *pu_var1 = u_var3 };
        }
        u_var4 = ((u_var6 & 1) != 0);
        while (u_var2 = (param_2 & 0xff), u_var4 != 0) {
            u_var4 = u_var4 - 1;
            pu_var1 = param_1;
            param_1._0_2_ = (param_1 + 1);
            unsafe { *pu_var1 = u_var2 };
        }
        if (u_var5 != 0) {
            u_var4 = u_var5 >> 1;
            while (u_var4 != 0) {
                u_var4 = u_var4 - 1;
                pu_var1 = param_1;
                param_1._0_2_ = param_1 + 1;
                unsafe { *pu_var1 = u_var3 };
            }
            u_var5 = ((u_var5 & 1) != 0);
            while (u_var5 != 0) {
                u_var5 = u_var5 - 1;
                pu_var1 = param_1;
                param_1._0_2_ = (param_1 + 1);
                unsafe { *pu_var1 = u_var2 };
            }
        }
    }
    return param_1;
}

pub unsafe fn pass1_1000_49c6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    pfn_var16: u16,
) -> i32 {
    u_var1;
    let extraout_AH: u8;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let mut unaff_bp: i32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_bp + 1;
    local_4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    local_14 = param_3;
    local_12 = param_4;
    u_var4 = param_4;
    u_var1 = pass1_fn_1000_52be(param_5 - 1, -(param_5 == 0), param_6, 0);
    local_8 = CONCAT11(extraout_AH, u_var1) + param_3;
    local_6 = (u_var4 + CARRY2(CONCAT11(extraout_AH, u_var1), param_3)) * 0x100 + param_4;
    while (true) {
        if (local_6 < local_12) {
            return 0;
        }
        if ((local_6 <= local_12) && (local_8 < local_14)) {
            return 0;
        }
        local_e = param_5 >> 1;
        if (local_e == 0) {
            if ((param_5 != 0) && (i_var5 = (*pfn_var16)(), i_var5 == 0)) {
                return local_14;
            }
            return 0;
        }
        u_var2 = local_e;
        if ((param_5 & 1) == 0) {
            u_var2 = local_e - 1;
        }
        u_var3 = (u_var2 * param_6);
        u_var4 = u_var3 + local_14;
        local_a = ((u_var2 * param_6 >> 0x10) + CARRY2(u_var3, local_14)) * 0x100 + local_12;
        local_c = u_var4;
        i_var5 = (*pfn_var16)();
        if (i_var5 == 0) {
            break;
        }
        if (i_var5 < 0) {
            local_8 = -param_6 + local_c;
            local_6 = (CARRY2(-param_6, local_c) - (param_6 != 0)) * 0x100 + local_a;
            u_var2 = param_5 & 1;
            param_5 = local_e;
            if (u_var2 == 0) {
                param_5 = local_e - 1;
            }
        } else {
            local_14 = param_6 + local_c;
            local_12 = CARRY2(param_6, local_c) * 0x100 + local_a;
            param_5 = local_e;
        }
    }
    return u_var4;
}

pub unsafe fn pass1_1000_4aea(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    in_fn_ptr_1: *mut code,
) -> i32 {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let lVar3: u32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let pu_var8: *mut u8;
    let mut unaff_bp: i32;
    let mut u_var9: i32;
    let mut u_var10: i32;
    let mut u_var11: i32;
    let mut u_var12: i32;
    let mut unaff_ss: u16;
    let mut u_var13: u16;
    let mut b_var14: bool;
    let mut uStack44: u16;
    let mut uStack40: i32;
    let mut uStack38: i32;
    let mut uStack36: i32;
    let mut uStack34: i32;
    let mut uStack32: i32;
    let mut uStack30: i32;
    let mut uStack28: i32;
    let mut uStack26: i32;
    let mut uStack24: i32;
    let mut uStack22: i32;
    let mut uVar15: i32;
    let mut uVar16: i32;
    let mut uStack18: i32;
    let mut uStack16: i32;
    let mut uStack14: i32;
    let mut u_stack12: i32;
    let mut uStack4: u16;
    let mut iStack2: i32;
    let mut fn_ptr_1: u32;

    iStack2 = unaff_bp + 1;
    uStack4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    uStack28 = &ctx.g_alloc_addr_1050_1050;
    if ((param_4 != 0) && (param_3 != 0)) {
        i_var7 = param_3 + -1;
        u_stack12 = param_2;
        uStack14 = param_1;
        while (i_var7 != 0) {
            u_var9 = uStack14 + param_4;
            u_var12 = u_stack12 + (-CARRY2(uStack14, param_4) & 0x6c);
            uStack22 = 0x4b1c;
            uStack18 = u_var9;
            uStack16 = u_var12;
            unsafe {
                i32_var6 = (*in_fn_ptr_1)();
            }
            if (i32_var6 < 0) {
                u_var9 = param_3 - 1;
                i_var7 = 0;
                while {
                    u_var9 = u_var9 >> 1;
                    i_var7 = i_var7 + -1;
                    i_var7 != 0 && u_var9 != 0
                } {}
                if ((-i_var7 * 8 >> 0x10) == 0) {
                    u_stack12 = 0x4b4b;
                    u_var9 = pass1_fn_1000_3bac();
                    if ((-i_var7 * 8) <= u_var9) {
                        pu_var8 = &stack0xfff6;
                        lVar3 = (param_3 - 1) * param_4;
                        u_var9 = lVar3;
                        uStack14 = u_var9 + param_1;
                        u_stack12 = ((lVar3 >> 0x10) + CARRY2(u_var9, param_1)) * 0x100 + param_2;
                        uStack16 = param_2;
                        uStack18 = param_1;
                        // LAB_1000_4b7d:
                        if (pu_var8 <= &uStack18) {
                            return;
                        }
                        // LAB_1000_4b81:
                        if ((uStack16 < u_stack12)
                            || (uStack16 <= u_stack12 && (uStack18 < uStack14)))
                        {
                            uStack22 = uStack14;
                            pu_var1 = (pu_var8 + 0x14);
                            u_var12 = uStack16;
                            u_var9 = uStack18;
                            let pu_var1_val = unsafe { *pu_var1 };
                            uStack30 = uStack14 + pu_var1_val;
                            u_var4 = u_stack12 + (-CARRY2(uStack14, pu_var1_val) & 0x6c);
                            uStack26 = uStack18;
                            uStack24 = uStack16;
                            uVar15 = u_stack12;
                            u_var13 = uStack28;
                            // LAB_1000_4bbc:
                            loop {
                                uStack28 = u_var4;
                                pu_var1 = (pu_var8 + 0x14);
                                let pu_var1_val = unsafe { *pu_var1 };
                                u_var10 = u_var9 + pu_var1_val;
                                uStack38 = u_var12 + (-CARRY2(u_var9, pu_var1_val) & 0x6c);
                                if ((u_var10 != uStack14)
                                    || (
                                        u_var12 = uStack28,
                                        u_var11 = uStack30,
                                        u_var4 = uStack38,
                                        u_var5 = uStack22,
                                        uStack38 != u_stack12,
                                    ))
                                {
                                    uStack34 = uStack16;
                                    uStack36 = uStack18;
                                    uStack44 = SUB42(&PTR_LOOP_1050_4bde, 0);
                                    ppc_var2 = (pu_var8 + 0x16);
                                    uStack40 = u_var10;
                                    uStack32 = uStack38;
                                    i_var7 = ppc_var2();
                                    u_var12 = uStack28;
                                    u_var11 = uStack30;
                                    u_var4 = uStack32;
                                    u_var5 = uStack22;
                                    if (i_var7 < 1) {
                                        u_var12 = uStack32;
                                        u_var9 = u_var10;
                                        u_var4 = uStack28;
                                        if (i_var7 != 0) {
                                            uStack26 = u_var10;
                                            uStack24 = uStack32;
                                        }
                                        // goto LAB_1000_4bbc;
                                    }
                                }
                                while {
                                    uVar16 = uVar15;
                                    uStack22 = u_var5;
                                    uStack28 = u_var4;
                                    uStack30 = u_var10;
                                    pu_var1 = (pu_var8 + 0x14);
                                    let pu_var1_val = unsafe { *pu_var1 };
                                    b_var14 = u_var11 < pu_var1_val;
                                    u_var11 = u_var11 - pu_var1_val;
                                    uStack34 = u_var12 - (-b_var14 & 0x6c);
                                    uStack38 = uStack16;
                                    uStack40 = uStack18;
                                    uStack44 = SUB42(&PTR_LOOP_1050_4c0e, 0);
                                    ppc_var2 = (pu_var8 + 0x16);
                                    uStack36 = u_var11;
                                    uStack32 = uStack34;
                                    i_var7 = ppc_var2();
                                    u_var9 = uStack30;
                                    if (0 < i_var7) {
                                        break;
                                    }
                                    u_var12 = uStack32;
                                    u_var10 = uStack30;
                                    u_var4 = uStack28;
                                    u_var5 = u_var11;
                                    uVar15 = uStack32;
                                    ((i_var7 != 0)
                                        || (
                                            u_var5 = uStack22,
                                            uVar15 = uVar16,
                                            u_var11 != uStack18,
                                        ))
                                        || (uStack32 != uStack16)
                                } {}
                                if ((uStack32 < uStack28)
                                    || (uStack32 <= uStack28 && (u_var11 <= uStack30)))
                                {
                                }
                                // goto LAB_1000_4c58;
                                uStack30 = &PTR_LOOP_1050_4c46;
                                u_var12 = uStack28;
                                uStack28 = u_var13;
                                pass1_fn_1000_4ceb(*(pu_var8 + 0x14));
                                uStack30 = u_var11;
                                u_var4 = uStack32;
                                uStack26 = u_var9;
                                uStack24 = u_var12;
                                uStack22 = u_var11;
                                uVar15 = uStack32;
                                u_var13 = uStack28;
                            }
                        }
                        // goto LAB_1000_4b7d;
                    }
                }
                u_stack12 = 0x4b7b;
                bad_1000_25f2();
                return;
            }
            i_var7 = i_var7 + -1;
            u_stack12 = u_var12;
            uStack14 = u_var9;
        }
    }
    return;
    // LAB_1000_4c58:
    uStack32 = &PTR_LOOP_1050_4c68;
    uStack28 = u_var13;
    pass1_fn_1000_4ceb(*(pu_var8 + 0x14));
    u_var12 = ((u_stack12 - (-(uStack14 < uStack22) & 0x6c)) - uVar16)
        + (-CARRY2(uStack14 - uStack22, uStack18) & 0x6c)
        + uStack16;
    u_var9 = -((uStack14 - uStack22) + uStack18 < uStack26) & 0x6c;
    if ((u_var12 < u_var9) || (u_var12 - u_var9 < uStack24)) {
        u_stack12 = uStack24;
        uStack14 = uStack26;
    } else {
        uStack18 = uStack22;
        uStack16 = uVar16;
    }
    // goto LAB_1000_4b81;
}

pub unsafe fn pass1_fn_1000_4ceb(param_1: u16) {
    u_var1;
    let mut u_var2: u16;
    let mut unaff_si: i32;
    let mut unaff_DI: i32;
    let mut local_ES__1: u16;
    let temp_87f72e626cf: *mut u328_t;
    let temp_87f9aad3c2a: *mut u16;

    if ((param_1 & 1) != 0) {
        param_1 = param_1 - 1;
        temp_87f72e626cf = (param_1 + unaff_DI);
        unsafe { u_var1 = *temp_87f72e626cf };
        unsafe { *temp_87f72e626cf = *(param_1 + unaff_si) };
        *(param_1 + unaff_si) = u_var1;
        if (param_1 == 0) {
            return;
        }
    }
    while {
        param_1 = param_1 - 2;
        temp_87f9aad3c2a = (param_1 + unaff_DI);
        unsafe { u_var2 = *temp_87f9aad3c2a };
        unsafe { *temp_87f9aad3c2a = *(param_1 + unaff_si) };
        unsafe { *(param_1 + unaff_si) = u_var2 };
        param_1 != 0
    } {}
    return;
}

pub unsafe fn pass1_fn_1000_4d24() -> uint {
    let local_AL_23: u8;
    let local_AH_23: u8;
    let mut i_var1: i32;

    i_var1 = 3;
    local_AL_23 = pass1_fn_1000_52be(
        UINT_1050_61e8,
        PTR_LOOP_1050_61ea,
        (s_TPPOPMENU_1050_43fa + 3),
        3,
    );
    PTR_LOOP_1050_61ea = (i_var1 + 0x26 + (0x613c < CONCAT11(local_AH_23, local_AL_23)));
    UINT_1050_61e8 = CONCAT11(local_AH_23, local_AL_23) + 0x9ec3;
    return PTR_LOOP_1050_61ea & 0x7fff;
}

pub unsafe fn pass1_fn_1000_5008(param_1: u16, param_2: u16, param_3: u16) {
    let mut unaff_bp: i32;

    pass1_fn_1000_5026(
        0,
        param_1,
        param_2,
        param_3,
        &ctx.g_alloc_addr_1050_1050,
        unaff_bp + 1,
    );
    return;
}

pub unsafe fn pass1_fn_1000_5026(param_1: i32, in_mem_buf_ptr: u32, uparam_3: i32, param_4: u16) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut local_AX_282: u16;
    let mut unaff_bp: i32;

    let mut local_132: u16;
    let mut mem_buf_130: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_118: u16;
    local_116;
    let local_115: u8;
    let local_110: u8;
    let mut local_10e: u16;
    local_108;
    let uStack263: u8;
    let uStack262: u8;
    let mut auStack261: [u8; 257];
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_bp + 1;
    local_4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    _mem_buf_130 = CONCAT22(ctx.stack_seg_reg, &local_108);
    if (param_1 == 0) {
        param_1 = dos3_call_1000_4f94();
    }
    unsafe { *_mem_buf_130 = param_1 + '@' };
    uStack263 = 0x3a;
    local_10e = auStack261;
    uStack262 = 0x5c;
    local_115 = 'G';
    local_110 = param_1;
    dos3_call_1000_42de(&local_116);
    if (local_118 == 0) {
        u_var1 = get_string_index_1000_3da4(CONCAT22(ctx.stack_seg_reg, &local_108));
        u_var2 = u_var1 + 1;
        mem_buf_130 = in_mem_buf_ptr;
        local_12e = param_3;
        param_3 = param_3 | in_mem_buf_ptr;
        if (param_3 == 0) {
            if (param_4 < u_var2) {
                param_4 = u_var2;
            }
            mem_buf_130 = alloc_mem_1000_167a(param_4, 0);
            local_12e = param_3;
            if ((param_3 | mem_buf_130) == 0) {
                PTR_LOOP_1050_5f78 = &PTR_LOOP_1050_000c;
                return;
            }
        }
        if (param_4 < u_var2) {
            PTR_LOOP_1050_5f78 = (s_New_failed_in_Op__Op_1050_0020 + 2);
        } else {
            copy_string_1000_3d3e(
                CONCAT22(local_12e, mem_buf_130),
                CONCAT22(ctx.stack_seg_reg, &local_108),
            );
        }
    } else {
        PTR_LOOP_1050_5f78 = (&PTR_LOOP_1050_000c + 1);
        PTR_LOOP_1050_5f88 = local_124;
    }
    return;
}

pub unsafe fn pass1_fn_1000_52be(in_i16_1: u16, in_i16_2: u16, in_u16_3: u16, in_u16_4: u16) {
    if ((in_u16_4 | in_i16_2) == 0) {
        return (in_i16_1 * in_u16_3);
    }
    return (in_i16_1 * in_u16_3);
}

pub unsafe fn pass1_fn_1000_52f0(param_1: i32, param_2: i32, uparam_3: i32, param_4: i32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut u_var10: i32;
    let mut u_var11: i32;
    let mut b_var12: bool;
    let mut b_var13: bool;

    b_var13 = param_2 < 0;
    if (b_var13) {
        b_var12 = param_1 != 0;
        param_1 = -param_1;
        param_2 = -b_var12 - param_2;
    }
    u_var11 = b_var13;
    if (param_4 < 0) {
        b_var13 = param_3 != 0;
        param_3 = -param_3;
        param_4 = -b_var13 - param_4;
    }
    u_var3 = param_1;
    u_var4 = param_3;
    u_var8 = param_2;
    u_var9 = param_4;
    if (param_4 == 0) {
        i_var5 = ((param_2 % param_3 << 0x10 | param_1) % param_3);
        i32_var6 = 0;
        if ((u_var11 - 1) < 0) {}
        // goto LAB_1000_538a;
    } else {
        while {
            u_var10 = u_var9 >> 1;
            u_var4 = u_var4 >> 1 | ((u_var9 & 1) != 0) << 0xf;
            u_var7 = u_var8 >> 1;
            u_var3 = u_var3 >> 1 | ((u_var8 & 1) != 0) << 0xf;
            u_var8 = u_var7;
            u_var9 = u_var10;
            u_var10 != 0
        } {}
        u_var1 = CONCAT22(u_var7, u_var3) / u_var4;
        u_var3 = u_var1 * param_4;
        lVar2 = (u_var1 & 0xffff) * param_3;
        u_var8 = (lVar2 >> 0x10);
        u_var4 = lVar2;
        u_var9 = u_var8 + u_var3;
        if (((CARRY2(u_var8, u_var3)) || (param_2 < u_var9))
            || (param_2 <= u_var9 && (param_1 < u_var4)))
        {
            b_var13 = u_var4 < param_3;
            u_var4 = u_var4 - param_3;
            u_var9 = (u_var9 - param_4) - b_var13;
        }
        i_var5 = u_var4 - param_1;
        i32_var6 = (u_var9 - param_2) - (u_var4 < param_1);
        if (-1 < (u_var11 - 1)) {}
        // goto LAB_1000_538a;
    }
    b_var13 = i_var5 != 0;
    i_var5 = -i_var5;
    i32_var6 = -b_var13 - i32_var6;
    // LAB_1000_538a:
    return CONCAT22(i32_var6, i_var5);
}

pub unsafe fn pass1_fn_1000_5390(param_1: i32, param_2: i32, uparam_3: i32, param_4: i32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut u_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;

    u_var3 = param_1;
    u_var8 = param_4;
    u_var6 = param_2;
    u_var9 = param_3;
    if (param_4 == 0) {
        u_var3 = param_2 / param_3;
        i_var4 = ((param_2 % param_3 << 0x10 | param_1) / param_3);
    } else {
        while {
            u_var5 = u_var8 >> 1;
            u_var9 = u_var9 >> 1 | ((u_var8 & 1) != 0) << 0xf;
            u_var7 = u_var6 >> 1;
            u_var3 = u_var3 >> 1 | ((u_var6 & 1) != 0) << 0xf;
            u_var8 = u_var5;
            u_var6 = u_var7;
            u_var5 != 0
        } {}
        u_var1 = CONCAT22(u_var7, u_var3) / u_var9;
        i_var4 = u_var1;
        lVar2 = param_3 * (u_var1 & 0xffff);
        u_var3 = (lVar2 >> 0x10);
        u_var8 = u_var3 + i_var4 * param_4;
        if (((CARRY2(u_var3, i_var4 * param_4)) || (param_2 < u_var8))
            || (param_2 <= u_var8 && (param_1 < lVar2)))
        {
            i_var4 = i_var4 + -1;
        }
        u_var3 = 0;
    }
    return CONCAT22(u_var3, i_var4);
}

pub unsafe fn pass1_fn_1000_53f0(param_1: i32, param_2: i32, uparam_3: i32, param_4: i32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut u_var10: i32;
    let mut b_var11: bool;

    u_var3 = param_1;
    u_var4 = param_4;
    u_var9 = param_2;
    u_var10 = param_3;
    if (param_4 == 0) {
        i32_var6 = ((param_2 % param_3 << 0x10 | param_1) % param_3);
        i_var7 = 0;
    } else {
        while {
            u_var5 = u_var4 >> 1;
            u_var10 = u_var10 >> 1 | ((u_var4 & 1) != 0) << 0xf;
            u_var8 = u_var9 >> 1;
            u_var3 = u_var3 >> 1 | ((u_var9 & 1) != 0) << 0xf;
            u_var4 = u_var5;
            u_var9 = u_var8;
            u_var5 != 0
        } {}
        u_var1 = CONCAT22(u_var8, u_var3) / u_var10;
        u_var3 = u_var1 * param_4;
        lVar2 = (u_var1 & 0xffff) * param_3;
        u_var9 = (lVar2 >> 0x10);
        u_var4 = lVar2;
        u_var10 = u_var9 + u_var3;
        if (((CARRY2(u_var9, u_var3)) || (param_2 < u_var10))
            || (param_2 <= u_var10 && (param_1 < u_var4)))
        {
            b_var11 = u_var4 < param_3;
            u_var4 = u_var4 - param_3;
            u_var10 = (u_var10 - param_4) - b_var11;
        }
        i32_var6 = -(u_var4 - param_1);
        i_var7 = -(u_var4 - param_1 != 0) - ((u_var10 - param_2) - (u_var4 < param_1));
    }
    return CONCAT22(i_var7, i32_var6);
}

pub unsafe fn pass1_fn_1000_54a0(param_1: u32, param_2: u16, param_3: u16) -> u32 {
    let pu_var1: *mut u32;
    local_AL_41;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let pu_var6: *mut u32;
    let mut in_stack_00000006: i32;

    if (param_3 != 0) {
        u_var4 = -param_1;
        u_var5 = param_3;
        if (u_var4 != 0) {
            u_var5 = (u_var4 - param_3 & -(u_var4 < param_3)) + param_3;
            u_var4 = param_3 - u_var5;
        }
        u_var2 = param_2 & 0xff | param_2 << 8;
        u_var3 = u_var5 >> 1;
        pu_var6 = param_1;
        while (u_var3 != 0) {
            u_var3 = u_var3 - 1;
            pu_var1 = pu_var6;
            pu_var6 = pu_var6 + 1;
            unsafe { *pu_var1 = u_var2 };
        }
        u_var3 = ((u_var5 & 1) != 0);
        while (local_AL_41 = (param_2 & 0xff), u_var3 != 0) {
            u_var3 = u_var3 - 1;
            pu_var1 = pu_var6;
            pu_var6 = (pu_var6 + 1);
            unsafe { *pu_var1 = local_AL_41 };
        }
        if (u_var4 != 0) {
            u_var3 = u_var4 >> 1;
            while (u_var3 != 0) {
                u_var3 = u_var3 - 1;
                pu_var1 = pu_var6;
                pu_var6 = pu_var6 + 1;
                unsafe { *pu_var1 = u_var2 };
            }
            u_var4 = ((u_var4 & 1) != 0);
            while (u_var4 != 0) {
                u_var4 = u_var4 - 1;
                pu_var1 = pu_var6;
                pu_var6 = (pu_var6 + 1);
                unsafe { *pu_var1 = local_AL_41 };
            }
        }
    }
    return param_1;
}

pub unsafe fn pass1_fn_1008_04d2(param_1: u32, param_2: u8) {
    handle_error_1008_9466(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_fn_1008_07d8(in_u16_1: u16, in_u16_ptr_1: *mut u16) {
    let mut b_var1: bool;
    let mut u_var2: u16;
    let mut local_4: u16;

    if (ctx._g_bool_1050_5748 == 0x0) {
        process_struct_1000_179c(10, in_u16_ptr_1);
        u_var2 = in_u16_ptr_1 | in_u16_1;
        if (u_var2 != 0) {
            b_var1 = pass1_1030_8128(CONCAT22(in_u16_ptr_1, in_u16_1));
            in_u16_1 = b_var1;
        }
        if (ctx._g_bool_1050_5748 == 0x0) {
            fn_1008_6048(
                s_New_failed_in_Op__Op__Simulator_1050_0110,
                u_var2,
                SUB21(in_u16_1, 0),
            );
            call_fn_ptr_1000_24cd(1);
        }
        pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
        pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
        pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
        pass1_1030_838e(ctx._g_bool_1050_5748);
        pass1_1030_8334();
    }
    return;
}

pub unsafe fn pass1_1008_0a92(param_1: u32) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut local_es_4: i32;
    let mut fn_ptr_1: Vec<u8>;
    let mut temp_87f6e103bce: Vec<u8>;

    local_es_4 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xee) != 0) {
        fn_ptr_1 = ((i_var2 + 0xee) + 0x90);
        (**fn_ptr_1)();
    }
    if ((i_var2 + 0xe8) != 0) {
        temp_87f6e103bce = ((i_var2 + 0xe8) + 0x90);
        (**temp_87f6e103bce)();
    }
    if (_PTR_LOOP_1050_0388 != 0x0) {
        unsafe { pp_var1 = *_PTR_LOOP_1050_0388 };
        (**pp_var1)();
    }
    post_quit_msg_1008_3af4(param_1);
    return;
}

pub unsafe fn pass1_1008_1272(param_1: u32, param_2: i32) {
    let mut uint_1: i32;
    let mut fn_ptr_1: u32;

    uint_1 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        fn_ptr_1 = ((param_1 + 0xe8) + 0x88);
        (**fn_ptr_1)();
        return;
    }
    pass1_1008_9cc4(param_1 & 0xffff | uint_1 << 0x10, param_2);
    return;
}

pub unsafe fn pass1_1008_12aa(param_1: u32) {
    let mut local_es_3: i32;
    char * *fn_ptr_1;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        fn_ptr_1 = ((param_1 + 0xe8) + 0x8c);
        (**fn_ptr_1)();
        return;
    }
    pass1_1008_9ce0();
    return;
}

pub unsafe fn pass1_1008_3714(param_1: *mut *mut u8) {
    pass1_1008_3e0e(param_1);
    return;
}

pub unsafe fn pass1_1008_372c(param_1: i32, param_2: i32) {
    return CONCAT22(param_2, param_1 + 10);
}

pub unsafe fn pass1_1008_37aa(in_list_1: *mut u32, param_2: u8) -> *mut u32 {
    let mut u_var1: u16;

    u_var1 = (in_list_1 >> 0x10);
    unsafe { *in_list_1 = 0x380a };
    (in_list_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *in_list_1 = ctx.s_1_1050_389a };
    (in_list_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_list_1);
    }
    return in_list_1;
}

pub unsafe fn pass1_1008_37e4(param_1: u32, param_2: u8) {
    win_cleanup_1008_0618(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_392e(param_1: *mut u16, param_2: u16) {
    let local_bx_4: i16;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = (ctx.s_18_2_1050_3aa5 + 3) };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    (local_bx_4 + 4) = param_2;
    unsafe { *param_1 = ctx.s_0_020_1050_3ab0 };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_0_76_1050_3aa0 };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    return param_1;
}

pub unsafe fn pass1_1008_397a(param_1: *mut u16) {
    let mut local_bx_4: u16;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    unsafe { *param_1 = s_0_76_1050_3aa0 };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_0_020_1050_3ab0 };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (local_bx_4 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_3afe(param_1: *mut Struct181, param_2: u8) {
    let local_AX_8: *mut Struct181;
    let mut u_var1: u16;

    local_AX_8 = param_1;
    local_AX_8 = local_AX_8 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(local_AX_8)));
    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    local_AX_8.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    local_AX_8.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_3b7a(in_char_1: u8, in_char_2: u8, in_u16_3: u16) {
    let pb_var1: *mut u8;
    let local_SP: *mut u16;
    let local_BP__1: *mut u16;

    let char_1: u8;

    local_SP = &stack0xfffe;
    char_1 = '\x0f';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe { *local_SP = *local_BP__1 };
        char_1 = char_1 + -1;
        '\0' < char_1
    } {}
    pb_var1 = (in_u16_3 + in_char_1);
    unsafe { *pb_var1 = *pb_var1 | in_char_2 };
    loop {
        // WARNING: Do nothing block with infinite loop
    }
}

pub unsafe fn pass1_1008_3d44(param_1: u16, param_2: u8) {
    let mut u_var1: u16;
    let local_res7: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(local_res7, CONCAT12(param_2, param_1));
    u_var1 = (_param_1 >> 0x10);
    _param_1.ptr_a_lo = 0x380a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    _param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        error_check_1000_17ce(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub unsafe fn pass1_1008_3db2(
    param_1: u8,
    param_2: u16,
    param_3: u16,
    param_4: bool,
    param_5: u8,
    param_6: u8,
    param_7: u8,
    param_8: u8,
) {
    let pb_var1: *mut u8;
    let mut b_var2: bool;
    let mut b_var3: bool;
    let mut b_var4: bool;
    let mut b_var5: u8;
    let ppcVar6: fn();
    let mut cVar7: u8;
    let mut u_var8: u32;
    let mut b_var9: u8;
    let mut b_var10: u8;
    let mut i_var11: i32;
    let mut b_var12: u8;
    let mut b_var13: u8;
    let local_BX__1: *mut u32;
    let pu_var14: *mut u32;
    u32 * *local_SP;
    let unaff_bp: *mut u32;
    let unaff_si: *mut u8;
    let pb_var15: *mut u8;
    let unaff_DI: *mut u8;
    let unaff_es: *mut u8;

    let puStack2: *mut u32;
    let temp_179f29f37356: *mut u32;
    let mut temp_5fdbfafefb: u32;

    loop {
        local_SP = &puStack2;
        local_SP = &puStack2;
        cVar7 = '\x0f';
        temp_179f29f37356 = unaff_bp;
        while {
            temp_179f29f37356 = (temp_179f29f37356 + -2);
            local_SP = local_SP + -1;
            unsafe { *local_SP = temp_179f29f37356 };
            cVar7 = cVar7 + -1;
            '\0' < cVar7
        } {}
        i_var11 = param_1;
        pb_var1 = (local_BX__1 + i_var11);
        b_var12 = param_3;
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var2 = 9 < (unaff_si & 0xf);
        b_var5 = b_var2 | param_5;
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var3 = 9 < (unaff_si + b_var5 * '\x06' & 0xf);
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        let pb_var1_val = unsafe { *pb_var1 };
        b_var9 = ((POPCOUNT(pb_var1_val) & 1) == 0) * 0x4;
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var5 = 9 < b_var9 | b_var3 | b_var5;
        b_var10 = b_var9 + b_var5 * '\x06' & 0xf;
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var5 = 9 < b_var10 | b_var5;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        pb_var15 = unaff_DI + -1;
        pb_var1 = (local_BX__1 + pb_var15);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        pb_var1 = (local_BX__1 + pb_var15);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        param_3 = param_3 - 1;
        pb_var1 = (local_BX__1 + pb_var15);
        b_var13 = param_3;
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = unaff_DI;
        unsafe { *pb_var1 = *pb_var1 & (local_BX__1 >> 8) };
        pb_var1 = (local_BX__1 + pb_var15);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        b_var12 = (b_var10 + b_var5 * '\x06' & 0xf) + 1;
        let pv_var15_val = unsafe { *pb_var15 };
        out(pv_var15_val, param_3);
        b_var4 = 9 < (b_var12 & 0xf);
        b_var5 = b_var4 | b_var5;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pu_var14 = local_BX__1;

        if (pb_var1_val == 0) {}
        // goto code_r0x10083e29;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        if ((POPCOUNT(pb_var1_val) & 1) == 0) {
            break;
        }
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pu_var14 = ((param_8 & 1) * 0x4000
            | (param_7 & 1) * 0x200
            | (param_6 & 1) * 0x100
            | (pb_var1_val < '\0') * 0x80
            | (pb_var1_val == 0) * 0x40
            | ((b_var4 | 9) < b_var10 | (9 < b_var9) | b_var3 | b_var2 | param_5 & 1) * 0x10
            | ((POPCOUNT(pb_var1_val) & 1) == 0) * 4);
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = unaff_DI + 0x1008;
        unsafe { *pb_var1 = *pb_var1 ^ local_BX__1 };
        unsafe { b_var9 = *pb_var1 };
        unaff_DI[0x1008] = local_BX__1;
        temp_5fdbfafefb = (unaff_DI + 0x1008);
        unaff_es = (temp_5fdbfafefb >> 0x10);
        local_BX__1 = temp_5fdbfafefb;
        param_2 = param_2 - 1;
        if (param_2 == 0 || b_var9 == 0) {
            pb_var1 = (local_BX__1 + unaff_DI);
            unsafe { *pb_var1 = *pb_var1 | b_var13 };
            unaff_es = unaff_DI;
            if (pu_var14[1] != 0) {
                // code_r0x10083e29:
                u_var8 = pu_var14[1];
                unaff_es = (u_var8 >> 0x10);
                local_BX__1 = u_var8;
                // code_r0x10083e2d:
                let local_bx_1_val = unsafe { *local_BX__1 };
                ppcVar6 = (local_bx_1_val + 4);
                puStack2 = unaff_bp;
                (**ppcVar6)();
            }
            return;
        }
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        b_var9 = (b_var12 + b_var5 * '\x06' & 0xf) - 1;
        b_var5 = 9 < (b_var9 & 0xf) | b_var5;
        b_var9 = b_var9 + b_var5 * '\x06' & 0xf;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        param_5 = 9 < b_var9 | b_var5;
        param_1 = b_var9 + param_5 * '\x06' & 0xf;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        unaff_bp = local_SP;
        unaff_si = unaff_DI;
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val == 0) {}
    // goto code_r0x10083e29;
    // goto code_r0x10083e2d;
}

pub unsafe fn pass1_1008_3e0e(param_1: *mut *mut u8) {
    let mut local_es_3: u16;
    // fn_ptr_1: *mut *mut u8;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 4) != 0) {
        fn_ptr_1 = ((param_1 + 4) + 4);
        (**fn_ptr_1)();
    }
    return;
}

pub unsafe fn pass1_1008_3e54(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_es_6: u16;

    local_es_6 = (param_1 >> 0x10);
    unsafe { *param_1 = param_4 };
    (param_1 + 2) = param_3;
    (param_1 + 4) = param_2;
    return param_1;
}

pub unsafe fn pass1_1008_3e76(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_es_6: u16;

    local_es_6 = (param_1 >> 0x10);
    unsafe { *param_1 = param_4 };
    (param_1 + 2) = param_3;
    (param_1 + 4) = param_2;
    return;
}

pub unsafe fn pass1_1008_3e94(param_1: *mut Struct199, param_2: *mut u8, param_3: *mut u8) {
    let mut in_stack_00000006: u16;

    unsafe { param_3 = *_param_1 };
    param_2 = param_1.field_0x2;
    return;
}

pub unsafe fn pass1_1008_3eb4(param_1: *mut u16, param_2: u32, param_3: u32, param_4: u32) {
    let mut local_es_15: u16;

    unsafe { param_4 = *param_1 };
    local_es_15 = (param_1 >> 0x10);
    param_3 = (param_1 + 2);
    param_2 = (param_1 + 4);
    return;
}

pub unsafe fn pass1_1008_3ee2(param_1: *mut i32, param_2: *mut i32) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    let mut ctx.es_reg: u16;
    let mut local_es_33: u16;

    unsafe { i_var1 = *param_2 - *param_1 };
    if (i_var1 < 0) {
        i_var1 = -i_var1;
    }
    unsafe { *param_1 = i_var1 + 1 };
    ctx.es_reg = (param_2 >> 0x10);
    local_es_33 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var1 = (param_2 + 2) - (i_var2 + 2);
    if (i_var1 < 0) {
        i_var1 = -i_var1;
    }
    (i_var2 + 2) = i_var1 + 1;
    i_var1 = (param_2 + 4) - (i_var2 + 4);
    if (i_var1 < 0) {
        i_var1 = -i_var1;
    }
    (i_var2 + 4) = i_var1 + 1;
    return;
}

pub unsafe fn pass1_1008_3f32(param_1: *mut i32, param_2: *mut i32) {
    let pi_var1: *mut i32;
    let mut local_es_15: u16;
    let mut local_es_22: u16;

    unsafe { *param_1 = *param_1 + *param_2 };
    local_es_15 = (param_2 >> 0x10);
    local_es_22 = (param_1 >> 0x10);
    pi_var1 = (param_1 + 2);
    unsafe { *pi_var1 = *pi_var1 + (param_2 + 2) };
    pi_var1 = (param_1 + 4);
    unsafe { *pi_var1 = *pi_var1 + (param_2 + 4) };
    return;
}

pub unsafe fn pass1_1008_3f92(in_struct_a: *mut Struct103, param_2: *mut Struct183) {
    let struct_a_hi: *mut Struct103;
    let struct_a: *mut Struct103;
    // fn_ptr_1: *mut *mut u8;

    struct_a = in_struct_a;
    set_struct_1008_56b4(struct_a);
    struct_a_hi = (in_struct_a >> 0x10);
    struct_a.field_0x6 = 0;
    struct_a.field_0xa = 0;
    struct_a.field_0xe = 0;
    struct_a.field_0x10 = 0;
    struct_a.field_0x14 = 0;
    struct_a.field_0x18 = 0;
    struct_a.field_0x1c = 0;
    in_struct_a.ptr_1_lo = &PTR_LOOP_1050_48de;
    struct_a.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    if (param_2 == 0x0) {
        return;
    }
    fn_ptr_1 = (param_2 + 8);
    (**fn_ptr_1)();
    pass1_1008_4214(struct_a, param_2);
    process_struct_1008_47cc(in_struct_a);
    process_struct_1008_4834(in_struct_a);
    return;
}

pub unsafe fn pass1_1008_405c(
    param_1: *mut Struct103,
    param_2: *mut u32,
    param_3: u16,
    param_5: i32,
    param_4: i32,
) -> i32 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut local_AX_116: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let paVar6: *mut Struct103;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut temp_5fb8438c55: u32;
    let lVar3: u32;

    paVar6 = param_1;
    set_struct_1008_56b4(paVar6);
    u_var5 = (param_1 >> 0x10);
    paVar6.field_0x6 = 0;
    paVar6.field_0xa = 0;
    paVar6.field_0xe = 0;
    paVar6.field_0x10 = 0;
    paVar6.field_0x14 = 0;
    paVar6.field_0x18 = 0;
    paVar6.field_0x1c = 0;
    param_1.ptr_1_lo = &PTR_LOOP_1050_48de;
    paVar6.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    i_var2 = param_4 * 8 + 0x1f;
    i_var2 = ((i_var2 + (i_var2 >> 0xf & 0x1f)) >> 5) << 2;
    _local_a = param_3;
    u_var4 = ((i_var2 * param_3) >> 0x20);
    lVar3 = (i_var2 * param_3) + 0x436;
    alloc_mem_1000_0a48(1, lVar3, ctx.__g_Struct94_ptr_1);
    &paVar6.field_0x6 = lVar3;
    (&paVar6.field_0x6 + 2) = u_var4;
    process_struct_1008_47cc((param_1 & 0xffff | u_var5 << 0x10));
    &paVar6.field_0x18 = i_var2;
    (&paVar6.field_0x18 + 2) = i_var2 >> 0xf;
    paVar6.field_0x10 = 0x28;
    temp_5fb8438c55 = paVar6.field_0x10;
    (temp_5fb8438c55 + 4) = param_4;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 8) = _local_a;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0xc) = 1;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0xe) = 8;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x10) = 0;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x14) = paVar6.field_0x18 * _local_a;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x20) = 0x100;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x24) = 0x100;
    process_struct_1008_4834(param_1);
    pass1_1008_4d84(paVar6.field_0xa, param_2);
    return;
}

pub unsafe fn pass1_1008_4214(param_1: *mut Struct103, in_Struct183: *mut Struct183) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_Struct183: *mut Struct183;
    let mut local_es_5: u16;
    let mut in_stack_00000006: u16;
    let temp_862d998a63d: *mut u32;
    // fn_ptr_1: *mut *mut u8;

    local_es_5 = (in_Struct183 >> 0x10);
    local_Struct183 = in_Struct183;
    param_1.field_0x6 = local_Struct183.field_0x1a;
    local_Struct183.field_0x1a = 0;
    pu_var1 = local_Struct183.field_0x4;
    u_var2 = &local_Struct183.field_0x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe { fn_ptr_1 = *pu_var1 };
        (**fn_ptr_1)();
    }
    &local_Struct183.field_0x4 = 0;
    local_Struct183.field_0xe = 0;
    local_Struct183.field_0x12 = 0;
    local_Struct183.field_0x16 = 0;
    local_Struct183.field_0x1e = 0;
    return '\0';
}

pub unsafe fn pass1_1008_431c(in_Struct184: *mut Struct184, param_2: u8) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let mut b_var3: bool;
    let mut local_AX_134: u16;
    let mut local_DX_134: u16;
    let local_Struct184: *mut Struct184;
    let mut u_var5: i32;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fa7e01195: u32;
    let mut u_var4: u32;

    u_var5 = (in_Struct184 >> 0x10);
    local_Struct184 = in_Struct184;
    if (&local_Struct184.field_0x6 == 0) {
        process_struct_1008_47cc((in_Struct184 & 0xffff | u_var5 << 0x10));
    }
    if ((local_Struct184.field_0x8 | local_Struct184.field_0x6) == 0) {
        b_var3 = false;
    } else {
        if ((local_Struct184.field_0xc | local_Struct184.field_0xa) == 0) {
            process_struct_1008_4834((in_Struct184 & 0xffff | u_var5 << 0x10));
        }
        b_var3 = true;
    }
    if (b_var3) {
        if ((local_Struct184.field_0x16 | local_Struct184.field_0x14) == 0) {
            return;
        }
        local_6 = 0;
        while (true) {
            temp_5fa7e01195 = local_Struct184.field_0x10;
            pu_var1 = (temp_5fa7e01195 + 8);
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val == local_6 || pu_var1_val < local_6) {
                break;
            }
            u_var4 = local_6;
            process_struct_1008_4544(in_Struct184);
            u_var2 = local_Struct184.field_0x10;
            pass1_1000_4906(
                (u_var4 & 0xffff | local_DX_134 << 0x10),
                param_2,
                (u_var2 + 4),
            );
            local_6 = local_6 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1008_43cc(param1: *mut i32) {
    let mut b_var1: bool;
    let num_list: *mut i32;
    let mut u_var2: i32;
    let mut local_4: u16;

    u_var2 = (i32_list >> 0x10);
    num_list = i32_list;
    if ((num_list + 3) == 0) {
        process_struct_1008_47cc((i32_list & 0xffff | u_var2 << 0x10));
    }
    if ((num_list + 3) == 0) {
        b_var1 = false;
    } else {
        if ((num_list + 5) == 0) {
            process_struct_1008_4834((i32_list & 0xffff | u_var2 << 0x10));
        }
        b_var1 = true;
    }
    if (!b_var1) {
        return 0;
    }
    return CONCAT22(num_list[0xb], num_list[10]);
}

pub unsafe fn pass1_1008_4426(param_1: *mut Struct104) {
    let mut b_var1: bool;
    let local_bx_4: *mut Struct104;
    let mut u_var2: i32;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.a == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | u_var2 << 0x10));
    }
    if (&local_bx_4.a == 0) {
        b_var1 = false;
    } else {
        if (&local_bx_4.field_0xa == 0) {
            process_struct_1008_4834((param_1 & 0xffff | u_var2 << 0x10));
        }
        b_var1 = true;
    }
    if (!b_var1) {
        return 0;
    }
    return CONCAT22(&local_bx_4.field_0xc, &local_bx_4.field_0xa);
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_4480(param_1: u32, in_struct_1: *mut Struct417, param_3: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_DX_29: u16;
    let mut local_DX_97: u16;
    let mut local_DX_122: u16;

    let mut u_var4: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94(
        in_struct_1,
        CONCAT22(ctx.stack_seg_reg, &local_6),
        CONCAT22(ctx.stack_seg_reg, &local_4),
    );
    u_var4 = process_struct_1008_4772(param_3);
    local_DX_29 = (u_var4 >> 0x10);
    u_var1 = (u_var4 + 4);
    i_var2 = (u_var4 + 8);
    local_10 = 0;
    while (local_10 < i_var2) {
        u_var3 = local_6;
        local_6 = local_6 + 1;
        process_struct_1008_4544(param_1);
        _local_14 = CONCAT22(local_DX_97, u_var3);
        u_var4 = SEXT24(local_10);
        process_struct_1008_4544(param_3);
        _local_18 = (u_var4 & 0xffff | local_DX_122 << 0x10);
        local_1a = u_var1;
        while (local_1a != 0) {
            if (*_local_18 != -1) {
                unsafe { *_local_14 = *_local_18 };
            }
            _local_18 = CONCAT22(
                (_local_18 >> 0x10) + (-(0xfffe < local_18) & 0x6c),
                local_18 + 1,
            );
            _local_14 = CONCAT22(
                (_local_14 >> 0x10) + (-(0xfffe < local_14) & 0x6c),
                local_14 + 1,
            );
            local_1a = local_1a - 1;
        }
        local_10 = local_10 + 1;
    }
    return;
}

pub unsafe fn pass1_1008_4b5e(param_1: *mut Struct189) {
    let mut i_var1: i32;
    let local_struct_1: *mut Struct189;
    let mut local_es_3: u16;
    // fn_ptr: *mut *mut u8;

    local_es_3 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0x1e == 0) {
        fn_ptr = (param_1 + 8);
        i_var1 = (**fn_ptr)();
        if (i_var1 == 0) {
            return 0;
        }
    }
    return CONCAT22(local_struct_1.field_0x6, local_struct_1.field_0x4);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_4b8e(in_Struct7_1: *mut Struct7, param_2: *mut Struct131) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut local_DX_11: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_res6: u16;
    let mut in_stack_0000ffe4: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x48),
    );
    u_var2 = (ppVar4 >> 0x10);
    u_var1 = (ppVar4 + 0x18);
    i_var3 = (ppVar4 + 0x16) / 2;
    local_10 = 0;
    while (local_a._0_2_ = u_var1, local_10 < i_var3) {
        pass1_1008_4d26(
            &in_Struct7_1.func_ptr_0x4,
            u_var1 & 0xffff0000 | (local_10 * 4 + local_a),
            local_10,
        );
        local_10 = local_10 + 1;
    }
    local_12 = 0x100 - i_var3;
    while (local_12 < 0x100) {
        pass1_1008_4d26(
            &in_Struct7_1.func_ptr_0x4,
            u_var1 & 0xffff0000 | (local_10 * 4 + local_a),
            local_12,
        );
        local_12 = local_12 + 1;
        local_10 = local_10 + 1;
    }
    return;
}

pub unsafe fn pass1_1008_4cdc(param_1: *mut Struct192) {
    let local_bx_4: *mut Struct192;
    let mut local_es_4: u16;
    let mut temp_5f3eb4e5e7: u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0x4f1c;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    error_check_1000_17ce(local_bx_4.field_0xe);
    if (local_bx_4.field_0x12 != 0) {
        error_check_1000_17ce(local_bx_4.field_0x4);
    }
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_4d26(param_1: *mut Struct193, param_2: u32, param_3: u16) {
    let pu_var1: *mut u16;
    let local_bx_4: *mut Struct193;
    let mut i_var2: i32;
    let mut local_es_4: u16;
    let mut local_es_51: u16;
    let mut temp_5ffd109e1a: u16;
    let mut temp_5fce800a58: u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    let pu_var1_val = unsafe { *pu_var1 };
    if (((local_bx_4.field_0x4 != 0) && (-1 < param_3))
        && (
            pu_var1 = &local_bx_4.field_0xc,
            pu_var1_val != param_3 && param_3 <= pu_var1_val,
        ))
    {
        temp_5ffd109e1a = (param_2 + 2);
        temp_5fce800a58 = local_bx_4.field_0x4;
        local_es_51 = (temp_5fce800a58 >> 0x10);
        i_var2 = temp_5fce800a58;
        (i_var2 + param_3 * 4) = param_2;
        (i_var2 + param_3 * 4 + 2) = temp_5ffd109e1a;
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1008_4d72(param_1: u32) {
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub unsafe fn pass1_1008_4d84(param_1: *mut Struct194, param_2: *mut u32) {
    let pu_var1: *mut u16;
    let in_dx: *mut Struct199;
    let local_bx_4: *mut Struct194;
    let mut local_es_4: u16;
    let mut local_es_12: u16;
    let mut in_stack_0000000a: u16;
    let mut temp_5fed13c9c0: u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x12 != 0) {
        local_bx_4.field_0xc = (param_2 + 3);
        error_check_1000_17ce(&local_bx_4.field_0x4);
        &local_bx_4.field_0x4 = 0;
        pu_var1 = (local_bx_4.field_0xc << 2);
        process_struct_1000_179c(pu_var1, in_dx);
        local_bx_4.field_0x4 = pu_var1;
        &local_bx_4.field_0x6 = in_dx;
    }
    if (local_bx_4.field_0xc != 0x100) {
        return;
    }
    pass1_fn_1000_48a8(&local_bx_4.field_0x4, param_2[1], 0x400);
    return;
}

// WARNING: Variable defined which should be unmapped: local_4

pub unsafe fn pass1_1008_5068(param_1: *mut Struct103, param_2: u16, param_3: *mut Struct183) {
    pass1_1008_4214(param_1, param_2);
    return;
}

pub unsafe fn pass1_1008_507c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    process_struct_1008_41bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_5118(param_1: *mut Struct376) {
    let mut local_es_3: u16;
    let mut temp_5f496a68b0: u32;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 0x10) != 0) {
        temp_5f496a68b0 = (param_1 + 0x10);
        error_check_1000_0dc6(temp_5f496a68b0, (temp_5f496a68b0 >> 0x10));
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_5134(param_1: u32) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut var4: u16;
    let mut local_AX_21: u16;
    let mut u_var5: i32;
    let lVar6: u32;
    let mut i_var7: i32;
    let local_bx_6: *mut Struct196;
    let mut local_es_6: u16;
    let mut b_var8: bool;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_es_6 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    sVar4 = &local_bx_6.field_0x4 * &local_bx_6.field_0x8;
    u_var5 = (sVar4 >> 0x20);
    local_AX_21 = sVar4;
    alloc_mem_1000_0a48(
        1,
        local_AX_21,
        (sVar4 >> 0x10),
        ctx.__g_Struct94_ptr_1,
        (ctx.__g_Struct94_ptr_1 >> 0x10),
    );
    local_bx_6.field_0x10 = local_AX_21;
    local_bx_6.field_0x12 = u_var5;
    if ((u_var5 | local_bx_6.field_0x10) == 0) {
        return;
    }
    i_var7 = local_bx_6.field_0x8;
    i_var2 = local_bx_6.field_0xa;
    lVar6 = CONCAT22(i_var2 - (i_var7 == 0), i_var7 + -1) * &local_bx_6.field_0x4;
    pu_var1 = &local_bx_6.field_0x10;
    u_var5 = lVar6;
    let pu_var1_val = unsafe { *pu_var1 };
    local_a = u_var5 + pu_var1_val;
    local_8 = ((lVar6 >> 0x10) + CARRY2(u_var5, pu_var1_val)) * 0x100 + local_bx_6.field_0x12;
    _local_e = CONCAT22(i_var2, i_var7);
    local_10 = local_bx_6.field_0x2;
    while (_local_e != 0) {
        u_var3 = local_10 + 1;
        i_var7 = local_10 >> 0xf;
        process_struct_1008_4544(local_bx_6.field_0xc);
        pass1_fn_1000_48a8(
            CONCAT22(local_8, local_a),
            CONCAT22(i_var7, local_10),
            local_bx_6.field_0x4,
        );
        i_var7 = local_bx_6.field_0x4;
        u_var5 = -i_var7;
        b_var8 = CARRY2(local_a, u_var5);
        local_a = local_a + u_var5;
        local_8 = local_8 + (b_var8 - (local_bx_6.field_0x6 + (i_var7 != 0))) * 0x100;
        local_10 = u_var3;
        _local_e = _local_e + -1;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_5236(param_1: *mut Struct197) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let lVar5: u32;
    let mut i32_var6: i32;
    let local_bx_6: *mut Struct197;
    let mut local_es_6: u16;
    let mut b_var7: bool;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_6 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    i32_var6 = local_bx_6.field_0x8;
    i_var2 = local_bx_6.field_0xa;
    lVar5 = CONCAT22(i_var2 - (i32_var6 == 0), i32_var6 + -1) * &local_bx_6.field_0x4;
    pu_var1 = &local_bx_6.field_0x10;
    u_var4 = lVar5;
    let pu_var1_val = unsafe { *pu_var1 };
    local_6 = u_var4 + pu_var1_val;
    local_4 = ((lVar5 >> 0x10) + CARRY2(u_var4, pu_var1_val)) * 0x100 + local_bx_6.field_0x12;
    _local_a = CONCAT22(i_var2, i32_var6);
    local_c = local_bx_6.field_0x2;
    while (_local_a != 0) {
        u_var3 = local_c + 1;
        i32_var6 = local_c >> 0xf;
        process_struct_1008_4544(local_bx_6.field_0xc);
        pass1_fn_1000_48a8(
            CONCAT22(i32_var6, local_c),
            CONCAT22(local_4, local_6),
            local_bx_6.field_0x4,
        );
        i32_var6 = local_bx_6.field_0x4;
        u_var4 = -i32_var6;
        b_var7 = CARRY2(local_6, u_var4);
        local_6 = local_6 + u_var4;
        local_4 = local_4 + (b_var7 - (local_bx_6.field_0x6 + (i32_var6 != 0))) * 0x100;
        local_c = u_var3;
        _local_a = _local_a + -1;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_52fc(param_1: *mut Struct195) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let lVar4: u32;
    let mut local_DX_15: u16;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let local_bx_5: *mut Struct195;
    let mut local_es_5: u16;
    let mut u_var7: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let temp_87f1a411929: *mut u32;

    local_es_5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    u_var7 = process_struct_1008_4772(local_bx_5.field_0xc);
    local_DX_15 = (u_var7 >> 0x10);
    i32_var6 = u_var7;
    i_var5 = (i32_var6 + 4);
    u_var3 = i_var5 - 1;
    i_var5 = (i32_var6 + 6) - (i_var5 == 0);
    lVar4 = (i32_var6 + 8) + -1;
    u_var2 = param_1.field_0x0;
    temp_87f1a411929 = &local_bx_5.field_0x4;
    i32_var6 = (u_var2 >> 0xf) + (&local_bx_5.field_0x4 + 2) + CARRY2(u_var2, temp_87f1a411929);
    if ((i_var5 <= i32_var6) && (i_var5 < i32_var6 || (u_var3 < u_var2 + temp_87f1a411929))) {
        &local_bx_5.field_0x4 = u_var3 - u_var2;
        (&local_bx_5.field_0x4 + 2) = (i_var5 - (u_var2 >> 0xf)) - (u_var3 < u_var2);
    }
    u_var2 = local_bx_5.field_0x2;
    pu_var1 = &local_bx_5.field_0x8;
    i_var5 = (u_var2 >> 0xf) + (&local_bx_5.field_0x8 + 2) + CARRY2(u_var2, pu_var1);
    local_e._2_2_ = (lVar4 >> 0x10);
    if ((local_e._2_2_ <= i_var5)
        && ((
            local_e._0_2_ = lVar4,
            local_e._2_2_ < i_var5 || (local_e < u_var2 + pu_var1),
        )))
    {
        &local_bx_5.field_0x8 = local_e - u_var2;
        (&local_bx_5.field_0x8 + 2) = (local_e._2_2_ - (u_var2 >> 0xf)) - (local_e < u_var2);
    }
    return;
}

pub unsafe fn pass1_1008_570e(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_5784(param_1: *mut *mut u8, param_2: u32) {
    param_1 = param_2;
    (param_1 + 4) = 0;
    return;
}

pub unsafe fn pass1_1008_57a4(param_1: u32, param_2: u32) {
    unsafe {
        unsafe { *param_1 = param_2 };
        (param_1 + 4) = 0;
    }
}

pub unsafe fn pass1_1008_57c4(param_1: *mut Struct44) {
    let u_var1: u8;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    param_1.ptr_a_lo = (s__s__s__1050_5bc0 + 4);
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    u_var1 = pass1_1008_5830((param_1 & 0xffff | local_es_4 << 0x10));
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return u_var1;
}

pub unsafe fn pass1_1008_57f0(param_1: u32, param_2: u16) -> libc::c_long {
    let mut b_var1: bool;
    let lVar2: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, local_a), param_1);
    local_c = 0;
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_a));
        if (lVar2 == 0) {
            return 0;
        }
        b_var1 = local_c != param_2;
        local_c = local_c + 1;
        b_var1
    } {}
    return lVar2;
}

pub unsafe fn pass1_1008_5830(param_1: *mut Struct200) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let pu_var4: *mut u32;
    let local_bx_5: *mut Struct200;
    let mut i_var5: i32;
    let mut local_es_5: u16;
    let mut local_es_23: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7e01ca0f901: *mut u32;
    let mut temp_5f0adb14fb: u32;
    // temp_87fa1582f55: *mut *mut u8;
    let temp_8622404347c: *mut u32;

    while (true) {
        local_es_5 = (param_1 >> 0x10);
        local_bx_5 = param_1;
        if (local_bx_5.field_0x4 == 0) {
            break;
        }
        if (local_bx_5.field_0xa != 0) {
            temp_5f0adb14fb = local_bx_5.field_0x4;
            local_es_23 = (temp_5f0adb14fb >> 0x10);
            i_var5 = temp_5f0adb14fb;
            pu_var1 = (i_var5 + 8);
            u_var2 = (i_var5 + 10);
            if ((u_var2 | pu_var1) != 0) {
                unsafe { temp_87fa1582f55 = *pu_var1 };
                (**temp_87fa1582f55)();
            }
        }
        pu_var4 = local_bx_5.field_0x4;
        local_bx_5.field_0x4 = (pu_var4 + 4);
        if (pu_var4 != 0x0) {
            unsafe { ppc_var3 = *pu_var4 };
            (**ppc_var3)();
        }
    }
    local_bx_5.field_0x8 = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_fn_1008_58a6(param_1: u32, param_2: u32) {
    let pi_var1: *mut i32;
    let local_AX_10: *mut Struct201;
    let mut u_var2: i32;
    
    let mut i_var4: i32;
    let mut local_es_102: u16;
    let mut local_es_110: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let pu_var3: *mut u8;

    pu_var3 = _PTR_LOOP_1050_029c;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_029c);
    u_var2 = pu_var3;
    local_6 = pu_var3 & 0xffff | ctx.dx_reg << 0x10;
    if ((ctx.dx_reg | u_var2) == 0) {
        local_6 = 0;
    } else {
        local_6 = ctx.s_1_1050_389a;
        (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var2 + 4) = 0;
        (u_var2 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    if (local_6 == 0) {
        return;
    }
    local_es_102 = (local_6 >> 0x10);
    (local_6 + 8) = param_2;
    local_es_110 = (param_1 >> 0x10);
    i_var4 = param_1;
    (local_6 + 4) = (i_var4 + 4);
    (i_var4 + 4) = local_6;
    pi_var1 = (i_var4 + 8);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_593c(param_1: *mut u32, param_2: u32) {
    let pi_var1: *mut i32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let pu_var4: *mut u8;
    
    let mut i_var5: i32;
    let mut local_es_4: u16;
    let mut local_es_150: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let mut temp_5f74faf803: u32;

    local_es_4 = (param_1 >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 8) == 0) {
        let param_1_val = unsafe { *param_1 };
        ppc_var2 = (param_1_val + 4);
        ppc_var2();
        return;
    }
    pu_var4 = _PTR_LOOP_1050_029c;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_029c);
    u_var3 = pu_var4;
    local_6 = pu_var4 & 0xffff | ctx.dx_reg << 0x10;
    if ((ctx.dx_reg | u_var3) == 0) {
        local_6 = 0;
    } else {
        local_6 = ctx.s_1_1050_389a;
        (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var3 + 4) = 0;
        (u_var3 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    if (local_6 == 0) {
        return;
    }
    (local_6 + 8) = param_2;
    while {
        param_1 = (param_1 + 4);
        local_es_150 = (param_1 >> 0x10);
        (param_1 + 4) != 0
    } {}
    todo!();
    //(param_1 + 4) = local_6;
    pi_var1 = (i_var5 + 8);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_59f4(param_1: u32, param_2: u32) {
    let pi_var1: *mut i32;
    let pu_var2: *mut u32;
    let mut u_var3: i32;
    let mut local_AX_110: u16;
    let mut local_DX_110: u16;
    let mut i_var4: i32;
    let mut local_es_12: u16;
    let mut local_es_24: u16;
    let mut local_es_42: u16;
    let mut local_es_110: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let temp_8623d993e9d: *mut u32;
    let temp_79fa913f4bf: *mut u32;
    let mut temp_79f326fe867: u32;
    let fn_ptr_1: fn();

    local_6 = 0;
    local_es_12 = (param_1 >> 0x10);
    temp_79f326fe867 = local_6;
    temp_79fa913f4bf = param_1;
    while {
        local_6 = temp_79f326fe867;
        local_es_24 = (temp_79fa913f4bf >> 0x10);
        i_var4 = temp_79fa913f4bf;
        temp_79fa913f4bf = (i_var4 + 4);
        local_a._0_2_ = temp_79fa913f4bf;
        local_es_42 = (temp_79fa913f4bf >> 0x10);
        if (((i_var4 + 6) | local_a) == 0) {
            break;
        }
        temp_79f326fe867 = temp_79fa913f4bf;
        (local_a + 8) != param_2
    } {}
    if (temp_79fa913f4bf != 0x0) {
        if (local_6 == 0) {
            local_AX_110 = (local_a + 4);
            local_DX_110 = (local_a + 6);
            local_6 = param_1;
        } else {
            local_AX_110 = (local_a + 4);
            local_DX_110 = (local_a + 6);
        }
        local_es_110 = (local_6 >> 0x10);
        (local_6 + 4) = local_AX_110;
        (local_6 + 6) = local_DX_110;
        if ((param_1 + 10) != 0) {
            pu_var2 = (local_a + 8);
            u_var3 = (local_a + 10);
            if ((u_var3 | pu_var2) != 0) {
                unsafe { fn_ptr_1 = *pu_var2 };
                (**fn_ptr_1)();
            }
        }
        if (temp_79fa913f4bf != 0x0) {
            unsafe { fn_ptr_1 = *temp_79fa913f4bf };
            (**fn_ptr_1)();
        }
        pi_var1 = (param_1 + 8);
        unsafe { *pi_var1 = *pi_var1 + -1 };
        return;
    }
    return;
}

pub unsafe fn pass1_fn_1008_5ab8(param_1: u32) {
    let pi_var1: *mut i32;
    let pu_var2: *mut u32;
    let mut i_var3: i32;
    let mut local_es_4: u16;
    let mut u_var4: i32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7e05aed193b: *mut u32;
    let fn_ptr_1: fn();

    local_es_4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 4) == 0) {
        return;
    }
    pu_var2 = (i_var3 + 4);
    u_var4 = (pu_var2 >> 0x10);
    (i_var3 + 4) = (pu_var2 + 4);
    if ((u_var4 | pu_var2) != 0) {
        unsafe { fn_ptr_1 = *pu_var2 };
        (**fn_ptr_1)();
    }
    pi_var1 = (i_var3 + 8);
    unsafe { *pi_var1 = *pi_var1 + -1 };
    return;
}

pub unsafe fn pass1_1008_5b12(param_1: &mut Address<Struct306>) -> u32 {
    // let mut i_var1: u32;
    let mut i_var2: u32;
    // let mut local_es_23: u16;
    // let mut local_es_34: u16;
    let mut temp_5fb237ca1c: u32;

    if (param_1.field_0x0 != 0) && ((param_1.field_0x0 + 8) != 0) {
        local_es_23 = (param_1 >> 0x10);
        // i_var1 = param_1;
        if (param_1._type.field_0x4) == 0 {
            ctx.es_reg = (param_1.field_0x0 >> 0x10);
            i_var2 = param_1.field_0x0;
        } else {
            temp_5fb237ca1c = param_1._type.field_0x4;
            // local_es_34 = (temp_5fb237ca1c >> 0x10);
            i_var2 = temp_5fb237ca1c;
        }
        param_1._type.field_0x4 = (i_var2 + 4);
        if param_1._type.field_0x4 != 0 {
            return 0;
        }
    }
    return 0;
}

pub unsafe fn pass1_fn_1008_5b9a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_57c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_fn_1008_60e8(param_1: *mut char) -> uint {
    let mut u_var1: i32;
    let in_dx: *mut Struct199;
    let mut local_8: u16;
    let mut local_6: u16;

    if (param_1 != 0x0) {
        u_var1 = get_string_index_1000_3da4(param_1);
        u_var1 = u_var1 + 1;
        process_struct_1000_179c(u_var1, in_dx);
        if ((in_dx | u_var1) != 0) {
            copy_string_1000_3d3e(CONCAT22(in_dx, u_var1), param_1);
            return u_var1;
        }
    }
    return 0;
}

pub unsafe fn pass1_fn_1008_612e(param_1: u16, param_2: i32) {
    let mut in_ax: i32;
    let mut u_var1: i32;
    let lVar2: u32;
    let mut i_var3: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_fn_1000_4d24();
    u_var1 = (param_2 - param_1) + 1;
    if ((u_var1 >> 0xf | u_var1) == 0) {
        return;
    }
    local_10 = 1;
    local_12 = param_1;
    loop {
        if (param_2 < local_12) {
            return;
        }
        lVar2 = local_10 * (0x7fff / u_var1);
        i_var3 = (lVar2 >> 0x10);
        if (in_ax >> 0xf <= i_var3) {
            if (in_ax >> 0xf < i_var3) {
                return;
            }
            if (in_ax <= lVar2) {
                return;
            }
        }
        local_12 = local_12 + 1;
        local_10 = local_10 + 1;
    }
}

pub unsafe fn pass1_1008_6330(param_1: *mut u16, param_2: u8) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    u_var2 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x380a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_6562(
    param_1: *mut *mut Struct104,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: *mut u16,
) -> i32 {
    let mut i_var1: i32;
    let mut local_DX_57: u16;
    let mut u_var2: u16;
    let mut local_DX_103: u16;
    let mut local_DX_129: u16;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f1d1f0836: u32;

    let param_1_val = unsafe { *param_1 };
    if (param_1_val == 0x0) {
        return;
    }
    process_struct_1000_179c(0x1e, param_5);
    if ((param_5 | param_4) == 0) {
        param_4 = 0;
        u_var2 = 0;
    } else {
        temp_5f1d1f0836 = (param_1 + 4);
        pass1_1008_405c(
            param_4,
            param_5,
            temp_5f1d1f0836,
            (temp_5f1d1f0836 >> 0x10),
            param_2,
        );
        u_var2 = local_DX_57;
    }
    _local_6 = CONCAT22(u_var2, param_4);
    local_8 = 0;
    while (param_2 = param_2 & 0xffff0000 | (param_2 - 1), param_2 != 0) {
        i_var1 = param_3 + 1;
        process_struct_1008_4544(param_1_val);
        u_var2 = local_8 + 1;
        u_var3 = local_DX_103;
        process_struct_1008_4544(_local_6);
        pass1_fn_1000_48a8(
            CONCAT22(local_DX_129, local_8),
            CONCAT22(u_var3, param_3),
            param_2._2_2_,
        );
        param_3 = i_var1;
        local_8 = u_var2;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_6604(param_1: *mut u16, param_2: u16, param_3: u16) {
    let mut u_var1: u32;

    let mut i_var2: i32;
    let mut u_var3: u32;
    let mut local_DX__1: u16;
    let mut local_DX_105: u16;
    let mut local_es_15: u16;
    let mut local_es_177: u16;
    let paVar4: *mut Struct102;

    paVar4 = param_1;
    set_struct_1008_4016(paVar4);
    local_es_15 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x685a };
    paVar4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    alloc_mem_1000_0a48(1, 0x28, ctx.__g_Struct94_ptr_1);
    &paVar4.field_0x10 = ctx.ax_reg;
    (&paVar4.field_0x10 + 2) = local_DX__1;
    i_var2 = param_3 * 8 + 0x1f;
    i_var2 = ((i_var2 + (i_var2 >> 0xf & 0x1f)) >> 5) << 2;
    &paVar4.field_0x18 = i_var2;
    (&paVar4.field_0x18 + 2) = i_var2 >> 0xf;
    local_DX_105 = ((i_var2 * param_2) >> 0x20);
    u_var3 = (i_var2 * param_2);
    alloc_mem_1000_0a48(1, u_var3, ctx.__g_Struct94_ptr_1);
    &paVar4.field_0x6 = u_var3;
    (&paVar4.field_0x6 + 2) = local_DX_105;
    &paVar4.field_0x14 = &paVar4.field_0x6;
    (&paVar4.field_0x14 + 2) = local_DX_105;
    paVar4.field_0x10 = 0x28;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 4) = param_3;
    u_var1 = paVar4.field_0x10;
    local_es_177 = (u_var1 >> 0x10);
    i_var2 = u_var1;
    (i_var2 + 8) = param_2;
    (i_var2 + 10) = param_2 >> 0xf;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0xc) = 1;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0xe) = 8;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x10) = 0;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x14) = paVar4.field_0x18 * param_2;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x20) = 0x100;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x24) = 0x100;
    return;
}

pub unsafe fn pass1_1008_6732(param_1: *mut Struct182) {
    let mut i_var1: i32;
    let mut local_es_3: u16;
    let mut temp_5f5866abc5: u32;

    local_es_3 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1 = 0x685a;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((i_var1 + 0x10) != 0) {
        temp_5f5866abc5 = (i_var1 + 0x10);
        error_check_1000_0dc6(temp_5f5866abc5, (temp_5f5866abc5 >> 0x10));
    }
    (i_var1 + 0x10) = 0;
    process_struct_1008_41bc(param_1);
    return;
}

pub unsafe fn pass1_1008_6834(param_1: u32, param_2: u8) {
    pass1_1008_6732(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_687a(param_1: *mut Struct65, param_2: u32) {
    let mut i_var1: i32;
    let mut local_es_17: u16;

    set_struct_1008_9584(param_1, param_2);
    local_es_17 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xcc) = 0;
    (i_var1 + 0xce) = 0;
    process_struct_1008_574a((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    param_1.ptr_a_lo = 0x6bfc;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var1 + 0xdc) = 0;
    return;
}

pub unsafe fn pass1_1008_68ea(
    param_1: i32,
    param_2: u16,
    param_2_00: *mut u32,
    param_4: u16,
    param_5: u16,
    param_6: i32,
) -> i32 {
    let pp_var1: fn();
    let mut local_6: u32;
    // temp_87f4899415d: *mut *mut u8;

    if (param_6 == 0) {
        if ((param_1 + 0xce) != CONCAT22(param_4, param_2_00)) {
            if ((param_1 + 0xce) != 0) {
                temp_87f4899415d = ((param_1 + 0xce) + 0x10);
                (**temp_87f4899415d)();
            }
            (param_1 + 0xce) = CONCAT22(param_4, param_2_00);
            let param_2_val = unsafe { *param_2_00 };
            pp_var1 = (param_2_val + 0x10);
            (**pp_var1)();
            pp_var1 = ((param_1 + 0xce) + 0xc);
            (**pp_var1)();
            return;
        }
    } else {
        pass1_1008_3e0e(CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)));
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_6978(param_1: *mut Struct675, param_2: u16, param_3: u32) {
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    // func_ptr_1: *mut *mut u8;

    process_struct_1000_179c(10, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) == 0) {
        local_6 = 0;
    } else {
        if (param_2 == 0) {
            param_2 = (param_1 + 0xcc);
        }
        unsafe { *_local_a = ctx.s_1_1050_389a };
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
        (in_ax + 4) = param_3;
        (in_ax + 8) = param_2;
        unsafe { *_local_a = 0x6c8c };
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
        local_6 = _local_a;
    }
    func_ptr_1 = ((param_1 + 0xd2) + 4);
    (**func_ptr_1)(0x1000, (param_1 + 0xd2), param_1._2_2_, local_6);
    return;
}

pub unsafe fn pass1_1008_6a04(param_1: u32) {
    let local_AX_30: *mut u8;
    

    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];
    // temp_87f48b94a86: *mut *mut u8;

    pass1_1008_57a4(
        CONCAT22(ctx.stack_seg_reg, local_a),
        param_1 & 0xffff0000 | (param_1 + 0xd2),
    );
    while (true) {
        local_AX_30 = local_a;
        pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_30));
        if ((ctx.dx_reg | local_AX_30) == 0) {
            break;
        }
        temp_87f48b94a86 = ((local_AX_30 + 4) + 0xc);
        (**temp_87f48b94a86)();
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_6a4a(param_1: u32, param_2: i32, param_3: u16, param_3_00: i32) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_AX_38: *mut u8;
    
    

    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    // temp_87fb77c20c3: *mut *mut u8;

    if (param_3_00 == 2) {
        i_var2 = param_1;
        pass1_1008_57a4(
            CONCAT22(ctx.stack_seg_reg, local_e),
            param_1 & 0xffff0000 | (i_var2 + 0xd2),
        );
        while {
            local_AX_38 = local_e;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_38));
            _local_6 = CONCAT22(ctx.dx_reg, local_AX_38);
            if ((ctx.dx_reg | local_AX_38) == 0) {
                break;
            }
            (local_AX_38 + 8) != param_2
        } {}
        if (_local_6 != 0) {
            temp_87fb77c20c3 = ((i_var2 + 0xd2) + 0xc);
            (**temp_87fb77c20c3)();
            local_a = 0;
            local_6 = local_e;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_6));
            if ((ctx.dx_reg | local_6) != 0) {
                pp_var1 = ((local_6 + 4) + 0x10);
                local_4 = ctx.dx_reg;
                (**pp_var1)();
                (i_var2 + 0xce) = (local_6 + 4);
                return;
            }
            (i_var2 + 0xce) = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1008_6b02(param_1: u32) {
    let mut i_var1: i32;
    let mut local_es_3: u16;
    // fn_ptr_1: *mut *mut u8;

    local_es_3 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (((i_var1 + 0xd0) | (i_var1 + 0xce)) != 0) {
        fn_ptr_1 = ((i_var1 + 0xce) + 0x6c);
        (**fn_ptr_1)();
    }
    return;
}

pub unsafe fn pass1_1008_6b5a(param_1: *mut u16, param_2: u8) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let temp_862bb3f10cd: *mut u32;
    // fn_ptr_1: *mut *mut u8;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    unsafe { *param_1 = 0x6c8c };
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = (i_var3 + 4);
    u_var2 = (i_var3 + 6);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { fn_ptr_1 = *pu_var1 };
        (**fn_ptr_1)();
    }
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_6bb4(param_1: *mut Struct203, param_2: u8) {
    let local_AX_8: *mut Struct203;
    let mut u_var1: u16;

    local_AX_8 = param_1;
    local_AX_8 = local_AX_8 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(local_AX_8)));
    u_var1 = (param_1 >> 0x10);
    param_1 = 0x380a;
    local_AX_8.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = ctx.s_1_1050_389a;
    local_AX_8.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_6c30(
    param_1: u8,
    param_2: i32,
    param_3: u8,
    param_4: i32,
    param_5: bool,
    param_6: bool,
    param_7: bool,
    param_8: bool,
    param_9: bool,
) -> i32 {
    let pc_var1: *mut libc::c_char;
    let pu8_var2: *mut u8;
    let mut u_var3: i32;
    let mut b_var4: bool;
    let mut b_var5: bool;
    let mut b_var6: u8;
    let mut b_var7: u8;
    let mut b_var8: u8;
    let mut b_var9: u8;
    let mut b_var10: u8;
    let mut i_var11: i32;
    let mut b_var12: u8;
    u16 * *local_SP;
    u16 * *unaff_bp;
    let mut unaff_si: i32;
    let mut unaff_DI: i32;

    let mut b_var13: bool;
    let in_stack_0000f71d: *mut Struct1;
    let paStack2265: *mut Struct1;
    let mut cStack3: u8;
    let puStack2: *mut u16;
    let temp_179f295966e9: *mut u16;
    let mut byte3: u8;

    loop {
        local_SP = &puStack2;
        local_SP = &puStack2;
        byte3 = '\x0f';
        temp_179f295966e9 = unaff_bp;
        while {
            temp_179f295966e9 = temp_179f295966e9 + -1;
            local_SP = local_SP + -1;
            unsafe { *local_SP = *temp_179f295966e9 };
            byte3 = byte3 + -1;
            '\0' < byte3
        } {}
        i_var11 = param_1;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var4 = 9 < (unaff_si & 0xf);
        b_var6 = b_var4 | param_7;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var5 = 9 < (unaff_si + b_var6 * '\x06' & 0xf);
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        let pu8_var2_val = unsafe { *pu8_var2 };
        b_var7 = ((POPCOUNT(pu8_var2_val) & 1) == 0) * 0x4;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var6 = 9 < b_var7 | b_var5 | b_var6;
        b_var8 = b_var7 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var6 = 9 < b_var8 | b_var6;
        b_var9 = b_var8 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_DI);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        unaff_si = unaff_DI + -1;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var12 = param_3 - 1;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | b_var12 };
        b_var6 = 9 < b_var9 | b_var6;
        b_var10 = b_var9 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | b_var12 };
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | b_var12 };
        param_3 = param_3 - 2;
        b_var12 = (param_2 >> 8);
        b_var13 = CARRY1(u8_1050_086b, b_var12);
        u8_1050_086b = u8_1050_086b + b_var12;
        pc_var1 = &cStack3 + unaff_DI;
        unsafe { *pc_var1 = *pc_var1 + b_var10 + b_var13 };
        i_var11 = (param_4 + unaff_si);
        puStack2 = unaff_bp;
        let pc_var1_val = unsafe { *pc_var1 };
        if ((POPCOUNT(pc_var1_val) & 1) == 0) {}
        // goto code_r0x10086ca6;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        let pu8_var2_val = unsafe { *pu8_var2 };
        u_var3 = (param_9 & 1) * 0x4000
            | (param_8 & 1) * 0x200
            | (param_5 & 1) * 0x100
            | (pu8_var2_val < '\0') * 0x80
            | (pu8_var2_val == 0) * 0x40
            | ((9 < b_var9) | (9 < b_var8) | (9 < b_var7) | b_var5 | b_var4 | param_7 & 1) * 0x10
            | ((POPCOUNT(pu8_var2_val) & 1) == 0) * 4;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        paStack2265 = CONCAT22(u_var3, unaff_si);
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        pu8_var2 = (unaff_DI + 0x1007);
        unsafe { *pu8_var2 = *pu8_var2 ^ param_4 };
        unsafe { b_var7 = *pu8_var2 };
        *(unaff_DI + 0x1007) = param_4;
        param_4 = (unaff_DI + 0x1007);
        param_2 = i_var11 * 0x10 + -1;
        if (param_2 == 0 || b_var7 == 0) {
            break;
        }
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var10 = b_var10 - 1;
        b_var6 = 9 < (b_var10 & 0xf) | b_var6;
        b_var7 = b_var10 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        param_7 = (9 < b_var7 | b_var6);
        param_1 = b_var7 + param_7 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        unaff_bp = local_SP;
    }
    pu8_var2 = (param_4 + unaff_si);
    unsafe { *pu8_var2 = *pu8_var2 | param_3 };
    zero_list_1008_3e38(paStack2265);
    in_stack_0000f71d = CONCAT22(u_var3, unaff_DI + 5);
    // code_r0x10086ca6:
    zero_list_1008_3e38(in_stack_0000f71d);
    return;
}

pub unsafe fn pass1_1008_6cec(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u32,
) {
    pass1_1008_3e76(param_1, param_4, param_5, (param_5 >> 0x10));
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (param_1 + 6)),
        param_2,
        param_3,
        (param_3 >> 0x10),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_6d8a(param_1: u32, param_2: u32) {
    let mut local_AX_49: u16;
    let mut local_DX__1: u16;
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    param_1 = 0;
    (param_1 + 4) = 0xffff;
    PTR_LOOP_1050_0312 = &PTR_DAT_0005_0000_1050_0004;
    string_fn_1000_3f9c(
        s__1050_65a0,
        &ctx.g_alloc_addr_1050_1050,
        _PTR_s_SC_03d_1050_0314_1050_031c,
        (_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
        &PTR_DAT_0005_0000_1050_0004,
    );
    local_AX_49 = pass1_fn_1008_60e8(param_2);
    param_1 = local_AX_49;
    (param_1 + 2) = local_DX__1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_6eee(param_1: u16, param_2: u16, param_1_00: u32) {
    let local_AX_45: *mut u8;
    let b_var1: bool;
    let pu_var2: *mut u8;

    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { local_6 = *ctx._g_bool_1050_5748 };
    local_a = local_6;
    pass1_1020_a43e(CONCAT22(ctx.stack_seg_reg, local_e));
    local_AX_45 = write_to_file_1028_d7a0(local_a, (local_a >> 0x10), param_1_00);
    if (local_AX_45 != 0x0) {
        b_var1 = write_to_file_1030_5c1a(_PTR_LOOP_1050_5736, param_1_00);
        if (b_var1 != 0) {
            write_to_file_1028_dce2(ctx._PTR_LOOP_1050_65e2, param_1_00);
            if (b_var1 != 0) {
                write_to_file_1038_7b20(_PTR_LOOP_1050_5a64, param_1_00);
                if (b_var1 != 0) {
                    pu_var2 = local_e;
                    call_write_to_file_1020_a644(pu_var2, ctx.stack_seg_reg, param_1_00);
                    if (pu_var2 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_6f7a(param_1: u16, param_2: u16, param_1_00: u32) {
    let local_AX_45: *mut u8;
    let b_var1: bool;
    let pu_var2: *mut u8;

    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { local_6 = *ctx._g_bool_1050_5748 };
    local_a = local_6;
    pass1_1020_a43e(CONCAT22(ctx.stack_seg_reg, local_e));
    local_AX_45 = read_file_1028_d7ba(local_a, (local_a >> 0x10), param_1_00);
    if (local_AX_45 != 0x0) {
        b_var1 = read_file_1030_5c52(_PTR_LOOP_1050_5736, param_1_00);
        if (b_var1 != 0) {
            read_file_1028_def2(ctx._PTR_LOOP_1050_65e2, param_1_00);
            if (b_var1 != 0) {
                read_from_file_1038_7c02(_PTR_LOOP_1050_5a64, param_1_00);
                if (b_var1 != 0) {
                    pu_var2 = local_e;
                    call_read_file_1020_a65e(pu_var2, ctx.stack_seg_reg, param_1_00);
                    if (pu_var2 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_8
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_7006(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_DXAX_37: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        if (PTR_LOOP_1050_0334 <= local_4) {
            return 1;
        }
        local_DXAX_37 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(local_8, (local_4 * 2 + 800)),
        );
        unsafe { pp_var1 = (*local_DXAX_37 + 8) };
        i_var2 = (**pp_var1)(0x1010, local_DXAX_37, param_1_00);
        local_8 = local_DXAX_37;
        if (i_var2 == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return i_var2;
}

// WARNING: Variable defined which should be unmapped: local_8
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_7056(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_DXAX_37: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        if (PTR_LOOP_1050_0334 <= local_4) {
            return 1;
        }
        local_DXAX_37 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(local_8, (local_4 * 2 + 800)),
        );
        unsafe { pp_var1 = (*local_DXAX_37 + 0xc) };
        i_var2 = (**pp_var1)(0x1010, local_DXAX_37, param_1_00);
        local_8 = local_DXAX_37;
        if (i_var2 == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return i_var2;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_766e(param_1: u32, param_2: u32) {
    let pu_var1: *mut u32;
    let struct_a: *mut Struct199;
    let paVar2: *mut Struct199;
    let mut local_DX_74: u16;
    let mut u_var3: u16;

    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    param_2 = 0;
    local_6 = 0;
    pu_var1 = &local_6;
    read_file_1008_76e4(param_1, pu_var1, ctx.stack_seg_reg);
    if (pu_var1 != 0x0) {
        if (local_6 != 0) {
            paVar2 = struct_a;
            process_struct_1000_179c(0xc, struct_a);
            if ((paVar2 | pu_var1) == 0) {
                pu_var1 = 0x0;
                u_var3 = 0;
            } else {
                pass1_1010_8ef2(CONCAT22(paVar2, pu_var1));
                u_var3 = local_DX_74;
            }
            param_2 = pu_var1;
            (param_2 + 2) = u_var3;
            pass1_1010_905e(param_2, local_6);
        }
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_7e98(param_1: *mut u16, param_2: u8) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x380a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Instruction at (ram,0x10087f7b) overlaps instruction at (ram,0x10087f79)
//

pub unsafe fn pass1_1008_7f06(
    param_1: u8,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: bool,
    param_6: u8,
    param_7: u8,
    param_8: u8,
    param_9: u8,
) {
    let pb_var1: *mut u8;
    let mut b_var2: bool;
    let mut b_var3: bool;
    let mut b_var4: bool;
    let mut b_var5: bool;
    let mut b_var6: u8;
    let mut b_var7: u8;
    let mut u_var8: u32;
    let paVar9: *mut Struct65;
    let mut b_var10: u8;
    u16 * *ppu_var11;
    let pu_var12: *mut u8;
    let mut b_var13: u8;
    let mut b_var14: u8;
    let mut b_var15: u8;
    let HVar16: HGDIOBJ16;
    let HVar17: HCURSOR16;
    let mut b_var18: u8;
    let mut b_var19: u8;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut i_var22: i32;
    u16 * *local_SP;
    u16 * *unaff_bp;
    let mut unaff_si: i32;
    let local_SI_28: *mut u8;
    let mut unaff_DI: i32;
    let mut local_es_112: u16;
    let mut local_es_174: u16;
    let local_es_214: *mut u8;

    let mut u_var23: i32;
    let mut iStack2251: i32;
    u16 * *ppuStack2249;
    // local_8c7: *mut *mut u8;
    let mut uStack2245: u16;
    let mut uStack2243: u16;
    let mut uStack2241: u16;
    let mut uStack2239: i32;
    let puStack2237: *mut u8;
    let mut auStack2235: [u8; 2201];
    u16 * *ppuStack34;
    let puStack2: *mut u16;
    let temp_179f862c9b4a: *mut u16;
    let temp_5f36b4e76f: *mut u16;
    let mut temp_5f3d48f035: u32;
    let mut temp_5fd4f68048: u32;
    // temp_32379f6118dbc5: *mut *mut u8;
    let temp_36379f6118dbc5: *mut u8;
    let mut char_8: u8;

    while (true) {
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        char_8 = '\x0f';
        temp_179f862c9b4a = unaff_bp;
        while {
            temp_179f862c9b4a = temp_179f862c9b4a + -1;
            local_SP = local_SP + -1;
            unsafe { *local_SP = *temp_179f862c9b4a };
            char_8 = char_8 + -1;
            '\0' < char_8
        } {}
        iStack2251 = param_1;
        pb_var1 = (param_4 + iStack2251);
        b_var18 = param_3;
        unsafe { *b_var1 = *pb_var1 | b_var18 };
        puStack2237 = auStack2235;
        b_var2 = 9 < (unaff_si & 0xf);
        b_var13 = b_var2 | param_6;
        b_var10 = unaff_si + b_var13 * '\x06' & 0xf;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        local_8c7 = &puStack2237;
        b_var3 = 9 < b_var10;
        b_var6 = b_var3 | b_var13;
        uStack2239 = CONCAT11(
            (unaff_si >> 8) + b_var13 + b_var6,
            b_var10 + b_var6 * '\x06',
        ) & 0xff0f;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        unsafe { b_var13 = *pb_var1 };
        b_var10 = ((POPCOUNT(b_var13) & 1) == 0) * 0x4;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        b_var4 = 9 < b_var10;
        b_var6 = b_var4 | b_var6;
        b_var10 = b_var10 + b_var6 * '\x06' & 0xf;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        b_var5 = 9 < b_var10;
        b_var7 = b_var5 | b_var6;
        b_var14 = b_var10 + b_var7 * '\x06' & 0xf;
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        local_SI_28 = (unaff_DI + -1);
        u_var23 = (param_9 & 1) * 0x4000
            | SBORROW2(unaff_DI, 1) * 0x800
            | (param_8 & 1) * 0x200
            | (param_7 & 1) * 0x100
            | (local_SI_28 < 0) * 0x80
            | (local_SI_28 == 0x0) * 0x40
            | (b_var5 | b_var4 | b_var3 | b_var2 | param_6 & 1) * 0x10
            | ((POPCOUNT(local_SI_28 & 0xff) & 1) == 0) * 4;
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        u_var20 = param_3 - 1;
        pb_var1 = local_SI_28 + param_4;
        b_var19 = u_var20;
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        b_var18 = 9 < b_var14 | b_var7;
        b_var15 = b_var14 + b_var18 * '\x06' & 0xf;
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        let local_si_28_val = unsafe { *local_SI_28 };
        out(local_si_28_val, u_var20);
        b_var10 = 9 < b_var15 | b_var18;
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        ppu_var11 = local_SP;
        temp_32379f6118dbc5 = &puStack2237;
        uStack2245 = param_4;
        uStack2243 = param_3;
        uStack2241 = param_2;
        pu_var12 = auStack2235;
        temp_36379f6118dbc5 = local_SP;
        puStack2 = unaff_bp;
        let pb_var1_val = unsafe { *pb_var1 };
        if (pb_var1_val == 0) {
            // goto LAB_1008_7f73;
        }
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        let pb_var1_val = unsafe { *pb_var1 };
        if ((POPCOUNT(pb_var1_val) & 1) == 0) {
            break;
        }
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        unsafe { b_var13 = *pb_var1 };
        unsafe { b_var6 = *pb_var1 };
        unsafe { b_var7 = *pb_var1 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (&ctx.PTR_LOOP_1050_1008 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 ^ param_4 };
        unsafe { b_var18 = *pb_var1 };
        *(&ctx.PTR_LOOP_1050_1008 + unaff_DI) = param_4;
        u_var21 = *(&ctx.PTR_LOOP_1050_1008 + unaff_DI);
        if (param_2 - 1 == 0 || b_var18 == 0) {
            pb_var1 = (u_var21 + unaff_DI);
            unsafe { *pb_var1 = *pb_var1 | b_var19 };
            pass1_1008_687a(
                CONCAT22(
                    unaff_DI,
                    (param_9 & 1) * 0x4000
                        | (param_8 & 1) * 0x200
                        | (param_7 & 1) * 0x100
                        | (b_var13 < '\0') * 0x80
                        | (b_var6 == 0) * 0x40
                        | ((9 < b_var15)
                            | (9 < b_var14)
                            | b_var5
                            | b_var4
                            | b_var3
                            | b_var2
                            | param_6 & 1)
                            * 0x10
                        | ((POPCOUNT(b_var7) & 1) == 0) * 4,
                ),
                CONCAT22(0x37, u_var20),
            );
            local_SP = &stack0xf721;
            ppu_var11 = local_SP;
            temp_32379f6118dbc5 = local_8c7;
            pu_var12 = puStack2237;
            temp_36379f6118dbc5 = local_SP;
            // LAB_1008_7f73:
            local_SP = temp_36379f6118dbc5;
            puStack2237 = pu_var12;
            local_8c7 = temp_32379f6118dbc5;
            local_SP = ppu_var11;
            temp_5f36b4e76f = (local_SP + 6);
            local_es_112 = (temp_5f36b4e76f >> 0x10);
            i_var22 = temp_5f36b4e76f;
            (i_var22 + 0xde) = (local_SP + 10);
            unsafe { *temp_5f36b4e76f = 0x8042 };
            (i_var22 + 2) = &ctx.PTR_LOOP_1050_1008;
            copy_string_1000_3d3e(
                (temp_5f36b4e76f & 0xffff0000 | (i_var22 + 0x5b)),
                s_SOLChildPar_1050_0358,
            );
            HVar16 = GetStockObject16(5);
            temp_5f3d48f035 = (local_SP + 6);
            (temp_5f3d48f035 + 0xc6) = HVar16;
            HVar17 = LoadCursor16(0x7f00, 0);
            u_var8 = (local_SP + 6);
            local_es_174 = (u_var8 >> 0x10);
            i_var22 = u_var8;
            (i_var22 + 0xc4) = HVar17;
            (i_var22 + 200) = (s_572_bmp_1050_2007 + 1);
            (i_var22 + 0xac) = 0x44000000;
            u_var8 = (local_SP + 0xc);
            temp_5fd4f68048 = (local_SP + 6);
            (temp_5fd4f68048 + 0xbc) = (u_var8 + 8);
            paVar9 = (local_SP + 6);
            local_es_214 = (paVar9 >> 0x10);
            (paVar9 + 0xca) = (paVar9 + 0xde);
            reg_class_1008_96d2(paVar9, u_var23);
            return CONCAT22((local_SP + 8), (local_SP + 6));
        }
        pb_var1 = (u_var21 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        b_var13 = (b_var15 + b_var10 * '\x06' & 0xf) - 1;
        b_var10 = 9 < (b_var13 & 0xf) | b_var10;
        b_var13 = b_var13 + b_var10 * '\x06' & 0xf;
        pb_var1 = (u_var21 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        param_6 = 9 < b_var13 | b_var10;
        param_1 = b_var13 + param_6 * '\x06' & 0xf;
        pb_var1 = (u_var21 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        param_2 = param_2 - 1;
        param_3 = u_var20;
        param_4 = u_var21;
        unaff_bp = local_SP;
        unaff_si = unaff_DI;
    }
    cRam105007c7 = cRam105007c7
        + ((param_9 & 1) * '@'
            | (param_8 & 1) * 0x2
            | param_7 & 1
            | ((b_var13 < '\0') * 0x80 >> 8))
        + b_var6
        + b_var7
        + b_var18
        + b_var10;
    bRam105047c7 = bRam105047c7 & 2;
    pb_var1 = (unaff_DI + param_4);
    unsafe { *pb_var1 = *pb_var1 | b_var19 + 1 };
    local_SP = local_SP;
    ppu_var11 = local_SP;
    temp_32379f6118dbc5 = &puStack2237;
    pu_var12 = auStack2235;
    temp_36379f6118dbc5 = local_SP;
    // goto LAB_1008_7f73;
}

pub unsafe fn pass1_1008_7ffa(param_1: *mut u16, param_2: u8) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    u_var2 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x380a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_8aa2(param_1: *mut u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let mut i_var5: i32;
    let mut local_es_4: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let temp_8628f43bde0: *mut u32;
    let mut temp_5fc30efdd1: u32;

    local_es_4 = (param_1 >> 0x10);
    i_var5 = param_1;
    unsafe { *param_1 = 0x8e9a };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
    temp_5fc30efdd1 = (i_var5 + 4);
    if ((temp_5fc30efdd1 + 0x1c) != 0) {
        pu_var1 = (i_var5 + 4);
        u_var2 = (i_var5 + 6);
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppcVar4 = *pu_var1 };
            (**ppcVar4)();
        }
    }
    u_var2 = (i_var5 + 0x3a);
    u_var3 = (i_var5 + 0x3c);
    _local_10 = CONCAT22(u_var3, u_var2);
    if ((u_var3 | u_var2) != 0) {
        pass1_1008_5118(CONCAT22(u_var3, u_var2));
        error_check_1000_17ce(_local_10);
    }
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_8b20(param_1: *mut Struct206) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let mut local_AX_71: u16;
    let mut local_DX_71: u16;
    let local_struct_1: *mut Struct206;
    let mut local_es_6: u16;

    let mut local_1c: u32;
    let mut local_c: u16;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f83cd8e8d: u32;
    let mut u_var3: u32;

    local_es_6 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0x8 != 0) {
        i_var2 = local_struct_1.field_0x40;
        pi_var1 = &local_struct_1.field_0x40;
        unsafe { *pi_var1 = *pi_var1 + 1 };
        if (i_var2 % local_struct_1.field_0x3e == 0) {
            local_struct_1.field_0x40 = 1;
            u_var3 = local_struct_1.field_0x8;
            pass1_1018_20ee(_PTR_LOOP_1050_0382, u_var3);
            u_var3 = u_var3 & 0xffff | local_DX_71 << 0x10;
            pass1_1008_3e94(
                &local_struct_1.field_0x28,
                CONCAT22(ctx.stack_seg_reg, local_a),
                CONCAT22(ctx.stack_seg_reg, local_8),
            );
            process_struct_1008_8d8a(
                (param_1 & 0xffff | local_es_6 << 0x10),
                u_var3,
                local_struct_1.field_0x4,
            );
            pass1_1008_4480(
                local_struct_1.field_0x4,
                (param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x28)),
                u_var3,
            );
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_8bc6(param_1: u32) {
    let mut local_AX_42: u16;
    let mut local_DX_42: u16;
    let mut i_var2: i32;
    let mut local_es_6: u16;

    let mut local_1a: u32;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f9f6c97b2: u32;
    let mut u_var1: u32;

    local_es_6 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 8) == 0) {
        return;
    }
    u_var1 = (i_var2 + 8);
    pass1_1018_20ee(_PTR_LOOP_1050_0382, u_var1);
    u_var1 = u_var1 & 0xffff | local_DX_42 << 0x10;
    pass1_1008_3e94(
        (i_var2 + 0x28),
        CONCAT22(ctx.stack_seg_reg, local_a),
        CONCAT22(ctx.stack_seg_reg, local_8),
    );
    process_struct_1008_8d8a(
        (param_1 & 0xffff | local_es_6 << 0x10),
        u_var1,
        (i_var2 + 4),
    );
    pass1_1008_4480(
        (i_var2 + 4),
        (param_1 & 0xffff0000 | ZEXT24((i_var2 + 0x28))),
        u_var1,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_8c4e(in_struct: *mut Struct207, param_2: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let struct_a: *mut Struct199;
    let paVar4: *mut Struct199;
    let mut local_DX_97: u16;
    let mut u_var5: u16;
    let struct_1: *mut Struct207;
    let mut local_es_4: u16;
    let mut u_var6: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (in_struct >> 0x10);
    struct_1 = in_struct;
    u_var6 = process_struct_1008_4772(struct_1.Struct104_field_4);
    struct_a = (u_var6 >> 0x10);
    u_var1 = 0;
    if ((struct_1.field_0xc == 0) || (struct_1.field_0xe == 0)) {
        paVar4 = struct_a;
        process_struct_1000_179c(0x14, struct_a);
        _local_e = CONCAT22(paVar4, u_var1);
        if ((paVar4 | u_var1) == 0) {
            u_var2 = 0;
            u_var5 = 0;
        } else {
            u_var3 = in_struct & 0xffff0000 | ZEXT24(struct_1 + 1);
            process_struct_1008_50c2(_local_e, (u_var6 + 8), (u_var6 + 4), u_var3, param_2);
            u_var2 = u_var3;
            u_var5 = local_DX_97;
        }
        pass1_1008_5134(CONCAT22(u_var5, u_var2));
    }
    pass1_1008_4480(
        param_2,
        (in_struct & 0xffff0000 | ZEXT24(struct_1 + 1)),
        struct_1.Struct104_field_4,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_8ce4(param_1: *mut Struct207, param_2: u32, param_3: u32) {
    let local_AX_120: *mut u8;
    let struct_a: *mut Struct199;
    let paVar1: *mut Struct199;
    let mut local_DX_113: u16;
    let mut u_var2: u16;
    let local_bx_4: *mut Struct207;
    let mut local_es_4: u16;

    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 6];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_7ffc9379903: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    _local_6 = process_struct_1008_4772(local_bx_4.Struct104_field_4);
    local_a = 0;
    pass1_1008_3e54(
        CONCAT22(ctx.stack_seg_reg, local_10),
        0,
        &local_bx_4.field_0x12,
        &local_bx_4.field_0x10,
    );
    local_AX_120 = local_10;
    pass1_1008_3f32(param_2, local_AX_120, ctx.stack_seg_reg);
    paVar1 = struct_a;
    process_struct_1000_179c(0x14, struct_a);
    if ((paVar1 | local_AX_120) == 0) {
        local_AX_120 = 0x0;
        u_var2 = 0;
    } else {
        temp_7ffc9379903 = (_local_6 >> 0x10);
        process_struct_1008_50c2(
            CONCAT22(paVar1, local_AX_120),
            (_local_6 + 8),
            (_local_6 + 4),
            param_2,
            param_3,
        );
        u_var2 = local_DX_113;
    }
    local_a = CONCAT22(u_var2, local_AX_120);
    pass1_1008_5134(CONCAT22(u_var2, local_AX_120));
    pass1_1008_4480(param_3, param_2, local_bx_4.Struct104_field_4);
    return;
}

pub unsafe fn pass1_1008_8f24(param_1: *mut Struct211) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_5: *mut Struct211;
    let local_bx_59: *mut Struct212;
    let local_SI_56: *mut Struct213;
    let mut local_es_5: u16;
    let mut local_es_59: u16;
    let mut local_6: u32;
    let mut temp_5f9d841f90: u32;
    let temp_5f5a60757f: *mut u32;
    let fn_ptr_1: fn();

    local_es_5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = 0x9170;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    if (local_bx_5.field_0x1a != 0) {
        local_6 = 0;
        while (true) {
            pu_var1 = &local_bx_5.field_0xa;
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val < local_6 || pu_var1_val == local_6) {
                break;
            }
            local_SI_56 = (local_6 * 4);
            temp_5f9d841f90 = local_bx_5.field_0x6;
            local_es_59 = (temp_5f9d841f90 >> 0x10);
            local_bx_59 = temp_5f9d841f90;
            temp_5f5a60757f = (local_bx_59 + local_SI_56);
            u_var2 = (local_bx_59 + local_SI_56 + 2);
            if ((u_var2 | temp_5f5a60757f) != 0) {
                unsafe { fn_ptr_1 = *temp_5f5a60757f };
                (**fn_ptr_1)();
            }
            local_6 = local_6 + 1;
        }
    }
    error_check_1000_17ce(local_bx_5.field_0x6);
    param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_8faa(param_1: u32, param_2: u32) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    pass1_1008_9004(
        param_1 & 0xffff | u_var1 << 0x10,
        param_2,
        (param_2 >> 0x10),
        (param_1 + 10),
    );
    return;
}

pub unsafe fn bad_func_1008_8fc4(param_1: *mut u8, param_2: u32) {
    let mut local_6: u32;

    return;
}

pub unsafe fn pass1_1008_9004(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut local_es_126: u16;
    let mut b_var5: bool;
    let mut temp_5f0a5228db: u32;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pu_var1 = (i_var3 + 10);
    let pu_var1_val = unsafe { *pu_var1 };
    if ((pu_var1_val < param_4 || pu_var1_val == param_4) || ((i_var3 + 6) == 0)) {
        pu_var2 = (i_var3 + 0x14);
        unsafe { b_var5 = *pu_var2 < param_4._2_2_ };
        let pu_var2_val = unsafe { *pu_var2 };
        if ((b_var5 || pu_var2_val == param_4._2_2_)
            && (b_var5
                || (
                    pu_var2 = (i_var3 + 0x12),
                    pu_var2_val < param_4 || pu_var2_val == param_4,
                )))
        {
            alloc_mem_1008_909c((param_1 & 0xffff | u_var4 << 0x10));
        }
        pu_var1 = (i_var3 + 0x12);
        let pu_var1_val = unsafe { *pu_var1 };
        if ((pu_var1_val < param_4 || pu_var1_val == param_4) || ((i_var3 + 6) == 0)) {
            return;
        }
        pu_var2 = (i_var3 + 0xc);
        unsafe { b_var5 = *pu_var2 < param_4._2_2_ };
        let pu_var2_val = unsafe { *pu_var2 };
        if ((b_var5 || pu_var2_val == param_4._2_2_)
            && (b_var5
                || (
                    pu_var2 = (i_var3 + 10),
                    pu_var2_val < param_4 || pu_var2_val == param_4,
                )))
        {
            (i_var3 + 10) = (param_4 + 1);
            (i_var3 + 0xc) = (param_4 + 1 >> 0x10);
        }
    }
    temp_5f0a5228db = (i_var3 + 6);
    local_es_126 = (temp_5f0a5228db >> 0x10);
    i_var3 = temp_5f0a5228db;
    (i_var3 + param_4 * 4) = param_2;
    (i_var3 + param_4 * 4 + 2) = param_3;
    return;
}

pub unsafe fn pass1_1008_914a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_8f24(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_92b2(param_1: u32, param_2: libc::c_long, param_3: libc::c_long) {
    let pp_var1: fn();
    let local_AX_35: *mut u8;
    

    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1008_57a4(
        CONCAT22(ctx.stack_seg_reg, local_c),
        param_1 & 0xffff0000 | (param_1 + 6),
    );
    while (true) {
        local_AX_35 = local_c;
        pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_35));
        if ((ctx.dx_reg | local_AX_35) == 0) {
            break;
        }
        if (((local_AX_35 + 4) == param_3) && ((local_AX_35 + 8) == param_2)) {
            local_4 = 1;
            pp_var1 = ((param_1 + 6) + 0xc);
            (**pp_var1)();
            local_8 = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1008_932a(param_1: u32) {
    let mut u_var1: i32;
    let local_AX_44: *mut u8;
    
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut local_es_4: u16;

    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];
    let fn_ptr_1: fn();

    local_es_4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 4) == 0) {
        (i_var3 + 4) = 1;
        pass1_1008_57a4(
            CONCAT22(ctx.stack_seg_reg, local_a),
            param_1 & 0xffff0000 | (i_var3 + 6),
        );
        while (true) {
            local_AX_44 = local_a;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_44));
            if ((ctx.dx_reg | local_AX_44) == 0) {
                break;
            }
            u_var1 = (local_AX_44 + 0xc);
            i_var2 = (local_AX_44 + 0xe) - (u_var1 < 0x37);
            (local_AX_44 + 0xc) = u_var1 - 0x37;
            (local_AX_44 + 0xe) = i_var2;
            if ((i_var2 < 1)
                && ((i_var2 < 0 || ((local_AX_44 + 0xc) == 0)) && ((local_AX_44 + 0x10) == 0)))
            {
                fn_ptr_1 = ((local_AX_44 + 4) + 4);
                (**fn_ptr_1)();
                (local_AX_44 + 0xc) = (local_AX_44 + 8);
            }
        }
        (i_var3 + 4) = 0;
    }
    return;
}

pub unsafe fn pass1_1008_93c0(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_941a(param_1: *mut u16, param_2: u16, param_3: u16) {
    unsafe { *param_1 = param_2 };
    (param_1 + 2) = param_3;
    return param_1;
}

pub unsafe fn pass1_1008_9436(param_1: *mut u16) {
    unsafe { *param_1 = 0 };
    (param_1 + 2) = 0;
    return param_1;
}

pub unsafe fn pass1_1008_944e(param_1: *mut u16, param_2: u16, param_3: u16) {
    (param_1 + 2) = param_3;
    unsafe { *param_1 = param_2 };
    return;
}

pub unsafe fn pass1_1008_9628(param_1: u32, param_2: u16) {
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 8) == 0) {
        (param_1 + 8) = param_2;
    }
    return;
}

pub unsafe fn pass1_1008_9c60(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: i32) {
    let fn_ptr_1: fn();

    if ((param_2_00 == 199) && (param_1_00 != 0x0)) {
        unsafe { fn_ptr_1 = *param_1_00 };
        (**fn_ptr_1)();
    }
    return;
}

pub unsafe fn pass1_1008_9cc4(param_1: u32, param_2: u16) {
    if ((param_1 + 8) != param_2) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1008_9ce0() {
    return 0;
}

pub unsafe fn pass1_1008_9d02(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_9e5a(param_1: *mut u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut local_DX_58: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_862d4416fbd: *mut u32;

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    unsafe { *param_1 = 0x9fb2 };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var5 + 0x1c) = 0x9fca;
    (i_var5 + 0x1e) = &ctx.PTR_LOOP_1050_1008;
    if (_PTR_LOOP_1050_0388 != 0) {
        if (param_1 == 0x0) {
            i_var4 = 0;
            local_DX_58 = 0;
        } else {
            i_var4 = i_var5 + 0x1c;
            local_DX_58 = u_var6;
        }
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x50, i_var4, local_DX_58);
    }
    local_4 = 0;
    while {
        pu_var1 = (i_var5 + 0x22 + local_4 * 4);
        u_var2 = (i_var5 + 0x22 + local_4 * 4 + 2);
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppc_var3 = *pu_var1 };
            (**ppc_var3)();
        }
        local_4 = local_4 + 1;
        local_4 < 0xc
    } {}
    if (param_1 == 0x0) {
        i_var5 = 0;
        u_var6 = 0;
    } else {
        i_var5 = i_var5 + 0x1c;
    }
    _local_8 = CONCAT22(u_var6, i_var5);
    unsafe { *_local_8 = ctx.s_1_1050_389a };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
    win_cleanup_1018_4d22(param_1);
    return;
}

pub unsafe fn pass1_1008_9f18(param_1: u16, param_2: u16, param_3: u16) {
    if (param_3 == 2) {
        pass1_1008_9f64(CONCAT22(param_2, param_1 - 0x1c));
        pass1_1010_1f62(CONCAT22(param_2, param_1 - 0x1c), 2);
    }
    return;
}

pub unsafe fn pass1_1008_9f48(param_1: u32) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var2 = (i_var1 + 0x20) * 4;
    return CONCAT22((i_var1 + i_var2 + 0x24), (i_var1 + i_var2 + 0x22));
}

pub unsafe fn pass1_1008_9f64(param_1: u32) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    i_var2 = param_1;
    pi_var1 = (i_var2 + 0x20);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    if (0xb < (i_var2 + 0x20)) {
        (i_var2 + 0x20) = 0;
    }
    return;
}

pub unsafe fn pass1_1008_9f8c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_9e5a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_a086(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.ptr_a_lo = 0xad92;
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = (i_var4 + 10);
    u_var2 = (i_var4 + 0xc);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pu_var1 = (i_var4 + 0x410);
    u_var2 = (i_var4 + 0x412);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_a930(param_1: u32, param_2: u16) {
    let pp_var1: fn();
    let pu_var2: *mut u8;
    
    let mut u_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var4 + 0x410));
    while {
        pu_var2 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
        u_var3 = ctx.dx_reg | pu_var2;
        if (u_var3 == 0) {
            process_struct_1000_179c(6, 0x0);
            _local_18 = CONCAT22(u_var3, pu_var2);
            if ((u_var3 | pu_var2) == 0) {
                local_12 = 0;
            } else {
                unsafe { *_local_18 = ctx.s_1_1050_389a };
                (pu_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
                (pu_var2 + 4) = param_2;
                unsafe { *_local_18 = 0xad8a };
                (pu_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
                local_12 = _local_18;
            }
            pp_var1 = ((i_var4 + 0x410) + 8);
            (**pp_var1)(0x1000, (i_var4 + 0x410), local_12);
            return;
        }
        (pu_var2 + 4) != param_2
    } {}
    return;
}

pub unsafe fn pass1_1008_a9ec(param_1: *mut Struct218) {
    let mut u_var1: u32;

    let local_bx_9: *mut Struct218;
    let mut u_var2: i32;
    let mut local_4: u16;

    local_4 = 0;
    u_var2 = (param_1 >> 0x10);
    local_bx_9 = param_1;
    if ((local_bx_9.field_0x414 == 0) && (u_var1 = local_bx_9.field_0x410, (u_var1 + 8) != 0)) {
        local_bx_9.field_0x414 = 1;
        pass1_1008_aa28((param_1 & 0xffff | u_var2 << 0x10), in_ax);
        local_4 = in_ax;
    }
    return local_4;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_aa28(param_1: *mut Struct219, param_2: u16) {
    
    let local_bx_4: *mut Struct219;
    let mut local_es_4: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5ffab03f5c: u32;
    let fn_ptr_1: fn();

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x414 != 0) {
        temp_5ffab03f5c = local_bx_4.field_0x410;
        if ((temp_5ffab03f5c + 8) == 0) {
            local_bx_4.field_0x414 = 0;
            return;
        }
        fn_ptr_1 = (local_bx_4.field_0x410 + 0x10);
        (**fn_ptr_1)();
        _local_6 = CONCAT22(ctx.dx_reg, param_2);
        if ((ctx.dx_reg | param_2) != 0) {
            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, (param_2 + 4));
            if (_local_6 != 0x0) {
                unsafe { fn_ptr_1 = *_local_6 };
                (**fn_ptr_1)();
            }
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1008_ab12(param_1: *mut pass1_struct_1, param_2: u16, param_1_00: u8) {
    let in_stack_00000009: u8;
    let mut local_4: u16;

    if (_param_1_00 == 0x37) {
        return 0x22;
    }
    if (_param_1_00 < 0x38) {
        if (param_1_00 == '\r') {
            return 0xf;
        }
        if (param_1_00 == '*') {
            return 0x2b;
        }
    }
    return 0;
}

pub unsafe fn pass1_1008_ab54(param_1: u32) {
    let mut local_es_9: u16;
    let mut local_4: u16;
    let mut temp_5f94569d31: u32;

    local_4 = 0;
    local_es_9 = (param_1 >> 0x10);
    if (((param_1 + 10) != 0) && (temp_5f94569d31 = (param_1 + 10), (temp_5f94569d31 + 8) != 0)) {
        local_4 = 1;
    }
    return local_4;
}

pub unsafe fn pass1_1008_ad38(param_1: u16, param_2: u8) {
    let in_stack_00000007: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(in_stack_00000007, CONCAT12(param_2, param_1));
    _param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        error_check_1000_17ce(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub unsafe fn pass1_1008_ada2(param_1: *mut u16, param_2: *mut Struct220) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0 };
    (param_1 + 2) = 0;
    (param_1 + 4) = param_2;
    unsafe { *param_1 = (param_2 * 6 + 0x3a4) };
    return param_1;
}

pub unsafe fn pass1_1008_add2(param_1: *mut u16) {
    unsafe { *param_1 = ((param_1 + 4) * 6 + 0x3a4) };
    return;
}

pub unsafe fn pass1_1008_adf2(param_1: u32) {
    return ((param_1 + 4) * 6 + 0x3a4);
}

pub unsafe fn pass1_1008_ae0c(param_1: u32) {
    return ((param_1 + 4) * 6 + 0x3a6);
}

pub unsafe fn pass1_1008_ae26(param_1: *mut i32) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let local_bx_4: *mut Struct221;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    i_var2 = (local_bx_4.field_0x4 * 6 + 0x3a8);
    if (i_var2 == 2) {
        if (local_bx_4.field_0x2 == 1) {
            unsafe { *param_1 = *param_1 + -1 };
            i_var2 = local_bx_4.field_0x4 * 6;
            pi_var1 = (i_var2 + 0x3a4);
            let pi_var1_val = unsafe { *pi_var1 };
            let param_1_val = unsafe { *param_1 };
            if (pi_var1_val != param_1_val && param_1_val <= pi_var1_val) {
                unsafe { *param_1 = (i_var2 + 0x3a4) + 1 };
                local_bx_4.field_0x2 = 0;
                return;
            }
        } else {
            unsafe { *param_1 = *param_1 + 1 };
            let param_1_val = unsafe { *param_1 };
            i_var2 = local_bx_4.field_0x4 * 6;
            if ((i_var2 + 0x3a6) < param_1_val) {
                unsafe { *param_1 = (i_var2 + 0x3a6) + -1 };
                local_bx_4.field_0x2 = 1;
                return;
            }
        }
    } else {
        if ((i_var2 != 3) && (i_var2 != 4)) {
            unsafe { *param_1 = *param_1 + 1 };
            i_var2 = local_bx_4.field_0x4 * 6;
            let param_1_val = unsafe { *param_1 };
            if ((i_var2 + 0x3a6) < param_1_val) {
                unsafe { *param_1 = (i_var2 + 0x3a4) };
            }
        }
    }
    return;
}

pub unsafe fn pass1_1008_aed8(param_1: u32) {
    if (((param_1 + 4) * 6 + 0x3a4) != 0) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1008_aefe(param_1: *mut u8, param_2: *mut u8, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0xaf7c;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1b3);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_af38(param_1: *mut u16) {
    unsafe { *param_1 = 0xaf7c };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    win_cleanup_1018_4d22(param_1);
    return;
}

pub unsafe fn pass1_1008_af56(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_af38(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_af94(param_1: *mut Struct222, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x22 = 0;
    CONCAT22(u_var1, param_1) = 0xbdcc;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_afde(param_1: *mut u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    unsafe { *param_1 = 0xbdcc };
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = (i_var4 + 10);
    u_var2 = (i_var4 + 0xc);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pu_var1 = (i_var4 + 0xe);
    u_var2 = (i_var4 + 0x10);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pu_var1 = (i_var4 + 0x12);
    u_var2 = (i_var4 + 0x14);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_b05a(param_1: *mut u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var1 + 4) = 0;
    unsafe { *param_1 = 0xbdc8 };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return param_1;
}

pub unsafe fn pass1_1008_b08c(param_1: *mut u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    unsafe { *param_1 = 0xbdc8 };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    error_check_1000_17ce((i_var1 + 4));
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_b0bc(param_1: *mut Struct199) {
    let local_bx_12: *mut Struct223;
    let mut u_var1: u16;

    pass1_1008_b05a(param_1);
    u_var1 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    local_bx_12.field_0x8 = 0;
    local_bx_12.field_0xa = 0;
    local_bx_12.field_0xe = 0;
    local_bx_12.field_0x10 = 0;
    param_1.field_0x0 = 0xbdc4;
    local_bx_12.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return param_1;
}

pub unsafe fn pass1_1008_b0f2(param_1: *mut u16) {
    let mut u_var1: u16;

    pass1_1008_b05a(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 8) = 0;
    unsafe { *param_1 = 0xbdc0 };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return param_1;
}

pub unsafe fn pass1_1008_b11e(param_1: *mut u16) {
    let mut u_var1: u16;

    pass1_1008_b05a(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 8) = 0;
    unsafe { *param_1 = 0xbddc };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_b146(param_1: *mut Struct224) {
    let mut u_var1: u32;
    let ppVar2: *mut pass1_struct_2;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct224;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x16 != 0) {
        u_var1 = local_bx_4.field_0x16;
        ppVar2 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            (u_var1 + 10),
        );
        pass1_1038_3608(CONCAT22(in_dx, ppVar2));
        u_var1 = local_bx_4.field_0x16;
        (u_var1 + 8) = 0;
        u_var1 = local_bx_4.field_0x16;
        (u_var1 + 10) = 0;
        u_var1 = local_bx_4.field_0x16;
        (u_var1 + 0xe) = 0;
        u_var1 = local_bx_4.field_0x16;
        (u_var1 + 0x10) = 0;
    }
    return;
}

pub unsafe fn pass1_1008_b1a6(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var3: u16;

    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x16) != 0) {
        u_var1 = (i_var4 + 0x16);
        u_var2 = error_check_1000_17ce((u_var1 + 4));
        u_var3 = CONCAT31(extraout_var, u_var2);
        pass1_fn_1008_60e8(param_2);
        u_var1 = (i_var4 + 0x16);
        u_var7 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        (i_var5 + 4) = u_var3;
        (i_var5 + 6) = in_dx;
        (i_var4 + 0x16) = 0;
    }
    return;
}

pub unsafe fn pass1_1008_b200(param_1: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let pu_var3: *mut u32;
    let paVar4: *mut Struct199;
    let pu_var5: *mut u16;
    let mut u_var6: u16;
    let mut u_var7: u32;
    
    let mut u_var8: i32;
    
    
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut u_var13: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    u_var12 = (param_1 >> 0x10);
    i_var11 = param_1;
    if ((i_var11 + 0xe) != 0) {
        return;
    }
    pu_var3 = (i_var11 + 0xe);
    paVar4 = (i_var11 + 0x10);
    if ((paVar4 | pu_var3) != 0) {
        unsafe { ppc_var2 = *pu_var3 };
        ppc_var2();
        paVar4 = ctx.dx_reg;
    }
    process_struct_1000_179c(0xc, paVar4);
    u_var8 = paVar4 | pu_var3;
    if (u_var8 == 0) {
        paVar4 = 0x0;
        u_var8 = 0;
    } else {
        paVar4 = process_struct_1008_574a(CONCAT22(paVar4, pu_var3));
    }
    (i_var11 + 0xe) = paVar4;
    (i_var11 + 0x10) = u_var8;
    pass1_1028_dc52(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_14)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        pu_var5 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var5));
        _local_18 = CONCAT22(ctx.dx_reg, pu_var5);
        paVar4 = (ctx.dx_reg | pu_var5);
        if (paVar4 == 0x0) {
            break;
        }
        u_var1 = (pu_var5 + 2);
        if ((pu_var5 + 0x100) == 0x8000001) {
            u_var7 = u_var1;
            process_struct_1000_179c(0xc, paVar4);
            u_var8 = u_var7;
            if ((paVar4 | u_var8) == 0) {
                u_var8 = 0;
                u_var9 = 0;
            } else {
                pass1_1008_b0f2(u_var8, paVar4);
                u_var9 = ctx.dx_reg;
            }
            u_var13 = pass1_1038_4d28(_local_18);
            u_var10 = (u_var13 >> 0x10);
            u_var6 = u_var13;
            pass1_fn_1008_60e8();
            (u_var8 + 4) = u_var6;
            (u_var8 + 6) = u_var10;
            (u_var8 + 8) = u_var1;
            u_var1 = (i_var11 + 0xe);
            ppc_var2 = ((i_var11 + 0xe) + 8);
            ppc_var2(0x38, u_var1, (u_var1 >> 0x10));
        }
    }
    return;
}

pub unsafe fn pass1_1008_b340(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x16) != 0) {
        u_var1 = (param_1 + 0x16);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + 6), (i_var2 + 4));
    }
    return 0;
}

pub unsafe fn pass1_1008_b366(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) != 0) {
        u_var1 = (param_1 + 0x1a);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + 6), (i_var2 + 4));
    }
    return 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_b38c(param_1: u32) {
    let pp_var1: fn();
    let in_ax: *mut Struct199;
    let mut i_var2: i32;
    let ppVar3: *mut pass1_struct_2;
    let in_dx: *mut Struct199;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let lVar9: u32;
    let mut u_var10: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    if ((i32_var6 + 0x12) == 0) {
        process_struct_1000_179c(0xc, in_dx);
        u_var4 = in_dx | in_ax;
        if (u_var4 == 0) {
            (i32_var6 + 0x12) = 0;
        } else {
            in_ax = process_struct_1008_574a(CONCAT22(in_dx, in_ax));
            (i32_var6 + 0x12) = in_ax;
            (i32_var6 + 0x14) = u_var4;
        }
        lVar9 = CONCAT22(u_var4, in_ax);
        local_4 = 0x6d9;
        while (u_var5 = (lVar9 >> 0x10), local_4 < 0x6e7) {
            if (local_4 == 0x6e3) {
                ppVar3 = pass1_1030_8344(
                    ctx._g_bool_1050_5748,
                    (ctx._g_bool_1050_5748 >> 0x10),
                    0x8000001,
                );
                lVar9 = CONCAT22(u_var5, ppVar3);
                if (&ppVar3.field_0x136 != 0) {
                    // goto LAB_1008_b44a;
                }
            } else {
                // LAB_1008_b44a:
                process_struct_1000_179c(10, (lVar9 >> 0x10));
                if (lVar9 == 0) {
                    u_var10 = 0;
                } else {
                    u_var10 = pass1_1008_b11e(lVar9);
                }
                u_var5 = (u_var10 >> 0x10);
                i_var2 = u_var10;
                u_var8 = load_str_1010_84ac(
                    ctx._g_struct_73_1050_14cc,
                    (ctx._g_struct_73_1050_14cc >> 0x10),
                    local_4,
                );
                (i_var2 + 4) = u_var8;
                (i_var2 + 6) = (u_var8 >> 0x10);
                (i_var2 + 8) = local_4 - 0x6d8;
                u_var8 = (i32_var6 + 0x12);
                pp_var1 = ((i32_var6 + 0x12) + 8);
                lVar9 = (**pp_var1)(0x1010, u_var8, (u_var8 >> 0x10), u_var10);
            }
            local_4 = local_4 + 1;
        }
    }
    return CONCAT22((i32_var6 + 0x14), (i32_var6 + 0x12));
}

pub unsafe fn pass1_1008_b47a(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x1e) != 0) {
        u_var1 = (param_1 + 0x1e);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + 6), (i_var2 + 4));
    }
    return 0;
}

pub unsafe fn pass1_1008_b4a0(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;

    let mut u_var2: u16;
    let mut in_dx: u16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let lVar7: u32;

    i_var4 = param_1;
    u_var5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        (i_var4 + 0x16) = 0;
    } else {
        pass1_1008_b9ce(param_1, param_2);
        (i_var4 + 0x16) = in_ax;
        (i_var4 + 0x18) = in_dx;
    }
    u_var1 = (i_var4 + 0x16);
    if ((u_var1 + 8) != 0) {
        pass1_1008_b200(param_1);
        u_var6 = pass1_1008_b38c(param_1);
        u_var3 = (u_var6 >> 0x10);
        u_var2 = u_var6;
        u_var1 = (i_var4 + 0x16);
        pass1_1008_b85c(param_1, (u_var1 + 10));
        (i_var4 + 0x1a) = u_var2;
        (i_var4 + 0x1c) = u_var3;
        u_var1 = (i_var4 + 0x16);
        lVar7 = pass1_1008_b8ac(param_1, (u_var1 + 0xe));
        (i_var4 + 0x1e) = lVar7;
        (i_var4 + 0x20) = (lVar7 >> 0x10);
        return;
    }
    (i_var4 + 0x1a) = 0;
    (i_var4 + 0x1e) = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_b544(param_1: u32, param_2: i32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let mut u_var5: u32;
    let ppVar6: *mut pass1_struct_2;
    let mut in_dx: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut unaff_cs: u16;

    i_var7 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (param_2 != 0) {
        if ((i_var7 + 0x1a) != 0) {
            u_var4 = (i_var7 + 0x16);
            (u_var4 + 8) = 1;
            u_var4 = (i_var7 + 0x1a);
            u_var5 = (i_var7 + 0x16);
            (u_var5 + 10) = (u_var4 + 8);
            u_var4 = (i_var7 + 0x1e);
            u_var5 = (i_var7 + 0x16);
            (u_var5 + 0xe) = (u_var4 + 8);
            u_var4 = (i_var7 + 0x16);
            ppVar6 = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                (u_var4 + 10),
            );
            unaff_cs = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_3608(CONCAT22(in_dx, ppVar6));
        }
    }
    (i_var7 + 0x1e) = 0;
    (i_var7 + 0x1a) = 0;
    (i_var7 + 0x16) = 0;
    pu_var1 = (i_var7 + 0xe);
    u_var2 = (i_var7 + 0x10);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    (i_var7 + 0xe) = 0;
    pu_var1 = (i_var7 + 0x12);
    u_var2 = (i_var7 + 0x14);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    (i_var7 + 0x12) = 0;
    return;
}

pub unsafe fn pass1_1008_b61a(param_1: u32, param_2: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;

    u_var2 = pass1_1008_b8fa(param_1, param_2);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1a) = u_var2;
    (param_1 + 0x1c) = (u_var2 >> 0x10);
    return;
}

pub unsafe fn pass1_1008_b63a(param_1: *mut u8, param_2: *mut u8) {
    let mut in_dx: u16;
    let mut u_var1: u16;

    pass1_1008_b964(param_1, param_2);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = in_ax;
    (param_1 + 0x20) = in_dx;
    return;
}

pub unsafe fn pass1_1008_b820(param_1: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    if (&ppVar1.field_0x152 == 0) {
        return 0;
    }
    u_var2 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xc), (param_1 + 10));
}

pub unsafe fn pass1_1008_b85c(param_1: u32, param_2: libc::c_long) {
    let pu_var1: *mut u8;
    
    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0xe));
    while {
        pu_var1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        (pu_var1 + 8) != param_2
    } {}
    return;
}

pub unsafe fn pass1_1008_b8ac(param_1: u32, param_2: i32) -> libc::c_long {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x12));
    while {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            return 0;
        }
        (lVar1 + 8) != param_2
    } {}
    return lVar1;
}

// WARNING: Variable defined which should be unmapped: local_12

pub unsafe fn pass1_1008_b8fa(param_1: u32, param_2: u32) {
    let pu_var1: *mut u8;
    
    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0xe));
    while {
        pu_var1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        pass1_1000_3d7a((pu_var1 + 4), param_2, 0);
        pu_var1 != 0x0
    } {}
    return;
}

// WARNING: Variable defined which should be unmapped: local_12

pub unsafe fn pass1_1008_b964(param_1: u32, param_2: libc::c_long) {
    let pu_var1: *mut u8;
    
    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x12));
    while {
        pu_var1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        pass1_1000_3d7a((pu_var1 + 4), param_2, 0);
        pu_var1 != 0x0
    } {}
    return;
}

// WARNING: Variable defined which should be unmapped: local_12

pub unsafe fn pass1_1008_b9ce(param_1: u32, param_2: libc::c_long) {
    let pu_var1: *mut u8;
    
    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while {
        pu_var1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        pass1_1000_3d7a((pu_var1 + 4), param_2, 0);
        pu_var1 != 0x0
    } {}
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1008_ba38(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let BVar3: bool;
    let pu_var4: *mut u8;
    let extraout_var: u32;
    
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_2a: u32;
    let mut local_1e: u16;
    let mut local_14: [u8; 8];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    u_var2 = write_to_file_1008_7cac(param_2, 0x16);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        u_var6 = (param_1 >> 0x10);
        i_var5 = param_1;
        local_c = (i_var5 + 0x22);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (BVar3 != 0) {
            if ((i_var5 + 10) == 0) {
                local_c = 0;
            } else {
                u_var1 = (i_var5 + 10);
                local_c = (u_var1 + 8);
            }
            local_1e = local_c;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1e), 2);
            if (BVar3 != 0) {
                pass1_1008_5784(CONCAT22(unaff_ss, local_14), (i_var5 + 10));
                while {
                    pu_var4 = local_14;
                    pass1_1008_5b12(CONCAT22(unaff_ss, pu_var4));
                    _local_a = CONCAT22(ctx.dx_reg, pu_var4);
                    if ((ctx.dx_reg | pu_var4) == 0) {
                        return;
                    }
                    u_var1 = (pu_var4 + 4);
                    write_to_file_1008_7c2a(param_2, u_var1, (u_var1 >> 0x10));
                    if (pu_var4 == 0x0) {
                        break;
                    }
                    local_6 = (_local_a + 8);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_2a = (_local_a + 10);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_2a), 4);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_6 = (_local_a + 0xe);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
                    BVar3 != 0
                } {}
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn pass1_1008_bd02(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_afde(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd28(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd4e(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd74(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd9a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// void  pass1_1008_bde0(param_1: u32)
pub unsafe fn pass1_1008_bde0(param_1: u32) {
    let mut u_var1: u32;
    let in_dx: *mut u16;
    let local_bx_64: *mut Struct225;
    let local_bx_92: *mut Struct226;
    let local_bx_120: *mut Struct227;
    let local_bx_148: *mut Struct228;
    let local_bx_176: *mut Struct229;
    let local_bx_204: *mut Struct230;
    let local_bx_232: *mut Struct231;
    let local_bx_260: *mut Struct232;
    let local_bx_288: *mut Struct233;
    let local_bx_316: *mut Struct234;
    let local_bx_344: *mut Struct235;
    let local_bx_372: *mut Struct236;
    let local_bx_400: *mut Struct237;
    let local_bx_428: *mut Struct238;
    let local_bx_456: *mut Struct239;
    let local_bx_484: *mut Struct240;
    let local_bx_512: *mut Struct241;
    let local_bx_540: *mut Struct242;
    let local_bx_568: *mut Struct243;
    let local_bx_596: *mut Struct244;
    let local_bx_624: *mut Struct245;
    let local_bx_654: *mut Struct246;
    let local_bx_685: *mut Struct247;
    let local_bx_716: *mut Struct248;
    let local_bx_747: *mut Struct249;
    let local_bx_778: *mut Struct250;
    let local_bx_809: *mut Struct251;
    let local_bx_840: *mut Struct252;
    let local_bx_871: *mut Struct253;
    let local_bx_902: *mut Struct254;
    let local_bx_933: *mut Struct255;
    let local_bx_964: *mut Struct256;
    let local_bx_995: *mut Struct257;
    let local_bx_1026: *mut Struct258;
    let local_bx_1057: *mut Struct259;
    let local_bx_1088: *mut Struct260;
    let local_bx_1119: *mut Struct261;
    let local_bx_1150: *mut Struct262;
    let local_bx_1181: *mut Struct263;
    let local_bx_1212: *mut Struct264;
    let local_bx_1243: *mut Struct265;
    let local_bx_1274: *mut Struct266;
    let local_bx_1305: *mut Struct267;
    let local_bx_1336: *mut Struct268;
    let local_bx_1367: *mut Struct269;
    let local_bx_1398: *mut Struct270;
    let local_bx_1429: *mut Struct271;
    let local_bx_1460: *mut Struct272;
    let local_bx_1491: *mut Struct273;
    let local_bx_1522: *mut Struct274;
    let local_bx_1553: *mut Struct275;
    let local_bx_1584: *mut Struct276;
    let local_bx_1615: *mut Struct277;
    let local_bx_1646: *mut Struct278;
    let local_bx_1677: *mut Struct279;
    let local_bx_1708: *mut Struct280;
    let local_bx_1739: *mut Struct281;
    let local_bx_1770: *mut Struct282;
    let local_bx_1801: *mut Struct283;
    let local_bx_1832: *mut Struct284;
    let local_bx_1863: *mut Struct285;
    let local_bx_1894: *mut Struct286;
    let local_bx_1925: *mut Struct287;
    let local_bx_1956: *mut Struct288;
    let local_bx_1987: *mut Struct289;
    let local_bx_2018: *mut Struct290;
    let local_bx_2049: *mut Struct291;
    let local_bx_2080: *mut Struct292;
    let mut u_var2: u16;

    ctx._PTR_LOOP_1050_06e0 = param_1;
    if (ctx.__g_Struct94_ptr_1 == 0) {
        u_var1 = param_1;
        struct_fn_1000_160a();
        _g_Struct94_ptr_1 = u_var1;
        ctx.g_u16_ptr_1050_5f2e = in_dx;
    } else {
    }
    alloc_mem_1000_1708(0x1aa, 0x10000, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    param_1 = _g_Struct94_ptr_1;
    (param_1 + 2) = ctx.g_u16_ptr_1050_5f2e;
    u_var2 = (*param_1 >> 0x10);
    local_bx_64 = param_1;
    local_bx_64.field_0x6 = 0x6e4;
    local_bx_64.field_0x8 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 10) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_92 = param_1;
    local_bx_92.field_0xc = 0x6ea;
    local_bx_92.field_0xe = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x10) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_120 = param_1;
    local_bx_120.field_0x12 = 0x6ee;
    local_bx_120.field_0x14 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x16) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_148 = param_1;
    local_bx_148.field_0x18 = 0x6f2;
    local_bx_148.field_0x1a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x1c) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_176 = param_1;
    local_bx_176.field_0x1e = 0x6f6;
    local_bx_176.field_0x20 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x22) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_204 = param_1;
    local_bx_204.field_0x24 = 0x6fe;
    local_bx_204.field_0x26 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x28) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_232 = param_1;
    local_bx_232.field_0x2a = 0x702;
    local_bx_232.field_0x2c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x2e) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_260 = param_1;
    local_bx_260.field_0x30 = 0x708;
    local_bx_260.field_0x32 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x34) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_288 = param_1;
    local_bx_288.field_0x36 = 0x70e;
    local_bx_288.field_0x38 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x3a) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_316 = param_1;
    local_bx_316.field_0x3c = 0x714;
    local_bx_316.field_0x3e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x40) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_344 = param_1;
    local_bx_344.field_0x42 = 0x71a;
    local_bx_344.field_0x44 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x46) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_372 = param_1;
    local_bx_372.field_0x48 = 0x71e;
    local_bx_372.field_0x4a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x4c) = 7;
    u_var2 = (*param_1 >> 0x10);
    local_bx_400 = param_1;
    local_bx_400.field_0x4e = 0x72c;
    local_bx_400.field_0x50 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x52) = 6;
    u_var2 = (*param_1 >> 0x10);
    local_bx_428 = param_1;
    local_bx_428.field_0x54 = 0x738;
    local_bx_428.field_0x56 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x58) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_456 = param_1;
    local_bx_456.field_0x5a = 0x73e;
    local_bx_456.field_0x5c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x5e) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_484 = param_1;
    local_bx_484.field_0x60 = 0x744;
    local_bx_484.field_0x62 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 100) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_512 = param_1;
    local_bx_512.field_0x66 = 0x74c;
    local_bx_512.field_0x68 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x6a) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_540 = param_1;
    local_bx_540.field_0x6c = 0x750;
    local_bx_540.field_0x6e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x70) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_568 = param_1;
    local_bx_568.field_0x72 = 0x756;
    local_bx_568.field_0x74 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x76) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_596 = param_1;
    local_bx_596.field_0x78 = 0x75a;
    local_bx_596.field_0x7a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x7c) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_624 = param_1;
    local_bx_624.field_0x7e = 0x75e;
    local_bx_624.field_0x80 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x82) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_654 = param_1;
    local_bx_654.field_0x84 = 0x764;
    local_bx_654.field_0x86 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x88) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_685 = param_1;
    local_bx_685.field_0x8a = 0x76a;
    local_bx_685.field_0x8c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x8e) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_716 = param_1;
    local_bx_716.field_0x90 = 0x770;
    local_bx_716.field_0x92 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x94) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_747 = param_1;
    local_bx_747.field_0x96 = 0x774;
    local_bx_747.field_0x98 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x9a) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_778 = param_1;
    local_bx_778.field_0x9c = 0x77c;
    local_bx_778.field_0x9e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xa0) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_809 = param_1;
    local_bx_809.field_0xa2 = 0x780;
    local_bx_809.field_0xa4 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xa6) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_840 = param_1;
    local_bx_840.field_0xa8 = 0x782;
    local_bx_840.field_0xaa = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xac) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_871 = param_1;
    local_bx_871.field_0xae = 0x786;
    local_bx_871.field_0xb0 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xb2) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_902 = param_1;
    local_bx_902.field_0xb4 = 0x78a;
    local_bx_902.field_0xb6 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xb8) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_933 = param_1;
    local_bx_933.field_0xba = 0x78e;
    local_bx_933.field_0xbc = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xbe) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_964 = param_1;
    local_bx_964.field_0xc0 = 0x792;
    local_bx_964.field_0xc2 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xc4) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_995 = param_1;
    local_bx_995.field_0xc6 = 0x796;
    local_bx_995.field_0xc8 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xca) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1026 = param_1;
    local_bx_1026.field_0xcc = 0x79e;
    local_bx_1026.field_0xce = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xd0) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1057 = param_1;
    local_bx_1057.field_0xd2 = 0x7a0;
    local_bx_1057.field_0xd4 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xd6) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1088 = param_1;
    local_bx_1088.field_0xd8 = 0x7a4;
    local_bx_1088.field_0xda = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xdc) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1119 = param_1;
    local_bx_1119.field_0xde = 0x7a6;
    local_bx_1119.field_0xe0 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xe2) = 6;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1150 = param_1;
    local_bx_1150.field_0xe4 = 0x7b2;
    local_bx_1150.field_0xe6 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xe8) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1181 = param_1;
    local_bx_1181.field_0xea = 0x7b4;
    local_bx_1181.field_0xec = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xee) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1212 = param_1;
    local_bx_1212.field_0xf0 = 0x7ba;
    local_bx_1212.field_0xf2 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xf4) = 0x2d;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1243 = param_1;
    local_bx_1243.field_0xf6 = 0x814;
    local_bx_1243.field_0xf8 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xfa) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1274 = param_1;
    local_bx_1274.field_0xfc = 0x81a;
    local_bx_1274.field_0xfe = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x100) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1305 = param_1;
    local_bx_1305.field_0x102 = 0x81c;
    local_bx_1305.field_0x104 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x106) = 0x4b;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1336 = param_1;
    local_bx_1336.field_0x108 = 0x8b2;
    local_bx_1336.field_0x10a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x10c) = 6;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1367 = param_1;
    local_bx_1367.field_0x10e = 0x8be;
    local_bx_1367.field_0x110 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x112) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1398 = param_1;
    local_bx_1398.field_0x11a = 0x8c6;
    local_bx_1398.field_0x11c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x11e) = 0x35;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1429 = param_1;
    local_bx_1429.field_0x120 = 0x930;
    local_bx_1429.field_0x122 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x124) = 0x2e;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1460 = param_1;
    local_bx_1460.field_0x114 = 0x98c;
    local_bx_1460.field_0x116 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x118) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1491 = param_1;
    local_bx_1491.field_0x126 = 0x98e;
    local_bx_1491.field_0x128 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x12a) = 9;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1522 = param_1;
    local_bx_1522.field_0x12c = 0x9a0;
    local_bx_1522.field_0x12e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x130) = 0x1a;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1553 = param_1;
    local_bx_1553.field_0x132 = 0x9d4;
    local_bx_1553.field_0x134 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x136) = 8;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1584 = param_1;
    local_bx_1584.field_0x138 = 0x9e4;
    local_bx_1584.field_0x13a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x13c) = 0x4a;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1615 = param_1;
    local_bx_1615.field_0x144 = 0xa78;
    local_bx_1615.field_0x146 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x148) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1646 = param_1;
    local_bx_1646.field_0x14a = 0xa7c;
    local_bx_1646.field_0x14c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x14e) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1677 = param_1;
    local_bx_1677.field_0x150 = 0xa7e;
    local_bx_1677.field_0x152 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x154) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1708 = param_1;
    local_bx_1708.field_0x156 = 0xa80;
    local_bx_1708.field_0x158 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x15a) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1739 = param_1;
    local_bx_1739.field_0x15c = 0xa86;
    local_bx_1739.field_0x15e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x160) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1770 = param_1;
    local_bx_1770.field_0x168 = 0xa8a;
    local_bx_1770.field_0x16a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x16c) = 0x1b;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1801 = param_1;
    local_bx_1801.field_0x16e = 0xac0;
    local_bx_1801.field_0x170 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x172) = 0x16;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1832 = param_1;
    local_bx_1832.field_0x174 = 0xaec;
    local_bx_1832.field_0x176 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x178) = 0x3e;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1863 = param_1;
    local_bx_1863.field_0x17a = 0xb68;
    local_bx_1863.field_0x17c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x17e) = 0x46;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1894 = param_1;
    local_bx_1894.field_0x180 = 0xbf4;
    local_bx_1894.field_0x182 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x184) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1925 = param_1;
    local_bx_1925.field_0x186 = 0xbf6;
    local_bx_1925.field_0x188 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x18a) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1956 = param_1;
    local_bx_1956.field_0x18c = 0xbfc;
    local_bx_1956.field_0x18e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 400) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1987 = param_1;
    local_bx_1987.field_0x192 = 0xc02;
    local_bx_1987.field_0x194 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x196) = 10;
    u_var2 = (*param_1 >> 0x10);
    local_bx_2018 = param_1;
    local_bx_2018.field_0x198 = 0xc16;
    local_bx_2018.field_0x19a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x19c) = 0x24;
    u_var2 = (*param_1 >> 0x10);
    local_bx_2049 = param_1;
    local_bx_2049.field_0x19e = 0xc5e;
    local_bx_2049.field_0x1a0 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x1a2) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_2080 = param_1;
    local_bx_2080.field_0x1a4 = 0xc62;
    local_bx_2080.field_0x1a6 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x1a8) = 0x44;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_c626(param_1: *mut u32) {
    ctx._PTR_LOOP_1050_06e0 = 0;
    let param_1_val = unsafe { *param_1 };
    error_check_1000_17ce(param_1_val);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_c646(param_1: u16, param_2: u32) -> i32 {
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let pu_var4: *mut u8;
    let mut in_dx: i32;
    let mut unaff_si: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var4 = pass1_1008_c6fa(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    local_12 = 0;
    loop {
        pi_var1 = (pu_var4 + 4);
        let pi_var1_val = unsafe { *pi_var1 };
        if (pi_var1_val == local_12 || pi_var1_val < local_12) {
            // LAB_1008_c6a5:
            return local_12._2_2_;
        }
        u_var3 = (pu_var4 & 0xffff | in_dx << 0x10);
        u_var2 = (u_var3 + local_12 * 2);
        if ((u_var2 * 2 + ppVar5 + 10) != 0) {
            local_12 = u_var2 << 0x10;
            // goto LAB_1008_c6a5;
        }
        local_12 = (local_12 + 1);
    }
}

pub unsafe fn pass1_1008_c6ae(param_1: u32, param_2: i32, param_3: u16) -> bool {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let pu_var3: *mut u8;
    let mut in_dx: i32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var3 = pass1_1008_c6fa(param_1, param_3);
    local_8 = 0;
    while (true) {
        pu_var1 = (pu_var3 + 4);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_8 || pu_var1_val < local_8) {
            return 0;
        }
        u_var2 = (pu_var3 & 0xffff | in_dx << 0x10);
        if ((u_var2 + local_8 * 2) == param_2) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return 1;
}

pub unsafe fn pass1_1008_c6fa(param_1: *mut u8, param_2: u16) {
    let mut in_eax: u32;

    if ((0 < param_2) && (param_2 < 0x47)) {
        return (in_eax & 0xffff0000 | (param_2 * 6 + param_1));
    }
    return (in_eax & 0xffff0000);
}

pub unsafe fn pass1_1008_c72a(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    (param_1 + 0xe) = 0;
    CONCAT22(param_2, param_1) = 0xca4a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_c75c(param_1: *mut Struct293) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct293;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = 0xca4a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = local_bx_5.field_0xa;
    u_var2 = local_bx_5.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_c79a(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let pu_var2: *mut u8;
    let mut u_var3: u32;
    
    
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: [u8; 4];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    u_var5 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while (true) {
        pu_var2 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
        _local_e = CONCAT22(ctx.dx_reg, pu_var2);
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        u_var1 = (pu_var2 + 4);
        pass1_1000_3d7a(u_var1, (u_var1 >> 0x10), param_2);
        if (pu_var2 == 0x0) {
            pass1_1020_a43e(CONCAT22(unaff_ss, local_12));
            pass1_1020_a6ee(CONCAT22(unaff_ss, local_12), (_local_e + 0x12));
            u_var3 = (ctx._PTR_LOOP_1050_65e2 + 0x52);
            u_var4 = ctx.dx_reg;
            pass1_1030_4bbe(u_var3, (_local_e + 0x12));
            (param_1 + 0xe) = (u_var3 + 0x94) + *ctx._PTR_LOOP_1050_65e2;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_c83a(param_1: u32) {
    if (*ctx._PTR_LOOP_1050_65e2 <= (param_1 + 0xe)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1008_c85e(param_1: u32) {
    let local_bx_3: *mut Struct294;
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (&local_bx_3.field_0xa == 0) {
        process_struct_1008_c882(param_1 & 0xffff | u_var1 << 0x10);
    }
    return CONCAT22(local_bx_3.field_0xc, local_bx_3.field_0xa);
}

pub unsafe fn pass1_1008_ca24(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_c75c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ca5a(param_1: *mut Struct295, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    CONCAT22(u_var1, param_1) = 0xd71a;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_caa0(param_1: *mut u16) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0xd71a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    pass1_1008_cac6(param_1 & 0xffff | u_var1 << 0x10);
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_cac6(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 10);
    u_var2 = (i_var4 + 0xc);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 10) = 0;
    pu_var1 = (i_var4 + 0xe);
    u_var2 = (i_var4 + 0x10);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0xe) = 0;
    pu_var1 = (i_var4 + 0x12);
    u_var2 = (i_var4 + 0x14);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x12) = 0;
    pu_var1 = (i_var4 + 0x16);
    u_var2 = (i_var4 + 0x18);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x16) = 0;
    pu_var1 = (i_var4 + 0x1a);
    u_var2 = (i_var4 + 0x1c);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x1a) = 0;
    pu_var1 = (i_var4 + 0x1e);
    u_var2 = (i_var4 + 0x20);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    (i_var4 + 0x1e) = 0;
    return;
}

pub unsafe fn pass1_1008_cfa0(param_1: *mut Struct298, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut b_var3: bool;
    let pu_var4: *mut u32;
    let pa_var5: *mut Struct199;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u32;
    
    let mut u_var9: i32;
    
    let struct_a: *mut Struct199;
    
    
    let struct_a_00: *mut Struct199;
    
    let mut u_var10: u16;
    let mut extraout_dx_04: i32;
    let struct_a_01: *mut Struct199;
    let mut extraout_dx_05: u16;
    let struct_a_02: *mut Struct199;
    let mut extraout_dx_06: u16;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let u_var14: u8;
    let mut uVar15: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var13 = (param_1 >> 0x10);
    i_var12 = param_1;
    pu_var4 = (i_var12 + 0x16);
    pa_var5 = (i_var12 + 0x18);
    if ((pa_var5 | pu_var4) != 0) {
        unsafe { ppc_var2 = *pu_var4 };
        ppc_var2();
        pa_var5 = ctx.dx_reg;
    }
    process_struct_1000_179c(0xc, pa_var5);
    u_var9 = pa_var5 | pu_var4;
    if (u_var9 == 0) {
        pa_var5 = 0x0;
        u_var9 = 0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, pu_var4));
    }
    (i_var12 + 0x16) = pa_var5;
    (i_var12 + 0x18) = u_var9;
    b_var3 = false;
    u_var8 = (param_2 + 0x1f6);
    u_var11 = u_var8;
    uVar15 = (u_var8 >> 0x10);
    pass1_1030_38f2(u_var11, uVar15, 2);
    u_var9 = u_var8;
    if ((-1 < ctx.dx_reg) && (0 < ctx.dx_reg || (u_var9 != 0))) {
        u_var7 = u_var9;
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x430,
        );
        u_var14 = 0;
        pa_var5 = struct_a;
        u_var9 = u_var7;
        process_struct_1000_179c(0x14, struct_a);
        if ((pa_var5 | u_var9) == 0) {
            u_var9 = 0;
            u_var10 = 0;
        } else {
            u_var14 = 0x18;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var9),
                u_var8 & 0xffff | ctx.dx_reg << 0x10,
                CONCAT22(struct_a, u_var7),
                2,
            );
            u_var10 = ctx.dx_reg;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var9, u_var10);
        b_var3 = true;
    }
    pass1_1030_38f2(u_var11, uVar15, 3);
    if ((-1 < ctx.dx_reg) && (0 < ctx.dx_reg || (u_var9 != 0))) {
        u_var6 = u_var9;
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x431,
        );
        u_var14 = 0;
        pa_var5 = struct_a_00;
        u_var7 = u_var6;
        process_struct_1000_179c(0x14, struct_a_00);
        if ((pa_var5 | u_var7) == 0) {
            u_var9 = 0;
            u_var10 = 0;
        } else {
            u_var14 = 0x18;
            u_var8 = CONCAT22(ctx.dx_reg, u_var9);
            u_var9 = u_var7;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var7),
                u_var8,
                CONCAT22(struct_a_00, u_var6),
                3,
            );
            u_var10 = ctx.dx_reg;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var9, u_var10);
        b_var3 = true;
    }
    pass1_1030_38f2(u_var11, uVar15, 4);
    if ((-1 < extraout_dx_04) && (0 < extraout_dx_04 || (u_var9 != 0))) {
        u_var6 = u_var9;
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x432,
        );
        u_var14 = 0;
        pa_var5 = struct_a_01;
        u_var7 = u_var6;
        process_struct_1000_179c(0x14, struct_a_01);
        if ((pa_var5 | u_var7) == 0) {
            u_var9 = 0;
            u_var11 = 0;
        } else {
            u_var14 = 0x18;
            u_var8 = CONCAT22(extraout_dx_04, u_var9);
            u_var9 = u_var7;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var7),
                u_var8,
                CONCAT22(struct_a_01, u_var6),
                4,
            );
            u_var11 = extraout_dx_05;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var9, u_var11);
        b_var3 = true;
    }
    if (!b_var3) {
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x440,
        );
        u_var14 = 0;
        pa_var5 = struct_a_02;
        u_var7 = u_var9;
        process_struct_1000_179c(0x14, struct_a_02);
        if ((pa_var5 | u_var7) == 0) {
            u_var7 = 0;
            u_var11 = 0;
        } else {
            u_var14 = 0x18;
            pass1_1018_4842(
                CONCAT22(pa_var5, u_var7),
                0,
                CONCAT22(struct_a_02, u_var9),
                0,
            );
            u_var11 = extraout_dx_06;
        }
        u_var1 = (i_var12 + 0x16);
        ppc_var2 = ((i_var12 + 0x16) + 4);
        ppc_var2(u_var14, u_var1, (u_var1 >> 0x10), u_var7, u_var11);
    }
    return;
}

pub unsafe fn pass1_1008_d6f4(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_caa0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_d72e(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    CONCAT22(param_2, param_1) = 0xd780;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_d75a(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Variable defined which should be unmapped: local_6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_d790(in_struct_1: *mut Struct299, param_2: u32) {
    let mut local_AX_18: u16;
    let mut local_DX_49: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    local_6 = param_2;
    uStack4 = (param_2 >> 0x10);
    process_struct_1010_1d48(CONCAT22(local_6, in_struct_1), uStack4);
    local_AX_18 = 0;
    &in_struct_1.field_0xa = 0;
    in_struct_1.field_0xe = 0;
    CONCAT22(local_6, in_struct_1) = 0xd98e;
    in_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x9b);
    in_struct_1.field_0xa = local_AX_18;
    in_struct_1.field_0xc = local_DX_49;
    return;
}

pub unsafe fn pass1_1008_d7da(param_1: *mut u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    unsafe { *param_1 = 0xd98e };
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = (i_var4 + 10);
    u_var2 = (i_var4 + 0xc);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_d968(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_d7da(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_d99e(param_1: *mut Struct534, param_2: u16, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0xd9fa;
    param_1.u16_x02 = &ctx.PTR_LOOP_1050_1008;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a);
    _PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

pub unsafe fn pass1_1008_d9d4(param_1: u32, param_2: u8) {
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_dc2c(param_1: *mut u16) {
    let mut local_es_4: u16;
    let mut temp_5f04f790ee: u32;

    local_es_4 = (param_1 >> 0x10);
    unsafe { *param_1 = 0xdc80 };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    error_check_1000_17ce((param_1 + 0x18));
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1008_dc5a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1008_dc2c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_dc90(in_struct_1: *mut Struct301, param_2: u32, param_3: u32) {
    let local_bx_4: *mut Struct301;
    let mut u_var1: u16;

    u_var1 = (in_struct_1 >> 0x10);
    local_bx_4 = in_struct_1;
    in_struct_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_3;
    local_bx_4.field_0x8 = param_2;
    local_bx_4.field_0xc = 0;
    local_bx_4.field_0xe = 0;
    local_bx_4.field_0x12 = 0;
    in_struct_1 = 0xdd4a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_dd1e(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_14f0_01b8(param_1: u8, param_2: i32) {
    let mut cVar1: u8;
    let mut in_CL: u8;
    let local_SP: *mut u16;
    let local_BP__1: *mut u16;
    let unaff_si: *mut libc::c_char;

    let mut local_DS__1: u16;
    let string_1: *mut libc::c_char;

    local_SP = &stack0xfffe;
    cVar1 = '\x1e';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe { *local_SP = *local_BP__1 };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    string_1 = unaff_si + param_2;
    unsafe { *string_1 = *string_1 - in_CL };
    string_1 = unaff_si + param_2;
    unsafe { *string_1 = *string_1 + param_1 };
    string_1 = unaff_si + param_2;
    unsafe { *string_1 = *string_1 + param_1 };
    string_1 = unaff_si;
    unsafe { *string_1 = *string_1 + in_CL };
    return;
}

pub unsafe fn pass1_11a0_03fl4(param_1: *mut char, param_2: u16, param_3: *mut char) {
    let pi_var1: *mut i32;
    let pu_var2: *mut u8;
    let pb_var3: *mut u8;
    let mut b_var4: u8;
    let mut b_var5: u8;
    let mut u_var6: i32;
    let mut cVar7: u8;
    let mut b_var8: u8;
    let string_2: *mut libc::c_char;
    let mut cVar9: u8;
    let string_5: *mut libc::c_char;
    let string_6: *mut libc::c_char;
    let local_BP_188: *mut u8;
    let string_4: *mut libc::c_char;
    let string_1: *mut libc::c_char;

    let mut local_DS__1: u16;
    let mut b_var10: bool;
    let in_stack_000003fa: *mut libc::c_char;
    let in_stack_000003fc: *mut libc::c_char;
    let mut in_stack_000003fe: u16;
    let mut in_stack_00000400: i32;
    let local_76ef: u8;
    let mut temp_13fb229f94a: u16;
    let mut temp_5f472f0e69: u16;
    let bytes_2: *mut u8;
    let string_3: *mut libc::c_char;

    bytes_2 = (param_3 + string_4);
    b_var4 = param_1;
    unsafe { *bytes_2 = *bytes_2 | b_var4 };
    string_3 = param_3;
    unsafe { *string_3 = *string_3 + 0x2 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    local_BP_188 = &stack0x86fa;
    bytes_2 = (param_3 + string_4);
    unsafe { *bytes_2 = *bytes_2 | b_var4 };
    string_3 = param_3;
    unsafe { *string_3 = *string_3 + 0x2 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    param_3[string_4] = -0x5e;
    string_3 = string_4;
    cVar9 = (param_2 >> 8);
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    cVar7 = param_2;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = (param_3 + string_1 + 0x217);
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    param_3[string_4] = -0x51;
    string_3 = string_4;
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + cVar7 };
    pu_var2 = &local_76ef + string_1;
    unsafe { *pu_var2 = *pu_var2 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + b_var4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    temp_5f472f0e69 = (param_3 + string_4);
    string_3 = param_3 + string_1;
    cVar9 = (param_3 >> 8);
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + cVar7 };
    pi_var1 = (&stack0x86fa + string_4);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    bytes_2 = &stack0x86fa + string_4;
    unsafe { b_var5 = *bytes_2 };
    unsafe { *bytes_2 = *bytes_2 + b_var4 };
    string_3 = &stack0x875b + string_4;
    unsafe { *string_3 = *string_3 + 'w' + CARRY1(b_var5, b_var4) };
    bytes_2 = (string_4 + 0x6f);
    unsafe { *bytes_2 = *bytes_2 & b_var4 };
    string_2 = param_1;
    string_5 = param_1;
    string_6 = (param_3 & 0xff | (cVar9 * 0x2) << 8);
    let bytes_2_val = unsafe { *bytes_2 };
    if (bytes_2_val != 0) {
        cVar7 = _in(param_1);
        unsafe { *string_1 = cVar7 };
        temp_13fb229f94a = _in(param_1);
        (string_1 + 1) = temp_13fb229f94a;
        local_BP_188 = ((param_1 + 0x69) * 0x676e);
        string_3 = in_stack_000003fa;
        unsafe { *string_3 = *string_3 + in_stack_00000400 };
        u_var6 = in_stack_00000400 & (in_stack_000003fa + param_1);
        b_var5 = u_var6 ^ in_stack_000003fa[param_1];
        string_2 = (u_var6 & 0xff00 | b_var5);
        bytes_2 = (in_stack_000003fa + param_1);
        unsafe { *bytes_2 = *bytes_2 | b_var5 };
        string_3 = param_1 + -0x3a00;
        unsafe { *string_3 = *string_3 + b_var5 };
        pu_var2 = local_BP_188 + param_1;
        unsafe { *pu_var2 = *pu_var2 + (in_stack_000003fc >> 8) };
        param_2 = in_stack_000003fe;
        string_5 = in_stack_000003fc;
        string_6 = in_stack_000003fa;
        string_4 = param_1;
        string_1 = param_1;
    }
    string_3 = (local_BP_188 + string_1 + 0x217);
    unsafe { *string_3 = *string_3 + (string_6 >> 8) };
    pu_var2 = local_BP_188 + string_4;
    b_var5 = string_2;
    unsafe { *pu_var2 = *pu_var2 + b_var5 };
    string_3 = string_6 + string_4;
    unsafe { *string_3 = *string_3 };
    string_6[string_4] = '2';
    string_3 = 0x100;
    cVar7 = param_2;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = string_6 + string_1;
    unsafe { *string_3 = *string_3 + b_var5 };
    string_3 = string_6 + string_1;
    unsafe { *string_3 = *string_3 + b_var5 };
    if (string_5 != 0xffff) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    if (string_5 == 0xffff) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    string_5 = string_4 + 1;
    let string_4_val = unsafe { *string_4 };
    out(string_4_val, 0);
    string_3 = string_6 + 0x6b;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = string_6 + string_5;
    unsafe { *string_3 = *string_3 + b_var5 };
    b_var8 = cVar7 - 1;
    pu_var2 = local_BP_188 + string_5;
    unsafe { *pu_var2 = *pu_var2 + b_var5 };
    bytes_2 = 0x3000;
    unsafe { b_var4 = *bytes_2 };
    unsafe { *bytes_2 = *bytes_2 + b_var8 };
    string_3 = string_6 + string_5;
    unsafe { *string_3 = (*string_3 - b_var5) - CARRY1(b_var4, b_var8) };
    string_3 = string_6 + (string_1 + -1);
    unsafe { *string_3 = *string_3 + b_var5 };
    b_var10 = string_2 * 0x100 == -1;
    if (b_var10) {
        if (!b_var10) {
            pb_var3 = (string_4 + 2);
            let string_5_val = unsafe { *string_5 };
            out(string_5_val, 0);
            pu_var2 = local_BP_188 + 0x65;
            unsafe { *pu_var2 = *pu_var2 + b_var8 };
            string_3 = string_6;
            b_var8 = (string_2 >> 8);
            unsafe { *string_3 = *string_3 + b_var8 };
            pu_var2 = local_BP_188 + pb_var3;
            unsafe { *pu_var2 = *pu_var2 };
            b_var5 = b_var5 ^ string_6[pb_var3];
            bytes_2 = pb_var3;
            unsafe { b_var4 = *bytes_2 };
            unsafe { *bytes_2 = *bytes_2 + b_var8 };
            string_3 = string_6 + pb_var3;
            unsafe { *string_3 = (*string_3 - b_var5) - CARRY1(b_var4, b_var8) };
            string_3 = string_6 + (string_1 + -2);
            unsafe { *string_3 = *string_3 + b_var5 };
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_
}

pub unsafe fn pass1_1050_3552() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;

    pu_var2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub unsafe fn pass1_1050_3654() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;

    pu_var2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub unsafe fn pass1_1050_37d4() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;

    pu_var2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_bp };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub unsafe fn pass1_1050_309c() {
    let pb_var1: *mut u8;
    let pu_var2: *mut u32;
    let pcVar3: *mut libc::c_char;
    let piVar4: *mut i32;
    let mut b_var5: u8;
    let mut b_var6: u8;

    let mut b_var11: u8;
    let local_AX_15: *mut Struct329;
    let mut cVar12: u8;
    let local_AX_111: *mut Struct327;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let pc_var10: *mut libc::c_char;
    let mut b_var13: u8;
    let mut in_cx: i32;
    let mut b_var14: u8;
    let mut b_var15: u8;
    let mut cVar16: u8;
    let mut b_var17: u8;
    let mut b_var18: u8;
    let mut in_dx: i32;
    let mut i_var19: i32;
    let mut b_var20: u8;
    let mut c_var21: u8;
    let mut b_var22: u8;
    let local_BX__1: *mut Struct328;
    let mut b_var23: u8;
    let pu_var24: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: *mut libc::c_char;
    let pc_var25: *mut libc::c_char;
    let unaff_DI: *mut u8;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;
    let mut in_CF: u8;
    let mut b_var26: bool;
    let mut b_var27: bool;
    let mut bStack002a: u8;
    let mut in_stack_0000002a: u16;
    let mut bStack002b: u8;
    let mut in_stack_0000502a: u8;
    let mut in_stack_0000502b: u8;
    let mut bStack3: u8;

    pu_var24 = &stack0xfffe;
    cVar12 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var24 = pu_var24 + -1;
        unsafe { *pu_var24 = *unaff_bp };
        cVar12 = cVar12 + -1;
        '\0' < cVar12
    } {}
    pb_var1 = &stack0xfffe + unaff_DI;
    b_var13 = in_cx;
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 >> (b_var13 & 0x1f) };
    b_var26 = (in_cx & 0x1f) != 0;
    b_var22 = local_BX__1;
    b_var11 = (in_ax >> 8)
        + b_var22
        + (!b_var26 & in_CF | (b_var26 && (b_var6 >> (b_var13 & 0x1f) - 1 & 1) != 0));
    pu_var2 = (local_BX__1 + unaff_si + 0x10);
    let pu_var2_val = unsafe { *pu_var2 };
    i_var19 = in_dx - pu_var2_val;
    out(0x2b, in_ax);
    local_AX_15._0_1_ = in_ax + (i_var19 >> 8) + (in_dx < pu_var2_val);
    i_var19 = (i_var19 - (local_BX__1 + unaff_si + 0x10)) - (local_BX__1 + unaff_si + 0x10);
    pb_var1 = 0x502c;
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var13 + (0xd3 < local_AX_15) };
    pcVar3 = unaff_si + &local_BX__1.field_0x0;
    let pc_var3_val = unsafe { *pcVar3 };
    unsafe {
        *pcVar3 = *pcVar3
            + b_var22
            + (CARRY1(b_var6, b_var13) || CARRY1(b_var18 + b_var13, 0xd3 < local_AX_15))
    };
    pcVar3 = &stack0xfffe + unaff_si;
    unsafe { *pcVar3 = *pcVar3 + b_var11 + ((local_AX_15 + 0x2c) < 0x50) };
    pcVar3 = unaff_si;
    b_var14 = (in_cx >> 8);
    unsafe { *pcVar3 = *pcVar3 + b_var14 + ((local_AX_15 - 0x24) < 0x50) };
    b_var26 = (local_AX_15 + 0x8c) < 0x50;
    b_var5 = local_AX_15 + 0x3c;
    pb_var1 = 0x502c;
    b_var20 = (i_var19 >> 8);
    unsafe {
        b_var27 = CARRY1(*pb_var1, b_var20) || CARRY1(*pb_var1 + b_var20, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var20 + b_var26 };
        pb_var1 = (local_BX__1 + unaff_si + 0x2c);
        b_var26 = CARRY1(*pb_var1, b_var5) || CARRY1(*pb_var1 + b_var5, b_var27);
        unsafe { *pb_var1 = *pb_var1 + b_var5 + b_var27 };
        pb_var1 = ((register0x00000010 + 0x2a) + unaff_si);
        b_var27 = CARRY1(*pb_var1, b_var13) || CARRY1(*pb_var1 + b_var13, b_var26);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var13 + b_var26 };
    pb_var1 = (unaff_si + 0x2c);
    unsafe { b_var6 = *pb_var1 };
    b_var15 = i_var19;
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var15 + b_var27 };
    b_var26 = CARRY1(bStack002a, b_var22)
        || CARRY1(
            bStack002a + b_var22,
            CARRY1(b_var6, b_var15) || CARRY1(b_var18 + b_var15, b_var27),
        );
    pb_var1 = (local_BX__1 + unaff_si + 0x2c);
    unsafe {
        b_var27 = CARRY1(*pb_var1, b_var14) || CARRY1(*pb_var1 + b_var14, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var14 + b_var26 };
        pb_var1 = ((register0x00000010 + 0x2a) + unaff_si);
        b_var26 = CARRY1(*pb_var1, b_var20) || CARRY1(*pb_var1 + b_var20, b_var27);
        unsafe { *pb_var1 = *pb_var1 + b_var20 + b_var27 };
        pb_var1 = (unaff_si + 0x2c);
        b_var23 = (local_BX__1 >> 8);
        b_var27 = CARRY1(*pb_var1, b_var23) || CARRY1(*pb_var1 + b_var23, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var23 + b_var26 };
        b_var26 = CARRY1(in_stack_0000502a, b_var5) || CARRY1(in_stack_0000502a + b_var5, b_var27);
        b_var6 = in_stack_0000502a + b_var5 + b_var27;
        pb_var1 = (local_BX__1 + unaff_si + 0x502c);
        b_var27 = CARRY1(*pb_var1, b_var15) || CARRY1(*pb_var1 + b_var15, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var15 + b_var26 };
        pb_var1 = &stack0x502a + unaff_si;
        b_var26 = CARRY1(*pb_var1, b_var22) || CARRY1(*pb_var1 + b_var22, b_var27);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var22 + b_var27 };
    pb_var1 = (unaff_si + 0x502c);
    unsafe { b_var18 = *pb_var1 };
    unsafe { b_var17 = *pb_var1 };
    unsafe { *pb_var1 = b_var17 + b_var11 + b_var26 };
    b_var26 = CARRY1(b_var6, b_var14)
        || CARRY1(
            b_var6 + b_var14,
            CARRY1(b_var18, b_var11) || CARRY1(b_var17 + b_var11, b_var26),
        );
    pb_var1 = (local_BX__1 + unaff_si + 0x502c);
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var23 + b_var26 };
    cVar16 = b_var15 + b_var5 + (CARRY1(b_var6, b_var23) || CARRY1(b_var18 + b_var23, b_var26));
    cVar12 = b_var11 + b_var13 + (b_var5 < 0x50);
    c_var21 = b_var20 + cVar16 + ((local_AX_15 - 0x14) < 0x50);
    local_AX_111._0_1_ = local_AX_15 + 0x4c + cVar12 + ((local_AX_15 + 0x9c) < 0x50);
    b_var17 = cVar16 + b_var14 + (local_AX_111 < 0x50);
    b_var11 = c_var21 + b_var23 + ((local_AX_111 + 0x60) < 0x50);
    u_var7 = CONCAT11(
        cVar12 + c_var21 + ((local_AX_111 + 0xb0) < 0x50),
        local_AX_111 - 0x40,
    );
    pcVar3 = unaff_si + &local_BX__1.field_0x0;
    unsafe { *pcVar3 = *pcVar3 + b_var13 + ((local_AX_111 + 0x10) < 0x50) };
    pb_var1 = unaff_DI;
    unsafe {
        b_var6 = b_var14 + *pb_var1;
    }
    b_var5 = b_var6 + (u_var7 < 0x1050);
    pcVar3 = unaff_si;
    unsafe {
        *pcVar3 = *pcVar3 + b_var22 + (CARRY1(b_var14, *pb_var1) || CARRY1(b_var6, u_var7 < 0x1050))
    };
    pb_var1 = unaff_DI;
    unsafe { *pb_var1 = *pb_var1 ^ b_var5 };
    pcVar3 = &stack0xfffe + unaff_si;
    unsafe { *pcVar3 = *pcVar3 + b_var23 };
    pc_var25 = unaff_si + -1;
    u_var7 = CONCAT11(local_AX_111 + 0x70, (u_var7 + 0xcf10 >> 8));
    u_var8 = u_var7 + 0xefb0;
    b_var14 = u_var8;
    cVar12 = (u_var8 >> 8);
    pb_var1 = (unaff_si + 0x2c);
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var5 + (u_var7 < 0x1050) };
    b_var26 = CARRY1(bStack002b, b_var11)
        || CARRY1(
            bStack002b + b_var11,
            CARRY1(b_var6, b_var5) || CARRY1(b_var18 + b_var5, u_var7 < 0x1050),
        );
    pb_var1 = (local_BX__1 + pc_var25 + 0x502d);
    unsafe {
        b_var27 = CARRY1(*pb_var1, b_var14) || CARRY1(*pb_var1 + b_var14, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var14 + b_var26 };
        pb_var1 = &stack0x502b + pc_var25;
        b_var26 = CARRY1(*pb_var1, b_var13) || CARRY1(*pb_var1 + b_var13, b_var27);
        unsafe { *pb_var1 = *pb_var1 + b_var13 + b_var27 };
        pb_var1 = (unaff_si + 0x502c);
        unsafe { b_var6 = *pb_var1 };
        unsafe { b_var18 = *pb_var1 };
        unsafe { *pb_var1 = b_var18 + b_var17 + b_var26 };
        b_var26 = CARRY1(in_stack_0000502b, b_var22)
            || CARRY1(
                in_stack_0000502b + b_var22,
                CARRY1(b_var6, b_var17) || CARRY1(b_var18 + b_var17, b_var26),
            );
        pb_var1 = (local_BX__1 + pc_var25 + 0x502d);
        b_var27 = CARRY1(*pb_var1, b_var5) || CARRY1(*pb_var1 + b_var5, b_var26);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var5 + b_var26 };
    pb_var1 = &stack0x502b + pc_var25;
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var11 + b_var27 };
    pcVar3 = unaff_si + 0x502c;
    unsafe {
        *pcVar3 =
            *pcVar3 + b_var23 + (CARRY1(b_var6, b_var11) || CARRY1(b_var18 + b_var11, b_var27))
    };
    i_var19 = u_var7 + 0xdf60;
    pb_var1 = unaff_DI;
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 >> 1 };
    b_var18 = b_var17 + b_var22 + (b_var6 & 1);
    b_var6 = _in(0x2d);
    u_var9 = (u_var7 + 0xcf10 & 0xff00 | b_var6) + 0xdf60;
    b_var5 = b_var5 + *0x1050;
    pc_var10 = (u_var9 | 0x2e);
    pb_var1 = 0x502e;
    unsafe {
        b_var26 = CARRY1(*pb_var1, b_var18);
        unsafe { *pb_var1 = *pb_var1 + b_var18 };
        pb_var1 = (pc_var25 + &local_BX__1.field_0x0);
        b_var17 = (u_var9 >> 8);
        b_var27 = CARRY1(*pb_var1, b_var17) || CARRY1(*pb_var1 + b_var17, b_var26);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var17 + b_var26 };
    pb_var1 = &stack0xfffe + pc_var25;
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 + b_var5 };
    unsafe { *pb_var1 = b_var18 + b_var27 };
    pcVar3 = &stack0xfffe + pc_var25;
    unsafe { *pcVar3 = *pcVar3 + b_var17 + (CARRY1(b_var6, b_var5) || CARRY1(b_var18, b_var27)) };
    pcVar3 = pc_var10 + u_var7 + 0xe364;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + u_var7 + 0xdf64;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pu_var2 = (pc_var10 + i_var19);
    unsafe { *pu_var2 = *pu_var2 | u_var8 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    i_var19 = CONCAT11((i_var19 >> 8), 4);
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    b_var18 = b_var14 ^ pc_var10[i_var19];
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pb_var1 = (pc_var10 + i_var19);
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 + b_var18 };
    piVar4 = (pc_var10 + i_var19);
    unsafe { *piVar4 = (*piVar4 - CONCAT11(4, b_var18)) - CARRY1(b_var6, b_var18) };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    b_var18 = b_var18 ^ pc_var10[i_var19];
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pb_var1 = (pc_var10 + 0x404);
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 + b_var18 };
    piVar4 = (pc_var10 + 0x404);
    unsafe { *piVar4 = (*piVar4 - CONCAT11(4, b_var18)) - CARRY1(b_var6, b_var18) };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x508;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2 + 0x104);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2 + 4);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    cVar12 = b_var14 + 4;
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + 0x104;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + 4;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    cVar12 = cVar12 + b_var14;
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = 0x608;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    cVar12 = cVar12 * 0x2;
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    c_var21 = cVar12 + 0x4;
    i_var19 = CONCAT11(4, c_var21);
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + c_var21 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + c_var21 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 + 0x1 };
    cVar16 = cVar12 + 0x4;
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + c_var21 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    i_var19 = CONCAT11(cVar12 + 0x8, c_var21);
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + b_var14 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    unsafe { *0x2 = pc_var10 };
    i_var19 = i_var19 + -1;
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    pcVar3 = pc_var10;
    unsafe { *pcVar3 = *pcVar3 + cVar12 + 0x8 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    return;
}

pub unsafe fn pass1_1050_3344() {
    let pc_var1: *mut libc::c_char;
    let pu8_var2: *mut u8;
    let pi_var3: *mut i32;
    let mut b_var4: u8;
    let mut b_var5: u8;
    let mut in_ax: i32;
    let mut u_var6: u16;
    let mut extraout_DH: u8;
    let mut in_b_x: i32;
    let unaff_s_i: *mut libc::c_char;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    b_var4 = in_ax;
    out(4, b_var4);
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + b_var4 };
    b_var4 = b_var4 ^ unaff_s_i[in_b_x];
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + b_var4 };
    out(4, in_ax & 0xff00 | b_var4);
    pu8_var2 = (unaff_s_i + in_b_x);
    unsafe { b_var5 = b_var4 + *pu8_var2 };
    pi_var3 = (unaff_s_i + in_b_x);
    unsafe { *pi_var3 = (*pi_var3 - (in_ax & 0xff00 | b_var5)) - CARRY1(b_var4, *pu8_var2) };
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + b_var5 };
    u_var6 = bad_fn_1050_335f();
    pc_var1 = &stack0xfffe + unaff_s_i;
    unsafe { *pc_var1 = *pc_var1 + extraout_DH };
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + u_var6 };
    pc_var1 = unaff_s_i;
    unsafe { *pc_var1 = *pc_var1 + (u_var6 >> 8) };
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + u_var6 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub unsafe fn pass1_1048_1e02() -> i32 {
    let pc_var1: *mut libc::c_char;
    let mut c_var2: u8;
    let mut in_a_x: i32;
    let mut in_b_x: i32;
    let pu_var3: *mut u16;
    let unaff_b_p: *mut u16;
    let mut unaff_s_i: i32;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    pu_var3 = &stack0xfffe;
    c_var2 = '\x0f';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_b_p };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    pc_var1 = (in_b_x + unaff_s_i);
    c_var2 = in_a_x;
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    return in_a_x + (in_b_x + unaff_s_i);
}

pub unsafe fn pass1_1048_3c50() {
    let pi_var1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let mut c_var3: u8;
    let mut in_a_x: u16;
    let mut i_var4: i32;
    let mut in_c_x: u16;
    let mut in_d_x: u16;
    let in_b_x: *mut libc::c_char;
    let mut c_var5: u8;
    let pu_var6: *mut u16;
    let unaff_b_p: *mut u16;
    let unaff_s_i: *mut libc::c_char;
    let unaff_d_i: *mut libc::c_char;
    let mut unaff_e_s: u16;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    pu_var6 = &stack0xfffe;
    c_var3 = '\x1e';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var6 = pu_var6 + -1;
        unsafe { *pu_var6 = *unaff_b_p };
        c_var3 = c_var3 + -1;
        '\0' < c_var3
    } {}
    out(in_d_x, in_a_x);
    pi_var1 = (in_b_x + unaff_d_i);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    pc_var2 = unaff_d_i;
    unsafe { *pc_var2 = *pc_var2 + in_d_x };
    pc_var2 = in_b_x + unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + (in_c_x >> 8) };
    pc_var2 = &stack0xfffe + unaff_s_i;
    c_var5 = (in_b_x >> 8);
    unsafe { *pc_var2 = *pc_var2 + c_var5 };
    pc_var2 = &stack0xfffe + unaff_d_i;
    unsafe { *pc_var2 = *pc_var2 + in_c_x };
    pc_var2 = unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + (in_a_x >> 8) };
    c_var3 = _in(in_d_x);
    unsafe { *unaff_d_i = c_var3 };
    pc_var2 = in_b_x + (unaff_d_i + 1);
    unsafe { *pc_var2 = *pc_var2 + (in_d_x >> 8) };
    i_var4 = (in_b_x + unaff_s_i) * 0x62;
    pc_var2 = in_b_x;
    unsafe { *pc_var2 = *pc_var2 + in_d_x };
    pc_var2 = in_b_x + unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + c_var5 };
    pc_var2 = 0x1300;
    unsafe { *pc_var2 = *pc_var2 + (i_var4 >> 8) };
    pc_var2 = in_b_x + unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + i_var4 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub unsafe fn pass1_1048_3da0() {
    let ppc_var1: fn();
    let mut c_var2: u8;
    let in_a_l: u8;
    let mut in_b_x: i32;
    let pu_var3: *mut u16;
    let unaff_b_p: *mut u16;
    let mut unaff_s_i: i32;
    let unaff_d_i: *mut u8;
    let mut unaff_e_s: u16;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;
    let mut local_6e03: u16;
    let mut local_6703: u16;
    let pu_stack64: *mut u8;

    pu_stack64 = &stack0xfffe;
    pu_var3 = &stack0xfffe;
    c_var2 = '\x1e';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_b_p };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    unsafe { *unaff_d_i = in_a_l };
    ppc_var1 = (in_b_x + -0x6801);
    (**ppc_var1)();
    (**(&local_6e03 + unaff_s_i))();
    (**(&local_6703 + unaff_s_i))();
    // WARNING: Could not recover jumptable at 0x10483db1. Too many branches
    // WARNING: Treating indirect jump as call
    (**(in_b_x + unaff_s_i + -0x5401))();
    return;
}

pub unsafe fn pass1_1048_3de8() {
    let mut c_var1: u8;
    let mut in_a_x: u16;
    let mut in_b_x: i32;
    let p_uvar2: *mut u16;
    let unaff_b_p: *mut u16;
    let mut unaff_s_i: i32;
    let unaff_d_i: *mut u16;
    let mut unaff_e_s: u16;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    pu_var2 = &stack0xfffe;
    c_var1 = '\x1e';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_b_p };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    } {}
    unsafe { *unaff_d_i = in_a_x };
    // WARNING: Could not recover jumptable at 0x10483ded. Too many branches
    // WARNING: Treating indirect jump as call
    (**(in_b_x + unaff_s_i + -0x6701))();
    return;
}

pub unsafe fn pass1_1048_4072() {
    let mut c_var1: u8;
    let p_uvar2: *mut u16;
    let unaff_b_p: *mut u16;
    let mut unaff_d_i: i32;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;
    let mut local_5503: u16;
    let pu_stack64: *mut u8;

    pu_stack64 = &stack0xfffe;
    pu_var2 = &stack0xfffe;
    c_var1 = '\x1e';
    unsafe {
        unaff_b_p = unaff_b_p + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_b_p };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    }
    []
    // WARNING: Could not recover jumptable at 0x10484079. Too many branches
    // WARNING: Treating indirect jump as call
    (* * (&stack0xaaff + unaff_d_i))();
    return;
}

pub unsafe fn pass1_1040_d89e(param_1: u32, param_2: u8) {
    win_cleanup_1040_d1bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_d76e(param_1: *mut Struct330) {
    let mut u_var1: u32;
    let local_b_x_3: *mut Struct330;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    local_b_x_3 = param_1;
    u_var1 = local_b_x_3.field_0x94;
    pass1_1018_5742(
        u_var1,
        (u_var1 >> 0x10),
        local_b_x_3.field_0x9c,
        local_b_x_3.field_0x98,
    );
    local_b_x_3.field_0x9c = 0;
    return;
}

pub unsafe fn pass1_1040_d056(param_1: u32, param_2: u8) {
    pass1_1040_ca74(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_d0d8(param_1: u32) {
    let pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut b_var3: u8;
    let mut c_var4: u8;
    let mut u_var5: i32;
    let mut in_a_l: u8;
    let mut in_c_l: u8;
    let mut b_var6: u8;
    let mut in_d_x: i32;
    let mut b_var7: u8;
    let mut in_b_x: i32;
    let mut b_var8: u8;
    let pu_var9: *mut u16;
    let unaff_b_p: *mut u16;
    let unaff_s_i: *mut u8;
    let mut unaff_s_s: u16;
    let mut in_c_f: u8;
    let mut b_var10: bool;
    let mut b_var11: bool;
    let local_4e: u8;

    pu_var9 = &stack0xfffe;
    c_var4 = '\x0f';
    unsafe {
        unaff_b_p = unaff_b_p + -1;
        pu_var9 = pu_var9 + -1;
        unsafe { *pu_var9 = *unaff_b_p };
        c_var4 = c_var4 + -1;
        '\0' < c_var4
    }
    {}
    b_var8 = (in_b_x >> 8);
    unaff_s_i[in_b_x] = b_var8;
    b_var7 = (in_d_x + 1 >> 8);
    b_var2 = b_var7 + b_var8;
    b_var10 = CARRY1(b_var7, b_var8) || CARRY1(b_var2, in_c_f);
    u_var5 = in_d_x + 1 & 0xff;
    pb_var1 = unaff_s_i + in_b_x;
    b_var6 = u_var5;
    unsafe { b_var7 = *pb_var1 };
    unsafe { b_var3 = *pb_var1 - b_var6 };
    unsafe { b_var11 = *pb_var1 < b_var6 || b_var3 < b_var10 };
    unsafe { *pb_var1 = b_var3 - b_var10 };
    let pb_var1_val = unsafe { *pb_var1 };
    if (((pb_var1_val != 0) && SBORROW1(b_var7, b_var6))
        != (SBORROW1(b_var3, b_var10) == (pb_var1_val < '\0')))
    {
        pb_var1 = unaff_s_i;
        unsafe { b_var7 = *pb_var1 };
        unsafe { b_var3 = *pb_var1 };
        unsafe { *pb_var1 = b_var3 + b_var8 + b_var11 };
        b_var10 = CARRY1(local_4e, in_b_x)
            || CARRY1(
                local_4e + in_b_x,
                CARRY1(b_var7, b_var8) || CARRY1(b_var3 + b_var8, b_var11),
            );
        pb_var1 = unaff_s_i + -0x4f;
        unsafe { b_var7 = *pb_var1 };
        unsafe { b_var3 = *pb_var1 };
        unsafe { *pb_var1 = b_var3 + b_var8 + b_var10 };
        return CONCAT22(
            u_var5
                | (b_var2
                    + in_c_f
                    + in_c_l
                    + (CARRY1(b_var7, b_var8) || CARRY1(b_var3 + b_var8, b_var10)))
                    << 8,
            CONCAT11(0x40, in_a_l + 0x1) + 2,
        );
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_d0f8(param_1: *mut u16, param_2: u16) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let struct_a: *mut Struct199;
    let pa_var4: *mut Struct199;
    let mut extraout_d_x: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let pp_var7: *mut pass1_struct_1;
    let mut in_stack_0000fffa: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    (i_var5 + 0x94) = 0;
    (i_var5 + 0x98) = _ctx.PTR_LOOP_1050_5f16;
    (i_var5 + 0x9c) = 0;
    (i_var5 + 0xa0) = 0;
    unsafe { *param_1 = 0xd8c4 };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1040;
    pp_var7 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fffa, 0x47),
    );
    u_var3 = (pp_var7 >> 0x10);
    u_var1 = SUB42(pp_var7, 0);
    (i_var5 + 0x94) = u_var1;
    (i_var5 + 0x96) = u_var3;
    pass1_1018_5732((i_var5 + 0x94), u_var3, (i_var5 + 0x98));
    (i_var5 + 0x9c) = u_var1;
    (i_var5 + 0x9e) = struct_a;
    u_var2 = struct_a | (i_var5 + 0x9c);
    if (u_var2 == 0) {
        pa_var4 = struct_a;
        process_struct_1000_179c(0xc, struct_a);
        if ((pa_var4 | u_var2) == 0) {
            (i_var5 + 0x9c) = 0;
        } else {
            pass1_1010_8ef2(CONCAT22(pa_var4, u_var2));
            (i_var5 + 0x9c) = u_var2;
            (i_var5 + 0x9e) = extraout_d_x;
        }
    }
    return;
}

pub unsafe fn pass1_1040_ca74(param_1: *mut u16) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0xd07c };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    PTR_LOOP_1050_5f10 = 0x0;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1040_ca16(param_1: *mut Struct331, param_2: u16) {
    let mut i_var1: i32;
    let mut unaff_b_p: u16;
    let mut u_var2: u16;
    let pp_var3: *mut pass1_struct_1;
    let local_a: *mut Struct331;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1840));
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = _ctx.PTR_LOOP_1050_5f0c;
    (i_var1 + 0x98) = 0;
    (i_var1 + 0x9c) = 0;
    (i_var1 + 0x9e) = 0;
    param_1.field_0x0 = 0xd07c;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pp_var3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_b_p, 0x3e));
    (i_var1 + 0x98) = pp_var3;
    (i_var1 + 0x9a) = (pp_var3 >> 0x10);
    return;
}

pub unsafe fn pass1_1040_c9cc(param_1: u32, param_2: u8) {
    pass1_1040_c5ac(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

pub unsafe fn pass1_1040_ca03() {
    let pb_var1: *mut u8;
    let pc_var2: *mut libc::c_char;
    let mut b_var3: u8;
    let mut b_var4: u8;
    let pu_var5: *mut u16;
    let mut c_var6: u8;
    let pu_var7: *mut u16;
    let mut u_var8: u32;
    let mut in_a_l: u8;
    let mut b_var9: u8;
    let mut in_d_x: i32;
    let mut in_b_l: u8;
    let local_b_x_41: *mut Struct332;
    let local_b_x_92: *mut Struct333;
    let pu_var10: *mut u16;
    let unaff_b_p: *mut u16;
    let mut unaff_s_i: i32;
    let mut unaff_d_i: i32;
    let mut u_var11: u16;
    let mut unaff_s_s: u16;
    let mut in_c_f: u8;
    let mut b_var12: bool;
    let mut b_var13: bool;
    let pp_var14: *mut pass1_struct_1;
    let mut in_stack_00000000: u16;

    pu_var10 = &stack0xfffe;
    c_var6 = '\t';
    pu_var5 = unaff_b_p;
    while {
        pu_var5 = pu_var5 + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *pu_var5 };
        c_var6 = c_var6 + -1;
        '\0' < c_var6
    } {}
    b_var9 = in_d_x * 0x2 + in_c_f;
    b_var12 = (in_a_l + 1) < 8;
    pb_var1 = (unaff_b_p + unaff_s_i + -0x38);
    unsafe {
        b_var13 = CARRY1(*pb_var1, in_b_l) || CARRY1(*pb_var1 + in_b_l, b_var12);
    }
    unsafe { *pb_var1 = *pb_var1 + in_b_l + b_var12 };
    pb_var1 = (unaff_b_p + unaff_s_i + 0x40c8);
    unsafe { b_var3 = *pb_var1 };
    unsafe { b_var4 = *pb_var1 + in_a_l + 2 };
    unsafe { *pb_var1 = b_var4 + b_var13 };
    pc_var2 = (unaff_d_i + -0x75);
    unsafe {
        *pc_var2 = *pc_var2 + b_var9 + (CARRY1(b_var3, in_a_l + 2) || CARRY1(b_var4, b_var13))
    };
    _in(in_d_x & 0xff00 | b_var9);
    process_struct_1040_b082((unaff_b_p + 3), CONCAT22(unaff_b_p[5], 0x1840));
    pu_var7 = (unaff_b_p + 3);
    u_var11 = (pu_var7 >> 0x10);
    local_b_x_41 = pu_var7;
    local_b_x_41.field_0x94 = _ctx.PTR_LOOP_1050_5f0c;
    local_b_x_41.field_0x98 = 0;
    local_b_x_41.field_0x9c = 0;
    local_b_x_41.field_0x9e = 0;
    unsafe { *pu_var7 = 0xd07c };
    local_b_x_41.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pp_var14 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_00000000, 0x3e),
    );
    u_var8 = (unaff_b_p + 3);
    u_var11 = (u_var8 >> 0x10);
    local_b_x_92 = u_var8;
    local_b_x_92.field_0x98 = pp_var14;
    local_b_x_92.field_0x9a = (pp_var14 >> 0x10);
    return;
}

pub unsafe fn pass1_1040_c71e(param_1: *mut Struct335) {
    let local_b_x_12: *mut Struct335;
    let mut u_var1: u16;

    process_struct_1040_9252(param_1);
    u_var1 = (param_1 >> 0x10);
    local_b_x_12 = param_1;
    local_b_x_12.field_0x28 = local_b_x_12.field_0x24 / 2 - local_b_x_12.field_0x2c / 2;
    return;
}

pub unsafe fn pass1_1040_c54a(param_1: u32, param_2: u16, in_Struct3: *mut Struct114) {
    let ppc_var1: fn();
    let mut i_var2: i32;
    let mut extraout_d_x: u16;
    let local_b_x_7: *mut Struct114;
    let local_b_x_49: *mut Struct113;
    let mut u_var3: i32;
    let in_Struct1: *mut Struct74;
    let pa_var4: *mut Struct114;
    let mut u_var5: u16;
    let mut u_var6: u32;

    local_b_x_7 = in_Struct3;
    i_var2 = local_b_x_7.field_0x12 + 200;
    u_var6 = 0;
    u_var5 = 0;
    ppc_var1 = (in_Struct3 + 0x14);
    pa_var4 = in_Struct3;
    (**ppc_var1)();
    in_Struct1 = param_1;
    make_proc_inst_1040_8fb8(
        in_Struct1,
        0,
        CONCAT22(extraout_d_x, i_var2),
        pa_var4,
        (pa_var4 >> 0x10),
        u_var5,
        u_var6,
        (u_var6 >> 0x10),
    );
    u_var3 = (param_1 >> 0x10);
    (in_Struct1 + 1) = in_Struct3;
    in_Struct1[1].field_0x4 = 0;
    in_Struct1[1].field_0x6 = param_2;
    param_1 = 0xc9f2;
    in_Struct1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    process_struct_1040_c630((param_1 & 0xffff | u_var3 << 0x10));
    return;
}

pub unsafe fn pass1_1040_c5ac(param_1: *mut Struct337) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_b_x_5: *mut Struct337;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_b_x_5 = param_1;
    param_1 = 0xc9f2;
    local_b_x_5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + -1;
    if (PTR_LOOP_1050_5f02 == 0x0) {
        pu_var1 = local_b_x_5.field_0x8;
        u_var2 = local_b_x_5.field_0xa;
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppc_var3 = *pu_var1 };
            (**ppc_var3)();
        }
        pu_var1 = local_b_x_5.field_0xc;
        u_var2 = local_b_x_5.field_0xe;
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppc_var3 = *pu_var1 };
            (**ppc_var3)();
        }
    }
    free_proc_inst_1040_911e(param_1);
    return;
}

pub unsafe fn pass1_1040_c60e(param_1: &mut Address<u8>) -> u32 {
    let mut u_var1  = param_1.get_u32(0x42).unwrap();
    if (u_var1) != 0 {
        return u_var1 + 0x12;
    }
    return 0;
}

pub unsafe fn pass1_1040_bfde(param_1: *mut Struct338, param_2: u32) {
    let ppc_var1: fn();
    let mut u_var2: u32;
    let local_b_x_15: *mut Struct338;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    local_b_x_15 = param_1;
    local_b_x_15.field_0x6 = param_2;
    ppc_var1 = (param_2 + 4);
    (**ppc_var1)();
    u_var2 = local_b_x_15.field_0x6;
    (u_var2 + 0x22) = local_b_x_15.field_0x4;
    pass1_1010_2ee2(local_b_x_15.field_0x6);
    return;
}

pub unsafe fn pass1_1040_be94(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_bb5a(param_1: u32) {
    let fn_ptr_1: fn();

    fn_ptr_1 = ((param_1 + 0x94) + 8);
    (**fn_ptr_1)();
    return 0;
}

pub unsafe fn pass1_1040_b864(
    param_1: *mut u32,
    param_2: *mut i32,
    param_3: u16,
    param_4: u16,
    param_5: *mut u8,
) {
    let ppc_var1: fn();
    let u_var2: u8;
    let extraout_a_h: u8;

    if (param_5 == (s_New_failed_in_Op__Op_1050_0020 + 0xb)) {
        let param_2_val = unsafe { *param_2 };
        if (param_2_val == 4) {
            get_prop_1040_9566(param_2, param_3);
        }
    } else {
        if (param_5 != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1)) {
            u_var2 = pass1_1040_b316(param_1, param_2, param_3, param_3_00, param_5);
            return CONCAT11(extraout_a_h, u_var2);
        }
        let param_1_val = unsafe { *param_1 };
        ppc_var1 = (param_1_val + 0x7c);
        (**ppc_var1)();
    }
    return 1;
}

pub unsafe fn pass1_1040_b74c(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_b7ce(param_1: u32) {
    let pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut c_var3: u8;
    let mut u_var4: i32;
    let mut in_c_x: u16;
    let mut b_var5: u8;
    let mut in_d_x: i32;
    let mut b_var6: u8;
    let mut in_b_x: i32;
    let mut b_var7: u8;
    let local_bx_55: *mut Struct343;
    let pu_var8: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: *mut u8;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var11: bool;
    let mut b_var12: bool;
    let paVar13: *mut Struct346;
    let lVar14: u32;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    pu_var8 = &stack0xfffe;
    c_var3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var8 = pu_var8 + -1;
        unsafe { *pu_var8 = *unaff_bp };
        c_var3 = c_var3 + -1;
        '\0' < c_var3
    } {}
    b_var7 = (in_b_x >> 8);
    unaff_si[in_b_x] = b_var7;
    b_var6 = (in_d_x + 1 >> 8);
    b_var2 = b_var6 + b_var7;
    b_var11 = CARRY1(b_var6, b_var7) || CARRY1(b_var2, in_CF);
    u_var4 = in_d_x + 1 & 0xff;
    lVar14 = CONCAT22(u_var4 | (b_var2 + in_CF) << 8, in_b_x);
    paVar13 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pb_var1 = unaff_si + in_b_x;
    b_var5 = u_var4;
    unsafe { b_var2 = *pb_var1 };
    unsafe { b_var6 = *pb_var1 - b_var5 };
    unsafe { b_var12 = *pb_var1 < b_var5 || b_var6 < b_var11 };
    unsafe { *pb_var1 = b_var6 - b_var11 };
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0
        && (SBORROW1(b_var2, b_var5) != SBORROW1(b_var6, b_var11)) == (pb_var1_val < '\0'))
    {
        pb_var1 = unaff_si;
        b_var11 = CARRY1(pb_var1_val, b_var7) || CARRY1(pb_var1_val + b_var7, b_var12);
        unsafe { *pb_var1 = *pb_var1 + b_var7 + b_var12 };
        b_var2 = local_4e + in_b_x;
        b_var12 = CARRY1(local_4e, in_b_x) || CARRY1(b_var2, b_var11);
        local_4e = b_var2 + b_var11;
        pb_var1 = unaff_si + -0x4f;
        unsafe { b_var2 = *pb_var1 };
        unsafe { b_var6 = *pb_var1 };
        unsafe { *pb_var1 = b_var6 + b_var7 + b_var12 };
        pb_var1 = unaff_si + -0x78;
        unsafe {
            *pb_var1 =
                *pb_var1 + in_c_x + (CARRY1(b_var2, b_var7) || CARRY1(b_var6 + b_var7, b_var12))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_b0bc(paVar13, 0, CONCAT22(in_c_x, 0xfab));
        u_var9 = (paVar13 >> 0x10);
        local_bx_55 = paVar13;
        local_bx_55.field_0x94 = 0;
        local_bx_55.field_0x98 = 0;
        local_bx_55.field_0xb0 = 0;
        local_bx_55.field_0xb4 = 0;
        local_bx_55.field_0xb6 = 0;
        paVar13 = 0xbeba;
        local_bx_55.field_0x2 = &ctx.PTR_LOOP_1050_1040;
        if (lVar14 != 0) {
            u_var10 = (lVar14 >> 0x10);
            local_bx_55.field_0xb0 = (lVar14 + 6);
            local_bx_55.field_0xb4 = (lVar14 + 0x14);
        }
        return;
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1040_b316(
    param_1: *mut u32,
    param_2: *mut u8,
    param_3: *mut u8,
    param_3_00: *mut u8,
    param_5: *mut u8,
) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut local_4: u16;

    if (param_5 == (&PTR_LOOP_1050_000e + 1)) {
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x60);
        local_4._0_1_ = (**pp_var1)();
    } else {
        if (param_5 == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1)) {
            let param_1_val = unsafe { *param_1 };
            pp_var1 = (param_1_val + 0x10);
            (**pp_var1)();
            local_4._0_1_ = 0x1;
        } else {
            u_var2 = pass1_1040_79c0(param_1, param_2, param_3, param_3_00, param_5);
            local_4._0_1_ = u_var2;
        }
    }
    return local_4;
}

pub unsafe fn pass1_1040_af9e(param_1: u32, param_2: u8) {
    pass1_1040_ace8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_b020(param_1: u32) {
    let pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u_var4: i32;
    let in_cx: *mut u8;
    let mut b_var5: u8;
    let mut in_dx: i32;
    let mut u_var6: i32;
    let mut in_bx: i32;
    let mut b_var9: u8;
    let mut i_var8: i32;
    let pu_var10: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: *mut u8;
    let mut u_var11: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var12: bool;
    let mut b_var13: bool;
    let in_struct_1: *mut Struct68;
    let mut u_var14: u32;
    let local_4e: u8;
    let puStack34: *mut u8;
    let mut b_var7: u8;

    puStack34 = &stack0xfffe;
    pu_var10 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_bp };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    b_var9 = (in_bx >> 8);
    unaff_si[in_bx] = b_var9;
    b_var7 = (in_dx + 1 >> 8);
    b_var2 = b_var7 + b_var9;
    b_var12 = CARRY1(b_var7, b_var9) || CARRY1(b_var2, in_CF);
    u_var4 = in_dx + 1 & 0xff;
    u_var6 = u_var4 | (b_var2 + in_CF) << 8;
    u_var14 = CONCAT22(u_var6, in_bx);
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pb_var1 = unaff_si + in_bx;
    b_var5 = u_var4;
    unsafe { b_var2 = *pb_var1 };
    unsafe { b_var7 = *pb_var1 - b_var5 };
    unsafe { b_var13 = *pb_var1 < b_var5 || b_var7 < b_var12 };
    unsafe { *pb_var1 = b_var7 - b_var12 };
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0
        && (SBORROW1(b_var2, b_var5) != SBORROW1(b_var7, b_var12)) == (pb_var1_val < '\0'))
    {
        pb_var1 = unaff_si;
        b_var12 = CARRY1(pb_var1_val, b_var9) || CARRY1(pb_var1_val + b_var9, b_var13);
        unsafe { *pb_var1 = *pb_var1 + b_var9 + b_var13 };
        b_var2 = local_4e + in_bx;
        b_var13 = CARRY1(local_4e, in_bx) || CARRY1(b_var2, b_var12);
        local_4e = b_var2 + b_var12;
        pb_var1 = unaff_si + -0x4f;
        unsafe { b_var2 = *pb_var1 };
        unsafe { b_var7 = *pb_var1 };
        unsafe { *pb_var1 = b_var7 + b_var9 + b_var13 };
        pb_var1 = unaff_si + -0x78;
        unsafe {
            *pb_var1 =
                *pb_var1 + in_cx + (CARRY1(b_var2, b_var9) || CARRY1(b_var7 + b_var9, b_var13))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_7728(
            in_struct_1,
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            *(in_bx + 0x12),
            in_cx,
        );
        u_var11 = (in_struct_1 >> 0x10);
        i_var8 = in_struct_1;
        (i_var8 + 0x8e) = 0;
        (i_var8 + 0x90) = u_var14;
        in_struct_1.field_0x0 = 0xb772;
        (i_var8 + 2) = &ctx.PTR_LOOP_1050_1040;
        return;
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1040_b040(param_1: *mut u16, param_2: u32, param_3: u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        *(param_2 + 0x12),
        param_3,
    );
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8e) = 0;
    (i_var1 + 0x90) = param_2;
    unsafe { *param_1 = 0xb772 };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub unsafe fn pass1_1040_ac64(param_1: u32) {
    byte * *ppb_var1;
    let pu8_var2: *mut u8;
    let mut b_var3: u8;
    let mut cVar4: u8;
    let mut b_var5: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut b_var6: u8;
    let mut in_bx: i32;
    let mut b_var8: u8;
    let mut i_var7: i32;
    let pu_var9: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: *mut u8;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var11: bool;
    let mut b_var12: bool;
    let ppVar13: *mut pass1_struct_1;
    let pu_var14: *mut u8;
    let pu_var15: *mut u16;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    pu_var9 = &stack0xfffe;
    pu_var14 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var9 = pu_var9 + -1;
        unsafe { *pu_var9 = *unaff_bp };
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    b_var8 = (in_bx >> 8);
    unaff_si[in_bx] = b_var8;
    b_var6 = ((in_dx + 1) >> 8);
    b_var11 = CARRY1(b_var6, b_var8) || CARRY1(b_var6 + b_var8, in_CF);
    b_var5 = (in_dx + 1);
    pu_var15 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var2 = unaff_si + in_bx;
    unsafe { b_var6 = *pu8_var2 };
    unsafe { b_var3 = *pu8_var2 - b_var5 };
    unsafe { b_var12 = *pu8_var2 < b_var5 || b_var3 < b_var11 };
    unsafe { *pu8_var2 = b_var3 - b_var11 };
    let pu8_var2_val = unsafe { *pu8_var2 };
    if (pu8_var2_val != 0
        && (SBORROW1(b_var6, b_var5) != SBORROW1(b_var3, b_var11)) == (pu8_var2_val < '\0'))
    {
        pu8_var2 = unaff_si;
        b_var11 = CARRY1(pu8_var2_val, b_var8) || CARRY1(pu8_var2_val + b_var8, b_var12);
        unsafe { *pu8_var2 = *pu8_var2 + b_var8 + b_var12 };
        b_var6 = local_4e + in_bx;
        b_var12 = CARRY1(local_4e, in_bx) || CARRY1(b_var6, b_var11);
        local_4e = b_var6 + b_var11;
        pu8_var2 = unaff_si + -0x4f;
        unsafe { b_var6 = *pu8_var2 };
        unsafe { b_var3 = *pu8_var2 };
        unsafe { *pu8_var2 = b_var3 + b_var8 + b_var12 };
        pu8_var2 = unaff_si + -0x78;
        unsafe {
            *pu8_var2 =
                *pu8_var2 + in_CL + (CARRY1(b_var6, b_var8) || CARRY1(b_var3 + b_var8, b_var12))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_b082(CONCAT22(&stack0xbf2a, &stack0xfffe), CONCAT22(in_bx, 499));
        u_var10 = (pu_var15 >> 0x10);
        i_var7 = pu_var15;
        (i_var7 + 0x94) = 0;
        (i_var7 + 0x98) = 0;
        unsafe { *pu_var15 = 0xafc4 };
        (i_var7 + 2) = &ctx.PTR_LOOP_1050_1040;
        (i_var7 + 0x94) = _PTR_LOOP_1050_5ef0;
        _PTR_LOOP_1050_5ef0 = 0;
        ppVar13 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var14, 0x3d));
        u_var10 = (pu_var15 >> 0x10);
        (pu_var15 + 0x98) = ppVar13;
        (pu_var15 + 0x9a) = (ppVar13 >> 0x10);
        return;
    }
    ppb_var1 = (unaff_si + 9);
    unsafe { *ppb_var1 = unaff_si + *ppb_var1 };
    error_check_1000_17ce(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_ac84(param_1: *mut Struct350, param_2: u16) {
    let local_bx_18: *mut Struct350;
    let mut unaff_bp: u16;
    let mut u_var1: u16;
    let ppVar2: *mut pass1_struct_1;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 499));
    u_var1 = (param_1 >> 0x10);
    local_bx_18 = param_1;
    local_bx_18.field_0x94 = 0;
    &local_bx_18.field_0x98 = 0;
    param_1 = 0xafc4;
    local_bx_18.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    local_bx_18.field_0x94 = _PTR_LOOP_1050_5ef0;
    _PTR_LOOP_1050_5ef0 = 0;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x3d));
    local_bx_18.field_0x98 = ppVar2;
    local_bx_18.field_0x9a = (ppVar2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_ace8(in_struct_1: *mut Struct348) {
    let local_struct_1: *mut Struct348;
    let mut u_var1: u16;

    u_var1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.field_0x0 = 0xafc4;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_Struct112_a, local_struct_1.field_0x6);
    win_cleanup_func_1040_b0f8(in_struct_1);
    return;
}

pub unsafe fn pass1_1008_de58(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();
    let mut b_var2: bool;
    let local_AX_39: *mut Struct305;
    
    let struct_a: *mut Struct199;
    
    let paVar3: *mut Struct305;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    b_var2 = false;
    while {
        local_AX_39 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_39));
        struct_a = (ctx.dx_reg | local_AX_39);
        paVar3 = local_AX_39;
        if (struct_a == 0x0) {}
        // goto LAB_1008_dedb;
        ((local_AX_39.field_0x4 != param_3) || (local_AX_39.field_0x8 != param_2))
            && (local_AX_39.field_0x8 != param_3 || (local_AX_39.field_0x4 != param_2))
    } {}
    local_AX_39.field_0xc = 1;
    u_var6 = pass1_1030_8326();
    struct_a = (u_var6 >> 0x10);
    paVar3 = u_var6;
    local_AX_39.field_0xe = paVar3;
    local_AX_39.field_0x10 = struct_a;
    b_var2 = true;
    // LAB_1008_dedb:
    if (!b_var2) {
        process_struct_1000_179c(0x14, struct_a);
        if ((struct_a | paVar3) == 0) {
            paVar3 = 0x0;
            u_var5 = 0;
        } else {
            pass1_1008_dc90(CONCAT22(struct_a, paVar3), param_2, param_3);
            u_var5 = ctx.dx_reg;
        }
        paVar3.field_0xc = 1;
        u_var6 = pass1_1030_8326();
        paVar3.field_0xe = u_var6;
        paVar3.field_0x10 = (u_var6 >> 0x10);
        pp_var1 = ((param_1 + 10) + 4);
        (**pp_var1)();
    }
    return;
}

pub unsafe fn pass1_1008_df4a(param_1: u32) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut u_var3: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var2 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while (true) {
        u_var3 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var1 = (u_var3 >> 0x10);
        if (u_var3 == 0) {
            break;
        }
        if (((u_var3 + 0xc) == 2) || ((u_var3 + 0xc) == 3)) {
            pass1_1008_e9a4(param_1, u_var2, u_var3);
        }
    }
    return;
}

pub unsafe fn pass1_1008_dfa6(param_1: u32, param_2: u32, param_3: u32) {
    let local_AX_37: *mut Struct306;
    
    let mut unaff_ss: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while {
        local_AX_37 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_37));
        if ((ctx.dx_reg | local_AX_37) == 0) {
            return;
        }
        ((local_AX_37.field_0x4 != param_3) || (local_AX_37.field_0x8 != param_2))
            && (local_AX_37.field_0x8 != param_3 || (local_AX_37.field_0x4 != param_2))
    } {}
    if (local_AX_37.field_0xc != 1) {
        return;
    }
    return;
}

pub unsafe fn pass1_1008_e01c(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x1a) = param_2;
    return;
}

pub unsafe fn pass1_1008_e038(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x16);
    param_2 = (param_1 + 0x1a);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
