use crate::block_1000::block_1000_1000::{msg_box_op_1000_1f24, pass1_1000_1f68};
use crate::block_1000::block_1000_2000::mem_op_1000_21b6;

pub fn _SHI_INVOKEERRORHANDLER1() -> u16
{
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut uVar2: u16;
    code *pcStack6;
    let mut puStack4: *mut u16;
    let mut uVar3: u16;

    puStack4 =  &DAT_1050_1050;
    if (( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0x0) {
        pcStack6 = NULL;
        puStack4 = NULL;
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pcStack6 = (code *) PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0x0) {
            PTR_PTR_1050_5f1a =  &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c =  &PTR_LOOP_1050_1000;
            pcStack6 = (code *) &PTR_PTR_1050_1f7e;
            puStack4 =  &PTR_LOOP_1050_1000;
        }
    }
    if (( puStack4 |  pcStack6) != 0x0) {
        BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                      &DAT_1050_1050,
                                     0x0);
        if (BVar2 == 0x0) {
            uVar2 = (*pcStack6)();
        } else {
            puStack4 = NULL;
            pcStack6 = NULL;
            uVar2 = 0x0;
        }
        if (( puStack4 |  pcStack6) != 0x0) {
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
