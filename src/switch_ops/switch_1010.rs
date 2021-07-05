
void  switchD_1010:2ab5::caseD_13(param_1: u32,param_2: i16)

{
  let uVar1: u32;
  let iVar2: i16;
  let unaff_SS: u16;
  undefined1 in_AF;
  
  iVar2 = param_2 * 0x8 + param_1;
  if (((((iVar2 + 0x22) != 0x0) || ((iVar2 + 0x24) != 0x0)) ||
      ((iVar2 + 0x26) != 0x0)) || ((iVar2 + 0x28) != 0x0)) {
    uVar1 = (param_1 + 0xe);
    sys_1000_3f9c((uchar *)uVar1,(uchar *)(uVar1 >> 0x10),
                  s__d__d__d__d_1050_14ae,&USHORT_1050_1050,
                  (param_2 * 0x8 + param_1 + 0x22),
                  &stack0xfffe,param_1._2_2_,0x1000,unaff_SS,in_AF);
    uVar1 = (param_1 + 0xa);
    WritePrivateProfileString16
              ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar1,(LPCSTR)(uVar1 >> 0x10),
               (LPCSTR)(param_1 + 0xe));
  }
  return;
}


fn switch_1010_6646(param_1: u16,param_2: u16,param_3: *mut u16,param_4: u16)
{
  switch(param_4) {
  case 0x83:
    *param_3 = 0xa;
    break;
  case 0x84:
    *param_3 = 0xe;
    break;
  case 0x85:
    *param_3 = 0x12;
    break;
  case 0x86:
    *param_3 = 0x16;
    return;
  case 0x87:
    *param_3 = 0x1a;
    return;
  case 0x88:
    *param_3 = 0x1e;
    return;
  case 0x89:
    *param_3 = 0x22;
    return;
  default:
    *param_3 = 0x0;
    return;
  }
  return;
}
