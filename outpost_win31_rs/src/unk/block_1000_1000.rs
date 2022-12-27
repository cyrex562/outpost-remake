use crate::{block_1000, mem_ops};
use crate::unk::block_1000_0000::{call_fn_ptr_1000_0dc6, mem_op_1000_0a48, pass1_1000_010c, pass1_1000_0ed4};
use crate::globals::{DAT_1050_1050, DAT_1050_5f30, PTR_LOOP_1050_000c, PTR_LOOP_1050_1000, PTR_LOOP_1050_1040, PTR_LOOP_1050_5f26, REG_CS, u16_1050_0002};
use crate::utils::{CARRY2, CONCAT22};
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::unk::block_1000_2000::{mem_op_1000_21b6, ret_true_1000_2146};
use crate::unk::block_1000_5000::pass1_1000_52be;
use crate::mem_ops::mem_op_1000_1532;
use crate::structs::struct_7::Struct7;
use crate::structs::struct_d::StructD;
use crate::os_base::_SHI_INVOKEERRORHANDLER1;
use crate::unk::{block_1000_0000, block_1000_2000};
use crate::error_handling::smart_heap_library_err_1000_214c;
use crate::structs::struct_57::Struct57;
use crate::winapi16::{GLobalAlloc16, GlobalDOSAlloc16, GlobalDOSFree16, GlobalFree16, GlobalHandle16, GlobalLock16, GlobalPageLock16, GlobalPageUnlock16, GlobalReAlloc16, GlobalSize16, SegmentLimit};
use crate::windef16::HGLOBAL16;

pub fn pass1_1000_1284(mut param_1: u32) -> u32 {
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
            DVar7 = mem_ops::get_mem_sz_1000_1532(0x0, param_1);
            return CONCAT22((DVar7 >> 0x10) - (DVar7 < 0x14), DVar7 - 0x14);
        }
    }
    u_var2 = (param_1 -0x2);
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

pub fn pass1_1000_16ee(mut param_1: u16, mut param_2: u16) {
    if ((param_2 | param_1) != 0) {
        block_1000_0000::call_fn_ptr_1000_0dc6(ctx, CONCAT22(param_2, param_1));
    }
    return;
}

pub  fn mem_op_1000_179c(mut param_1: i16, param_2: *mut Struct57) {
    let mut pu_var1: *mut u8;
    let mut pu_var2: *mut u8;

    pu_var1 = PTR_LOOP_1050_5f2c;
    pu_var2 = PTR_LOOP_1050_5f2e;
    if (PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0 {
        pu_var1 = mem_op_1000_160a(param_2);
        pu_var2 = param_2;
    }
    fn_ptr_op_1000_1708(param_1, 0x0, 0x0, pu_var1, pu_var2);
    return;
}

pub fn fn_ptr_1000_17ce(param_1: *mut c_char) {
    if !param_1.is_null() {
        call_fn_ptr_1000_0dc6(ctx, param_1);
    }
    return;
}

pub fn pass1_1000_18d2(mut param_1: u16, mut param_2: u16) {
    if ((param_2 | param_1) != 0) {
        call_fn_ptr_1000_0dc6(ctx, CONCAT22(param_2, param_1));
    }
    return;
}

pub fn pass1_1000_1e61(mut param_1: u16, mut param_2: u16, param_3: *mut Struct7) -> u16 {
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut UVar3: u16;
    let mut UStack64: u16;
    let mut UStack62: u16;
    let mut UStack60: u16;
    let mut pcStack6: *mut code;
    let mut puStack4: *mut u8;
    let mut uVar3: u16;

    // 0x1050
    uVar3 = 0x1050;
    UStack62 = param_3;
    UStack60 = param_4;
    UStack64 = param_2;
    puStack4 = 0x1050; // 0x1050;
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
    BVar2 = msg_box_op_1000_1f24(&PTR_PTR_1050_5f1a, 0x1050, 0x0);
    if (BVar2 == 0) {
        u16_var3 = (*pcStack6)(0x1000, &UStack64, 0x1050, uVar3);
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

pub fn pass1_1000_1f68() {
    PTR_LOOP_1050_5f26 -= 1;
    if (PTR_LOOP_1050_5f26 < 0x0) {
        PTR_LOOP_1050_5f26 = 0;
    }
}

pub fn mem_op_1000_14f2(
    mut param_1: u16,
    mut param_2: u32,
    param_4: *mut Struct7,
    mut param_5: u16,
) -> bool {
    let mut in_AX: u16;
    let mut in_DX: u16;

    if (((param_1 & 0x1000) != 0) || (param_3 == 0x0 && (param_2 < 0xfff1))) {
        mem_ops::realloc_1000_1408(
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

pub fn mem_op_1000_160a(param_1: *mut StructD) -> *const u8 {
    let mut pu_var1: *mut u8;
    let mut u_var1: u16;
    let mut u_var2: u32;

    u_var1 = param_1;
    pu_var1 = ret_true_1000_2146();
    if (pu_var1.is_null()) {
        return pu_var1;
    }
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        DAT_1050_5f30 = 0x1;
        DAT_1050_5f32 = 0x1;
        u_var2 = mem_op_1000_18ec(DAT_1050_5f46, u_var1);
        PTR_LOOP_1050_5f2e = (u_var2 >> 0x10);
        PTR_LOOP_1050_5f2c = u_var2;
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

pub fn mem_1000_167a(mut param_1: u16, mut param_2: u16) -> u16 {
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

pub fn pass1_1000_16aa(
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

pub fn fn_ptr_op_1000_1708(
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



pub fn pass1_1000_180c(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    let mut puVar1: *mut u8;
    let mut pSVar2: *mut StructD;
    let mut lVar3: i32;

    pSVar2 = CONCAT22(param_3, param_1);
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

pub fn mem_op_1000_18ec(mut param_1: u16, mut param_2: u16) -> u32 {
    let mut uVar1: u32;

    uVar1 = mem_op_1000_1902(param_2, param_1, 0x0, 0x0, 0xc);
    return uVar1;
}

pub fn mem_op_1000_1902(
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
        mem_ops::alloc_mem_1000_131c(pUVar1, 0x100);
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
        pUVar1[0x18] = 0x1050;
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

pub fn pass1_1000_1a54(mut param_1: u16, mut param_2: i16, mut param_3: u16) -> u16 {
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

pub fn pass1_1000_1ab0(mut param_1: u16) -> u16 {
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

pub fn pass1_1000_1afe(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> bool {
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

pub fn mem_op_1000_1b68(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u32 {
    let mut unaff_CS: u16;
    let mut uVar1: u32;

    if ((param_2 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS, 0xa, 0x0);
        return param_1 << 0x10;
    }
    uVar1 = mem_op_1000_1b9a(0x0, param_2, param_3);
    return uVar1;
}

pub fn mem_op_1000_1b9a(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u32 {
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
                mem_ops::free_mem_1000_13ce((iVar6 + 0x8), (iVar6 + 0xa));
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
        mem_ops::free_mem_1000_13ce(uVar4, uVar5);
        uVar4 = uVar1;
        uVar5 = uVar2;
    }
    block_1000_2000::pass1_1000_20a2(param_2, param_3);
    lVar7 = mem_ops::free_mem_1000_13ce(param_2, param_3);
    return CONCAT22((lVar7 >> 0x10), 1);
}

pub fn msg_box_op_1000_1f24(mut param_1: i16, mut param_2: u16, mut param_3: u16) -> bool {
    let mut piVar1: *mut i16;
    let mut unaff_CS: u16;

    if (param_3 < (param_1 + 0xc)) {
        smart_heap_library_err_1000_214c(0x0, 0x0, 0xd940, &PTR_LOOP_1050_1040);
        return 0x1;
    }
    piVar1 = (param_1 + 0xc);
    *piVar1 = *piVar1 + 1;
    return 0x0;
}

pub fn pass1_1000_1fd2(mut param_1: i16) -> *mut c_char {
    if (param_1 == 0x2) {
        return "Out of memory.  Please free some memory, then choose retry.";
    }
    return CONCAT22(0x1000, param_1 * 0x17 + 0x1c7a);
}

pub fn pass1_1000_1fea() -> bool {
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
