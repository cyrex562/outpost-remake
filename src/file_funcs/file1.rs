use crate::app_context::AppContext;
use crate::err_funcs::{error_check_1000_0dc6, error_check_1000_17ce};
use crate::file_funcs::file2;
use crate::list_funcs::set_array_val_1008_72a8;
use crate::mem_funcs::{Address, alloc_mem_1000_07fc, alloc_mem_1000_0a48, alloc_mem_1000_1708, get_fn_ptr_at_address};
use crate::other_funcs::{modify_list_1008_3f62, set_param_3_with_switch_1008_72bc, switch_statement_1008_738c, switch_statement_1008_73ea, zero_list_1008_3e38};
use crate::pass::pass13_funcs::pass1_1008_b0bc;
use crate::pass::pass14_funcs::{pass1_1008_3e76, pass1_1008_3eb4, pass1_1008_3f92, pass1_1008_4b8e, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6eee, pass1_1008_6f7a, pass1_1008_7006, pass1_1008_7056, pass1_1008_766e, pass1_fn_1008_60e8};
use crate::pass::pass15_funcs::{pass1_1020_ba3e, pass1_1020_bb16, pass1_1020_bb8a, pass1_1020_c444, pass1_1020_c4a8};
use crate::pass::pass17_funcs::pass1_1030_1cd8;
use crate::pass::pass4_funcs::pass1_1028_e1ec;
use crate::prog_structs::prog_structs_10::Struct933;
use crate::prog_structs::prog_structs_11::Struct934;
use crate::prog_structs::prog_structs_12::Struct235;
use crate::prog_structs::prog_structs_16::{Struct1143, Struct493};
use crate::prog_structs::prog_structs_17::Struct407;
use crate::prog_structs::prog_structs_2::{Struct131, Struct199, Struct390, Struct7};
use crate::prog_structs::prog_structs_20::{Struct120, Struct506};
use crate::prog_structs::prog_structs_23::{Struct1167, Struct1168, Struct935};
use crate::prog_structs::prog_structs_24::{Struct103, Struct8};
use crate::prog_structs::prog_structs_25::Struct9;
use crate::prog_structs::prog_structs_26::{Struct1140, Struct1141, Struct1142, Struct121};
use crate::prog_structs::prog_structs_28::{FileObject, Struct936, Struct961};
use crate::prog_structs::prog_structs_29::Struct425;
use crate::prog_structs::prog_structs_31::Struct962;
use crate::prog_structs::prog_structs_5::Struct1174;
use crate::prog_structs::prog_structs_6::Struct473;
use crate::string_funcs::{get_string_index_1000_3da4, process_string_1008_7e4a, string_fn_1000_3f9c};
use crate::struct_funcs::{pass1_1038_6520, process_struct_1000_179c, process_struct_1008_48fe, process_struct_1008_4c98, process_struct_1008_574a, process_struct_1008_dcdc, struct_fn_1000_160a};
use crate::typedefs::HFILE16;
use crate::util::{CONCAT11, CONCAT22, CONCAT31, ZEXT24};
use crate::winapi_funcs::{_hread, _hwrite16, _lclose16, _lcreat16, _llseek16, _lopen16};

pub unsafe fn close_file_1008_496c(ctx: &mut AppContext, struct_param_2: &mut Struct7) {
    let pu_var1: u32;
    let mut u_var2: u32;
    let local_bx_5: Struct7;
    let mut local_res6: u16;
    let temp_86216c208fd: u32;
    let mut func_ptr: u32;
    let mut temp_5f096a4ace: u32;

    param_2.u16_field_0 = ctx.PTR_LOOP_1050_4c4c;
    struct_param_2.u16_field_1 = ctx.PTR_LOOP_1050_1008;
    pu_var1 = struct_param_2.func_ptr_2;
    u_var2 = struct_param_2.u32_field_3;
    if (u_var2 | pu_var1) != 0 {
        let fn_to_call = get_fn_ptr_at_address(struct_param_2.func_ptr_2);
        fn_to_call();
    }
    error_check_1000_17ce(ctx, struct_param_2);
    if &struct_param_2.pv_buffer_0x1a != 0 {
        temp_5f096a4ace = struct_param_2.pv_buffer_0x1a;
        error_check_1000_0dc6(ctx, temp_5f096a4ace);
    }
    if (struct_param_2.hfile_field_5 != 0xffff) {
        _lclose16(struct_param_2.hfile_field_5);
    }
    struct_param_2._type.u16_field_0 = ctx.s_1_1050_389a.clone();
    struct_param_2.u16_field_1 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn read_from_file_1008_49e8(ctx: &mut AppContext, param_1: &mut Address<Struct7>) {
    let paVar1: *mut Struct131;
    let mut u_var2: u32;
    let mut file_handle: HFILE16;
    let mut ptr_var3: Vec<u8>;
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

    if (param_1.file_path != 0x0) {
        if (param_1.u16_field_0x1e != 0) {
            return;
        }
        if (param_1.file == 0xffff) {
            file_handle = _lopen16(param_1.file_path, 0 );
            param_1.file = file_handle;
            if (file_handle == 0xffff) {
                return;
            }
        }
        read_count = 0;
        bytes_read = _hread(0xe, CONCAT22(unaff_ss, &local_18), param_1.file);
        if ((bytes_read == 0xe) && ((bytes_read >> 0x10) == 0)) {
            read_count = local_16;
            if (local_18 == &ctx.PTR_LOOP_1050_4d42) {
                lVar8 = _llseek16(0, 0, param_1.file);
                ptr_var3 = lVar8;
                local_1c = ctx.__g_Struct94_ptr_1;
                local_1a = (ctx.__g_Struct94_ptr_1 >> 0x10);
                alloc_mem_1000_0a48(1, read_count, (read_count >> 0x10), local_1c, local_1a);
                param_1.pv_buffer_0x1a = ptr_var3;
                param_1.u16_field_0x1c = ctx.dx_reg;
                if ((ctx.dx_reg | param_1.pv_buffer_0x1a) == 0) {
                    return;
                }
                // WARNING: Load size is inaccurate
                lVar8 = _hread(read_count, param_1.pv_buffer_0x1a, param_1.file);
                struct_a = (lVar8 >> 0x10);
                local_a = lVar8;
                local_18 = param_1.file;
                local_8 = struct_a;
                _lclose16(local_18);
                param_1.file = 0xffff;
                param_1.u16_field_0x1e = 1;
                // WARNING: Load size is inaccurate
                param_1.pv_field_0xe = param_1.pv_buffer_0x1a;
                u_var6 = &param_1.pv_buffer_0x1a;
                i_var4 = u_var6;
                u_var6 = u_var6 & 0xffff0000;
                param_1.pv_field_0x12 = (u_var6 | i_var4 + 0xe);
                u_var6 = u_var6 | i_var4 + 0x436;
                param_1.u32_field_0x16 = u_var6;
                local_16 = CONCAT22(local_16._2_2_, 0x14);
                local_18 = offset;
                process_struct_1000_179c(0x14, struct_a);
                pa_var5 = u_var6;
                if ((struct_a | pa_var5) == 0) {
                    param_1.func_ptr_0x4 = 0;
                } else {
                    local_16 = CONCAT22(local_16._2_2_, 0x100);
                    paVar1 = param_1.pv_field_0x12;
                    Struct131_4 = paVar1;
                    Struct131_4 = Struct131_4 + 0x28;
                    u_var2 = paVar1 & 0xffff0000;
                    u_var7 = u_var2 | Struct131_4;
                    local_18 = (u_var2 >> 0x10);
                    process_struct_1008_4c98(
                        ctx,(u_var6 & 0xffff | Struct131_4 << 0x10),
                        u_var7,
                        0x100,
                    );
                    pa_var5 = u_var7;
                    param_1.func_ptr_0x4 = pa_var5;
                    param_1.i_field_4 = ctx.dx_reg;
                }
                if (param_1.pv_field_0x22 != 0) {
                    local_16 = local_16 & 0xffff0000 | in_stack_00000006;
                    local_18 = param_1;
                    pass1_1008_4b8e(param_1, pa_var5);
                    return;
                }
                return;
            }
        }
        _lclose16(param_1.file);
        param_1.file = 0xffff;
    }
    return;
}

pub unsafe fn close_file_1008_4c26(struct_param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    close_file_1008_496c(ctx, struct_param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, struct_param_1);
    }
    return struct_param_1;
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
    local_Struct117.field_0x4 = 0;
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
    error_check_1000_17ce(ctx,param_1);
    return;
}

pub unsafe fn write_to_file_1008_6e02(ctx: &mut AppContext, param_1: &mut FileObject) -> bool {
    let mut local_4: [u8; 2];

    ctx.g_u16_1050_0310 = 0;
    let mut b_var1 = write_to_file_1008_70a6(ctx,param_1);
    if b_var1 != false {
        // u_var4 = (param_1 >> 0x10);
        // u_var3 = param_1;
        set_array_val_1008_72a8(&local_4, param_1.file);
        let mut i_var2 = pass1_1008_7006(param_1, &local_4);
        if (i_var2 != 0)
            && (
                i_var2 = pass1_1008_6eee(param_1, &local_4),
                i_var2 != 0,
            )
        {
            return close_file_1008_726c(ctx,param_1);
        }
        _lclose16(param_1.file);
    }
    return false;
}

pub unsafe fn close_file_1008_6e78(ctx: &mut AppContext, struct_param_1: &mut FileObject) -> bool {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    // let mut unaff_ss: u16;
    let mut local_4: [u8; 2];

    ctx.g_u16_1050_0310 = 0;
    b_var1 = read_from_file_1008_7146(ctx, struct_param_1);
    if b_var1 != 0 {
        // u_var4 = (struct_param_1 >> 0x10);
        // u_var3 = struct_param_1;
        // CONCAT22(unaff_ss, local_4)
        // (u_var3 + 4)
        // set_array_val_1008_72a8(local_4, struct_param_1 );
        // CONCAT22(unaff_ss, local_4)
        i_var2 = pass1_1008_7056(struct_param_1, &mut local_4);
        if (i_var2 != 0)
            && (
                i_var2 = pass1_1008_6f7a(struct_param_1, &mut local_4),
                i_var2 != 0,
            )
        {
            b_var1 = close_file_1008_726c(ctx, struct_param_1);
            if (b_var1 == 0) {
                return 0;
            }
            return 1;
        }
        _lclose16((u_var3 + 4));
    }
    return 0;
}

pub fn write_to_file_1008_70a6(ctx: &mut AppContext,
                               param_1: &mut FileObject) -> bool {
    let mut local_file: HFILE16;
    if param_1._type.file != 0xffff {
        _lclose16(param_1.file);
        param_1._type.file = 0xffff;
    }
    local_file = _lcreat16(&param_1.path, 0);
    param_1.file = local_file;
    if local_file == 0xffff {
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
        let count = get_string_index_1000_3da4(ctx.s__1050_65a0);
        let bytes_written = _hwrite16(param_1.file, ctx.s__1050_65a0, count as usize);
        if bytes_written == count as usize {
            return true;
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return false;
}

pub fn read_from_file_1008_7146(ctx: &mut AppContext, struct_param_1: &mut FileObject) -> bool {
    let mut file: u16;
    let b_var1: bool;
    // let local_file_obj: *mut file_object;
    let mut u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    // local_file_obj = param_1;
    if local_file_obj.file != 0xffff {
        _lclose16(struct_param_1.file);
        struct_param_1.file = 0xffff;
    }
    struct_param_1.file = _lopen16(&param_1.path, 0 );
    // local_file_obj.file = file;
    if file == 0xffff {
        ctx.g_u16_1050_0310 = 0x6cf;
    } else {
        return read_file_func_1008_71a0(ctx, struct_param_1);
    }
    return false;
}

pub fn read_file_func_1008_71a0(ctx: &mut AppContext, in_file_object: &mut FileObject) -> bool {
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

pub fn close_file_1008_726c(ctx: &mut AppContext, param_1: &mut FileObject) -> bool {
    let mut file_handle: HFILE16;
    if param_1.file != 0xffff {
        file_handle = _lclose16(param_1.file);
        if file_handle == 0xffff {
            ctx.g_u16_1050_0310 = 0x6d1;
            return false;
        }
        param_1.file = 0xffff;
        ctx.g_u16_1050_0310 = 0;
    }
    return true;
}

pub unsafe fn read_file_1008_7548(
    ctx: &mut AppContext,
    in_file_handle: &HFILE16,
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
                pass1_1020_c444(ctx, CONCAT22(local_6, u_var3), 100, local_a);
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
    param_1: &HFILE16,
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

pub fn write_to_file_1008_79f0(ctx: &mut AppContext, in_file: &HFILE16, param_2:u32) {
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

pub fn write_to_file_1008_7a22(param_1: &HFILE16, param_2: *mut Vec<u8>) {
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

pub unsafe fn write_file_1008_7b4c(hfile_param_1: &HFILE16, param_2: Vec<u8>) -> u16 {
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
    b_var1 = write_to_file_1008_7e1c(hfile_param_1, CONCAT22(ctx.stack_seg_reg, &local_12), 2);
    if b_var1 != 0 {
        local_c = local_6;
        b_var1 = write_to_file_1008_7e1c(hfile_param_1, CONCAT22(ctx.stack_seg_reg, &local_c), 2);
        if b_var1 != 0 {
            local_c = local_8;
            b_var1 = write_to_file_1008_7e1c(hfile_param_1, CONCAT22(ctx.stack_seg_reg, &local_c), 2);
            if b_var1 != 0 {
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
                               param_1: &HFILE16,
                               param_2: &mut String) -> bool {
    if param_2 != 0x0 {
        let u_var1 = get_string_index_1000_3da4(param_2);
        let write_file_result = write_to_file_1008_7e1c(param_1, param_2, (u_var1 + 1) as usize);
        return write_file_result;
    }
    write_to_file_1008_7e1c(param_1, &ctx.s_playerName_1050_148e[0xc..], 1);
    return true;
}

pub unsafe fn read_file_into_str_1008_7c6e(hfile_param1: &HFILE16, out_str: &mut String) {
    let mut pc_var1: String = String::new();
    // let mut unaff_ss: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let local_c: u8 = 0;

    loop {
        pc_var1 = out_str.clone();
        // read 1 byte i32o a stack buffer
        let mut buffer: Vec<u8> = Vec::new();
        // CONCAT22(unaff_ss, &local_c)
        let bytes_read = _hread(hfile_param1, &mut buffer, 1);
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

pub fn write_to_file_1008_7cac(param_1: &HFILE16, param_2: u16) -> bool {
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

pub unsafe fn read_file_1008_7cfe(ctx: &mut AppContext, param_1: &HFILE16) -> u8 {
    let mut b_var1: bool;
    let b_var2: bool;
    // let mut unaff_ss: u16;
    let mut bytes_read: usize;
    let mut local_410: usize;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    // let mut local_406: [u8;1024];
    // let mut local_406: String = String::new();
    let mut read_buffer: Vec<u8> = Vec::new();
    let mut seek_offset = 0;
    let mut b_var1 = false;
    loop {
        // let param_1_val =  *param_1;
        _llseek16(param_1, seek_offset, 0);
        bytes_read = _hread(param_1, &mut read_buffer, 0x400);
        local_410 = 0;
        while local_410 < bytes_read {
            if local_406[local_410] == _PTR_s_dcbSC_1050_0336_1050_033c[0] {
                if !b_var1 {
                    b_var1 = true;
                    // seek_offset = CONCAT22(
                    //     (seek_offset >> 0x10) + local_410._2_2_ + CARRY2(seek_offset, local_410),
                    //     seek_offset + local_410,
                    // );
                    seek_offset = seek_offset + local_410;
                    break;
                }
                b_var1 = false;
                b_var2 = process_string_1008_7e4a();
                if b_var2 != false {
                    _llseek16(param_1_val, local_410 + seek_offset + 7, 0);
                    return 0x1;
                }
            }
            local_410 = local_410 + 1;
        }
        if !b_var1 {
            if bytes_read < 0x400 {
                return 0;
            }
            // seek_offset._0_2_ = CONCAT11(seek_offset._1_1_ + 4, seek_offset);
            // seek_offset = CONCAT22((seek_offset >> 0x10) + (0xfb < seek_offset._1_1_), seek_offset);
            seek_offset = seek_offset + 4;
        }
    }
}

pub fn read_file_1008_7dee(
    file_handle: &HFILE16,
    buffer: &mut Vec<u8>,
    count: usize,
) -> bool {
    let bytes_read = _hread(&file_handle, buffer, count);
    bytes_read == count
}

pub fn write_to_file_1008_7e1c(
    file: &HFILE16,
    buffer: &Vec<u8>,
    count: usize,
) -> bool {
    let bytes_written = _hwrite16(file, buffer, count);
    bytes_written == count
}

pub unsafe fn read_file_1008_bb5e(param_1: &mut Struct120, param_2: &HFILE16) {
    let pp_var1: fn();
    let mut b_read_result_1: u16;
    let b_read_result_2: bool;
    let var3: *mut Struct199;
    let mut local_AX_168: *mut Struct121;
    let mut local_AX_189: String = String::new();
    let u_var2: u8;
    let b_var4: bool;
    let mut string_1: String = String::new();
    let mut u_var6: i32;
    let extraout_var: u32;
    let struct_a: Struct120;
    let extraout_var_00: u32;
    let in_dx: Struct199;
    let struct_a: Struct199;
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

    if ctx.PTR_LOOP_1050_0312 < 2 {
        // bail if the read fails
        return;
    }
    u_var2 = read_file_1008_7cfe(ctx, param_2);
    if u_var2 == 0 {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        struct_a = param_1.clone();
        struct_a._0_2_ = struct_a + 0x22;
        // read two bytes from the file i32o the structure passed in by param_1
        b_read_result_1 = read_file_1008_7dee(
            _file_handle_ptr,
            &mut param_1.field_0x0,
            2,
        );
    }
    b_read_result_2 = read_file_1008_7dee(_file_handle_ptr, CONCAT22buffer_1, 2);
    if b_read_result_1 != 0 && b_read_result_2 != false
    {
        // read two bytes from the file i32o buffer_1)) {
        // bail if the file read fails
        if (buffer_1._0_2_ == 0) {
            return;
        }
        process_struct_1000_179c(0xc, in_dx);
        struct_a = (in_dx | b_read_result_2);
        if (struct_a == 0x0) {
            var3 = 0x0;
            struct_a = 0x0;
        } else {
            var3 = process_struct_1008_574a(CONCAT22(in_dx, b_read_result_2));
        }
        u_var9 = (param_1 >> 0x10);
        (struct_a + 10) = var3;
        (struct_a + 0xc) = struct_a;
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
            string_1 = local_118;
            pass1_fn_1008_60e8(string_1);
            local_AX_168.field_0x4 = string_1;
            local_AX_168.field_0x6 = u_var7;
            local_AX_168.two_file_bytes_0x8 = two_file_bytes;
            local_AX_168.four_file_bytes_0xa = four_file_bytes;
            local_AX_168.two_file_bytes_0xe = two_file_bytes_2;
            func_ptr_1 = ((struct_a + 10) + 8);
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

pub unsafe fn read_file_1008_c9d4(ctx: &mut AppContext, param_1: u32, hfile_param_2: &mut HFILE16) {
    let u_var1: u8;
    let b_var2: bool;
    let extraout_var: u32;

    if 1 < ctx.PTR_LOOP_1050_0312 {
        u_var1 = read_file_1008_7cfe(ctx, hfile_param_2);
        if CONCAT31(extraout_var, u_var1) == 0 {
            ctx.g_u16_1050_0310 = 0x6d4;
            return;
        }
        b_var2 = read_file_1008_7dee(hfile_param_2, (param_1 & 0xffff0000 | (param_1 + 0xe)), 4);
        if b_var2 == false {
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
    let pu_var4: Vec<u8>;
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

pub unsafe fn read_file_1008_e70e(ctx: &mut AppContext, param_1: u32, in_file_1: &HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let b_var3: bool;
    let mut u_var4: u16;
    let extraout_var: u32;
    // let in_dx: *mut Struct199;
    let mut u_var5: u16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    if ctx.PTR_LOOP_1050_0312 < 2 {
        return;
    }
    u_var2 = read_file_1008_7cfe(ctx, in_file_1);
    if CONCAT31(extraout_var, u_var2) != 0 {
        b_var3 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_4), 2);
        if (b_var3 != 0) {
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
                b_var3 = read_file_1008_7dee(in_file_1, CONCAT22(u_var5, u_var4 + 4), 4);
                if ((((b_var3 == 0)
                    || (
                    b_var3 = read_file_1008_7dee(
                            in_file_1,
                            (_local_e & 0xffff0000 | (_local_e + 8)),
                            4,
                        ),
                    b_var3 == 0,
                    ))
                    || (
                    b_var3 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_ss, &local_12), 2),
                    b_var3 == 0,
                    ))
                    || ((
                    b_var3 = read_file_1008_7dee(
                            in_file_1,
                            (_local_e & 0xffff0000 | (_local_e + 0xe)),
                            4,
                        ),
                    b_var3 == 0
                            || (
                        b_var3 = read_file_1008_7dee(
                                    in_file_1,
                                    (_local_e & 0xffff0000 | (_local_e + 0x12)),
                                    2,
                                ),
                        b_var3 == 0,
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
    let pcVar4: String;
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
            BVar3 != false
        } {}
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn read_file_1010_0c7c(ctx: &mut AppContext, param_1: u32, in_file_handle_2:&HFILE16) {
    let pp_var1: fn();
    let b_var3: bool;
    let mut u_var4: Struct199 = Struct199::new();
    let b_var5: bool;
    let extraout_var: u32;
    // let in_dx: *mut Struct199;

    let mut i32_var6: u32;
    let mut unaff_ss: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_26: u16;
    let mut local_1a: String = String::new();
    let mut local_16: Vec<u8> = Vec::new();
    let mut buffer_12: Vec<u8> = Vec::new();
    let mut local_8: u16;
    let mut local_6: Vec<u8> = Vec::new();
    let mut local_4: Vec<u8> = Vec::new();

    let u_var2 = read_file_1008_7cfe(ctx, in_file_handle_2);
    // if (CONCAT31(extraout_var, u_var2) == 0) {
    if u_var2 == 0 {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        // CONCAT22(unaff_ss, &local_6)
        let mut b_var3 = read_file_1008_7dee(in_file_handle_2, &mut local_6, 2);
        if b_var3 != false {
            let mut local_8 = 0;
            while (i32_var6 = param_1, local_8 < local_6) {
                let mut u_var4 = local_6.clone();
                process_struct_1000_179c(10, &mut u_var4);
                // let mut local_1a = CONCAT22(ctx.dx_reg, u_var4);
                if (ctx.dx_reg | u_var4) == 0 {
                    // local_16 = 0;
                } else {
                    local_1a = ctx.s_1_1050_389a.clone();
                    (u_var4 + 2) = ctx.PTR_LOOP_1050_1008;
                    local_1a = 0xea8;
                    (u_var4 + 2) = 0x1010;
                    local_16 = local_1a;
                }
                b_var3 = read_file_1008_7dee(in_file_handle_2, &mut buffer_12, 2);
                if (b_var3 == false)
                    || (
                    b_var3 = read_file_1008_7dee(
                        in_file_handle_2,
                        &local_16[6..],
                        4,
                        ),
                    b_var3 == false,
                    )
                {
                    local_1a = local_16.clone();
                    if local_16 != 0 {
                        pp_var1 = local_16;
                        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, &local_16, 1);
                    }
                    // goto LAB_1010_0cb1;
                }
                (local_16 + 4) = buffer_12;
                pp_var1 = ((i32_var6 + 10) + 8);
                (pp_var1)();
                local_8 = local_8 + 1;
                in_dx = ctx.dx_reg;
            }
            b_var3 = read_file_1008_7dee(in_file_handle_2, (param_1 & 0xffff0000 | (i32_var6 + 0xe)), 2);
            if ((b_var3 != 0)
                && (
                b_var3 =
                        read_file_1008_7dee(in_file_handle_2, (param_1 & 0xffff0000 | (i32_var6 + 0x10)), 2),
                b_var3 != 0,
                ))
            {
                local_4 = 0;
                while (local_4 < 10) {
                    b_var3 = read_file_1008_7dee(in_file_handle_2, CONCAT22(unaff_ss, &local_2a), 2);
                    if (b_var3 == 0) {
                        // goto LAB_1010_0cb1;
                    }
                    b_var5 = local_4;
                    if (PTR_LOOP_1050_0312 < 2) {
                        switch_statement_1008_738c(in_file_handle_2, (in_file_handle_2 >> 0x10), local_4);
                        b_var5 = b_var3;
                    }
                    (b_var5 * 8 + 0xe28) = local_2a;
                    local_4 = local_4 + 1;
                    local_26 = b_var5;
                }
                if (2 < PTR_LOOP_1050_0312) {
                    local_4 = 0;
                    while {
                        b_var3 = read_file_1008_7dee(in_file_handle_2, CONCAT22(unaff_ss, &local_2a), 2);
                        if (b_var3 == 0) {}
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

pub fn write_to_file_1010_3fc2(ctx: &mut AppContext, struct_param_1: &mut Struct390, hfile_param_2: &HFILE16) -> bool {
    let b_result: bool;
    let struct_var_1: Struct390;
    let string_var_4: String;
    let u_var1 = write_to_file_1008_7cac(hfile_param_2, 5);
    if u_var1 != false {
        let string_var_2 = struct_param_1.string_buf_ptr_1.clone();
        b_result = write_to_file_1008_7e1c(hfile_param_2, &string_var_2, 2);
        if b_result != false {
            string_var_4 = struct_var_1.string_buf_ptr_2.clone();
            b_result = write_to_file_1008_7e1c(hfile_param_2, &string_var_4, 2);
            if b_result != false {
                string_var_4 = struct_var_1.string_buf_ptr_3.clone();
                b_result =
                    write_to_file_1008_7e1c(hfile_param_2, &string_var_4, 2);
                if b_result != false {
                    return true;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return false;
}

pub unsafe fn read_file_1010_404a(ctx: &mut AppContext, param_1: &mut Struct407, param_2: &HFILE16) {
    // let local_AX_39: Struct407;
    let b_var2: bool;
    let extraout_var: u32;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    let mut u_var1 = read_file_1008_7cfe(ctx, param_2);
    // if CONCAT31(extraout_var, u_var1) == 0 {
    if u_var1 == 0 {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        // local_AX_39 = param_1;
        // local_AX_39 = local_AX_39.field_0x24;
        b_var2 = read_file_1008_7dee(param_2, param_1, 2);
        if b_var2 != false {
            b_var2 = read_file_1008_7dee(param_2, &mut local_4, 2);
            if b_var2 != false {
                // (param_1 & 0xffff0000 | ZEXT24(local_AX_39 + 1))
                b_var2 = read_file_1008_7dee(
                    param_2,
                    param_1,
                    2,
                );
                if b_var2 != false {
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
    in_file_1: &HFILE16) -> bool {
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
        i_var3 = write_to_file_1008_7c2a(ctx,in_file_1, u_var1, (u_var1 >> 0x10));
        if (i_var3 != 0) {
            u_var1 = ctx.bx_reg.field_0x6c;
            i_var3 = write_to_file_1008_7c2a(ctx,in_file_1, u_var1, (u_var1 >> 0x10));
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

pub unsafe fn read_file_1010_5e56(ctx: &mut AppContext, param_1: u32, hfile_param_2: &HFILE16) {
    let mut u_var1: u8;
    let mut pu_var2: String = String::new();
    let mut b_var3: bool;
    let mut extraout_var: u32;
    let mut in_dx: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_404: u16;
    let mut local_402: String = String::new();

    let mut u_var1 = read_file_1008_7cfe(ctx, hfile_param_2);
    if CONCAT31(extraout_var, u_var1) == 0 {
        ctx.g_u16_1050_0310 = 0x6d4;
    } else {
        pu_var2 = local_402;
        read_file_into_str_1008_7c6e(hfile_param_2, CONCAT22(unaff_ss, pu_var2));
        if (pu_var2 != 0x0) {
            pu_var2 = local_402;
            pass1_fn_1008_60e8(pu_var2);
            u_var5 = (param_1 >> 0x10);
            i_var4 = param_1;
            (i_var4 + 0x68) = pu_var2;
            (i_var4 + 0x6a) = in_dx;
            pu_var2 = local_402;
            read_file_into_str_1008_7c6e(hfile_param_2, CONCAT22(unaff_ss, pu_var2));
            if (pu_var2 != 0x0) {
                pu_var2 = local_402;
                pass1_fn_1008_60e8(pu_var2);
                (i_var4 + 0x6c) = pu_var2;
                (i_var4 + 0x6e) = in_dx;
                b_var3 = read_file_1008_7dee(hfile_param_2, CONCAT22(unaff_ss, &local_404), 2);
                if (b_var3 != 0) {
                    u16_1050_13ae = local_404;
                    if (PTR_LOOP_1050_0312 < 2) {
                        return;
                    }
                    b_var3 =
                        read_file_1008_7dee(hfile_param_2, (param_1 & 0xffff0000 | (i_var4 + 0x82)), 2);
                    if (b_var3 != 0) {
                        return;
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_file_1010_6372(param_1: u32, hfile_param2: &HFILE16) {
    let u_var1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_8: u32;

    u_var1 = write_to_file_1008_7cac(hfile_param2, 7);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        local_10 = (i_var3 + 10);
        BVar2 = write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_10), 4);
        if (BVar2 != 0) {
            local_8 = (i_var3 + 0xe);
            BVar2 = write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_8), 4);
            if (BVar2 != 0) {
                local_8 = (i_var3 + 0x12);
                BVar2 = write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_8), 4);
                if (BVar2 != 0) {
                    local_8 = (i_var3 + 0x16);
                    BVar2 = write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_8), 4);
                    if (BVar2 != 0) {
                        local_8 = (i_var3 + 0x1a);
                        BVar2 = write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_8), 4);
                        if (BVar2 != 0) {
                            local_8 = (i_var3 + 0x1e);
                            BVar2 =
                                write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_8), 4);
                            if (BVar2 != 0) {
                                local_8 = (i_var3 + 0x22);
                                BVar2 = write_to_file_1008_7e1c(
                                    hfile_param2,
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

pub unsafe fn read_file_1010_648a(ctx: &mut AppContext, param_1: u32, param_2: &HFILE16) {
    let mut i_var2: i32;
    let b_var3: bool;
    let extraout_var: u32;

    let u_var1 = read_file_1008_7cfe(ctx, param_2);
    if (CONCAT31(extraout_var, u_var1) != 0) {
        i_var2 = param_1;
        b_var3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 10)), 4);
        if (b_var3 != 0) {
            b_var3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 0xe)), 4);
            if (b_var3 != 0) {
                b_var3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 0x12)), 4);
                if (b_var3 != 0) {
                    b_var3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (i_var2 + 0x16)), 4);
                    if (b_var3 != 0) {
                        b_var3 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (i_var2 + 0x1a)),
                            4,
                        );
                        if (b_var3 != 0) {
                            b_var3 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | (i_var2 + 0x1e)),
                                4,
                            );
                            if (b_var3 != 0) {
                                b_var3 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | (i_var2 + 0x22)),
                                    4,
                                );
                                if (b_var3 != 0) {
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

pub fn write_to_file_1010_6846(param_1: u32, param_2: &HFILE16) {
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

pub unsafe fn read_file_1010_68c6(param_1: u32, param_2: &HFILE16) {
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

    u_var2 = read_file_1008_7cfe(ctx,param_2);
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
        u_var2 = error_check_1000_17ce(ctx,local_a);
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
        error_check_1000_17ce(ctx,local_12);
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

pub unsafe fn write_to_file_1010_9900(ctx: &mut AppContext, param_1: u16, param_2: &HFILE16) -> bool {
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

pub unsafe fn read_file_1010_9b72(param_1: u32, hfile_param_2: &HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let struct_a_2: Address<Struct199>;
    let var3: Address<Struct199>;
    let b_result: bool;
    // let local_AX_165: *mut Struct473;
    let mut struct_b: Address<Struct1174>;
    let mut u_var4: i32;
    // let local_AX_571: *mut Struct472;
    let mut u_var5: u16;
    let extraout_var: u32;
    let mut struct_a_1: Address<Struct199>;
    let mut var6: Address<Struct199>;

    // let ctx.dx_reg: *mut Struct199;
    // let ctx.dx_reg: *mut Struct199;
    // let local_bx_60: *mut Struct471;
    let pu_var7: *mut u32;
    let local_es_60: Vec<u8>;
    let local_es_165: Vec<u8>;
    let mut u_var8: u16;
    let string_a_1: String;
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
    let string_a_2: String;
    let mut local_4: u16;

    let u_var2 = read_file_1008_7cfe(ctx, hfile_param_2);
    if CONCAT31(extraout_var, u_var2) != 0 {
        struct_a_2 = read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_4), 2);
        if struct_a_2 != 0x0 {
            local_bx_60 = param_1;
            local_es_60 = (param_1 >> 0x10);
            u_var9 = hfile_param_2;
            u_var10 = (hfile_param_2 >> 0x10);
            if local_4 != 0 {
                if local_bx_60.field_0xa == 0 {
                    process_struct_1000_179c(0xc, struct_a_1);
                    _local_16 = CONCAT22(struct_a_1, struct_a_2);
                    var6 = (struct_a_1 | struct_a_2);
                    if var6 == 0x0 {
                        local_bx_60.field_0xa = 0;
                        struct_a_1 = var6;
                    } else {
                        var3 = process_struct_1008_574a(CONCAT22(struct_a_1, struct_a_2));
                        local_bx_60.field_0xa = var3;
                        local_bx_60.field_0xc = var6;
                        struct_a_1 = var6;
                    }
                }
                local_12 = 0;
                while local_12 < local_4 {
                    struct_b = local_4;
                    process_struct_1000_179c(8, struct_a_1);
                    _local_16 = CONCAT22(struct_a_1, struct_b);
                    if (struct_a_1 | struct_b) == 0 {
                        local_e = 0x0;
                    } else {
                        _local_16.field_0x0 = ctx.s_1_1050_389a;
                        struct_b.base_field_0x2 = &ctx.PTR_LOOP_1050_1008;
                        _local_16.field_0x0 = 0xa1c4;
                        struct_b.base_field_0x2 = 0x1010;
                        local_e = _local_16;
                    }
                    b_result = read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &string_a_2), 2);
                    if b_result == 0 {
                        // LAB_1010_9c32:
                        _local_16 = local_e;
                        if local_e == 0x0 {}
                        // goto LAB_1010_9ba1;
                        u_var8 = (local_e >> 0x10);
                        pu_var7 = local_e;
                        // goto LAB_1010_9e9e;
                    }
                    b_result =
                        read_file_1008_7dee(hfile_param_2, (local_e & 0xffff0000 | (local_e + 6)), 2);
                    if b_result == 0 {}
                    // goto LAB_1010_9c32;
                    switch_statement_1008_73ea(u_var9, u_var10, string_a_2);
                    (local_e + 4) = b_result;
                    pp_var1 = (&local_bx_60.field_0xa + 4);
                    (**pp_var1)();
                    local_12 = local_12 + 1;
                    struct_a_1 = ctx.dx_reg;
                }
            }
            u_var4 = read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_2a), 2);
            if u_var4 != 0 {
                if local_2a != 0 {
                    if &local_bx_60.field_0xe == 0 {
                        process_struct_1000_179c(0xc, struct_a_1);
                        local_2e = CONCAT22(struct_a_1, u_var4);
                        var6 = (struct_a_1 | u_var4);
                        if var6 == 0x0 {
                            local_bx_60.field_0xe = 0;
                            struct_a_1 = var6;
                        } else {
                            var3 = process_struct_1008_574a(CONCAT22(struct_a_1, u_var4));
                            local_bx_60.field_0xe = var3;
                            local_bx_60.field_0x10 = var6;
                            struct_a_1 = var6;
                        }
                    }
                    local_22 = 0;
                    while local_22 < local_2a {
                        local_AX_571 = local_2a;
                        process_struct_1000_179c(8, struct_a_1);
                        _local_16 = CONCAT22(struct_a_1, local_AX_571);
                        if (struct_a_1 | local_AX_571) == 0 {
                            local_1e = 0;
                        } else {
                            _local_16.field_0x0 = ctx.s_1_1050_389a;
                            local_AX_571.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                            _local_16.field_0x0 = 0xa1c4;
                            local_AX_571.field_0x2 = 0x1010;
                            local_1e = _local_16;
                        }
                        b_result = read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_1a), 2);
                        if b_result == 0 {
                            // LAB_1010_9d5c:
                            _local_16 = local_1e;
                            if local_1e == 0 {}
                            // goto LAB_1010_9ba1;
                            u_var8 = (local_1e >> 0x10);
                            pu_var7 = local_1e;
                            // goto LAB_1010_9e9e;
                        }
                        b_result = read_file_1008_7dee(
                            hfile_param_2,
                            (local_1e & 0xffff0000 | (local_1e + 6)),
                            2,
                        );
                        if b_result == 0 {}
                        // goto LAB_1010_9d5c;
                        switch_statement_1008_73ea(u_var9, u_var10, local_1a);
                        (local_1e + 4) = b_result;
                        pp_var1 = (&local_bx_60.field_0xe + 4);
                        (**pp_var1)();
                        local_22 = local_22 + 1;
                        struct_a_1 = ctx.dx_reg;
                    }
                }
                u_var4 = read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_36), 2);
                if u_var4 != 0 {
                    if local_36 != 0 {
                        var6 = struct_a_1;
                        if &local_bx_60.field_0x12 == 0 {
                            process_struct_1000_179c(0xc, struct_a_1);
                            _local_16 = CONCAT22(struct_a_1, u_var4);
                            var6 = (struct_a_1 | u_var4);
                            if (var6 == 0x0) {
                                local_bx_60.field_0x12 = 0;
                            } else {
                                var3 = process_struct_1008_574a(CONCAT22(struct_a_1, u_var4));
                                local_bx_60.field_0x12 = var3;
                                local_bx_60.field_0x14 = var6;
                            }
                        }
                        local_2a = 0;
                        while local_2a < local_36 {
                            u_var5 = local_36;
                            process_struct_1000_179c(8, var6);
                            local_2e = CONCAT22(var6, u_var5);
                            if (var6 | u_var5) == 0 {
                                paStack38 = 0x0;
                            } else {
                                local_2e = ctx.s_1_1050_389a;
                                (u_var5 + 2) = ctx.PTR_LOOP_1050_1008;
                                local_2e = 0xa1c4;
                                (u_var5 + 2) = 0x1010;
                                paStack38 = local_2e;
                            }
                            b_result =
                                read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_22), 2);
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
                                hfile_param_2,
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
                            var6 = ctx.dx_reg;
                        }
                    }
                    b_result = read_file_1008_7dee(
                        hfile_param_2,
                        (param_1 & 0xffff0000 | &local_bx_60.field_0x1a),
                        2,
                    );
                    if (b_result != 0) {
                        b_result = read_file_1008_7dee(
                            hfile_param_2,
                            (param_1 & 0xffff0000 | &local_bx_60.field_0x1c),
                            2,
                        );
                        if (b_result != 0) {
                            b_result = read_file_1008_7dee(
                                hfile_param_2,
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

pub unsafe fn write_to_file_1010_ed58(param_1: u32, in_file: &HFILE16) {
    let pu_var1: *mut u16;
    let u_var2: u8;
    let BVar3: bool;
    let extraout_var: u32;
    // ppu_var4: *mut Vec<u8>;
    let local_bx_30: *mut Struct506;
    let local_es_30: Vec<u8>;
    let string_base_a: Vec<u8>;
    let local_22: Vec<u8>;
    let mut uStack30: u16;
    let mut string_offset_a: u32;
    let mut string_offset_b: u32;
    let mut local_4: u16;
    let temp_5fb1d7bd90: Vec<u8>;

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

pub unsafe fn read_file_1018_0000(param_1: u32, param_2: &HFILE16) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let u_var3: u8;
    let mut i_var4: i32;
    let b_var5: bool;
    let puVar6: Vec<u8>;
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
    u_var3 = read_file_1008_7cfe(ctx,param_2);
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
                    read_file_1008_7bc8(ctx,param_2, (param_2 >> 0x10), puVar6, unaff_ss);
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

pub unsafe fn write_to_file_1038_7b20(param_1: *mut u32, param_2: &HFILE16) -> u16 {
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
                i_var4 = write_to_file_1038_75ca(ctx, lVar6, param_2, (param_2 >> 0x10));
                i_var4 != 0
            } {}
        }
    }
    return 0;
}

pub unsafe fn read_from_file_1038_7c02(param_1: *mut Vec<u8>, file_b: &HFILE16) -> u16 {
    let u8_a: u8;
    // let extraout_AH: u8;
    let bool_a: bool;
    let mut u_var1: u16;
    let mut i_var2: i32;
    // let in_dx: *mut Struct199;

    let mut u_var3: i32;
    // let ctx.dx_reg: *mut Struct199;
    // let mut unaff_ss: u16;
    let ph_var4: &HFILE16;
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
    u8_a = read_file_1008_7cfe(ctx,file_b);
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

pub unsafe fn write_to_file_1038_75ca(ctx: &mut AppContext, param_1: &mut Struct1167, file_handle: &HFILE16) {
    let b_var1: bool;
    let pu_var2: Vec<u8>;
    let mut u_var3: u16;
    let mut base: u16;
    let mut offset_2: u32;
    let mut offset_1: u32;

    // u_var3 = (param_1 >> 0x10);
    // local_bx_4 = param_1;
    write_to_file_1008_79f0(ctx,file_handle, param_1.field_0x4);
    if ctx.ax_reg != 0 {
        offset_2 = param_1.field_0x8;
        b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_2), 4);
        if b_var1 != 0 {
            write_to_file_1008_7a22(file_handle, param_1.field_0xe);
            if b_var1 != 0 {
                offset_1 = CONCAT22(offset_1._2_2_, param_1.field_0xc);
                b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                if b_var1 != 0 {
                    offset_1 = offset_1 & 0xffff0000 | param_1.field_0x12;
                    b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                    if (b_var1 != 0) {
                        offset_1 = offset_1 & 0xffff0000 | param_1.field_0x14;
                        b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                        if (b_var1 != 0) {
                            offset_1 = param_1.field_0x16;
                            b_var1 =
                                write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 4);
                            if (b_var1 != 0) {
                                pu_var2 = (param_1 & 0xffff0000 | &param_1.field_0x1a);
                                write_file_1008_7b4c(file_handle, pu_var2);
                                if (pu_var2 != 0) {
                                    offset_1 = param_1.field_0x20;
                                    b_var1 = write_to_file_1008_7e1c(
                                        file_handle,
                                        CONCAT22(base, &offset_1),
                                        4,
                                    );
                                    if (b_var1 != 0) {
                                        offset_1 = offset_1 & 0xffff0000 | param_1.field_0x24;
                                        b_var1 = write_to_file_1008_7e1c(
                                            file_handle,
                                            CONCAT22(base, &offset_1),
                                            2,
                                        );
                                        if (b_var1 != 0) {
                                            offset_1 =
                                                offset_1 & 0xffff0000 | param_1.field_0x26;
                                            b_var1 = write_to_file_1008_7e1c(
                                                file_handle,
                                                CONCAT22(base, &offset_1),
                                                2,
                                            );
                                            if (b_var1 != 0) {
                                                offset_1 =
                                                    offset_1 & 0xffff0000 | param_1.field_0x28;
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

pub unsafe fn read_from_file_1038_774e(param_1: u32, param_2: &HFILE16) {
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
        read_file_1008_77cc(ctx,u_var5, u_var6, i_var3, u_var1);
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
            read_file_1008_7bc8(ctx,u_var5, u_var6, i_var3, u_var1);
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

pub unsafe fn read_from_file_1038_6118(param_1: &mut Struct933, hfile_param_2: &HFILE16) {
    let b_var1: bool;
    let u_var2: u8;
    let pu_var3: *mut u32;
    let b_var4: bool;
    let local_AX_307: *mut Struct1142;
    let pu_var5: Vec<u8>;
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

    u_var2 = file2::pass1_1030_1730(param_1, hfile_param_2);
    if (CONCAT31(extraout_var, u_var2) == 0) {
        return;
    }
    local_6 = 0;
    pu_var3 = &local_6;
    // u_var8 = (hfile_param_2 >> 0x10);
    read_file_1008_7548(ctx, hfile_param_2, pu_var3, unaff_ss);
    if (pu_var3 != 0x0) {
        u_var7 = (param_1 >> 0x10);
        local_bx_68 = param_1;
        local_bx_68.field_0xc = local_6;
        struct_a = ctx.dx_reg;
        b_var4 = read_file_1008_7dee(hfile_param_2, (param_1 & 0xffff0000 | &local_bx_68.field_0x10), 4);
        if (((((b_var4 != 0)
            && (
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x18),
                    2,
                ),
                b_var4 != 0,
            ))
            && (
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x1a),
                    2,
                ),
                b_var4 != 0,
            ))
            && ((
            b_var4 = read_file_1008_7dee(hfile_param_2, CONCAT22(unaff_ss, &local_8), 2),
            b_var4 != 0
                    && (
                        b_var4 = read_file_1008_7dee(
                            hfile_param_2,
                            (param_1 & 0xffff0000 | &local_bx_68.field_0x1e),
                            4,
                        ),
                        b_var4 != 0,
                    ),
            )))
            && (
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x22),
                    2,
                ),
                b_var4 != 0,
            ))
        {
            local_bx_68.field_0x1c = local_8;
            b_var4 =
                read_file_1008_7dee(hfile_param_2, (param_1 & 0xffff0000 | &local_bx_68.field_0x24), 2);
            if ((b_var4 != 0)
                && (
                    local_AX_307 = read_file_1008_7dee(
                        hfile_param_2,
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
                        hfile_param_2,
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
                        set_param_3_with_switch_1008_72bc(hfile_param_2, u_var8, local_412);
                        b_var1 = (local_AX_307 + local_412 * 4);
                        u_var9 = (local_AX_307 + local_412 * 4 + 2);
                        (&local_bx_68.field_0x14e + b_var4 * 4) = b_var1;
                        (&local_bx_68.field_0x150 + b_var4 * 4) = u_var9;
                        local_412 = &local_412.field_0x1;
                        b_var4 = b_var1;
                        local_412 < 0x15
                    } {}
                    b_var4 = read_file_1008_7dee(hfile_param_2, _local_416, 0x54);
                    if (b_var4 == 0) {}
                    // goto LAB_1038_626a;
                    local_412 = 0x0;
                    while {
                        set_param_3_with_switch_1008_72bc(hfile_param_2, u_var8, local_412);
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
                        hfile_param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x14e),
                        0x54,
                    );
                    if (b_var4 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                    b_var4 = read_file_1008_7dee(
                        hfile_param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x1a2),
                        0x54,
                    );
                    if (b_var4 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                }
                file2::read_from_file_1030_33f0(local_bx_68.field_0x1f6, hfile_param_2);
                pu_var5 = local_408;
                read_file_into_str_1008_7c6e(hfile_param_2, CONCAT22(unaff_ss, pu_var5));
                if (pu_var5 != 0x0) {
                    pu_var5 = local_408;
                    pass1_fn_1008_60e8(pu_var5);
                    local_bx_68.field_0x1fa = pu_var5;
                    local_bx_68.field_0x1fc = struct_a;
                    b_var4 = read_file_1008_7dee(
                        hfile_param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x1fe),
                        2,
                    );
                    if (((((b_var4 != 0)
                        && (
                            b_var4 = read_file_1008_7dee(
                                hfile_param_2,
                                (param_1 & 0xffff0000 | CONCAT11((param_1 >> 8) + 0x2, param_1)),
                                4,
                            ),
                            b_var4 != 0,
                        ))
                        && (
                            b_var4 = read_file_1008_7dee(
                                hfile_param_2,
                                (param_1 & 0xffff0000 | &local_bx_68.field_0x204),
                                2,
                            ),
                            b_var4 != 0,
                        ))
                        && ((
                            b_var4 = read_file_1008_7dee(
                                hfile_param_2,
                                (param_1 & 0xffff0000 | &local_bx_68.field_0x206),
                                2,
                            ),
                            b_var4 != 0
                                && (
                                    b_var4 = read_file_1008_7dee(
                                        hfile_param_2,
                                        (param_1 & 0xffff0000 | &local_bx_68.field_0x208),
                                        2,
                                    ),
                                    b_var4 != 0,
                                ),
                        ) && ((
                            b_var4 = read_file_1008_7dee(
                                hfile_param_2,
                                (param_1 & 0xffff0000 | &local_bx_68.field_0x20a),
                                2,
                            ),
                            b_var4 != 0
                                && ((
                                    b_var4 = read_file_1008_7dee(
                                        hfile_param_2,
                                        (param_1 & 0xffff0000 | &local_bx_68.field_0x20c),
                                        2,
                                    ),
                                    b_var4 != 0
                                        && (
                                            b_var4 = read_file_1008_7dee(
                                                hfile_param_2,
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
                                    hfile_param_2,
                                    (param_1 & 0xffff0000 | &local_bx_68.field_0x214),
                                    2,
                                ),
                                b_var4 != 0
                                    && (
                                        b_var4 = read_file_1008_7dee(
                                            hfile_param_2,
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

pub fn write_to_file_1038_5e16(ctx: &mut AppContext, param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;
    let pu_var2: *mut u32;
    let local_bx_28: *mut Struct1140;
    let mut u_var3: u16;
    let mut string_base: u16;
    let mut offset_1: u32;
    let mut offset_2: u32;
    let mut local_6: u32;

    file2::write_to_file_1030_16d6(param_1, param_2);
    if (ctx.ax_reg != 0) {
        u_var3 = (param_1 >> 0x10);
        local_bx_28 = param_1;
        pu_var2 = local_bx_28.field_0xc;
        local_6 = pu_var2;
        write_to_file_1008_7898(ctx,param_2, pu_var2);
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
                                                    file2::write_to_file_1030_32e4(
                                                        local_bx_28.field_0x1f6,
                                                        param_2,
                                                    );
                                                    write_to_file_1008_7c2a(
                                                        ctx,param_2,
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

pub fn read_from_file_1030_dec4(param_1: u32, param_2: &HFILE16) {

    let b_var1: bool;

    file2::read_file_fn_1028_b81a(param_1, param_2);
    if (((ctx.ax_reg != 0) && (1 < PTR_LOOP_1050_0312))
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

pub fn write_to_file_1030_de7c(param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_10: u32;

    file2::write_to_file_1028_b5ec(param_1, param_2);
    if (ctx.ax_reg != 0) {
        local_10 = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_10), 4);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}

pub fn write_to_file_1030_d61c(param_1: &mut FileObject, param_2: &HFILE16) {

    let b_success: bool;
    let mut i_var1: i32;
    let mut local_es_61: u16;
    let mut string_base: u16;
    let mut string_off_4: u32;
    let mut string_off_3: u16;
    let mut string_off_2: u16;
    let mut string_off_1: u32;
    let mut local_4: u16;

    file2::write_to_file_1028_b5ec(param_1, param_2);
    if (ctx.ax_reg != 0) {
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

pub fn pass1_1030_d72e(param_1: u32, param_2: &HFILE16) {

    let b_var1: bool;
    let mut i_var2: i32;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    file2::read_file_fn_1028_b81a(param_1, param_2);
    if (ctx.ax_reg == 0) {
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

pub fn pass1_1030_c84e(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = file2::write_to_file_1028_b5ec(param_1, param_2);
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

pub fn read_from_file_1030_c894(param_1: u32, param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    b_var1 = file2::read_file_fn_1028_b81a(param_1, param_2);
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

pub fn write_to_file_1030_c230(param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_8: u16;

    file2::write_to_file_1028_b5ec(param_1, param_2);
    if (ctx.ax_reg != 0) {
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

pub fn read_from_file_1030_c29c(param_1: u32, param_2: &HFILE16) {

    let b_var1: bool;

    file2::read_file_fn_1028_b81a(param_1, param_2);
    if (ctx.ax_reg != 0) {
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

pub unsafe fn write_to_file_1030_b768(param_1: *mut Struct961, param_2: &HFILE16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let pu_var3: Vec<u8>;
    let pu_var4: Vec<u8>;

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
                    write_to_file_1038_75ca(ctx, pu_var3, ctx.dx_reg, param_2);
                    pu_var3 != 0x0
                } {}
                return;
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn pass1_1030_b836(struct_b: *mut Struct962, file_a: &HFILE16) {
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
    let pHVar7: &HFILE16;
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
        read_file_1008_7bc8(ctx,file_a, u_var4);
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

pub unsafe fn write_to_file_1030_7418(struct_param_1: &mut FileObject, hfile_param_2: &HFILE16) {
    let mut u_var1: u32;

    let struct_b: &mut FileObject;
    let b_success: bool;
    let pc_var2: String;


    let mut local_es_52: u16;
    let char_ptr_a: String;
    let mut local_3e: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_2a: u16;
    let mut char_array_14_a: [u8; 14];
    let char_ptr_a_4: String;
    let mut local_14: u16;
    let mut local_12: u16;
    let char_ptr_a_2: String;
    let char_ptr_a_5: String;
    let char_ptr_a_3: String;
    let pu_var3: Vec<u8>;

    file2::write_to_file_1030_16d6(struct_param_1, hfile_param_2);
    if ctx.ax_reg == 0 {
        return;
    }
    struct_b = struct_param_1;
    struct_b = struct_b.field_0xc;
    pu_var3 = (struct_param_1 & 0xffff0000 | ZEXT24(struct_b));
    write_file_1008_7b4c(hfile_param_2, pu_var3);
    if (pu_var3 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    local_es_52 = (struct_param_1 >> 0x10);
    char_ptr_a_2 = struct_b.field_0x12;
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_3 = *&struct_b.field_0x14;
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_4 = *&struct_b.field_0x16;
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_4), 4);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7954(ctx, hfile_param_2, &struct_b.field_0x1e);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(hfile_param_2, (&struct_b.field_0x20 + 2));
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(hfile_param_2, (&struct_b.field_0x24 + 2));
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_5 = *(&struct_b.field_0x28 + 2);
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_5), 4);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_2 = *&struct_b[1].field_0x6;
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_2 = *&struct_b[1].field_0x8;
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_79f0(ctx, hfile_param_2, &struct_b[1].field_0xa);
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
    b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
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
            b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &local_2a), 2);
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
                b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_4), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                _local_14 = _local_14 & 0xffff0000 | *(pc_var2 + 6);
                b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &local_14), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                char_ptr_a_2 = *(pc_var2 + 8);
                b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
                if (b_success == 0) {
                    break;
                }
                char_ptr_a_2 = *(pc_var2 + 10);
                b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
                char_ptr_a_3 = *(pc_var2 + 0xc);
                b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
                if (b_success == 0) {
                    ctx.g_u16_1050_0310 = 0x6d0;
                    return;
                }
            }
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(pc_var2 + 4);
        b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 6);
        b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            break;
        }
        char_ptr_a_3 = *(_local_14 + 8);
        b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 10);
        b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 0xc);
        b_success = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn read_from_file_1030_778c(struct_b: Struct933, file_a: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let local_AX_32: Struct933;
    let BVar3: bool;
    let paVar4: *mut Struct493;
    let local_AX_482: Struct935;
    let mut u_var5: u16;
    let local_AX_662: Struct934;
    let local_AX_1036: Struct936;
    let extraout_var: u32;

    let mut u_var8: i32;

    let paVar9: *mut Struct199;

    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let base_ptr: Vec<u8>;
    let mut local_56: u16;
    let mut local_52: u16;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_42: u16;
    let mut local_3e: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let local_2c: Vec<u8>;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_e: u16;
    let offset_ptr: Vec<u8>;
    let mut u_var6: u32;
    let pu_var7: Vec<u8>;
    let fn_ptr_a: fn();

    u_var2 = file2::pass1_1030_1730(struct_b, file_a);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        local_AX_32 = struct_b;
        local_AX_32 = &local_AX_32.field_0xc;
        u_var6 = struct_b & 0xffff0000 | ZEXT24(local_AX_32);
        read_file_1008_7bc8(ctx,file_a, u_var6);
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
                    read_file_1008_76e4(ctx,file_a, u_var6);
                    if (u_var6 != 0) {
                        u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x22;
                        read_file_1008_77cc(ctx,file_a, u_var6);
                        if (u_var6 != 0) {
                            u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x26;
                            read_file_1008_77cc(ctx,file_a, u_var6);
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
                                            if local_AX_32.field_0x3a == 0 {
                                                u_var5 = local_2e;
                                                process_struct_1000_179c(0xc, paVar9);
                                                _local_32 = CONCAT22(paVar9, u_var5);
                                                u_var8 = paVar9 | u_var5;
                                                if u_var8 == 0 {
                                                    local_AX_32.field_0x3a = 0;
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
