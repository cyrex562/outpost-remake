use crate::defines::Struct169;

pub fn fn_ptr_1010_905e(param_1: u32, param_2: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct169;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = &i_var4.field_0x4;
    u_var2 = (&i_var4.field_0x4 + 0x2);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    i_var4.field_0x4 = param_2;
    return;
}
