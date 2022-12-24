pub mod draw_a;
pub mod draw_b;
pub mod draw_c;
pub mod draw_d;
pub mod draw_e;
pub mod draw_f;

use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1000_3000::{str_op_1000_3da4, sys_1000_3f9c};
use crate::unk::block_1000_4000::pass1_1000_484c;
use crate::unk::block_1008_4000::{pass1_1008_4772, pass1_1008_4d72};
use crate::unk::block_1008_7000::switch_1008_73ea;
use crate::unk::block_1010_1000::pass1_1010_1f62;
use crate::unk::block_1010_6000::pass1_1010_6ca2;
use crate::unk::block_1010_7000::pass1_1010_715c;
use crate::unk::block_1010_8000::{FUN_1010_830a, pass1_1010_8170};
use crate::unk::block_1018_4000::{pass1_1018_4dce, struct_op_1018_4cda};
use crate::unk::block_1018_6000;
use crate::unk::block_1018_6000::{pass1_1018_642e, pass1_1018_6630};
use crate::unk::block_1020_2000::{pass1_1020_2286, pass1_1020_239c, pass1_1020_2488};
use crate::unk::block_1040_9000;
use crate::dos_interrupt::swi;
use crate::{file_ops, gui, sound_ops, winapp};
use crate::file_ops::{read_file_1008_7dee, write_to_file_1008_7e1c};
use crate::globals::{DAT_1050_1050, DAT_1050_4216, DAT_1050_422c};
use crate::structs::struct_57::Struct57;
use crate::unk::{block_1008_4000, block_1008_6000, block_1020_3000, block_1020_6000, block_1020_7000};
use crate::utils::{CONCAT11, CONCAT22};
use crate::gui::{cleanup, window};
use crate::gui::window::win_c;
use crate::winapi16::{CreateDC16, CreatePalette16, DeleteDC16, DeleteObject16, GetStockObject16, InvalidateRect16, RealizePalette16, Rectangle16, SelectObject16, SelectPalette16, SetBkColor16, SetTextColor16, TextOut16, WritePrivateProfileString16};
use crate::winapp::winapp_a;
use crate::windef16::{COLORREF, DEVMODEA, HDC16, HFILE16, HGDIOBJ16, HPALETTE16, HPEN16, HWND16, LOGPALETTE, RECT16};


pub unsafe fn unk_draw_op_1040_9458(param_1: *mut astruct_17, param_2: u8, mut param_3: u16)

{
    let mut ppcVar1: *mut *mut code;
    let mut hpal: *mut u16;
    let mut obj: HPALETTE16;
    let mut uVar3: u16;
    let mut iVar4: *mut astruct_17;
    let mut uVar4: u16;
    let mut puStack8: *mut u16;
    let mut puStack6: *mut u32;
    let mut UVar2: u32;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (iVar4.field8_0x8.is_null() == false) {
        puStack6 = iVar4.field8_0x8;
        if ((((&iVar4.field9_0xc + 0x2) | &iVar4.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack6 = iVar4.field9_0xc;
        }
        if ((iVar4.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
            puStack6 = iVar4.field10_0x10;
        }
        hpal = &param_3;
        UVar2 = *puStack6;
        ppcVar1 = (UVar2 + 0x8);
        (**ppcVar1)();
        ppcVar1 = (UVar2 + 0x4);
        (**ppcVar1)();
        obj = SelectPalette16(0x0, hpal, param_3);
        DeleteObject16(obj);
    }
    return;
}


pub unsafe fn palette_op_1040_c886(struct_param_1: *mut astruct_769, param_2: u8, hdc_param_3: HDC16)

{
    let mut uVar1: u16;
    let mut palette_2: HPALETTE16;
    let mut uVar4: u16;
    let mut struct_2: *mut astruct_769;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut puStack8: *mut u32;
    let mut palette: HPALETTE16;
    let mut fn_ptr_1: *mut *mut code;

    uVar2 = (struct_param_1 >> 0x10);
    struct_2 = struct_param_1;
    if (((&struct_2.field8_0x8 + 0x2) | &struct_2.field8_0x8) != 0) {
        palette = 0;
        if (struct_2.field59_0x46 == 0) {
            uVar3 = (_PTR_LOOP_1050_4230 >> 0x10);
            uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
            unaff_CS = 0x1008;
            palette = draw_a::palette_op_1008_4e08(&hdc_param_3, uVar1,
                                                   CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)),
                                                   CONCAT22(0x1050, &hdc_param_3));
        }
        puStack8 = struct_2.field8_0x8;
        uVar4 = (&struct_2.field8_0x8 + 2);
        if ((((&struct_2.field9_0xc + 0x2) | &struct_2.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack8 = struct_2.field9_0xc;
            uVar4 = (&struct_2.field9_0xc + 2);
        }
        if ((struct_2.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
            puStack8 = struct_2.field10_0x10;
            uVar4 = (&struct_2.field10_0x10 + 2);
        }
        fn_ptr_1 = (*puStack8 + 0x4);
        (**fn_ptr_1)(unaff_CS, puStack8, uVar4, struct_2.field30_0x28, struct_2.field29_0x26, &hdc_param_3,
                     0x1050);
        if (struct_2.field59_0x46 == 0) {
            palette_2 = SelectPalette16(0x0, palette, hdc_param_3);
            DeleteObject16(palette_2);
        }
    }
    return;
}


pub unsafe fn cleanup_palette_1008_56e2(mut param_1: u32, mut param_2: u32) -> BOOL16

{
  let mut hpalette_a: HPALETTE16;
  let mut u16_a: u16;

  u16_a = (param_1 >> 0x10);
  hpalette_a = SelectPalette16(0x0,(param_1 + 0x4),*param_2);
  (param_1 + 0x4) = hpalette_a;
  DeleteObject16(hpalette_a);
  return 0x1;
}

pub unsafe fn win_ui_palette_op_1020_81c0(mut param_1: u32) {
    let mut in_struct_1: *mut astruct_13;
    let mut hdc: HDC16;
    let mut hpal: HDC16;
    let mut hpal_00: HPALETTE16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut hdc_00: HDC16;
    let mut uStack6: u16;

    uVar2 = (_PTR_LOOP_1050_4230 >> 0x10);
    in_struct_1 = (_PTR_LOOP_1050_4230 + 0xe);
    uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
    uStack6 = in_struct_1;
    if ((uVar1 | uStack6) == 0) {
        return;
    }
    hdc = GetDC16((param_1 + 0x8));
    hpal = hdc;
    hdc_00 = hdc;
    create_palette_1008_4e38(in_struct_1, uVar1);
    hpal_00 = SelectPalette16(0x0, hpal, hdc_00);
    RealizePalette16(hdc);
    SelectPalette16(0x1, hpal_00, hdc);
    RealizePalette16(hdc);
    DeleteObject16(hpal);
    if (0x0 < hpal) {
        InvalidateRect16(0x1, NULL, 0x0);
    }
    return;
}

pub unsafe fn unk_draw_op_1008_da12(param_1: *mut Struct19, mut param_2: u16) {
    let mut bVar1: u8;
    let mut hdc: HDC16;
    let mut horiz_res: i16;
    let mut vert_res: i16;
    let mut iVar2: i16;
    let mut raster_caps: u16;
    let mut sz_palette: i16;
    let mut num_reserved: i16;
    PALETTEENTRY * entries;
    let mut uVar4: u16;
    let mut start: u16;
    let mut count: *mut u8;
    let mut count_00: u16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut puStack32: *mut u16;
    let mut iStack16: i16;
    let mut lStack8: i32;
    let mut uVar3: u32;
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut pSVar2: *mut StructD;

    uVar5 = (in_EDX >> 0x10);
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    pass1_1008_3e38((param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), param_1 + 0xe)));
    (param_1 + 0x14) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x18) = 0;
    param_1.offset_0x0 = 0xdc80;
    (param_1 + 0x2) = 0x1008;
    hdc = GetDC16(0x0);
    horiz_res = GetDeviceCaps16(HORZRES, hdc);
    (param_1 + 0xa) = horiz_res;
    vert_res = GetDeviceCaps16(VERTRES, hdc);
    (param_1 + 0xc) = vert_res;
    iVar2 = (param_1 + 0xc) -0x1e0;
    count = (iVar2 >> 0xf);
    pSVar2 = CONCAT22(uVar5, count);
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (param_1 + 0xe)),
        0x0,
        iVar2 / 0x2,
        ((param_1 + 0xa) -0x280) / 0x2,
    );
    raster_caps = GetDeviceCaps16(RASTERCAPS, hdc);
    if ((raster_caps & 0x100) != 0) {
        sz_palette = GetDeviceCaps16(SIZEPALETTE, hdc);
        (param_1 + 0x14) = sz_palette;
        num_reserved = GetDeviceCaps16(NUMRESERVED, hdc);
        (param_1 + 0x16) = num_reserved;
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
        } else {
            pSVar2 = (pSVar2 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
        }
        entries = fn_ptr_op_1000_1708(
            (num_reserved + 1) * 0x4,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            pSVar2,
        );
        count_00 = pSVar2;
        lStack8 = CONCAT22(count_00, entries);
        iVar6 = (param_1 + 0x16);
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
            PTR_LOOP_1050_5f2e = pSVar2;
        } else {
        }
        uVar4 = fn_ptr_op_1000_1708(
            (iVar6 + 1) * 0x4,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            PTR_LOOP_1050_5f2e,
        );
        (param_1 + 0x18) = uVar4;
        (param_1 + 0x1a) = PTR_LOOP_1050_5f2e;
        if (lStack8 != 0) {
            if ((param_1 + 0x18) != 0) {
                start = (param_1 + 0x16) / 0x2;
                GetSystemPaletteEntries(entries, count_00, start, 0x0);
                GetSystemPaletteEntries(entries + start, count_00, start, (param_1 + 0x14) - start);
                puStack32 = (param_1 + 0x18);
                // for (iStack16 = 0; puVar2 = puStack32, piVar1 = (param_1 + 0x16),
                // *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 1)
                iStack16 = 0;
                puVar2 = (param_1 + 0x16);
                while (*piVar1 != iStack16) && (iStack16 <= *pivar1) {
                    bVar1 = (entries + iStack16).pe_red;
                    uVar3 = puStack32 >> 0x10;
                    iVar6 = puStack32;
                    puStack32 = (puStack32 & 0xffff0000 | (iVar6 + 0x4));
                    *puVar2 = CONCAT11(entries[iStack16].pe_green, entries[iStack16].pe_blue);
                    (iVar6 + 0x2) = bVar1;
                    iStack16 += 1;
                }
            }
        }
        fn_ptr_1000_17ce(CONCAT22(count_00, entries));
    }
    ReleaseDC16(hdc, 0x0);
    return;
}


pub unsafe fn unk_win_ui_op_1038_9bc8(mut param_1: u16, mut param_2: u16, mut param_3: u16, param_4: *mut StructB)

{
  let mut IVar2: i16;
  let mut iVar2: i16;
  let mut hdc: HDC16;
  let mut IVar1: i16;
  let mut HVar2: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
   let mut struct_b_7: *mut StructB;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u16;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut iStack36: i16;
  let mut local_16: [u8;0x2] = [0;0x2];
  let mut iStack20: i16;
  let mut iStack16: i16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut local_6: i16;
  let mut local_4: i16;
  let mut iVar3: *mut astruct_778;
  let mut piVar1: *mut i16;
  let mut in_stack_0000ffc6: u32;
  let mut uVar16: u16;
  let mut fn_ptr_1: *mut *mut code;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_4);
  if (PTR_LOOP_1050_5ef8 == (&u32_1050_0004 + 1)) {
    PTR_LOOP_1050_5ef8 = null_mut();
  }
  piVar8 = &local_4;
  uVar9 = SUB42(0x1050,0x0);
  piVar6 = &local_6;
  uVar7 = SUB42(0x1050,0x0);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(piVar6,0x48),in_stack_0000fe60,in_stack_0000ff84
                           ,in_stack_0000ff8a,in_stack_0000ff8e);
  uVar4 = (paVar3 >> 0x10);
  uStack10 = puVar5;
  uStack10 = (puVar5 >> 0x10);
  pass1_1008_3e94((puVar5 & 0xffff0000 | (uStack10 + 0xe)),CONCAT22(uVar7,piVar6),
                  CONCAT22(uVar9,piVar8));
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  paVar3 = CONCAT22(uVar4,((IVar2 * PTR_LOOP_1050_5ef8) >> 0x10));
  iVar2 = (IVar2 * PTR_LOOP_1050_5ef8) + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
  uStack14 = CONCAT22(iVar2 + local_4,iVar2 + local_6);
  uVar4 = (param_4 >> 0x10);
  struct_b_7 = param_4;
  GetWindowRect16(CONCAT22(0x1050,local_16),struct_b_7.lpvoid_field_0x8);
  hdc = GetDC16(0x0);
  IVar1 = GetDeviceCaps16(VERTRES,hdc);
  ReleaseDC16(hdc,0x0);
  if (IVar1 < iStack16) {
    uStack14 = uStack14 & 0xffff0000 | ((iStack20 - (iStack16 - IVar1)) + 1);
  }
  SetWindowPos16(0x1,0x0,0x0,uStack14,(uStack14 >> 0x10),0x0,struct_b_7.lpvoid_field_0x8);
  _param_3 = CONCAT22(param_2,0x3);
  uVar9 = 0x1010;
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,_param_3,in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  uVar7 = (puVar5 >> 0x10);
  iStack36 = 0;
  while (iVar3 = (iStack36 * 0x2), (&iVar3[0x52].field_0x0 + puVar5) != 0) {
    _param_3 = (_param_3 & 0xffff0000);
    uVar9 = SUB42(0x1538,0x0);
    HVar2 = GetDlgItem16((&iVar3[0x52].field_0x0 + puVar5),struct_b_7.lpvoid_field_0x8);
    (&iVar3[0x4a].field_0x0 + &struct_b_7.field0_0x0) = HVar2;
    iStack36 += 0x1;
    piVar1 = &struct_b_7[0xe].field8_0x10;
    *piVar1 = *piVar1 + 1;
  }
  fn_ptr_1 = (param_4 + 0x6c);
  (**fn_ptr_1)(uVar9,param_4,param_2);
  return;
}

pub unsafe fn set_struct_op_1008_0536(param_1: *mut astruct_20, mut param_2: u16) {
    let mut hicon_1: HICON16;
    let mut hcursor_1: HCURSOR16;
    let mut hbrush_1: HGDIOBJ16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut paVar1: *mut Struct57;
    let mut paVar3: *mut astruct_20;
    let mut puVar4: *mut u32;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd6: u16;
    let mut in_stack_0000ffda: u16;

    uVar2 = (in_EDX >> 0x10);
    paVar3 = pass1_1008_3ab8(param_1);
    paVar1 = CONCAT22(uVar2, (paVar3 >> 0x10));
    (param_1 + 0xe0) = 0;
    (param_1 + 0xe4) = 0;
    (param_1 + 0xe8) = 0;
    (param_1 + 0xec) = 0;
    (param_1 + 0xee) = 0;
    (param_1 + 0xf2) = 0;
    (param_1 + 0xf4) = 0;
    (param_1 + 0xf8) = 0;
    param_1.offset_0x0 = 0x389e;
    (param_1 + 0x2) = 0x1008;
    (param_1 + 0xc8) = 0x2008;
    (param_1 + 0xac) = 0;
    (param_1 + 0xae) = 0x8700;
    hicon_1 = LoadIcon16(s_Op_1050_00d4, HINSTANCE16_1050_038c);
    (param_1 + 0xc2) = hicon_1;
    hcursor_1 = LoadCursor16(0x7f00, 0x0);
    (param_1 + 0xc4) = hcursor_1;
    hbrush_1 = GetStockObject16(BLACK_BRUSH);
    (param_1 + 0xc6) = hbrush_1;
    puVar4 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0 & 0xffff,
        CONCAT22(param_1, 0x48),
        in_stack_0000feac,
        in_stack_0000ffd0,
        in_stack_0000ffd6,
        in_stack_0000ffda,
    );
    paVar1 = (paVar1 & 0xffff0000 | puVar4 >> 0x10);
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0xa)),
        s_Outpost_1050_00d7,
    );
    puVar4 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0 & 0xffff,
        CONCAT22(param_1, 0x32),
        in_stack_0000feac,
        in_stack_0000ffd0,
        in_stack_0000ffd6,
        in_stack_0000ffda,
    );
    (param_1 + 0xf4) = puVar4;
    (param_1 + 0xf6) = (puVar4 >> 0x10);
    draw_a::set_sys_color_1008_357e(param_1, 0x1, paVar1 & 0xffff0000 | puVar4 >> 0x10);
    return;
}


pub unsafe fn unk_draw_op_1008_61b2(
    mut param_1: u16,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) -> *mut astruct_20 {
    let mut l_hgdiobj_1: HGDIOBJ16;
    let mut l_hcursor_1: HCURSOR16;
    let mut in_EDX: *mut Struct57;
    let mut iVar2: *mut astruct_20;
    let mut uVar2: u16;
    let mut l_struct_2: *mut u16;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbe: u16;
    let mut uVar5: *mut astruct_20;
    let mut in_stack_0000ffe8: u16;
    let mut iVar1: *mut astruct_20;
    let mut iVar4: *mut astruct_20;
    let mut uVar1: *mut u16;

    block_1008_6000::set_struct_1008_687a(param_2, param_5);
    uVar2 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2.field164_0xde = param_3;
    iVar2.field165_0xe0 = 0;
    param_2.field0_0x0 = 0x6378;
    iVar2.base_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | ZEXT24(&iVar2.field60_0x5b)),
        s_DanBrotherton_1050_0302,
    );
    l_hgdiobj_1 = GetStockObject16(BLACK_BRUSH);
    iVar2.hgdiobj_field_0xc6 = l_hgdiobj_1;
    l_hcursor_1 = LoadCursor16(0x7f00, 0x0);
    iVar2.hcursor_field_0xc4 = l_hcursor_1;
    iVar2.field150_0xc8 = 0x200b;
    iVar2.field139_0xac = 0x45000000;
    iVar2.field145_0xbc = (param_5 + 0x8);
    l_struct_2 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(param_1, 0x48),
        in_stack_0000fe90,
        in_stack_0000ffb4,
        in_stack_0000ffba,
        in_stack_0000ffbe,
    );
    uVar1 = (l_struct_2 >> 0x10);
    iVar2.field141_0xb4 = 0;
    iVar2.field142_0xb6 = 0;
    iVar2.field143_0xb8 = (l_struct_2 + 0xa);
    iVar2.field144_0xba = (l_struct_2 + 0xc);
    iVar2.field151_0xca = param_4;
    win_ui_reg_class_1008_96d2(param_2);
    return param_2;
}

pub unsafe fn unk_draw_op_1008_7f62(
    param_1: *mut astruct_20,
    param_2: u16,
    param_3: u32,
) -> *mut astruct_20 {
    let mut HVar1: HGDIOBJ16;
    let mut HVar2: HCURSOR16;
    let mut iVar4: *mut astruct_20;
    let mut uVar3: u16;
    let mut uVar4: *mut astruct_20;
    let mut iVar3: *mut astruct_20;

    set_struct_1008_687a(param_1, param_3);
    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field164_0xde = param_2;
    param_1.offset_0x0 = 0x8042;
    iVar4.base_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field60_0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    iVar4.hgdiobj_field_0xc6 = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0x0);
    iVar4.hcursor_field_0xc4 = HVar2;
    iVar4.field150_0xc8 = 0x2008;
    iVar4.field139_0xac = 0x44000000;
    iVar4.field145_0xbc = (param_3 + 0x8);
    iVar4.field151_0xca = iVar4.field164_0xde;
    win_ui_reg_class_1008_96d2(param_1);
    return param_1;
}


pub unsafe fn unk_win_ui_op_1020_67ce(
    in_struct_1: *mut StructA,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut HVar1: HGDIOBJ16;
    let mut hcursor_2: HCURSOR16;
    let struct_a_1_lo: *mut StructA;
    let struct_a_1_hi: *mut StructA;
    let mut in_stack_0000fe10: u16;
    let mut in_stack_0000fe14: u16;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000ff3a: u16;
    let mut in_stack_0000ff3e: u16;
    let mut in_stack_0000ff42: u16;
    let mut in_stack_0000ff82: u16;
    let mut in_stack_0000ff8a: u16;
    let mut in_stack_0000ff90: u16;
    let mut in_stack_0000ff94: u16;

    struct_1020_790e(
        &in_struct_1.field0_0x0,
        s_TPPOPMENU_1050_43fa,
        param_2,
        param_3,
    );
    struct_a_1_hi = (in_struct_1 >> 0x10);
    struct_a_1_lo = in_struct_1;
    struct_a_1_lo[0x1].field20_0x26 = 0;
    struct_a_1_lo[0x1].field22_0x2a = 0;
    in_struct_1.field0_0x0 = 0x70e6;
    struct_a_1_lo.field1_0x2 = 0x1020;
    unk_str_op_1000_3d3e(
        (in_struct_1 & 0xffff0000 | ZEXT24(&struct_a_1_lo.field60_0x5b)),
        s_VrMode2_1050_4404,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    struct_a_1_lo.field157_0xc6 = HVar1;
    hcursor_2 = LoadCursor16(0x7f00, 0x0);
    struct_a_1_lo.field156_0xc4 = hcursor_2;
    struct_a_1_lo.field140_0xac = 0x44c00000;
    struct_a_1_lo.field158_0xc8 = 0x2020;
    struct_a_1_lo.field149_0xbc = (param_3 + 0x8);
    struct_a_1_lo.field159_0xca = param_2;
    win_ui_reg_class_1008_96d2(in_struct_1);
    win_c::window_op_1020_6c3a(
        param_4,
        in_struct_1,
        in_stack_0000fe66,
        in_stack_0000ff82,
        in_stack_0000ff8a,
        in_stack_0000ff90,
        in_stack_0000ff94,
        in_stack_0000fe10,
        in_stack_0000fe14,
        in_stack_0000ff3a,
        in_stack_0000ff3e,
        in_stack_0000ff42,
    );
    return;
}

pub unsafe fn win_ui_op_1020_737a(mut param_1: u16, param_2: *mut astruct_788) -> BOOL16 {
    let mut uVar8: u16;
    let mut is_iconic: bool;
    let mut puVar3: *mut c_char;
    let mut RVar9: RECT16;
    let mut in_DX: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut uVar10: u32;
    let mut struct_1: *mut astruct_788;
    let mut uVar6: u16;
    let mut local_42: [u8; 0x6] = [0; 0x6];
    let mut local_brush_handle: RECT16;
    let mut iStack56: i16;
    let mut HStack54: HWND16;
    let mut HStack52: HWND16;
    let mut iStack50: i16;
    let mut rect_30: RECT16;
    let mut iStack44: i16;
    let mut iStack42: i16;
    let mut local_rect: HGDIOBJ16;
    let mut hicon_38: bool;
    let mut hdc_24: HDC16;
    let mut local_paint_struct: [u8; 0x20] = [0; 0x20];
    let mut uVar3: *mut astruct_126;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar4: *mut astruct_126;
    let mut uVar7: u8;
    let mut fn_ptr_1: *mut *mut code;

    uVar6 = (param_2 >> 0x10);
    struct_1 = param_2;
    hdc_24 = BeginPaint16(
        CONCAT13(0x10, CONCAT12(0x50, local_paint_struct)),
        struct_1.hwnd_0x4,
    );
    is_iconic = IsIconic16(struct_1.hwnd_0x4);
    if (is_iconic == 0) {
        uVar4 = struct_1.field22_0x1c;
        RVar9 = *(uVar4 + 0x24);
        local_brush_handle = RVar9;
        pass1_1018_2e5e(SUB42(RVar9, 0x0), in_DX, struct_1.field22_0x1c);
        rect_30 = RVar9 & 0xffff | in_DX << 0x10;
        pass1_1008_3e54(CONCAT13(0x10, CONCAT12(0x50, local_42)), 0x0, 0x35, 0xc);
        if (&struct_1.field19_0x14 != 0) {
            pass1_1008_5236(&struct_1.field19_0x14);
        }
        if (rect_30 != 0) {
            uVar1 = struct_1.field19_0x14;
            uVar8 = struct_1.field20_0x16;
            uVar10 = CONCAT22(in_register_0000000a, uVar8);
            if ((uVar8 | uVar1) != 0) {
                pass1_1008_5118(CONCAT22(uVar8, uVar1));
                fn_ptr_1000_17ce(CONCAT22(uVar8, uVar1));
            }
            puVar3 = local_42;
            pass1_1008_8ce4(
                rect_30,
                CONCAT22(0x1050, puVar3),
                local_brush_handle,
                uVar10,
            );
            struct_1.field19_0x14 = puVar3;
            struct_1.field20_0x16 = uVar10;
        }
        fn_ptr_1 = (local_brush_handle + 0x4);
        (**fn_ptr_1)(
            0x1008,
            SUB42(local_brush_handle, 0x0),
            (local_brush_handle >> 0x10),
            0x0,
            0x0,
            &hdc_24,
            0x1050,
        );
        fn_ptr_1 = (*struct_1.field21_0x18 + 0x94);
        (**fn_ptr_1)(0x1008, struct_1.field21_0x18, 1);
        HStack52 = GetDlgItem16(0x1797, struct_1.hwnd_0x4);
        if (HStack52 != 0) {
            ShowWindow16(0x1, HStack52);
        }
    } else if (PTR_LOOP_1050_0010.is_null()) {
        uVar3 = struct_1.field22_0x1c;
        fn_ptr_1 = (struct_1.field22_0x1c + 0x2c);
        (**fn_ptr_1)(0x1538, uVar3, (uVar3 >> 0x10));
        hicon_38 = is_iconic;
        if (is_iconic != 0) {
            local_rect = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(&rect_30, 0x1050);
            local_brush_handle = 0x0;
            iStack56 = (iStack44 - rect_30.x) - 0x1;
            HStack54 = (iStack42 - rect_30.y) - 0x1;
            HStack52 = HStack54;
            iStack50 = iStack56;
            FillRect16(local_rect, &local_brush_handle, 0x1050);
            DrawIcon16(hicon_38, 0x2, 0x2, hdc_24);
        }
    }
    is_iconic = EndPaint16(CONCAT22(0x1050, local_paint_struct), struct_1.hwnd_0x4);
    return is_iconic;
}


pub unsafe fn unk_win_ui_op_1040_9854(param_1: *mut astruct_787) -> *mut u16

{
  let mut HVar1: HCURSOR16;
  let mut HVar2: HGDIOBJ16;
   let mut struct_1: *mut astruct_787;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.offset = 0x389a;
  struct_1.base = 0x1008;
  param_1.offset = 0xa230;
  struct_1.base = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&struct_1.field_0x4)),s_OPButton_1050_5ece);
  struct_1.field82_0x54 = 0x3;
  HVar1 = LoadCursor16(0x7f00,0x0);
  struct_1.field84_0x58 = HVar1;
  HVar2 = GetStockObject16(BLACK_BRUSH);
  struct_1.field83_0x56 = HVar2;
  winapp_a::reg_class_1040_98c0(param_1 & 0xffff | uVar3 << 0x10);
  return &param_1.offset;
}

pub unsafe fn get_stock_obj_1008_9c56() {
    GetStockObject16(HOLLOW_BRUSH);
    return;
}

pub unsafe fn realize_palette_1020_0e46(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut puVar4: HGDIOBJ16;
    let mut iVar4: *mut astruct_800;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut puVar2: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        puVar2 = iVar4.field102_0x66;
        fn_ptr_1 = (*puVar2 + 0x18);
        (**fn_ptr_1)();
        UnrealizeObject16(puVar2);
        uVar1 = (param_1 + 0xf2);
        RealizePalette16(*(uVar1 + 0x7c));
    }
    return;
}

pub unsafe fn realize_palette_1020_2992(mut param_1: u32, mut param_2: i16) {
    let mut obj: HGDIOBJ16;
    let mut hdc: HDC16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (param_2 != 0) {
        uVar1 = (param_1 >> 0x10);
        puVar1 = pass1_1018_0a50((param_1 + 0xf2));
        fn_ptr_1 = (*puVar1 + 0x18);
        obj = (**fn_ptr_1)(0x1018);
        UnrealizeObject16(obj);
        hdc = GetDC16((param_1 + 0x8));
        RealizePalette16(hdc);
    }
    return;
}

pub unsafe fn realize_palette_1020_6896(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar3: u32;
    let mut puVar4: u32;
    let mut iVar4: *mut astruct_801;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        puVar4 = iVar4.field36_0x24;
        ppcVar1 = (puVar4 + 0x18);
        (**ppcVar1)();
        UnrealizeObject16(puVar4);
        uVar3 = (param_1 + 0xf2);
        RealizePalette16(*(uVar3 + 0x178));
    }
    return;
}


pub unsafe fn create_palette_1008_4e38(in_struct_1: *mut astruct_13, mut param_2: u16) {
    let mut piVar1: *mut i16;
    let mut pLVar2: *mut LOGPALETTE;
    let mut iVar3: i16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut local_struct_1: *mut astruct_13;
    let mut iVar5: i16;
    let mut uVar8: *mut astruct_13;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iStack14: i16;
    let mut UpuStack12: *mut c_char;
    let mut UpuStack8: *mut c_char;
    let mut uVar3: *mut LOGPALETTE;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    uVar8 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    iVar3 = (local_struct_1.field9_0xc + 0x2) * 0x4;
    if (local_struct_1.field10_0xe.is_null()) {
        mem_op_1000_179c(iVar3, paVar4);
        local_struct_1.field10_0xe = iVar3;
        (&local_struct_1.field10_0xe + 0x2) = paVar4;
        local_struct_1.field10_0xe.pal_version = 0x300;
        uVar3 = local_struct_1.field10_0xe;
        (uVar3 + 0x2) = local_struct_1.field9_0xc;
        pLVar2 = local_struct_1.field10_0xe;
        puStack8 = (pLVar2 & 0xffff0000 | (pLVar2 + 0x4));
        puStack12 = local_struct_1.field4_0x4;
        iStack14 = 0;
        loop {
            piVar1 = &local_struct_1.field9_0xc;
            if (*piVar1 == iStack14 || *piVar1 < iStack14) {
                break;
            }
            uVar9 = (puStack12 >> 0x10);
            iVar3 = puStack12;
            *puStack8 = (iVar3 + 2);
            uVar10 = (puStack8 >> 0x10);
            iVar5 = puStack8;
            *(iVar5 + 1) = *(iVar3 + 1);
            (iVar5 + 0x2) = *puStack12;
            *(iVar5 + 0x3) = 0;
            iStack14 += 0x1;
            puStack8 = (puStack8 & 0xffff0000 | (iVar5 + 0x4));
            puStack12 = (puStack12 & 0xffff0000 | (iVar3 + 0x4));
        }
    }
    CreatePalette16(local_struct_1.field10_0xe);
    return;
}

pub unsafe fn begin_end_paint_1008_97c8(param_1: *mut astruct_837, mut param_2: u16) {
    BeginPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    EndPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    return;
}

pub unsafe fn misc_draw_op_1018_5d6c(param_1: *mut astruct_839) {
    let mut pa_var1: *mut astruct_76;
    let mut struct_4: *mut astruct_839;
    let mut u_var5: u16;
    let mut pa_var2: *mut astruct_76;
    let mut local_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut pu_var1: *mut u32;
    let mut u_var4: *mut astruct_134;
    let mut fn_ptr_1: *mut *mut code;

    u_var5 = (param_1 >> 0x10);
    struct_4 = param_1;
    BeginPaint16(CONCAT22(0x1050, &local_22), struct_4.field4_0x4);
    u_var4 = struct_4.pstruct134_0x14;
    pa_var1 = (u_var4 + 0xa);
    pa_var2 = pass1_1008_9f48(struct_4.pstruct134_0x14);
    pass1_1008_5236(struct_4.field20_0x18);
    pass1_1008_4480(
        pa_var1,
        (param_1 & 0xffff0000 | ZEXT24(struct_4 + 1)),
        pa_var2,
    );
    fn_ptr_1 = (pa_var1 + 0x4);
    (**fn_ptr_1)(
        0x1008,
        pa_var1,
        (pa_var1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | ZEXT24(&struct_4.field_0xa),
    );
    EndPaint16(CONCAT22(0x1050, &local_22), struct_4.field4_0x4);
    return;
}

pub unsafe fn unk_draw_op_1020_0000(param_1: *mut astruct_840) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut iVar4: *mut astruct_840;
    let mut uVar5: u16;
    let mut uVar4: u16;
    let mut local_c4: [u8; 0x6] = [0; 0x6];
    let mut local_be: *mut i16;
    let mut piStack184: *mut i16;
    let mut uStack182: u16;
    let mut local_b4: i16;
    let mut iStack178: i16;
    let mut aiStack176: [i16; 0x3c] = [0; 0x3c];
    let mut iStack56: *mut astruct_841;
    let mut iStack48: i16;
    let mut paStack46: *mut astruct_76;
    let mut local_2a: i16;
    let mut local_28: i16;
    let mut puStack38: *mut u32;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u32;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_841;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), iVar4.field4_0x4);
    uVar3 = iVar4.field19_0x14;
    puStack38 = (uVar3 + 0xa);
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
        CONCAT22(0x1050, &local_2a),
        CONCAT22(0x1050, &local_28),
    );
    uVar4 = 0x1008;
    pass1_1008_4480(
        puStack38,
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
        iVar4.field32_0x24,
    );
    paStack46 = null_mut();
    for iStack48 in 0..0x6 {
        uVar2 = iVar4.field19_0x14;
        uVar4 = 0x1010;
        pass1_1010_2b78(
            uVar2,
            (uVar2 >> 0x10),
            iStack48,
            CONCAT22(0x1050, &local_b4),
        );
        if (local_b4 == 0) {
            //   for (iStack56 = null_mut(); iVar3 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 1)
            iStack56 = null_mut();
            iVar3 = iStack56;
            while iStack56 <= iStack178 {
                piVar1 = aiStack176 + iStack56 * 0x3;
                uStack182 = 0x1050;
                piStack184 = piVar1;
                if (aiStack176[iStack56 * 0x3 + 0x2] != 0) {
                    paStack46 = pass1_1010_2b98(iVar4.field19_0x14, aiStack176[iStack56 * 0x3 + 0x2]);
                    pass1_1008_3e54(
                        CONCAT22(0x1050, &local_be),
                        0x0,
                        aiStack176[iVar3 * 0x3 + 0x1] + local_2a,
                        *piVar1 + local_28,
                    );
                    uVar4 = 0x1008;
                    pass1_1008_4480(puStack38, CONCAT22(0x1050, &local_be), paStack46);
                }
                iStack56 += 1;
            }
        } else {
            local_be = CONCAT22(0x1050, aiStack176 + iStack178 * 0x3);
            if (aiStack176[iStack178 * 0x3 + 0x2] != 0) {
                paStack46 = pass1_1010_2b98(iVar4.field19_0x14, aiStack176[iStack178 * 0x3 + 0x2]);
                pass1_1008_3e54(
                    CONCAT22(0x1050, local_c4),
                    0x0,
                    (local_be + 0x2) + local_2a,
                    *local_be + local_28,
                );
                uVar4 = 0x1008;
                pass1_1008_4480(puStack38, CONCAT22(0x1050, local_c4), paStack46);
            }
        }
    }
    ppcVar2 = (*puStack38 + 0x4);
    (**ppcVar2)(
        uVar4,
        puStack38,
        (puStack38 >> 0x10),
        0x0,
        0x0,
        &iVar4.field_0xa,
        uVar5,
    );
    EndPaint16(CONCAT22(0x1050, local_22), iVar4.field4_0x4);
    return;
}

pub unsafe fn mix_draw_op_1020_650c(param_1: *mut StructA) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let struct_1: *mut StructA;
    let mut uVar3: u16;
    let mut local_28: [u8; 0x20] = [0; 0x20];
    let mut iStack8: i16;
    let mut puStack6: *mut u32;

    uVar3 = (param_1 >> 0x10);
    struct_1 = param_1;
    uVar2 = &struct_1.field10_0x14;
    puStack6 = (uVar2 + 0xa);
    if ((struct_1.field8_0x10 != 0) || (uVar2 = &struct_1.field10_0x14, (uVar2 + 0x24) != 0)) {
        draw_c::draw_op_1020_9364(param_1);
        if (&struct_1.field19_0x24 != 0) {
            uVar2 = &struct_1.field19_0x24;
            ppcVar1 = (*&struct_1.field19_0x24 + 0x14);
            (**ppcVar1)(0x1020, uVar2, (uVar2 >> 0x10));
        }
    }
    iStack8 = 0;
    loop {
        if ((&struct_1.field12_0x18 + iStack8 * 0x2) != 0) {
            uVar2 = (&struct_1.field12_0x18 + iStack8 * 0x2);
            ppcVar1 = (*(&struct_1.field12_0x18 + iStack8 * 0x2) + 0x8);
            (**ppcVar1)(0x1020, uVar2, (uVar2 >> 0x10), puStack6, (puStack6 >> 0x10));
        }
        iStack8 += 0x1;
        if iStack8 >= 0x5 {
            break;
        }
    }
    BeginPaint16(CONCAT22(0x1050, local_28), struct_1.field2_0x4);
    ppcVar1 = (*puStack6 + 0x4);
    (**ppcVar1)(
        0x1538,
        puStack6,
        (puStack6 >> 0x10),
        0x0,
        0x0,
        &struct_1.field5_0xa,
        uVar3,
    );
    EndPaint16(CONCAT22(0x1050, local_28), struct_1.field2_0x4);
    return;
}

pub unsafe fn mix_draw_op_1020_9312(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x6);
    puVar1 = (uVar3 + 0xa);
    ppcVar2 = (*puVar1 + 0x4);
    (**ppcVar2)(
        0x1538,
        puVar1,
        (puVar1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | (iVar4 + 0xa),
    );
    EndPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    return;
}

pub unsafe fn get_dc_1018_4db0(param_1: *mut astruct_126, mut param_2: u16) {
    let mut HVar1: HDC16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_2;
    HVar1 = GetDC16(param_2);
    *(param_1 + 0x14) = HVar1;
    return;
}
