use crate::err_ops::error_check_1000_17ce;
use crate::pass::pass14_funcs::{pass1_1008_5784, pass1_1008_5b12, pass1_fn_1008_60e8};
use crate::pass::pass8_funcs::{pass1_1010_5f4c, pass1_1010_60cc, process_struct_1010_20ba};
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_fn_1000_5008};
use crate::string_ops::misc::{copy_string_1000_3d3e, fn_1008_6048, get_string_index_1000_3da4, process_string_1000_3cea, process_string_1000_3dbe, process_string_1000_4d58};
use crate::sys_ops::dos_ops::dos3_call_1000_4f2e;
use crate::ui_ops::misc;
use crate::util::{CONCAT12, CONCAT13, CONCAT22, SUB21, ZEXT24};
use crate::winapi::MessageBox16;

pub fn open_save_1008_30cc(param_1: u32) {
    let mut ctx.stack_seg_reg: i32;
    let in_string_2: String;
    let mut local_DXAX_125: u32;
    let mut local_218: i32;
    let mut local_216: i32;
    let mut local_214: u16;
    let mut local_212: u16;
    let mut string_1: [u8; 10];
    let mut string_2: [u8; 256];
    let mut local_106: u16;
    let mut local_104: u16;
    let mut string_3: [u8; 256];

    string_3[0] = '\0';
    in_string_2 = open_save_fn_1008_3178(param_1, 2);
    if (in_string_2 != 0x0) {
        copy_string_1000_3d3e(CONCAT22(ctx.stack_seg_reg, string_3), in_string_2);
        process_string_1000_4d58(string_3);
        if (string_1[0] != '\0') {
            process_string_1000_3cea(
                CONCAT22(ctx.stack_seg_reg, string_2),
                CONCAT22(ctx.stack_seg_reg, string_1),
            );
        }
        local_DXAX_125 =
            process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(string_2, 2));
        pass1_1010_5f4c(local_DXAX_125, CONCAT22(ctx.stack_seg_reg, string_2));
        if (string_3[0] != '\0') {
            misc::win_fn_1008_12dc(param_1, string_3, ctx.stack_seg_reg);
        }
    }
    return;
}

pub fn open_save_fn_1008_3178(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let p_uvar2: &mut  u16;
    let pu_var3: Vec<u8>;
    let mut i_var4: i32;
    let pu_var5: &mut  u32;
    let mut u_var6: u32;



    let mut u_var7: u16;


    let mut u_var8: u16;
    let mut extraout_dx_04: u16;
    let mut extraout_dx_05: u16;
    let mut unaff_si: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: i32;
    let mut local_786: u32;
    let mut local_782: [u8; 260];
    let mut local_67e: u16;
    let mut local_676: u16;
    let mut local_674: u16;
    let mut local_672: u16;
    let mut local_670: u16;
    let mut local_66e: u32;
    let local_666: u8;
    let mut local_566: u32;
    let mut local_562: u32;
    let mut local_55e: u16;
    let mut local_55a: u16;
    let mut local_558: u16;
    let mut local_54a: u16;
    let mut local_548: u16;
    let mut local_546: u32;
    let mut local_542: u16;
    let mut local_540: u16;
    let mut local_53e: u32;
    let mut local_53a: u16;
    let mut local_538: u16;
    let mut local_536: u16;
    let mut local_534: u16;
    let mut local_532: u32;
    let mut local_52e: u16;
    let mut local_52a: u16;
    let mut local_528: u16;
    let local_51a: u8;
    let local_519: u8;
    let local_518: u8;
    let mut local_418: u16;
    let local_416: u8;
    let mut local_415: [u8; 7];
    let mut local_40e: u16;
    let mut local_40c: [u8; 258];
    let mut local_30a: u32;
    let mut local_306: u16;
    let mut local_304: u16;
    let local_302: u8;
    let local_202: u8;
    let local_103: u8;
    let local_102: u8;

    local_102 = '\0';
    local_302 = '\0';
    local_202 = '\0';
    _local_306 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
  // u_var9 = (_local_306  >> 0x10);
    i_var4 = _local_306;
    local_30a = (i_var4 + 0x1a);
    u_var10 = (i_var4 + 0x1c);
    if ((u_var10 | local_30a) == 0) {
        local_66e = (i_var4 + 100);
        u_var10 = (i_var4 + 0x66);
        if ((u_var10 | local_66e) != 0) {
            pass1_1008_5784(
                CONCAT22(unaff_ss, &local_67e),
                local_66e & 0xffff | u_var10 << 0x10,
            );
            pu_var2 = &local_67e;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
            _local_676 = CONCAT22(ctx.dx_reg, pu_var2);
            if ((ctx.dx_reg | pu_var2) != 0) {
                u_var1 = (pu_var2 + 2);
                local_30a._0_2_ = u_var1;
              // u_var10 = (u_var1  >> 0x10);
                // goto LAB_1008_3206;
            }
        }
    } else {
        // LAB_1008_3206:
        copy_string_1000_3d3e(CONCAT22(unaff_ss, &local_102), CONCAT22(u_var10, local_30a));
    }
    pass1_fn_1000_5008(local_40c);
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_40c));
    if (local_40c[local_40e - 1] == '\\') {
        local_40c[local_40e - 1] = 0;
    }
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_102));
    if ((&local_103)[local_40e] == '\\') {
        (&local_103)[local_40e] = '\0';
    }
    dos3_call_1000_4f2e(&local_102);
  // u_var9 = (_local_306  >> 0x10);
    local_30a = (_local_306 + 0x12);
    u_var10 = (_local_306 + 0x14);
    pu_var3 = (u_var10 | local_30a);
    if (pu_var3 != 0x0) {
        pu_var3 = &local_202;
        copy_string_1000_3d3e(
            CONCAT22(unaff_ss, pu_var3),
            (local_30a & 0xffff | u_var10 << 0x10),
        );
    }
    local_416 = '\0';
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x579,
    );
    copy_string_1000_3d3e(
        CONCAT22(unaff_ss, &local_416),
        CONCAT22(ctx.dx_reg, pu_var3),
    );
    local_418 = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_416));
    u_var6 = local_418;
    local_40e = local_418;
    while (u_var9 = u_var6, -1 < local_418) {
        if ((&local_416)[local_418] == '.') {
            copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_67e),
                CONCAT22(unaff_ss, local_415 + local_418),
            );
            u_var6 = ZEXT24(&local_416);
            copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_416),
                CONCAT22(unaff_ss, &local_67e),
            );
        }
        local_418 = local_418 - 1;
    }
    local_518 = '\0';
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x74c,
    );
    u_var7 = copy_string_1000_3d3e(
        CONCAT22(unaff_ss, &local_518),
        CONCAT22(ctx.dx_reg, u_var9),
    );
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_518));
    local_51a = (&local_519)[local_40e];
    local_418 = 0;
    while ((&local_518)[local_418] != '\0') {
        if ((&local_518)[local_418] == local_51a) {
            (&local_518)[local_418] = '\0';
        }
        local_418 = local_418 + 1;
    }
    pass1_1000_4906(CONCAT22(unaff_ss, &local_562), 0, 0x48);
    local_562 = 0x48;
  // u_var9 = (param_1  >> 0x10);
    local_55e = (param_1 + 8);
    local_55a = &local_518;
    _local_54a = CONCAT22(unaff_ss, &local_202);
    local_542 = &local_302;
    local_546 = 0x100;
    local_53e = 0x100;
    local_53a = &local_102;
    local_52a = &local_416;
    local_566 = 0;
    local_666 = '\0';
    i_var4 = param_2 + -1;
  // u_var8 = (ctx._g_struct_73_1050_14cc  >> 0x10);
    if (i_var4 == 0) {
        local_532 = 0x1804;
        load_string_1010_847e(ctx._g_struct_73_1050_14cc, u_var8, 0x74d);
        copy_string_1000_3d3e(
            CONCAT22(unaff_ss, &local_666),
            CONCAT22(ctx.dx_reg, i_var4),
        );
        local_536 = &local_666;
        pu_var5 = &local_562;
        GetOpenFileName16(0x1000, pu_var5);
    } else {
        param_2 = param_2 + -2;
        if (param_2 != 0) {
            fn_1008_6048(
                s_Unsupported_FileStructType_in_Op_1050_01ca,
                u_var7,
                SUB21(param_2, 0),
            );
            // goto LAB_1008_3461;
        }
        local_532 = 6;
        load_string_1010_847e(ctx._g_struct_73_1050_14cc, u_var8, 0x74e);
        copy_string_1000_3d3e(
            CONCAT22(unaff_ss, &local_666),
            CONCAT22(extraout_dx_05, param_2),
        );
        local_536 = &local_666;
        pu_var5 = &local_562;
        GetSaveFileName16(0x1000, pu_var5);
    }
    if (pu_var5 != 0x0) {
        local_566 = _local_54a;
    }
    // LAB_1008_3461:
    if (local_566 != 0) {
        local_67e = local_52e;
        if (local_52e < 0) {
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x3fd,
            );
            _local_676 = CONCAT22(ctx.dx_reg, local_52e);
            u_var8 = ctx.dx_reg;
            pass1_fn_1008_60e8(local_52e, ctx.dx_reg);
            _local_676 = CONCAT22(u_var8, local_52e);
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x57b,
            );
            local_670 = extraout_dx_04;
            MessageBox16(
                0x10,
                CONCAT13((extraout_dx_04 >> 8), CONCAT12(extraout_dx_04, local_52e)),
                _local_676,
                (param_1 + 8),
            );
            local_566 = 0;
            local_66e = _local_676;
            error_check_1000_17ce(_local_676);
        } else {
            process_string_1000_3dbe(
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_782)),
                _local_54a,
                local_52e,
            );
            local_782[local_52e] = '\0';
            if (local_782[0] != '\0') {
                pass1_1010_60cc(_local_306, CONCAT22(unaff_ss, local_782));
            }
        }
    }
    dos3_call_1000_4f2e(local_40c);
    return;
}
