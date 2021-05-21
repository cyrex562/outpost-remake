use crate::err_funcs::error_check_1000_17ce;
use crate::mem_funcs::alloc_mem_1008_909c;
use crate::pass::{pass14_funcs, pass_funcs};
use crate::pass::pass17_funcs::pass1_1030_8344;
use crate::pass::pass18_funcs::pass1_1038_3608;
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::pass::pass6_funcs::pass1_1038_4d28;
use crate::pass::pass7_funcs::{pass1_1018_4cda, pass1_1018_4dce};
use crate::pass::pass8_funcs::{pass1_1010_1d80, pass1_1010_1f62};
use crate::prog_structs::prog_structs_11::{Struct218, Struct222};
use crate::prog_structs::prog_structs_2::{Struct199, Struct223};
use crate::prog_structs::prog_structs_24::Struct2111;
use crate::prog_structs::prog_structs_25::{Struct211, Struct219};
use crate::prog_structs::prog_structs_27::pass1_struct_2;
use crate::prog_structs::prog_structs_28::{Struct207, Struct224};
use crate::prog_structs::prog_structs_29::{Struct212, Struct213, Struct220, Struct221};
use crate::prog_structs::prog_structs_7::{Struct376, Struct44};
use crate::sound_funcs::mci_send_cmd_1008_5c5c;
use crate::struct_ops1::{process_struct_1000_179c, process_struct_1008_4772, process_struct_1008_50c2, process_struct_1008_574a, process_struct_1010_1d48};
use crate::ui_ops::misc::win_cleanup_1018_4d22;
use crate::util::{CONCAT12, CONCAT13, CONCAT22, CONCAT31};

pub unsafe fn pass1_1008_8ce4(param_1: *mut Struct207, param_2: u32, param_3: u32) {
    let local_AX_120: Vec<u8>;
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
    pass14_funcs::pass1_1008_3e54(
        CONCAT22(ctx.stack_seg_reg, local_10),
        0,
        &local_bx_4.field_0x12,
        &local_bx_4.field_0x10,
    );
    local_AX_120 = local_10;
    pass14_funcs::pass1_1008_3f32(param_2, local_AX_120, ctx.stack_seg_reg);
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
    pass14_funcs::pass1_1008_5134(CONCAT22(u_var2, local_AX_120));
    pass14_funcs::pass1_1008_4480(param_3, param_2, local_bx_4.Struct104_field_4);
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

pub unsafe fn bad_func_1008_8fc4(param_1: Vec<u8>, param_2: u32) {
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

pub unsafe fn pass1_1008_914a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1008_8f24(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_92b2(param_1: u32, param_2: libc::c_long, param_3: libc::c_long) {
    let pp_var1: fn();
    let local_AX_35: Vec<u8>;


    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    pass14_funcs::pass1_1008_57a4(
        CONCAT22(ctx.stack_seg_reg, local_c),
        param_1 & 0xffff0000 | (param_1 + 6),
    );
    while (true) {
        local_AX_35 = local_c;
        pass14_funcs::pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_35));
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
    let local_AX_44: Vec<u8>;

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
        pass14_funcs::pass1_1008_57a4(
            CONCAT22(ctx.stack_seg_reg, local_a),
            param_1 & 0xffff0000 | (i_var3 + 6),
        );
        while (true) {
            local_AX_44 = local_a;
            pass14_funcs::pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_44));
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

pub unsafe fn pass1_1008_9f8c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
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
    let pu_var2: Vec<u8>;

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
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var4 + 0x410));
    while {
        pu_var2 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
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

pub unsafe fn pass1_1008_ab12(param_1: *mut Struct2111, param_2: u16, param_1_00: u8) {
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

pub unsafe fn pass1_1008_aefe(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u16) {
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

pub unsafe fn pass1_1008_af56(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
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
        pass14_funcs::pass1_fn_1008_60e8(param_2);
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
            pass14_funcs::pass1_fn_1008_60e8();
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
