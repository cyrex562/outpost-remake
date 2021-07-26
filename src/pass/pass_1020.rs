
astruct_18 *  pass1_1020_0d82(param_1: &mut Struct18,param_2: u8)

{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  param_1.field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



use std::default::default;

use crate::cleanup::{cleanup_menu_ui_op_1020_795c, cleanup_ui_op_1020_1038, destroy_icon_1020_2c88, destroy_icon_1020_6bd2, destroy_win_1008_628e, destroy_win_1040_8212, destroy_window_1020_3b3e};
use crate::defines::{Struct110, Struct18, Struct57, Struct76, Struct99, U32Ptr};
use crate::draw::draw_1010::pt_in_rect_1010_4e08;
use crate::draw::draw_1018::pt_in_rect_1018_1bda;
use crate::draw::draw_1020::{draw_op_1020_15de, draw_op_1020_30be, draw_op_1020_7cc8, draw_op_1020_9364, draw_polygon_1020_2474, invalidate_rect_1020_735a, invalidate_rect_1020_8d90, mixed_draw_op_1020_3fa0, palette_op_1020_92c4, pt_in_rect_op_1020_58ce};
use crate::file::file_1008::{close_file_1008_496c, read_file_1008_7dee, write_to_file_1008_7cac};
use crate::file::file_1010::unk_io_op_1010_830a;
use crate::file::file_1028::file_1028_b81a;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1030_835a;
use crate::init::init_globals_1020_96d4;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::misc::empty_1008_8fc4;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_07fc, pass1_1000_093a, pass1_1000_0ed4, pass1_1000_4906, pass1_1000_49c6, pass1_1000_4aea};
use crate::pass::pass_1008::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_3f32, pass1_1008_3f62, pass1_1008_41bc, pass1_1008_4480, pass1_1008_4772, pass1_1008_50c2, pass1_1008_5118, pass1_1008_5134, pass1_1008_5236, pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_612e, pass1_1008_68c6, pass1_1008_68ea, pass1_1008_6c90, pass1_1008_6cec, pass1_1008_8c4e};
use crate::pass::pass_1010::{pass1_1010_043a, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_4df0, pass1_1010_4f20, pass1_1010_4f48, pass1_1010_65d0, pass1_1010_9674, pass1_1010_96a8, pass1_1010_96c2, pass1_1010_988c, pass1_1010_ec68};
use crate::pass::pass_1018::{pass1_1018_04b8, pass1_1018_0902, pass1_1018_0ae8, pass1_1018_1662, pass1_1018_1b02, pass1_1018_2548, pass1_1018_2646, pass1_1018_2678, pass1_1018_268e, pass1_1018_26c2, pass1_1018_26d8, pass1_1018_26f8, pass1_1018_280c, pass1_1018_2862, pass1_1018_2d84, pass1_1018_2e5e};
use crate::pass::pass_1028::{pass1_1028_09b8, pass1_1028_68de, pass1_1028_b39e, pass1_1028_b418, pass1_1028_b46e, pass1_1028_b4f2, pass1_1028_b514, pass1_1028_b58e, pass1_1028_bab6, pass1_1028_bb24, pass1_1028_bc4a, pass1_1028_bdac, pass1_1028_be9e, pass1_1028_bf22, pass1_1028_c1f8, pass1_1028_c23e, pass1_1028_c3aa, pass1_1028_c7b6, pass1_1028_c89c, pass1_1028_c8ee, pass1_1028_c952, pass1_1028_cb04, pass1_1028_ccd0, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass_1030::{fn_ptr_1030_7296, pass1_1030_1d28, pass1_1030_1d58, pass1_1030_1d7c, pass1_1030_25b2, pass1_1030_2fac, pass1_1030_5b1c, pass1_1030_5b3e, pass1_1030_5b5c, pass1_1030_627e, pass1_1030_64ce, pass1_1030_6522, pass1_1030_6fa0, pass1_1030_730a, pass1_1030_7ddc, pass1_1030_8308, pass1_1030_bcae, pass1_1030_bcbc, pass1_1030_bcde, pass1_1030_bd74, pass1_1030_dcc2, pass1_1030_de7c, pass1_1030_dec4, pass1_1030_df0c};
use crate::pass::pass_1038::{pass1_1038_3fb0, pass1_1038_43cc, pass1_1038_4e78, pass1_1038_57dc, pass1_1038_5804};
use crate::resources::find_n_load_rsrc_1010_4e9e;
use crate::string::string_1000::{str_op_1000_3da4, string_1000_3d3e};
use crate::string::string_1010::load_string_1010_847e;
use crate::string::string_1020::string_1020_79b4;
use crate::string::string_1040::string_1040_8520;
use crate::struct_ops::struct_1008::{clear_struct_1008_3e38, pass1_1008_c6ae, pass1_1008_c6fa, struct_op_1008_3f92, struct_op_1008_48fe};
use crate::struct_ops::struct_1020::{set_struct_op_1020_921c, struct_1020_1738, struct_1020_790e, struct_1020_847a};
use crate::struct_ops::struct_1028::{struct_op_1028_87f0, struct_op_1028_8888};
use crate::struct_ops::struct_1030::{struct_1030_e4fa, struct_op_1030_73a8};
use crate::sys_api::unk_win_msg_op_1008_9510;
use crate::ui::ui_1008::post_win_msg_1008_a0e4;
use crate::ui::ui_1020::{get_win_ui_info_op_1020_7a50, ui_op_1020_536e, win_ui_cursor_op_1020_1294, win_ui_op_1020_5de8, win_ui_op_1020_5e76, win_ui_op_1020_737a, window_op_1020_10a0};
use crate::ui::ui_1038::bring_win_to_top_1038_b72e;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SBORROW2, SUB42, ZEXT24};
use crate::win_struct::{HCURSOR16, HDC16, HGDIOBJ16, SEGPTR};
use crate::winapi::{GetDC16, GetStockObject16, SelectObject16, SetCapture16, SetCursor16, SetMapMode16};

pub fn pass1_1020_0dc4(param_1: U32Ptr, param_2: u16, param_3: u32, param_4: u16)
{
  let i_var1: i16;
  let u_var2: u16;
  
  struct_1020_790e(param_1,s_PCPOPMENU_1050_4256,param_2,param_3,param_4);
 // u_var2 = (param_1 >> 0x10);
  i_var1 = param_1;
  (i_var1 + 0xf2) = 0x0;
  (i_var1 + 0xf6) = 0x0;
  (i_var1 + 0xfa) = 0x0;
  *param_1 = 0x1384;
  (i_var1 + 0x2) = 0x1020;
  string_1000_3d3e
            ((param_1 & 0xffff0000 | (i_var1 + 0x5b)),
             s_VrMode_1050_4260);
  (i_var1 + 0xac) = 0x44c00000;
  window_op_1020_10a0(param_1);
  return;
}



pub fn pass1_1020_0e2c(param_1: u32,param_2: u16)
{
  get_win_ui_info_op_1020_7a50(param_1,param_2);
  cleanup_ui_op_1020_1038(param_1);
  return;
}


pub fn
pass1_1020_0e8e(param_1: i16,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: u16,param_7: u16)

{
  let ppcVar1: u32;
  
  win_ui_cursor_op_1020_1294(CONCAT22(param_2,param_1),param_3,param_4,param_6,param_7);
  if (param_5 == 0x0) {
    ppcVar1 = ((param_1 + 0x4) + 0x5c);
    (**ppcVar1)();
  }
  return;
}


pub fn pass1_1020_1022(param_1: u32,param_2: u16)
{
  draw_op_1020_15de((param_1 + 0xf6),param_2);
  return;
}


astruct_3 *  pass1_1020_135e(param_1: &mut Struct3,param_2: u8,param_3: u16)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1020_170a(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  win_ui_op_1020_150e(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_1780(param_1: U32Ptr)
{
  let ppcVar1: u32;
  
  ppcVar1 = (*param_1 + 0x6c);
  (**ppcVar1)();
  destroy_win_1040_8212(param_1,&ctx.PTR_LOOP_1050_1040);
  return;
}


pub fn pass1_1020_1b68(param_1: u32,param_2: u16)
{
  let pu_var1: u32;
  let u_var2: u16;
  let ppc_var3: u32;
  let i_var4: i16;
  let u_var5: u16;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1;
  pu_var1 = (i_var4 + 0x92);
  u_var2 = (i_var4 + 0x94);
  if ((u_var2 | pu_var1) != 0x0) {
    ppc_var3 = *pu_var1;
    (**ppc_var3)();
  }
  (i_var4 + 0x92) = 0x0;
  pass1_1010_4f48((i_var4 + 0x8e),param_2);
  (i_var4 + 0x8e) = 0x0;
  return;
}



pub fn pass1_1020_1bb6(param_1: u32) -> u16

{
  let ppcVar1: u32;
  
  ppcVar1 = ((param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}


pub fn pass1_1020_1d8e(param_1: u32,param_2: u32)
{
  pt_in_rect_1010_4e08
            ((param_1 + 0x8e),param_2,(param_2 >> 0x10),
             0x1010);
  return;
}



pub fn pass1_1020_1da8(param_1: u32,param_2: i16,param_3: u16,param_4: u16) -> u16

{
  let u_var1: u32;
  let i_var2: i16;
  let u_var3: u16;
  
 // u_var3 = (param_1 >> 0x10);
  i_var2 = param_1;
  u_var1 = (i_var2 + 0x8e);
  if ((u_var1 + 0x30) == 0x1) {
    return 0x1;
  }
  u_var1 = (i_var2 + 0x8e);
  if (((u_var1 + 0x30) < 0x3) &&
     (pass1_1010_4df0((i_var2 + 0x8e),param_3,param_4), param_2 == 0x0)) {
    return 0x1;
  }
  return 0x0;
}


astruct_18 *  pass1_1020_1e54(param_1: &mut Struct18,param_2: u8)

{
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1040);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_1eea(param_1: U32Ptr,param_2: u32,param_3: u16,param_4: U32Ptr,param_5: i16,
               param_6: u16)

{
  let ppcVar1: u32;
  let u_var2: u16;
  let iVar3: &mut Struct663;
  let u_var3: u16;
  let puVar4: U32Ptr;
  
 // u_var3 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0x389a;
  iVar3.field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x4 = param_3;
  *param_1 = 0x3ab0;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x6 = 0x0;
  iVar3.field_0xa = param_2;
  *param_1 = 0x2518;
  iVar3.field_0x2 = 0x1020;
  puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x39,param_6,param_4,param_5);
 // u_var2 = (puVar4 >> 0x10);
  &iVar3.field_0x6 = puVar4;
  (&iVar3.field_0x6 + 0x2) = u_var2;
  ppcVar1 = (*iVar3.field_0x6 + 0x4);
  (**ppcVar1)(0x1010,&iVar3.field_0x6,u_var2,0x0,param_1);
  return;
}



pub fn pass1_1020_1f74(param_1: U32Ptr,param_2: u16)
{
  let i_var1: &mut Struct582;
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x2518;
  i_var1.field_0x2 = 0x1020;
  pass1_1010_1ea6(i_var1.field_0x6,param_1 & 0xffff | u_var1 << 0x10,param_2)
  ;
  *param_1 = 0x3ab0;
  i_var1.field_0x2 = 0x1008;
  *param_1 = 0x389a;
  i_var1.field_0x2 = 0x1008;
  return;
}


pub fn pass1_1020_2286(param_1: u16,param_2: u16,param_3: &mut i16,param_4: i16)
{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}


pub fn draw_1020_239c(param_1: u32, param_2: &mut i16, param_3: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u32;
  let u_var3: u16;
  let local_a: [u8;6] = [0;6];
  let u_stack4: u16;
  
  if param_2 != 0x0 {
    u_stack4 = ((param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
    pu_var1 = pass1_1008_3e54(CONCAT22(param_3, local_a), 0x0, 0x57, u_stack4);
   // u_var3 = (param_1 >> 0x10);
    u_var2 = pass1_1020_23f2(param_1, u_var3, CONCAT22(param_3, local_a),
                             (pu_var1 >> 0x10), param_3);
    draw_polygon_1020_2474(param_1, u_var3, CONCAT22(u_var2, 0x3), 0x1008);
  }
  return;
}



pub fn
pass1_1020_23f2(param_1: u16,param_2: u16,param_3: U32Ptr,param_4: U32Ptr,
               param_5: u16) -> u32

{
  let pi_var1: U32Ptr;
  let iStack18: i16;
  let local_6: i16;
  let local_4: i16;
  
  pi_var1 = &local_6;
  pass1_1008_3e94(param_3,CONCAT22(param_5,pi_var1),
                  CONCAT22(param_5,&local_4));
  mem_op_1000_179c(0xc,param_4,0x1000);
// TODO: refactor for loop
  // for (iStack18 = 0x0; iStack18 < 0x3; iStack18 += 0x1) {
  //   pi_var1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4270) + local_4;
  //   pi_var1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x4272) + local_6;
  // }
  return CONCAT22(param_4,pi_var1);
}


pub fn pass1_1020_2488(param_1: u32,param_2: u16,param_3: u16)
{
  let u_var1: u32;
  let in_dlg_id_5: u16;
  let u_var2: u16;
  let iVar3: i16;
  let u_var4: u16;
  let iStack12: i16;
  SStack10: SEGPTR;
  
 // u_var4 = (param_1 >> 0x10);
  iVar3 = param_1;
  find_n_load_rsrc_1010_4e9e((iVar3 + 0x6),0x1010);
  if ((param_3 | param_2) != 0x0) {
    SStack10 = param_2;
      // TODO: refactor for loop
    // for (iStack12 = 0x0; iStack12 < 0x9; iStack12 += 0x1) {
    //   u_var1 = (iVar3 + 0x6);
    //   in_dlg_id_5 = pass1_1010_4f20(u_var1,(u_var1 >> 0x10),iStack12)
    //   ;
    //   u_var1 = (iVar3 + 0xa);
    //   set_win_tet_1020_1d2a
    //             (u_var1,(u_var1 >> 0x10),SStack10,param_3,in_dlg_id_5
    //              ,0x1010);
    //   u_var2 = str_op_1000_3da4(CONCAT22(param_3,SStack10));
    //   SStack10 += u_var2 + 0x1;
    // }
  }
  return;
}



pub fn
pass1_1020_24f2(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  pass1_1020_1f74(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(ctx, param_1, 0x1000);
  }
  return param_1;
}


pub fn pass1_1020_2594(param_1: U32Ptr)
{
  let i_var1: &mut Struct583;
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x270c;
  i_var1.field_0x2 = 0x1020;
  i_var1.field_0xe2 = 0x27a8;
  i_var1.field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



pub fn pass1_1020_25c0(param_1: u32,param_2: u16,param_3: u16)
{
  let pi_var1: U32Ptr;
  let ppcVar2: u32;
  let u_var3: u16;
  let u_var4: u16;
  let iVar3: &mut Struct277;
  let u_var5: u16;
  let paVar6: &mut Struct57;
  let puStack6: u32;
  
  paVar6 = CONCAT22(param_3,param_2);
 // u_var5 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (iVar3.field_0xee != 0x0) {
    ppcVar2 = (*iVar3.field_0xee + 0x8);
    paVar6 = (**ppcVar2)();
  }
  if (iVar3.field_0xea == 0x0) {
    iVar3.field_0xea = 0x1;
    mem_op_1000_179c(0x98,(paVar6 >> 0x10),0x1000);
    u_var3 = paVar6;
    u_var4 = (paVar6 >> 0x10) | u_var3;
    if (paVar6 == 0x0) {
      puStack6 = 0x0;
    }
    else {
      pi_var1 = &iVar3.field_0xcc;
      *pi_var1 = *pi_var1 + 0x1;
      struct_1020_1738(paVar6,iVar3.field_0xcc,param_1);
      puStack6 = CONCAT22(u_var4,u_var3);
    }
    ppcVar2 = (*puStack6 + 0x8);
    (**ppcVar2)(0x1000,puStack6,(puStack6 >> 0x10));
  }
  return;
}


pub fn pass1_1020_26a6(param_1: i32)
{
  let pu_var1: u32;
  let u_var2: u16;
  let ppc_var3: u32;
  let u_var4: u16;
  
 // u_var4 = (param_1 >> 0x10);
  pu_var1 = (param_1 + 0xee);
  u_var2 = (param_1 + 0xf0);
  if ((u_var2 | pu_var1) != 0x0) {
    ppc_var3 = *pu_var1;
    (**ppc_var3)();
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 *  pass1_1020_26d8(param_1: &mut Struct18,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_2594(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_27b0(param_1: &mut Struct664,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  let ppcVar1: u32;
  let u_var2: u32;
  let iVar3: i16;
  let extraout_dx: U32Ptr;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  
  set_struct_op_1020_921c(CONCAT22(param_2,param_1),param_3);
  &param_1.field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0x288e;
  param_1.field_0x2 = 0x1020;
  pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2a,param_5,extraout_dx,param_4);
 // u_var4 = (pu_var5 >> 0x10);
  param_1.field_0x14 = pu_var5;
  param_1.field_0x16 = u_var4;
  param_1.field_0x6 = param_1.field_0x14;
  param_1.field_0x8 = u_var4;
  u_var2 = &param_1.field_0x14;
  iVar3 = &param_1.field_0xa;
  ppcVar1 = ((u_var2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1.field_0x12 = iVar3;
  draw_op_1020_9364(CONCAT22(param_2,param_1),0x1010,param_5);
  return;
}



pub fn pass1_1020_2838(param_1: U32Ptr,param_2: u16)
{
  let i_var1: &mut Struct584;
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x288e;
  i_var1.field_0x2 = 0x1020;
  if (i_var1.field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1dda(i_var1.field_0x14);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



astruct_18 * 
pass1_1020_2868(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  pass1_1020_2838(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1020_289a(param_1: U32Ptr,param_2: u16,param_3: u32,param_4: u16)
{
  let i_var1: i16;
  let u_var2: u16;
  
  struct_1020_790e(param_1,s_SCPOPMENU_1050_427c,param_2,param_3,param_4);
 // u_var2 = (param_1 >> 0x10);
  i_var1 = param_1;
  (i_var1 + 0xf2) = 0x0;
  (i_var1 + 0xf6) = 0x0;
  (i_var1 + 0xfa) = 0x0;
  (i_var1 + 0xfc) = 0x0;
  *param_1 = 0x2e4a;
  (i_var1 + 0x2) = 0x1020;
  string_1000_3d3e
            ((param_1 & 0xffff0000 | (i_var1 + 0x5b)),
             s_VrMode_1050_4286);
  (i_var1 + 0xac) = 0x44c00000;
  return;
}



pub fn pass1_1020_28fc(param_1: &mut Struct3,param_2: u16)
{
  param_1.address_offset_field_0x0 = 0x2e4a;
  (param_1 + 0x2) = 0x1020;
  cleanup_menu_ui_op_1020_795c(param_1,param_2);
  return;
}


pub fn pass1_1020_2936()
{
  pass1_1020_79ae();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_294a(param_1: u32,param_2: u32,param_3: u16,param_4: U32Ptr,param_5: i16,
               param_6: u16)

{
  let u_var1: u16;
  let iVar3: &mut Struct665;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  
 // u_var2 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar3.field_0xfc = param_3;
  pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,param_3,param_6,param_4,param_5);
 // u_var1 = (pu_var3 >> 0x10);
  iVar3.field_0xf2 = pu_var3;
  iVar3.field_0xf4 = u_var1;
  iVar3.field_0xe0 = iVar3.field_0xf2;
  iVar3.field_0xe2 = u_var1;
  pass1_1018_0902(&iVar3.field_0xf2,param_2);
  return;
}


pub fn pass1_1020_2a46(param_1: u16,param_2: u16,param_3: i16)
{
  pass1_1018_0ae8((param_1 + 0xf2),0x1);
  pass1_1008_68c6(param_1,param_2,param_3,0x1008);
  return;
}



pub fn pass1_1020_2a6a(param_1: u32,param_2: u16)
{
  get_win_ui_info_op_1020_7a50(param_1,param_2);
  pass1_1018_0ae8((param_1 + 0xf2),0x0);
  destroy_icon_1020_2c88(param_1,0x1018);
  return;
}



pub fn pass1_1020_2a94(param_1: u32,param_2: u32,param_3: u16)
{
  pass1_1018_1662((param_1 + 0xf2),param_2,(param_2 >> 0x10),
                  param_3);
  return;
}


pub fn pass1_1020_2c72(param_1: u32,param_2: u16,param_3: u16)
{
  draw_op_1020_30be((param_1 + 0xf6),param_2,param_3);
  return;
}


astruct_3 *  pass1_1020_2e24(param_1: &mut Struct3,param_2: u8)

{
  let unaff_CS: u16;
  
  pass1_1020_28fc(param_1,unaff_CS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn
pass1_1020_3540(param_1: u16,param_2: u16,param_3: i16,param_4: U32Ptr,param_5: U32Ptr,
               param_6: u16)

{
  let u_var1: u16;
  let i_var2: &mut Struct279;
  let iStack18: i16;
  let iStack12: i16;
  let uStack10: i16;
  let local_6: i16;
  let local_4: i16;
  
  pass1_1008_3e94(param_4,CONCAT22(param_6,&local_6),
                  CONCAT22(param_6,&local_4));
  if (param_3 == 0x0) {
    iStack12 = 0x3;
    iStack10 = 0x42a6;
  }
  else {
    if (param_3 == 0x1) {
      iStack12 = 0x4;
      iStack10 = (s_SITEICON_1050_428d + 0x9);
    }
    else {
      if (param_3 != 0x2) {
        return;
      }
      iStack12 = 0x4;
      iStack10 = 0x42b2;
    }
  }
  u_var1 = iStack12 << 0x2;
  mem_op_1000_179c(u_var1,param_5,0x1000);
// TODO: refactor for loop
  // for (iStack18 = 0x0; iStack18 < iStack12; iStack18 += 0x1) {
  //   i_var2 = (iStack18 * 0x4);
  //   (i_var2 + u_var1) = (i_var2 + iStack10) + local_4;
  //   (i_var2 + u_var1 + 0x2) = (i_var2 + iStack10 + 0x2) + local_6;
  // }
  return;
}


astruct_18 * 
pass1_1020_3616(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  cleanup_win_ui_1020_2fea(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_3898(param_1: &mut Struct30,param_2: u16)
{
  destroy_window_1020_3b3e(param_1,param_2);
  return;
}


pub fn pass1_1020_3bd6(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let i_var1: i16;
  let u_var2: u16;
  let u_var3: u16;
  let u_var4: u32;
  
 // u_var3 = (param_1 >> 0x10);
  u_var2 = param_1;
  mixed_draw_op_1020_3fa0((u_var2 + 0xf6),param_3,param_4);
  if ((u_var2 + 0x100) == 0x0) {
    (u_var2 + 0x100) = 0x1;
    u_var4 = (u_var2 + 0xfa);
    if ((u_var4 + 0x56) == 0x0) {
      i_var1 = 0x5;
    }
    else {
      i_var1 = 0x8;
    }
    u_var4 = pass1_1038_af40(ctx.PTR_LOOP_1050_5b7c,(u_var2 + 0x8),i_var1,param_2,
                            u_var2,&ctx.PTR_LOOP_1050_1038,param_4);
    (u_var2 + 0x10e) = u_var4;
    (u_var2 + 0x110) = (u_var4 >> 0x10);
  }
  return;
}



pub fn pass1_1020_3c32(param_1: i16,param_2: u16,param_3: u16,param_4: u16)
{
  let cVar1: u8;
  let i_var2: i16;
  
  if (param_3 == 0xf5) {
    i_var2 = 0x1;
//LAB_1020_3c52:
    pass1_1018_1b02(param_4,(param_1 + 0xfa),i_var2);
    return;
  }
  if ((param_3 < 0xf6) && (cVar1 = param_3, cVar1 != '\0')) {
    if (cVar1 == '\x01' || cVar1 == '\x02') {
      return;
    }
    if (cVar1 == -0xc) {
      i_var2 = 0x0;
//       TODO: goto LAB_1020_3c52;
    }
  }
  pass1_1020_3c32(param_1,param_2,param_3,param_4);
  return;
}



pub fn pass1_1020_3c74(param_1: u16,param_2: u32,param_3: u16,param_4: u16)
{
  pass1_1020_3c8c(CONCAT22(param_2,param_1),CONCAT22(param_3,(param_2 >> 0x10)),
                  param_4);
  return;
}



pub fn pass1_1020_3c8c(param_1: u32,param_2: u32,param_3: u16)
{
  pt_in_rect_1018_1bda
            ((param_1 + 0xfa),param_2,(param_2 >> 0x10),
             param_3);
  return;
}



astruct_3 * 
pass1_1020_3ca6(param_1: &mut Struct3,param_2: u8,param_3: u16)

{
  let u_var1: u32;
  let puStack10: U32Ptr;
  
  u_var1 = param_1 & 0xffff0000;
  param_1 = (u_var1 | param_1 - 0xf2);
  param_1._2_2_ = (u_var1 >> 0x10);
  if (param_1 == 0x0) {
    param_1._0_2_ = 0x0;
    param_1._2_2_ = 0x0;
  }
  puStack10 = CONCAT22(param_1._2_2_,param_1);
  *puStack10 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
// 
// WARNING: Variable defined which should be unmapped: param_16
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_3d08(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               ,param_6: u16,param_7: u16,param_8: u16,param_9: u16,param_10: u8
               ,param_11: u8,param_12: u8,param_13: u8,param_14: u8,param_1: u325
               ,param_16: u16,param_17: u16,param_18: u16,param_19: u16)

{
  let mut pcVar1: String; 
  let pbVar2: U32Ptr;
  let bVar3: bool;
  let bVar4: bool;
  let ppcVar5: u32;
  let puVar6: U32Ptr;
  let uVar7: u32;
  let puVar8: u32;
  let uVar9: u32;
  let bVar10: u8;
  let bVar12: u8;
  let uVar13: u16;
  let bVar18: u8;
  let cVar19: u8;
  let HVar14: HDC16;
  int16_t iVar15;
  let HVar16: HGDIOBJ16;
  let puVar17: U32Ptr;
  let uVar20: u16;
  let uVar21: u16;
  let bVar22: u8;
  let bVar23: u8;
  let pu_var24: U32Ptr;
  let bVar25: u8;
  let iVar26: i16;
  let mut pcVar27: String; 
  let mut pcVar28: String; 
  let uVar29: u16;
  let uVar30: u16;
  let bVar31: bool;
  let bVar32: bool;
  let pu_var33: U32Ptr;
  let uStack8: u16;
  let uStack6: u16;
  let pcStack4: u32;
  let bVar11: u8;
  
  uVar9 = CONCAT22(param_19,param_18);
  bVar22 = param_2 + (param_1 >> 0x8) + param_10;
  *param_6 = 0x3c;
  pu_var24 = CONCAT11((param_2 >> 0x8) + '<' +
                              ((param_3 + param_5) < 0x20),bVar22);
  pcStack4 = switchD_1008:1091::caseD_a7;
  uVar13 = 0x203d;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 & bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  bVar10 = (param_6 + 0x2);
  bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
  bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  bVar10 = 0x9 < bVar11 | bVar12;
  uVar20 = CONCAT11((param_6 + 0x2 >> 0x8) + bVar12 + bVar10,
                    bVar11 + bVar10 * '\x06') & 0xff0f;
  pcVar28 = CONCAT11(0x79,param_5 + -0x37);
  loop {
    pcVar27 = pcVar28;
    pbVar2 = (param_3 + uVar13);
    bVar23 = pu_var24;
    *pbVar2 = *pbVar2 | bVar23;
    bVar12 = (uVar20 - 0x1);
    bVar3 = 0x9 < (bVar12 & 0xf);
    bVar22 = bVar3 | bVar10;
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar23;
    bVar4 = 0x9 < (bVar12 + bVar22 * '\x06' & 0xf);
    bVar18 = (uVar20 - 0x1 >> 0x8) + bVar22 + (bVar4 | bVar22);
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar23;
    uVar20 = 0x0;
    bVar31 = &pcStack4 < (param_3 + uVar13);
    pbVar2 = (param_3 + uVar13 + 0x896);
    bVar25 = param_3;
    bVar32 = CARRY1(*pbVar2,bVar25) || CARRY1(*pbVar2 + bVar25,bVar31);
    *pbVar2 = *pbVar2 + bVar25 + bVar31;
    pbVar2 = (param_3 + uVar13 + 0x2038);
    bVar12 = *pbVar2;
    bVar11 = *pbVar2;
    *pbVar2 = bVar11 + bVar25 + bVar32;
    pcVar1 = (param_4 + uVar13);
    *pcVar1 = *pcVar1 + (pu_var24 >> 0x8) +
              (CARRY1(bVar12,bVar25) || CARRY1(bVar11 + bVar25,bVar32));
    pcVar1 = (param_3 + uVar13 + -0x64);
    *pcVar1 = *pcVar1 + bVar18 + '\x01';
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar23;
    pcVar1 = pcVar27;
    pcVar28 = pcVar27 + 0x1;
    bVar31 = *pcVar1 != '\0';
    if (-*pcVar1 < '\0') {
      pcVar1 = (param_4 + 0x37);
      *pcVar1 = *pcVar1 + bVar25 + bVar31;
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar23;
      bVar31 = SBORROW2(uVar13,0x1);
      uVar13 -= 0x1;
      uStack6 = (param_14 & 0x1) * 0x4000 | bVar31 * 0x800 |
                (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
                (uVar13 < 0x0) * 0x80 | (uVar13 == 0x0) * 0x40 |
                (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                ((POPCOUNT(uVar13 & 0xff) & 0x1) == 0x0) * 0x4;
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar23;
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar23;
      bVar31 = SBORROW2(pu_var24,0x1);
      pu_var24 = pu_var24 + -0x1;
      uStack8 = (param_14 & 0x1) * 0x4000 | bVar31 * 0x800 |
                (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
                (pu_var24 < 0x0) * 0x80 | (pu_var24 == 0x0) * 0x40
                | (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                ((POPCOUNT(pu_var24 & 0xff) & 0x1) == 0x0) * 0x4;
      pbVar2 = (param_3 + uVar13);
      bVar12 = pu_var24;
      *pbVar2 = *pbVar2 | bVar12;
      if (*pbVar2 == 0x0) {
        pbVar2 = (param_3 + uVar13);
        *pbVar2 = *pbVar2 & bVar12;
code_r0x10203d96:
        pbVar2 = (param_3 + uVar13);
        *pbVar2 = *pbVar2 | pu_var24;
        pbVar2 = (param_3 + uVar13);
        *pbVar2 = *pbVar2 & pu_var24;
        pcVar28 = pcVar27 + 0x2;
        uVar21 = pu_var24 & 0xff;
        pu_var24 = (uVar21 | ((pu_var24 >> 0x8) * '\x02' +
                                                 (uVar20 < 0x20)) << 0x8);
        pbVar2 = (param_3 + uVar13 + 0x1);
        *pbVar2 = *pbVar2 & uVar21;
        param_4 = &stack0xfff6;
        param_16 = pcStack4;
       // param_17 = (pcStack4 >> 0x10);
        uVar9 = param_15;
code_r0x10203db1:
        get_sys_metrics_1020_7c1a(CONCAT22(param_17,param_16),uVar9,param_8);
        puVar6 = (param_4 + 0x6);
       // uVar29 = (puVar6 >> 0x10);
        (puVar6 + 0x14) = 0x0;
        *puVar6 = 0x408a;
        (puVar6 + 0x2) = 0x1020;
        pu_var33 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x6,param_9,pu_var24,pcVar28);
       // uVar29 = (pu_var33 >> 0x10);
        uVar7 = (param_4 + 0x6);
       // uVar30 = (uVar7 >> 0x10);
        iVar26 = uVar7;
        (iVar26 + 0x14) = pu_var33;
        (iVar26 + 0x16) = uVar29;
        ppcVar5 = ((iVar26 + 0x14) + 0x4);
        (**ppcVar5)(0x1010,(iVar26 + 0x14),uVar29,0x0,iVar26,uVar30);
        HVar14 = GetDC16(0x1010);
        *(param_4 + -0x2) = HVar14;
        iVar15 = SetMapMode16(ctx.s_tile2_bmp_1050_1538,0x1);
        uVar7 = (param_4 + 0x6);
        (uVar7 + 0x1e) = iVar15;
        uVar29 = (param_4 + -0x2);
        HVar16 = GetStockObject16(s_tile2_bmp_1050_1538);
        HVar16 = SelectObject16(ctx.s_tile2_bmp_1050_1538,HVar16);
        uVar7 = (param_4 + 0x6);
        (uVar7 + 0x18) = HVar16;
        uVar30 = (param_4 + -0x2);
        HVar16 = GetStockObject16(s_tile2_bmp_1050_1538);
        HVar16 = SelectObject16(ctx.s_tile2_bmp_1050_1538,HVar16);
        uVar7 = (param_4 + 0x6);
        (uVar7 + 0x1a) = HVar16;
        uVar7 = (param_4 + 0x6);
        uVar7 = (uVar7 + 0x14);
        (param_4 + -0x6) = (uVar7 + 0x24);
        puVar17 = (param_4 + -0x2);
        puVar8 = (param_4 + -0x6);
        ppcVar5 = (*puVar8 + 0x8);
        (**ppcVar5)(ctx.s_tile2_bmp_1050_1538,puVar8,(puVar8 >> 0x10),
                    puVar17,param_9,uVar30,uVar29);
        uVar7 = (param_4 + 0x6);
       // uVar30 = (uVar7 >> 0x10);
        iVar26 = uVar7;
        (iVar26 + 0x1c) = puVar17;
        uVar29 = (param_4 + -0x2);
        (param_4 + -0x14) = uVar29;
        uVar7 = (iVar26 + 0x14);
        (uVar7 + 0x4c) = uVar29;
        return;
      }
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 & bVar12;
      pcVar1 = (param_4 + uVar13 + 0x20);
      bVar11 = param_1 & 0x1f;
      cVar19 = *pcVar1;
      *pcVar1 = *pcVar1 >> bVar11;
      pcVar1 = (param_4 + uVar13 + 0x6a);
      *pcVar1 = *pcVar1 + param_1 +
                ((param_1 & 0x1f) != 0x0 && (cVar19 >> bVar11 - 0x1 & 0x1) != 0x0);
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar12;
      param_8 = 0x1020;
      uVar20 = (param_3 + uVar13) * 0x10;
      pbVar2 = (pcVar28 + param_4 + 0x8);
      bVar12 = (uVar20 >> 0x8);
      uVar21 = uVar20 & 0xff | (bVar12 + *pbVar2) << 0x8;
      pcVar1 = (param_4 + uVar13 + 0x37);
      *pcVar1 = *pcVar1 + (param_3 >> 0x8) + CARRY1(bVar12,*pbVar2);
    }
    else {
      pcVar1 = (param_4 + uVar13);
      *pcVar1 = *pcVar1 + bVar31;
      uVar21 = (param_3 + uVar13) * 0x10;
      if ((POPCOUNT(*pcVar1) & 0x1) == 0x0) {
          // goto
          // code_r0x10203db1;
      }
    }
    pbVar2 = (param_3 + uVar13);
    bVar11 = pu_var24;
    *pbVar2 = *pbVar2 | bVar11;
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_16 = (param_14 & 0x1) * 0x4000 | (param_13 & 0x1) * 0x200 |
               (param_12 & 0x1) * 0x100 | (*pbVar2 < '\0') * 0x80 |
               (*pbVar2 == 0x0) * 0x40 |
               (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
               ((POPCOUNT(*pbVar2) & 0x1) == 0x0) * 0x4;
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar11;
    bVar12 = in(0x79);
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 & bVar11;
    if (-0x1 < *pbVar2) {
      uVar9 = CONCAT22(param_19,param_18);
      if ((bVar18 | (param_4 - 0x1)) == 0x0) {
        param_16 = param_7;
        uVar9 = CONCAT22(param_19,param_18);
      }
//       TODO: goto code_r0x10203db1;
    }
    pbVar2 = (param_4 + 0x89c);
    bVar31 = CARRY1(*pbVar2,bVar12);
    *pbVar2 = *pbVar2 + bVar12;
    bVar23 = bVar18 + bVar12;
    cVar19 = bVar23 + bVar31;
    uVar20 = CONCAT11(cVar19,bVar12);
    pcStack4._0_3_ =
         CONCAT21((param_14 & 0x1) * 0x4000 |
                  (SCARRY1(bVar18,bVar12) != SCARRY1(bVar23,bVar31)) * 0x800 |
                  (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
                  (cVar19 < '\0') * 0x80 | (cVar19 == '\0') * 0x40 |
                  (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                  ((POPCOUNT(cVar19) & 0x1) == 0x0) * 0x4 |
                  (CARRY1(bVar18,bVar12) || CARRY1(bVar23,bVar31)),pcStack4._0_1_);
    pcStack4 = (pcStack4 & 0xff000000 | pcStack4);
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_1 = uVar21 - 0x1;
    bVar10 = bVar4 | bVar22;
    if (param_1 == 0x0 || *pbVar2 == 0x0) {
        // goto
        // code_r0x10203d96;
    }
  }
}


pub fn pass1_1020_4092(param_1: U32Ptr) -> u16

{
  let i_var1: i16;
  let u_var2: u16;
  
  clear_struct_1008_3e38(param_1);
 // u_var2 = (param_1 >> 0x10);
  i_var1 = param_1;
  (i_var1 + 0x6) = 0x0;
  (i_var1 + 0x8) = 0x0;
  (i_var1 + 0xa) = 0x1;
  (i_var1 + 0xc) = 0x0;
  (i_var1 + 0xe) = 0x0;
  clear_struct_1008_3e38((param_1 & 0xffff0000 | (i_var1 + 0x10)));
  return param_1;
}


pub fn
pass1_1020_434c(param_1: i16,param_2: u16,param_3: U32Ptr,param_4: u16,param_5: u16,
               param_6: i16,param_7: u16,param_8: u16)

{
  if (param_6 == 0x1) {
    pass1_1020_6184(CONCAT22(param_2,param_1),param_5,param_8);
    return;
  }
  if (param_6 == 0x2) {
    ui_op_1020_536e(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),param_5,0x2,
                    param_7);
    return;
  }
  pass1_1008_68ea(param_1,param_2,param_3,param_4,param_5,param_6);
  return;
}


pub fn pass1_1020_44b0(param_1: U32Ptr)
{
  let ppcVar1: u32;
  let i_var2: i16;
  let u_var3: u16;
  
 // u_var3 = (param_1 >> 0x10);
  i_var2 = param_1;
  if ((i_var2 + 0xf6) != 0x0) {
    ppcVar1 = (*param_1 + 0x98);
    (**ppcVar1)();
    (i_var2 + 0x112) = 0x0;
    ppcVar1 = ((i_var2 + 0xf6) + 0x8);
    (**ppcVar1)();
  }
  return;
}


pub fn pass1_1020_51c6(param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u16)
{
  let ppcVar1: u32;
  let i_var2: i16;
  let iVar3: i16;
  let u_var4: u16;
  let u_var5: u16;
  
 // u_var4 = (param_1 >> 0x10);
  iVar3 = param_1;
  i_var2 = (iVar3 + 0xf4);
  u_var5 = param_3;
  if (i_var2 == 0x2) {
    win_ui_op_1020_5e76(param_1 & 0xffff | u_var4 << 0x10,param_2,u_var5);
    return;
  }
  i_var2 += -0x3;
  if (i_var2 != 0x0) {
    pt_in_rect_op_1020_58ce
              (param_1 & 0xffff | u_var4 << 0x10,param_2,u_var5,
               (param_3 >> 0x10),param_4,param_5);
    if (i_var2 == 0x0) {
      ppcVar1 = ((iVar3 + 0x4) + 0x5c);
      (**ppcVar1)(param_4,(iVar3 + 0x4),param_2,param_3);
    }
    return;
  }
  win_ui_op_1020_5de8(param_1 & 0xffff | u_var4 << 0x10,param_2,u_var5,param_5);
  return;
}


pub fn pass1_1020_52de(param_1: i32,param_2: u16)
{
  let pu_var1: u32;
  let u_var2: u16;
  let ppc_var3: u32;
  let i_var4: i16;
  let u_var5: u16;
  let iVar6: i16;
  let uVar7: u16;
  
 // uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pu_var1 = (iVar6 + 0xf6);
  u_var2 = (iVar6 + 0xf8);
  if ((u_var2 | pu_var1) != 0x0) {
    ppc_var3 = *pu_var1;
    (**ppc_var3)();
  }
  (iVar6 + 0xf6) = 0x0;
  if ((iVar6 + 0xfa) != 0x0) {
    if (param_1 == 0x0) {
      i_var4 = 0x0;
      u_var5 = 0x0;
    }
    else {
      i_var4 = iVar6 + 0xe2;
      u_var5 = uVar7;
    }
    pass1_1010_1ea6((iVar6 + 0xfa),CONCAT22(u_var5,i_var4),param_2);
  }
  destroy_win_1008_628e(param_1,0x1008);
  if ((iVar6 + 0xfa) != 0x0) {
    pass1_1010_1dda((iVar6 + 0xfa));
  }
  (iVar6 + 0xfa) = 0x0;
  return;
}

G: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_5d56(param_1: U32Ptr,param_2: u32,param_3: U32Ptr,param_4: i16,param_5: u16) -> u16

{
  let ppcVar1: u32;
  let u_var2: u16;
  let u_var3: u16;
  i16 local_12 [0x2];
  let local_e: i16;
  let local_c: i16;
  i16 local_a [0x2];
  let iStack6: i16;
  
  iStack6 = (param_2 + 0x2e);
  u_var2 = param_1;
 // u_var3 = (param_1 >> 0x10);
  if (iStack6 == 0x47) {
    pass1_1020_61c4(u_var2,u_var3,CONCAT22(param_5,&local_c),
                    CONCAT22(param_5,local_a),param_3,param_4,param_5);
    if (local_a[0] == 0x0) {
        // goto
        // LAB_1020_5d8b;
    }
    if (local_c <= local_a[0]) {
      return 0x1;
    }
  }
  else {
    if (iStack6 != 0x6a) {
      return 0x0;
    }
    pass1_1020_61c4(u_var2,u_var3,CONCAT22(param_5,&local_e),
                    CONCAT22(param_5,local_12),param_3,param_4,param_5);
    if (local_e <= local_12[0]) {
//LAB_1020_5d8b:
      ppcVar1 = (*param_1 + 0x40);
      (**ppcVar1)();
      return 0x1;
    }
  }
  pass1_1038_af40(ctx.PTR_LOOP_1050_5b7c,(u_var2 + 0x8),0x9,param_3,u_var2,
                  &ctx.PTR_LOOP_1050_1038,param_5);
  return 0x1;
}


pub fn pass1_1020_6184(param_1: u32,param_2: u16,param_3: u16)
{
  let HVar1: HCURSOR16;
  let i_var2: i16;
  let u_var3: u16;
  
 // u_var3 = (param_1 >> 0x10);
  i_var2 = param_1;
  if ((i_var2 + 0xf4) == 0x1) {
    HVar1 = SetCursor16(param_3);
    (i_var2 + 0xee) = HVar1;
    (i_var2 + 0x10c) = param_2;
    SetCapture16(s_tile2_bmp_1050_1538);
    (i_var2 + 0xf4) = 0x2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_61c4(param_1: u16,param_2: u16,param_3: u32,param_4: U32Ptr,param_5: U32Ptr
               ,param_6: i16,param_7: u16)

{
  let u_var1: u32;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  
  pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
 // u_var2 = (pu_var3 >> 0x10);
  u_var1 = (pu_var3 + 0x20);
  pass1_1030_8308(ctx.PTR_LOOP_1050_5748,(ctx.PTR_LOOP_1050_5748 >> 0x10)
                  ,param_3,param_4,u_var1,u_var1,u_var2);
  *param_4 = (pu_var3 + 0x1e);
  return;
}



astruct_18 * 
pass1_1020_6208(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  destroy_cursor_1020_42f4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_62e0(param_1: i16,param_2: u16,param_3: u16,param_4: u16)
{
  let pu_var1: u32;
  let ppcVar2: u32;
  let pu_var3: U32Ptr;
  let u_var4: u32;
  let extraout_dx: U32Ptr;
  let pu_var5: U32Ptr;
  let puVar6: U32Ptr;
  let uVar7: u16;
  let extraout_DX_00: U32Ptr;
  let unaff_DI: i16;
  let puVar8: U32Ptr;
  let uVar9: u16;
  let u_var10: u16;
  let iVar11: i16;
  let uVar12: u16;
  
  set_struct_op_1020_921c(CONCAT22(param_2,param_1),param_3);
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x2c) = 0x0;
  CONCAT22(param_2,param_1) = 0x67c2;
  (param_1 + 0x2) = 0x1020;
  puVar6 = extraout_dx;
  pu_var3 = pass1_1000_4906(CONCAT22(param_2,param_1 + 0x18),
                           0x0,0x14);
  mem_op_1000_179c(0x3c,puVar6,0x1000);
  pu_var5 = (puVar6 | pu_var3);
  if (pu_var5 == 0x0) {
    (param_1 + 0x1c) = 0x0;
  }
  else {
    pass1_1020_87c2(CONCAT22(puVar6,pu_var3),param_4,unaff_DI);
    (param_1 + 0x1c) = pu_var3;
    (param_1 + 0x1e) = pu_var5;
  }
  mem_op_1000_179c(0x26,pu_var5,0x1000);
  puVar6 = (pu_var5 | pu_var3);
  if (puVar6 == 0x0) {
    pu_var3 = 0x0;
    puVar6 = 0x0;
  }
  else {
    pass1_1020_8a9c(CONCAT22(pu_var5,pu_var3));
  }
  (param_1 + 0x20) = pu_var3;
  (param_1 + 0x22) = puVar6;
  mem_op_1000_179c(0xbe,puVar6,0x1000);
  pu_var5 = (puVar6 | pu_var3);
  if (pu_var5 == 0x0) {
    pu_var3 = 0x0;
    pu_var5 = 0x0;
  }
  else {
    pass1_1020_8eaa(CONCAT22(puVar6,pu_var3),param_4);
  }
  (param_1 + 0x24) = pu_var3;
  (param_1 + 0x26) = pu_var5;
  mem_op_1000_179c(0x20,pu_var5,0x1000);
  puVar6 = (pu_var5 | pu_var3);
  if (puVar6 == 0x0) {
    pu_var3 = 0x0;
    puVar6 = 0x0;
  }
  else {
    pass1_1020_8360(CONCAT22(pu_var5,pu_var3),param_4);
  }
  (param_1 + 0x28) = pu_var3;
  (param_1 + 0x2a) = puVar6;
  pass1_1020_6746(CONCAT22(param_2,param_1),0x1,0x4);
  puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x29,param_4,puVar6,unaff_DI);
 // uVar7 = (puVar8 >> 0x10);
  (param_1 + 0x14) = puVar8;
  (param_1 + 0x16) = uVar7;
  u_var10 = 0x0;
  uVar9 = (param_1 + 0x14);
  ppcVar2 = ((param_1 + 0x14) + 0x4);
  iVar11 = param_1;
  uVar12 = param_2;
  (**ppcVar2)();
  (param_1 + 0x6) = (param_1 + 0x14);
  u_var4 = (param_1 + 0x14);
  pu_var1 = (u_var4 + 0xa);
  u_var4 = CONCAT22(param_2,param_1 + 0xa);
  ppcVar2 = (*pu_var1 + 0x8);
  (**ppcVar2)(0x1010,pu_var1,(pu_var1 >> 0x10),u_var4,uVar9,uVar7,u_var10,
              iVar11,uVar12);
  (param_1 + 0x12) = u_var4;
  (param_1 + 0x10) = 0x1;
  puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2,param_4,extraout_DX_00,unaff_DI);
  (param_1 + 0x2c) = puVar8;
  (param_1 + 0x2e) = (puVar8 >> 0x10);
  return;
}



pub fn pass1_1020_6466(param_1: U32Ptr,param_2: u16,param_3: u16)
{
  let i_var1: &mut Struct585;
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x67c2;
  i_var1.field_0x2 = 0x1020;
  if (i_var1.field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6(i_var1.field_0x14,param_1 & 0xffff | u_var1 << 0x10,
                    param_3);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



pub fn pass1_1020_6498(param_1: u32,param_2: i16) -> u32

{
  let u_var1: u32;
  let i_var2: i16;
  let u_var3: u16;
  
 // u_var3 = (param_1 >> 0x10);
  if ((param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    u_var1 = (param_1 + 0x18 + param_2 * 0x4);
   // u_var3 = (u_var1 >> 0x10);
    i_var2 = u_var1;
    return CONCAT22((i_var2 + 0xa),(i_var2 + 0x8));
  }
  return 0x0;
}



pub fn pass1_1020_64d4(param_1: u32,param_2: i16) -> u16

{
  let u_var1: u32;
  let u_var2: u16;
  
 // u_var2 = (param_1 >> 0x10);
  if ((param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    u_var1 = (param_1 + 0x18 + param_2 * 0x4);
    return (u_var1 + 0x4);
  }
  return 0x0;
}


pub fn pass1_1020_6746(param_1: u32,param_2: i16,param_3: i16)
{
  let ppcVar1: u32;
  let u_var2: u32;
  let iVar3: i16;
  let u_var4: u16;
  
  if (param_3 != 0x0) {
   // u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x18 + param_3 * 0x4) != 0x0) {
      u_var2 = (iVar3 + 0x18 + param_3 * 0x4);
      (u_var2 + 0x4) = param_2;
      (iVar3 + 0x10) = 0x1;
      if (param_2 == 0x0) {
        ppcVar1 = (
                                  (iVar3 + 0x18 + param_3 * 0x4) + 0x14);
        (**ppcVar1)();
      }
    }
  }
  return;
}



astruct_18 * 
pass1_1020_679c(param_1: &mut Struct18,param_2: u8,param_3: u16,param_4: u16)

{
  pass1_1020_6466(param_1,param_3,param_4);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1020_687c(param_1: u32,param_2: u16,param_3: u16)
{
  let u_var1: u8;
  
  u_var1 = (uchar)param_2;
  get_win_ui_info_op_1020_7a50(param_1,param_3);
  destroy_icon_1020_6bd2(param_1,u_var1,param_3);
  return;
}


pub fn pass1_1020_68de(param_1: u32,param_2: u16)
{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  if ((param_1 + 0xf6) != 0x0) {
    invalidate_rect_1020_735a((param_1 + 0xf6),param_2);
  }
  return;
}


pub fn pass1_1020_6bbc(param_1: u32,param_2: u16,param_3: u16)
{
  win_ui_op_1020_737a((param_1 + 0xf6),param_2,param_3);
  return;
}


pub fn
pass1_1020_6e52(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: u16,
               param_6: i16)

{
  let u_var1: u16;
  let mut pcVar2: String; 
  
  pass1_1018_2e5e(param_1,param_2,param_3,(param_4 + 0xf2));
  u_var1 = param_3 | param_2;
  if (u_var1 == 0x0) {
    pcVar2 = load_string_1010_847e
                       (ctx.PTR_LOOP_1050_14cc,
                        (ctx.PTR_LOOP_1050_14cc >> 0x10),0x1010);
  }
  else {
    pass1_1018_2d84(param_2,(param_4 + 0xf2));
    pcVar2 = CONCAT22(u_var1,param_2);
  }
  string_1020_79b4(param_1,CONCAT22(param_5,param_4),param_6,pcVar2);
  return;
}


astruct_3 *  pass1_1020_70c0(param_1: &mut Struct3,param_2: u8,param_3: u16)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1020_7526(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  palette_op_1020_7270(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_75c4(param_1: U32Ptr)
{
  let i_var1: &mut Struct586;
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x7780;
  i_var1.field_0x2 = 0x1020;
  i_var1.field_0xe2 = 0x781c;
  i_var1.field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}


pub fn pass1_1020_770e(param_1: u32)
{
  let pu_var1: u32;
  let u_var2: u16;
  let ppc_var3: u32;
  let i_var4: i16;
  let u_var5: u16;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1;
  pu_var1 = (i_var4 + 0xee);
  u_var2 = (i_var4 + 0xf0);
  if ((u_var2 | pu_var1) != 0x0) {
    ppc_var3 = *pu_var1;
    (**ppc_var3)();
  }
  (i_var4 + 0xee) = 0x0;
  destroy_win_1008_628e(param_1 & 0xffff | u_var5 << 0x10,0x1008);
  return;
}



astruct_18 *  pass1_1020_774c(param_1: &mut Struct18,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_75c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_7824(param_1: &mut Struct666,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  let ppcVar1: u32;
  let u_var2: u32;
  let iVar3: i16;
  let extraout_dx: U32Ptr;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  
  set_struct_op_1020_921c(CONCAT22(param_2,param_1),param_3);
  &param_1.field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0x7902;
  param_1.field_0x2 = 0x1020;
  pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x25,param_5,extraout_dx,param_4);
 // u_var4 = (pu_var5 >> 0x10);
  param_1.field_0x14 = pu_var5;
  param_1.field_0x16 = u_var4;
  param_1.field_0x6 = param_1.field_0x14;
  param_1.field_0x8 = u_var4;
  u_var2 = &param_1.field_0x14;
  iVar3 = &param_1.field_0xa;
  ppcVar1 = ((u_var2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1.field_0x12 = iVar3;
  draw_op_1020_9364(CONCAT22(param_2,param_1),0x1010,param_5);
  return;
}



pub fn pass1_1020_78ac(param_1: U32Ptr,param_2: u16)
{
  let i_var1: &mut Struct587;
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x7902;
  i_var1.field_0x2 = 0x1020;
  if (i_var1.field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1dda(i_var1.field_0x14);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



astruct_18 * 
pass1_1020_78dc(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  pass1_1020_78ac(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_79ae() -> u16

{
  return 0x1;
}


pub fn pass1_1020_79e4(param_1: u32,param_2: u16,param_3: u16)
{
  draw_op_1020_7cc8((param_1 + 0xe8),param_2,param_3);
  return;
}


astruct_3 *  pass1_1020_7b60(param_1: &mut Struct3,param_2: u8,param_3: u16)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 *  pass1_1020_7f38(param_1: &mut Struct18,param_2: u8)

{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  param_1.field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_808e(param_1: U32Ptr)
{
  let pu_var1: U32Ptr;
  let u_var2: u16;
  let iVar3: &mut Struct574;
  let u_var3: u16;
  let puStack6: U32Ptr;
  
 // u_var3 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0x82bc;
  iVar3.field_0x2 = 0x1020;
  iVar3.field_0xe2 = 0x8358;
  iVar3.field_0xe4 = 0x1020;
  if (param_1 == 0x0) {
    pu_var1 = 0x0;
    u_var2 = 0x0;
  }
  else {
    pu_var1 = &iVar3.field_0xe2;
    u_var2 = u_var3;
  }
  puStack6 = CONCAT22(u_var2,pu_var1);
  *puStack6 = 0x389a;
  pu_var1[0x1] = 0x1008;
  pass1_1008_57c4(
                  (param_1 & 0xffff0000 | &iVar3.field_0xd2));
  *param_1 = 0x380a;
  iVar3.field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar3.field_0x2 = 0x1008;
  return;
}



pub fn pass1_1020_8106(param_1: u32)
{
  let ppcVar1: u32;
  
  ppcVar1 = ((param_1 + 0x4) + 0x60);
  (**ppcVar1)();
  return;
}


astruct_18 *  pass1_1020_8288(param_1: &mut Struct18,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_808e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_8360(param_1: U32Ptr,param_2: u16)
{
  let u_var1: u32;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let u_var4: u16;
  let i_var4: &mut Struct667;
  
  i_var4 = param_1;
 // u_var4 = (param_1 >> 0x10);
  struct_1020_847a(param_1,0x1,param_2);
  pu_var3 = clear_struct_1008_3e38(
                           (param_1 & 0xffff0000 | &i_var4.field_0x16)
                          );
  &i_var4.field_0x1c = 0x0;
  *param_1 = 0x8462;
  i_var4.field_0x2 = 0x1020;
  pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x29,param_2,
                           (pu_var3 >> 0x10),u_var4);
 // u_var2 = (pu_var3 >> 0x10);
  i_var4.field_0x1c = pu_var3;
  i_var4.field_0x1e = u_var2;
  pass1_1018_26f8(i_var4.field_0x1c,u_var2,

                  (param_1 & 0xffff0000 | &i_var4.field_0x16));
  u_var1 = &i_var4.field_0x1c;
  pass1_1020_8712(param_1 & 0xffff | u_var4 << 0x10,i_var4.field_0x8,
                  (u_var1 + 0x2a),

                  (param_1 & 0xffff0000 | &i_var4.field_0x16));
  return;
}



pub fn pass1_1020_83f8(param_1: u32,param_2: u16)
{
  let u_var1: u32;
  let u_var2: u32;
  let iVar3: i16;
  let u_var4: u16;
  
 // u_var4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x4) != 0x0) {
    u_var1 = (iVar3 + 0x1c);
    u_var2 = (iVar3 + 0x1c);
    pass1_1008_4480((u_var1 + 0xa),
                    (param_1 & 0xffff0000 | (iVar3 + 0x16)),
                    (u_var2 + 0x2a),param_2);
  }
  return;
}



astruct_18 *  pass1_1020_843c(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_8556(param_1: U32Ptr)
{
  let pi_var1: U32Ptr;
  let u_var2: u16;
  let paVar3: &mut Struct18;
  let iVar5: &mut Struct588;
  let i_var4: &mut Struct589;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let iStack12: i16;
  
 // uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  *param_1 = 0x87aa;
  iVar5.field_0x2 = 0x1020;
  fn_ptr_1000_17ce(ctx, iVar5.field_0x8, 0x1000);
  if (((&iVar5.field_0xc + 0x2) | &iVar5.field_0xc) != 0x0) {
    iStack12 = 0x0;
    loop {
      pi_var1 = &iVar5.field_0x6;
      if (*pi_var1 == iStack12 || *pi_var1 < iStack12) { break; }
      iVar6 = iStack12 * 0x4;
      paVar3 = iVar5.field_0xc;
     // uVar8 = (paVar3 >> 0x10);
      i_var4 = paVar3;
      if ((i_var4 + iVar6) != 0x0) {
        paVar3 = (i_var4 + iVar6);
        u_var2 = (i_var4 + iVar6 + 0x2);
        if ((u_var2 | paVar3) != 0x0) {
          pass1_1008_5118(paVar3 & 0xffff | u_var2 << 0x10);
          fn_ptr_1000_17ce(ctx, paVar3, 0x1000);
        }
      }
      iStack12 += 0x1;
    }
    fn_ptr_1000_17ce(ctx, iVar5.field_0xc, 0x1000);
  }
  *param_1 = 0x389a;
  iVar5.field_0x2 = 0x1008;
  return;
}



pub fn pass1_1020_85f6(param_1: u32)
{
  let pi_var1: U32Ptr;
  let u_var2: u16;
  let paVar3: &mut Struct18;
  let u_var4: u32;
  let iVar5: i16;
  let iVar6: &mut Struct590;
  let u_var6: u16;
  let uVar7: u16;
  let i_stack4: i16;
  
  i_stack4 = 0x0;
  loop {
   // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pi_var1 = &iVar6.field_0x6;
    if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) { break; }
    u_var4 = iVar6.field_0xc;
   // u_var6 = (u_var4 >> 0x10);
    iVar5 = u_var4;
    paVar3 = (iVar5 + i_stack4 * 0x4);
    u_var2 = (iVar5 + i_stack4 * 0x4 + 0x2);
    if ((u_var2 | paVar3) != 0x0) {
      pass1_1008_5118(paVar3 & 0xffff | u_var2 << 0x10);
      fn_ptr_1000_17ce(ctx, paVar3, 0x1000);
    }
    u_var4 = iVar6.field_0xc;
    (u_var4 + i_stack4 * 0x4) = 0x0;
    i_stack4 += 0x1;
  }
  return;
}



pub fn pass1_1020_865a(param_1: u32)
{
  let pi_var1: U32Ptr;
  let u_var2: u16;
  let paVar3: &mut Struct18;
  let u_var4: u32;
  let iVar5: i16;
  let iVar7: &mut Struct592;
  let iVar6: &mut Struct591;
  let i_var8: i16;
  let uVar9: u16;
  let u_var10: u16;
  let i_stack4: i16;
  
  i_stack4 = 0x0;
  loop {
   // uVar9 = (param_1 >> 0x10);
    iVar5 = param_1;
    pi_var1 = (iVar5 + 0x6);
    if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) { break; }
    i_var8 = i_stack4 * 0x4;
    u_var4 = (iVar5 + 0xc);
   // u_var10 = (u_var4 >> 0x10);
    iVar7 = u_var4;
    if ((iVar7 + i_var8) != 0x0) {
      pass1_1008_5236((iVar7 + i_var8));
      u_var4 = (iVar5 + 0xc);
     // u_var10 = (u_var4 >> 0x10);
      iVar6 = u_var4;
      paVar3 = (iVar6 + i_var8);
      u_var2 = (iVar6 + i_var8 + 0x2);
      if ((u_var2 | paVar3) != 0x0) {
        pass1_1008_5118(paVar3 & 0xffff | u_var2 << 0x10);
        fn_ptr_1000_17ce(ctx, paVar3, 0x1000);
      }
      u_var4 = (iVar5 + 0xc);
      (u_var4 + i_stack4 * 0x4) = 0x0;
    }
    i_stack4 += 0x1;
  }
  return;
}



pub fn pass1_1020_86d8(param_1: u32)
{
  let pi_var1: U32Ptr;
  let u_var2: u32;
  let iVar3: i16;
  let u_var4: u16;
  let i_stack4: i16;
  
  i_stack4 = 0x0;
  loop {
   // u_var4 = (param_1 >> 0x10);
    pi_var1 = (param_1 + 0x6);
    if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) { break; }
    u_var2 = (param_1 + 0xc);
   // u_var4 = (u_var2 >> 0x10);
    iVar3 = u_var2;
    if ((iVar3 + i_stack4 * 0x4) != 0x0) {
      pass1_1008_5236((iVar3 + i_stack4 * 0x4));
    }
    i_stack4 += 0x1;
  }
  return;
}



pub fn pass1_1020_8712(param_1: u32,param_2: &mut i16,param_3: &mut Struct76,param_4: U32Ptr)
{
  let u_var1: u16;
  let u_var2: u32;
  
  pass1_1008_3f32(param_4,
                  (param_1 & 0xffff0000 | (param_1 + 0x10)));
  u_var2 = pass1_1008_4772(param_3);
 // u_var1 = (u_var2 >> 0x10);
  pass1_1008_3e94(param_4,
                          (param_2 & 0xffff0000 |
                          ZEXT24((param_2 + 0x2))),
                  (param_2 & 0xffff | param_2._2_2_ << 0x10));
  (param_2 + 0x4) = (u_var2 + 0x4) + *param_2;
  (param_2 + 0x6) = (u_var2 + 0x8) + (param_2 + 0x2)
  ;
  return;
}



astruct_18 *  pass1_1020_8784(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_87c2(param_1: U32Ptr,param_2: u16,param_3: i16)
{
  let u_var1: u32;
  let i_var2: &mut Struct281;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let local_12: [u8;8];
  let uStack10: i16;
  let puStack8: U32Ptr;
  let i_stack4: i16;
  
  struct_1020_847a(param_1,0x4,param_2);
  i_stack4 = 0x4;
  i_var2 = param_1;
  i_var2 = &i_var2.field_0x16;
  puStack8 = (param_1 & 0xffff0000 | ZEXT24(i_var2));
  loop {
    clear_struct_1008_3e38(puStack8);
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0x6));
    i_stack4 += -0x1;
    if (i_stack4 != 0x0) == false {break;}
  }
 // u_var2 = (param_1 >> 0x10);
  &i_var2.field_0x2e = 0x0;
  pu_var3 = clear_struct_1008_3e38(
                           (param_1 & 0xffff0000 | &i_var2.field_0x32)
                          );
  i_var2.field_0x38 = 0x0;
  *param_1 = 0x8a84;
  i_var2.field_0x2 = 0x1020;
  pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x29,param_2,
                           (pu_var3 >> 0x10),param_3);
  i_var2.field_0x2e = pu_var3;
  i_var2.field_0x30 = (pu_var3 >> 0x10);
  iStack10 = 0x0;
  loop {
    u_var1 = &i_var2.field_0x2e;
    pass1_1018_26d8(u_var1,(u_var1 >> 0x10),iStack10,

                    (param_1 & 0xffff0000 |
                    (&i_var2.field_0x16 + iStack10 * 0x6)));
    u_var1 = &i_var2.field_0x2e;
    pass1_1020_8712(param_1 & 0xffff | u_var2 << 0x10,
                    CONCAT22(i_var2.field_0xa,i_var2.field_0x8 + iStack10 * 0x8),
                    (u_var1 + 0x2e + iStack10 * 0x4),

                    (param_1 & 0xffff0000 |
                    (&i_var2.field_0x16 + iStack10 * 0x6)));
    iStack10 += 0x1;
    if (iStack10 < 0x4) == false { break; }
  }
  u_var1 = &i_var2.field_0x2e;
  pass1_1018_2548(u_var1,(u_var1 >> 0x10),

                  (param_1 & 0xffff0000 | &i_var2.field_0x32));
  u_var1 = &i_var2.field_0x2e;
  i_var2.field_0x38 = (u_var1 + 0x6e);
  pass1_1020_8712(param_1 & 0xffff | u_var2 << 0x10,
                  CONCAT22(param_2,local_12),i_var2.field_0x38,

                  (param_1 & 0xffff0000 | &i_var2.field_0x32));
  return;
}



pub fn pass1_1020_8908(param_1: u32,param_2: u32,param_3: u16)
{
  let paVar1: &mut Struct76;
  let u_var2: u32;
  let u_var3: u16;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  let puVar6: U32Ptr;
  let uVar7: u16;
  let i_var8: &mut Struct284;
  let i_var9: i16;
  let iVar10: i16;
  let u_var11: u16;
  let uVar12: u16;
  let uVar13: u32;
  paStack28: &mut Struct110;
  let i_stack4: i16;
    // TODO: refactor for loop
  // for (i_stack4 = 0x0; i_var8 = param_1,
  //     u_var11 = (param_1 >> 0x10), i_stack4 < 0x4; i_stack4 += 0x1) {
  //   if (i_var8.field_0x4 == 0x0) {
  //     u_var2 = i_var8.field_0xc;
  //     u_var11 = (u_var2 >> 0x10);
  //     iVar10 = u_var2;
  //     i_var9 = i_stack4 * 0x4;
  //     if (((iVar10 + i_var9 + 0x2) | (iVar10 + i_var9)) != 0x0) {
  //       pass1_1008_5236((iVar10 + i_var9));
  //     }
  //   }
  //   else {
  //     u_var2 = i_var8.field_0x2e;
  //     paVar1 = (u_var2 + 0x2e + i_stack4 * 0x4);
  //     uVar13 = pass1_1008_4772(paVar1);
  //     pu_var5 = (uVar13 >> 0x10);
  //     u_var3 = uVar13;
  //     u_var2 = i_var8.field_0xc;
  //     iVar10 = i_stack4 * 0x4;
  //     if ((u_var2 + iVar10) == 0x0) {
  //       puVar6 = pu_var5;
  //       u_var4 = u_var3;
  //       mem_op_1000_179c(0x14,pu_var5,0x1000);
  //       paStack28 = CONCAT22(puVar6,u_var4);
  //       if ((puVar6 | u_var4) == 0x0) {
  //         u_var2 = i_var8.field_0xc;
  //         (u_var2 + i_stack4 * 0x4) = 0x0;
  //       }
  //       else {
  //         u_var4 = &i_var8.field_0x16 + i_stack4 * 0x6;
  //         uVar7 = u_var11;
  //         pass1_1008_50c2(paStack28,(u_var3 + 0x8),(u_var3 + 0x4),
  //                         (param_1 & 0xffff0000 | u_var4),param_2);
  //         u_var2 = i_var8.field_0xc;
  //         uVar12 = (u_var2 >> 0x10);
  //         i_var9 = u_var2;
  //         (i_var9 + iVar10) = u_var4;
  //         (i_var9 + iVar10 + 0x2) = uVar7;
  //       }
  //       u_var2 = i_var8.field_0xc;
  //       pass1_1008_5134((u_var2 + i_stack4 * 0x4));
  //     }
  //     u_var2 = i_var8.field_0xc;
  //     pass1_1008_5236((u_var2 + i_stack4 * 0x4));
  //     pass1_1008_4480(param_2,
  //                             (param_1 & 0xffff0000 |
  //                             (&i_var8.field_0x16 + i_stack4 * 0x6)),paVar1,
  //                     param_3);
  //   }
  // }
  if (i_var8.field_0x4 != 0x0) {
    pass1_1008_4480(param_2,
                            (param_1 & 0xffff0000 | &i_var8.field_0x32),
                    i_var8.field_0x38,param_3);
  }
  return;
}



astruct_18 *  pass1_1020_8a5e(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_8a9c(param_1: U32Ptr)
{
  let u_var1: u32;
  let u_var2: u32;
  let u_var3: u16;
  let u_var4: u16;
  let unaff_SS: u16;
  let pu_var5: U32Ptr;
  let paVar6: &mut Struct76;
  let paVar7: &mut Struct43;
  let i_var8: i16;
  let uVar9: u16;
  let puStack76: U32Ptr;
  u8 local_48 [0x1e];
  let local_2a: [u8;24];
  let uStack6: u16;
  let uStack4: u16;
  
  i_var8 = param_1;
 // uVar9 = (param_1 >> 0x10);
  struct_1020_847a(param_1,0x2,unaff_SS);
  u_var3 = i_var8 + 0x16;
  clear_struct_1008_3e38((param_1 & 0xffff0000 | u_var3));
  puStack76 = (param_1 & 0xffff0000 | (i_var8 + 0x1c));
  pu_var5 = clear_struct_1008_3e38(
                           (param_1 & 0xffff0000 | (i_var8 + 0x1c)));
  (i_var8 + 0x22) = 0x0;
  *param_1 = 0x8e92;
  (i_var8 + 0x2) = 0x1020;
  pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x29,unaff_SS,
                           (pu_var5 >> 0x10),uVar9);
 // u_var4 = (pu_var5 >> 0x10);
  (i_var8 + 0x22) = pu_var5;
  (i_var8 + 0x24) = u_var4;
  pass1_1018_2678((i_var8 + 0x22),u_var4,
                  (param_1 & 0xffff0000 | u_var3));
  paVar6 = pass1_1018_268e((i_var8 + 0x22));
 // uStack4 = (paVar6 >> 0x10);
  uStack6 = SUB42(paVar6,0x0);
  pass1_1020_8712(param_1 & 0xffff | uVar9 << 0x10,(i_var8 + 0x8),
                  paVar6,(param_1 & 0xffff0000 | u_var3));
  u_var1 = (i_var8 + 0x22);
  pass1_1018_26c2(u_var1,(u_var1 >> 0x10),puStack76);
  paVar7 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc,0x2,unaff_SS);
  struct_op_1008_48fe(
                      CONCAT13((unaff_SS >> 0x8),CONCAT12(unaff_SS,local_2a)),
                      0x1,paVar7,(paVar7 >> 0x10));
  struct_op_1008_3f92(CONCAT22(unaff_SS,local_48),
                      CONCAT22(unaff_SS,local_2a));
  u_var2 = (i_var8 + 0x8);
  pass1_1020_8712(param_1 & 0xffff | uVar9 << 0x10,
                  (u_var2 & 0xffff0000 | (u_var2 + 0x8)),
                  CONCAT22(unaff_SS,local_48),puStack76);
  pass1_1008_41bc(CONCAT22(unaff_SS,local_48));
  close_file_1008_496c(local_2a,unaff_SS);
  return;
}



pub fn pass1_1020_8bae(param_1: U32Ptr)
{
  *param_1 = 0x8e92;
  (param_1 + 0x2) = 0x1020;
  pass1_1020_8556(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_8bcc(param_1: u32,param_2: u16)
{
  let u_var1: u32;
  let pu_var2: u32;
  let u_var3: u16;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  let puVar6: U32Ptr;
  let uVar7: u16;
  let extraout_dx: u16;
  let i_var9: &mut Struct285;
  let iVar10: &mut Struct286;
  let uVar8: u16;
  let uVar9: u16;
  let paVar10: &mut Struct43;
  u8 local_58 [0x1e];
  let local_3a: [u8;26];
  let uStack20: u32;
  let uStack12: u16;
  let paStack10: &mut Struct76;
  let uStack6: u32;
  
 // uVar8 = (param_1 >> 0x10);
  i_var9 = param_1;
  if (i_var9.field_0x4 != 0x0) {
    u_var1 = i_var9.field_0x22;
    uStack6 = (u_var1 + 0xa);
    paStack10 = pass1_1018_268e(i_var9.field_0x22);
   // uVar9 = (paStack10 >> 0x10);
    u_var1 = i_var9.field_0x22;
    uStack12 = (u_var1 + 0x16);
    if (*i_var9.field_0xc == 0x0) {
      uStack20 = pass1_1008_4772(paStack10);
     // puVar6 = (uStack20 >> 0x10);
      u_var3 = uStack20;
      mem_op_1000_179c(0x14,puVar6,0x1000);
      uVar7 = puVar6 | u_var3;
      if (uVar7 == 0x0) {
        *i_var9.field_0xc = 0x0;
      }
      else {
        pu_var5 = (param_1 & 0xffff0000 | &i_var9.field_0x16);
       // uVar9 = (uStack20 >> 0x10);
        pass1_1008_50c2(CONCAT22(puVar6,u_var3),
                        (uStack20 + 0x8),(uStack20 + 0x4),
                        pu_var5,uStack6);
        pu_var2 = i_var9.field_0xc;
        pu_var2 = pu_var5;
        (pu_var2 + 0x2) = uVar7;
      }
      pass1_1008_5134(*i_var9.field_0xc);
      paVar10 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc,0x2,param_2);
      struct_op_1008_48fe(CONCAT22(param_2,local_3a),0x1,paVar10,
                          (paVar10 >> 0x10));
      struct_op_1008_3f92(CONCAT22(param_2,local_58),
                          CONCAT22(param_2,local_3a));
      uStack20 = pass1_1008_4772(CONCAT22(param_2,local_58));
     // puVar6 = (uStack20 >> 0x10);
      u_var3 = uStack20;
      mem_op_1000_179c(0x14,puVar6,0x1000);
      uVar7 = puVar6 | u_var3;
      if (uVar7 == 0x0) {
        pu_var2 = i_var9.field_0xc;
        (pu_var2 + 0x4) = 0x0;
      }
      else {
        u_var4 = &i_var9.field_0x16;
       // uVar9 = (uStack20 >> 0x10);
        pass1_1008_50c2(CONCAT22(puVar6,u_var3),
                        (uStack20 + 0x8),(uStack20 + 0x4),
                        (param_1 & 0xffff0000 | u_var4),uStack6);
        pu_var2 = i_var9.field_0xc;
       // uVar9 = (pu_var2 >> 0x10);
        iVar10 = pu_var2;
        iVar10.field_0x4 = u_var4;
        iVar10.field_0x6 = uVar7;
      }
      pu_var2 = i_var9.field_0xc;
      pass1_1008_5134((pu_var2 + 0x4));
      pass1_1008_41bc(CONCAT22(param_2,local_58));
      close_file_1008_496c(local_3a,param_2);
      uVar9 = extraout_dx;
    }
    pu_var2 = i_var9.field_0xc;
    pass1_1008_5236((pu_var2 + 0x4));
    pass1_1008_5236(*i_var9.field_0xc);
    u_var3 = &i_var9.field_0x16;
    pass1_1008_4480(uStack6,(param_1 & 0xffff0000 | u_var3),paStack10,
                    param_2);
    invalidate_rect_1020_8d90(param_1,uStack12,uStack6,u_var3,uVar9,param_2);
  }
  return;
}


astruct_18 *  pass1_1020_8e6c(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_8bae(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_8eaa(param_1: U32Ptr,param_2: u16)
{
  let u_var1: u16;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let i_var4: &mut Struct668;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  let paVar6: &mut Struct43;
  let local_a: [u8;8];
  
  struct_1020_847a(param_1,0x25,param_2);
 // u_var4 = (param_1 >> 0x10);
  i_var4 = param_1;
  &i_var4.field_0x16 = 0x0;
  i_var4.field_0xaa = 0x0;
  u_var1 = &i_var4.field_0xae;
  pu_var5 = clear_struct_1008_3e38((param_1 & 0xffff0000 | u_var1));
  &i_var4.field_0xb4 = 0x0;
  i_var4.field_0xb8 = 0xffff;
  &i_var4.field_0xba = 0x0;
  *param_1 = 0x9204;
  i_var4.field_0x2 = 0x1020;
  pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x29,param_2,
                           (pu_var5 >> 0x10),u_var4);
 // u_var2 = (pu_var5 >> 0x10);
  i_var4.field_0x16 = pu_var5;
  i_var4.field_0x18 = u_var2;
  pass1_1018_2646(i_var4.field_0x16,u_var2,
                  (param_1 & 0xffff0000 | u_var1));
  paVar6 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc,0x1ce,param_2);
 // pu_var3 = (paVar6 >> 0x10);
  i_var4.field_0xb4 = paVar6;
  i_var4.field_0xb6 = pu_var3;
  pass1_1020_8712(param_1 & 0xffff | u_var4 << 0x10,
                  CONCAT22(param_2,local_a),
                  (paVar6 & 0xffff0000 | i_var4.field_0xb4),
                  (param_1 & 0xffff0000 | u_var1));
  pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2,param_2,pu_var3,u_var4);
  i_var4.field_0xba = pu_var5;
  i_var4.field_0xbc = (pu_var5 >> 0x10);
  return;
}



pub fn pass1_1020_8f74(param_1: U32Ptr)
{
  let pu_var1: u32;
  let u_var2: u16;
  let ppc_var3: u32;
  let i_var4: &mut Struct593;
  let u_var4: u16;
  
 // u_var4 = (param_1 >> 0x10);
  i_var4 = param_1;
  *param_1 = 0x9204;
  i_var4.field_0x2 = 0x1020;
  pu_var1 = i_var4.field_0xb4;
  u_var2 = i_var4.field_0xb6;
  if ((u_var2 | pu_var1) != 0x0) {
    ppc_var3 = *pu_var1;
    (**ppc_var3)();
  }
  pass1_1020_8556(param_1);
  return;
}


pub fn pass1_1020_9068(param_1: U32Ptr,param_2: U32Ptr,param_3: i16,param_4: u16)
{
  let i_var1: i16;
  let u_var2: u32;
  let ppc_var3: u32;
  let u_var4: u32;
  let u_var5: u16;
  let u_var6: u32;
  let extraout_dx: u16;
  let uVar7: u16;
  let i_var8: i16;
  let i_var9: i16;
  let u_var10: u16;
  let u_var11: u16;
  let uStack10: i16;
  
 // u_var10 = (param_1 >> 0x10);
  i_var8 = param_1;
  u_var4 = (i_var8 + 0x16);
  u_var2 = (u_var4 + 0xa);
  u_var6 = u_var2;
  pass1_1018_280c((i_var8 + 0x16));
  (i_var8 + 0xaa) = u_var6;
  (i_var8 + 0xac) = param_2;
  u_var5 = param_2 | (i_var8 + 0xaa);
  if (u_var5 == 0x0) {
    pass1_1018_2862((i_var8 + 0x16));
    (i_var8 + 0xaa) = u_var5;
    (i_var8 + 0xac) = param_2;
  }
  if (((i_var8 + 0xac) | (i_var8 + 0xaa)) != 0x0) {
    pass1_1020_915a(param_1 & 0xffff | u_var10 << 0x10,param_2,param_3,
                    param_4);
    pass1_1008_4480(u_var2,(param_1 & 0xffff0000 | (i_var8 + 0xae)),
                    (i_var8 + 0xb4),param_4);
    ppc_var3 = (*param_1 + 0x10);
    (**ppc_var3)();
    u_var4 = (i_var8 + 0xaa);
    i_var1 = (u_var4 + 0xa);
      // TODO: refactor for loop
    // for (iStack10 = 0x0; iStack10 < i_var1; iStack10 += 0x1) {
    //   u_var6 = SEXT24(iStack10);
    //   empty_1008_8fc4((i_var8 + 0xaa),u_var6);
    //   u_var5 = u_var6;
    //   uVar7 = extraout_dx | u_var5;
    //   if (uVar7 != 0x0) {
    //     pass1_1008_8c4e(u_var6 & 0xffff | extraout_dx << 0x10,u_var2,param_4);
    //     u_var4 = (i_var8 + 0xc);
    //     u_var11 = (u_var4 >> 0x10);
    //     i_var9 = u_var4;
    //     (i_var9 + iStack10 * 0x4) = u_var5;
    //     (i_var9 + iStack10 * 0x4 + 0x2) = uVar7;
    //   }
    // }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_915a(param_1: u32,param_2: U32Ptr,param_3: i16,param_4: u16)
{
  let i_var1: i16;
  let i_var2: &mut Struct669;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let paVar4: &mut Struct43;
  let uStack12: u16;
  
  pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_4,param_2,param_3);
  i_var1 = (pu_var3 + 0x1e);
 // u_var2 = (param_1 >> 0x10);
  i_var2 = param_1;
  if (i_var2.field_0xb8 != i_var1) {
    uStack12 = 0x1ce;
    if (i_var1 == 0x1) {
      uStack12 = 0x1cf;
    }
    else {
      if (i_var1 == 0x2) {
        uStack12 = 0x1d0;
      }
      else {
        if (i_var1 == 0x3) {
          uStack12 = 0x1d1;
        }
        else {
          if (i_var1 == 0x4) {
            uStack12 = 0x1d2;
          }
        }
      }
    }
    paVar4 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc,uStack12,param_4);
    i_var2.field_0xb4 = paVar4;
    i_var2.field_0xb6 = (paVar4 >> 0x10);
    i_var2.field_0xb8 = i_var1;
  }
  return;
}



astruct_18 *  pass1_1020_91de(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_8f74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1020_96a2(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  palette_op_1020_92c4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_a426() -> u16

{
  let pu_var1: U32Ptr;
  
  clear_struct_1008_3e38(&ctx.PTR_LOOP_1048_4230);
  pu_var1 = clear_struct_1008_3e38(0x10484236);
  return pu_var1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_a43e(param_1: u16,param_2: U32Ptr,param_3: U32Ptr) -> u16

{
  let unaff_DI: i16;
  
  *param_3 = 0xba36;
  (param_3 + 0x2) = 0x1020;
  if (ctx.PTR_LOOP_1050_4e74 != 0x0) {
    return param_3;
  }
  mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2,param_1,param_2,unaff_DI);
  if ((0x0 < ctx.PTR_LOOP_1050_13ae) && (!SBORROW2(ctx.PTR_LOOP_1050_13ae,0x1))) {
    if (ctx.PTR_LOOP_1050_13ae == &ctx.PTR_LOOP_1050_0002 ||
        (ctx.PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      ctx.PTR_LOOP_1050_4e74 = 0x44b4;
//       TODO: goto LAB_1020_a482;
    }
    if (ctx.PTR_LOOP_1050_13ae == &DAT_1050_0004) {
      ctx.PTR_LOOP_1050_4e74 = 0x4b2c;
//       TODO: goto LAB_1020_a482;
    }
  }
  ctx.PTR_LOOP_1050_4e74 = 0x47f0;
//LAB_1020_a482:
  ctx._PTR_LOOP_1050_4e74 = CONCAT22(0x1050,PTR_LOOP_1050_4e74);
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_a49a(param_1: u16,param_2: u8,param_3: U32Ptr,param_4: u32,param_5: &mut i16,
               param_6: u16)

{
  let u_var1: u32;
  let unaff_DI: i16;
  let u_var2: u16;
  let u_var3: u16;
  let local_136: [u8;128];
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let puStack6: U32Ptr;
  
  puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_1,param_3,unaff_DI);
 // uStack12 = (puStack6 >> 0x10);
  u_var1 = (puStack6 + 0x20);
  uStack10 = u_var1;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
  uStack14 = u_var1;
  if (param_5 != 0x0) {
   // u_var2 = (param_5 >> 0x10);
    if ((param_5 + 0x1) == 0x0) {
      u_var3 = SUB42(&ctx.PTR_LOOP_1050_4230,0x0);
    }
    else {
      u_var3 = 0x4236;
    }
    pass1_1008_3f32(param_5,CONCAT22(0x1048,u_var3));
    struct_op_1028_87f0(param_1,param_2,CONCAT22(param_1,local_136),0x0,0x0,
                        param_6,param_5,u_var2,
                        (ctx.PTR_LOOP_1050_4e70 + 0x4),uStack10);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_136));
    return;
  }
  pass1_1020_abc0(param_1,param_2,param_4,param_6,u_var1 & 0xffff | uStack12 << 0x10
                 );
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_a54c(param_1: u16,param_2: u8,param_3: U32Ptr,param_4: u16,param_5: u16,
               param_6: i16)

{
  let u_var1: u32;
  let unaff_DI: i16;
  let u_var2: u16;
  let u_var3: u16;
  let u_var4: u16;
  let u_var5: u16;
  let u_var6: u16;
  let local_140: [u8;124];
  let puStack28: u32;
  let local_18: i16;
  let local_16: u16;
  let local_14: u32;
  let puStack16: U32Ptr;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let puStack6: U32Ptr;
  
  puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_1,param_3,unaff_DI);
 // uStack12 = (puStack6 >> 0x10);
  u_var1 = (puStack6 + 0x20);
  uStack10 = u_var1;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
  uStack14 = u_var1;
  local_14 = ctx._PTR_LOOP_1048_4230;
  puStack16 = ctx.PTR_LOOP_1048_4234;
  puStack28 = &local_14;
  pass1_1008_3e94(CONCAT22(param_1,&local_14),
                  CONCAT22(param_1,&local_18),
                  CONCAT22(param_1,&local_16));
  if ((param_6 < 0x0) || (0x5 < param_6)) {
    pass1_1008_3e76(CONCAT22(param_1,&local_14),0x0,local_18 - 0x9,local_16);
    u_var5 = uStack10;
   // u_var6 = (uStack10 >> 0x10);
    u_var1 = (ctx.PTR_LOOP_1050_4e70 + 0x4);
    u_var3 = u_var1;
   // u_var4 = (u_var1 >> 0x10);
    u_var2 = 0x14;
  }
  else {
    pass1_1008_3e76(CONCAT22(param_1,&local_14),0x0,(local_18 - param_6) - 0x3,
                    local_16);
    u_var5 = uStack10;
   // u_var6 = (uStack10 >> 0x10);
    u_var1 = (ctx.PTR_LOOP_1050_4e70 + 0x4);
    u_var3 = u_var1;
   // u_var4 = (u_var1 >> 0x10);
    u_var2 = 0x7b;
  }
  struct_op_1028_87f0(param_1,param_2,CONCAT22(param_1,local_140),0x0,0x0,
                      u_var2,&local_14,param_1,CONCAT22(u_var4,u_var3),CONCAT22(u_var6,u_var5))
  ;
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_140));
  return;
}



pub fn pass1_1020_a644(param_1: u16,param_2: u16,param_3: u32,param_4: u16) -> bool

{
  let b_var1: bool;
  
  b_var1 = write_to_file_1008_7cac(param_3,param_4);
  if (b_var1 != 0x0) {
    b_var1 = 0x1;
  }
  return b_var1;
}



pub fn
pass1_1020_a6ee(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
               param_6: u16,param_7: u8)

{
  let u_var1: u32;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let u_var4: u16;
  let local_13e: [u8;120];
  let uStack30: u32;
  let BStack26: bool;
  let local_18: u32;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  let uStack14: u32;
  let puStack10: U32Ptr;
  let uStack6: u32;
  
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
  uStack6 = CONCAT22(param_4,param_3);
  if (((param_4 | param_3) == 0x0) ||
     ((param_3 + 0x200) == 0x8000002)) {
    puStack10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_6,
                                (param_4 | param_3),param_5);
   // uStack16 = (puStack10 >> 0x10);
    u_var1 = (puStack10 + 0x20);
    uStack14 = u_var1;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    iStack18 = u_var1;
    pu_var3 = clear_struct_1008_3e38(CONCAT22(param_6, &local_18));
   // u_var2 = (pu_var3 >> 0x10);
    BStack26 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,param_2,0x28);
    if (BStack26 != 0x0) {
      uStack20 = 0x1;
    }
   // u_var4 = (param_1 >> 0x10);
    pass1_1020_b2da(param_6,param_1,u_var4,(BStack26 != 0x0),
                    CONCAT22(param_6,&local_18),CONCAT22(uStack16,iStack18));
    struct_op_1028_87f0(param_6,param_7,CONCAT22(param_6,local_13e),0x0,0x0,
                        param_2,&local_18,param_6,
                        (ctx.PTR_LOOP_1050_4e70 + 0x4),
                        (iStack18 + 0x4));
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_6,local_13e));
    if (BStack26 != 0x0) {
      pass1_1020_ad90(param_6,u_var2,param_1,u_var4,
                      CONCAT22(param_6,&local_18),(iStack18 + 0x4));
    }
    (uStack30 + 0x1c) = 0x8000001;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_a80e(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
               param_6: u16,param_7: u8,param_8: i16)

{
  let u_var1: u16;
  let u_var2: u32;
  let u_var3: u16;
  let puVar4: U32Ptr;
  
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
  if (((param_5 | param_4) == 0x0) ||
     ((param_4 + 0x200) == 0x8000002)) {
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_6,(param_5 | param_4)
                             ,param_8);
   // u_var3 = (puVar4 >> 0x10);
    u_var2 = (puVar4 + 0x20);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    u_var1 = u_var2;
    if (param_3 == 0xa) {
      pass1_1020_b872(param_6,param_7,CONCAT22(param_2,param_1),
                      u_var2 & 0xffff | u_var3 << 0x10);
      return;
    }
    pass1_1020_b0aa(param_1,param_2,param_3,u_var3);
    if (u_var1 != 0x0) {
      pass1_1020_abc0(param_6,param_7,CONCAT22(param_2,param_1),u_var1,
                      u_var2 & 0xffff | u_var3 << 0x10);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_a89e(param_1: u16,param_2: u32,param_3: U32Ptr,param_4: u16)
{
  let pi_var1: U32Ptr;
  let pu_var2: U32Ptr;
  let u_var3: u16;
  let u_var4: u16;
  let u_var5: u32;
  let in_DX: U32Ptr;
  let u_var6: u16;
  let puVar7: u32;
  let extraout_dx: u16;
  let unaff_DI: i16;
  let in_AF: u8;
  let uVar8: u16;
  let uVar9: u16;
  let u_var10: u8;
  let u_var11: u8;
  let local_5ee: u16;
  let uStack1516: u16;
  let puStack1218: u32;
  let iStack1214: i16;
  let uStack1212: u32;
  let local_4b8: [u8;8];
  let uStack1200: u32;
  let puStack1196: U32Ptr;
  let local_4a8: [u8;124];
  let local_384: [u8;124];
  let local_260: [u8;124];
  let local_13c: [u8;124];
  let local_18: u16;
  let local_16: u16;
  let local_14: u32;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u32;
  let puStack6: U32Ptr;
  
  puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_1,in_DX,unaff_DI);
 // u_var6 = (puStack6 >> 0x10);
  u_var5 = (puStack6 + 0x20);
  uStack10 = u_var5;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
  uStack14 = u_var5 & 0xffff | u_var6 << 0x10;
  local_14 = *param_3;
  uStack16 = (param_3 + 0x1);
  puStack1218 = &local_14;
  puVar7 = &local_14;
  pass1_1008_3e94(CONCAT22(param_1,puVar7),CONCAT22(param_1,&local_18)
                  ,CONCAT22(param_1,&local_16));
  u_var10 = param_1;
  u_var11 = (param_1 >> 0x8);
  pass1_1008_3e76(CONCAT13(u_var11,CONCAT12(u_var10,&local_14)),0x0,local_18,
                  local_16 + 0x2);
  struct_op_1028_8888(param_1,in_AF,CONCAT22(param_1,local_13c),0x0,0x7a,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_13c));
  pass1_1008_3e76(CONCAT13(u_var11,CONCAT12(u_var10,&local_14)),0x0,local_18 - 0x2
                  ,local_16);
  struct_op_1028_8888(param_1,in_AF,CONCAT22(param_1,local_260),0x0,0x47,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_260));
  pass1_1008_3e76(CONCAT13(u_var11,CONCAT12(u_var10,&local_14)),0x1,local_18 - 0x2
                  ,local_16);
  struct_op_1028_8888(param_1,in_AF,CONCAT22(param_1,local_384),0x0,0x6a,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_384));
  uVar8 = param_2;
 // uVar9 = (param_2 >> 0x10);
  pass1_1020_ad90(param_1,puVar7,uVar8,uVar9,CONCAT22(param_1,&local_14)
                  ,uStack10);
  pass1_1008_3e76(CONCAT13(u_var11,CONCAT12(u_var10,&local_14)),0x1,local_18 - 0x2
                  ,local_16 + 0x1);
  struct_op_1028_8888(param_1,in_AF,CONCAT22(param_1,local_4a8),0x0,0x7f,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_4a8));
  pass1_1020_ad90(param_1,puVar7,uVar8,uVar9,CONCAT22(param_1,&local_14)
                  ,uStack10);
  puStack1196 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x8,param_1,puVar7,
                                &uStack14);
  uStack1200 = (puStack1196 + 0x12);
  pass1_1008_5784(CONCAT22(param_1,local_4b8),uStack1200);
  iStack1214 = 0x0;
  loop {
    loop {
      pu_var2 = local_4b8;
      pass1_1008_5b12(pu_var2,param_1);
      uStack1212 = CONCAT22(extraout_dx,pu_var2);
      if ((extraout_dx | pu_var2) == 0x0) {
        pass1_1010_9674(puStack1196);
        return;
      }
        if (((pu_var2 + 0x4) != 0x3e) && ((pu_var2 + 0x4) != 0x41)) == false { break;}
    }
    while 0x0 < (uStack1212 + 0x6) {
      if (iStack1214 == 0x0) {
        u_var4 = local_16 - 0x2;
//LAB_1020_ab4a:
        u_var3 = local_18 - 0x2;
//LAB_1020_ab51:
        iStack1214 = iStack1214 + 0x1;
        pass1_1008_3e76(CONCAT13(u_var11,CONCAT12(u_var10,&local_14)),0x0,u_var3,
                        u_var4);
      }
      else {
        if (iStack1214 == 0x1) {
          u_var4 = local_16 + 0x2;
//           TODO: goto LAB_1020_ab4a;
        }
        if (iStack1214 == 0x2) {
          u_var4 = local_16 + 0x2;
//LAB_1020_ab6e:
          u_var3 = local_18 + 0x2;
//           TODO: goto LAB_1020_ab51;
        }
        if (iStack1214 == 0x3) {
          u_var4 = local_16 - 0x2;
//           TODO: goto LAB_1020_ab6e;
        }
        iStack1214 = iStack1214 + 0x1;
        pass1_1020_b2da(param_1,uVar8,uVar9,0x0,CONCAT22(param_1,&local_14),
                        uStack14);
      }
      struct_op_1028_8888(param_1,in_AF,CONCAT22(param_1,&local_5ee),0x0,
                          (uStack1212 + 0x4),&local_14,param_1,0x8000002,
                          0x4000002,uStack10);
      fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,&local_5ee));
      pi_var1 = (uStack1212 + 0x6);
      *pi_var1 = *pi_var1 + -0x1;
      local_5ee = 0x389a;
      uStack1516 = 0x1008;
    }
  }
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_abc0(param_1: u16,param_2: u8,param_3: u32,param_4: u16,param_5: u32)
{
  let u_var1: u16;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let u_var4: u16;
  let local_12e: [u8;124];
  let BStack10: bool;
  let local_8: u32;
  let uStack4: u16;
  
  pu_var3 = clear_struct_1008_3e38(CONCAT22(param_1, &local_8));
 // u_var1 = (pu_var3 >> 0x10);
  BStack10 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,param_4,0x28);
  if (BStack10 != 0x0) {
    uStack4 = 0x1;
  }
 // u_var4 = (param_3 >> 0x10);
  pass1_1020_b2da(param_1,param_3,u_var4,(BStack10 != 0x0),
                  CONCAT22(param_1,&local_8),param_5);
 // u_var2 = (param_5 >> 0x10);
  struct_op_1028_87f0(param_1,param_2,CONCAT22(param_1,local_12e),0x0,0x0,
                      param_4,&local_8,param_1,(ctx.PTR_LOOP_1050_4e70 + 0x4),
                      (param_5 + 0x4));
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_12e));
  if (BStack10 != 0x0) {
    pass1_1020_ad90(param_1,u_var1,param_3,u_var4,
                    CONCAT22(param_1,&local_8),(param_5 + 0x4));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_ac6e(param_1: u16,param_2: u8,param_3: u32,param_4: i16,param_5: i16,
               param_6: i16)

{
  let u_var1: u16;
  let pu_var2: u32;
  let u_var3: u32;
  let u_var4: u16;
  let unaff_DI: i16;
  let pu_var5: U32Ptr;
  let u_var6: u16;
  u8 local_146 [0x12c];
  let iStack26: i16;
  let uStack24: u16;
  let uStack22: u32;
  let puStack18: U32Ptr;
  let local_e: u32;
  let local_8: u16;
  let local_6: u32;
  
  if (param_4 == 0x0) {
    u_var6 = SUB42(&ctx.PTR_LOOP_1050_4230,0x0);
  }
  else {
    u_var6 = 0x4236;
  }
  pass1_1008_3eb4(CONCAT22(0x1048,u_var6),CONCAT22(param_1,&local_8),
                  CONCAT22(param_1,&local_6),
                  CONCAT22(param_1,&local_6 + 0x2));
  if (param_6 == 0x0) {
    local_6 = local_6 & 0xffff | (local_6._2_2_ + param_5) << 0x10;
  }
  else {
    if (param_6 == 0x1) {
      local_6 = local_6 & 0xffff0000 | (local_6 + param_5);
    }
    else {
      if (param_6 == 0x2) {
        local_6 = local_6 & 0xffff | (local_6._2_2_ - param_5) << 0x10;
      }
    }
  }
  pu_var5 = pass1_1008_3e54(CONCAT22(param_1,&local_e),local_8,local_6,
                           (local_6 >> 0x10));
  puStack18 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_1,
                              (pu_var5 >> 0x10),unaff_DI);
 // u_var4 = (puStack18 >> 0x10);
  u_var3 = (puStack18 + 0x20);
  uStack22 = u_var3;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
  iStack26 = u_var3;
  uStack24 = u_var4;
  u_var1 = pass1_1020_b1ae(&local_e,u_var4,param_1,param_3,
                          (param_3 >> 0x10),CONCAT22(param_1,&local_e),
                          (iStack26 + 0x4));
  if (u_var1 != 0x0) {
    pu_var2 = &local_e;
    pass1_1020_b240(param_1,u_var4,param_3,CONCAT22(param_1,pu_var2),
                    CONCAT22(uStack24,iStack26));
    if (pu_var2 != 0x0) {
      struct_op_1028_87f0(param_1,param_2,CONCAT22(param_1,local_146),0x0,
                          0x0,(-(param_4 == 0x0) & 0xfffb) + 0x7f,&local_e,param_1,
                          (ctx.PTR_LOOP_1050_4e70 + 0x4),uStack22);
      fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,local_146));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_ad90(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
               param_5: U32Ptr,param_6: u32)

{
  let ppcVar1: u32;
  let pu_var2: U32Ptr;
  let pu_var3: U32Ptr;
  let i_var4: i16;
  let u_var5: u32;
  let u_var6: u16;
  let extraout_dx: u16;
  let in_AF: u8;
  let puVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let u_var10: u16;
  let local_17e: u16;
  let uStack380: u16;
  let iStack90: i16;
  let puStack78: u32;
  let uStack70: u16;
  let iStack68: i16;
  let uStack66: u32;
  let puStack62: u32;
  u8 local_3a [0xc];
  let local_2e: u32;
  let uStack42: u16;
  let iStack40: i16;
  let uStack38: u16;
  let local_24: i16;
  let local_22: i16;
  let uStack32: u32;
  let uStack28: u32;
  let uStack24: u32;
  let puStack20: U32Ptr;
  let uStack18: u16;
  let iStack16: i16;
  let iStack14: i16;
  let uStack12: u32;
  let local_8: u16;
  let local_6: i16;
  let local_4: i16;
  
  pu_var2 = &local_8;
  pass1_1008_3eb4(param_5,CONCAT22(param_1,pu_var2),
                  CONCAT22(param_1,&local_6),
                  CONCAT22(param_1,&local_4));
  pass1_1030_627e(param_1,pu_var2,param_2,_PTR_LOOP_1050_5740,param_5,param_6)
  ;
  puStack20 = pu_var2;
  uStack18 = param_2;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2);
  uStack24 = CONCAT22(param_2,pu_var2);
  uStack28 = (pu_var2 + 0x17);
  u_var5 = (uStack28 + 0x4);
  uStack32 = u_var5;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6);
  iStack40 = u_var5;
  uStack38 = param_2;
  puVar7 = pass1_1030_5b5c(iStack40,param_2);
 // u_var6 = (puVar7 >> 0x10);
  local_2e = *puVar7;
  uStack42 = (puVar7 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94(CONCAT22(param_1,&local_2e),
                  CONCAT22(param_1,&local_24),
                  CONCAT22(param_1,&local_22));
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (local_6 - 0x1);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if (uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90(CONCAT22(param_1,local_3a));
  pass1_1008_6cec(CONCAT22(param_1,local_3a),local_8,CONCAT22(iStack14,iStack16)
                  ,local_8,uStack12);
  pu_var3 = local_3a;
  pass1_1030_6522(ctx.PTR_LOOP_1050_5740,CONCAT22(param_1,pu_var3),param_6,param_1);
  puStack62 = CONCAT22(u_var6,pu_var3);
  if ((u_var6 | pu_var3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
// TODO: refactor for loop
//     for (uStack70 = uStack12; uStack70 <= iStack16; uStack70 += 0x1) {
//       for (puStack78 = uStack12._2_2_; puStack78 <= iStack14;
//           puStack78 = (puStack78 + 0x1)) {
//         ppcVar1 = (*puStack62 + 0x4);
//         i_var4 = iStack68;
//         iStack68 = iStack68 + 0x1;
//         (**ppcVar1)(0x1030,puStack62,(puStack62 >> 0x10));
//         uStack66 = CONCAT22(extraout_dx,i_var4);
//         uStack66._3_1_ = (extraout_dx >> 0x8);
//         if (uStack66._3_1_ == '\0') {
//           iStack90 = i_var4;
//           if (i_var4 == 0x7) {
//             pass1_1008_3e76(param_5,local_8,uStack70,puStack78);
//             uVar9 = uStack32;
//             u_var10 = (uStack32 >> 0x10);
//             uVar8 = 0x6;
//           }
//           else {
//             if (i_var4 == 0x8) {
//               pass1_1008_3e76(param_5,local_8,uStack70,puStack78);
//               uVar9 = uStack32;
//               u_var10 = (uStack32 >> 0x10);
//               uVar8 = 0x7;
//             }
//             else {
//               if (i_var4 != 0x9) goto LAB_1020_af1c;
//               pass1_1008_3e76(param_5,local_8,uStack70,puStack78);
//               uVar9 = uStack32;
//               u_var10 = (uStack32 >> 0x10);
//               uVar8 = 0x8;
//             }
//           }
//           struct_op_1028_87f0(param_1,in_AF,CONCAT22(param_1,&local_17e),0x0
//                               ,0x0,uVar8,param_5,(param_5 >> 0x10)
//                               ,CONCAT22(u_var10,uVar9),param_6);
//           fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,&local_17e));
//           local_17e = 0x389a;
//           uStack380 = 0x1008;
//         }
// //LAB_1020_af1c:
//       }
//     }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_afc4(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
               param_5: U32Ptr,param_6: i32)

{
  let pu_var1: u32;
  let u_var2: u16;
  let u_var3: u16;
  let u_var4: u32;
  let bStack27: u8;
  let local_a: u32;
  let uStack6: u32;
  
  pu_var1 = &local_a;
  pass1_1030_64ce(param_1,pu_var1,param_2,_PTR_LOOP_1050_5740,param_5,param_6,
                  CONCAT22(param_1,pu_var1));
  uStack6 = *pu_var1;
  u_var3 = (pu_var1 + 0x2);
  bStack27 = (uStack6 >> 0x18);
  u_var2 = bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
  u_var4 = struct_op_1030_73a8(CONCAT22(u_var3,u_var2));
 // u_var3 = (u_var4 >> 0x10);
  if ((u_var3 | u_var4) != 0x0) {
    switch((u_var4 + 0xc)) {
    0x1 =>
      break;
    0x2 =>
      break;
    0x3 =>
      break;
    0x4 =>
      break;
    0x5 =>
      break;
    0x6 =>
      break;
    0x7 =>
      return;
    0x8 =>
      return;
    0x9 =>
      return;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_b0aa(param_1: u16,param_2: u16,param_3: i16,param_4: u16)
{
  let pu_var1: u32;
  let ppcVar2: u32;
  let iVar3: i16;
  let puVar4: u32;
  let extraout_dx: u16;
  let u_var5: u16;
  let u_var6: u16;
  let uVar7: u16;
  let uVar8: u32;
  let uStack20: u32;
  
  uVar7 = (ctx.PTR_LOOP_1050_4e74 >> 0x10);
  if ((ctx.PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) == 0x0) {
    return;
  }
  if ((ctx.PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) != -0x1) {
    if (ctx.PTR_LOOP_1050_4e78 == 0x0) {
      iVar3 = param_3;
      pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
      pu_var1 = (iVar3 + 0xc);
      ppcVar2 = (*pu_var1 + 0x10);
      puVar4 = pu_var1;
      (**ppcVar2)();
      u_var6 = extraout_dx;
        // TODO: refactor for loop
      // for (uStack20 = 0x0;
      //     uStack20 < (puVar4 & 0xffff | extraout_dx << 0x10);
      //     uStack20 += 0x1) {
      //   uVar8 = pass1_1030_1d7c((puVar4 & 0xffff),u_var6,pu_var1);
      //   u_var5 = (uVar8 >> 0x10);
      //   u_var6 = u_var5 | uVar8;
      //   if ((u_var6 != 0x0) &&
      //      ((iVar3 = (uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b)))) {
      //     ctx.PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
      //     break;
      //   }
      // }
      if (ctx.PTR_LOOP_1050_4e78 == 0x0) {
        ctx.PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        return;
      }
    }
    iVar3 = (ctx.PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) + -0x1;
    pass1_1008_612e(0x0,iVar3,iVar3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1020_b1ae(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: U32Ptr,param_7: u32)

{
  let pu_var1: u32;
  let local_14: i16;
  let local_12: i16;
  let local_10: i16;
  let local_e: i16;
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,param_7,(param_7 >> 0x10));
  iStack6 = param_1;
  uStack4 = param_2;
  pu_var1 = pass1_1030_5b5c(param_1,param_2);
  local_c = *pu_var1;
  uStack8 = (pu_var1 + 0x4);
  pass1_1008_3e94(param_6,CONCAT22(param_3,&local_10),
                  CONCAT22(param_3,&local_e));
  pass1_1008_3e94(CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_14),
                  CONCAT22(param_3,&local_12));
  if ((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) &&
     (local_10 < local_14 + -0xb)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_b240(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u32)
{
  let pu_var1: u32;
  let u_var2: u16;
  let u_var3: u16;
  let u_var4: u16;
  let u_var5: u16;
  let u_var6: u16;
  let uVar7: u32;
  let bStack31: u8;
  let local_a: u32;
  let uStack6: u32;
  
  pu_var1 = &local_a;
 // u_var6 = (param_5 >> 0x10);
  pass1_1030_64ce(param_1,pu_var1,param_2,_PTR_LOOP_1050_5740,param_4,
                  (param_5 + 0x4),CONCAT22(param_1,pu_var1));
  uStack6 = *pu_var1;
  u_var5 = (pu_var1 + 0x2);
  bStack31 = (uStack6 >> 0x18);
  u_var2 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
    uVar7 = struct_op_1030_73a8(CONCAT22(u_var5,u_var2));
   // u_var4 = (uVar7 >> 0x10);
    u_var2 = uVar7;
    u_var5 = u_var4 | u_var2;
    if ((u_var5 != 0x0) && (u_var2 = (u_var2 + 0xc), 0x9 < u_var2)) {
      return;
    }
  }
  u_var3 = pass1_1020_b1ae(u_var2,u_var5,param_1,param_3,(param_3 >> 0x10),
                          param_4,(param_5 + 0x4));
  if (u_var3 == 0x0) {
    return;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub fn
pass1_1020_b2da(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: U32Ptr,
               param_6: u32)

{
  let i_var1: i16;
  let u_var2: u16;
  let u_var3: u16;
  let puVar4: U32Ptr;
  let u_var5: u16;
  let in_AF: u8;
  let puVar6: U32Ptr;
  u8 **ppuVar7;
  let iStack28: i16;
  let local_1a: [u8;6];
  let uStack20: u16;
  let uStack18: u16;
  let piStack16: U32Ptr;
  let piStack12: U32Ptr;
  let local_8: u16;
  let local_6: u32;
  
  if (param_4 == 0x0) {
    u_var2 = 0x4e6a;
  }
  else {
    u_var2 = 0x4e6e;
  }
  piStack12 = CONCAT22(0x1050,u_var2);
  if (param_4 == 0x0) {
    uStack20 = 0x4e68;
  }
  else {
    uStack20 = 0x4e6c;
  }
  uStack18 = SUB42(ctx.data_seg,0x0);
  piStack16 = CONCAT22(0x1050,uStack20);
  loop {
    if (param_4 == 0x0) {
      ppuVar7 = &ctx.PTR_LOOP_1048_4230;
    }
    else {
      ppuVar7 = (u8 **)0x10484236;
    }
    pass1_1008_3eb4(ppuVar7,CONCAT22(param_1,&local_8),
                    CONCAT22(param_1,&local_6),
                    CONCAT22(param_1,&local_6 + 0x2));
    i_var1 = *piStack12;
    if (i_var1 == 0x0) {
      local_6 = CONCAT22(local_6._2_2_ + *piStack16,local_6 + -0x1);
    }
    else {
      if (i_var1 == 0x1) {
        local_6 = CONCAT22(local_6._2_2_ + -0x1,local_6 + *piStack16);
      }
      else {
        if (i_var1 == 0x2) {
          local_6 = CONCAT22(local_6._2_2_ - *piStack16,local_6 + -0x1);
        }
      }
    }
    puVar6 = pass1_1008_3e54(CONCAT22(param_1,local_1a),local_8,local_6,
                             (local_6 >> 0x10));
   // u_var5 = (puVar6 >> 0x10);
   // u_var2 = (param_6 >> 0x10);
    u_var3 = pass1_1020_b1ae(local_1a,u_var5,param_1,param_2,param_3,
                            CONCAT22(param_1,local_1a),
                            (param_6 + 0x4));
    if (u_var3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(param_1,u_var5,CONCAT22(param_3,param_2),CONCAT22(param_1,puVar4),
                      param_6);
      if (puVar4 != 0x0) {
//LAB_1020_b46e:
        pass1_1008_3e76(param_5,local_8,local_6,(local_6 >> 0x10));
        return;
      }
    }
    i_var1 = *piStack12;
    if (i_var1 == 0x0) {
//LAB_1020_b45e:
      local_6 = local_6 & 0xffff0000 | (local_6 + 0x2);
    }
    else {
      if (i_var1 == 0x1) {
        local_6 = local_6 & 0xffff | (local_6._2_2_ + 0x2) << 0x10;
      }
      else {
        if (i_var1 == 0x2) {
            // goto
            // LAB_1020_b45e;
        }
      }
    }
    pass1_1008_3e76(CONCAT22(param_1,local_1a),local_8,local_6,
                    (local_6 >> 0x10));
    u_var3 = pass1_1020_b1ae(local_1a,u_var5,param_1,param_2,param_3,
                            CONCAT22(param_1,local_1a),
                            (param_6 + 0x4));
    if (u_var3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(param_1,u_var5,CONCAT22(param_3,param_2),CONCAT22(param_1,puVar4),
                      param_6);
      if (puVar4 != 0x0) {
          // goto
          // LAB_1020_b46e;
      }
    }
    iStack28 = *piStack12 + 0x1;
    if (0x2 < iStack28) {
      iStack28 = 0x0;
      *piStack16 = *piStack16 + 0x1;
    }
    *piStack12 = iStack28;
    pass1_1020_ac6e(param_1,in_AF,CONCAT22(param_3,param_2),param_4,*piStack16,iStack28);
  }
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_b482(param_1: u16,param_2: u32,param_3: U32Ptr,param_4: u32)
{
  let pu_var1: U32Ptr;
  let pu_var2: u32;
  let u_var3: u32;
  let u_var4: u16;
  let u_var5: u16;
  let puVar6: u32;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let puVar10: u32;
  let iStack46: i16;
  let local_2a: u32;
  let local_26: u16;
  let local_24: u32;
  let uStack32: u16;
  let lStack30: i32;
  let uStack26: u32;
  let local_16: [u8;12];
  let local_4: [u8;2];
  
  uVar7 = pass1_1030_bcae(local_4,param_1);
 // u_var4 = (uVar7 >> 0x10);
  pass1_1028_dc52(CONCAT22(param_1,local_16),0x1,0x0,0x400);
  loop {
    pu_var1 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,pu_var1));
    uStack26 = CONCAT22(u_var4,pu_var1);
    u_var5 = u_var4 | pu_var1;
    if (u_var5 == 0x0) {
      pass1_1020_b240(param_1,0x0,param_2,param_3,param_4);
      if (pu_var1 != 0x0) {
        lStack30 = (param_4 + 0x4);
        local_24 = *param_3;
        uStack32 = (param_3 + 0x4);
        puVar6 = &local_2a;
        pass1_1008_3eb4(CONCAT22(param_1,&local_24),
                        CONCAT22(param_1,puVar6),
                        CONCAT22(param_1,&local_2a + 0x2),
                        CONCAT22(param_1,&local_26));
        pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                        local_2a._2_2_ - 0x1,local_26 - 0x1);
        pu_var2 = &local_24;
        uVar8 = param_2;
       // uVar9 = (param_2 >> 0x10);
        pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                        CONCAT22(param_1,pu_var2),lStack30);
        if (pu_var2 != 0x0) {
          pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                          (local_2a >> 0x10),local_26 - 0x1);
          pu_var2 = &local_24;
          pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                          CONCAT22(param_1,pu_var2),lStack30);
          if (pu_var2 != 0x0) {
            pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                            local_2a._2_2_ + 0x1,local_26 - 0x1);
            pu_var2 = &local_24;
            pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                            CONCAT22(param_1,pu_var2),lStack30);
            if (pu_var2 != 0x0) {
              pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                              local_2a._2_2_ - 0x1,local_26);
              pu_var2 = &local_24;
              pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                              CONCAT22(param_1,pu_var2),lStack30);
              if (pu_var2 != 0x0) {
                pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                                local_2a._2_2_ + 0x1,local_26);
                pu_var2 = &local_24;
                pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                CONCAT22(param_1,pu_var2),lStack30);
                if (pu_var2 != 0x0) {
                  pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                                  local_2a._2_2_ + 0x1,local_26 + 0x1);
                  pu_var2 = &local_24;
                  pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                  CONCAT22(param_1,pu_var2),lStack30);
                  if (pu_var2 != 0x0) {
                    pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a
                                    ,(local_2a >> 0x10),local_26 + 0x1);
                    pu_var2 = &local_24;
                    pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                    CONCAT22(param_1,pu_var2),lStack30);
                    if (pu_var2 != 0x0) {
                      pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                      local_2a,local_2a._2_2_ - 0x1,local_26 + 0x1
                                     );
                      pu_var2 = &local_24;
                      pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                      CONCAT22(param_1,pu_var2),lStack30);
                      if (pu_var2 != 0x0) {
                        pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                        local_2a,local_2a._2_2_ - 0x2,
                                        local_26 - 0x2);
                        pu_var2 = &local_24;
                        pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                        CONCAT22(param_1,pu_var2),lStack30);
                        if (pu_var2 != 0x0) {
                          pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                          local_2a,local_2a._2_2_ + 0x2,
                                          local_26 - 0x2);
                          pu_var2 = &local_24;
                          pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                          CONCAT22(param_1,pu_var2),lStack30);
                          if (pu_var2 != 0x0) {
                            pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                            local_2a,local_2a._2_2_ - 0x2,
                                            local_26 + 0x2);
                            pu_var2 = &local_24;
                            pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                            CONCAT22(param_1,pu_var2),lStack30);
                            if (pu_var2 != 0x0) {
                              pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                              local_2a,local_2a._2_2_ + 0x2,
                                              local_26 + 0x2);
                              pu_var2 = &local_24;
                              pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                              CONCAT22(param_1,pu_var2),lStack30)
                              ;
                              if (pu_var2 != 0x0) {
                                pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                                local_2a,local_2a._2_2_ - 0x1,
                                                local_26 + 0x2);
                                pu_var2 = &local_24;
                                pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                                CONCAT22(param_1,pu_var2),
                                                lStack30);
                                if (pu_var2 != 0x0) {
                                  pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                                  local_2a,local_2a._2_2_ - 0x1,
                                                  local_26 + 0x3);
                                  pu_var2 = &local_24;
                                  pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                                  CONCAT22(param_1,pu_var2),
                                                  lStack30);
                                  if (pu_var2 != 0x0) {
                                    iStack46 = 0x3;
                                    loop {
                                      if (0x9 < iStack46) {
                                        return;
                                      }
                                      pass1_1008_3e76(
                                                      CONCAT22(param_1,&local_24),0x0,
                                                      local_2a._2_2_ - iStack46,local_26);
                                      pu_var2 = &local_24;
                                      pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                                      CONCAT22(param_1,pu_var2),
                                                      lStack30);
                                      if (pu_var2 == 0x0) { break; }
                                      iStack46 += 0x1;
                                    }
                                    return;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      return;
    }
    u_var3 = (pu_var1 + 0x10);
    puVar10 = param_3;
    uVar7 = param_4;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
    pu_var1 = local_4;
    pass1_1030_bcbc(param_1,pu_var1,CONCAT22(u_var3,param_1),
                    CONCAT22(puVar10,u_var5),(puVar10 >> 0x10),uVar7);
    if (pu_var1 < 0x0) { break; }
    u_var4 = u_var5;
    if (pu_var1 < 0x65) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_b872(param_1: u16,param_2: u8,param_3: u32,param_4: u32)
{
  let u_var1: u16;
  let u_var2: u16;
  let u_var3: u16;
  let puVar4: u32;
  let pu_var5: U32Ptr;
  let puVar6: u32;
  let puVar7: U32Ptr;
  let uVar8: u16;
  let local_136: [u8;124];
  let local_12: u32;
  let local_c: i16;
  let local_a: i16;
  let local_8: u32;
  let uStack4: u16;
  
 // uVar8 = (param_4 >> 0x10);
  puVar6 = pass1_1030_5b5c(param_4,uVar8);
  local_8 = *puVar6;
  uStack4 = (puVar6 + 0x4);
  pass1_1008_3e94(CONCAT22(param_1,&local_8),
                  CONCAT22(param_1,&local_c),
                  CONCAT22(param_1,&local_a));
  u_var1 = local_a - 0xa;
  pass1_1008_612e(0xa,u_var1,u_var1);
  u_var2 = local_c - 0xa;
  pass1_1008_612e(0xa,u_var2,u_var2);
  puVar7 = pass1_1008_3e54(CONCAT22(param_1,&local_12),0x0,u_var2,u_var1);
 // u_var1 = (puVar7 >> 0x10);
  loop {
    puVar4 = &local_12;
    pass1_1020_b482(param_1,param_3,CONCAT22(param_1,puVar4),param_4);
    if (puVar4 != 0x0) { break; }
    u_var2 = local_a - 0xa;
    pass1_1008_612e(0xa,u_var2,u_var2);
    u_var3 = local_c - 0xa;
    pass1_1008_612e(0xa,u_var3,u_var3);
    pass1_1008_3e76(CONCAT22(param_1,&local_12),0x0,u_var3,u_var2);
  }
  struct_op_1028_8888(param_1,param_2,CONCAT22(param_1,local_136),0x0,0xa,
                      &local_12,param_1,0x8000002,0x0,(param_4 + 0x4));
  pu_var5 = local_136;
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_1,pu_var5));
  pass1_1020_b97e(param_1,pu_var5,u_var1,param_3,(param_3 >> 0x10),0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_b97e(param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: i16)

{
  let u_var1: u32;
  let local_e: i16;
  let local_c: u16;
  let uStack10: i16;
  let uStack8: u16;
  let uStack6: u32;
  
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
  ctx._PTR_LOOP_1050_4e70 = CONCAT22(param_3,param_2);
  u_var1 = (param_2 + 0x10);
  uStack6 = u_var1;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
  iStack10 = u_var1;
  uStack8 = param_3;
  pass1_1008_3f62(&ctx.PTR_LOOP_1048_4230,CONCAT22(param_3,iStack10 + 0xc)
                 );
  pass1_1008_3e94(&ctx.PTR_LOOP_1048_4230,CONCAT22(param_1,&local_e),
                  CONCAT22(param_1,&local_c));
  if (param_6 == 0x0) {
    pass1_1008_3e76(&ctx.PTR_LOOP_1048_4230,0x0,local_e + 0x1,local_c - 0x1);
    pass1_1008_3e94(&ctx.PTR_LOOP_1048_4230,CONCAT22(param_1,&local_e),
                    CONCAT22(param_1,&local_c));
  }
  pass1_1008_3e76(0x10484236,0x1,local_e - 0x2,local_c);
  return;
}



pub fn pass1_1020_ba2b()
{
  init_globals_1020_96d4();
  pass1_1020_a426();
  return;
}



pub fn pass1_1020_ba3e(param_1: &i32,param_2: u16,param_3: i16,param_4: u16,param_5: u16)
{
  let i_var1: &mut Struct172;
  let u_var1: u16;
  let unaff_SS: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x0;
  i_var1.field_0x4 = 0x0;
  i_var1.field_0x6 = param_3;
  i_var1.field_0x8 = param_2;
  if (i_var1.field_0x6 == 0x0) {
    i_var1.field_0x6 = 0x5;
  }
  pass1_1020_bcc4(param_1,param_4,unaff_SS);
  return;
}


pub fn pass1_1020_ba94(param_1: &i32)
{
  let pu_var1: U32Ptr;
  let uStack8: u16;
  
  if (*param_1 == 0x0) {
    return;
  }
  uStack8 = 0x0;
  loop {
    pu_var1 = (param_1 + 0x4);
    if (*pu_var1 < uStack8 || *pu_var1 == uStack8) { break; }
    uStack8 += 0x1;
  }
  return;
}



pub fn pass1_1020_bae6(param_1: u16,param_2: u32,param_3: u16,param_4: u16,param_5: u16) -> u32

{
  let puStack6: U32Ptr;
  
  pass1_1020_bc92(CONCAT22(param_2,param_1),(param_2 >> 0x10),param_5
                 );
  puStack6 = CONCAT22(param_4,param_3);
  if ((param_4 | param_3) != 0x0) {
    return CONCAT22((param_3 + 0x2),*puStack6);
  }
  return 0x0;
}



pub fn pass1_1020_bb16(param_1: U32Ptr,param_2: U32Ptr,param_3: U32Ptr,param_4: u16)
{
  if ((param_1 + 0x4) < param_4) {
    *param_3 = 0x0;
    *param_2 = 0x0;
    return;
  }
  *param_3 = (param_4 * 0x6 + *param_1 + 0x4);
  *param_2 = (*param_1 + param_4 * 0x6);
  return;
}



pub fn
pass1_1020_bb70(param_1: &i32,param_2: u16,param_3: u32,param_4: u16,
               param_5: u16,param_6: u16)

{
  pass1_1020_bba4(param_1,0x1,param_2,param_3,(param_3 >> 0x10),
                  param_4,param_6);
  return;
}



pub fn
pass1_1020_bb8a(param_1: &i32,param_2: u16,param_3: u32,param_4: u16,
               param_5: u16)

{
  pass1_1020_bba4(param_1,0x0,param_2,param_3,(param_3 >> 0x10),
                  param_4,param_5);
  return;
}



bool 
pass1_1020_bba4(param_1: &i32,param_2: i16,param_3: u16,param_4: i16,param_5: u16,
               param_6: u16,param_7: u16)

{
  let in_AX: U32Ptr;
  let in_DX: u16;
  let u_var1: u16;
  let u_var2: u16;
  let bVar3: bool;
  let puStack6: U32Ptr;
  
  pass1_1020_bc92(param_1,param_5,param_7);
  puStack6 = CONCAT22(in_DX,in_AX);
  u_var1 = in_DX | in_AX;
  if (u_var1 == 0x0) {
    pass1_1020_bc92(param_1,0x0,param_7);
    u_var2 = u_var1 | in_AX;
    if (u_var2 == 0x0) {
      pass1_1020_bcc4(param_1,param_6,param_7);
      pass1_1020_bc92(param_1,0x0,param_7);
      if ((u_var2 | in_AX) == 0x0) {
        return 0x0;
      }
      in_AX[0x2] = param_5;
      u_var1 = u_var2;
    }
    else {
      in_AX[0x2] = param_5;
    }
    if (param_2 != 0x0) {
      u_var2 = *in_AX;
      bVar3 = CARRY2(u_var2,param_3);
      param_3 = u_var2 + param_3;
      param_4 = in_AX[0x1] + param_4 + bVar3;
    }
    *in_AX = param_3;
    in_AX[0x1] = param_4;
    pass1_1020_bc72(param_1,param_6,param_7);
  }
  else {
    if (param_2 != 0x0) {
      bVar3 = CARRY2(*puStack6,param_3);
      param_3 = *puStack6 + param_3;
      param_4 = in_AX[0x1] + param_4 + bVar3;
    }
    *in_AX = param_3;
    in_AX[0x1] = param_4;
  }
  return 0x1;
}



pub fn pass1_1020_bc72(param_1: U32Ptr,param_2: u16,param_3: u16)
{
  let u_var1: u32;
  let u_var2: u16;
  
 // u_var2 = (param_1 >> 0x10);
  u_var1 = (param_1 + 0x2);
  pass1_1000_4aea(*param_1,u_var1,(u_var1 >> 0x10),0x6,0xbd6c,
                  &stack0xfffe,param_2,u_var2,0x1000,param_3);
  return;
}



pub fn pass1_1020_bc92(param_1: U32Ptr,param_2: u16,param_3: u16)
{
  let u_var1: u32;
  let local_c: [u8;4];
  let uStack8: u16;
  
  uStack8 = param_2;
  u_var1 = (param_1 + 0x2);
  pass1_1000_49c6(local_c,param_3,*param_1,u_var1,
                  (u_var1 >> 0x10),0x6,0xbd6c,&stack0xfffe);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_bcc4(param_1: &i32,param_2: u16,param_3: u16)
{
  let pu_var1: U32Ptr;
  let i_var2: i16;
  let u_var3: u16;
  let i_var4: i16;
  let u_var5: u16;
  let lVar6: i32;
  let lStack12: i32;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1;
  if ((i_var4 + 0x4) == 0x0) {
    ctx.PTR_LOOP_1050_5f2e = 0x0;
    i_var2 = (i_var4 + 0x6);
  }
  else {
    u_var3 = (i_var4 + 0x4);
    pu_var1 = (i_var4 + 0x8);
    i_var2 = u_var3 + *pu_var1;
    ctx.PTR_LOOP_1050_5f2e = CARRY2(u_var3,*pu_var1);
  }
  if ((false) || (ctx.PTR_LOOP_1050_5f2e == 0x0)) {
    if (*param_1 == 0x0) {
      if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      u_var3 = fn_ptr_op_1000_1708(i_var2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                  ctx.PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
      lVar6 = pass1_1000_0ed4(
          ctx, 0x1000, param_3, 0x1, i_var2 * 0x6, 0x0, *param_1,
          (*param_1 >> 0x10));
      ctx.PTR_LOOP_1050_5f2e = (lVar6 >> 0x10);
      u_var3 = lVar6;
    }
    lStack12 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,u_var3);
    if ((ctx.PTR_LOOP_1050_5f2e | u_var3) != 0x0) {
      (i_var4 + 0x4) = i_var2;
      *param_1 = lStack12;
      pass1_1020_bc72((param_1 & 0xffff | u_var5 << 0x10),param_2,
                      param_3);
    }
  }
  return;
}



i16  pass1_1020_bd6c(param_1: u32,param_2: u32)

{
  return (param_1 + 0x4) - (param_2 + 0x4);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_bd80(param_1: u16) -> u16

{
  let mut pcVar1: String; 
  let uStack6: u16;
  
  switch(param_1) {
  0x1 =>
  0x6 =>
    break;
  0x2 =>
    break;
  0x3 =>
  0x7 =>
    break;
  0x4 =>
  0x8 =>
    break;
  0x5 =>
  0x9 =>
    break;
  0xa =>
    break;
  0xb =>
  0x37 =>
    break;
  0xc =>
  0x35 =>
  0x36 =>
    break;
  0xd =>
    break;
  0xe =>
    break;
  0xf =>
    break;
  0x10 =>
    break;
  0x11 =>
    break;
  0x12 =>
    break;
  0x13 =>
  0x14 =>
  0x15 =>
    break;
  0x16 =>
  0x19 =>
    break;
  0x17 =>
  0x1a =>
    break;
  0x18 =>
    break;
  0x1b =>
  0x1c =>
  0x1d =>
    break;
  0x1e =>
  0x1f =>
  0x20 =>
    break;
  0x21 =>
    break;
  0x22 =>
  0x23 =>
  0x24 =>
    break;
  0x25 =>
  0x26 =>
  0x27 =>
    break;
  0x28 =>
  0x29 =>
    break;
  0x2a =>
  0x2b =>
    break;
  0x2c =>
    break;
  0x2d =>
  0x2e =>
    break;
  0x2f =>
  0x30 =>
    break;
  0x31 =>
  0x32 =>
    break;
  0x33 =>
  0x34 =>
    break;
  0x38 =>
  0x39 =>
    break;
  0x3a =>
  0x3b =>
    break;
  0x3c =>
  0x3d =>
    break;
  0x3e =>
    break;
  0x3f =>
    break;
  0x40 =>
    break;
  0x41 =>
    break;
  0x42 =>
  0x46 =>
  0x6b =>
    break;
  0x43 =>
    uStack6 = s_bidLRoadConst_1050_4e7a;
    return uStack6;
  0x44 =>
    uStack6 = s_bidRRoadConst_1050_4e88;
    return uStack6;
  0x45 =>
    uStack6 = s_bidXRoadConst_1050_4e96;
    return uStack6;
  0x47 =>
    break;
  0x48 =>
  0x49 =>
  0x4a =>
  0x70 =>
  0x71 =>
  0x72 =>
    break;
  0x4b =>
    break;
  0x4c =>
    break;
  0x4d =>
    break;
  0x4e =>
    break;
  0x4f =>
  0x50 =>
  0x51 =>
    break;
  0x52 =>
  0x53 =>
    break;
  0x54 =>
  0x55 =>
  0x56 =>
    break;
  0x57 =>
  0x58 =>
  0x59 =>
    break;
  0x5a =>
    break;
  0x5b =>
  0x5c =>
    break;
  0x5d =>
  0x5e =>
  0x5f =>
    break;
  0x60 =>
  0x61 =>
    break;
  0x62 =>
  0x63 =>
    break;
  0x64 =>
    break;
  0x65 =>
    break;
  0x66 =>
  0x67 =>
    break;
  0x68 =>
  0x69 =>
    break;
  0x6a =>
    break;
  0x6c =>
  0x6d =>
    break;
  0x6e =>
    break;
  0x6f =>
    break;
  0x73 =>
  0x77 =>
    break;
  0x74 =>
  0x78 =>
  0x79 =>
    break;
  0x75 =>
    break;
  0x76 =>
    break;
  0x7a =>
    break;
  0x7b =>
    break;
  0x7c =>
    break;
  0x7d =>
    break;
  0x7e =>
    break;
  0x7f =>
    break;
  0x80 =>
    break;
  0x81 =>
    break;
  0x82 =>
    break;
  0x83 =>
    break;
  0x84 =>
    break;
  0x85 =>
    break;
  0x86 =>
    break;
  0x87 =>
    break;
  0x88 =>
    break;
  0x89 =>
  }
  pcVar1 = load_string_1010_847e
                     (ctx.PTR_LOOP_1050_14cc,(ctx.PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}



pub fn string_1020_c0ca(param_1: u16)
{
  string_1020_c0d8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn string_1020_c0d8(param_1: u16) -> U32Ptr

{
  let mut pcVar1: String; 
  
  if (true) {
    switch(param_1) {
    0x1 =>
      break;
    0x2 =>
      break;
    0x3 =>
      break;
    0x4 =>
      break;
    0x5 =>
      break;
    0x6 =>
      break;
    0x7 =>
      break;
    0x8 =>
      break;
    0x9 =>
      break;
    0xa =>
      break;
    0xb =>
      break;
    0xc =>
      break;
    0xd =>
      break;
    0xe =>
      break;
    0xf =>
      break;
    0x10 =>
      break;
    0x11 =>
      break;
    0x12 =>
      break;
    0x13 =>
      break;
    0x14 =>
      break;
    0x15 =>
      break;
    0x16 =>
      break;
    0x17 =>
      break;
    0x18 =>
      break;
    0x19 =>
      break;
    0x1a =>
      break;
    0x1b =>
      break;
    0x1c =>
      break;
    0x1d =>
      break;
    0x1e =>
      break;
    0x1f =>
      break;
    0x21 =>
      break;
    0x23 =>
      break;
    0x24 =>
    }
  }
  pcVar1 = load_string_1010_847e
                     (ctx.PTR_LOOP_1050_14cc,(ctx.PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn string_op_1020_c222(param_1: u16) -> U32Ptr

{
  let mut pcVar1: String; 
  
  switch(param_1) {
  0x1 =>
    break;
  0x2 =>
    break;
  0x3 =>
    break;
  0x4 =>
    break;
  0x5 =>
    break;
  0x6 =>
    break;
  0x7 =>
    break;
  0x8 =>
    break;
  0x9 =>
    break;
  0xa =>
    break;
  0xb =>
    break;
  0xc =>
    break;
  0xd =>
    break;
  0xe =>
    break;
  0xf =>
    break;
  0x10 =>
    break;
  0x11 =>
    break;
  0x12 =>
    break;
  0x13 =>
    break;
  0x14 =>
  }
  pcVar1 = load_string_1010_847e
                     (ctx.PTR_LOOP_1050_14cc,(ctx.PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}


pub fn pass1_1020_c3ae() -> u16

{
  return 0x1;
}


pub fn pass1_1020_c42e(param_1: i16) -> u16

{
  let u_var1: u16;
  
  if (param_1 == 0xf) {
    u_var1 = 0x1;
  }
  else {
    u_var1 = 0x3;
  }
  return u_var1;
}


pub fn pass1_1020_c47a(param_1: U32Ptr)
{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  *param_1 = 0xc834;
  (param_1 + 0x2) = 0x1020;
  fn_ptr_1000_17ce(ctx, (param_1 + 0x18), 0x1000);
  pass1_1030_1d28(param_1);
  return;
}



pub fn
pass1_1020_c4a8(param_1: u32,param_2: U32Ptr,param_3: U32Ptr,param_4: i16,param_5: u16,
               param_6: u16)

{
  let u_var1: u32;
  let pu_var2: u32;
  let u_var3: u16;
  let u_var4: u16;
  
 // u_var3 = (param_1 >> 0x10);
  if ((param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | u_var3 << 0x10,param_5,param_6);
  }
  u_var1 = (param_1 + 0x18);
 // u_var4 = (u_var1 >> 0x10);
  pu_var2 = (u_var1 + param_4 * 0x6);
  *param_3 = *pu_var2;
  *param_2 = (pu_var2 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_c4f4(param_1: u32,param_2: u16,param_3: u16,param_4: u32,
               param_5: &mut Struct361,param_6: u16)

{
  let paVar1: &mut Struct361;
  let u_var2: u16;
  let u_var3: u16;
  
  pass1_1020_c6de(param_1,param_4);
  u_var3 = param_6 | param_5;
  if (u_var3 != 0x0) {
    paVar1 = param_5;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var2 = pass1_1030_6fa0(CONCAT22(u_var3,paVar1));
    param_5.field_0x4 = (u_var2 * 0x2 + 0x4ea4);
  }
  return;
}



pub fn pass1_1020_c538(param_1: u32) -> u32

{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x12),
                  (param_1 + 0x10));
}



pub fn pass1_1020_c54a(param_1: u32,param_2: i16,param_3: u16)
{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  if ((param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | u_var1 << 0x10,param_2,param_3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_c5b8(param_1: u16,param_2: u16,param_3: i16,param_4: u16)
{
  plVar1: &i32;
  let u_var2: u32;
  let ppc_var3: u32;
  let puVar4: u32;
  let u_var5: u16;
  let extraout_dx: u16;
  let u_var6: u16;
  let iVar7: i16;
  let uVar8: u16;
  
  u_var2 = (param_3 + 0xa);
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
  u_var5 = pass1_1030_6fa0(CONCAT22(param_2,param_1));
  (param_3 + -0x6) = u_var5;
  pass1_1020_c6de((param_3 + 0x6),0x0);
  (param_3 + -0xa) = u_var5;
  (param_3 + -0x8) = param_2;
  if ((param_2 | u_var5) == 0x0) {
    ppc_var3 = ((param_3 + 0x6) + 0x20);
    (**ppc_var3)();
    u_var6 = extraout_dx;
    pass1_1020_c6de((param_3 + 0x6),0x0);
    (param_3 + -0xa) = u_var5;
    (param_3 + -0x8) = u_var6;
    if ((u_var6 | u_var5) == 0x0) {
      return;
    }
  }
  u_var2 = (param_3 + 0x6);
 // uVar8 = (u_var2 >> 0x10);
  iVar7 = u_var2;
  (iVar7 + 0x1c) = 0x1;
  plVar1 = (iVar7 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  puVar4 = (param_3 + -0xa);
  *puVar4 = (param_3 + 0xa);
  (puVar4 + 0x4) =
       ((param_3 + -0x6) * 0x2 + 0x4ea4);
  return;
}



pub fn pass1_1020_c644(param_1: U32Ptr,param_2: u16,param_3: u32)
{
  plVar1: &i32;
  let u_var2: u16;
  let ppc_var3: u32;
  let i_var4: i16;
  let iVar5: i16;
  let u_var6: u16;
  let puStack6: u32;
  
 // u_var6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x18) == 0x0) {
    ppc_var3 = (*param_1 + 0x20);
    (**ppc_var3)();
  }
  i_var4 = (iVar5 + 0x8) * 0x6 + (iVar5 + 0x18);
  u_var2 = (iVar5 + 0x1a);
  puStack6 = CONCAT22(u_var2,i_var4);
  plVar1 = (iVar5 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  *puStack6 = param_3;
  (i_var4 + 0x4) = param_2;
  return;
}



pub fn pass1_1020_c694(param_1: u32,param_2: i16,param_3: u16)
{
  pass1_1020_c6a4(param_1,param_2,param_3);
  return;
}



pub fn pass1_1020_c6a4(param_1: u32,param_2: i16,param_3: u16)
{
  let lVar1: i32;
  let i_var2: &mut Struct359;
  let u_var2: u16;
  
 // u_var2 = (param_1 >> 0x10);
  i_var2 = param_1;
  if ((i_var2.field_0x18 != 0x0) && (i_var2.field_0x8 != 0x0)) {
    lVar1 = i_var2.field_0x18;
    pass1_1000_4aea(lVar1,(lVar1 >> 0x10),i_var2.field_0x10,0x6,
                    0xc7fa,&stack0xfffe,param_2,u_var2,0x1000,param_3);
    i_var2.field_0x1c = 0x0;
  }
  return;
}



pub fn pass1_1020_c6de(param_1: u32,param_2: i32)
{
  let pu_var1: u32;
  let u_var2: u32;
  let iVar3: &mut Struct360;
  let unaff_DI: i16;
  let u_var3: u16;
  let unaff_SS: u16;
  let uStack6: u32;
  
 // u_var3 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (iVar3.field_0x1c != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | u_var3 << 0x10,unaff_DI,unaff_SS);
  }
  uStack6 = 0x0;
  loop {
    pu_var1 = &iVar3.field_0x10;
    if (*pu_var1 < uStack6 || *pu_var1 == uStack6) {
      return;
    }
    u_var2 = iVar3.field_0x18;
    if ((u_var2 + uStack6 * 0x6) == param_2) { break; }
    uStack6 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_c73a(param_1: u32,param_2: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u16;
  let u_var3: u32;
  let u_var4: u16;
  let iVar5: i16;
  let u_var6: u16;
  let lVar7: i32;
  let uStack10: u32;
  let uStack6: u32;
  
 // u_var6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x10) == 0x0) {
    u_var4 = (iVar5 + 0xc);
    ctx.PTR_LOOP_1050_5f2e = (iVar5 + 0xe);
  }
  else {
    u_var2 = (iVar5 + 0x10);
    pu_var1 = (iVar5 + 0x14);
    u_var4 = u_var2 + *pu_var1;
    ctx.PTR_LOOP_1050_5f2e =
         
         ((iVar5 + 0x12) + (iVar5 + 0x16) + CARRY2(u_var2,*pu_var1));
  }
  uStack6 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,u_var4);
  if ((iVar5 + 0x18) == 0x0) {
    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
      ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    u_var4 = fn_ptr_op_1000_1708(u_var4 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    u_var3 = (iVar5 + 0x18);
    lVar7 = pass1_1000_0ed4(
        ctx, 0x1000, param_2, 0x1, u_var4 * 0x6,
        (ctx.PTR_LOOP_1050_5f2e * 0x3 + CARRY2(u_var4,u_var4) +
        CARRY2(u_var4 * 0x2,u_var4)) * 0x2 +
        CARRY2(u_var4 * 0x3,u_var4 * 0x3), u_var3,
        (u_var3 >> 0x10));
    ctx.PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
    u_var4 = lVar7;
  }
  uStack10 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,u_var4);
  if ((ctx.PTR_LOOP_1050_5f2e | u_var4) != 0x0) {
    (iVar5 + 0x10) = uStack6;
    (iVar5 + 0x18) = uStack10;
  }
  (iVar5 + 0x1c) = 0x1;
  return;
}



i16  pass1_1020_c7fa(param_1: u32,param_2: u32)

{
  return (param_1 + 0x4) - (param_2 + 0x4);
}



astruct_18 *  pass1_1020_c80e(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_c47a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1020_c860(param_1: u32) -> u32

{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x6),(param_1 + 0x4))
  ;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_c872(param_1: u32,param_2: u32,param_3: u32)
{
  let pu_var1: U32Ptr;
  let pu_var2: u32;
  let piVar3: U32Ptr;
  let u_var4: &mut Struct98;
  let u_var6: u16;
  let iVar7: i16;
  let i_var8: i16;
  let uVar9: u16;
  let u_var10: u16;
  let u_var11: u16;
  let bVar12: bool;
  let uStack14: u32;
  let uStack10: u32;
  let puStack6: &mut Struct99;
  let u_var5: &mut Struct99;
  
  puStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_4fb8);
 // u_var6 = (puStack6 >> 0x10);
  u_var5 = puStack6;
  if ((u_var6 | u_var5) == 0x0) {
    puStack6 = 0x0;
  }
  else {
    puStack6.field_0x0 = 0x389a;
    u_var5.field_0x2 = 0x1008;
    u_var5.field_0x4 = 0x0;
    u_var5.field_0x8 = 0x0;
    puStack6.field_0x0 = 0x5bc0;
    u_var5.field_0x2 = 0x1008;
    u_var5.field_0xe = 0x0;
    u_var5.field_0xc = 0x0;
    puStack6.field_0x0 = 0xc9e6;
    u_var5.field_0x2 = 0x1020;
  }
  if (puStack6 == 0x0) {
    return;
  }
 // uVar9 = (puStack6 >> 0x10);
  iVar7 = puStack6;
  (iVar7 + 0x8) = param_3;
  (iVar7 + 0xc) = param_2;
 // u_var10 = (param_1 >> 0x10);
  i_var8 = param_1;
  uStack14 = (i_var8 + 0x4);
  u_var11 = (i_var8 + 0x6);
  if ((i_var8 + 0x8) == 0x0) {
//LAB_1020_c92d:
    (iVar7 + 0x4) = (i_var8 + 0x4);
  }
  else {
    pu_var1 = (uStack14 + 0xe);
    bVar12 = *pu_var1 < param_2._2_2_;
    if ((bVar12 || *pu_var1 == param_2._2_2_) &&
       ((bVar12 ||
        (pu_var1 = (uStack14 + 0xc),
        *pu_var1 < param_2 || *pu_var1 == param_2)))) {
        // goto
        // LAB_1020_c92d;
    }
    bVar12 = false;
    loop {
      if (uStack14 == 0x0) { break; }
     // u_var11 = (uStack14 >> 0x10);
      pu_var2 = (uStack14 + 0xc);
      if (*pu_var2 < param_2 || *pu_var2 == param_2) {
        bVar12 = true;
        (iVar7 + 0x4) = uStack14;
        (uStack10 + 0x4) = puStack6;
        break;
      }
      uStack10 = uStack14;
      uStack14 = (uStack14 + 0x4);
    }
    param_1 = uStack10;
    if (bVar12) {
        // goto
        // LAB_1020_c9ab;
    }
  }
 // u_var11 = (param_1 >> 0x10);
  (param_1 + 0x4) = iVar7;
  (param_1 + 0x6) = uVar9;
//LAB_1020_c9ab:
  piVar3 = (i_var8 + 0x8);
  *piVar3 = *piVar3 + 0x1;
  return;
}



pub fn pass1_1020_c9ba(param_1: U32Ptr,param_2: u8) -> u16

{
  let u_var1: u16;
  
 // u_var1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  (param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a(param_1,u_var1,0x1000);
  }
  return param_1;
}



u16 * 
pass1_1020_ca0c(param_1: &mut Struct179,param_2: u16,param_3: i16,param_4: u32,
               param_5: u16)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xcc7c;
  param_1.field_0x2 = 0x1020;
  return CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_ca36(param_1: i16,param_2: u16,param_3: u16,param_4: i16,param_5: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u32;
  let pu_var3: U32Ptr;
  
  pass1_1028_09b8(CONCAT22(param_2,param_1));
  u_var2 = pass1_1028_b4f2(CONCAT22(param_2,param_1));
 // pu_var1 = (u_var2 >> 0x10);
  if ((u_var2 + 0x200) != 0x8000002) {
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x8,param_5,pu_var1,param_4);
    pass1_1010_988c(pu_var3,(param_1 + 0xc));
  }
  return;
}



pub fn pass1_1020_ca82(param_1: U32Ptr,param_2: u16,param_3: u16,param_4: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u16;
  let u_var3: u32;
  
  pass1_1028_be9e(param_1,param_2,param_3,&USHORT_1050_1028,param_4);
  u_var3 = pass1_1028_b4f2(param_1);
 // pu_var1 = (u_var3 >> 0x10);
  if ((u_var3 + 0x200) != 0x8000002) {
   // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
      pass1_1020_cac2(param_1 & 0xffff | u_var2 << 0x10,pu_var1,param_2,
                      param_3,param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_cac2(param_1: u32,param_2: U32Ptr,param_3: u16,param_4: u16,param_5: u16)
{
  let pi_var1: U32Ptr;
  let ppcVar2: u32;
  let pu_var3: U32Ptr;
  let puVar4: U32Ptr;
  let u_var5: u16;
  let iVar6: i16;
  let extraout_dx: u16;
  let extraout_DX_00: u16;
  let uVar7: u16;
  let puVar8: U32Ptr;
  let iStack52: i16;
  let puStack36: U32Ptr;
  let local_1c: [u8;4];
  let uStack24: u32;
  let puStack20: u32;
  let puStack16: u32;
  let puStack12: U32Ptr;
  let puStack8: U32Ptr;
  let uStack6: u16;
  let puStack4: U32Ptr;
  
  puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2,param_5,param_2,param_4);
 // puStack4 = (puVar8 >> 0x10);
  uStack6 = SUB42(puVar8,0x0);
  puStack8 = ctx.PTR_LOOP_1050_13ae;
  if (ctx.PTR_LOOP_1050_13ae == (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
    puStack8 = &ctx.PTR_LOOP_1050_0002;
  }
  puStack12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x8,param_5,puStack4,param_4);
 // uVar7 = (puStack12 >> 0x10);
  puStack16 = (puStack12 + 0xa);
  puStack20 = (puStack12 + 0xe);
  pass1_1008_5784(CONCAT22(param_5,local_1c),puStack16);
  loop {
    loop {
      loop {
        loop {
          pu_var3 = local_1c;
          pass1_1008_5b12(pu_var3,param_5);
          if ((extraout_dx | pu_var3) == 0x0) {
            return;
          }
          iVar6 = (pu_var3 + 0x4);
            if ((iVar6 < 0x12) || (SBORROW2(iVar6,0x12))) == false { break;}
        }
        if (iVar6 != 0x13 && 0x0 < iVar6 + -0x12) { break; }
        iStack52 = 0x0;
        if (puStack8 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
          iStack52 = (pu_var3 + 0x6) / 0x2;
        }
        else {
          if (puStack8 == &DAT_1050_0004) {
            iVar6 = (pu_var3 + 0x6) * 0x3;
            iStack52 = (iVar6 + (iVar6 >> 0xf & 0x3)) >> 0x2;
          }
        }
        pi_var1 = (pu_var3 + 0x6);
        *pi_var1 = *pi_var1 - iStack52;
       // uVar7 = (puStack16 >> 0x10);
        (puStack16 + 0xa) = 0x0;
        ppcVar2 = (*puStack16 + 0xc);
        (**ppcVar2)(0x1008,puStack16,uVar7,pu_var3,extraout_dx);
        (puStack16 + 0xa) = 0x1;
        uStack24 = 0x0;
        ppcVar2 = (*puStack20 + 0x4);
        (**ppcVar2)(0x1008,puStack20,(puStack20 >> 0x10),pu_var3,
                    extraout_dx);
      }
        if (iVar6 != 0x81) == false { break;}
    }
    puStack36 = 0x0;
    if (puStack8 == &ctx.PTR_LOOP_1050_0002) {
      iVar6 = (pu_var3 + 0x6);
//LAB_1020_cba7:
      puVar4 = ((iVar6 + (iVar6 >> 0xf & 0x3)) >> 0x2);
      puStack36 = puVar4;
    }
    else {
      if (puStack8 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
        puVar4 = ((pu_var3 + 0x6) / 0x2);
        puStack36 = puVar4;
      }
      else {
        puVar4 = puStack8 + -0x4;
        if (puVar4 == 0x0) {
          iVar6 = (pu_var3 + 0x6) * 0x3;
//           TODO: goto LAB_1020_cba7;
        }
      }
    }
    pass1_1028_b58e(param_1);
    u_var5 = (pu_var3 + 0x6) - puStack36;
    pass1_1030_7ddc(CONCAT13((extraout_DX_00 >> 0x8),
                             CONCAT12(extraout_DX_00,puVar4)),u_var5,0x1e,
                    u_var5,(u_var5 >> 0xf),param_3,param_4,param_5);
    ppcVar2 = (*puStack16 + 0xc);
    (**ppcVar2)(0x1030,puStack16,(puStack16 >> 0x10),pu_var3,extraout_dx)
    ;
    uStack24 = 0x0;
  }
}



astruct_18 *  pass1_1020_cc56(param_1: &mut Struct18,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1020_cd06(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xcd7e;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



pub fn pass1_1020_cd30(param_1: u32) -> u16

{
  let i_var1: i16;
  let u_var2: u16;
  
 // u_var2 = (param_1 >> 0x10);
  i_var1 = param_1;
  if ((((i_var1 + 0x12) == 0x6) || ((i_var1 + 0x12) == 0x5)) &&
     (((i_var1 + 0x1a) & 0x2) != 0x0)) {
    return 0x1;
  }
  return 0x0;
}



astruct_18 *  pass1_1020_cd58(param_1: &mut Struct18,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


u16 * 
pass1_1020_ce08(param_1: &mut Struct179,param_2: u16,param_3: i16,param_4: u32,
               param_5: u16)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd004;
  param_1.field_0x2 = 0x1020;
  return CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_ce32(param_1: i16,param_2: u16,param_3: i16,param_4: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u32;
  let pu_var3: U32Ptr;
  let paVar4: &mut Struct67;
  let u_var5: u16;
  let u_var6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let u_var10: u16;
  let iVar11: i16;
  
  pass1_1028_09b8(CONCAT22(param_2,param_1));
  u_var2 = pass1_1028_b4f2(CONCAT22(param_2,param_1));
 // pu_var1 = (u_var2 >> 0x10);
  if ((u_var2 + 0x200) != 0x8000002) {
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x8,param_4,pu_var1,param_3);
   // pu_var1 = (pu_var3 >> 0x10);
    pass1_1010_988c(pu_var3,(param_1 + 0xc));
    u_var10 = 0x0;
    iVar11 = 0x9;
    uVar8 = 0x1;
    uVar9 = 0x0;
    u_var6 = 0x0;
    iVar7 = 0x0;
    u_var5 = 0x0;
    paVar4 = 
             mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x37,param_4,pu_var1,param_3);
    post_win_msg_1008_a0e4
              (paVar4,CONCAT22(u_var6,u_var5),iVar7,uVar8,CONCAT22(u_var10,uVar9),iVar11,
               0x1008,param_4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_ce9e(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u32;
  let pu_var3: U32Ptr;
  
  pass1_1028_be9e(param_1,param_4,param_2,&USHORT_1050_1028,param_3);
  if ((param_1 + 0x12) == 0x5) {
    pass1_1020_cefc(param_1,param_2,param_3);
    u_var2 = pass1_1028_b4f2(param_1);
   // pu_var1 = (u_var2 >> 0x10);
    if ((u_var2 + 0x200) != 0x8000002) {
      pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2b,param_3,pu_var1,param_2);
      pass1_1010_043a(pu_var3,(u_var2 + 0x4),0x5,param_3);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_cefc(param_1: u32,param_2: i16,param_3: u16)
{
  let pu_var1: U32Ptr;
  let u_var2: u32;
  let pu_var3: U32Ptr;
  let uStack12: u16;
  
  u_var2 = pass1_1028_b4f2(param_1);
 // pu_var1 = (u_var2 >> 0x10);
  if ((u_var2 + 0x200) == 0x8000002) {
    uStack12 = 0x32;
  }
  else {
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x8,param_3,pu_var1,param_2);
    uStack12 = pass1_1010_96c2(pu_var3);
    if (0x32 < uStack12) {
      uStack12 = 0x32;
    }
    pass1_1010_96a8(pu_var3,uStack12);
  }
  pass1_1020_cf6c(param_1,(param_1 >> 0x10),uStack12,u_var2);
  return;
}



pub fn pass1_1020_cf6c(param_1: u16,param_2: u16,param_3: i16,param_4: u32)
{
  let pu_var1: U32Ptr;
  let piVar2: U32Ptr;
  let u_var3: u16;
  let u_var4: u32;
  let u_var5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let i_var9: i16;
  let u_var10: u16;
  
 // u_var10 = (param_4 >> 0x10);
  u_var4 = (param_4 + 0x1f6);
  iVar6 = u_var4;
 // u_var5 = (u_var4 >> 0x10);
  uVar7 = param_3 / 0x5;
  uVar8 = uVar7 * -0x4 + param_3;
  pu_var1 = (iVar6 + 0x50);
  u_var3 = *pu_var1;
  *pu_var1 = *pu_var1 + uVar8;
  piVar2 = (iVar6 + 0x52);
  *piVar2 = *piVar2 + (uVar8 >> 0xf) + CARRY2(u_var3,uVar8);
  i_var9 = uVar7 >> 0xf;
  pu_var1 = (iVar6 + 0x78);
  u_var3 = *pu_var1;
  *pu_var1 = *pu_var1 + uVar7;
  piVar2 = (iVar6 + 0x7a);
  *piVar2 = *piVar2 + i_var9 + CARRY2(u_var3,uVar7);
  pu_var1 = (iVar6 + 0xa0);
  u_var3 = *pu_var1;
  *pu_var1 = *pu_var1 + uVar7;
  piVar2 = (iVar6 + 0xa2);
  *piVar2 = *piVar2 + i_var9 + CARRY2(u_var3,uVar7);
  pu_var1 = (iVar6 + 0xc8);
  u_var3 = *pu_var1;
  *pu_var1 = *pu_var1 + uVar7;
  piVar2 = (iVar6 + 0xca);
  *piVar2 = *piVar2 + i_var9 + CARRY2(u_var3,uVar7);
  pu_var1 = (iVar6 + 0xf0);
  u_var3 = *pu_var1;
  *pu_var1 = *pu_var1 + uVar7;
  piVar2 = (iVar6 + 0xf2);
  *piVar2 = *piVar2 + i_var9 + CARRY2(u_var3,uVar7);
  (param_4 + 0x1fe) = 0x1;
  return;
}



astruct_18 *  pass1_1020_cfde(param_1: &mut Struct18,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_d08e(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd314;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



pub fn pass1_1020_d0b8(param_1: U32Ptr,param_2: u16,param_3: u16,param_4: u16)
{
  let u_var1: u32;
  let i_var2: i16;
  
  if ((param_1 + 0x12) != 0x6) {
    return;
  }
  u_var1 = pass1_1028_b4f2(param_1);
  i_var2 = u_var1;
  if ((i_var2 + 0x200) != 0x8000002) {
    pass1_1028_cb04(param_1,param_2,param_3,param_4);
    if ((i_var2 == 0x0) || (pass1_1020_d194(param_1,param_3,param_4), i_var2 == 0x0))
    {
      i_var2 = 0x6;
//       TODO: goto LAB_1020_d10b;
    }
    pass1_1028_c952(param_1,param_2,param_3,param_4);
  }
  i_var2 = 0x5;
//LAB_1020_d10b:
  pass1_1028_bdac(param_1,i_var2,&USHORT_1050_1028);
  return;
}



u16 
pass1_1020_d118(param_1: u32,param_2: U32Ptr,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16,param_7: u16)

{
  let b_var1: bool;
  let u_var2: u16;
  let u_var3: u16;
  
  u_var2 = param_1;
 // u_var3 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_6,u_var2,u_var3,param_2,param_4);
  if (param_5 == 0x5) {
    pass1_1028_c23e(u_var2,u_var3,param_2,param_3,param_4,0x5,param_6,param_7);
    if (param_5 != 0x0) {
      pass1_1028_c3aa(u_var2,u_var3,param_2,param_3,param_4,param_7);
      if (param_5 != 0x0) {
        b_var1 = pass1_1028_c314(param_7,param_5,param_6,u_var2,u_var3,param_2,
                                param_3,(param_3 >> 0x10),param_4);
        if (b_var1 != 0x0) {
          return 0x1;
        }
      }
    }
  }
  else {
    ctx.PTR_LOOP_1050_50ca = 0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_d194(param_1: u32,param_2: i16,param_3: u16)
{
  let ppcVar1: u32;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let u_var4: u16;
  let u_var5: u16;
  let u_var6: u32;
  let uVar7: u16;
  let extraout_dx: U32Ptr;
  let puVar8: U32Ptr;
  let puVar9: U32Ptr;
  let extraout_DX_00: u16;
  let u_var10: u16;
  let u_var11: u16;
  let uVar12: u32;
  let puVar13: U32Ptr;
  let puVar14: u32;
  let uVar15: u8;
  let uVar16: u8;
  let puVar17: U32Ptr;
  let uVar18: u16;
  let uVar19: u16;
  let uStack42: u32;
  let uStack38: u32;
  let puStack34: u32;
  let local_4: [u8;2];
  
  pass1_1030_bcae(local_4,param_3);
  uVar12 = pass1_1028_b4f2(param_1);
 // uVar7 = (uVar12 >> 0x10);
  u_var6 = (uVar12 + 0x10);
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var6);
  u_var2 = u_var6;
  pass1_1028_b58e(param_1);
  pu_var3 = local_4;
  puVar8 = extraout_dx;
  pass1_1030_bd74(pu_var3,param_3,u_var6 & 0xffff | uVar7 << 0x10,
                  CONCAT22(extraout_dx,u_var2),param_3);
  if (pu_var3 < 0x0) {
    return;
  }
  if (0x1e < pu_var3) {
    u_var4 = 0x87;
    puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x9,param_3,puVar8,param_2);
    u_var4 = pass1_1010_65d0(param_3,puVar13,u_var4);
    if (u_var4 == 0x0) {
      puVar14 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0,0x15);
     // puVar9 = (puVar14 >> 0x10);
      uVar7 = puVar14;
      u_var11 = SUB42(&ctx.PTR_LOOP_1050_1038,0x0);
      pass1_1038_4e78(uVar7,puVar9,uVar12,puVar14);
      puStack34 = CONCAT22(puVar9,uVar7);
      ppcVar1 = (*puStack34 + 0x10);
      u_var10 = uVar7;
      uVar18 = uVar7;
      puVar8 = puVar9;
      (**ppcVar1)(&ctx.PTR_LOOP_1050_1038,uVar7,puVar9);
      uStack38 = CONCAT22(extraout_DX_00,u_var10);
      uStack42 = 0x0;
      u_var10 = extraout_DX_00;
      loop {
        if (uStack38 <= uStack42) {
          if (puStack34 == 0x0) {
            return;
          }
          ppcVar1 = *puStack34;
          (**ppcVar1)(u_var11,uVar7,puVar9,0x1,uVar18,puVar8,puStack34,puStack34);
          return;
        }
        uVar15 = u_var2;
        uVar16 = (u_var2 >> 0x8);
        u_var6 = uStack38;
        puVar17 = extraout_dx;
        pass1_1030_1d58(puStack34);
        u_var5 = u_var6;
        pu_var3 = local_4;
        u_var11 = 0x1030;
        uVar19 = u_var10;
        pass1_1030_bd74(pu_var3,param_3,u_var6 & 0xffff | u_var10 << 0x10,
                        CONCAT22(puVar17,CONCAT11(uVar16,uVar15)),param_3);
        if ((0x0 < pu_var3) && (pu_var3 < 0x1f)) { break; }
        uStack42 += 0x1;
      }
      if (puStack34 == 0x0) {
        return;
      }
      ppcVar1 = *puStack34;
      (**ppcVar1)(0x1030,uVar7,puVar9,0x1,uVar18,puVar8,puStack34,puStack34,u_var5,
                  uVar19);
      return;
    }
  }
  return;
}



astruct_18 *  pass1_1020_d2ee(param_1: &mut Struct18,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_d3a4(param_1: U32Ptr,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  let u_var1: u16;
  
  pass1_1028_b39e(param_1,param_3,param_4,param_5);
 // u_var1 = (param_1 >> 0x10);
  (param_1 + 0x20) = param_2;
  *param_1 = 0xd53e;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}


pub fn pass1_1020_d41a(param_1: u32,param_2: u32,param_3: bool,param_4: U32Ptr,param_5: u16) -> bool

{
  let b_var1: bool;
  let local_4: u16;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    b_var1 = read_file_1008_7dee(param_2,(param_2 >> 0x10),&local_4
                                ,0x0,param_5,0x2,0x1008);
    if (b_var1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return b_var1;
    }
    (param_1 + 0x20) = local_4;
    param_3 = 0x1;
  }
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1020_d460(param_1: U32Ptr,param_2: U32Ptr,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16,param_7: i16,undefined8 param_8)

{
  let u_var1: u16;
  let pu_var2: U32Ptr;
  let unaff_SS: u16;
  let u_var3: u32;
  let puVar4: U32Ptr;
  
  u_var1 = pass1_1028_bc90(param_1,param_2,param_3,param_4,param_5,param_6,unaff_SS);
  if (u_var1 != 0x0) {
    u_var3 = pass1_1038_af40(ctx.PTR_LOOP_1050_5b7c,
                            (ctx.PTR_LOOP_1050_4230 + 0x16),0x11,param_6,
                            ctx._PTR_LOOP_1050_4230,&ctx.PTR_LOOP_1050_1038,
                            unaff_SS);
   // pu_var2 = (u_var3 >> 0x10);
    ctx.PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5b80,0x1008,unaff_SS);
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x3a,unaff_SS,pu_var2,param_7);
    (param_1 + 0x20) = (puVar4 + 0xa);
    u_var1 = 0x1;
  }
  return u_var1;
}



pub fn pass1_1020_d4ca(param_1: u32,param_2: i16)
{
  let b_var1: bool;
  let u_var2: u32;
  let extraout_dx: u16;
  let u_var3: u16;
  let i_var4: i16;
  
  if ((param_1 + 0x20) == 0x2) {
    return;
  }
  pass1_1028_b58e(param_1);
  u_var2 = (param_2 + 0x2e);
  i_var4 = 0x63;
  u_var3 = extraout_dx;
  pass1_1038_3fb0(u_var2);
  b_var1 = pass1_1030_25b2(u_var2 & 0xffff | u_var3 << 0x10,i_var4);
  if (b_var1 != 0x0) {
    return;
  }
  return;
}



astruct_18 *  pass1_1020_d518(param_1: &mut Struct18,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1020_d5c8(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd7fe;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_d5f2(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  let ppcVar1: u32;
  let u_var2: u16;
  let pu_var3: u32;
  let extraout_dx: u16;
  let u_var4: u16;
  let u_var5: u32;
  let puVar6: u32;
  let puVar7: u32;
  let bStack55: u8;
  u8 local_32 [0xa];
  let uStack40: u32;
  let uStack36: u32;
  let puStack28: u32;
  let local_1a: u32;
  let iStack22: i16;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  let iStack14: i16;
  let local_c: u32;
  let iStack8: i16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_b58e(param_1);
  local_c = (param_3 + 0xc);
  iStack18 = (param_3 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_dx;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_3;
  uStack4 = extraout_dx;
  pass1_1028_bab6(param_1,iStack18,extraout_dx);
  u_var2 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack14 += 0x1;
  uStack20 = u_var2;
  if (iStack14 < u_var2) {
    puVar7 = CONCAT22(param_4,local_32);
    iStack22 = iStack14;
    u_var5 = pass1_1028_bb24(param_1);
   // u_var4 = (u_var5 >> 0x10);
    pu_var3 = &local_1a;
    pass1_1030_64ce(param_4,pu_var3,u_var4,_PTR_LOOP_1050_5740,
                    CONCAT22(param_4,pu_var3),
                    u_var5 & 0xffff | u_var4 << 0x10,puVar7);
    uStack40 = *pu_var3;
    u_var4 = (pu_var3 + 0x2);
    bStack55 = (uStack40 >> 0x18);
    u_var2 = bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0x0) {
      pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack40);
      puVar6 = struct_op_1030_73a8(CONCAT22(u_var4,u_var2));
      u_var2 = puVar6;
      ppcVar1 = (*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(param_1,param_2,u_var2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_d6e6(param_1: u32,param_2: i16,param_3: u16)
{
  let ppcVar1: u32;
  let pu_var2: u32;
  let u_var3: u16;
  let extraout_dx: u16;
  let u_var4: u16;
  let u_var5: u32;
  let puVar6: u32;
  let puVar7: u32;
  let bStack55: u8;
  u8 local_32 [0xa];
  let uStack40: u32;
  let uStack36: u32;
  let puStack28: u32;
  let local_1a: u32;
  let iStack22: i16;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  let iStack14: i16;
  let local_c: u32;
  let iStack8: i16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_b514(param_1);
  pass1_1028_b58e(param_1);
  local_c = (param_2 + 0xc);
  iStack18 = (param_2 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_dx;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_2;
  uStack4 = extraout_dx;
  pass1_1028_bab6(param_1,iStack18,extraout_dx);
  uStack20 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack22 = iStack14 + 0x1;
  if (iStack22 < uStack20) {
    puVar7 = CONCAT22(param_3,local_32);
    iStack14 = iStack22;
    u_var5 = pass1_1028_bb24(param_1);
   // u_var4 = (u_var5 >> 0x10);
    pu_var2 = &local_1a;
    pass1_1030_64ce(param_3,pu_var2,u_var4,_PTR_LOOP_1050_5740,
                    CONCAT22(param_3,pu_var2),
                    u_var5 & 0xffff | u_var4 << 0x10,puVar7);
    uStack40 = *pu_var2;
    u_var4 = (pu_var2 + 0x2);
    bStack55 = (uStack40 >> 0x18);
    u_var3 = bStack55;
    if (bStack55 != 0x0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack40);
      puVar6 = struct_op_1030_73a8(CONCAT22(u_var4,u_var3));
      if ((puVar6 + 0xc) == 0x6a) {
        ppcVar1 = (*puVar6 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



astruct_18 *  pass1_1020_d7d8(param_1: &mut Struct18,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_d888(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd8ec;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



pub fn pass1_1020_d8ca(param_1: i16,param_2: u16) -> u32

{
  pass1_1028_b418((param_1 + 0x6));
  if (((param_1 + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
  }
  return CONCAT22((param_1 + 0x8),(param_1 + 0x6));
}


pub fn pass1_1020_d9fa(param_1: u32,param_2: u16)
{
  let extraout_dx: u16;
  
  if ((param_1 + 0xc) != 0x79) {
    pass1_1030_df0c(param_1,param_2);
    pass1_1028_b58e(param_1);
    pass1_1038_57dc((param_2 + 0x2e),0x1,0x2);
  }
  return;
}



pub fn pass1_1020_da3c(param_1: U32Ptr)
{
  pass1_1028_bdac(param_1,0x2,&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_da4e(param_1: U32Ptr,param_2: U32Ptr,param_3: u32,param_4: u32,param_5: u16,
               param_6: i16,param_7: u16)

{
  let ppcVar1: u32;
  let pu_var2: u32;
  let u_var3: u16;
  let Bvar4: bool;
  let extraout_dx: U32Ptr;
  let pu_var5: U32Ptr;
  let extraout_DX_00: U32Ptr;
  let u_var6: u16;
  let uVar7: u32;
  let uVar8: u32;
  let uVar9: u16;
  let u_var11: u16;
  let puVar10: U32Ptr;
  let uVar12: u32;
  let bStack31: u8;
  let local_e: u32;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u32;
  
  pu_var2 = &local_e;
  pass1_1030_64ce(param_7,pu_var2,param_5,_PTR_LOOP_1050_5740,param_2,param_4,
                  CONCAT22(param_7,pu_var2));
  uStack6 = *pu_var2;
  u_var6 = (pu_var2 + 0x2);
  bStack31 = (uStack6 >> 0x18);
  u_var3 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
    uVar7 = struct_op_1030_73a8(CONCAT22(u_var6,u_var3));
   // u_var6 = (uVar7 >> 0x10);
    u_var3 = uVar7;
    if ((u_var3 + 0xc) == 0x10) {
      ctx.PTR_LOOP_1050_50ca = 0x6a9;
      return;
    }
  }
  uVar9 = param_1;
 // u_var11 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,u_var6,uVar9,u_var11,param_2,param_4);
  uVar8 = param_1 & 0xffff | u_var11 << 0x10;
  ppcVar1 = (*param_1 + 0x60);
  puVar10 = param_2;
  uVar7 = param_3;
  uVar12 = param_4;
  uStack8 = u_var3;
  (**ppcVar1)();
  if (((u_var3 != 0x0) &&
      (pu_var5 = extraout_dx,
      pass1_1028_c23e(uVar9,u_var11,param_2,param_3,param_4,u_var3,extraout_dx,param_7
                     ), u_var3 != 0x0)) &&
     (BVar4 = pass1_1028_c314(param_7,u_var3,pu_var5,uVar9,u_var11,param_2,
                              param_3,(param_3 >> 0x10),param_4),
     BVar4 != 0x0)) {
   // u_var6 = (param_2 >> 0x10);
    if ((((param_2 + 0x4) == 0x0) && (uStack8 != 0x4)) &&
       (ppcVar1 = (*param_1 + 0x5c),
       (**ppcVar1)(&USHORT_1050_1028,param_1,param_2,param_3,param_4,uVar8,puVar10,
                   uVar7,uVar12), pu_var5 = extraout_DX_00, BVar4 == 0x0)) {
      return;
    }
    uStack10 = (param_2 + 0x4);
    if (uStack10 != 0x0) {
      pass1_1020_df10(param_1,
                      (param_2 & 0xffff | u_var6 << 0x10),param_4,
                      uStack10,pu_var5,param_6,param_7);
      return;
    }
    pass1_1020_deac(param_1,
                    (param_2 & 0xffff | u_var6 << 0x10),param_4,0x0
                    ,pu_var5,param_6,param_7);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_db86(param_1: u16,param_2: u16,param_3: U32Ptr,param_4: u32,param_5: i32,
               param_6: u16)

{
  let i_var1: i16;
  let pu_var2: U32Ptr;
  let u_var3: u16;
  let u_var4: u32;
  let pu_var5: U32Ptr;
  let local_4: [u8;2];
  
  u_var4 = pass1_1030_bcae(local_4,param_6);
 // u_var3 = (u_var4 >> 0x10);
  i_var1 = u_var4;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
  u_var4 = (i_var1 + 0x10);
  pu_var5 = param_3;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
  pu_var2 = local_4;
  pass1_1030_bcde(param_6,pu_var2,param_6,u_var4 & 0xffff | u_var3 << 0x10,
                  pu_var5,param_5);
  if (pu_var2 < 0x0) {
    ctx.PTR_LOOP_1050_50ca = 0x6af;
  }
  else {
    if ((pu_var2 < 0x1f) || ((param_3 + 0x4) < 0x1)) {
      return;
    }
    ctx.PTR_LOOP_1050_50ca = 0x6b6;
    ctx.PTR_LOOP_1050_50cc = pu_var2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_dc1c(param_1: u32,param_2: U32Ptr,param_3: u16)
{
  let i_var1: i16;
  let ppcVar2: u32;
  let pu_var3: u32;
  let u_var4: u16;
  let u_var5: u16;
  let u_var6: u32;
  let puVar7: u32;
  let puVar8: u32;
  let bStack27: u8;
  let local_a: [u8;4];
  let uStack6: u32;
  
  puVar8 = CONCAT22(param_3,local_a);
  u_var6 = pass1_1028_bb24(param_1);
 // u_var5 = (u_var6 >> 0x10);
  pu_var3 = u_var6;
  pass1_1030_64ce(param_3,pu_var3,u_var5,_PTR_LOOP_1050_5740,param_2,
                  u_var6 & 0xffff | u_var5 << 0x10,puVar8);
  uStack6 = *pu_var3;
  u_var5 = (pu_var3 + 0x2);
  bStack27 = (uStack6 >> 0x18);
  u_var4 = bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
    puVar7 = struct_op_1030_73a8(CONCAT22(u_var5,u_var4));
    i_var1 = (puVar7 + 0xc);
    if (((i_var1 < 0x1) || (SBORROW2(i_var1,0x1))) ||
       ((i_var1 != 0x9 && 0x7 < i_var1 + -0x1 &&
        ((i_var1 + -0x9 < 0x6a || (0x6 < i_var1 + -0x73)))))) {
      ppcVar2 = (*puVar7 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1020_dca8(param_1: u32,param_2: u16,param_3: u16)
{
  let u_var1: u16;
  let u_var2: u16;
  u8 local_2e [0xe];
  let puStack32: u32;
  let uStack30: u16;
  let uStack28: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u16;
  let uStack20: u16;
  let uStack18: u16;
  let local_10: u32;
  let uStack12: u16;
  let uStack10: u32;
  let local_6: [u8;2];
  let local_4: i16;
  
  pass1_1028_c1f8(param_3,local_6,param_2,param_1,CONCAT22(param_3,local_6)
                  ,CONCAT22(param_3,&local_4));
  uStack10 = pass1_1028_b58e(param_1);
 // u_var1 = (uStack10 >> 0x10);
  local_10 = (uStack10 + 0xc);
  uStack12 = (uStack10 + 0x10);
  puStack32 = &local_10;
  uStack18 = local_10;
 // uStack20 = (local_10 >> 0x10);
  uStack24 = local_10 - 0x1;
  if (uStack24 < 0x0) {
    uStack24 = 0x0;
  }
  u_var2 = local_4 - 0x1;
  uStack26 = local_10 + 0x1;
  if (u_var2 < (local_10 + 0x1)) {
    uStack26 = u_var2;
  }
  uStack28 = uStack20 - 0x1;
  if (uStack28 < 0x0) {
    uStack28 = 0x0;
  }
  uStack30 = uStack20 + 0x1;
  if (u_var2 < (uStack20 + 0x1)) {
    uStack30 = u_var2;
  }
  uStack22 = uStack12;
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack12,uStack28,uStack24);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack28,uStack18);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack28,uStack26);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack20,uStack24);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack20,uStack26);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack30,uStack24);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack30,uStack18);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack30,uStack26);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_de32(param_1: u32,param_2: u16,param_3: U32Ptr,param_4: i16,param_5: u16)
{
  let b_var1: bool;
  let u_var2: u16;
  let u_var3: u16;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  
  pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x5,param_5,param_3,param_4);
 // u_var2 = (pu_var5 >> 0x10);
  (pu_var5 + 0x12) = param_2;
  u_var3 = u_var2;
  b_var1 = bring_win_to_top_1038_b72e(ctx.PTR_LOOP_1050_5b7c,0x4,&ctx.PTR_LOOP_1050_1038);
  if (b_var1 == 0x0) {
    pass1_1038_af40(ctx.PTR_LOOP_1050_5b7c,(ctx.PTR_LOOP_1050_4230 + 0x16),
                    0x4,u_var3,_PTR_LOOP_1050_4230,&ctx.PTR_LOOP_1050_1038,
                    param_5);
  }
  ctx.PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5b80,0x1008,param_5);
 // u_var4 = (param_1 >> 0x10);
  (param_1 + 0x24) = (pu_var5 + 0xa);
  if ((param_1 + 0x24) == 0x0) {
    ctx.PTR_LOOP_1050_50ca = 0x6b2;
  }
  return;
}



bool 
pass1_1020_deac(param_1: u32,param_2: U32Ptr,param_3: i32,param_4: i16,param_5: U32Ptr,
               param_6: i16,param_7: u16)

{
  let u_var1: u16;
  let u_var2: u16;
  
  u_var1 = param_1;
 // u_var2 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_5,u_var1,u_var2,param_2,param_3);
  if (param_4 < 0x1) {
    return 0x0;
  }
  if (SBORROW2(param_4,0x1)) {
    return 0x0;
  }
  if (param_4 != 0x3 && 0x0 < param_4 + -0x2) {
    if (param_4 == 0x4) {
      pass1_1020_de32(param_1,0x4,param_5,param_6,param_7);
      if ((u_var1 + 0x24) == 0x6) {
        return 0x1;
      }
      return 0x0;
    }
    if (param_4 != 0x5) {
      return 0x0;
    }
  }
  (u_var1 + 0x24) = 0x1;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_df10(param_1: u32,param_2: U32Ptr,param_3: i32,param_4: u16,param_5: U32Ptr,
               param_6: i16,param_7: u16)

{
  let pu_var1: u32;
  let u_var2: u16;
  let BVar3: bool;
  let u_var4: u16;
  let u_var5: u32;
  let u_var6: u16;
  let uVar7: u16;
  let bStack31: u8;
  let local_e: u32;
  let uStack10: u32;
  let uStack6: u16;
  let uStack4: u16;
  
  uStack4 = 0x0;
  u_var6 = param_1;
 // uVar7 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_5,u_var6,uVar7,param_2,param_3);
  uStack6 = param_4;
  if (param_4 == 0x0) {
    pu_var1 = &local_e;
    pass1_1030_64ce(param_7,pu_var1,param_5,_PTR_LOOP_1050_5740,param_2,param_3,
                    CONCAT22(param_7,pu_var1));
    uStack10 = *pu_var1;
    u_var4 = (pu_var1 + 0x2);
    bStack31 = (uStack10 >> 0x18);
    u_var2 = bStack31;
    if (bStack31 != 0x0) {
      pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack10);
      u_var5 = struct_op_1030_73a8(CONCAT22(u_var4,u_var2));
      if ((u_var5 + 0xc) == 0x6a) {
        BVar3 = pass1_1020_e044(param_1);
        if (BVar3 == 0x0) {
          (u_var6 + 0x24) = 0x1;
        }
        else {
          ctx.PTR_LOOP_1050_50ca = 0x6ac;
        }
      }
    }
  }
  else {
    if (((0x5 < param_4) && (!SBORROW2(param_4,0x6))) && ((param_4 - 0x6) < 0x4)
       ) {
      pass1_1020_de32(param_1,param_4,param_5,param_6,param_7);
      if (true) {
        switch((u_var6 + 0x24)) {
        0x1 =>
          BVar3 = pass1_1020_e044(param_1);
          if (BVar3 != 0x0) {
            ctx.PTR_LOOP_1050_50ca = 0x6ac;
          }
          break;
        0x2 =>
        0x3 =>
        0x4 =>
        0x5 =>
          pass1_1020_e652(param_1,param_2,(param_2 >> 0x10),
                          param_3,param_7);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_e044(param_1: u32) -> bool

{
  let u_var1: u32;
  let u_var2: u16;
  let u_var3: u16;
  let u_var4: u32;
  
 // u_var3 = (param_1 >> 0x10);
  u_var4 = pass1_1018_04b8((param_1 + 0x28));
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
  u_var2 = pass1_1030_2fac(u_var4);
  u_var1 = (param_1 + 0x28);
  if (u_var2 <= (u_var1 + 0x1e)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_e08e(param_1: u32,param_2: u16,param_3: u16,param_4: u8)
{
  let i_var1: i16;
  let u_var2: u32;
  let iVar3: i16;
  let i_var4: i16;
  let u_var5: u32;
  let extraout_dx: u16;
  let u_var6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let piVar9: U32Ptr;
  let puVar10: U32Ptr;
  let u_var11: u16;
  let uVar12: u16;
  let local_158: u16;
  let uStack342: u16;
  let uStack50: u32;
  let puStack42: u32;
  let local_22: i16;
  let local_20: [u8;2];
  let local_1e: [u8;2];
  let uStack28: u16;
  let piStack26: U32Ptr;
  let local_18: i16;
  let local_16: u16;
  let uStack20: u32;
  let local_10: u32;
  let iStack12: i16;
  let uStack10: u32;
  let uStack6: u32;
  
 // uVar8 = (param_1 >> 0x10);
  uVar7 = param_1;
  iVar3 = (uVar7 + 0xc);
  if (iVar3 == 0x74) {
    i_var1 = (uVar7 + 0x24);
    iVar3 = i_var1 + -0x1;
    if (iVar3 == 0x0) {
        // goto
        // LAB_1020_e0ae;
    }
    iVar3 = i_var1 + -0x6;
    if (iVar3 != 0x0) {
        // goto
        // LAB_1020_e0b9;
    }
    u_var11 = 0x1;
  }
  else {
    if (iVar3 == 0x78) {
      i_var1 = (uVar7 + 0x24);
      i_var4 = i_var1 + -0x1;
      if (i_var4 != 0x0) {
        iVar3 = i_var1 + -0x2;
        if ((0x0 < i_var4) && (!SBORROW2(i_var4,0x1))) {
          if (i_var1 + -0x5 == 0x0 || iVar3 < 0x3) {
            iVar3 = i_var1 + -0x5;
            pass1_1020_e49a(param_1,param_3,param_4);
          }
          else {
            iVar3 = i_var1 + -0x6;
            if (iVar3 == 0x0) {
              pass1_1020_e39c(param_1,0x6,0x0,param_3,param_4);
              pass1_1020_dca8(param_1,param_2,param_3);
            }
          }
        }
//         TODO: goto LAB_1020_e0b9;
      }
      u_var11 = 0x6a;
      iVar3 = i_var4;
    }
    else {
      iVar3 += -0x79;
      if (iVar3 == 0x0) {
        pass1_1020_e49a(param_1,param_3,param_4);
        return;
      }
//LAB_1020_e0ae:
      u_var11 = 0x47;
    }
  }
  pass1_1020_e39c(param_1,u_var11,iVar3,param_3,param_4);
//LAB_1020_e0b9:
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_dx,iVar3);
  u_var5 = (iVar3 + 0x2e);
  u_var6 = (iVar3 + 0x30);
  uStack10 = u_var5;
  if ((uVar7 + 0xc) != 0x79) {
    pass1_1038_5804(u_var5 & 0xffff | u_var6 << 0x10,0x1,0x2);
  }
  if ((uVar7 + 0x24) == 0x6) {
    pass1_1038_43cc(uStack10,(uStack10 >> 0x10),0x1,0x2,u_var5,u_var6);
  }
  local_10 = (uStack6 + 0xc);
  iStack12 = (uStack6 + 0x10);
  puStack42 = &local_10;
  if (((uVar7 + 0x24) == 0x6) && (iStack12 == 0x0)) {
    return;
  }
  u_var2 = (uVar7 + 0x28);
  u_var5 = (u_var2 + 0x20);
  puVar10 = &local_16;
  piStack26 = &local_18;
  piVar9 = piStack26;
  u_var11 = param_3;
  uVar12 = param_3;
  uStack20 = u_var5;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
  uStack28 = u_var5;
  pass1_1030_5b1c(u_var5 & 0xffff | ZEXT24(piStack26) << 0x10,
                  CONCAT22(u_var11,piVar9),CONCAT22(uVar12,puVar10));
  pass1_1028_c8ee(param_3,uVar7,uVar8,(uVar7 + 0x24),
                  CONCAT22(param_3,&local_10));
  pass1_1008_3eb4(CONCAT22(param_3,&local_10),
                  CONCAT22(param_3,&local_22),
                  CONCAT22(param_3,local_20),
                  CONCAT22(param_3,local_1e));
  if ((uVar7 + 0x24) == 0x1) {
    if (local_18 < local_22) {
      pass1_1030_5b3e(CONCAT22(piStack26,uStack28),local_22,local_16);
      pass1_1030_5b1c(CONCAT22(piStack26,uStack28),CONCAT22(param_3,&local_18),
                      CONCAT22(param_3,&local_16));
    }
    uStack50 = (uStack10 + 0x4);
    struct_op_1028_87f0(param_3,param_4,CONCAT22(param_3,&local_158),0x0,0x0
                        ,0x6a,&local_10,param_3,uStack50,uStack20);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_3,&local_158));
    local_158 = 0x389a;
    uStack342 = 0x1008;
  }
  pass1_1028_ccd0(param_4,param_3,param_1,CONCAT22(param_3,&local_10));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_e294(param_1: u32,param_2: u16,param_3: u8)
{
  let u_var1: u32;
  let pu_var2: u32;
  let u_var3: u32;
  let extraout_dx: u16;
  let u_var4: u16;
  let u_var5: u16;
  let u_var6: u16;
  let cStack347: u8;
  u8 local_150 [0xc];
  let puStack324: u32;
  let local_140: [u8;124];
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u32;
  let local_10: u32;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u16;
  let uStack6: u32;
  
 // u_var6 = (param_1 >> 0x10);
  u_var5 = param_1;
  if ((0x1 < (u_var5 + 0x24)) && ((u_var5 + 0x24) < 0x6)) {
    u_var1 = (u_var5 + 0x28);
    u_var3 = (u_var1 + 0x20);
    uStack6 = u_var3;
    pass1_1028_b58e(param_1);
    iStack10 = u_var3;
    local_10 = (iStack10 + 0xc);
    uStack12 = (iStack10 + 0x10);
    puStack324 = &local_10;
    u_var4 = extraout_dx;
    uStack8 = extraout_dx;
    pass1_1028_c8ee(param_2,u_var5,u_var6,(u_var5 + 0x24),
                    CONCAT22(param_2,puStack324));
    pu_var2 = &local_10;
    pass1_1028_c89c(param_1,CONCAT22(param_2,pu_var2),
                    CONCAT22(param_2,local_150),pu_var2,param_2);
    uStack20 = *pu_var2;
    cStack347 = (uStack20 >> 0x18);
    if ((cStack347 == '\0') && (uStack20 == 0x9)) {
      (u_var5 + 0x14) = 0x1;
    }
    uStack24 = (iStack10 + 0x2e);
    uStack28 = (uStack24 + 0x4);
    struct_op_1028_87f0(param_2,param_3,CONCAT22(param_2,local_140),0x0,
                        (u_var5 + 0x14) + 0x1,0x79,&local_10,param_2,uStack28,
                        uStack6);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_2,local_140));
  }
  (u_var5 + 0x26) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_e39c(param_1: u32,param_2: u16,param_3: i16,param_4: u16,param_5: u8)
{
  let u_var1: u32;
  let u_var2: u16;
  let extraout_dx: u16;
  let local_13c: [u8;124];
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  let local_c: u32;
  let iStack8: i16;
  let uStack6: u32;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_dx,param_3);
  local_c = (param_3 + 0xc);
  iStack8 = (param_3 + 0x10);
  if (iStack8 < 0x1) {
    u_var2 = 0x5;
  }
  else {
    u_var2 = 0x6;
  }
  (param_3 + 0x14) = u_var2;
  u_var1 = (param_1 + 0x28);
  uStack16 = (u_var1 + 0x20);
  uStack20 = (param_3 + 0x2e);
  uStack24 = (uStack20 + 0x4);
  struct_op_1028_87f0(param_4,param_5,CONCAT22(param_4,local_13c),0x0,0x1,
                      param_2,&local_c,param_4,uStack24,uStack16);
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_4,local_13c));
  return;
}



pub fn pass1_1020_e44c(param_1: u32,param_2: u16,param_3: u16,param_4: u8)
{
  let pi_var1: U32Ptr;
  let i_var2: i16;
  let u_var3: u16;
  
 // u_var3 = (param_1 >> 0x10);
  i_var2 = param_1;
  if ((i_var2 + 0x12) == 0x2) {
    pi_var1 = (i_var2 + 0x14);
    *pi_var1 = *pi_var1 + -0x1;
    if (((i_var2 + 0x26) == 0x0) && ((i_var2 + 0xc) == 0x78)) {
      pass1_1020_e294(param_1 & 0xffff | u_var3 << 0x10,param_3,param_4);
    }
    if ((i_var2 + 0x14) == 0x0) {
      pass1_1020_e08e(param_1 & 0xffff | u_var3 << 0x10,param_2,param_3,param_4);
      return;
    }
    if ((i_var2 + 0x24) == 0x6) {
      (i_var2 + 0xe) = 0x49;
    }
  }
  return;
}



pub fn pass1_1020_e49a(param_1: u32,param_2: u16,param_3: u8)
{
  let i_var1: i16;
  let i_var2: i16;
  let u_var3: u32;
  let uStack10: u16;
  
  u_var3 = pass1_1028_b58e(param_1);
  i_var1 = (u_var3 + 0x14);
  uStack10 = 0x0;
  i_var2 = i_var1 + -0x6;
  if (i_var2 == 0x0) {
    uStack10 = 0x9;
  }
  else {
    i_var2 = i_var1 + -0x7;
    if (i_var2 == 0x0) {
      uStack10 = 0x6;
    }
    else {
      i_var2 = i_var1 + -0x8;
      if (i_var2 == 0x0) {
        uStack10 = 0x7;
      }
      else {
        i_var2 = i_var1 + -0x9;
        if (i_var2 == 0x0) {
          uStack10 = 0x8;
        }
      }
    }
  }
  pass1_1020_e39c(param_1,uStack10,i_var2,param_2,param_3);
  return;
}



i16  pass1_1020_e4fa(param_1: u32,param_2: u16)

{
  let u_var1: u32;
  let i_stack4: i16;
  
  if (false) {
switchD_1020_e53c_caseD_4:
    u_var1 = pass1_1028_b58e(param_1);
    i_stack4 = (u_var1 + 0x14) + 0x2;
  }
  else {
    switch(param_2) {
    default:
      i_stack4 = 0x4;
      break;
    0x3 =>
    0x8 =>
      i_stack4 = 0x5;
      break;
    default:
//       TODO: goto switchD_1020_e53c_caseD_4;
    }
  }
  return i_stack4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_e558(param_1: u32,param_2: i16,param_3: u16)
{
  let pu_var1: u32;
  let u_var2: u16;
  let iVar3: i16;
  let extraout_dx: u16;
  let u_var4: u16;
  let u_var5: u16;
  let u_var6: u16;
  let uVar7: u16;
  let bStack45: u8;
  u8 local_24 [0xc];
  let uStack24: u32;
  let uStack20: u32;
  let local_10: u32;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
 // uVar7 = (param_1 >> 0x10);
  u_var6 = param_1;
  if ((u_var6 + 0xc) == 0x79) {
    param_2 = (u_var6 + 0x24);
    (u_var6 + 0x14) = param_2;
    (u_var6 + 0x24) = 0x0;
  }
  if ((u_var6 + 0x24) != 0x6) {
    pass1_1028_b58e(param_1);
    uStack8 = (param_2 + 0x14);
    iStack6 = param_2;
    uStack4 = extraout_dx;
    iStack10 = pass1_1020_e4fa(param_1,uStack8);
    local_10 = (iStack6 + 0xc);
    uStack12 = (iStack6 + 0x10);
    uStack24 = CONCAT22(uStack24._2_2_,&local_10);
    u_var4 = uStack4;
    pass1_1028_c8ee(param_3,u_var6,uVar7,(u_var6 + 0x24),
                    CONCAT22(param_3,&local_10));
    pu_var1 = &local_10;
    pass1_1028_c89c(param_1,CONCAT22(param_3,pu_var1),
                    CONCAT22(param_3,local_24),pu_var1,param_3);
    uStack24 = *pu_var1;
    u_var5 = (pu_var1 + 0x2);
    bStack45 = (uStack24 >> 0x18);
    u_var2 = bStack45;
    uStack20._0_2_ = uStack24;
    uStack20 = uStack24;
    if (bStack45 != 0x0) {
      pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack20);
      uStack20._0_2_ = (u_var2 + 0x14);
    }
    uStack8 = uStack20;
    iVar3 = pass1_1020_e4fa(param_1,uStack20);
    (u_var6 + 0x14) = iStack10 + iVar3;
    return;
  }
  (u_var6 + 0x14) = 0x1;
  return;
}



pub fn pass1_1020_e652(param_1: u32,param_2: U32Ptr,param_3: u16,param_4: i32,param_5: u16) -> u32

{
  let pu_var1: u32;
  let u_var2: u16;
  let u_var3: u16;
  let local_8: u32;
  let uStack4: u16;
  
  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
 // u_var3 = (param_1 >> 0x10);
  u_var2 = param_1;
  pass1_1028_c8ee(param_5,u_var2,u_var3,(u_var2 + 0x24),
                  CONCAT22(param_5,&local_8));
  pu_var1 = &local_8;
  pass1_1028_c7b6(param_5,param_3,u_var2,u_var3,CONCAT22(param_5,pu_var1),param_4);
  if (pu_var1 != 0x0) {
    pu_var1 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  }
  return pu_var1;
}


pub fn pass1_1020_e70e(param_1: u32,param_2: u32,param_3: i16,param_4: U32Ptr,param_5: u16)
{
  let u_var1: u16;
  let BVar2: bool;
  let u_var3: u16;
  
  pass1_1030_dec4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
   // u_var1 = (param_1 >> 0x10);
   // u_var3 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(param_2,u_var3,param_1 + 0x24,0x0,u_var1,0x2,
                                0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(param_2,u_var3,param_1 + 0x26,0x0,u_var1,0x2,
                                  0x1008);
      if (BVar2 != 0x0) {
        return;
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



astruct_18 *  pass1_1020_e76c(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  pass1_1030_dcf4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_e81c(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



pub fn pass1_1020_e846(param_1: U32Ptr)
{
  *param_1 = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  pass1_1028_b418(param_1);
  return;
}



astruct_18 *  pass1_1020_e868(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_e846(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


pub fn pass1_1020_e91e(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1030_dcc2(param_1,param_2,param_3,param_4,param_5);
  (param_1 + 0x24) = 0x0;
  CONCAT22(param_2,param_1) = 0xeef6;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



pub fn pass1_1020_e94e(param_1: u32,param_2: u32,param_3: u16) -> bool

{
  let in_AX: bool;
  let b_var1: bool;
  let local_c: [u16;0x5];
  
  pass1_1030_de7c(param_1,param_2,param_3);
  if (in_AX != 0x0) {
    local_c[0] = (param_1 + 0x24);
    b_var1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),local_c,param_3,
                       0x2,0x1008);
    if (b_var1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d0;
      return b_var1;
    }
    in_AX = 0x1;
  }
  return in_AX;
}



pub fn pass1_1020_e994(param_1: u32,param_2: u32,param_3: i16,param_4: U32Ptr,param_5: u16)
{
  let b_var1: bool;
  
  pass1_1030_dec4(param_1,param_2,param_3,param_4,param_5);
  if ((param_3 != 0x0) &&
     (b_var1 = read_file_1008_7dee(param_2,(param_2 >> 0x10),
                                  param_1 + 0x24,0x0,(param_1 >> 0x10),0x2,
                                  0x1008), b_var1 == 0x0)) {
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
  }
  return;
}



pub fn pass1_1020_e9d4(param_1: u16,param_2: u16,param_3: u16)
{
  let extraout_dx: u16;
  
  pass1_1030_df0c(CONCAT22(param_2,param_1),param_3);
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  pass1_1038_57dc((param_3 + 0x2e),0x1,0x1);
  return;
}



pub fn pass1_1020_ea0e(param_1: U32Ptr)
{
  pass1_1028_bdac(param_1,0x1,&USHORT_1050_1028);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1020_ea20(param_1: u32,param_2: U32Ptr,param_3: u32,param_4: u32,param_5: u16,
               param_6: u16,param_7: u8,param_8: u16)

{
  let u_var1: u16;
  let ppcVar2: u32;
  let u_var3: u16;
  let cVar4: u8;
  let pu_var5: u32;
  let u_var6: u16;
  let puVar7: U32Ptr;
  let extraout_dx: U32Ptr;
  let unaff_DI: i16;
  let uVar8: u16;
  let uVar9: u16;
  u8 local_146 [0x10c];
  let uStack58: u16;
  let puStack56: U32Ptr;
  let uStack50: u32;
  let puStack46: U32Ptr;
  let puStack42: u32;
  let uStack38: u32;
  let uStack34: u32;
  let uStack28: u32;
  let local_12: u32;
  let iStack14: i16;
  let puStack12: u32;
  let uStack8: u32;
  let BStack4: bool;
  
  uVar9 = param_1;
 // u_var3 = (param_1 >> 0x10);
  pass1_1028_c3aa(uVar9,u_var3,param_2,param_3,param_4,param_6);
  if (param_5 == 0x0) {
    return;
  }
  pass1_1028_c23e(uVar9,u_var3,param_2,param_3,param_4,param_5,param_8,param_6);
  if (param_5 == 0x0) {
    return;
  }
  BStack4 = pass1_1028_c314(param_6,param_5,param_8,uVar9,u_var3,param_2,param_3,
                            (param_3 >> 0x10),param_4);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_6,param_8,uVar9,u_var3,param_2,param_4);
  if ((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9)) {
    ctx.PTR_LOOP_1050_50ca = 0x6a8;
    return;
  }
  if (BStack4 != 0x0) {
    return;
  }
  pu_var5 = &local_12;
  pass1_1030_64ce(param_6,pu_var5,param_8,_PTR_LOOP_1050_5740,param_2,param_4,
                  CONCAT22(param_6,pu_var5));
  uStack38 = *pu_var5;
  puStack56 = (pu_var5 + 0x2);
  uStack38._3_1_ = (uStack38 >> 0x18);
  uStack58 = uStack38._3_1_;
  uStack28 = uStack38;
  uStack8 = uStack38;
  if (uStack38._3_1_ == 0x0) {
      // goto
      // LAB_1020_eb4e;
  }
  uStack8._0_2_ = uStack38;
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack8);
  uStack38 = CONCAT22(puStack56,uStack58);
  uStack34 = (uStack58 + 0x2e);
  if ((uStack34 + 0x4) != param_3) {
    ctx.PTR_LOOP_1050_50ca = 0x6b7;
    return;
  }
  uStack28 = struct_op_1030_73a8(CONCAT22(puStack56,uStack58));
 // puStack56 = (uStack28 >> 0x10);
  u_var1 = (uStack28 + 0xc);
  uStack58 = u_var1;
  if (u_var1 != 0x41) {
    if (0x41 < u_var1) {
      if (u_var1 == 0x6b) {
        ctx.PTR_LOOP_1050_50ca = 0x6b1;
        return;
      }
      if (u_var1 < 0x6c) {
        if (u_var1 == 0x42) {
          ctx.PTR_LOOP_1050_50ca = 0x6b1;
          return;
        }
        uStack58 = u_var1 - 0x4b;
        if (uStack58 == 0x0) {
          ctx.PTR_LOOP_1050_50ca = 0x6b1;
          return;
        }
      }
      else {
        if (u_var1 == 0x6e) {
          return;
        }
        uStack58 = u_var1 - 0x73;
        if ((0x4 < (u_var1 - 0x6e)) &&
           (uStack58 = u_var1 - 0x79, uStack58 == 0x0 || (u_var1 - 0x73) < 0x6)) {
          ctx.PTR_LOOP_1050_50ca = 0x6b0;
          return;
        }
      }
//       TODO: goto LAB_1020_eb4e;
    }
    if (u_var1 != 0x3e) {
      if (u_var1 < 0x3f) {
        cVar4 = u_var1;
        if (cVar4 != '\xb') {
          if (cVar4 == '\x10') {
            return;
          }
          uStack58 = u_var1 & 0xff00 | (cVar4 - 0x37);
          if ((cVar4 - 0x37) != 0x0) {
              // goto
              // LAB_1020_eb4e;
          }
        }
        ctx.PTR_LOOP_1050_50ca = 0x6b4;
        return;
      }
//       TODO: goto LAB_1020_eb4e;
    }
  }
  if ((uStack28 + 0x12) == 0x4) {
    ctx.PTR_LOOP_1050_50ca = 0x6b1;
    return;
  }
//LAB_1020_eb4e:
  uVar8 = 0x1000;
  mem_op_1000_179c(0xb4,puStack56,0x1000);
  puVar7 = (puStack56 | uStack58);
  if (puVar7 == 0x0) {
    iStack14 = 0x0;
    puVar7 = 0x0;
  }
  else {
    uVar8 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
    iStack14 = string_1040_8520(
                                CONCAT13((puStack56 >> 0x8),
                                         CONCAT12(puStack56,uStack58)),
                                ctx.PTR_LOOP_1050_0396,0x24,0x2,0x57b,0x5e8,puVar7,
                                param_6);
  }
  puStack12 = CONCAT22(puVar7,iStack14);
  ppcVar2 = (*puStack12 + 0x74);
  (**ppcVar2)(uVar8,iStack14,puVar7);
  if (iStack14 != 0x7) {
    puStack46 = uStack8;
    uStack34 = uStack8;
    uStack34._3_1_ = (uStack8 >> 0x18);
    u_var6 = uStack34._3_1_;
    if (uStack34._3_1_ != 0x0) {
      pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack8);
      uStack50 = CONCAT22(uStack8._2_2_,u_var6);
      fn_ptr_1030_7296(CONCAT22(uStack8._2_2_,u_var6));
      pass1_1030_730a(uStack50,u_var6,0x1030,param_6);
      puStack46 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2f,param_6,uStack8._2_2_,unaff_DI)
      ;
      pass1_1010_ec68(puStack46,uStack50,param_6);
      puStack42 = struct_op_1030_73a8(uStack50);
      pu_var5 = puStack42;
      ppcVar2 = (*puStack42 + 0x24);
      (**ppcVar2)(0x1030,puStack42,(puStack42 >> 0x10));
      u_var6 = pass1_1028_bc4a(puStack42,pu_var5,extraout_dx,param_6);
      (uVar9 + 0x24) = u_var6;
      struct_1030_e4fa(CONCAT22(param_6,local_146),
                       (uStack50 + 0x16),param_6,param_7);
      fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_6,local_146));
    }
    return;
  }
  ctx.PTR_LOOP_1050_50ca = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_ecb0(param_1: u32,param_2: i16,param_3: u16)
{
  let u_var1: u32;
  let i_var2: i16;
  let u_var3: u16;
  let unaff_SS: u16;
  let uStack8: u16;
  
 // u_var3 = (param_1 >> 0x10);
  i_var2 = param_1;
  u_var1 = (i_var2 + 0x8);
  pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
  if ((i_var2 + 0x12) != 0x1) {
    pass1_1028_bf22(param_1 & 0xffff | u_var3 << 0x10,param_3,unaff_SS);
    return;
  }
  if (false) {
switchD_1020_ed0e_caseD_4:
    uStack8 = (param_2 + 0x14);
  }
  else {
    switch((param_2 + 0x14)) {
    default:
      uStack8 = 0x2;
      break;
    0x3 =>
    0x8 =>
      uStack8 = 0x3;
      break;
    default:
//       TODO: goto switchD_1020_ed0e_caseD_4;
    0x5 =>
    0x6 =>
      uStack8 = 0x1;
    }
  }
  (i_var2 + 0x14) = uStack8;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_ed3c(param_1: u32,param_2: i16,param_3: u16,param_4: u8)
{
  let pi_var1: U32Ptr;
  let u_var2: u16;
  let extraout_dx: u16;
  let extraout_DX_00: u16;
  let iVar3: i16;
  let u_var4: u16;
  let local_138: [u8;112];
  let uStack38: u32;
  let puStack30: u32;
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u16;
  let local_12: i16;
  let local_10: [u8;2];
  let local_e: [u8;2];
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
 // u_var4 = (param_1 >> 0x10);
  iVar3 = param_1;
  pi_var1 = (iVar3 + 0x14);
  *pi_var1 = *pi_var1 + -0x1;
  if (*pi_var1 == 0x0) {
    (iVar3 + 0x12) = 0x0;
    pass1_1028_b58e(param_1);
    local_c = (param_2 + 0xc);
    uStack8 = (param_2 + 0x10);
    puStack30 = &local_c;
    iStack6 = param_2;
    uStack4 = extraout_dx;
    pass1_1008_3eb4(CONCAT22(param_3,&local_c),
                    CONCAT22(param_3,&local_12),
                    CONCAT22(param_3,local_10),
                    CONCAT22(param_3,local_e));
    if (local_12 < 0x1) {
      puStack30 = 0x5;
    }
    else {
      puStack30 = 0x6;
    }
    (iStack6 + 0x14) = puStack30;
    if (local_12 < 0x1) {
      u_var2 = 0x5;
    }
    else {
      u_var2 = 0x9;
    }
    uStack20 = u_var2;
    pass1_1020_ee3a(param_1,u_var2,u_var2,param_3,param_4);
    pass1_1028_b58e(param_1);
    uStack24 = CONCAT22(extraout_DX_00,u_var2);
    uStack28 = (u_var2 + 0x2e);
    pass1_1038_5804(uStack28,0x1,0x1);
    if (0x0 < (iVar3 + 0x24)) {
      uStack38 = (uStack28 + 0x4);
      pass1_1028_68de(CONCAT22(param_3,local_138),(iVar3 + 0x24)
                      ,uStack38,param_4,param_3);
      fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_3,local_138));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_ee3a(param_1: u32,param_2: u16,param_3: i16,param_4: u16,param_5: u8)
{
  let extraout_dx: u16;
  let local_13c: [u8;124];
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_b58e(param_1);
  local_c = (param_3 + 0xc);
  uStack8 = (param_3 + 0x10);
  iStack6 = param_3;
  uStack4 = extraout_dx;
  uStack16 = pass1_1028_bb24(param_1);
  uStack20 = (iStack6 + 0x2e);
  uStack24 = (uStack20 + 0x4);
  struct_op_1028_87f0(param_4,param_5,CONCAT22(param_4,local_13c),0x0,0x1,
                      param_2,&local_c,param_4,uStack24,uStack16);
  fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_4,local_13c));
  return;
}



astruct_18 *  pass1_1020_eed0(param_1: &mut Struct18,param_2: u8,param_3: u16)

{
  pass1_1030_dcf4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1020_ef5e(param_1: U32Ptr)
{
  *param_1 = 0x0;
  (param_1 + 0x2) = &USHORT_1050_1028;
  pass1_1028_b418(param_1);
  return;
}



astruct_18 *  pass1_1020_ef94(param_1: &mut Struct18,param_2: u8)

{
  pass1_1020_ef5e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}
