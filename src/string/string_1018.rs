
fn unk_str_op_1018_35b0(param_1: u32,param_2: u16,param_3: u16)
{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let uVar3: u16;
  let uVar4: u16;
  code **ppcVar5;
  let uVar6: u16;
  let puVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let extraout_DX: u16;
  let uVar10: u16;
  let valist: *mut u16;
  let bVar11: bool;
  let uVar12: u32;
  let uVar13: u32;
  let local_12: i16;
  let local_10: i16;
  let lStack14: i32;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let uStack4: u16;
  
  uVar12 = pass1_1030_8326();
  uStack4 = (uVar12 >> 0x10);
  uStack6 = uVar12;
  valist = (param_1 >> 0x10);
  uVar10 = param_1;
  puVar1 = (uVar10 + 0x140);
  bVar11 = *puVar1 < uStack4;
  if ((bVar11) ||
     ((bVar11 || *puVar1 == uStack4 && ((uVar10 + 0x13e) < uStack6)))) {
    uVar3 = (uVar10 + 0x13c);
    if ((uVar10 + 0x13a) < uVar3) {
      uVar13 = switch_1018_3b9e(param_1,(uVar10 + 0x12e),uVar3,uStack4,param_2)
      ;
      uVar8 = (uVar13 >> 0x10);
      uVar6 = uVar13;
      uStack10 = uVar6;
      uStack8 = uVar8;
      pass1_1018_427c(param_1);
      lStack14 = CONCAT22(uVar8,uVar6);
      pass1_1018_3e8c(uVar10,valist,CONCAT22(param_2,&local_12),
                      CONCAT22(param_2,&local_10));
      if (lStack14 < local_12) {
        local_12 = lStack14;
      }
      uVar4 = (uVar10 + 0x138);
      puVar7 = (uVar10 + 0x136);
      uVar9 = uVar4 | puVar7;
      if (uVar9 != 0x0) {
        ppcVar5 = (code **)*puVar7;
        (**ppcVar5)(0x30,puVar7,uVar4,0x1);
        uVar9 = extraout_DX;
      }
      pass1_1018_435e(param_1,lStack14,local_12,local_10,uVar9,param_2);
      (uVar10 + 0x136) = puVar7;
      (uVar10 + 0x138) = uVar9;
      piVar2 = (i16 *)(uVar10 + 0x13a);
      *piVar2 = *piVar2 + 0x1;
      wsprintf16((LPSTR)0x1030,(LPCSTR)(uVar10 + 0x22),valist);
      return;
    }
    (uVar10 + 0x13e) = uStack6;
    (uVar10 + 0x140) = uStack4;
    (uVar10 + 0x13a) = 0x0;
    pass1_1008_612e(0x8,0xc,uStack6);
    (uVar10 + 0x13c) = uStack6;
  }
  return;
}


fn string_1018_39d8(param_1: u16,param_2: u32,param_3: u32,param_4: u32) -> bool

{
  let iVar1: i16;
  char *pcVar2;
  let lVar3: i32;
  let uVar4: u32;
  
  uVar4 = param_3;
  pcVar2 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  iVar1 = pass1_1000_3d7a(pcVar2,uVar4);
  if (iVar1 != 0x0) {
    iVar1 = pass1_1000_3d7a(param_4,param_3);
    if (iVar1 != 0x0) {
      lVar3 = pass1_1018_4608(param_1,param_2,param_3,param_4);
      if ((lVar3 != 0x0) && ((lVar3 + 0xc) == 0x1)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}

