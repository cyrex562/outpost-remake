use std::intrinsics::offset;

use crate::app_context::AppContext;
use crate::pass::pass14_funcs::pass1_1008_4d72;
use crate::pass::pass20_funcs::pass1_1018_0a50;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_47cc, process_struct_1008_4834};
use crate::structs::prog_structs_24::Struct103;
use crate::structs::prog_structs_25::{Struct66, Struct67};
use crate::sys_structs::BITMAPINFO;
use crate::typedefs::{COLORREF, HDC16, HGDIOBJ16, HPALETTE16, HWND16};
use crate::util::{CONCAT11, CONCAT12, CONCAT22};
use crate::winapi::{DeleteObject16, GetDC16, GetDlgCtrlID16, GetStockObject16, RealizePalette16, SelectPalette16, SetBkColor16, SetTextColor16, UnrealizeObject16};

pub fn sys_color_func_1008_357e(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let local_bx_221: &mut  Struct67;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let CVar6: COLORREF;
    let mut color_ref: u32;
    let puStack140: &mut  u32;
    let mut local_84: u16;
    let mut local_82: u16;
    let mut local_80: u32;
    let mut local_7c: u16;
    let mut local_7a: u16;
    let mut local_78: u16;
    let mut local_76: u16;
    let mut local_74: u16;
    let mut local_72: u16;
    let mut local_70: u32;
    let mut local_6c: u32;
    let mut local_68: u16;
    let mut local_66: u16;
    let mut local_64: u16;
    let mut local_62: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut count: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    count = 0x70004;
    local_28 = 0xf0000;
    local_24 = 0x100014;
    local_20 = 0xd0012;
    local_1c = 0x6000e;
    local_18 = 0x80005;
    local_14 = 0x10011;
    local_10 = 0x30002;
    local_c = 0xa0009;
    local_8 = 0xc000b;
    local_4 = 0x13;
    local_80 = 0;
    local_6c = 0x808080;
    u_var3 = 0x100;
    local_74 = 0;
    local_72 = 0x100;
    local_64 = 0;
    local_62 = 0x100;
    local_60 = 0xffff;
    local_5e = 0;
    local_7c = 2;
    local_7a = 0x100;
    local_78 = 2;
    local_76 = 0x100;
    local_68 = 2;
    local_66 = 0x100;
    local_5c = 2;
    local_5a = 0x100;
    local_58 = 0;
    local_50 = 0xc0c0;
    local_4e = 0;
    local_4c = 0;
    local_48 = 0;
    local_44 = 0;
    local_34 = 0;
    u_var2 = 0x8000;
    local_70 = 0x8000;
    local_54 = 0x8000;
    local_40 = 0x8000;
    local_3c = 0x8000;
    local_38 = 0x8000;
    local_30 = 0x8000;
  // u_var5 = (param_1  >> 0x10);
    local_bx_221 = param_1;
    if (&local_bx_221.field_0xf8 == 0) {
        process_struct_1000_179c(0x54, (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21));
        local_bx_221.field_0xf8 = u_var2;
        &local_bx_221.field_0xfa = u_var3;
        local_84 = 0;
        while (local_84 < 0x15) {
            CVar6 = GetSysColor16(&count + local_84 * 2);
            u_var1 = &local_bx_221.field_0xf8;
          // u_var3 = (u_var1  >> 0x10);
            i_var4 = u_var1;
            (i_var4 + local_84 * 4) = CVar6;
            (i_var4 + local_84 * 4 + 2) = (CVar6 >> 0x10);
            local_84 = local_84 + 1;
        }
    }
    if (param_2 != 0) {
        CVar6 = GetSysColor16(count);
        if ((CVar6 == local_80) && ((CVar6 >> 0x10) == local_80)) {
            return;
        }
    }
    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        local_70 = 0xc0c0c0;
    }
    if (param_2 == 0) {
        color_ref = &local_bx_221.field_0xf8;
    } else {
        color_ref = CONCAT22(unaff_ss, &local_80);
    }
    puStack140 = &count;
    SetSysColors16(color_ref, color_ref, puStack140);
    return;
}

pub fn stretch_di_bits_1008_465a(param_1: &mut  Struct103, param_2: HDC16) {
    let x_src: u16;
    let y_src: u16;
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let info: &mut  BITMAPINFO;
    let mut u_var4: u16;
    let bits: &mut Vec<u8>;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 6) == 0) {
        process_struct_1008_47cc(param_1);
    }
    if (((i_var3 + 8) | (i_var3 + 6)) == 0) {
        b_var2 = false;
    } else {
        if (((i_var3 + 0xc) | (i_var3 + 10)) == 0) {
            process_struct_1008_4834(param_1);
        }
        b_var2 = true;
    }
    if (!b_var2) {
        return;
    }
    u_var1 = (i_var3 + 0x10);
  // bits = (u_var1  >> 0x10);
    info = u_var1;
    x_src = &(info.bmi_header).bi_width;
    y_src = &(info.bmi_header).bi_height;
    u_var1 = (i_var3 + 0x14);
    StretchDIBits16(
        0xcc0020,
        0,
        info,
        bits,
        u_var1,
        (u_var1 >> 0x10),
        y_src,
        x_src,
        0,
        0,
        y_src,
        x_src,
        param_2,
    );
    return;
}

pub fn cleanup_palette_1008_56e2(param_1: u32, param_2: &mut  HDC16) -> u16 {
    let h_gdi_obj: HPALETTE16;
    let mut u_var1: u16;
  // u_var1 = (param_1  >> 0x10);
    h_gdi_obj = SelectPalette16(0, (param_1 + 4), unsafe { *param_2 });
    (param_1 + 4) = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    return 1;
}

pub fn set_di_bits_to_dev_1008_45d6(param_1: &mut  Struct103) {
    let mut bVar1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_6: u16;

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 6) == 0) {
        process_struct_1008_47cc(param_1);
    }
    if (((i_var2 + 8) | (i_var2 + 6)) == 0) {
        bVar1 = false;
    } else {
        if (((i_var2 + 0xc) | (i_var2 + 10)) == 0) {
            process_struct_1008_4834(param_1);
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    SetDIBitsToDevice();
    return;
}

pub fn set_colors_1040_0cc0(param_1: &mut u32, param_2: u16, param_3: i32, param_4: HDC16) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_4 = GetStockObject16(4);
    if (_PTR_LOOP_1050_5cd0 == 0) {
      // u_var3 = (param_1  >> 0x10);
        unsafe {
            pp_var1 = (*param_1 + 0x68);
        }
        u_var4 = (**pp_var1)(offset, param_1, u_var3, (param_1 + 0x6e));
        u_var4 = pass1_1008_4d72(u_var4);
      // u_var3 = (u_var4  >> 0x10);
        i_var2 = u_var4;
        _PTR_LOOP_1050_5cd0 = CONCAT22(
            CONCAT11(2, *(i_var2 + 0x94)),
            CONCAT11(*(i_var2 + 0x95), *(i_var2 + 0x96)),
        );
    }
    if (3 < param_2) {
        if (param_2 == 4) {
            local_4 = GetStockObject16(5);
        } else {
            if ((param_2 == 4) || (1 < param_2 - 5)) {
                return 0;
            }
        }
    }
    SetTextColor16(_PTR_LOOP_1050_5cd0, param_3);
    SetBkColor16(0x1000000, param_3);
    return CONCAT22(0x1050, local_4);
}

pub fn set_colors_1038_ac38(
    param_1: u16,
    param_2: u16,
    dialog_handle: HWND16,
    uparam_2_00: i32,
    param_5: HDC16,
) {
    let mut i_var1: i32;
    let local_AX_35: &mut Struct66;
    let i_var2: u16;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    GetStockObject16(4);
    if (_PTR_LOOP_1050_5b78 == 0) {
        u_var3 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
      // u_var4 = (u_var3  >> 0x10);
        i_var1 = u_var3;
        _PTR_LOOP_1050_5b6c = CONCAT12(
            *(i_var1 + 0x3ec),
            CONCAT11(*(i_var1 + 0x3ed), *(i_var1 + 0x3ee)),
        );
        _PTR_LOOP_1050_5b70 = CONCAT12(
            *(i_var1 + 0x3e4),
            CONCAT11(*(i_var1 + 0x3e5), *(i_var1 + 0x3e6)),
        );
        _PTR_LOOP_1050_5b74 = CONCAT12(
            *(i_var1 + 0x3f8),
            CONCAT11(*(i_var1 + 0x3f9), *(i_var1 + 0x3fa)),
        );
        _PTR_LOOP_1050_5b78 = CONCAT12(
            *(i_var1 + 0x94),
            CONCAT11(*(i_var1 + 0x95), *(i_var1 + 0x96)),
        );
    }
    if (param_2_00 < 4) {
        // LAB_1038_acf0:
        i_var2 = GetDlgCtrlID16(dialog_handle);
        if (i_var2 == 0xfd4) {
            u_var4 = _PTR_LOOP_1050_5b70;
          // u_var5 = (_PTR_LOOP_1050_5b70  >> 0x10);
            // goto LAB_1038_ad0e;
        }
        if (i_var2 != 0xfd5) {
            if (i_var2 == 0xfd6) {
                u_var4 = _PTR_LOOP_1050_5b6c;
              // u_var5 = (_PTR_LOOP_1050_5b6c  >> 0x10);
                // goto LAB_1038_ad0e;
            }
            if (i_var2 == 0xfd7) {
                u_var4 = _PTR_LOOP_1050_5b74;
              // u_var5 = (_PTR_LOOP_1050_5b74  >> 0x10);
                // goto LAB_1038_ad0e;
            }
        }
    } else {
        if (param_2_00 != 4) {
            if ((param_2_00 == 4) || (1 < param_2_00 - 5)) {
                return;
            }
            // goto LAB_1038_acf0;
        }
    }
    u_var4 = _PTR_LOOP_1050_5b78;
  // u_var5 = (_PTR_LOOP_1050_5b78  >> 0x10);
    // LAB_1038_ad0e:
    SetTextColor16(CONCAT22(u_var5, u_var4), param_5);
    SetBkColor16(0x1000000, param_5);
    return;
}

pub fn palette_func_1020_2992(param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let obj: HGDIOBJ16;
    let hdc: HDC16;
    let mut u_var2: u16;
    let pu_var3: &mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0) {
      // u_var2 = (param_1  >> 0x10);
        pu_var3 = pass1_1018_0a50((param_1 + 0xf2));
        unsafe {
            pp_var1 = (*pu_var3 + 0x18);
        }
        obj = (**pp_var1)(0x1018);
        UnrealizeObject16(obj);
        hdc = GetDC16((param_1 + 8));
        RealizePalette16(hdc);
    }
    return;
}

pub fn set_colors_1020_7070(ctx: &mut AppContext) {
    let mut in_dx: i32;
    let mut CVar1: COLORREF;
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: HDC16;
    let mut local_4: u16;

    GetStockObject16(4);
    if (ctx._PTR_LOOP_1050_441e == 0) {
        ctx._PTR_LOOP_1050_441e = 0x1000002;
    }
    if (6 < in_stack_0000000a) {
        return in_dx << 0x10;
    }
    SetTextColor16(ctx._PTR_LOOP_1050_441e, in_stack_0000000c);
    CVar1 = SetBkColor16(0x1000000, in_stack_0000000c);
    return CVar1 & 0xffff0000 | 0x100;
}
