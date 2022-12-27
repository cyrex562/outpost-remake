

pub fn big_switch_1008_15d4(
    param_1: *mut astruct_20,
    param_2: *mut Struct72,
    param_3: i32,
) {
    let mut var3: u16;
    let mut var2: *mut u8;
    let mut var4: u16;
    let mut var5: *mut astruct_20;
    let mut puVar1: *mut u8;
    let mut uVar3: u16;
    let mut in_edx: *mut Struct57;
    let mut uVar4: u32;
    let mut var6: u16;
    let mut paVar2: *mut astruct_20;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa6: u16;
    let mut uVar5: u16;
    let mut paStack32: *mut astruct_20;
    let mut local_e: [u8; 0x8] = [0; 0x8];
    let mut uStack6: u32;
    let mut uVar2: u32;
    let mut var_1: *mut i32;
    let mut piVar1: *mut i16;

    uStack6 = 0;
    var3 = param_2;
    var3 = var3 + 0xd2;
    pass1_1008_57a4(CONCAT22(0x1050, local_e), param_2 & 0xffff0000 | var3);
    loop {
        var2 = local_e;
        pass1_1008_5b12(CONCAT22(0x1050, var2));
        uVar3 = in_edx;
        var5 = (uVar3 | var2);
        uVar4 = in_edx & 0xffff0000;
        in_edx = (uVar4 | ZEXT24(var5));
        // if (var5.is_null()) {goto LAB_1008_162a;}
        uVar2 = (var2 + 0x4);
        uVar3 = (var2 + 0x6);
        in_edx = (uVar4 | uVar3);
        param_1 = uVar2;
        if !(param_1.field164_0xde != param_3) {
            break;
        }
    }
    uStack6 = uVar2 & 0xffff | uVar3 << 0x10; //
                                              // LAB_1008_162a:
    if (uStack6 != 0) {
        return;
    }
    uVar5 = (param_2 >> 0x10);
    //   switch(param_3 - 1)
    match param_3 - 1 {
        //   case 0x0:
        0x0 => {
            mem_op_1000_179c(0xec, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            if (puVar1.is_null()) {
                //
                // LAB_1008_169a:
                puVar1 = uVar4;
                uStack6 = 0;
                // TODO: goto LAB_1008_2b3a;
            }
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            pass1_1020_08b6(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
        }
        // break;
        //   _ =>
        _ => {
            debug_print_1008_6048(in_edx, s_OpWnd__getKid__Unknown_target_mo_1050_01a3);
            puVar1 = in_edx;
            fn_ptr_op_1000_24cd(1);
        }
        // TODO: goto LAB_1008_2b3a;
        //   case 0x2:
        0x2 => {
            mem_op_1000_179c(0xfa, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            pass1_1018_e91e(
                puVar1,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
            );
        }
        // break;
        //   case 0x3:
        0x3 => {
            mem_op_1000_179c(0xf6, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            pass1_1018_e230(
                puVar1,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
            );
        }
        // break;
        //   case 0x4:
        0x4 => {
            mem_op_1000_179c(0xf6, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            struct_1020_7554(
                puVar1,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
            );
        }
        // break;
        //   case 0x5:
        0x5 => {
            mem_op_1000_179c(0xf8, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = in_edx & 0xffff0000 | uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            struct_1018_5840(
                uVar4,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
                in_stack_0000fe78,
                in_stack_0000ff9c,
                in_stack_0000ffa2,
                in_stack_0000ffa6,
            );
            puVar1 = uVar4;
        }
        // break;
        //   case 0x6:
        0x6 => {
            mem_op_1000_179c(0xf6, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            struct_1020_2524(
                puVar1,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
            );
        }
        // break;
        //   case 0x7:
        0x7 => {
            mem_op_1000_179c(0x118, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            unk_draw_op_1020_41c8(CONCAT22(in_edx, param_1), (var3 + 0xcc), var3, 0x1020);
        }
        // break;
        //   case 0x8:
        0x8 => {
            mem_op_1000_179c(0xf6, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            pass1_1018_e5dc(
                puVar1,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
            );
        }
        // break;
        //   case 0x9:
        0x9 => {
            mem_op_1000_179c(0xf6, in_edx);
            puVar1 = (in_edx | param_1);
            uVar4 = ZEXT24(puVar1);
            // if (puVar1.is_null()) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            struct_1018_66cc(
                puVar1,
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                var3,
                uVar5,
            );
        }
        // break;
        //   case 0xa:
        0xa => {
            win_1008_5c5c(param_1, in_edx, _u16_1050_02a0, 0x1d3);
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6d02(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0xb:
        0xb => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6d38(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0xc:
        0xc => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6d6e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0xd:
        0xd => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6da4(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0xe:
        0xe => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6dda(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0xf:
        0xf => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6e10(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x10:
        0x10 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6e46(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x11:
        0x11 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6e7c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x12:
        0x12 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6eb2(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x13:
        0x13 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6ee8(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x14:
        0x14 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6f1e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x15:
        0x15 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6f54(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x16:
        0x16 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6f8a(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x17:
        0x17 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6fc0(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x18:
        0x18 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_6ff6(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x19:
        0x19 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_702c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x1a:
        0x1a => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7062(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x1b:
        0x1b => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7098(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x1c:
        0x1c => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_70ce(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x1d:
        0x1d => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7104(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x1e:
        0x1e => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_713a(
                CONCAT22(in_edx, param_1),
                (var3 + 0xcc),
                param_2,
                0x1050,
            );
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x20:
        0x20 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7170(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x21:
        0x21 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_745e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x22:
        0x22 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_71a6(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x23:
        0x23 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_71dc(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x24:
        0x24 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7212(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x25:
        0x25 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_c958(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x26:
        0x26 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_c9a6(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x27:
        0x27 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_c9f4(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x28:
        0x28 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_ca48(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x29:
        0x29 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_ca96(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x2a:
        0x2a => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_caea(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x2b:
        0x2b => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cb38(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x2c:
        0x2c => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cb86(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x2d:
        0x2d => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cbda(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        //     break;
        //   case 0x2e:
        0x2e => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cc28(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x2f:
        0x2f => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cc76(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x30:
        0x30 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_ccc4(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x31:
        0x31 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cd12(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x32:
        0x32 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cd60(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x33:
        0x33 => {
            mem_op_1000_179c(0x114, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_cf74(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x34:
        0x34 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_73c2(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x35:
        0x35 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7494(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x36:
        0x36 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_74ca(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x37:
        0x37 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7500(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x38:
        0x38 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_73f8(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x39:
        0x39 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7536(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x3a:
        0x3a => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_756c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x3b:
        0x3b => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_75a2(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x3c:
        0x3c => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = pass1_1018_75d8(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x3d:
        0x3d => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_760e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x3e:
        0x3e => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7644(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x3f:
        0x3f => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_767a(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x40:
        0x40 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_76b0(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x41:
        0x41 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_76e6(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x42:
        0x42 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_771c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x43:
        0x43 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7752(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x44:
        0x44 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7788(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x45:
        0x45 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_77be(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x46:
        0x46 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_77f4(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x47:
        0x47 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_782a(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x48:
        0x48 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7860(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x49:
        0x49 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7896(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x4a:
        0x4a => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_78cc(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x4b:
        0x4b => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7902(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x4c:
        0x4c => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7938(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x4d:
        0x4d => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_796e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x4e:
        0x4e => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_79a4(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x4f:
        0x4f => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_79da(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x50:
        0x50 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7a10(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x51:
        0x51 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7a46(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x52:
        0x52 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7a7c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x54:
        0x54 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7ab2(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x55:
        0x55 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7ae8(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x56:
        0x56 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7b1e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x57:
        0x57 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7b54(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x58:
        0x58 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7b8a(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x59:
        0x59 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7bc0(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x5a:
        0x5a => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7bf6(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x5b:
        0x5b => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7c2c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x5c:
        0x5c => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7c62(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x5d:
        0x5d => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7c98(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x5e:
        0x5e => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7cce(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x5f:
        0x5f => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7d04(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x60:
        0x60 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7d3a(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x61:
        0x61 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7d70(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x62:
        0x62 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7248(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x63:
        0x63 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_727e(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x64:
        0x64 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_72b4(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x65:
        0x65 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_72ea(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x66:
        0x66 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7320(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x67:
        0x67 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            piVar1 = (var3 + 0xcc);
            *piVar1 = *piVar1 + 1;
            paVar2 = struct_1018_7356(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
        // break;
        //   case 0x68:
        0x68 => {
            mem_op_1000_179c(0xf2, in_edx);
            uVar3 = in_edx | param_1;
            uVar4 = uVar3;
            // if (uVar3 == 0) goto LAB_1008_169a;
            var_1 = (var3 + 0xcc);
            var_1 = var_1 + 1;
            paVar2 = struct_1018_738c(CONCAT22(in_edx, param_1), (var3 + 0xcc), param_2);
            puVar1 = (paVar2 >> 0x10);
            param_1 = paVar2;
        }
    };
    uStack6 = CONCAT22(puVar1, param_1); //
                                         // LAB_1008_2b3a:
    pass1_1008_6978(param_1, puVar1, param_2, 0x0, uStack6);
    return;
}
