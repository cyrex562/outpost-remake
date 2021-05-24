pub fn unlock_mem_1000_3cd8(param_1: u16, param_2: u16) {
    unlock_mem_1000_407a(param_1, param_2);
    return;
}

pub fn unlock_mem_1000_407a(a: u16, b: u16) {
    GlobalFree16(&ctx.g_alloc_addr_1050_1050);
    return;
}
