
uint  check_flag_1000_1ab0(param_1: u16)

{
    let mut in_AX: i32;
    let mut uVar1: i32;
    let mut uVar2: i32;

    if (in_AX == 0x2000)
    {
        return 0x2000;
    }
    if (in_AX < 0xfff0)
    {
        if (in_AX < 0x1001)
        {
            return 0x1000;
        }
        uVar1 = 0x2000;
        if (in_AX < (s_571_bmp_1050_1fff + 2))
        {
            do
            {
                uVar2 = uVar1;
                uVar1 = uVar2 >> 1;
            } while (in_AX <= uVar1);
            return uVar2 & 0xfffe;
        }
        while (uVar1 = uVar1 * 2, uVar1 != 0)
        {
            if (in_AX <= uVar1)
            {
                return (uVar1 + 0x10 & -(uVar1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}


pub fn check_and_clear_global_1000_1f68()

{
    PTR_LOOP_1050_5f26 = PTR_LOOP_1050_5f26 + -1;
    if (PTR_LOOP_1050_5f26 < 0)
    {
        PTR_LOOP_1050_5f26 = 0x0;
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10002018)

u16  check_and_set_global_1000_1fea()

{
    let puVar1: *mut u8;
    let mut bVar2: bool;

    puVar1 = PTR_LOOP_1050_5f22 + 1;
    bVar2 = PTR_LOOP_1050_5f22 == 0x0;
    PTR_LOOP_1050_5f22 = puVar1;
    if ((bVar2) && ((g_alloc_addr_1050_5f20 | g_astruct_94_1050_5f1e) != 0))
    {
        PTR_LOOP_1050_5f22 = &dos_alloc_addr_1050_0002;
    }
    return 1;
}
