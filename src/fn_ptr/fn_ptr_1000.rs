
fn  call_fn_ptr_1000_0dc6(param_1: u16,param_2: u16,param_3: u16) -> bool

{
  if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(param_3,0xe,0x0,0x0);
    return 0x0;
  }
  (**(code **)0x8)(&USHORT_1050_1050);
  return 0x1;
}



fn fn_ptr_op_1000_1708(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
                   param_6: u16) -> u16

{
  let iVar1: i16;
  let bVar2: bool;
  let lVar3: i32;
  
  if ((param_2 | param_1) == 0x0) {
    bVar2 = 0xfffe < param_1;
    param_1 += 0x1;
    param_2 += bVar2;
  }
LAB_1000_1724:
  do {
    if ((param_5 | param_4) != 0x0) {
      lVar3 = mem_op_1000_0a48((byte)param_3,param_1,param_2,CONCAT22(param_5,param_4),
                               param_6);
      if (lVar3 != 0x0) {
        return lVar3;
      }
    }
    if (((param_3 & 0x8000) == 0x0) ||
       ((PTR_LOOP_1050_5f3a | PTR_LOOP_1050_5f38) == 0x0)) {
      if ((PTR_LOOP_1050_5f36 | PTR_LOOP_1050_5f34) == 0x0) {
        if ((PTR_LOOP_1050_5f3e | PTR_LOOP_1050_5f3c) == 0x0) {
          return 0x0;
        }
        (*(code *)PTR_LOOP_1050_5f3c)();
        goto LAB_1000_1724;
      }
      iVar1 = (*(code *)PTR_LOOP_1050_5f34)();
    }
    else {
      iVar1 = (*(code *)PTR_LOOP_1050_5f38)(param_6,param_1);
    }
    if (iVar1 == 0x0) {
      return 0x0;
    }
  } while( true );
}



fn fn_ptr_1000_17ce(astruct_18 *param_1,param_2: u16)

{
  if (param_1 != (astruct_18 *)0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_1._2_2_,param_2);
  }
  return;
}


pub fn fn_ptr_op_1000_24cd(param_1: i16,param_2: i16)
{
  code *pcVar1;
  let iVar2: i16;
  let cVar3: u8;
  let uVar5: u16;
  let uVar3: u16;
  u16_t uVar4;
  
  iVar2 = param_2 + 0x1;
  uVar5 = &USHORT_1050_1050;
  PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar3 = 0x0;
  if (true) {
    fn_ptr_op_1000_2594(0x68b6,0x68b6);
    fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210,0x620c);
    ret_op_1000_55ac(param_1,uVar3,uVar5,iVar2);
  }
  cVar3 = (uVar3 >> 0x8);
  fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210,&PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210,&PTR_LOOP_1050_6210);
  dos3_op_1000_256b();
  if (cVar3 == '\0') {
    if (true) {
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
    }
    else {
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    }
  }
  return;
}



fn fn_ptr_op_1000_2594(code **param_1,code **param_2)
{
  code **ppcVar1;
  code **ppcVar2;
  code **fn_ptr_1;
  
  while (param_2 < param_1) {
    ppcVar2 = param_1 + -0x2;
    ppcVar1 = param_1 + -0x1;
    param_1 = ppcVar2;
    if ((*ppcVar2 | *ppcVar1) != 0x0) {
      fn_ptr_1 = ppcVar2;
      (**fn_ptr_1)();
    }
  }
  return;
}

