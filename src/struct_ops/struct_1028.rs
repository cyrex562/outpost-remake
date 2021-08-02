use crate::defines::{U32Ptr, Struct183};
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1028_d566;
use crate::mem_1000::mem_op_1000_179c;
use crate::pass::pass_1000::pass1_1000_4906;
use crate::pass::pass_1020::pass1_1020_ba3e;
use crate::pass::pass_1028::{pass1_1028_3fa2, pass1_1028_d6b2, struct_1028_b354};
use crate::struct_ops::struct_1008::set_struct_1008_574a;
use crate::struct_ops::struct_1030::struct_1030_dc96;
use crate::sys_api::sys_1000_3f9c;
use crate::util::{CONCAT22, ZEXT24};

pub fn struct_1028_0068(struct_2: &mut Struct183, param_2: U32Ptr, extraout_dx: u16) {
    let u_var1: u16;
    // let extraout_dx: u16;
    let struct_1: &mut Struct183;

    struct_1028_b354(struct_2);
    // u_var2 = (param_1 >> 0x10);
    struct_1 = struct_2;
    u_var1 = 0x0;
    struct_1.field_0x20 = 0x0;
    &struct_1.field_0x22 = 0x0;
    *struct_2 = 0x8ec;
    struct_1.field_0x2 = &USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    if ((param_2 | u_var1) == 0x0) {
        &struct_1.field_0x22 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(param_2, u_var1));
        struct_1.field_0x22 = u_var1;
        struct_1.field_0x24 = extraout_dx;
    }
    return;
}


pub fn struct_1028_0954(param_1: U32Ptr) -> u16

{
    let i_var1: &mut Struct185;
    let u_var1: u16;

    struct_1028_b354(param_1);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x20 = 0x0;
    *param_1 = 0xada;
    i_var1.field_0x2 = &USHORT_1050_1028;
    i_var1.field_0xe = 0x4b;
    return param_1;
}


pub fn struct_1028_0b42(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0xbbc;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_178c(param_1: U32Ptr) -> u16

{
    struct_1030_dc96(param_1);
    *param_1 = 0x1b54;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_1bbc(param_1: U32Ptr) -> u16

{
    let i_var1: &mut Struct190;
    let u_var1: u16;

    struct_1028_b354(param_1);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x20 = 0x0;
    i_var1.field_0x22 = 0x0;
    *param_1 = 0x1eee;
    i_var1.field_0x2 = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_1f56(param_1: U32Ptr, param_2: U32Ptr) {
    let u_var1: u32;
    let u_var2: u16;
    let extraout_dx: u16;
    let i_var3: &mut Struct186;
    let u_var3: u16;

    struct_1028_b354(param_1);
    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = 0x0;
    &i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    *param_1 = 0x2572;
    i_var3.field_0x2 = &USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    if ((param_2 | u_var2) == 0x0) {
        &i_var3.field_0x20 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(param_2, u_var2));
        i_var3.field_0x20 = u_var2;
        i_var3.field_0x22 = extraout_dx;
    }
    u_var1 = &i_var3.field_0x20;
    (u_var1 + 0xa) = 0x0;
    return;
}


pub fn struct_1028_25da(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = (s_fem16_wav_1050_2644 + 0x8);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_26b4(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x2788;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_27f0(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x2a92;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_2bdc(param_1: U32Ptr) -> u16

{
    struct_1028_0954(param_1);
    *param_1 = 0x341c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_355e(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x3608;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_37a6(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct189;
    let u_var3: u16;

    struct_1028_b354(param_1);
    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = 0x0;
    i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    &i_var3.field_0x28 = 0x0;
    *param_1 = 0x3e2c;
    i_var3.field_0x2 = &USHORT_1050_1028;
    mem_op_1000_179c(0xa, param_2, 0x1000);
    u_var2 = param_2 | u_var1;
    if (u_var2 == 0x0) {
        &i_var3.field_0x28 = 0x0;
    } else {
        pass1_1020_ba3e(CONCAT22(param_2, u_var1), 0x5, 0x5, param_4, param_3);
        i_var3.field_0x28 = u_var1;
        i_var3.field_0x2a = u_var2;
    }
    return;
}


pub fn struct_1028_3e94(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    struct_1028_b354(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1 = 0x4004;
    (param_1 + 0x2) = &USHORT_1050_1028;
    pass1_1028_3fa2(param_1 & 0xffff | u_var1 << 0x10);
    return param_1;
}


pub fn struct_1028_406c(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x42ec;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_4354(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x446a;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_44d2(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    struct_1028_b354(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1 = 0x4836;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_489e(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = &ctx.PTR_LOOP_1050_4942;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_49aa(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x4b1c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    pass1_1000_4906(
        (param_1 & 0xffff0000 | (param_1 + 0x20)),
        0x0, 0xa);
    return param_1;
}


pub fn struct_1028_4b84(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = (s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_50d8(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x5280;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_52e8(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x535e;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_53c6(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x54bc;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_5630(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x56ac;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_57a6(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = 0x581c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_5966(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = (s_mineToSmelter__no_mines_1050_59df + 0x1);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_5a48(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = s_thisLo_1050_5bec;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_5c54(param_1: U32Ptr) -> u16

{
    struct_1028_b354(param_1);
    *param_1 = (s_static_1050_5d8b + 0x3);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_5ed8(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    struct_1028_b354(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1 = 0x6054;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


pub fn struct_1028_60bc(param_1: U32Ptr, param_2: u16, param_3: U32Ptr) -> u16

{
    let u_var1: u32;
    let u_var2: u16;
    let i_var2: &mut Struct187;

    i_var2 = param_1;
    // u_var2 = (param_1 >> 0x10);
    struct_1028_b354(param_1);
    &i_var2.field_0x20 = 0x0;
    *param_1 = 0x6876;
    i_var2.field_0x2 = &USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_3, 0x1000);
    if ((param_3 | param_2) == 0x0) {
        &i_var2.field_0x20 = 0x0;
    } else {
        u_var1 = set_struct_1008_574a(CONCAT22(param_3, param_2));
        i_var2.field_0x20 = u_var1;
        i_var2.field_0x22 = (u_var1 >> 0x10);
    }
    return param_1;
}


pub fn struct_op_1028_87f0(param_1: u16, param_2: u8, param_3: &mut Struct97, param_4: u16,
                           param_5: u16, param_6: u16, param_7: U32Ptr, param_8: u16,
                           param_9: u32, param_10: u32)

{
    let i_var1: &mut Struct97;
    let pu_var1: &mut Struct97;

    struct_op_1028_d1dc(param_1, param_2, param_3, 0x3e8);
    // pu_var1 = (param_3 >> 0x10);
    i_var1 = param_3;
    i_var1.field_0x108 = param_10;
    i_var1.field_0x10c = param_9;
    i_var1.field_0x110 = 0x0;
    i_var1.field_0x114 = *param_7;
    i_var1.field_0x118 = (param_7 + 0x1);
    i_var1.field_0x11a = param_6;
    i_var1.field_0x11c = param_5;
    i_var1.field_0x11e = param_4;
    i_var1.field_0x122 = 0x0;
    i_var1.field_0x120 = 0x0;
    param_3 = 0x8d8e;
    i_var1.field_0x2 = &USHORT_1050_1028;
    sys_1000_3f9c(&i_var1.field_0x8, pu_var1,
                  s_SCInternalPutBldg_site_0x_08lx__b_1050_5046,
                  ctx.data_seg, param_10, &stack0xfffe, pu_var1, 0x1000,
                  param_1, param_2);
    return;
}


pub fn struct_op_1028_8888(param_1: u16, param_2: u8, param_3: &mut Struct100, param_4: u16,
                           param_5: u16, param_6: U32Ptr, param_7: u16, param_8: u32,
                           param_9: u32, param_1: u320)

{
    let i_var1: &mut Struct100;
    let pu_var1: U32Ptr;

    struct_op_1028_d1dc(param_1, param_2, param_3, 0x3e8);
    // pu_var1 = (param_3 >> 0x10);
    i_var1 = param_3;
    i_var1.field_0x108 = param_10;
    i_var1.field_0x10c = param_9;
    i_var1.field_0x110 = param_8;
    i_var1.field_0x114 = *param_6;
    i_var1.field_0x118 = (param_6 + 0x1);
    i_var1.field_0x11a = param_5;
    i_var1.field_0x11c = 0x0;
    i_var1.field_0x11e = param_4;
    i_var1.field_0x122 = 0x0;
    i_var1.field_0x120 = 0x0;
    param_3.field_0x0 = 0x8d8e;
    i_var1.field_0x2 = &USHORT_1050_1028;
    sys_1000_3f9c(&i_var1.field_0x8, pu_var1,
                  s_SCInternalPutBldg2_site_0x_08lx__1050_506f,
                  ctx.data_seg, param_10, &stack0xfffe, pu_var1, 0x1000,
                  param_1, param_2);
    return;
}


pub fn struct_op_1028_933c(param_1: &mut Struct100, param_2: u16, param_3: u16, param_4: u16,
                           param_5: U32Ptr, param_6: u16, param_7: u32, param_8: u32,
                           param_9: u16, param_10: u8)

{
    let i_var1: i16;
    let pu_var2: U32Ptr;

    struct_op_1028_d1dc(param_9, param_10, param_1, 0x3e8);
    // pu_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x108) = param_8;
    (i_var1 + 0x10c) = param_7;
    (i_var1 + 0x110) = 0x0;
    (i_var1 + 0x114) = *param_5;
    (i_var1 + 0x118) = (param_5 + 0x1);
    (i_var1 + 0x11a) = param_4;
    (i_var1 + 0x11c) = param_2;
    (i_var1 + 0x120) = 0x0;
    (i_var1 + 0x11e) = 0x0;
    (i_var1 + 0x122) = param_3;
    param_1.field_0x0 = 0x9934;
    (i_var1 + 0x2) = &USHORT_1050_1028;
    sys_1000_3f9c((i_var1 + 0x8), pu_var2,
                  s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
                  ctx.data_seg, param_8, &stack0xfffe, pu_var2, 0x1000,
                  param_9, param_10);
    return;
}


pub fn struct_1028_9c62(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8) -> u16

{
    struct_op_1028_d1dc(param_4, param_5, CONCAT22(param_2, param_1), param_3);
    (param_1 + 0x108) = param_3;
    CONCAT22(param_2, param_1) = 0x9eb6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}


pub fn struct_op_1028_d1dc(
    param_1: u16,
    param_2: u8,
    param_3: &mut Struct100,
    param_4: u16,
    in_stack_0000fffa: u16
) -> &mut Struct100

{
    let i_var1: &mut Struct101;
    let pu_var1: U32Ptr;

    // pu_var1 = (param_3 >> 0x10);
    i_var1 = param_3;
    param_3.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x4 = param_4;
    i_var1.field_0x6 = 0x0;
    param_3.field_0x0 = 0x6ad2;
    i_var1.field_0x2 = &USHORT_1050_1028;
    sys_1000_3f9c(&i_var1.field_0x8, pu_var1, 0x5160, ctx.data_seg,
                  in_stack_0000fffa, &stack0xfffe, pu_var1, 0x1000, param_1, param_2);
    return param_3;
}


pub fn struct_1028_d22e(param_1: U32Ptr, param_2: u32, param_3: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    *param_1 = 0x0;
    (param_1 + 0x4) = param_2;
    mem_op_1000_179c(0xc, param_3, 0x1000);
    u_var1 = param_2;
    pu_var2 = (param_3 | u_var1);
    if (pu_var2 == 0x0) {
        *param_1 = 0x0;
    } else {
        struct_1028_d59c((param_2 & 0xffff | param_3 << 0x10), pu_var2);
        param_1 = u_var1;
        (param_1 + 0x2) = pu_var2;
    }
    return;
}


pub fn struct_1028_d2b0(param_1: U32Ptr, param_2: u16, param_3: u8) {
    let local_10c: u16;
    let uStack266: u16;

    struct_1028_9c62(&local_10c, param_2, 0x3e80, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x3a98, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x36b0, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x32c8, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x2ee0, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x2af8, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x2710, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, (s_noth_bmp_1050_2321 + 0x7), param_2,
                     param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x1f40, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x1b58, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x1770, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x1388, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0xfa0, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0xbb8, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    struct_1028_9c62(&local_10c, param_2, 0x3e8, param_2, param_3);
    fn_ptr_1028_d566(param_1, CONCAT22(param_2, &local_10c));
    local_10c = 0x389a;
    uStack266 = 0x1008;
    pass1_1028_d6b2(*param_1);
    return;
}


pub fn struct_1028_d59c(param_1: U32Ptr, param_2: U32Ptr) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let extraout_dx: U32Ptr;
    let iVar5: &mut Struct158;
    let u_var5: u16;
    let puStack14: U32Ptr;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x0;
    iVar5.field_0x4 = 0x0;
    iVar5.field_0x8 = 0x0;
    pu_var3 = *_PTR_LOOP_1050_5748;
    *param_1 = pu_var3;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    pu_var1 = (pu_var3 & 0xffff | ZEXT24(param_2) << 0x10);
    puVar4 = (param_2 | pu_var3);
    if (puVar4 == 0x0) {
        iVar5.field_0x4 = 0x0;
    } else {
        set_struct_1008_574a((pu_var3 & 0xffff | ZEXT24(param_2) << 0x10));
        *pu_var1 = 0xd804;
        (pu_var3 + 0x2) = &USHORT_1050_1028;
        iVar5.field_0x4 = pu_var1;
        pu_var3 = pu_var1;
        puVar4 = extraout_dx;
    }
    u_var2 = pu_var3;
    mem_op_1000_179c(0xc, puVar4, 0x1000);
    puStack14 = CONCAT22(puVar4, u_var2);
    if ((puVar4 | u_var2) == 0x0) {
        iVar5.field_0x8 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar4, u_var2));
        *puStack14 = 0xd804;
        (u_var2 + 0x2) = &USHORT_1050_1028;
        iVar5.field_0x8 = puStack14;
    }
    return;
}

