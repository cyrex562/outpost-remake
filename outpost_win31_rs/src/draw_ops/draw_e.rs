use std::os::raw::c_char;
use crate::draw_ops::draw_a;
use crate::globals::{DAT_1050_4216, DAT_1050_422c};
use crate::structs::struct_57::Struct57;
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::utils::CONCAT22;
use crate::winapi16::{CreateDC16, DeleteDC16, DeleteObject16, GetStockObject16, SelectPalette16};
use crate::windef16::{DEVMODEA, HDC16, HGDIOBJ16, HPALETTE16, HWND16, RECT16};

pub fn create_dc_1018_4e04(
    in_string_6: u16,
    in_string_5: u16,
    param_3: *mut astruct_8,
    mut param_4: u16,
    mut param_5: i16,
    mut param_6: i16,
) {
    let mut hvar1: HDC16;
    let mut p_hvar2: *mut HDC16;
    let mut i_var4: *mut astruct_8;
    let mut u_var3: u16;
    let mut devmodea_init_data: *mut DEVMODEA;
    let mut i_stack16: i16;
    let mut fn_ptr_1: *mut *mut code;

    u_var3 = (param_3 >> 0x10);
    i_var4 = param_3;
    fn_ptr_1 = (param_3 + 0x14);
    (**fn_ptr_1)();
    devmodea_init_data = pass1_1008_4772(i_var4.field10_0xa);
    pass1_1008_43cc(i_var4.field10_0xa);
    hvar1 = CreateDC16(devmodea_init_data, NULL, NULL, s_dib_1050_4234);
    i_var4.field15_0x12 = hvar1;
    p_hvar2 = &i_var4.field15_0x12;
    fn_ptr_1 = (i_var4.field10_0xa + 0x8);
    (**fn_ptr_1)();
    i_var4.field22_0x1a = p_hvar2;
    if ((DAT_1050_422e != 0) && (0x280 < param_6)) {
        // for (iStack16 = 0; iStack16 < DAT_1050_4216 + 1; iStack16 += 1)
        for iStack16 in 0..DAT_1050_4216 {
            (&u16_1050_4172 + iStack16 * 0x2) = (((&u16_1050_4172 + i_stack16 * 0x2) * (param_6 + 1)) / 0x280);
        }
        // for (iStack16 = 0; iStack16 < DAT_1050_422c + 1; iStack16 += 1)
        for iStack16 in 0..DAT_1050_422c {
            (&u16_1050_419a + iStack16 * 0x2) = (((&u16_1050_419a + i_stack16 * 0x2) * (param_5 + 1)) / 0x1e0);
        }
    }
    DAT_1050_422e = 0;
    return;
}

// l
pub fn fill_rect_1008_39ac(in_win_handle_1: *mut astruct_930, mut param_2: u16) {
    let mut hbrush: HBRUSH16;
    let mut local_paint_struct: [u8; 0x20] = [0; 0x20];

    BeginPaint16(
        CONCAT22(0x1050, local_paint_struct),
        in_win_handle_1.field4_0x4,
    );
    hbrush = CreateSolidBrush16(0x210070b);
    GetClientRect16(&stack0xffd2, 0x1050);
    FillRect16(hbrush, &stack0xffd2, 0x1050);
    EndPaint16(
        CONCAT22(0x1050, local_paint_struct),
        in_win_handle_1.field4_0x4,
    );
    DeleteObject16(hbrush);
    return;
}


pub fn fill_rect_1020_065e(astruct754_param_1: *mut astruct_754) {
    let mut uVar1: u32;
    let mut struct754_var1: *mut astruct_754;
    let mut uVar4: u16;
    let mut rect_1: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut brush_handle_1: HBRUSH16;
    let mut palette_handle_var42: *mut HDC16;
    let mut puStack40: *mut u32;
    let mut hdc_var_24: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u16;
    let mut uVar2: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar4 = (astruct754_param_1 >> 0x10);
    struct754_var1 = astruct754_param_1;
    hdc_var_24 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), &struct754_var1.field_0x4);
    if (0x280 < struct754_var1.field7_0xa) {
        brush_handle_1 = CreateSolidBrush16(0x210070b);
        rect_1.x = 0;
        rect_1.y = 0;
        iStack48 = struct754_var1.field7_0xa - 0x1;
        iStack46 = struct754_var1.field8_0xc - 0x1;
        FillRect16(brush_handle_1, &rect_1, 0x1050);
        DeleteObject16(brush_handle_1);
    }
    uVar2 = struct754_var1.field6_0x6;
    puStack40 = (uVar2 + 0xe);
    palette_handle_var42 = &hdc_var_24;
    uVar3 = puStack40;
    uVar1 = *puStack40;
    fn_ptr_1 = (uVar1 + 0x8);
    (**fn_ptr_1)(
        0x1538,
        uVar3,
        (puStack40 >> 0x10),
        palette_handle_var42,
        0x1050,
    );
    fn_ptr_1 = (uVar1 + 0x4);
    (**fn_ptr_1)(
        0x1538,
        puStack40,
        (puStack40 >> 0x10),
        struct754_var1.field10_0x10,
        struct754_var1.field9_0xe,
        &hdc_var_24,
        0x1050,
        uVar3,
    );
    palette_handle_var42 = SelectPalette16(0x0, palette_handle_var42, hdc_var_24);
    DeleteObject16(palette_handle_var42);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), &struct754_var1.field_0x4);
    return;
}

pub fn draw_op_1040_82ee(astruct14_param_1: *mut astruct_14)

{
   let mut struct_1: *mut astruct_14;
  let mut struct_1_hi: HDC16;
  let mut local_1a: u32;
  let mut uStack22: u32;
  let mut rect_var_12: RECT16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut brush_handle_1: HBRUSH16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  struct_1_hi = (astruct14_param_1 >> 0x10);
  struct_1 = astruct14_param_1;
  iStack6 = (struct_1.field118_0x80 - struct_1.field116_0x7c) -0x2;
  iStack8 = (-(struct_1.field95_0x60 == 0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  brush_handle_1 = CreateSolidBrush16(CONCAT22(0x100,iStack8));
  if (&struct_1[0x1].field_0x4 == 0) {
    local_1a = CONCAT22(struct_1.field98_0x66 + 0x2,struct_1.field97_0x64 + 2);
    uStack22 = CONCAT22(iStack4,iStack6);
    (struct_1 + 1) = local_1a;
    struct_1[0x1].field_0x4 = uStack22;
  }
  rect_var_12.x = (struct_1 + 1) + 2;
  rect_var_12.y =
       (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 + &struct_1[0x1].field_0x2 +
       -0x2;
  iStack14 = &struct_1[0x1].field_0x4 -0x2;
  iStack12 = (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 +
             &struct_1[0x1].field_0x2 + 2;
  FrameRect16(brush_handle_1,(struct_1 + 1),struct_1_hi);
  FrameRect16(brush_handle_1,&rect_var_12,0x1050);
  DeleteObject16(brush_handle_1);
  struct_1.field115_0x7a = &struct_1[0x1].field_0x4 + 2;
  return;
}


pub fn fill_rect_1008_62c0(param_1: *mut astruct_838, mut param_2: u16) {
    //   RECT16 rect_2e [0x2];
    let mut rect_2e: [RECt16; 2] = [RECT16::default(); 2];
    let mut hbrush_var38: HBRUSH16;
    let mut hbrush_var36: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];

    hbrush_var36 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), param_1.field8_0x8);
    hbrush_var38 = CreateSolidBrush16(0x210070b);
    GetClientRect16(rect_2e, 0x1050);
    FillRect16(hbrush_var38, rect_2e, 0x1050);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), param_1.field8_0x8);
    DeleteObject16(hbrush_var38);
    return;
}

pub fn unk_draw_op_1018_c578(param_1: *mut Struct57, param_2: *mut astruct_36) {
    let mut paVar1: *mut astruct_76;
    let mut uVar2: u16;
    let mut uVar5: u16;
    let mut hpal: *mut HDC16;
    let mut iVar5: i16;
    let mut iVar3: i16;
    let mut uVar6: u16;
    let mut uVar9: u16;
    let mut uVar7: u16;
    let mut extraout_DX: u16;
    let mut obj: HPALETTE16;
    let mut iVar4: *mut astruct_36;
    let mut uVar10: u16;
    let mut unaff_SI: u16;
    let mut uVar8: *mut astruct_36;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut in_stack_0000fe56: u16;
    let mut in_stack_0000ff7a: u16;
    let mut in_stack_0000ff80: u16;
    let mut in_stack_0000ff84: u16;
    let mut rect_34: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut hbrush_44: HBRUSH16;
    let mut hdc_2a: HDC16;
    let mut uStack40: u16;
    let mut puStack38: *mut u32;
    let mut paintstruct_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut uVar1: u16;
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut puVar3: *mut u8;
    let mut fn_ptr_1: *mut *mut code;

    puStack38 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2),
        in_stack_0000fe56,
        in_stack_0000ff7a,
        in_stack_0000ff80,
        in_stack_0000ff84,
    );
    uVar9 = (puStack38 >> 0x10);
    uVar5 = (puStack38 + 0x20);
    iVar4 = param_2;
    uVar8 = (param_2 >> 0x10);
    uStack40 = uVar5;
    if (uVar5 == 0) {
        BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
        EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
        PostMessage16(0x0, iVar4.wparam_0xea, 0x111, HWND16_1050_0396);
        return;
    }
    if ((iVar4.field235_0xf0 == 0) && (iVar4.field238_0xf4 != 0)) {
        iVar4.field235_0xf0 = 0x1;
        puVar3 = &iVar4.field_0xf2;
        win_1008_5c9e(
            puVar3,
            uVar9,
            _u16_1050_02a0,
            (param_2 & 0xffff0000 | ZEXT24(puVar3)),
        );
        uVar5 = puVar3;
    }
    if ((_u16_1050_02a0 + 0x12) == 0) {
        win_1008_5c5c(uVar5, uVar9, _u16_1050_02a0, 0x1d3);
    }
    hdc_2a = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
    hbrush_44 = CreateSolidBrush16(0x2000000);
    rect_34 = 0x0;
    iStack48 = iVar4.field239_0xf6 - 0x1;
    iStack46 = iVar4.field240_0xf8 - 0x1;
    FillRect16(hbrush_44, &rect_34, 0x1050);
    DeleteObject16(hbrush_44);
    uVar3 = iVar4.field225_0xe2;
    paVar1 = (uVar3 + 0xe);
    hpal = &hdc_2a;
    uVar11 = (paVar1 >> 0x10);
    uVar10 = paVar1;
    uVar4 = paVar1;
    fn_ptr_1 = (uVar4 + 0x8);
    (**fn_ptr_1)(0x1538, uVar10, uVar11, hpal, 0x1050);
    uVar12 = pass1_1008_4772(paVar1);
    uVar2 = (uVar12 >> 0x10);
    iVar1 = (uVar12 + 0x4);
    iVar2 = (uVar12 + 0x8);
    iVar5 = 0x1e0 - iVar2;
    extraout_DX = iVar5 >> 0xf;
    iVar3 = iVar5 / 0x2;
    iVar4.field249_0x10c = iVar3 + iVar2 + iVar4.field251_0x110;
    fn_ptr_1 = (uVar4 + 0x4);
    (**fn_ptr_1)(
        0x1008,
        uVar10,
        uVar11,
        iVar4.field242_0xfc + iVar4.field243_0xfe + iVar3,
        iVar4.field241_0xfa + (0x280 - iVar1) / 0x2,
        &hdc_2a,
        0x1050,
    );
    draw_a::draw_text_1018_c742(extraout_DX, param_2, &hdc_2a, 0x1050, uVar10);
    obj = SelectPalette16(0x0, hpal, hdc_2a);
    DeleteObject16(obj);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
    return;
}

pub fn unk_draw_op_1018_cda8(
    param_1: *mut Struct57,
    mut param_2: u16,
    mut param_3: u16,
    struct36_param_1: *mut astruct_36,
) {
    let mut paVar1: *mut astruct_76;
    let mut uVar9: u16;
    let mut uVar4: u16;
    let mut uVar11: u16;
    let mut uVar7: u16;
    let mut uVar3: u16;
    let mut hpalette_var1: *mut HDC16;
    let mut iVar4: i16;
    let mut iVar10: i16;
    let mut iVar2: i16;
    let mut selected_obj: HPALETTE16;
    let mut uVar12: u16;
    let mut struct36_var3: *mut astruct_36;
    let mut uVar13: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar8: u16;
    let mut uVar14: u32;
    let mut in_stack_0000fe5a: u16;
    let mut in_stack_0000ff7e: u16;
    let mut in_stack_0000ff84: u16;
    let mut in_stack_0000ff88: u16;
    let mut rect_var34: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut brush_handle_var44: HBRUSH16;
    let mut hdc_2a: HDC16;
    let mut uStack40: u16;
    let mut puStack38: *mut u32;
    let mut paintstruct_var_22: [u8; 0x20] = [0; 0x20];
    let mut piVar1: *mut i16;
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut in_stack_0000ffb0: u32;
    let mut uVar10: u16;
    let mut fn_ptr_2: *mut *mut code;

    puStack38 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2),
        in_stack_0000fe5a,
        in_stack_0000ff7e,
        in_stack_0000ff84,
        in_stack_0000ff88,
    );
    uVar11 = (puStack38 >> 0x10);
    uVar3 = (puStack38 + 0x20);
    struct36_var3 = struct36_param_1;
    uVar5 = (struct36_param_1 >> 0x10);
    uStack40 = uVar3;
    if (uVar3 == 0) {
        BeginPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
        EndPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
        PostMessage16(0x0, struct36_var3.wparam_0xea, 0x111, HWND16_1050_0396);
        return;
    }
    if (struct36_var3.field235_0xf0 == 0) {
        struct36_var3.field235_0xf0 = 0x1;
        win_1008_5c5c(uVar3, uVar11, _u16_1050_02a0, 0x1f3);
        uVar6 = (_u16_1050_02a0 >> 0x10);
        if ((_u16_1050_02a0 + 0x12) == 0) {
            win_1008_5c5c(uVar3, uVar11, _u16_1050_02a0, 0x1d3);
        }
    }
    hdc_2a = BeginPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
    brush_handle_var44 = CreateSolidBrush16(0x2000000);
    rect_var34 = 0x0;
    iStack48 = struct36_var3.field239_0xf6 - 0x1;
    iStack46 = struct36_var3.field240_0xf8 - 0x1;
    FillRect16(brush_handle_var44, &rect_var34, 0x1050);
    DeleteObject16(brush_handle_var44);
    uVar2 = struct36_var3.field225_0xe2;
    paVar1 = (uVar2 + 0xe);
    hpalette_var1 = &hdc_2a;
    uVar8 = (paVar1 >> 0x10);
    uVar13 = paVar1;
    fn_ptr_2 = (paVar1 + 0x8);
    (**fn_ptr_2)(
        0x1538,
        uVar13,
        uVar8,
        hpalette_var1,
        0x1050,
    );
    uVar14 = pass1_1008_4772(paVar1);
    uVar9 = (uVar14 >> 0x10);
    iVar4 = (0x280 - (uVar14 + 0x4)) / 0x2;
    iVar1 = (uVar14 + 0x8);
    iVar10 = 0x1e0 - iVar1;
    uVar12 = iVar10 >> 0xf;
    iVar2 = iVar10 / 0x2;
    struct36_var3.field249_0x10c = iVar2 + iVar1 + struct36_var3.field251_0x110;
    if ((struct36_var3.field241_0xfa == 0) && (iVar4 == 0)) {
        piVar1 = &struct36_var3.field241_0xfa;
        *piVar1 = *piVar1 + 2;
    }
    fn_ptr_2 = (paVar1 + 0x4);
    (**fn_ptr_2)(
        0x1008,
        uVar13,
        uVar8,
        struct36_var3.field242_0xfc + struct36_var3.field243_0xfe + iVar2,
        struct36_var3.field241_0xfa + iVar4,
        &hdc_2a,
        0x1050,
    );
    draw_a::draw_text_1018_c742(uVar12, struct36_param_1, &hdc_2a, 0x1050, uVar13);
    selected_obj = SelectPalette16(0x0, hpalette_var1, hdc_2a);
    DeleteObject16(selected_obj);
    EndPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
    return;
}

pub fn unk_draw_op_1018_cfc0(
    param_1: *mut Struct57,
    mut param_2: u16,
    struct36_param_1: *mut astruct_36,
) {
    let mut uVar1: u16;
    let mut uVar4: u16;
    let mut hpal: *mut HDC16;
    let mut iVar3: i16;
    let mut iVar2: i16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar9: u16;
    let mut uVar6: u16;
    let mut obj: HPALETTE16;
    let mut uVar10: u16;
    let mut struct36_var5: *mut astruct_36;
    let mut uVar11: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar12: u32;
    let mut in_stack_0000fe58: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff82: u16;
    let mut in_stack_0000ff86: u16;
    let mut rect_34: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut hbrush_44: HBRUSH16;
    let mut local_2a: HDC16;
    let mut iStack40: i16;
    let mut puStack38: *mut u32;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut piVar1: *mut i16;
    let mut iVar1: i16;
    let mut uVar3: u32;
    let mut in_stack_0000ffb0: u16;
    let mut fn_ptr_2: *mut *mut code;
    let mut struct76_var1: *mut astruct_76;

    puStack38 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x2),
        in_stack_0000fe58,
        in_stack_0000ff7c,
        in_stack_0000ff82,
        in_stack_0000ff86,
    );
    uVar9 = (puStack38 >> 0x10);
    iStack40 = (puStack38 + 0x20);
    struct36_var5 = struct36_param_1;
    uVar7 = (struct36_param_1 >> 0x10);
    if (iStack40 == 0) {
        BeginPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
        EndPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
        PostMessage16(0x0, struct36_var5.wparam_0xea, 0x111, HWND16_1050_0396);
        return;
    }
    if ((struct36_var5.field235_0xf0 == 0) && (struct36_var5.field238_0xf4 != 0)) {
        struct36_var5.field235_0xf0 = 0x1;
        uVar4 = &struct36_var5.field_0xf2;
        win_1008_5c9e(
            uVar4,
            uVar9,
            _u16_1050_02a0,
            (struct36_param_1 & 0xffff0000 | uVar4),
        );
        if ((_u16_1050_02a0 + 0x12) == 0) {
            win_1008_5c5c(uVar4, uVar9, _u16_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
    hbrush_44 = CreateSolidBrush16(0x2000000);
    rect_34 = 0x0;
    iStack48 = struct36_var5.field239_0xf6 - 0x1;
    iStack46 = struct36_var5.field240_0xf8 - 0x1;
    FillRect16(hbrush_44, &rect_34, 0x1050);
    DeleteObject16(hbrush_44);
    uVar3 = struct36_var5.field225_0xe2;
    struct76_var1 = (uVar3 + 0xe);
    hpal = &local_2a;
    uVar8 = (struct76_var1 >> 0x10);
    uVar11 = struct76_var1;
    fn_ptr_2 = (struct76_var1 + 0x8);
    (**fn_ptr_2)(0x1538, uVar11, uVar8, hpal, 0x1050);
    uVar12 = pass1_1008_4772(struct76_var1);
    uVar1 = (uVar12 >> 0x10);
    iVar3 = (0x280 - (uVar12 + 0x4)) / 0x2;
    iVar1 = (uVar12 + 0x8);
    iVar2 = 0x1e0 - iVar1;
    uVar10 = iVar2 >> 0xf;
    iVar4 = iVar2 / 0x2;
    struct36_var5.field249_0x10c = iVar4 + iVar1 + struct36_var5.field251_0x110;
    if ((struct36_var5.field241_0xfa == 0) && (iVar3 == 0)) {
        piVar1 = &struct36_var5.field241_0xfa;
        *piVar1 = *piVar1 + 2;
    }
    fn_ptr_2 = (struct76_var1 + 0x4);
    (**fn_ptr_2)(
        0x1008,
        uVar11,
        uVar8,
        struct36_var5.field242_0xfc + struct36_var5.field243_0xfe + iVar4,
        struct36_var5.field241_0xfa + iVar3,
        &local_2a,
        0x1050,
    );
    draw_a::draw_text_1018_c742(uVar10, struct36_param_1, &local_2a, 0x1050, uVar11);
    obj = SelectPalette16(0x0, hpal, local_2a);
    DeleteObject16(obj);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
    return;
}


pub fn clenaup_win_ui_1018_4d22(in_struct_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut local_struct_1: *mut StructD;
    let mut uVar4: *mut StructD;
    let mut uVar3: u16;
    let mut unaff_SS: u16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut puVar2: *mut u32;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar3 = 0x1018;
    uVar4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    // just 0x5058
    in_struct_1.address_offset_field_0x0 = s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
    local_struct_1.address_offset_field_0x2 = 0x1018;
    if (local_struct_1.field11_0x12 != 0) {
        obj = SelectPalette16(
            0x0,
            local_struct_1.field14_0x1a,
            local_struct_1.field11_0x12,
        );
        DeleteObject16(obj);
        uVar3 = SUB42(0x1538, 0x0);
        DeleteDC16(local_struct_1.field11_0x12);
    }
    puVar1 = local_struct_1.field6_0xa;
    uVar1 = local_struct_1.field7_0xc;
    if ((uVar1 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)(uVar3, puVar1, uVar1, 1);
    }
    puVar2 = local_struct_1.field8_0xe;
    uVar2 = &local_struct_1.field_0x10;
    if ((uVar2 | puVar2) != 0) {
        fn_ptr_1 = *puVar2;
        (**fn_ptr_1)(uVar3, puVar2, uVar2, 1);
    }
    _PTR_LOOP_1050_4230 = 0;
    pass1_1010_1d80(in_struct_1);
    return;
}


pub fn win_ui_op_1008_3c34(mut param_1: u32, param_2: u8, hdc_param_3: HDC16) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut HVar3: HPALETTE16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puStack6: *mut u32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (((iVar4 + 0xa) | (iVar4 + 0x8)) != 0) {
        puStack6 = (iVar4 + 0x8);
        if (((iVar4 + 0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack6 = (iVar4 + 0xc);
        }
        if (((iVar4 + 0x10) != 0) && ((param_2 & 0x4) != 0)) {
            puStack6 = (iVar4 + 0x10);
        }
        uVar6 = (_PTR_LOOP_1050_4230 >> 0x10);
        uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
        HVar3 = draw_a::palette_op_1008_4e08(
            &hdc_param_3,
            uVar1,
            CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)),
            CONCAT22(0x1050, &hdc_param_3),
        );
        ppcVar2 = (*puStack6 + 0x4);
        (**ppcVar2)();
        HVar3 = SelectPalette16(0x0, HVar3, hdc_param_3);
        DeleteObject16(HVar3);
    }
    return;
}


pub fn delete_palette_1018_e16c(param_1: *mut astruct_795) {
    let mut puVar2: *mut u32;
    let mut hpal: *mut HDC16;
    let mut hpalette_a: HPALETTE16;
    let mut iVar5: *mut astruct_795;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut hwnd16_ss: u16;
    let mut hdc_var24: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut uVar4: u32;
    let mut uVar3: u32;
    let mut puVar3: *mut u32;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    hdc_var24 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), iVar5.hwnd_0x4);
    uVar3 = iVar5.field5_0x6;
    puVar2 = (uVar3 + 0xa);
    hpal = &hdc_var24;
    uVar6 = (puVar2 >> 0x10);
    uVar4 = *puVar2;
    fn_ptr_1 = (uVar4 + 0x8);
    (**fn_ptr_1)(0x1538, puVar2, uVar6, hpal, 0x1050);
    fn_ptr_1 = (uVar4 + 0x4);
    (**fn_ptr_1)(
        0x1538,
        puVar2,
        uVar6,
        0x0,
        &hdc_var24,
        0x1050,
    );
    hpalette_a = SelectPalette16(0x0, hpal, hdc_var24);
    DeleteObject16(hpalette_a);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), iVar5.hwnd_0x4);
    return;
}


pub fn unk_draw_op_1020_0c3e(param_1: *mut astruct_771) {
    let mut puVar2: *mut u32;
    let mut hpal: *mut HDC16;
    let mut obj: HPALETTE16;
    let mut struct_1: *mut astruct_771;
    let mut iVar5: *mut astruct_842;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uStack40: u16;
    let mut local_24: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u32;
    let mut puVar1: *mut u32;
    let mut uVar4: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (param_1 >> 0x10);
    struct_1 = param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), struct_1.hwnd_0x4);
    uVar3 = struct_1.field5_0x6;
    uVar6 = (uVar3 >> 0x10);
    iVar5 = uVar3;
    puVar2 = &iVar5.field_0xa;
    uStack40 = puVar2;
    if ((iVar5.field12_0xc | uStack40) != 0) {
        hpal = &local_24;
        uVar4 = *puVar2;
        fn_ptr_1 = (uVar4 + 0x8);
        (**fn_ptr_1)(
            0x1538,
            uStack40,
            (puVar2 >> 0x10),
            hpal,
            0x1050,
        );
        fn_ptr_1 = (uVar4 + 0x4);
        (**fn_ptr_1)(
            0x1538,
            puVar2,
            struct_1.field7_0xc,
            struct_1.field6_0xa,
            &local_24,
            0x1050,
        );
        obj = SelectPalette16(0x0, hpal, local_24);
        DeleteObject16(obj);
    }
    EndPaint16(CONCAT22(0x1050, paintstruct_22), struct_1.hwnd_0x4);
    return;
}


pub fn win_ui_op_1020_150e(param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut iVar1: *mut StructD;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut uVar1: i32;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x1730;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1ea6(iVar1.field12_0x14, (param_1 & 0xffff | uVar2 << 0x10));
    }
    uVar1 = iVar1.field12_0x14;
    obj = SelectPalette16(0x0, &iVar1.field_0x1c, *(uVar1 + 0x7c));
    iVar1.field_0x1c = obj;
    DeleteObject16(obj);
    param_1.address_offset_field_0x0 = 0x3ab0;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    return;
}


pub fn draw_op_1020_15de(param_1: *mut astruct_779) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut iVar3: *mut astruct_779;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut paVar3: *mut astruct_76;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2: u32;
    let mut uVar1: u32;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, local_22), &iVar3.field_0x4);
    BVar2 = IsIconic16(&iVar3.field_0x4);
    if (BVar2 == 0) {
        uVar4 = 0x1010;
        paVar3 = pass1_1010_454a(iVar3.field20_0x14);
        if (((paVar3 >> 0x10) | paVar3) != 0) {
            uVar1 = iVar3.field20_0x14;
            uVar4 = 0x1008;
            pass1_1008_4480((iVar3 + 1), (uVar1 & 0xffff0000 | (uVar1 + 0x76)), paVar3);
        }
        uVar2 = (iVar3 + 1);
        ppcVar1 = (*(iVar3 + 1) + 0x4);
        (**ppcVar1)(
            uVar4,
            uVar2,
            (uVar2 >> 0x10),
            0x0,
            0x0,
            &local_24,
            0x1050,
        );
    } else {
        draw_op_1020_1674(param_1);
    }
    EndPaint16(CONCAT22(0x1050, local_22), &iVar3.field_0x4);
    return;
}


pub fn draw_op_1020_1674(param_1: *mut astruct_779) {
    let mut hdc: HDC16;
    let mut struct_1: *mut astruct_779;
    let mut uVar2: u16;
    let mut rect_1a: RECT16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut rect_e: RECT16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut brush_handle: HGDIOBJ16;
    let mut icon_handle: HICON16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (PTR_LOOP_1050_0010.is_null()) {
        uVar2 = (param_1 >> 0x10);
        struct_1 = param_1;
        fn_ptr_1 = (*struct_1.field20_0x14 + 0x2c);
        icon_handle = (**fn_ptr_1)();
        if (icon_handle != 0) {
            brush_handle = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(&rect_e, 0x1050);
            rect_1a.x = 0;
            rect_1a.y = 0;
            iStack22 = (iStack10 - rect_e.x) - 0x1;
            iStack20 = (iStack8 - rect_e.y) - 0x1;
            puVar1 = struct_1.field20_0x14;
            hdc = *(puVar1 + 0x7c);
            iStack18 = iStack20;
            iStack16 = iStack22;
            FillRect16(brush_handle, &rect_1a, 0x1050);
            DrawIcon16(icon_handle, 0x2, 0x2, hdc);
        }
    }
    return;
}


pub fn palette_op_1020_7270(pstruct_param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct767_var1: *mut StructD;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut pcStack8: *mut c_char;
    let mut uVar3: u32;
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar4 = (pstruct_param_1 >> 0x10);
    struct767_var1 = pstruct_param_1;
    pstruct_param_1.address_offset_field_0x0 = 0x754c;
    struct767_var1.address_offset_field_0x2 = 0x1020;
    if (&struct767_var1.field_0x1c != 0) {
        pass1_1010_1ea6(
            &struct767_var1.field_0x1c,
            (pstruct_param_1 & 0xffff | uVar4 << 0x10),
        );
    }
    uVar1 = &struct767_var1.field12_0x14;
    uVar2 = (&struct767_var1.field12_0x14 + 2);
    pcStack8 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_5118(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack8);
    }
    uVar3 = &struct767_var1.field_0x1c;
    obj = SelectPalette16(0x0, struct767_var1.field19_0x20, *(uVar3 + 0x178));
    struct767_var1.field19_0x20 = obj;
    DeleteObject16(obj);
    pstruct_param_1.address_offset_field_0x0 = 0x3ab0;
    struct767_var1.address_offset_field_0x2 = 0x1008;
    pstruct_param_1.address_offset_field_0x0 = 0x389a;
    struct767_var1.address_offset_field_0x2 = 0x1008;
    return;
}

pub fn ui_cleanup_op_1040_782c(param_1: *mut StructD)

{
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut struct_1: *mut StructD;
  let mut uVar2: u16;
  let mut uVar1: u16;
  let mut puVar1: *mut u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar2 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.address_offset_field_0x0 = 0x840c;
  (struct_1 + 0x2) = &PTR_LOOP_1050_1040;
  puVar2 = (struct_1 + 0x70);
  uVar3 = (struct_1 + 0x72);
  if ((uVar3 | puVar2) != 0) {
    fn_ptr_1 = *puVar2;
    (**fn_ptr_1)();
  }
  if ((struct_1 + 0x4) != 0) {
    DeleteObject16((struct_1 + 0x4));
    (struct_1 + 0x4) = 0;
  }
  if ((struct_1 + 0x68) != 0) {
    DestroyMenu16((struct_1 + 0x68));
  }
  RemoveProp16(s_thisLo_1050_5db1,(struct_1 + 0x6));
  RemoveProp16(s_thisHi_1050_5db8,(struct_1 + 0x6));
  RemoveProp16(s_procLo_1050_5dbf,(struct_1 + 0x6));
  RemoveProp16(s_procHi_1050_5dc6,(struct_1 + 0x6));
  param_1.address_offset_field_0x0 = 0x389a;
  (struct_1 + 0x2) = 0x1008;
  return;
}


pub fn unk_draw_op_1040_b0f8(mut param_1: u16, param_2: *mut StructD)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar3: *mut StructD;
  let mut uVar3: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut uStack22: u16;
  let mut pcStack10: *mut c_char;
  let mut in_stack_0000ffe8: u32;

  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  param_2.address_offset_field_0x0 = 0xb772;
  iVar3.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  _param_1 = CONCAT22(uStack22,0x32);
  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,_param_1,in_stack_0000fe92,in_stack_0000ffb6,
                           in_stack_0000ffbc,in_stack_0000ffc0);
  pass1_1010_7b8c(puVar3,&iVar3.field_0x6);
  if (&iVar3.field_0x8e != 0) {
    DeleteObject16(&iVar3.field_0x8e);
    iVar3.field_0x8e = 0;
  }
  uVar1 = &iVar3.field_0x90;
  uVar2 = &iVar3.field_0x92;
  pcStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    _param_1 = CONCAT22(uVar2,uVar1);
    pass1_1040_a5d0(_param_1);
    fn_ptr_1000_17ce(pcStack10);
  }
  ui_cleanup_op_1040_782c(param_2);
  return;
}

pub fn win_prop_op_1038_d118(base_addr_param_4: u16, mut param_2: u32, mut param_3: u32, hwnd_param_3: HWND16)

{
  let mut uVar1: u32;
  let mut cVar2: u8;
  let mut HVar3: HANDLE16;
  let mut HVar4: HANDLE16;
  let mut puStack6: *mut u32;
  let mut uVar2: u32;
  let mut fn_ptr_1: *mut *mut code;

  HVar3 = GetProp16(CONCAT22(base_addr_param_4,0x5bf3),hwnd_param_3);
  HVar4 = GetProp16(CONCAT22(base_addr_param_4,0x5bec),hwnd_param_3);
  puStack6 = CONCAT22(HVar3,HVar4);
  if (param_3 == 0x30) {
    if (param_3 == 0) {
      return;
    }
    SetProp16(param_3,CONCAT22(base_addr_param_4,0x5c06),hwnd_param_3);
    return;
  }
  if (param_3 < 0x310000) {
    cVar2 = (param_3 >> 0x10);
    if (cVar2 == '\x02') {
      if ((HVar3 | HVar4) != 0) {
        uVar1 = *puStack6;
        fn_ptr_1 = uVar1 + 0x6;
        (**fn_ptr_1)(0x1538,HVar4,HVar3,param_2,param_3);
        if (puStack6.is_null() == false) {
          fn_ptr_1 = uVar1;
          (**fn_ptr_1)(0x1538,HVar4,HVar3,1);
        }
      }
      HVar3 = GetProp16(CONCAT22(base_addr_param_4,0x5bfa),hwnd_param_3);
      if (HVar3 == 0) {
        return;
      }
      DeleteObject16(HVar3);
      RemoveProp16(CONCAT22(base_addr_param_4,0x5c00),hwnd_param_3);
      return;
    }
    if (cVar2 == '\x06') {
      if ((param_3 != 1) && (param_3 != 0x2)) {
        uVar1 = &u16_1050_5bc8;
        (uVar1 + 0x8) = 0;
        return;
      }
      uVar2 = &u16_1050_5bc8;
      (uVar2 + 0x8) = hwnd_param_3;
      return;
    }
  }
  if (HVar3 | HVar4) != 0 {
    fn_ptr_1 = (*puStack6 + 0xc);
    (**fn_ptr_1)(0x1538,HVar4,HVar3,param_2,param_3);
  }
  return;
}
