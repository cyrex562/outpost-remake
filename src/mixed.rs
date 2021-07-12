use crate::pass::pass_1010::{pass1_1010_2816, pass1_1010_3d82, pass1_1010_503e, pass1_1010_3e3c, pass1_1010_3702, pass1_1010_6abc, pass1_1010_195e, pass1_1010_1146, pass1_1010_6700, pass1_1010_4a8a, pass1_1010_8c32, pass1_1010_2bfc, pass1_1010_0000};
use crate::struct_ops::struct_1010::{struct_1010_3b7a, struct_1010_a1d8, struct_1010_95aa, struct_1010_6326, struct_1010_02e0, struct_1010_e9e4, struct_1010_50b2};
use crate::sys_api::win_sys_op_1010_5404;
use crate::struct_ops::struct_1018::{struct_1018_2b10, struct_1018_0570};
use crate::pass::pass_1008::{pass1_1008_eabc, pass1_1008_d99e, pass1_1008_9d36, pass1_1008_eb2a, pass1_1008_d790, pass1_1008_d72e, pass1_1008_ec10};
use crate::pass::pass_1018::{pass1_1018_18b8, pass1_1018_4aaa, pass1_1018_1ff4, pass1_1018_5070, pass1_1018_56e6};
use crate::mem_1000::mem_op_1000_179c;
use crate::struct_ops::struct_1008::{struct_1008_9fd2, struct_1008_dd4e, pass1_1008_c72a, struct_1008_ecb2};
use crate::ui::ui_1008::pass1_1008_af94;
use crate::draw::draw_1008::unk_draw_op_1008_da12;
use crate::util::{CONCAT22, get_struct_at_addr};
use crate::defines::{Struct79, U32Ptr, Struct19};
use crate::global::AppContext;
use crate::win_struct::WNDCLASS16;

pub unsafe fn mixed_1010_20ba(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_3: &mut WNDCLASS16,
    param_4: &mut Struct19,
    param_5: i16,
    extraout_dx: u16) -> u16

{
  let ppc_var1: u32;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  // let extraout_dx: *mut u8;
  // let pa_var4: &mut Struct636;
  // let i_var5: i16;
  // let u_var6: u16;
  let u_var7: u16;
  let pu_var8: &mut Struct19;
  let pu_var9: &mut Struct19;
  let u_var10: u32;
  let u_var11: &mut Struct19;
  // let pu_stack6: *mut u16;
  
  pass1_1010_2816(param_1);
  let mut struct_ref_1 = get_struct_at_addr::<Struct19>((param_2 * 0x4) as u32);
  // let u_var6 = (param_1 >> 0x10);
  let i_var5 = param_1;
  let mut pu_stack6 = (struct_ref_1.field_0x0 + i_var5);
  if pu_stack6 != 0x0 {
    return pu_stack6;
  }
  if false {
      // // goto switchD_1010_2765_caseD_38;
  }
  match param_2 {
  0x1 => {
      mem_op_1000_179c(ctx, 0x18, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3.is_null() {
//LAB_1010_2126:
//           struct_ref_1 = 0x0;
          //TODO: pu_var3 = 0x0;
      } else {
          struct_1010_3b7a(struct_ref_1, param_2);
      }

  }
  0x2 => {
      mem_op_1000_179c(ctx, 0x84, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      win_sys_op_1010_5404(struct_ref_1, param_4, param_2, param_3);

  }
  0x3 => {
      mem_op_1000_179c(ctx, 0x53c, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      struct_1010_a1d8(struct_ref_1, param_4, param_2, param_3);

  }
  0x4 => {
      mem_op_1000_179c(ctx, 0x18a, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      struct_1018_2b10(CONCAT22(param_4, struct_ref_1), param_2, param_3);

  }
  0x5 => {
      mem_op_1000_179c(ctx, 0x14, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_2126
      pu_var9 = pass1_1008_eabc(struct_ref_1, param_4, param_2);
     // pu_var3 = (pu_var9 >> 0x10);
      struct_ref_1 = pu_var9;

  }
  0x6 => {
      mem_op_1000_179c(ctx, 0x58, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      pass1_1018_18b8(param_3, CONCAT22(param_4, struct_ref_1), param_2);

  }
  0x7 => {
      mem_op_1000_179c(ctx, 0xe, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1010_3d82(ctx, struct_ref_1, param_4, param_2, param_3);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;

  }
  0x8 => {
      mem_op_1000_179c(ctx, 0x20, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      struct_1010_95aa(struct_ref_1, param_4, param_2);

  }
  0x9 => {
      mem_op_1000_179c(ctx, 0x26, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      struct_1010_6326(struct_ref_1, param_4, param_2);

  }
  0xa => {
      mem_op_1000_179c(ctx, 0x1c, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1010_0eac(struct_ref_1, param_4, param_2,
                                (param_4 | struct_ref_1), param_3);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;

  }
  0xb => {
      mem_op_1000_179c(ctx, 0x1c, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1008_aefe(struct_ref_1, param_4, param_2,
                                (param_4 | struct_ref_1), param_3);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;

  }
  0xc | 0xd | 0xe | 0xf | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 => {
      mem_op_1000_179c(ctx, 0xaa, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      struct_1018_0570(CONCAT22(param_4, struct_ref_1), param_2, param_3);

  }
  0x25 => {
      mem_op_1000_179c(ctx, 0x1c, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1018_4aaa(struct_ref_1, param_4, param_2, pu_var3, param_3);

  }
  0x26 => {
      mem_op_1000_179c(ctx, 0x1c, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1008_d99e(struct_ref_1, param_4, param_2, pu_var3, param_3);

  }
  0x27 => {
      mem_op_1000_179c(ctx, 0x58, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1008_9d36(struct_ref_1, param_4, param_2, param_3);

  }
  0x28 => {
      mem_op_1000_179c(ctx, 0x2c, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1010_28e6(struct_ref_1, param_4, param_2, pu_var3, param_3);

  }
  0x29 => {
      mem_op_1000_179c(ctx, 0x72, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      struct_1018_229c(struct_ref_1, param_4, param_2, pu_var3, param_3);

  }
  0x2a => {
      mem_op_1000_179c(ctx, 0x1c, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1010_503e(struct_ref_1, param_4, param_2, pu_var3, param_3);

  }
  0x2b => {
      mem_op_1000_179c(ctx, 0x1a, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      struct_1010_02e0(ctx, struct_ref_1, param_2, 0);

  }
  0x2c => {
      mem_op_1000_179c(ctx, 0x10, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      pass1_1008_eb2a(struct_ref_1, param_4, param_2);

  }
  0x2d => {
      mem_op_1000_179c(ctx, 0x80, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1010_3e3c(CONCAT22(param_4, struct_ref_1), param_2, param_3);

  }
  0x2e => {
      mem_op_1000_179c(ctx, 0x806, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1018_1ff4(struct_ref_1, param_4, param_2);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;

  }
  0x2f => {
      mem_op_1000_179c(ctx, 0x58, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      struct_1010_e9e4(struct_ref_1, param_4, param_2);

  }
  0x30 => {
      mem_op_1000_179c(ctx, 0xe, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      pu_var8 = pass1_1010_3702(struct_ref_1, param_4, param_2);
     // pu_var3 = (pu_var8 >> 0x10);
      struct_ref_1 = pu_var8;

  }
  0x31 => {
      uVar2 = 0x60;
      u_var7 = 0x1000;
      mem_op_1000_179c(ctx, 0x60, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {
//LAB_1010_2680:
          u_var7 = 0x1000;
          struct_ref_1 = 0x0;
          pu_var3 = 0x0;
      } else {
          u_var11 = pass1_1010_9298(struct_ref_1, param_4, param_2,
                                    struct_ref_1, (param_4 | struct_ref_1),
                                    param_3);
         // pu_var3 = (u_var11 >> 0x10);
          struct_ref_1 = u_var11;
      }
//     TODO: // goto // LAB_1010_2683
}
  0x32 => {
      mem_op_1000_179c(ctx, 0x26, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1010_6abc(struct_ref_1, param_4, param_2);
  }
  0x33 => {
      mem_op_1000_179c(ctx, 0x72, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {
//LAB_1010_25b8:
          u_var2 = 0x0;
          u_var7 = 0x0;
      } else {
          u_var10 = pass1_1010_195e(struct_ref_1, param_4, param_2);
         // u_var7 = (u_var10 >> 0x10);
          u_var2 = u_var10;
      }
//     TODO: // goto // LAB_1010_25bb
}
  0x34 => {
      mem_op_1000_179c(ctx, 0x72, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_25b8
      u_var11 = pass1_1010_1b6e(struct_ref_1, param_4, param_2, param_3,
                                (param_4 | struct_ref_1));
     // u_var7 = (u_var11 >> 0x10);
      u_var2 = u_var11;
//LAB_1010_25bb:
      pu_stack6 = CONCAT22(u_var7, u_var2);
      pass1_1010_1146(CONCAT22(u_var7, u_var2), 0x0, param_5, param_3);
//     TODO: // goto switchD_1010_2765_caseD_38;
}
  0x35 => {
      mem_op_1000_179c(ctx, 0x14a, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1010_6700(struct_ref_1, param_4, param_2);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;
  }
  0x36 => {
      mem_op_1000_179c(ctx, 0x10, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // goto
      // LAB_1010_2126
      pass1_1008_d790(struct_ref_1, param_4, param_2, param_3);
  }
  0x37 => {
      mem_op_1000_179c(ctx, 0x420, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      struct_1008_9fd2(struct_ref_1, param_4, param_2);
      // break;
      // default:
//     TODO: // goto switchD_1010_2765_caseD_38;
}
  0x39 => {
      mem_op_1000_179c(ctx, 0x36, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1010_4a8a(struct_ref_1, param_4, param_2, param_3);
  }
  0x3a => {
      mem_op_1000_179c(ctx, 0xc, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      pu_var8 = pass1_1008_d72e(struct_ref_1, param_4, param_2);
     // pu_var3 = (pu_var8 >> 0x10);
      struct_ref_1 = pu_var8;
  }
  0x3b => {
      mem_op_1000_179c(ctx, 0x22, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      struct_1008_dd4e(struct_ref_1, param_4, param_2);
  }
  0x3c => {
      mem_op_1000_179c(ctx, 0x146, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1018_331c(struct_ref_1, param_4, param_2, param_3, pu_var3);
  }
  0x3d => {
      mem_op_1000_179c(ctx, 0xe, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1010_8c32(struct_ref_1, param_4, param_2, param_3);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;
  }
  0x3e => {
      mem_op_1000_179c(ctx, 0x18, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1018_5070(struct_ref_1, param_4, param_2);
  }
  0x3f => {
      mem_op_1000_179c(ctx, 0x12, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1008_c72a(struct_ref_1, param_4, param_2);
  }
  0x40 => {
      mem_op_1000_179c(ctx, 0x24, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      pass1_1008_af94(struct_ref_1, param_4, param_2);
  }
  0x41 => {
      uVar2 = 0x60;
      mem_op_1000_179c(ctx, 0x60, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2680
      u_var7 = 0x1008;
      struct_1008_ecb2(struct_ref_1, param_4, param_2);
      pu_var3 = extraout_dx;
//LAB_1010_2683:
      (param_2 * 0x4 + i_var5) = struct_ref_1;
      (param_2 * 0x4 + i_var5 + 0x2) = pu_var3;
      ppc_var1 = (struct_ref_1 + 0x10);
      (**ppc_var1)(u_var7, struct_ref_1, pu_var3, u_var2);
  }
  0x42 => {
      mem_op_1000_179c(ctx, 0xc, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_2126
      pu_var8 = pass1_1008_ec10(struct_ref_1, param_4, param_2);
     // pu_var3 = (pu_var8 >> 0x10);
      struct_ref_1 = pu_var8;

  }
  0x43 => {
      mem_op_1000_179c(ctx, 0x12, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      pu_var8 = pass1_1010_2bfc(struct_ref_1, param_4, param_2);
     // pu_var3 = (pu_var8 >> 0x10);
      struct_ref_1 = pu_var8;

  }
  0x45 => {
      mem_op_1000_179c(ctx, 0xe, param_4, 0x1000);
      if ((param_4 | struct_ref_1) == 0x0) {}
      // goto
      // LAB_1010_2126
      u_var11 = pass1_1010_0000(struct_ref_1, param_2, param_3);
     // pu_var3 = (u_var11 >> 0x10);
      struct_ref_1 = u_var11;

  }
  0x46 => {
      mem_op_1000_179c(ctx, 0x1a, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if (pu_var3 == 0x0) {}
      // goto
      // LAB_1010_2126
      struct_1010_50b2(struct_ref_1, param_4, param_2);

  }
  0x47 => {
      mem_op_1000_179c(ctx, 0xe, param_4, 0x1000);
      if (param_4 | struct_ref_1) == 0x0 {}
      // goto
      // LAB_1010_2126
      pu_var8 = pass1_1018_56e6(struct_ref_1, param_4, param_2);
     // pu_var3 = (pu_var8 >> 0x10);
      struct_ref_1 = pu_var8;

  }
  0x48 => {
      mem_op_1000_179c(ctx, 0x1c, param_4, 0x1000);
      pu_var3 = (param_4 | struct_ref_1);
      if pu_var3 == 0x0 {}
      // // goto
      // // LAB_1010_2126
      unk_draw_op_1008_da12(ctx, struct_ref_1, param_4, param_2);
  }
      _ => {}
  }
  pu_stack6 = CONCAT22(pu_var3, struct_ref_1);
// switchD_1010_2765_caseD_38:
  (param_2 * 0x4 + i_var5) = pu_stack6;
  return pu_stack6;
}
