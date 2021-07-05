fn  poss_str_op_1000_28dc(param_1: i16) -> *mut u8

{
  let piVar1: *mut i16;
  *mut let piVar2: u8;
  let iVar2: i16;
  *mut let piVar3: u8;
  
  piVar3 = (*mut u8)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = (i16 *)piVar3;
    piVar3 = (*mut u8)(piVar3 + 0x2);
    iVar2 = *piVar1;
    piVar2 = piVar3;
    if ((iVar2 == param_1) || (piVar2 = (*mut u8)(iVar2 + 0x1), piVar2 == (*mut u8)0x0)) {
      return (*mut u8)(i16 *)piVar2;
    }
    iVar2 = -0x1;
    do {
      if (iVar2 == 0x0) break;
      iVar2 += -0x1;
      piVar1 = (i16 *)piVar3;
      piVar3 = (*mut u8)(piVar3 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}


fn unk_str_op_1000_3d3e(char *param_1,char *in_string_2)
{
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let uVar6: u16;
  let uVar7: u16;
  char *l_string_2;
  char *puVar6;
  char *puVar7;
  let uVar8: u16;
  char *l_string_1;
  let l_b_var8: bool;
  char *puVar3;
  char *puVar2;
  char *puVar1;
  
  l_string_1 = (in_string_2 >> 0x10);
  l_string_2 = in_string_2;
  l_b_var8 = true;
  uVar6 = 0xffff;
  puVar6 = l_string_2;
  do {
    if (uVar6 == 0x0) break;
    uVar6 -= 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    l_b_var8 = *puVar1 == '\0';
  } while (!l_b_var8);
  uVar6 = ~uVar6;
  uVar8 = (param_1 >> 0x10);
  puVar7 = param_1;
  if (l_b_var8) {
    if ((param_1 & 0x1) != 0x0) {
      puVar7 = puVar7 + 0x1;
      l_string_2 = l_string_2 + 0x1;
      *param_1 = *in_string_2;
      uVar6 -= 0x1;
    }
  }
  else {
    puVar7 = puVar7 + 0x2;
    l_string_2 = l_string_2 + 0x2;
    param_1 = in_string_2;
    uVar6 -= 0x1;
  }
  for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
    puVar5 = (u16 *)puVar7;
    puVar7 = (puVar7 + 0x2);
    puVar4 = (u16 *)l_string_2;
    l_string_2 = (l_string_2 + 0x2);
    *puVar5 = *puVar4;
  }
  for (uVar6 = ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
    puVar5 = (u16 *)puVar7;
    puVar7 = (puVar7 + 0x1);
    puVar4 = (u16 *)l_string_2;
    l_string_2 = (l_string_2 + 0x1);
    *puVar5 = *puVar4;
  }
  return;
}


fn str_op_1000_3da4(char *param_1) -> u16

{
  char *pcVar1;
  let uVar2: u16;
  char *pcVar3;
  let bVar4: bool;
  
  pcVar3 = param_1;
  bVar4 = true;
  uVar2 = 0xffff;
  do {
    if (uVar2 == 0x0) break;
    uVar2 -= 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar4 = *pcVar1 == '\0';
  } while (!bVar4);
  uVar2 = ~uVar2;
  if (bVar4) {
    uVar2 -= 0x1;
  }
  return uVar2;
}



uchar  str_op_1000_3dbe(char *param_1,char *param_2,param_3: u16)

{
  char *pcVar1;
  let cVar2: u8;
  char *pcVar3;
  char *pcVar4;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  pcVar4 = param_1;
  pcVar3 = param_2;
  if (param_3 != 0x0) {
    do {
      pcVar1 = pcVar3;
      pcVar3 = pcVar3 + 0x1;
      cVar2 = *pcVar1;
      if (cVar2 == '\0') break;
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = cVar2;
      param_3 -= 0x1;
    } while (param_3 != 0x0);
    for (; param_3 != 0x0; param_3 -= 0x1) {
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = '\0';
    }
  }
  return (uchar)param_1;
}



void 
str_1000_4d58(char *in_string_1,char *in_string_2,param_3: u32,param_4: u32,
             WNDCLASS16 *param_5)

{
  let uVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  char *pcStack18;
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  
  uStack10 = 0x0;
  uStack12 = 0x0;
  uVar4 = (in_string_1 >> 0x10);
  iVar2 = in_string_1;
  if ((*in_string_1 == '\0') || (*(iVar2 + 0x1) != ':')) {
    if (in_string_2 != 0x0) {
      *in_string_2 = '\0';
    }
  }
  else {
    if (in_string_2 != 0x0) {
      *in_string_2 = *in_string_1;
      *(in_string_2 + 0x1) = *(iVar2 + 0x1);
      *(in_string_2 + 0x2) = 0x0;
    }
    in_string_1 = (in_string_1 & 0xffff0000 | (iVar2 + 0x2));
  }
  uStack6 = 0x0;
  uStack8 = 0x0;
  pcStack18 = in_string_1;
  while( true ) {
    uVar5 = (pcStack18 >> 0x10);
    uVar3 = pcStack18;
    if (*pcStack18 == '\0') break;
    if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
      uStack8 = uVar3 + 0x1;
      uStack6 = uVar5;
    }
    else {
      if (*pcStack18 == '.') {
        uStack12 = uVar3;
        uStack10 = uVar5;
      }
    }
    pcStack18 = (pcStack18 & 0xffff0000 | (uVar3 + 0x1));
  }
  if ((uStack6 | uStack8) == 0x0) {
    if (param_3 != 0x0) {
      *param_3 = 0x0;
    }
  }
  else {
    if (param_3 != 0x0) {
      uVar1 = uStack8 - in_string_1;
      if (0xff < uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((param_3 & 0xffff | param_3._2_2_ << 0x10),
                       in_string_1,uVar1);
      *(param_3 + uVar1) = 0x0;
    }
    in_string_1 = CONCAT22(uStack6,uStack8);
  }
  if (((uStack10 | uStack12) != 0x0) && (in_string_1 <= uStack12)) {
    if (param_4 != 0x0) {
      uVar1 = uStack12 - in_string_1;
      if (0xff < uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((param_4 & 0xffff | param_4._2_2_ << 0x10),
                       (in_string_1 & 0xffff |
                               in_string_1._2_2_ << 0x10),uVar1);
      *(param_4 + uVar1) = 0x0;
    }
    if (param_5 == (WNDCLASS16 *)0x0) {
      return;
    }
    uVar1 = uVar3 - uStack12;
    if (0xff < uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((param_5 & 0xffff | param_5._2_2_ << 0x10),
                     CONCAT22(uStack10,uStack12),uVar1);
    *(param_5 + uVar1) = 0x0;
    return;
  }
  if (param_4 != 0x0) {
    uVar1 = uVar3 - in_string_1;
    if (0xff < uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((param_4 & 0xffff | param_4._2_2_ << 0x10),
                     (in_string_1 & 0xffff |
                             in_string_1._2_2_ << 0x10),uVar1);
    *(param_4 + uVar1) = 0x0;
  }
  if (param_5 != (WNDCLASS16 *)0x0) {
    *&param_5->style = 0x0;
  }
  return;
}
