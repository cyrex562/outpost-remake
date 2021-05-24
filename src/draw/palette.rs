use crate::app_context::AppContext;
use crate::err_ops::error_check_1000_17ce;
use crate::pass::pass14_funcs::{pass1_1008_57a4, pass1_1008_5b12};
use crate::pass::pass8_funcs::{pass1_1010_1dda, pass1_1010_1ea6};
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_47cc, process_struct_1008_4834};
use crate::structs::prog_structs_2::{Struct199, Struct7};
use crate::structs::prog_structs_24::Struct103;
use crate::structs::prog_structs_31::Struct33;
use crate::structs::prog_structs_7::{Struct376, Struct44};
use crate::structs::prog_structs_9::{Struct633, Struct634};
use crate::typedefs::{HDC16, HPALETTE16};
use crate::util::{CONCAT22, ZEXT24};
use crate::winapi::{CreatePalette16, DeleteObject16, GetDC16, InvalidateRect16, RealizePalette16, ReleaseDC16, SelectPalette16, UnrealizeObject16};

pub fn realize_palette_1008_46e4(param_1: &mut Struct103, param_2: &mut  HDC16) -> u16 {
    let mut bVar1: bool;
    let mut u_var2: u16;
    let mut HVar3: HPALETTE16;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut local_4: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 6) == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | u_var5 << 0x10));
    }
    if ((i_var4 + 6) == 0) {
        bVar1 = false;
    } else {
        if ((i_var4 + 10) == 0) {
            process_struct_1008_4834((param_1 & 0xffff | u_var5 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    u_var2 = create_palette_1008_4e38((i_var4 + 10));
    (i_var4 + 0xe) = u_var2;
    let param_2_val = unsafe { *param_2 };
    HVar3 = SelectPalette16(0, (i_var4 + 0xe), param_2_val);
    (i_var4 + 4) = HVar3;
    RealizePalette16(param_2_val);
    return (i_var4 + 4);
}

pub fn realize_palette_1008_4e08(param_1: u32, param_2: &mut  HDC16) -> HDC16 {
    let mut h_palette: HPALETTE16;
    let mut HVar1: HDC16;
    let mut local_4: u16;

    unsafe { HVar1 = *param_2 };
    h_palette = create_palette_1008_4e38(param_1);
    SelectPalette16(0, h_palette, HVar1);
    unsafe { HVar1 = *param_2 };
    RealizePalette16(HVar1);
    return HVar1;
}

pub fn create_palette_1008_4e38(param_1: u32) {
    let pu_var1: &mut  u16;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let in_dx: &mut  Struct199;
    let local_bx_4: &mut  Struct33;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

  // u_var7 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    u_var4 = (local_bx_4.field_0xc + 2) * 4;
    if (&local_bx_4.palette == 0) {
        process_struct_1000_179c(u_var4, in_dx);
        &local_bx_4.palette = u_var4;
        local_bx_4.field_0x10 = in_dx;
        *&local_bx_4.palette = 0x300;
        u_var3 = &local_bx_4.palette;
        (u_var3 + 2) = local_bx_4.field_0xc;
        u_var2 = &local_bx_4.palette;
        local_8 = u_var2 & 0xffff0000 | (u_var2 + 4);
        local_c = local_bx_4.field_0x4;
        local_e = 0;
        loop {
            pu_var1 = &local_bx_4.field_0xc;
            let pu_var_1_val = unsafe { *pu_var1 };
            if (pu_var_1_val == local_e || pu_var_1_val < local_e) {
                break;
            }
          // u_var8 = (local_c  >> 0x10);
            i_var5 = local_c;
            *local_8 = *(i_var5 + 2);
          // u_var9 = (local_8  >> 0x10);
            i32_var6 = local_8;
            *(i32_var6 + 1) = *(i_var5 + 1);
            *(i32_var6 + 2) = *local_c;
            *(i32_var6 + 3) = 0;
            local_e = local_e + 1;
            local_8 = local_8 & 0xffff0000 | (i32_var6 + 4);
            local_c = local_c & 0xffff0000 | (i_var5 + 4);
        }
    }
    CreatePalette16(&local_bx_4.palette);
    return;
}

pub unsafe fn set_palette_fn_1018_69ac(in_Struct376: &mut  Struct376) {
    let local_Struct376: &mut  Struct376;
    let mut u16_1: u16;

  // u16_1 = (in_Struct376  >> 0x10);
    local_Struct376 = in_Struct376;
    in_Struct376.offset = 0x6a02;
    local_Struct376.segment = 0x1018;
    if (&local_Struct376.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_Struct376.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_Struct376);
    return;
}

pub unsafe fn set_palette_fn_1018_69dc(
    in_Struct376: &mut  Struct376,
    param_2: u8,
) -> &mut  Struct376 {
    set_palette_fn_1018_69ac(in_Struct376);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_Struct376);
    }
    return in_Struct376;
}

pub fn palette_fn_1020_0aa6(param_1: &mut Vec<u8>) {
    palette_fn_1020_0cd2((param_1 + 0xe2));
    return;
}

pub fn palette_fn_1020_0cd2(ctx: &mut AppContext, in_struct_1: &mut  Struct633) {
    let paVar1: &mut  Struct634;
    let mut u_var2: i32;
    let mut hdc: HDC16;
    let mut h_palette: HDC16;
    let mut h_palette_00: HPALETTE16;
    let mut u_var3: u16;

    let local_struct_1: &mut  Struct633;
    let local_struct_2: &mut  Struct634;
    let local_struct_1_hi: &mut  Struct633;
    let local_struct_2_hi: &mut  Struct634;
    let mut h_dc: HDC16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut local_a: u32;
    let temp_5f7b277b00: &mut  u32;
    let fn_ptr_1: fn();

  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = local_struct_1.field_0x6;
  // local_struct_2_hi = (paVar1  >> 0x10);
    local_struct_2 = paVar1;
    temp_5f7b277b00 = &local_struct_2.field_0xa;
    local_6._0_2_ = temp_5f7b277b00;
    u_var2 = &local_struct_2.field_0xc | local_6;
    if (u_var2 != 0) {
        let tmp_5f7b_val = unsafe { *temp_5f7b277b00 };
        fn_ptr_1 = (tmp_5f7b_val + 0x14);
        (**fn_ptr_1)();
        local_a = CONCAT22(ctx.dx_reg, u_var2);
        if ((ctx.dx_reg | u_var2) != 0) {
            hdc = GetDC16(local_struct_1.win_handle_0x4);
            h_palette = hdc;
            h_dc = hdc;
            create_palette_1008_4e38(local_a);
            h_palette_00 = SelectPalette16(0, h_palette, h_dc);
            u_var3 = RealizePalette16(hdc);
            SelectPalette16(1, h_palette_00, hdc);
            DeleteObject16(h_palette);
            if (0 < u_var3) {
                InvalidateRect16(1, 0x0, 0);
            }
            ReleaseDC16(hdc, local_struct_1.win_handle_0x4);
            return;
        }
    }
    return;
}

pub fn realize_palette_1020_0e46(param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: &mut  u32;
    let mut u_var4: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
      // u_var4 = (param_1  >> 0x10);
        u_var2 = (param_1 + 0xf2);
        pu_var3 = (u_var2 + 0x66);
        let pu_var3_val = unsafe { *pu_var3 };
        pp_var1 = (pu_var3_val + 0x18);
        (**pp_var1)();
        UnrealizeObject16(pu_var3_val);
        u_var2 = (param_1 + 0xf2);
        RealizePalette16((u_var2 + 0x7c));
    }
    return;
}

pub unsafe fn palette_func_1020_150e(ctx: &mut AppContext, in_struct_1: &mut  Struct376) {
    let mut u_var1: u32;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1_low: &mut  Struct376;
    let local_struct_1: &mut  Struct376;

  // local_struct_1 = (in_struct_1  >> 0x10);
    local_struct_1_low = in_struct_1;
    in_struct_1.offset = 0x1730;
    local_struct_1_low.segment = 0x1020;
    if (&local_struct_1_low.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(
            local_struct_1_low.u8_ptr_x14,
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1) << 0x10),
        );
    }
    u_var1 = &local_struct_1_low.u8_ptr_x14;
    h_gdi_obj = SelectPalette16(0, &local_struct_1_low.field_0x1c, (u_var1 + 0x7c));
    &local_struct_1_low.field_0x1c = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    in_struct_1.offset = ctx.s_0_020_1050_3ab0;
    local_struct_1_low.segment = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.offset = ctx.s_1_1050_389a;
    local_struct_1_low.segment = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn delete_palette_func_1040_9458(param_1: u32, param_2: u8, param_3: HDC16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut h_gdi_obj: HPALETTE16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut uStack28: u16;
    let mut uStack26: u16;
    let puStack24: &mut  u16;
    let mut uStack20: u16;
    let mut uStack18: u16;
    let puStack16: &mut  u16;
    let mut local_8: u16;
    let mut local_6: u32;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 8) != 0) {
        local_6 = (i_var3 + 8);
        uStack18 = (i_var3 + 10);
        if ((((i_var3 + 0xe) | (i_var3 + 0xc)) != 0) && ((param_2 & 1) != 0)) {
            local_6 = (i_var3 + 0xc);
            uStack18 = (i_var3 + 0xe);
        }
        if (((i_var3 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_6 = (i_var3 + 0x10);
            uStack18 = (i_var3 + 0x12);
        }
        local_8 = &param_3;
        uStack20 = local_6;
        u_var2 = local_6;
        puStack24 = 0x94c5;
        pp_var1 = (u_var2 + 8);
        puStack16 = local_8;
        (**pp_var1)();
        puStack24 = &param_3;
        uStack26 = (i_var3 + 0x26);
        uStack28 = (i_var3 + 0x28);
        pp_var1 = (u_var2 + 4);
        (**pp_var1)();
        h_gdi_obj = SelectPalette16(0, local_8, param_3);
        DeleteObject16(h_gdi_obj);
    }
    return;
}

pub fn select_and_delete_palette_1020_92c4(ctx: &mut AppContext, struct_param_1: &mut Struct7) {
    let mut h_gdi_obj: HPALETTE16;
    let local_Struct376: &mut  Struct376;
    let local_struct_376_hi: &mut  Struct376;

  // local_struct_376_hi = (struct_param_1  >> 0x10);
    local_Struct376 = struct_param_1;
    struct_param_1.ptr_a_lo = 0x96c8;
    local_Struct376.segment = 0x1020;
    if (local_Struct376.palette_handle_x12 != 0) {
        h_gdi_obj = SelectPalette16(
            0,
            local_Struct376.palette_handle_x12,
            local_Struct376.dc_handle_x0a,
        );
        DeleteObject16(h_gdi_obj);
    }
    struct_param_1.ptr_a_lo = ctx.s_0_020_1050_3ab0;
    local_Struct376.segment = &ctx.PTR_LOOP_1050_1008;
    struct_param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_Struct376.segment = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn realize_palette_1020_8128(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: Vec<u8>;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_22: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: [u8; 8];
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
      // u_var7 = (param_1  >> 0x10);
        i32_var6 = param_1;
        u_var2 = (i32_var6 + 0xe6);
        pu_var5 = (u_var2 + 10);
        let pu_var5_val = unsafe { *pu_var5 };
        pp_var1 = (pu_var5_val + 0x18);
        local_6 = pu_var5;
        (**pp_var1)();
        local_8 = pu_var5;
        UnrealizeObject16(local_8);
        u_var2 = (i32_var6 + 0xe6);
        local_a = (u_var2 + 0x14);
        RealizePalette16(local_a);
        pass1_1008_57a4(
            CONCAT22(unaff_ss, local_12),
            param_1 & 0xffff0000 | (i32_var6 + 0xd2),
        );
        loop {
            pu_var3 = local_12;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
            if ((ctx.dx_reg | pu_var3) == 0) {
                break;
            }
            u_var2 = (pu_var3 + 4);
            u_var7 = (pu_var3 + 6);
            pu_var4 = u_var2;
            let pu_var4_1 = unsafe { *pu_var4 };
            pp_var1 = (pu_var4_1 + 0x90);
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, pu_var4, u_var7, 1, u_var2);
        }
    }
    return;
}

pub unsafe fn select_and_delete_palette_fn_1018_e57a(
    ctx: &mut AppContext,
    in_struct_ptr_1: &mut Struct7,
) {
    let local_struct_ptr_1: &mut  Struct376;
    let mut u_var1: u16;

  // u_var1 = (in_struct_ptr_1  >> 0x10);
    local_struct_ptr_1 = in_struct_ptr_1;
    in_struct_ptr_1.ptr_a_lo = 0xe5d0;
    local_struct_ptr_1.segment = 0x1018;
    if (&local_struct_ptr_1.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_struct_ptr_1.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(ctx, in_struct_ptr_1);
    return;
}

pub unsafe fn call_palette_fn_1020_679c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    call_palette_fn_1020_6466(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn realize_palette_1020_6896(param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: &mut  u32;
    let mut u_var4: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
      // u_var4 = (param_1  >> 0x10);
        u_var2 = (param_1 + 0xf2);
        pu_var3 = (u_var2 + 0x24);
        let pu_var3_val = unsafe { *pu_var3 };
        pp_var1 = (pu_var3_val + 0x18);
        (**pp_var1)();
        UnrealizeObject16(pu_var3);
        u_var2 = (param_1 + 0xf2);
        RealizePalette16((u_var2 + 0x178));
    }
    return;
}

pub unsafe fn call_palette_fn_1020_6466(in_struct_1: &mut Struct44) {
    let local_struct_1: &mut Struct44;
    let local_struct_1_hi: &mut Struct44;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x67c2;
    local_struct_1.base_fld_2 = 0x1020;
    if (local_struct_1.field_0x14 != 0) {
        pass1_1010_1ea6(
            local_struct_1.field_0x14,
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        );
    }
    select_and_delete_palette_1020_92c4(in_struct_1);
    return;
}

pub unsafe fn call_palette_fn_1020_170a(
    in_struct_1: &mut  Struct376,
    param_2: u8,
) -> &mut  Struct376 {
    palette_func_1020_150e(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}
