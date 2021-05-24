use crate::app_context::AppContext;
use crate::draw::misc;
use crate::file_ops::close::close_file_1008_496c;
use crate::other_funcs::mixed_fn_1010_830a;
use crate::pass::pass13_funcs::bad_func_1008_8fc4;
use crate::pass::pass14_funcs::{pass1_1008_3e94, pass1_1008_3f92, pass1_1008_4480, pass1_1008_5134, pass1_1008_5236};
use crate::pass::pass20_funcs::pass1_1018_0afa;
use crate::pass::pass7_funcs::{pass1_1018_1eda, pass1_1018_265c, pass1_1018_266a, pass1_1018_268e, pass1_1018_2862, pass1_1018_31d0};
use crate::pass::pass8_funcs::{pass1_1010_209e, pass1_1010_2b66, process_struct_1010_20ba};
use crate::string_ops::misc::string_fn_1000_3f9c;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_41bc, process_struct_1008_4772, process_struct_1008_48fe, process_struct_1008_50c2};
use crate::structs::prog_structs_1::{Struct197, Struct538, Struct693, Struct694};
use crate::structs::prog_structs_16::Struct13;
use crate::structs::prog_structs_2::{Struct199, Struct318, Struct665};
use crate::structs::prog_structs_22::Struct69;
use crate::structs::prog_structs_3::Struct664;
use crate::structs::prog_structs_4::Struct653;
use crate::structs::prog_structs_8::Struct642;
use crate::sys_structs::{PAINTSTRUCT16, POINT16, RECT16};
use crate::typedefs::{HGDIOBJ16, HWND16};
use crate::ui_ops::misc::mixed_1040_8520;
use crate::util::{CONCAT22, SUB42, ZEXT24};
use crate::winapi::{BeginPaint16, CreatePen16, CreateSolidBrush16, DeleteObject16, EndPaint16, FillRect16, GetClientRect16, GetStockObject16, GetSystemMetrics16, GetWindowRect16, InvalidateRect16, PtInRect16, Rectangle16, ScreenToClient16, SelectObject16, SelectPalette16, ValidateRect16};

pub fn invalidate_rect_1018_5d32(param_1: u32, param_2: i32) {
    let mut hwnd: HWND16;

  // hwnd = (param_1  >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 2) {
        return;
    }
    InvalidateRect16(0, (param_1 + 0x22), hwnd);
    return;
}

pub unsafe fn pt_in_rect_1018_1bda(param_1: &mut  Struct318, param_2: u16, param_3: u16) {
    let pu_var1: &mut  u16;
    let mut i_var2: i32;
    let b_var3: bool;
    let paVar4: &mut  Struct199;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_e = 0;
    i_var2 = param_1;
    pass1_1008_3e94(
        (i_var2 + 0x3a),
        CONCAT22(unaff_ss, &local_14),
        CONCAT22(unaff_ss, &local_12),
    );
    let local_18 = CONCAT22(param_2, param_3);
    local_10 = 0;
    local_1a = 0;
    while (true) {
      // u_var5 = (param_1  >> 0x10);
        pu_var1 = (i_var2 + 0x44);
        let var = unsafe { *pu_var1 };
        if (var == local_1a || var < local_1a) {
            return;
        }
        paVar4 = ((i_var2 + 0x40) + local_1a * 0x18);
        local_e = CONCAT22((i_var2 + 0x42), paVar4);
        pass1_1008_3e94(
            paVar4,
            CONCAT22(unaff_ss, &local_8),
            CONCAT22(unaff_ss, &local_a),
        );
        local_a = local_a + (local_12 - 6);
        local_6 = local_a + 0xc;
        local_8 = local_8 + (local_14 - 6);
        local_4 = local_8 + 0xc;
        b_var3 = PtInRect16(local_18, &local_a);
        if (b_var3 != 0) {
            break;
        }
        local_1a = local_1a + 1;
    }
    pass1_1018_1eda(param_1, local_e);
    return;
}

pub unsafe fn pt_in_rect_1010_40f8(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: &mut  POINT16,
) -> u16 {
    let pi_var1: &mut  i32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let pp_var8: &mut  Struct2551;
    let pu_var9: &mut  u16;
    let mut in_stack_0000ffec: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    loop {
      // u_var7 = (param_1  >> 0x10);
        i32_var6 = param_1;
        pi_var1 = (i32_var6 + 0x74);
        let pi_var1_val = unsafe { *pi_var1 };
        if (pi_var1_val == local_6 || pi_var1_val < local_6) {
            // LAB_1010_413e:
            if ((local_6._2_2_ != 0) && (3 < ctx.PTR_LOOP_1050_3960)) {
                pp_var8 = process_struct_1010_20ba(
                    ctx.g_struct_var_1050_0ed0,
                    CONCAT22(in_stack_0000ffec, local_6 + 0xc),
                );
                pu_var9 = pass1_1018_0afa(pp_var8);
                if (pu_var9 == 0) {
                    local_6 = local_6 & 0xffff;
                    u_var7 = 0x1000;
                    process_struct_1000_179c(0xb4, (pu_var9 >> 0x10));
                  // u_var5 = (pu_var9  >> 0x10) | pu_var9;
                    if (pu_var9 == 0x0) {
                        u_var4 = 0;
                        u_var5 = 0;
                    } else {
                        u_var7 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                        u_var4 = mixed_1040_8520(pu_var9, ctx.g_h_window, 0x30, 2, 0x643, 0x645);
                    }
                    let local_10 = CONCAT22(u_var5, u_var4);
                    ppc_var2 = (*local_10 + 0x74);
                    ppc_var2(u_var7, u_var4, u_var5);
                    pass1_1010_209e(ctx.g_struct_var_1050_0ed0, local_6 + 0xc);
                }
            }
            if (local_6._2_2_ != 0) {
                return local_6;
            }
            return 0xffff;
        }
        let param_2_val = unsafe { param_2 };
        b_var3 = PtInRect16(
            param_2_val,
            ((local_6 * 0x10 + (i32_var6 + 0x24)) * 8 + (i32_var6 + 0x70)),
        );
        if (b_var3 != 0) {
            local_6 = CONCAT22(1, local_6);
            // goto LAB_1010_413e;
        }
        local_6 = (local_6 + 1);
    }
}

pub fn pt_in_rect_1010_4e08(in_struct_1: &mut  Struct642, param_2: u16, param_3: u16) {
    let pi_var1: &mut  i32;
    let mut b_var2: bool;
    let mut point_in_rect_result: bool;
    let local_struct_1: &mut  Struct642;
    let local_struct_1_hi: &mut  Struct642;
    let mut local_c: u32;
    let mut point_1: POINT16;
    let mut local_4: u16;

    point_1 = CONCAT22(param_2, param_3);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    &local_struct_1.field_0x22 = local_struct_1.field_0x20;
    b_var2 = false;
    local_struct_1.field_0x24 = 0;
    local_c = 0;
    loop {
        pi_var1 = local_struct_1.field_0x30;
        if (pi_var1 == local_c || pi_var1 < local_c) {
            // LAB_1010_4e67:
            if (local_c._2_2_ != 0) {
                local_struct_1.field_0x20 = local_c._2_2_;
            }
            if (b_var2) {
                return;
            }
            return;
        }
        point_in_rect_result = PtInRect16(point_1, (local_struct_1.field_0x1a + local_c * 8));
        if (point_in_rect_result != 0) {
            local_c = local_c << 0x10;
            b_var2 = true;
            // goto LAB_1010_4e67;
        }
        local_c = (local_c + 1);
    }
}

pub unsafe fn invalidate_rect_1018_edd8(param_1: u32, param_2: i32) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: HWND16;
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

    iVar1 = param_1;
  // u_var2 = (param_1  >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xc) {
        return;
    }
    local_8 = iVar1 + 0x18;
    pass1_1008_3e94(
        local_8,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    local_a = pass1_1010_2b66((iVar1 + 0x14));
    local_e = process_struct_1008_4772(CONCAT22(local_8, local_a));
  // u_var2 = (local_e  >> 0x10);
    local_16 = local_4;
    local_14 = local_6;
    local_12 = local_4 + (local_e + 4);
    local_10 = local_6 + (local_e + 8);
    InvalidateRect16(0, &local_16, unaff_ss);
    return;
}

pub fn call_fill_rect_1020_041e(in_struct_1: &mut  Struct13) {
    fill_rect_1020_065e((in_struct_1 + 0xe6));
    return;
}

pub fn fill_rect_1020_065e(param_1: &mut  Struct13) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u32;
    let local_bx_6: &mut  Struct13;
    let mut u_var4: u16;
    let mut h_dc: u16;
    let mut rectangle: RECT16;
    let mut brush: u16;
    let mut palette: u16;
    let mut local_28: u32;
    let mut result: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_6 = param_1;
    result = BeginPaint16(CONCAT22(h_dc, &local_22), local_bx_6.field_0x4);
    if (0x280 < local_bx_6.field_0xa) {
        brush = CreateSolidBrush16(0x210070b);
        rectangle.left = 0;
        rectangle.top = 0;
        rectangle.right = local_bx_6.field_0xa + -1;
        rectangle.bottom = local_bx_6.field_0xc + -1;
        FillRect16(brush, &rectangle, h_dc);
        DeleteObject16(brush);
    }
    u_var2 = local_bx_6.field_0x6;
    local_28 = (u_var2 + 0xe);
    palette = &result;
    u_var3 = local_28;
    pp_var1 = (u_var3 + 8);
    (**pp_var1)(offset, local_28, (local_28 >> 0x10), palette);
    pp_var1 = (u_var3 + 4);
    (**pp_var1)(
        offset,
        local_28,
        local_bx_6.field_0x10,
        local_bx_6.field_0xe,
        &result,
    );
    palette = SelectPalette16(0, palette, result);
    DeleteObject16(palette);
    EndPaint16(&local_22, h_dc);
    return;
}

pub fn pt_in_rect_1040_8158(param_1: &mut  Struct69, param_2: u32, param_3: u16) {
    let pp_var1: fn();
    let mut i_var2: u16;
    let b_var3: bool;
    let local_bx_10: &mut  Struct69;
    let mut unaff_cs: u16;
    let mut unaff_ss: HWND16;
    let mut local_6: u32;
    let mut offset: u16;

    if (param_3 == 2) {
        local_bx_10 = param_1;
        if (local_bx_10.field_0x76 != 0) {
            local_6 = param_2;
            ScreenToClient16(CONCAT22(&local_6, unaff_cs), unaff_ss);
            i_var2 = GetSystemMetrics16(4);
            local_6 = local_6 & 0xffff | (local_6._2_2_ + i_var2) << 0x10;
            b_var3 = PtInRect16(local_6, (local_bx_10 + 1));
            if (b_var3 != 0) {
                pp_var1 = (param_1 + 0x14);
                (**pp_var1)(offset, param_1, 0);
            }
        }
    }
    return;
}

pub unsafe fn call_invalidate_rect_1020_8bcc(ctx: &mut AppContext, in_struct_1: &mut  Struct693) {
    let paVar1: &mut  Struct197;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let paVar4: &mut  Struct199;



    let local_struct_1: &mut  Struct693;
    let local_bx_310: &mut  Struct694;
    let local_struct_1_hi: &mut  Struct693;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 30];
    let mut local_3a: [u8; 38];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let local_struct_2: &mut  Struct538;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x4 != 0) {
        local_struct_2 = local_struct_1.struct538_ptr_0x22;
        local_6 = (local_struct_2 + 10);
        local_a = pass1_1018_268e(local_struct_1.struct538_ptr_0x22);
        local_struct_2 = local_struct_1.struct538_ptr_0x22;
        local_c = (local_struct_2 + 0x16);
        if (local_struct_1.field_0xc == 0) {
            local_14 = process_struct_1008_4772(local_a);
          // paVar4 = (local_14  >> 0x10);
            u_var2 = local_14;
            process_struct_1000_179c(0x14, paVar4);
            local_5c = CONCAT22(paVar4, u_var2);
            if ((paVar4 | u_var2) == 0) {
                local_struct_1.field_0xc = 0;
            } else {
                u_var3 = in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16;
              // u_var5 = (local_14  >> 0x10);
                process_struct_1008_50c2(local_5c, (local_14 + 8), (local_14 + 4), u_var3, local_6);
                paVar1 = local_struct_1.field_0xc;
                u_var2 = u_var3;
                paVar1 = u_var2;
                (paVar1 + 2) = ctx.dx_reg;
            }
            pass1_1008_5134(local_struct_1.field_0xc);
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 2);
            process_struct_1008_48fe(
                CONCAT22(unaff_ss, local_3a),
                1,
                CONCAT22(ctx.dx_reg, u_var2),
            );
            pass1_1008_3f92(CONCAT22(unaff_ss, local_58), CONCAT22(unaff_ss, local_3a));
            local_14 = process_struct_1008_4772(CONCAT22(unaff_ss, local_58));
          // paVar4 = (local_14  >> 0x10);
            u_var2 = local_14;
            process_struct_1000_179c(0x14, paVar4);
            local_5c = CONCAT22(paVar4, u_var2);
            if ((paVar4 | u_var2) == 0) {
                paVar1 = local_struct_1.field_0xc;
                (paVar1 + 4) = 0;
            } else {
                u_var3 = in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16;
              // u_var5 = (local_14  >> 0x10);
                process_struct_1008_50c2(local_5c, (local_14 + 8), (local_14 + 4), u_var3, local_6);
                paVar1 = local_struct_1.field_0xc;
              // u_var5 = (paVar1  >> 0x10);
                local_bx_310 = paVar1;
                local_bx_310.field_0x4 = u_var3;
                local_bx_310.field_0x6 = ctx.dx_reg;
            }
            paVar1 = local_struct_1.field_0xc;
            pass1_1008_5134((paVar1 + 4));
            process_struct_1008_41bc(CONCAT22(unaff_ss, local_58));
            close_file_1008_496c(local_3a);
        }
        paVar1 = local_struct_1.field_0xc;
        pass1_1008_5236((paVar1 + 4));
        pass1_1008_5236(local_struct_1.field_0xc);
        pass1_1008_4480(
            local_6,
            (in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16),
            local_a,
        );
        invalidate_rect_1020_8d90(in_struct_1, local_c, local_6, (local_6 >> 0x10));
    }
    return;
}

pub unsafe fn invalidate_rect_1020_8d90(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_3: u32,
) {
    let mut u_var1: u32;
    let mut in_dx: i32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: String;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 40];
    let mut local_10: [u8; 10];
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    local_6 = pass1_1018_266a((i_var2 + 0x22));
    if (local_6 != 0x0) {
        pass1_1018_265c((i_var2 + 0x22));
        if (in_dx | local_6) != 0 {
            string_fn_1000_3f9c(
                local_10,
                unaff_ss,
                ctx.s__03ld_1050_442a,
                &ctx.g_alloc_addr_1050_1050,
                local_6,
            );
            u_var1 = (i_var2 + 0x22);
            u_var1 = (u_var1 + 0xe);
            misc::draw_1008_4f20(
                local_38,
                unaff_ss,
                u_var1,
                (u_var1 >> 0x10),
                0x25,
                local_10,
                unaff_ss,
            );
            pass1_1008_4480(
                param_3,
                (param_1 & 0xffff0000 | (i_var2 + 0x1c)),
                CONCAT22(unaff_ss, local_38),
            );
            local_3c = process_struct_1008_4772(CONCAT22(unaff_ss, local_38));
            pass1_1008_3e94(
                (i_var2 + 0x1c),
                CONCAT22(unaff_ss, &local_40),
                CONCAT22(unaff_ss, &local_3e),
            );
            local_48 = local_3e;
            local_46 = local_40;
          // u_var3 = (local_3c  >> 0x10);
            local_44 = local_3e + (local_3c + 4);
            local_42 = local_40 + (local_3c + 8);
            InvalidateRect16(0, &local_48, unaff_ss);
            process_struct_1008_41bc(CONCAT22(unaff_ss, local_38));
        }
    }
    return;
}

pub fn fill_rect_1008_62c0(param_1: u32) {
    let mut u_var1: u16;
    let unaff_ss: HWND16;
    let local_2e: RECT16;
    let mut local_26: u16;
    let mut local_24: u16;
    let local_22: PAINTSTRUCT16;

  // u_var1 = (param_1  >> 0x10);
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (param_1 + 8));
    local_26 = CreateSolidBrush16(0x210070b);
    GetClientRect16(CONCAT22(unaff_ss, &local_2e), (param_1 + 8));
    FillRect16(local_26, &local_2e, unaff_ss);
    EndPaint(&local_22, unaff_ss);
    DeleteObject16(local_26);
    return;
}

pub fn draw_rect_1020_3488(param_1: &mut Struct653, in_h_dc: u16) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let paVar4: &mut Struct199;
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let mut stock_obj_5: u16;
    let obj_handle: HGDIOBJ16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut h_dc: u16;
    let mut left: i32;
    let mut h_dc_2: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

  // u_var5 = (param_1  >> 0x10);
    u_var2 = (param_1 + 0x14);
    paVar4 = (u_var2 + 0x36);
    local_6 = u_var2 & 0xffff0000 | ZEXT24(paVar4);
    pass1_1008_3e94(
        paVar4,
        CONCAT22(unaff_ss, &local_a),
        CONCAT22(unaff_ss, &local_8),
    );
    u_var2 = (local_8 - 3) << 0x10;
    if ((local_8 - 3) < 0) {
        u_var2 = 0;
    }
    u_var1 = _local_a - 3;
    _local_a = u_var1;
    if (u_var1 < 0) {
        _local_a = 0;
    }
    _local_a = u_var2 | _local_a;
    u_var3 = (param_1 + 0x14);
    h_dc = in_h_dc;
    pen = CreatePen16((u_var3 + 100), 1, 0);
    pen_obj_handle = SelectObject16(pen, h_dc);
    stock_obj_5 = GetStockObject16(5);
    obj_handle = SelectObject16(stock_obj_5, h_dc_2);
  // left = (_local_a  >> 0x10);
    Rectangle16(_local_a + 6, left + 6, _local_a, left, in_h_dc);
    SelectObject16(pen_obj_handle, in_h_dc);
    SelectObject16(obj_handle, in_h_dc);
    DeleteObject16(pen);
    return;
}

pub fn call_invalidate_rect_1020_68de(param_1: Vec<u8>) {
    let mut local_es_3: u16;

  // local_es_3 = (param_1  >> 0x10);
    if ((param_1 + 0xf6) != 0) {
        invalidate_rect_1020_735a((param_1 + 0xf6));
    }
    return;
}

pub fn pt_in_rect_1020_68fc(param_1: &mut  u32, param_2: i32, param_3: u16) {
    let pp_var1: fn();
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    local_6 = CONCAT22(param_2, param_3);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    pass1_1018_31d0(*(i_var3 + 0xf2));
    if (param_2 != 0) {
        BVar2 = PtInRect16(local_6, ((i_var3 + 0xf2) + 0x16c));
        if (BVar2 != 0) {
            let param_1_val = unsafe { *param_1 };
            pp_var1 = (param_1_val + 0x40);
            (**pp_var1)(offset, i_var3, u_var4, 0xef);
        }
    }
    return;
}

pub fn invalidate_rect_1020_735a(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x1c);
    InvalidateRect16(0, (u_var1 + 0x16c), (u_var1 >> 0x10));
    return;
}

pub unsafe fn pt_in_rect_1020_5856(in_struct_1: &mut Struct26, in_struct_2: &mut  Struct665) {
    let pu_var1: &mut  u32;
    let mut in_a_x: i32;
    let b_var2: bool;
    let mut u_var3: u32;
    let mut extraout_d_x: i32;
    let mut extraout_d_x_00: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1018_2862((in_struct_1 + 0xfa));
    if ((extraout_d_x | in_a_x) != 0) {
        local_a = 0;
        while (true) {
            pu_var1 = (in_a_x + 10);
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val < local_a || pu_var1_val == local_a) {
                break;
            }

            u_var3 = local_a;
            bad_func_1008_8fc4(CONCAT22(extraout_d_x, in_a_x), local_a);
            if ((extraout_d_x_00 | u_var3) != 0) {
                b_var2 = PtInRect16(in_struct_2.field_0x0, (u_var3 + 0x14));
                if (b_var2 != 0) {
                    return;
                }
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub unsafe fn call_pt_in_rect_fn_1020_3c8c(param_1: &mut Vec<u8>, param_2: u32) {
    pt_in_rect_1018_1bda((param_1 + 0xfa), param_2);
    return;
}

pub fn validate_rect_1020_3f12(param_1: u32, param_2: i32) {
    let mut h_wnd: u16;
    let mut rect: u32;
    let mut local_6: u32;

    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    rect = 0x8000e;
    local_6 = 0x1100116;
    InvalidateRect16(0, &rect, h_wnd);
    rect = 0xf10000;
    local_6 = 0x1220030;
    ValidateRect16(&rect, h_wnd);
    rect = 0xf100f5;
    local_6 = 0x1220127;
    ValidateRect16(&rect, h_wnd);
    return;
}

pub fn call_pt_in_rect_fn_1020_1d8e(param_1: &mut Vec<u8>, param_2: u32) {
    pt_in_rect_1010_4e08((param_1 + 0x8e), param_2, (param_2 >> 0x10));
    return;
}

pub fn invalidate_window_rect_1020_1fb2(param_1: u32, param_2: i32) {
    let mut unaff_cs: u16;
    let mut unaff_ss: HWND16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 1) {
        (param_1 + 6) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
    local_16 = 0;
    local_6 = 0x46;
    local_14 = 0x46;
    local_12 = local_a - local_e;
    local_4 = 0x5f;
    local_10 = 0x5f;
    InvalidateRect16(0, &local_16, unaff_ss);
    return;
}
