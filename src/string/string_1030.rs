
pub fn vsprintf_op_1030_840a(param_1: u32,param_2: &mut String,param_3: *mut u16,param_4: u16)
{
  let mut pCVar1: String;
  let unaff_ES: u16;
  ulet in_AF: u8;
  let mut pCStack6: String;
  let args: *mut u16;
  
  if (ctx.PTR_LOOP_1050_574c != 0x0) {
    pCVar1 = &stack0x0008;
    pCStack6 = pCVar1;
    args = param_3;
    if (ctx.PTR_LOOP_1050_5750 == 0x0) {
      param_2 = &ctx.PTR_LOOP_1050_1000;
      pass1_1000_2b3c(s_simres_out_1050_5758,ctx.data_seg,0x5756,
                      ctx.data_seg,param_4,&stack0xfffe);
      ctx._PTR_LOOP_1050_5752 = CONCAT22(param_4,pCVar1);
      ctx.PTR_LOOP_1050_5750 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    }
    wvsprintf16(param_2,pCStack6,args);
    pass1_1000_2b5c(ctx.PTR__LOOP_1050_5752,
                    (ctx.PTR__LOOP_1050_5752 >> 0x10),0x5763,
                    ctx.data_seg,unaff_ES,&stack0xfffe,0x1000,
                    param_3);
    pass1_1000_2f48(ctx.PTR__LOOP_1050_5752,&stack0xfffe,unaff_ES,0x1000,param_3,
                    in_AF);
  }
  return;
}

