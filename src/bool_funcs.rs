
uint  check_flag_1000_1ab0(param_1: u16)

{
    let mut in_AX: i32;
    let mut u_var1: i32;
    let mut u_var2: i32;

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
        u_var1 = 0x2000;
        if (in_AX < (s_571_bmp_1050_1fff + 2))
        {
            do
            {
                u_var2 = u_var1;
                u_var1 = u_var2 >> 1;
            } while (in_AX <= u_var1);
            return u_var2 & 0xfffe;
        }
        while (u_var1 = u_var1 * 2, u_var1 != 0)
        {
            if (in_AX <= u_var1)
            {
                return (u_var1 + 0x10 & -(u_var1 < 0xfff0)) - 0x10;
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
    let pu_var1: *mut u8;
    let mut b_var2: bool;

    pu_var1 = PTR_LOOP_1050_5f22 + 1;
    b_var2 = PTR_LOOP_1050_5f22 == 0x0;
    PTR_LOOP_1050_5f22 = pu_var1;
    if ((b_var2) && ((g_alloc_addr_1050_5f20 | g_astruct_94_1050_5f1e) != 0))
    {
        PTR_LOOP_1050_5f22 = &dos_alloc_addr_1050_0002;
    }
    return 1;
}
