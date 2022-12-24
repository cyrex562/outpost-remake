use crate::block_1008::block_1008_5000;
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
