use crate::{err_funcs::{error_check_1000_0dc6, error_check_1000_17ce}, list_funcs::set_array_val_1008_72a8, mem_funcs::{alloc_mem_1000_0a48, alloc_mem_1000_1708}, pass3_funcs::{pass1_1020_ba3e, pass1_1020_bb8a, pass1_1020_c444, pass1_1020_c4a8}, pass5_funcs::pass1_1030_1cd8, pass_funcs::{pass1_1008_3e76, pass1_1008_3f92, pass1_1008_4b8e, pass1_1008_6eee, pass1_1008_6f7a, pass1_1008_7006, pass1_1008_7056}, string_funcs::{get_string_index_1000_3da4, string_fn_1000_3f9c}, struct_funcs::{
    process_struct_1000_179c, process_struct_1008_48fe, process_struct_1008_4c98,
    struct_fn_1000_160a,
}, sys_funcs::{_hread, _hwrite16, _lclose16, _lcreat16, _llseek16, _lopen16}, util::{CONCAT12, CONCAT13, CONCAT22, CONCAT31, ZEXT24}, mixed_fn_1010_830a};
use crate::app_context::AppContext;
use crate::struct_funcs::{process_struct_1008_41bc, pass1_1038_6520, process_struct_1008_574a, process_struct_1008_dcdc};
use crate::pass3_funcs::{pass1_1020_8712, process_struct_1020_847a, pass1_1020_b97e, pass1_1028_21ba, pass1_1020_bb16};
use crate::pass7_funcs::{pass1_1018_26c2, pass1_1018_268e, pass1_1018_2678};
use crate::other_funcs::{zero_list_1008_3e38, set_param_3_with_switch_1008_72bc, switch_statement_1008_73ea, modify_list_1008_3f62, switch_statement_1008_738c};
use crate::prog_structs::prog_structs_1::{Struct393, Struct104};
use crate::prog_structs::prog_structs_24::{pass1_struct_1, Struct103, Struct8};
use crate::util::{CONCAT11, SBORROW2, SUB41, SUB21, SUB42, CARRY2};
use crate::typedefs::HFILE16;
use crate::mem_funcs::{alloc_mem_1000_07fc, Address};
use crate::prog_structs::prog_structs_29::{Struct732, Struct763, Struct425};
use crate::pass_funcs::{pass1_1008_5b12, pass1_1008_5784, pass1_1008_3eb4, pass1_fn_1008_612e, pass1_1008_c6ae, pass1_fn_1008_60e8, pass1_1008_766e, pass1_1008_b0bc};
use crate::prog_structs::prog_structs_25::{Struct731, Struct9};
use crate::prog_structs::prog_structs_2::{Struct199, Struct390, Struct117, Struct7, Struct131};
use crate::prog_structs::prog_structs_28::{Struct772, Struct936, Struct961, file_object};
use crate::prog_structs::prog_structs_20::{Struct771, Struct506, Struct120};
use crate::pass4_funcs::{pass1_1028_e100, pass1_1028_e0bc, pass1_1028_e4ec, pass1_1028_dc52, ret_1030_154c, pass1_1030_14b4, pass1_1030_145a, pass1_1030_0000, pass1_1028_90e6, pass1_1028_780c, pass1_1028_e1ec, pass1_1028_b204, pass1_1028_e2ac};
use crate::pass6_funcs::{pass1_1038_3ba0, pass1_1038_79b2, pass1_1038_30aa};
use crate::pass5_funcs::{pass1_1030_2068, pass1_1030_835a, pass1_1030_671c, pass1_1030_67cc, pass1_1030_6740, pass1_1030_66de, pass1_1030_6222, pass1_1030_560e, pass1_1030_7e5a, pass1_1030_73ee, pass1_1030_61fe, pass1_1030_2958, pass1_1030_5d0a, pass1_1030_4782, pass1_1030_84ae};
use crate::pass2_funcs::pass1_1010_ebf8;
use crate::pass8_funcs::{pass1_1010_043a, pass1_1010_ed22, pass1_1018_0196, pass1_1010_82f8, pass1_1018_04a4, pass1_1018_04ca};
use crate::string_funcs::{pass1_1030_e4fa, pass1_1030_5164, process_string_1008_7e4a};
use crate::bad_funcs::halt_baddata;
use crate::prog_structs::prog_structs_16::{Struct493, Struct1143};
use crate::prog_structs::prog_structs_9::Struct844;
use crate::prog_structs::prog_structs_10::Struct933;
use crate::prog_structs::prog_structs_26::{Struct866, Struct873, Struct874, Struct1140, Struct1141, Struct1142, Struct121};
use crate::prog_structs::prog_structs_12::{Struct94, Struct235};
use crate::sys_funcs::dos3_call_1000_51aa;
use crate::func_ptr_funcs::call_fn_ptr_1000_5586;
use crate::prog_structs::prog_structs_11::Struct934;
use crate::prog_structs::prog_structs_23::{Struct935, Struct1168, Struct1167};
use crate::prog_structs::prog_structs_31::{Struct962, Struct471, Struct472};
use crate::prog_structs::prog_structs_6::Struct473;
use crate::prog_structs::prog_structs_5::Struct1174;
use crate::prog_structs::prog_structs_30::Struct470;
use crate::prog_structs::prog_structs_17::Struct407;
use crate::prog_structs::prog_structs_27::Struct11;

pub unsafe fn close_file_1008_496c(ctx: &mut AppContext, param_2: &mut Address<Struct7>) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_5: *mut Struct7;
    let mut local_res6: u16;
    let temp_86216c208fd: *mut u32;
    let mut func_ptr: u32;
    let mut temp_5f096a4ace: u32;

    // param_2 = &ctx.PTR_LOOP_1050_4c4c;
    param_2.u16_field_2 = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = param_2.func_ptr_0x4;
    u_var2 = param_2.i_field_4;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            func_ptr = *pu_var1;
            (**func_ptr)();
        }
    }
    error_check_1000_17ce(ctx, param_2.file_path);
    if (&param_2.pv_buffer_0x1a != 0) {
        temp_5f096a4ace = &param_2.pv_buffer_0x1a;
        error_check_1000_0dc6(ctx, temp_5f096a4ace);
    }
    if (param_2.file != 0xffff) {
        _lclose16(param_2.file);
    }
    param_2._type.u16_field_0 = ctx.s_1_1050_389a.clone();
    param_2.u16_field_2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn read_from_file_1008_49e8(ctx: &mut AppContext, in_Struct7: *mut Struct7) {
    let paVar1: *mut Struct131;
    let mut u_var2: u32;
    let mut file_handle: u16;
    let pvVar3: &mut Vec<u8>;
    let mut i_var4: i32;
    let pa_var5: *mut Struct131;
    let mut Struct131_4: u16;
    let mut u_var6: u32;
    
    let struct_a: *mut Struct199;
    
    let local_bx_4: *mut Struct8;
    let mut unaff_ss: u16;
    let mut bytes_read: u32;
    let lVar8: u32;
    let mut in_stack_00000006: i32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut read_count: u32;
    let mut u_var7: u32;
    let mut offset: u16;

    if (in_Struct7.file_path != 0x0) {
        if (in_Struct7.u16_field_0x1e != 0) {
            return;
        }
        if (in_Struct7.file == 0xffff) {
            file_handle = _lopen16(0, in_Struct7.file_path);
            in_Struct7.file = file_handle;
            if (file_handle == 0xffff) {
                return;
            }
        }
        read_count = 0;
        bytes_read = _hread(0xe, CONCAT22(unaff_ss, &local_18), in_Struct7.file);
        if ((bytes_read == 0xe) && ((bytes_read >> 0x10) == 0)) {
            read_count = local_16;
            if (local_18 == &ctx.PTR_LOOP_1050_4d42) {
                lVar8 = _llseek16(0, 0, in_Struct7.file);
                pvVar3 = lVar8;
                local_1c = ctx.__g_Struct94_ptr_1;
                local_1a = (ctx.__g_Struct94_ptr_1 >> 0x10);
                alloc_mem_1000_0a48(1, read_count, (read_count >> 0x10), local_1c, local_1a);
                in_Struct7.pv_buffer_0x1a = pvVar3;
                in_Struct7.u16_field_0x1c = ctx.dx_reg;
                if ((ctx.dx_reg | in_Struct7.pv_buffer_0x1a) == 0) {
                    return;
                }
                // WARNING: Load size is inaccurate
                lVar8 = _hread(read_count, in_Struct7.pv_buffer_0x1a, in_Struct7.file);
                struct_a = (lVar8 >> 0x10);
                local_a = lVar8;
                local_18 = in_Struct7.file;
                local_8 = struct_a;
                _lclose16(local_18);
                in_Struct7.file = 0xffff;
                in_Struct7.u16_field_0x1e = 1;
                // WARNING: Load size is inaccurate
                in_Struct7.pv_field_0xe = in_Struct7.pv_buffer_0x1a;
                u_var6 = &in_Struct7.pv_buffer_0x1a;
                i_var4 = u_var6;
                u_var6 = u_var6 & 0xffff0000;
                in_Struct7.pv_field_0x12 = (u_var6 | i_var4 + 0xe);
                u_var6 = u_var6 | i_var4 + 0x436;
                in_Struct7.u32_field_0x16 = u_var6;
                local_16 = CONCAT22(local_16._2_2_, 0x14);
                local_18 = offset;
                process_struct_1000_179c(0x14, struct_a);
                pa_var5 = u_var6;
                if ((struct_a | pa_var5) == 0) {
                    &in_Struct7.func_ptr_0x4 = 0;
                } else {
                    local_16 = CONCAT22(local_16._2_2_, 0x100);
                    paVar1 = in_Struct7.pv_field_0x12;
                    Struct131_4 = paVar1;
                    Struct131_4 = Struct131_4 + 0x28;
                    u_var2 = paVar1 & 0xffff0000;
                    u_var7 = u_var2 | Struct131_4;
                    local_18 = (u_var2 >> 0x10);
                    process_struct_1008_4c98(
                        (u_var6 & 0xffff | Struct131_4 << 0x10),
                        u_var7,
                        0x100,
                    );
                    pa_var5 = u_var7;
                    in_Struct7.func_ptr_0x4 = pa_var5;
                    in_Struct7.i_field_4 = ctx.dx_reg;
                }
                if (in_Struct7.pv_field_0x22 != 0) {
                    local_16 = local_16 & 0xffff0000 | in_stack_00000006;
                    local_18 = in_Struct7;
                    pass1_1008_4b8e(in_Struct7, pa_var5);
                    return;
                }
                return;
            }
        }
        _lclose16(in_Struct7.file);
        in_Struct7.file = 0xffff;
    }
    return;
}

pub unsafe fn close_file_1008_4c26(param_1: *mut Struct7, param_2: u8) -> *mut Struct7 {
    close_file_1008_496c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn file_fn_1008_6414(ctx: &mut AppContext, param_1: *mut Struct7, param_2: u32) {
    let pp_var1: fn();
    let in_struct_a: *mut Struct103;
    let paVar2: *mut Struct7;
    let struct_a: *mut Struct199;
    let paVar3: *mut Struct199;
    
    
    let local_Struct117: *mut Struct7;
    let mut local_es_4: u16;
    
    let mut local_2a: u16;
    let mut local_28: u16;
    let local_26: u8;

    // local_es_4 = (param_1 >> 0x10);
    local_Struct117 = param_1;
    // param_1 = 0;
    &local_Struct117.field_0x4 = 0;
    paVar2 = &local_26;
    process_struct_1008_48fe(ctx, paVar2,
        1,
        param_2,
    );
    paVar3 = struct_a;
    process_struct_1000_179c(0x1e, struct_a);
    if ((paVar3 | paVar2) == 0) {
        // param_1 = 0;
    } else {
        in_struct_a = CONCAT22(paVar3, paVar2);
        paVar2 = &local_26;
        pass1_1008_3f92(in_struct_a, CONCAT22(ctx.stack_seg_reg, &local_26));
        *param_1 = paVar2;
        local_Struct117.field_0x2 = ctx.dx_reg;
    }
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)(0x1000, param_1);
    local_Struct117.field_0x4 = paVar2;
    local_Struct117.field_0x6 = ctx.dx_reg;
    close_file_1008_496c(ctx, &local_26);
    return;
}

pub unsafe fn close_file_1008_6dd0(param_1: *mut Struct9) {
    let local_bx_4: *mut Struct9;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.file != 0xffff) {
        _lclose16(local_bx_4.file);
        local_bx_4.file = 0xffff;
    }
    error_check_1000_17ce(param_1);
    return;
}
pub unsafe fn write_to_file_1008_6e02(ctx: &mut AppContext, param_1: *mut file_object) -> u16 {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_4: [u8; 2];

    ctx.g_u16_1050_0310 = 0;
    b_var1 = write_to_file_1008_70a6(param_1);
    if (b_var1 != 0) {
        u_var4 = (param_1 >> 0x10);
        u_var3 = param_1;
        set_array_val_1008_72a8(CONCAT22(unaff_ss, local_4), (u_var3 + 4));
        i_var2 = pass1_1008_7006(u_var3, u_var4, CONCAT22(unaff_ss, local_4));
        if ((i_var2 != 0)
            && (
                i_var2 = pass1_1008_6eee(u_var3, u_var4, CONCAT22(unaff_ss, local_4)),
                i_var2 != 0,
            ))
        {
            b_var1 = close_file_1008_726c(param_1);
            if (b_var1 == 0) {
                return 0;
            }
            return 1;
        }
        _lclose16((u_var3 + 4));
    }
    return 0;
}

pub unsafe fn close_file_1008_6e78(ctx: &mut AppContext, param_1: &mut Vec<u8>) -> bool {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_4: [u8; 2];

    ctx.g_u16_1050_0310 = 0;
    b_var1 = read_from_file_1008_7146(param_1);
    if (b_var1 != 0) {
        u_var4 = (param_1 >> 0x10);
        u_var3 = param_1;
        set_array_val_1008_72a8(CONCAT22(unaff_ss, local_4), (u_var3 + 4));
        i_var2 = pass1_1008_7056(u_var3, u_var4, CONCAT22(unaff_ss, local_4));
        if ((i_var2 != 0)
            && (
                i_var2 = pass1_1008_6f7a(u_var3, u_var4, CONCAT22(unaff_ss, local_4)),
                i_var2 != 0,
            ))
        {
            b_var1 = close_file_1008_726c(param_1);
            if (b_var1 == 0) {
                return 0;
            }
            return 1;
        }
        _lclose16((u_var3 + 4));
    }
    return 0;
}
pub fn write_to_file_1008_70a6(ctx: &mut AppContext, param_1: *mut file_object) -> bool {
    let mut local_file: u16;
    let mut count: u16;
    let local_bx_6: *mut file_object;
    let mut u_var1: u16;
    let mut file_handle: u16;
    let mut bytes_written: u32;

    u_var1 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    if (local_bx_6.file != 0xffff) {
        _lclose16(local_bx_6.file);
        local_bx_6.file = 0xffff;
    }
    local_file = _lcreat16(0, param_1.path);
    local_bx_6.file = local_file;
    if (local_file == 0xffff) {
        ctx.g_u16_1050_0310 = 0x6cf;
    } else {
        ctx.PTR_LOOP_1050_0312 = &ctx.PTR_DAT_0005_0000_1050_0004;
        string_fn_1000_3f9c(
            ctx.s__1050_65a0,
            &ctx.g_alloc_addr_1050_1050,
            ctx._PTR_s_SC_03d_1050_0314_1050_031c,
            (ctx._PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
            &ctx.PTR_DAT_0005_0000_1050_0004,
        );
        count = get_string_index_1000_3da4(ctx.s__1050_65a0);
        bytes_written = _hwrite16(count, ctx.s__1050_65a0, local_bx_6.file);
        if (bytes_written == count) {
            return 1;
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

// WARNING: Restarted to delay deadcode elimination for space: stack

pub fn read_from_file_1008_7146(ctx: &mut AppContext, param_1: *mut file_object) -> bool {
    let mut file: u16;
    let b_var1: bool;
    let local_file_obj: *mut file_object;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    local_file_obj = param_1;
    if (local_file_obj.file != 0xffff) {
        _lclose16(local_file_obj.file);
        local_file_obj.file = 0xffff;
    }
    file = _lopen16(0, param_1.path);
    local_file_obj.file = file;
    if (file == 0xffff) {
        ctx.g_u16_1050_0310 = 0x6cf;
    } else {
        b_var1 = read_file_func_1008_71a0((param_1 & 0xffff));
        if (b_var1 != 0) {
            return true;
        }
    }
    return false;
}

// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_func_1008_71a0(ctx: &mut AppContext, in_file_object: *mut file_object) -> bool {
    let mut count: u16;
    let mut unaff_ss: u16;
    let mut bytes_read: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 9];
    let local_5: u8;
    let mut local_4: u16;

    local_4 = 1;
    count = get_string_index_1000_3da4(ctx.s__1050_65a0);
    local_16 = 0;
    bytes_read = _hread(count, CONCAT22(unaff_ss, local_e), (in_file_object + 4));
    bytes_read._0_2_ = bytes_read;
    if (count < bytes_read) {
        bytes_read._0_2_ = count;
    }
    local_18 = bytes_read - 2;
    if (local_18 < 0) {
        local_18 = 0;
    }
    local_1a = 2;
    while (local_18 != 0) {
        local_16 = local_16 * 10 + local_e[local_1a] + -0x30;
        local_1a = local_1a + 1;
        local_18 = local_18 - 1;
    }
    if (local_16 == 1) {
        ctx.PTR_LOOP_1050_0312 = (&ctx.PTR_LOOP_1050_0000 + 1);
    } else {
        if (local_16 == 4) {
            ctx.PTR_LOOP_1050_0312 = &ctx.PTR_DAT_0005_0000_1050_0004;
        } else {
            local_5 = '\0';
            ctx.PTR_LOOP_1050_0312 = (&ctx.PTR_LOOP_1050_0000 + 1);
            local_4 = 0;
        }
    }
    string_fn_1000_3f9c(
        ctx.s__1050_65a0,
        &ctx.g_alloc_addr_1050_1050,
        ctx._PTR_s_SC_03d_1050_0314_1050_031c,
        (ctx._PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
        ctx.PTR_LOOP_1050_0312,
    );
    return local_4;
}

pub fn close_file_1008_726c(ctx: &mut AppContext, param_1: *mut Struct11) -> bool {
    let mut file_handle: u16;
    let local_bx_4: *mut Struct11;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.file != 0xffff) {
        file_handle = _lclose16(local_bx_4.file);
        if (file_handle == 0xffff) {
            ctx.g_u16_1050_0310 = 0x6d1;
            return 0;
        }
        local_bx_4.file = 0xffff;
        ctx.g_u16_1050_0310 = 0;
    }
    return 1;
}

pub unsafe fn read_file_1008_7548(
    ctx: &mut AppContext,
    in_file_handle: &mut Address<HFILE16>,
    param_2: &mut usize,
) {
    let pp_var1: fn();
    let was_file_read: bool;
    let mut u_var3: i32;
    let mut u_var4: u32;
    // let mut local_DX_119: u16;
    // let mut local_CS__1: u16;
    let mut lVar5: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    local_6 = 0;
    was_file_read = read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, &local_6), 4);
    if !was_file_read {
        return;
    }
    if local_6 != 0 {
        let mut u_var4 = local_6;
        if (local_6 < 200) {
            local_6 = 0x0;
            u_var4 = 200;
        }
        let u_var3 = u_var4;
        local_a = u_var4 & 0xffff | local_6 << 0x10;
        let param_2_val = *param_2;
        if param_2_val == 0 {
            ctx.code_seg_reg = 0x1000;
            process_struct_1000_179c(0x1e, local_6);
            if ((local_6._2_2_ | u_var3) == 0) {
                *param_2 = 0;
            } else {
                ctx.code_seg_reg = 0x1020;
                pass1_1020_c444(CONCAT22(local_6, u_var3), 100, local_a);
                *param_2 = u_var3;
                (param_2 + 2) = local_DX_119;
            }
        }

        lVar5 = *param_2;
        pp_var1 = (*param_2 + 0x24);
        (**pp_var1)();

        local_e = 0;
        while (local_e < local_6) {
            was_file_read =
                read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, &local_1c), 4);
            if ((was_file_read == 0)
                || (
                    was_file_read = read_file_1008_7dee(
                        in_file_handle,
                        CONCAT22(ctx.stack_seg_reg, &local_18),
                        2,
                    ),
                    was_file_read == 0,
                ))
            {
                unsafe {
                    pp_var1 = (*param_2 + 0x1c);
                    (**pp_var1)(local_CS__1, *param_2);
                }
                return;
            }

            unsafe {
                pp_var1 = (*param_2 + 0x28);
                (**pp_var1)(
                    local_CS__1,
                    *param_2,
                    (*param_2 >> 0x10),
                    local_18,
                    local_1c,
                );
            }
            local_e = local_e + 1;
        }

        unsafe {
            pp_var1 = (*param_2 + 0x1c);
            (**pp_var1)(local_CS__1, *param_2, lVar5);
        }
    }
    return;
}

pub unsafe fn read_file_1008_76e4(
    ctx: &mut AppContext,
    param_1: &mut Address<HFILE16>,
    param_2: &mut usize,
) {
    let pp_var1: fn();
    let mut b_var3: bool = false;
    // let in_dx: *mut Struct199;
    let mut local_1c: u16;
    let mut local_1a: u16;
    // let mut local_18: u32;
    let mut local_a: u32;
    let mut val: u16;

    // let mut local_6: u32 = 0;
    let mut local_6: Vec<u8> = Vec::new();
    // CONCAT22(ctx.stack_seg_reg, &local_6)
    let mut u_var2 = read_file_1008_7dee(
        param_1,
        &mut local_6,
        4);
    if u_var2 == 0 {
        return;
    }
    if local_6 != 0 {
        let param_2_val = unsafe { *param_2 };
        if param_2_val == 0 {
            process_struct_1000_179c(0x18, in_dx);
            if (ctx.dx_reg | u_var2) == 0 {
                unsafe {
                    *param_2 = 0;
                }
            } else {
                pass1_1030_1cd8(CONCAT22(in_dx, u_var2), 5, local_6);
                *param_2 = u_var2;
                (param_2 + 2) = local_DX_93;
            }
        }
        let param_2_val = unsafe { *param_2 };
        pp_var1 = (param_2_val + 0x14);
        (**pp_var1)();
        local_a = 0;
        while (local_a < local_6) {
            b_var3 = read_file_1008_7dee(param_1, CONCAT22(ctx.stack_seg_reg, &local_18), 4);
            if (b_var3 == 0) {
                return;
            }
            let param_2_val = unsafe { *param_2 };
            pp_var1 = (param_2_val + 0x18);
            (**pp_var1)();
            local_a = local_a + 1;
        }
        let param_2_val = unsafe { *param_2 };
        pp_var1 = (param_2_val + 0x1c);
        (**pp_var1)();
    }
    return;
}

pub fn read_file_1008_77cc(
    ctx: &mut AppContext,
    in_hfile: &mut HFILE16,
    param_2: &mut usize,
) -> u16 {
    let mut u_var1: i32;
    let BVar2: bool;
    let in_dx: *mut Struct199;
    // 
    let mut local_DXAX_90: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    u_var1 = read_file_1008_7dee(in_hfile, CONCAT22(ctx.stack_seg_reg, &local_4), 2);
    if u_var1 == 0 {
        return 0;
    }
    if local_4 != 0 {
        let param_2_val = unsafe { *param_2 };
        if param_2_val == 0 {
            process_struct_1000_179c(10, ctx.dx_reg);
            if (ctx.dx_reg | u_var1) == 0 {
                unsafe { *param_2 = 0 };
            } else {
                ctx.dx_ax_reg = pass1_1020_ba3e(CONCAT22(ctx.dx_reg, u_var1), 5, 5);
                *param_2 = ctx.dx_ax_reg;
                (param_2 + 2) = (ctx.dx_ax_reg >> 0x10);
            }
        }
        local_6 = 0;
        while local_6 < local_4 {
            BVar2 = read_file_1008_7dee(in_hfile, CONCAT22(ctx.stack_seg_reg, &local_14), 2);
            if BVar2 == 0 {
                return 0;
            }
            BVar2 = read_file_1008_7dee(in_hfile, CONCAT22(ctx.stack_seg_reg, &local_10), 4);
            if (BVar2 == 0) {
                return 0;
            }
            let param_2_val = unsafe { *param_2 };
            pass1_1020_bb8a(
                param_2_val,
                (param_2_val >> 0x10),
                local_10,
                (local_10 >> 0x10),
                local_14,
            );
            local_6 = local_6 + 1;
        }
    }
    return 1;
}

// WARNING: Could not reconcile some variable overlaps

pub fn write_to_file_1008_7898(ctx: &mut AppContext, in_file: &mut HFILE16, param_2: *mut u32) {
    let pp_var1: fn();
    // 
    let BVar2: bool;
    // 
    let mut u_var3: u16;
    // 
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x0) {
        ctx.ax_reg = 0;
        u_var3 = 0;
    } else {
        let param_2_val = unsafe { *param_2 };
        pp_var1 = (param_2_val + 0x10);
        (**pp_var1)();
        u_var3 = ctx.dx_reg;
    }
    local_6 = CONCAT22(u_var3, ctx.ax_reg);
    local_18 = CONCAT22(u_var3, ctx.ax_reg);
    BVar2 = write_to_file_1008_7e1c(in_file, CONCAT22(ctx.stack_seg_reg, &local_18), 4);
    if (BVar2 != 0) {
        local_a = 0;
        while (true) {
            if (local_6 <= local_a) {
                return;
            }
            pass1_1020_c4a8(
                param_2,
                CONCAT22(ctx.stack_seg_reg, &local_14),
                CONCAT22(ctx.stack_seg_reg, &local_18),
                local_a,
            );
            local_24 = local_18;
            BVar2 = write_to_file_1008_7e1c(in_file, CONCAT22(ctx.stack_seg_reg, &local_24), 4);
            if (BVar2 == 0) {
                break;
            }
            local_26 = local_14;
            BVar2 = write_to_file_1008_7e1c(in_file, CONCAT22(ctx.stack_seg_reg, &local_26), 2);
            if (BVar2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return;
            }
            local_a = local_a + 1;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn write_to_file_1008_7954(ctx: &mut AppContext, param_1: &mut HFILE16, param_2: *mut u32) {
    let pp_var1: fn();
    // 
    let BVar2: bool;
    let mut u_var3: u32;
    //
    // 
    //
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x0) {
        ctx.ax_reg = 0;
        local_16 = 0;
    } else {
        let param_2_val = unsafe { *param_2 };
        pp_var1 = (param_2_val + 0x10);
        (**pp_var1)();
        local_16 = ctx.dx_reg;
    }
    local_6 = CONCAT22(local_16, ctx.ax_reg);
    local_18 = ctx.ax_reg;
    BVar2 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_18), 4);
    if (BVar2 != 0) {
        local_a = 0;
        while (true) {
            if (local_6 <= local_a) {
                return;
            }
            let param_2_val = unsafe { *param_2 };
            pp_var1 = (param_2_val + 4);
            u_var3 = local_6;
            (**pp_var1)();
            local_20 = u_var3;
            local_1e = ctx.dx_reg;
            local_18 = local_20;
            local_16 = ctx.dx_reg;
            BVar2 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_20), 4);
            if (BVar2 == 0) {
                break;
            }
            local_a = local_a + 1;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn write_to_file_1008_79f0(ctx: &mut AppContext, in_file: u32, param_2: libc::c_long) {
    // 
    let mut local_es_11: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        ctx.ax_reg = 0;
        local_4 = 0;
    } else {
        local_es_11 = (param_2 >> 0x10);
        ctx.ax_reg = (param_2 + 4);
        local_4 = (param_2 + 6);
    }
    write_to_file_1008_7954(ctx, in_file, CONCAT22(local_4, ctx.ax_reg));
    return;
}

pub fn write_to_file_1008_7a22(param_1: &mut HFILE16, param_2: *mut *mut u8) {
    let b_var1: bool;
    let mut local_es_9: u16;
    
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x0) {
        local_4 = 0;
    } else {
        local_4 = (param_2 + 4);
    }
    local_12 = local_4;
    b_var1 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_12), 2);
    if (b_var1 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
    } else {
        local_6 = 0;
        while (true) {
            if (local_4 <= local_6) {
                return;
            }
            pass1_1020_bb16(
                param_2,
                CONCAT22(ctx.stack_seg_reg, &local_10),
                CONCAT22(ctx.stack_seg_reg, &local_12),
                local_6,
            );
            local_a = local_12;
            local_1c = local_12;
            b_var1 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_1c), 2);
            if (b_var1 == 0) {
                break;
            }
            local_24 = local_10;
            b_var1 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_24), 4);
            if (b_var1 == 0) {
                return;
            }
            local_6 = local_6 + 1;
        }
    }
    return;
}

pub fn read_file_1008_7ad4(in_file: &mut HFILE16, param_2: u32) -> u16 {
    let b_var1: bool;
    
    let mut local_14: u16;
    let mut local_10: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    b_var1 = read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_4), 2);
    if (b_var1 != 0) {
        local_6 = 0;
        while (true) {
            if (local_4 <= local_6) {
                return 1;
            }
            b_var1 = read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_14), 2);
            if ((b_var1 == 0)
                || (
                    b_var1 = read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_10), 4),
                    b_var1 == 0,
                ))
            {
                break;
            }
            pass1_1020_bb8a(param_2, local_10, (local_10 >> 0x10), local_14);
            local_6 = local_6 + 1;
        }
    }
    return 0;
}

pub unsafe fn write_file_1008_7b4c(param_1: &mut HFILE16, param_2: *mut u8) -> u16 {
    let b_var1: bool;
    
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(ctx.stack_seg_reg, &local_8),
        CONCAT22(ctx.stack_seg_reg, &local_6),
        CONCAT22(ctx.stack_seg_reg, &local_4),
    );
    local_12 = local_4;
    b_var1 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_12), 2);
    if (b_var1 != 0) {
        local_c = local_6;
        b_var1 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_c), 2);
        if (b_var1 != 0) {
            local_c = local_8;
            b_var1 = write_to_file_1008_7e1c(param_1, CONCAT22(ctx.stack_seg_reg, &local_c), 2);
            if (b_var1 != 0) {
                return 1;
            }
        }
    }
    return 0;
}

pub unsafe fn read_file_1008_7bc8(ctx: &mut AppContext, in_file: &mut HFILE16, param_2: *mut u16) -> u16 {
    let b_var1: bool;
    
    let mut local_8: u16;
    let mut local_6: u32;

    b_var1 = read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_6 + 2), 2);
    if (b_var1 != 0) {
        b_var1 = read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_6), 2);
        if (b_var1 != 0) {
            b_var1 = read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_8), 2);
            if (b_var1 != 0) {
                pass1_1008_3e76(param_2, local_8, local_6, (local_6 >> 0x10));
                return 1;
            }
        }
    }
    return 0;
}

pub fn write_to_file_1008_7c2a(ctx: &mut AppContext,
                               param_1: &mut Address<HFILE16>,
                               param_2: &mut String) -> bool {
    if param_2 != 0x0 {
        let u_var1 = get_string_index_1000_3da4(param_2);
        let write_file_result = write_to_file_1008_7e1c(param_1, param_2, (u_var1 + 1) as usize);
        return write_file_result;
    }
    write_to_file_1008_7e1c(param_1, &ctx.s_playerName_1050_148e[0xc..], 1);
    return true;
}

pub unsafe fn read_file_into_str_1008_7c6e(p_file_handle: &mut Address<HFILE16>, out_str: &mut String) {
    let mut pc_var1: String = String::new();
    // let mut unaff_ss: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let local_c: u8 = 0;

    loop {
        pc_var1 = out_str.clone();
        // read 1 byte i32o a stack buffer
        let mut p_file_handle_val = p_file_handle._type;
        let mut buffer: Vec<u8> = Vec::new();
        // CONCAT22(unaff_ss, &local_c)
        let bytes_read = _hread(&mut p_file_handle_val, &mut buffer, 1);
        // read until the terminator is reached; this might overflow if the string is
        // longer than 9 bytes
        if local_c == 0 {
            break;
        }
        // advance the p i32er param2 by 1
        // TODO
        // out_str = (out_str & 0xffff0000 | (out_str + 1));
        pc_var1[0] = local_c;
    }
    out_str[0] = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_to_file_1008_7cac(param_1: &mut Address<HFILE16>, param_2: u16) -> bool {
    let mut b_var1: bool;
    let mut char_buf: String;

    string_fn_1000_3f9c(
        &mut char_buf,
        &ctx.stack_seg_reg,
        &ctx.s__s_02x_1050_0340,
        &ctx.g_alloc_addr_1050_1050,
        &ctx._PTR_s_dcbSC_1050_0336_1050_033c,
    );
    let mut buf_size = get_string_index_1000_3da4(&mut char_buf);
    b_var1 = write_to_file_1008_7e1c(&param_1, &mut char_buf, &buf_size);
    if b_var1 == false {
        ctx.g_u16_1050_0310 = 0x6d0;
        return false;
    }
    return true;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_7cfe(param_1: *mut HFILE16) -> u8 {
    let mut bVar1: bool;
    let BVar2: bool;
    let mut unaff_ss: u16;
    let mut bytes_read: u32;
    let mut local_410: u32;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    // let mut local_406: [u8;1024];
    let mut local_406: [u8; 1024];
    let mut local_6: u32;

    local_6 = 0;
    bVar1 = false;
    loop {
        let param_1_val = unsafe { *param_1 };
        _llseek16(0, local_6, param_1_val);
        bytes_read = _hread(0x400, CONCAT22(unaff_ss, local_406), param_1_val);
        local_410 = 0;
        while (local_410 < bytes_read) {
            if (local_406[local_410] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
                if (!bVar1) {
                    bVar1 = true;
                    local_6 = CONCAT22(
                        (local_6 >> 0x10) + local_410._2_2_ + CARRY2(local_6, local_410),
                        local_6 + local_410,
                    );
                    break;
                }
                bVar1 = false;
                BVar2 = process_string_1008_7e4a();
                if (BVar2 != 0) {
                    _llseek16(0, local_410 + local_6 + 7, param_1_val);
                    return 0x1;
                }
            }
            local_410 = local_410 + 1;
        }
        if (!bVar1) {
            if (bytes_read < 0x400) {
                return '\0';
            }
            local_6._0_2_ = CONCAT11(local_6._1_1_ + 4, local_6);
            local_6 = CONCAT22((local_6 >> 0x10) + (0xfb < local_6._1_1_), local_6);
        }
    }
}

pub fn read_file_1008_7dee(
    file_handle: &mut Address<HFILE16>,
    buffer: &mut Vec<u8>,
    count: usize,
) -> bool {
    let bytes_read = _hread(&mut file_handle._type, buffer, count);
    if bytes_read == count {
        return true;
    }
    return false;
}

pub fn write_to_file_1008_7e1c(
    file: &mut Address<HFILE16>,
    buffer: &String,
    count: usize,
) -> bool {
    let bytes_written = _hwrite16(file._type, buffer, count);
    if bytes_written == count {
        return true;
    }
    return false;
}

pub unsafe fn read_file_1008_bb5e(in_Struct120: &mut Struct120, file_handle_ptr: &mut HFILE16) {
    let pp_var1: fn();
    let mut b_read_result_1: u16;
    let b_read_result_2: bool;
    let paVar3: *mut Struct199;
    let local_AX_168: *mut Struct121;
    let local_AX_189: *mut libc::c_char;
    let u_var2: u8;
    let b_var4: bool;
    let pcVar5: *mut libc::c_char;
    let mut u_var6: i32;
    let extraout_var: u32;
    let local_Struct120: *mut Struct120;
    let extraout_var_00: u32;
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var7: u16;
    
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut in_stack_0000000a: u16;
    let offset2: u8;
    let base: u8;
    let mut local_13c: u32;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    // let mut local_118: [u8;256];
    let local_118: [u8; 256];
    // let mut two_file_bytes_2: [u8;2];
    let two_file_bytes_2: [u8; 2];
    // let mut two_file_bytes: [u8;2];
    let two_file_bytes: [u8; 2];
    // let mut buffer_1: [u8;4];
    let buffer_1: [u8; 4];
    // let mut four_file_bytes: [u8;4];
    let four_file_bytes: [u8; 4];

    let mut func_ptr_1: u32;

    if (PTR_LOOP_1050_0312 < 2) {
        // bail if the read fails
        return;
    }
    u_var2 = read_file_1008_7cfe(CONCAT22(in_stack_0000000a, file_handle_ptr));
    if (CONCAT31(extraout_var, u_var2) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        local_Struct120._0_2_ = in_Struct120;
        local_Struct120._0_2_ = local_Struct120 + 0x22;
        // read two bytes from the file i32o the structure passed in by param_1
        b_read_result_1 = read_file_1008_7dee(
            _file_handle_ptr,
            (in_Struct120 & 0xffff0000 | local_Struct120),
            2,
        );
    }
    if ((b_read_result_1 != 0)
        && (
            b_read_result_2 =
                read_file_1008_7dee(_file_handle_ptr, CONCAT22(unaff_ss, buffer_1), 2),
            b_read_result_2 != 0,
        ))
    {
        // read two bytes from the file i32o buffer_1)) {
        // bail if the file read fails
        if (buffer_1._0_2_ == 0) {
            return;
        }
        process_struct_1000_179c(0xc, in_dx);
        struct_a = (in_dx | b_read_result_2);
        if (struct_a == 0x0) {
            paVar3 = 0x0;
            struct_a = 0x0;
        } else {
            paVar3 = process_struct_1008_574a(CONCAT22(in_dx, b_read_result_2));
        }
        u_var9 = (in_Struct120 >> 0x10);
        (local_Struct120 + 10) = paVar3;
        (local_Struct120 + 0xc) = struct_a;
        local_11e = 0;
        while (true) {
            if (buffer_1._0_2_ <= local_11e) {
                return;
            }
            u_var6 = buffer_1._0_2_;
            process_struct_1000_179c(0x12, struct_a);
            u_var8 = struct_a | u_var6;
            if (u_var8 == 0) {
                local_AX_168 = 0x0;
                u_var8 = 0;
            } else {
                u_var2 = pass1_1008_b0bc(CONCAT22(struct_a, u_var6));
                local_AX_168 = CONCAT31(extraout_var_00, u_var2);
            }
            _local_11c = CONCAT22(u_var8, local_AX_168);
            local_AX_189 = local_118;
            u_var7 = u_var8;
            read_file_into_str_1008_7c6e(_file_handle_ptr, CONCAT22(unaff_ss, local_AX_189));
            if (((local_AX_189 == 0x0)
                || (
                    b_var4 = read_file_1008_7dee(
                        _file_handle_ptr,
                        CONCAT22(unaff_ss, two_file_bytes),
                        2,
                    ),
                    b_var4 == 0,
                ))
                || (
                    b_var4 = read_file_1008_7dee(
                        _file_handle_ptr,
                        CONCAT22(unaff_ss, four_file_bytes),
                        4,
                    ),
                    b_var4
                        == 0
                    // read four bytes from file i32o stack buffer)) ||
           (b_var4 = read_file_1008_7dee(_file_handle_ptr,CONCAT22(unaff_ss,two_file_bytes_2) ,2), b_var4 == 0),
                ))
            {
                // read two bytes from file i32o local buffer))
                break;
            }
            pcVar5 = local_118;
            pass1_fn_1008_60e8(pcVar5);
            local_AX_168.field_0x4 = pcVar5;
            local_AX_168.field_0x6 = u_var7;
            local_AX_168.two_file_bytes_0x8 = two_file_bytes;
            local_AX_168.four_file_bytes_0xa = four_file_bytes;
            local_AX_168.two_file_bytes_0xe = two_file_bytes_2;
            func_ptr_1 = ((local_Struct120 + 10) + 8);
            (**func_ptr_1)();
            local_11e = local_11e + 1;
            struct_a = ctx.dx_reg;
        }
        if (_local_11c != 0x0) {
            pp_var1 = *_local_11c;
            (**pp_var1)(0, local_AX_168, u_var8, 1, _local_11c);
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;

    return;
}

pub fn write_to_file_1008_c98e(param_1: u32, param_2: u32) {
    let u_var1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut unaff_ss: u16;
    let mut local_10: u32;

    u_var1 = write_to_file_1008_7cac(param_2, 0x15);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        local_10 = (param_1 + 0xe);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}

pub fn read_file_1008_c9d4(param_1: u32, in_file: *mut HFILE16) {
    let u_var1: u8;
    let BVar2: bool;
    let extraout_var: u32;

    if (1 < PTR_LOOP_1050_0312) {
        u_var1 = read_file_1008_7cfe(in_file);
        if (CONCAT31(extraout_var, u_var1) == 0) {
            ctx.g_u16_1050_0310 = 0x6d4;
            return;
        }
        BVar2 = read_file_1008_7dee(in_file, (param_1 & 0xffff0000 | (param_1 + 0xe)), 4);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub unsafe fn write_to_file_1008_e5da(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let BVar3: bool;
    let pu_var4: *mut u8;
    let extraout_var: u32;
    
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_30: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    //let mut local_c: [u8;8];
    let local_c: [u8; 8];
    let mut local_4: u16;

    u_var2 = write_to_file_1008_7cac(param_2, 0x14);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        u_var6 = (param_1 >> 0x10);
        i_var5 = param_1;
        if ((i_var5 + 10) == 0) {
            local_4 = 0;
        } else {
            u_var1 = (i_var5 + 10);
            local_4 = (u_var1 + 8);
        }
        local_1c = local_4;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1c), 2);
        if (BVar3 != 0) {
            pass1_1008_5784(CONCAT22(unaff_ss, local_c), (i_var5 + 10));
            while {
                pu_var4 = local_c;
                pass1_1008_5b12(CONCAT22(unaff_ss, pu_var4));
                _local_10 = CONCAT22(ctx.dx_reg, pu_var4);
                if ((ctx.dx_reg | pu_var4) == 0) {
                    return;
                }
                local_24 = (pu_var4 + 4);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_24), 4);
                if (BVar3 == 0) {
                    break;
                }
                local_28 = (_local_10 + 8);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_28), 4);
                if (BVar3 == 0) {
                    break;
                }
                local_16 = (_local_10 + 0xc);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_16), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_30 = (_local_10 + 0xe);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_30), 4);
                if (BVar3 == 0) {
                    break;
                }
                local_16 = (_local_10 + 0x12);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_16), 2);
                BVar3 != 0
            } {}
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_38
// WARNING: Could not reconcile some variable overlaps

pub fn read_file_1008_e70e(param_1: u32, in_file_1: *mut HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let BVar3: bool;
    let mut u_var4: u16;
    let extraout_var: u32;
    let in_dx: *mut Struct199;
    let mut u_var5: u16;
    
    
    let mut unaff_ss: u16;
    let mut local_38: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    if (PTR_LOOP_1050_0312 < 2) {
        return;
    }
    u_var2 = read_file_1008_7cfe(in_file_1);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        BVar3 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_4), 2);
        if (BVar3 != 0) {
            if (local_4 == 0) {
                return;
            }
            local_a = 0;
            while (true) {
                if (local_4 <= local_a) {
                    return;
                }
                u_var4 = local_4;
                process_struct_1000_179c(0x14, in_dx);
                if ((in_dx | u_var4) == 0) {
                    u_var4 = 0;
                    u_var5 = 0;
                } else {
                    process_struct_1008_dcdc(CONCAT22(in_dx, u_var4));
                    u_var5 = ctx.dx_reg;
                }
                _local_e = CONCAT22(u_var5, u_var4);
                BVar3 = read_file_1008_7dee(in_file_1, CONCAT22(u_var5, u_var4 + 4), 4);
                if ((((BVar3 == 0)
                    || (
                        BVar3 = read_file_1008_7dee(
                            in_file_1,
                            (_local_e & 0xffff0000 | (_local_e + 8)),
                            4,
                        ),
                        BVar3 == 0,
                    ))
                    || (
                        BVar3 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_12), 2),
                        BVar3 == 0,
                    ))
                    || ((
                        BVar3 = read_file_1008_7dee(
                            in_file_1,
                            (_local_e & 0xffff0000 | (_local_e + 0xe)),
                            4,
                        ),
                        BVar3 == 0
                            || (
                                BVar3 = read_file_1008_7dee(
                                    in_file_1,
                                    (_local_e & 0xffff0000 | (_local_e + 0x12)),
                                    2,
                                ),
                                BVar3 == 0,
                            ),
                    )))
                {
                    break;
                }
                (_local_e + 0xc) = local_12;
                pp_var1 = ((param_1 + 10) + 4);
                (**pp_var1)();
                local_a = local_a + 1;
                in_dx = ctx.dx_reg;
            }
            if (_local_e != 0x0) {
                pp_var1 = *_local_e;
                (**pp_var1)(0x1000, _local_e, (_local_e >> 0x10), 1, _local_e);
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_file_1010_0ad2(ctx: &mut AppContext, in_struct_1: *mut Struct235, param_2: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let BVar3: bool;
    let pcVar4: *mut libc::c_char;
    let extraout_var: u32;
    let mut local_2a: u32;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = write_to_file_1008_7cac(param_2, 6);
    if (CONCAT31(extraout_var, u_var2) == 0) {
        return;
    }
    ctx.es_reg = (in_struct_1 >> 0x10);
    ctx.bx_reg = in_struct_1;
    if (ctx.bx_reg.field_0xa == 0) {
        local_6 = 0;
    } else {
        u_var1 = ctx.bx_reg.field_0xa;
        local_6 = (u_var1 + 8);
    }
    local_1e = local_6;
    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_1e), 2);
    if (BVar3 != 0) {
        pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, local_e), ctx.bx_reg.field_0xa);
        while {
            pcVar4 = local_e;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, pcVar4));
            _local_12 = CONCAT22(ctx.dx_reg, pcVar4);
            if ((ctx.dx_reg | pcVar4) == 0) {
                local_22 = ctx.bx_reg.field_0xe;
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_22), 2);
                if (BVar3 == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                local_22 = ctx.bx_reg.field_0x10;
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_22), 2);
                if (BVar3 == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                if (ctx.bx_reg.field_0x18 != 0) {
                    u16_1050_0e28 = ctx.bx_reg.field_0x12;
                    PTR_LOOP_1050_0e30 = ctx.bx_reg.field_0x14;
                    PTR_LOOP_1050_0ea8 = ctx.bx_reg.field_0x16;
                }
                local_4 = 0;
                while (true) {
                    if (9 < local_4) {
                        local_4 = 0;
                        while (true) {
                            if (2 < local_4) {
                                if (ctx.bx_reg.field_0x18 != 0) {
                                    u16_1050_0e28 = 0;
                                    PTR_LOOP_1050_0e30 = 0x0;
                                    PTR_LOOP_1050_0ea8 = 0x0;
                                }
                                return;
                            }
                            local_1e = (local_4 * 8 + 0xea8);
                            BVar3 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(ctx.stack_seg_reg, &local_1e),
                                2,
                            );
                            if (BVar3 == 0) {
                                break;
                            }
                            local_4 = local_4 + 1;
                        }
                        ctx.g_u16_1050_0310 = 0x6d0;
                        return;
                    }
                    local_1e = (local_4 * 8 + 0xe28);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_1e), 2);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_4 = local_4 + 1;
                }
                ctx.g_u16_1050_0310 = 0x6d0;
                return;
            }
            local_18 = (pcVar4 + 4);
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_18), 2);
            if (BVar3 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return;
            }
            local_2a = (_local_12 + 6);
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_2a), 4);
            BVar3 != 0
        } {}
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn read_file_1010_0c7c(param_1: u32, param_2: *mut HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let BVar3: bool;
    let mut u_var4: u16;
    let b_var5: bool;
    let extraout_var: u32;
    let in_dx: *mut Struct199;
    
    let mut i32_var6: i32;
    let mut unaff_ss: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_26: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, u_var2) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_6), 2);
        if (BVar3 != 0) {
            local_8 = 0;
            while (i32_var6 = param_1, local_8 < local_6) {
                u_var4 = local_6;
                process_struct_1000_179c(10, in_dx);
                local_1a = CONCAT22(in_dx, u_var4);
                if ((in_dx | u_var4) == 0) {
                    local_16 = 0;
                } else {
                    local_1a = ctx.s_1_1050_389a;
                    (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
                    local_1a = 0xea8;
                    (u_var4 + 2) = 0x1010;
                    local_16 = local_1a;
                }
                BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_12), 2);
                if ((BVar3 == 0)
                    || (
                        BVar3 = read_file_1008_7dee(
                            param_2,
                            (local_16 & 0xffff0000 | (local_16 + 6)),
                            4,
                        ),
                        BVar3 == 0,
                    ))
                {
                    local_1a = local_16;
                    if (local_16 != 0) {
                        pp_var1 = local_16;
                        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, local_16, (local_16 >> 0x10), 1);
                    }
                    // goto LAB_1010_0cb1;
                }
                (local_16 + 4) = local_12;
                pp_var1 = ((i32_var6 + 10) + 8);
                (**pp_var1)();
                local_8 = local_8 + 1;
                in_dx = ctx.dx_reg;
            }
            BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i32_var6 + 0xe)), 2);
            if ((BVar3 != 0)
                && (
                    BVar3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i32_var6 + 0x10)), 2),
                    BVar3 != 0,
                ))
            {
                local_4 = 0;
                while (local_4 < 10) {
                    BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_2a), 2);
                    if (BVar3 == 0) {
                        // goto LAB_1010_0cb1;
                    }
                    b_var5 = local_4;
                    if (PTR_LOOP_1050_0312 < 2) {
                        switch_statement_1008_738c(param_2, (param_2 >> 0x10), local_4);
                        b_var5 = BVar3;
                    }
                    (b_var5 * 8 + 0xe28) = local_2a;
                    local_4 = local_4 + 1;
                    local_26 = b_var5;
                }
                if (2 < PTR_LOOP_1050_0312) {
                    local_4 = 0;
                    while {
                        BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_2a), 2);
                        if (BVar3 == 0) {}
                        // goto LAB_1010_0cb1;
                        (local_4 * 8 + 0xea8) = local_2a;
                        local_4 = local_4 + 1;
                        local_4 < 3
                    } {}
                }
                return;
            }
        }
        // LAB_1010_0cb1:
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1010_3fc2(in_struct_1: *mut Struct390, in_file: *mut HFILE16) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let bool_rc: bool;
    let struct_a_lo: *mut Struct390;
    let struct_a_hi: *mut Struct390;
    let string_base: *mut libc::c_char;
    let string_offset_a: *mut libc::c_char;
    let string_offset_b: *mut libc::c_char;

    u_var1 = write_to_file_1008_7cac(in_file, 5);
    if (CONCAT11(extraout_AH, u_var1) != 0) {
        struct_a_hi = (in_struct_1 >> 0x10);
        struct_a_lo = in_struct_1;
        string_offset_a = struct_a_lo.string_buf_ptr_1;
        bool_rc = write_to_file_1008_7e1c(in_file, CONCAT22(string_base, &string_offset_a), 2);
        if (bool_rc != 0) {
            string_offset_b = struct_a_lo.string_buf_ptr_2;
            bool_rc = write_to_file_1008_7e1c(in_file, CONCAT22(string_base, &string_offset_b), 2);
            if (bool_rc != 0) {
                string_offset_b = struct_a_lo.string_buf_ptr_3;
                bool_rc =
                    write_to_file_1008_7e1c(in_file, CONCAT22(string_base, &string_offset_b), 2);
                if (bool_rc != 0) {
                    return 1;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn read_file_1010_404a(param_1: *mut Struct407, param_2: u32) {
    let u_var1: u8;
    let local_AX_39: *mut Struct407;
    let BVar2: bool;
    let extraout_var: u32;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    u_var1 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, u_var1) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        local_AX_39 = param_1;
        local_AX_39 = &local_AX_39.field_0x24;
        BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | ZEXT24(local_AX_39)), 2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | ZEXT24(local_AX_39 + 1)),
                    2,
                );
                if (BVar2 != 0) {
                    local_AX_39.field_0x6a = local_4;
                    return;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn write_to_file_1010_5dc6(
    in_struct_1: &mut Address<Struct425>,
    in_file_1: &mut Address<HFILE16>) -> bool {
    let mut u_var1: u32;
    let u_var2: u8;
    // let extraout_AH: u8;
    let mut i_var3: i32;
    let b_var4: bool;
    // let ctx.bx_reg: *mut Struct425;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    let u_var2 = write_to_file_1008_7cac(in_file_1, 4);
    if u_var2 != false {
        ctx.bx_reg = in_struct_1;
        let u_var1 = ctx.bx_reg.field_0x68;
        i_var3 = write_to_file_1008_7c2a(in_file_1, u_var1, (u_var1 >> 0x10));
        if (i_var3 != 0) {
            u_var1 = ctx.bx_reg.field_0x6c;
            i_var3 = write_to_file_1008_7c2a(in_file_1, u_var1, (u_var1 >> 0x10));
            if (i_var3 != 0) {
                local_c = u16_1050_13ae;
                b_var4 = write_to_file_1008_7e1c(in_file_1, CONCAT22(unaff_ss, &local_c), 2);
                if (b_var4 != 0) {
                    local_6 = ctx.bx_reg.field_0x82;
                    b_var4 = write_to_file_1008_7e1c(in_file_1, CONCAT22(unaff_ss, &local_6), 2);
                    if (b_var4 != 0) {
                        return true;
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return false;
}

pub unsafe fn read_file_1010_5e56(param_1: u32, param_2: *mut HFILE16) {
    let u_var1: u8;
    let pu_var2: *mut u8;
    let BVar3: bool;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    u_var1 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, u_var1) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        pu_var2 = local_402;
        read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_ss, pu_var2));
        if (pu_var2 != 0x0) {
            pu_var2 = local_402;
            pass1_fn_1008_60e8(pu_var2);
            u_var5 = (param_1 >> 0x10);
            i_var4 = param_1;
            (i_var4 + 0x68) = pu_var2;
            (i_var4 + 0x6a) = in_dx;
            pu_var2 = local_402;
            read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_ss, pu_var2));
            if (pu_var2 != 0x0) {
                pu_var2 = local_402;
                pass1_fn_1008_60e8(pu_var2);
                (i_var4 + 0x6c) = pu_var2;
                (i_var4 + 0x6e) = in_dx;
                BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_404), 2);
                if (BVar3 != 0) {
                    u16_1050_13ae = local_404;
                    if (PTR_LOOP_1050_0312 < 2) {
                        return;
                    }
                    BVar3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x82)), 2);
                    if (BVar3 != 0) {
                        return;
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_file_1010_6372(param_1: u32, param_2: *mut HFILE16) {
    let u_var1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_8: u32;

    u_var1 = write_to_file_1008_7cac(param_2, 7);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        local_10 = (i_var3 + 10);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
        if (BVar2 != 0) {
            local_8 = (i_var3 + 0xe);
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 4);
            if (BVar2 != 0) {
                local_8 = (i_var3 + 0x12);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 4);
                if (BVar2 != 0) {
                    local_8 = (i_var3 + 0x16);
                    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 4);
                    if (BVar2 != 0) {
                        local_8 = (i_var3 + 0x1a);
                        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 4);
                        if (BVar2 != 0) {
                            local_8 = (i_var3 + 0x1e);
                            BVar2 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 4);
                            if (BVar2 != 0) {
                                local_8 = (i_var3 + 0x22);
                                BVar2 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(unaff_ss, &local_8),
                                    4,
                                );
                                if (BVar2 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_file_1010_648a(param_1: u32, param_2: *mut HFILE16) {
    let u_var1: u8;
    let mut i_var2: i32;
    let BVar3: bool;
    let extraout_var: u32;

    u_var1 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        i_var2 = param_1;
        BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 10)), 4);
        if (BVar3 != 0) {
            BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 0xe)), 4);
            if (BVar3 != 0) {
                BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 0x12)), 4);
                if (BVar3 != 0) {
                    BVar3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 0x16)), 4);
                    if (BVar3 != 0) {
                        BVar3 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (i_var2 + 0x1a)),
                            4,
                        );
                        if (BVar3 != 0) {
                            BVar3 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | (i_var2 + 0x1e)),
                                4,
                            );
                            if (BVar3 != 0) {
                                BVar3 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | (i_var2 + 0x22)),
                                    4,
                                );
                                if (BVar3 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1010_6846(param_1: u32, param_2: *mut HFILE16) {
    let u_var1: u8;
    let mut i_var2: i32;
    let BVar3: bool;
    let extraout_var: u32;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    u_var1 = write_to_file_1008_7cac(param_2, 3);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        i_var2 = param_1;
        BVar3 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var2 + 10)), 0x114);
        if (BVar3 != 0) {
            BVar3 =
                write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var2 + 0x11e)), 0x2a);
            if (BVar3 != 0) {
                local_c = (i_var2 + 0x148);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
                if (BVar3 != 0) {
                    return;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn read_file_1010_68c6(param_1: u32, param_2: *mut HFILE16) {
    let b_var1: bool;
    let u_var2: u8;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let b_var5: bool;
    let b_var6: bool;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let in_dx: *mut Struct199;
    
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = read_file_1008_7cfe(param_2);
    i_var3 = CONCAT31(extraout_var, u_var2);
    if (i_var3 == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
        return;
    }
    i_var4 = param_1;
    u_var8 = param_2;
    u_var9 = (param_2 >> 0x10);
    u_var7 = (param_1 >> 0x10);
    if (PTR_LOOP_1050_0312 < 2) {
        u_var10 = 0x102;
        u_var11 = 0;
        process_struct_1000_179c(0x102, in_dx);
        local_a = CONCAT22(in_dx, i_var3);
        b_var5 = read_file_1008_7dee(param_2, CONCAT22(in_dx, i_var3), CONCAT22(u_var11, u_var10));
        local_12 = local_a;
        if (b_var5 == 0) {}
        // goto LAB_1010_692c;
        local_4 = 1;
        while {
            switch_statement_1008_73ea(u_var8, u_var9, local_4);
            b_var6 = (local_4 * 2 + i_var3);
            (i_var4 + b_var5 * 2 + 10) = b_var6;
            local_4 = local_4 + 1;
            b_var5 = b_var6;
            local_4 < 0x81
        } {}
        in_dx = ctx.dx_reg;
        u_var2 = error_check_1000_17ce(local_a);
        b_var5 = CONCAT31(extraout_var_00, u_var2);
    } else {
        b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 10)), 0x114);
        if (b_var5 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    if (PTR_LOOP_1050_0312 < 2) {
        u_var10 = 0x2a;
        u_var11 = 0;
        process_struct_1000_179c(0x2a, in_dx);
        local_12 = CONCAT22(in_dx, b_var5);
        b_var6 = read_file_1008_7dee(param_2, CONCAT22(in_dx, b_var5), CONCAT22(u_var11, u_var10));
        if (b_var6 == 0) {
            // LAB_1010_692c:
            ctx.g_u16_1050_0310 = 0x6d2;
            error_check_1000_17ce((local_12 & 0xffff | ZEXT24(in_dx) << 0x10));
            return;
        }
        local_4 = 0;
        while {
            set_param_3_with_switch_1008_72bc(u_var8, u_var9, local_4);
            b_var1 = (local_4 * 2 + b_var5);
            (i_var4 + b_var6 * 2 + 0x11e) = b_var1;
            local_4 = local_4 + 1;
            b_var6 = b_var1;
            local_4 < 0x15
        } {}
        error_check_1000_17ce(local_12);
    } else {
        b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x11e)), 0x2a);
        if (b_var5 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    b_var5 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_6), 2);
    if (b_var5 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    switch_statement_1008_73ea(u_var8, u_var9, local_6);
    (i_var4 + 0x148) = b_var5;
    return;
}

pub unsafe fn write_to_file_1010_9900(ctx: &mut AppContext, param_1: u16, param_2: &mut Address<HFILE16>) -> bool {
    let mut u_var1: u32;
    let u_var2: u8;
    // let extraout_AH: u8;
    let BVar3: bool;
    let mut u_var4: u16;
    // let ctx.bx_reg: *mut Struct470;
    let mut u_var5: u16;
    // let mut unaff_ss: u16;
    let lVar6: u32;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: String;
    let mut local_4: u16;

    let b_var2 = write_to_file_1008_7cac(param_2, 1);
    if CONCAT11(ctx.ah_reg, b_var2) == 0 {
        return false;
    }
    // u_var5 = (param_1 >> 0x10);
    ctx.bx_reg = param_1;
    if ctx.bx_reg.field_0xa == 0 {
        local_4 = 0;
    } else {
        u_var1 = ctx.bx_reg.field_0xa;
        local_4 = (u_var1 + 8);
    }
    local_1c = local_4;
    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_1c), 2);
    if BVar3 == 0 {
        ctx.g_u16_1050_0310 = 0x6d0;
        return false;
    }
    pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, local_c), ctx.bx_reg.field_0xa);
    while {
        _local_10 = pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_c));
        if _local_10 == 0 {
            if ctx.bx_reg.field_0xe == 0 {
                local_24 = 0;
            } else {
                u_var1 = ctx.bx_reg.field_0xe;
                local_24 = (u_var1 + 8);
            }
            local_16 = local_24;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(ctx.stack_seg_reg, &local_16), 2);
            if BVar3 == 0 {
                ctx.g_u16_1050_0310 = 0x6d0;
                return false;
            }
            pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, local_c), ctx.bx_reg.field_0xe);
            while {
                lVar6 = pass1_1008_5b12(CONCAT22(unaff_ss, local_c));
                u_var4 = (lVar6 >> 0x10);
                if (lVar6 == 0) {
                    if (ctx.bx_reg.field_0x12 == 0) {
                        local_24 = 0;
                    } else {
                        u_var1 = ctx.bx_reg.field_0x12;
                        local_24 = (u_var1 + 8);
                    }
                    local_16 = local_24;
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_16), 2);
                    if BVar3 == 0 {
                        ctx.g_u16_1050_0310 = 0x6d0;
                        return 0;
                    }
                    pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, local_c), ctx.bx_reg.field_0x12);
                    while {
                        lVar6 = pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_c));
                        u_var4 = (lVar6 >> 0x10);
                        if lVar6 == 0 {
                            local_1c = ctx.bx_reg.field_0x1a;
                            BVar3 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1c), 2);
                            if BVar3 == 0 {
                                ctx.g_u16_1050_0310 = 0x6d0;
                                return 0;
                            }
                            local_1c = ctx.bx_reg.field_0x1c;
                            BVar3 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1c), 2);
                            if (BVar3 == 0) {
                                ctx.g_u16_1050_0310 = 0x6d0;
                                return 0;
                            }
                            local_1c = ctx.bx_reg.field_0x1e;
                            BVar3 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1c), 2);
                            if (BVar3 == 0) {
                                ctx.g_u16_1050_0310 = 0x6d0;
                                return 0;
                            }
                            return 1;
                        }
                        _local_10 = _local_10 & 0xffff0000 | *(lVar6 + 4);
                        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 2);
                        if (BVar3 == 0) {
                            ctx.g_u16_1050_0310 = 0x6d0;
                            return 0;
                        }
                        local_4 = (lVar6 + 6);
                        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_4), 2);
                        BVar3 != 0
                    } {}
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return 0;
                }
                _local_10 = _local_10 & 0xffff0000 | *(lVar6 + 4);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 2);
                if (BVar3 == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return 0;
                }
                local_4 = (lVar6 + 6);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_4), 2);
                BVar3 != 0
            } {}
            ctx.g_u16_1050_0310 = 0x6d0;
            return 0;
        }
        local_16 = (_local_10 + 4);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_16), 2);
        if (BVar3 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return 0;
        }
        local_16 = (_local_10 + 6);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_16), 2);
        BVar3 != 0
    } {}
    ctx.g_u16_1050_0310 = 0x6d0;
    return 0;
}

// WARNING: Could not reconcile some variable overlaps

pub fn read_file_1010_9b72(param_1: u32, param_2: *mut HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let struct_a_2: *mut Struct199;
    let paVar3: *mut Struct199;
    let b_result: bool;
    // let local_AX_165: *mut Struct473;
    let struct_b: *mut Struct1174;
    let mut u_var4: i32;
    // let local_AX_571: *mut Struct472;
    let mut u_var5: u16;
    let extraout_var: u32;
    let struct_a_1: *mut Struct199;
    let paVar6: *mut Struct199;
    
    // let ctx.dx_reg: *mut Struct199;
    // let ctx.dx_reg: *mut Struct199;
    let local_bx_60: *mut Struct471;
    let pu_var7: *mut u32;
    let local_es_60: *mut u8;
    let local_es_165: *mut u8;
    let mut u_var8: u16;
    let string_a_1: *mut libc::c_char;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_36: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let paStack38: *mut Struct473;
    let mut local_22: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let local_e: *mut Struct473;
    let string_a_2: *mut libc::c_char;
    let mut local_4: u16;

    u_var2 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        struct_a_2 = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_4), 2);
        if (struct_a_2 != 0x0) {
            local_bx_60 = param_1;
            local_es_60 = (param_1 >> 0x10);
            u_var9 = param_2;
            u_var10 = (param_2 >> 0x10);
            if (local_4 != 0) {
                if (&local_bx_60.field_0xa == 0) {
                    process_struct_1000_179c(0xc, struct_a_1);
                    _local_16 = CONCAT22(struct_a_1, struct_a_2);
                    paVar6 = (struct_a_1 | struct_a_2);
                    if (paVar6 == 0x0) {
                        &local_bx_60.field_0xa = 0;
                        struct_a_1 = paVar6;
                    } else {
                        paVar3 = process_struct_1008_574a(CONCAT22(struct_a_1, struct_a_2));
                        local_bx_60.field_0xa = paVar3;
                        &local_bx_60.field_0xc = paVar6;
                        struct_a_1 = paVar6;
                    }
                }
                local_12 = 0;
                while (local_12 < local_4) {
                    struct_b = local_4;
                    process_struct_1000_179c(8, struct_a_1);
                    _local_16 = CONCAT22(struct_a_1, struct_b);
                    if ((struct_a_1 | struct_b) == 0) {
                        local_e = 0x0;
                    } else {
                        &_local_16.field_0x0 = ctx.s_1_1050_389a;
                        struct_b.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                        &_local_16.field_0x0 = 0xa1c4;
                        struct_b.field_0x2 = 0x1010;
                        local_e = _local_16;
                    }
                    b_result = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &string_a_2), 2);
                    if (b_result == 0) {
                        // LAB_1010_9c32:
                        _local_16 = local_e;
                        if (local_e == 0x0) {}
                        // goto LAB_1010_9ba1;
                        u_var8 = (local_e >> 0x10);
                        pu_var7 = local_e;
                        // goto LAB_1010_9e9e;
                    }
                    b_result =
                        read_file_1008_7dee(param_2, (local_e & 0xffff0000 | (local_e + 6)), 2);
                    if (b_result == 0) {}
                    // goto LAB_1010_9c32;
                    switch_statement_1008_73ea(u_var9, u_var10, string_a_2);
                    (local_e + 4) = b_result;
                    pp_var1 = (&local_bx_60.field_0xa + 4);
                    (**pp_var1)();
                    local_12 = local_12 + 1;
                    struct_a_1 = ctx.dx_reg;
                }
            }
            u_var4 = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_2a), 2);
            if (u_var4 != 0) {
                if (local_2a != 0) {
                    if (&local_bx_60.field_0xe == 0) {
                        process_struct_1000_179c(0xc, struct_a_1);
                        local_2e = CONCAT22(struct_a_1, u_var4);
                        paVar6 = (struct_a_1 | u_var4);
                        if (paVar6 == 0x0) {
                            &local_bx_60.field_0xe = 0;
                            struct_a_1 = paVar6;
                        } else {
                            paVar3 = process_struct_1008_574a(CONCAT22(struct_a_1, u_var4));
                            &local_bx_60.field_0xe = paVar3;
                            local_bx_60.field_0x10 = paVar6;
                            struct_a_1 = paVar6;
                        }
                    }
                    local_22 = 0;
                    while (local_22 < local_2a) {
                        local_AX_571 = local_2a;
                        process_struct_1000_179c(8, struct_a_1);
                        _local_16 = CONCAT22(struct_a_1, local_AX_571);
                        if ((struct_a_1 | local_AX_571) == 0) {
                            local_1e = 0;
                        } else {
                            &_local_16.field_0x0 = ctx.s_1_1050_389a;
                            local_AX_571.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                            &_local_16.field_0x0 = 0xa1c4;
                            local_AX_571.field_0x2 = 0x1010;
                            local_1e = _local_16;
                        }
                        b_result = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_1a), 2);
                        if (b_result == 0) {
                            // LAB_1010_9d5c:
                            _local_16 = local_1e;
                            if (local_1e == 0) {}
                            // goto LAB_1010_9ba1;
                            u_var8 = (local_1e >> 0x10);
                            pu_var7 = local_1e;
                            // goto LAB_1010_9e9e;
                        }
                        b_result = read_file_1008_7dee(
                            param_2,
                            (local_1e & 0xffff0000 | (local_1e + 6)),
                            2,
                        );
                        if (b_result == 0) {}
                        // goto LAB_1010_9d5c;
                        switch_statement_1008_73ea(u_var9, u_var10, local_1a);
                        (local_1e + 4) = b_result;
                        pp_var1 = (&local_bx_60.field_0xe + 4);
                        (**pp_var1)();
                        local_22 = local_22 + 1;
                        struct_a_1 = ctx.dx_reg;
                    }
                }
                u_var4 = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_36), 2);
                if (u_var4 != 0) {
                    if (local_36 != 0) {
                        paVar6 = struct_a_1;
                        if (&local_bx_60.field_0x12 == 0) {
                            process_struct_1000_179c(0xc, struct_a_1);
                            _local_16 = CONCAT22(struct_a_1, u_var4);
                            paVar6 = (struct_a_1 | u_var4);
                            if (paVar6 == 0x0) {
                                &local_bx_60.field_0x12 = 0;
                            } else {
                                paVar3 = process_struct_1008_574a(CONCAT22(struct_a_1, u_var4));
                                local_bx_60.field_0x12 = paVar3;
                                &local_bx_60.field_0x14 = paVar6;
                            }
                        }
                        local_2a = 0;
                        while (local_2a < local_36) {
                            u_var5 = local_36;
                            process_struct_1000_179c(8, paVar6);
                            local_2e = CONCAT22(paVar6, u_var5);
                            if ((paVar6 | u_var5) == 0) {
                                paStack38 = 0x0;
                            } else {
                                local_2e = ctx.s_1_1050_389a;
                                (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                                local_2e = 0xa1c4;
                                (u_var5 + 2) = 0x1010;
                                paStack38 = local_2e;
                            }
                            b_result =
                                read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_22), 2);
                            if (b_result == 0) {
                                // LAB_1010_9e86:
                                _local_16 = paStack38;
                                if (paStack38 != 0x0) {
                                    u_var8 = (paStack38 >> 0x10);
                                    pu_var7 = paStack38;
                                    // LAB_1010_9e9e:
                                    pp_var1 = unsafe { *pu_var7 };
                                    local_2e = _local_16;
                                    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, _local_16, u_var8, 1);
                                }
                                // goto LAB_1010_9ba1;
                            }
                            b_result = read_file_1008_7dee(
                                param_2,
                                (paStack38 & 0xffff0000 | (paStack38 + 6)),
                                2,
                            );
                            if (b_result == 0) {}
                            // goto LAB_1010_9e86;
                            switch_statement_1008_73ea(u_var9, u_var10, local_22);
                            (paStack38 + 4) = b_result;
                            pp_var1 = (&local_bx_60.field_0x12 + 4);
                            (**pp_var1)();
                            local_2a = local_2a + 1;
                            paVar6 = ctx.dx_reg;
                        }
                    }
                    b_result = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_bx_60.field_0x1a),
                        2,
                    );
                    if (b_result != 0) {
                        b_result = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | &local_bx_60.field_0x1c),
                            2,
                        );
                        if (b_result != 0) {
                            b_result = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | ZEXT24(local_bx_60 + 1)),
                                2,
                            );
                            if (b_result != 0) {
                                return;
                            }
                        }
                    }
                }
            }
        }
        // LAB_1010_9ba1:
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_to_file_1010_ed58(param_1: u32, in_file: *mut HFILE16) {
    let pu_var1: *mut u16;
    let u_var2: u8;
    let BVar3: bool;
    let extraout_var: u32;
    // ppu_var4: *mut *mut u8;
    let local_bx_30: *mut Struct506;
    let local_es_30: *mut u8;
    let string_base_a: *mut u8;
    let local_22: *mut u8;
    let mut uStack30: u16;
    let mut string_offset_a: u32;
    let mut string_offset_b: u32;
    let mut local_4: u16;
    let temp_5fb1d7bd90: *mut u8;

    u_var2 = write_to_file_1008_7cac(in_file, 2);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        local_es_30 = (param_1 >> 0x10);
        local_bx_30 = param_1;
        string_offset_a = local_bx_30.field_0x16;
        BVar3 = write_to_file_1008_7e1c(in_file, CONCAT22(string_base_a, &string_offset_a), 4);
        if (BVar3 != 0) {
            string_offset_b = local_bx_30.field_0x1a;
            BVar3 = write_to_file_1008_7e1c(in_file, CONCAT22(string_base_a, &string_offset_b), 4);
            if (BVar3 != 0) {
                string_offset_b = local_bx_30.field_0x20;
                BVar3 =
                    write_to_file_1008_7e1c(in_file, CONCAT22(string_base_a, &string_offset_b), 4);
                if (BVar3 != 0) {
                    string_offset_b = local_bx_30.field_0x24;
                    BVar3 = write_to_file_1008_7e1c(
                        in_file,
                        CONCAT22(string_base_a, &string_offset_b),
                        4,
                    );
                    if (BVar3 != 0) {
                        string_offset_b = string_offset_b & 0xffff0000 | local_bx_30.field_0x30;
                        BVar3 = write_to_file_1008_7e1c(
                            in_file,
                            CONCAT22(string_base_a, &string_offset_b),
                            2,
                        );
                        if (BVar3 != 0) {
                            string_offset_b = string_offset_b & 0xffff0000 | local_bx_30.field_0x32;
                            BVar3 = write_to_file_1008_7e1c(
                                in_file,
                                CONCAT22(string_base_a, &string_offset_b),
                                2,
                            );
                            if (BVar3 != 0) {
                                local_4 = 0;
                                while (true) {
                                    pu_var1 = &local_bx_30.field_0x30;
                                    let pu_var1_val = unsafe { *pu_var1 };
                                    if (pu_var1_val == local_4 || pu_var1_val < local_4) {
                                        return;
                                    }
                                    temp_5fb1d7bd90 = local_bx_30.field_0x2e;
                                    ppu_var4 = (local_bx_30.field_0x2c + local_4 * 6);
                                    local_22 = *ppu_var4;
                                    uStack30 = (ppu_var4 + 1);
                                    ppu_var4 = &local_22;
                                    string_offset_a =
                                        string_offset_a & 0xffff0000 | ZEXT24(ppu_var4);
                                    write_file_1008_7b4c(
                                        in_file,
                                        CONCAT22(string_base_a, ppu_var4),
                                    );
                                    if (ppu_var4 == 0x0) {
                                        break;
                                    }
                                    local_4 = local_4 + 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn read_file_1018_0000(param_1: u32, param_2: *mut HFILE16) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let u_var3: u8;
    let mut i_var4: i32;
    let b_var5: bool;
    let puVar6: *mut u8;
    let extraout_var: u32;
    let in_dx: *mut u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_20: [u8; 16];
    let mut local_10: u16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    u_var3 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, u_var3) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        i_var4 = param_1;
        b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x16)), 4);
        if ((((b_var5 != 0)
            && (
                b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x1a)), 4),
                b_var5 != 0,
            ))
            && (
                b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x20)), 4),
                b_var5 != 0,
            ))
            && ((
                b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x24)), 4),
                b_var5 != 0
                    && (
                        b_var5 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (i_var4 + 0x30)),
                            2,
                        ),
                        b_var5 != 0,
                    ),
            ) && (
                b_var5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var4 + 0x32)), 2),
                b_var5 != 0,
            )))
        {
            u_var7 = (param_1 >> 0x10);
            if ((i_var4 + 0x30) != 0) {
                _g_Struct94_ptr_1 = ((i_var4 + 0x32) * 6);
                if (ctx.__g_Struct94_ptr_1 == 0) {
                    struct_fn_1000_160a();
                    ctx.g_u16_ptr_1050_5f2e = in_dx;
                } else {
                }
                alloc_mem_1000_1708();
                (i_var4 + 0x2c) = _g_Struct94_ptr_1;
                (i_var4 + 0x2e) = ctx.g_u16_ptr_1050_5f2e;
                zero_list_1008_3e38(CONCAT22(unaff_ss, local_20));
                local_10 = 0;
                let pu_var1_val = unsafe { *pu_var1 };
                while (
                    pu_var1 = (i_var4 + 0x30),
                    pu_var1_val != local_10 && local_10 <= pu_var1_val,
                ) {
                    puVar6 = local_20;
                    read_file_1008_7bc8(param_2, (param_2 >> 0x10), puVar6, unaff_ss);
                    if (puVar6 == 0x0) {
                        ctx.g_u16_1050_0310 = 0x6d0;
                        return;
                    }
                    u_var2 = (i_var4 + 0x2c);
                    modify_list_1008_3f62(
                        (u_var2 & 0xffff0000 | (u_var2 + local_10 * 6)),
                        CONCAT22(unaff_ss, local_20),
                    );
                    local_10 = local_10 + 1;
                }
            }
            return;
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_to_file_1038_7b20(param_1: *mut u32, param_2: *mut HFILE16) -> u16 {
    let mut u_var1: u32;
    let u_var2: u8;
    let extraout_AH: u8;
    let BVar3: bool;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let lVar6: u32;
    let pa_var7: *mut Struct961;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 8];
    let mut local_4: u16;

    u_var2 = write_to_file_1008_7cac(param_2, 0x17);
    if (CONCAT11(extraout_AH, u_var2) != 0) {
        let param_1_val = unsafe { *param_1 };
        local_1c = (param_1_val + 8);
        local_4 = local_1c;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1c), 2);
        if (BVar3 != 0) {
            pass1_1008_5784(CONCAT22(unaff_ss, local_c), param_1_val);
            while {
                lVar6 = pass1_1008_5b12(CONCAT22(unaff_ss, local_c));
                _local_10 = lVar6;
                if (lVar6 == 0) {
                    u_var5 = (param_1 >> 0x10);
                    u_var1 = (param_1 + 4);
                    local_1c = (u_var1 + 8);
                    local_4 = local_1c;
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_4), 2);
                    if (BVar3 == 0) {
                        return 0;
                    }
                    pass1_1008_5784(CONCAT22(unaff_ss, local_c), (param_1 + 4));
                    while {
                        pa_var7 = pass1_1008_5b12(CONCAT22(unaff_ss, local_c));
                        local_1a = pa_var7;
                        if (pa_var7 == 0x0) {
                            return 1;
                        }
                        write_to_file_1030_b768(pa_var7, param_2);
                        local_18 = (pa_var7 >> 0x10);
                        pa_var7 != 0
                    } {}
                    return 0;
                }
                i_var4 = write_to_file_1038_75ca(lVar6, param_2, (param_2 >> 0x10));
                i_var4 != 0
            } {}
        }
    }
    return 0;
}

pub unsafe fn read_from_file_1038_7c02(param_1: *mut *mut u8, file_b: *mut HFILE16) -> u16 {
    let u8_a: u8;
    // let extraout_AH: u8;
    let bool_a: bool;
    let mut u_var1: u16;
    let mut i_var2: i32;
    // let in_dx: *mut Struct199;
    
    let mut u_var3: i32;
    // let ctx.dx_reg: *mut Struct199;
    // let mut unaff_ss: u16;
    let ph_var4: *mut HFILE16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;
    let fn_ptr_a: fn();

    if (PTR_LOOP_1050_0312 < 2) {
        return 1;
    }
    u8_a = read_file_1008_7cfe(file_b);
    if ((CONCAT11(extraout_AH, u8_a) != 0)
        && (
            bool_a = read_file_1008_7dee(file_b, CONCAT22(unaff_ss, &local_4), 2),
            bool_a != 0,
        ))
    {
        while (local_4 != 0) {
            u_var1 = local_4;
            local_4 = local_4 - 1;
            ph_var4 = file_b;
            process_struct_1000_179c(0x2a, in_dx);
            if ((in_dx | u_var1) == 0) {
                _local_e = 0;
            } else {
                _local_e = pass1_1038_6520(CONCAT22(in_dx, u_var1));
            }
            i_var2 = read_from_file_1038_774e(_local_e, ph_var4);
            if (i_var2 == 0) {
                return 0;
            }
            fn_ptr_a = (param_1 + 4);
            (**fn_ptr_a)();
            in_dx = ctx.dx_reg;
        }
        local_4 = local_4 - 1;
        bool_a = read_file_1008_7dee(file_b, CONCAT22(unaff_ss, &local_12), 2);
        if (bool_a != 0) {
            while (true) {
                if (local_12 == 0) {
                    return 1;
                }
                u_var1 = local_12;
                local_12 = local_12 - 1;
                ph_var4 = file_b;
                process_struct_1000_179c(0x14, in_dx);
                u_var3 = in_dx | u_var1;
                if (u_var3 == 0) {
                    u_var1 = 0;
                    u_var3 = 0;
                } else {
                    pass1_1030_ae6c(CONCAT22(in_dx, u_var1));
                }
                i_var2 = pass1_1030_b836(CONCAT22(u_var3, u_var1), ph_var4);
                if (i_var2 == 0) {
                    break;
                }
                fn_ptr_a = ((param_1 + 4) + 4);
                (**fn_ptr_a)();
                in_dx = ctx.dx_reg;
            }
            return 0;
        }
    }
    return 0;
}

pub unsafe fn write_to_file_1038_75ca(param_1: u32, file_handle: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;
    let pu_var2: *mut u8;
    let local_bx_4: *mut Struct1167;
    let mut u_var3: u16;
    let mut base: u16;
    let mut offset_2: u32;
    let mut offset_1: u32;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    write_to_file_1008_79f0(file_handle, local_bx_4.field_0x4);
    if (in_ax != 0) {
        offset_2 = local_bx_4.field_0x8;
        b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_2), 4);
        if (b_var1 != 0) {
            write_to_file_1008_7a22(file_handle, local_bx_4.field_0xe);
            if (b_var1 != 0) {
                offset_1 = CONCAT22(offset_1._2_2_, local_bx_4.field_0xc);
                b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                if (b_var1 != 0) {
                    offset_1 = offset_1 & 0xffff0000 | local_bx_4.field_0x12;
                    b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                    if (b_var1 != 0) {
                        offset_1 = offset_1 & 0xffff0000 | local_bx_4.field_0x14;
                        b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                        if (b_var1 != 0) {
                            offset_1 = local_bx_4.field_0x16;
                            b_var1 =
                                write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 4);
                            if (b_var1 != 0) {
                                pu_var2 = (param_1 & 0xffff0000 | &local_bx_4.field_0x1a);
                                write_file_1008_7b4c(file_handle, pu_var2);
                                if (pu_var2 != 0) {
                                    offset_1 = local_bx_4.field_0x20;
                                    b_var1 = write_to_file_1008_7e1c(
                                        file_handle,
                                        CONCAT22(base, &offset_1),
                                        4,
                                    );
                                    if (b_var1 != 0) {
                                        offset_1 = offset_1 & 0xffff0000 | local_bx_4.field_0x24;
                                        b_var1 = write_to_file_1008_7e1c(
                                            file_handle,
                                            CONCAT22(base, &offset_1),
                                            2,
                                        );
                                        if (b_var1 != 0) {
                                            offset_1 =
                                                offset_1 & 0xffff0000 | local_bx_4.field_0x26;
                                            b_var1 = write_to_file_1008_7e1c(
                                                file_handle,
                                                CONCAT22(base, &offset_1),
                                                2,
                                            );
                                            if (b_var1 != 0) {
                                                offset_1 =
                                                    offset_1 & 0xffff0000 | local_bx_4.field_0x28;
                                                b_var1 = write_to_file_1008_7e1c(
                                                    file_handle,
                                                    CONCAT22(base, &offset_1),
                                                    2,
                                                );
                                                if (b_var1 != 0) {
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
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn read_from_file_1038_774e(param_1: u32, param_2: *mut HFILE16) {
    let mut u_var1: u16;
    let local_AX_22: *mut Struct1168;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut unaff_ss: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var4: u32;

    if (PTR_LOOP_1050_0312 < 2) {
        return;
    }
    local_AX_22 = param_1;
    local_AX_22 = &local_AX_22.field_0x4;
    u_var4 = param_1 & 0xffff0000 | ZEXT24(local_AX_22);
    pass1_1008_766e(param_2, u_var4);
    if ((u_var4 != 0)
        && (
            BVar2 =
                read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | &local_AX_22.field_0x8), 4),
            BVar2 != 0,
        ))
    {
        i_var3 = &local_AX_22.field_0xe;
        u_var1 = (param_1 >> 0x10);
        u_var5 = param_2;
        u_var6 = (param_2 >> 0x10);
        read_file_1008_77cc(u_var5, u_var6, i_var3, u_var1);
        if ((i_var3 != 0)
            && (((
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2),
                BVar2 != 0
                    && (
                        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_6), 2),
                        BVar2 != 0,
                    ),
            ) && (
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_8), 2),
                BVar2 != 0,
            )) && (
                BVar2 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_AX_22.field_0x16),
                    4,
                ),
                BVar2 != 0,
            )))
        {
            i_var3 = &local_AX_22.field_0x1a;
            read_file_1008_7bc8(u_var5, u_var6, i_var3, u_var1);
            if ((((i_var3 != 0)
                && (
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_22.field_0x20),
                        4,
                    ),
                    BVar2 != 0,
                ))
                && (
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_22.field_0x24),
                        2,
                    ),
                    BVar2 != 0,
                ))
                && ((
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_22.field_0x26),
                        2,
                    ),
                    BVar2 != 0
                        && (
                            BVar2 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_AX_22.field_0x28),
                                2,
                            ),
                            BVar2 != 0,
                        ),
                )))
            {
                local_AX_22.field_0xc = local_4;
                set_param_3_with_switch_1008_72bc(u_var5, u_var6, local_6);
                local_AX_22.field_0x12 = local_4;
                local_AX_22.field_0x14 = local_8;
                return;
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub unsafe fn read_from_file_1038_6118(param_1: *mut Struct933, param_2: *mut HFILE16) {
    let b_var1: bool;
    let u_var2: u8;
    let pu_var3: *mut u32;
    let b_var4: bool;
    let local_AX_307: *mut Struct1142;
    let pu_var5: *mut u8;
    let extraout_var: u32;
    
    let struct_a: *mut Struct199;
    let paVar6: *mut Struct199;
    let local_bx_68: *mut Struct1141;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_426: u32;
    let mut local_416: u16;
    let mut local_414: u16;
    let local_412: *mut Struct1143;
    let mut local_40e: u32;
    let mut local_408: [u8; 1024];
    let mut local_8: u16;
    let mut local_6: u32;

    u_var2 = pass1_1030_1730(param_1, param_2);
    if (CONCAT31(extraout_var, u_var2) == 0) {
        return;
    }
    local_6 = 0;
    pu_var3 = &local_6;
    u_var8 = (param_2 >> 0x10);
    read_file_1008_7548(param_2, u_var8, pu_var3, unaff_ss);
    if (pu_var3 != 0x0) {
        u_var7 = (param_1 >> 0x10);
        local_bx_68 = param_1;
        local_bx_68.field_0xc = local_6;
        struct_a = ctx.dx_reg;
        b_var4 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | &local_bx_68.field_0x10), 4);
        if (((((b_var4 != 0)
            && (
                b_var4 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x18),
                    2,
                ),
                b_var4 != 0,
            ))
            && (
                b_var4 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x1a),
                    2,
                ),
                b_var4 != 0,
            ))
            && ((
                b_var4 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_8), 2),
                b_var4 != 0
                    && (
                        b_var4 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | &local_bx_68.field_0x1e),
                            4,
                        ),
                        b_var4 != 0,
                    ),
            )))
            && (
                b_var4 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x22),
                    2,
                ),
                b_var4 != 0,
            ))
        {
            local_bx_68.field_0x1c = local_8;
            b_var4 =
                read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | &local_bx_68.field_0x24), 2);
            if ((b_var4 != 0)
                && (
                    local_AX_307 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x26),
                        0x94,
                    ),
                    local_AX_307 != 0x0,
                ))
            {
                if (PTR_LOOP_1050_0312 < 2) {
                    u_var9 = 0x54;
                    u_var10 = 0;
                    process_struct_1000_179c(0x54, struct_a);
                    _local_416 = CONCAT22(struct_a, local_AX_307);
                    b_var4 = read_file_1008_7dee(
                        param_2,
                        CONCAT22(struct_a, local_AX_307),
                        CONCAT22(u_var10, u_var9),
                    );
                    if (b_var4 == 0) {
                        // LAB_1038_626a:
                        ctx.g_u16_1050_0310 = 0x6d2;
                        error_check_1000_17ce(_local_416);
                        return;
                    }
                    local_412 = 0x0;
                    while {
                        set_param_3_with_switch_1008_72bc(param_2, u_var8, local_412);
                        b_var1 = (local_AX_307 + local_412 * 4);
                        u_var9 = (local_AX_307 + local_412 * 4 + 2);
                        (&local_bx_68.field_0x14e + b_var4 * 4) = b_var1;
                        (&local_bx_68.field_0x150 + b_var4 * 4) = u_var9;
                        local_412 = &local_412.field_0x1;
                        b_var4 = b_var1;
                        local_412 < 0x15
                    } {}
                    b_var4 = read_file_1008_7dee(param_2, _local_416, 0x54);
                    if (b_var4 == 0) {}
                    // goto LAB_1038_626a;
                    local_412 = 0x0;
                    while {
                        set_param_3_with_switch_1008_72bc(param_2, u_var8, local_412);
                        b_var1 = (local_AX_307 + local_412 * 4);
                        paVar6 = (local_AX_307 + local_412 * 4 + 2);
                        (&local_bx_68.field_0x1a2 + b_var4 * 4) = b_var1;
                        (&local_bx_68.field_0x1a4 + b_var4 * 4) = paVar6;
                        local_412 = &local_412.field_0x1;
                        b_var4 = b_var1;
                        local_412 < 0x15
                    } {}
                    error_check_1000_17ce(_local_416);
                    struct_a = paVar6;
                } else {
                    b_var4 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x14e),
                        0x54,
                    );
                    if (b_var4 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                    b_var4 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x1a2),
                        0x54,
                    );
                    if (b_var4 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                }
                read_from_file_1030_33f0(local_bx_68.field_0x1f6, param_2);
                pu_var5 = local_408;
                read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_ss, pu_var5));
                if (pu_var5 != 0x0) {
                    pu_var5 = local_408;
                    pass1_fn_1008_60e8(pu_var5);
                    local_bx_68.field_0x1fa = pu_var5;
                    local_bx_68.field_0x1fc = struct_a;
                    b_var4 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x1fe),
                        2,
                    );
                    if (((((b_var4 != 0)
                        && (
                            b_var4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | CONCAT11((param_1 >> 8) + 0x2, param_1)),
                                4,
                            ),
                            b_var4 != 0,
                        ))
                        && (
                            b_var4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_bx_68.field_0x204),
                                2,
                            ),
                            b_var4 != 0,
                        ))
                        && ((
                            b_var4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_bx_68.field_0x206),
                                2,
                            ),
                            b_var4 != 0
                                && (
                                    b_var4 = read_file_1008_7dee(
                                        param_2,
                                        (param_1 & 0xffff0000 | &local_bx_68.field_0x208),
                                        2,
                                    ),
                                    b_var4 != 0,
                                ),
                        ) && ((
                            b_var4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_bx_68.field_0x20a),
                                2,
                            ),
                            b_var4 != 0
                                && ((
                                    b_var4 = read_file_1008_7dee(
                                        param_2,
                                        (param_1 & 0xffff0000 | &local_bx_68.field_0x20c),
                                        2,
                                    ),
                                    b_var4 != 0
                                        && (
                                            b_var4 = read_file_1008_7dee(
                                                param_2,
                                                (param_1 & 0xffff0000 | &local_bx_68.field_0x20e),
                                                2,
                                            ),
                                            b_var4 != 0,
                                        ),
                                )),
                        ))))
                        && (PTR_LOOP_1050_0312 < 2
                            || ((
                                b_var4 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | &local_bx_68.field_0x214),
                                    2,
                                ),
                                b_var4 != 0
                                    && (
                                        b_var4 = read_file_1008_7dee(
                                            param_2,
                                            (param_1 & 0xffff0000 | ZEXT24(local_bx_68 + 1)),
                                            4,
                                        ),
                                        b_var4 != 0,
                                    ),
                            ))))
                    {
                        return;
                    }
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn write_to_file_1038_5e16(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;
    let pu_var2: *mut u32;
    let local_bx_28: *mut Struct1140;
    let mut u_var3: u16;
    let mut string_base: u16;
    let mut offset_1: u32;
    let mut offset_2: u32;
    let mut local_6: u32;

    write_to_file_1030_16d6(param_1, param_2);
    if (in_ax != 0) {
        u_var3 = (param_1 >> 0x10);
        local_bx_28 = param_1;
        pu_var2 = local_bx_28.field_0xc;
        local_6 = pu_var2;
        write_to_file_1008_7898(param_2, pu_var2);
        if (pu_var2 != 0) {
            offset_1 = local_bx_28.field_0x10;
            b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_1), 4);
            if (b_var1 != 0) {
                offset_2 = CONCAT22(offset_2._2_2_, local_bx_28.field_0x18);
                b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                if (b_var1 != 0) {
                    offset_2 = offset_2 & 0xffff0000 | local_bx_28.field_0x1a;
                    b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                    if (b_var1 != 0) {
                        offset_2 = offset_2 & 0xffff0000 | local_bx_28.field_0x1c;
                        b_var1 =
                            write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                        if (b_var1 != 0) {
                            offset_2 = local_bx_28.field_0x1e;
                            b_var1 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(string_base, &offset_2),
                                4,
                            );
                            if (b_var1 != 0) {
                                offset_2 = offset_2 & 0xffff0000 | local_bx_28.field_0x22;
                                b_var1 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(string_base, &offset_2),
                                    2,
                                );
                                if (b_var1 != 0) {
                                    offset_2 = offset_2 & 0xffff0000 | local_bx_28.field_0x24;
                                    b_var1 = write_to_file_1008_7e1c(
                                        param_2,
                                        CONCAT22(string_base, &offset_2),
                                        2,
                                    );
                                    if (b_var1 != 0) {
                                        b_var1 = write_to_file_1008_7e1c(
                                            param_2,
                                            (param_1 & 0xffff0000 | &local_bx_28.field_0x26),
                                            0x94,
                                        );
                                        if (b_var1 != 0) {
                                            b_var1 = write_to_file_1008_7e1c(
                                                param_2,
                                                (param_1 & 0xffff0000 | &local_bx_28.field_0x14e),
                                                0x54,
                                            );
                                            if (b_var1 != 0) {
                                                b_var1 = write_to_file_1008_7e1c(
                                                    param_2,
                                                    (param_1 & 0xffff0000
                                                        | &local_bx_28.field_0x1a2),
                                                    0x54,
                                                );
                                                if (b_var1 != 0) {
                                                    write_to_file_1030_32e4(
                                                        local_bx_28.field_0x1f6,
                                                        param_2,
                                                    );
                                                    write_to_file_1008_7c2a(
                                                        param_2,
                                                        local_bx_28.field_0x1fa,
                                                    );
                                                    if (b_var1 != 0) {
                                                        offset_2 = offset_2 & 0xffff0000
                                                            | local_bx_28.field_0x1fe;
                                                        b_var1 = write_to_file_1008_7e1c(
                                                            param_2,
                                                            CONCAT22(string_base, &offset_2),
                                                            2,
                                                        );
                                                        if (b_var1 != 0) {
                                                            offset_2 = local_bx_28.field_0x200;
                                                            b_var1 = write_to_file_1008_7e1c(
                                                                param_2,
                                                                CONCAT22(string_base, &offset_2),
                                                                4,
                                                            );
                                                            if (b_var1 != 0) {
                                                                offset_2 = offset_2 & 0xffff0000
                                                                    | local_bx_28.field_0x204;
                                                                b_var1 = write_to_file_1008_7e1c(
                                                                    param_2,
                                                                    CONCAT22(
                                                                        string_base,
                                                                        &offset_2,
                                                                    ),
                                                                    2,
                                                                );
                                                                if (b_var1 != 0) {
                                                                    offset_2 = offset_2
                                                                        & 0xffff0000
                                                                        | local_bx_28.field_0x206;
                                                                    b_var1 =
                                                                        write_to_file_1008_7e1c(
                                                                            param_2,
                                                                            CONCAT22(
                                                                                string_base,
                                                                                &offset_2,
                                                                            ),
                                                                            2,
                                                                        );
                                                                    if (b_var1 != 0) {
                                                                        offset_2 = offset_2
                                                                            & 0xffff0000
                                                                            | local_bx_28
                                                                                .field_0x208;
                                                                        b_var1 =
                                                                            write_to_file_1008_7e1c(
                                                                                param_2,
                                                                                CONCAT22(
                                                                                    string_base,
                                                                                    &offset_2,
                                                                                ),
                                                                                2,
                                                                            );
                                                                        if (b_var1 != 0) {
                                                                            offset_2 = offset_2
                                                                                & 0xffff0000
                                                                                | local_bx_28
                                                                                    .field_0x20a;
                                                                            b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                            if (b_var1 != 0) {
                                                                                offset_2 = offset_2 & 0xffff0000 |
                                                                                           local_bx_28.field_0x20c;
                                                                                b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                                if (b_var1 != 0) {
                                                                                    offset_2 = offset_2 & 0xffff0000 |
                                                                                               local_bx_28.field_0x20e;
                                                                                    b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                                    if (b_var1 != 0)
                                                                                    {
                                                                                        offset_2 = offset_2 & 0xffff0000 |
                                                                                                   local_bx_28.field_0x214;
                                                                                        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                                        if (b_var1
                                                                                            != 0)
                                                                                        {
                                                                                            offset_2 = local_bx_28.field_0x216;
                                                                                            b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 4);
                                                                                            if (b_var1 != 0)
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
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_from_file_1030_dec4(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;

    read_file_fn_1028_b81a(param_1, param_2);
    if (((in_ax != 0) && (1 < PTR_LOOP_1050_0312))
        && (
            b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 4),
            b_var1 == 0,
        ))
    {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn write_to_file_1030_de7c(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_10: u32;

    write_to_file_1028_b5ec(param_1, param_2);
    if (in_ax != 0) {
        local_10 = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}

pub fn write_to_file_1030_d61c(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_success: bool;
    let mut i_var1: i32;
    let mut local_es_61: u16;
    let mut string_base: u16;
    let mut string_off_4: u32;
    let mut string_off_3: u16;
    let mut string_off_2: u16;
    let mut string_off_1: u32;
    let mut local_4: u16;

    write_to_file_1028_b5ec(param_1, param_2);
    if (in_ax != 0) {
        local_4 = 0;
        while (local_4 < 10) {
            local_es_61 = (param_1 >> 0x10);
            i_var1 = param_1;
            string_off_1 = (i_var1 + local_4 * 0xc + 0x20);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_1), 4);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            string_off_2 = (i_var1 + local_4 * 0xc + 0x24);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_2), 2);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            string_off_3 = (i_var1 + local_4 * 0xc + 0x26);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_3), 2);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            string_off_4 = (i_var1 + local_4 * 0xc + 0x28);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_4), 4);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            local_4 = local_4 + 1;
        }
        string_off_3 = PTR_LOOP_1050_5812;
        b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_3), 2);
        if (b_success != 0) {
            return;
        }
        // LAB_1030_d701:
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn pass1_1030_d72e(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;
    let mut i_var2: i32;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_ax == 0) {
        return;
    }
    local_a = 0;
    while (true) {
        if (9 < local_a) {
            if ((3 < PTR_LOOP_1050_0312)
                && (
                    b_var1 = read_file_1008_7dee(param_2, &PTR_LOOP_1050_5812, 2),
                    b_var1 == 0,
                ))
            {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            return;
        }
        b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_8), 4);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
        b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (b_var1 == 0) {
            break;
        }
        i_var2 = local_a * 0xc + param_1;
        (i_var2 + 0x20) = local_8;
        (i_var2 + 0x22) = local_6;
        set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_4);
        (i_var2 + 0x24) = local_8;
        if (PTR_LOOP_1050_0312 < 2) {
            i_var2 = local_a * 0xc + param_1;
            (i_var2 + 0x26) = 3;
            (i_var2 + 0x28) = 0;
        } else {
            b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
            if (b_var1 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_8), 4);
            if (b_var1 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            i_var2 = local_a * 0xc + param_1;
            (i_var2 + 0x26) = local_4;
            (i_var2 + 0x28) = _local_8;
        }
        local_a = local_a + 1;
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_c84e(param_1: *mut Struct771, param_2: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write_to_file_1028_b5ec(param_1, param_2);
    if (b_var1 != 0) {
        local_c = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub fn read_from_file_1030_c894(param_1: u32, param_2: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    b_var1 = read_file_fn_1028_b81a(param_1, param_2);
    if (b_var1 != 0) {
        b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return b_var1;
        }
        (param_1 + 0x20) = local_4;
        b_var1 = 1;
    }
    return b_var1;
}

pub fn write_to_file_1030_c230(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_8: u16;

    write_to_file_1028_b5ec(param_1, param_2);
    if (in_ax != 0) {
        u_var2 = (param_1 >> 0x10);
        local_10 = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
        if (b_var1 != 0) {
            local_8 = (param_1 + 0x24);
            b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 2);
            if (b_var1 != 0) {
                return;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_from_file_1030_c29c(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_ax != 0) {
        b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 4);
        if (b_var1 != 0) {
            b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x24)), 2);
            if (b_var1 != 0) {
                return;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_to_file_1030_b768(param_1: *mut Struct961, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let pu_var3: *mut u8;
    let pu_var4: *mut u8;
    
    let local_bx_4: *mut Struct961;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_22: u16;
    let mut local_1a: [u8; 10];
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    local_10 = local_bx_4.field_0x4;
    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
    if (BVar2 != 0) {
        pu_var4 = (param_1 & 0xffff0000 | &local_bx_4.field_0x8);
        write_file_1008_7b4c(param_2, pu_var4);
        if (pu_var4 != 0) {
            local_8 = local_bx_4.field_0xe;
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 2);
            if (BVar2 != 0) {
                u_var1 = local_bx_4.field_0x10;
                local_22 = (u_var1 + 8);
                local_10 = local_10 & 0xffff0000 | local_22;
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_22), 2);
                if (BVar2 == 0) {
                    return;
                }
                pass1_1008_5784(CONCAT22(unaff_ss, local_1a), local_bx_4.field_0x10);
                while {
                    pu_var3 = local_1a;
                    pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
                    if ((ctx.dx_reg | pu_var3) == 0) {
                        return;
                    }
                    local_c = pu_var3;
                    local_a = ctx.dx_reg;
                    write_to_file_1038_75ca(pu_var3, ctx.dx_reg, param_2);
                    pu_var3 != 0x0
                } {}
                return;
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn pass1_1030_b836(struct_b: *mut Struct962, file_a: *mut HFILE16) {
    let pp_var1: fn();
    // let local_AX_11: *mut Struct962;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut u_var4: u32;
    
    let struct_a: *mut Struct199;
    
    let mut u_var5: u16;
    // let ctx.dx_reg: *mut Struct199;
    let mut u_var6: u16;
    // let mut unaff_ss: u16;
    let pHVar7: *mut HFILE16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_4: u16;

    local_AX_11 = struct_b;
    local_AX_11 = &local_AX_11.field_0x4;
    BVar2 = read_file_1008_7dee(file_a, (struct_b & 0xffff0000 | ZEXT24(local_AX_11)), 4);
    if (BVar2 != 0) {
        u_var4 = struct_b & 0xffff0000 | &local_AX_11.field_0x8;
        read_file_1008_7bc8(file_a, u_var4);
        if ((u_var4 != 0)
            && (
                struct_a = ctx.dx_reg,
                BVar2 = read_file_1008_7dee(file_a, CONCAT22(unaff_ss, &local_4), 2),
                BVar2 != 0,
            ))
        {
            u_var6 = (struct_b >> 0x10);
            local_AX_11.field_0xe = local_4;
            BVar2 = read_file_1008_7dee(file_a, CONCAT22(unaff_ss, &local_12), 2);
            if (BVar2 == 0) {
                return;
            }
            while (true) {
                if (local_12 == 0) {
                    return;
                }
                u_var3 = local_12;
                pHVar7 = file_a;
                local_12 = local_12 - 1;
                process_struct_1000_179c(0x2a, struct_a);
                if ((struct_a | u_var3) == 0) {
                    u_var3 = 0;
                    u_var5 = 0;
                } else {
                    pass1_1038_6520(CONCAT22(struct_a, u_var3));
                    u_var5 = ctx.dx_reg;
                }
                read_from_file_1038_774e(u_var3, u_var5, pHVar7);
                if (u_var3 == 0) {
                    break;
                }
                pp_var1 = (local_AX_11.field_0x10 + 4);
                (**pp_var1)();
                struct_a = ctx.dx_reg;
            }
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub unsafe fn write_to_file_1030_7418(struct_a: *mut Struct771, file_a: *mut HFILE16) {
    let mut u_var1: u32;
    let mut in_ax: i32;
    let struct_b: *mut Struct771;
    let b_success: bool;
    let pc_var2: *mut libc::c_char;
    
    
    let mut local_es_52: u16;
    let char_ptr_a: *mut libc::c_char;
    let mut local_3e: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_2a: u16;
    let mut char_array_14_a: [u8; 14];
    let char_ptr_a_4: *mut libc::c_char;
    let mut local_14: u16;
    let mut local_12: u16;
    let char_ptr_a_2: *mut libc::c_char;
    let char_ptr_a_5: *mut libc::c_char;
    let char_ptr_a_3: *mut libc::c_char;
    let pu_var3: *mut u8;

    write_to_file_1030_16d6(struct_a, file_a);
    if (in_ax == 0) {
        return;
    }
    struct_b = struct_a;
    struct_b = &struct_b.field_0xc;
    pu_var3 = (struct_a & 0xffff0000 | ZEXT24(struct_b));
    write_file_1008_7b4c(file_a, pu_var3);
    if (pu_var3 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    local_es_52 = (struct_a >> 0x10);
    char_ptr_a_2 = struct_b.field_0x12;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_3 = *&struct_b.field_0x14;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_4 = *&struct_b.field_0x16;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_4), 4);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7954(file_a, &struct_b.field_0x1e);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(file_a, (&struct_b.field_0x20 + 2));
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(file_a, (&struct_b.field_0x24 + 2));
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_5 = *(&struct_b.field_0x28 + 2);
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_5), 4);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_2 = *&struct_b[1].field_0x6;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_2 = *&struct_b[1].field_0x8;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_79f0(file_a, &struct_b[1].field_0xa);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    if (&struct_b[1].field_0xe == 0) {
        char_ptr_a_4 = (char_ptr_a_4 & 0xffff0000);
    } else {
        u_var1 = &struct_b[1].field_0xe;
        char_ptr_a_4 = (char_ptr_a_4 & 0xffff0000 | *(u_var1 + 8));
    }
    char_ptr_a_3 = char_ptr_a_4;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_5784(
        CONCAT22(char_ptr_a, char_array_14_a),
        &struct_b[1].field_0xe,
    );
    while (true) {
        pc_var2 = char_array_14_a;
        pass1_1008_5b12(CONCAT22(char_ptr_a, pc_var2));
        _local_14 = CONCAT22(ctx.dx_reg, pc_var2);
        if ((ctx.dx_reg | pc_var2) == 0) {
            if (&struct_b[1].field_0x12 == 0) {
                local_3e = 0;
            } else {
                u_var1 = &struct_b[1].field_0x12;
                local_3e = (u_var1 + 8);
            }
            local_2a = local_3e;
            b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &local_2a), 2);
            if (b_success == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return;
            }
            pass1_1008_5784(
                CONCAT22(char_ptr_a, char_array_14_a),
                &struct_b[1].field_0x12,
            );
            while (true) {
                pc_var2 = char_array_14_a;
                pass1_1008_5b12(CONCAT22(char_ptr_a, pc_var2));
                if ((ctx.dx_reg | pc_var2) == 0) {
                    return;
                }
                char_ptr_a_4 = (char_ptr_a_4 & 0xffff0000 | *(pc_var2 + 4));
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_4), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                _local_14 = _local_14 & 0xffff0000 | *(pc_var2 + 6);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &local_14), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                char_ptr_a_2 = *(pc_var2 + 8);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
                if (b_success == 0) {
                    break;
                }
                char_ptr_a_2 = *(pc_var2 + 10);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                char_ptr_a_3 = *(pc_var2 + 0xc);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
            }
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(pc_var2 + 4);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 6);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            break;
        }
        char_ptr_a_3 = *(_local_14 + 8);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 10);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 0xc);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn read_from_file_1030_778c(struct_b: *mut Struct933, file_a: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let local_AX_32: *mut Struct933;
    let BVar3: bool;
    let paVar4: *mut Struct493;
    let local_AX_482: *mut Struct935;
    let mut u_var5: u16;
    let local_AX_662: *mut Struct934;
    let local_AX_1036: *mut Struct936;
    let extraout_var: u32;
    
    let mut u_var8: i32;
    
    let paVar9: *mut Struct199;
    
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let base_ptr: *mut u8;
    let mut local_56: u16;
    let mut local_52: u16;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_42: u16;
    let mut local_3e: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let local_2c: *mut u8;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_e: u16;
    let offset_ptr: *mut u8;
    let mut u_var6: u32;
    let pu_var7: *mut u8;
    let fn_ptr_a: fn();

    u_var2 = pass1_1030_1730(struct_b, file_a);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        local_AX_32 = struct_b;
        local_AX_32 = &local_AX_32.field_0xc;
        u_var6 = struct_b & 0xffff0000 | ZEXT24(local_AX_32);
        read_file_1008_7bc8(file_a, u_var6);
        if ((u_var6 != 0)
            && (
                BVar3 = read_file_1008_7dee(file_a, CONCAT22(base_ptr, &offset_ptr), 2),
                BVar3 != 0,
            ))
        {
            u_var11 = (struct_b >> 0x10);
            local_AX_32.field_0x12 = offset_ptr;
            BVar3 = read_file_1008_7dee(file_a, CONCAT22(base_ptr, &offset_ptr), 2);
            if (BVar3 != 0) {
                local_AX_32.field_0x14 = offset_ptr;
                BVar3 = read_file_1008_7dee(
                    file_a,
                    (struct_b & 0xffff0000 | &local_AX_32.field_0x16),
                    4,
                );
                if (BVar3 != 0) {
                    u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x1e;
                    read_file_1008_76e4(file_a, u_var6);
                    if (u_var6 != 0) {
                        u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x22;
                        read_file_1008_77cc(file_a, u_var6);
                        if (u_var6 != 0) {
                            u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x26;
                            read_file_1008_77cc(file_a, u_var6);
                            if ((u_var6 != 0)
                                && (
                                    u_var12 = ctx.dx_reg,
                                    BVar3 = read_file_1008_7dee(
                                        file_a,
                                        (struct_b & 0xffff0000 | ZEXT24(&local_AX_32.field_0x2a)),
                                        4,
                                    ),
                                    BVar3 != 0,
                                ))
                            {
                                if (local_AX_32.field_0x2a != 0) {
                                    u_var1 = local_AX_32.field_0x2a;
                                    paVar4 = pass1_1028_e1ec(
                                        ctx._PTR_LOOP_1050_65e2,
                                        u_var1,
                                        (u_var1 >> 0x10),
                                    );
                                    local_AX_32.field_0x2e = paVar4;
                                    local_AX_32.field_0x30 = u_var12;
                                }
                                if (PTR_LOOP_1050_0312 < 2) {
                                    return;
                                }
                                BVar3 = read_file_1008_7dee(
                                    file_a,
                                    (struct_b & 0xffff0000 | &local_AX_32.field_0x32),
                                    2,
                                );
                                if ((BVar3 != 0)
                                    && (
                                        BVar3 = read_file_1008_7dee(
                                            file_a,
                                            (struct_b & 0xffff0000 | &local_AX_32.field_0x34),
                                            2,
                                        ),
                                        BVar3 != 0,
                                    ))
                                {
                                    u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x36;
                                    pass1_1008_766e(file_a, u_var6);
                                    if ((u_var6 != 0)
                                        && (
                                            BVar3 = read_file_1008_7dee(
                                                file_a,
                                                CONCAT22(base_ptr, &local_20),
                                                2,
                                            ),
                                            BVar3 != 0,
                                        ))
                                    {
                                        local_e = 0;
                                        while (local_e < local_20) {
                                            local_3e = _PTR_LOOP_1050_68a2;
                                            pu_var7 = _PTR_LOOP_1050_68a2;
                                            alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                                            u_var8 = pu_var7;
                                            _local_32 = (pu_var7 & 0xffff | ctx.dx_reg << 0x10);
                                            paVar9 = (ctx.dx_reg | u_var8);
                                            if (paVar9 == 0x0) {
                                                local_2c = 0x0;
                                            } else {
                                                *_local_32 = ctx.s_1_1050_389a;
                                                (u_var8 + 2) = &ctx.PTR_LOOP_1050_1008;
                                                (u_var8 + 4) = 0;
                                                (u_var8 + 6) = 0;
                                                (u_var8 + 8) = 0;
                                                (u_var8 + 10) = 0;
                                                (u_var8 + 0xc) = 0;
                                                *_local_32 = 0x56ce;
                                                (u_var8 + 2) = 0x1018;
                                                local_2c = _local_32;
                                            }
                                            BVar3 = read_file_1008_7dee(
                                                file_a,
                                                CONCAT22(base_ptr, &local_28),
                                                2,
                                            );
                                            if ((((BVar3 == 0)
                                                || (
                                                    BVar3 = read_file_1008_7dee(
                                                        file_a,
                                                        CONCAT22(base_ptr, &local_24),
                                                        2,
                                                    ),
                                                    BVar3 == 0,
                                                ))
                                                || (
                                                    BVar3 = read_file_1008_7dee(
                                                        file_a,
                                                        CONCAT22(base_ptr, &local_2e),
                                                        2,
                                                    ),
                                                    BVar3 == 0,
                                                ))
                                                || ((
                                                    local_AX_482 = local_2c,
                                                    local_AX_482 = &local_AX_482.field_0xa,
                                                    BVar3 = read_file_1008_7dee(
                                                        file_a,
                                                        (local_2c & 0xffff0000
                                                            | ZEXT24(local_AX_482)),
                                                        2,
                                                    ),
                                                    BVar3 == 0
                                                        || (
                                                            BVar3 = read_file_1008_7dee(
                                                                file_a,
                                                                (local_2c & 0xffff0000
                                                                    | (local_2c + 0xc)),
                                                                2,
                                                            ),
                                                            BVar3 == 0,
                                                        ),
                                                )))
                                            {
                                                // goto LAB_1030_77be;
                                            }
                                            u_var12 = (local_2c >> 0x10);
                                            i_var10 = local_2c;
                                            (i_var10 + 4) = local_28;
                                            (i_var10 + 6) = local_24;
                                            (i_var10 + 8) = local_2e;
                                            if (&local_AX_32.field_0x3a == 0) {
                                                u_var5 = local_2e;
                                                process_struct_1000_179c(0xc, paVar9);
                                                _local_32 = CONCAT22(paVar9, u_var5);
                                                u_var8 = paVar9 | u_var5;
                                                if (u_var8 == 0) {
                                                    &local_AX_32.field_0x3a = 0;
                                                } else {
                                                    paVar9 = process_struct_1008_574a(CONCAT22(
                                                        paVar9, u_var5,
                                                    ));
                                                    local_AX_32.field_0x3a = paVar9;
                                                    &local_AX_32.field_0x3c = u_var8;
                                                }
                                            }
                                            fn_ptr_a = (&local_AX_32.field_0x3a + 8);
                                            (**fn_ptr_a)();
                                            local_e = local_e + 1;
                                        }
                                        BVar3 = read_file_1008_7dee(
                                            file_a,
                                            CONCAT22(base_ptr, &local_56),
                                            2,
                                        );
                                        if (BVar3 != 0) {
                                            local_52 = 0;
                                            while (true) {
                                                if (local_56 <= local_52) {
                                                    return;
                                                }
                                                local_2c = _PTR_LOOP_1050_68a2;
                                                pu_var7 = _PTR_LOOP_1050_68a2;
                                                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                                                u_var8 = pu_var7;
                                                _local_32 =
                                                    (pu_var7 & 0xffff | ctx.dx_reg << 0x10);
                                                paVar9 = (ctx.dx_reg | u_var8);
                                                if (paVar9 == 0x0) {
                                                    local_4a = 0;
                                                } else {
                                                    *_local_32 = ctx.s_1_1050_389a;
                                                    (u_var8 + 2) = &ctx.PTR_LOOP_1050_1008;
                                                    (u_var8 + 4) = 0;
                                                    (u_var8 + 6) = 0;
                                                    (u_var8 + 8) = 0;
                                                    (u_var8 + 10) = 0;
                                                    (u_var8 + 0xc) = 0;
                                                    *_local_32 = 0x56ce;
                                                    (u_var8 + 2) = 0x1018;
                                                    local_4a = _local_32;
                                                }
                                                BVar3 = read_file_1008_7dee(
                                                    file_a,
                                                    CONCAT22(base_ptr, &local_46),
                                                    2,
                                                );
                                                if (((BVar3 == 0)
                                                    || (
                                                        BVar3 = read_file_1008_7dee(
                                                            file_a,
                                                            CONCAT22(base_ptr, &local_42),
                                                            2,
                                                        ),
                                                        BVar3 == 0,
                                                    ))
                                                    || ((
                                                        BVar3 = read_file_1008_7dee(
                                                            file_a,
                                                            CONCAT22(base_ptr, &local_3e),
                                                            2,
                                                        ),
                                                        BVar3 == 0
                                                            || ((
                                                                BVar3 = read_file_1008_7dee(
                                                                    file_a,
                                                                    (local_4a & 0xffff0000
                                                                        | (local_4a + 10)),
                                                                    2,
                                                                ),
                                                                BVar3 == 0
                                                                    || (
                                                                        BVar3 = read_file_1008_7dee(
                                                                            file_a,
                                                                            (local_4a & 0xffff0000
                                                                                | (local_4a + 0xc)),
                                                                            2,
                                                                        ),
                                                                        BVar3 == 0,
                                                                    ),
                                                            )),
                                                    )))
                                                {
                                                    break;
                                                }
                                                u_var12 = (local_4a >> 0x10);
                                                i_var10 = local_4a;
                                                (i_var10 + 4) = local_46;
                                                (i_var10 + 6) = local_42;
                                                (i_var10 + 8) = local_3e;
                                                if (&local_AX_32.field_0x3e == 0) {
                                                    process_struct_1000_179c(0xc, paVar9);
                                                    _local_32 = CONCAT22(paVar9, local_3e);
                                                    u_var8 = paVar9 | local_3e;
                                                    if (u_var8 == 0) {
                                                        &local_AX_32.field_0x3e = 0;
                                                    } else {
                                                        paVar9 = process_struct_1008_574a(
                                                            CONCAT22(paVar9, local_3e),
                                                        );
                                                        &local_AX_32.field_0x3e = paVar9;
                                                        local_AX_32.field_0x40 = u_var8;
                                                    }
                                                }
                                                fn_ptr_a = (&local_AX_32.field_0x3e + 8);
                                                (**fn_ptr_a)();
                                                local_52 = local_52 + 1;
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
        // LAB_1030_77be:
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_to_file_1030_5dbe(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let b_var4: bool;
    let pu_var5: *mut u8;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    i_var3 = write_to_file_1030_1978(param_1, param_2);
    if (i_var3 != 0) {
        u_var7 = (param_1 >> 0x10);
        i32_var6 = param_1;
        write_to_file_1008_7c2a(param_2, (i32_var6 + 0x10));
        if (i_var3 != 0) {
            u_var1 = (i32_var6 + 0x10);
            pu_var5 = (u_var1 & 0xffff0000 | (u_var1 + 4));
            write_file_1008_7b4c(param_2, pu_var5);
            if (pu_var5 != 0) {
                u_var2 = (i32_var6 + 0x10);
                local_c = (u_var2 + 10);
                b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
                if (b_var4 != 0) {
                    u_var2 = (i32_var6 + 0x10);
                    if ((u_var2 + 10) == 0) {
                        return;
                    }
                    u_var2 = (i32_var6 + 0x10);
                    u_var7 = (u_var2 >> 0x10);
                    i_var3 = u_var2;
                    b_var4 = write_to_file_1008_7e1c(param_2, (i_var3 + 0xc), ((i_var3 + 10) * 2));
                    if (b_var4 != 0) {
                        return;
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn read_from_file_1030_5e70(param_1: u32, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let pi_var3: *mut i32;
    let in_ax: *mut Struct94;
    let pu_var4: *mut u8;
    let mut i_var5: i32;
    let b_var6: bool;
    let mut u_var7: i32;
    let in_dx: *mut u16;
    let puVar8: *mut u16;
    
    let struct_a: *mut Struct199;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let pp_var11: *mut pass1_struct_1;
    let mut u_var12: u16;
    let mut local_420: u32;
    let mut local_416: u16;
    let mut local_414: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    u_var12 = (param_1 >> 0x10);
    pass1_1030_19b4(param_1, param_2);
    if (in_ax != 0x0) {
        if (ctx.__g_Struct94_ptr_1 == 0) {
            _g_Struct94_ptr_1 = in_ax;
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        alloc_mem_1000_1708();
        _local_40a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, _g_Struct94_ptr_1);
        puVar8 = (ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1);
        i_var9 = param_1;
        if (puVar8 == 0x0) {
            (i_var9 + 0x10) = 0;
        } else {
            zero_list_1008_3e38(CONCAT22(
                ctx.g_u16_ptr_1050_5f2e,
                &_g_Struct94_ptr_1.field_0x4,
            ));
            (i_var9 + 0x10) = _local_40a;
            puVar8 = ctx.g_u16_ptr_1050_5f2e;
        }
        pu_var4 = local_402;
        read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_ss, pu_var4));
        if (pu_var4 != 0x0) {
            pu_var4 = local_402;
            pass1_fn_1008_60e8(pu_var4);
            pi_var3 = (i_var9 + 0x10);
            let pi_var3_val = unsafe { *pi_var3 };
            unsafe { *pi_var3 = pu_var4 };
            (pi_var3 + 2) = puVar8;
            u_var1 = (i_var9 + 0x10);
            i_var5 = u_var1 + 4;
            read_file_1008_7bc8(param_2, (param_2 >> 0x10), i_var5, (u_var1 >> 0x10));
            if (i_var5 != 0) {
                u_var2 = (i_var9 + 0x10);
                local_420._0_2_ = u_var2 + 10;
                struct_a = ctx.dx_reg;
                b_var6 = read_file_1008_7dee(param_2, (u_var2 & 0xffff0000 | local_420), 2);
                if (b_var6 != 0) {
                    u_var1 = (i_var9 + 0x10);
                    u_var10 = (u_var1 >> 0x10);
                    i_var5 = u_var1;
                    if ((i_var5 + 10) == 0) {
                        // LAB_1030_5fb7:
                        pp_var11 = process_struct_1010_20ba(
                            ctx._g_Struct372_1050_0ed0,
                            CONCAT22(local_420, 0x2f),
                        );
                        pass1_1018_04ca(pp_var11, (i_var9 + 4));
                        return;
                    }
                    local_420._0_2_ = (i_var5 + 10) * 2;
                    u_var7 = local_420;
                    process_struct_1000_179c(local_420, struct_a);
                    u_var1 = (i_var9 + 0x10);
                    u_var10 = (u_var1 >> 0x10);
                    i_var5 = u_var1;
                    (i_var5 + 0xc) = u_var7;
                    (i_var5 + 0xe) = struct_a;
                    u_var1 = (i_var9 + 0x10);
                    b_var6 = read_file_1008_7dee(param_2, *(u_var1 + 0xc), local_420);
                    if (b_var6 != 0) {}
                    // goto LAB_1030_5fb7;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1030_5c1a(param_1: &mut string, param_2: *mut HFILE16) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;

    u_var1 = write_to_file_1008_7cac(param_2, 9);
    BVar2 = CONCAT11(extraout_AH, u_var1);
    if (BVar2 != 0) {
        BVar2 = write_to_file_1008_7e1c(param_2, param_1, 0x24);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return BVar2;
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub fn read_file_1030_5c52(buf_a: *mut u8, file_a: *mut HFILE16) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;

    u_var1 = read_file_1008_7cfe(file_a);
    BVar2 = CONCAT11(extraout_AH, u_var1);
    if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee(file_a, buf_a, 0x24);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return BVar2;
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub unsafe fn write_to_file_1030_56f6(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let b_var5: bool;
    let mut i32_var6: i32;
    let pu_var7: *mut u8;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    i_var4 = write_to_file_1030_1978(param_1, param_2);
    if (i_var4 != 0) {
        u_var8 = (param_1 >> 0x10);
        i_var4 = param_1;
        local_e = *(i_var4 + 0x10);
        b_var5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_e), 2);
        if (b_var5 != 0) {
            u_var3 = (i_var4 + 0x10);
            local_8 = (u_var3 + 2);
            b_var5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 2);
            if ((b_var5 != 0)
                && (
                    u_var3 = (i_var4 + 0x10),
                    write_to_file_1008_7c2a(param_2, (u_var3 + 4)),
                    b_var5 != 0,
                ))
            {
                u_var3 = (i_var4 + 0x10);
                local_8 = (u_var3 + 0x1a);
                b_var5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 2);
                if (b_var5 != 0) {
                    local_4 = 0;
                    let pu_var1_val = unsafe { *pu_var1 };
                    while (
                        u_var3 = (i_var4 + 0x10),
                        pu_var1 = (u_var3 + 0x1a),
                        pu_var1_val != local_4 && local_4 <= pu_var1_val,
                    ) {
                        i32_var6 = local_4 * 6;
                        u_var3 = (i_var4 + 0x10);
                        u_var2 = (u_var3 + 0x16);
                        write_file_1008_7b4c(param_2, (u_var2 & 0xffff0000 | (u_var2 + i32_var6)));
                        if (i32_var6 == 0) {}
                        // goto LAB_1030_5734;
                        local_4 = local_4 + 1;
                    }
                    pu_var7 = (param_1 & 0xffff0000 | (i_var4 + 0x14));
                    write_file_1008_7b4c(param_2, pu_var7);
                    if (pu_var7 != 0) {
                        local_8 = (i_var4 + 0x1c);
                        b_var5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 2);
                        if (b_var5 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        // LAB_1030_5734:
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn read_from_file_1030_581e(param_1: u32, param_2: *mut HFILE16) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let in_ax: *mut Struct94;
    let BVar3: bool;
    let pu_var4: *mut u8;
    let mut in_i16_5: u16;
    let mut u_var5: u32;
    let in_dx: *mut u16;
    
    let struct_a: *mut Struct199;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_432: u32;
    let mut local_426: u32;
    let mut local_41c: u16;
    let mut local_41a: u16;
    let mut local_410: u16;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: [u8; 1024];
    let mut local_8: u32;
    let mut local_4: u16;

    u_var10 = (param_2 >> 0x10);
    u_var9 = (param_1 >> 0x10);
    pass1_1030_19b4(param_1, param_2);
    if (in_ax != 0x0) {
        if (ctx.__g_Struct94_ptr_1 == 0) {
            _g_Struct94_ptr_1 = in_ax;
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        alloc_mem_1000_1708(0x20, 0, 1, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
        if ((ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0) {
            _g_Struct94_ptr_1 = 0x0;
            struct_a = 0x0;
        } else {
            pass1_1030_84ae(_g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
            struct_a = ctx.dx_reg;
        }
        i32_var6 = param_1;
        (i32_var6 + 0x10) = _g_Struct94_ptr_1;
        (i32_var6 + 0x12) = struct_a;
        BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (BVar3 != 0) {
            u_var5 = (ctx._PTR_LOOP_1050_65e2 + 0x52);
            local_8 = u_var5;
            pass1_1030_4782(u_var5, (u_var5 >> 0x10), 0, 1, local_4);
            (i32_var6 + 0x10) = u_var5;
            (i32_var6 + 0x12) = struct_a;
            BVar3 = read_file_1008_7dee(param_2, CONCAT22(struct_a, (i32_var6 + 0x10) + 2), 2);
            if (BVar3 != 0) {
                pu_var4 = local_408;
                read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_ss, pu_var4));
                if (pu_var4 != 0x0) {
                    u_var2 = (i32_var6 + 0x10);
                    error_check_1000_17ce((u_var2 + 4));
                    pu_var4 = local_408;
                    pass1_fn_1008_60e8(pu_var4);
                    u_var2 = (i32_var6 + 0x10);
                    u_var8 = (u_var2 >> 0x10);
                    i_var7 = u_var2;
                    (i_var7 + 4) = pu_var4;
                    (i_var7 + 6) = struct_a;
                    u_var5 = (i32_var6 + 0x10);
                    BVar3 =
                        read_file_1008_7dee(param_2, (u_var5 & 0xffff0000 | (u_var5 + 0x1a)), 2);
                    if (BVar3 != 0) {
                        u_var2 = (i32_var6 + 0x10);
                        i_var7 = (u_var2 + 0x1a);
                        in_i16_5 = i_var7 * 6;
                        process_struct_1000_179c(in_i16_5, struct_a);
                        _local_410 = CONCAT22(struct_a, in_i16_5);
                        if ((struct_a | in_i16_5) == 0) {
                            u_var2 = (i32_var6 + 0x10);
                            (u_var2 + 0x16) = 0;
                        } else {
                            call_fn_ptr_1000_5586(
                                0x3e38,
                                &ctx.PTR_LOOP_1050_1008,
                                i_var7,
                                6,
                                in_i16_5,
                                struct_a,
                            );
                            u_var2 = (i32_var6 + 0x10);
                            (u_var2 + 0x16) = _local_410;
                        }
                        local_40c = 0;
                        let pu_var1_val = unsafe { *pu_var1 };
                        while (
                            u_var2 = (i32_var6 + 0x10),
                            pu_var1 = (u_var2 + 0x1a),
                            pu_var1_val != local_40c && local_40c <= pu_var1_val,
                        ) {
                            i_var7 = local_40c * 6;
                            u_var2 = (i32_var6 + 0x10);
                            u_var2 = (u_var2 + 0x16);
                            read_file_1008_7bc8(
                                param_2,
                                u_var10,
                                u_var2 + i_var7,
                                (u_var2 >> 0x10),
                            );
                            if (i_var7 == 0) {}
                            // goto LAB_1030_58a7;
                            local_40c = local_40c + 1;
                        }
                        i_var7 = i32_var6 + 0x14;
                        read_file_1008_7bc8(param_2, u_var10, i_var7, u_var9);
                        if ((i_var7 != 0)
                            && (
                                BVar3 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | (i32_var6 + 0x1c)),
                                    2,
                                ),
                                BVar3 != 0,
                            ))
                        {
                            return;
                        }
                    }
                }
            }
        }
        // LAB_1030_58a7:
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn read_from_file_1030_4e70(
    param_1: &mut String,
    param_2: &mut u32,
    param_3: &mut Vec<u8>,
    param_4: &mut String,
) {
    let _file: HFILE16;
    let mut var1: String = String::new();
    // let mut path: String = String::new();
    // let mut u_var2: u16;
    // let mut pu_var3: Vec<u8> = Vec::new();
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    // let mut local_2e: Vec<u8> = Vec::new();
    let mut local_14: u32;

    if param_4 != 0x0 {
        let mut pu_var3 = local_2e;
        let mut u_var2 = 0;
        let mut path = pass1_1030_5164(param_1, param_4);
        var1 = path;
        dos3_call_1000_51aa(&mut var1, &path, pu_var3);
        if var1.is_empty() == false {
            *param_2 = local_14;
            _file = _lopen16(path, 0);
            var1 = _file + 1;
            if (var1 != 0) {
                let param_2_val = unsafe { *param_2 };
                let param_3_val = unsafe { *param_3 };

                alloc_mem_1000_0a48(
                    1,
                    param_2_val,
                    (param_2_val >> 0x10),
                    ctx.__g_Struct94_ptr_1,
                    (ctx.__g_Struct94_ptr_1 >> 0x10),
                );
                *param_3 = var1;
                (param_3 + 2) = ctx.dx_reg;
                if ((ctx.dx_reg | param_3) != 0) {
                    _local_38 = _hread(param_2_val, param_3_val, _file);
                    _lclose16(_file);
                    local_3c = param_3_val;
                    while (_local_38 != 0) {
                        if ((*(*local_3c + 0x608b) & 0x20) == 0) {
                            *local_3c = *local_3c + -0x80;
                        }
                        local_3c = local_3c & 0xffff0000 | (local_3c + 1);
                        _local_38 = _local_38 + -1;
                    }
                    return;
                }
            }
        }
    }
    return;
}

pub fn read_from_file_1030_33f0(param_1: u32, param_2: *mut HFILE16) {
    let local_AX_14: *mut Struct874;
    let b_var1: bool;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0x4;
    b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0x16c);
    if (((((b_var1 != 0)
        && (
            b_var1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x174),
                0xc,
            ),
            b_var1 != 0,
        ))
        && (
            b_var1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x180),
                0xc,
            ),
            b_var1 != 0,
        ))
        && ((
            b_var1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x18c),
                0x18,
            ),
            b_var1 != 0
                && (
                    b_var1 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_14.field_0x1a8),
                        2,
                    ),
                    b_var1 != 0,
                ),
        )))
        && (
            b_var1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x1aa),
                4,
            ),
            b_var1 != 0,
        ))
    {
        if (PTR_LOOP_1050_0312 < 2) {
            return;
        }
        b_var1 = read_file_1008_7dee(
            param_2,
            (param_1 & 0xffff0000 | &local_AX_14.field_0x170),
            4,
        );
        if ((b_var1 != 0)
            && (
                b_var1 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | ZEXT24(local_AX_14 + 1)),
                    2,
                ),
                b_var1 != 0,
            ))
        {
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn write_to_file_1030_32e4(param_1: u32, param_2: *mut HFILE16) {
    let local_AX_14: *mut Struct873;
    let b_var1: bool;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_16: u32;
    let mut local_c: u16;
    let mut local_a: u32;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0x4;
    b_var1 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0x16c);
    if (b_var1 != 0) {
        b_var1 = write_to_file_1008_7e1c(
            param_2,
            (param_1 & 0xffff0000 | &local_AX_14.field_0x174),
            0xc,
        );
        if (b_var1 != 0) {
            b_var1 = write_to_file_1008_7e1c(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x180),
                0xc,
            );
            if (b_var1 != 0) {
                b_var1 = write_to_file_1008_7e1c(
                    param_2,
                    (param_1 & 0xffff0000 | &local_AX_14.field_0x18c),
                    0x18,
                );
                if (b_var1 != 0) {
                    u_var2 = (param_1 >> 0x10);
                    local_c = local_AX_14.field_0x1a8;
                    b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
                    if (b_var1 != 0) {
                        local_16 = local_AX_14.field_0x1aa;
                        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_16), 4);
                        if (b_var1 != 0) {
                            local_a = local_AX_14.field_0x170;
                            b_var1 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_a), 4);
                            if (b_var1 != 0) {
                                local_c = local_AX_14.field_0x1ae;
                                b_var1 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(unaff_ss, &local_c),
                                    2,
                                );
                                if (b_var1 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn write_to_file_1030_2aca(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let b_var4: bool;
    let pu_var5: *mut u8;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut local_18: u32;
    let mut local_c: u16;
    let mut local_6: u16;

    i_var3 = write_to_file_1030_1978(param_1, param_2);
    if (i_var3 == 0) {
        return;
    }
    u_var7 = (param_1 >> 0x10);
    i_var3 = param_1;
    local_c = *(i_var3 + 0x10);
    b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
    if ((b_var4 != 0)
        && (
            u_var2 = (i_var3 + 0x10),
            write_to_file_1008_7c2a(param_2, (u_var2 + 2)),
            b_var4 != 0,
        ))
    {
        u_var1 = (i_var3 + 0x10);
        pu_var5 = (u_var1 & 0xffff0000 | (u_var1 + 6));
        write_file_1008_7b4c(param_2, pu_var5);
        if (pu_var5 != 0) {
            u_var2 = (i_var3 + 0x10);
            local_6 = (u_var2 + 0xc);
            b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
            if (b_var4 != 0) {
                u_var2 = (i_var3 + 0x10);
                local_18 = (u_var2 + 0xe);
                b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_18), 4);
                if ((b_var4 != 0)
                    && (
                        u_var1 = (i_var3 + 0x10),
                        b_var4 = write_to_file_1008_7e1c(
                            param_2,
                            (u_var1 & 0xffff0000 | (u_var1 + 0x12)),
                            0x10,
                        ),
                        b_var4 != 0,
                    ))
                {
                    u_var2 = (i_var3 + 0x10);
                    local_c = (u_var2 + 0x22);
                    b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
                    if ((b_var4 != 0)
                        && ((
                            u_var2 = (i_var3 + 0x10),
                            (u_var2 + 0x22) == 0
                                || (
                                    u_var2 = (i_var3 + 0x10),
                                    u_var8 = (u_var2 >> 0x10),
                                    i32_var6 = u_var2,
                                    b_var4 = write_to_file_1008_7e1c(
                                        param_2,
                                        (i32_var6 + 0x24),
                                        ((i32_var6 + 0x22) * 2),
                                    ),
                                    b_var4 != 0,
                                ),
                        )))
                    {
                        local_c = (i_var3 + 0x14);
                        b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
                        if (b_var4 != 0) {
                            local_c = (i_var3 + 0x16);
                            b_var4 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
                            if (b_var4 != 0) {
                                local_c = (i_var3 + 0x18);
                                b_var4 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(unaff_ss, &local_c),
                                    2,
                                );
                                if (b_var4 != 0) {
                                    local_c = (i_var3 + 0x1a);
                                    b_var4 = write_to_file_1008_7e1c(
                                        param_2,
                                        CONCAT22(unaff_ss, &local_c),
                                        2,
                                    );
                                    if (b_var4 != 0) {
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
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn read_file_1030_2c8a(param_1: u32, param_2: *mut HFILE16) {
    let pu_var1: *mut u16;
    let in_ax: *mut Struct94;
    let BVar2: bool;
    let local_AX_184: *mut u8;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let in_dx: *mut u16;
    let pu_var5: *mut u16;
    
    let struct_a: *mut Struct199;
    let local_bx_91: *mut Struct866;
    let mut i32_var6: i32;
    let mut local_SI__1: u16;
    let mut local_es_210: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pp_var8: *mut pass1_struct_1;
    let mut u_var9: u16;
    let mut local_424: u32;
    let mut local_41a: u16;
    let mut local_418: u16;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    u_var9 = (param_1 >> 0x10);
    pass1_1030_19b4(param_1, param_2);
    if (in_ax == 0x0) {
        return;
    }
    if (ctx.__g_Struct94_ptr_1 == 0) {
        _g_Struct94_ptr_1 = in_ax;
        struct_fn_1000_160a();
        ctx.g_u16_ptr_1050_5f2e = in_dx;
    } else {
    }
    alloc_mem_1000_1708();
    _local_40e = CONCAT22(ctx.g_u16_ptr_1050_5f2e, _g_Struct94_ptr_1);
    pu_var5 = (ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1);
    local_bx_91 = param_1;
    if (pu_var5 == 0x0) {
        local_bx_91.field_0x10 = 0x0;
    } else {
        zero_list_1008_3e38(CONCAT22(
            ctx.g_u16_ptr_1050_5f2e,
            &_g_Struct94_ptr_1.field_0x6,
        ));
        local_bx_91.field_0x10 = _local_40e;
        pu_var5 = ctx.g_u16_ptr_1050_5f2e;
    }
    BVar2 = read_file_1008_7dee(param_2, local_bx_91.field_0x10, 2);
    if (BVar2 != 0) {
        local_AX_184 = local_402;
        read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_ss, local_AX_184));
        if (local_AX_184 != 0x0) {
            pu_var3 = local_402;
            pass1_fn_1008_60e8(pu_var3);
            pu_var1 = local_bx_91.field_0x10;
            local_es_210 = (pu_var1 >> 0x10);
            i32_var6 = pu_var1;
            *(i32_var6 + 2) = pu_var3;
            (i32_var6 + 4) = pu_var5;
            pu_var1 = local_bx_91.field_0x10;
            i32_var6 = pu_var1 + 6;
            read_file_1008_7bc8(param_2, (param_2 >> 0x10), i32_var6, (pu_var1 >> 0x10));
            if ((((i32_var6 != 0)
                && (
                    pu_var1 = local_bx_91.field_0x10,
                    struct_a = ctx.dx_reg,
                    BVar2 =
                        read_file_1008_7dee(param_2, (pu_var1 & 0xffff0000 | (pu_var1 + 0xc)), 2),
                    BVar2 != 0,
                ))
                && (
                    pu_var1 = local_bx_91.field_0x10,
                    BVar2 =
                        read_file_1008_7dee(param_2, (pu_var1 & 0xffff0000 | (pu_var1 + 0xe)), 4),
                    BVar2 != 0,
                ))
                && ((
                    pu_var1 = local_bx_91.field_0x10,
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (pu_var1 & 0xffff0000 | (pu_var1 + 0x12)),
                        0x10,
                    ),
                    BVar2 != 0
                        && (
                            pu_var1 = local_bx_91.field_0x10,
                            BVar2 = read_file_1008_7dee(
                                param_2,
                                (pu_var1 & 0xffff0000 | (pu_var1 + 0x22)),
                                2,
                            ),
                            BVar2 != 0,
                        ),
                )))
            {
                pu_var1 = local_bx_91.field_0x10;
                if ((pu_var1 + 0x22) != 0) {
                    pu_var1 = local_bx_91.field_0x10;
                    u_var7 = (pu_var1 >> 0x10);
                    i32_var6 = pu_var1;
                    u_var4 = (i32_var6 + 0x22) * 2;
                    process_struct_1000_179c(u_var4, struct_a);
                    (i32_var6 + 0x24) = u_var4;
                    (i32_var6 + 0x26) = struct_a;
                    pu_var1 = local_bx_91.field_0x10;
                    u_var7 = (pu_var1 >> 0x10);
                    i32_var6 = pu_var1;
                    BVar2 =
                        read_file_1008_7dee(param_2, *(i32_var6 + 0x24), ((i32_var6 + 0x22) * 2));
                    if (BVar2 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                }
                BVar2 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_bx_91.field_0x14),
                    2,
                );
                if (((BVar2 != 0)
                    && (
                        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_404), 2),
                        BVar2 != 0,
                    ))
                    && ((
                        BVar2 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | &local_bx_91.field_0x18),
                            2,
                        ),
                        BVar2 != 0
                            && (
                                BVar2 =
                                    read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_406), 2),
                                BVar2 != 0,
                            ),
                    )))
                {
                    local_bx_91.field_0x16 = local_404;
                    local_bx_91.field_0x1a = local_406;
                    pp_var8 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(local_SI__1, 0x2f),
                    );
                    pass1_1018_04a4(pp_var8, local_bx_91.field_0x4);
                    pass1_1010_82f8(ctx._g_struct_73_1050_14cc, *local_bx_91.field_0x10);
                    return;
                }
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_227a(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut i_var1: i32;
    let BVar2: bool;

    i_var1 = write_to_file_1030_1978(param_1, param_2);
    if (i_var1 != 0) {
        i_var1 = param_1;
        BVar2 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x10)), 0x106);
        if (BVar2 != 0) {
            BVar2 =
                write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x116)), 0x86);
            if (BVar2 != 0) {
                BVar2 =
                    write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x19c)), 10);
                if (BVar2 != 0) {
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        (param_1 & 0xffff0000 | (i_var1 + 0x1a6)),
                        0x106,
                    );
                    if (BVar2 != 0) {
                        BVar2 = write_to_file_1008_7e1c(
                            param_2,
                            (param_1 & 0xffff0000 | (i_var1 + 0x2ac)),
                            0x106,
                        );
                        if (BVar2 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn read_file_1030_232e(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let mut i_var1: i32;
    let BVar2: bool;

    pass1_1030_19b4(param_1, param_2);
    if (in_ax != 0) {
        i_var1 = param_1;
        BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x10)), 0x106);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x116)), 0x86);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x19c)), 10);
                if (BVar2 != 0) {
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | (i_var1 + 0x1a6)),
                        0x106,
                    );
                    if (BVar2 != 0) {
                        BVar2 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (i_var1 + 0x2ac)),
                            0x106,
                        );
                        if (BVar2 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1030_1a9c(
    ctx: &mut AppContext,
    param_1: *mut Struct771,
    param_2: *mut HFILE16,
) -> bool {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let BVar3: bool;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    i_var2 = write_to_file_1030_1978(param_1, param_2);
    if (i_var2 != 0) {
        u_var4 = (param_1 >> 0x10);
        i_var2 = param_1;
        local_c = *(i_var2 + 0x10);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (BVar3 != 0) {
            if ((i_var2 + 0x10) == 0) {
                return 1;
            }
            pi_var1 = (i_var2 + 0x10);
            let pi_var1_val = unsafe { *pi_var1 };
            BVar3 = write_to_file_1008_7e1c(param_2, (pi_var1 + 2), (pi_var1_val * 2));
            if (BVar3 != 0) {
                return 1;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub unsafe fn pass1_1030_1b18(ctx: &mut AppContext, param_1: u32, param_2: &mut HFILE16) -> u16 {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut in_ax: i32;
    let mut u_var4: u16;
    let b_var5: bool;
    let mut u_var6: i32;
    let in_dx: *mut u16;
    let struct_a: *mut Struct199;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_18: u16;

    pass1_1030_19b4(param_1, param_2);
    if (in_ax != 0) {
        if (ctx.__g_Struct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        u_var4 = alloc_mem_1000_1708();
        u_var9 = (param_1 >> 0x10);
        i_var7 = param_1;
        (i_var7 + 0x10) = u_var4;
        (i_var7 + 0x12) = ctx.g_u16_ptr_1050_5f2e;
        struct_a = (i_var7 + 0x12);
        b_var5 = read_file_1008_7dee(param_2, CONCAT22(struct_a, (i_var7 + 0x10)), 2);
        if (b_var5 != 0) {
            pi_var1 = (i_var7 + 0x10);
            let pi_var1_val = unsafe { *pi_var1 };
            if (pi_var1_val == 0) {
                return 1;
            }
            u_var3 = pi_var1_val * 2;
            u_var6 = u_var3;
            process_struct_1000_179c(u_var3, struct_a);
            u_var2 = (i_var7 + 0x10);
            u_var4 = (u_var2 >> 0x10);
            i_var8 = u_var2;
            (i_var8 + 2) = u_var6;
            (i_var8 + 4) = struct_a;
            u_var2 = (i_var7 + 0x10);
            b_var5 = read_file_1008_7dee(param_2, *(u_var2 + 2), u_var3);
            if (b_var5 != 0) {
                return 1;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return 0;
}

pub fn write_to_file_1030_1978(param_1: *mut Struct771, param_2: *mut HFILE16) -> i32 {
    let mut i_var1: i32;

    i_var1 = write_to_file_1030_16d6(param_1, param_2);
    if (i_var1 != 0) {
        i_var1 = write_to_file_1008_7954(param_2, (param_1 + 0xc));
        if (i_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return i_var1;
        }
        i_var1 = 1;
    }
    return i_var1;
}

pub unsafe fn pass1_1030_19b4(param_1: u32, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut u_var2: u32;

    u_var1 = pass1_1030_1730(param_1, param_2);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        u_var2 = param_1 & 0xffff0000 | (param_1 + 0xc);
        read_file_1008_76e4(param_2, u_var2);
        if (u_var2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub fn write_to_file_1030_16d6(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let b_var1: bool;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_8: u32;

    u_var2 = (param_1 >> 0x10);
    local_10 = (param_1 + 4);
    b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
    if (b_var1 != 0) {
        local_8 = (param_1 + 8);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_8), 4);
        if (b_var1 != 0) {
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn pass1_1030_1730(param_1: *mut Struct933, param_2: *mut HFILE16) -> u8 {
    let b_var1: bool;

    b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 4)), 4);
    if (b_var1 != 0) {
        b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 8)), 4);
        if (b_var1 != 0) {
            return 0x1;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return '\0';
}

pub unsafe fn pass1_1028_e628(param_1: u32, param_2: u16, param_3: u16, param_3_00: i32, param_5: i32) {
    let pc_var1: *mut libc::c_char;
    let pi_var2: *mut i32;
    let mut u_var3: u32;
    let mut cVar4: u8;
    let pu_var5: *mut u32;
    let mut u_var6: u32;
    let pa_var7: *mut Struct844;
    let mut u_var8: u32;
    let lVar9: u32;
    let pp_var10: fn();
    let pu_var11: *mut u16;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let paVar18: *mut Struct493;
    let pu_var19: *mut u16;
    let mut u_var20: u16;
    let BVar21: bool;
    let mut u_var22: i32;
    let pu_var23: *mut u8;
    let mut u_var24: u32;
    
    let mut i_var25: i32;
    // let ctx.dx_reg: *mut Struct199;
    // let ctx.dx_reg: *mut Struct199;
    // let ctx.dx_reg: *mut Struct199;
    
    // let mut extraout_dx_04: u16;
    // let mut extraout_dx_05: u16;
    // let mut extraout_dx_06: u16;
    // let mut extraout_dx_07: u16;
    // let extraout_dx_08: *mut Struct199;
    // let mut extraout_dx_09: u16;
    // let mut extraout_dx_10: u16;
    let paVar26: *mut Struct199;
    let mut u_var27: u16;
    // let mut unaff_si: u16;
    // let unaff_DI: *mut u8;
    // let mut unaff_es: u16;
    let mut u_var28: u16;
    // let mut unaff_cs: u16;
    let mut u_var29: u16;
    let mut uVar30: u16;
    // let mut unaff_ss: u16;
    let mut bVar31: bool;
    let mut bVar32: bool;
    let ppVar33: *mut pass1_struct_1;
    let pu_var34: *mut u16;
    let paVar35: *mut Struct763;
    let mut local_154: u16;
    let mut local_152: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut uStack80: u16;
    let mut uStack78: u16;
    let uStack76: u8;
    let uStack75: u8;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut uVar36: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut uStack52: u16;
    let mut uStack50: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar36 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    local_42._0_1_ = SUB21(&local_6, 0);
    local_42._1_1_ = (&local_6 >> 8);
    local_46._0_1_ = param_2;
    u_var12 = local_46;
    local_46._1_1_ = (param_2 >> 8);
    uVar15 = local_46._1_1_;
    local_44 = param_3;
    local_4a._0_1_ = 0x3f;
    local_4a._1_1_ = 0xe6;
    local_48 = unaff_cs;
    local_3a = unaff_si;
    local_38._0_2_ = unaff_DI;
    BVar21 = read_file_1008_7dee(CONCAT22(param_3, param_2), CONCAT22(unaff_ss, &local_6), 4);
    u_var29 = local_44;
    uVar16 = local_46._1_1_;
    u_var13 = local_46;
    if (BVar21 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    local_a = 0;
    if (((param_3_00 == 0) && ((param_5 - 0x100) == '\0'))
        && (
            paVar26 = (param_5 - 0x100 >> 7),
            paVar26 < (&PTR_LOOP_1050_000e + 1),
        ))
    {
        uVar30 = &PTR_LOOP_1050_1028;
        local_46._0_1_ = param_1;
        u_var14 = local_46;
        local_46._1_1_ = (param_1 >> 8);
        uVar17 = local_46._1_1_;
        local_44 = (param_1 >> 0x10);
        u_var28 = local_44;
        u_var27 = param_1;
        match paVar26 {
            0x0 => {
                pa_var7 = (u_var27 + 0xe);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x73;
                local_46._1_1_ = 0xe6;
                pass1_1030_145a(pa_var7, local_6);
                local_1e._2_2_ = 0;
                local_1a = 0;
                while (CONCAT22(local_1a, local_1e._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe6d5;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x14, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0xeb;
                        local_42._1_1_ = 0xe6;
                        u_var22 = local_20;
                        pass1_1030_5d0a();
                        local_40 = extraout_dx_04;
                    }
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x97;
                    local_46._1_1_ = 0xe6;
                    pp_var10 = (CONCAT22(local_40, u_var22) + 0x10);
                    local_44 = u_var29;
                    local_18 = u_var22;
                    local_16 = local_40;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    u_var24 = (local_18 + 4);
                    u_var22 = (local_18 + 6);
                    local_e = u_var24;
                    local_c = (u_var24 >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_40 = local_16;
                    local_42._0_1_ = local_18;
                    local_42._1_1_ = (local_18 >> 8);
                    u_var8 = (u_var27 + 0xe);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_4a._0_1_ = 0xc0;
                    local_4a._1_1_ = 0xe6;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        local_18,
                        local_16,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(local_1a, local_1e._2_2_) + 1;
                    local_1e._2_2_ = lVar9;
                    local_1a = (lVar9 >> 0x10);
                }
            }
            0x1 => {
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            0x2 => {
                pa_var7 = (u_var27 + 0x12);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf;
                local_46._1_1_ = 0xe7;
                pass1_1030_145a(pa_var7, local_6);
                local_2a._2_2_ = 0;
                local_26 = 0;
                while (CONCAT22(local_26, local_2a._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe771;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x1c, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0x87;
                        local_42._1_1_ = 0xe7;
                        u_var22 = local_20;
                        pass1_1030_2958((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_dx_05;
                    }
                    _local_24 = CONCAT22(local_40, u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x33;
                    local_46._1_1_ = 0xe7;
                    pp_var10 = (*_local_24 + 0x10);
                    local_44 = u_var29;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    local_40 = (_local_24 >> 0x10);
                    uVar30 = _local_24;
                    u_var24 = (uVar30 + 4);
                    u_var22 = (uVar30 + 6);
                    local_e = u_var24;
                    local_c = (u_var24 >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_42._0_1_ = SUB41(_local_24, 0);
                    local_42._1_1_ = (_local_24 >> 8);
                    u_var8 = (u_var27 + 0x12);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_4a._0_1_ = 0x5c;
                    local_4a._1_1_ = 0xe7;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        uVar30,
                        local_40,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(local_26, local_2a._2_2_) + 1;
                    local_2a._2_2_ = lVar9;
                    local_26 = (lVar9 >> 0x10);
                }
            }
            0x3 => {
                local_42._0_1_ = 0;
                local_42._1_1_ = 0;
                local_40 = 0x500;
                u_var22 = u_var27 + 0x114;
                local_46._0_1_ = u_var22;
                u_var12 = local_46;
                local_46._1_1_ = (u_var22 >> 8);
                u_var13 = local_46._1_1_;
                local_48 = 0;
                uStack76 = ctx._PTR_LOOP_1050_65e2;
                uStack75 = (ctx._PTR_LOOP_1050_65e2 >> 8);
                local_4a._0_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                local_4a._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x18);
                uStack78 = SUB42(&PTR_LOOP_1050_1028, 0);
                uStack80 = 0x970b;
                local_16 = u_var22;
                pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
                local_46._0_1_ = local_16;
                local_46._1_1_ = (local_16 >> 8);
                local_4a._0_1_ = _PTR_LOOP_1050_5740;
                local_4a._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                local_48 = (_PTR_LOOP_1050_5740 >> 0x10);
                uStack76 = 0x28;
                uStack75 = 0x10;
                u_var29 = 0x1030;
                uStack78 = 0x9728;
                local_44 = paVar26;
                local_42._0_1_ = u_var12;
                local_42._1_1_ = u_var13;
                local_40 = u_var28;
                local_14 = paVar26;
                pass1_1030_61fe(
                    _PTR_LOOP_1050_5740,
                    CONCAT22(paVar26, local_16),
                    param_1 & 0xffff0000 | u_var22,
                    (u_var27 + 0x108),
                );
                if (((u_var27 + 0x11a) == 10) || ((u_var27 + 0x11a) == 0x37)) {
                    if ((u_var27 + 0x11a) == 0x37) {
                        u_var3 = (u_var27 + 0x11e);
                        paVar26 = (u_var27 + 0x120);
                        local_38._0_2_ = u_var3;
                        local_38._2_2_ = (u_var3 >> 0x10);
                        u_var3 = (u_var27 + 0x10c);
                        local_2a._0_2_ = u_var3;
                        local_2a._2_2_ = (u_var3 >> 0x10);
                        (local_38 + 0x20) = u_var3;
                    }
                    local_42._0_1_ = 0;
                    local_42._1_1_ = 0;
                    local_40 = 0x400;
                    i_var25 = u_var27 + 0x114;
                    local_46._0_1_ = i_var25;
                    local_46._1_1_ = (i_var25 >> 8);
                    local_48 = 0;
                    uStack76 = ctx._PTR_LOOP_1050_65e2;
                    uStack75 = (ctx._PTR_LOOP_1050_65e2 >> 8);
                    local_4a._0_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                    local_4a._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x18);
                    uStack78 = 0x1030;
                    uStack80 = 0x9788;
                    local_44 = u_var28;
                    pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
                    (u_var27 + 0x10c) = i_var25;
                    (u_var27 + 0x10e) = paVar26;
                    u_var20 = (u_var27 + 0x10c);
                    local_42._0_1_ = u_var20;
                    local_42._1_1_ = (u_var20 >> 8);
                    local_46._0_1_ = local_6;
                    local_46._1_1_ = (local_6 >> 8);
                    local_44 = (local_6 >> 0x10);
                    local_48 = 0x1030;
                    u_var29 = 0x1018;
                    local_4a._0_1_ = 0xaa;
                    local_4a._1_1_ = 0x97;
                    local_40 = paVar26;
                    pass1_1018_0196(local_6, CONCAT22(paVar26, u_var20), (u_var27 + 0x108));
                    paVar26 = ctx.dx_reg;
                    if ((u_var27 + 0x11a) == 10) {
                        local_42._0_1_ = local_6;
                        local_42._1_1_ = (local_6 >> 8);
                        local_40 = (local_6 >> 0x10);
                        local_44 = 0x1018;
                        u_var29 = 0x1010;
                        local_46._0_1_ = 0xc4;
                        local_46._1_1_ = 0x97;
                        pass1_1010_ed22(local_6, (u_var27 + 0x10c));
                        paVar26 = ctx.dx_reg;
                    }
                }
                u_var3 = (u_var27 + 0x10c);
                local_42._0_1_ = ctx._PTR_LOOP_1050_65e2;
                local_42._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 8);
                local_40 = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                local_46._0_1_ = 0xd7;
                local_46._1_1_ = 0x97;
                local_44 = u_var29;
                paVar18 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var3, (u_var3 >> 0x10));
                (u_var27 + 0x110) = paVar18;
                (u_var27 + 0x112) = paVar26;
                if ((paVar26 | (u_var27 + 0x110)) != 0) {
                    local_40 = local_16;
                    local_42._0_1_ = 0;
                    local_42._1_1_ = 0;
                    u_var20 = (u_var27 + 0x110);
                    local_46._0_1_ = u_var20;
                    local_46._1_1_ = (u_var20 >> 8);
                    local_4a._0_1_ = 7;
                    local_4a._1_1_ = 0x98;
                    pp_var10 = ((u_var27 + 0x110) + 8);
                    local_48 = u_var29;
                    local_44 = paVar26;
                    (**pp_var10)();
                    paVar26 = ctx.dx_reg;
                }
                local_42._0_1_ = ctx._PTR_LOOP_1050_65e2;
                local_42._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 8);
                local_40 = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                local_46._0_1_ = 0x15;
                local_46._1_1_ = 0x98;
                local_44 = u_var29;
                local_1a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_16, local_14);
                local_42._0_1_ = local_1a;
                local_42._1_1_ = (local_1a >> 8);
                local_46._0_1_ = 0x2b;
                local_46._1_1_ = 0x98;
                local_44 = u_var29;
                local_40 = paVar26;
                local_18 = paVar26;
                pass1_1030_73ee(CONCAT22(paVar26, local_1a), (u_var27 + 0x10c));
                local_42._0_1_ = ctx._PTR_LOOP_1050_06e0;
                local_42._1_1_ = (ctx._PTR_LOOP_1050_06e0 >> 8);
                local_40 = (ctx._PTR_LOOP_1050_06e0 >> 0x10);
                local_44 = 0x1030;
                u_var29 = &ctx.PTR_LOOP_1050_1008;
                local_46._0_1_ = 0x3f;
                local_46._1_1_ = 0x98;
                local_20 = ctx.dx_reg;
                BVar21 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var27 + 0x11a), 0x31);
                if ((BVar21 == 0) && ((u_var27 + 0x122) == 0)) {
                    uVar36 = (*(local_1a + 0xc) >> 0x10);
                    local_3a = (local_1a + 0x10);
                    local_38._0_2_ = &stack0xffc2;
                    if (local_3a < 1) {
                        u_var20 = 5;
                    } else {
                        u_var20 = 6;
                    }
                    (local_1a + 0x14) = u_var20;
                    local_20 = local_18;
                }
                u_var3 = (local_1a + 0x16);
                local_1e._0_2_ = u_var3;
                local_1e._2_2_ = (u_var3 >> 0x10);
                u_var8 = &PTR_LOOP_1050_65e2;
                local_42._0_1_ = u_var8;
                local_42._1_1_ = (u_var8 >> 8);
                local_40 = (u_var8 >> 0x10);
                local_44 = &ctx.PTR_LOOP_1050_1008;
                local_46._0_1_ = 0x9b;
                local_46._1_1_ = 0x98;
                paVar18 = pass1_1028_e1ec(u_var8, local_1e, local_1e._2_2_);
                _local_24 = CONCAT22(paVar18, local_24);
                if (CONCAT22(local_1e._2_2_, local_1e) != 0) {
                    local_42._0_1_ = SUB21(&local_14c, 0);
                    local_42._1_1_ = (&local_14c >> 8);
                    local_44 = &ctx.PTR_LOOP_1050_1008;
                    local_46._0_1_ = 0xb7;
                    local_46._1_1_ = 0x98;
                    pass1_1030_e4fa(
                        CONCAT22(unaff_ss, &local_14c),
                        CONCAT22(local_1e._2_2_, local_1e),
                    );
                    pu_var5 = &g_bool_1050_5748;
                    local_42._0_1_ = SUB41(pu_var5, 0);
                    local_42._1_1_ = (pu_var5 >> 8);
                    local_40 = (pu_var5 >> 0x10);
                    local_44 = 0x1030;
                    u_var29 = 0x1030;
                    local_46._0_1_ = 199;
                    local_46._1_1_ = 0x98;
                    pass1_1030_835a(pu_var5, CONCAT22(unaff_ss, &local_14c));
                    local_14c = ctx.s_1_1050_389a;
                    local_14a = &ctx.PTR_LOOP_1050_1008;
                }
                u_var3 = (u_var27 + 0x11e);
                local_42._0_1_ = u_var3;
                local_42._1_1_ = (u_var3 >> 8);
                local_40 = (u_var3 >> 0x10);
                local_46._0_1_ = 0xec;
                local_46._1_1_ = 0x98;
                pp_var10 = ((u_var27 + 0x11e) + 4);
                local_44 = u_var29;
                (**pp_var10)();
                u_var3 = (u_var27 + 0x11e);
                u_var8 = (u_var3 + 4);
                local_46._0_1_ = u_var8;
                local_46._1_1_ = (u_var8 >> 8);
                local_44 = (u_var8 >> 0x10);
                local_4a._0_1_ = local_1a;
                local_4a._1_1_ = (local_1a >> 8);
                local_48 = local_18;
                uStack76 = u_var29;
                uStack75 = (u_var29 >> 8);
                uStack78 = 0x9902;
                pass1_1030_7e5a(CONCAT22(local_18, local_1a), u_var8);
                return;
            }
            0x4 => {
                pa_var7 = (u_var27 + 0x16);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xa7;
                local_46._1_1_ = 0xe7;
                pass1_1030_145a(pa_var7, local_6);
                local_2a._2_2_ = 0;
                local_26 = 0;
                while (CONCAT22(local_26, local_2a._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe828;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x1e, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0x40;
                        local_42._1_1_ = 0xe8;
                        u_var22 = local_20;
                        pass1_1030_560e((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_dx_06;
                    }
                    _local_24 = CONCAT22(local_40, u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0xcb;
                    local_46._1_1_ = 0xe7;
                    pp_var10 = (*_local_24 + 0x10);
                    local_44 = u_var29;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    uVar36 = (_local_24 >> 0x10);
                    u_var8 = (_local_24 + 4);
                    local_e = u_var8;
                    local_c = (u_var8 >> 0x10);
                    u_var6 = (_local_24 + 0x10);
                    local_1e._2_2_ = u_var6;
                    local_1a = (u_var6 >> 0x10);
                    local_42._0_1_ = u_var6;
                    local_42._1_1_ = (u_var6 >> 8);
                    local_44 = 0;
                    local_48 = _PTR_LOOP_1050_5740;
                    local_46._0_1_ = (_PTR_LOOP_1050_5740 >> 0x10);
                    local_46._1_1_ = (_PTR_LOOP_1050_5740 >> 0x18);
                    local_4a._0_1_ = u_var29;
                    local_4a._1_1_ = (u_var29 >> 8);
                    uStack76 = 0xf8;
                    uStack75 = 0xe7;
                    local_40 = local_1a;
                    pass1_1030_6222(_PTR_LOOP_1050_5740, 0, u_var6, u_var8);
                    paVar26 = (local_c & 0xff);
                    local_42._0_1_ = SUB41(_local_24, 0);
                    local_42._1_1_ = (_local_24 >> 8);
                    local_40 = (_local_24 >> 0x10);
                    u_var8 = (u_var27 + 0x16);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0x13;
                    local_4a._1_1_ = 0xe8;
                    pass1_1030_14b4(
                        u_var8,
                        _local_24,
                        local_40,
                        CONCAT22(local_c, local_e) & 0xffffff,
                    );
                    lVar9 = CONCAT22(local_26, local_2a._2_2_) + 1;
                    local_2a._2_2_ = lVar9;
                    local_26 = (lVar9 >> 0x10);
                }
            }
            0x5 => {
                (paVar26).field_0x0 = "\x02";
                paVar26.field_0x2 = &PTR_LOOP_1050_1028;
                return;
            }
            0x6 => {
                pa_var7 = (u_var27 + 0x1a);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x61;
                local_46._1_1_ = 0xe8;
                pass1_1030_145a(pa_var7, local_6);
                local_30 = 0;
                while (local_30 < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe8c4;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x21e, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = &PTR_LOOP_1050_1038;
                        local_42._0_1_ = 0xda;
                        local_42._1_1_ = 0xe8;
                        u_var22 = local_20;
                        pass1_1038_30aa();
                        local_40 = extraout_dx_07;
                    }
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x85;
                    local_46._1_1_ = 0xe8;
                    pp_var10 = (CONCAT22(local_40, u_var22) + 0x10);
                    local_44 = u_var29;
                    local_2c = u_var22;
                    local_2a._0_2_ = local_40;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    u_var24 = (local_2c + 4);
                    u_var22 = (local_2c + 6);
                    local_e = u_var24;
                    local_c = (u_var24 >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_40 = local_2a;
                    local_42._0_1_ = local_2c;
                    local_42._1_1_ = (local_2c >> 8);
                    u_var8 = (u_var27 + 0x1a);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_4a._0_1_ = 0xae;
                    local_4a._1_1_ = 0xe8;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        local_2c,
                        local_2a,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    local_30 = local_30 + 1;
                }
            }
            // default:
            _ => {
                pa_var7 = (u_var27 + 0x1e);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf9;
                local_46._1_1_ = 0xe8;
                pass1_1030_145a(pa_var7, local_6);
                local_42._0_1_ = _PTR_LOOP_1050_5740;
                local_42._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                local_40 = (_PTR_LOOP_1050_5740 >> 0x10);
                local_44 = 0x1030;
                local_46._0_1_ = 7;
                local_46._1_1_ = 0xe9;
                pass1_1030_66de(_PTR_LOOP_1050_5740, local_6);
                local_30 = 0;
                while (true) {
                    if (local_6 <= local_30) {
                        local_40 = 0x1030;
                        local_42._0_1_ = 0xd5;
                        local_42._1_1_ = 0xe9;
                        ret_1030_154c();
                        local_40 = 0x1030;
                        local_42._0_1_ = 0xdf;
                        local_42._1_1_ = 0xe9;
                        pass1_1030_6740(_PTR_LOOP_1050_5740);
                        return;
                    }
                    local_14 = _PTR_LOOP_1050_5744;
                    local_12 = (_PTR_LOOP_1050_5744 >> 0x10);
                    local_40 = 0x1030;
                    u_var29 = 0x1000;
                    local_42._0_1_ = 0xaf;
                    local_42._1_1_ = 0xe9;
                    pu_var23 = _PTR_LOOP_1050_5744;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_5744);
                    local_20 = pu_var23;
                    local_1e._0_2_ = extraout_dx_08;
                    if ((extraout_dx_08 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0xc4;
                        local_42._1_1_ = 0xe9;
                        u_var22 = local_20;
                        pass1_1030_67cc((pu_var23 & 0xffff | ZEXT24(extraout_dx_08) << 0x10));
                        local_40 = extraout_dx_09;
                    }
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x2d;
                    local_46._1_1_ = 0xe9;
                    pp_var10 = (CONCAT22(local_40, u_var22) + 0x10);
                    local_44 = u_var29;
                    local_2c = u_var22;
                    local_2a._0_2_ = local_40;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        break;
                    }
                    u_var8 = (local_2c + 4);
                    local_e = u_var8;
                    local_c = (u_var8 >> 0x10);
                    u_var24 = (local_2c + 8);
                    local_2a._2_2_ = u_var24;
                    local_26 = (u_var24 >> 0x10);
                    u_var3 = (local_2c + 0xc);
                    local_38._2_2_ = u_var3;
                    uStack52 = (u_var3 >> 0x10);
                    uStack50 = (local_2c + 0x10);
                    u_var22 = &local_38 + 2;
                    _local_24 = (_local_24 & 0xffff0000 | u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = local_c;
                    local_4a._0_1_ = _PTR_LOOP_1050_5740;
                    local_4a._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                    local_48 = (_PTR_LOOP_1050_5740 >> 0x10);
                    uStack76 = u_var29;
                    uStack75 = (u_var29 >> 8);
                    uStack78 = 0xe977;
                    pass1_1030_671c(
                        _PTR_LOOP_1050_5740,
                        u_var8,
                        CONCAT22(unaff_ss, u_var22),
                        u_var24,
                    );
                    local_42._0_1_ = local_2c;
                    local_42._1_1_ = (local_2c >> 8);
                    local_40 = local_2a;
                    u_var8 = (u_var27 + 0x1e);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0x92;
                    local_4a._1_1_ = 0xe9;
                    pass1_1030_14b4(
                        u_var8,
                        local_2c,
                        local_2a,
                        CONCAT22(local_c, local_e) & 0xffffff,
                    );
                    local_30 = local_30 + 1;
                }
                return;
            }
            0x9 => {
                local_6 = local_6 & 0xffff;
                local_42._0_1_ = param_3;
                local_42._1_1_ = (param_3 >> 8);
                local_40 = param_3_00;
                local_44 = param_2;
                local_46._0_1_ = (param_1 >> 0x10);
                local_46._1_1_ = (param_1 >> 0x18);
                local_c = *(u_var27 + 0x2e);
                local_a = *(u_var27 + 0x30);
                local_4a._0_1_ = 0x28;
                local_4a._1_1_ = 0x10;
                uStack76 = 0x2d;
                uStack75 = 0xe3;
                local_48 = u_var27;
                (*local_c)();
                return;
            }
            0xa => {
                pa_var7 = (u_var27 + 0x22);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf3;
                local_46._1_1_ = 0xe9;
                pass1_1030_145a(pa_var7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    local_40 = 0xea55;
                    u_var24 = local_6;
                    process_struct_1000_179c(0xe, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        i_var25 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        local_42._0_1_ = 0x6b;
                        local_42._1_1_ = 0xea;
                        pu_var34 = pass1_1028_b204((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = (pu_var34 >> 0x10);
                        i_var25 = pu_var34;
                    }
                    local_30 = CONCAT22(local_40, i_var25);
                    local_42._0_1_ = i_var25;
                    local_42._1_1_ = (i_var25 >> 8);
                    local_44 = 0x1000;
                    local_46._0_1_ = 0x17;
                    local_46._1_1_ = 0xea;
                    pp_var10 = (local_30 + 0x10);
                    (**pp_var10)();
                    if (i_var25 == 0) {
                        return;
                    }
                    local_40 = (local_30 >> 0x10);
                    u_var29 = local_30;
                    u_var24 = (u_var29 + 4);
                    u_var22 = (u_var29 + 6);
                    local_e = u_var24;
                    local_c = (u_var24 >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_42._0_1_ = local_30;
                    local_42._1_1_ = (local_30 >> 8);
                    u_var8 = (u_var27 + 0x22);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_48 = 0x1000;
                    local_4a._0_1_ = 0x40;
                    local_4a._1_1_ = 0xea;
                    pass1_1030_14b4(
                        u_var8,
                        u_var29,
                        local_40,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                    uStack52 = (lVar9 >> 0x10);
                }
            }
            0xb => {
                if (paVar26 < (&PTR_LOOP_1050_000e + 1)) {
                    pc_var1 = (unaff_si + 0x23);
                    pv_var1_val = unsafe { *pc_var1 };
                    cVar4 = pv_var1_val;
                    unsafe { *pc_var1 = *pc_var1 << 6 };
                    uVar36 = 0x2b;
                    pi_var2 = (&paVar26.field_0x0 + unaff_si);
                    let pi_var_2_val = unsafe { pi_var2 };
                    unsafe {
                        *pi_var2 = *pi_var2 + (-0x6600 - ((cVar4 << 5) < '\0'));
                    }
                } else {
                    uVar36 = 0x7af0;
                    local_46._0_1_ = u_var13;
                    local_46._1_1_ = uVar16;
                    local_44 = u_var29;
                    pass1_1028_780c(local_3a, local_38, CONCAT22(uStack52, local_38._2_2_));
                    if (param_3_00 == 0) {}
                    // goto code_r0x10287b17;
                }
                local_42._0_1_ = 0;
                local_42._1_1_ = 4;
                local_40 = 0x1d;
                local_46._0_1_ = 0x2b;
                local_46._1_1_ = 0;
                local_44 = 1;
                local_4a._0_1_ = SUB41(ctx._g_Struct372_1050_0ed0, 0);
                local_4a._1_1_ = (ctx._g_Struct372_1050_0ed0 >> 8);
                local_48 = (ctx._g_Struct372_1050_0ed0 >> 0x10);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uStack78 = 0x7b0a;
                ppVar33 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
                local_44 = (ppVar33 >> 0x10);
                local_48 = ppVar33;
                local_46._0_1_ = (ppVar33 >> 0x10);
                local_46._1_1_ = (ppVar33 >> 0x18);
                local_4a._0_1_ = 0x10;
                local_4a._1_1_ = 0x10;
                uVar30 = 0x1010;
                uStack76 = 0x17;
                uStack75 = 0x7b;
                pass1_1010_043a(
                    ppVar33,
                    CONCAT13(local_42._1_1_, CONCAT12(local_42, local_44)),
                    local_40,
                );
                paVar26 = ctx.dx_reg;
                // code_r0x10287b17:
                local_42._0_1_ = 2;
                local_42._1_1_ = 0;
                local_40 = 0x400;
                local_46._0_1_ = ctx._PTR_LOOP_1050_65e2;
                local_46._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 8);
                local_44 = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                local_4a._0_1_ = 0x27;
                local_4a._1_1_ = 0x7b;
                local_48 = uVar30;
                paVar18 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
                local_42._0_1_ = SUB21(paVar18, 0);
                local_42._1_1_ = (paVar18 >> 8);
                local_4a._0_1_ = 0x38;
                local_4a._1_1_ = 0x7b;
                local_48 = uVar30;
                local_46._0_1_ = u_var14;
                local_46._1_1_ = uVar17;
                local_44 = u_var28;
                local_40 = paVar26;
                pass1_1028_780c(u_var27, u_var28, CONCAT22(paVar26, paVar18));
                local_40 = 2;
                local_44 = ctx._g_Struct372_1050_0ed0;
                local_42._0_1_ = (ctx._g_Struct372_1050_0ed0 >> 0x10);
                local_42._1_1_ = (ctx._g_Struct372_1050_0ed0 >> 0x18);
                local_46._0_1_ = uVar30;
                local_46._1_1_ = (uVar30 >> 8);
                local_48 = 0x7b44;
                local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(uVar36, 2));
                local_c = u16_1050_13ae;
                if (2 < u16_1050_13ae) {
                    local_40 = 0x2f;
                    local_44 = ctx._g_Struct372_1050_0ed0;
                    local_42._0_1_ = (ctx._g_Struct372_1050_0ed0 >> 0x10);
                    local_42._1_1_ = (ctx._g_Struct372_1050_0ed0 >> 0x18);
                    local_46._0_1_ = 0x10;
                    local_46._1_1_ = 0x10;
                    u_var29 = 0x1010;
                    local_48 = 0x7b63;
                    ppVar33 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(uVar36, 0x2f),
                    );
                    local_48 = (ppVar33 >> 0x10);
                    local_4a._0_1_ = SUB41(ppVar33, 0);
                    local_4a._1_1_ = (ppVar33 >> 8);
                    local_46._0_1_ = 1;
                    local_46._1_1_ = 0;
                    while (CONCAT11(local_46._1_1_, local_46) < 9) {
                        if ((CONCAT11(local_4a._1_1_, local_4a)
                            + 0x34
                            + CONCAT11(local_46._1_1_, local_46) * 4)
                            == local_6)
                        {
                            local_40 = 100;
                            paVar26 = (&ctx.PTR_LOOP_1050_0000 + 1);
                            local_30 = CONCAT22(local_30._2_2_, 1);
                            local_42._0_1_ = 1;
                            local_42._1_1_ = 0;
                            u_var28 = &ctx.PTR_LOOP_1050_1008;
                            local_46._0_1_ = 0xd7;
                            local_46._1_1_ = 0x7b;
                            local_44 = u_var29;
                            pass1_fn_1008_612e();
                            pu_var19 = (CONCAT11(local_46._1_1_, local_46) - 7);
                            if (pu_var19 == 0x0) {
                                bVar32 = SBORROW2(paVar26, 0x32);
                                pu_var11 = &paVar26[-5].field_0xa;
                                bVar31 = paVar26 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);
                                // LAB_1028_7b74:
                                if ((!bVar31 && bVar32) == (pu_var11 < 0)) {
                                    local_30 = local_30 & 0xffff0000;
                                }
                            } else {
                                pu_var19 = (CONCAT11(local_46._1_1_, local_46) - 8);
                                if (pu_var19 == 0x0) {
                                    bVar32 = SBORROW2(paVar26, 0x19);
                                    pu_var11 = (&paVar26[-3].field_0xa + 1);
                                    bVar31 = pu_var11 == 0x0;
                                    // goto LAB_1028_7b74;
                                }
                            }
                            local_1e._0_2_ = paVar26;
                            if (local_30 != 0) {
                                local_40 = CONCAT11(local_46._1_1_, local_46);
                                local_44 = &local_154;
                                local_42._0_1_ = unaff_ss;
                                local_42._1_1_ = (unaff_ss >> 8);
                                local_46._0_1_ = 8;
                                local_46._1_1_ = 0x10;
                                local_48 = 0x7bfc;
                                pass1_1028_90e6(
                                    CONCAT13(local_42._1_1_, CONCAT12(local_42, local_44)),
                                    local_40,
                                );
                                pu_var19 = &local_154;
                                local_42._0_1_ = SUB21(pu_var19, 0);
                                local_42._1_1_ = (pu_var19 >> 8);
                                local_46._0_1_ = SUB41(ctx._g_bool_1050_5748, 0);
                                local_46._1_1_ = (ctx._g_bool_1050_5748 >> 8);
                                local_44 = (ctx._g_bool_1050_5748 >> 0x10);
                                local_48 = &ctx.PTR_LOOP_1050_1008;
                                u_var28 = 0x1030;
                                local_4a._0_1_ = 0xc;
                                local_4a._1_1_ = 0x7c;
                                pass1_1030_835a(
                                    ctx._g_bool_1050_5748,
                                    CONCAT22(unaff_ss, pu_var19),
                                );
                                local_154 = ctx.s_1_1050_389a;
                                local_152 = &ctx.PTR_LOOP_1050_1008;
                            }
                            local_42._0_1_ = 0;
                            local_42._1_1_ = 0;
                            local_40 = 10;
                            u_var29 = &ctx.PTR_LOOP_1050_1008;
                            local_46._0_1_ = 0x23;
                            local_46._1_1_ = 0x7c;
                            local_44 = u_var28;
                            pass1_fn_1008_612e();
                            local_18 = pu_var19;
                            if (CONCAT11(local_46._1_1_, local_46) == 7) {
                                local_40 = 7;
                                pu_var19 = pu_var19 + 0x37;
                                i_var25 = pu_var19 >> 0xf;
                            } else {
                                if (CONCAT11(local_46._1_1_, local_46) != 8) {}
                                // goto LAB_1028_7ba0;
                                local_40 = 8;
                                pu_var19 = pu_var19 + 0x32;
                                i_var25 = (pu_var19 >> 0xf) + (0xff9b < pu_var19);
                            }
                            local_44 = CONCAT11(local_42._1_1_, local_42) + pu_var19;
                            local_40 = local_40
                                + i_var25
                                + CARRY2(CONCAT11(local_42._1_1_, local_42), pu_var19);
                            local_42._0_1_ = local_40;
                            local_42._1_1_ = (local_40 >> 8);
                            u_var28 = CONCAT11(local_4a._1_1_, local_4a);
                            local_46._0_1_ = local_48;
                            local_46._1_1_ = (local_48 >> 8);
                            local_4a._0_1_ = 8;
                            local_4a._1_1_ = 0x10;
                            u_var29 = 0x1010;
                            uStack76 = 0xa0;
                            uStack75 = 0x7b;
                            local_48 = u_var28;
                            pass1_1010_ebf8(
                                CONCAT13(local_46._1_1_, CONCAT12(local_46, u_var28)),
                                local_44,
                                local_40,
                                local_40,
                            );
                        }
                        // LAB_1028_7ba0:
                        i_var25 = CONCAT11(local_46._1_1_, local_46) + 1;
                        local_46._0_1_ = i_var25;
                        local_46._1_1_ = (i_var25 >> 8);
                    }
                }
                return;
            }
            0xc => {
                pa_var7 = (u_var27 + 0x26);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x8b;
                local_46._1_1_ = 0xea;
                pass1_1030_145a(pa_var7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    local_42._0_1_ = SUB21(&local_30, 0);
                    local_42._1_1_ = (&local_30 >> 8);
                    local_44 = param_3;
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0xd;
                    local_4a._1_1_ = 0xeb;
                    local_46._0_1_ = u_var12;
                    local_46._1_1_ = uVar15;
                    u_var29 = read_file_1008_7dee(
                        CONCAT22(param_3, param_2),
                        CONCAT22(unaff_ss, &local_30),
                        2,
                    );
                    if (u_var29 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                    local_40 = param_2;
                    local_42._0_1_ = 8;
                    local_42._1_1_ = 0x10;
                    local_44 = 0xeaa2;
                    switch_statement_1008_73ea(param_2, param_3, local_30);
                    local_42._0_1_ = 8;
                    local_42._1_1_ = 0x10;
                    local_44 = 0xeaaf;
                    local_40 = u_var27;
                    local_2c = u_var29;
                    paVar35 = pass1_1030_0000(u_var27, u_var28, u_var29);
                    local_40 = (paVar35 >> 0x10);
                    u_var29 = paVar35;
                    local_42._0_1_ = SUB41(paVar35, 0);
                    local_42._1_1_ = (paVar35 >> 8);
                    local_44 = 0x1030;
                    local_46._0_1_ = 0xc5;
                    local_46._1_1_ = 0xea;
                    pp_var10 = (paVar35 + 0x10);
                    local_2a._2_2_ = u_var29;
                    local_26 = local_40;
                    (**pp_var10)();
                    if (u_var29 == 0) {
                        return;
                    }
                    u_var24 = (local_2a._2_2_ + 4);
                    local_e = u_var24;
                    local_c = (u_var24 >> 0x10);
                    local_40 = local_26;
                    local_42._0_1_ = local_2a._2_2_;
                    local_42._1_1_ = (local_2a._2_2_ >> 8);
                    u_var8 = (u_var27 + 0x26);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0xee;
                    local_4a._1_1_ = 0xea;
                    pass1_1030_14b4(
                        u_var8,
                        local_2a._2_2_,
                        local_26,
                        u_var24 & 0xffff | (*(local_2a._2_2_ + 6) & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                    uStack52 = (lVar9 >> 0x10);
                }
            }
            0xd => {
                local_a = ZEXT24(paVar26) << 0x10;
                u_var3 = &PTR_LOOP_1050_000c;
                local_10 = u_var3;
                local_e = (u_var3 >> 0x10);
                local_c = &ctx.PTR_LOOP_1050_0010;
                local_18 = &local_10;
                local_42._0_1_ = SUB21(&local_14, 0);
                local_42._1_1_ = (&local_14 >> 8);
                local_46._0_1_ = SUB21(&local_16, 0);
                local_46._1_1_ = (&local_16 >> 8);
                local_4a._0_1_ = SUB21(&local_10, 0);
                local_4a._1_1_ = (&local_10 >> 8);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uStack78 = SUB42(s_523a_bmp_1050_2116 + 7, 0);
                pass1_1008_3eb4(
                    CONCAT22(unaff_ss, &local_10),
                    CONCAT22(unaff_ss, &local_16),
                    CONCAT22(unaff_ss, &local_14),
                    CONCAT22(unaff_ss, &local_12),
                );
                local_18 = local_14 - 1;
                local_42._0_1_ = SUB21(&local_10, 0);
                local_42._1_1_ = (&local_10 >> 8);
                local_48 = &ctx.PTR_LOOP_1050_1008;
                local_4a._0_1_ = 0x39;
                local_4a._1_1_ = 0x21;
                local_46._0_1_ = u_var14;
                local_46._1_1_ = uVar17;
                local_e = local_18;
                BVar21 = pass1_1028_21ba(u_var27, local_44, CONCAT22(unaff_ss, &local_10), local_6);
                if (BVar21 == 0) {
                    local_18 = local_14 + 1;
                    local_42._0_1_ = SUB21(&local_10, 0);
                    local_42._1_1_ = (&local_10 >> 8);
                    local_48 = &ctx.PTR_LOOP_1050_1008;
                    local_4a._0_1_ = 0x59;
                    local_4a._1_1_ = 0x21;
                    local_46._0_1_ = u_var14;
                    local_46._1_1_ = uVar17;
                    local_44 = u_var28;
                    local_e = local_18;
                    BVar21 =
                        pass1_1028_21ba(u_var27, u_var28, CONCAT22(unaff_ss, &local_10), local_6);
                    if (BVar21 == 0) {
                        local_e = local_14;
                        local_18 = local_12 - 1;
                        local_42._0_1_ = SUB21(&local_10, 0);
                        local_42._1_1_ = (&local_10 >> 8);
                        local_48 = &ctx.PTR_LOOP_1050_1008;
                        local_4a._0_1_ = 0x82;
                        local_4a._1_1_ = 0x21;
                        local_46._0_1_ = u_var14;
                        local_46._1_1_ = uVar17;
                        local_44 = u_var28;
                        local_10 = local_18;
                        BVar21 = pass1_1028_21ba(
                            u_var27,
                            u_var28,
                            CONCAT22(unaff_ss, &local_10),
                            local_6,
                        );
                        if (BVar21 == 0) {
                            local_18 = local_12 + 1;
                            local_42._0_1_ = SUB21(&local_10, 0);
                            local_42._1_1_ = (&local_10 >> 8);
                            local_48 = &ctx.PTR_LOOP_1050_1008;
                            local_4a._0_1_ = 0xa2;
                            local_4a._1_1_ = 0x21;
                            local_46._0_1_ = u_var14;
                            local_46._1_1_ = uVar17;
                            local_44 = u_var28;
                            local_10 = local_18;
                            BVar21 = pass1_1028_21ba(
                                u_var27,
                                u_var28,
                                CONCAT22(unaff_ss, &local_10),
                                local_6,
                            );
                            if (BVar21 == 0) {
                                return;
                            }
                        }
                    }
                }
                local_42._0_1_ = SUB41(_PTR_LOOP_1050_5a64, 0);
                local_42._1_1_ = (_PTR_LOOP_1050_5a64 >> 8);
                local_40 = (_PTR_LOOP_1050_5a64 >> 0x10);
                local_44 = &ctx.PTR_LOOP_1050_1008;
                local_46._0_1_ = 0xb4;
                local_46._1_1_ = 0x21;
                pass1_1038_79b2(_PTR_LOOP_1050_5a64, local_a);
                return;
            }
            0xe => {
                pa_var7 = (u_var27 + 0x2a);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
                local_40 = (pa_var7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x31;
                local_46._1_1_ = 0xeb;
                pass1_1030_145a(pa_var7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xeb94;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x3b2, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0xaa;
                        local_42._1_1_ = 0xeb;
                        u_var22 = local_20;
                        pass1_1030_2068((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_dx_10;
                    }
                    local_30 = CONCAT22(local_40, u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x55;
                    local_46._1_1_ = 0xeb;
                    pp_var10 = (local_30 + 0x10);
                    local_44 = u_var29;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    local_40 = (local_30 >> 0x10);
                    uVar30 = local_30;
                    u_var24 = (uVar30 + 4);
                    u_var22 = (uVar30 + 6);
                    local_e = u_var24;
                    local_c = (u_var24 >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_42._0_1_ = local_30;
                    local_42._1_1_ = (local_30 >> 8);
                    u_var8 = (u_var27 + 0x2a);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = (u_var8 >> 0x10);
                    local_4a._0_1_ = 0x7e;
                    local_4a._1_1_ = 0xeb;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        uVar30,
                        local_40,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                    uStack52 = (lVar9 >> 0x10);
                }
            }
        }
        local_40 = 0x1030;
        local_42._0_1_ = 0xfb;
        local_42._1_1_ = 0xe6;
        ret_1030_154c();
    }
    return;
}

pub fn write_to_file_1028_e56c(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut HFILE16,
    param_2_00: u32,
) {
    let pp_var1: fn();
    let p_uvar2: *mut u16;
    let BVar3: bool;
    
    
    let mut unaff_ss: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        param_2_00,
        (param_2_00 >> 0x10),
    );
    local_18 = 0;
    while (true) {
        pu_var2 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
        _local_1c = CONCAT22(ctx.dx_reg, pu_var2);
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        local_18 = local_18 + 1;
    }
    local_2a = local_18;
    BVar3 = write_to_file_1008_7e1c(param_1_00, CONCAT22(unaff_ss, &local_2a), 4);
    if (BVar3 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
    } else {
        local_c = local_8;
        local_a = local_6;
        if (local_4 != 0) {
            local_c = 1;
            local_a = 0;
        }
        while {
            pu_var2 = &local_14;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
            _local_1c = CONCAT22(ctx.dx_reg, pu_var2);
            if ((ctx.dx_reg | pu_var2) == 0) {
                return;
            }
            pp_var1 = (*_local_1c + 0xc);
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, pu_var2, ctx.dx_reg);
            local_2a = local_2a & 0xffff0000 | ZEXT24(pu_var2);
            pu_var2 != 0x0
        } {}
    }
    return;
}

pub unsafe fn write_to_file_1028_dce2(param_1: *mut u32, param_2: *mut HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let BVar3: bool;
    let mut i_var4: i32;
    let pu_var5: *mut u16;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let extraout_var_04: u32;
    let extraout_var_05: u32;
    let extraout_var_06: u32;
    let extraout_var_07: u32;
    
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_2a: u16;
    let mut local_26: u32;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    u_var2 = write_to_file_1008_7cac(param_2, 10);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        let param1_val = unsafe { *param_1 };
        local_26 = param1_val;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_26), 4);
        if (BVar3 != 0) {
            u_var7 = (param_1 >> 0x10);
            u_var6 = param_1;
            local_1e = (u_var6 + 8);
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1e), 2);
            if (BVar3 != 0) {
                pp_var1 = (*_PTR_LOOP_1050_5166 + 0xc);
                (**pp_var1)(
                    &ctx.PTR_LOOP_1050_1008,
                    _PTR_LOOP_1050_5166,
                    (_PTR_LOOP_1050_5166 >> 0x10),
                    param_2,
                );
                if (BVar3 != 0) {
                    u_var2 = write_to_file_1008_7cac(param_2, 0xc);
                    i_var4 = CONCAT31(extraout_var_00, u_var2);
                    if (i_var4 != 0) {
                        write_to_file_1028_e56c(u_var6, u_var7, param_2, 0x1000000);
                        if (i_var4 != 0) {
                            u_var2 = write_to_file_1008_7cac(param_2, 0xd);
                            i_var4 = CONCAT31(extraout_var_01, u_var2);
                            if (i_var4 != 0) {
                                write_to_file_1028_e56c(u_var6, u_var7, param_2, 0x2000000);
                                if (i_var4 != 0) {
                                    u_var2 = write_to_file_1008_7cac(param_2, 0xe);
                                    i_var4 = CONCAT31(extraout_var_02, u_var2);
                                    if (i_var4 != 0) {
                                        write_to_file_1028_e56c(u_var6, u_var7, param_2, 0x3000000);
                                        if (i_var4 != 0) {
                                            u_var2 = write_to_file_1008_7cac(param_2, 0xf);
                                            i_var4 = CONCAT31(extraout_var_03, u_var2);
                                            if (i_var4 != 0) {
                                                write_to_file_1028_e56c(
                                                    u_var6, u_var7, param_2, 0x4000000,
                                                );
                                                if (i_var4 != 0) {
                                                    u_var2 = write_to_file_1008_7cac(param_2, 0x10);
                                                    i_var4 = CONCAT31(extraout_var_04, u_var2);
                                                    if (i_var4 != 0) {
                                                        write_to_file_1028_e56c(
                                                            u_var6, u_var7, param_2, 0x5000000,
                                                        );
                                                        if (i_var4 != 0) {
                                                            u_var2 = write_to_file_1008_7cac(
                                                                param_2, 0x11,
                                                            );
                                                            i_var4 =
                                                                CONCAT31(extraout_var_05, u_var2);
                                                            if (i_var4 != 0) {
                                                                write_to_file_1028_e56c(
                                                                    u_var6, u_var7, param_2,
                                                                    0x6000000,
                                                                );
                                                                if (i_var4 != 0) {
                                                                    u_var2 =
                                                                        write_to_file_1008_7cac(
                                                                            param_2, 0x12,
                                                                        );
                                                                    i_var4 = CONCAT31(
                                                                        extraout_var_06,
                                                                        u_var2,
                                                                    );
                                                                    if (i_var4 != 0) {
                                                                        write_to_file_1028_e56c(
                                                                            u_var6, u_var7,
                                                                            param_2, 0x7000000,
                                                                        );
                                                                        if (i_var4 != 0) {
                                                                            u_var2 = write_to_file_1008_7cac(param_2, 0x13);
                                                                            i_var4 = CONCAT31(
                                                                                extraout_var_07,
                                                                                u_var2,
                                                                            );
                                                                            if (i_var4 != 0) {
                                                                                write_to_file_1028_e56c(u_var6, u_var7, param_2, 0x8000000);
                                                                                if (i_var4 != 0) {
                                                                                    pass1_1028_dc52(
                                                                                                        CONCAT22(unaff_ss, &local_14),
                                                                                                    (&ctx.PTR_LOOP_1050_0000 + 1), 0,
                                                                                                    0x400);
                                                                                    while (true) {
                                                                                        pu_var5 = &local_14;
                                                                                        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var5));
                                                                                        local_18 = CONCAT22(ctx.dx_reg, pu_var5);
                                                                                        if ((ctx.dx_reg | pu_var5) == 0) {
                                                                                            break;
                                                                                        }
                                                                                        if ((pu_var5 + 0x100) != 0x8000002)
                                                                                        {
                                                                                            pass1_1038_3ba0(
                                                                                                                CONCAT22(ctx.dx_reg, pu_var5));
                                                                                        }
                                                                                    }
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
    return;
}

pub unsafe fn read_file_1028_def2(param_1: *mut u8, file_handle_1: *mut HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let BVar3: bool;
    let mut i_var4: i32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let extraout_var_04: u32;
    let extraout_var_05: u32;
    let extraout_var_06: u32;
    let extraout_var_07: u32;
    let mut u_var5: u16;
    let mut u_var6: u16;

    u_var5 = (file_handle_1 >> 0x10);
    u_var2 = read_file_1008_7cfe(file_handle_1);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        BVar3 = read_file_1008_7dee(file_handle_1, param_1, 4);
        if (BVar3 != 0) {
            BVar3 = read_file_1008_7dee(file_handle_1, (param_1 & 0xffff0000 | (param_1 + 8)), 2);
            if (BVar3 != 0) {
                u_var6 = file_handle_1;
                pp_var1 = (*_PTR_LOOP_1050_5166 + 0x10);
                (**pp_var1)(
                    &ctx.PTR_LOOP_1050_1008,
                    _PTR_LOOP_1050_5166,
                    (_PTR_LOOP_1050_5166 >> 0x10),
                    file_handle_1,
                );
                if (BVar3 != 0) {
                    u_var2 = read_file_1008_7cfe(file_handle_1);
                    i_var4 = CONCAT31(extraout_var_00, u_var2);
                    if (i_var4 != 0) {
                        pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x100);
                        if (i_var4 != 0) {
                            u_var2 = read_file_1008_7cfe(file_handle_1);
                            i_var4 = CONCAT31(extraout_var_01, u_var2);
                            if (i_var4 != 0) {
                                pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x200);
                                if (i_var4 != 0) {
                                    u_var2 = read_file_1008_7cfe(file_handle_1);
                                    i_var4 = CONCAT31(extraout_var_02, u_var2);
                                    if (i_var4 != 0) {
                                        pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x300);
                                        if (i_var4 != 0) {
                                            u_var2 = read_file_1008_7cfe(file_handle_1);
                                            i_var4 = CONCAT31(extraout_var_03, u_var2);
                                            if (i_var4 != 0) {
                                                pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x400);
                                                if (i_var4 != 0) {
                                                    u_var2 = read_file_1008_7cfe(file_handle_1);
                                                    i_var4 = CONCAT31(extraout_var_04, u_var2);
                                                    if (i_var4 != 0) {
                                                        pass1_1028_e628(
                                                            param_1, u_var6, u_var5, 0, 0x500,
                                                        );
                                                        if (i_var4 != 0) {
                                                            u_var2 =
                                                                read_file_1008_7cfe(file_handle_1);
                                                            i_var4 =
                                                                CONCAT31(extraout_var_05, u_var2);
                                                            if (i_var4 != 0) {
                                                                pass1_1028_e628(
                                                                    param_1, u_var6, u_var5, 0,
                                                                    0x600,
                                                                );
                                                                if (i_var4 != 0) {
                                                                    u_var2 = read_file_1008_7cfe(
                                                                        file_handle_1,
                                                                    );
                                                                    i_var4 = CONCAT31(
                                                                        extraout_var_06,
                                                                        u_var2,
                                                                    );
                                                                    if (i_var4 != 0) {
                                                                        pass1_1028_e628(
                                                                            param_1, u_var6,
                                                                            u_var5, 0, 0x700,
                                                                        );
                                                                        if (i_var4 != 0) {
                                                                            u_var2 =
                                                                                read_file_1008_7cfe(
                                                                                    file_handle_1,
                                                                                );
                                                                            i_var4 = CONCAT31(
                                                                                extraout_var_07,
                                                                                u_var2,
                                                                            );
                                                                            if (i_var4 != 0) {
                                                                                pass1_1028_e628(
                                                                                    param_1,
                                                                                    u_var6, u_var5,
                                                                                    0, 0x800,
                                                                                );
                                                                                if (i_var4 != 0) {
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
    return;
}

pub fn write_to_file_1028_d7a0(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut i_var2: i32;

    u_var1 = write_to_file_1008_7cac(param_1_00, 8);
    i_var2 = CONCAT11(extraout_AH, u_var1);
    if (i_var2 != 0) {
        i_var2 = 1;
    }
    return i_var2;
}

pub fn read_file_1028_d7ba(param_1: u16, param_2: u16, param_1_00: *mut HFILE16) -> i32 {
    let u_var1: u8;
    let extraout_AH: u8;

    u_var1 = read_file_1008_7cfe(param_1_00);
    if (CONCAT11(extraout_AH, u_var1) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
        return CONCAT11(extraout_AH, u_var1);
    }
    return 1;
}

pub fn write_to_file_1028_b5ec(in_struct_1: *mut Struct771, in_file_handle: *mut HFILE16) -> u16 {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    u_var5 = (in_struct_1 >> 0x10);
    i_var4 = in_struct_1;
    local_e = (i_var4 + 0xc);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_e), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    i_var3 = write_to_file_1030_16d6(in_struct_1, in_file_handle);
    if (i_var3 == 0) {
        return 0;
    }
    local_8 = (i_var4 + 0xc);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0xe);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x10);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x12);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x18);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x1a);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_4 = (i_var4 + 0x12);
    if (local_4 == 6) {
        local_4 = (i_var4 + 0x18);
    }
    if (local_4 < 1) {
        return 1;
    }
    if (SBORROW2(local_4, 1)) {
        return 1;
    }
    if (local_4 == 3 || (local_4 - 2) < 1) {
        local_8 = (i_var4 + 0x14);
    } else {
        if (local_4 == 4) {
            if ((i_var4 + 0x14) == 0) {
                local_8 = 0;
                BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
                // goto joined_r0x1028b766;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0x94);
        } else {
            if (local_4 != 5) {
                return 1;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xa4);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (BVar2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xa6);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (BVar2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xa8);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (BVar2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xaa);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (BVar2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xac);
        }
    }
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    // joined_r0x1028b766:
    if (BVar2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    return 1;
}

pub fn read_file_fn_1028_b81a(param_1: u32, in_file_1: *mut HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let u_var3: u8;
    let b_var4: bool;
    let mut u_var5: u16;
    let mut i32_var6: i32;
    let extraout_var: u32;
    
    let mut u_var7: i32;
    let mut unaff_ss: u16;
    let mut u_var8: u32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut local_3a: u32;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2a: u16;
    let mut local_26: [u8; 22];
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var9 = param_1;
    u_var7 = (param_1 >> 0x10);
    u_var3 = pass1_1030_1730(param_1, in_file_1);
    if (CONCAT31(extraout_var, u_var3) == 0) {
        return;
    }
    b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_4), 2);
    if (b_var4 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_6), 2);
    if (b_var4 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var4 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    u_var10 = in_file_1;
    u_var11 = (in_file_1 >> 0x10);
    switch_statement_1008_73ea(u_var10, u_var11, local_4);
    (i_var9 + 0xc) = b_var4;
    switch_statement_1008_73ea(u_var10, u_var11, local_6);
    (i_var9 + 0xe) = b_var4;
    switch_statement_1008_73ea(u_var10, u_var11, local_8);
    (i_var9 + 0x10) = b_var4;
    u_var5 = ctx.dx_reg;
    b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_4), 2);
    if (b_var4 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_6), 2);
    if (b_var4 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    b_var4 = read_file_1008_7dee(in_file_1, (param_1 & 0xffff0000 | (i_var9 + 0x1a)), 2);
    if (b_var4 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    (i_var9 + 0x12) = local_4;
    (i_var9 + 0x18) = local_6;
    local_a = (i_var9 + 0x12);
    if (local_a == 6) {
        local_a = (i_var9 + 0x18);
    }
    match (local_a) {
        1 | 2 | 3 => {
            u_var5 = i_var9 + 0x14;
            // LAB_1028_b968:
            b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(u_var7, u_var5), 2);
        }
        4 => {
            u_var8 = pass1_1028_e0bc(ctx._PTR_LOOP_1050_65e2, CONCAT22(local_3a, (i_var9 + 0xc)));
            local_e = (u_var8 >> 0x10);
            (i_var9 + 0x14) = u_var8;
            (i_var9 + 0x16) = local_e;
            if ((local_e | (i_var9 + 0x14)) != 0) {
                u_var5 = (i_var9 + 0x14) + 0x94;
                u_var7 = local_e;
                local_10 = u_var5;
                // goto LAB_1028_b968;
            }
            b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, local_26), 2);
        }
        5 => {
            i32_var6 = i_var9;
            pass1_1028_e100(ctx._PTR_LOOP_1050_65e2, (i_var9 + 0xc));
            (i_var9 + 0x14) = i32_var6;
            (i_var9 + 0x16) = u_var5;
            local_10 = (i_var9 + 0x14) + 0xa4;
            local_e = u_var5;
            b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(u_var5, local_10), 2);
            if (b_var4 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            b_var4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_2a), 2);
            if (b_var4 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            u_var1 = (i_var9 + 0x14);
            b_var4 = read_file_1008_7dee(in_file_1, (u_var1 & 0xffff0000 | (u_var1 + 0xa8)), 2);
            if (b_var4 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            u_var1 = (i_var9 + 0x14);
            b_var4 = read_file_1008_7dee(in_file_1, (u_var1 & 0xffff0000 | (u_var1 + 0xaa)), 2);
            if (b_var4 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            u_var1 = (i_var9 + 0x14);
            b_var4 = read_file_1008_7dee(in_file_1, (u_var1 & 0xffff0000 | (u_var1 + 0xac)), 2);
            if (b_var4 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            set_param_3_with_switch_1008_72bc(u_var10, u_var11, local_2a);
            u_var2 = (i_var9 + 0x14);
            (u_var2 + 0xa6) = b_var4;
            return;
        }
        _ => {}
        // default:
        // goto switchD_1028_ba97_caseD_6;
        9 => {
            i32_var6 = i_var9;
            pass1_1028_e100(ctx._PTR_LOOP_1050_65e2, (i_var9 + 0xc));
            (i_var9 + 0x14) = i32_var6;
            (i_var9 + 0x16) = u_var5;
            // goto switchD_1028_ba97_caseD_6;

            if (b_var4 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
        }
        6 => {
            // switchD_1028_ba97_caseD_6:
            return;
        }
    }
}

pub fn pass1_1028_b282(param_1: *mut Struct771, param_2: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write_to_file_1030_16d6(param_1, param_2);
    if (b_var1 != 0) {
        local_c = (param_1 + 0xc);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub fn pass1_1028_b2c8(param_1: u32, param_2: *mut HFILE16) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_1730(param_1, param_2);
    BVar2 = CONCAT11(extraout_AH, u_var1);
    if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return BVar2;
        }
        u_var3 = set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_4);
        (param_1 + 0xc) = u_var3;
        BVar2 = 1;
    }
    return BVar2;
}

pub fn read_from_file_1028_65e2(param_1: u32, param_2: *mut HFILE16) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let pu_var5: *mut u8;
    
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_ax != 0) {
        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (BVar2 != 0) {
            local_6 = 0;
            while (true) {
                if (local_4 <= local_6) {
                    return;
                }
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                u_var4 = pu_var5;
                local_14 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var4) == 0) {
                    local_14 = 0;
                } else {
                    local_14 = ctx.s_1_1050_389a;
                    (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var4 + 4) = 0;
                    (u_var4 + 6) = 0;
                    (u_var4 + 8) = 0;
                    (u_var4 + 10) = 0;
                    (u_var4 + 0xc) = 0;
                    local_14 = 0x56ce;
                    (u_var4 + 2) = 0x1018;
                }
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_10), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_c), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_16), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 10)), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 0xc)), 2);
                if (BVar2 == 0) {
                    break;
                }
                (local_14 + 4) = local_10;
                u_var3 = local_10;
                set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_c);
                u_var6 = (local_14 >> 0x10);
                (local_14 + 6) = u_var3;
                (local_14 + 8) = local_16;
                pp_var1 = ((param_1 + 0x20) + 8);
                (**pp_var1)();
                local_6 = local_6 + 1;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_to_file_1028_64d6(param_1: *mut Struct771, param_2: *mut HFILE16) -> u16 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let pu_var3: *mut u16;
    let b_var4: bool;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    i_var2 = write_to_file_1028_b5ec(param_1, param_2);
    if (i_var2 != 0) {
        u_var5 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x20));
        u_var1 = (param_1 + 0x20);
        local_1c = (u_var1 + 8);
        pu_var3 = &local_1c;
        local_10 = local_1c;
        while (true) {
            b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, pu_var3), 2);
            if (b_var4 == 0) {
                break;
            }
            _local_e = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (_local_e == 0) {
                return 1;
            }
            local_1e = (_local_e + 4);
            b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1e), 2);
            if (b_var4 == 0) {
                break;
            }
            local_20 = (_local_e + 6);
            b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_20), 2);
            if (b_var4 == 0) {
                break;
            }
            local_22 = (_local_e + 8);
            b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_22), 2);
            if (b_var4 == 0) {
                break;
            }
            local_24 = (_local_e + 10);
            b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_24), 2);
            if (b_var4 == 0) {
                break;
            }
            local_26 = (_local_e + 0xc);
            pu_var3 = &local_26;
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn write_to_file_1028_5f82(param_1: *mut Struct771, param_2: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write_to_file_1028_b5ec(param_1, param_2);
    if (b_var1 != 0) {
        local_c = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub fn read_from_file_1028_5fc8(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;

    read_file_fn_1028_b81a(param_1, param_2);
    if ((in_ax != 0)
        && (
            b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 2),
            b_var1 == 0,
        ))
    {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn write_file_func_1028_4a1a(param_1: *mut Struct771, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;

    write_to_file_1028_b5ec(param_1, param_2);
    if ((in_ax != 0)
        && (
            b_var1 =
                write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 10),
            b_var1 == 0,
        ))
    {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    return;
}

pub fn pass1_1028_4a5a(param_1: u32, in_file_1: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;

    read_file_fn_1028_b81a(param_1, in_file_1);
    if ((in_ax != 0)
        && (
            b_var1 = read_file_1008_7dee(in_file_1, (param_1 & 0xffff0000 | (param_1 + 0x20)), 10),
            b_var1 == 0,
        ))
    {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn write_to_file_1028_3d0e(in_struct_1: *mut Struct771, in_file_handle: *mut HFILE16) {
    let mut in_ax: i32;
    let bool_res: bool;
    let local_struct_1: *mut Struct771;
    let mut local_struct_1_hi: u16;
    
    let local_10: *mut u8;
    let local_8: *mut u8;

    write_to_file_1028_b5ec(in_struct_1, in_file_handle);
    if (in_ax != 0) {
        local_struct_1_hi = (in_struct_1 >> 0x10);
        local_struct_1 = in_struct_1;
        local_10 = local_struct_1.field_0x20;
        bool_res = write_to_file_1008_7e1c(in_file_handle, CONCAT22(ctx.stack_seg_reg, &local_10), 4);
        if (bool_res != 0) {
            local_8 = local_struct_1.field_0x24;
            bool_res = write_to_file_1008_7e1c(in_file_handle, CONCAT22(ctx.stack_seg_reg, &local_8), 4);
            if (bool_res != 0) {
                write_to_file_1008_7a22(in_file_handle, local_struct_1.field_0x28);
                if (bool_res != 0) {
                    return;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_file_1028_3d92(in_struct: *mut Struct772, in_file_handle: *mut HFILE16) {
    let mut in_ax: i32;
    let local_struct_1: *mut Struct772;
    let b_var1: bool;
    let mut u_var2: u16;

    read_file_fn_1028_b81a(in_struct, in_file_handle);
    if (in_ax != 0) {
        local_struct_1 = in_struct;
        local_struct_1 = &local_struct_1.field_0x20;
        b_var1 = read_file_1008_7dee(
            in_file_handle,
            (in_struct & 0xffff0000 | ZEXT24(local_struct_1)),
            4,
        );
        if (b_var1 != 0) {
            b_var1 = read_file_1008_7dee(
                in_file_handle,
                (in_struct & 0xffff0000 | &local_struct_1.field_0x24),
                4,
            );
            if (b_var1 != 0) {
                u_var2 = read_file_1008_7ad4(in_file_handle, local_struct_1.field_0x28);
                if (u_var2 != 0) {
                    return;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn write_file_fn_1028_2418(param_1: u32, param_2: *mut HFILE16) -> bool {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_1c: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar2 != 0) {
        u_var3 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x20));
        u_var1 = (param_1 + 0x20);
        local_1c = (u_var1 + 8);
        local_10 = local_1c;
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1c), 2);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return BVar2;
        }
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            _local_e = lVar4;
            BVar2 = write_to_file_1038_75ca(lVar4, param_2);
            if (BVar2 == 0) {
                return BVar2;
            }
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub unsafe fn read_file_fn_1028_24a2(param_1: u32, param_2: *mut HFILE16) -> bool {
    let pp_var1: fn();
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let struct_a: *mut Struct199;
    
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let ph_var6: *mut HFILE16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = read_file_fn_1028_b81a(param_1, param_2);
    struct_a = (u_var5 >> 0x10);
    if (u_var5 == 0) {
        return 0;
    }
    if (1 < PTR_LOOP_1050_0312) {
        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return 0;
        }
        local_6 = 0;
        while (local_6 < local_4) {
            u_var4 = local_4;
            ph_var6 = param_2;
            process_struct_1000_179c(0x2a, struct_a);
            if ((struct_a | u_var4) == 0) {
                u_var5 = 0;
            } else {
                u_var5 = pass1_1038_6520(CONCAT22(struct_a, u_var4));
            }
            i_var3 = read_from_file_1038_774e(u_var5, ph_var6);
            if (i_var3 == 0) {
                return 0;
            }
            pp_var1 = ((param_1 + 0x20) + 8);
            (**pp_var1)();
            local_6 = local_6 + 1;
            struct_a = ctx.dx_reg;
        }
    }
    return 1;
}

pub fn file_write_fn_1028_1452(param_1: u32, param_2: *mut HFILE16) -> bool {
    let mut i_var1: i32;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    i_var1 = write_to_file_1028_b5ec(param_1, param_2);
    if (i_var1 != 0) {
        u_var3 = (param_1 >> 0x10);
        local_c = (param_1 + 0x22);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (BVar2 != 0) {
            local_6 = (param_1 + 0x20);
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
            if (BVar2 != 0) {
                local_6 = PTR_LOOP_1050_4fbc;
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
                if (BVar2 != 0) {
                    return 1;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn read_file_fn_1028_14d8(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_ax != 0) {
        b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x22)), 2);
        if ((b_var1 != 0)
            && (
                b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2),
                b_var1 != 0,
            ))
        {
            (param_1 + 0x20) = local_4;
            if (PTR_LOOP_1050_0312 < 2) {
                return;
            }
            b_var1 = read_file_1008_7dee(param_2, &PTR_LOOP_1050_4fbc, 2);
            if (b_var1 != 0) {
                return;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn file_write_fn_1028_0234(param_1: *mut Struct731, param_2: u32) -> bool {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let BVar3: bool;
    let local_bx_28: *mut Struct731;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    i_var2 = write_to_file_1028_b5ec(param_1, param_2);
    if (i_var2 != 0) {
        u_var4 = (param_1 >> 0x10);
        local_bx_28 = param_1;
        local_1a = local_bx_28.field_0x20;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1a), 2);
        if (BVar3 != 0) {
            pass1_1008_5784(CONCAT22(unaff_ss, local_a), local_bx_28.field_0x22);
            u_var1 = local_bx_28.field_0x22;
            local_14 = (u_var1 + 8);
            local_10 = local_14;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_14), 2);
            while (BVar3 != 0) {
                _local_e = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
                if (_local_e == 0) {
                    return 1;
                }
                local_14 = (_local_e + 4);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 6);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 8);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 10);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 0xc);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_14), 2);
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn file_read_fn_1028_0374(param_1: u32, param_2: *mut HFILE16) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let BVar2: bool;
    let mut u_var3: u16;
    let local_AX_291: *mut Struct732;
    let mut u_var4: i32;
    
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_28: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var5: *mut u8;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_ax != 0) {
        BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
            if (BVar2 != 0) {
                local_6 = 0;
                while (true) {
                    if (local_4 <= local_6) {
                        return;
                    }
                    pu_var5 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    u_var4 = pu_var5;
                    local_14 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
                    if ((ctx.dx_reg | u_var4) == 0) {
                        local_14 = 0;
                    } else {
                        local_14 = ctx.s_1_1050_389a;
                        (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
                        (u_var4 + 4) = 0;
                        (u_var4 + 6) = 0;
                        (u_var4 + 8) = 0;
                        (u_var4 + 10) = 0;
                        (u_var4 + 0xc) = 0;
                        local_14 = 0x56ce;
                        (u_var4 + 2) = 0x1018;
                    }
                    BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_10), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_c), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_18), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 =
                        read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 10)), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 =
                        read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 0xc)), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    (local_14 + 4) = local_10;
                    u_var3 = local_10;
                    set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_c);
                    u_var6 = (local_14 >> 0x10);
                    (local_14 + 6) = u_var3;
                    (local_14 + 8) = local_18;
                    pp_var1 = ((param_1 + 0x22) + 8);
                    (**pp_var1)();
                    local_6 = local_6 + 1;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn file_write_fn_1020_e94e(param_1: u32, param_2: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write_to_file_1030_de7c(param_1, param_2);
    if (b_var1 != 0) {
        local_c = (param_1 + 0x24);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub fn file_read_fn_1020_e994(param_1: u32, param_2: *mut HFILE16) {
    let mut in_ax: i32;
    let b_var1: bool;

    read_from_file_1030_dec4(param_1, param_2);
    if ((in_ax != 0)
        && (
            b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x24)), 2),
            b_var1 == 0,
        ))
    {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn file_write_fn_1020_e6a4(param_1: u32, param_2: *mut HFILE16) -> u16 {
    let mut i_var1: i32;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    i_var1 = write_to_file_1030_de7c(param_1, param_2);
    if (i_var1 != 0) {
        u_var3 = (param_1 >> 0x10);
        local_c = (param_1 + 0x24);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (BVar2 != 0) {
            local_6 = (param_1 + 0x26);
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
            if (BVar2 != 0) {
                return 1;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn file_read_fn_1020_e70e(param_1: *mut u8, param_2: u32) {
    let mut in_ax: i32;
    let b_var1: bool;

    read_from_file_1030_dec4(param_1, param_2);
    if (in_ax != 0) {
        b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x24)), 2);
        if (b_var1 != 0) {
            b_var1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x26)), 2);
            if (b_var1 != 0) {
                return;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn call_write_to_file_1020_d3d4(param_1: *mut u8, in_file_handle: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write_to_file_1028_b5ec(param_1, in_file_handle);
    if (b_var1 != 0) {
        local_c = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub fn call_read_file_1020_d41a(param_1: *mut u8, param_2: *mut HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let local_4: *mut u8;

    b_var1 = read_file_fn_1028_b81a(param_1, param_2);
    if (b_var1 != 0) {
        b_var1 = read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return b_var1;
        }
        *(param_1 + 0x20) = local_4;
        b_var1 = 1;
    }
    return b_var1;
}

pub fn call_write_to_file_1020_a644(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let u_var1: u8;
    let mut write_file_result: i32;
    let extraout_AH: u8;

    u_var1 = write_to_file_1008_7cac(param_1_00, 0xb);
    write_file_result = CONCAT11(extraout_AH, u_var1);
    if (write_file_result != 0) {
        write_file_result = 1;
    }
    return write_file_result;
}

pub fn call_read_file_1020_a65e(param_1: u32, in_file_handle: *mut HFILE16) -> u16 {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;
    
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: [u8; 2];
    let mut local_4: [u8; 2];

    u_var1 = read_file_1008_7cfe(in_file_handle);
    if (CONCAT11(extraout_AH, u_var1) != 0) {
        if (1 < PTR_LOOP_1050_0312) {
            // LAB_1020_a6dc:
            pass1_1020_b97e(param_1, (param_1 >> 0x10), 0);
            return 1;
        }
        BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, local_4), 2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, local_8), 2);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, local_6), 2);
                if (BVar2 != 0) {
                    BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, local_a), 2);
                    if (BVar2 != 0) {}
                    // goto LAB_1020_a6dc;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return 0;
}

pub unsafe fn pass1_1020_8a9c(param_1: *mut Struct393) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let ppVar4: *mut pass1_struct_1;
    let pa_var5: *mut Struct104;
    let local_struct_1: *mut Struct393;
    let local_struct_1_hi: *mut Struct393;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: [u8; 30];
    let mut local_2a: [u8; 36];
    let mut local_6: u16;
    let mut local_4: u16;
    let local_struct_1_1: *mut Struct393;

    local_struct_1 = param_1;
    local_struct_1_1 = local_struct_1;
    local_struct_1_hi = (param_1 >> 0x10);
    process_struct_1020_847a(param_1, 2);
    u_var2 = &local_struct_1.field_0x14 + 2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | u_var2));
    _local_4c = param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x1c);
    zero_list_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x1c)));
    &local_struct_1.field_0x22 = 0;
    param_1.field_0x0 = 0x8e92;
    local_struct_1.u16_0x2 = 0x1020;
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    u_var3 = (ppVar4 >> 0x10);
    local_struct_1.field_0x22 = ppVar4;
    local_struct_1.field_0x24 = u_var3;
    pass1_1018_2678(
        local_struct_1.field_0x22,
        u_var3,
        param_1 & 0xffff0000 | u_var2,
    );
    pa_var5 = pass1_1018_268e(&local_struct_1.field_0x22);
    local_4 = (pa_var5 >> 0x10);
    u_var3 = pa_var5;
    u_var1 = &local_struct_1.field_0x8;
    local_6 = u_var3;
    pass1_1020_8712(
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        u_var1,
        (u_var1 >> 0x10),
        pa_var5,
        param_1 & 0xffff0000 | u_var2,
    );
    u_var1 = &local_struct_1.field_0x22;
    local_struct_1 = (u_var1 >> 0x10);
    pass1_1018_26c2(u_var1, local_struct_1, _local_4c);
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 2);
    process_struct_1008_48fe(
        CONCAT22(unaff_ss, local_2a),
        1,
        CONCAT22(ctx.dx_reg, u_var3),
    );
    pass1_1008_3f92(CONCAT22(unaff_ss, local_48), CONCAT22(unaff_ss, local_2a));
    u_var1 = &local_struct_1_1.field_0x8;
    pass1_1020_8712(
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        (u_var1 + 8),
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, local_48),
        _local_4c,
    );
    process_struct_1008_41bc(CONCAT22(unaff_ss, local_48));
    close_file_1008_496c(local_2a);
    return;
}
