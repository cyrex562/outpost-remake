use crate::block_1000::block_1000_1000;
use crate::globals::DAT_1050_1050;
use crate::structs::struct_7::Struct7;
use crate::utils::CONCAT22;
use crate::winbase::{GLobalAlloc16, GlobalDOSFree16, GlobalFree16, GlobalHandle16, GlobalLock16, GlobalPageLock16, GlobalPageUnlock16, GlobalReAlloc16, GlobalSize16};
use crate::windef::HGLOBAL16;

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
        var2 = block_1000_1000::mem_op_1000_1558(param_2, param_2);
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

// WARNING: Unable to use type for symbol uVar3
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
