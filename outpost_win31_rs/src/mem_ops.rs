use std::ptr::null_mut;
use crate::unk::block_1000_0000::mem_op_1000_0a48;
use crate::unk::block_1000_1000::{mem_op_1000_179c, pass1_1000_1284};
use crate::unk::block_1008_4000;
use crate::globals::{DAT_1050_1050, u16_1050_0002};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_7::Struct7;
use crate::unk::{block_1000_1000, block_1000_2000};
use crate::utils::{CARRY2, CONCAT22};
use crate::winapi16::{GLobalAlloc16, GlobalDOSAlloc16, GlobalDOSFree16, GlobalFree16, GlobalHandle16, GlobalLock16, GlobalPageLock16, GlobalPageUnlock16, GlobalReAlloc16, GlobalSize16, hmemcpy16, SegmentLimit};
use crate::windef16::HGLOBAL16;

// dvar6 = mem_op_1000_1532(param_1, 0x1050);
pub unsafe fn mem_op_1000_1532(a: *mut Struct7, b: u16) -> u32 {
    todo!()
}

pub unsafe fn alloc_mem_1000_131c(mut param_1: u16, mut param_2: u32) {
    let mut handle: HGLOBAL16;
    let mut b_var1: bool;
    let mut u_stack10 = 0u16;
    let mut u_stack8 = 0u16;

    let mut var2 = CONCAT22(u_stack8, u_stack10);
    let mut flags = 0x32;

    if ((param_1 & 0x1000) != 0) && (param_2 != 0x0 || (0xfff0 < param_2)) {
        param_2 = 0xfff0;
    }
    if (param_1 & 0x100) != 0 {
        flags = 0x72;
    }
    if (param_1 & 1) != 0 {
        flags |= 0x2000;
    }
    if (param_1 & 0x4) != 0 {
        flags &= 0xfffd;
        var2 = mem_op_1000_1558(param_2, param_2);
    }

    let mut i_stack6 = 0x1;
    loop {
        handle = GLobalAlloc16(param_2 & 0xffff | param_2 << 0x10, flags);
        u_stack8 = (var2 >> 0x10) as u16;
        u_stack10 = var2 as u16;
        if handle != 0 {
            break;
        }
        flags &= 0xffcf;
        let b_var1 = i_stack6 != 0;
        i_stack6 -= 1;
        if b_var1 == false {
            break;
        }
    }
    if (param_1 & 0x4) != 0 {
        if handle != 0 {
            GlobalPageLock16(handle);
        }
        free_blocks_1000_15ce(u_stack10, u_stack8);
    }
    if handle == 0 {
        return;
    }
    GlobalLock16(handle);
    return;
}

pub unsafe fn realloc_1000_1408(mut param_1: *mut u8, mut re_alloc_size: u16, mut param_3: *mut Struct7, selector: u16) {
    let mut handle: HGLOBAL16 = 0xffff;
    let mut realloc_flags: u16;
    let mut global_handle_2: HGLOBAL16;

    let global_handle_1 = GlobalHandle16(selector);
    //  global_handle_1 = global_handle_1;
    realloc_flags = 0x32;
    // (((param_1 & 0x1000) != 0) && ((re_alloc_size != 0x0 || (0xfff0 < re_alloc_size))))
    if ((param_1 & 0x1000) != 0) && (re_alloc_size != 0x0 || 0xfff0 < re_alloc_size) {
        re_alloc_size = 0xfff0;
    }
    if (param_1 & 0x100) != 0 {
        realloc_flags = 0x72;
    }
    if (param_1 & 0x804) != 0 {
        realloc_flags &= 0xfffd;
    }
    if global_handle_1 != 0 {
        if (param_1 & 0x4) != 0 {
            GlobalPageUnlock16(global_handle_1 as HGLOBAL16);
        }
        while global_handle_2 != 0 {
            global_handle_2 = global_handle_1 as HGLOBAL16;
            handle = GlobalReAlloc16(realloc_flags, re_alloc_size, global_handle_1);
            if handle != 0 {
                break;
            }
            realloc_flags &= 0xffcf;
        }
        if (handle != 0) && ((param_1 & 0x4) != 0) {
            GlobalPageLock16(handle);
        }
        if handle != 0 {
            GlobalLock16(handle);
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


pub unsafe fn free_blocks_1000_15ce(param_1: *mut u16, mut param_2: u16) {
    let mut pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut pu_var2: *mut u16;
    let mut u_var3: u16;

    u_var2 = param_2 | param_1;
    while u_var2 != 0 {
        pu_var1 = *param_1;
        u_var3 = param_1[0x1];
        GlobalDOSFree16(param_2);
        param_1 = pu_var1;
        param_2 = u_var3;
        u_var2 = u_var3 | pu_var1;
    }
    return;
}

// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)
pub unsafe fn free_mem_1000_407a(mut param_1: u16, mut param_2: u16) {
    GlobalFree16(DAT_1050_1050);
    return;
}

pub unsafe fn free_mem_1000_13ce(mut param_1: *mut Struct7, mut param_2: u16) -> i32 {
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


pub unsafe fn memcpy_op_1008_4274(mut param_1: u16, param_2: *mut astruct_826) {
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_826;
    let mut uVar3: *mut astruct_827;
    let mut uVar4: u16;
    let mut count: u32;
    // pub unsafe fn *dst;
    let mut paStack14: *mut astruct_76;
    let mut paVar2: *mut Struct57;

    uVar5 = (in_EDX >> 0x10);
    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if (iVar3.pvoid32_0x6.is_null() == false) {
        count = pass1_1000_1284(iVar3.pvoid32_0x6);
        dst = mem_op_1000_0a48(0x1, count, (count >> 0x10), _PTR_LOOP_1050_5f2c);
        uVar3 = dst;
        uVar1 = (dst >> 0x10) | uVar3;
        paVar2 = CONCAT22(uVar5, uVar1);
        if (uVar1 != 0) {
            hmemcpy16(count, iVar3.pvoid32_0x6, dst);
            mem_op_1000_179c(0x1e, paVar2);
            uVar2 = paVar2 | uVar3;
            if (uVar2 == 0) {
                uVar3 = null_mut();
                uVar2 = 0;
            } else {
                block_1008_4000::pass1_1008_4016(CONCAT22(paVar2, uVar3));
            }
            paStack14 = CONCAT22(uVar2, uVar3);
            uVar3.field6_0x6 = dst;
            block_1008_4000::pass1_1008_47cc(CONCAT22(uVar2, uVar3));
            block_1008_4000::pass1_1008_4834(paStack14);
            uVar3.field25_0x1c = 0x1;
            return;
        }
    }
    return;
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
            BVar2 = block_1000_2000::pass1_1000_22c0(param_2, _param_1, param_2, param_3, param_4);
            if (BVar2 != 0) {
                return 0x1;
            }
        }
    }
    return 0x0;
}
