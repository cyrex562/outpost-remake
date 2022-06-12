
u32 * pass1_1008_80d2(u32 *param_1)

{
  *param_1 = 0x0;
  ((int)param_1 + 0x4) = 0x0;
  return param_1;
}



astruct_23 * unk_draw_op_1008_80ee(param_1: *mut astruct_23)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  astruct_23 *iVar3;
  astruct_23 *uVar3;

  uVar3 = (astruct_23 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_23 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field79_0x54 = 0x0;
  iVar3->field80_0x56 = 0x0;
  iVar3->field81_0x58 = 0x0;
  param_1->field0_0x0 = 0x87c8;
  iVar3->field1_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field2_0x4)),s_MicroSpinControl_1050_0370);
  iVar3->field79_0x54 = 0x3;
  HVar1 = LoadCursor16((char *)0x7f00,0x0);
  iVar3->field81_0x58 = HVar1;
  HVar2 = GetStockObject16(0x4);
  iVar3->field80_0x56 = HVar2;
  pass1_1008_818c((astruct_23 *)((u32)param_1 & 0xffff | ZEXT24(uVar3) << 0x10));
  return param_1;
}
pub fn pass1_1008_8168(char *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1 = 0x87c8;
  ((int)param_1 + 0x2) = 0x1008;
  param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1008_818c(param_1: *mut astruct_23)

{
  let mut BVar1: bool;
  ATOM AVar2;
  WNDCLASS16 wndclass;

  wndclass.lpsz_class_name._0_2_ = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(&wndclass,(char *)CONCAT22((int)wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0x0) {
    wndclass.style = ((int)param_1 + 0x54);
    wndclass.lpfn_wnd_proc._0_2_ = 0x84f2;
    wndclass.lpfn_wnd_proc = 0x1008;
    wndclass._6_4_ = 0x40000;
    wndclass.h_instance = HINSTANCE16_1050_038c;
    wndclass.h_icon = 0x0;
    wndclass.h_cursor = *(HCURSOR16 *)((int)param_1 + 0x58);
    wndclass.hbr_background = *(HBRUSH16 *)((int)param_1 + 0x56);
    wndclass.lpsz_menu_name = 0x0;
    wndclass.lpsz_class_name = param_1;
    AVar2 = RegisterClass16(&wndclass);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0);
    }
  }
  return;
}



u16 win_ui_op_1008_8214(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u16 ,mut param_6: u32)

{
  let mut uVar1: u16;
  INT16 IVar2;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  u32 *puVar4;
  let mut puVar5: *mut u16;
  let mut offset: i16;
  let mut hwnd: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (param_4 == 0x81) {
    offset = 0x0;
    hwnd = param_3;
    mem_op_1000_179c(0x6,paVar3);
    if ((paVar3 | param_1) == 0x0) {
      uVar1 = 0x0;
      puVar4 = NULL;
    }
    else {
      puVar4 = pass1_1008_80d2((u32 *)CONCAT22(paVar3,param_1));
      uVar1 = puVar4;
    }
    SetWindowLong16((u32)puVar4 & 0xffff0000 | (u32)uVar1,offset,hwnd);
  }
  if (param_4 == 0x1) {
    puVar5 = (u16 *)GetWindowLong16(0x0,param_3);
    *puVar5 = ((int)param_6 + 0x8);
    IVar2 = GetDlgCtrlID16(param_3);
    *(INT16 *)((int)puVar5 + 0x2) = IVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
pub fn draw_op_1008_8288(mut param_1: u16 ,mut param_2: u32,HDC16 hdc16_param_1,mut param_4: u32)

{
  HDC16 paint_handle_1;
  HPEN16 pen_handle_1;
  HPEN16 pen_handle_3;
  HBRUSH16 brush_handle_1;
  HGDIOBJ16 hgdiobj16_var1;
  let mut y: u16;
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  HDC16 hdc;
  let mut right_00: u16;
  HGDIOBJ16 obj;
  u8 paintstruct_3c [0x20];
  POINT16 point_1c;
  let mut iStack24: i16;
  let mut iStack22: i16;
  let mut iStack20: i16;
  let mut iStack18: i16;
  POINT16 local_10;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;
  let mut in_stack_0000ffa6: u32;
  let mut left_01: i16;
  let mut top: i16;
  let mut left: i16;
  let mut x2: u16;
  let mut uVar1: u32;
  let mut right: u16;
  let mut in_stack_0000ffae: u16;
  let mut bottom: u16;

  paint_handle_1 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_3c),hdc16_param_1);
  uStack4 = 0x0;
  pen_handle_1 = CreatePen16(COLORREF_1050_0368,0x1,0x0);
  pen_handle_3 = CreatePen16(COLORREF_1050_0364,0x1,0x0);
  brush_handle_1 = CreateSolidBrush16(COLORREF_1050_0364);
  uVar1 = CONCAT22(pen_handle_3,brush_handle_1);
  hdc = hdc16_param_1;
  GetClientRect16((RECT16 *)((int)&param_2 + 0x2),(HWND16)&DAT_1050_1050);
  y = param_1 >> 0x1;
  right_00 = x2;
  hgdiobj16_var1 = GetStockObject16(BLACK_PEN);
  hgdiobj16_var1 = SelectObject16(hgdiobj16_var1,hdc);
  param_2 = param_2 & 0xffff0000 | (u32)hgdiobj16_var1;
  hgdiobj16_var1 = GetStockObject16(BLACK_BRUSH);
  SelectObject16(hgdiobj16_var1,left);
  Rectangle16(param_1,right_00,top,(INT16)(param_2 >> 0x10),paint_handle_1);
  MoveTo16(y,0x0,paint_handle_1);
  param_2 = param_2 & 0xffff0000 | (u32)paint_handle_1;
  LineTo16(y,x2,paint_handle_1);
  uVar3 = (param_4 >> 0x10);
  if ((*((int)param_4 + 0x4) & 0x4) != 0x0) {
    uStack4 = 0x1;
  }
  local_10.x = (x2 >> 0x1) + uStack4;
  iVar1 = (param_1 >> 0x2) + uStack4;
  local_10.y = iVar1 + -0x2;
  iStack12 = local_10.x + -0x3;
  iStack10 = iVar1 + 0x1;
  iStack8 = local_10.x + 0x3;
  iStack6 = iStack10;
  param_2._0_2_ = pen_handle_1;
  param_2 = paint_handle_1;
  SelectObject16(pen_handle_1,paint_handle_1);
  if (uStack4 == 0x0) {
    param_2 = CONCAT22(paint_handle_1,0x1);
    MoveTo16(y - 0x2,0x1,paint_handle_1);
    param_2 = 0x10001;
    LineTo16(0x1,0x1,paint_handle_1);
    param_2 = 0x1;
    param_2._0_2_ = (HPEN16)s_tile2_bmp_1050_1538;
    LineTo16(0x1,x2 - 0x1,paint_handle_1);
  }
  uStack4 = ((*((int)param_4 + 0x4) & 0x8) != 0x0);
  point_1c.x = (x2 >> 0x1) + uStack4;
  iVar2 = (param_1 - (param_1 >> 0x2)) + uStack4;
  point_1c.y = iVar2 + 0x1;
  iStack24 = point_1c.x + -0x3;
  iStack22 = iVar2 + -0x2;
  iStack20 = point_1c.x + 0x3;
  iStack18 = iStack22;
  if (uStack4 == 0x0) {
    param_2 = 0x15388429;
    MoveTo16(paint_handle_1 - 0x2,0x1,paint_handle_1);
    param_2 = 0x843a;
    LineTo16(y + 0x1,0x1,paint_handle_1);
    uVar1 = CONCAT22(paint_handle_1,x2 - 0x1);
    LineTo16(y + 0x1,x2 - 0x1,paint_handle_1);
  }
  param_2 = CONCAT22(0x8453,(HPEN16)param_2);
  SelectObject16((HGDIOBJ16)(uVar1 >> 0x10),paint_handle_1);
  param_2 = CONCAT22(0x845e,(HPEN16)param_2);
  SelectObject16((HGDIOBJ16)uVar1,paint_handle_1);
  obj = (HGDIOBJ16)(uVar1 >> 0x10);
  param_2 = 0x31538;
  Polygon16(0x3,&local_10,(HDC16)&DAT_1050_1050);
  param_2 = 0x31538;
  hgdiobj16_var1 = 0x847c;
  Polygon16(0x3,&point_1c,(HDC16)&DAT_1050_1050);
  param_2 = CONCAT22(0x8487,(HPEN16)param_2);
  SelectObject16(hgdiobj16_var1,paint_handle_1);
  param_2 = CONCAT22(0x8492,(HPEN16)param_2);
  SelectObject16((HPEN16)param_2,paint_handle_1);
  DeleteObject16(pen_handle_1);
  DeleteObject16(obj);
  DeleteObject16(obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_3c),hdc16_param_1);
  return;
}
pub fn send_msg_1008_84ba(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  WPARAM16 WStack4;

  uVar2 = (param_2 >> 0x10);
  iVar1 = (int)param_2;
  if ((*(iVar1 + 0x4) & 0x4) == 0x0) {
    if ((*(iVar1 + 0x4) & 0x8) == 0x0) {
      return;
    }
    WStack4 = 0x1;
  }
  else {
    WStack4 = 0x0;
  }
  SendMessage16((u32)(iVar1 + 0x2),WStack4,0x115,*(HWND16 *)param_2);
  return;
}
pub fn win_sys_op_1008_84f2(LPARAM lparam_param_1,WPARAM16 wparam_param_2,u16 msg_param_3,HWND16 hwnd_param_4)

{
  let mut puVar1: *mut u16;
  char cVar2;
  let mut BVar3: bool;
  let mut uVar4: u16;
  i32 win_long_1;
  let mut in_stack_0000ff7c: u32;
  let mut in_stack_0000ff84: u16;
  RECT16 rect_a;
  let mut iStack4: i16;
  astruct_863 *iVar3;

  win_long_1 = GetWindowLong16(0x0,hwnd_param_4);
  win_long_1 = ((u32)win_long_1 >> 0x10);
  iVar3 = (astruct_863 *)win_long_1;
  if (msg_param_3 == 0x1f) {
    iVar3->field3_0x4 = 0x0;
    KillTimer16(0xfa6,hwnd_param_4);
    KillTimer16(0xfa7,hwnd_param_4);
    ReleaseCapture16();
  }
  else if (msg_param_3 < 0x20) {
    if (msg_param_3 != 0x14) {
      if (0x14 < msg_param_3) goto LAB_1008_8771;
      cVar2 = (char)msg_param_3;
      uVar4 = msg_param_3 & 0xff00 | (u8)(cVar2 - 0x1U);
      if ((u8)(cVar2 - 0x1U) == 0x0) {//
LAB_1008_8560:
        win_ui_op_1008_8214(uVar4,win_long_1,hwnd_param_4,msg_param_3,wparam_param_2,lparam_param_1);
        return;
      }
      if (cVar2 == '\x02') {
        fn_ptr_1000_17ce((char *)win_long_1);
      }
      else if (cVar2 != '\f') {
        if (cVar2 != '\x0f') goto LAB_1008_8771;
        draw_op_1008_8288(in_stack_0000ff84,in_stack_0000ff7c,hwnd_param_4,win_long_1);
      }
    }
  }
  else if (msg_param_3 == 0x200) {
    if ((*&iVar3->field3_0x4 & 0x1) != 0x0) {
      GetClientRect16(&rect_a,(HWND16)&DAT_1050_1050);
      uVar4 = iVar3->field3_0x4;
      puVar1 = &iVar3->field3_0x4;
      *puVar1 = *puVar1 & 0xf3;
      BVar3 = PtInRect16((POINT16)lparam_param_1,&rect_a);
      if (BVar3 == 0x0) {
        puVar1 = &iVar3->field3_0x4;
        *puVar1 = *puVar1 | 0x2;
      }
      else {
        if ((int)lparam_param_1 < iStack4 >> 0x1) {
          puVar1 = &iVar3->field3_0x4;
          *puVar1 = *puVar1 | 0x4;
        }
        else {
          puVar1 = &iVar3->field3_0x4;
          *puVar1 = *puVar1 | 0x8;
        }
        puVar1 = &iVar3->field3_0x4;
        *puVar1 = *puVar1 & 0xfd;
      }
      if (iVar3->field3_0x4 != uVar4) {
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
      }
    }
  }
  else if (msg_param_3 < 0x201) {
    uVar4 = msg_param_3 - 0x81;
    if (uVar4 == 0x0) goto LAB_1008_8560;
    if (msg_param_3 != 0x113) {//
LAB_1008_8771:
      DefWindowProc16(lparam_param_1,wparam_param_2,msg_param_3,hwnd_param_4);
      return;
    }
    if (wparam_param_2 == 0xfa6) {
      KillTimer16(0xfa6,hwnd_param_4);
      SetTimer16(NULL,0x1,0xfa7,hwnd_param_4);
    }
    if ((*&iVar3->field3_0x4 & 0x2) == 0x0) {
      send_msg_1008_84ba(hwnd_param_4,win_long_1);
    }
  }
  else {
    if (msg_param_3 != 0x201) {
      if (msg_param_3 == 0x202) {
        KillTimer16(0xfa6,hwnd_param_4);
        KillTimer16(0xfa7,hwnd_param_4);
        ReleaseCapture16();
        uVar4 = iVar3->field3_0x4;
        if (((uVar4 & 0x1) != 0x0) && ((uVar4 & 0xfffd) != 0x0)) {
          puVar1 = &iVar3->field3_0x4;
          *puVar1 = *puVar1 & 0xf2;
          InvalidateRect16(0x1,NULL,0x0);
          UpdateWindow16(hwnd_param_4);
        }
        SendMessage16((u32)iVar3->field2_0x2,0xf9,0x111,*(HWND16 *)win_long_1);
        return;
      }
      if (msg_param_3 != 0x203) goto LAB_1008_8771;
    }
    puVar1 = &iVar3->field3_0x4;
    *puVar1 = *puVar1 | 0x1;
    GetClientRect16(&rect_a,(HWND16)&DAT_1050_1050);
    if (lparam_param_1 < (iStack4 >> 0x1)) {
      puVar1 = &iVar3->field3_0x4;
      *puVar1 = *puVar1 | 0x4;
    }
    else {
      puVar1 = &iVar3->field3_0x4;
      *puVar1 = *puVar1 | 0x8;
    }
    send_msg_1008_84ba(hwnd_param_4,win_long_1);
    SetTimer16(NULL,0x12c,0xfa6,hwnd_param_4);
    InvalidateRect16(0x1,NULL,0x0);
    UpdateWindow16(hwnd_param_4);
    SetCapture16(hwnd_param_4);
  }
  return;
}



pub fn pass1_1008_87a2(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_8168((char *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_87cc(param_1: *mut astruct_86,mut param_2: i16,mut param_3: i16,mut param_4: u16 ,param_5: *mut astruct_76,mut param_6: u32,
                    mut param_7: u32)

{
  i32 lVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut piVar4: *mut i16;
  let mut uVar5: u16;
  astruct_57 *paVar6;
  astruct_86 *iVar5;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u16;
  let mut puVar12: *mut u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut piStack48: *mut i16;
  astruct_19 *local_24;
  let mut uStack32: u16;
  let mut uStack30: u32;
  char *pcStack26;
  let mut uStack18: u32;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut uStack6: u32;

  uVar10 = ((u32)param_7 >> 0x10);
  uVar8 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_86 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar5->field1_0x2 = 0x1008;
  iVar5->field2_0x4 = param_5;
  iVar5->field3_0x8 = 0x0;
  iVar5->field4_0xc = param_3;
  iVar5->field5_0xe = param_2;
  iVar5->field6_0x10 = 0x0;
  iVar5->field7_0x12 = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field12_0x1c)));
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field14_0x22)));
  puVar11 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field17_0x28)));
  paVar6 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)puVar11 >> 0x10));
  iVar5->field20_0x2e = param_4;
  iVar5->field21_0x30 = 0xffff;
  iVar5->field27_0x3a = 0x0;
  iVar5->field28_0x3e = 0x1;
  iVar5->field29_0x40 = 0x1;
  iVar5->field30_0x42 = param_6;
  param_1->field0_0x0 = 0x8e9a;
  iVar5->field1_0x2 = 0x1008;
  if (_PTR_LOOP_1050_0382 == NULL) {
    _PTR_LOOP_1050_0382 =
         mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2e),in_stack_0000fe70,in_stack_0000ff94
                         ,in_stack_0000ff9a,in_stack_0000ff9e);
    paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  }
  uVar10 = ((u32)paVar6 >> 0x10);
  uStack6 = pass1_1008_4772(iVar5->field2_0x4);
  iVar5->field7_0x12 = 0x2f - ((int)uStack6 + 0x8);
  uVar9 = ((u32)_PTR_LOOP_1050_0382 >> 0x10);
  iVar7 = (int)_PTR_LOOP_1050_0382;
  iStack8 = (iVar7 + 0xa);
  iStack10 = (iVar7 + 0xc);
  iStack12 = (iVar7 + 0xe);
  iStack14 = (iVar7 + 0x10);
  iVar7 = iVar5->field4_0xc;
  lVar1 = (long)(iVar7 + iVar5->field5_0xe) * (long)iStack14;
  paVar6 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)lVar1 >> 0x10));
  pass1_1008_3e76((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field12_0x1c)),0x0,
                  (int)lVar1 + iVar5->field7_0x12 + iStack10,
                  (iVar7 - iVar5->field5_0xe) * iStack12 + iVar5->field6_0x10 + iStack8);
  iVar5->field8_0x14 = &iVar5->field12_0x1c + 0x20;
  iVar5->field9_0x16 = ((int)uStack6 + 0x8) + ((int)&iVar5->field12_0x1c + 0x2) + -0x25;
  iVar5->field10_0x18 = iVar5->field8_0x14 + 0x32;
  uVar2 = iVar5->field9_0x16 + 0x19;
  iVar5->field11_0x1a = uVar2;
  mem_op_1000_179c(0x6,paVar6);
  uVar5 = paVar6;
  pcStack26 = (char *)CONCAT22(uVar5,uVar2);
  uStack18 = uVar5 | uVar2;
  if (uStack18 == 0x0) {
    iVar5->field3_0x8 = 0x0;
  }
  else {
    puVar12 = pass1_1008_ada2((u16 *)CONCAT22(uVar5,uVar2),iVar5->field20_0x2e);
    uStack18 = ((u32)puVar12 >> 0x10);
    &iVar5->field3_0x8 = (int)puVar12;
    ((int)&iVar5->field3_0x8 + 0x2) = uStack18;
  }
  BVar3 = pass1_1008_aed8(iVar5->field3_0x8);
  if (BVar3 == 0x0) {
    pcStack26 = (char *)iVar5->field3_0x8;
    uStack18 = pcStack26;
    fn_ptr_1000_17ce(pcStack26);
    iVar5->field3_0x8 = 0x0;
  }
  else {
    piVar4 = iVar5->field3_0x8;
    pass1_1018_20ee((u32)_PTR_LOOP_1050_0382,piVar4);
    uStack18._0_2_ = SUB42(piVar4,0x0);
    pass1_1008_add2((u16 *)iVar5->field3_0x8);
    uStack30 = pass1_1008_4772((astruct_76 *)CONCAT22(uStack18,uStack18));
    pass1_1018_214e(_PTR_LOOP_1050_0382,((u32)_PTR_LOOP_1050_0382 >> 0x10),
                    (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field17_0x28)),iVar5->field20_0x2e);
    local_24 = iVar5->field12_0x1c;
    uStack32 = iVar5->field13_0x20;
    pass1_1008_3f32(CONCAT22(0x1050,&local_24),
                    ((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field17_0x28)));
    piStack48 = ((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x32));
    pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_24),
                    (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field24_0x34)),
                    (char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x32)));
    uVar10 = (uStack30 >> 0x10);
    iVar5->field25_0x36 = ((int)uStack30 + 0x4) + *piStack48;
    uVar2 = ((int)uStack30 + 0x8) + iVar5->field24_0x34;
    iVar5->field26_0x38 = uVar2;
    pass1_1008_612e(uVar2,0x2,0x5);
    iVar5->field28_0x3e = uVar2;
  }
  return;
}
pub fn pass1_1008_8aa2(param_1: *mut astruct_462)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut uVar5: u32;
  astruct_462 *iVar6;
  let mut uVar6: u16;
  char *pcStack16;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar6 = (astruct_462 *)param_1;
  param_1 = 0x8e9a;
  iVar6->field2_0x2 = 0x1008;
  uVar5 = (u32)&iVar6->field3_0x4;
  if (((int)uVar5 + 0x1c) != 0x0) {
    puVar1 = iVar6->field3_0x4;
    uVar2 = iVar6->field4_0x6;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  uVar2 = iVar6->field55_0x3a;
  uVar3 = iVar6->field56_0x3c;
  pcStack16 = (char *)CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar3,uVar2));
    fn_ptr_1000_17ce(pcStack16);
  }
  param_1 = 0x389a;
  iVar6->field2_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_8b20(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut piVar2: *mut i16;
  let mut in_EDX: u32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u8 local_a [0x2];
  u8 local_8 [0x2];
  astruct_76 *paStack6;

  uVar6 = (param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(i32 *)(iVar5 + 0x8) != 0x0) {
    iVar1 = (iVar5 + 0x40);
    piVar2 = (iVar5 + 0x40);
    *piVar2 = *piVar2 + 0x1;
    uVar4 = (long)iVar1 % (long)(iVar5 + 0x3e) & 0xffff;
    uVar3 = in_EDX & 0xffff0000 | uVar4;
    if ((int)uVar4 == 0x0) {
      (iVar5 + 0x40) = 0x1;
      piVar2 = *(i16 **)(iVar5 + 0x8);
      pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar2);
      paStack6 = (astruct_76 *)((u32)piVar2 & 0xffff | uVar3 << 0x10);
      uVar4 = uVar3 & 0xffff0000 | (u32)uVar6;
      pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)(iVar5 + 0x28U)),(u16 *)CONCAT22(0x1050,local_a),
                      (char *)CONCAT22(0x1050,local_8));
      pass1_1008_8d8a((astruct_76 *)(param_1 & 0xffff | (u32)uVar6 << 0x10),paStack6,(u32)(iVar5 + 0x4),uVar4);
      pass1_1008_4480(*(astruct_76 **)(iVar5 + 0x4),(u16 *)(param_1 & 0xffff0000 | (u32)(iVar5 + 0x28U)),paStack6);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_8bc6(mut param_1: u16 ,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut in_register_0000000a: u16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  u8 local_a [0x2];
  u8 local_8 [0x2];
  astruct_76 *paStack6;

  uVar4 = (param_2 >> 0x10);
  iVar3 = (int)param_2;
  if (*(i32 *)(iVar3 + 0x8) == 0x0) {
    return;
  }
  piVar1 = *(i16 **)(iVar3 + 0x8);
  pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar1);
  paStack6 = (astruct_76 *)((u32)piVar1 & 0xffff | (u32)param_1 << 0x10);
  uVar2 = CONCAT22(in_register_0000000a,uVar4);
  pass1_1008_3e94((u16 *)(param_2 & 0xffff0000 | (u32)(iVar3 + 0x28U)),(u16 *)CONCAT22(0x1050,local_a),
                  (char *)CONCAT22(0x1050,local_8));
  pass1_1008_8d8a((astruct_76 *)(param_2 & 0xffff | (u32)uVar4 << 0x10),paStack6,(u32)(iVar3 + 0x4),uVar2);
  pass1_1008_4480(*(astruct_76 **)(iVar3 + 0x4),(u16 *)(param_2 & 0xffff0000 | (u32)(iVar3 + 0x28U)),paStack6);
  return;
}
pub fn pass1_1008_8c4e(mut param_1: u32,mut param_2: u32,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  astruct_110 *paStack14;
  astruct_57 *paVar4;

  uVar5 = ((u32)param_3 >> 0x10);
  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar8 = pass1_1008_4772(*(astruct_76 **)(iVar6 + 0x4));
  uVar2 = (uVar8 >> 0x10);
  paVar4 = (astruct_57 *)CONCAT22(uVar5,uVar2);
  uVar3 = 0x0;
  if (((iVar6 + 0xc) == 0x0) || ((iVar6 + 0xe) == 0x0)) {
    mem_op_1000_179c(0x14,paVar4);
    paStack14 = (astruct_110 *)CONCAT22(paVar4,uVar3);
    uVar3 = paVar4 | uVar3;
    if (uVar3 == 0x0) {
      uVar2 = 0x0;
      uVar3 = 0x0;
    }
    else {
      puVar1 = (u16 *)(param_1 & 0xffff0000 | (u32)(iVar6 + 0x1c));
      pass1_1008_50c2(paStack14,(u32)((int)uVar8 + 0x8),(u32)((int)uVar8 + 0x4),puVar1,(astruct_76 *)param_2);
      uVar2 = SUB42(puVar1,0x0);
    }
    pass1_1008_5134(CONCAT22(uVar3,uVar2));
  }
  pass1_1008_4480((astruct_76 *)param_2,(u16 *)(param_1 & 0xffff0000 | (u32)(iVar6 + 0x1c)),
                  *(astruct_76 **)(iVar6 + 0x4));
  return;
}
pub fn pass1_1008_8ce4(mut param_1: u32,param_2: *mut u16,mut param_3: u32,mut param_4: u32)

{
  u8 *puVar1;
  let mut uVar2: u16;
  astruct_57 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;
  u8 local_10 [0x6];
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar6 = ((u32)param_4 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  uStack6 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x4));
  uStack10 = 0x0;
  puVar7 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_10),0x0,(iVar4 + 0x12),(iVar4 + 0x10));
  paVar3 = (astruct_57 *)CONCAT22(uVar6,(int)((u32)puVar7 >> 0x10));
  puVar1 = local_10;
  pass1_1008_3f32(param_2,CONCAT22(0x1050,puVar1));
  mem_op_1000_179c(0x14,paVar3);
  uVar2 = paVar3 | puVar1;
  if (uVar2 == 0x0) {
    puVar1 = NULL;
    uVar2 = 0x0;
  }
  else {
    uVar6 = (uStack6 >> 0x10);
    pass1_1008_50c2((astruct_110 *)CONCAT22(paVar3,puVar1),(u32)((int)uStack6 + 0x8),
                    (u32)((int)uStack6 + 0x4),param_2,(astruct_76 *)param_3);
  }
  uStack10 = CONCAT22(uVar2,puVar1);
  pass1_1008_5134(CONCAT22(uVar2,puVar1));
  pass1_1008_4480((astruct_76 *)param_3,param_2,*(astruct_76 **)(iVar4 + 0x4));
  return;
}
pub fn pass1_1008_8d8a(param_1: *mut astruct_76,param_2: *mut astruct_76,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  char cVar2;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar8: u16;
  astruct_76 *pstruct76_var7;
  astruct_76 *uVar7;
  let mut uVar9: u32;
  astruct_110 *paStack10;
  astruct_57 *paVar6;

  uVar8 = ((u32)param_4 >> 0x10);
  uVar7 = (astruct_76 *)((u32)param_1 >> 0x10);
  pstruct76_var7 = (astruct_76 *)param_1;
  uVar1 = pstruct76_var7[0x1].field3_0x6;
  if ((int)uVar1 < 0x28) {
    if (((int)uVar1 < 0x25) && (uVar1 != 0x23)) {
      if (0x23 < uVar1) {
        return;
      }
      cVar2 = (char)uVar1;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else if ((int)uVar1 < 0x46) {
    if ((int)uVar1 < 0x43) {
      if ((int)uVar1 < 0x33) {
        return;
      }
      if ((uVar1 != 0x34 && 0x0 < (int)(uVar1 - 0x33)) && (uVar1 != 0x37)) {
        return;
      }
    }
  }
  else if (uVar1 != 0x49) {
    if ((int)(uVar1 - 0x49) < 0x2a) {
      return;
    }
    if (0x5 < (int)(uVar1 - 0x73)) {
      return;
    }
  }
  if (*(i32 *)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2) == 0x0) {
    uVar9 = pass1_1008_4772(param_2);
    uVar4 = (uVar9 >> 0x10);
    paVar6 = (astruct_57 *)CONCAT22(uVar8,uVar4);
    uVar1 = uVar9;
    uVar5 = uVar1;
    mem_op_1000_179c(0x14,paVar6);
    paStack10 = (astruct_110 *)CONCAT22(paVar6,uVar5);
    uVar5 = paVar6 | uVar5;
    if (uVar5 == 0x0) {
      (u32)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2) = 0x0;
    }
    else {
      puVar3 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(pstruct76_var7 + 0x1));
      pass1_1008_50c2(paStack10,(u32)(uVar1 + 0x8),(u32)(uVar1 + 0x4),puVar3,(astruct_76 *)param_3);
      ((int)&pstruct76_var7[0x1].field8_0x10 + 0x2) = (int)puVar3;
      pstruct76_var7[0x1].field9_0x14 = uVar5;
    }
    pass1_1008_5134((u32)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2));
    return;
  }
  pass1_1008_5236(*(astruct_109 **)((int)&pstruct76_var7[0x1].field8_0x10 + 0x2));
  return;
}



StructD * pass1_1008_8e74(StructD *param_1,param_2: u8)

{
  pass1_1008_8aa2((astruct_462 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_op_1008_8e9e(param_1: *mut astruct_78,mut param_2: u32,mut param_3: u32)

{
  astruct_78 *iVar1;
  astruct_78 *uVar1;

  uVar1 = (astruct_78 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_78 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x6 = NULL;
  iVar1->field4_0xa = 0x0;
  iVar1->field5_0xe = param_3;
  iVar1->field6_0x12 = 0x0;
  iVar1->field7_0x16 = param_2;
  iVar1->field8_0x1a = 0x1;
  param_1->field0_0x0 = 0x9170;
  iVar1->field1_0x2 = 0x1008;
  if (iVar1->field5_0xe < 0x7) {
    iVar1->field5_0xe = 0x6;
  }
  pass1_1008_909c(param_1);
  (u32)iVar1->field3_0x6 = 0x0;
  return;
}
pub fn pass1_1008_8f24(param_1: *mut astruct_463)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut uVar5: u32;
  astruct_463 *iVar6;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack6: u32;

  uVar9 = ((u32)param_1 >> 0x10);
  iVar6 = (astruct_463 *)param_1;
  param_1 = 0x9170;
  iVar6->field2_0x2 = 0x1008;
  if (iVar6->field19_0x1a != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      puVar1 = &iVar6->field6_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = (int)uStack6 * 0x4;
      uVar5 = iVar6->field5_0x6;
      uVar10 = ((u32)uVar5 >> 0x10);
      iVar7 = (int)uVar5;
      puVar2 = (u32 *)(iVar7 + iVar8);
      uVar3 = (iVar7 + iVar8 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 += 0x1;
    }
  }
  fn_ptr_1000_17ce((char *)iVar6->field5_0x6);
  param_1 = 0x389a;
  iVar6->field2_0x2 = 0x1008;
  return;
}
pub fn pass1_1008_8faa(param_1: *mut astruct_78,mut param_2: u32)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  pass1_1008_9004((astruct_78 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10),param_2,
                  (param_2 >> 0x10),(u32)((int)param_1 + 0xa));
  return;
}
pub fn empty_1008_8fc4(void)

{
  return;
}
