use crate::structs::struct_57::Struct57;
use crate::unk::block_1008_5000;
use crate::unk::block_1008_5000::win_1008_5c5c;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::utils::{CONCAT22, SUB42};
use crate::gui::cleanup;
use crate::winapi16::ShowCursor16;
use crate::windef16::HWND16;

pub unsafe fn win_ui_op_1008_5cfe(param_1: *mut Struct27, param_2: *mut c_char, in_wnd_class: *mut WNDCLASS16 )

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut iVar3: *mut Struct27;
  let mut uVar3: u16;
  let mut DVar4: u32;
  let mut iVar5: i16;
  let mut message_1: HWND16;
  let mut uStack298: u16;
  let mut window_handle_1: HWND16;
  let mut local_11e: [u8;0x100] = [0;0x100];
  let mut string_1: *mut c_char;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut local_16: [u8;0x4] = [0;0x4];
  let mut offset_val: u16;
  let mut pcStack14: *mut c_char;
  let mut pcStack10: *mut c_char;

  pass1_1000_4906(CONCAT22(0x1050,local_16),NULL,0x14);
  pcStack10 = param_2;
  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = &iVar3.field_0xc;
  iStack24 = (uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,NULL,0x0,0x0,CONCAT22(0x1050,local_11e));
  iVar2 = pass1_1000_475e(CONCAT22(0x1050,local_11e),s__mid_1050_02ae);
  if (iVar2 == 0) {
    uVar1 = &iVar3.field_0xc;
    iStack24 = (uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0;
  }
  if (iStack24 != 0) {
    if ((iStack26 != 0) && (&iVar3.field_0x10 != 0)) {
      return;
    }
    if ((iStack26 == 0) && (iVar3.field18_0x12 != 0)) {
      return;
    }
    pcStack14 = string_1;
    DVar4 = mciSendCommand16(CONCAT22(0x1050,local_16),0x2200,0x803,0x0);
    if (((DVar4 >> 0x10) | DVar4) == 0) {
      if (iStack26 == 0) {
        iVar3.field18_0x12 = 0x1;
      }
      else {
        iVar3.field_0xa = offset_val;
        iVar3.field_0x10 = 0x1;
      }
      window_handle_1 = create_window_1008_5e7e();
      if (window_handle_1 == 0) {
        block_1008_5000::mci_send_command_1008_5cb6(param_1, offset_val);
        return;
      }
      pass1_1000_4906(CONCAT22(0x1050,&message_1),NULL,0xc);
      message_1 = window_handle_1;
      uStack298 = 0;
      mciSendCommand16(CONCAT22(0x1050,&message_1),0x1,0x806,offset_val);
      SetWindowWord16(offset_val,0x0,window_handle_1);
      return;
    }
  }
  if (iStack26 == 0) {
    iVar5 = 0x11;
  }
  else {
    iVar5 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar5);
  return;
}

pub unsafe fn mci_send_command_1008_53ae(mut param_1: u32, mut param_2: u16 )

{
  let mut DVar1: u32;
  let mut DVar2: u32;
  let mut w_error: u32;
  let mut local_32: u16;
  let mut uStack48: u16;
  let mut local_2e: u16;
  let mut uStack44: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut local_1e: u32;
  let mut UStack26: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;

  local_1e = 0;
  uStack22 = 0x28c;
  uStack20 = 0x1050;
  uStack18 = param_1;
  uStack14 = 0;
  uStack10 = 0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(CONCAT22(0x1050,&local_1e),0x30200,0x803,0x0);
  DVar1 = (DVar1 >> 0x10);
  uStack34 = DVar1;
  if (UStack26 != 0) {
    if ((DVar1 | uStack34) != 0) {
      mciGetErrorString16(0x400,CONCAT22(0x1050,&stack0xfbce),DVar1);
    }
    pass1_1000_4906(CONCAT22(0x1050,&local_2e),NULL,0xc);
    local_2e = param_2;
    uStack44 = 0;
    DVar2 = mciSendCommand16(CONCAT22(0x1050,&local_2e),0x2,0x806,UStack26);
    DVar2 = (DVar2 >> 0x10);
    uStack34 = DVar2;
    DVar1 = DVar2;
    if ((DVar2 | uStack34) != 0) {
      mciGetErrorString16(0x400,CONCAT22(0x1050,&stack0xfbce),DVar2);
    }
    local_32 = param_2;
    uStack48 = 0;
    w_error = mciSendCommand16(CONCAT22(0x1050,&local_32),0x1,0x804,UStack26);
    DVar1 = (w_error >> 0x10);
    uStack34 = w_error;
    if ((DVar1 | uStack34) != 0) {
      mciGetErrorString16(0x400,CONCAT22(0x1050,&stack0xfbce),w_error);
    }
  }
  return;
}


pub unsafe fn mix_ui_op_1018_6adc(
    param_1: *mut astruct_28,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar6: u32;
    let mut pstruct28_1: *mut astruct_28;
    let mut uVar5: *mut astruct_28;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    puVar8 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x48),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    uVar6 = puVar8 >> 0x10;
    uVar7 = (puVar8 >> 0x10);
    iVar1 = (puVar8 + 0xa);
    iVar2 = (puVar8 + 0xc);
    uVar5 = (param_1 >> 0x10);
    pstruct28_1 = param_1;
    if (0x1 < iVar1 - pstruct28_1.field227_0xe6) {
        uVar6 = (iVar1 >> 0xf);
        pstruct28_1.field225_0xe2 = iVar1 / 0x2 - (pstruct28_1.field227_0xe6 + 1) / 0x2;
    }
    if (0x1 < iVar2 - pstruct28_1.field228_0xe8) {
        uVar6 = (iVar2 >> 0xf);
        pstruct28_1.field226_0xe4 = iVar2 / 0x2 - (pstruct28_1.field228_0xe8 + 1) / 0x2;
    }
    uVar6 = paVar4 & 0xffff0000 | uVar6;
    uVar7 = SUB42(0x1538, 0x0);
    uVar3 = ShowCursor16(0x0);
    if (pstruct28_1.field231_0xee != 0) {
        uVar7 = 0x1008;
        win_1008_5c5c(uVar3, uVar6, _u16_1050_02a0, pstruct28_1.field231_0xee);
        pstruct28_1.hwnd_0xf0 = uVar3;
    }
    uVar7 = FUN_1010_830a(
        uVar3,
        uVar6,
        uVar7,
        _u16_1050_14cc,
        pstruct28_1.field230_0xec,
    );
    mci_send_command_1008_53ae(CONCAT22(uVar6, uVar7), pstruct28_1.field8_0x8);
    ShowCursor16(1);
    cleanup::unk_destroy_window_op_1018_6bb6(param_1);
    return;
}
