
fn str_op_1008_60e8(char *param_1,param_2: u16) -> u16

{
  let uVar1: u16;
  
  if (param_1 != 0x0) {
    uVar1 = str_op_1000_3da4(param_1);
    uVar1 += 0x1;
    mem_op_1000_179c(uVar1,(uchar *)param_2,0x1000);
    if ((param_2 | uVar1) != 0x0) {
      unk_str_op_1000_3d3e(CONCAT22(param_2,uVar1),param_1);
      return uVar1;
    }
  }
  return 0x0;
}


fn str_1008_6d8a(param_1: *mut u32,char *param_2,param_3: u16,param_4: u16,param_5: u8) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x0;
  (param_1 + 0x4) = 0xffff;
  PTR_LOOP_1050_0312 = &DAT_1050_0004;
  sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,
                _PTR_s_SC_03d_1050_0314_1050_031c,
                (_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),0x4,
                &stack0xfffe,uVar2,0x1000,param_4,param_5);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  param_1 = uVar1;
  (param_1 + 0x2) = param_3;
  return param_1;
}


fn load_string_1008_b1f0(void) -> *mut u8

{
  char *pcVar1;
  
  pcVar1 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}


fn load_string_1008_b65a(Uparam_1: i32,LPSTR in_string_2,Uparam_3: i32,param_4: u16)
{
  let unaff_SS: u16;
  
  pass1_1008_b9ce(param_1,CONCAT22(param_4,param_3._2_2_),unaff_SS);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,in_string_2,(short)param_3
            );
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn load_str_and_sprintf_1008_b69c(astruct_25 *param_1,param_2: *mut u16,uchar *param_3)
{
  code **ppcVar1;
  char *in_buffer_4;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  astruct_25 *iVar5;
  let uVar5: u16;
  astruct_26 *paVar6;
  let uVar7: u32;
  let iStack516: i16;
  char local_202 [0x100];
  CHAR local_102 [0x100];
  
  in_buffer_4 = local_202;
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,in_buffer_4,(short)param_2
            );
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_25 *)param_1;
  if (iVar5->field_0xa == 0x0) {
    mem_op_1000_179c(0xc,param_3,0x1000);
    if ((param_3 | in_buffer_4) == 0x0) {
      paVar6 = (astruct_26 *)0x0;
    }
    else {
      paVar6 = (astruct_26 *)
               set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,in_buffer_4));
    }
    &iVar5->field_0xa = paVar6;
    (&iVar5->field_0xa + 0x2) = (paVar6 >> 0x10);
    for (iStack516 = 0x1; iStack516 < 0x6; iStack516 += 0x1) {
      mem_op_1000_179c(0x12,(uchar *)(paVar6 >> 0x10),0x1000);
      if (paVar6 == (astruct_26 *)0x0) {
        uVar7 = 0x0;
      }
      else {
        uVar7 = set_stuct_1008_b0bc(paVar6);
      }
      uVar3 = (uVar7 >> 0x10);
      uVar4 = uVar3;
      wsprintf16((LPSTR)&PTR_LOOP_1050_1000,local_102,param_2);
      uVar2 = str_op_1008_60e8(CONCAT22(param_2,local_102),uVar4);
      (uVar7 + 0x4) = uVar2;
      (uVar7 + 0x6) = uVar4;
      ppcVar1 = (code **)(*iVar5->field_0xa + 0x8);
      paVar6 = (astruct_26 *)(**ppcVar1)();
    }
    iVar5->field_0x22 = 0x5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn load_str_and_sprintf_1008_b78a(Uparam_1: i32,param_2: *mut u16,uchar *param_3,param_4: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  char local_206 [0x100];
  CHAR local_106 [0x100];
  let iStack6: i16;
  let uStack4: u16;
  
  mem_op_1000_179c(0x12,param_3,0x1000);
  if ((param_3 | param_4) == 0x0) {
    uVar6 = 0x0;
  }
  else {
    uVar6 = set_stuct_1008_b0bc((astruct_26 *)CONCAT22(param_3,param_4));
  }
  uStack4 = (uVar6 >> 0x10);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,(short)param_2);
  iStack6 = uVar6;
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  piVar1 = (i16 *)(iVar4 + 0x22);
  *piVar1 = *piVar1 + 0x1;
  wsprintf16((LPSTR)0x1010,local_106,param_2);
  iStack6 = uVar6;
  uVar3 = str_op_1008_60e8(CONCAT22(param_2,local_106),
                           (uVar6 >> 0x10));
  iStack6 = uVar6;
  (iStack6 + 0x4) = uVar3;
  (iStack6 + 0x6) = (uVar6 >> 0x10);
  ppcVar2 = (code **)((iVar4 + 0xa) + 0x8);
  (**ppcVar2)(s_tile2_bmp_1050_1538,(iVar4 + 0xa),iStack6,uStack4);
  return;
}


fn unk_str_op_1008_d1c6(param_1: u32,param_2: u32)
{
  let iVar1: i16;
  let uVar2: u32;
  code **ppcVar3;
  let bVar4: bool;
  let puVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u8;
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let puVar11: *mut u8
  let extraout_DX_01: *mut u8
  let uVar12: u16;
  let puVar13: *mut u8
  let extraout_DX_02: *mut u8
  let puVar14: *mut u8
  let uVar15: u16;
  let iVar16: i16;
  let valist: *mut u16;
  let uVar17: u16;
  let puVar18: u32;
  let uVar19: u32;
  let uStack52: u16;
  let uStack20: u32;
  let uStack14: u32;
  let puStack10: u32;
  
  valist = (u16 *)(param_1 >> 0x10);
  iVar16 = param_1;
  puVar5 = (iVar16 + 0x12);
  puVar13 = *(uchar **)(iVar16 + 0x14);
  if ((puVar13 | puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar13 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar13,0x1000);
  if ((puVar13 | puVar5) == 0x0) {
    puVar5 = 0x0;
    uVar17 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar13,puVar5));
    uVar17 = extraout_DX_00;
  }
  (iVar16 + 0x12) = puVar5;
  (iVar16 + 0x14) = uVar17;
  puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
  puVar11 = (uchar *)(puVar18 >> 0x10);
  uVar6 = puVar18;
  uVar17 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4e78(uVar6,puVar11,param_2,puVar18);
  puStack10 = CONCAT22(puVar11,uVar6);
  ppcVar3 = (code **)(*puStack10 + 0x10);
  uVar9 = uVar6;
  (**ppcVar3)(&PTR_LOOP_1050_1038,uVar6,puVar11);
  uStack14 = CONCAT22(extraout_DX_01,uVar9);
  bVar4 = false;
  puVar13 = extraout_DX_01;
  for (uStack20 = 0x0; uStack20 < uStack14; uStack20 += 0x1) {
    uVar17 = 0x1030;
    uVar19 = pass1_1030_1d7c(uVar9,puVar13,puStack10);
    uVar12 = (uVar19 >> 0x10);
    uVar15 = uVar19;
    puVar13 = (uchar *)(uVar12 | uVar15);
    if (((puVar13 != (uchar *)0x0) && (*(long *)(uVar15 + 0x1c) != 0x8000002)) &&
       ((iVar1 = (uVar15 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      puVar13 = (uchar *)((uVar15 + 0x6) & 0xff);
      pass1_1020_bd80((uVar15 + 0xc));
      wsprintf16((LPSTR)0x1020,(LPCSTR)(iVar16 + 0x22),valist);
      uVar7 = str_op_1008_60e8((param_1 & 0xffff0000 | (iVar16 + 0x22)),
                               puVar13);
      uVar17 = 0x1000;
      puVar14 = puVar13;
      uVar8 = uVar7;
      mem_op_1000_179c(0x12,puVar13,0x1000);
      uStack52 = puVar14 | uVar8;
      if (uStack52 == 0x0) {
        uVar8 = 0x0;
        uStack52 = 0x0;
      }
      else {
        uVar17 = 0x1018;
        pass1_1018_4808((u16 *)CONCAT22(puVar14,uVar8),0x1,
                        CONCAT13((char)(puVar13 >> 0x8),
                                 CONCAT12((char)puVar13,uVar7)),(uVar15 + 0x4));
      }
      uVar2 = (iVar16 + 0x12);
      ppcVar3 = (code **)((iVar16 + 0x12) + 0x4);
      (**ppcVar3)(uVar17,uVar2,(char)(uVar2 >> 0x10),uVar8,uStack52);
      bVar4 = true;
      puVar13 = extraout_DX_02;
    }
  }
  if (!bVar4) {
    load_string_1010_84ac
              (_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar9 = uStack14;
    uVar17 = 0x1000;
    puVar14 = puVar13;
    mem_op_1000_179c(0x12,puVar13,0x1000);
    uVar15 = puVar14 | uVar9;
    if (uVar15 == 0x0) {
      uVar9 = 0x0;
      uVar10 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      pass1_1018_4808((u16 *)CONCAT22(puVar14,uVar9),0x0,
                      uStack14 & 0xffff | ZEXT24(puVar13) << 0x10,0x0);
      uVar10 = (u8)uVar15;
    }
    uVar2 = (iVar16 + 0x12);
    ppcVar3 = (code **)((iVar16 + 0x12) + 0x4);
    (**ppcVar3)(uVar17,(char)uVar2,(uVar2 >> 0x10),uVar9,uVar10);
  }
  if ((puVar11 | uVar6) != 0x0) {
    ppcVar3 = (code **)*puStack10;
    (**ppcVar3)(uVar17,uVar6,(char)puVar11,0x1);
  }
  return;
}


fn unk_str_op_1008_d4f6(param_1: u32,param_2: u32)
{
  let lVar1: i32;
  let puVar2: u32;
  let uVar3: u32;
  code **ppcVar4;
  let bVar5: bool;
  let puVar6: u32;
  let BVar7: bool;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let puVar11: u32;
  let uVar12: u32;
  let uVar13: u8;
  let extraout_DX: *mut u8
  let puVar14: *mut u8
  let extraout_DX_00: u16;
  let uVar15: u16;
  let extraout_DX_01: *mut u8
  let pWVar16: *mut u16;
  let pWVar17: *mut u16;
  let puVar18: *mut u8
  let uVar19: u16;
  let iVar20: i16;
  let iVar21: i16;
  let uVar22: u16;
  let valist: *mut u16;
  let uVar23: u32;
  let uStack58: u16;
  let uStack20: u32;
  
  uVar22 = (param_2 >> 0x10);
  iVar20 = param_2;
  lVar1 = *(long *)(iVar20 + 0x200);
  valist = (u16 *)(param_1 >> 0x10);
  iVar21 = param_1;
  puVar6 = (iVar21 + 0xe);
  puVar14 = *(uchar **)(iVar21 + 0x10);
  if ((puVar14 | puVar6) != 0x0) {
    ppcVar4 = (code **)*puVar6;
    (**ppcVar4)();
    puVar14 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar14,0x1000);
  if ((puVar14 | puVar6) == 0x0) {
    puVar6 = 0x0;
    uVar15 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar14,puVar6));
    uVar15 = extraout_DX_00;
  }
  (iVar21 + 0xe) = puVar6;
  (iVar21 + 0x10) = uVar15;
  puVar2 = (iVar20 + 0xc);
  ppcVar4 = (code **)(*puVar2 + 0x10);
  puVar11 = puVar2;
  (**ppcVar4)(0x1000,puVar2,(iVar20 + 0xe));
  uVar12 = puVar11 & 0xffff | ZEXT24(extraout_DX_01) << 0x10;
  bVar5 = false;
  for (uStack20 = 0x0; uStack20 < uVar12; uStack20 += 0x1) {
    uVar23 = pass1_1030_1d7c((puVar11 & 0xffff),extraout_DX_01,puVar2);
    uVar19 = (uVar23 >> 0x10);
    uVar10 = uVar23;
    if ((((uVar19 | uVar10) != 0x0) && (*(long *)(uVar10 + 0x1c) != 0x8000002)) &&
       ((iVar20 = (uVar10 + 0x12), iVar20 == 0x5 || (iVar20 == 0x6)))) {
      uVar9 = (uVar10 + 0xc);
      BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar9,0x34);
      if ((BVar7 == 0x0) && (*(long *)(uVar10 + 0x1c) != lVar1)) {
        pass1_1020_bd80(uVar9);
        pWVar16 = valist;
        wsprintf16((LPSTR)0x1020,(LPCSTR)(iVar21 + 0x22),valist);
        uVar8 = str_op_1008_60e8((param_1 & 0xffff0000 |
                                         ZEXT24((LPCSTR)(iVar21 + 0x22))),pWVar16)
        ;
        uVar22 = 0x1000;
        pWVar17 = pWVar16;
        uVar9 = uVar8;
        mem_op_1000_179c(0x14,(uchar *)pWVar16,0x1000);
        uStack58 = pWVar17 | uVar9;
        if (uStack58 == 0x0) {
          uVar9 = 0x0;
          uStack58 = 0x0;
        }
        else {
          uVar22 = 0x1018;
          struct_1018_47c8((u16 *)CONCAT22(pWVar17,uVar9),0x1,CONCAT22(pWVar16,uVar8),
                           (uVar10 + 0xc),(uVar10 + 0x4));
        }
        uVar3 = (iVar21 + 0xe);
        ppcVar4 = (code **)((iVar21 + 0xe) + 0x4);
        (**ppcVar4)(uVar22,uVar3,(char)(uVar3 >> 0x10),uVar9,uStack58);
        bVar5 = true;
      }
    }
  }
  if (!bVar5) {
    puVar14 = extraout_DX_01;
    load_string_1010_84ac
              (_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar22 = 0x1000;
    puVar18 = puVar14;
    uVar10 = uVar12;
    mem_op_1000_179c(0x14,puVar14,0x1000);
    uVar19 = puVar18 | uVar10;
    if (uVar19 == 0x0) {
      uVar10 = 0x0;
      uVar13 = 0x0;
    }
    else {
      uVar22 = 0x1018;
      struct_1018_47c8((u16 *)CONCAT22(puVar18,uVar10),0x0,
                       CONCAT13((char)(puVar14 >> 0x8),
                                CONCAT12((char)puVar14,uVar12)),0x0,0x0);
      uVar13 = (u8)uVar19;
    }
    uVar3 = (iVar21 + 0xe);
    ppcVar4 = (code **)((iVar21 + 0xe) + 0x4);
    (**ppcVar4)(uVar22,(char)uVar3,(uVar3 >> 0x10),uVar10,uVar13);
  }
  return;
}


fn string_1008_e586(param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: u16) -> u32

{
  let uVar1: u16;
  let puVar2: *mut u8
  char *in_string_2;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3,(param_3 >> 0x10));
  puVar2 = (uchar *)(param_5 | param_4);
  if (puVar2 == (uchar *)0x0) {
    return 0x0;
  }
  uVar1 = param_4;
  mem_op_1000_179c(0x80,puVar2,0x1000);
  in_string_2 = pass1_1038_4d28(CONCAT22(param_5,param_4));
  unk_str_op_1000_3d3e(CONCAT22(puVar2,uVar1),in_string_2);
  return CONCAT22(puVar2,uVar1);
}


fn load_string_1008_ee56(void) -> *mut u8

{
  char *pcVar1;
  
  pcVar1 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}


fn pass1_1008_ee72(param_1: u16,param_2: u16,param_3: i16)
{
  code **ppcVar1;
  let uVar2: u32;
  
  if (*(long *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)(CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



fn pass1_1008_eea6(void) -> u16

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool 
pass1_1008_eeac(param_1: u16,param_2: u16,param_3: u32,uchar *param_4,param_5: i16,
               param_6: u16)

{
  let uVar1: u16;
  let cVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let uVar7: u16;
  
  uVar7 = (param_3 + 0x12);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_4,param_5);
  uVar4 = (puVar6 >> 0x10);
  uVar1 = puVar6;
  uVar5 = uVar4;
  if (uVar7 == 0x7d) {
    pass1_1010_a5ca(uVar1,uVar4,0x7c,0x7d,uVar4);
    if (uVar7 != 0x0) {
      return false;
    }
    pass1_1010_a5ca(uVar1,uVar4,0x7d,0x0,uVar5);
    if (uVar7 != 0x0) {
      return false;
    }
    uVar3 = uVar7;
    uVar7 = 0x78;
  }
  else {
    uVar3 = uVar7;
    if (uVar7 < 0x7e) {
      cVar2 = (char)uVar7;
      uVar3 = uVar7 & 0xff00;
      if ((byte)(cVar2 + 0x8dU) == 0x0) {
        uVar7 = 0x9;
        uVar3 = uVar3 | (byte)(cVar2 + 0x8dU);
      }
      else {
        if ((byte)(cVar2 + 0x89U) == 0x0) {
          uVar7 = 0x2e;
          uVar3 = uVar3 | (byte)(cVar2 + 0x89U);
        }
        else {
          uVar3 |= (byte)(cVar2 + 0x87U);
          if ((byte)(cVar2 + 0x87U) == 0x0) {
            uVar7 = 0x5b;
          }
        }
      }
    }
  }
  pass1_1010_a5ca(uVar1,uVar4,uVar7,uVar3,uVar5);
  return uVar3 == 0x0;
}



fn pass1_1008_ef38(param_1: u32) -> u16

{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x16);
  return (uVar1 + 0x2);
}



fn pass1_1008_ef4a(void) -> u16

{
  return 0x41;
}



fn pass1_1008_ef50(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1008_ec94(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_18 *  pass1_1008_ef76(astruct_18 *param_1,param_2: u8)

{
  let unaff_SS: u16;
  
  pass1_1008_ed00((u16 *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}
