
pub fn pass1_1010_8018(u8 *param_1,mut param_2: u32,mut param_3: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: *mut u16;

  if ((param_3 * 0xa + 0x1fa0) != 0x0) {
    pass1_1010_878c((astruct_87 **)param_2,(param_3 * 0xa + 0x1fa0));
    uVar2 = (param_2 >> 0x10);
    if (*(i32 *)(param_2 + 0x67c) != 0x0) {
      iVar1 = param_3 * 0xa;
      pass1_1008_64c8(param_3,param_1,(u32*)(param_2 + 0x67c),
                      CONCAT22((iVar1 + 0x1fa6),(iVar1 + 0x1fa8)),(iVar1 + 0x1fa4)
                     );
      return;
    }
  }
  return;
}
pub fn pass1_1010_8096(u32 *param_1,mut param_2: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut pcVar5: *mut c_char;
  let mut puVar6: *mut u16;
  u8 local_306 [0x100];
  u8 local_206 [0x100];
  u8 local_106 [0x104];

  uVar4 = (param_1 >> 0x10);
  uVar3 = param_1;
  str_1000_4d58(*(char **)((uVar3 + 0xe82) * 0x4 + 0x2526),NULL,0x0,CONCAT22(0x1050,local_206),
                (WNDCLASS16 *)CONCAT22(0x1050,local_306));
  unk_str_op_1000_3d3e(CONCAT22(0x1050,local_106),CONCAT22(0x1050,local_206));
  if (param_2 == 0x2) {
    puVar6 = &u16_1050_3194;
  }
  else {
    puVar6 = &u16_1050_3196;
  }
  pass1_1000_3cea(CONCAT22(0x1050,local_106),puVar6);
  pass1_1000_3cea(CONCAT22(0x1050,local_106),CONCAT22(0x1050,local_306));
  pcVar5 = set_err_mode_1010_8b14(param_1,CONCAT22(0x1050,local_106));
  uVar2 = (pcVar5 >> 0x10);
  if ((pcVar5 == local_106) && (uVar2 == &DAT_1050_1050)) {
    msg_box_op_1010_8bb4(uVar3,uVar4,pcVar5 & 0xffff | 0x10500000);
  }
  fn_ptr_1000_17ce(*param_1);
  uVar1 = str_op_1008_60e8(uVar2,pcVar5);
  param_1 = uVar1;
  (uVar3 + 0x2) = uVar2;
  return;
}
pub fn pass1_1010_8170(u8 *param_1,param_2: *mut astruct_87,mut param_3: i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_2 >> 0x10);
  iVar2 = param_2;
  iVar1 = (iVar2 + 0x680);
  iVar3 = param_3 * 0x10;
  if ((iVar3 + 0x16) != iVar1) {
    pass1_1010_8096(param_2,(iVar3 + 0x16));
    pass1_1010_878c((astruct_87 **)param_2,(iVar3 + 0x16));
    if (*(i32 *)(iVar2 + 0x67c) == 0x0) {
      return;
    }
  }
  iVar3 = param_3 * 0x10;
  pass1_1008_6562((astruct_57 *)CONCAT22(iVar1,param_1),*(astruct_76 **)(iVar2 + 0x67c),
                  CONCAT22((iVar3 + 0x1c),(iVar3 + 0x1e)),(iVar3 + 0x1a));
  return;
}
pub fn pass1_1010_81f6(astruct_87 **param_1,i32 param_2,mut param_3: i16)

{
  let mut in_EDX: u32;
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uStack12: u16;
  let mut uStack10: u32;

  if (param_2 == 0x8000001) {
    uVar1 = in_EDX & 0xffff0000 | param_1;
    uStack10 = param_1 & 0xffff0000 | (param_1 + 0x22c);
    uStack12 = 0xfa;
  }
  else if (param_2 == 0x8000002) {
    uVar1 = in_EDX & 0xffff0000 | param_1;
    uStack10 = param_1 & 0xffff0000 | (param_1 + 0x454);
    uStack12 = 0xfc;
  }
  else {
    uVar1 = in_EDX & 0xffff0000 | param_1;
    uStack10 = param_1 & 0xffff0000 | (param_1 + 0x4);
    uStack12 = 0x2;
  }
  uVar2 = (uStack10 >> 0x10);
  uVar3 = uVar1;
  if (*(i32 *)(param_3 * 0x4 + uStack10) == 0x0) {
    if ((0x0 < param_3) && (param_3 < 0xa)) {
      pass1_1010_89f0(param_1,uVar3,uStack12,uStack10);
      return;
    }
    if (param_3 < 0xa) {
      return;
    }
    if (0x7f < param_3) {
      return;
    }
    if (*(i32 *)(uStack10 + 0x14) == 0x0) {
      pass1_1010_89f0(param_1,uVar3,uStack12,uStack10);
    }
    pass1_1010_887a(param_1,uStack10,param_3,uVar1);
    uVar3 = param_1;
  }
  pass1_1010_866c(uVar1,param_1,uVar3,uStack10,param_3);
  return;
}
pub fn pass1_1010_82f8(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0xe82) = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps

u16 FUN_1010_830a(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,param_4: *mut astruct_87,mut param_5: i16)

{
  let mut uVar2: u16;
  astruct_81 *puVar2;
  astruct_81 *paVar3;
  let mut uVar4: u16;
  let mut paVar5: *mut Struct57;
  let mut iVar6: i16;
  let mut unaff_SS: u16;
  let mut unaff_DS: u16;
  let mut pcVar7: *mut c_char;
  let mut uVar8: u32;
  astruct_81 local_2e;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut uStack6: u32;
  let mut uVar1: u16;
  astruct_87 *uVar9;
  astruct_87 *uVar10;

  uStack6 = 0x0;
  uVar1 = 0x63f0;
  iVar6 = param_5 * 0x10;
  uVar9 = (astruct_87 *)param_4;
  uVar10 = (astruct_87 *)(param_4 >> 0x10);
  if ((iVar6 + 0x10) == 0x1) {
    pcVar7 = set_err_mode_1010_8b14(param_4,(iVar6 + 0x12));
    paVar5 = (astruct_57 *)(param_2 & 0xffff0000 | pcVar7 >> 0x10);
    iStack10 = pcVar7;
    iStack8 = (pcVar7 >> 0x10);
    uVar2 = 0x63f0;
    if (((iVar6 + 0x12) == iStack10) && ((iVar6 + 0x14) == iStack8)) {
      msg_box_op_1010_8bb4(uVar9,uVar10,pcVar7);
      return 0x0;
    }
    puVar2 = &local_2e;
    struct_op_1008_48fe(paVar5,(astruct_81 *)CONCAT22(unaff_SS,puVar2),0x1,pcVar7);
    mem_op_1000_179c(0x1e,paVar5);
    uVar4 = paVar5 | puVar2;
    if (uVar4 == 0x0) {
      paVar3 = NULL;
      uVar4 = 0x0;
    }
    else {
      paVar3 = &local_2e;
      struct_op_1008_3f92((astruct_76 *)CONCAT22(paVar5,puVar2),CONCAT22(unaff_SS,paVar3));
    }
    uStack6 = CONCAT22(uVar4,paVar3);
    close_file_1008_496c((astruct_803 *)CONCAT22(unaff_SS,&local_2e));
  }
  else {
    if ((param_5 * 0x10 + 0x10) == 0x2) {
      pass1_1010_878c((astruct_87 **)param_4,(param_5 * 0x10 + 0x16));
      if (*(i32 *)&uVar9->field1660_0x67c == 0x0) {
        return 0x0;
      }
      uVar2 = 0x63f0;
      iVar6 = param_5 * 0x10;
      pass1_1008_6562((astruct_57 *)(param_2 & 0xffff | param_1 << 0x10),*(astruct_76 **)&uVar9->field1660_0x67c,
                      CONCAT22((iVar6 + 0x1c),(iVar6 + 0x1e)),(iVar6 + 0x1a));
    }
    else {
      iVar6 = param_5 * 0x10;
      if ((iVar6 + 0x10) == 0x3) {
        uVar8 = set_err_mode_1010_8b14(param_4,(iVar6 + 0x12));
        local_2e.field1_0x2 = (uVar8 >> 0x10);
        param_1 = uVar8;
        uVar2 = 0x63f0;
        if (((iVar6 + 0x12) == param_1) && ((iVar6 + 0x14) == local_2e.field1_0x2)) {
          local_2e.field0_0x0 = param_1;
          msg_box_op_1010_8bb4(uVar9,uVar10,uVar8);
          param_1 = local_2e.field0_0x0;
        }
      }
      else {
        if ((param_5 * 0x10 + 0x10) != 0x4) goto LAB_1010_8473;
        uVar8 = set_err_mode_1010_8b14(param_4,(param_5 * 0x10 + 0x12));
        param_1 = uVar8;
      }
    }
    uStack6 = param_1;
  }//
LAB_1010_8473:
  return uStack6;
}



char * load_string_1010_847e(mut param_1: u32,mut param_2: u16 )

{
  LoadString16(0x3ff,(param_1 & 0xffff0000 | (param_1 + 0x682U)),param_2,HINSTANCE16_1050_038c);
  return (param_1 & 0xffff0000 | (param_1 + 0x682U));
}
pub fn load_string_1010_84ac(mut param_1: i16,INT16 param_2,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = param_2;
  LoadString16(0x3ff,CONCAT22(param_2,param_1 + 0x682),param_3,HINSTANCE16_1050_038c);
  str_op_1008_60e8(uVar1,CONCAT22(param_2,param_1 + 0x682));
  return;
}
pub fn load_string_1010_84e0(mut param_1: u16 ,mut param_2: u16 ,u16 in_resc_id_3,char *in_buffer_4,short in_buf_len_5)

{
  let mut in_stack_0000000e: u16;

  LoadString16(in_resc_id_3,CONCAT22(in_buf_len_5,in_buffer_4),in_stack_0000000e,HINSTANCE16_1050_038c);
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_84f8(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut pcStack780: *mut c_char;
  char local_308 [0x100];
  u8 local_208 [0x100];
  u8 local_108 [0x104];
  let mut iStack4: i16;

  if ((param_2 * 0x10 + 0x10) == 0x3) {
    uVar1 = (param_1 + 0xe88);
    iStack4 = (uVar1 + 0x70);
    str_1000_4d58(*(char **)(param_2 * 0x10 + 0x12),NULL,0x0,CONCAT22(0x1050,local_208),
                  (WNDCLASS16 *)CONCAT22(0x1050,local_308));
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_108),CONCAT22(0x1050,local_208));
    if (local_308[0] == '\0') {
      if (iStack4 == 0x0) {
        pcStack780 = s__mid_1050_14c0;
      }
      else {
        pcStack780 = s__wav_1050_14ba;
      }
    }
    else {
      pcStack780 = local_308;
    }
    pcStack780 = CONCAT22(0x1050,pcStack780);
    pass1_1000_3cea(CONCAT22(0x1050,local_108),pcStack780);
    set_err_mode_1010_8b14(param_1,CONCAT22(0x1050,local_108));
    return;
  }
  return;
}
pub fn pass1_1010_85be(mut param_1: u32,mut param_2: i16,mut param_3: i16)

{
  let mut uVar1: u32;
  u8 local_30a [0x100];
  u8 local_20a [0x100];
  u8 local_10a [0x108];

  if (param_2 == 0x2) {
    uVar1 = (param_3 * 0x4 + 0x2e34);
    str_1000_4d58((uVar1 & 0xffff0000 | (uVar1 + 0x3)),NULL,0x0,CONCAT22(0x1050,local_20a),
                  (WNDCLASS16 *)CONCAT22(0x1050,local_30a));
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_10a),s_male_1050_14c6);
    pass1_1000_3cea(CONCAT22(0x1050,local_10a),CONCAT22(0x1050,local_20a));
    pass1_1000_3cea(CONCAT22(0x1050,local_10a),CONCAT22(0x1050,local_30a));
    set_err_mode_1010_8b14(param_1,CONCAT22(0x1050,local_10a));
    return;
  }
  set_err_mode_1010_8b14(param_1,(param_3 * 0x4 + 0x2e34));
  return;
}
pub fn pass1_1010_866c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

{
  astruct_828 *paVar1;
  char cVar2;
  let mut iVar3: i16;
  let mut bVar4: bool;

  if (param_5 < 0x28) {
    if ((param_5 < 0x25) && (param_5 != 0x23)) {
      if (0x23 < param_5) {
        return;
      }
      cVar2 = (char)param_5;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else if (param_5 != 0x37) {
    if (param_5 < 0x38) {
      if (param_5 < 0x33) {
        return;
      }
      bVar4 = SBORROW2(param_5 - 0x33,0x1);
      iVar3 = param_5 - 0x34;
    }
    else {
      if (param_5 == 0x49) goto LAB_1010_8691;
      if ((param_5 - 0x49) < 0x2a) {
        return;
      }
      bVar4 = SBORROW2(param_5 - 0x73,0x5);
      iVar3 = param_5 - 0x78;
    }
    if (iVar3 != 0x0 && bVar4 == iVar3 < 0x0) {
      return;
    }
  }//
LAB_1010_8691:
  paVar1 = *(astruct_828 **)(param_5 * 0x4 + param_4);
  memcpy_op_1008_676e((astruct_830 *)paVar1,param_1,paVar1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_86de(mut param_1: u16 ,mut param_2: u16 ,uchar param_3,param_4: *mut astruct_76)

{
  i32 *plVar1;
  let mut iVar2: i16;
  let mut bVar3: bool;
  let mut uVar4: u16;
  i32 lVar5;
  let mut uVar6: u32;
  i32 lStack20;
  let mut uStack10: u32;

  uVar6 = pass1_1008_4772(param_4);
  uVar4 = (uVar6 >> 0x10);
  uStack10 = 0x0;
  do {
    plVar1 = (i32 *)(uVar6 + 0x8);
    if (*plVar1 == uStack10 || *plVar1 < uStack10) {
      return;
    }
    lVar5 = uStack10;
    pass1_1008_4544(param_4);
    iVar2 = lVar5;
    bVar3 = false;
    for (lStack20 = 0x0; plVar1 = (i32 *)(uVar6 + 0x4), *plVar1 != lStack20 && lStack20 <= *plVar1;
        lStack20 += 0x1) {
      if (bVar3) {//
LAB_1010_86fc:
        if (bVar3) {
          if (*(lStack20 + iVar2) == -0x1) {
            *(lStack20 + iVar2) = param_3;
            break;
          }
        }
      }
      else {
        if (*(lStack20 + iVar2) == -0x1) goto LAB_1010_86fc;
        *(lStack20 + iVar2 + -0x1) = param_3;
        bVar3 = true;
      }
    }
    uStack10 += 0x1;
  } while( true );
}
pub fn pass1_1010_878c(astruct_87 **param_1,mut param_2: i16)

{
  let mut uVar4: u16;
  let mut uVar1: u16;
  let mut puVar2: *mut u8,
  let mut in_EDX: u32;
  let mut paVar3: *mut Struct57;
  astruct_87 *uVar6;
  let mut iVar4: i16;
  let mut uVar5: u16;
  astruct_87 *paVar6;
  let mut pcStack6: *mut c_char;

  uVar5 = (param_1 >> 0x10);
  uVar6 = (astruct_87 *)param_1;
  if (uVar6->field1662_0x680 == param_2) {
    return;
  }
  uVar4 = uVar6->field1660_0x67c;
  puVar2 = uVar6->field1661_0x67e;
  pcStack6 = CONCAT22(puVar2,uVar4);
  paVar3 = (astruct_57 *)(in_EDX & 0xffff0000 | (puVar2 | uVar4));
  if ((puVar2 | uVar4) != 0x0) {
    pass1_1008_64a2(CONCAT22(puVar2,uVar4));
    fn_ptr_1000_17ce(pcStack6);
  }
  if ((param_2 == 0x1) || (param_2 == 0x2)) {
    mem_op_1000_179c(0x8,paVar3);
    uVar1 = paVar3;
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | (uVar1 | uVar4));
    if ((uVar1 | uVar4) == 0x0) {
      &uVar6->field1660_0x67c = 0x0;
      goto LAB_1010_8869;
    }
    paVar6 = *param_1;//
LAB_1010_8853:
    file_1008_6414(paVar3,CONCAT22(uVar1,uVar4),paVar6);
    puVar2 = paVar3;
  }
  else {
    iVar4 = param_2 * 0x4;
    paVar6 = (astruct_87 *)set_err_mode_1010_8b14(param_1,(iVar4 + 0x172a));
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | paVar6 >> 0x10);
    uVar4 = paVar6;
    uVar1 = (paVar6 >> 0x10);
    if (((iVar4 + 0x172a) == uVar4) && ((iVar4 + 0x172c) == uVar1)) {
      msg_box_op_1010_8bb4(uVar6,uVar5,paVar6 & 0xffff | uVar1 << 0x10);
    }
    mem_op_1000_179c(0x8,paVar3);
    uVar1 = paVar3;
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | (uVar1 | uVar4));
    if ((uVar1 | uVar4) != 0x0) goto LAB_1010_8853;
    uVar4 = 0x0;
    puVar2 = NULL;
  }
  uVar6->field1660_0x67c = uVar4;
  uVar6->field1661_0x67e = puVar2;//
LAB_1010_8869:
  uVar6->field1662_0x680 = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_887a(astruct_87 **param_1,mut param_2: u32,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut paVar5: *mut Struct57;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  u8 bVar10;
  u8 uVar11;
  u8 local_26 [0x6];
  let mut uStack32: u16;
  let mut uStack30: u16;
  astruct_76 *paStack28;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut uStack16: u32;
  astruct_76 *paStack12;
  astruct_76 *paStack8;
  let mut uStack4: u16;

  uStack4 = param_3 - 0xa;
  pass1_1010_878c(param_1,(uStack4 * 0xa + 0x3382));
  uVar7 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x67c) != 0x0) {
    iVar6 = uStack4 * 0xa;
    uVar1 = uStack4;
    pass1_1008_6562((astruct_57 *)(param_4 & 0xffff | uStack4 << 0x10),*(astruct_76 **)(param_1 + 0x67c),
                    CONCAT22((iVar6 + 0x3388),(iVar6 + 0x338a)),(iVar6 + 0x3386));
    paStack8 = (astruct_76 *)CONCAT22(param_4,uVar1);
    uVar8 = (param_2 >> 0x10);
    paStack12 = *(astruct_76 **)(param_2 + 0x14);
    uStack16 = pass1_1008_4772(paStack12);
    uVar2 = param_4 & 0xffff0000;
    uStack20 = pass1_1008_4772(paStack8);
    paVar5 = (astruct_57 *)(uVar2 & 0xffff0000 | uStack20 >> 0x10);
    uVar7 = (uStack20 >> 0x10);
    uVar2 = (uStack20 + 0x4);
    uVar9 = (uStack16 >> 0x10);
    iVar6 = uStack16;
    if ((long)uVar2 < *(i32 *)(iVar6 + 0x4)) {
      uVar2 = (iVar6 + 0x4);
    }
    uVar3 = (uStack20 + 0x8);
    if ((long)uVar3 < *(i32 *)(iVar6 + 0x8)) {
      uVar3 = (iVar6 + 0x8);
    }
    uVar1 = uVar3;
    uStack24 = uVar3 & 0xffff | uVar2 << 0x10;
    bVar10 = 0xff;
    uVar11 = 0x0;
    mem_op_1000_179c(0x1e,paVar5);
    uVar4 = paVar5 | uVar1;
    if (uVar4 == 0x0) {
      uVar1 = 0x0;
      uVar4 = 0x0;
    }
    else {
      struct_op_1008_6604((astruct_76 *)CONCAT22(paVar5,uVar1),uStack24,
                          CONCAT13(uVar11,CONCAT12(bVar10,(uStack24 >> 0x10))));
    }
    paStack28 = (astruct_76 *)CONCAT22(uVar4,uVar1);
    pass1_1008_431c((astruct_76 *)CONCAT22(uVar4,uVar1),bVar10);
    uVar7 = (uStack16 >> 0x10);
    uStack30 = (uStack24 - (uStack16 + 0x4)) / 0x2;
    uStack32 = uStack24 - (uStack16 + 0x8);
    pass1_1008_3e54(CONCAT22(0x1050,local_26),0x0,uStack32,uStack30);
    pass1_1008_4480(paStack28,CONCAT22(0x1050,local_26),paStack12);
    pass1_1008_3e76(CONCAT22(0x1050,local_26),0x0,0x0,0x7);
    pass1_1008_4480(paStack28,CONCAT22(0x1050,local_26),paStack8);
    *(astruct_76 **)(param_3 * 0x4 + param_2) = paStack28;
  }
  return;
}
pub fn pass1_1010_89f0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u8,
  let mut in_EDX: u32;
  let mut paVar4: *mut Struct57;
  let mut iVar7: i16;
  let mut pcVar8: *mut c_char;
  let mut uStack22: u32;
  let mut pcStack12: *mut c_char;
  let mut uStack8: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u16;

  uVar1 = (param_1 + 0x67c);
  uVar2 = (param_1 + 0x67e);
  uVar6 = (in_EDX >> 0x10);
  pcStack12 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack12);
  }
  pcVar8 = set_err_mode_1010_8b14(CONCAT22(param_2,param_1),((param_1 + 0xe82) * 0x4 + 0x24be));
  uVar2 = (pcVar8 >> 0x10);
  paVar4 = (astruct_57 *)CONCAT22(uVar6,uVar2);
  uVar1 = pcVar8;
  iVar7 = (param_1 + 0xe82) * 0x4;
  if (((iVar7 + 0x24be) == uVar1) && ((iVar7 + 0x24c0) == uVar2)) {
    msg_box_op_1010_8bb4(param_1,param_2,pcVar8 & 0xffff | uVar2 << 0x10);
  }
  mem_op_1000_179c(0x8,paVar4);
  uVar2 = paVar4 | uVar1;
  paVar5 = (astruct_57 *)(paVar4 & 0xffff0000 | uVar2);
  if (uVar2 == 0x0) {
    uVar1 = 0x0;
    paVar5 = NULL;
  }
  else {
    file_1008_6414(paVar5,CONCAT22(paVar4,uVar1),pcVar8);
  }
  (param_1 + 0x67c) = uVar1;
  (param_1 + 0x67e) = paVar5;
  (param_1 + 0x680) = 0x0;
  if (((param_1 + 0x67e) | (param_1 + 0x67c)) != 0x0) {
    for (uStack8 = 0x1; puVar3 = paVar5, uStack8 < 0xa; uStack8 += 0x1) {
      iVar7 = uStack8 * 0xa;
      uVar1 = uStack8;
      pass1_1008_64c8(uStack8,puVar3,(u32*)(param_1 + 0x67c),
                      CONCAT22((iVar7 + 0x2558),(iVar7 + 0x255a)),(iVar7 + 0x2556)
                     );
      uStack22 = CONCAT22(puVar3,uVar1);
      pass1_1010_86de(param_1,param_2,(uchar)param_3,(astruct_76 *)CONCAT22(puVar3,uVar1));
      paVar5 = (astruct_57 *)ZEXT24(puVar3);
      (uStack8 * 0x4 + param_4) = uStack22;
    }
  }
  return;
}



pub fn set_err_mode_1010_8b14(mut param_1: u32,mut param_2: u32) -> u32

{
  let mut mode: u16;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  i32 lVar4;
  u8 local_a [0x8];

  uVar3 = (param_1 >> 0x10);
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0xe84));
  mode = SetErrorMode16(SEM_FAILCRITICALERRORS);
  do {
    lVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar4 == 0x0) {
      SetErrorMode16(mode);
      return param_2;
    }
    uVar1 = param_1 + 0xa82;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | uVar1),*(char **)(lVar4 + 0x4));
    pass1_1000_3cea(param_1 & 0xffff0000 | uVar1,param_2);
    uVar2 = dos3_call_1000_51aa(uVar1,uVar3,0x1);
  } while (uVar2 != 0x0);
  SetErrorMode16(mode);
  return param_1 & 0xffff0000 | uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1010_8bb4(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut pcVar1: *mut c_char;
  u8 local_402 [0x400];

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,0x3fa);
  unk_str_op_1000_3d3e(CONCAT22(0x1050,local_402),pcVar1);
  pass1_1000_3cea(CONCAT22(0x1050,local_402),param_3);
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,0x57b);
  MessageBox16(0x1010,pcVar1,CONCAT22(0x1050,local_402),HWND16_1050_0396);
  PostMessage16(0x0,0xee,0x111,HWND16_1050_0396);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_8c32(param_1: *mut astruct_19,mut param_2: u16 ) -> u32

{
  let mut in_EDX: u32;
  let mut uVar1: u16;
  let mut unaff_BP: u16;
  astruct_19 *paVar2;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  uVar1 = (in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
    //        1010:8ee2  bc  8e  10  10      addr         pass1_1010_8ebc
  param_1->offset_0x0 = 0x8ee2;
  (param_1 + 0x2) = 0x1010;
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar1,(paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (param_1 + 0xa) = puVar3;
  (param_1 + 0xc) = (puVar3 >> 0x10);
  return param_1;
}
pub fn pass1_1010_8c78(param_1: *mut u16)

{
  *param_1 = 0x8ee2;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_load_str_op_1010_8c96(u8 *param_1,mut param_2: u32,mut param_3: u32,mut param_4: u32) -> u32

{
  let mut uVar1: u32;
  INT16 IVar2;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  astruct_15 *paVar9;
  let mut uVar10: u32;
  let mut pcVar11: *mut c_char;
  WORD *valist;
  short in_buf_len_5;
  let mut puVar12: *mut u8,
  let mut uStack46: u32;
  let mut local_10: u32;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut puStack8: *mut u8,
  let mut uStack6: u16;
  let mut uStack4: u16;

  uVar7 = (param_4 >> 0x10);
  iVar6 = param_4;
  uVar5 = (iVar6 + 0x6);
  uVar10 = uVar5;
  valist = (WORD *)param_3;
  in_buf_len_5 = (short)(param_3 >> 0x10);
  if (uVar5 != 0x0) {
    uVar8 = (param_2 >> 0x10);
    if (uVar5 == 0x1) {
      uVar10 = param_4 & 0xffff;
      iVar4 = uVar10;
      switch((iVar6 + 0x4) + -0x1) {
      case 0x0:
      case 0x1:
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar6 + 0x8));
        local_10 = (iVar4 + 0xc);
        iStack12 = (iVar4 + 0x10);
        iStack10 = iVar4;
        puStack8 = param_1;
        if (0x0 < iStack12) {
          pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x437);
          uStack4 = (pcVar11 >> 0x10);
          uStack6 = SUB42(pcVar11,0x0);
          IVar2 = wsprintf16(valist,CONCAT22(uStack6,in_buf_len_5),CONCAT22(iStack12,uStack4));
          return CONCAT22(IVar2,uStack4);
        }
        break;
      case 0x2:
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar6 + 0x8));
        local_10 = (iVar4 + 0xc);
        iStack12 = (iVar4 + 0x10);
        if (0x0 < iStack12) {
          iStack12 = 0x0;
          paVar9 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(param_1,iVar4),&local_10,param_1);
          uVar10 = pass1_1028_bb24(paVar9);
          param_1 = (uVar10 >> 0x10);
          iStack10 = uVar10;
          puVar3 = &local_10;
          puStack8 = param_1;
          pass1_1030_627e(puVar3,param_1,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),uVar10);
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar3));
          uVar1 = (param_2 + 0xa);
          pass1_1010_c3c2(param_1,uVar1,(uVar1 >> 0x10),0x0,CONCAT22(param_1,puVar3));
          uStack46 = CONCAT22(param_1,puVar3);
          puVar12 = param_1;
          pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x439);
          uStack4 = (pcVar11 >> 0x10);
          uStack6 = SUB42(pcVar11,0x0);
          wsprintf16(valist,CONCAT22(uStack6,in_buf_len_5),CONCAT22(puVar3,uStack4),puVar12);
          goto LAB_1010_8def;
        }
        break;
      default:
        goto switchD_1010_8e11_caseD_4;
      case 0x4:
      case 0x7:
      case 0x8:
      case 0xa:
        goto LAB_1010_8ea5;
      }
      uVar10 = ZEXT24(&local_10);
    }
    else {
      uVar10 = (uVar5 - 0x2);
      if (uVar5 - 0x2 == 0x0) {
        iVar4 = (iVar6 + 0x4);
        uVar5 = iVar4 - 0x4;
        if (uVar5 != 0x0) {
          uVar5 = iVar4 - 0xc;
          uVar10 = uVar5;
          if (uVar5 != 0x0) goto LAB_1010_8ea5;
        }
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar6 + 0x8));
        uVar1 = (param_2 + 0xa);
        pass1_1010_c3c2(param_1,uVar1,(uVar1 >> 0x10),0x0,CONCAT22(param_1,uVar5));
        uStack46 = CONCAT22(param_1,uVar5);
        puVar12 = param_1;
        pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x43b);
        uStack4 = (pcVar11 >> 0x10);
        uStack6 = SUB42(pcVar11,0x0);
        wsprintf16(valist,CONCAT22(uStack6,in_buf_len_5),CONCAT22(uVar5,uStack4),puVar12);//
LAB_1010_8def:
        fn_ptr_1000_17ce((uStack46 & 0xffff | ZEXT24(param_1) << 0x10));
        return CONCAT22(uStack46,param_1);
      }
    }
  }//
LAB_1010_8ea5:
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,valist,in_buf_len_5);
switchD_1010_8e11_caseD_4:
  return CONCAT22(uVar10,param_1);
}



pub fn pass1_1010_8ebc(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1010_8c78(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_8ef2(param_1: *mut astruct_57,param_2: *mut astruct_170,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                    mut param_6: u16 ,mut param_7: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_170 *iVar3;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut paVar3: *mut Struct57;

  uVar4 = (param_2 >> 0x10);
  iVar3 = (astruct_170 *)param_2;
  param_2->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  uVar1 = 0x0;
  &iVar3->field2_0x4 = 0x0;
  &iVar3->field4_0x8 = 0x0;
  param_2->field0_0x0 = 0x9254;
  iVar3->field1_0x2 = 0x1010;
  mem_op_1000_179c(0x18,param_1);
  uVar2 = param_1 | uVar1;
  paVar3 = (astruct_57 *)(param_1 & 0xffff0000 | uVar2);
  if (uVar2 == 0x0) {
    &iVar3->field2_0x4 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_1,uVar1),0x5,0x5);
    iVar3->field2_0x4 = uVar1;
    iVar3->field3_0x6 = paVar3;
  }
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3),param_4,param_5,param_6,param_7);
  iVar3->field4_0x8 = puVar5;
  iVar3->field5_0xa = (puVar5 >> 0x10);
  return;
}
pub fn pass1_1010_8f78(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  StructD *uVar4;

  uVar4 = (StructD *)(param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x9254;
  iVar4->address_offset_field_0x2 = 0x1010;
  puVar1 = iVar4->hfile_0x4;
  uVar2 = &iVar4->field_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar4->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1010_8fba(mut param_1: u16 ,param_2: *mut astruct_411)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  astruct_411 *iVar3;
  let mut uVar3: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar3 = (param_2 >> 0x10);
  iVar3 = (astruct_411 *)param_2;
  ppcVar1 = (code **)(*iVar3->field4_0x4 + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(extraout_DX,param_1);
  uStack14 = 0x0;
  while( true ) {
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar1 = (code **)(*iVar3->field4_0x4 + 0x4);
    uVar2 = uStack10;
    (**ppcVar1)();
    if ((extraout_DX_00 | uVar2) != 0x0) break;
    uStack14 += 0x1;
  }
  ppcVar1 = (code **)(*iVar3->field4_0x4 + 0x8);
  (**ppcVar1)();
  return;
}
