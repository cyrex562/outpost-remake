use crate::app_context::AppContext;
use crate::file_ops::{close, file2};
use crate::list_funcs::set_array_val_1008_72a8;
use crate::mem_funcs::Address;
use crate::pass::pass14_funcs::{pass1_1008_3eb4, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6eee, pass1_1008_7006};
use crate::pass::pass15_funcs::{pass1_1020_bb16, pass1_1020_c4a8};
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::pass::pass6_funcs::pass1_1038_3ba0;
use crate::string_ops::misc::{get_string_index_1000_3da4, string_fn_1000_3f9c};
use crate::structs::prog_structs_12::Struct235;
use crate::structs::prog_structs_20::{Struct506, Struct771};
use crate::structs::prog_structs_23::Struct1167;
use crate::structs::prog_structs_25::Struct731;
use crate::structs::prog_structs_26::{Struct1140, Struct873};
use crate::structs::prog_structs_28::{FileObject, Struct961};
use crate::structs::prog_structs_29::Struct425;
use crate::structs::prog_structs_2::Struct390;
use crate::typedefs::HFILE16;
use crate::util::{CONCAT11, CONCAT22, CONCAT31, SBORROW2, ZEXT24};
use crate::winapi::{_hwrite16, _lclose16, _lcreat16};

pub unsafe fn write_to_file_1008_6e02(ctx: &mut AppContext, param_1: &mut FileObject) -> bool {
    let mut local_4: [u8; 2];

    ctx.g_u16_1050_0310 = 0;
    let mut b_var1 = write_to_file_1008_70a6(ctx,param_1);
    if b_var1 != false {
        //// _var4 = (param_1  >> 0x10);
        // u_var3 = param_1;
        set_array_val_1008_72a8(&local_4, param_1.file);
        let mut i_var2 = pass1_1008_7006(param_1, &local_4);
        if (i_var2 != 0)
            && (
                i_var2 = pass1_1008_6eee(param_1, &local_4),
                i_var2 != 0,
            )
        {
            return close::close_file_1008_726c(ctx, param_1);
        }
        _lclose16(param_1.file);
    }
    return false;
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

pub fn write_to_file_1008_7898(ctx: &mut AppContext, in_file: &mut HFILE16, param_2: &mut  u32) {
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

pub fn write_to_file_1008_7954(ctx: &mut AppContext, param_1: &mut HFILE16, param_2: &mut  u32) {
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
      // local_es_11 = (param_2  >> 0x10);
        ctx.ax_reg = (param_2 + 4);
        local_4 = (param_2 + 6);
    }
    write_to_file_1008_7954(ctx, in_file, CONCAT22(local_4, ctx.ax_reg));
    return;
}

pub fn write_to_file_1008_7a22(param_1: &HFILE16, param_2: &mut  Vec<u8>) {
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

pub fn write_to_file_1008_7e1c(
    file: &HFILE16,
    buffer: &Vec<u8>,
    count: usize,
) -> bool {
    let bytes_written = _hwrite16(file, buffer, count);
    bytes_written == count
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
      // u_var6 = (param_1  >> 0x10);
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

pub unsafe fn write_file_1010_0ad2(ctx: &mut AppContext, in_struct_1: &mut  Struct235, param_2: u32) {
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

pub fn write_to_file_1010_5dc6(
    in_struct_1: &mut Address<Struct425>,
    in_file_1: &HFILE16) -> bool {
    let mut u_var1: u32;
    let u_var2: u8;
    // let extraout_AH: u8;
    let mut i_var3: i32;
    let b_var4: bool;
    // let ctx.bx_reg: &mut  Struct425;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    let u_var2 = write_to_file_1008_7cac(in_file_1, 4);
    if u_var2 != false {
        ctx.bx_reg = in_struct_1;
        let u_var1 = ctx.bx_reg.field_0x68;
      // i_var3 = write_to_file_1008_7c2a(ctx,in_file_1, u_var1, (u_var1  >> 0x10));
        if (i_var3 != 0) {
            u_var1 = ctx.bx_reg.field_0x6c;
          // i_var3 = write_to_file_1008_7c2a(ctx,in_file_1, u_var1, (u_var1  >> 0x10));
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
      // u_var4 = (param_1  >> 0x10);
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

pub unsafe fn write_to_file_1010_9900(ctx: &mut AppContext, param_1: u16, param_2: &HFILE16) -> bool {
    let mut u_var1: u32;
    let u_var2: u8;
    // let extraout_AH: u8;
    let BVar3: bool;
    let mut u_var4: u16;
    // let ctx.bx_reg: &mut  Struct470;
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
    //// _var5 = (param_1  >> 0x10);
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
              // u_var4 = (lVar6  >> 0x10);
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
                      // u_var4 = (lVar6  >> 0x10);
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

pub unsafe fn write_to_file_1010_ed58(param_1: u32, in_file: &HFILE16) {
    let pu_var1: &mut  u16;
    let u_var2: u8;
    let BVar3: bool;
    let extraout_var: u32;
    // ppu_var4: &mut  Vec<u8>;
    let local_bx_30: &mut  Struct506;
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
      // local_es_30 = (param_1  >> 0x10);
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

pub unsafe fn write_to_file_1038_7b20(param_1: &mut  u32, param_2: &HFILE16) -> u16 {
    let mut u_var1: u32;
    let u_var2: u8;
    let extraout_AH: u8;
    let BVar3: bool;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let lVar6: u32;
    let pa_var7: &mut  Struct961;
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
                  // u_var5 = (param_1  >> 0x10);
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
                      // local_18 = (pa_var7  >> 0x10);
                        pa_var7 != 0
                    } {}
                    return 0;
                }
              // i_var4 = write_to_file_1038_75ca(ctx, lVar6, param_2, (param_2  >> 0x10));
                i_var4 != 0
            } {}
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

    //// _var3 = (param_1  >> 0x10);
    // local_bx_4 = param_1;
    write_to_file_1008_79f0(ctx,file_handle, param_1.field_0x4);
    if ctx.ax_reg != 0 {
        offset_2 = param_1.field_0x8;
        b_var1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_2), 4);
        if b_var1 != 0 {
            write_to_file_1008_7a22(file_handle, param_1.field_0xe);
            if b_var1 != 0 {
                offset_1 = CONCAT22(offset_1, param_1.field_0xc);
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

pub fn write_to_file_1038_5e16(ctx: &mut AppContext, param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;
    let pu_var2: &mut  u32;
    let local_bx_28: &mut  Struct1140;
    let mut u_var3: u16;
    let mut string_base: u16;
    let mut offset_1: u32;
    let mut offset_2: u32;
    let mut local_6: u32;

    write_to_file_1030_16d6(param_1, param_2);
    if (ctx.ax_reg != 0) {
      // u_var3 = (param_1  >> 0x10);
        local_bx_28 = param_1;
        pu_var2 = local_bx_28.field_0xc;
        local_6 = pu_var2;
        write_to_file_1008_7898(ctx,param_2, pu_var2);
        if (pu_var2 != 0) {
            offset_1 = local_bx_28.field_0x10;
            b_var1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_1), 4);
            if (b_var1 != 0) {
                offset_2 = CONCAT22(offset_2, local_bx_28.field_0x18);
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

pub fn write_to_file_1030_de7c(param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_10: u32;

    write_to_file_1028_b5ec(param_1, param_2);
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

    write_to_file_1028_b5ec(param_1, param_2);
    if (ctx.ax_reg != 0) {
        local_4 = 0;
        while (local_4 < 10) {
          // local_es_61 = (param_1  >> 0x10);
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

pub fn write_to_file_1030_c230(param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_10: u32;
    let mut local_8: u16;

    write_to_file_1028_b5ec(param_1, param_2);
    if (ctx.ax_reg != 0) {
      // u_var2 = (param_1  >> 0x10);
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

pub unsafe fn write_to_file_1030_b768(param_1: &mut  Struct961, param_2: &HFILE16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let pu_var3: Vec<u8>;
    let pu_var4: Vec<u8>;

    let local_bx_4: &mut  Struct961;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_22: u16;
    let mut local_1a: [u8; 10];
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

  // u_var5 = (param_1  >> 0x10);
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

    write_to_file_1030_16d6(struct_param_1, hfile_param_2);
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
  // local_es_52 = (struct_param_1  >> 0x10);
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

pub unsafe fn write_to_file_1030_5dbe(struct_param_1: &mut Struct771, hfile_param2: &HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let b_var4: bool;
    let pu_var5: Vec<u8>;
    // let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    i_var3 = write_to_file_1030_1978(struct_param_1, hfile_param2);
    if i_var3 != 0 {
      // u_var7 = (struct_param_1  >> 0x10);
        // i32_var6 = struct_param_1;
        // (i32_var6 + 0x10)
        write_to_file_1008_7c2a(ctx, hfile_param2, &mut struct_param_1.string_field_0x10);
        if i_var3 != 0 {
            u_var1 = (i32_var6 + 0x10);
            pu_var5 = (u_var1 & 0xffff0000 | (u_var1 + 4));
            write_file_1008_7b4c(hfile_param2, pu_var5);
            if (pu_var5 != 0) {
                u_var2 = (i32_var6 + 0x10);
                local_c = (u_var2 + 10);
                b_var4 = write_to_file_1008_7e1c(hfile_param2, CONCAT22(unaff_ss, &local_c), 2);
                if (b_var4 != 0) {
                    u_var2 = (i32_var6 + 0x10);
                    if ((u_var2 + 10) == 0) {
                        return;
                    }
                    u_var2 = (i32_var6 + 0x10);
                  // u_var7 = (u_var2  >> 0x10);
                    i_var3 = u_var2;
                    b_var4 = write_to_file_1008_7e1c(hfile_param2, (i_var3 + 0xc), ((i_var3 + 10) * 2));
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

pub fn write_to_file_1030_5c1a(param_1: &mut string, param_2: &HFILE16) -> bool {
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

pub unsafe fn write_to_file_1030_56f6(struct_param_1: &mut FileObject, hfile_param_2: &HFILE16) {
    let pu_var1: &mut  u16;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let b_var5: bool;
    let mut i32_var6: i32;
    let pu_var7: Vec<u8>;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    i_var4 = write_to_file_1030_1978(struct_param_1, hfile_param_2);
    if i_var4 != 0 {
      // u_var8 = (struct_param_1  >> 0x10);
        i_var4 = struct_param_1;
        local_e = *(i_var4 + 0x10);
        b_var5 = write_to_file_1008_7e1c(hfile_param_2, &local_e, 2);
        if (b_var5 != 0) {
            u_var3 = (i_var4 + 0x10);
            local_8 = (u_var3 + 2);
            b_var5 = write_to_file_1008_7e1c(hfile_param_2, &local_e, 2);
            if ((b_var5 != 0)
                && (
                u_var3 = (i_var4 + 0x10),
                write_to_file_1008_7c2a(ctx, hfile_param_2, (u_var3 + 4)),
                b_var5 != 0,
                ))
            {
                u_var3 = (i_var4 + 0x10);
                local_8 = (u_var3 + 0x1a);
                b_var5 = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(unaff_ss, &local_8), 2);
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
                        write_file_1008_7b4c(hfile_param_2, (u_var2 & 0xffff0000 | (u_var2 + i32_var6)));
                        if (i32_var6 == 0) {}
                        // goto LAB_1030_5734;
                        local_4 = local_4 + 1;
                    }
                    pu_var7 = (struct_param_1 & 0xffff0000 | (i_var4 + 0x14));
                    write_file_1008_7b4c(hfile_param_2, pu_var7);
                    if (pu_var7 != 0) {
                        local_8 = (i_var4 + 0x1c);
                        b_var5 = write_to_file_1008_7e1c(hfile_param_2, CONCAT22(unaff_ss, &local_8), 2);
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

pub fn write_to_file_1030_32e4(param_1: u32, param_2: &HFILE16) {
    let local_AX_14: &mut  Struct873;
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
                  // u_var2 = (param_1  >> 0x10);
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

pub unsafe fn write_to_file_1030_2aca(param_1: &mut FileObject, param_2: &HFILE16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let b_var4: bool;
    let pu_var5: Vec<u8>;
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
  // u_var7 = (param_1  >> 0x10);
    i_var3 = param_1;
    local_c = *(i_var3 + 0x10);
    b_var4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
    if ((b_var4 != 0)
        && (
        u_var2 = (i_var3 + 0x10),
        write_to_file_1008_7c2a(ctx, param_2, (u_var2 + 2)),
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
                                  // u_var8 = (u_var2  >> 0x10),
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

pub fn write_to_file_1030_1978(param_1: &mut FileObject, param_2: &HFILE16) -> i32 {
    let mut i_var1: i32;

    i_var1 = write_to_file_1030_16d6(param_1, param_2);
    if i_var1 != 0 {
        i_var1 = write_to_file_1008_7954(ctx, param_2, (param_1 + 0xc));
        if i_var1 == 0 {
            ctx.g_u16_1050_0310 = 0x6d0;
            return i_var1;
        }
        i_var1 = 1;
    }
    return i_var1;
}

pub fn write_to_file_1030_16d6(param_1: &mut FileObject, param_2: &HFILE16) {
    let b_var1: bool;
    let mut u_var2: u16;
    let mut local_10: u32;
    let mut local_8: u32;

  // u_var2 = (param_1  >> 0x10);
    local_10 = (param_1 + 4);
    b_var1 = write_to_file_1008_7e1c(param_2, &local_1, 4);
    if b_var1 != false {
        local_8 = (param_1 + 8);
        b_var1 = write_to_file_1008_7e1c(param_2, &local_8, 4);
        if b_var1 != false {
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn write_to_file_1028_e56c(
    param_1: u16,
    param_2: u16,
    param_1_00: &HFILE16,
    param_2_00: u32,
) {
    let pp_var1: fn();
    let p_uvar2: &mut  u16;
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

pub unsafe fn write_to_file_1028_dce2(param_1: &mut  u32, param_2: &HFILE16) {
    let pp_var1: fn();
    let u_var2: u8;
    let BVar3: bool;
    let mut i_var4: i32;
    let pu_var5: &mut  u16;
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
          // u_var7 = (param_1  >> 0x10);
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

pub fn write_to_file_1028_b5ec(in_struct_1: &mut FileObject, in_file_handle: &HFILE16) -> bool {
    let mut u_var1: u32;
    let b_var2: bool;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_4: u16;

  // u_var5 = (in_struct_1  >> 0x10);
    i_var4 = in_struct_1;
    local_e = (i_var4 + 0xc);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, &local_e, 2);
    if b_var2 == false {
        ctx.g_u16_1050_0310 = 0x6d0;
        return false;
    }
    i_var3 = write_to_file_1030_16d6(in_struct_1, in_file_handle);
    if i_var3 == 0 {
        return false;
    }
    local_8 = (i_var4 + 0xc);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0xe);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x10);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x12);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x18);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (i_var4 + 0x1a);
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    if (b_var2 == 0) {
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
                b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
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
            b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (b_var2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xa6);
            b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (b_var2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xa8);
            b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (b_var2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xaa);
            b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
            if (b_var2 == 0) {
                ctx.g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            u_var1 = (i_var4 + 0x14);
            local_8 = (u_var1 + 0xac);
        }
    }
    b_var2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_ss, &local_8), 2);
    // joined_r0x1028b766:
    if (b_var2 == 0) {
        ctx.g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    return 1;
}

pub unsafe fn write_to_file_1028_64d6(param_1: &mut FileObject, param_2: &HFILE16) -> u16 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let pu_var3: &mut  u16;
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
      // u_var5 = (param_1  >> 0x10);
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

pub fn write_to_file_1028_5f82(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
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

pub fn write_file_func_1028_4a1a(param_1: &mut FileObject, param_2: &HFILE16) {

    let b_var1: bool;

    write_to_file_1028_b5ec(param_1, param_2);
    if ((ctx.ax_reg != 0)
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

pub fn write_to_file_1028_3d0e(in_struct_1: &mut FileObject, in_file_handle: &HFILE16) {

    let bool_res: bool;
    let local_struct_1: &mut FileObject;
    let mut local_struct_1_hi: u16;

    let local_10: Vec<u8>;
    let local_8: Vec<u8>;

    write_to_file_1028_b5ec(in_struct_1, in_file_handle);
    if (ctx.ax_reg != 0) {
      // local_struct_1_hi = (in_struct_1  >> 0x10);
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

pub unsafe fn write_file_fn_1028_2418(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
    let mut u_var1: u32;
    let b_var2: bool;
    let mut u_var3: u16;
    // let mut unaff_ss: u16;
    let mut struct_var4: Struct1167;
    let mut local_1c: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    b_var2 = write_to_file_1028_b5ec(param_1, param_2);
    if b_var2 != 0 {
        //// _var3 = (param_1  >> 0x10);
        pass1_1008_5784(local_a, (param_1 + 0x20));
        u_var1 = (param_1 + 0x20);
        local_1c = (u_var1 + 8);
        local_10 = local_1c;
        b_var2 = write_to_file_1008_7e1c(param_2, &local_1c, 2);
        if b_var2 == 0 {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var2;
        }
        loop {
            struct_var4 = pass1_1008_5b12(local_a);
            if struct_var4 == 0 {
                break;
            }
            _local_e = struct_var4;
            b_var2 = write_to_file_1038_75ca(ctx, &mut struct_var4, param_2);
            if b_var2 == 0 {
                return b_var2;
            }
        }
        b_var2 = 1;
    }
    return b_var2;
}

pub fn file_write_fn_1028_1452(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
    let mut i_var1: i32;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    i_var1 = write_to_file_1028_b5ec(param_1, param_2);
    if (i_var1 != 0) {
      // u_var3 = (param_1  >> 0x10);
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

pub unsafe fn file_write_fn_1028_0234(param_1: &mut Struct731, param_2: u32) -> bool {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let BVar3: bool;
    // let local_bx_28: Struct731;
    let mut u_var4: u16;
    // let mut unaff_ss: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    i_var2 = write_to_file_1028_b5ec(param_1, param_2);
    if i_var2 != 0 {
      // u_var4 = (param_1  >> 0x10);
        local_bx_28 = param_1;
        local_1a = local_bx_28.field_0x20;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1a), 2);
        if BVar3 != 0 {
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

pub fn file_write_fn_1020_e94e(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
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

pub fn file_write_fn_1020_e6a4(param_1: &mut FileObject, param_2: &HFILE16) -> u16 {
    let mut i_var1: i32;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    i_var1 = write_to_file_1030_de7c(param_1, param_2);
    if (i_var1 != 0) {
      // u_var3 = (param_1  >> 0x10);
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

pub fn call_write_to_file_1020_d3d4(struct_param_1: &mut FileObject, hfile_param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write_to_file_1028_b5ec(struct_param_1, hfile_param_2);
    if b_var1 != false {
        local_c = (struct_param_1 + 0x20);
        // CONCAT22(unaff_ss, &local_c)
        b_var1 = write_to_file_1008_7e1c(hfile_param_2, &local_c, 2);
        if b_var1 == 0 {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = true;
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

pub fn write_to_file_1030_1a9c(
    ctx: &mut AppContext,
    param_1: &mut FileObject,
    param_2: &HFILE16,
) -> bool {
    let pi_var1: &mut  i32;
    let mut i_var2: i32;
    let BVar3: bool;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    i_var2 = write_to_file_1030_1978(param_1, param_2);
    if (i_var2 != 0) {
      // u_var4 = (param_1  >> 0x10);
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
