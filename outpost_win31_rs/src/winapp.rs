use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1040::block_1040_7000::dialog_ui_fn_1040_78e2;
use crate::globals::DAT_1050_1050;
use crate::utils::CONCAT22;
use crate::winbase::SetWindowPos16;

pub unsafe fn unk_win_sys_op_1038_da68(param_1: *mut StructD, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut in_BX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u8;
  let mut iVar9: i16;
  let mut puStack14: *mut u32;
  let mut uStack8: u16;
  let mut iStack4: i16;

  paVar5 = CONCAT22(in_register_0000000a,param_1);
  uVar8 = (param_3 >> 0x8);
  if (param_4 == 0x204) {
    pass1_1038_de20(in_BX,param_1,CONCAT13(uVar8,CONCAT12(param_3,param_2)),0x204,param_5,
                    param_5);
    return;
  }
  iStack4 = 0;
  uStack8 = 0;
  if (param_5 == 0x121) {
    iStack4 = 0x6ec;
    uStack8 = 0x15;
// TODO: goto LAB_1038_dac3;
  }
  if (param_5 < 0x1220000) {
    uVar2 = param_5 - 0x100;
    if (uVar2 == 0) {
      param_5 = uVar2;
      if ((param_2 + 0x8e) == 0) {
        pass1_1010_1ea6(_u16_1050_02a0,CONCAT22(param_3,param_2));
        (param_2 + 0x90) = 0;
      }
      iStack4 = 0x72c;
      uStack8 = 0x48;
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 - 0x11c == 0) {
      param_5 = param_5 - 0x11c;
      pass1_1038_df86(param_1,CONCAT22(param_3,param_2));
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 == 0x11d) {
      uVar7 = pass1_1038_df5c(param_1,CONCAT22(param_3,param_2));
      paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
      param_5 = uVar7;
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 == 0x11e) {
      iVar9 = 0x1d;
    }
    else {
      if (param_5 != 0x120) {//
// LAB_1038_dc20:
        post_win_msg_1040_7b3c
                  (CONCAT13(uVar8,CONCAT12(param_3,param_2)),param_4,param_5,param_5);
        return;
      }
      iVar9 = 0x1c;
    }
  }
  else if (param_5 == 0x122) {
    iVar9 = 0x14;
  }
  else {
    if (param_5 != 0x123) {
      if (param_5 - 0x125 == 0) {
        ppcVar1 = (*_u16_1050_02a0 + 0x4);
        param_5 = param_5 - 0x125;
        (**ppcVar1)();
        (param_2 + 0x90) = 0x1;
        win_1008_5c5c(param_5,paVar5,_u16_1050_02a0,0x1db);
        (param_2 + 0x8e) = 0x100;
      }
      else {
        iVar9 = param_5 - 0x126;
        if (iVar9 == 0) {
          (param_2 + 0x8e) = 0;
          win_1008_5c7c(0x0,param_1,_u16_1050_02a0,0xcb0001);
          uVar3 = FUN_1010_830a(iVar9,paVar5,0x1008,_u16_1050_14cc,0x1f8);
          param_5 = WinHelp16(0x0,0x3,CONCAT22(paVar5,uVar3),(param_2 + 0x6));
        }
        else {
//          if (param_5 - 0x127 != 0) goto LAB_1038_dc20;
          param_5 = param_5 - 0x127;
          post_win_msg_1038_dcb0(0x0,paVar5,CONCAT22(param_3,param_2));
        }
      }
  // TODO: goto LAB_1038_dac3;
    }
    iVar9 = 0x28;
  }
  uVar7 = pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),iVar9);
  paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
  param_5 = uVar7;//
// LAB_1038_dac3:
  if (iStack4 != 0) {
    mem_op_1000_179c(0xb4,paVar5);
    uVar4 = paVar5 | param_5;
    uVar7 = paVar5 & 0xffff0000 | uVar4;
    if (uVar4 == 0) {
      uVar6 = 0x1000;
      iVar9 = 0;
      uVar3 = 0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar9 = string_1040_8520(uVar7,
                                     CONCAT13((paVar5 >> 0x8),CONCAT12(paVar5,param_5)),
                               (param_2 + 0x6),0x20000,CONCAT22(iStack4,0x634));
      uVar3 = uVar7;
    }
    puStack14 = CONCAT22(uVar3,iVar9);
    if (uStack8 == 0) {
      ppcVar1 = (*puStack14 + 0x74);
      (**ppcVar1)(uVar6,iVar9,uVar3);
    }
    else {
      pass1_1008_941a(CONCAT22(0x1050,&stack0xffea),0x1,uStack8);
      ppcVar1 = (*puStack14 + 0x6c);
      (**ppcVar1)(0x1008,iVar9,uVar3,&stack0xffea,&DAT_1050_1050);
    }
  }
  return;
}


pub unsafe fn create_win_1040_20d4(mut param_1: u32, struct_b_param_2: *mut StructB, mut param_3: u16)

{
    let mut cx: i16;
    let mut struct_b_1: *mut StructB;
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut window_name: *mut c_char;
    let mut in_stack_0000fe72: u16;
    let mut in_stack_0000ff96: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut local_1e: RECT16;
    let mut iStack26: i16;
    let mut iStack24: *mut astruct_858;
    let mut uStack22: u32;
    let mut uStack18: u32;
    let mut iStack14: i16;
    let mut uStack12: u16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut uStack6: u16;
    let mut iStack4: i16;

    dialog_ui_fn_1040_78e2(struct_b_param_2);
    puVar2 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_3, 0x48), in_stack_0000fe72,
                             in_stack_0000ff96, in_stack_0000ff9c, in_stack_0000ffa0);
    uStack12 = (puVar2 >> 0x10);
    iStack14 = puVar2;
    iStack8 = (iStack14 + 0xa);
    iStack10 = (iStack14 + 0xc);
    uVar1 = (struct_b_param_2 >> 0x10);
    struct_b_1 = struct_b_param_2;
    uStack18 = pass1_1008_4772(&struct_b_1[0x7].field1_0x2);
    cx = (uStack18 + 0x4);
    iStack4 = (iStack8 - cx) / 0x2;
    uStack6 = 0x5;
    SetWindowPos16(0x6, 0x1d6, cx, 0x5, iStack4, 0x0, struct_b_1.lpvoid_field_0x8);
    GetClientRect16(&local_1e, &DAT_1050_1050);
    window_name = load_string_1010_847e(_u16_1050_14cc, 0x592);
    uStack22 = 0x50010001;
    CreateWindow16(0x0, CONCAT22(0x1, HINSTANCE16_1050_038c), struct_b_1.lpvoid_field_0x8, 0x19, 0x58,
                   (iStack24 - 0x28), (iStack26 - 0x58) / 0x2, 0x1, s_Rebel_1050_4ffc + 0x5, window_name,
                   s_OPButton_1050_5ce4);
    SetWindowPos16(0x45, iStack10 - 0xa, (uStack18 + 0x4), 0x5, iStack4, 0x0,
                   struct_b_1.lpvoid_field_0x8);
    return;
}
