
pub fn dos3_calls_1000_5198(mut param_1: u16 ,
                         mut param_2: u16 ,
                         mut param_3: u16 ,
                         mut param_4: u16 ) -> u16
{
    let mut pcVar1 =  swi(0x21);
    (*pcVar1)( 0x1050);
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0) {
        return 0x0;
    }
    pass1_1000_29b5(param_3);
    return param_3 & 0xff;
}


pub fn pass1_1000_5224(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    let mut cVar11: u8;
    let mut uVar9: u16;

    cVar11 =  param_2 < 0x0;
    if ( cVar11) {
        bVar10 = param_1 != 0;
        param_1 = -param_1;
        param_2 = - bVar10 - param_2;
    }
    if ( param_4 < 0x0) {
        cVar11 += '\x01';
        bVar10 = param_3 != 0;
        param_3 = -param_3;
        param_4 = - bVar10 - param_4;
    }
    uVar3 = param_1;
    uVar5 = param_3;
    uVar6 = param_2;
    uVar9 = param_4;
    if (param_4 == 0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (( param_2 %  param_3 << 0x10 |  param_1) /  param_3);
    } else {
        loop {
            uVar8 = uVar9 >> 0x1;
            uVar5 = uVar5 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 1) != 0) << 0xf;
            uVar6 = uVar7;
            uVar9 = uVar8;
            if uVar9 == 0 {
                break;
            }
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar5;
        iVar4 =  uVar1;
        lVar2 =  param_3 * (uVar1 & 0xffff);
        uVar3 =  ( lVar2 >> 0x10);
        uVar5 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0;
    }
    if (cVar11 == '\x01') {
        bVar10 = iVar4 != 0;
        iVar4 = -iVar4;
        uVar3 = - bVar10 - uVar3;
    }
    return CONCAT22(uVar3,
                    iVar4);
}


pub fn pass1_1000_53f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;

    uVar3 = param_1;
    uVar4 = param_4;
    uVar9 = param_2;
    uVar10 = param_3;
    if (param_4 == 0) {
        iVar6 =  (( param_2 %  param_3 << 0x10 |  param_1) %  param_3);
        iVar7 = 0;
    } else {
        loop {
            uVar5 = uVar4 >> 0x1;
            uVar10 = uVar10 >> 0x1 |  ((uVar4 & 1) != 0) << 0xf;
            uVar8 = uVar9 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar4 = uVar5;
            uVar9 = uVar8;
            if uVar5 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar8,
                         uVar3) /  uVar10;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) *  param_3;
        uVar9 =  ( lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar10 = uVar9 + uVar3;
        if (((CARRY2(uVar9,
                     uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
            bVar11 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar10 = (uVar10 - param_4) -  bVar11;
        }
        iVar6 = -(uVar4 - param_1);
        iVar7 = - (uVar4 - param_1 != 0) - ((uVar10 - param_2) -  (uVar4 < param_1));
    }
    return CONCAT22(iVar7,
                    iVar6);
}

pub fn pass1_1000_5512(param_1: *mut u8,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: u16 )
{
    let mut b_var1: bool;
    let mut u_stack4: u16;

    pass1_1000_52be(param_3,
                    param_4,
                    param_5,
                    0x0);
    while (true) {
        b_var1 = param_3 == 0;
        param_3 += -0x1;
        param_4 -= b_var1;
        if (param_4 < 0x0) {
            break;
        }
        u_stack4 = param_2;
        ( param_1)();
    }
}



pub fn pass1_1008_049c(mut param_1: u16, mut param_2: u16, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut puVar2: *mut u8;

    if (param_3.is_null() == false) {
        uVar1 = str_op_1000_3da4(param_3);
        if (uVar1 != 0) {
            puVar2 = pass1_1000_545a(param_3 & 0xffff0000 | (param_3 + 1), s_nomono2_1050_00cc);
            if (puVar2.is_null()) {
                PTR_LOOP_1050_02ec = puVar2;
            }
        }
    }
    return;
}


pub fn pass1_1008_04d2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_9466(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1008_04f8(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1008_0036(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1008_0932() -> u32 {
    let mut uVar1: u32;

    if (_u16_1050_14cc.is_null() == false) {
        pass1_1010_7fd6(_u16_1050_14cc);
    }
    mem_1000_0016(_PTR_LOOP_1050_03a0);
    mem_1000_0016(_PTR_LOOP_1050_029c);
    mem_1000_0016(_PTR_LOOP_1050_4fb8);
    mem_1000_0016(_PTR_LOOP_1050_68a2);
    mem_1000_0016(_PTR_LOOP_1050_5744);
    uVar1 = mem_1000_0016(_PTR_LOOP_1050_5f2c);
    return uVar1;
}


pub fn pass1_1008_0984(mut param_1: i16, mut param_2: u16, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut in_EDX: u32;

    set_sys_color_1008_357e(CONCAT22(param_2, param_1), param_3, in_EDX);
    if ((param_1 + 0xe8) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x98);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1008_0a92(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xee) != 0) {
        ppcVar1 = ((iVar2 + 0xee) + 0x90);
        (**ppcVar1)();
    }
    if ((iVar2 + 0xe8) != 0) {
        ppcVar1 = ((iVar2 + 0xe8) + 0x90);
        (**ppcVar1)();
    }
    if (_PTR_LOOP_1050_0388.is_null() == false) {
        ppcVar1 = *_PTR_LOOP_1050_0388;
        (**ppcVar1)();
    }
    post_quit_msg_1008_3af4();
    return;
}
