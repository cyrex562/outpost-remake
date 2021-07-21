use crate::defines::{Struct13, Struct18, Struct76, Struct79, U32Ptr};
use crate::draw::draw_1008::unk_draw_op_1008_61b2;
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::global::AppContext;
use crate::mem_1000::mem_op_1000_179c;
use crate::misc::empty_1008_8fc4;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1008::{pass1_1008_3e76, pass1_1008_3e94, pass1_1008_41bc, pass1_1008_4480, pass1_1008_4772, pass1_1008_5118, pass1_1008_57a4, pass1_1008_5b12, pass1_1008_8b20, pass1_1008_941a};
use crate::pass::pass_1010::{pass1_1010_1ea6, pass1_1010_375e, pass1_1010_3770, pass1_1010_454a, pass1_1010_4c2c, pass1_1010_4dc8, pass1_1010_4df0, pass1_1010_ecc6};
use crate::pass::pass_1018::{pass1_1018_017c, pass1_1018_0a50, pass1_1018_0a76, pass1_1018_0d9a, pass1_1018_1054, pass1_1018_108c, pass1_1018_1320, pass1_1018_15f6, pass1_1018_161c, pass1_1018_181c, pass1_1018_25d2, pass1_1018_265c, pass1_1018_266a, pass1_1018_2862, pass1_1018_31d0};
use crate::pass::pass_1020::{draw_1020_239c, pass1_1020_2286, pass1_1020_2488, pass1_1020_5d56, pass1_1020_6498, pass1_1020_64d4, pass1_1020_68de};
use crate::pass::pass_1030::pass1_1030_8308;
use crate::string::string_1000::string_1000_3d3e;
use crate::struct_ops::struct_1008::clear_struct_1008_3e38;
use crate::sys_api::get_sys_metrics_1020_7c1a;
use crate::ui::ui_1008::{create_palette_1008_4e38, file_and_draw_op_1008_4f20, win_1008_5c9e, win_ui_reg_class_1008_96d2};
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, read_struct_from_addr, SUB42, ZEXT24};
use crate::win_struct::{COLORREF, DEVMODEA, HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HINSTANCE16, HPALETTE16, HPEN16, HWND16, LOGPALETTE, PAINTSTRUCT16, POINT16, RECT16};
use crate::winapi::{BeginPaint16, CreateDC16, CreatePen16, CreateSolidBrush16, DeleteDC16, DeleteObject16, DrawIcon16, Ellipse16, EndPaint16, FillRect16, GetClientRect16, GetDC16, GetStockObject16, GetTextExtent16, GetWindowDC16, GetWindowRect16, InvalidateRect16, IsIconic16, LineTo16, LoadAccelerators16, lstrlen16, MoveTo16, MoveToEx16, Polygon16, PostMessage16, PtInRect16, RealizePalette16, Rectangle16, ReleaseCapture16, ReleaseDC16, SelectObject16, SelectPalette16, SetBkColor16, SetCursor16, SetMapMode16, SetTextColor16, TextOut16, UnrealizeObject16, ValidateRect16};

pub fn unk_draw_op_1020_0000(param_1: u32, param_2: HWND16, param_3: u16)
{
  let pi_var1: U32Ptr;
  let ppc_var2: u32;
  let u_var3: u32;
  let i_var4: i16;
  let i_var5: i16 = 0i16;
  let u_var6: u16 = 0u16;
  let hwnd: HWND16;
  let u_var7: u16;
  let local_c4: [u8;6];
  let local_be: [u8;2];
  let pi_stack184: U32Ptr;
  let local_b4: i16;
  let i_stack178: i16;
  let ai_stack176: [i16;0x3c];
  let i_stack56: i16;
  let i_stack48: i16;
  let pa_stack46: &mut Struct76;
  let local_2a: i16 = 0;
  let local_28: i16 = 0;
  let pu_stack38: u32;
  let mut paint_1: PAINTSTRUCT16;
  
                    // Segment:    5
                    // Offset:     00033420
                    // Length:     efba
                    // Min Alloc:  efba
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  // u_var6 = (param_1 >> 0x10);
  // i_var5 = param_1;
  u_var7 = (i_var5 + 0x4) as u16;
  BeginPaint16(param_2,&mut paint_1);
  u_var3 = (i_var5 + 0x14) as u32;
  pu_stack38 = (u_var3 + 0xa);
  pass1_1008_3e94((param_1 & 0xffff0000 | (i_var5 + 0x18)),
                  CONCAT22(param_3, local_2a as u16),
                  CONCAT22(param_3, local_28 as u16));
  hwnd = 0x1008;
  pass1_1008_4480(pu_stack38,
                  (param_1 & 0xffff0000 | (i_var5 + 0x18)),
                  &mut struct_from_addr::<Struct76>((i_var5 + 0x24) as u32), param_3);
  // pa_stack46 = 0x0;
    // TODO: refactor
  // for (i_stack48 = 0x0; i_stack48 < 0x6; i_stack48 += 0x1) {
  //   u_var3 = (i_var5 + 0x14);
  //   hwnd = 0x1010;
  //   pass1_1010_2b78(u_var3,(u_var3 >> 0x10),i_stack48,
  //                   CONCAT22(param_3,&local_b4));
  //   if (local_b4 == 0x0) {
  //     for (i_stack56 = 0x0; i_var4 = i_stack56, i_stack56 <= i_stack178; i_stack56 += 0x1) {
  //       pi_var1 = ai_stack176 + i_stack56 * 0x3;
  //       pi_stack184 = pi_var1;
  //       if (ai_stack176[i_stack56 * 0x3 + 0x2] != 0x0) {
  //         pa_stack46 =
  //                     pass1_1010_2b98((i_var5 + 0x14),
  //                                     ai_stack176[i_stack56 * 0x3 + 0x2]);
  //         pass1_1008_3e54(CONCAT22(param_3,local_be),0x0,
  //                         ai_stack176[i_var4 * 0x3 + 0x1] + local_2a,*pi_var1 + local_28);
  //         hwnd = 0x1008;
  //         pass1_1008_4480(pu_stack38,CONCAT22(param_3,local_be),pa_stack46,
  //                         param_3);
  //       }
  //     }
  //   }
  //   else {
  //     _local_be = CONCAT22(param_3,ai_stack176 + i_stack178 * 0x3);
  //     if (ai_stack176[i_stack178 * 0x3 + 0x2] != 0x0) {
  //       pa_stack46 =
  //                   pass1_1010_2b98((i_var5 + 0x14),
  //                                   ai_stack176[i_stack178 * 0x3 + 0x2]);
  //       pass1_1008_3e54(CONCAT22(param_3,local_c4),0x0,
  //                       (_local_be + 0x2) + local_2a,*_local_be + local_28);
  //       hwnd = 0x1008;
  //       pass1_1008_4480(pu_stack38,CONCAT22(param_3,local_c4),pa_stack46,
  //                       param_3);
  //     }
  //   }
  // }
  ppc_var2 = (*pu_stack38 + 0x4);
  (**ppc_var2)(hwnd, pu_stack38, (pu_stack38 >> 0x10), 0x0, 0x0, i_var5 + 0xa,
               u_var6, u_var7);
  EndPaint16(hwnd,&mut paint_1);
  return;
}


pub fn draw_op_1020_041e(param_1: u32,param_2: u16)
{
  fill_rect_1020_065e((param_1 + 0xe6),param_2);
  return;
}


pub fn fill_rect_1020_065e(param_1: u32,in_win_handle_2: HWND16)
{
  let ppc_var1: u32;
  let u_var2: u32;
  let i_var3: i16;
  let u_var4: u16;
  let mut rect_1: RECT16;
  let mut u_stack50: RECT16;
  let i_stack48: i16;
  let i_stack46: i16;
  let brush_1:HBRUSH16;
  HDC16 *pHStack42;
  let pu_stack40: u32;
  let local_24: HDC16;
  let mut paint_1: PAINTSTRUCT16;
  
  // u_var4 = (param_1 >> 0x10) as u16;
  i_var3 = param_1 as i16;
  local_24 = BeginPaint16(in_win_handle_2,&mut paint_1);
  if (0x280 < (i_var3 + 0xa)) {
    brush_1 = CreateSolidBrush16(ctx.s_tile2_bmp_1050_1538 as COLORREF);
    rect_1 = RECT16{ x: 0, y: 0 };
    u_stack50 = RECT16{ x: 0, y: 0 };
    i_stack48 = (i_var3 + 0xa) + -0x1;
    i_stack46 = (i_var3 + 0xc) + -0x1;
    FillRect16(ctx.s_tile2_bmp_1050_1538, &rect_1, brush_1);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  u_var2 = (i_var3 + 0x6) as u32;
  pu_stack40 = (u_var2 + 0xe);
  pHStack42 = &local_24;
  u_var2 = *pu_stack40;
  ppc_var1 = (u_var2 + 0x8);
  (**ppc_var1)(ctx.s_tile2_bmp_1050_1538, pu_stack40, (pu_stack40 >> 0x10),
               pHStack42);
  ppc_var1 = (u_var2 + 0x4);
  (**ppc_var1)(ctx.s_tile2_bmp_1050_1538, pu_stack40, (i_var3 + 0x10),
               (i_var3 + 0xe), &local_24);
  pHStack42 = SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,pHStack42)
  ;
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&mut paint_1);
  return;
}


pub fn unk_draw_op_1020_0c3e(param_1: u32,param_2: HWND16)
{
  let pu_var1: u32;
  let ppc_var2: u32;
  let u_var3: u32;
  HDC16 *b_force_background;
  let i_var4: i16;
  let i_var5: i16;
  let u_var6: u16;
  let u_var7: u16;
  let u_stack40: u16;
  let local_24: HDC16;
  let mut paint_1: PAINTSTRUCT16;
  
  // u_var6 = (param_1 >> 0x10) as u16;
  i_var4 = param_1 as i16;
  local_24 = BeginPaint16(param_2,&mut paint_1);
  u_var3 = (i_var4 + 0x6) as u32;
  // u_var7 = (u_var3 >> 0x10);
  i_var5 = u_var3 as i16;
  pu_var1 = (i_var5 + 0xa) as u32;
  u_stack40 = pu_var1 as u16;
  if (((i_var5 + 0xc) | u_stack40) != 0x0) {
    b_force_background = &local_24;
    u_var3 = *pu_var1;
    ppc_var2 = (u_var3 + 0x8);
    (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, u_stack40, (pu_var1 >> 0x10),
                 b_force_background);
    ppc_var2 = (u_var3 + 0x4);
    (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, pu_var1, (i_var4 + 0xc),
                 (i_var4 + 0xa), &local_24);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  EndPaint16(s_tile2_bmp_1050_1538,&mut paint_1);
  return;
}



pub unsafe fn win_ui_palette_op_1020_0cd2(param_1: u32,
                                          param_2: HWND16,
                                          extraout_dx: u16)
{
  let u_var1: u16;
  let pu_var2: u32;
  let ppc_var3: u32;
  let u_var4: u32;
  let u_var5: u16;
  let hdc: HDC16;
  let b_force_background: bool;
  let palette_1: HPALETTE16;
  let uvar6: u16;
  // let extraout_dx: u16;
  let i_var7: i16;
  let u_var8: u16;
  let mut struct_1: Struct13;
  let u_stack6: u16;
  
  u_var4 = (param_1 + 0x6);
  // u_var8 = (u_var4 >> 0x10);
  i_var7 = u_var4 as i16;
  pu_var2 = (i_var7 + 0xa) as u32;
  u_var1 = (i_var7 + 0xc) as u16;
  u_stack6 = pu_var2 as u16;
  u_var5 = u_var1 | u_stack6;
  if (u_var5 != 0x0) {
    ppc_var3 = (*pu_var2 + 0x14);
    (**ppc_var3)(param_2, u_stack6, u_var1);
    struct_1 = struct_from_addr::<Struct13>(CONCAT22(extraout_dx, u_var5));
    u_var5 = extraout_dx | u_var5;
    if u_var5 != 0x0 {
      hdc = GetDC16(param_2);
      b_force_background = (hdc != 0);
      create_palette_1008_4e38(&mut struct_1, 0x1008, u_var5 as U32Ptr);
      palette_1 = SelectPalette16(0x1008, 0x0, b_force_background);
      uvar6 = RealizePalette16(ctx.s_tile2_bmp_1050_1538);
      SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x1, palette_1 !=0);
      DeleteObject16(ctx.s_tile2_bmp_1050_1538);
      if 0x0 < uvar6 {
        InvalidateRect16(s_tile2_bmp_1050_1538,
                         (&ctx.PTR_LOOP_1050_0000 + 0x1),false);
      }
      ReleaseDC16(s_tile2_bmp_1050_1538,hdc);
      return;
    }
  }
  return;
}


pub fn realize_palette_1020_0e46(param_1: u32,param_2: i16,param_3: HGDIOBJ16)
{
  let pu_var1: u32;
  let ppc_var2: u32;
  let u_var3: u32;
  let i_var4: i16;
  let u_var5: u16;
  
  if (param_2 != 0x0) {
    u_var3 = (param_1 + 0xf2);
    // u_var5 = (u_var3 >> 0x10);
    i_var4 = u_var3 as i16;
    pu_var1 = (i_var4 + 0x66) as u32;
    ppc_var2 = (*pu_var1 + 0x18);
    (**ppc_var2)(param_3, pu_var1, (i_var4 + 0x68));
    UnrealizeObject16(param_3);
    RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}


pub fn invalidate_rect_1020_157c(param_1: u32,param_2: i16,param_3: HWND16)
{
  let bvar1: bool;
  let mut rect_1: RECT16 = RECT16::new();
  let u_stack4: u16;
  
  if param_2 == 0x1 {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if param_2 == 0x2 {
    bvar1 = IsIconic16(param_3);
    if bvar1 == false {
      GetClientRect16(s_tile2_bmp_1050_1538,&mut rect_1);
      u_stack4 = 0x9a;
      InvalidateRect16(s_tile2_bmp_1050_1538, &mut rect_1, false);
      return;
    }
  }
  return;
}



pub fn draw_op_1020_15de(
    param_1: i32,
    in_win_handle_2: HWND16,
    unaff_ss: u16)
{
  let u_var1: u32;
  let ppc_var2: u32;
  let bvar3: bool;
  let mut u_var4 = 0u16;
  let i_var5: i16;
  let u_var6: u16;
  let hwnd: HWND16;
  let u_var7: u32;
  let u_var8: u16;
  let u_var9: u16;
  let local_24: HDC16;
  let mut paint_1: PAINTSTRUCT16;
  
 // u_var6 = (param_1 >> 0x10);
  i_var5 = param_1 as i16;
  u_var9 = (i_var5 + 0x4) as u16;
  local_24 = BeginPaint16(in_win_handle_2,&mut paint_1);
  u_var8 = (i_var5 + 0x4) as u16;
  hwnd = ctx.s_tile2_bmp_1050_1538;
  bvar3 = IsIconic16(s_tile2_bmp_1050_1538);
  if (bvar3 == false) {
    hwnd = 0x1010;
    u_var7 = pass1_1010_454a(((i_var5 + 0x14) as u32));
   // u_var4 = (u_var7 >> 0x10);
    if ((u_var4 | u_var7) != 0x0) {
      u_var1 = (i_var5 + 0x14) as u32;
      hwnd = 0x1008;
      pass1_1008_4480(((i_var5 + 0x18) as u32),
                      (u_var1 & 0xffff0000 | (u_var1 + 0x76)),
                      read_struct_from_addr(u_var7 & 0xffff | u_var4 << 0x10), unaff_ss);
    }
    ppc_var2 = ((i_var5 + 0x18) + 0x4) as u32;
    (**ppc_var2)(hwnd, (i_var5 + 0x18), 0x0, &local_24, unaff_ss, u_var8, u_var9);
  }
  else {
    draw_op_1020_1674(ctx, param_1, s_tile2_bmp_1050_1538);
  }
  EndPaint16(hwnd,&mut paint_1);
  return;
}



pub fn draw_op_1020_1674(
    ctx: &mut AppContext,
    param_1: i32,
    obj_id: i16)
{
  let ppc_var1: u32;
  let u_var2: u16;
  let mut local_1a = RECT16::new();
  let u_stack24: u16;
  let i_stack22: i16;
  let i_stack20: i16;
  let i_stack18: i16;
  let i_stack16: i16;
  let mut local_e = RECT16::new();
  let u_stack10: i16;
  let mut i_stack8 = 0i16;
  let mut obj_handle_1: HGDIOBJ16;
  let i_stack4: i16;
  
  if ctx.PTR_LOOP_1050_0010 == 0x0 {
   // u_var2 = (param_1 >> 0x10);
    ppc_var1 = ((param_1 + 0x14) + 0x2c) as u32;
    i_stack4 = (**ppc_var1)(obj_id, (param_1 + 0x14));
    if i_stack4 != 0x0 {
      obj_handle_1 = GetStockObject16(obj_id);
      GetClientRect16(ctx.s_tile2_bmp_1050_1538 as u16, &mut local_e);
      // local_1a = 0x0;
      u_stack24 = 0x0;
      i_stack22 = (iStack10 - local_e.x) + -0x1;
      i_stack20 = (i_stack8 - local_e.y) + -0x1;
      i_stack18 = i_stack20;
      i_stack16 = i_stack22;
      FillRect16(ctx.s_tile2_bmp_1050_1538 as u16, &local_1a, obj_handle_1);
      DrawIcon16(ctx.s_tile2_bmp_1050_1538 as u16, i_stack4, 0x2, 0x2);
    }
  }
  return;
}


pub fn invalidate_rect_1020_1fb2(param_1: u32,param_2: i16,param_3: HWND16)
{
  let mut local_16 = RECT16::new();
  let u_stack20: u16;
  let i_stack18: i16;
  let u_stack16: u16;
  let mut local_e = RECT16::new();
  let u_stack10: i16;
  let u_stack6: u16;
  let u_stack4: u16;
  
  if (param_2 == 0x1) {
    (param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  GetWindowRect16(param_3,&mut local_e);
  // local_16 = 0x0;
  u_stack6 = 0x46;
  u_stack20 = 0x46;
  i_stack18 = iStack10 - local_e.x;
  u_stack4 = 0x5f;
  u_stack16 = 0x5f;
  InvalidateRect16(s_tile2_bmp_1050_1538,&local_16, false);
  return;
}



// WARNING: Inlined function: struct_1010_4d5c

pub unsafe fn unk_draw_op_1020_2020(
    param_1: u32,
    param_2: HWND16,
    param_3: u16,
    stack0xfffe: u16,
    extraout_dx: u16)
{
  let ppc_var1: u32;
  let u_var2: u32;
  let pu_var3: u32;
  let u_var4: u16;
  let mut paint_handle_2: HDC16;
  let i_var6: i16;
  let hvar7: HPEN16;
  let hvar8: HGDIOBJ16;
  let hvar9: HBRUSH16;
  let mut pu_var10: U32Ptr = 0;
  // let extraout_dx: u16;
  let u_var11: u16;
  let i_var12: i16;
  let i_var13: i16;
  let mut ptr_1: U32Ptr = 0;
  let u_var15: u16;
  let u_var16: u16;
  let style: i16;
  let u_var17: u32;
  let mut pi_var18:U32Ptr = 0;
  let u_var19: u8;
  let u_var20: u8;
  let i_var21: i16;
  let mut u_var22: u8 = 0;
  let u_var23: u8;
  let mut local_38: [u8;6] = [0;6];
  let local_32: u16;
  let u_stack48: u16;
  let u_stack46: u32;
  let u_stack42: u16;
  let pu_stack40: u32;
  let mut paint_handle_1: HDC16;
  let mut local_22: PAINTSTRUCT16;
  
  ptr_1 = stack0xfffe as u32;
 // u_var15 = (param_1 >> 0x10);
  i_var12 = param_1 as i16;
  u_var16 = (i_var12 + 0x4) as u16;
  paint_handle_1 = BeginPaint16(param_2, &mut local_22);
  pu_stack40 = pass1_1010_4c2c(((i_var12 + 0x6) as u32));
  paint_handle_2 = paint_handle_1;
  ppc_var1 = (*pu_stack40 + 0x8);
  (**ppc_var1)(0x1010, pu_stack40, (pu_stack40 >> 0x10), paint_handle_2, param_3, u_var16)
  ;
  (i_var12 + 0x10) = paint_handle_2;
  u_var2 = (i_var12 + 0x6) as u32;
  u_stack42 = (u_var2 + 0x30) as u16;
  u_var2 = (i_var12 + 0x6) as u32;
  u_stack46 = (u_var2 + 0x12);
  u_stack48 = 0x14;
  local_32 = 0x0;
  style = 0x1008;
  clear_struct_1008_3e38(CONCAT22(param_3, local_38[0] as u16));
  while (ptr_1 + -0x38) < (ptr_1 + -0x28) {
    i_var12 = ((ptr_1 + -0x38) * 0x4) as i16;
    u_var2 = (ptr_1 + -0x2c);
    u_var17 = pass1_1008_4772(read_struct_from_addr((i_var12 + u_var2) as u32));
   // pu_var10 = (u_var17 >> 0x10);
    (ptr_1 + -0x44) = u_var17;
    (ptr_1 + -0x42) = pu_var10;
    u_var2 = (ptr_1 + 0x6);
    pass1_1020_2286(u_var2 as u16,
                    ((u_var2 >> 0x10) as u16),
                    &mut CONCAT13((param_3 >> 0x8),
                                    CONCAT12(param_3 as u8,
                                             (ptr_1 + -0x30) as u16)),
                    ((u_var17 + 0x8) as i16));
    u_var2 = (ptr_1 + -0x30);
    pass1_1008_3e76(CONCAT22(param_3, (ptr_1 + -0x36) as u16), 0x0, u_var2 as i16,
                    ((u_var2 >> 0x10) as u16));
    u_var2 = (ptr_1 + -0x2c);
    pass1_1008_4480((ptr_1 + -0x26),
                    CONCAT22(param_3, (ptr_1 + -0x36) as u16),
                    read_struct_from_addr(u_var2 + i_var12), param_3);
    i_var12 = (ptr_1 + -0x38) as i16;
    u_var2 = (ptr_1 + -0x30);
    u_var15 = u_var2 as u16;
   // u_var22 = (u_var2 >> 0x10);
    u_var23 = (u_var2 >> 0x18) as u8;
    u_var2 = (ptr_1 + -0x44);
   // u_var16 = (u_var2 >> 0x10);
    i_var13 = u_var2 as i16;
    i_var6 = (i_var13 + 0x4) + (ptr_1 + -0x2e);
    i_var13 = (i_var13 + 0x8) + (ptr_1 + -0x30);
    u_var2 = (ptr_1 + 0x6);
    u_var2 = (u_var2 + 0x6);
    i_var21 = u_var2 as i16;
   // u_var16 = (u_var2 >> 0x10);
    u_var19 = 0x8;
    u_var20 = 0x10;
    if ((i_var21 + 0x1a) == 0x0) {
      u_var4 = ((i_var21 + 0x30) << 0x3) as u16;
      mem_op_1000_179c(ctx, u_var4, read_struct_from_addr::<Struct79>(pu_var10), 0x1000);
      (i_var21 + 0x1a) = u_var4 as i16;
      (i_var21 + 0x1c) = pu_var10 as i16;
    }
    u_var2 = (i_var21 + 0x1a) as u32;
    i_var12 *= 0x8;
    (u_var2 + i_var12) = CONCAT11(u_var23, u_var22) as u32;
    u_var2 = (i_var21 + 0x1a) as u32;
    (u_var2 + i_var12 + 0x2) = u_var15 as u32;
    u_var2 = (i_var21 + 0x1a) as u32;
    (u_var2 + i_var12 + 0x4) = i_var6 as u32;
    u_var2 = (i_var21 + 0x1a) as u32;
    (u_var2 + i_var12 + 0x6) = i_var13 as u32;
    style = CONCAT11(u_var20, u_var19) as i16;
    u_var2 = (ptr_1 + -0x44);
    pi_var18 = (ptr_1 + -0x2e);
    *pi_var18 = *pi_var18 +
               (-((ptr_1 + -0x38) == 0x0) & 0x5) + 0x14 +
               (u_var2 + 0x4);
    pi_var18 = (ptr_1 + -0x38);
    *pi_var18 = *pi_var18 + 0x1;
  }
  pu_var3 = (ptr_1 + -0x26);
  ppc_var1 = (*pu_var3 + 0x4);
  (**ppc_var1)(style, pu_var3, (pu_var3 >> 0x10), 0x0, 0x0, ptr_1 + -0x22
               , param_3);
  u_var11 = extraout_dx;
  hvar7 = CreatePen16(style, 0x25, 0x100);
 (ptr_1 + -0x3a) = hvar7 as u32;
  hvar8 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar7);
  (ptr_1 + -0x3c) = hvar8 as u32;
  hvar9 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
  (ptr_1 + -0x3e) = hvar9 as u32;
  hvar8 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar9);
  (ptr_1 + -0x40) = hvar8 as u32;
  draw_line_1020_229c((ptr_1 + 0x6), s_tile2_bmp_1050_1538);
  u_var2 = (ptr_1 + 0x6);
  pass1_1010_4df0((u_var2 + 0x6), u_var11, param_3);
  if (hvar8 == 0x0) {
    SelectObject16(0x1010,((ptr_1 + -0x3c) as u16));
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538,((ptr_1 + -0x40) as u16));
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    hvar9 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    (ptr_1 + -0x3e) = hvar9 as u32;
    hvar7 = CreatePen16(s_tile2_bmp_1050_1538, 0xff, 0x0);
   (ptr_1 + -0x3a) = hvar7 as u32;
    SelectObject16(ctx.s_tile2_bmp_1050_1538,((ptr_1 + -0x3e) as u16));
    SelectObject16(ctx.s_tile2_bmp_1050_1538,((ptr_1 + -0x3a) as u16));
  }
  u_var2 = (ptr_1 + 0x6);
  pi_var18 = pass1_1010_4dc8((u_var2 + 0x6));
 // u_var15 = (pi_var18 >> 0x10);
  u_var16 = SUB42(pi_var18 as u16, 0x0) as u16;
  draw_1020_239c((ptr_1 + 0x6), pi_var18, param_3);
  u_var2 = (ptr_1 + 0x6);
  u_var2 = (u_var2 + 0x6);
  if ((u_var2 + 0x2c) != 0x0) {
    pass1_1020_2488((ptr_1 + 0x6), u_var16, u_var15);
  }
  u_var2 = (ptr_1 + 0x6);
  SelectPalette16(0x1010,0x0,(u_var2 + 0x10));
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  SelectObject16(ctx.s_tile2_bmp_1050_1538,((ptr_1 + -0x3c) as u16));
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  SelectObject16(ctx.s_tile2_bmp_1050_1538,((ptr_1 + -0x40) as u16));
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,(ptr_1 + -0x20));
  return;
}


pub fn draw_line_1020_229c(param_1: u32,param_2: HDC16)
{
  let i_var1: i16;
  INT16 *pIVar2;
  let u_var3: u32;
  let i_var4: i16;
  let i_var5: i16;
  let pi_var6: U32Ptr;
  let u_var7: u16;
  let u_stack10: i16;
  
 // u_var7 = (param_1 >> 0x10);
  u_var3 = (param_1 + 0x6);
  i_var1 = (u_var3 + 0x30) as i16;
  u_var3 = (param_1 + 0x6);
  pIVar2 = (u_var3 + 0x1a);
  MoveTo16(param_2,0x5,*pIVar2);
 // u_var7 = (pIVar2 >> 0x10);
  i_var5 = pIVar2;
  LineTo16(ctx.s_tile2_bmp_1050_1538,0x5,(i_var5 + i_var1 * 0x8 + -0x4));
    // TODO: refactor
  // for (iStack10 = 0x0; iStack10 < i_var1; iStack10 += 0x1) {
  //   pi_var6 = (iStack10 * 0x8 + i_var5);
  //   i_var4 = (pi_var6[0x2] - *pi_var6 >> 0x1) + *pi_var6;
  //   MoveTo16(ctx.s_tile2_bmp_1050_1538,0x5,i_var4);
  //   LineTo16(ctx.s_tile2_bmp_1050_1538,0xa,i_var4);
  // }
  MoveTo16(ctx.s_tile2_bmp_1050_1538,0x5f,*pIVar2);
  LineTo16(ctx.s_tile2_bmp_1050_1538,0x5f,(i_var5 + i_var1 * 0x8 + -0x4));
  // TODO: refactor for loop
    // for (iStack10 = 0x0; iStack10 < i_var1; iStack10 += 0x1) {
  //   pi_var6 = (iStack10 * 0x8 + i_var5);
  //   i_var4 = (pi_var6[0x2] - *pi_var6 >> 0x1) + *pi_var6;
  //   MoveTo16(ctx.s_tile2_bmp_1050_1538,0x5f,i_var4);
  //   LineTo16(ctx.s_tile2_bmp_1050_1538,0x5a,i_var4);
  // }
  return;
}


pub fn draw_polygon_1020_2474(param_1: u16,param_2: u16,param_3: u32,param_4: HDC16)
{
  Polygon16(param_4,param_3,((param_3 >> 0x10) as i16));
  return;
}


pub fn realize_palette_1020_2992(param_1: i32,param_2: i16)
{
  let ppc_var1: u32;
  let pu_var2: u32;
  
  if (param_2 != 0x0) {
    pu_var2 = pass1_1018_0a50(((param_1 + 0xf2) as u32));
    ppc_var1 = (*pu_var2 + 0x18);
    (**ppc_var1)(0x1018, pu_var2, (pu_var2 >> 0x10));
    UnrealizeObject16(0x1018);
    GetDC16(s_tile2_bmp_1050_1538);
    RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}


pub unsafe fn invalidate_rect_1020_2ae4(param_1: U32Ptr, param_2: u16, param_3: HWND16, param_4: u16)
{
  let ppc_var1: u32;
  let c_var2: u8;
  let i_var3: i16;
  let in_dx: U32Ptr;
  let u_var4: u16;
  let u_var5: u16;
  let unaff_di: i16;
  let pu_var6: U32Ptr;
  let u_var7: u32;
  let pa_var8: &mut Struct43;
  let u_var9: u16;
  let u_var10: u16;
  
  if (param_2 != 0x129) {
    u_var5 = param_1;
   // u_var9 = (param_1 >> 0x10);
    if (0x129 < param_2) {
      if (param_2 == 0x12a) {
        u_var9 = 0xf012;
      }
      else {
        if (param_2 == 0x12b) {
          return;
        }
        if (param_2 == 0x12c) {
          u_var9 = 0xf020;
        }
        else {
          if (param_2 == 0x12d) {
            return;
          }
          if (param_2 != 0x12e) {
            return;
          }
          u_var9 = 0xf060;
        }
      }
      PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x112, u_var9) as i32);
      return;
    }
    if (param_2 == 0xfb) {
      pu_var6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x30, param_4, in_dx, unaff_di);
      pass1_1010_375e(pu_var6);
      ppc_var1 = (*param_1 + 0x14);
      (**ppc_var1)();
      u_var7 = pass1_1010_375e(pu_var6);
     // u_var4 = (u_var7 >> 0x10);
      pass1_1018_181c(((u_var5 + 0xf2) as u32),
                      (u_var7 & 0xffff | u_var4 << 0x10),
                      (uchar)(u_var7 & 0xffff), u_var4);
      return;
    }
    if (param_2 < 0xfc) {
      c_var2 = param_2 as u8;
      if (c_var2 == 'o') {
        pa_var8 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0x1f8, param_4);
        WinHelp16(0x1010,(s_New_failed_in_Op__Op_1050_0020 + 0xa),0x0,
                  CONCAT22(pa_var8, 0x1));
        return;
      }
      if (c_var2 == 'r') {
        i_var3 = (u_var5 + 0xa) as i16;
        u_var10 = u_var9;
        pu_var6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x30, param_4, in_dx, unaff_di);
       // u_var4 = (pu_var6 >> 0x10);
        pass1_1010_3770(pu_var6, CONCAT22(u_var10, i_var3 as u16), u_var4);
        pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (u_var5 + 0x8), 0x3, u_var4, u_var5,
                        &ctx.PTR_LOOP_1050_1038, param_4);
        return;
      }
      if (c_var2 == 'u') {
        pass1_1018_0a76(((u_var5 + 0xf2) as u32), param_4);
        InvalidateRect16(0x1018,0x0,0x0);
        return;
      }
    }
  }
  return;
}


pub unsafe fn load_draw_op_1020_2ede(ctx: &mut AppContext, param_1: U32Ptr, param_2: u32, param_3: u16)
{
  let u_var1: u32;
  let ppc_var2: u32;
  let hvar3: HDC16;
  let i_var4: i16;
  let handle: HPEN16;
  let hvar5: HGDIOBJ16;
  let in_dx: U32Ptr;
  let i_var6: i16;
  let unaff_di: i16;
  let u_var7: u16;
  let unaff_ss: u16;
  let pu_var8: U32Ptr;
  let pa_var9: &mut Struct76;
  let u_var10: u32;
  let init_data: DEVMODEA;
  
  get_sys_metrics_1020_7c1a(param_1, param_2, param_3 as i16);
 // u_var7 = (param_1 >> 0x10);
  i_var6 = param_1;
  (i_var6 + 0x14) = 0x0;
  (i_var6 + 0x18) = 0x0;
  (i_var6 + 0x1a) = 0x0;
  (i_var6 + 0x1c) = 0x0;
  (i_var6 + 0x1e) = 0x0;
  (i_var6 + 0x20) = 0x0;
  *param_1 = 0x363c;
  (i_var6 + 0x2) = 0x1020;
  pu_var8 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, (param_2 + 0xfc), unaff_ss,
                            in_dx, unaff_di);
  (i_var6 + 0x14) = pu_var8;
  (i_var6 + 0x16) = (pu_var8 >> 0x10);
  u_var1 = (i_var6 + 0x14) as u32;
  ppc_var2 = ((i_var6 + 0x14) + 0x4) as u32;
  (**ppc_var2)(0x1010, u_var1, (u_var1 >> 0x10), 0x0, param_1);
  init_data = 0x0;
  pa_var9 = pass1_1018_0a50(((i_var6 + 0x14) as u32));
  u_var10 = pass1_1008_4772(pa_var9);
  hvar3 = CreateDC16(0x1008, u_var10, (u_var10 >> 0x10), init_data);
  *(i_var6 + 0x18) = hvar3;
  i_var4 = i_var6 + 0x18;
  ppc_var2 = (pa_var9 + 0x8);
  (**ppc_var2)();
  (i_var6 + 0x20) = i_var4;
  u_var1 = (i_var6 + 0x14) as u32;
  u_var1 = (u_var1 + 0x64);
  handle = CreatePen16(s_tile2_bmp_1050_1538, u_var1 as i16,
                       (u_var1 >> 0x10));
 (i_var6 + 0x1a) = handle as i16;
  hvar5 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
  (i_var6 + 0x1c) = hvar5 as i16;
  hvar5 = GetStockObject16(s_tile2_bmp_1050_1538);
  hvar5 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar5);
  (i_var6 + 0x1e) = hvar5 as i16;
  return;
}


pub fn invalidate_rect_1020_3080(param_1: u32,param_2: i16,param_3: HWND16)
{
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if ((param_2 != 0x4) && (param_2 != 0x6)) {
    return;
  }
  InvalidateRect16(param_3,0x0,0x0);
  return;
}



pub fn draw_op_1020_30be(param_1: u32,param_2: HWND16,param_3: u16)
{
  let ppc_var1: u32;
  let u_var2: u32;
  let bvar3: bool;
  let i_var4: i16;
  let u_var5: u16;
  let hwnd: HWND16;
  let u_var6: u16;
  let u_var7: u16;
  let local_3c: u32;
  let i_stack56: i16;
  let i_stack54: i16;
  let i_stack52: i16;
  let i_stack50: i16;
  let local_30: RECT16;
  let i_stack44: i16;
  let i_stack42: i16;
  let p_rstack40: *mut RECT16;
  let i_stack38: i16;
  let local_24: HDC16;
  let local_22: PAINTSTRUCT16;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1 as i16;
  u_var7 = (i_var4 + 0x4) as u16;
  local_24 = BeginPaint16(param_2,&local_22);
  u_var6 = (i_var4 + 0x4) as u16;
  hwnd = ctx.s_tile2_bmp_1050_1538;
  bvar3 = IsIconic16(s_tile2_bmp_1050_1538);
  if (bvar3 == 0x0) {
    hwnd = 0x1018;
    local_3c = pass1_1018_0a50(((i_var4 + 0x14) as u32));
    ppc_var1 = (*local_3c + 0x8);
    (**ppc_var1)(0x1018, local_3c, (local_3c >> 0x10), &local_24, param_3,
                 u_var6, u_var7);
    u_var2 = (i_var4 + 0x14) as u32;
    if ((u_var2 + 0x84) == 0x1) {
      unk_draw_op_1020_320e(param_1,local_24,param_3);
    }
    ppc_var1 = (*local_3c + 0x4);
    (**ppc_var1)(0x1018, local_3c, (local_3c >> 0x10), 0x0, 0x0, 0xdc, param_3);
    u_var2 = (i_var4 + 0x14) as u32;
    if ((u_var2 + 0x84) != 0x1) {
      unk_draw_op_1020_320e(param_1,local_24,param_3);
    }
    draw_op_1020_3488(param_1 as i32);
    ppc_var1 = (*local_3c + 0xc);
    (**ppc_var1)(0x1018, local_3c, (local_3c >> 0x10), &local_24, param_3);
  }
  else {
    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
      ppc_var1 = ((i_var4 + 0x14) + 0x2c) as u32;
      i_stack38 = (**ppc_var1)(ctx.s_tile2_bmp_1050_1538);
      if (i_stack38 != 0x0) {
        p_rstack40 = GetStockObject16(s_tile2_bmp_1050_1538);
        GetClientRect16(s_tile2_bmp_1050_1538,&local_30);
        local_3c = 0x0;
        i_stack56 = (i_stack44 - local_30.x) + -0x1;
        i_stack54 = (i_stack42 - local_30.y) + -0x1;
        i_stack52 = i_stack54;
        i_stack50 = i_stack56;
        FillRect16(ctx.s_tile2_bmp_1050_1538, p_rstack40, &local_3c);
        hwnd = ctx.s_tile2_bmp_1050_1538;
        DrawIcon16(ctx.s_tile2_bmp_1050_1538, i_stack38, 0x2, 0x2);
      }
    }
  }
  EndPaint16(hwnd,&local_22);
  return;
}



pub fn unk_draw_op_1020_320e(param_1: u32,param_2: HDC16,param_3: u16)
{
  let pu_var1: u32;
  let ppc_var2: u32;
  let u_var3: u32;
  let i_var4: i16;
  let i_var5: i16;
  let u_var6: u16;
  let u_var7: u16;
  let u_var8: u32;
  let init_data: DEVMODEA;
  let local_c: i16;
  let local_a: u32;
  // HDC16 *pHStack6;
  let pHStack6: HDC16;
  let local_4: HDC16;
  
  local_4 = param_2;
 // u_var6 = (param_1 >> 0x10);
  i_var4 = param_1 as i16;
  u_var3 = (i_var4 + 0x14) as u32;
  if ((u_var3 + 0x84) == 0x1) {
    u_var3 = (i_var4 + 0x14) as u32;
   // u_var7 = (u_var3 >> 0x10);
    i_var5 = u_var3 as i16;
    pu_var1 = (i_var5 + 0x24) as u32;
    init_data = 0x0;
    u_var8 = pass1_1008_4772(
                            (pu_var1 & 0xffff |
                            (i_var5 + 0x26) << 0x10));
    local_4 = CreateDC16(0x1008, u_var8, (u_var8 >> 0x10), init_data);
    pHStack6 = &local_4;
    ppc_var2 = (*pu_var1 + 0x8);
    (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, pu_var1, (pu_var1 >> 0x10),
                 pHStack6, param_3);
  }
  pass1_1018_0d9a(((i_var4 + 0x14) as u32), CONCAT22(param_3, &local_c),
                  CONCAT22(param_3,&local_a));
  u_var3 = (i_var4 + 0x14) as u32;
  draw_op_1020_33c0(param_1, (u_var3 + 0x6c), local_c, local_a, 0x1, local_4,
                    0x1018);
  pass1_1018_1054(((i_var4 + 0x14) as u32), CONCAT22(param_3, &local_c),
                  CONCAT22(param_3,&local_a), param_3);
  u_var3 = (i_var4 + 0x14) as u32;
  draw_op_1020_33c0(param_1, (u_var3 + 0x74), local_c, local_a, 0x2, local_4,
                    0x1018);
  pass1_1018_1320(((i_var4 + 0x14) as u32), CONCAT22(param_3, &local_c),
                  CONCAT22(param_3,&local_a));
  u_var3 = (i_var4 + 0x14) as u32;
  draw_op_1020_33c0(param_1, (u_var3 + 0x68), local_c, local_a, 0x1, local_4,
                    0x1018);
  pass1_1018_15f6(((i_var4 + 0x14) as u32), CONCAT22(param_3, &local_c),
                  CONCAT22(param_3,&local_a));
  if (local_c != 0x0) {
    u_var3 = (i_var4 + 0x14) as u32;
    draw_op_1020_33c0(param_1, (u_var3 + 0x70), local_c, local_a, 0x1, local_4,
                      0x1018);
  }
  pass1_1018_108c(((i_var4 + 0x14) as u32), CONCAT22(param_3, &local_c),
                  CONCAT22(param_3,&local_a), param_3);
  if (local_c != 0x0) {
    u_var3 = (i_var4 + 0x14) as u32;
    draw_op_1020_33c0(param_1, (u_var3 + 0x78), local_c, local_a, 0x0, local_4,
                      0x1018);
  }
  u_var3 = (i_var4 + 0x14) as u32;
  if ((u_var3 + 0x84) == 0x1) {
    SelectPalette16(0x1018,0x0,pHStack6);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteDC16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}



pub fn
draw_op_1020_33c0(param_1: u32,param_2: u32,param_3: i16,param_4: u32,param_5: i16,
                 param_6: u16,param_7: u16)

{
  let pen_handle: HPEN16;
  let object_handle: HGDIOBJ16;
  let brush_handle: HBRUSH16;
  let obj_handle_2: HGDIOBJ16;
  let i_var1: i16;
  let u_var2: u16;
  let in_dx: u16;
  let u_var3: u16;
  let hdc: HDC16;
  let unaff_ss: u16;
  let u_var4: u16;
  let i_stack20: i16;
  let pu_stack14: U32Ptr;
  
  if (param_3 != 0x0) {
    pen_handle = CreatePen16(param_7 as i16, param_2 as i16, (param_2 >> 0x10));
    object_handle = SelectObject16(ctx.s_tile2_bmp_1050_1538,pen_handle);
    brush_handle = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    hdc = ctx.s_tile2_bmp_1050_1538;
    obj_handle_2 = SelectObject16(ctx.s_tile2_bmp_1050_1538,brush_handle);
    pu_stack14 = param_4;
// TODO: refactor for loop
//     for (i_stack20 = 0x0; i_stack20 < param_3; i_stack20 += 0x1) {
//       u_var4 = (param_1 >> 0x10);
//       i_var1 = param_3;
//       pass1_1020_3540(param_1,u_var4,param_5,pu_stack14,in_dx,unaff_ss);
//       if (param_5 < 0x1) {
//         u_var2 = 0x3;
//       }
//       else {
//         u_var2 = 0x4;
//       }
//       u_var3 = in_dx;
//       draw_polygon_1020_3602(param_1,u_var4,CONCAT22(i_var1,u_var2),hdc);
//       hdc = 0x1000;
//       fn_ptr_1000_17ce(CONCAT22(in_dx,i_var1),0x1000);
//       pu_stack14 =
//                   (pu_stack14 & 0xffff0000 | (pu_stack14 + 0x6));
//       in_dx = u_var3;
//     }
    SelectObject16(hdc,obj_handle_2);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538,object_handle);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub fn draw_op_1020_3488(param_1: i32)
{
  let u_var1: u16;
  let u_var2: u32;
  let u_var3: u32;
  let handle: HPEN16;
  let handle_00: HGDIOBJ16;
  let hvar4: HGDIOBJ16;
  let u_var5: u16;
  let unaff_ss: u16;
  let bottom: i16;
  let local_a: u32;
  let pu_stack6: U32Ptr;
  
 // u_var5 = (param_1 >> 0x10);
  u_var2 = (param_1 + 0x14) as u32;
  pu_stack6 = (u_var2 & 0xffff0000 | (u_var2 + 0x36));
  pass1_1008_3e94(pu_stack6, CONCAT22(unaff_ss, &local_a),
                  CONCAT22(unaff_ss, (&local_a + 0x2) as u16));
  u_var2 = (local_a._2_2_ - 0x3) << 0x10;
  if ((local_a._2_2_ - 0x3) < 0x0) {
    u_var2 = 0x0;
  }
  u_var1 = (local_a - 0x3) as u16;
  local_a = u_var1 as u32;
  if (u_var1 < 0x0) {
    local_a = 0x0;
  }
  local_a = u_var2 | local_a;
  u_var3 = (param_1 + 0x14) as u32;
  u_var3 = (u_var3 + 0x64);
  handle = CreatePen16(0x1008, u_var3 as i16, (u_var3 >> 0x10));
  handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  hvar4 = GetStockObject16(s_tile2_bmp_1050_1538);
  hvar4 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar4);
 // bottom = (local_a >> 0x10);
  Rectangle16(ctx.s_tile2_bmp_1050_1538, (local_a + 0x6) as i16, bottom + 0x6, local_a as i16,
              bottom);
  SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_00);
  SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar4);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  return;
}


pub fn draw_polygon_1020_3602(param_1: u16,param_2: u16,param_3: u32,param_4: HDC16)
{
  Polygon16(param_4,param_3,((param_3 >> 0x10) as i16));
  return;
}


pub unsafe fn unk_draw_op_1020_3da4(param_1: &mut Struct24, param_2: i32)
{
  let pu_var1: u32;
  let ppc_var2: u32;
  let u_var3: u32;
  let i_var4: i16;
  let hvar5: HGDIOBJ16;
  let p_hvar6: HDC16;
  let in_dx: U32Ptr;
  let u_var7: u16;
  let i_var8: i16;
  let unaff_di: i16;
  let u_var9: u16;
  let unaff_cs: u16;
  let unaff_ss: u16;
  let pu_var10: U32Ptr;
  let local_4: HDC16;
  let i_var9: &mut Struct24;
  let u_var8: &mut Struct24;
  
  get_sys_metrics_1020_7c1a(param_1, param_2 as u32, unaff_cs as i16);
 // u_var9 = (param_1 >> 0x10);
  i_var8 = param_1;
  (i_var8 + 0x14) = 0x0;
  param_1.field_0x0 = 0x408a;
  (i_var8 + 0x2) = 0x1020;
  pu_var10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x6, unaff_ss, in_dx, unaff_di);
 // u_var7 = (pu_var10 >> 0x10);
  (i_var8 + 0x14) = pu_var10;
  (i_var8 + 0x16) = u_var7 as i16;
  ppc_var2 = ((i_var8 + 0x14) + 0x4) as u32;
  (**ppc_var2)(0x1010, (i_var8 + 0x14), u_var7, 0x0, param_1);
  local_4 = GetDC16(0x1010);
  i_var4 = SetMapMode16(ctx.s_tile2_bmp_1050_1538, 0x1);
  (i_var8 + 0x1e) = i_var4;
  hvar5 = GetStockObject16(s_tile2_bmp_1050_1538);
  hvar5 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar5);
  (i_var8 + 0x18) = hvar5 as i16;
  hvar5 = GetStockObject16(s_tile2_bmp_1050_1538);
  hvar5 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar5);
  (i_var8 + 0x1a) = hvar5 as i16;
  u_var3 = (i_var8 + 0x14) as u32;
  pu_var1 = (u_var3 + 0x24);
  p_hvar6 = &local_4;
  ppc_var2 = (*pu_var1 + 0x8);
  (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, pu_var1, (pu_var1 >> 0x10), p_hvar6);
  (i_var8 + 0x1c) = p_hvar6 as i16;
  u_var3 = (i_var8 + 0x14) as u32;
  *(u_var3 + 0x4c) = local_4;
  return;
}


pub fn win_ui_palette_op_1020_3e84(param_1: &mut Struct16)
{
  let i_var1: &mut Struct16;
  let u_var1: u16;
  let unaff_ss: u16;
  
 // u_var1 = (param_1 >> 0x10);
  i_var1 = param_1;
  param_1 = 0x408a;
  i_var1.field_0x2 = 0x1020;
  pass1_1010_1ea6(i_var1.field_0x14, param_1 & 0xffff | u_var1 << 0x10,
                  unaff_ss);
  SelectObject16(0x1010, i_var1.field_0x18);
  SelectObject16(ctx.s_tile2_bmp_1050_1538, i_var1.field_0x1a);
  SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, i_var1.field_0x1c);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  SetMapMode16(ctx.s_tile2_bmp_1050_1538, i_var1.field_0x1e);
  param_1 = 0x3ab0;
  i_var1.field_0x2 = 0x1008;
  param_1 = 0x389a;
  i_var1.field_0x2 = 0x1008;
  return;
}


pub fn validate_rect_1020_3f12(param_1: u32,param_2: i16,param_3: HWND16)
{
  let local_a: RECT16;
  let u_stack6: u32;
  
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  local_a =0x8000e;
  u_stack6 = 0x1100116;
  InvalidateRect16(param_3,0x0,&local_a);
  local_a =0xf10000;
  u_stack6 = 0x1220030;
  ValidateRect16(s_tile2_bmp_1050_1538,&local_a);
  local_a =0xf100f5;
  u_stack6 = 0x1220127;
  ValidateRect16(s_tile2_bmp_1050_1538,&local_a);
  return;
}



pub fn mixed_draw_op_1020_3fa0(param_1: u32,param_2: HWND16,param_3: u16)
{
  let u_var1: u32;
  let ppc_var2: u32;
  let u_var3: u32;
  let i_var4: i16;
  let u_var5: u16;
  let u_var6: u16;
  let i_stack56: i16;
  let u_stack54: u32;
  let local_32: u32;
  let i_stack46: i16;
  let u_stack44: u32;
  let pu_stack40: u32;
  let local_24: u16;
  let local_22: PAINTSTRUCT16;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1 as i16;
  u_var6 = (i_var4 + 0x4) as u16;
  BeginPaint16(param_2,&local_22);
  u_var3 = (i_var4 + 0x14) as u32;
  local_24 = (u_var3 + 0x4c) as u16;
  u_var3 = (i_var4 + 0x14) as u32;
  pu_stack40 = (u_var3 + 0x24);
  ppc_var2 = (*pu_stack40 + 0x4);
  (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, pu_stack40, (pu_stack40 >> 0x10),
               0x0, &local_24, param_3, u_var6);
  u_var3 = (i_var4 + 0x14) as u32;
  i_stack46 = (u_var3 + 0x44) as i16;
  u_var3 = (i_var4 + 0x14) as u32;
  u_stack44 = (u_var3 + 0x40);
  u_var1 = (i_var4 + 0x14) as u32;
  pass1_1008_3e94((u_var1 & 0xffff0000 | (u_var1 + 0x3a)),
                  CONCAT22(param_3,&local_32),
                  CONCAT22(param_3, (&local_32 + 0x2) as u16));
  u_stack54 = u_stack44;
    // TODO: refactor for loop
  // for (i_stack56 = 0x0; i_stack56 < i_stack46; i_stack56 += 0x1) {
  //   draw_rect_1020_40ce(u_stack54,local_32,(local_32 >> 0x10),param_3);
  //   u_stack54 = u_stack54 & 0xffff0000 | (u_stack54 + 0x18);
  // }
  EndPaint16(0x1008,&local_22);
  return;
}



pub fn  pass1_1020_4064(param_1: &mut Struct18,param_2: u8)

{
  win_ui_palette_op_1020_3e84(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(ctx, param_1, 0x1000);
  }
  return param_1;
}


pub fn draw_rect_1020_40ce(param_1: u32,param_2: i16,param_3: i16,param_4: u16)
{
  let i_var1: i16;
  let hvar2: HGDIOBJ16;
  let handle: HPEN16;
  let local_6: i16;
  let local_4: i16;
  
  pass1_1008_3e94((param_1 & 0xffff0000 | (param_1 + 0x10)),
                  CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_4));
  pass1_1008_3e94(param_1,CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_4));
  i_var1 = (param_1 + 0xa) as i16;
  Ellipse16(0x1008, i_var1 + local_6 + param_2, i_var1 + local_4 + param_3,
            (local_6 - (param_1 + 0xa)) + param_2,
            (local_4 - (param_1 + 0xa)) + param_3);
  if (((param_1 + 0xe) & 0x1) != 0x0) {
    hvar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar2);
    handle = CreatePen16(s_tile2_bmp_1050_1538,0xf9,0x100);
    SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
    Rectangle16(ctx.s_tile2_bmp_1050_1538,local_6 + param_2 + 0x5,
                local_4 + param_3 + 0x5,local_6 + param_2 + -0x5,local_4 + param_3 + -0x5)
    ;
    hvar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar2);
    hvar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, hvar2);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn unk_draw_op_1020_41c8(param_1: &mut Struct20, param_2: u16, param_3: u16, param_4: u16)
{
  let ppc_var1: u32;
  let hvar2: HCURSOR16;
  let pu_var3: U32Ptr;
  let extraout_dx: U32Ptr;
  let pu_var4: U32Ptr;
  let u_var6: u16;
  let u_var5: &mut Struct64;
  let unaff_di: i16;
  let u_var7: u16;
  let unaff_ss: u16;
  let pu_var8: U32Ptr;
  let pu_var9: U32Ptr;
  let pu_var10: U32Ptr;
  let pu_var11: U32Ptr;
  
  unk_draw_op_1020_7f7a(param_1, 0x8, CONCAT22(param_3, param_2) as i32);
 // u_var7 = (param_1 >> 0x10);
  u_var5 = param_1;
  u_var5.field_0xee = 0x0;
  u_var5.field_0xf0 = 0x0;
  u_var5.field_0xf2 = 0x0;
  u_var5.field_0xf4 = 0x1;
  u_var5.field_0xf6 = 0x0;
  u_var5.field_0xfa = 0x0;
  u_var5.field_0xfe = 0x0;
  u_var5.field_0x102 = 0x0;
  u_var5.field_0x106 = 0x0;
  u_var5.field_0x10a = 0x0;
  u_var5.field_0x108 = 0x0;
  u_var5.field_0x10c = 0x0;
  u_var5.field_0x110 = 0x0;
  u_var5.field_0x10e = 0x0;
  u_var5.field_0x112 = 0x0;
  u_var5.field_0x114 = 0x0;
  u_var5.field_0x116 = 0x0;
  param_1.field_0x0 = 0x623c;
  u_var5.field_0x2 = 0x1020;
  u_var5.field_0xe2 = 0x62d8;
  u_var5.field_0xe4 = 0x1020;
  pu_var4 = extraout_dx;
  pu_var11 = ctx.PTR_LOOP_1050_038c;
  hvar2 = LoadCursor16(param_4, (s__s__ld_1050_019c + 0x2));
  u_var5.field_0xf0 = hvar2;
  pu_var10 = ctx.PTR_LOOP_1050_038c;
  hvar2 = LoadCursor16(s_tile2_bmp_1050_1538,
                       (s__s__ld_1050_019c + 0x3));
  u_var5.field_0xf2 = hvar2;
  pu_var9 = ctx.PTR_LOOP_1050_038c;
  ctx.PTR_LOOP_1050_0398 =
       
       LoadAccelerators16(s_tile2_bmp_1050_1538,s_OpAccel_1050_43e8);
  pu_var8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x29, unaff_ss, pu_var4, unaff_di);
  &u_var5.field_0xfa = pu_var8;
  (&u_var5.field_0xfa + 0x2) = (pu_var8 >> 0x10);
  if (param_1 == 0x0) {
    pu_var3 = 0x0;
    u_var6 = 0x0;
  }
  else {
    pu_var3 = &u_var5.field_0xe2;
    u_var6 = u_var7;
  }
  ppc_var1 = (*u_var5.field_0xfa + 0x4);
  (**ppc_var1)(0x1010, u_var5.field_0xfa, 0x0, pu_var3, u_var6, pu_var9, pu_var10, pu_var11);
  u_var5.field_0xe6 = u_var5.field_0xfa;
  return;
}


pub unsafe fn set_cursor_1020_5764(param_1: u32, param_2: i16, param_3: u16)
{
  let u_var1: u16;
  let u_var2: u32;
  let in_dx: U32Ptr;
  let i_var3: i16;
  let unaff_di: i16;
  let u_var4: u16;
  let h_instance: HINSTANCE16;
  let hcursor: HCURSOR16;
  let local_e: i16;
  let local_c: [u8;2];
  let u_stack10: u32;
  let pu_stack6: U32Ptr;
  
  pu_stack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2f, param_3, in_dx, unaff_di);
 // u_var4 = (pu_stack6 >> 0x10);
  u_stack10 = (pu_stack6 + 0x20);
  u_var1 = (pu_stack6 + 0x22);
  if ((u_var1 | u_stack10) != 0x0) {
    h_instance = 0x1030;
    pass1_1030_8308(ctx.PTR__LOOP_1050_5748,
                    (ctx.PTR__LOOP_1050_5748 >> 0x10),
                    CONCAT22(param_3,&local_e),

                    CONCAT13((param_3 >> 0x8),CONCAT12(param_3 as u8, local_c)),
                    u_stack10 & 0xffff | u_var1 << 0x10, &local_e, u_var1);
    if (param_2 <= local_e) {
     // u_var4 = (param_1 >> 0x10);
      i_var3 = param_1 as i16;
      if ((i_var3 + 0xf4) != 0x1) {
        SetCursor16(0x1030);
        (i_var3 + 0xee) = 0x0;
        (i_var3 + 0xf4) = 0x1;
        (i_var3 + 0x10c) = 0x0;
        h_instance = s_tile2_bmp_1050_1538;
        ReleaseCapture16();
      }
      LoadCursor16(h_instance,0x7f02);
      SetCursor16(ctx.s_tile2_bmp_1050_1538);
      hcursor = 0x1018;
      pass1_1018_017c(pu_stack6, param_2 as u16, param_3);
      u_var2 = (i_var3 + 0xf6) as u32;
      (u_var2 + 0x10) = 0x1;
      if ((i_var3 + 0xfe) != 0x0) {
        pass1_1020_68de(((i_var3 + 0xfe) as u32), 0x1018);
        hcursor = ctx.s_tile2_bmp_1050_1538;
        PostMessage16(0x1018,0x0,0x0,0x11100eb);
      }
      SetCursor16(hcursor);
    }
  }
  return;
}



pub fn pt_in_rect_1020_5856(param_1: u32,param_2: &POINT16,param_3: u16)
{
  let pu_var1: u32;
  let bvar2: bool;
  let u_var3: u32;
  let in_dx: u16;
  let extraout_dx: u16;
  let u_stack10: u32;
  
  pass1_1018_2862((param_1 + 0xfa));
  if ((in_dx | param_3) != 0x0) {
    u_stack10 = 0x0;
    loop {
      pu_var1 = (param_3 + 0xa) as u32;
      if (*pu_var1 < u_stack10 || *pu_var1 == u_stack10) { break; }
      u_var3 = u_stack10;
      empty_1008_8fc4(param_3, in_dx, u_stack10, (u_stack10 >> 0x10));
      if ((extraout_dx | u_var3) != 0x0) {
        bvar2 = PtInRect16(0x1008, *param_2);
        if (bvar2 != 0x0) {
          return;
        }
      }
      u_stack10 += 0x1;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn
pt_in_rect_op_1020_58ce
          (param_1: u32,param_2: u16,param_3: u16,param_4: u8,param_5: &RECT16,
          param_6: u16)

{
  let ppc_var1: u32;
  let u_var2: u32;
  let u_var3: u16;
  let bvar4: bool;
  let msg: U32Ptr;
  let in_dx: U32Ptr;
  let u_var5: u16;
  let pu_var6: U32Ptr;
  let i_var7: i16;
  let i_var8: i16;
  let unaff_di: i16;
  let u_var9: u16;
  let u_var10: u16;
  let rect: *mut RECT16;
  let rect_00: *mut RECT16;
  let u_var11: u32;
  let pu_var12: U32Ptr;
  let local_34: [u8;6];
  let u_stack46: u32;
  let pu_stack38: U32Ptr;
  let u_stack30: u32;
  let pu_stack26: U32Ptr;
  let local_18: [u16;0x2];
  let u_stack20: u16;
  let u_stack18: u32;
  let u_stack14: u16;
  let pu_stack12: U32Ptr;
  let u_stack10: u16;
  let u_stack8: u16;
  let local_6: u16;
  let u_stack4: u16;
  
  local_6 = param_3;
  u_stack4 = param_2;
  u_stack8 = (param_4 & 0x8) as u16;
  u_stack10 = (param_4 & 0x4) as u16;
 // u_var9 = (param_1 >> 0x10);
  i_var7 = param_1 as i16;
  u_var3 = pass1_1020_64d4(((i_var7 + 0xf6) as u32), 0x2);
  u_stack30._2_2_ = in_dx;
  rect = param_5;
  if (u_var3 == 0x0) {
//LAB_1020_5942:
    u_var3 = pass1_1020_64d4(((i_var7 + 0xf6) as u32), 0x4);
    rect_00 = rect;
    if (u_var3 == 0x0) {
//LAB_1020_5a16:
      u_var3 = pass1_1020_64d4(((i_var7 + 0xf6) as u32), 0x1);
      if (u_var3 != 0x0) {
        u_stack30 = pass1_1020_6498(((i_var7 + 0xf6) as u32), 0x1);
        u_stack30._2_2_ = (u_stack30 >> 0x10);
// TODO: refactor for loop
        // for (pu_stack26 = 0x0; pu_stack26 < 0x4;
        //     pu_stack26 = (pu_stack26 + 0x1)) {
        //   BVar4 = PtInRect16(rect_00,(POINT16)CONCAT22(u_stack4,local_6));
        //   if (BVar4 != 0x0) {
        //     local_18[0] = 0x0;
        //     u_stack20 = 0x0;
        //     if (pu_stack26 == 0x0) {
        //       u_stack20 = (-(u_stack10 == 0x0) & 0x4) - 0x5;
        //     }
        //     else {
        //       if (pu_stack26 == (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
        //         u_stack20 = (-(u_stack10 == 0x0) & 0xfffc) + 0x5;
        //       }
        //       else {
        //         if (pu_stack26 == &ctx.PTR_LOOP_1050_0002) {
        //           local_18[0] = (-(u_stack10 == 0x0) & 0x4) - 0x5;
        //         }
        //         else {
        //           if (pu_stack26 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
        //             local_18[0] = (-(u_stack10 == 0x0) & 0xfffc) + 0x5;
        //           }
        //         }
        //       }
        //     }
        //     pass1_1020_2a94((i_var7 + 0xce),CONCAT22(local_18[0],u_stack20),
        //                     param_6);
        //     return;
        //   }
        //   rect_00 = s_tile2_bmp_1050_1538;
        // }
      }
      u_var3 = pass1_1020_64d4(((i_var7 + 0xf6) as u32), 0x3);
      if (u_var3 != 0x0) {
        u_stack30._0_2_ = &local_6;
        pt_in_rect_1020_5856
                  (param_1, CONCAT22(param_6, u_stack30 as u16),
                   u_stack30 as u16);
        u_var5 = u_stack30._2_2_ | u_stack30;
        if (u_var5 != 0x0) {
          pu_stack26 = (u_stack30)[0x17];
          if (((u_stack8 == 0x0) || (u_stack10 == 0x0)) && (u_stack10 == 0x0)) {
            local_18[0] = 0x1;
          }
          else {
            local_18[0] = 0x2;
          }
          u_stack20 = (u_stack30)[0x6];
          u_stack18 = CONCAT22(u_stack18._2_2_, (u_stack30)[0x7]);
          if ((pu_stack26 == 0xb) || (pu_stack26 == 0x37)) {
            u_var2 = (i_var7 + 0xfa) as u32;
           // u_var10 = (u_var2 >> 0x10);
            i_var8 = u_var2 as i16;
            u_stack46 = (i_var8 + 0x20) as u32;
            u_var5 = (i_var8 + 0x22) as u16;
            if ((u_var5 | u_stack46) != 0x0) {
              pu_var12 = clear_struct_1008_3e38(CONCAT22(param_6, local_34));
             // pu_var6 = (pu_var12 >> 0x10);
              pass1_1018_161c(param_6, u_stack46, CONCAT22(param_6, local_34),
                              u_stack18 as i16, u_stack20 as i16);
              pu_stack38 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2f, param_6, pu_var6, unaff_di
                                         );
             // u_var5 = (pu_stack38 >> 0x10);
              pass1_1010_ecc6(pu_stack38, CONCAT22(param_6, local_34),
                              ((u_stack46 + 0x3c) as i32), local_34, u_var5,
                              param_6);
            }
          }
          u_var3 = pass1_1018_25d2(((i_var7 + 0xfa) as u32), local_18[0],
                                   u_stack18 & 0xffff | u_stack20 << 0x10,
                                   unaff_di, param_6);
          if (u_var3 != 0x0) {
            return;
          }
          u_var3 = pass1_1020_5d56(param_1,
                                   CONCAT22(u_stack30._2_2_, u_stack30 as u16), u_var5,
                                   unaff_di, param_6);
          if (u_var3 != 0x0) {
            return;
          }
        }
      }
      return;
    }
    u_var11 = pass1_1020_6498(((i_var7 + 0xf6) as u32), 0x4);
    u_stack30._2_2_ = (u_var11 >> 0x10);
    u_var10 = u_var11 as u16;
    rect_00 = s_tile2_bmp_1050_1538;
    pu_var6 = u_stack30._2_2_;
    u_stack14 = u_var10;
    pu_stack12 = u_stack30._2_2_;
    BVar4 = PtInRect16(rect,CONCAT22(u_stack4, local_6));
    if (BVar4 == 0x0) {
        // goto
        // LAB_1020_5a16;
    }
    rect = 0x1010;
    u_stack18 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_6, u_stack30._2_2_, unaff_di);
    if ((u_stack18 + 0x72) != 0x0) {
      (i_var7 + 0x116) = 0x1;
      if (param_1 == 0x0) {
        i_var7 = 0x0;
        u_var9 = 0x0;
      }
      else {
        i_var7 += 0xe2;
      }
      u_stack30 = CONCAT22(u_var9, i_var7 as u16);
      ppc_var1 = (*_PTR_LOOP_1050_02a0 + 0x4);
      (**ppc_var1)(0x1010, _PTR_LOOP_1050_02a0,
                   (ctx.PTR__LOOP_1050_02a0 >> 0x10), 0x10, i_var7, u_var9, u_var10, pu_var6
                 );
      pu_var12 = pass1_1008_941a(CONCAT22(param_6, local_18), 0x1, 0x86);
      msg = local_18;
      rect = 0x1008;
      win_1008_5c9e(ctx.PTR__LOOP_1050_02a0, CONCAT22(param_6,msg), msg,
                    (pu_var12 >> 0x10), param_6);
      if (msg != 0x0) {
        return;
      }
      u_var9 = 0xf6;
      pu_stack26 = msg;
//       TODO: goto LAB_1020_5936;
    }
    u_var9 = 0xf6;
  }
  else {
    u_var11 = pass1_1020_6498(((i_var7 + 0xf6) as u32), 0x2);
    u_stack30._2_2_ = (u_var11 >> 0x10);
    u_stack14 = u_var11 as u16;
    rect = s_tile2_bmp_1050_1538;
    pu_stack12 = u_stack30._2_2_;
    BVar4 = PtInRect16(param_5,CONCAT22(u_stack4, local_6));
    if (BVar4 == 0x0) {
        // goto
        // LAB_1020_5942;
    }
    u_var9 = 0x68;
  }
  msg = 0x0;
//LAB_1020_5936:
  PostMessage16(rect, msg, msg, CONCAT22(0x111, u_var9) as i32);
  return;
}


pub fn mix_draw_op_1020_650c(param_1: &mut Struct7,param_2: HWND16,param_3: u16)
{
  let ppc_var1: u32;
  let u_var2: u32;
  let i_var3: i16;
  let i_var4: i16;
  let i_var5: i16;
  let u_var6: u16;
  let u_var7: u16;
  let local_28: PAINTSTRUCT16;
  let i_stack8: i16;
  let pu_stack6: u32;
  
 // u_var6 = (param_1 >> 0x10);
  i_var3 = param_1;
  u_var2 = (i_var3 + 0x14) as u32;
  pu_stack6 = (u_var2 + 0xa);
  if (((i_var3 + 0x10) != 0x0) ||
     (u_var2 = (i_var3 + 0x14) as u32, (u_var2 + 0x24) != 0x0)) {
    draw_op_1020_9364(param_1, param_2, param_3 as i16);
    if ((i_var3 + 0x24) != 0x0) {
      u_var2 = (i_var3 + 0x24) as u32;
      ppc_var1 = ((i_var3 + 0x24) + 0x14) as u32;
      (**ppc_var1)(param_2, u_var2, (u_var2 >> 0x10));
    }
  }
  i_stack8 = 0x0;
  loop {
    i_var4 = i_var3 + 0x18;
    i_var5 = i_stack8 * 0x4;
    if ((i_var4 + i_var5) != 0x0) {
      u_var2 = (i_var4 + i_var5) as u32;
      ppc_var1 = ((i_var4 + i_var5) + 0x8) as u32;
      (**ppc_var1)(param_2, u_var2, (u_var2 >> 0x10), pu_stack6,
                   (pu_stack6 >> 0x10));
    }
    i_stack8 += 0x1;
    if i_stack8 >= 0x5 {
        break;
    }
  }
  u_var7 = (i_var3 + 0x4) as u16;
  BeginPaint16(param_2,&local_28);
  ppc_var1 = (*pu_stack6 + 0x4);
  (**ppc_var1)(ctx.s_tile2_bmp_1050_1538, pu_stack6, (pu_stack6 >> 0x10), 0x0,
               0x0, i_var3 + 0xa, u_var6, u_var7);
  EndPaint16(s_tile2_bmp_1050_1538,&local_28);
  return;
}


pub unsafe fn pt_in_rect_1020_68fc(param_1: U32Ptr, param_2: u16, param_3: u16)
{
  let ppc_var1: u32;
  let u_var2: u16;
  let bvar3: bool;
  let u_var4: u32;
  let u_var5: u16;
  let pstack6: i16;
  
  pstack6 = CONCAT22(param_2, param_3) as i16;
 // u_var5 = (param_1 >> 0x10);
  u_var2 = pass1_1018_31d0((param_1 + 0xf2));
  if (u_var2 != 0x0) {
    u_var4 = (param_1 + 0xf2);
    u_var4 = u_var4 & 0xffff0000 | (u_var4 + 0x16c);
    bvar3 = PtInRect16(0x1018, pstack6);
    if (bvar3 != 0x0) {
      ppc_var1 = (*param_1 + 0x40);
      (**ppc_var1)(ctx.s_tile2_bmp_1050_1538, param_1, 0xef, u_var4);
    }
  }
  return;
}


pub fn  draw_op_1020_7070(param_1: i16,param_2: u16) -> HGDIOBJ16

{
  let hvar1: HGDIOBJ16;
  
  hvar1 = GetStockObject16(param_1);
  if (ctx.PTR__LOOP_1050_441e == 0x0) {
    ctx._PTR_LOOP_1050_441e = 0x1000002;
  }
  if (0x6 < param_2) {
    return 0x0;
  }
  SetTextColor16(ctx.s_tile2_bmp_1050_1538,_PTR_LOOP_1050_441e);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  return hvar1;
}


pub unsafe fn palette_op_1020_7270(param_1: U32Ptr, param_2: HDC16)
{
  let u_var1: u16;
  let u_var2: u16;
  let hvar3: HPALETTE16;
  let i_var4: i16;
  let u_var5: u16;
  let unaff_ss: u16;
  let pa_stack8: &mut Struct18;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1;
  *param_1 = 0x754c;
  (i_var4 + 0x2) = 0x1020;
  if ((i_var4 + 0x1c) != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6(((i_var4 + 0x1c) as u32),
                    param_1 & 0xffff | u_var5 << 0x10, unaff_ss);
  }
  u_var1 = (i_var4 + 0x14) as u16;
  u_var2 = (i_var4 + 0x16) as u16;
  pa_stack8 = CONCAT22(u_var2, u_var1);
  if ((u_var2 | u_var1) != 0x0) {
    pass1_1008_5118(CONCAT22(u_var2, u_var1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(ctx, pa_stack8, 0x1000);
  }
  hvar3 = SelectPalette16(param_2, 0x0, (i_var4 + 0x20));
  (i_var4 + 0x20) = hvar3 as i16;
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  *param_1 = 0x3ab0;
  (i_var4 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (i_var4 + 0x2) = 0x1008;
  return;
}


pub fn invalidate_rect_1020_735a(param_1: u32,param_2: HWND16)
{
  InvalidateRect16(param_2,0x0,(param_1 + 0x1c) + 0x16c
                  );
  return;
}


pub fn draw_op_1020_7cc8(param_1: i32,in_win_handle_2: HWND16,param_3: u16)
{
  let ppc_var1: u32;
  let rect: *mut RECT16;
  let color:COLORREF;
  let handle: HPEN16;
  let handle_00: HGDIOBJ16;
  let mut count: String; 
  let mut str: String;
  let pu_var2: u32;
  let in_dx: u16;
  let mut str_00: String; 
  let i_var4: &mut Struct6;
  let i_var3: i16;
  let u_var4: u16;
  let u_var5: u16;
  let dvar6: u32;
  let u_var7: u32;
  let u_var8: u32;
  let hbrush: HBRUSH16;
  let u_var9: u32;
  let hvar10: HDC16;
  let u_var11: u16;
  let i_stack66: i16;
  let local_20: u16;
  let i_stack30: i16;
  let i_stack28: i16;
  let i_stack26: i16;
  let i_stack24: i16;
  let i_stack22: i16;
  let local_rect_1: RECT16;
  let i_stack16: i16;
  let i_stack14: i16;
  let hstack12: HPALETTE16;
  let pa_stack10: &mut Struct13;
  let local_hdc_1: HDC16;
  let is_iconic: bool;
  
 // u_var4 = (param_1 >> 0x10);
  i_var4 = param_1;
  is_iconic = IsIconic16(in_win_handle_2);
  if ((is_iconic == 0x0) || (ctx.PTR_LOOP_1050_0010 != 0x0)) {
    local_hdc_1 = GetWindowDC16(s_tile2_bmp_1050_1538);
    pa_stack10 = (ctx.PTR__LOOP_1050_4230 + 0xe);
    hstack12 = palette_op_1008_4e08(pa_stack10, &local_hdc_1, in_dx, 0x1008);
    u_var11 = i_var4.field_0x4;
    GetWindowRect16(0x1008,&local_rect_1);
    i_stack28 = (i_stack16 - local_rect_1.x) + -0x1;
    i_stack24 = (i_stack14 - local_rect_1.y) + -0x1;
    local_20 = i_var4.field_0x10;
    i_stack30 = i_var4.field_0x12;
    i_stack26 = i_stack24;
    if (is_iconic == 0x0) {
      i_stack26 = i_var4.field_0xe - i_var4.field_0x12;
    }
    u_var9 = CONCAT22(param_3, &local_20);
    hbrush = 0x4;
    hvar10 = local_hdc_1;
    i_stack22 = i_stack28;
    rect = GetStockObject16(s_tile2_bmp_1050_1538);
    FillRect16(ctx.s_tile2_bmp_1050_1538,rect,hbrush);
    pu_var2 = i_var4.field_0x6;
   // u_var5 = (pu_var2 >> 0x10);
    i_var3 = pu_var2 as i16;
    pu_var2 = (i_var3 + 0xe0) as u32;
    ppc_var1 = (*pu_var2 + 0x24);
    (**ppc_var1)(ctx.s_tile2_bmp_1050_1538, pu_var2, (i_var3 + 0xe2), 0x0,
                 u_var9, hvar10, u_var11);
    color = (-(pu_var2 == 0x0) & 0x1e) + 0x25;
    handle = CreatePen16(s_tile2_bmp_1050_1538, color as i16, 0x100);
    handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
    MoveTo16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
    LineTo16(ctx.s_tile2_bmp_1050_1538, 0x0, i_stack22);
    LineTo16(ctx.s_tile2_bmp_1050_1538, i_stack24, i_stack22);
    u_var7 = (local_hdc_1 << 0x10) as u32;
    LineTo16(ctx.s_tile2_bmp_1050_1538, i_stack24, 0x0);
    u_var8 = u_var7 & 0xffff0000 | local_hdc_1;
    u_var7 = 0x0;
    count = LineTo16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
    if (is_iconic == 0x0) {
      i_var3 = i_var4.field_0xe - i_var4.field_0x12;
      u_var7 = (local_hdc_1 << 0x10) as u32;
      MoveTo16(ctx.s_tile2_bmp_1050_1538, i_var3, 0x0);
      u_var7 = u_var7 & 0xffff0000 | local_hdc_1;
      count = LineTo16(ctx.s_tile2_bmp_1050_1538, i_var3, i_stack22);
    }
    ppc_var1 = (*i_var4.field_0x6 + 0x18);
    (**ppc_var1)(ctx.s_tile2_bmp_1050_1538, i_var4.field_0x6, u_var7, u_var8);
    if (*count != '\0') {
      SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
      SetTextColor16(ctx.s_tile2_bmp_1050_1538,color);
      str = lstrlen16(ctx.s_tile2_bmp_1050_1538);
      dvar6 = GetTextExtent16(ctx.s_tile2_bmp_1050_1538, str, count);
     // i_var3 = (dvar6 >> 0x10);
      if (is_iconic == 0x0) {
        i_stack66 = (i_stack26 - i_stack30) / 0x2 - i_var3 / 0x2;
      }
      else {
        i_stack66 = i_stack24 / 0x2 - i_var3 / 0x2;
      }
      TextOut16(ctx.s_tile2_bmp_1050_1538, str, count, str_00, i_stack66 as usize);
    }
    hstack12 = SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, hstack12);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_00);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    ReleaseDC16(s_tile2_bmp_1050_1538,local_hdc_1);
  }
  return;
}


pub unsafe fn unk_draw_op_1020_7f7a(param_1: &mut Struct20, param_2: u16, param_3: i32)
{
  let u_var1: u16;
  let hvar2: HGDIOBJ16;
  let hvar3: HCURSOR16;
  let pu_var4: U32Ptr;
  let i_var4: &mut Struct20;
  let unaff_di: i16;
  let u_var5: u16;
  let unaff_ss: u16;
  let pa_var6: &mut Struct20;
  let pu_var7: U32Ptr;
  let in_stack_0000000e: u16;
  
  pa_var6 = unk_draw_op_1008_61b2
                     (param_1, param_2, param_3 as u16,
                      CONCAT22(in_stack_0000000e, param_3._2_2_) as u16, unaff_ss as i32);
 // pu_var4 = (pa_var6 >> 0x10);
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1;
  ((i_var4 + 0x1)).field_0x0 = 0x389a;
  i_var4[0x1].field_0x2 = 0x1008;
  ((i_var4 + 0x1)).field_0x0 = 0x3aa8;
  i_var4[0x1].field_0x2 = 0x1008;
  i_var4[0x1].field_0x4 = 0x0;
  i_var4[0x1].field_0x8 = 0x0;
  i_var4[0x1].field_0xa = 0x0;
  param_1.field_0x0 = 0x82bc;
  i_var4.field_0x2 = 0x1020;
  ((i_var4 + 0x1)).field_0x0 = 0x8358;
  i_var4[0x1].field_0x2 = 0x1020;
  string_1000_3d3e
            ((param_1 & 0xffff0000 | ZEXT24(&i_var4.field_0x5b)),
             s_VrMode_1050_4422);
  hvar2 = GetStockObject16(0x1000);
  i_var4.hgdiobj_field_0xc6 = hvar2;
  hvar3 = LoadCursor16(s_tile2_bmp_1050_1538, 0x7f00);
  i_var4.hcursor_field_0xc4 = hvar3;
  i_var4.field_0xc8 = 0x2028;
  i_var4.field_0xac = 0x47000000;
  i_var4.field_0xbc = (param_3._2_2_ + 0x8);
  pu_var7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, unaff_ss, pu_var4, unaff_di);
 // u_var1 = (pu_var7 >> 0x10);
  i_var4.field_0xb4 = 0x0;
  i_var4.field_0xb6 = 0x0;
  i_var4.field_0xb8 = (pu_var7 + 0xa);
  i_var4.field_0xba = (pu_var7 + 0xc);
  i_var4.field_0xca = param_3;
  win_ui_reg_class_1008_96d2(ctx, param_1, 0x1008, unaff_ss);
  return;
}


pub fn realize_palette_1020_8128(param_1: u32,param_2: i16,param_3: HGDIOBJ16,param_4: u16)
{
  let ppc_var1: u32;
  let u_var2: u32;
  let pu_var3: U32Ptr;
  let pu_var4: u32;
  let pu_var5: u32;
  let extraout_dx: u16;
  let i_var6: i16;
  let i_var7: i16;
  let u_var8: u16;
  let u_var9: u16;
  let local_12: [u8;8];
  let u_stack10: u16;
  let u_stack8: u16;
  let pu_stack6: u32;
  
  if (param_2 != 0x0) {
   // u_var8 = (param_1 >> 0x10);
    i_var6 = param_1 as i16;
    u_var2 = (i_var6 + 0xe6) as u32;
   // u_var9 = (u_var2 >> 0x10);
    i_var7 = u_var2 as i16;
    pu_var5 = (i_var7 + 0xa) as u32;
    ppc_var1 = (*pu_var5 + 0x18);
    pu_stack6 = pu_var5;
    (**ppc_var1)(param_3, pu_var5, (i_var7 + 0xc));
    u_stack8 = SUB42(pu_var5 as u16, 0x0) as u16;
    UnrealizeObject16(param_3);
    u_var2 = (i_var6 + 0xe6) as u32;
    u_var8 = (u_var2 + 0x14) as u16;
    u_stack10 = u_var8;
    RealizePalette16(ctx.s_tile2_bmp_1050_1538);
    pass1_1008_57a4(CONCAT22(param_4,local_12),
                    param_1 & 0xffff0000 | (i_var6 + 0xd2));
    loop {
      pu_var3 = local_12;
      pass1_1008_5b12(pu_var3, param_4);
      if ((extraout_dx | pu_var3) == 0x0) { break; }
      u_var9 = (pu_var3 + 0x6);
      pu_var4 = (pu_var3 + 0x4);
      ppc_var1 = (*pu_var4 + 0x90);
      (**ppc_var1)(0x1008, pu_var4, u_var9, 0x1, u_var8);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_palette_op_1020_81c0(param_1: HWND16)
{
  let in_struct_1: &mut Struct13;
  let b_force_background: bool;
  let b_force_background_00: HPALETTE16;
  let uvar1: u16;
  let u_var2: u16;
  let u_var3: u16;
  let u_stack6: u16;
  
  u_var3 = (ctx.PTR__LOOP_1050_4230 >> 0x10);
  in_struct_1 = (ctx.PTR__LOOP_1050_4230 + 0xe);
  u_var2 = (ctx.PTR__LOOP_1050_4230 + 0x10);
  u_stack6 = in_struct_1;
  if ((u_var2 | u_stack6) == 0x0) {
    return;
  }
  b_force_background = GetDC16(param_1);
  create_palette_1008_4e38(in_struct_1, 0x1008, u_var2 as u32);
  b_force_background_00 = SelectPalette16(0x1008,0x0,b_force_background);
  uvar1 = RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x1,b_force_background_00);
  RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  if (0x0 < uvar1) {
    InvalidateRect16(s_tile2_bmp_1050_1538,
                     (&ctx.PTR_LOOP_1050_0000 + 0x1),0x0);
  }
  return;
}


pub fn
invalidate_rect_1020_8d90
          (param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u16,
          param_6: u16)

{
  let u_var1: u32;
  let i_var2: i16;
  let u_var3: u16;
  let in_af: u8;
  let local_48: i16;
  let i_stack70: i16;
  let i_stack68: i16;
  let i_stack66: i16;
  let local_40: i16;
  let local_3e: i16;
  let u_stack60: u32;
  let local_38: [u8;28];
  ulocal_10: u8 [0xa];
  let u_stack6: u16;
  let u_stack4: u16;
  
 // u_var3 = (param_1 >> 0x10);
  i_var2 = param_1 as i16;
  u_stack6 = pass1_1018_266a(((i_var2 + 0x22) as u32));
  if (u_stack6 != 0x0) {
    pass1_1018_265c((i_var2 + 0x22));
    if ((param_5 | u_stack6) != 0x0) {
      u_stack4 = param_5;
      sys_1000_3f9c(local_10, param_6, s__03ld_1050_442a,
                    ctx.data_seg, u_stack6, &stack0xfffe, u_var3, 0x1000, param_6,
                    in_af);
      u_var1 = (i_var2 + 0x22) as u32;
      file_and_draw_op_1008_4f20
                (CONCAT22(param_6,local_38), (u_var1 + 0xe), 0x25,
                 CONCAT22(param_6,local_10), param_6);
      pass1_1008_4480(param_3, (param_1 & 0xffff0000 | (i_var2 + 0x1c)),
                      CONCAT22(param_6,local_38), param_6);
      u_stack60 = pass1_1008_4772(CONCAT22(param_6, local_38));
      pass1_1008_3e94((param_1 & 0xffff0000 | (i_var2 + 0x1c)),
                      CONCAT22(param_6,&local_40),
                      CONCAT22(param_6,&local_3e));
      local_48 = local_3e;
      i_stack70 = local_40;
     // u_var3 = (u_stack60 >> 0x10);
      i_stack68 = local_3e + (u_stack60 + 0x4);
      i_stack66 = local_40 + (u_stack60 + 0x8);
      InvalidateRect16(0x1008,0x0,&local_48);
      pass1_1008_41bc(CONCAT22(param_6,local_38));
    }
  }
  return;
}


pub fn invalidate_rect_1020_8fb4(param_1: u32,param_2: u16)
{
  let i_var1: i16;
  let u_var2: u32;
  let erase: u16;
  let u_var3: u32;
  let in_dx: u16;
  let extraout_dx: u16;
  let u_var4: u16;
  let i_var5: i16;
  let u_var6: u16;
  let unaff_ss: u16;
  let i_stack8: i16;
  
 // u_var6 = (param_1 >> 0x10);
  i_var5 = param_1 as i16;
  u_var2 = (i_var5 + 0xba) as u32;
  if ((u_var2 + 0x1e) != 0x0) {
    pass1_1018_2862(((i_var5 + 0x16) as u32));
    (i_var5 + 0xaa) = param_2 as i16;
    (i_var5 + 0xac) = in_dx as i16;
    if ((in_dx | (i_var5 + 0xaa)) != 0x0) {
      u_var2 = (i_var5 + 0xaa) as u32;
      i_var1 = (u_var2 + 0xa) as i16;
        // TODO: refactor for loop

        // for (i_stack8 = 0x0; i_stack8 < i_var1; i_stack8 += 0x1) {
      //   u_var3 = SEXT24(i_stack8);
      //   empty_1008_8fc4((i_var5 + 0xaa),u_var3);
      //   erase = u_var3;
      //   u_var4 = extraout_dx | erase;
      //   if (((u_var4 != 0x0) && (0x9 < (erase + 0x2e))) &&
      //      (pass1_1008_8b20(u_var3 & 0xffff | extraout_dx << 0x10,unaff_ss),
      //      (u_var4 | erase) != 0x0)) {
      //     InvalidateRect16(0x1008,0x0,erase);
      //   }
      // }
    }
  }
  return;
}


pub unsafe fn palette_op_1020_92c4(param_1: U32Ptr, param_2: HDC16)
{
  let i_var1: i16;
  let u_var2: u16;
  
 // u_var2 = (param_1 >> 0x10);
  i_var1 = param_1;
  *param_1 = 0x96c8;
  (i_var1 + 0x2) = 0x1020;
  if ((i_var1 + 0x12) != 0x0) {
    SelectPalette16(param_2,0x0,(i_var1 + 0x12));
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  *param_1 = 0x3ab0;
  (i_var1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (i_var1 + 0x2) = 0x1008;
  return;
}



pub fn mix_draw_op_1020_9312(param_1: u32,param_2: HWND16)
{
  let pu_var1: u32;
  let ppc_var2: u32;
  let u_var3: u32;
  let i_var4: i16;
  let u_var5: u16;
  let u_var6: u16;
  let local_22: PAINTSTRUCT16;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1 as i16;
  u_var6 = (i_var4 + 0x4) as u16;
  BeginPaint16(param_2,&local_22);
  u_var3 = (i_var4 + 0x6) as u32;
  pu_var1 = (u_var3 + 0xa);
  ppc_var2 = (*pu_var1 + 0x4);
  (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, pu_var1, (pu_var1 >> 0x10), 0x0,
               param_1 & 0xffff0000 | (i_var4 + 0xa), u_var6);
  EndPaint16(s_tile2_bmp_1050_1538,&local_22);
  return;
}


pub fn draw_op_1020_9364(param_1: &mut Struct7,in_win_handle_2: HWND16,param_3: i16)
{
  let pi_var1: U32Ptr;
  let u_var2: u16;
  let i_var3: i16;
  let u_var4: u32;
  let i_var5: i16;
  let p_rvar6: *mut RECT16;
  let local_struct_1: &mut Struct7;
  let var7: u16;
  let u_var7: u16;
  let i_stack62: i16;
  let u_stack58: u16;
  let local_38: [u8;4];
  let hstack52: HGDIOBJ16;
  let hstack50: HPEN16;
  let u_stack48: u16;
  let u_stack46: u32;
  let u_stack42: u32;
  let u_stack38: u32;
  let u_stack34: u32;
  let u_stack30: u32;
  let pu_stack26: U32Ptr;
  let i_stack22: i16;
  let i_stack20: i16;
  let local_12: u32;
  let u_stack14: u32;
  let local_a: RECT16;
  let u_stack6: u32;
  
 // var7 = (param_1 >> 0x10);
  local_struct_1 = param_1;
  GetClientRect16(in_win_handle_2,&local_a);
  local_12 = local_a;
  u_stack14 = u_stack6;
  i_stack20 = ctx.DAT_1050_4216;
  i_stack22 = ctx.DAT_1050_422c;
  pu_stack26 = ctx._PTR_PTR_DAT_1050_0009_1050_4172_1050_4212;
  u_stack30 = ctx._PTR_PTR_1050_4218;
  u_stack34 = ctx._PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
  u_stack38 = ctx._PTR_PTR_DAT_1050_0041_1050_4202_1050_4220;
  u_stack42 = ctx._PTR_DAT_1050_419a_1050_4224;
  u_stack46 = ctx._PTR_PTR_1050_4228;
  u_var4 = local_struct_1.field_0x6;
  u_stack48 = (u_var4 + 0x12) as u16;
  u_stack58 = 0x9;
  loop {
    u_var4 = (u_stack58 * 0x4 + u_stack34) as u32;
    hstack50 = CreatePen16(s_tile2_bmp_1050_1538, u_var4 as i16,
                           (u_var4 >> 0x10));
    hstack52 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hstack50);
    MoveToEx16(ctx.s_tile2_bmp_1050_1538,local_38,param_3,
               (u_stack58 * 0x2 + pu_stack26));
    LineTo16(ctx.s_tile2_bmp_1050_1538, (pu_stack26 + u_stack58 * 0x2),
             u_stack6 as i16);
    i_var3 = (i_stack20 - u_stack58) * 0x2;
    MoveToEx16(ctx.s_tile2_bmp_1050_1538,local_38,param_3,
               (i_var3 + pu_stack26));
    LineTo16(ctx.s_tile2_bmp_1050_1538, (pu_stack26 + i_var3),
             u_stack6 as i16);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, hstack52);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    u_stack58 -= 0x1;
    if u_stack58 >= 0x8000 { break; }
  }
  p_rvar6 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
 // u_var7 = (pu_stack26 >> 0x10);
  local_a = CONCAT22((pu_stack26 + 0x12) + 0x1, local_a.x as u16);
  u_var2 = (pu_stack26 + 0x14);
  u_stack14 = u_stack14 & 0xffff | u_var2 << 0x10;
  u_stack6 = CONCAT22(u_var2, u_stack6 as u16);
  FillRect16(ctx.s_tile2_bmp_1050_1538, p_rvar6, &local_a);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  i_stack62 = 0x8;
    // TODO: refactor for loop
    // for (u_stack58 = 0x1; u_stack58 < 0xa; u_stack58 += 0x1) {
  //   p_rvar6 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
  //   u_stack6 = u_stack6 & 0xffff | (local_a.y - 0x1) << 0x10;
  //   local_12 = local_12 & 0xffff | (u_stack14._2_2_ + 0x1) << 0x10;
  //   u_var7 = (pu_stack26 >> 0x10);
  //   local_a = local_a & 0xffff |
  //             ((i_stack62 * 0x2 + pu_stack26) + 0x1) << 0x10;
  //   u_stack14 = u_stack14 & 0xffff |
  //              (u_stack58 * 0x2 + pu_stack26 + 0x14) << 0x10;
  //   FillRect16(ctx.s_tile2_bmp_1050_1538,p_rvar6,&local_a);
  //   FillRect16(ctx.s_tile2_bmp_1050_1538,p_rvar6,&local_12);
  //   DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  //   i_stack62 += -0x1;
  // }
  p_rvar6 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
  local_a &= 0xffff;
  u_stack6 = u_stack6 & 0xffff | *pu_stack26 << 0x10;
  local_12 = local_12 & 0xffff |
             ((i_stack20 * 0x2 + pu_stack26) + 0x1) << 0x10;
  u_stack14 = u_stack14 & 0xffff | local_struct_1.field_0xe << 0x10;
  FillRect16(ctx.s_tile2_bmp_1050_1538, p_rvar6, &local_a);
  FillRect16(ctx.s_tile2_bmp_1050_1538, p_rvar6, &local_12);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  u_stack58 = 0x3;
  loop {
    u_var4 = (u_stack58 * 0x4 + u_stack38) as u32;
    hstack50 = CreatePen16(s_tile2_bmp_1050_1538, u_var4 as i16,
                           (u_var4 >> 0x10));
    hstack52 = SelectObject16(ctx.s_tile2_bmp_1050_1538, hstack50);
    i_var5 = (u_stack58 * 0x2) as i16;
    i_var3 = (i_var5 + u_stack42);
   // u_var7 = (u_stack46 >> 0x10);
    pi_var1 = (i_var5 + u_stack46);
    MoveToEx16(ctx.s_tile2_bmp_1050_1538,local_38,param_3,
               ((i_var5 + u_stack46) * 0x2 + pu_stack26));
    LineTo16(ctx.s_tile2_bmp_1050_1538,
             ((i_stack20 - *pi_var1) * 0x2 + pu_stack26), i_var3 + local_a.x);
    i_var3 = ((i_stack22 - u_stack58) * 0x2 + u_stack42);
    MoveToEx16(ctx.s_tile2_bmp_1050_1538,local_38,param_3,
               (*pi_var1 * 0x2 + pu_stack26));
    LineTo16(ctx.s_tile2_bmp_1050_1538,
             ((i_stack20 - *pi_var1) * 0x2 + pu_stack26), i_var3 + local_a.x);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, hstack52);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    u_stack58 -= 0x1;
    if u_stack58 >= 0x8000 { break; }
  }
  local_struct_1.field_0x10 = 0x0;
  return;
}




