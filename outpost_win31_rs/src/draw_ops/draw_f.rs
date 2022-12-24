use crate::draw_ops::{draw_a, draw_d};
use crate::structs::struct_57::Struct57;
use crate::unk::block_1000_3000::sys_1000_3f9c;
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::utils::CONCAT22;
use crate::{draw_ops, gui};
use crate::winapi16::{DeleteObject16, InvalidateRect16, RealizePalette16, SelectPalette16};
use crate::windef16::{HDC16, HPALETTE16, HWND16, RECT16};

pub unsafe fn invalidate_rect_1018_5d32(mut param_1: u32, mut param_2: i16) {
    let mut hwnd: HWND16;

    hwnd = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    InvalidateRect16(0x0, (param_1 + 0x22), hwnd);
    return;
}


pub unsafe fn invalidate_rect_1040_3ddc(in_struct_1: *mut StructC)

{
  let mut rect: RECT16;
  let mut uStack6: u32;

  rect = 0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(0x0,&rect,0x1050);
  return;
}


pub unsafe fn invalidate_rect_1020_8d90(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_48: RECT16;
    let mut iStack68: i16;
    let mut iStack66: i16;
    let mut local_40: i16;
    let mut local_3e: i16;
    let mut uStack60: u32;
    let mut local_38: astruct_288 = astruct_288::default();
    let mut local_10: [u8; 0xa] = [0; 0xa];
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    uVar2 = pass1_1018_266a((iVar4 + 0x22));
    if (uVar2 != 0) {
        uVar6 = pass1_1018_265c();
        uStack4 = (uVar6 >> 0x10);
        uStack6 = uVar6;
        uVar3 = CONCAT22(in_register_0000000a, uStack4 | uStack6);
        if ((uStack4 | uStack6) != 0) {
            sys_1000_3f9c(CONCAT22(0x1050, local_10), s__03ld_1050_442a, uStack6);
            uVar1 = (iVar4 + 0x22);
            draw_a::file_and_draw_op_1008_4f20(
                uVar3,
                CONCAT22(0x1050, &local_38),
                (uVar1 + 0xe),
                0x25,
                CONCAT22(0x1050, local_10),
                param_6,
                param_7,
            );
            pass1_1008_4480(
                param_5,
                (param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                CONCAT22(0x1050, &local_38),
            );
            uStack60 = pass1_1008_4772(CONCAT22(0x1050, &local_38));
            pass1_1008_3e94(
                (param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                CONCAT22(0x1050, &local_40),
                CONCAT22(0x1050, &local_3e),
            );
            local_48.x = local_3e;
            local_48.y = local_40;
            uVar5 = (uStack60 >> 0x10);
            iStack68 = local_3e + (uStack60 + 0x4);
            iStack66 = local_40 + (uStack60 + 0x8);
            // just 0x0
            InvalidateRect16(0x0, &local_48, 0x1050);
            pass1_1008_41bc(CONCAT22(0x1050, &local_38));
        }
    }
    return;
}


pub unsafe fn get_dc_op_1040_3d5e(param_1: *mut astruct_1) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_1;
    let mut iVar3: *mut astruct_934;
    let mut uVar3: u16;
    let mut puStack8: *mut u32;
    let mut local_4: HDC16;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    local_4 = GetDC16(iVar4.field6_0x6);
    uVar2 = FUN_1010_830a(local_4, in_EDX, 0x1538, _u16_1050_14cc, iVar4.field163_0xa4);
    puStack8 = CONCAT22(in_EDX, uVar2);
    iVar3 = *puStack8;
    ppcVar1 = &iVar3.field6_0x8;
    (**ppcVar1)(0x1010, uVar2, in_EDX, &local_4, 0x1050);
    ppcVar1 = &iVar3.field4_0x4;
    (**ppcVar1)(0x1010, puStack8, 0x50078, &local_4, 0x1050);
    ppcVar1 = &iVar3.field8_0xc;
    (**ppcVar1)(0x1010, puStack8, &local_4, 0x1050);
    ReleaseDC16(local_4, iVar4.field6_0x6);
    return 0x0;
}


pub unsafe fn call_fn_ptr_1038_9ffa(mut param_1: u32, pstruct_param_2: *mut astruct_733, mut param_3: u16 ) -> u16

{
  let mut ppcVar1: *mut *mut code;
   let mut struct_3: *mut astruct_43;
   let mut struct_2: *mut astruct_43;
  let mut puStack8: *mut u32;
  let mut hdc: HDC16;
  let mut var_5: u16;

  hdc = GetDC16(pstruct_param_2.hwnd_0x6);
  struct_2 = FUN_1010_830a(hdc,param_1,0x1538,_u16_1050_14cc,0x3);
  puStack8 = CONCAT22(param_1,struct_2);
  struct_3 = *puStack8;
  ppcVar1 = &struct_3.fn_ptr_field_0x8;
  (**ppcVar1)(0x1010,struct_2,param_1,&hdc,0x1050);
  ppcVar1 = &struct_3.fn_ptr_field_0x4;
  (**ppcVar1)(0x1010,puStack8,0x50005,&hdc,0x1050);
  ppcVar1 = &struct_3.fn_ptr_field_0xc;
  (**ppcVar1)(0x1010,puStack8,&hdc,0x1050);
  ReleaseDC16(hdc,pstruct_param_2.hwnd_0x6);
  return 0x0;
}

pub unsafe fn get_sys_metrics_1040_8c66(param_1: *mut Struct37)

{
  let mut piVar1: *mut i16;
  let mut bVar2: u8;
  let mut HVar3: HDC16;
  let mut IVar4: i16;
   let mut struct_1: *mut Struct37;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  struct_1 = param_1;
  HVar3 = GetDC16((&struct_1.field1_0x4 + 0x2));
  draw_d::draw_text_1040_8d14(param_1, HVar3);
  struct_1[0x1].field1_0x4 = *&struct_1.field144_0x9e;
  &struct_1[0x1].field_0x8 = (struct_1 + 1).field0_0x0;
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + IVar4;
  bVar2 = struct_1.field138_0x98 & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar4 = GetSystemMetrics16(SM_CYICON);
    if (&struct_1[0x1].field_0xa < IVar4) {
      IVar4 = GetSystemMetrics16(SM_CYICON);
      struct_1[0x1].field_0xa = IVar4;
    }
  }
  piVar1 = &struct_1[0x1].field_0x8;
  *piVar1 = *piVar1 + 0x14;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0xa;
  struct_1[0x1].field_0xe = &struct_1[0x1].field_0xa;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0x30;
  HVar3 = *(&struct_1.field1_0x4 + 2);
  ReleaseDC16(HVar3,HVar3);
  return;
}


pub unsafe fn set_struct_op_1020_921c(
    mut param_1: u16,
    pstructa_param_2: *mut StructA,
    mut param_3: u16,
    param_4: *mut *mut u8,
) {
    let mut uVar1: u16;
    let mut HVar2: HDC16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let iVar3: *mut StructA;
    let uVar3: *mut StructA;
    let mut pUVar3: *mut u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe2: u32;
    let mut uVar2: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    uVar3 = (pstructa_param_2 >> 0x10);
    iVar3 = pstructa_param_2;
    pstructa_param_2.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    pstructa_param_2.field0_0x0 = 0x3aa8;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field2_0x4 = param_3;
    pstructa_param_2.field0_0x0 = 0x3ab0;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field3_0x6 = 0;
    iVar3.field5_0xa = 0;
    iVar3.field6_0xc = 0;
    iVar3.field7_0xe = 0;
    iVar3.field8_0x10 = 0;
    iVar3.field9_0x12 = 0;
    pstructa_param_2.field0_0x0 = 0x96c8;
    iVar3.field1_0x2 = 0x1020;
    HVar2 = GetDC16(iVar3.field2_0x4);
    iVar3.field5_0xa = HVar2;
    param_4 = CONCAT22(param_4, 0x48);
    pUVar3 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        param_4,
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar1 = (pUVar3 >> 0x10);
    iVar3.field6_0xc = (pUVar3 + 0xa);
    iVar3.field7_0xe = (pUVar3 + 0xc);
    return;
}

pub unsafe fn unk_win_ui_op_1020_717e(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_40,
    param_4: *mut StructA,
) {
    let mut paVar1: *mut astruct_13;
    let mut ppcVar2: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut HVar6: HPALETTE16;
    let mut puVar6: *mut u32;
    let mut uVar6: u16;
    let mut uVar9: u16;
    let mut puVar7: *mut u8;
    let mut puVar10: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut iVar7: *mut astruct_40;
    let mut iVar8: *mut astruct_933;
    let mut uVar7: *mut astruct_40;
    let mut uVar8: u16;
    let mut puVar12: *mut u32;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_4: HDC16;
    let mut uVar3: u32;
    let mut iVar9: *mut astruct_41;
    let mut in_stack_0000ffdc: u16;

    paVar11 = CONCAT22(in_register_0000000a, param_1);
    gui::get_sys_metrics_1020_7c1a(param_3, param_4);
    uVar7 = (param_3 >> 0x10);
    iVar7 = param_3;
    iVar7.field17_0x14 = 0;
    iVar7.field20_0x18 = param_4;
    iVar7.field21_0x1c = 0;
    (&iVar7[0x1].field0_0x0 + 1) = 0;
    param_3.field0_0x0 = 0x754c;
    iVar7.field1_0x2 = 0x1020;
    puVar12 = mixed_1010_20ba(
        paVar11,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x4),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    uVar5 = (paVar11 >> 0x10);
    iVar7.field21_0x1c = puVar12;
    uVar9 = (puVar12 >> 0x10);
    iVar7.field_0x1e = uVar9;
    ppcVar2 = (*&iVar7.field21_0x1c + 0x4);
    (**ppcVar2)(0x1010, iVar7.field21_0x1c, uVar9, 0x0, param_3);
    local_4 = GetDC16(iVar7.hwnd_0x4);
    uVar3 = &iVar7.field21_0x1c;
    *(uVar3 + 0x178) = local_4;
    uVar4 = &iVar7.field21_0x1c;
    uVar8 = (uVar4 >> 0x10);
    iVar8 = uVar4;
    puVar6 = iVar8.field36_0x24;
    uVar9 = (&iVar8.field36_0x24 + 2);
    paVar11 = CONCAT22(uVar5, uVar9);
    uVar5 = SUB42(puVar6, 0x0);
    ppcVar2 = (*puVar6 + 0x14);
    (**ppcVar2)(0x38, uVar5, uVar9);
    puVar12 = mixed_1010_20ba(
        paVar11,
        _u16_1050_0ed0,
        CONCAT22(uVar5, 0x29),
        in_stack_0000fe78,
        in_stack_0000ff9c,
        in_stack_0000ffa2,
        in_stack_0000ffa6,
    );
    puVar10 = (puVar12 >> 0x10);
    paVar1 = (puVar12 + 0xe);
    pass1_1008_4d84(puVar10, (puVar6 & 0xffff | paVar11 << 0x10), paVar1);
    HVar6 = draw_a::palette_op_1008_4e08(
        &local_4,
        puVar10,
        paVar1,
        CONCAT13(0x10, CONCAT12(0x50, &local_4)),
    );
    (&iVar7[0x1].field0_0x0 + 1) = HVar6;
    return;
}


pub unsafe fn win_ui_palette_op_1020_0cd2(struct_param_1: *mut astruct_775) {
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar7: u16;
    let mut hdc: HDC16;
    let mut hpal: HDC16;
    let mut hpal_00: HPALETTE16;
    let mut UVar4: u16;
    let mut extraout_DX: u16;
    let mut uVar8: u16;
    let mut struct_1: *mut astruct_775;
    let mut iVar5: *mut astruct_774;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut hdc_00: HDC16;
    let mut paStack10: *mut astruct_13;
    let mut uStack6: u16;
    let mut puVar1: *mut u32;
    let mut uVar4: u32;

    uVar5 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    uVar4 = struct_1.field5_0x6;
    uVar6 = (uVar4 >> 0x10);
    iVar5 = uVar4;
    puVar2 = &iVar5.field_0xa;
    uStack6 = puVar2;
    uVar7 = iVar5.field12_0xc | uStack6;
    if (uVar7 != 0) {
        ppcVar3 = (*puVar2 + 0x14);
        (**ppcVar3)();
        paStack10 = CONCAT22(extraout_DX, uVar7);
        uVar8 = extraout_DX | uVar7;
        if (uVar8 != 0) {
            hdc = GetDC16(struct_1.field4_0x4);
            hpal = hdc;
            hdc_00 = hdc;
            draw_ops::create_palette_1008_4e38(paStack10, uVar8);
            hpal_00 = SelectPalette16(0x0, hpal, hdc_00);
            UVar4 = RealizePalette16(hdc);
            SelectPalette16(0x1, hpal_00, hdc);
            DeleteObject16(hpal);
            if (0x0 < UVar4) {
                InvalidateRect16(0x1, NULL, 0x0);
            }
            ReleaseDC16(hdc, struct_1.field4_0x4);
            return;
        }
    }
    return;
}
