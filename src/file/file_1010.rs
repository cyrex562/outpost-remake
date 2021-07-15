use crate::util::{CONCAT22, ZEXT24};
use crate::file::file_1008::{write_to_file_1008_7cac, read_file_1008_7dee, read_file_1008_7cfe, close_file_1008_496c};
use crate::mem_1000::mem_op_1000_179c;
use crate::sys_api::set_err_mode_1010_8b14;
use crate::ui::ui_1010::msg_box_op_1010_8bb4;
use crate::pass::pass_1008::pass1_1008_6562;
use crate::pass::pass_1010::pass1_1010_878c;
use crate::struct_ops::struct_1008::{struct_op_1008_3f92, struct_op_1008_48fe};
use crate::win_struct::HINSTANCE16;

pub fn file_1010_0c7c(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16)
{
  let puVar1: u32;
  let ppcVar2: u32;
  let BVar3: bool;
  let uVar4: &mut Struct229;
  let uVar5: u16;
  let extraout_DX: *mut u8;
  let iVar6: &mut Struct228;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let local_2a: [u16;0x2];
  let uStack38: u16;
  let puStack26: u32;
  let puStack22: u32;
  let local_12: [u16;0x5];
  let paStack8: &mut Struct229;
  let local_6: &mut Struct229;
  let uStack4: u16;
  
  uVar7 = param_2;
 // uVar8 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar7,uVar8,0x6,0x1008,param_5);
  if (param_3 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d4;
  }
  else {
    BVar3 = read_file_1008_7dee(uVar7,uVar8,&local_6,0x0,param_5,0x2,0x1008);
    if (BVar3 != 0x0) {
      paStack8 = 0x0;
      loop {
        iVar6 = param_1;
       // uVar5 = (param_1 >> 0x10);
        if (local_6 <= paStack8) { break; }
        uVar4 = local_6;
        mem_op_1000_179c(0xa,param_4,0x1000);
        puStack26 = CONCAT22(param_4,uVar4);
        if ((param_4 | uVar4) == 0x0) {
          puStack22 = 0x0;
        }
        else {
          puStack26 = 0x389a;
          uVar4.field_0x2 = 0x1008;
          puStack26 = 0xea8;
          uVar4.field_0x2 = 0x1010;
          puStack22 = puStack26;
        }
        BVar3 = read_file_1008_7dee(uVar7,uVar8,local_12,0x0,param_5,0x2,0x1008);
        if ((BVar3 == 0x0) ||
           (BVar3 = read_file_1008_7dee(uVar7,uVar8,puStack22 + 0x6,0x0,
                                        (puStack22 >> 0x10),0x4,0x1008),
           BVar3 == 0x0)) {
          puStack26 = puStack22;
          if (puStack22 != 0x0) {
            ppcVar2 = *puStack22;
            (**ppcVar2)(0x1008,puStack22,(puStack22 >> 0x10),0x1);
          }
//           TODO: goto LAB_1010_0cb1;
        }
       // uVar6 = (puStack22 >> 0x10);
        (puStack22 + 0x4) = local_12[0];
        puVar1 = iVar6.field_0xa;
        ppcVar2 = (*iVar6.field_0xa + 0x8);
        (**ppcVar2)(0x8,puVar1,(puVar1 >> 0x10),puStack22,uVar6);
        paStack8 = &paStack8.field_0x1;
        param_4 = extraout_DX;
      }
      BVar3 = read_file_1008_7dee(uVar7,uVar8,&iVar6.field_0xe,0x0,uVar5,0x2,0x1008);
      if ((BVar3 != 0x0) &&
         (BVar3 = read_file_1008_7dee(uVar7,uVar8,&iVar6.field_0x10,0x0,uVar5,0x2,0x1008)
         , BVar3 != 0x0)) {
          // TODO: refactor for loop
        // for (uStack4 = 0x0; uStack4 < 0xa; uStack4 += 0x1) {
        //   BVar3 = read_file_1008_7dee(uVar7,uVar8,local_2a,0x0,param_5,0x2,0x1008)
        //   ;
        //   if (BVar3 == 0x0) goto LAB_1010_0cb1;
        //   uVar5 = uStack4;
        //   if (ctx.PTR_LOOP_1050_0312 < 0x2) {
        //     uVar5 = pass1_1008_738c(uVar7,uVar8,uStack4);
        //   }
        //   (uVar5 * 0x8 + 0xe28) = local_2a[0];
        //   uStack38 = uVar5;
        // }
        if (0x2 < ctx.PTR_LOOP_1050_0312) {
          uStack4 = 0x0;
          loop {
            BVar3 = read_file_1008_7dee(uVar7,uVar8,local_2a,0x0,param_5,0x2,
                                        0x1008);
            if (BVar3 == 0x0) {
                // goto
                // LAB_1010_0cb1;
            }
            (uStack4 * 0x8 + 0xea8) = local_2a[0];
            uStack4 += 0x1;
                if uStack4 >= 0x3 { break; }
          }
        }
        return;
      }
    }
//LAB_1010_0cb1:
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}


pub fn write_to_file_1010_6372(param_1: u32,param_2: u32,param_3: u16)
{
  let BVar1: bool;
  let iVar2: &mut Struct729;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_10: [u32;0x2];
  let local_8: u32;
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
   // uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    local_10[0] = iVar2.field_0xa;
    uVar3 = param_2;
   // uVar4 = (param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c
                      (uVar3,uVar4,local_10,param_3,0x4,0x1008);
    if (BVar1 != 0x0) {
      local_8 = iVar2.field_0xe;
      BVar1 = write_to_file_1008_7e1c
                        (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
      if (BVar1 != 0x0) {
        local_8 = iVar2.field_0x12;
        BVar1 = write_to_file_1008_7e1c
                          (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
        if (BVar1 != 0x0) {
          local_8 = iVar2.field_0x16;
          BVar1 = write_to_file_1008_7e1c
                            (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
          if (BVar1 != 0x0) {
            local_8 = iVar2.field_0x1a;
            BVar1 = write_to_file_1008_7e1c
                              (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
            if (BVar1 != 0x0) {
              local_8 = iVar2.field_0x1e;
              BVar1 = write_to_file_1008_7e1c
                                (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
              if (BVar1 != 0x0) {
                local_8 = iVar2.field_0x22;
                BVar1 = write_to_file_1008_7e1c
                                  (uVar3,uVar4,&local_8,param_3,0x4,0x1008
                                  );
                if (BVar1 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}


pub fn write_to_file_1010_6846(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let local_c: [u16;0x5];
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 != 0x0) {
    iVar3 = param_1;
   // uVar1 = (param_1 >> 0x10);
    uVar4 = param_2;
   // uVar5 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,iVar3 + 0xa,uVar1,0x114,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,iVar3 + 0x11e,uVar1,0x2a,0x1008)
      ;
      if (BVar2 != 0x0) {
        local_c[0] = (iVar3 + 0x148);
        BVar2 = write_to_file_1008_7e1c
                          (uVar4,uVar5,local_c,param_3,0x2,0x1008);
        if (BVar2 != 0x0) {
          return;
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}


pub unsafe fn unk_io_op_1010_830a(param_1: u32,param_2: u16,param_3: u16) -> Option<&mut Struct43>

{
  let in_ax: u16;
  let pu_var1: u32;
  let pu_var2: u32;
  let in_dx: *mut u8;
  let u_var3: u16;
  let i_var3: &mut Struct45;
  let i_var2: &mut Struct44;
  let i_var4: i16;
  let unaff_cs: HINSTANCE16;
  let u_var5: u16;
  let u_var6: u16;
  let local_2e: u32;
  let u_stack10: u32;
  let pa_stack6: &mut Struct43;
  
  pa_stack6 = 0x0;
  i_var3 = (param_2 * 0x10);
  u_var5 = param_1;
 // u_var6 = (param_1 >> 0x10);
  if i_var3.field_0x10 == 0x1 {
    u_stack10 = set_err_mode_1010_8b14
                                 (param_1, &i_var3.field_0x12, param_3);
    u_stack10._2_2_ = (u_stack10 >> 0x10);
    if (i_var3.field_0x12 == u_stack10) && (i_var3.field_0x14 == u_stack10._2_2_) {
      msg_box_op_1010_8bb4(u_var5, u_var6, u_stack10, unaff_cs, param_3);
      return None;
    }
    pu_var1 = &local_2e;
    struct_op_1008_48fe(CONCAT22(param_3, pu_var1), 0x1, u_stack10, u_stack10._2_2_
                       );
    mem_op_1000_179c(0x1e, (u_stack10 >> 0x10), 0x1000);
    u_var3 = (u_stack10 >> 0x10) | pu_var1;
    if u_var3 == 0x0 {
      pu_var2 = 0x0;
      u_var3 = 0x0;
    }
    else {
      pu_var2 = &local_2e;
      struct_op_1008_3f92((u_stack10 & 0xffff0000 | ZEXT24(pu_var1)),
                          CONCAT22(param_3, pu_var2));
    }
    pa_stack6 = CONCAT22(u_var3, pu_var2);
    close_file_1008_496c(&local_2e,param_3);
    local_2e = pa_stack6;
  }
  else {
    if (param_2 * 0x10 + 0x10) == 0x2 {
      pass1_1010_878c(param_1, (param_2 * 0x10 + 0x16), unaff_cs);
      if (u_var5 + 0x67c) == 0x0 {
        return None;
      }
      i_var2 = (param_2 * 0x10);
      pass1_1008_6562((u_var5 + 0x67c),
                      CONCAT22(i_var2.field_0x1c, i_var2.field_0x1e), i_var2.field_0x1a,
                      in_ax, in_dx);
      local_2e = CONCAT22(in_dx, in_ax);
    }
    else {
      i_var4 = param_2 * 0x10;
      if (i_var4 + 0x10) == 0x3 {
        local_2e = 
                   set_err_mode_1010_8b14(param_1, (i_var4 + 0x12), param_3);
        if ((i_var4 + 0x12) == local_2e) &&
           ((i_var4 + 0x14) == (local_2e >> 0x10)) {
          msg_box_op_1010_8bb4(u_var5, u_var6, local_2e, unaff_cs, param_3);
          local_2e = local_2e;
        }
      }
      else {
        local_2e = pa_stack6;
        if (param_2 * 0x10 + 0x10) == 0x4 {
          local_2e = 
                     set_err_mode_1010_8b14
                               (param_1,(param_2 * 0x10 + 0x12),param_3);
        }
      }
    }
  }
  pa_stack6 = local_2e;
  return Some(pa_stack6);
}


pub fn write_to_file_1010_ed58(param_1: u32,param_2: u32,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let BVar3: bool;
  let iVar4: i16;
  let puVar5: u32;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let local_22: u32;
  let uStack30: u16;
  let local_12: [u32;0x2];
  let local_a: u32;
  let iStack4: i16;
  
  BVar3 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar3 != 0x0) {
   // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    local_12[0] = (iVar6 + 0x16);
    uVar8 = param_2;
   // uVar9 = (param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c
                      (uVar8,uVar9,local_12,param_3,0x4,0x1008);
    if (BVar3 != 0x0) {
      local_a = (iVar6 + 0x1a);
      BVar3 = write_to_file_1008_7e1c
                        (uVar8,uVar9,&local_a,param_3,0x4,0x1008);
      if (BVar3 != 0x0) {
        local_a = (iVar6 + 0x20);
        BVar3 = write_to_file_1008_7e1c
                          (uVar8,uVar9,&local_a,param_3,0x4,0x1008);
        if (BVar3 != 0x0) {
          local_a = (iVar6 + 0x24);
          BVar3 = write_to_file_1008_7e1c
                            (uVar8,uVar9,&local_a,param_3,0x4,0x1008);
          if (BVar3 != 0x0) {
            local_a = local_a & 0xffff0000 | (iVar6 + 0x30);
            BVar3 = write_to_file_1008_7e1c
                              (uVar8,uVar9,&local_a,param_3,0x2,0x1008);
            if (BVar3 != 0x0) {
              local_a = local_a & 0xffff0000 | (iVar6 + 0x32);
              BVar3 = write_to_file_1008_7e1c
                                (uVar8,uVar9,&local_a,param_3,0x2,0x1008);
              if (BVar3 != 0x0) {
                iStack4 = 0x0;
                loop {
                  piVar1 = (iVar6 + 0x30);
                  if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                    return;
                  }
                  uVar2 = (iVar6 + 0x2e);
                  puVar5 = ((iVar6 + 0x2c) + iStack4 * 0x6);
                  local_22 = *puVar5;
                  uStack30 = (puVar5 + 0x1);
                  local_12[0] = local_12[0] & 0xffff0000 | ZEXT24(&local_22);
                  iVar4 = write_to_file_1008_7b4c
                                    (param_2,CONCAT22(param_3,&local_22),0x1008,param_3);
                  if (iVar4 == 0x0) { break; }
                  iStack4 += 0x1;
                }
              }
            }
          }
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}
