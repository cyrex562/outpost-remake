use std::ptr::null_mut;
use crate::block_1000::block_1000_1000::{msg_box_op_1000_1f24, pass1_1000_1f68};
use crate::block_1000::block_1000_2000::{mem_op_1000_21b6, pass1_1000_25a8, pass1_1000_2913, poss_str_op_1000_28dc};
use crate::block_1000::block_1000_4000;
use crate::globals::{DAT_1050_1050, PTR_LOOP_1050_1000, PTR_LOOP_1050_63fe};
use crate::structs::struct_825::Struct825;
use crate::utils::{CARRY2, CONCAT11, CONCAT22};
use crate::winbase::{FatalAppExit16, FatalExit, GLobalAlloc16, GlobalFree16, GlobalUnlock16};
use crate::windef::HGLOBAL16;

pub unsafe fn _SHI_INVOKEERRORHANDLER1() -> u16
{
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut uVar2: u16;
    let mut pcStack6: *mut code;
    let mut puStack4: *mut u16;
    let mut uVar3: u16;

    puStack4 =  &DAT_1050_1050;
    if (( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0) {
        pcStack6 = null_mut();
        puStack4 = null_mut();
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pcStack6 =  PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0) {
            PTR_PTR_1050_5f1a =  &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c =  &PTR_LOOP_1050_1000;
            pcStack6 =  &PTR_PTR_1050_1f7e;
            puStack4 =  &PTR_LOOP_1050_1000;
        }
    }
    if (( puStack4 |  pcStack6) != 0) {
        BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                      &DAT_1050_1050,
                                     0x0);
        if (BVar2 == 0) {
            uVar2 = (*pcStack6)();
        } else {
            puStack4 = null_mut();
            pcStack6 = null_mut();
            uVar2 = 0;
        }
        if (( puStack4 |  pcStack6) != 0) {
            pass1_1000_1f68();
        }
        return uVar2;
    }
    return 0x0;
}

pub unsafe fn ___EXPORTEDSTUB() -> u16
{
    return 0x0;
}

pub unsafe fn mixed_sys_op_1000_40af(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
) -> *mut i16 {
    let mut ppa_var1: *mut *mut struct_824;
    let mut pc_var2: *mut c_char;
    let mut pu_var4: *mut u16;
    let mut pc_var5: *mut c_char;
    let mut i_var6: i16;
    let mut ppa_var7: *mut *mut struct_824;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut i_var8: i16;
    let mut hglobal_7: HGLOBAL16;
    // pub unsafe fn *SVar8;
    let mut ppa_var8: *mut *mut struct_824;
    let mut unaff_si: i16;
    let mut pi_var9: *mut i16;
    let mut pc_var10: *mut c_char;
    let mut hglobal_di: *mut astruct_824;
    let mut pu_var11: *mut u16;
    let mut unaff_cs: u8;
    let mut unaff_ss: u16;
    let mut b_var12: bool;
    // pub unsafe fn *pvVar13;
    let mut pa_var14: *mut Struct825;
    let mut pu_var3: *mut u16;
    let mut u_var13: u8;
    let mut u_var14: u8;
    let mut i_var12: i16;
    let mut temp_5fa27366cb: *mut astruct_824;

    loop {
        u_var7 = (param_1 * param_3);
        u_var8 = param_2 * param_3 + (param_1 * param_3 >> 0x10);
        if ((u_var8 | u_var7) != 0) {
            pi_var9 = null_mut();
            if ((u_var8 < 0x3) && (u_var8 < 0x2 || (u_var7 == 0))) {
                if (u_var8 == 0) {
                    u_var7 = u_var7 + 0xfff & 0xf000;
                    if (u_var7 == 0) {
                        u_var8 = 0x1;
                    }
                } else if ((param_3 - 0x1 & param_3) != 0) {
                    pi_var9 = ((u_var8 << 0x10) % param_3);
                    b_var12 = CARRY2(u_var7, pi_var9);
                    u_var7 += pi_var9;
                    if (b_var12) {
                        // TODO: goto LAB_1000_41aa;
                    }
                    u_var8 = 0x1;
                }
            } else if ((param_3 - 0x1 & param_3) != 0) {
                // TODO: goto LAB_1000_41aa;
            }
            hglobal_7 = GLobalAlloc16(CONCAT13((u_var8 >> 0x8), CONCAT12(u_var8, u_var7)), 0x20);
            if ((hglobal_7 != 0) && ((u_var7 & 1) != 0)) {
                pvVar13 = WIN16_GlobalLock16(hglobal_7);
                SVar8 = pvVar13;
                if ((SVar8 != 0) || (pvVar13.is_null())) {
                    pa_var14 = CONCAT22(hglobal_7, 0x12);
                    u_var13 = '\x12';
                    u_var14 = '\0';
                    pass1_1000_25a8();
                    pass1_1000_2913(CONCAT11(u_var14, u_var13));
                    pc_var5 = poss_str_op_1000_28dc(pa_var14);
                    if (pc_var5.is_null()) {
                        // TODO: goto LAB_1000_28cb;
                    }
                    i_var6 = 0x9;
                    if (*pc_var5 == 'M') {
                        i_var6 = 0xf;
                    }
                    pc_var5 = pc_var5 + i_var6;
                    i_var6 = 0x22;
                    pc_var10 = pc_var5;
                    break;
                }
                hglobal_7 = block_1000_4000::pass1_1000_422a((pvVar13 >> 0x10), hglobal_7);
                if (hglobal_7 == 0) {
                    GlobalUnlock16(u_var8);
                    GlobalFree16(hglobal_di);
                    hglobal_7 = 0;
                }
            }
            unaff_cs = 0x38;
            if (hglobal_7 != 0) {
                pu_var11 = null_mut();
                // for (; unaff_si != 0; unaff_si += -1)
                while unaff_si != 0 {
                    // for (iVar6 = -0x8000; i_var6 != 0; i_var6 += -1)
                    for ivar6 in -0x8000..0 {
                        pu_var3 = pu_var11;
                        pu_var11 = pu_var11 + 1;
                        *pu_var3 = 0;
                    }
                    hglobal_7 += 0x100;
                    unaff_si -= 1;
                }
                if (hglobal_di.is_null() == false) {
                    // for (; hglobal_di.is_null() == false; hglobal_di = hglobal_di -1)
                    while hglobal_di.is_null() == false {
                        pu_var4 = pu_var11;
                        pu_var11 = (pu_var11 + 1);
                        *pu_var4 = 0;
                        hglobal_di -= 1;
                    }
                }
                return pi_var9;
            }
        } //
          //        LAB_1000_41aa:
        if ((u16_1050_618e | PTR_LOOP_1050_618c) == 0) {
            return NULL;
        }
        i_var8 = (PTR_LOOP_1050_618c)(unaff_cs, param_3, param_1, param_2);
        if (i_var8 == 0) {
            return NULL;
        }
    }
    loop {
        i_var6 += -0x1;
        pc_var2 = pc_var10;
        pc_var10 = pc_var10 + 1;
        if (*pc_var2 == '\r') {
            break;
        }
        if (i_var6 == 0) {
            break;
        }
    }
    pc_var10[-0x1] = '\0'; //
                           //    LAB_1000_28cb:
    FatalAppExit16(CONCAT13(0x10, CONCAT12(0x50, pc_var5)), 0x0);
    FatalExit();
    ppa_var8 = &PTR_LOOP_1050_63fe;
    loop {
        ppa_var1 = ppa_var8;
        ppa_var8 = ppa_var8 + 1;
        temp_5fa27366cb = *ppa_var1;
        ppa_var7 = ppa_var8;
        if ((temp_5fa27366cb == hglobal_di)
            || (ppa_var7 = (temp_5fa27366cb + 1), ppa_var7.is_null()))
        {
            return ppa_var7;
        }
        i_var6 = -0x1;
        loop {
            if (i_var6 == 0) {
                break;
            }
            i_var6 += -0x1;
            ppa_var1 = ppa_var8;
            ppa_var8 = (ppa_var8 + 1);
            if ppa_var1.is_null() {
                break;
            }
        }
    }
}
