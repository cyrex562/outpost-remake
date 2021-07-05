

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_0036(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  let puVar2: u32;
  astruct_18 *paVar3;
  code **ppcVar4;
  let uVar5: u16;
  astruct_449 *iVar6;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_449 *)param_1;
  *param_1 = 0x51e;
  iVar6->field_0x2 = 0x1008;
  paVar3 = *(astruct_18 **)&iVar6->field_0x8;
  uVar1 = iVar6->field_0xa;
  uVar5 = paVar3;
  if ((uVar1 | uVar5) != 0x0) {
    pass1_1008_53aa(uVar5,uVar1);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5748;
  _PTR_LOOP_1050_0298 = 0x0;
  if (_PTR_LOOP_1050_5748 != (astruct_18 *)0x0) {
    pass1_1030_8210((u16 *)_PTR_LOOP_1050_5748);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_0ed0;
  if (_PTR_LOOP_1050_0ed0 != (astruct_18 *)0x0) {
    pass1_1010_2050(_PTR_LOOP_1050_0ed0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_14cc;
  if (_PTR_LOOP_1050_14cc != (astruct_18 *)0x0) {
    pass1_1010_7efc(_PTR_LOOP_1050_14cc,0x1010);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5b7c;
  if (_PTR_LOOP_1050_5b7c != (astruct_18 *)0x0) {
    pass1_1038_af34();
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  if (_PTR_LOOP_1050_5bc8 != 0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_5bc8;
    (**ppcVar4)(param_2,_PTR_LOOP_1050_5bc8,(_PTR_LOOP_1050_5bc8 >> 0x10)
                ,0x1);
  }
  if (_PTR_LOOP_1050_02a0 != 0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_02a0;
    (**ppcVar4)(param_2,_PTR_LOOP_1050_02a0,(_PTR_LOOP_1050_02a0 >> 0x10)
                ,0x1);
  }
  puVar2 = iVar6->field_0x4;
  uVar1 = iVar6->field_0x6;
  if ((uVar1 | puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(param_2,puVar2,uVar1,0x1);
  }
  pass1_1008_9466(param_1);
  return;
}



fn pass1_1008_049c(param_1: u16,param_2: u16,char *param_3)
{
  let uVar1: u16;
  let puVar2: *mut u8;
  
  if (param_3 != 0x0) {
    uVar1 = str_op_1000_3da4(param_3);
    if (uVar1 != 0x0) {
      puVar2 = 
               pass1_1000_545a(param_3 & 0xffff0000 | (param_3 + 0x1),
                               0x105000cc);
      if (puVar2 == 0x0) {
        PTR_LOOP_1050_02ec = puVar2;
      }
    }
  }
  return;
}



fn pass1_1008_04d2(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_9466(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_04f8(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1008_0036(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_07d8(param_1: u16,bool param_2,uchar *param_3,param_4: u16) -> bool

{
  let uVar2: u16;
  let uVar1: u16;
  ulet in_AF: u8;
  let uVar3: u32;
  
  if (_PTR_LOOP_1050_5748 == 0x0) {
    uVar1 = 0x1000;
    mem_op_1000_179c(0xa,param_3,0x1000);
    uVar2 = param_3 | param_2;
    if (uVar2 != 0x0) {
      uVar1 = 0x1030;
      struct_1030_8128(CONCAT22(param_3,param_2),uVar2,param_4);
    }
    if (_PTR_LOOP_1050_5748 == 0x0) {
      debug_print_1008_6048
                (s_New_failed_in_Op__Op__Simulator_1050_0110,uVar1,param_4);
      fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
    }
    uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,uVar2,0x8);
    uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(uVar3 >> 0x10),0x8);
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(uVar3 >> 0x10),0xff);
    pass1_1030_838e(_PTR_LOOP_1050_5748,param_4,in_AF);
    param_2 = pass1_1030_8334(_PTR_LOOP_1050_5748);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_087e(param_1: u16,uchar *param_2,param_3: u16,param_4: u8)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  let local_112: u16;
  let uStack272: u16;
  let uStack6: u16;
  let puStack4: *mut u8
  
  uVar2 = 0x1000;
  mem_op_1000_179c(0xa,param_2,0x1000);
  uVar1 = param_2 | param_1;
  uStack6 = param_1;
  puStack4 = param_2;
  if (uVar1 != 0x0) {
    uVar2 = 0x1030;
    struct_1030_8128(CONCAT22(param_2,param_1),uVar1,param_3);
  }
  if (_PTR_LOOP_1050_5748 == (u32 **)0x0) {
    debug_print_1008_6048
              (s_New_failed_in_Op__Op__Simulator_1050_0130,uVar2,param_3);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,uVar1,0x8);
  pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(uVar3 >> 0x10),0x8);
  pass1_1030_532e((astruct_100 *)CONCAT22(param_3,&local_112),0xff000000,param_3,param_4);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_3,&local_112));
  pass1_1030_838e(_PTR_LOOP_1050_5748,param_3,param_4);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334(_PTR_LOOP_1050_5748);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_0932(void) -> u32

{
  let uVar1: u32;
  
  if (_PTR_LOOP_1050_14cc != 0x0) {
    pass1_1010_7fd6(_PTR_LOOP_1050_14cc);
  }
  mem_1000_0016(_PTR_LOOP_1050_03a0,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_029c,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_4fb8,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_68a2,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_5744,0x1000);
  uVar1 = mem_1000_0016(_PTR_LOOP_1050_5f2c,0x1000);
  return uVar1;
}



fn pass1_1008_0984(param_1: i16,param_2: u16,param_3: i16,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  
  set_sys_color_1008_357e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    uVar1 = (param_1 + 0xe8);
    ppcVar2 = (code **)((param_1 + 0xe8) + 0x98);
    (**ppcVar2)(param_4,uVar1,(uVar1 >> 0x10),param_3);
  }
  return;
}


fn pass1_1008_0a92(param_1: u32,short param_2)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0xee) != 0x0) {
    ppcVar1 = (code **)((iVar2 + 0xee) + 0x90);
    (**ppcVar1)(param_2,(iVar2 + 0xee));
  }
  if (*(long *)(iVar2 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((iVar2 + 0xe8) + 0x90);
    (**ppcVar1)(param_2,(iVar2 + 0xe8));
  }
  if (_PTR_LOOP_1050_0388 != 0x0) {
    ppcVar1 = (code **)*_PTR_LOOP_1050_0388;
    (**ppcVar1)(param_2,_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10)
                ,0x1);
  }
  post_quit_msg_1008_3af4(param_2);
  return;
}


fn pass1_1008_1246(param_1: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x4c);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_1272(param_1: u32,param_2: i16)
{
  code **ppcVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x88);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9cc4(param_1 & 0xffff | uVar2 << 0x10,param_2);
  return;
}



fn pass1_1008_12aa(param_1: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x8c);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9ce0();
  return;
}


fn pass1_1008_3018(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  Ulet UVar1: i32;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uStack266: u16;
  let uStack262: u32;
  char local_102 [0x100];
  
  local_102[0] = '\0';
  uStack262 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  uVar2 = (uStack262 >> 0x10);
  iVar3 = uStack262;
  UVar1 = *(ULONG *)(iVar3 + 0x12);
  uVar4 = (iVar3 + 0x14);
  uStack266 = UVar1;
  if ((uVar4 | uStack266) == 0x0) {
    pass1_1008_30cc(param_1,0x0,uVar4,param_3,param_4);
  }
  else {
    unk_str_op_1000_3d3e(CONCAT22(param_4,local_102),*(char **)(iVar3 + 0x1a));
    uVar4 = str_op_1000_3da4(CONCAT22(param_4,local_102));
    if (local_102[uVar4 - 0x1] != '\\') {
      local_102[uVar4] = '\\';
      local_102[uVar4 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(param_4,local_102),UVar1);
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc(param_1,CONCAT22(param_4,local_102),0x1000,param_4);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_30cc(param_1: u32,param_2: u16,param_3: u16,param_4: i16,param_5: u16)
{
  let puVar1: *mut u8
  let puVar2: *mut u16;
  let puVar3: *mut u8;
  let uVar4: u16;
  char local_210 [0xa];
  let local_206: [u8;100];
  let uStack262: u16;
  let uStack260: u16;
  char local_102 [0x100];
  
  local_102[0] = '\0';
  save_file_1008_3178(param_1,0x2,param_5);
  puVar1 = (param_3 | param_2);
  if (puVar1 != 0x0) {
    uStack262 = param_2;
    uStack260 = param_3;
    unk_str_op_1000_3d3e
              (CONCAT22(param_5,local_102),CONCAT22(param_3,param_2));
    str_1000_4d58(CONCAT22(param_5,local_102),0x0,0x0,
                  CONCAT22(param_5,local_206),(WNDCLASS16 *)CONCAT22(param_5,local_210));
    if (local_210[0] != '\0') {
      pass1_1000_3cea(CONCAT22(param_5,local_206),CONCAT22(param_5,local_210));
    }
    puVar3 = local_206;
    uVar4 = param_5;
    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,puVar1,param_4);
    pass1_1010_5f4c(puVar2,CONCAT22(uVar4,puVar3),
                    (puVar2 >> 0x10));
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc(param_1,CONCAT22(param_5,local_102),0x1010,param_5);
    }
  }
  return;
}


fn pass1_1008_3714(param_1: u32)
{
  pass1_1008_3e0e(param_1);
  return;
}



fn pass1_1008_372c(param_1: i16,param_2: u16) -> u32

{
  return CONCAT22(param_2,param_1 + 0xa);
}



fn pass1_1008_373c(void)
{
  return;
}



fn pass1_1008_3740(void)
{
  return;
}



fn pass1_1008_3744(void)
{
  return;
}



fn pass1_1008_3748(void)
{
  return;
}



fn pass1_1008_374c(void)
{
  return;
}



fn pass1_1008_3750(void)
{
  return;
}



fn pass1_1008_3754(void)
{
  return;
}



fn pass1_1008_3758(void) -> u16

{
  return 0x1;
}



fn pass1_1008_375e(void)
{
  return;
}



fn pass1_1008_3762(void)
{
  return;
}



fn pass1_1008_3766(void)
{
  return;
}



fn pass1_1008_377a(void)
{
  return;
}



fn pass1_1008_377e(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_37aa(param_1: *mut u16,param_2: u8) -> u16

{
  astruct_450 *uVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (astruct_450 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_37e4(param_1: u32,param_2: u8) -> u32

{
  cleanup_ui_op_1008_0618(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_392e(param_1: *mut u16,param_2: u16) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa8;
  (iVar1 + 0x2) = 0x1008;
  (iVar1 + 0x4) = param_2;
  *param_1 = 0x3ab0;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa0;
  (iVar1 + 0x2) = 0x1008;
  return param_1;
}



fn pass1_1008_397a(param_1: *mut u16)
{
  astruct_452 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_452 *)param_1;
  *param_1 = 0x3aa0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}


fn pass1_1008_3a10(void)
{
  return;
}



fn pass1_1008_3a14(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_3a40(param_1: *mut u16,param_2: u8) -> u16

{
  astruct_451 *uVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (astruct_451 *)param_1;
  *param_1 = 0x3ab0;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_3a7a(param_1: u32,param_2: u8) -> u32

{
  pass1_1008_397a((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_20 *  pass1_1008_3ab8(astruct_20 *param_1)

{
  let iVar1: i16;
  let uVar2: u16;
  
  set_struct_1008_687a(param_1,0x0);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xde) = 0x0;
  param_1->field_0x0 = 0x3b46;
  (iVar1 + 0x2) = 0x1008;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar1 + 0x5b)),
             s_SOLDefaultWindowClass_1050_01fe);
  return param_1;
}



fn pass1_1008_3afe(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



void 
pass1_1008_3bd6(astruct_160 *param_1,param_2: u16,param_3: u16,param_4: u32,
               param_5: u16,param_6: u32,param_7: u32,param_8: u16,param_9: u16)

{
  mixed_struct_op_1040_8fb8
            ((u16 *)CONCAT22(param_2,param_1),param_3,0x0,param_5,
             param_6,(param_6 >> 0x10),param_7,
             (param_7 >> 0x10),param_8,&PTR_LOOP_1050_1040,param_9);
  CONCAT22(param_2,param_1) = 0x3cfc;
  param_1->field_0x2 = 0x1008;
  param_1->field_0x36 = 0x0;
  param_1->field_0x26 = 0x0;
  pass1_1040_9252(CONCAT22(param_2,param_1),&PTR_LOOP_1050_1040);
  create_window_1040_92dc(CONCAT22(param_2,param_1),&PTR_LOOP_1050_1040);
  mov_update_win_1040_93aa
            ((astruct_65 *)CONCAT22(param_2,param_1),param_4,
             (param_4 >> 0x10),&PTR_LOOP_1050_1040);
  return;
}


fn pass1_1008_3cd6(param_1: *mut u16,param_2: u8) -> u16

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass_1008_3d44(param_1: *mut u16,param_2: u8) -> u16

{
  astruct_453 *uVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (astruct_453 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_3e0e(param_1: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0x4) + 0x4);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_3e38(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x0;
  (param_1 + 0x2) = 0x0;
  (param_1 + 0x4) = 0x0;
  return param_1;
}



fn pass1_1008_3e54(param_1: *mut u16,param_2: u16,param_3: u16,param_4: u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = param_4;
  (param_1 + 0x2) = param_3;
  (param_1 + 0x4) = param_2;
  return param_1;
}



fn pass1_1008_3e76(param_1: *mut u16,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = param_4;
  (param_1 + 0x2) = param_3;
  (param_1 + 0x4) = param_2;
  return;
}



fn pass1_1008_3e94(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)
{
  *param_3 = *param_1;
  *param_2 = (param_1 + 0x2);
  return;
}



fn pass1_1008_3eb4(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16,param_4: *mut u16)
{
  let uVar1: u16;
  
  *param_4 = *param_1;
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x2);
  *param_2 = (param_1 + 0x4);
  return;
}



fn pass1_1008_3ee2(i16 *param_1,i16 *param_2)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  
  iVar1 = *param_2 - *param_1;
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *param_1 = iVar1 + 0x1;
  uVar3 = (param_2 >> 0x10);
  uVar4 = (param_1 >> 0x10);
  iVar2 = param_1;
  iVar1 = (param_2 + 0x2) - (iVar2 + 0x2);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  (iVar2 + 0x2) = iVar1 + 0x1;
  iVar1 = (param_2 + 0x4) - (iVar2 + 0x4);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  (iVar2 + 0x4) = iVar1 + 0x1;
  return;
}



fn pass1_1008_3f32(i16 *param_1,i16 *param_2)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  
  *param_1 = *param_1 + *param_2;
  uVar2 = (param_2 >> 0x10);
  uVar3 = (param_1 >> 0x10);
  piVar1 = (i16 *)(param_1 + 0x2);
  *piVar1 = *piVar1 + (param_2 + 0x2);
  piVar1 = (i16 *)(param_1 + 0x4);
  *piVar1 = *piVar1 + (param_2 + 0x4);
  return;
}



fn pass1_1008_3f62(param_1: *mut u16,param_2: *mut u16)
{
  let uVar1: u16;
  let uVar2: u16;
  
  *param_1 = *param_2;
  uVar1 = (param_2 >> 0x10);
  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x2) = (param_2 + 0x2);
  (param_1 + 0x4) = (param_2 + 0x4);
  return;
}


fn pass1_1008_4016(astruct_76 *param_1)
{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_op_1008_56b4(param_1);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x6) = 0x0;
  (iVar1 + 0xa) = 0x0;
  (iVar1 + 0xe) = 0x0;
  (iVar1 + 0x10) = 0x0;
  (iVar1 + 0x14) = 0x0;
  (iVar1 + 0x18) = 0x0;
  (iVar1 + 0x1c) = 0x0;
  param_1->field_0x0 = &PTR_LOOP_1050_48de;
  (iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_405c(astruct_76 *param_1,param_2: u32,param_3: i16,param_4: i16)
{
  let uVar1: u32;
  sqword sVar2;
  let iVar3: i16;
  let lVar4: i32;
  let puVar5: *mut u8
  astruct_76 *iVar4;
  let uVar6: u16;
  let uStack10: u32;
  
  struct_op_1008_56b4(param_1);
  uVar6 = (param_1 >> 0x10);
  iVar4 = (astruct_76 *)param_1;
  &iVar4->field_0x6 = 0x0;
  (&iVar4->field_0x8 + 0x2) = 0x0;
  &iVar4->field_0xe = 0x0;
  (&iVar4->field_0xe + 0x2) = 0x0;
  iVar4->field_0x14 = 0x0;
  &iVar4->field_0x18 = 0x0;
  iVar4->field_0x1c = 0x0;
  param_1->field_0x0 = &PTR_LOOP_1050_48de;
  iVar4->field_0x2 = 0x1008;
  iVar3 = param_4 * 0x8 + 0x1f;
  iVar3 = ((iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  uStack10 = SEXT24(param_3);
  lVar4 = (long)iVar3 * (long)param_3 + 0x436;
  lVar4 = mem_op_1000_0a48(0x1,lVar4,(lVar4 >> 0x10),_PTR_LOOP_1050_5f2c
                           ,0x1000);
  iVar4->field_0x6 = lVar4;
  &iVar4->field_0x8 = (lVar4 >> 0x10);
  pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar6 << 0x10));
  &iVar4->field_0x18 = iVar3;
  iVar4->field_0x1a = iVar3 >> 0xf;
  (&iVar4->field_0xe + 0x2) = 0x28;
  uVar1 = (&iVar4->field_0xe + 0x2);
  *(long *)(uVar1 + 0x4) = (long)param_4;
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0x8) = uStack10;
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0xc) = 0x1;
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0xe) = 0x8;
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0x10) = 0x0;
  sVar2 = (qword)&iVar4->field_0x18 * (qword)uStack10;
  puVar5 = ((qword)sVar2 >> 0x20);
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0x14) = (long)sVar2;
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0x20) = 0x100;
  uVar1 = (&iVar4->field_0xe + 0x2);
  (uVar1 + 0x24) = 0x100;
  pass1_1008_4834(param_1);
  pass1_1008_4d84(*(astruct_90 **)(&iVar4->field_0x8 + 0x2),param_2,puVar5);
  return;
}



fn pass1_1008_41bc(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let lVar3: i32;
  code **ppcVar4;
  astruct_288 *iVar5;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_288 *)param_1;
  *param_1 = &PTR_LOOP_1050_48de;
  iVar5->field_0x2 = 0x1008;
  puVar1 = iVar5->field_0xa;
  uVar2 = iVar5->field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  if (iVar5->field_0x6 != 0x0) {
    lVar3 = iVar5->field_0x6;
    call_fn_ptr_1000_0dc6(lVar3,(lVar3 >> 0x10),0x1000);
  }
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  return;
}


fn pass1_1008_431c(param_1: u32,param_2: u8)
{
  let puVar1: u32;
  let uVar2: u32;
  let bVar3: bool;
  let uVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  let uStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (*(long *)(iVar5 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar6 << 0x10));
  }
  if (((iVar5 + 0x8) | (iVar5 + 0x6)) == 0x0) {
    bVar3 = false;
  }
  else {
    if (((iVar5 + 0xc) | (iVar5 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar6 << 0x10));
    }
    bVar3 = true;
  }
  if (bVar3) {
    if (((iVar5 + 0x16) | (iVar5 + 0x14)) == 0x0) {
      return;
    }
    uStack6 = 0x0;
    while( true ) {
      uVar2 = (iVar5 + 0x10);
      puVar1 = (uVar2 + 0x8);
      if (*puVar1 == uStack6 || (long)*puVar1 < (long)uStack6) break;
      uVar4 = uStack6;
      pass1_1008_4544(param_1);
      uVar2 = (iVar5 + 0x10);
      pass1_1000_4906((astruct_20 *)(uVar4 & 0xffff | uStack6._2_2_ << 0x10),
                      (WNDCLASS16 *)param_2,(uVar2 + 0x4));
      uStack6 += 0x1;
    }
  }
  return;
}



fn pass1_1008_43cc(param_1: u32) -> u32

{
  let bVar1: bool;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22((iVar2 + 0x16),(iVar2 + 0x14));
}



fn pass1_1008_4426(param_1: u32) -> u32

{
  let bVar1: bool;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22((iVar2 + 0xc),(iVar2 + 0xa));
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1008_4480(param_1: u32,param_2: *mut u16,astruct_76 *param_3,param_4: u16)
{
  let iVar1: i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let iStack26: i16;
  char *pcStack24;
  char *pcStack20;
  let iStack16: i16;
  let local_6: i16;
  let local_4: [u8;2];
  
  pass1_1008_3e94(param_2,CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,local_4));
  uVar6 = pass1_1008_4772(param_3);
  uVar4 = (uVar6 >> 0x10);
  iVar1 = (uVar6 + 0x4);
  iVar2 = (uVar6 + 0x8);
  for (iStack16 = 0x0; iStack16 < iVar2; iStack16 += 0x1) {
    uVar5 = local_6 >> 0xf;
    iVar3 = local_6;
    local_6 = local_6 + 0x1;
    pass1_1008_4544(param_1);
    pcStack20 = CONCAT22(uVar5,iVar3);
    uVar6 = SEXT24(iStack16);
    pass1_1008_4544(param_3);
    pcStack24 = (uVar6 & 0xffff | uVar5 << 0x10);
    iStack26 = iVar1;
    while (iStack26 != 0x0) {
      if (*pcStack24 != -0x1) {
        *pcStack20 = *pcStack24;
      }
      pcStack24 = CONCAT22((pcStack24 >> 0x10) +
                                   (-(0xfffe < pcStack24) & 0x6c),
                                   pcStack24 + 0x1);
      pcStack20 = CONCAT22((pcStack20 >> 0x10) +
                                   (-(0xfffe < pcStack20) & 0x6c),
                                   pcStack20 + 0x1);
      iStack26 = iStack26 + -0x1;
    }
  }
  return;
}



fn pass1_1008_4544(param_1: u32)
{
  let bVar1: bool;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  return;
}



fn pass1_1008_4772(astruct_76 *param_1) -> u32

{
  let bVar1: bool;
  astruct_76 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(long *)&iVar2->field_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar2 << 0x10));
  }
  if (*(long *)&iVar2->field_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(&iVar2->field_0x8 + 0x2) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2->field_0x12,(&iVar2->field_0xe + 0x2));
}



fn pass1_1008_47cc(astruct_76 *param_1)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let iVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uStack14: u32;
  let iVar4: i16;
  
  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (*(long *)(iVar5 + 0x6) != 0x0) {
    uVar1 = (iVar5 + 0x6);
    iVar6 = (iVar5 + 0x8);
    iVar4 = uVar1;
    uVar3 = iVar4 + 0xe;
    (iVar5 + 0x10) = uVar1 & 0xffff0000 | uVar3;
    (iVar5 + 0x14) = iVar4 + 0x436;
    (iVar5 + 0x16) = iVar6 + (-(0xfbd7 < uVar3) & 0x6c);
    uVar2 = (iVar5 + 0x10);
    uVar8 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    uStack14 = (iVar6 + 0xe);
    *(long *)(iVar5 + 0x18) =
         (long)(uStack14 * *(long *)(iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
  }
  return;
}



fn pass1_1008_4834(astruct_76 *param_1)
{
  code **ppcVar1;
  let puVar2: u32;
  let uVar3: u32;
  let extraout_DX: *mut u8
  let puVar4: *mut u8
  let extraout_DX_00: u16;
  astruct_76 *struct_var5_1;
  astruct_76 *struct_var5;
  astruct_76 *paStack10;
  
  struct_var5 = (astruct_76 *)(param_1 >> 0x10);
  struct_var5_1 = (astruct_76 *)param_1;
  puVar2 = (&struct_var5_1->field_0x8 + 0x2);
  puVar4 = struct_var5_1->field_0xc;
  if ((puVar4 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x14,puVar4,0x1000);
  paStack10 = (astruct_76 *)CONCAT22(puVar4,puVar2);
  if ((puVar4 | puVar2) != 0x0) {
    uVar3 = (&struct_var5_1->field_0xe + 0x2);
    uVar3 = uVar3 & 0xffff0000 | (uVar3 + 0x28);
    struct_op_1008_4c98(paStack10,uVar3,0x100);
    (&struct_var5_1->field_0x8 + 0x2) = uVar3;
    struct_var5_1->field_0xc = extraout_DX_00;
    return;
  }
  (&struct_var5_1->field_0x8 + 0x2) = 0x0;
  return;
}



fn pass1_1008_48aa(param_1: u32) -> u16

{
  return (param_1 + 0xe);
}



fn pass1_1008_48b8(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10084942) overlaps instruction at (ram,0x10084941)
// 

void 
pass1_1008_48de(param_1: u16,param_2: u32,param_3: i16,param_4: u16,param_5: *mut u16,
               param_6: i16,param_7: i16,param_8: *mut u8,param_9: u16,param_10: u16,
               char param_11,param_12: u16,param_13: u8)

{
  byte *pbVar1;
  let uVar2: u32;
  let bVar3: u8;
  let uVar4: u16;
  let bVar5: u8;
  let uVar6: u16;
  let puVar7: *mut u8;
  let iVar8: i16;
  let uVar9: u16;
  
  uVar6 = param_4 & 0xff |
          (byte)((char)(param_4 >> 0x8) + param_4 + param_11) << 0x8;
  puVar7 = (param_6 + 0x1);
  pbVar1 = (byte *)(param_5 + param_7);
  bVar5 = (byte)(param_4 & 0xff);
  *pbVar1 = *pbVar1 | bVar5;
  bVar3 = in(0x46);
  pbVar1 = (byte *)(param_5 + param_7);
  *pbVar1 = *pbVar1 | bVar5;
  if (param_3 == 0x1) {
    pbVar1 = (byte *)(param_5 + param_7);
    *pbVar1 = *pbVar1 | bVar5;
    iVar8 = param_7 + 0x1;
    pbVar1 = (byte *)(param_5 + iVar8);
    bVar5 = (byte)param_12;
    *pbVar1 = *pbVar1 | bVar5;
    pbVar1 = (byte *)(param_5 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    *param_8 = bVar3;
    pbVar1 = (byte *)(param_5 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    uVar6 = param_12;
    if (*pbVar1 != 0x0) {
      pbVar1 = (byte *)(param_5 + iVar8);
      *pbVar1 = *pbVar1 | bVar5;
      puVar7 = (&param_12 + 0x1);
      param_5 = (param_2 >> 0x8);
      CONCAT13(param_13,param_2._1_3_) = 0x389a;
      param_5[0x1] = 0x1008;
      param_9 = (CONCAT13(param_13,param_2._1_3_) >> 0x10);
      (param_5 + 0x2) = 0x0;
      (param_5 + 0x4) = 0x0;
      param_5[0x6] = 0xffff;
      (param_5 + 0x7) = 0x0;
      (param_5 + 0x9) = 0x0;
      (param_5 + 0xb) = 0x0;
      (param_5 + 0xd) = 0x0;
      param_5[0xf] = 0x0;
    }
  }
  else {
    param_5[0x11] = bVar3 | 0x800;
  }
  param_5[0x11] = (puVar7 + 0xa);
  *param_5 = &PTR_LOOP_1050_4c4c;
  param_5[0x1] = 0x1008;
  uVar4 = str_op_1008_60e8(*(char **)(puVar7 + 0xc),uVar6);
  uVar2 = (puVar7 + 0x6);
  uVar9 = (uVar2 >> 0x10);
  iVar8 = uVar2;
  (iVar8 + 0x8) = uVar4;
  (iVar8 + 0xa) = uVar6;
  return;
}


fn pass1_1008_4b5e(param_1: *mut u32) -> u32

{
  code **ppcVar1;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x1e) == 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x8);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0x0) {
      return 0x0;
    }
  }
  return CONCAT22((iVar3 + 0x6),(iVar3 + 0x4));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_4b8e(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u16;
  let iStack18: i16;
  let iStack16: i16;
  let uStack10: i16;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,param_2,param_3);
  uVar2 = (puVar3 >> 0x10);
  uVar1 = (puVar3 + 0x18);
  iStack18 = (puVar3 + 0x16) / 0x2;
  for (iStack16 = 0x0; iStack10 = uVar1, uVar2 = (param_1 >> 0x10),
      iStack16 < iStack18; iStack16 += 0x1) {
    pass1_1008_4d26((param_1 + 0x4),

                    (uVar1 & 0xffff0000 | (iStack16 * 0x4 + iStack10)),
                    iStack16);
  }
  for (iStack18 = 0x100 - iStack18; iStack18 < 0x100; iStack18 += 0x1) {
    pass1_1008_4d26((param_1 + 0x4),

                    (uVar1 & 0xffff0000 | (iStack16 * 0x4 + iStack10)),
                    iStack18);
    iStack16 += 0x1;
  }
  return;
}


fn pass1_1008_4cdc(param_1: *mut u16)
{
  astruct_454 *iVar2;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar2 = (astruct_454 *)param_1;
  *param_1 = 0x4f1c;
  iVar2->field_0x2 = 0x1008;
  fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0xe,0x1000);
  if (iVar2->field_0x12 != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0x4,0x1000);
  }
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}



fn pass1_1008_4d26(param_1: u32,param_2: *mut u16,param_3: i16) -> u16

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let lVar3: i32;
  astruct_650 *iVar5;
  astruct_649 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_650 *)param_1;
  if (((iVar5->field_0x4 != 0x0) && (-0x1 < param_3)) &&
     (piVar1 = &iVar5->field_0xc, *piVar1 != param_3 && param_3 <= *piVar1)) {
    uVar2 = (param_2 + 0x2);
    lVar3 = iVar5->field_0x4;
    uVar4 = (lVar3 >> 0x10);
    iVar4 = (astruct_649 *)lVar3;
    (iVar4 + param_3 * 0x4) = *param_2;
    (iVar4 + param_3 * 0x4 + 0x2) = uVar2;
    return 0x1;
  }
  return 0x0;
}



fn pass1_1008_4d72(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x6),(param_1 + 0x4))
  ;
}



fn pass1_1008_4d84(astruct_90 *param_1,param_2: u32,uchar *param_3)
{
  let uVar1: u16;
  astruct_90 *iVar3;
  let uVar2: u16;
  let uVar3: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_90 *)param_1;
  uVar3 = (param_2 >> 0x10);
  if (iVar3->field_0x12 != 0x0) {
    iVar3->field_0xc = (param_2 + 0xc);
    fn_ptr_1000_17ce((astruct_18 *)iVar3->field_0x4,0x1000);
    iVar3->field_0x4 = 0x0;
    uVar1 = iVar3->field_0xc << 0x2;
    mem_op_1000_179c(uVar1,param_3,0x1000);
    &iVar3->field_0x4 = uVar1;
    *(uchar **)(&iVar3->field_0x4 + 0x2) = param_3;
  }
  if (iVar3->field_0xc != 0x100) {
    return;
  }
  pass1_1000_48a8(iVar3->field_0x4,(param_2 + 0x4),0x400);
  return;
}


fn pass1_1008_4ef6(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_4cdc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_5068(astruct_76 *param_1,astruct_83 *param_2)
{
  struct_op_1008_4214(param_1,param_2);
  return;
}



fn pass1_1008_507c(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1008_50c2(astruct_110 *param_1,param_2: u32,param_3: u32,param_4: *mut u16,
               param_5: u32)

{
  astruct_110 *iVar1;
  let uVar1: u16;
  
  param_1->field_0x0 = *param_4;
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_110 *)param_1;
  iVar1->field_0x2 = (param_4 + 0x2);
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = param_5;
  iVar1->field_0x10 = 0x0;
  pass1_1008_52fc((u16 *)(param_1 & 0xffff | uVar1 << 0x10));
  return;
}



fn pass1_1008_5118(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x10) != 0x0) {
    uVar1 = (param_1 + 0x10);
    call_fn_ptr_1000_0dc6(uVar1,(uVar1 >> 0x10),0x1000);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_5134(param_1: u32)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  let lVar4: i32;
  let iVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  let iStack16: i16;
  let lStack14: i32;
  let uStack10: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  lVar4 = *(long *)(iVar6 + 0x4) * *(long *)(iVar6 + 0x8);
  lVar4 = mem_op_1000_0a48(0x1,lVar4,(lVar4 >> 0x10),_PTR_LOOP_1050_5f2c
                           ,0x1000);
  uVar3 = (lVar4 >> 0x10);
  (iVar6 + 0x10) = lVar4;
  (iVar6 + 0x12) = uVar3;
  if ((uVar3 | (iVar6 + 0x10)) == 0x0) {
    return;
  }
  iVar5 = (iVar6 + 0x8);
  iVar2 = (iVar6 + 0xa);
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0x0),iVar5 + -0x1) * *(long *)(iVar6 + 0x4);
  puVar1 = (iVar6 + 0x10);
  uVar3 = lVar4;
  uStack10 = CONCAT22(((lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100
                      + (iVar6 + 0x12),uVar3 + *puVar1);
  lStack14 = CONCAT22(iVar2,iVar5);
  iStack16 = (iVar6 + 0x2);
  while (lStack14 != 0x0) {
    iVar2 = iStack16 + 0x1;
    iVar5 = iStack16 >> 0xf;
    pass1_1008_4544((iVar6 + 0xc));
    pass1_1000_48a8(uStack10,CONCAT22(iVar5,iStack16),(iVar6 + 0x4));
    iVar5 = (iVar6 + 0x4);
    uVar3 = -iVar5;
    uStack10 = CONCAT22((uStack10 >> 0x10) +
                        (CARRY2(uStack10,uVar3) -
                        ((iVar6 + 0x6) + (iVar5 != 0x0))) * 0x100,
                        uStack10 + uVar3);
    iStack16 = iVar2;
    lStack14 = lStack14 + -0x1;
  }
  return;
}



fn pass1_1008_5236(param_1: u32)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  let lVar4: i32;
  let iVar5: i16;
  astruct_109 *iVar6;
  let uVar6: u16;
  let bVar7: bool;
  let iStack12: i16;
  let lStack10: i32;
  let uStack6: u16;
  let iStack4: i16;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_109 *)param_1;
  iVar5 = iVar6->field_0x8;
  iVar2 = iVar6->field_0xa;
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0x0),iVar5 + -0x1) * *(long *)&iVar6->field_0x4
  ;
  puVar1 = &iVar6->field_0x10;
  uVar3 = lVar4;
  uStack6 = uVar3 + *puVar1;
  iStack4 = ((lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 +
            iVar6->field_0x12;
  lStack10 = CONCAT22(iVar2,iVar5);
  iStack12 = iVar6->field_0x2;
  while (lStack10 != 0x0) {
    iVar2 = iStack12 + 0x1;
    iVar5 = iStack12 >> 0xf;
    pass1_1008_4544(iVar6->field_0xc);
    pass1_1000_48a8(CONCAT22(iVar5,iStack12),CONCAT22(iStack4,uStack6),
                    &iVar6->field_0x4);
    iVar5 = &iVar6->field_0x4;
    uVar3 = -iVar5;
    bVar7 = CARRY2(uStack6,uVar3);
    uStack6 += uVar3;
    iStack4 += (bVar7 - (iVar6->field_0x6 + (iVar5 != 0x0))) * 0x100;
    iStack12 = iVar2;
    lStack10 = lStack10 + -0x1;
  }
  return;
}



fn pass1_1008_52fc(param_1: *mut u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  let lVar4: i32;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  astruct_111 *iVar8;
  let uVar8: u16;
  let uVar9: u32;
  let uStack14: u16;
  let iStack12: i16;
  
  uVar8 = (param_1 >> 0x10);
  iVar8 = (astruct_111 *)param_1;
  uVar9 = pass1_1008_4772(iVar8->field_0xc);
  uVar5 = (uVar9 >> 0x10);
  iVar7 = uVar9;
  iVar6 = (iVar7 + 0x4);
  uVar3 = iVar6 - 0x1;
  iVar6 = (iVar7 + 0x6) - (iVar6 == 0x0);
  lVar4 = *(long *)(iVar7 + 0x8) + -0x1;
  uVar2 = *param_1;
  puVar1 = &iVar8->field_0x4;
  iVar7 = (uVar2 >> 0xf) + iVar8->field_0x6 + CARRY2(uVar2,*puVar1);
  if ((iVar6 <= iVar7) && ((iVar6 < iVar7 || (uVar3 < uVar2 + *puVar1)))) {
    iVar8->field_0x4 = uVar3 - uVar2;
    iVar8->field_0x6 = (iVar6 - (uVar2 >> 0xf)) - (uVar3 < uVar2);
  }
  uVar2 = iVar8->field_0x2;
  puVar1 = &iVar8->field_0x8;
  iVar6 = (uVar2 >> 0xf) + iVar8->field_0xa + CARRY2(uVar2,*puVar1);
  iStack12 = (lVar4 >> 0x10);
  if ((iStack12 <= iVar6) &&
     ((uStack14 = lVar4, iStack12 < iVar6 || (uStack14 < uVar2 + *puVar1)))) {
    iVar8->field_0x8 = uStack14 - uVar2;
    iVar8->field_0xa = (iStack12 - (uVar2 >> 0xf)) - (uStack14 < uVar2);
  }
  return;
}



fn pass1_1008_5394(param_1: *mut u32) -> u32

{
  *param_1 = 0x0;
  return param_1;
}



fn pass1_1008_53aa(void)
{
  return;
}


fn pass1_1008_570e(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_5784(param_1: *mut u32,param_2: u32)
{
  *param_1 = param_2;
  (param_1 + 0x4) = 0x0;
  return;
}



fn pass1_1008_57a4(param_1: *mut u32,param_2: u32)
{
  *param_1 = param_2;
  (param_1 + 0x4) = 0x0;
  return;
}



fn pass1_1008_57c4(param_1: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x5bc4;
  (param_1 + 0x2) = 0x1008;
  pass1_1008_5830(param_1 & 0xffff | uVar1 << 0x10);
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  return;
}



long  pass1_1008_57f0(param_1: u32,param_2: i16,param_3: u16)

{
  let bVar1: bool;
  let lVar2: i32;
  let iStack12: i16;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_3,local_a),param_1);
  iStack12 = 0x0;
  do {
    lVar2 = pass1_1008_5b12(local_a,param_3);
    if (lVar2 == 0x0) {
      return 0x0;
    }
    bVar1 = iStack12 != param_2;
    iStack12 = iStack12 + 0x1;
  } while (bVar1);
  return lVar2;
}



fn pass1_1008_5830(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u32;
  let puVar5: u32;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  
  while( true ) {
    uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (*(long *)(iVar6 + 0x4) == 0x0) break;
    if ((iVar6 + 0xa) != 0x0) {
      uVar4 = (iVar6 + 0x4);
      uVar9 = (uVar4 >> 0x10);
      iVar7 = uVar4;
      puVar1 = (iVar7 + 0x8);
      uVar2 = (iVar7 + 0xa);
      if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
      }
    }
    puVar5 = *(long *)(iVar6 + 0x4);
    (iVar6 + 0x4) = (puVar5 + 0x4);
    if (puVar5 != 0x0) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
    }
  }
  (iVar6 + 0x8) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_58a6(param_1: u32,param_2: u32)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  astruct_99 *paStack6;
  
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_029c);
  uVar3 = (paStack6 >> 0x10);
  uVar2 = paStack6;
  if ((uVar3 | uVar2) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    (uVar2 + 0x2) = 0x1008;
    (uVar2 + 0x4) = 0x0;
    (uVar2 + 0x8) = 0x0;
    paStack6->field_0x0 = 0x5bc0;
    (uVar2 + 0x2) = 0x1008;
  }
  if (paStack6 == (astruct_99 *)0x0) {
    return;
  }
  uVar5 = (paStack6 >> 0x10);
  (paStack6 + 0x8) = param_2;
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  (paStack6 + 0x4) = (iVar4 + 0x4);
  *(astruct_99 **)(iVar4 + 0x4) = paStack6;
  piVar1 = (i16 *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_593c(param_1: *mut u32,param_2: u32)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  astruct_99 *paStack6;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x8) == 0x0) {
    ppcVar2 = (code **)(*param_1 + 0x4);
    (**ppcVar2)();
    return;
  }
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_029c);
  uVar4 = (paStack6 >> 0x10);
  uVar3 = paStack6;
  if ((uVar4 | uVar3) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    (uVar3 + 0x2) = 0x1008;
    (uVar3 + 0x4) = 0x0;
    (uVar3 + 0x8) = 0x0;
    paStack6->field_0x0 = 0x5bc0;
    (uVar3 + 0x2) = 0x1008;
  }
  if (paStack6 == (astruct_99 *)0x0) {
    return;
  }
  (paStack6 + 0x8) = param_2;
  do {
    param_1 = *(u32 **)(param_1 + 0x4);
    uVar7 = (param_1 >> 0x10);
  } while (*(long *)(param_1 + 0x4) != 0x0);
  *(astruct_99 **)(param_1 + 0x4) = paStack6;
  piVar1 = (i16 *)(iVar5 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



fn pass1_1008_59f4(param_1: u32,param_2: i32)
{
  let piVar1: *mut i16;
  let puVar2: u32;
  let uVar3: u16;
  let puVar4: u32;
  code **ppcVar5;
  let puVar6: u32;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u16;
  let uStack10: u16;
  let puStack6: u32;
  
  puStack6 = 0x0;
  uVar9 = (param_1 >> 0x10);
  puVar6 = puStack6;
  puVar4 = param_1;
  do {
    puStack6 = puVar6;
    uVar10 = (puVar4 >> 0x10);
    iVar8 = puVar4;
    puVar4 = *(long *)(iVar8 + 0x4);
    uStack10 = puVar4;
    uVar11 = (puVar4 >> 0x10);
    if (((iVar8 + 0x6) | uStack10) == 0x0) break;
    puVar6 = puVar4;
  } while (*(long *)(uStack10 + 0x8) != param_2);
  if (puVar4 != 0x0) {
    if (puStack6 == 0x0) {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
      puStack6 = param_1;
    }
    else {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
    }
    uVar12 = (puStack6 >> 0x10);
    (puStack6 + 0x4) = uVar10;
    (puStack6 + 0x6) = uVar7;
    if ((param_1 + 0xa) != 0x0) {
      puVar2 = (uStack10 + 0x8);
      uVar3 = (uStack10 + 0xa);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar5 = (code **)*puVar2;
        (**ppcVar5)();
      }
    }
    if (puVar4 != 0x0) {
      ppcVar5 = (code **)*puVar4;
      (**ppcVar5)();
    }
    piVar1 = (i16 *)(param_1 + 0x8);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  return;
}



fn pass1_1008_5ab8(param_1: u32)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let puVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x4) == 0x0) {
    return;
  }
  puVar3 = (iVar4 + 0x4);
  uVar6 = (puVar3 >> 0x10);
  (iVar4 + 0x4) = (puVar3 + 0x4);
  if ((uVar6 | puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
  }
  piVar1 = (i16 *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + -0x1;
  return;
}



fn pass1_1008_5b12(long *param_1)
{
  let uVar1: u32;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  
  if ((*param_1 != 0x0) && ((*param_1 + 0x8) != 0x0)) {
    uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (*(long *)(iVar2 + 0x4) == 0x0) {
      uVar5 = (*param_1 >> 0x10);
      iVar3 = *param_1;
    }
    else {
      uVar1 = (iVar2 + 0x4);
      uVar5 = (uVar1 >> 0x10);
      iVar3 = uVar1;
    }
    (iVar2 + 0x4) = (iVar3 + 0x4);
    if (*(long *)(iVar2 + 0x4) != 0x0) {
      return;
    }
  }
  return;
}



fn pass1_1008_5b6e(param_1: *mut u16,param_2: u8) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  ((i16 *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((i16 *)param_1,uVar1,0x1000);
  }
  return param_1;
}



fn pass1_1008_5b9a(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_5bdc(astruct_79 *param_1,param_2: i16,param_3: u16)
{
  astruct_651 *puVar1;
  let uVar1: u16;
  astruct_79 *paVar2;
  let puVar3: *mut u16;
  
  paVar2 = struct_op_1010_1d48(param_1,0x44);
  uVar1 = (param_1 >> 0x10);
  puVar1 = (astruct_651 *)param_1;
  puVar1->field_0xa = 0x0;
  &puVar1->field_0xc = 0x0;
  puVar1->field_0x10 = 0x0;
  puVar1->field_0x12 = 0x0;
  param_1->field_0x0 = 0x5fc8;
  puVar1->field_0x2 = 0x1008;
  _PTR_LOOP_1050_02a0 = param_1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,
                           (paVar2 >> 0x10),param_2);
  puVar1->field_0xc = puVar3;
  puVar1->field_0xe = (puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_5c34(param_1: *mut u16)
{
  let unaff_SS: u16;
  
  *param_1 = 0x5fc8;
  (param_1 + 0x2) = 0x1008;
  _PTR_LOOP_1050_02a0 = 0x0;
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}


fn pass1_1008_5fa2(param_1: u32,param_2: u8) -> u32

{
  pass1_1008_5c34((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar *  pass1_1008_5fd8(param_1: u16,uchar *param_2)

{
  let piVar1: *mut i16;
  char *pcVar2;
  let puStack10: *mut u8
  let puStack8: *mut u8
  let iStack6: i16;
  
  puStack10 = &stack0x0006;
  _iStack6 = (i16 *)CONCAT22(param_1,puStack10);
  mem_op_1000_179c(0x1000,param_2,0x1000);
  puStack8 = param_2;
  pcVar2 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  unk_str_op_1000_3d3e(CONCAT22(puStack8,puStack10),pcVar2);
  while( true ) {
    piVar1 = _iStack6;
    _iStack6 = (i16 *)(_iStack6 & 0xffff0000 | (iStack6 + 0x2));
    if (*piVar1 == 0x0) break;
    pcVar2 = load_string_1010_847e
                       (_PTR_LOOP_1050_14cc,
                        (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    pass1_1000_3cea(CONCAT22(puStack8,puStack10),(ULONG)pcVar2);
  }
  return puStack10;
}


fn pass1_1008_612e(param_1: i16,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let lVar3: i32;
  let iVar4: i16;
  let iStack18: i16;
  let iStack16: i16;
  
  uVar1 = pass1_1000_4d24();
  uVar2 = (param_2 - param_1) + 0x1;
  if ((uVar2 >> 0xf | uVar2) == 0x0) {
    return;
  }
  iStack16 = 0x1;
  iStack18 = param_1;
  do {
    if (param_2 < iStack18) {
      return;
    }
    lVar3 = (long)iStack16 * (long)(0x7fff / (sqword)(long)uVar2);
    iVar4 = (lVar3 >> 0x10);
    if (uVar1 >> 0xf <= iVar4) {
      if (uVar1 >> 0xf < iVar4) {
        return;
      }
      if (uVar1 <= lVar3) {
        return;
      }
    }
    iStack18 += 0x1;
    iStack16 += 0x1;
  } while( true );
}


fn pass1_1008_6330(param_1: *mut u16,param_2: u8)
{
  astruct_456 *uVar1;
  let uVar2: u16;
  
  uVar1 = (astruct_456 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}


fn pass1_1008_64a2(param_1: *mut u16)
{
  let uVar1: u16;
  code **ppcVar2;
  
  uVar1 = (param_1 + 0x2);
  if ((uVar1 | *param_1) != 0x0) {
    ppcVar2 = (code **)*param_1;
    (**ppcVar2)();
  }
  return;
}



fn pass1_1008_64c8(param_1: *mut u32,param_2: u32,param_3: i16,param_4: u16,uchar *param_5)
{
  let iVar1: i16;
  let iVar2: i16;
  let extraout_DX: u16;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let iStack8: i16;
  let uStack6: u32;
  
  if (*param_1 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_5,0x1000);
  if ((param_5 | param_4) == 0x0) {
    param_4 = 0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1008_6604((astruct_85 *)CONCAT22(param_5,param_4),param_2,
                        (param_2 >> 0x10));
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_4);
  iStack8 = 0x0;
  while (param_2 = param_2 & 0xffff0000 | (param_2 - 0x1), param_2 != 0x0
        ) {
    iVar1 = param_3 + 0x1;
    iVar4 = param_3 >> 0xf;
    pass1_1008_4544(*param_1);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(uStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_3),param_2._2_2_);
    param_3 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



fn pass1_1008_6562(param_1: *mut u32,param_2: u32,param_3: i16,param_4: u16,uchar *param_5)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let iStack8: i16;
  let uStack6: u32;
  
  if (*param_1 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_5,0x1000);
  uVar3 = param_5 | param_4;
  if (uVar3 == 0x0) {
    param_4 = 0x0;
    uVar3 = 0x0;
  }
  else {
    pass1_1008_405c((astruct_76 *)CONCAT22(param_5,param_4),(param_1 + 0x4)
                    ,param_2,(param_2 >> 0x10));
  }
  uStack6 = CONCAT22(uVar3,param_4);
  iStack8 = 0x0;
  while (param_2 = param_2 & 0xffff0000 | (param_2 - 0x1), param_2 != 0x0
        ) {
    iVar1 = param_3 + 0x1;
    iVar4 = param_3 >> 0xf;
    pass1_1008_4544(*param_1);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(uStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_3),param_2._2_2_);
    param_3 = iVar1;
    iStack8 = iVar2;
  }
  return;
}


fn pass1_1008_6732(param_1: *mut u16)
{
  let lVar1: i32;
  astruct_457 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_457 *)param_1;
  *param_1 = 0x685a;
  iVar2->field_0x2 = 0x1008;
  if (iVar2->field_0x10 != 0x0) {
    lVar1 = iVar2->field_0x10;
    call_fn_ptr_1000_0dc6(lVar1,(lVar1 >> 0x10),0x1000);
  }
  iVar2->field_0x10 = 0x0;
  pass1_1008_41bc(param_1);
  return;
}


fn pass1_1008_6834(param_1: u32,param_2: u8) -> u32

{
  pass1_1008_6732((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_68c6(param_1: u16,param_2: u16,param_3: i16,param_4: u16) -> bool

{
  let BVar1: bool;
  let in_DX: u16;
  let unaff_SS: u16;
  
  BVar1 = show_win_1008_96ae(CONCAT22(param_2,param_1),param_3,param_4);
  pass1_1008_6a04(CONCAT22(param_2,param_1),in_DX,unaff_SS);
  return BVar1;
}



void 
pass1_1008_68ea(param_1: i16,param_2: u16,param_3: *mut u32,param_4: u16,param_5: u16,
               param_6: i16)

{
  code **ppcVar1;
  
  if (param_6 == 0x0) {
    if (*(long *)(param_1 + 0xce) != CONCAT22(param_4,param_3)) {
      if (*(long *)(param_1 + 0xce) != 0x0) {
        ppcVar1 = (code **)((param_1 + 0xce) + 0x10);
        (**ppcVar1)();
      }
      *(long *)(param_1 + 0xce) = CONCAT22(param_4,param_3);
      ppcVar1 = (code **)(*param_3 + 0x10);
      (**ppcVar1)();
      ppcVar1 = (code **)((param_1 + 0xce) + 0xc);
      (**ppcVar1)();
      return;
    }
  }
  else {
    pass1_1008_3e0e(CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)));
  }
  return;
}



fn pass1_1008_6978(param_1: u32,param_2: i16,param_3: u32,param_4: u16,uchar *param_5)
{
  code **ppcVar1;
  let puStack10: *mut u16;
  let puStack6: *mut u16;
  
  mem_op_1000_179c(0xa,param_5,0x1000);
  puStack10 = CONCAT22(param_5,param_4);
  if ((param_5 | param_4) == 0x0) {
    puStack6 = 0x0;
  }
  else {
    if (param_2 == 0x0) {
      param_2 = (param_1 + 0xcc);
    }
    *puStack10 = 0x389a;
    (param_4 + 0x2) = 0x1008;
    (param_4 + 0x4) = param_3;
    (param_4 + 0x8) = param_2;
    *puStack10 = 0x6c8c;
    (param_4 + 0x2) = 0x1008;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)((param_1 + 0xd2) + 0x4);
  (**ppcVar1)(0x1000,(param_1 + 0xd2),param_1._2_2_,puStack6);
  return;
}



fn pass1_1008_6a04(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let local_a: [u8;8];
  
  pass1_1008_57a4(CONCAT22(param_3,local_a),
                  param_1 & 0xffff0000 | (param_1 + 0xd2));
  while( true ) {
    puVar2 = local_a;
    pass1_1008_5b12(puVar2,param_3);
    if ((extraout_DX | puVar2) == 0x0) break;
    ppcVar1 = (code **)(**(u32 **)(puVar2 + 0x4) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1008_6a4a(param_1: u32,param_2: i16,param_3: u16,param_4: i16,param_5: u16)
{
  code **ppcVar1;
  let iVar2: i16;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let local_e: [u8;4];
  let uStack10: u32;
  let uStack6: u32;
  
  if (param_4 == 0x2) {
    iVar2 = param_1;
    pass1_1008_57a4(CONCAT22(param_5,local_e),
                    param_1 & 0xffff0000 | (iVar2 + 0xd2));
    do {
      puVar3 = local_e;
      pass1_1008_5b12(puVar3,param_5);
      uStack6 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | puVar3) == 0x0) break;
    } while ((puVar3 + 0x8) != param_2);
    if (uStack6 != 0x0) {
      ppcVar1 = (code **)((iVar2 + 0xd2) + 0xc);
      (**ppcVar1)();
      uStack10 = 0x0;
      uStack6._0_2_ = local_e;
      pass1_1008_5b12();
      if ((extraout_DX_00 | uStack6) != 0x0) {
        ppcVar1 = (code **)(**(u32 **)(uStack6 + 0x4) + 0x10);
        uStack6._2_2_ = extraout_DX_00;
        (**ppcVar1)();
        (iVar2 + 0xce) = (uStack6 + 0x4);
        return;
      }
      (iVar2 + 0xce) = 0x0;
    }
  }
  return;
}



fn pass1_1008_6b02(param_1: u32)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0x0) {
    ppcVar1 = (code **)((iVar2 + 0xce) + 0x6c);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_6b2e(param_1: u32)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0x0) {
    ppcVar1 = (code **)((iVar2 + 0xce) + 0x68);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_6b5a(param_1: *mut u16,param_2: u8) -> u16

{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_458 *uVar4;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  uVar4 = (astruct_458 *)param_1;
  *param_1 = 0x6c8c;
  uVar4->field_0x2 = 0x1008;
  puVar1 = uVar4->field_0x4;
  uVar2 = uVar4->field_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  uVar4->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_6bb4(param_1: *mut u16,param_2: u8)
{
  astruct_459 *uVar1;
  let uVar2: u16;
  
  uVar1 = (astruct_459 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1008_6c90(param_1: *mut u16)
{
  pass1_1008_3e38(param_1);
  pass1_1008_3e38((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x6)));
  return;
}



u32 * 
pass1_1008_6cb4(param_1: *mut u32,param_2: *mut u32,param_3: u16,param_4: *mut u32,param_5: u16
               )

{
  astruct_362 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_362 *)param_1;
  *param_1 = *param_4;
  iVar1->field_0x4 = (param_4 + 0x1);
  iVar1->field_0x6 = *param_2;
  iVar1->field_0xa = (param_2 + 0x1);
  return param_1;
}



fn pass1_1008_6cec(param_1: *mut u16,param_2: u16,param_3: u32,param_4: u16,param_5: u32)
{
  pass1_1008_3e76(param_1,param_4,param_5,(param_5 >> 0x10));
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x6)),
                  param_2,param_3,(param_3 >> 0x10));
  return;
}



fn pass1_1008_6d18(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_1,param_3);
  pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x6)),
                  param_2);
  return;
}



fn pass1_1008_6d3e(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,param_1);
  pass1_1008_3f62(param_2,
                          (param_1 & 0xffff0000 | (param_1 + 0x6)));
  return;
}



fn pass1_1008_6d64(param_1: *mut u16,param_2: *mut u16)
{
  pass1_1008_3f62(param_2,param_1);
  pass1_1008_3ee2((i16 *)param_2,
                  (i16 *)(param_1 & 0xffff0000 | (param_1 + 0x6)));
  return;
}


fn pass1_1008_72a8(param_1: *mut u16,param_2: u16) -> u16

{
  *param_1 = param_2;
  return param_1;
}


fn pass1_1008_766e(param_1: u32,param_2: *mut u32,param_3: u16,param_4: u16,uchar *param_5)
{
  let puVar1: u32;
  let puVar2: *mut u8
  let local_6: u32;
  
  *param_2 = 0x0;
  local_6 = 0x0;
  puVar1 = &local_6;
  file_1008_76e4(param_1,(long *)CONCAT22(param_3,puVar1),param_4,param_3,param_5)
  ;
  if (puVar1 != 0x0) {
    if (local_6 != 0x0) {
      mem_op_1000_179c(0xc,param_5,0x1000);
      puVar2 = (param_5 | puVar1);
      if (puVar2 == 0x0) {
        puVar1 = 0x0;
        puVar2 = 0x0;
      }
      else {
        pass1_1010_8ef2((u16 *)CONCAT22(param_5,puVar1),puVar2,param_3);
      }
      *(u32 **)param_2 = puVar1;
      *(uchar **)(param_2 + 0x2) = puVar2;
      fn_ptr_1010_905e(*param_2,local_6);
    }
    return;
  }
  return;
}


void 
pass1_1008_7898(param_1: u32,param_2: *mut u32,param_3: u16,param_4: u16,param_5: HFILE16
               ,param_6: u16)

{
  code **ppcVar1;
  let BVar2: bool;
  let extraout_DX: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let local_26: u16;
  let local_24: [u32;0x3];
  let local_18: u32;
  let local_14: [u16;0x5];
  let uStack10: u32;
  let uStack6: u32;
  
  if (param_2 == 0x0) {
    param_3 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)(*param_2 + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_3);
  local_18 = CONCAT22(uVar3,param_3);
  uVar4 = param_1;
  uVar5 = (param_1 >> 0x10);
  BVar2 = write_to_file_1008_7e1c
                    (uVar4,uVar5,&local_18,param_6,0x4,param_5);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return;
      }
      pass1_1020_c4a8(param_2,CONCAT22(param_6,local_14),
                      CONCAT22(param_6,&local_18),uStack10,param_4,param_6);
      local_24[0] = local_18;
      BVar2 = write_to_file_1008_7e1c
                        (uVar4,uVar5,local_24,param_6,0x4,0x1020);
      if (BVar2 == 0x0) break;
      local_26 = local_14[0];
      BVar2 = write_to_file_1008_7e1c
                        (uVar4,uVar5,&local_26,param_6,0x2,0x1020);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = 0x6d0;
        return;
      }
      uStack10 += 0x1;
    }
  }
  PTR_LOOP_1050_0310 = 0x6d0;
  return;
}


fn pass1_1008_79f0(param_1: u32,param_2: i32,param_3: HFILE16,param_4: u16)
{
  u16_t uVar1;
  let uVar2: u16;
  let uStack4: u16;
  
  if (param_2 == 0x0) {
    uVar1 = 0x0;
    uStack4 = 0x0;
  }
  else {
    uVar2 = (param_2 >> 0x10);
    uVar1 = *(u16_t *)(param_2 + 0x4);
    uStack4 = (param_2 + 0x6);
  }
  write_to_file_1008_7954
            (param_1,CONCAT22(uStack4,uVar1),uVar1,param_3,param_4);
  return;
}


fn pass1_1008_7ad4(param_1: u32,long *param_2,param_3: u16,param_4: HFILE16,param_5: u16) -> u16

{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_14: [u16;0x2];
  let local_10: [u32;0x2];
  let uStack6: u16;
  let local_4: u16;
  
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar2,uVar3,&local_4,0x0,param_5,0x2,param_4);
  if (BVar1 != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      if (local_4 <= uStack6) {
        return 0x1;
      }
      BVar1 = read_file_1008_7dee(uVar2,uVar3,local_14,0x0,param_5,0x2,param_4);
      if ((BVar1 == 0x0) ||
         (BVar1 = read_file_1008_7dee(uVar2,uVar3,local_10,0x0,param_5,0x4,param_4
                                     ), BVar1 == 0x0)) break;
      param_4 = 0x1020;
      pass1_1020_bb8a(param_2,local_10[0],
                      CONCAT22(local_14[0],(local_10[0] >> 0x10)),param_3,
                      param_5);
      uStack6 += 0x1;
    }
  }
  return 0x0;
}


fn pass1_1008_7c2a(param_1: u32,char *param_2,param_3: HFILE16) -> bool

{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  if (param_2 != 0x0) {
    uVar1 = str_op_1000_3da4(param_2);
    BVar2 = write_to_file_1008_7e1c
                      (param_1,uVar3,param_2,
                       (param_2 >> 0x10),(long)(uVar1 + 0x1),
                       0x1000);
    return BVar2;
  }
  write_to_file_1008_7e1c
            (param_1,uVar3,(s_playerName_1050_148e + 0xc),
             &USHORT_1050_1050,0x1,param_3);
  return 0x1;
}


u16 
pass1_1008_7e4a(param_1: u16,uchar *param_2,param_3: u8,char *param_4,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  
  sys_1000_3f9c((uchar *)&param_5,param_2,0x347,&USHORT_1050_1050,
                _PTR_s_dcbSC_1050_0336_1050_033c,&stack0xfffe,param_1,0x1000,
                param_2,param_3);
  uVar1 = str_op_1000_3da4(CONCAT22(param_2,&param_5));
  uVar1 = pass1_1000_3de8(param_4,CONCAT22(param_2,&param_5),uVar1,param_5,param_6
                         );
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1008_7e98(param_1: *mut u16,param_2: u8) -> u16

{
  astruct_460 *uVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (astruct_460 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_7ffa(param_1: *mut u16,param_2: u8)
{
  astruct_461 *uVar1;
  let uVar2: u16;
  
  uVar1 = (astruct_461 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1008_80d2(param_1: *mut u32) -> u32

{
  *param_1 = 0x0;
  (param_1 + 0x4) = 0x0;
  return param_1;
}


fn pass1_1008_8168(param_1: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x87c8;
  (param_1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  return;
}



fn pass1_1008_818c(astruct_23 *param_1,HINSTANCE16 param_2,WNDCLASS16 *param_3)
{
  let BVar1: bool;
  ATOM AVar2;
  let local_1c: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u32;
  let puStack18: *mut u8;
  let uStack16: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let iStack6: i16;
  let uStack4: u16;
  
  iStack6 = param_1 + 0x4;
  BVar1 = GetClassInfo16(param_2,(SEGPTR)&local_1c,param_3);
  if (BVar1 == 0x0) {
    local_1c = (param_1 + 0x54);
    uStack26 = 0x84f2;
    uStack24 = 0x1008;
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = 0x0;
    uStack14 = (param_1 + 0x58);
    uStack12 = (param_1 + 0x56);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}


fn pass1_1008_87a2(param_1: u32,param_2: u8) -> u32

{
  pass1_1008_8168((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}

void 
pass1_1008_87cc(astruct_86 *param_1,param_2: i16,param_3: i16,param_4: u16,param_5: u32,
               param_6: u32,param_7: u16)

{
  let lVar1: i32;
  let uVar2: u16;
  let BVar3: bool;
  let piVar4: *mut i16;
  let puVar5: *mut u8
  astruct_86 *iVar5;
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let uVar8: u16;
  let puVar9: *mut u16;
  let piStack48: *mut i16;
  let local_24: u32;
  let uStack32: u16;
  let uStack30: u32;
  astruct_18 *paStack26;
  let uStack18: u32;
  let iStack14: i16;
  let iStack12: i16;
  let uStack10: i16;
  let iStack8: i16;
  let uStack6: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_86 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  iVar5->field_0x4 = (astruct_76 *)param_5;
  &iVar5->field_0x8 = 0x0;
  iVar5->field_0xc = param_3;
  iVar5->field_0xe = param_2;
  iVar5->field_0x10 = 0x0;
  iVar5->field_0x12 = 0x0;
  pass1_1008_3e38((u16 *)(param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c)));
  pass1_1008_3e38((u16 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0x22));
  puVar9 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar5->field_0x28)
                          );
  iVar5->field_0x2e = param_4;
  iVar5->field_0x30 = 0xffff;
  iVar5->field_0x3a = 0x0;
  iVar5->field_0x3e = 0x1;
  iVar5->field_0x40 = 0x1;
  iVar5->field_0x42 = param_6;
  param_1->field_0x0 = 0x8e9a;
  iVar5->field_0x2 = 0x1008;
  if (_PTR_LOOP_1050_0382 == 0x0) {
    _PTR_LOOP_1050_0382 =
         mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2e,param_7,(puVar9 >> 0x10)
                         ,unaff_DI);
  }
  uStack6 = pass1_1008_4772(iVar5->field_0x4);
  iVar5->field_0x12 = 0x2f - (uStack6 + 0x8);
  uVar8 = (_PTR_LOOP_1050_0382 >> 0x10);
  iVar6 = _PTR_LOOP_1050_0382;
  iStack8 = (iVar6 + 0xa);
  iStack10 = (iVar6 + 0xc);
  iStack12 = (iVar6 + 0xe);
  iStack14 = (iVar6 + 0x10);
  iVar6 = iVar5->field_0xc;
  lVar1 = (long)(iVar6 + iVar5->field_0xe) * (long)iStack14;
  puVar5 = (lVar1 >> 0x10);
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c)),0x0
                  ,lVar1 + iVar5->field_0x12 + iStack10,
                  (iVar6 - iVar5->field_0xe) * iStack12 + iVar5->field_0x10 + iStack8);
  iVar5->field_0x14 = iVar5->field_0x1c + 0x20;
  iVar5->field_0x16 = (uStack6 + 0x8) + iVar5->field_0x1e + -0x25;
  iVar5->field_0x18 = iVar5->field_0x14 + 0x32;
  uVar2 = iVar5->field_0x16 + 0x19;
  iVar5->field_0x1a = uVar2;
  mem_op_1000_179c(0x6,puVar5,0x1000);
  paStack26 = (astruct_18 *)CONCAT22(puVar5,uVar2);
  uStack18._2_2_ = puVar5 | uVar2;
  if (uStack18._2_2_ == 0x0) {
    &iVar5->field_0x8 = 0x0;
  }
  else {
    puVar9 = pass1_1008_ada2((u16 *)CONCAT22(puVar5,uVar2),iVar5->field_0x2e);
    uStack18._2_2_ = (puVar9 >> 0x10);
    iVar5->field_0x8 = puVar9;
    iVar5->field_0xa = uStack18._2_2_;
  }
  BVar3 = pass1_1008_aed8(&iVar5->field_0x8);
  if (BVar3 == 0x0) {
    paStack26 = *(astruct_18 **)&iVar5->field_0x8;
    uStack18 = paStack26;
    fn_ptr_1000_17ce(paStack26,0x1000);
    &iVar5->field_0x8 = 0x0;
  }
  else {
    piVar4 = *(i16 **)&iVar5->field_0x8;
    pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar4);
    uStack18._0_2_ = SUB42(piVar4,0x0);
    pass1_1008_add2(*(u16 **)&iVar5->field_0x8);
    uStack30 = pass1_1008_4772((astruct_76 *)CONCAT22(uStack18._2_2_,uStack18)
                              );
    pass1_1018_214e(_PTR_LOOP_1050_0382,
                    (_PTR_LOOP_1050_0382 >> 0x10),

                    (param_1 & 0xffff0000 | &iVar5->field_0x28),
                    iVar5->field_0x2e);
    local_24 = &iVar5->field_0x1c;
    uStack32 = iVar5->field_0x20;
    pass1_1008_3f32((i16 *)CONCAT22(param_7,&local_24),
                    (i16 *)(param_1 & 0xffff0000 | &iVar5->field_0x28)
                   );
    piStack48 = (i16 *)(param_1 & 0xffff0000 | &iVar5->field_0x32);
    pass1_1008_3e94((u16 *)CONCAT22(param_7,&local_24),
                    (param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x34)),

                    (param_1 & 0xffff0000 | &iVar5->field_0x32));
    uVar8 = (uStack30 >> 0x10);
    iVar5->field_0x36 = (uStack30 + 0x4) + *piStack48;
    uVar2 = (uStack30 + 0x8) + iVar5->field_0x34;
    iVar5->field_0x38 = uVar2;
    pass1_1008_612e(0x2,0x5,uVar2);
    iVar5->field_0x3e = uVar2;
  }
  return;
}



fn pass1_1008_8aa2(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  code **ppcVar4;
  let uVar5: u32;
  astruct_462 *iVar6;
  let uVar6: u16;
  astruct_18 *paStack16;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_462 *)param_1;
  *param_1 = 0x8e9a;
  iVar6->field_0x2 = 0x1008;
  uVar5 = &iVar6->field_0x4;
  if ((uVar5 + 0x1c) != 0x0) {
    puVar1 = iVar6->field_0x4;
    uVar2 = iVar6->field_0x6;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  uVar2 = iVar6->field_0x3a;
  uVar3 = iVar6->field_0x3c;
  paStack16 = (astruct_18 *)CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar3,uVar2));
    fn_ptr_1000_17ce(paStack16,0x1000);
  }
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_8b20(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  let piVar2: *mut i16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let local_a: [u8;2];
  let local_8: [u8;2];
  astruct_76 *paStack6;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x8) != 0x0) {
    iVar1 = (iVar4 + 0x40);
    piVar2 = (i16 *)(iVar4 + 0x40);
    *piVar2 = *piVar2 + 0x1;
    uVar3 = iVar1 % (iVar4 + 0x3e);
    if (uVar3 == 0x0) {
      (iVar4 + 0x40) = 0x1;
      piVar2 = *(i16 **)(iVar4 + 0x8);
      pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar2);
      paStack6 = (astruct_76 *)(piVar2 & 0xffff | uVar3 << 0x10);
      pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (iVar4 + 0x28U)),
                      CONCAT22(param_2,local_a),
                      CONCAT22(param_2,local_8));
      pass1_1008_8d8a(param_1 & 0xffff | uVar5 << 0x10,paStack6,
                      (iVar4 + 0x4));
      pass1_1008_4480((iVar4 + 0x4),
                      (param_1 & 0xffff0000 | (iVar4 + 0x28U)),paStack6,
                      param_2);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_8bc6(param_1: u16,param_2: u16,param_3: u32)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let local_a: [u8;2];
  let local_8: [u8;2];
  astruct_76 *paStack6;
  
  uVar3 = (param_3 >> 0x10);
  iVar2 = param_3;
  if (*(long *)(iVar2 + 0x8) == 0x0) {
    return;
  }
  piVar1 = *(i16 **)(iVar2 + 0x8);
  pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar1);
  paStack6 = (astruct_76 *)(piVar1 & 0xffff | param_2 << 0x10);
  pass1_1008_3e94((u16 *)(param_3 & 0xffff0000 | (iVar2 + 0x28U)),
                  CONCAT22(param_1,local_a),CONCAT22(param_1,local_8))
  ;
  pass1_1008_8d8a(param_3 & 0xffff | uVar3 << 0x10,paStack6,(iVar2 + 0x4)
                 );
  pass1_1008_4480((iVar2 + 0x4),
                  (param_3 & 0xffff0000 | (iVar2 + 0x28U)),paStack6,
                  param_1);
  return;
}



fn pass1_1008_8c4e(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u16;
  let puVar3: *mut u8
  let puVar4: *mut u8
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u32;
  astruct_110 *paStack14;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar8 = pass1_1008_4772(*(astruct_76 **)(iVar6 + 0x4));
  puVar3 = (uVar8 >> 0x10);
  uVar5 = 0x0;
  if (((iVar6 + 0xc) == 0x0) || ((iVar6 + 0xe) == 0x0)) {
    puVar4 = puVar3;
    mem_op_1000_179c(0x14,puVar3,0x1000);
    paStack14 = (astruct_110 *)CONCAT22(puVar4,uVar5);
    uVar5 = puVar4 | uVar5;
    if (uVar5 == 0x0) {
      uVar1 = 0x0;
      uVar5 = 0x0;
    }
    else {
      puVar2 = (param_1 & 0xffff0000 | (iVar6 + 0x1c));
      pass1_1008_50c2(paStack14,(uVar8 + 0x8),(uVar8 + 0x4),
                      puVar2,param_2);
      uVar1 = SUB42(puVar2,0x0);
    }
    pass1_1008_5134(CONCAT22(uVar5,uVar1));
  }
  pass1_1008_4480(param_2,(param_1 & 0xffff0000 | (iVar6 + 0x1c)),
                  *(astruct_76 **)(iVar6 + 0x4),param_3);
  return;
}



fn pass1_1008_8ce4(param_1: u32,param_2: *mut u16,param_3: u32,param_4: u16)
{
  let puVar1: *mut u8;
  let puVar2: *mut u8
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let local_10: [u8;6];
  let uStack10: u32;
  let uStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uStack6 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x4));
  uStack10 = 0x0;
  puVar7 = pass1_1008_3e54((u16 *)CONCAT22(param_4,local_10),0x0,
                           (iVar4 + 0x12),(iVar4 + 0x10));
  puVar2 = (puVar7 >> 0x10);
  puVar1 = local_10;
  pass1_1008_3f32((i16 *)param_2,(i16 *)CONCAT22(param_4,puVar1));
  mem_op_1000_179c(0x14,puVar2,0x1000);
  uVar3 = puVar2 | puVar1;
  if (uVar3 == 0x0) {
    puVar1 = 0x0;
    uVar3 = 0x0;
  }
  else {
    uVar6 = (uStack6 >> 0x10);
    pass1_1008_50c2((astruct_110 *)CONCAT22(puVar2,puVar1),(uStack6 + 0x8),
                    (uStack6 + 0x4),param_2,param_3);
  }
  uStack10 = CONCAT22(uVar3,puVar1);
  pass1_1008_5134(CONCAT22(uVar3,puVar1));
  pass1_1008_4480(param_3,param_2,*(astruct_76 **)(iVar4 + 0x4),param_4);
  return;
}



fn pass1_1008_8d8a(param_1: u32,astruct_76 *param_2,param_3: u32)
{
  let uVar1: u16;
  let cVar2: u8;
  let puVar3: *mut u16;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let uVar6: u16;
  astruct_112 *iVar7;
  let uVar7: u16;
  let uVar8: u32;
  astruct_110 *paStack10;
  
  uVar7 = (param_1 >> 0x10);
  iVar7 = (astruct_112 *)param_1;
  uVar1 = iVar7->field_0x2e;
  if (uVar1 < 0x28) {
    if ((uVar1 < 0x25) && (uVar1 != 0x23)) {
      if (0x23 < uVar1) {
        return;
      }
      cVar2 = uVar1;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else {
    if (uVar1 < 0x46) {
      if (uVar1 < 0x43) {
        if (uVar1 < 0x33) {
          return;
        }
        if ((uVar1 != 0x34 && 0x0 < (uVar1 - 0x33)) && (uVar1 != 0x37)) {
          return;
        }
      }
    }
    else {
      if (uVar1 != 0x49) {
        if ((uVar1 - 0x49) < 0x2a) {
          return;
        }
        if (0x5 < (uVar1 - 0x73)) {
          return;
        }
      }
    }
  }
  if (iVar7->field_0x3a == 0x0) {
    uVar8 = pass1_1008_4772(param_2);
    puVar4 = (uVar8 >> 0x10);
    uVar1 = uVar8;
    puVar5 = puVar4;
    uVar6 = uVar1;
    mem_op_1000_179c(0x14,puVar4,0x1000);
    paStack10 = (astruct_110 *)CONCAT22(puVar5,uVar6);
    uVar6 = puVar5 | uVar6;
    if (uVar6 == 0x0) {
      iVar7->field_0x3a = 0x0;
    }
    else {
      puVar3 = (param_1 & 0xffff0000 | &iVar7->field_0x28);
      pass1_1008_50c2(paStack10,(uVar1 + 0x8),(uVar1 + 0x4),puVar3,
                      param_3);
      &iVar7->field_0x3a = puVar3;
      (&iVar7->field_0x3a + 0x2) = uVar6;
    }
    pass1_1008_5134(iVar7->field_0x3a);
    return;
  }
  pass1_1008_5236(iVar7->field_0x3a);
  return;
}



fn pass1_1008_8e74(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_8aa2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_8f24(param_1: *mut u16)
{
  let puVar1: u32;
  let puVar2: u32;
  let uVar3: u16;
  code **ppcVar4;
  let uVar5: u32;
  astruct_463 *iVar6;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uStack6: u32;
  
  uVar9 = (param_1 >> 0x10);
  iVar6 = (astruct_463 *)param_1;
  *param_1 = 0x9170;
  iVar6->field_0x2 = 0x1008;
  if (iVar6->field_0x1a != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      puVar1 = &iVar6->field_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = uStack6 * 0x4;
      uVar5 = iVar6->field_0x6;
      uVar10 = (uVar5 >> 0x10);
      iVar7 = uVar5;
      puVar2 = (iVar7 + iVar8);
      uVar3 = (iVar7 + iVar8 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 += 0x1;
    }
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x6,0x1000);
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



fn pass1_1008_8faa(param_1: u32,param_2: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1008_9004(param_1 & 0xffff | uVar1 << 0x10,param_2,
                  (param_2 >> 0x10),(param_1 + 0xa));
  return;
}


fn pass1_1008_9004(param_1: u32,param_2: u16,param_3: u16,param_4: u32)
{
  let puVar1: u32;
  let puVar2: *mut u16;
  let lVar3: i32;
  astruct_107 *iVar4;
  astruct_108 *iVar5;
  let uVar4: u16;
  let uVar5: u16;
  let unaff_SS: u16;
  let bVar6: bool;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_107 *)param_1;
  puVar1 = &iVar4->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
    puVar2 = (&iVar4->field_0x12 + 0x2);
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar1 = &iVar4->field_0x12,
                  puVar1 < param_4 || puVar1 == param_4))))
    {
      pass1_1008_909c(param_1 & 0xffff | uVar4 << 0x10,unaff_SS);
    }
    puVar1 = &iVar4->field_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
      return;
    }
    puVar2 = &iVar4->field_0xc;
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar2 = &iVar4->field_0xa,
                  *puVar2 < param_4 || *puVar2 == param_4)))) {
      iVar4->field_0xa = (param_4 + 0x1);
      iVar4->field_0xc = (param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar4->field_0x6;
  uVar5 = (lVar3 >> 0x10);
  iVar5 = (astruct_108 *)lVar3;
  (iVar5 + param_4 * 0x4) = param_2;
  (iVar5 + param_4 * 0x4 + 0x2) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_909c(param_1: u32,param_2: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  astruct_106 *iVar5;
  let uVar4: u16;
  let lVar5: i32;
  let lStack10: i32;
  let uStack6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_106 *)param_1;
  if (*(long *)&iVar5->field_0x12 == 0x0) {
    uVar3 = iVar5->field_0xe;
    PTR_LOOP_1050_5f2e = iVar5->field_0x10;
  }
  else {
    uVar2 = &iVar5->field_0x12;
    puVar1 = &iVar5->field_0x16;
    uVar3 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e =
         
         (iVar5->field_0x14 + iVar5->field_0x18 + CARRY2(uVar2,*puVar1));
  }
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if (iVar5->field_0x6 == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(uVar3 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    lVar5 = iVar5->field_0x6;
    lVar5 = pass1_1000_0ed4(0x1000,param_2,0x1,uVar3 * 0x4,
                            (PTR_LOOP_1050_5f2e * 0x2 + CARRY2(uVar3,uVar3)) *
                            0x2 + CARRY2(uVar3 * 0x2,uVar3 * 0x2),lVar5,
                            (lVar5 >> 0x10));
    PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
    uVar3 = lVar5;
  }
  lStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if ((PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
    &iVar5->field_0x12 = uStack6;
    iVar5->field_0x6 = lStack10;
  }
  return;
}



fn pass1_1008_914a(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_8f24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_91ba(param_1: *mut u16,HWND16 param_2) -> u16

{
  let UVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  *param_1 = 0x389a;
  (iVar2 + 0x2) = 0x1008;
  (iVar2 + 0x4) = 0x0;
  set_struct_1008_574a((astruct_21 *)(param_1 & 0xffff0000 | (iVar2 + 0x6)))
  ;
  *param_1 = 0x9416;
  (iVar2 + 0x2) = 0x1008;
  _PTR_LOOP_1050_0388 = param_1;
  UVar1 = SetTimer16(param_2,0x0,0x0,(LPVOID)(&PTR_LOOP_1050_0000 + 0x1));
  if (UVar1 == 0x0) {
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  PTR_LOOP_1050_038a = (_PTR_LOOP_1050_0388 >> 0x10);
  return param_1;
}




// WARNING: Globals starting with '_' overlap smaller symbols at the same address


void 
pass1_1008_9262(param_1: i16,param_2: u16,param_3: u32,param_4: u32,param_5: u16,
               uchar *param_6)

{
  code **ppcVar1;
  let uVar2: u16;
  
  mem_op_1000_179c(0x12,param_6,0x1000);
  uVar2 = param_6 | param_5;
  if (uVar2 == 0x0) {
    param_5 = 0x0;
    uVar2 = 0x0;
  }
  else {
    struct_op_1008_9174((astruct_88 *)CONCAT22(param_6,param_5),param_3,param_4);
  }
  if ((uVar2 | param_5) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0x6) + 0x4);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_92b2(param_1: u32,param_2: i32,param_3: i32)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let unaff_SS: u16;
  let local_c: [u8;4];
  let uStack8: u32;
  let uStack4: u16;
  
  uStack4 = 0x0;
  pass1_1008_57a4(CONCAT22(unaff_SS,local_c),
                  param_1 & 0xffff0000 | (param_1 + 0x6));
  while( true ) {
    puVar2 = local_c;
    pass1_1008_5b12(puVar2,unaff_SS);
    if ((extraout_DX | puVar2) == 0x0) break;
    if ((*(long *)(puVar2 + 0x4) == param_3) && (*(long *)(puVar2 + 0x8) == param_2)) {
      uStack4 = 0x1;
      ppcVar1 = (code **)((param_1 + 0x6) + 0xc);
      (**ppcVar1)();
      uStack8 = 0x0;
    }
  }
  return;
}



fn pass1_1008_932a(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  code **ppcVar2;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let local_a: [u8;8];
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x4) == 0x0) {
    (iVar5 + 0x4) = 0x1;
    pass1_1008_57a4(CONCAT22(param_2,local_a),
                    param_1 & 0xffff0000 | (iVar5 + 0x6));
    while( true ) {
      puVar3 = local_a;
      pass1_1008_5b12(puVar3,param_2);
      if ((extraout_DX | puVar3) == 0x0) break;
      uVar1 = (puVar3 + 0xc);
      iVar4 = (puVar3 + 0xe) - (uVar1 < 0x37);
      (puVar3 + 0xc) = uVar1 - 0x37;
      (puVar3 + 0xe) = iVar4;
      if ((iVar4 < 0x1) &&
         (((iVar4 < 0x0 || ((puVar3 + 0xc) == 0x0)) &&
          ((puVar3 + 0x10) == 0x0)))) {
        ppcVar2 = (code **)(**(u32 **)(puVar3 + 0x4) + 0x4);
        (**ppcVar2)();
        (puVar3 + 0xc) = (puVar3 + 0x8);
      }
    }
    (iVar5 + 0x4) = 0x0;
  }
  return;
}



fn pass1_1008_93c0(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_93ec(param_1: *mut u16,param_2: u8) -> u16

{
  let unaff_CS: u16;
  
  kill_timer_1008_921c(param_1,unaff_CS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_941a(param_1: *mut u16,param_2: u16,param_3: u16) -> u16

{
  *param_1 = param_2;
  (param_1 + 0x2) = param_3;
  return param_1;
}



fn pass1_1008_9436(param_1: *mut u16) -> u16

{
  *param_1 = 0x0;
  (param_1 + 0x2) = 0x0;
  return param_1;
}



fn pass1_1008_944e(param_1: *mut u16,param_2: u16,param_3: u16)
{
  (param_1 + 0x2) = param_3;
  *param_1 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_9466(param_1: *mut u16)
{
  *param_1 = 0x52a;
  (param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(_PTR_LOOP_1050_0392,0x1000);
  _PTR_LOOP_1050_0392 = (astruct_18 *)0x0;
  return;
}


fn pass1_1008_9628(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x8) == 0x0) {
    (param_1 + 0x8) = param_2;
  }
  return;
}


fn pass1_1008_9c16(param_1: u16,param_2: u32,param_3: u32,HWND16 param_4) -> LRESULT
{
  LRESULT LVar1;
  
  LVar1 = make_def_wnd_proc_1008_9ce6
                    (param_1,param_2,(param_2 >> 0x10),(WPARAM16)param_3,
                     CONCAT22(0x85,(param_3 >> 0x10)),param_4);
  return LVar1;
}



fn pass1_1008_9c30(param_1: u16,param_2: u32,param_3: u32,HWND16 param_4) -> LRESULT
{
  LRESULT LVar1;
  
  LVar1 = make_def_wnd_proc_1008_9ce6
                    (param_1,param_2,(param_2 >> 0x10),(WPARAM16)param_3,
                     CONCAT22(0x86,(param_3 >> 0x10)),param_4);
  return LVar1;
}



fn pass1_1008_9c4a(void)
{
  return;
}



fn pass1_1008_9c4e(void)
{
  return;
}



fn pass1_1008_9c52(void)
{
  return;
}



fn pass1_1008_9c60(param_1: u16,param_2: u16,param_3: *mut u32,param_4: i16)
{
  code **ppcVar1;
  
  if ((param_4 == 0xc7) && (param_3 != 0x0)) {
    ppcVar1 = (code **)*param_3;
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_9c86(param_1: u32,char *param_2,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = str_op_1000_3da4((param_1 & 0xffff0000 | (param_1 + 0xa)));
  if (param_3 < uVar1) {
    uVar1 = param_3 - 0x1;
  }
  str_op_1000_3dbe(param_2,(param_1 & 0xffff0000 | (param_1 + 0xa)),
                   uVar1);
  return;
}



fn pass1_1008_9cc4(param_1: u32,param_2: i16) -> bool

{
  if ((param_1 + 0x8) != param_2) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1008_9ce0(void) -> u16

{
  return 0x0;
}



fn pass1_1008_9d02(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_9d36(uchar *param_1,uchar *param_2,param_3: u16,param_4: u16)
{
  let puVar1: *mut u8
  let uVar2: u16;
  let puVar3: *mut u16;
  astruct_43 *paVar4;
  let uVar5: u32;
  let iStack4: i16;
  
  struct_op_1018_4cda(param_1,param_2,param_3);
  (param_1 + 0x1c) = 0x389a;
  (param_1 + 0x1e) = 0x1008;
  (param_1 + 0x1c) = 0x3aa8;
  (param_1 + 0x1e) = 0x1008;
  (param_1 + 0x20) = 0x0;
  puVar3 = pass1_1008_3e38((u16 *)CONCAT22(param_2,param_1 + 0x52));
  puVar1 = (puVar3 >> 0x10);
  CONCAT22(param_2,param_1) = 0x9fb2;
  (param_1 + 0x2) = 0x1008;
  (param_1 + 0x1c) = 0x9fca;
  (param_1 + 0x1e) = 0x1008;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x22),(WNDCLASS16 *)0x0,0x30);
  pass1_1018_4dce(CONCAT22(param_2,param_1),0x1c0,puVar1,param_4);
  iStack4 = 0x0;
  do {
    paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack4 + 0x1c0,param_4);
    (param_1 + iStack4 * 0x4 + 0x22) = paVar4;
    (param_1 + iStack4 * 0x4 + 0x24) = (paVar4 >> 0x10);
    iStack4 += 0x1;
  } while (iStack4 < 0xc);
  uVar5 = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x22));
  uVar2 = (uVar5 >> 0x10);
  pass1_1008_3e76((u16 *)CONCAT22(param_2,param_1 + 0x52),0x0,
                  (0x1e0 - (uVar5 + 0x8)) / 0x2 - 0x32,
                  (0x280 - (uVar5 + 0x4)) / 0x2);
  if (CONCAT22(param_2,param_1) == 0x0) {
    puVar1 = 0x0;
    param_2 = 0x0;
  }
  else {
    puVar1 = param_1 + 0x1c;
  }
  pass1_1008_9262(_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),
                  0x50,CONCAT22(param_2,puVar1),puVar1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_9e5a(astruct_11 *param_1)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: *mut u16;
  let uVar6: u16;
  astruct_464 *uVar5;
  let uVar7: u16;
  let puStack8: *mut u16;
  let iStack4: i16;
  
  uVar7 = (param_1 >> 0x10);
  uVar5 = (astruct_464 *)param_1;
  param_1 = 0x9fb2;
  uVar5->field_0x2 = 0x1008;
  uVar5->field_0x1c = 0x9fca;
  uVar5->field_0x1e = 0x1008;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == (astruct_11 *)0x0) {
      puVar4 = 0x0;
      uVar6 = 0x0;
    }
    else {
      puVar4 = &uVar5->field_0x1c;
      uVar6 = uVar7;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x50,CONCAT22(uVar6,puVar4));
  }
  iStack4 = 0x0;
  do {
    puVar1 = (&uVar5[0x1].field_0x0 + iStack4 * 0x4);
    uVar2 = (&uVar5[0x1].field_0x2)[iStack4 * 0x2];
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0xc);
  if (param_1 == (astruct_11 *)0x0) {
    puVar4 = 0x0;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar5->field_0x1c;
  }
  puStack8 = CONCAT22(uVar7,puVar4);
  *puStack8 = 0x389a;
  puVar4[0x1] = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}



fn pass1_1008_9f18(param_1: i16,param_2: u16,param_3: i16,param_4: u16)
{
  if (param_3 == 0x2) {
    pass1_1008_9f64(CONCAT22(param_2,param_1 + -0x1c));
    pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0x1c),0x2);
  }
  return;
}



fn pass1_1008_9f48(param_1: u32) -> u32

{
  astruct_134 *iVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar1 = (astruct_134 *)param_1;
  iVar2 = iVar1->field_0x20 * 0x4;
  return CONCAT22((&iVar1[0x1].field_0x2 + iVar2),
                  (&iVar1[0x1].field_0x0 + iVar2));
}



fn pass1_1008_9f64(param_1: u32)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  piVar1 = (i16 *)(iVar2 + 0x20);
  *piVar1 = *piVar1 + 0x1;
  if (0xb < (iVar2 + 0x20)) {
    (iVar2 + 0x20) = 0x0;
  }
  return;
}



astruct_11 *  pass1_1008_9f80(astruct_11 *param_1,param_2: u8)

{
  param_1 = (astruct_11 *)(param_1 & 0xffff0000 | (param_1 - 0x1c));
  pass1_1008_9e5a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void pass1_1008_9fb2(param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16
                    ,param_6: u8,param_7: u8,param_8: i16,param_9: i16,param_10: u8)

{
  char *pcVar1;
  byte *pbVar2;
  let bVar3: u8;
  let uVar4: u16;
  code *pcVar5;
  let bVar6: u8;
  let uVar7: u16;
  let uVar8: u16;
  let extraout_DL: u8;
  let puVar9: *mut u8
  let puVar10: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let uVar11: u16;
  let bVar12: u8;
  let bVar13: bool;
  let bVar14: bool;
  astruct_79 *paVar15;
  
  (param_8 + 0x1008) = &USHORT_1050_1050;
  uVar8 = param_10;
  uVar4 = param_5 + 0xeff0;
  bVar12 = param_5 < 0x1010 || uVar4 < uVar8;
  uVar7 = uVar4 - uVar8;
  pcVar5 = (code *)swi(0x4);
  if (SBORROW2(param_5,0x1010) != SBORROW2(uVar4,uVar8)) {
    (*pcVar5)();
    param_7 = extraout_DL;
  }
  bVar6 = (byte)((char)(uVar7 + 0xeff0) - bVar12) % 0x1d;
  pcVar1 = (param_8 + param_9);
  *pcVar1 = *pcVar1 + param_7 + (uVar7 < 0x1010 || uVar7 + 0xeff0 < bVar12);
  pbVar2 = (byte *)(param_8 + param_9);
  bVar13 = *pbVar2 < param_7 || (byte)(*pbVar2 - param_7) < (0xb1 < bVar6);
  *pbVar2 = (*pbVar2 - param_7) - (0xb1 < bVar6);
  pbVar2 = (byte *)(param_8 + 0x18);
  bVar14 = *pbVar2 < param_6 || (byte)(*pbVar2 - param_6) < bVar13;
  *pbVar2 = (*pbVar2 - param_6) - bVar13;
  pbVar2 = (byte *)(param_8 + param_9 + 0x89f);
  bVar12 = *pbVar2;
  bVar3 = *pbVar2 + bVar6 + 0x4e;
  *pbVar2 = bVar3 + bVar14;
  pcVar1 = (param_8 + param_9);
  *pcVar1 = *pcVar1 + param_8 +
            (CARRY1(bVar12,bVar6 + 0x4e) || CARRY1(bVar3,bVar14));
  pbVar2 = (byte *)(param_8 + param_9);
  *pbVar2 = *pbVar2 | param_7;
  paVar15 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_3,param_2),param_4);
  puVar9 = (paVar15 >> 0x10);
  uVar8 = 0x0;
  (param_2 + 0xa) = 0x0;
  (param_2 + 0x410) = 0x0;
  (param_2 + 0x414) = 0x0;
  (param_2 + 0x416) = 0x0;
  (param_2 + 0x418) = 0x0;
  (param_2 + 0x41a) = 0x0;
  (param_2 + 0x41c) = 0x0;
  (param_2 + 0x41e) = 0x0;
  CONCAT22(param_3,param_2) = 0xad92;
  (param_2 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,puVar9,0x1000);
  puVar10 = (puVar9 | uVar8);
  if (puVar10 == 0x0) {
    (param_2 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar9,uVar8));
    (param_2 + 0xa) = uVar8;
    *(uchar **)(param_2 + 0xc) = extraout_DX;
    puVar10 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar10,0x1000);
  if ((puVar10 | uVar8) == 0x0) {
    uVar8 = 0x0;
    uVar11 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10,uVar8));
    uVar11 = extraout_DX_00;
  }
  (param_2 + 0x410) = uVar8;
  (param_2 + 0x412) = uVar11;
  return;
}


fn pass1_1008_a086(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_465 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_465 *)param_1;
  *param_1 = 0xad92;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x410;
  uVar2 = iVar4->field_0x412;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}


fn pass1_1008_b200(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let puVar3: u32;
  let puVar4: *mut u8;
  astruct_195 *uVar5;
  let uVar6: u16;
  let uVar7: u32;
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let extraout_DX_01: u16;
  let uVar10: u16;
  let uVar11: u16;
  let extraout_DX_02: *mut u8
  astruct_194 *iVar12;
  let uVar12: u16;
  char *pcVar13;
  let local_14: [u8;12];
  
  uVar12 = (param_1 >> 0x10);
  iVar12 = (astruct_194 *)param_1;
  if (iVar12->field_0xe != 0x0) {
    return;
  }
                    // WARNING: Load size is inaccurate
  puVar3 = iVar12->field_0xe;
  puVar9 = *(uchar **)(&iVar12->field_0xe + 0x2);
  if ((puVar9 | puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
    puVar9 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar9,0x1000);
  if ((puVar9 | puVar3) == 0x0) {
    puVar3 = 0x0;
    puVar9 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar9,puVar3));
    puVar9 = extraout_DX_00;
  }
  *(u32 **)&iVar12->field_0xe = puVar3;
  *(uchar **)(&iVar12->field_0xe + 0x2) = puVar9;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  while( true ) {
    puVar8 = puVar9;
    puVar4 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar4));
    puVar9 = (puVar8 | puVar4);
    if (puVar9 == 0x0) break;
    uVar1 = (puVar4 + 0x4);
    if (*(long *)(puVar4 + 0x200) == 0x8000001) {
      uVar7 = uVar1;
      mem_op_1000_179c(0xc,puVar9,0x1000);
      uVar5 = (astruct_195 *)uVar7;
      if ((puVar9 | uVar5) == 0x0) {
        uVar5 = (astruct_195 *)0x0;
        uVar10 = 0x0;
      }
      else {
        pass1_1008_b0f2((u16 *)(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10));
        uVar10 = extraout_DX_01;
      }
      pcVar13 = pass1_1038_4d28(CONCAT22(puVar8,puVar4));
      uVar11 = (pcVar13 >> 0x10);
      uVar6 = str_op_1008_60e8(pcVar13,uVar11);
      uVar5->field_0x4 = uVar6;
      uVar5->field_0x6 = uVar11;
      uVar5->field_0x8 = uVar1;
      ppcVar2 = (code **)(*iVar12->field_0xe + 0x8);
      (**ppcVar2)(&PTR_LOOP_1050_1038,iVar12->field_0xe,uVar5,uVar10);
      puVar9 = extraout_DX_02;
    }
  }
  return;
}



fn pass1_1008_b340(param_1: u32) -> u32

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x16) != 0x0) {
    uVar1 = (param_1 + 0x16);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



fn pass1_1008_b366(param_1: u32) -> u32

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) != 0x0) {
    uVar1 = (param_1 + 0x1a);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_b38c(param_1: u32,param_2: u16,uchar *param_3) -> u32

{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  astruct_197 *iVar3;
  astruct_196 *iVar4;
  let uVar4: u16;
  let puVar5: *mut u16;
  let iStack4: i16;
  astruct_198 *iVar5;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_196 *)param_1;
  if (iVar4->field_0x12 == 0x0) {
    mem_op_1000_179c(0xc,param_3,0x1000);
    puVar5 = CONCAT22(param_3 | param_2,param_2);
    if ((param_3 | param_2) == 0x0) {
      iVar4->field_0x12 = 0x0;
    }
    else {
      puVar5 = set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,param_2));
      &iVar4->field_0x12 = puVar5;
      (&iVar4->field_0x12 + 0x2) = (puVar5 >> 0x10);
    }
    for (iStack4 = 0x6d9; iStack4 < 0x6e7; iStack4 += 0x1) {
      if (iStack4 == 0x6e3) {
        pass1_1030_8344(_PTR_LOOP_1050_5748,
                        (_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
        if ((puVar5 + 0x136) != 0x0) goto LAB_1008_b44a;
      }
      else {
LAB_1008_b44a:
        mem_op_1000_179c(0xa,(puVar5 >> 0x10),0x1000);
        if (puVar5 == 0x0) {
          puVar5 = 0x0;
        }
        else {
          puVar5 = pass1_1008_b11e(puVar5);
        }
        uVar3 = (puVar5 >> 0x10);
        iVar5 = (astruct_198 *)puVar5;
        load_string_1010_84ac
                  (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),
                   0x1010);
        iVar5->field_0x4 = puVar5;
        iVar5->field_0x6 = (puVar5 >> 0x10);
        iVar5->field_0x8 = iStack4 + -0x6d8;
        puVar1 = iVar4->field_0x12;
        ppcVar2 = (code **)(*iVar4->field_0x12 + 0x8);
        puVar5 =
                 (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),iVar5,uVar3);
      }
    }
  }
  return CONCAT22((&iVar4->field_0x12 + 0x2),
                  &iVar4->field_0x12);
}



fn pass1_1008_b47a(param_1: u32) -> u32

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1e) != 0x0) {
    uVar1 = (param_1 + 0x1e);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



fn pass1_1008_b4a0(param_1: u32,param_2: i32,param_3: u16,uchar *param_4,param_5: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  let lVar7: i32;
  
  iVar4 = param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    (iVar4 + 0x16) = 0x0;
  }
  else {
    pass1_1008_b9ce(param_1,param_2,param_5);
    (iVar4 + 0x16) = param_3;
    *(uchar **)(iVar4 + 0x18) = param_4;
  }
  uVar1 = (iVar4 + 0x16);
  if ((uVar1 + 0x8) != 0x0) {
    pass1_1008_b200(param_1,param_5);
    uVar6 = pass1_1008_b38c(param_1,param_3,param_4);
    uVar3 = (uVar6 >> 0x10);
    uVar2 = uVar6;
    uVar1 = (iVar4 + 0x16);
    pass1_1008_b85c(param_1,*(long *)(uVar1 + 0xa));
    (iVar4 + 0x1a) = uVar2;
    (iVar4 + 0x1c) = uVar3;
    uVar1 = (iVar4 + 0x16);
    lVar7 = pass1_1008_b8ac(param_1,(uVar1 + 0xe),param_5);
    (iVar4 + 0x1e) = lVar7;
    (iVar4 + 0x20) = (lVar7 >> 0x10);
    return;
  }
  (iVar4 + 0x1a) = 0x0;
  (iVar4 + 0x1e) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_b544(param_1: u32,param_2: i16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u32;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  
  iVar7 = param_1;
  uVar8 = (param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (*(long *)(iVar7 + 0x1a) != 0x0) {
      uVar4 = (iVar7 + 0x16);
      (uVar4 + 0x8) = 0x1;
      uVar4 = (iVar7 + 0x1a);
      uVar5 = (iVar7 + 0x16);
      (uVar5 + 0xa) = (uVar4 + 0x8);
      uVar4 = (iVar7 + 0x1e);
      uVar6 = (uVar4 + 0x8);
      uVar4 = (iVar7 + 0x16);
      (uVar4 + 0xe) = uVar6;
      uVar4 = (iVar7 + 0x16);
      pass1_1030_8344(_PTR_LOOP_1050_5748,
                      (_PTR_LOOP_1050_5748 >> 0x10),
                      (uVar4 + 0xa));
      param_4 = &PTR_LOOP_1050_1038;
      pass1_1038_3608(CONCAT22(param_3,uVar6));
    }
  }
  (iVar7 + 0x1e) = 0x0;
  (iVar7 + 0x1a) = 0x0;
  (iVar7 + 0x16) = 0x0;
  puVar1 = (iVar7 + 0xe);
  uVar2 = (iVar7 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_4,puVar1,uVar2,0x1);
  }
  (iVar7 + 0xe) = 0x0;
  puVar1 = (iVar7 + 0x12);
  uVar2 = (iVar7 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_4,puVar1,uVar2,0x1);
  }
  (iVar7 + 0x12) = 0x0;
  return;
}



fn pass1_1008_b61a(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u16;
  
  pass1_1008_b8fa(param_1,param_2,param_4,param_5);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1a) = param_3;
  (param_1 + 0x1c) = param_4;
  return;
}



fn pass1_1008_b63a(param_1: u32,param_2: u32)
{
  let in_AX: u16;
  let in_DX: u16;
  let uVar1: u16;
  let unaff_SS: u16;
  
  pass1_1008_b964(param_1,param_2,unaff_SS);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1e) = in_AX;
  (param_1 + 0x20) = in_DX;
  return;
}


fn pass1_1008_b820(param_1: u32,param_2: i16,param_3: u16) -> u32

{
  let uVar1: u16;
  
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x8000001);
  if ((param_2 + 0x152) == 0x0) {
    return 0x0;
  }
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0xc),(param_1 + 0xa))
  ;
}



fn pass1_1008_b85c(param_1: u32,param_2: i32)
{
  let puVar1: *mut u8;
  let extraout_DX: u16;
  let unaff_SS: u16;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(unaff_SS,local_a),(param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,unaff_SS);
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (*(long *)(puVar1 + 0x8) != param_2);
  return;
}



fn pass1_1008_b8ac(param_1: u32,param_2: i16,param_3: u16) -> i32
{
  let lVar1: i32;
  let local_a [u8;8];
  
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0x12));
  do {
    lVar1 = pass1_1008_5b12(local_a,param_3);
    if (lVar1 == 0x0) {
      return 0x0;
    }
  } while ((lVar1 + 0x8) != param_2);
  return lVar1;
}



fn pass1_1008_b8fa(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let puVar1: *mut u8;
  let iVar2: i16;
  let extraout_DX: u16;
  let local_a: [u8;8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784(CONCAT22(param_4,local_a),(param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a((puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



fn pass1_1008_b964(param_1: u32,param_2: u32,param_3: u16)
{
  let puVar1: *mut u8;
  let iVar2: i16;
  let extraout_DX: u16;
  let local_a: [u8;8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0x12));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a((puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



fn pass1_1008_b9ce(param_1: u32,param_2: u32,param_3: u16)
{
  let puVar1: *mut u8;
  let iVar2: i16;
  let extraout_DX: u16;
  let local_a: [u8;8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a((puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



fn pass1_1008_ba38(param_1: u32,param_2: u32,param_3: HFILE16,param_4: u16)
{
  let uVar1: u32;
  let BVar2: bool;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let local_2a: [u32;0x3];
  let local_1e: [u16;0x5];
  let local_14: [u8;8];
  let local_c: u16;
  let uStack10: u32;
  let local_6: [u16;0x2];
  
  BVar2 = write_to_file_1008_7cac(param_2,param_4);
  if BVar2 != 0x0 {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    local_c = (iVar4 + 0x22);
    uVar6 = param_2;
    uVar7 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c
                      (uVar6,uVar7,&local_c,param_4,0x2,param_3);
    if BVar2 != 0x0 {
      if *(long *)(iVar4 + 0xa) == 0x0 {
        local_c = 0x0;
      }
      else {
        uVar1 = (iVar4 + 0xa);
        local_c = (uVar1 + 0x8);
      }
      local_1e[0] = local_c;
      BVar2 = write_to_file_1008_7e1c
                        (uVar6,uVar7,local_1e,param_4,0x2,param_3);
      if BVar2 != 0x0 {
        pass1_1008_5784(CONCAT22(param_4,local_14),(iVar4 + 0xa));
        do {
          puVar3 = local_14;
          pass1_1008_5b12(puVar3,param_4);
          uStack10 = CONCAT22(extraout_DX,puVar3);
          if ((extraout_DX | puVar3) == 0x0) {
            return;
          }
          BVar2 = pass1_1008_7c2a(param_2,*(char **)(puVar3 + 0x4),param_3);
          if BVar2 == 0x0
              break;
          local_6[0] = (uStack10 + 0x8);
          BVar2 = write_to_file_1008_7e1c
                            (uVar6,uVar7,local_6,param_4,0x2,param_3);
          if BVar2 == 0x0
              break;
          local_2a[0] = (uStack10 + 0xa);
          BVar2 = write_to_file_1008_7e1c
                            (uVar6,uVar7,local_2a,param_4,0x4,param_3);
          if BVar2 == 0x0
              break;
          local_6[0] = (uStack10 + 0xe);
          BVar2 = write_to_file_1008_7e1c
                            (uVar6,uVar7,local_6,param_4,0x2,param_3);
        } while BVar2 != 0x0;
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}


fn pass1_1008_bd02(param_1: u32,param_2: u8) -> u32

{
  let unaff_SS: u16;
  
  pass1_1008_afde((u16 *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_bd28(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_b08c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_18 *  pass1_1008_bd4e(astruct_18 *param_1,param_2: u8)

{
  pass1_1008_b08c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 *  pass1_1008_bd74(astruct_18 *param_1,param_2: u8)

{
  pass1_1008_b08c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 *  pass1_1008_bd9a(astruct_18 *param_1,param_2: u8)

{
  pass1_1008_b08c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_d3ae(param_1: u32)
{
  let puVar1: u32;
  let puVar2: u32;
  code **ppcVar3;
  let bVar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let puVar8: *mut u8
  let puVar9: *mut u8
  astruct_208 *iVar13;
  let uVar10: u16;
  let uVar11: u16;
  astruct_21 *paVar12;
  let uVar13: u32;
  let uVar14: u32;
  let uVar15: u32;
  let uStack6: u16;
  
  uVar10 = (param_1 >> 0x10);
  iVar13 = (astruct_208 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar1 = iVar13->field_0xa;
  uVar5 = (&iVar13->field_0xa + 0x2);
  paVar12 = (astruct_21 *)CONCAT22(uVar5,puVar1);
  if ((uVar5 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    paVar12 = (astruct_21 *)(**ppcVar3)();
  }
  mem_op_1000_179c(0xc,(paVar12 >> 0x10),0x1000);
  if (paVar12 == (astruct_21 *)0x0) {
    uVar13 = 0x0;
  }
  else {
    uVar13 = set_struct_1008_574a(paVar12);
  }
  &iVar13->field_0xa = uVar13;
  (&iVar13->field_0xa + 0x2) = (uVar13 >> 0x10);
  bVar4 = false;
  for (uStack6 = 0x21; 0x10 < uStack6; uStack6 -= 0x1) {
    uVar15 = uVar13;
    empty_1038_540a();
    puVar8 = (uVar15 >> 0x10);
    uVar5 = puVar8 | uVar15;
    uVar13 = uVar15 & 0xffff0000 | uVar5;
    if (uVar15 != 0x0) {
      bVar4 = true;
      string_1020_c0ca(uStack6);
      uVar6 = str_op_1008_60e8(CONCAT22(puVar8,uVar5),puVar8);
      uVar11 = 0x1000;
      uVar7 = uVar6;
      puVar9 = puVar8;
      mem_op_1000_179c(0x10,puVar8,0x1000);
      if ((puVar9 | uVar7) == 0x0) {
        uVar14 = 0x0;
      }
      else {
        uVar11 = 0x1018;
        uVar14 = struct_1018_4790(CONCAT22(puVar9,uVar7),uVar15,CONCAT22(puVar8,uVar6),
                                  uStack6);
      }
      puVar2 = iVar13->field_0xa;
      ppcVar3 = (code **)(*iVar13->field_0xa + 0x4);
      uVar13 = (**ppcVar3)(uVar11,puVar2,(puVar2 >> 0x10),uVar14);
    }
  }
  if (!bVar4) {
    load_string_1010_84ac
              (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar11 = 0x1000;
    uVar15 = uVar13;
    mem_op_1000_179c(0x10,(uVar13 >> 0x10),0x1000);
    if (uVar15 == 0x0) {
      uVar14 = 0x0;
    }
    else {
      uVar11 = 0x1018;
      uVar14 = struct_1018_4790(uVar15,0x0,uVar13,0x0);
    }
    puVar2 = iVar13->field_0xa;
    ppcVar3 = (code **)(*iVar13->field_0xa + 0x4);
    (**ppcVar3)(uVar11,puVar2,(puVar2 >> 0x10),uVar14);
  }
  return;
}



fn pass1_1008_d6f4(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1008_caa0(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_d72e(param_1: i16,param_2: u16,param_3: u16) -> u16

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  CONCAT22(param_2,param_1) = 0xd780;
  (param_1 + 0x2) = 0x1008;
  return CONCAT22(param_2,param_1);
}



fn pass1_1008_d75a(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_d790(astruct_647 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  astruct_43 *paVar1;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  CONCAT22(param_2,param_1) = 0xd98e;
  param_1->field_0x2 = 0x1008;
  paVar1 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x9b,param_4);
  param_1->field_0xa = paVar1;
  param_1->field_0xc = (paVar1 >> 0x10);
  return;
}



fn pass1_1008_d7da(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0xd98e;
  (iVar4 + 0x2) = 0x1008;
  puVar1 = (iVar4 + 0xa);
  uVar2 = (iVar4 + 0xc);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1008_d818(param_1: u32,param_2: i16)
{
  astruct_732 *iVar1;
  let uVar1: u16;
  
  if (param_2 - 0x1a0U < 0x15) {
    iVar1 = (astruct_732 *)param_1;
    uVar1 = (param_1 >> 0x10);
    switch(param_2) {
    case 0x1a0:
      iVar1->field_0xe = 0x14;
      break;
    case 0x1a1:
      iVar1->field_0xe = 0x3;
      break;
    case 0x1a2:
      iVar1->field_0xe = 0x2;
      break;
    case 0x1a3:
      iVar1->field_0xe = 0xe;
      break;
    case 0x1a4:
      iVar1->field_0xe = 0xc;
      break;
    case 0x1a5:
      iVar1->field_0xe = 0x4;
      break;
    case 0x1a6:
      iVar1->field_0xe = 0xb;
      break;
    case 0x1a7:
      iVar1->field_0xe = 0x6;
      break;
    case 0x1a8:
      iVar1->field_0xe = 0xa;
      break;
    case 0x1a9:
      iVar1->field_0xe = 0xd;
      break;
    case 0x1aa:
      iVar1->field_0xe = 0x13;
      break;
    case 0x1ab:
      iVar1->field_0xe = 0x5;
      break;
    case 0x1ac:
      iVar1->field_0xe = 0x9;
      break;
    case 0x1ad:
      iVar1->field_0xe = 0x8;
      break;
    case 0x1ae:
      iVar1->field_0xe = 0x12;
      break;
    case 0x1af:
      iVar1->field_0xe = 0x11;
      break;
    case 0x1b0:
      iVar1->field_0xe = 0x7;
      return;
    case 0x1b1:
      iVar1->field_0xe = 0x10;
      return;
    case 0x1b2:
      iVar1->field_0xe = 0x1;
      return;
    case 0x1b3:
      iVar1->field_0xe = 0xf;
      return;
    case 0x1b4:
      iVar1->field_0xe = 0x15;
      return;
    }
  }
  return;
}



fn pass1_1008_d968(param_1: *mut u16,param_2: u8) -> u16

{
  let unaff_SS: u16;
  
  pass1_1008_d7da(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_d99e(param_1: i16,param_2: u16,param_3: u16,uchar *param_4,param_5: u16)
{
  struct_op_1018_4cda(param_1,param_2,param_3);
  CONCAT22(param_2,param_1) = 0xd9fa;
  (param_1 + 0x2) = 0x1008;
  pass1_1018_4dce(CONCAT22(param_2,param_1),0x9a,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}



astruct_11 *  pass1_1008_d9d4(astruct_11 *param_1,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_dc2c(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0xdc80;
  (param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x18),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1008_dc5a(param_1: *mut u16,param_2: u8) -> u16

{
  let unaff_SS: u16;
  
  pass1_1008_dc2c(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Variable defined which should be unmapped: param_10

void 
pass1_1008_dc80(param_1: u16,param_2: *mut u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u8,param_7: i16,param_8: i16,param_9: u8,param_10: u16)

{
  char *pcVar1;
  let uVar2: u16;
  let uVar3: u16;
  code *pcVar4;
  let uVar5: u16;
  let cVar6: u8;
  let extraout_DL: u8;
  let bVar7: u8;
  let iVar8: i16;
  let uVar9: u16;
  let bVar10: u8;
  
  bVar7 = (byte)(param_10 >> 0x8);
  bVar10 = (byte)param_10 + bVar7;
  cVar6 = bVar10 + param_9;
  uVar2 = (CARRY1((byte)param_10,bVar7) || CARRY1(bVar10,param_9));
  uVar3 = param_5 + 0xeff0;
  bVar10 = param_5 < 0x1010 || uVar3 < uVar2;
  uVar5 = uVar3 - uVar2;
  pcVar4 = (code *)swi(0x4);
  if (SBORROW2(param_5,0x1010) != SBORROW2(uVar3,uVar2)) {
    (*pcVar4)();
    cVar6 = extraout_DL;
  }
  pcVar1 = (param_7 + param_8);
  *pcVar1 = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < bVar10);
  uVar9 = (param_2 >> 0x10);
  iVar8 = param_2;
  *param_2 = 0x389a;
  (iVar8 + 0x2) = 0x1008;
  (iVar8 + 0x4) = param_4;
  (iVar8 + 0x8) = param_3;
  (iVar8 + 0xc) = 0x0;
  (iVar8 + 0xe) = 0x0;
  (iVar8 + 0x12) = 0x0;
  *param_2 = 0xdd4a;
  (iVar8 + 0x2) = 0x1008;
  return;
}


fn pass1_1008_dd1e(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1008_ddca(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_471 *iVar5;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_471 *)param_1;
  *param_1 = 0xeaac;
  iVar5->field_0x2 = 0x1008;
  puVar1 = iVar5->field_0xe;
  uVar2 = iVar5->field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar5->field_0x12;
  uVar2 = iVar5->field_0x14;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar5->field_0xa;
  uVar2 = iVar5->field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x1e,0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_de58(param_1: u32,param_2: i32,param_3: i32,param_4: u16)
{
  code **ppcVar1;
  let bVar2: bool;
  astruct_210 *puVar4;
  let extraout_DX: u16;
  let puVar3: *mut u8
  let uVar4: u16;
  astruct_211 *iVar6;
  astruct_210 *paVar5;
  let uVar6: u16;
  let uVar7: u32;
  let local_a: [u8;8];
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_211 *)param_1;
  pass1_1008_5784(CONCAT22(param_4,local_a),iVar6->field_0xa);
  bVar2 = false;
  do {
    puVar4 = (astruct_210 *)local_a;
    pass1_1008_5b12(puVar4,param_4);
    puVar3 = (extraout_DX | puVar4);
    paVar5 = puVar4;
    if (puVar3 == 0x0) goto LAB_1008_dedb;
  } while (((puVar4->field_0x4 != param_3) || (puVar4->field_0x8 != param_2)) &&
          ((puVar4->field_0x8 != param_3 || (puVar4->field_0x4 != param_2))));
  puVar4->field_0xc = 0x1;
  uVar7 = pass1_1030_8326();
  puVar3 = (uVar7 >> 0x10);
  paVar5 = (astruct_210 *)uVar7;
  puVar4->field_0xe = paVar5;
  puVar4->field_0x10 = puVar3;
  bVar2 = true;
LAB_1008_dedb:
  if (!bVar2) {
    mem_op_1000_179c(0x14,puVar3,0x1000);
    uVar4 = puVar3 | paVar5;
    if (uVar4 == 0x0) {
      paVar5 = (astruct_210 *)0x0;
      uVar4 = 0x0;
    }
    else {
      struct_1008_dc90((u16 *)CONCAT22(puVar3,paVar5),param_2,param_3);
    }
    paVar5->field_0xc = 0x1;
    uVar7 = pass1_1030_8326();
    paVar5->field_0xe = uVar7;
    paVar5->field_0x10 = (uVar7 >> 0x10);
    ppcVar1 = (code **)(*iVar6->field_0xa + 0x4);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1008_df4a(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  let local_a: [u8;8];
  
  uVar2 = (param_1 >> 0x10);
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0xa));
  while( true ) {
    uVar3 = pass1_1008_5b12(local_a,param_3);
    uVar1 = (uVar3 >> 0x10);
    if (uVar3 == 0x0) break;
    if (((uVar3 + 0xc) == 0x2) || ((uVar3 + 0xc) == 0x3)) {
      pass1_1008_e9a4(param_1,uVar2,uVar3,param_2,param_3);
    }
  }
  return;
}



fn pass1_1008_dfa6(param_1: u32,param_2: i32,param_3: i32,param_4: u16)
{
  let puVar1: *mut u8;
  let extraout_DX: u16;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_4,local_a),(param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (((*(long *)(puVar1 + 0x4) != param_3) || (*(long *)(puVar1 + 0x8) != param_2))
          && ((*(long *)(puVar1 + 0x8) != param_3 || (*(long *)(puVar1 + 0x4) != param_2))
             ));
  if ((puVar1 + 0xc) != 0x1) {
    return;
  }
  return;
}



fn pass1_1008_e01c(param_1: u32,param_2: u32,param_3: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x16) = param_3;
  (param_1 + 0x1a) = param_2;
  return;
}



fn pass1_1008_e038(param_1: u32,param_2: *mut u32,param_3: *mut u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x16);
  *param_2 = (param_1 + 0x1a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1008_e05e(param_1: u32,param_2: u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u8)

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  let local_122: [u8;112];
  let iStack16: i16;
  let local_e: [u8;8];
  let lStack6: i32;
  
  lStack6 = pass1_1008_e8cc(param_5,param_1,param_3,param_4);
  if (lStack6 != 0x0) {
    uVar3 = pass1_1030_8326();
    uVar2 = (lStack6 >> 0x10);
    iVar1 = lStack6;
    (iVar1 + 0xe) = uVar3;
    (iVar1 + 0x10) = (uVar3 >> 0x10);
    (iVar1 + 0xc) = param_2;
  }
  pass1_1008_5784(CONCAT22(param_5,local_e),(param_1 + 0xa));
  iStack16 = 0x0;
  do {
    lStack6 = pass1_1008_5b12(local_e,param_5);
    if (lStack6 == 0x0) goto LAB_1008_e0d3;
  } while ((lStack6 + 0xc) != 0x1);
  iStack16 = 0x1;
LAB_1008_e0d3:
  if (iStack16 == 0x0) {
    struct_1030_e2be((astruct_100 *)CONCAT22(param_5,local_122),0x0,0x0,0x0,param_5,
                     param_6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_5,local_122));
  }
  return;
}



i16 
pass1_1008_e10c(param_1: u32,param_2: u32,param_3: u32,param_4: i16,param_5: u16)

{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u32;
  
  uVar3 = pass1_1008_e8cc(param_5,param_1,param_2,param_3);
  if (uVar3 == 0x0) {
    return 0x1;
  }
  iVar1 = (uVar3 + 0xc);
  if ((-0x1 < iVar1) && (true)) {
    if (iVar1 < 0x2) {
      return 0x1;
    }
    if ((0x0 < iVar1 + -0x1) && (iVar2 = iVar1 + -0x3, iVar2 == 0x0 || iVar1 + -0x2 < 0x1)
       ) {
      pass1_1008_e9a4(param_1,(param_1 >> 0x10),uVar3,param_4,param_5);
      return iVar2;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_e164(param_1: u32,param_2: u16,param_3: u8)
{
  let puVar1: u32;
  code **ppcVar2;
  astruct_215 *uVar5;
  astruct_215 *paVar3;
  astruct_216 *paVar4;
  let puVar5: *mut u8
  let puVar6: *mut u8
  let puVar7: *mut u8
  let uVar8: u16;
  astruct_214 *uVar11;
  astruct_215 *paVar9;
  astruct_213 *iVar12;
  let uVar10: u16;
  let uVar12: u16;
  let uVar13: u32;
  let local_118: [u8;112];
  let lStack6: i32;
  astruct_216 *iVar1;
  
  uVar10 = (param_1 >> 0x10);
  uVar11 = (astruct_214 *)param_1;
  lStack6 = pass1_1008_e8cc(param_2,param_1,uVar11->field_0x1a,uVar11->field_0x16);
  uVar8 = (lStack6 >> 0x10);
  uVar5 = (astruct_215 *)lStack6;
  puVar5 = (uVar8 | uVar5);
  if (lStack6 == 0x0) {
    pass1_1008_e852(uVar11,uVar10,uVar11->field_0x16,param_2,puVar5);
    paVar3 = uVar5;
    puVar6 = puVar5;
    pass1_1008_e852(uVar11,uVar10,uVar11->field_0x1a,param_2,puVar5);
    paVar9 = paVar3;
    puVar7 = puVar6;
    mem_op_1000_179c(0x14,puVar6,0x1000);
    uVar8 = puVar7 | paVar9;
    if (uVar8 == 0x0) {
      paVar9 = (astruct_215 *)0x0;
      uVar8 = 0x0;
    }
    else {
      struct_1008_dc90((u16 *)CONCAT22(puVar7,paVar9),
                       CONCAT13((char)(puVar6 >> 0x8),CONCAT12((char)puVar6,paVar3))
                       ,CONCAT22(puVar5,uVar5));
    }
    lStack6 = CONCAT22(uVar8,paVar9);
    paVar9->field_0xc = 0x1;
    uVar13 = pass1_1030_8326();
    uVar12 = (lStack6 >> 0x10);
    iVar12 = (astruct_213 *)lStack6;
    iVar12->field_0xe = uVar13;
    iVar12->field_0x10 = (uVar13 >> 0x10);
    puVar1 = uVar11->field_0xa;
    ppcVar2 = (code **)(*uVar11->field_0xa + 0x4);
    (**ppcVar2)(0x1030,puVar1,(puVar1 >> 0x10),iVar12,uVar12);
  }
  else {
    iVar1 = (astruct_216 *)uVar5->field_0xc;
    paVar4 = iVar1 + -0x1;
    if (paVar4 == (astruct_216 *)0x0) {
      return;
    }
    if (((0x0 < paVar4) && (!SBORROW2(paVar4,0x1))) &&
       ((iVar1 + -0x2) < 0x2)) {
      uVar5->field_0x12 = 0x1;
    }
    uVar5->field_0xc = 0x1;
  }
  uVar12 = (lStack6 >> 0x10);
  struct_1030_e2be((astruct_100 *)CONCAT22(param_2,local_118),0x1,
                   (lStack6 + 0x8),(lStack6 + 0x4),param_2,
                   param_3);
  uVar13 = pass1_1030_8326();
  pass1_1030_8372(_PTR_LOOP_1050_5748,uVar13 + 0x1,CONCAT22(param_2,local_118));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_e2a4(param_1: u32,param_2: u32,param_3: u32) -> u16

{
  let iVar1: i16;
  let iVar2: i16;
  let unaff_SS: u16;
  char *pcVar3;
  let lVar4: i32;
  let uVar5: u32;
  
  uVar5 = param_2;
  pcVar3 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  iVar1 = pass1_1000_3d7a(pcVar3,uVar5);
  if ((iVar1 == 0x0) || (iVar1 = pass1_1000_3d7a(param_3,param_2), iVar1 == 0x0)) {
    return 0x0;
  }
  lVar4 = pass1_1008_e8cc(unaff_SS,param_1,param_2,param_3);
  if (lVar4 != 0x0) {
    iVar1 = (lVar4 + 0xc);
    iVar2 = iVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x2;
    }
    if (iVar2 < 0x1) {
      return 0x0;
    }
    if (SBORROW2(iVar2,0x1)) {
      return 0x0;
    }
    if (0x1 < iVar1 + -0x2) {
      return 0x0;
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_e320(astruct_102 *param_1,param_2: u32,param_3: u32,param_4: u16)
{
  astruct_103 *paVar1;
  astruct_103 *uVar2;
  let uVar3: u16;
  let uVar4: u16;
  astruct_102 *iVar5;
  astruct_102 *uVar6;
  char *pcVar5;
  let lVar6: i32;
  let uVar7: u32;
  
  uVar6 = (astruct_102 *)(param_1 >> 0x10);
  iVar5 = (astruct_102 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x1e,0x1000);
  &iVar5->field_0x1e = 0x0;
  uVar7 = param_2;
  pcVar5 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  uVar4 = (pcVar5 >> 0x10);
  uVar2 = (astruct_103 *)pass1_1000_3d7a(pcVar5,uVar7);
  if ((uVar2 != (astruct_103 *)0x0) &&
     (uVar2 = (astruct_103 *)pass1_1000_3d7a(param_3,param_2), uVar2 != (astruct_103 *)0x0
     )) {
    lVar6 = pass1_1008_e8cc(param_4,param_1,param_2,param_3);
    uVar3 = (lVar6 >> 0x10);
    uVar2 = (astruct_103 *)lVar6;
    uVar4 = uVar3 | uVar2;
    if ((uVar4 != 0x0) &&
       (((paVar1 = (astruct_103 *)uVar2->field_0xc, uVar2 = paVar1,
         paVar1 != (astruct_103 *)0x0 &&
         (uVar2 = (astruct_103 *)(&paVar1[-0x1].field_0xc + 0x1),
         uVar2 != (astruct_103 *)0x0)) &&
        (uVar2 = (astruct_103 *)&paVar1[-0x1].field_0xc, uVar2 != (astruct_103 *)0x0)))) {
      uVar2 = (astruct_103 *)&paVar1[-0x1].field_0xb;
    }
  }
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  iVar5->field_0x1e = uVar2;
  iVar5->field_0x20 = uVar4;
  return;
}



fn pass1_1008_e3ec(param_1: u32,param_2: *mut u32,param_3: *mut u32,param_4: u16)
{
  let uVar1: u32;
  let puVar2: u32;
  code **ppcVar3;
  astruct_219 *paVar4;
  let puVar5: u32;
  astruct_219 *puVar4;
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let uVar6: u16;
  let uVar7: u16;
  let extraout_DX_01: u16;
  let puVar8: *mut u8
  let extraout_DX_02: *mut u8
  let extraout_DX_03: u16;
  let uVar9: u16;
  let extraout_DX_04: u16;
  astruct_218 *iVar10;
  let uVar10: u16;
  astruct_219 local_14;
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let iStack4: i16;
  
  uVar10 = (param_1 >> 0x10);
  iVar10 = (astruct_218 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar5 = iVar10->field_0xe;
  puVar8 = *(uchar **)(&iVar10->field_0xe + 0x2);
  if ((puVar8 | puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar8 = extraout_DX;
  }
  mem_op_1000_179c(0x18,puVar8,0x1000);
  if ((puVar8 | puVar5) == 0x0) {
    puVar5 = 0x0;
    uVar6 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)
                        CONCAT13((char)(puVar8 >> 0x8),CONCAT12((char)puVar8,puVar5)
                                ),0x5,0x5);
    uVar6 = extraout_DX_00;
  }
  *(u32 **)&iVar10->field_0xe = puVar5;
  (&iVar10->field_0xe + 0x2) = uVar6;
  pass1_1028_dc52((astruct_92 *)
                  CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,&local_14)),0x1,
                  0x0,0x400);
  while( true ) {
    uVar7 = uVar6;
    paVar4 = &local_14;
    pass1_1028_e4ec(CONCAT22(param_4,paVar4));
    if ((uVar7 | paVar4) == 0x0) break;
    uVar6 = uVar7 | paVar4;
    if (*(long *)(paVar4 + 0x40) != 0x8000002) {
      uVar1 = paVar4->field_0x4;
      puVar2 = iVar10->field_0xe;
      ppcVar3 = (code **)(*iVar10->field_0xe + 0xc);
      (**ppcVar3)(0x28,puVar2,(puVar2 >> 0x10),uVar1,
                  (uVar1 >> 0x10));
      uVar6 = extraout_DX_01;
    }
  }
  *param_3 = iVar10->field_0xe;
  uVar6 = (&iVar10->field_0x12 + 0x2);
  puVar5 = iVar10->field_0x12;
  puVar8 = (uVar6 | puVar5);
  if (puVar8 != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)(0x28,puVar5,uVar6,0x1);
    puVar8 = extraout_DX_02;
  }
  mem_op_1000_179c(0x18,puVar8,0x1000);
  if ((puVar8 | puVar5) == 0x0) {
    puVar5 = 0x0;
    uVar9 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)
                        CONCAT13((char)(puVar8 >> 0x8),CONCAT12((char)puVar8,puVar5)
                                ),0x5,0x5);
    uVar9 = extraout_DX_03;
  }
  *(u32 **)&iVar10->field_0x12 = puVar5;
  (&iVar10->field_0x12 + 0x2) = uVar9;
  uStack12 = uStack8;
  uStack10 = uStack6;
  if (iStack4 != 0x0) {
    uStack12 = 0x1;
    uStack6 = 0x0;
    uStack10 = uStack6;
  }
  while( true ) {
    puVar4 = &local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar4));
    if ((uStack6 | puVar4) == 0x0) break;
    uVar1 = puVar4->field_0x4;
    puVar2 = iVar10->field_0x12;
    ppcVar3 = (code **)(*iVar10->field_0x12 + 0xc);
    (**ppcVar3)(0x28,puVar2,(puVar2 >> 0x10),uVar1,
                (uVar1 >> 0x10));
    uStack6 = extraout_DX_04;
  }
  *param_2 = iVar10->field_0x12;
  return;
}


fn pass1_1008_e5da(param_1: u32,param_2: u32,param_3: HFILE16,param_4: u16)
{
  let uVar1: u32;
  let BVar2: bool;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let local_30: [u32;0x2];
  let local_28: u32;
  let local_24: [u32;0x2];
  let local_1c: [u16;0x3];
  let local_16: [u16;0x3];
  let uStack16: u32;
  let local_c: [u8;8];
  let uStack4: u16;
  
  BVar2 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar2 != 0x0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (*(long *)(iVar4 + 0xa) == 0x0) {
      uStack4 = 0x0;
    }
    else {
      uVar1 = (iVar4 + 0xa);
      uStack4 = (uVar1 + 0x8);
    }
    local_1c[0] = uStack4;
    uVar6 = param_2;
    uVar7 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c
                      (uVar6,uVar7,local_1c,param_4,0x2,param_3);
    if (BVar2 != 0x0) {
      pass1_1008_5784(CONCAT22(param_4,local_c),(iVar4 + 0xa));
      do {
        puVar3 = local_c;
        pass1_1008_5b12(puVar3,param_4);
        uStack16 = CONCAT22(extraout_DX,puVar3);
        if ((extraout_DX | puVar3) == 0x0) {
          return;
        }
        local_24[0] = (puVar3 + 0x4);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,local_24,param_4,0x4,param_3);
        if (BVar2 == 0x0) break;
        local_28 = (uStack16 + 0x8);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,&local_28,param_4,0x4,param_3);
        if (BVar2 == 0x0) break;
        local_16[0] = (uStack16 + 0xc);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,local_16,param_4,0x2,param_3);
        if (BVar2 == 0x0) break;
        local_30[0] = (uStack16 + 0xe);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,local_30,param_4,0x4,param_3);
        if (BVar2 == 0x0) break;
        local_16[0] = (uStack16 + 0x12);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,local_16,param_4,0x2,param_3);
      } while (BVar2 != 0x0);
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}


fn pass1_1008_e852(param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: u16)
{
  let puVar1: *mut u8;
  let iVar2: i16;
  char *pcVar3;
  let local_14: [u8;12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
  do {
    puVar1 = local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar1));
    if ((param_5 | puVar1) == 0x0) {
      return;
    }
    pcVar3 = pass1_1038_4d28(CONCAT22(param_5,puVar1));
    param_5 = (pcVar3 >> 0x10);
    iVar2 = pass1_1000_3d7a(param_3,pcVar3 & 0xffff | param_5 << 0x10);
  } while (iVar2 != 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

long 
pass1_1008_e8cc(param_1: u16,param_2: u32,param_3: u32,param_4: u32)

{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let lVar7: i32;
  char *pcVar8;
  char *pcVar9;
  let uStack22: u32;
  let uStack18: u32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_1,local_a),(param_2 + 0xa));
  while( true ) {
    lVar7 = pass1_1008_5b12(local_a,param_1);
    uVar5 = (lVar7 >> 0x10);
    uVar2 = lVar7;
    uVar6 = uVar5 | uVar2;
    if (lVar7 == 0x0) {
      return 0x0;
    }
    uVar1 = (uVar2 + 0x4);
    uVar3 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    uStack18 = CONCAT22(uVar6,uVar3);
    uVar1 = (uVar2 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    uStack22 = CONCAT22(uVar6,uVar3);
    pcVar8 = pass1_1038_4d28(uStack18);
    pcVar9 = pass1_1038_4d28(uStack22);
    iVar4 = pass1_1000_3d7a(param_4,pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3,pcVar9), iVar4 == 0x0))
    break;
    iVar4 = pass1_1000_3d7a(param_3,pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4,pcVar9), iVar4 == 0x0))
    {
      return lVar7;
    }
  }
  return lVar7;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_e9a4(param_1: u16,param_2: u16,param_3: u32,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u8
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u32;
  let iStack20: i16;
  let uStack16: u32;
  let uStack6: u32;
  
  uVar7 = pass1_1030_8326();
  uVar6 = (param_3 >> 0x10);
  iVar5 = param_3;
  puVar1 = (iVar5 + 0xe);
  uVar2 = uVar7 - *puVar1;
  puVar4 = (((uVar7 >> 0x10) - (iVar5 + 0x10)) -
                    (uVar7 < *puVar1));
  uStack6 = CONCAT22(puVar4,uVar2);
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,puVar4,param_4);
  uStack16 = 0x0;
  if ((PTR_LOOP_1050_13ae < 0x1) || (SBORROW2(PTR_LOOP_1050_13ae,0x1)))
  goto LAB_1008_ea2b;
  if (PTR_LOOP_1050_13ae == &PTR_LOOP_1050_0002 ||
      (PTR_LOOP_1050_13ae + -0x1) < 0x1) {
    if ((iVar5 + 0x12) == 0x0) {
LAB_1008_ea20:
      uVar3 = 0x1e;
    }
    else {
      uVar3 = 0xa;
    }
  }
  else {
    if (PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0002 + 0x1)) {
      if ((iVar5 + 0x12) == 0x0) {
        uVar3 = 0x28;
      }
      else {
        uVar3 = 0x14;
      }
    }
    else {
      if (PTR_LOOP_1050_13ae != &DAT_1050_0004) goto LAB_1008_ea2b;
      if ((iVar5 + 0x12) != 0x0) goto LAB_1008_ea20;
      uVar3 = 0x32;
    }
  }
  uStack16 = uVar3;
LAB_1008_ea2b:
  if (uStack16 < uStack6) {
    pass1_1008_612e(0x1,0x64,uVar2);
    iStack20 = 0x0;
    iVar5 = (iVar5 + 0xc);
    if (iVar5 == 0x2) {
      iStack20 = 0x32;
    }
    else {
      if (iVar5 == 0x3) {
        iStack20 = 0x19;
      }
    }
    if (uStack6 < iStack20) {
      return;
    }
  }
  return;
}



fn pass1_1008_ea86(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1008_ddca(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_eabc(param_1: i16,param_2: u16,param_3: u16) -> u16

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  pass1_1008_3e38((u16 *)CONCAT22(param_2,param_1 + 0xc));
  CONCAT22(param_2,param_1) = 0xeb1a;
  (param_1 + 0x2) = 0x1008;
  return CONCAT22(param_2,param_1);
}



fn pass1_1008_eaf4(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_eb2a(param_1: i16,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xc) = 0x0;
  CONCAT22(param_2,param_1) = 0xec00;
  (param_1 + 0x2) = 0x1008;
  return;
}



fn pass1_1008_eb5c(param_1: u16,param_2: u16,param_3: i16) -> u32

{
  return CONCAT22(0x1050,param_3 * 0x10 + 0xd0e);
}



fn pass1_1008_eb6e(void) -> u16

{
  return 0x5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1008_eb74(param_1: u32,param_2: i16,uchar *param_3,param_4: i16,param_5: u16)
{
  (param_1 + 0xa) = param_2;
  if (param_2 != 0x0) {
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_5,param_3,param_4);
    pass1_1010_c312();
  }
  return;
}



fn pass1_1008_ebda(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_ec10(param_1: i16,param_2: u16,param_3: u16) -> u16

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  CONCAT22(param_2,param_1) = 0xec62;
  (param_1 + 0x2) = 0x1008;
  return CONCAT22(param_2,param_1);
}



fn pass1_1008_ec3c(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1008_ec94(param_1: *mut u16)
{
  *param_1 = 0xefc4;
  (param_1 + 0x2) = 0x1008;
  pass1_1010_3880(param_1);
  return;
}



fn pass1_1008_ed00(param_1: *mut u16,param_2: u16)
{
  *param_1 = 0xef9c;
  (param_1 + 0x2) = 0x1008;
  pass1_1010_2db2(param_1,param_2);
  return;
}



fn pass1_1008_ed62(param_1: u32,param_2: i16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x16) = param_2 * 0x8 + 0xd5e;
  (iVar1 + 0x18) = &USHORT_1050_1050;
  (iVar1 + 0x12) = (param_2 * 0x8 + 0xd64);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1008_ed8a(param_1: *mut u32,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: i16,param_7: u16)

{
  code **ppcVar1;
  let cVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let bVar5: bool;
  let uVar6: u32;
  let uVar7: u32;
  
  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_0df6 == 0x0) {
      ppcVar1 = (code **)(*param_1 + 0x18);
      uVar6 = (**ppcVar1)();
      _PTR_LOOP_1050_0df6 =
           mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar6,param_7,
                           (uVar6 >> 0x10),param_6);
    }
    uVar6 = (param_1 + 0xc);
    uVar7 = pass1_1010_2e02(_PTR_LOOP_1050_0df6,(uVar6 + 0x12));
    uVar3 = param_2 + 0x1;
    uVar4 = param_3 + (0xfffe < param_2);
    for (cVar2 = ((char)param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 += -0x1) {
      bVar5 = CARRY2(uVar3,uVar3);
      uVar3 *= 0x2;
      uVar4 = uVar4 * 0x2 + bVar5;
    }
    pass1_1010_2e30(_PTR_LOOP_1050_0df6,uVar3 | uVar7,
                    uVar4 | (uVar7 >> 0x10),(param_5 * 0x8 + 0xd64));
  }
  return;
}



fn pass1_1008_ee14(param_1: u32,param_2: u16)
{
  let puVar1: *mut u8;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let puVar5: *mut u16;
  u8 local_1c [0x1a];
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x56) == 0x0) {
    puVar5 = struct_1008_ec72((u16 *)CONCAT22(param_2,local_1c));
    uVar2 = (puVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e(CONCAT22(param_2,puVar1),0x0,0x0,0x0,puVar1);
    (iVar3 + 0x56) = puVar1;
    (iVar3 + 0x58) = uVar2;
    pass1_1008_ec94((u16 *)CONCAT22(param_2,local_1c));
  }
  return;
}

