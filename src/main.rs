mod pass_1000;
mod mem_1000;
mod outpost_src;
mod outpost_header;
mod util;

fn main() {}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 * entry(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
           CONTEXT *in_task_context,param_7: u16,param_8: i16)

{
  let puVar1: *mut u16;
  let uVar2: u16;
  char *pcVar3;
  code *pcVar4;
  let uVar8: u8;
  let uVar5: u16;
  let uVar6: u16;
  LPCSTR str;
  let puVar7: *mut u16;
  let uVar9: u16;
  let puVar10: *mut u16;
  char *pcVar11;
  CHAR *unaff_SS;
  let bVar12: bool;
  let DVar13: u32;
  let uVar14: u32;
  let uVar15: u32;
  let iVar16: i16;
  let iVar17: i16;
  let puVar18: *mut u8;
  let uVar19: u16;
  
  uVar14 = CONCAT22(param_7,PTR_LOOP_1050_5f84);
  do {
    uVar19 = 0x0;
    InitTask16(in_task_context);
    PTR_LOOP_1050_5f84 = uVar14;
    if ((param_8 != 0x0) &&
       (bVar12 = param_1 < 0xff00, param_1 += 0x100,
       PTR_LOOP_1050_5f7e = param_5, bVar12)) {
      PTR_LOOP_1050_5f48 = param_1;
      PTR_LOOP_1050_5f4a = param_3;
      PTR_LOOP_1050_5f4c = param_4;
      PTR_LOOP_1050_5f4e = param_2;
      PTR_LOOP_1050_5f50 = param_5;
      LockSegment16((HGLOBAL16)s_tile2_bmp_1050_1538);
      PTR_LOOP_1050_5f52 = (uVar14 >> 0x10);
      PTR_LOOP_1050_5f84 = uVar14;
      DVar13 = GetVersion16();
      PTR_LOOP_1050_5f52 = (uVar14 >> 0x10);
      PTR_LOOP_1050_5f84 = uVar14;
      uVar9 = (DVar13 >> 0x10);
      uVar8 = (u8)(DVar13 >> 0x8);
      PTR_LOOP_1050_5f80 = CONCAT11((char)DVar13,uVar8);
      uVar5 = CONCAT11(0x30,uVar8);
      if (true) {
        pcVar4 = (code *)swi(0x21);
        uVar15 = uVar14;
        uVar14 = (*pcVar4)(uVar19);
        PTR_LOOP_1050_5f52 = (uVar15 >> 0x10);
        PTR_LOOP_1050_5f84 = uVar15;
      }
      else {
        DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
        PTR_LOOP_1050_5f52 = (uVar14 >> 0x10);
        PTR_LOOP_1050_5f84 = uVar14;
        uVar14 = CONCAT22(uVar9,uVar5);
      }
      _DAT_1050_5f82 = CONCAT11((char)uVar14,(char)(uVar14 >> 0x8));
      if (true) {
        DAT_1050_5f87 = 0x0;
      }
      WaitEvent16(0x1000);
      PTR_LOOP_1050_5f84 = uVar14;
      puVar18 = PTR_LOOP_1050_5f4c;
      param_8 = InitApp16((HINSTANCE16)s_tile2_bmp_1050_1538);
      PTR_LOOP_1050_5f84 = uVar14;
      if (param_8 != 0x0) break;
    }
    in_task_context = (CONTEXT *)s_tile2_bmp_1050_1538;
    param_8 = CONCAT11((char)(param_8 >> 0x8),0xff);
    pass1_1000_24db(param_8,0x0);
    PTR_LOOP_1050_5f84 = uVar14;
  } while( true );
  dos3_call_1000_23ea(param_2,param_5,0x0,unaff_SS);
  PTR_LOOP_1050_5f84 = uVar14;
  pass1_1000_262c(0x238d,s_tile2_bmp_1050_1538,unaff_SS,
                  s_tile2_bmp_1050_1538);
  PTR_LOOP_1050_5f84 = uVar14;
  pass1_1000_27d6((uVar14 >> 0x10));
  uVar14 = ret_op_1000_55ac(puVar18);
  uVar6 = uVar14;
  init_1000_23be(param_1,(uVar14 >> 0x10),unaff_SS);
  fn_ptr_op_1000_24cd(uVar6,0x0);
  iVar17 = 0x15;
  iVar16 = 0x15;
  pass1_1000_25a8(param_5,s_tile2_bmp_1050_1538);
  pass1_1000_2913(iVar16,param_5,(u16_t)s_tile2_bmp_1050_1538);
  str = poss_str_op_1000_28dc(iVar17);
  if (str != (*mut u8)0x0) {
    iVar16 = 0x9;
    if (*str == 'M') {
      iVar16 = 0xf;
    }
    str = str + iVar16;
    iVar16 = 0x22;
    pcVar11 = str;
    do {
      if (iVar16 == 0x0) break;
      iVar16 += -0x1;
      pcVar3 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar11[-0x1] = '\0';
  }
  FatalAppExit16(s_tile2_bmp_1050_1538,str);
  FatalExit();
  puVar10 = (u16 *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar10;
    puVar10 = puVar10 + 0x1;
    uVar2 = *puVar1;
    puVar7 = puVar10;
    if ((uVar2 == uVar6) || (puVar7 = (u16 *)(uVar2 + 0x1), puVar7 == (u16 *)0x0)) {
      return (i16 *)puVar7;
    }
    iVar16 = -0x1;
    do {
      if (iVar16 == 0x0) break;
      iVar16 += -0x1;
      puVar1 = puVar10;
      puVar10 = (u16 *)(puVar10 + 0x1);
    } while (*puVar1 != '\0');
  } while( true );
}

fn ___EXPORTEDSTUB(void) -> u16

{
  return 0x0;
}


fn init_op_1008_54aa(uchar *param_1,char *param_2,uchar *param_3,uchar *param_4,
                 param_5: u16,param_6: u16,param_7: u16,param_8: u16)

{
  code **ppcVar1;
  let uVar3: u16;
  let in_CX: u16;
  let in_DX: u16;
  let puVar4: *mut u8
  let extraout_DX: u16;
  let uVar5: u16;
  let extraout_DX_00: u16;
  let uVar6: u16;
  let extraout_DX_01: u16;
  let uVar7: u32;
  let puStack12: u32;
  let uVar2: u32;
  
  if (param_3 != (uchar *)0x0) {
    return;
  }
  dos3_call_op_1000_435c((u16 *)0x0,in_CX,in_DX,&stack0xfffe,param_8);
  pass1_1000_4d0c(param_5);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(0x0,0x32,0x0,0x12,0x1000,in_DX);
  _PTR_LOOP_1050_029c =
       mem_op_1000_1902(0x0,0x64,0x0,0xc,0x1000,(_PTR_LOOP_1050_03a0 >> 0x10));
  _PTR_LOOP_1050_4fb8 =
       mem_op_1000_1902(0x0,0x64,0x0,0x10,0x1000,(_PTR_LOOP_1050_029c >> 0x10));
  _PTR_LOOP_1050_68a2 =
       mem_op_1000_1902(0x0,0x64,0x0,0xe,0x1000,(_PTR_LOOP_1050_4fb8 >> 0x10));
  _PTR_LOOP_1050_5744 =
       mem_op_1000_1902(0x0,0x1f4,0x0,0x42,0x1000,(_PTR_LOOP_1050_68a2 >> 0x10));
  uVar7 = mem_op_1000_1902(0x0,0x32,0x0,0x6,0x1000,(_PTR_LOOP_1050_5744 >> 0x10));
  puVar4 = (uchar *)(uVar7 >> 0x10);
  PTR_LOOP_1050_5768 = uVar7;
  PTR_LOOP_1050_038c = param_4;
  PTR_LOOP_1050_038e = param_3;
  PTR_LOOP_1050_0390 = param_1;
  PTR_LOOP_1050_576a = puVar4;
  uVar3 = str_op_1008_60e8(param_2,puVar4);
  _PTR_LOOP_1050_0392 = CONCAT22(puVar4,uVar3);
  mem_op_1000_179c(0xc,puVar4,0x1000);
  if ((puVar4 | uVar3) == 0x0) {
    uVar3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    struct_op_1008_0000((u16 *)
                        CONCAT13((char)(puVar4 >> 0x8),CONCAT12((char)puVar4,uVar3))
                       );
    uVar5 = extraout_DX;
  }
  puStack12 = CONCAT22(uVar5,uVar3);
  if (_PTR_LOOP_1050_0392 != 0x0) {
    ppcVar1 = (code **)(*puStack12 + 0x4);
    (**ppcVar1)(0x1000,(char)uVar3,uVar5,_PTR_LOOP_1050_0392,
                (char)(_PTR_LOOP_1050_0392 >> 0x10));
  }
  uVar2 = *puStack12;
  ppcVar1 = (code **)uVar2 + 0x4;
  (**ppcVar1)(0x1000,uVar3,(char)uVar5);
  uVar6 = extraout_DX_00;
  win_msg_op_1008_9498((MSG *)&PTR_LOOP_1050_1000,(MSG16 *)param_8);
  if (puStack12 != 0x0) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)(0x1000,uVar3,(char)uVar5,0x1);
    uVar6 = extraout_DX_01;
  }
  uVar7 = mem_op_1000_1b68(uVar6,0x1000,_PTR_LOOP_1050_03a0,
                           (_PTR_LOOP_1050_03a0 >> 0x10));
  uVar7 = mem_op_1000_1b68((uVar7 >> 0x10),0x1000,_PTR_LOOP_1050_029c,
                           (_PTR_LOOP_1050_029c >> 0x10));
  uVar7 = mem_op_1000_1b68((uVar7 >> 0x10),0x1000,_PTR_LOOP_1050_4fb8,
                           (_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar7 = mem_op_1000_1b68((uVar7 >> 0x10),0x1000,_PTR_LOOP_1050_68a2,
                           (_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((uVar7 >> 0x10),0x1000,_PTR_LOOP_1050_5744,
                   (_PTR_LOOP_1050_5744 >> 0x10));
  return;
}


void 
def_win_proc_1008_5632
          (param_1: *mut u32,WPARAM16 param_2,param_3: u16,param_4: i16,param_5: u16)

{
  code **ppcVar1;
  HWND16 unaff_CS;
  let unaff_SS: u16;
  let uVar2: u16;
  let puStack6: u32;
  
  uVar2 = SUB42(&USHORT_1050_1050,0x0);
  puStack6 = GetWindowLong16(unaff_CS,0x0);
  if (((puStack6 >> 0x10) | puStack6) == 0x0) {
    if (param_4 != 0x1) {
      DefWindowProc16((HWND16)s_tile2_bmp_1050_1538,param_1,param_2,
                      CONCAT22(param_4,param_3));
      return;
    }
    puStack6 = *param_1;
    SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,(INT16)puStack6,puStack6 >> 0x10)
    ;
    pass1_1008_9628(puStack6,param_5);
  }
  ppcVar1 = (code **)(*puStack6 + 0x1c);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack6,(puStack6 >> 0x10),
              param_1,param_2,param_3,param_4,uVar2);
  return;
}
