use crate::defines::U32Ptr;

pub fn fn_ptr_1028_d566(param_1: U32Ptr, param_2: U32Ptr) -> bool {
    code * *ppcVar1;
    let iVar2: i16;
    let uVar3: u16;

    ppcVar1 = (*param_2 + 0x8);
    iVar2 = (**ppcVar1)();
    if (iVar2 != 0x0) {
        uVar3 = fn_ptr_1028_d742(*param_1, param_2);
        if (uVar3 != 0x0) {
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
    let uVar2: u32;

    ppcVar1 = (*param_2 + 0xc);
    uVar2 = (**ppcVar1)();
    pass1_1020_c872((param_1 + 0x4), (uVar2 + 0x4), uVar2);
    return 0x1;
}


pub fn fn_ptr_1030_835a(param_1: U32Ptr, param_2: U32Ptr) {
    fn_ptr_1028_d566(param_1, param_2);
    return;
}
