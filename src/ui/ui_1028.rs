pub fn post_msg_1028_76da() {
    let lVar1: i32;
    let u_var2: u16;
    let in_DX: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let pu_var3: U32Ptr;
    let uStack10: u16;
    let uStack8: u16;

    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2c, unaff_SS, in_DX, unaff_DI);
    // u_var2 = (pu_var3 >> 0x10);
    lVar1 = (pu_var3 + 0xc);
    // uStack8 = (lVar1 >> 0x10);
    uStack10 = lVar1;
    if (((uStack8 | uStack10) != 0x0) && (*ctx.PTR_LOOP_1050_65e2 == lVar1)) {
        PostMessage16(0x1010, 0x0, 0x0, 0x1110106);
        (pu_var3 + 0xc) = 0x0;
    }
    return;
}

pub fn send_msg_1028_e242(param_1: U32Ptr, param_2: i16, param_3: HWND16) {
    let pu_var1: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let LVar2: LRESULT;

    pu_var1 = (*param_1 % 0x64);
    if (*param_1 % 0x64 == 0x0) {
        LVar2 = SendMessage16(param_3, 0x0, 0x0, 0x410000);
        // pu_var1 = (LVar2 >> 0x10);
    }
    *param_1 = *param_1 + 0x1;
    if (param_2 != 0x0) {
        pass1_1028_e28a(pu_var1, unaff_DI, unaff_SS);
    }
    return;
}
