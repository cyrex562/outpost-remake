use std::ptr::null_mut;
use crate::unk::block_1000_1000::{msg_box_op_1000_1f24, pass1_1000_1f68};
use crate::unk::block_1000_2000::mem_op_1000_21b6;
use crate::globals::{DAT_1050_1050, PTR_LOOP_1050_1000};

pub fn _SHI_INVOKEERRORHANDLER1() -> u16
{
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut uVar2: u16;
    let mut pcStack6: *mut code;
    let mut puStack4: *mut u16;
    let mut uVar3: u16;

    puStack4 =  0x1050;
    if (( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0) {
        pcStack6 = null_mut();
        puStack4 = null_mut();
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pcStack6 =  PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0) {
            PTR_PTR_1050_5f1a =  &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c =  &PTR_LOOP_1050_1000;
            pcStack6 =  &PTR_PTR_1050_1f7e;
            puStack4 =  &PTR_LOOP_1050_1000;
        }
    }
    if (( puStack4 |  pcStack6) != 0) {
        BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                      0x1050,
                                     0x0);
        if (BVar2 == 0) {
            uVar2 = (*pcStack6)();
        } else {
            puStack4 = null_mut();
            pcStack6 = null_mut();
            uVar2 = 0;
        }
        if (( puStack4 |  pcStack6) != 0) {
            pass1_1000_1f68();
        }
        return uVar2;
    }
    return 0x0;
}

pub fn ___EXPORTEDSTUB() -> u16
{
    return 0x0;
}
