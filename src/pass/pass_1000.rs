use crate::{string::string_1000::poss_str_op_1000_28dc, win_struct::{HINSTANCE16, SEGPTR}, winapi::{DOS3Call, GetDOSEnvironment16}};
use crate::winapi::{FatalExit, FatalAppExit16, swi, GetModuleFileName16, SegmentLimit, GlobalDOSFree16};
use crate::util::{CONCAT22, CARRY2, SUB42, ZEXT24, CONCAT11, CONCAT12, CONCAT13};
use crate::exit::exit_1000_25f2;
use crate::win_struct::WNDCLASS16;
use crate::sys_api::{dos3_call_1000_5174, mixed_dos3_call_1000_39f2, dos3_call_1000_514e, dos3_call_op_1000_35fe, dos3_op_1000_256b};
use crate::string::string_1000::{str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::mem_1000::{free_mem_1000_407a, mem_1000_167a, mem_op_1000_1dfa, mem_op_1000_1b9a, mem_op_1000_21b6, mem_op_1000_0a48, mem_op_1000_160a, mem_op_1000_1532, mem_1000_0670, mem_op_1000_0510, mem_op_1000_0838};
use crate::global::AppContext;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_op_1000_2594, call_fn_ptr_1000_0dc6};
use crate::misc::ret_op_1000_55ac;
use crate::msg_box::{msg_box_op_1000_214c, msg_box_op_1000_1f24};

// pub fn pass1_1000_010c

pub unsafe fn  pass1_1000_0368(param_1: u16, param_2: u16, param_3: u16)

{
  let puVar1: *mut u16;
  
  if ((param_1 + 0x4) == param_1) {
    (param_3 + param_2 * 0x2) = 0x0;
  }
  else {
    ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
    ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
    puVar1 = (param_2 * 0x2 + param_3);
    if (*puVar1 == param_1) {
      *puVar1 = (param_1 + 0x4);
    }
  }
  (param_1 + 0x4) = (param_3 + 0xa);
  (param_3 + 0xa) = param_1;
  return;
}

pub fn  pass1_1000_05b4(param_1: u8,param_2: i16)

{
  (param_2 + 0xa) = 0x1;
  (param_2 + 0x8) = 0x668;
  (param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
  (param_2 + 0x10) = 0x0;
  (param_2 + 0xe) = 0x0;
  return;
}

pub unsafe fn pass1_1000_0782(param_1: u32,param_2: u16,param_3: i16, in_stack_00000004: u16) -> u16

{
  (param_3 + 0xe) = 0x0;
  (param_3 + 0x10) = param_3 + 0x14;
  (param_3 + 0x8) = 0x9a0;
  pass1_1000_07ac((param_1 + 0x18),param_2,param_3);
  return 0x1;
}



pub unsafe fn  pass1_1000_07ac(param_1: u16,param_2: i16,param_3: i16)

{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  
  puVar1 = (param_3 + 0x10);
  (param_3 + 0xe) = puVar1;
  uVar3 = param_2 + (param_3 - puVar1);
  iVar2 = puVar1 + (uVar3 - uVar3 % param_1);
  (param_3 + 0x10) = iVar2;
  while (puVar1 < (iVar2 - param_1)) {
    *puVar1 = (puVar1 + param_1);
    puVar1 = (puVar1 + param_1);
  }
  *puVar1 = 0x0;
  return;
}



pub unsafe fn  pass1_1000_07fc(param_1: u16,param_2: u32) -> *mut astruct_99

{
  let paVar1: &mut Struct99;
  
  if ((param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_1,0xa,0x0,0x0);
    return 0x0;
  }
  paVar1 = mem_op_1000_0838(0x0,param_1);
  return paVar1;
}

pub unsafe fn  pass1_1000_093a(ctx: &mut AppContext, param_1: *mut i16,param_2: u16,param_3: u16) -> u16

{
  let piVar1: *mut i16;
  
  if (&ctx.PTR_LOOP_1050_000c != -0x352f) {
    pass1_1000_1e61(param_3,0xe,0x0,0x0);
    return 0x0;
  }
  *param_1 = &ctx.PTR_LOOP_1050_000e;
  if (*param_1 == 0x0) {
    &ctx.DAT_1050_0004 = 0x1;
  }
  &ctx.PTR_LOOP_1050_000e = param_1;
  piVar1 = &ctx.PTR_LOOP_1050_000a;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    mem_op_1000_0510(0x1,0x0,param_3);
  }
  return 0x1;
}



pub unsafe fn  pass1_1000_09a0(param_1: *mut u16,param_2: u16) -> *mut u8

{
  let puVar1: *mut u8;
  let uVar2: u32;
  
  *param_1 = ctx.PTR_LOOP_1050_000e;
  if (ctx.PTR_LOOP_1050_000e == 0x0) {
    *DAT_1050_0004 = 0x1;
  }
  ctx.PTR_LOOP_1050_000a = ctx.PTR_LOOP_1050_000a + -0x1;
  puVar1 = ctx.PTR_LOOP_1050_000e;
  ctx.PTR_LOOP_1050_000e = param_1;
  if (ctx.PTR_LOOP_1050_000a == 0x0) {
    uVar2 = mem_op_1000_0510(0x1,0x0,param_2);
    puVar1 = uVar2;
  }
  return puVar1;
}



pub unsafe fn  pass1_1000_09ca(param_1: i16,param_2: *mut u32) -> u16

{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u32;
  let puVar4: *mut u16;
  
  puVar1 = param_2 + 0xa;
  puVar4 =
           ((param_2 + (param_1 - puVar1) + -0x6 & 0xfffc) + puVar1);
  *puVar4 = 0x1;
  param_2[0x7] = puVar1;
  puVar4[0x2] = puVar4;
  puVar4[0x1] = puVar4;
  param_2[0x8] = puVar4;
  if (((param_2 + 0x6) & 0x7) == 0x2) {
    param_2[0x9] = 0x8;
  }
  else {
    uVar3 = param_2;
    iVar2 = (uVar3 + 0x18);
    param_2[0x9] = (iVar2 - 0x5 & !-(iVar2 + 0x3 < 0x8)) + 0x8;
  }
  puVar4[-0x1] = (puVar4 - puVar1);
  *puVar1 = (puVar4 - puVar1) | 0x2;
  param_2[0xc] = puVar4;
  param_2[0xb] = puVar4[0x1];
  (puVar4[0x1] + 0x4) = puVar1;
  puVar4[0x1] = puVar1;
  param_2[0x4] = 0xe08;
  return *puVar1 & 0xfffc;
}


pub unsafe fn  pass1_1000_0c32(param_1: u16,param_2: u16,param_3: u16) -> u32

{
  let puVar1: *mut u16;
  let pbVar2: *mut u8;
  let piVar3: *mut i16;
  let uVar4: u32;
  let uVar5: u16;
  let puVar6: *mut u16;
  let iVar7: i16;
  let puVar8: *mut u16;
  let uVar9: u16;
  let uStack14: u16;
  let puStack8: *mut u16;
  let uStack6: u16;
  
  puVar8 = (param_3 + 0xe);
  uStack6 = 0x0;
  puVar6 = puVar8;
  loop {
    loop {
      uVar5 = *puVar6;
      if (param_1 <= uVar5) {
        uVar5 = (uVar5 & 0xfffc) - param_1;
        puVar1 = (param_3 + 0x12);
        puStack8 = puVar6;
        if (*puVar1 < uVar5 || *puVar1 == uVar5) {
          uStack14 = param_1;
          if ((param_2 & 0x6) == 0x0) {
            puStack8 = (uVar5 + puVar6);
            puStack8[-0x1] = uVar5;
            *puVar6 = uVar5 | 0x2;
            puVar8 = puVar6[0x1];
            pbVar2 = (puStack8 + param_1);
            *pbVar2 = *pbVar2 | 0x2;
            *puStack8 = param_1 | 0x1;
          }
          else {
            *puVar6 = param_1 & 0xff00 | puVar6 & 0x2 | param_1 & 0xff | 0x1;
            (puVar6[0x2] + 0x2) = puVar6[0x1];
            (puVar6[0x1] + 0x4) = puVar6[0x2];
            puVar8 = (puVar6 + param_1);
            (puVar8 + (uVar5 - 0x2)) = uVar5;
            *puVar8 = uVar5 | 0x2;
            uVar5 = (param_3 + 0x10);
            puVar8[0x2] = uVar5;
            puVar8[0x1] = (uVar5 + 0x2);
            ((uVar5 + 0x2) + 0x4) = puVar8;
            (uVar5 + 0x2) = puVar8;
          }
        }
        else {
          puVar8 = puVar6[0x1];
          (puVar6[0x2] + 0x2) = puVar8;
          (puVar6[0x1] + 0x4) = puVar6[0x2];
          puVar1 = puVar6;
          puVar1 = puVar1 | 0x1;
          uStack14 = *puVar6 & 0xfffc;
          (puVar6 + uStack14) = (puVar6 + uStack14) | 0x2;
        }
        (param_3 + 0xe) = puVar8;
        if ((param_2 & 0x1) != 0x0) {
          puVar6 = puStack8;
          // TODO
          // for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0;
          //     uVar5 -= 0x1) {
          //   *puVar6 = 0x0;
          // }
          if ((uStack14 - 0x2 & 0x1) != 0x0) {
            *puVar6 = 0x0;
          }
        }
        if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
          *(param_3 + 0x4) =
               *((param_3 + 0x10) + 0x2) & 0xfffc;
          uVar4 = (param_3 + 0x4);
          pbVar2 = (uVar4 + 0x3);
          *pbVar2 = *pbVar2 | 0x80;
        }
        piVar3 = (param_3 + 0xa);
        *piVar3 = *piVar3 + 0x1;
        return CONCAT22(0x1050,puStack8 + 0x1);
      }
      if (uStack6 < uVar5) {
        uStack6 = uVar5;
      }
      puVar6 = puVar6[0x1];
      if puVar6 == puVar8 {
        break;
      }
    } 
    if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) {break;}
    uVar4 = param_3;
   // uVar9 = (uVar4 >> 0x10);
    iVar7 = uVar4;
    if ((iVar7 + 0x34) == 0x0) {break;}
    uStack6 = ((iVar7 + 0x34))();
    if ((uStack6 < param_1) || (puVar6 = (param_3 + 0xe), puVar6 == 0x0)
       ) {break;}
  }
  *(param_3 + 0x4) = uStack6 & 0xfffc;
  return 0x0;
}


pub unsafe fn  pass1_1000_0e08(param_1: i16,param_2: u16) -> u16

{
  let puVar1: *mut u16;
  let pbVar2: *mut u8;
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let bVar6: bool;
  let uVar7: u32;
  
  puVar5 = (param_1 + -0x2);
  bVar6 = (puVar5 & 0x2) != 0x0;
  if (bVar6) {
    puVar1 = puVar5;
    puVar1 = puVar1 & 0xfe;
  }
  else {
    puVar4 = (puVar5 - (param_1 + -0x4));
    puVar1 = puVar4;
    *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
    puVar5 = puVar4;
  }
  puVar4 = ((*puVar5 & 0xfffc) + puVar5);
  if ((puVar4 & 0x1) == 0x0) {
    puVar1 = puVar5;
    *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
    if (puVar4 == ctx.PTR_LOOP_1050_000e) {
      ctx.PTR_LOOP_1050_000e = puVar5;
    }
    (puVar4[0x2] + 0x2) = puVar4[0x1];
    (puVar4[0x1] + 0x4) = puVar4[0x2];
    puVar4 = ((*puVar5 & 0xfffc) + puVar5);
  }
  puVar4[-0x1] = *puVar5 & 0xfffc;
  uVar3 = *DAT_1050_0004;
  puVar1 = puVar4 + -0x1;
  if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
    uVar3 = *puVar5 & 0xfffc;
    *DAT_1050_0004 = uVar3;
  }
  puVar1 = puVar4;
  puVar1 = puVar1 & 0xfd;
  if (bVar6) {
    if ((ctx.PTR_LOOP_1050_0010 + 0x2) != ctx.PTR_LOOP_1050_0010) {
      pbVar2 = (ctx.DAT_1050_0004 + 0x3);
      *pbVar2 = *pbVar2 & 0x7f;
    }
    puVar5[0x2] = ctx.PTR_LOOP_1050_0010;
    uVar3 = (ctx.PTR_LOOP_1050_0010 + 0x2);
    puVar5[0x1] = uVar3;
    ((ctx.PTR_LOOP_1050_0010 + 0x2) + 0x4) = puVar5;
    (ctx.PTR_LOOP_1050_0010 + 0x2) = puVar5;
  }
  ctx.PTR_LOOP_1050_000a = ctx.PTR_LOOP_1050_000a + -0x1;
  if (ctx.PTR_LOOP_1050_000a == 0x0) {
    uVar7 = mem_op_1000_0510(0x1,0x0,param_2);
    uVar3 = uVar7;
  }
  return uVar3;
}



pub unsafe fn pass1_1000_0ed4(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: *mut u16,param_7: u16) -> i32

{
  let puVar1: *mut u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  u16 **ppuVar4;
  let uVar5: u16;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
 let UVar8: u16;
  let UVar9: u16;
  let UVar10: u16;
  let lStack12: i32;
  let uStack8: u16;
  let UStack: u16;
  let UStack4: u16;
  
  if ((&ctx.PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
    UStack6 = 0x0;
    UStack4 = &ctx.PTR_LOOP_1050_0002;
    if ((param_3 & 0x8) == 0x0) {
      ppuVar4 = &param_6;
    }
    else {
      ppuVar4 = 0x0;
      param_2 = 0x0;
    }
    lStack12 = CONCAT22(param_2,ppuVar4);
    uStack8 = pass1_1000_0fb8(param_1,param_4,param_6,param_5,param_3,
                              ppuVar4,param_2);
    if (uStack8 == 0x0) {
      return CONCAT22(param_7,param_6);
    }
    if ((param_3 & 0x8) == 0x0) {
      lStack12 = mem_op_1000_0a48(param_3,param_4,param_5,CONCAT22(UStack4,UStack6),
                                  param_1);
     // uVar3 = (lStack12 >> 0x10);
      puVar7 = lStack12;
      if (lStack12 != 0x0) {
        puVar6 = param_6;
        // TODO
        // for (uVar5 = uStack8 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
        //   puVar2 = puVar7;
        //   puVar7 = puVar7 + 0x1;
        //   puVar1 = puVar6;
        //   puVar6 = puVar6 + 0x1;
        //   *puVar2 = *puVar1;
        // }
        
        // TODO
        // for (uVar5 = ((uStack8 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
        //   puVar2 = puVar7;
        //   puVar7 = (puVar7 + 0x1);
        //   puVar1 = puVar6;
        //   puVar6 = (puVar6 + 0x1);
        //   *puVar2 = *puVar1;
        // }
        call_fn_ptr_1000_0dc6(param_6,param_7,param_1);
      }
      return lStack12;
    }
    if ((param_5 | param_4) == 0x0) {
      return 0x0;
    }
    UVar8 = 0x5;
    UVar9 = UStack6;
    UVar10 = UStack4;
  }
  else {
    UVar8 = 0xe;
    UVar9 = 0x0;
    UVar10 = 0x0;
  }
  pass1_1000_1e61(param_1,UVar8,UVar9,UVar10);
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1000_0fb8(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
                    param_6: *mut u16,param_7: u16) -> u16

{
  let puVar1: *mut u16;
  let bVar2: u8;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let puVar8: *mut u16;
  let uVar9: u32;
  let uStack4: u16;
  
  if ((param_4 | param_2) == 0x0) {
    pass1_1000_1e61(param_1,0x4,PTR_LOOP_1050_0000,PTR_LOOP_1050_0002);
    if ((param_7 | param_6) != 0x0) {
      param_6[0x1] = 0x0;
      *param_6 = 0x0;
      return 0x0;
    }
    return 0x1;
  }
  bVar2 = ctx.PTR_LOOP_1050_000c & 0x7;
  if ((ctx.PTR_LOOP_1050_000c & 0x7) != 0x0) {
    if (bVar2 == 0x1) {
      uVar3 = (ctx.PTR_LOOP_1050_0000 + 0x18);
      if (false) {
        return 0x0;
      }
      if (param_4 == 0x0) {
        if (param_2 <= uVar3) {
          return 0x0;
        }
        return uVar3;
      }
      return uVar3;
    }
    if (bVar2 != 0x2) {
      if (bVar2 != 0x3) {
        if ((param_7 | param_6) != 0x0) {
          param_6[0x1] = 0x0;
          *param_6 = 0x0;
          return 0x0;
        }
        return 0x1;
      }
      if ((((param_7 | param_6) != 0x0) && (param_4 == 0x0)) &&
         ((false || (param_2 <= (ctx.PTR_LOOP_1050_0000 + 0x1c))))) {
        uVar9 = pass1_1000_1284(CONCAT22(0x1050,param_3),param_1);
        uVar3 = uVar9;
        if (uVar9 <= CONCAT22(param_4,param_2)) {
          return uVar3;
        }
        if ((false) && (uVar3 <= param_2)) {
          return uVar3;
        }
        return param_2;
      }
      iVar5 = mem_1000_0670(param_5,CONCAT22(param_7,param_6),param_2,0x0,
                            param_4,param_1);
      if (iVar5 != 0x0) {
        return 0x0;
      }
      if ((param_7 | param_6) != 0x0) {
        return 0x0;
      }
      return 0x1;
    }
  }
  puVar8 = (param_3 + -0x2);
  uVar3 = *puVar8 & 0x7ffc;
  uStack4 = uVar3 - 0x2;
  if (((param_3 + -0x1) & 0x80) != 0x0) {
    uStack4 = uVar3 - 0x6;
  }
  if ((true) && ((param_4 != 0x0 || (uStack4 < param_2)))) {
    if (true) {
      if (param_4 != 0x0) {
        return uStack4;
      }
      if ((ctx.PTR_LOOP_1050_0000 + 0x1c) < param_2) {
        return uStack4;
      }
    }
  }
  BVar4 = pass1_1000_115c(param_2,puVar8);
  if (BVar4 == 0x0) {
    return uStack4;
  }
  if ((param_5 & 0x1) != 0x0) {
    uVar3 = (*puVar8 & 0x7ffc) - 0x2;
    if (uStack4 < param_2) {
      puVar7 = (uStack4 + param_3);
      iVar5 = -uStack4;
    }
    else {
      if (uVar3 <= param_2) {
        return 0x0;
      }
      puVar7 = (param_2 + param_3);
      iVar5 = -param_2;
    }
    uVar3 += iVar5;
    // TODO
    // for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
    //   puVar1 = puVar7;
    //   puVar7 = puVar7 + 0x1;
    //   *puVar1 = 0x0;
    // }
    if ((uVar3 & 0x1) != 0x0) {
      *puVar7 = 0x0;
    }
  }
  return 0x0;
}



pub unsafe fn  pass1_1000_115c(param_1: i16,param_2: *mut u16) -> bool

{
  let pbVar1: *mut u8;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  let iVar6: i16;
  let uStack4: u16;
  
  uVar3 = *param_2 & 0x7ffc;
  uVar4 = param_1 + 0x5 & 0xfffc;
  uVar4 = (uVar4 - 0x8 & !-(uVar4 < 0x8)) + 0x8;
  if (uVar3 < uVar4) {
    puVar5 = (uVar3 + param_2);
    if (((puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
      return 0x0;
    }
    if (puVar5 == ctx.PTR_LOOP_1050_000e) {
      ctx.PTR_LOOP_1050_000e = puVar5[0x1];
    }
    (puVar5[0x2] + 0x2) = puVar5[0x1];
    (puVar5[0x1] + 0x4) = puVar5[0x2];
    uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      puVar2 = param_2;
      *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
      pbVar1 = (puVar5 + (*puVar5 & 0xfffc));
      *pbVar1 = *pbVar1 | 0x2;
      return 0x1;
    }
  }
  else {
    uStack4 = uVar3 - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      return 0x1;
    }
    puVar5 = (uVar3 + param_2);
    if ((puVar5 & 0x1) == 0x0) {
      uStack4 += *puVar5 & 0xfffc;
      if (puVar5 == ctx.PTR_LOOP_1050_000e) {
        ctx.PTR_LOOP_1050_000e = puVar5[0x1];
      }
      (puVar5[0x2] + 0x2) = puVar5[0x1];
      (puVar5[0x1] + 0x4) = puVar5[0x2];
    }
    if (*DAT_1050_0004 < uStack4) {
      *DAT_1050_0004 = uStack4;
    }
  }
  *param_2 = *param_2 & 0x8003 | uVar4;
  (uVar4 + param_2) = uStack4 | 0x2;
  iVar6 = uVar4 + param_2;
  (iVar6 + 0x4) = ctx.PTR_LOOP_1050_0010;
  (iVar6 + 0x2) = (ctx.PTR_LOOP_1050_0010 + 0x2);
  ((ctx.PTR_LOOP_1050_0010 + 0x2) + 0x4) = iVar6;
  (ctx.PTR_LOOP_1050_0010 + 0x2) = iVar6;
  ((iVar6 + uStack4) + -0x2) = uStack4;
  pbVar1 = (iVar6 + uStack4);
  *pbVar1 = *pbVar1 & 0xfd;
  return 0x1;
}



pub fn  pass1_1000_1284(param_1: u32,param_2: u16) -> u32

{
  let bVar1: u8;
  let uVar2: u16;
  let uVar3: u32;
  let bVar4: u8;
  let uVar5: u16;
  let bVar6: bool;
  let DVar7: u32;
  let uStack6: u16;
  let iStack4: i16;
  
  if ((&ctx.PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(param_2,0xe,0x0,0x0);
    return 0xffffffff;
  }
  bVar1 = &ctx.PTR_LOOP_1050_000c;
  bVar4 = bVar1 & 0x7;
  if ((bVar1 & 0x7) != 0x0) {
    if (bVar4 == 0x1) {
      uVar3 = 0x0;
      return (uVar3 + 0x18);
    }
    if (bVar4 != 0x2) {
      if (bVar4 != 0x3) {
        return 0xffffffff;
      }
      DVar7 = mem_op_1000_1532(param_2);
      return CONCAT22((DVar7 >> 0x10) - (DVar7 < 0x14),DVar7 - 0x14
                     );
    }
  }
  uVar2 = (param_1 + -0x2);
  uVar5 = uVar2 & 0x7ffc;
  uStack6 = uVar5 - 0x2;
  iStack4 = 0x0;
  if ((uVar2 & 0x8000) != 0x0) {
    bVar6 = uStack6 < 0x4;
    uStack6 = uVar5 - 0x6;
    iStack4 = -bVar6;
  }
  return CONCAT22(iStack4,uStack6);
}


pub unsafe fn pass1_1000_15ce(param_1: *mut u16,param_2: u16,param_3: u16)

{
  let puVar1: *mut u16;
  let uVar2: u16;
  
  uVar2 = param_2 | param_1;
  while (uVar2 != 0x0) {
    puVar1 = *param_1;
    param_2 = param_1[0x1];
    GlobalDOSFree16(param_3);
    param_1 = puVar1;
    param_3 = ctx.s_tile2_bmp_1050_1538;
    uVar2 = param_2 | puVar1;
  }
  return;
}



pub unsafe fn pass1_1000_16aa(
  param_1: *mut u16,
  param_2: u16,
  param_3: u16,
  param_4: u16,
  param_5: u16, 
  param_6: u16) -> u16

{
  let uVar1: u16;
  let lVar2: i32;
  
  if ((param_2 | param_1) == 0x0) {
    uVar1 = mem_1000_167a(param_3,param_5,param_4);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_16ee(param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return lVar2;
}



pub fn pass1_1000_16ee(param_1: u16,param_2: u16,param_3: u16)
{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}


pub fn  pass1_1000_17e8(param_1: *mut u8,param_2: *mut u8) -> *mut u8

{
  let pu_var1: *mut u8;
  pu_var1 = ctx.PTR_LOOP_1050_5f34;
  ctx.PTR_LOOP_1050_5f34 = param_1;
  ctx.PTR_LOOP_1050_5f36 = param_2;
  return pu_var1;
}



pub fn  pass1_1000_180c(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u8;
  let lVar2: i32;
  
  if ((ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(param_2,param_3);
    if ((param_2 | puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar2 = mem_op_1000_0a48(0x0,param_1,0x0,CONCAT22(ctx.PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c)
                           ,param_3);
  return lVar2;
}



pub fn pass1_1000_183c(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u8;
  let lVar2: i32;
  
  puVar1 = 0x0;
  if ((param_2 * param_1 >> 0x10) != 0x0) {
    return 0x0;
  }
  if (((ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0) &&
     (ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0,param_3), ctx.PTR_LOOP_1050_5f2e = puVar1,
     (puVar1 | ctx.PTR_LOOP_1050_5f2c) == 0x0)) {
    return 0x0;
  }
  lVar2 = mem_op_1000_0a48(0x1,(param_2 * param_1),0x0,
                           CONCAT22(ctx.PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c),param_3);
  return lVar2;
}



pub unsafe fn pass1_1000_188e(
  param_1: *mut u16,
  param_2: u16,
  param_3: u16,
  param_4: u16,
  param_5: u16, 
  param_6: u16) -> u16
{
  let uVar1: u16;
  let lVar2: i32;
  
  if ((param_2 | param_1) == 0x0) {
    uVar1 = pass1_1000_180c(param_3,param_4,param_5);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_18d2(param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return lVar2;
}

pub fn  pass1_1000_18d2(param_1: u16,param_2: u16,param_3: u16)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}


pub fn pass1_1000_1a54(param_1: u16,param_2: i16,param_3: u16,param_4: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  
  if ((param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_4,0xa,0x0,0x0);
    return 0x0;
  }
  uVar1 = pass1_1000_1ab0(param_1);
  if (uVar1 < (param_2 + 0x18) + 0x14) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = (param_2 + 0x1a);
    (param_2 + 0x1a) = uVar1;
    (param_2 + 0x1c) = uVar1 >> 0x2;
  }
  return uVar2;
}



pub fn  pass1_1000_1ab0(param_1: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  
  if (param_1 == 0x2000) {
    return 0x2000;
  }
  if (param_1 < 0xfff0) {
    if (param_1 < 0x1001) {
      return 0x1000;
    }
    uVar1 = 0x2000;
    if (param_1 < 0x2001) {
      loop {
        uVar2 = uVar1;
        uVar1 = uVar2 >> 0x1;
        if param_1 > u_var1 {
          break;
        }
      }
      return uVar2 & 0xfffe;
    }
    while (uVar1 *= 0x2, uVar1 != 0x0) {
      if (param_1 <= uVar1) {
        return (uVar1 + 0x10 & -(uVar1 < 0xfff0)) - 0x10;
      }
    }
  }
  return 0xfff0;
}



pub fn  pass1_1000_1afe(param_1: u16,param_2: u32,param_3: u16, unaff_CS: u16) -> bool

{
  let uVar1: u16;

  if (param_1 == 0x0) {
    uVar1 = 0x0;
  }
  else {
    uVar1 = param_1 + 0x1 & 0xfffe;
  }
  if ((param_2 + 0x14) == -0x4153) {
    if ((uVar1 < param_1) || ((param_2 + 0x1a) - 0x14 < uVar1)) {
      pass1_1000_1e61(unaff_CS,0x3,param_2,param_3);
    }
    else {
      if ((param_2 + 0x2) == 0x0) {
        (param_2 + 0x18) = uVar1;
        return 0x1;
      }
    }
    return 0x0;
  }
  pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
  return 0x0;
}




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_1e61(ctx: &mut AppContext, param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u16

{
  let i_var1: i16;
  let b_var2: bool;
  let u_var3: u16;
  let u_stack64: u16;
  let u_stack62: u16;
  let u_stack60: u16;
  let pcStack6: u32;
  let pu_stack4: *mut u8;
  let u_var3: u16;
  
  u_var3 = ctx.data_seg;
  u_stack62 = param_3;
  u_stack60 = param_4;
  u_stack64 = param_2;
  pu_stack4 = ctx.data_seg;
  if (true) {
    pcStack6 = &ctx.PTR_PTR_1050_5f1a;
    if ((ctx.PTR_LOOP_1050_5f1c | ctx.PTR_PTR_1050_5f1a) == 0x0) {
      pcStack6 = 0x0;
      pu_stack4 = 0x0;
    }
    else {
      i_var1 = mem_op_1000_21b6(ctx.PTR_PTR_1050_5f1a,ctx.PTR_LOOP_1050_5f1c);
      pcStack6 = ctx.PTR_PTR_1050_5f1a;
      pu_stack4 = ctx.PTR_LOOP_1050_5f1c;
      if (i_var1 == 0x0) {
        ctx.PTR_PTR_1050_5f1a = &ctx.PTR_PTR_1050_1f7e;
        ctx.PTR_LOOP_1050_5f1c = &ctx.PTR_LOOP_1050_1000;
        pcStack6 = &ctx.PTR_PTR_1050_1f7e;
        pu_stack4 = &ctx.PTR_LOOP_1050_1000;
      }
    }
    if ((pu_stack4 | pcStack6) != 0x0) {
      b_var2 = msg_box_op_1000_1f24
                        (&ctx.PTR_PTR_1050_5f1a,ctx.data_seg,0x0,0x1000);
      if (b_var2 == 0x0) {
        u_var3 = (*pcStack6)(0x1000,&u_stack64);
      }
      else {
        pu_stack4 = 0x0;
        pcStack6 = 0x0;
        u_var3 = 0x0;
      }
      if ((pu_stack4 | pcStack6) != 0x0) {
        pass1_1000_1f68(u_var3);
      }
      return u_var3;
    }
  }
  return 0x0;
}


pub fn pass1_1000_1f68()
{
  if ((true) &&
     (ctx.PTR_LOOP_1050_5f26 = ctx.PTR_LOOP_1050_5f26 + -0x1, ctx.PTR_LOOP_1050_5f26 < 0x0)) {
    ctx.PTR_LOOP_1050_5f26 = 0x0;
  }
  return;
}



pub unsafe fn pass1_1000_1f7e(param_1: *mut u16,param_2: u16) -> bool

{
  let c_var1: u8;
  let b_var2: bool;
  let u_var3: u16;
  let i_var4: i16;
  // let mut pcVar5: String; 
  let mut pcVar5 = "".to_string();

  u_var3 = *param_1;
  if (false) {
    return 0x0;
  }
  if (u_var3 == 0xf) {
//LAB_1000_1fb6:
    i_var4 = 0x1;
  }
  else {
    if (u_var3 < 0x10) {
      c_var1 = u_var3;
      if (c_var1 == '\x02') {
        // TODO
        // goto LAB_1000_1fb6;
      }
      if (('\0' < (c_var1 + -0x2)) && ((c_var1 + -0x3) < 0xf)) {
        i_var4 = 0x0;
        // TODO
        // goto LAB_1000_1fbe;
      }
    }
    i_var4 = 0x0;
    u_var3 = 0x1;
  }
//LAB_1000_1fbe:
  pcVar5 = pass1_1000_1fd2(u_var3);
  b_var2 = msg_box_op_1000_214c
                    (0x0,i_var4,pcVar5,(pcVar5 >> 0x10),param_2);
  return b_var2;
}



pub fn pass1_1000_1fd2(param_1: i16) -> *mut u8

{
  if (param_1 == 0x2) {
    return "Out of memory.  Please free some memory, then choose retry.";
  }
  return CONCAT22(0x1000,param_1 * 0x17 + 0x1c7a);
}



pub fn pass1_1000_1fea() -> bool

{
  let puVar1: *mut u8;
  let bVar2: bool;
  
  if (((true) &&
      (puVar1 = ctx.PTR_LOOP_1050_5f22 + 0x1, bVar2 = ctx.PTR_LOOP_1050_5f22 == 0x0,
      ctx.PTR_LOOP_1050_5f22 = puVar1, bVar2)) &&
     ((ctx.PTR_LOOP_1050_5f20 | ctx.PTR_LOOP_1050_5f1e) != 0x0)) {
    ctx.PTR_LOOP_1050_5f22 = &ctx.PTR_LOOP_1050_0002;
  }
  if (true) {
    return 0x1;
  }
  return 0x0;
}



pub unsafe fn pass1_1000_201c(param_1: i16,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  
  if (param_1 == 0x0) {
    (param_2 + 0x6) = 0x0;
    (param_2 + 0x4) = 0x0;
  }
  uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
  while (uVar3 != 0x0) {
    BVar4 = pass1_1000_206c((param_2 + 0x4),(param_2 + 0x6));
    if (BVar4 == 0x0) {
      uVar2 = (param_2 + 0x4);
     // uVar6 = (uVar2 >> 0x10);
      iVar5 = uVar2;
      uVar1 = (iVar5 + 0x2c);
      (param_2 + 0x4) = (iVar5 + 0x2a);
      (param_2 + 0x6) = uVar1;
    }
    else {
      mem_op_1000_1b9a(0x1,(param_2 + 0x4),(param_2 + 0x6),
                       param_3);
    }
    uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
  }
  return;
}



pub unsafe fn pass1_1000_206c(param_1: u16,param_2: u16) -> bool

{
  let uVar1: u16;
  
  uVar1 = pass1_1000_21d2(0x2,0x42,param_1,param_2,0x1);
  if ((uVar1 != 0x0) && ((param_1 + 0x14) == -0x4153)) {
    return 0x1;
  }
  return 0x0;
}



pub fn pass1_1000_20a2(param_1: u16,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uStack8: u16;
  let uStack4: u16;
  
  iVar1 = (param_1 + 0x2e);
  uVar2 = (param_1 + 0x30);
  uStack8 = 0x0;
  uVar3 = (iVar1 + 0x4);
  uStack4 = (iVar1 + 0x6);
  uVar7 = 0x0;
  if ((uStack4 | uVar3) != 0x0) {
    while ((uVar6 = uVar3, uVar4 = uStack4, uVar6 != param_1 || (uStack4 != param_2))) {
      uVar3 = (uVar6 + 0x2a);
      uStack4 = (uVar6 + 0x2c);
      uVar7 = uVar6;
      uStack8 = uVar4;
      if ((uStack4 | uVar3) == 0x0) {
        return;
      }
    }
    if ((uStack8 | uVar7) != 0x0) {
      uVar2 = (uVar6 + 0x2c);
      (uVar7 + 0x2a) = (uVar6 + 0x2a);
      (uVar7 + 0x2c) = uVar2;
      return;
    }
    uVar5 = (uVar6 + 0x2c);
    (iVar1 + 0x4) = (uVar6 + 0x2a);
    (iVar1 + 0x6) = uVar5;
  }
  return;
}


pub unsafe fn pass1_1000_21d2(
  param_1: u8,
  param_2: i32,
  param_3: u16,
  param_4: u16,
  param_5: u8) -> u16

{
  let mut uVar1 = 0u32;
  let b_var2: bool;
  
  if (true) {
    b_var2 = mem_op_1000_1dfa(0x0,param_1,param_3,param_4);
    if (b_var2 != 0x0) {
      return 0x0;
    }
    if ((param_1 & 0x4) == 0x0) {
      uVar1 = SegmentLimit(param_4);
      if (!((uVar1 >> 0x10) & 0x1)) {
        return 0x0;
      }
      if (param_2 == 0x0) {
        return 0x1;
      }
      if (CARRY4(param_3,param_2 - 0x1)) {
        return 0x0;
      }
      if (param_3 + (param_2 - 0x1) <= uVar1) {
        return 0x1;
      }
      return 0x0;
    }
  }
  b_var2 = pass1_1000_22c0(param_3,param_4,param_2,param_2._2_2_,_param_1);
  if (b_var2 == 0x0) {
    return 0x0;
  }
  return 0x1;
}



pub unsafe fn pass1_1000_2242(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: u16,
                     param_6: *mut u8) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let bVar3: bool;
  
  uVar1 = param_2 | param_1;
  loop {
    if (uVar1 == 0x0) {
      return 0x0;
    }
    uVar1 = param_1;
    if (param_2 != 0x0) {
      uVar1 = 0xffff;
    }
    if (CARRY2(param_3,uVar1) != false) {
      uVar1 = -param_3;
    }
    bVar3 = param_1 < uVar1;
    param_1 -= uVar1;
    param_2 -= bVar3;
    uVar2 = (*param_6)(uVar1,param_5,param_3,param_4);
    if (uVar2 != 0x0) {break;}
    bVar3 = CARRY2(param_3,uVar1);
    param_3 += uVar1;
    param_4 += bVar3 * 0x100;
    uVar1 = param_2 | param_1;
  }
  return CONCAT22(param_2 + CARRY2(uVar2,param_1),uVar2 + param_1);
}



pub unsafe fn pass1_1000_22c0(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
                      param_5: u1) -> bool

{
  let uVar1: u32;
  
  uVar1 = pass1_1000_2242(param_3,param_4,param_1,param_2,param_5,0x1dfa);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}


pub fn pass1_1000_24db(param_1: i16,param_2: u16)
{
  let pcVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  let cVar4: u8;
  let uVar5: u16;
  
  iVar2 = param_2 + 0x1;
  uVar5 = SUB42(ctx.data_seg,0x0);
  ctx.PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar3 = 0x1;
  if (false) {
    fn_ptr_op_1000_2594(0x68b6,0x68b6);
    fn_ptr_op_1000_2594(&ctx.PTR_LOOP_1050_6210,0x620c);
    ret_op_1000_55ac(param_1,uVar3,uVar5,iVar2);
  }
  cVar4 = (uVar3 >> 0x8);
  fn_ptr_op_1000_2594(&ctx.PTR_LOOP_1050_6210,&ctx.PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594(&ctx.PTR_LOOP_1050_6210,&ctx.PTR_LOOP_1050_6210);
  dos3_op_1000_256b();
  if (cVar4 == '\0') {
    if (true) {
      pcVar1 = swi(0x21);
      (*pcVar1)();
    }
    else {
      DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
  }
  return;
}


pub fn pass1_1000_25a8(param_1: u16,param_2: u16)
{
  pass1_1000_2913(0xfc,param_1,param_2);
  pass1_1000_2913(0xff,param_1,param_2);
  return;
}




pub unsafe fn pass1_1000_25d2(param_1: i16,param_2: i16,param_3: u16,param_4: u16,param_5: u16
                     ,param_6: *mut u8) -> *mut i16

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let mut pcVar3: String; 
  let puVar4: *mut u8;
  let uVar5: u16;
  let piVar6: *mut i16;
  // let mut str: String;
  let pcVar10 = "".String();
  let piVar7: *mut i16;
  let mut pcVar8: String; 
  let iVar9: i16;
  
  puVar4 = (param_2 + 0x1 & 0xfffe);
  if ((puVar4 < &param_1) &&
     (uVar5 = -(puVar4 + -&param_1), puVar1 = &ctx.PTR_LOOP_1050_000a,
     *puVar1 < uVar5 || *puVar1 == uVar5)) {
    puVar1 = &ctx.PTR_LOOP_1050_000c;
    if (uVar5 <= *puVar1 && *puVar1 != uVar5) {
      &ctx.PTR_LOOP_1050_000c = uVar5;
    }
                    // WARNING: Could not recover jumptable at 0x100025f0. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    piVar6 = (*param_6)();
    return piVar6;
  }
  iVar9 = 0x0;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(iVar9,param_3,param_4);
  pcVar10 = poss_str_op_1000_28dc(ctx, 0x0);
  if (pcVar10 != 0x0) {
    iVar9 = 0x9;
    if (*pcVar10 == 'M') {
      iVar9 = 0xf;
    }
    pcVar10 = pcVar10 + iVar9;
    iVar9 = 0x22;
    pcVar8 = pcVar10;
    loop {
      if (iVar9 == 0x0) {break;}
      iVar9 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
      if *pcVar3 == '\r' {
        break;
      }
    } 
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,pcVar10);
  FatalExit();
  piVar6 = &ctx.PTR_LOOP_1050_63fe;
  loop {
    piVar2 = piVar6;
    piVar6 = piVar6 + 0x1;
    iVar9 = *piVar2;
    piVar7 = piVar6;
    if ((iVar9 == param_1) || (piVar7 = (iVar9 + 0x1), piVar7 == 0x0)) {
      return piVar7;
    }
    iVar9 = -0x1;
    loop {
      if (iVar9 == 0x0) {break;}
      iVar9 += -0x1;
      piVar2 = piVar6;
      piVar6 = (piVar6 + 0x1);
      if *piVar2 == '\0' {
        break;
      }
    } 
  } 
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 :
// 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

pub fn pass1_1000_262c(
    param_1: *mut u8,
    param_2: *mut u8,
    param_3: &str,
    param_4: HINSTANCE16
)

{
  let mut pcVar1: String; 
  let cVar2: u8;
  let uVar3: u16;
  let puVar4: *mut u8;
  let IVar5: i16;
  let uVar6 = 0u16;
  let uVar7: u16;
  let uVar8: u16;
  let in_DX: *mut u8;
  let iVar9: i16;
  char **ppcVar10;
  let mut pcVar11: String; 
  let mut pcVar12: String; 
  let mut pcVar13: String; 
  let unaff_ES: u16;
  let uVar14: u16;
  let puStack4: *mut u8;
  let mut pCStack2: String; 
  
  ctx.PTR_LOOP_1050_5fd2 = param_1;
  ctx.PTR_LOOP_1050_5fd4 = param_2;
  param_2 = 0x263d;
  param_1 = pass1_1000_2950(0x8,in_DX,unaff_ES,param_4);
  pCStack2 = ctx.PTR_LOOP_1050_5f4c;
  puStack4 = in_DX;
  ctx.PTR_LOOP_1050_5fc2 = param_1;
  ctx.PTR_LOOP_1050_5fc4 = in_DX;
  IVar5 = GetModuleFileName16(param_4,(s_You_may_not_run_a_turn__The_game_1050_00df
                                             + 0x25),param_1);
  puStack4[IVar5] = 0x0;
  iVar9 = 0x1;
  ctx.PTR_LOOP_1050_5fb8 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  pcVar11 = (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
//LAB_1000_266c:
  loop {
    loop {
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
      if cVar2 != ' ' {
        break;
      }
    } 
    if cVar2 != '\t' {
      break;
    }
  } 
  if ((cVar2 != '\r') && (cVar2 != '\0')) {
    ctx.PTR_LOOP_1050_5fb8 = ctx.PTR_LOOP_1050_5fb8 + 0x1;
    loop {
      pcVar11 = pcVar11 + -0x1;
//LAB_1000_267f:
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
      if ((cVar2 == ' ') || (cVar2 == '\t')) {
        // goto LAB_1000_266c;
      }
      if ((cVar2 == '\r') || (cVar2 == '\0')) {break;}
      if (cVar2 == '\"') {
//LAB_1000_26b8:
        loop {
          loop {
            loop {
              pcVar1 = pcVar11;
              pcVar11 = pcVar11 + 0x1;
              cVar2 = *pcVar1;
              if ((cVar2 == '\r') || (cVar2 == '\0')) {
                // goto LAB_1000_26e8;
              }
              if (cVar2 == '\"') {
                // goto LAB_1000_267f;
              }
              if (cVar2 == '\\') {break;}
              iVar9 += 0x1;
            }
            uVar7 = 0x0;
            loop {
              pcVar13 = pcVar11;
              uVar7 += 0x1;
              pcVar11 = pcVar13 + 0x1;
              cVar2 = *pcVar13;
              if cVar2 == '\\' {
                break;
              }
            } 
            if (cVar2 == '\"'){ break;}
            iVar9 += uVar7;
            pcVar11 = pcVar13;
          }
          iVar9 = iVar9 + (uVar7 >> 0x1) + ((uVar7 & 0x1) != 0x0);
          if (uVar7 & 0x1) == 0 {
            break;
          }
        } 
        // TODO: goto LAB_1000_267f;
      }
      if (cVar2 != '\\') {
        iVar9 += 0x1;
        // TODO: goto LAB_1000_267f;
      }
      uVar7 = 0x0;
      loop {
        uVar7 += 0x1;
        pcVar1 = pcVar11;
        pcVar11 = pcVar11 + 0x1;
        cVar2 = *pcVar1;
        if cVar2 == '\\' {
          break;
        }
      } 
      if (cVar2 == '\"') {
        iVar9 = iVar9 + (uVar7 >> 0x1) + ((uVar7 & 0x1) != 0x0);
        if ((uVar7 & 0x1) == 0x0) {
          // TODO: goto LAB_1000_26b8;
        }
        // TODO: goto LAB_1000_267f;
      }
      iVar9 += uVar7;
    } 
  }
//LAB_1000_26e8:
  pCStack2 = ctx.data_seg;
  iVar9 = -((ctx.PTR_LOOP_1050_5fb8 +
                  (ctx.PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar9 + 0x1) & 0xfffe);
  ctx.PTR_LOOP_1050_5fba = (&param_1 + iVar9);
  pcVar13 = (&param_1 + (ctx.PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar9);
  ctx.PTR_LOOP_1050_5fbc = param_3;
  (&pCStack2 + iVar9) = param_3;
  puVar4 = ctx.PTR_LOOP_1050_5fc4;
  uVar14 = (&pCStack2 + iVar9);
  (&param_1 + iVar9) = ctx.PTR_LOOP_1050_5fc2;
  (&param_2 + iVar9) = puVar4;
  ppcVar10 = (&stack0x0004 + iVar9);
  (&pCStack2 + iVar9) = (&param_1 + iVar9);
  (&puStack4 + iVar9) = ctx.s_tile2_bmp_1050_1538;
  (&stack0xfffa + iVar9) = 0x271f;
  uVar6 = pass1_1000_29dc(param_3);
  uVar3 = &ctx.PTR_LOOP_1050_5f7e;
  pcVar11 = (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
//LAB_1000_272e:
  loop {
    loop {
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
      if cVar2 != ' ' {
        break;
      }
    } 
    if cVar2 == '\t' {
      break;
    }
  } 
  if ((cVar2 == '\r') || (cVar2 == '\0')) {
//LAB_1000_27c1:
    (&pCStack2 + iVar9) = ctx.s_tile2_bmp_1050_1538;
    (&puStack4 + iVar9) = 0x27c5;
    uVar6 = pass1_1000_29dc(param_3);
    *ppcVar10 = 0x0;
    ppcVar10[0x1] = 0x0;
                    // WARNING: Could not recover jumptable at 0x100027d2. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    (*&ctx.PTR_LOOP_1050_5fd2)();
    ctx._PTR_LOOP_1050_5fc2 = CONCAT22(ctx.PTR_LOOP_1050_5fc4,PTR_LOOP_1050_5fc2);
    return;
  }
  *ppcVar10 = pcVar13;
  ppcVar10[0x1] = param_3;
  ppcVar10 = ppcVar10 + 0x2;
  loop {
    pcVar11 = pcVar11 + -0x1;
//LAB_1000_274f:
    pcVar1 = pcVar11;
    pcVar11 = pcVar11 + 0x1;
    cVar2 = *pcVar1;
    if ((cVar2 == ' ') || (cVar2 == '\t')) {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\0';
//       TODO: goto LAB_1000_272e;
    }
    if ((cVar2 == '\r') || (cVar2 == '\0')) {
//LAB_1000_27be:
      *pcVar13 = '\0';
//       TODO: goto LAB_1000_27c1;
    }
    pcVar12 = pcVar11;
    if (cVar2 == '\"') {
//LAB_1000_278b:
      loop {
        pcVar11 = pcVar12 + 0x1;
        cVar2 = *pcVar12;
        if ((cVar2 == '\r') || (cVar2 == '\0')) {
          // goto LAB_1000_27be;
        }
        if (cVar2 == '\"') {
          break;
        }
        if (cVar2 == '\\') {
          uVar7 = 0x0;
          loop {
            pcVar12 = pcVar11;
            uVar7 += 0x1;
            pcVar11 = pcVar12 + 0x1;
            cVar2 = *pcVar12;
            if cVar2 != '\\' {
              break;
            }
          } 
          if (cVar2 == '\"') {
            // TODO: refactor for loop
            // for (uVar8 = uVar7 >> 0x1; uVar8 != 0x0; uVar8 -= 0x1) {
            //   pcVar1 = pcVar13;
            //   pcVar13 = pcVar13 + 0x1;
            //   *pcVar1 = '\\';
            // }
            if ((uVar7 & 0x1) == 0x0) {break;}
            pcVar1 = pcVar13;
            pcVar13 = pcVar13 + 0x1;
            *pcVar1 = '\"';
            pcVar12 = pcVar11;
          }
          else {
            // TODO: refactor for loop
            // for (; uVar7 != 0x0; uVar7 -= 0x1) {
            //   pcVar1 = pcVar13;
            //   pcVar13 = pcVar13 + 0x1;
            //   *pcVar1 = '\\';
            // }
          }
        }
        else {
          pcVar1 = pcVar13;
          pcVar13 = pcVar13 + 0x1;
          *pcVar1 = cVar2;
          pcVar12 = pcVar11;
        }
      }
//       TODO: goto LAB_1000_274f;
    }
    if (cVar2 != '\\') {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = cVar2;
//       TODO: goto LAB_1000_274f;
    }
    uVar7 = 0x0;
    loop {
      uVar7 += 0x1;
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
      if cVar2 != '\\' {
        break;
      }
    } 
    if (cVar2 == '\"') {
      // TODO: refactor for loop
      // for (uVar8 = uVar7 >> 0x1; uVar8 != 0x0; uVar8 -= 0x1) {
      //   pcVar1 = pcVar13;
      //   pcVar13 = pcVar13 + 0x1;
      //   *pcVar1 = '\\';
      // }
      pcVar12 = pcVar11;
      if ((uVar7 & 0x1) == 0x0) {
        // goto LAB_1000_278b;
      }
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\"';
//       TODO: goto LAB_1000_274f;
    }
    // TODO: refactor for loop
    // for (; uVar7 != 0x0; uVar7 -= 0x1) {
    //   pcVar1 = pcVar13;
    //   pcVar13 = pcVar13 + 0x1;
    //   *pcVar1 = '\\';
    // }
  } 
}



pub unsafe fn pass1_1000_27d6(ctx: &mut AppContext, param_1: *mut u16)
{
  let piVar1: *mut i16;
  let mut pcVar2: String; 
  let puVar3: *mut u16;
  let piVar4: *mut i16;
  let cVar5: u8;
  let SVar6: SEGPTR;
  let puVar7: *mut u16;
  let ppuVar8: *mut *mut u16;
  let iVar9: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let iVar12: i16;
  let piVar13: *mut i16;
  let piVar14: *mut i16;
  let mut pcVar15: String; 
  let piVar16: *mut i16;
  let bVar17: bool;
  let puVar18: *mut u16;
  
  SVar6 = GetDOSEnvironment16();
  if (SVar6 != 0x0) {
    param_1 = 0x0;
  }
  iVar12 = 0x0;
  pcVar15 = 0x0;
  iVar9 = -0x1;
  if (param_1 != 0x0) {
    cVar5 = *0x0;
    while (cVar5 != '\0') {
      loop {
        if (iVar9 == 0x0) {break;}
        iVar9 += -0x1;
        pcVar2 = pcVar15;
        pcVar15 = pcVar15 + 0x1;
        if *pcVar2 == '\0' {
          break;
        }
      } 
      iVar12 += 0x1;
      pcVar2 = pcVar15;
      pcVar15 = pcVar15 + 0x1;
      cVar5 = *pcVar2;
    }
  }
  uVar10 = 0x9;
  puVar11 = param_1;
  puVar7 = pass1_1000_2950(0x9,param_1,param_1,ctx.s_tile2_bmp_1050_1538);
  puVar18 = puVar11;
  ppuVar8 = pass1_1000_2950(uVar10,puVar11,param_1,ctx.s_tile2_bmp_1050_1538);
  piVar13 = 0x0;
  ctx.PTR_LOOP_1050_5fbe = ppuVar8;
  ctx.PTR_LOOP_1050_5fc0 = puVar11;
  loop {
    if (iVar12 == 0x0) {
      *ppuVar8 = 0x0;
      ppuVar8[0x1] = 0x0;
      return;
    }
    bVar17 = *piVar13 == ctx.s__C_FILE_INFO__1050_5f5c._0_2_;
    if (bVar17) {
      piVar16 = ctx.s__C_FILE_INFO__1050_5f5c;
      iVar9 = 0x6;
      piVar14 = piVar13;
      loop {
        if (iVar9 == 0x0) {
          break;
        }
        iVar9 += -0x1;
        piVar4 = piVar16;
        piVar16 = piVar16 + 0x1;
        piVar1 = piVar14;
        piVar14 = piVar14 + 0x1;
        bVar17 = *piVar1 == *piVar4;
        if bVar17 == false {
          break;
        }
      } 
      if (!bVar17) {
        // goto LAB_1000_2867;
      }
    }
    else {
//LAB_1000_2867:
      *ppuVar8 = puVar7;
      ppuVar8[0x1] = puVar18;
      ppuVar8 = ppuVar8 + 0x2;
    }
    loop {
      piVar1 = piVar13;
      piVar13 = (piVar13 + 0x1);
      cVar5 = *piVar1;
      puVar3 = puVar7;
      puVar7 = (puVar7 + 0x1);
      *puVar3 = cVar5;
      if cVar5 == '\0' {
        break;
      }
    } 
    iVar12 += -0x1;
  } 
}



pub fn pass1_1000_2913(param_1: i16,param_2: u16,param_3: u16)
{
  let mut pcVar1: String; 
  let mut pcVar2: String; 
  let iVar3: i16;
  
  if (ctx.PTR_LOOP_1050_61ec != 0x0) {
    pcVar2 = poss_str_op_1000_28dc(ctx, param_1);
    if (pcVar2 != 0x0) {
      iVar3 = -0x1;
      loop {
        if (iVar3 == 0x0) {break;}
        iVar3 += -0x1;
        pcVar1 = pcVar2;
        pcVar2 = pcVar2 + 0x1;
        if *pcVar1 == '\0' {
          break;
        }
      } 
      pass1_1000_55b1(0x2944,param_2,param_3);
    }
  }
  return;
}



pub fn pass1_1000_2950(param_1: i16,param_2: u16,param_3: u16,param_4: u16) -> *mut u16

{
  let puVar1: *mut u16;
  let uVar2: u16;
  let mut pcVar3: String; 
  let puVar4: *mut u8;
  let mut str: String;
  let iVar5: i16;
  let puVar6: *mut u16;
  let in_AX: u16;
  let puVar7: *mut u16;
  let mut pcVar8: String; 
  let uVar9: u16;
  
  puVar4 = ctx.PTR_LOOP_1050_6066;
  ctx.PTR_LOOP_1050_6066 = &ctx.PTR_LOOP_1050_1000;
  uVar9 = param_3;
  puVar7 = (fn mem_1000_167a(in_AX,param_4,param_2) -> u16;
  ctx.PTR_LOOP_1050_6066 = puVar4;
  if ((param_2 | puVar7) != 0x0) {
    return puVar7;
  }
  iVar5 = param_1;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(param_1,param_3,param_4);
  str = poss_str_op_1000_28dc(ctx, iVar5);
  if (str != 0x0) {
    iVar5 = 0x9;
    if (*str == 'M') {
      iVar5 = 0xf;
    }
    str = str + iVar5;
    iVar5 = 0x22;
    pcVar8 = str;
    loop {
      if (iVar5 == 0x0) { break; }
      iVar5 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
          if *pcVar == '\r' {
              break;
          }
    }
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,str);
  FatalExit();
  puVar7 = &ctx.PTR_LOOP_1050_63fe;
  loop {
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    uVar2 = *puVar1;
    puVar6 = puVar7;
    if ((uVar2 == uVar9) ||
       (puVar6 = (uVar2 + 0x1), puVar6 == 0x0)) {
      return puVar6;
    }
    iVar5 = -0x1;
    loop {
      if (iVar5 == 0x0) { break; }
      iVar5 += -0x1;
      puVar1 = puVar7;
      puVar7 = (puVar7 + 0x1);
        if *puVar1 == '\0' {
            break;
        }
    }
  }
}



pub fn pass1_1000_29af(param_1: u16)
{
  pass1_1000_29b5(param_1 & 0xff);
  return;
}



pub fn pass1_1000_29b5(param_1: u16)
{
  let cVar1: u8;
  
  ctx.PTR_LOOP_1050_5f88._0_1_ = param_1;
  cVar1 = (param_1 >> 0x8);
  if (cVar1 != '\0') {
      // goto
      // LAB_1000_29d2;
  }
  if (ctx.PTR_LOOP_1050_5f88 < 0x22) {
    if (ctx.PTR_LOOP_1050_5f88 < 0x20) {
      if (0x13 < ctx.PTR_LOOP_1050_5f88) {
          // goto
          // LAB_1000_29cc;
      }
    }
    else {
      param_1 = 0x5;
    }
  }
  else {
//LAB_1000_29cc:
    param_1 = 0x13;
  }
  cVar1 = *((param_1 & 0xff) + 0x5fd6);
//LAB_1000_29d2:
  ctx.PTR_LOOP_1050_5f78 = cVar1;
  return;
}



pub fn pass1_1000_29dc(param_1: u16) -> u16

{
  if (___EXPORTEDSTUB != (code)0xb8) {
    return param_1;
  }
  return uRam100029ed;
}



pub fn pass1_1000_2a00(param_1: *mut u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8) -> u16

{
  let bVar1: bool;
  let piVar2: *mut i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let puStack20: *mut u8;
  let local_10: u8;
  let uStack15: u8;
  let local_e: [u8;8];
  let uStack6: u16;
  let local_4: u16;
  let iStack2: i16;
  
  iStack2 = param_2 + 0x1;
  local_4 = SUB42(ctx.data_seg,0x0);
  uVar5 = 0xffff;
  if (((param_1 + 0x5) & 0x40) != 0x0) {
    *(param_1 + 0x5) = 0x0;
    return 0xffff;
  }
  if (((param_1 + 0x5) & 0x83) == 0x0) {
      // goto
      // LAB_1000_2af2;
  }
  uVar5 = pass1_1000_2fa4(param_1,param_3,param_4,param_5,param_6);
  uStack6 = param_1[0x7a];
  pass1_1000_2cb0(param_1,param_4);
  if (ctx.DAT_1050_5f8a < (param_1 + 0xb)) {
    piVar2 = pass1_1000_55b1(0x2a63,param_3,param_4);
    if (piVar2 < 0x0) {
        // goto
        // LAB_1000_2a6a;
    }
//LAB_1000_2a82:
    bVar1 = false;
  }
  else {
    iVar3 = dos3_call_op_1000_35fe((param_1 + 0xb),&iStack2);
    if (-0x1 < iVar3) {
        // goto
        // LAB_1000_2a82;
    }
//LAB_1000_2a6a:
    bVar1 = true;
  }
  if (!bVar1) {
    if (uStack6 == 0x0) {
        // goto
        // LAB_1000_2af2;
    }
    unk_str_op_1000_3d3e(CONCAT22(param_5,&local_10),0x10505fea);
    puStack20 = local_e;
    if (local_10 == '\\') {
      puStack20 = &uStack15;
    }
    else {
      pass1_1000_3cea(CONCAT22(param_5,&local_10),0x10505fec);
    }
    pass1_1000_3e82(uStack6,CONCAT22(param_5,puStack20),0xa);
    uVar4 = dos3_call_1000_514e(&iStack2);
    if (uVar4 == 0x0) {
        // goto
        // LAB_1000_2af2;
    }
  }
  uVar5 = 0xffff;
//LAB_1000_2af2:
  *(param_1 + 0x5) = 0x0;
  return uVar5;
}



pub fn pass1_1000_2b02(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u8,
               param_6: u16,param_7: i16) -> *mut u16

{
  let puVar1: *mut u16;
  let iStack2: i16;
  
  iStack2 = param_7 + 0x1;
  puVar1 = pass1_1000_35aa();
  if ((param_6 | puVar1) == 0x0) {
    puVar1 = 0x0;
  }
  else {
    puVar1 = pass1_1000_2d34(param_1,param_2,CONCAT22(param_4,param_3),param_5,
                             puVar1,&iStack2);
  }
  return puVar1;
}



pub fn pass1_1000_2b3c(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               ,param_6: i16)

{
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  pass1_1000_2b02(param_1,param_2,param_3,param_4,0x0,param_5,&iStack2);
  return;
}



pub fn pass1_1000_2b5c(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               ,param_6: i16,param_7: u16,param_8: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  let in_AF: u8;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  uVar1 = pass1_1000_2e74(param_1,param_7);
  uVar2 = sys_1000_30b4(param_1,ctx.data_seg,
                        CONCAT22(param_4,param_3),&iStack2,param_1,param_5,
                        param_7,param_8);
  pass1_1000_2f00(uVar1,param_1,param_5,param_7,param_8,in_AF);
  return uVar2;
}



pub fn pass1_1000_2ba0(param_1: u16,param_2: u16,param_3: u16,param_4: u8)
{
  pass1_1000_3024(param_1,param_2,param_3,param_4);
  if (ctx.PTR_LOOP_1050_5fc9 != '\0') {
    pass1_1000_3f5c(&stack0xfffe,param_1,param_2,param_3,param_4);
  }
  return;
}



pub fn pass1_1000_2cb0(param_1: *mut u16,param_2: u16)
{
  let puVar1: *mut u16;
  let bVar2: u8;
  
  bVar2 = (param_1 + 0x5);
  if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
    pass1_1000_16ee(param_1[0x3],param_1[0x4],param_2);
    puVar1 = param_1 + 0x5;
    puVar1 = puVar1 & 0xf7;
    param_1[0x3] = 0x0;
    param_1[0x4] = 0x0;
    *param_1 = 0x0;
    param_1[0x1] = 0x0;
    param_1[0x2] = 0x0;
  }
  return;
}


pub fn pass1_1000_2d34(param_1: u16,param_2: u16,param_3: *mut u8,param_4: u8,param_5: *mut u16,
               param_6: i16) -> *mut u16

{
  let bVar1: u8;
  let bVar2: bool;
  let bVar3: bool;
  let uVar4: u16;
  let uStack14: u8;
  let bStack8: u8;
  let uStack6: u8;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  bStack8 = ctx.PTR_LOOP_1050_6062;
  bVar3 = false;
  bVar1 = *param_3;
  if (bVar1 == 0x77) {
    uVar4 = 0x301;
  }
  else {
    if (0x77 < bVar1) {
      return 0x0;
    }
    if (bVar1 != 0x61) {
      if (bVar1 != 0x72) {
        return 0x0;
      }
      uVar4 = 0x0;
      uStack6 = 0x1;
//       TODO: goto LAB_1000_2d6c;
    }
    uVar4 = 0x109;
  }
  uStack6 = 0x2;
//LAB_1000_2d6c:
  bVar2 = true;
//LAB_1000_2d71:
  param_3 = (param_3 & 0xffff0000 | (param_3 + 0x1));
  if ((*param_3 == 0x0) || (!bVar2)) {
    uVar4 = mixed_dos3_call_1000_370a(param_1,param_2,uVar4,param_4,0x1a4,&iStack2);
    if (uVar4 < 0x0) {
      return 0x0;
    }
    ctx.PTR_LOOP_1050_5fee = ctx.PTR_LOOP_1050_5fee + 0x1;
    *(param_5 + 0x5) = uStack6;
    param_5[0x1] = 0x0;
    *param_5 = 0x0;
    param_5[0x4] = 0x0;
    param_5[0x3] = 0x0;
    uStack14 = uVar4;
    *(param_5 + 0xb) = uStack14;
    (param_5 + 0x78) = bStack8;
    param_5[0x2] = 0x0;
    param_5[0x7a] = 0x0;
    return param_5;
  }
  bVar1 = *param_3;
  if (bVar1 == 0x74) {
    if ((uVar4 & 0xc000) == 0x0) {
      uVar4 |= 0x4000;
//       TODO: goto LAB_1000_2d71;
    }
  }
  else {
    if (0x74 < bVar1) {
        // goto
        // LAB_1000_2da4;
    }
    if (bVar1 == 0x2b) {
      if ((uVar4 & 0x2) != 0x0) {
          // goto
          // LAB_1000_2da4;
      }
      uVar4 = uVar4 & 0xfffe | 0x2;
      uStack6 = 0x80;
//       TODO: goto LAB_1000_2d71;
    }
    if (bVar1 == 0x62) {
      if ((uVar4 & 0xc000) == 0x0) {
        uVar4 |= 0x8000;
//         TODO: goto LAB_1000_2d71;
      }
    }
    else {
      if (bVar1 != 0x63) {
        if ((bVar1 != 0x6e) || (bVar3)) {
            // goto
            // LAB_1000_2da4;
        }
        bVar3 = true;
        bStack8 &= 0xbf;
//         TODO: goto LAB_1000_2d71;
      }
      if (!bVar3) {
        bVar3 = true;
        bStack8 |= 0x40;
//         TODO: goto LAB_1000_2d71;
      }
    }
  }
//LAB_1000_2da4:
  bVar2 = false;
//   TODO: goto LAB_1000_2d71;
}



pub fn pass1_1000_2e74(param_1: *mut u16,param_2: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  
  if (ctx.PTR_LOOP_1050_61ec != 0x0) {
    puVar5 = param_1 + 0x78;
    puVar4 = 0x5ff2;
    if ((param_1 == 0x621c) ||
       (puVar4 = &ctx.PTR_LOOP_1050_5ff6, param_1 == 0x6228)) {
      if ((((param_1 + 0x5) & 0xc) == 0x0) && ((puVar5 & 0x1) == 0x0)) {
        uVar2 = *puVar4;
        uVar3 = puVar4[0x1];
        if ((uVar2 | uVar3) == 0x0) {
          uVar2 = mem_1000_167a(0x200,param_2,uVar3);
          if (uVar3 == 0x0) {
            return 0x0;
          }
          *puVar4 = uVar2;
          puVar4[0x1] = uVar3;
        }
        param_1[0x3] = uVar2;
        param_1[0x4] = uVar3;
        *param_1 = uVar2;
        param_1[0x1] = uVar3;
        param_1[0x2] = 0x200;
        param_1[0x79] = 0x200;
        puVar1 = param_1 + 0x5;
        puVar1 = puVar1 | 0x2;
        puVar5 = 0x11;
        return 0x1;
      }
    }
    else {
      if (ctx.DAT_1050_5f8a <= (param_1 + 0xb)) {
        puVar1 = puVar5;
        puVar1 = puVar1 | 0x10;
      }
    }
  }
  return 0x0;
}



pub fn pass1_1000_2f00(param_1: i16,param_2: &mut i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8)

{
  if ((((param_2 + 0x78) & 0x10) != 0x0) &&
     ((((param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)) {
    pass1_1000_2fa4(param_2,param_3,param_4,param_5,param_6);
    if (param_1 != 0x0) {
      (param_2 + 0x78) = 0x0;
      param_2[0x79] = 0x0;
      *param_2 = 0x0;
      param_2[0x1] = 0x0;
      param_2[0x3] = 0x0;
      param_2[0x4] = 0x0;
    }
  }
  return;
}



pub fn pass1_1000_2f48(param_1: i32,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8) -> u16

{
  let uVar1: u16;
  let puVar2: *mut u8;
  let iStack2: i16;
  
  iStack2 = param_2 + 0x1;
  if (param_1 == 0x0) {
    uVar1 = pass1_1000_3038(0x0,param_3,param_4,param_5,param_6);
  }
  else {
    uVar1 = pass1_1000_2fa4(param_1,param_3,param_4,param_5,param_6);
    if (uVar1 == 0x0) {
      if (((param_1 + 0x78) & 0x40) != 0x0) {
        puVar2 = pass1_1000_400a((param_1 + 0xb),
                                 &iStack2);
        uVar1 = -(puVar2 != 0x0);
      }
    }
    else {
      uVar1 = 0xffff;
    }
  }
  return uVar1;
}



pub fn pass1_1000_2fa4(param_1: &mut i16,param_2: u16,param_3: u16,param_4: u16,param_5: u8) -> u16

{
  let piVar1: *mut i16;
  let bVar2: u8;
  let iVar3: i16;
  let puVar4: *mut u8;
  let puVar5: *mut u8;
  let uVar6: u16;
  
  uVar6 = 0x0;
  bVar2 = (param_1 + 0x5);
  if (((bVar2 & 0x3) == 0x2) &&
     (((bVar2 & 0x8) != 0x0 || (((param_1 + 0x78) & 0x1) != 0x0)))) {
    puVar4 = (*param_1 - param_1[0x3]);
    if (0x0 < puVar4) {
      puVar5 = mixed_dos3_call_1000_39f2
                         ((param_1 + 0xb),
                          CONCAT22(param_1[0x4],param_1[0x3]),puVar4,param_2,
                          param_3,param_4,param_5);
      if (puVar5 == puVar4) {
        if (((param_1 + 0x5) & 0x80) != 0x0) {
          piVar1 = param_1 + 0x5;
          piVar1 = piVar1 & 0xfd;
        }
      }
      else {
        piVar1 = param_1 + 0x5;
        piVar1 = piVar1 | 0x20;
        uVar6 = 0xffff;
      }
    }
  }
  iVar3 = param_1[0x4];
  *param_1 = param_1[0x3];
  param_1[0x1] = iVar3;
  param_1[0x2] = 0x0;
  return uVar6;
}



pub fn pass1_1000_3024(param_1: u16,param_2: u16,param_3: u16,param_4: u8)
{
  pass1_1000_3038(0x1,param_1,param_2,param_3,param_4);
  return;
}



i16 pass1_1000_3038(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u8
                   )

{
  let uVar1: u16;
  let puVar2: *mut u8;
  let iVar3: i16;
  let iStack4: i16;
  
  iVar3 = 0x0;
  iStack4 = 0x0;
// TODO: refactor for loop
//   for (puVar2 = &ctx.PTR_LOOP_1050_6210; puVar2 <= ctx.PTR_LOOP_1050_5ff0;
//       puVar2 = puVar2 + 0xc) {
//     if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
//       uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),&stack0xfffe,param_2,param_3,
//                               param_4,param_5);
//       if (uVar1 != 0xffff) {
//         iVar3 += 0x1;
//       }
//     }
//     else {
//       if ((param_1 == 0x0) &&
//          (((puVar2[0xa] & 0x2) != 0x0 &&
//           (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),&stack0xfffe,param_2,
//                                    param_3,param_4,param_5), uVar1 == 0xffff)))) {
//         iStack4 = -0x1;
//       }
//     }
//   }
  if (param_1 == 0x1) {
    iStack4 = iVar3;
  }
  return iStack4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack

u16 pass1_1000_30a4(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
                      param_6: u16,param_7: u16,param_8: u16,param_9: u16,
                      param_10: u8)

{
  let puVar1: *mut u16;
  let cVar2: u8;
  let mut pcVar3: String; 
  let bVar4: u8;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  puVar6 = (param_5 + (param_3 + param_6) + param_10);
  puVar1 = puVar6;
  *puVar1 = *puVar1 ^ puVar6;
  puVar1 = (puVar6 + param_3 + 0x31);
  *puVar1 = *puVar1 ^ param_4;
  puVar1 = (puVar6 + -0x3acf);
  *puVar1 = *puVar1 ^ param_3;
  puVar1 = puVar6 + -0x3794;
  *puVar1 = *puVar1 ^ param_2;
  (param_1 + -0x2) = param_4 + 0x1;
  (param_1 + -0x4) = ctx.data_seg;
  (param_1 + -0x6) = param_8;
  (param_1 + -0x8) = 0x30c5;
  exit_1000_25f2((param_1 + -0x8),(param_1 + -0x6),
                 (param_1 + -0x4),0x214,param_7,param_8,param_9);
  (param_1 + -0x6) = puVar6;
  (param_1 + -0x8) = param_6 ^ puVar6;
  (param_1 + -0xc) = 0x0;
  *(param_1 + -0x9) = 0x0;
  pcVar3 = (param_1 + 0x8);
  cVar2 = *pcVar3;
  (param_1 + 0x8) = pcVar3 + 0x1;
  *(param_1 + -0x6) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xc))) {
    if ((cVar2 - 0x20) < 0x59) {
      bVar4 = ((cVar2 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = ((bVar4 * '\b' + *(param_1 + -0x9)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x9) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = ((bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xc);
}




// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_3113(param_1: u16,param_2: u16) -> u16

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let bVar3: u8;
  let uVar4: u16;
  
  pass1_1000_3552(0x1,param_1,param_2);
  pcVar2 = (param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 - 0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 - 0xa))) {
    if ((cVar1 - 0x20) < 0x59) {
      bVar3 = ((cVar1 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = ((bVar3 * '\b' + *(param_1 - 0x7)) + 0x5ffe) >>
            0x4;
    (param_1 - 0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = ((bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 - 0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_311e(param_1: i16,param_2: u16) -> u16

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let bVar3: u8;
  let uVar4: u16;
  
  (param_1 + -0x12) = 0x0;
  (param_1 + -0xc) = 0x0;
  (param_1 + -0x14) = 0x0;
  (param_1 + -0x6) = 0x20;
  (param_1 + -0xe) = 0xffff;
  pcVar2 = (param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((cVar1 - 0x20) < 0x59) {
      bVar3 = ((cVar1 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = ((bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = ((bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_3134(param_1: i16,param_2: u16) -> u16

{
  let pbVar1: *mut u8;
  let cVar2: u8;
  let mut pcVar3: String; 
  let bVar4: u8;
  let uVar5: u16;
  
  cVar2 = *(param_1 + -0x4);
  if (cVar2 == '-') {
    pbVar1 = (param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x4;
  }
  else {
    if (cVar2 == '+') {
      pbVar1 = (param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x1;
    }
    else {
      if (cVar2 == ' ') {
        pbVar1 = (param_1 + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
      }
      else {
        if (cVar2 == '#') {
          pbVar1 = (param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x80;
        }
        else {
          pbVar1 = (param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = (param_1 + 0xa);
  cVar2 = *pcVar3;
  (param_1 + 0xa) = pcVar3 + 0x1;
  *(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((cVar2 - 0x20) < 0x59) {
      bVar4 = ((cVar2 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = ((bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = ((bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_3168(param_1: i16,param_2: u16) -> u16

{
  let pbVar1: *mut u8;
  let cVar2: u8;
  let mut pcVar3: String; 
  let bVar4: u8;
  let uVar5: u16;
  
  cVar2 = *(param_1 + -0x4);
  if (cVar2 == '*') {
    uVar5 = pass1_1000_34cf(param_1,param_2);
    if (uVar5 < 0x0) {
      uVar5 = -uVar5;
      pbVar1 = (param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x4;
    }
  }
  else {
    uVar5 = (param_1 + -0xc) * 0xa + (cVar2 - 0x30);
  }
  (param_1 + -0xc) = uVar5;
  pcVar3 = (param_1 + 0xa);
  cVar2 = *pcVar3;
  (param_1 + 0xa) = pcVar3 + 0x1;
  *(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((cVar2 - 0x20) < 0x59) {
      bVar4 = ((cVar2 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = ((bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = ((bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_3194(param_1: i16,param_2: u16) -> u16

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let bVar3: u8;
  let uVar4: u16;
  
  (param_1 + -0xe) = 0x0;
  pcVar2 = (param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((cVar1 - 0x20) < 0x59) {
      bVar3 = ((cVar1 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = ((bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = ((bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_319c(param_1: i16,param_2: u16) -> u16

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let bVar3: u8;
  let uVar4: u16;
  
  cVar1 = *(param_1 + -0x4);
  if (cVar1 == '*') {
    uVar4 = pass1_1000_34cf(param_1,param_2);
    if (uVar4 < 0x0) {
      uVar4 = 0xffff;
    }
  }
  else {
    uVar4 = (param_1 + -0xe) * 0xa + (cVar1 - 0x30);
  }
  (param_1 + -0xe) = uVar4;
  pcVar2 = (param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((cVar1 - 0x20) < 0x59) {
      bVar3 = ((cVar1 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = ((bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = ((bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_31c5(param_1: i16,param_2: u16) -> u16

{
  let pbVar1: *mut u8;
  let cVar2: u8;
  let mut pcVar3: String; 
  let bVar4: u8;
  let uVar5: u16;
  
  cVar2 = *(param_1 + -0x4);
  if (cVar2 == 'l') {
    pbVar1 = (param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x10;
  }
  else {
    if (cVar2 == 'F') {
      pbVar1 = (param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x20;
    }
    else {
      if (cVar2 == 'N') {
        pbVar1 = (param_1 + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
      }
      else {
        if (cVar2 == 'L') {
          pbVar1 = (param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x4;
        }
        else {
          pbVar1 = (param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = (param_1 + 0xa);
  cVar2 = *pcVar3;
  (param_1 + 0xa) = pcVar3 + 0x1;
  *(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((cVar2 - 0x20) < 0x59) {
      bVar4 = ((cVar2 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = ((bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = ((bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_31f7(param_1: u16,param_2: i16,param_3: *mut u16,param_4: i16,param_5: u16) -> u16

{
  let piVar1: *mut i16;
  let pbVar2: *mut u8;
  let puVar3: *mut u16;
  let cVar4: u8;
  let mut pcVar5: String; 
  let bVar6: u8;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let iVar10: i16;
  let puVar11: *mut u16;
  let uVar12: u16;
  let mut pcVar13: String; 
  let mut pcVar14: String; 
  let bVar15: bool;
  let uVar16: u32;
  
  cVar4 = *(param_2 + -0x4);
  if ((cVar4 == 'd') || (cVar4 == 'i')) {
    pbVar2 = (param_2 + -0x6);
    *pbVar2 = *pbVar2 | 0x40;
//LAB_1000_3399:
    *(param_2 + -0x8) = 0xa;
//LAB_1000_33d4:
    if (((param_2 + -0x6) & 0x10) == 0x0) {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      if (((param_2 + -0x6) & 0x40) == 0x0) {
        uVar16 = uVar7;
      }
      else {
        uVar16 = SEXT24(uVar7);
      }
    }
    else {
      uVar16 = pass1_1000_34d8(param_2,param_5);
    }
    if ((((param_2 + -0x6) & 0x40) != 0x0) && (uVar16 < 0x0)) {
      pbVar2 = (param_2 + -0x5);
      *pbVar2 = *pbVar2 | 0x1;
      uVar16 = CONCAT22(-((uVar16 >> 0x10) + (uVar16 != 0x0)),-uVar16
                       );
    }
    if ((param_2 + -0xe) < 0x0) {
      (param_2 + -0xe) = 0x1;
    }
    else {
      pbVar2 = (param_2 + -0x6);
      *pbVar2 = *pbVar2 & 0xf7;
    }
    if (uVar16 == 0x0) {
      (param_2 + -0x12) = 0x0;
    }
    puVar11 = (param_2 + -0x8);
    pass1_1000_356e(uVar16,puVar11,(uVar16 >> 0x10),param_2,
                    (param_2 + -0xe),(param_2 + -0x17),param_5,param_5);
    if ((((param_2 + -0x5) & 0x2) != 0x0) &&
       ((puVar11 == 0x0 || ((param_2 + -0x17) != 0x30)))) {
      *(param_2 + -0x18) = 0x30;
      puVar11 = (puVar11 + 0x1);
    }
  }
  else {
    if (cVar4 == 'u') {
        // goto
        // LAB_1000_3399;
    }
    if (cVar4 == 'X') {
      *(param_2 + -0x3) = 0x7;
//LAB_1000_33a9:
      if (((param_2 + -0x6) & 0x80) != 0x0) {
        (param_2 + -0x12) = 0x2;
        *(param_2 + -0x10) = 0x30;
        *(param_2 + -0xf) = *(param_2 + -0x3) + 'Q';
      }
      *(param_2 + -0x8) = 0x10;
//       TODO: goto LAB_1000_33d4;
    }
    if (cVar4 == 'x') {
      *(param_2 + -0x3) = 0x27;
//       TODO: goto LAB_1000_33a9;
    }
    if (cVar4 == 'o') {
      if (((param_2 + -0x6) & 0x80) != 0x0) {
        pbVar2 = (param_2 + -0x5);
        *pbVar2 = *pbVar2 | 0x2;
      }
      *(param_2 + -0x8) = 0x8;
//       TODO: goto LAB_1000_33d4;
    }
    if (cVar4 == 'c') {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      *(param_2 + -0x216) = uVar7;
      puVar11 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    }
    else {
      if (cVar4 == 's') {
        pass1_1000_34e6(param_1,param_2,param_5);
        if ((param_3 != 0x0) || (puVar11 = ctx.DAT_1050_605d, param_4 != 0x0)) {
          iVar10 = (param_2 + -0xe);
          puVar11 = param_3;
          if (iVar10 != 0x0) {
            bVar15 = true;
            loop {
              if (iVar10 == 0x0) { break; }
              iVar10 += -0x1;
              puVar3 = puVar11;
              puVar11 = (puVar11 + 0x1);
              bVar15 = *puVar3 == '\0';
                  if bVar15 { break; }
            }
            if (bVar15) {
              puVar11 = (puVar11 + -0x1);
            }
          }
          puVar11 = (puVar11 - param_3);
        }
      }
      else {
        if (cVar4 == 'n') {
          pass1_1000_34e6(param_1,param_2,param_5);
          *param_3 = (param_2 + -0xa);
          if (((param_2 + -0x6) & 0x10) != 0x0) {
            param_3[0x1] = 0x0;
          }
//           TODO: goto LAB_1000_30cf;
        }
        if (cVar4 == 'p') {
          if (((param_2 + -0x6) & 0x30) == 0x0) {
            uVar7 = pass1_1000_34cf(param_2,param_5);
            uVar16 = uVar7;
          }
          else {
            uVar16 = pass1_1000_34d8(param_2,param_5);
           // uVar12 = (uVar16 >> 0x10);
            if (((param_2 + -0x5) & 0x18) == 0x0) {
              *(param_2 + -0x3) = 0x7;
              pass1_1000_356e(uVar16,0x10,0x0,param_2,0x4,(param_2 + -0x20e)
                              ,param_5,param_5);
              pass1_1000_356e(uVar12,0x10,0x0,param_2,0x4,(param_2 + -0x213),
                              param_5,param_5);
              *(param_2 + -0x212) = 0x3a;
              puVar11 = &DAT_1050_0009;
//               TODO: goto LAB_1000_3444;
            }
          }
          *(param_2 + -0x3) = 0x7;
          pass1_1000_356e(uVar16,0x10,0x0,param_2,0x4,(param_2 + -0x213),
                          param_5,param_5);
          puVar11 = &DAT_1050_0004;
        }
        else {
          if ((cVar4 == 'E') || (cVar4 == 'G')) {
            piVar1 = (param_2 + -0x14);
            *piVar1 = *piVar1 + 0x1;
          }
          pbVar2 = (param_2 + -0x6);
          *pbVar2 = *pbVar2 | 0x40;
          bVar6 = (param_2 + -0x4) | 0x20;
          iVar10 = (param_2 + -0xe);
          if (iVar10 < 0x1) {
            if (iVar10 == 0x0) {
              if (bVar6 == 0x67) {
                (param_2 + -0xe) = 0x1;
              }
            }
            else {
              (param_2 + -0xe) = 0x6;
            }
          }
          pcVar13 = (param_2 + -0x216);
          if (((param_2 + -0x5) & 0x4) == 0x0) {
            (*PTR_s_3_wav_1050_25cc_1050_6068)();
            piVar1 = (param_2 + 0xe);
            *piVar1 = *piVar1 + 0x8;
          }
          else {
            (*PTR_s_3_wav_1050_25cc_1050_607c)();
            piVar1 = (param_2 + 0xe);
            *piVar1 = *piVar1 + 0xa;
          }
          if ((((param_2 + -0x6) & 0x80) != 0x0) &&
             ((param_2 + -0xe) == 0x0)) {
            (*PTR_s_3_wav_1050_25cc_1050_6074)();
          }
          if ((bVar6 == 0x67) && (((param_2 + -0x6) & 0x80) == 0x0)) {
            (*PTR_s_3_wav_1050_25cc_1050_6070)();
          }
          if (*pcVar13 == '-') {
            pcVar13 = (param_2 + -0x215);
            pbVar2 = (param_2 + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
          }
          iVar10 = -0x1;
          pcVar14 = pcVar13;
          loop {
            if (iVar10 == 0x0) { break; }
            iVar10 += -0x1;
            pcVar5 = pcVar14;
            pcVar14 = pcVar14 + 0x1;
                if *pcVar5 == '\0' { break; }
          }
          puVar11 = (pcVar14 + (-0x1 - pcVar13));
        }
      }
    }
  }
//LAB_1000_3444:
  if (((param_2 + -0x6) & 0x40) != 0x0) {
    if (((param_2 + -0x5) & 0x1) == 0x0) {
      if (((param_2 + -0x6) & 0x1) == 0x0) {
        if (((param_2 + -0x6) & 0x2) != 0x0) {
          *(param_2 + -0x10) = 0x20;
          (param_2 + -0x12) = 0x1;
        }
      }
      else {
        *(param_2 + -0x10) = 0x2b;
        (param_2 + -0x12) = 0x1;
      }
    }
    else {
      *(param_2 + -0x10) = 0x2d;
      (param_2 + -0x12) = 0x1;
    }
  }
  iVar8 = (param_2 + -0xc) - puVar11;
  iVar10 = (param_2 + -0x12);
  iVar9 = iVar8 - iVar10;
  if (iVar8 < iVar10) {
    iVar9 = 0x0;
  }
  if (((param_2 + -0x6) & 0xc) == 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534((param_2 + -0x12),param_2,param_5);
  if ((((param_2 + -0x6) & 0x8) != 0x0) &&
     (((param_2 + -0x6) & 0x4) == 0x0)) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534(puVar11,param_2,param_5);
  if (((param_2 + -0x6) & 0x4) != 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
//LAB_1000_30cf:
  pcVar5 = (param_2 + 0xa);
  cVar4 = *pcVar5;
  (param_2 + 0xa) = pcVar5 + 0x1;
  *(param_2 + -0x4) = cVar4;
  if ((cVar4 != '\0') && (-0x1 < (param_2 + -0xa))) {
    if ((cVar4 - 0x20) < 0x59) {
      bVar6 = ((cVar4 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar6 = 0x0;
    }
    bVar6 = ((bVar6 * '\b' + *(param_2 + -0x7)) + 0x5ffe) >>
            0x4;
    (param_2 + -0x7) = bVar6;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar7 = ((bVar6 * 0x2 + 0x30a4))();
    return uVar7;
  }
  return (param_2 + -0xa);
}



pub fn pass1_1000_34cf(param_1: i16,param_2: u16) -> u16

{
  let uVar1: u16;
  let puVar2: *mut u16;
  
  puVar2 = (param_1 + 0xe);
  uVar1 = *puVar2;
  (param_1 + 0xe) = puVar2 + 0x2;
  return uVar1;
}



pub fn pass1_1000_34d8(param_1: i16,param_2: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  puVar3 = (param_1 + 0xe);
  uVar1 = *puVar3;
  uVar2 = (puVar3 + 0x2);
  (param_1 + 0xe) = puVar3 + 0x4;
  return CONCAT22(uVar2,uVar1);
}



pub fn pass1_1000_34e6(param_1: u16,param_2: i16,param_3: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u32;
  
  if (((param_2 + -0x6) & 0x20) != 0x0) {
    uVar2 = pass1_1000_34d8(param_2,param_3);
    return uVar2;
  }
  uVar1 = pass1_1000_34cf(param_2,param_3);
  if (uVar1 == 0x0) {
    return param_1 << 0x10;
  }
  return CONCAT22(param_1,uVar1);
}



i16 
pass1_1000_3503(param_1: u8,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
               param_6: u8)

{
  let piVar1: *mut i16;
  let mut pcVar2: String; 
  char **ppcVar3;
  let uVar4: u16;
  let piVar5: *mut i16;
  let uVar6: u16;
  
  ppcVar3 = (param_3 + 0x6);
 // uVar6 = (ppcVar3 >> 0x10);
  piVar5 = ppcVar3;
  piVar1 = piVar5 + 0x2;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    uVar4 = mem_1000_2bb6(param_1,piVar5,param_3,uVar6,param_4,param_5,param_6,
                          param_2);
    if (uVar4 == 0xffff) {
      return -0x1;
    }
  }
  else {
    pcVar2 = *ppcVar3;
    *ppcVar3 = *ppcVar3 + 0x1;
    *pcVar2 = param_1;
  }
  return 0x0;
}



pub fn pass1_1000_3534(param_1: i16,param_2: i16,param_3: u16)
{
  let piVar1: *mut i16;
  let puVar2: *mut u8;
  let uVar3: u16;
  let in_DX: u16;
  let unaff_DI: *mut u8;
  let uVar4: u16;
  let unaff_ES: u16;
  let unaff_CS: u16;
  let in_AF: u8;
  
  if (param_1 != 0x0) {
    piVar1 = (param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar4 = 0x0;
    loop {
      puVar2 = unaff_DI;
      unaff_DI = unaff_DI + 0x1;
      uVar3 = pass1_1000_3503(*puVar2,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar4 |= uVar3;
      param_1 += -0x1;
          if param_1 == 0 { break; }
    }
    if (uVar4 != 0x0) {
      (param_2 + -0xa) = 0xffff;
    }
  }
  return;
}



pub fn pass1_1000_3552(param_1: i16,param_2: i16,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let in_DX: u16;
  let uVar3: u16;
  let unaff_CS: u16;
  let in_AF: u8;
  
  if (param_1 != 0x0) {
    piVar1 = (param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar3 = 0x0;
    loop {
      uVar2 = pass1_1000_3503(in_DX,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar3 |= uVar2;
      param_1 += -0x1;
          if pram_1 == 0 { break; }
    }
    if (uVar3 != 0x0) {
      (param_2 + -0xa) = 0xffff;
    }
  }
  return;
}



pub fn
pass1_1000_356e(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: i16,
               param_6: *mut u8,param_7: u16,param_8: u16)

{
  let pbVar1: *mut u8;
  let uVar2: u32;
  let bVar3: u8;
  
  while (((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0))) {
    uVar2 = param_3;
    param_3 /= param_2;
    uVar2 = uVar2 % param_2 << 0x10 | param_1;
    param_1 = (uVar2 / param_2);
    bVar3 = (uVar2 % param_2) + 0x30;
    if (0x39 < bVar3) {
      bVar3 += *(param_4 + -0x3);
    }
    pbVar1 = param_6;
    param_6 = param_6 + -0x1;
    *pbVar1 = bVar3;
    param_5 += -0x1;
  }
  return;
}



pub fn pass1_1000_35aa() -> u16

{
  let puVar1: *mut u16;
  
  puVar1 = &ctx.PTR_LOOP_1050_6210;
  loop {
    if (ctx.PTR_LOOP_1050_5ff0 < puVar1) {
      return 0x0;
    }
    if (((puVar1 + 0x5) & 0x83) == 0x0) { break; }
    puVar1 = puVar1 + 0x6;
  }
  *(puVar1 + 0x5) = 0x0;
  puVar1[0x2] = 0x0;
  puVar1[0x4] = 0x0;
  puVar1[0x3] = 0x0;
  puVar1[0x1] = 0x0;
  *puVar1 = 0x0;
  *(puVar1 + 0xb) = 0xff;
  return puVar1;
}




pub fn pass1_1000_39e1()
{
  return;
}


pub fn  pass1_1000_3bac() -> i16

{
  let iVar1: i16;
  
  if (ctx.PTR_LOOP_1050_5f48 < &stack0x0004) {
    iVar1 = -(ctx.PTR_LOOP_1050_5f48 + -&stack0x0004);
  }
  else {
    iVar1 = 0x0;
  }
  return iVar1;
}



pub fn
pass1_1000_3bc0(param_1: i16,param_2: i16,param_3: *mut u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let puVar6: *mut u16;
  let bVar7: bool;
  let uVar8: u32;
  
  if (((param_2 + 0x2) & 0x1) != 0x0) {
    pass1_1000_3cb7(param_2);
    uVar4 = *param_3;
    if ((uVar4 & 0x1) != 0x0) {
      param_1 = (param_1 - uVar4) + -0x1;
    }
    uVar4 = (param_2 + 0x4);
    if (uVar4 != 0x0) {
      uVar3 = param_1 + 0x2 + uVar4;
      if (!CARRY2(param_1 + 0x2,uVar4)) {
        param_4 = pass1_1000_29dc(param_6);
        uVar4 = &ctx.PTR_LOOP_1050_6066;
        if (uVar4 == 0x1000) {
            // goto
            // LAB_1000_3c12;
        }
        uVar2 = 0x8000;
        while (uVar4 <= uVar2) {
          uVar2 >>= 0x1;
          if (uVar2 == 0x0) {
              // goto
              // LAB_1000_3c2b;
          }
        }
        if (uVar2 < 0x8) {
            // goto
            // LAB_1000_3c2b;
        }
        uVar4 = uVar2 << 0x1;
//         TODO: goto LAB_1000_3c12;
      }
      uVar2 = 0x0;
      uVar4 = 0xfff0;
      if (uVar3 == 0x0) {
        loop {
          bVar7 = false;
          uVar8 = mixed_mem_op_1000_3c51(uVar2,uVar3,param_2,param_4,param_5,0x3c23);
          if (!bVar7) { break; }
          if (uVar4 == 0xfff0) {
            return;
          }
//LAB_1000_3c2b:
          uVar4 = 0x10;
//LAB_1000_3c12:
          uVar4 -= 0x1;
          uVar2 = uVar4 + uVar3;
          if (CARRY2(uVar4,uVar3)) {
            uVar2 = 0x0;
          }
          uVar4 = ~uVar4;
          uVar2 &= uVar4;
        }
        iVar5 = uVar8 - (param_2 + 0x4);
        (param_2 + 0x4) = uVar8;
        (param_2 + 0xa) = param_3;
        piVar1 = (param_2 + 0xc);
        *piVar1 = iVar5 + -0x1;
        puVar6 = (piVar1 + iVar5);
        *puVar6 = 0xfffe;
        (param_2 + 0xc) = puVar6;
      }
    }
  }
  return;
}


pub fn pass1_1000_3cb7(param_1: i16)
{
  let uVar1: u16;
  let puVar2: *mut u16;
  
  puVar2 = (param_1 + 0xa);
  if (puVar2 == (param_1 + 0xc)) {
    puVar2 = (param_1 + 0x8);
  }
  loop {
    uVar1 = *puVar2;
    if (uVar1 == 0xfffe) { break; }
    puVar2 = (puVar2 + (uVar1 & 0xfffe) + 0x2);
  }
  return;
}



pub fn pass1_1000_3cd8(param_1: u16,param_2: u16)
{
  free_mem_1000_407a(param_1,param_2,&stack0xfffe);
  return;
}



pub fn pass1_1000_3cea(param_1: i32,param_2: i32) -> u16

{
  let pUVar1: *mut u16;
  let mut pcVar2: String; 
  let pUVar3: *mut u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let pUVar7: *mut u16;
  let mut pcVar8: String; 
  let pUVar9: *mut u16;
  let pUVar10: *mut u16;
  let uVar11: u16;
  let uVar12: u16;
  let bVar13: bool;
  
 // uVar11 = (param_1 >> 0x10);
  bVar13 = true;
  iVar4 = -0x1;
  pUVar7 = param_1;
  loop {
    if (iVar4 == 0x0) { break; }
    iVar4 += -0x1;
    pUVar1 = pUVar7;
    pUVar7 = (pUVar7 + 0x1);
    bVar13 = *pUVar1 == '\0';
    if bVar13 {break;}
  }
  pUVar10 = (pUVar7 + -0x1);
 // uVar12 = (param_2 >> 0x10);
  pcVar8 = param_2;
  uVar5 = 0xffff;
  loop {
    if (uVar5 == 0x0) { break; }
    uVar5 -= 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
    bVar13 = *pcVar2 == '\0';
      if bVar13 { break; }
  }
  uVar5 = ~uVar5;
  if (!bVar13) {
    pcVar8 = pcVar8 + -uVar5;
    uVar5 += 0x1;
  }
  pUVar9 = (pcVar8 + -uVar5);
  if (uVar5 == 0x0) {
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar10 = *pUVar1;
    uVar5 = 0xfffe;
    pUVar10 = (pUVar7 + 0x1);
  }
  else {
    if ((pUVar9 & 0x1) != 0x0) {
      pUVar1 = pUVar9;
      pUVar9 = (pUVar9 + 0x1);
      *pUVar10 = *pUVar1;
      uVar5 -= 0x1;
      pUVar10 = pUVar7;
    }
  }
    // TODO: refactor for loop
  // for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
  //   pUVar3 = pUVar10;
  //   pUVar10 = pUVar10 + 0x1;
  //   pUVar1 = pUVar9;
  //   pUVar9 = pUVar9 + 0x1;
  //   *pUVar3 = *pUVar1;
  // }
    // TODO: refactor for loop
  // for (uVar5 = ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
  //   pUVar3 = pUVar10;
  //   pUVar10 = (pUVar10 + 0x1);
  //   pUVar1 = pUVar9;
  //   pUVar9 = (pUVar9 + 0x1);
  //   *pUVar3 = *pUVar1;
  // }
  return param_1;
}



i16  pass1_1000_3d7a(param_1: u32,param_2: u32)

{
  let pbVar1: *mut u8;
  let mut pcVar2: String; 
  let pbVar3: *mut u8;
  let iVar4: i16;
  let uVar5: u16;
  let mut pcVar6: String; 
  let pbVar7: *mut u8;
  let mut pcVar8: String; 
  let pbVar9: *mut u8;
  let uVar10: u16;
  let bVar11: bool;
  let bVar12: bool;
  
  pbVar7 = param_1;
 // uVar10 = (param_2 >> 0x10);
  pcVar8 = param_2;
  iVar4 = 0x0;
  uVar5 = 0xffff;
  loop {
    if (uVar5 == 0x0) { break; }
    uVar5 -= 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
if *pcVar2 == '\0' { break; }
  }
  pcVar6 = ~uVar5;
  bVar11 = pcVar8 < pcVar6;
  pbVar9 = (pcVar8 + -pcVar6);
  bVar12 = pbVar9 == 0x0;
  loop {
    if (pcVar6 == 0x0) { break; }
    pcVar6 = pcVar6 + -0x1;
    pbVar3 = pbVar9;
    pbVar9 = pbVar9 + 0x1;
    pbVar1 = pbVar7;
    pbVar7 = pbVar7 + 0x1;
    bVar11 = *pbVar1 < *pbVar3;
    bVar12 = *pbVar1 == *pbVar3;
if !bVar12 { breka; }
  }
  if (!bVar12) {
    iVar4 = (0x1 - bVar11) - (bVar11 != 0x0);
  }
  return iVar4;
}



pub fn
pass1_1000_3de8(param_1: &mut String,param_2: &mut String,param_3: u16,param_4: u16,param_5: u16
               ) -> u16

{
  let pbVar1: *mut u8;
  let mut pcVar2: String; 
  let mut pcVar3: String; 
  let bVar4: u8;
  let uVar5: u16;
  let iVar6: i16;
  let mut pcVar7: String; 
  let mut pcVar8: String; 
  let uVar9: u16;
  let uVar10: u16;
  let bVar11: bool;
  
  if (param_3 != 0x0) {
   // uVar9 = (param_1 >> 0x10);
    pcVar8 = param_1;
    uVar5 = param_3;
    pcVar7 = pcVar8;
    loop {
      if (uVar5 == 0x0) { break; }
      uVar5 -= 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
          if *pcVar2 == '\0' { break; }
    }
    iVar6 = param_3 - uVar5;
   // uVar10 = (param_2 >> 0x10);
    pcVar7 = param_2;
    loop {
      if (iVar6 == 0x0) { break; }
      iVar6 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
          if *pcVar2 != *pcVar3 { break; }
    }
    bVar4 = pcVar7[-0x1];
    uVar5 = 0x0;
    pbVar1 = (pcVar8 + -0x1);
    bVar11 = bVar4 == *pbVar1;
    if (bVar4 < *pbVar1 || bVar11) {
      if (bVar11) {
        return 0x0;
      }
      uVar5 = 0xfffe;
    }
    param_3 = ~uVar5;
  }
  return param_3;
}



pub fn  pass1_1000_3e2c(param_1: u32) -> i16

{
  let pbVar1: *mut u8;
  let bVar2: u8;
  let bVar3: u8;
  let iVar4: i16;
  let pbVar5: *mut u8;
  let uVar6: u16;
  
 // uVar6 = (param_1 >> 0x10);
  pbVar5 = param_1;
  iVar4 = 0x0;
  loop {
    loop {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
      if bVar2 != 0x20 {
        break;
      }
    } 
    if bVar2 != 0x9 {
      break;
    }
  } 
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
    // goto LAB_1000_3e4d;
  }
  loop {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
//LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) { break; }
    iVar4 = iVar4 * 0xa + (bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



pub fn  pass1_1000_3e2c(param_1: u32) -> i16

{
  let pbVar1: *mut u8;
  let bVar2: u8;
  let bVar3: u8;
  let iVar4: i16;
  let pbVar5: *mut u8;
  let uVar6: u16;
  
 // uVar6 = (param_1 >> 0x10);
  pbVar5 = param_1;
  iVar4 = 0x0;
  loop {
    loop {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
      if bVar2 != 20 {
        break;
      }
    } 
    if bVar2 != 0x9 {
      break;
    }
  } 
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
    // goto LAB_1000_3e4d;
  }
  loop {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
//LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) { break; }
    iVar4 = iVar4 * 0xa + (bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



pub fn  pass1_1000_3e2c(param_1: u32) -> i16

{
  let pbVar1: *mut u8;
  let bVar2: u8;
  let bVar3: u8;
  let iVar4: i16;
  let pbVar5: *mut u8;
  let uVar6: u16;
  
 // uVar6 = (param_1 >> 0x10);
  pbVar5 = param_1;
  iVar4 = 0x0;
  loop {
    loop {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
      if bVar2 != 0x20 {
        break;
      }
    } 
    if bVar2 != 0x9 {
      break;
    }
  } 
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
    // goto LAB_1000_3e4d;
  }
  loop {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
//LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) { break; }
    iVar4 = iVar4 * 0xa + (bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



byte *  pass1_1000_3e82(param_1: u16,param_2: *mut u8,param_3: u16)

{
  let pbVar1: *mut u8;
  let uVar2: u32;
  let bVar3: u8;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let pbVar8: *mut u8;
  let pbVar9: *mut u8;
  let pbVar10: *mut u8;
  let pbVar11: *mut u8;
  let uVar12: u16;
  let bVar13: bool;
  let cVar4: u8;
  
  uVar6 = 0x0;
  if (param_3 == 0xa) {
    uVar6 = param_1 >> 0xf;
  }
 // uVar12 = (param_2 >> 0x10);
  pbVar9 = param_2;
  pbVar10 = pbVar9;
  pbVar8 = pbVar9;
  if (((true) && (param_3 == 0xa)) && (uVar6 < 0x0)) {
    pbVar10 = pbVar9 + 0x1;
    *param_2 = '-';
    bVar13 = param_1 != 0x0;
    param_1 = -param_1;
    uVar6 = -(uVar6 + bVar13);
    pbVar8 = pbVar10;
  }
  loop {
    uVar7 = 0x0;
    uVar5 = uVar6;
    if (uVar6 != 0x0) {
      uVar5 = uVar6 / param_3;
      uVar7 = uVar6 % param_3;
    }
    uVar2 = CONCAT22(uVar7,param_1);
    param_1 = (uVar2 / param_3);
    cVar4 = (uVar2 % param_3);
    bVar3 = cVar4 + 0x30;
    if (0x39 < bVar3) {
      bVar3 = cVar4 + 0x57;
    }
    pbVar11 = pbVar10 + 0x1;
    *pbVar10 = bVar3;
    uVar6 = uVar5;
    pbVar10 = pbVar11;
    if (uVar5 | param_1) == 0 {
      break;
    }
  } 
  *pbVar11 = 0x0;
  loop {
    pbVar11 = pbVar11 + -0x1;
    pbVar1 = pbVar11;
    bVar3 = *pbVar1;
    *pbVar1 = *pbVar8;
    *pbVar8 = bVar3;
    pbVar10 = pbVar8 + 0x2;
    pbVar8 = pbVar8 + 0x1;
    if pbVar10 >= pbVar11 {
      break;
    }
  } 
  return pbVar9;
}


pub fn pass1_1000_3ec0(param_1: u16,param_2: u16) -> i16

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let unaff_SI: u16;
  let uVar4: u16;
  let puVar4: u32;
  
  puVar4 = CONCAT22(ctx.PTR_LOOP_1050_5fc0,PTR_LOOP_1050_5fbe);
  if (((ctx.PTR_LOOP_1050_5fc0 | ctx.PTR_LOOP_1050_5fbe) != 0x0) &&
     ((param_2 | param_1) != 0x0)) {
    uVar1 = str_op_1000_3da4(CONCAT22(param_2,param_1));
    loop {
     // uVar4 = (puVar4 >> 0x10);
      uVar3 = puVar4;
      if (((uVar3 + 0x2) | puVar4) == 0x0) { break; }
      uVar2 = str_op_1000_3da4(CONCAT22((uVar3 + 0x2),
                                                puVar4));
      if (((uVar1 < uVar2) && (*(*puVar4 + uVar1) == '=')) &&
         (uVar2 = pass1_1000_3de8(CONCAT22((uVar3 + 0x2),
                                                   puVar4),
                                  CONCAT22(param_2,param_1),uVar1,unaff_SI,uVar3),
         uVar2 == 0x0)) {
        return puVar4 + uVar1 + 0x1;
      }
      puVar4 = (puVar4 & 0xffff0000 | (uVar3 + 0x4));
    }
  }
  return 0x0;
}



pub fn pass1_1000_3f5c(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u8) -> i16

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let iVar3: i16;
  let iStack2: i16;
  
  iStack2 = param_1 + 0x1;
  iVar3 = 0x0;
  if (ctx.PTR_LOOP_1050_61ec == 0x0) {
    puVar2 = &ctx.PTR_LOOP_1050_6210;
  }
  else {
    puVar2 = 0x6234;
  }
    // TODO: refactor for loop
  // for (; puVar2 <= ctx.PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
  //   uVar1 = pass1_1000_2a00(puVar2,&iStack2,param_2,param_3,param_4,param_5);
  //   if (uVar1 != 0xffff) {
  //     iVar3 += 0x1;
  //   }
  // }
  return iVar3;
}



pub fn pass1_1000_400a(param_1: i16,param_2: u16) -> *mut u8

{
  let puVar1: *mut u8;
  let iStack2: i16;
  
  iStack2 = param_2 + 0x1;
  if (param_1 < 0x0) || (ctx.PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1) {
    ctx.PTR_LOOP_1050_5f78 = &DAT_1050_0009;
    puVar1 = 0xffff;
  }
  else {
    if (((ctx.PTR_LOOP_1050_61ec == 0x0) ||
        ((param_1 < ctx.DAT_1050_5f8a && (0x2 < param_1)))) &&
       (0x31d < CONCAT11(ctx.DAT_1050_5f83,DAT_1050_5f82))) {
      puVar1 = ctx.PTR_LOOP_1050_5f88;
      if ((((param_1 + 0x5f90) & 0x1) == 0x0) ||
         (puVar1 = dos3_call_1000_5174(&iStack2), puVar1 != 0x0)
         ) {
        ctx.PTR_LOOP_1050_5f88 = puVar1;
        ctx.PTR_LOOP_1050_5f78 = &DAT_1050_0009;
        puVar1 = 0xffff;
      }
    }
    else {
      puVar1 = 0x0;
    }
  }
  return puVar1;
}


pub fn pass1_1000_41e0(param_1: i16) -> u16

{
  let piStack6: *mut i16;
  
  piStack6 = CONCAT22(ctx.PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  loop {
    if (ctx.PTR_LOOP_1050_6190 + (ctx.PTR_LOOP_1050_6194 & 0xfffc) <= piStack6)
    {
      return 0x0;
    }
    if (*piStack6 == param_1) { break; }
    piStack6 = (piStack6 & 0xffff0000 | ZEXT24(piStack6 + 0x4))
    ;
  }
  *piStack6 = 0x0;
  return (piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1000_422a(param_1: i16,param_2: u16,param_3: u16,param_4: u16) -> i16

{
  let puVar1: *mut u8;
  let puVar2: *mut u8;
  let puVar3: *mut u8;
  let puVar4: *mut u8;
  let piStack6: *mut i16;
  
  piStack6 = CONCAT22(ctx.PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  loop {
    if (ctx.PTR_LOOP_1050_6190 + (ctx.PTR_LOOP_1050_6194 & 0xfffc) <= piStack6)
    {
      puVar2 = ctx.PTR_LOOP_1050_6194 + 0x28;
      puVar4 = ctx.PTR_LOOP_1050_6192;
      puVar3 = 
               pass1_1000_16aa(PTR_LOOP_1050_6190,PTR_LOOP_1050_6192,
                               puVar2,PTR_LOOP_1050_6192,param_3,param_4);
      if ((puVar4 | puVar3) == 0x0) {
        param_1 = 0x0;
      }
      else {
        puVar1 = puVar3 + (ctx.PTR_LOOP_1050_6194 & 0xfffc);
        piStack6 = CONCAT22(puVar4,puVar1);
        ctx.PTR_LOOP_1050_6190 = puVar3;
        ctx.PTR_LOOP_1050_6192 = puVar4;
        *piStack6 = param_1;
        (puVar1 + 0x2) = param_2;
        ctx.PTR_LOOP_1050_6194 = puVar2;
        pass1_1000_4906(CONCAT22(puVar4,puVar1 + 0x4),0x0,0x24
                       );
      }
      return param_1;
    }
    if (*piStack6 == 0x0) { break; }
    piStack6 = (piStack6 & 0xffff0000 | ZEXT24(piStack6 + 0x4))
    ;
  }
  (piStack6 + 0x2) = param_2;
  *piStack6 = param_1;
  return param_1;
}


pub fn pass1_1000_43f0(param_1: u16,param_2: u16)
{
  if (ctx.PTR_LOOP_1050_68b4 == 0x0) {
    pass1_1000_440c(param_2);
    ctx.PTR_LOOP_1050_68b4 = ctx.PTR_LOOP_1050_68b4 + 0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_440c(param_1: u16)
{
  let cVar1: u8;
  let mut pcVar2: String; 
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let lVar6: i32;
  let uVar7: u16;
  let uVar8: u16;
  let mut pcStack8: String; 
  
  uVar3 = pass1_1000_3ec0(0x61ca,ctx.data_seg);
  pcStack8 = CONCAT22(param_1,uVar3);
  if (((param_1 | uVar3) != 0x0) &&
     (_DAT_1050_61ce = CONCAT22(ctx.PTR_LOOP_1050_61d0,DAT_1050_61ce), *pcStack8 != '\0')) {
    str_op_1000_3dbe(CONCAT13((ctx.PTR_USHORT_1050_1050_1050_61de >> 0x8),
                                      CONCAT12(ctx.PTR_USHORT_1050_1050_1050_61de,
                                               ctx.PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc))
                     ,CONCAT22(param_1,uVar3),0x3);
    pcStack8 = CONCAT22(param_1,uVar3 + 0x3);
    cVar1 = *pcStack8;
    if (cVar1 == '-') {
      pcStack8 = CONCAT22(param_1,uVar3 + 0x4);
    }
    uVar5 = 0x0;
    uVar8 = 0x0;
    uVar7 = 0xe10;
    iVar4 = pass1_1000_3e2c(pcStack8 & 0xffff | param_1 << 0x10);
    _DAT_1050_61ce = pass1_1000_52be(iVar4,uVar5,uVar7,uVar8);
    for (; (pcVar2 = pcStack8, *pcStack8 == '+' ||
           (('/' < *pcStack8 && (*pcStack8 < ':'))));
        pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1)))
    {
    }
    if (*pcStack8 == ':') {
      uVar5 = 0x0;
      uVar8 = 0x0;
      uVar7 = 0x3c;
      pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1));
      iVar4 = pass1_1000_3e2c(pcVar2 & 0xffff0000 | (pcStack8 + 0x1));
      lVar6 = pass1_1000_52be(iVar4,uVar5,uVar7,uVar8);
     // uVar5 = (lVar6 >> 0x10);
      _DAT_1050_61ce += lVar6;
        // TODO: refactor for loop
        // for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
      //     pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1))
      //     ) {
      // }
      if (*pcStack8 == ':') {
        pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1));
        iVar4 = pass1_1000_3e2c(pcVar2 & 0xffff0000 | (pcStack8 + 0x1))
        ;
        _DAT_1050_61ce += CONCAT22(uVar5,iVar4);
          // TODO: refactor for loop
          // for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
        //     pcStack8 = (pcStack8 & 0xffff0000 |
        //                        (pcStack8 + 0x1))) {
        // }
      }
    }
    ctx.PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
    if (cVar1 == '-') {
      _DAT_1050_61ce =
           CONCAT22(-(ctx.PTR_LOOP_1050_61d0 + (ctx.DAT_1050_61ce != 0x0)),-DAT_1050_61ce);
    }
    ctx.DAT_1050_61d2 = *pcStack8;
    if (ctx.DAT_1050_61d2 == 0x0) {
      *_PTR_PTR_1050_61e0 = '\0';
    }
    else {
      str_op_1000_3dbe(ctx.PTR__PTR_1050_61e0,pcStack8,0x3);
    }
  }
  ctx.PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
  return;
}



pub fn pass1_1000_455a(param_1: u32,param_2: u16) -> u16

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let UVar5: u16;
  let uVar6: u32;
  let iStack6: i16;
  
  if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) ||
     (0x9 < (param_1 + 0x8))) {
      // goto
      // LAB_1000_4623;
  }
  if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
    uVar3 = (param_1 + 0xa);
    if ((uVar3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
      iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
    }
    else {
      iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
    }
    if ((uVar3 & 0x3) == 0x0) {
      iStack6 += 0x1;
    }
    uVar3 = (uVar3 - 0x46) * 0x16d + ((uVar3 - 0x1) >> 0x2) + iStack6;
    uVar6 = pass1_1000_52f0(uVar3 - 0xd,(uVar3 >> 0xf) - (uVar3 < 0xd),0x7,0x0)
    ;
    iStack6 = uVar6 - iStack6;
    iVar4 = -iStack6;
    if ((param_1 + 0x8) == 0x3) {
      iVar2 = (param_1 + 0xe);
      if ((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < (param_1 + 0x4)))))
//       TODO: goto LAB_1000_460e;
    }
    else {
      piVar1 = (param_1 + 0xe);
      iVar2 = *piVar1;
      if ((SBORROW2(*piVar1,iVar4) != iVar2 + iStack6 < 0x0) ||
         ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1)))) {
          // goto
          // LAB_1000_460e;
      }
    }
//LAB_1000_4623:
    UVar5 = 0x0;
  }
  else {
//LAB_1000_460e:
    UVar5 = 0x1;
  }
  return UVar5;
}



pub fn pass1_1000_462e(param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: i16,param_7: i16,param_8: &mut String,param_9: u16) -> i16

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let UVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let iStack26: i16;
  let local_16: [u8;4];
  let uStack18: u16;
  let iStack14: i16;
  let iStack12: i16;
  let iStack8: i16;
  let local_4: u16;
  let iStack2: i16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  
  iStack2 = param_7 + 0x1;
  local_4 = ctx.data_seg;
  uVar8 = (param_2 * 0x2 + 0x61ae);
  if (((param_1 & 0x3) == 0x0) && (0x2 < param_2)) {
    uVar8 += 0x1;
  }
  pass1_1000_43f0(&iStack2,param_9);
  uVar13 = 0x0;
  uVar12 = 0x3c;
  uVar11 = 0x0;
  uVar10 = 0x3c;
  uVar1 = (param_1 * 0x16d);
  uVar2 = (param_1 + 0x3) >> 0x2;
  uVar3 = uVar2 + param_3;
  uVar6 = uVar1 + uVar3;
  uVar7 = uVar6 + uVar8;
  uVar9 = pass1_1000_52be(uVar7 + 0xe44,
                          ((param_1 * 0x16d) >> 0x10) +
                          ((param_1 + 0x3) >> 0xf) + (param_3 >> 0xf) +
                          CARRY2(uVar2,param_3) + CARRY2(uVar1,uVar3) +
                          (uVar8 >> 0xf) + CARRY2(uVar6,uVar8) +
                          (0xf1bb < uVar7),0x18,0x0);
  uVar9 = pass1_1000_52be(uVar9 + param_4,
                          (uVar9 >> 0x10) + (param_4 >> 0xf) +
                          CARRY2(uVar9,param_4),uVar10,uVar11);
  iVar4 = pass1_1000_52be(uVar9 + param_5,
                          (uVar9 >> 0x10) + (param_5 >> 0xf) +
                          CARRY2(uVar9,param_5),uVar12,uVar13);
  iStack26 = iVar4 + param_6 + ctx.DAT_1050_61ce;
  iStack8 = param_3 + uVar8;
  iStack12 = param_1 + 0x50;
  iStack14 = param_2 + -0x1;
  uStack18 = param_4;
  if (ctx.DAT_1050_61d2 != 0x0) {
    UVar5 = pass1_1000_455a(local_16,param_8);
    if (UVar5 != 0x0) {
      iStack26 += -0xe10;
    }
  }
  return iStack26;
}



pub fn pass1_1000_472c(param_1: u32,param_2: u8) -> *mut u8

{
  let mut pcVar1: String; 
  let uVar2: u16;
  let mut pcVar3: String; 
  let mut pcVar4: String; 
  let uVar5: u16;
  let bVar6: bool;
  
 // uVar5 = (param_1 >> 0x10);
  pcVar3 = param_1;
  bVar6 = true;
  uVar2 = 0xffff;
  pcVar4 = pcVar3;
  loop {
    if (uVar2 == 0x0) { break; }
    uVar2 -= 0x1;
    pcVar1 = pcVar4;
    pcVar4 = pcVar4 + 0x1;
    bVar6 = *pcVar1 == '\0';
    if bVar6 { break; }
  } 
  uVar2 = !uVar2;
  loop {
    if (uVar2 == 0x0) { break; }
    uVar2 -= 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar6 = param_2 == *pcVar1;
    if bVar6 { break;}
  } 
  if (!bVar6) {
    if (param_2 != '\0') {
      return 0x0;
    }
    pcVar3 = pcVar3 + 0x1;
  }
  return pcVar3 + -0x1;
}



pub fn pass1_1000_475e(param_1: u32,param_2: u32) -> i16

{
  let mut pcVar1: String; 
  let cVar2: u8;
  let cVar3: u8;
  let bVar4: u8;
  let bVar3: &mut Struct235;
  let bVar5: &mut Struct236;
  let mut pcVar5: String; 
  let mut pcVar6: String; 
  
  pcVar6 = param_2;
  pcVar5 = param_1;
  bVar5 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
  loop {
    loop {
      cVar3 = bVar5;
      if (cVar3 == '\0'){ 
        // goto LAB_1000_479d;
      }
      pcVar1 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
      cVar3 = *pcVar1;
      cVar2 = *pcVar5;
      bVar5 = CONCAT11(cVar2,cVar3);
      pcVar5 = pcVar5 + 0x1;
      if cVar2 != cVar3 { break; }
    } 
    bVar4 = cVar3 + 0xbf + (-((cVar3 + 0xbf) < 0x1a) & 0x20) + 0x41;
    bVar3._0_1_ = cVar2 + 0xbf;
    bVar5._0_1_ = bVar3 + (-(bVar3 < 0x1a) & 0x20) + 0x41;
    bVar5 = CONCAT11(bVar4,bVar5);
  } while (bVar5 == bVar4);
  cVar3 = (bVar5 < bVar4) * -0x2 + '\x01';
//LAB_1000_479d:
  return cVar3;
}



pub fn pass1_1000_47a4(param_1: u32,param_2: u32,param_3: u16) -> u16

{
  let pbVar1: *mut u8;
  let bVar2: u8;
  let puVar3: *mut u16;
  let pbVar4: *mut u8;
  let iVar5: i16;
  let pbVar6: *mut u8;
  let puVar7: *mut u16;
  let uVar8: u16;
  let local_22: [u16;0x10];
  
  puVar7 = local_22;
    // TODO: refactor for loop
  // for (iVar5 = 0x10; iVar5 != 0x0; iVar5 += -0x1) {
  //   puVar3 = puVar7;
  //   puVar7 = puVar7 + 0x1;
  //   *puVar3 = 0x0;
  // }
  pbVar6 = param_2;
  loop {
    pbVar1 = pbVar6;
    pbVar6 = pbVar6 + 0x1;
    bVar2 = *pbVar1;
    if (bVar2 == 0x0) { break; }
    pbVar1 = (local_22 + (bVar2 >> 0x3));
    *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
  }
  pbVar1 = param_1;
  if (param_1 == 0x0) {
    pbVar1 = pbRam105061e4;
  }
  loop {
    pbRam105061e4 = pbVar1;
   // uVar8 = (pbRam105061e4 >> 0x10);
    pbVar6 = (pbRam105061e4 + 0x1);
    bVar2 = *pbRam105061e4;
    if (bVar2 == 0x0) {
      return 0x0;
    }
    pbVar1 = (pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
    if ('\x01' << (bVar2 & 0x7) & (local_22 + (bVar2 >> 0x3))) !=
        0x0) == false { break; }
  }
  loop {
    pbVar4 = pbVar6;
    bVar2 = *pbVar4;
    if (bVar2 == 0x0) {
        // goto
        // LAB_1000_483c;
    }
    pbVar6 = pbVar4 + 0x1;
      if (('\x01' << (bVar2 & 0x7) & (local_22 + (bVar2 >> 0x3))) ==
          0x0) == false {
          break;
      }
  }
  *pbVar4 = 0x0;
  pbVar4 = pbVar4 + 0x1;
//LAB_1000_483c:
  pbRam105061e4 = (pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
  return pbRam105061e4;
}



pub fn pass1_1000_484c(param_1: u32,param_2: u32,param_3: u16) -> u16

{
  let pbVar1: *mut u8;
  let pbVar2: *mut u8;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let pbVar6: *mut u8;
  let pbVar7: *mut u8;
  let iVar8: i16;
  let bVar9: bool;
  let bVar10: bool;
  
  if (param_3 == 0x0) {
    return 0x0;
  }
  loop {
   // iVar8 = (param_2 >> 0x10);
    pbVar7 = param_2;
   // iVar3 = (param_1 >> 0x10);
    pbVar6 = param_1;
    uVar4 = ~pbVar7;
    uVar4 = ((param_3 - 0x1) - uVar4 & -(param_3 - 0x1 < uVar4)) + uVar4;
    uVar5 = ~pbVar6;
    uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 0x1;
    bVar9 = param_3 < uVar4;
    param_3 -= uVar4;
    bVar10 = param_3 == 0x0;
    loop {
      if (uVar4 == 0x0) { break; }
      uVar4 -= 0x1;
      pbVar2 = pbVar7;
      pbVar7 = pbVar7 + 0x1;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar9 = *pbVar1 < *pbVar2;
      bVar10 = *pbVar1 == *pbVar2;
        if bVar10 == false { break; }
    }
    param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
    if (!bVar10) {
      return (0x1 - bVar9) - (bVar9 != 0x0);
    }
    if (param_3 == 0x0) {
      return uVar4;
    }
    if (pbVar6 == 0x0) {
      iVar3 += 0x6c;
    }
    param_1 = CONCAT22(iVar3,pbVar6);
    if (pbVar7 == 0x0) {
      param_2 = (iVar8 + 0x6c) << 0x10;
      param_1 = CONCAT22(iVar3,pbVar6);
    }
  }
}



pub fn pass1_1000_48a8(param_1: u32,param_2: u32,param_3: i16) -> u16

{
  let puVar1: *mut u16;
  let puVar2: *mut u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
  let iVar8: i16;
  
  if (param_3 != 0x0) {
    loop {
     // iVar3 = (param_2 >> 0x10);
      puVar6 = param_2;
     // iVar8 = (param_1 >> 0x10);
      puVar7 = param_1;
      uVar4 = ~puVar7;
      uVar4 = ((param_3 - 0x1) - uVar4 & -(param_3 - 0x1 < uVar4)) + uVar4;
      uVar5 = ~puVar6;
      uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 0x1;
      param_3 -= uVar4;
        // TODO: refactor for loop
      // for (uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
      //   puVar2 = puVar7;
      //   puVar7 = puVar7 + 0x1;
      //   puVar1 = puVar6;
      //   puVar6 = puVar6 + 0x1;
      //   *puVar2 = *puVar1;
      // }
        // TODO: refactor for loop
      // for (uVar4 = ((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 -= 0x1) {
      //   puVar2 = puVar7;
      //   puVar7 = (puVar7 + 0x1);
      //   puVar1 = puVar6;
      //   puVar6 = (puVar6 + 0x1);
      //   *puVar2 = *puVar1;
      // }
      if (param_3 == 0x0) { break; }
      if (puVar6 == 0x0) {
        iVar3 += 0x6c;
      }
      param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
      param_2 = CONCAT22(iVar3,puVar6);
      if (puVar7 == 0x0) {
        param_1 = (iVar8 + 0x6c) << 0x10;
        param_2 = CONCAT22(iVar3,puVar6);
      }
    }
  }
  return param_1;
}



pub fn pass1_1000_4906(param_1: &mut Struct20,WNDCLASS16 *in_wnd_class,param_3: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let iVar8: i16;
  
  if (param_3 != 0x0) {
   // iVar8 = (param_1 >> 0x10);
    uVar5 = -param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = in_wnd_class & 0xff | in_wnd_class << 0x8;
    puVar7 = param_1;
      // TODO: refactor for loop
    // for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
    //   puVar1 = puVar7;
    //   puVar7 = puVar7 + 0x1;
    //   *puVar1 = uVar3;
    // }
    // for (uVar6 = ((uVar6 & 0x1) != 0x0);
    //     uVar2 = (in_wnd_class & 0xff), uVar6 != 0x0; uVar6 -= 0x1) {
    //   puVar1 = puVar7;
    //   puVar7 = (puVar7 + 0x1);
    //   *puVar1 = uVar2;
    // }
    if (uVar5 != 0x0) {
        // TODO: refactor for loop
      // for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
      //   puVar1 = puVar7;
      //   puVar7 = puVar7 + 0x1;
      //   *puVar1 = uVar3;
      // }
      // for (uVar6 = ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
      //   puVar1 = puVar7;
      //   puVar7 = (puVar7 + 0x1);
      //   *puVar1 = uVar2;
      // }
    }
  }
  return param_1;
}



pub fn  pass1_1000_49b2(param_1: u16) -> i16

{
  return (param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf);
}



pub fn pass1_1000_49c6(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16,param_7: *mut u8,param_8: i16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let lVar5: i32;
  let uStack20: u16;
  let uStack18: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u16;
  let uStack6: u16;
  let local_4: u16;
  let iStack2: i16;
  
  iStack2 = param_8 + 0x1;
  local_4 = SUB42(ctx.data_seg,0x0);
  uStack20 = param_3;
  uStack18 = param_4;
  lVar5 = pass1_1000_52be(param_5 - 0x1,-(param_5 == 0x0),param_6,0x0);
  uStack8 = (lVar5 + 0x8);
  uStack6 = ((lVar5 + 0x8) >> 0x10) * 0x100 + param_4;
  loop {
    if (uStack6 < uStack18) {
      return 0x0;
    }
    if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
      return 0x0;
    }
    uStack14 = param_5 >> 0x1;
    if (uStack14 == 0x0) {
      if ((param_5 != 0x0) && (iVar4 = (*param_7)(), iVar4 == 0x0)) {
        return uStack20;
      }
      return 0x0;
    }
    uVar1 = uStack14;
    if ((param_5 & 0x1) == 0x0) {
      uVar1 = uStack14 - 0x1;
    }
    uVar2 = (uVar1 * param_6);
    uVar3 = uVar2 + uStack20;
    iStack10 = ((uVar1 * param_6 >> 0x10) +
               CARRY2(uVar2,uStack20)) * 0x100 + uStack18;
    uStack12 = uVar3;
    iVar4 = (*param_7)();
    if (iVar4 == 0x0) { break; }
    if (iVar4 < 0x0) {
      uStack8 = -param_6 + uStack12;
      uStack6 = (CARRY2(-param_6,uStack12) - (param_6 != 0x0)) * 0x100 +
                iStack10;
      uVar1 = param_5 & 0x1;
      param_5 = uStack14;
      if (uVar1 == 0x0) {
        param_5 = uStack14 - 0x1;
      }
    }
    else {
      uStack20 = param_6 + uStack12;
      uStack18 = CARRY2(param_6,uStack12) * 0x100 + iStack10;
      param_5 = uStack14;
    }
  }
  return uVar3;
}



pub fn pass1_1000_4aea(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: *mut u8,
               param_6: i16,param_7: i16,param_8: u16,param_9: u16,param_10: u16)

{
  let puVar1: *mut u16;
  let ppcVar2: u32;
  let lVar3: i32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let puVar11: &mut Struct171;
  let uVar11: u16;
  let uVar12: u16;
  let bVar13: bool;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let uVar17: u16;
  let uVar18: u16;
  let uVar19: u16;
  let uStack4: u16;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  uStack4 = SUB42(ctx.data_seg,0x0);
  uVar12 = SUB42(ctx.data_seg,0x0);
  if ((param_4 != 0x0) && (param_3 != 0x0)) {
      // TODO: refactor for loop
//       for (iVar8 = param_3 + -0x1; iVar8 != 0x0; iVar8 += -0x1) {
//       iVar4 = (*param_5)(param_9);
//       if (iVar4 < 0x0) {
//         uVar5 = param_3 - 0x1;
//         iVar8 = 0x0;
//         loop {
//           uVar5 >>= 0x1;
//           iVar8 += -0x1;
//       if (iVar8 != 0x0 && uVar5 != 0x0) == false { break;}
//         }
//         if (((-iVar8 * 0x8 >> 0x10) != 0x0) ||
//            (uVar5 = pass1_1000_3bac(), uVar5 < (-iVar8 * 0x8))) {
//           exit_1000_25f2(0x4b7b,param_9,param_7,-0x4,param_8,param_9,param_10);
//           return;
//         }
//         puVar11 = &stack0xfff6;
//         lVar3 = (param_3 - 0x1) * param_4;
//         uVar6 = lVar3;
//         uVar5 = uVar6 + param_1;
//         uVar6 = ((lVar3 >> 0x10) + CARRY2(uVar6,param_1)) * 0x100 +
//                 param_2;
// //LAB_1000_4b7d:
//         if (puVar11 <= &stack0xffee) {
//           return;
//         }
// //LAB_1000_4b81:
//         if ((param_2 < uVar6) || ((param_2 <= uVar6 && (param_1 < uVar5)))) {
//           puVar1 = &puVar11.field_0x14;
//           uVar10 = uVar5 + *puVar1;
//           uVar9 = uVar6 + (-CARRY2(uVar5,*puVar1) & 0x6c);
//           uVar14 = param_1;
//           uVar15 = param_2;
//           uVar18 = uVar5;
//           uVar19 = uVar6;
//           uVar7 = param_1;
//           uVar11 = param_2;
// //LAB_1000_4bbc:
//           loop {
//             puVar1 = &puVar11.field_0x14;
//             bVar13 = CARRY2(param_1,*puVar1);
//             param_1 += *puVar1;
//             param_2 += -bVar13 & 0x6c;
//             if ((param_1 != uVar18) || (param_2 != uVar19)) {
//               ppcVar2 = &puVar11.field_0x16;
//               iVar8 = (**ppcVar2)(param_9,param_1,param_2,uVar7,uVar11);
//               if (iVar8 < 0x1) {
//                 if (iVar8 != 0x0) {
//                   uVar14 = param_1;
//                   uVar15 = param_2;
//                 }
// //                 TODO: goto LAB_1000_4bbc;
//               }
//             }
//             loop {
//               uVar17 = uVar6;
//               uVar16 = uVar5;
//               puVar1 = &puVar11.field_0x14;
//               bVar13 = uVar10 < *puVar1;
//               uVar10 -= *puVar1;
//               uVar9 -= -bVar13 & 0x6c;
//               ppcVar2 = &puVar11.field_0x16;
//               iVar8 = (**ppcVar2)(param_9,uVar7,uVar11,uVar10,uVar9);
//               if (0x0 < iVar8) break;
//               uVar5 = uVar10;
//               uVar6 = uVar9;
//             } while (((iVar8 != 0x0) || (uVar5 = uVar16, uVar6 = uVar17, uVar10 != uVar7))
//                     || (uVar9 != uVar11));
//             if ((uVar9 < param_2) || ((uVar9 <= param_2 && (uVar10 <= param_1))))
// //             TODO: goto LAB_1000_4c58;
//             pass1_1000_4ceb(puVar11.field_0x14,param_1,uVar10,uVar9);
//             uVar14 = param_1;
//             uVar15 = param_2;
//             uVar5 = uVar10;
//             uVar6 = uVar9;
//           } while( true );
//         }
// //         TODO: goto LAB_1000_4b7d;
//       }
//     }
  }
  return;
//LAB_1000_4c58:
  param_1 = uVar7;
  param_2 = uVar11;
  pass1_1000_4ceb(puVar11.field_0x14,uVar7,uVar10,uVar9);
  uVar11 = ((uVar19 - (-(uVar18 < uVar16) & 0x6c)) - uVar17) +
           (-CARRY2(uVar18 - uVar16,param_1) & 0x6c) + param_2;
  uVar7 = -((uVar18 - uVar16) + param_1 < uVar14) & 0x6c;
  uVar5 = uVar14;
  uVar6 = uVar15;
  if ((uVar7 <= uVar11) && (uVar15 <= uVar11 - uVar7)) {
    uVar5 = uVar18;
    uVar6 = uVar19;
    param_1 = uVar16;
    param_2 = uVar17;
  }
//   TODO: goto LAB_1000_4b81;
}



pub fn pass1_1000_4ceb(param_1: u16,param_2: i16,param_3: i16,param_4: u16)
{
  let puVar1: *mut u8;
  let puVar2: *mut u16;
  let uVar3: u8;
  let uVar4: u16;
  
  if ((param_1 & 0x1) != 0x0) {
    param_1 -= 0x1;
    puVar1 = (param_1 + param_3);
    uVar3 = *puVar1;
    *puVar1 = *(param_1 + param_2);
    *(param_1 + param_2) = uVar3;
    if (param_1 == 0x0) {
      return;
    }
  }
  loop {
    param_1 -= 0x2;
    puVar2 = (param_1 + param_3);
    uVar4 = *puVar2;
    *puVar2 = (param_1 + param_2);
    (param_1 + param_2) = uVar4;
      if param_1 == 0 { break; }
  }
  return;
}



pub fn pass1_1000_4d0c(param_1: u16)
{
  ctx.DAT_1050_61e8 = param_1;
  ctx.PTR_LOOP_1050_61ea = 0x0;
  return;
}



pub fn pass1_1000_4d24() -> u16

{
  let lVar1: i32;
  
  lVar1 = pass1_1000_52be(ctx.DAT_1050_61e8,PTR_LOOP_1050_61ea,
                          (s_TPPOPMENU_1050_43fa + 0x3),0x3);
  ctx.PTR_LOOP_1050_61ea = ((lVar1 + 0x269ec3) >> 0x10);
  ctx.DAT_1050_61e8 = (lVar1 + 0x269ec3);
  return ctx.PTR_LOOP_1050_61ea & 0x7fff;
}


pub fn pass1_1000_4f1a(param_1: i16,param_2: u16,param_3: u16) -> *mut i16

{
  let piVar1: *mut i16;
  let mut pcVar2: String; 
  let mut str: String;
  let piVar3: *mut i16;
  let piVar4: *mut i16;
  let mut pcVar5: String; 
  let iVar6: i16;
  let iVar7: i16;
  
  iVar7 = 0x19;
  iVar6 = 0x19;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar6,param_2,param_3);
  str = poss_str_op_1000_28dc(ctx, iVar7);
  if (str != 0x0) {
    iVar6 = 0x9;
    if (*str == 'M') {
      iVar6 = 0xf;
    }
    str = str + iVar6;
    iVar6 = 0x22;
    pcVar5 = str;
    loop {
      if (iVar6 == 0x0) { break; }
      iVar6 += -0x1;
      pcVar2 = pcVar5;
      pcVar5 = pcVar5 + 0x1;
          if pcVar2 == '\r' { break; }
    }
    pcVar5[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar4 = &ctx.PTR_LOOP_1050_63fe;
  loop {
    piVar1 = piVar4;
    piVar4 = piVar4 + 0x1;
    iVar6 = *piVar1;
    piVar3 = piVar4;
    if ((iVar6 == param_1) || (piVar3 = (iVar6 + 0x1), piVar3 == 0x0)) {
      return piVar3;
    }
    iVar6 = -0x1;
    loop {
      if (iVar6 == 0x0) { break; }
      iVar6 += -0x1;
      piVar1 = piVar4;
      piVar4 = (piVar4 + 0x1);
          if *piVar1 == '\0' { break; }
    }
  }
}


pub fn pass1_1000_4f2e(param_1: u16) -> u16

{
  let pcVar1: u32;
  let uVar2: u16;
  let uVar3: u8;
  
  uVar2 = 0x3b50;
  uVar3 = 0x0;
  if (true) {
    pcVar1 = swi(0x21);
    uVar2 = (*pcVar1)(ctx.data_seg,param_1 + 0x1);
  }
  else {
    DOS3Call(&ctx.PTR_LOOP_1050_1000);
  }
  if (!uVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return 0xffff;
}

pub fn pass1_1000_5008(param_1: u16,param_2: u16,param_3: u16,param_4: i16)
{
  let unaff_CS: u16;
  let unaff_SS: u16;
  let iStack2: i16;
  
  iStack2 = param_4 + 0x1;
  pass1_1000_5026(0x0,param_1,param_2,param_3,&iStack2,unaff_CS,unaff_SS);
  return;
}



pub fn pass1_1000_5224(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let bVar10: bool;
  let cVar11: u8;
  let uVar9: u16;
  
  cVar11 = param_2 < 0x0;
  if (cVar11) {
    bVar10 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -bVar10 - param_2;
  }
  if (param_4 < 0x0) {
    cVar11 += '\x01';
    bVar10 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -bVar10 - param_4;
  }
  uVar3 = param_1;
  uVar5 = param_3;
  uVar6 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = ((param_2 % param_3 << 0x10 | param_1) /
                 param_3);
  }
  else {
    loop {
      uVar8 = uVar9 >> 0x1;
      uVar5 = uVar5 >> 0x1 | ((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar6 & 0x1) != 0x0) << 0xf;
      uVar6 = uVar7;
      uVar9 = uVar8;
          if uVar8 == 0x0 { break; }
    }
    uVar1 = CONCAT22(uVar7,uVar3) / uVar5;
    iVar4 = uVar1;
    lVar2 = param_3 * (uVar1 & 0xffff);
   // uVar3 = (lVar2 >> 0x10);
    uVar5 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar5)) ||
       ((param_2 <= uVar5 && (param_1 < lVar2)))) {
      iVar4 += -0x1;
    }
    uVar3 = 0x0;
  }
  if (cVar11 == '\x01') {
    bVar10 = iVar4 != 0x0;
    iVar4 = -iVar4;
    uVar3 = -bVar10 - uVar3;
  }
  return CONCAT22(uVar3,iVar4);
}



pub fn pass1_1000_52be(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  if ((param_4 | param_2) == 0x0) {
    return param_1 * param_3;
  }
  return param_1 * param_3 & 0xffff |
         ((param_1 * param_3 >> 0x10) +
                param_2 * param_3 + param_1 * param_4) << 0x10;
}



pub fn pass1_1000_52f0(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let bVar12: bool;
  let bVar13: bool;
  
  bVar13 = param_2 < 0x0;
  if (bVar13) {
    bVar12 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -bVar12 - param_2;
  }
  uVar11 = bVar13;
  if (param_4 < 0x0) {
    bVar13 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -bVar13 - param_4;
  }
  uVar3 = param_1;
  uVar4 = param_3;
  uVar8 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    iVar5 = ((param_2 % param_3 << 0x10 | param_1) %
                 param_3);
    iVar6 = 0x0;
    if ((uVar11 - 0x1) < 0x0) {
        // goto
        // LAB_1000_538a;
    }
  }
  else {
    loop {
      uVar10 = uVar9 >> 0x1;
      uVar4 = uVar4 >> 0x1 | ((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar8 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar8 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar7;
      uVar9 = uVar10;
          if uVar10 == 0 { break; }
    }
    uVar1 = CONCAT22(uVar7,uVar3) / uVar4;
    uVar3 = uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * param_3;
   // uVar8 = (lVar2 >> 0x10);
    uVar4 = lVar2;
    uVar9 = uVar8 + uVar3;
    if (((CARRY2(uVar8,uVar3)) || (param_2 < uVar9)) ||
       ((param_2 <= uVar9 && (param_1 < uVar4)))) {
      bVar13 = uVar4 < param_3;
      uVar4 -= param_3;
      uVar9 = (uVar9 - param_4) - bVar13;
    }
    iVar5 = uVar4 - param_1;
    iVar6 = (uVar9 - param_2) - (uVar4 < param_1);
    if (-0x1 < (uVar11 - 0x1)) {
        // goto
        // LAB_1000_538a;
    }
  }
  bVar13 = iVar5 != 0x0;
  iVar5 = -iVar5;
  iVar6 = -bVar13 - iVar6;
//LAB_1000_538a:
  return CONCAT22(iVar6,iVar5);
}



pub fn pass1_1000_5390(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  
  uVar3 = param_1;
  uVar8 = param_4;
  uVar6 = param_2;
  uVar9 = param_3;
  if param_4 == 0x0 {
    uVar3 = param_2 / param_3;
    iVar4 = ((param_2 % param_3 << 0x10 | param_1) /
                 param_3);
  }
  else {
    loop {
      uVar5 = uVar8 >> 0x1;
      uVar9 = uVar9 >> 0x1 | ((uVar8 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar6 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar5;
      uVar6 = uVar7;
        if uVar5 == 0 { break; }
    }
    uVar1 = CONCAT22(uVar7,uVar3) / uVar9;
    iVar4 = uVar1;
    lVar2 = param_3 * (uVar1 & 0xffff);
   // uVar3 = (lVar2 >> 0x10);
    uVar8 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar8)) ||
       ((param_2 <= uVar8 && (param_1 < lVar2)))) {
      iVar4 += -0x1;
    }
    uVar3 = 0x0;
  }
  return CONCAT22(uVar3,iVar4);
}



pub fn pass1_1000_53f0(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let bVar11: bool;
  
  uVar3 = param_1;
  uVar4 = param_4;
  uVar9 = param_2;
  uVar10 = param_3;
  if (param_4 == 0x0) {
    iVar6 = ((param_2 % param_3 << 0x10 | param_1) %
                 param_3);
    iVar7 = 0x0;
  }
  else {
    loop {
      uVar5 = uVar4 >> 0x1;
      uVar10 = uVar10 >> 0x1 | ((uVar4 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar9 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar9 & 0x1) != 0x0) << 0xf;
      uVar4 = uVar5;
      uVar9 = uVar8;
          if uVar5 == 0 { break; }
    }
    uVar1 = CONCAT22(uVar8,uVar3) / uVar10;
    uVar3 = uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * param_3;
   // uVar9 = (lVar2 >> 0x10);
    uVar4 = lVar2;
    uVar10 = uVar9 + uVar3;
    if (((CARRY2(uVar9,uVar3)) || (param_2 < uVar10)) ||
       ((param_2 <= uVar10 && (param_1 < uVar4)))) {
      bVar11 = uVar4 < param_3;
      uVar4 -= param_3;
      uVar10 = (uVar10 - param_4) - bVar11;
    }
    iVar6 = -(uVar4 - param_1);
    iVar7 = -(uVar4 - param_1 != 0x0) -
            ((uVar10 - param_2) - (uVar4 < param_1));
  }
  return CONCAT22(iVar7,iVar6);
}



pub fn  pass1_1000_545a(param_1: u32,param_2: u32) -> i16

{
  let pbVar1: *mut u8;
  let bVar2: u8;
  let bVar3: u8;
  let bVar4: u8;
  let pbVar5: *mut u8;
  let pbVar6: *mut u8;
  
  pbVar6 = param_2;
  pbVar5 = param_1;
  bVar4 = 0xff;
  loop {
    loop {
      if (bVar4 == 0x0) {
          // goto
          // LAB_1000_5499;
      }
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar4 = *pbVar1;
      bVar3 = *pbVar5;
      pbVar5 = pbVar5 + 0x1;
       if bVar3 != bVar4 {break;}
    }
    bVar2 = bVar4 + 0xbf + (-((bVar4 + 0xbf) < 0x1a) & 0x20) + 0x41;
    bVar3 += 0xbf;
    bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20) + 0x41;
      if bVar4 != bVar2 {break;}
  }
  bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;
//LAB_1000_5499:
  return bVar4;
}



pub fn pass1_1000_54a0(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let iVar8: i16;
  
  if (param_3 != 0x0) {
   // iVar8 = (param_1 >> 0x10);
    uVar5 = -param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = param_2 & 0xff | param_2 << 0x8;
    puVar7 = param_1;
      // TODO: refactor for loop
    // for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
    //   puVar1 = puVar7;
    //   puVar7 = puVar7 + 0x1;
    //   *puVar1 = uVar3;
    // }
    // for (uVar6 = ((uVar6 & 0x1) != 0x0); uVar2 = (param_2 & 0xff),
    //     uVar6 != 0x0; uVar6 -= 0x1) {
    //   puVar1 = puVar7;
    //   puVar7 = (puVar7 + 0x1);
    //   *puVar1 = uVar2;
    // }
    if (uVar5 != 0x0) {
        // TODO: refactor for loop
      // for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
      //   puVar1 = puVar7;
      //   puVar7 = puVar7 + 0x1;
      //   *puVar1 = uVar3;
      // }
      // for (uVar6 = ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
      //   puVar1 = puVar7;
      //   puVar7 = (puVar7 + 0x1);
      //   *puVar1 = uVar2;
      // }
    }
  }
  return param_1;
}



pub fn pass1_1000_54e8(param_1: *mut u8,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: u16)

{
  let iVar1: i16;
  let iVar2: i16;
  let uStack14: u16;
  let uStack10: i16;
  let uStack8: u16;
  
  iVar2 = param_5 + param_4 * param_3;
  iVar1 = param_3;
  while (iVar1 += -0x1, -0x1 < iVar1) {
    iVar2 -= param_4;
    uStack8 = param_6;
    uStack14 = 0x5506;
    iStack10 = iVar2;
    (*param_1)();
  }
  return;
}



pub fn pass1_1000_5512(param_1: *mut u8,param_2: u16,param_3: i16,param_4: i16,param_5: u16)
{
  let bVar1: bool;
  let uStack4: u16;
  
  pass1_1000_52be(param_3,param_4,param_5,0x0);
  loop {
    bVar1 = param_3 == 0x0;
    param_3 += -0x1;
    param_4 -= bVar1;
    if (param_4 < 0x0) { break; }
    uStack4 = param_2;
    (*param_1)();
  }
  return;
}



pub fn pass1_1000_5586(param_1: *mut u8,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: u16)

{
  let iVar1: i16;
  let iVar2: i16;
  let uStack14: u16;
  let uStack10: i16;
  let uStack8: u16;
  
  iVar1 = param_5;
  iVar2 = param_3;
  while (iVar2 += -0x1, -0x1 < iVar2) {
    uStack8 = param_6;
    uStack14 = 0x559b;
    iStack10 = iVar1;
    (*param_1)();
    iVar1 += param_4;
  }
  return;
}


pub fn pass1_1000_55b1(param_1: i16,param_2: u16,param_3: u16) -> *mut i16

{
  let piVar2: *mut i16;
  let mut pcVar3: String; 
  let mut str: String;
  let piVar4: *mut i16;
  let piVar5: *mut i16;
  let mut pcVar6: String; 
  let iVar7: i16;
  let iVar8: i16;
  let mut piVar1: String; 
  
  iVar8 = 0x14;
  iVar7 = 0x14;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar7,param_2,param_3);
  str = poss_str_op_1000_28dc(ctx, iVar8);
  if (str != 0x0) {
    iVar7 = 0x9;
    if (*str == 'M') {
      iVar7 = 0xf;
    }
    str = str + iVar7;
    iVar7 = 0x22;
    pcVar6 = str;
    loop {
      if (iVar7 == 0x0) { break; }
      iVar7 += -0x1;
      pcVar3 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
        if *pcVar3 == '\r' { break; }
    }
    pcVar6[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar5 = &ctx.PTR_LOOP_1050_63fe;
  loop {
    piVar2 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar2;
    piVar4 = piVar5;
    if ((iVar7 == param_1) || (piVar4 = (iVar7 + 0x1), piVar4 == 0x0)) {
      return piVar4;
    }
    iVar7 = -0x1;
    loop {
      if (iVar7 == 0x0) { break; }
      iVar7 += -0x1;
      piVar1 = piVar5;
      piVar5 = (piVar5 + 0x1);
        if *piVar1 == '\0' { break; }
    }
  }
}
