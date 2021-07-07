
pub fn fn_ptr_1010_905e(param_1: u32,param_2: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  let ppcVar3: u32;
  let iVar4: &mut Struct169;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = *(u32 **)&iVar4.field_0x4;
  uVar2 = (&iVar4.field_0x4 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  iVar4.field_0x4 = param_2;
  return;
}
