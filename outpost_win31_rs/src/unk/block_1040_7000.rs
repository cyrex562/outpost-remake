








pub unsafe fn pass1_1040_78de()

{
  return;
}


pub unsafe fn pass1_1040_79c0(param_1: u32,param_2: *mut i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut cVar2: u8;
  let mut uVar3: u16;

  if (param_5 == 0xa1) {
    ppcVar1 = (*param_1 + 0x38);
    uVar3 = (**ppcVar1)();
    return uVar3;
  }
  if (param_5 < 0xa2) {
    if (param_5 == 0x85) {
      ppcVar1 = (*param_1 + 0x1c);
      (**ppcVar1)();
      return 0x1;
    }
    if (param_5 < 0x86) {
      cVar2 = param_5;
      if (cVar2 == '\x02') {
        ppcVar1 = (*param_1 + 0x24);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (*param_1 + 0x18);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (*param_1 + 0x60);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (cVar2 == '+') {
        if (*param_2 != 0x4) {
          return 0x1;
        }
        win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2));
        return 0x1;
      }
    }
  }
  else {
    if (param_5 == 0x114) {
      ppcVar1 = (*param_1 + 0x58);
      uVar3 = (**ppcVar1)();
      return uVar3;
    }
    if (param_5 < 0x115) {
      if (param_5 == 0x104) {
        ppcVar1 = (*param_1 + 0x30);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x111) {
        ppcVar1 = (*param_1 + 0x10);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
    }
    else {
      if (param_5 == 0x115) {
        ppcVar1 = (*param_1 + 0x54);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x201) {
        ppcVar1 = (*param_1 + 0x44);
        (**ppcVar1)();
        return 0x1;
      }
      if (param_5 == 0x204) {
        ppcVar1 = (*param_1 + 0x28);
        (**ppcVar1)();
        return 0x1;
      }
    }
  }
  return 0x0;
}
