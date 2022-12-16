use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000;
use crate::block_1000::block_1000_1000::mem_1000_167a;
use crate::block_1000::block_1000_5000::ret_op_1000_55ac;
use crate::dos_interrupt::swi;
use crate::globals::{DAT_1050_1050, HINSTANCE16_1050_5f4c, PTR_LOOP_1050_000a, PTR_LOOP_1050_000c, PTR_LOOP_1050_1000, PTR_LOOP_1050_5f7e, PTR_LOOP_1050_5fd2, PTR_LOOP_1050_5fd4, PTR_LOOP_1050_6066, PTR_LOOP_1050_63fe, u8_1050_5fc9};
use crate::structs::struct_825::Struct825;
use crate::structs::struct_d::StructD;
use crate::sys_ops::___EXPORTEDSTUB;
use crate::utils::{CONCAT22, SUB42};
use crate::winbase::{FatalAppExit16, FatalExit, GetModuleFileName16};

pub unsafe fn pass1_1000_201c(mut param_1: i16, mut param_2: i16) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut BVar4: bool;
    let mut i_var5: i16;
    let mut u_var6: u16;

    if (param_1 == 0) {
        (param_2 + 0x6) = 0;
        (param_2 + 0x4) = 0;
    }
    u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    while (u_var3 != 0) {
        BVar4 = pass1_1000_206c((param_2 + 0x4), (param_2 + 0x6));
        if (BVar4 == 0) {
            u_var2 = (param_2 + 0x4);
            u_var6 = (u_var2 >> 0x10);
            i_var5 = u_var2;
            u_var1 = (i_var5 + 0x2c);
            (param_2 + 0x4) = (i_var5 + 0x2a);
            (param_2 + 0x6) = u_var1;
        } else {
            mem_op_1000_1b9a(0x1, (param_2 + 0x4), (param_2 + 0x6));
        }
        u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}

pub unsafe fn pass1_1000_20a2(mut param_1: u16, mut param_2: u16) {
    let mut i_var1: i16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_stack8: u16;
    let mut u_stack4: u16;

    i_var1 = (param_1 + 0x2e);
    u_var2 = (param_1 + 0x30);
    u_stack8 = 0;
    u_var3 = (i_var1 + 0x4);
    u_stack4 = (i_var1 + 0x6);
    u_var7 = 0;
    if ((u_stack4 | u_var3) != 0) {
        while ((
            u_var6 = u_var3,
            u_var4 = u_stack4,
            u_var6 != param_1 || (u_stack4 != param_2),
        )) {
            u_var3 = (u_var6 + 0x2a);
            u_stack4 = (u_var6 + 0x2c);
            u_var7 = u_var6;
            u_stack8 = u_var4;
            if ((u_stack4 | u_var3) == 0) {
                return;
            }
        }
        if ((u_stack8 | u_var7) != 0) {
            u_var2 = (u_var6 + 0x2c);
            (u_var7 + 0x2a) = (u_var6 + 0x2a);
            (u_var7 + 0x2c) = u_var2;
            return;
        }
        u_var5 = (u_var6 + 0x2c);
        (i_var1 + 0x4) = (u_var6 + 0x2a);
        (i_var1 + 0x6) = u_var5;
    }
}

pub unsafe fn empty_fn_1000_214a() {
    return;
}

// WARNING: Removing unreachable block (ram,0x1000234c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn init_1000_23be(mut param_1: u16, mut param_2: u16) {
    init_op_1008_54aa(
        &DAT_1050_1050,
        param_1,
        param_2,
        PTR_LOOP_1050_5f52,
        CONCAT22(PTR_LOOP_1050_5f50, PTR_LOOP_1050_5f4e),
        PTR_LOOP_1050_5f4a,
        HINSTANCE16_1050_5f4c,
    );
    return;
}

pub unsafe fn fn_ptr_op_1000_2594() {
    let mut ppcVar1: *mut *mut code;
    let mut unaff_SI: *mut *mut code;
    let mut unaff_DI: *mut *mut code;
    let mut ppcVar2: *mut *mut code;
    let mut fn_ptr_1: *mut *mut code;

    while (unaff_SI < unaff_DI) {
        ppcVar2 = unaff_DI -0x2;
        ppcVar1 = unaff_DI -0x1;
        unaff_DI = ppcVar2;
        if ((*ppcVar2 | *ppcVar1) != 0) {
            fn_ptr_1 = ppcVar2;
            (**fn_ptr_1)();
        }
    }
    return;
}

//
pub unsafe fn pass1_1000_25a8() {
    pass1_1000_2913(0xfc);
    pass1_1000_2913(0xff);
    return;
}

pub unsafe fn pass1_1000_27d6(mut param_1: u16) {
    let mut piVar2: *mut i16;
    let mut pcVar3: *mut c_char;
    let mut cVar4: u8;
    let mut pu_var5: *mut u16;
    u16 * *ppuVar6;
    let mut i_var7: i16;
    let mut u_var7: u16;
    let mut i_var8: i16;
    let mut pu_var8: *mut u16;
    let mut i_var9: i16;
    let mut piVar9: *mut i16;
    let mut piVar10: *mut i16;
    let mut pcVar11: *mut c_char;
    let mut piVar12: *mut i16;
    let mut b_var13: bool;
    let mut pu_var14: *mut u16;
    let mut piVar1: *mut i16;
    let mut pu_var4: *mut u16;
    let mut piVar4: *mut i16;

    let dos_env = GetDOSEnvironment16();
    let pu_var7 = (dos_env >> 0x10);
    if (dos_env != 0) {
        pu_var7 = null_mut();
    }
    i_var9 = 0;
    pcVar11 = null_mut();
    i_var7 = -0x1;
    if (pu_var7.is_null() == false) {
        cVar4 = *NULL;
        while (cVar4 != '\0') {
            loop {
                if (i_var7 == 0) {
                    break;
                }
                i_var7 += -0x1;
                pcVar3 = pcVar11;
                pcVar11 = pcVar11 + 1;
                if *pcVar3 == '\0' {
                    break;
                }
            }
            i_var9 += 0x1;
            pcVar3 = pcVar11;
            pcVar11 = pcVar11 + 1;
            cVar4 = *pcVar3;
        }
    }
    u_var7 = 0x9;
    pu_var8 = pu_var7;
    pu_var5 = exit_op_1000_2950(ctx, 0x9, pu_var7, (pcVar11 + 1) & 0xfffe);
    pu_var14 = pu_var8;
    ppuVar6 = exit_op_1000_2950(ctx, u_var7, pu_var8, (i_var9 + 1) * 0x4);
    piVar9 = null_mut();
    PTR_LOOP_1050_5fbe = ppuVar6;
    PTR_LOOP_1050_5fc0 = pu_var8;
    loop {
        if (i_var9 == 0) {
            *ppuVar6 = null_mut();
            ppuVar6[0x1] = null_mut();
            return;
        }
        b_var13 = *piVar9 == s__C_FILE_INFO__1050_5f5c;
        if (b_var13) {
            piVar12 = s__C_FILE_INFO__1050_5f5c;
            i_var8 = 0x6;
            piVar10 = piVar9;
            loop {
                if (i_var8 == 0) {
                    break;
                }
                i_var8 += -0x1;
                piVar4 = piVar12;
                piVar12 = piVar12 + 1;
                piVar1 = piVar10;
                piVar10 = piVar10 + 1;
                b_var13 = *piVar1 == *piVar4;
                if b_var13 == false {
                    break;
                }
            }
            if (!b_var13) {
                // TODO: goto LAB_1000_2867;
            }
        } else {
            //
            //            LAB_1000_2867:
            *ppuVar6 = pu_var5;
            ppuVar6[0x1] = pu_var14;
            ppuVar6 = ppuVar6 + 2;
        }
        loop {
            piVar2 = piVar9;
            piVar9 = (piVar9 + 1);
            cVar4 = piVar2;
            pu_var4 = pu_var5;
            pu_var5 = (pu_var5 + 1);
            pu_var4 = cVar4;
            if cVar4 == '\0' {
                break;
            }
        }
        i_var9 += -0x1;
    }
}

pub unsafe fn pass1_1000_2913(mut param_1: u16) {
    let mut pcVar1: *mut c_char;
    let mut pcVar2: *mut c_char;
    let mut i_var3: i16;
    let mut unaff_di: u16;
    let mut unaff_es: u16;
    let mut paVar4: *mut Struct825;
    let mut i_var5: i16;

    i_var5 = &DAT_1050_1050;
    if (u16_1050_61ec != 0) {
        paVar4 = CONCAT22(unaff_di, param_1);
        pcVar2 = poss_str_op_1000_28dc(paVar4);
        if (pcVar2.is_null() == false) {
            i_var3 = -0x1;
            loop {
                if (i_var3 == 0) {
                    break;
                }
                i_var3 += -0x1;
                pcVar1 = pcVar2;
                pcVar2 = pcVar2 + 1;
                if *pcVar1 == '\0' {
                    break;
                }
            }
            pass1_1000_55b1((paVar4 >> 0x10), i_var5);
        }
    }
    return;
}

pub unsafe fn pass1_1000_29af(mut param_1: u16) {
    pass1_1000_29b5(param_1 & 0xff);
    return;
}

pub unsafe fn pass1_1000_29b5(mut param_1: u16) {
    let mut cVar1: u8;

    PTR_LOOP_1050_5f88._0_1_ = param_1;
    cVar1 = (param_1 >> 0x8);
    if (cVar1 != '\0') {
        // TODO: goto LAB_1000_29d2;
    }
    if (PTR_LOOP_1050_5f88 < 0x22) {
        if (PTR_LOOP_1050_5f88 < 0x20) {
            if (0x13 < PTR_LOOP_1050_5f88) {
                // TODO: goto LAB_1000_29cc;
            }
        } else {
            param_1 = 0x5;
        }
    } else {
        //
        //        LAB_1000_29cc:
        param_1 = 0x13;
    }
    cVar1 = ((param_1 & 0xff) + 0x5fd6); //
                                         //    LAB_1000_29d2:
    PTR_LOOP_1050_5f78 = cVar1;
    return;
}

pub unsafe fn pass1_1000_2b3c(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: i16,
) {
    pass1_1000_2b02(param_1, param_2, param_3, param_4, param_5, 0x0);
    return;
}

pub unsafe fn pass1_1000_2ba0(param_1: u8) {
    pass1_1000_3024();
    if (u8_1050_5fc9 != '\0') {
        pass1_1000_3f5c();
    }
    return;
}

pub unsafe fn pass1_1000_2cb0(param_1: *mut u16) {
    let mut pu_var1: *mut u16;
    let mut bVar2: u8;

    bVar2 = *(param_1 + 0x5);
    if (((bVar2 & 0x83) != 0) && ((bVar2 & 0x8) != 0)) {
        pass1_1000_16ee(param_1[0x3], param_1[0x4]);
        pu_var1 = param_1 + 0x5;
        *pu_var1 = *pu_var1 & 0xf7;
        param_1[0x3] = 0;
        param_1[0x4] = 0;
        *param_1 = 0;
        param_1[0x1] = 0;
        param_1[0x2] = 0;
    }
    return;
}

pub unsafe fn mem_1000_2ce8(mut param_1: u16, param_2: *mut i16) {
    let mut piVar1: *mut i16;
    let mut u_var2: u16;

    u_var2 = mem_1000_167a(param_1, 0x200);
    if (param_1 == 0) {
        piVar1 = param_2 + 0x5;
        *piVar1 = *piVar1 | 0x4;
        param_2[0x79] = 0x1;
        param_1 = &DAT_1050_1050;
        u_var2 = param_2 + 0xf1;
    } else {
        piVar1 = param_2 + 0x5;
        *piVar1 = *piVar1 | 0x8;
        param_2[0x79] = 0x200;
    }
    param_2[0x1] = param_1;
    *param_2 = u_var2;
    param_2[0x4] = param_1;
    param_2[0x3] = u_var2;
    param_2[0x2] = 0;
    return;
}

pub unsafe fn pass1_1000_2f00(mut param_1: i16, param_2: *mut i16) {
    if (((*(param_2 + 0x78) & 0x10) != 0) && ((*(*(param_2 + 0xb) + 0x5f90) & 0x40) != 0)) {
        pass1_1000_2fa4(param_2);
        if (param_1 != 0) {
            *(param_2 + 0x78) = 0;
            param_2[0x79] = 0;
            *param_2 = 0;
            param_2[0x1] = 0;
            param_2[0x3] = 0;
            param_2[0x4] = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1000_206c(mut param_1: u16, mut param_2: u16) -> bool {
    let mut uVar1: u16;

    uVar1 = pass1_1000_21d2(0x2, 0x42, param_1, param_2, 1);
    if ((uVar1 != 0) && ((param_1 + 0x14) == -0x4153)) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn ret_true_1000_2146() -> u16 {
    return 0x1;
}

pub unsafe fn msg_box_op_1000_214c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
) -> bool {
    let mut IVar1: i16;
    let mut iVar2: i16;
    let mut msg_type: u16;

    msg_type = 0x2 - (param_2 == 0) | 0x2110;
    MessageBeep16(0x0);
    loop {
        IVar1 = MessageBox16(
            msg_type,
            "SmartHeap Library",
            CONCAT22(param_4, param_3),
            0x0,
        );
        iVar2 = IVar1 -0x1;
        if (iVar2 == 0) {
            return 0x0;
        }
        if ((0x0 < iVar2) && (!SBORROW2(iVar2, 1))) {
            if (IVar1 == 0x3 || IVar1 -0x2 < 1) {
                fatal_app_exit_1000_3e9e();
                return 0x0;
            }
            if (IVar1 == 0x4) {
                return 0x1;
            }
            if (IVar1 == 0x5) {
                return 0x0;
            }
        }
        if ((msg_type & 0x2000) == 0) {
            return 0x0;
        }
        msg_type = msg_type & 0xdfef | 0x1010;
    }
}

pub unsafe fn mem_op_1000_21b6(mut param_1: u16, mut param_2: u16) -> bool {
    let mut BVar1: bool;

    BVar1 = mem_op_1000_1dfa(0x0, 0x4, param_1, param_2);
    return BVar1 == 0;
}

pub unsafe fn pass1_1000_21d2(
    param_1: u8,
    param_2: i32,
    mut param_3: u16,
    mut param_4: u16,
    param_5: u8,
) -> u16 {
    let mut uVar1: u32;
    let mut BVar2: bool;

    BVar2 = mem_op_1000_1dfa(0x0, param_1, param_3, param_4);
    if (BVar2 == 0) {
        if ((param_1 & 0x4) == 0) {
            uVar1 = SegmentLimit(param_4);
            if ((uVar1 >> 0x10) & 1) {
                if (param_2 == 0) {
                    return 0x1;
                }
                if ((!CARRY4(param_3, param_2 - 1)) && (param_3 + (param_2 - 1) <= uVar1)) {
                    return 0x1;
                }
            }
        } else {
            BVar2 = pass1_1000_22c0(param_2, _param_1, param_2, param_3, param_4);
            if (BVar2 != 0) {
                return 0x1;
            }
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1000_2242(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: i16,
) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut bVar3: bool;

    uVar1 = param_4 | param_3;
    loop {
        if (uVar1 == 0) {
            return 0x0;
        }
        uVar1 = param_3;
        if (param_4 != 0) {
            uVar1 = 0xffff;
        }
        if (CARRY2(param_5, uVar1) != false) {
            uVar1 = -param_5;
        }
        bVar3 = param_3 < uVar1;
        param_3 -= uVar1;
        param_4 -= bVar3;
        uVar2 = (param_2)(uVar1, param_1, param_5, param_6);
        if (uVar2 != 0) {
            break;
        }
        bVar3 = CARRY2(param_5, uVar1);
        param_5 += uVar1;
        param_6 += bVar3 * 0x100;
        uVar1 = param_4 | param_3;
    }
    return CONCAT22(param_4 + CARRY2(uVar2, param_3), uVar2 + param_3);
}

pub unsafe fn pass1_1000_22c0(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) -> bool {
    let mut u32_var1: u32;

    u32_var1 = pass1_1000_2242(param_2, 0x1dfa, param_1, param_3, param_4, param_5);
    if u32_var1 == 0 {
        return true;
    }
    return false;
}

pub unsafe fn pass1_1000_25d2(
    mut param_1: i16,
    mut param_2: i16,
    fn_ptr_param_3: code2,
    mut param_4: i16,
) -> u16 {
    let mut var1: *mut i16;
    let mut var2: *mut c_char;
    // let mut pstruct_d_var4: *mut StructD;
    let mut var8: *mut i16;
    let mut string9: *mut c_char;
    let mut var3 = (param_1 + 0x1 & 0xfffe);

    let mut var5 = 0u16;
    let mut pstruct_d_var4 = -(var3 - &param_2);
    if (var3 < &param_2) && (PTR_LOOP_1050_000a <= pstruct_d_var4.address_offset_field_0x0)
    {
        if pstruct_d_var4.address_offset_field_0x0 < PTR_LOOP_1050_000c {
            PTR_LOOP_1050_000c = pstruct_d_var4.address_offset_field_0x0;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
        // WARNING: Treating indirect jump as call
        var5 = fn_ptr_param_3();
        return var5;
    }
    let mut paVar10 = (param_2 << 0x10);
    let mut offset7 = 0;
    pass1_1000_25a8();
    pass1_1000_2913(offset7);
    let mut string6 = poss_str_op_1000_28dc(paVar10);
    if string6.is_null() == false {
        offset7 = 0x9;
        if *string6 == 'M' as i8 {
            offset7 = 0xf;
        }
        string6 = string6 + offset7;
        offset7 = 0x22;
        string9 = string6;
        loop {
            if offset7 == 0 {
                break;
            }
            offset7 += -0x1;
            var2 = string9;
            string9 = string9 + 1;
            if *var2 == '\r' as c_char {
                break;
            }
        }
        string9[-0x1] = '\0';
    }
    // FatalAppExit16( 0x0, CONCAT22(0x1050, pcVar6));
    FatalAppExit16(0x0, string6);
    FatalExit();
    var5 = PTR_LOOP_1050_63fe;
    loop {
        var1 = var5;
        var5 = var5 + 1;
        offset7 = *var1;
        var8 = var5;
        if (offset7 == param_4) || (var8 = (offset7 + 1), var8.is_null()) {
            return var8;
        }
        offset7 = -0x1;
        loop {
            if (offset7 == 0) {
                break;
            }
            offset7 += -0x1;
            var1 = var5;
            var5 = (var5 + 1);
            if var1 == '\0' {
                break;
            }
        }
    }
}

pub unsafe fn poss_str_op_1000_28dc(param_1: *mut Struct825) -> *mut c_char {
    // let mut ppaVar1: *mut *mut struct_825;
    let mut ppaVar1: *mut *mut Struct825;
    char * piVar2;
    let mut iVar3: i16;
    char * string_var3;
    let mut iVar2: *mut Struct825;

    string_var3 = PTR_LOOP_1050_63fe;
    loop {
        ppaVar1 = string_var3;
        string_var3 = (string_var3 + 2);
        iVar2 = *ppaVar1;
        piVar2 = string_var3;
        if ((iVar2 == param_1) || (piVar2 = (iVar2 + 1), piVar2.is_null())) {
            return piVar2;
        }
        iVar3 = -0x1;
        loop {
            if (iVar3 == 0) {
                break;
            }
            iVar3 += -0x1;
            ppaVar1 = string_var3;
            string_var3 = (string_var3 + 1);
            if ppaVar1 == '\0' {
                break;
            }
        }
    }
}

pub unsafe fn exit_op_1000_2950(ctx: &mut AppContext, mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    let mut var2: u16;
    let mut var3: *mut c_char;
    let mut var6: i16;

    let mut var9: *mut c_char;

    // let mut paVar10: *mut Struct825;

    let mut var4 = PTR_LOOP_1050_6066;
    PTR_LOOP_1050_6066 = PTR_LOOP_1050_1000;
    let mut var8 = mem_1000_167a(param_2, param_3);
    PTR_LOOP_1050_6066 = var4;
    if (param_2 | var8) != 0 {
        return var8;
    }
    let mut var10 = CONCAT22(ctx.ES_REG, param_1);
    pass1_1000_25a8();
    pass1_1000_2913(param_1);
    let mut var5 = poss_str_op_1000_28dc(var10);
    if var5.is_null() == false {
        var6 = 0x9;
        if *var5 == 'M' as i8 {
            var6 = 0xf;
        }
        var5 = var5 + var6;
        var6 = 0x22;
        var9 = var5;
        loop {
            if var6 == 0 {
                break;
            }
            var6 += -0x1;
            var3 = var9;
            var9 = var9 + 1;
            if *var3 == '\r' as i8 {
                break;
            }
        }
        var9[-0x1] = '\0';
    }
    FatalAppExit16(0, var5);
    FatalExit();
    var8 = PTR_LOOP_1050_63fe;
    loop {
        let mut var1 = var8;
        var8 = var8 + 1;
        var2 = var1;
        let mut var7 = var8;
        var7 = var2 + 1;
        if var2 == ctx.BP_REG || var7.is_null() {
            return var7;
        }
        var6 = -0x1;
        loop {
            if var6 == 0 {
                break;
            }
            var6 += -0x1;
            var1 = var8;
            var8 = (var8 + 1);
            if var1 == '\0' {
                break;
            }
        }
    }
}

pub unsafe fn pass1_1000_29dc(mut param_1: u16) -> u16 {
    if ___EXPORTEDSTUB != 0xb8 {
        return DAT_1050_1050;
    }
    return uRam100029ed;
}

pub unsafe fn pass1_1000_2a00(param_1: *mut u16) -> u16 {
    let mut uVar1: u16;
    let mut bVar2: bool;
    let mut piVar3: *mut i16;
    let mut uVar4: u16;
    let mut unaff_BP: i16;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut puStack20: *mut u8;
    let mut local_10: u8;
    let mut uStack15: u8;
    // let mut local_e: [u8;0x8] = [0;0x8];
    let mut local_e: [u8; 8] = [0; 8];
    let mut uStack6: u16;
    let mut local_4: u16;
    let mut iStack2: i16;

    iStack2 = unaff_BP + 1;
    local_4 = SUB42(DAT_1050_1050, 0x0);
    uVar5 = 0xffff;
    if ((*(param_1 + 0x5) & 0x40) != 0) {
        *(param_1 + 0x5) = 0;
        return 0xffff;
    }
    if ((*(param_1 + 0x5) & 0x83) == 0) {
        // TODO: goto LAB_1000_2af2;
    }
    uVar5 = pass1_1000_2fa4(param_1);
    uStack6 = param_1[0x7a];
    pass1_1000_2cb0(param_1);
    uVar1 = *(param_1 + 0xb);
    if (u16_1050_5f8a < uVar1) {
        piVar3 = pass1_1000_55b1(unaff_CS, uVar1);
        if (piVar3 < 0x0) {
            // TODO: goto LAB_1000_2a6a;
        } //
          //        LAB_1000_2a82:
        bVar2 = false;
    } else {
        uVar4 = dos3_call_op_1000_35fe(*(param_1 + 0xb), &iStack2);
        if (-0x1 < uVar4) {
            // TODO: goto LAB_1000_2a82;
        } //
          //        LAB_1000_2a6a:
        bVar2 = true;
    }
    if (!bVar2) {
        if (uStack6 == 0) {
            // TODO: goto LAB_1000_2af2;
        }
        unk_str_op_1000_3d3e(CONCAT22(0x1050, &local_10), s___1050_5fea);
        puStack20 = local_e;
        if (local_10 == '\\') {
            puStack20 = &uStack15;
        } else {
            pass1_1000_3cea(CONCAT22(0x1050, &local_10), s___1050_5fec);
        }
        pass1_1000_3e82(uStack6, CONCAT22(0x1050, puStack20), 0xa);
        uVar4 = dos3_call_1000_514e();
        if (uVar4 == 0) {
            // TODO: goto LAB_1000_2af2;
        }
    }
    uVar5 = 0xffff; //
                    //    LAB_1000_2af2:
    *(param_1 + 0x5) = 0;
    return uVar5;
}

pub unsafe fn pass1_1000_2b02(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    param_6: u8,
) -> *mut u16 {
    let mut puVar1: *mut u16;

    puVar1 = pass1_1000_35aa();
    if ((param_1 | puVar1) == 0) {
        puVar1 = null_mut();
    } else {
        puVar1 = pass1_1000_2d34(
            param_2,
            param_3,
            CONCAT22(param_5, param_4),
            param_6,
            puVar1,
        );
    }
    return puVar1;
}

pub unsafe fn pass1_1000_2b5c(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = pass1_1000_2e74(param_1);
    uVar2 = FUN_1000_30b4();
    pass1_1000_2f00(uVar1, param_1);
    return uVar2;
}

pub unsafe fn mem_1000_2bb6(mut param_1: u16, mut param_2: u16, param_3: *mut i16) -> u16 {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut piVar3: *mut i16;
    let mut bVar4: u8;
    let mut puVar5: *mut u8;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;

    piVar3 = param_3;
    bVar4 = *(param_3 + 0x5);
    if (((bVar4 & 0x82) != 0) && ((bVar4 & 0x40) == 0)) {
        param_3[0x2] = 0;
        if ((bVar4 & 1) != 0) {
            if ((bVar4 & 0x10) == 0) {
                // TODO: goto LAB_1000_2c37;
            }
            *param_3 = param_3[0x3];
            bVar4 &= 0xfe;
        }
        *(param_3 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7 = *(param_3 + 0xb);
        if (((bVar4 & 0x8) == 0)
            && ((bVar4 & 0x4) != 0x0
                || ((*(param_3 + 0x78) & 1) == 0x0
                    && ((u16_1050_61ec != 0x0
                        && ((param_3 == 0x621c || (param_3 == 0x6228))
                            && ((puVar7[0x5f90] & 0x40) != 0)))
                        || (
                            mem_1000_2ce8(param_1, param_3),
                            (*(piVar3 + 0x5) & 0x8) == 0,
                        )))))
        {
            puVar5 = mixed_dos3_call_1000_39f2(
                puVar7,
                CONCAT22(0x1050, &param_2),
                (&PTR_LOOP_1050_0000 + 1),
            );
            puVar6 = (&PTR_LOOP_1050_0000 + 1);
        } else {
            iVar2 = piVar3[0x3];
            puVar6 = (*piVar3 - iVar2);
            *piVar3 = iVar2 + 1;
            piVar3[0x2] = piVar3[0x79] -0x1;
            if (puVar6.is_null()) {
                puVar5 = null_mut();
                if ((puVar7[0x5f90] & 0x20) != 0) {
                    mixed_dos3_call_1000_3636(puVar7, 0x0, 0x0, 0x2);
                    puVar5 = null_mut();
                    puVar6 = puVar5;
                }
            } else {
                puVar5 =
                    mixed_dos3_call_1000_39f2(puVar7, CONCAT22(piVar3[0x4], piVar3[0x3]), puVar6);
            }
            *(piVar3 + 0x3) = param_2;
        }
        if (puVar5 == puVar6) {
            return param_2 & 0xff;
        }
    } //
      //    LAB_1000_2c37:
    piVar1 = piVar3 + 0x5;
    *piVar1 = *piVar1 | 0x20;
    return 0xffff;
}

pub unsafe fn pass1_1000_2d34(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut u8,
    param_4: u8,
    param_5: *mut u16,
) -> *const u16 {
    let mut bVar1: u8;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut uVar4: u16;
    let mut in_stack_0000ffd8: u16;
    let mut uStack14: u8;
    let mut bStack8: u8;
    let mut uStack6: u8;

    bStack8 = PTR_LOOP_1050_6062;
    bVar3 = false;
    bVar1 = *param_3;
    if (bVar1 == 0x77) {
        uVar4 = 0x301;
    } else {
        if (0x77 < bVar1) {
            return NULL;
        }
        if (bVar1 != 0x61) {
            if (bVar1 != 0x72) {
                return NULL;
            }
            uVar4 = 0;
            uStack6 = 0x1;
            // TODO: goto LAB_1000_2d6c;
        }
        uVar4 = 0x109;
    }
    uStack6 = 0x2; //
                   //    LAB_1000_2d6c:
    bVar2 = true; //
                  //    LAB_1000_2d71:
    param_3 = (param_3 & 0xffff0000 | (param_3 + 1));
    if ((*param_3 == 0) || (!bVar2)) {
        uVar4 =
            mixed_dos3_call_1000_370a(in_stack_0000ffd8, param_1, param_2, uVar4, param_4, 0x1a4);
        if (uVar4 < 0x0) {
            return NULL;
        }
        PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 1;
        *(param_5 + 0x5) = uStack6;
        param_5[0x1] = 0;
        *param_5 = 0;
        param_5[0x4] = 0;
        param_5[0x3] = 0;
        uStack14 = uVar4;
        *(param_5 + 0xb) = uStack14;
        *(param_5 + 0x78) = bStack8;
        param_5[0x2] = 0;
        param_5[0x7a] = 0;
        return param_5;
    }
    bVar1 = *param_3;
    if (bVar1 == 0x74) {
        if ((uVar4 & 0xc000) == 0) {
            uVar4 |= 0x4000;
            // TODO: goto LAB_1000_2d71;
        }
    } else {
        if (0x74 < bVar1) {
            // TODO: goto LAB_1000_2da4;
        }
        if (bVar1 == 0x2b) {
            if ((uVar4 & 0x2) != 0) {
                // TODO: goto LAB_1000_2da4;
            }
            uVar4 = uVar4 & 0xfffe | 0x2;
            uStack6 = 0x80;
            // TODO: goto LAB_1000_2d71;
        }
        if (bVar1 == 0x62) {
            if ((uVar4 & 0xc000) == 0) {
                uVar4 |= 0x8000;
                // TODO: goto LAB_1000_2d71;
            }
        } else {
            if (bVar1 != 0x63) {
                if ((bVar1 != 0x6e) || (bVar3)) {
                    // TODO: goto LAB_1000_2da4;
                }
                bVar3 = true;
                bStack8 &= 0xbf;
                // TODO: goto LAB_1000_2d71;
            }
            if (!bVar3) {
                bVar3 = true;
                bStack8 |= 0x40;
                // TODO: goto LAB_1000_2d71;
            }
        }
    } //
      //    LAB_1000_2da4:
    bVar2 = false;
    // TODO: goto LAB_1000_2d71;
}

pub unsafe fn pass1_1000_2e74(param_1: *mut u16) -> u16 {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u16;

    if (u16_1050_61ec != 0) {
        puVar5 = param_1 + 0x78;
        puVar4 = 0x5ff2;
        if ((param_1 == 0x621c) || (puVar4 = &PTR_LOOP_1050_5ff6, param_1 == 0x6228)) {
            if (((*(param_1 + 0x5) & 0xc) == 0) && ((*puVar5 & 1) == 0)) {
                uVar2 = *puVar4;
                uVar3 = puVar4[0x1];
                if ((uVar2 | uVar3) == 0) {
                    uVar2 = mem_1000_167a(uVar3, 0x200);
                    if (uVar3 == 0) {
                        return 0x0;
                    }
                    *puVar4 = uVar2;
                    puVar4[0x1] = uVar3;
                }
                param_1[0x3] = uVar2;
                param_1[0x4] = uVar3;
                *param_1 = uVar2;
                param_1[0x1] = uVar3;
                param_1[0x2] = 0x200;
                param_1[0x79] = 0x200;
                puVar1 = param_1 + 0x5;
                *puVar1 = *puVar1 | 0x2;
                *puVar5 = 0x11;
                return 0x1;
            }
        } else if (u16_1050_5f8a <= *(param_1 + 0xb)) {
            puVar1 = puVar5;
            *puVar1 = *puVar1 | 0x10;
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1000_2f48(param_1: i32) -> u16 {
    let mut uVar1: u16;
    let mut puVar2: *mut u8;

    if (param_1 == 0) {
        uVar1 = pass1_1000_3038(0x0);
    } else {
        uVar1 = pass1_1000_2fa4(param_1);
        if (uVar1 == 0) {
            if ((*(param_1 + 0x78) & 0x40) != 0) {
                puVar2 = pass1_1000_400a(*(param_1 + 0xb));
                uVar1 = -(puVar2.is_null() == false);
            }
        } else {
            uVar1 = 0xffff;
        }
    }
    return uVar1;
}

pub unsafe fn pass1_1000_2fa4(param_1: *mut i16) -> u16 {
    let mut piVar1: *mut i16;
    let mut bVar2: u8;
    let mut iVar3: i16;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;

    uVar6 = 0;
    bVar2 = *(param_1 + 0x5);
    if (((bVar2 & 0x3) == 0x2) && ((bVar2 & 0x8) != 0x0 || ((*(param_1 + 0x78) & 1) != 0))) {
        puVar4 = (*param_1 - param_1[0x3]);
        if (0x0 < puVar4) {
            puVar5 = mixed_dos3_call_1000_39f2(
                *(param_1 + 0xb),
                CONCAT22(param_1[0x4], param_1[0x3]),
                puVar4,
            );
            if (puVar5 == puVar4) {
                if ((*(param_1 + 0x5) & 0x80) != 0) {
                    piVar1 = param_1 + 0x5;
                    *piVar1 = *piVar1 & 0xfd;
                }
            } else {
                piVar1 = param_1 + 0x5;
                *piVar1 = *piVar1 | 0x20;
                uVar6 = 0xffff;
            }
        }
    }
    iVar3 = param_1[0x4];
    *param_1 = param_1[0x3];
    param_1[0x1] = iVar3;
    param_1[0x2] = 0;
    return uVar6;
}
