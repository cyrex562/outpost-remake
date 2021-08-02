use crate::defines::{Struct1, Struct19, U32Ptr};
use crate::global::AppContext;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1008::pass1_1008_3e94;
use crate::pass::pass_1010::pass1_1010_6118;
use crate::ui::ui_1040;
use crate::util::CONCAT22;
use crate::win_struct::WNDCLASS16;
use crate::winapi::ShowWindow16;

pub unsafe fn show_win_1040_0766(
    ctx: &mut AppContext,
    struct_1: &mut Struct1,
    param_2: u16,
    struct_2: &mut Struct19,
    unaff_di: i16,
    wndclass_1: &mut WNDCLASS16,
) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let pi_var3: U32Ptr;
    let pi_var4: U32Ptr;
    let u_var5: u16;
    let local_a: i16;
    let local_8: i16;
    let struct_3: &mut Struct19;

    ui_1040::dialog_ui_fn_1040_78e2(struct_1, param_2);
    struct_3 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x2,
        wndclass_1,
        struct_2,
        unaff_di,
    );
    // pu_var1 = (u_stack6 >> 0x10);
    pass1_1010_6118(struct_3);
    pi_var4 = &local_8;
    pi_var3 = &local_a;
    u_var5 = wndclass_1;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x48,
        wndclass_1,
        pu_var1,
        unaff_di,
    );
    pass1_1008_3e94((pu_var2 & 0xffff0000 | (pu_var2 + 0xe)),
                    CONCAT22(wndclass_1, pi_var3), CONCAT22(u_var5, pi_var4));
    ui_1040::move_win_1040_826c(struct_1, local_a + 0x8c);
    ShowWindow16(0x1008, 0x5);
    return;
}
