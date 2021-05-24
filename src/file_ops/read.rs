use crate::app_context::AppContext;
use crate::err_ops::error_check_1000_17ce;
use crate::file_ops::{close, file2, misc};
use crate::func_ptr_funcs::call_fn_ptr_1000_5586;
use crate::mem_funcs::Address;
use crate::mem_funcs::alloc_mem::{alloc_mem_1000_07fc, alloc_mem_1000_0a48, alloc_mem_1000_1708};
use crate::mem_funcs::mem_ops_1::StructuredData;
use crate::other_funcs::{
    modify_list_1008_3f62, set_param_3_with_switch_1008_72bc, switch_statement_1008_738c,
    switch_statement_1008_73ea, zero_list_1008_3e38,
};
use crate::pass::pass13_funcs::pass1_1008_b0bc;
use crate::pass::pass14_funcs::{pass1_1008_3e76, pass1_1008_4b8e, pass1_1008_7056, pass1_1008_766e, pass1_fn_1008_60e8};
use crate::pass::pass15_funcs::{pass1_1020_a43e, pass1_1020_b97e, pass1_1020_ba3e, pass1_1020_bb8a, pass1_1020_c444};
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_4782};
use crate::pass::pass20_funcs::{pass1_1018_04a4, pass1_1018_04ca};
use crate::pass::pass4_funcs::{pass1_1028_e0bc, pass1_1028_e100, pass1_1028_e1ec};
use crate::pass::pass5_funcs::pass1_1030_84ae;
use crate::pass::pass8_funcs::pass1_1010_82f8;
use crate::string_ops::misc::{get_string_index_1000_3da4, process_string_1008_7e4a, string_fn_1000_3f9c};
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_4c98, process_struct_1008_574a, process_struct_1008_dcdc, struct_fn_1000_160a};
use crate::struct_ops::struct_ops_2::pass1_1038_6520;
use crate::structs::prog_structs_10::Struct933;
use crate::structs::prog_structs_11::Struct934;
use crate::structs::prog_structs_16::{Struct1143, Struct493};
use crate::structs::prog_structs_17::Struct407;
use crate::structs::prog_structs_2::{Struct131, Struct199, Struct7};
use crate::structs::prog_structs_20::Struct120;
use crate::structs::prog_structs_23::{Struct1168, Struct935};
use crate::structs::prog_structs_24::{Struct2111, Struct8};
use crate::structs::prog_structs_26::{Struct1141, Struct1142, Struct121, Struct866, Struct874};
use crate::structs::prog_structs_28::{FileObject, Struct772, Struct936};
use crate::structs::prog_structs_29::Struct732;
use crate::structs::prog_structs_5::Struct1174;
use crate::structs::prog_structs_6::Struct473;
use crate::sys_ops::dos_ops::dos3_call_1000_51aa;
use crate::typedefs::HFILE16;
use crate::util::{CONCAT11, CONCAT22, CONCAT31, ZEXT24};
use crate::winapi::{_hread, _lclose16, _llseek16, _lopen16};

/*
Referenced in drawing function
 */
pub unsafe fn read_from_file_1008_49e8(
    ctx: &mut AppContext,
    struct_param_1: &mut Struct7,
    param_3: u16,
) {
    let struct_var1: &mut Struct131;
    let mut u_var2: u32;
    let mut file_handle: HFILE16;
    let mut usize_var3: StructuredData;
    let mut struct_var4: &mut StructuredData;
    let struct_var5: &mut StructuredData;
    let mut u16_var6: &mut Struct131;
    let mut u32_var7: &mut StructuredData;
    let mut struct_var8: Struct199 = Struct199::new();
    let local_bx_4: &mut Struct8;
    let mut unaff_ss: u16;
    let mut bytes_read: u32;
    let new_offset: u32;
    let mut in_stack_00000006: i32;
    let mut local_1c: u16;
    let mut local_1a: u16 = 0;
    let mut local_18: StructuredData = StructuredData::new();
    let mut local_16: u32 = 0;
    let mut local_a: u32;
    let mut local_8: &mut Struct199;
    let mut read_count: u32;
    let mut u_var7: u32;
    let mut offset: u16;

    if struct_param_1.file_path != 0x0 {
        if struct_param_1.u16_field_0x1e != 0 {
            return;
        }
        if struct_param_1.file == 0xffff {
            file_handle = _lopen16(struct_param_1.file_path, 0);
            struct_param_1.file = file_handle;
            if file_handle == 0xffff {
                return;
            }
        }
        let mut read_count: u32 = 0;
        bytes_read = _hread(struct_param_1.file, &mut local_18.buffer, 0xe);
        if bytes_read == 0xe {
            // read_count = local_16;
            if local_18 == ctx.PTR_LOOP_1050_4d42 {
                new_offset = _llseek16(struct_param_1.file, 0, 0);
                usize_var3 = StructuredData::new();
                usize_var3.full_address = new_offset;
                local_1c = ctx.g_struct_ptr_1;
                // local_1a = (ctx.g_struct_ptr_1  >> 0x10);
                alloc_mem_1000_0a48(1, read_count, local_1c, local_1a);
                struct_param_1.pv_buffer_0x1a = usize_var3;
                struct_param_1.u16_field_0x1c = ctx.dx_reg;
                if (ctx.dx_reg | struct_param_1.pv_buffer_0x1a.full_address) == 0 {
                    return;
                }
                // WARNING: Load size is inaccurate
                // new_offset = _hread(read_count, struct_param_1.pv_buffer_0x1a, struct_param_1.file);
                new_offset = _hread(
                    struct_param_1.file,
                    &mut struct_param_1.pv_buffer_0x1a.buffer,
                    read_count,
                );
                // struct_a = (lVar8  >> 0x10);
                local_a = new_offset;
                // local_18 = struct_param_1.file;
                // local_8 = struct_var8;
                _lclose16(struct_param_1.file);
                struct_param_1.file = 0xffff;
                struct_param_1.u16_field_0x1e = 1;
                // WARNING: Load size is inaccurate
                struct_param_1.pv_field_0xe = struct_param_1.pv_buffer_0x1a.clone();
                u32_var7 = &mut struct_param_1.pv_buffer_0x1a;
                struct_var4 = u32_var7;
                u32_var7 = u32_var7 & 0xffff0000;
                struct_param_1.pv_field_0x12 = (u32_var7 | struct_var4 + 0xe);
                u32_var7 = u32_var7 | struct_var4 + 0x436;
                struct_param_1.u32_field_0x16 = u32_var7.clone();
                // local_16 = CONCAT22(local_16, 0x14);
                // local_18 = offset;
                process_struct_1000_179c(ctx, 0x14, &mut struct_var8);
                struct_var5 = u32_var7;
                if (struct_var8 | struct_var5) == 0 {
                    struct_param_1.func_ptr_0x4 = 0;
                } else {
                    local_16 = CONCAT22(local_16, 0x100);
                    struct_var1 = struct_param_1.pv_field_0x12;
                    u16_var6 = struct_var1;
                    u16_var6 = u16_var6 + 0x28;
                    u_var2 = struct_var1 & 0xffff0000;
                    u_var7 = u_var2 | u16_var6;
                    // local_18 = (u_var2  >> 0x10);
                    process_struct_1008_4c98(
                        ctx,
                        (u32_var7 & 0xffff | u16_var6 << 0x10),
                        u_var7,
                        0x100,
                    );
                    struct_var5.full_address = u_var7;
                    struct_param_1.func_ptr_0x4 = struct_var5;
                    struct_param_1.i_field_4 = ctx.dx_reg;
                }
                if struct_param_1.pv_field_0x22 != 0 {
                    local_16 = local_16 & 0xffff0000 | param_3;
                    local_18.buffer = struct_param_1.into();
                    let mut var_51: Struct131 = Struct131::from(&struct_var5.buffer);
                    pass1_1008_4b8e(struct_param_1, &mut var_51);
                    return;
                }
                return;
            }
        }
        _lclose16(struct_param_1.file);
        struct_param_1.file = 0xffff;
    }
    return;
}

/*
Only one read reference
 */
pub fn read_from_file_1008_7146(ctx: &mut AppContext, struct_param_1: &mut FileObject) -> bool {
    let mut file: u16;
    let b_var1: bool;
    // let local_file_obj: &mut  file_object;
    let mut u_var2: u16;

    if local_file_obj.file != 0xffff {
        _lclose16(struct_param_1.file);
        struct_param_1.file = 0xffff;
    }
    struct_param_1.file = _lopen16(&param_1.path, 0);
    // local_file_obj.file = file;
    if file == 0xffff {
        ctx.g_u16_1050_0310 = 0x6cf;
    } else {
        return read_file_func_1008_71a0(ctx, struct_param_1);
    }
    return false;
}

/*
Referenced above
 */
pub fn read_file_func_1008_71a0(ctx: &mut AppContext, in_file_object: &mut FileObject) -> bool {
    let mut count: u32;
    let mut unaff_ss: u16;
    let mut bytes_read: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: Vec<u8> = Vec::new();
    let local_5: u8;
    let mut local_4: bool;

    local_4 = true;
    let mut count = get_string_index_1000_3da4(&ctx.s__1050_65a0);
    local_16 = 0;
    // bytes_read = _hread(count, CONCAT22(unaff_ss, local_e), (in_file_object + 4));
    let mut bytes_read = _hread(in_file_object.file, &mut local_e, count as u32);
    bytes_read._0_2_ = bytes_read;
    if count < bytes_read as u16 {
        bytes_read = count as u32;
    }
    local_18 = (bytes_read - 2) as u16;
    if local_18 < 0 {
        local_18 = 0;
    }
    local_1a = 2;
    while local_18 != 0 {
        local_16 = local_16 * 10 + local_e[local_1a] + -0x30;
        local_1a = local_1a + 1;
        local_18 = local_18 - 1;
    }
    if local_16 == 1 {
        ctx.PTR_LOOP_1050_0312 = (ctx.PTR_LOOP_1050_0000 + 1);
    } else {
        if local_16 == 4 {
            ctx.PTR_LOOP_1050_0312 = ctx.PTR_DAT_0005_0000_1050_0004;
        } else {
            local_5 = '\0' as u8;
            ctx.PTR_LOOP_1050_0312 = (ctx.PTR_LOOP_1050_0000 + 1);
            local_4 = false;
        }
    }
    string_fn_1000_3f9c(
        ctx,
        &ctx.s__1050_65a0,
        &ctx.g_alloc_addr_1050_1050,
        &ctx._PTR_s_SC_03d_1050_0314_1050_031c,
        &ctx.PTR_LOOP_1050_0312,
    );
    return local_4;
}

/*
Refernced in another read function
 */
pub unsafe fn read_file_1008_7548(
    ctx: &mut AppContext,
    in_file_handle: &HFILE16,
    param_2: &mut usize,
) {
    let pp_var1: fn();
    let was_file_read: bool;
    let mut u_var3: i32 = 0;
    let mut u_var4: u32;
    // let mut local_DX_119: u16;
    // let mut local_CS__1: u16;
    let mut lVar5: usize;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_e: u32;
    let mut local_a: &mut StructuredData;
    let mut local_8: u16;
    let mut local_6: StructuredData = StructuredData::new();

    local_6 = 0;
    was_file_read = read_file_1008_7dee(in_file_handle, &mut local_6.buffer, 4);
    if !was_file_read {
        return;
    }
    if local_6 != 0 {
        // let mut u_var4 = local_6;
        // if local_6 < 200 {
        //     local_6 = 0x0;
        //     u_var4 = 200;
        // }
        // let u_var3 = u_var4;
        // local_a = u_var4 & 0xffff | local_6 << 0x10;
        local_a = &mut local_6;
        let param_2_val = *param_2;
        if param_2_val == 0 {
            ctx.code_seg_reg = 0x1000;
            process_struct_1000_179c(ctx, 0x1e, local_6);
            if local_6 == 0 {
                *param_2 = 0;
            } else {
                ctx.code_seg_reg = 0x1020;
                pass1_1020_c444(ctx, local_6, 100, local_a);
                // *param_2 = local_6;
                // (param_2 + 2) = local_DX_119;
            }
        }

        // lVar5 = *param_2;
        // pp_var1 = (*param_2 + 0x24);
        // (**pp_var1)();

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

/*
referenced by outside functions
 */
pub unsafe fn read_file_1008_76e4(ctx: &mut AppContext, param_1: &HFILE16, param_2: &mut u32) {
    let pp_var1: fn();
    let mut b_var3: bool = false;
    // let in_dx: &mut  Struct199;
    let mut local_1c: u16;
    let mut local_1a: u16;
    // let mut local_18: u32;
    let mut local_a: u32;
    let mut val: u16;

    // let mut local_6: u32 = 0;
    let mut local_6: Vec<u8> = Vec::new();
    // CONCAT22(ctx.stack_seg_reg, &local_6)
    let mut u_var2 = read_file_1008_7dee(param_1, &mut local_6, 4);
    if u_var2 == 0 {
        return;
    }
    if local_6 != 0 {
        let param_2_val = unsafe { *param_2 };
        if param_2_val == 0 {
            process_struct_1000_179c(ctx, 0x18, in_dx);
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
        while local_a < local_6 {
            b_var3 = read_file_1008_7dee(param_1, CONCAT22(ctx.stack_seg_reg, &local_18), 4);
            if b_var3 == 0 {
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
    let in_dx: &mut Struct199;
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
                    b_var1 =
                        read_file_1008_7dee(in_file, CONCAT22(ctx.stack_seg_reg, &local_10), 4),
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

pub unsafe fn read_file_1008_7bc8(
    ctx: &mut AppContext,
    in_file: &HFILE16,
    param_2: u32,
) -> u16 {
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
                    //     (seek_offset >> 0x10) + local_410 + CARRY2(seek_offset, local_410),
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
            // seek_offset._0_2_ = CONCAT11(seek_offset + 4, seek_offset);
            //// eek_offset = CONCAT22((seek_offset  >> 0x10) + (0xfb < seek_offset), seek_offset);
            seek_offset = seek_offset + 4;
        }
    }
}

pub fn read_file_1008_7dee(file_handle: &HFILE16, buffer: &mut Vec<u8>, count: u32) -> bool {
    let bytes_read = _hread(&file_handle, buffer, count);
    bytes_read == count
}

pub unsafe fn read_file_1008_bb5e(param_1: &mut Struct120, param_2: &HFILE16) {
    let pp_var1: fn();
    let mut b_read_result_1: u16;
    let b_read_result_2: bool;
    let var3: &mut Struct199;
    let mut local_AX_168: &mut Struct121;
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
        b_read_result_1 = read_file_1008_7dee(_file_handle_ptr, &mut param_1.field_0x0, 2);
    }
    b_read_result_2 = read_file_1008_7dee(_file_handle_ptr, CONCAT22buffer_1, 2);
    if b_read_result_1 != 0 && b_read_result_2 != false {
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
        // u_var9 = (param_1  >> 0x10);
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

pub unsafe fn read_file_1008_e70e(ctx: &mut AppContext, param_1: u32, in_file_1: &HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let b_var3: bool;
    let mut u_var4: u16;
    let extraout_var: u32;
    // let in_dx: &mut  Struct199;
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

pub unsafe fn read_file_1010_0c7c(ctx: &mut AppContext, param_1: u32, in_file_handle_2: &HFILE16) {
    let pp_var1: fn();
    let b_var3: bool;
    let mut u_var4: Struct199 = Struct199::new();
    let b_var5: bool;
    let extraout_var: u32;
    // let in_dx: &mut  Struct199;

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
                        b_var3 = read_file_1008_7dee(in_file_handle_2, &local_16[6..], 4),
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
            b_var3 = read_file_1008_7dee(
                in_file_handle_2,
                (param_1 & 0xffff0000 | (i32_var6 + 0xe)),
                2,
            );
            if ((b_var3 != 0)
                && (
                    b_var3 = read_file_1008_7dee(
                        in_file_handle_2,
                        (param_1 & 0xffff0000 | (i32_var6 + 0x10)),
                        2,
                    ),
                    b_var3 != 0,
                ))
            {
                local_4 = 0;
                while (local_4 < 10) {
                    b_var3 =
                        read_file_1008_7dee(in_file_handle_2, CONCAT22(unaff_ss, &local_2a), 2);
                    if (b_var3 == 0) {
                        // goto LAB_1010_0cb1;
                    }
                    b_var5 = local_4;
                    if (PTR_LOOP_1050_0312 < 2) {
                        switch_statement_1008_738c(
                            in_file_handle_2,
                            (in_file_handle_2 >> 0x10),
                            local_4,
                        );
                        b_var5 = b_var3;
                    }
                    (b_var5 * 8 + 0xe28) = local_2a;
                    local_4 = local_4 + 1;
                    local_26 = b_var5;
                }
                if (2 < PTR_LOOP_1050_0312) {
                    local_4 = 0;
                    while {
                        b_var3 =
                            read_file_1008_7dee(in_file_handle_2, CONCAT22(unaff_ss, &local_2a), 2);
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

pub unsafe fn read_file_1010_404a(
    ctx: &mut AppContext,
    param_1: &mut Struct407,
    param_2: &HFILE16,
) {
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
                b_var2 = read_file_1008_7dee(param_2, param_1, 2);
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
            // u_var5 = (param_1  >> 0x10);
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
                    b_var3 = read_file_1008_7dee(
                        hfile_param_2,
                        (param_1 & 0xffff0000 | (i_var4 + 0x82)),
                        2,
                    );
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

pub unsafe fn read_file_1010_68c6(param_1: u32, param_2: &HFILE16) {
    let b_var1: bool;
    let u_var2: u8;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let b_var5: bool;
    let b_var6: bool;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let in_dx: &mut Struct199;

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

    u_var2 = read_file_1008_7cfe(ctx, param_2);
    i_var3 = CONCAT31(extraout_var, u_var2);
    if (i_var3 == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
        return;
    }
    i_var4 = param_1;
    u_var8 = param_2;
    // u_var9 = (param_2  >> 0x10);
    // u_var7 = (param_1  >> 0x10);
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
        u_var2 = error_check_1000_17ce(ctx, local_a);
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
        error_check_1000_17ce(ctx, local_12);
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

pub unsafe fn read_file_1010_9b72(param_1: u32, hfile_param_2: &HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let struct_a_2: Address<Struct199>;
    let var3: Address<Struct199>;
    let b_result: bool;
    // let local_AX_165: &mut  Struct473;
    let mut struct_b: Address<Struct1174>;
    let mut u_var4: i32;
    // let local_AX_571: &mut  Struct472;
    let mut u_var5: u16;
    let extraout_var: u32;
    let mut struct_a_1: Address<Struct199>;
    let mut var6: Address<Struct199>;

    // let ctx.dx_reg: &mut  Struct199;
    // let ctx.dx_reg: &mut  Struct199;
    // let local_bx_60: &mut  Struct471;
    let pu_var7: &mut u32;
    let local_es_60: Vec<u8>;
    let local_es_165: Vec<u8>;
    let mut u_var8: u16;
    let string_a_1: String;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_36: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let paStack38: &mut Struct473;
    let mut local_22: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let local_e: &mut Struct473;
    let string_a_2: String;
    let mut local_4: u16;

    let u_var2 = read_file_1008_7cfe(ctx, hfile_param_2);
    if CONCAT31(extraout_var, u_var2) != 0 {
        struct_a_2 = read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_4), 2);
        if struct_a_2 != 0x0 {
            local_bx_60 = param_1;
            // local_es_60 = (param_1  >> 0x10);
            u_var9 = hfile_param_2;
            // u_var10 = (hfile_param_2  >> 0x10);
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
                    b_result =
                        read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &string_a_2), 2);
                    if b_result == 0 {
                        // LAB_1010_9c32:
                        _local_16 = local_e;
                        if local_e == 0x0 {}
                        // goto LAB_1010_9ba1;
                        // u_var8 = (local_e  >> 0x10);
                        pu_var7 = local_e;
                        // goto LAB_1010_9e9e;
                    }
                    b_result = read_file_1008_7dee(
                        hfile_param_2,
                        (local_e & 0xffff0000 | (local_e + 6)),
                        2,
                    );
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
                        b_result =
                            read_file_1008_7dee(hfile_param_2, CONCAT22(string_a_1, &local_1a), 2);
                        if b_result == 0 {
                            // LAB_1010_9d5c:
                            _local_16 = local_1e;
                            if local_1e == 0 {}
                            // goto LAB_1010_9ba1;
                            // u_var8 = (local_1e  >> 0x10);
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
                            b_result = read_file_1008_7dee(
                                hfile_param_2,
                                CONCAT22(string_a_1, &local_22),
                                2,
                            );
                            if (b_result == 0) {
                                // LAB_1010_9e86:
                                _local_16 = paStack38;
                                if (paStack38 != 0x0) {
                                    // u_var8 = (paStack38  >> 0x10);
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

pub unsafe fn read_file_1018_0000(param_1: u32, param_2: &HFILE16) {
    let pu_var1: &mut u16;
    let mut u_var2: u32;
    let u_var3: u8;
    let mut i_var4: i32;
    let b_var5: bool;
    let puVar6: Vec<u8>;
    let extraout_var: u32;
    let in_dx: &mut u16;
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
    u_var3 = read_file_1008_7cfe(ctx, param_2);
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
            // u_var7 = (param_1  >> 0x10);
            if ((i_var4 + 0x30) != 0) {
                g_struct_ptr_1 = ((i_var4 + 0x32) * 6);
                if (ctx.g_struct_ptr_1 == 0) {
                    struct_fn_1000_160a();
                    ctx.g_u16_ptr_1050_5f2e = in_dx;
                } else {
                }
                alloc_mem_1000_1708();
                (i_var4 + 0x2c) = g_struct_ptr_1;
                (i_var4 + 0x2e) = ctx.g_u16_ptr_1050_5f2e;
                zero_list_1008_3e38(CONCAT22(unaff_ss, local_20));
                local_10 = 0;
                let pu_var1_val = unsafe { *pu_var1 };
                while (
                    pu_var1 = (i_var4 + 0x30),
                    pu_var1_val != local_10 && local_10 <= pu_var1_val,
                ) {
                    puVar6 = local_20;
                    read_file_1008_7bc8(ctx, param_2, (param_2 >> 0x10), puVar6, unaff_ss);
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

pub unsafe fn read_from_file_1038_7c02(param_1: &mut Vec<u8>, file_b: &HFILE16) -> u16 {
    let u8_a: u8;
    // let extraout_AH: u8;
    let bool_a: bool;
    let mut u_var1: u16;
    let mut i_var2: i32;
    // let in_dx: &mut  Struct199;

    let mut u_var3: i32;
    // let ctx.dx_reg: &mut  Struct199;
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
    u8_a = read_file_1008_7cfe(ctx, file_b);
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
                i_var2 = misc::pass1_1030_b836(CONCAT22(u_var3, u_var1), ph_var4);
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

pub unsafe fn read_from_file_1038_774e(param_1: u32, param_2: &HFILE16) {
    let mut u_var1: u16;
    let local_AX_22: &mut Struct1168;
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
        // u_var1 = (param_1  >> 0x10);
        u_var5 = param_2;
        // u_var6 = (param_2  >> 0x10);
        read_file_1008_77cc(ctx, u_var5, u_var6, i_var3, u_var1);
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
            read_file_1008_7bc8(ctx, u_var5, u_var6, i_var3, u_var1);
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
    let pu_var3: &mut u32;
    let b_var4: bool;
    let local_AX_307: &mut Struct1142;
    let pu_var5: Vec<u8>;
    let extraout_var: u32;

    let struct_a: &mut Struct199;
    let paVar6: &mut Struct199;
    let local_bx_68: &mut Struct933;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_426: u32;
    let mut local_416: u16;
    let mut local_414: u16;
    let local_412: &mut Struct1143;
    let mut local_40e: u32;
    let mut local_408: [u8; 1024] = [0;1024];
    let mut local_8: u16;
    let mut local_6: u32;

    u_var2 = pass1_1030_1730(param_1, hfile_param_2);
    if CONCAT31(extraout_var, u_var2) == 0 {
        return;
    }
    local_6 = 0;
    pu_var3 = &mut local_6;
    //// _var8 = (hfile_param_2  >> 0x10);
    read_file_1008_7548(ctx, hfile_param_2, pu_var3);
    if pu_var3 != 0x0 {
        // u_var7 = (param_1  >> 0x10);
        local_bx_68 = param_1;
        local_bx_68.field_0xc = local_6;
        struct_a = ctx.dx_reg;
        b_var4 = read_file_1008_7dee(
            hfile_param_2,
            (param_1 & 0xffff0000 | &local_bx_68.field_0x10),
            4,
        );

        if b_var4 != false {
            b_var4 = read_file_1008_7dee(
                hfile_param_2,
                (param_1 & 0xffff0000 | &local_bx_68.field_0x18),
                2,
            );
            if b_var4 != false {
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x1a),
                    2,
                )
            }
            if b_var4 != false {
                b_var4 = read_file_1008_7dee(hfile_param_2, CONCAT22(unaff_ss, &local_8), 2)
            }
            if b_var4 != false {
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x1e),
                    4,
                )
            }
            if b_var4 != false {
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    (param_1 & 0xffff0000 | &local_bx_68.field_0x22),
                    2,
                )
            }

        }

        if b_var4 != false {
            local_bx_68.field_0x1c = local_8;
            b_var4 = read_file_1008_7dee(
                hfile_param_2,
                (param_1 & 0xffff0000 | &local_bx_68.field_0x24),
                2,
            );
        }
        if b_var4 != false {
            local_AX_307 = read_file_1008_7dee(
                hfile_param_2,
                (param_1 & 0xffff0000 | &local_bx_68.field_0x26),
                0x94,
            );
        }
        if local_AX_307 != 0x0 {
            if (ctx.PTR_LOOP_1050_0312 < 2) {
                u_var9 = 0x54;
                u_var10 = 0;
                process_struct_1000_179c(0x54, struct_a);
                _local_416 = CONCAT22(struct_a, local_AX_307);
                b_var4 = read_file_1008_7dee(
                    hfile_param_2,
                    CONCAT22(struct_a, local_AX_307),
                    CONCAT22(u_var10, u_var9),
                );
                if b_var4 == 0 {
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
            read_from_file_1030_33f0(local_bx_68.field_0x1f6, hfile_param_2);
            pu_var5 = local_408;
            read_file_into_str_1008_7c6e(hfile_param_2, CONCAT22(unaff_ss, pu_var5));
        }



                if pu_var5 != 0x0 {
                    pu_var5 = local_408;
                    pass1_fn_1008_60e8(pu_var5);
                    local_bx_68.field_0x1fa = pu_var5;
                    local_bx_68.field_0x1fc = struct_a;
                    b_var4 = read_file_1008_7dee(
                        hfile_param_2,
                        (param_1 & 0xffff0000 | &local_bx_68.field_0x1fe),
                        2,
                    );
                    if ((((b_var4 != 0)
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
                            )))
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

pub fn read_from_file_1030_dec4(ctx: &mut AppContext, param_1: u32, param_2: &HFILE16) {
    let b_var1: bool;

    read_from_file_1028_b81a(param_1, param_2);
    if (ctx.ax_reg != 0) && (1 < ctx.PTR_LOOP_1050_0312) {
        // TODO: read from file into u32
        // b_var1 = read_file_1008_7dee(param_2, param_1, 4),
        if b_var1 == false {
            ctx.g_u16_1050_0310 = 0x6d2;
        }
    }
}

pub fn read_from_file_1030_c894(param_1: u32, param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut local_4: u16;

    // read from file
    b_var1 = read_from_file_1028_b81a(param_1, param_2);
    if b_var1 != false {
        // read u16 from file
        b_var1 = read_file_1008_7dee(param_2, local_4, 2);
        if b_var1 == false {
            ctx.g_u16_1050_0310 = 0x6d2;
            return b_var1;
        }
        (param_1 + 0x20) = local_4;
        b_var1 = 1;
    }
    return b_var1;
}

pub fn read_from_file_1030_c29c(ctx: &mut AppContext, param_1: &mut StructuredData, param_2: &HFILE16) {
    let b_var1: bool;

    read_from_file_1028_b81a(param_1, param_2);
    if ctx.ax_reg != 0 {
        // read u32
        b_var1 = read_file_1008_7dee(param_2, (param_1 + 0x20), 4);
        if b_var1 != false {
            // read u16
            b_var1 = read_file_1008_7dee(param_2, (param_1 + 0x24), 2);
            if b_var1 != false {
                return;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn read_from_file_1030_778c(struct_b: &mut Struct933, file_a: &HFILE16) {
    let mut u_var1: u32;
    let u_var2: u8;
    let local_AX_32: &mut Struct933;
    let BVar3: bool;
    let paVar4: &mut Struct493;
    let local_AX_482: Struct935;
    let mut u_var5: u16;
    let local_AX_662: Struct934;
    let local_AX_1036: Struct936;
    let extraout_var: u32;

    let mut u_var8: i32;

    let paVar9: &mut Struct199;

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

    u_var2 = pass1_1030_1730(struct_b, file_a);
    if CONCAT31(extraout_var, u_var2) != 0 {
        local_AX_32 = struct_b;
        local_AX_32 = &local_AX_32.field_0xc;
        u_var6 = struct_b & 0xffff0000 | local_AX_32;
        read_file_1008_7bc8(ctx, file_a, u_var6);
        if (u_var6 != 0) {
            BVar3 = read_file_1008_7dee(file_a, CONCAT22(base_ptr, &offset_ptr), 2)
            if BVar3 != false {
                // u_var11 = (struct_b  >> 0x10);
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
                        read_file_1008_76e4(ctx, file_a, u_var6);
                        if (u_var6 != 0) {
                            u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x22;
                            read_file_1008_77cc(ctx, file_a, u_var6);
                            if (u_var6 != 0) {
                                u_var6 = struct_b & 0xffff0000 | &local_AX_32.field_0x26;
                                read_file_1008_77cc(ctx, file_a, u_var6);
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
                                                // u_var12 = (local_2c  >> 0x10);
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
                                                    _local_32 = (pu_var7 & 0xffff | ctx.dx_reg << 0x10);
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
                                                    // u_var12 = (local_4a  >> 0x10);
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
        }
            && (
                ,
                BVar3 != false,
            )
        {

        }
        // LAB_1030_77be:
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn read_from_file_1030_5e70(ctx: &mut AppContext, param_1: u32, param_2: &HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let pi_var3: &mut i32;

    let pu_var4: Vec<u8>;
    let mut i_var5: i32;
    let b_var6: bool;
    let mut u_var7: i32;
    let in_dx: &mut u16;
    let puVar8: &mut u16;

    let struct_a: &mut Struct199;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let pp_var11: &mut Struct2111;
    let mut u_var12: u16;
    let mut local_420: u32;
    let mut local_416: u16;
    let mut local_414: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    // u_var12 = (param_1  >> 0x10);
    misc::pass1_1030_19b4(param_1, param_2);
    if (ctx.ax_reg != 0x0) {
        if (ctx.g_struct_ptr_1 == 0) {
            g_struct_ptr_1 = ctx.ax_reg;
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        alloc_mem_1000_1708();
        _local_40a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, g_struct_ptr_1);
        puVar8 = (ctx.g_u16_ptr_1050_5f2e | g_struct_ptr_1);
        i_var9 = param_1;
        if (puVar8 == 0x0) {
            (i_var9 + 0x10) = 0;
        } else {
            zero_list_1008_3e38(CONCAT22(ctx.g_u16_ptr_1050_5f2e, &g_struct_ptr_1.field_0x4));
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
            read_file_1008_7bc8(ctx, param_2, (param_2 >> 0x10), i_var5, (u_var1 >> 0x10));
            if (i_var5 != 0) {
                u_var2 = (i_var9 + 0x10);
                local_420._0_2_ = u_var2 + 10;
                struct_a = ctx.dx_reg;
                b_var6 = read_file_1008_7dee(param_2, (u_var2 & 0xffff0000 | local_420), 2);
                if (b_var6 != 0) {
                    u_var1 = (i_var9 + 0x10);
                    // u_var10 = (u_var1  >> 0x10);
                    i_var5 = u_var1;
                    if ((i_var5 + 10) == 0) {
                        // LAB_1030_5fb7:
                        pp_var11 = process_struct_1010_20ba(
                            ctx.g_struct_var_1050_0ed0,
                            CONCAT22(local_420, 0x2f),
                        );
                        pass1_1018_04ca(pp_var11, (i_var9 + 4));
                        return;
                    }
                    local_420._0_2_ = (i_var5 + 10) * 2;
                    u_var7 = local_420;
                    process_struct_1000_179c(local_420, struct_a);
                    u_var1 = (i_var9 + 0x10);
                    // u_var10 = (u_var1  >> 0x10);
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

pub unsafe fn read_file_1030_5c52(buf_a: Vec<u8>, hfile_param_2: &HFILE16) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;

    u_var1 = read_file_1008_7cfe(ctx, hfile_param_2);
    BVar2 = CONCAT11(extraout_AH, u_var1);
    if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee(hfile_param_2, buf_a, 0x24);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return BVar2;
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub unsafe fn read_from_file_1030_581e(param_1: u32, param_2: &HFILE16) {
    let pu_var1: &mut u16;
    let mut u_var2: u32;

    let BVar3: bool;
    let pu_var4: Vec<u8>;
    let mut in_i16_5: u16;
    let mut u_var5: u32;
    let in_dx: &mut u16;

    let struct_a: &mut Struct199;
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

    // u_var10 = (param_2  >> 0x10);
    // u_var9 = (param_1  >> 0x10);
    misc::pass1_1030_19b4(param_1, param_2);
    if (ctx.ax_reg != 0x0) {
        if (ctx.g_struct_ptr_1 == 0) {
            g_struct_ptr_1 = ctx.ax_reg;
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        alloc_mem_1000_1708(0x20, 0, 1, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
        if ((ctx.g_u16_ptr_1050_5f2e | g_struct_ptr_1) == 0) {
            g_struct_ptr_1 = 0x0;
            struct_a = 0x0;
        } else {
            pass1_1030_84ae(g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
            struct_a = ctx.dx_reg;
        }
        i32_var6 = param_1;
        (i32_var6 + 0x10) = g_struct_ptr_1;
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
                    error_check_1000_17ce(ctx, (u_var2 + 4));
                    pu_var4 = local_408;
                    pass1_fn_1008_60e8(pu_var4);
                    u_var2 = (i32_var6 + 0x10);
                    // u_var8 = (u_var2  >> 0x10);
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
                                ctx,
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
                        read_file_1008_7bc8(ctx, param_2, u_var10, i_var7, u_var9);
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
    let mut local_14: u32 = 0;

    if param_4 != 0x0 {
        let mut pu_var3 = local_2e;
        let mut u_var2 = 0;
        let mut path = pass1_1030_5164(param_1, param_4);
        var1 = path;
        dos3_call_1000_51aa(&mut var1, &path, pu_var3);
        if var1.is_empty() == false {
            *param_2 = local_14;
            _file = _lopen16(&path, 0);
            var1 = _file + 1;
            if var1 != 0 {
                let param_2_val = param_2;
                let param_3_val = param_3;

                alloc_mem_1000_0a48(
                    1,
                    param_2_val,
                    (param_2_val >> 0x10),
                    ctx.g_struct_ptr_1,
                    (ctx.g_struct_ptr_1 >> 0x10),
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

pub fn read_from_file_1030_33f0(param_1: u32, param_2: &HFILE16) {
    let local_AX_14: &mut Struct874;
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

pub unsafe fn read_file_1030_2c8a(param_1: u32, param_2: &HFILE16) {
    let pu_var1: &mut u16;

    let BVar2: bool;
    let local_AX_184: Vec<u8>;
    let pu_var3: Vec<u8>;
    let mut u_var4: u16;
    let in_dx: &mut u16;
    let pu_var5: &mut u16;

    let struct_a: &mut Struct199;
    let local_bx_91: &mut Struct866;
    let mut i32_var6: i32;
    let mut local_SI__1: u16;
    let mut local_es_210: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pp_var8: &mut Struct2111;
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

    // u_var9 = (param_1  >> 0x10);
    misc::pass1_1030_19b4(param_1, param_2);
    if (ctx.ax_reg == 0x0) {
        return;
    }
    if (ctx.g_struct_ptr_1 == 0) {
        g_struct_ptr_1 = ctx.ax_reg;
        struct_fn_1000_160a();
        ctx.g_u16_ptr_1050_5f2e = in_dx;
    } else {
    }
    alloc_mem_1000_1708();
    _local_40e = CONCAT22(ctx.g_u16_ptr_1050_5f2e, g_struct_ptr_1);
    pu_var5 = (ctx.g_u16_ptr_1050_5f2e | g_struct_ptr_1);
    local_bx_91 = param_1;
    if (pu_var5 == 0x0) {
        local_bx_91.field_0x10 = 0x0;
    } else {
        zero_list_1008_3e38(CONCAT22(ctx.g_u16_ptr_1050_5f2e, &g_struct_ptr_1.field_0x6));
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
            // local_es_210 = (pu_var1  >> 0x10);
            i32_var6 = pu_var1;
            *(i32_var6 + 2) = pu_var3;
            (i32_var6 + 4) = pu_var5;
            pu_var1 = local_bx_91.field_0x10;
            i32_var6 = pu_var1 + 6;
            read_file_1008_7bc8(ctx, param_2, (param_2 >> 0x10), i32_var6, (pu_var1 >> 0x10));
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
                    // u_var7 = (pu_var1  >> 0x10);
                    i32_var6 = pu_var1;
                    u_var4 = (i32_var6 + 0x22) * 2;
                    process_struct_1000_179c(u_var4, struct_a);
                    (i32_var6 + 0x24) = u_var4;
                    (i32_var6 + 0x26) = struct_a;
                    pu_var1 = local_bx_91.field_0x10;
                    // u_var7 = (pu_var1  >> 0x10);
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
                        ctx.g_struct_var_1050_0ed0,
                        CONCAT22(local_SI__1, 0x2f),
                    );
                    pass1_1018_04a4(pp_var8, local_bx_91.field_0x4);
                    pass1_1010_82f8(ctx.g_struct_73_1050_14cc, *local_bx_91.field_0x10);
                    return;
                }
            }
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub unsafe fn read_file_1030_232e(param_1: u32, param_2: &HFILE16) {
    let mut i_var1: i32;
    let BVar2: bool;

    misc::pass1_1030_19b4(param_1, param_2);
    if (ctx.ax_reg != 0) {
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

pub unsafe fn read_file_1028_def2(param_1: Vec<u8>, file_handle_1: &HFILE16) {
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

    // u_var5 = (file_handle_1  >> 0x10);
    u_var2 = read_file_1008_7cfe(ctx, file_handle_1);
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
                    u_var2 = read_file_1008_7cfe(ctx, file_handle_1);
                    i_var4 = CONCAT31(extraout_var_00, u_var2);
                    if (i_var4 != 0) {
                        misc::pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x100);
                        if (i_var4 != 0) {
                            u_var2 = read_file_1008_7cfe(ctx, file_handle_1);
                            i_var4 = CONCAT31(extraout_var_01, u_var2);
                            if (i_var4 != 0) {
                                misc::pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x200);
                                if (i_var4 != 0) {
                                    u_var2 = read_file_1008_7cfe(ctx, file_handle_1);
                                    i_var4 = CONCAT31(extraout_var_02, u_var2);
                                    if (i_var4 != 0) {
                                        misc::pass1_1028_e628(param_1, u_var6, u_var5, 0, 0x300);
                                        if (i_var4 != 0) {
                                            u_var2 = read_file_1008_7cfe(ctx, file_handle_1);
                                            i_var4 = CONCAT31(extraout_var_03, u_var2);
                                            if (i_var4 != 0) {
                                                misc::pass1_1028_e628(
                                                    param_1, u_var6, u_var5, 0, 0x400,
                                                );
                                                if (i_var4 != 0) {
                                                    u_var2 =
                                                        read_file_1008_7cfe(ctx, file_handle_1);
                                                    i_var4 = CONCAT31(extraout_var_04, u_var2);
                                                    if (i_var4 != 0) {
                                                        misc::pass1_1028_e628(
                                                            param_1, u_var6, u_var5, 0, 0x500,
                                                        );
                                                        if (i_var4 != 0) {
                                                            u_var2 = read_file_1008_7cfe(
                                                                ctx,
                                                                file_handle_1,
                                                            );
                                                            i_var4 =
                                                                CONCAT31(extraout_var_05, u_var2);
                                                            if (i_var4 != 0) {
                                                                misc::pass1_1028_e628(
                                                                    param_1, u_var6, u_var5, 0,
                                                                    0x600,
                                                                );
                                                                if (i_var4 != 0) {
                                                                    u_var2 = read_file_1008_7cfe(
                                                                        ctx,
                                                                        file_handle_1,
                                                                    );
                                                                    i_var4 = CONCAT31(
                                                                        extraout_var_06,
                                                                        u_var2,
                                                                    );
                                                                    if (i_var4 != 0) {
                                                                        misc::pass1_1028_e628(
                                                                            param_1, u_var6,
                                                                            u_var5, 0, 0x700,
                                                                        );
                                                                        if (i_var4 != 0) {
                                                                            u_var2 =
                                                                                read_file_1008_7cfe(
                                                                                    ctx,
                                                                                    file_handle_1,
                                                                                );
                                                                            i_var4 = CONCAT31(
                                                                                extraout_var_07,
                                                                                u_var2,
                                                                            );
                                                                            if (i_var4 != 0) {
                                                                                misc::pass1_1028_e628(
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

pub unsafe fn read_file_1028_d7ba(param_1: u16, param_2: u16, param_1_00: &HFILE16) -> i32 {
    let u_var1: u8;
    let extraout_AH: u8;

    u_var1 = read_file_1008_7cfe(ctx, param_1_00);
    if (CONCAT11(extraout_AH, u_var1) == 0) {
        ctx.g_u16_1050_0310 = 0x6d4;
        return CONCAT11(extraout_AH, u_var1);
    }
    return 1;
}

pub fn read_from_file_1028_b81a(param_1: &mut StructuredData, in_file_1: &HFILE16) {
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
    // u_var7 = (param_1  >> 0x10);
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
    // u_var11 = (in_file_1  >> 0x10);
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
            // local_e = (u_var8  >> 0x10);
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

pub fn read_from_file_1028_65e2(param_1: u32, param_2: &HFILE16) {
    let pp_var1: fn();

    let BVar2: bool;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let pu_var5: Vec<u8>;

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

    read_from_file_1028_b81a(param_1, param_2);
    if (ctx.ax_reg != 0) {
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
                // u_var6 = (local_14  >> 0x10);
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

pub fn read_from_file_1028_5fc8(param_1: u32, param_2: &HFILE16) {
    let b_var1: bool;

    read_from_file_1028_b81a(param_1, param_2);
    if ((ctx.ax_reg != 0)
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

pub fn read_file_1028_3d92(in_struct: &mut Struct772, in_file_handle: &HFILE16) {
    let local_struct_1: &mut Struct772;
    let b_var1: bool;
    let mut u_var2: u16;

    read_from_file_1028_b81a(in_struct, in_file_handle);
    if (ctx.ax_reg != 0) {
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

pub unsafe fn read_file_fn_1028_24a2(param_1: u32, param_2: &HFILE16) -> bool {
    let pp_var1: fn();
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let struct_a: &mut Struct199;

    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let ph_var6: &HFILE16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = read_from_file_1028_b81a(param_1, param_2);
    // struct_a = (u_var5  >> 0x10);
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

pub fn read_file_fn_1028_14d8(param_1: u32, param_2: &HFILE16) {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    read_from_file_1028_b81a(param_1, param_2);
    if (ctx.ax_reg != 0) {
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

pub fn file_read_fn_1028_0374(param_1: u32, param_2: &HFILE16) {
    let pp_var1: fn();

    let BVar2: bool;
    let mut u_var3: u16;
    let local_AX_291: &mut Struct732;
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
    let pu_var5: Vec<u8>;

    read_from_file_1028_b81a(param_1, param_2);
    if (ctx.ax_reg != 0) {
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
                    // u_var6 = (local_14  >> 0x10);
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

pub fn file_read_fn_1020_e994(param_1: u32, param_2: &HFILE16) {
    let b_var1: bool;

    read_from_file_1030_dec4(param_1, param_2);
    if ((ctx.ax_reg != 0)
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

pub fn file_read_fn_1020_e70e(param_1: Vec<u8>, param_2: u32) {
    let b_var1: bool;

    read_from_file_1030_dec4(param_1, param_2);
    if (ctx.ax_reg != 0) {
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

pub fn call_read_file_1020_d41a(param_1: Vec<u8>, param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let local_4: Vec<u8>;

    b_var1 = read_from_file_1028_b81a(param_1, param_2);
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

pub unsafe fn call_read_file_1020_a65e(param_1: u32, in_file_handle: &HFILE16) -> u16 {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;

    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: [u8; 2];
    let mut local_4: [u8; 2];

    u_var1 = read_file_1008_7cfe(ctx, in_file_handle);
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
                BVar2 =
                    read_file_1008_7dee(in_file_handle, CONCAT22(ctx.stack_seg_reg, local_6), 2);
                if (BVar2 != 0) {
                    BVar2 = read_file_1008_7dee(
                        in_file_handle,
                        CONCAT22(ctx.stack_seg_reg, local_a),
                        2,
                    );
                    if (BVar2 != 0) {}
                    // goto LAB_1020_a6dc;
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return 0;
}

pub fn pass1_1030_1730(param_1: &mut Struct933, param_2: &HFILE16) -> u8 {
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

pub unsafe fn read_from_file_1008_6f7a(ctx: &mut AppContext,
                                       param_1: &FileObject,
                                       hfile_param_3: &HFILE16) {
    let local_AX_45: Vec<u8>;
    let b_var1: bool;
    let pu_var2: Vec<u8>;

    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = ctx._g_bool_1050_5748;
    local_a = local_6;
    pass1_1020_a43e(local_e);
  // local_AX_45 = read_file_1028_d7ba(local_a, (local_a  >> 0x10), param_1_00);
    if local_AX_45 != 0x0 {
        b_var1 = read_file_1030_5c52(&mut ctx._PTR_LOOP_1050_5736, hfile_param_3);
        if b_var1 != false {
            read_file_1028_def2(&mut ctx._PTR_LOOP_1050_65e2, hfile_param_3);
            if b_var1 != false {
                read_from_file_1038_7c02(&mut ctx._PTR_LOOP_1050_5a64, hfile_param_3);
                if b_var1 != false {
                    pu_var2 = Vec::from(local_e);
                    call_read_file_1020_a65e(pu_var2, hfile_param_3);
                    if pu_var2 != 0x0 {
                        return;
                    }
                }
            }
        }
    }
    return;
}

pub unsafe fn read_from_file_1008_6e78(ctx: &mut AppContext, struct_param_1: &mut FileObject) -> bool {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    // let mut unaff_ss: u16;
    let mut local_4: HFILE16;

    ctx.g_u16_1050_0310 = 0;
    b_var1 = read_from_file_1008_7146(ctx, struct_param_1);
    if b_var1 != false {
        //// _var4 = (struct_param_1  >> 0x10);
        // u_var3 = struct_param_1;
        // CONCAT22(unaff_ss, local_4)
        // (u_var3 + 4)
        // set_array_val_1008_72a8(local_4, struct_param_1 );
        // CONCAT22(unaff_ss, local_4)
        i_var2 = pass1_1008_7056(struct_param_1, &mut local_4);
        if (i_var2 != 0)
            && (
            i_var2 = read_from_file_1008_6f7a(ctx, struct_param_1, &mut local_4),
            i_var2 != 0,
            )
        {
            b_var1 = close::close_file_1008_726c(ctx, struct_param_1);
            if b_var1 == 0 {
                return false;
            }
            return true;
        }
        _lclose16((u_var3 + 4));
    }
    return false;
}
