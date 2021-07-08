
pub fn close_file_1008_496c(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u32;
  let ppcVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  *param_1 = &ctx.PTR_LOOP_1050_4c4c;
  (iVar5 + 0x2) = 0x1008;
  puVar1 = (iVar5 + 0x4);
  uVar2 = (iVar5 + 0x6);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = *puVar1;
    (**ppcVar4)();
  }
  fn_ptr_1000_17ce((iVar5 + 0x8),0x1000);
  if ((iVar5 + 0x1a) != 0x0) {
    uVar3 = (iVar5 + 0x1a);
    call_fn_ptr_1000_0dc6(uVar3,(uVar3 >> 0x10),0x1000);
  }
  if ((iVar5 + 0xc) != -0x1) {
    _lclose16(0x1000);
  }
  *param_1 = 0x389a;
  (iVar5 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_49e8(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  HFILE16 HVar1;
  let iVar2: i16;
  let uVar3: u32;
  let uVar4: u32;
  let puVar5: *mut u8;
  let puVar6: *mut u8;
  let extraout_DX: *mut u8;
  let iVar7: i16;
  let unaff_DI: i16;
  let uVar8: u16;
  let h_file: u16;
  let unaff_SS: u16;
  let lVar9: i32;
  let local_18: i16;
  let uStack22: u32;
  let uStack10: u16;
  let puStack8: *mut u8;
  let uStack6: u32;
  
  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  if ((iVar7 + 0x8) != 0x0) {
    if ((iVar7 + 0x1e) != 0x0) {
      return param_3;
    }
    h_file = param_2;
    if ((iVar7 + 0xc) == -0x1) {
      h_file = ctx.s_tile2_bmp_1050_1538;
      HVar1 = _lopen16(param_2,0x0);
      *(HFILE16 *)(iVar7 + 0xc) = HVar1;
      if (HVar1 == 0xffff) {
        return param_3;
      }
    }
    uStack6 = 0x0;
    lVar9 = WIN16_hread(h_file,0xe,ZEXT24(&local_18) << 0x10);
    param_3 = (lVar9 >> 0x10);
    if ((lVar9 == 0xe) && ((uchar *)param_3 == 0x0)) {
      uStack6 = uStack22;
      if (local_18 == &ctx.PTR_LOOP_1050_4d42) {
        _llseek16((HFILE16)s_tile2_bmp_1050_1538,0x0,0x0);
        lVar9 = mem_op_1000_0a48(0x1,uStack6,(uStack6 >> 0x10),
                                 ctx._PTR_LOOP_1050_5f2c,0x1000);
        puVar6 = (lVar9 >> 0x10);
        (iVar7 + 0x1a) = lVar9;
        *(uchar **)(iVar7 + 0x1c) = puVar6;
        if ((puVar6 | (iVar7 + 0x1a)) == 0x0) {
          return puVar6;
        }
        lVar9 = WIN16_hread(0x1000,(SEGPTR)uStack6,
                            CONCAT22((iVar7 + 0x1a),
                                     (uStack6 >> 0x10)));
        puVar5 = (lVar9 >> 0x10);
        uStack10 = lVar9;
        puStack8 = puVar5;
        _lclose16((HFILE16)s_tile2_bmp_1050_1538);
        (iVar7 + 0xc) = 0xffff;
        (iVar7 + 0x1e) = 0x1;
        (iVar7 + 0xe) = (iVar7 + 0x1a);
        uVar3 = (iVar7 + 0x1a);
        iVar2 = uVar3;
        uVar3 &= 0xffff0000;
        (iVar7 + 0x12) = uVar3 | iVar2 + 0xe;
        uVar3 |= iVar2 + 0x436;
        (iVar7 + 0x16) = uVar3;
        mem_op_1000_179c(0x14,puVar5,0x1000);
        puVar6 = (puVar5 | uVar3);
        if (puVar6 == 0x0) {
          (iVar7 + 0x4) = 0x0;
        }
        else {
          uVar4 = (iVar7 + 0x12);
          uVar4 = uVar4 & 0xffff0000 | (uVar4 + 0x28);
          struct_op_1008_4c98((uVar3 & 0xffff | ZEXT24(puVar5) << 0x10),
                              uVar4,0x100);
          (iVar7 + 0x4) = uVar4;
          *(uchar **)(iVar7 + 0x6) = extraout_DX;
          puVar6 = extraout_DX;
        }
        if ((iVar7 + 0x22) != 0x0) {
          pass1_1008_4b8e(param_1,puVar6,unaff_DI,unaff_SS);
          return puVar6;
        }
        return puVar6;
      }
    }
    _lclose16((HFILE16)s_tile2_bmp_1050_1538);
    (iVar7 + 0xc) = 0xffff;
  }
  return param_3;
}


pub fn file_1008_4c26(param_1: u32,param_2: u8) -> u32

{
  close_file_1008_496c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn file_1008_6414(param_1: *mut u32,param_2: u32,param_3: u16,param_4: *mut u8)
{
  let ppcVar1: u32;
  let puVar2: *mut u8;
  let uVar3: u16;
  let extraout_DX: u16;
  let iVar4: i16;
  let uVar5: u16;
  let paStack42: &mut Struct76;
  let local_26: [u8;24];
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x0;
  (iVar4 + 0x4) = 0x0;
  puVar2 = local_26;
  struct_op_1008_48fe(CONCAT22(param_3,puVar2),0x1,param_2,
                      param_4);
  mem_op_1000_179c(0x1e,param_4,0x1000);
  paStack42 = CONCAT22(param_4,puVar2);
  uVar3 = param_4 | puVar2;
  if (uVar3 == 0x0) {
    *param_1 = 0x0;
  }
  else {
    puVar2 = local_26;
    struct_op_1008_3f92(paStack42,CONCAT22(param_3,puVar2));
    param_1 = puVar2;
    (iVar4 + 0x2) = uVar3;
  }
  ppcVar1 = (*param_1 + 0x14);
  (**ppcVar1)();
  (iVar4 + 0x4) = puVar2;
  (iVar4 + 0x6) = extraout_DX;
  close_file_1008_496c(local_26,param_3);
  return;
}


pub fn close_file_1008_6dd0(param_1: *mut u32,param_2: HFILE16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x4) != -0x1) {
    _lclose16(param_2);
    (param_1 + 0x4) = 0xffff;
  }
  fn_ptr_1000_17ce(*param_1,0x1000);
  return;
}



pub fn file_fn_1008_6e02(uint32_t *param_1,LPCSTR in_string,param_3: u16) -> bool

{
  let var1: i16;
  let var2: bool;
  let extraout_DX: *mut u8;
  let unaff_DI: i16;
  let uVar1: u16;
  let local_4: [u8;2];
  
  ctx.PTR_LOOP_1050_0310 = 0x0;
  var1 = write_to_file_1008_70a6(param_1,in_string);
  if (var1 != 0x0) {
    uVar1 = (param_1 >> 0x10);
    pass1_1008_72a8();
    var1 = pass1_1008_7006(param_1,uVar1,CONCAT22(param_3,local_4),extraout_DX,
                           unaff_DI,param_3);
    if ((var1 != 0x0) && (var1 = file_fn_1008_6eee(param_1,local_4,param_3), var1 != 0x0))
    {
      var2 = file_fn_1008_726c(param_1,uVar1,(HFILE16)in_string);
      if (var2 == 0x0) {
        return 0x0;
      }
      return 0x1;
    }
    _lclose16((HFILE16)in_string);
  }
  return 0x0;
}



pub fn read_file_1008_6e78(uint32_t param_1,param_2: u16,LPCSTR in_string,param_4: u16) -> bool

{
  let b_var1: bool;
  let i_var2: i16;
  let var3: *mut u8;
  let extraout_DX: *mut u8;
  let unaff_DI: i16;
  let local_4: [u8;2];
  
  ctx.PTR_LOOP_1050_0310 = 0x0;
  b_var1 = read_file_1008_7146(param_1,param_2,in_string,param_4);
  if (b_var1 != 0x0) {
    pass1_1008_72a8();
    i_var2 = pass1_1008_7056(param_1,param_2,CONCAT22(param_4,local_4),extraout_DX,
                             unaff_DI,param_4);
    if (i_var2 != 0x0) {
      var3 = local_4;
      read_file_1008_6f7a(param_1,param_2,CONCAT22(param_4,var3),param_4);
      if (var3 != 0x0) {
        b_var1 = file_fn_1008_726c(param_1,param_2,(HFILE16)in_string);
        if (b_var1 == 0x0) {
          return 0x0;
        }
        return 0x1;
      }
    }
    _lclose16((HFILE16)in_string);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn file_fn_1008_6eee(param_1: u16,param_2: u16,param_3: u32)
{
  let BVar1: bool;
  let uVar2: u16;
  let in_DX: *mut u8;
  let unaff_SS: u16;
  let uVar3: u32;
  let local_e: [u8;4];
  let uStack10: u32;
  let puStack6: u32;
  
  puStack6 = *_PTR_LOOP_1050_5748;
  uStack10 = *puStack6;
  pass1_1020_a43e(unaff_SS,in_DX,CONCAT22(unaff_SS,local_e));
  BVar1 = pass1_1028_d7a0(uStack10,(uStack10 >> 0x10),param_3,
                          unaff_SS);
  if (BVar1 != 0x0) {
    BVar1 = pass1_1030_5c1a(ctx.PTR__LOOP_1050_5736,param_3,unaff_SS);
    if (BVar1 != 0x0) {
      uVar3 = write_to_file_1028_dce2(ctx.PTR__LOOP_1050_65e2,param_3,unaff_SS);
      if ((uVar3 >> 0x10) != 0x0) {
        uVar2 = pass1_1038_7b20(ctx.PTR__LOOP_1050_5a64,param_3,unaff_SS);
        if (uVar2 != 0x0) {
          BVar1 = pass1_1020_a644(local_e,unaff_SS,param_3,unaff_SS);
          if (BVar1 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
read_file_1008_6f7a(param_1: u16,param_2: u16,param_3: u32,
                   param_4: u16)

{
  let var5: u16;
  let i_var3: i16;
  let b_var4: bool;
  let in_DX: *mut u8;
  let uVar1: u16;
  let puVar2: *mut u16;
  let local_e: [u8;4];
  let uStack10: u32;
  let puStack6: u32;
  
  puStack6 = *_PTR_LOOP_1050_5748;
  uStack10 = *puStack6;
  puVar2 = pass1_1020_a43e(param_4,in_DX,CONCAT22(param_4,local_e));
  uVar1 = (puVar2 >> 0x10);
  var5 = read_file_1028_d7ba(uStack10,(uStack10 >> 0x10),param_3,param_4,
                             puVar2);
  if (var5 != 0x0) {
    var5 = read_file_1030_5c52(ctx.PTR__LOOP_1050_5736,param_3,var5,param_4);
    if (var5 != 0x0) {
      read_file_1028_def2(ctx.PTR__LOOP_1050_65e2,param_3,param_4,var5);
      if (var5 != 0x0) {
        i_var3 = read_file_1038_7c02(ctx.PTR__LOOP_1050_5a64,param_3,var5,uVar1);
        if (i_var3 != 0x0) {
          b_var4 = read_file_1020_a65e(CONCAT22(param_4,local_e),param_3,param_4,
                                       local_e);
          if (b_var4 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}


pub fn write_to_file_1008_70a6(param_1: *mut u32,LPCSTR param_2) -> u16

{
  HFILE16 HVar1;
  let iVar2: i16;
  let uVar3: u16;
  let mut pCVar4: String;
  let unaff_SS: u16;
  let in_AF: u8;
  let lVar5: i32;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  pCVar4 = param_2;
  if ((iVar2 + 0x4) != -0x1) {
    pCVar4 = ctx.s_tile2_bmp_1050_1538;
    _lclose16((HFILE16)param_2);
    (iVar2 + 0x4) = 0xffff;
  }
  HVar1 = _lcreat16(pCVar4,0x0);
  *(HFILE16 *)(iVar2 + 0x4) = HVar1;
  if (HVar1 == 0xffff) {
    ctx.PTR_LOOP_1050_0310 = 0x6cf;
  }
  else {
    ctx.PTR_LOOP_1050_0312 = &DAT_1050_0004;
    sys_1000_3f9c((uchar *)0x65a0,ctx.data_seg,
                  ctx._PTR_s_SC_03d_1050_0314_1050_031c,
                  (ctx.PTR__s_SC_03d_1050_0314_1050_031c >> 0x10),0x4,
                  &stack0xfffe,uVar3,0x1000,unaff_SS,in_AF);
    pCVar4 = str_op_1000_3da4(0x105065a0);
    lVar5 = _hwrite16(0x1000,pCVar4,CONCAT22(0x65a0,pCVar4 >> 0xf));
    if ((lVar5 == pCVar4) && (true)) {
      return 0x1;
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  return 0x0;
}



pub fn read_file_1008_7146(int32_t param_1,param_2: u16,LPCSTR param_3,param_4: u16) -> bool

{
  HFILE16 HVar1;
  let iVar2: i16;
  let mut path: String;
  
  path = param_3;
  if ((param_1 + 0x4) != -0x1) {
    path = ctx.s_tile2_bmp_1050_1538;
    _lclose16((HFILE16)param_3);
    (param_1 + 0x4) = 0xffff;
  }
  HVar1 = _lopen16(path,0x0);
  *(HFILE16 *)(param_1 + 0x4) = HVar1;
  if (HVar1 == 0xffff) {
    ctx.PTR_LOOP_1050_0310 = 0x6cf;
  }
  else {
    iVar2 = read_file_1008_71a0(CONCAT22(param_2,param_1),param_4);
    if (iVar2 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_71a0(param_1: u32,param_2: u16) -> u16

{
  let buffer: u16;
  let uVar1: u16;
  let in_AF: u8;
  let lVar2: i32;
  let iStack26: i16;
  let iStack24: i16;
  let iStack22: i16;
  char local_e [0x9];
  let uStack5: u8;
  let uStack4: u16;
  
  uStack4 = 0x1;
  buffer = str_op_1000_3da4(0x105065a0);
  iStack22 = 0x0;
  lVar2 = WIN16_hread(0x1000,buffer,CONCAT22(local_e,buffer >> 0xf));
  uVar1 = lVar2;
  if ((buffer < lVar2) && ((true || (buffer < uVar1)))) {
    uVar1 = buffer;
  }
  iStack24 = uVar1 - 0x2;
  if (iStack24 < 0x0) {
    iStack24 = 0x0;
  }
  iStack26 = 0x2;
  while (iStack24 != 0x0) {
    iStack22 = iStack22 * 0xa + local_e[iStack26] + -0x30;
    iStack26 += 0x1;
    iStack24 = iStack24 + -0x1;
  }
  if (iStack22 == 0x1) {
    ctx.PTR_LOOP_1050_0312 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  }
  else {
    if (iStack22 == 0x4) {
      ctx.PTR_LOOP_1050_0312 = &DAT_1050_0004;
    }
    else {
      uStack5 = 0x0;
      ctx.PTR_LOOP_1050_0312 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
      uStack4 = 0x0;
    }
  }
  sys_1000_3f9c((uchar *)0x65a0,ctx.data_seg,
                ctx._PTR_s_SC_03d_1050_0314_1050_031c,
                (ctx.PTR__s_SC_03d_1050_0314_1050_031c >> 0x10),
                ctx.PTR_LOOP_1050_0312,&stack0xfffe,(param_1 >> 0x10),
                0x1000,param_2,in_AF);
  return uStack4;
}



pub fn file_fn_1008_726c(uint32_t param_1,param_2: u16,HFILE16 file_handle) -> bool

{
  HFILE16 HVar1;
  
  if ((param_1 + 0x4) != -0x1) {
    HVar1 = _lclose16(file_handle);
    if (HVar1 == 0xffff) {
      ctx.PTR_LOOP_1050_0310 = 0x6d1;
      return 0x0;
    }
    (param_1 + 0x4) = 0xffff;
    ctx.PTR_LOOP_1050_0310 = 0x0;
  }
  return 0x1;
}


pub fn file_1008_7548(param_1: u32,long *param_2,param_3: HFILE16,param_4: u16)
{
  let ppcVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let local_1c: u32;
  let local_18: [u16;0x5];
  let uStack14: u32;
  let uStack10: u32;
  let local_6: u32;
  
  local_6 = 0x0;
  uVar7 = param_1;
  uVar2 = (param_1 >> 0x10);
  BVar3 = read_file_1008_7dee(uVar7,uVar2,&local_6,0x0,param_4,0x4,param_3);
  if (BVar3 == 0x0) {
    return;
  }
  if (local_6 != 0x0) {
    uVar5 = local_6;
    if (local_6 < 0xc8) {
      local_6._2_2_ = 0x0;
      uVar5 = 0xc8;
    }
    uVar4 = uVar5;
    uStack10 = uVar5 & 0xffff | ZEXT24(local_6._2_2_) << 0x10;
    if (*param_2 == 0x0) {
      param_3 = 0x1000;
      mem_op_1000_179c(0x1e,local_6._2_2_,0x1000);
      uVar6 = local_6._2_2_ | uVar4;
      if (uVar6 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_3 = 0x1020;
        struct_1020_c444(CONCAT22(local_6._2_2_,uVar4),0x64,uStack10);
        param_2 = uVar4;
        (param_2 + 0x2) = uVar6;
      }
    }
    ppcVar1 = (*param_2 + 0x24);
    (**ppcVar1)(param_3,*param_2);
    for (uStack14 = 0x0; uStack14 < local_6; uStack14 += 0x1) {
      BVar3 = read_file_1008_7dee(uVar7,uVar2,&local_1c,0x0,param_4,0x4,param_3);
      if ((BVar3 == 0x0) ||
         (BVar3 = read_file_1008_7dee(uVar7,uVar2,local_18,0x0,param_4,0x2,param_3
                                     ), BVar3 == 0x0)) {
        ppcVar1 = (*param_2 + 0x1c);
        (**ppcVar1)(param_3,*param_2,(*param_2 >> 0x10));
        return;
      }
      ppcVar1 = (*param_2 + 0x28);
      (**ppcVar1)(param_3,*param_2,(*param_2 >> 0x10),local_18[0],
                  local_1c,(local_1c >> 0x10));
    }
    ppcVar1 = (*param_2 + 0x1c);
    (**ppcVar1)(param_3,*param_2,(*param_2 >> 0x10));
  }
  return;
}


pub fn file_1008_76e4(param_1: u32,long *param_2,param_3: u16,param_4: u16,param_5: u16)
{
  let ppcVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  let extraout_DX: u16;
  let uVar4: u16;
  u8 local_18 [0xe];
  let uStack10: u32;
  let local_6: u32;
  
  local_6 = 0x0;
  uVar4 = (param_1 >> 0x10);
  uVar2 = read_file_1008_7dee(param_1,uVar4,&local_6,0x0,param_4,0x4,
                              param_3);
  if (uVar2 == 0x0) {
    return;
  }
  if (local_6 != 0x0) {
    if (*param_2 == 0x0) {
      param_3 = 0x1000;
      mem_op_1000_179c(0x18,param_5,0x1000);
      if ((param_5 | uVar2) == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_3 = 0x1030;
        struct_op_1030_1cd8(CONCAT22(param_5,uVar2),0x5,local_6);
        param_2 = uVar2;
        (param_2 + 0x2) = extraout_DX;
      }
    }
    ppcVar1 = (*param_2 + 0x14);
    (**ppcVar1)(param_3,*param_2,(*param_2 >> 0x10),local_6);
    for (uStack10 = 0x0; uStack10 < local_6; uStack10 += 0x1) {
      BVar3 = read_file_1008_7dee(param_1,uVar4,local_18,0x0,param_4,0x4,
                                  param_3);
      if (BVar3 == 0x0) {
        return;
      }
      ppcVar1 = (*param_2 + 0x18);
      (**ppcVar1)();
    }
    ppcVar1 = (*param_2 + 0x1c);
    (**ppcVar1)();
  }
  return;
}



pub fn file_1008_77cc(param_1: u32,long *param_2,param_3: *mut u8,param_4: HFILE16,param_5: u16) -> u16

{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  let unaff_SI: u16;
  let unaff_DI: u16;
  let uVar4: u16;
  let uVar5: u16;
  let local_14: [u16;0x2];
  let local_10: [u32;0x2];
  let uStack6: u16;
  let local_4: u16;
  
  local_4 = 0x0;
  uVar4 = param_1;
  uVar5 = (param_1 >> 0x10);
  uVar1 = read_file_1008_7dee(uVar4,uVar5,&local_4,0x0,param_5,0x2,param_4);
  if (uVar1 == 0x0) {
    return 0x0;
  }
  if (local_4 != 0x0) {
    if (*param_2 == 0x0) {
      param_4 = 0x1000;
      mem_op_1000_179c(0xa,param_3,0x1000);
      uVar3 = param_3 | uVar1;
      if (uVar3 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_4 = 0x1020;
        pass1_1020_ba3e((long *)CONCAT22(param_3,uVar1),0x5,0x5,unaff_DI,unaff_SI);
        param_2 = uVar1;
        (param_2 + 0x2) = uVar3;
      }
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 += 0x1) {
      BVar2 = read_file_1008_7dee(uVar4,uVar5,local_14,0x0,param_5,0x2,param_4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      BVar2 = read_file_1008_7dee(uVar4,uVar5,local_10,0x0,param_5,0x4,param_4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      param_4 = 0x1020;
      pass1_1020_bb8a((long *)*param_2,local_10[0],
                      CONCAT22(local_14[0],(local_10[0] >> 0x10)),unaff_DI,
                      param_5);
    }
  }
  return 0x1;
}


u16_t 
write_to_file_1008_7954
          (param_1: u32,param_2: *mut u32,param_3: u16,param_4: HFILE16,
          param_5: u16)

{
  let ppcVar1: u32;
  let BVar2: bool;
  let uVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let local_20: u16;
  let uStack30: u16;
  let local_18: u16;
  let uStack22: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  if (param_2 == 0x0) {
    param_3 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (*param_2 + 0x10);
    (**ppcVar1)(param_4,param_2);
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_3);
  uVar5 = (param_1 >> 0x10);
  local_18 = param_3;
  uStack22 = uVar4;
  BVar2 = write_to_file_1008_7e1c
                    (param_1,uVar5,&local_18,param_5,0x4,param_4);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return uVar4;
      }
      ppcVar1 = (*param_2 + 0x4);
      uVar3 = uStack6;
      (**ppcVar1)();
      local_20 = uVar3;
      uVar4 = extraout_DX_00;
      uStack30 = extraout_DX_00;
      local_18 = local_20;
      uStack22 = extraout_DX_00;
      BVar2 = write_to_file_1008_7e1c
                        (param_1,uVar5,&local_20,param_5,0x4,
                         param_4);
      if (BVar2 == 0x0) break;
      uStack10 += 0x1;
    }
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d0;
  return uVar4;
}


pub fn write_to_file_1008_7a22(param_1: u32,param_2: i32,param_3: HFILE16,param_4: u16)
{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_24: [u32;0x2];
  let local_1c: [u16;0x5];
  let local_12: u16;
  let local_10: u32;
  let uStack10: u16;
  let uStack6: u16;
  let uStack4: u16;
  
  if (param_2 == 0x0) {
    uStack4 = 0x0;
  }
  else {
    uStack4 = (param_2 + 0x4);
  }
  local_12 = uStack4;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  BVar1 = write_to_file_1008_7e1c
                    (uVar2,uVar3,&local_12,param_4,0x2,param_3);
  if (BVar1 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  else {
    uStack6 = 0x0;
    while( true ) {
      if (uStack4 <= uStack6) {
        return;
      }
      pass1_1020_bb16(param_2,CONCAT22(param_4,&local_10),
                      CONCAT22(param_4,&local_12),uStack6);
      uStack10 = local_12;
      local_1c[0] = local_12;
      BVar1 = write_to_file_1008_7e1c
                        (uVar2,uVar3,local_1c,param_4,0x2,0x1020);
      if (BVar1 == 0x0) break;
      local_24[0] = local_10;
      BVar1 = write_to_file_1008_7e1c
                        (uVar2,uVar3,local_24,param_4,0x4,0x1020);
      if (BVar1 == 0x0) {
        return;
      }
      uStack6 += 0x1;
    }
  }
  return;
}


u16 
write_to_file_1008_7b4c
          (param_1: u32,param_2: *mut u16,param_3: HFILE16,param_4: u16)

{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_12: [u16;0x3];
  let local_c: [u16;0x2];
  let local_8: u16;
  let local_6: u16;
  let local_4: u16;
  
  pass1_1008_3eb4(param_2,CONCAT22(param_4,&local_8),
                  CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_4));
  local_12[0] = local_4;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  BVar1 = write_to_file_1008_7e1c
                    (uVar2,uVar3,local_12,param_4,0x2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = local_6;
    BVar1 = write_to_file_1008_7e1c
                      (uVar2,uVar3,local_c,param_4,0x2,param_3);
    if (BVar1 != 0x0) {
      local_c[0] = local_8;
      BVar1 = write_to_file_1008_7e1c
                        (uVar2,uVar3,local_c,param_4,0x2,param_3);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



pub fn read_file_1008_7bc8(param_1: u32,param_2: *mut u16,param_3: HFILE16,param_4: u16) -> bool

{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_8: u16;
  let local_6: u32;
  
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar2,uVar3,&local_6 + 0x2,0x0,param_4,0x2,param_3);
  if (BVar1 != 0x0) {
    BVar1 = read_file_1008_7dee(uVar2,uVar3,&local_6,0x0,param_4,0x2,param_3);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(uVar2,uVar3,&local_8,0x0,param_4,0x2,param_3);
      if (BVar1 != 0x0) {
        pass1_1008_3e76(param_2,local_8,local_6,(local_6 >> 0x10));
        return 0x1;
      }
    }
  }
  return 0x0;
}


pub fn read_file_1008_7c6e(param_1: u16,param_2: u16,char *param_3,param_4: HFILE16)
{
  let mut pcVar1: String; 
  char local_c [0xa];
  
  while( true ) {
    pcVar1 = param_3;
    WIN16_hread(param_4,0x1,ZEXT24(local_c) << 0x10);
    if (local_c[0] == '\0') break;
    param_3 = (param_3 & 0xffff0000 | (param_3 + 0x1));
    *pcVar1 = local_c[0];
    param_4 = (HFILE16)s_tile2_bmp_1050_1538;
  }
  *param_3 = '\0';
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_to_file_1008_7cac(param_1: u32,param_2: u16) -> bool

{
  let uVar1: u16;
  let BVar2: bool;
  let unaff_ES: u16;
  undefined1 in_AF;
  uchar local_c [0xa];
  
  sys_1000_3f9c(local_c,param_2,0x340,ctx.data_seg,
                ctx._PTR_s_dcbSC_1050_0336_1050_033c,&stack0xfffe,unaff_ES,0x1000,
                param_2,in_AF);
  uVar1 = str_op_1000_3da4(CONCAT22(param_2,local_c));
  BVar2 = write_to_file_1008_7e1c
                    (param_1,(param_1 >> 0x10),local_c,
                     param_2,uVar1,0x1000);
  if (BVar2 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return BVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
read_file_1008_7cfe(param_1: u16,param_2: u16,param_3: u16,
                   param_4: u16,param_5: u16)

{
  let bVar1: bool;
  let uVar2: u16;
  ulet in_AF: u8;
  let lVar3: i32;
  let in_stack_0000fbd2: u16;
  let in_stack_0000fbd4: u16;
  let uStack1040: u32;
  char local_406 [0x400];
  let uStack6: u32;
  
  uStack6 = 0x0;
  bVar1 = false;
  do {
    _llseek16(param_4,uStack6 << 0x10,(uStack6 >> 0x10));
    param_4 = ctx.s_tile2_bmp_1050_1538;
    lVar3 = WIN16_hread((HFILE16)s_tile2_bmp_1050_1538,0x400,ZEXT24(local_406) << 0x10);
    for (uStack1040 = 0x0; uStack1040 < lVar3; uStack1040 += 0x1) {
      if (local_406[uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
        if (!bVar1) {
          bVar1 = true;
          uStack6 = CONCAT22((uStack6 >> 0x10) + uStack1040._2_2_ +
                             CARRY2(uStack6,uStack1040),
                             uStack6 + uStack1040);
          break;
        }
        bVar1 = false;
        uVar2 = pass1_1008_7e4a((ctx.PTR__s_dcbSC_1050_0336_1050_033c >> 0x10),
                                param_5,in_AF,
                                CONCAT22(param_5,local_406 + uStack1040),
                                in_stack_0000fbd2,in_stack_0000fbd4);
        if (uVar2 != 0x0) {
          lVar3 = uStack1040 + uStack6 + 0x7;
          _llseek16((HFILE16)s_tile2_bmp_1050_1538,lVar3 * 0x10000,
                    (lVar3 >> 0x10));
          return;
        }
      }
    }
    if (!bVar1) {
      if (lVar3 < 0x400) {
        return;
      }
      uStack6._0_2_ = CONCAT11(uStack6._1_1_ + 0x4,uStack6);
      uStack6 = CONCAT22((uStack6 >> 0x10) + (0xfb < uStack6._1_1_),
                         uStack6);
    }
  } while( true );
}



bool 
read_file_1008_7dee(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
                   param_5: u16,SEGPTR param_6,param_7: HFILE16)

{
  let lVar1: i32;
  
  lVar1 = WIN16_hread(param_7,param_6,CONCAT22(param_3,param_4));
  if ((lVar1 == CONCAT22(param_4,param_6)) && (true)) {
    return 0x1;
  }
  return 0x0;
}



bool 
write_to_file_1008_7e1c
          (param_1: u16,param_2: u16,param_3: u16,param_4: u16,char *buf_to_write,
          HFILE16 file_handle)

{
  let mut pcVar1: String; 
  
  pcVar1 = _hwrite16(file_handle,buf_to_write,
                             CONCAT22(param_3,(buf_to_write >> 0x10)));
  if ((pcVar1 == buf_to_write) && (true)) {
    return 0x1;
  }
  return 0x0;
}


pub fn file_1008_bb5e(param_1: u32,param_2: u32,param_3: i16,param_4: *mut u8,param_5: u16,
              param_6: u16)

{
  let ppcVar1: u32;
  let uVar2: u16;
  let iVar3: &mut Struct199;
  let BVar3: bool;
  let uVar5: u16;
  let uVar4: &mut Struct200;
  let puVar6: *mut u8;
  let uVar7: u16;
  let extraout_DX: *mut u8;
  let puVar8: *mut u8;
  let uVar9: u16;
  let uVar10: u16;
  let extraout_DX_00: *mut u8;
  let extraout_DX_01: u16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  let paStack286: &mut Struct200;
  let puStack284: u32;
  let local_118: [u8;100];
  let local_18: [u16;0x2];
  let local_14: [u16;0x2];
  local_10: &mut Struct200 [0x4];
  let local_8: u32;
  
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  uVar11 = param_2;
  uVar12 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar11,uVar12,0x16,param_5,param_6);
  if (param_3 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d4;
  }
  else {
    iVar3 = param_1;
    iVar3 = &iVar3.field_0x22;
    uVar2 = (param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar11,uVar12,iVar3,0x0,uVar2,0x2,param_5);
    if ((BVar3 != 0x0) &&
       (uVar5 = read_file_1008_7dee(uVar11,uVar12,local_10,0x0,param_6,0x2,param_5
                                   ), uVar5 != 0x0)) {
      if (local_10[0] == 0x0) {
        return;
      }
      uVar14 = 0xc;
      mem_op_1000_179c(0xc,param_4,0x1000);
      if ((param_4 | uVar5) == 0x0) {
        uVar5 = 0x0;
        puVar8 = 0x0;
      }
      else {
        set_struct_1008_574a(CONCAT22(param_4,uVar5));
        puVar8 = extraout_DX;
      }
      &iVar3.field_0xa = uVar5;
      *(uchar **)(&iVar3.field_0xa + 0x2) = puVar8;
      paStack286 = 0x0;
      while( true ) {
        if (local_10[0] <= paStack286) {
          return;
        }
        uVar13 = 0x12;
        uVar4 = local_10[0];
        mem_op_1000_179c(0x12,puVar8,0x1000);
        if ((puVar8 | uVar4) == 0x0) {
          uVar4 = 0x0;
          uVar9 = 0x0;
        }
        else {
          set_stuct_1008_b0bc(CONCAT22(puVar8,uVar4));
          uVar9 = extraout_DX_01;
        }
        puStack284 = CONCAT22(uVar9,uVar4);
        puVar6 = local_118;
        uVar10 = uVar9;
        read_file_1008_7c6e(uVar11,uVar12,CONCAT22(param_6,puVar6),0x1000);
        if ((((puVar6 == 0x0) ||
             (BVar3 = read_file_1008_7dee(uVar11,uVar12,local_14,0x0,param_6,0x2,
                                          0x1000), BVar3 == 0x0)) ||
            (BVar3 = read_file_1008_7dee(uVar11,uVar12,&local_8,0x0,param_6,0x4,
                                         0x1000), BVar3 == 0x0)) ||
           (BVar3 = read_file_1008_7dee(uVar11,uVar12,local_18,0x0,param_6,0x2,
                                        0x1000), BVar3 == 0x0)) break;
        uVar7 = str_op_1008_60e8(CONCAT22(param_6,local_118),uVar10);
        uVar4.field_0x4 = uVar7;
        uVar4.field_0x6 = uVar10;
        uVar4.field_0x8 = local_14[0];
        uVar4.field_0xa = local_8;
        uVar4.field_0xe = local_18[0];
        ppcVar1 = (*iVar3.field_0xa + 0x8);
        (**ppcVar1)();
        paStack286 = &paStack286.field_0x1;
        puVar8 = extraout_DX_00;
      }
      if (puStack284 != 0x0) {
        ppcVar1 = *puStack284;
        (**ppcVar1)(0x1000,uVar4,uVar9,0x1,uVar13,uVar14,puStack284);
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}


void 
file_1008_e70e(param_1: u32,param_2: u32,param_3: i16,param_4: *mut u8,param_5: u16,
              param_6: u16)

{
  let uVar1: u32;
  let ppcVar2: u32;
  let BVar3: bool;
  let uVar4: u16;
  let extraout_DX: *mut u8;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let local_12: [u16;0x2];
  let puStack14: u32;
  let uStack10: u16;
  let local_4: u16;
  
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  uVar7 = param_2;
  uVar8 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar7,uVar8,0x14,param_5,param_6);
  if (param_3 != 0x0) {
    BVar3 = read_file_1008_7dee(uVar7,uVar8,&local_4,0x0,param_6,0x2,param_5);
    if (BVar3 != 0x0) {
      if (local_4 == 0x0) {
        return;
      }
      uStack10 = 0x0;
      while( true ) {
        if (local_4 <= uStack10) {
          return;
        }
        uVar9 = 0x14;
        uVar4 = local_4;
        mem_op_1000_179c(0x14,param_4,0x1000);
        uVar5 = param_4 | uVar4;
        if (uVar5 == 0x0) {
          uVar4 = 0x0;
          uVar5 = 0x0;
        }
        else {
          struct_1008_dcdc(CONCAT22(param_4,uVar4));
        }
        puStack14 = CONCAT22(uVar5,uVar4);
        BVar3 = read_file_1008_7dee(uVar7,uVar8,uVar4 + 0x4,0x0,uVar5,0x4,0x1000);
        if ((((BVar3 == 0x0) ||
             (BVar3 = read_file_1008_7dee(uVar7,uVar8,puStack14 + 0x8,0x0,
                                          (puStack14 >> 0x10),0x4,0x1000),
             BVar3 == 0x0)) ||
            (BVar3 = read_file_1008_7dee(uVar7,uVar8,local_12,0x0,param_6,0x2,
                                         0x1000), BVar3 == 0x0)) ||
           ((BVar3 = read_file_1008_7dee(uVar7,uVar8,puStack14 + 0xe,0x0,
                                         (puStack14 >> 0x10),0x4,0x1000),
            BVar3 == 0x0 ||
            (BVar3 = read_file_1008_7dee(uVar7,uVar8,puStack14 + 0x12,0x0,
                                         (puStack14 >> 0x10),0x2,0x1000),
            BVar3 == 0x0)))) break;
        uVar9 = (puStack14 >> 0x10);
        (puStack14 + 0xc) = local_12[0];
        uVar6 = (param_1 >> 0x10);
        uVar1 = (param_1 + 0xa);
        ppcVar2 = ((param_1 + 0xa) + 0x4)
        ;
        (**ppcVar2)(0x0,uVar1,(uVar1 >> 0x10),puStack14,uVar9);
        uStack10 += 0x1;
        param_4 = extraout_DX;
      }
      if (puStack14 != 0x0) {
        ppcVar2 = *puStack14;
        (**ppcVar2)(0x1000,puStack14,(puStack14 >> 0x10),0x1,uVar9,
                    puStack14);
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}

