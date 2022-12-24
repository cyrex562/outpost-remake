use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::draw_ops::draw_a::invalidate_rect_1020_735a;
use crate::globals::DAT_1050_1050;
use crate::resources::{load_string_1010_84e0, msg_box_ui_op_1040_64ca};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1000_3000::unk_str_op_1000_3d3e;
use crate::unk::block_1008_9000::pass1_1008_941a;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::unk::block_1018_3000::pass1_1018_31d0;
use crate::unk::block_1030_5000::pass1_1030_5b00;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_4000::{pass1_1040_4d7e, pass1_1040_4dcc};
use crate::unk::block_1040_8000::{pass1_1040_8478, string_1040_8520};
use crate::utils::{CONCAT22, SUB42};
use crate::winapi16::{DestroyWindow16, DispatchMessage16, GetDlgItem16, GetMessage16, IsDialogMessage16, PostMessage16, SendMessage16, SetWindowLong16, TranslateAccelerator16, TranslateMessage16, WinHelp16};
use crate::winapp;
use crate::winapp::winapp_c;
use crate::windef16::{HWND16, LPARAM, LRESULT, MSG16, WPARAM16};

pub unsafe fn unk_destroy_win_op_1020_694c(
    mut param_1: u16,
    param_2: *mut StructA,
    mut param_3: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut HVar4: HWND16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut uVar6: u32;
    let iVar5: *mut StructA;
    let uVar4: *mut StructA;
    let mut unaff_CS: u16;
    let mut wparam: WPARAM16;

    uVar6 = CONCAT22(in_register_0000000a, param_1);
    uVar1 = param_3;
    if (param_3 != 0x12b) {
        iVar5 = param_2;
        uVar4 = (param_2 >> 0x10);
        if (param_3 < 0x12c) {
            if (param_3 == 0x6f) {
                uVar2 = FUN_1010_830a(0x0, uVar6, unaff_CS, _u16_1050_14cc, 0x1f8);
                BVar3 = WinHelp16(0x29, 0x1, CONCAT22(uVar6, uVar2), iVar5.field4_0x8);
                return BVar3;
            }
            if (param_3 == 0xeb) {
                uVar1 = GetDlgItem16(0x1797, iVar5.field4_0x8);
                uVar5 = uVar6;
                if (uVar1 != 0) {
                    //
                    // LAB_1020_6a6f:
                    win_ui_fn_1020_6e98(uVar5, param_2);
                    return uVar1;
                }
            } else {
                uVar1 = param_3 - 0xef;
                if (uVar1 == 0) {
                    pass1_1018_2e28(&iVar5[0x1].field20_0x26);
                    pass1_1008_3e0e(param_2);
                } else {
                    uVar1 = param_3 - 0x129;
                    if ((uVar1 != 0) && (uVar1 = param_3 - 0x12a, uVar1 == 0)) {
                        HVar4 = iVar5.field4_0x8;
                        wparam = 0xf012; //
                        // LAB_1020_69c3:
                        BVar3 = PostMessage16(0x0, wparam, 0x112, HVar4);
                        return BVar3;
                    }
                }
            }
        } else if (param_3 == 0xbb8) {
            HVar4 = GetDlgItem16(0x1797, iVar5.field4_0x8);
            if (HVar4 != 0) {
                DestroyWindow16(HVar4);
            }
            uVar1 = pass1_1018_31d0(&iVar5[0x1].field20_0x26);
            if (uVar1 != 0) {
                uVar1 = pass1_1018_2d9a(&iVar5[0x1].field20_0x26); //
                // LAB_1020_6a0b:
                invalidate_rect_1020_735a(&iVar5[0x1].field22_0x2a);
                return uVar1;
            }
        } else if (param_3 < 0xbb9) {
            if (param_3 == 0x12c) {
                HVar4 = iVar5.field4_0x8;
                wparam = 0xf020;
                // TODO: goto LAB_1020_69c3;
            }
            uVar1 = param_3 - 0x12d;
            if (param_3 != 0x12c) {
                uVar1 = param_3 - 0x12e;
            }
        } else if (param_3 == 0xbb9) {
            HVar4 = GetDlgItem16(0x1797, iVar5.field4_0x8);
            if (HVar4 != 0) {
                DestroyWindow16(HVar4);
            }
            uVar1 = pass1_1018_31d0(&iVar5[0x1].field20_0x26);
            if (uVar1 != 0) {
                uVar1 = pass1_1018_2dde(&iVar5[0x1].field20_0x26);
                // TODO: goto LAB_1020_6a0b;
            }
        } else {
            uVar1 = param_3 - 0xbba;
            if (uVar1 == 0) {
                uVar1 = GetDlgItem16(0x1797, iVar5.field4_0x8);
                uVar5 = uVar6;
                if (uVar1 != 0) {
                    BVar3 = DestroyWindow16(uVar1);
                    return BVar3;
                }
                // TODO: goto LAB_1020_6a6f;
            }
        }
    }
    return uVar1;
}

pub unsafe fn pass1_1008_818c(param_1: *mut astruct_23) {
    let mut BVar1: bool;
    let mut AVar2: ATOM;
    let mut wndclass: WNDCLASS16;

    wndclass.lpsz_class_name = param_1 + 0x4;
    BVar1 = GetClassInfo16(
        &wndclass,
        CONCAT22(wndclass.lpsz_class_name, 0x1050),
        param_1,
    );
    if (BVar1 == 0) {
        wndclass.style = (param_1 + 0x54);
        wndclass.lpfn_wnd_proc = 0x84f2;
        wndclass.lpfn_wnd_proc = 0x1008;
        wndclass._6_4_ = 0x40000;
        wndclass.h_instance = HINSTANCE16_1050_038c;
        wndclass.h_icon = 0;
        wndclass.h_cursor = (param_1 + 0x58);
        wndclass.hbr_background = (param_1 + 0x56);
        wndclass.lpsz_menu_name = 0;
        wndclass.lpsz_class_name = param_1;
        AVar2 = RegisterClass16(&wndclass);
        if (AVar2 == 0) {
            fn_ptr_op_1000_24cd(0x0);
        }
    }
    return;
}

pub unsafe fn win_ui_reg_class_1008_96d2(param_1: *mut StructA) {
    let mut BVar1: bool;
    let mut AVar2: ATOM;
    let mut wndclass: WNDCLASS16;

    wndclass.lpsz_class_name = param_1 + 0x5b;
    BVar1 = GetClassInfo16(
        &wndclass,
        CONCAT22(wndclass.lpsz_class_name, 0x1050),
        param_1,
    );
    if (BVar1 == 0) {
        wndclass.style = (param_1 + 0xc8);
        wndclass.lpfn_wnd_proc = 0x5632;
        wndclass.lpfn_wnd_proc = 0x1008;
        wndclass._6_4_ = 0x40000;
        wndclass.h_instance = HINSTANCE16_1050_038c;
        wndclass.h_icon = (param_1 + 0xc2);
        wndclass.h_cursor = (param_1 + 0xc4);
        wndclass.hbr_background = (param_1 + 0xc6);
        wndclass.lpsz_menu_name = 0;
        wndclass.lpsz_class_name = param_1;
        AVar2 = RegisterClass16(&wndclass);
        if (AVar2 == 0) {
            fn_ptr_op_1000_24cd(0x0);
        }
    }
    return;
}


pub unsafe fn send_win_msg_1040_4a0a(struct_param_1: *mut astruct_48) -> LRESULT

{
    WPARAM16 * pWVar1;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut dlg_item: HWND16;
    let mut uVar5: u16;
    let mut iVar5: *mut astruct_48;
    let mut uVar6: *mut astruct_48;
    let mut lresult_6: LRESULT;
    let mut pcVar6: *mut c_char;
    let mut wparam: WPARAM16;
    let mut UVar7: u16;
    let mut HVar8: HWND16;
    let mut WStack10: WPARAM16;

    uVar6 = (struct_param_1 >> 0x10);
    iVar5 = struct_param_1;
    ppcVar2 = (struct_param_1 + 0x74);
    (**ppcVar2)();
    dlg_item = GetDlgItem16(0x1770, iVar5.hwnd_0x6);
    SendMessage16(0x0, 0x0, 0x40b, dlg_item);
    lresult_6 = SendMessage16(0x0, 0x0, 0xb, dlg_item);
    uVar5 = (lresult_6 >> 0x10);
//   for (WStack10 = 0; uVar3 = iVar5.field143_0x90, pWVar1 = (uVar3 + 0x10),
//       *pWVar1 != WStack10 && WStack10 <= *pWVar1; WStack10 += 1)
    WStack10 = 0;
    uVar3 = iVar5.field143_0x90;
    pWVar1 = uVar3 + 0x10;
    while *pWVar1 != WStack10 && WStack10 <= *pWVar1

    {
        wparam = 0;
        UVar7 = 0x403;
        uVar3 = iVar5.field143_0x90;
        uVar3 = (uVar3 + 0xc);
        HVar8 = dlg_item;
        pcVar6 = pass1_1040_4dcc(uVar5, struct_param_1, (uVar3 + WStack10 * 0x2));
        lresult_6 = SendMessage16(pcVar6, wparam, UVar7, HVar8);
        uVar5 = (lresult_6 >> 0x10);
    }
    pass1_1040_4d7e(struct_param_1);
    if (WStack10 == 0) {
        UVar7 = 0x40a;
        uVar4 = iVar5.field143_0x90;
        uVar3 = iVar5.field144_0x94;
        HVar8 = dlg_item;
        pcVar6 = string_op_1010_ada6(uVar5, uVar3, (uVar3 >> 0x10), 0x0, (uVar4 + 0xa));
        SendMessage16(pcVar6, WStack10, UVar7, HVar8);
    }
    lresult_6 = SendMessage16(0x0, 0x1, 0xb, dlg_item);
    return lresult_6;
}


pub unsafe fn pass1_1040_4cf4(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut hwnd: HWND16;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut LVar9: LRESULT;
    let mut uVar10: u32;
    let mut local_52: [u8; 0x50] = [0; 0x50];

    uVar8 = (in_EDX >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    hwnd = GetDlgItem16(0x1770, (iVar5 + 0x6));
    LVar9 = SendMessage16(0x0, 0x0, 0x407, hwnd);
    uVar4 = CONCAT22(uVar8, (LVar9 >> 0x10));
    if (LVar9 != 0xffff) {
        uVar10 = SendMessage16(CONCAT22(0x1050, local_52), LVar9, 0x408, hwnd);
        uVar4 = uVar4 & 0xffff0000 | uVar10 >> 0x10;
    }
    uVar2 = (iVar5 + 0x90);
    uVar1 = (iVar5 + 0x94);
    uVar3 = pass1_1010_ae12(uVar4, uVar1, (uVar1 >> 0x10), CONCAT22(0x1050, local_52),
                            (uVar2 + 0xa));
    if (uVar3 != 0xffff) {
        uVar1 = (iVar5 + 0x90);
        uVar8 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        uVar10 = (iVar6 + 0x6);
        pass1_1010_ae92((iVar5 + 0x94), uVar10, (iVar6 + 0xa), uVar10, uVar4);
    }
    return;
}

pub unsafe fn get_dlg_item_1040_a3d0(param_1: *mut astruct_49)

{
    let mut lVar1: i32;
    let mut dlg_item: HWND16;
    let mut iVar3: *mut astruct_49;
    let mut uVar2: *mut astruct_49;

    uVar2 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field143_0x90 != 0) {
        lVar1 = iVar3.field143_0x90;
        (lVar1 + 0x14) = iVar3.field6_0x6;
        dlg_item = GetDlgItem16(0x1826, iVar3.field6_0x6);
        winapp_c::win_msg_1040_a308(param_1, 0x0, 0x0, dlg_item);
    }
    return;
}


pub unsafe fn send_msg_1038_ed8a(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u32;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut hwnd: HWND16;
    let mut in_stack_0000ffe2: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    hwnd = HWND16_1050_0396;
    if (param_4 != 0x1c8) {
        if (param_4 == 0x1c9) {
            puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe2, 0x2f), in_stack_0000fe8a,
                                     in_stack_0000ffae, in_stack_0000ffb4, in_stack_0000ffb8);
            uVar2 = (puVar7 >> 0x10);
            uVar5 = (puVar7 + 0x20);
            uVar1 = (puVar7 + 0x22);
            uVar8 = paVar6 & 0xffff0000 | uVar1;
            uVar3 = uVar1 | uVar5;
            if (uVar3 == 0) {
                return;
            }
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar1, uVar5));
            uVar5 = uVar8 | uVar3;
            paVar6 = (uVar8 & 0xffff0000 | uVar5);
            if (uVar5 == 0) {
                return;
            }
            iVar4 = pass1_1030_5b00(CONCAT22(uVar8, uVar3));
            puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe2, iVar4), in_stack_0000fe8a,
                                     in_stack_0000ffae, in_stack_0000ffb4, in_stack_0000ffb8);
            if (((puVar7 >> 0x10) | puVar7) == 0) {
                return;
            }
            uVar8 = pass1_1018_0ad4(puVar7);
            uVar5 = (uVar8 >> 0x10);
            if ((uVar5 | uVar8) == 0) {
                return;
            }
            param_4 = 0x72;
            hwnd = (uVar8 + 0x8);
        } else if (param_4 != 0x1ca) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), (param_3 >> 0x10), param_4, param_4);
            return;
        }
    }
    SendMessage16(0x0, param_4, 0x111, hwnd);
    return;
}


pub unsafe fn def_win_proc_1008_5632(param_1: LPARAM, param_2: WPARAM, mut param_3: u16, param_4: HWND16)

{
  let mut ppcVar1: *mut *mut code;
  let mut puStack6: *mut u32;
  let mut uVar2: u16;

  uVar2 = 0x1050;
  puStack6 = GetWindowLong16(0x0,param_4);
  if (((puStack6 >> 0x10) | puStack6) == 0) {
    if (param_3 != 1) {
      DefWindowProc16(param_1,param_2,param_3,param_4);
      return;
    }
    puStack6 = param_1;
    SetWindowLong16(puStack6,0x0,param_4);
    pass1_1008_9628(puStack6,param_4);
  }
  ppcVar1 = (*puStack6 + 0x1c);
  (**ppcVar1)(0x1538,puStack6,(puStack6 >> 0x10),param_1,param_2,param_3,uVar2);
  return;
}


pub unsafe fn unk_win_msg_op_1008_9510(param_1: *mut i16) {
    let mut has_message: bool;
    let mut IVar1: i16;
    let mut local_14: MSG16; //
                             //
                             // LAB_1008_9578:
    if (*param_1 != 0) {
        has_message = GetMessage16(0x0, 0x0, 0x0, &local_14);
        if (has_message != 0) {
            //   if ((_u16_1050_5bc8 + 0x8) != 0) goto code_r0x10089538;
            // TODO: goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
    has_message = IsDialogMessage16(&local_14, 0x1050);
    if (has_message == 0) {
        //
        // LAB_1008_9547:
        if (PTR_LOOP_1050_0398.is_null() == false) {
            IVar1 =
                TranslateAccelerator16(&local_14, (HACCEL16) & DAT_1050_1050, PTR_LOOP_1050_0398);
            //   if (IVar1 != 0) goto LAB_1008_9578;
        }
        TranslateMessage16(&local_14);
        DispatchMessage16(&local_14);
    }
    //   goto LAB_1008_9578;
}


pub unsafe fn post_win_msg_1008_a0e4(
    param_1: *mut astruct_67,
    param_2: i32,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: i16,
) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut bVar4: bool;
    let mut puVar4: *mut c_char;
    let mut uVar5: *mut astruct_66;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let mut iVar7: *mut astruct_67;
    let mut uVar6: *mut astruct_67;
    let mut paStack14: *mut astruct_99;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar6 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), iVar7.field10_0xa);
    bVar4 = false;
    loop {
        puVar4 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar4));
        // if ((extraout_DX | puVar4) == 0) goto LAB_1008_a146;
        if (puVar4 + 0x4) == param_6 {
            break;
        }
    }
    (puVar4 + 0xc) = (puVar4 + 0xc) + param_3;
    (puVar4 + 0xe) = (puVar4 + 0xe) + param_2;
    bVar4 = true; //
                  // LAB_1008_a146:
    if (!bVar4) {
        paStack14 = pass1_1000_07fc(_PTR_LOOP_1050_03a0);
        uVar7 = (paStack14 >> 0x10);
        uVar3 = paStack14;
        if ((uVar7 | uVar3) == 0) {
            paStack14 = null_mut();
        } else {
            paStack14.field0_0x0 = 0x389a;
            (uVar3 + 0x2) = 0x1008;
            (uVar3 + 0x4) = param_6;
            (uVar3 + 0x6) = param_5;
            (uVar3 + 0xa) = param_4;
            (uVar3 + 0xc) = param_3;
            (uVar3 + 0xe) = param_2;
            paStack14.field0_0x0 = 0xad8e;
            (uVar3 + 0x2) = 0x1008;
        }
        puVar1 = iVar7.field10_0xa;
        ppcVar2 = (*iVar7.field10_0xa + 0x8);
        (**ppcVar2)(
            0x1000,
            puVar1,
            (puVar1 >> 0x10),
            paStack14,
            (paStack14 >> 0x10),
        );
    }
    if (param_6 == 0x14) {
        PostMessage16(0x0, 0xfc, 0x111, HWND16_1050_0396);
    }
    return;
}

pub unsafe fn post_win_msg_1020_291a(mut param_1: u32) {
    PostMessage16(0x0, 0x0, 0x10, (param_1 + 0x8));
    return;
}


pub unsafe fn post_msg_1020_55b0(
    param_1: *mut StructD,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: i16,
) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut pcVar8: *mut c_char;
    let mut puVar6: *mut u32;
    let mut LVar9: LRESULT;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fd80: u16;
    let mut in_stack_0000fd82: u16;
    let mut in_stack_0000fea4: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb0: u16;
    let mut paStack288: *mut Struct57;
    let mut puStack284: *mut u32;
    let mut local_114: WPARAM16;
    let mut local_112: [u8; 0x2] = [0; 0x2];
    let mut iStack272: i16;
    let mut local_10e: i16;
    let mut local_10c: [u8; 0x100] = [0; 0x100];
    let mut puStack12: *mut u32;
    let mut iStack8: i16;
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2),
        in_stack_0000fd80,
        in_stack_0000fea4,
        in_stack_0000feaa,
        in_stack_0000feae,
    );
    paVar4 = (param_1 & 0xffff0000 | puStack6 >> 0x10);
    iStack8 = (puStack6 + 0x20);
    puStack12 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x37),
        in_stack_0000fd80,
        in_stack_0000fea4,
        in_stack_0000feaa,
        in_stack_0000feae,
    );
    uVar6 = (paVar4 >> 0x10);
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x100,
        local_10c,
        0x1050,
    );
    puVar7 = pass1_1008_9436(CONCAT22(0x1050, local_112));
    pcVar8 = pass1_1008_a8f4(
        (puVar7 >> 0x10),
        puStack12,
        CONCAT22(0x1050, &local_114),
        CONCAT22(0x1050, local_112),
        CONCAT22(0x1050, &local_10e),
    );
    uVar2 = pcVar8;
    paVar4 = CONCAT22(uVar6, (pcVar8 >> 0x10) | uVar2);
    uVar5 = (param_2 >> 0x10);
    if ((pcVar8.is_null() == false) && (*pcVar8 != '\0')) {
        uVar6 = 0x1000;
        mem_op_1000_179c(0xb4, paVar4);
        paStack288 = CONCAT22(paVar4, uVar2);
        uVar2 = paVar4 | uVar2;
        paVar4 = (paVar4 & 0xffff0000);
        if (uVar2 == 0) {
            puVar6 = 0;
        } else {
            uVar6 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            puVar6 = pass1_1040_8478(
                uVar2,
                paStack288,
                0x0,
                CONCAT13(0x10, CONCAT12(0x50, local_10c)),
                pcVar8,
                (param_2 + 0x8),
            );
            paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
            puVar6 = SUB42(puVar6, 0x0);
        }
        uVar3 = SUB42(paVar4, 0x0);
        puStack284 = CONCAT22(uVar3, puVar6);
        if (iStack272 == 0) {
            ppcVar1 = (*puStack284 + 0x74);
            (**ppcVar1)(uVar6, puVar6, uVar3);
        } else {
            ppcVar1 = (*puStack284 + 0x6c);
            (**ppcVar1)(uVar6, puVar6, uVar3, local_112, 0x1050);
        }
        if ((iStack8 == 0) || (local_114 == 0)) {
            PostMessage16(0x0, 0xfc, 0x111, HWND16_1050_0396);
        }
    }
    if ((iStack8 != 0) && (local_114 != 0)) {
        LVar9 = SendMessage16(0x0, local_114, 0x111, HWND16_1050_0396);
        (param_2 + 0x112) = 0x1;
        return (LVar9 >> 0x10);
    }
    if (local_10e == 0x6) {
        PostMessage16(0x0, 0xb0, 0x111, HWND16_1050_0396);
        puVar10 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(param_4, 0x2),
            in_stack_0000fd82,
            in_stack_0000fea6,
            in_stack_0000feac,
            in_stack_0000feb0,
        );
        paVar4 = (paVar4 & 0xffff0000 | puVar10 >> 0x10);
        param_4 = puVar10;
        (param_4 + 0x20) = 0x1;
    }
    if (local_10e == 0x15) {
        PostMessage16(0x0, 0x97, 0x111, HWND16_1050_0396);
        puVar10 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(param_4, 0x2),
            in_stack_0000fd82,
            in_stack_0000fea6,
            in_stack_0000feac,
            in_stack_0000feb0,
        );
        paVar4 = (puVar10 >> 0x10);
        (puVar10 + 0x20) = 0x1;
    }
    return paVar4;
}

pub unsafe fn post_win_msg_1020_79fc(
    param_1: *mut astruct_69,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut iVar3: i16;
    let mut iVar4: *mut astruct_69;
    let mut uVar4: u16;
    let mut uVar5: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar5 = (iVar4.field223_0xe0 >> 0x10);
    ppcVar2 = (*iVar4.field223_0xe0 + 0x24);
    iVar3 = (**ppcVar2)();
    if (iVar3 != param_4) {
        PostMessage16(0x0, 0x0, 0x85, iVar4.field8_0x8);
        puVar1 = iVar4.field223_0xe0;
        ppcVar2 = (*iVar4.field223_0xe0 + 0x28);
        (**ppcVar2)(
            0x1538,
            puVar1,
            (puVar1 >> 0x10),
            param_4,
            uVar5,
        );
    }
    return;
}


pub unsafe fn post_msg_1028_76da() {
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffe4: u32;
    let mut uStack10: u16;
    let mut uStack8: u16;

    puVar3 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x2c),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar2 = (puVar3 >> 0x10);
    lVar1 = (puVar3 + 0xc);
    uStack8 = (lVar1 >> 0x10);
    uStack10 = lVar1;
    if (((uStack8 | uStack10) != 0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
        PostMessage16(0x0, 0x106, 0x111, HWND16_1050_0396);
        (puVar3 + 0xc) = 0;
    }
    return;
}


pub unsafe fn post_win_msg_1038_dcb0(mut param_1: u16, param_2: *mut Struct57, mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u32;
  let mut puVar9: *mut u16;
  let mut paVar10: *mut astruct_67;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u8;
  let mut uVar15: u8;
  let mut local_18: u16;
  let mut uStack22: u16;
  let mut local_14: [u8;0x4] = [0;0x4];
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut puStack6: *mut u32;

  mem_op_1000_179c(0xb4,param_2);
  uStack14 = param_2;
  uVar8 = param_2 & 0xffff0000;
  uVar6 = uVar8 | (uStack14 | param_1);
  iVar4 = param_3;
  uVar5 = (param_3 >> 0x10);
  uStack16 = param_1;
  if ((uStack14 | param_1) == 0) {
    iVar2 = 0;
  }
  else {
    iVar2 = string_1040_8520(uVar6,CONCAT22(uStack14,param_1),(iVar4 + 0x6),0x30004,0x7260634);
    uVar8 = uVar6;
  }
  puStack6 = CONCAT22(uVar8,iVar2);
  puVar9 = pass1_1008_941a(CONCAT22(0x1050,local_a),0x1,0x49);
  paVar7 = (uVar8 & 0xffff0000 | puVar9 >> 0x10);
  ppcVar1 = (*puStack6 + 0x6c);
  uVar3 = (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_a,0x1050);
  uStack12 = uVar3;
  if (uVar3 == 0x6) {
    mem_op_1000_179c(0xb4,paVar7);
    uStack14 = paVar7;
    uVar8 = paVar7 & 0xffff0000;
    uVar6 = uVar8 | (uStack14 | uVar3);
    uStack16 = uVar3;
    if ((uStack14 | uVar3) == 0) {
      iVar4 = 0;
    }
    else {
      iVar4 = string_1040_8520(uVar6,CONCAT13((paVar7 >> 0x8),CONCAT12(paVar7,uVar3)),
                               (iVar4 + 0x6),0x20000,0x7280634);
      uVar8 = uVar6;
    }
    puStack6 = CONCAT22(uVar8,iVar4);
    puVar9 = pass1_1008_941a(CONCAT22(0x1050,local_14),0x1,0x4a);
    paVar7 = (uVar8 & 0xffff0000 | puVar9 >> 0x10);
    ppcVar1 = (*puStack6 + 0x6c);
    (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_14,0x1050);
    uVar14 = 0;
    uVar15 = 0;
    iVar2 = 0x15;
    uVar12 = 0x1;
    uVar13 = 0;
    uVar11 = 0;
    iVar4 = 0;
    uVar5 = 0;
    paVar10 =
              mixed_1010_20ba(paVar7,_u16_1050_0ed0,0x37,in_stack_0000fe72,in_stack_0000ff96,
                              in_stack_0000ff9c,in_stack_0000ffa0);
    uStack22 = (paVar10 >> 0x10);
    local_18 = SUB42(paVar10,0x0);
    post_win_msg_1008_a0e4(paVar10,CONCAT22(uVar11,uVar5),iVar4,uVar12,CONCAT13(uVar15,CONCAT12(uVar14,uVar13)),iVar2);
    PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    return;
  }
  mem_op_1000_179c(0xb4,paVar7);
  uStack14 = paVar7;
  uVar8 = paVar7 & 0xffff0000 | (uStack14 | uVar3);
  uStack16 = uVar3;
  if ((uStack14 | uVar3) == 0) {
    iVar4 = 0;
    uVar5 = 0;
  }
  else {
    iVar4 = string_1040_8520(uVar8,CONCAT13((paVar7 >> 0x8),CONCAT12(paVar7,uVar3)),
                             (iVar4 + 0x6),0x20000,0x7290634);
    uVar5 = uVar8;
  }
  puStack6 = CONCAT22(uVar5,iVar4);
  pass1_1008_941a(CONCAT22(0x1050,&local_18),0x1,0x4b);
  ppcVar1 = (*puStack6 + 0x6c);
  (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),&local_18,0x1050);
  return;
}


pub unsafe fn post_win_msg_1040_7b3c(param_1: *mut StructC, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;

  if ((param_4 == 1) || (param_4 == 0x2)) {
    ppcVar1 = (param_1 + 0x14);
    (**ppcVar1)();
  }
  else if (param_4 == 0x6f) {
    ppcVar1 = (param_1 + 0x2c);
    (**ppcVar1)();
  }
  else {
    if (param_4 != 0x12e) {
      return 0x0;
    }
    PostMessage16(0x0,0xf060,0x112,(param_1 + 0x6));
  }
  return 0x1;
}

pub unsafe fn destroy_win_1038_a3d2(mut param_1: u32)

{
  let mut hwnd: HWND16;
  hwnd = GetWindowWord16(-0x8,(param_1 + 0x6));
  PostMessage16(0x0,0x105,0x111,hwnd);
  destroy_win_1040_7b98(param_1);
  return;
}

pub unsafe fn post_win_msg_1038_d840(param_1: *mut astruct_70, mut param_2: u16)

{
    let mut iVar1: *mut astruct_70;
    let mut uVar1: *mut astruct_70;

    iVar1 = param_1;
    uVar1 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if (iVar1.field142_0x8e != 0) {
            PostMessage16(0x0, iVar1.field142_0x8e, 0x111, &iVar1.field_0x6);
            iVar1.field142_0x8e = 0;
            return;
        }
    } else if (param_2 < 0x11) {
        if (param_2 == '\x01') {
            iVar1.field143_0x90 = 0;
            iVar1.field144_0x92 = 0;
            return;
        }
        if (param_2 == '\x03') {
            pass1_1038_e03e(param_1);
            return;
        }
    }
    return;
}


pub unsafe fn post_win_msg_1040_0d5e(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    if (param_3 != 0) {
        PostMessage16(0x0, 0x1, 0x111, (param_1 + 0x8));
    }
    return;
}


pub unsafe fn post_win_msg_1040_672e(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut iVar1: i16;

    if (param_5 == s_vrpal_bmp_1050_183a + 0x7) {
        msg_box_ui_op_1040_64ca(0x0, param_1, CONCAT22(param_3, param_2));
    } else {
        if (param_5 == 0x1851) {
            iVar1 = 0x2a;
        } else {
            if (param_5 != 0x1852) {
                post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
                return;
            }
            iVar1 = 0x29;
        }
        pass1_1038_af40(param_2, param_1, _PTR_LOOP_1050_5b7c, (param_2 + 0x8), iVar1);
        PostMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
    }
    return;
}


pub unsafe fn post_win_msg_1040_7f1c(mut param_1: u32, mut param_2: i16) -> BOOL16

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x78) != 0) {
        return 0x0;
    }
    if ((iVar1 + 0x60) != param_2) {
        (iVar1 + 0x60) = param_2;
        PostMessage16(0x0, 0x0, 0x85, (iVar1 + 0x6));
    }
    return 0x1;
}


pub unsafe fn post_win_msg_1040_7f56(mut param_1: u32, param_2: *mut c_char)

{
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x10)), param_2);
    PostMessage16(0x0, 0x0, 0x85, (param_1 + 0x6));
    return;
}


pub unsafe fn post_msg_1008_3d20(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    PostMessage16(0x0, (param_1 + 0xcc), 0x111, (param_1 + 0xbc));
    return;
}

pub unsafe fn invalidate_rect_1018_58e2(param_1: *mut astruct_58, mut param_2: i16) {
    let mut piVar1: *mut i16;
    let mut iVar2: *mut astruct_58;
    let mut uVar2: u16;

    if (param_2 == 0x105) {
        uVar2 = (param_1 >> 0x10);
        iVar2 = param_1;
        piVar1 = &iVar2.field245_0xf6;
        *piVar1 = *piVar1 + 1;
        if (u16_1050_4240 <= iVar2.field245_0xf6) {
            PostMessage16(0x0, 0xca, 0x111, HWND16_1050_0396);
            return;
        }
        iVar2.field234_0xea = 0;
        InvalidateRect16(0x0, NULL, 0x0);
    }
    return;
}

pub unsafe fn post_win_msg_1018_ea0a(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    if (param_3 == 0xed) {
        PostMessage16(0x0, 0xc6, 0x111, HWND16_1050_0396);
    }
    return;
}

pub unsafe fn post_msg_1020_03d6(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn post_msg_1020_03fa(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn post_win_msg_1020_061c(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x6) = 0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    uVar1 = (param_1 + 0x6);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}
