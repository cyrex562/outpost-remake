use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::dos_interrupt::swi;
use crate::draw_ops::{draw_a, draw_b, draw_c};
use crate::file_ops::{read_file_1008_7dee, write_to_file_1008_7e1c};
use crate::structs::struct_57::Struct57;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1000_3000::sys_1000_3f9c;
use crate::unk::block_1000_4000::pass1_1000_484c;
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::unk::block_1008_7000::switch_1008_73ea;
use crate::unk::block_1010_1000::pass1_1010_1f62;
use crate::unk::block_1010_6000::pass1_1010_6ca2;
use crate::unk::block_1010_7000::pass1_1010_715c;
use crate::unk::block_1010_8000::pass1_1010_8170;
use crate::unk::block_1018_4000::{pass1_1018_4dce, struct_op_1018_4cda};
use crate::unk::block_1018_6000::{pass1_1018_642e, pass1_1018_6630};
use crate::unk::block_1020_2000::{pass1_1020_2286, pass1_1020_239c, pass1_1020_2488};
use crate::utils::{CONCAT11, CONCAT22};
use crate::winapi16::{CreateDC16, DeleteDC16, DeleteObject16, GetStockObject16, Rectangle16, SelectObject16, SelectPalette16, WritePrivateProfileString16};
use crate::windef16::{COLORREF, DEVMODEA, HDC16, HFILE16, HGDIOBJ16, HPALETTE16, HPEN16, RECT16};

pub unsafe fn load_draw_op_1020_2ede(
    param_1: *mut Struct57,
    mut param_2: u16,
    param_3: *mut astruct_40,
    param_4: *mut StructA,
    mut param_5: u16,
) {
    let mut hdc_dev_ctx_1: HDC16;
    StructA * *ppSVar1;
    let mut handle: HPEN16;
    let mut pHVar2: *mut HDC16;
    let mut h_null_brush: HGDIOBJ16;
    let mut in_DX: *mut u8;
    let mut struct_1: *mut astruct_40;
    let mut unaff_DI: i16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut puVar5: *mut u16;
    let mut paVar3: *mut astruct_76;
    let mut devmodea_ptr_var11: *mut DEVMODEA;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut device: *mut c_char;
    let mut driver: *mut c_char;
    let mut in_stack_0000ffea: u16;
    let mut puVar1: *mut u32;
    let mut puVar2: u32;
    let mut uVar10: COLORREF;
    let mut output: LPCSTR = null_mut();
    let mut fn_ptr_1: *mut *mut code;

    get_sys_metrics_1020_7c1a(param_3, CONCAT22(param_5, param_4));
    uVar5 = (param_3 >> 0x10);
    struct_1 = param_3;
    struct_1.field17_0x14 = 0;
    struct_1.field20_0x18 = 0;
    (&struct_1.field20_0x18 + 0x2) = 0;
    struct_1.field21_0x1c = null_mut();
    struct_1.field_0x1e = 0;
    (&struct_1[0x1].field0_0x0 + 1) = 0;
    param_3.field0_0x0 = 0x363c;
    struct_1.field1_0x2 = 0x1020;
    puVar5 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_2, param_4[0x1].field26_0x30),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    struct_1.field17_0x14 = puVar5;
    struct_1.field_0x16 = (puVar5 >> 0x10);
    fn_ptr_1 = (*&struct_1.field17_0x14 + 0x4);
    (**fn_ptr_1)();
    driver = s_dib_1050_42c2;
    device = null_mut();
    output = null_mut();
    paVar3 = pass1_1018_0a50(&struct_1.field17_0x14);
    devmodea_ptr_var11 = pass1_1008_4772(paVar3);
    hdc_dev_ctx_1 = CreateDC16(devmodea_ptr_var11, output, device, driver);
    *&struct_1.field20_0x18 = hdc_dev_ctx_1;
    ppSVar1 = &struct_1.field20_0x18;
    fn_ptr_1 = (paVar3 + 0x8);
    (**fn_ptr_1)();
    (&struct_1[0x1].field0_0x0 + 1) = ppSVar1;
    puVar2 = &struct_1.field17_0x14;
    handle = CreatePen16(*(puVar2 + 0x64), 0x1, 0x0);
    (&struct_1.field20_0x18 + 0x2) = handle;
    pHVar2 = SelectObject16(handle, *&struct_1.field20_0x18);
    struct_1.field21_0x1c = pHVar2;
    h_null_brush = GetStockObject16(HOLLOW_BRUSH);
    h_null_brush = SelectObject16(h_null_brush, *&struct_1.field20_0x18);
    struct_1.field_0x1e = h_null_brush;
    return;
}

pub unsafe fn cleanup_win_ui_1020_2fea(in_struct_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct_1: *mut StructD;
    let mut var2: u16;
    let mut unaff_SS: u16;

    var2 = (in_struct_1 >> 0x10);
    struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x363c;
    struct_1.address_offset_field_0x2 = 0x1020;
    if (struct_1.field12_0x14 != 0) {
        pass1_1010_1ea6(struct_1.field12_0x14, (in_struct_1 & 0xffff | var2 << 0x10));
    }
    SelectObject16(&struct_1.field_0x1c, struct_1.field13_0x18);
    SelectObject16(&struct_1.field_0x1e, struct_1.field13_0x18);
    DeleteObject16(struct_1.field14_0x1a);
    obj = SelectPalette16(0x0, struct_1.field19_0x20, struct_1.field13_0x18);
    DeleteObject16(obj);
    DeleteDC16(struct_1.field13_0x18);
    in_struct_1.address_offset_field_0x0 = 0x3ab0;
    struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    struct_1.address_offset_field_0x2 = 0x1008;
    return;
}


pub unsafe fn win_ui_palette_op_1020_3e84(param_1: *mut StructD) {
    let mut hdc: HDC16;
    let mut obj: HPALETTE16;
    let mut struct_v1: *mut StructD;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: i32;

    uVar1 = (param_1 >> 0x10);
    struct_v1 = param_1;
    param_1.address_offset_field_0x0 = 0x408a;
    struct_v1.address_offset_field_0x2 = 0x1020;
    pass1_1010_1ea6(struct_v1.field12_0x14, (param_1 & 0xffff | uVar1 << 0x10));
    uVar2 = struct_v1.field12_0x14;
    hdc = *(uVar2 + 0x4c);
    SelectObject16(struct_v1.field13_0x18, hdc);
    SelectObject16(struct_v1.field14_0x1a, hdc);
    obj = SelectPalette16(0x0, &struct_v1.field_0x1c, hdc);
    DeleteObject16(obj);
    SetMapMode16(&struct_v1.field_0x1e, hdc);
    param_1.address_offset_field_0x0 = 0x3ab0;
    struct_v1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    struct_v1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn mixed_draw_op_1020_3fa0(mut param_1: u32) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iStack56: i16;
    let mut uStack54: u32;
    let mut local_32: u32;
    let mut iStack46: i16;
    let mut uStack44: u32;
    let mut puStack40: *mut u32;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x14);
    local_24 = *(uVar3 + 0x4c);
    uVar3 = (iVar4 + 0x14);
    puStack40 = (uVar3 + 0x24);
    uVar5 = puStack40;
    ppcVar2 = (*puStack40 + 0x4);
    (**ppcVar2)(
        0x1538,
        uVar5,
        (puStack40 >> 0x10),
        0x0,
        &local_24,
        0x1050,
    );
    uVar3 = (iVar4 + 0x14);
    iStack46 = (uVar3 + 0x44);
    uVar3 = (iVar4 + 0x14);
    uStack44 = (uVar3 + 0x40);
    uVar1 = (iVar4 + 0x14);
    pass1_1008_3e94(
        (uVar1 & 0xffff0000 | (uVar1 + 0x3a)),
        CONCAT22(0x1050, &local_32),
        CONCAT22(0x1050, &local_32 + 0x2),
    );
    uStack54 = uStack44;
    for iStack56 in 0..iStack46 {
        draw_c::draw_rect_1020_40ce(uStack54, local_32, (local_32 >> 0x10), local_24, uVar5);
        uStack54 = uStack54 & 0xffff0000 | (uStack54 + 0x18);
    }
    EndPaint16(CONCAT13(0x10, CONCAT12(0x50, local_22)), (iVar4 + 0x4));
    return;
}

pub unsafe fn draw_text_1040_8d14(param_1: *mut Struct37, hdc_param_2: HDC16)

{
  let mut in_string: LPCSTR;
  let mut bVar1: u8;
  let mut IVar2: i16;
  let mut handle: HANDLE16;
   let mut struct_1: *mut Struct37;
  let mut uVar3: u16;
  let mut obj_handle_1: HGDIOBJ16;

  uVar3 = (param_1 >> 0x10);
  struct_1 = param_1;
  bVar1 = struct_1.field138_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    struct_1.field145_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(SM_CXICON);
    struct_1.field144_0x9e = IVar2 + 0x28;
  }
  else {
    struct_1.field145_0xa0 = 0xa;
    struct_1.field144_0x9e = 0x14;
  }
  handle = GetProp16(s_hfont_1050_5e0f,(&struct_1.field1_0x4 + 0x2));
  if (handle != 0) {
    SelectObject16(handle,hdc_param_2);
  }
  in_string = struct_1.field133_0x90;
  obj_handle_1 = (in_string >> 0x10);
  DrawText16(0x410,(param_1 & 0xffff0000 | ZEXT24(&struct_1.field144_0x9e)),-0x1,in_string,hdc_param_2
            );
  if (obj_handle_1 != 0) {
    SelectObject16(obj_handle_1,hdc_param_2);
  }
  return;
}

pub unsafe fn unk_draw_op_1040_c226(struct_param_1: *mut astruct_772)

{
    let mut handle: HPEN16;
    let mut obj_handle_var3: HGDIOBJ16;
    let mut iVar3: *mut astruct_772;
    let mut uVar4: u16;
    let mut hdc: HDC16;
    let mut rect_var_32: RECT16;
    let mut iStack46: i16;
    let mut iStack44: i16;
    let mut uStack42: u16;
    let mut iStack40: i16;
    let mut hbrush_var38: HBRUSH16;
    let mut hdc16_var36: HDC16;
//   PAINTSTRUCT16 *paintstruct_22;
    let mut paintstruct_22: *mut PAINTSTRUCT16;
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar4 = (struct_param_1 >> 0x10);
    iVar3 = struct_param_1;
    hdc16_var36 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar3.hwnd_0x4);
    hbrush_var38 = CreateSolidBrush16(0x8000);
    GetClientRect16(&rect_var_32, 0x1050);
    uVar1 = iVar3.field5_0x6;
    iStack40 = (uVar1 + 0x1a);
    uVar2 = iVar3.field5_0x6;
    uStack42 = (uVar2 + 0x1c);
    rect_var_32.y += 0x2;
    rect_var_32.x = iStack40 - 0xa;
    iStack46 += -0x2;
    iStack44 += -0x2;
    FrameRect16(hbrush_var38, &rect_var_32, 0x1050);
    DeleteObject16(hbrush_var38);
    hdc = hdc16_var36;
    handle = CreatePen16(0x8080, 0x2, 0x0);
    obj_handle_var3 = SelectObject16(handle, hdc);
    draw_c::draw_line_1040_c302(struct_param_1, hdc16_var36);
    draw_c::draw_op_1040_c38e(struct_param_1);
    obj_handle_var3 = SelectObject16(obj_handle_var3, hdc16_var36);
    DeleteObject16(obj_handle_var3);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar3.hwnd_0x4);
    return;
}

pub unsafe fn fn_xyz() {
    let mut piVar2: *mut i16;
    let mut uVar3_01: u32;
    let mut uVar6: u16;
    let mut pHVar14: *mut HDC16;
    let mut iVar7: i16;
    let mut iVar11: i16;
    let mut handle: HPEN16;
    let mut hgdiobj16_00: HBRUSH16;
    let mut uVar7: u16;
    let mut obj: HPALETTE16;
    let mut puVar7: *mut u8;
    let mut hgdiobj16_var7: HGDIOBJ16;
    let mut in_EDX: u32;
    let mut paVar25: *mut Struct57;
    let mut uVar27: u16;
    let mut uVar26: u32;
    let mut struct742_var8: *mut astruct_742;
    let mut iVar9: *mut astruct_755;
    let mut iVar10: *mut astruct_756;
    let mut puVar11: *mut astruct_734;
    let mut iVar12: i16;
    let mut uVar12: u16;
    let mut uVar14: u16;
    let mut uVar13: u16;
    let mut uVar28: u32;
    let mut piVar16: *mut i16;
    let mut iVar29: i16;
    let mut local_38: [u8; 0x6] = [0; 0x6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut local_24: HDC16;
    let mut paintstruct16_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2_01: u32;
    let mut piVar1: *mut i16;
    let mut uVar15: u32;
    let mut uVar5: u32;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut iVar13: *mut astruct_758;
    let mut uVar11: u32;
    let mut uVar2: u32;
    let mut puVar4: *mut u32;
    let mut uVar4: u32;
    let mut uVar16: u8;
    let mut uVar17: u8;
    let mut uVar18: u16;
    let mut iVar16: *mut astruct_757;
    let mut uVar19: u16;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar3: u32;
    let mut uVar2_00: u32;
    let mut uVar3_00: u32;
    let mut uVar23: u32;
    let mut uVar24: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar22 = (in_EDX >> 0x10);
    puVar11 = &stack0xfffe;
    uVar12 = (param_1 >> 0x10);
    struct742_var8 = param_1;
    local_24 = BeginPaint16(
        CONCAT22(0x1050, paintstruct16_22),
        struct742_var8.field4_0x4,
    );
    puStack40 = pass1_1010_4c2c(struct742_var8.field5_0x6);
    pHVar14 = &local_24;
    fn_ptr_1 = (*puStack40 + 0x8);
    (**fn_ptr_1)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar14,
        0x1050,
    );
    struct742_var8.field12_0x10 = pHVar14;
    uVar2 = struct742_var8.field5_0x6;
    uStack42 = (uVar2 + 0x30);
    uVar28 = struct742_var8.field5_0x6;
    uStack46 = *(uVar28 + 0x12);
    uStack48 = 0x14;
    local_32 = 0;
    uVar13 = 0x1008;
    pass1_1008_3e38(CONCAT22(0x1050, local_38));
    paVar25 = (uVar22 << 0x10);
    while (
        uVar27 = (paVar25 >> 0x10),
        &puVar11[-0x6].field_0x4 < (puVar11 - 0x4),
    ) {
        iVar9 = (&puVar11[-0x6].field_0x4 * 0x4);
        uVar8 = puVar11[-0x5].field6_0x6;
        uVar28 = pass1_1008_4772((iVar9 + uVar8));
        puVar7 = (uVar28 >> 0x10);
        paVar25 = CONCAT22(uVar27, puVar7);
        puVar11[-0x7].field_0x2 = uVar28;
        puVar11[-0x7].field_0x4 = puVar7;
        uVar3 = puVar11.field6_0x6;
        pass1_1018_642e(
            uVar3,
            (uVar3 >> 0x10),
            CONCAT13(0x10, CONCAT12(0x50, &puVar11[-0x5].field_0x2)),
            (uVar28 + 0x8),
        );
        uVar9 = &puVar11[-0x5].field_0x2;
        pass1_1008_3e76(
            CONCAT22(0x1050, &puVar11[-0x6].field6_0x6),
            0x0,
            uVar9,
            (uVar9 >> 0x10),
        );
        uVar23 = puVar11[-0x5].field6_0x6;
        pass1_1008_4480(
            &puVar11[-0x4].field_0x2,
            CONCAT22(0x1050, &puVar11[-0x6].field6_0x6),
            (iVar9 + uVar23),
        );
        iVar29 = &puVar11[-0x6].field_0x4;
        uVar10 = &puVar11[-0x5].field_0x2;
        uVar19 = uVar10;
        uVar20 = (uVar10 >> 0x10);
        uVar21 = (uVar10 >> 0x18);
        uVar11 = &puVar11[-0x7].field_0x2;
        uVar14 = (uVar11 >> 0x10);
        iVar10 = uVar11;
        iVar7 = iVar10.field4_0x4 + &puVar11[-0x5].field_0x4;
        iVar11 = iVar10.field7_0x8 + &puVar11[-0x5].field_0x2;
        uVar26 = puVar11.field6_0x6;
        uVar2_00 = (uVar26 + 0x6);
        iVar16 = uVar2_00;
        uVar18 = (uVar2_00 >> 0x10);
        uVar16 = '\x0b';
        uVar17 = '\x10';
        if (&iVar16.field_0x1a == 0) {
            uVar6 = iVar16.field47_0x30 << 0x3;
            mem_op_1000_179c(uVar6, paVar25);
            iVar16.field_0x1a = uVar6;
            iVar16.field28_0x1c = paVar25;
        }
        uVar3_00 = &iVar16.field_0x1a;
        iVar13 = (iVar29 * 0x8);
        (iVar13 + uVar3_00) = CONCAT11(uVar21, uVar20);
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x2) = uVar19;
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x4) = iVar7;
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x6) = iVar11;
        uVar13 = CONCAT11(uVar17, uVar16);
        uVar2_01 = &puVar11[-0x7].field_0x2;
        piVar2 = &puVar11[-0x5].field_0x4;
        *piVar2 = *piVar2 + (-(&puVar11[-0x6].field_0x4 == 0) & 0x5) + 0x14 + (uVar2_01 + 0x4);
        piVar2 = &puVar11[-0x6].field_0x4;
        *piVar2 = *piVar2 + 1;
    }
    puVar4 = &puVar11[-0x4].field_0x2;
    fn_ptr_1 = (*puVar4 + 0x4);
    (**fn_ptr_1)(
        uVar13,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar11 - 0x22,
        0x1050,
    );
    handle = CreatePen16(0x1000025, 0x1, 0x0);
    &puVar11[-0x6].field_0x2 = handle;
    hgdiobj16_var7 = SelectObject16(handle, *&puVar11[-0x4].field6_0x6);
    (puVar11 - 0x6) = hgdiobj16_var7;
    hgdiobj16_00 = CreateSolidBrush16(0x1000025);
    (&puVar11[-0x7].field6_0x6 + 0x2) = hgdiobj16_00;
    hgdiobj16_var7 = SelectObject16(hgdiobj16_00, *&puVar11[-0x4].field6_0x6);
    puVar11[-0x7].field6_0x6 = hgdiobj16_var7;
    draw_b::draw_line_1018_6444(puVar11.field6_0x6, *&puVar11[-0x4].field6_0x6);
    uVar4 = puVar11.field6_0x6;
    piVar16 = pass1_1010_4dc8((uVar4 + 0x6));
    uVar26 = piVar16 >> 0x10;
    uVar24 = piVar16 & 0xffff;
    draw_c::draw_op_1018_6544(
        puVar11.field6_0x6,
        (piVar16 & 0xff000000 | CONCAT12((piVar16 >> 0x10), uVar24)),
    );
    pass1_1018_6630((uVar26 & 0xffff | uVar24 << 0x10), puVar11.field6_0x6);
    uVar5 = puVar11.field6_0x6;
    obj = SelectPalette16(0x0, (uVar5 + 0x10), *&puVar11[-0x4].field6_0x6);
    DeleteObject16(obj);
    hgdiobj16_var7 = SelectObject16((puVar11 - 0x6), *&puVar11[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var7);
    hgdiobj16_var7 = SelectObject16(&puVar11[-0x7].field6_0x6, *&puVar11[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var7);
    uVar15 = puVar11.field6_0x6;
    EndPaint16(
        CONCAT22(0x1050, (&puVar11[-0x4].field6_0x6 + 0x2)),
        (uVar15 + 0x4),
    );
    return;
}

pub unsafe fn unk_draw_op_1020_2020(param_1: *mut astruct_743) {
    let mut piVar1: *mut i16;
    let mut uVar3: u32;
    let mut uVar6: u32;
    let mut iVar8: i16;
    let mut pHVar9: *mut HDC16;
    let mut iVar7: i16;
    let mut iVar9: i16;
    let mut hgdiobj16_var8: HGDIOBJ16;
    let mut hgdiobj16_var9: HGDIOBJ16;
    let mut hgdiobj16_var10: HGDIOBJ16;
    let mut HVar10: HBRUSH16;
    let mut HVar11: HPEN16;
    let mut uVar11: u16;
    let mut obj: HPALETTE16;
    let mut uVar13: u16;
    let mut extraout_DX: u16;
    let mut in_EDX: u32;
    let mut paVar14: *mut Struct57;
    let mut iVar10: *mut astruct_743;
    let mut iVar11: *mut astruct_744;
    let mut iVar12: *mut astruct_745;
    let mut puVar14: *mut astruct_735;
    let mut iVar13: i16;
    let mut uVar12: u16;
    let mut uVar15: u16;
    let mut uVar14: u16;
    let mut puVar15: *mut u16;
    let mut uVar16: u32;
    let mut piVar17: *mut i16;
    let mut local_38: [u8; 0x6] = [0; 0x6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut hdc_24: HDC16;
    let mut paintstruct16_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2: u32;
    let mut puVar4: *mut u32;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut uVar24: u16;
    let mut iVar14: *mut astruct_746;
    let mut iVar15: *mut astruct_748;
    let mut uVar1: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut iVar16: i16;
    let mut fn_ptr_1: *mut *mut code;

    uVar24 = (in_EDX >> 0x10);
    puVar14 = &stack0xfffe;
    uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    hdc_24 = BeginPaint16(CONCAT22(0x1050, paintstruct16_22), iVar10.field4_0x4);
    puStack40 = pass1_1010_4c2c(iVar10.field5_0x6);
    pHVar9 = &hdc_24;
    fn_ptr_1 = (*puStack40 + 0x8);
    (**fn_ptr_1)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar9,
        0x1050,
    );
    iVar10.field12_0x10 = pHVar9;
    uVar2 = iVar10.field5_0x6;
    uStack42 = (uVar2 + 0x30);
    uVar16 = iVar10.field5_0x6;
    uStack46 = *(uVar16 + 0x12);
    uStack48 = 0x14;
    local_32 = 0;
    uVar14 = 0x1008;
    puVar15 = pass1_1008_3e38(CONCAT22(0x1050, local_38));
    paVar14 = CONCAT22(uVar24, (puVar15 >> 0x10));
    while (&puVar14[-0x6].field_0x4 < (puVar14 - 0x4)) {
        iVar11 = (&puVar14[-0x6].field_0x4 * 0x4);
        uVar6 = puVar14[-0x5].field6_0x6;
        uVar16 = pass1_1008_4772((iVar11 + uVar6));
        paVar14 = (paVar14 & 0xffff0000 | uVar16 >> 0x10);
        puVar14[-0x7].field_0x2 = uVar16;
        uVar13 = (uVar16 >> 0x10);
        puVar14[-0x7].field_0x4 = uVar13;
        uVar6 = puVar14.field6_0x6;
        pass1_1020_2286(
            uVar6,
            (uVar6 >> 0x10),
            CONCAT13(0x10, CONCAT12(0x50, &puVar14[-0x5].field_0x2)),
            (uVar16 + 0x8),
        );
        uVar3 = &puVar14[-0x5].field_0x2;
        pass1_1008_3e76(
            CONCAT22(0x1050, &puVar14[-0x6].field6_0x6),
            0x0,
            uVar3,
            (uVar3 >> 0x10),
        );
        uVar6 = puVar14[-0x5].field6_0x6;
        pass1_1008_4480(
            &puVar14[-0x4].field_0x2,
            CONCAT22(0x1050, &puVar14[-0x6].field6_0x6),
            (iVar11 + uVar6),
        );
        iVar16 = &puVar14[-0x6].field_0x4;
        uVar3 = &puVar14[-0x5].field_0x2;
        uVar23 = uVar3;
        uVar20 = (uVar3 >> 0x10);
        uVar21 = (uVar3 >> 0x18);
        uVar3 = &puVar14[-0x7].field_0x2;
        uVar15 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        iVar7 = iVar12.field4_0x4 + &puVar14[-0x5].field_0x4;
        iVar9 = iVar12.field7_0x8 + &puVar14[-0x5].field_0x2;
        uVar6 = puVar14.field6_0x6;
        uVar3 = (uVar6 + 0x6);
        iVar14 = uVar3;
        uVar22 = (uVar3 >> 0x10);
        uVar18 = '\x0b';
        uVar19 = '\x10';
        if (&iVar14.field_0x1a == 0) {
            iVar8 = iVar14.field47_0x30 << 0x3;
            mem_op_1000_179c(iVar8, paVar14);
            iVar14.field_0x1a = iVar8;
            iVar14.field28_0x1c = paVar14;
        }
        uVar3 = &iVar14.field_0x1a;
        iVar15 = (iVar16 * 0x8);
        (iVar15 + uVar3) = CONCAT11(uVar21, uVar20);
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x2) = uVar23;
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x4) = iVar7;
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x6) = iVar9;
        uVar14 = CONCAT11(uVar19, uVar18);
        uVar3 = &puVar14[-0x7].field_0x2;
        piVar1 = &puVar14[-0x5].field_0x4;
        *piVar1 = *piVar1 + (-(&puVar14[-0x6].field_0x4 == 0) & 0x5) + 0x14 + (uVar3 + 0x4);
        piVar1 = &puVar14[-0x6].field_0x4;
        *piVar1 = *piVar1 + 1;
    }
    puVar4 = &puVar14[-0x4].field_0x2;
    fn_ptr_1 = (*puVar4 + 0x4);
    (**fn_ptr_1)(
        uVar14,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar14 - 0x22,
        0x1050,
    );
    extraout_DX = paVar14;
    hgdiobj16_var8 = CreatePen16(0x1000025, 0x1, 0x0);
    puVar14[-0x6].field_0x2 = hgdiobj16_var8;
    hgdiobj16_var10 = SelectObject16(hgdiobj16_var8, *&puVar14[-0x4].field6_0x6);
    (puVar14 - 0x6) = hgdiobj16_var10;
    hgdiobj16_var9 = CreateSolidBrush16(0x1000025);
    (&puVar14[-0x7].field6_0x6 + 0x2) = hgdiobj16_var9;
    hgdiobj16_var10 = SelectObject16(hgdiobj16_var9, *&puVar14[-0x4].field6_0x6);
    puVar14[-0x7].field6_0x6 = hgdiobj16_var10;
    draw_c::draw_line_1020_229c(puVar14.field6_0x6, &puVar14[-0x4].field6_0x6);
    uVar1 = puVar14.field6_0x6;
    pass1_1010_4df0(extraout_DX, (uVar1 + 0x6));
    if (hgdiobj16_var10 == 0) {
        hgdiobj16_var10 = SelectObject16((puVar14 - 0x6), *&puVar14[-0x4].field6_0x6);
        DeleteObject16(hgdiobj16_var10);
        hgdiobj16_var10 = SelectObject16(&puVar14[-0x7].field6_0x6, *&puVar14[-0x4].field6_0x6);
        DeleteObject16(hgdiobj16_var10);
        HVar10 = CreateSolidBrush16(0xff);
        (&puVar14[-0x7].field6_0x6 + 0x2) = HVar10;
        HVar11 = CreatePen16(0xff, 0x1, 0x0);
        puVar14[-0x6].field_0x2 = HVar11;
        SelectObject16(
            (&puVar14[-0x7].field6_0x6 + 0x2),
            *&puVar14[-0x4].field6_0x6,
        );
        SelectObject16(&puVar14[-0x6].field_0x2, *&puVar14[-0x4].field6_0x6);
    }
    uVar5 = puVar14.field6_0x6;
    piVar17 = pass1_1010_4dc8((uVar5 + 0x6));
    uVar13 = (piVar17 >> 0x10);
    uVar11 = piVar17;
    pass1_1020_239c(puVar14.field6_0x6, piVar17);
    uVar6 = puVar14.field6_0x6;
    uVar4 = (uVar6 + 0x6);
    if ((uVar4 + 0x2c) != 0) {
        pass1_1020_2488(uVar11, uVar13, puVar14.field6_0x6);
    }
    uVar6 = puVar14.field6_0x6;
    obj = SelectPalette16(0x0, (uVar6 + 0x10), *&puVar14[-0x4].field6_0x6);
    DeleteObject16(obj);
    hgdiobj16_var10 = SelectObject16((puVar14 - 0x6), *&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    hgdiobj16_var10 = SelectObject16(&puVar14[-0x7].field6_0x6, *&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    uVar6 = puVar14.field6_0x6;
    EndPaint16(
        CONCAT22(0x1050, (&puVar14[-0x4].field6_0x6 + 0x2)),
        (uVar6 + 0x4),
    );
    return;
}

pub unsafe fn FUN_1010_2a32(
    buffer_param_2: *mut u8,
    mut param_2: u32,
    hfile_param: *mut HFILE16,
    mut param_4: u16,
) -> u16 {
    let mut piVar1: *mut i16;
    let mut pcVar2: *mut c_char;
    let mut pbVar3: *mut u8;
    let mut bVar4: u8;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u32;
    let mut ppcVar7: *mut *mut code;
    let mut pcVar8: *mut code;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut HVar11: HDC16;
    let mut HVar12: HPALETTE16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut BVar15: bool;
    let mut iVar16: i16;
    let mut bVar17: u8;
    let mut puVar18: *mut u8;
    let mut puVar19: *mut u8;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut in_EDX: u32;
    let mut in_BX: *mut u32;
    let mut unaff_BP: u16;
    let mut unaff_SI: i16;
    let mut iVar23: i16;
    let mut iVar24: i16;
    let mut unaff_ES: u16;
    let mut uVar25: u16;
    let mut bVar26: u8;
    let mut bVar27: bool;
    let mut init_data: *mut DEVMODEA;
    let mut in_stack_00000000: i16;
    let mut in_stack_00000002: u16;
    let mut uStack54: u16;
    let mut puStack46: *mut u16;
    let mut uStack42: u16;
    let mut read_buffer_38: *mut u8;
    let mut read_buffer_22: *mut u8;
    let mut HStack30: HGDIOBJ16;
    let mut HStack28: HGDIOBJ16;
    let mut in_stack_0000ffec: u8;
    let mut uVar28: u8;
    let mut in_stack_0000ffed: u8;
    let mut uVar29: u8;
    let mut uVar30: u8;
    let mut uVar31: u8;
    let mut uVar32: u8;
    let mut uVar33: u8;
    let mut uVar34: u8;
    let mut uVar35: u8;
    let mut uVar36: u8;
    let mut uVar37: u8;
    let mut uVar38: u8;
    let mut paVar22: *mut Struct57;

    uVar28 = unaff_BP;
    uVar29 = (unaff_BP >> 0x8);
    bVar26 = 0;
    uVar38 = 0;
    //   if ((param_2 + 0xec76 & 0x3) != 0) goto LAB_1010_2ad8;
    uVar10 = param_2 + 0xec76 >> 0x1;
    //   if (0x1c < uVar10) goto LAB_1010_2ad8;
    uVar14 = in_EDX;
    //   switch(uVar10)
    match uVar10 {
        //   _ =>
        _ => {}
        // TODO: goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            (uVar10 + 0xa) = in_BX;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            (uVar10 + 0xa) = in_BX;
            (uVar10 + 0x10) = in_BX;
            (uVar10 + 0xc) = in_BX;
            return uVar10;
        }
        0x5 => {
            BVar15 = write_to_file_1008_7e1c(
                param_2,
                ZEXT24(in_BX),
                CONCAT13(
                    (in_stack_00000000 >> 0x8),
                    CONCAT12(in_stack_00000000, unaff_BP),
                ),
                CONCAT11(in_stack_0000ffed, in_stack_0000ffec),
            );
            if (BVar15 != 0) {
                return 0x1;
            }
            u16_1050_0310 = 0x6d0;
            return 0x0;
        }
        0x6 => {
            bVar26 = 0;
        }
        // TODO: goto LAB_1010_2ad8;
        0x7 => {
            ppcVar7 = *in_BX;
            (**ppcVar7)();
            iVar16 = param_2 + 0x105;
            puVar18 = in_EDX;
            pass1_1010_8170(puVar18, _u16_1050_14cc, iVar16);
            iVar23 = param_2 * 0x4;
            (buffer_param_2 + iVar23 + 0x26) = iVar16;
            (buffer_param_2 + iVar23 + 0x28) = puVar18;
            uVar36 = 0x50;
            uVar37 = 0x10;
            uVar34 = 0x80;
            uVar35 = 0x13;
            uVar30 = 0;
            uVar31 = 0;
            uVar32 = 0;
            uVar33 = 0;
            uVar28 = 0;
            uVar29 = 0;
            init_data = pass1_1008_4772((buffer_param_2 + 0x26 + iVar23));
            uVar25 = (init_data >> 0x10);
            HVar11 = CreateDC16(
                init_data,
                CONCAT13(uVar29, CONCAT12(uVar28, uVar25)),
                CONCAT13(uVar33, CONCAT12(uVar32, CONCAT11(uVar31, uVar30))),
                CONCAT13(uVar37, CONCAT12(uVar36, CONCAT11(uVar35, uVar34))),
            );
            uVar28 = HVar11;
            uVar29 = (HVar11 >> 0x8);
            HVar12 = draw_a::palette_op_1008_4e08(
                &stack0xffec,
                uVar25,
                (_PTR_LOOP_1050_4230 + 0xe),
                CONCAT13(0x10, CONCAT12(0x50, &stack0xffec)),
            );
            HStack28 = SelectObject16(CONCAT11(uVar38, bVar26), CONCAT11(uVar29, uVar28));
            HStack30 = SelectObject16(CONCAT11(uVar29, uVar28), CONCAT11(uVar29, uVar28));
            read_buffer_22 = 0;
            piVar1 = buffer_param_22 + 0x74;
            // for (read_buffer_22 = 0; piVar1 = (buffer_param_2 + 0x74),
            // *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1; read_buffer_22 += 1)
            while *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1 {
                iVar16 = (read_buffer_22 * 0x10 + param_2) * 0x8;
                uVar25 = (buffer_param_2 + 0x72);
                uVar13 = pass1_1000_484c(
                    CONCAT13(0x10, CONCAT12(0x50, &stack0xfff2)),
                    CONCAT13(
                        (uVar25 >> 0x8),
                        CONCAT12(uVar25, iVar16 + (buffer_param_2 + 0x70)),
                    ),
                    0x8,
                );
                if (uVar13 != 0) {
                    uVar9 = (buffer_param_2 + 0x70);
                    uVar25 = (uVar9 >> 0x10);
                    iVar23 = uVar9;
                    iVar24 = iVar16 + iVar23;
                    Rectangle16(
                        (iVar24 + 0x6),
                        (iVar24 + 0x4),
                        (iVar24 + 0x2),
                        (iVar23 + iVar16),
                        CONCAT11(uVar29, uVar28),
                    );
                }
                read_buffer_22 += 1;
            }
            HVar12 = SelectPalette16(0x0, HVar12, CONCAT11(uVar29, uVar28));
            DeleteObject16(HVar12);
            SelectObject16(HStack28, CONCAT11(uVar29, uVar28));
            SelectObject16(HStack30, CONCAT11(uVar29, uVar28));
            uVar38 = 0x38;
            uVar30 = 0x15;
            DeleteDC16(CONCAT11(uVar29, uVar28));
            BVar15 = DeleteObject16(CONCAT11(uVar30, uVar38));
            return BVar15;
        }
        0x8 => {
            bVar26 = 0x3;
        }
        // TODO: goto LAB_1010_2ad8;
        0xd => {
            pbVar3 = (uVar10 + unaff_SI);
            bVar26 = *pbVar3;
            bVar4 = *pbVar3 + in_EDX;
            *pbVar3 = bVar4 + (uVar10 < 0x1c);
            puVar5 = (CARRY1(bVar26, in_EDX) || CARRY1(bVar4, uVar10 < 0x1c));
            puVar6 = in_BX -0x404;
            bVar26 = in_BX < 0x1010 || puVar6 < puVar5;
            uVar21 = puVar6 - puVar5;
            pcVar8 = swi(0x4);
            if (SBORROW2(in_BX, 0x1010) != SBORROW2(puVar6, puVar5)) {
                (*pcVar8)();
            }
            puVar19 = in_EDX;
            bVar27 = uVar21 < 0x1010 || uVar21 + 0xeff0 < bVar26;
            pbVar3 = (uVar10 + unaff_SI);
            bVar26 = *pbVar3;
            bVar17 = in_EDX;
            bVar4 = *pbVar3;
            *pbVar3 = bVar4 + bVar17 + bVar27;
            pcVar2 = (uVar10 + unaff_SI);
            *pcVar2 = *pcVar2 + bVar17 + (CARRY1(bVar26, bVar17) || CARRY1(bVar4 + bVar17, bVar27));
            struct_op_1018_4cda(
                CONCAT13(
                    in_stack_00000000,
                    CONCAT12(uVar29, CONCAT11(uVar28, uVar38)),
                ),
                CONCAT11(in_stack_00000002, (in_stack_00000000 >> 0x8)),
            );
            iVar16 = CONCAT11(uVar28, uVar38);
            uVar30 = in_stack_00000000;
            piVar1 = CONCAT13(uVar30, CONCAT12(uVar29, iVar16));
            *piVar1 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 1;
            (iVar16 + 0x2) = 0x1010;
            pass1_1018_4dce(puVar19, CONCAT13(uVar30, CONCAT12(uVar29, iVar16)), 0x1b3);
            _PTR_LOOP_1050_4230 = CONCAT13(uVar30, CONCAT12(uVar29, CONCAT11(uVar28, uVar38)));
            return CONCAT11(uVar28, uVar38);
        }
        0xe => {
            (buffer_param_2 + 0x20) = param_2;
        }

        0x11 => {
            // loop {
            // // WARNING: Do nothing block with infinite loop
            // }
        }
        0x12 => {
            PTR_LOOP_1050_10c6 = (0x0 < param_2);
            PTR_LOOP_1050_1142 = (0x2 < param_2);
        }
        0x13 => {
            iVar16 = buffer_param_2 * 0x8 + in_stack_00000000;
            if ((((iVar16 + 0x22) != 0) || ((iVar16 + 0x24) != 0))
                || ((iVar16 + 0x26) != 0x0 || ((iVar16 + 0x28) != 0)))
            {
                HStack28 = 0x1010;
                HStack30 = 0x627c;
                sys_1000_3f9c(
                    *(in_stack_00000000 + 0xe),
                    s__d__d__d__d_1050_14ae,
                    (buffer_param_2 * 0x8 + in_stack_00000000 + 0x22),
                );
                HStack28 = 0x1000;
                HStack30 = 0x62a0;
                in_BX = WritePrivateProfileString16(
                    *(in_stack_00000000 + 0xa),
                    *(in_stack_00000000 + 0xe),
                    *(buffer_param_2 * 0x4 + 0x1446),
                    s_windows_1050_13b8,
                );
            }
            return in_BX;
        }
        0x14 => {
            (buffer_param_2 + 0x24) = param_2;
        }

        0x17 => {
            uVar21 = uVar14 - 0x1;
            paVar22 = (in_EDX & 0xffff0000 | uVar21);
            pbVar3 = (uVar10 + unaff_SI);
            *pbVar3 = *pbVar3 | uVar21;
            (buffer_param_2 + 0x12) = in_BX;
            (buffer_param_2 + 0x14) = uVar21;
            uStack42 = 0;
            loop {
                if (uStack54 <= uStack42) {
                    uVar38 = (buffer_param_2 >> 0x10);
                    BVar15 = read_file_1008_7dee(
                        param_2,
                        ((buffer_param_2 & 0xff00) << 0x10
                            | CONCAT12(uVar38, buffer_param_2 + 0x1a)),
                        0x2,
                    );
                    if (((BVar15 != 0)
                        && (
                            BVar15 = read_file_1008_7dee(
                                param_2,
                                ((buffer_param_2 & 0xff00) << 0x10
                                    | CONCAT12(uVar38, buffer_param_2 + 0x1c)),
                                0x2,
                            ),
                            BVar15 != 0,
                        ))
                        && (
                            BVar15 = read_file_1008_7dee(
                                param_2,
                                ((buffer_param_2 & 0xff00) << 0x10
                                    | CONCAT12(uVar38, buffer_param_2 + 0x1e)),
                                0x2,
                            ),
                            BVar15 != 0,
                        ))
                    {
                        return 0x1;
                    }
                    u16_1050_0310 = 0x6d2;
                    return 0x0;
                }
                uVar10 = uStack54;
                mem_op_1000_179c(0x8, paVar22);
                uVar21 = paVar22;
                puStack46 = CONCAT22(uVar21, uVar10);
                paVar22 = (paVar22 & 0xffff0000 | (uVar21 | uVar10));
                if ((uVar21 | uVar10) == 0) {
                    read_buffer_38 = null_mut();
                } else {
                    *puStack46 = 0x389a;
                    (uVar10 + 0x2) = 0x1008;
                    *puStack46 = 0xa1c4;
                    (uVar10 + 0x2) = 0x1010;
                    read_buffer_38 = puStack46;
                }
                BVar15 = read_file_1008_7dee(
                    param_2,
                    CONCAT13(0x10, CONCAT12(0x50, &read_buffer_22)),
                    0x2,
                );
                if (BVar15 == 0)
                    || (
                        BVar15 = read_file_1008_7dee(
                            param_2,
                            (read_buffer_38 & 0xff000000
                                | CONCAT12((read_buffer_38 >> 0x10), read_buffer_38 + 0x6)),
                            0x2,
                        ),
                        BVar15 == 0,
                    )
                {}
                iVar16 = switch_1008_73ea(param_2, param_2, read_buffer_22);
                (read_buffer_38 + 0x4) = iVar16;
                ppcVar7 = ((buffer_param_2 + 0x12) + 0x4);
                (**ppcVar7)();
                uStack42 += 0x1;
            }
            if read_buffer_38.is_null() {
                u16_1050_0310 = 0x6d2;
                return 0x0;
            }
            ppcVar7 = read_buffer_38;
            (**ppcVar7)();
            u16_1050_0310 = 0x6d2;
            return 0x0;
        }
        0x18 => {
            bVar26 = 0x2;
        }
        // TODO: goto LAB_1010_2ad8;
        0x19 => {
            uVar14 = pass1_1010_6ca2(uVar14, buffer_param_2, 0x8);
            uVar20 = in_EDX;
            if uVar14 != 0 {
                pass1_1010_715c(uVar14, uVar20, CONCAT22(0x1a, buffer_param_2), 0x1a);
                buffer_param_2 = 0x1a001a;
            }
            if param_2 == 0x2c {
                pass1_1010_715c(uVar14, uVar20, CONCAT22(0x1d, buffer_param_2), 0x1d);
            }
            uVar14 = pass1_1010_6ca2(uVar20, 0x5a, 0x2);
            if uVar14 != 0 {
                pass1_1010_715c(uVar14, uVar20, 0x1c005a, 0x1c);
            }
            return 0x1;
        }
        0x1a => {
            (buffer_param_2 + 0x26) = param_2;
        }
    };
    bVar26 = 0x1; //
                  // LAB_1010_2ad8:
    if (bVar26 == 1) || (bVar26 == 0x2) {
        if bVar26 == 1 {
            param_2 = ((buffer_param_2 + 0x20)
                + (buffer_param_2 + 0x22)
                + (buffer_param_2 + 0x24)
                + (buffer_param_2 + 0x26));
        }
        if param_2.is_null() == false {
            uVar10 = param_2 / 0x2 + 1;
            if 0x5 < uVar10 {
                uVar10 = 0x5;
            }
            param_2 = uVar10;
            if (bVar26 == 1) && (PTR_LOOP_1050_10c6.is_null() == false) {
                if 0x4 < uVar10 {
                    uVar10 = 0x4;
                }
                param_2 = uVar10;
            }
        }
    }
    (bVar26 * 0x7c + 0xed6) = param_2;
    in_BX = param_2;
    pass1_1010_1f62(buffer_param_2, 0xc);
    // switchD_1010_2ab5_caseD_0:
    return in_BX;
}
