
fn _SHI_INVOKEERRORHANDLER1() -> u16

{
  let iVar1: i16;
  let BVar2: bool;
  let uVar2: u16;
  let unaff_CS: u16;
  code *pcStack6;
  let puStack4: *mut u8;
  let uVar3: u16;
  
  uVar3 = &USHORT_1050_1050;
  puStack4 = &USHORT_1050_1050;
  if (true) {
    if ((PTR_LOOP_1050_5f1c | PTR_PTR_1050_5f1a) == 0x0) {
      pcStack6 = (code *)0x0;
      puStack4 = 0x0;
    }
    else {
      iVar1 = mem_op_1000_21b6(PTR_PTR_1050_5f1a,PTR_LOOP_1050_5f1c);
      pcStack6 = (code *)PTR_PTR_1050_5f1a;
      puStack4 = PTR_LOOP_1050_5f1c;
      if (iVar1 == 0x0) {
        PTR_PTR_1050_5f1a = &PTR_PTR_1050_1f7e;
        PTR_LOOP_1050_5f1c = &PTR_LOOP_1050_1000;
        pcStack6 = (code *)&PTR_PTR_1050_1f7e;
        puStack4 = &PTR_LOOP_1050_1000;
      }
    }
    if ((puStack4 | pcStack6) != 0x0) {
      BVar2 = msg_box_op_1000_1f24
                        (&PTR_PTR_1050_5f1a,&USHORT_1050_1050,0x0,unaff_CS);
      if (BVar2 == 0x0) {
        uVar2 = (*pcStack6)();
      }
      else {
        puStack4 = 0x0;
        pcStack6 = (code *)0x0;
        uVar2 = 0x0;
      }
      if ((puStack4 | pcStack6) != 0x0) {
        pass1_1000_1f68(uVar3);
      }
      return uVar2;
    }
  }
  return 0x0;
}




fn dos3_call_1000_23ea(param_1: u16,param_2: u16,param_3: i16,param_4: u16) -> *mut i16

{
  byte *pbVar1;
  byte *pbVar2;
  let bVar3: u8;
  let piVar4: *mut i16;
  byte *pbVar5;
  char *pcVar6;
  u16_t uVar7;
  code **ppcVar8;
  code *pcVar9;
  let uVar10: u16;
  let bVar11: u8;
  let bVar12: u8;
  u16_t uVar13;
  LPCSTR str;
  let piVar14: *mut i16;
  let uVar15: u16;
  let extraout_DX: u16;
  let uVar16: u16;
  byte *pbVar17;
  let piVar18: *mut i16;
  byte *pbVar19;
  char *pcVar20;
  u16_t action;
  let bVar21: bool;
  let uVar22: u32;
  let iVar23: i16;
  let iVar24: i16;
  let iVar25: i16;
  
  iVar25 = &USHORT_1050_1050;
  if (true) {
    pcVar9 = (code *)swi(0x21);
    (*pcVar9)(param_3 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar15 = 0x2890;
  action = 0x1000;
  PTR_LOOP_1050_5f6a = param_1;
  PTR_LOOP_1050_5f6c = param_2;
  if (true) {
    pcVar9 = (code *)swi(0x21);
    (*pcVar9)();
    uVar15 = extraout_DX;
  }
  else {
    action = (u16_t)s_tile2_bmp_1050_1538;
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar13 = pass1_1000_29dc(param_4);
  uVar22 = CONCAT22(uVar15,uVar13);
  if (&PTR_LOOP_1050_6202 != 0x0) {
    uVar7 = *(u16_t *)&PTR_LOOP_1050_5f7e;
    bVar21 = false;
    ppcVar8 = (code **)&PTR_LOOP_1050_6200;
    (**ppcVar8)(action);
    if (bVar21) {
      iVar24 = 0x2;
      iVar23 = 0x2;
      pass1_1000_25a8(uVar7,action);
      pass1_1000_2913(iVar23,uVar7,action);
      str = poss_str_op_1000_28dc(iVar24);
      if (str != (*mut u8)0x0) {
        iVar23 = 0x9;
        if (*str == 'M') {
          iVar23 = 0xf;
        }
        str = str + iVar23;
        iVar23 = 0x22;
        pcVar20 = str;
        do {
          if (iVar23 == 0x0) break;
          iVar23 += -0x1;
          pcVar6 = pcVar20;
          pcVar20 = pcVar20 + 0x1;
        } while (*pcVar6 != '\r');
        pcVar20[-0x1] = '\0';
      }
      FatalAppExit16(action,str);
      FatalExit();
      piVar18 = (i16 *)&PTR_LOOP_1050_63fe;
      do {
        piVar4 = piVar18;
        piVar18 = piVar18 + 0x1;
        iVar23 = *piVar4;
        piVar14 = piVar18;
        if ((iVar23 == iVar25) || (piVar14 = (i16 *)(iVar23 + 0x1), piVar14 == (i16 *)0x0)
           ) {
          return piVar14;
        }
        iVar23 = -0x1;
        do {
          if (iVar23 == 0x0) break;
          iVar23 += -0x1;
          piVar4 = piVar18;
          piVar18 = (i16 *)(piVar18 + 0x1);
        } while (*piVar4 != '\0');
      } while( true );
    }
    ppcVar8 = (code **)&PTR_LOOP_1050_6200;
    uVar22 = (**ppcVar8)(action);
  }
  iVar25 = (s_New_failed_in_Op__Op_1050_0020 + 0xc);
  piVar18 = (i16 *)uVar22;
  if (iVar25 != 0x0) {
    pbVar19 = (byte *)0x0;
    piVar14 = (i16 *)uVar22;
    do {
      bVar21 = *pbVar19 == 0x0;
      piVar18 = piVar14;
      if (bVar21) break;
      iVar23 = 0xd;
      pbVar17 = (byte *)s__C_FILE_INFO__1050_5f5c;
      do {
        if (iVar23 == 0x0) break;
        iVar23 += -0x1;
        pbVar5 = pbVar19;
        pbVar19 = pbVar19 + 0x1;
        pbVar1 = pbVar17;
        pbVar17 = pbVar17 + 0x1;
        bVar21 = *pbVar1 == *pbVar5;
      } while (bVar21);
      if (bVar21) {
        pbVar17 = (byte *)0x5f90;
        uVar16 = (uVar22 >> 0x10);
        goto LAB_1000_2495;
      }
      iVar23 = 0x7fff;
      piVar18 = (i16 *)0x0;
      bVar21 = true;
      do {
        if (iVar23 == 0x0) break;
        iVar23 += -0x1;
        pbVar1 = pbVar19;
        pbVar19 = pbVar19 + 0x1;
        bVar21 = *pbVar1 == 0x0;
      } while (!bVar21);
      piVar14 = piVar18;
    } while (bVar21);
  }
LAB_1000_24a9:
  fn_ptr_op_1000_2594(0x620c,0x620c);
  fn_ptr_op_1000_2594(0x620c,0x620c);
  fn_ptr_op_1000_2594(0x61fe,0x61ee);
  return piVar18;
LAB_1000_2495:
  pbVar2 = pbVar19 + 0x1;
  bVar3 = *pbVar19;
  uVar10 = piVar14 & 0xff00;
  bVar11 = bVar3 + 0xbf;
  piVar18 = (i16 *)(uVar10 | bVar11);
  if (bVar3 < 0x41) goto LAB_1000_24a9;
  pbVar19 = pbVar19 + 0x2;
  bVar3 = *pbVar2;
  piVar14 = (i16 *)(uVar16 & 0xff00);
  bVar12 = bVar3 + 0xbf;
  piVar18 = (i16 *)(piVar14 | bVar12);
  if (bVar3 < 0x41) goto LAB_1000_24a9;
  pbVar1 = pbVar17;
  pbVar17 = pbVar17 + 0x1;
  *pbVar1 = bVar12 | bVar11 * '\x10';
  uVar16 = uVar10;
  goto LAB_1000_2495;
}


fn dos3_op_1000_256b(void)
{
  code *pcVar1;
  
  if (PTR_LOOP_1050_6202 != 0x0) {
    (*(code *)PTR_LOOP_1050_6200)();
  }
  if (true) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  return;
}


u16 
sys_1000_30b4(param_1: u16,param_2: u16,param_3: *mut u8,param_4: i16,param_5: u16,
             param_6: u16,param_7: u16,param_8: u16)

{
  let bVar1: u8;
  let bVar2: u8;
  let uVar3: u16;
  let iVar3: i16;
  let uVar4: u16;
  
  iVar3 = param_4 + 0x1;
  uVar4 = SUB42(&USHORT_1050_1050,0x0);
  exit_1000_25f2(0x30c5,param_7,&USHORT_1050_1050,0x214,param_6,param_7,param_8);
  bVar1 = *param_3;
  if ((bVar1 != 0x0) && (true)) {
    if ((byte)(bVar1 - 0x20) < 0x59) {
      bVar2 = *(byte *)((byte)(bVar1 - 0x20) + 0x5ffe) & 0xf;
    }
    else {
      bVar2 = 0x0;
    }
                    // WARNING: Could not emulate address calculation at 0x10003101
                    // WARNING: Treating indirect jump as call
    uVar3 = (**(code **)((char)(*(byte *)((byte)(bVar2 * '\b') + 0x5ffe) >> 0x4) *
                         0x2 + 0x30a4))(param_5 & 0xff00 | bVar1,uVar4,iVar3);
    return uVar3;
  }
  return 0x0;
}



fn dos3_call_op_1000_35fe(param_1: u16,param_2: i16) -> u16

{
  code *pcVar1;
  let uVar2: u16;
  let uVar3: u8;
  
  if (param_1 < DAT_1050_5f8a) {
    uVar2 = 0x3e50;
    uVar3 = 0x0;
    if (true) {
      pcVar1 = (code *)swi(0x21);
      uVar2 = (*pcVar1)(param_2 + 0x1);
    }
    else {
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    }
    if (!(bool)uVar3) {
      *(param_1 + 0x5f90) = 0x0;
    }
  }
  else {
    uVar2 = 0x900;
    uVar3 = true;
  }
  if (!(bool)uVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return 0xffff;
}



void 
mixed_dos3_call_1000_3636
          (param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16)

{
  byte *pbVar1;
  code *pcVar2;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u8;
  let uVar9: u32;
  
  if (((param_1 < DAT_1050_5f8a) || (PTR_LOOP_1050_61ec == 0x0)) ||
     (0x2 < param_1)) {
    if ((PTR_LOOP_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0x0))
    goto LAB_1000_36e3;
    if (param_4 == 0x0) goto LAB_1000_369b;
    uVar5 = 0x0;
    uVar6 = 0x0;
    uVar4 = 0x4201;
    uVar8 = 0x0;
    if (true) {
      pcVar2 = (code *)swi(0x21);
      uVar9 = (*pcVar2)();
    }
    else {
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
      uVar9 = CONCAT22(uVar6,uVar4);
    }
    iVar7 = (uVar9 >> 0x10);
    uVar3 = uVar9;
    if ((bool)uVar8) goto LAB_1000_299d;
    if ((param_4 & 0x2) == 0x0) {
      if (-0x1 < (iVar7 + param_3 + CARRY2(uVar3,param_2))) {
LAB_1000_36e3:
        uVar3 = CONCAT11(0x42,(u8)param_4);
        uVar8 = 0x0;
        if (true) {
          pcVar2 = (code *)swi(0x21);
          uVar3 = (*pcVar2)();
        }
        else {
          DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
        }
        if (!(bool)uVar8) {
          pbVar1 = (byte *)(param_1 + 0x5f90);
          uVar8 = false;
          *pbVar1 = *pbVar1 & 0xfd;
        }
        goto LAB_1000_299d;
      }
    }
    else {
      uVar4 = SUB42(&PTR_DAT_1050_0041_1050_4202,0x0);
      if (true) {
        pcVar2 = (code *)swi(0x21);
        uVar9 = (*pcVar2)(iVar7);
      }
      else {
        DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
        uVar9 = CONCAT22(uVar5,uVar4);
      }
      if (-0x1 < ((uVar9 >> 0x10) + param_3 +
                      CARRY2(uVar9,param_2))) goto LAB_1000_36e3;
      if (true) {
        pcVar2 = (code *)swi(0x21);
        (*pcVar2)();
      }
      else {
        DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
      }
    }
LAB_1000_369b:
    uVar3 = s_471_bmp_1050_1600;
  }
  else {
    uVar3 = 0x900;
  }
  uVar8 = true;
LAB_1000_299d:
  if ((bool)uVar8) {
    pass1_1000_29b5(uVar3);
  }
  return;
}



u16 
mixed_dos3_call_1000_370a
          (param_1: u16,param_2: u16,param_3: u16,param_4: u8,param_5: u16,
          param_6: i16)

{
  code *pcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let extraout_DX: u16;
  let uVar7: u8;
  let uVar8: u32;
  let uVar9: u16;
  let bVar10: u8;
  let bVar11: u8;
  let in_stack_0000fffb: u8;
  
  _param_4 = param_5;
  bVar10 = 0x0;
  if (((param_3 & 0x8000) == 0x0) &&
     (((param_3 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0x0)))) {
    bVar10 = 0x80;
  }
  uVar9 = SUB42(&USHORT_1050_1050,0x0);
  uVar3 = CONCAT11(0x3d,(byte)param_3 & 0x3 | param_4);
  uVar7 = 0x0;
  uVar6 = param_3;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar8 = (*pcVar1)(bVar10,param_4,&USHORT_1050_1050,param_6 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    uVar8 = CONCAT22(param_1,uVar3);
  }
  uVar2 = uVar8;
  if ((bool)uVar7) {
    if ((uVar2 == 0x2) && ((uVar6 & 0x100) != 0x0)) {
      bVar11 = 0x0;
      uVar7 = pass1_1000_39e1();
      _param_4 = param_5;
      if ((param_4 != 0x0) || (uVar6 = param_5, (param_3 & 0x2) == 0x0)) {
        uVar6 = 0x0;
      }
LAB_1000_38e3:
      uVar5 = CONCAT11(0x3c,uVar7);
      uVar7 = 0x0;
      if (true) {
        pcVar1 = (code *)swi(0x21);
        uVar5 = (*pcVar1)();
      }
      else {
        DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
      }
      uVar2 = uVar5;
      if ((bool)uVar7) goto LAB_1000_299d;
      if ((param_4 != 0x0) || ((param_3 & 0x2) == 0x0)) {
        if (true) {
          pcVar1 = (code *)swi(0x21);
          (*pcVar1)();
        }
        else {
          DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
        }
        uVar5 = CONCAT11(0x3d,(byte)param_3 & 0x3 | param_4);
        uVar7 = 0x0;
        if (true) {
          pcVar1 = (code *)swi(0x21);
          uVar5 = (*pcVar1)();
        }
        else {
          DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
        }
        uVar2 = uVar5;
        if ((bool)uVar7) goto LAB_1000_299d;
        if (((bVar11 & 0x1) == 0x0) && ((_param_4 & 0x1) != 0x0)) {
          uVar6 = (byte)((byte)uVar6 | 0x1);
          uVar2 = 0x4301;
          uVar7 = 0x0;
          if (true) {
            pcVar1 = (code *)swi(0x21);
            uVar2 = (*pcVar1)();
          }
          else {
            DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
          }
          if ((bool)uVar7) goto LAB_1000_299d;
        }
      }
LAB_1000_3973:
      if ((bVar10 & 0x40) == 0x0) {
        if (true) {
          pcVar1 = (code *)swi(0x21);
          (*pcVar1)();
        }
        else {
          DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
        }
        bVar11 = 0x0;
        if ((uVar6 & 0x1) != 0x0) {
          bVar11 = 0x10;
        }
        if ((param_3 & 0x8) != 0x0) {
          bVar11 |= 0x20;
        }
      }
      else {
        bVar11 = 0x0;
      }
      if (uVar5 < &DAT_1050_5f8a) {
        *(byte *)(uVar5 + 0x5f90) = bVar11 | bVar10 | 0x1;
        return uVar5;
      }
      if (true) {
        pcVar1 = (code *)swi(0x21);
        (*pcVar1)();
      }
      else {
        DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
      }
      uVar2 = 0x1800;
    }
  }
  else {
    if ((uVar6 & 0x500) != 0x500) {
      bVar11 = 0x1;
      if (true) {
        pcVar1 = (code *)swi(0x21);
        (*pcVar1)();
        uVar8 = CONCAT22(extraout_DX,uVar2);
      }
      else {
        DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
      }
      uVar5 = uVar8;
      if ((uVar8 & 0x800000) != 0x0) {
        bVar10 |= 0x40;
      }
      if ((bVar10 & 0x40) == 0x0) {
        if ((param_3 & 0x200) == 0x0) {
          if (((bVar10 & 0x80) != 0x0) && ((param_3 & 0x2) != 0x0)) {
            uVar7 = 0x2;
            if (true) {
              pcVar1 = (code *)swi(0x21);
              uVar7 = (*pcVar1)();
            }
            else {
              DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
            }
            iVar4 = CONCAT11(0x3f,uVar7);
            if (true) {
              pcVar1 = (code *)swi(0x21);
              iVar4 = (*pcVar1)();
            }
            else {
              DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
            }
            if ((iVar4 != 0x0) && (in_stack_0000fffb == '\x1a')) {
              if (true) {
                pcVar1 = (code *)swi(0x21);
                (*pcVar1)();
              }
              else {
                DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
              }
              if (true) {
                pcVar1 = (code *)swi(0x21);
                (*pcVar1)();
              }
              else {
                DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
              }
            }
            uVar6 = 0x0;
            if (true) {
              pcVar1 = (code *)swi(0x21);
              (*pcVar1)();
            }
            else {
              DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
            }
          }
        }
        else {
          if ((param_3 & 0x3) == 0x0) {
            if (true) {
              pcVar1 = (code *)swi(0x21);
              (*pcVar1)();
            }
            else {
              DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
            }
            uVar7 = 0x0;
            if (true) {
              pcVar1 = (code *)swi(0x21);
              uVar7 = (*pcVar1)();
            }
            else {
              DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
            }
            goto LAB_1000_38e3;
          }
          uVar6 = 0x0;
          if (true) {
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
          }
          else {
            DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
          }
        }
      }
      goto LAB_1000_3973;
    }
    if (true) {
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
    }
    else {
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    }
    uVar2 = 0x1100;
  }
  uVar7 = true;
LAB_1000_299d:
  if ((bool)uVar7) {
    pass1_1000_29b5(uVar2);
    uVar2 = 0xffff;
  }
  return uVar2;
}





// WARNING: Unable to track spacebase fully for stack

uchar * 
mixed_dos3_call_1000_39f2
          (uchar *param_1,char *param_2,uchar *param_3,param_4: u16,param_5: u16,
          param_6: u16,char param_7)

{
  byte *pbVar1;
  let puVar2: *mut u8;
  let puVar3: *mut u8
  code *pcVar4;
  let uVar5: u16;
  let uVar6: u8;
  let piVar7: *mut i16;
  let uVar8: u16;
  let piVar9: *mut i16;
  let piVar10: *mut i16;
  let uVar11: u16;
  let puVar12: *mut u8
  let iVar13: i16;
  let puVar14: *mut u8;
  byte *pbVar15;
  let piVar16: *mut i16;
  let puVar17: *mut u8;
  let unaff_BP: i16;
  byte *pbVar18;
  let puVar19: *mut u8;
  let uVar20: u16;
  let uVar21: u8;
  let bVar22: u8;
  let cVar23: u8;
  let bVar24: bool;
  let cVar25: u8;
  let cVar26: u8;
  let uVar27: u32;
  char *pcVar28;
  let piStack14: *mut i16;
  let puStack12: *mut u8;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let uStack4: u16;
  let iStack2: i16;
  
  iStack2 = unaff_BP + 0x1;
  uStack4 = SUB42(&USHORT_1050_1050,0x0);
  piStack14 = DAT_1050_5f8a;
  piVar7 = DAT_1050_5f8a;
  if ((PTR_LOOP_1050_61ec != 0x0) &&
     (piVar7 = (i16 *)PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e,
     param_1 < (uchar *)(&PTR_LOOP_1050_0002 + 0x1U))) {
    param_1 = (uchar *)DAT_1050_5f8a;
  }
  if (piVar7 <= param_1) {
    uVar21 = true;
    piVar7 = (i16 *)0x900;
    goto LAB_1000_299d;
  }
  piVar16 = (i16 *)param_1;
  if ((param_1[0x5f90] & 0x20) != 0x0) {
    piVar7 = (i16 *)&PTR_DAT_1050_0041_1050_4202;
    param_5 = 0x1000;
    uVar21 = 0x0;
    if (true) {
      pcVar4 = (code *)swi(0x21);
      piVar7 = (i16 *)(*pcVar4)();
    }
    else {
      param_5 = s_tile2_bmp_1050_1538;
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    }
    if ((bool)uVar21) goto LAB_1000_299d;
  }
  uVar6 = SUB21(piVar7,0x0);
  pbVar15 = (byte *)param_2;
  puVar17 = puStack12;
  if ((*(byte *)(piVar16 + 0x2fc8) & 0x80) == 0x0) {
LAB_1000_3acf:
    puStack12 = puVar17;
    uVar21 = false;
    piVar7 = (i16 *)param_3;
    if (param_3 != (uchar *)0x0) {
      uVar21 = piVar16 < piStack14;
      if ((bool)uVar21) {
        uVar11 = CONCAT11(0x40,uVar6);
        uVar21 = 0x0;
        if (true) {
          pcVar4 = (code *)swi(0x21);
          uVar27 = (*pcVar4)();
        }
        else {
          DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
          uVar27 = CONCAT22(pbVar15,uVar11);
        }
      }
      else {
        piVar7 = pass1_1000_55b1(0x3b71,param_4,param_5);
        uVar27 = CONCAT22(pbVar15,piVar7);
      }
      piVar7 = (i16 *)uVar27;
      if ((bool)uVar21) {
        piVar7 = (i16 *)CONCAT11(0x9,(char)uVar27);
      }
      else {
        uVar21 = false;
        if (piVar7 == (i16 *)0x0) {
          if (((*(byte *)(piVar16 + 0x2fc8) & 0x40) == 0x0) ||
             (*(uVar27 >> 0x10) != '\x1a')) {
            uVar21 = true;
            piVar7 = (i16 *)0x1c00;
          }
          else {
            uVar21 = false;
          }
        }
      }
    }
  }
  else {
    uStack10 = SUB42(&USHORT_1050_1050,0x0);
    bVar24 = true;
    uStack6 = 0x0;
    uStack8 = 0x0;
    puStack12 = &stack0xffee;
    puVar17 = &stack0xffee;
    if (param_3 != (uchar *)0x0) {
      uVar6 = 0xa;
      puVar12 = param_3;
      pbVar18 = pbVar15;
      do {
        if (puVar12 == (uchar *)0x0) break;
        puVar12 = puVar12 + -0x1;
        pbVar1 = pbVar18;
        pbVar18 = pbVar18 + 0x1;
        bVar24 = *pbVar1 == 0xa;
      } while (!bVar24);
      param_4 = param_2._2_2_;
      puVar17 = &stack0xffee;
      if (!bVar24) goto LAB_1000_3acf;
      pcVar28 = param_2;
      uVar8 = pass1_1000_3bac();
      pcVar28._2_2_ = (pcVar28 >> 0x10);
      iVar13 = pcVar28;
      if (uVar8 < 0xa9) {
        piVar9 = exit_1000_25f2(0x3ad9,param_5,pcVar28._2_2_,-0x4,param_2._2_2_,param_5,
                                param_6);
        piVar7 = (i16 *)(pbVar18 + -iVar13);
        if (piVar7 == (i16 *)0x0) {
          return (uchar *)piVar9;
        }
        bVar22 = param_1 < piStack14;
        puVar3 = param_1 + -piStack14;
        cVar26 = puVar3 < 0x0;
        cVar25 = puVar3 == (uchar *)0x0;
        cVar23 = (POPCOUNT(puVar3 & 0xff) & 0x1U) == 0x0;
        if ((bool)bVar22) {
          piVar10 = (i16 *)CONCAT11(0x40,(char)piVar9);
          bVar22 = 0x0;
          cVar26 = '\0';
          cVar25 = '\x01';
          cVar23 = '\x01';
          if (true) {
            pcVar4 = (code *)swi(0x21);
            piVar10 = (i16 *)(*pcVar4)(&USHORT_1050_1050,puVar12,piVar16);
          }
          else {
            DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
          }
        }
        else {
          piVar10 = pass1_1000_55b1(0x3af1,param_2._2_2_,param_5);
        }
        if (!(bool)bVar22) {
          uStack6 += piVar10;
          bVar22 = piVar7 < piVar10;
          piVar7 = (i16 *)(piVar7 - piVar10);
          cVar26 = piVar7 < 0x0;
          cVar25 = piVar7 == (i16 *)0x0;
          cVar23 = (POPCOUNT(piVar7 & 0xff) & 0x1U) == 0x0;
          if ((bool)bVar22 || (bool)cVar25) {
            return (uchar *)piVar9;
          }
        }
        uVar8 = (byte)(cVar26 << 0x7 | cVar25 << 0x6 | param_7 << 0x4 |
                             cVar23 << 0x2 | 0x2U | bVar22) << 0x8;
        piVar7 = (i16 *)(piVar10 & 0xff | uVar8);
        puVar17 = puStack12;
        if (uStack6 == 0x0) {
          uVar21 = (uVar8 & 0x100) != 0x0;
          if ((bool)uVar21) {
            piVar7 = (i16 *)CONCAT11(0x9,(char)(piVar10 & 0xff));
          }
          else {
            if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
              uVar21 = true;
              piVar7 = (i16 *)0x1c00;
            }
            else {
              uVar21 = false;
            }
          }
          goto LAB_1000_299d;
        }
      }
      else {
        puVar17 = &stack0xffec;
        iVar13 = 0x200;
        if (uVar8 < 0x228) {
          iVar13 = 0x80;
        }
        iVar13 = -iVar13;
        puVar14 = &stack0xffec + iVar13;
        puVar19 = &stack0xffec + iVar13;
        (&stack0xffea + iVar13) = param_6;
        uVar20 = (&stack0xffea + iVar13);
        do {
          pbVar1 = pbVar15;
          pbVar15 = pbVar15 + 0x1;
          bVar22 = *pbVar1;
          uVar5 = uVar8 & 0xff00;
          uVar8 = uVar5 | bVar22;
          if (bVar22 == 0xa) {
            uVar8 = CONCAT11((char)(uVar5 >> 0x8),0xd);
            if (puVar19 == puVar17) {
              (&stack0xffea + iVar13) = 0x3abd;
              uVar8 = mixed_dos3_call_1000_3ad9
                                (uVar8,puVar14,&iStack2,puVar19,uVar20,param_5,param_6,
                                 param_7);
            }
            puVar2 = puVar19;
            puVar19 = puVar19 + 0x1;
            *puVar2 = (char)uVar8;
            uVar8 = CONCAT11((char)(uVar8 >> 0x8),0xa);
            uStack8 += 0x1;
          }
          if (puVar19 == puVar17) {
            (&stack0xffea + iVar13) = 0x3ac8;
            uVar8 = mixed_dos3_call_1000_3ad9
                              (uVar8,puVar14,&iStack2,puVar19,uVar20,param_5,param_6,
                               param_7);
          }
          puVar2 = puVar19;
          puVar19 = puVar19 + 0x1;
          *puVar2 = (char)uVar8;
          param_3 = param_3 + -0x1;
        } while (param_3 != (uchar *)0x0);
        (&stack0xffea + iVar13) = 0x3ab1;
        mixed_dos3_call_1000_3ad9
                  (uVar8,puVar14,&iStack2,puVar19,uVar20,param_5,param_6,param_7);
        puVar17 = puStack12;
      }
    }
    puStack12 = puVar17;
    uVar21 = uStack6 < uStack8;
    piVar7 = (i16 *)(uStack6 - uStack8);
  }
LAB_1000_299d:
  if ((bool)uVar21) {
    pass1_1000_29b5(piVar7);
    piVar7 = (i16 *)0xffff;
  }
  return (uchar *)piVar7;
}



// WARNING: Unable to track spacebase fully for stack

u16 
mixed_dos3_call_1000_3ad9
          (param_1: u16,param_2: i16,param_3: i16,param_4: i16,param_5: u16,param_6: u16,
          param_7: u16,char param_8)

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  code *pcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let piVar6: *mut i16;
  let piVar7: *mut i16;
  let uVar8: u16;
  let bVar9: u8;
  let bVar10: bool;
  let cVar11: u8;
  let cVar12: u8;
  let cVar13: u8;
  
  piVar7 = (i16 *)(param_4 - param_2);
  if (piVar7 == (i16 *)0x0) {
    return param_1;
  }
  uVar8 = (param_3 + 0x6);
  puVar1 = (u16 *)(param_3 + -0xc);
  bVar9 = uVar8 < *puVar1;
  uVar5 = uVar8 - *puVar1;
  cVar13 = uVar5 < 0x0;
  cVar12 = uVar5 == 0x0;
  cVar11 = (POPCOUNT(uVar5 & 0xff) & 0x1U) == 0x0;
  if ((bool)bVar9) {
    piVar6 = (i16 *)CONCAT11(0x40,(char)param_1);
    bVar9 = 0x0;
    cVar13 = '\0';
    cVar12 = '\x01';
    cVar11 = '\x01';
    if (true) {
      pcVar3 = (code *)swi(0x21);
      piVar6 = (i16 *)(*pcVar3)(&USHORT_1050_1050);
    }
    else {
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    }
  }
  else {
    piVar6 = pass1_1000_55b1(0x3af1,param_5,param_6);
  }
  if (!(bool)bVar9) {
    piVar2 = (i16 *)(param_3 + -0x4);
    *piVar2 = *piVar2 + piVar6;
    bVar9 = piVar7 < piVar6;
    piVar7 = (i16 *)(piVar7 - piVar6);
    cVar13 = piVar7 < 0x0;
    cVar12 = piVar7 == (i16 *)0x0;
    cVar11 = (POPCOUNT(piVar7 & 0xff) & 0x1U) == 0x0;
    if ((bool)bVar9 || (bool)cVar12) {
      return param_1;
    }
  }
  uVar4 = (byte)(cVar13 << 0x7 | cVar12 << 0x6 | param_8 << 0x4 | cVar11 << 0x2 |
                       0x2U | bVar9) << 0x8;
  uVar5 = piVar6 & 0xff | uVar4;
  if ((param_3 + -0x4) == 0x0) {
    bVar10 = (uVar4 & 0x100) != 0x0;
    if (bVar10) {
      uVar5 = CONCAT11(0x9,(char)(piVar6 & 0xff));
    }
    else {
      if (((*(byte *)(uVar8 + 0x5f90) & 0x40) == 0x0) ||
         (**(char **)(param_3 + 0x8) != '\x1a')) {
        bVar10 = true;
        uVar5 = 0x1c00;
      }
      else {
        bVar10 = false;
      }
    }
  }
  else {
    uVar5 = (param_3 + -0x4);
    puVar1 = (u16 *)(param_3 + -0x6);
    bVar10 = uVar5 < *puVar1;
    uVar5 -= *puVar1;
  }
  if (bVar10) {
    ((param_3 + -0xa) + 0x2) = 0x29a2;
    pass1_1000_29b5(uVar5);
    uVar5 = 0xffff;
  }
  return uVar5;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
sys_1000_3f9c(uchar *param_1,uchar *param_2,param_3: u16,param_4: u16,param_5: u16,
             param_6: i16,param_7: u16,param_8: u16,param_9: u16,param_10: u8)

{
  let puVar1: *mut u8;
  let uVar2: u16;
  let local_4: u16;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  PTR_LOOP_1050_68b2._0_1_ = 0x42;
  PTR_LOOP_1050_68ae = param_1;
  PTR_LOOP_1050_68b0 = param_2;
  _USHORT_1050_68a8 = CONCAT22(param_2,param_1);
  PTR_LOOP_1050_68ac = 0x7fff;
  uVar2 = sys_1000_30b4(&USHORT_1050_68a8,&USHORT_1050_1050,
                        (byte *)CONCAT22(param_4,param_3),&iStack2,
                        &USHORT_1050_68a8,param_7,param_8,param_9);
  puVar1 = _USHORT_1050_68a8;
  PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
  if (PTR_LOOP_1050_68ac < 0x0) {
    mem_1000_2bb6(0x0,(i16 *)&USHORT_1050_68a8,&iStack2,param_7,param_8,param_9,
                  param_10,param_2);
  }
  else {
    _USHORT_1050_68a8 =
         
         (_USHORT_1050_68a8 & 0xffff0000 | (USHORT_1050_68a8 + 0x1));
    *puVar1 = 0x0;
  }
  return uVar2;
}


fn mixed_sys_op_1000_40af
          (param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16) -> *mut i16

{
  let puVar1: *mut u16;
  let uVar2: u16;
  char *pcVar3;
  let puVar4: *mut u16;
  LPCSTR str;
  let puVar5: *mut u16;
  let uVar6: u16;
  let uVar7: u16;
  HGLOBAL16 HVar8;
  SEGPTR SVar9;
  let iVar10: i16;
  let uVar11: u16;
  let puVar12: *mut u16;
  char *pcVar13;
  let puVar14: *mut u16;
  let unaff_SS: u16;
  let bVar15: bool;
  let iVar16: i16;
  let uVar17: u16;
  
  do {
    uVar6 = (param_1 * param_3);
    uVar7 = param_2 * param_3 + (param_1 * param_3 >> 0x10);
    if ((uVar7 | uVar6) != 0x0) {
      puVar12 = (u16 *)0x0;
      if ((uVar7 < 0x3) && ((uVar7 < 0x2 || (uVar6 == 0x0)))) {
        if (uVar7 == 0x0) {
          uVar6 = uVar6 + 0xfff & 0xf000;
          if (uVar6 == 0x0) {
            uVar7 = 0x1;
          }
        }
        else {
          if ((param_3 - 0x1 & param_3) != 0x0) {
            puVar12 = (u16 *)((uVar7 << 0x10) % param_3);
            bVar15 = CARRY2(uVar6,puVar12);
            uVar6 += puVar12;
            if (bVar15) goto LAB_1000_41aa;
            uVar7 = 0x1;
          }
        }
      }
      else {
        if ((param_3 - 0x1 & param_3) != 0x0) goto LAB_1000_41aa;
      }
      uVar17 = 0x0;
      uVar11 = uVar7;
      HVar8 = GLobalAlloc16(0x1000,CONCAT22(uVar7,uVar6));
      if ((HVar8 != 0x0) && ((uVar17 & 0x1) != 0x0)) {
        SVar9 = WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        if ((SVar9 != 0x0) || (uVar7 == 0x0)) {
          iVar16 = 0x12;
          iVar10 = 0x12;
          pass1_1000_25a8(param_5,s_tile2_bmp_1050_1538);
          pass1_1000_2913(iVar10,param_5,(u16_t)s_tile2_bmp_1050_1538);
          str = poss_str_op_1000_28dc(iVar16);
          if (str == (*mut u8)0x0) goto LAB_1000_28cb;
          iVar10 = 0x9;
          if (*str == 'M') {
            iVar10 = 0xf;
          }
          str = str + iVar10;
          iVar10 = 0x22;
          pcVar13 = str;
          break;
        }
        HVar8 = pass1_1000_422a(uVar7,HVar8,s_tile2_bmp_1050_1538,unaff_SS);
        if (HVar8 == 0x0) {
          GlobalUnlock16((HGLOBAL16)s_tile2_bmp_1050_1538);
          GlobalFree16((HGLOBAL16)s_tile2_bmp_1050_1538);
          HVar8 = 0x0;
        }
      }
      param_4 = s_tile2_bmp_1050_1538;
      if (HVar8 != 0x0) {
        puVar14 = (u16 *)0x0;
        for (; uVar11 != 0x0; uVar11 -= 0x1) {
          for (iVar10 = -0x8000; iVar10 != 0x0; iVar10 += -0x1) {
            puVar4 = puVar14;
            puVar14 = puVar14 + 0x1;
            *puVar4 = 0x0;
          }
          HVar8 += 0x100;
        }
        if (uVar6 != 0x0) {
          for (; uVar6 != 0x0; uVar6 -= 0x1) {
            puVar4 = puVar14;
            puVar14 = (u16 *)(puVar14 + 0x1);
            *puVar4 = 0x0;
          }
        }
        return (i16 *)puVar12;
      }
    }
LAB_1000_41aa:
    if ((PTR_LOOP_1050_618e | PTR_LOOP_1050_618c) == 0x0) {
      return (i16 *)(u16 *)0x0;
    }
    iVar10 = (*(code *)PTR_LOOP_1050_618c)(param_4,param_3,param_1,param_2);
    if (iVar10 == 0x0) {
      return (i16 *)(u16 *)0x0;
    }
  } while( true );
  while( true ) {
    iVar10 += -0x1;
    pcVar3 = pcVar13;
    pcVar13 = pcVar13 + 0x1;
    if (*pcVar3 == '\r') break;
    if (iVar10 == 0x0) break;
  }
  pcVar13[-0x1] = '\0';
LAB_1000_28cb:
  FatalAppExit16(s_tile2_bmp_1050_1538,str);
  FatalExit();
  puVar12 = (u16 *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar12;
    puVar12 = puVar12 + 0x1;
    uVar2 = *puVar1;
    puVar5 = puVar12;
    if ((uVar2 == HVar8) || (puVar5 = (u16 *)(uVar2 + 0x1), puVar5 == (u16 *)0x0)) {
      return (i16 *)puVar5;
    }
    iVar10 = -0x1;
    do {
      if (iVar10 == 0x0) break;
      iVar10 += -0x1;
      puVar1 = puVar12;
      puVar12 = (u16 *)(puVar12 + 0x1);
    } while (*puVar1 != '\0');
  } while( true );
}


fn dos3_call_set_struct_1000_42de(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  code *pcVar4;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u8;
  let uVar13: u32;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar5 = *param_1;
  uVar7 = (iVar8 + 0x2);
  uVar6 = (iVar8 + 0x4);
  uVar11 = (iVar8 + 0x6);
  uVar1 = (iVar8 + 0x8);
  uVar9 = (iVar8 + 0xa);
  uVar10 = (param_3 >> 0x10);
  uVar2 = *param_3;
  uVar3 = (param_3 + 0x6);
  uVar12 = 0x0;
  if (true) {
    pcVar4 = (code *)swi(0x21);
    uVar13 = (*pcVar4)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    uVar13 = CONCAT22(uVar11,uVar5);
  }
  *param_3 = uVar2;
  (param_3 + 0x6) = uVar3;
  uVar11 = (param_2 >> 0x10);
  iVar8 = param_2;
  *param_2 = uVar13;
  (iVar8 + 0x2) = uVar7;
  (iVar8 + 0x4) = uVar6;
  (iVar8 + 0x6) = (uVar13 >> 0x10);
  (iVar8 + 0x8) = uVar1;
  (iVar8 + 0xa) = uVar9;
  if ((bool)uVar12) {
    pass1_1000_29af(uVar13);
  }
  (iVar8 + 0xc) = (byte)uVar12;
  return;
}



fn dos3_call_op_1000_435c(param_1: *mut u16,param_2: u16,param_3: u16,param_4: i16,param_5: u16)

{
  code *pcVar1;
  let UVar2: u16;
  let uVar3: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar4: u16;
  let cVar5: u8;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let iStack2: i16;
  
  iStack2 = param_4 + 0x1;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)(&USHORT_1050_1050);
    param_3 = extraout_DX;
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar3 = param_2;
  uVar4 = param_3;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    param_3 = extraout_DX_00;
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar9 = param_3 >> 0x8;
  uVar8 = uVar3 & 0xff;
  uVar6 = uVar3 >> 0x8;
  uVar7 = uVar6;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    cVar5 = (char)uVar6;
    param_3 = extraout_DX_01;
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    cVar5 = (char)uVar6;
  }
  if ((uVar4 != param_3) && (cVar5 == '\x17')) {
    uVar3 = param_2;
    param_3 = uVar4;
  }
  UVar2 = pass1_1000_462e(uVar3 - 0x7bc,param_3 >> 0x8,param_3 & 0xff,uVar7,uVar8,uVar9,
                          &iStack2,param_5,param_3);
  if (param_1._2_2_ != 0x0) {
    (param_1 + 0x2) = param_3;
    *param_1 = UVar2;
  }
  return;
}


fn dos3_call_1000_4f20(param_1: u16) -> u16

{
  code *pcVar1;
  let uVar2: u16;
  let uVar3: u8;
  
  uVar2 = 0x3950;
  uVar3 = 0x0;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(&USHORT_1050_1050,param_1 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if (!(bool)uVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return 0xffff;
}

fn dos3_call_1000_514e(param_1: i16) -> u16

{
  code *pcVar1;
  let uVar2: u16;
  let uVar3: u8;
  
  uVar2 = SUB42(s__ld__s_1050_4150,0x0);
  uVar3 = 0x0;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(&USHORT_1050_1050,param_1 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if (!(bool)uVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return 0xffff;
}

fn dos3_call_1000_5174(param_1: u16) -> u16

{
  code *pcVar1;
  let uVar2: u16;
  let uVar3: u8;
  
  uVar2 = 0x6850;
  uVar3 = 0x0;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(param_1 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if ((bool)uVar3) {
    pass1_1000_29b5(uVar2);
    return uVar2 & 0xff;
  }
  return 0x0;
}


fn dos3_calls_1000_5198(param_1: i16) -> u16

{
  code *pcVar1;
  let uVar2: u8;
  let uVar3: u16;
  let bVar4: u8;
  let uVar5: u16;
  
  uVar2 = 0x4f;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(&USHORT_1050_1050,param_1 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar3 = CONCAT11(uVar2,uVar2);
  bVar4 = 0x0;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar3 = (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar5 = bVar4 << 0x8;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if ((uVar5 & 0x100) != 0x0) {
    pass1_1000_29b5(uVar3);
    return uVar3 & 0xff;
  }
  return 0x0;
}



fn dos3_call_1000_51aa(param_1: u16) -> u16

{
  code *pcVar1;
  let uVar2: u8;
  let uVar3: u16;
  let bVar4: u8;
  let uVar5: u16;
  
  uVar2 = 0x4e;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(&USHORT_1050_1050,param_1 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar3 = CONCAT11(uVar2,uVar2);
  bVar4 = 0x0;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar3 = (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  uVar5 = bVar4 << 0x8;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if ((uVar5 & 0x100) != 0x0) {
    pass1_1000_29b5(uVar3);
    return uVar3 & 0xff;
  }
  return 0x0;
}


fn mixed_win_sys_op_1008_016e(param_1: u32,param_2: u16)
{
  code **ppcVar1;
  let puVar2: *mut u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: *mut u8
  let extraout_DX: *mut u8
  let puVar7: *mut u8
  let uVar8: u16;
  let unaff_DI: i16;
  let uVar9: u16;
  HINSTANCE16 instance;
  let uVar10: u16;
  let DVar11: u32;
  let puVar12: u32;
  let uVar13: u32;
  astruct_20 *paVar14;
  CHAR local_1be [0x80];
  CHAR local_13e [0xac];
  CHAR local_92 [0x80];
  let uStack18: u16;
  let puStack16: *mut u8
  let puStack14: u32;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let puStack4: *mut u8
  
  instance = (HINSTANCE16)s_tile2_bmp_1050_1538;
  DVar11 = GetVersion16();
  puVar7 = (uchar *)(DVar11 >> 0x10);
  uStack6 = (DVar11 & 0xffff);
  uVar4 = DVar11 & 0xff;
  uStack10 = (byte)((DVar11 & 0xffff) >> 0x8);
  uStack8 = uVar4;
  puStack4 = puVar7;
  if ((uVar4 < 0x3) || ((uVar4 == 0x3 && (uStack10 < 0xa)))) {
    uVar10 = 0x1000;
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    puVar6 = (uchar *)(puVar7 | uVar4);
    uStack18 = uVar4;
    puStack16 = puVar7;
    if (puVar6 == (uchar *)0x0) {
      iVar3 = 0x0;
      puVar6 = (uchar *)0x0;
    }
    else {
      uVar10 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(puVar7,uVar4),0x0,0x10,0x2,0x5de,
                               0x5dd,puVar6,param_2);
    }
    puStack14 = CONCAT22(puVar6,iVar3);
    ppcVar1 = (code **)(*puStack14 + 0x74);
    (**ppcVar1)(uVar10,iVar3,(char)puVar6);
    instance = 0x1000;
    puVar7 = extraout_DX;
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  debug_print_1008_6048(s_version__d__d_1050_0012,instance,param_2);
  if ((uStack8 == 0x3) && (0xb < uStack10)) {
    PTR_LOOP_1050_0010 = (&PTR_LOOP_1050_0000 + 0x1);
  }
  LoadString16(instance,0x80,local_92,param_2);
  uVar4 = dos3_call_1000_51aa(&stack0xfffe);
  if (uVar4 != 0x0) {
    LoadString16(0x1000,0x80,local_13e,param_2);
    LoadString16((HINSTANCE16)s_tile2_bmp_1050_1538,0x80,local_1be,param_2);
    uVar4 = MessageBox16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,
                         local_13e,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0x4,puVar7,0x1000);
  if ((puVar7 | uVar4) == 0x0) {
    uVar10 = 0x0;
    puVar6 = (uchar *)0x0;
    uStack18 = uVar4;
    puStack16 = puVar7;
  }
  else {
    uStack18 = uVar4;
    puStack16 = puVar7;
    puVar12 = pass1_1008_5394(CONCAT22(puVar7,uVar4));
    puVar6 = (uchar *)(puVar12 >> 0x10);
    uVar10 = SUB42(puVar12,0x0);
  }
  uVar9 = (param_1 >> 0x10);
  iVar3 = param_1;
  (iVar3 + 0x8) = uVar10;
  *(uchar **)(iVar3 + 0xa) = puVar6;
  uVar5 = (iVar3 + 0x8);
  puVar2 = (u16 *)(iVar3 + 0x8);
  _PTR_LOOP_1050_0298 = uVar5;
  *puVar2 = 0x70;
  (puVar2 + 0x2) = s_tile2_bmp_1050_1538;
  uVar10 = 0x1000;
  mem_op_1000_179c(0x126,puVar6,0x1000);
  uVar4 = uVar5;
  puVar7 = (uchar *)(puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    uVar10 = 0x1010;
    uVar13 = pass1_1010_2024(CONCAT13((char)(puVar6 >> 0x8),
                                      CONCAT12((char)puVar6,uVar4)));
    puVar7 = (uchar *)(uVar13 >> 0x10);
    uVar4 = uVar13;
  }
  if (_PTR_LOOP_1050_0ed0 == 0x0) {
    debug_print_1008_6048(s_New_failed_in_Op__Op_1050_0020,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xe8c,puVar7,0x1000);
  puVar6 = (uchar *)(puVar7 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (puVar6 != (uchar *)0x0) {
    uVar10 = 0x1010;
    pass1_1010_7e40(CONCAT22(puVar7,uVar4),puVar6,unaff_DI,param_2);
  }
  if (_PTR_LOOP_1050_14cc == 0x0) {
    debug_print_1008_6048(0x10500035,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xb0,puVar6,0x1000);
  puVar7 = (uchar *)(puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
    paVar14 = pass1_1038_aeca((astruct_20 *)CONCAT22(puVar6,uVar4),param_2);
    puVar7 = (uchar *)(paVar14 >> 0x10);
    uVar4 = paVar14;
  }
  if (_PTR_LOOP_1050_5b7c == 0x0) {
    debug_print_1008_6048
              (s_New_failed_in_Op__Op__DialogCtr_1050_0053,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xa,puVar7,0x1000);
  puVar6 = (uchar *)(puVar7 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (puVar6 != (uchar *)0x0) {
    uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
    make_proc_inst_1038_cf6c
              ((u16 *)CONCAT22(puVar7,uVar4),puVar6,&PTR_LOOP_1050_1038);
  }
  if (_PTR_LOOP_1050_5bc8 == 0x0) {
    debug_print_1008_6048
              (s_New_failed_in_Op__Op__DialogHand_1050_0073,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (uchar *)(puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    pass1_1008_5bdc((astruct_79 *)CONCAT22(puVar6,uVar4),unaff_DI,param_2);
  }
  if (_PTR_LOOP_1050_02a0 == 0x0) {
    debug_print_1008_6048
              (s_New_failed_in_Op__Op__Simulator_1050_0097,0x1000,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0xfc,puVar7,0x1000);
  uVar8 = puVar7 | uVar4;
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (uVar8 == 0x0) {
    uVar4 = 0x0;
    uVar8 = 0x0;
  }
  else {
    set_struct_op_1008_0536((u16 *)CONCAT22(puVar7,uVar4),0x1000,param_2);
  }
  (iVar3 + 0x4) = uVar4;
  (iVar3 + 0x6) = uVar8;
  if (*(long *)(iVar3 + 0x4) == 0x0) {
    debug_print_1008_6048(s_New_failed_in_Op__Op_1050_00b7,0x1000,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  win_ui_reg_class_1008_96d2(*(astruct_20 **)(iVar3 + 0x4),0x1000,param_2);
  uVar5 = (iVar3 + 0x4);
  ppcVar1 = (code **)((iVar3 + 0x4) + 0x8);
  (**ppcVar1)(0x1000,uVar5,(uVar5 >> 0x10));
  uVar5 = (iVar3 + 0x4);
  PTR_LOOP_1050_0396 = (uVar5 + 0x8);
  ppcVar1 = (code **)((iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1000,(iVar3 + 0x4),0x3);
  UpdateWindow16(0x1000);
  return;
}


fn kill_timer_1008_921c(param_1: *mut u16,HWND16 param_2)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x9416;
  (iVar1 + 0x2) = 0x1008;
  KillTimer16(param_2,0x1);
  _PTR_LOOP_1050_0388 = 0x0;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0x6)));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}


WPARAM16  win_msg_op_1008_9498(MSG *in_msg_1,MSG16 *in_msg_2)

{
  let BVar1: bool;
  let IVar2: i16;
  MSG16 local_msg_1;
  
LAB_1008_949c:
  BVar1 = GetMessage16((MSG16 *)in_msg_1,0x0,0x0,0x0);
  if (BVar1 == 0x0) {
    return local_msg_1.wparam;
  }
  if ((_PTR_LOOP_1050_5bc8 + 0x8) != 0x0) goto code_r0x100894cd;
  goto LAB_1008_94dc;
code_r0x100894cd:
  in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
  BVar1 = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_msg_1);
  if (BVar1 == 0x0) {
LAB_1008_94dc:
    if (PTR_LOOP_1050_0398 != 0x0) {
      in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
      IVar2 = TranslateAccelerator16
                        ((HWND16)s_tile2_bmp_1050_1538,(HACCEL16)&local_msg_1,in_msg_2);
      if (IVar2 != 0x0) goto LAB_1008_949c;
    }
    TranslateMessage16((MSG16 *)s_tile2_bmp_1050_1538);
    in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
    DispatchMessage16((MSG16 *)s_tile2_bmp_1050_1538);
  }
  goto LAB_1008_949c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn unk_win_msg_op_1008_9510(i16 *param_1,MSG16 *param_2,MSG16 *param_3)
{
  let has_message: bool;
  let IVar1: i16;
  MSG16 local_14;
  
LAB_1008_9578:
  if (*param_1 != 0x0) {
    has_message = GetMessage16(param_2,0x0,0x0,0x0);
    if (has_message != 0x0) {
      if ((_PTR_LOOP_1050_5bc8 + 0x8) != 0x0) goto code_r0x10089538;
      goto LAB_1008_9547;
    }
  }
  return;
code_r0x10089538:
  param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
  has_message = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_14);
  if (has_message == 0x0) {
LAB_1008_9547:
    if (PTR_LOOP_1050_0398 != 0x0) {
      param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
      IVar1 = TranslateAccelerator16
                        ((HWND16)s_tile2_bmp_1050_1538,(HACCEL16)&local_14,param_3);
      if (IVar1 != 0x0) goto LAB_1008_9578;
    }
    TranslateMessage16((MSG16 *)s_tile2_bmp_1050_1538);
    param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
    DispatchMessage16((MSG16 *)s_tile2_bmp_1050_1538);
  }
  goto LAB_1008_9578;
}


fn get_sys_metrics_1010_46f6(param_1: u32)
{
  let uVar1: u16;
  let IVar2: i16;
  let IVar3: i16;
  let in_DX: *mut u8
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let uVar7: u32;
  let puVar8: *mut u16;
  let puVar9: *mut u16;
  let local_6: i16;
  let local_4: i16;
  
  puVar9 = (u16 *)CONCAT22(unaff_SS,&local_4);
  puVar8 = (u16 *)CONCAT22(unaff_SS,&local_6);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((u16 *)(puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                  puVar8,puVar9);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar7 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x66));
  uVar1 = (uVar7 >> 0x10);
  (iVar4 + 0x18) = local_4 + 0x8;
  (iVar4 + 0x1a) = local_6 + 0x9;
  IVar2 = GetSystemMetrics16(0x1008);
  (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar7 + 0x4);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar7 + 0x8);
  return;
}


fn free_rsrc_1010_4b3e(param_1: *mut u16,HGLOBAL16 param_2)
{
  let piVar1: *mut i16;
  let puVar2: u32;
  let uVar3: u16;
  code **ppcVar4;
  let puVar5: u32;
  let uVar6: u32;
  let BVar7: bool;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u16;
  HGLOBAL16 HVar12;
  let unaff_SS: u16;
  let iStack4: i16;
  
  uVar10 = (param_1 >> 0x10);
  iVar8 = param_1;
  *param_1 = (s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6);
  (iVar8 + 0x2) = 0x1010;
  HVar12 = param_2;
  if ((iVar8 + 0x2a) != 0x0) {
    HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
    BVar7 = GlobalUnlock16(param_2);
    if (BVar7 == 0x0) {
      HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
      FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
  }
  (iVar8 + 0x2a) = 0x0;
  if (**(long **)(iVar8 + 0x12) != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      puVar5 = (iVar8 + 0x12);
      piVar1 = (i16 *)(puVar5 + 0x8);
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      uVar11 = (*puVar5 >> 0x10);
      iVar9 = *puVar5;
      puVar2 = (iVar9 + iStack4 * 0x4);
      uVar3 = (iVar9 + iStack4 * 0x4 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)(HVar12,puVar2,uVar3,0x1);
      }
      iStack4 += 0x1;
    }
  }
  uVar6 = (iVar8 + 0x12);
  fn_ptr_1000_17ce(*(astruct_18 **)(uVar6 + 0x4),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x12),0x1000);
  puVar2 = (iVar8 + 0x16);
  uVar3 = (iVar8 + 0x18);
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(0x1000,puVar2,uVar3,0x1);
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x1a),0x1000);
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}


fn find_n_load_rsrc_1010_4e9e(param_1: u32,HGLOBAL16 param_2)
{
  let BVar1: bool;
  HRSRC16 h_rsrc;
  let iVar2: i16;
  let uVar3: u16;
  HGLOBAL16 HVar3;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x20) != 0x0) {
    HVar3 = param_2;
    if ((iVar2 + 0x2a) != 0x0) {
      HVar3 = (HGLOBAL16)s_tile2_bmp_1050_1538;
      BVar1 = GlobalUnlock16(param_2);
      if (BVar1 == 0x0) {
        HVar3 = (HGLOBAL16)s_tile2_bmp_1050_1538;
        FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
      }
    }
    h_rsrc = FindResource16(HVar3,(LPCSTR)&PTR_LOOP_1050_000a,(LPCSTR)0x0);
    HVar3 = LoadResource16((HMODULE16)s_tile2_bmp_1050_1538,h_rsrc);
    *(HGLOBAL16 *)(iVar2 + 0x2a) = HVar3;
    if (HVar3 != 0x0) {
      WIN16_LockResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
      return;
    }
  }
  return;
}


fn win_sys_op_1010_5404(astruct_54 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  u16 **ppuVar2;
  let uVar3: u32;
  let puVar4: u32;
  code **ppcVar5;
  LPCSTR pCVar6;
  let iVar7: i16;
  let uVar8: u16;
  let puVar9: *mut u16;
  let uVar10: u16;
  let puVar11: *mut u8
  let extraout_DX: *mut u8
  let puVar12: *mut u8
  let extraout_DX_00: *mut u8
  let extraout_DX_01: *mut u8
  let puVar13: *mut u8;
  let puVar14: *mut u16;
  let unaff_DI: i16;
  let uVar15: u16;
  LPCSTR pCVar16;
  let index: i16;
  astruct_79 *paVar17;
  char *pcVar18;
  let puVar19: *mut u16;
  let uVar20: u16;
  let local_134: [u8;102];
  let puStack50: *mut u16;
  let uStack46: u16;
  let puStack44: *mut u8
  let iStack42: i16;
  let iStack26: i16;
  let puStack24: *mut u8
  let iStack22: i16;
  let puStack20: *mut u16;
  let uStack16: u32;
  let iStack12: i16;
  let uStack10: i16;
  let uStack8: u16;
  let puStack6: *mut u8
  let uStack4: u16;
  
  paVar17 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar12 = (uchar *)(paVar17 >> 0x10);
  uVar15 = 0x0;
  &param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  &param_1->field_0x1a = 0x0;
  param_1->field_0x62 = 0x0;
  param_1->field_0x64 = 0x0;
  &param_1->field_0x68 = 0x0;
  &param_1->field_0x6c = 0x0;
  param_1->field_0x70 = 0x1;
  param_1->field_0x7a = 0x0;
  param_1->field_0x7c = 0x0;
  param_1->field_0x7e = 0x0;
  param_1->field_0x80 = 0x0;
  param_1->field_0x82 = 0x1;
  CONCAT22(param_2,param_1) = 0x6312;
  param_1->field_0x2 = 0x1010;
  pass1_1010_6034(CONCAT22(param_2,param_1),puVar12);
  mem_op_1000_179c(0x101,puVar12,0x1000);
  &param_1->field_0xe = uVar15;
  *(uchar **)(&param_1->field_0xe + 0x2) = puVar12;
  pass1_1000_5008(&param_1->field_0xe,puVar12,0x100,&stack0xfffe);
  uStack4 = str_op_1000_3da4(param_1->field_0xe);
  pcVar18 = param_1->field_0xe;
  uVar15 = (pcVar18 >> 0x10);
  puVar13 = (pcVar18 + uStack4);
  if (puVar13[-0x1] != '\\') {
    *puVar13 = 0x5c;
    pcVar18 = param_1->field_0xe;
    *(pcVar18 + uStack4 + 0x1) = 0x0;
  }
  pcVar18 = load_string_1010_847e
                      (_PTR_LOOP_1050_14cc,
                       (INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  puVar12 = (uchar *)(pcVar18 >> 0x10);
  uStack8 = SUB42(pcVar18,0x0);
  puStack6 = puVar12;
  pass1_1000_3cea((ULONG)param_1->field_0xe,(ULONG)pcVar18);
  pCVar6 = (LPCSTR)str_op_1008_60e8(param_1->field_0xe,puVar12);
  param_1->field_0xa = pCVar6;
  param_1->field_0xc = puVar12;
  pcVar18 = param_1->field_0xe;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            ((LPCSTR)0x1008,param_1->field_0xa,(LPCSTR)puVar12,
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    iStack22 = pass1_1000_3e2c(param_1->field_0xe);
    puVar19 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar12,unaff_DI);
    puVar12 = (uchar *)(puVar19 >> 0x10);
    iStack26 = puVar19;
    iStack10 = (iStack26 + 0xa);
    iStack12 = (iStack26 + 0xc);
    param_1->field_0x62 = (iStack22 != iStack10);
    puStack24 = puVar12;
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e(param_1->field_0xe,0x105013c4);
    if (iVar7 == 0x0) {
      param_1->field_0x80 = 0x1;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e(param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x74 = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e(param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x72 = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e(param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x1e = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e(param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x20 = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  puVar11 = puVar12;
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    uStack46 = pass1_1000_3e2c(param_1->field_0xe);
    puVar11 = (uchar *)(puVar12 | uStack46);
    puStack44 = puVar12;
    if ((uchar *)(puVar12 | uStack46) != (uchar *)0x0) {
      param_1->field_0x76 = uStack46;
      param_1->field_0x78 = puVar12;
      puVar11 = puVar12;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e(param_1->field_0xe,0x105013c4);
    if (iVar7 == 0x0) {
      param_1->field_0x7a = 0x1;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)0x1008;
    uVar8 = str_op_1008_60e8(param_1->field_0xe,puVar11);
    param_1->field_0x1a = uVar8;
    param_1->field_0x1c = puVar11;
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)0x1008;
    uVar8 = str_op_1008_60e8(param_1->field_0xe,puVar11);
    param_1->field_0x68 = uVar8;
    param_1->field_0x6a = puVar11;
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  index = (INT16)s_tile2_bmp_1050_1538;
  puVar9 = (u16 *)GetPrivateProfileString16
                             (pCVar16,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
                              (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21)
                              ,pcVar18,(LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    index = 0x1008;
    puVar9 = (u16 *)str_op_1008_60e8(param_1->field_0xe,puVar11);
    param_1->field_0x6c = puVar9;
    param_1->field_0x6e = puVar11;
  }
  if (param_1->field_0x62 == 0x0) {
    uVar15 = SUB42(s_tile2_bmp_1050_1538,0x0);
    uStack46 = GetSystemMetrics16(index);
    iStack42 = 0x1;
    do {
      get_private_profile_string_1010_6132(CONCAT22(param_2,param_1),iStack42,uVar15);
      puVar14 = &param_1->field_0x0 + iStack42 * 0x4;
      if ((((puVar14[0x11] < 0x0) || (puVar14[0x12] < 0x0)) ||
          (piVar1 = puVar14 + 0x11,
          *piVar1 != iStack10 - uStack46 && (iStack10 - uStack46) <= *piVar1)) ||
         (puVar9 = (u16 *)(iStack12 - uStack46), ppuVar2 = (u16 **)(puVar14 + 0x12),
         *ppuVar2 != puVar9 && puVar9 <= *ppuVar2)) {
        uVar15 = 0x1000;
        puVar9 = pass1_1000_4906((astruct_20 *)
                                 CONCAT22(param_2,&param_1->field_0x22 + iStack42 * 0x8),
                                 (WNDCLASS16 *)0x0,0x8);
      }
      iStack42 += 0x1;
    } while (iStack42 < 0x8);
  }
  mem_op_1000_179c(0xc,puVar11,0x1000);
  puStack50 = (u16 *)CONCAT22(puVar11,puVar9);
  if ((puVar11 | puVar9) == 0x0) {
    puVar9 = (u16 *)0x0;
    puVar12 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar11,puVar9));
    puVar12 = extraout_DX;
  }
  *(u16 **)&param_1->field_0x64 = puVar9;
  *(uchar **)(&param_1->field_0x64 + 0x2) = puVar12;
  pcVar18 = param_1->field_0xe;
  pass1_1000_5008(pcVar18,(pcVar18 >> 0x10),0x100,&stack0xfffe
                 );
  uStack4 = str_op_1000_3da4(param_1->field_0xe);
  pcVar18 = param_1->field_0xe;
  uVar15 = (pcVar18 >> 0x10);
  puVar13 = (pcVar18 + uStack4);
  if (puVar13[-0x1] != '\\') {
    *puVar13 = 0x5c;
    pcVar18 = param_1->field_0xe;
    *(pcVar18 + uStack4 + 0x1) = 0x0;
  }
  uVar10 = str_op_1008_60e8(param_1->field_0xe,puVar12);
  uStack16 = CONCAT22(puVar12,uVar10);
  mem_op_1000_179c(0x8,puVar12,0x1000);
  puStack50 = (u16 *)CONCAT22(puVar12,uVar10);
  if ((puVar12 | uVar10) == 0x0) {
    puStack20 = (u16 *)0x0;
  }
  else {
    *puStack50 = 0x389a;
    (uVar10 + 0x2) = 0x1008;
    (uVar10 + 0x4) = uStack16;
    *puStack50 = 0x6322;
    (uVar10 + 0x2) = 0x1010;
    puStack20 = puStack50;
  }
  puVar4 = param_1->field_0x64;
  ppcVar5 = (code **)(*param_1->field_0x64 + 0x4);
  (**ppcVar5)(0x1000,puVar4,(puVar4 >> 0x10),puStack20,
              (puStack20 >> 0x10));
  pcVar18 = param_1->field_0xe;
  uVar3 = &param_1->field_0xa;
  puVar12 = extraout_DX_00;
  GetPrivateProfileString16
            ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),pcVar18,
             (LPCSTR)(pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pcVar18 = param_1->field_0xe;
    uVar15 = SUB42(pcVar18,0x0);
    uVar20 = (pcVar18 >> 0x10);
    while (uStack46 = pass1_1000_47a4(CONCAT22(uVar20,uVar15),0x105013f8,param_4),
          (puVar12 | uStack46) != 0x0) {
      puStack44 = puVar12;
      unk_str_op_1000_3d3e
                (CONCAT22(param_4,local_134),CONCAT22(puVar12,uStack46));
      uStack4 = str_op_1000_3da4(CONCAT22(param_4,local_134));
      if ((&stack0xfecb)[uStack4] != '\\') {
        local_134[uStack4] = 0x5c;
        local_134[uStack4 + 0x1] = 0x0;
      }
      uVar10 = str_op_1008_60e8(CONCAT22(param_4,local_134),puVar12);
      uStack16 = CONCAT22(puVar12,uVar10);
      mem_op_1000_179c(0x8,puVar12,0x1000);
      puStack50 = (u16 *)CONCAT22(puVar12,uVar10);
      if ((puVar12 | uVar10) == 0x0) {
        puStack20 = (u16 *)0x0;
      }
      else {
        *puStack50 = 0x389a;
        (uVar10 + 0x2) = 0x1008;
        (uVar10 + 0x4) = uStack16;
        *puStack50 = 0x6322;
        (uVar10 + 0x2) = 0x1010;
        puStack20 = puStack50;
      }
      puVar4 = param_1->field_0x64;
      ppcVar5 = (code **)(*param_1->field_0x64 + 0x8);
      (**ppcVar5)(0x1000,puVar4,(puVar4 >> 0x10),puStack20,
                  (puStack20 >> 0x10));
      uVar15 = 0x0;
      uVar20 = 0x0;
      puVar12 = extraout_DX_01;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn write_private_profile_str_1010_5b10(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u32;
  code **ppcVar4;
  LPCSTR pCVar5;
  let in_DX: *mut u8
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let unaff_SS: u16;
  let in_AF: u8;
  let puVar8: *mut u16;
  let iStack12: i16;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  *param_1 = 0x6312;
  (iVar6 + 0x2) = 0x1010;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  uVar3 = (iVar6 + 0xe);
  sys_1000_3f9c((uchar *)uVar3,(uchar *)(uVar3 >> 0x10),0x149c,
                &USHORT_1050_1050,(puVar8 + 0xa),&stack0xfffe,
                uVar7,0x1000,unaff_SS,in_AF);
  if ((iVar6 + 0x80) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             pCVar5);
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPCSTR)(iVar6 + 0xe));
  if ((iVar6 + 0x1e) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             pCVar5);
  if ((iVar6 + 0x74) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             pCVar5);
  if ((iVar6 + 0x72) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             pCVar5);
  if ((iVar6 + 0x20) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             pCVar5);
  uVar3 = (iVar6 + 0xe);
  sys_1000_3f9c((uchar *)uVar3,(uchar *)(uVar3 >> 0x10),0x14a2,
                &USHORT_1050_1050,(iVar6 + 0x76),
                &stack0xfffe,uVar7,0x1000,unaff_SS,in_AF);
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPCSTR)(iVar6 + 0xe));
  if ((iVar6 + 0x7a) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             pCVar5);
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPCSTR)(iVar6 + 0x1a));
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPCSTR)(iVar6 + 0x68));
  uVar3 = (iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)(uVar3 >> 0x10),
             (LPCSTR)(iVar6 + 0x6c));
  iStack12 = 0x1;
  do {
    switchD_1010:2ab5::caseD_13(param_1,iStack12);
    iStack12 += 0x1;
  } while (iStack12 < 0x8);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0xa),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0xe),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x12),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x16),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x1a),0x1000);
  puVar1 = (iVar6 + 0x64);
  uVar2 = (iVar6 + 0x66);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)(0x1000,puVar1,uVar2,0x1);
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x68),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x6c),0x1000);
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}


fn get_private_profile_string_1010_6132(param_1: u32,param_2: i16,LPCSTR param_3)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  let in_DX: u16;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let unaff_SS: u16;
  
  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  uVar1 = (iVar7 + 0xe);
  uVar2 = (iVar7 + 0xa);
  GetPrivateProfileString16
            (param_3,(LPCSTR)uVar2,(LPCSTR)(uVar2 >> 0x10),
             (LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),uVar1,
             (LPCSTR)(uVar1 >> 0x10));
  if (**(char **)(iVar7 + 0xe) != '\0') {
    uVar3 = pass1_1000_47a4((iVar7 + 0xe),0x105014a6,unaff_SS);
    uVar5 = in_DX | uVar3;
    if (uVar5 != 0x0) {
      iVar4 = pass1_1000_3e2c(CONCAT22(in_DX,uVar3));
      iVar7 = param_2 * 0x8 + iVar7;
      (iVar7 + 0x22) = iVar4;
      uVar3 = pass1_1000_47a4(0x0,0x105014a8,unaff_SS);
      uVar6 = uVar5 | uVar3;
      if (uVar6 != 0x0) {
        iVar4 = pass1_1000_3e2c(CONCAT22(uVar5,uVar3));
        (iVar7 + 0x24) = iVar4;
        uVar3 = pass1_1000_47a4(0x0,0x105014aa,unaff_SS);
        uVar5 = uVar6 | uVar3;
        if (uVar5 != 0x0) {
          iVar4 = pass1_1000_3e2c(CONCAT22(uVar6,uVar3));
          (iVar7 + 0x26) = iVar4;
          uVar3 = pass1_1000_47a4(0x0,0x105014ac,unaff_SS);
          if ((uVar5 | uVar3) != 0x0) {
            iVar4 = pass1_1000_3e2c(CONCAT22(uVar5,uVar3));
            (iVar7 + 0x28) = iVar4;
          }
        }
      }
    }
  }
  return;
}


fn set_err_mode_1010_8b14(param_1: u32,Uparam_2: i32,param_3: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let lVar3: i32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0xe84));
  SetErrorMode16(0x1008);
  do {
    lVar3 = pass1_1008_5b12(local_a,param_3);
    if (lVar3 == 0x0) {
      SetErrorMode16(0x1008);
      return param_2;
    }
    uVar1 = param_1 + 0xa82;
    unk_str_op_1000_3d3e
              ((param_1 & 0xffff0000 | uVar1),*(char **)(lVar3 + 0x4))
    ;
    pass1_1000_3cea(param_1 & 0xffff0000 | uVar1,param_2);
    uVar2 = dos3_call_1000_51aa(&stack0xfffe);
  } while (uVar2 != 0x0);
  SetErrorMode16(0x1000);
  return param_1 & 0xffff0000 | uVar1;
}


fn get_sys_metrics_1018_09a8(param_1: u32,INT16 param_2)
{
  let uVar1: u32;
  let IVar2: i16;
  let IVar3: i16;
  let in_DX: *mut u8
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
  let puVar8: *mut u16;
  let local_a: i16;
  let local_8: i16;
  let iStack6: i16;
  let IStack4: i16;
  
  IStack4 = GetSystemMetrics16(param_2);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  iStack6 = (iVar4 + 0x12) + -0x2;
  puVar8 = (u16 *)CONCAT22(unaff_SS,&local_8);
  puVar7 = (u16 *)CONCAT22(unaff_SS,&local_a);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((u16 *)(puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                  puVar7,puVar8);
  (iVar4 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
  (iVar4 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
  IVar2 = GetSystemMetrics16(0x1008);
  uVar1 = (iVar4 + 0x5a);
  (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar1 + 0x4);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  uVar1 = (iVar4 + 0x5a);
  (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar1 + 0x8);
  return;
}


fn get_sys_metrics_1018_1ea0(astruct_55 *param_1,param_2: u16)
{
  let IVar1: i16;
  let IVar2: i16;
  astruct_55 *iVar3;
  let uVar3: u16;
  
  IVar1 = GetSystemMetrics16(param_2);
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_55 *)param_1;
  iVar3->field_0x2e = IVar1 * 0x2 + iVar3->field_0x36;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x30 = IVar1 + iVar3->field_0x38 + IVar2;
  return;
}


fn mixed_sys_op_1018_2978(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let puVar3: *mut u8;
  RECT16 *rect;
  let iVar4: i16;
  let in_DX: *mut u8
  let uVar5: u16;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u8;
  astruct_76 *paStack62;
  RECT16 local_3a;
  let iStack54: i16;
  let iStack52: i16;
  let uStack50: u32;
  let puStack46: u32;
  let local_2a: [u8;24];
  let uStack6: u16;
  
  pass1_1010_8096(_PTR_LOOP_1050_14cc,0x1);
  puVar2 = local_2a;
  uStack6 = param_2;
  struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3,puVar2),0x1,
                      CONCAT22(in_DX,param_2),in_DX);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,in_DX,0x1000);
  uVar5 = in_DX | puVar2;
  if (uVar5 == 0x0) {
    puVar3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    puVar3 = local_2a;
    uVar9 = 0x1008;
    struct_op_1008_3f92((astruct_76 *)CONCAT22(in_DX,puVar2),
                        (astruct_83 *)CONCAT22(param_3,puVar3));
  }
  puStack46 = CONCAT22(uVar5,puVar3);
  ppcVar1 = (code **)(*puStack46 + 0x14);
  (**ppcVar1)(uVar9,puVar3,uVar5);
  uStack50 = CONCAT22(extraout_DX,puVar3);
  puVar6 = extraout_DX;
  mem_op_1000_179c(0x14,extraout_DX,0x1000);
  puVar7 = (uchar *)(puVar6 | puVar3);
  if (puVar7 == (uchar *)0x0) {
    puVar3 = 0x0;
    puVar7 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((u16 *)
                     CONCAT13((char)(puVar6 >> 0x8),CONCAT12((char)puVar6,puVar3)));
  }
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  (iVar8 + 0xe) = puVar3;
  *(uchar **)(iVar8 + 0x10) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0xe),uStack50,puVar7);
  uVar12 = SUB21(PTR_LOOP_1050_0396,0x0);
  rect = &local_3a;
  GetClientRect16(0x1008,rect);
  uVar11 = 0x1e;
  uVar10 = 0x1000;
  mem_op_1000_179c(0x1e,puVar7,0x1000);
  paStack62 = (astruct_76 *)CONCAT22(puVar7,rect);
  uVar5 = puVar7 | rect;
  if (uVar5 == 0x0) {
    (iVar8 + 0xa) = 0x0;
  }
  else {
    iVar4 = (iStack52 - local_3a.y) + 0x1;
    uVar10 = 0x1008;
    pass1_1008_405c(paStack62,(iVar8 + 0xe),iVar4,(iStack54 - local_3a.x) + 0x1)
    ;
    (iVar8 + 0xa) = iVar4;
    (iVar8 + 0xc) = uVar5;
  }
  if (puStack46 != 0x0) {
    ppcVar1 = (code **)*puStack46;
    (**ppcVar1)(uVar10,puStack46,(puStack46 >> 0x10),0x1,uVar11,uVar12);
  }
  close_file_1008_496c(local_2a,param_3);
  return;
}


fn get_sys_metrics_1018_2f56(param_1: u32)
{
  let uVar1: u16;
  let IVar2: i16;
  let IVar3: i16;
  let in_DX: *mut u8
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let uVar7: u32;
  let puVar8: *mut u16;
  let puVar9: *mut u16;
  let local_6: i16;
  let local_4: i16;
  
  puVar9 = (u16 *)CONCAT22(unaff_SS,&local_4);
  puVar8 = (u16 *)CONCAT22(unaff_SS,&local_6);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((u16 *)(puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                  puVar8,puVar9);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar7 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x24));
  uVar1 = (uVar7 >> 0x10);
  (iVar4 + 0x18) = local_4 + 0xb5;
  (iVar4 + 0x1a) = local_6 + 0x9;
  IVar2 = GetSystemMetrics16(0x1008);
  (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar7 + 0x4);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar7 + 0x8);
  return;
}


fn sprintf_op_1018_34b6(param_1: u32,param_2: u8)
{
  let iVar1: i16;
  undefined3 in_register_00000001;
  let in_DX: u16;
  let iVar2: i16;
  let valist: *mut u16;
  LPSTR buffer;
  let unaff_SS: u16;
  let uVar3: u32;
  let lVar4: i32;
  
  valist = (u16 *)(param_1 >> 0x10);
  iVar2 = param_1;
  uVar3 = switch_1018_3b9e(param_1,(iVar2 + 0x12e),
                           CONCAT31(in_register_00000001,param_2),in_DX,unaff_SS);
  iVar1 = (iVar2 + 0x12e);
  if (iVar1 == 0x188) {
    lVar4 = pass1_1008_57f0(uVar3,(iVar2 + 0x130),unaff_SS);
    buffer = (LPSTR)0x1020;
    string_1020_c0d8((lVar4 + 0xe));
  }
  else {
    if (iVar1 == 0x18b) {
      buffer = (LPSTR)0x1008;
      pass1_1008_57f0(uVar3,(iVar2 + 0x130),unaff_SS);
    }
    else {
      if (iVar1 != 0x18c) {
        load_string_1010_84e0
                  (0x1010,_PTR_LOOP_1050_14cc,
                   (_PTR_LOOP_1050_14cc >> 0x10),0x100,
                   (iVar2 + 0x22),(short)valist);
        return;
      }
      buffer = (LPSTR)0x1008;
      pass1_1008_57f0(uVar3,(iVar2 + 0x130),unaff_SS);
    }
  }
  wsprintf16(buffer,(LPCSTR)(iVar2 + 0x22),valist);
  return;
}



fn get_sys_metrics_1018_4b1e(astruct_55 *param_1,param_2: u16,param_3: u16) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_op_1010_1d48((astruct_79 *)param_1,param_3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x12) = param_2;
  (iVar1 + 0x14) = 0x0;
  param_1->field_0x0 = &PTR_LOOP_1050_4c9e;
  (iVar1 + 0x2) = 0x1018;
  if (PTR_LOOP_1050_416c == 0x0) {
    PTR_LOOP_1050_416c = GetSystemMetrics16(0x1010);
    PTR_LOOP_1050_416e = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    PTR_LOOP_1050_4170 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  }
  return (u16 *)param_1;
}



fn get_sys_metrics_1020_7c1a(param_1: *mut u16,param_2: u32,INT16 param_3)
{
  let IVar1: i16;
  astruct_56 *iVar3;
  let uVar3: u16;
  let uVar4: u16;
  let uVar1: u16;
  
  uVar3 = (param_2 >> 0x10);
  uVar1 = (param_2 + 0x8);
  uVar4 = (param_1 >> 0x10);
  iVar3 = (astruct_56 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = uVar1;
  *param_1 = 0x3ab0;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x6 = param_2;
  iVar3->field_0xa = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x10 = 0x0;
  iVar3->field_0x12 = 0x0;
  *param_1 = 0x7f72;
  iVar3->field_0x2 = 0x1020;
  iVar3->field_0xa = (param_2 + 0xe4);
  IVar1 = GetSystemMetrics16(param_3);
  iVar3->field_0xe = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x10 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x12 = IVar1;
  return;
}


fn make_proc_inst_1038_cf6c(param_1: *mut u16,uchar *param_2,LPVOID param_3)
{
  LPVOID pvVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  *param_1 = 0x389a;
  (iVar2 + 0x2) = 0x1008;
  (iVar2 + 0x4) = 0x0;
  (iVar2 + 0x8) = 0x0;
  *param_1 = 0xd23e;
  (iVar2 + 0x2) = &PTR_LOOP_1050_1038;
  _PTR_LOOP_1050_5bc8 = param_1;
  pvVar1 = MakeProcInstance16(param_3,(HANDLE16)PTR_LOOP_1050_038c);
  *(LPVOID *)(iVar2 + 0x4) = pvVar1;
  *(uchar **)(iVar2 + 0x6) = param_2;
  PTR_LOOP_1050_5bcc =
       
       MakeProcInstance16((LPVOID)s_tile2_bmp_1050_1538,(HANDLE16)PTR_LOOP_1050_038c);
  PTR_LOOP_1050_5bce = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn free_proc_inst_1038_cfda(param_1: *mut u16,LPVOID param_2)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0xd23e;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
  FreeProcInstance16(param_2);
  FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
  (iVar1 + 0x4) = 0x0;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}



long 
call_win_proc_1038_d020
          (HWND16 param_1,param_2: u32,LPARAM param_3,param_4: u16,HWND16 param_5)

{
  code **ppcVar1;
  WPARAM16 wparam;
  HANDLE16 HVar2;
  HANDLE16 HVar3;
  let uVar4: u16;
  LRESULT LVar5;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let lStack14: i32;
  let puStack10: u32;
  let lStack6: i32;
  
  uVar9 = SUB42(&USHORT_1050_1050,0x0);
  uVar8 = param_3._2_2_;
  HVar2 = GetProp16(param_5,(LPCSTR)s_procHi_1050_5bd7);
  uVar7 = param_3._2_2_;
  HVar3 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5bd0);
  lStack6 = CONCAT22(HVar2,HVar3);
  uVar6 = param_3._2_2_;
  HVar2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5be5);
  HVar3 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5bde);
  puStack10 = CONCAT22(HVar2,HVar3);
  wparam = (WPARAM16)(param_2 >> 0x10);
  if ((HVar2 | HVar3) != 0x0) {
    lStack14 = 0x0;
    if (param_3 == 0x19) {
      ppcVar1 = (code **)(*puStack10 + 0x34);
      lStack14 = (**ppcVar1)(s_tile2_bmp_1050_1538,(char)HVar3,HVar2,param_1,param_2,
                             param_3._2_2_,uVar6,uVar7,uVar8,uVar9);
    }
    else {
      if (param_3 == 0x86) {
        ppcVar1 = (code **)(*puStack10 + 0x20);
        uVar4 = (**ppcVar1)(s_tile2_bmp_1050_1538,HVar3,HVar2,wparam);
        goto LAB_1038_d10e;
      }
      if ((param_3 == 0x112) && ((wparam & 0xfff0) == 0xf140)) {
        LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x112f140);
        uVar4 = (LVar5 == 0x0);
        goto LAB_1038_d10e;
      }
    }
    if (lStack14 != 0x0) {
      return lStack14;
    }
  }
  if (lStack6 != 0x0) {
    LVar5 = CallWindowProc16((LPVOID)s_tile2_bmp_1050_1538,param_1,param_2,wparam,
                             param_3);
    return LVar5;
  }
  uVar4 = 0x0;
LAB_1038_d10e:
  return (long)uVar4;
}



void 
win_prop_op_1038_d118
          (param_1: u32,param_2: u32,param_3: u16,param_4: u16,HWND16 param_5)

{
  code **ppcVar1;
  let uVar2: u32;
  let cVar3: u8;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let puStack6: u32;
  
  uVar8 = SUB42(&USHORT_1050_1050,0x0);
  uVar7 = param_3;
  HVar4 = GetProp16(param_5,(LPCSTR)s_thisHi_1050_5bf3);
  uVar6 = param_3;
  HVar5 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5bec);
  puStack6 = CONCAT22(HVar4,HVar5);
  if (param_2._2_2_ == 0x30) {
    if ((LPCSTR)param_2 == (LPCSTR)0x0) {
      return;
    }
    SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)param_2,0x5c06);
    return;
  }
  if (param_2 < 0x310000) {
    cVar3 = (char)(param_2 >> 0x10);
    if (cVar3 == '\x02') {
      if ((HVar4 | HVar5) != 0x0) {
        uVar2 = *puStack6;
        ppcVar1 = (code **)uVar2 + 0x6;
        (**ppcVar1)(s_tile2_bmp_1050_1538,HVar5,HVar4,param_1,param_2,uVar6,uVar7,
                    uVar8);
        if (puStack6 != 0x0) {
          ppcVar1 = (code **)uVar2;
          (**ppcVar1)(s_tile2_bmp_1050_1538,HVar5,HVar4,0x1);
        }
      }
      HVar4 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5bfa);
      if (HVar4 == 0x0) {
        return;
      }
      DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
      RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5c00);
      return;
    }
    if (cVar3 == '\x06') {
      if (((LPCSTR)param_2 != (LPCSTR)(&PTR_LOOP_1050_0000 + 0x1)) &&
         ((LPCSTR)param_2 != (LPCSTR)&PTR_LOOP_1050_0002)) {
        uVar2 = &PTR_LOOP_1050_5bc8;
        (uVar2 + 0x8) = 0x0;
        return;
      }
      uVar2 = &PTR_LOOP_1050_5bc8;
      (uVar2 + 0x8) = param_3;
      return;
    }
  }
  if ((HVar4 | HVar5) != 0x0) {
    ppcVar1 = (code **)(*puStack6 + 0xc);
    (**ppcVar1)(s_tile2_bmp_1050_1538,HVar5,HVar4,param_1,param_2);
  }
  return;
}


void 
get_sys_metrics_1040_7728
          (astruct_57 *param_1,param_2: u16,param_3: u32,param_4: u16,param_5: u16)

{
  let IVar1: i16;
  astruct_57 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_57 *)param_1;
  param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  param_1 = 0x3aa8;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = 0x0;
  iVar2->field_0x6 = 0x0;
  iVar2->field_0x8 = param_5;
  iVar2->field_0xa = param_4;
  iVar2->field_0xc = 0x0;
  iVar2->field_0x60 = 0x0;
  iVar2->field_0x62 = 0x0;
  iVar2->field_0x64 = 0x0;
  iVar2->field_0x66 = 0x0;
  iVar2->field_0x68 = 0x0;
  iVar2->field_0x6a = param_3;
  iVar2->field_0x6e = param_2;
  iVar2->field_0x70 = 0x0;
  iVar2->field_0x74 = 0x0;
  iVar2->field_0x76 = 0x0;
  iVar2->field_0x78 = 0x0;
  iVar2->field_0x8a = 0x0;
  iVar2->field_0x8c = 0x0;
  param_1 = 0x840c;
  iVar2->field_0x2 = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | &iVar2->field_0x10),
             0x10505db0);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar2->field_0x7a),
                  (WNDCLASS16 *)0x0,0x8);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar2->field_0x82),
                  (WNDCLASS16 *)0x0,0x8);
  IVar1 = GetSystemMetrics16(0x1000);
  iVar2->field_0x62 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar2->field_0x64 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar2->field_0x66 = IVar1;
  return;
}


fn get_sys_metrics_1040_8c66(astruct_37 *param_1,HWND16 param_2)
{
  let piVar1: *mut i16;
  let bVar2: u8;
  HDC16 hdc;
  let IVar3: i16;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  hdc = GetDC16(param_2);
  draw_text_1040_8d14(param_1,s_tile2_bmp_1050_1538);
  (iVar4 + 0xa6) = (iVar4 + 0x9e);
  (iVar4 + 0xaa) = (iVar4 + 0xa2);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  piVar1 = (i16 *)(iVar4 + 0xac);
  *piVar1 = *piVar1 + IVar3;
  bVar2 = *(byte *)(iVar4 + 0x98) & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    if ((iVar4 + 0xac) < IVar3) {
      IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
      *(INT16 *)(iVar4 + 0xac) = IVar3;
    }
  }
  piVar1 = (i16 *)(iVar4 + 0xaa);
  *piVar1 = *piVar1 + 0x14;
  piVar1 = (i16 *)(iVar4 + 0xac);
  *piVar1 = *piVar1 + 0xa;
  (iVar4 + 0xb0) = (iVar4 + 0xac);
  piVar1 = (i16 *)(iVar4 + 0xac);
  *piVar1 = *piVar1 + 0x30;
  ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,hdc);
  return;
}


fn reg_class_1040_98c0(Uparam_1: i32,HINSTANCE16 param_2,WNDCLASS16 *in_wnd_class_3)
{
  let BVar1: bool;
  ATOM AVar2;
  let l_name: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u32;
  let puStack18: *mut u8;
  let uStack16: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let iStack6: i16;
  let uStack4: u16;
  
  iStack6 = param_1 + 0x4;
  BVar1 = GetClassInfo16(param_2,(SEGPTR)&l_name,in_wnd_class_3);
  if (BVar1 == 0x0) {
    l_name = (param_1 + 0x54);
    uStack26 = 0x9cde;
    uStack24 = SUB42(&PTR_LOOP_1050_1040,0x0);
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = 0x0;
    uStack14 = (param_1 + 0x58);
    uStack12 = (param_1 + 0x56);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
make_proc_inst_1040_a234
          (uchar *param_1,uchar *param_2,param_3: u16,param_4: u32,LPVOID param_5)

{
  LPVOID pvVar1;
  let in_DX: u16;
  
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),
                  (param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0xa4e8;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  if (_PTR_LOOP_1050_5edc == 0x0) {
    pvVar1 = MakeProcInstance16(param_5,(HANDLE16)PTR_LOOP_1050_038c);
    _PTR_LOOP_1050_5edc = CONCAT22(in_DX,pvVar1);
  }
  *(long *)(param_1 + 0xc) = _PTR_LOOP_1050_5edc;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 0x1;
  PTR_LOOP_1050_5ee0 = param_1;
  PTR_LOOP_1050_5ee2 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn free_proc_inst_1040_a294(astruct_18 *param_1,param_2: u16)
{
  param_1->field_0x0 = 0xa4e8;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -0x1;
  if (PTR_LOOP_1050_5eda == 0x0) {
    FreeProcInstance16((LPVOID)param_2);
    _PTR_LOOP_1050_5edc = 0x0;
  }
  unk_draw_op_1040_b0f8(param_1);
  return;
}


u32 
call_win_proc_1040_a40e
          (HWND16 param_1,param_2: u32,LPARAM param_3,param_4: u16,LPVOID param_5,
          param_6: u16)

{
  let uVar1: u16;
  code **ppcVar2;
  let puVar4: u32;
  WPARAM16 wparam;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  let uVar7: u32;
  let uStack6: u32;
  let puVar3: u32;
  let uVar5: u32;
  
  uStack6 = 0x0;
  wparam = (WPARAM16)(param_2 >> 0x10);
  if (param_3 == 0x19) {
    puVar4 = &PTR_LOOP_1050_5ee0;
    ppcVar2 = (code **)(*puVar4 + 0x34);
    uStack6 = (**ppcVar2)(param_5,(char)puVar4,(puVar4 >> 0x10),param_1,
                          param_2,&USHORT_1050_1050);
    param_4 = (uStack6 >> 0x10);
  }
  else {
    if (param_3 == 0x86) {
      puVar4 = &PTR_LOOP_1050_5ee0;
      ppcVar2 = (code **)(*puVar4 + 0x20);
      uVar7 = (**ppcVar2)(param_5,puVar4,(puVar4 >> 0x10),wparam);
      return uVar7;
    }
    if (param_3 == 0x110) {
      uVar7 = win_msg_1040_a308(&PTR_LOOP_1050_5ee0,unaff_DI,(HWND16)param_5,
                                param_6);
      return uVar7;
    }
  }
  if (uStack6 != 0x0) {
    return uStack6 & 0xffff | param_4 << 0x10;
  }
  uVar5 = &PTR_LOOP_1050_5bc8;
  uVar6 = (uVar5 >> 0x10);
  iVar5 = uVar5;
  uVar1 = (iVar5 + 0x6);
  if ((uVar1 | (iVar5 + 0x4)) == 0x0) {
    return uVar1 << 0x10;
  }
  uVar7 = CallWindowProc16(param_5,param_1,param_2,wparam,param_3);
  return uVar7;
}

