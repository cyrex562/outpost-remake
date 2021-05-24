use crate::mem_funcs::{alloc_mem, mem_size};
use crate::pass::pass14_funcs::pass1_1008_6604;
use crate::pass::pass_funcs::pass1_fn_1000_48a8;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_47cc, process_struct_1008_4834, set_struct_1008_4016};
use crate::structs::prog_structs_12::Struct102;
use crate::structs::prog_structs_29::Struct101;
use crate::structs::prog_structs_2::Struct199;
use crate::util::{CONCAT12, CONCAT13, CONCAT22};

pub unsafe fn copy_mem_1008_4274(in_Struct101: &mut  Struct101) {
    let mut u_var1: u32;
    let in_ax: &mut  Struct102;


    let struct_a: &mut  Struct199;

    let local_Struct101: &mut  Struct101;
    let mut a_struct_102: u16;
    let mut u_var2: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var2 = (in_Struct101  >> 0x10);
    local_Struct101 = in_Struct101;
    if (local_Struct101.field_0x6 != 0) {
        mem_size::get_mem_sz_1000_1284(local_Struct101.field_0x6);
        a_struct_102 = in_ax;
        alloc_mem::alloc_mem_1000_0a48(1, in_ax, ctx.dx_reg, ctx.g_struct_ptr_1);
        _local_a = CONCAT22(ctx.dx_reg, a_struct_102);
        struct_a = (ctx.dx_reg | a_struct_102);
        if (struct_a != 0x0) {
            u_var1 = local_Struct101.field_0x6;
            hmemcpy16(
                CONCAT22(in_ax, 0x1000),
                CONCAT13((u_var1 >> 8), CONCAT12(u_var1, ctx.dx_reg)),
            );
            process_struct_1000_179c(0x1e, struct_a);
            if ((struct_a | a_struct_102) == 0) {
                a_struct_102 = 0;
                u_var2 = 0;
            } else {
                set_struct_1008_4016(a_struct_102);
                u_var2 = ctx.dx_reg;
            }
            (a_struct_102 + 6) = _local_a;
            process_struct_1008_47cc(CONCAT22(u_var2, a_struct_102));
            process_struct_1008_4834(CONCAT22(u_var2, a_struct_102));
            (a_struct_102 + 0x1c) = 1;
            return;
        }
    }
    return;
}

pub unsafe fn copy_mem_1008_676e(param_1: u32) {
    let mut u_var1: u32;
    let mut count: u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x6 == 0) {
        return;
    }
    process_struct_1000_179c(0x1e, in_dx);
    u_var2 = in_dx | ctx.ax_reg;
    if (u_var2 == 0) {
        ctx.ax_reg = 0x0;
        u_var2 = 0;
    } else {
        u_var1 = local_bx_4.field_0x10;
      // u_var4 = (u_var1  >> 0x10);
        local_bx_42 = u_var1;
        pass1_1008_6604(
            CONCAT22(in_dx, ctx.ax_reg),
            local_bx_42.field_0x8,
            local_bx_42.field_0x4,
        );
    }
    pass1_fn_1000_48a8(ctx.ax_reg.field_0x10, local_bx_4.field_0x10, 0x28);
    u_var1 = ctx.ax_reg.field_0x10;
    count = (u_var1 + 8) * local_bx_4.field_0x18;
    hmemcpy16(
        CONCAT22(count, 0x1000),
        CONCAT22(local_bx_4.field_0x6, (count >> 0x10)),
    );
    ctx.ax_reg.field_0x1c = 1;
    return;
}
