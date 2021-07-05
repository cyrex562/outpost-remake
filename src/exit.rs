


fn exit_1000_25cc(param_1: i16,param_2: u16,param_3: u16) -> *mut i16

{
  let piVar1: *mut i16;
  char *pcVar2;
  LPCSTR str;
  let piVar3: *mut i16;
  let piVar4: *mut i16;
  char *pcVar5;
  let iVar6: i16;
  let iVar7: i16;
  
  iVar7 = 0x2;
  iVar6 = 0x2;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar6,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar7);
  if (str != 0x0) {
    iVar6 = 0x9;
    if (*str == 'M') {
      iVar6 = 0xf;
    }
    str = str + iVar6;
    iVar6 = 0x22;
    pcVar5 = str;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      pcVar2 = pcVar5;
      pcVar5 = pcVar5 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar5[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar4 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar4;
    piVar4 = piVar4 + 0x1;
    iVar6 = *piVar1;
    piVar3 = piVar4;
    if ((iVar6 == param_1) || (piVar3 = (i16 *)(iVar6 + 0x1), piVar3 == (i16 *)0x0)) {
      return piVar3;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      piVar1 = piVar4;
      piVar4 = (i16 *)(piVar4 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack
// WARNING: Variable defined which should be unmapped: param_2
// WARNING: Variable defined which should be unmapped: param_1

i16 * 
exit_1000_25f2(param_1: u16,param_2: u16,param_3: i16,param_4: i16,param_5: u16,
              param_6: u16,param_7: u16)

{
  i16 **ppiVar1;
  let piVar2: *mut i16;
  char *pcVar3;
  let puVar4: *mut u8;
  let piVar5: *mut i16;
  u16_t uVar6;
  LPCSTR str;
  let iVar7: i16;
  let piVar8: *mut i16;
  char *pcVar9;
  
  puVar4 = (param_4 + 0x1U & 0xfffe);
  if ((puVar4 < &param_3) &&
     (piVar5 = (i16 *)-(puVar4 + -&param_3),
     ppiVar1 = (i16 **)&PTR_LOOP_1050_000a, *ppiVar1 < piVar5 || *ppiVar1 == piVar5)) {
    ppiVar1 = (i16 **)&PTR_LOOP_1050_000c;
    if (piVar5 <= *ppiVar1 && *ppiVar1 != piVar5) {
      *(i16 **)&PTR_LOOP_1050_000c = piVar5;
    }
    piVar5[-0x1] = param_2;
    piVar5[-0x2] = param_1;
    return piVar5;
  }
  uVar6 = pass1_1000_29dc(param_7);
  if (0x5fce != -0x1) {
                    // WARNING: Could not recover jumptable at 0x10002622. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    piVar5 = (i16 *)(*(code *)0x5fce)();
    return piVar5;
  }
  pass1_1000_25a8(param_5,param_6);
  pass1_1000_2913(0x0,param_5,param_6);
  str = poss_str_op_1000_28dc(0x0);
  if (str != 0x0) {
    iVar7 = 0x9;
    if (*str == 'M') {
      iVar7 = 0xf;
    }
    str = str + iVar7;
    iVar7 = 0x22;
    pcVar9 = str;
    do {
      if (iVar7 == 0x0) break;
      iVar7 += -0x1;
      pcVar3 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar9[-0x1] = '\0';
  }
  FatalAppExit16(param_6,str);
  FatalExit();
  piVar5 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar2;
    piVar8 = piVar5;
    if ((iVar7 == param_3) || (piVar8 = (i16 *)(iVar7 + 0x1), piVar8 == (i16 *)0x0)) {
      return piVar8;
    }
    iVar7 = -0x1;
    do {
      if (iVar7 == 0x0) break;
      iVar7 += -0x1;
      piVar2 = piVar5;
      piVar5 = (i16 *)(piVar5 + 0x1);
    } while (*piVar2 != '\0');
  } while( true );
}


fn fatal_app_exit_1000_3e9e(u16 app_exit_action)
{
  FatalAppExit16(app_exit_action,(LPCSTR)s_ABNORMAL_PROGRAM_TERMINATION_1050_6544);
  return;
}
