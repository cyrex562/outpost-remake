pub fn set_fn_ptr_1000_17e8(param_1: *mut u8, param_2: *mut u8) -> *mut u8 {
    let puVar1: *mut u8;
    let mut local_4: u16;

    puVar1 = func_ptr_1050_5f34;
    func_ptr_1050_5f34 = param_1;
    PTR_LOOP_1050_5f36 = param_2;
    return puVar1;
}

// WARNING: Removing unreachable block (ram,0x10002557)

pub fn call_fn_ptr_1000_24cd(a: u16) {
    let pcVar1: *mut code;
    let mut unaff_BP: i32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut cVar4: u8;
    let mut uVar5: u16;
    let mut fn_ptr_1: u32;

    iVar2 = unaff_BP + 1;
    uVar5 = SUB42(&g_alloc_addr_1050_1050, 0);
    PTR_LOOP_1050_5fc9._0_1_ = 0;
    uVar3 = 0;
    call_fn_ptr_1000_2594(0, &g_alloc_addr_1050_1050, iVar2);
    call_fn_ptr_1000_2594();
    empty_fn_1000_55ac(a);
    call_fn_ptr_1000_2594(uVar3, uVar5, iVar2);
    cVar4 = (uVar3 >> 8);
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if (cVar4 == '\0') {
        unsafe {
            pcVar1 = swi(0x21);
            (*pcVar1)();
        }
    }
}

// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)

pub fn call_fn_ptr_1000_24db() {
    let pcVar1: *mut fn();
    let mut unaff_BP: i32;
    let mut cVar2: u8;

    PTR_LOOP_1050_5fc9._0_1_ = 0;
    cVar2 = '\0';
    call_fn_ptr_1000_2594(1, &g_alloc_addr_1050_1050, unaff_BP + 1);
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if (cVar2 == '\0') {
        unsafe {
            pcVar1 = swi(0x21);
            (*pcVar1)();
        }
    }
}

// WARNING: Removing unreachable block (ram,0x10002589)

pub fn call_fn_ptr_1000_256b() {
    let pcVar1: *mut code;
    let mut fn_ptr_1: u32;

    if (g_fn_ptr_1050_6202 != 0x0) {
        (*g_fn_ptr_1050_6200)();
    }
    pcVar1 = swi(0x21);
    unsafe {
        (*pcVar1)();
    }
}

pub fn call_fn_ptr_1000_2594() {
    let puVar1: *mut u32;
    let unaff_SI: *mut u32;
    let unaff_DI: *mut u32;
    let puVar2: *mut u32;
    let mut func_ptr: u32;

    while {
        if (unaff_DI <= unaff_SI) {
            return;
        }
        puVar2 = unaff_DI + -2;
        puVar1 = unaff_DI + -1;
        unaff_DI = puVar2;
        let val = unsafe { *puVar2 };
        let val2 = unsafe { *puVar1 };
        (val | val2) == 0
    } {}
    func_ptr = puVar2;
    (**func_ptr)();
    call_fn_ptr_1000_2594();
}

pub fn call_fn_ptr_in_loop_1000_54e8(// in_fn_ptr_1: u16,
// param_2: u16,
// param_2: u16,
// param_4: u16,
// param_5: u16,
// param_6: u16,
) {
    let mut i16_1: u16;
    let mut i16_2: u16;
    let mut local_e: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;

    i16_2 = param_5 + param_4 * param_2_00;
    i16_1 = param_2_00;
    while (i16_1 = i16_1 - 1, -1 < i16_1) {
        i16_2 = i16_2 - param_4;
        uStack8 = param_6;
        local_e = 0x5506;
        uStack10 = i16_2;
        (*in_fn_ptr_1)();
    }
}

pub fn call_fn_ptr_1000_5512(// in_fn_ptr_1: u16,
// in_u16_2: u16,
// in_i16_3: u16,
// in_i16_4: u16,
// in_u16_5: u16,
) {
    let mut bVar1: bool;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_fn_1000_52be(in_i16_3, in_i16_4, in_u16_5, 0);
    while (true) {
        bVar1 = in_i16_3 == 0;
        in_i16_3 = in_i16_3 - 1;
        in_i16_4 = in_i16_4 - bVar1;
        if (in_i16_4 < 0) {
            break;
        }

        local_4 = in_u16_2;
        (*in_fn_ptr_1)();
    }
    return;
}

pub fn call_fn_ptr_1000_5586(// in_fn_ptr_1: *mut void,
// in_u16_2: u16,
// in_i16_3: i32,
// in_i16_4: i32,
// in_i16_5: i32,
// in_u16_6: u16,
) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uStack14: u16;
    let mut iStack10: i32;
    let mut uStack8: u16;

    iVar1 = in_i16_5;
    iVar2 = in_i16_3;
    while (iVar2 = iVar2 + -1, -1 < iVar2) {
        uStack8 = in_u16_6;
        uStack14 = 0x559b;
        iStack10 = iVar1;
        unsafe { (*in_fn_ptr_1)() };
        iVar1 = iVar1 + in_i16_4;
    }
    return;
}

pub fn call_fn_ptr_1008_64a2(param_1: *mut u32) {
    let mut uVar1: i32;
    let fn_ptr_1: fn() -> *mut u8;

    uVar1 = (param_1 + 2);
    unsafe {
        if ((uVar1 | *param_1) != 0) {
            fn_ptr_1 = *param_1;
            fn_ptr_1();
        }
    }
}

pub fn call_fn_ptr_1008_6b2e(param_1: *mut astruct_202) {
    let local_BX_3: *mut astruct_202;
    let mut local_ES_3: u16;
    let fn_ptr_1: *mut *mut u8;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((local_BX_3.field_0xd0 | &local_BX_3.field_0xce) != 0) {
        unsafe {
            fn_ptr_1 = (&local_BX_3.field_0xce + 0x68);
            (fn_ptr_1)();
        }
    }
}

pub fn call_fn_ptr_1040_b8be(param_1: *mut u32) {
    let fn_ptr_1: fn();

    unsafe {
        fn_ptr_1 = (*param_1 + 0x80);
        fn_ptr_1();
    }
}

pub fn call_fn_ptr_1020_0abc(param_1: *mut void) {
    let mut uVar1: u16;
    let fn_ptr_1: fn();

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0xe6) != 0) {
        fn_ptr_1 = ((param_1 + 0xe8) + 0x10);
        (**fn_ptr_1)();
    }
    return;
}

pub fn pass1_1040_746c(param_1: u32) -> u16 {
    let ppcVar1: fn();

    ppcVar1 = ((param_1 + 0x98) + 8);
    (**ppcVar1)();
    return 1;
}

pub fn pass1_1040_70a0(param_1: *mut u32) {
    let ppcVar1: fn();
    unsafe {
        ppcVar1 = (*param_1 + 0x7c);
        (**ppcVar1)();
    }
}

pub fn pass1_1040_692e(param_1: *mut u32) {
    let ppcVar1: fn();
    unsafe {
        ppcVar1 = (*param_1 + 0x7c);
        (ppcVar1)();
    }
}

pub fn pass1_1038_927c(param_1: *mut *mut u8) {
    let ppcVar1: fn();

    ppcVar1 = (param_1 + 0x74);
    (**ppcVar1)();
    return;
}

pub fn pass1_1038_7a5a(param_1: *mut *mut u8) {
    let ppcVar1: fn();

    ppcVar1 = (param_1 + 4);
    (**ppcVar1)();
    return;
}

pub fn call_fn_ptr_1020_8106(param_1: *mut u8) {
    let fn_ptr_1_1: fn();

    fn_ptr_1_1 = ((param_1 + 4) + 0x60);
    (**fn_ptr_1_1)();
    return;
}

pub fn call_fn_ptr_1_1020_6746(param_1: *mut astruct_672, param_2: u16, param_3: u16) {
    let local_struct_1: *mut astruct_672;
    let local_struct_1_hi: *mut astruct_672;
    let mut temp_5f3de8bd2d: u32;
    let fn_ptr_1: fn();

    if (param_3 != 0) {
        local_struct_1_hi = (param_1 >> 0x10);
        local_struct_1 = param_1;
        if ((&local_struct_1[1].field_0x0 + param_3 * 4) != 0) {
            temp_5f3de8bd2d = (&local_struct_1[1].field_0x0 + param_3 * 4);
            (temp_5f3de8bd2d + 4) = param_2;
            local_struct_1.field_0x10 = 1;
            if (param_2 == 0) {
                fn_ptr_1 = ((&local_struct_1[1].field_0x0 + param_3 * 4) + 0x14);
                (**fn_ptr_1)();
            }
        }
    }
    return;
}

pub fn call_fn_ptr_1020_44b0(in_struct_1: *mut astruct_662) {
    let local_struct_1: *mut astruct_662;
    let local_struct_1_hi: *mut astruct_662;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0xf6 != 0) {
        fn_ptr_1 = (in_struct_1 + 0x98);
        (**fn_ptr_1)();
        local_struct_1.field_0x112 = 0;
        fn_ptr_1 = (local_struct_1.field_0xf6 + 8);
        (**fn_ptr_1)();
    }
    return;
}

pub fn  call_fn_ptr_1020_1bb6(param_1: *mut void) -> bool

{
    let fn_ptr_1: fn();

    fn_ptr_1 = ((param_1 + 0x92) + 8);
    (**fn_ptr_1)();
    return 0;
}