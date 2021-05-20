use crate::{defines::{Struct126, Struct145}, mixed_fn_1010_830a, string_ops1::{process_string_1000_28dc, process_string_1000_55b1}, util::CONCAT22};
use crate::app_context::AppContext;
use crate::bad_funcs::bad1::halt_baddata;
use crate::bad_funcs::bad2::bad_fn_1050_335f;
use crate::draw::draw1::process_struct_1040_9252;
use crate::err_funcs::{error_check_1000_0dc6, error_check_1000_17ce, handle_error_1008_9466};
use crate::file_funcs::file1::{read_file_1008_76e4, read_from_file_1038_7c02, write_to_file_1008_7c2a, write_to_file_1008_7cac, write_to_file_1008_7e1c, write_to_file_1038_7b20};
use crate::file_funcs::file2::{call_read_file_1020_a65e, call_write_to_file_1020_a644, read_file_1028_d7ba, read_file_1028_def2, read_file_1030_5c52, write_to_file_1028_d7a0, write_to_file_1028_dce2, write_to_file_1030_5c1a};
use crate::func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_256b, call_fn_ptr_1000_2594};
use crate::mem_funcs::{Address, alloc_mem_1000_07fc, alloc_mem_1000_0a48, alloc_mem_1000_167a, alloc_mem_1000_16aa, alloc_mem_1000_1708, alloc_mem_1008_909c};
use crate::other_funcs::{empty_fn_1000_55ac, zero_list_1008_3e38};
use crate::pass::{pass13_funcs, pass14_funcs, pass9_funcs};
use crate::pass::pass15_funcs::{pass1_1020_a43e, pass1_1020_a6ee};
use crate::pass::pass17_funcs::{pass1_1030_38f2, pass1_1030_4bbe, pass1_1030_8128, pass1_1030_8326, pass1_1030_8334, pass1_1030_8344};
use crate::pass::pass18_funcs::pass1_1038_3608;
use crate::pass::pass19_funcs::pass1_1040_79c0;
use crate::pass::pass20_funcs::{pass1_1010_8ef2, pass1_1010_905e};
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e2e0, pass1_1028_e4ec};
use crate::pass::pass6_funcs::{pass1_1038_4d28, pass1_1038_b6e0};
use crate::pass::pass7_funcs::{pass1_1018_20ee, pass1_1018_4842, pass1_1018_4cda, pass1_1018_4dce, pass1_1018_5732, pass1_1018_5742};
use crate::pass::pass8_funcs::{pass1_1008_e9a4, pass1_1010_1d80, pass1_1010_1f62, pass1_1010_2ee2};
use crate::prog_structs::prog_structs_1::{Struct104, Struct197, Struct283};
use crate::prog_structs::prog_structs_10::Struct252;
use crate::prog_structs::prog_structs_11::{Struct159, Struct218, Struct222};
use crate::prog_structs::prog_structs_12::{Struct102, Struct235, Struct266};
use crate::prog_structs::prog_structs_13::{Struct146, Struct279};
use crate::prog_structs::prog_structs_15::{Struct181, Struct194, Struct337};
use crate::prog_structs::prog_structs_16::{Struct174, Struct248};
use crate::prog_structs::prog_structs_17::{Struct284, Struct534};
use crate::prog_structs::prog_structs_18::{Struct195, Struct338};
use crate::prog_structs::prog_structs_19::Struct271;
use crate::prog_structs::prog_structs_2::{Struct131, Struct163, Struct199, Struct223, Struct293, Struct306, Struct7};
use crate::prog_structs::prog_structs_20::Struct253;
use crate::prog_structs::prog_structs_21::{Struct164, Struct240, Struct343, Struct350, Struct74};
use crate::prog_structs::prog_structs_22::Struct147;
use crate::prog_structs::prog_structs_23::{Struct184, Struct193, Struct267, Struct330};
use crate::prog_structs::prog_structs_24::{Struct103, Struct182, Struct2111, Struct236, Struct249};
use crate::prog_structs::prog_structs_25::{Struct211, Struct219, Struct272, Struct65};
use crate::prog_structs::prog_structs_26::{Struct183, Struct196, Struct206, Struct241, Struct254};
use crate::prog_structs::prog_structs_27::{pass1_struct_2, Struct165, Struct268, Struct285, Struct298, Struct301};
use crate::prog_structs::prog_structs_28::{FileObject, Struct148, Struct170, Struct207, Struct224, Struct237, Struct255, Struct260, Struct273, Struct290, Struct327, Struct331, Struct346};
use crate::prog_structs::prog_structs_29::{Struct114, Struct149, Struct153, Struct192, Struct212, Struct213, Struct220, Struct221, Struct225, Struct226, Struct227, Struct228, Struct229, Struct238, Struct242, Struct243, Struct256, Struct269, Struct286, Struct291, Struct299, Struct328, Struct332};
use crate::prog_structs::prog_structs_30::{Struct168, Struct172, Struct200, Struct201, Struct203, Struct230, Struct231, Struct232, Struct233, Struct234, Struct239, Struct244, Struct245, Struct246, Struct247, Struct257, Struct261, Struct274, Struct287, Struct292, Struct294, Struct295, Struct333, Struct417};
use crate::prog_structs::prog_structs_31::{Struct113, Struct126, Struct145, Struct156, Struct157, Struct158, Struct160, Struct161, Struct162, Struct169, Struct173, Struct175, Struct250, Struct251, Struct258, Struct259, Struct262, Struct263, Struct264, Struct275, Struct276, Struct277, Struct278, Struct280, Struct281, Struct282, Struct288, Struct289, Struct305, Struct329, Struct335, Struct348};
use crate::prog_structs::prog_structs_4::Struct270;
use crate::prog_structs::prog_structs_5::{Struct1, Struct150};
use crate::prog_structs::prog_structs_6::Struct675;
use crate::prog_structs::prog_structs_7::{Struct189, Struct376, Struct44};
use crate::prog_structs::prog_structs_8::{Struct265, Struct68};
use crate::sound_funcs::mci_send_cmd_1008_5c5c;
use crate::string_ops1::{copy_string_1000_3d3e, fn_1008_6048, get_string_index_1000_3da4, load_str_1010_84ac, process_string_1000_2a00, process_string_1000_440c, string_fn_1000_3f9c};
use crate::struct_ops1::{process_struct_1000_179c, process_struct_1000_2e74, process_struct_1000_2f00, process_struct_1008_41bc, process_struct_1008_4544, process_struct_1008_4772, process_struct_1008_47cc, process_struct_1008_4834, process_struct_1008_50c2, process_struct_1008_574a, process_struct_1008_8d8a, process_struct_1008_c882, process_struct_1010_1d48, process_struct_1040_b082, process_struct_1040_b0bc, process_struct_1040_c630, set_struct_1008_4016, set_struct_1008_56b4, set_struct_1008_9584, struct_fn_1000_160a};
use crate::struct_ops2::process_struct_1040_7728;
use crate::sys1::{dos3_call_1000_2bb6, dos3_call_1000_370a, dos3_call_1000_39f2, dos3_call_1000_42de, dos3_call_1000_4f94, dos3_call_1000_5174, reg_class_1008_96d2, win_cleanup_func_1040_b0f8};
use crate::sys2::{get_prop_1040_9566, make_proc_inst_1040_8fb8, pass1_1030_838e, process_win_msg_1008_54aa};
use crate::typedefs::{HCURSOR16, HGDIOBJ16};
use crate::ui_funcs::ui1::{free_proc_inst_1040_911e, post_quit_msg_1008_3af4, win_cleanup_1008_0618, win_cleanup_1018_4d22, win_cleanup_1040_d1bc};
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT31, POPCOUNT, SBORROW1, SBORROW2, SUB21, SUB42, ZEXT24};
use crate::winapi_funcs::{GetStockObject16, LoadCursor16, swi};

pub unsafe fn pass1_fn_1000_0c32(param_1: u16, Struct126_: *mut Struct126, param_3: u16) -> u16 {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct145;
    let pb_var3: Vec<u8>;
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

pub unsafe fn pass1_fn_1000_29af(a: u16, b: u16, c: i32) {
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
    if (param_6 | paVar1) == 0 {
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

pub unsafe fn pass1_fn_1000_2b5c(param_1: u16, param_2: u32, param_4: u16) -> u16 {
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
    let pb_var1: Vec<u8>;
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

pub unsafe fn pass1_fn_1000_3024(ctx: &mut AppContext) {
    pass1_fn_1000_3038(1, ctx.g_alloc_addr_1050_1050, (ctx.bp_reg + 1) as u32);
    return;
}

pub unsafe fn pass1_fn_1000_3038(ctx: &mut AppContext, param_1: i32, param_2: u32, param_3: u32) -> u16 {
    let mut i_var1: i32;

    let mut struct_var2 = ctx.PTR_LOOP_1050_6210;
    let mut u_var3 = 0;
    let mut local_4 = 0;
    while struct_var2 <= PTR_LOOP_1050_5ff0 {
        if (param_1 == 1) && ((struct_var2.field_0xa & 0x83) != 0) {
            i_var1 = pass1_fn_1000_2f48(struct_var2, &ctx.g_alloc_addr_1050_1050);
            if i_var1 != -1 {
                u_var3 = u_var3 + 1;
            }
        } else {
            if (param_1 == 0)
                && ((struct_var2.field_0xa & 2) != 0
                    && (
                i_var1 = pass1_fn_1000_2f48(struct_var2, &ctx.g_alloc_addr_1050_1050),
                i_var1 == -1,
                    ))
            {
                local_4 = 0xffff;
            }
        }
        struct_var2 = &struct_var2.field_0xc;
    }
    if param_1 == 1 {
        local_4 = u_var3;
    }
    return local_4;
}

pub unsafe fn pass1_fn_1000_30b4(ctx: &mut AppContext, param_1: u16, param_2: u16, param_1_00: &mut Vec<u8>) -> u16 {
    let mut b_var1: u8;
    let mut u_var2: u16;
    let mut local_c: u16;
    let local_9: u8;
    let local_6: u8;

    bad_1000_25f2(&ctx.g_alloc_addr_1050_1050, ctx.bp_reg + 1);
    unsafe { b_var1 = *param_1_00 };
    _local_6 = ctx.si_reg & 0xff00 | b_var1;
    if b_var1 == 0 {
        return 0;
    }
    if (b_var1 - 0x20) < 0x59 {
        unsafe { b_var1 = *((b_var1 - 0x20) + 0x5ffe) & 0xf };
    } else {
        b_var1 = 0;
    }
    u_var2 = (**((*((b_var1 * 0x8) + 0x5ffe) >> 4) * 2 + 0x30a4))(_local_6);
    return u_var2;
}

pub unsafe fn pass1_fn_1000_34cf(ctx: &mut AppContext) -> u16 {
    //
    let var_1: Address<Struct150> = Address:new();
    let mut pu_var2 = var_1._type.field_0xe;
    let mut u_var1 = pu_var2;
    var_1._type.field_0xe = pu_var2 + 2;
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
    let pb_var1: Vec<u8>;
    let mut u_var2: u32;
    let mut b_var3: u8;
    let mut unaff_bp: i32;
    let mut unaff_si: i32;
    let unaff_DI: Vec<u8>;
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
    let pb_var1: Vec<u8>;
    let paVar2: *mut Struct168;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pa_var5: *mut Struct168;
    let pb_var6: Vec<u8>;
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
    let pb_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut b_var3: u8;
    let mut i_var4: i32;
    let pb_var5: Vec<u8>;
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
    if (b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b) {}
    // goto LAB_1000_3e4d;
    loop {
        pb_var1 = pb_var5;
        pb_var5 = pb_var5 + 1;
        unsafe { b_var3 = *pb_var1 };
        // LAB_1000_3e4d:
        if (0x39 < b_var3) || (b_var3 < 0x30) {
            break;
        }
        i_var4 = i_var4 * 10 + (b_var3 - 0x30);
    }
    if b_var2 == 0x2d {
        i_var4 = -i_var4;
    }
    return i_var4;
}

// pub unsafe fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
//     let pb_var1: Vec<u8>;
//     let mut b_var2: u8;
//     let mut b_var3: u8;
//     let mut i_var4: i32;
//     let pb_var5: Vec<u8>;
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
//     let pb_var1: Vec<u8>;
//     let mut b_var2: u8;
//     let mut b_var3: u8;
//     let mut i_var4: i32;
//     let pb_var5: Vec<u8>;
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

pub unsafe fn pass1_fn_1000_3e82(param_1: i32, param_2: Vec<u8>, uparam_3: i32) -> *mut Struct169 {
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
    let pu_var1: Vec<u8>;
    let pu_var2: Vec<u8>;
    let pu_var3: Vec<u8>;
    let pu_var4: Vec<u8>;
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
    let pb_var1: Vec<u8>;
    let pu8_var2: Vec<u8>;
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
    let pb_var1: Vec<u8>;
    let pu8_var2: Vec<u8>;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let pb_var6: Vec<u8>;
    let pb_var7: Vec<u8>;
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
    let pu_var8: Vec<u8>;
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
    if (param_4 != 0) && (param_3 != 0) {
        i_var7 = param_3 + -1;
        u_stack12 = param_2;
        uStack14 = param_1;
        while i_var7 != 0 {
            u_var9 = uStack14 + param_4;
            u_var12 = u_stack12 + (-CARRY2(uStack14, param_4) & 0x6c);
            uStack22 = 0x4b1c;
            uStack18 = u_var9;
            uStack16 = u_var12;
            unsafe {
                i32_var6 = (*in_fn_ptr_1)();
            }
            if i32_var6 < 0 {
                u_var9 = param_3 - 1;
                i_var7 = 0;
                while {
                    u_var9 = u_var9 >> 1;
                    i_var7 = i_var7 + -1;
                    i_var7 != 0 && u_var9 != 0
                } {}
                if (-i_var7 * 8 >> 0x10) == 0 {
                    u_stack12 = 0x4b4b;
                    u_var9 = pass1_fn_1000_3bac();
                    if (-i_var7 * 8) <= u_var9 {
                        pu_var8 = &stack0xfff6;
                        lVar3 = (param_3 - 1) * param_4;
                        u_var9 = lVar3;
                        uStack14 = u_var9 + param_1;
                        u_stack12 = ((lVar3 >> 0x10) + CARRY2(u_var9, param_1)) * 0x100 + param_2;
                        uStack16 = param_2;
                        uStack18 = param_1;
                        // LAB_1000_4b7d:
                        if pu_var8 <= &uStack18 {
                            return;
                        }
                        // LAB_1000_4b81:
                        if (uStack16 < u_stack12)
                            || (uStack16 <= u_stack12 && (uStack18 < uStack14))
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
                                if (u_var10 != uStack14)
                                    || (
                                        u_var12 = uStack28,
                                        u_var11 = uStack30,
                                        u_var4 = uStack38,
                                        u_var5 = uStack22,
                                        uStack38 != u_stack12,
                                    )
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

pub unsafe fn pass1_fn_1000_5008(ctx: &mut AppContext, param_1: &String, param_2: &Struct199, param_3: u16) {
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
    pass13_funcs::pass1_1008_9cc4(param_1 & 0xffff | uint_1 << 0x10, param_2);
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
    pass13_funcs::pass1_1008_9ce0();
    return;
}

pub unsafe fn pass1_1008_3714(param_1: *mut Vec<u8>) {
    pass14_funcs::pass1_1008_3e0e(param_1);
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
    pass14_funcs::pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(local_AX_8)));
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
    let pb_var1: Vec<u8>;
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
