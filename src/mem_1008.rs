use crate::defines::{Struct_1008_4274, U32Ptr};
use crate::global::AppContext;
use crate::struct_ops::struct_1008::struct_1008_ec72;
use crate::{
    mem_1000::{mem_op_1000_0a48, mem_op_1000_179c},
    pass::{
        pass_1000::{pass1_1000_1284, pass1_1000_48a8},
        pass_1008::{pass1_1008_4016, pass1_1008_47cc, pass1_1008_4834},
    },
    struct_ops::struct_1008::struct_op_1008_6604,
    util::CONCAT22,
    winapi::hmemcpy16,
};

pub fn memcpy_op_1008_4274(ctx: &mut AppContext, param_1: &mut Struct_1008_4274, param_2: u16) {
    let i_var1: u32;
    let pu_var2: Struct_1008_4274;
    let u_var3: u16;
    let i_var4: &mut Struct_1008_4274;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let lVar8: i32;

    // u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (i_var4 + 0x6) != 0x0 {
        uVar7 = pass1_1000_1284(ctx, i_var4.field_0x6, 0x1000);
        i_var1 = uVar7;
        lVar8 = mem_op_1000_0a48(ctx, 0x1, uVar7, ctx._PTR_LOOP_1050_5f2c, 0x1000);
        u_var5 = lVar8;
        pu_var2 = ((lVar8 >> 0x10) | u_var5);
        if (pu_var2 != 0x0) {
            hmemcpy16(
                &ctx.PTR_LOOP_1050_1000,
                uVar7,
                CONCAT22((i_var4 + 0x6), i_var1),
            );
            mem_op_1000_179c(ctx, 0x1e, pu_var2, 0x1000);
            u_var3 = pu_var2 | u_var5;
            if (u_var3 == 0x0) {
                u_var5 = 0x0;
                u_var3 = 0x0;
            } else {
                pass1_1008_4016(ctx, CONCAT22(pu_var2, u_var5));
            }
            (u_var5 + 0x6) = lVar8;
            pass1_1008_47cc(CONCAT22(u_var3, u_var5));
            pass1_1008_4834(CONCAT22(u_var3, u_var5));
            (u_var5 + 0x1c) = 0x1;
            return;
        }
    }
    return;
}

pub fn memcpy_op_1008_676e(ctx: &mut AppContext, param_1: u32, param_2: u16, param_3: U32Ptr) {
    let u_var1: u32;
    let lVar2: U32Ptr;
    let u_var3: u16;
    let i_var4: i16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;

    // u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x6) == 0x0) {
        return;
    }
    mem_op_1000_179c(ctx, 0x1e, param_3, 0x1000);
    u_var3 = param_3 | param_2;
    if (u_var3 == 0x0) {
        param_2 = 0x0;
        u_var3 = 0x0;
    } else {
        u_var1 = (i_var4 + 0x10);
        // uVar7 = (u_var1 >> 0x10);
        iVar5 = u_var1;
        struct_op_1008_6604(CONCAT22(param_3, param_2), (iVar5 + 0x8), (iVar5 + 0x4));
    }
    pass1_1000_48a8((param_2 + 0x10), (i_var4 + 0x10), 0x28);
    u_var1 = (param_2 + 0x10);
    lVar2 = (u_var1 + 0x8) * (i_var4 + 0x18);
    hmemcpy16(
        ctx.PTR_LOOP_1050_1000,
        lVar2,
        CONCAT22((i_var4 + 0x6), (lVar2 >> 0x10)),
    );
    (param_2 + 0x1c) = 0x1;
    return;
}

pub unsafe fn mem_1008_ed1e(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: U32Ptr,
) {
    if (param_3 != 0x0) {
        mem_op_1000_179c(ctx, param_3 << 0x2, param_5, 0x1000);
        return;
    }
    mem_op_1000_179c(ctx, 0x1a, param_5, 0x1000);
    if ((param_5 | param_4) != 0x0) {
        struct_1008_ec72(CONCAT22(param_5, param_4));
        return;
    }
    return;
}
