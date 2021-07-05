use crate::app_context::AppContext;
use crate::other_funcs::empty_fn_1000_55ac;
use crate::pass::pass_funcs::pass1_fn_1000_52be;
use crate::structs::prog_structs_28::Struct202;
use crate::structs::prog_structs_2::Struct662;
use crate::structs::prog_structs_6::Struct672;
use crate::structs::prog_structs_9::Struct594;
use crate::ui_ops::window;
use crate::util::SUB42;
use crate::winapi::swi;
use crate::mem_funcs::mem_ops_1::get_fn_ptr_at_address;
use crate::typedefs::HWND16;

pub fn set_fn_ptr_1000_17e8(param_1: Vec<u8>, param_2: Vec<u8>) -> Vec<u8> {
    let pu_var1: Vec<u8>;
    let mut local_4: u16;

    pu_var1 = ctx.func_ptr_1050_5f34;
    ctx.func_ptr_1050_5f34 = param_1;
    ctx.PTR_LOOP_1050_5f36 = param_2;
    return pu_var1;
}

// WARNING: Removing unreachable block (ram,0x10002557)

pub fn call_fn_ptr_1000_24cd(ctx: &mut AppContext, fn_ptr_param_1: Option<&fn()>) -> HWND16 {
    // let pc_var1: &mut  code;
    // let mut unaff_bp: i32;
    let mut i_var2: u16;
    let mut u_var3: u16;
    let mut c_var4: u8;
    let mut u_var5: u16;
    let mut fn_ptr_1: u32;

    i_var2 = ctx.bp + 1;
    // TODO
    // u_var5 = &ctx.g_alloc_addr_1050_1050;
    ctx.PTR_LOOP_1050_5fc9 = 0;
    u_var3 = 0;
    call_fn_ptr_1000_2594(ctx, 0, &ctx.g_alloc_addr_1050_1050, i_var2);
    call_fn_ptr_1000_2594(ctx, 0, None, 0);
    empty_fn_1000_55ac(fn_ptr_param_1);
    call_fn_ptr_1000_2594(u_var3, u_var5, i_var2);
    c_var4 = (u_var3 >> 8) as u8;
    call_fn_ptr_1000_2594(ctx, 0, None, 0);
    call_fn_ptr_1000_256b(ctx);
    if c_var4 == 0 {
        pc_var1 = swi(0x21);
        pc_var1();
    }
}

// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)

pub fn call_fn_ptr_1000_24db(ctx: &mut AppContext) {
    let pc_var1: fn();
    let mut unaff_bp: u16;
    let mut c_var2: char;

    ctx.PTR_LOOP_1050_5fc9 = 0;
    c_var2 = '\0';
    call_fn_ptr_1000_2594(ctx, 1, Some(&ctx.g_alloc_addr_1050_1050), ctx.bp_reg + 1);
    call_fn_ptr_1000_2594(ctx, 0, None, 0);
    call_fn_ptr_1000_256b(ctx);
    if c_var2 == '\0' {
        // DOS API call
        pc_var1 = swi(0x21);
        pc_var1()
    }
}

pub fn call_fn_ptr_1000_256b(ctx: &mut AppContext) {
    let pc_var1: &mut  code;
    let mut fn_ptr_1: u32;

    if ctx.g_fn_ptr_1050_6202 != 0x0 {
        let fn_ptr = get_fn_ptr_at_address(ctx.g_fn_ptr_1050_6200);
    }
    pc_var1 = swi(0x21);
    pc_var1();
}

pub fn call_fn_ptr_1000_2594(ctx: &mut AppContext, a: u16, b: Option<&String>, c: u16) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut func_ptr: u32;

    while {
        if ctx.di_reg <= ctx.si_reg {
            return;
        }
        pu_var2 = unaff_DI + -2;
        pu_var1 = unaff_DI + -1;
        unaff_DI = pu_var2;
        let val = unsafe { *pu_var2 };
        let val2 = unsafe { *pu_var1 };
        (val | val2) == 0
    } {}
    func_ptr = pu_var2;
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

pub fn call_fn_ptr_1000_5586(// in_fn_ptr_1: &mut Vec<u8>,
// in_u16_2: u16,
// in_i16_3: i32,
// in_i16_4: i32,
// in_i16_5: i32,
// in_u16_6: u16,
) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut uStack14: u16;
    let mut iStack10: i32;
    let mut uStack8: u16;

    iVar1 = in_i16_5;
    i_var2 = in_i16_3;
    while (i_var2 = i_var2 + -1, -1 < i_var2) {
        uStack8 = in_u16_6;
        uStack14 = 0x559b;
        iStack10 = iVar1;
        unsafe { (*in_fn_ptr_1)() };
        iVar1 = iVar1 + in_i16_4;
    }
    return;
}

pub fn call_fn_ptr_1008_64a2(param_1: &mut  u32) {
    let mut u_var1: i32;
    let fn_ptr_1: fn() -> Vec<u8>;

    u_var1 = (param_1 + 2);
    unsafe {
        if ((u_var1 | *param_1) != 0) {
            fn_ptr_1 = *param_1;
            fn_ptr_1();
        }
    }
}

pub fn call_fn_ptr_1008_6b2e(param_1: &mut  Struct202) {
    let local_bx_3: &mut  Struct202;
    let mut local_es_3: u16;
    let fn_ptr_1: &mut  Vec<u8>;

  // local_es_3 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    if ((local_bx_3.field_0xd0 | &local_bx_3.field_0xce) != 0) {
        unsafe {
            fn_ptr_1 = (&local_bx_3.field_0xce + 0x68);
            (fn_ptr_1)();
        }
    }
}

pub fn call_fn_ptr_1040_b8be(param_1: &mut  u32) {
    let fn_ptr_1: fn();

    unsafe {
        fn_ptr_1 = (*param_1 + 0x80);
        fn_ptr_1();
    }
}

pub fn call_fn_ptr_1020_0abc(param_1: &mut Vec<u8>) {
    let mut u_var1: u16;
    let fn_ptr_1: fn();

  // u_var1 = (param_1  >> 0x10);
    if ((param_1 + 0xe6) != 0) {
        fn_ptr_1 = ((param_1 + 0xe8) + 0x10);
        (**fn_ptr_1)();
    }
    return;
}

pub fn pass1_1040_746c(param_1: u32) -> u16 {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0x98) + 8);
    (**pp_var1)();
    return 1;
}

pub fn pass1_1040_70a0(param_1: &mut  u32) {
    let pp_var1: fn();
    unsafe {
        pp_var1 = (*param_1 + 0x7c);
        (**pp_var1)();
    }
}

pub fn pass1_1040_692e(param_1: &mut  u32) {
    let pp_var1: fn();
    unsafe {
        pp_var1 = (*param_1 + 0x7c);
        (pp_var1)();
    }
}

pub fn pass1_1038_927c(param_1: &mut  Vec<u8>) {
    let pp_var1: fn();

    pp_var1 = (param_1 + 0x74);
    (**pp_var1)();
    return;
}

pub fn pass1_1038_7a5a(param_1: &mut  Vec<u8>) {
    let pp_var1: fn();

    pp_var1 = (param_1 + 4);
    (**pp_var1)();
    return;
}

pub fn call_fn_ptr_1020_8106(param_1: Vec<u8>) {
    let fn_ptr_1_1: fn();

    fn_ptr_1_1 = ((param_1 + 4) + 0x60);
    (**fn_ptr_1_1)();
    return;
}

pub fn call_fn_ptr_1_1020_6746(param_1: &mut  Struct672, param_2: u16, param_3: u16) {
    let local_struct_1: &mut  Struct672;
    let local_struct_1_hi: &mut  Struct672;
    let mut temp_5f3de8bd2d: u32;
    let fn_ptr_1: fn();

    if (param_3 != 0) {
      // local_struct_1_hi = (param_1  >> 0x10);
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

pub fn call_fn_ptr_1020_44b0(in_struct_1: &mut  Struct662) {
    let local_struct_1: &mut  Struct662;
    let local_struct_1_hi: &mut  Struct662;
    let fn_ptr_1: fn();

  // local_struct_1_hi = (in_struct_1  >> 0x10);
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

pub fn call_fn_ptr_1020_1bb6(param_1: &mut Vec<u8>) -> bool {
    let fn_ptr_1: fn();

    fn_ptr_1 = ((param_1 + 0x92) + 8);
    (**fn_ptr_1)();
    return 0;
}

pub fn call_fn_ptr_1020_26a6(in_struct_1: &mut Struct594) {
    let pu_var1: &mut u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut in_stack_0000fff6: u16;
    let fn_ptr_1: fn();

  // u_var3 = (in_struct_1  >> 0x10);
    pu_var1 = (in_struct_1 + 0xee);
    u_var2 = (in_struct_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    window::destroy_win_1008_628e(in_struct_1, in_stack_0000fff6);
    return;
}