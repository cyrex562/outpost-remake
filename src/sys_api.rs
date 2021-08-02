#![allow(non_snake_case)]

use crate::debug::debug_print_1008_6048;
use crate::defines::{Struct18, Struct19, Struct20, Struct37, Struct57, Struct76, Struct79, Struct87, Struct_1000_2cb0, Struct_1040_98c0, U32Ptr};
use crate::draw::draw_1040::{draw_text_1040_8d14, unk_draw_op_1040_b0f8};
use crate::exit::exit_1000_25f2;
use crate::file::file_1008::close_file_1008_496c;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_24cd, fn_ptr_op_1000_2594};
use crate::global::AppContext;
use crate::mem_1000::{mem_1000_2bb6, mem_op_1000_179c, mem_op_1000_21b6};
use crate::mixed::mixed_1010_20ba;
use crate::msg_box::msg_box_op_1000_1f24;
use crate::pass::pass_1000::{pass1_1000_1f68, pass1_1000_25a8, pass1_1000_2913, pass1_1000_29af, pass1_1000_29b5, pass1_1000_29dc, pass1_1000_39e1, pass1_1000_3bac, pass1_1000_3e2c, pass1_1000_422a, pass1_1000_462e, pass1_1000_47a4, pass1_1000_4906, pass1_1000_5008, pass1_1000_55b1};
use crate::pass::pass_1008::{pass1_1008_3e94, pass1_1008_405c, pass1_1008_4772, pass1_1008_4d84, pass1_1008_5394, pass1_1008_5784, pass1_1008_57c4, pass1_1008_57f0, pass1_1008_5b12, pass1_1008_5bdc, pass1_1008_9628};
use crate::pass::pass_1010::{pass1_1010_1d80, pass1_1010_2024, pass1_1010_6034, pass1_1010_7e40, pass1_1010_8096};
use crate::pass::pass_1020::string_1020_c0d8;
use crate::pass::pass_1040::pass1_1040_b040;
use crate::string::string_1000::{str_op_1000_28dc, str_op_1000_3da4, string_1000_3cea, string_1000_3d3e, string_1000_475e};
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::{load_string_1010_847e, load_string_1010_84e0};
use crate::string::string_1040::string_1040_8520;
use crate::struct_ops::struct_1008::{set_struct_1008_574a, set_struct_op_1008_0536, struct_1008_4c58, struct_op_1008_3f92, struct_op_1008_48fe};
use crate::switch_ops::switch_1010::switchD_1010;
use crate::switch_ops::switch_1018::switch_1018_3b9e;
use crate::ui::ui_1008::win_ui_reg_class_1008_96d2;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42};
use crate::win_struct::{ATOM, HANDLE16, HDC16, HGLOBAL16, HINSTANCE16, HRSRC16, HWND16, LPARAM, MSG16, RECT16, SEGPTR, WNDCLASS16, WPARAM16};
use crate::winapi::{DefWindowProc16, DeleteObject16, DispatchMessage16, DOS3Call, FatalAppExit16, FatalExit, FindResource16, FreeProcInstance16, FreeResource16, GetClassInfo16, GetClientRect16, GetDC16, GetMessage16, GetPrivateProfileString16, GetProp16, GetSystemMetrics16, GetVersion16, GetWindowLong16, GLobalAlloc16, GlobalFree16, GlobalUnlock16, IsDialogMessage16, KillTimer16, LoadResource16, LoadString16, MakeProcInstance16, MessageBox16, RegisterClass16, ReleaseDC16, RemoveProp16, SetErrorMode16, SetProp16, SetWindowLong16, swi, TranslateAccelerator16, TranslateMessage16, UpdateWindow16, WIN16_GlobalLock16, WIN16_LockResource16, WritePrivateProfileString16, wsprintf16};

pub fn _SHI_INVOKEERRORHANDLER1() -> u16

{
    let mut i_var1: bool;
    let BVar2: bool;
    let u_var2: u16;
    let unaff_CS: u16;
    let pcStack6: u32;
    let puStack4: U32Ptr;
    let u_var3: u16;

    u_var3 = ctx.data_seg;
    puStack4 = ctx.data_seg;
    if (true) {
        if ((ctx.PTR_LOOP_1050_5f1c | ctx.PTR_PTR_1050_5f1a) == 0x0) {
            pcStack6 = 0x0;
            puStack4 = 0x0;
        } else {
            i_var1 = mem_op_1000_21b6(ctx.PTR_PTR_1050_5f1a, PTR_LOOP_1050_5f1c);
            pcStack6 = ctx.PTR_PTR_1050_5f1a;
            puStack4 = ctx.PTR_LOOP_1050_5f1c;
            if i_var1 == false {
                ctx.PTR_PTR_1050_5f1a = &ctx.PTR_PTR_1050_1f7e;
                ctx.PTR_LOOP_1050_5f1c = &ctx.PTR_LOOP_1050_1000;
                pcStack6 = &ctx.PTR_PTR_1050_1f7e;
                puStack4 = &ctx.PTR_LOOP_1050_1000;
            }
        }
        if ((puStack4 | pcStack6) != 0x0) {
            BVar2 = msg_box_op_1000_1f24(&ctx.PTR_PTR_1050_5f1a, ctx.data_seg, 0x0, unaff_CS);
            if BVar2 == false {
                u_var2 = (*pcStack6)();
            } else {
                puStack4 = 0x0;
                pcStack6 = 0x0;
                u_var2 = 0x0;
            }
            if ((puStack4 | pcStack6) != 0x0) {
                pass1_1000_1f68(u_var3);
            }
            return u_var2;
        }
    }
    return 0x0;
}


pub fn dos3_call_1000_23ea(param_1: u16, param_2: u16, param_3: i16, param_4: &mut String) -> U32Ptr

{
    let pbVar1: U32Ptr;
    let pbVar2: U32Ptr;
    let bVar3: u8;
    let piVar4: U32Ptr;
    let pbVar5: U32Ptr;
    let mut pcVar6: String;
    let uVar7: u16;
    let ppcVar8: u32;
    let pcVar9: u32;
    let u_var10: u16;
    let bVar11: u8;
    let bVar12: u8;
    let uVar13: u16;
    let mut str: String;
    let piVar14: U32Ptr;
    let uVar15: u16;
    let extraout_dx: u16;
    let uVar16: u16;
    let pbVar17: U32Ptr;
    let piVar18: U32Ptr;
    let pbVar19: U32Ptr;
    let mut pcVar20: String;
    let action: u16;
    let bVar21: bool;
    let uVar22: u32;
    let iVar23: i16;
    let iVar24: i16;
    let iVar25: i16;

    iVar25 = ctx.data_seg;
    if (true) {
        pcVar9 = swi(0x21);
        (*pcVar9)(param_3 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    uVar15 = 0x2890;
    action = 0x1000;
    ctx.PTR_LOOP_1050_5f6a = param_1;
    ctx.PTR_LOOP_1050_5f6c = param_2;
    if (true) {
        pcVar9 = swi(0x21);
        (*pcVar9)();
        uVar15 = extraout_dx;
    } else {
        action = ctx.s_tile2_bmp_1050_1538;
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    uVar13 = pass1_1000_29dc(param_4);
    uVar22 = CONCAT22(uVar15, uVar13);
    if (&ctx.PTR_LOOP_1050_6202 != 0x0) {
        uVar7 = &ctx.PTR_LOOP_1050_5f7e;
        bVar21 = false;
        ppcVar8 = &ctx.PTR_LOOP_1050_6200;
        (**ppcVar8)(action);
        if (bVar21) {
            iVar24 = 0x2;
            iVar23 = 0x2;
            pass1_1000_25a8(ctx, uVar7, action);
            pass1_1000_2913(ctx, iVar23, uVar7, action);
            str = str_op_1000_28dc(iVar24);
            if (str != 0x0) {
                iVar23 = 0x9;
                if (*str == 'M') {
                    iVar23 = 0xf;
                }
                str = str + iVar23;
                iVar23 = 0x22;
                pcVar20 = str;
                loop {
                    if (iVar23 == 0x0) { break; }
                    iVar23 += -0x1;
                    pcVar6 = pcVar20;
                    pcVar20 = pcVar20 + 0x1;
                    if *pcVar6 == '\r' { break; }
                }
                pcVar20[-0x1] = '\0';
            }
            FatalAppExit16(action, str);
            FatalExit();
            piVar18 = &ctx.PTR_LOOP_1050_63fe;
            loop {
                piVar4 = piVar18;
                piVar18 = piVar18 + 0x1;
                iVar23 = *piVar4;
                piVar14 = piVar18;
                if ((iVar23 == iVar25) || (piVar14 = (iVar23 + 0x1), piVar14 == 0x0)
                ) {
                    return piVar14;
                }
                iVar23 = -0x1;
                loop {
                    if (iVar23 == 0x0) { break; }
                    iVar23 += -0x1;
                    piVar4 = piVar18;
                    piVar18 = (piVar18 + 0x1);
                    if *piVar4 == '\0' {
                        break;
                    }
                }
            }
        }
        ppcVar8 = &ctx.PTR_LOOP_1050_6200;
        uVar22 = (**ppcVar8)(action);
    }
    iVar25 = (s_New_failed_in_Op__Op_1050_0020 + 0xc);
    piVar18 = uVar22;
    if iVar25 != 0x0 {
        pbVar19 = 0x0;
        piVar14 = uVar22;
        loop {
            bVar21 = *pbVar19 == 0x0;
            piVar18 = piVar14;
            if (bVar21) { break; }
            iVar23 = 0xd;
            pbVar17 = s__C_FILE_INFO__1050_5f5c;
            loop {
                if (iVar23 == 0x0) { break; }
                iVar23 += -0x1;
                pbVar5 = pbVar19;
                pbVar19 = pbVar19 + 0x1;
                pbVar1 = pbVar17;
                pbVar17 = pbVar17 + 0x1;
                bVar21 = *pbVar1 == *pbVar5;
                if bVar21 == false {
                    break;
                }
            }
            if (bVar21) {
                pbVar17 = 0x5f90;
                // uVar16 = (uVar22 >> 0x10);
//         TODO: goto LAB_1000_2495;
            }
            iVar23 = 0x7fff;
            piVar18 = 0x0;
            bVar21 = true;
            loop {
                if (iVar23 == 0x0) { break; }
                iVar23 += -0x1;
                pbVar1 = pbVar19;
                pbVar19 = pbVar19 + 0x1;
                bVar21 = *pbVar1 == 0x0;
                if bVar21 {
                    break;
                }
            }
            piVar14 = piVar18;
            if bVar21 == false {
                break;
            }
        }
    }
//LAB_1000_24a9:
    fn_ptr_op_1000_2594(0x620c, 0x620c);
    fn_ptr_op_1000_2594(0x620c, 0x620c);
    fn_ptr_op_1000_2594(0x61fe, 0x61ee);
    return piVar18;
//LAB_1000_2495:
    pbVar2 = pbVar19 + 0x1;
    bVar3 = *pbVar19;
    u_var10 = piVar14 & 0xff00;
    bVar11 = bVar3 + 0xbf;
    piVar18 = (u_var10 | bVar11);
    if (bVar3 < 0x41) {
        // goto
        // LAB_1000_24a9;
    }
    pbVar19 = pbVar19 + 0x2;
    bVar3 = *pbVar2;
    piVar14 = (uVar16 & 0xff00);
    bVar12 = bVar3 + 0xbf;
    piVar18 = (piVar14 | bVar12);
    if (bVar3 < 0x41)
    goto
    LAB_1000_24a9;
    pbVar1 = pbVar17;
    pbVar17 = pbVar17 + 0x1;
    *pbVar1 = bVar12 | bVar11 * '\x10';
    uVar16 = u_var10;
//   TODO: goto LAB_1000_2495;
}


pub fn dos3_op_1000_256b() {
    let pcVar1: u32;

    if (ctx.PTR_LOOP_1050_6202 != 0x0) {
        (*PTR_LOOP_1050_6200)();
    }
    if (true) {
        pcVar1 = swi(0x21);
        (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    return;
}


pub fn sys_1000_30b4(
    ctx: &mut AppContext,
    param_1: &mut Struct_1000_2cb0,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: &mut Struct_1000_2cb0,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) -> u16

{
    let bVar1: u8;
    let bVar2: u8;
    let u_var3: u16;
    let i_var3: i16;
    let u_var4: u16;

    i_var3 = param_4 + 0x1;
    u_var4 = SUB42(ctx.data_seg, 0x0);
    exit_1000_25f2(ctx, 0x30c5, param_7, ctx.data_seg, 0x214, param_6, param_7, param_8);
    bVar1 = *param_3;
    if (bVar1 != 0x0) & &(true) {
        if (bVar1 - 0x20) < 0x59 {
            bVar2 = ((bVar1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar2 = 0x0;
        }
// WARNING: Could not emulate address calculation at 0x10003101
// WARNING: Treating indirect jump as call
        u_var3 = ((((bVar2 * 0x8) + 0x5ffe) > > 0x4) * 0x2 + 0x30a4)(param_5 & 0xff00 | bVar1, u_var4, i_var3);
        return u_var3;
    }
    return 0x0;
}


pub fn dos3_call_op_1000_35fe(param_1: u16, param_2: i16) -> u16

{
    let pcVar1: u32;
    let u_var2: u16;
    let u_var3: u8;

    if (param_1 < ctx.DAT_1050_5f8a) {
        u_var2 = 0x3e50;
        u_var3 = 0x0;
        if (true) {
            pcVar1 = swi(0x21);
            u_var2 = (*pcVar1)(param_2 + 0x1);
        } else {
            DOS3Call(&ctx.PTR_LOOP_1050_1000);
        }
        if (!u_var3) {
            *(param_1 + 0x5f90) = 0x0;
        }
    } else {
        u_var2 = 0x900;
        u_var3 = true;
    }
    if (!u_var3) {
        return 0x0;
    }
    pass1_1000_29b5(ctx, u_var2);
    return 0xffff;
}


pub fn mixed_dos3_call_1000_3636(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16)

{
    let pbVar1: U32Ptr;
    let pcVar2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u8;
    let uVar9: u32;

    if (((param_1 < ctx.DAT_1050_5f8a) || (ctx.PTR_LOOP_1050_61ec == 0x0)) || (0x2 < param_1)) {
        if ((ctx.PTR_LOOP_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0x0))
//     TODO: goto LAB_1000_36e3;
        if (param_4 == 0x0) {
            // goto
            // LAB_1000_369b;
        }
        u_var5 = 0x0;
        u_var6 = 0x0;
        u_var4 = 0x4201;
        uVar8 = 0x0;
        if (true) {
            pcVar2 = swi(0x21);
            uVar9 = (*pcVar2)();
        } else {
            DOS3Call(&ctx.PTR_LOOP_1050_1000);
            uVar9 = CONCAT22(u_var6, u_var4);
        }
        // iVar7 = (uVar9 >> 0x10);
        u_var3 = uVar9;
        if (uVar8) {
            // goto
            // LAB_1000_299d;
        }
        if ((param_4 & 0x2) == 0x0) {
            if (-0x1 < (iVar7 + param_3 + CARRY2(u_var3, param_2))) {
//LAB_1000_36e3:
                u_var3 = CONCAT11(0x42, param_4);
                uVar8 = 0x0;
                if (true) {
                    pcVar2 = swi(0x21);
                    u_var3 = (*pcVar2)();
                } else {
                    DOS3Call(&ctx.PTR_LOOP_1050_1000);
                }
                if (!uVar8) {
                    pbVar1 = (param_1 + 0x5f90);
                    uVar8 = false;
                    *pbVar1 = *pbVar1 & 0xfd;
                }
//         TODO: goto LAB_1000_299d;
            }
        } else {
            u_var4 = SUB42(&ctx.PTR_DAT_1050_0041_1050_4202, 0x0);
            if (true) {
                pcVar2 = swi(0x21);
                uVar9 = (*pcVar2)(iVar7);
            } else {
                DOS3Call(&ctx.PTR_LOOP_1050_1000);
                uVar9 = CONCAT22(u_var5, u_var4);
            }
            if (-0x1 < ((uVar9 >> 0x10) + param_3 + CARRY2(uVar9, param_2))) {
                // goto
                // LAB_1000_36e3;
            }
            if (true) {
                pcVar2 = swi(0x21);
                (*pcVar2)();
            } else {
                DOS3Call(&ctx.PTR_LOOP_1050_1000);
            }
        }
//LAB_1000_369b:
        u_var3 = s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    uVar8 = true;
//LAB_1000_299d:
    if (uVar8) {
        pass1_1000_29b5(ctx, u_var3);
    }
    return;
}


pub fn mixed_dos3_call_1000_370a(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u8,
    param_5: u16,
    param_6: i16,
) -> u16

{
    let pcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let extraout_dx: u16;
    let uVar7: u8;
    let uVar8: u32;
    let uVar9: u16;
    let bVar10: u8;
    let bVar11: u8;
    let in_stack_0000fffb: u8;

    _param_4 = param_5;
    bVar10 = 0x0;
    if ((param_3 & 0x8000) == 0x0) && (((param_3 & 0x4000) != 0x0 || ((ctx.DAT_1050_6061 & 0x80) == 0x0))) {
        bVar10 = 0x80;
    }
    uVar9 = SUB42(ctx.data_seg, 0x0) as u16;
    u_var3 = CONCAT11(0x3d, (param_3 & 0x3 | param_4) as u8);
    uVar7 = 0x0;
    u_var6 = param_3;
    if true {
        pcVar1 = swi(0x21);
        uVar8 = (*pcVar1)(bVar10, param_4, ctx.data_seg, param_6 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
        uVar8 = CONCAT22(param_1, u_var3);
    }
    u_var2 = uVar8;
    if (uVar7) {
        if ((u_var2 == 0x2) && ((u_var6 & 0x100) != 0x0)) {
            bVar11 = 0x0;
            uVar7 = pass1_1000_39e1();
            _param_4 = param_5;
            if ((param_4 != 0x0) || (u_var6 = param_5, (param_3 & 0x2) == 0x0)) {
                u_var6 = 0x0;
            }
//LAB_1000_38e3:
            u_var5 = CONCAT11(0x3c, uVar7);
            uVar7 = 0x0;
            if (true) {
                pcVar1 = swi(0x21);
                u_var5 = (*pcVar1)();
            } else {
                DOS3Call(&ctx.PTR_LOOP_1050_1000);
            }
            u_var2 = u_var5;
            if (uVar7) {
                //goto LAB_1000_299d;
            }
            if ((param_4 != 0x0) || ((param_3 & 0x2) == 0x0)) {
                if (true) {
                    pcVar1 = swi(0x21);
                    (*pcVar1)();
                } else {
                    DOS3Call(&ctx.PTR_LOOP_1050_1000);
                }
                u_var5 = CONCAT11(0x3d, param_3 & 0x3 | param_4);
                uVar7 = 0x0;
                if (true) {
                    pcVar1 = swi(0x21);
                    u_var5 = (*pcVar1)();
                } else {
                    DOS3Call(&ctx.PTR_LOOP_1050_1000);
                }
                u_var2 = u_var5;
                if (uVar7) {
// goto LAB_1000_299d;
                }
                if (((bVar11 & 0x1) == 0x0) && ((_param_4 & 0x1) != 0x0)) {
                    u_var6 = (u_var6 | 0x1);
                    u_var2 = 0x4301;
                    uVar7 = 0x0;
                    if (true) {
                        pcVar1 = swi(0x21);
                        u_var2 = (*pcVar1)();
                    } else {
                        DOS3Call(&ctx.PTR_LOOP_1050_1000);
                    }
                    if (uVar7) {
                        goto
                        LAB_1000_299d;
                    }
                }
            }
//LAB_1000_3973:
            if ((bVar10 & 0x40) == 0x0) {
                if (true) {
                    pcVar1 = swi(0x21);
                    (*pcVar1)();
                } else {
                    DOS3Call(&ctx.PTR_LOOP_1050_1000);
                }
                bVar11 = 0x0;
                if ((u_var6 & 0x1) != 0x0) {
                    bVar11 = 0x10;
                }
                if ((param_3 & 0x8) != 0x0) {
                    bVar11 |= 0x20;
                }
            } else {
                bVar11 = 0x0;
            }
            if (u_var5 < &DAT_1050_5f8a) {
                (u_var5 + 0x5f90) = bVar11 | bVar10 | 0x1;
                return u_var5;
            }
            if (true) {
                pcVar1 = swi(0x21);
                (*pcVar1)();
            } else {
                DOS3Call(&ctx.PTR_LOOP_1050_1000);
            }
            u_var2 = 0x1800;
        }
    } else {
        if ((u_var6 & 0x500) != 0x500) {
            bVar11 = 0x1;
            if (true) {
                pcVar1 = swi(0x21);
                (*pcVar1)();
                uVar8 = CONCAT22(extraout_dx, u_var2);
            } else {
                DOS3Call(&ctx.PTR_LOOP_1050_1000);
            }
            u_var5 = uVar8;
            if ((uVar8 & 0x800000) != 0x0) {
                bVar10 |= 0x40;
            }
            if ((bVar10 & 0x40) == 0x0) {
                if ((param_3 & 0x200) == 0x0) {
                    if (((bVar10 & 0x80) != 0x0) && ((param_3 & 0x2) != 0x0)) {
                        uVar7 = 0x2;
                        if (true) {
                            pcVar1 = swi(0x21);
                            uVar7 = (*pcVar1)();
                        } else {
                            DOS3Call(&ctx.PTR_LOOP_1050_1000);
                        }
                        i_var4 = CONCAT11(0x3f, uVar7);
                        if (true) {
                            pcVar1 = swi(0x21);
                            i_var4 = (*pcVar1)();
                        } else {
                            DOS3Call(&ctx.PTR_LOOP_1050_1000);
                        }
                        if ((i_var4 != 0x0) && (in_stack_0000fffb == '\x1a')) {
                            if (true) {
                                pcVar1 = swi(0x21);
                                (*pcVar1)();
                            } else {
                                DOS3Call(&ctx.PTR_LOOP_1050_1000);
                            }
                            if (true) {
                                pcVar1 = swi(0x21);
                                (*pcVar1)();
                            } else {
                                DOS3Call(&ctx.PTR_LOOP_1050_1000);
                            }
                        }
                        u_var6 = 0x0;
                        if (true) {
                            pcVar1 = swi(0x21);
                            (*pcVar1)();
                        } else {
                            DOS3Call(&ctx.PTR_LOOP_1050_1000);
                        }
                    }
                } else {
                    if ((param_3 & 0x3) == 0x0) {
                        if (true) {
                            pcVar1 = swi(0x21);
                            (*pcVar1)();
                        } else {
                            DOS3Call(&ctx.PTR_LOOP_1050_1000);
                        }
                        uVar7 = 0x0;
                        if (true) {
                            pcVar1 = swi(0x21);
                            uVar7 = (*pcVar1)();
                        } else {
                            DOS3Call(&ctx.PTR_LOOP_1050_1000);
                        }
//             TODO: goto LAB_1000_38e3;
                    }
                    u_var6 = 0x0;
                    if (true) {
                        pcVar1 = swi(0x21);
                        (*pcVar1)();
                    } else {
                        DOS3Call(&ctx.PTR_LOOP_1050_1000);
                    }
                }
            }
//       TODO: goto LAB_1000_3973;
        }
        if (true) {
            pcVar1 = swi(0x21);
            (*pcVar1)();
        } else {
            DOS3Call(&ctx.PTR_LOOP_1050_1000);
        }
        u_var2 = 0x1100;
    }
    uVar7 = true;
//LAB_1000_299d:
    if (uVar7) {
        pass1_1000_29b5(u_var2);
        u_var2 = 0xffff;
    }
    return u_var2;
}


// WARNING: Unable to track spacebase fully for stack

pub fn mixed_dos3_call_1000_39f2(
    ctx: &mut AppContext,
    param_1: U32Ptr,
    param_2: &mut String,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u8) -> U32Ptr

{
    let pbVar1: U32Ptr;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let pcVar4: u32;
    let u_var5: u16;
    let u_var6: u8;
    let piVar7: U32Ptr;
    let uVar8: u16;
    let piVar9: U32Ptr;
    let piVar10: U32Ptr;
    let u_var11: u16;
    let puVar12: U32Ptr;
    let iVar13: i16;
    let puVar14: U32Ptr;
    let pbVar15: U32Ptr;
    let piVar16: U32Ptr;
    let puVar17: U32Ptr;
    let unaff_BP: i16;
    let pbVar18: U32Ptr;
    let puVar19: U32Ptr;
    let uVar20: u16;
    let uVar21: u8;
    let bVar22: u8;
    let cVar23: u8;
    let bVar24: bool;
    let cVar25: u8;
    let cVar26: u8;
    let uVar27: u32;
    let mut pcVar28: String;
    let piStack14: U32Ptr;
    let puStack12: U32Ptr;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let uStack4: u16;
    let iStack2: i16;

    iStack2 = unaff_BP + 0x1;
    uStack4 = SUB42(ctx.data_seg, 0x0);
    piStack14 = ctx.DAT_1050_5f8a;
    piVar7 = ctx.DAT_1050_5f8a;
    if ((ctx.PTR_LOOP_1050_61ec != 0x0) && (piVar7 = ctx.PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e,
                                            param_1 < (&ctx.PTR_LOOP_1050_0002 + 0x1))) {
        param_1 = ctx.DAT_1050_5f8a;
    }
    if (piVar7 <= param_1) {
        uVar21 = true;
        piVar7 = 0x900;
//     TODO: goto LAB_1000_299d;
    }
    piVar16 = param_1;
    if ((param_1[0x5f90] & 0x20) != 0x0) {
        piVar7 = &ctx.PTR_DAT_1050_0041_1050_4202;
        param_5 = 0x1000;
        uVar21 = 0x0;
        if (true) {
            pcVar4 = swi(0x21);
            piVar7 = (*pcVar4)();
        } else {
            param_5 = ctx.s_tile2_bmp_1050_1538;
            DOS3Call(&ctx.PTR_LOOP_1050_1000);
        }
        if (uVar21)
        goto
        LAB_1000_299d;
    }
    u_var6 = SUB21(piVar7, 0x0);
    pbVar15 = param_2;
    puVar17 = puStack12;
    if (((piVar16 + 0x2fc8) & 0x80) == 0x0) {
//LAB_1000_3acf:
        puStack12 = puVar17;
        uVar21 = false;
        piVar7 = param_3;
        if (param_3 != 0x0) {
            uVar21 = piVar16 < piStack14;
            if (uVar21) {
                u_var11 = CONCAT11(0x40, u_var6);
                uVar21 = 0x0;
                if (true) {
                    pcVar4 = swi(0x21);
                    uVar27 = (*pcVar4)();
                } else {
                    DOS3Call(&ctx.PTR_LOOP_1050_1000);
                    uVar27 = CONCAT22(pbVar15, u_var11);
                }
            } else {
                piVar7 = pass1_1000_55b1(ctx, 0x3b71, param_4, param_5);
                uVar27 = CONCAT22(pbVar15, piVar7);
            }
            piVar7 = uVar27;
            if (uVar21) {
                piVar7 = CONCAT11(0x9, uVar27);
            } else {
                uVar21 = false;
                if (piVar7 == 0x0) {
                    if ((((piVar16 + 0x2fc8) & 0x40) == 0x0) || (*(uVar27 >> 0x10) != '\x1a')) {
                        uVar21 = true;
                        piVar7 = 0x1c00;
                    } else {
                        uVar21 = false;
                    }
                }
            }
        }
    } else {
        uStack10 = SUB42(ctx.data_seg, 0x0);
        bVar24 = true;
        uStack6 = 0x0;
        uStack8 = 0x0;
        puStack12 = &stack0xffee;
        puVar17 = &stack0xffee;
        if (param_3 != 0x0) {
            u_var6 = 0xa;
            puVar12 = param_3;
            pbVar18 = pbVar15;
            loop {
                if (puVar12 == 0x0) { break; }
                puVar12 = puVar12 + -0x1;
                pbVar1 = pbVar18;
                pbVar18 = pbVar18 + 0x1;
                bVar24 = *pbVar1 == 0xa;
                if bVar24 {
                    break;
                }
            }
            param_4 = param_2._2_2_;
            puVar17 = &stack0xffee;
            if (!bVar24) {
                // goto
                // LAB_1000_3acf;
            }
            pcVar28 = param_2;
            uVar8 = pass1_1000_3bac();
            pcVar28._2_2_ = (pcVar28 >> 0x10);
            iVar13 = pcVar28;
            if (uVar8 < 0xa9) {
                piVar9 = exit_1000_25f2(
                    ctx, 0x3ad9, param_5, pcVar28._2_2_, -0x4, param_2._2_2_, param_5,
                    param_6);
                piVar7 = (pbVar18 + -iVar13);
                if (piVar7 == 0x0) {
                    return piVar9;
                }
                bVar22 = param_1 < piStack14;
                pu_var3 = param_1 + -piStack14;
                cVar26 = pu_var3 < 0x0;
                cVar25 = pu_var3 == 0x0;
                cVar23 = (POPCOUNT(pu_var3 & 0xff) & 0x1) == 0x0;
                if (bVar22) {
                    piVar10 = CONCAT11(0x40, piVar9);
                    bVar22 = 0x0;
                    cVar26 = '\0';
                    cVar25 = '\x01';
                    cVar23 = '\x01';
                    if (true) {
                        pcVar4 = swi(0x21);
                        piVar10 = (*pcVar4)(ctx.data_seg, puVar12, piVar16);
                    } else {
                        DOS3Call(&ctx.PTR_LOOP_1050_1000);
                    }
                } else {
                    piVar10 = pass1_1000_55b1(ctx, 0x3af1, param_2._2_2_, param_5);
                }
                if (!bVar22) {
                    uStack6 += piVar10;
                    bVar22 = piVar7 < piVar10;
                    piVar7 = (piVar7 - piVar10);
                    cVar26 = piVar7 < 0x0;
                    cVar25 = piVar7 == 0x0;
                    cVar23 = (POPCOUNT(piVar7 & 0xff) & 0x1) == 0x0;
                    if (bVar22 || cVar25) {
                        return piVar9;
                    }
                }
                uVar8 = (cVar26 << 0x7 | cVar25 << 0x6 | param_7 << 0x4 | cVar23 << 0x2 | 0x2 | bVar22) << 0x8;
                piVar7 = (piVar10 & 0xff | uVar8);
                puVar17 = puStack12;
                if (uStack6 == 0x0) {
                    uVar21 = (uVar8 & 0x100) != 0x0;
                    if (uVar21) {
                        piVar7 = CONCAT11(0x9, (piVar10 & 0xff));
                    } else {
                        if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
                            uVar21 = true;
                            piVar7 = 0x1c00;
                        } else {
                            uVar21 = false;
                        }
                    }
//           TODO: goto LAB_1000_299d;
                }
            } else {
                puVar17 = &stack0xffec;
                iVar13 = 0x200;
                if (uVar8 < 0x228) {
                    iVar13 = 0x80;
                }
                iVar13 = -iVar13;
                puVar14 = &stack0xffec + iVar13;
                puVar19 = &stack0xffec + iVar13;
                (&stack0xffea + iVar13) = param_6;
                uVar20 = (&stack0xffea + iVar13);
                loop {
                    pbVar1 = pbVar15;
                    pbVar15 = pbVar15 + 0x1;
                    bVar22 = *pbVar1;
                    u_var5 = uVar8 & 0xff00;
                    uVar8 = u_var5 | bVar22;
                    if (bVar22 == 0xa) {
                        uVar8 = CONCAT11((u_var5 >> 0x8), 0xd);
                        if (puVar19 == puVar17) {
                            (&stack0xffea + iVar13) = 0x3abd;
                            uVar8 = mixed_dos3_call_1000_3ad9(uVar8, puVar14, &iStack2, puVar19, uVar20, param_5, param_6,
                                                              param_7);
                        }
                        pu_var2 = puVar19;
                        puVar19 = puVar19 + 0x1;
                        *pu_var2 = uVar8;
                        uVar8 = CONCAT11((uVar8 >> 0x8), 0xa);
                        uStack8 += 0x1;
                    }
                    if (puVar19 == puVar17) {
                        (&stack0xffea + iVar13) = 0x3ac8;
                        uVar8 = mixed_dos3_call_1000_3ad9(uVar8, puVar14, &iStack2, puVar19, uVar20, param_5, param_6,
                                                          param_7);
                    }
                    pu_var2 = puVar19;
                    puVar19 = puVar19 + 0x1;
                    *pu_var2 = uVar8;
                    param_3 = param_3 + -0x1;
                    if param_3 == 0 {
                        break;
                    }
                }
                (&stack0xffea + iVar13) = 0x3ab1;
                mixed_dos3_call_1000_3ad9(uVar8, puVar14, &iStack2, puVar19, uVar20, param_5, param_6, param_7);
                puVar17 = puStack12;
            }
        }
        puStack12 = puVar17;
        uVar21 = uStack6 < uStack8;
        piVar7 = (uStack6 - uStack8);
    }
//LAB_1000_299d:
    if (uVar21) {
        pass1_1000_29b5(ctx, piVar7);
        piVar7 = 0xffff;
    }
    return piVar7;
}


// WARNING: Unable to track spacebase fully for stack

pub fn mixed_dos3_call_1000_3ad9(param_1: u16, param_2: i16, param_3: i16, param_4: i16, param_5: u16, param_6: u16,
                                 param_7: u16, param_8: u8) -> u16

{
    let pu_var1: *mut u16;
    let piVar2: *mut i16;
    let pcVar3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let piVar6: *mut i16;
    let piVar7: U32Ptr;
    let uVar8: u16;
    let bVar9: u8;
    let bVar10: bool;
    let cVar11: u8;
    let cVar12: u8;
    let cVar13: u8;

    piVar7 = (param_4 - param_2);
    if (piVar7 == 0x0) {
        return param_1;
    }
    uVar8 = (param_3 + 0x6);
    pu_var1 = (param_3 + -0xc);
    bVar9 = uVar8 < *pu_var1;
    u_var5 = uVar8 - *pu_var1;
    cVar13 = u_var5 < 0x0;
    cVar12 = u_var5 == 0x0;
    cVar11 = (POPCOUNT(u_var5 & 0xff) & 0x1) == 0x0;
    if (bVar9) {
        piVar6 = CONCAT11(0x40, param_1);
        bVar9 = 0x0;
        cVar13 = '\0';
        cVar12 = '\x01';
        cVar11 = '\x01';
        if (true) {
            pcVar3 = swi(0x21);
            piVar6 = (*pcVar3)(ctx.data_seg);
        } else {
            DOS3Call(&ctx.PTR_LOOP_1050_1000);
        }
    } else {
        piVar6 = pass1_1000_55b1(0x3af1, param_5, param_6);
    }
    if (!bVar9) {
        piVar2 = (param_3 + -0x4);
        *piVar2 = *piVar2 + piVar6;
        bVar9 = piVar7 < piVar6;
        piVar7 = (piVar7 - piVar6);
        cVar13 = piVar7 < 0x0;
        cVar12 = piVar7 == 0x0;
        cVar11 = (POPCOUNT(piVar7 & 0xff) & 0x1) == 0x0;
        if (bVar9 | | cVar12) {
            return param_1;
        }
    }
    u_var4 = (cVar13 < < 0x7 | cVar12 << 0x6 | param_8 < < 0x4 | cVar11 << 0x2 | 0x2 | bVar9) < < 0x8;
    u_var5 = piVar6 & 0xff | u_var4;
    if ((param_3 + -0x4) == 0x0) {
        bVar10 = (u_var4 & 0x100) != 0x0;
        if (bVar10) {
            u_var5 = CONCAT11(0x9, (piVar6 & 0xff));
        } else {
            if ((((uVar8 + 0x5f90) & 0x40) == 0x0) | | (*(param_3 + 0x8) != '\x1a')) {
                bVar10 = true;
                u_var5 = 0x1c00;
            } else {
                bVar10 = false;
            }
        }
    } else {
        u_var5 = (param_3 + -0x4);
        pu_var1 = (param_3 + -0x6);
        bVar10 = u_var5 < *pu_var1;
        u_var5 -= *pu_var1;
    }
    if (bVar10) {
        ((param_3 + -0xa) + 0x2) = 0x29a2;
        pass1_1000_29b5(u_var5);
        u_var5 = 0xffff;
    }
    return u_var5;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn sys_1000_3f9c(
    ctx: &mut AppContext,
    param_1: U32Ptr,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: &mut String,
    param_6: i16,
    param_7: u16,
    param_8: u16,
    param_9: U32Ptr,
    param_10: u8,
) -> u16

{
    let pu_var1: *mut u8;
    let u_var2: u16;
    let local_4: u16;
    let iStack2: i16;

    iStack2 = param_6 + 0x1;
    ctx.PTR_LOOP_1050_68b2._0_1_ = 0x42;
    ctx.PTR_LOOP_1050_68ae = param_1;
    ctx.PTR_LOOP_1050_68b0 = param_2;
    ctx._USHORT_1050_68a8 = CONCAT22(param_2, param_1);
    ctx.PTR_LOOP_1050_68ac = 0x7fff;
    u_var2 = sys_1000_30b4(
        ctx,
        &ctx.USHORT_1050_68a8,
        ctx.data_seg,
        CONCAT22(param_4, param_3),
        &iStack2,
        &ctx.USHORT_1050_68a8,
        param_7,
        param_8,
        param_9
    );
    pu_var1 = _USHORT_1050_68a8;
    ctx.PTR_LOOP_1050_68ac = ctx.PTR_LOOP_1050_68ac + -0x1;
    if (ctx.PTR_LOOP_1050_68ac < 0x0) {
        mem_1000_2bb6(0x0, &ctx.USHORT_1050_68a8, &iStack2, param_7, param_8, param_9,
                      param_10, param_2);
    } else {
        ctx._USHORT_1050_68a8 =

            (ctx._USHORT_1050_68a8 & 0xffff0000 | (ctx.USHORT_1050_68a8 + 0x1));
        *pu_var1 = 0x0;
    }
    return u_var2;
}


pub fn mixed_sys_op_1000_40af(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16) -> U32Ptr

{
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let mut pcVar3: String;
    let puVar4: U32Ptr;
    let mut str: String;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let uVar7: u16;
    HVar8: HGLOBAL16;
    SVar9: SEGPTR;
    let iVar10: i16;
    let u_var11: u16;
    let puVar12: U32Ptr;
    let mut pcVar13: String;
    let puVar14: U32Ptr;
    let unaff_SS: u16;
    let bVar15: bool;
    let iVar16: i16;
    let uVar17: u16;

    loop {
        u_var6 = (param_1 * param_3);
        uVar7 = param_2 * param_3 + (param_1 * param_3 >> 0x10);
        if ((uVar7 | u_var6) != 0x0) {
            puVar12 = 0x0;
            if ((uVar7 < 0x3) && ((uVar7 < 0x2 || (u_var6 == 0x0)))) {
                if (uVar7 == 0x0) {
                    u_var6 = u_var6 + 0xfff & 0xf000;
                    if (u_var6 == 0x0) {
                        uVar7 = 0x1;
                    }
                } else {
                    if ((param_3 - 0x1 & param_3) != 0x0) {
                        puVar12 = ((uVar7 << 0x10) % param_3);
                        bVar15 = CARRY2(u_var6, puVar12);
                        u_var6 += puVar12;
                        if (bVar15) {
                            // goto
                            // LAB_1000_41aa;
                        }
                        uVar7 = 0x1;
                    }
                }
            } else {
                if ((param_3 - 0x1 & param_3) != 0x0) {
                    // goto
                    // LAB_1000_41aa;
                }
            }
            uVar17 = 0x0;
            u_var11 = uVar7;
            HVar8 = GLobalAlloc16(0x1000, CONCAT22(uVar7, u_var6));
            if ((HVar8 != 0x0) && ((uVar17 & 0x1) != 0x0)) {
                SVar9 = WIN16_GlobalLock16(ctx.s_tile2_bmp_1050_1538);
                if ((SVar9 != 0x0) || (uVar7 == 0x0)) {
                    iVar16 = 0x12;
                    iVar10 = 0x12;
                    pass1_1000_25a8(ctx, param_5, s_tile2_bmp_1050_1538);
                    pass1_1000_2913(ctx, iVar10, param_5, s_tile2_bmp_1050_1538);
                    str = str_op_1000_28dc(iVar16);
                    if (str == 0x0) {
                        goto
                        LAB_1000_28cb;
                    }
                    iVar10 = 0x9;
                    if (*str == 'M') {
                        iVar10 = 0xf;
                    }
                    str = str + iVar10;
                    iVar10 = 0x22;
                    pcVar13 = str;
                    break;
                }
                HVar8 = pass1_1000_422a(ctx, uVar7, HVar8, s_tile2_bmp_1050_1538, unaff_SS);
                if (HVar8 == 0x0) {
                    GlobalUnlock16(ctx.s_tile2_bmp_1050_1538);
                    GlobalFree16(ctx.s_tile2_bmp_1050_1538);
                    HVar8 = 0x0;
                }
            }
            param_4 = ctx.s_tile2_bmp_1050_1538;
            if (HVar8 != 0x0) {
                puVar14 = 0x0;
                // TODO: refactor
                // for (; u_var11 != 0x0; u_var11 -= 0x1) {
                //   for (iVar10 = -0x8000; iVar10 != 0x0; iVar10 += -0x1) {
                //     puVar4 = puVar14;
                //     puVar14 = puVar14 + 0x1;
                //     *puVar4 = 0x0;
                //   }
                //   HVar8 += 0x100;
                // }
                if (u_var6 != 0x0) {
                    // TODO: refactor
                    // for (; u_var6 != 0x0; u_var6 -= 0x1) {
                    //   puVar4 = puVar14;
                    //   puVar14 = (puVar14 + 0x1);
                    //   *puVar4 = 0x0;
                    // }
                }
                return puVar12;
            }
        }
//LAB_1000_41aa:
        if ((ctx.PTR_LOOP_1050_618e | ctx.PTR_LOOP_1050_618c) == 0x0) {
            return 0x0;
        }
        iVar10 = (*PTR_LOOP_1050_618c)(param_4, param_3, param_1, param_2);
        if (iVar10 == 0x0) {
            return 0x0;
        }
    }
    loop {
        iVar10 += -0x1;
        pcVar3 = pcVar13;
        pcVar13 = pcVar13 + 0x1;
        if (*pcVar3 == '\r') { break; }
        if (iVar10 == 0x0) { break; }
    }
    pcVar13[-0x1] = '\0';
//LAB_1000_28cb:
    FatalAppExit16(ctx.s_tile2_bmp_1050_1538, str);
    FatalExit();
    puVar12 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        pu_var1 = puVar12;
        puVar12 = puVar12 + 0x1;
        u_var2 = *pu_var1;
        pu_var5 = puVar12;
        if ((u_var2 == HVar8) || (pu_var5 = (u_var2 + 0x1), pu_var5 == 0x0)) {
            return pu_var5;
        }
        iVar10 = -0x1;
        loop {
            if (iVar10 == 0x0) { break; }
            iVar10 += -0x1;
            pu_var1 = puVar12;
            puVar12 = (puVar12 + 0x1);
            if *pu_var1 == '\0' { break; }
        }
    }
}


pub fn dos3_call_set_struct_1000_42de(param_1: U32Ptr, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let pcVar4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u8;
    let uVar13: u32;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var5 = *param_1;
    uVar7 = (i_var8 + 0x2);
    u_var6 = (i_var8 + 0x4);
    u_var11 = (i_var8 + 0x6);
    u_var1 = (i_var8 + 0x8);
    uVar9 = (i_var8 + 0xa);
    // u_var10 = (param_3 >> 0x10);
    u_var2 = *param_3;
    u_var3 = (param_3 + 0x6);
    uVar12 = 0x0;
    if (true) {
        pcVar4 = swi(0x21);
        uVar13 = (*pcVar4)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
        uVar13 = CONCAT22(u_var11, u_var5);
    }
    *param_3 = u_var2;
    (param_3 + 0x6) = u_var3;
    // u_var11 = (param_2 >> 0x10);
    i_var8 = param_2;
    *param_2 = uVar13;
    (i_var8 + 0x2) = uVar7;
    (i_var8 + 0x4) = u_var6;
    (i_var8 + 0x6) = (uVar13 >> 0x10);
    (i_var8 + 0x8) = u_var1;
    (i_var8 + 0xa) = uVar9;
    if (uVar12) {
        pass1_1000_29af(uVar13);
    }
    (i_var8 + 0xc) = uVar12;
    return;
}


pub fn dos3_call_op_1000_435c(
    param_1: U32Ptr,
    param_2: u16,
    param_3: &mut u16,
    param_4: &mut i16,
    param_5: &mut String) {
    let pcVar1: u32;
    let UVar2: u16;
    let u_var3: u16;
    let mut extraout_dx: u16 = 0;
    let mut extraout_DX_00: u16 = 0;
    let mut extraout_DX_01: u16 = 0;
    let u_var4: u16;
    let cVar5: u8;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let iStack2: i16;

    iStack2 = param_4 + 0x1;
    if true {
        pcVar1 = swi(0x21);
        (*pcVar1)(ctx.data_seg);
        *param_3 = extraout_dx;
    }
    // else {
    //   DOS3Call(&ctx.PTR_LOOP_1050_1000);
    // }
    u_var3 = param_2;
    u_var4 = *param_3;
    if true {
        pcVar1 = swi(0x21);
        (*pcVar1)();
        *param_3 = extraout_DX_00;
    }
    // else {
    //   DOS3Call(&ctx.PTR_LOOP_1050_1000);
    // }
    uVar9 = param_3 >> 0x8;
    uVar8 = u_var3 & 0xff;
    u_var6 = u_var3 >> 0x8;
    uVar7 = u_var6;
    if true {
        pcVar1 = swi(0x21);
        (*pcVar1)();
        cVar5 = u_var6 as u8;
        *param_3 = extraout_DX_01;
    }
    // else {
    //   DOS3Call(&ctx.PTR_LOOP_1050_1000);
    //   cVar5 = u_var6;
    // }
    if (u_var4 != param_3) && (cVar5 == '\x17') {
        u_var3 = param_2;
        *param_3 = u_var4;
    }
    UVar2 = pass1_1000_462e(
        ctx, u_var3 - 0x7bc, param_3 >> 0x8, param_3 & 0xff, uVar7, uVar8, uVar9,
        &iStack2, param_5, param_3);
    if param_1._2_2_ != 0x0 {
        (param_1 + 0x2) = param_3;
        *param_1 = UVar2;
    }
    return;
}


pub fn dos3_call_1000_4f20(param_1: u16) -> u16

{
    let pcVar1: u32;
    let u_var2: u16;
    let u_var3: u8;

    u_var2 = 0x3950;
    u_var3 = 0x0;
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)(ctx.data_seg, param_1 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if (!u_var3) {
        return 0x0;
    }
    pass1_1000_29b5(ctx, u_var2);
    return 0xffff;
}

pub fn dos3_call_1000_514e(param_1: i16) -> u16

{
    let pcVar1: u32;
    let u_var2: u16;
    let u_var3: u8;

    u_var2 = SUB42(s__ld__s_1050_4150, 0x0);
    u_var3 = 0x0;
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)(ctx.data_seg, param_1 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if (!u_var3) {
        return 0x0;
    }
    pass1_1000_29b5(ctx, u_var2);
    return 0xffff;
}

pub fn dos3_call_1000_5174(param_1: u16) -> u16

{
    let pcVar1: u32;
    let u_var2: u16;
    let u_var3: u8;

    u_var2 = 0x6850;
    u_var3 = 0x0;
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)(param_1 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if (u_var3) {
        pass1_1000_29b5(ctx, u_var2);
        return u_var2 & 0xff;
    }
    return 0x0;
}


pub fn dos3_calls_1000_5198(param_1: i16) -> u16

{
    let pcVar1: u32;
    let u_var2: u8;
    let u_var3: u16;
    let bVar4: u8;
    let u_var5: u16;

    u_var2 = 0x4f;
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)(ctx.data_seg, param_1 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    u_var3 = CONCAT11(u_var2, u_var2);
    bVar4 = 0x0;
    if (true) {
        pcVar1 = swi(0x21);
        u_var3 = (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    u_var5 = bVar4 << 0x8;
    if (true) {
        pcVar1 = swi(0x21);
        (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if ((u_var5 & 0x100) != 0x0) {
        pass1_1000_29b5(ctx, u_var3);
        return u_var3 & 0xff;
    }
    return 0x0;
}


pub fn dos3_call_1000_51aa(param_1: u16) -> u16

{
    let pcVar1: u32;
    let u_var2: u8;
    let u_var3: u16;
    let bVar4: u8;
    let u_var5: u16;

    u_var2 = 0x4e;
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)(ctx.data_seg, param_1 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if (true) {
        pcVar1 = swi(0x21);
        u_var2 = (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    u_var3 = CONCAT11(u_var2, u_var2);
    bVar4 = 0x0;
    if (true) {
        pcVar1 = swi(0x21);
        u_var3 = (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    u_var5 = bVar4 << 0x8;
    if (true) {
        pcVar1 = swi(0x21);
        (*pcVar1)();
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if ((u_var5 & 0x100) != 0x0) {
        pass1_1000_29b5(ctx, u_var3);
        return u_var3 & 0xff;
    }
    return 0x0;
}


pub fn mixed_win_sys_op_1008_016e(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16
) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let version_part_2: u16;
    let u_var5: u32;
    let puVar6: U32Ptr;
    let extraout_dx: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let unaff_DI: i16;
    let uVar9: u16;
    let instance: HINSTANCE16;
    let u_var10: u16;
    let version: u32;
    let puVar12: u32;
    let uVar13: u32;
    let paVar14: &mut Struct20;
    local_1be: u8[0x80];
    local_13e: u8[0xac];
    local_92: u8[0x80];
    let uStack18: u16;
    let puStack16: U32Ptr;
    let puStack14: u32;
    let version_part_3: u16;
    let uStack8: u16;
    let version_part_1: u16;
    let puStack4: U32Ptr;

    instance = s_tile2_bmp_1050_1538;
    version = GetVersion16();
    // puVar7 = (DVar11 >> 0x10);
    version_part_1 = (version & 0xffff);
    version_part_2 = version & 0xff;
    version_part_3 = ((version & 0xffff) >> 0x8);
    uStack8 = version_part_2;
    puStack4 = puVar7;
    if (version_part_2 < 0x3) || (version_part_2 == 0x3 && (version_part_3 < 0xa)) {
        u_var10 = 0x1000;
        mem_op_1000_179c(ctx, 0xb4, puVar7, 0x1000);
        puVar6 = (puVar7 | version_part_2);
        uStack18 = version_part_2;
        puStack16 = puVar7;
        if (puVar6 == 0x0) {
            i_var3 = 0x0;
            puVar6 = 0x0;
        } else {
            u_var10 = SUB42(&ctx.PTR_LOOP_1050_1040, 0x0);
            i_var3 = string_1040_8520(CONCAT22(puVar7, version_part_2), 0x0, 0x10, 0x2, 0x5de,
                                     0x5dd, puVar6, param_2);
        }
        puStack14 = CONCAT22(puVar6, i_var3);
        ppcVar1 = (*puStack14 + 0x74);
        (**ppcVar1)(u_var10, i_var3, puVar6);
        instance = 0x1000;
        puVar7 = extraout_dx;
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    debug_print_1008_6048(s_version__d__d_1050_0012, instance, param_2);
    if ((uStack8 == 0x3) && (0xb < version_part_3)) {
        ctx.PTR_LOOP_1050_0010 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    }
    LoadString16(instance, 0x80, local_92, param_2);
    version_part_2 = dos3_call_1000_51aa(&stack0xfffe);
    if (version_part_2 != 0x0) {
        LoadString16(0x1000, 0x80, local_13e, param_2);
        LoadString16(s_tile2_bmp_1050_1538, 0x80, local_1be, param_2);
        version_part_2 = MessageBox16(ctx.s_tile2_bmp_1050_1538, &ctx.PTR_LOOP_1050_0010,
                                      local_13e, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    mem_op_1000_179c(0x4, puVar7, 0x1000);
    if ((puVar7 | version_part_2) == 0x0) {
        u_var10 = 0x0;
        puVar6 = 0x0;
        uStack18 = version_part_2;
        puStack16 = puVar7;
    } else {
        uStack18 = version_part_2;
        puStack16 = puVar7;
        puVar12 = pass1_1008_5394(CONCAT22(puVar7, version_part_2));
        // puVar6 = (puVar12 >> 0x10);
        u_var10 = SUB42(puVar12, 0x0);
    }
    // uVar9 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x8) = u_var10;
    (i_var3 + 0xa) = puVar6;
    u_var5 = (i_var3 + 0x8);
    pu_var2 = (i_var3 + 0x8);
    ctx._PTR_LOOP_1050_0298 = u_var5;
    *pu_var2 = 0x70;
    (pu_var2 + 0x2) = ctx.s_tile2_bmp_1050_1538;
    u_var10 = 0x1000;
    mem_op_1000_179c(0x126, puVar6, 0x1000);
    version_part_2 = u_var5;
    puVar7 = (puVar6 | version_part_2);
    uStack18 = version_part_2;
    puStack16 = puVar6;
    if (puVar7 != 0x0) {
        u_var10 = 0x1010;
        uVar13 = pass1_1010_2024(CONCAT13((puVar6 >> 0x8),
                                          CONCAT12(puVar6, version_part_2)));
        // puVar7 = (uVar13 >> 0x10);
        version_part_2 = uVar13;
    }
    if (ctx.PTR_LOOP_1050_0ed0 == 0x0) {
        debug_print_1008_6048(s_New_failed_in_Op__Op_1050_0020, u_var10, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    u_var10 = 0x1000;
    mem_op_1000_179c(0xe8c, puVar7, 0x1000);
    puVar6 = (puVar7 | version_part_2);
    uStack18 = version_part_2;
    puStack16 = puVar7;
    if (puVar6 != 0x0) {
        u_var10 = 0x1010;
        pass1_1010_7e40(CONCAT22(puVar7, version_part_2), puVar6, unaff_DI, param_2);
    }
    if (ctx.PTR_LOOP_1050_14cc == 0x0) {
        debug_print_1008_6048(0x10500035, u_var10, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    u_var10 = 0x1000;
    mem_op_1000_179c(0xb0, puVar6, 0x1000);
    puVar7 = (puVar6 | version_part_2);
    uStack18 = version_part_2;
    puStack16 = puVar6;
    if (puVar7 != 0x0) {
        u_var10 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
        paVar14 = pass1_1038_aeca(CONCAT22(puVar6, version_part_2), param_2);
        // puVar7 = (paVar14 >> 0x10);
        version_part_2 = paVar14;
    }
    if (ctx.PTR_LOOP_1050_5b7c == 0x0) {
        debug_print_1008_6048(s_New_failed_in_Op__Op__DialogCtr_1050_0053, u_var10, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    u_var10 = 0x1000;
    mem_op_1000_179c(0xa, puVar7, 0x1000);
    puVar6 = (puVar7 | version_part_2);
    uStack18 = version_part_2;
    puStack16 = puVar7;
    if (puVar6 != 0x0) {
        u_var10 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
        make_proc_inst_1038_cf6c(CONCAT22(puVar7, version_part_2), puVar6, &ctx.PTR_LOOP_1050_1038);
    }
    if (ctx.PTR_LOOP_1050_5bc8 == 0x0) {
        debug_print_1008_6048(s_New_failed_in_Op__Op__DialogHand_1050_0073, u_var10, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (puVar6 | version_part_2);
    uStack18 = version_part_2;
    puStack16 = puVar6;
    if (puVar7 != 0x0) {
        pass1_1008_5bdc(CONCAT22(puVar6, version_part_2), unaff_DI, param_2);
    }
    if (ctx.PTR_LOOP_1050_02a0 == 0x0) {
        debug_print_1008_6048(s_New_failed_in_Op__Op__Simulator_1050_0097, 0x1000, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    mem_op_1000_179c(0xfc, puVar7, 0x1000);
    uVar8 = puVar7 | version_part_2;
    uStack18 = version_part_2;
    puStack16 = puVar7;
    if (uVar8 == 0x0) {
        version_part_2 = 0x0;
        uVar8 = 0x0;
    } else {
        set_struct_op_1008_0536(CONCAT22(puVar7, version_part_2), 0x1000, param_2);
    }
    (i_var3 + 0x4) = version_part_2;
    (i_var3 + 0x6) = uVar8;
    if ((i_var3 + 0x4) == 0x0) {
        debug_print_1008_6048(s_New_failed_in_Op__Op_1050_00b7, 0x1000, param_2);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    win_ui_reg_class_1008_96d2(ctx, (i_var3 + 0x4), 0x1000, param_2);
    u_var5 = (i_var3 + 0x4);
    ppcVar1 = ((i_var3 + 0x4) + 0x8);
    (**ppcVar1)(0x1000, u_var5, (u_var5 >> 0x10));
    u_var5 = (i_var3 + 0x4);
    ctx.PTR_LOOP_1050_0396 = (u_var5 + 0x8);
    ppcVar1 = ((i_var3 + 0x4) + 0xc);
    (**ppcVar1)(0x1000, (i_var3 + 0x4), 0x3);
    UpdateWindow16(0x1000);
    return;
}


pub fn kill_timer_1008_921c(param_1: U32Ptr, param_2: HWND16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x9416;
    (i_var1 + 0x2) = 0x1008;
    KillTimer16(param_2, 0x1);
    ctx._PTR_LOOP_1050_0388 = 0x0;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0x6)));
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    return;
}


pub fn win_msg_op_1008_9498(
    ctx: &mut AppContext,
    in_msg_1: &mut MSG16,
    in_msg_2: &mut String) -> WPARAM16 {
    let b_var1: bool;
    let IVar2: i16;
    let local_msg_1: MSG16;

//LAB_1008_949c:
    b_var1 = GetMessage16(in_msg_1, 0x0, 0x0, 0x0);
    if b_var1 == false {
        return local_msg_1.wparam;
    }
    if (ctx.PTR_LOOP_1050_5bc8 + 0x8) != 0x0 {
        // goto code_r0x100894cd;
    }
//   TODO: goto LAB_1008_94dc;
    code_r0x100894cd: in_msg_1 = s_tile2_bmp_1050_1538;
    b_var1 = IsDialogMessage16(ctx.s_tile2_bmp_1050_1538, &local_msg_1);
    if b_var1 == false {
//LAB_1008_94dc:
        if ctx.PTR_LOOP_1050_0398 != 0x0 {
            in_msg_1 = s_tile2_bmp_1050_1538;
            IVar2 = TranslateAccelerator16(ctx.s_tile2_bmp_1050_1538, (HACCEL16) & local_msg_1, in_msg_2);
            if (IVar2 != 0x0) {
                goto
                LAB_1008_949c;
            }
        }
        TranslateMessage16(s_tile2_bmp_1050_1538);
        in_msg_1 = s_tile2_bmp_1050_1538;
        DispatchMessage16(s_tile2_bmp_1050_1538);
    }
//   TODO: goto LAB_1008_949c;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_win_msg_op_1008_9510(
    ctx: &mut AppContext,
    param_1: &mut i16,
    param_2: &mut MSG16,
    param_3: &MSG16
) {
    let has_message: bool;
    let IVar1: i16;
    let local_14: MSG16;

//LAB_1008_9578:
    if *param_1 != 0x0 {
        has_message = GetMessage16(param_2, 0x0, 0x0, 0x0);
        if has_message != false {
            if (ctx.PTR_LOOP_1050_5bc8 + 0x8) != 0x0 {}
            // goto code_r0x10089538;
//       TODO: goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
        param_2 = s_tile2_bmp_1050_1538;
    has_message = IsDialogMessage16(ctx.s_tile2_bmp_1050_1538, &local_14);
    if has_message == 0x0 {
//LAB_1008_9547:
        if ctx.PTR_LOOP_1050_0398 != 0x0 {
            param_2 = s_tile2_bmp_1050_1538;
            IVar1 = TranslateAccelerator16(ctx.s_tile2_bmp_1050_1538, (HACCEL16) & local_14, param_3);
            if IVar1 != 0x0 {
                // goto LAB_1008_9578;
            }
        }
        TranslateMessage16(s_tile2_bmp_1050_1538);
        param_2 = s_tile2_bmp_1050_1538;
        DispatchMessage16(s_tile2_bmp_1050_1538);
    }
//   TODO: goto LAB_1008_9578;
}


pub fn get_sys_metrics_1010_46f6(param_1: u32) {
    let u_var1: u16;
    let IVar2: i16;
    let i_var3: i16;
    let in_DX: U32Ptr;
    let i_var4: i16;
    let unaff_DI: i16;
    let u_var5: u16;
    let unaff_SS: u16;
    let puVar6: U32Ptr;
    let uVar7: u32;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let local_6: i16;
    let local_4: i16;

    puVar9 = CONCAT22(unaff_SS, &local_4);
    puVar8 = CONCAT22(unaff_SS, &local_6);
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                    puVar8, puVar9);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    uVar7 = pass1_1008_4772((i_var4 + 0x66));
    // u_var1 = (uVar7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 0x8;
    (i_var4 + 0x1a) = local_6 + 0x9;
    IVar2 = GetSystemMetrics16(0x1008);
    (i_var4 + 0x1c) = IVar2 * 0x2 + (uVar7 + 0x4);
    IVar2 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    (i_var4 + 0x1e) = i_var3 + IVar2 + (uVar7 + 0x8);
    return;
}


pub fn win_sys_op_1010_5404(param_1: &mut Struct54, param_2: &mut Struct19, param_3: u16, param_4: &mut WNDCLASS16) {
    let pi_var1: U32Ptr;
    u16 * *ppu_var2;
    let u_var3: u32;
    let puVar4: u32;
    let ppcVar5: u32;
    let mut pCVar6: String;
    let iVar7: i16;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let puVar11: U32Ptr;
    let extraout_dx: U32Ptr;
    let puVar12: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let extraout_DX_01: U32Ptr;
    let puVar13: U32Ptr;
    let puVar14: U32Ptr;
    let unaff_DI: i16;
    let uVar15: u16;
    let mut pCVar16: String;
    let index: i16;
    let paVar17: &mut Struct79;
    let mut pcVar18: String;
    let puVar19: U32Ptr;
    let uVar20: u16;
    let local_134: [u8; 102];
    let puStack50: U32Ptr;
    let uStack46: u16;
    let puStack44: U32Ptr;
    let iStack42: i16;
    let iStack26: i16;
    let puStack24: U32Ptr;
    let iStack22: i16;
    let puStack20: U32Ptr;
    let uStack16: u32;
    let iStack12: i16;
    let uStack10: i16;
    let uStack8: u16;
    let puStack6: U32Ptr;
    let uStack4: u16;

    paVar17 = struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    // puVar12 = (paVar17 >> 0x10);
    uVar15 = 0x0;
    &param_1.field_0xa = 0x0;
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    &param_1.field_0x1a = 0x0;
    param_1.field_0x62 = 0x0;
    param_1.field_0x64 = 0x0;
    &param_1.field_0x68 = 0x0;
    &param_1.field_0x6c = 0x0;
    param_1.field_0x70 = 0x1;
    param_1.field_0x7a = 0x0;
    param_1.field_0x7c = 0x0;
    param_1.field_0x7e = 0x0;
    param_1.field_0x80 = 0x0;
    param_1.field_0x82 = 0x1;
    CONCAT22(param_2, param_1) = 0x6312;
    param_1.field_0x2 = 0x1010;
    pass1_1010_6034(CONCAT22(param_2, param_1), puVar12);
    mem_op_1000_179c(0x101, puVar12, 0x1000);
    &param_1.field_0xe = uVar15;
    (&param_1.field_0xe + 0x2) = puVar12;
    pass1_1000_5008(&param_1.field_0xe, puVar12, 0x100, &stack0xfffe);
    uStack4 = str_op_1000_3da4(param_1.field_0xe);
    pcVar18 = param_1.field_0xe;
    // uVar15 = (pcVar18 >> 0x10);
    puVar13 = (pcVar18 + uStack4);
    if (puVar13[-0x1] != '\\') {
        *puVar13 = 0x5c;
        pcVar18 = param_1.field_0xe;
        *(pcVar18 + uStack4 + 0x1) = 0x0;
    }
    pcVar18 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                    (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1000);
    // puVar12 = (pcVar18 >> 0x10);
    uStack8 = SUB42(pcVar18, 0x0);
    puStack6 = puVar12;
    string_1000_3cea(param_1.field_0xe, pcVar18);
    pCVar6 = str_op_1008_60e8(param_1.field_0xe, puVar12);
    param_1.field_0xa = pCVar6;
    param_1.field_0xc = puVar12;
    pcVar18 = param_1.field_0xe;
    pCVar6 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(0x1008, param_1.field_0xa, puVar12,
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar6 = &ctx.PTR_LOOP_1050_1000;
        iStack22 = pass1_1000_3e2c(param_1.field_0xe);
        puVar19 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_4, puVar12, unaff_DI);
        // puVar12 = (puVar19 >> 0x10);
        iStack26 = puVar19;
        iStack10 = (iStack26 + 0xa);
        iStack12 = (iStack26 + 0xc);
        param_1.field_0x62 = (iStack22 != iStack10);
        puStack24 = puVar12;
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar16 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar16 = &ctx.PTR_LOOP_1050_1000;
        iVar7 = string_1000_475e(ctx, param_1.field_0xe, 0x105013c4);
        if (iVar7 == 0x0) {
            param_1.field_0x80 = 0x1;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar6 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar6 = &ctx.PTR_LOOP_1050_1000;
        iVar7 = string_1000_475e(ctx, param_1.field_0xe, 0x105013c8);
        if (iVar7 == 0x0) {
            param_1.field_0x74 = 0x0;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar16 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar16 = &ctx.PTR_LOOP_1050_1000;
        iVar7 = string_1000_475e(ctx, param_1.field_0xe, 0x105013c8);
        if (iVar7 == 0x0) {
            param_1.field_0x72 = 0x0;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar6 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar6 = &ctx.PTR_LOOP_1050_1000;
        iVar7 = string_1000_475e(ctx, param_1.field_0xe, 0x105013c8);
        if (iVar7 == 0x0) {
            param_1.field_0x1e = 0x0;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar16 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar16 = &ctx.PTR_LOOP_1050_1000;
        iVar7 = string_1000_475e(ctx, param_1.field_0xe, 0x105013c8);
        if (iVar7 == 0x0) {
            param_1.field_0x20 = 0x0;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar6 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    puVar11 = puVar12;
    if (*param_1.field_0xe != '\0') {
        pCVar6 = &ctx.PTR_LOOP_1050_1000;
        uStack46 = pass1_1000_3e2c(param_1.field_0xe);
        puVar11 = (puVar12 | uStack46);
        puStack44 = puVar12;
        if ((puVar12 | uStack46) != 0x0) {
            param_1.field_0x76 = uStack46;
            param_1.field_0x78 = puVar12;
            puVar11 = puVar12;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar16 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar16 = &ctx.PTR_LOOP_1050_1000;
        iVar7 = string_1000_475e(ctx, param_1.field_0xe, 0x105013c4);
        if (iVar7 == 0x0) {
            param_1.field_0x7a = 0x1;
        }
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar6 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar6 = 0x1008;
        uVar8 = str_op_1008_60e8(param_1.field_0xe, puVar11);
        param_1.field_0x1a = uVar8;
        param_1.field_0x1c = puVar11;
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    pCVar16 = ctx.s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pCVar16 = 0x1008;
        uVar8 = str_op_1008_60e8(param_1.field_0xe, puVar11);
        param_1.field_0x68 = uVar8;
        param_1.field_0x6a = puVar11;
    }
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    index = ctx.s_tile2_bmp_1050_1538;
    puVar9 = GetPrivateProfileString16(pCVar16, u_var3, (u_var3 >> 0x10),
                                       (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18, (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        index = 0x1008;
        puVar9 = str_op_1008_60e8(param_1.field_0xe, puVar11);
        param_1.field_0x6c = puVar9;
        param_1.field_0x6e = puVar11;
    }
    if (param_1.field_0x62 == 0x0) {
        uVar15 = SUB42(ctx.s_tile2_bmp_1050_1538, 0x0);
        uStack46 = GetSystemMetrics16(index);
        iStack42 = 0x1;
        loop {
            get_private_profile_string_1010_6132(CONCAT22(param_2, param_1), iStack42, uVar15);
            puVar14 = &param_1.field_0x0 + iStack42 * 0x4;
            if ((((puVar14[0x11] < 0x0) || (puVar14[0x12] < 0x0)) || (pi_var1 = puVar14 + 0x11,
                                                                      *pi_var1 != iStack10 - uStack46 && (iStack10 - uStack46) <= *pi_var1)) || (puVar9 = (iStack12 - uStack46), ppu_var2 = (puVar14 + 0x12),
                                                                                                                                               *ppu_var2 != puVar9 && puVar9 <= *ppu_var2)) {
                uVar15 = 0x1000;
                puVar9 = pass1_1000_4906(
                    CONCAT22(param_2, &param_1.field_0x22 + iStack42 * 0x8),
                    0x0, 0x8);
            }
            iStack42 += 0x1;
            if iStack42 >= 0x8 { break; }
        }
    }
    mem_op_1000_179c(0xc, puVar11, 0x1000);
    puStack50 = CONCAT22(puVar11, puVar9);
    if ((puVar11 | puVar9) == 0x0) {
        puVar9 = 0x0;
        puVar12 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar11, puVar9));
        puVar12 = extraout_dx;
    }
    &param_1.field_0x64 = puVar9;
    (&param_1.field_0x64 + 0x2) = puVar12;
    pcVar18 = param_1.field_0xe;
    pass1_1000_5008(pcVar18, (pcVar18 >> 0x10), 0x100, &stack0xfffe,
    );
    uStack4 = str_op_1000_3da4(param_1.field_0xe);
    pcVar18 = param_1.field_0xe;
    // uVar15 = (pcVar18 >> 0x10);
    puVar13 = (pcVar18 + uStack4);
    if (puVar13[-0x1] != '\\') {
        *puVar13 = 0x5c;
        pcVar18 = param_1.field_0xe;
        *(pcVar18 + uStack4 + 0x1) = 0x0;
    }
    u_var10 = str_op_1008_60e8(param_1.field_0xe, puVar12);
    uStack16 = CONCAT22(puVar12, u_var10);
    mem_op_1000_179c(0x8, puVar12, 0x1000);
    puStack50 = CONCAT22(puVar12, u_var10);
    if ((puVar12 | u_var10) == 0x0) {
        puStack20 = 0x0;
    } else {
        *puStack50 = 0x389a;
        (u_var10 + 0x2) = 0x1008;
        (u_var10 + 0x4) = uStack16;
        *puStack50 = 0x6322;
        (u_var10 + 0x2) = 0x1010;
        puStack20 = puStack50;
    }
    puVar4 = param_1.field_0x64;
    ppcVar5 = (*param_1.field_0x64 + 0x4);
    (**ppcVar5)(0x1000, puVar4, (puVar4 >> 0x10), puStack20,
                (puStack20 >> 0x10));
    pcVar18 = param_1.field_0xe;
    u_var3 = &param_1.field_0xa;
    puVar12 = extraout_DX_00;
    GetPrivateProfileString16(&ctx.PTR_LOOP_1050_1000, u_var3, (u_var3 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), pcVar18,
                              (pcVar18 >> 0x10));
    if (*param_1.field_0xe != '\0') {
        pcVar18 = param_1.field_0xe;
        uVar15 = SUB42(pcVar18, 0x0);
        // uVar20 = (pcVar18 >> 0x10);
        while (uStack46 = pass1_1000_47a4(CONCAT22(uVar20, uVar15), 0x105013f8, param_4),
               (puVar12 | uStack46) != 0x0) {
            puStack44 = puVar12;
            string_1000_3d3e(CONCAT22(param_4, local_134), CONCAT22(puVar12, uStack46));
            uStack4 = str_op_1000_3da4(CONCAT22(param_4, local_134));
            if ((&stack0xfecb)[uStack4] != '\\') {
                local_134[uStack4] = 0x5c;
                local_134[uStack4 + 0x1] = 0x0;
            }
            u_var10 = str_op_1008_60e8(CONCAT22(param_4, local_134), puVar12);
            uStack16 = CONCAT22(puVar12, u_var10);
            mem_op_1000_179c(0x8, puVar12, 0x1000);
            puStack50 = CONCAT22(puVar12, u_var10);
            if ((puVar12 | u_var10) == 0x0) {
                puStack20 = 0x0;
            } else {
                *puStack50 = 0x389a;
                (u_var10 + 0x2) = 0x1008;
                (u_var10 + 0x4) = uStack16;
                *puStack50 = 0x6322;
                (u_var10 + 0x2) = 0x1010;
                puStack20 = puStack50;
            }
            puVar4 = param_1.field_0x64;
            ppcVar5 = (*param_1.field_0x64 + 0x8);
            (**ppcVar5)(0x1000, puVar4, (puVar4 >> 0x10), puStack20,
                        (puStack20 >> 0x10));
            uVar15 = 0x0;
            uVar20 = 0x0;
            puVar12 = extraout_DX_01;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_private_profile_str_1010_5b10(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let ppc_var4: u32;
    let mut pCVar5: String;
    let in_DX: U32Ptr;
    let iVar6: i16;
    let unaff_DI: i16;
    let uVar7: u16;
    let unaff_SS: u16;
    let in_AF: u8;
    let puVar8: U32Ptr;
    let iStack12: i16;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x6312;
    (iVar6 + 0x2) = 0x1010;
    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    u_var3 = (iVar6 + 0xe);
    sys_1000_3f9c(u_var3, (u_var3 >> 0x10), 0x149c,
                  ctx.data_seg, (puVar8 + 0xa), &stack0xfffe,
                  uVar7, 0x1000, unaff_SS, in_AF);
    if ((iVar6 + 0x80) == 0x0) {
        pCVar5 = 0x13c8;
    } else {
        pCVar5 = 0x13c4;
    }
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(&ctx.PTR_LOOP_1050_1000, u_var3, (u_var3 >> 0x10),
                                pCVar5);
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                (iVar6 + 0xe));
    if ((iVar6 + 0x1e) == 0x0) {
        pCVar5 = 0x13c8;
    } else {
        pCVar5 = 0x13c4;
    }
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                pCVar5);
    if ((iVar6 + 0x74) == 0x0) {
        pCVar5 = 0x13c8;
    } else {
        pCVar5 = 0x13c4;
    }
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                pCVar5);
    if ((iVar6 + 0x72) == 0x0) {
        pCVar5 = 0x13c8;
    } else {
        pCVar5 = 0x13c4;
    }
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                pCVar5);
    if ((iVar6 + 0x20) == 0x0) {
        pCVar5 = 0x13c8;
    } else {
        pCVar5 = 0x13c4;
    }
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                pCVar5);
    u_var3 = (iVar6 + 0xe);
    sys_1000_3f9c(u_var3, (u_var3 >> 0x10), 0x14a2,
                  ctx.data_seg, (iVar6 + 0x76),
                  &stack0xfffe, uVar7, 0x1000, unaff_SS, in_AF);
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(&ctx.PTR_LOOP_1050_1000, u_var3, (u_var3 >> 0x10),
                                (iVar6 + 0xe));
    if ((iVar6 + 0x7a) == 0x0) {
        pCVar5 = 0x13c8;
    } else {
        pCVar5 = 0x13c4;
    }
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                pCVar5);
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                (iVar6 + 0x1a));
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                (iVar6 + 0x68));
    u_var3 = (iVar6 + 0xa);
    WritePrivateProfileString16(ctx.s_tile2_bmp_1050_1538, u_var3, (u_var3 >> 0x10),
                                (iVar6 + 0x6c));
    iStack12 = 0x1;
    loop {
        // switchD_1010:2ab5::caseD_13(param_1,iStack12);
        iStack12 += 0x1;
        if iStack12 >= 0x8 { break; }
    }
    fn_ptr_1000_17ce(ctx, (iVar6 + 0xa), 0x1000);
    fn_ptr_1000_17ce(ctx, (iVar6 + 0xe), 0x1000);
    fn_ptr_1000_17ce(ctx, (iVar6 + 0x12), 0x1000);
    fn_ptr_1000_17ce(ctx, (iVar6 + 0x16), 0x1000);
    fn_ptr_1000_17ce(ctx, (iVar6 + 0x1a), 0x1000);
    pu_var1 = (iVar6 + 0x64);
    u_var2 = (iVar6 + 0x66);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var4 = *pu_var1;
        (**ppc_var4)(0x1000, pu_var1, u_var2, 0x1);
    }
    fn_ptr_1000_17ce(ctx, (iVar6 + 0x68), 0x1000);
    fn_ptr_1000_17ce(ctx, (iVar6 + 0x6c), 0x1000);
    pass1_1010_1d80(param_1, unaff_SS);
    return;
}


pub fn get_private_profile_string_1010_6132(param_1: u32, param_2: i16, param_3: &String) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let in_DX: u16;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let unaff_SS: u16;

    // uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    u_var1 = (iVar7 + 0xe);
    u_var2 = (iVar7 + 0xa);
    GetPrivateProfileString16(param_3, u_var2, (u_var2 >> 0x10),
                              (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), u_var1,
                              (u_var1 >> 0x10));
    if (*(iVar7 + 0xe) != '\0') {
        u_var3 = pass1_1000_47a4((iVar7 + 0xe), 0x105014a6, unaff_SS);
        u_var5 = in_DX | u_var3;
        if (u_var5 != 0x0) {
            i_var4 = pass1_1000_3e2c(CONCAT22(in_DX, u_var3));
            iVar7 = param_2 * 0x8 + iVar7;
            (iVar7 + 0x22) = i_var4;
            u_var3 = pass1_1000_47a4(0x0, 0x105014a8, unaff_SS);
            u_var6 = u_var5 | u_var3;
            if (u_var6 != 0x0) {
                i_var4 = pass1_1000_3e2c(CONCAT22(u_var5, u_var3));
                (iVar7 + 0x24) = i_var4;
                u_var3 = pass1_1000_47a4(0x0, 0x105014aa, unaff_SS);
                u_var5 = u_var6 | u_var3;
                if (u_var5 != 0x0) {
                    i_var4 = pass1_1000_3e2c(CONCAT22(u_var6, u_var3));
                    (iVar7 + 0x26) = i_var4;
                    u_var3 = pass1_1000_47a4(0x0, 0x105014ac, unaff_SS);
                    if ((u_var5 | u_var3) != 0x0) {
                        i_var4 = pass1_1000_3e2c(CONCAT22(u_var5, u_var3));
                        (iVar7 + 0x28) = i_var4;
                    }
                }
            }
        }
    }
    return;
}


pub fn set_err_mode_1010_8b14(param_1: &mut Struct87, param_2: i32, param_3: u16) -> u32

{
    let u_var1: u16;
    let u_var2: u16;
    let lVar3: i32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0xe84));
    SetErrorMode16(0x1008);
    loop {
        lVar3 = pass1_1008_5b12(local_a, param_3);
        if (lVar3 == 0x0) {
            SetErrorMode16(0x1008);
            return param_2;
        }
        u_var1 = param_1 + 0xa82;
        string_1000_3d3e((param_1 & 0xffff0000 | u_var1), (lVar3 + 0x4));
        string_1000_3cea(param_1 & 0xffff0000 | u_var1, param_2);
        u_var2 = dos3_call_1000_51aa(&stack0xfffe);
        if u_var2 == 0 { break; }
    }
    SetErrorMode16(0x1000);
    return param_1 & 0xffff0000 | u_var1;
}


pub fn get_sys_metrics_1018_09a8(param_1: u32, param_2: i16) {
    let u_var1: u32;
    let IVar2: i16;
    let i_var3: i16;
    let in_DX: U32Ptr;
    let i_var4: i16;
    let unaff_DI: i16;
    let u_var5: u16;
    let unaff_SS: u16;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let local_a: i16;
    let local_8: i16;
    let i_stack6: i16;
    let IStack4: i16;

    IStack4 = GetSystemMetrics16(param_2);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    i_stack6 = (i_var4 + 0x12) + -0x2;
    puVar8 = CONCAT22(unaff_SS, &local_8);
    puVar7 = CONCAT22(unaff_SS, &local_a);
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                    puVar7, puVar8);
    (i_var4 + 0x18) = i_stack6 * IStack4 + local_8 + 0x146;
    (i_var4 + 0x1a) = i_stack6 * IStack4 + local_a + 0x9;
    IVar2 = GetSystemMetrics16(0x1008);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1c) = IVar2 * 0x2 + (u_var1 + 0x4);
    IVar2 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1e) = i_var3 + IVar2 + (u_var1 + 0x8);
    return;
}


pub fn get_sys_metrics_1018_1ea0(param_1: &mut Struct55, param_2: u16) {
    let IVar1: i16;
    let IVar2: i16;
    let i_var3: &mut Struct55;
    let u_var3: u16;

    IVar1 = GetSystemMetrics16(param_2);
    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    i_var3.field_0x2e = IVar1 * 0x2 + i_var3.field_0x36;
    IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    IVar2 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var3.field_0x30 = IVar1 + i_var3.field_0x38 + IVar2;
    return;
}


pub fn mixed_sys_op_1018_2978(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let rect: *mut RECT16;
    let i_var4: i16;
    let in_DX: U32Ptr;
    let u_var5: u16;
    let extraout_dx: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u8;
    let paStack62: &mut Struct76;
    let local_3a: RECT16;
    let iStack54: i16;
    let iStack52: i16;
    let uStack50: u32;
    let puStack46: u32;
    let local_2a: [u8; 24];
    let uStack6: u16;

    pass1_1010_8096(ctx.PTR_LOOP_1050_14cc, 0x1);
    pu_var2 = local_2a;
    uStack6 = param_2;
    struct_op_1008_48fe(CONCAT22(param_3, pu_var2), 0x1,
                        CONCAT22(in_DX, param_2), in_DX);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, in_DX, 0x1000);
    u_var5 = in_DX | pu_var2;
    if (u_var5 == 0x0) {
        pu_var3 = 0x0;
        u_var5 = 0x0;
    } else {
        pu_var3 = local_2a;
        uVar9 = 0x1008;
        struct_op_1008_3f92(CONCAT22(in_DX, pu_var2),
                            CONCAT22(param_3, pu_var3));
    }
    puStack46 = CONCAT22(u_var5, pu_var3);
    ppcVar1 = (*puStack46 + 0x14);
    (**ppcVar1)(uVar9, pu_var3, u_var5);
    uStack50 = CONCAT22(extraout_dx, pu_var3);
    puVar6 = extraout_dx;
    mem_op_1000_179c(0x14, extraout_dx, 0x1000);
    puVar7 = (puVar6 | pu_var3);
    if (puVar7 == 0x0) {
        pu_var3 = 0x0;
        puVar7 = 0x0;
    } else {
        struct_1008_4c58(
            CONCAT13((puVar6 >> 0x8), CONCAT12(puVar6, pu_var3)));
    }
    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    (i_var8 + 0xe) = pu_var3;
    (i_var8 + 0x10) = puVar7;
    pass1_1008_4d84((i_var8 + 0xe), uStack50, puVar7);
    uVar12 = SUB21(ctx.PTR_LOOP_1050_0396, 0x0);
    rect = &local_3a;
    GetClientRect16(0x1008, rect);
    u_var11 = 0x1e;
    u_var10 = 0x1000;
    mem_op_1000_179c(0x1e, puVar7, 0x1000);
    paStack62 = CONCAT22(puVar7, rect);
    u_var5 = puVar7 | rect;
    if (u_var5 == 0x0) {
        (i_var8 + 0xa) = 0x0;
    } else {
        i_var4 = (iStack52 - local_3a.y) + 0x1;
        u_var10 = 0x1008;
        pass1_1008_405c(ctx, paStack62, (i_var8 + 0xe), i_var4, (iStack54 - local_3a.x) + 0x1);
        (i_var8 + 0xa) = i_var4;
        (i_var8 + 0xc) = u_var5;
    }
    if (puStack46 != 0x0) {
        ppcVar1 = *puStack46;
        (**ppcVar1)(u_var10, puStack46, (puStack46 >> 0x10), 0x1, u_var11, uVar12);
    }
    close_file_1008_496c(local_2a, param_3);
    return;
}


pub fn get_sys_metrics_1018_2f56(param_1: u32) {
    let u_var1: u16;
    let IVar2: i16;
    let i_var3: i16;
    let in_DX: U32Ptr;
    let i_var4: i16;
    let unaff_DI: i16;
    let u_var5: u16;
    let unaff_SS: u16;
    let puVar6: U32Ptr;
    let uVar7: u32;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let local_6: i16;
    let local_4: i16;

    puVar9 = CONCAT22(unaff_SS, &local_4);
    puVar8 = CONCAT22(unaff_SS, &local_6);
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                    puVar8, puVar9);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    uVar7 = pass1_1008_4772((i_var4 + 0x24));
    // u_var1 = (uVar7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 0xb5;
    (i_var4 + 0x1a) = local_6 + 0x9;
    IVar2 = GetSystemMetrics16(0x1008);
    (i_var4 + 0x1c) = IVar2 * 0x2 + (uVar7 + 0x4);
    IVar2 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    (i_var4 + 0x1e) = i_var3 + IVar2 + (uVar7 + 0x8);
    return;
}


pub fn sprintf_op_1018_34b6(param_1: u32, param_2: u8) {
    let i_var1: i16;
    undefined3
    in_register_00000001;
    let in_DX: u16;
    let i_var2: i16;
    let valist: U32Ptr;
    buffer: &mut String;
    let unaff_SS: u16;
    let u_var3: u32;
    let lVar4: i32;

    // valist = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var3 = switch_1018_3b9e(param_1, (i_var2 + 0x12e),
                             CONCAT31(in_register_00000001, param_2), in_DX, unaff_SS);
    i_var1 = (i_var2 + 0x12e);
    if (i_var1 == 0x188) {
        lVar4 = pass1_1008_57f0(u_var3, (i_var2 + 0x130), unaff_SS);
        buffer = 0x1020;
        string_1020_c0d8((lVar4 + 0xe));
    } else {
        if (i_var1 == 0x18b) {
            buffer = 0x1008;
            pass1_1008_57f0(u_var3, (i_var2 + 0x130), unaff_SS);
        } else {
            if (i_var1 != 0x18c) {
                load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                                      (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x100,
                                      (i_var2 + 0x22), valist);
                return;
            }
            buffer = 0x1008;
            pass1_1008_57f0(u_var3, (i_var2 + 0x130), unaff_SS);
        }
    }
    wsprintf16(buffer, (i_var2 + 0x22), valist);
    return;
}


pub fn get_sys_metrics_1018_4b1e(param_1: &mut Struct55, param_2: u16, param_3: u16) -> u16

{
    let i_var1: i16;
    let u_var2: u16;

    struct_op_1010_1d48(param_1, param_3);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x12) = param_2;
    (i_var1 + 0x14) = 0x0;
    param_1.field_0x0 = &ctx.PTR_LOOP_1050_4c9e;
    (i_var1 + 0x2) = 0x1018;
    if (ctx.PTR_LOOP_1050_416c == 0x0) {
        ctx.PTR_LOOP_1050_416c = GetSystemMetrics16(0x1010);
        ctx.PTR_LOOP_1050_416e = GetSystemMetrics16(s_tile2_bmp_1050_1538);
        ctx.PTR_LOOP_1050_4170 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    }
    return param_1;
}


pub fn get_sys_metrics_1020_7c1a(param_1: U32Ptr, param_2: u32, param_3: i16) {
    let IVar1: i16;
    let i_var3: &mut Struct56;
    let u_var3: u16;
    let u_var4: u16;
    let u_var1: u16;

    // u_var3 = (param_2 >> 0x10);
    u_var1 = (param_2 + 0x8);
    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    *param_1 = 0x3aa8;
    i_var3.field_0x2 = 0x1008;
    i_var3.field_0x4 = u_var1;
    *param_1 = 0x3ab0;
    i_var3.field_0x2 = 0x1008;
    i_var3.field_0x6 = param_2;
    i_var3.field_0xa = 0x0;
    i_var3.field_0xe = 0x0;
    i_var3.field_0x10 = 0x0;
    i_var3.field_0x12 = 0x0;
    *param_1 = 0x7f72;
    i_var3.field_0x2 = 0x1020;
    i_var3.field_0xa = (param_2 + 0xe4);
    IVar1 = GetSystemMetrics16(param_3);
    i_var3.field_0xe = IVar1;
    IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var3.field_0x10 = IVar1;
    IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var3.field_0x12 = IVar1;
    return;
}


pub fn make_proc_inst_1038_cf6c(param_1: U32Ptr, param_2: U32Ptr, param_3: U32Ptr) {
    pvVar1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    *param_1 = 0x389a;
    (i_var2 + 0x2) = 0x1008;
    (i_var2 + 0x4) = 0x0;
    (i_var2 + 0x8) = 0x0;
    *param_1 = 0xd23e;
    (i_var2 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    ctx._PTR_LOOP_1050_5bc8 = param_1;
    pvVar1 = MakeProcInstance16(param_3, PTR_LOOP_1050_038c);
    (i_var2 + 0x4) = pvVar1;
    (i_var2 + 0x6) = param_2;
    ctx.PTR_LOOP_1050_5bcc =

        MakeProcInstance16(s_tile2_bmp_1050_1538, PTR_LOOP_1050_038c);
    ctx.PTR_LOOP_1050_5bce = param_2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn free_proc_inst_1038_cfda(param_1: U32Ptr, param_2: U32Ptr) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xd23e;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    FreeProcInstance16(param_2);
    FreeProcInstance16(s_tile2_bmp_1050_1538);
    (i_var1 + 0x4) = 0x0;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    return;
}



long
call_win_proc_1038_d020
(param_1: HWND16,param_2: u32,param_3: LPARAM,param_4: u16,param_5: HWND16)

{
let ppcVar1: u32; wparam: WPARAM16; let HVar2: HANDLE16; let HVar3: HANDLE16; let u_var4: u16; let LVar5: LRESULT; let u_var6: u16; let uVar7: u16; let uVar8: u16; let uVar9: u16; let lStack14: i32; let puStack10: u32; let lStack6: i32;

uVar9 = SUB42(ctx.data_seg, 0x0); uVar8 = param_3._2_2_; HVar2 = GetProp16(param_5, s_procHi_1050_5bd7);
uVar7 = param_3._2_2_; HVar3 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_procLo_1050_5bd0); lStack6 = CONCAT22(HVar2, HVar3); u_var6 = param_3._2_2_; HVar2 = GetProp16(ctx.s_tile2_bmp_1050_1538,s_thisHi_1050_5be5); HVar3 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_thisLo_1050_5bde); puStack10 = CONCAT22(HVar2,HVar3);
// wparam = (param_2 >> 0x10); if ((HVar2 | HVar3) != 0x0) {
lStack14 = 0x0; if (param_3 == 0x19) {
ppcVar1 = ( * puStack10 + 0x34); lStack14 = ( * * ppcVar1)(ctx.s_tile2_bmp_1050_1538, HVar3, HVar2, param_1, param_2,
param_3._2_2_, u_var6, uVar7, uVar8, uVar9);
}
else {
if (param_3 == 0x86) {
ppcVar1 = ( * puStack10 + 0x20); u_var4 = ( * * ppcVar1)(ctx.s_tile2_bmp_1050_1538, HVar3, HVar2, wparam);
//         TODO: goto LAB_1038_d10e;
}
if ((param_3 == 0x112) & & ((wparam & 0xfff0) == 0xf140)) {
LVar5 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0,0x0, 0x112f140); u_var4 = (LVar5 == 0x0);
//         TODO: goto LAB_1038_d10e;
}
}
if (lStack14 != 0x0) {
return lStack14;
}
} if (lStack6 != 0x0) {
LVar5 = CallWindowProc16(s_tile2_bmp_1050_1538, param_1, param_2,wparam,
param_3); return LVar5;
}
u_var4 = 0x0;
//LAB_1038_d10e: return u_var4;
}



pub fn win_prop_op_1038_d118(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: HWND16)

{
    let ppcVar1: u32;
    let u_var2: u32;
    let cVar3: u8;
    let HVar4: HANDLE16;
    let HVar5: HANDLE16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let puStack6: u32;

    uVar8 = SUB42(ctx.data_seg, 0x0);
    uVar7 = param_3;
    HVar4 = GetProp16(param_5, s_thisHi_1050_5bf3);
    u_var6 = param_3;
    HVar5 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_thisLo_1050_5bec);
    puStack6 = CONCAT22(HVar4, HVar5);
    if (param_2._2_2_ == 0x30) {
        if (param_2 == 0x0) {
            return;
        }
        SetProp16(ctx.s_tile2_bmp_1050_1538, param_2, 0x5c06);
        return;
    }
    if (param_2 < 0x310000) {
        // cVar3 = (param_2 >> 0x10);
        if (cVar3 == '\x02') {
            if ((HVar4 | HVar5) != 0x0) {
                u_var2 = *puStack6;
                ppcVar1 = u_var2 + 0x6;
                (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, HVar5, HVar4, param_1, param_2, u_var6, uVar7,
                            uVar8);
                if (puStack6 != 0x0) {
                    ppcVar1 = u_var2;
                    (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, HVar5, HVar4, 0x1);
                }
            }
            HVar4 = GetProp16(ctx.s_tile2_bmp_1050_1538, 0x5bfa);
            if (HVar4 == 0x0) {
                return;
            }
            DeleteObject16(ctx.s_tile2_bmp_1050_1538);
            RemoveProp16(ctx.s_tile2_bmp_1050_1538, 0x5c00);
            return;
        }
        if (cVar3 == '\x06') {
            if ((param_2 != (&ctx.PTR_LOOP_1050_0000 + 0x1)) && (param_2 != &ctx.PTR_LOOP_1050_0002)) {
                u_var2 = &ctx.PTR_LOOP_1050_5bc8;
                (u_var2 + 0x8) = 0x0;
                return;
            }
            u_var2 = &ctx.PTR_LOOP_1050_5bc8;
            (u_var2 + 0x8) = param_3;
            return;
        }
    }
    if ((HVar4 | HVar5) != 0x0) {
        ppcVar1 = (*puStack6 + 0xc);
        (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, HVar5, HVar4, param_1, param_2);
    }
    return;
}


pub fn get_sys_metrics_1040_7728(param_1: &mut Struct57, param_2: u16, param_3: u32, param_4: u16, param_5: u16)

{
    let IVar1: i16;
    let i_var2: &mut Struct57;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    param_1 = 0x389a;
    i_var2.field_0x2 = 0x1008;
    param_1 = 0x3aa8;
    i_var2.field_0x2 = 0x1008;
    i_var2.field_0x4 = 0x0;
    i_var2.field_0x6 = 0x0;
    i_var2.field_0x8 = param_5;
    i_var2.field_0xa = param_4;
    i_var2.field_0xc = 0x0;
    i_var2.field_0x60 = 0x0;
    i_var2.field_0x62 = 0x0;
    i_var2.field_0x64 = 0x0;
    i_var2.field_0x66 = 0x0;
    i_var2.field_0x68 = 0x0;
    i_var2.field_0x6a = param_3;
    i_var2.field_0x6e = param_2;
    i_var2.field_0x70 = 0x0;
    i_var2.field_0x74 = 0x0;
    i_var2.field_0x76 = 0x0;
    i_var2.field_0x78 = 0x0;
    i_var2.field_0x8a = 0x0;
    i_var2.field_0x8c = 0x0;
    param_1 = 0x840c;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    string_1000_3d3e((param_1 & 0xffff0000 | &i_var2.field_0x10),
                     0x10505db0);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var2.field_0x7a),
        0x0, 0x8);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var2.field_0x82),
        0x0, 0x8);
    IVar1 = GetSystemMetrics16(0x1000);
    i_var2.field_0x62 = IVar1;
    IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var2.field_0x64 = IVar1;
    IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    i_var2.field_0x66 = IVar1;
    return;
}


pub fn get_sys_metrics_1040_8c66(param_1: &mut Struct37, param_2: HWND16) {
    let pi_var1: U32Ptr;
    let bVar2: u8;
    let hdc: HDC16;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    hdc = GetDC16(param_2);
    draw_text_1040_8d14(ctx, param_1, s_tile2_bmp_1050_1538);
    (i_var4 + 0xa6) = (i_var4 + 0x9e);
    (i_var4 + 0xaa) = (i_var4 + 0xa2);
    i_var3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    pi_var1 = (i_var4 + 0xac);
    *pi_var1 = *pi_var1 + i_var3;
    bVar2 = (i_var4 + 0x98) & 0xf0;
    if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
        i_var3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
        if ((i_var4 + 0xac) < i_var3) {
            i_var3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
            (i_var4 + 0xac) = i_var3;
        }
    }
    pi_var1 = (i_var4 + 0xaa);
    *pi_var1 = *pi_var1 + 0x14;
    pi_var1 = (i_var4 + 0xac);
    *pi_var1 = *pi_var1 + 0xa;
    (i_var4 + 0xb0) = (i_var4 + 0xac);
    pi_var1 = (i_var4 + 0xac);
    *pi_var1 = *pi_var1 + 0x30;
    ReleaseDC16(ctx.s_tile2_bmp_1050_1538, hdc);
    return;
}


pub fn reg_class_1040_98c0(
    ctx: &mut AppContext,
    param_1: &mut Struct_1040_98c0,
    inst_handle: HINSTANCE16,
    wnd_class: &WNDCLASS16,
    stack0xfffe: u16
) {
    let b_result: bool;
    let AVar2: ATOM;
    let mut class_name = String::new();
    let uStack26: u16;
    let uStack24: u32;
    let uStack22: u32;
    let puStack18: U32Ptr;
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u32;
    let i_stack6: i16;
    let uStack4: u16;

    i_stack6 = param_1 + 0x4;
    b_result = GetClassInfo16(inst_handle, &class_name, wnd_class);
    if b_result == false {
        class_name = (param_1 + 0x54);
        uStack26 = 0x9cde;
        uStack24 = ctx.PTR_LOOP_1050_1040;
        uStack22 = 0x40000;
        puStack18 = ctx.PTR_LOOP_1050_038c;
        uStack16 = 0x0;
        uStack14 = (param_1 + 0x58);
        uStack12 = (param_1 + 0x56);
        uStack10 = 0x0;
        uStack4 = param_1._2_2_;
        AVar2 = RegisterClass16(&ctx.s_tile2_bmp_1050_1538 as &WNDCLASS16);
        if AVar2 == 0x0 {
            fn_ptr_op_1000_24cd(ctx, 0x0, stack0xfffe);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn make_proc_inst_1040_a234(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: u32, param_5: U32Ptr)

{
    pvVar1: U32Ptr;
    let in_DX: u16;

    pass1_1040_b040(CONCAT22(param_2, param_1), CONCAT22(param_4, param_3),
                    (param_4 >> 0x10));
    CONCAT22(param_2, param_1) = 0xa4e8;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    if (ctx.PTR_LOOP_1050_5edc == 0x0) {
        pvVar1 = MakeProcInstance16(param_5, PTR_LOOP_1050_038c);
        ctx._PTR_LOOP_1050_5edc = CONCAT22(in_DX, pvVar1);
    }
    (param_1 + 0xc) = ctx._PTR_LOOP_1050_5edc;
    ctx.PTR_LOOP_1050_5eda = ctx.PTR_LOOP_1050_5eda + 0x1;
    ctx.PTR_LOOP_1050_5ee0 = param_1;
    ctx.PTR_LOOP_1050_5ee2 = param_2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn free_proc_inst_1040_a294(param_1: &mut Struct18, param_2: u16) {
    param_1.field_0x0 = 0xa4e8;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    ctx.PTR_LOOP_1050_5eda = ctx.PTR_LOOP_1050_5eda + -0x1;
    if (ctx.PTR_LOOP_1050_5eda == 0x0) {
        FreeProcInstance16(param_2);
        ctx._PTR_LOOP_1050_5edc = 0x0;
    }
    unk_draw_op_1040_b0f8(param_1);
    return;
}


pub fn call_win_proc_1040_a40e(param_1: HWND16, param_2: u32, param_3: LPARAM, param_4: u16, param_5: *mut u8,
                               param_6: u16) -> u32

{
    let u_var1: u16;
    let ppcVar2: u32;
    let puVar4: u32;
    wparam: WPARAM16;
    let iVar5: i16;
    let unaff_DI: i16;
    let u_var6: u16;
    let uVar7: u32;
    let uStack6: u32;
    let pu_var3: u32;
    let u_var5: u32;

    uStack6 = 0x0;
// wparam = (param_2 >> 0x10); if (param_3 == 0x19) {
    puVar4 = &ctx.PTR_LOOP_1050_5ee0;
    ppcVar2 = (*puVar4 + 0x34);
    uStack6 = (**ppcVar2)(param_5, puVar4, (puVar4 > > 0x10), param_1,
                          param_2, ctx.data_seg);
// param_4 = (uStack6 >> 0x10);
}
else {
if (param_3 == 0x86) {
puVar4 = & ctx.PTR_LOOP_1050_5ee0; ppcVar2 = (* puVar4 + 0x20); uVar7 = ( ** ppcVar2)(param_5, puVar4, (puVar4 > > 0x10), wparam); return uVar7;
}
if (param_3 == 0x110) {
uVar7 = win_msg_1040_a308( & ctx.PTR_LOOP_1050_5ee0, unaff_DI, param_5,
param_6); return uVar7;
}
} if (uStack6 != 0x0) {
return uStack6 & 0xffff | param_4 < < 0x10;
}
u_var5 = & ctx.PTR_LOOP_1050_5bc8;
// u_var6 = (u_var5 >> 0x10);
iVar5 = u_var5; u_var1 = (iVar5 + 0x6); if ((u_var1 | (iVar5 + 0x4)) == 0x0) {
return u_var1 < < 0x10;
}
uVar7 = CallWindowProc16(param_5, param_1, param_2,wparam, param_3); return uVar7;
}


unsafe fn def_win_proc_1008_5632(
    ctx: &mut AppContext,
    param_1: *mut i32,
    param_2: WPARAM16,
    param_3: u16,
    param_4: i16,
    param_5: u16,
    unaff_cs: HWND16,
    unaff_ss: u16,
)

{
    let ppcVar1: u32;
    let u_var2: u16;
    let pu_stack6: libc::c_long;

    u_var2 = SUB42(ctx.data_seg, 0x0) as u16;
    pu_stack6 = GetWindowLong16(unaff_cs, 0x0);
    if ((pu_stack6 >> 0x10) | pu_stack6) == 0x0 {
        if param_4 != 0x1 {
            DefWindowProc16(ctx.s_tile2_bmp_1050_1538, param_1, param_2,
                            CONCAT22(param_4 as u16, param_3));
            return;
        }
        pu_stack6 = *param_1;
        SetWindowLong16(ctx.s_tile2_bmp_1050_1538, pu_stack6, pu_stack6 >> 0x10);
        pass1_1008_9628(pu_stack6, param_5);
    }
    ppcVar1 = (*pu_stack6 + 0x1c);
    (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, pu_stack6, (pu_stack6 >> 0x10),
                param_1, param_2, param_3, param_4, u_var2);
    return;
}

pub fn __exported_stub() -> u16

{
    return 0x0;
}

