

u16 pass1_1010_b028(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15)

{
  return (param_3 + 0xc);
}


/*
Unable to decompile 'pass1_1010_b038'
Cause:
Low-level Error: Overlapping input varnodes
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn bad_1010_bf08(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  bad_1028_e1bc();
  return;
}
pub fn pass1_1010_bf1e(mut param_1: i16,param_2: *mut u8,mut param_3: u32,param_4: *mut i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  astruct_92 *paVar4;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uStack40: u32;
  let mut iStack36: i16;
  let mut puStack26: *mut u16;
  astruct_92 local_16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  bad_1010_bf08(param_3,(param_3 >> 0x10),0x1000000);
  iVar2 = param_1 + -0x1;
  *param_4 = iVar2;
  uVar3 = iVar2 * 0x18;
  mem_op_1000_179c(uVar3,paVar6);
  uVar5 = paVar6;
  uStack40 = CONCAT22(uVar5,uVar3);
  uVar7 = (uVar5 | uVar3);
  iVar8 = param_4;
  uVar9 = (param_4 >> 0x10);
  if ((uVar5 | uVar3) == 0) {
    (iVar8 + 0x2) = 0;
  }
  else {
    pass1_1000_5586(0x4092,0x1020,iVar2,0x18,uVar3,uVar5);
    (iVar8 + 0x2) = uStack40;
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x100);
  puStack26 = (u16*)(iVar8 + 2);
  iStack36 = 0;
  while( true ) {
    uVar3 = uVar7;
    paVar4 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
    uVar7 = (uVar3 | paVar4);
    if ((uVar3 | paVar4) == 0) break;
    uVar1 = &paVar4->field6_0x10;
    if (uVar1 != 0) {
      uVar7 = uVar1 >> 0x10;
      pass1_1008_3f62(puStack26,(uVar1 & 0xffff0000 | (uVar1 + 0x4)));
      (puStack26 + 0xc) = iStack36;
      iStack36 += 0x1;
      puStack26 = (puStack26 & 0xffff0000 | (puStack26 + 0x18));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_bffa(mut param_1: i16,param_2: *mut u8,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_257 *puVar5;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut paVar9: *mut Struct57;
  astruct_254 *iVar6;
  astruct_255 *iVar7;
  astruct_256 *iVar8;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut iStack34: i16;
  let mut iStack30: i16;
  astruct_92 local_18;
  let mut puStack6: *mut u32;

  paVar9 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0xa,paVar9);
  uVar7 = SUB42(paVar9,0x0);
  puStack6 = CONCAT22(uVar7,param_1);
  iVar4 = param_1;
  bad_1010_bf08(param_3,(param_3 >> 0x10),0x2000000);
  (param_1 + 0x8) = iVar4;
  if (iVar4 == 0) {
    (param_1 + 0x8) = 0x1;
  }
  uVar5 = (param_1 + 0x8) << 0x2;
  mem_op_1000_179c(uVar5,paVar9);
  puStack6 = uVar5;
  (param_1 + 0x2) = paVar9;
  if ((paVar9 | puStack6) == 0) {
    (param_1 + 0x8) = 0;
  }
  else {
    iVar4 = (param_1 + 0x8) * 0x2;
    mem_op_1000_179c(iVar4,paVar9);
    (param_1 + 0x4) = iVar4;
    (param_1 + 0x6) = paVar9;
    uVar5 = paVar9 | (param_1 + 0x4);
    if (uVar5 != 0) {
      uVar6 = FUN_1010_830a(uVar5,paVar9,0x1000,_u16_1050_14cc,0x1b4);
      puVar1 = *puStack6;
      *puVar1 = uVar6;
      (puVar1 + 0x2) = paVar9;
      (param_1 + 0x4) = 0;
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x200);
      iStack30 = 0x1;
      while( true ) {
        puVar5 = (astruct_257 *)&local_18;
        uVar6 = 0x1028;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,puVar5));
        uVar5 = paVar9;
        uVar8 = uVar5 | puVar5;
        paVar9 = (astruct_57 *)(paVar9 & 0xffff0000 | uVar8);
        if (uVar8 == 0) break;
        piVar2 = puVar5->field16_0x10;
        uVar8 = piVar2;
        iVar4 = *piVar2;
        iStack34 = 0;
        uVar5 = iVar4 - 0x1;
        if (uVar5 < 0xa) {
          uVar6 = 0x1010;
          switch(uVar5) {
          case 0x0:
            iStack34 = 0x1b5;
            uVar5 = uVar8;
            break;
          case 0x1:
            iStack34 = 0x1b6;
            uVar5 = uVar8;
            break;
          case 0x2:
            iStack34 = 0x1b7;
            uVar5 = uVar8;
            break;
          case 0x3:
            iStack34 = 0x1b8;
            uVar5 = uVar8;
            break;
          case 0x4:
            iStack34 = 0x1b9;
            uVar5 = uVar8;
            break;
          case 0x5:
            iStack34 = 0x1ba;
            uVar5 = uVar8;
            break;
          case 0x6:
            iStack34 = 0x1bb;
            uVar5 = uVar8;
            break;
          case 0x7:
            iStack34 = 0x1bc;
            uVar5 = uVar8;
            break;
          case 0x8:
            iStack34 = 0x1bd;
            uVar5 = uVar8;
            break;
          case 0x9:
            iStack34 = 0x1be;
            uVar5 = uVar8;
          }
        }
        uVar6 = FUN_1010_830a(uVar5,paVar9,uVar6,_u16_1050_14cc,iStack34);
        uVar11 = (*puStack6 >> 0x10);
        iVar10 = *puStack6;
        (iVar10 + iStack30 * 0x4) = uVar6;
        (iVar10 + iStack30 * 0x4 + 0x2) = paVar9;
        uVar3 = (param_1 + 0x4);
        (uVar3 + iStack30 * 0x2) = iVar4;
        iStack30 += 0x1;
      }
      return;
    }
  }
  return;
}
