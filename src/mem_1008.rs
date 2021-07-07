use crate::util::CONCAT22;


pub fn memcpy_op_1008_4274(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  let puVar2: *mut u8
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let lVar8: i32;
  
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x6) != 0x0) {
    uVar7 = pass1_1000_1284((iVar4 + 0x6),0x1000);
    iVar1 = (uVar7 >> 0x10);
    lVar8 = mem_op_1000_0a48(0x1,uVar7,iVar1,ctx._PTR_LOOP_1050_5f2c,0x1000);
    uVar5 = lVar8;
    puVar2 = ((lVar8 >> 0x10) | uVar5);
    if (puVar2 != 0x0) {
      hmemcpy16((LPVOID)&ctx.PTR_LOOP_1050_1000,uVar7,
                CONCAT22((iVar4 + 0x6),iVar1));
      mem_op_1000_179c(0x1e,puVar2,0x1000);
      uVar3 = puVar2 | uVar5;
      if (uVar3 == 0x0) {
        uVar5 = 0x0;
        uVar3 = 0x0;
      }
      else {
        pass1_1008_4016(CONCAT22(puVar2,uVar5));
      }
      (uVar5 + 0x6) = lVar8;
      pass1_1008_47cc(CONCAT22(uVar3,uVar5));
      pass1_1008_4834(CONCAT22(uVar3,uVar5));
      (uVar5 + 0x1c) = 0x1;
      return;
    }
  }
  return;
}




pub fn memcpy_op_1008_676e(param_1: u32,param_2: u16,uchar *param_3)
{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x6) == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_3,0x1000);
  uVar3 = param_3 | param_2;
  if (uVar3 == 0x0) {
    param_2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    uVar1 = (iVar4 + 0x10);
    uVar7 = (uVar1 >> 0x10);
    iVar5 = uVar1;
    struct_op_1008_6604(CONCAT22(param_3,param_2),
                        (iVar5 + 0x8),(iVar5 + 0x4));
  }
  pass1_1000_48a8((param_2 + 0x10),(iVar4 + 0x10),0x28);
  uVar1 = (param_2 + 0x10);
  lVar2 = (uVar1 + 0x8) * (iVar4 + 0x18);
  hmemcpy16((LPVOID)&ctx.PTR_LOOP_1050_1000,lVar2,
            CONCAT22((iVar4 + 0x6),(lVar2 >> 0x10)));
  (param_2 + 0x1c) = 0x1;
  return;
}



pub fn mem_1008_ed1e(param_1: u16,param_2: u16,param_3: i16,param_4: u16,uchar *param_5)
{
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,param_5,0x1000);
    return;
  }
  mem_op_1000_179c(0x1a,param_5,0x1000);
  if ((param_5 | param_4) != 0x0) {
    struct_1008_ec72(CONCAT22(param_5,param_4));
    return;
  }
  return;
}

