use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::unk::block_1000_3000::{pass1_1000_3cea, str_op_1000_3da4, sys_1000_3f9c, unk_str_op_1000_3d3e};
use crate::unk::block_1000_4000::str_1000_4d58;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1030_5000::pass1_1030_532e;
use crate::unk::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8308, pass1_1030_8326, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e, pass1_1030_83ba};
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::{file_ops, gui, winapp};
use crate::no_refs::f::pass1_1010_40cc;
use crate::no_refs::i::struct_1010_5f1e;
use crate::resources::load_string_1010_847e;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_op_1000_1708, mem_op_1000_160a, mem_op_1000_179c};
use crate::unk::block_1008_1000::big_switch_1008_15d4;
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e0e, pass1_1008_3e94};
use crate::unk::block_1008_a000::pass1_1008_a9ec;
use crate::unk::block_1010_0000::{pass1_1010_0886, pass1_1010_08e2};
use crate::unk::block_1018_0000::pass1_1018_017c;
use crate::unk::block_1018_2000::pass1_1018_262e;
use crate::unk::block_1018_5000::pass1_1018_57e6;
use crate::unk::block_1020_6000::pass1_1020_68de;
use crate::gui::{dialog, menu, window};
use crate::gui::window::{win_a, win_e};
use crate::winapi16::{LoadCursor16, MapDialogRect16, WinAPI16_MessageBox16, PostMessage16, ReleaseCapture16, SetCapture16, SetCursor16, SetWindowText16, ShowWindow16, UpdateWindow16};
use crate::winapp::winapp_c::send_msg_1020_097e;
use crate::winapp::winapp_e;
use crate::windef16::{HCURSOR16, WNDCLASS16};

pub fn cursor_op_1008_2dcc(mut param_1: u16, param_2: *mut Struct20, mut param_3: u16, mut param_4: u16, mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut cursor_handle: HCURSOR16;
  let mut hcursor: HCURSOR16;
  let mut extraout_DX: u16;
  let mut paVar3: *mut Struct20;

  cursor_handle = LoadCursor16(0x7f02,0x0);
  hcursor = SetCursor16(cursor_handle);
  paVar3 = param_2;
  cursor_handle = hcursor;
  if ((&(param_2)[0x1].field2_0x4 + 0x2) != 0) {
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    paVar3 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = paVar3.field_0x90;
    (**ppcVar2)(0x1538,uVar1,(uVar1 >> 0x10));
    param_1 = extraout_DX;
  }
  big_switch_1008_15d4(paVar3,param_2,CONCAT22(param_5,param_3));
  (&(param_2)[0x1].field2_0x4 + 0x2) = cursor_handle;
  (param_2)[0x1].field3_0x8 = param_1;
  uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
  if ((uVar1 + 0xe0) == 0) {
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = ((&(param_2)[0x1].field2_0x4 + 0x2) + 0x8);
    (**ppcVar2)(0x1538,uVar1,(uVar1 >> 0x10));
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = ((&(param_2)[0x1].field2_0x4 + 0x2) + 0xc);
    (**ppcVar2)(0x1538,uVar1,(uVar1 >> 0x10),0x3);
    (param_2).field153_0xce = (&(param_2)[0x1].field2_0x4 + 2);
  }
  else {
    (&(param_2)[0x1].field2_0x4 + 0x2) = 0;
    win_a::ui_op_1008_2c4e(param_1, param_4, param_2, param_3);
    (param_2).field153_0xce = 0;
  }
  SetCursor16(hcursor);
  return;
}


pub fn win_ui_cursor_op_1008_2e9a
               (param_1: *mut Struct57,param_2: *mut Struct72,param_3: *mut WNDCLASS16 ,mut param_4: u16 ,mut param_5: u16 ,
               mut param_6: u16 )

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut HVar6: HCURSOR16;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut unaff_SI: u16;
  let mut uVar7: *mut Struct72;
  let mut in_stack_0000fc78: u16;
  let mut in_stack_0000fd9c: u16;
  let mut in_stack_0000fda2: u16;
  let mut in_stack_0000fda6: u16;
  let mut local_224: [u8;0x108] = [0;0x108];
  let mut uStack266: u16;
  let mut uStack262: u32;
  let mut local_102: [u8;0x100] = [0;0x100];

  local_102[0] = '\0';
  uStack262 =
              mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fc78,
                              in_stack_0000fd9c,in_stack_0000fda2,in_stack_0000fda6);
  uVar3 = (uStack262 >> 0x10);
  iVar4 = uStack262;
  pcVar1 = *(iVar4 + 0x16);
  uVar5 = (iVar4 + 0x18);
  uVar9 = param_1 & 0xffff0000 | uVar5;
  uStack266 = pcVar1;
  uStack266 = uVar5 | uStack266;
  if uStack266 == 0 {
    file_ops::save_file_1008_3178(uVar5, param_2, 1);
    uVar5 = uVar9;
    uVar8 = uVar5 | uStack266;
    uVar9 = uVar9 & 0xffff0000 | uVar8;
    if uVar8 == 0 {
      PostMessage16(0x0,0x13d,0x111,HWND16_1050_0396);
      return;
    }
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),CONCAT22(uVar5,uStack266));
    str_1000_4d58(CONCAT22(0x1050,local_102),NULL,0x0,CONCAT22(0x1050,local_224),
                  CONCAT22(0x1050,&param_3));
    if (param_3 != '\0') {
        let cea = pass1_1000_3cea(CONCAT22(0x1050, local_224), CONCAT22(0x1050, &param_3));
    }
    struct_1010_5f1e(uVar9,uStack262,CONCAT22(0x1050,local_224));
  }
  else {
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),*(iVar4 + 0x1a));
    uVar5 = str_op_1000_3da4(CONCAT22(0x1050,local_102));
    if (local_102[uVar5 - 0x1] != '\\') {
      local_102[uVar5] = '\\';
      local_102[uVar5 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(0x1050,local_102),pcVar1);
  }
  if (local_102[0] != '\0') {
    uVar7 = (param_2 >> 0x10);
    send_msg_1020_097e((param_2 + 0xe8));
    uVar2 = (param_2 + 0xe8);
    UpdateWindow16((uVar2 + 0x8));
    HVar6 = LoadCursor16(0x7f02,0x0);
    param_3 = (param_3 & 0xffff0000 | HVar6);
    HVar6 = SetCursor16(HVar6);
    param_3 = CONCAT22(0x1050,local_102);
    winapp_e::win_ui_op_1008_1414(uVar9, param_2, param_3, param_6, param_5, param_4);
    param_3 = CONCAT22(HVar6,0x1538);
    SetCursor16(HVar6);
  }
  return;
}


pub fn win_ui_cursor_op_1038_bc30(param_1: *mut StructC)

{
  let mut uVar1: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  let mut HStack6: HCURSOR16;
  let mut HStack4: HCURSOR16;

  HStack4 = LoadCursor16(0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  uVar1 = (param_1 + 0x8e);
  pass1_1030_532e(CONCAT22(0x1050,&local_112),(uVar1 + 0xe) + 0x1000000);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_112));
  pass1_1030_838e(_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  SetCursor16(HStack6);
  return;
}


pub fn win_ui_cursor_op_1008_06c0(
    param_1: u32,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: u16;
    let mut in_EDX: u32;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_AF: u8;
    let mut pcVar6: *mut c_char;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe44: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6e: u16;
    let mut in_stack_0000ff72: u16;
    let mut iVar8: i16;
    let mut in_stack_0000ff9c: u16;
    let mut local_5a: [u8; 0x50] = [0; 0x50];
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut HStack6: HCURSOR16;
    let mut HStack4: HCURSOR16;

    if (param_4 == 0x400) {
        pass1_1030_8344(_u16_1050_5748, 0x4000001);
        in_AX = in_EDX | in_AX;
        paVar2 = (in_EDX & 0xffff0000 | in_AX);
        if (in_AX != 0) {
            iVar4 = param_1;
            uVar5 = (param_1 >> 0x10);
            if (PTR_LOOP_1050_4fe8.is_null() == false) {
                pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
                WinAPI16_MessageBox16(
                    0x10,
                    pcVar6,
                    s_You_may_not_run_a_turn__The_game_1050_00df,
                    (iVar4 + 0x8),
                );
                return;
            }
            HStack4 = LoadCursor16(0x7f02, 0x0);
            HStack6 = SetCursor16(HStack4);
            pass1_1030_83ba(in_AF, _u16_1050_5748, param_2);
            (_u16_1050_5748 + 0x8) = 0x1;
            puVar7 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000ff9c, 0x29),
                in_stack_0000fe44,
                in_stack_0000ff68,
                in_stack_0000ff6e,
                in_stack_0000ff72,
            );
            uVar3 = (paVar2 >> 0x10);
            uStack10 = SUB42(puVar7, 0x0);
            uStack8 = (puVar7 >> 0x10);
            pass1_1018_262e(puVar7);
            pass1_1030_8326();
            pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x5dc);
            paVar2 = CONCAT22(uVar3, (pcVar6 >> 0x10));
            sys_1000_3f9c(
                CONCAT13(0x10, CONCAT12(0x50, local_5a)),
                s__s__ld_1050_0109,
                pcVar6,
            );
            ppcVar1 = (*param_1 + 0x14);
            iVar8 = iVar4;
            (**ppcVar1)(
                0x1000,
                iVar4,
                (param_1 >> 0x10),
                0x0,
                local_5a,
                0x1050,
            );
            puVar7 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(iVar8, 0x37),
                in_stack_0000fe3a,
                in_stack_0000ff5e,
                in_stack_0000ff64,
                in_stack_0000ff68,
            );
            pass1_1008_a9ec(puVar7);
            SetCursor16(HStack6);
            PostMessage16(0x0, 0xfc, 0x111, (iVar4 + 0x8));
        }
    }
    return;
}


pub fn pass1_1020_6184(mut param_1: u32, mut param_2: u16) {
    let mut HVar1: HCURSOR16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xf4) == 1) {
        HVar1 = SetCursor16((iVar2 + 0xf0));
        (iVar2 + 0xee) = HVar1;
        (iVar2 + 0x10c) = param_2;
        SetCapture16((iVar2 + 0x8));
        (iVar2 + 0xf4) = 0x2;
    }
    return;
}


pub fn set_cursor_1020_5764(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut HVar3: HCURSOR16;
    let mut in_EDX: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_e: i16;
    let mut local_c: [u8; 0x2] = [0; 0x2];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe6, 0x2f),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar5 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x20);
    uVar1 = (puStack6 + 0x22);
    if ((uVar1 | uStack10) != 0) {
        pass1_1030_8308(
            &local_e,
            uVar1,
            _u16_1050_5748,
            (_u16_1050_5748 >> 0x10),
            CONCAT22(0x1050, &local_e),
            CONCAT22(0x1050, local_c),
            CONCAT13((uVar1 >> 0x8), CONCAT12(uVar1, uStack10)),
        );
        if (param_2 <= local_e) {
            uVar5 = (param_1 >> 0x10);
            iVar4 = param_1;
            if ((iVar4 + 0xf4) != 1) {
                SetCursor16((iVar4 + 0xee));
                (iVar4 + 0xee) = 0;
                (iVar4 + 0xf4) = 0x1;
                (iVar4 + 0x10c) = 0;
                ReleaseCapture16();
            }
            HVar3 = LoadCursor16(0x7f02, 0x0);
            HVar3 = SetCursor16(HVar3);
            pass1_1018_017c(puStack6, param_2);
            uVar2 = (iVar4 + 0xf6);
            (uVar2 + 0x10) = 0x1;
            if ((iVar4 + 0xfe) != 0) {
                pass1_1020_68de((iVar4 + 0xfe));
                uVar2 = (iVar4 + 0xfe);
                PostMessage16(0x0, 0xeb, 0x111, (uVar2 + 0x8));
            }
            SetCursor16(HVar3);
        }
    }
    return;
}

pub fn win_ui_cursor_op_1020_522e(
    mut param_1: u16,
    param_2: *mut astruct_52,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar4: *mut astruct_52;
    let mut uVar4: *mut astruct_52;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut uVar6: u8;
    let mut uVar7: u8;
    let mut uVar8: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    iVar1 = iVar4.field242_0xf4;
    if (iVar1 == 0x2) {
        SetCursor16(iVar4.field237_0xee);
        iVar4.field237_0xee = 0;
        iVar4.field242_0xf4 = 0x1;
        (iVar4 + 1) = 0;
        ReleaseCapture16();
        return;
    }
    if (iVar1 == 0x3) {
        SetCursor16(iVar4.field237_0xee);
        iVar4.field237_0xee = 0;
        iVar4.field242_0xf4 = 0x1;
        (iVar4 + 1) = 0;
        ReleaseCapture16();
        uVar6 = 0;
        uVar7 = 0;
        uVar8 = 0;
        puVar5 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            0x47,
            in_stack_0000fea0,
            in_stack_0000ffc4,
            in_stack_0000ffca,
            in_stack_0000ffce,
        );
        pass1_1018_57e6(
            puVar5,
            CONCAT22(uVar8, CONCAT11(uVar7, uVar6)),
            puVar5,
            (puVar5 >> 0x10),
        );
        return;
    }
    BVar3 = menu::menu_ui_op_1020_5bf2(param_2, param_3, param_4);
    if (BVar3 == 0) {
        ppcVar2 = (*&iVar4.field_0x4 + 0x60);
        (**ppcVar2)();
    }
    return;
}


pub fn FUN_1038_d8ae(param_1: *mut StructD, mut param_2: u16, struct_b_param_2: *mut StructB, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut puVar3: *mut u32;
    let mut rect: *mut Struct57;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut struct_b_1: *mut StructB;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar9: u16;
    let mut uVar8: u16;
    let mut in_stack_0000fe24: u16;
    let mut in_stack_0000fe28: u16;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000ff4e: u16;
    let mut in_stack_0000ff52: u16;
    let mut in_stack_0000ff56: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa6: u16;
    let mut local_26: u16;
    let mut uStack36: u16;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut puStack30: *mut u16;
    let mut puStack14: *mut u32;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut HStack6: HCURSOR16;
    let mut HStack4: HCURSOR16;

    HStack4 = LoadCursor16(0x7f02, 0x0);
    HStack6 = SetCursor16(HStack4);
    dialog::dialog_ui_fn_1040_78e2(struct_b_param_2);
    uVar7 = (struct_b_param_2 >> 0x10);
    struct_b_1 = struct_b_param_2;
    uStack8 = pass1_1010_0886();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_1);
    } else {
        param_1 = (param_1 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack30 = CONCAT22(param_1, PTR_LOOP_1050_5f2c);
    puVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, param_1);
    struct_b_1[0x7].field5_0xa = puVar3;
    struct_b_1[0x7].field6_0xc = param_1;
//   for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1)
    for iStack10 in 1..uStack8 {
        uVar2 = &struct_b_1[0x7].lpvoid_field_0x8;
        puStack30 = pass1_1010_08e2(uVar2, (uVar2 >> 0x10), iStack10);
        paVar5 = (param_1 & 0xffff0000 | puStack30 >> 0x10);
        local_26 = *puStack30;
        uStack36 = (puStack30 + 2);
        uStack34 = 0x1;
        uStack32 = 0x1;
        rect = &local_26;
        MapDialogRect16(rect, 0x1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | rect;
        param_1 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar2 = &struct_b_1[0x7].field5_0xa;
            (uVar2 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_1.lpvoid_field_0x8;
            pass1_1008_3bd6(param_1, rect, paVar5, 0x0, CONCAT22(local_26, uStack36), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack30 + 0x4))), param_4, in_stack_0000fe24, in_stack_0000fe28, in_stack_0000ff4e, in_stack_0000ff52, in_stack_0000ff56,
            );
            uVar2 = &struct_b_1[0x7].field5_0xa;
            uVar8 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            (iVar6 + iStack10 * 0x4) = rect;
            (iVar6 + iStack10 * 0x4 + 0x2) = param_1;
        }
        uVar2 = &struct_b_1[0x7].field5_0xa;
        uVar9 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        if ((iVar6 + iStack10 * 0x4) != 0) {
            uVar2 = (iVar6 + iStack10 * 0x4);
            (uVar2 + 0x3e) = 0x1;
            uVar2 = &struct_b_1[0x7].field5_0xa;
            win_e::enable_win_1040_9234((uVar2 + iStack10 * 0x4), (puStack30 + 0x6));
        }
    }
    puStack14 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_5, 0x2), in_stack_0000fe78, in_stack_0000ff9c, in_stack_0000ffa2, in_stack_0000ffa6);
    SetWindowText16((puStack14 + 0x68), struct_b_1.lpvoid_field_0x8);
    ShowWindow16(0x5, struct_b_1.lpvoid_field_0x8);
    SetCursor16(HStack6);
    return;
}

pub fn win_ui_cursor_op_1020_1294(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: i16,
    mut param_4: i16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut in_AX: u16;
    let mut hcursor: HCURSOR16;
    let mut hcursor_00: HCURSOR16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffc2: u16;
    let mut local_12: i16;
    let mut local_10: i16;
    let mut puStack14: *mut u16;
    let mut puStack10: *mut u32;
    let mut local_6: i16;
    let mut iStack4: i16;

    pass1_1030_8344(_u16_1050_5748, 0x4000001);
    if ((param_1 | in_AX) == 0) {
        local_6 = param_4;
        iStack4 = param_3;
        uVar5 = (param_2 >> 0x10);
        iVar4 = param_2;
        puStack10 = pass1_1010_40cc(param_3, 0x0, (iVar4 + 0xf2));
        uVar3 = CONCAT22(in_register_0000000a, (puStack10 >> 0x10));
        uVar1 = (iVar4 + 0xf2);
        puStack14 = (uVar1 & 0xffff0000 | (uVar1 + 0x76));
        pass1_1008_3e94(
            puStack14,
            CONCAT22(0x1050, &local_12),
            CONCAT22(0x1050, &local_10),
        );
        local_6 -= local_10;
        iStack4 -= local_12;
        hcursor = winapp_e::pt_in_rect_1010_40f8(,
                                                 uVar3,
                                                 (iVar4 + 0xf2),
                                                 CONCAT22(0x1050, &local_6),
                                                 in_stack_0000ffc2,
        );
        if (hcursor != 0xffff) {
            hcursor_00 = LoadCursor16(0x7f02, 0x0);
            SetCursor16(hcursor_00);
            ppcVar2 = (*puStack10 + 0x4);
            (**ppcVar2)();
            pass1_1008_3e0e(param_2);
            SetCursor16(hcursor);
        }
    }
    return;
}
