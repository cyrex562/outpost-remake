use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::draw_ops::draw_a::invalidate_rect_1020_735a;
use crate::globals::PTR_LOOP_1050_0000;
use crate::gui::window;
use crate::gui::window::{set_win_pos_1040_331a, win_e};
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::structs::struct_27::Struct27;
use crate::structs::struct_57::Struct57;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1000_3000::unk_str_op_1000_3d3e;
use crate::unk::block_1008_5000::pass1_1008_5fd8;
use crate::unk::block_1010_0000::pass1_1010_038e;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_3000::{pass1_1010_375e, pass1_1010_3770};
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::unk::block_1018_3000::pass1_1018_3710;
use crate::unk::block_1030_8000::pass1_1030_8344;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_4000::{pass1_1040_4d7e, pass1_1040_4dcc};
use crate::unk::block_1040_8000::string_1040_8520;
use crate::utils::{CONCAT22, SUB42};
use crate::winapi16::{CallWindowProc16, GetDlgItem16, MessageBox16, PostMessage16, SendMessage16, SetFocus16, WinHelp16};
use crate::winapp::winapp_b;
use crate::windef16::{HWND16, LPARAM, LRESULT, WPARAM16};

pub fn win_help_op_1020_0ec4(mut param_1: u16, param_2: *mut u32, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut cVar2: u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut paVar8: *mut astruct_477;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut wparam: WPARAM16;
    let mut hwnd: HWND16;
    let mut iVar11: i16;
    let mut in_stack_0000fff4: u16;
    let mut uVar12: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    uVar7 = (param_2 >> 0x10);
    uVar3 = param_2;
    if (param_3 == 0xfb) {
        puVar9 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fff4, 0x30),
            in_stack_0000fe9c,
            in_stack_0000ffc0,
            in_stack_0000ffc6,
            in_stack_0000ffca,
        );
        uVar12 = puVar9;
        pass1_1010_375e(puVar9);
        ppcVar1 = (*param_2 + 0x14);
        (**ppcVar1)();
        uVar10 = pass1_1010_375e(puVar9 & 0xffff0000 | uVar12);
        uVar12 = (uVar10 >> 0x10);
        pass1_1010_4788(
            uVar10,
            uVar12,
            (uVar3 + 0xf2),
            (uVar10 & 0xffff | uVar12 << 0x10),
        );
        return;
    }
    if (0xfb < param_3) {
        match param_3 {
            _ => {
                return;
            }
            0x12a => {
                hwnd = (uVar3 + 0x8);
                wparam = 0xf012;
            }
            //   break;
            0x12c => {
                hwnd = (uVar3 + 0x8);
                wparam = 0xf020;
            }
        }
        PostMessage16(0x0, wparam, 0x112, hwnd);
        return;
    }
    if (param_3 == 0xf3) {
        iVar11 = 0x3;
    } else {
        if (0xf3 < param_3) {
            return;
        }
        cVar2 = param_3;
        if ((cVar2 + 0x91) == 0) {
            uVar4 = FUN_1010_830a(
                param_3 & 0xff00 | (cVar2 + 0x91),
                paVar6,
                unaff_CS,
                _u16_1050_14cc,
                0x1f8,
            );
            WinHelp16(0x28, 0x1, CONCAT22(paVar6, uVar4), (uVar3 + 0x8));
            return;
        }
        if (cVar2 == 'r') {
            iVar11 = uVar3 + 0xa;
            uVar4 = uVar7;
            paVar8 = mixed_1010_20ba(
                paVar6,
                _u16_1050_0ed0,
                CONCAT22(iVar11, 0x30),
                in_stack_0000fe98,
                in_stack_0000ffbc,
                in_stack_0000ffc2,
                in_stack_0000ffc6,
            );
            uVar5 = (paVar8 >> 0x10);
            pass1_1010_3770(uVar5, paVar8, CONCAT22(uVar4, iVar11));
            pass1_1038_af40(uVar3, uVar5, _PTR_LOOP_1050_5b7c, (uVar3 + 0x8), 0x3);
            return;
        }
        if (cVar2 == -0xf) {
            iVar11 = 0x1;
        } else {
            if (cVar2 != -0xe) {
                return;
            }
            iVar11 = 0x2;
        }
    }
    pass1_1010_4674((uVar3 + 0xf2), iVar11, 0x1010, 0x1050);
    return;
}

pub fn invalidate_rect_1020_2ae4(
    mut param_1: u16,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut cVar3: u8;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_CS: u16;
    let mut paVar10: *mut astruct_477;
    let mut puVar11: *mut u32;
    let mut uVar12: u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut wparam: WPARAM16;
    let mut hwnd: HWND16;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    if (param_3 != 0x129) {
        uVar8 = param_2;
        uVar9 = (param_2 >> 0x10);
        if (0x129 < param_3) {
            if (param_3 == 0x12a) {
                hwnd = (uVar8 + 0x8);
                wparam = 0xf012;
            } else {
                if (param_3 == 0x12b) {
                    return;
                }
                if (param_3 == 0x12c) {
                    hwnd = (uVar8 + 0x8);
                    wparam = 0xf020;
                } else {
                    if (param_3 == 0x12d) {
                        return;
                    }
                    if (param_3 != 0x12e) {
                        return;
                    }
                    hwnd = (uVar8 + 0x8);
                    wparam = 0xf060;
                }
            }
            PostMessage16(0x0, wparam, 0x112, hwnd);
            return;
        }
        if (param_3 == 0xfb) {
            puVar11 = mixed_1010_20ba(
                paVar7,
                _u16_1050_0ed0,
                CONCAT22(param_4, 0x30),
                in_stack_0000fe9c,
                in_stack_0000ffc0,
                in_stack_0000ffc6,
                in_stack_0000ffca,
            );
            pass1_1010_375e(puVar11);
            ppcVar1 = (*param_2 + 0x14);
            (**ppcVar1)();
            uVar12 = pass1_1010_375e(puVar11);
            uVar2 = (uVar12 >> 0x10);
            pass1_1018_181c(
                CONCAT22(uVar12, uVar2),
                (uVar8 + 0xf2),
                (uVar12 & 0xffff | uVar2 << 0x10),
            );
            return;
        }
        if (param_3 < 0xfc) {
            cVar3 = param_3;
            if ((cVar3 + 0x91) == 0) {
                uVar5 = FUN_1010_830a(
                    param_3 & 0xff00 | (cVar3 + 0x91),
                    paVar7,
                    unaff_CS,
                    _u16_1050_14cc,
                    0x1f8,
                );
                WinHelp16(0x2a, 0x1, CONCAT22(paVar7, uVar5), (uVar8 + 0x8));
                return;
            }
            if (cVar3 == 'r') {
                iVar4 = uVar8 + 0xa;
                uVar5 = uVar9;
                paVar10 = mixed_1010_20ba(
                    paVar7,
                    _u16_1050_0ed0,
                    CONCAT22(iVar4, 0x30),
                    in_stack_0000fe98,
                    in_stack_0000ffbc,
                    in_stack_0000ffc2,
                    in_stack_0000ffc6,
                );
                uVar6 = (paVar10 >> 0x10);
                pass1_1010_3770(uVar6, paVar10, CONCAT22(uVar5, iVar4));
                pass1_1038_af40(uVar8, uVar6, _PTR_LOOP_1050_5b7c, (uVar8 + 0x8), 0x3);
                return;
            }
            if (cVar3 == 'u') {
                pass1_1018_0a76((uVar8 + 0xf2));
                InvalidateRect16(0x0, NULL, 0x0);
                return;
            }
        }
    }
    return;
}

pub fn post_msg_1020_4394(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((iVar2 + 0x34) != 0) {
            PostMessage16(0x0, 0xf6, 0x111, HWND16_1050_0396);
            return;
        }
    } else if (param_2 < 0x11) {
        if (param_2 == '\x01') {
            (iVar2 + 0x18) = 0;
            return;
        }
        // if (param_2 == '\v') {
        //   uVar1 = (iVar2 + 0x2c);
        //   (uVar1 + 0xe) = (iVar2 -0xda);
        //   return;
        // }
    }
    return;
}

pub fn post_win_msg_1020_7308(mut param_1: u32, mut param_2: u16) {
    let mut cVar1: u8;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar1 = param_2;
            if (cVar1 == '\x01') {
                (param_1 + 0x1c) = 0;
                return;
            }
            //      if (('\x03' < (cVar1 -1)) && ((cVar1 -0x5) < '\x02')) goto LAB_1020_7310;
        }
        return;
    } //
    // LAB_1020_7310:
    PostMessage16(0x0, 0xeb, 0x111, (param_1 + 0x4));
    invalidate_rect_1020_735a(param_1);
    return;
}

pub fn send_msg_1008_84ba(mut param_1: u16, mut param_2: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut WStack4: WPARAM16;

    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    if ((*(iVar1 + 0x4) & 0x4) == 0) {
        if ((*(iVar1 + 0x4) & 0x8) == 0) {
            return;
        }
        WStack4 = 0x1;
    } else {
        WStack4 = 0;
    }
    SendMessage16((iVar1 + 0x2), WStack4, 0x115, param_2);
    return;
}

pub fn send_msg_1020_097e(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0xea) | (iVar2 + 0xe8)) != 0) {
        uVar1 = (iVar2 + 0xe8);
        SendMessage16(0x0, 0x1, 0x111, (uVar1 + 0x6));
        (iVar2 + 0xe8) = 0;
    }
    return;
}

pub fn send_msg_1028_e242(param_1: u32, mut param_2: i16) {
    let mut puVar1: *mut u8;
    let mut unaff_DI: i16;
    let mut LVar2: LRESULT;

    puVar1 = (*param_1 % 0x64);
    if (*param_1 % 0x64 == 0) {
        LVar2 = SendMessage16(0x0, 0x0, 0x41, HWND16_1050_0396);
        puVar1 = (LVar2 >> 0x10);
    }
    *param_1 = *param_1 + 1;
    if (param_2 != 0) {
        pass1_1028_e28a(puVar1, unaff_DI, 0x1050);
    }
    return;
}


pub fn send_msg_1038_c228(mut param_1: u32) -> LRESULT

{
  let mut wparam: WPARAM16;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut wparam_00: WPARAM16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,(iVar1 + 0x92));
  wparam = LVar3;
  SendMessage16(0x0,0x0,0x407,(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x9e),wparam,0x408,(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x19e),wparam_00,0x408,(iVar1 + 0x94));
  return LVar3;
}

pub fn send_msg_1038_c374(mut param_1: u32, param_2: *mut u32, mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut lparam: *mut c_char;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar5 = SUB42(0x1538,0x0);
  LVar6 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar3 = LVar6;
  ppcVar2 = (*param_2 + 0x10);
  (**ppcVar2)(0x1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar2 = (*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar2)(uVar5,param_2,uStack10,(uStack10 >> 0x10));
    uVar1 = (param_1 + 0x8e);
    lparam = string_1008_e586(uVar1,(uVar1 >> 0x10),
                                      CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,uVar4))
                                      ,uVar4,extraout_DX_00);
    LVar6 = SendMessage16(lparam,0x0,0x403,param_3);
    uVar5 = 0x1000;
    fn_ptr_1000_17ce;
    if (LVar6 == -1) { break; }
    if (LVar6 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}


pub fn send_msg_1040_1696(param_1: *mut StructB, mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut puVar3: *mut u16;
  let mut puVar4: *mut u8;
  let mut puVar5: *mut u8;
  let mut uVar6: u16;
  let mut LVar7: LRESULT;
  let mut pcVar8: *mut c_char;
  let mut WVar9: WPARAM16;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut uStack18: u16;
  let mut local_4: u16;

  SendMessage16(0x0,0x0,0x40b,param_2);
  LVar7 = SendMessage16(0x0,0x0,0xb,param_2);
  puVar4 = (LVar7 >> 0x10);
  local_4 = 0;
  puVar3 = &local_4;
  uVar6 = (param_1 >> 0x10);
  pass1_1010_519a(puVar4,(param_1 + 0x8e),CONCAT22(0x1050,puVar3));
  puVar5 = puVar4;
  for uStack18 in 0 .. local_4 {
    uVar1 = (puVar3 + uStack18 * 0x2);
    WVar9 = 0;
    UVar10 = 0x403;
    uVar2 = (param_1 + 0x8e);
    uVar11 = param_2;
    pcVar8 = string_1010_5286(uVar1,puVar5,uVar2,(uVar2 >> 0x10),uVar1);
    LVar7 = SendMessage16(pcVar8,WVar9,UVar10,uVar11);
    puVar5 = (LVar7 >> 0x10);
    fn_ptr_1000_17ce(pcVar8);
  }
  WVar9 = 0;
  UVar10 = 0x40a;
  uVar11 = param_2;
  pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendMessage16(pcVar8,WVar9,UVar10,uVar11);
  SendMessage16(0x0,0x1,0xb,param_2);
  return;
}


pub fn send_msg_1040_323c(mut param_1: u32) -> LRESULT

{
  let mut wparam: WPARAM16;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut wparam_00: WPARAM16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,(iVar1 + 0x92));
  wparam = LVar3;
  SendMessage16(0x0,0x0,0x407,(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x9a),wparam,0x408,(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x19a),wparam_00,0x408,(iVar1 + 0x94));
  return LVar3;
}

pub fn send_msg_1040_3374(mut param_1: u32, param_2: *mut u32, mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut lparam: *mut c_char;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar4 = SUB42(0x1538,0x0);
  LVar5 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar2 = LVar5;
  ppcVar1 = (*param_2 + 0x10);
  (**ppcVar1)(0x1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar2);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (*param_2 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)(uVar4,param_2,uStack10,(uStack10 >> 0x10));
    lparam = pass1_1018_3a7a(uVar3,extraout_DX_00,(param_1 + 0x96),
                                     CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,uVar3)))
    ;
    LVar5 = SendMessage16(lparam,0x0,0x403,param_3);
    uVar4 = 0x1000;
    fn_ptr_1000_17ce;
    if (LVar5 == -1) { break; }
    if (LVar5 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}


pub fn win_msg_1040_a308(param_1: *mut astruct_49, mut param_2: u16, mut param_3: u16, mut param_4: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut puVar7: *mut u32;
  let mut pcVar8: *mut c_char;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut WVar9: WPARAM16;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fff2: u16;
  let mut iStack12: i16;

  uVar3 = (in_EDX >> 0x10);
  SendMessage16(0x0,0x0,0x405,param_4);
  LVar6 = SendMessage16(0x0,0x0,0xb,param_4);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar2 = (iVar4 + 0x90);
  if ((uVar2 + 0x10) == 0) {
    WVar9 = 0;
    UVar10 = 0x401;
    uVar11 = param_4;
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
    SendMessage16(pcVar8,WVar9,UVar10,uVar11);
  }
  else {
    puVar7 = mixed_1010_20ba(CONCAT22(uVar3,(LVar6 >> 0x10)),_u16_1050_0ed0,
                             CONCAT22(in_stack_0000fff2,0x3),in_stack_0000fe9a,in_stack_0000ffbe,
                             in_stack_0000ffc4,in_stack_0000ffc8);
    // for (iStack12 = 0; uVar2 = (iVar4 + 0x90), piVar1 = (uVar2 + 0x10),
    //     *piVar1 != iStack12 && iStack12 <= *piVar1; iStack12 += 1)
    iStack12 = 0;
    uVar2 = iVar4 + 0x90;
    piVar1 = uVar2 + 0x10;
    while *piVar1 != iStack12 && iStack12 <= *piVar1
    {
      WVar9 = 0;
      UVar10 = 0x401;
      uVar2 = (iVar4 + 0x90);
      uVar2 = (uVar2 + 0xc);
      uVar11 = param_4;
      pcVar8 = load_string_1010_ac92
                         (puVar7,(puVar7 >> 0x10),(uVar2 + iStack12 * 0x2));
      SendMessage16(pcVar8,WVar9,UVar10,uVar11);
      iStack12 += 1;
    }
  }
  LVar6 = SendMessage16(0x0,0x1,0xb,param_4);
  return CONCAT22((LVar6 >> 0x10),1);
}


pub fn call_win_proc_1040_a40e(mut param_1: u16, param_2: HWND16, mut param_3: u32, param_4: LPARAM) -> u32

{
  let mut func: LPVOID = null_mut();
  let mut uVar1: u16;
  let mut ppcVar2: *mut *mut code;
  let mut wparam: WPARAM16;
  let mut uVar6: u16;
  let mut uVar3: u32;
  let mut uStack6: u32;
  let mut puVar3: *mut u32;
  let mut uVar5: u32;

  uStack6 = 0;
  wparam = (param_3 >> 0x10);
  if (param_4 == 0x19) {
    ppcVar2 = (_PTR_LOOP_1050_5ee0 + 0x34);
    uStack6 = (**ppcVar2)();
    param_1 = (uStack6 >> 0x10);
  }
  else {
    if (param_4 == 0x86) {
      ppcVar2 = (_PTR_LOOP_1050_5ee0 + 0x20);
      uVar3 = (**ppcVar2)();
      return uVar3;
    }
    if (param_4 == 0x110) {
      uVar3 = win_msg_1040_a308(_PTR_LOOP_1050_5ee0,param_2,param_3,wparam);
      return uVar3;
    }
  }
  if (uStack6 != 0) {
    return uStack6 & 0xffff | param_1 << 0x10;
  }
  uVar6 = (_u16_1050_5bc8 >> 0x10);
  func = (_u16_1050_5bc8 + 0x4);
  uVar1 = (_u16_1050_5bc8 + 0x6);
  if ((uVar1 | func) == 0) {
    return uVar1 << 0x10;
  }
  uVar3 = CallWindowProc16(CONCAT22(param_3,param_2),wparam,param_4,(param_4 >> 0x10),func);
  return uVar3;
}


pub fn send_msg_1040_4cb2(mut param_1: u16, mut param_2: u32) -> LRESULT

{
    let mut uVar1: u8;
    let mut HVar1: HWND16;
    let mut uVar2: u32;
    let mut LVar2: LRESULT;
    let mut hwnd: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    pass1_1040_b45e(param_2);
    HVar1 = GetDlgItem16(0x1770, (param_2 + 0x6));
    uVar3 = 0xffff;
    uVar4 = 0x40d;
    hwnd = HVar1;
    pass1_1040_4d7e(param_2);
    uVar2 = pass1_1040_4dcc(param_1, param_2, HVar1);
    LVar2 = SendMessage16(uVar2, uVar3, uVar4, hwnd);
    return LVar2;
}

pub fn pass1_1040_93e6(mut param_1: u32) -> LRESULT

{
    let mut uVar1: u16;
    let mut LVar2: LRESULT;

    uVar1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0x0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}


pub fn win_ui_op_1040_311a(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut in_EDX: u32;
    let mut uVar4: u16;
    let mut uVar3: u32;
    let mut LVar5: LRESULT;
    let mut paVar6: *mut Struct27;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut iVar7: i16;

    uVar4 = (in_EDX >> 0x10);
    send_msg_1040_323c(CONCAT22(param_2, param_1));
    load_string_1010_847e(_u16_1050_14cc, 0x531);
    if (param_4 == 0x181) {
        uVar3 = CONCAT22(uVar4, param_2);
        iVar1 = param_1 + 0x9a;
        iVar7 = iVar1;
        pass1_1018_3cda((param_1 + 0x96), CONCAT22(param_2, param_1 + 0x19a),
                        CONCAT22(param_2, iVar1));
        pass1_1018_3424(iVar7, uVar3, (param_1 + 0x96));
        if (iVar7 == 0) {
            iVar7 = 0x21;
        } else {
            pass1_1018_3a42(uVar3, (param_1 + 0x96), CONCAT22(param_2, iVar1));
            pass1_1030_8344(_u16_1050_5748, CONCAT22(uVar3, iVar7));
            uVar2 = (iVar7 + 0x10);
            pass1_1030_8344(_u16_1050_5748, uVar2);
            PTR_LOOP_1050_5f0c = uVar2;
            PTR_LOOP_1050_5f0e = uVar3;
            PTR_LOOP_1050_5f10 = (&PTR_LOOP_1050_0000 + 1);
            iVar7 = 0x25;
        }
        pass1_1038_af40(param_1, uVar3, _PTR_LOOP_1050_5b7c, (param_1 + 0x8), iVar7);
        uVar4 = (uVar3 >> 0x10);
        LVar5 = SendMessage16(0x0, 0x2, 0x111, (param_1 + 0x6));
        iVar7 = 0x1;
        paVar6 = mixed_1010_20ba(CONCAT22(uVar4, (LVar5 >> 0x10)), _u16_1050_0ed0,
                                 0x1002b, in_stack_0000fe8e, in_stack_0000ffb2, in_stack_0000ffb8,
                                 in_stack_0000ffbc);
        pass1_1010_038e(paVar6, iVar7);
    } else {
        if ((param_4 == 0x181) || (0x1 < param_4 - 0x182)) {
            winapp_b::post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4);
            return;
        }
        set_win_pos_1040_331a(CONCAT22(param_2, param_1), param_3, param_4);
    }
    return;
}

pub fn send_msg_1008_9640(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        SendMessage16(0x0, param_2, 0x86, (param_1 + 0x8));
    }
    return;
}


pub fn send_msg_1010_7c42(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x1e) | (iVar2 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar2 + 0x1c));
        loop {
            lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 0x8);
            SendMessage16(0x0, 0xeb, 0x111, (uVar1 + 0x6));
        }
    }
    return;
}

pub fn send_msg_1020_29d8(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut astruct_69,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
) -> u32 {
    let mut puVar1: *mut u8;
    let mut paVar2: *mut Struct27;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut iVar3: i16;

    iVar3 = (param_5 >> 0x10);
    winapp_b::post_win_msg_1020_79fc(param_3, param_4, param_5, iVar3);
    paVar2 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(param_6, 0x29),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    puVar1 = (paVar2 >> 0x10);
    if (iVar3 == 0) {
        pass1_1018_270e(puVar1, paVar2, 0x1, (param_3 + 0xfc));
    } else {
        pass1_1018_270e(puVar1, paVar2, 0x0, (param_3 + 0xfc));
        SendMessage16(0x0, 0x69, 0x111, HWND16_1050_0396);
    }
    return CONCAT22(param_2, param_1);
}


pub fn win_help_1040_800c(mut param_1: u32)

{
    let mut in_AX: u16;
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut iVar4: i16;
    let mut w_command: u16;
    let mut hwnd: HWND16;

    uVar1 = FUN_1010_830a(in_AX, in_EDX, unaff_CS, _u16_1050_14cc, 0x1f8);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x8a) == 0) {
        hwnd = (iVar2 + 0x6);
        iVar4 = 0;
        w_command = 0x3;
        iVar2 = 0;
    } else {
        hwnd = (iVar2 + 0x6);
        w_command = 0x1;
        iVar2 = (iVar2 + 0x8a);
        iVar4 = iVar2 >> 0xf;
    }
    WinHelp16(CONCAT22(iVar4, iVar2), w_command, CONCAT22(in_EDX, uVar1), hwnd);
    return;
}

pub fn win_ui_op_1008_2b54(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> i16

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar7: u16;
  let mut pcVar8: *mut c_char;
//   local_a6: *mut u32 [0x14];
let mut local_a6: [*mut u32;0x14] = [null_mut();0x14];
let mut local_56: [u8;0x50] = [0;0x50];
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut uVar6: u32;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0;
  if (_PTR_LOOP_1050_4230 == 0) {
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x5f2);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_56),pcVar8);
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_a6),pcVar8);
    iStack4 = MessageBox16(0x21,CONCAT13(0x10,CONCAT12(0x50,local_a6)),CONCAT22(0x1050,local_56),
                           HWND16_1050_0396);
  }
  else {
    uVar7 = 0x1000;
    mem_op_1000_179c(0xb4,paVar5);
    uVar3 = paVar5 | param_1;
    uVar6 = paVar5 & 0xffff0000 | uVar3;
    if (uVar3 == 0) {
      iVar2 = 0;
      uVar4 = 0;
    }
    else {
      uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar6,CONCAT22(paVar5,param_1),HWND16_1050_0396,0x20021,0x5f2057b);
      uVar4 = uVar6;
    }
    local_a6[0] = CONCAT22(uVar4,iVar2);
    ppcVar1 = (*local_a6[0] + 0x74);
    iStack4 = (**ppcVar1)(uVar7,iVar2,uVar4,param_1);
  }
  iStack6 = iStack4;
  if (iStack4 != 1) {
    iStack6 = 0;
  }
  if (((iStack6 != 0) && (_u16_1050_5748 != 0)) &&
     (uVar3 = (_u16_1050_5748 + 0x8),
     local_a6[0] = (local_a6[0] & 0xffff0000 | uVar3), uVar3 != 0)) {
    PostMessage16(0x0,0xb4,0x111,(param_3 + 0x8));
    iStack6 = 0;
  }
  return iStack6;
}


pub fn send_msg_1040_c85a(mut param_1: u32)

{
    _PTR_LOOP_1050_5efe = param_1;
    SendMessage16(0x0, 0xfa, 0x111, (param_1 + 0x1a));
    return;
}


pub fn message_box_op_1038_c672(param_1: u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
  let mut uVar1: u16;
  let mut local_404: [u8;0x402] = [0;0x402];

  uVar1 = (_u16_1050_14cc >> 0x10);
  if (param_5 == 0x17d) {
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,0x1050);
    MessageBox16(0x30,*(param_2 + 0x92),CONCAT22(0x1050,local_404),(param_2 + 0x6));
  }
  else {
    if (param_5 != 0x17e) {
      winapp_b::post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
      return;
    }
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,0x1050);
    MessageBox16(0x30,*(param_2 + 0x92),CONCAT22(0x1050,local_404),(param_2 + 0x6));
    pass1_1008_e164((param_2 + 0x8e));
  }
  PostMessage16(0x0,0x2,0x111,(param_2 + 0x6));
  return;
}

pub fn msg_box_op_1038_c07a(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut local_70c: [u8; 0x200] = [0; 0x200];
    let mut local_50c: [u8; 0x100] = [0; 0x100];
    let mut local_40c: [u8; 0x402] = [0; 0x402];
    let mut uStack10: u32;
    let mut uStack6: u32;

    send_msg_1038_c228(CONCAT22(param_2, param_1));
    uStack6 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    if (param_4 == 0x177) {
        pass1_1008_e05e((param_1 + 0x8e), 0x2, CONCAT22(param_2, param_1 + 0x19e),
                        CONCAT22(param_2, param_1 + 0x9e));
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x200, local_40c, 0x1050);
        sys_1000_3f9c(CONCAT22(0x1050, local_70c), CONCAT22(0x1050, local_40c), param_1 + 0x19e);
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x100, local_50c, 0x1050);
        MessageBox16(0x30, CONCAT22(0x1050, local_50c), CONCAT22(0x1050, local_70c), (param_1 + 0x6));
    } else {
        if (param_4 != 0x178) {
            if ((param_4 != 0x178) && (param_4 - 0x179 < 0x2)) {
                window::set_win_pos_1038_c31a(CONCAT22(param_2, param_1), param_3, param_4);
                return;
            }
            winapp_b::post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4);
            return;
        }
        uStack10 = CONCAT22(param_2, param_1 + 0x9e);
        uVar2 = param_2;
        iVar1 = pass1_1008_e10c((param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e),
                                CONCAT22(param_2, param_1 + 0x9e), param_2, 0x1050);
        if (iVar1 == 0) {
            load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_40c, 0x1050);
            load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_50c, 0x1050);
            MessageBox16(0x30, CONCAT22(0x1050, local_50c), CONCAT22(0x1050, local_40c), (param_1 + 0x6),
            );
            return;
        }
        pass1_1008_e01c((param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e), uStack10);
        pass1_1038_af40(param_1, uVar2, _PTR_LOOP_1050_5b7c, (param_1 + 0x8), 0x1f);
    }
    PostMessage16(0x0, 0x2, 0x111, (param_1 + 0x6));
    return;
}


pub fn win_ui_op_1040_07dc(
    mut param_1: u16,
    pstruct_c_param_2: *mut StructC,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut IVar2: i16;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut puVar8: *mut u32;
    let mut in_stack_0000f69a: u16;
    let mut in_stack_0000f7be: u16;
    let mut in_stack_0000f7c4: u16;
    let mut in_stack_0000f7c8: u16;
    let mut BVar9: bool;
    let mut in_stack_0000f7f2: u16;
    let mut uStack2060: u32;
    let mut local_806: [u8; 0x400] = [0; 0x400];
    let mut local_406: [u32; 0x100] = [0; 0x100];
    let mut uStack6: u32;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    uStack6 = 0;
    if (param_5 == 0x73) {
        win_e::enable_window_1040_0acc(pstruct_c_param_2, 0x0);
        puVar4 = paVar6;
        puVar3 = pass1_1008_5fd8(puVar4);
        uStack2060 = CONCAT22(puVar4, puVar3);
        puVar5 = puVar4;
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_806,
            0x1050,
        );
        IVar2 = MessageBox16(
            0x34,
            CONCAT13(0x10, CONCAT12(0x50, local_806)),
            CONCAT22(puVar4, puVar3),
            HWND16_1050_0396,
        );
        local_406[0] = uStack2060;
        fn_ptr_1000_17ce(CONCAT22(puVar4, puVar3));
        if (IVar2 == 0x6) {
            PostMessage16(0x0, 0xcb, 0x111, HWND16_1050_0396);
            BVar9 = winapp_b::post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, 1);
            uStack6 = CONCAT22(puVar5, BVar9);
        }
    } else {
        if (param_5 < 0x74) {
            if (param_5 == 0x6e) {
                (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
                puVar8 = pass1_1038_af40(
                    pstruct_c_param_2,
                    param_1,
                    _PTR_LOOP_1050_5b7c,
                    (pstruct_c_param_2 + 0x6),
                    0x2,
                );
                ppcVar1 = (*puVar8 + 0x3c);
                (**ppcVar1)(&u16_1050_1038, puVar8, (puVar8 >> 0x10));
                SetFocus16((pstruct_c_param_2 + 0x6));
                return;
            }
            if (0x6e < param_5) {
                //
                // LAB_1040_09f9:
                winapp_b::post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, param_5);
                return;
            }
            if (param_5 == '\x02') {
                //
                // LAB_1040_09b4:
                winapp_b::post_win_msg_1040_7b3c(pstruct_c_param_2, 0x0, 0x0, 0x2);
                PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
                return;
            }
            //      if (param_5 != 'd') goto LAB_1040_09f9;
            PostMessage16(0x0, 0x64, 0x111, HWND16_1050_0396);
            BVar9 = 0;
            // TODO: goto LAB_1040_0821;
        }
        if (param_5 != 0x74) {
            //      if (param_5 == 0xee) goto LAB_1040_09b4;
            if (param_5 == 0x13d) {
                win_e::enable_window_1040_0acc(pstruct_c_param_2, 1);
                return;
            }
            // TODO: goto LAB_1040_09f9;
        }
        win_e::enable_window_1040_0acc(pstruct_c_param_2, 0x0);
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_406,
            0x1050,
        );
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_806,
            0x1050,
        );
        IVar2 = MessageBox16(
            0x34,
            CONCAT13(0x10, CONCAT12(0x50, local_406)),
            CONCAT22(0x1050, local_806),
            HWND16_1050_0396,
        );
        if (IVar2 == 0x6) {
            PostMessage16(0x0, 0x7a, 0x111, HWND16_1050_0396);
            BVar9 = winapp_b::post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, 1);
            uStack6 = CONCAT22(paVar6, BVar9);
            puVar7 = mixed_1010_20ba(
                paVar6,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000f7f2, 0x2),
                in_stack_0000f69a,
                in_stack_0000f7be,
                in_stack_0000f7c4,
                in_stack_0000f7c8,
            );
            pass1_1010_60fa(puVar7);
        }
    }
    BVar9 = 0x1; //
    // LAB_1040_0821:
    win_e::enable_window_1040_0acc(pstruct_c_param_2, BVar9);
    return;
}


pub fn message_box_op_1040_37f0(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut uVar4: u16;
    let mut LVar5: LRESULT;
    let mut in_stack_0000fa94: u16;
    let mut in_stack_0000fbb8: u16;
    let mut in_stack_0000fbbe: u16;
    let mut in_stack_0000fbc2: u16;
    let mut in_stack_0000fbec: u16;
    let mut iVar6: i16;
    let mut local_40c: [u8; 0x402] = [0; 0x402];
    let mut pcStack10: *mut c_char;
    let mut paStack6: *mut Struct27;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 == 0x193) {
        paStack6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fbec, 0x2), in_stack_0000fa94,
                                   in_stack_0000fbb8, in_stack_0000fbbe, in_stack_0000fbc2);
        uVar2 = (paStack6 >> 0x10);
        pcStack10 = *(paStack6 + 0x68);
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_40c, 0x1050);
        uVar1 = MessageBox16(0x30, pcStack10, CONCAT22(0x1050, local_40c), (param_2 + 0x6));
        pass1_1018_3710(uVar1, uVar2, (param_2 + 0x8e));
        PostMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
    } else {
        if (param_5 != 0x194) {
            winapp_b::post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
            return;
        }
        pass1_1038_af40(param_2, param_1, _PTR_LOOP_1050_5b7c, (param_2 + 0x8), 0x21);
        uVar4 = (paVar3 >> 0x10);
        LVar5 = SendMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
        iVar6 = 0x1;
        paStack6 = mixed_1010_20ba(CONCAT22(uVar4, (LVar5 >> 0x10)), _u16_1050_0ed0,
                                   0x1002b, in_stack_0000fa94, in_stack_0000fbb8, in_stack_0000fbbe,
                                   in_stack_0000fbc2);
        pass1_1010_038e(paStack6, iVar6);
    }
    return;
}
