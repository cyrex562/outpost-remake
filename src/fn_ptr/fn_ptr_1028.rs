use crate::defines::U32Ptr;

pub fn fn_ptr_1028_d566(param_1: U32Ptr, param_2: U32Ptr) -> bool {
    code * *ppcVar1;
    let i_var2: i16;
    let u_var3: u16;

    ppcVar1 = (*param_2 + 0x8);
    i_var2 = (**ppcVar1)();
    if (i_var2 != 0x0) {
        u_var3 = fn_ptr_1028_d742(*param_1, param_2);
        if (u_var3 != 0x0) {
            return 0x1;
        }
    }
    return 0x0;
}

pub fn fn_ptr_1028_d728(param_1: u32) {
    code * *ppcVar1;

    ppcVar1 = ((param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}

pub fn fn_ptr_1028_d742(param_1: u32, param_2: U32Ptr) -> u16 {
    code * *ppcVar1;
    let u_var2: u32;

    ppcVar1 = (*param_2 + 0xc);
    u_var2 = (**ppcVar1)();
    pass1_1020_c872((param_1 + 0x4), (u_var2 + 0x4), u_var2);
    return 0x1;
}


pub fn fn_ptr_1030_835a(param_1: U32Ptr, param_2: U32Ptr) {
    fn_ptr_1028_d566(param_1, param_2);
    return;
}
