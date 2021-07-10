use crate::global::AppContext;
use crate::defines::U32Ptr;
use crate::util::get_string_at_addr;

pub fn  poss_str_op_1000_28dc(
    ctx: &mut AppContext,
    param_1: i16) -> String

{
  let string_1: &String;
  let pi_var2: *mut u8;
  let i_var2: i16;
  let mut string_ptr_1: String;
  
  string_ptr_1 = get_string_at_addr(ctx.PTR_LOOP_1050_63fe);
  loop {
    string_1 = &string_ptr_1;
    string_ptr_1 = string_ptr_1[2..];
    i_var2 = string_1[0];
    pi_var2 = string_ptr_1;
    if (i_var2 == param_1) || (pi_var2 = (i_var2 + 0x1), pi_var2 == 0x0) {
      return pi_var2;
    }
    i_var2 = -0x1;
    loop {
      if i_var2 == 0x0 { break; }
      i_var2 += -0x1;
      string_1 = string_ptr_1;
      string_ptr_1 = (string_ptr_1 + 0x1);
          if *string_1 == 0 {
              break;
          }
    }
  }
}


pub fn unk_str_op_1000_3d3e(param_1: &mut String,in_string_2: &mut String)
{
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let uVar6: u16;
  let uVar7: u16;
  let mut l_string_2: String; 
  let mut puVar6: String; 
  let mut puVar7: String; 
  let uVar8: u16;
  let mut l_string_1: String; 
  let l_b_var8: bool;
  let mut puVar3: String; 
  let mut puVar2: String; 
  let mut puVar1: String; 
  
 // l_string_1 = (in_string_2 >> 0x10);
  l_string_2 = in_string_2;
  l_b_var8 = true;
  uVar6 = 0xffff;
  puVar6 = l_string_2;
  loop {
    if (uVar6 == 0x0) { break; }
    uVar6 -= 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    l_b_var8 = *puVar1 == '\0';
    if l_b_var8 == true {
        break;
    }
  }
  uVar6 = ~uVar6;
 // uVar8 = (param_1 >> 0x10);
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
    // TODO: refactor
  // for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
  //   puVar5 = puVar7;
  //   puVar7 = (puVar7 + 0x2);
  //   puVar4 = l_string_2;
  //   l_string_2 = (l_string_2 + 0x2);
  //   *puVar5 = *puVar4;
  // }
  // for (uVar6 = ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
  //   puVar5 = puVar7;
  //   puVar7 = (puVar7 + 0x1);
  //   puVar4 = l_string_2;
  //   l_string_2 = (l_string_2 + 0x1);
  //   *puVar5 = *puVar4;
  // }
  return;
}


pub fn str_op_1000_3da4(param_1: &mut String) -> u16

{
  let mut pcVar1: String; 
  let uVar2: u16;
  let mut pcVar3: String; 
  let bVar4: bool;
  
  pcVar3 = param_1;
  bVar4 = true;
  uVar2 = 0xffff;
  loop {
    if (uVar2 == 0x0) { break; }
    uVar2 -= 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar4 = *pcVar1 == '\0';
      if (!bVar4) == false { break; }
  }
  uVar2 = ~uVar2;
  if (bVar4) {
    uVar2 -= 0x1;
  }
  return uVar2;
}



uchar  str_op_1000_3dbe(param_1: &mut String,param_2: &mut String,param_3: u16)

{
  let mut pcVar1: String; 
  let cVar2: u8;
  let mut pcVar3: String; 
  let mut pcVar4: String; 
  let uVar5: u16;
  
 // uVar5 = (param_1 >> 0x10);
  pcVar4 = param_1;
  pcVar3 = param_2;
  if (param_3 != 0x0) {
    loop {
      pcVar1 = pcVar3;
      pcVar3 = pcVar3 + 0x1;
      cVar2 = *pcVar1;
      if (cVar2 == '\0') { break; }
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = cVar2;
      param_3 -= 0x1;
if (param_3 != 0x0) == false { break; }
    }
    // for (; param_3 != 0x0; param_3 -= 0x1) {
    //   pcVar1 = pcVar4;
    //   pcVar4 = pcVar4 + 0x1;
    //   *pcVar1 = '\0';
    // }
  }
  return (uchar)param_1;
}



pub fn
str_1000_4d58(in_string_1: &mut String,in_string_2: &mut String,param_3: u32,param_4: u32,
             WNDCLASS16 *param_5)

{
  let uVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let mut pcStack18: String; 
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  
  uStack10 = 0x0;
  uStack12 = 0x0;
 // uVar4 = (in_string_1 >> 0x10);
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
  loop {
   // uVar5 = (pcStack18 >> 0x10);
    uVar3 = pcStack18;
    if (*pcStack18 == '\0') { break; }
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
    if (param_5 == 0x0) {
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
  if (param_5 != 0x0) {
    *&param_5.style = 0x0;
  }
  return;
}
