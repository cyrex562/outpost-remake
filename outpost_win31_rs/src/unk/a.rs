use std::ptr::null_mut;
use crate::file_ops::close_file_1008_496c;
use crate::structs::struct_57::Struct57;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1008_3000::struct_op_1008_3f92;
use crate::unk::block_1008_4000::{pass1_1008_405c, pass1_1008_4d84, struct_1008_4c58, struct_op_1008_48fe};
use crate::unk::block_1010_8000::pass1_1010_8096;
use crate::utils::{CONCAT22, SUB42};
use crate::winapi16::GetClientRect16;
use crate::windef16::{HWND16, RECT16};

pub unsafe fn mixed_sys_op_1018_2978(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_931,
) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut paVar2: *mut astruct_394;
    let rect: *mut RECT16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut paVar7: *mut Struct57;
    let mut iVar9: *mut astruct_931;
    let mut uVar8: u16;
    let mut uVar7: u16;
    let mut uVar9: u16;
    let mut HVar8: HWND16;
    let mut paVar9: *mut astruct_394;
    let mut local_3a: RECT16;
    let mut iStack54: i16;
    let mut iStack52: i16;
    let mut uStack50: u32;
    let mut puStack46: *mut u32;
    let mut local_2a: astruct_394 = astruct_394::default();
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut uVar10: u8;
    let mut paVar6: *mut Struct57;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    pass1_1010_8096(_u16_1050_14cc, 1);
    uStack4 = SUB42(paVar5, 0x0);
    puVar2 = &local_2a;
    uStack6 = param_1;
    struct_op_1008_48fe(
        paVar5,
        CONCAT13(0x10, CONCAT12(0x50, puVar2)),
        0x1,
        CONCAT22(uStack4, param_1),
    );
    uVar7 = 0x1000;
    mem_op_1000_179c(0x1e, paVar5);
    uVar5 = paVar5 | puVar2;
    paVar7 = (paVar5 & 0xffff0000);
    paVar6 = (paVar7 | uVar5);
    if (uVar5 == 0) {
        paVar2 = null_mut();
    } else {
        paVar2 = &local_2a;
        uVar7 = 0x1008;
        struct_op_1008_3f92(CONCAT22(paVar5, puVar2), CONCAT22(0x1050, paVar2));
        paVar7 = paVar6;
    }
    uVar3 = SUB42(paVar7, 0x0);
    puStack46 = CONCAT22(uVar3, paVar2);
    ppcVar1 = (*puStack46 + 0x14);
    paVar9 = paVar2;
    (**ppcVar1)(uVar7, paVar2, uVar3);
    uStack50 = CONCAT22(paVar7, paVar2);
    mem_op_1000_179c(0x14, paVar7);
    uVar4 = paVar7 | paVar2;
    paVar7 = (paVar7 & 0xffff0000);
    paVar5 = (paVar7 | uVar4);
    if (uVar4 == 0) {
        paVar2 = null_mut();
    } else {
        struct_1008_4c58(paVar2);
        paVar7 = paVar5;
    }
    uVar8 = (param_3 >> 0x10);
    iVar9 = param_3;
    iVar9.field12_0xe = paVar2;
    (iVar9.field12_0xe + 0x2) = paVar7;
    pass1_1008_4d84(paVar7, iVar9.field12_0xe, uStack50);
    rect = &local_3a;
    HVar8 = HWND16_1050_0396;
    GetClientRect16(rect, 0x1050);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, paVar7);
    uVar6 = paVar7 | rect;
    if (uVar6 == 0) {
        iVar9.field10_0xa = 0;
    } else {
        iVar4 = (iStack52 - local_3a.y) + 1;
        uVar9 = 0x1008;
        pass1_1008_405c(
            CONCAT22(paVar7, rect),
            iVar9.field12_0xe,
            iVar4,
            (iStack54 - local_3a.x) + 1,
        );
        iVar9.field10_0xa = iVar4;
        iVar9.field11_0xc = uVar6;
    }
    if (puStack46.is_null() == false) {
        ppcVar1 = *puStack46;
        (**ppcVar1)(
            uVar9,
            puStack46,
            (puStack46 >> 0x10),
            0x1,
            HVar8,
            paVar9,
            uVar3,
            puStack46,
            puStack46,
        );
    }
    close_file_1008_496c(CONCAT13(0x10, CONCAT12(0x50, &local_2a)));
    return;
}
