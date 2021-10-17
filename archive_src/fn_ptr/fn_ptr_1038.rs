use crate::{defines::Struct43, file::file_1010::unk_io_op_1010_830a, global::AppContext, win_struct::{HDC16, HWND16}, winapi::{GetDC16, ReleaseDC16}};

pub fn call_fn_ptr_1038_9ffa(
    ctx: &mut AppContext,
    param_1: HWND16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
) -> u16 {
    let ppcVar1: u32;
    let i_var2: &mut Struct43;
    let pu_var2: &mut Struct43;
    let local_4: HDC16;
    let u_var2: u16;

    u_var2 = (param_3 + 0x6);
    local_4 = GetDC16(param_1);
    pu_var2 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x3, param_2);
    i_var2 = pu_var2;
    ppcVar1 = &i_var2.field_0x8;
    (**ppcVar1)(
        0x1010,
        pu_var2,
        (pu_var2 >> 0x10),
        &local_4,
        param_2,
        u_var2,
    );
    ppcVar1 = &i_var2.field_0x4;
    (**ppcVar1)(0x1010, pu_var2, 0x50005, &local_4, param_2);
    ppcVar1 = &i_var2.field_0xc;
    (**ppcVar1)(0x1010, pu_var2, &local_4, param_2);
    ReleaseDC16(0x1010, local_4);
    return 0x0;
}
