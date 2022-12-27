use std::ptr::null_mut;
use std::os::raw::c_char;
use crate::draw_ops::draw_e::ui_cleanup_op_1040_782c;
use crate::draw_ops::draw_a::set_sys_color_1008_357e;
use crate::structs::Struct1000::Struct1000;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_65::Struct65;
use crate::structs::struct_872::Struct872;
use crate::structs::struct_873::Struct873;
use crate::structs::struct_874::Struct874;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::fn_ptr_1000_17ce;
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12};
use crate::unk::block_1008_b000::pass1_1008_b544;
use crate::unk::block_1010_1000::{pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_1f62};
use crate::unk::block_1010_3000;
use crate::unk::block_1020_0000::pass1_1020_022c;
use crate::unk::block_1020_8000::pass1_1020_808e;
use crate::unk::block_1038_b000::pass1_1038_b6e0;
use crate::unk::block_1040_c000::pass1_1040_c60e;
use crate::utils::{CONCAT22, SUB42};
use crate::gui;
use crate::gui::window;
use crate::gui::window::win_c;
use crate::winapi16::{DestroyCursor16, DestroyIcon16, DestroyMenu16, DestroyWindow16, GetWindowWord16, IsDlgButtonChecked, IsWindow16, PostMessage16, SendMessage16};
use crate::windef16::{BOOL16, HWND16};

pub fn destroy_cursor_1020_42f4(param_1: *mut StructD) {
    let mut struct_1: *mut StructD;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    struct_1 = param_1;
    param_1.address_offset_field_0x0 = 0x623c;
    struct_1.address_offset_field_0x2 = 0x1020;
    struct_1.field_0xe2 = 0x62d8;
    struct_1.field_0xe4 = 0x1020;
    if struct_1[0x1].field13_0x18 != 0 {
        DestroyMenu16(struct_1[0x1].field13_0x18);
    }
    DestroyCursor16(struct_1[0x1].address_offset_field_0x2);
    DestroyCursor16(struct_1[0x1].hfile_0x4);
    pass1_1020_808e(param_1);
    return;
}

pub fn unk_destroy_window_op_1018_6bb6(param_1: *mut astruct_28) {
    let mut b_result: bool;
    let mut struct_1: *mut astruct_28;
    let mut u_var2: *mut astruct_28;

    u_var2 = (param_1 >> 0x10);
    struct_1 = param_1;
    if struct_1.field229_0xea != 0 {
        PostMessage16(0x0, struct_1.field229_0xea, 0x111, HWND16_1050_0396);
    }
    PostMessage16(0x0, 0x79, 0x111, HWND16_1050_0396);
    if struct_1.hwnd_0xf0 != 0 {
        b_result = IsWindow16(struct_1.hwnd_0xf0);
        if b_result != false {
            DestroyWindow16(struct_1.hwnd_0xf0);
            struct_1.hwnd_0xf0 = 0;
        }
    }
    return;
}

pub fn destroy_window_1018_c518(param_1: *mut astruct_29) {
    let mut is_window: bool;
    let mut pstruct_29_1: *mut astruct_29;
    let mut pstruct_29_hi: *mut astruct_29;

    pstruct_29_hi = (param_1 >> 0x10);
    pstruct_29_1 = param_1;
    param_1.field0_0x0 = 0xc8bc;
    pstruct_29_1.field1_0x2 = 0x1018;
    fn_ptr_1000_17ce(pstruct_29_1.field259_0x108);
    if pstruct_29_1.hwnd_0x112.is_null() == false {
        is_window = IsWindow16(pstruct_29_1.hwnd_0x112);
        if is_window != false {
            DestroyWindow16(pstruct_29_1.hwnd_0x112);
            pstruct_29_1.hwnd_0x112 = null_mut();
        }
    }
    pass1_1020_022c(param_1);
    return;
}

pub fn destroy_win_1020_1dea(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
) -> BOOL16 {
    let mut bvar1: bool;
    let mut wvar2: u16;

    bvar1 = IsWindow16(param_3);
    if bvar1 != false {
        wvar2 = GetWindowWord16(-0xc, param_3);
        if wvar2 == 0x175 {
            DestroyWindow16(param_3);
            return 0x0;
        }
    }
    return 0x1;
}


pub fn destroy_win_1040_5256(param_1: *mut astruct_34)

{
    let mut p_uvar1: *mut u32;
    let mut bool3: bool;
    let mut pstruct34_5: *mut astruct_34;
    let mut pstruct34_hi: *mut astruct_34;
    let mut unaff_cs: u16;
    let mut u_var2: u16;
    let mut fn_ptr_1: *mut *mut code;

    pstruct34_hi = (param_1 >> 0x10);
    pstruct34_5 = param_1;
    if pstruct34_5.hwnd_0xb6 != 0 {
        // 0x1538
        unaff_cs = 0x1538;
        bool3 = IsWindow16(pstruct34_5.hwnd_0xb6);
        if bool3 != false {
            // 0x1538
            unaff_cs =0x1538;
            DestroyWindow16(pstruct34_5.hwnd_0xb6);
        }
    }
    pstruct34_5.hwnd_0xb6 = 0;
    p_uvar1 = pstruct34_5.field148_0x94;
    u_var2 = pstruct34_5.field149_0x96;
    if (u_var2 | p_uvar1) != 0 {
        // TODO: fix getting function pointer
        // fn_ptr_1 = *p_uvar1;
        // (**fn_ptr_1)(unaff_cs, p_uvar1, u_var2, 1);
    }
    pstruct34_5.field148_0x94 = 0;
    pstruct34_5.field150_0x98 = 0;
    return;
}


pub fn destroy_win_1040_bb78(param_1: *mut astruct_35)

{
    let mut u_var1: u16;
    let mut is_window: bool;
    let mut pstruct35_5: *mut astruct_35;
    let mut pstruct35_hi: *mut astruct_35;
    let mut unaff_cs: u16;
    let mut pu_var1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    pstruct35_hi = (param_1 >> 0x10);
    pstruct35_5 = param_1;
    if pstruct35_5.hwnd_0xb6 != 0 {
        // 0x1538
        unaff_cs = 0x1538;
        is_window = IsWindow16(pstruct35_5.hwnd_0xb6);
        if is_window !=false {
            // 0x1538
            unaff_cs = 0x1538;
            DestroyWindow16(pstruct35_5.hwnd_0xb6);
        }
    }
    pstruct35_5.hwnd_0xb6 = 0;
    pu_var1 = pstruct35_5.field148_0x94;
    u_var1 = pstruct35_5.field149_0x96;
    if (u_var1 | pu_var1) != 0 {
        // TODO: fix function pointer
        // fn_ptr_1 = *pu_var1;
        // (**fn_ptr_1)(unaff_cs, pu_var1, u_var1, 1);
    }
    pstruct35_5.field148_0x94 = 0;
    pstruct35_5.field150_0x98 = 0;
    return;
}

pub fn destroy_win_1008_628e(mut param_1: *mut Struct1000) {
    // fn_ptr_1 = ((param_1 + 0xd2) + 0x14);
    let mut fn_ptr_1: fn() = param_1.field_0xd2 + 0x14;
    // (**fn_ptr_1)();
    fn_ptr_1();
    DestroyWindow16(param_1.win_handle_0x8);
    // (param_1 + 0x8) = 0;
    param_1.win_handle_0x8 = 0;
    return;
}


pub fn unk_destroy_win_op_1010_2fa0(pstruct_873_param_1: *mut Struct873) {
    let mut pstruct873_1 = pstruct_873_param_1;
    pstruct873_1.field39_0x28 = 0;
    let mut pstruct872_2: *mut Struct872 = null_mut();
    loop {
        let mut pstruct872_1 = pstruct873_1.pstruct872_0x16;
        if pstruct872_1 == pstruct872_2 || pstruct872_1 < pstruct872_2 {
            break;
        }
        let mut u_var3 = (pstruct873_1.field_0x2a + pstruct872_2[0x4]);
        DestroyWindow16((u_var3 + 0x18));
        pstruct872_2 = pstruct872_2 + 1;
    }
    pstruct873_1.pstruct872_0x16 = null_mut();
    if (pstruct873_1.field82_0x54 | pstruct873_1.field_0x52) != 0 {
        pstruct872_2 = null_mut();
        loop {
            let mut u_var2 = pstruct873_1.field_0x52;
            if (u_var2 + pstruct872_2[4]) != 0 {
                u_var2 = pstruct873_1.field_0x52;
                u_var2 = (u_var2 + pstruct872_2[4]);
                DestroyWindow16((u_var2 + 0x18));
                u_var2 = pstruct873_1.field_0x52;
                (u_var2 + pstruct872_2 * 0x4) = 0;
            }
            pstruct872_2 = pstruct872_2 + 1;
            if pstruct872_2 >= 0xa {
                break;
            }
        }
        fn_ptr_1000_17ce(pstruct873_1.field_0x52);
        pstruct873_1.field_0x52 = 0;
    }
    return;
}

pub fn unk_destroy_win_op_1010_305a(
    mut param_1: u16,
    param_2: *mut Struct27,
    mut param_3: i16,
    param_4: *mut Struct65,
) {
    // Struct874 * *ppa_var1;
    let mut ppstruct874_var1: *mut *mut Struct874;
    let mut pi_var2: *mut i16;
    let mut uvar3: u32;
    let mut l_var4: i32;
    let mut u_var5: u32;
    let mut b_var6: bool;
    let mut u_var8: u16;
    let mut pstruct27_var4: *mut Struct27;
    let mut pstruct874_var9: *mut Struct874;
    let mut u_var7: *mut Struct27;
    let mut u_var10: u16;
    let mut i_stack10: i16;
    let mut pstruct874_var8: *mut Struct874;
    let mut pa_stack6: *mut Struct874;
    let mut i_var7: *mut Struct874;

    u_var8 = pass1_1040_c60e(param_4);
    pstruct27_var4 = param_2;
    pstruct27_var4.field18_0x12 = u_var8;
    pstruct27_var4.field19_0x14 = 0;
    pa_stack6 = null_mut();
    b_var6 = false;
    pstruct27_var4.field33_0x28 = 0;
    pstruct874_var8 = null_mut();
    loop {
        ppstruct874_var1 = &mut pstruct27_var4.pstruct874_0x16;
        if *ppstruct874_var1 == pstruct874_var8 || *ppstruct874_var1 < pstruct874_var8 {
// TODO:
// LAB_1010_30ad:
            i_var7 = pa_stack6;
            if b_var6 {
                while (
                    pstruct874_var8 = i_var7 + 0x1,
                    ppstruct874_var1 = &mut pstruct27_var4.pstruct874_0x16,
                    *ppstruct874_var1 != pstruct874_var8 && pstruct874_var8 <= *ppstruct874_var1,
                ) {
                    uvar3 = (pstruct27_var4.field36_0x2e)[i_var7];
                    DestroyWindow16((uvar3 + 0x18));
                    (&pstruct27_var4.field36_0x2e)[i_var7] = 0;
                    i_var7 = pstruct874_var8;
                }
                pstruct27_var4.pstruct874_0x16 = (pa_stack6 + 1);
                pass1_1010_1f62(param_2, 0x9);
            } else {
                pstruct874_var9 = pstruct27_var4.pstruct874_0x16;
                (&pstruct27_var4.field34_0x2a)[pstruct874_var9 * 0x2] = param_4;
                (&pstruct27_var4.field35_0x2c)[pstruct874_var9 * 0x2] = (param_4 >> 0x10);
                i_stack10 = 0xa;
                pi_var2 = &pstruct27_var4.pstruct874_0x16;
                *pi_var2 = *pi_var2 + 1;
                if (0x1 < pstruct27_var4.pstruct874_0x16) {
                    uvar3 = (&pstruct27_var4.field30_0x22)[pstruct27_var4.pstruct874_0x16];
                    pstruct874_var9 = uvar3;
                    // u_var10 = (uvar3 >> 0x10);
                    i_stack10 = (pstruct874_var9.field_0x20) + (pstruct874_var9 + 0x24) + 0x8;
                }
                win_c::mov_update_win_1040_93aa(param_4, i_stack10, pstruct27_var4.field23_0x1a);
            }
            if !b_var6 {
                pass1_1010_1f62(param_2, 0xa);
            }
            if param_3 == 0 {
                if pstruct27_var4.field69_0x52 != 0 {
                    pstruct874_var8 = null_mut();
                    loop {
                        l_var4 = pstruct27_var4.field69_0x52;
                        // u_var10 = (l_var4 >> 0x10);
                        pstruct874_var9 = l_var4;
                        if ((pstruct874_var9 + pstruct874_var8[4]) != 0)
                            && ((pstruct874_var9 + pstruct874_var8[4]) != param_4)
                        {
                            l_var4 = pstruct27_var4.field69_0x52;
                            u_var5 = (l_var4 + pstruct874_var8[4]);
                            DestroyWindow16((u_var5 + 0x18));
                        }
                        l_var4 = pstruct27_var4.field69_0x52;
                        (l_var4 + pstruct874_var8[4]) = 0;
                        pstruct874_var8 = (pstruct874_var8 + 1);
                        if pstruct874_var8.field_0x0 >= 0xa {
                            break;
                        }
                    }
                }
                block_1010_3000::pass1_1010_32da(param_2, param_4);
                pass1_1010_1f62(param_2, 0x8);
            }
            return;
        }
        if (&pstruct27_var4.field34_0x2a + pstruct874_var8 * 0x2) == param_4 {
            b_var6 = true;
            pa_stack6 = pstruct874_var8;
            // TODO: goto LAB_1010_30ad;
        }
        pstruct874_var8 = pstruct874_var8 + 1;
    }
}

pub fn destroy_win_1020_1e1e(mut param_1: u16, mut param_2: u16, param_3: HWND16) -> u16 {
    let mut BVar1: bool;
    let mut WVar2: u16;

    BVar1 = IsWindow16(param_3);
    if (BVar1 != 0) {
        WVar2 = GetWindowWord16(-0xc, param_3);
        if ((WVar2 != 1) && (WVar2 != 0x175)) {
            DestroyWindow16(param_3);
        }
    }
    return 0x1;
}

pub fn destroy_window_1020_8250(param_1: *mut astruct_879) {
    let mut BVar1: bool;
    let mut iVar2: *mut astruct_879;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if iVar2.field236_0xec != 0 {
        BVar1 = IsWindow16(iVar2.field236_0xec);
        if (BVar1 != 0) {
            DestroyWindow16(iVar2.field236_0xec);
            iVar2.field236_0xec = 0;
        }
    }
    return;
}

pub fn destroy_window_1020_3b3e(param_1: *mut astruct_30) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut lVar4: i32;
    let mut puVar5: *mut u8;
    let mut paVar6: *mut astruct_30;
    let mut struct_5: *mut astruct_30;
    let mut struct_6: *mut astruct_30;
    let mut unaff_CS: u16;

    struct_6 = (param_1 >> 0x10);
    struct_5 = param_1;
    struct_5.field262_0x10e = 0;
    if (struct_5.field261_0x10a != 0) {
        lVar4 = struct_5.field261_0x10a;
        // 0x1538
        unaff_CS = SUB42(0x1538, 0x0);
        DestroyWindow16((lVar4 + 0x6));
    }
    puVar1 = struct_5.field246_0xf6;
    uVar2 = struct_5.field247_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(unaff_CS, puVar1);
    }
    struct_5.field246_0xf6 = 0;
    if (struct_5.field248_0xfa != 0) {
        puVar5 = (struct_6 | struct_5);
        if (param_1.is_null()) {
            paVar6 = null_mut();
        } else {
            puVar5 = &struct_5.field_0xf2;
            paVar6 = struct_6;
        }
        pass1_1010_1ea6(struct_5.field248_0xfa, CONCAT22(paVar6, puVar5));
    }
    struct_5.field248_0xfa = 0;
    return;
}


pub fn destroy_win_1038_ef3a(param_1: *mut StructD)

{
  let mut uVar2: u32;
  let mut iVar1: *mut StructD;
  let mut uVar1: *mut StructD;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x67c;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&iVar1.field_0x96 != 0) {
    uVar2 = &iVar1.field_0x96;
    DestroyWindow16((uVar2 + 0x6));
    iVar1.field_0x96 = 0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}

pub fn destroy_win_1040_7b98(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x74) == 0) {
    DestroyWindow16((param_1 + 0x6));
  }
  return;
}


pub fn destroy_win_1040_8b7e(mut param_1: u32)

{
  DestroyWindow16((param_1 + 0x6));
  return;
}

pub fn destroy_window_1040_b726(mut param_1: u32, mut param_2: i16)

{
  let mut fn_ptr_1: *mut *mut code;

  if (param_2 != 0) {
    fn_ptr_1 = (param_1 + 0x78);
    (**fn_ptr_1)();
  }
  DestroyWindow16((param_1 + 0x6));
  return;
}

pub fn destroy_window_1038_7d88(mut param_1: u32, mut param_2: u16, mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  pass1_1008_b544(param_3,(param_1 + 0x94),param_2);
  DestroyWindow16((param_1 + 0x6));
  return;
}

pub fn destroy_window_1038_a072(param_1: *mut astruct_880, mut param_2: u16, mut param_3: i16)

{
  if (param_3 != 0) {
    DestroyWindow16(param_1.field6_0x6);
  }
  return;
}

pub fn destroy_win_1008_9698(param_1: *mut astruct_871, mut param_2: u16) {
    DestroyWindow16(param_1.hwnd_0x8);
    return;
}


pub fn destroy_window_1010_7b26(mut param_1: u16, mut param_2: u32, param_3: i32) -> u32 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut UpuVar2: *mut c_char;
    let mut extraout_DX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    uVar2 = (iVar3 + 0x1e) | (iVar3 + 0x1c);
    if (uVar2 != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar3 + 0x1c));
        loop {
            puVar2 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, puVar2));
            param_1 = extraout_DX | puVar2;
            if (param_1 == 0) {
                break;
            }
            if !((puVar2 + 0x4) != param_3) {
                break;
            }
        }
        uVar2 = extraout_DX | puVar2;
        if (uVar2 != 0) {
            uVar1 = (puVar2 + 0x8);
            uVar2 = DestroyWindow16((uVar1 + 0x6));
        }
    }
    return CONCAT22(uVar2, param_1);
}

pub fn destroy_win_1038_e1dc(param_1: *mut astruct_886, mut param_2: u16, mut param_3: i16)

{
    let mut UVar1: u16;
    let mut uVar2: u32;

    if (param_3 != 0) {
        UVar1 = IsDlgButtonChecked(0x1807, param_1.field6_0x6);
        if (UVar1 == 0) {
            UVar1 = IsDlgButtonChecked(0x1806, param_1.field6_0x6);
//      if (UVar1 == 0) goto LAB_1038_e229;
            uVar2 = 0x1110130;
        } else {
            uVar2 = 0x111012f;
        }
        SendMessage16(0x0, uVar2, (uVar2 >> 0x10), HWND16_1050_0396);
    }//
// LAB_1038_e229:
    DestroyWindow16(param_1.field6_0x6);
    return;
}

pub fn cleanup_ui_op_1008_0618(param_1: *mut astruct_53) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_53;
    let mut uVar3: u16;
    let mut puVar1: *mut u32;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1 = 0x389e;
    iVar4.field_0x2 = 0x1008;
    set_sys_color_1008_357e(param_1, 0x0, in_EDX);
    fn_ptr_1000_17ce(*&iVar4.field248_0xf8);
    if (&iVar4.field_0xec != 0) {
        DestroyMenu16(&iVar4.field_0xec);
    }
    DestroyIcon16(&iVar4.field_0xc2);
    iVar4.field_0xc2 = 0;
    puVar1 = &iVar4.field_0xe0;
    uVar1 = &iVar4.field_0xe2;
    if ((uVar1 | puVar1) != 0) {
        ppcVar2 = *puVar1;
        (**ppcVar2)(0x1538, puVar1, uVar1, 1);
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
    param_1 = 0x380a;
    iVar4.field_0x2 = 0x1008;
    param_1 = 0x389a;
    iVar4.field_0x2 = 0x1008;
    return;
}

pub fn cleanup_menu_ui_op_1020_795c(in_struct_1: *mut StructD) {
    let mut local_struct_1: *mut StructD;
    let mut uVar1: *mut StructD;

    uVar1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x7b86;
    local_struct_1.address_offset_field_0x2 = 0x1020;
    if (local_struct_1.hmenu_0xec != 0) {
        DestroyMenu16(local_struct_1.hmenu_0xec);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.field192_0xd2)));
    in_struct_1.address_offset_field_0x0 = 0x380a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    return;
}


pub fn cleanup_ui_op_1020_1038(param_1: *mut astruct_868) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_868;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    DestroyIcon16(iVar4.hicon_0xc2);
    iVar4.hicon_0xc2 = 0;
    iVar4.field8_0x8 = 0;
    puVar1 = iVar4.field241_0xf6;
    uVar2 = iVar4.field242_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1538, puVar1, uVar2, 1);
    }
    iVar4.field241_0xf6 = 0;
    pass1_1010_1dda(iVar4.field240_0xf2);
    iVar4.field240_0xf2 = 0;
    return;
}


pub fn destroy_icon_1020_2c88(param_1: *mut astruct_869) {
    let mut ppcVar1: *mut *mut code;
    let mut struct_1: *mut astruct_869;
    let mut uVar3: u16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;

    uVar3 = (param_1 >> 0x10);
    struct_1 = param_1;
    DestroyIcon16(struct_1.field193_0xc2);
    struct_1.field193_0xc2 = 0;
    struct_1.field8_0x8 = 0;
    puVar1 = struct_1.field241_0xf6;
    uVar1 = struct_1.field242_0xf8;
    if ((uVar1 | puVar1) != 0) {
        ppcVar1 = *puVar1;
        (**ppcVar1)(0x1538, puVar1, uVar1, 1);
    }
    struct_1.field241_0xf6 = 0;
    pass1_1010_1dda(struct_1.field240_0xf2);
    struct_1.field240_0xf2 = 0;
    return;
}


pub fn destroy_icon_1020_6bd2(param_1: *mut astruct_868, param_2: u8) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut struct_1: *mut astruct_868;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    struct_1 = param_1;
    DestroyIcon16(struct_1.hicon_0xc2);
    struct_1.hicon_0xc2 = 0;
    struct_1.field8_0x8 = 0;
    puVar1 = struct_1.field241_0xf6;
    uVar2 = struct_1.field242_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1538, puVar1, uVar2, 1);
    }
    struct_1.field241_0xf6 = 0;
    pass1_1010_1dda(struct_1.field240_0xf2);
    struct_1.field240_0xf2 = 0;
    return;
}
