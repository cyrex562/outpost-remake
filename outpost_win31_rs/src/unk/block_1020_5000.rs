pub fn pass1_1020_5d56(mut param_1: u16, param_2: *mut u32, mut param_3: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut local_12: [i16; 0x2] = [0; 0x2];
    let mut local_e: i16;
    let mut local_c: i16;
    let mut local_a: [i16; 0x2] = [0; 0x2];
    let mut iStack6: i16;

    iStack6 = (param_3 + 0x2e);
    uVar2 = param_2;
    uVar3 = (param_2 >> 0x10);
    if (iStack6 == 0x47) {
        pass1_1020_61c4(
            uVar2,
            uVar3,
            CONCAT22(0x1050, &local_c),
            CONCAT22(0x1050, local_a),
        );
        //    if (local_a[0] == 0) goto LAB_1020_5d8b;
        if (local_c <= local_a[0]) {
            return 0x1;
        }
    } else {
        if (iStack6 != 0x6a) {
            return 0x0;
        }
        pass1_1020_61c4(
            uVar2,
            uVar3,
            CONCAT22(0x1050, &local_e),
            CONCAT22(0x1050, local_12),
        );
        if (local_e <= local_12[0]) {
            //
            // LAB_1020_5d8b:
            ppcVar1 = (*param_2 + 0x40);
            (**ppcVar1)();
            return 0x1;
        }
    }
    pass1_1038_af40(uVar2, param_1, _PTR_LOOP_1050_5b7c, (uVar2 + 0x8), 0x9);
    return 0x1;
}
