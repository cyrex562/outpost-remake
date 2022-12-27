use crate::unk::block_1040_5000::{pass1_1040_5cd6, pass1_1040_5d42, pass1_1040_5dc4, pass1_1040_5eaa};
use crate::unk::block_1040_6000::pass1_1040_6470;
use crate::resources::msg_box_ui_op_1040_64ca;
use crate::winapp::winapp_a::create_window_1040_6eae;


pub fn pass1_1040_557c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_4f0a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_57d4(param_1: *mut u8, param_2: *mut StructB)

{
    pass1_1040_5d42(param_2);
    pass1_1040_5eaa(param_2);
    pass1_1040_5dc4(param_1, param_2);
    unk_win_ui_op_1040_b230(param_1, param_2);
    return;
}


pub fn pass1_1040_6360(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_6794(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_6470(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_68d2(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        }
    } else {
        if (param_5 != 0x111) {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x80);
        (**ppcVar1)();
    }
    return 0x1;
}


pub fn pass1_1040_692e(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}
