
pub fn unk_str_op_1018_35b0(param_1: u32,param_2: u16,param_3: u16)
{
  let pu_var1: U32Ptr;
  let piVar2: U32Ptr;
  let u_var3: u16;
  let u_var4: u16;
  let ppcVar5: u32;
  let u_var6: u16;
  let puVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let extraout_dx: u16;
  let u_var10: u16;
  let valist: U32Ptr;
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
 // uStack4 = (uVar12 >> 0x10);
  uStack6 = uVar12;
 // valist = (param_1 >> 0x10);
  u_var10 = param_1;
  pu_var1 = (u_var10 + 0x140);
  bVar11 = *pu_var1 < uStack4;
  if ((bVar11) ||
     ((bVar11 || *pu_var1 == uStack4 && ((u_var10 + 0x13e) < uStack6)))) {
    u_var3 = (u_var10 + 0x13c);
    if ((u_var10 + 0x13a) < u_var3) {
      uVar13 = switch_1018_3b9e(param_1,(u_var10 + 0x12e),u_var3,uStack4,param_2)
      ;
     // uVar8 = (uVar13 >> 0x10);
      u_var6 = uVar13;
      uStack10 = u_var6;
      uStack8 = uVar8;
      pass1_1018_427c(param_1);
      lStack14 = CONCAT22(uVar8,u_var6);
      pass1_1018_3e8c(u_var10,valist,CONCAT22(param_2,&local_12),
                      CONCAT22(param_2,&local_10));
      if (lStack14 < local_12) {
        local_12 = lStack14;
      }
      u_var4 = (u_var10 + 0x138);
      puVar7 = (u_var10 + 0x136);
      uVar9 = u_var4 | puVar7;
      if (uVar9 != 0x0) {
        ppcVar5 = *puVar7;
        (**ppcVar5)(0x30,puVar7,u_var4,0x1);
        uVar9 = extraout_dx;
      }
      pass1_1018_435e(param_1,lStack14,local_12,local_10,uVar9,param_2);
      (u_var10 + 0x136) = puVar7;
      (u_var10 + 0x138) = uVar9;
      piVar2 = (u_var10 + 0x13a);
      *piVar2 = *piVar2 + 0x1;
      wsprintf16(0x1030,(u_var10 + 0x22),valist);
      return;
    }
    (u_var10 + 0x13e) = uStack6;
    (u_var10 + 0x140) = uStack4;
    (u_var10 + 0x13a) = 0x0;
    pass1_1008_612e(0x8,0xc,uStack6);
    (u_var10 + 0x13c) = uStack6;
  }
  return;
}


pub fn string_1018_39d8(param_1: u16,param_2: u32,param_3: u32,param_4: u32) -> bool

{
  let i_var1: i16;
  let mut pcVar2: String; 
  let lVar3: i32;
  let u_var4: u32;
  
  u_var4 = param_3;
  pcVar2 = load_string_1010_847e
                     (ctx.PTR_LOOP_1050_14cc,(ctx.PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  i_var1 = pass1_1000_3d7a(pcVar2,u_var4);
  if (i_var1 != 0x0) {
    i_var1 = pass1_1000_3d7a(param_4,param_3);
    if (i_var1 != 0x0) {
      lVar3 = pass1_1018_4608(param_1,param_2,param_3,param_4);
      if ((lVar3 != 0x0) && ((lVar3 + 0xc) == 0x1)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}

