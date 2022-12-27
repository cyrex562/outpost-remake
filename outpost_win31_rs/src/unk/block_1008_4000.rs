use std::ptr::null_mut;
use crate::unk::block_1000_0000::{call_fn_ptr_1000_0dc6, mem_op_1000_0a48};
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c, pass1_1000_1284};
use crate::unk::block_1000_3000::str_op_1000_3da4;
use crate::unk::block_1000_4000::{pass1_1000_48a8, pass1_1000_4906};
use crate::unk::block_1008_3000::pass1_1008_3e94;
use crate::unk::block_1008_5000::struct_op_1008_56b4;
use crate::unk::block_1008_6000::str_op_1008_60e8;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::{draw_ops, file_ops};
use crate::globals::DAT_1050_1050;
use crate::structs::struct_288::astruct_288;
use crate::structs::struct_394::astruct_394;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_d::StructD;
use crate::utils::CONCAT22;
use crate::winapi16::{_lclose16, _llseek16, _lopen16, CreateDC16, CreatePalette16, DeleteDC16, DeleteObject16, hmemcpy16, RealizePalette16, SelectPalette16, SetBkColor16, SetTextColor16, TextOut16};
use crate::windef16::{COLORREF, HDC16, HFILE16, HPALETTE16, LOGPALETTE};

pub fn pass1_1008_4016(param_1: *mut astruct_76) {
    let mut iVar1: *mut astruct_76;
    let mut uVar1: u16;

    struct_op_1008_56b4(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field3_0x6 = 0;
    iVar1.field5_0xa = 0;
    iVar1.field7_0xe = 0;
    iVar1.field8_0x10 = 0;
    iVar1.field9_0x14 = 0;
    iVar1.field11_0x18 = 0;
    iVar1.field13_0x1c = 0;
    // just 0x48de
    param_1.offset_0x0 = &PTR_LOOP_1050_48de;
    iVar1.base_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_405c(
    param_1: *mut astruct_76,
    mut param_2: u32,
    mut param_3: i16,
    mut param_4: i16,
) {
    let mut uVar1: u32;
    let mut sVar2: i64;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut lVar5: i32;
    let mut puVar6: *mut u8;
    let mut iVar4: *mut astruct_76;
    let mut uVar7: u16;
    let mut uStack10: u32;

    struct_op_1008_56b4(param_1);
    uVar7 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field3_0x6 = 0;
    iVar4.field5_0xa = 0;
    iVar4.field7_0xe = 0;
    iVar4.field8_0x10 = 0;
    iVar4.field9_0x14 = 0;
    iVar4.field11_0x18 = 0;
    iVar4.field13_0x1c = 0;
    param_1.offset_0x0 = &PTR_LOOP_1050_48de;
    iVar4.base_0x2 = 0x1008;
    iVar3 = param_4 * 0x8 + 0x1f;
    uVar4 = ((iVar3 + (iVar3 >> 0xf & 0x1f)) >> 0x5) << 0x2;
    uStack10 = param_3;
    lVar5 = uVar4 * param_3 + 0x436;
    lVar5 = mem_op_1000_0a48(0x1, lVar5, (lVar5 >> 0x10), _PTR_LOOP_1050_5f2c);
    iVar4.field3_0x6 = lVar5;
    iVar4.field4_0x8 = (lVar5 >> 0x10);
    pass1_1008_47cc((param_1 & 0xffff | uVar7 << 0x10));
    iVar4.field11_0x18 = uVar4;
    iVar4.field12_0x1a = uVar4 >> 0xf;
    iVar4.field8_0x10 = 0x28;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0x4) = param_4;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0x8) = uStack10;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0xc) = 0x1;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0xe) = 0x8;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0x10) = 0;
    sVar2 = &iVar4.field11_0x18 * uStack10;
    puVar6 = (sVar2 >> 0x20);
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0x14) = sVar2;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0x20) = 0x100;
    uVar1 = iVar4.field8_0x10;
    (uVar1 + 0x24) = 0x100;
    pass1_1008_4834(param_1);
    pass1_1008_4d84(puVar6, &iVar4.field5_0xa, param_2);
    return;
}

pub fn pass1_1008_41bc(param_1: *mut astruct_288) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar5: *mut astruct_288;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1 = &PTR_LOOP_1050_48de;
    iVar5.field2_0x2 = 0x1008;
    puVar1 = iVar5.field6_0xa;
    uVar2 = iVar5.field7_0xc;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    if (iVar5.field5_0x6 != 0) {
        call_fn_ptr_1000_0dc6(iVar5.field5_0x6);
    }
    param_1 = 0x389a;
    iVar5.field2_0x2 = 0x1008;
    return;
}
pub fn struct_op_1008_4214(param_1: *mut astruct_76, param_2: *mut astruct_81) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_81;
    let mut uVar4: *mut astruct_81;

    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    (param_1 + 0x6) = iVar4.buffer_0x1a;
    iVar4.buffer_0x1a = 0;
    puVar1 = &iVar4.field2_0x4;
    uVar2 = (&iVar4.field2_0x4 + 2);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar4.field2_0x4 = 0;
    iVar4.field6_0xe = 0;
    iVar4.field7_0x12 = 0;
    iVar4.field8_0x16 = 0;
    iVar4.field10_0x1e = 0;
    return;
}


pub fn pass1_1008_431c(param_1: *mut astruct_76, param_2: u8) {
    let mut puVar1: *mut u32;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut uVar4: u32;
    let mut iVar5: *mut astruct_76;
    let mut uVar5: *mut astruct_76;
    let mut uStack6: u32;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (&iVar5.field3_0x6 == 0) {
        pass1_1008_47cc((param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
    }
    if ((iVar5.field4_0x8 | iVar5.field3_0x6) == 0) {
        bVar3 = false;
    } else {
        if ((iVar5.field6_0xc | iVar5.field5_0xa) == 0) {
            pass1_1008_4834((param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
        }
        bVar3 = true;
    }
    if (bVar3) {
        if ((iVar5.field10_0x16 | iVar5.field9_0x14) == 0) {
            return;
        }
        uStack6 = 0;
        loop {
            uVar2 = iVar5.field8_0x10;
            puVar1 = (uVar2 + 0x8);
            if (*puVar1 == uStack6 || *puVar1 < uStack6) {
                break;
            }
            uVar4 = uStack6;
            pass1_1008_4544(param_1);
            uVar2 = iVar5.field8_0x10;
            pass1_1000_4906((uVar4 & 0xffff | uStack6 << 0x10), param_2, (uVar2 + 0x4));
            uStack6 += 0x1;
        }
    }
    return;
}

pub fn pass1_1008_43cc(param_1: *mut astruct_76) -> u32 {
    let mut bVar1: bool;
    let mut iVar2: *mut astruct_76;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (&iVar2.field3_0x6 == 0) {
        pass1_1008_47cc((param_1 & 0xffff | uVar2 << 0x10));
    }
    if (&iVar2.field3_0x6 == 0) {
        bVar1 = false;
    } else {
        if (&iVar2.field5_0xa == 0) {
            pass1_1008_4834((param_1 & 0xffff | uVar2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0x0;
    }
    return CONCAT22(iVar2.field10_0x16, iVar2.field9_0x14);
}


pub fn pass1_1008_4480(
    param_1: *mut astruct_76,
    param_2: *mut u16,
    param_3: *mut astruct_76,
) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut iStack26: i16;
    let mut pcStack24: *mut c_char;
    let mut pcStack20: *mut c_char;
    let mut iStack16: i16;
    let mut local_6: i16;
    let mut local_4: [u8; 0x2] = [0; 0x2];

    pass1_1008_3e94(
        param_2,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, local_4),
    );
    uVar7 = pass1_1008_4772(param_3);
    uVar5 = (uVar7 >> 0x10);
    iVar1 = (uVar7 + 0x4);
    iVar2 = (uVar7 + 0x8);
    //   for (iStack16 = 0; iStack16 < iVar2; iStack16 += 1)
    for iStack16 in 0..iVar2 {
        uVar6 = local_6 >> 0xf;
        iVar3 = local_6;
        local_6 = local_6 + 1;
        pass1_1008_4544(param_1);
        pcStack20 = CONCAT22(uVar6, iVar3);
        uVar4 = iStack16;
        pass1_1008_4544(param_3);
        pcStack24 = (uVar4 & 0xffff | uVar6 << 0x10);
        iStack26 = iVar1;
        while (iStack26 != 0) {
            if (*pcStack24 != -1) {
                *pcStack20 = *pcStack24;
            }
            pcStack24 = CONCAT22(
                (pcStack24 >> 0x10) + (-(0xfffe < pcStack24) & 0x6c),
                pcStack24 + 1,
            );
            pcStack20 = CONCAT22(
                (pcStack20 >> 0x10) + (-(0xfffe < pcStack20) & 0x6c),
                pcStack20 + 1,
            );
            iStack26 = iStack26 -0x1;
        }
    }
    return;
}

pub fn pass1_1008_4544(param_1: *mut astruct_76) {
    let mut bVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x6) == 0) {
        pass1_1008_47cc((param_1 & 0xffff | uVar3 << 0x10));
    }
    if ((iVar2 + 0x6) == 0) {
        bVar1 = false;
    } else {
        if ((iVar2 + 0xa) == 0) {
            pass1_1008_4834((param_1 & 0xffff | uVar3 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    return;
}

pub fn pass1_1008_4772(param_1: *mut astruct_76) -> u32 {
    let mut bVar1: bool;
    let mut iVar2: *mut astruct_76;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (&iVar2.field3_0x6 == 0) {
        pass1_1008_47cc((param_1 & 0xffff | uVar2 << 0x10));
    }
    if (&iVar2.field3_0x6 == 0) {
        bVar1 = false;
    } else {
        if (&iVar2.field5_0xa == 0) {
            pass1_1008_4834((param_1 & 0xffff | uVar2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0x0;
    }
    return CONCAT22((&iVar2.field8_0x10 + 0x2), &iVar2.field8_0x10);
}
pub fn pass1_1008_47cc(param_1: *mut astruct_76) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: *mut astruct_76;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uStack14: u32;

    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (&iVar5.field3_0x6 != 0) {
        uVar2 = &iVar5.field3_0x6;
        uVar1 = iVar5.field4_0x8;
        iVar6 = uVar2;
        uVar4 = iVar6 + 0xe;
        iVar5.field8_0x10 = uVar2 & 0xffff0000 | uVar4;
        iVar5.field9_0x14 = iVar6 + 0x436;
        iVar5.field10_0x16 = uVar1 + (-(0xfbd7 < uVar4) & 0x6c);
        uVar3 = iVar5.field8_0x10;
        uVar8 = (uVar3 >> 0x10);
        iVar6 = uVar3;
        uStack14 = (iVar6 + 0xe);
        iVar5.field11_0x18 = (uStack14 * (iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
    }
    return;
}
pub fn pass1_1008_4834(param_1: *mut astruct_76) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut paVar5: *mut Struct57;
    let mut struct_var5_1: *mut astruct_76;
    let mut struct_var5: *mut astruct_76;
    let mut paStack10: *mut astruct_76;

    struct_var5 = (param_1 >> 0x10);
    struct_var5_1 = param_1;
    puVar2 = struct_var5_1.field5_0xa;
    uVar4 = struct_var5_1.field6_0xc;
    paVar5 = (in_EDX & 0xffff0000 | uVar4);
    if ((uVar4 | puVar2) != 0) {
        ppcVar1 = *puVar2;
        (**ppcVar1)();
    }
    mem_op_1000_179c(0x14, paVar5);
    paStack10 = CONCAT22(paVar5, puVar2);
    uVar4 = paVar5 | puVar2;
    if (uVar4 != 0) {
        uVar3 = struct_var5_1.field8_0x10;
        uVar3 = uVar3 & 0xffff0000 | (uVar3 + 0x28);
        struct_op_1008_4c98(paStack10, uVar3, 0x100);
        struct_var5_1.field5_0xa = uVar3;
        struct_var5_1.field6_0xc = uVar4;
        return;
    }
    struct_var5_1.field5_0xa = 0;
    return;
}

// WARNING: Instruction at (ram,0x10084942) overlaps instruction at (ram,0x10084941)
//

pub fn struct_op_1008_48fe(
    param_1: *mut Struct57,
    param_2: *mut astruct_81,
    mut param_3: u16,
    param_4: *mut c_char,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut pstruct81_2: *mut astruct_81;
    let mut uVar3: u16;

    uVar2 = param_1;
    uVar3 = (param_2 >> 0x10);
    pstruct81_2 = param_2;
    param_2.field0_0x0 = 0x389a;
    pstruct81_2.field1_0x2 = 0x1008;
    pstruct81_2.field2_0x4 = 0;
    pstruct81_2.field3_0x8 = 0;
    pstruct81_2.hfile_0xc = 0xffff;
    pstruct81_2.field6_0xe = 0;
    pstruct81_2.field7_0x12 = 0;
    pstruct81_2.field8_0x16 = 0;
    pstruct81_2.buffer_0x1a = 0;
    pstruct81_2.field10_0x1e = 0;
    pstruct81_2.field13_0x22 = param_3;
    // just 0x4c4c
    param_2.field0_0x0 = &u16_1050_4c4c;
    pstruct81_2.field1_0x2 = 0x1008;
    uVar1 = str_op_1008_60e8(uVar2, param_4);
    pstruct81_2.field3_0x8 = uVar1;
    pstruct81_2.field4_0xa = uVar2;
    return;
}




pub fn pass1_1008_4b8e(param_1: *mut u8, param_2: *mut astruct_807) {
    let mut uVar1: u32;
    let mut in_register_0000000a: u16;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffe4: u32;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut iStack10: i16;

    puVar3 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x48),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar2 = (puVar3 >> 0x10);
    uVar1 = (puVar3 + 0x18);
    iStack18 = (puVar3 + 0x16) / 0x2;
    iStack10 = uvar1;
    uVar2 = param_2 >> 0x10;
    // for (iStack16 = 0; iStack10 = uVar1, uVar2 = (param_2 >> 0x10), iStack16 < iStack18;    iStack16 += 1)
    for iStack16 in 0..iStack18 {
        pass1_1008_4d26(
            (param_2 + 0x4),
            (uVar1 & 0xffff0000 | (iStack16 * 0x4 + iStack10)),
            iStack16,
        );
    }
    //   for (iStack18 = 0x100 - iStack18; iStack18 < 0x100; iStack18 += 1)
    for iStack18 in 0x100 - iStack18..0x100 {
        pass1_1008_4d26(
            (param_2 + 0x4),
            (uVar1 & 0xffff0000 | (iStack16 * 0x4 + iStack10)),
            iStack18,
        );
        iStack16 += 0x1;
    }
    return;
}

pub fn struct_1008_4c58(param_1: *mut astruct_394) {
    let mut iVar1: *mut astruct_394;
    let mut in_stack_00000006: u16;

    *_param_1 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    param_1.field3_0x4 = 0;
    param_1.field8_0xc = 0;
    param_1.field9_0xe = 0;
    param_1.field10_0x12 = 0x1;
    *_param_1 = 0x4f1c;
    param_1.field2_0x2 = 0x1008;
    return;
}

pub fn struct_op_1008_4c98(param_1: *mut astruct_76, mut param_2: u32, mut param_3: u16) {
    let mut iVar1: *mut astruct_76;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.offset_0x0 = 0x389a;
    iVar1.base_0x2 = 0x1008;
    iVar1.field2_0x4 = param_2;
    iVar1.field6_0xc = param_3;
    iVar1.field7_0xe = 0;
    (&iVar1.field8_0x10 + 0x2) = 0;
    param_1.offset_0x0 = 0x4f1c;
    iVar1.base_0x2 = 0x1008;
    return;
}

pub fn pass1_1008_4cdc(param_1: *mut astruct_454) {
    let mut iVar2: *mut astruct_454;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1 = 0x4f1c;
    iVar2.field2_0x2 = 0x1008;
    fn_ptr_1000_17ce(iVar2.field10_0xe);
    if (iVar2.field11_0x12 != 0) {
        fn_ptr_1000_17ce(iVar2.field3_0x4);
    }
    param_1 = 0x389a;
    iVar2.field2_0x2 = 0x1008;
    return;
}

pub fn pass1_1008_4d26(
    param_1: *mut astruct_650,
    param_2: *mut u16,
    mut param_3: i16,
) -> u16 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut lVar3: i32;
    let mut iVar5: *mut astruct_650;
    let mut iVar4: *mut astruct_649;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (((iVar5.field4_0x4 != 0) && (-0x1 < param_3))
        && (
            piVar1 = &iVar5.field9_0xc,
            *piVar1 != param_3 && param_3 <= *piVar1,
        ))
    {
        uVar2 = (param_2 + 2);
        lVar3 = iVar5.field4_0x4;
        uVar4 = (lVar3 >> 0x10);
        iVar4 = lVar3;
        (iVar4 + param_3 * 0x4) = *param_2;
        (iVar4 + param_3 * 0x4 + 0x2) = uVar2;
        return 0x1;
    }
    return 0x0;
}

pub fn pass1_1008_4d72(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x6), (param_1 + 0x4));
}
pub fn pass1_1008_4d84(param_1: *mut u8, param_2: *mut astruct_90, mut param_3: u32) {
    let mut iVar1: i16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar3: *mut astruct_90;
    let mut uVar3: u16;
    let mut uVar4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    uVar3 = (param_2 >> 0x10);
    iVar3 = param_2;
    uVar4 = (param_3 >> 0x10);
    if (iVar3.field14_0x12 != 0) {
        iVar3.field9_0xc = (param_3 + 0xc);
        fn_ptr_1000_17ce(iVar3.field4_0x4);
        iVar3.field4_0x4 = 0;
        iVar1 = iVar3.field9_0xc << 0x2;
        mem_op_1000_179c(iVar1, paVar2);
        iVar3.field4_0x4 = iVar1;
        (&iVar3.field4_0x4 + 0x2) = paVar2;
    }
    if (iVar3.field9_0xc != 0x100) {
        return;
    }
    pass1_1000_48a8(iVar3.field4_0x4, (param_3 + 0x4), 0x400);
    return;
}

pub fn pass1_1008_5068(param_1: *mut astruct_76, param_2: *mut astruct_81) {
    struct_op_1008_4214(param_1, param_2);
    return;
}
