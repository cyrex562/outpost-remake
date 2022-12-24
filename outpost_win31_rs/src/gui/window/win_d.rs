use std::os::raw::c_char;
use std::ffi::c_void;
use crate::globals::PTR_LOOP_1050_1040;
use crate::no_refs::h::pass1_1010_4f30;
use crate::no_refs::i::pass1_1010_6006;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708, mem_op_1000_160a, mem_op_1000_179c};
use crate::unk::block_1000_3000::{pass1_1000_3e2c, str_op_1000_3da4, sys_1000_3f9c, unk_str_op_1000_3d3e};
use crate::unk::block_1000_4000::pass1_1000_4906;
use crate::unk::block_1008_3000::pass1_1008_3bd6;
use crate::unk::block_1008_5000::win_1008_5c7c;
use crate::unk::block_1008_9000::pass1_1008_941a;
use crate::unk::block_1008_b000::{load_string_1008_b65a, pass1_1008_b820};
use crate::unk::block_1008_e000::{pass1_1008_eb5c, pass1_1008_eb6e};
use crate::unk::block_1010_0000::{pass1_1010_0892, pass1_1010_0932};
use crate::unk::block_1010_2000::{mixed_1010_20ba, unk_load_str_op_1010_2c34};
use crate::unk::block_1018_1000::{pass1_1018_1c9a, pass1_1018_1e78};
use crate::unk::block_1018_3000::{pass1_1018_3a7a, pass1_1018_3a94, pass1_1018_3d44, sprintf_op_1018_34b6, unk_str_op_1018_35b0};
use crate::unk::block_1020_1000::pass1_1020_1d8e;
use crate::unk::block_1040_8000::string_1040_8520;
use crate::unk::block_1040_b000::pass1_1040_b54a;
use crate::utils::{CONCAT22, SUB42};
use crate::gui::{cleanup, dialog, window};
use crate::gui::window::win_e;
use crate::winapi16::{CheckRadioButton16, DestroyWindow16, EnableWindow16, GetDlgItem16, GetWindowText16, GetWindowWord16, IsDlgButtonChecked, MapDialogRect16, PostMessage16, SendDlgItemMessage16, SendMessage16, SetDlgItemInt16, SetDlgItemText16, SetFocus16, SetWindowPos16, SetWindowText16, ShowWindow16};
use crate::winapp::winapp_c::send_msg_1040_3374;
use crate::winapp::winapp_b::{post_win_msg_1040_7b3c, unk_win_msg_op_1008_9510};
use crate::windef16::{BOOL16, HWND16, LRESULT};

pub unsafe fn show_win_1008_96ae(mut param_1: u32, param_2: INT16) -> BOOL16 {
    let mut BVar1: bool;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        BVar1 = ShowWindow16(param_2, (param_1 + 0x8));
        return BVar1;
    }
    return 0x0;
}

pub unsafe fn show_win_1038_9fd0(param_1: *mut StructB)

{
  dialog::dialog_ui_fn_1040_78e2(param_1);
  window::move_win_1040_826c(param_1, -0x1, 0xffff);
  ShowWindow16(0x5,(param_1 + 0x6));
  return;
}


pub unsafe fn win_ui_op_1038_a972(struct_b_param_1: *mut StructB)

{
  let mut hwnd: HWND16;
  let mut BVar1: bool;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
   let mut struct_b_3: *mut StructB;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut in_stack_0000ffaa: u16;

  uVar3 = (in_EDX >> 0x10);
  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar4 = (struct_b_param_1 >> 0x10);
  struct_b_3 = struct_b_param_1;
  SendDlgItemMessage16(0x0,0x1,0x401,0x116,struct_b_3.lpvoid_field_0x8);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0x401,0x11a,struct_b_3.lpvoid_field_0x8);
  uVar2 = CONCAT22(uVar3,(LVar5 >> 0x10));
  hwnd = GetDlgItem16(0x11a,struct_b_3.lpvoid_field_0x8);
  BVar1 = EnableWindow16(0x0,hwnd);
  win_1008_5c7c(BVar1,uVar2,_u16_1050_02a0,0x40001);
  (struct_b_3 + 0x7).field0_0x0 = BVar1;
  window::unk_win_ui_op_1038_a18c(uVar2, struct_b_param_1, in_stack_0000ffaa);
  ShowWindow16(0x5,struct_b_3.lpvoid_field_0x8);
  return;
}


pub unsafe fn show_win_1038_cb5c(mut param_1: u32, struct_b_param_1: *mut StructB, mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
   let mut struct_b_5: *mut StructB;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut in_stack_0000fe48: u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut iStack10: i16;

  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar8 = (struct_b_param_1 >> 0x10);
  struct_b_5 = struct_b_param_1;
  uVar3 = pass1_1008_eb6e();
  for iStack10 in 0 .. uVar3 {
    uVar1 = &struct_b_5[0x7].field1_0x2;
    puVar9 = pass1_1008_eb5c(uVar1,(uVar1 >> 0x10),iStack10);
    paVar7 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    paVar4 = puVar9;
    uVar2 = (puVar9 >> 0x10);
    paVar5 = paVar4;
    mem_op_1000_179c(0x42,paVar7);
    uVar6 = paVar7 | paVar5;
    param_1 = paVar7 & 0xffff0000 | uVar6;
    if (uVar6 != 0) {
      pass1_1008_3bd6(param_1,paVar5,paVar7,0x0,CONCAT22(*puVar9,paVar4.field1_0x2),0x101,0xff0100,
                      CONCAT22(struct_b_5.lpvoid_field_0x8,paVar4.field2_0x4),param_3,in_stack_0000fe48,
                      in_stack_0000fe4c,in_stack_0000ff72,in_stack_0000ff76,in_stack_0000ff7a);
    }
  }
  win_1008_5c7c(uVar3,param_1,_u16_1050_02a0,0x90001);
  ShowWindow16(0x5,struct_b_5.lpvoid_field_0x8);
  return;
}

pub unsafe fn check_radio_btn_show_win_1038_e19a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog::dialog_ui_fn_1040_78e2(param_1);
    uVar1 = (param_1 >> 0x10);
    CheckRadioButton16(0x1807, 0x1807, 0x1807, (param_1 + 0x6));
    window::move_win_1040_826c(param_1, 0xc8, 0xc8);
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}

pub unsafe fn unk_win_ui_op_1038_e71c(mut param_1: u16, param_2: *mut StructB)

{
    let mut extraout_DX: u16;
    let mut struct_1: *mut StructB;
    let mut struct_1_lo: u16;
    let mut pcStack6: *mut c_char;

    dialog::dialog_ui_fn_1040_78e2(param_2);
    struct_1_lo = (param_2 >> 0x10);
    struct_1 = param_2;
    unk_load_str_op_1010_2c34(&struct_1[0x7].field1_0x2);
    pcStack6 = CONCAT22(extraout_DX, param_1);
    unk_str_op_1000_3d3e((param_2 & 0xffff0000 | ZEXT24(&struct_1.field8_0x10)),
                         CONCAT22(extraout_DX, param_1));
    fn_ptr_1000_17ce(pcStack6);
    window::move_win_1040_826c(param_2, -0x1, 0xffff);
    ShowWindow16(0x5, struct_1.lpvoid_field_0x8);
    struct_1[0x7].lpvoid_field_0x8 = (&PTR_LOOP_1050_0000 + 1);
    unk_win_msg_op_1008_9510((param_2 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
    DestroyWindow16(struct_1.lpvoid_field_0x8);
    return;
}


pub unsafe fn FUN_1038_ec16(mut param_1: u16, param_2: *mut StructB, param_3: *mut Struct57, mut param_4: u16)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut rect: *mut Struct57;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut pSVar5: *mut StructD;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;
    let mut paVar6: *mut Struct57;

    dialog::dialog_ui_fn_1040_78e2(param_2);
    puStack6 = mixed_1010_20ba(param_3, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b), in_stack_0000fe7e,
                               in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    pSVar5 = (param_3 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_0892();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    } else {
        pSVar5 = (pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(pSVar5, PTR_LOOP_1050_5f2c);
    uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar5);
    uVar9 = (param_2 >> 0x10);
    iVar7 = param_2;
    (iVar7 + 0x8e) = uVar2;
    (iVar7 + 0x90) = pSVar5;
//   for (iStack10 = 0x1; uVar10 = (pSVar5 >> 0x10), iStack10 <= uStack8; iStack10 += 1)
    iStack10 = 1;
    uVar10 = pSVar5 >> 0x10;
    while iStack10 <= uStack8 {
        puStack26 = pass1_1010_0932(puStack6, (puStack6 >> 0x10), iStack10);
        uVar3 = (puStack26 >> 0x10);
        paVar6 = CONCAT22(uVar10, uVar3);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, 0x1050);
        mem_op_1000_179c(0x42, paVar6);
        uVar4 = paVar6 | rect;
        pSVar5 = (paVar6 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar1 = (iVar7 + 0x8e);
            (uVar1 + iStack10 * 0x4) = 0;
        } else {
            uVar10 = (iVar7 + 0x6);
            pass1_1008_3bd6(pSVar5, rect, paVar6, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((uVar10 >> 0x8), CONCAT12(uVar10, (puStack26 + 0x4))), param_4, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar1 = (iVar7 + 0x8e);
            uVar10 = (uVar1 >> 0x10);
            iVar8 = uVar1;
            (iVar8 + iStack10 * 0x4) = rect;
            (iVar8 + iStack10 * 0x4 + 0x2) = pSVar5;
        }
        uVar1 = (iVar7 + 0x8e);
        uVar10 = (uVar1 >> 0x10);
        iVar8 = uVar1;
        if ((iVar8 + iStack10 * 0x4) != 0) {
            win_e::enable_win_1040_9234((iVar8 + iStack10 * 0x4), (puStack26 + 0x6));
        }
        iStack10 += 1;
    }
    window::move_win_1040_826c(param_2, -0x1, 0xffff);
    ShowWindow16(0x5, (iVar7 + 0x6));
    return;
}

pub unsafe fn show_win_1040_0c7c(param_1: *mut StructB) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut local_6: u32;

    dialog::dialog_ui_fn_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x8e);
    pass1_1010_4f30(
        uVar1,
        (uVar1 >> 0x10),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_6 + 0x2),
    );
    window::move_win_1040_826c(param_1, local_6, (local_6 >> 0x10));
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}

pub unsafe fn show_win_1040_1d50(param_1: *mut StructB)

{
    dialog::dialog_ui_fn_1040_78e2(param_1);
    window::move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}


pub unsafe fn set_win_text_1040_3590(mut param_1: u16, param_2: *mut astruct_923)

{
    let mut HVar1: HWND16;
    let mut HVar2: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut iVar5: *mut astruct_923;
    let mut uVar5: u16;
    let mut in_stack_0000f8f8: u16;
    let mut in_stack_0000fa1c: u16;
    let mut in_stack_0000fa22: u16;
    let mut in_stack_0000fa26: u16;
    let mut uVar6: u8;
    let mut in_stack_0000fa50: u16;
    let mut local_59a: u32;
    let mut local_596: u32;
    let mut BStack1426: bool;
    let mut uStack1424: u16;
    let mut local_58e: [u16; 0x41] = [0; 0x41];
    let mut local_50c: [u16; 0x80] = [0; 0x80];
    let mut uStack1036: u32;
    let mut puStack1032: *mut u32;
    let mut local_404: [u8; 0x402] = [0; 0x402];

    puStack1032 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0,
                                  CONCAT22(in_stack_0000fa50, 0x2), in_stack_0000f8f8, in_stack_0000fa1c,
                                  in_stack_0000fa22, in_stack_0000fa26);
    uVar4 = (puStack1032 >> 0x10);
    uStack1036 = (puStack1032 + 0x68);
    uVar5 = (param_2 >> 0x10);
    iVar5 = param_2;
    GetWindowText16(0x80, CONCAT22(0x1050, local_50c), iVar5.field6_0x6);
    uVar6 = SUB21(local_50c, 0x0);
    wsprintf16(local_58e, CONCAT13((local_50c >> 0x8), CONCAT12(uVar6, 0x1050)), uVar6,
               CONCAT22(uStack1036, 0x1050), (uStack1036 >> 0x10));
    BStack1426 = SetWindowText16(CONCAT22(0x1050, local_58e), iVar5.field6_0x6);
    sprintf_op_1018_34b6(BStack1426, uVar4, iVar5.field141_0x8e);
    uStack1424 = uVar4;
    pass1_1018_3d44(iVar5.field141_0x8e, CONCAT22(0x1050, &local_59a), CONCAT22(0x1050, &local_596));
    HVar1 = GetDlgItem16(0x193, iVar5.field6_0x6);
    iVar5.field148_0x98 = HVar1;
    EnableWindow16(0x1, HVar1);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, 0x1050);
    wsprintf16(local_50c, 0x50, CONCAT22(local_404, 0x1050), CONCAT22(local_596, 0x1050),
               (local_596 >> 0x10));
    HVar1 = GetDlgItem16(0x195, iVar5.field6_0x6);
    SetWindowText16(CONCAT22(0x1050, local_50c), HVar1);
    HVar2 = GetDlgItem16(0x196, iVar5.field6_0x6);
    HVar1 = HVar2;
    sprintf_op_1018_34b6(HVar2, uVar4, iVar5.field141_0x8e);
    SetWindowText16(CONCAT22(uVar4, HVar2), HVar1);
    HVar1 = GetDlgItem16(0x197, iVar5.field6_0x6);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, 0x1050);
    SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, 0x1050);
    wsprintf16(local_50c, CONCAT22(local_404, 0x1050), CONCAT22(local_59a, 0x1050),
               (local_59a >> 0x10));
    HVar1 = GetDlgItem16(0x198, iVar5.field6_0x6);
    SetWindowText16(CONCAT22(0x1050, local_50c), HVar1);
    uVar3 = GetDlgItem16(0x199, iVar5.field6_0x6);
    HVar1 = uVar3;
    unk_str_op_1018_35b0(uVar3, iVar5.field141_0x8e);
    if ((uVar4 | uVar3) == 0) {
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, 0x1050);
        SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
        GetDlgItem16(0x19a, iVar5.field6_0x6);
        HVar1 = _u16_1050_14cc;
        load_string_1010_84e0(HVar1, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, 0x1050);
        SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
        EnableWindow16(0x0, iVar5.field148_0x98);
        return;
    }
    SetWindowText16(CONCAT22(uVar4, uVar3), HVar1);
    return;
}


pub unsafe fn win_ui_op_1038_a584(mut param_1: u16, mut param_2: u16, mut param_3: i16, mut param_4: i16)

{
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  let mut hwnd_00: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut paVar3: *mut astruct_486;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut local_52: [u8;0x50] = [0;0x50];

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_4 != 0) {
    hwnd = GetDlgItem16(0x114,(param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050,local_52));
    if (uVar1 != 0) {
      puVar4 = local_52;
      uVar5 = SUB42(0x1050,0x0);
      paVar3 =
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_6006((paVar3 >> 0x10),paVar3,CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      cleanup::destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    }
  }
  return;
}


pub unsafe fn win_dlg_op_1040_2f90(mut param_1: u16, mut param_2: u32)

{
    let mut uVar1: u16;
    let mut HVar2: HWND16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: *mut astruct_943;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut l_param: *mut c_char;
    let mut in_stack_0000fd7a: u16;
    let mut in_stack_0000fd7c: u16;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000fea4: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000fea8: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000fed2: u16;
    let mut in_stack_0000fed4: u16;
    let mut local_116: *mut u32;
    let mut local_112: *mut u32;
    let mut local_10e: [u16; 0x41] = [0; 0x41];
    let mut local_8c: [u8; 0x82] = [0; 0x82];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    puStack6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fed2, 0x2), in_stack_0000fd7a,
                               in_stack_0000fe9e, in_stack_0000fea4, in_stack_0000fea8);
    paVar3 = (paVar3 & 0xffff0000 | puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x68);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    GetWindowText16(0x80, CONCAT22(0x1050, local_8c), iVar4.field6_0x6);
    wsprintf16(local_10e, CONCAT22(local_8c, 0x1050), CONCAT22(uStack10, 0x1050),
               (uStack10 >> 0x10));
    SetWindowText16(CONCAT22(0x1050, local_10e), iVar4.field6_0x6);
    HVar2 = GetDlgItem16(0x182, iVar4.field6_0x6);
    iVar4.field143_0x92 = HVar2;
    pass1_1018_3a94(iVar4.field145_0x96, CONCAT22(0x1050, &local_116), CONCAT22(0x1050, &local_112));
    send_msg_1040_3374(param_2, local_112, iVar4.field143_0x92);
    puVar5 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fed4, 0x2f), in_stack_0000fd7c,
                             in_stack_0000fea0, in_stack_0000fea6, in_stack_0000feaa);
    uVar1 = (puVar5 >> 0x10);
    uVar6 = (puVar5 + 0x24);
    uVar6 = pass1_1018_3a7a(uVar6, uVar1, iVar4.field145_0x96, uVar6);
    SendMessage16(uVar6, 0xffff, 0x40d, iVar4.field143_0x92);
    HVar2 = GetDlgItem16(0x183, iVar4.field6_0x6);
    iVar4.field144_0x94 = HVar2;
    send_msg_1040_3374(param_2, local_116, HVar2);
    l_param = load_string_1010_847e(_u16_1050_14cc, 0x531);
    SendDlgItemMessage16(l_param, 0x0, 0x403, 0x183, iVar4.field6_0x6);
    SendDlgItemMessage16(l_param, 0xffff, 0x40d, 0x183, iVar4.field6_0x6);
    HVar2 = GetDlgItem16(0x181, iVar4.field6_0x6);
    iVar4.field141_0x8e = HVar2;
    HVar2 = GetDlgItem16(0x184, iVar4.field6_0x6);
    iVar4.field142_0x90 = HVar2;
    return;
}

pub unsafe fn set_window_text_1018_6066(
    param_1: *mut astruct_937,
    mut param_2: u16,
    in_win_text_3: *mut c_char,
    dialog_id_5: INT16,
) {
    let mut hwnd: HWND16;

    hwnd = GetDlgItem16(dialog_id_5, param_1.hwnd_field_0x6);
    SetWindowText16(in_win_text_3, hwnd);
    return;
}


pub unsafe fn set_window_text_1018_6086(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut hwnd_1: HWND16;
    let mut uVar2: u16;

    wsprintf16(&stack0xfff4, 0x42421050, CONCAT22(param_3, 0x1050));
    uVar2 = (param_1 >> 0x10);
    hwnd_1 = GetDlgItem16(0x1be, (param_1 + 0x6));
    SetWindowText16(CONCAT22(0x1050, &stack0xfff4), hwnd_1);
    wsprintf16(&stack0xfff4, 0x42451050, CONCAT22(param_2, 0x1050));
    hwnd_1 = GetDlgItem16(0x1bf, (param_1 + 0x6));
    SetWindowText16(CONCAT22(0x1050, &stack0xfff4), hwnd_1);
    return;
}


pub unsafe fn set_win_tet_1020_1d2a(
    param_1: *mut astruct_938,
    mut param_2: u16,
    in_win_text_3: *mut c_void,
    mut param_4: u16,
    in_dlg_id_5: INT16,
) {
    let mut hwnd: HWND16;

    hwnd = GetDlgItem16(param_4, param_1.field6_0x6);
    SetWindowText16(in_win_text_3, hwnd);
    return;
}


pub unsafe fn set_win_text_1038_8358(mut param_1: u16, param_2: *mut Struct903)

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut uVar4: u16;
  let mut uVar3: u16;
  let mut local_30a: [u8;0x102] = [0;0x102];
  let mut local_208: [u8;0x100] = [0;0x100];
  let mut local_108: [u8;0x100] = [0;0x100];
  let mut uStack8: u32;
  let mut HStack4: HWND16;
  let mut uVar1: u32;

  uVar3 = (param_2 >> 0x10);
  uVar4 = param_2;
  HStack4 = GetDlgItem16(0x1857,(uVar4 + 0x6));
  uStack8 = pass1_1008_b820(HStack4,param_1,(uVar4 + 0x94));
  if (uStack8 == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_30a,0x1050);
    pcVar1 = local_30a;
  }
  else {
    uVar2 = win_a::send_dlg_item_msg_1038_8164(uVar4, uVar3, CONCAT22(0x1050, local_108), 0x1854);
    if (uVar2 == 0) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_208,0x1050);
    }
    else {
      load_string_1008_b65a((uVar4 + 0x94),local_208,CONCAT22(local_108,0x1050),0x1050);
    }
    pcVar1 = local_208;
  }
  SetWindowText16(CONCAT22(0x1050,pcVar1),HStack4);
  return;
}


pub unsafe fn win_ui_op_1038_a4ee(mut param_1: u16, mut param_2: u16, struct_b_param_1: *mut StructB)

{
  let mut lp_string: u32;
  let mut hwnd: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
   let mut struct_b_1: *mut StructB;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut LVar5: LRESULT;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
  win_1008_5c7c(param_1,paVar1,_u16_1050_02a0,0x20001);
  uVar3 = (struct_b_param_1 >> 0x10);
  struct_b_1 = struct_b_param_1;
  (struct_b_1 + 0x7).field0_0x0 = param_1;
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar2 = (paVar1 >> 0x10);
  lp_string = (puVar4 + 0x6c);
  hwnd = GetDlgItem16(0x114,struct_b_1.lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar5 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  window::unk_win_ui_op_1038_a18c(CONCAT22(uVar2, (LVar5 >> 0x10)), struct_b_param_1, in_stack_0000ff9e);
  ShowWindow16(0x5,struct_b_1.lpvoid_field_0x8);
  return;
}

pub unsafe fn set_win_text_1008_9664(mut param_1: u32, mut param_2: u16, param_3: *mut c_char) {
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 0xa), (param_1 + 0x8));
    return;
}


pub unsafe fn win_ui_op_1040_2512(param_1: *mut Struct57, mut param_2: u16, param_3: *mut StructC, mut param_4: u32, mut param_5: u16) -> u32

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_263;
    let mut uVar4: u16;
    let mut UVar4: u16;
    let mut HVar5: HWND16;
    let mut BVar6: bool;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut UVar6: u16;
    let mut uVar7: u16;
    let mut uVar11: u16;
    let mut puVar8: *mut u8;
    let mut uVar12: u16;
    let mut puVar9: *mut u8;
    let mut iVar8: *mut StructC;
    let mut iVar9: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut puVar15: *mut u16;
    let mut uVar10: u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_1e: [u8; 0x4] = [0; 0x4];
    let mut uStack26: u16;
    let mut uStack24: u16;
    let mut local_16: [*mut u32; 0x2];
    let mut uStack12: u16;
    let mut puStack10: *mut u32;
    let mut BStack6: bool;
    let mut uStack4: u16;
    let mut piVar1: *mut i16;
    let mut in_stack_0000ffdc: u16;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let mut fn_ptr_21: *mut *mut code;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0x2) {//
// LAB_1040_266d:
        BStack6 = 0x1;
        uStack4 = 0;
    } else {
        iVar8 = param_3;
        if (param_5 - 0x2 < 0x19e) {//
// LAB_1040_2539:
            param_2 = param_5;
        } else {
            uVar8 = (param_3 >> 0x10);
            if (param_5 - 0x1a0 < 0x14 || param_5 == 0x1b4) {
                UVar4 = IsDlgButtonChecked(param_5, iVar8.field6_0x6);
                if (UVar4 == 0) {
                    puVar1 = &iVar8.field142_0x92;
                    puVar1 = puVar1 + 1;
                    if (0x0 < &iVar8.field142_0x92) {
                        (&iVar8.field142_0x92 + 0x2) = 0;
                    }
                    paVar3 = iVar8.field141_0x8e;
                    if ((paVar3 + 0x28) == &iVar8.field142_0x92) {
                        HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                        EnableWindow16(0x0, HVar5);
                    }
                } else {
                    puVar1 = &iVar8.field142_0x92;
                    puVar1 = puVar1 - 0x1;
                    HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                    BVar6 = IsWindowEnabled16(HVar5);
                    if (BVar6 == 0) {
                        HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                        EnableWindow16(0x1, HVar5);
                    }
                    if (&iVar8.field142_0x92 < 1) {
                        (&iVar8.field142_0x92 + 0x2) = 0x1;
                    }
                    pass1_1018_1c9a(iVar8.field141_0x8e, param_5);
                    puStack10 = pass1_1018_1e78(iVar8.field141_0x8e, -1);
                    uVar2 = (puStack10 >> 0x10);
                    uVar11 = uVar2 | puStack10;
                    if (uVar11 == 0) {
                        uStack12 = 0;
                    } else {
                        uStack12 = (puStack10 + 0x1c);
                    }
                    win_1008_5c7c(uStack12, uVar11, _u16_1050_02a0, CONCAT22(uStack12, 1));
                }
                if ((-0x1 < &iVar8.field142_0x92) && (paVar3 = iVar8.field141_0x8e, &iVar8.field142_0x92 <= (paVar3 + 0x28))) {
                    sys_1000_3f9c(CONCAT13(0x10, CONCAT12(0x50, local_16)), s__d_1050_5cf4, &iVar8.field142_0x92);
                    SetDlgItemText16(CONCAT22(0x1050, local_16), 0xfb2, iVar8.field6_0x6);
                }
                // TODO: goto LAB_1040_266d;
            }
            uVar4 = param_5 - 0xfb1;
//      if (uVar4 != 0) goto LAB_1040_2539;
            if (&iVar8.field142_0x92 < 0x0) {
                mem_op_1000_179c(0xb4, param_1);
                uStack24 = param_1;
                puVar8 = (uStack24 | uVar4);
                uVar13 = param_1 & 0xffff0000 | ZEXT24(puVar8);
                uStack26 = uVar4;
                if (puVar8.is_null()) {
                    iVar6 = 0;
                    uVar12 = 0;
                } else {
                    iVar6 = string_1040_8520(uVar13,
                                             CONCAT13((param_1 >> 0x8), CONCAT12(param_1, uVar4)),
                                             HWND16_1050_0396, 0x20030, 0x57c057b);
                    uVar12 = uVar13;
                }
                puStack10 = CONCAT22(uVar12, iVar6);
                fn_ptr_21 = (*puStack10 + 0x74);
                (**fn_ptr_21)(0x1000, iVar6, uVar12);
                // TODO: goto LAB_1040_27c0;
            }
            if (0x0 < &iVar8.field142_0x92) {
                mem_op_1000_179c(0xb4, param_1);
                uStack24 = param_1;
                puVar9 = (uStack24 | uVar4);
                uVar13 = param_1 & 0xffff0000;
                uVar14 = uVar13 | ZEXT24(puVar9);
                uStack26 = uVar4;
                if (puVar9.is_null()) {
                    iVar7 = 0;
                } else {
                    iVar7 = string_1040_8520(uVar14,
                                             CONCAT13((param_1 >> 0x8), CONCAT12(param_1, uVar4)),
                                             HWND16_1050_0396, 0x20021, 0x57d057b);
                    uVar13 = uVar14;
                }
                puStack10 = CONCAT22(uVar13, iVar7);
                puVar15 = pass1_1008_941a(CONCAT22(0x1050, local_1e), 0x1, 0xc2);
                param_1 = (uVar13 & 0xffff0000 | puVar15 >> 0x10);
                param_2 = 0x1050;
                fn_ptr_21 = (*puStack10 + 0x6c);
                uVar10 = (**fn_ptr_21)(0x1008, puStack10, (puStack10 >> 0x10), local_1e);
//        if (uVar10 == 0x2) goto LAB_1040_27c0;
            }
            local_16[0] = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x6), in_stack_0000fe84,
                                          in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            param_1 = (local_16[0] >> 0x10);
            uStack12 = 0x1a0;
            loop {
                UVar6 = IsDlgButtonChecked(uStack12, iVar8.field6_0x6);
                if (UVar6 == 1) {
                    uVar9 = (local_16[0] >> 0x10);
                    iVar9 = local_16[0];
                    (iVar9 + (iVar9 + 0x56) * 0x2 + 0x4e) = uStack12;
                    piVar1 = (iVar9 + 0x56);
                    *piVar1 = *piVar1 + 1;
                }
                uStack12 += 0x1;
                if uStack12 >= 0x1b5 { break; }
            }
            uVar2 = &iVar8.field142_0x92;
            puStack10 = (puStack10 & 0xffff0000 | uVar2);
            paVar3 = iVar8.field141_0x8e;
            (paVar3 + 0x28) = uVar2;
            PostMessage16(0x0, 0xc8, 0x111, HWND16_1050_0396);
            param_2 = 0x1;
        }
        uVar12 = SUB42(param_1, 0x0);
        BStack6 = post_win_msg_1040_7b3c(param_3, param_4, (param_4 >> 0x10), param_2);
        uStack4 = uVar12;
    }//
// LAB_1040_27c0:
    return CONCAT22(uStack4, BStack6);
}


pub unsafe fn win_ui_op_1040_42b2(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iVar5: *mut astruct_893;
    let mut uVar5: u16;
    let mut LVar6: LRESULT;
    let mut local_54: [u16; 0x29] = [0; 0x29];

    iVar5 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        iVar5.field147_0x9a = 0x1;
        DestroyWindow16(iVar5.field6_0x6);
        return;
    }
    pass1_1000_4906(CONCAT22(0x1050, local_54), NULL, 0x51);
    HVar2 = GetDlgItem16(0xfb5, iVar5.field6_0x6);
    LVar6 = SendMessage16(CONCAT22(0x1050, local_54), 0x51, 0xd, HVar2);
    uVar4 = (LVar6 >> 0x10);
    uVar3 = pass1_1000_3e2c(CONCAT22(0x1050, local_54));
    if ((uVar4 | uVar3) != 0) {
        iVar5.field142_0x92 = uVar3;
        (&iVar5.field142_0x92 + 0x2) = uVar4;
    }
    if (uVar4 < 0x0) {
        uVar1 = iVar5.field141_0x8e;
        uVar1 = (uVar1 + 0x76);
        wsprintf16(local_54, 0x5d3c1050, CONCAT22(uVar1, 0x1050), (uVar1 >> 0x10));
        SendMessage16(CONCAT22(0x1050, local_54), 0x0, 0xc, HVar2);
        SetFocus16(HVar2);
        SendMessage16(-0x10000, 0x0, 0x401, HVar2);
        return;
    }
    HVar2 = GetDlgItem16(0x1, iVar5.field6_0x6);
    EnableWindow16(0x0, HVar2);
    uVar1 = iVar5.field141_0x8e;
    (uVar1 + 0x76) = iVar5.field142_0x92;
    PostMessage16(iVar5.field142_0x92, 0x0, 0x400, HWND16_1050_0396);
    HVar2 = GetDlgItem16(0x1, iVar5.field6_0x6);
    EnableWindow16(0x1, HVar2);
    return;
}


pub unsafe fn enable_win_1040_5780(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut hwnd: HWND16;
    let mut in_EDX: *mut Struct57;
    let mut iVar4: *mut astruct_945;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut paVar6: *mut astruct_945;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    ppcVar1 = (*param_1 + 0x74);
    paVar6 = iVar4;
    (**ppcVar1)();
    puVar5 = mixed_1010_20ba(in_EDX, _u16_1050_0ed0, CONCAT22(paVar6, 0x3), in_stack_0000fe98, in_stack_0000ffbc,
                             in_stack_0000ffc2, in_stack_0000ffc6);
    uVar2 = iVar4.field143_0x90;
    uVar3 = pass1_1010_acc0(puVar5, (puVar5 >> 0x10), (uVar2 + 0x6));
    if (uVar3 != 0) {
        hwnd = GetDlgItem16(0x1790, iVar4.field6_0x6);
        EnableWindow16(0x1, hwnd);
    }
    return;
}

pub unsafe fn enable_win_1040_6880(param_1: *mut astruct_925, mut param_2: i16)

{
    let mut uVar2: u32;
    let mut HVar3: HWND16;
    let mut iVar3: *mut astruct_925;
    let mut uVar4: u16;
    let mut uVar1: u32;

    if (param_2 == 0x8) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar3 = GetDlgItem16(0x107, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x24), HVar3);
        HVar3 = GetDlgItem16(0x108, iVar3.field6_0x6);
        uVar2 = iVar3.field147_0x94;
        EnableWindow16((uVar2 + 0x26), HVar3);
    }
    return;
}

pub unsafe fn enable_win_1040_6ff2(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut iVar3: *mut astruct_926;
    let mut uVar3: u16;

    if (param_2 == 0x8) {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar2 = GetDlgItem16(0x107, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x26), HVar2);
    }
    return;
}


pub unsafe fn enable_window_1040_8ea0(param_1: *mut u8, param_2: *mut Struct903, mut param_3: u16, mut param_4: u32)

{
    let mut enable: bool;
    let mut hwnd: HWND16;

    if (param_4 == 0xf8) {
        hwnd = GetDlgItem16(0x17d8, (param_2 + 0x6));
        enable = 0x1;
    } else {
        if (param_4 != 0x17d8) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4);
            return;
        }
        SetWindowPos16(0x6, 0xf6, 0x269, 0x0, 0x0, 0x0, (param_2 + 0x6));
        enable = 0x1538;
        GetDlgItem16(0x17d8, (param_2 + 0x6));
        hwnd = 0;
    }
    EnableWindow16(enable, hwnd);
    return;
}


pub unsafe fn enable_win_1040_3a36(param_1: *mut astruct_924, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> u16

{
    let mut puVar1: *mut u16;
    let mut bVar2: bool;
    let mut iVar3: *mut astruct_924;
    let mut uVar3: u16;

    bVar2 = false;
    iVar3 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_4 == 0) {
//    if (iVar3.field155_0x9e <= iVar3.field154_0x9c) goto LAB_1040_3a79;
        puVar1 = &iVar3.field154_0x9c;
        *puVar1 = *puVar1 + 1;
    } else {
//    if (param_4 != 1) goto LAB_1040_3a79;
//    if (iVar3.field154_0x9c == 0) goto LAB_1040_3a79;
        puVar1 = &iVar3.field154_0x9c;
        *puVar1 = *puVar1 - 0x1;
    }
    bVar2 = true;//
// LAB_1040_3a79:
    if (bVar2) {
        SetDlgItemInt16(0x0, iVar3.field154_0x9c, 0x18e, iVar3.field6_0x6);
    }
    if ((iVar3.field154_0x9c != 0) && (iVar3.field158_0xa2 == 0)) {
        iVar3.field158_0xa2 = 0x1;
        EnableWindow16(0x1, iVar3.field153_0x9a);
    }
    if ((iVar3.field154_0x9c == 0) && (iVar3.field158_0xa2 != 0)) {
        iVar3.field158_0xa2 = 0;
        EnableWindow16(0x0, iVar3.field153_0x9a);
    }
    return 0x0;
}

pub unsafe fn enable_window_1020_1bd4(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_901,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut bVar2: bool;
    let mut hwnd: HWND16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar8: u16;
    let mut puStack12: *mut u32;
    let mut uVar7: u32;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    bVar2 = false;
    pass1_1020_1d8e(
        CONCAT13((param_4 >> 0x8), CONCAT12(param_4, param_3)),
        CONCAT22(param_6, param_5),
    );
    if (param_1 != 0) {
        if (param_1 < 0x2) {
            bVar2 = true;
        } else {
            hwnd = GetDlgItem16(0x1, param_3.field6_0x6);
            pass1_1010_4e8c(param_3.field141_0x8e);
            param_1 = EnableWindow16(0x1, hwnd);
            pass1_1010_4df0(paVar6, param_3.field141_0x8e);
            if ((param_1 == 0) && (bVar2 = true, param_3.field146_0x96 == 0)) {
                param_1 = EnableWindow16(0x0, hwnd);
            }
        }
    }
    if (bVar2) {
        uVar8 = 0x1000;
        mem_op_1000_179c(0xb4, paVar6);
        uVar4 = paVar6 | param_1;
        uVar7 = paVar6 & 0xffff0000 | uVar4;
        if (uVar4 == 0) {
            iVar3 = 0;
            uVar5 = 0;
        } else {
            uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar3 = string_1040_8520(
                uVar7,
                CONCAT22(paVar6, param_1),
                param_3.field6_0x6,
                0x20030,
                0x62a057b,
            );
            uVar5 = uVar7;
        }
        puStack12 = CONCAT22(uVar5, iVar3);
        ppcVar1 = (*puStack12 + 0x74);
        (**ppcVar1)(uVar8, iVar3, uVar5);
    }
    return;
}


pub unsafe fn win_ui_op_1040_2bb2(param_1: *mut u8, pstruct_903_param_2: *mut Struct903, mut param_3: u16, mut param_4: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut HVar3: HWND16;
    let mut iVar4: *mut astruct_922;
    let mut iVar5: i16;
    let mut iVar3: *mut astruct_920;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut id: *mut u8;
    let mut iStack8: i16;
    let mut iStack4: i16;

    if (param_4 == 0x158) {
        PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04.is_null());
        if (PTR_LOOP_1050_5d04.is_null()) {
            //   for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 1)
            for iStack8 in 1..5 {
                iVar5 = iStack8 * 0xc;
                HVar3 = GetDlgItem16((iVar5 + 0x5d00), (pstruct_903_param_2 + 0x6));
                EnableWindow16(0x0, HVar3);
                (&PTR_LOOP_1050_5d04 + iVar5) = 0;
                SetDlgItemText16((pstruct_903_param_2 + 0x94),
                                 (&PTR_s_post_1050_015d_1050_5d06 + iVar5),
                                 (pstruct_903_param_2 + 0x6));
            }
            HVar3 = (pstruct_903_param_2 + 0x6);
            uVar1 = (pstruct_903_param_2 + 0x94);
            uVar6 = uVar1;
            uVar7 = (uVar1 >> 0x10);
            id = PTR_s_post_1050_015d_1050_5d06;
            // TODO: goto LAB_1040_2ccc;
        }
        // for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 1)
        for iStack8 in 1..5 {
            iVar3 = (iStack8 * 0xc);
            HVar3 = GetDlgItem16((iVar3 + 0x5d00), (pstruct_903_param_2 + 0x6));
            EnableWindow16(0x1, HVar3);
            (iVar3 + 0x5d04) = 0;
            SetDlgItemText16((pstruct_903_param_2 + 0x94), (iVar3 + 0x5d06),
                             (pstruct_903_param_2 + 0x6));
        }
        HVar3 = (pstruct_903_param_2 + 0x6);
        id = PTR_s_post_1050_015d_1050_5d06;
    } else {
        if (param_4 == 0x159) {
            iStack4 = 0x1;
        } else if (param_4 == 0x15a) {
            iStack4 = 0x2;
        } else if (param_4 == 0x15b) {
            iStack4 = 0x3;
        } else {
            if (param_4 != 0x15c) {
                pass1_1040_b54a(param_1, pstruct_903_param_2, param_3, param_4);
                return;
            }
            iStack4 = 0x4;
        }
        if (iStack4 == 0) {
            return;
        }
        iVar4 = (iStack4 * 0xc);
        uVar2 = ((iVar4 + 0x5d04) == 0);
        (iVar4 + 0x5d04) = uVar2;
        if (uVar2 == 0) {
            HVar3 = (pstruct_903_param_2 + 0x6);
            uVar1 = (pstruct_903_param_2 + 0x94);
            uVar6 = uVar1;
            uVar7 = (uVar1 >> 0x10);
            id = (iVar4 + 0x5d06);
            // TODO: goto LAB_1040_2ccc;
        }
        HVar3 = (pstruct_903_param_2 + 0x6);
        id = (&PTR_s_post_1050_015d_1050_5d06 + iStack4 * 0xc);
    }
    uVar1 = (pstruct_903_param_2 + 0x98);
    uVar6 = uVar1;
    uVar7 = (uVar1 >> 0x10);//
// LAB_1040_2ccc:
    SetDlgItemText16(CONCAT22(uVar7, uVar6), id, HVar3);
    return;
}
