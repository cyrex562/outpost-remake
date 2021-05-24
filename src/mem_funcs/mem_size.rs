use crate::err_ops::invoke_error_handler_1000_1e61;
use crate::mem_funcs::alloc_mem;
use crate::pass::pass_funcs::pass1_fn_1000_5390;
use crate::util::{CARRY2, CONCAT22};

pub unsafe fn get_mem_sz_1000_01b0() -> bool {
    let pu_var1: &mut  u32;
    let pi_var2: &mut  i32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut in_bx: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut unaff_cs: u16;
    let lVar7: u32;
    let lVar8: u32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_e = 0;
    if (((in_bx + 0x40) | (in_bx + 0x3e)) == 0) {
        u_var5 = in_bx + 0x36;
        lVar8 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
        lVar7 = lVar8;
    } else {
        lVar7 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
        u_var5 = lVar7;
        if (((lVar7 >> 0x10) != 0) || (0xffef < u_var5)) {
            invoke_error_handler_1000_1e61(unaff_cs, 8, in_bx);
            return false;
        }
        if (0x1fff < u_var5) {
            u_var5 = 0x2000;
        }
        while (true) {
            u_var4 = u_var5;
          // local_4 = (lVar7  >> 0x10);
            local_6 = lVar7;
            lVar8 = lVar7 + 0x18;
            if (((lVar8 >> 0x10) != 0) || (0xfff0 < lVar8)) {
                lVar8 = 0xfff0;
            }
            i_var3 = alloc_mem::alloc_mem_1000_14f2(
                (in_bx + 0x16) | 0x1000,
                lVar8,
                in_bx,
                &ctx.g_alloc_addr_1050_1050,
            );
            if (i_var3 != 0) {
                break;
            }
            u_var5 = u_var4 >> 1;
            if (u_var5 < 0xc) {
                i_var3 = invoke_error_handler_1000_1e61(unaff_cs, 2, in_bx);
                if (i_var3 == 0) {
                    return (in_bx + 10) != 0;
                }
                lVar7 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
                u_var5 = u_var4 & 0xfffe;
            }
        }
        i_var3 = pass1_fn_1000_5390(local_6 - 0x42, local_4 - (local_6 < 0x42), 0xc, 0);
        u_var5 = i_var3 * 0xc + in_bx + 0x42;
    }
    pu_var1 = (in_bx + 0x1e);
    unsafe {
        u_var4 = *pu_var1;
        *pu_var1 = *pu_var1 - lVar7;
        pi_var2 = (in_bx + 0x20);
       // pi_var2 = (*pi_var2 - (lVar7  >> 0x10)) - (u_var4 < lVar7);
    }
    if (u_var5 != 0) {
        u_var10 = 0;
        u_var9 = 0xc;
        lVar8 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
        i_var3 = pass1_fn_1000_5390(
            lVar8 - 0x42,
            (lVar8 >> 0x10) - (lVar8 < 0x42),
            u_var9,
            u_var10,
        );
        local_e = i_var3 * 0xc + in_bx + 0x36;
    }
  // local_a = (lVar8  >> 0x10);
    local_c = lVar8;
    pu_var1 = (in_bx + 0x1e);
    unsafe {
        u_var4 = *pu_var1;
        *pu_var1 = *pu_var1 + local_c;
        pi_var2 = (in_bx + 0x20);
        *pi_var2 = *pi_var2 + local_a + CARRY2(u_var4, local_c);
        u_var4 = (in_bx + 10);
    }
    while {
        u_var6 = u_var5;
        (u_var6 + 4) = u_var4;
        u_var4 = u_var6;
        u_var5 = u_var6 + 0xc;
        u_var6 < local_e
    } {}
    (in_bx + 10) = u_var6;
    return true;
}

pub unsafe fn get_mem_sz_1000_0308() -> i32 {
    let mut iVar1: i32;
    let mut in_ax: i32;
    let mut i_var2: i32;
    let mut in_bx: i32;
    let pi_var3: &mut  i32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (((in_bx + 10) == 0) && (i_var2 = get_mem_sz_1000_01b0(), i_var2 == 0)) {
        return 0;
    }
    i_var2 = (in_bx + 10);
    (in_bx + 10) = (i_var2 + 4);
    pi_var3 = (in_ax * 2 + in_bx);
    let pi_var3_val = unsafe { *pi_var3 };
    if (pi_var3_val == 0) {
        (i_var2 + 6) = i_var2;
        (i_var2 + 4) = i_var2;
    } else {
        iVar1 = pi_var3_val;
        (i_var2 + 6) = iVar1;
        (i_var2 + 4) = (iVar1 + 4);
        ((iVar1 + 4) + 6) = i_var2;
        (iVar1 + 4) = i_var2;
    }
    unsafe { *pi_var3 = i_var2 };
    return i_var2;
}

pub unsafe fn get_mem_sz_1000_1284(param_1: u32) -> u32 {
    let mut bVar1: u8;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u8_var4: u8;
    let mut u_var5: i32;
    let mut unaff_cs: u16;
    let mut bVar6: bool;
    let mut alloc_size: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0 {
        invoke_error_handler_1000_1e61(unaff_cs, 0xe, 0);
        return 0xffffffff;
    }
    bVar1 = *&PTR_LOOP_1050_000c;
    u8_var4 = bVar1 & 7;
    if ((bVar1 & 7) != 0) {
        if (u8_var4 == 1) {
            u_var3 = 0x0;
            return *(u_var3 + 0x18);
        }
        if (u8_var4 != 2) {
            if (u8_var4 != 3) {
                return 0xffffffff;
            }
            alloc_size = get_mem_sz_1000_1532(param_1);
            return CONCAT22(
                (alloc_size >> 0x10) - (alloc_size < 0x14),
                alloc_size - 0x14,
            );
        }
    }
    u_var2 = (param_1 + -2);
    u_var5 = u_var2 & 0x7ffc;
    local_6 = u_var5 - 2;
    local_4 = 0;
    if ((u_var2 & 0x8000) != 0) {
        bVar6 = local_6 < 4;
        local_6 = u_var5 - 6;
        local_4 = -bVar6;
    }
    return CONCAT22(local_4, local_6);
}

pub fn get_mem_sz_1000_1532(alloc_addr: u32) -> u32 {
    let mut alloc_handle: u32;
    let dVar1: u32;

    alloc_handle = GlobalHandle16(alloc_addr);
    if (alloc_handle != 0) {
        dVar1 = GlobalSize16(alloc_handle);
        return dVar1;
    }
    return 0;
}
