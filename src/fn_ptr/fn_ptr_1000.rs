use crate::defines::{Struct18, U32Ptr};
use crate::winapi::{DOS3Call, swi};
use crate::sys_api::dos3_op_1000_256b;
use crate::misc::ret_op_1000_55ac;
use crate::util::{CONCAT22, get_string_from_addr, get_struct_from_addr};
use crate::mem_1000::mem_op_1000_0a48;
use crate::pass::pass_1000::pass1_1000_1e61;
use crate::global::AppContext;
use crate::win_struct::CONTEXT;


pub fn  call_fn_ptr_1000_0dc6(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_3: u16) -> bool

{
  if (ctx.PTR_LOOP_1050_000c & 0xfff8) != 0xcad0 {
    pass1_1000_1e61(ctx, param_3,0xe,0x0,0x0);
    return false;
  }
  return true;
}


pub unsafe fn fn_ptr_op_1000_1708(
    ctx: &mut AppContext,
    param_1: &mut i16,
    param_2: &mut u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16) -> bool

{
  let i_var1: i16;
  let b_var2: bool;
  let l_var3: i32;
  
  if (param_2 | param_1) == 0x0 {
    b_var2 = 0xfffe < param_1;
    *param_1 += 0x1;
    *param_2 += b_var2;
  }
//LAB_1000_1724:
  loop {
    if (param_5 | param_4) != 0x0 {
      l_var3 = mem_op_1000_0a48(ctx, param_3 as u8, *param_1 as u16, CONCAT22(param_5 as u16, param_4), param_6, 0);
      if l_var3 != 0x0 {
        return true;
      }
    }
    if ((param_3 & 0x8000) == 0x0) ||
       ((ctx.PTR_LOOP_1050_5f3a | ctx.PTR_LOOP_1050_5f38) == 0x0) {
      if (ctx.PTR_LOOP_1050_5f36 | ctx.PTR_LOOP_1050_5f34) == 0x0 {
        if (ctx.PTR_LOOP_1050_5f3e | ctx.PTR_LOOP_1050_5f3c) == 0x0 {
          return false;
        }
        (*ctx.PTR_LOOP_1050_5f3c)();
//         TODO: goto LAB_1000_1724;
      }
      i_var1 = (*ctx.PTR_LOOP_1050_5f34)();
    }
    else {
      i_var1 = (*ctx.PTR_LOOP_1050_5f38)(param_6, param_1);
    }
    if i_var1 == 0x0 {
      return false;
    }
  }
}



pub fn fn_ptr_1000_17ce(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u16)

{
  if param_1 != 0x0 {
    call_fn_ptr_1000_0dc6(ctx, param_1,param_2);
  }
}


pub fn fn_ptr_op_1000_24cd(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: i16)
{
  let pc_var1: u32;
  let i_var2: i16;
  let c_var3: u8;
  let u_var5: u16;
  let u_var3: u16;
  let u_var4: u16;

  i_var2 = param_2 + 0x1;
  u_var5 = ctx.data_seg;
  ctx.PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  u_var3 = 0x0;
  if true {
    fn_ptr_op_1000_2594(0x68b6,0x68b6);
    fn_ptr_op_1000_2594(ctx.PTR_LOOP_1050_6210,0x620c);
    ret_op_1000_55ac(param_1, u_var3, u_var5, i_var2);
  }
  c_var3 = (u_var3 >> 0x8) as u8;
  fn_ptr_op_1000_2594(ctx.PTR_LOOP_1050_6210,ctx.PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594(ctx.PTR_LOOP_1050_6210,ctx.PTR_LOOP_1050_6210);
  dos3_op_1000_256b();
  if c_var3 == '\0' as u8 {
    if true {
      pc_var1 = swi(0x21);
      (*pc_var1)();
    }
    else {
        let dos3_ctx = get_struct_from_addr::<CONTEXT>(ctx.PTR_LOOP_1050_1000);
      DOS3Call(dos3_ctx);
    }
  }
  return;
}



pub fn fn_ptr_op_1000_2594(mut param_1: U32Ptr, param_2: U32Ptr)
{
  let ppc_var1: u32;
  let ppc_var2: u32;
  let fn_ptr_1: u32;
  
  while param_2 < param_1 {
    ppc_var2 = param_1 + -0x2;
    ppc_var1 = param_1 + -0x1;
    param_1 = ppc_var2;
    if (*ppc_var2 | *ppc_var1) != 0x0 {
      fn_ptr_1 = ppc_var2;
      (**fn_ptr_1)();
    }
  }
  return;
}


