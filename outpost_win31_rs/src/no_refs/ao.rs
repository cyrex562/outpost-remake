pub fn pass1_1038_a80c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a6c8(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub fn pass1_1038_aaf0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a8cc(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub fn pass1_1038_ad4c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_abb0(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub fn pass1_1038_ae28(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_ae08(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub fn pass1_1038_be76(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0) {
    iVar2 = 0;
    paVar1 =
             mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}


pub fn pass1_1038_c410(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_be4a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub fn pass1_1038_c52a(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0) {
    iVar2 = 0;
    paVar1 =
             mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}


pub fn show_win_1038_c558(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  ShowWindow16(0x5,(struct_b_param_1 + 0x6));
  SetFocus16((struct_b_param_1 + 0x6));
  return;
}


pub fn pass1_1038_c726(StructD_32: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_c4fe(StructD_32);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(StructD_32);
  }
  return StructD_32;
}

pub fn FUN_1038_ca42()

{
  return;
}

pub fn pass1_1038_ca46(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_c80a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub fn destroy_window_1038_cc00(param_1: *mut StructC,mut param_2: u16 ,mut param_3: u32,param_4: *mut u8)

{
  let mut uVar1: u16;
  let mut iVar2: i16;

  uVar1 = param_3 - 0x1cd;
  if (uVar1 == 0) {
    iVar2 = 0x1;
  }
  else {
    uVar1 = param_3 - 0x1ce;
    if (uVar1 == 0) {
      iVar2 = 0x2;
    }
    else {
      uVar1 = param_3 - 0x1cf;
      if (uVar1 == 0) {
        iVar2 = 0x3;
      }
      else {
        uVar1 = param_3 - 0x1d0;
        if (uVar1 == 0) {
          iVar2 = 0x4;
        }
        else {
          uVar1 = param_3 - 0x1d1;
          if (uVar1 != 0) {
            post_win_msg_1040_7b3c(param_1,param_2,param_3,param_3);
            return;
          }
          iVar2 = 0x5;
        }
      }
    }
  }
  pass1_1008_eb74(param_4,(param_1 + 0x8e),iVar2);
  if (uVar1 != 0) {
    win_1008_5c7c(uVar1,param_4,_u16_1050_02a0,CONCAT22(uVar1,1));
    DestroyWindow16((param_1 + 0x6));
  }
  return;
}


pub fn pass1_1038_cc74(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_cb30(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub fn FUN_1038_ced6()

{
  return;
}

pub fn pass1_1038_ceda(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_cd5c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub fn pass1_1038_d218(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  free_proc_inst_1038_cfda(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
