
pub fn
draw_op_1038_92f6(param_1: u16,param_2: u16,param_3: u16,param_4: u32,
                 param_5: HWND16,param_6: u16)

{
  let uVar1: u32;
  let ppcVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  let paVar5: &mut Struct18;
  let in_DX: *mut u8;
  let puVar6: *mut u8;
  let puVar7: *mut u8;
  let unaff_DI: i16;
  let uVar8: u16;
  bool local_1a [0x2];
  let UStack22: u16;
  let paStack20: &mut Struct18;
  let paStack16: &mut Struct18;
  let iStack12: i16;
  let paStack10: &mut Struct18;
  let paStack6: &mut Struct20;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = 
               mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
   // puVar6 = (paStack6 >> 0x10);
    paVar5 = (param_1 + 0x90);
    if (paVar5 != 0x0) {
      paStack10 = paVar5;
      mem_op_1000_179c(0x18,puVar6,0x1000);
      uVar3 = paVar5;
      paStack16 = (paVar5 & 0xffff | ZEXT24(puVar6) << 0x10);
      puVar7 = (puVar6 | uVar3);
      if (puVar7 == 0x0) {
        uVar3 = 0x0;
        puVar7 = 0x0;
      }
      else {
        struct_1040_a598((paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
      }
      (param_1 + 0x90) = uVar3;
      (param_1 + 0x92) = puVar7;
      (param_1 + 0x90) = 0x11;
      iStack12 = *(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      paStack16 = CONCAT22(puVar7,uVar3);
      if ((puVar7 | uVar3) == 0x0) {
        uVar1 = (param_1 + 0x90);
        (uVar1 + 0x2) = 0x0;
      }
      else {
        paStack16 = iStack12;
        pass1_1000_5586(0xa564,&ctx.PTR_LOOP_1050_1040,iStack12,0xa,
                        uVar3 + 0x2,puVar7);
        uVar1 = (param_1 + 0x90);
       // uVar8 = (uVar1 >> 0x10);
        iVar4 = uVar1;
        (iVar4 + 0x2) = uVar3 + 0x2;
        (iVar4 + 0x4) = puVar7;
      }
     // uVar8 = (paStack10 >> 0x10);
      uVar1 = (param_1 + 0x90);
      (uVar1 + 0x6) = (paStack10 + 0x6);
      uVar1 = (param_1 + 0x90);
      (uVar1 + 0xa) = (paStack10 + 0xa);
      uVar1 = (param_1 + 0x90);
      (uVar1 + 0x12) = (param_1 + 0xa);
      uVar8 = 0x1010;
      pass1_1010_a50c(paStack6,0x10505b42,(param_1 + 0x90));
      paStack20 = paStack10;
      paStack16 = paStack10;
      if (paStack10 != 0x0) {
        pass1_1040_a5d0(paStack10);
        uVar8 = 0x1000;
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar2 = (CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar2)(uVar8,param_1,param_2);
    }
  }
  else {
    if (param_4._2_2_ != 0xf9) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,&ctx.PTR_LOOP_1050_1040,
                      param_6);
      return;
    }
    iVar4 = pass1_1038_993a(param_1,param_2,param_3);
    if (-0x1 < iVar4) {
      uVar8 = (param_1 + 0x6);
      UStack22 = GetDlgItemInt16(param_5,0x1,local_1a,param_6);
      if (local_1a[0] != 0x0) {
        uVar1 = (param_1 + 0x98);
        draw_fn_1010_2a32(0x94be,s_tile2_bmp_1050_1538,uVar1,
                          (uVar1 >> 0x10),UStack22,
                          CONCAT22(uVar8,(iVar4 * 0xe + 0x5a72)),
                          in_DX,param_1,&stack0xfffe,param_2);
      }
    }
  }
  return;
}


pub fn
draw_op_1038_9dcc(in_struct_1: &mut Struct10,param_2: i16,param_3: u16,COLORREF in_colorref_4,
                 param_5: u16)

{
  let puVar1: *mut u16;
  let bVar2: bool;
  let uVar3: u16;
  let iVar4: i16;
  let local_brush_handle: HBRUSH16;
  let uVar5: i32;
  let extraout_DX: u16;
  let local_struct_5: &mut Struct10;
  let var5: &mut Struct10;
  let hdc: COLORREF;
  let uVar6: u32;
  let uStack14: u16;
  
 // var5 = (in_struct_1 >> 0x10);
  local_struct_5 = in_struct_1;
  hdc = in_colorref_4;
  if (local_struct_5.brush_handle_field_0x8e == 0x0) {
    hdc = s_tile2_bmp_1050_1538;
    local_brush_handle = CreateSolidBrush16(in_colorref_4);
    local_struct_5.brush_handle_field_0x8e = local_brush_handle;
  }
  if (ctx.PTR__LOOP_1050_5b64 == 0x0) {
    hdc = 0x1008;
    uVar6 = pass1_1008_4d72((ctx.PTR__LOOP_1050_4230 + 0xe));
   // uVar3 = (uVar6 >> 0x10);
    iVar4 = uVar6;
    ctx._PTR_LOOP_1050_5b64 =
         CONCAT12(*(iVar4 + 0x94),
                         CONCAT11(*(iVar4 + 0x95),
                                  *(iVar4 + 0x96)));
    ctx.PTR_LOOP_1050_5b68 =
         
         CONCAT11(*(iVar4 + 0x3e5),*(iVar4 + 0x3e6));
    ctx.PTR_LOOP_1050_5b6a = (iVar4 + 0x3e4);
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar2 = false;
    // TODO: refactor for loop
    // for (uStack14 = 0x0; puVar1 = &local_struct_5.field_0x128,
    //     uStack14 <= *puVar1 && *puVar1 != uStack14; uStack14 += 0x1) {
    //   if ((&local_struct_5.field_0x94 + uStack14 * 0x2) == param_2) {
    //     bVar2 = true;
    //     break;
    //   }
    // }
    if (bVar2) {
      ctx.PTR_LOOP_1050_5b64 = ctx.PTR_LOOP_1050_5b68;
    }
  }
  SetTextColor16(hdc,PTR_LOOP_1050_5b64);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  return;
}


