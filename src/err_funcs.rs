
pub fn  error_check_1000_0dc6(param_1: u16, param_2: u16) -> bool
{
    let mut unaff_CS: u16;
    let mut local_8: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0)
    {
        invoke_error_handler_1000_1e61(unaff_CS, 0xe, 0);
        return 0;
    }
    (**&BYTE_1050_0008)(&g_alloc_addr_1050_1050);
    return 1;
}


pub fn error_check_1000_16ee(uparam_1: i32, uparam_2: i32)

{
    if ((param_2 | param_1) != 0)
    {
        error_check_1000_0dc6(param_1, param_2);
    }
    return;
}


pub fn  error_check_1000_17ce(in_struct_1: *mut Struct44) -> u8

{
    let mut bVar1: u8;
    let BVar2: bool;

    bVar1 = (in_struct_1 >> 0x10) | in_struct_1;
    if (in_struct_1 != 0x0)
    {
        BVar2 = error_check_1000_0dc6(in_struct_1, in_struct_1._2_2_);
        bVar1 = BVar2;
    }
    return bVar1;
}


pub fn error_check_1000_18d2(uparam_1: i32, uparam_2: i32)

{
    if ((param_2 | param_1) != 0)
    {
        error_check_1000_0dc6(param_1, param_2);
    }
    return;
}



pub fn invoke_error_handler_1000_1e61()

{
    _SHI_INVOKEERRORHANDLER1();
    return;
}

pub fn  _SHI_INVOKEERRORHANDLER1() -> u16

{
    let mut uVar1: i32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = SUB42(&g_alloc_addr_1050_1050, 0);
    local_4 = &g_alloc_addr_1050_1050;
    if ((PTR_LOOP_1050_5f1c | PTR_s__1050_1f7e_1050_5f1a) == 0)
    {
        local_6 = 0x0;
        local_4 = 0x0;
    }
    else
    {
        uVar1 = pass1_fn_1000_21b6(PTR_s__1050_1f7e_1050_5f1a, PTR_LOOP_1050_5f1c);
        local_6 = PTR_s__1050_1f7e_1050_5f1a;
        local_4 = PTR_LOOP_1050_5f1c;
        if (uVar1 == 0)
        {
            PTR_s__1050_1f7e_1050_5f1a = (s_554_bmp_1050_1f77 + 7);
            PTR_LOOP_1050_5f1c = &PTR_LOOP_1050_1000;
            local_6 = (s_554_bmp_1050_1f77 + 7);
            local_4 = &PTR_LOOP_1050_1000;
        }
    }
    if ((local_4 | local_6) != 0)
    {
        iVar2 = msg_box_1000_1f24(&PTR_s__1050_1f7e_1050_5f1a, &g_alloc_addr_1050_1050);
        if (iVar2 == 0)
        {
            uVar3 = (*local_6)();
        }
        else
        {
            local_4 = 0;
            local_6 = 0;
            uVar3 = 0;
        }
        if ((local_4 | local_6) != 0)
        {
            check_and_clear_global_1000_1f68(uVar4);
        }
        return uVar3;
    }
    return 0;
}

pub fn handle_error_1008_0036(param_1: *mut u16)

{
    let mut uVar1: i32;
    let puVar2: *mut u32;
    let paVar3: *mut Struct44;
    let in_struct_1: *mut Struct444;
    let mut iVar4: i32;
    let mut local_ES_4: u16;
    let mut local_CS__1: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    void **fn_ptr_1;
    void **fn_ptr_2;
    let mut temp_5f1cd1a162: u32;
    let temp_862db56c250: *mut u32;
    void **fn_ptr_3;

    local_ES_4 = (param_1 >> 0x10);
    iVar4 = param_1;
    unsafe {*param_1 = 0x51e};
    (iVar4 + 2) = &PTR_LOOP_1050_1008;
    temp_5f1cd1a162 = (iVar4 + 8);
    uVar1 = (iVar4 + 10);
    if ((uVar1 | temp_5f1cd1a162) != 0)
    {
        return_1008_53aa(temp_5f1cd1a162, uVar1);
        local_CS__1 = 0x1000;
        error_check_1000_17ce(temp_5f1cd1a162);
    }
    paVar3 = _g_bool_1050_5748;
    _PTR_LOOP_1050_0298 = 0;
    if (_g_bool_1050_5748 != 0x0)
    {
        pass1_1030_8210(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10));
        local_CS__1 = 0x1000;
        error_check_1000_17ce(paVar3);
    }
    paVar3 = _g_Struct372_1050_0ed0;
    if (_g_Struct372_1050_0ed0 != 0x0)
    {
        pass1_1010_2050(_g_Struct372_1050_0ed0, (_g_Struct372_1050_0ed0 >> 0x10));
        local_CS__1 = 0x1000;
        error_check_1000_17ce(paVar3);
    }
    in_struct_1 = _g_struct_73_1050_14cc;
    if (_g_struct_73_1050_14cc != 0x0)
    {
        pass1_1010_7efc(_g_struct_73_1050_14cc);
        local_CS__1 = 0x1000;
        error_check_1000_17ce(in_struct_1);
    }
    paVar3 = _g_Struct112_a;
    if (_g_Struct112_a != 0x0)
    {
        pass1_1038_af34(_g_Struct112_a, (_g_Struct112_a >> 0x10));
        local_CS__1 = 0x1000;
        error_check_1000_17ce(paVar3);
    }
    if (_PTR_LOOP_1050_5bc8 != 0x0)
    {
        fn_ptr_1 = u16_PTR_LOOP_1050_5bc8;
        (**fn_ptr_1)(local_CS__1, _PTR_LOOP_1050_5bc8, (_PTR_LOOP_1050_5bc8 >> 0x10), 1);
    }
    if (_g_struct_ptr_1050_02a0 != 0x0)
    {
        fn_ptr_2 = u16_g_struct_ptr_1050_02a0;
        (**fn_ptr_2)(local_CS__1, _g_struct_ptr_1050_02a0,
                             (_g_struct_ptr_1050_02a0 >> 0x10), 1);
    }
    puVar2 = (iVar4 + 4);
    uVar1 = (iVar4 + 6);
    if ((uVar1 | puVar2) != 0)
    {
        fn_ptr_3 = u16puVar2;
        (**fn_ptr_3)(local_CS__1, puVar2, uVar1, 1);
    }
    handle_error_1008_9466(param_1);
    return;
}

pub fn handle_error_1008_04f8(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    handle_error_1008_0036(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn check_error_1008_087e(param_1: u16, param_2: *mut u16)

{
    let bool_1: u8;
    let mut uVar1: u16;
    let mut local_SS__1: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(10, param_2);
    uVar1 = param_2 | param_1;
    local_6 = param_1;
    local_4 = param_2;
    if (uVar1 != 0)
    {
        bool_1 = pass1_1030_8128(CONCAT22(param_2, param_1));
        param_1 = bool_1;
    }
    if (_g_bool_1050_5748 == 0x0)
    {
        fn_1008_6048(s_New_failed_in_Op__Op__Simulator_1050_0130, uVar1, SUB21(param_1, 0));
        call_fn_ptr_1000_24cd(1);
    }
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
    pass1_1030_532e(CONCAT22(local_SS__1, &local_112), 0xff000000);
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(local_SS__1, &local_112));
    pass1_1030_838e(_g_bool_1050_5748);
    local_112 = s_1_1050_389a;
    local_110 = &PTR_LOOP_1050_1008;
    pass1_1030_8334();
    return;
}

pub fn error_check_1008_3a7a(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    pass1_1008_397a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn error_check_1008_5fa2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    modify_u16_list_1008_5c34(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn error_check_1008_8e74(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    pass1_1008_8aa2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn handle_error_1008_9466(param_1: *mut u16)

{
    unsafe {*param_1 = 0x52a};
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    error_check_1000_17ce(_PTR_LOOP_1050_0392);
    _PTR_LOOP_1050_0392 = 0x0;
    return;
}

pub fn error_check_1008_ad64(param_1: u32, param_2: u8)

{
    pass1_1008_a086(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn set_error_mode_1010_8b14(param_1: u32, param_2: *mut libc::c_char) -> *mut libc::c_char

{
    let mut mode: u16;
    let mut uVar1: i32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_3c: u16;
    let mut local_3a: [u8;44];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;8];

    uVar3 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0xe84));
    mode = SetErrorMode16(1);
    while
    {
        _local_e = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (_local_e == 0)
        {
            SetErrorMode16(mode);
            return param_2;
        }
        uVar1 = param_1 + 0xa82;
        copy_string_1000_3d3e((param_1 & 0xffff0000 | uVar1), *(_local_e + 4));
        process_string_1000_3cea((param_1 & 0xffff0000 | uVar1), param_2);
        iVar2 = dos3_call_1000_51aa(uVar1, uVar3, 1, local_3a);
        iVar2 != 0
    } {}
    SetErrorMode16(mode);
    return (param_1 & 0xffff0000 | uVar1);
}


pub fn 
set_error_mode_1010_85be(param_1: u32, in_struct_1: *mut Struct13, in_struct_1_hi: *mut Struct13)

{
    let mut u_var1: u32;
    let mut unaff_SS: u16;
    let mut local_30e: u16;
    let mut local_30c: u16;
    let mut string_1: [u8;256];
    let mut string_2: [u8;256];
    let mut string_3: [u8;260];
    let mut local_6: u16;
    let mut local_4: u16;

    if (in_struct_1 == &dos_alloc_addr_1050_0002)
    {
        uVar1 = (in_struct_1_hi * 4 + 0x2e34);
        process_string_1000_4d58(uVar1 & 0xffff0000 | (uVar1 + 3), 0, 0);
        copy_string_1000_3d3e(CONCAT22(unaff_SS, string_3), s_male_1050_14c6);
        process_string_1000_3cea(CONCAT22(unaff_SS, string_3), CONCAT22(unaff_SS, string_2));
        process_string_1000_3cea(CONCAT22(unaff_SS, string_3), CONCAT22(unaff_SS, string_1));
        set_error_mode_1010_8b14(param_1, string_3, unaff_SS);
        return;
    }
    set_error_mode_1010_8b14(param_1, (in_struct_1_hi * 4 + 0x2e34));
    return;
}


pub fn error_check_1040_a582(param_1: *mut u32)

{
    let param_1_val = unsafe{*param_1};
    error_check_1000_17ce(param_1_val);
    return;
}

pub fn cleanup_1040_a4c2(param_1: *mut Struct348, param_2: u8) -> *mut Struct348

{
    win_cleanup_fn_1040_a294(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn error_check_1040_8db6(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    error_check_1040_869a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn error_check_1040_869a(param_1: *mut Struct363)

{
    let local_BX_4: *mut Struct363;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0x8ddc;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1040;
    error_check_1000_17ce(local_BX_4.field_0x90);
    error_check_1000_17ce(local_BX_4.field_0x94);
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1030_e4be(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_e282(param_1: *mut Struct44, param_2: u8) -> *mut Struct44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1030_4538(param_1: *mut *mut Struct44)

{
    let mut uVar1: u16;

    let param_1_val = unsafe{*param_1};

    error_check_1000_17ce(param_1_val);
    uVar1 = (param_1 >> 0x10);
    error_check_1000_17ce((param_1 + 0x12));
    error_check_1000_17ce((param_1 + 0x15c));
    return;
}
