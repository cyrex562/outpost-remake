use crate::block_1000;
use crate::block_1000::block_1000_0000::call_fn_ptr_1000_0dc6;
use crate::block_1000::{block_1000_0000, block_1000_2000};
use crate::globals::{DAT_1050_5f30, REG_CS};
use crate::utils::CONCAT22;
use std::os::raw::c_char;
use crate::mem_ops::mem_op_1000_1532;
use crate::structs::struct_7::Struct7;

pub unsafe fn pass1_1000_1284(mut param_1: u32) -> u32 {
    let mut bVar1: u8;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut bVar4: u8;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut b_var6: bool;
    let mut DVar7: u32;
    let mut u_stack6: u16;
    let mut i_stack4: i16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        pass1_1000_1e61(REG_CS, 0xe, 0x0);
        return 0xffffffff;
    }
    bVar1 = *&PTR_LOOP_1050_000c;
    bVar4 = bVar1 & 0x7;
    if ((bVar1 & 0x7) != 0) {
        if (bVar4 == 1) {
            u_var3 = *NULL;
            return (u_var3 + 0x18);
        }
        if (bVar4 != 0x2) {
            if (bVar4 != 0x3) {
                return 0xffffffff;
            }
            DVar7 = get_mem_sz_1000_1532(0x0, param_1);
            return CONCAT22((DVar7 >> 0x10) - (DVar7 < 0x14), DVar7 - 0x14);
        }
    }
    u_var2 = (param_1 + -0x2);
    u_var5 = u_var2 & 0x7ffc;
    u_stack6 = u_var5 - 0x2;
    i_stack4 = 0;
    if ((u_var2 & 0x8000) != 0) {
        b_var6 = u_stack6 < 0x4;
        u_stack6 = u_var5 - 0x6;
        i_stack4 = -b_var6;
    }
    return CONCAT22(i_stack4, u_stack6);
}

pub unsafe fn mem_op_1000_131c(mut param_1: u16, mut param_2: u32) {
    let mut handle: HGLOBAL16;
    let mut flags: u16;
    let mut b_var1: bool;
    let mut lVar2: i32;
    let mut u_stack10: u16;
    let mut u_stack8: u16;
    let mut i_stack6: i16;

    lVar2 = CONCAT22(u_stack8, u_stack10);
    flags = 0x32;
    i_stack6 = 0x1;
    if (((param_1 & 0x1000) != 0) && (param_2 != 0x0 || (0xfff0 < param_2))) {
        param_2 = 0xfff0;
    }
    if ((param_1 & 0x100) != 0) {
        flags = 0x72;
    }
    if ((param_1 & 1) != 0) {
        flags |= 0x2000;
    }
    if ((param_1 & 0x4) != 0) {
        flags &= 0xfffd;
        lVar2 = mem_op_1000_1558(param_2, param_2);
    }
    loop {
        handle = GLobalAlloc16(param_2 & 0xffff | param_2 << 0x10, flags);
        u_stack8 = (lVar2 >> 0x10);
        u_stack10 = lVar2;
        if (handle != 0) {
            break;
        }
        flags &= 0xffcf;
        b_var1 = i_stack6 != 0;
        i_stack6 = i_stack6 + -0x1;
        if b_var1 == false {
            break;
        }
    }
    if ((param_1 & 0x4) != 0) {
        if (handle != 0) {
            GlobalPageLock16(handle);
        }
        pass1_1000_15ce(u_stack10, u_stack8);
    }
    if (handle == 0) {
        return;
    }
    WIN16_GlobalLock16(handle);
    return;
}

pub unsafe fn mem_op_1000_1408(mut param_1: *mut u8, re_alloc_size: u32, mut param_3: *mut Struct7, selector: u16) {
    let mut handle: HGLOBAL16;
    let mut global_handle_1: u32;
    let mut realloc_flags: u16;
    let mut global_handle_2: HGLOBAL16;

    global_handle_1 = GlobalHandle16(selector);
    //  global_handle_1 = global_handle_1;
    realloc_flags = 0x32;
    // (((param_1 & 0x1000) != 0) && ((re_alloc_size != 0x0 || (0xfff0 < re_alloc_size))))
    if (((param_1 & 0x1000) != 0) && (re_alloc_size != 0x0 || 0xfff0 < re_alloc_size)) {
        re_alloc_size = 0xfff0;
    }
    if ((param_1 & 0x100) != 0) {
        realloc_flags = 0x72;
    }
    if ((param_1 & 0x804) != 0) {
        realloc_flags &= 0xfffd;
    }
    if (global_handle_1 != 0) {
        if ((param_1 & 0x4) != 0) {
            GlobalPageUnlock16(global_handle_1);
        }
        while (global_handle_2 != 0) {
            global_handle_2 = global_handle_1;
            handle = GlobalReAlloc16(realloc_flags, re_alloc_size, global_handle_1);
            if (handle != 0) {
                break;
            }
            realloc_flags &= 0xffcf;
        }
        if ((handle != 0) && ((param_1 & 0x4) != 0)) {
            GlobalPageLock16(handle);
        }
        if (handle != 0) {
            WIN16_GlobalLock16(handle);
            return;
        }
    }
    return;
}

pub unsafe fn get_mem_sz_1000_1532(param_1: *mut Struct7, selector: i16) -> u32 {
    let mut mem_size: u32;

    // get handle to global memory
    mem_size = GlobalHandle16(selector);
    if mem_size != 0x0 {
        // get size of memory area
        mem_size = GlobalSize16(mem_size);
        return mem_size;
    }
    return 0x0;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn pass1_1000_15ce(param_1: *mut u16, mut param_2: u16) {
    let mut pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut pu_var2: *mut u16;
    let mut u_var3: u16;

    u_var2 = param_2 | param_1;
    while (u_var2 != 0) {
        pu_var1 = *param_1;
        u_var3 = param_1[0x1];
        GlobalDOSFree16(param_2);
        param_1 = pu_var1;
        param_2 = u_var3;
        u_var2 = u_var3 | pu_var1;
    }
    return;
}

pub unsafe fn pass1_1000_16ee(mut param_1: u16, mut param_2: u16) {
    if ((param_2 | param_1) != 0) {
        block_1000_0000::call_fn_ptr_1000_0dc6(CONCAT22(param_2, param_1));
    }
    return;
}

pub unsafe fn mem_op_1000_179c(mut param_1: i16, param_2: *mut astruct_57) {
    let mut puVar1: *mut u8;
    let mut puVar2: *mut u8;

    puVar1 = PTR_LOOP_1050_5f2c;
    puVar2 = PTR_LOOP_1050_5f2e;
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        puVar1 = mem_op_1000_160a(param_2);
        puVar2 = param_2;
    }
    fn_ptr_op_1000_1708(param_1, 0x0, 0x0, puVar1, puVar2);
    return;
}

pub unsafe fn fn_ptr_1000_17ce(param_1: *mut c_char) {
    if !param_1.is_null() {
        call_fn_ptr_1000_0dc6(param_1);
    }
    return;
}

pub unsafe fn pass1_1000_18d2(mut param_1: u16, mut param_2: u16) {
    if ((param_2 | param_1) != 0) {
        call_fn_ptr_1000_0dc6(CONCAT22(param_2, param_1));
    }
    return;
}

pub unsafe fn pass1_1000_1e61(mut param_1: u16, mut param_2: u16, param_3: *mut Struct7) -> u16 {
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut UVar3: u16;
    let mut UStack64: u16;
    let mut UStack62: u16;
    let mut UStack60: u16;
    let mut pcStack6: *mut code;
    let mut puStack4: *mut u8;
    let mut uVar3: u16;

    // &DAT_1050_1050
    uVar3 = 0x1050;
    UStack62 = param_3;
    UStack60 = param_4;
    UStack64 = param_2;
    puStack4 = 0x1050; // &DAT_1050_1050;
    pcStack6 = &PTR_PTR_1050_5f1a;
    if (PTR_LOOP_1050_5f1c | PTR_PTR_1050_5f1a) == 0x0 {
        pcStack6 = null_mut();
        puStack4 = null_mut();
    } else {
        iVar1 = mem_op_1000_21b6(PTR_PTR_1050_5f1a, PTR_LOOP_1050_5f1c);
        pcStack6 = PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0) {
            PTR_PTR_1050_5f1a = &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c = &PTR_LOOP_1050_1000;
            pcStack6 = &PTR_PTR_1050_1f7e;
            puStack4 = &PTR_LOOP_1050_1000;
        }
    }
    if ((puStack4 | pcStack6) == 0) {
        return 0x0;
    }
    BVar2 = msg_box_op_1000_1f24(&PTR_PTR_1050_5f1a, &DAT_1050_1050, 0x0);
    if (BVar2 == 0) {
        u16_var3 = (*pcStack6)(0x1000, &UStack64, &DAT_1050_1050, uVar3);
    } else {
        puStack4 = null_mut();
        pcStack6 = null_mut();
        u16_var3 = 0;
    }
    if ((puStack4 | pcStack6) != 0) {
        pass1_1000_1f68();
    }
    return u16_var3;
}

pub unsafe fn pass1_1000_1f68() {
    PTR_LOOP_1050_5f26 -= 1;
    if (PTR_LOOP_1050_5f26 < 0x0) {
        PTR_LOOP_1050_5f26 = 0;
    }
}

pub unsafe fn mem_op_1000_13ce(mut param_1: *mut Struct7, mut param_2: u16) -> i32 {
    let mut HVar1: HGLOBAL16;
    let mut uVar2: u16;
    let mut DVar3: u32;

    DVar3 = GlobalHandle16(param_2);
    uVar2 = (DVar3 >> 0x10);
    if (DVar3 != 0) {
        HVar1 = GlobalFree16(DVar3);
        return CONCAT22(uVar2, (HVar1 == 0));
    }
    return uVar2 << 0x10;
}

pub unsafe fn mem_op_1000_14f2(
    mut param_1: u16,
    mut param_2: u32,
    param_4: *mut Struct7,
    mut param_5: u16,
) -> bool {
    let mut in_AX: u16;
    let mut in_DX: u16;

    if (((param_1 & 0x1000) != 0) || (param_3 == 0x0 && (param_2 < 0xfff1))) {
        mem_op_1000_1408(
            param_1 & 0xfdff | 0x800,
            CONCAT22(param_3, param_2),
            param_4,
            param_5,
        );
        if ((in_DX | in_AX) != 0) {
            return 0x1;
        }
    }
    return 0x0;
}

pub unsafe fn mem_op_1000_1558(mut param_1: u16, mut param_2: u16) -> i32 {
    let mut uVar1: u16;
    let mut uVar3: u32;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uVar2: u16;

    uStack12 = 0;
    uStack10 = 0;
    uStack8 = 0x8;
    if ((param_2 < 0x9) && (param_2 < 0x8 || (param_1 == 0))) {
        while ((param_2 < uStack8) || (param_2 <= uStack8 && (param_1 <= uStack10))) {
            loop {
                uVar3 = GlobalDOSAlloc16(CONCAT22(uStack8, uStack10));
                uVar1 = uVar3;
                if (uVar1 == 0) {
                    break;
                }
                // *NULL = 0;
                u16_1050_0002 = uStack12;
                uStack12 = uVar1;
            }
            uVar2 = uStack8 & 0x1;
            uStack8 >>= 0x1;
            uStack10 = uStack10 >> 0x1 | (uVar2 != 0) << 0xf;
        }
    }
    return uStack12 << 0x10;
}

pub unsafe fn mem_op_1000_160a(param_1: *mut StructD) -> *const u8 {
    let mut puVar1: *mut u8;
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar1 = param_1;
    puVar1 = ret_true_1000_2146();
    if (puVar1.is_null()) {
        return puVar1;
    }
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        DAT_1050_5f30 = 0x1;
        DAT_1050_5f32 = 0x1;
        uVar2 = mem_op_1000_18ec(DAT_1050_5f46, uVar1);
        PTR_LOOP_1050_5f2e = (uVar2 >> 0x10);
        PTR_LOOP_1050_5f2c = uVar2;
        if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) != 0) {
            if (PTR_LOOP_1050_5f42.is_null() == false) {
                pass1_1000_1a54(PTR_LOOP_1050_5f42, PTR_LOOP_1050_5f2c, PTR_LOOP_1050_5f2e);
            }
            if (DAT_1050_5f44 != 0xffff) {
                pass1_1000_1afe(DAT_1050_5f44, PTR_LOOP_1050_5f2c, PTR_LOOP_1050_5f2e);
            }
        }
    }
    block_1000_2000::empty_fn_1000_214a();
    return PTR_LOOP_1050_5f2c;
}

pub unsafe fn mem_1000_167a(mut param_1: u16, mut param_2: u16) -> u16 {
    let mut puVar1: *mut u8;
    let mut in_register_0000000a: u16;
    let mut pSVar2: *mut StructD;
    let mut lVar3: i32;

    pSVar2 = CONCAT22(in_register_0000000a, param_1);
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        puVar1 = mem_op_1000_160a(pSVar2);
        if ((pSVar2 | puVar1) == 0) {
            return 0x0;
        }
    }
    lVar3 = mem_op_1000_0a48(
        0x0,
        param_2,
        0x0,
        CONCAT22(PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c),
    );
    return lVar3;
}

pub unsafe fn pass1_1000_16aa(
    mut param_1: u16,
    param_2: *mut u16,
    mut param_3: u16,
    mut param_4: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;

    if ((param_3 | param_2) == 0) {
        uVar1 = mem_1000_167a(param_1, param_4);
        return uVar1;
    }
    if (param_4 == 0) {
        pass1_1000_16ee(param_2, param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0, param_4, 0x0, param_2, param_3);
    return lVar2;
}

pub unsafe fn fn_ptr_op_1000_1708(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) -> u16 {
    let mut iVar1: i16;
    let mut bVar2: bool;
    let mut lVar3: i32;

    if ((param_2 | param_1) == 0) {
        bVar2 = 0xfffe < param_1;
        param_1 += 0x1;
        param_2 += bVar2;
    } //
      //    LAB_1000_1724:
    loop {
        if ((param_5 | param_4) != 0) {
            lVar3 = mem_op_1000_0a48(param_3, param_1, param_2, CONCAT22(param_5, param_4));
            if (lVar3 != 0) {
                return lVar3;
            }
        }
        if (((param_3 & 0x8000) == 0) || ((PTR_LOOP_1050_5f3a | PTR_LOOP_1050_5f38) == 0)) {
            if ((PTR_LOOP_1050_5f36 | PTR_LOOP_1050_5f34) == 0) {
                if ((PTR_LOOP_1050_5f3e | PTR_LOOP_1050_5f3c) == 0) {
                    return 0x0;
                }
                (PTR_LOOP_1050_5f3c)();
                // TODO
                // goto LAB_1000_1724;
            }
            iVar1 = (PTR_LOOP_1050_5f34)();
        } else {
            iVar1 = (PTR_LOOP_1050_5f38)();
        }
        if (iVar1 == 0) {
            return 0x0;
        }
    }
}

pub unsafe fn pass1_1000_17e8(param_1: *const u8, param_2: *const u8) -> *const u8 {
    let mut puVar1: *mut u8;

    puVar1 = PTR_LOOP_1050_5f34;
    PTR_LOOP_1050_5f34 = param_1;
    PTR_LOOP_1050_5f36 = param_2;
    return puVar1;
}

pub unsafe fn pass1_1000_180c(mut param_1: u16, mut param_2: u16) -> u16 {
    let mut puVar1: *mut u8;
    let mut in_register_0000000a: u16;
    let mut pSVar2: *mut StructD;
    let mut lVar3: i32;

    pSVar2 = CONCAT22(in_register_0000000a, param_1);
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        puVar1 = mem_op_1000_160a(pSVar2);
        if ((pSVar2 | puVar1) == 0) {
            return 0x0;
        }
    }
    lVar3 = mem_op_1000_0a48(
        0x0,
        param_2,
        0x0,
        CONCAT22(PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c),
    );
    return lVar3;
}

pub unsafe fn pass1_1000_183c(mut param_1: u16, mut param_2: u16) -> u16 {
    let mut in_EDX: u32;
    let mut pSVar1: *mut StructD;
    let mut lVar2: i32;

    pSVar1 = (in_EDX & 0xffff0000);
    if ((param_2 * param_1 >> 0x10) != 0) {
        return 0x0;
    }
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar1);
        PTR_LOOP_1050_5f2e = pSVar1;
        if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
            return 0x0;
        }
    }
    lVar2 = mem_op_1000_0a48(
        0x1,
        (param_2 * param_1),
        0x0,
        CONCAT22(PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c),
    );
    return lVar2;
}

pub unsafe fn pass1_1000_188e(
    mut param_1: u16,
    param_2: *mut u16,
    mut param_3: u16,
    mut param_4: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;

    if ((param_3 | param_2) == 0) {
        uVar1 = pass1_1000_180c(param_1, param_4);
        return uVar1;
    }
    if (param_4 == 0) {
        pass1_1000_18d2(param_2, param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0, param_4, 0x0, param_2, param_3);
    return lVar2;
}

pub unsafe fn mem_op_1000_18ec(mut param_1: u16, mut param_2: u16) -> u32 {
    let mut uVar1: u32;

    uVar1 = mem_op_1000_1902(param_2, param_1, 0x0, 0x0, 0xc);
    return uVar1;
}

pub unsafe fn mem_op_1000_1902(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) -> u32 {
    let mut pUVar1: *mut u16;
    let mut UVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar3: u16;
    let mut UVar5: u16;
    let mut pUVar6: *mut u16;
    let mut unaff_CS: u16;
    let mut DVar7: u32;
    let mut uVar8: u32;
    let mut puVar1: *mut u16;

    UVar5 = param_1;
    if (((param_2 & 0x8000) != 0) && (UVar5 = param_1, _SHI_INVOKEERRORHANDLER1 != -0x6f70)) {
        param_2 |= 0x1;
        UVar5 = param_1;
    }
    loop {
        uVar3 = UVar5;
        pUVar1 = (param_2 & 0xfffb | 0x1000);
        mem_op_1000_131c(pUVar1, 0x100);
        UVar5 = uVar3 | pUVar1;
        if (UVar5 != 0) {
            break;
        }
        UVar2 = pass1_1000_1e61(unaff_CS, 0x2, 0x0);
        if UVar2 == 0x0 {
            break;
        }
    }
    if ((uVar3 | pUVar1) != 0) {
        pUVar1[0x17] = &PTR_PTR_1050_5f1a;
        pUVar1[0x18] = &DAT_1050_1050;
        pUVar1[0x15] = PTR_LOOP_1050_5f1e;
        pUVar1[0x16] = PTR_LOOP_1050_5f20;
        pUVar6 = pUVar1;
        PTR_LOOP_1050_5f1e = pUVar1;
        PTR_LOOP_1050_5f20 = uVar3;
        // for (iVar4 = 0x5; iVar4 != 0; iVar4 += -1)
        for iVar4 in 5..0 {
            puVar1 = pUVar6;
            pUVar6 = pUVar6 + 1;
            *puVar1 = 0;
        }
        pUVar1[0x5] = 0;
        pUVar1[0x7] = 0;
        pUVar1[0x6] = 0;
        pUVar1[0x9] = 0;
        pUVar1[0x8] = 0;
        pUVar1[0xa] = 0xbead;
        pUVar1[0xb] = param_2 & 0xfffd;
        pUVar1[0xc] = 0;
        pUVar1[0xd] = 0x2000;
        pUVar1[0xe] = 0x800;
        DVar7 = mem_op_1000_1532(pUVar1, uVar3);
        pUVar1[0xf] = DVar7;
        pUVar1[0x10] = (DVar7 >> 0x10);
        pUVar1[0x12] = 0;
        pUVar1[0x11] = 0;
        pUVar1[0x13] = 0xfffe;
        pUVar1[0x14] = 0xffff;
        pUVar1[0x19] = 0;
        pUVar1[0x1a] = 0;
        pUVar1[0x20] = 0;
        pUVar1[0x1f] = 0;
        BVar3 = pass1_1000_1afe(param_5, pUVar1, uVar3);
        if (BVar3 != 0) {
            if ((param_4 | param_3) != 0) {
                pUVar6 = pUVar1;
                UVar5 = uVar3;
                uVar8 = pass1_1000_52be(param_3, param_4, param_5, 0x0);
                pass1_1000_010c(0x1, uVar8, (uVar8 >> 0x10), pUVar6, UVar5);
            }
            return CONCAT22(uVar3, pUVar1);
        }
        mem_op_1000_1b9a(0x0, pUVar1, uVar3);
    }
    return 0x0;
}

pub unsafe fn pass1_1000_1a54(mut param_1: u16, mut param_2: i16, mut param_3: u16) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut unaff_CS: u16;

    if ((param_2 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS, 0xa, 0x0);
        return 0x0;
    }
    uVar1 = pass1_1000_1ab0(param_1);
    if (uVar1 < (param_2 + 0x18) + 0x14) {
        uVar2 = 0;
    } else {
        uVar2 = (param_2 + 0x1a);
        (param_2 + 0x1a) = uVar1;
        (param_2 + 0x1c) = uVar1 >> 0x2;
    }
    return uVar2;
}

pub unsafe fn pass1_1000_1ab0(mut param_1: u16) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;

    if (param_1 == 0x2000) {
        return 0x2000;
    }
    if (param_1 < 0xfff0) {
        if (param_1 < 0x1001) {
            return 0x1000;
        }
        uVar1 = 0x2000;
        if (param_1 < 0x2001) {
            loop {
                uVar2 = uVar1;
                uVar1 = uVar2 >> 0x1;
                if parem_1 > uVar1 {
                    break;
                }
            }
            return uVar2 & 0xfffe;
        }
        while (uVar1 *= 0x2, uVar1 != 0) {
            if (param_1 <= uVar1) {
                return (uVar1 + 0x10 & -(uVar1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}

pub unsafe fn pass1_1000_1afe(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> bool {
    let mut uVar1: u16;
    let mut unaff_CS: u16;

    if (param_1 == 0) {
        uVar1 = 0;
    } else {
        uVar1 = param_1 + 0x1 & 0xfffe;
    }
    if ((param_2 + 0x14) == -0x4153) {
        if ((uVar1 < param_1) || ((param_2 + 0x1a) - 0x14 < uVar1)) {
            pass1_1000_1e61(unaff_CS, 0x3, param_2);
        } else if ((param_2 + 0x2) == 0) {
            (param_2 + 0x18) = uVar1;
            return 0x1;
        }
        return 0x0;
    }
    pass1_1000_1e61(unaff_CS, 0xa, 0x0);
    return 0x0;
}

pub unsafe fn mem_op_1000_1b68(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u32 {
    let mut unaff_CS: u16;
    let mut uVar1: u32;

    if ((param_2 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS, 0xa, 0x0);
        return param_1 << 0x10;
    }
    uVar1 = mem_op_1000_1b9a(0x0, param_2, param_3);
    return uVar1;
}

pub unsafe fn mem_op_1000_1b9a(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut lVar7: i32;
    let mut puStack8: *mut u16;
    let mut uStack4: u16;

    (param_2 + 0x14) = 0;
    uStack4 = 0;
    loop {
        iVar6 = (uStack4 * 0x2);
        if (iVar6 != 0) {
            loop {
                uVar3 = (iVar6 + 0x8);
                (uVar3 + 0xc) = 0;
                mem_op_1000_13ce((iVar6 + 0x8), (iVar6 + 0xa));
                iVar6 = (iVar6 + 0x4);
                if (uStack4 * 0x2) == iVar6 {
                    break;
                }
            }
        }
        uStack4 += 0x1;
        if uStack4 >= 5 {
            break;
        }
    }
    uVar4 = (param_2 + 0x10);
    uVar5 = (param_2 + 0x12);
    loop {
        puStack8 = CONCAT22(uVar5, uVar4);
        if ((uVar5 | uVar4) == 0) {
            break;
        }
        uVar1 = *puStack8;
        uVar2 = (uVar4 + 2);
        mem_op_1000_13ce(uVar4, uVar5);
        uVar4 = uVar1;
        uVar5 = uVar2;
    }
    block_1000_2000::pass1_1000_20a2(param_2, param_3);
    lVar7 = mem_op_1000_13ce(param_2, param_3);
    return CONCAT22((lVar7 >> 0x10), 1);
}

pub unsafe fn mem_op_1000_1dfa(mut param_1: i16, param_2: u8, mut param_3: u16, mut param_4: u16) -> bool {
    let mut uVar1: u32;
    let mut uVar2: u16;

    if ((param_2 & 0x4) == 0) {
        uVar2 = (((-((param_2 & 0x2) == 0) >> 0x8) & 0xfe) + 0x92) << 0x8;
    } else {
        uVar2 = 0x1800;
    }
    if ((param_4 == 0)
        || ((param_4 & 0xff00 & (((-((param_2 & 0x4) == 0) >> 0x8) & 0x82) + 0x18) << 0x8)
            != uVar2))
    {
        return 0x1;
    }
    if (param_1 != 0) {
        uVar1 = SegmentLimit(param_4);
        if (CARRY2(param_3, param_1 - 1)) {
            return 0x1;
        }
        if (uVar1 < param_3 + (param_1 - 1)) {
            return 0x1;
        }
    }
    return 0x0;
}

pub unsafe fn msg_box_op_1000_1f24(mut param_1: i16, mut param_2: u16, mut param_3: u16) -> bool {
    let mut piVar1: *mut i16;
    let mut unaff_CS: u16;

    if (param_3 < (param_1 + 0xc)) {
        msg_box_op_1000_214c(0x0, 0x0, 0xd940, &PTR_LOOP_1050_1040);
        return 0x1;
    }
    piVar1 = (param_1 + 0xc);
    *piVar1 = *piVar1 + 1;
    return 0x0;
}

pub unsafe fn pass1_1000_1f7e(param_1: *mut u16) -> bool {
    let mut cVar1: u8;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut pcVar5: *mut c_char;

    uVar3 = *param_1;
    if (uVar3 == 0xf) {
        //
        //        LAB_1000_1fb6:
        iVar4 = 0x1;
    } else {
        if (uVar3 < 0x10) {
            cVar1 = uVar3;
            if (cVar1 == '\x02') {
                // TODO: goto LAB_1000_1fb6;
            }
            if (('\0' < (cVar1 + -0x2)) && ((cVar1 + -0x3) < '\x0f')) {
                iVar4 = 0;
                // TODO: goto LAB_1000_1fbe;
            }
        }
        iVar4 = 0;
        uVar3 = 0x1;
    } //
      //    LAB_1000_1fbe:
    pcVar5 = pass1_1000_1fd2(uVar3);
    BVar2 = msg_box_op_1000_214c(0x0, iVar4, pcVar5, (pcVar5 >> 0x10));
    return BVar2;
}

pub unsafe fn ass1_1000_1fd2(mut param_1: i16) -> *mut c_char {
    if (param_1 == 0x2) {
        return "Out of memory.  Please free some memory, then choose retry.";
    }
    return CONCAT22(0x1000, param_1 * 0x17 + 0x1c7a);
}

pub unsafe fn pass1_1000_1fea() -> bool {
    let mut puVar1: *mut u8;
    let mut bVar2: bool;

    puVar1 = PTR_LOOP_1050_5f22 + 1;
    bVar2 = PTR_LOOP_1050_5f22.is_null();
    PTR_LOOP_1050_5f22 = puVar1;
    if ((bVar2) && ((PTR_LOOP_1050_5f20 | PTR_LOOP_1050_5f1e) != 0)) {
        PTR_LOOP_1050_5f22 = &u16_1050_0002;
    }
    return 0x1;
}
