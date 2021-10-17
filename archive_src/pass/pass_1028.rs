use crate::bad::bad_1030_1312;
use crate::defines::{
    Struct18, Struct_1028_00cc, Struct_1028_0138, Struct_1028_0176, Struct_1028_b514, U32Ptr,
};
use crate::file::file_1008::{read_file_1008_7dee, write_to_file_1008_7cac};
use crate::file::file_1028::{file_1028_b81a, write_to_file_1028_b5ec};
use crate::file::file_1030::file_1030_1730;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1020::fn_ptr_1020_ba7e;
use crate::fn_ptr::fn_ptr_1028::{fn_ptr_1028_d728, fn_ptr_1028_d742, fn_ptr_1030_835a};
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_07fc, pass1_1000_4906};
use crate::pass::pass_1008::{
    pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_5784,
    pass1_1008_5b12, pass1_1008_612e, pass1_1008_6c90, pass1_1008_6cec, pass1_1008_7ad4,
    pass1_1008_8faa, pass1_1008_df4a,
};
use crate::pass::pass_1010::{
    pass1_1010_043a, pass1_1010_089e, pass1_1010_65d0, pass1_1010_9766, pass1_1010_988c,
    pass1_1010_9f72, pass1_1010_ebf8, pass1_1010_ec84, pass1_1010_ed22, pass1_1010_ed3e,
};
use crate::pass::pass_1018::{pass1_1018_0196, pass1_1018_04b8, pass1_1018_34a6, pass1_1018_dcf6};
use crate::pass::pass_1020::{
    pass1_1020_a43e, pass1_1020_a49a, pass1_1020_a54c, pass1_1020_a80e, pass1_1020_a89e,
    pass1_1020_ba3e, pass1_1020_bae6, pass1_1020_bb16, pass1_1020_c42e, pass1_1020_c860,
    pass1_1020_c872,
};
use crate::pass::pass_1030::{
    fn_ptr_1030_7296, pass1_1030_1358, pass1_1030_13f6, pass1_1030_145a, pass1_1030_14b4,
    pass1_1030_154c, pass1_1030_165e, pass1_1030_16b2, pass1_1030_177a, pass1_1030_1a32,
    pass1_1030_1d58, pass1_1030_1d7c, pass1_1030_2068, pass1_1030_25b2, pass1_1030_2690,
    pass1_1030_2958, pass1_1030_2a2c, pass1_1030_2fac, pass1_1030_3006, pass1_1030_34da,
    pass1_1030_3534, pass1_1030_3548, pass1_1030_355c, pass1_1030_35a4, pass1_1030_3694,
    pass1_1030_375a, pass1_1030_387c, pass1_1030_38b8, pass1_1030_38f2, pass1_1030_4538,
    pass1_1030_4594, pass1_1030_4628, pass1_1030_4782, pass1_1030_4bbe, pass1_1030_4c06,
    pass1_1030_560e, pass1_1030_5a80, pass1_1030_5b1c, pass1_1030_5b3e, pass1_1030_5b5c,
    pass1_1030_5d0a, pass1_1030_5d3c, pass1_1030_5fe2, pass1_1030_615a, pass1_1030_61b0,
    pass1_1030_61fe, pass1_1030_627e, pass1_1030_64ce, pass1_1030_6522, pass1_1030_66de,
    pass1_1030_671c, pass1_1030_6740, pass1_1030_67cc, pass1_1030_684c, pass1_1030_6c4c,
    pass1_1030_6d80, pass1_1030_6fa0, pass1_1030_72d0, pass1_1030_730a, pass1_1030_73ee,
    pass1_1030_7c28, pass1_1030_7c50, pass1_1030_7d1c, pass1_1030_7d7c, pass1_1030_7ddc,
    pass1_1030_7e5a, pass1_1030_7f1a, pass1_1030_b936, pass1_1030_bc24, pass1_1030_bcae,
    pass1_1030_bcde, pass1_1030_bd74, pass1_1030_dcc2, pass1_1030_df0c,
};
use crate::pass::pass_1038::{
    empty_1038_540a, pass1_1038_3222, pass1_1038_349e, pass1_1038_362e, pass1_1038_3698,
    pass1_1038_387e, pass1_1038_3ba0, pass1_1038_3fb0, pass1_1038_3fca, pass1_1038_42cc,
    pass1_1038_43cc, pass1_1038_4900, pass1_1038_4d0e, pass1_1038_4d3c, pass1_1038_4d6e,
    pass1_1038_4e78, pass1_1038_4f54, pass1_1038_518c, pass1_1038_52b8, pass1_1038_5464,
    pass1_1038_565e, pass1_1038_56d6, pass1_1038_5770, pass1_1038_57dc, pass1_1038_5804,
    pass1_1038_75ca, pass1_1038_79b2, pass1_1038_79f2,
};
use crate::string::string_1000::string_1000_3d3e;
use crate::string::string_1008::load_str_and_sprintf_1008_b78a;
use crate::string::string_1030::vsprintf_op_1030_840a;
use crate::struct_ops::struct_1008::{
    clear_struct_1008_3e38, pass1_1008_c626, pass1_1008_c6ae, pass1_1008_c6fa,
    set_struct_1008_574a, struct_1008_bde0, struct_op_1008_8e9e,
};
use crate::struct_ops::struct_1028::{struct_1028_0068, struct_op_1028_87f0};
use crate::struct_ops::struct_1030::{
    struct_1030_11aa, struct_1030_1628, struct_1030_299a, struct_1030_44be, struct_1030_4574,
    struct_1030_565a, struct_1030_dc96, struct_1030_e4fa, struct_op_1030_73a8,
};
use crate::switch_ops::switch_1008::{switch_1008_72bc, switch_1008_73ea};
use crate::switch_ops::switch_1020::switch_1020_c3b4;
use crate::ui::ui_1008::post_win_msg_1008_a0e4;
use crate::ui::ui_1028::post_msg_1028_76da;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};

pub fn pass1_1028_00cc(
    ctx: &mut AppContext,
    param_1: &mut Struct_1028_00cc,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: &mut Struct18,
    extraout_dx: u16,
) {
    let u_var1: u16;
    pass1_1028_b39e(param_1, param_3, param_4, param_5);
    u_var1 = 0x0;
    // (param_1 + 0x20) = 0x0;
    param_1.field_0x20 = 0;
    // (param_1 + 0x22) = 0x0;
    param_1.field_0x22 = 0;
    // CONCAT22(param_2,param_1) = 0x8ec;
    param_1.field_0x0 = 0x8ec;
    // (param_1 + 0x2) = ctx.USHORT_1050_1028;
    param_1.field_0x2 = ctx.USHORT_1050_1028;
    let mut a = 0xcu16;
    mem_op_1000_179c(ctx, &mut a, param_5, 0x1000);
    if (param_5 | u_var1) == 0x0 {
        (param_1 + 0x22) = 0x0;
    } else {
        // set_struct_1008_574a(CONCAT22(param_5 as u16, u_var1));
        set_struct_1008_574a(param_5);
        // (param_1 + 0x22) = u_var1;
        param_1.field_0x22 = u_var1;
        // (param_1 + 0x24) = extraout_dx;
        param_1.field_0x24 = extraout_dx;
    }
    return;
}

pub fn pass1_1028_0138(ctx: &mut AppContext, param_1: &mut Struct_1028_0138) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct_1028_0138;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.field_0x0 = 0x8ec;
    (i_var4.field_0x2) = ctx.USHORT_1050_1028;
    pu_var1 = (i_var4.field_0x22) as u32;
    u_var2 = (i_var4.field_0x24);
    if (u_var2 | pu_var1) != 0x0 {
        ppc_var3 = pu_var1;
        (**ppc_var3)();
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_0176(
    ctx: &mut AppContext,
    param_1: &mut Struct_1028_0176,
    param_2: u32,
    param_3: u16,
    param_4: u16,
) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u32;
    let u_var5: u16;
    let paVar6: &mut Struct21;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let i_var9: &mut Struct306;
    let i_var8: &mut Struct298;

    i_var9 = param_1;
    // uVar8 = (param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_4);
    pu_var1 = i_var9.field_0x22;
    u_var2 = i_var9.field_0x24;
    u_var5 = u_var2 | pu_var1;
    paVar6 = CONCAT22(u_var5, pu_var1);
    if u_var5 != 0x0 {
        ppc_var3 = *pu_var1;
        paVar6 = (**ppc_var3)();
    }
    mem_op_1000_179c(ctx, 0xc, (paVar6 >> 0x10), 0x1000);
    if (paVar6 == 0x0) {
        uVar7 = 0x0;
    } else {
        uVar7 = set_struct_1008_574a(paVar6);
    }
    i_var9.field_0x22 = uVar7;
    i_var9.field_0x24 = (uVar7 >> 0x10);
    uVar9 = 0x14;
    u_var4 = pass1_1028_b58e(param_1);
    pass1_1030_7f1a(u_var4, uVar9, param_3);
    return;
}

pub fn pass1_1028_01ec(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x12) == 0x6) || ((i_var2 + 0x12) == 0x5)) {
        u_var1 = (i_var2 + 0x14);
        // u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        if (((i_var2 + 0xa6) == 0x14) || ((i_var2 + 0xa8) == 0x10)) {
            pass1_1028_bdac(param_1, 0x6, param_4);
            return;
        }
        pass1_1028_be2a(param_1, param_2, param_3, param_4, param_5);
    }
    return;
}

pub fn pass1_1028_0374(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let ppcVar1: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let local_18: [u16; 0x2];
    let paStack20: &mut Struct99;
    let local_10: [u16; 0x2];
    let local_c: [u16; 0x3];
    let uStack6: u16;
    let local_4: u16;
    let u_var2: &mut Struct728;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 != 0x0) {
        // u_var3 = (param_1 >> 0x10);
        uVar8 = param_2;
        // uVar9 = (param_2 >> 0x10);
        BVar4 = read_file_1008_7dee(uVar8, uVar9, param_1 + 0x20, 0x0, u_var3, 0x2, 0x1008);
        if (BVar4 != 0x0) {
            BVar4 = read_file_1008_7dee(uVar8, uVar9, &local_4, 0x0, param_5, 0x2, 0x1008);
            if (BVar4 != 0x0) {
                uStack6 = 0x0;
                loop {
                    if (local_4 <= uStack6) {
                        return;
                    }
                    paStack20 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                    // u_var6 = (paStack20 >> 0x10);
                    u_var2 = paStack20;
                    if ((u_var6 | u_var2) == 0x0) {
                        paStack20 = 0x0;
                    } else {
                        paStack20.field_0x0 = 0x389a;
                        u_var2.field_0x2 = 0x1008;
                        u_var2.field_0x4 = 0x0;
                        u_var2.field_0x6 = 0x0;
                        u_var2.field_0x8 = 0x0;
                        u_var2.field_0xa = 0x0;
                        u_var2.field_0xc = 0x0;
                        paStack20.field_0x0 = 0x56ce;
                        u_var2.field_0x2 = 0x1018;
                    }
                    BVar4 = read_file_1008_7dee(uVar8, uVar9, local_10, 0x0, param_5, 0x2, 0x1008);
                    if (BVar4 == 0x0) {
                        break;
                    }
                    BVar4 = read_file_1008_7dee(uVar8, uVar9, local_c, 0x0, param_5, 0x2, 0x1008);
                    if (BVar4 == 0x0) {
                        break;
                    }
                    BVar4 = read_file_1008_7dee(uVar8, uVar9, local_18, 0x0, param_5, 0x2, 0x1008);
                    if (BVar4 == 0x0) {
                        break;
                    }
                    BVar4 = read_file_1008_7dee(
                        uVar8,
                        uVar9,
                        paStack20 + 0xa,
                        0x0,
                        (paStack20 >> 0x10),
                        0x2,
                        0x1008,
                    );
                    if (BVar4 == 0x0) {
                        break;
                    }
                    BVar4 = read_file_1008_7dee(
                        uVar8,
                        uVar9,
                        paStack20 + 0xc,
                        0x0,
                        (paStack20 >> 0x10),
                        0x2,
                        0x1008,
                    );
                    if (BVar4 == 0x0) {
                        break;
                    }
                    (paStack20 + 0x4) = local_10[0];
                    u_var5 = switch_1008_72bc(uVar8, uVar9, local_c[0]);
                    // uVar7 = (paStack20 >> 0x10);
                    (paStack20 + 0x6) = u_var5;
                    (paStack20 + 0x8) = local_18[0];
                    ppcVar1 = ((param_1 + 0x22) + 0x8);
                    (**ppcVar1)();
                    uStack6 += 0x1;
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

pub fn pass1_1028_04ee(param_1: u32, param_2: U32Ptr, param_3: u16) -> u16 {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let lVar5: i32;
    let local_a: [u8; 8];

    *param_2 = 0x0;
    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x22));
    loop {
        lVar5 = pass1_1008_5b12(local_a, param_3);
        if lVar5 == 0x0 {
            return 0x0;
        }
        u_var2 = (lVar5 + 0xc);
        // u_var4 = (param_2 >> 0x10);
        u_var3 = param_2;
        param_2 = param_2 + u_var2;
        pi_var1 = (param_2 + 0x2);
        *pi_var1 = *pi_var1 + CARRY2(u_var3, u_var2);
        if (((param_2 + 0x2) == 0x0) && (param_2 < 0x1e)) == false {
            break;
        }
    }
    return 0x1;
}

pub fn pass1_1028_0550(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: i16;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        u_var4 = 0x0;
        iVar5 = 0x4;
        u_var3 = 0x1;
        u_var2 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        pass1_1030_7c50(
            u_var2,
            CONCAT22(u_var4, u_var3),
            iVar5,
            u_var2,
            (u_var2 >> 0x10),
        );
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_0582(
    param_1: U32Ptr,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u8,
    param_6: u16,
) {
    u32 * *ppuVar1;
    let plVar2: &i32;
    let u_var3: u32;
    let ppc_var4: u32;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u32;
    let extraout_dx: u16;
    let uVar9: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let iVar10: i16;
    let iVar11: i16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let local_138: [u8; 0x10e];
    let local_2a: u32;
    let paStack38: &mut Struct99;
    let paStack34: &mut Struct99;
    let uStack30: u32;
    let uStack18: u32;
    let uStack14: u32;
    let local_a: [u8; 4];
    let uStack6: u32;

    // uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    uVar8 = (iVar10 + 0x14);
    // uVar13 = (uVar8 >> 0x10);
    iVar11 = uVar8;
    uStack6 = uVar8 & 0xffff0000 | (iVar11 + 0xa4);
    if (((iVar11 + 0xa6) != 0x0) && ((iVar11 + 0xac) != 0x0)) {
        pass1_1028_081e(param_1, param_2, param_6);
        param_2 = (iVar10 + 0x20);
        ppuVar1 = (uStack6 + 0x8);
        if (*ppuVar1 < param_2 || *ppuVar1 == param_2) {
            pu_var5 = local_a;
            ppc_var4 = (*param_1 + 0x40);
            (**ppc_var4)(param_3, param_1);
            uVar8 = ZEXT24(pu_var5);
            param_6 = extraout_dx;
            if (pu_var5 == 0x0) {
                if ((uStack6 + 0x2) == 0xc) {
                    uStack14 = pass1_1028_b4f2(param_1);
                    // param_6 = (uStack14 >> 0x10);
                    uVar8 = (uStack14 + 0x1f6);
                    plVar2 = (uVar8 + 0x170);
                    *plVar2 = *plVar2 + 0x1;
                    uStack18 = uVar8;
                } else {
                    uStack18 = ctx.PTR_LOOP_1050_68a2;
                    paStack38 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                    // uVar9 = (paStack38 >> 0x10);
                    u_var6 = paStack38;
                    if ((uVar9 | u_var6) == 0x0) {
                        paStack34 = 0x0;
                    } else {
                        paStack38.field_0x0 = 0x389a;
                        (u_var6 + 0x2) = 0x1008;
                        (u_var6 + 0x4) = 0x0;
                        (u_var6 + 0x6) = 0x0;
                        (u_var6 + 0x8) = 0x0;
                        (u_var6 + 0xa) = 0x0;
                        (u_var6 + 0xc) = 0x0;
                        paStack38.field_0x0 = 0x56ce;
                        (u_var6 + 0x2) = 0x1018;
                        paStack34 = paStack38;
                    }
                    // uVar13 = (uStack6 >> 0x10);
                    iVar11 = uStack6;
                    // uVar14 = (paStack34 >> 0x10);
                    (paStack34 + 0x6) = (iVar11 + 0x2);
                    (paStack34 + 0xa) = (iVar11 + 0x6);
                    param_3 = 0x1020;
                    uVar7 = switch_1020_c3b4((iVar11 + 0x2));
                    u_var6 = uVar7 * (uStack6 + 0x6);
                    uVar8 = u_var6;
                    (paStack34 + 0xc) = u_var6;
                    u_var3 = (iVar10 + 0x22);
                    ppc_var4 = ((iVar10 + 0x22) + 0x4);
                    (**ppc_var4)(0x1020, u_var3, (u_var3 >> 0x10));
                    param_6 = extraout_DX_00;
                }
            }
            param_2 = uVar8;
            (iVar10 + 0x20) = 0x0;
        }
    }
    // uVar13 = (uStack6 >> 0x10);
    if (((uStack6 + 0x4) != 0x0) && ((uStack6 + 0x8) != 0x0)) {
        pass1_1028_081e(param_1, param_2, param_6);
        param_2 = (iVar10 + 0x20);
        ppuVar1 = (uStack6 + 0x8);
        if (*ppuVar1 < param_2 || *ppuVar1 == param_2) {
            param_2 = &local_2a;
            ppc_var4 = (*param_1 + 0x40);
            (**ppc_var4)(param_3, param_1);
            if (param_2 == 0x0) {
                uStack18 = ctx.PTR_LOOP_1050_68a2;
                paStack38 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                // uVar9 = (paStack38 >> 0x10);
                u_var6 = paStack38;
                if ((uVar9 | u_var6) == 0x0) {
                    paStack34 = 0x0;
                } else {
                    paStack38.field_0x0 = 0x389a;
                    (u_var6 + 0x2) = 0x1008;
                    (u_var6 + 0x4) = 0x0;
                    (u_var6 + 0x6) = 0x0;
                    (u_var6 + 0x8) = 0x0;
                    (u_var6 + 0xa) = 0x0;
                    (u_var6 + 0xc) = 0x0;
                    paStack38.field_0x0 = 0x56ce;
                    (u_var6 + 0x2) = 0x1018;
                    paStack34 = paStack38;
                }
                // uVar13 = (uStack6 >> 0x10);
                iVar11 = uStack6;
                // uVar14 = (paStack34 >> 0x10);
                (paStack34 + 0x8) = (iVar11 + 0x4);
                (paStack34 + 0xa) = (iVar11 + 0x6);
                uVar7 = pass1_1020_c42e((iVar11 + 0x4));
                param_2 = (uVar7 * (uStack6 + 0x6));
                (paStack34 + 0xc) = param_2;
                u_var3 = (iVar10 + 0x22);
                ppc_var4 = ((iVar10 + 0x22) + 0x4);
                (**ppc_var4)(0x1020, u_var3, (u_var3 >> 0x10));
            }
            (iVar10 + 0x20) = 0x0;
        }
    }
    if ((iVar10 + 0xc) != 0xe) {
        pass1_1028_b58e(param_1 & 0xffff | uVar12 << 0x10);
        local_2a = CONCAT22(extraout_DX_01, param_2);
        paStack34 = (param_2 + 0x2e);
        uStack30 = (paStack34 + 0x4);
        pass1_1028_68de(
            CONCAT22(param_4, local_138),
            0x1,
            uStack30,
            param_5,
            param_4,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_4, local_138));
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_081e(param_1: u32, param_2: i16, param_3: u16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;

    pass1_1028_b58e(param_1);
    u_var4 = (param_2 + 0x2e);
    i_var2 = (u_var4 + 0x18);
    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pi_var1 = (iVar6 + 0x20);
    *pi_var1 = *pi_var1 + 0x1;
    u_var5 = *ctx.PTR_LOOP_1050_65e2;
    u_var3 = (ctx.PTR_LOOP_1050_65e2 + 0x2);
    if (i_var2 < 0xfa) {
        u_var5 &= 0x1;
    } else {
        if (0x1c1 < i_var2) {
            if (i_var2 < 0x226) {
                return;
            }
            if ((i_var2 < 0x2ee) && (CONCAT22(u_var3, u_var5) % 0x3 != 0x0)) {
                return;
            }
            pi_var1 = (iVar6 + 0x20);
            *pi_var1 = *pi_var1 + 0x1;
            return;
        }
        u_var5 = (CONCAT22(u_var3, u_var5) % 0x3);
    }
    if (u_var5 != 0x0) {
        return;
    }
    pi_var1 = (iVar6 + 0x20);
    *pi_var1 = *pi_var1 + -0x1;
    return;
}

pub fn pass1_1028_08c6(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_0138(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_0982(
    param_1: &mut Struct179,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    param_1.field_0x20 = 0x0;
    CONCAT22(param_2, param_1) = 0xada;
    param_1.field_0x2 = &USHORT_1050_1028;
    param_1.field_0xe = 0x4b;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_09b8(param_1: u32) {
    let u_var1: u32;

    u_var1 = pass1_1028_b58e(param_1);
    (u_var1 + 0x14) = 0x1;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_09d4(
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u32,
    param_7: i32,
) {
    let i_var1: i16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let local_6: [u8; 2];
    let BStack4: bool;

    u_var5 = param_4;
    // u_var6 = (param_4 >> 0x10);
    // uVar7 = (param_6 >> 0x10);
    BStack4 = pass1_1028_c314(
        param_1, param_2, param_3, u_var5, u_var6, param_5, param_6, uVar7, param_7,
    );
    if (BStack4 == 0x0) {
        return;
    }
    pass1_1028_c7b6(param_1, param_3, u_var5, u_var6, param_5, param_7);
    if ((BStack4 != 0x0) && (BStack4 != 0x4)) {
        if (((BStack4 - 0x5) < 0x1) || (SBORROW2(BStack4 - 0x5, 0x1) || (0x3 < (BStack4 - 0x6)))) {
            if (((u_var5 + 0xc) != 0x3e) && ((u_var5 + 0xc) != 0x41)) {
                return;
            }
            u_var4 = pass1_1030_bcae(local_6, param_1);
            // u_var3 = (u_var4 >> 0x10);
            i_var1 = u_var4;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6);
            u_var4 = (i_var1 + 0x10);
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
            pu_var2 = local_6;
            pass1_1030_bcde(
                param_1,
                pu_var2,
                param_1,
                u_var4 & 0xffff | u_var3 << 0x10,
                param_5,
                param_7,
            );
            if (pu_var2 < 0x0) {
                ctx.PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            if (0x5 < pu_var2) {
                ctx.PTR_LOOP_1050_50ca = 0x6b5;
                return;
            }
            return;
        }
    }
    ctx.PTR_LOOP_1050_50ca = 0x6a8;
    return;
}

pub fn pass1_1028_0ab4(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_0b64(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0xbbc;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_0b96(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn struct_1028_0c24(param_1: U32Ptr) -> u16 {
    let i_var1: &mut Struct191;
    let u_var1: u16;

    struct_1028_b354(param_1);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x20 = 0x0;
    i_var1.field_0x22 = 0x0;
    *param_1 = (s_480_bmp_1050_1721 + 0x3);
    i_var1.field_0x2 = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_0c50(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    (param_1 + 0x22) = 0x0;
    CONCAT22(param_2, param_1) = (s_480_bmp_1050_1721 + 0x3);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_0c84(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let pu_var3: u32;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u32;
    let puVar6: u32;
    let puVar7: u32;
    let bStack55: u8;
    let local_32: [u8; 0xa];
    let uStack40: u32;
    let uStack36: u32;
    let puStack28: u32;
    let local_1a: u32;
    let iStack22: i16;
    let uStack20: u16;
    let iStack18: i16;
    let uStack16: u16;
    let iStack14: i16;
    let local_c: u32;
    let iStack8: i16;
    let uStack6: u32;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_3);
    local_c = (param_3 + 0xc);
    iStack18 = (param_3 + 0x10);
    puStack28 = &local_c;
    uStack16 = extraout_dx;
    iStack14 = iStack18;
    iStack8 = iStack18;
    pass1_1028_bab6(param_1, iStack18, extraout_dx);
    u_var2 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    iStack22 = iStack8;
    uStack36 = CONCAT22(uStack36._2_2_, &local_1a);
    iStack14 += 0x1;
    uStack20 = u_var2;
    if (iStack14 <= u_var2) {
        puVar7 = CONCAT22(param_4, local_32);
        iStack22 = iStack14;
        u_var5 = pass1_1028_bb24(param_1);
        // u_var4 = (u_var5 >> 0x10);
        pu_var3 = &local_1a;
        pass1_1030_64ce(
            param_4,
            pu_var3,
            u_var4,
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_4, pu_var3),
            u_var5 & 0xffff | u_var4 << 0x10,
            puVar7,
        );
        uStack40 = *pu_var3;
        u_var4 = (pu_var3 + 0x2);
        bStack55 = (uStack40 >> 0x18);
        u_var2 = bStack55;
        uStack36 = uStack40;
        if (bStack55 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack40);
            puVar6 = struct_op_1030_73a8(CONCAT22(u_var4, u_var2));
            u_var2 = puVar6;
            ppcVar1 = (*puVar6 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(param_1, param_2, u_var2);
    fn_ptr_1030_7296(uStack6);
    return;
}

pub fn pass1_1028_0d80(param_1: u32) -> u16 {
    let u_var1: u16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x20);
    pass1_1028_1646(param_1 & 0xffff | u_var2 << 0x10);
    return u_var1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_0d9c(param_1: u32, param_2: i16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let BVar5: bool;
    let extraout_dx: u16;
    let u_var6: u16;
    let uVar7: u32;
    let puVar8: u32;
    let uStack58: u32;
    let local_32: [u8; 6];
    let puStack44: u32;
    let uStack40: u32;
    let uStack36: u32;
    let puStack28: u32;
    let local_1a: u32;
    let iStack22: i16;
    let uStack20: u16;
    let iStack18: i16;
    let uStack16: u16;
    let iStack14: i16;
    let local_c: u32;
    let iStack8: i16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_c = (param_2 + 0xc);
    iStack18 = (param_2 + 0x10);
    puStack28 = &local_c;
    uStack16 = extraout_dx;
    iStack14 = iStack18;
    iStack8 = iStack18;
    i_stack6 = param_2;
    uStack4 = extraout_dx;
    pass1_1028_bab6(param_1, iStack18, extraout_dx);
    uStack20 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    uStack36 = CONCAT22(uStack36._2_2_, &local_1a);
    iStack22 = iStack14 + 0x1;
    if (iStack22 <= uStack20) {
        puVar8 = CONCAT22(param_3, local_32);
        iStack14 = iStack22;
        uVar7 = pass1_1028_bb24(param_1);
        // u_var6 = (uVar7 >> 0x10);
        pu_var2 = &local_1a;
        pass1_1030_64ce(
            param_3,
            pu_var2,
            u_var6,
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_3, pu_var2),
            uVar7 & 0xffff | u_var6 << 0x10,
            puVar8,
        );
        uStack40 = *pu_var2;
        u_var6 = (pu_var2 + 0x2);
        uStack58._3_1_ = (uStack40 >> 0x18);
        u_var3 = uStack58._3_1_;
        if (uStack58._3_1_ != 0x0) {
            uStack36 = uStack40;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack40);
            uStack58 = CONCAT22(u_var6, u_var3);
            u_var4 = pass1_1030_6fa0(CONCAT22(u_var6, u_var3));
            BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var4, 0x13);
            if (BVar5 != 0x0) {
                puStack44 = struct_op_1030_73a8(uStack58);
                ppcVar1 = (*puStack44 + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_0ea6(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let pi_var1: U32Ptr;
    let BVar2: bool;
    let u_var3: u16;
    let i_var3: &mut Struct597;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (i_var3.field_0xc != 0x10) {
        BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var3.field_0xc, 0x13);
        if (BVar2 == 0x0) {
            BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var3.field_0xc, 0x2);
            if (((BVar2 != 0x0) && (i_var3.field_0x12 != 0x7)) && (i_var3.field_0x12 != 0x4)) {
                u_var3 =
                    pass1_1028_1556(param_1 & 0xffff | u_var4 << 0x10, BVar2, param_2, param_5);
                if (u_var3 == 0x0) {
                    // goto
                    // LAB_1028_0f0a;
                }
                if (i_var3.field_0x12 == 0x9) {
                    i_var3.field_0x12 = 0x5;
                }
            }
        } else {
            if (i_var3.field_0x22 < 0x1) {
                if ((i_var3.field_0x12 != 0x5) && (i_var3.field_0x12 != 0x6)) {
                    return;
                }
                fn_ptr_1000_17ce(ctx, i_var3.field_0x14, 0x1000);
                i_var3.field_0x14 = 0x0;
                //LAB_1028_0f0a:
                i_var3.field_0x12 = 0x9;
                return;
            }
        }
        pass1_1028_be2a(param_1, param_3, param_4, 0x1008, param_5);
        if (i_var3.field_0x12 == 0x5) {
            pi_var1 = &i_var3.field_0x22;
            *pi_var1 = *pi_var1 + -0x1;
            pass1_1028_b58e(param_1 & 0xffff | u_var4 << 0x10);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_0fa4(
    param_1: U32Ptr,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;

    pass1_1028_be9e(param_1, param_3, param_4, param_5, param_6);
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_6, param_2, param_4);
    i_var8 = (puVar4 + 0x82);
    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x12) == 0x5) {
        b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (i_var2 + 0xc), 0x2);
        if ((b_var1 != 0x0) && (ctx.PTR_LOOP_1050_4fbc == 0x0 || (i_var8 != 0x0))) {
            ctx.PTR_LOOP_1050_4fbc = (&ctx.PTR_LOOP_1050_0000 + 0x1);
            uVar7 = 0x0;
            i_var8 = 0x4;
            u_var6 = 0x1;
            u_var5 = pass1_1028_b58e(param_1);
            pass1_1030_7c50(
                u_var5,
                CONCAT22(uVar7, u_var6),
                i_var8,
                u_var5,
                (u_var5 >> 0x10),
            );
        }
        (i_var2 + 0x22) = 0x64;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_1024(param_1: u32, param_2: i16, param_3: u16, param_4: u16) -> i16 {
    let b_var1: bool;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let iStack26: i16;
    let iStack24: i16;
    let local_16: u32;
    let iStack18: i16;
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u32;
    let uStack8: u16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_bab6(param_1, param_2, param_3);
    i_stack6 = param_2;
    uStack4 = param_3;
    uStack8 = pass1_1030_2fac(CONCAT22(param_3, param_2));
    uStack12 = pass1_1028_bb24(param_1);
    u_var6 = pass1_1028_b58e(param_1);
    // uStack14 = (u_var6 >> 0x10);
    local_16 = (u_var6 + 0xc);
    iStack26 = 0x0;
    iStack24 = 0x0;
    loop {
        if (uStack8 < iStack26) {
            return iStack24;
        }
        iStack18 = iStack26;
        pu_var2 = &local_16;
        pass1_1030_627e(
            param_4,
            pu_var2,
            (u_var6 >> 0x10),
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_4, pu_var2),
            uStack12,
        );
        uStack16 = u_var6;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2, (u_var6 >> 0x10));
        uStack16 = u_var6;
        if (((u_var6 >> 0x10) | pu_var2) == 0x0) {
            break;
        }
        u_var6 = struct_op_1030_73a8(u_var6 & 0xffff0000 | ZEXT24(pu_var2));
        // u_var4 = (u_var6 >> 0x10);
        u_var3 = u_var6;
        u_var5 = u_var4 | u_var3;
        if (u_var6 == 0x0) {
            return iStack24;
        }
        b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var3 + 0xc), 0x13);
        u_var6 = CONCAT22(u_var5, uStack16);
        if ((b_var1 != 0x0) & &((u_var3 + 0x12) == 0x5)) {
            iStack24 += 0x1;
        }
        iStack26 += 0x1;
    }
    return iStack24;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_1106(param_1: u32, param_2: i16, param_3: u16, param_4: u16) -> i16 {
    let b_var1: bool;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let iStack26: i16;
    let iStack24: i16;
    let local_16: u32;
    let iStack18: i16;
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u32;
    let uStack8: u16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_bab6(param_1, param_2, param_3);
    i_stack6 = param_2;
    uStack4 = param_3;
    uStack8 = pass1_1030_2fac(CONCAT22(param_3, param_2));
    uStack12 = pass1_1028_bb24(param_1);
    u_var5 = pass1_1028_b58e(param_1);
    // uStack14 = (u_var5 >> 0x10);
    local_16 = (u_var5 + 0xc);
    iStack26 = 0x0;
    iStack24 = 0x0;
    loop {
        if (uStack8 < iStack26) {
            return iStack24;
        }
        iStack18 = iStack26;
        pu_var2 = &local_16;
        pass1_1030_627e(
            param_4,
            pu_var2,
            (u_var5 >> 0x10),
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_4, pu_var2),
            uStack12,
        );
        uStack16 = u_var5;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2, (u_var5 >> 0x10));
        uStack16 = u_var5;
        if (((u_var5 >> 0x10) | pu_var2) == 0x0) {
            break;
        }
        u_var5 = struct_op_1030_73a8(u_var5 & 0xffff0000 | ZEXT24(pu_var2));
        // u_var3 = (u_var5 >> 0x10);
        u_var4 = u_var3 | u_var5;
        if (u_var5 == 0x0) {
            return iStack24;
        }
        b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var5 + 0xc), 0x13);
        u_var5 = CONCAT22(u_var4, uStack16);
        if (b_var1 != 0x0) {
            iStack24 += 0x1;
        }
        iStack26 += 0x1;
    }
    return iStack24;
}

pub fn pass1_1028_11de(param_1: u32) -> bool {
    let u_var1: u32;

    u_var1 = pass1_1028_b58e(param_1);
    return (u_var1 + 0x10) == 0x0;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_121e(param_1: u32, param_2: u16) -> u32 {
    let bVar1: bool;
    let extraout_AH: u8;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let local_c: u32;
    let uStack8: u16;
    let uStack6: u32;

    bVar1 = pass1_1028_11de(param_1);
    if (CONCAT11(extraout_AH, bVar1) != 0x0) {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = (uStack6 + 0xc);
    uStack8 = 0x0;
    u_var4 = pass1_1028_bb24(param_1);
    // u_var3 = (u_var4 >> 0x10);
    pu_var2 = &local_c;
    pass1_1030_627e(
        param_2,
        pu_var2,
        u_var3,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_2, pu_var2),
        u_var4 & 0xffff | u_var3 << 0x10,
    );
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2);
    if ((u_var3 | pu_var2) == 0x0) {
        return 0x0;
    }
    u_var4 = struct_op_1030_73a8(CONCAT22(u_var3, pu_var2));
    return u_var4;
}

pub fn pass1_1028_12be(param_1: u32, param_2: U32Ptr, param_3: u16) -> u16 {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let ppc_var3: u32;
    let bVar4: bool;
    let extraout_AH: u8;
    let u_var5: u16;
    let puVar6: u32;
    let uVar7: u32;
    let uVar8: u32;
    let uStack8: u16;

    bVar4 = pass1_1028_11de(param_1);
    if (CONCAT11(extraout_AH, bVar4) == 0x0) {
        puVar6 = pass1_1028_121e(param_1, param_3);
        ppc_var3 = (*puVar6 + 0x40);
        u_var5 = (**ppc_var3)();
        return u_var5;
    }
    *param_2 = 0x0;
    uVar7 = pass1_1028_b58e(param_1);
    uStack8 = 0x4;
    uVar8 = uVar7;
    loop {
        uVar8 = pass1_1030_7c28(uVar7, uStack8, uVar8, (uVar8 >> 0x10), param_3);
        u_var2 = param_2;
        param_2 = param_2 + uVar8;
        pi_var1 = (param_2 + 0x2);
        *pi_var1 = *pi_var1 + (uVar8 >> 0x10) + CARRY2(u_var2, uVar8);
        uStack8 += 0x1;
        if (uStack8 < 0xe) == false {
            break;
        }
    }
    if (0x1f4 < *param_2) {
        return 0x1;
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_134a(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let BVar3: bool;
    let plVar4: &i32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let lStack26: i32;
    let iStack22: i16;
    let uStack18: u32;
    let uStack10: u32;
    let local_6: i32;

    // u_var6 = (param_1 >> 0x10);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (param_1 + 0xc), 0x13);
    if (BVar3 != 0x0) {
        plVar4 = &local_6;
        ppcVar2 = (*param_1 + 0x40);
        (**ppcVar2)(0x1008, param_1, plVar4, param_4);
        if (plVar4 != 0x0) {
            pi_var1 = (param_1 + 0x22);
            *pi_var1 = *pi_var1 + 0x1;
            return;
        }
        uStack10 = 0x1f4 - local_6;
        uVar7 = pass1_1028_121e(param_1, param_4);
        // u_var5 = (uVar7 >> 0x10);
        u_var6 = uVar7;
        pass1_1028_b58e(uVar7);
        uStack18 = CONCAT22(u_var5, u_var6);
        // TODO: refactor for loop
        // for (iStack22 = 0x0; iStack22 < 0xa; iStack22 += 0x1) {
        //   uStack10._0_2_ = (iStack22 * 0x2 + 0x4fbe);
        //   uStack10._2_2_ = (uStack10 >> 0xf);
        //   if (uStack10 < uStack10) {
        //   }
        //   lStack26 = CONCAT22(uStack10._2_2_,uStack10);
        //   pass1_1030_7ddc(uStack18,CONCAT13((uStack10._2_2_ >> 0x8),
        //                                     CONCAT12(uStack10._2_2_,uStack10)),
        //                   iStack22 + 0x4,uStack10,uStack10._2_2_,param_2,param_3,param_4
        //                  );
        //   uStack10 -= lStack26;
        //   if (uStack10 < 0x1) {
        //     return;
        //   }
        // }
    }
    return;
}

pub fn pass1_1028_1416(param_1: u32, param_2: u16, param_3: u16) -> i16 {
    let bVar1: bool;
    let extraout_AH: u8;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u32;

    bVar1 = pass1_1028_11de(param_1);
    if (CONCAT11(extraout_AH, bVar1) == 0x0) {
        u_var4 = pass1_1028_121e(param_1, param_3);
        // u_var3 = (u_var4 >> 0x10);
        i_var2 = pass1_1028_1416(u_var4 & 0xffff | u_var3 << 0x10, u_var3, param_3);
        return i_var2;
    }
    i_var2 = pass1_1028_1024(param_1, CONCAT11(extraout_AH, bVar1), param_2, param_3);
    return i_var2 * 0xf;
}

pub fn pass1_1028_14d8(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u16;
    let local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 != 0x0) {
        // u_var1 = (param_1 >> 0x10);
        u_var3 = param_2;
        // u_var4 = (param_2 >> 0x10);
        BVar2 = read_file_1008_7dee(u_var3, u_var4, param_1 + 0x22, 0x0, u_var1, 0x2, 0x1008);
        if ((BVar2 != 0x0)
            && (
                BVar2 = read_file_1008_7dee(u_var3, u_var4, &local_4, 0x0, param_5, 0x2, 0x1008),
                BVar2 != 0x0,
            ))
        {
            (param_1 + 0x20) = local_4;
            if (ctx.PTR_LOOP_1050_0312 < 0x2) {
                return;
            }
            BVar2 = read_file_1008_7dee(
                u_var3,
                u_var4,
                &ctx.PTR_LOOP_1050_4fbc,
                0x0,
                ctx.data_seg,
                0x2,
                0x1008,
            );
            if (BVar2 != 0x0) {
                return;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_1556(param_1: u32, param_2: i16, param_3: u16, param_4: u16) -> u16 {
    let i_var1: i16;
    let pu_var2: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let iStack26: i16;
    let local_16: u32;
    let iStack18: i16;
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u32;
    let uStack8: u16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_bab6(param_1, param_2, param_3);
    i_stack6 = param_2;
    uStack4 = param_3;
    uStack8 = pass1_1030_2fac(CONCAT22(param_3, param_2));
    uStack12 = pass1_1028_bb24(param_1);
    uVar7 = pass1_1028_b58e(param_1);
    // uStack14 = (uVar7 >> 0x10);
    local_16 = (uVar7 + 0xc);
    iStack26 = 0x1;
    loop {
        if (uStack8 < iStack26) {
            return 0x0;
        }
        iStack18 = iStack26;
        pu_var2 = &local_16;
        pass1_1030_627e(
            param_4,
            pu_var2,
            (uVar7 >> 0x10),
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_4, pu_var2),
            uStack12,
        );
        uStack16 = uVar7;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2);
        uStack16 = uVar7;
        if (((uVar7 >> 0x10) | pu_var2) == 0x0) {
            return 0x0;
        }
        uVar7 = struct_op_1030_73a8(uVar7 & 0xffff0000 | ZEXT24(pu_var2));
        // u_var5 = (uVar7 >> 0x10);
        u_var3 = uVar7;
        u_var6 = u_var5 | u_var3;
        if (uVar7 == 0x0) {
            return 0x0;
        }
        i_var1 = (u_var3 + 0xc);
        BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var1, 0x13);
        uVar7 = CONCAT22(u_var6, uStack16);
        if ((BVar4 == 0x0) && (i_var1 != 0x75)) {
            break;
        }
        if ((u_var3 + 0x12) != 0x9) {
            return 0x1;
        }
        iStack26 += 0x1;
    }
    return 0x0;
}

pub fn pass1_1028_1646(param_1: u32) -> Struct409 {
    let paVar1: &mut Struct409;
    let u_var2: &mut Struct409;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    u_var2 = param_1;
    paVar1 = (u_var2.field_0x20 + -0x4);
    if (paVar1 < &ctx.DAT_1050_0009) {
        match (paVar1) {
            0x0 => {
                u_var2.field_0x20 = 0x5;
            }
            0x1 => {
                u_var2.field_0x20 = 0x6;
            }
            0x2 => {
                u_var2.field_0x20 = 0x7;
            }
            0x3 => {
                u_var2.field_0x20 = 0x8;
            }
            0x4 => {
                u_var2.field_0x20 = 0x9;
            }
            0x5 => {
                u_var2.field_0x20 = 0xa;
                return u_var2;
            }
            0x6 => {
                u_var2.field_0x20 = 0xb;
                return u_var2;
            }
            0x7 => {
                u_var2.field_0x20 = 0xc;
                return u_var2;
            }
            0x8 => {
                u_var2.field_0x20 = 0xd;
                return u_var2;
            }
        }
        return u_var2;
    }
    u_var2.field_0x20 = 0x4;
    return paVar1;
}

pub fn pass1_1028_16fe(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_17ae(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1030_dcc2(param_1, param_2, param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x1b54;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_17d8(param_1: u16, param_2: u16, param_3: u16) {
    let extraout_dx: u16;

    pass1_1030_df0c(CONCAT22(param_2, param_1), param_3);
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc((param_3 + 0x2e), 0x1, 0x3);
    return;
}

pub fn pass1_1028_1812(param_1: U32Ptr, param_2: u16) {
    pass1_1028_bdac(param_1, 0x2, param_2);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_1824(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u32,
    param_4: u32,
    param_5: i16,
    param_6: u16,
    param_7: i16,
    param_8: u16,
) {
    let b_var1: bool;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let local_2a: u32;
    let iStack38: i16;
    let iStack36: i16;
    let uStack34: u16;
    let uStack32: u16;
    let puStack30: U32Ptr;
    let uStack28: u16;
    let iStack24: i16;
    let puStack22: U32Ptr;
    let uStack16: u16;
    let uStack14: u16;
    let local_c: u32;
    let iStack8: i16;
    let uStack6: u32;

    uVar8 = param_1;
    // uVar9 = (param_1 >> 0x10);
    pass1_1028_c3aa(uVar8, uVar9, param_2, param_3, param_4, param_8);
    if (param_5 == 0x0) {
        return;
    }
    b_var1 = pass1_1028_c314(
        param_8,
        param_5,
        param_6,
        uVar8,
        uVar9,
        param_2,
        param_3,
        (param_3 >> 0x10),
        param_4,
    );
    if (b_var1 == 0x0) {
        return;
    }
    pu_var2 = &local_c;
    pass1_1030_64ce(
        param_8,
        pu_var2,
        param_6,
        ctx.PTR_LOOP_1050_5740,
        param_2,
        param_4,
        CONCAT22(param_8, pu_var2),
    );
    uStack6 = *pu_var2;
    pu_var5 = (pu_var2 + 0x2);
    // u_var6 = (param_2 >> 0x10);
    iStack8 = (param_2 + 0x4);
    puStack22 = (uStack6 & 0xffff | ZEXT24(pu_var5) << 0x10);
    uStack32 = uStack6;
    u_var3 = pu_var5 >> 0x8;
    if ((pu_var5 >> 0x8) != '\0') {
        puStack30 = pu_var5;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
        uStack32 = u_var3;
        puStack30 = pu_var5;
        uStack28 = pass1_1030_6fa0(CONCAT22(pu_var5, u_var3));
        if (uStack28 == 0x10) {
            if (iStack8 != 0x0) {
                ctx.PTR_LOOP_1050_50ca = 0x6ab;
                return;
            }
            return;
        }
        if ((uStack28 == 0x60) || (uStack28 == 0x61)) {
            puStack22 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_8, pu_var5, param_7);
            uVar7 = pass1_1018_04b8(puStack22);
            // uStack34 = (uVar7 >> 0x10);
            uStack16 = uVar7;
            iStack36 = (puStack22 + 0x1e);
            iStack24 = iStack36;
            uStack14 = uStack34;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack16);
            u_var4 = pass1_1030_2fac(CONCAT22(uStack34, iStack36));
            if (u_var4 <= iStack24) {
                ctx.PTR_LOOP_1050_50ca = 0x6ac;
                return;
            }
            local_2a = *param_2;
            iStack38 = iStack8 + 0x1;
            pu_var2 = &local_2a;
            pass1_1028_c7b6(
                param_8,
                u_var6,
                uVar8,
                uVar9,
                CONCAT22(param_8, pu_var2),
                param_4,
            );
            if (pu_var2 == 0x0) {
                return;
            }
            if (pu_var2 == (&ctx.DAT_1050_0004 + 0x2)) {
                return;
            }
            return;
        }
    }
    ctx.PTR_LOOP_1050_50ca = 0x6aa;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_199a(param_1: u16, param_2: i16, param_3: u8, param_4: u32) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let extraout_dx: U32Ptr;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let piVar5: U32Ptr;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let local_15e: u16;
    let uStack348: u16;
    let puStack50: u32;
    let uStack42: u32;
    let uStack38: u16;
    let piStack36: U32Ptr;
    let local_22: i16;
    let local_20: u16;
    let uStack30: u32;
    let puStack26: U32Ptr;
    let local_16: i16;
    let local_14: u32;
    let local_10: u32;
    let uStack12: u16;
    let uStack10: u32;
    let i_stack6: i16;
    let puStack4: U32Ptr;

    pi_var1 = (param_4 + 0x14);
    *pi_var1 = *pi_var1 + -0x1;
    if (*pi_var1 == 0x0) {
        pass1_1028_b58e(param_4);
        uStack10 = (param_2 + 0x2e);
        i_stack6 = param_2;
        puStack4 = extraout_dx;
        pass1_1038_5804(uStack10, 0x1, 0x3);
        local_10 = (i_stack6 + 0xc);
        uStack12 = (i_stack6 + 0x10);
        puStack50 = &local_10;
        pu_var3 = puStack4;
        pass1_1008_3eb4(
            CONCAT22(param_1, &local_10),
            CONCAT22(param_1, &local_16),
            CONCAT22(param_1, &local_14),
            CONCAT22(param_1, &local_14 + 0x2),
        );
        puStack26 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_1, pu_var3, &uStack10);
        u_var2 = (puStack26 + 0x20);
        puVar7 = &local_20;
        piStack36 = &local_22;
        piVar5 = piStack36;
        u_var6 = param_1;
        uVar8 = param_1;
        uStack30 = u_var2;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        uStack38 = u_var2;
        pass1_1030_5b1c(
            u_var2 & 0xffff | ZEXT24(piStack36) << 0x10,
            CONCAT22(u_var6, piVar5),
            CONCAT22(uVar8, puVar7),
        );
        if (local_22 < local_16 + 0x1) {
            pass1_1030_5b3e(CONCAT22(piStack36, uStack38), local_16 + 0x1, local_20);
            pass1_1030_5b1c(
                CONCAT22(piStack36, uStack38),
                CONCAT22(param_1, &local_22),
                CONCAT22(param_1, &local_20),
            );
        }
        // u_var4 = (uStack10 >> 0x10);
        uStack42 = (uStack10 + 0x4);
        struct_op_1028_87f0(
            param_1,
            param_3,
            CONCAT22(param_1, &local_15e),
            0x0,
            0x0,
            (-(local_16 == 0x0) & 0xffd3) + 0x60,
            &local_10,
            param_1,
            uStack42 & 0xffff | (uStack10 + 0x6) << 0x10,
            uStack30,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, &local_15e));
        local_15e = 0x389a;
        uStack348 = 0x1008;
        pass1_1008_3e76(
            CONCAT22(param_1, &local_10),
            local_16 + 0x1,
            local_14,
            (local_14 >> 0x10),
        );
        struct_op_1028_87f0(
            param_1,
            param_3,
            CONCAT22(param_1, &local_15e),
            0x0,
            0x0,
            0x60,
            &local_10,
            param_1,
            uStack42,
            uStack30,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, &local_15e));
    }
    return;
}

pub fn pass1_1028_1b1e(param_1: u32) {
    (param_1 + 0x14) = 0x7;
    return;
}

pub fn pass1_1028_1b2e(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    pass1_1030_dcf4(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_1be8(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    (param_1 + 0x22) = 0x0;
    CONCAT22(param_2, param_1) = 0x1eee;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_1c1c() -> u16 {
    return 0x0;
}

pub fn pass1_1028_1c22(param_1: u32) -> u16 {
    let u_var1: u16;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x20) != 0x0) && ((i_var2 + 0x12) == 0x5 || ((i_var2 + 0x12) == 0x6))) {
        if ((i_var2 + 0xc) == 0x16) {
            return 0x19;
        }
        if ((i_var2 + 0xc) == 0x17) {
            return 0x1a;
        }
    }
    u_var1 = pass1_1028_bc1c(param_1 & 0xffff | u_var3 << 0x10);
    return u_var1;
}

pub fn pass1_1028_1c66(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let u_var3: u32;

    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        ppcVar1 = (*param_1 + 0x64);
        i_var2 = (**ppcVar1)(param_4, param_1);
        if (i_var2 == 0x0) {
            return;
        }
        pass1_1028_cb04(param_1, param_2, param_3, param_5);
        if (i_var2 == 0x0) {
            i_var2 = 0x6;
            //       TODO: goto LAB_1028_1cbd;
        }
        pass1_1028_c952(param_1, param_2, param_3, param_5);
    }
    i_var2 = 0x5;
    //LAB_1028_1cbd:
    pass1_1028_bdac(param_1, i_var2, param_4);
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_1cca(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i32,
    param_7: u16,
) -> u16 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let local_e: [u8; 2];
    let local_c: i16;
    let local_a: i16;
    let local_8: u32;
    let uStack4: u16;

    local_8 = *param_2;
    uStack4 = (param_2 + 0x1);
    pass1_1008_3eb4(
        CONCAT22(param_7, &local_8),
        CONCAT22(param_7, local_e),
        CONCAT22(param_7, &local_c),
        CONCAT22(param_7, &local_a),
    );
    local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    u_var1 = pass1_1028_1e14(
        u_var2,
        u_var3,
        0x16,
        CONCAT22(param_7, &local_8),
        param_6,
        &local_8,
        param_3,
        param_7,
    );
    if (u_var1 == 0x0) {
        local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
        u_var1 = pass1_1028_1e14(
            u_var2,
            u_var3,
            0x16,
            CONCAT22(param_7, &local_8),
            param_6,
            &local_8,
            param_3,
            param_7,
        );
        if (u_var1 == 0x0) {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            u_var1 = pass1_1028_1e14(
                u_var2,
                u_var3,
                0x17,
                CONCAT22(param_7, &local_8),
                param_6,
                &local_8,
                param_3,
                param_7,
            );
            if (u_var1 == 0x0) {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                u_var1 = pass1_1028_1e14(
                    u_var2,
                    u_var3,
                    0x17,
                    CONCAT22(param_7, &local_8),
                    param_6,
                    &local_8,
                    param_3,
                    param_7,
                );
                if (u_var1 == 0x0) {
                    return u_var1;
                }
            }
        }
    }
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_1da4(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: i32,
    param_6: u16,
) {
    let i_var1: i16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u32;
    let local_4: [u8; 2];

    u_var4 = pass1_1030_bcae(local_4, param_6);
    // u_var3 = (u_var4 >> 0x10);
    i_var1 = u_var4;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var4 = (i_var1 + 0x10);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
    pu_var2 = local_4;
    pass1_1030_bcde(
        param_6,
        pu_var2,
        param_6,
        u_var4 & 0xffff | u_var3 << 0x10,
        param_3,
        param_5,
    );
    if (pu_var2 < 0x0) {
        ctx.PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_1e14(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: U32Ptr,
    param_5: i32,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) -> u16 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;

    pass1_1030_627e(
        param_8,
        param_6,
        param_7,
        ctx.PTR_LOOP_1050_5740,
        param_4,
        param_5,
    );
    u_var2 = param_7 | param_6;
    if (u_var2 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6, param_7);
        if ((u_var2 | param_6) != 0x0) {
            u_var3 = struct_op_1030_73a8(CONCAT22(u_var2, param_6));
            if (u_var3 != 0x0) {
                i_var1 = (u_var3 + 0xc);
                if (((i_var1 == 0x18) || (i_var1 == 0x3f)) || (i_var1 == param_3)) {
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}

pub fn pass1_1028_1e8a(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) -> u16 {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;

    // u_var1 = (param_1 >> 0x10);
    if (((param_1 + 0x1a) & 0x2) == 0x0) {
        u_var4 = 0x0;
        u_var5 = 0x23;
        u_var3 = 0x1;
        u_var2 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        pass1_1030_7d7c(
            u_var2,
            u_var3,
            CONCAT22(u_var5, u_var4),
            u_var2,
            (u_var2 >> 0x10),
            param_2,
            param_3,
            param_4,
        );
        pass1_1028_bdac(param_1, 0x6, 0x1030);
        return 0x0;
    }
    return 0x1;
}

pub fn pass1_1028_1ec8(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_1fc8(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: U32Ptr) {
    let u_var1: u32;
    let u_var2: u16;
    let extraout_dx: u16;

    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    u_var2 = 0x0;
    (param_1 + 0x20) = 0x0;
    (param_1 + 0x24) = 0x0;
    CONCAT22(param_2, param_1) = 0x2572;
    (param_1 + 0x2) = &USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_5, 0x1000);
    if ((param_5 | u_var2) == 0x0) {
        (param_1 + 0x20) = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(param_5, u_var2));
        (param_1 + 0x20) = u_var2;
        (param_1 + 0x22) = extraout_dx;
    }
    u_var1 = (param_1 + 0x20);
    (u_var1 + 0xa) = 0x0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_2042(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u32;
    let iVar5: &mut Struct602;
    let u_var5: u16;
    let u_var6: u32;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x2572;
    iVar5.field_0x2 = &USHORT_1050_1028;
    u_var4 = &iVar5.field_0x20;
    (u_var4 + 0xa) = 0x1;
    pu_var1 = iVar5.field_0x20;
    u_var2 = iVar5.field_0x22;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    if ((ctx.PTR_LOOP_1050_65e2 != 0x0) && (ctx.PTR_LOOP_1050_5a64 != 0x0)) {
        u_var6 = pass1_1028_b58e(param_1);
        pass1_1038_79f2(ctx.PTR_LOOP_1050_5a64, u_var6, param_2);
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_20b0() -> u16 {
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_20b6(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let auStack22: [u8; 2];
    let iStack20: i16;
    let iStack18: i16;
    let uStack16: u32;
    let uStack12: u16;
    let uStack10: u32;
    let uStack6: u32;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    // u_var4 = (param_1 >> 0x10);
    u_var3 = param_1;
    if ((u_var3 + 0x12) == 0x5) {
        uStack6 = pass1_1028_bb24(param_1 & 0xffff | u_var4 << 0x10);
        uStack10 = pass1_1028_b58e(param_1);
        // pu_var2 = (uStack10 >> 0x10);
        uStack16 = (uStack10 + 0xc);
        uStack12 = (uStack10 + 0x10);
        pass1_1008_3eb4(
            CONCAT22(param_5, &uStack16),
            CONCAT22(param_5, auStack22),
            CONCAT22(param_5, &iStack20),
            CONCAT22(param_5, &iStack18),
        );
        uStack16 = uStack16 & 0xffff | (iStack20 - 0x1) << 0x10;
        u_var1 = pass1_1028_21ba(
            u_var3,
            u_var4,
            CONCAT22(param_5, &uStack16),
            uStack6,
            &uStack16,
            pu_var2,
            param_5,
        );
        if (u_var1 == 0x0) {
            uStack16 = uStack16 & 0xffff | (iStack20 + 0x1) << 0x10;
            u_var1 = pass1_1028_21ba(
                u_var3,
                u_var4,
                CONCAT22(param_5, &uStack16),
                uStack6,
                &uStack16,
                pu_var2,
                param_5,
            );
            if (u_var1 == 0x0) {
                uStack16 = CONCAT22(iStack20, iStack18 + -0x1);
                u_var1 = pass1_1028_21ba(
                    u_var3,
                    u_var4,
                    CONCAT22(param_5, &uStack16),
                    uStack6,
                    &uStack16,
                    pu_var2,
                    param_5,
                );
                if (u_var1 == 0x0) {
                    uStack16 = uStack16 & 0xffff0000 | (iStack18 + 0x1);
                    u_var1 = pass1_1028_21ba(
                        u_var3,
                        u_var4,
                        CONCAT22(param_5, &uStack16),
                        uStack6,
                        &uStack16,
                        pu_var2,
                        param_5,
                    );
                    if (u_var1 == 0x0) {
                        return;
                    }
                }
            }
        }
        pass1_1038_79b2(ctx.PTR_LOOP_1050_5a64, uStack10, u_var1, pu_var2);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_21ba(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i32,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) -> u16 {
    let u_var1: u16;
    let u_var2: u32;

    pass1_1030_627e(
        param_7,
        param_5,
        param_6,
        ctx.PTR_LOOP_1050_5740,
        param_3,
        param_4,
    );
    u_var1 = param_6 | param_5;
    if (u_var1 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5, param_6);
        if ((u_var1 | param_5) != 0x0) {
            u_var2 = struct_op_1030_73a8(CONCAT22(u_var1, param_5));
            if ((u_var2 != 0x0) && ((u_var2 + 0xc) == 0x40)) {
                return 0x1;
            }
        }
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_2220(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: U32Ptr,
    param_5: i32,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) -> u16 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;

    pass1_1030_627e(
        param_8,
        param_6,
        param_7,
        ctx.PTR_LOOP_1050_5740,
        param_4,
        param_5,
    );
    u_var2 = param_7 | param_6;
    if (u_var2 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6, param_7);
        if ((u_var2 | param_6) != 0x0) {
            u_var3 = struct_op_1030_73a8(CONCAT22(u_var2, param_6));
            if ((u_var3 != 0x0)
                && ((
                    i_var1 = (u_var3 + 0xc),
                    i_var1 == 0x40 || (i_var1 == param_3),
                )))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_2290(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i32,
    param_7: u16,
) -> i16 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let local_e: [u8; 2];
    let local_c: i16;
    let local_a: i16;
    let local_8: u32;
    let uStack4: u16;

    local_8 = *param_2;
    uStack4 = (param_2 + 0x1);
    pass1_1008_3eb4(
        CONCAT22(param_7, &local_8),
        CONCAT22(param_7, local_e),
        CONCAT22(param_7, &local_c),
        CONCAT22(param_7, &local_a),
    );
    local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    i_var1 = pass1_1028_2220(
        u_var2,
        u_var3,
        0x16,
        CONCAT22(param_7, &local_8),
        param_6,
        &local_8,
        param_3,
        param_7,
    );
    if (i_var1 == 0x0) {
        local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
        i_var1 = pass1_1028_2220(
            u_var2,
            u_var3,
            0x16,
            CONCAT22(param_7, &local_8),
            param_6,
            &local_8,
            param_3,
            param_7,
        );
        if (i_var1 == 0x0) {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            i_var1 = pass1_1028_2220(
                u_var2,
                u_var3,
                0x17,
                CONCAT22(param_7, &local_8),
                param_6,
                &local_8,
                param_3,
                param_7,
            );
            if (i_var1 == 0x0) {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                i_var1 = pass1_1028_2220(
                    u_var2,
                    u_var3,
                    0x17,
                    CONCAT22(param_7, &local_8),
                    param_6,
                    &local_8,
                    param_3,
                    param_7,
                );
                if (i_var1 == 0x0) {
                    return i_var1;
                }
            }
        }
    }
    return 0x1;
}

pub fn pass1_1028_236a(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) -> u16 {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;

    // u_var1 = (param_1 >> 0x10);
    if (((param_1 + 0x1a) & 0x2) == 0x0) {
        u_var4 = 0x0;
        u_var5 = 0x23;
        u_var3 = 0x1;
        u_var2 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        pass1_1030_7d7c(
            u_var2,
            u_var3,
            CONCAT22(u_var5, u_var4),
            u_var2,
            (u_var2 >> 0x10),
            param_2,
            param_3,
            param_4,
        );
        pass1_1028_bdac(param_1, 0x6, 0x1030);
        return 0x0;
    }
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_23a8(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: i32,
    param_6: u16,
) {
    let i_var1: i16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u32;
    let local_4: [u8; 2];

    u_var4 = pass1_1030_bcae(local_4, param_6);
    // u_var3 = (u_var4 >> 0x10);
    i_var1 = u_var4;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var4 = (i_var1 + 0x10);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
    pu_var2 = local_4;
    pass1_1030_bcde(
        param_6,
        pu_var2,
        param_6,
        u_var4 & 0xffff | u_var3 << 0x10,
        param_3,
        param_5,
    );
    if (pu_var2 < 0x0) {
        ctx.PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub fn pass1_1028_2418(param_1: u32, param_2: u32, param_3: u16) -> bool {
    let u_var1: u32;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u32;
    let local_1c: [u16; 0x6];
    let uStack16: u16;
    let iStack14: i16;
    let uStack12: u16;
    let local_a: [u8; 8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (BVar2 != 0x0) {
        // u_var3 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x20));
        u_var1 = (param_1 + 0x20);
        local_1c[0] = (u_var1 + 0x8);
        uStack16 = local_1c[0];
        BVar2 = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), local_1c, param_3, 0x2, 0x1008);
        if (BVar2 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return BVar2;
        }
        loop {
            u_var4 = pass1_1008_5b12(local_a, param_3);
            iStack14 = u_var4;
            if (u_var4 == 0x0) {
                break;
            }
            pass1_1038_75ca(u_var4, param_2, iStack14, param_3);
            // uStack12 = (u_var4 >> 0x10);
            if (u_var4 == 0x0) {
                return u_var4;
            }
        }
        BVar2 = 0x1;
    }
    return BVar2;
}

pub fn pass1_1028_254c(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    pass1_1028_2042(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_25fc(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = (s_fem16_wav_1050_2644 + 0x8);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2626(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_26d6(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x2788;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2700(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u32;

    pass1_1028_be2a(param_1, param_2, param_3, param_4, param_5);
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        u_var2 = pass1_1028_b4f2(param_1 & 0xffff | u_var1 << 0x10);
        (u_var2 + 0x204) = 0x1;
    }
    return;
}

pub fn pass1_1028_272e(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    u_var1 = pass1_1028_b4f2(param_1);
    if ((param_1 + 0x12) == 0x5) {
        (u_var1 + 0x204) = 0x1;
    }
    return;
}

pub fn pass1_1028_2762(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_2812(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x2a92;
    (param_1 + 0x2) = &USHORT_1050_1028;
    (param_1 + 0xe) = (param_1 + 0xc);
    return CONCAT22(param_2, param_1);
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_2844(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i32,
    param_7: u16,
) -> u16 {
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let local_e: [u8; 2];
    let local_c: i16;
    let local_a: i16;
    let local_8: u32;
    let uStack4: u16;

    local_8 = *param_2;
    uStack4 = (param_2 + 0x1);
    pass1_1008_3eb4(
        CONCAT22(param_7, &local_8),
        CONCAT22(param_7, local_e),
        CONCAT22(param_7, &local_c),
        CONCAT22(param_7, &local_a),
    );
    local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
    u_var3 = param_1;
    // u_var4 = (param_1 >> 0x10);
    b_var1 = pass1_1028_c5a6(
        u_var3,
        u_var4,
        0x7b,
        CONCAT22(param_7, &local_8),
        param_6,
        &local_8,
        param_3,
        param_7,
    );
    if (b_var1 == 0x0) {
        u_var2 = pass1_1028_297c(
            param_1,
            CONCAT22(param_7, &local_8),
            param_6,
            &local_8,
            param_3,
            param_7,
        );
        if (u_var2 == 0x0) {
            local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
            b_var1 = pass1_1028_c5a6(
                u_var3,
                u_var4,
                0x7b,
                CONCAT22(param_7, &local_8),
                param_6,
                &local_8,
                param_3,
                param_7,
            );
            if (b_var1 == 0x0) {
                u_var2 = pass1_1028_297c(
                    param_1,
                    CONCAT22(param_7, &local_8),
                    param_6,
                    &local_8,
                    param_3,
                    param_7,
                );
                if (u_var2 == 0x0) {
                    local_8._0_2_ = local_a + -0x1;
                    local_8._2_2_ = local_c;
                    b_var1 = pass1_1028_c5a6(
                        u_var3,
                        u_var4,
                        0x7c,
                        CONCAT22(param_7, &local_8),
                        param_6,
                        &local_8,
                        param_3,
                        param_7,
                    );
                    if (b_var1 == 0x0) {
                        u_var2 = pass1_1028_297c(
                            param_1,
                            CONCAT22(param_7, &local_8),
                            param_6,
                            &local_8,
                            param_3,
                            param_7,
                        );
                        if (u_var2 == 0x0) {
                            local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                            b_var1 = pass1_1028_c5a6(
                                u_var3,
                                u_var4,
                                0x7c,
                                CONCAT22(param_7, &local_8),
                                param_6,
                                &local_8,
                                param_3,
                                param_7,
                            );
                            if (b_var1 == 0x0) {
                                u_var3 = pass1_1028_297c(
                                    param_1,
                                    CONCAT22(param_7, &local_8),
                                    param_6,
                                    &local_8,
                                    param_3,
                                    param_7,
                                );
                                if (u_var3 == 0x0) {
                                    return u_var3;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_297c(
    param_1: u32,
    param_2: U32Ptr,
    param_3: i32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> u16 {
    let cVar1: u8;
    let u_var2: u16;
    let u_var3: u32;

    pass1_1028_c7b6(
        param_6,
        param_5,
        param_1,
        (param_1 >> 0x10),
        param_2,
        param_3,
    );
    if (param_4 == 0x0) {
        pass1_1030_627e(
            param_6,
            0x0,
            param_5,
            ctx.PTR_LOOP_1050_5740,
            param_2,
            param_3,
        );
        u_var2 = param_5 | param_4;
        if (u_var2 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4, param_5);
            u_var3 = struct_op_1030_73a8(CONCAT22(u_var2, param_4));
            u_var2 = (u_var3 + 0xc);
            if (u_var2 < 0x4b) {
                if (0x47 < u_var2) {
                    return 0x0;
                }
                if (u_var2 == 0x41) {
                    return 0x0;
                }
                if (u_var2 < 0x42) {
                    cVar1 = u_var2;
                    if (cVar1 < '5') {
                        if ('2' < cVar1) {
                            return 0x0;
                        }
                        cVar1 += -0x10;
                    } else {
                        cVar1 += -0x3e;
                    }
                    if (cVar1 == '\0') {
                        return 0x0;
                    }
                }
            } else {
                if (true) {
                    match (u_var2) {
                        0x4c | 0x4d | 0x4e | 0x60 | 0x61 | 0x62 | 0x63 | 0x6e | 0x73 | 0x74
                        | 0x75 | 0x76 | 0x77 | 0x78 | 0x79 => {} //             TODO: goto switchD_1028_2a0b_caseD_4c;
                    }
                }
            }
            return 0x1;
        }
    }
    // switchD_1028_2a0b_caseD_4c:
    return 0x0;
}

pub fn pass1_1028_2a6c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn struct_1028_2afa(param_1: U32Ptr) -> u16 {
    struct_1030_dc96(param_1);
    *param_1 = 0x2b74;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_2b1c(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1030_dcc2(param_1, param_2, param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x2b74;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2b4e(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    pass1_1030_dcf4(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_2bfe(
    param_1: &mut Struct179,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) {
    pass1_1028_0982(param_1, param_2, param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x341c;
    param_1.field_0x2 = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2c28(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u32,
    param_7: u32,
) -> u16 {
    let pu_var1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_e: u16;
    let local_c: u16;
    let local_a: u16;
    let local_8: u32;
    let uStack4: u16;

    pass1_1028_09d4(
        param_1, param_2, param_3, param_4, param_5, param_6, param_7,
    );
    if (param_2 != 0x0) {
        local_8 = *param_5;
        uStack4 = (param_5 + 0x4);
        pu_var2 = &local_e;
        pass1_1008_3eb4(
            CONCAT22(param_1, &local_8),
            CONCAT22(param_1, pu_var2),
            CONCAT22(param_1, &local_c),
            CONCAT22(param_1, &local_a),
        );
        pass1_1008_3e76(
            CONCAT22(param_1, &local_8),
            local_e,
            local_c - 0x1,
            local_a - 0x1,
        );
        pu_var1 = &local_8;
        u_var3 = param_4;
        // u_var4 = (param_4 >> 0x10);
        pass1_1028_c7b6(
            param_1,
            pu_var2,
            u_var3,
            u_var4,
            CONCAT22(param_1, pu_var1),
            param_7,
        );
        if (pu_var1 != 0x0) {
            pass1_1008_3e76(CONCAT22(param_1, &local_8), local_e, local_c - 0x1, local_a);
            pu_var1 = &local_8;
            pass1_1028_c7b6(
                param_1,
                pu_var2,
                u_var3,
                u_var4,
                CONCAT22(param_1, pu_var1),
                param_7,
            );
            if (pu_var1 != 0x0) {
                pass1_1008_3e76(
                    CONCAT22(param_1, &local_8),
                    local_e,
                    local_c - 0x1,
                    local_a + 0x1,
                );
                pu_var1 = &local_8;
                pass1_1028_c7b6(
                    param_1,
                    pu_var2,
                    u_var3,
                    u_var4,
                    CONCAT22(param_1, pu_var1),
                    param_7,
                );
                if (pu_var1 != 0x0) {
                    pass1_1008_3e76(CONCAT22(param_1, &local_8), local_e, local_c, local_a - 0x1);
                    pu_var1 = &local_8;
                    pass1_1028_c7b6(
                        param_1,
                        pu_var2,
                        u_var3,
                        u_var4,
                        CONCAT22(param_1, pu_var1),
                        param_7,
                    );
                    if (pu_var1 != 0x0) {
                        pass1_1008_3e76(CONCAT22(param_1, &local_8), local_e, local_c, local_a);
                        pu_var1 = &local_8;
                        pass1_1028_c7b6(
                            param_1,
                            pu_var2,
                            u_var3,
                            u_var4,
                            CONCAT22(param_1, pu_var1),
                            param_7,
                        );
                        if (pu_var1 != 0x0) {
                            pass1_1008_3e76(
                                CONCAT22(param_1, &local_8),
                                local_e,
                                local_c,
                                local_a + 0x1,
                            );
                            pu_var1 = &local_8;
                            pass1_1028_c7b6(
                                param_1,
                                pu_var2,
                                u_var3,
                                u_var4,
                                CONCAT22(param_1, pu_var1),
                                param_7,
                            );
                            if (pu_var1 != 0x0) {
                                pass1_1008_3e76(
                                    CONCAT22(param_1, &local_8),
                                    local_e,
                                    local_c + 0x1,
                                    local_a - 0x1,
                                );
                                pu_var1 = &local_8;
                                pass1_1028_c7b6(
                                    param_1,
                                    pu_var2,
                                    u_var3,
                                    u_var4,
                                    CONCAT22(param_1, pu_var1),
                                    param_7,
                                );
                                if (pu_var1 != 0x0) {
                                    pass1_1008_3e76(
                                        CONCAT22(param_1, &local_8),
                                        local_e,
                                        local_c + 0x1,
                                        local_a,
                                    );
                                    pu_var1 = &local_8;
                                    pass1_1028_c7b6(
                                        param_1,
                                        pu_var2,
                                        u_var3,
                                        u_var4,
                                        CONCAT22(param_1, pu_var1),
                                        param_7,
                                    );
                                    if (pu_var1 != 0x0) {
                                        pass1_1008_3e76(
                                            CONCAT22(param_1, &local_8),
                                            local_e,
                                            local_c + 0x1,
                                            local_a + 0x1,
                                        );
                                        pu_var1 = &local_8;
                                        pass1_1028_c7b6(
                                            param_1,
                                            pu_var2,
                                            u_var3,
                                            u_var4,
                                            CONCAT22(param_1, pu_var1),
                                            param_7,
                                        );
                                        if (pu_var1 != 0x0) {
                                            return 0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_2e40(
    param_1: U32Ptr,
    param_2: i16,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u8,
) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u32;

    pass1_1028_be9e(param_1, param_4, param_5, param_6, param_7);
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        pass1_1028_2f18(param_7, param_2, param_8, param_1 & 0xffff | u_var1 << 0x10);
        pass1_1028_3246(param_1, param_2, param_3, param_4, param_5, param_7);
        u_var3 = 0x50001;
        pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_7, param_3, param_5);
        pass1_1010_089e(param_7, pu_var2, u_var3, (u_var3 >> 0x10));
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_2e84(
    param_1: u16,
    param_2: u32,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) {
    let pu_var1: U32Ptr;
    let paVar2: &mut Struct67;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let iVar10: i16;

    pass1_1028_09b8(CONCAT22(param_2, param_1));
    if ((param_2 >> 0x10) == 0x0) {
        uVar9 = 0x0;
        iVar10 = 0x8;
        uVar7 = 0x1;
        uVar8 = 0x0;
        u_var5 = 0x0;
        iVar6 = 0x0;
        u_var4 = 0x0;
        paVar2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_5, param_3, param_4);
        // pu_var1 = (paVar2 >> 0x10);
        post_win_msg_1008_a0e4(
            paVar2,
            CONCAT22(u_var5, u_var4),
            iVar6,
            uVar7,
            CONCAT22(uVar9, uVar8),
            iVar10,
            0x1008,
            param_5,
        );
        u_var5 = 0x400;
        iVar6 = 0x3;
        u_var4 = 0x1;
        pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, pu_var1, param_4);
        // pu_var1 = (pu_var3 >> 0x10);
        pass1_1010_043a(pu_var3, CONCAT22(u_var5, u_var4), iVar6, param_5);
        pass1_1010_043a(pu_var3, 0x4000001, 0x4, param_5);
        pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_5, pu_var1, param_4);
        // pu_var1 = (pu_var3 >> 0x10);
        pass1_1010_ec84(pu_var3, param_5, param_6);
        pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_5, pu_var1, param_4);
        pass1_1010_9766(pu_var3, (pu_var3 >> 0x10), param_4, param_5);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_2f18(param_1: u16, param_2: i16, param_3: u8, param_4: u32) {
    let i_var1: i16;
    let pu_var2: u32;
    let extraout_dx: u16;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let local_944: [u8; 124];
    let local_820: [u8; 124];
    let local_6fc: [u8; 124];
    let local_5d8: [u8; 124];
    let local_4b4: [u8; 124];
    let local_390: u32;
    let local_38a: [u8; 124];
    let local_266: [u8; 124];
    let local_142: [u8; 124];
    let local_1e: u32;
    let local_1a: u16;
    let local_18: u32;
    let uStack20: u16;
    let uStack18: u32;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    uStack6 = pass1_1028_bb24(param_4);
    i_var1 = uStack6;
    pass1_1028_b58e(param_4);
    uStack10 = CONCAT22(extraout_dx, i_var1);
    uStack14 = (i_var1 + 0x2e);
    uStack18 = (uStack14 + 0x4);
    local_18 = (i_var1 + 0xc);
    uStack20 = (i_var1 + 0x10);
    pass1_1008_3eb4(
        CONCAT22(param_1, &local_18),
        CONCAT22(param_1, &local_1e),
        CONCAT22(param_1, &local_1e + 0x2),
        CONCAT22(param_1, &local_1a),
    );
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        local_1e._2_2_ - 0x1,
        local_1a - 0x1,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_142),
        0x0,
        0x0,
        0xd,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_142));
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        local_1e._2_2_ + 0x1,
        local_1a + 0x1,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_266),
        0x0,
        0x0,
        0xc,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_266));
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        local_1e._2_2_ + 0x1,
        local_1a - 0x1,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_38a),
        0x0,
        0x0,
        0xe,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_38a));
    pu_var5 = pass1_1008_3e54(
        CONCAT22(param_1, &local_390),
        local_1e,
        local_1e._2_2_ - 0x1,
        local_1a + 0x1,
    );
    // u_var3 = (pu_var5 >> 0x10);
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_4b4),
        0x0,
        0x0,
        0xb,
        &local_390,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_4b4));
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        local_1e._2_2_ - 0x1,
        local_1a,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_5d8),
        0x0,
        0x0,
        0x7a,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_5d8));
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        (local_1e >> 0x10),
        local_1a + 0x1,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_6fc),
        0x0,
        0x0,
        0x7a,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_6fc));
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        local_1e._2_2_ + 0x1,
        local_1a,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_820),
        0x0,
        0x0,
        0x7a,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_820));
    pass1_1008_3e76(
        CONCAT22(param_1, &local_18),
        local_1e,
        (local_1e >> 0x10),
        local_1a - 0x1,
    );
    struct_op_1028_87f0(
        param_1,
        param_3,
        CONCAT22(param_1, local_944),
        0x0,
        0x0,
        0x7a,
        &local_18,
        param_1,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_1, local_944));
    pu_var2 = &local_390;
    pass1_1030_627e(
        param_1,
        pu_var2,
        u_var3,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_1, pu_var2),
        uStack6,
    );
    // u_var4 = (uStack14 >> 0x10);
    (uStack14 + 0x10) = pu_var2;
    (uStack14 + 0x12) = u_var3;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_3246(
    param_1: u32,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let extraout_dx: U32Ptr;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let local_20: [u8; 6];
    let puStack26: U32Ptr;
    let uStack18: u16;
    let puStack16: U32Ptr;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_2);
    uStack10 = (param_2 + 0x2e);
    u_var2 = (uStack10 + 0x10);
    u_var5 = 0x0;
    iVar6 = 0x1;
    u_var4 = 0x1;
    pu_var3 = extraout_dx;
    uStack14 = u_var2;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    u_var1 = u_var2;
    uStack18 = u_var1;
    puStack16 = pu_var3;
    pass1_1030_7c50(
        u_var2 & 0xffff | ZEXT24(pu_var3) << 0x10,
        CONCAT22(u_var5, u_var4),
        iVar6,
        u_var1,
        pu_var3,
    );
    pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x1, 0x2, u_var1, pu_var3);
    pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x1, 0x3, u_var1, pu_var3);
    pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x1, 0x5, u_var1, pu_var3);
    puStack26 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_6, pu_var3, param_5);
    // pu_var3 = (puStack26 >> 0x10);
    u_var1 = puStack26;
    if ((u_var1 + 0x82) == 0x0) {
        pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x4, 0x4, u_var1, pu_var3);
    }
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x11,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x12,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x13,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x14,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0x14,
        0x15,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0x14,
        0x16,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x17,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x18,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0xc8,
        0x19,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0x14,
        0x1a,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0x14,
        0x1b,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    pass1_1030_7ddc(
        CONCAT22(puStack16, uStack18),
        0x14,
        0x1c,
        u_var1,
        pu_var3,
        param_4,
        param_5,
        param_6,
    );
    if ((uStack10 + 0x200) == 0x8000002) {
        pass1_1020_a43e(param_6, pu_var3, CONCAT22(param_6, local_20));
        pass1_1020_a89e(
            param_6,
            CONCAT22(param_6, local_20),
            (uStack6 + 0xc),
            (uStack6 >> 0x10),
        );
    }
    return;
}

pub fn pass1_1028_33f6(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn struct_1028_3484(param_1: U32Ptr) -> u16 {
    let in_DX: U32Ptr;

    struct_1028_0068(param_1, in_DX);
    *param_1 = 0x34f6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_34a6(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: U32Ptr,
) -> u16 {
    pass1_1028_00cc(param_1, param_2, param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x34f6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_34d0(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_0138(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_3580(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x3608;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_35aa() -> u16 {
    return 0x1;
}

pub fn pass1_1028_35b0(param_1: u32, param_2: i16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let u_var2: u16;

    u_var1 = pass1_1028_b58e(param_1);
    if (param_2 == 0x0) {
        u_var2 = 0x0;
    } else {
        u_var2 = 0x32;
    }
    pass1_1030_7d1c(
        u_var1,
        u_var2,
        0x230000,
        u_var1,
        (u_var1 >> 0x10),
        param_3,
        param_4,
        param_5,
    );
    return;
}

pub fn pass1_1028_35e2(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_3692(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) -> u16 {
    pass1_1028_3816(
        param_1, param_2, param_3, param_4, param_5, param_6, param_7,
    );
    CONCAT22(param_2, param_1) = 0x373e;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_36bc(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u32;
    let i_stack4: i16;

    u_var5 = CONCAT22(param_4, param_3);
    *param_2 = 0x0;
    // u_var4 = (param_1 >> 0x10);
    if ((param_1 + 0x28) != 0x0) {
        i_stack4 = 0x4;
        loop {
            if (0x1c < i_stack4) {
                break;
            }
            u_var3 = (param_1 + 0x28);
            u_var5 = pass1_1020_bae6(
                u_var3,
                CONCAT22(i_stack4, (u_var3 >> 0x10)),
                u_var5,
                (u_var5 >> 0x10),
                param_5,
            );
            u_var2 = param_2;
            param_2 = param_2 + u_var5;
            pi_var1 = (param_2 + 0x2);
            *pi_var1 = *pi_var1 + (u_var5 >> 0x10) + CARRY2(u_var2, u_var5);
            if (0xf9 < *param_2) {
                return 0x1;
            }
            i_stack4 += 0x1;
        }
    }
    return 0x0;
}

pub fn pass1_1028_3718(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_388e(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_3816(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    let u_var1: u16;
    let u_var2: u16;

    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    u_var1 = 0x0;
    (param_1 + 0x20) = 0x0;
    (param_1 + 0x24) = 0x0;
    (param_1 + 0x28) = 0x0;
    CONCAT22(param_2, param_1) = 0x3e2c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    mem_op_1000_179c(0xa, param_5, 0x1000);
    u_var2 = param_5 | u_var1;
    if (u_var2 == 0x0) {
        (param_1 + 0x28) = 0x0;
    } else {
        pass1_1020_ba3e(CONCAT22(param_5, u_var1), 0x5, 0x5, param_7, param_6);
        (param_1 + 0x28) = u_var1;
        (param_1 + 0x2a) = u_var2;
    }
    return;
}

pub fn pass1_1028_388e(param_1: U32Ptr) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x3e2c;
    (i_var3 + 0x2) = &USHORT_1050_1028;
    paVar2 = (i_var3 + 0x28);
    u_var1 = (i_var3 + 0x2a);
    if ((u_var1 | paVar2) != 0x0) {
        fn_ptr_1020_ba7e((paVar2 & 0xffff | u_var1 << 0x10));
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_38d4(
    param_1: U32Ptr,
    param_2: U32Ptr,
    param_3: u32,
    param_4: u32,
    param_5: i16,
    param_6: u16,
    param_7: u16,
) -> u16 {
    let ppcVar1: u32;
    let BVar2: bool;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;

    u_var4 = param_1;
    // u_var5 = (param_1 >> 0x10);
    pass1_1028_c7b6(param_7, param_6, u_var4, u_var5, param_2, param_4);
    if ((param_5 == 0x5) || (param_5 == 0x6)) {
        ppcVar1 = (*param_1 + 0x60);
        u_var3 = (**ppcVar1)();
        if (u_var3 != 0x0) {
            pass1_1028_c23e(
                u_var4,
                u_var5,
                param_2,
                param_3,
                param_4,
                u_var3,
                (u_var3 >> 0x10),
                param_7,
            );
            if (u_var3 != 0x0) {
                BVar2 = pass1_1028_c314(
                    param_7,
                    u_var3,
                    (u_var3 >> 0x10),
                    u_var4,
                    u_var5,
                    param_2,
                    param_3,
                    (param_3 >> 0x10),
                    param_4,
                );
                if (BVar2 != 0x0) {
                    return 0x1;
                }
            }
        }
    } else {
        ctx.PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_3958(
    param_1: u32,
    param_2: i16,
    param_3: u16,
    param_4: i16,
    param_5: u16,
    param_6: u16,
) {
    let plVar1: &i32;
    let qVar2: u64;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let uStack52: u32;
    let local_2c: [u16; 0x2];
    let local_28: u32;
    let iStack36: i16;
    let uStack34: u32;
    let uStack30: u32;
    let uStack22: u16;
    let uStack20: u16;
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: u32;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_b58e(param_1);
    puStack10 = (param_2 + 0x22);
    u_var5 = (param_2 + 0x24);
    u_var6 = u_var5;
    if ((u_var5 | puStack10) != 0x0) {
        i_stack6 = param_2;
        uStack4 = param_3;
        if (ctx.PTR_LOOP_1050_574c != 0x0) {
            uStack30 = (puStack10 + 0x4);
            // TODO: refactor for loop
            // for (uStack34 = 0x0; uStack34 < uStack30; uStack34 += 0x1) {
            //   pass1_1020_bb16(puStack10,CONCAT22(param_6,local_2c),
            //                   CONCAT22(param_6,&local_28),uStack34);
            // }
        }
        uStack14 = (i_stack6 + 0x2e);
        uStack18 = *ctx.PTR_LOOP_1050_65e2;
        uStack20 = uStack18 & 0x1;
        // TODO: refactor for loop
        //     for (uStack22 = 0x4; uStack22 < 0xe; uStack22 += 0x1) {
        //       local_2c[0] = uStack22;
        //       local_28 = pass1_1020_bae6(puStack10,
        //                                  CONCAT22(uStack22,(puStack10 >> 0x10)),
        //                                  uStack22,u_var6,param_6);
        //       u_var5 = (local_28 >> 0x10) | local_28;
        //       u_var6 = u_var5;
        //       if (u_var5 != 0x0) {
        //         pass1_1020_bb8a(puStack10,0x0,local_2c[0] << 0x10,param_5,param_6);
        //         u_var5 = -(local_28._2_2_ + (local_28 != 0x0));
        //         u_var6 = u_var5;
        //         uStack34 = CONCAT22(u_var5,-local_28);
        //         pass1_1038_5694(uStack14,CONCAT22(u_var5,-local_28),local_2c[0]);
        //         uStack30 = 0x0;
        //         iStack36 = 0x0;
        //         iVar7 = param_1;
        //         uVar8 = (param_1 >> 0x10);
        //         match(uStack22) {
        //         0x4 =>
        //           uStack30 = local_28 >> 0x1;
        //           if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
        //             uStack30 = 0x1;
        //           }
        //           iStack36 = 0x11;
        //           break;
        //         0x5 =>
        //           uStack30 = local_28 >> 0x1;
        //           if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
        //             uStack30 = 0x1;
        //           }
        //           iStack36 = 0x12;
        //           break;
        //         0x6 =>
        //           uStack30 = local_28 >> 0x1;
        //           if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
        //             uStack30 = 0x1;
        //           }
        //           iStack36 = 0x13;
        //           break;
        //         0x7 =>
        //           uStack30 = local_28 >> 0x1;
        //           if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
        //             uStack30 = 0x1;
        //           }
        //           iStack36 = 0x14;
        //           break;
        //         0x8 =>
        //           uStack30 = local_28;
        //           iStack36 = 0x1a;
        //           break;
        //         0x9 =>
        //           uStack30 = local_28;
        //           iStack36 = 0x1b;
        //           break;
        //         0xa =>
        //           uStack30 = local_28;
        //           iStack36 = 0x1c;
        //           break;
        //         0xb =>
        //           uStack30 = local_28;
        //           iStack36 = 0x17;
        //           break;
        //         0xc =>
        //           iStack36 = 0x18;
        //           uStack30 = local_28;
        //           plVar1 = (iVar7 + 0x20);
        //           *plVar1 = *plVar1 + local_28;
        //           u_var5 = (iVar7 + 0x20);
        //           u_var3 = (iVar7 + 0x22);
        //           u_var4 = u_var5 >> 0x1 | ((u_var3 & 0x1) != 0x0) << 0xf;
        //           uStack52 = CONCAT22(u_var3 >> 0x1,u_var4);
        //           u_var4 = (u_var3 & 0xfffe) + CARRY2(u_var4,u_var4);
        //           u_var6 = u_var4;
        //           param_4 = (u_var3 - u_var4) - (u_var5 < (u_var5 & 0xfffe));
        //           (iVar7 + 0x20) = u_var5 - (u_var5 & 0xfffe);
        //           (iVar7 + 0x22) = param_4;
        //           if (uStack52 != 0x0) {
        //             uVar9 = 0x15;
        // //LAB_1028_3b14:
        //             uStack30 = local_28;
        //             pass1_1020_bb8a(*(long **)(iVar7 + 0x28),uStack52,
        //                             CONCAT22(uVar9,(uStack52 >> 0x10)),param_5,param_6
        //                            );
        //           }
        //           break;
        //         0xd =>
        //           iStack36 = 0x19;
        //           uStack30 = local_28;
        //           plVar1 = (iVar7 + 0x24);
        //           *plVar1 = *plVar1 + local_28;
        //           u_var5 = (iVar7 + 0x24);
        //           qVar2 = (local_28 & 0xffff0000 | u_var5) / 0x3;
        //           uStack52 = qVar2;
        //           uStack52._2_2_ = (qVar2 >> 0x10);
        //           u_var3 = qVar2;
        //           u_var4 = uStack52._2_2_ * 0x3 + CARRY2(u_var3,u_var3) +
        //                   CARRY2(u_var3 * 0x2,u_var3);
        //           u_var6 = u_var4;
        //           param_4 = u_var5 + u_var3 * -0x3;
        //           param_5 = ((iVar7 + 0x26) - u_var4) - (u_var5 < u_var3 * 0x3);
        //           (iVar7 + 0x24) = param_4;
        //           (iVar7 + 0x26) = param_5;
        //           if (uStack52 != 0x0) {
        //             uVar9 = 0x16;
        // //             TODO: goto LAB_1028_3b14;
        //           }
        //         }
        //         if (((uStack30._2_2_ | uStack30) != 0x0) && (iStack36 != 0x0)) {
        //           pass1_1020_bb70(*(long **)(iVar7 + 0x28),uStack30,
        //                           CONCAT22(iStack36,uStack30._2_2_),param_5,param_4,param_6);
        //         }
        //       }
        //     }
    }
    return;
}

pub fn pass1_1028_3c32(param_1: U32Ptr) -> u32 {
    let ppcVar1: u32;
    let i_var2: i16;
    let local_6: u16;
    let i_stack4: i16;

    ppcVar1 = (*param_1 + 0x40);
    i_var2 = (**ppcVar1)();
    if (i_var2 != 0x0) {
        return 0x0;
    }
    return CONCAT22(-(0x3e8 < local_6) - i_stack4, 0x3e8 - local_6);
}

pub fn pass1_1028_3c60(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16, param_5: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;
    let local_10: i32;
    let local_c: [u8; 4];
    let iStack8: i16;
    let uStack6: u16;
    let uStack4: u16;

    u_var6 = CONCAT22(param_4, param_3);
    *param_2 = 0x0;
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x28) != 0x0) {
        iStack8 = 0x4;
        loop {
            if (0x1c < iStack8) {
                break;
            }
            u_var3 = (i_var4 + 0x28);
            u_var6 = pass1_1020_bae6(
                u_var3,
                CONCAT22(iStack8, (u_var3 >> 0x10)),
                u_var6,
                (u_var6 >> 0x10),
                param_5,
            );
            u_var2 = param_2;
            param_2 = param_2 + u_var6;
            pi_var1 = (param_2 + 0x2);
            *pi_var1 = *pi_var1 + (u_var6 >> 0x10) + CARRY2(u_var2, u_var6);
            if (0x3e7 < *param_2) {
                return;
            }
            iStack8 += 0x1;
        }
    }
    u_var6 = (i_var4 + 0x28);
    uStack4 = (u_var6 + 0x4);
    uStack6 = 0x0;
    loop {
        if (uStack4 <= uStack6) {
            return;
        }
        pass1_1020_bb16(
            (i_var4 + 0x28),
            CONCAT22(param_5, &local_10),
            CONCAT22(param_5, local_c),
            uStack6,
        );
        *param_2 = *param_2 + local_10;
        if (0x3e7 < *param_2) {
            break;
        }
        uStack6 += 0x1;
    }
    return;
}

pub fn pass1_1028_3d92(
    param_1: u32,
    param_2: u32,
    param_3: i16,
    param_4: U32Ptr,
    param_5: u16,
    param_6: u16,
) {
    let i_var1: i16;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_6, param_4);
    if (param_3 != 0x0) {
        i_var1 = param_1;
        // u_var3 = (param_1 >> 0x10);
        // u_var4 = (param_2 >> 0x10);
        BVar2 = read_file_1008_7dee(param_2, u_var4, i_var1 + 0x20, 0x0, u_var3, 0x4, 0x1008);
        if (BVar2 != 0x0) {
            BVar2 = read_file_1008_7dee(param_2, u_var4, i_var1 + 0x24, 0x0, u_var3, 0x4, 0x1008);
            if (BVar2 != 0x0) {
                u_var3 = pass1_1008_7ad4(param_2, (i_var1 + 0x28), param_5, 0x1008, param_6);
                if (u_var3 != 0x0) {
                    return;
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

pub fn pass1_1028_3e06(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_388e(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_3ec8(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    CONCAT22(param_2, param_1) = 0x4004;
    (param_1 + 0x2) = &USHORT_1050_1028;
    pass1_1028_3fa2(CONCAT22(param_2, param_1));
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_3f04(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    u_var6 = 0x1f;
    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(param_3, param_2);
    uStack10 = pass1_1030_7c28(
        CONCAT22(param_3, param_2),
        u_var6,
        param_2,
        param_3,
        param_6,
    );
    // pu_var3 = (uStack10 >> 0x10);
    u_var2 = uStack10 & 0xffff;
    pass1_1030_7d1c(
        uStack6, 0x0, 0x1f0000, u_var2, pu_var3, param_4, param_5, param_6,
    );
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0xc) != 0x22) {
        if ((i_var4 + 0xc) == 0x23) {
            u_var1 = 0xa;
        } else {
            u_var1 = 0x5;
        }
        uStack14 = u_var1;
        uStack10 += (i_var4 + 0x20);
        (i_var4 + 0x20) = uStack10 % u_var1;
        u_var2 = uStack10 / uStack14;
        pu_var3 = (uStack10 % uStack14);
        uStack10 += u_var2;
    }
    pass1_1030_7ddc(
        uStack6, uStack10, 0x21, u_var2, pu_var3, param_4, param_5, param_6,
    );
    return;
}

pub fn pass1_1028_3fa2(param_1: u32) {
    let u_var1: u16;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xc) != 0x22) {
        if ((i_var2 + 0xc) == 0x23) {
            u_var1 = 0xa;
        } else {
            u_var1 = 0x5;
        }
        u_var1 >>= 0x1;
        pass1_1008_612e(0x0, u_var1, u_var1);
        (i_var2 + 0x20) = u_var1;
        (i_var2 + 0x22) = u_var1 >> 0xf;
    }
    return;
}

pub fn pass1_1028_3fde(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_408e(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x42ec;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_40b8(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let extraout_dx: u16;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: u32;
    let uStack54: u32;
    let local_2c: [u8; 6];
    let puStack38: u32;
    let uStack34: u32;
    let puStack26: u32;
    let uStack24: u32;
    let local_14: u32;
    let iStack16: i16;
    let iStack14: i16;
    let local_c: u32;
    let iStack8: i16;
    let uStack6: u16;
    let uStack4: u16;

    pass1_1028_b58e(param_1);
    local_14 = (param_2 + 0xc);
    iStack8 = (param_2 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34._2_2_, &local_14);
    iStack16 = iStack8 + 0x1;
    puVar7 = CONCAT22(param_3, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    uStack6 = param_2;
    uStack4 = extraout_dx;
    u_var6 = pass1_1028_bb24(param_1);
    // u_var5 = (u_var6 >> 0x10);
    pu_var2 = &local_14;
    pass1_1030_64ce(
        param_3,
        pu_var2,
        u_var5,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_3, pu_var2),
        u_var6 & 0xffff | u_var5 << 0x10,
        puVar7,
    );
    uStack34 = *pu_var2;
    u_var5 = (pu_var2 + 0x2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    u_var3 = uStack54._3_1_;
    uStack24 = uStack34;
    if (uStack54._3_1_ != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack34);
        uStack54 = CONCAT22(u_var5, u_var3);
        u_var4 = pass1_1030_6fa0(CONCAT22(u_var5, u_var3));
        if (u_var4 == 0x64) {
            puStack38 = struct_op_1030_73a8(uStack54);
            ppcVar1 = (*puStack38 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b514(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4194(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let pu_var3: U32Ptr;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    u_var2 = pass1_1028_b4f2(param_1);
    // pu_var1 = (u_var2 >> 0x10);
    if (((u_var2 + 0x200) != 0x8000002) && ((param_1 + 0x12) == 0x5)) {
        pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, pu_var1, param_3);
        pass1_1010_043a(pu_var3, (u_var2 + 0x4), 0xe, param_5);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_41ea(param_1: u32, param_2: i16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let extraout_dx: u16;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: u32;
    let uStack54: u32;
    let local_2c: [u8; 6];
    let puStack38: u32;
    let uStack34: u32;
    let puStack26: u32;
    let uStack24: u32;
    let local_14: u32;
    let iStack16: i16;
    let iStack14: i16;
    let local_c: u32;
    let iStack8: i16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_14 = (param_2 + 0xc);
    iStack8 = (param_2 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34._2_2_, &local_14);
    iStack16 = iStack8 + 0x1;
    puVar7 = CONCAT22(param_3, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    i_stack6 = param_2;
    uStack4 = extraout_dx;
    u_var6 = pass1_1028_bb24(param_1);
    // u_var5 = (u_var6 >> 0x10);
    pu_var2 = &local_14;
    pass1_1030_64ce(
        param_3,
        pu_var2,
        u_var5,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_3, pu_var2),
        u_var6 & 0xffff | u_var5 << 0x10,
        puVar7,
    );
    uStack34 = *pu_var2;
    u_var5 = (pu_var2 + 0x2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    u_var3 = uStack54._3_1_;
    if (uStack54._3_1_ != 0x0) {
        uStack24 = uStack34;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack34);
        uStack54 = CONCAT22(u_var5, u_var3);
        u_var4 = pass1_1030_6fa0(CONCAT22(u_var5, u_var3));
        if (u_var4 == 0x64) {
            puStack38 = struct_op_1030_73a8(uStack54);
            ppcVar1 = (*puStack38 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}

pub fn pass1_1028_42ca(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_4376(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x446a;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_43a0(param_1: u32) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if (((param_1 + 0x12) != 0x6) && ((param_1 + 0x12) != 0x5)) {
        return 0x0;
    }
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_43c2(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
) {
    let pu_var1: U32Ptr;

    pass1_1028_bd38(CONCAT22(param_2, param_1), param_4, param_6);
    if (param_3 == 0x0) {
        pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_6, param_4, param_5);
        pass1_1010_988c(pu_var1, (param_1 + 0xc));
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_43f6(
    param_1: u32,
    param_2: i16,
    param_3: U32Ptr,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u32;
    let u_var4: u16;

    u_var1 = 0x83;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x9, param_6, param_3, param_5);
    u_var1 = pass1_1010_65d0(param_6, pu_var2, u_var1);
    if (0x0 < u_var1) {
        u_var3 = pass1_1028_b58e(param_1);
        if (param_2 == 0x0) {
            u_var4 = 0x0;
        } else {
            u_var4 = 0x3e8;
        }
        pass1_1030_7d1c(
            u_var3,
            u_var4,
            0x230000,
            u_var3,
            (u_var3 >> 0x10),
            param_4,
            param_5,
            param_6,
        );
    }
    return;
}

pub fn pass1_1028_4444(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_44fe(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    CONCAT22(param_2, param_1) = 0x4836;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_4530(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x4836;
    (i_var4 + 0x2) = &USHORT_1050_1028;
    pu_var1 = (i_var4 + 0x20);
    u_var2 = (i_var4 + 0x22);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_456e(param_1: u32, param_2: u32, param_3: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    pass1_1028_b46e(param_1, param_2, param_3);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0x20);
    u_var2 = (i_var4 + 0x22);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    (i_var4 + 0x20) = 0x0;
    return;
}

pub fn pass1_1028_45b0(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: i16;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        u_var4 = 0x0;
        iVar5 = 0x4;
        u_var3 = 0x2;
        u_var2 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        pass1_1030_7c50(
            u_var2,
            CONCAT22(u_var4, u_var3),
            iVar5,
            u_var2,
            (u_var2 >> 0x10),
        );
    }
    return;
}

pub fn pass1_1028_45e2(param_1: u32, param_2: u16, param_3: i16, param_4: u16) -> u32 {
    pass1_1028_478a(param_1, param_2, param_4);
    return CONCAT22(-(0x3e8 < param_2) - param_3, 0x3e8 - param_2);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_45fe(param_1: u32, param_2: i16, param_3: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let pu_var3: u32;
    let extraout_dx: u16;
    let extraout_DX_00: U32Ptr;
    let puVar4: U32Ptr;
    let extraout_DX_01: u16;
    let u_var5: u16;
    let iVar6: &mut Struct312;
    let iVar5: &mut Struct314;
    let u_var6: u16;
    let uVar7: u16;
    let paStack44: &mut Struct99;
    let local_28: i32;
    let puStack34: u32;
    let puStack32: U32Ptr;
    let paStack30: &mut Struct99;
    let local_1a: [i16; 0x4];
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: u32;
    let uStack6: u32;
    let u_var2: &mut Struct313;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_2);
    puStack10 = (param_2 + 0x22);
    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    // WARNING: Load size is inaccurate
    pu_var3 = iVar6.field_0x20;
    puVar4 = (&iVar6.field_0x20 + 0x2);
    paStack30 = CONCAT22(puVar4, pu_var3);
    puStack34 = pu_var3;
    puStack32 = puVar4;
    if ((puVar4 | pu_var3) != 0x0) {
        ppcVar2 = *pu_var3;
        (**ppcVar2)();
        puVar4 = extraout_DX_00;
    }
    mem_op_1000_179c(0xc, puVar4, 0x1000);
    puStack34 = pu_var3;
    puStack32 = puVar4;
    if ((puVar4 | pu_var3) == 0x0) {
        pu_var3 = 0x0;
        uVar7 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar4, pu_var3));
        uVar7 = extraout_DX_01;
    }
    &iVar6.field_0x20 = pu_var3;
    (&iVar6.field_0x20 + 0x2) = uVar7;
    if (puStack10 != 0x0) {
        uStack14 = (puStack10 + 0x4);
        // TODO: refactor for loop
        // for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
        //   pass1_1020_bb16(puStack10,
        //                   CONCAT13((param_3 >> 0x8),
        //                                     CONCAT12(param_3,&local_28)),
        //                   CONCAT22(param_3,local_1a),uStack18);
        //   if ((local_28 != 0x0) && (local_1a[0] != 0x0)) {
        //     paStack30 = pass1_1000_07fc(0x1000,PTR_LOOP_1050_68a2);
        //     u_var5 = (paStack30 >> 0x10);
        //     u_var2 = paStack30;
        //     if ((u_var5 | u_var2) == 0x0) {
        //       paStack44 = 0x0;
        //     }
        //     else {
        //       paStack30.field_0x0 = 0x389a;
        //       u_var2.field_0x2 = 0x1008;
        //       u_var2.field_0x4 = 0x0;
        //       u_var2.field_0x6 = 0x0;
        //       u_var2.field_0x8 = 0x0;
        //       u_var2.field_0xa = 0x0;
        //       u_var2.field_0xc = 0x0;
        //       paStack30.field_0x0 = 0x56ce;
        //       u_var2.field_0x2 = 0x1018;
        //       paStack44 = paStack30;
        //     }
        //     uVar7 = (paStack44 >> 0x10);
        //     iVar5 = paStack44;
        //     iVar5.field_0x4 = local_1a[0];
        //     iVar5.field_0xa = local_28;
        //     iVar5.field_0xc = local_28;
        //     pu_var1 = iVar6.field_0x20;
        //     ppcVar2 = (*iVar6.field_0x20 + 0x8);
        //     (**ppcVar2)(0x1000,pu_var1,(pu_var1 >> 0x10),iVar5,uVar7);
        //   }
        // }
    }
    return;
}

pub fn pass1_1028_4768(param_1: u32, param_2: u16, param_3: i16, param_4: u16) -> u16 {
    pass1_1028_478a(param_1, param_2, param_4);
    if ((param_3 == 0x0) && (param_2 < 0x3e8)) {
        return 0x0;
    }
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_478a(param_1: u32, param_2: i16, param_3: u16) {
    let extraout_dx: u16;
    let local_1e: i32;
    let local_1a: [i16; 0x4];
    let uStack18: u16;
    let uStack16: u16;
    let lStack14: i32;
    let puStack10: u32;
    let uStack6: u32;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_2);
    puStack10 = (param_2 + 0x22);
    lStack14 = 0x0;
    if (((param_2 + 0x24) | puStack10) == 0x0) {
        return;
    }
    uStack16 = (puStack10 + 0x4);
    // TODO: refactor for loop
    // for (uStack18 = 0x0; uStack18 < uStack16; uStack18 += 0x1) {
    //   pass1_1020_bb16(puStack10,CONCAT22(param_3,&local_1e),
    //                   CONCAT22(param_3,local_1a),uStack18);
    //   if (0x0 < local_1a[0]) {
    //     lStack14 += local_1e;
    //   }
    // }
    return;
}

pub fn pass1_1028_4810(param_1: &mut Struct18, param_2: u8) {
    pass1_1028_4530(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_48c0(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = &ctx.PTR_LOOP_1050_4942;
    (param_1 + 0x2) = &USHORT_1050_1028;
    (param_1 + 0xe) = (param_1 + 0xc);
    (param_1 + 0x10) = (param_1 + 0xc);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_48fa(param_1: U32Ptr, param_2: u16) {
    pass1_1028_bdac(param_1, 0x0, param_2);
    return;
}

pub fn pass1_1028_4920(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_49de(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x4b1c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x20), 0x0, 0xa);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_4a1a(param_1: u32, param_2: u32, param_3: u16) {
    let b_var1: bool;

    b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if ((b_var1 != 0x0)
        && (
            b_var1 = write_to_file_1008_7e1c(
                param_2,
                (param_2 >> 0x10),
                param_1 + 0x20,
                (param_1 >> 0x10),
                0xa,
                0x1008,
            ),
            b_var1 == 0x0,
        ))
    {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
    }
    return;
}

pub fn pass1_1028_4a5a(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let b_var1: bool;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if ((param_3 != 0x0)
        && (
            b_var1 = read_file_1008_7dee(
                param_2,
                (param_2 >> 0x10),
                param_1 + 0x20,
                0x0,
                (param_1 >> 0x10),
                0xa,
                0x1008,
            ),
            b_var1 == 0x0,
        ))
    {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn pass1_1028_4a9a(param_1: u32, param_2: i16) -> u16 {
    return (param_1 + 0x20 + param_2 * 0x2);
}

pub fn pass1_1028_4ab2(param_1: u32, param_2: u16, param_3: i16) {
    (param_1 + param_3 * 0x2 + 0x20) = param_2;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4aca(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let pu_var1: U32Ptr;

    if ((param_1 + 0x20) != 0x0) {
        pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, param_2, param_3);
        pass1_1010_ed3e(pu_var1);
        pass1_1030_2a2c(pu_var1, (pu_var1 >> 0x10), param_3, param_4);
    }
    return;
}

pub fn pass1_1028_4af6(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_4ba6(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = (s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_4bd0(param_1: u32) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if (((param_1 + 0x12) != 0x6) && ((param_1 + 0x12) != 0x5)) {
        return 0x0;
    }
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4bf2(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u32;
    let puVar6: u32;
    let uStack54: u32;
    let local_2c: [u8; 6];
    let puStack38: u32;
    let uStack34: u32;
    let puStack26: u32;
    let uStack24: u32;
    let local_14: u32;
    let iStack16: i16;
    let iStack14: i16;
    let local_c: u32;
    let iStack8: i16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_b58e(param_1);
    local_14 = (param_3 + 0xc);
    iStack8 = (param_3 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34._2_2_, &local_14);
    iStack16 = iStack8 + 0x1;
    puVar6 = CONCAT22(param_4, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    i_stack6 = param_3;
    uStack4 = extraout_dx;
    u_var5 = pass1_1028_bb24(param_1);
    // u_var4 = (u_var5 >> 0x10);
    pu_var2 = &local_14;
    pass1_1030_64ce(
        param_4,
        pu_var2,
        u_var4,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_4, pu_var2),
        u_var5 & 0xffff | u_var4 << 0x10,
        puVar6,
    );
    uStack34 = *pu_var2;
    u_var4 = (pu_var2 + 0x2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    u_var3 = uStack54._3_1_;
    uStack24 = uStack34;
    if (uStack54._3_1_ != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack34);
        uStack54 = CONCAT22(u_var4, u_var3);
        u_var3 = pass1_1030_6fa0(CONCAT22(u_var4, u_var3));
        if ((u_var3 == 0x62) || (u_var3 == 0x63)) {
            puStack38 = struct_op_1030_73a8(uStack54);
            u_var3 = puStack38;
            ppcVar1 = (*puStack38 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(param_1, param_2, u_var3);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4cd6(param_1: u32, param_2: i16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let extraout_dx: u16;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: u32;
    let uStack54: u32;
    let local_2c: [u8; 6];
    let puStack38: u32;
    let uStack34: u32;
    let puStack26: u32;
    let uStack24: u32;
    let local_14: u32;
    let iStack16: i16;
    let iStack14: i16;
    let local_c: u32;
    let iStack8: i16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_14 = (param_2 + 0xc);
    iStack8 = (param_2 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34._2_2_, &local_14);
    iStack16 = iStack8 + 0x1;
    puVar7 = CONCAT22(param_3, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    i_stack6 = param_2;
    uStack4 = extraout_dx;
    u_var6 = pass1_1028_bb24(param_1);
    // u_var5 = (u_var6 >> 0x10);
    pu_var2 = &local_14;
    pass1_1030_64ce(
        param_3,
        pu_var2,
        u_var5,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_3, pu_var2),
        u_var6 & 0xffff | u_var5 << 0x10,
        puVar7,
    );
    uStack34 = *pu_var2;
    u_var5 = (pu_var2 + 0x2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    u_var3 = uStack54._3_1_;
    if (uStack54._3_1_ != 0x0) {
        uStack24 = uStack34;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack34);
        uStack54 = CONCAT22(u_var5, u_var3);
        u_var4 = pass1_1030_6fa0(CONCAT22(u_var5, u_var3));
        if ((u_var4 == 0x62) || (u_var4 == 0x63)) {
            puStack38 = struct_op_1030_73a8(uStack54);
            ppcVar1 = (*puStack38 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4db2(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
    param_7: u8,
) {
    let b_var1: bool;
    let piVar2: U32Ptr;
    let extraout_dx: u16;
    let pu_var3: U32Ptr;
    let piVar4: U32Ptr;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let local_14e: [u8; 124];
    let uStack42: u32;
    let uStack38: u32;
    let local_22: i16;
    let local_20: [u8; 2];
    let local_1e: [u8; 2];
    let local_1c: u32;
    let iStack24: i16;
    let uStack22: u32;
    let piStack18: U32Ptr;
    let uStack16: u16;
    let local_e: i16;
    let local_c: u16;
    let uStack10: u32;
    let puStack6: U32Ptr;

    b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (param_1 + 0xc), 0x29);
    if (b_var1 != 0x0) {
        pass1_1028_bd38(CONCAT22(param_2, param_1), param_4, param_6);
        if ((param_3 == 0x0) && ((param_1 + 0xc) == 0x13)) {
            pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_6, param_4, param_5);
            // param_4 = (pu_var3 >> 0x10);
            pass1_1010_988c(pu_var3, (param_1 + 0xc));
        }
        puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_6, param_4, param_5);
        // uStack16 = (puStack6 >> 0x10);
        uStack10 = (puStack6 + 0x20);
        puVar6 = &local_c;
        piVar2 = &local_e;
        piVar4 = piVar2;
        u_var5 = param_6;
        uVar7 = param_6;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack10);
        piStack18 = piVar2;
        pass1_1030_5b1c(
            CONCAT22(uStack16, piVar2),
            CONCAT22(u_var5, piVar4),
            CONCAT22(uVar7, puVar6),
        );
        pass1_1028_b58e(CONCAT22(param_2, param_1));
        uStack22 = CONCAT22(extraout_dx, piVar2);
        local_1c = (piVar2 + 0x6);
        iStack24 = piVar2[0x8];
        pass1_1028_c8ee(param_6, param_1, param_2, 0x1, CONCAT22(param_6, &local_1c));
        pass1_1008_3eb4(
            CONCAT22(param_6, &local_1c),
            CONCAT22(param_6, &local_22),
            CONCAT22(param_6, local_20),
            CONCAT22(param_6, local_1e),
        );
        if (local_e < local_22) {
            pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
            pass1_1030_5b1c(
                CONCAT22(uStack16, piStack18),
                CONCAT22(param_6, &local_e),
                CONCAT22(param_6, &local_c),
            );
        }
        uStack38 = (uStack22 + 0x2e);
        uStack42 = (uStack38 + 0x4);
        struct_op_1028_87f0(
            param_6,
            param_7,
            CONCAT22(param_6, local_14e),
            0x0,
            0x0,
            0x62,
            &local_1c,
            param_6,
            uStack42,
            uStack10,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_6, local_14e));
        pass1_1028_ccd0(
            param_7,
            param_6,
            CONCAT22(param_2, param_1),
            CONCAT22(param_6, &local_1c),
        );
    }
    return;
}

pub fn pass1_1028_4f30(param_1: u32, param_2: i16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let u_var2: u16;

    u_var1 = pass1_1028_b58e(param_1);
    if (param_2 == 0x0) {
        u_var2 = 0x0;
    } else {
        u_var2 = 0x3e8;
    }
    pass1_1030_7d1c(
        u_var1,
        u_var2,
        0x230000,
        u_var1,
        (u_var1 >> 0x10),
        param_3,
        param_4,
        param_5,
    );
    return;
}

pub fn pass1_1028_4f62(param_1: u32) -> u16 {
    let u_var1: u32;

    u_var1 = pass1_1028_b58e(param_1);
    if ((u_var1 + 0x10) == 0x0) {
        return 0x1;
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4faa(param_1: u32, param_2: u16) -> u32 {
    let u_var1: u16;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let local_c: u32;
    let uStack8: u16;
    let uStack6: u32;

    u_var1 = pass1_1028_4f62(param_1);
    if (u_var1 != 0x0) {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = (uStack6 + 0xc);
    uStack8 = 0x0;
    u_var4 = pass1_1028_bb24(param_1);
    // u_var3 = (u_var4 >> 0x10);
    pu_var2 = &local_c;
    pass1_1030_627e(
        param_2,
        pu_var2,
        u_var3,
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_2, pu_var2),
        u_var4 & 0xffff | u_var3 << 0x10,
    );
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2);
    if ((u_var3 | pu_var2) == 0x0) {
        return 0x0;
    }
    u_var4 = struct_op_1030_73a8(CONCAT22(u_var3, pu_var2));
    return u_var4;
}

pub fn pass1_1028_504a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_50fa(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x5280;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_5128(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
    param_6: u8,
) {
    let pi_var1: U32Ptr;
    let extraout_dx: u16;
    let piVar2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let local_14e: [u8; 124];
    let uStack42: u32;
    let uStack38: u32;
    let local_22: i16;
    let local_20: [u8; 2];
    let local_1e: [u8; 2];
    let local_1c: u32;
    let iStack24: i16;
    let uStack22: u32;
    let piStack18: U32Ptr;
    let uStack16: u16;
    let local_e: i16;
    let local_c: u16;
    let uStack10: u32;
    let puStack6: U32Ptr;

    pass1_1028_bd38(CONCAT22(param_2, param_1), param_3, param_5);
    puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_5, param_3, param_4);
    // uStack16 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x20);
    puVar4 = &local_c;
    pi_var1 = &local_e;
    piVar2 = pi_var1;
    u_var3 = param_5;
    u_var5 = param_5;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack10);
    piStack18 = pi_var1;
    pass1_1030_5b1c(
        CONCAT22(uStack16, pi_var1),
        CONCAT22(u_var3, piVar2),
        CONCAT22(u_var5, puVar4),
    );
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    uStack22 = CONCAT22(extraout_dx, pi_var1);
    local_1c = (pi_var1 + 0x6);
    iStack24 = pi_var1[0x8];
    pass1_1028_c8ee(param_5, param_1, param_2, 0x1, CONCAT22(param_5, &local_1c));
    pass1_1008_3eb4(
        CONCAT22(param_5, &local_1c),
        CONCAT22(param_5, &local_22),
        CONCAT22(param_5, local_20),
        CONCAT22(param_5, local_1e),
    );
    if (local_e < local_22) {
        pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
        pass1_1030_5b1c(
            CONCAT22(uStack16, piStack18),
            CONCAT22(param_5, &local_e),
            CONCAT22(param_5, &local_c),
        );
    }
    uStack38 = (uStack22 + 0x2e);
    uStack42 = (uStack38 + 0x4);
    struct_op_1028_87f0(
        param_5,
        param_6,
        CONCAT22(param_5, local_14e),
        0x0,
        0x0,
        0x6f,
        &local_1c,
        param_5,
        uStack42,
        uStack10,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_5, local_14e));
    pass1_1028_ccd0(
        param_6,
        param_5,
        CONCAT22(param_2, param_1),
        CONCAT22(param_5, &local_1c),
    );
    return;
}

pub fn pass1_1028_525a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_530a(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x535e;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_533c(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_53e8(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x54bc;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5412(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: i16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            i_var4 = 0x6;
            //       TODO: goto code_r0x1028548e;
        }
        ppcVar1 = (*param_1 + 0x64);
        i_var4 = (**ppcVar1)(param_4, param_1);
        if (i_var4 == 0x0) {
            return;
        }
        pass1_1028_c0f0(param_1, 0x1, i_var4, param_2, param_3, param_5);
        if (i_var4 == 0x0) {
            i_var4 = 0x6;
            //       TODO: goto code_r0x1028548e;
        }
        pass1_1028_c952(param_1, param_2, param_3, param_5);
        pass1_1028_c00a(param_1, 0x1, i_var4, param_5);
    }
    i_var4 = 0x5;
    // code_r0x1028548e:
    pass1_1028_bdac(param_1, i_var4, param_4);
    return;
}

pub fn pass1_1028_5496(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_5524(param_1: U32Ptr, param_2: U32Ptr) -> u16 {
    struct_1028_0068(param_1, param_2);
    *param_1 = 0x55c8;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_5546(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
) -> u16 {
    pass1_1028_00cc(param_1, param_2, param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x55c8;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5570(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: i16;

    pass1_1028_0550(param_1, param_2, param_3, param_4, param_5);
    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        u_var4 = 0x0;
        iVar5 = 0x4;
        u_var3 = 0x1;
        u_var2 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        pass1_1030_7c50(
            u_var2,
            CONCAT22(u_var4, u_var3),
            iVar5,
            u_var2,
            (u_var2 >> 0x10),
        );
    }
    return;
}

pub fn pass1_1028_55a2(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_0138(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_5652(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x56ac;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_568a(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_571c(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_57c8(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x581c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_57fa(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_5884(param_1: U32Ptr) -> u16 {
    struct_1028_b354(param_1);
    *param_1 = 0x58fe;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_58a6(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = 0x58fe;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_58dc(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_5988(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = (s_mineToSmelter__no_mines_1050_59df + 0x1);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_59be(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_5a6a(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = ctx.s_thisLo_1050_5bec;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5a98(param_1: u16, param_2: i16, param_3: u16) {
    let plVar1: &i32;
    let i_var2: i16;
    let ppc_var3: u32;
    let u_var4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let extraout_dx: u16;
    let uVar7: u16;
    let extraout_DX_00: u16;
    let uVar8: u32;

    ppc_var3 = ((param_2 + 0xa) + 0x10);
    (**ppc_var3)();
    (param_2 + -0x4) = param_1;
    (param_2 + -0x2) = extraout_dx;
    if ((extraout_dx | param_1) == 0x0) {
        return;
    }
    (param_2 + -0x6) = 0x1;
    uVar8 = pass1_1030_bcae(param_2 - 0x8, param_3);
    // uVar7 = (uVar8 >> 0x10);
    (param_2 + -0xc) = 0x0;
    loop {
        uVar8 = (param_2 + -0x4);
        if (uVar8 <= (param_2 + -0xc)) {
            return;
        }
        pass1_1030_1d58((param_2 + 0xa));
        u_var5 = uVar8;
        (param_2 + -0x10) = u_var5;
        (param_2 + -0xe) = uVar7;
        uVar8 = uVar8 & 0xffff | uVar7 << 0x10;
        pass1_1028_b58e((param_2 + 0x6));
        u_var6 = param_2 - 0x8;
        uVar7 = extraout_DX_00;
        pass1_1030_bd74(
            u_var6,
            param_3,
            CONCAT22(extraout_DX_00, u_var5),
            uVar8,
            param_3,
        );
        (param_2 + -0x12) = u_var6;
        if (u_var6 < 0x5) {
            break;
        }
        plVar1 = (param_2 + -0xc);
        *plVar1 = *plVar1 + 0x1;
    }
    uVar8 = struct_op_1030_73a8((param_2 + -0x10));
    (param_2 + -0x16) = uVar8;
    (param_2 + -0x14) = (uVar8 >> 0x10);
    u_var4 = (param_2 + -0x16);
    i_var2 = (u_var4 + 0x20);
    (param_2 + -0x18) = i_var2;
    if (i_var2 == 0x2) {
        (param_2 + -0x6) = 0x2;
    }
    if (i_var2 != 0x1) {
        return;
    }
    (param_2 + -0x6) = 0x3;
    return;
}

pub fn pass1_1028_5b42(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: i16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            i_var4 = 0x6;
            //       TODO: goto code_r0x10285bbe;
        }
        ppcVar1 = (*param_1 + 0x64);
        i_var4 = (**ppcVar1)(param_4, param_1);
        if (i_var4 == 0x0) {
            return;
        }
        pass1_1028_c0f0(param_1, 0x2, i_var4, param_2, param_3, param_5);
        if (i_var4 == 0x0) {
            i_var4 = 0x6;
            //       TODO: goto code_r0x10285bbe;
        }
        pass1_1028_c952(param_1, param_2, param_3, param_5);
        pass1_1028_c00a(param_1, 0x2, i_var4, param_5);
    }
    i_var4 = 0x5;
    // code_r0x10285bbe:
    pass1_1028_bdac(param_1, i_var4, param_4);
    return;
}

pub fn pass1_1028_5bc6(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_5c76(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = (s_static_1050_5d8b + 0x3);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_5ca4(param_1: u16, param_2: i16, param_3: u16, param_4: u8) {
    let u_var1: u32;
    let u_var2: u32;
    let extraout_dx: u16;
    let u_var3: u32;

    pass1_1028_b58e((param_2 + 0x6));
    (param_2 + -0x4) = param_1;
    (param_2 + -0x2) = extraout_dx;
    u_var1 = (param_2 + -0x4);
    (param_2 + -0x8) = (u_var1 + 0x2e);
    u_var3 = pass1_1028_bb24((param_2 + 0x6));
    u_var2 = (param_2 + -0x8);
    u_var1 = (param_2 + -0x4);
    struct_op_1028_87f0(
        param_3,
        param_4,
        CONCAT22(param_3, param_2 + -0x12c),
        0x0,
        0x0,
        0x65,
        (u_var1 + 0xc),
        (u_var1 >> 0x10),
        (u_var2 + 0x4),
        u_var3,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_3, param_2 + -0x12c));
    (param_2 + -0x12c) = 0x389a;
    (param_2 + -0x12a) = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_5d12(param_1: u16, param_2: i16, param_3: u16, param_4: u8) {
    let u_var1: u32;
    let u_var2: u32;
    let extraout_dx: u16;

    pass1_1028_b58e((param_2 + 0x6));
    (param_2 + -0x4) = param_1;
    (param_2 + -0x2) = extraout_dx;
    u_var2 = (param_2 + -0x4);
    (param_2 + -0x8) = (u_var2 + 0x2e);
    u_var2 = (param_2 + -0x8);
    u_var1 = (u_var2 + 0x4);
    (param_2 + -0xc) = u_var1;
    pass1_1028_68de(
        CONCAT22(param_3, param_2 + -0x11a),
        0x1,
        u_var1,
        param_4,
        param_3,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_3, param_2 + -0x11a));
    (param_2 + -0x11a) = 0x389a;
    (param_2 + -0x118) = 0x1008;
    return;
}

pub fn pass1_1028_5d68(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_5df6(param_1: U32Ptr) -> u16 {
    struct_1028_b354(param_1);
    *param_1 = (s_thisHi_1050_5e6f + 0x1);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_5e18(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    CONCAT22(param_2, param_1) = (s_thisHi_1050_5e6f + 0x1);
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5e4e(param_1: i16, param_2: u16) -> u32 {
    pass1_1028_b418((param_1 + 0x6));
    if (((param_1 + 0xa) & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, (param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

pub fn pass1_1028_5f00(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
) -> u16 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    (param_1 + 0x20) = 0x0;
    CONCAT22(param_2, param_1) = 0x6054;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5f34(
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let BVar3: bool;
    let u_var4: u32;
    let extraout_dx: u16;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;

    pass1_1028_be9e((param_2 + 0x6), param_3, param_4, param_5, param_6);
    u_var4 = (param_2 + 0x6);
    // u_var6 = (u_var4 >> 0x10);
    if ((u_var4 + 0x12) == 0x5) {
        (u_var4 + 0x20) = 0x64;
        pass1_1028_b58e(u_var4);
        (param_2 + -0x4) = param_1;
        (param_2 + -0x2) = extraout_dx;
        u_var2 = (param_2 + -0x4);
        u_var4 = (u_var2 + 0x2e);
        iVar7 = 0x61;
        u_var5 = extraout_dx;
        pass1_1038_3fb0(u_var4);
        BVar3 = pass1_1030_25b2(u_var4 & 0xffff | u_var5 << 0x10, iVar7);
        if (BVar3 != 0x0) {
            u_var2 = (param_2 + 0x6);
            pi_var1 = (u_var2 + 0x20);
            *pi_var1 = *pi_var1 + 0x64;
        }
    }
    return;
}

pub fn pass1_1028_6008(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;

    pass1_1028_be2a(param_1, param_2, param_3, param_4, param_5);
    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x12) == 0x5) && (0x0 < (i_var2 + 0x20))) {
        pi_var1 = (i_var2 + 0x20);
        *pi_var1 = *pi_var1 + -0x1;
    }
    return;
}

pub fn pass1_1028_602e(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_611e(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
    param_6: U32Ptr,
) -> u32 {
    let u_var1: u32;

    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_4, param_6);
    (param_1 + 0x20) = 0x0;
    CONCAT22(param_2, param_1) = 0x6876;
    (param_1 + 0x2) = &USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_6, 0x1000);
    if ((param_6 | param_5) == 0x0) {
        (param_1 + 0x20) = 0x0;
    } else {
        u_var1 = set_struct_1008_574a(CONCAT22(param_6, param_5));
        (param_1 + 0x20) = u_var1;
        (param_1 + 0x22) = (u_var1 >> 0x10);
    }
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_6186(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct603;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x6876;
    i_var4.field_0x2 = &USHORT_1050_1028;
    pu_var1 = i_var4.field_0x20;
    u_var2 = i_var4.field_0x22;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_61c4(param_1: u32, param_2: u32, param_3: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;
    let paVar5: &mut Struct21;
    let u_var6: u32;
    let uVar7: u16;
    let iVar7: &mut Struct315;

    iVar7 = param_1;
    // uVar7 = (param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_3);
    pu_var1 = iVar7.field_0x20;
    u_var2 = iVar7.field_0x22;
    u_var4 = u_var2 | pu_var1;
    paVar5 = CONCAT22(u_var4, pu_var1);
    if (u_var4 != 0x0) {
        ppc_var3 = *pu_var1;
        paVar5 = (**ppc_var3)();
    }
    mem_op_1000_179c(0xc, (paVar5 >> 0x10), 0x1000);
    if (paVar5 == 0x0) {
        u_var6 = 0x0;
    } else {
        u_var6 = set_struct_1008_574a(paVar5);
    }
    iVar7.field_0x20 = u_var6;
    iVar7.field_0x22 = (u_var6 >> 0x10);
    return;
}

pub fn pass1_1028_6228(param_1: u32, param_2: u16, param_3: i16, param_4: i16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let bVar8: bool;
    let lVar9: i32;
    let local_a: [u8; 4];
    let uStack6: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(param_5, local_a), (iVar6 + 0x20));
    loop {
        loop {
            lVar9 = pass1_1008_5b12(local_a, param_5);
            // u_var5 = (lVar9 >> 0x10);
            i_var4 = lVar9;
            if (lVar9 == 0x0) {
                return;
            }
            if ((i_var4 + 0x6) != param_4) == false {
                break;
            }
        }
        u_var1 = (i_var4 + 0xa);
        if ((param_3 == 0x0) && (false || (param_2 < u_var1))) {
            break;
        }
        bVar8 = param_2 < u_var1;
        param_2 -= u_var1;
        param_3 -= bVar8;
        ppc_var3 = ((iVar6 + 0x20) + 0xc);
        (**ppc_var3)(0x1008, (iVar6 + 0x20));
        uStack6 = 0x0;
    }
    u_var2 = (i_var4 + 0xc);
    (i_var4 + 0xa) = u_var1 - param_2;
    (i_var4 + 0xc) = -(param_2 * (u_var2 / u_var1) - (i_var4 + 0xc));
    return;
}

pub fn pass1_1028_62c8(param_1: u32, param_2: u16) -> u32 {
    let u_var1: u16;
    let u_var2: u32;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        u_var2 = pass1_1028_67d4(param_1 & 0xffff | u_var1 << 0x10, param_2);
        u_var1 = u_var2;
        if (((u_var2 >> 0x10) == 0x0) && (u_var1 < 0x64)) {
            return CONCAT22(-(0x64 < u_var1), 0x64 - u_var1);
        }
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_6302(param_1: u32, param_2: u16) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let lVar3: i32;
    let uStack18: u32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_2, local_a), (param_1 + 0x20));
    uStack18 = 0x0;
    loop {
        lVar3 = pass1_1008_5b12(local_a, param_2);
        // u_var2 = (lVar3 >> 0x10);
        if (lVar3 == 0x0) {
            break;
        }
        if ((lVar3 + 0x8) != 0x0) {
            u_var1 = (lVar3 + 0xa);
            uStack18 = CONCAT22(
                (uStack18 >> 0x10) + CARRY2(uStack18, u_var1),
                uStack18 + u_var1,
            );
        }
    }
    return uStack18;
}

pub fn pass1_1028_6356(param_1: u32, param_2: i16, param_3: u16, param_4: i16, param_5: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let ppc_var4: u32;
    let iVar5: i16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let bVar9: bool;
    let lVar10: i32;
    let local_a: [u8; 4];
    let uStack6: u32;

    // uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1008_5784(CONCAT22(param_5, local_a), (iVar7 + 0x20));
    loop {
        loop {
            lVar10 = pass1_1008_5b12(local_a, param_5);
            // u_var6 = (lVar10 >> 0x10);
            iVar5 = lVar10;
            if lVar10 == 0x0 {
                return;
            }
            if ((((iVar5 + 0x8) == 0x0) || (param_2 != 0x0 && ((iVar5 + 0x8) != param_2)))
                || ((iVar5 + 0x8) == 0xf && (param_2 != 0xf)))
                == false
            {
                break;
            }
        }
        u_var2 = (iVar5 + 0xa);
        if (param_4 == 0x0) && (false || (param_3 < u_var2)) {
            break;
        }
        bVar9 = param_3 < u_var2;
        param_3 -= u_var2;
        param_4 -= bVar9;
        ppc_var4 = ((iVar7 + 0x20) + 0xc);
        (**ppc_var4)(0x1008, (iVar7 + 0x20));
        uStack6 = 0x0;
    }
    u_var3 = (iVar5 + 0xc);
    pi_var1 = (iVar5 + 0xa);
    *pi_var1 = *pi_var1 - param_3;
    pi_var1 = (iVar5 + 0xc);
    *pi_var1 = *pi_var1 - param_3 * (u_var3 / u_var2);
    return;
}

pub fn pass1_1028_6408(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let ppcVar1: u32;
    let bVar2: bool;
    let pu_var3: U32Ptr;
    let extraout_dx: u16;
    let i_var4: i16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let local_a: [u8; 8];

    // u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    pass1_1008_5784(CONCAT22(param_3, local_a), (i_var4 + 0x20));
    bVar2 = false;
    loop {
        pu_var3 = local_a;
        pass1_1008_5b12(pu_var3, param_3);
        iVar5 = param_2;
        // uVar7 = (param_2 >> 0x10);
        if ((extraout_dx | pu_var3) == 0x0) {
            break;
        }
        if (((pu_var3 + 0x4) == (iVar5 + 0x4)) && ((pu_var3 + 0x6) == (iVar5 + 0x6))) {
            if ((pu_var3 + 0x8) == (iVar5 + 0x8)) {
                bVar2 = true;
                (pu_var3 + 0xa) = (pu_var3 + 0xa) + (iVar5 + 0xa);
                (pu_var3 + 0xc) = (pu_var3 + 0xc) + (iVar5 + 0xc);
            }
        }
    }
    if (bVar2) {
        if (param_2 != 0x0) {
            ppcVar1 = *param_2;
            (**ppcVar1)(0x1008, param_2, 0x1, param_2, param_2);
            return;
        }
    } else {
        ppcVar1 = ((i_var4 + 0x20) + 0x4);
        (**ppcVar1)(0x1008, (i_var4 + 0x20), param_2);
    }
    return;
}

pub fn pass1_1028_64d6(param_1: u32, param_2: u32, param_3: u16) -> u16 {
    let u_var1: u32;
    let BVar2: bool;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let local_26: u16;
    let local_24: u16;
    let local_22: u16;
    let local_20: u16;
    let local_1e: u16;
    let local_1c: [u16; 0x6];
    let uStack16: u16;
    let lStack14: i32;
    let local_a: [u8; 8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if (BVar2 != 0x0) {
        // u_var4 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x20));
        u_var1 = (param_1 + 0x20);
        local_1c[0] = (u_var1 + 0x8);
        pu_var3 = local_1c;
        uStack16 = local_1c[0];
        loop {
            u_var5 = param_2;
            // u_var6 = (param_2 >> 0x10);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, pu_var3, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                break;
            }
            lStack14 = pass1_1008_5b12(local_a, param_3);
            if (lStack14 == 0x0) {
                return 0x1;
            }
            local_1e = (lStack14 + 0x4);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, &local_1e, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                break;
            }
            local_20 = (lStack14 + 0x6);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, &local_20, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                break;
            }
            local_22 = (lStack14 + 0x8);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, &local_22, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                break;
            }
            local_24 = (lStack14 + 0xa);
            BVar2 = write_to_file_1008_7e1c(u_var5, u_var6, &local_24, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                break;
            }
            local_26 = (lStack14 + 0xc);
            pu_var3 = &local_26;
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_65e2(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let local_16: u16;
    let paStack20: &mut Struct99;
    let local_10: [u16; 0x2];
    let local_c: [u16; 0x3];
    let uStack6: u16;
    let local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if (param_3 != 0x0) {
        uVar7 = param_2;
        // uVar8 = (param_2 >> 0x10);
        BVar3 = read_file_1008_7dee(uVar7, uVar8, &local_4, 0x0, param_5, 0x2, 0x1008);
        if (BVar3 != 0x0) {
            uStack6 = 0x0;
            loop {
                if (local_4 <= uStack6) {
                    return;
                }
                paStack20 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                // u_var5 = (paStack20 >> 0x10);
                u_var2 = paStack20;
                if ((u_var5 | u_var2) == 0x0) {
                    paStack20 = 0x0;
                } else {
                    paStack20.field_0x0 = 0x389a;
                    (u_var2 + 0x2) = 0x1008;
                    (u_var2 + 0x4) = 0x0;
                    (u_var2 + 0x6) = 0x0;
                    (u_var2 + 0x8) = 0x0;
                    (u_var2 + 0xa) = 0x0;
                    (u_var2 + 0xc) = 0x0;
                    paStack20.field_0x0 = 0x56ce;
                    (u_var2 + 0x2) = 0x1018;
                }
                BVar3 = read_file_1008_7dee(uVar7, uVar8, local_10, 0x0, param_5, 0x2, 0x1008);
                if (BVar3 == 0x0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(uVar7, uVar8, local_c, 0x0, param_5, 0x2, 0x1008);
                if (BVar3 == 0x0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(uVar7, uVar8, &local_16, 0x0, param_5, 0x2, 0x1008);
                if (BVar3 == 0x0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(
                    uVar7,
                    uVar8,
                    paStack20 + 0xa,
                    0x0,
                    (paStack20 >> 0x10),
                    0x2,
                    0x1008,
                );
                if (BVar3 == 0x0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(
                    uVar7,
                    uVar8,
                    paStack20 + 0xc,
                    0x0,
                    (paStack20 >> 0x10),
                    0x2,
                    0x1008,
                );
                if (BVar3 == 0x0) {
                    break;
                }
                (paStack20 + 0x4) = local_10[0];
                u_var4 = switch_1008_72bc(uVar7, uVar8, local_c[0]);
                // u_var6 = (paStack20 >> 0x10);
                (paStack20 + 0x6) = u_var4;
                (paStack20 + 0x8) = local_16;
                ppcVar1 = ((param_1 + 0x20) + 0x8);
                (**ppcVar1)();
                uStack6 += 0x1;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

pub fn pass1_1028_6744(param_1: u16, param_2: u32, param_3: i16) -> u16 {
    let u_var1: u16;
    let lVar2: i32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_1, local_a), (param_2 + 0x20));
    loop {
        lVar2 = pass1_1008_5b12(local_a, param_1);
        // u_var1 = (lVar2 >> 0x10);
        if lVar2 == 0x0 {
            return 0x0;
        }
        if ((lVar2 + 0x6) != param_3) == false {
            break;
        }
    }
    return (lVar2 + 0xa);
}

pub fn pass1_1028_678c(param_1: u32, param_2: i16, param_3: u16) -> u16 {
    let u_var1: u16;
    let lVar2: i32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x20));
    loop {
        lVar2 = pass1_1008_5b12(local_a, param_3);
        // u_var1 = (lVar2 >> 0x10);
        if (lVar2 == 0x0) {
            return 0x0;
        }
        if ((lVar2 + 0x8) != param_2) == false {
            break;
        }
    }
    return (lVar2 + 0xa);
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_67d4(param_1: u32, param_2: u16) -> u32 {
    let u_var1: u16;
    let lVar2: i32;
    let uStack18: u32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_2, local_a), (param_1 + 0x20));
    uStack18 = 0x0;
    loop {
        lVar2 = pass1_1008_5b12(local_a, param_2);
        if (lVar2 == 0x0) {
            break;
        }
        u_var1 = (lVar2 + 0xc);
        uStack18 = CONCAT22(
            (uStack18 >> 0x10) + CARRY2(uStack18, u_var1),
            uStack18 + u_var1,
        );
    }
    return uStack18;
}

pub fn pass1_1028_6822(param_1: u32, param_2: U32Ptr, param_3: u16) -> u16 {
    let i_var1: i16;
    let u_var2: u32;

    u_var2 = pass1_1028_67d4(param_1, param_3);
    // i_var1 = (u_var2 >> 0x10);
    *param_2 = u_var2;
    (param_2 + 0x2) = i_var1;
    if ((i_var1 == 0x0) && (*param_2 < 0x64)) {
        return 0x0;
    }
    return 0x1;
}

pub fn pass1_1028_6850(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_6186(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_68de(
    param_1: &mut Struct100,
    param_2: u16,
    param_3: u32,
    param_4: u8,
    param_5: u16,
) {
    let i_var1: i16;
    let u_var2: u16;

    struct_op_1028_d1dc(param_5, param_4, param_1, 0x3e8);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x108) = param_3;
    (i_var1 + 0x10c) = param_2;
    param_1.field_0x0 = 0x6ae2;
    (i_var1 + 0x2) = &USHORT_1050_1028;
    string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var1 + 0x8)),
        ctx.s_SCAddSpew_1050_4fd2,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_6926(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let puVar11: u32;
    let puStack14: u32;

    // uVar9 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0xa);
    // puVar6 = (puVar11 >> 0x10);
    u_var4 = puVar11;
    u_var10 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(CONCAT22(param_3, param_2), puVar11, u_var4, puVar6);
    puStack14 = CONCAT22(puVar6, u_var4);
    u_var2 = *puStack14;
    uVar8 = u_var2;
    ppc_var3 = (uVar8 + 0x10);
    u_var5 = u_var4;
    (**ppc_var3)(&ctx.PTR_LOOP_1050_1038, u_var4, puVar6);
    if ((extraout_dx | u_var5) != 0x0) {
        ppc_var3 = (uVar8 + 0x4);
        (**ppc_var3)(0x38, u_var4, puVar6, 0x0, 0x0);
        puVar7 = extraout_DX_00;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
        u_var1 = (param_1 + 0x10c);
        u_var10 = 0x1030;
        pass1_1030_7ddc(
            CONCAT22(puVar7, u_var5),
            CONCAT13((u_var1 >> 0xf), u_var1),
            0x1f,
            u_var1,
            puVar7,
            uVar8,
            (u_var2 >> 0x10),
            param_4,
        );
    }
    if (puStack14 != 0x0) {
        ppc_var3 = *puStack14;
        (**ppc_var3)(u_var10, u_var4, puVar6, 0x1);
    }
    return;
}

pub fn pass1_1028_69cc(param_1: u32, param_2: &mut Struct317, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct316;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10e, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = 0x6ae2;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_6a7a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_6aa6(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_6af2(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u8,
) {
    let i_var1: &mut Struct683;
    let u_var1: u16;

    struct_op_1028_d1dc(param_4, param_5, param_1, 0x1387);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x108 = param_3;
    i_var1.field_0x10c = param_2;
    param_1.field_0x0 = 0x6e50;
    i_var1.field_0x2 = &USHORT_1050_1028;
    return;
}

pub fn pass1_1028_6b2c(param_1: u32, param_2: u16) -> u16 {
    pass1_1028_6b40(param_1, param_2);
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_6b40(param_1: u32, param_2: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let in_DX: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let local_36: [u8; 0xe];
    let puStack40: u32;
    let uStack38: u16;
    let uStack36: u16;
    let uStack34: u16;
    let uStack32: u16;
    let uStack30: u16;
    let uStack28: u16;
    let uStack26: u16;
    let local_18: u32;
    let uStack20: u16;
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: u32;
    let local_6: [u8; 2];
    let local_4: i16;

    pu_var3 = local_6;
    pass1_1028_6daa(
        param_1,
        CONCAT22(param_2, pu_var3),
        CONCAT22(param_2, &local_4),
        pu_var3,
        in_DX,
        param_2,
    );
    // u_var6 = (param_1 >> 0x10);
    u_var5 = param_1;
    u_var1 = (u_var5 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    puStack10 = CONCAT22(in_DX, pu_var3);
    ppcVar2 = (*puStack10 + 0x24);
    (**ppcVar2)();
    uStack14 = pass1_1028_b58e(puStack10);
    uStack18 = pass1_1028_bb24(puStack10);
    local_18 = (uStack14 + 0xc);
    uStack20 = (uStack14 + 0x10);
    puStack40 = &local_18;
    uStack26 = local_18;
    // uStack28 = (local_18 >> 0x10);
    uStack32 = local_18 - 0x1;
    if (uStack32 < 0x0) {
        uStack32 = 0x0;
    }
    u_var4 = local_4 - 0x1;
    uStack34 = local_18 + 0x1;
    if (u_var4 < (local_18 + 0x1)) {
        uStack34 = u_var4;
    }
    uStack36 = uStack28 - 0x1;
    if (uStack36 < 0x0) {
        uStack36 = 0x0;
    }
    uStack38 = uStack28 + 0x1;
    if (u_var4 < (uStack28 + 0x1)) {
        uStack38 = u_var4;
    }
    uStack30 = uStack20;
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack20, uStack36, uStack32);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack36, uStack26);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack36, uStack34);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack28, uStack32);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack28, uStack34);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack38, uStack32);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack38, uStack26);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    puVar7 = pass1_1008_3e54(CONCAT22(param_2, local_36), uStack30, uStack38, uStack34);
    pass1_1028_6d24(
        u_var5,
        u_var6,
        CONCAT22(param_2, local_36),
        uStack18,
        (puVar7 >> 0x10),
        param_2,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_6d24(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i32,
    param_5: u16,
    param_6: u16,
) {
    let i_var1: i16;
    let ppcVar2: u32;
    let pu_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: u32;
    let bStack27: u8;
    let local_a: u32;
    let uStack6: u32;

    pu_var3 = &local_a;
    pass1_1030_64ce(
        param_6,
        pu_var3,
        param_5,
        ctx.PTR_LOOP_1050_5740,
        param_3,
        param_4,
        CONCAT22(param_6, pu_var3),
    );
    uStack6 = *pu_var3;
    u_var5 = (pu_var3 + 0x2);
    bStack27 = (uStack6 >> 0x18);
    u_var4 = bStack27;
    if (bStack27 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
        puVar6 = struct_op_1030_73a8(CONCAT22(u_var5, u_var4));
        i_var1 = (puVar6 + 0xc);
        if (((i_var1 < 0x1) || (SBORROW2(i_var1, 0x1)))
            || (i_var1 != 0x9
                && 0x7 < i_var1 + -0x1
                && (i_var1 + -0x9 < 0x6a || (0x6 < i_var1 + -0x73))))
        {
            ppcVar2 = (*puVar6 + 0x24);
            (**ppcVar2)();
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_6daa(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u32;
    let pu_var2: u32;
    let local_18: u32;
    let uStack20: u16;
    let iStack18: i16;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u16;
    let uStack4: u16;

    u_var1 = (param_1 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    uStack6 = param_4;
    uStack4 = param_5;
    uStack10 = pass1_1028_b4f2(CONCAT22(param_5, param_4));
    // uStack16 = (uStack10 >> 0x10);
    u_var1 = (uStack10 + 0x8);
    uStack14 = u_var1;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    iStack18 = u_var1;
    pu_var2 = pass1_1030_5b5c(iStack18, uStack16);
    local_18 = *pu_var2;
    uStack20 = (pu_var2 + 0x4);
    pass1_1008_3e94(CONCAT22(param_6, &local_18), param_2, param_3);
    return;
}

pub fn pass1_1028_6e24(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_6e60(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x32c7);
    param_1.field_0x0 = 0x6fb0;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCConstruct_1050_4fdc,
    );
    return param_1;
}

pub fn pass1_1028_6e96(param_1: u16, param_2: u16) -> u16 {
    let i_var1: i16;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let extraout_dx: u16;
    let puStack24: u32;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x700);
    loop {
        pu_var3 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var3));
        puStack24 = CONCAT22(param_1, pu_var3);
        u_var4 = param_1 | pu_var3;
        if (u_var4 == 0x0) {
            break;
        }
        i_var1 = (pu_var3 + 0x12);
        param_1 = u_var4;
        if (((0x0 < i_var1) && (!SBORROW2(i_var1, 0x1))) && (i_var1 + -0x1 < 0x4)) {
            ppcVar2 = (*puStack24 + 0x38);
            (**ppcVar2)();
            param_1 = extraout_dx;
        }
    }
    return 0x1;
}

pub fn pass1_1028_6ef6(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0x6fb0;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_6f84(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_6fc0(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3e7);
    param_1.field_0x0 = 0x749e;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCEndSim_1050_4fea,
    );
    return param_1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_6ff6(param_1: u32, param_2: u16, param_3: i16, param_4: u16) {
    let plVar1: &i32;
    let pu_var2: U32Ptr;
    let paVar3: &mut Struct67;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let iVar11: i16;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u16;
    let uVar16: u16;
    let uVar17: u16;
    let iVar18: i16;
    let local_46: [u8; 12];
    let uStack52: u32;
    let iStack48: i16;
    let puStack46: U32Ptr;
    let paStack38: &mut Struct67;
    let puStack34: U32Ptr;
    let puStack32: U32Ptr;
    let iStack30: i16;
    let iStack28: i16;
    let iStack26: i16;
    let uStack24: u32;
    let local_14: [u8; 8];
    let uStack12: u16;
    let puStack10: U32Ptr;
    let uStack8: u16;
    let puStack6: U32Ptr;
    let i_stack4: i16;

    uVar13 = (param_4 >> 0x8);
    pass1_1028_dc52(
        CONCAT13(uVar13, CONCAT12(param_4, local_14)),
        0x1,
        0x0,
        0x400,
    );
    iStack26 = 0x1;
    iStack28 = 0x0;
    loop {
        loop {
            uVar7 = param_2;
            pu_var2 = local_14;
            pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
            uStack24 = CONCAT22(uVar7, pu_var2);
            param_2 = uVar7 | pu_var2;
            if (param_2 == 0x0) {
                // goto
                // LAB_1028_7066;
            }
            if (((pu_var2 + 0x1fe) == 0x0) || ((pu_var2 + 0x200) == 0x8000002)) == false {
                break;
            }
        }
        iStack28 = 0x1;
        paVar3 = (pu_var2 + 0x1f6);
        paStack38 = paVar3;
        pass1_1030_38b8();
        if ((param_2 < 0x0) || (param_2 < 0x1 && (paVar3 == 0x0))) == false {
            break;
        }
    }
    iStack26 = 0x0;
    //LAB_1028_7066:
    puStack10 = puStack6;
    uStack12 = uStack8;
    if i_stack4 != 0x0 {
        puStack10 = 0x0;
        uStack12 = 0x1;
    }
    iStack30 = 0x0;
    puVar4 = puStack10;
    loop {
        pu_var2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
        uStack24 = CONCAT22(puVar4, pu_var2);
        puStack32 = (puVar4 | pu_var2);
        if puStack32 == 0x0 {
            break;
        }
        plVar1 = (pu_var2 + 0x200);
        puVar4 = puStack32;
        if *plVar1 == 0x8000001 {
            iStack30 = 0x1;
        }
    }
    if (iStack30 == 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
        uStack24 = CONCAT22(puStack32, pu_var2);
        puStack32 = (puStack32 | pu_var2);
        if (puStack32 != 0x0) {
            ctx.PTR_LOOP_1050_4fe8 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
            uVar16 = 0x0;
            iVar11 = 0x1;
            uStack52 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, puStack32, param_3);
            // puStack32 = (uStack52 >> 0x10);
            pu_var2 = uStack52;
            pass1_1010_089e(param_4, uStack52, uVar16, iVar11);
            pass1_1010_089e(param_4, uStack52, 0x0, 0x2);
            pass1_1010_089e(param_4, uStack52, 0x0, 0x3);
            pass1_1010_089e(param_4, uStack52, 0x0, 0x4);
            pass1_1010_089e(param_4, uStack52, 0x0, 0x5);
            pass1_1010_089e(param_4, uStack52, 0x0, 0x7);
            pass1_1010_089e(param_4, uStack52, 0x0, 0x8);
            pass1_1010_089e(param_4, uStack52, 0x0, 0xa);
        }
    }
    if ((iStack28 != 0x0) && (iStack26 != 0x0)) {
        uVar17 = 0x0;
        iVar18 = 0x6;
        uVar12 = 0x1;
        uVar14 = 0x0;
        uVar15 = 0x0;
        u_var10 = 0x0;
        iVar11 = 0x0;
        uVar9 = 0x0;
        uStack52 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, puStack32, param_3);
        // puStack32 = (uStack52 >> 0x10);
        pu_var2 = uStack52;
        post_win_msg_1008_a0e4(
            uStack52,
            CONCAT22(u_var10, uVar9),
            iVar11,
            CONCAT11(uVar14, uVar12),
            CONCAT22(uVar17, uVar15),
            iVar18,
            0x1008,
            param_4,
        );
    }
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
    puVar4 = (puStack32 | pu_var2);
    puStack34 = pu_var2;
    if (((((puVar4 != 0x0)
        && (
            pu_var2 = pass1_1030_2242(CONCAT22(puStack32, pu_var2), 0x4),
            pu_var2 == 0x0,
        ))
        && (
            pu_var2 = pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x2a),
            pu_var2 == 0x0,
        ))
        && ((
            pu_var2 = pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x4b),
            pu_var2 == 0x0
                && (
                    pu_var2 = pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x54),
                    pu_var2 == 0x0,
                ),
        )))
        && ((
            pu_var2 = pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x2c),
            pu_var2 == 0x0
                && ((
                    pu_var2 = pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x3c),
                    pu_var2 == 0x0
                        && (
                            pu_var2 = pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x3d),
                            pu_var2 == 0x0,
                        ),
                )),
        )))
    {
        if (i_stack4 != 0x0) {
            uStack8 = 0x1;
            puStack6 = 0x0;
        }
        uStack52 = (uStack52 & 0xffff0000);
        iStack48 = 0x0;
        uStack12 = uStack8;
        puStack10 = puStack6;
        loop {
            loop {
                pu_var5 = puStack6;
                pu_var2 = local_14;
                pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
                uStack24 = CONCAT22(pu_var5, pu_var2);
                puVar4 = (pu_var5 | pu_var2);
                if puVar4 == 0x0 {
                    // goto
                    // LAB_1028_72d3;
                }
                puStack6 = puVar4;
                if ((pu_var2 + 0x200) == 0x8000002) == false {
                    break;
                }
            }
            // uVar16 = (param_1 >> 0x10);
            if (uStack52 == 0x0)
                && (
                    pass1_1028_740c(param_1, uVar16, 0x22, CONCAT22(pu_var5, pu_var2)),
                    pu_var2 != 0x0,
                )
            {
                uStack52 = CONCAT22(uStack52._2_2_, 0x1);
            }
            if (iStack48 == 0x0)
                && (
                    pass1_1028_740c(param_1, uVar16, 0x24, uStack24),
                    pu_var2 != 0x0,
                )
            {
                iStack48 = 0x1;
            }
            puStack6 = puVar4;
            if ((uStack52 == 0x0) || (iStack48 == 0x0)) == false {
                break;
            }
        }
        uVar17 = 0x0;
        iVar18 = 0x14;
        uVar12 = 0x1;
        uVar14 = 0x0;
        uVar15 = 0x0;
        u_var10 = 0x0;
        iVar11 = 0x0;
        uVar9 = 0x0;
        paStack38 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, puVar4, param_3);
        // puVar4 = (paStack38 >> 0x10);
        pu_var2 = paStack38;
        post_win_msg_1008_a0e4(
            paStack38,
            CONCAT22(u_var10, uVar9),
            iVar11,
            CONCAT11(uVar14, uVar12),
            CONCAT22(uVar17, uVar15),
            iVar18,
            0x1008,
            param_4,
        );
    }
    //LAB_1028_72d3:
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
    uStack24 = CONCAT22(puVar4, pu_var2);
    if ((puVar4 | pu_var2) != 0x0) {
        puVar8 = mixed_1010_20ba(
            ctx.PTR_LOOP_1050_0ed0,
            0x3b,
            param_4,
            (puVar4 | pu_var2),
            param_3,
        );
        // puVar4 = (puVar8 >> 0x10);
        iStack48 = puVar8;
        puStack46 = puVar4;
        pass1_1008_df4a(puVar8, param_3, param_4);
        puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3c, param_4, puVar4, param_3);
        // uVar7 = (puVar8 >> 0x10);
        iStack48 = puVar8;
        puStack46 = uVar7;
        pass1_1018_34a6(puVar8);
        pass1_1028_dc52(
            CONCAT13(uVar13, CONCAT12(param_4, local_46)),
            0x1,
            0x0,
            0x400,
        );
        loop {
            u_var6 = uVar7;
            pu_var2 = local_46;
            pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
            uStack52 = CONCAT22(u_var6, pu_var2);
            uVar7 = u_var6 | pu_var2;
            if (uVar7 == 0x0) {
                break;
            }
            if ((pu_var2 + 0x200) != 0x8000002) {
                pass1_1038_3ba0(CONCAT22(u_var6, pu_var2));
            }
        }
    }
    return;
}

pub fn pass1_1028_737e(param_1: u32, param_2: u16, param_3: u16) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0x749e;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_740c(param_1: u16, param_2: u16, param_3: i16, param_4: u32) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let puVar6: u32;
    let lStack14: i32;
    let puStack10: u32;

    puVar6 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, param_3);
    // pu_var5 = (puVar6 >> 0x10);
    u_var3 = puVar6;
    pass1_1038_4d6e(param_4, puVar6, u_var3, pu_var5);
    puStack10 = CONCAT22(pu_var5, u_var3);
    u_var2 = *puStack10;
    ppcVar1 = u_var2 + 0x8;
    u_var4 = u_var3;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var3, pu_var5);
    lStack14 = CONCAT22(extraout_dx, u_var4);
    if (puStack10 != 0x0) {
        ppcVar1 = u_var2;
        (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var3, pu_var5, 0x1);
    }
    if (lStack14 != 0x0) {
        return;
    }
    return;
}

pub fn pass1_1028_7472(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_74ae(param_1: &mut Struct100) -> Struct100 {
    let unaff_SS: u16;
    let in_AF: u8;

    struct_op_1028_d1dc(unaff_SS, in_AF, param_1, 0x1387);
    param_1.field_0x0 = 0x819a;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCEvent_1050_4ff4,
    );
    return param_1;
}

pub fn pass1_1028_74e4(param_1: u32, param_2: i32, param_3: i16, param_4: u16, param_5: u8) -> u16 {
    let i_var1: i16;

    pass1_1028_7fb6(param_1, param_3, param_4, param_5);
    pass1_1028_7c4e(param_1, param_2, param_3, param_4);
    pass1_1028_7dfc(param_1, param_2, param_3, param_4, param_5);
    i_var1 = post_msg_1028_76da(param_1);
    pass1_1028_767e(i_var1, param_2, param_3, param_4);
    pass1_1028_75bc(param_4);
    pass1_1028_78b8(param_1, param_2, param_3, param_4, param_5);
    return 0x1;
}

pub fn pass1_1028_752e(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0x819a;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_75bc(param_1: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u32;
    let uStack28: u32;
    let local_18: [u8; 8];
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u16;
    let iStack8: i16;
    let uStack6: u32;

    uStack6 = *ctx.PTR_LOOP_1050_65e2;
    u_var1 = (uStack6 % 0x7b);
    u_var4 = u_var1;
    if ((u_var1 == 0x0) && (0x95 < uStack6)) {
        pass1_1028_dc52(CONCAT22(param_1, local_18), 0x1, 0x0, 0x400);
        loop {
            u_var1 = u_var4;
            pu_var2 = local_18;
            pass1_1028_e4ec(CONCAT22(param_1, pu_var2));
            uStack28 = CONCAT22(u_var1, pu_var2);
            u_var4 = (u_var1 | pu_var2);
            if ((u_var1 | pu_var2) == 0x0) {
                break;
            }
            pass1_1008_612e(0x1, 0x64, pu_var2);
            if (pu_var2 < 0x6) {
                pass1_1038_362e(uStack28);
            }
        }
        if (iStack8 != 0x0) {
            uStack12 = 0x1;
            uStack10 = 0x0;
        }
        u_var4 = uStack10;
        uStack16 = uStack12;
        uStack14 = uStack10;
        loop {
            u_var1 = u_var4;
            pu_var2 = local_18;
            pass1_1028_e4ec(CONCAT22(param_1, pu_var2));
            u_var3 = u_var1 | pu_var2;
            u_var4 = u_var3;
            if (u_var3 == 0x0) {
                break;
            }
            pass1_1038_3698(CONCAT22(u_var1, pu_var2), pu_var2, u_var3, param_1);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_767e(param_1: i16, param_2: u16, param_3: i16, param_4: u16) {
    let pu_var1: U32Ptr;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
    if (((param_1 + 0x152) != 0x0) && ((*ctx.PTR_LOOP_1050_65e2 % 0x64) == 0x0)) {
        pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x40, param_4, 0x0, param_3);
        load_str_and_sprintf_1008_b78a(pu_var1, param_4, (pu_var1 >> 0x10), pu_var1);
    }
    return;
}

pub fn pass1_1028_7742(param_1: u16, param_2: u16, param_3: i16, param_4: u32, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let u_var6: u16;
    let extraout_DX_00: u16;
    let puVar7: u32;
    let uVar8: u32;
    let uVar9: u8;
    let u_var10: u8;
    let u_var11: u16;
    let uStack26: u32;
    let local_16: [u8; 2];
    let uStack20: u32;
    let uStack16: u16;
    let puStack14: u32;
    let uStack10: u16;
    let puStack8: U32Ptr;
    let uStack6: u16;
    let uStack4: u16;

    puVar7 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x18);
    // u_var4 = (puVar7 >> 0x10);
    u_var6 = SUB42(puVar7, 0x0);
    uStack6 = u_var6;
    uStack4 = u_var4;
    uVar8 = pass1_1028_b4f2(param_4);
    // pu_var5 = (uVar8 >> 0x10);
    u_var2 = uVar8;
    uStack10 = u_var2;
    puStack8 = pu_var5;
    pass1_1038_4d6e(uVar8, CONCAT22(u_var4, u_var6), u_var2, pu_var5);
    puStack14 = CONCAT22(pu_var5, u_var2);
    uStack16 = 0x0;
    ppcVar1 = (*puStack14 + 0x10);
    u_var11 = u_var2;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var2, pu_var5);
    uStack20 = CONCAT22(extraout_dx, u_var2);
    uVar8 = pass1_1030_bcae(local_16, param_5);
    // u_var6 = (uVar8 >> 0x10);
    uStack26 = 0x0;
    loop {
        if (uStack20 <= uStack26) {
            //LAB_1028_77e7:
            if (puStack14 != 0x0) {
                ppcVar1 = *puStack14;
                (**ppcVar1)(
                    0x1030,
                    puStack14,
                    (puStack14 >> 0x10),
                    0x1,
                    u_var11,
                    pu_var5,
                    puStack14,
                    puStack14,
                );
            }
            return;
        }
        uVar8 = uStack20;
        pass1_1030_1d58(puStack14);
        u_var4 = uVar8;
        uVar9 = uVar8;
        u_var10 = (uVar8 >> 0x8);
        pass1_1028_b58e(param_4);
        pu_var3 = local_16;
        uVar8 = CONCAT22(u_var6, CONCAT11(u_var10, uVar9));
        u_var6 = extraout_DX_00;
        pass1_1030_bd74(
            pu_var3,
            param_5,
            CONCAT22(extraout_DX_00, u_var4),
            uVar8,
            param_5,
        );
        if (pu_var3 <= param_3) {
            uStack16 = 0x1;
            //       TODO: goto LAB_1028_77e7;
        }
        uStack26 += 0x1;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_780c(param_1: u16, param_2: u16, param_3: u32) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var6: u16;
    let uVar7: u16;
    let puVar8: u32;
    let puVar9: u32;
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: u32;

    puVar8 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x25);
    // pu_var5 = (puVar8 >> 0x10);
    u_var2 = puVar8;
    uVar7 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4e78(u_var2, pu_var5, param_3, puVar8);
    puStack10 = CONCAT22(pu_var5, u_var2);
    ppcVar1 = (*puStack10 + 0x10);
    u_var6 = u_var2;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var2, pu_var5);
    uStack14 = CONCAT22(extraout_dx, u_var6);
    if ((extraout_dx | u_var6) == 0x0) {
        return;
    }
    // for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
    //   ppcVar1 = (*puStack10 + 0x4);
    //   u_var4 = uStack14;
    //   (**ppcVar1)();
    //   u_var3 = u_var4;
    //   u_var6 = extraout_DX_00;
    //   pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var3,extraout_DX_00);
    //   uVar7 = 0x1030;
    //   puVar9 = struct_op_1030_73a8(CONCAT22(u_var6,u_var3));
    //   ppcVar1 = (*puVar9 + 0x24);
    //   (**ppcVar1)();
    // }
    if (puStack10 != 0x0) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(uVar7, u_var2, pu_var5, 0x1);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_78b8(param_1: u32, param_2: i32, param_3: i16, param_4: u16, param_5: u8) {
    let pu_var1: u32;
    u32 * *ppu_var2;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let pu_var5: u32;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let i_var9: i16;
    let u_var10: u32;
    let bVar11: bool;
    let bVar12: bool;
    let puVar13: U32Ptr;
    let uVar14: u16;
    let uVar15: u16;
    let uVar16: u16;
    let iVar17: i16;
    let uStack340: u16;
    let uStack338: u16;
    let puStack74: U32Ptr;
    let puStack70: U32Ptr;
    let puStack68: U32Ptr;
    let local_42: [U32Ptr; 0x4];
    let local_30: U32Ptr;
    let puStack46: U32Ptr;
    let local_1e: [U32Ptr; 0x3];
    let local_18: u32;
    let puStack20: U32Ptr;
    let puStack18: U32Ptr;
    let uStack16: u32;
    let puStack12: U32Ptr;
    let uStack10: u16;
    let puStack8: U32Ptr;
    let puStack6: u32;

    puVar6 = param_2;
    pu_var5 = *ctx.PTR_LOOP_1050_65e2;
    puStack6 = pu_var5;
    if (pu_var5 == 0x98) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
        puVar6 = param_2;
        uStack16 = pu_var5 & 0xffff | param_2 << 0x10;
        if ((pu_var5 + 0x200) == 0x8000002) {
            pass1_1020_a43e(param_4, puVar6, CONCAT22(param_4, &local_18));
            puVar13 = clear_struct_1008_3e38(CONCAT22(param_4, local_1e));
            // puVar6 = (puVar13 >> 0x10);
            pu_var1 = &local_18;
            pass1_1020_a49a(
                param_4,
                param_5,
                puVar6,
                CONCAT22(param_4, pu_var1),
                CONCAT22(param_4, local_1e),
                0x7a,
            );
            pass1_1038_4f54(uStack16, 0x1, pu_var1);
            if (pu_var1 == 0x0) {
                pass1_1020_a49a(
                    param_4,
                    param_5,
                    puVar6,
                    CONCAT22(param_4, &local_18),
                    0x0,
                    0x35,
                );
            }
        }
    }
    if ((0xe < puStack6) && (puStack6 < 0x16)) {
        puVar13 = pass1_1020_a43e(param_4, puVar6, CONCAT22(param_4, local_1e));
        local_18 = puStack6 - 0xf;
        pass1_1020_a54c(
            param_4,
            param_5,
            (puVar13 >> 0x10),
            local_1e,
            param_4,
            local_18,
        );
    }
    u_var10 = (ZEXT48(puStack6) % 0x7d);
    puVar8 = (ZEXT48(puStack6) % 0x7d);
    puVar6 = puVar8;
    if (u_var10 == 0x0) {
        local_1e[0] = puVar8;
        pass1_1008_612e(0x1, 0x64, puVar8);
        puVar8 = u_var10;
        puVar6 = local_1e[0];
        if (local_1e[0] < 0x1a) {
            pass1_1028_dc52(CONCAT22(param_4, &local_30), 0x1, 0x0, 0x400);
            loop {
                uVar7 = u_var10;
                u_var10 = ZEXT24(&local_30);
                pass1_1028_e4ec(CONCAT22(param_4, &local_30));
                puVar6 = u_var10;
                local_18 = u_var10 & 0xffff | uVar7 << 0x10;
                puVar8 = (uVar7 | puVar6);
                u_var10 = ZEXT24(puVar8);
                if puVar8 == 0x0 {
                    // goto
                    // LAB_1028_79d6;
                }
                if ((puVar6 + 0x200) == 0x8000002) == false {
                    break;
                }
            }
            pass1_1038_43cc(puVar6, uVar7, 0x1, 0x4, puVar6, puVar8);
            //LAB_1028_79d6:
            local_30 = 0x389a;
            puStack46 = 0x1008;
        }
    }
    if (puStack6 == 0x5) {
        uVar16 = SUB42(ctx.data_seg, 0x0);
        uVar15 = SUB42(s_Rebel_1050_4ffc, 0x0);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
        local_30 = puVar6;
        puStack46 = puVar8;
        pass1_1038_4d3c(CONCAT22(puVar8, puVar6), CONCAT22(uVar16, uVar15), puVar8);
    }
    if (puStack6 == 0x12c) {
        uVar16 = 0x400;
        i_var9 = 0xf;
        uVar15 = 0x1;
        puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, puVar8, param_3);
        // puVar8 = (puVar13 >> 0x10);
        puVar6 = puVar13;
        local_30 = puVar6;
        puStack46 = puVar8;
        pass1_1010_043a(puVar13, CONCAT22(uVar16, uVar15), i_var9, param_4);
    }
    if (puStack6 == 0x3d) {
        puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, puVar8, param_3);
        u_var10 = puVar13 >> 0x10;
        local_30 = puVar13;
        // puVar8 = (puVar13 >> 0x10);
        local_1e[0] = ctx.PTR_LOOP_1050_13ae;
        puVar6 = ctx.PTR_LOOP_1050_13ae;
        puStack46 = puVar8;
        if (ctx.PTR_LOOP_1050_13ae != (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
            pass1_1028_dc52(CONCAT22(param_4, local_42), 0x1, 0x0, 0x400);
            loop {
                uVar7 = u_var10;
                ppu_var2 = local_42;
                pass1_1028_e4ec(CONCAT22(param_4, ppu_var2));
                local_18 = CONCAT22(uVar7, ppu_var2);
                u_var10 = (uVar7 | ppu_var2);
                if ((uVar7 | ppu_var2) == 0x0) {
                    break;
                }
                uStack16 = (ppu_var2 + 0x1f6);
                pass1_1030_34da(uStack16);
            }
            uVar16 = 0x400;
            i_var9 = 0x10;
            uVar15 = 0x1;
            puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, 0x0, param_3);
            // puVar8 = (puVar13 >> 0x10);
            puVar6 = puVar13;
            puStack20 = puVar6;
            puStack18 = puVar8;
            pass1_1010_043a(puVar13, CONCAT22(uVar16, uVar15), i_var9, param_4);
            local_42[0] = &ULONG_1008_389a;
        }
    }
    if (puStack6 == 0x96) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
        puStack74 = CONCAT22(puVar8, puVar6);
        // uVar14 = (param_1 >> 0x10);
        pass1_1028_780c(param_1, uVar14, CONCAT22(puVar8, puVar6));
        if (puVar6 != 0x0) {
            uVar16 = 0x400;
            i_var9 = 0x1d;
            uVar15 = 0x1;
            puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, puVar8, param_3);
            // puVar8 = (puVar13 >> 0x10);
            puVar6 = puVar13;
            puStack70 = puVar6;
            puStack68 = puVar8;
            pass1_1010_043a(puVar13, CONCAT22(uVar16, uVar15), i_var9, param_4);
        }
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2);
        puStack74 = CONCAT22(puVar8, puVar6);
        pass1_1028_780c(param_1, uVar14, CONCAT22(puVar8, puVar6));
    }
    puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, puVar8, param_3);
    // puStack8 = (puVar13 >> 0x10);
    uStack10 = SUB42(puVar13, 0x0);
    puStack12 = ctx.PTR_LOOP_1050_13ae;
    if (0x2 < ctx.PTR_LOOP_1050_13ae) {
        puStack74 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, puStack8, param_3);
        //     for (puStack70 = 0x1; puStack70 < 0x9;
        //         puStack70 = (puStack70 + 0x1)) {
        //       local_42[0] = *(u32 **)(puStack74 + 0x34 + puStack70 * 0x4);
        //       if (local_42[0] == puStack6) {
        //         puVar6 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        //         local_30 = 0x1;
        //         pass1_1008_612e(0x1,0x64,0x1);
        //         puVar4 = (puStack70 - 0x7);
        //         if (puVar4 == 0x0) {
        //           bVar12 = SBORROW2(puVar6,0x32);
        //           puVar8 = puVar6 + -0x32;
        //           bVar11 = puVar6 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);
        // //LAB_1028_7b74:
        //           if (!bVar11 && bVar12 == puVar8 < 0x0) {
        //             local_30 = 0x0;
        //           }
        //         }
        //         else {
        //           puVar4 = (puStack70 - 0x8);
        //           if (puVar4 == 0x0) {
        //             bVar12 = SBORROW2(puVar6,0x19);
        //             puVar8 = puVar6 + -0x19;
        //             bVar11 = puVar8 == 0x0;
        // //             TODO: goto LAB_1028_7b74;
        //           }
        //         }
        //         local_1e[0] = puVar6;
        //         if (local_30 != 0x0) {
        //           pass1_1028_90e6(CONCAT22(param_4,&uStack340),puStack70,
        //                           param_4,param_5);
        //           puVar4 = &uStack340;
        //           fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_4,puVar4));
        //           uStack340 = 0x389a;
        //           uStack338 = 0x1008;
        //         }
        //         pass1_1008_612e(0x0,0xa,puVar4);
        //         local_18 = local_18 & 0xffff0000 | ZEXT24(puVar4);
        //         if (puStack70 == 0x7) {
        //           iVar17 = 0x7;
        //           pu_var3 = puVar4 + 0x37;
        //           i_var9 = pu_var3 >> 0xf;
        //         }
        //         else {
        //           if (puStack70 != 0x8) goto LAB_1028_7ba0;
        //           iVar17 = 0x8;
        //           pu_var3 = puVar4 + 0x32;
        //           i_var9 = (puVar4 >> 0xf) + (0xff9b < puVar4);
        //         }
        //         uVar14 = (local_42[0] >> 0x10) + i_var9 +
        //                  CARRY2(local_42[0],pu_var3);
        //         local_42[0] = CONCAT22(uVar14,local_42[0] + pu_var3);
        //         pass1_1010_ebf8(puStack74,local_42[0] + pu_var3,uVar14,iVar17);
        //       }
        // //LAB_1028_7ba0:
        //     }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_7c4e(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u8;
    let in_AF: u8;
    let puVar7: U32Ptr;
    let puVar8: u32;
    let paVar9: &mut Struct100;
    let u_var10: u8;
    let local_156: u16;
    let uStack340: u16;
    let uStack70: u16;
    let uStack68: u16;
    let i_stack66: i16;
    let uStack64: u32;
    let uStack56: u32;
    let uStack52: u16;
    let uStack50: u32;
    let puStack46: u32;
    let uStack42: u16;
    let puStack40: U32Ptr;
    let uStack38: u32;
    let local_22: [u8; 12];
    let iStack16: i16;
    let iStack14: i16;
    let uStack12: u32;
    let puStack8: U32Ptr;
    let uStack6: u16;
    let uStack4: u16;

    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    // uStack4 = (puVar7 >> 0x10);
    uStack6 = SUB42(puVar7, 0x0);
    puStack8 = ctx.PTR_LOOP_1050_13ae;
    if (0x2 < ctx.PTR_LOOP_1050_13ae) {
        uStack12 = *ctx.PTR_LOOP_1050_65e2;
        uStack12._2_2_ = (uStack12 >> 0x10);
        if (0x2 < uStack12) {
            iStack16 = uStack12 - 0x2;
            iStack14 = uStack12._2_2_ - (uStack12 < 0x2);
            u_var5 = CONCAT22(iStack14, iStack16) % 0x14;
            if (u_var5 == 0x0) {
                u_var10 = (param_4 >> 0x8);
                pass1_1028_dc52(
                    CONCAT13(u_var10, CONCAT12(param_4, local_22)),
                    0x1,
                    0x0,
                    0x400,
                );
                loop {
                    u_var4 = u_var5;
                    pu_var2 = local_22;
                    pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
                    uStack38 = CONCAT22(u_var4, pu_var2);
                    u_var5 = (u_var4 | pu_var2);
                    if ((u_var4 | pu_var2) == 0x0) {
                        break;
                    }
                    if ((pu_var2 + 0x200) != 0x8000002) {
                        puVar8 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2a);
                        u_var5 = puVar8 >> 0x10;
                        u_var4 = puVar8;
                        // puStack40 = (puVar8 >> 0x10);
                        u_var6 = 0x38;
                        uStack42 = u_var4;
                        pass1_1038_4d6e(uStack38, puVar8, u_var4, puStack40);
                        puStack46 = CONCAT22(u_var5, u_var4);
                        ppcVar1 = (*puStack46 + 0x10);
                        (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var4, u_var5);
                        uStack50 = CONCAT22(u_var5, u_var4);
                        if (puStack8 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
                            uStack52 = 0x6;
                        } else {
                            uStack52 = 0xc;
                        }
                        // for (uStack56 = 0x0; uStack56 < uStack50; uStack56 += 0x1) {
                        //   uStack64 = pass1_1030_1d7c(uStack50,u_var5,puStack46);
                        //   u_var5 = uStack64 >> 0x10;
                        //   i_var3 = uStack64;
                        //   pass1_1028_7742(param_1,(param_1 >> 0x10),0x4,uStack64,
                        //                   param_4);
                        //   u_var4 = uStack52;
                        //   if (i_var3 == 0x0) {
                        //     u_var4 = 0x19;
                        //   }
                        //   u_var6 = 0x8;
                        //   uStack68 = u_var4;
                        //   i_stack66 = i_var3;
                        //   pass1_1008_612e(0x1,0x64,u_var4);
                        //   uStack70 = u_var4;
                        //   if (u_var4 <= uStack68) {
                        //     paVar9 = pass1_1028_8fc0(
                        //                              CONCAT13(u_var10,CONCAT12(param_4,&local_156
                        //                                                      )),
                        //                              (uStack64 + 0x4),
                        //                              (uStack38 + 0x4),param_4,in_AF);
                        //     u_var5 = paVar9 >> 0x10;
                        //     u_var6 = 0x30;
                        //     fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_4,&local_156)
                        //                     );
                        //     local_156 = 0x389a;
                        //     uStack340 = 0x1008;
                        //   }
                        // }
                        if (puStack46 != 0x0) {
                            ppcVar1 = *puStack46;
                            (**ppcVar1)(
                                u_var6,
                                puStack46,
                                (puStack46 >> 0x10),
                                0x1,
                                puStack46,
                                puStack46,
                            );
                        }
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_7dfc(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16, param_5: u8) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;
    let u_var6: u8;
    let puVar7: U32Ptr;
    let puVar8: u32;
    let paVar9: &mut Struct100;
    let u_var10: u8;
    let local_158: u16;
    let uStack342: u16;
    let uStack72: u16;
    let uStack70: u16;
    let uStack68: u32;
    let uStack60: u32;
    let uStack56: u16;
    let uStack54: u16;
    let iStack52: i16;
    let uStack50: u32;
    let puStack46: u32;
    let uStack42: u16;
    let puStack40: U32Ptr;
    let uStack38: u32;
    let local_22: [u8; 12];
    let iStack16: i16;
    let iStack14: i16;
    let uStack12: u32;
    let puStack8: U32Ptr;
    let uStack6: u16;
    let uStack4: u16;

    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    // uStack4 = (puVar7 >> 0x10);
    uStack6 = SUB42(puVar7, 0x0);
    puStack8 = ctx.PTR_LOOP_1050_13ae;
    if (0x2 < ctx.PTR_LOOP_1050_13ae) {
        uStack12 = *ctx.PTR_LOOP_1050_65e2;
        uStack12._2_2_ = (uStack12 >> 0x10);
        if (0x3 < uStack12) {
            iStack16 = uStack12 - 0x3;
            iStack14 = uStack12._2_2_ - (uStack12 < 0x3);
            u_var5 = uStack12 % 0x14;
            if (u_var5 == 0x0) {
                u_var10 = (param_4 >> 0x8);
                pass1_1028_dc52(
                    CONCAT13(u_var10, CONCAT12(param_4, local_22)),
                    0x1,
                    0x0,
                    0x400,
                );
                loop {
                    u_var3 = u_var5;
                    pu_var2 = local_22;
                    pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
                    uStack38 = CONCAT22(u_var3, pu_var2);
                    u_var5 = (u_var3 | pu_var2);
                    if ((u_var3 | pu_var2) == 0x0) {
                        break;
                    }
                    if ((pu_var2 + 0x200) != 0x8000002) {
                        puVar8 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x29);
                        // puVar4 = (puVar8 >> 0x10);
                        u_var3 = puVar8;
                        uStack42 = u_var3;
                        puStack40 = puVar4;
                        pass1_1038_4d6e(uStack38, puVar8, u_var3, puVar4);
                        puStack46 = CONCAT22(puVar4, u_var3);
                        ppcVar1 = (*puStack46 + 0x10);
                        (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var3, puVar4);
                        uStack50 = CONCAT22(puVar4, u_var3);
                        u_var6 = 0x10;
                        puVar7 =
                            mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, puVar4, param_3);
                        u_var5 = puVar7 >> 0x10;
                        uStack56 = SUB42(puVar7, 0x0);
                        // uStack54 = (puVar7 >> 0x10);
                        if (puStack8 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
                            iStack52 = 0x5;
                        } else {
                            iStack52 = 0x1e;
                        }
                        // for (uStack60 = 0x0; uStack60 < uStack50; uStack60 += 0x1) {
                        //   uStack68 = pass1_1030_1d7c(uStack50,u_var5,puStack46);
                        //   u_var5 = uStack68 >> 0x10;
                        //   u_var3 = uStack68;
                        //   u_var6 = 0x8;
                        //   pass1_1008_612e(0x1,0x64,u_var3);
                        //   uStack70 = u_var3;
                        //   if ((u_var3 <= iStack52) &&
                        //      (pass1_1028_7742(param_1,(param_1 >> 0x10),0x4,uStack68,
                        //                       param_4), uStack72 = u_var3, u_var3 == 0x0)) {
                        //     paVar9 = pass1_1028_b0de(
                        //                              CONCAT13(u_var10,CONCAT12(param_4,&local_158
                        //                                                      )),
                        //                              (uStack68 + 0x4),
                        //                              (uStack38 + 0x4),param_4,param_5);
                        //     u_var5 = paVar9 >> 0x10;
                        //     u_var6 = 0x30;
                        //     fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_4,&local_158)
                        //                     );
                        //     local_158 = 0x389a;
                        //     uStack342 = 0x1008;
                        //   }
                        // }
                        if (puStack46 != 0x0) {
                            ppcVar1 = *puStack46;
                            (**ppcVar1)(
                                u_var6,
                                puStack46,
                                (puStack46 >> 0x10),
                                0x1,
                                puStack46,
                                puStack46,
                            );
                        }
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_7fb6(param_1: u32, param_2: i16, param_3: u16, param_4: u8) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;
    let u_var6: u8;
    let puVar7: u32;
    let puVar8: U32Ptr;
    let paVar9: &mut Struct100;
    let u_var10: u8;
    let local_158: u16;
    let uStack342: u16;
    let uStack72: u16;
    let uStack68: u16;
    let uStack66: u16;
    let uStack64: u32;
    let uStack56: u32;
    let iStack52: i16;
    let puStack50: U32Ptr;
    let uStack48: u16;
    let uStack46: u16;
    let uStack44: u32;
    let puStack40: u32;
    let uStack36: u16;
    let puStack34: U32Ptr;
    let uStack32: u32;
    let local_1c: [u8; 12];
    let uStack10: i16;
    let iStack8: i16;
    let uStack6: u32;

    uStack6 = *ctx.PTR_LOOP_1050_65e2;
    uStack6._2_2_ = (uStack6 >> 0x10);
    if (0xb < uStack6) {
        iStack10 = uStack6 - 0xb;
        iStack8 = uStack6._2_2_ - (uStack6 < 0xb);
        u_var5 = uStack6 % 0x32;
        if (u_var5 == 0x0) {
            u_var10 = (param_3 >> 0x8);
            pass1_1028_dc52(
                CONCAT13(u_var10, CONCAT12(param_3, local_1c)),
                0x1,
                0x0,
                0x400,
            );
            loop {
                u_var3 = u_var5;
                pu_var2 = local_1c;
                pass1_1028_e4ec(CONCAT22(param_3, pu_var2));
                uStack32 = CONCAT22(u_var3, pu_var2);
                u_var5 = (u_var3 | pu_var2);
                if ((u_var3 | pu_var2) == 0x0) {
                    break;
                }
                if ((pu_var2 + 0x200) != 0x8000002) {
                    puVar7 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x11);
                    // puVar4 = (puVar7 >> 0x10);
                    u_var3 = puVar7;
                    uStack36 = u_var3;
                    puStack34 = puVar4;
                    pass1_1038_4d6e(uStack32, puVar7, u_var3, puVar4);
                    puStack40 = CONCAT22(puVar4, u_var3);
                    ppcVar1 = (*puStack40 + 0x10);
                    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var3, puVar4);
                    uStack44 = CONCAT22(puVar4, u_var3);
                    u_var6 = 0x10;
                    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_3, puVar4, param_2);
                    u_var5 = puVar8 >> 0x10;
                    uStack48 = SUB42(puVar8, 0x0);
                    // uStack46 = (puVar8 >> 0x10);
                    puStack50 = ctx.PTR_LOOP_1050_13ae;
                    if (ctx.PTR_LOOP_1050_13ae < 0x3) {
                        iStack52 = 0x5;
                    } else {
                        iStack52 = 0x14;
                    }
                    // for (uStack56 = 0x0; uStack56 < uStack44; uStack56 += 0x1) {
                    //   u_var6 = 0x30;
                    //   uStack64 = pass1_1030_1d7c(uStack44,u_var5,puStack40);
                    //   u_var5 = uStack64 >> 0x10;
                    //   u_var3 = (uStack64 + 0x20);
                    //   uStack66 = u_var3;
                    //   if (((u_var3 != 0x0) && (u_var3 != 0x70)) && (u_var3 != 0x71)) {
                    //     u_var6 = 0x8;
                    //     pass1_1008_612e(0x1,0x64,u_var3);
                    //     uStack68 = u_var3;
                    //     if ((u_var3 <= iStack52) &&
                    //        (pass1_1028_7742(param_1,(param_1 >> 0x10),0x4,uStack64,
                    //                         param_3), uStack72 = u_var3, u_var3 == 0x0)) {
                    //       paVar9 = pass1_1028_8698(
                    //                                CONCAT13(u_var10,CONCAT12(param_3,&local_158
                    //                                                        )),
                    //                                (uStack64 + 0x4),
                    //                                (uStack32 + 0x4),param_4,param_3);
                    //       u_var5 = paVar9 >> 0x10;
                    //       u_var6 = 0x30;
                    //       fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_3,&local_158)
                    //                       );
                    //       local_158 = 0x389a;
                    //       uStack342 = 0x1008;
                    //     }
                    //   }
                    // }
                    if (puStack40 != 0x0) {
                        ppcVar1 = *puStack40;
                        (**ppcVar1)(
                            u_var6,
                            puStack40,
                            (puStack40 >> 0x10),
                            0x1,
                            puStack40,
                            puStack40,
                        );
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_816e(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_81aa(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x1b57);
    param_1.field_0x0 = 0x836e;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCFactory_1050_5002,
    );
    return param_1;
}

pub fn pass1_1028_81e0(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let i_var1: i16;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let extraout_dx: u16;
    let puStack24: u32;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    // switchD_1028_8225_caseD_0:
    loop {
        loop {
            u_var4 = param_1;
            pu_var3 = local_14;
            pass1_1028_e4ec(CONCAT22(param_3, pu_var3));
            puStack24 = CONCAT22(u_var4, pu_var3);
            param_1 = u_var4 | pu_var3;
            if (param_1 == 0x0) {
                return 0x1;
            }
            i_var1 = (pu_var3 + 0xc);
            if (i_var1 < 0x35) {
                // goto
                // code_r0x10288218;
            }
            if (0x61 < i_var1) {
                break;
            }
            if ((i_var1 < 0x5d) && (i_var1 != 0x37 && (i_var1 != 0x47))) {}
            //       TODO: goto switchD_1028_8225_caseD_1;
        }
        if ((i_var1 == 0x6a)
            || (0x8 < i_var1 + -0x6a
                && (i_var1 == 0x75
                    || i_var1 + -0x74 < 0x1
                    || (0x0 < i_var1 + -0x76 && (i_var1 + -0x78 < 0x2)))))
            == false
        {
            break;
        }
    }
    //   TODO: goto switchD_1028_8225_caseD_1;
    // code_r0x10288218:
    if false {
        // goto
        // switchD_1028_8225_caseD_0;
    }
    if true {
        param_2 = &USHORT_1050_1028;
        match (i_var1) {
            _ => {}
            //       TODO: goto switchD_1028_8225_caseD_0;
            0x1 | 0x2 | 0x3 | 0x4 | 0x6 | 0x7 | 0x8 | 0xa | 0xb | 0xc | 0xd | 0xe | 0xf | 0x11 => {}
        }
    }
    // switchD_1028_8225_caseD_1:
    if ((pu_var3 + 0x12) == 0x5) {
        ppcVar2 = (*puStack24 + 0x30);
        (**ppcVar2)(param_2);
        param_1 = extraout_dx;
    }
    //   TODO: goto switchD_1028_8225_caseD_0;
}

pub fn pass1_1028_82b4(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0x836e;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_8342(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_837e(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f);
    param_1.field_0x0 = 0x84ba;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCFillResources_1050_500c,
    );
    return param_1;
}

pub fn pass1_1028_83b4(param_1: u16, param_2: u16) -> u16 {
    let pu_var1: U32Ptr;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    loop {
        pu_var1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var1));
        if ((param_1 | pu_var1) == 0x0) {
            break;
        }
        (pu_var1 + 0x206) = 0x1;
        param_1 = param_1 | pu_var1;
    }
    return 0x1;
}

pub fn pass1_1028_8400(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0x84ba;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_848e(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_84ca(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u8,
) {
    let u_var1: u16;
    let i_var2: i16;
    let pu_var3: U32Ptr;

    struct_op_1028_d1dc(param_6, param_7, param_1, 0x3e7);
    // pu_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    (i_var2 + 0x108) = param_5;
    (i_var2 + 0x10a) = param_4;
    (i_var2 + 0x10c) = param_3;
    (i_var2 + 0x10e) = param_2;
    param_1.field_0x0 = 0x8688;
    (i_var2 + 0x2) = &USHORT_1050_1028;
    if ((i_var2 + 0x108) == 0x1) {
        u_var1 = ctx.s_max_1050_501c;
    } else {
        u_var1 = ctx.s_min_1050_5020;
    }
    sys_1000_3f9c(
        (i_var2 + 0x8),
        pu_var3,
        ctx.s_SCForceMorale__s_for_colony__08l_1050_5024,
        ctx.data_seg,
        u_var1,
        &stack0xfffe,
        pu_var3,
        0x1000,
        param_6,
        param_7,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_853e(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let u_var1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x108) == 0x0) {
        return 0x0;
    }
    u_var1 = (i_var3 + 0x10e);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    if ((i_var3 + 0x108) == 0x1) {
        u_var2 = 0x3e8;
    } else {
        u_var2 = 0x0;
    }
    pass1_1038_4d0e(CONCAT22(param_3, param_2), u_var2);
    return 0x1;
}

pub fn pass1_1028_858c(param_1: u32, param_2: &mut Struct318, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct319;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x112, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10a = iVar5.field_0x10a;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x10e = iVar5.field_0x10e;
        *puStack10 = 0x8688;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_865c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_8698(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u8,
    param_5: u16,
) -> Struct100 {
    pass1_1028_6af2(param_1, param_2, param_3, param_5, param_4);
    param_1.field_0x0 = 0x87e0;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_86c2(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let paVar1: &mut Struct67;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;

    uVar7 = 0x0;
    i_var8 = 0x1d;
    u_var5 = 0x1;
    u_var6 = 0x0;
    u_var3 = 0x0;
    i_var4 = 0x0;
    u_var2 = 0x0;
    paVar1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
    post_win_msg_1008_a0e4(
        paVar1,
        CONCAT22(u_var3, u_var2),
        i_var4,
        u_var5,
        CONCAT22(uVar7, u_var6),
        i_var8,
        0x1008,
        param_4,
    );
    pass1_1028_6b2c(param_1, param_4);
    return;
}

pub fn pass1_1028_86f4(param_1: u32, param_2: &mut Struct320, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct321;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x110, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = 0x6e50;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0x87e0;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_87b4(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_8920(param_1: u32, param_2: u16, param_3: u16, param_4: u8) {
    u16 * *ppuVar1;
    let u_var2: u32;
    let ppc_var3: u32;
    u16 * *ppuVar4;
    let iVar5: i16;
    let BVar6: bool;
    let uVar7: u32;
    let puVar8: U32Ptr;
    let extraout_dx: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let iVar11: i16;
    let iVar12: &mut Struct684;
    let iVar13: i16;
    let uVar14: u16;
    let uVar15: u8;
    let uVar16: u16;
    u32 * local_156[0x43];
    let local_4a: u32;
    let iStack70: i16;
    let uStack68: u32;
    let uStack56: u32;
    let puStack52: u32;
    let uStack48: u16;
    let puStack46: U32Ptr;
    let uStack42: u32;
    let local_26: [u8; 4];
    let uStack34: u32;
    let uStack30: u32;
    let uStack26: u32;
    let uStack22: u32;
    let puStack18: U32Ptr;
    let uStack14: u16;
    let local_c: [u8; 2];
    let local_a: [u8; 2];
    let local_8: [u8; 2];
    let uStack6: u32;

    // iVar13 = (param_1 >> 0x10);
    iVar11 = param_1;
    ppuVar1 = (iVar11 + 0x114);
    ppuVar4 = ppuVar1;
    pass1_1030_64ce(
        param_3,
        ppuVar1,
        param_2,
        ctx.PTR_LOOP_1050_5740,
        (param_1 & 0xffff0000 | ZEXT24(ppuVar1)),
        (iVar11 + 0x108),
        CONCAT22(param_3, local_26),
    );
    uStack6 = *ppuVar4;
    uVar15 = (param_3 >> 0x8);
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | ZEXT24(ppuVar1)),
        CONCAT22(param_3, local_c),
        CONCAT13(uVar15, CONCAT12(param_3, local_a)),
        CONCAT22(param_3, local_8),
    );
    puStack46 = uStack6;
    uStack56 = uStack6;
    uStack56._3_1_ = (uStack6 >> 0x18);
    uStack14 = (uStack56._3_1_ != '\0');
    if (uStack14 == 0x0) {
        uVar7 = (iVar11 + 0x114);
        pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x500);
        puStack18 = (uVar7 & 0xffff | ZEXT24(uStack6._2_2_) << 0x10);
        uVar14 = 0x1030;
        pass1_1030_61fe(
            ctx.PTR_LOOP_1050_5740,
            uVar7 & 0xffff | ZEXT24(uStack6._2_2_) << 0x10,
            param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), iVar11 + 0x114),
            (iVar11 + 0x108),
            uVar7,
            uStack6._2_2_,
            param_3,
        );
        uStack56 = 0x0;
        if (((iVar11 + 0x11a) == 0xa) || ((iVar11 + 0x11a) == 0x37)) {
            if ((iVar11 + 0x11a) == 0x37) {
                uStack56 = (iVar11 + 0x10c);
            }
            iVar5 = iVar11 + 0x114;
            pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x400);
            (iVar11 + 0x10c) = iVar5;
            (iVar11 + 0x10e) = uStack6._2_2_;
            puStack46 =
                mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_3, uStack6._2_2_, iVar13);
            uStack6._2_2_ = (puStack46 >> 0x10);
            uVar7 = puStack46 & 0xffff;
            uVar14 = 0x1018;
            pass1_1018_0196(
                uVar7 | ZEXT24(uStack6._2_2_) << 0x10,
                (iVar11 + 0x10c),
                (iVar11 + 0x108),
                uVar7,
                uStack6._2_2_,
                param_3,
            );
            iVar5 = uVar7;
            if ((iVar11 + 0x110) != 0x0) {
                u_var2 = (iVar11 + 0x10c);
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
                uStack42 = CONCAT22(uStack6._2_2_, iVar5);
                uVar7 = (iVar11 + 0x110);
                (iVar5 + 0x200) = uVar7;
                uStack68 = uVar7;
            }
        }
        uStack6._0_2_ = uVar7;
        u_var2 = (iVar11 + 0x10c);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        puStack52 = CONCAT22(uStack6._2_2_, uStack6);
        puVar8 = (uStack6._2_2_ | uStack6);
        if (puVar8 != 0x0) {
            ppc_var3 = (*puStack52 + 0x8);
            (**ppc_var3)(
                uVar14,
                uStack6,
                uStack6._2_2_,
                0x0,
                puStack18,
                (puStack18 >> 0x10),
                0x0,
            );
            puVar8 = extraout_dx;
        }
    } else {
        puStack18 = uStack6;
        puVar8 = uStack6._2_2_;
    }
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, puStack18);
    uStack22 = CONCAT22(puVar8, uStack6);
    pass1_1030_73ee(
        CONCAT13((puVar8 >> 0x8), CONCAT12(puVar8, uStack6)),
        (iVar11 + 0x10c),
        puVar8,
    );
    BVar6 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (iVar11 + 0x11a), 0x31);
    if ((BVar6 == 0x0) && ((iVar11 + 0x11c) == 0x0)) {
        local_4a = (uStack22 + 0xc);
        iStack70 = (uStack22 + 0x10);
        uStack68 = uStack68 & 0xffff0000 | ZEXT24(&local_4a);
        if (iStack70 < 0x1) {
            uStack48 = 0x5;
        } else {
            uStack48 = 0x6;
        }
        (uStack22 + 0x14) = uStack48;
    }
    uStack26 = (uStack22 + 0x16);
    uVar9 = (uStack22 + 0x18);
    if ((uVar9 | uStack26) != 0x0) {
        struct_1030_e4fa(
            CONCAT13(uVar15, CONCAT12(param_3, local_156)),
            uStack26 & 0xffff | uVar9 << 0x10,
            param_3,
            param_4,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_3, local_156));
        local_156[0] = &ULONG_1008_389a;
    }
    uStack30 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2, uVar9, 0x7);
    uVar9 = uStack30;
    u_var10 = (uStack30 >> 0x10) | uVar9;
    if (u_var10 == 0x0) {
        return;
    }
    pass1_1030_7e5a(uStack22, uStack30, u_var10);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack30);
    uStack34 = CONCAT22(u_var10, uVar9);
    uVar14 = SUB42(puStack18, 0x0);
    // uVar16 = (puStack18 >> 0x10);
    uVar15 = u_var10;
    iVar12 = *uStack34;
    ppc_var3 = &iVar12.field_0x4;
    (**ppc_var3)();
    ppc_var3 = &iVar12.field_0x20;
    (**ppc_var3)(0x1030, uStack34, uVar9, uVar15, uVar14, uVar16);
    ppc_var3 = &iVar12.field_0x18;
    (**ppc_var3)(0x1030, uStack34, (uStack34 >> 0x10), 0x1);
    if ((iVar11 + 0x11a) == 0x37) {
        (uStack34 + 0x20) = (iVar11 + 0x10c);
    }
    (iVar11 + 0x120) = uStack34;
    return;
}

pub fn pass1_1028_8c46(param_1: u32, param_2: &mut Struct322, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct323;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x124, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        param_2.field_0x118 = iVar5.field_0x118;
        param_2.field_0x11a = iVar5.field_0x11a;
        param_2.field_0x11c = iVar5.field_0x11c;
        param_2.field_0x11e = iVar5.field_0x11e;
        param_2.field_0x120 = iVar5.field_0x120;
        *puStack10 = 0x8d8e;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_8d62(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_8d9e(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: u8,
) {
    let i_var1: i16;
    let u_var2: u16;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x3e8);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x108) = param_4;
    (i_var1 + 0x10c) = param_3;
    (i_var1 + 0x110) = param_2;
    (i_var1 + 0x114) = 0x0;
    param_1.field_0x0 = 0x8fb0;
    (i_var1 + 0x2) = &USHORT_1050_1028;
    return;
}

pub fn pass1_1028_8dec(param_1: U32Ptr) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x8fb0;
    (i_var1 + 0x2) = &USHORT_1050_1028;
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x114), 0x1000);
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_8e1e(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    pass1_1030_355c((param_2 + 0x1f6), (param_1 + 0x114));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_8e5c(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = (i_var3 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    u_var2 = (param_2 + 0x1f6);
    pass1_1030_35a4(u_var2, (i_var3 + 0x110), param_3, 0x1030, param_4);
    (i_var3 + 0x114) = u_var2;
    (i_var3 + 0x116) = param_3;
    return;
}

pub fn pass1_1028_8ea6(param_1: u32, param_2: &mut Struct324, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct325;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x118, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    iVar5 = param_1;
    // u_var6 = (param_1 >> 0x10);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = 0x8fb0;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    iVar5.field_0x114 = 0x0;
    return;
}

pub fn pass1_1028_8f8a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_8dec(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_8fc0(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u8,
) -> Struct100 {
    pass1_1028_6af2(param_1, param_2, param_3, param_4, param_5);
    param_1.field_0x0 = 0x90d6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_8fea(param_1: u32, param_2: &mut Struct326, param_3: U32Ptr) {
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct327;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;
    let pu_var1: u32;

    mem_op_1000_179c(0x110, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = 0x6e50;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0x90d6;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_90aa(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_90e6(
    param_1: &mut Struct100,
    param_2: u16,
    param_3: u16,
    param_4: u8,
) -> Struct100 {
    let u_var1: u16;

    struct_op_1028_d1dc(param_3, param_4, param_1, 0x1387);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x108) = param_2;
    param_1.field_0x0 = 0x932c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9114(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let paVar4: &mut Struct67;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let i_var8: i16;
    let uVar9: u16;
    let iVar10: i16;
    let uStack10: u16;

    paVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
    u_var3 = param_1;
    iVar10 = (u_var3 + 0x108);
    if (iVar10 - 0x1 < 0x8) {
        uStack10 = *ctx.PTR_LOOP_1050_65e2;
        i_var8 = (*ctx.PTR_LOOP_1050_65e2 >> 0x10);
        match (iVar10) {
            0x1 => {
                iVar10 = 0x16;
            }

            0x2 => {
                iVar10 = 0x17;
            }

            0x3 => {
                iVar10 = 0x18;
            }

            0x4 => {
                iVar10 = 0x1b;
            }

            0x5 => {
                iVar10 = 0x1f;
            }

            0x6 => {
                iVar10 = 0x24;
            }

            0x7 => {
                pass1_1008_612e(0x0, 0x14, u_var3);
                pu_var2 = ((u_var3 >> 0xf) + (0xff91 < u_var3));
                u_var6 = uStack10 + u_var3 + 0x6e;
                puVar7 = pu_var2 + CARRY2(uStack10, u_var3 + 0x6e) + i_var8;
                iVar10 = 0x7;
                pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, pu_var2, param_3);
                // u_var1 = (pu_var5 >> 0x10);
                u_var3 = pu_var5;
                pass1_1010_ebf8(pu_var5, u_var6, puVar7, iVar10);
                pass1_1008_612e(0x1, 0x64, u_var3);
                if (0x32 < u_var3) {
                    return;
                }
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
                pass1_1038_4900(CONCAT22(u_var1, u_var3));
                iVar10 = 0x2c;
            }

            0x8 => {
                pass1_1008_612e(0x0, 0x14, u_var3);
                pu_var2 = ((u_var3 >> 0xf) + (0xff9b < u_var3));
                u_var6 = uStack10 + u_var3 + 0x64;
                puVar7 = pu_var2 + CARRY2(uStack10, u_var3 + 0x64) + i_var8;
                i_var8 = 0x8;
                pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, pu_var2, param_3);
                // u_var1 = (pu_var5 >> 0x10);
                iVar10 = pu_var5;
                pass1_1010_ebf8(pu_var5, u_var6, puVar7, i_var8);
                if (0x19 < u_var3) {
                    return;
                }
                u_var3 = 0x1;
                uVar9 = 0x2;
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
                pass1_1038_43cc(iVar10, u_var1, u_var3, uVar9, iVar10, u_var1);
                iVar10 = 0x2d;
            }
        }
        post_win_msg_1008_a0e4(paVar4, 0x0, 0x0, 0x1, 0x0, iVar10, 0x1008, param_4);
    }
    return;
}

pub fn pass1_1028_9264(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let iVar5: i16;
    let puVar6: u32;
    let uVar7: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10a, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // uVar7 = (param_1 >> 0x10);
        iVar5 = param_1;
        (param_2 + 0x4) = (iVar5 + 0x4);
        pu_var3 = (iVar5 + 0x8);
        puVar6 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = puVar6;
        //   puVar6 = puVar6 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        (param_2 + 0x108) = (iVar5 + 0x108);
        *puStack10 = 0x932c;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_9300(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_93d4(param_1: u32, param_2: u16, param_3: i16, param_4: u16, param_5: u8) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let local_112: [u8; 0x10c];
    let uStack6: u32;

    ctx.PTR_LOOP_1050_50ca = 0x0;
    ctx.PTR_LOOP_1050_50cc = 0x0;
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    uStack6 = pass1_1028_e2e0(ctx.PTR_LOOP_1050_65e2, param_2, 0x7);
    // u_var3 = (uStack6 >> 0x10);
    u_var2 = uStack6;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    (i_var4 + 0x11e) = u_var2;
    (i_var4 + 0x120) = u_var3;
    u_var2 = i_var4 + 0x114;
    ppcVar1 = ((i_var4 + 0x11e) + 0x1c);
    (**ppcVar1)();
    if (u_var2 != 0x0) {
        pass1_1028_9624(param_1, u_var2, extraout_dx, param_4, param_3, param_5);
        ppcVar1 = ((i_var4 + 0x11e) + 0x20);
        (**ppcVar1)();
        ppcVar1 = ((i_var4 + 0x11e) + 0x18);
        (**ppcVar1)();
        pass1_1028_9600(param_1, extraout_DX_00, param_3, param_4, param_5);
        return;
    }
    (i_var4 + 0x11e) = 0x0;
    struct_1030_e4fa(CONCAT22(param_4, local_112), uStack6, param_4, param_5);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_4, local_112));
    if (ctx.PTR_LOOP_1050_50ca == 0x0) {
        ctx.PTR_LOOP_1050_50ca = 0x6ad;
    }
    return;
}

pub fn pass1_1028_94e4(param_1: u32, param_2: &mut Struct328, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct329;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x124, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        param_2.field_0x118 = iVar5.field_0x118;
        param_2.field_0x11a = iVar5.field_0x11a;
        param_2.field_0x11c = iVar5.field_0x11c;
        param_2.field_0x11e = iVar5.field_0x11e;
        param_2.field_0x122 = iVar5.field_0x122;
        *puStack10 = 0x9934;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_9600(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16, param_5: u8) {
    let pu_var1: U32Ptr;
    let local_6: [u8; 4];

    pu_var1 = pass1_1020_a43e(param_4, param_2, CONCAT22(param_4, local_6));
    pass1_1020_a80e(
        local_6,
        param_4,
        (param_1 + 0x11a),
        local_6,
        (pu_var1 >> 0x10),
        param_4,
        param_5,
        param_3,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9624(
    param_1: u32,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u16,
    param_5: i16,
    param_6: u8,
) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let BVar5: bool;
    let uVar7: u32;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let i_var9: &mut Struct688;
    let uVar8: u16;
    let uVar9: u16;
    let puVar10: U32Ptr;
    let uStack332: u16;
    let uStack330: u16;
    let uStack64: u16;
    let uStack62: u32;
    let iStack58: i16;
    let uStack56: u32;
    let puStack46: u32;
    let uStack42: u32;
    let local_26: [u8; 4];
    let uStack34: u16;
    let puStack32: U32Ptr;
    let uStack30: u32;
    let uStack26: u32;
    let puStack22: u32;
    let local_12: [u8; 2];
    let local_10: [u8; 2];
    let local_e: [u8; 2];
    let uStack12: u16;
    let uStack10: u32;
    let puStack6: U32Ptr;
    let puVar6: u32;

    // uVar8 = (param_1 >> 0x10);
    i_var9 = param_1;
    uVar7 = i_var9.field_0x10c;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar7);
    &i_var9.field_0x110 = param_2;
    (&i_var9.field_0x110 + 0x2) = param_3;
    puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, param_3, param_5);
    uStack10._2_2_ = (puStack6 >> 0x10);
    pu_var2 = &i_var9.field_0x114;
    pass1_1030_64ce(
        param_4,
        pu_var2,
        uStack10._2_2_,
        ctx.PTR_LOOP_1050_5740,
        (param_1 & 0xffff0000 | ZEXT24(pu_var2)),
        i_var9.field_0x108,
        CONCAT22(param_4, local_26),
    );
    uStack56 = *pu_var2;
    uStack56._3_1_ = (uStack56 >> 0x18);
    uStack12 = (uStack56._3_1_ != '\0');
    uVar9 = 0x1008;
    puStack46 = uStack56;
    uStack10 = uStack56;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | ZEXT24(&i_var9.field_0x114)),
        CONCAT22(param_4, local_12),
        CONCAT22(param_4, local_10),
        CONCAT22(param_4, local_e),
    );
    if (uStack12 == 0x0) {
        pu_var2 = &i_var9.field_0x114;
        pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x500);
        puStack22 = CONCAT22(uStack10._2_2_, pu_var2);
        uVar9 = 0x1030;
        pass1_1030_61fe(
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(uStack10._2_2_, pu_var2),
            param_1 & 0xffff0000 | ZEXT24(&i_var9.field_0x114),
            i_var9.field_0x108,
            pu_var2,
            uStack10._2_2_,
            param_4,
        );
        if ((i_var9.field_0x11a == 0xa) || (i_var9.field_0x11a == 0x37)) {
            if (i_var9.field_0x11a == 0x37) {
                uStack56 = i_var9.field_0x11e;
                uStack10._2_2_ = (&i_var9.field_0x11e + 0x2);
                uStack42 = i_var9.field_0x10c;
                (uStack56 + 0x20) = uStack42;
            }
            pu_var2 = &i_var9.field_0x114;
            pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x400);
            &i_var9.field_0x10c = pu_var2;
            (&i_var9.field_0x10c + 0x2) = uStack10._2_2_;
            uVar9 = 0x1018;
            pass1_1018_0196(
                puStack6,
                CONCAT22(uStack10._2_2_, &i_var9.field_0x10c),
                i_var9.field_0x108,
                pu_var2,
                uStack10._2_2_,
                param_4,
            );
            if (i_var9.field_0x11a == 0xa) {
                uVar9 = 0x1010;
                pass1_1010_ed22(puStack6, i_var9.field_0x10c, param_4);
            }
        }
        uVar7 = i_var9.field_0x10c;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar7);
        &i_var9.field_0x110 = pu_var2;
        (&i_var9.field_0x110 + 0x2) = uStack10._2_2_;
        u_var4 = uStack10._2_2_ | &i_var9.field_0x110;
        puVar6 = u_var4;
        if (u_var4 == 0x0) {
            // goto
            // LAB_1028_9807;
        }
        u_var3 = SUB42(puStack22, 0x0);
        // puVar10 = (puStack22 >> 0x10);
    } else {
        puStack22 = uStack10;
        puVar6 = uStack10;
        if (i_var9.field_0x11a != 0x75) {
            // goto
            // LAB_1028_9807;
        }
        u_var3 = SUB42(uStack10, 0x0);
        puVar10 = uStack10._2_2_;
        uStack10._2_2_ = (&i_var9.field_0x110 + 0x2);
    }
    ppcVar1 = (*i_var9.field_0x110 + 0x8);
    (**ppcVar1)(
        uVar9,
        &i_var9.field_0x110,
        uStack10._2_2_,
        0x0,
        u_var3,
        puVar10,
        0x0,
    );
    uStack10._2_2_ = extraout_dx;
    //LAB_1028_9807:
    uVar9 = SUB42(puVar6, 0x0);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, puStack22);
    uStack26 = CONCAT22(uStack10._2_2_, uVar9);
    pass1_1030_73ee(
        CONCAT22(uStack10._2_2_, uVar9),
        i_var9.field_0x10c,
        uStack10._2_2_,
    );
    BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var9.field_0x11a, 0x31);
    puStack32 = uStack10._2_2_;
    if ((BVar5 == 0x0) && (i_var9.field_0x122 == 0x0)) {
        uStack62 = (uStack26 + 0xc);
        iStack58 = (uStack26 + 0x10);
        uStack56 = (uStack56 & 0xffff0000 | ZEXT24(&uStack62));
        if (iStack58 < 0x1) {
            uStack64 = 0x5;
        } else {
            uStack64 = 0x6;
        }
        (uStack26 + 0x14) = uStack64;
        puStack32 = uStack26._2_2_;
    }
    uVar7 = (uStack26 + 0x16);
    uStack30 = uVar7;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar7);
    uStack34 = uVar7;
    if (uStack30 != 0x0) {
        struct_1030_e4fa(CONCAT22(param_4, &uStack332), uStack30, param_4, param_6);
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_4, &uStack332));
        uStack332 = 0x389a;
        uStack330 = 0x1008;
    }
    ppcVar1 = (*i_var9.field_0x11e + 0x4);
    (**ppcVar1)();
    puVar6 = i_var9.field_0x11e;
    pass1_1030_7e5a(uStack26, (puVar6 + 0x4), extraout_DX_00);
    return;
}

pub fn pass1_1028_9908(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_9944(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: u8,
) {
    let i_var1: &mut Struct699;
    let u_var1: u16;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x1387);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x108 = param_4;
    i_var1.field_0x10c = param_3;
    i_var1.field_0x110 = param_2;
    i_var1.field_0x114 = 0x0;
    param_1.field_0x0 = 0x9c52;
    i_var1.field_0x2 = &USHORT_1050_1028;
    return;
}

pub fn pass1_1028_9992(param_1: U32Ptr) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x9c52;
    (i_var1 + 0x2) = &USHORT_1050_1028;
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x114), 0x1000);
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_99c4(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    pass1_1030_355c((param_2 + 0x1f6), (param_1 + 0x114));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9a02(param_1: u32, param_2: i16, param_3: u16, param_4: u16, param_5: i16) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let paVar9: &mut Struct67;
    let u_var10: u16;
    let u_var11: u8;
    let uVar12: u8;
    let uVar13: u16;
    let uVar14: u16;
    let iVar15: i16;
    let local_30: [u8; 12];
    let iStack30: i16;
    let uStack26: u16;
    let uStack22: u16;
    let uStack20: u16;
    let lStack18: i32;
    let uStack10: u32;
    let uStack6: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    u_var1 = (iVar6 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    uStack6 = CONCAT22(param_3, param_2);
    u_var3 = (param_2 + 0x1f6);
    uStack10 = u_var3;
    pass1_1030_3694(u_var3, 0x0, (iVar6 + 0x110), param_3, 0x1030, param_4);
    u_var4 = u_var3;
    (iVar6 + 0x114) = u_var4;
    (iVar6 + 0x116) = param_3;
    pass1_1030_38b8();
    if ((param_3 | u_var4) == 0x0) {
        lStack18 = (uStack6 + 0x200);
        puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, 0x0, param_5);
        // uStack20 = (puVar8 >> 0x10);
        uStack22 = SUB42(puVar8, 0x0);
        if (lStack18 == 0x8000002) {
            iVar6 = 0x1f;
        } else {
            iVar6 = 0xb;
        }
        pass1_1010_043a(puVar8, (uStack6 + 0x4), iVar6, param_4);
        if (lStack18 == 0x8000001) {
            uVar7 = 0x2;
        } else {
            uVar7 = 0x1;
        }
        u_var4 = 0x800;
        lStack18 = CONCAT22(0x800, uVar7);
        pass1_1038_349e(uStack6, CONCAT22(0x800, uVar7));
        iStack30 = 0x0;
        uStack26 = 0x0;
        pass1_1028_dc52(
            CONCAT13((param_4 >> 0x8), CONCAT12(param_4, local_30)),
            0x1,
            0x0,
            0x400,
        );
        loop {
            pu_var2 = local_30;
            pass1_1028_e4ec(CONCAT22(param_4, pu_var2));
            uStack6 = CONCAT22(u_var4, pu_var2);
            u_var5 = u_var4 | pu_var2;
            if (u_var5 == 0x0) {
                break;
            }
            if ((pu_var2 + 0x200) == 0x8000002) {
                uStack26 = 0x1;
                u_var4 = u_var5;
            } else {
                iStack30 = 0x1;
                u_var4 = u_var5;
            }
        }
        if (iStack30 == 0x0) {
            uVar14 = 0x0;
            iVar15 = 0x3c;
            u_var11 = 0x1;
            uVar12 = 0x0;
            uVar13 = 0x0;
            u_var10 = 0x0;
            iVar6 = 0x0;
            uVar7 = 0x0;
            paVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, 0x0, param_5);
            post_win_msg_1008_a0e4(
                paVar9,
                CONCAT22(u_var10, uVar7),
                iVar6,
                CONCAT11(uVar12, u_var11),
                CONCAT22(uVar14, uVar13),
                iVar15,
                0x1008,
                param_4,
            );
        }
    }
    return;
}

pub fn pass1_1028_9b48(param_1: u32, param_2: &mut Struct330, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct331;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x118, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    iVar5 = param_1;
    // u_var6 = (param_1 >> 0x10);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = 0x9c52;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    iVar5.field_0x114 = 0x0;
    return;
}

pub fn pass1_1028_9c2c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_9992(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_9c90(param_1: u32) -> u16 {
    let u_var1: u16;
    let u_var2: u16;

    u_var1 = (param_1 + 0x108) - 0x3e8;
    if ((u_var1 < 0x3a99) && (u_var1 % 0x3e8 == 0x0)) {
        // WARNING: Could not recover jumptable at 0x10289dc0. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var2 = ((u_var1 / 0x3e8) * 0x2 + -0x623a)();
        return u_var2;
    }
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9ca0(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_acb6(CONCAT22(param_2, param_1 + -0x108), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x108));
    (param_1 + -0x108) = 0x389a;
    (param_1 + -0x106) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9cca(param_1: i16, param_2: u16) -> u16 {
    let in_AF: u8;

    pass1_1038_28d8(CONCAT22(param_2, param_1 + -0x108), param_2, in_AF);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x108));
    (param_1 + -0x108) = 0x389a;
    (param_1 + -0x106) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9cd8(param_1: u16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_a866(CONCAT22(param_2, param_1 - 0x108), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 - 0x108));
    (param_1 - 0x108) = 0x389a;
    (param_1 - 0x106) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9ce6(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_6e60(CONCAT22(param_2, param_1 + -0x108), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x108));
    (param_1 + -0x108) = 0x389a;
    (param_1 + -0x106) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9cf4(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_ab32(CONCAT22(param_2, param_1 + -0x108), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x108));
    (param_1 + -0x108) = 0x389a;
    (param_1 + -0x106) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d02(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1030_e09e(CONCAT22(param_2, param_1 + -0x108), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x108));
    (param_1 + -0x108) = 0x389a;
    (param_1 + -0x106) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d10(param_1: i16, param_2: i16, param_3: u16, param_4: u8) -> u16 {
    pass1_1038_0ba6(
        CONCAT22(param_3, param_1 + -0x220),
        param_2,
        param_3,
        param_4,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_3, param_1 + -0x220));
    (param_1 + -0x220) = 0x389a;
    (param_1 + -0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d3a(param_1: u16, param_2: u16) -> u16 {
    let in_AF: u8;

    pass1_1028_9ec6(CONCAT22(param_2, param_1 - 0x220), param_2, in_AF);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 - 0x220));
    (param_1 - 0x220) = 0x389a;
    (param_1 - 0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d48(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1030_eb50(CONCAT22(param_2, param_1 + -0x220), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x220));
    (param_1 + -0x220) = 0x389a;
    (param_1 + -0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d56(param_1: i16, param_2: u16) -> u16 {
    let in_AF: u8;

    pass1_1028_81aa(CONCAT22(param_2, param_1 + -0x220), param_2, in_AF);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x220));
    (param_1 + -0x220) = 0x389a;
    (param_1 + -0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d64(param_1: u16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_a9be(CONCAT22(param_2, param_1 - 0x220), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 - 0x220));
    (param_1 - 0x220) = 0x389a;
    (param_1 - 0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d72(param_1: u16, param_2: u16) -> u16 {
    pass1_1028_74ae(param_1 - 0x220, param_2);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 - 0x220));
    (param_1 - 0x220) = 0x389a;
    (param_1 - 0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d80(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1030_ecc2(CONCAT22(param_2, param_1 + -0x220), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x220));
    (param_1 + -0x220) = 0x389a;
    (param_1 + -0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d8e(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_a706(CONCAT22(param_2, param_1 + -0x220), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x220));
    (param_1 + -0x220) = 0x389a;
    (param_1 + -0x21e) = 0x1008;
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9d9c(param_1: i16, param_2: u16, param_3: u8) -> u16 {
    pass1_1028_6fc0(CONCAT22(param_2, param_1 + -0x220), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, param_1 + -0x220));
    (param_1 + -0x220) = 0x389a;
    (param_1 + -0x21e) = 0x1008;
    return 0x1;
}

pub fn pass1_1028_9dee(param_1: u32, param_2: &mut Struct332, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct333;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10a, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        *puStack10 = 0x9eb6;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_9e8a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_9ec6(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, (s_noth_bmp_1050_2321 + 0x6));
    param_1.field_0x0 = 0xa6f6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), 0x105050f0);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_9efc(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u16,
    param_4: i16,
    param_5: u16,
    param_6: u8,
) {
    let lVar1: i32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let i_var4: i16;
    let lVar5: i32;
    let puVar6: U32Ptr;
    let in_register_0000000a: u16;
    let uVar7: u32;
    let paVar8: &mut Struct67;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let local_18: u16;
    let uStack22: u16;
    let puStack6: U32Ptr;
    let puStack4: U32Ptr;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
    uVar7 = CONCAT22(in_register_0000000a, param_3 | param_2);
    if ((param_3 | param_2) != 0x0) {
        puStack6 = param_2;
        puStack4 = param_3;
        pass1_1028_dc52(
            CONCAT13((param_5 >> 0x8), CONCAT12(param_5, &local_18)),
            0x1,
            0x0,
            0x400,
        );
        loop {
            pu_var2 = &local_18;
            pass1_1028_e4ec(CONCAT22(param_5, pu_var2));
            puStack4 = uVar7;
            puStack6 = pu_var2;
            if ((puStack4 | pu_var2) == 0x0) {
                break;
            }
            lVar1 = (pu_var2 + 0x100);
            u_var3 = pu_var2[0x101];
            uVar7 = uVar7 & 0xffff0000 | u_var3;
            if (pu_var2[0xff] != 0x0) {
                // u_var10 = (param_1 >> 0x10);
                lVar5 = lVar1;
                if ((lVar1 != 0x2) || (u_var3 != 0x800)) {
                    pass1_1028_a3ae(
                        param_1,
                        u_var10,
                        CONCAT22(puStack4, pu_var2),
                        uVar7,
                        param_4,
                        param_5,
                        param_6,
                        lVar1,
                    );
                }
                u_var3 = lVar5;
                pass1_1028_a28a(param_1, u_var10, CONCAT22(puStack4, puStack6));
                if ((uVar7 < 0x1) && (uVar7 < 0x0 || (u_var3 < 0x64))) {
                    pass1_1028_a4ee(param_1, CONCAT22(puStack4, puStack6), param_4, param_5);
                }
                if (lVar1 != 0x8000002) {
                    pass1_1038_42cc(CONCAT22(puStack4, puStack6), param_5);
                    puVar6 = (uVar7 | u_var3);
                    uVar7 = uVar7 & 0xffff0000 | ZEXT24(puVar6);
                    if (puVar6 != 0x0) {
                        paVar8 =
                            mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_5, puVar6, param_4);
                        uVar7 = uVar7 & 0xffff0000 | paVar8 >> 0x10;
                        post_win_msg_1008_a0e4(
                            paVar8,
                            0x0,
                            u_var3,
                            puStack6[0x104],
                            (puStack6 + 0x2),
                            0x2,
                            0x1008,
                            param_5,
                        );
                    }
                }
            }
        }
        local_18 = 0x389a;
        uStack22 = 0x1008;
        puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x8, param_5, 0x0, param_4);
        // puVar6 = (puVar9 >> 0x10);
        i_var4 = puVar9;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
        puStack6 = i_var4;
        puStack4 = puVar6;
        pass1_1010_9f72(puVar9, 0x3e, param_5);
        if (i_var4 != 0x0) {
            i_var4 = pass1_1010_96d0(puVar9);
            if (i_var4 < 0x1) {
                if (i_var4 < 0x0) {
                    i_var4 = (puStack6 + 0x1f6);
                    pass1_1030_38b8();
                    if ((puVar6 < 0x1) && (puVar6 < 0x0 || (i_var4 == 0x0))) {
                        paVar8 =
                            mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_5, puVar6, param_4);
                        post_win_msg_1008_a0e4(
                            paVar8,
                            0x0,
                            0x0,
                            0x1,
                            (puStack6 + 0x4),
                            0x6,
                            0x1008,
                            param_5,
                        );
                    }
                }
            } else {
                puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_5, puVar6, param_4);
                // puVar6 = (puVar9 >> 0x10);
                post_win_msg_1008_a0e4(
                    (puVar9 & 0xffff | ZEXT24(puVar6) << 0x10),
                    0x0,
                    i_var4,
                    (puStack6 + 0x208),
                    0x4000001,
                    0x2,
                    0x1008,
                    param_5,
                );
                puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, puVar6, param_4);
                pass1_1010_043a(puVar9, (puStack6 + 0x4), 0x14, param_5);
            }
        }
    }
    return;
}

pub fn pass1_1028_a0fa(param_1: u32, param_2: &mut Struct334, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = &param_2.field_0x8;
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0xa6f6;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_a188(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    param_5: u32,
    param_6: u16,
) {
    let u_var1: u32;
    let lVar2: i32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let lVar8: i32;
    let lVar9: i32;
    let u_var10: u16;
    let iVar11: i16;
    let iVar12: i16;
    let puVar13: U32Ptr;
    let uVar14: u16;
    let puVar15: U32Ptr;
    let uStack18: u16;
    let uStack16: u16;
    let uStack14: u16;
    let iStack12: i16;

    // uVar14 = (param_5 >> 0x10);
    iVar11 = param_5;
    u_var1 = (iVar11 + 0x1f6);
    u_var6 = (iVar11 + 0x1f8);
    u_var5 = u_var1 + 0x18c;
    // u_var4 = (u_var1 >> 0x10);
    uVar7 = u_var5;
    pass1_1030_38f2(u_var1 & 0xffff | u_var6 << 0x10, param_4, param_6);
    u_var3 = 0x64 / param_3;
    u_var10 = u_var3 >> 0xf;
    iVar12 = param_4 * 0x4;
    lVar2 = (uVar7 & 0xffff | u_var6 << 0x10) + (iVar12 + u_var5);
    lVar8 = lVar2 / (u_var3 & 0xffff | u_var10 << 0x10);
    lVar9 = lVar8 * (u_var3 & 0xffff | u_var10 << 0x10);
    uStack14 = lVar2;
    // iStack12 = (lVar2 >> 0x10);
    u_var6 = lVar9;
    puVar13 = ((iStack12 - (lVar9 >> 0x10)) - (uStack14 < u_var6));
    (u_var5 + iVar12) = uStack14 - u_var6;
    (u_var5 + iVar12 + 0x2) = puVar13;
    // uStack16 = (lVar8 >> 0x10);
    uStack18 = lVar8;
    if ((uStack16 | uStack18) != 0x0) {
        pass1_1030_375a(u_var1, param_4, lVar8, param_6);
        if ((iVar11 + 0x200) != 0x8000002) {
            puVar15 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_6, puVar13, iVar12);
            // puVar13 = (puVar15 >> 0x10);
            post_win_msg_1008_a0e4(
                (puVar15 & 0xffff | ZEXT24(puVar13) << 0x10),
                0x0,
                uStack18,
                (iVar11 + 0x208),
                (iVar11 + 0x4),
                0x2,
                0x1008,
                param_6,
            );
            puVar15 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar13, iVar12);
            pass1_1010_043a(puVar15, (iVar11 + 0x4), 0xd, param_6);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_a28a(param_1: u16, param_2: u16, param_3: u32) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let i_var9: &mut Struct691;
    let uVar9: u16;
    let puVar10: u32;
    let puStack10: u32;

    puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0xe);
    // pu_var5 = (puVar10 >> 0x10);
    u_var2 = puVar10;
    pass1_1038_4d6e(param_3, puVar10, u_var2, pu_var5);
    puStack10 = CONCAT22(pu_var5, u_var2);
    // uVar9 = (param_3 >> 0x10);
    i_var9 = param_3;
    u_var4 = i_var9.field_0x1f6;
    ppcVar1 = (*puStack10 + 0x10);
    puVar6 = pu_var5;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var2, pu_var5);
    u_var3 = u_var4;
    puVar7 = puVar6;
    pass1_1030_38b8();
    if ((u_var4 & 0xffff | ZEXT24(puVar6) << 0x10) == 0x0) {
        u_var4 = 0x64;
        uVar8 = 0x0;
    } else {
        u_var4 = CONCAT22(puVar7, u_var3) / (u_var4 & 0xffff | ZEXT24(puVar6) << 0x10);
        // uVar8 = (u_var4 >> 0x10);
    }
    u_var4 = u_var4 & 0xffff | uVar8 << 0x10;
    if (puStack10 != 0x0) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(0x1030, u_var2, pu_var5, 0x1);
    }
    if (u_var4 < 0x64) {
        if (u_var4 < 0x55) {
            if (u_var4 < 0x4b) {
                if (u_var4 < 0x32) {
                    if (u_var4 < 0x19) {
                        i_var9.field_0x20a = 0x1;
                        i_var9.field_0x20c = 0xffff;
                        return;
                    }
                    i_var9.field_0x20a = 0x0;
                    i_var9.field_0x20c = 0x0;
                    return;
                }
                i_var9.field_0x20a = 0xfffb;
            } else {
                i_var9.field_0x20a = 0xfff6;
            }
        } else {
            i_var9.field_0x20a = 0xfff1;
        }
    } else {
        i_var9.field_0x20a = 0xffec;
    }
    i_var9.field_0x20c = 0x1;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_a3ae(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: i32,
    param_5: i16,
    param_6: u16,
    param_7: u8,
    param_8: i16,
) {
    let u_var1: u16;
    let u_var2: u16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u32;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let i_var9: i16;
    let u_var10: u16;
    let local_146: u16;
    let uStack324: u16;
    let uStack32: u16;
    let uStack30: u16;
    let uStack26: u32;
    let uStack22: u32;
    let uStack18: u16;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u32;
    let i_stack6: i16;
    let uStack4: u16;

    i_var9 = param_3;
    // u_var10 = (param_3 >> 0x10);
    pass1_1038_3fb0(param_3);
    uStack4 = param_4;
    i_stack6 = param_8;
    if (((i_var9 + 0x204) != 0x0)
        && (
            BVar3 = pass1_1030_25b2(CONCAT22(uStack4, param_8), 0x82),
            BVar3 != 0x0,
        ))
    {
        return;
    }
    u_var5 = (i_var9 + 0x1f6);
    uStack10 = u_var5;
    pass1_1030_38b8();
    u_var4 = u_var5;
    uStack16 = param_4;
    uStack14 = u_var5 & 0xffff | param_4 << 0x10;
    empty_1038_540a();
    puVar6 = (uStack16 | u_var4);
    uStack18 = u_var4;
    if ((((puVar6 == 0x0) && ((i_var9 + 0x200) != 0x8000002))
        && (pass1_1030_38b8(), -0x1 < puVar6))
        && (0x0 < puVar6 || (u_var4 != 0x0)))
    {
        puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar6, param_5);
        // uStack30 = (puVar8 >> 0x10);
        uStack32 = SUB42(puVar8, 0x0);
        pass1_1010_043a(puVar8, (i_var9 + 0x4), 0x11, param_6);
    }
    u_var2 = uStack16;
    u_var1 = uStack18;
    uStack26 = uStack14;
    u_var4 = uStack18 * 0xa;
    uVar7 = (uStack16 * 0x5
        + CARRY2(uStack18, uStack18) * 0x2
        + CARRY2(uStack18 * 0x2, uStack18 * 0x2)
        + CARRY2(uStack18 * 0x4, uStack18))
        * 0x2
        + CARRY2(uStack18 * 0x5, uStack18 * 0x5);
    uStack22 = CONCAT22(uVar7, u_var4);
    if ((uVar7 <= uStack14._2_2_) && (uVar7 < uStack14._2_2_ || (u_var4 < uStack14))) {
        pass1_1028_ae66(
            CONCAT22(param_6, &local_146),
            uStack14,
            CONCAT22(uVar7, u_var4),
            (i_var9 + 0x4),
            param_6,
            param_7,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_6, &local_146));
        uStack26 = uStack22;
        local_146 = 0x389a;
        uStack324 = 0x1008;
    }
    uStack26 += 0x9;
    pass1_1038_52b8(
        param_3,
        uStack26 / 0xa,
        0x1e,
        u_var1,
        u_var2,
        &ctx.PTR_LOOP_1050_1038,
        param_6,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_a4ee(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let puVar11: u32;
    let uVar12: u16;
    let iStack50: i16;
    let puStack18: u32;

    // uVar9 = (param_2 >> 0x10);
    u_var1 = (param_2 + 0x1f6);
    u_var6 = *ctx.PTR_LOOP_1050_65e2;
    puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x26);
    // puVar7 = (puVar11 >> 0x10);
    u_var5 = puVar11;
    u_var10 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(param_2, puVar11, u_var5, puVar7);
    puStack18 = CONCAT22(puVar7, u_var5);
    ppcVar2 = (*puStack18 + 0x10);
    u_var3 = u_var5;
    puVar8 = puVar7;
    (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, u_var5, puVar7);
    if ((puVar8 | u_var3) != 0x0) {
        u_var10 = 0x1030;
        pass1_1030_3548(u_var1, CONCAT22(puVar8, u_var3));
    }
    if (puStack18 != 0x0) {
        ppcVar2 = *puStack18;
        (**ppcVar2)(u_var10, u_var5, puVar7, 0x1);
    }
    u_var3 = (u_var6 % 0xc);
    // uVar12 = (param_1 >> 0x10);
    u_var5 = u_var3;
    if (u_var6 % 0xc == 0x0) {
        pass1_1030_387c(u_var1);
        pass1_1028_a61e(
            param_1, uVar12, u_var1, param_2, u_var5, u_var3, param_3, param_4,
        );
    }
    pass1_1038_3fb0(param_2);
    if (((param_2 + 0x204) != 0x0)
        && (
            BVar4 = pass1_1030_25b2(CONCAT13((u_var3 >> 0x8), CONCAT12(u_var3, u_var5)), 0x80),
            BVar4 != 0x0,
        ))
    {
        return;
    }
    // uVar9 = (u_var1 >> 0x10);
    u_var5 = u_var1 + 0x180;
    u_var6 = u_var5;
    iStack50 = 0x1;
    loop {
        if (iStack50 * 0x2 + u_var5) != 0x0 {
            pass1_1008_612e(0x1, 0x64, u_var6);
            if u_var6 <= (iStack50 * 0x2 + u_var5) {
                pass1_1028_a188(
                    param_1,
                    uVar12,
                    (iStack50 * 0x2 + u_var1 + 0x174),
                    iStack50,
                    param_2,
                    param_4,
                );
            }
        }
        iStack50 += 0x1;
        if (iStack50 < 0x6) == false {
            break;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_a61e(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: i16,
    param_7: i16,
    param_8: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let i_var3: i16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uStack16: u16;
    let uStack14: u32;

    pass1_1030_38b8();
    if ((param_6 < 0x3fff) || (param_6 < 0x4000 && (param_5 != 0xffff))) {
        pass1_1030_38f2(param_3, 0x3, param_8);
        u_var1 = param_5;
        i_var3 = param_6;
        pass1_1030_38f2(param_3, 0x4, param_8);
        uStack14 = CONCAT22(param_6 + i_var3 + CARRY2(param_5, u_var1), param_5 + u_var1);
        uStack16 = (param_3 + 0x1a8);
        if (uStack16 == 0x0) {
            uStack16 = 0x5;
        }
        u_var2 = uStack14 / uStack16;
        uStack14._2_2_ = (u_var2 >> 0x10);
        puVar4 = (uStack14._2_2_ | u_var2);
        if ((puVar4 != 0x0) && (u_var5 = (param_4 >> 0x10), (param_4 + 0x200) != 0x8000002)) {
            puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_8, puVar4, param_7);
            pass1_1010_043a(puVar6, (param_4 + 0x4), 0xc, param_8);
            pass1_1030_3534(param_3, u_var2);
        }
    }
    return;
}

pub fn pass1_1028_a6ca(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_a706(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xbb7);
    param_1.field_0x0 = 0xa856;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCPrelimAlloc_1050_50f6,
    );
    return param_1;
}

pub fn pass1_1028_a73c(param_1: u16, param_2: u16) -> u16 {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    loop {
        pu_var1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var1));
        u_var3 = param_1 | pu_var1;
        if (u_var3 == 0x0) {
            break;
        }
        pu_var2 = pu_var1;
        pass1_1038_5464(
            CONCAT22(param_1, pu_var1),
            pu_var1,
            &ctx.PTR_LOOP_1050_1038,
            param_2,
        );
        pass1_1038_56d6(CONCAT22(param_1, pu_var1), 0x0);
        pass1_1038_518c(CONCAT22(param_1, pu_var1), pu_var2, &ctx.PTR_LOOP_1050_1038);
        param_1 = u_var3;
    }
    return 0x1;
}

pub fn pass1_1028_a79c(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0xa856;
        (param_2 + 0x2) = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_a82a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_a866(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x36af);
    param_1.field_0x0 = 0xa9ae;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCProdSched_1050_5104,
    );
    return param_1;
}

pub fn pass1_1028_a89c(param_1: u16, param_2: u16) -> u16 {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    loop {
        u_var2 = param_1;
        pu_var1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var1));
        param_1 = u_var2 | pu_var1;
        if (param_1 == 0x0) {
            break;
        }
        if ((pu_var1 + 0x200) != 0x8000002) {
            pass1_1038_3fca(CONCAT22(u_var2, pu_var1), pu_var1, param_2);
        }
    }
    return 0x1;
}

pub fn pass1_1028_a8f4(param_1: u32, param_2: &mut Struct335, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = &param_2.field_0x8;
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0xa9ae;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_a982(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_a9be(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x176f);
    param_1.field_0x0 = 0xab22;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCPower_1050_5110,
    );
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_a9f4(param_1: u16, param_2: u16) -> u16 {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let BVar3: bool;
    let u_var4: u16;
    let extraout_dx: u16;
    let puStack24: u32;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x700);
    loop {
        pu_var2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var2));
        puStack24 = CONCAT22(param_1, pu_var2);
        u_var4 = param_1 | pu_var2;
        if (u_var4 == 0x0) {
            break;
        }
        BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (pu_var2 + 0xc), 0xc);
        param_1 = u_var4;
        if (BVar3 != 0x0) {
            ppcVar1 = (*puStack24 + 0x34);
            (**ppcVar1)(0x1008, pu_var2);
            param_1 = extraout_dx;
        }
    }
    return 0x1;
}

pub fn pass1_1028_aa68(param_1: u32, param_2: &mut Struct336, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = &param_2.field_0x8;
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0xab22;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_aaf6(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_ab32(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x2edf);
    param_1.field_0x0 = 0xaca6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCRchSched_1050_5118,
    );
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_ab68(param_1: u16, param_2: u16) -> u16 {
    let u_var1: u16;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let Bvar4: bool;
    let u_var5: u16;
    let extraout_dx: u16;
    let puStack24: u32;
    let local_14: [u8; 12];

    pass1_1028_dc52(
        CONCAT13((param_2 >> 0x8), CONCAT12(param_2, local_14)),
        0x1,
        0x0,
        0x700,
    );
    //LAB_1028_ab7e:
    u_var5 = param_1;
    pu_var3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2, pu_var3));
    puStack24 = CONCAT22(u_var5, pu_var3);
    param_1 = u_var5 | pu_var3;
    if (param_1 == 0x0) {
        return 0x1;
    }
    u_var1 = (pu_var3 + 0xc);
    BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x11);
    if (BVar4 == 0x0) {
        // goto
        // code_r0x1028abad;
    }
    //   TODO: goto LAB_1028_abc0;
    // code_r0x1028abad:
    BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x12);
    if (BVar4 != 0x0) {
        //LAB_1028_abc0:
        if ((pu_var3 + 0x12) == 0x5) {
            ppcVar2 = (*puStack24 + 0x30);
            (**ppcVar2)(0x1008);
            param_1 = extraout_dx;
        }
    }
    //   TODO: goto LAB_1028_ab7e;
}

pub fn pass1_1028_abec(param_1: u32, param_2: &mut Struct337, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = &param_2.field_0x8;
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0xaca6;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_ac7a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_acb6(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3e7f);
    param_1.field_0x0 = 0xae56;
    (param_1 + 0x2) = &USHORT_1050_1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCSetup_1050_5124,
    );
    return param_1;
}

pub fn pass1_1028_acec(param_1: u16, param_2: u16) -> u16 {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let local_14: u16;
    let uStack18: u16;

    pass1_1028_dc52(CONCAT22(param_2, &local_14), 0x1, 0x0, 0x400);
    loop {
        u_var3 = param_1;
        pu_var1 = &local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var1));
        param_1 = u_var3 | pu_var1;
        if (param_1 == 0x0) {
            break;
        }
        pu_var2 = pu_var1;
        vsprintf_op_1030_840a(
            ctx.s_SCSetup__calcMe_clearing_colony_0_1050_512c,
            0x1030,
            param_2,
            param_1,
        );
        if ((pu_var1 + 0x100) != 0x8000002) {
            pass1_1038_5464(
                CONCAT22(u_var3, pu_var1),
                pu_var2,
                &ctx.PTR_LOOP_1050_1038,
                param_2,
            );
            pass1_1038_56d6(CONCAT22(u_var3, pu_var1), 0x1);
        }
    }
    local_14 = 0x389a;
    uStack18 = 0x1008;
    pass1_1028_dc52(CONCAT22(param_2, &local_14), 0x1, 0x0, 0x800);
    loop {
        pu_var1 = &local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var1));
        u_var3 = param_1 | pu_var1;
        if (u_var3 == 0x0) {
            break;
        }
        pass1_1030_2690(CONCAT22(param_1, pu_var1));
        param_1 = u_var3;
    }
    return 0x1;
}

pub fn pass1_1028_ad9c(param_1: u32, param_2: &mut Struct338, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = &param_2.field_0x8;
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0xae56;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_ae2a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_ae66(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: u8,
) {
    let i_var1: &mut Struct689;
    let u_var1: u16;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x1387);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x108 = param_4;
    i_var1.field_0x10c = param_3;
    i_var1.field_0x110 = param_2;
    i_var1.field_0x114 = 0x0;
    param_1.field_0x0 = 0xb0ce;
    i_var1.field_0x2 = &USHORT_1050_1028;
    string_1000_3d3e(
        (param_1 & 0xffff0000 | &i_var1.field_0x8),
        ctx.s_SCStarve_1050_5156,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_aec0(param_1: u32, param_2: i16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    pass1_1030_375a((param_2 + 0x1f6), 0x0, (param_1 + 0x114), param_4);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_af08(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) -> u16 {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let iVar6: &mut Struct693;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let paVar8: &mut Struct67;
    let paVar9: &mut Struct67;
    let iStack12: i16;
    let uStack10: i16;

    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    // puVar4 = (puVar7 >> 0x10);
    pu_var2 = ctx.PTR_LOOP_1050_13ae + -0x1;
    if ((ctx.PTR_LOOP_1050_13ae < 0x1) || (SBORROW2(ctx.PTR_LOOP_1050_13ae, 0x1))) {
        //LAB_1028_af27:
        iStack10 = 0x1;
    } else {
        pu_var3 = ctx.PTR_LOOP_1050_13ae + -0x2;
        if (pu_var3 == 0x0 || pu_var2 < 0x1) {
            iStack12 = 0x1;
            iStack10 = 0x1;
            //       TODO: goto LAB_1028_af42;
        }
        pu_var2 = ctx.PTR_LOOP_1050_13ae + -0x4;
        if (pu_var2 != 0x0) {
            // goto
            // LAB_1028_af27;
        }
        iStack10 = 0x2;
    }
    iStack12 = 0x3;
    pu_var3 = pu_var2;
    //LAB_1028_af42:
    pass1_1008_612e(iStack10, iStack12, pu_var3);
    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    iVar6.field_0x114 = pu_var3;
    paVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_4, puVar4, param_3);
    u_var1 = iVar6.field_0x108;
    paVar9 = paVar8;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    // pu_var5 = (paVar9 >> 0x10);
    puVar4 = pu_var5;
    post_win_msg_1008_a0e4(
        paVar8,
        0x0,
        iVar6.field_0x114,
        (paVar9 + 0x208),
        iVar6.field_0x108,
        0x2,
        0x1008,
        param_4,
    );
    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, puVar4, param_3);
    pass1_1010_043a(puVar7, (paVar9 + 0x4), 0xd, param_4);
    return 0x1;
}

pub fn pass1_1028_afce(param_1: u32, param_2: &mut Struct339, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct340;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x116, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = 0xb0ce;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_b0a2(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_b0de(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u8,
) -> Struct100 {
    pass1_1028_6af2(param_1, param_2, param_3, param_4, param_5);
    param_1.field_0x0 = 0xb1f4;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_b108(param_1: u32, param_2: &mut Struct341, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct342;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x110, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = 0x6e50;
        param_2.field_0x2 = &USHORT_1050_1028;
        *puStack10 = 0xb1f4;
        param_2.field_0x2 = &USHORT_1050_1028;
    }
    return;
}

pub fn pass1_1028_b1c8(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_b204(param_1: U32Ptr) -> u16 {
    let u_var1: u16;

    struct_1030_1628(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0xc) = 0x0;
    *param_1 = 0xb33c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_b22c(param_1: U32Ptr, param_2: u16, param_3: u32, param_4: u16) -> u16 {
    let u_var1: u16;

    pass1_1030_165e(param_1, 0x6000000, param_3, param_4);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0xc) = param_2;
    *param_1 = 0xb33c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}

pub fn pass1_1028_b260(param_1: U32Ptr) {
    *param_1 = 0xb33c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    pass1_1030_16b2(param_1);
    return;
}

pub fn pass1_1028_b2c8(param_1: u32, param_2: u32, param_3: bool, param_4: u16) -> bool {
    let b_var1: bool;
    let u_var2: u16;
    let local_4: u16;

    file_1030_1730(param_1, param_2);
    if (param_3 != 0x0) {
        // u_var2 = (param_2 >> 0x10);
        b_var1 = read_file_1008_7dee(param_2, u_var2, &local_4, 0x0, param_4, 0x2, 0x1008);
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return b_var1;
        }
        u_var2 = switch_1008_72bc(param_2, u_var2, local_4);
        (param_1 + 0xc) = u_var2;
        param_3 = 0x1;
    }
    return param_3;
}

pub fn pass1_1028_b316(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b260(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn struct_1028_b354(param_1: U32Ptr) {
    let i_var1: &mut Struct180;
    let u_var1: u16;

    struct_1030_1628(param_1);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0xc = 0x0;
    i_var1.field_0xe = 0x0;
    i_var1.field_0x10 = 0x0;
    i_var1.field_0x12 = 0x0;
    i_var1.field_0x18 = 0x0;
    i_var1.field_0x1a = 0x0;
    i_var1.field_0x1c = 0x0;
    *param_1 = 0xcf6a;
    i_var1.field_0x2 = &USHORT_1050_1028;
    i_var1.field_0x16 = 0x0;
    i_var1.field_0x14 = 0x0;
    return;
}

pub fn pass1_1028_b39e(
    param_1: &mut Struct_1028_00cc,
    param_2: i16,
    param_3: u32,
    param_4: &mut Struct18,
) {
    let i_var1: &mut Struct173;
    let u_var1: u16;

    pass1_1030_165e(param_1, 0x7000000, param_3, param_4);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0xc = param_2;
    i_var1.field_0xe = 0x42;
    i_var1.field_0x10 = 0x0;
    i_var1.field_0x12 = 0x0;
    i_var1.field_0x18 = 0x0;
    i_var1.field_0x1a = 0x0;
    i_var1.field_0x1c = 0x0;
    *param_1 = 0xcf6a;
    i_var1.field_0x2 = &USHORT_1050_1028;
    pass1_1028_bf76(param_1 & 0xffff | u_var1 << 0x10, 0x0);
    i_var1.field_0x14 = 0x0;
    if ((0x4e < i_var1.field_0xc) && (i_var1.field_0xc < 0x70)) {
        i_var1.field_0xe = 0x6b;
    }
    return;
}

pub fn pass1_1028_b418(param_1: &mut Struct_1028_0138) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    *param_1 = 0xcf6a;
    (i_var2 + 0x2) = &USHORT_1050_1028;
    i_var1 = (i_var2 + 0x12);
    if (((i_var1 == 0x4) || (i_var1 == 0x5))
        || (i_var1 == 0x6 && ((i_var1 = (i_var2 + 0x18), i_var1 == 0x4 || (i_var1 == 0x5)))))
    {
        fn_ptr_1000_17ce(ctx, (i_var2 + 0x14), 0x1000);
    }
    pass1_1030_16b2(param_1);
    return;
}

pub fn pass1_1028_b46e(param_1: &mut Struct_1028_0176, param_2: u32, param_3: u16) {
    let i_var1: i16;
    let i_var2: i16;
    let extraout_dx: u16;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u16;
    let u_var6: u16;

    u_var4 = pass1_1028_b4f2(param_1);
    i_var2 = u_var4;
    u_var5 = 0x0;
    u_var6 = 0x0;
    pass1_1028_b58e(param_1);
    u_var3 = extraout_dx;
    pass1_1030_6d80(CONCAT22(extraout_dx, i_var2), CONCAT22(u_var6, u_var5));
    i_var1 = (i_var2 + 0x32);
    if (i_var1 != 0x0) {
        pass1_1030_6c4c(CONCAT22(extraout_dx, i_var2), 0x0);
        pass1_1038_387e(u_var4, 0x0, i_var1, CONCAT22(extraout_dx, i_var2), u_var3);
    }
    fn_ptr_1030_7296(CONCAT22(extraout_dx, i_var2));
    (param_1 + 0x1c) = (param_2 + 0x200);
    return;
}

pub fn pass1_1028_b4f2(param_1: u32) -> u32 {
    let u_var1: u16;
    let u_var2: u32;

    u_var2 = pass1_1028_b58e(param_1);
    // u_var1 = (u_var2 >> 0x10);
    return CONCAT22((u_var2 + 0x30), (u_var2 + 0x2e));
}

pub fn pass1_1028_b514(struct_1: &mut Struct_1028_b514, unaff_ss: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let struct_2: &mut Struct_1028_b514;
    let u_var4: u32;

    // u_var3 = (param_1 >> 0x10);
    struct_2 = struct_1;
    i_var1 = struct_2.field_0x12;
    i_var1 = struct_2.field_0x18;
    if ((i_var1 == 0x4) || (i_var1 == 0x5)) || (i_var1 == 0x6 && (i_var1 == 0x4 || (i_var1 == 0x5)))
    {
        fn_ptr_1000_17ce(ctx, &mut struct_2.field_0x14.unwrap(), 0x1000);
    }
    struct_2.field_0x14 = None;
    struct_2.field_0x12 = 0x7;
    u_var4 = pass1_1028_b58e(ctx, struct_1);
    u_var2 = u_var4;
    fn_ptr_1030_7296(u_var4);
    pass1_1030_72d0(u_var4);
    pass1_1030_730a(u_var4, u_var2 as u16, 0x1030, unaff_ss);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_b58e(ctx: &mut AppContext, param_1: &mut Struct_1028_b514) -> u32 {
    let u_var1: u32;

    u_var1 = (param_1 + 0x8);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    return u_var1;
}

pub fn pass1_1028_b5a8(param_1: u32) -> u16 {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if (param_1 + 0x12) != 0x5 {
        return 0x0;
    }
    u_var1 = (param_1 + 0x14);
    return (u_var1 + 0x94);
}

pub fn pass1_1028_b5ca(param_1: u32) -> u16 {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if (param_1 + 0x12) != 0x5 {
        return 0x0;
    }
    u_var1 = (param_1 + 0x14);
    return (u_var1 + 0x9c);
}

pub fn pass1_1028_bab6(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u32;

    u_var1 = pass1_1028_bad4(param_1, param_2, param_3);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    return;
}

pub fn pass1_1028_bad4(param_1: u32, param_2: i16, param_3: u16) -> u32 {
    pass1_1028_baf6(param_1);
    return CONCAT22((param_2 + 0xa), (param_2 + 0x8));
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_baf6(param_1: u32) {
    let u_var1: u32;

    u_var1 = pass1_1028_bb24(param_1);
    if (u_var1 == 0x0) {
        return;
    }
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    return;
}

pub fn pass1_1028_bb24(param_1: u32) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u32;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) == 0x0) {
        return 0x0;
    }
    u_var3 = pass1_1028_b58e(param_1 & 0xffff | u_var2 << 0x10);
    // u_var1 = (u_var3 >> 0x10);
    return CONCAT22((u_var3 + 0xa), (u_var3 + 0x8));
}

pub fn pass1_1028_bb56(param_1: u32, param_2: u32) {
    pass1_1030_177a(param_1, param_2);
    return;
}

pub fn pass1_1028_bb6a(param_1: u32) -> u32 {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (((i_var1 + 0x12) != 0x5) && ((i_var1 + 0x12) != 0x6)) {
        return 0x0;
    }
    return CONCAT22((i_var1 + 0x16), (i_var1 + 0x14) + 0xa4);
}

pub fn pass1_1028_bb96(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let pu_var1: u32;
    let pu_var2: u32;
    let u_var3: u32;
    let iVar6: i16;
    let iVar5: &mut Struct296;
    let i_var4: &mut Struct297;
    let puVar7: u32;
    let uVar8: u16;
    let uVar9: u16;

    // uVar8 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5.field_0x12 == 0x5) || (iVar5.field_0x12 == 0x6)) {
        u_var3 = iVar5.field_0x14;
        // uVar9 = (u_var3 >> 0x10);
        puVar7 = (u_var3 + 0xa4);
        // for (iVar6 = 0x2; iVar6 != 0x0; iVar6 += -0x1) {
        //   pu_var2 = puVar7;
        //   puVar7 = puVar7 + 0x1;
        //   pu_var1 = param_2;
        //   param_2 = param_2 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        puVar7 = param_2;
        pass1_1028_c724(param_1);
        u_var3 = iVar5.field_0x14;
        // uVar8 = (u_var3 >> 0x10);
        i_var4 = u_var3;
        if (i_var4.field_0xaa == 0x0) {
            i_var4.field_0xaa = 0x1;
        }
    }
    return;
}

pub fn pass1_1028_bbf0(param_1: u16, param_2: u16, param_3: U32Ptr) {
    *param_3 = 0x0;
    return;
}

pub fn pass1_1028_bc02(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x40);
    (**ppcVar1)();
    return;
}

pub fn pass1_1028_bc1c(param_1: u32) -> u16 {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x12) == 0x4) {
        return (i_var1 + 0xe);
    }
    if ((i_var1 + 0x12) == 0x7) {
        return (i_var1 + 0x10);
    }
    return (i_var1 + 0xc);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_bc4a(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: u16) -> u16 {
    let u_var1: u16;
    let paVar2: &mut Struct18;

    paVar2 = pass1_1028_e0bc(
        ctx.PTR_LOOP_1050_65e2,
        (param_1 + 0xc),
        param_2,
        param_3,
        param_4,
    );
    u_var1 = (paVar2 + 0x96);
    fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    return u_var1;
}

pub fn pass1_1028_bc7e(param_1: U32Ptr, param_2: u16) {
    pass1_1028_bdac(param_1, 0x4, param_2);
    return;
}

pub fn pass1_1028_bc90(
    param_1: U32Ptr,
    param_2: U32Ptr,
    param_3: u32,
    param_4: u32,
    param_5: i16,
    param_6: u16,
    param_7: u16,
) -> u16 {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let Bvar4: bool;
    let u_var5: u32;
    let u_var6: u16;
    let uVar7: u16;

    u_var6 = param_1;
    // uVar7 = (param_1 >> 0x10);
    pass1_1028_c7b6(param_7, param_6, u_var6, uVar7, param_2, param_4);
    if ((param_5 == 0x5) || (param_5 == 0x6)) {
        u_var2 = *param_1;
        ppcVar1 = (u_var2 + 0x60);
        i_var3 = (**ppcVar1)();
        if (i_var3 != 0x0) {
            ppcVar1 = (u_var2 + 0x5c);
            u_var5 = (**ppcVar1)();
            if (u_var5 != 0x0) {
                pass1_1028_c23e(
                    u_var6,
                    uVar7,
                    param_2,
                    param_3,
                    param_4,
                    u_var5,
                    (u_var5 >> 0x10),
                    param_7,
                );
                if (u_var5 != 0x0) {
                    BVar4 = pass1_1028_c314(
                        param_7,
                        u_var5,
                        (u_var5 >> 0x10),
                        u_var6,
                        uVar7,
                        param_2,
                        param_3,
                        (param_3 >> 0x10),
                        param_4,
                    );
                    if (BVar4 != 0x0) {
                        return 0x1;
                    }
                }
            }
        }
    } else {
        ctx.PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_bd38(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u32;
    let extraout_dx: u16;
    let iStack20: i16;

    u_var5 = (ctx.PTR_LOOP_1050_65e2 + 0x52);
    pass1_1030_4bbe(param_3, param_2, u_var5, (param_1 + 0xc));
    i_var3 = u_var5;
    i_var4 = i_var3;
    pass1_1028_b58e(param_1);
    u_var5 = (i_var4 + 0x2e);
    iStack20 = 0x11;
    loop {
        u_var1 = (iStack20 * 0x4 + i_var3);
        u_var2 = (iStack20 * 0x4 + i_var3 + 0x2);
        if (u_var2 | u_var1) != 0x0 {
            pass1_1038_5770(u_var5, CONCAT22(u_var2, u_var1), iStack20);
        }
        iStack20 += 0x1;
        if (iStack20 < 0x25) == false {
            break;
        }
    }
    return;
}

pub fn pass1_1028_bdac(param_1: U32Ptr, param_2: i16, param_3: u16) {
    let i_var1: i16;
    let ppcVar2: u32;
    let i_var3: &mut Struct599;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (i_var3.field_0x12 != param_2) {
        if (i_var3.field_0x12 == 0x6) {
            if (i_var3.field_0x18 == param_2) {
                i_var3.field_0x12 = i_var3.field_0x18;
                i_var3.field_0x18 = 0x0;
                return;
            }
        } else {
            if (param_2 != 0x6) {
                i_var1 = i_var3.field_0x12;
                if ((i_var1 == 0x4) || (i_var1 == 0x5)) {
                    param_3 = 0x1000;
                    fn_ptr_1000_17ce(ctx, i_var3.field_0x14, 0x1000);
                }
                i_var3.field_0x12 = param_2;
                ppcVar2 = (*param_1 + 0x3c);
                (**ppcVar2)(param_3, param_1);
                return;
            }
            i_var3.field_0x18 = i_var3.field_0x12;
            i_var3.field_0x12 = 0x6;
        }
    }
    return;
}

pub fn pass1_1028_be2a(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: i16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    u_var3 = pass1_1028_b4f2(param_1);
    if ((u_var3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            i_var4 = 0x6;
            //       TODO: goto code_r0x1028be96;
        }
        ppcVar1 = (*param_1 + 0x64);
        i_var4 = (**ppcVar1)(param_4, param_1);
        if (i_var4 == 0x0) {
            return;
        }
        pass1_1028_cb04(param_1, param_2, param_3, param_5);
        if (i_var4 == 0x0) {
            i_var4 = 0x6;
            //       TODO: goto code_r0x1028be96;
        }
        pass1_1028_c952(param_1, param_2, param_3, param_5);
    }
    i_var4 = 0x5;
    // code_r0x1028be96:
    pass1_1028_bdac(param_1, i_var4, param_4);
    return;
}

pub fn pass1_1028_be9e(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x12) == 0x4) {
        u_var6 = pass1_1028_b4f2(param_1);
        i_var3 = u_var6;
        if ((i_var3 + 0x200) == 0x8000002) {
            u_var2 = (i_var4 + 0x14);
            pi_var1 = (u_var2 + 0x94);
            *pi_var1 = *pi_var1 + -0x1;
        } else {
            pass1_1028_cb04(param_1, param_2, param_3, param_5);
            if (i_var3 == 0x0) {
                return;
            }
            u_var2 = (i_var4 + 0x14);
            pi_var1 = (u_var2 + 0x94);
            *pi_var1 = *pi_var1 + -0x1;
            pass1_1028_c952(param_1, param_2, param_3, param_5);
        }
        u_var2 = (i_var4 + 0x14);
        if ((u_var2 + 0x94) < 0x1) {
            pass1_1028_bdac(param_1, 0x5, param_4);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_bf22(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u32;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var1 = (i_var2 + 0x12);
    if (i_var1 == 0x4) {
        u_var4 = pass1_1028_e0bc(
            ctx.PTR_LOOP_1050_65e2,
            (i_var2 + 0xc),
            0x0,
            param_2,
            param_3,
        );
    } else {
        i_var1 += -0x5;
        if (i_var1 != 0x0) {
            if (i_var1 != 0x1) {
                (i_var2 + 0x14) = 0x0;
            }
            return;
        }
        pass1_1028_e100(ctx.PTR_LOOP_1050_65e2, (i_var2 + 0xc), param_2);
        u_var4 = CONCAT22(param_2, i_var1);
    }
    (i_var2 + 0x14) = u_var4;
    (i_var2 + 0x16) = (u_var4 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_bf76(param_1: u32, param_2: u16) {
    let b_var1: bool;
    let i_var2: &mut Struct174;
    let u_var2: u16;

    pass1_1008_612e(0x1, 0x3, param_2);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var2.field_0xc, 0x28);
    if (b_var1 == 0x0) {
        if (param_2 == 0x1) {
            i_var2.field_0x10 = 0x48;
            return;
        }
        if (param_2 != 0x2) {
            i_var2.field_0x10 = 0x4a;
            return;
        }
        i_var2.field_0x10 = 0x49;
        return;
    }
    if (param_2 == 0x1) {
        i_var2.field_0x10 = 0x70;
        return;
    }
    if (param_2 != 0x2) {
        i_var2.field_0x10 = 0x72;
        return;
    }
    i_var2.field_0x10 = 0x71;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c00a(param_1: u32, param_2: i32, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let extraout_dx: u16;
    let puVar4: U32Ptr;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let u_var5: u16;
    let u_var6: u16;
    let puVar7: u32;
    let uVar8: u32;
    let uVar9: u32;
    let uStack26: u32;
    let uStack22: u32;
    let puStack18: u32;

    pass1_1028_b58e(param_1);
    uVar8 = (param_3 + 0x2e);
    puVar7 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
    // puVar4 = (puVar7 >> 0x10);
    u_var2 = puVar7;
    u_var6 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(uVar8, puVar7, u_var2, puVar4);
    puStack18 = CONCAT22(puVar4, u_var2);
    ppcVar1 = (*puStack18 + 0x10);
    u_var5 = u_var2;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var2, puVar4);
    uStack22 = CONCAT22(extraout_DX_00, u_var5);
    uStack26 = 0x0;
    loop {
        if (uStack22 <= uStack26) {
            //LAB_1028_c0d6:
            if (puStack18 != 0x0) {
                ppcVar1 = *puStack18;
                (**ppcVar1)(u_var6, u_var2, puVar4, 0x1);
            }
            return;
        }
        ppcVar1 = (*puStack18 + 0x4);
        uVar8 = uStack22;
        (**ppcVar1)(u_var6, u_var2, puVar4, uStack26, (uStack26 >> 0x10));
        u_var3 = uVar8;
        u_var5 = extraout_DX_01;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
        u_var6 = 0x1030;
        uVar8 = struct_op_1030_73a8(CONCAT22(u_var5, u_var3));
        uVar9 = pass1_1028_6302(uVar8, param_4);
        // u_var5 = (uVar9 >> 0x10);
        if ((param_2._2_2_ <= u_var5) && (param_2._2_2_ < u_var5 || (param_2 <= uVar9))) {
            pass1_1028_6356(uVar8, 0x0, param_2, param_2._2_2_, param_4);
            //       TODO: goto LAB_1028_c0d6;
        }
        pass1_1028_6356(uVar8, 0x0, uVar9, u_var5, param_4);
        param_2 -= uVar9;
        uStack26 += 0x1;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c0f0(
    param_1: u32,
    param_2: i32,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let extraout_dx: u16;
    let puVar4: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let extraout_DX_01: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let extraout_DX_02: U32Ptr;
    let uVar7: u16;
    let puVar8: u32;
    let uVar9: u32;
    let uStack28: u32;
    let uStack24: u32;
    let puStack20: u32;
    let uStack6: u32;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_3);
    uVar9 = (param_3 + 0x2e);
    pass1_1028_cb04(param_1, param_4, param_5, param_6);
    // uVar7 = (uVar9 >> 0x10);
    if (((uVar9 + 0x204) == 0x0) && ((uVar9 + 0x206) == 0x0)) {
        puVar8 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
        // puVar4 = (puVar8 >> 0x10);
        u_var2 = puVar8;
        uVar7 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
        pass1_1038_4d6e(uVar9, puVar8, u_var2, puVar4);
        puStack20 = CONCAT22(puVar4, u_var2);
        ppcVar1 = (*puStack20 + 0x10);
        u_var5 = u_var2;
        (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var2, puVar4);
        uStack24 = CONCAT22(extraout_DX_00, u_var5);
        puVar6 = extraout_DX_00;
        // for (uStack28 = 0x0; uStack28 < uStack24; uStack28 += 0x1) {
        //   ppcVar1 = (*puStack20 + 0x4);
        //   uVar9 = uStack24;
        //   (**ppcVar1)(uVar7,u_var2,puVar4,uStack28,(uStack28 >> 0x10));
        //   u_var3 = uVar9;
        //   u_var5 = extraout_DX_01;
        //   pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var3,extraout_DX_01);
        //   uVar7 = 0x1030;
        //   uVar9 = struct_op_1030_73a8(CONCAT22(u_var5,u_var3));
        //   uVar9 = pass1_1028_6302(uVar9,param_6);
        //   puVar6 = (uVar9 >> 0x10);
        //   u_var5 = uVar9;
        //   if ((param_2._2_2_ <= puVar6) &&
        //      ((param_2._2_2_ < puVar6 || (param_2 <= u_var5)))) {
        //     param_2 = 0x0;
        //     break;
        //   }
        //   param_2 = CONCAT22(param_2._2_2_ + (-(param_2 < u_var5) - puVar6),
        //                      param_2 - u_var5);
        // }
        if (puStack20 != 0x0) {
            ppcVar1 = *puStack20;
            (**ppcVar1)(uVar7, u_var2, puVar4, 0x1);
            puVar6 = extraout_DX_02;
        }
        if (param_2 != 0x0) {
            pass1_1030_7d7c(
                uStack6,
                param_2,
                CONCAT22(0x1d, (param_2 >> 0x10)),
                puStack20,
                puVar6,
                param_4,
                param_5,
                param_6,
            );
        }
    }
    return;
}

pub fn pass1_1028_c1f8(
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: U32Ptr,
) {
    let pu_var1: u32;
    let local_c: u32;
    let uStack8: u16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_baf6(param_4);
    i_stack6 = param_2;
    uStack4 = param_3;
    pu_var1 = pass1_1030_5b5c(param_2, param_3);
    local_c = *pu_var1;
    uStack8 = (pu_var1 + 0x4);
    pass1_1008_3e94(CONCAT22(param_1, &local_c), param_5, param_6);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c23e(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: i32,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let u_var1: u32;
    let ppcVar2: u32;
    let pu_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let uVar7: u16;
    let uVar8: u16;
    let extraout_dx: u16;
    let puStack22: u32;
    let uStack10: u32;
    let uStack6: u32;

    pass1_1030_627e(
        param_8,
        param_6,
        param_7,
        ctx.PTR_LOOP_1050_5740,
        param_3,
        param_5,
    );
    uStack6 = CONCAT22(param_7, param_6);
    uVar7 = param_7 | param_6;
    if (uVar7 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6);
        uStack10 = CONCAT22(uVar7, param_6);
        u_var1 = (param_6 + 0x2a);
        if (u_var1 != param_4) {
            u_var6 = param_4;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
            u_var4 = u_var6;
            pu_var3 = (u_var6 & 0xffff | uVar7 << 0x10);
            uVar8 = uVar7;
            u_var5 = u_var4;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
            puStack22 = CONCAT22(uVar8, u_var5);
            if (((pu_var3 == 0x0) || ((uVar8 | u_var5) == 0x0))
                || ((u_var5 + 0x200) != (u_var4 + 0x200)))
            {
                return;
            }
            ppcVar2 = (*pu_var3 + 0x18);
            (**ppcVar2)(0x1030, u_var4, uVar7, uStack6);
            ppcVar2 = (*puStack22 + 0x8);
            (**ppcVar2)();
            pass1_1030_73ee(uStack10, param_4, extraout_dx);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c314(
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: u16,
    param_8: u16,
    param_9: u32,
) -> bool {
    let pu_var1: u32;
    let local_14: i16;
    let local_12: i16;
    let local_10: i16;
    let local_e: i16;
    let local_c: u32;
    let uStack8: u16;
    let i_stack6: i16;
    let uStack4: u16;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_9, (param_9 >> 0x10));
    i_stack6 = param_2;
    uStack4 = param_3;
    pu_var1 = pass1_1030_5b5c(param_2, param_3);
    local_c = *pu_var1;
    uStack8 = (pu_var1 + 0x4);
    pass1_1008_3e94(
        param_6,
        CONCAT22(param_1, &local_10),
        CONCAT22(param_1, &local_e),
    );
    pass1_1008_3e94(
        CONCAT22(param_1, &local_c),
        CONCAT22(param_1, &local_14),
        CONCAT22(param_1, &local_12),
    );
    if ((((0x1 < local_e) & &(0x1 < local_10)) & &(local_e < local_12 + -0x1))
        & &(local_10 < local_14 + -0x1))
    {
        return 0x1;
    }
    ctx.PTR_LOOP_1050_50ca = 0x6b8;
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c3aa(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: u32,
    param_6: u16,
) {
    let ppcVar1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let extraout_dx: u16;
    let u_var10: u16;
    let unaff_DI: i16;
    let u_var11: u16;
    let uVar12: u32;
    let puVar13: U32Ptr;
    let puVar14: u32;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u16;
    let uVar18: u16;
    let uVar19: u32;
    let uVar20: u16;
    let uStack40: u32;
    let uStack36: u32;
    let puStack32: u32;
    let puStack24: U32Ptr;
    let local_4: [u8; 2];

    uVar12 = pass1_1030_bcae(local_4, param_6);
    // puVar7 = (uVar12 >> 0x10);
    i_var2 = uVar12;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    uVar12 = (i_var2 + 0x10);
    uVar15 = SUB41(param_3, 0x0);
    uVar16 = (param_3 >> 0x8);
    // u_var11 = (param_3 >> 0x10);
    puVar8 = puVar7;
    uVar19 = param_5;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar12);
    puStack24 = local_4;
    pass1_1030_bcde(
        param_6,
        puStack24,
        param_6,
        uVar12 & 0xffff | ZEXT24(puVar8) << 0x10,
        CONCAT22(u_var11, CONCAT11(uVar16, uVar15)),
        uVar19,
    );
    if (puStack24 < 0x0) {
        ctx.PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    if (0x1e < puStack24) {
        u_var3 = 0x87;
        puVar13 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x9, param_6, puVar8, unaff_DI);
        u_var3 = pass1_1010_65d0(param_6, puVar13, u_var3);
        if (u_var3 == 0x0) {
            puVar14 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x15);
            // puVar9 = (puVar14 >> 0x10);
            u_var4 = puVar14;
            u_var11 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
            pass1_1038_4d6e(CONCAT22(puVar7, i_var2), puVar14, u_var4, puVar9);
            puStack32 = CONCAT22(puVar9, u_var4);
            ppcVar1 = (*puStack32 + 0x10);
            u_var10 = u_var4;
            uVar18 = u_var4;
            puVar8 = puVar9;
            (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var4, puVar9);
            uStack36 = CONCAT22(extraout_dx, u_var10);
            uStack40 = 0x0;
            u_var10 = extraout_dx;
            loop {
                if (uStack36 <= uStack40) {
                    if (puStack32 != 0x0) {
                        ppcVar1 = *puStack32;
                        (**ppcVar1)(
                            u_var11, u_var4, puVar9, 0x1, uVar18, puVar8, puStack32, puStack32,
                        );
                    }
                    ctx.PTR_LOOP_1050_50ca = 0x6b6;
                    ctx.PTR_LOOP_1050_50cc = puStack24 + -0x1e;
                    return;
                }
                uVar15 = param_5;
                uVar16 = (param_5 >> 0x8);
                uVar12 = uStack36;
                puVar13 = param_3;
                // uVar17 = (param_5 >> 0x10);
                pass1_1030_1d58(puStack32);
                u_var5 = uVar12;
                puVar6 = local_4;
                u_var11 = 0x1030;
                uVar20 = u_var10;
                pass1_1030_bcde(
                    param_6,
                    puVar6,
                    param_6,
                    uVar12 & 0xffff | u_var10 << 0x10,
                    puVar13,
                    CONCAT22(uVar17, CONCAT11(uVar16, uVar15)),
                );
                if ((0x0 < puVar6) && (puVar6 < 0x1f)) {
                    break;
                }
                if (puVar6 < puStack24) {
                    puStack24 = puVar6;
                }
                uStack40 += 0x1;
            }
            if (puStack32 == 0x0) {
                return;
            }
            ppcVar1 = *puStack32;
            (**ppcVar1)(
                0x1030, u_var4, puVar9, 0x1, uVar18, puVar8, puStack32, puStack32, u_var5, uVar20,
            );
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c522(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: i32,
    param_6: u16,
) {
    let i_var1: i16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u32;
    let local_4: [u8; 2];

    u_var4 = pass1_1030_bcae(local_4, param_6);
    // u_var3 = (u_var4 >> 0x10);
    i_var1 = u_var4;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var4 = (i_var1 + 0x10);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
    pu_var2 = local_4;
    pass1_1030_bcde(
        param_6,
        pu_var2,
        param_6,
        u_var4 & 0xffff | u_var3 << 0x10,
        param_3,
        param_5,
    );
    if (pu_var2 < 0x0) {
        ctx.PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if (pu_var2 < 0x1f) {
            return;
        }
        ctx.PTR_LOOP_1050_50ca = 0x6b6;
        ctx.PTR_LOOP_1050_50cc = pu_var2 + -0x1e;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c5a6(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: U32Ptr,
    param_5: i32,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) -> bool {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;
    let iStack14: i16;
    let uStack10: u32;

    pass1_1030_627e(
        param_8,
        param_6,
        param_7,
        ctx.PTR_LOOP_1050_5740,
        param_4,
        param_5,
    );
    u_var2 = param_7 | param_6;
    if (u_var2 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6, param_7);
        uStack10 = CONCAT22(u_var2, param_6);
        iStack14 = 0x7a;
        if (0x0 < (param_4 + 0x4)) {
            if (param_3 == 0x7b) {
                param_3 = 0x7e;
            } else {
                if (param_3 == 0x7c) {
                    param_3 = 0x7d;
                }
            }
            iStack14 = 0x7f;
        }
        if (uStack10 != 0x0) {
            u_var3 = struct_op_1030_73a8(uStack10);
            if ((u_var3 != 0x0)
                & &((
                    i_var1 = (u_var3 + 0xc),
                    i_var1 == iStack14 || (i_var1 == param_3),
                )))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_c64a(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i32,
    param_7: u16,
) -> bool {
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let local_e: [u8; 2];
    let local_c: i16;
    let local_a: i16;
    let local_8: u32;
    let uStack4: u16;

    local_8 = *param_2;
    uStack4 = (param_2 + 0x1);
    pass1_1008_3eb4(
        CONCAT22(param_7, &local_8),
        CONCAT22(param_7, local_e),
        CONCAT22(param_7, &local_c),
        CONCAT22(param_7, &local_a),
    );
    local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    b_var1 = pass1_1028_c5a6(
        u_var2,
        u_var3,
        0x7b,
        CONCAT22(param_7, &local_8),
        param_6,
        &local_8,
        param_3,
        param_7,
    );
    if (b_var1 == 0x0) {
        local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
        b_var1 = pass1_1028_c5a6(
            u_var2,
            u_var3,
            0x7b,
            CONCAT22(param_7, &local_8),
            param_6,
            &local_8,
            param_3,
            param_7,
        );
        if (b_var1 == 0x0) {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            b_var1 = pass1_1028_c5a6(
                u_var2,
                u_var3,
                0x7c,
                CONCAT22(param_7, &local_8),
                param_6,
                &local_8,
                param_3,
                param_7,
            );
            if (b_var1 == 0x0) {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                b_var1 = pass1_1028_c5a6(
                    u_var2,
                    u_var3,
                    0x7c,
                    CONCAT22(param_7, &local_8),
                    param_6,
                    &local_8,
                    param_3,
                    param_7,
                );
                if (b_var1 == 0x0) {
                    return b_var1;
                }
            }
        }
    }
    return 0x1;
}

pub fn pass1_1028_c724(param_1: u32) {
    let u_var1: u16;
    let u_var2: u32;
    let i_var3: &mut Struct295;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = i_var3.field_0x14;
    if ((u_var2 + 0xac) != 0x0) {
        return;
    }
    u_var2 = i_var3.field_0x14;
    u_var1 = (u_var2 + 0xa6);
    if (u_var1 == 0xd) {
        u_var2 = i_var3.field_0x14;
        (u_var2 + 0xac) = 0x1;
        //     TODO: goto LAB_1028_c770;
    }
    if (u_var1 < 0xe) {
        if (u_var1 == '\0') {
            // goto
            // LAB_1028_c770;
        }
        if (u_var1 == '\x07') {
            u_var2 = i_var3.field_0x14;
            (u_var2 + 0xac) = 0xa;
            //       TODO: goto LAB_1028_c770;
        }
    }
    u_var2 = i_var3.field_0x14;
    (u_var2 + 0xac) = 0x5;
    //LAB_1028_c770:
    u_var2 = i_var3.field_0x14;
    if ((u_var2 + 0xac) == 0x0) {
        u_var2 = i_var3.field_0x14;
        if ((u_var2 + 0xa8) != 0x0) {
            u_var2 = i_var3.field_0x14;
            (u_var2 + 0xac) = 0x1;
        }
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c7b6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: U32Ptr,
    param_6: i32,
) {
    let pu_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let bStack27: u8;
    let local_a: u32;
    let uStack6: u32;

    pu_var1 = &local_a;
    pass1_1030_64ce(
        param_1,
        pu_var1,
        param_2,
        ctx.PTR_LOOP_1050_5740,
        param_5,
        param_6,
        CONCAT22(param_1, pu_var1),
    );
    uStack6 = *pu_var1;
    u_var3 = (pu_var1 + 0x2);
    bStack27 = (uStack6 >> 0x18);
    u_var2 = bStack27;
    if (bStack27 == 0x0) {
        return;
    }
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack6);
    u_var4 = struct_op_1030_73a8(CONCAT22(u_var3, u_var2));
    // u_var3 = (u_var4 >> 0x10);
    if ((u_var3 | u_var4) != 0x0) {
        match (u_var4 + 0xc) {
            0x1 => {}
            0x2 => {}
            0x3 => {}
            0x4 => {}
            0x5 => {}
            0x6 => {}
            0x7 => {
                return;
            }
            0x8 => {
                return;
            }
            0x9 => {
                return;
            }
        }
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c89c(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: u32;
    let extraout_dx: u16;
    let u_var2: u16;
    let local_16: [u32; 0x3];
    let lStack10: i32;
    let uStack6: u32;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_dx, param_4);
    lStack10 = (param_4 + 0x8);
    pu_var1 = local_16;
    u_var2 = extraout_dx;
    pass1_1030_64ce(
        param_5,
        pu_var1,
        extraout_dx,
        ctx.PTR_LOOP_1050_5740,
        param_2,
        lStack10,
        CONCAT22(param_5, pu_var1),
    );
    *param_3 = *pu_var1;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_c8ee(param_1: u16, param_2: u16, param_3: u16, param_4: i16, param_5: U32Ptr) {
    let local_8: u16;
    let local_6: u32;

    pass1_1008_3eb4(
        param_5,
        CONCAT22(param_1, &local_8),
        CONCAT22(param_1, &local_6),
        CONCAT22(param_1, &local_6 + 0x2),
    );
    if (param_4 == 0x1) {
        local_8 += 0x1;
    } else {
        if (param_4 == 0x2) {
            local_6 = local_6 & 0xffff0000 | (local_6 - 0x1);
        } else {
            if (param_4 == 0x3) {
                local_6 = local_6 & 0xffff0000 | (local_6 + 0x1);
            } else {
                if (param_4 == 0x4) {
                    local_6 = local_6 & 0xffff | (local_6._2_2_ + 0x1) << 0x10;
                } else {
                    if (param_4 == 0x5) {
                        local_6 = local_6 & 0xffff | (local_6._2_2_ - 0x1) << 0x10;
                    }
                }
            }
        }
    }
    pass1_1008_3e76(param_5, local_8, local_6, (local_6 >> 0x10));
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_c952(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var4: u32;
    let u_var3: &mut Struct600;
    let BVar5: bool;
    let paVar6: &mut Struct600;
    let paVar7: &mut Struct600;
    let uVar8: u32;
    let uVar9: u32;
    let u_var10: u16;
    let iVar11: i16;
    let uVar12: u16;
    let uVar13: u16;
    let uStack30: u32;
    let uStack16: u16;
    let uStack14: u16;

    // uVar12 = (param_1 >> 0x10);
    iVar11 = param_1;
    u_var2 = (iVar11 + 0x14);
    u_var3 = u_var2;
    u_var10 = (iVar11 + 0x16) | u_var3;
    if (u_var10 != 0x0) {
        uVar8 = u_var2;
        pass1_1028_b58e(param_1);
        u_var4 = (uVar8 + 0x2e);
        uStack14 = u_var4;
        if ((((uVar8 + 0x30) | uStack14) != 0x0)
            && (uVar13 = (u_var4 >> 0x10), (uStack14 + 0x206) == 0x0))
        {
            BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (iVar11 + 0xc), 0x32);
            if (BVar5 == 0x0) {
                BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (iVar11 + 0xc), 0x33);
                if ((BVar5 != 0x0) && ((*ctx.PTR_LOOP_1050_65e2 % 0x5) == 0x0)) {
                    return;
                }
            } else {
                if ((*ctx.PTR_LOOP_1050_65e2 % 0xa) == 0x0) {
                    return;
                }
            }
            // uVar12 = (u_var2 >> 0x10);
            if ((uStack14 + 0x204) == 0x0) {
                // for (uStack16 = 0x0; uStack16 < 0x25; uStack16 += 0x1) {
                //   uStack30 = (&u_var3.field_0x0 + uStack16 * 0x4);
                //   paVar7 = uStack30;
                //   u_var10 = (&u_var3.field_0x2 + uStack16 * 0x4) | paVar7;
                //   if (u_var10 != 0x0) {
                //     uVar9 = uStack30;
                //     empty_1038_540a();
                //     uStack30._2_2_ = (uStack30 >> 0x10);
                //     paVar6 = u_var3;
                //     if ((uVar9 & 0xffff | u_var10 << 0x10) < uStack30) {
                //       paVar6 = (paVar7 - uVar9);
                //       param_3 = (uStack30._2_2_ - u_var10) - (paVar7 < uVar9);
                //       pass1_1038_52b8(u_var4,CONCAT22(param_3,paVar6),0x21,paVar6,param_3,
                //                       &ctx.PTR_LOOP_1050_1038,param_4);
                //       uStack30 = CONCAT22((uStack30._2_2_ - param_3) - (paVar7 < paVar6),
                //                           (paVar7 - paVar6));
                //     }
                //     if ((uStack30._2_2_ | uStack30) != 0x0) {
                //       pass1_1038_52b8(u_var4,uStack30,uStack16,paVar6,param_3,
                //                       &ctx.PTR_LOOP_1050_1038,param_4);
                //     }
                //   }
                // }
            } else {
                u_var10 = u_var3.field_0x8c;
                u_var1 = u_var3.field_0x8e;
                if ((u_var1 | u_var10) != 0x0) {
                    pass1_1038_52b8(
                        u_var4,
                        CONCAT22(u_var1, u_var10),
                        0x23,
                        param_2,
                        param_3,
                        &ctx.PTR_LOOP_1050_1038,
                        param_4,
                    );
                }
                u_var10 = u_var3.field_0x90;
                u_var1 = u_var3.field_0x92;
                if ((u_var1 | u_var10) != 0x0) {
                    pass1_1038_52b8(
                        u_var4,
                        CONCAT22(u_var1, u_var10),
                        0x24,
                        param_2,
                        param_3,
                        &ctx.PTR_LOOP_1050_1038,
                        param_4,
                    );
                    return;
                }
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_cb04(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u32;
    let lVar7: i32;
    let extraout_dx: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let bVar11: bool;
    let puVar12: U32Ptr;
    let puStack52: U32Ptr;
    let uStack38: u16;
    let puStack36: U32Ptr;
    let iStack22: i16;
    let uStack18: u16;
    let puStack16: U32Ptr;
    let uStack14: u16;

    u_var1 = (param_1 + 0x14);
    if (u_var1 != 0x0) {
        u_var5 = u_var1;
        pass1_1028_b58e(param_1);
        u_var3 = u_var5 & 0xffff | extraout_dx << 0x10;
        u_var2 = (u_var5 + 0x2e);
        puStack52 = (u_var5 + 0x30);
        uStack14 = u_var2;
        uStack18 = puStack52 | uStack14;
        if (uStack18 != 0x0) {
            // uVar9 = (u_var2 >> 0x10);
            if ((uStack14 + 0x206) != 0x0) {
                return;
            }
            uVar8 = u_var1;
            // u_var10 = (u_var1 >> 0x10);
            if ((uStack14 + 0x204) != 0x0) {
                u_var2 = (uVar8 + 0x8c);
                u_var6 = u_var2;
                empty_1038_540a();
                // puStack36 = (u_var2 >> 0x10);
                if ((puStack52 <= puStack36)
                    && ((
                        u_var4 = u_var6,
                        uStack38 = u_var2,
                        puStack52 < puStack36 || (u_var4 < uStack38),
                    )))
                {
                    pass1_1030_7d7c(
                        u_var3,
                        uStack38 - u_var4,
                        CONCAT22(0x23, puStack36 + (-(uStack38 < u_var4) - puStack52)),
                        u_var4,
                        puStack52,
                        param_2,
                        param_3,
                        param_4,
                    );
                    puVar12 =
                        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, puStack52, param_3);
                    // puStack52 = (puVar12 >> 0x10);
                    pass1_1010_043a(
                        puVar12 & 0xffff | ZEXT24(puStack52) << 0x10,
                        (uStack14 + 0x4),
                        0x12,
                        param_4,
                    );
                }
                u_var2 = (uVar8 + 0x90);
                u_var6 = u_var2;
                empty_1038_540a();
                // puStack36 = (u_var2 >> 0x10);
                if ((puStack52 <= puStack36)
                    && ((
                        u_var4 = u_var6,
                        uStack38 = u_var2,
                        puStack52 < puStack36 || (u_var4 < uStack38),
                    )))
                {
                    pass1_1030_7d7c(
                        u_var3,
                        uStack38 - u_var4,
                        CONCAT22(0x24, puStack36 + (-(uStack38 < u_var4) - puStack52)),
                        u_var4,
                        puStack52,
                        param_2,
                        param_3,
                        param_4,
                    );
                }
                return;
            }
            empty_1038_540a();
            puStack16 = puStack52;
            // for (iStack22 = 0x11; iStack22 < 0x25; iStack22 += 0x1) {
            //   u_var1 = (iStack22 * 0x4 + uVar8);
            //   u_var5 = u_var1;
            //   empty_1038_540a();
            //   u_var5 = u_var5 & 0xffff | ZEXT24(puStack52) << 0x10;
            //   puStack52 = (u_var1 >> 0x10);
            //   if (u_var5 < u_var1) {
            //     if ((((iStack22 == 0x23) || (iStack22 == 0x24)) || (puStack16 < puStack52)) ||
            //        ((u_var4 = u_var1, puStack16 <= puStack52 && (uStack18 < u_var4)))) {
            //       lVar7 = u_var1 - u_var5;
            //       u_var4 = lVar7;
            //       pass1_1030_7d7c(u_var3,u_var4,CONCAT22(iStack22,(lVar7 >> 0x10)),
            //                       u_var4,puStack52,uVar8,param_3,param_4);
            //       if (iStack22 == 0x23) {
            //         puVar12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x2b,param_4,puStack52,param_3
            //                                  );
            //         puStack52 = (puVar12 >> 0x10);
            //         pass1_1010_043a(puVar12 & 0xffff | ZEXT24(puStack52) << 0x10,
            //                         (uStack14 + 0x4),0x12,param_4);
            //       }
            //     }
            //     else {
            //       bVar11 = uStack18 < u_var4;
            //       uStack18 -= u_var4;
            //       puStack16 = puStack16 + (-bVar11 - puStack52);
            //     }
            //   }
            // }
            return;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_ccd0(param_1: u8, param_2: u16, param_3: u32, param_4: U32Ptr) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let i_var4: i16;
    let extraout_dx: U32Ptr;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let iVar7: i16;
    let extraout_DX_00: u16;
    let unaff_DI: i16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u16;
    let local_178: u16;
    let uStack374: u16;
    let iStack84: i16;
    let uStack72: u16;
    let uStack64: u16;
    let i_stack62: i16;
    let uStack60: u32;
    let puStack56: u32;
    let uStack52: u32;
    let puStack48: U32Ptr;
    let local_2c: [u8; 0xc];
    let local_20: i16;
    let local_1e: i16;
    let uStack28: u32;
    let uStack24: u32;
    let uStack20: u32;
    let iStack16: i16;
    let iStack14: i16;
    let uStack12: u16;
    let uStack10: u16;
    let local_8: u16;
    let local_6: i16;
    let local_4: i16;

    pu_var2 = &local_8;
    pass1_1008_3eb4(
        param_4,
        CONCAT22(param_2, pu_var2),
        CONCAT22(param_2, &local_6),
        CONCAT22(param_2, &local_4),
    );
    pass1_1028_b58e(param_3);
    uStack20 = CONCAT22(extraout_dx, pu_var2);
    uStack24 = (pu_var2 + 0x17);
    uStack28 = (uStack24 + 0x4);
    pu_var5 = extraout_dx;
    pass1_1028_c1f8(
        param_2,
        &local_20,
        extraout_dx,
        param_3,
        CONCAT22(param_2, &local_20),
        CONCAT22(param_2, &local_1e),
    );
    uStack10 = local_4 - 0x1;
    iStack14 = local_4 + 0x1;
    uStack12 = local_6 - 0x1;
    iStack16 = local_6 + 0x1;
    if (uStack10 < 0x0) {
        uStack10 = 0x0;
    }
    if (local_1e <= iStack14) {
        iStack14 = local_1e + -0x1;
    }
    if (uStack12 < 0x0) {
        uStack12 = 0x0;
    }
    if (local_20 <= iStack16) {
        iStack16 = local_20 + -0x1;
    }
    pass1_1008_6c90(CONCAT22(param_2, local_2c));
    pass1_1008_6cec(
        CONCAT22(param_2, local_2c),
        local_8,
        CONCAT22(iStack14, iStack16),
        local_8,
        CONCAT22(uStack10, uStack12),
    );
    puStack48 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_2, pu_var5, unaff_DI);
    // u_var6 = (puStack48 >> 0x10);
    uStack52 = (puStack48 + 0x20);
    pu_var3 = local_2c;
    pass1_1030_6522(
        ctx.PTR_LOOP_1050_5740,
        CONCAT22(param_2, pu_var3),
        uStack52,
        param_2,
    );
    puStack56 = CONCAT22(u_var6, pu_var3);
    if ((u_var6 | pu_var3) != 0x0) {
        uStack60 = 0x0;
        i_stack62 = 0x0;
        //     for (uStack64 = uStack12; uStack64 <= iStack16; uStack64 += 0x1) {
        //       for (uStack72 = uStack10; i_var4 = i_stack62, uStack72 <= iStack14;
        //           uStack72 += 0x1) {
        //         iVar7 = i_stack62 >> 0xf;
        //         ppcVar1 = (*puStack56 + 0x4);
        //         i_stack62 = i_stack62 + 0x1;
        //         (**ppcVar1)(0x1030,puStack56,(puStack56 >> 0x10),i_var4,iVar7);
        //         uStack60 = CONCAT22(extraout_DX_00,i_var4);
        //         uStack60._3_1_ = (extraout_DX_00 >> 0x8);
        //         if (uStack60._3_1_ == '\0') {
        //           iStack84 = i_var4;
        //           if (i_var4 == 0x7) {
        //             pass1_1008_3e76(param_4,local_8,uStack64,uStack72);
        //             u_var11 = uStack52;
        //             uVar12 = (uStack52 >> 0x10);
        //             uVar9 = uStack28;
        //             u_var10 = (uStack28 >> 0x10);
        //             uVar8 = 0x6;
        //           }
        //           else {
        //             if (i_var4 == 0x8) {
        //               pass1_1008_3e76(param_4,local_8,uStack64,uStack72);
        //               u_var11 = uStack52;
        //               uVar12 = (uStack52 >> 0x10);
        //               uVar9 = uStack28;
        //               u_var10 = (uStack28 >> 0x10);
        //               uVar8 = 0x7;
        //             }
        //             else {
        //               if (i_var4 != 0x9) goto LAB_1028_ce2c;
        //               pass1_1008_3e76(param_4,local_8,uStack64,uStack72);
        //               u_var11 = uStack52;
        //               uVar12 = (uStack52 >> 0x10);
        //               uVar9 = uStack28;
        //               u_var10 = (uStack28 >> 0x10);
        //               uVar8 = 0x8;
        //             }
        //           }
        //           struct_op_1028_87f0(param_2,param_1,CONCAT22(param_2,&local_178),
        //                               0x0,0x0,uVar8,param_4,
        //                               (param_4 >> 0x10),CONCAT22(u_var10,uVar9),
        //                               CONCAT22(uVar12,u_var11));
        //           fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748,CONCAT22(param_2,&local_178));
        //           local_178 = 0x389a;
        //           uStack374 = 0x1008;
        //         }
        // //LAB_1028_ce2c:
        //       }
        //     }
    }
    return;
}

pub fn pass1_1028_ced2(
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let u_var1: u16;
    let bVar2: bool;
    let bVar3: bool;
    let u_var4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;

    // u_var1 = (param_1 >> 0x10);
    bVar2 = ((param_1 + 0x1a) & 0x2) == 0x0;
    if (bVar2) {
        u_var6 = 0x0;
        uVar7 = 0x23;
        u_var5 = 0x1;
        u_var4 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        param_4 = 0x1030;
        pass1_1030_7d7c(
            u_var4,
            u_var5,
            CONCAT22(uVar7, u_var6),
            u_var4,
            (u_var4 >> 0x10),
            param_2,
            param_3,
            param_5,
        );
    }
    bVar3 = ((param_1 + 0x1a) & 0x1) == 0x0;
    if (bVar3) {
        u_var6 = 0x0;
        uVar7 = 0xe;
        u_var5 = 0x1;
        u_var4 = pass1_1028_b58e(param_1 & 0xffff | u_var1 << 0x10);
        param_4 = 0x1030;
        pass1_1030_7d7c(
            u_var4,
            u_var5,
            CONCAT22(uVar7, u_var6),
            u_var4,
            (u_var4 >> 0x10),
            param_2,
            param_3,
            param_5,
        );
    }
    if (bVar3 || bVar2) {
        pass1_1028_bdac(param_1, 0x6, param_4);
        return 0x0;
    }
    return 0x1;
}

pub fn pass1_1028_cf44(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1028_b418(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1028_cfd2(param_1: U32Ptr, param_2: u32) {
    *param_1 = param_2;
    (param_1 + 0x4) = 0x0;
    return;
}

pub fn pass1_1028_cff2(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    pu_var1 = (param_1 + 0x4);
    u_var2 = (param_1 + 0x6);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    return;
}

pub fn pass1_1028_d01a(param_1: U32Ptr) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let extraout_dx: u16;
    let puStack14: u32;

    pu_var1 = **param_1;
    puStack14 = pu_var1;
    loop {
        u_var4 = puStack14;
        fn_ptr_1028_d728(pu_var1);
        puStack14 = CONCAT22(extraout_dx, u_var4);
        if ((extraout_dx | u_var4) == 0x0) {
            break;
        }
        u_var3 = *puStack14;
        ppcVar2 = u_var3 + 0x2;
        (**ppcVar2)();
        if (puStack14 != 0x0) {
            ppcVar2 = u_var3;
            (**ppcVar2)();
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_d078(param_1: u16, param_2: u32, param_3: u32) {
    let ppcVar1: u32;
    let extraout_dx: U32Ptr;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let u_var6: u32;
    let local_16: [u8; 4];
    let puStack18: u32;
    let puStack16: U32Ptr;
    let uStack14: u32;
    let uStack10: u16;
    let uStack8: u16;
    let puStack6: u32;
    let uStack4: u16;

    // u_var4 = (param_2 >> 0x10);
    i_var3 = param_2;
    puStack6 = (i_var3 + 0x4);
    pu_var2 = (i_var3 + 0x6);
    uStack14 = CONCAT22(pu_var2, puStack6);
    puStack18 = puStack6;
    puStack16 = pu_var2;
    if ((pu_var2 | puStack6) != 0x0) {
        ppcVar1 = *puStack6;
        (**ppcVar1)();
        pu_var2 = extraout_dx;
    }
    mem_op_1000_179c(0x1c, pu_var2, 0x1000);
    uStack4 = pu_var2 | puStack6;
    puStack18 = puStack6;
    puStack16 = pu_var2;
    if (uStack4 == 0x0) {
        puStack6 = 0x0;
        uStack4 = 0x0;
    } else {
        struct_op_1008_8e9e(CONCAT22(pu_var2, puStack6), 0x6, 0x24);
    }
    (i_var3 + 0x4) = puStack6;
    (i_var3 + 0x6) = uStack4;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3);
    if ((uStack4 | puStack6) == 0x0) {
        pu_var5 = pass1_1018_dcf6(CONCAT22(param_1, local_16));
        u_var6 = pass1_1018_dd1e(
            param_1,
            local_16,
            (pu_var5 >> 0x10),
            local_16,
            param_1,
            0x0,
            0xa0000,
        );
        pass1_1008_8faa((i_var3 + 0x4), u_var6);
        return;
    }
    u_var6 = pass1_1038_565e(param_1, (uStack4 | puStack6), CONCAT22(uStack4, puStack6));
    // uStack8 = (u_var6 >> 0x10);
    uStack10 = u_var6;
    if ((uStack8 | uStack10) != 0x0) {
        pass1_1028_d172(param_1, param_2, u_var6 & 0xffff | uStack8 << 0x10);
    }
    return;
}

pub fn pass1_1028_d172(param_1: u16, param_2: u32, param_3: u32) {
    let u_var1: u16;
    let lVar2: i32;
    let u_var3: u32;
    let local_e: [u8; 8];
    let local_6: [u8; 4];

    pass1_1018_dcf6(CONCAT22(param_1, local_6));
    pass1_1008_5784(CONCAT22(param_1, local_e), param_3);
    loop {
        lVar2 = pass1_1008_5b12(local_e, param_1);
        // u_var1 = (lVar2 >> 0x10);
        if (lVar2 == 0x0) {
            break;
        }
        u_var3 = pass1_1018_dd1e(
            param_1,
            local_6,
            (u_var1 | lVar2),
            local_6,
            param_1,
            0x0,
            (lVar2 + 0x4) << 0x10,
        );
        pass1_1008_8faa((param_2 + 0x4), u_var3);
    }
    return;
}

pub fn pass1_1028_d282(param_1: U32Ptr) {
    let u_var1: u16;
    let u_var2: u16;
    let paStack6: &mut Struct18;

    u_var1 = *param_1;
    u_var2 = (param_1 + 0x2);
    paStack6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1028_d658(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack6, 0x1000);
    }
    return;
}

pub fn pass1_1028_d52c(param_1: U32Ptr, param_2: u32, param_3: U32Ptr) -> bool {
    let ppcVar1: u32;
    let i_var2: i16;
    let BVar3: bool;

    ppcVar1 = (*param_3 + 0x8);
    i_var2 = (**ppcVar1)();
    if (i_var2 != 0x0) {
        BVar3 = pass1_1028_d776(*param_1, param_2, param_3);
        if (BVar3 != 0x0) {
            return 0x1;
        }
    }
    return 0x0;
}

pub fn pass1_1028_d658(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct446;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = i_var4.field_0x4;
    u_var2 = i_var4.field_0x6;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = i_var4.field_0x8;
    u_var2 = i_var4.field_0xa;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    return;
}

pub fn pass1_1028_d69e(param_1: u32) -> u16 {
    let u_var1: u32;

    u_var1 = (param_1 + 0x4);
    return (u_var1 + 0x8);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_d6b2(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u32;
    let ppc_var3: u32;
    let puVar4: u32;
    let u_var5: u16;
    let extraout_dx: u16;
    let u_var6: u16;
    let uVar7: u32;

    u_var2 = *ctx.PTR_LOOP_1050_65e2;
    loop {
        // u_var6 = (param_1 >> 0x10);
        uVar7 = pass1_1020_c860((param_1 + 0x8));
        // u_var5 = (uVar7 >> 0x10);
        if (((u_var5 | uVar7) == 0x0)
            || (
                pu_var1 = (uVar7 + 0xc),
                u_var2 <= *pu_var1 && *pu_var1 != u_var2,
            ))
        {
            break;
        }
        ppc_var3 = ((param_1 + 0x8) + 0x10);
        uVar7 = u_var2;
        (**ppc_var3)();
        puVar4 = (uVar7 & 0xffff | extraout_dx << 0x10);
        fn_ptr_1028_d742(param_1, (uVar7 & 0xffff | extraout_dx << 0x10));
        if (puVar4 != 0x0) {
            ppc_var3 = *puVar4;
            (**ppc_var3)(0x1020, uVar7, extraout_dx, 0x1);
        }
    }
    return;
}

pub fn pass1_1028_d776(param_1: u32, param_2: u32, param_3: U32Ptr) -> bool {
    let ppcVar1: u32;
    let u_var2: u32;

    ppcVar1 = (*param_3 + 0xc);
    u_var2 = (**ppcVar1)();
    pass1_1020_c872((param_1 + 0x8), param_2, u_var2);
    return 0x1;
}

pub fn pass1_1028_d7a0(param_1: u16, param_2: u16, param_3: u32, param_4: u16) -> bool {
    let b_var1: bool;

    b_var1 = write_to_file_1008_7cac(param_3, param_4);
    if (b_var1 != 0x0) {
        b_var1 = 0x1;
    }
    return b_var1;
}

pub fn pass1_1028_d7de(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1008_57c4(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_d81c(param_1: U32Ptr, param_2: u32, param_3: U32Ptr, param_4: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let iVar6: &mut Struct136;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x0;
    iVar6.field_0x4 = param_2;
    &iVar6.field_0x52 = 0x0;
    ctx.PTR_LOOP_1050_65e2 = param_1;
    iVar6.field_0x32 = 0xec36;
    iVar6.field_0x34 = &USHORT_1050_1028;
    iVar6.field_0x36 = 0xecac;
    iVar6.field_0x38 = &USHORT_1050_1028;
    iVar6.field_0x3a = 0xed2c;
    iVar6.field_0x3c = &USHORT_1050_1028;
    iVar6.field_0x3e = 0xedc4;
    iVar6.field_0x40 = &USHORT_1050_1028;
    iVar6.field_0x42 = 0xee54;
    iVar6.field_0x44 = &USHORT_1050_1028;
    iVar6.field_0x46 = 0xef00;
    iVar6.field_0x48 = &USHORT_1050_1028;
    iVar6.field_0x4a = 0x10b0;
    iVar6.field_0x4c = 0x1030;
    iVar6.field_0x4e = 0x1120;
    iVar6.field_0x50 = 0x1030;
    mem_op_1000_179c(0x8, param_3, 0x1000);
    u_var4 = param_2;
    pu_var2 = (param_3 | u_var4);
    if (pu_var2 != 0x0) {
        pass1_1030_615a((param_2 & 0xffff | ZEXT24(param_3) << 0x10), pu_var2);
    }
    mem_op_1000_179c(0x56c, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | u_var4);
    if (pu_var3 == 0x0) {
        u_var4 = 0x0;
        pu_var3 = 0x0;
    } else {
        struct_1030_44be(CONCAT22(pu_var2, u_var4), pu_var3);
    }
    iVar6.field_0x52 = u_var4;
    iVar6.field_0x54 = pu_var3;
    mem_op_1000_179c(0x4, pu_var3, 0x1000);
    pu_var2 = (pu_var3 | u_var4);
    if (pu_var2 != 0x0) {
        struct_1008_bde0(CONCAT22(pu_var3, u_var4), pu_var2);
    }
    pu_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &iVar6.field_0xa), 0x0, 0x24);
    mem_op_1000_179c(0x1c, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | pu_var1);
    if (pu_var3 == 0x0) {
        &iVar6.field_0xe = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var2, pu_var1), 0x5, 0x15, param_4);
        iVar6.field_0xe = pu_var1;
        iVar6.field_0x10 = pu_var3;
    }
    mem_op_1000_179c(0x1c, pu_var3, 0x1000);
    pu_var2 = (pu_var3 | pu_var1);
    if (pu_var2 == 0x0) {
        pu_var1 = 0x0;
        pu_var2 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var3, pu_var1), 0x5, 0xa, param_4);
    }
    iVar6.field_0x12 = pu_var1;
    iVar6.field_0x14 = pu_var2;
    mem_op_1000_179c(0x1c, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | pu_var1);
    if (pu_var3 == 0x0) {
        pu_var1 = 0x0;
        pu_var3 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var2, pu_var1), 0x5, 0x19, param_4);
    }
    iVar6.field_0x16 = pu_var1;
    iVar6.field_0x18 = pu_var3;
    mem_op_1000_179c(0x1c, pu_var3, 0x1000);
    pu_var2 = (pu_var3 | pu_var1);
    if (pu_var2 == 0x0) {
        pu_var1 = 0x0;
        pu_var2 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var3, pu_var1), 0x5, 0xa, param_4);
    }
    iVar6.field_0x1a = pu_var1;
    iVar6.field_0x1c = pu_var2;
    mem_op_1000_179c(0x1c, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | pu_var1);
    if (pu_var3 == 0x0) {
        pu_var1 = 0x0;
        pu_var3 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var2, pu_var1), 0x64, 0x1f4, param_4);
    }
    iVar6.field_0x1e = pu_var1;
    iVar6.field_0x20 = pu_var3;
    mem_op_1000_179c(0x1c, pu_var3, 0x1000);
    pu_var2 = (pu_var3 | pu_var1);
    if (pu_var2 == 0x0) {
        pu_var1 = 0x0;
        pu_var2 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var3, pu_var1), 0x19, 0x64, param_4);
    }
    iVar6.field_0x22 = pu_var1;
    iVar6.field_0x24 = pu_var2;
    mem_op_1000_179c(0x1c, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | pu_var1);
    if (pu_var3 == 0x0) {
        pu_var1 = 0x0;
        pu_var3 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var2, pu_var1), 0x64, 0x1f4, param_4);
    }
    iVar6.field_0x26 = pu_var1;
    iVar6.field_0x28 = pu_var3;
    mem_op_1000_179c(0x1c, pu_var3, 0x1000);
    u_var4 = pu_var3 | pu_var1;
    if (u_var4 == 0x0) {
        pu_var1 = 0x0;
        u_var4 = 0x0;
    } else {
        struct_1030_11aa(CONCAT22(pu_var3, pu_var1), 0x5, 0x2, param_4);
    }
    iVar6.field_0x2a = pu_var1;
    iVar6.field_0x2c = u_var4;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_daba(param_1: u32, param_2: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: u32;
    let ppc_var4: u32;
    let paVar5: &mut Struct18;
    let iVar5: &mut Struct447;
    let u_var6: u16;
    let paStack14: &mut Struct18;

    paVar5 = ctx.PTR_LOOP_1050_5740;
    if (ctx.PTR_LOOP_1050_5740 != 0x0) {
        pass1_1030_61b0(ctx.PTR_LOOP_1050_5740);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, paVar5, 0x1000);
    }
    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    u_var1 = iVar5.field_0x52;
    u_var2 = iVar5.field_0x54;
    paStack14 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1030_4538(CONCAT22(u_var2, u_var1));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, paStack14, 0x1000);
    }
    if (ctx.PTR_LOOP_1050_5166 != 0x0) {
        ppc_var4 = *ctx.PTR_LOOP_1050_5166;
        (**ppc_var4)(param_2, ctx.PTR_LOOP_1050_5166);
    }
    paVar5 = ctx.PTR_LOOP_1050_06e0;
    ctx.PTR_LOOP_1050_65e2 = 0x0;
    if (ctx.PTR_LOOP_1050_06e0 != 0x0) {
        pass1_1008_c626(ctx.PTR_LOOP_1050_06e0);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, paVar5, 0x1000);
    }
    pu_var3 = iVar5.field_0xe;
    u_var1 = iVar5.field_0x10;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x12;
    u_var1 = iVar5.field_0x14;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x16;
    u_var1 = iVar5.field_0x18;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x1a;
    u_var1 = iVar5.field_0x1c;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x1e;
    u_var1 = iVar5.field_0x20;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x22;
    u_var1 = iVar5.field_0x24;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x26;
    u_var1 = iVar5.field_0x28;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    pu_var3 = iVar5.field_0x2a;
    u_var1 = iVar5.field_0x2c;
    if ((u_var1 | pu_var3) != 0x0) {
        ppc_var4 = *pu_var3;
        (**ppc_var4)(param_2, pu_var3, u_var1, 0x1);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_dc52(param_1: &mut Struct92, param_2: i16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let i_var2: &mut Struct92;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    param_1 = 0x389a;
    i_var2.field_0x2 = 0x1008;
    i_var2.field_0x4 = (ctx.PTR_LOOP_1050_65e2 + (param_4 >> 0x8) * 0x4 + 0xa);
    i_var2.field_0x8 = 0x1;
    i_var2.field_0x10 = param_2;
    param_1 = 0x11a6;
    i_var2.field_0x2 = 0x1030;
    u_var1 = i_var2.field_0x4;
    i_var2.field_0xc = (u_var1 + 0xa);
    if (param_2 == 0x0) {
        i_var2.field_0x8 = i_var2.field_0xc;
    } else {
        i_var2.field_0x8 = 0x1;
    }
    return;
}

pub fn pass1_1028_e0a0(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u16, param_5: u8) {
    let u_var1: u32;

    u_var1 = (param_1 + 0x52);
    pass1_1030_4782(
        param_4,
        param_5,
        param_3,
        u_var1,
        (u_var1 >> 0x10),
        0x1,
        param_2,
        (param_2 >> 0x10),
    );
    return;
}

pub fn pass1_1028_e0bc(
    param_1: u32,
    param_2: i16,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: u16,
) -> u32 {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: U32Ptr;
    let puVar6: u32;

    mem_op_1000_179c(0x98, param_4, 0x1000);
    pu_var3 = param_3;
    pu_var5 = param_4;
    pass1_1030_4bbe(param_5, param_4, (param_1 + 0x52), param_2);
    puVar6 = param_3;
    // for (i_var4 = 0x26; i_var4 != 0x0; i_var4 += -0x1) {
    //   pu_var2 = puVar6;
    //   puVar6 = puVar6 + 0x1;
    //   pu_var1 = pu_var3;
    //   pu_var3 = pu_var3 + 0x1;
    //   *pu_var2 = *pu_var1;
    // }
    return CONCAT22(param_4, param_3);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_e100(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let u_var4: &mut Struct311;
    let i_var4: i16;
    let u_var5: u16;
    let puVar6: u32;
    let puVar7: u32;
    let uVar8: u16;
    let unaff_SS: u16;
    let uStack10: u32;
    let uStack6: u32;
    let u_var3: u32;

    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_3, 0x1000);
        ctx.PTR_LOOP_1050_5f2e = param_3;
    } else {
    }
    u_var4 = fn_ptr_op_1000_1708(
        0xae,
        0x0,
        0x1,
        ctx.PTR_LOOP_1050_5f2c,
        ctx.PTR_LOOP_1050_5f2e,
        0x1000,
    );
    u_var3 = ZEXT24(u_var4);
    uStack10 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var4);
    u_var5 = ctx.PTR_LOOP_1050_5f2e | u_var4;
    if (u_var5 == 0x0) {
        uStack6 = 0x0;
    } else {
        u_var4.field_0xa4 = 0x0;
        u_var4.field_0xa8 = 0x0;
        u_var4.field_0xac = 0x0;
        uStack6 = uStack10;
        u_var3 = uStack10;
    }
    puVar6 = u_var3;
    pass1_1030_4c06((param_1 + 0x52), param_2, u_var5, unaff_SS);
    // uVar8 = (uStack6 >> 0x10);
    puVar7 = uStack6;
    // for (i_var4 = 0x2b; i_var4 != 0x0; i_var4 += -0x1) {
    //   pu_var2 = puVar7;
    //   puVar7 = puVar7 + 0x1;
    //   pu_var1 = puVar6;
    //   puVar6 = puVar6 + 0x1;
    //   *pu_var2 = *pu_var1;
    // }
    puVar7 = puVar6;
    return;
}

pub fn pass1_1028_e198(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: u32,
    param_5: u16,
    param_6: u16,
) {
    pass1_1028_e1ec(param_1, param_4);
    pass1_1030_5b1c(CONCAT22(param_6, param_5), param_2, param_3);
    return;
}

pub fn pass1_1028_e1ec(param_1: u32, param_2: U32Ptr) {
    if param_3._1_1_ == '\0' {
        return;
    }
    if param_3._1_1_ == -0x1 {
        return;
    }
    bad_1030_1312();
    return;
}

pub fn pass1_1028_e28a(param_1: U32Ptr, param_2: i16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: u32;
    let u_var3: u16;
    let puVar4: U32Ptr;

    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_3, param_1, param_2);
    // u_var3 = (puVar4 >> 0x10);
    pu_var2 = (puVar4 + 0xa);
    ppcVar1 = (*pu_var2 + 0x4);
    (**ppcVar1)(0x1010, pu_var2, u_var3, 0x5);
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_e2ac(param_1: u32, param_2: u16) {
    let uStack6: u32;

    uStack6 = (param_1 + (param_2 >> 0x8) * 0x4 + 0x2e);
    (*uStack6)();
    return;
}

pub fn pass1_1028_e2e0(param_1: u32, param_2: u16, param_3: u8) -> u32 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;
    let auStack10: [u16; 0x3];
    let uStack4: u16;

    uStack4 = param_3;
    if (uStack4 == 0xff) {
        u_var3 = pass1_1028_ebee(param_1, 0x0, param_2);
        return u_var3;
    }
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1 + 0x2e;
    auStack10[0] = (i_var1 + uStack4 * 0x4 + 0x2);
    u_var3 = (i_var1 + uStack4 * 0x4)();
    return u_var3;
}

pub fn pass1_1028_e332(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    if ((param_3._1_1_ != 0x0) && (param_3._1_1_ < 0xa)) {
        pass1_1030_13f6(
            (param_1 + 0xa + param_3._1_1_ * 0x4),
            CONCAT22(param_3, param_2) & 0xffffff,
            param_2,
            param_3 & 0xff,
            param_4,
        );
    }
    return;
}

pub fn pass1_1028_e372(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let ppc_var4: u32;
    let u_var5: u32;
    let u_var6: u32;
    let uVar7: u16;
    let uVar8: u16;
    let uStack32: u32;
    let uStack16: u32;
    let uStack10: u16;

    if (param_3 >> 0x8 != 0xff) {
        u_var1 = (param_1 + 0xa + (param_3 >> 0x8) * 0x4);
        u_var2 = (u_var1 + 0xa);
        uVar7 = param_3 & 0xff;
        uStack16 = CONCAT22(param_3, param_2) & 0xffffff;
        pass1_1028_e1ec(param_1, param_2);
        u_var5 = (param_2 + 0x8);
        pass1_1028_e1ec(param_1, u_var5);
        // for (uStack32 = 0x1; uStack10 = (u_var2 >> 0x10), uStack32 < u_var2;
        //     uStack32 += 0x1) {
        //   if (uStack32 != uStack16) {
        //     u_var6 = uStack16;
        //     bad_1030_1312();
        //     uVar8 = uStack10 | u_var6;
        //     if (uVar8 != 0x0) {
        //       u_var3 = (u_var6 + 0x4);
        //       pass1_1030_13f6(u_var1,uStack32,u_var3,uVar8,param_4);
        //       ppc_var4 = ((u_var5 & 0xffff | uVar7 << 0x10)
        //                          + 0x18);
        //       (**ppc_var4)(0x1030,(u_var5 & 0xffff),uVar7,u_var3);
        //     }
        //   }
        // }
    }
    return;
}

pub fn pass1_1028_e44a(param_1: u32, param_2: i32, param_3: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let uVar7: u16;
    let uVar8: u16;
    let uStack18: u32;
    let uStack12: u16;

    pass1_1028_e372(param_1, param_2, (param_2 >> 0x10), param_3);
    // uVar8 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x26);
    u_var2 = (param_1 + 0x1e);
    u_var3 = (u_var2 + 0xa);
    // for (uStack18 = 0x1; uStack12 = (u_var3 >> 0x10), uStack18 < u_var3; uStack18 += 0x1
    //     ) {
    //   u_var6 = u_var3;
    //   bad_1030_1312();
    //   u_var5 = u_var6;
    //   if (((uStack12 | u_var5) != 0x0) && ((u_var5 + 0x8) != param_2)) {
    //     uVar8 = (u_var5 + 0x16);
    //     u_var5 = (u_var5 + 0x18);
    //     uVar7 = u_var5 & 0xff;
    //     u_var4 = pass1_1030_13f6(u_var1,CONCAT22(u_var5,uVar8) & 0xffffff,uVar8,uVar7,param_3);
    //     pass1_1030_13f6(u_var2,uStack18,u_var4,uVar7,param_3);
    //   }
    // }
    return;
}

pub fn pass1_1028_e4ec(param_1: u32) {
    let pu_var1: u32;
    let plVar2: &i32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u32;
    let in_DX: u16;
    let iVar6: i16;
    let uVar7: u16;

    u_var5 = 0x0;
    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (iVar6 + 0x10) == 0x0 {
        loop {
            if (iVar6 + 0x8) == 0x0 {
                return;
            }
            plVar2 = (iVar6 + 0x8);
            *plVar2 = *plVar2 + -0x1;
            bad_1030_1312();
            in_DX |= u_var5;
            if (in_DX == 0x0) == false {
                break;
            }
        }
    } else {
        loop {
            u_var3 = (iVar6 + 0xc);
            pu_var1 = (iVar6 + 0x8);
            if u_var3 <= *pu_var1 && *pu_var1 != u_var3 {
                return;
            }
            u_var4 = (iVar6 + 0x8);
            plVar2 = (iVar6 + 0x8);
            *plVar2 = *plVar2 + 0x1;
            bad_1030_1312();
            in_DX |= u_var4;
            if (in_DX == 0x0) == false {
                break;
            }
        }
    }
    return;
}

pub fn pass1_1028_e628(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: i16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
    param_9: u16,
    param_10: u8,
) {
    let mut pcVar1: String;
    let piVar2: U32Ptr;
    let cVar3: u8;
    let u_var4: u32;
    let u_var5: u32;
    let lVar6: i32;
    let ppcVar7: u32;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let BVar11: bool;
    let uVar12: u16;
    let uVar13: u32;
    let iVar14: i16;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let extraout_DX_01: U32Ptr;
    let uVar15: u16;
    let uVar16: u16;
    let puVar17: U32Ptr;
    let uVar18: &mut Struct348;
    let paVar18: &mut Struct349;
    let uVar19: u16;
    let uVar20: &mut Struct349;
    let uVar21: u16;
    let uVar22: u16;
    let bVar23: bool;
    let bVar24: bool;
    let pu_var25: U32Ptr;
    let paVar26: &mut Struct99;
    let pu_var27: u32;
    let uVar28: u8;
    let uVar29: u8;
    let uVar30: u8;
    let uVar31: u8;
    let uVar32: u8;
    let uVar33: u8;
    let uVar34: u8;
    let uVar35: u8;
    let uVar36: u8;
    let uVar37: u16;
    let uVar38: u8;
    let uVar39: u8;
    let iVar40: i16;
    let in_stack_0000ffca: u16;
    let in_stack_0000ffcc: u16;
    let uStack50: u16;
    let local_30: u32;
    let uStack44: u16;
    let uStack42: u16;
    let uStack40: u16;
    let uStack38: u16;
    let puStack36: u32;
    let puStack32: U32Ptr;
    let puStack30: U32Ptr;
    let uStack28: u16;
    let uStack26: u16;
    u16 * *ppuStack24;
    let local_16: u16;
    let local_14: U32Ptr;
    let local_12: i16;
    let local_10: U32Ptr;
    let puStack14: U32Ptr;
    let pcStack12: u32;
    let puStack10: U32Ptr;
    let local_6: u32;

    uVar21 = SUB42(ctx.data_seg, 0x0);
    uVar19 = param_6;
    uVar22 = param_7;
    BVar11 = read_file_1008_7dee(param_2, param_3, &local_6, 0x0, param_9, 0x4, 0x1008);
    if (BVar11 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    puStack10 = 0x0;
    if (((param_4 == 0x0) && ((param_5 - 0x100) == '\0'))
        && (
            puVar17 = (param_5 - 0x100 >> 0x7),
            puVar17 < (&ctx.PTR_LOOP_1050_000e + 0x1),
        ))
    {
        // uVar37 = (param_1 >> 0x10);
        uVar20 = param_1;
        uVar34 = (param_9 >> 0x8);
        match (puVar17) {
            0x0 => {
                pass1_1030_145a(uVar20.field_0xe, local_6);
                uStack28 = 0x0;
                uStack26 = 0x0;
                while (CONCAT22(uStack26, uStack28) < local_6) {
                    pu_var27 = local_6;
                    mem_op_1000_179c(0x14, puVar17, 0x1000);
                    puStack32 = pu_var27;
                    puStack30 = puVar17;
                    if ((puVar17 | puStack32) == 0x0) {
                        puVar9 = 0x0;
                        local_16 = 0x0;
                    } else {
                        pu_var25 = pass1_1030_5d0a((pu_var27 & 0xffff | ZEXT24(puVar17) << 0x10));
                        // local_16 = (pu_var25 >> 0x10);
                        puVar9 = pu_var25;
                    }
                    ppcVar7 = (CONCAT22(local_16, puVar9) + 0x10);
                    ppuStack24 = puVar9;
                    (**ppcVar7)();
                    if (puVar9 == 0x0) {
                        return;
                    }
                    u_var5 = (ppuStack24 + 0x4);
                    uVar16 = (ppuStack24 + 0x6);
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    puVar17 = (uVar16 & 0xff);
                    pass1_1030_14b4(
                        uVar20.field_0xe,
                        ppuStack24,
                        local_16,
                        u_var5 & 0xffff | (uVar16 & 0xff) << 0x10,
                        param_9,
                    );
                    lVar6 = CONCAT22(uStack26, uStack28) + 0x1;
                    uStack28 = lVar6;
                    // uStack26 = (lVar6 >> 0x10);
                }
            }

            0x1 => {
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            0x2 => {
                pass1_1030_145a(uVar20.field_0x12, local_6);
                uStack40 = 0x0;
                uStack38 = 0x0;
                while (CONCAT22(uStack38, uStack40) < local_6) {
                    pu_var27 = local_6;
                    mem_op_1000_179c(0x1c, puVar17, 0x1000);
                    puStack32 = pu_var27;
                    uVar16 = puVar17 | puStack32;
                    puStack30 = puVar17;
                    if (uVar16 == 0x0) {
                        uVar12 = 0x0;
                        uVar16 = 0x0;
                    } else {
                        uVar12 = puStack32;
                        pass1_1030_2958((pu_var27 & 0xffff | ZEXT24(puVar17) << 0x10));
                    }
                    puStack36 = CONCAT22(uVar16, uVar12);
                    ppcVar7 = (*puStack36 + 0x10);
                    (**ppcVar7)();
                    if (uVar12 == 0x0) {
                        return;
                    }
                    // uVar19 = (puStack36 >> 0x10);
                    uVar18 = puStack36;
                    u_var5 = &uVar18.field_0x4;
                    uVar16 = uVar18.field_0x6;
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    puVar17 = (uVar16 & 0xff);
                    pass1_1030_14b4(
                        uVar20.field_0x12,
                        uVar18,
                        uVar19,
                        u_var5 & 0xffff | (uVar16 & 0xff) << 0x10,
                        param_9,
                    );
                    lVar6 = CONCAT22(uStack38, uStack40) + 0x1;
                    uStack40 = lVar6;
                    // uStack38 = (lVar6 >> 0x10);
                }
            }
            0x3 => {
                uVar19 = &uVar20.field_0x114;
                pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x500);
                local_16 = uVar19;
                local_14 = puVar17;
                pass1_1030_61fe(
                    ctx.PTR_LOOP_1050_5740,
                    CONCAT22(puVar17, uVar19),
                    param_1 & 0xffff0000 | &uVar20.field_0x114,
                    &uVar20.field_0x108,
                    uVar19,
                    puVar17,
                    param_9,
                );
                if ((uVar20.field_0x11a == 0xa) || (uVar20.field_0x11a == 0x37)) {
                    if (uVar20.field_0x11a == 0x37) {
                        puVar17 = (&uVar20.field_0x11e + 0x2);
                        u_var5 = uVar20.field_0x10c;
                        uStack42 = u_var5;
                        // uStack40 = (u_var5 >> 0x10);
                        (uVar20.field_0x11e + 0x20) = u_var5;
                    }
                    uVar19 = &uVar20.field_0x114;
                    pass1_1028_e2ac(ctx.PTR_LOOP_1050_65e2, 0x400);
                    &uVar20.field_0x10c = uVar19;
                    (&uVar20.field_0x10c + 0x2) = puVar17;
                    pass1_1018_0196(
                        local_6,
                        CONCAT22(puVar17, &uVar20.field_0x10c),
                        &uVar20.field_0x108,
                        uVar19,
                        puVar17,
                        param_9,
                    );
                    if (uVar20.field_0x11a == 0xa) {
                        pass1_1010_ed22(local_6, uVar20.field_0x10c, param_9);
                    }
                }
                u_var5 = uVar20.field_0x10c;
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
                &uVar20.field_0x110 = uVar19;
                (&uVar20.field_0x110 + 0x2) = puVar17;
                uStack26 = puVar17 | &uVar20.field_0x110;
                if (uStack26 != 0x0) {
                    ppcVar7 = (*uVar20.field_0x110 + 0x8);
                    (**ppcVar7)();
                    puVar17 = extraout_dx;
                }
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, local_16);
                ppuStack24 = puVar17;
                pass1_1030_73ee(CONCAT22(puVar17, uStack26), uVar20.field_0x10c, puVar17);
                BVar11 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uVar20.field_0x11a, 0x31);
                puStack32 = puVar17;
                if ((BVar11 == 0x0) && (uVar20.field_0x122 == 0x0)) {
                    uVar21 = ((uStack26 + 0xc) >> 0x10);
                    if ((uStack26 + 0x10) < 0x1) {
                        u_var10 = 0x5;
                    } else {
                        u_var10 = 0x6;
                    }
                    (uStack26 + 0x14) = u_var10;
                    puStack32 = ppuStack24;
                }
                uVar13 = (uStack26 + 0x16);
                puStack30 = uVar13;
                // uStack28 = (uVar13 >> 0x10);
                pass1_1028_e1ec(&ctx.PTR_LOOP_1050_65e2, puStack30);
                puStack36 = CONCAT22(uVar13, puStack36._0_2_);
                if (CONCAT22(uStack28, puStack30) != 0x0) {
                    struct_1030_e4fa(
                        CONCAT22(param_9, &stack0xfeb4),
                        CONCAT22(uStack28, puStack30),
                        param_9,
                        param_10,
                    );
                    fn_ptr_1030_835a(&ctx.PTR_LOOP_1050_5748, CONCAT22(param_9, &stack0xfeb4));
                }
                ppcVar7 = (*uVar20.field_0x11e + 0x4);
                (**ppcVar7)();
                pu_var27 = uVar20.field_0x11e;
                pass1_1030_7e5a(
                    CONCAT13((ppuStack24 >> 0x8), CONCAT12(ppuStack24, uStack26)),
                    (pu_var27 + 0x4),
                    extraout_DX_00,
                );
                return;
            }
            0x4 => {
                pass1_1030_145a(uVar20.field_0x16, local_6);
                uStack40 = 0x0;
                uStack38 = 0x0;
                while (CONCAT22(uStack38, uStack40) < local_6) {
                    pu_var27 = local_6;
                    mem_op_1000_179c(0x1e, puVar17, 0x1000);
                    puStack32 = pu_var27;
                    puStack30 = puVar17;
                    if ((puVar17 | puStack32) == 0x0) {
                        iVar14 = 0x0;
                        uVar21 = 0x0;
                    } else {
                        pu_var25 = pass1_1030_560e((pu_var27 & 0xffff | ZEXT24(puVar17) << 0x10));
                        // uVar21 = (pu_var25 >> 0x10);
                        iVar14 = pu_var25;
                    }
                    puStack36 = CONCAT22(uVar21, iVar14);
                    ppcVar7 = (*puStack36 + 0x10);
                    (**ppcVar7)();
                    if (iVar14 == 0x0) {
                        return;
                    }
                    // uVar21 = (puStack36 >> 0x10);
                    u_var5 = (puStack36 + 0x4);
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    u_var4 = (puStack36 + 0x10);
                    uStack28 = u_var4;
                    // uStack26 = (u_var4 >> 0x10);
                    pass1_1030_6222(
                        ctx.PTR_LOOP_1050_5740,
                        0x0,
                        u_var4,
                        u_var5,
                        uStack28,
                        extraout_DX_01,
                        param_9,
                    );
                    puVar17 = (pcStack12 & 0xff);
                    pass1_1030_14b4(
                        uVar20.field_0x16,
                        puStack36,
                        (puStack36 >> 0x10),
                        CONCAT22(pcStack12, puStack14) & 0xffffff,
                        param_9,
                    );
                    lVar6 = CONCAT22(uStack38, uStack40) + 0x1;
                    uStack40 = lVar6;
                    // uStack38 = (lVar6 >> 0x10);
                }
            }
            0x5 => {
                *puVar17 = 0x5280;
                puVar17[0x1] = &USHORT_1050_1028;
                return;
            }
            0x6 => {
                pass1_1030_145a(uVar20.field_0x1a, local_6);
                // for (local_30 = 0x0; local_30 < local_6;
                //     local_30 = (local_30 + 0x1)) {
                //   pu_var27 = local_6;
                //   mem_op_1000_179c(0x21e,puVar17,0x1000);
                //   puStack32 = pu_var27;
                //   uVar16 = puVar17 | puStack32;
                //   puStack30 = puVar17;
                //   if (uVar16 == 0x0) {
                //     uVar12 = 0x0;
                //     uVar16 = 0x0;
                //   }
                //   else {
                //     uVar12 = puStack32;
                //     pass1_1038_30aa((pu_var27 & 0xffff | ZEXT24(puVar17) << 0x10),
                //                     param_9);
                //   }
                //   ppcVar7 = (CONCAT22(uVar16,uVar12) + 0x10);
                //   uStack44 = uVar12;
                //   uStack42 = uVar16;
                //   (**ppcVar7)();
                //   if (uVar12 == 0x0) {
                //     return;
                //   }
                //   u_var5 = (uStack44 + 0x4);
                //   uVar16 = (uStack44 + 0x6);
                //   puStack14 = u_var5;
                //   pcStack12 = (u_var5 >> 0x10);
                //   puVar17 = (uVar16 & 0xff);
                //   pass1_1030_14b4(uVar20.field_0x1a,uStack44,uStack42,
                //                   u_var5 & 0xffff | (uVar16 & 0xff) << 0x10,param_9);
                // }
            }
            _ => {
                pass1_1030_145a(uVar20.field_0x1e, local_6);
                pass1_1030_66de(ctx.PTR_LOOP_1050_5740, local_6, param_9);
                local_30 = 0x0;
                loop {
                    if (local_6 <= local_30) {
                        pass1_1030_154c();
                        pass1_1030_6740(ctx.PTR_LOOP_1050_5740, param_9, param_7);
                        return;
                    }
                    local_14 = ctx.PTR_LOOP_1050_5744;
                    local_12 = (ctx.PTR_LOOP_1050_5744 >> 0x10);
                    paVar26 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_5744);
                    // puStack30 = (paVar26 >> 0x10);
                    puStack32 = paVar26;
                    uVar16 = puStack30 | puStack32;
                    if (uVar16 == 0x0) {
                        uVar12 = 0x0;
                        uVar16 = 0x0;
                    } else {
                        uVar12 = puStack32;
                        pass1_1030_67cc(paVar26);
                    }
                    ppcVar7 = (CONCAT22(uVar16, uVar12) + 0x10);
                    uStack44 = uVar12;
                    uStack42 = uVar16;
                    (**ppcVar7)();
                    if (uVar12 == 0x0) {
                        break;
                    }
                    u_var5 = (uStack44 + 0x4);
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    lVar6 = (uStack44 + 0x8);
                    uStack40 = lVar6;
                    // uStack38 = (lVar6 >> 0x10);
                    param_7 = &local_30;
                    uStack50 = (uStack44 + 0x10);
                    puStack36 = (puStack36 & 0xffff0000 | ZEXT24(&stack0xffca));
                    pass1_1030_671c(
                        ctx.PTR_LOOP_1050_5740,
                        u_var5,
                        CONCAT22(param_9, &stack0xffca),
                        lVar6,
                        &stack0xffca,
                        uStack42,
                        param_7,
                        param_9,
                    );
                    pass1_1030_14b4(
                        uVar20.field_0x1e,
                        uStack44,
                        uStack42,
                        CONCAT22(pcStack12, puStack14) & 0xffffff,
                        param_9,
                    );
                    local_30 = (local_30 + 0x1);
                }
                return;
            }
            0x9 => {
                local_6 = (local_6 & 0xffff);
                if (false) {
                    pass1_1028_ebee(param_1, 0x0, puVar17);
                    return;
                }
                pcStack12 = uVar20.field_0x2e;
                puStack10 = uVar20.field_0x30;
                (*pcStack12)();
                return;
            }
            0xa => {
                pass1_1030_145a(uVar20.field_0x22, local_6);
                uVar21 = 0x0;
                u_var10 = 0x0;
                while (CONCAT22(u_var10, uVar21) < local_6) {
                    pu_var27 = local_6;
                    mem_op_1000_179c(0xe, puVar17, 0x1000);
                    puStack32 = pu_var27;
                    puStack30 = puVar17;
                    if ((puVar17 | puStack32) == 0x0) {
                        iVar14 = 0x0;
                        uVar15 = 0x0;
                    } else {
                        pu_var25 = pass1_1028_b204((pu_var27 & 0xffff | ZEXT24(puVar17) << 0x10));
                        // uVar15 = (pu_var25 >> 0x10);
                        iVar14 = pu_var25;
                    }
                    local_30 = CONCAT22(uVar15, iVar14);
                    ppcVar7 = (*local_30 + 0x10);
                    (**ppcVar7)();
                    if (iVar14 == 0x0) {
                        return;
                    }
                    // uVar22 = (local_30 >> 0x10);
                    uVar19 = local_30;
                    u_var5 = (uVar19 + 0x4);
                    uVar16 = (uVar19 + 0x6);
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    puVar17 = (uVar16 & 0xff);
                    pass1_1030_14b4(
                        uVar20.field_0x22,
                        uVar19,
                        uVar22,
                        u_var5 & 0xffff | (uVar16 & 0xff) << 0x10,
                        param_9,
                    );
                    lVar6 = CONCAT22(u_var10, uVar21) + 0x1;
                    uVar21 = lVar6;
                    // u_var10 = (lVar6 >> 0x10);
                }
            }
            0xb => {
                if (puVar17 < (&ctx.PTR_LOOP_1050_000e + 0x1)) {
                    pcVar1 = (param_6 + 0x23);
                    cVar3 = *pcVar1;
                    *pcVar1 = *pcVar1 << 0x6;
                    piVar2 = (puVar17 + param_6);
                    *piVar2 = *piVar2 + (-0x6600 - ((cVar3 << 0x5) < '\0'));
                } else {
                    pass1_1028_780c(
                        uVar19,
                        uVar22,
                        CONCAT22(in_stack_0000ffcc, in_stack_0000ffca),
                    );
                    if (param_4 == 0x0) {
                        // goto
                        // code_r0x10287b17;
                    }
                }
                uVar29 = 0x0;
                uVar31 = 0x4;
                iVar14 = 0x1d;
                pu_var25 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_9, puVar17, param_7);
                // puVar17 = (pu_var25 >> 0x10);
                param_4 = pu_var25;
                pass1_1010_043a(
                    pu_var25,
                    CONCAT13(uVar31, CONCAT12(uVar29, puVar17)),
                    iVar14,
                    param_9,
                );
                // code_r0x10287b17:
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x2, 0x400);
                pass1_1028_780c(uVar20, uVar37, CONCAT22(puVar17, param_4));
                puStack10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_9, puVar17, param_7);
                pcStack12 = ctx.PTR_LOOP_1050_13ae;
                if (0x2 < ctx.PTR_LOOP_1050_13ae) {
                    pu_var25 = mixed_1010_20ba(
                        ctx.PTR_LOOP_1050_0ed0,
                        0x2f,
                        param_9,
                        (puStack10 >> 0x10),
                        param_7,
                    );
                    // uVar32 = (pu_var25 >> 0x10);
                    uVar33 = (pu_var25 >> 0x18);
                    uVar35 = 0x1;
                    uVar36 = 0x0;
                    uVar29 = pu_var25;
                    uVar31 = (pu_var25 >> 0x8);
                    while (CONCAT11(uVar36, uVar35) < 0x9) {
                        uVar28 = uVar29;
                        uVar30 = uVar31;
                        if ((CONCAT11(uVar31, uVar29) + 0x34 + CONCAT11(uVar36, uVar35) * 0x4)
                            == local_6)
                        {
                            puVar17 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
                            local_30 = CONCAT22(local_30._2_2_, 0x1);
                            uVar35 = 0xd7;
                            uVar36 = 0x7b;
                            pass1_1008_612e(0x1, 0x64, 0x1);
                            puVar9 = (CONCAT11(uVar36, uVar35) - 0x7);
                            if (puVar9 == 0x0) {
                                bVar24 = SBORROW2(puVar17, 0x32);
                                puVar8 = puVar17 + -0x19;
                                bVar23 = puVar17 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);
                                //LAB_1028_7b74:
                                if ((!bVar23 && bVar24) == (puVar8 < 0x0)) {
                                    local_30 = (local_30 & 0xffff0000);
                                }
                            } else {
                                puVar9 = (CONCAT11(uVar36, uVar35) - 0x8);
                                if (puVar9 == 0x0) {
                                    bVar24 = SBORROW2(puVar17, 0x19);
                                    puVar8 = (puVar17 + -0x19);
                                    bVar23 = puVar8 == 0x0;
                                    //                 TODO: goto LAB_1028_7b74;
                                }
                            }
                            puStack30 = puVar17;
                            if (local_30 != 0x0) {
                                pass1_1028_90e6(
                                    CONCAT13(uVar34, CONCAT12(param_9, &stack0xfeac)),
                                    CONCAT11(uVar36, uVar35),
                                    param_9,
                                    param_10,
                                );
                                puVar9 = &stack0xfeac;
                                uVar32 = 0x8;
                                uVar33 = 0x10;
                                uVar29 = 0xc;
                                uVar31 = 0x7c;
                                fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_9, puVar9));
                            }
                            uVar38 = 0x0;
                            uVar39 = 0x0;
                            uVar35 = 0x23;
                            uVar36 = 0x7c;
                            pass1_1008_612e(0x0, 0xa, puVar9);
                            ppuStack24 = puVar9;
                            if (CONCAT11(uVar36, uVar35) == 0x7) {
                                iVar40 = 0x7;
                                puVar9 = puVar9 + 0x6e;
                                iVar14 = puVar9 >> 0xf;
                            } else {
                                uVar28 = uVar29;
                                uVar30 = uVar31;
                                if (CONCAT11(uVar36, uVar35) != 0x8) {
                                    // goto
                                    // LAB_1028_7ba0;
                                }
                                iVar40 = 0x8;
                                puVar9 = puVar9 + 0x64;
                                iVar14 = (puVar9 >> 0xf) + (0xff9b < puVar9);
                            }
                            uVar19 = iVar40 + iVar14 + CARRY2(CONCAT11(uVar39, uVar38), puVar9);
                            uVar28 = 0x8;
                            uVar30 = 0x10;
                            uVar35 = uVar32;
                            uVar36 = uVar33;
                            pass1_1010_ebf8(
                                CONCAT13(uVar33, CONCAT12(uVar32, CONCAT11(uVar31, uVar29))),
                                (puVar9 + CONCAT11(uVar39, uVar38)),
                                uVar19,
                                uVar19,
                            );
                            uVar32 = uVar29;
                            uVar33 = uVar31;
                        }
                        //LAB_1028_7ba0:
                        iVar14 = CONCAT11(uVar36, uVar35) + 0x1;
                        uVar35 = iVar14;
                        uVar29 = uVar28;
                        uVar31 = uVar30;
                        uVar36 = (iVar14 >> 0x8);
                    }
                }
                return;
            }
            0xc => {
                paVar18 = uVar20;
                pass1_1030_145a(uVar20.field_0x26, local_6);
                uVar21 = 0x0;
                u_var10 = 0x0;
                while (CONCAT22(u_var10, uVar21) < local_6) {
                    BVar11 =
                        read_file_1008_7dee(param_2, param_3, &local_30, 0x0, param_9, 0x2, 0x1008);
                    if (BVar11 == 0x0) {
                        ctx.PTR_LOOP_1050_0310 = 0x6d2;
                        return;
                    }
                    uStack44 = switch_1008_73ea(param_2, param_3, local_30);
                    pu_var27 = switch_1030_0000(
                        uVar20, uVar37, uStack44, puVar17, paVar18, param_6, param_7,
                    );
                    // uStack38 = (pu_var27 >> 0x10);
                    uVar19 = pu_var27;
                    ppcVar7 = (*pu_var27 + 0x10);
                    uStack40 = uVar19;
                    (**ppcVar7)();
                    if (uVar19 == 0x0) {
                        return;
                    }
                    u_var5 = (uStack40 + 0x4);
                    uVar16 = (uStack40 + 0x6);
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    puVar17 = (uVar16 & 0xff);
                    paVar18 = uVar20;
                    pass1_1030_14b4(
                        uVar20.field_0x26,
                        uStack40,
                        uStack38,
                        u_var5 & 0xffff | (uVar16 & 0xff) << 0x10,
                        param_9,
                    );
                    lVar6 = CONCAT22(u_var10, uVar21) + 0x1;
                    uVar21 = lVar6;
                    // u_var10 = (lVar6 >> 0x10);
                }
            }
            0xd => {
                puStack10 = (ZEXT24(puVar17) << 0x10);
                uVar13 = &ctx.PTR_LOOP_1050_000c;
                local_10 = uVar13;
                // puStack14 = (uVar13 >> 0x10);
                pcStack12 = *&ctx.PTR_LOOP_1050_0010;
                ppuStack24 = &local_10;
                pass1_1008_3eb4(
                    CONCAT13(uVar34, CONCAT12(param_9, &local_10)),
                    CONCAT22(param_9, &local_16),
                    CONCAT22(param_9, &local_14),
                    CONCAT22(param_9, &local_12),
                );
                ppuStack24 = (local_14 + -0x1);
                puStack14 = ppuStack24;
                uVar16 = pass1_1028_21ba(
                    uVar20,
                    uVar37,
                    CONCAT22(param_9, &local_10),
                    local_6,
                    &local_10,
                    puVar17,
                    param_9,
                );
                if (uVar16 == 0x0) {
                    ppuStack24 = (local_14 + 0x1);
                    puStack14 = ppuStack24;
                    uVar16 = pass1_1028_21ba(
                        uVar20,
                        uVar37,
                        CONCAT22(param_9, &local_10),
                        local_6,
                        &local_10,
                        puVar17,
                        param_9,
                    );
                    if (uVar16 == 0x0) {
                        puStack14 = local_14;
                        ppuStack24 = (local_12 + -0x1);
                        local_10 = ppuStack24;
                        uVar16 = pass1_1028_21ba(
                            uVar20,
                            uVar37,
                            CONCAT22(param_9, &local_10),
                            local_6,
                            &local_10,
                            puVar17,
                            param_9,
                        );
                        if (uVar16 == 0x0) {
                            ppuStack24 = (local_12 + 0x1);
                            local_10 = ppuStack24;
                            uVar16 = pass1_1028_21ba(
                                uVar20,
                                uVar37,
                                CONCAT22(param_9, &local_10),
                                local_6,
                                &local_10,
                                puVar17,
                                param_9,
                            );
                            if (uVar16 == 0x0) {
                                return;
                            }
                        }
                    }
                }
                pass1_1038_79b2(ctx.PTR_LOOP_1050_5a64, puStack10, uVar16, puVar17);
                return;
            }
            0xe => {
                pass1_1030_145a(uVar20.field_0x2a, local_6);
                uVar21 = 0x0;
                u_var10 = 0x0;
                while (CONCAT22(u_var10, uVar21) < local_6) {
                    pu_var27 = local_6;
                    mem_op_1000_179c(0x3b2, puVar17, 0x1000);
                    puStack32 = pu_var27;
                    uVar16 = puVar17 | puStack32;
                    puStack30 = puVar17;
                    if (uVar16 == 0x0) {
                        uVar12 = 0x0;
                        uVar16 = 0x0;
                    } else {
                        uVar12 = puStack32;
                        pass1_1030_2068((pu_var27 & 0xffff | ZEXT24(puVar17) << 0x10));
                    }
                    local_30 = CONCAT22(uVar16, uVar12);
                    ppcVar7 = (*local_30 + 0x10);
                    (**ppcVar7)();
                    if (uVar12 == 0x0) {
                        return;
                    }
                    // uVar22 = (local_30 >> 0x10);
                    uVar19 = local_30;
                    u_var5 = (uVar19 + 0x4);
                    uVar16 = (uVar19 + 0x6);
                    puStack14 = u_var5;
                    // pcStack12 = (u_var5 >> 0x10);
                    puVar17 = (uVar16 & 0xff);
                    pass1_1030_14b4(
                        uVar20.field_0x2a,
                        uVar19,
                        uVar22,
                        u_var5 & 0xffff | (uVar16 & 0xff) << 0x10,
                        param_9,
                    );
                    lVar6 = CONCAT22(u_var10, uVar21) + 0x1;
                    uVar21 = lVar6;
                    // u_var10 = (lVar6 >> 0x10);
                }
            }
        }
        pass1_1030_154c();
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_ebee(param_1: u32, param_2: u16, param_3: u16) -> u32 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;

    mem_op_1000_179c(0x14, param_3, 0x1000);
    if ((param_3 | param_2) != 0x0) {
        pass1_1030_1a32(CONCAT22(param_3, param_2), param_2, (param_3 | param_2));
    }
    u_var3 = struct_1030_4574((param_1 + 0x52));
    u_var2 = (ctx.PTR_LOOP_1050_5166 >> 0x10);
    i_var1 = ctx.PTR_LOOP_1050_5166;
    (i_var1 + 0x10) = u_var3;
    (i_var1 + 0x12) = (u_var3 >> 0x10);
    u_var2 = (ctx.PTR_LOOP_1050_5166 >> 0x10);
    return CONCAT22(
        (ctx.PTR_LOOP_1050_5166 + 0x6),
        (ctx.PTR_LOOP_1050_5166 + 0x4),
    );
}

pub fn pass1_1028_ec36(
    param_1: u32,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u32,
    param_6: u16,
    param_7: U32Ptr,
    param_8: u16,
) {
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let puVar7: U32Ptr;

    mem_op_1000_179c(0x14, param_7, 0x1000);
    if ((param_7 | param_6) == 0x0) {
        u_var2 = 0x0;
        puVar4 = 0x0;
    } else {
        puVar7 = pass1_1030_5d3c(
            CONCAT22(param_7, param_6),
            param_5,
            param_6,
            (param_7 | param_6),
        );
        // puVar4 = (puVar7 >> 0x10);
        u_var2 = puVar7;
    }
    // u_var6 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x52);
    pu_var5 = puVar4;
    u_var3 = u_var2;
    pass1_1030_4594(puVar4, u_var1, (u_var1 >> 0x10), param_3);
    pass1_1030_5fe2(CONCAT22(puVar4, u_var2), CONCAT22(pu_var5, u_var3));
    pass1_1030_1358(
        (param_1 + 0xe),
        u_var2,
        puVar4,
        (u_var2 + 0x4) & 0xffff | ((u_var2 + 0x6) & 0xff) << 0x10,
        param_8,
    );
    return;
}

pub fn pass1_1028_ecac(
    param_1: u32,
    param_2: u16,
    param_3: &mut i16,
    param_4: u16,
    param_5: u32,
    param_6: u16,
    param_7: U32Ptr,
    param_8: u16,
) {
    let u_var1: u32;
    i16 * *ppiVar2;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let u_var5: u16;

    mem_op_1000_179c(0x1c, param_7, 0x1000);
    pu_var3 = (param_7 | param_6);
    if (pu_var3 == 0x0) {
        param_6 = 0x0;
        pu_var3 = 0x0;
    } else {
        struct_1030_299a(CONCAT22(param_7, param_6), param_5, param_6, pu_var3);
    }
    // u_var5 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x52);
    puVar4 = pu_var3;
    ppiVar2 = param_3;
    pass1_1030_4628(pu_var3, u_var1, (u_var1 >> 0x10), param_3);
    *ppiVar2 = param_3;
    pass1_1030_3006(CONCAT22(pu_var3, param_6), CONCAT22(puVar4, ppiVar2));
    pass1_1030_1358(
        (param_1 + 0x12),
        param_6,
        pu_var3,
        (param_6 + 0x4) & 0xffff | ((param_6 + 0x6) & 0xff) << 0x10,
        param_8,
    );
    return;
}

// WARNING: Unable to use type for symbol u_var1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_ed2c(
    param_1: u32,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u32,
    param_6: u16,
    param_7: U32Ptr,
    param_8: u16,
    param_9: u8,
) {
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let u_var1: u32;

    mem_op_1000_179c(0x1e, param_7, 0x1000);
    if ((param_7 | param_6) == 0x0) {
        u_var3 = 0x0;
        pu_var5 = 0x0;
    } else {
        puVar9 = struct_1030_565a(
            CONCAT22(param_7, param_6),
            param_5,
            param_6,
            (param_7 | param_6),
        );
        // pu_var5 = (puVar9 >> 0x10);
        u_var3 = puVar9;
    }
    // uVar8 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x52);
    puVar6 = pu_var5;
    u_var4 = u_var3;
    pass1_1030_4782(
        param_8,
        param_9,
        pu_var5,
        u_var1,
        (u_var1 >> 0x10),
        0x1,
        0x1,
        param_3,
    );
    puVar7 = puVar6;
    pass1_1030_5a80(CONCAT22(pu_var5, u_var3), CONCAT22(puVar6, u_var4), param_8);
    u_var2 = (u_var3 + 0x4);
    pass1_1030_6222(
        ctx.PTR_LOOP_1050_5740,
        0x1,
        CONCAT22(puVar6, u_var4),
        u_var2,
        u_var2,
        puVar7,
        param_8,
    );
    pass1_1030_1358(
        (param_1 + 0x16),
        u_var3,
        pu_var5,
        u_var2 & 0xffffff,
        param_8,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_edc4(
    param_1: u32,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i32,
    param_5: U32Ptr,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let in_AF: u8;
    let local_1a: [u8; 4];
    let uStack22: u32;
    let uStack18: u32;
    let uStack14: u32;
    let uStack10: u32;
    let puStack6: U32Ptr;

    puStack6 = param_3;
    pass1_1030_64ce(
        param_6,
        param_3,
        param_5,
        ctx.PTR_LOOP_1050_5740,
        param_3,
        param_4,
        CONCAT22(param_6, local_1a),
    );
    u_var2 = param_3;
    uStack14 = u_var2;
    uStack10 = u_var2;
    mem_op_1000_179c(0x21e, param_5, 0x1000);
    u_var1 = u_var2;
    pu_var3 = (param_5 | u_var1);
    if (pu_var3 == 0x0) {
        u_var1 = 0x0;
        pu_var3 = 0x0;
    } else {
        pass1_1038_3222(
            (u_var2 & 0xffff | ZEXT24(param_5) << 0x10),
            uStack14,
            param_4,
            u_var1,
            pu_var3,
            in_AF,
            param_6,
        );
    }
    uStack18 = CONCAT22(pu_var3, u_var1);
    uStack22 = (u_var1 + 0x4);
    pass1_1030_1358(
        (param_1 + 0x1a),
        u_var1,
        pu_var3,
        uStack22 & 0xffff | ((u_var1 + 0x6) & 0xff) << 0x10,
        param_6,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_ee54(param_1: u32, param_2: u16, param_3: U32Ptr, param_4: u32) {
    let in_DX: u16;
    let u_var1: u16;
    let unaff_SS: u16;
    let paVar2: &mut Struct99;
    let local_16: [u8; 4];
    let uStack18: u32;
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u32;
    let puStack6: U32Ptr;

    puStack6 = param_3;
    pass1_1030_64ce(
        unaff_SS,
        param_3,
        in_DX,
        ctx.PTR_LOOP_1050_5740,
        param_3,
        param_4,
        CONCAT22(unaff_SS, local_16),
    );
    uStack10 = param_3;
    paVar2 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_5744);
    // u_var1 = (paVar2 >> 0x10);
    uStack14 = paVar2;
    uStack12 = u_var1 | uStack14;
    if (uStack12 == 0x0) {
        uStack14 = 0x0;
        uStack12 = 0x0;
    } else {
        pass1_1030_684c(
            (paVar2 & 0xffff | u_var1 << 0x10),
            puStack6,
            (puStack6 >> 0x10),
            uStack10,
            (uStack10 >> 0x10),
            param_4,
            uStack12,
        );
    }
    uStack18 = (uStack14 + 0x4);
    pass1_1030_61fe(
        ctx.PTR_LOOP_1050_5740,
        uStack18,
        puStack6,
        param_4,
        uStack18,
        uStack12,
        unaff_SS,
    );
    pass1_1030_1358(
        (param_1 + 0x1e),
        uStack14,
        uStack12,
        uStack18 & 0xffff | (uStack18._2_2_ & 0xff) << 0x10,
        unaff_SS,
    );
    return;
}

pub fn pass1_1028_ef00(
    param_1: u16,
    param_2: U32Ptr,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) {
    let u_var1: u16;
    let pu_var2: U32Ptr;

    if (param_5 == 0x4) {
        mem_op_1000_179c(0x16, param_2, 0x1000);
        u_var1 = param_2 | param_5;
        if (u_var1 != 0x0) {
            pass1_1030_b936(param_5, param_2, 0x4, _param_6, u_var1);
            //       TODO: goto LAB_1028_ef8b;
        }
    } else {
        if (param_5 == 0xc) {
            mem_op_1000_179c(0xe, param_2, 0x1000);
            if ((param_2 | param_5) != 0x0) {
                pu_var2 = pass1_1030_bc24(param_2 | param_5, param_5, param_2, 0xc, _param_6);
                // u_var1 = (pu_var2 >> 0x10);
                param_5 = pu_var2;
                //         TODO: goto LAB_1028_ef8b;
            }
        } else {
            u_var1 = param_5;
            mem_op_1000_179c(0xe, param_2, 0x1000);
            if ((param_2 | u_var1) != 0x0) {
                pu_var2 = pass1_1028_b22c(
                    CONCAT22(param_2, u_var1),
                    param_5,
                    _param_6,
                    param_2 | u_var1,
                );
                // u_var1 = (pu_var2 >> 0x10);
                param_5 = pu_var2;
                //         TODO: goto LAB_1028_ef8b;
            }
        }
    }
    param_5 = 0x0;
    u_var1 = 0x0;
    //LAB_1028_ef8b:
    pass1_1030_1358(
        (param_3 + 0x22),
        param_5,
        u_var1,
        (param_5 + 0x4) & 0xffff | ((param_5 + 0x6) & 0xff) << 0x10,
        param_1,
    );
    return;
}
