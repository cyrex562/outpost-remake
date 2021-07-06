use crate::{defines::{Struct18, Struct27, Struct65, StructB}, fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce, global::AppContext, pass::{pass_1008::pass1_1008_57c4, pass_1040::pass1_1040_c60e}, ui::ui_1008::set_sys_color_1008_357e, win_struct::{HICON16, HWND16}, winapi::{DeleteDC16, DeleteObject16, DestroyIcon16, DestroyMenu16, DestroyWindow16, GetWindowWord16, IsDlgButtonChecked, IsWindow16, PostMessage16, SelectPalette16, SendMessage16, ShowWindow16}};


pub fn cleanup_ui_op_1008_0618(
  ctx: &mut AppContext, 
  param_1: &mut StructB, 
  unaff_CS: u16, 
  unaff_SS: u16)
{
  let pu_var1: u32;
  let u_var2: u16;
  // astruct_18 *paVar3;
  let mut pa_var3: *mut Struct18;
  // code **ppcVar4;
  let mut ppc_var4: u32;
  let i_var5: i16;
  let u_var6: u16;
  // let unaff_CS: u16;
  // HICON16 h_icon;
  let mut h_icon: HICON16;
  // let unaff_SS: u16;
  let u_var7: u16;
  let u_var8: u16;
  
  // u_var6 = (param_1 >> 0x10);
  // i_var5 = param_1;
  param_1.field_0x0 = 0x389e;
  param_1.field_0x2 = 0x1008;
  set_sys_color_1008_357e(param_1,0x0,unaff_CS,unaff_SS);
  pa_var3 = &mut param_1.field_0xf8; // (i_var5 + 0xf8);
  // u_var8 = (pa_var3 >> 0x10);
  h_icon = 0x1000;
  fn_ptr_1000_17ce(pa_var3,0x1000);
  if ((i_var5 + 0xec) != 0x0) {
    u_var8 = (i_var5 + 0xec);
    h_icon = ctx.s_tile2_bmp_1050_1538;
    DestroyMenu16(0x1000);
  }
  u_var7 = (i_var5 + 0xc2);
  DestroyIcon16(h_icon);
  (i_var5 + 0xc2) = 0x0;
  pu_var1 = (i_var5 + 0xe0);
  u_var2 = (i_var5 + 0xe2);
  if ((u_var2 | pu_var1) != 0x0) {
    ppc_var4 = *pu_var1;
    (**ppc_var4)(ctx.s_tile2_bmp_1050_1538,pu_var1,u_var2,0x1,u_var7,pa_var3);
  }
  pass1_1008_57c4((param_1 & 0xffff0000 | (i_var5 + 0xd2)));
  *param_1 = 0x380a;
  (i_var5 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (i_var5 + 0x2) = 0x1008;
  return;
}



pub fn destroy_win_1008_628e(Uparam_1: i32, param_2: HWND16)
{
  code **ppcVar1;
  
  ppcVar1 = ((param_1 + 0xd2) + 0x14);
  (**ppcVar1)(param_2,(param_1 + 0xd2),param_1._2_2_);
  DestroyWindow16(param_2);
  (param_1 + 0x8) = 0x0;
  return;
}


pub fn destroy_win_1008_9698(param_1: HWND16)
{
  DestroyWindow16(param_1);
  return;
}


pub unsafe fn unk_destroy_win_op_1010_2fa0(ctx: &mut AppContext, param_1: i32, param_2: HWND16)
{
  let pi_var1: *mut i16;
  let u_var2: u32;
  let i_var3: i16;
  let u_var4: u16;
  // HWND16 HVar5;
  let mut h_var5: HWND16;
  let i_stack4: i16;
  
  // u_var4 = (param_1 >> 0x10);
  i_var3 = param_1;
  (i_var3 + 0x28) = 0x0;
  i_stack4 = 0x0;
  loop {
    pi_var1 = (i_var3 + 0x16);
    if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) {break;}
    DestroyWindow16(param_2);
    i_stack4 += 0x1;
    param_2 = ctx.s_tile2_bmp_1050_1538;
  }
  (i_var3 + 0x16) = 0x0;
  if (((i_var3 + 0x54) | (i_var3 + 0x52)) != 0x0) {
    i_stack4 = 0x0;
    loop {
      uVar2 = (iVar3 + 0x52);
      HVar5 = param_2;
      if (*(long *)(uVar2 + iStack4 * 0x4) != 0x0) {
        HVar5 = s_tile2_bmp_1050_1538;
        DestroyWindow16(param_2);
        uVar2 = (iVar3 + 0x52);
        (uVar2 + iStack4 * 0x4) = 0x0;
      }
      iStack4 += 0x1;
      param_2 = HVar5;
      if i_stack4 >= 0xa {
        break;
      }
    } 
    fn_ptr_1000_17ce((i_var3 + 0x52),0x1000);
    (i_var3 + 0x52) = 0x0;
  }
  return;
}



pub fn unk_destroy_win_op_1010_305a(
  ctx: &mut AppContext,
  param_1: &mut Struct27,
  param_2: i16,
  param_3: &mut Struct65, 
  param_4: u16,
  unaff_SS: u16)

{
  let pi_var1: *mut i16;
  let u_var2: i32;
  let l_var3: i32;
  let b_var4: bool;
  let u_var5: u16;
  // Struct27 *iVar4;
  let mut iVar4: &mut Struct27;
  let i_var6: i16;
  // Struct27 *uVar7;
  let mut uVar7: &mut Struct27;
  let u_var8: u16;
  // HWND16 hwnd;
  let mut hwnd: HWND16;
  // HWND16 hwnd_00;
  let mut hwnd_00: HWND16;
  let unaff_SS: u16;
  let u_stack10: i16;
  let i_stack8: i16;
  let i_stack6: i16;
  
  hwnd = ctx.PTR_LOOP_1050_1040;
  u_var5 = pass1_1040_c60e(param_3);
  // uVar7 = (Struct27 *)(param_1 >> 0x10);
  iVar4 = param_1;
  iVar4.field_0x12 = u_var5;
  iVar4.field_0x14 = 0x0;
  i_stack6 = 0x0;
  b_var4 = false;
  iVar4.field_0x28 = 0x0;
  i_stack8 = 0x0;
  loop {
    piVar1 = &iVar4->field_0x16;
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
//LAB_1010_30ad:
      iVar6 = iStack6;
      if (bVar4) {
        while (iStack8 = iVar6 + 0x1, piVar1 = &iVar4->field_0x16,
              *piVar1 != iStack8 && iStack8 <= *piVar1) {
          DestroyWindow16(hwnd);
          (&iVar4->field_0x2e)[iVar6] = 0x0;
          hwnd = s_tile2_bmp_1050_1538;
          iVar6 = iStack8;
        }
        iVar4->field_0x16 = iStack6 + 0x1;
        pass1_1010_1f62(unaff_SS,param_1,0x9);
      }
      else {
        iVar6 = iVar4->field_0x16;
        (&iVar4->field_0x2a)[iVar6 * 0x2] = param_3;
        (&iVar4->field_0x2c)[iVar6 * 0x2] = (param_3 >> 0x10);
        iStack10 = 0xa;
        piVar1 = &iVar4->field_0x16;
        *piVar1 = *piVar1 + 0x1;
        if (0x1 < iVar4->field_0x16) {
          UVar2 = (&iVar4->field_0x22)[iVar4->field_0x16];
          iVar6 = UVar2;
          uVar8 = (UVar2 >> 0x10);
          iStack10 = (iVar6 + 0x20) + (iVar6 + 0x24) + 0x8;
        }
        hwnd = &ctx.PTR_LOOP_1050_1040;
        mov_update_win_1040_93aa
                  (param_3,iStack10,iVar4->field_0x1a,&ctx.PTR_LOOP_1050_1040);
      }
      if (!bVar4) {
        pass1_1010_1f62(unaff_SS,param_1,0xa);
      }
      if (param_2 == 0x0) {
        if (iVar4->field_0x52 != 0x0) {
          iStack8 = 0x0;
          hwnd_00 = hwnd;
          do {
            lVar3 = iVar4->field_0x52;
            uVar8 = (lVar3 >> 0x10);
            iVar6 = lVar3;
            hwnd = hwnd_00;
            if ((*(long *)(iVar6 + iStack8 * 0x4) != 0x0) &&
               (*(Struct65 **)(iVar6 + iStack8 * 0x4) != param_3)) {
              hwnd = s_tile2_bmp_1050_1538;
              DestroyWindow16(hwnd_00);
            }
            lVar3 = iVar4->field_0x52;
            (lVar3 + iStack8 * 0x4) = 0x0;
            iStack8 += 0x1;
            hwnd_00 = hwnd;
          } while (iStack8 < 0xa);
        }
        pass1_1010_32da(param_1,param_3,hwnd,unaff_SS);
        pass1_1010_1f62(unaff_SS,param_1,0x8);
      }
      return;
    }
    if (*(Struct65 **)(&iVar4->field_0x2a + iStack8 * 0x2) == param_3) {
      bVar4 = true;
      iStack6 = iStack8;
      goto LAB_1010_30ad;
    }
    iStack8 += 0x1;
  }
}


pub fn destroy_window_1010_7b26(param_1: u32,param_2: i32,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u16;
  let puVar2: *mut u8
  let extraout_DX: u16;
  let iVar2: i16;
  let uVar4: u16;
  UCHAR local_a [0x8];
  
  uVar4 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0x1e) | (iVar2 + 0x1c);
  if (uVar1 != 0x0) {
    pass1_1008_5784(CONCAT22(param_3,local_a),(iVar2 + 0x1c));
    do {
      puVar2 = local_a;
      pass1_1008_5b12(puVar2,param_3);
      param_4 = extraout_DX | puVar2;
      if (param_4 == 0x0) break;
    } while (*(long *)(puVar2 + 0x4) != param_2);
    uVar1 = extraout_DX | puVar2;
    if (uVar1 != 0x0) {
      uVar1 = DestroyWindow16(0x1008);
    }
  }
  return CONCAT22(uVar1,param_4);
}


pub fn clenaup_win_ui_1018_4d22(astruct_11 *in_struct_1,HDC16 in_hdc_2)
{
  let uVar1: u16;
  code **ppcVar2;
  astruct_11 *local_struct_1;
  astruct_11 *uVar4;
  let unaff_SS: u16;
  ULONG *puVar2;
  ULONG *puVar1;
  
  uVar4 = (astruct_11 *)(in_struct_1 >> 0x10);
  local_struct_1 = (astruct_11 *)in_struct_1;
  in_struct_1 = (s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
  local_struct_1->field_0x2 = 0x1018;
  if (local_struct_1->field_0x12 != 0x0) {
    SelectPalette16(in_hdc_2,0x0,local_struct_1->field_0x1a);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    in_hdc_2 = (HDC16)s_tile2_bmp_1050_1538;
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  }
  puVar1 = local_struct_1->field_0xa;
  uVar1 = local_struct_1->field_0xc;
  if ((uVar1 | puVar1) != 0x0) {
    ppcVar2 = *puVar1;
    (**ppcVar2)(in_hdc_2,puVar1,uVar1,0x1);
  }
  puVar2 = local_struct_1->field_0xe;
  uVar1 = local_struct_1->field_0x10;
  if ((uVar1 | puVar2) != 0x0) {
    ppcVar2 = *puVar2;
    (**ppcVar2)(in_hdc_2,puVar2,uVar1,0x1);
  }
  _PTR_LOOP_1050_4230 = 0x0;
  pass1_1010_1d80(in_struct_1,unaff_SS);
  return;
}


pub fn unk_destroy_window_op_1018_6bb6(astruct_28 *param_1,HWND16 param_2)
{
  let BVar1: bool;
  astruct_28 *iVar2;
  let uVar2: u16;
  HWND16 hwnd;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_28 *)param_1;
  hwnd = param_2;
  if (iVar2->field_0xea != 0x0) {
    hwnd = s_tile2_bmp_1050_1538;
    PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,iVar2->field_0xea));
  }
  PostMessage16(hwnd,0x0,0x0,0x1110079);
  if (iVar2->field_0xf0 != 0x0) {
    BVar1 = IsWindow16(s_tile2_bmp_1050_1538);
    if (BVar1 != 0x0) {
      DestroyWindow16(s_tile2_bmp_1050_1538);
      iVar2->field_0xf0 = 0x0;
    }
  }
  return;
}


pub fn destroy_window_1018_c518(astruct_29 *param_1)
{
  let BVar1: bool;
  astruct_29 *iVar2;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = (astruct_29 *)param_1;
  param_1 = 0xc8bc;
  iVar2->field_0x2 = 0x1018;
  fn_ptr_1000_17ce(&iVar2->field_0x108,0x1000);
  if (iVar2->field_0x112 != 0x0) {
    BVar1 = IsWindow16(0x1000);
    if (BVar1 != 0x0) {
      DestroyWindow16(s_tile2_bmp_1050_1538);
      iVar2->field_0x112 = 0x0;
    }
  }
  pass1_1020_022c(param_1);
  return;
}


pub fn delete_palette_1018_e16c(param_1: u32,HWND16 param_2)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  HDC16 *b_force_background;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  local_24 = BeginPaint16(param_2,&local_22);
  uVar3 = (param_1 + 0x6);
  puVar1 = (uVar3 + 0xa);
  b_force_background = &local_24;
  uVar3 = *puVar1;
  ppcVar2 = (uVar3 + 0x8);
  (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(puVar1 >> 0x10),
              b_force_background);
  ppcVar2 = (uVar3 + 0x4);
  (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,0x0,&local_24);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&local_22);
  return;
}


pub fn cleanup_ui_op_1020_1038(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  HICON16 unaff_CS;
  let uVar6: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar6 = (iVar4 + 0xc2);
  DestroyIcon16(unaff_CS);
  (iVar4 + 0xc2) = 0x0;
  (iVar4 + 0x8) = 0x0;
  puVar1 = (iVar4 + 0xf6);
  uVar2 = (iVar4 + 0xf8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar6);
  }
  (iVar4 + 0xf6) = 0x0;
  pass1_1010_1dda((iVar4 + 0xf2));
  (iVar4 + 0xf2) = 0x0;
  return;
}


pub fn destroy_window_1020_1d4a(Uparam_1: i32,param_2: i16,HWND16 param_3)
{
  let BVar1: bool;
  HWND16 hwnd;
  
  if (param_2 != 0x0) {
    BVar1 = post_win_msg_1020_1ca4(param_1);
    if (BVar1 != 0x0) {
      hwnd = param_3;
      if ((param_1 + 0x96) != 0x0) {
        hwnd = s_tile2_bmp_1050_1538;
        PostMessage16(param_3,0x0,0x0,0x11100ee);
      }
      DestroyWindow16(hwnd);
    }
  }
  return;
}


pub fn destroy_win_1020_1dea(HWND16 param_1) -> bool

{
  let BVar1: bool;
  let WVar2: u16;
  
  BVar1 = IsWindow16(param_1);
  if (BVar1 != 0x0) {
    WVar2 = GetWindowWord16(s_tile2_bmp_1050_1538,-0xc);
    if (WVar2 == 0x175) {
      DestroyWindow16(s_tile2_bmp_1050_1538);
      return 0x0;
    }
  }
  return 0x1;
}



pub fn destroy_win_1020_1e1e(HWND16 param_1) -> u16

{
  let BVar1: bool;
  let WVar2: u16;
  
  BVar1 = IsWindow16(param_1);
  if (BVar1 != 0x0) {
    WVar2 = GetWindowWord16(s_tile2_bmp_1050_1538,-0xc);
    if ((WVar2 != 0x1) && (WVar2 != 0x175)) {
      DestroyWindow16(s_tile2_bmp_1050_1538);
    }
  }
  return 0x1;
}


pub fn destroy_icon_1020_2c88(param_1: u32,HICON16 param_2)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar6 = (iVar4 + 0xc2);
  DestroyIcon16(param_2);
  (iVar4 + 0xc2) = 0x0;
  (iVar4 + 0x8) = 0x0;
  puVar1 = (iVar4 + 0xf6);
  uVar2 = (iVar4 + 0xf8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar6);
  }
  (iVar4 + 0xf6) = 0x0;
  pass1_1010_1dda((iVar4 + 0xf2));
  (iVar4 + 0xf2) = 0x0;
  return;
}


pub fn cleanup_win_ui_1020_2fea(astruct_12 *in_struct_1,HDC16 in_dc_handle_2)
{
  astruct_12 *iVar1;
  let var2: u16;
  let unaff_SS: u16;
  
  var2 = (in_struct_1 >> 0x10);
  iVar1 = (astruct_12 *)in_struct_1;
  in_struct_1->offset_field_0x0 = 0x363c;
  iVar1->offset_field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    in_dc_handle_2 = 0x1010;
    pass1_1010_1ea6(iVar1->field_0x14,in_struct_1 & 0xffff | var2 << 0x10,
                    unaff_SS);
  }
  SelectObject16(in_dc_handle_2,iVar1->field_0x1c);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,iVar1->field_0x1e);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,iVar1->field_0x20);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  in_struct_1->offset_field_0x0 = 0x3ab0;
  iVar1->offset_field_0x2 = 0x1008;
  in_struct_1->offset_field_0x0 = 0x389a;
  iVar1->offset_field_0x2 = 0x1008;
  return;
}


pub fn destroy_window_1020_3b3e(astruct_30 *param_1,HWND16 param_2)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  astruct_30 *paVar4;
  astruct_30 *uVar5;
  astruct_30 *uVar6;
  HWND16 HVar5;
  let unaff_SS: u16;
  
  uVar6 = (astruct_30 *)(param_1 >> 0x10);
  uVar5 = (astruct_30 *)param_1;
  uVar5->field_0x10e = 0x0;
  HVar5 = param_2;
  if (uVar5->field_0x10a != 0x0) {
    HVar5 = s_tile2_bmp_1050_1538;
    DestroyWindow16(param_2);
  }
  puVar1 = uVar5->field_0xf6;
  uVar3 = uVar5->field_0xf8;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar2 = *puVar1;
    (**ppcVar2)(HVar5,puVar1);
  }
  &uVar5->field_0xf6 = 0x0;
  if (uVar5->field_0xfa != 0x0) {
    uVar3 = uVar6 | uVar5;
    if (param_1 == (astruct_30 *)0x0) {
      paVar4 = (astruct_30 *)0x0;
    }
    else {
      uVar3 = &uVar5->field_0xf2;
      paVar4 = uVar6;
    }
    pass1_1010_1ea6(uVar5->field_0xfa,CONCAT22(paVar4,uVar3),unaff_SS);
  }
  uVar5->field_0xfa = 0x0;
  return;
}



pub fn destroy_cursor_1020_42f4(param_1: *mut u16,HMENparam_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  HMENU16 h_cursor;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x623c;
  (iVar1 + 0x2) = 0x1020;
  (iVar1 + 0xe2) = 0x62d8;
  (iVar1 + 0xe4) = 0x1020;
  h_cursor = param_2;
  if ((iVar1 + 0x106) != 0x0) {
    h_cursor = (HMENU16)s_tile2_bmp_1050_1538;
    DestroyMenu16(param_2);
  }
  DestroyCursor16(h_cursor);
  DestroyCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  pass1_1020_808e(param_1);
  return;
}


pub fn unk_destroy_win_op_1020_694c(Uparam_1: i32,param_2: u16,HWND16 param_3,param_4: u16) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  HWND16 HVar3;
  let iVar4: i16;
  astruct_43 *paVar5;
  let uVar6: u16;
  
  uVar2 = param_2;
  if (param_2 != 0x12b) {
    iVar4 = param_1;
    uVar6 = (param_1 >> 0x10);
    if (param_2 < 0x12c) {
      if (param_2 == 0x6f) {
        paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_4);
        uVar2 = WinHelp16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x9),0x0,
                          CONCAT22(paVar5,0x1));
        return uVar2;
      }
      if (param_2 == 0xeb) {
        uVar2 = GetDlgItem16(param_3,0x1797);
        if (uVar2 != 0x0) {
//LAB_1020_6a6f:
          win_ui_fn_1020_6e98(param_1,s_tile2_bmp_1050_1538,param_4);
          return uVar2;
        }
      }
      else {
        uVar2 = param_2 - 0xef;
        if (uVar2 == 0x0) {
          pass1_1018_2e28((iVar4 + 0xf2));
          pass1_1008_3e0e(param_1);
        }
        else {
          uVar2 = param_2 - 0x129;
          if ((uVar2 != 0x0) && (uVar2 = param_2 - 0x12a, uVar2 == 0x0)) {
            uVar6 = 0xf012;
//LAB_1020_69c3:
            uVar2 = PostMessage16(param_3,0x0,0x0,CONCAT22(0x112,uVar6));
            return uVar2;
          }
        }
      }
    }
    else {
      if (param_2 == 0xbb8) {
        HVar3 = GetDlgItem16(param_3,0x1797);
        if (HVar3 != 0x0) {
          DestroyWindow16(s_tile2_bmp_1050_1538);
        }
        uVar2 = pass1_1018_31d0((iVar4 + 0xf2));
        if (uVar2 != 0x0) {
          uVar1 = (iVar4 + 0xf2);
          uVar2 = pass1_1018_2d9a(uVar1,(uVar1 >> 0x10));
//LAB_1020_6a0b:
          invalidate_rect_1020_735a((iVar4 + 0xf6),0x1018);
          return uVar2;
        }
      }
      else {
        if (param_2 < 0xbb9) {
          if (param_2 == 0x12c) {
            uVar6 = 0xf020;
            goto LAB_1020_69c3;
          }
          uVar2 = param_2 - 0x12d;
          if (param_2 != 0x12c) {
            uVar2 = param_2 - 0x12e;
          }
        }
        else {
          if (param_2 == 0xbb9) {
            HVar3 = GetDlgItem16(param_3,0x1797);
            if (HVar3 != 0x0) {
              DestroyWindow16(s_tile2_bmp_1050_1538);
            }
            uVar2 = pass1_1018_31d0((iVar4 + 0xf2));
            if (uVar2 != 0x0) {
              uVar1 = (iVar4 + 0xf2);
              uVar2 = pass1_1018_2dde(uVar1,(uVar1 >> 0x10));
              goto LAB_1020_6a0b;
            }
          }
          else {
            uVar2 = param_2 - 0xbba;
            if (uVar2 == 0x0) {
              uVar2 = GetDlgItem16(param_3,0x1797);
              if (uVar2 != 0x0) {
                uVar2 = DestroyWindow16(s_tile2_bmp_1050_1538);
                return uVar2;
              }
              goto LAB_1020_6a6f;
            }
          }
        }
      }
    }
  }
  return uVar2;
}


pub fn destroy_icon_1020_6bd2(param_1: u32,param_2: u8,HICON16 param_3)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar6 = (iVar4 + 0xc2);
  DestroyIcon16(param_3);
  (iVar4 + 0xc2) = 0x0;
  (iVar4 + 0x8) = 0x0;
  puVar1 = (iVar4 + 0xf6);
  uVar2 = (iVar4 + 0xf8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar6);
  }
  (iVar4 + 0xf6) = 0x0;
  pass1_1010_1dda((iVar4 + 0xf2));
  (iVar4 + 0xf2) = 0x0;
  return;
}


pub fn cleanup_menu_ui_op_1020_795c(astruct_3 *in_struct_1,HMENU16 in_menu_handle_2)
{
  astruct_3 *local_struct_1;
  astruct_3 *uVar1;
  
  uVar1 = (astruct_3 *)(in_struct_1 >> 0x10);
  local_struct_1 = (astruct_3 *)in_struct_1;
  in_struct_1->address_offset_field_0x0 = 0x7b86;
  local_struct_1->address_offset_field_0x2 = 0x1020;
  if (local_struct_1->field_0xec != 0x0) {
    DestroyMenu16(in_menu_handle_2);
  }
  pass1_1008_57c4(
                  (in_struct_1 & 0xffff0000 |
                  &local_struct_1->field_0xd2));
  in_struct_1->address_offset_field_0x0 = 0x380a;
  local_struct_1->address_offset_field_0x2 = 0x1008;
  in_struct_1->address_offset_field_0x0 = 0x389a;
  local_struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



pub fn destroy_window_1020_8250(param_1: u32,HWND16 param_2)
{
  let BVar1: bool;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0xec) != 0x0) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 != 0x0) {
      DestroyWindow16(s_tile2_bmp_1050_1538);
      (param_1 + 0xec) = 0x0;
    }
  }
  return;
}


pub fn destroy_window_1038_7d88(Uparam_1: i32,param_2: u16)
{
  let in_DX: u16;
  
  pass1_1008_b544((param_1 + 0x94),param_2,in_DX,0x1008);
  DestroyWindow16(0x1008);
  return;
}

pub fn destroy_window_1038_a072(param_1: u16,param_2: u16,param_3: i16,HWND16 param_4)
{
  if (param_3 != 0x0) {
    DestroyWindow16(param_4);
  }
  return;
}


pub fn destroy_win_1038_a3d2(Uparam_1: i32,HWND16 param_2)
{
  GetWindowWord16(param_2,-0x8);
  PostMessage16(s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
  destroy_win_1040_7b98(param_1,&ctx.PTR_LOOP_1050_1040);
  return;
}


pub fn destroy_window_1038_cc00(param_1: i16,param_2: u16,param_3: u16,Uparam_4: i32)
{
  let uVar1: u16;
  let in_DX: *mut u8
  let unaff_DI: i16;
  WNDCLASS16 *unaff_SS;
  let iVar2: i16;
  
  uVar1 = param_4._2_2_ - 0x1cd;
  if (uVar1 == 0x0) {
    iVar2 = 0x1;
  }
  else {
    uVar1 = param_4._2_2_ - 0x1ce;
    if (uVar1 == 0x0) {
      iVar2 = 0x2;
    }
    else {
      uVar1 = param_4._2_2_ - 0x1cf;
      if (uVar1 == 0x0) {
        iVar2 = 0x3;
      }
      else {
        uVar1 = param_4._2_2_ - 0x1d0;
        if (uVar1 == 0x0) {
          iVar2 = 0x4;
        }
        else {
          uVar1 = param_4._2_2_ - 0x1d1;
          if (uVar1 != 0x0) {
            post_win_msg_1040_7b3c
                      (CONCAT22(param_2,param_1),param_3,param_4,
                       param_4._2_2_,&ctx.PTR_LOOP_1050_1040);
            return;
          }
          iVar2 = 0x5;
        }
      }
    }
  }
  pass1_1008_eb74((param_1 + 0x8e),iVar2,in_DX,unaff_DI,unaff_SS);
  if (uVar1 != 0x0) {
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uVar1,0x1),unaff_SS,uVar1,in_DX);
    DestroyWindow16(0x1008);
  }
  return;
}


pub fn destroy_window_1038_cd88(astruct_1 *param_1)
{
  let unaff_SS: u16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  (param_1 + 0x92) = 0x1;
  unk_win_msg_op_1008_9510
            ((param_1 & 0xffff0000 | (param_1 + 0x92)),0x1008,
             unaff_SS);
  DestroyWindow16(0x1008);
  return;
}


pub fn destroy_win_1038_e1dc(param_1: u16,param_2: u16,param_3: i16,HWND16 param_4)
{
  let UVar1: u16;
  LPARAM lparam;
  
  if (param_3 != 0x0) {
    UVar1 = IsDlgButtonChecked(param_4,0x1807);
    if (UVar1 == 0x0) {
      param_4 = s_tile2_bmp_1050_1538;
      UVar1 = IsDlgButtonChecked(s_tile2_bmp_1050_1538,0x1806);
      if (UVar1 == 0x0) goto LAB_1038_e229;
      lparam = 0x1110130;
    }
    else {
      lparam = 0x111012f;
    }
    param_4 = s_tile2_bmp_1050_1538;
    SendMessage16(s_tile2_bmp_1050_1538,0x0,0x0,lparam);
  }
//LAB_1038_e229:
  DestroyWindow16(param_4);
  return;
}


pub fn destroy_win_1038_ef3a(astruct_31 *param_1,HWND16 param_2)
{
  astruct_31 *iVar1;
  astruct_31 *uVar1;
  
  uVar1 = (astruct_31 *)(param_1 >> 0x10);
  iVar1 = (astruct_31 *)param_1;
  param_1 = 0x67c;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  if (iVar1->field_0x96 != 0x0) {
    DestroyWindow16(param_2);
    iVar1->field_0x96 = 0x0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar1->field_0x6);
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1040);
  return;
}


pub fn destroy_win_1040_5256(astruct_34 *param_1,HWND16 param_2)
{
  ULONG *pUVar1;
  let uVar2: u16;
  code **ppcVar3;
  let Bvar4: bool;
  astruct_34 *iVar5;
  let uVar5: u16;
  HWND16 HVar6;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_34 *)param_1;
  HVar6 = param_2;
  if (iVar5->field_0xb6 != 0x0) {
    HVar6 = s_tile2_bmp_1050_1538;
    BVar4 = IsWindow16(param_2);
    if (BVar4 != 0x0) {
      HVar6 = s_tile2_bmp_1050_1538;
      DestroyWindow16(s_tile2_bmp_1050_1538);
    }
  }
  iVar5->field_0xb6 = 0x0;
  pUVar1 = iVar5->field_0x94;
  uVar2 = iVar5->field_0x96;
  if ((uVar2 | pUVar1) != 0x0) {
    ppcVar3 = *pUVar1;
    (**ppcVar3)(HVar6,pUVar1,uVar2,0x1);
  }
  &iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = 0x0;
  return;
}


pub fn destroy_win_1040_7b98(Uparam_1: i32,HWND16 param_2)
{
  if ((param_1 + 0x74) == 0x0) {
    DestroyWindow16(param_2);
  }
  return;
}



pub fn destroy_win_1040_8212(Uparam_1: i32,HWND16 param_2)
{
  let is_window: bool;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x8c) != 0x0) {
    is_window = IsWindow16(param_2);
    if (is_window != 0x0) {
      DestroyWindow16(s_tile2_bmp_1050_1538);
      (param_1 + 0x8c) = 0x0;
    }
  }
  return;
}


pub fn destroy_win_1040_8b7e(HWND16 param_1)
{
  DestroyWindow16(param_1);
  return;
}


pub fn destroy_window_1040_b726(ULONG *param_1,param_2: i16,HWND16 in_win_handle_3)
{
  code **ppcVar1;
  
  if (param_2 != 0x0) {
    ppcVar1 = (*param_1 + 0x78);
    (**ppcVar1)(in_win_handle_3,param_1);
  }
  DestroyWindow16(in_win_handle_3);
  return;
}


pub fn destroy_win_1040_bb78(astruct_35 *param_1,HWND16 param_2)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let Bvar4: bool;
  astruct_35 *iVar5;
  let uVar5: u16;
  HWND16 HVar6;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_35 *)param_1;
  HVar6 = param_2;
  if (iVar5->field_0xb6 != 0x0) {
    HVar6 = s_tile2_bmp_1050_1538;
    BVar4 = IsWindow16(param_2);
    if (BVar4 != 0x0) {
      HVar6 = s_tile2_bmp_1050_1538;
      DestroyWindow16(s_tile2_bmp_1050_1538);
    }
  }
  iVar5->field_0xb6 = 0x0;
  puVar1 = iVar5->field_0x94;
  uVar2 = iVar5->field_0x96;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(HVar6,puVar1,uVar2,0x1);
  }
  &iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = 0x0;
  return;
}
