use crate::util::{CARRY1, SCARRY1};

pub fn bad_fn_1138_0034() {
    let puVar1: *mut u8;
    let pcVar2: *mut char;
    let pbVar3: *mut u8;
    let piVar4: *mut i32;
    let puVar5: *mut u8;
    let mut uVar6: u16;
    let puVar7: *mut u32;
    let puVar8: *mut u16;
    let puVar9: *mut u32;
    let pcVar10: *mut char;
    let mut bVar11: u8;
    let mut cVar12: u8;
    let mut iVar13: i32;
    let lVar14;
    let mut in_AL: u8;
    let mut bVar15: u8;
    let mut bVar16: u8;
    let mut bVar17: u8;
    let mut cVar18: u8;
    let mut bVar19: u8;
    let mut cVar20: u8;
    let mut in_CL: u8;
    let mut in_CH: u8;
    let mut in_DX: i32;
    let mut uVar21: i32;
    let in_BX: *mut char;
    let mut uVar22: i32;
    let pcVar23: *mut char;
    let piVar24: *mut i32;
    let mut iVar25: i32;
    let unaff_SI: *mut u32;
    let puVar26: *mut u32;
    let unaff_DI: *mut char;
    let pcVar27: *mut char;
    let mut unaff_ES: u16;
    let mut unaff_CS: i32;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut in_GS: u16;
    let mut bVar28: bool;
    let acStack3: [u8; 2];
    let local_1: u8;

    pcVar2 = unaff_DI + (acStack3 + 1);
    unsafe {
        *pcVar2 = *pcVar2 + in_AL;
    }

    pcVar27 = unaff_DI + -1;
    uVar21 = in_DX + 1;
    if (uVar21 == 0) {
        if (uVar21 == 0) {
            pcVar2 = unaff_DI + 0x2fff;
            unsafe {
                *pcVar2 = *pcVar2 + in_CH;
            }
        } else {
            unsafe {
                puVar26 = (unaff_SI + 1);
                out(*unaff_SI, 0);
                pbVar3 = (in_BX + 0x66);
                bVar28 = CARRY1(*pbVar3, in_CL);
                *pbVar3 = *pbVar3 + in_CL;
                bVar15 = *pbVar3;
                if (bVar28) {
                    // code_r0x1138006f:
                    puVar8 = (puVar26 + 1);
                    out(*puVar26, uVar21);
                    puVar7 = (pcVar27 + 0x6c);
                    *puVar7 = *puVar7 + bVar28 * -(*puVar7 & 3);
                    pcVar2 = in_BX + puVar8;
                    *pcVar2 = *pcVar2 + unaff_CS;
                    bVar19 = (unaff_CS + 0x400 >> 8) + (uVar21 >> 8);
                    uVar22 = unaff_CS + 0x400 & 0xff;
                    unaff_CS = uVar22 | bVar19 << 8;
                    pcVar2 = in_BX;
                    *pcVar2 = *pcVar2 + in_BX;
                    pcVar2 = in_BX + 1;
                    bVar16 = uVar22;
                    *pcVar2 = *pcVar2 + bVar16;
                    pcVar2 = (register0x00000010 + -2 + puVar8);
                    *pcVar2 = *pcVar2 + bVar16;
                    pcVar2 = in_BX + puVar8;
                    *pcVar2 = *pcVar2;
                    pbVar3 = (in_BX + puVar8);
                    *pbVar3 = *pbVar3 ^ bVar16;
                    pcVar2 = (register0x00000010 + -2 + puVar8);
                    *pcVar2 = *pcVar2 + in_CL;
                    pcVar2 = (in_BX + puVar8 + 0x901);
                    *pcVar2 = *pcVar2 + in_CL;
                    pbVar3 = (register0x00000010 + -2 + puVar8);
                    bVar15 = *pbVar3;
                    *pbVar3 = *pbVar3 + bVar16;
                    pbVar3 = &stack0x0063 + puVar8;
                    bVar17 = *pbVar3;
                    bVar11 = *pbVar3;
                    *pbVar3 = bVar11 + 0x73 + CARRY1(bVar15, bVar16);
                    puVar9 = (puVar26 + 3);
                    out(*puVar8, uVar21);
                    if (*pbVar3 != 0) {
                        return;
                    }
                    puVar7 = (pcVar27 + 0x73);
                    *puVar7 = *puVar7
                        + (0x8c < bVar17 || CARRY1(bVar11 + 0x73, CARRY1(bVar15, bVar16)))
                            * -(*puVar7 & 3);
                    pcVar2 = in_BX + puVar9;
                    *pcVar2 = *pcVar2 + bVar16;
                    pcVar2 = in_BX + puVar9;
                    *pcVar2 = *pcVar2 + bVar19;
                    pcVar2 = (register0x00000010 + -2 + puVar9);
                    *pcVar2 = *pcVar2 + bVar19;
                    pcVar2 = (register0x00000010 + -2 + puVar9);
                    *pcVar2 = *pcVar2 + in_CL;
                    puVar7 = puVar26 + 0x482;
                    *puVar7 = *puVar7 + in_CL;
                    pcVar2 = in_BX + puVar9;
                    *pcVar2 = *pcVar2 + bVar16;
                    pbVar3 = (in_BX + pcVar27 + 0x73);
                    *pbVar3 = *pbVar3 + 0x73;
                    bVar15 = *pbVar3;
                    uVar21 = unaff_CS;
                    puVar26 = puVar9;
                } else {
                    if (!bVar28) {
                        pcVar2 = in_BX + puVar26;
                        *pcVar2 = *pcVar2 + unaff_CS;
                        unaff_CS = unaff_CS & (in_BX + puVar26);
                        pcVar2 = 0x200;
                        *pcVar2 = *pcVar2 + in_CL;
                        pcVar2 = in_BX + (unaff_DI + 1);
                        bVar15 = unaff_CS;
                        *pcVar2 = *pcVar2 + bVar15;
                        pcVar2 = unaff_DI + 1 + (acStack3 + 1);
                        *pcVar2 = *pcVar2 + bVar15;
                        uVar21 = in_DX + 2;
                        if (uVar21 != 0) {
                            // goto code_r0x113800dd;
                        }

                        if (uVar21 == 0) {
                            0x8350 = unaff_CS;
                            // goto code_r0x113800dd;
                        }
                        puVar7 = puVar26;
                        puVar26 = unaff_SI + 1;
                        out(*puVar7, 0);
                        pbVar3 = &stack0x005f + unaff_DI;
                        bVar28 = CARRY1(*pbVar3, bVar15);
                        *pbVar3 = *pbVar3 + bVar15;
                        pcVar27 = unaff_DI;
                        // goto code_r0x1138006f;
                    }
                }
                unaff_SI = puVar26;
                cVar18 = unaff_CS;
                if (bVar15 == 0) {
                    pcVar2 = (register0x00000010 + -2 + unaff_SI);
                    *pcVar2 = *pcVar2 + cVar18;
                    pcVar2 = in_BX + unaff_SI;
                    *pcVar2 = *pcVar2;
                    pcVar2 = 0x900;
                    *pcVar2 = *pcVar2 + cVar18;
                    // goto code_r0x1138013a;
                }
                pcVar2 = in_BX + unaff_SI;
                *pcVar2 = *pcVar2 + cVar18;
                puVar7 = unaff_SI;
                unaff_SI = unaff_SI + 1;
                unaff_CS = *puVar7;
                pcVar2 = in_BX + unaff_SI;
                *pcVar2 = *pcVar2 + (uVar21 >> 8);
            }
        }
    } else {
        unsafe {
            pbVar3 = (in_BX + unaff_SI);
            *pbVar3 = *pbVar3 ^ unaff_CS;
            unaff_CS = (in_BX + pcVar27);
            puVar7 = (in_BX + unaff_SI);
            *puVar7 = *puVar7 | unaff_CS;
            pcVar2 = (in_BX + unaff_SI + -0x80);
            *pcVar2 = *pcVar2 + uVar21;
            puVar7 = unaff_SI;
            unaff_SI = unaff_SI + 1;
            out(*puVar7, uVar21);
        }
    }
    pcVar2 = (register0x00000010 + -2 + unaff_SI);
    cVar20 = (unaff_CS >> 8);
    unsafe {
        *pcVar2 = *pcVar2 + cVar20;
        pcVar2 = (register0x00000010 + -2 + unaff_SI);
        *pcVar2 = *pcVar2 + in_CL;
        pcVar2 = &stack0x08ff + pcVar27;
        *pcVar2 = *pcVar2 + in_CL;
        pbVar3 = (in_BX + unaff_SI);
        bVar15 = *pbVar3;
        bVar17 = unaff_CS;
        *pbVar3 = *pbVar3 + bVar17;
        pcVar2 = (in_BX + unaff_SI + 0x65);
        cVar18 = *pcVar2;
        cVar12 = *pcVar2;
        *pcVar2 = cVar12 + 'o' + CARRY1(bVar15, bVar17);
    }
    if (SCARRY1(cVar18, 'o') == SCARRY1(cVar12 + 'o', CARRY1(bVar15, bVar17))) {
        pcVar2 = in_BX + unaff_SI;
        unsafe {
            *pcVar2 = *pcVar2 + bVar17;
            in_BX = in_BX + -1;
            pcVar2 = in_BX + pcVar27;
            *pcVar2 = *pcVar2 + in_CL;
            unaff_DI = pcVar27 + 2;
            pcVar2 = unaff_DI + (acStack3 + 1);
            *pcVar2 = *pcVar2 + in_CL;
            piVar4 = (in_BX + unaff_DI);
            *piVar4 = *piVar4 + unaff_CS;
            pcVar2 = &stack0x834e + unaff_DI;
            *pcVar2 = *pcVar2 + cVar20;
            puVar26 = unaff_SI;
            // code_r0x113800dd:
            pcVar2 = in_BX + puVar26;
            cVar18 = unaff_CS;
            *pcVar2 = *pcVar2 + cVar18;
            pcVar2 = (in_BX + unaff_DI + 0x1f00);
            *pcVar2 = *pcVar2 + (unaff_CS >> 8);
            pcVar2 = in_BX + puVar26;
            *pcVar2 = *pcVar2 + in_CL;
            uVar22 = in_BX & 0xff | ((in_BX >> 8) * 0x2) << 8;
            piVar4 = (uVar22 + puVar26);
            *piVar4 = *piVar4 + 1;
            puVar1 = &local_1 + puVar26;
            *puVar1 = *puVar1 + cVar18;
            pcVar2 = unaff_DI + uVar22 + 0x76;
            *pcVar2 = *pcVar2 + 'a';
            iVar13 = (puVar26 + 0x61);
            pcVar2 = (uVar22 + puVar26);
            *pcVar2 = *pcVar2 + cVar18;
            uVar6 = 0x3000;
            pcVar2 = (uVar22 + puVar26);
            *pcVar2 = *pcVar2 + in_CL;
            pcVar2 = unaff_DI + uVar22 + 0x201;
            *pcVar2 = *pcVar2 + uVar21;
            puVar7 = puVar26 + iVar13 * 0x3631;
            cVar18 = uVar6;
            *puVar7 = *puVar7 + cVar18;
            puVar5 = (uVar22 + puVar26);
            *puVar5 = *puVar5;
            pcVar2 = unaff_DI;
            *pcVar2 = *pcVar2 + cVar18;
        }
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    pcVar2 = in_BX + pcVar27;
    unsafe {
        *pcVar2 = *pcVar2 + in_CL;
    }
    // code_r0x1138013a:
    pcVar2 = in_BX + 1;
    unsafe {
        *pcVar2 = *pcVar2 + in_CL;
        pcVar2 = in_BX + unaff_SI;
        bVar15 = unaff_CS;
        *pcVar2 = *pcVar2 + bVar15;
    }
    lVar14 = (&stack0x006f + pcVar27) * 0x536f;
    iVar25 = lVar14;
    if (iVar25 != lVar14) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    unsafe { out(*unaff_SI, uVar21) };
    pcVar23 = in_BX + 1;
    out((unaff_SI + 1), uVar21);
    out(*(unaff_SI + 3), uVar21);
    if (pcVar23 == 0x0) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    puVar26 = unaff_SI + 3;
    out(unaff_SI[2], uVar21);
    pcVar10 = pcVar27 + 1;
    cVar18 = _in(uVar21);
    unsafe {
        *pcVar27 = cVar18;
        pcVar2 = pcVar23 + puVar26;
        *pcVar2 = *pcVar2 + bVar15;
        pcVar2 = acStack3 + pcVar10;
        cVar18 = (pcVar23 >> 8);
        *pcVar2 = *pcVar2 + cVar18;
        pcVar2 = (pcVar23 + pcVar10 + -0x7900);
        *pcVar2 = *pcVar2 + uVar21;
        pcVar2 = acStack3 + puVar26;
        *pcVar2 = *pcVar2 + (uVar21 >> 8);
        piVar24 = (pcVar23 & 0xff | (cVar18 * 0x2) << 8);
        piVar4 = piVar24;
        *piVar4 = *piVar4 + 1;
        pbVar3 = (piVar24 + puVar26);
        bVar17 = *pbVar3;
        *pbVar3 = *pbVar3 + bVar15;
        (iVar25 + -2) = unaff_CS;
        pcVar2 = (piVar24 + pcVar10 + 0x75);
        *pcVar2 = *pcVar2 + 'a' + CARRY1(bVar17, bVar15);
        out(*puVar26, uVar21);
        if (*pcVar2 == '\0') {
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
        if (*pcVar2 == '\0') {
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
        pcVar2 = (piVar24 + (unaff_SI + 7));
        *pcVar2 = *pcVar2 + bVar15;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

fn _in(u_var21: i32) -> u8 {
    todo!()
}

fn out(unaff_s_i: u32, arg: i32) -> () {
    todo!()
}

pub fn bad_fn_1050_525e() {
    let pbVar1: *mut byte;
    let puVar2: *mut u8;
    let puVar3: *mut u32;
    let piVar4: *mut i32;
    let puVar5: *mut u32;
    let pcVar6: *mut char;
    let puVar7: *mut u32;
    let uVar8: u8;
    let lVar9;
    let mut uVar10: u32;
    let mut bVar11: u8;
    let mut cVar12: u8;
    let mut bVar13: u8;
    let mut bVar14: u8;
    let mut iVar15: i32;
    let mut uVar16: i32;
    let mut cVar23: u8;
    let mut uVar17: i32;
    let mut bVar24: u8;
    let mut uVar18: i32;
    let mut uVar19: i32;
    let mut in_EAX: u32;
    let mut uVar20: u32;
    let mut uVar21: u32;
    let pcVar22: *mut char;
    let mut in_CX: i32;
    let mut iVar25: i32;
    let piVar26: *mut i32;
    let mut uVar27: i32;
    let mut uVar28: i32;
    let mut cVar29: u8;
    let mut bVar30: u8;
    let mut uVar31: i32;
    let mut uVar32: u16;
    let mut in_EDX: u32;
    let mut cVar36: u8;
    let mut uVar33: u32;
    let mut bVar37: u8;
    let mut uVar34: u32;
    let mut uVar35: u32;
    let mut bVar38: u8;
    let mut bVar39: u8;
    let in_BX: *mut i32;
    let piVar40: *mut i32;
    let mut bVar43: u8;
    let piVar41: *mut i32;
    let mut uVar42: i32;
    let puVar44: *mut u8;
    let puVar45: *mut u16;
    let ppuVar46: *mut *mut u32;
    let ppuVar47: *mut *mut u32;
    let ppuVar48: *mut *mut u32;
    let mut uVar49: i32;
    let mut in_EBP: u32;
    let mut uVar50: u32;
    let mut iVar51: i32;
    let piVar52: *mut i32;
    let piVar53: *mut i32;
    let pcVar54: *mut char;
    let puVar55: *mut u16;
    let puVar56: *mut u8;
    let puVar57: *mut u16;
    let puVar58: *mut u32;
    let puVar59: *mut u8;
    let mut in_ESI: u32;
    let puVar60: *mut u16;
    let mut unaff_DI: i32;
    let piVar61: *mut i32;
    let piVar62: *mut i32;
    let mut uVar63: i32;
    let puVar64: *mut u32;
    let puVar65: *mut u32;
    let mut unaff_ES: i32;
    let mut uVar66: u16;
    let mut uVar67: u16;
    let mut unaff_SS: u16;
    let unaff_DS: *mut i32;
    let mut in_FS: u16;
    let mut in_GS: u16;
    let mut in_AF: u8;
    let mut bVar68: bool;
    let mut bVar69: bool;
    let mut bVar70: bool;
    let mut in_stack_0000a0fe: i32;
    let mut in_stack_0000a100: i32;
    let in_stack_0000a104: *mut i32;
    let mut in_stack_0000a106: i32;
    let mut in_stack_0000a108: u16;
    let mut in_stack_0000a10a: i32;
    let in_stack_0000a10c: *mut i32;
    let mut in_stack_0000a10e: i32;
    let mut in_stack_0000a110: i32;
    let in_stack_0000a114: *mut i32;
    let mut in_stack_0000a116: i32;
    let mut in_stack_0000a118: u16;
    let mut in_stack_0000a11a: i32;
    let in_stack_0000a11c: *mut i32;
    let in_stack_0000a11e: *mut i32;
    let mut in_stack_0000a120: i32;
    let in_stack_0000a124: *mut i32;
    let mut in_stack_0000a126: i32;
    let mut in_stack_0000a128: u16;
    let in_stack_0000a12a: *mut i32;
    let mut uStack2: u16;

    piVar26 = &uStack2;
    uVar50 = in_EBP & 0xffff0000;
    uVar35 = uVar50 | ZEXT24(&uStack2);
    iVar25 = in_CX + -1;
    if (iVar25 != 0) {
        piVar4 = in_BX;
        unsafe {
            *piVar4 = *piVar4 + 1;
        }

        in_stack_0000a0fe = unaff_ES;
    }
    iVar51 = in_ESI;
    pcVar22 = (in_BX + iVar51);
    cVar12 = in_EAX;
    unsafe {
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (in_BX + iVar51);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (in_BX + iVar51);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (unaff_DI + 0x205);
        *pcVar22 = *pcVar22 + (iVar25 >> 8);
        pcVar22 = (&uStack2 + iVar51);
        cVar36 = (in_EDX >> 8);
        *pcVar22 = *pcVar22 + cVar36;
        pbVar1 = (&uStack2 + iVar51);
        bVar11 = *pbVar1;
        bVar38 = in_BX;
        *pbVar1 = *pbVar1 + bVar38;
        bVar11 = cVar12 + CARRY1(bVar11, bVar38);
        pbVar1 = (in_BX + iVar51);
        *pbVar1 = *pbVar1 | bVar11;
        pcVar22 = (in_BX + iVar51);
        *pcVar22 = *pcVar22 + bVar11;
        pcVar22 = (in_BX + iVar51);
        *pcVar22 = *pcVar22 + bVar11;
        uVar10 = in_EAX & 0xffff0000;
        iVar15 = (in_BX + iVar51) * -10 + (in_BX + iVar51);
        pcVar22 = (in_BX + iVar51);
        cVar12 = iVar15;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (in_BX + iVar51);
        *pcVar22 = *pcVar22 + cVar12;
        piVar61 = (unaff_DI + 3);
        uVar16 = iVar15 + 2;
        piVar4 = piVar61;
        cVar12 = uVar16;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (in_BX + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (in_BX + 0x7405);
        *pcVar22 = *pcVar22 + cVar36;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + (in_BX >> 8);
        cVar29 = in_EDX;
        uVar34 = in_EDX & 0xffff0000;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + 1;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (in_BX + piVar61 + 0x105);
        *pcVar22 = *pcVar22 + cVar36 * 0x2;
        pcVar22 = 0xaf00;
        bVar24 = iVar25;
        *pcVar22 = *pcVar22 + bVar24;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + cVar29;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        cVar23 = (uVar16 >> 8);
        cVar36 = cVar36 * 0x2 + cVar23;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + bVar24;
        uVar67 = CONCAT11(0xff, bVar24);
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = &stack0x0103 + piVar61;
        *pcVar22 = *pcVar22 + cVar36;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + cVar29;
        piVar4 = in_BX;
        *piVar4 = *piVar4 + 0x7300;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = 0x7405;
        *pcVar22 = *pcVar22 + cVar36;
        piVar4 = in_BX;
        *piVar4 = *piVar4 + bVar38;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + -1;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (unaff_DI + 0x108);
        *pcVar22 = *pcVar22 + cVar36;
        cVar23 = cVar23 + cVar29;
        piVar4 = in_BX;
        *piVar4 = *piVar4 + cVar36;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + 1;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = in_BX + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = 0x6401;
        *pcVar22 = *pcVar22 + -1;
        bVar38 = bVar38 + cVar29;
        piVar40 = (in_BX & 0xff00 | bVar38);
        piVar4 = piVar61;
        *piVar4 = *piVar4 + 1;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + -1;
        cVar12 = cVar12 + -1;
        uVar33 = uVar34 | in_stack_0000a0fe;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar38;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = 0x100;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = 0x100;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = 0x700;
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar24;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar24;
        cVar12 = cVar12 + in_stack_0000a0fe;
        piVar4 = (piVar40 + piVar61);
        *piVar4 = *piVar4 + CONCAT11(cVar23, cVar12);
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + bVar38;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        bVar14 = (in_stack_0000a0fe >> 8);
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + cVar12 + bVar14;
        uVar16 = CONCAT11(cVar23, cVar12 + bVar14) + piVar40[0x3980];
        uVar16 = uVar16 & 0xff00 | (uVar16 + *(piVar40 + 0x3980));
        puVar5 = (piVar40 + 0x3980);
        uVar17 = uVar16 + *puVar5;
        bVar11 = (uVar17 - *(piVar40 + 0x3980)) - CARRY2(uVar16, *puVar5);
        uVar16 = uVar17 & 0xff00 | bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar11;
        uVar16 = uVar16 + 0x700;
        bVar13 = uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = 0x0;
        *pcVar22 = *pcVar22 + bVar38;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + in_stack_0000a0fe;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar24;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + bVar24;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar38;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = 0x7300;
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar40 + 0x3980;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar14;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16 + CARRY1(bVar11, bVar14);
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar38;
        pcVar22 = 0x0;
        *pcVar22 = *pcVar22 + bVar24;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar40 + 0x3980;
        *piVar4 = *piVar4 + bVar13;
        bVar13 = bVar13 | *(piVar40 + 0x3980);
        bVar11 = (uVar16 >> 8);
        piVar52 = 0x7301;
        piVar41 = 0x7301;
        out(*0x7300, in_stack_0000a0fe);
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar24;
        pcVar22 = 0x0;
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar24;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + ((in_BX & 0xff00) >> 8);
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = 0x300;
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = 0x7301;
        *pcVar22 = *pcVar22 + bVar13;
        iVar25 = CONCAT11(bVar11 + bVar24, bVar13);
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar24);
        pcVar22 = 0x7301;
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = (piVar40 + 0x7301);
        *piVar4 = *piVar4 + iVar25;
        piVar4 = (piVar40 + 0x7301);
        *piVar4 = *piVar4 + iVar25;
        piVar4 = (piVar40 + 0x7301);
        *piVar4 = *piVar4 + iVar25;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = (piVar40 + 0x7301);
        *piVar4 = *piVar4 + iVar25;
        uVar16 = iVar25 + 0x500;
        pcVar22 = (piVar40 + piVar61);
        cVar12 = uVar16;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        bVar11 = (uVar16 >> 8);
        uVar21 = (uVar16 & 0xff | (bVar11 + bVar24) << 8);
        uVar20 = uVar10 | uVar21;
        pcVar22 = (piVar40 + 0x7301);
        cVar12 = (uVar16 & 0xff);
        *pcVar22 = *pcVar22 + cVar12 + CARRY1(bVar11, bVar24);
        piVar4 = piVar61;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = piVar61;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (&uStack2 + piVar61);
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar40 + 0x7301);
        *pcVar22 = *pcVar22 + cVar12;
        pbVar1 = (piVar40 + 0x65);
        bVar11 = *pbVar1;
        bVar13 = (uVar21 >> 8);
        bVar70 = SCARRY1(*pbVar1, bVar13);
        *pbVar1 = *pbVar1 + bVar13;
        if (*pbVar1 == 0) {
            puVar60 = (in_ESI & 0xffff0000 | ZEXT24(&uStack2));
            in_stack_0000a114 = in_stack_0000a104;
            // code_r0x10505706:
            uVar35 = uVar50 | in_stack_0000a100;
            uVar33 = uVar34 | in_stack_0000a106;
            uVar20 = uVar10 | in_stack_0000a10a;
            piVar41 = (puVar60 + 2);
            out(*puVar60, in_stack_0000a106);
            if (bVar70) {
                // code_r0x10505782:
                in_stack_0000a124 = in_stack_0000a114;
                in_stack_0000a128 = in_stack_0000a108;
                in_stack_0000a11c = (piVar26 + 1);
                uVar8 = _in(uVar33);
                *piVar26 = uVar8;
                piVar4 = in_stack_0000a124 + 0x33;
                *piVar4 = *piVar4 & (in_stack_0000a128 >> 8);
                piVar4 = in_stack_0000a11c;
                *piVar4 = *piVar4 & (uVar20 >> 8);
                piVar52 = piVar41;
                // code_r0x105057ab:
                piVar4 = in_stack_0000a11c;
                bVar11 = (uVar20 >> 8);
                *piVar4 = *piVar4 & bVar11;
                uVar8 = _in(uVar33);
                *in_stack_0000a11c = uVar8;
                piVar4 = in_stack_0000a124 + 0x33;
                *piVar4 = *piVar4 & (in_stack_0000a128 >> 8);
                piVar4 = (in_stack_0000a11c + 1);
                *piVar4 = *piVar4 & bVar11;
                uVar67 = in_stack_0000a128;
                piVar40 = in_stack_0000a124;
                piVar62 = (in_stack_0000a11c + 1);
                // goto code_r0x105057d9;
            }
            uVar35 = uVar50 | in_stack_0000a120;
            uVar33 = uVar34 | in_stack_0000a126;
            uVar50 = ZEXT24(in_stack_0000a12a);
            piVar61 = (in_stack_0000a11c + 1);
            piVar26 = (in_stack_0000a11e + 1);
            out(*in_stack_0000a11e, in_stack_0000a126);
            piVar40 = in_stack_0000a124;
            if (-1 < in_stack_0000a124) {
                pbVar1 = (in_stack_0000a124 + piVar26);
                *pbVar1 = *pbVar1 ^ (in_stack_0000a124 >> 8);
                uVar8 = _in(in_stack_0000a126);
                *piVar61 = uVar8;
                piVar40 = (in_stack_0000a124 + 1);
                piVar61 = (in_stack_0000a11c + 3);
                uVar8 = _in(in_stack_0000a126);
                *(in_stack_0000a11c + 1) = uVar8;
                puVar60 = (in_stack_0000a11e + 0x21);
                *puVar60 = *puVar60;
                bVar11 = in_stack_0000a12a & 100;
                pcVar22 = (piVar40 + piVar26);
                *pcVar22 = *pcVar22 + bVar11;
                pcVar22 = (piVar40 + piVar26);
                *pcVar22 = *pcVar22 + bVar11;
                uVar50 = uVar50 & 0xffff0060;
                pcVar22 = (piVar40 + piVar26);
                cVar12 = uVar50;
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar26);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar26);
                *pcVar22 = *pcVar22 + cVar12;
                in_stack_0000a11c = piVar40;
                in_stack_0000a11e = in_stack_0000a124;
            }
            uVar20 = uVar10 | uVar50;
            pcVar22 = (piVar40 + piVar26);
            cVar12 = uVar50;
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + piVar26);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + piVar26);
            *pcVar22 = *pcVar22 + cVar12;
            uVar67 = in_stack_0000a128;
            in_stack_0000a12a = in_stack_0000a124;
            // PTR_LOOP_1050_574c:
            piVar41 = piVar26;
            pcVar22 = (piVar40 + piVar41);
            bVar11 = uVar20;
            *pcVar22 = *pcVar22 + bVar11;
            pcVar22 = (piVar40 + piVar41);
            *pcVar22 = *pcVar22 + bVar11;
            pcVar22 = (piVar40 + piVar41);
            *pcVar22 = *pcVar22 + bVar11;
            pcVar22 = (piVar40 + piVar41);
            *pcVar22 = *pcVar22 + bVar11;
            pbVar1 = (piVar40 + piVar41);
            bVar70 = CARRY1(*pbVar1, bVar11);
            *pbVar1 = *pbVar1 + bVar11;
            bVar68 = *pbVar1 == 0;
            // code_r0x10505758:
            cVar12 = uVar20;
            piVar52 = piVar41;
            if (!bVar70) {
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                // goto code_r0x105057d3;
            }
            piVar62 = piVar61 + 1;
            uVar66 = uVar33;
            iVar25 = _in(uVar66);
            *piVar61 = iVar25;
            if (bVar70) {
                // code_r0x105057c2:
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                // goto code_r0x105057c6;
            }
            if (bVar70) {
                piVar52 = piVar41 + 1;
                out(*piVar41, uVar66);
                if (!bVar68) {
                    // goto code_r0x105057d6;
                }
                piVar4 = piVar62;
                bVar11 = *piVar4;
                bVar13 = (uVar20 >> 8);
                *piVar4 = *piVar4 + bVar13;
                cVar23 = *piVar4;
                bVar70 = *piVar4 == '\0';
                piVar26 = in_stack_0000a11c;
                piVar41 = in_stack_0000a11e;
                if (!CARRY1(bVar11, bVar13)) {
                    // goto code_r0x10505770;
                }
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                cVar23 = *pcVar22;
                bVar70 = *pcVar22 == '\0';
                if (!bVar70) {
                    // goto code_r0x1050576e;
                }
                // goto code_r0x105057e0;
            }
            piVar52 = piVar41 + 1;
            out(*piVar41, uVar66);
            piVar4 = piVar62;
            piVar62 = piVar61 + 2;
            iVar25 = _in(uVar66);
            *piVar4 = iVar25;
            pbVar1 = (piVar40 + piVar52);
            bVar11 = (uVar33 >> 8);
            *pbVar1 = *pbVar1 & bVar11;
            bVar13 = (piVar40 >> 8);
            if (-1 < *pbVar1) {
                pbVar1 = (piVar40 + piVar52);
                *pbVar1 = *pbVar1 ^ bVar13;
                bVar68 = *pbVar1 < '\0';
                bVar70 = *pbVar1 == 0;
                piVar4 = piVar62;
                piVar62 = (piVar61 + 5);
                uVar8 = _in(uVar66);
                *piVar4 = uVar8;
                if (!bVar70) {
                    // goto code_r0x1050579a;
                }

                // goto code_r0x1050580c;
            }
            pbVar1 = (piVar41 + 0x71);
            *pbVar1 = *pbVar1 & bVar11;
            pbVar1 = (piVar40 + piVar52);
            *pbVar1 = *pbVar1 & bVar11;
            if (-1 < *pbVar1) {
                pbVar1 = (piVar40 + piVar52);
                *pbVar1 = *pbVar1 ^ bVar13;
                piVar4 = piVar62;
                piVar62 = (piVar61 + 5);
                uVar8 = _in(uVar66);
                *piVar4 = uVar8;
                // goto code_r0x105057c2;
            }
            // code_r0x105057e4:
            pcVar22 = (piVar40 + piVar52);
            cVar12 = uVar20;
            *pcVar22 = *pcVar22 + cVar12;
            piVar4 = piVar52;
            cVar23 = (uVar20 >> 8);
            *piVar4 = *piVar4 + cVar23;
            piVar4 = piVar52;
            *piVar4 = *piVar4 + cVar23;
            pcVar22 = (piVar40 + piVar52);
            *pcVar22 = *pcVar22 - cVar12;
            pcVar22 = (piVar40 + piVar52);
            *pcVar22 = *pcVar22 - cVar12;
            piVar52 = (piVar52 + 1);
            pcVar22 = (uVar35 + piVar52);
            cVar12 = (uVar33 >> 8);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (uVar35 + piVar52);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = 0x1e00;
            *pcVar22 = *pcVar22 + piVar40;
            pcVar22 = 0x0;
            *pcVar22 = *pcVar22 + piVar40;
            // code_r0x105057ff:
            piVar4 = piVar52;
            *piVar4 = *piVar4 + (uVar20 >> 8);
            pcVar22 = (piVar40 + piVar52);
            cVar12 = uVar20;
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + piVar52);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + piVar52);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + piVar52);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + piVar52);
            *pcVar22 = *pcVar22 + cVar12;
        } else {
            piVar26 = 0x7301;
            if (!CARRY1(bVar11, bVar13)) {
                // code_r0x1050574d:
                piVar41 = piVar26;
                pcVar22 = (piVar40 + piVar41);
                cVar12 = uVar20;
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar41);
                *pcVar22 = *pcVar22 + cVar12;
                piVar4 = piVar40;
                bVar11 = (uVar33 >> 8);
                bVar70 = CARRY1(*piVar4, bVar11);
                *piVar4 = *piVar4 + bVar11;
                bVar68 = *piVar4 == '\0';
                // goto code_r0x10505758;
            }
            if (*pbVar1 == 0) {
                // code_r0x1050574b:
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + uVar20;
                piVar26 = piVar52;
                // goto code_r0x1050574d;
            }
            pbVar1 = 0x7362;
            *pbVar1 = *pbVar1 & bVar13;
            if (*pbVar1 == 0) {
                // goto PTR_LOOP_1050_574c;
            }
            pbVar1 = (piVar40 + 0x69);
            bVar70 = false;
            *pbVar1 = *pbVar1 & bVar14;
            bVar68 = *pbVar1 == 0;
            if (bVar68) {
                // goto code_r0x10505758;
            }
            piVar52 = 0x7303;
            out(0x7301, in_stack_0000a0fe);
            if (bVar68) {
                pbVar1 = (piVar40 + 0x736f);
                *pbVar1 = *pbVar1 & bVar14;
                uVar35 = uVar50 | in_stack_0000a100;
                uVar33 = uVar34 | in_stack_0000a106;
                uVar20 = uVar10 | in_stack_0000a10a;
                piVar52 = (&uStack2 + 1);
                out(uStack2, in_stack_0000a106);
                if (*pbVar1 != 0) {
                    pcVar22 = &stack0x006b + in_stack_0000a100;
                    cVar12 = (in_stack_0000a10a >> 8);
                    bVar70 = SCARRY1(*pcVar22, cVar12);
                    *pcVar22 = *pcVar22 + cVar12;
                    puVar60 = (in_ESI & 0xffff0000 | in_stack_0000a10e);
                    in_stack_0000a108 = in_stack_0000a118;
                    piVar26 = in_stack_0000a10c;
                    in_stack_0000a10a = in_stack_0000a11a;
                    in_stack_0000a106 = in_stack_0000a116;
                    in_stack_0000a100 = in_stack_0000a110;
                    // goto code_r0x10505706;
                }
                pcVar22 = (in_stack_0000a104 + piVar52);
                cVar12 = in_stack_0000a10a;
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (in_stack_0000a104 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (in_stack_0000a104 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (in_stack_0000a104 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (in_stack_0000a104 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (in_stack_0000a104 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                uVar67 = in_stack_0000a108;
                piVar40 = in_stack_0000a104;
                piVar61 = &uStack2;
                // goto code_r0x1050574b;
            }
            pcVar22 = (piVar40 + 0x7303);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar40 + 0x7303);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = 0x7375;
            *pcVar22 = *pcVar22 + bVar14;
            cVar23 = *pcVar22;
            bVar70 = *pcVar22 == '\0';
            piVar62 = piVar61;
            // code_r0x1050576e:
            piVar61 = piVar62;
            piVar26 = in_stack_0000a11c;
            piVar41 = in_stack_0000a11e;
            if (!bVar70) {
                // code_r0x105057d3:
                pcVar22 = (piVar40 + piVar52);
                cVar12 = uVar20;
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                piVar62 = piVar61;
                // code_r0x105057d9:
                pcVar22 = (piVar40 + piVar52);
                bVar11 = uVar20;
                *pcVar22 = *pcVar22 + bVar11;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + bVar11;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + bVar11;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + bVar11;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + bVar11;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + bVar11;
                piVar4 = piVar52;
                cVar12 = (uVar20 >> 8);
                *piVar4 = *piVar4 + cVar12;
                piVar4 = piVar52;
                *piVar4 = *piVar4 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                cVar12 = (uVar67 >> 8);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = uVar35;
                *pcVar22 = *pcVar22 + bVar11;
                bVar11 = bVar11 ^ *(piVar40 + piVar52) ^ *(piVar40 + piVar52);
                uVar20 = uVar20 & 0xffffff00 | bVar11;
                pcVar22 = 0x1e00;
                *pcVar22 = *pcVar22 + piVar40;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + bVar11;
                in_stack_0000a11c = unaff_DS;
                // goto code_r0x105057ff;
            }
            // code_r0x10505770:
            bVar68 = cVar23 < '\0';
            in_stack_0000a11c = piVar26;
            in_stack_0000a11e = piVar41;
            if (!bVar68) {
                pbVar1 = (piVar40 + piVar52);
                *pbVar1 = *pbVar1 ^ (piVar40 >> 8);
                bVar68 = *pbVar1 < '\0';
                bVar70 = *pbVar1 == 0;
                piVar4 = piVar62;
                piVar62 = (piVar62 + 1);
                uVar66 = uVar33;
                uVar8 = _in(uVar66);
                *piVar4 = uVar8;
                if (!bVar68) {
                    uVar8 = _in(uVar66);
                    *piVar62 = uVar8;
                    out(*piVar52, uVar66);
                    uVar35 = uVar35 & 0xffff0000 | in_stack_0000a120;
                    uVar33 = uVar33 & 0xffff0000 | in_stack_0000a126;
                    uVar20 = uVar20 & 0xffff0000 | ZEXT24(in_stack_0000a12a);
                    piVar4 = piVar26;
                    *piVar4 = *piVar4 & (ZEXT24(in_stack_0000a12a) >> 8);
                    in_stack_0000a108 = in_stack_0000a128;
                    in_stack_0000a114 = in_stack_0000a124;
                    // goto code_r0x10505782;
                }
            }
            // code_r0x1050579a:
            if (!bVar70) {
                // goto code_r0x105057ff;
            }
            if (bVar68) {
                // code_r0x105057c6:
                pcVar22 = (piVar40 + piVar52);
                cVar12 = uVar20;
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                // code_r0x105057d6:
                pcVar22 = (piVar40 + piVar52);
                cVar12 = uVar20;
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + cVar12;
                // code_r0x105057e0:
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + uVar20;
                pcVar22 = (piVar40 + piVar52);
                *pcVar22 = *pcVar22 + uVar20;
                // goto code_r0x105057e4;
            }
            pbVar1 = (piVar40 + piVar52);
            *pbVar1 = *pbVar1 ^ (piVar40 >> 8);
            bVar11 = *pbVar1;
            bVar13 = *pbVar1;
            piVar4 = piVar62;
            piVar62 = (piVar62 + 1);
            uVar66 = uVar33;
            uVar8 = _in(uVar66);
            *piVar4 = uVar8;
            if (bVar11 < '\0') {
                // goto code_r0x105057c6;
            }
            if (bVar13 == 0) {
                uVar8 = _in(uVar66);
                *piVar62 = uVar8;
                out(*piVar52, uVar66);
                uVar35 = uVar35 & 0xffff0000 | in_stack_0000a120;
                uVar33 = uVar33 & 0xffff0000 | in_stack_0000a126;
                uVar20 = uVar20 & 0xffff0000 | ZEXT24(in_stack_0000a12a);
                piVar52 = in_stack_0000a11e;
                // goto code_r0x105057ab;
            }
            in_AF = 9 < (uVar20 & 0xf) | in_AF;
            bVar11 = uVar20 + in_AF * '\x06';
            uVar20 = uVar20 & 0xffffff00
                | (bVar11 + (0x90 < (bVar11 & 0xf0) | in_AF * (0xf9 < bVar11)) * '`');
        }
        // code_r0x1050580c:
        pbVar1 = (piVar40 + piVar52);
        bVar11 = *pbVar1;
        bVar13 = uVar20;
        *pbVar1 = *pbVar1 + bVar13;
        uVar50 = (piVar40 + 0x28);
        uVar66 = (uVar50 >> 0x10);
        uVar50 = uVar50 & 0xffff;
        uVar34 = uVar33 & 0xffff0000 | uVar50;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar13);
        piVar4 = piVar40;
        cVar12 = (uVar20 >> 8);
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar62;
        bVar30 = uVar50;
        *piVar4 = *piVar4 + bVar30;
        iVar25 = uVar35;
        pcVar22 = (iVar25 + piVar52);
        bVar14 = uVar67;
        *pcVar22 = *pcVar22 + bVar14;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar14;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (iVar25 + piVar52);
        bVar43 = (piVar40 >> 8);
        *pcVar22 = *pcVar22 + bVar43;
        pcVar22 = (piVar40 + piVar62);
        bVar38 = (uVar67 >> 8);
        *pcVar22 = *pcVar22 + bVar38;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar38;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar14;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar14;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = 0x600;
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar52;
        bVar39 = piVar40;
        *piVar4 = *piVar4 + bVar39;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = 0xb00;
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar30;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar30;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar30;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar39;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar30;
        pcVar22 = 0x2600;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = 0xa00;
        bVar37 = (uVar50 >> 8);
        *pcVar22 = *pcVar22 + bVar37;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar14;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar14;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar39;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar39;
        piVar4 = piVar62;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar37;
        pcVar22 = 0x2600;
        *pcVar22 = *pcVar22 + bVar37;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar37;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar30;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        pcVar22 = 0xe00;
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = 0x3700;
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = 0x1a00;
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        piVar4 = piVar40;
        *piVar4 = *piVar4 + bVar30;
        pcVar22 = 0x1700;
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar39;
        pcVar22 = 0x1200;
        *pcVar22 = *pcVar22 + bVar14;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar40 + piVar52);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar52;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar30;
        uVar20 = uVar20 & 0xffff0000;
        pcVar22 = (piVar40 + piVar62);
        bVar13 = in_stack_0000a11c;
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar30);
        pbVar1 = 0x5058;
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar30;
        pcVar22 = (iVar25 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar30);
        piVar4 = piVar52;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar39;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar39);
        pbVar1 = 0x5058;
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar39;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar39);
        pbVar1 = 0x5058;
        bVar11 = *pbVar1;
        bVar24 = (ZEXT24(in_stack_0000a11c) >> 8);
        *pbVar1 = *pbVar1 + bVar24;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar24);
        pbVar1 = (piVar40 + piVar52);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar38;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar38);
        pbVar1 = (piVar40 + piVar52);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar37;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar37);
        pbVar1 = (iVar25 + piVar52);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar37;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar37);
        pbVar1 = (iVar25 + piVar52);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar43;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar43);
        pbVar1 = (piVar52 + iVar25 + 0x58);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar13);
        pbVar1 = (piVar52 + iVar25 + 0x58);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar14;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar14);
        piVar4 = piVar52 + 0x2c;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar14;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar14);
        piVar4 = piVar52 + 0x2c;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar30;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar30);
        piVar4 = piVar52 + 0x2c;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar39;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar39);
        piVar4 = piVar52 + 0x2c;
        bVar11 = *piVar4;
        bVar24 = (ZEXT24(in_stack_0000a11c) >> 8);
        *piVar4 = *piVar4 + bVar24;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar24);
        pbVar1 = (iVar25 + 0x58);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar24;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar24);
        pbVar1 = (iVar25 + 0x58);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar38;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar38);
        pbVar1 = (piVar40 + piVar52 + 0x58);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar37;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar37);
        pbVar1 = (piVar40 + piVar52 + 0x58);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar43;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar43);
        pbVar1 = (piVar40 + piVar52 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar13);
        pbVar1 = (piVar40 + piVar52 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar14;
        pcVar22 = (piVar40 + piVar62);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar14);
        pbVar1 = (piVar52 + iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar14;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar14);
        pbVar1 = (piVar52 + iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar30;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar30);
        pbVar1 = (piVar52 + iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar39;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar39);
        pbVar1 = (piVar52 + iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar24;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar24);
        pbVar1 = (piVar52 + iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar38;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar38);
        pbVar1 = (piVar52 + iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar37;
        pcVar22 = (iVar25 + piVar52);
        *pcVar22 = *pcVar22 + bVar13 + CARRY1(bVar11, bVar37);
        pbVar1 = (iVar25 + 0x5058);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar37;
        piVar4 = piVar52;
        *piVar4 = *piVar4 + bVar13 + CARRY1(bVar11, bVar37);
        pcVar22 = (piVar62 + iVar25 + 0x43);
        *pcVar22 = *pcVar22 + bVar30;
        piVar26 = ((piVar40 + piVar62 + 0x6e) * 0x7570);
        iVar25 = piVar52[0x36] * 0x6c42;
        uVar35 = uVar35 & 0xffff0000;
        bVar70 = in_stack_0000a11c < 0x303e;
        iVar15 = uVar50;
        if (-1 < (in_stack_0000a11c + -0x181f)) {
            pbVar1 = (piVar40 + piVar52);
            *pbVar1 = *pbVar1 ^ bVar43;
            uVar8 = _in(iVar15);
            *piVar62 = uVar8;
            iVar25 = piVar52[0x36];
            piVar61 = piVar62 + 1;
            uVar8 = _in(iVar15);
            *(piVar62 + 1) = uVar8;
            out(*piVar52, iVar15);
            out(*(piVar52 + 1), iVar15);
            pcVar22 = (piVar61 + iVar25 * 0x6f43 + 0x43);
            *pcVar22 = *pcVar22 + bVar30;
            lVar9 = (piVar52 + 0x6f) * 0x6552;
            iVar25 = lVar9;
            bVar70 = iVar25 != lVar9;
            out((piVar52 + 3), iVar15);
            piVar62 = (piVar62 + 3);
            uVar8 = _in(iVar15);
            *piVar61 = uVar8;
            puVar59 = (piVar52 + 7);
            out((piVar52 + 5), iVar15);
            piVar52 = piVar52 + 4;
            out(*puVar59, iVar15);
            in_stack_0000a104 = piVar40;
        }
        piVar61 = (piVar40 + 1);
        uVar50 = uVar35 | iVar25 - 1;
        out(*piVar52, iVar15);
        bVar11 = (in_stack_0000a11c >> 8);
        piVar41 = in_stack_0000a11c;
        if (bVar70 || iVar25 - 1 == 0) {
            uVar21 = (uVar20 | ZEXT24(in_stack_0000a11c + -0x181f)) ^ 0x756c;
            pbVar1 = (piVar62 + iVar25 + 0x6b);
            bVar13 = (uVar21 >> 8);
            bVar70 = CARRY1(*pbVar1, bVar13);
            *pbVar1 = *pbVar1 + bVar13;
            bVar68 = *pbVar1 < '\0';
            piVar53 = (piVar52 + 3);
            out(*(piVar52 + 1), iVar15);
            // code_r0x10505a1f:
            puVar5 = (piVar53 + 0x37);
            *puVar5 = *puVar5 + bVar70 * ((uVar50 & 3) - (*puVar5 & 3));
            if (!bVar68) {
                // code_r0x10505a44:
                pbVar1 = (piVar61 + piVar53);
                *pbVar1 = *pbVar1 & uVar21;
                // code_r0x10505a48:
                uVar21 = uVar21 & 0xffff2460;
                pbVar1 = (piVar61 + piVar53);
                *pbVar1 = *pbVar1 & uVar21;
                piVar62 = piVar41;
                // code_r0x10505a52:
                uVar50 = uVar50 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
                uVar34 = uVar34 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
                uVar21 = uVar21 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
                piVar61 = in_stack_0000a11c;
                piVar53 = in_stack_0000a11c;
                // goto code_r0x10505a76;
            }
            pbVar1 = (piVar61 + piVar53 + 0x25);
            bVar13 = (piVar61 >> 8);
            *pbVar1 = *pbVar1 ^ bVar13;
            pbVar1 = (piVar61 + piVar53);
            *pbVar1 = *pbVar1 ^ bVar13;
            bVar13 = *pbVar1;
            piVar40 = (piVar62 + 1);
            uVar8 = _in(uVar34);
            *piVar62 = uVar8;
            if (bVar13 < '\0') {
                pbVar1 = (piVar61 + piVar40 + 0x6c);
                bVar70 = CARRY1(*pbVar1, uVar21);
                *pbVar1 = *pbVar1 + uVar21;
            } else {
                pbVar1 = (piVar53 + 0x75);
                *pbVar1 = *pbVar1 & (uVar21 >> 8);
                // code_r0x10505a2f:
                iVar25 = _in(uVar34);
                *piVar40 = iVar25;
                uVar50 = uVar50 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
                uVar34 = uVar34 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
                pbVar1 = (in_stack_0000a11c * 2);
                *pbVar1 = *pbVar1 & bVar11;
                bVar70 = false;
                uVar21 = uVar21 & 0xffff0000 | ZEXT24(in_stack_0000a11c) & 0xffff0073;
                piVar26 = (in_stack_0000a11c + 1);
                piVar40 = (piVar41 + 1);
                uVar8 = _in(in_stack_0000a11c);
                *piVar41 = uVar8;
                piVar61 = in_stack_0000a11c;
                piVar53 = in_stack_0000a11c;
            }
            piVar62 = (piVar40 + 1);
            uVar8 = _in(uVar34);
            *piVar40 = uVar8;
            piVar4 = piVar53;
            piVar53 = piVar53 + 1;
            out(*piVar4, uVar34);
            puVar5 = (uVar50 + piVar53);
            *puVar5 = *puVar5 + bVar70 * ((piVar62 & 3) - (*puVar5 & 3));
            uVar21 = uVar21 & 0xffff0073;
            pcVar22 = (piVar61 + piVar53);
            cVar12 = uVar21;
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar61 + piVar53);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar61 + piVar53);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar61 + piVar53);
            *pcVar22 = *pcVar22 + cVar12;
            in_stack_0000a11c = piVar26;
        } else {
            uVar50 = uVar35 | ZEXT24(piVar40);
            uVar34 = uVar33 & 0xffff0000 | ZEXT24(in_stack_0000a11c);
            uVar10 = ZEXT24(in_stack_0000a11c);
            uVar21 = uVar20 | uVar10;
            bVar24 = (uVar10 >> 8);
            piVar61 = in_stack_0000a11c;
            piVar26 = in_stack_0000a104;
            if (!bVar70) {
                // code_r0x105059da:
                uVar16 = uVar50 - 1;
                uVar50 = uVar50 & 0xffff0000 | uVar16;
                piVar53 = piVar26 + 1;
                out(*piVar26, in_stack_0000a11c);
                if (bVar70 || uVar16 == 0) {
                    uVar8 = _in(in_stack_0000a11c);
                    *piVar40 = uVar8;
                    // goto code_r0x10505a44;
                }
                pbVar1 = (piVar40 + 0x69);
                bVar70 = CARRY1(*pbVar1, bVar11);
                *pbVar1 = *pbVar1 + bVar11;
                bVar14 = *pbVar1;
                out(*piVar53, in_stack_0000a11c);
                piVar53 = (piVar26 + 5);
                out((piVar26 + 3), in_stack_0000a11c);
                // code_r0x105059e5:
                iVar25 = _in(in_stack_0000a11c);
                *piVar40 = iVar25;
                piVar62 = (piVar40 + 3);
                uVar8 = _in(in_stack_0000a11c);
                *(piVar40 + 1) = uVar8;
                if (bVar14 != 0) {
                    if (!bVar70) {
                        out(*piVar53, in_stack_0000a11c);
                        out((piVar53 + 1), in_stack_0000a11c);
                        piVar4 = piVar40 + 0x36;
                        *piVar4 = *piVar4 & bVar11;
                        out(*(piVar53 + 3), in_stack_0000a11c);
                        pbVar1 = (piVar62 + uVar50 + 0x43);
                        bVar14 = *pbVar1;
                        *pbVar1 = *pbVar1 + bVar13;
                        piVar26 = piVar53 + 3;
                        out(piVar53[2], in_stack_0000a11c);
                        piVar53 = piVar53 + 4;
                        out(*piVar26, in_stack_0000a11c);
                        if (CARRY1(bVar14, bVar13) || *pbVar1 == 0) {
                            pcVar22 = (piVar61 + piVar53);
                            *pcVar22 = *pcVar22 + bVar13;
                            pcVar22 = (piVar61 + piVar53);
                            *pcVar22 = *pcVar22 + bVar13;
                            piVar4 = piVar62;
                            *piVar4 = *piVar4 + bVar24;
                            // goto code_r0x10505a6f;
                        }
                        pcVar22 = (piVar61 + piVar53);
                        *pcVar22 = *pcVar22 + bVar13;
                        uVar21 = uVar20
                            | CONCAT11(
                                ((ZEXT24(in_stack_0000a11c) | 0xa0d) >> 8) | *piVar62,
                                (ZEXT24(in_stack_0000a11c) | 0xa0d),
                            );
                    }
                    bVar70 = false;
                    uVar21 = uVar21 & 0xffff6c35;
                    bVar68 = false;
                    piVar26 = in_stack_0000a11c;
                    piVar41 = piVar61;
                    // goto code_r0x10505a1f;
                }
                pbVar1 = (piVar61 + piVar53);
                *pbVar1 = *pbVar1 & bVar13;
                piVar62 = piVar61;
                // goto code_r0x10505a52;
            }
            if (iVar15 == -1) {
                pbVar1 = (in_stack_0000a104 + 0x75);
                *pbVar1 = *pbVar1 & bVar24;
                // goto code_r0x10505a2f;
            }
            pbVar1 = (in_stack_0000a11c + in_stack_0000a104);
            bVar70 = false;
            *pbVar1 = *pbVar1 & bVar11;
            bVar14 = *pbVar1;
            piVar53 = in_stack_0000a104;
            if (*pbVar1 < '\0') {
                //goto code_r0x105059e5;
            }
            pbVar1 = (in_stack_0000a11c + in_stack_0000a104);
            *pbVar1 = *pbVar1 ^ bVar11;
            puVar59 = (piVar40 + 1);
            uVar8 = _in(in_stack_0000a11c);
            *piVar40 = uVar8;
            pcVar22 = (piVar40 + puVar59 + 0x43);
            *pcVar22 = *pcVar22 + bVar13;
            lVar9 = piVar40[0x32] * 0x73;
            iVar25 = lVar9;
            piVar41 = (in_stack_0000a11c + 1);
            uVar16 = iVar25 - 1;
            uVar50 = uVar35 | uVar16;
            piVar26 = in_stack_0000a104 + 1;
            out(*in_stack_0000a104, in_stack_0000a11c);
            if (iVar25 == lVar9) {
                piVar40 = piVar40 + 1;
                uVar8 = _in(in_stack_0000a11c);
                *puVar59 = uVar8;
                pbVar1 = (piVar41 + piVar26);
                bVar70 = CARRY1(*pbVar1, bVar13);
                *pbVar1 = *pbVar1 + bVar13;
                piVar61 = in_stack_0000a11c + 1;
                // goto code_r0x105059da;
            }
            pbVar1 = (iVar25 + 0x6e);
            *pbVar1 = *pbVar1 & bVar24;
            bVar11 = *pbVar1;
            uVar8 = _in(in_stack_0000a11c);
            *puVar59 = uVar8;
            piVar62 = (piVar40 + 3);
            uVar8 = _in(in_stack_0000a11c);
            *(piVar40 + 1) = uVar8;
            piVar53 = in_stack_0000a104 + 2;
            out(*piVar26, in_stack_0000a11c);
            if (bVar11 != 0) {
                cVar12 = *(piVar41 + piVar53);
                piVar4 = in_stack_0000a11c + -0x30f6;
                bVar11 = *piVar4;
                *piVar4 = *piVar4 + piVar41;
                puVar5 = (in_stack_0000a104 + -0x62e9);
                uVar17 = CARRY1(bVar11, piVar41);
                uVar18 = piVar41 + *puVar5;
                piVar61 = (uVar18 + uVar17);
                uVar21 = uVar20
                    | (bVar13 + cVar12)
                        + (uVar16 + piVar53)
                        + (CARRY2(piVar41, *puVar5) || CARRY2(uVar18, uVar17));
                in_stack_0000a11c = (in_stack_0000a11c + 1);
                // goto code_r0x10505a6f;
            }
            piVar4 = piVar62;
            bVar11 = *piVar4;
            *piVar4 = *piVar4 + bVar24;
            piVar61 = piVar41;
            if (CARRY1(bVar11, bVar24)) {
                uVar21 = uVar20 | uVar10 & 0xffff646c;
                pbVar1 = (piVar41 + piVar53);
                *pbVar1 = *pbVar1 & (uVar10 & 0xffff646c);
                piVar41 = in_stack_0000a11c;
                // goto code_r0x10505a48;
            }
            pcVar22 = (piVar41 + piVar53);
            *pcVar22 = *pcVar22 + bVar13;
            uVar8 = *(uVar16 + piVar62);
            uVar34 = CONCAT31((uVar34 >> 8) & 0xffff00 | (*(uVar16 + piVar62) >> 8), uVar8);
            *(uVar16 + piVar62) = uVar8;
            // code_r0x10505a76:
            (uVar50 + piVar62) = uVar34;
            uVar21 = uVar21 & 0xffffff00 | (uVar21 + *(piVar61 + piVar53));
            pcVar22 = (piVar61 + piVar53);
            *pcVar22 = *pcVar22 + -0x71;
        }
        // code_r0x10505a6f:
        uVar19 = uVar50;
        pbVar1 = (piVar62 + uVar19 + 0x8a13);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + in_stack_0000a11c;
        puVar5 = (piVar61 + piVar53 + -0x76ed);
        uVar16 = CARRY1(bVar11, in_stack_0000a11c);
        uVar17 = in_stack_0000a11c + *puVar5;
        uVar27 = uVar17 + uVar16;
        uVar18 = uVar21
            + (uVar19 + piVar53)
            + (CARRY2(in_stack_0000a11c, *puVar5) || CARRY2(uVar17, uVar16));
        piVar4 = piVar53;
        bVar13 = uVar18;
        *piVar4 = *piVar4 + bVar13;
        pbVar1 = (piVar61 + piVar53 + -0x7100);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (uVar19 + 0x8c13);
        uVar16 = uVar27 + *puVar5;
        uVar28 = uVar16
            + CARRY1(bVar11, bVar13)
            + (piVar62 + 0x113)
            + (CARRY2(uVar27, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar13)));
        piVar4 = piVar53;
        *piVar4 = *piVar4 + uVar34;
        pbVar1 = (piVar61 + piVar62 + -0x6d00);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (piVar53 + uVar19 + 0x9013);
        uVar16 = uVar34 + *puVar5;
        uVar17 = uVar16
            + CARRY1(bVar11, bVar13)
            + (piVar61 + piVar62 + 0x113)
            + (CARRY2(uVar34, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar13)));
        piVar4 = piVar53;
        *piVar4 = *piVar4 + uVar17;
        pbVar1 = (piVar53 + uVar19 + 0x9700);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (uVar19 + 0x9413);
        uVar16 = uVar17 + *puVar5;
        uVar31 = uVar16
            + CARRY1(bVar11, bVar13)
            + (piVar62 + 0x213)
            + (CARRY2(uVar17, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar13)));
        piVar4 = piVar53;
        *piVar4 = *piVar4 + bVar13;
        pbVar1 = 0x9b00;
        bVar11 = *pbVar1;
        bVar24 = (piVar61 >> 8);
        *pbVar1 = *pbVar1 + bVar24;
        puVar5 = (piVar53 + uVar19 + 0x9813);
        uVar16 = CARRY1(bVar11, bVar24);
        uVar17 = piVar61 + *puVar5;
        iVar25 = uVar17 + uVar16;
        uVar27 = iVar25
            + (piVar62 + iVar25 + 0x213)
            + (CARRY2(piVar61, *puVar5) || CARRY2(uVar17, uVar16));
        piVar4 = piVar53;
        *piVar4 = *piVar4 + bVar13;
        pbVar1 = (uVar27 + piVar62);
        bVar70 = CARRY1(*pbVar1, bVar13);
        *pbVar1 = *pbVar1 + bVar13;
        uVar18 = uVar18 & 0xff;
        uVar17 = uVar18
            | ((*pbVar1 < '\0') << 7
                | (*pbVar1 == 0) << 6
                | in_AF << 4
                | ((POPCOUNT(*pbVar1) & 1) == 0) << 2
                | 2
                | bVar70)
                << 8;
        puVar5 = (uVar19 + 0x9c13);
        uVar16 = uVar27 + *puVar5;
        uVar42 = uVar16
            + bVar70
            + (piVar62 + 0x213)
            + (CARRY2(uVar27, *puVar5) || CARRY2(uVar16, bVar70));
        pcVar22 = (uVar19 + piVar53);
        *pcVar22 = *pcVar22 + uVar18;
        pcVar22 = (uVar19 + piVar53);
        bVar24 = uVar28;
        *pcVar22 = *pcVar22 + bVar24;
        pbVar1 = (piVar62 + uVar19 + 0xa213);
        bVar11 = *pbVar1;
        bVar13 = (uVar17 >> 8);
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (piVar53 + uVar42 + 0xa113);
        uVar16 = CARRY1(bVar11, bVar13);
        puVar59 = &stack0xa11e + *puVar5;
        uVar18 = uVar17
            + (uVar42 + piVar62)
            + (CARRY2(&stack0xa11e, *puVar5) || CARRY2(puVar59, uVar16));
        pcVar22 = (uVar19 + piVar53);
        *pcVar22 = *pcVar22 + uVar18;
        pcVar22 = (uVar19 + piVar62);
        cVar12 = uVar31;
        *pcVar22 = *pcVar22 + cVar12;
        pbVar1 = (uVar42 + 0xa613);
        bVar11 = *pbVar1;
        bVar13 = (uVar18 >> 8);
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (piVar53 + -0x5aed);
        uVar17 = CARRY1(bVar11, bVar13);
        puVar56 = puVar59 + *puVar5 + uVar16;
        iVar25 = uVar18
            + (uVar42 + piVar53)
            + (CARRY2((puVar59 + uVar16), *puVar5) || CARRY2(puVar56, uVar17));
        pcVar22 = (uVar19 + piVar53);
        *pcVar22 = *pcVar22 + bVar24;
        pbVar1 = (piVar62 + uVar19 + 0xab00);
        bVar11 = *pbVar1;
        bVar13 = iVar25;
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (piVar53 + uVar19 + 0xa813);
        uVar16 = CARRY1(bVar11, bVar13);
        uVar18 = uVar19 + *puVar5;
        uVar27 = uVar18
            + uVar16
            + (piVar62 + uVar42 + 0x13)
            + (CARRY2(uVar19, *puVar5) || CARRY2(uVar18, uVar16));
        piVar4 = piVar53;
        *piVar4 = *piVar4 + cVar12;
        pcVar22 = (uVar27 + piVar53);
        *pcVar22 = *pcVar22 + cVar12;
        pbVar1 = (uVar42 + 0xae13);
        bVar11 = *pbVar1;
        bVar13 = (uVar28 >> 8);
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (piVar53 + -0x52ed);
        uVar16 = CARRY1(bVar11, bVar13);
        uVar18 = uVar27 + *puVar5;
        uVar49 = uVar18 + uVar16;
        uVar18 = iVar25 + (uVar42 + piVar53) + (CARRY2(uVar27, *puVar5) || CARRY2(uVar18, uVar16));
        piVar4 = piVar53;
        bVar13 = uVar18;
        *piVar4 = *piVar4 + bVar13;
        piVar4 = piVar53 + -0x2680;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar13;
        puVar5 = (piVar53 + uVar49 + 0xb013);
        uVar16 = piVar53 + *puVar5;
        pcVar54 = (uVar16
            + CARRY1(bVar11, bVar13)
            + (piVar62 + uVar42 + 0x13)
            + (CARRY2(piVar53, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar13))));
        pcVar22 = pcVar54;
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar62 + -0x2480;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + bVar13;
        puVar5 = (uVar49 + 0xb413);
        pcVar54 = pcVar54
            + *puVar5
            + (CARRY2(pcVar54, *puVar5) || CARRY2((pcVar54 + *puVar5), CARRY1(bVar11, bVar13)))
            + (piVar62 + 0x13)
            + CARRY1(bVar11, bVar13);
        pcVar22 = pcVar54 + uVar49;
        *pcVar22 = *pcVar22 + bVar13;
        pbVar1 = (uVar49 + 0xbb00);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (pcVar54 + uVar49 + 0xb813);
        uVar16 = piVar62 + *puVar5;
        iVar25 = uVar16 + CARRY1(bVar11, bVar13);
        uVar27 = iVar25
            + (uVar42 + iVar25 + 0x13)
            + (CARRY2(piVar62, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar13)));
        pcVar22 = pcVar54 + uVar49;
        *pcVar22 = *pcVar22 + bVar24;
        pbVar1 = (uVar42 + 0xbf00);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar13;
        puVar5 = (uVar49 + 0xbc13);
        uVar16 = uVar27 + *puVar5;
        iVar25 = uVar16 + CARRY1(bVar11, bVar13);
        uVar63 = iVar25
            + (iVar25 + 0x13)
            + (CARRY2(uVar27, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar13)));
        pcVar22 = pcVar54;
        *pcVar22 = *pcVar22 + bVar13;
        pbVar1 = (pcVar54 + uVar42 + 0xc300);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar24;
        uVar27 = uVar18 + uVar31 + CARRY1(bVar11, bVar24);
        uVar16 = (CARRY2(uVar18, uVar31) || CARRY2(uVar18 + uVar31, CARRY1(bVar11, bVar24)));
        uVar19 = uVar27 * 2 + uVar16;
        uVar16 = (CARRY2(uVar27, uVar27) || CARRY2(uVar27 * 2, uVar16));
        uVar18 = uVar19 + uVar28;
        uVar27 = uVar18
            + uVar16
            + (pcVar54 + uVar42)
            + (CARRY2(uVar19, uVar28) || CARRY2(uVar18, uVar16));
        pcVar22 = pcVar54;
        cVar12 = uVar27;
        *pcVar22 = *pcVar22 + cVar12;
        pbVar1 = (uVar42 + uVar63 + -0x7600);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar24;
        puVar5 = (uVar49 + 0x9213);
        uVar16 = uVar28 + *puVar5;
        uVar19 = uVar16 + CARRY1(bVar11, bVar24);
        puVar3 = (uVar49 + 0x9a13);
        uVar16 = (CARRY2(uVar28, *puVar5) || CARRY2(uVar16, CARRY1(bVar11, bVar24)));
        uVar18 = uVar31 + *puVar3;
        uVar28 = uVar18 + uVar16;
        uVar35 = uVar34 & 0xffff0000 | uVar28;
        puVar5 = (uVar49 + 0xa213);
        uVar16 = (CARRY2(uVar31, *puVar3) || CARRY2(uVar18, uVar16));
        uVar18 = uVar42 + *puVar5;
        piVar26 = (uVar18 + uVar16);
        puVar3 = (uVar49 + 0xaa13);
        uVar18 = (CARRY2(uVar42, *puVar5) || CARRY2(uVar18, uVar16));
        puVar59 = puVar56 + *puVar3 + uVar17;
        puVar5 = (uVar49 + 0xb213);
        uVar16 = (CARRY2((puVar56 + uVar17), *puVar3) || CARRY2(puVar59, uVar18));
        uVar17 = uVar49 + *puVar5;
        iVar25 = uVar17 + uVar16;
        puVar3 = (iVar25 + -0x45ed);
        uVar16 = (CARRY2(uVar49, *puVar5) || CARRY2(uVar17, uVar16));
        puVar55 = (pcVar54 + *puVar3 + uVar16);
        puVar5 = (iVar25 + -0x3ded);
        uVar16 = (CARRY2(pcVar54, *puVar3) || CARRY2((pcVar54 + *puVar3), uVar16));
        uVar17 = uVar63 + *puVar5;
        puVar64 = (uVar17 + uVar16);
        uVar17 = iVar25
            + (piVar26 + puVar55 + -0x56f1)
            + (CARRY2(uVar63, *puVar5) || CARRY2(uVar17, uVar16));
        uVar20 = uVar50 & 0xffff0000 | uVar17;
        uVar67 = LocalDescriptorTableRegister();
        (piVar26 + puVar55) = uVar67;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        puVar5 = (piVar26 + puVar55);
        uVar16 = *puVar5;
        *puVar5 = *puVar5 + uVar27;
        bVar11 = cVar12 + CARRY2(uVar16, uVar27);
        uVar21 = uVar21 & 0xffff0000 | uVar27 & 0xffffff00 | bVar11;
        puVar44 = puVar59 + (uVar18 - 2);
        puVar45 = (puVar59 + (uVar18 - 2));
        (puVar59 + (uVar18 - 2)) = uVar66;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + bVar11;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + bVar11;
        puVar5 = puVar64;
        *puVar5 = *puVar5 + bVar11;
        pcVar22 = (uVar17 + puVar55);
        *pcVar22 = *pcVar22 + bVar11;
        pcVar22 = (uVar17 + puVar64);
        cVar12 = uVar19;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (uVar17 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        puVar5 = puVar64;
        *puVar5 = *puVar5 + bVar11;
        puVar60 = puVar55;
        *puVar60 = *puVar60 + bVar11;
        puVar60 = puVar55;
        *puVar60 = *puVar60 + bVar11;
        pcVar22 = (uVar17 + puVar64);
        *pcVar22 = *pcVar22 + bVar11;
        puVar60 = puVar55;
        *puVar60 = *puVar60 + cVar12;
        piVar4 = piVar26;
        *piVar4 = *piVar4 + bVar11;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55);
        *pcVar22 = *pcVar22 + bVar11;
        puVar5 = puVar64;
        bVar11 = *puVar5;
        bVar13 = ((uVar27 & 0xffffff00) >> 8);
        *puVar5 = *puVar5 + bVar13;
        bVar24 = (uVar28 >> 8);
        if (CARRY1(bVar11, bVar13)) {
            uVar21 = uVar21 & 0xffff2173;
            pcVar22 = (piVar26 + puVar55);
            bVar14 = uVar21;
            *pcVar22 = *pcVar22 + bVar14;
            pcVar22 = (piVar26 + puVar55);
            *pcVar22 = *pcVar22 + bVar14;
            pcVar22 = (piVar26 + puVar55);
            *pcVar22 = *pcVar22 + bVar14;
            pcVar22 = (piVar26 + puVar55);
            *pcVar22 = *pcVar22 + bVar14;
            pbVar1 = (piVar26 + puVar55);
            bVar11 = *pbVar1;
            bVar13 = *pbVar1;
            *pbVar1 = *pbVar1 + bVar14;
            if (!SCARRY1(bVar13, bVar14)) {
                out(*puVar55, uVar28);
                puVar5 = (puVar55 + 0x71);
                *puVar5 = *puVar5 + CARRY1(bVar11, bVar14) * ((uVar19 & 3) - (*puVar5 & 3));
                pbVar1 = (piVar26 + (puVar55 + 1) + 0x72);
                bVar11 = *pbVar1;
                *pbVar1 = *pbVar1 + bVar24;
                out(puVar55[1], uVar28);
                puVar5 = (piVar26 + puVar55 + 0x6d);
                *puVar5 = *puVar5 + CARRY1(bVar11, bVar24) * ((uVar19 & 3) - (*puVar5 & 3));
                puVar60 = puVar55 + 0x36;
                *puVar60 = *puVar60 + bVar24;
                bVar70 = *puVar60 == '\0';
                puVar55 = ((puVar64 + uVar17 + 0x4c) * 0x6f);
                if (!bVar70) {
                    // goto code_r0x10505be7;
                }
                pcVar22 = (puVar55 + uVar17 + 1);
                *pcVar22 = *pcVar22 + bVar14;
                puVar64 = (puVar64 + 1);
                puVar44 = puVar59 + (uVar18 - 4);
                (puVar59 + (uVar18 - 4)) = uVar66;
            }
            // code_r0x10505c44:
            uVar19 = uVar19 + 1;
            piVar4 = piVar26 + 3;
            iVar25 = uVar21;
            *piVar4 = *piVar4 + iVar25;
            pcVar22 = (piVar26 + puVar55 + 1);
            *pcVar22 = *pcVar22 + uVar19;
            pcVar22 = (piVar26 + puVar55);
            *pcVar22 = *pcVar22 + (iVar25 + 0x7400);
            uVar35 = uVar34 & 0xffff0000 | (uVar28 + 1);
            piVar4 = piVar26 + 3;
            *piVar4 = *piVar4 + iVar25 + 0x7400;
            uVar21 = uVar21 & 0xffff0000 | (iVar25 - 0xb00);
            pcVar22 = (piVar26 + puVar64 + 1);
            *pcVar22 = *pcVar22 + uVar19;
            puVar45 = puVar44;
            // code_r0x10505c5a:
            pcVar22 = (piVar26 + puVar55);
            *pcVar22 = *pcVar22 + uVar21;
            piVar26 = (piVar26 + 1);
        } else {
            out(*puVar55, uVar28);
            puVar60 = puVar55 + 0x35;
            *puVar60 = *puVar60 + bVar24;
            bVar70 = *puVar60 == '\0';
            // code_r0x10505be7:
            puVar55 = ((puVar64 + uVar17 + 0x48) * 0x69);
            if (bVar70) {
                piVar4 = (piVar26 + puVar55);
                *piVar4 = *piVar4 + 0x149;
                // goto code_r0x10505c5a;
            }
            puVar55 = ((puVar64 + uVar17 + 0x4c) * 0x6f);
            puVar45 = (puVar59 + (uVar18 - 2));
            if (!bVar70) {
                puVar56 = ((puVar64 + uVar17 + 0x48) * 0x69);
                (puVar59 + (uVar18 - 4)) = 0x6f66;
                out(*puVar56, uVar28);
                (puVar59 + (uVar18 - 6)) = 0x6f66;
                out(puVar56[1], uVar28);
                (puVar59 + (uVar18 - 8)) = 0x6f66;
                puVar55 = (puVar56 + 3);
                out(puVar56[2], uVar28);
                iVar25 = uVar21;
                pcVar22 = (puVar55 + uVar17 + 1);
                *pcVar22 = *pcVar22 + cVar12;
                piVar4 = (piVar26 + puVar55);
                *piVar4 = *piVar4 + iVar25 + 0xe00;
                puVar44 = puVar59 + (uVar18 - 7);
                piVar4 = (uVar17 + 6);
                *piVar4 = *piVar4 + iVar25 + 0xe00;
                pcVar22 = (puVar64 + uVar17 + 1);
                *pcVar22 = *pcVar22 + cVar12;
                piVar4 = (piVar26 + puVar55);
                *piVar4 = *piVar4 + iVar25 + 0x2900;
                pcVar22 = (piVar26 + puVar55);
                cVar12 = (iVar25 + 0x2900);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar26 + puVar55);
                *pcVar22 = *pcVar22 + cVar12;
                pbVar1 = (puVar64 + 1);
                bVar13 = (iVar25 + 0x6600);
                *pbVar1 = *pbVar1 + bVar13;
                pcVar22 = (piVar26 + puVar55);
                *pcVar22 = *pcVar22 + bVar13;
                bVar11 = 9 < (bVar13 & 0xf) | in_AF;
                uVar16 = CONCAT11(((iVar25 + 0x6600) >> 8) - bVar11, bVar13 + bVar11 * -6) & 0xff0f;
                piVar4 = piVar26 + 3;
                *piVar4 = *piVar4 + uVar16;
                pcVar22 = (uVar17 + 1);
                *pcVar22 = *pcVar22 + uVar16;
                pcVar22 = (piVar26 + puVar55);
                *pcVar22 = *pcVar22 + uVar16;
                piVar4 = (piVar26 + puVar55 + 6);
                *piVar4 = *piVar4 + uVar19;
                uVar21 = uVar21 & 0xffff0000 | (uVar16 + 0xb101);
                pcVar22 = (piVar26 + 1);
                cVar12 = (uVar16 + 0xb101);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar26 + puVar55);
                *pcVar22 = *pcVar22 + cVar12;
                // goto code_r0x10505c44;
            }
        }
        piVar4 = (piVar26 + puVar55 + 6);
        *piVar4 = *piVar4 + uVar19;
        uVar67 = *puVar45;
        puVar60 = puVar55;
        *puVar60 = *puVar60 + uVar35;
        pcVar22 = (piVar26 + 1);
        cVar12 = (uVar21 >> 8);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = (piVar26 + puVar55);
        *piVar4 = *piVar4 + 0x6f;
        piVar4 = (uVar17 + 6);
        uVar18 = uVar21;
        *piVar4 = *piVar4 + uVar18;
        pcVar22 = (uVar17 + puVar64);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar55 + 1);
        cVar23 = (uVar19 >> 8);
        *pcVar22 = *pcVar22 + cVar23;
        puVar57 = (puVar55 + 1);
        *puVar45 = uVar67;
        puVar65 = puVar64 + 1;
        *puVar64 = uVar18;
        piVar4 = piVar26;
        *piVar4 = *piVar4 + uVar21;
        pcVar22 = (uVar17 + puVar57);
        *pcVar22 = *pcVar22 + 0x7;
        pcVar22 = (piVar26 + puVar65 + 1);
        cVar12 = *pcVar22;
        *pcVar22 = *pcVar22 + cVar23;
        puVar5 = (piVar26 + puVar57);
        uVar16 = *puVar5;
        *puVar5 = uVar18;
        if (SCARRY1(cVar12, cVar23)) {
            puVar57 = puVar55 + 1;
        }
        puVar45[-1] = uVar67;
        uVar18 = 0x700;
        pbVar1 = (piVar26 + puVar65);
        bVar11 = *pbVar1;
        bVar13 = uVar16;
        *pbVar1 = *pbVar1 + bVar13;
        puVar45[-2] = 1;
        *(piVar26 + puVar57) = bVar13;
        if (!CARRY1(bVar11, bVar13)) {
            puVar57 = (puVar57 + 1);
        }
        puVar45[-3] = uVar67;
        uVar35 = uVar35 & 0xffff0000;
        puVar45[-4] = 0;
        pcVar22 = (puVar65 + uVar17 + 1);
        *pcVar22 = *pcVar22 + 0x7;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        uVar67 = puVar45[-4];
        puVar2 = (uVar17 + puVar65);
        *puVar2 = *puVar2;
        bVar11 = _in(0);
        *puVar65 = bVar11;
        piVar4 = puVar57 + 0x3980;
        *piVar4 = *piVar4 + uVar16;
        piVar4 = (uVar17 + 6);
        *piVar4 = *piVar4 + uVar16;
        pbVar1 = (puVar64 + 0x6d03);
        *pbVar1 = *pbVar1 + bVar13;
        piVar4 = (piVar26 + puVar57);
        *piVar4 = *piVar4 + uVar16;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar26 + puVar57);
        *pcVar22 = *pcVar22 + bVar13;
        piVar4 = piVar26;
        *piVar4 = *piVar4 + bVar13;
        puVar60 = puVar57 + 0x3700;
        *puVar60 = *puVar60 + piVar26;
        piVar4 = (uVar17 + puVar57);
        *piVar4 = *piVar4 + 0x700;
        puVar2 = (puVar57 + 1);
        *puVar2 = *puVar2;
        puVar58 = (puVar57 + 1);
        0x2afe = uVar67;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + bVar13;
        pcVar22 = (uVar21 & 0xffff0000 | uVar16 & 0xffff0071);
        puVar64 = puVar64 + 1;
        0x2afc = (uVar16 & 0xffff0071);
        uVar34 = uVar35 | 1;
        uVar66 = 0x8217;
        piVar4 = piVar26;
        bVar11 = *piVar4;
        *piVar4 = *piVar4 + -0x7f;
        uVar66 = 0x8219;
        piVar4 = piVar26;
        *piVar4 = *piVar4 + 0x20 + (0x7e < bVar11);
        pbVar1 = (piVar26 + puVar58 + 0x17);
        bVar24 = (piVar26 >> 8);
        bVar70 = CARRY1(*pbVar1, bVar24);
        bVar11 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar24;
        bVar13 = *pbVar1;
        bVar69 = *pbVar1 == 0;
        bVar68 = (POPCOUNT(*pbVar1) & 1) != 0;
        ppuVar46 = 0x821b;
        if (SCARRY1(bVar11, bVar24)) {
            if (!bVar69) {
                // goto code_r0x10505d87;
            }
            // code_r0x10505de8:
            pcVar6 = (piVar26 + puVar58 + 0x66);
            *pcVar6 = *pcVar6 + (uVar18 >> 8);
            puVar55 = (puVar58 + 2);
            out(puVar58, uVar34);
            puVar58 = (puVar58 + 3);
            out(*puVar55, uVar34);
            pcVar6 = (piVar26 + puVar58);
            cVar12 = pcVar22;
            *pcVar6 = *pcVar6 + cVar12;
            pcVar6 = (piVar26 + puVar58);
            *pcVar6 = *pcVar6 + cVar12;
            pcVar6 = (piVar26 + puVar58);
            *pcVar6 = *pcVar6 + cVar12;
            pcVar6 = (piVar26 + puVar58);
            *pcVar6 = *pcVar6 + cVar12;
            pcVar6 = (piVar26 + puVar58);
            *pcVar6 = *pcVar6 + cVar12;
            // code_r0x10505df9:
            pbVar1 = (piVar26 + puVar58 + 0x66);
            bVar11 = (uVar18 >> 8);
            bVar70 = CARRY1(*pbVar1, bVar11);
            *pbVar1 = *pbVar1 + bVar11;
            puVar7 = puVar58;
            puVar58 = (puVar58 + 2);
            out(puVar7, uVar34);
            // code_r0x10505dfd:
            puVar7 = puVar58;
            puVar58 = (puVar58 + 1);
            out(*puVar7, uVar34);
            // code_r0x10505dfe:
            puVar64 = (puVar64 + -1);
            // code_r0x10505e01:
            ppuVar48 = (ppuVar46 + -2);
            (ppuVar46 + -2) = pcVar22;
            iVar25 = uVar34;
            uVar16 = iVar25 + 1;
            uVar34 = uVar34 & 0xffff0000 | uVar16;
            if (uVar16 == 0) {
                bVar68 = SCARRY2(iVar25, 1);
                if (uVar16 != 0) {
                    // code_r0x10505e07:
                    puVar7 = puVar58;
                    puVar58 = (puVar58 + 1);
                    out(*puVar7, uVar34);
                    // code_r0x10505e08:
                    pcVar6 = (piVar26 + puVar58 + 0x66);
                    *pcVar6 = *pcVar6 + (uVar18 >> 8);
                    puVar57 = (puVar58 + 2);
                    out(puVar58, uVar34);
                    // goto code_r0x10505e0c;
                }
                // code_r0x10505e76:
                iVar25 = uVar20;
                if (!bVar68) {
                    puVar7 = puVar58;
                    puVar58 = (puVar58 + 2);
                    out(puVar7, uVar34);
                    // goto code_r0x10505e79;
                }
                pcVar6 = (piVar26 + puVar58);
                cVar12 = pcVar22;
                *pcVar6 = *pcVar6 + cVar12;
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + cVar12;
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + cVar12;
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + cVar12;
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + cVar12;
                uVar16 = pcVar22 & 0x646c;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + uVar16;
                // goto code_r0x10505ef5;
            }
            // code_r0x10505e79:
            puVar5 = (puVar58 + 0x6f);
            *puVar5 = *puVar5 + bVar70 * ((uVar18 & 3) - (*puVar5 & 3));
            pbVar1 = (piVar26 + puVar58 + 0x72);
            bVar11 = *pbVar1;
            bVar13 = (uVar34 >> 8);
            *pbVar1 = *pbVar1 + bVar13;
            out(puVar58, uVar34);
            puVar5 = (piVar26 + puVar58 + 0x6b);
            *puVar5 = *puVar5 + CARRY1(bVar11, bVar13) * ((uVar18 & 3) - (*puVar5 & 3));
            puVar60 = (puVar58 + 0x6a);
            *puVar60 = *puVar60 + bVar13;
            bVar68 = *puVar60 == '\0';
            // code_r0x10505e86:
            uVar16 = pcVar22;
            iVar25 = uVar20;
            puVar58 = ((puVar64 + iVar25 + 0x4c) * 0x6f);
            if (bVar68) {
                // goto code_r0x10505ef5;
            }
            // code_r0x10505e8d:
            uVar16 = pcVar22;
            iVar25 = uVar20;
            lVar9 = (puVar64 + iVar25 + 0x48) * 0x69;
            puVar58 = lVar9;
            uVar18 = uVar18 - 1;
            uVar32 = uVar34;
            cVar23 = (pcVar22 >> 8);
            cVar12 = pcVar22;
            if (puVar58 == lVar9) {
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + cVar12;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + cVar12;
                puVar5 = puVar64;
                *puVar5 = *puVar5 + cVar23;
                puVar5 = puVar64;
                puVar64 = (puVar64 + 1);
                bVar11 = _in(uVar32);
                *puVar5 = bVar11;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + cVar12;
                // goto code_r0x10505ee9;
            }
            puVar5 = puVar64;
            puVar64 = (puVar64 + 1);
            bVar11 = _in(uVar32);
            *puVar5 = bVar11;
            pcVar6 = (uVar34 + 0x75);
            *pcVar6 = *pcVar6 + cVar23;
            if (*pcVar6 != '\0') {
                puVar55 = (puVar58 + 2);
                out(puVar58, uVar32);
                puVar58 = (puVar58 + 3);
                out(*puVar55, uVar32);
                // goto code_r0x10505e9e;
            }
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            uVar16 = uVar16 & 0x73;
        } else {
            ppuVar46 = 0x821b;
            if (bVar70) {
                // code_r0x10505d87:
                if (bVar69) {
                    pcVar6 = (piVar26 + puVar58);
                    bVar11 = pcVar22;
                    *pcVar6 = *pcVar6 + bVar11;
                    pcVar6 = (piVar26 + puVar58);
                    *pcVar6 = *pcVar6 + bVar11;
                    pcVar6 = (piVar26 + puVar58);
                    *pcVar6 = *pcVar6 + bVar11;
                    pbVar1 = (piVar26 + puVar58);
                    bVar70 = CARRY1(*pbVar1, bVar11);
                    *pbVar1 = *pbVar1 + bVar11;
                    puVar59 = ppuVar46;
                    // code_r0x10505dfa:
                    ppuVar46 = (puVar59 + -2);
                    (puVar59 + -2) = 0x6f66;
                    // goto code_r0x10505dfd;
                }
                // code_r0x10505d89:
                puVar5 = (piVar26 + puVar58);
                iVar25 = (pcVar22 & 3) - (*puVar5 & 3);
                bVar69 = 0 < iVar25;
                *puVar5 = *puVar5 + bVar70 * iVar25;
                // code_r0x10505d8b:
                if (!bVar70) {
                    // goto code_r0x10505e01;
                }
                // code_r0x10505d8d:
                puVar64 = *ppuVar46;
                puVar58 = ppuVar46[1];
                uVar20 = uVar20 & 0xffff0000 | ZEXT24(ppuVar46[2]);
                piVar26 = ppuVar46[4];
                uVar34 = uVar34 & 0xffff0000 | ZEXT24(ppuVar46[5]);
                uVar18 = ppuVar46[6];
                ppuVar47 = ppuVar46 + 7;
                pcVar22 = (pcVar22 & 0xffff0000 | ZEXT24(*ppuVar47));
                ppuVar48 = ppuVar46 + 8;
                ppuVar46 = ppuVar46 + 8;
                if (bVar69) {
                    // goto code_r0x10505df9;
                }
                puVar5 = (piVar26 + puVar58);
                iVar25 = (*ppuVar47 & 3) - (*puVar5 & 3);
                bVar69 = 0 < iVar25;
                *puVar5 = *puVar5 + bVar70 * iVar25;
                // code_r0x10505d95:
                if (bVar69) {
                    puVar7 = puVar58;
                    puVar58 = (puVar58 + 2);
                    out(puVar7, uVar34);
                    // goto code_r0x10505e07;
                }
                // code_r0x10505d97:
                out(*puVar58, uVar34);
                pbVar1 = (piVar26 + (puVar58 + 1));
                bVar70 = CARRY1(*pbVar1, pcVar22);
                *pbVar1 = *pbVar1 + pcVar22;
                bVar11 = *pbVar1;
                ppuVar46 = ppuVar48;
                if (!bVar70) {
                    puVar59 = (puVar58 + 5);
                    out((puVar58 + 1), uVar34);
                    // goto code_r0x10505e0f;
                }
                // code_r0x10505d9c:
                bVar69 = bVar11 == 0;
                puVar64 = *ppuVar46;
                puVar58 = ppuVar46[1];
                uVar20 = uVar20 & 0xffff0000 | ZEXT24(ppuVar46[2]);
                piVar26 = ppuVar46[4];
                uVar34 = uVar34 & 0xffff0000 | ZEXT24(ppuVar46[5]);
                uVar18 = ppuVar46[6];
                pcVar22 = (pcVar22 & 0xffff0000 | ZEXT24(ppuVar46[7]));
                ppuVar48 = ppuVar46 + 8;
                // code_r0x10505d9d:
                if (bVar69) {
                    // goto code_r0x10505e08;
                }
                puVar5 = (piVar26 + puVar58);
                iVar25 = (pcVar22 & 3) - (*puVar5 & 3);
                *puVar5 = *puVar5 + bVar70 * iVar25;
                if (!bVar70) {
                    // goto code_r0x10505e17;
                }
                puVar64 = *ppuVar48;
                puVar59 = ppuVar48[1];
                uVar27 = ppuVar48[2];
                uVar20 = uVar20 & 0xffff0000 | uVar27;
                piVar26 = ppuVar48[4];
                uVar17 = ppuVar48[5];
                uVar34 = uVar34 & 0xffff0000 | uVar17;
                uVar18 = ppuVar48[6];
                uVar16 = ppuVar48[7];
                pcVar22 = (pcVar22 & 0xffff0000 | uVar16);
                ppuVar46 = ppuVar48 + 8;
                ppuVar48 = ppuVar48 + 8;
                if (0 < iVar25) {
                    // goto code_r0x10505e0f;
                }
                puVar5 = (piVar26 + puVar59);
                iVar25 = (uVar16 & 3) - (*puVar5 & 3);
                bVar68 = 0 < iVar25;
                *puVar5 = *puVar5 + bVar70 * iVar25;
                if (bVar68) {
                    if (!bVar68) {
                        // goto code_r0x10505e1e;
                    }
                    // goto code_r0x10505e86;
                }
                out(*puVar59, uVar17);
                pcVar6 = (piVar26 + (puVar59 + 1));
                *pcVar6 = *pcVar6 + uVar16;
                pcVar6 = puVar59 + 0x69;
                bVar11 = (uVar17 >> 8);
                *pcVar6 = *pcVar6 + bVar11;
                if (*pcVar6 == '\0') {
                    pcVar6 = ((puVar64 + uVar27 + 0x4c) * 0x6f + 0x68);
                    *pcVar6 = *pcVar6 + bVar11;
                    // goto code_r0x10505e25;
                }
                lVar9 = (puVar64 + uVar27 + 0x48) * 0x69;
                puVar55 = lVar9;
                bVar70 = puVar55 != lVar9;
                if (bVar70) {
                    // goto code_r0x10505e33;
                }
                out(*puVar55, uVar17);
                puVar5 = (puVar55 + 0x71);
                *puVar5 = *puVar5 + bVar70 * ((uVar18 & 3) - (*puVar5 & 3));
                pbVar1 = (piVar26 + (puVar55 + 1) + 0x72);
                bVar13 = *pbVar1;
                *pbVar1 = *pbVar1 + bVar11;
                out(puVar55[1], uVar17);
                puVar5 = (piVar26 + puVar55 + 0x6d);
                *puVar5 = *puVar5 + CARRY1(bVar13, bVar11) * ((uVar18 & 3) - (*puVar5 & 3));
                puVar60 = puVar55 + 0x36;
                *puVar60 = *puVar60 + bVar11;
                puVar58 = ((puVar64 + uVar27 + 0x4c) * 0x6f);
                if (*puVar60 != '\0') {
                    lVar9 = (puVar64 + uVar27 + 0x48) * 0x69;
                    puVar55 = lVar9;
                    bVar70 = puVar55 != lVar9;
                    if (!bVar70) {
                        out(*puVar55, uVar17);
                        puVar5 = (puVar55 + 0x71);
                        *puVar5 = *puVar5 + bVar70 * ((uVar18 & 3) - (*puVar5 & 3));
                        pbVar1 = (piVar26 + (puVar55 + 1) + 0x72);
                        bVar13 = *pbVar1;
                        *pbVar1 = *pbVar1 + bVar11;
                        puVar58 = (puVar55 + 2);
                        out(puVar55[1], uVar17);
                        puVar5 = (piVar26 + puVar58 + 0x69);
                        *puVar5 = *puVar5 + CARRY1(bVar13, bVar11) * ((uVar18 & 3) - (*puVar5 & 3));
                        // goto code_r0x10505de8;
                    }
                    // goto code_r0x10505e4f;
                }
                pcVar6 = (puVar58 + uVar27 + 0x75);
                *pcVar6 = *pcVar6 + (uVar16 >> 8);
                if (*pcVar6 != '\0') {
                    puVar7 = puVar58;
                    puVar58 = (puVar58 + 2);
                    out(puVar7, uVar17);
                    // goto code_r0x10505e44;
                }
                pbVar1 = (puVar58 + uVar27 + 100);
                *pbVar1 = *pbVar1 & bVar11;
                bVar11 = *pbVar1;
                // code_r0x10505eba:
                cVar12 = uVar18;
                bVar70 = false;
                out(puVar58, uVar34);
                // code_r0x10505ebb:
                iVar25 = uVar20;
                uVar16 = pcVar22;
                puVar58 = (puVar58 + 2);
                iVar15 = uVar34;
                puVar65 = puVar64;
                if (!bVar70 && bVar11 != 0) {
                    // code_r0x10505f2b:
                    pcVar22 = (piVar26 + puVar58);
                    cVar12 = uVar16;
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar64);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar64);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    // code_r0x10505f3b:
                    pcVar22 = (piVar26 + puVar58);
                    cVar12 = uVar16;
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar64);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    piVar26 = (piVar26 & 0xff | ((piVar26 >> 8) * 0x2) << 8);
                    piVar4 = (piVar26 + puVar58);
                    *piVar4 = *piVar4 + 1;
                    // goto code_r0x10505f47;
                }
                // code_r0x10505ebd:
                bVar11 = uVar16 * 0x2;
                uVar16 = uVar16 & 0xff00 | bVar11;
                pcVar22 = (piVar26 + puVar58 + 0x80);
                *pcVar22 = *pcVar22 + 0x2;
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + bVar11;
                puVar64 = (puVar65 * 2);
                piVar4 = (piVar26 + puVar58);
                *piVar4 = *piVar4 + 1;
                piVar4 = piVar26 + 0x28;
                *piVar4 = *piVar4 + cVar12;
                uVar17 = iVar15 + 1;
                uVar34 = uVar17;
                if (uVar17 != 0) {
                    // code_r0x10505f47:
                    puVar2 = (piVar26 + puVar58);
                    *puVar2 = *puVar2;
                    pcVar22 = (piVar26 + puVar58);
                    cVar12 = uVar16;
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    pcVar22 = (piVar26 + puVar58);
                    *pcVar22 = *pcVar22 + cVar12;
                    piVar4 = (piVar26 + puVar58);
                    *piVar4 = *piVar4 + uVar16;
                    // WARNING: Bad instruction - Truncating control flow here
                    halt_baddata();
                }
                if (uVar17 == 0) {
                    return;
                }
                puVar7 = puVar58;
                puVar58 = (puVar58 + 1);
                out(*puVar7, 0);
                pcVar22 = (piVar26 + puVar58);
                *pcVar22 = *pcVar22 + bVar11;
            } else {
                ppuVar46 = 0x821b;
                if (!bVar70) {
                    // goto code_r0x10505d89;
                }
                ppuVar46 = 0x821b;
                if (bVar69) {
                    // goto code_r0x10505d8b;
                }
                if (-1 < bVar13) {
                    // goto code_r0x10505d8d;
                }
                ppuVar46 = 0x821b;
                if (bVar68) {
                    *piVar26 = 1;
                    uVar34 = uVar35 | *piVar26;
                    ppuVar48 = ppuVar46;
                    if (!bVar70) {
                        // goto code_r0x10505d95;
                    }
                    if (bVar69) {
                        // goto code_r0x10505d97;
                    }
                    if (-1 < bVar13) {
                        pbVar1 = (puVar64 + uVar17 + 0x74);
                        bVar70 = false;
                        *pbVar1 = *pbVar1;
                        bVar11 = *pbVar1;
                        // goto code_r0x10505d9c;
                    }
                    if (bVar68) {
                        puVar59 = 0x821b;
                        if (bVar70) {
                            puVar64 = *0x821b;
                            puVar58 = 0x821d;
                            uVar20 = uVar50 & 0xffff0000 | *0x821f;
                            piVar26 = 0x8223;
                            uVar34 = uVar35 | 0x8225;
                            uVar18 = 0x8227;
                            pcVar22 = (uVar21 & 0xffff0000 | *0x8229);
                            ppuVar46 = 0x822b;
                            // goto code_r0x10505d87;
                        }
                        // goto code_r0x10505dfa;
                    }
                    if (!bVar69) {
                        // goto code_r0x10505d9d;
                    }
                    // goto code_r0x10505dfe;
                }
                ppuVar48 = ((uVar17 + puVar64) * 0x7562);
                if (!bVar69) {
                    puVar7 = puVar58;
                    puVar58 = (puVar57 + 3);
                    out(puVar7, 1);
                    // goto code_r0x10505d97;
                }
                puVar57 = (puVar57 + 5);
                out(*puVar58, 1);
                // code_r0x10505e0c:
                puVar59 = (puVar57 + 1);
                out(*puVar57, uVar34);
                // code_r0x10505e0f:
                (ppuVar48 + -2) = 0x6f66;
                puVar58 = (puVar59 + 1);
                out(*puVar59, uVar34);
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + pcVar22;
                // code_r0x10505e17:
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + pcVar22;
                pcVar6 = (piVar26 + puVar58);
                *pcVar6 = *pcVar6 + pcVar22;
                puVar7 = puVar58 + 0x1a;
                *puVar7 = *puVar7 + (uVar34 >> 8);
                bVar68 = *puVar7 == '\0';
                // code_r0x10505e1e:
                if (bVar68) {
                    // goto code_r0x10505e8d;
                }
                // code_r0x10505e25:
                lVar9 = (puVar64 + uVar20 + 0x48) * 0x69;
                puVar58 = lVar9;
                if (puVar58 != lVar9) {
                    // code_r0x10505e9e:
                    uVar16 = pcVar22;
                    cVar12 = uVar18;
                    puVar5 = puVar64;
                    *puVar5 = *puVar5 & (pcVar22 >> 8);
                    pbVar1 = (piVar26 + puVar58);
                    *pbVar1 = *pbVar1 ^ (piVar26 >> 8);
                    bVar24 = *pbVar1;
                    bVar13 = *pbVar1;
                    puVar65 = (puVar64 + 1);
                    uVar32 = uVar34;
                    bVar11 = _in(uVar32);
                    *puVar64 = bVar11;
                    if (-1 < bVar24) {
                        puVar64 = puVar64 + 1;
                        bVar11 = _in(uVar32);
                        *puVar65 = bVar11;
                        puVar7 = puVar58;
                        puVar58 = (puVar58 + 2);
                        out(puVar7, uVar32);
                        if (bVar13 == 0) {
                            // code_r0x10505eaa:
                            uVar16 = pcVar22;
                            pcVar6 = (puVar58 + uVar20 + 0x75);
                            bVar11 = (pcVar22 >> 8);
                            *pcVar6 = *pcVar6 + bVar11;
                            if (*pcVar6 != '\0') {
                                puVar55 = (puVar58 + 2);
                                out(puVar58, uVar34);
                                puVar58 = (puVar58 + 3);
                                out(*puVar55, uVar34);
                                puVar5 = puVar64;
                                *puVar5 = *puVar5 & bVar11;
                                // goto code_r0x10505eb3;
                            }
                            pcVar6 = (piVar26 + puVar58);
                            cVar12 = pcVar22;
                            *pcVar6 = *pcVar6 + cVar12;
                            pcVar22 = (piVar26 + puVar58);
                            *pcVar22 = *pcVar22 + cVar12;
                            pcVar22 = (piVar26 + puVar58);
                            *pcVar22 = *pcVar22 + cVar12;
                            pcVar22 = (piVar26 + puVar58);
                            *pcVar22 = *pcVar22 + cVar12;
                            // goto code_r0x10505f2b;
                        }
                        pcVar6 = (piVar26 + puVar58);
                        cVar12 = *pcVar6;
                        *pcVar6 = *pcVar6 + pcVar22;
                        if (*pcVar6 != '\0' && SCARRY1(cVar12, pcVar22) == (*pcVar6 < '\0')) {
                            // goto PTR_LOOP_1050_5f1c;
                        }

                        // goto code_r0x10505f3b;
                    }
                    // code_r0x10505ec5:
                    uVar16 = pcVar22;
                    iVar15 = uVar34;
                    iVar25 = uVar20;
                    pcVar6 = (iVar25 + puVar58);
                    *pcVar6 = *pcVar6 + pcVar22;
                    // goto code_r0x10505ebd;
                }
                puVar55 = (puVar58 + 2);
                out(puVar58, uVar34);
                puVar5 = (puVar58 + 0x71);
                *puVar5 = *puVar5 + (puVar58 != lVar9) * ((uVar18 & 3) - (*puVar5 & 3));
                pbVar1 = (piVar26 + puVar55 + 0x72);
                bVar11 = (uVar34 >> 8);
                bVar70 = CARRY1(*pbVar1, bVar11);
                *pbVar1 = *pbVar1 + bVar11;
                // code_r0x10505e33:
                puVar58 = (puVar55 + 1);
                out(*puVar55, uVar34);
                puVar5 = (piVar26 + puVar58 + 0x69);
                *puVar5 = *puVar5 + bVar70 * ((uVar18 & 3) - (*puVar5 & 3));
                pcVar6 = (piVar26 + puVar64 + 0x73);
                *pcVar6 = *pcVar6 + uVar18;
                puVar5 = puVar64;
                puVar64 = (puVar64 + 1);
                bVar11 = _in(uVar34);
                *puVar5 = bVar11;
                *pcVar22 = *pcVar22 + pcVar22;
                if (*pcVar22 != '\0') {
                    // code_r0x10505e44:
                    out(*puVar58, uVar34);
                    pbVar1 = (piVar26 + (puVar58 + 1) + 0x72);
                    bVar11 = *pbVar1;
                    bVar13 = (uVar34 >> 8);
                    *pbVar1 = *pbVar1 + bVar13;
                    puVar55 = (puVar58 + 3);
                    out((puVar58 + 1), uVar34);
                    puVar5 = (piVar26 + puVar55 + 0x69);
                    *puVar5 = *puVar5 + CARRY1(bVar11, bVar13) * ((uVar18 & 3) - (*puVar5 & 3));
                    pbVar1 = (piVar26 + puVar55 + 0x72);
                    bVar70 = CARRY1(*pbVar1, bVar13);
                    *pbVar1 = *pbVar1 + bVar13;
                    // code_r0x10505e4f:
                    cVar12 = uVar18;
                    uVar32 = uVar34;
                    out(*puVar55, uVar32);
                    puVar5 = (puVar55 + 0x71);
                    *puVar5 = *puVar5 + bVar70 * ((uVar18 & 3) - (*puVar5 & 3));
                    puVar60 = puVar55 + 0x35;
                    bVar13 = (uVar34 >> 8);
                    *puVar60 = *puVar60 + bVar13;
                    iVar25 = uVar20;
                    puVar58 = ((puVar64 + iVar25 + 0x48) * 0x69);
                    puVar65 = puVar64;
                    if (*puVar60 != '\0') {
                        lVar9 = (puVar64 + iVar25 + 0x4c) * 0x6f;
                        puVar58 = lVar9;
                        uVar18 = uVar18 - 1;
                        cVar12 = uVar18;
                        if (puVar58 == lVar9) {
                            puVar7 = puVar58;
                            puVar58 = (puVar58 + 1);
                            out(*puVar7, uVar32);
                            // goto code_r0x10505eaa;
                        }
                        puVar5 = puVar64;
                        puVar64 = (puVar64 + 1);
                        bVar11 = _in(uVar32);
                        *puVar5 = bVar11;
                        pbVar1 = (pcVar22 + uVar20 * 2 + 0x69);
                        bVar70 = CARRY1(*pbVar1, bVar13);
                        *pbVar1 = *pbVar1 + bVar13;
                        bVar11 = *pbVar1;
                        if (bVar70) {
                            out(puVar58, uVar32);
                            puVar60 = (puVar58 + 0x6a);
                            *puVar60 = *puVar60 + bVar13;
                            lVar9 = (puVar64 + iVar25 + 0x48) * 0x69;
                            puVar58 = lVar9;
                            bVar70 = puVar58 != lVar9;
                            bVar68 = bVar70;
                            // goto code_r0x10505e76;
                        }
                        out(puVar58, uVar32);
                        // goto code_r0x10505ebb;
                    }
                    // goto code_r0x10505ec5;
                }
                // code_r0x10505eb3:
                iVar25 = uVar20;
                uVar16 = pcVar22;
                pbVar1 = (piVar26 + puVar58);
                *pbVar1 = *pbVar1 ^ (piVar26 >> 8);
                bVar24 = *pbVar1;
                bVar11 = *pbVar1;
                puVar5 = puVar64;
                puVar64 = (puVar64 + 1);
                bVar13 = _in(uVar34);
                *puVar5 = bVar13;
                if (-1 < bVar24) {}
                // goto code_r0x10505eba;
            }
            pcVar22 = (piVar26 + puVar58);
            cVar12 = uVar16;
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            uVar16 = uVar16 & 0x646c;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + uVar16;
            // code_r0x10505ee9:
            pcVar22 = (piVar26 + puVar58);
            cVar12 = uVar16;
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            puVar5 = puVar64;
            *puVar5 = *puVar5 + (uVar16 >> 8);
            // code_r0x10505ef5:
            bVar11 = _in(uVar34);
            *puVar64 = bVar11;
            pcVar22 = (piVar26 + puVar58);
            cVar12 = uVar16;
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pcVar22 = (piVar26 + puVar58);
            *pcVar22 = *pcVar22 + cVar12;
            pbVar1 = (puVar64 + 1);
            *pbVar1 = *pbVar1 + (uVar16 >> 8);
        }
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + uVar16;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + uVar16;
        pcVar22 = (iVar25 + 0x1f);
        *pcVar22 = *pcVar22 + (piVar26 >> 8);
        // PTR_LOOP_1050_5f1c:
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + uVar34;
        pcVar22 = (piVar26 + puVar58);
        cVar12 = uVar16;
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = (piVar26 + puVar58);
        *piVar4 = *piVar4 + uVar16;
        piVar4 = (piVar26 + puVar58);
        *piVar4 = *piVar4 + uVar16;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
        piVar4 = (piVar26 + puVar58);
        *piVar4 = *piVar4 + uVar16;
        pcVar22 = (piVar26 + puVar58);
        *pcVar22 = *pcVar22 + cVar12;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_10d0_02c2() {
    let pbVar1: *mut byte;
    let mut in_AL: u8;
    let mut in_BX: i32;
    let mut unaff_SI: i32;
    let mut unaff_DS: u16;

    pbVar1 = (in_BX + unaff_SI);
    unsafe { *pbVar1 = *pbVar1 | in_AL };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

// WARNING: Unable to track spacebase fully for stack

pub fn bad_fn_1110_029e() {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let piVar3: *mut i32;
    char * *ppcVar4;
    let puVar5: *mut u32;
    let puVar6: *mut u8;
    let pcVar7: *mut char;
    let puVar8: *mut u16;
    char * *ppcVar9;
    let mut cVar10: u8;
    let pcVar11: *mut code;
    let mut in_AL: u8;
    let mut cVar12: u8;
    let mut bVar13: u8;
    let mut cVar14: u8;
    let mut bVar15: u8;
    let mut bVar16: u8;
    let mut cVar17: u8;
    let mut cVar19: u8;
    let mut cVar20: u8;
    let mut in_CX: i32;
    let mut uVar21: i32;
    let mut extraout_DL: u8;
    let mut extraout_DL_00: u8;
    let mut cVar22: u8;
    let mut cVar23: u8;
    let mut in_BX: i32;
    let mut uVar25: i32;
    let puVar26: *mut u32;
    let piVar27: *mut i32;
    let mut unaff_BP: u16;
    let unaff_SI: *mut char;
    let pcVar29: *mut char;
    let unaff_DI: *mut char;
    let mut unaff_ES: u16;
    let mut unaff_DS: u16;
    let mut in_FS: u16;
    let mut uVar30: u32;
    let mut uStack000d: u16;
    let mut in_stack_0000000c: u32;
    let mut uStack0019: u16;
    let mut in_stack_00000018: u32;
    let mut in_stack_00000062: u16;
    let mut bStack0063: u8;
    let mut cStack17: u8;
    let uStack3: u8;
    let mut uVar32: i32;
    let mut iVar18: i32;
    let mut uVar24: i32;
    let mut bVar28: u8;
    let mut uVar31: u32;

    _uStack3 = CONCAT21(unaff_BP, uStack3);
    uVar31 = _uStack3;
    pcVar1 = unaff_SI + in_BX;
    unsafe {
        *pcVar1 = *pcVar1 + in_AL;
        pcVar11 = swi(0);
        cVar12 = (*pcVar11)();
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + extraout_DL;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + in_CX;
        uVar21 = in_CX & 0xff00 | (in_CX * 0x2);
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar11 = swi(0);
        cVar12 = (*pcVar11)();
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + cStack17;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + uVar21;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar11 = swi(0);
        cVar12 = (*pcVar11)();
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + extraout_DL_00;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + uVar21;
        uVar25 = in_BX & 0xff00 | (in_BX + uVar21);
        pcVar1 = unaff_SI + uVar25;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar1 = unaff_SI + uVar25;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar11 = swi(0);
        uVar30 = (*pcVar11)();
        pcVar7 = unaff_SI + 1;
        out(*unaff_SI, (uVar30 >> 0x10));
        pcVar1 = pcVar7;
        *pcVar1 = *pcVar1 + (uVar30 >> 0x10);
        pcVar1 = pcVar7 + uVar25;
        cVar20 = uVar21;
        *pcVar1 = *pcVar1 + cVar20;
        cVar19 = (uVar30 >> 8) + cVar20;
        bVar13 = uVar30;
        _uStack3 = uVar31;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 + bVar13;
        pbVar2 = (pcVar7 + uVar25);
        bVar16 = *pbVar2;
        *pbVar2 = *pbVar2 + bVar13;
        bVar13 = bVar13 + CARRY1(bVar16, bVar13);
        iVar18 = CONCAT11(cVar19, bVar13);
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 - bVar13;
        pbVar2 = (pcVar7 + uVar25);
        *pbVar2 = *pbVar2 | bVar13;
        *0x17 = bVar13;
        pcVar1 = &stack0xfffe + pcVar7;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1;
        pcVar1 = pcVar7 + uVar25;
        cVar22 = (uVar21 >> 8);
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = &stack0x0015 + unaff_DI;
        *pcVar1 = *pcVar1 + cVar19;
        pcVar1 = &stack0xfffe + pcVar7;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1;
        piVar3 = (pcVar7 + uVar25);
        *piVar3 = *piVar3 - iVar18;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 - bVar13;
        pbVar2 = (pcVar7 + uVar25);
        *pbVar2 = *pbVar2 | bVar13;
        puVar8 = (unaff_SI + 2);
        *unaff_DI = *pcVar7;
        pcVar1 = (uVar25 + puVar8);
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = (uVar25 + puVar8);
        *pcVar1 = *pcVar1 + bVar13;
        bVar13 = bVar13 ^ *(uVar25 + puVar8);
        pcVar1 = (uVar25 + puVar8);
        *pcVar1 = *pcVar1 - bVar13;
        pbVar2 = (uVar25 + puVar8);
        *pbVar2 = *pbVar2 | bVar13;
        pcVar7 = unaff_SI + 4;
        (unaff_DI + 1) = *puVar8;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = pcVar7 + uVar25;
        *pcVar1 = *pcVar1 - bVar13;
        pbVar2 = (pcVar7 + uVar25);
        *pbVar2 = *pbVar2 | bVar13;
        pcVar7 = unaff_DI + 4;
        pcVar1 = unaff_SI + 5 + uVar25;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = unaff_SI + 5 + uVar25;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar29 = unaff_SI + 6;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = (uVar25 + 0x17);
        *pcVar1 = *pcVar1 + cVar19;
        pcVar1 = &stack0xfffe + pcVar29;
        *pcVar1 = *pcVar1 + bVar13;
        uVar32 = (uVar31 >> 8) & 0xff00 | (CONCAT11(cVar19, bVar13) >> 8);
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = pcVar29 + uVar25 + 0x17;
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = &stack0xfffe + pcVar29;
        *pcVar1 = *pcVar1 + bVar13;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar20;
        cVar14 = bVar13 * 0x2;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar14;
        cVar12 = pcVar29[uVar25 - 0x7e];
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar14;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar20;
        cVar20 = cVar20 + cVar14;
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar14;
        cVar12 = bVar13 + cVar12 + pcVar29[uVar25 - 0x7e];
        pcVar1 = pcVar29 + uVar25;
        *pcVar1 = *pcVar1 + cVar14;
        ppcVar9 = (unaff_SI + 7);
        out(*pcVar29, CONCAT11(cVar19, cVar12));
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar22;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        cVar22 = cVar12 + cVar14 + *(ppcVar9 + (uVar25 - 0x7e));
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        ppcVar4 = ppcVar9;
        *ppcVar4 = *ppcVar4 + CONCAT11(cVar19, cVar22);
        ppcVar4 = ppcVar9;
        *ppcVar4 = *ppcVar4 + cVar22;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        cVar12 = *(ppcVar9 + (uVar25 - 0x7e));
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        piVar3 = 0x1400;
        *piVar3 = *piVar3 + uVar25;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        cVar22 = cVar22 + cVar12 + *(ppcVar9 + (uVar25 - 0x7e));
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        piVar3 = (pcVar7 + uVar25);
        *piVar3 = (&stack0xfffe + *piVar3);
        ppcVar4 = ppcVar9;
        *ppcVar4 = *ppcVar4 + cVar22;
        pcVar1 = (uVar25 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar20;
        bVar28 = (uVar25 >> 8) + cVar20;
        uVar25 = uVar25 & 0xff;
        puVar26 = (uVar25 | bVar28 << 8);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        cVar22 = cVar22 + *(puVar26 + ppcVar9 + -0x7e);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        piVar3 = (&stack0xfffe + ppcVar9);
        *piVar3 = *piVar3 + ppcVar9;
        ppcVar4 = ppcVar9;
        *ppcVar4 = *ppcVar4 + cVar22;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar20;
        cVar14 = cVar14 + cVar22;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        cVar22 = cVar22 + *(puVar26 + ppcVar9 + -0x7e);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        ppcVar4 = ppcVar9;
        *ppcVar4 = pcVar7 + *ppcVar4;
        ppcVar4 = ppcVar9;
        *ppcVar4 = *ppcVar4 + cVar22;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar20;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        cVar12 = *(puVar26 + ppcVar9 + -0x7e);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar14;
        bVar15 = cVar14 + CARRY2(uVar32, CONCAT11(cVar19, cVar14));
        pbVar2 = (puVar26 + ppcVar9);
        *pbVar2 = *pbVar2 | bVar15;
        puVar5 = puVar26;
        bVar13 = (cVar20 + cVar22 & 0x1f) % 9;
        bVar16 = *puVar5;
        *puVar5 = bVar16 << bVar13 | bVar16 >> 9 - bVar13;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + bVar15;
        cVar23 = cVar22 + cVar12 + *(puVar26 + ppcVar9 + -0x7e);
        uVar24 = CONCAT11(cVar19 + cVar20, cVar23);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + bVar15;
        puVar5 = (puVar26 + pcVar7);
        uVar21 = *puVar5;
        *puVar5 = *puVar5 + uVar24;
        bVar15 = bVar15 + CARRY2(uVar21, uVar24);
        pbVar2 = (puVar26 + ppcVar9);
        *pbVar2 = *pbVar2 | bVar15;
        puVar5 = puVar26;
        bVar16 = (cVar20 + cVar22 & 0x1f) % 0x11;
        uVar21 = *puVar5;
        *puVar5 = uVar21 << bVar16 | uVar21 >> 0x11 - bVar16;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + bVar15;
        cVar12 = *(puVar26 + ppcVar9 + -0x7e);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + bVar15;
        puVar5 = (&stack0xfffe + ppcVar9);
        uVar21 = *puVar5;
        *puVar5 = *puVar5 + puVar26;
        bVar15 = bVar15 + CARRY2(uVar21, puVar26);
        pbVar2 = (puVar26 + ppcVar9);
        *pbVar2 = *pbVar2 | bVar15;
        bVar16 = bVar15 % 0x17;
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + bVar16;
        cVar22 = *(puVar26 + ppcVar9 + -0x7e);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + bVar16;
        ppcVar4 = ppcVar9;
        pcVar29 = *ppcVar4;
        *ppcVar4 = &stack0x001d + *ppcVar4;
        bVar16 = bVar16 + CARRY2(pcVar29, &stack0x001d);
        pbVar2 = (puVar26 + ppcVar9);
        *pbVar2 = *pbVar2 | bVar16;
        cVar17 = bVar16 + (bVar15 / 0x17) * '\x17';
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar17;
        cVar10 = *(puVar26 + ppcVar9 + -0x7e);
        pcVar1 = (puVar26 + ppcVar9);
        *pcVar1 = *pcVar1 + cVar17;
        pbVar2 = (puVar26 + ppcVar9);
        *pbVar2 = *pbVar2 | cVar17 + CARRY2(uVar32 + CONCAT11(cVar19, cVar14), &stack0xfffe);
        puVar6 = (puVar26 + ppcVar9);
        *puVar6 = *puVar6;
        cVar14 = *(puVar26 + ppcVar9 + -0x7e);
        puVar6 = (puVar26 + ppcVar9);
        *puVar6 = *puVar6;
        pcVar1 = pcVar7;
        bVar13 = ppcVar9;
        *pcVar1 = *pcVar1 + bVar13;
        puVar5 = puVar26 + 0x3c00;
        *puVar5 = *puVar5 + uVar25;
        piVar27 = (uVar25 | (bVar28 * 0x2) << 8);
        piVar3 = piVar27;
        *piVar3 = *piVar3 + 1;
        piVar3 = piVar27;
        bVar16 = *piVar3;
        *piVar3 = *piVar3 + bVar13;
        out(
            0x0,
            CONCAT11(cVar19 + cVar20, cVar23 + cVar12 + cVar22 + cVar10 + cVar14),
        );
        if ((bStack0063 + 0x73 + CARRY1(bVar16, bVar13)) == '\0') {
            puVar5 = (unaff_DI + 0x77);
            *puVar5 = *puVar5
                + (0x8c < bStack0063 || CARRY1(bStack0063 + 0x73, CARRY1(bVar16, bVar13)))
                    * ((&stack0x001f & 3) - (*puVar5 & 3));
            piVar3 = piVar27 + 1;
            *piVar3 = *piVar3 + bVar13;
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_3d38() {
    let piVar1: *mut i32;
    let mut cVar2: u8;
    let pcVar3: *mut code;
    let puVar4: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut in_OF: u8;
    let in_ST0: [u8; 10];
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar4 = &stack0xfffe;
    cVar2 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar4 = puVar4 + -1;
        unsafe { *puVar4 = *unaff_BP };
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    pcVar3 = swi(4);
    if (in_OF == 0x1) {
        unsafe { *pcVar3() };
    }
    (&stack0x0000 + unaff_SI) = in_ST0;
    _in(0x3a);
    piVar1 = (unaff_SI + -0x51fd);
    unsafe { *piVar1 = *piVar1 + unaff_SI + 3 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

fn swi(arg: i32) -> () {
    todo!()
}

pub fn bad_fn_1050_335f() {
    let pcVar1: *mut char;
    let mut in_AL: u8;
    let mut in_AH: u8;
    let mut in_BX: i32;
    let unaff_SI: *mut char;
    let mut unaff_DS: u16;

    pcVar1 = unaff_SI;
    unsafe {
        *pcVar1 = *pcVar1 + in_AH;
        pcVar1 = unaff_SI + in_BX;
        *pcVar1 = *pcVar1 + in_AL;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

// baddata();
//}

pub fn bad_fn_1050_180a() {
    let pbVar1: *mut byte;
    let pcVar2: *mut char;
    let puVar3: *mut u16;
    let puVar4: *mut u32;
    let piVar5: *mut i32;
    let uVar6: u8;
    let mut bVar7: u8;
    let mut bVar8: u8;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut bVar11: u8;
    let lVar12: u32;
    let mut bVar13: u8;
    let mut bVar14: u8;
    let mut bVar15: u8;
    let mut bVar16: u8;
    let mut bVar17: u8;
    let in_AX: *mut u16;
    let mut uVar18: u16;
    let mut uVar19: u16;
    let mut cVar20: u8;
    let mut in_CX: i32;
    let mut iVar21: i32;
    let mut bVar23: u8;
    let mut uVar22: u16;
    let mut bVar24: u8;
    let mut cVar25: u8;
    let mut in_DX: u16;
    let mut bVar27: u8;
    let mut uVar26: i32;
    let in_BX: *mut byte;
    let pbVar28: *mut byte;
    let piVar29: *mut i32;
    let mut uVar30: i32;
    char * *ppcVar31;
    char * *ppcVar32;
    char * *ppcVar33;
    i32 * *ppiVar34;
    i32 * *ppiVar35;
    i32 * *ppiVar36;
    let mut iVar37: i32;
    i32 * *ppiVar38;
    i32 * *ppiVar39;
    i32 * *ppiVar40;
    i32 * *ppiVar41;
    let unaff_BP: *mut u16;
    let mut uVar42: i32;
    let mut uVar43: i32;
    let puVar44: *mut u32;
    let mut uVar45: i32;
    let puVar46: *mut u32;
    let mut iVar47: i32;
    let mut uVar48: i32;
    let piVar49: *mut i32;
    let in_ESI: *mut u16;
    let unaff_DI: *mut u16;
    let puVar50: *mut u16;
    let puVar51: *mut u8;
    let puVar52: *mut u8;
    let puVar53: *mut u16;
    let mut iVar54: i32;
    let puVar55: *mut u16;
    let pcVar56: *mut char;
    let piVar57: *mut i32;
    let piVar58: *mut i32;
    let piVar59: *mut i32;
    let mut unaff_ES: u16;
    let mut unaff_SS: i32;
    let mut unaff_DS: u16;
    let mut in_FS: u16;
    let mut in_GS: u16;
    let mut in_CF: u8;
    let mut bVar60: bool;
    let mut bVar61: bool;
    let mut bVar62: bool;
    let mut in_AF: u8;
    let mut in_ZF: u8;
    let mut in_stack_00000070: u8;
    let mut in_stack_0000afbc: i32;
    let mut in_stack_0000afc2: i32;
    let mut in_stack_0000afc4: i32;
    let in_stack_0000afca: *mut u8;
    let in_stack_0000afcc: *mut u8;
    let mut in_stack_0000afce: i32;
    let in_stack_0000afd2: *mut u32;
    let mut in_stack_0000afd4: i32;
    let mut in_stack_0000afd6: u16;
    let mut in_stack_0000afd8: u8;

    puVar50 = &stack0xfffe;
    cVar25 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar50 = puVar50 + -1;
        unsafe { *puVar50 = *unaff_BP };
        cVar25 = cVar25 + -1;
        '\0' < cVar25
    } {}
    pbVar1 = 0x1050;
    bVar60 = unsafe { *pbVar1 < '\0' };
    unsafe { *pbVar1 = *pbVar1 << 1 | in_CF };
    iVar21 = in_CX + -1;
    iVar47 = in_ESI;
    bVar16 = (iVar21 >> 8);
    bVar15 = in_DX;
    bVar23 = (in_DX >> 8);
    bVar24 = (in_BX >> 8);
    if (iVar21 == 0 || in_ZF != '\0') {
        unsafe {
            bVar13 = in_AX + bVar16;
            bVar61 = CARRY1(in_AX, bVar16) || CARRY1(bVar13, bVar60);
            bVar13 = bVar13 + bVar60;
            in_stack_0000afc2 = in_AX & 0xff00 | bVar13;
            bVar60 = CARRY1(bVar13, bVar23) || CARRY1(bVar13 + bVar23, bVar61);
            bVar14 = bVar13 + bVar23 + bVar61;
            bVar13 = bVar14 + bVar24;
            bVar61 = CARRY1(bVar14, bVar24) || CARRY1(bVar13, bVar60);
            bVar13 = bVar13 + bVar60;
            in_AX = (in_AX & 0xff00 | bVar13);
            pbVar1 = in_BX + iVar47;
            bVar60 = CARRY1(*pbVar1, bVar13) || CARRY1(*pbVar1 + bVar13, bVar61);
            *pbVar1 = *pbVar1 + bVar13 + bVar61;
            pbVar1 = in_BX + iVar47;
            bVar13 = *pbVar1 + iVar21;
            bVar62 = CARRY1(*pbVar1, iVar21) || CARRY1(bVar13, bVar60);
            *pbVar1 = bVar13 + bVar60;
            pbVar1 = in_BX + iVar47;
            bVar61 = CARRY1(*pbVar1, bVar15) || CARRY1(*pbVar1 + bVar15, bVar62);
            *pbVar1 = *pbVar1 + bVar15 + bVar62;
            in_stack_0000afbc = unaff_SS;
            in_stack_0000afc4 = unaff_SS;
        }
    } else {
        unsafe {
            pbVar1 = in_BX;
            bVar61 = CARRY1(*pbVar1, bVar15) || CARRY1(*pbVar1 + bVar15, bVar60);
            *pbVar1 = *pbVar1 + bVar15 + bVar60;
        }
    }

    unsafe {
        pbVar1 = in_BX + iVar47;
        bVar15 = *pbVar1 + in_BX;
        bVar60 = CARRY1(*pbVar1, in_BX) || CARRY1(bVar15, bVar61);
        *pbVar1 = bVar15 + bVar61;
        pbVar1 = in_BX + unaff_DI;
        bVar15 = *pbVar1;
        bVar14 = (in_AX >> 8);
        bVar13 = *pbVar1;
        *pbVar1 = bVar13 + bVar14 + bVar60;
        if (SCARRY1(in_stack_00000070, bVar23)
            == SCARRY1(
                in_stack_00000070 + bVar23,
                CARRY1(bVar15, bVar14) || CARRY1(bVar13 + bVar14, bVar60),
            ))
        {
            puVar50 = (unaff_DI + 1);
            uVar6 = _in(in_DX);
            *unaff_DI = uVar6;
            pcVar2 = (iVar47 + 0x69);
            *pcVar2 = *pcVar2 + bVar14;
            if (-1 < *pcVar2) {
                pcVar2 = (iVar47 + 0x6f);
                *pcVar2 = *pcVar2 + bVar16;
                out(*in_ESI, in_DX);
                pcVar2 = 0x3035;
                *pcVar2 = *pcVar2 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                puVar3 = puVar50;
                *puVar3 = *puVar3 + bVar23;
                unaff_DI = (unaff_DI + 3);
                uVar22 = _in(in_DX);
                *puVar50 = uVar22;
                puVar3 = unaff_DI;
                *puVar3 = *puVar3 + bVar23;
                in_ESI = in_ESI + 1;
                // goto code_r0x105018a4;
            }
        } else {
            // code_r0x105018a4:
            puVar3 = unaff_DI;
            *puVar3 = *puVar3 + bVar23;
            puVar3 = unaff_DI;
            *puVar3 = *puVar3 + bVar23;
            puVar3 = unaff_DI;
            *puVar3 = *puVar3 + bVar23;
            puVar3 = unaff_DI;
            *puVar3 = *puVar3 + bVar23;
            puVar50 = unaff_DI;
        }
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar23;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar23;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar23;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar23;
        uVar22 = _in(in_DX);
        *puVar50 = uVar22;
        pcVar2 = 0x3230;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3330;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3430;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3530;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3630;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3834;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3830;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3930;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3031;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3131;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3231;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3331;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3431;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3531;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3631;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3731;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3831;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3931;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3032;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3132;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3232;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3332;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3432;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3532;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3632;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3732;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3832;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3932;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3033;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3133;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3233;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3333;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3433;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3533;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3633;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3733;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3833;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3933;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3034;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3234;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3334;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3434;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3534;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3634;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3734;
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = 0x3934;
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = in_BX + in_ESI;
        *pbVar1 = *pbVar1 + bVar24;
        pcVar2 = (in_stack_0000afc2 + in_stack_0000afbc);
        cVar25 = (in_stack_0000afc2 >> 8);
        *pcVar2 = *pcVar2 + cVar25;
        uVar22 = _in(in_stack_0000afc4);
        *in_AX = uVar22;
        pcVar2 = (in_stack_0000afc2 + in_stack_0000afbc);
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = (in_stack_0000afc2 + in_stack_0000afbc);
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = (in_stack_0000afd2 + in_stack_0000afcc);
        bVar13 = (in_stack_0000afd2 >> 8);
        *pcVar2 = *pcVar2 + bVar13;
        bVar15 = in_stack_0000afd8 ^ 0x39;
        puVar4 = (in_stack_0000afcc + 0x69);
        *puVar4 = *puVar4 ^ in_stack_0000afcc;
        uVar6 = _in(in_stack_0000afd4);
        *in_stack_0000afca = uVar6;
        pcVar2 = in_stack_0000afcc + 0x69;
        bVar24 = (in_stack_0000afd4 >> 8);
        *pcVar2 = *pcVar2 + bVar24;
        puVar50 = (in_stack_0000afca + 2);
        uVar6 = _in(in_stack_0000afd4);
        in_stack_0000afca[1] = uVar6;
        piVar29 = ((puVar50 + in_stack_0000afce + 0x6f) * 0x2e6e);
        pbVar1 = (in_stack_0000afd2 + in_stack_0000afcc + 0x6c);
        bVar16 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar24;
        puVar44 = (in_stack_0000afcc + 1);
        out(*in_stack_0000afcc, in_stack_0000afd4);
        bVar23 = (in_stack_0000afd6 >> 8);
        if (*pbVar1 == 0) {
            // code_r0x10501b31:
            uVar43 = puVar44 ^ 0x662e;
            uVar6 = _in(in_stack_0000afd4);
            *puVar50 = uVar6;
            puVar3 = (in_stack_0000afd2 + uVar43);
            *puVar3 = *puVar3;
            puVar44 = (uVar43 ^ *in_stack_0000afd2);
            uVar6 = _in(in_stack_0000afd4);
            *(puVar50 + 1) = uVar6;
            puVar3 = (in_stack_0000afd2 + puVar44);
            *puVar3 = *puVar3;
            puVar51 = ((puVar50 + 1) ^ (in_stack_0000afd2 + puVar44));
            puVar52 = puVar51 + 1;
            uVar6 = _in(in_stack_0000afd4);
            *puVar51 = uVar6;
            puVar3 = (in_stack_0000afd2 + puVar44);
            *puVar3 = *puVar3;
            puVar52 = (puVar52 ^ (in_stack_0000afd2 + puVar52));
            uVar6 = _in(in_stack_0000afd4);
            *puVar52 = uVar6;
            puVar3 = (in_stack_0000afd2 + puVar44);
            *puVar3 = *puVar3;
            uVar6 = _in(in_stack_0000afd4);
            puVar52[1] = uVar6;
            puVar3 = (in_stack_0000afd2 + puVar44);
            *puVar3 = *puVar3;
            bVar16 = bVar15 ^ 0x32;
            puVar53 = (puVar52 + 3);
            uVar6 = _in(in_stack_0000afd4);
            puVar52[2] = uVar6;
            puVar3 = (in_stack_0000afd2 + puVar44);
            *puVar3 = *puVar3;
        } else {
            puVar4 = in_stack_0000afd2 + 0x37;
            *puVar4 = *puVar4 + CARRY1(bVar16, bVar24) * ((in_stack_0000afce & 3) - (*puVar4 & 3));
            pcVar2 = (puVar50 + in_stack_0000afce + 0x69);
            *pcVar2 = *pcVar2 + bVar24;
            if (*pcVar2 == '\0') {
                in_AF = 9 < (bVar15 & 0xf) | in_AF;
                bVar15 = bVar15 + in_AF * '\x06' & 0xf;
                // goto code_r0x10501b31;
            }
            piVar29 = ((puVar50 + in_stack_0000afce + 0x6f) * 0x2e6e);
            pcVar2 = (in_stack_0000afd2 + puVar50);
            *pcVar2 = *pcVar2 + bVar24;
            pbVar1 = (in_stack_0000afce + puVar44);
            *pbVar1 = *pbVar1 ^ bVar24;
            pbVar1 = 0x6d62;
            *pbVar1 = *pbVar1 ^ bVar23;
            in_stack_0000afce = (in_stack_0000afce + 0x74) * 0x6564;
            puVar53 = (in_stack_0000afca + 4);
            uVar22 = _in(in_stack_0000afd4);
            *puVar50 = uVar22;
            bVar16 = in_stack_0000afd8 ^ 0x58;
            pcVar2 = (in_stack_0000afd2 + puVar53 + 0x6e);
            *pcVar2 = *pcVar2 + bVar23;
            if (*pcVar2 != '\0') {
                uVar22 = _in(in_stack_0000afd4);
                *puVar53 = uVar22;
                pcVar2 = (in_stack_0000afd2 + (in_stack_0000afca + 6));
                *pcVar2 = *pcVar2 + bVar24;
                pbVar1 = 0x6c66;
                *pbVar1 = *pbVar1 ^ bVar23;
                puVar3 = (in_stack_0000afd2 + puVar44);
                *puVar3 = *puVar3;
                in_stack_0000afd4 =
                    in_stack_0000afd4 & 0xff | (bVar24 ^ *(in_stack_0000afd2 + puVar44)) << 8;
                uVar6 = _in(in_stack_0000afd4);
                in_stack_0000afca[6] = uVar6;
                puVar3 = (in_stack_0000afd2 + puVar44);
                *puVar3 = *puVar3;
                uVar43 = puVar44 ^ (in_stack_0000afd2 + puVar44);
                uVar6 = _in(in_stack_0000afd4);
                in_stack_0000afca[7] = uVar6;
                puVar3 = (in_stack_0000afd2 + uVar43);
                *puVar3 = *puVar3;
                uVar43 = uVar43 ^ (in_stack_0000afce + uVar43);
                uVar6 = _in(in_stack_0000afd4);
                in_stack_0000afca[8] = uVar6;
                puVar3 = (in_stack_0000afd2 + uVar43);
                *puVar3 = *puVar3;
                puVar44 = (uVar43 ^ (in_stack_0000afca + 9 + in_stack_0000afce));
                uVar6 = _in(in_stack_0000afd4);
                in_stack_0000afca[9] = uVar6;
                puVar3 = (in_stack_0000afd2 + puVar44);
                *puVar3 = *puVar3;
                uVar43 = puVar44 ^ *puVar44;
                uVar6 = _in(in_stack_0000afd4);
                in_stack_0000afca[10] = uVar6;
                puVar3 = (in_stack_0000afd2 + uVar43);
                *puVar3 = *puVar3;
                puVar44 = (uVar43 ^ (in_stack_0000afca + 0xb));
                puVar50 = (in_stack_0000afca + 0xc);
                uVar6 = _in(in_stack_0000afd4);
                in_stack_0000afca[0xb] = uVar6;
                puVar3 = (in_stack_0000afd2 + puVar44);
                *puVar3 = *puVar3;
                // goto code_r0x10501b31;
            }
        }
        uVar6 = _in(in_stack_0000afd4);
        *puVar53 = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 1) = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 1) = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 3) = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 2) = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar11 = 9 < ((bVar16 ^ 5) & 0xf) | in_AF;
        bVar16 = (bVar16 ^ 5) + bVar11 * '\x06' & 0xf;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar11 = 9 < bVar16 | bVar11;
        uVar43 = (bVar16 + bVar11 * '\x06' & 0xf);
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar16 = *(in_stack_0000afd2 + puVar44);
        bVar27 = (in_stack_0000afd4 >> 8);
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 5) = uVar6;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (bVar16 < bVar27) * ((uVar43 & 3) - (*puVar4 & 3));
        bVar16 = *(in_stack_0000afce + puVar44);
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 3) = uVar6;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (bVar16 < bVar27) * ((uVar43 & 3) - (*puVar4 & 3));
        bVar16 = *puVar44;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 7) = uVar6;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (bVar16 < bVar27) * ((uVar43 & 3) - (*puVar4 & 3));
        bVar16 = *0x662e;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 4) = uVar6;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (bVar16 < bVar27) * ((uVar43 & 3) - (*puVar4 & 3));
        puVar46 = *(in_stack_0000afd2 + puVar44);
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 9) = uVar6;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (puVar46 < puVar44) * ((uVar43 & 3) - (*puVar4 & 3));
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 ^ puVar44;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + (puVar53 + 5));
        *puVar4 = *puVar4 ^ puVar44;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 5) = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + (puVar53 + 0xb));
        *puVar4 = *puVar4 ^ puVar44;
        puVar46 = puVar53 + 6;
        uVar6 = _in(in_stack_0000afd4);
        *(puVar53 + 0xb) = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar46);
        *puVar4 = *puVar4 ^ puVar44;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (*0x6c66 < bVar23) * (((uVar43 ^ 0x662e) & 3) - (*puVar4 & 3));
        puVar4 = (in_stack_0000afce + puVar46);
        *puVar4 = *puVar4 ^ puVar44;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = puVar44;
        *puVar4 = *puVar4 ^ puVar44;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = puVar46;
        *puVar4 = *puVar4 ^ puVar44;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = 0x2e30;
        *puVar4 = *puVar4 ^ puVar44;
        puVar52 = (puVar53 + 0xd);
        uVar6 = _in(in_stack_0000afd4);
        *puVar46 = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = in_stack_0000afd2;
        *puVar4 = *puVar4 ^ puVar44;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 ^ puVar52;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar52);
        *puVar4 = *puVar4 ^ puVar52;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar52);
        *puVar4 = *puVar4 ^ puVar52;
        puVar4 = 0x6c66;
        *puVar4 = *puVar4 ^ in_stack_0000afce;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar52);
        *puVar4 = *puVar4 ^ puVar52;
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar52);
        *puVar4 = *puVar4 ^ puVar52;
        uVar42 = in_stack_0000afce ^ 0x6c66;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar52);
        *puVar4 = *puVar4 ^ puVar52;
        bVar17 = (uVar43 ^ 0x662e);
        uVar45 = (bVar17 ^ 0x2e);
        puVar50 = puVar53 + 7;
        uVar6 = _in(in_stack_0000afd4);
        *puVar52 = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        puVar4 = (in_stack_0000afd2 + puVar50);
        *puVar4 = *puVar4 ^ puVar50;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (*0x6c66 < bVar23) * ((uVar45 & 3) - (*puVar4 & 3));
        puVar4 = (in_stack_0000afd2 + puVar50);
        *puVar4 = *puVar4 ^ puVar50;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (0x6c66 < uVar42) * ((uVar45 & 3) - (*puVar4 & 3));
        bVar16 = *(in_stack_0000afd2 + puVar44);
        uVar43 = in_stack_0000afd4 & 0xff;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar15 = *(in_stack_0000afd2 + puVar44);
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar24 = *(in_stack_0000afd2 + puVar44);
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (*0x6c66 < bVar23) * ((uVar45 & 3) - (*puVar4 & 3));
        bVar14 = *(in_stack_0000afd2 + puVar50);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar7 = *(in_stack_0000afd2 + puVar50);
        puVar4 = 0x6c66;
        *puVar4 = *puVar4 ^ uVar42;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar8 = *(uVar42 + puVar44);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar9 = *(uVar42 + puVar50);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar10 = *puVar44;
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar14 =
            bVar27 ^ bVar16 ^ bVar15 ^ bVar24 ^ bVar14 ^ bVar7 ^ bVar8 ^ bVar9 ^ bVar10 ^ *puVar44;
        pbVar28 = (puVar53 + 0xf);
        uVar6 = _in(uVar43 | bVar14 << 8);
        *puVar50 = uVar6;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar16 = *puVar44;
        puVar4 = (in_stack_0000afd2 + puVar44);
        *puVar4 = *puVar4 + (*0x6c66 < bVar23) * ((uVar45 & 3) - (*puVar4 & 3));
        bVar15 = *pbVar28;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar24 = *pbVar28;
        puVar4 = 0x6c66;
        *puVar4 = *puVar4 ^ uVar42;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar15 = bVar14 ^ bVar16 ^ bVar15 ^ bVar24 ^ *0x2e30;
        bVar16 = _in(uVar43 | bVar15 << 8);
        *pbVar28 = bVar16;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar15 = bVar15 ^ *in_stack_0000afd2;
        uVar43 = uVar43 | bVar15 << 8;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (in_stack_0000afd2 + puVar44);
        *puVar3 = *puVar3;
        bVar13 = bVar13 ^ *(in_stack_0000afd2 + puVar44);
        uVar26 = in_stack_0000afd2 & 0xff;
        uVar48 = uVar26 | bVar13 << 8;
        puVar52 = (puVar53 + 0x11);
        uVar6 = _in(uVar43);
        *(puVar53 + 8) = uVar6;
        puVar3 = (uVar48 + puVar44);
        *puVar3 = *puVar3;
        bVar13 = bVar13 ^ *(uVar48 + puVar44);
        uVar48 = uVar26 | bVar13 << 8;
        puVar4 = (uVar48 + puVar44);
        *puVar4 = *puVar4 + (*0x6c66 < bVar23) * ((uVar45 & 3) - (*puVar4 & 3));
        bVar13 = bVar13 ^ puVar52[uVar48];
        uVar45 = uVar26 | bVar13 << 8;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar45 + puVar44);
        *puVar3 = *puVar3;
        bVar13 = bVar13 ^ puVar52[uVar45];
        uVar45 = uVar26 | bVar13 << 8;
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (uVar45 + puVar44);
        *puVar3 = *puVar3;
        uVar26 = uVar26 | (bVar13 ^ puVar52[uVar45]) << 8;
        uVar6 = _in(uVar43);
        *puVar52 = uVar6;
        puVar3 = (uVar26 + puVar44);
        *puVar3 = *puVar3;
        uVar45 = puVar44 ^ (uVar26 + puVar44);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ (uVar26 + uVar45);
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ (uVar26 + uVar45);
        uVar6 = _in(uVar43);
        *(puVar53 + 9) = uVar6;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ (uVar26 + uVar45);
        puVar44 = puVar53 + 10;
        uVar6 = _in(uVar43);
        *(puVar53 + 0x13) = uVar6;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ (uVar26 + puVar44);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        puVar46 = (uVar45 ^ (uVar42 + puVar44));
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + puVar46);
        *puVar3 = *puVar3;
        puVar46 = (puVar46 ^ *puVar46);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + puVar46);
        *puVar3 = *puVar3;
        uVar45 = puVar46 ^ *puVar46;
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ *puVar44;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ *puVar44;
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar45 = uVar45 ^ 0x2e30;
        uVar6 = _in(uVar43);
        *puVar44 = uVar6;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar43 = (puVar53 + 0x15) ^ (uVar26 + uVar45);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar43 = uVar43 ^ (uVar26 + uVar43);
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar43 = uVar43 ^ (uVar26 + uVar43);
        bVar23 = bVar23 ^ *0x6c66;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar43 = uVar43 ^ (uVar26 + uVar43);
        puVar4 = (uVar26 + uVar45);
        *puVar4 = *puVar4 + (*0x6c66 < bVar23) * (((bVar17 ^ 0x2e) & 3) - (*puVar4 & 3));
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        pbVar1 = 0x6c66;
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        puVar4 = 0x6c66;
        *puVar4 = *puVar4 ^ uVar42;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        uVar42 = uVar42 ^ 0x6c66;
        puVar3 = (uVar26 + uVar45);
        *puVar3 = *puVar3;
        puVar4 = (uVar26 + uVar45);
        *puVar4 = *puVar4 ^ uVar45;
        pcVar2 = (uVar26 + uVar43);
        *pcVar2 = *pcVar2 + bVar15;
        pbVar1 = (uVar42 + uVar43);
        *pbVar1 = *pbVar1 ^ bVar15;
        pcVar2 = (uVar26 + uVar43);
        *pcVar2 = *pcVar2 + bVar15;
        pbVar1 = (uVar42 + uVar43);
        *pbVar1 = *pbVar1 ^ bVar15;
        iVar54 = *piVar29;
        iVar47 = piVar29[1];
        iVar21 = piVar29[2];
        pbVar28 = piVar29[4];
        uVar18 = piVar29[7];
        pbVar1 = pbVar28 + iVar54;
        bVar23 = (piVar29[5] >> 8);
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = (iVar21 + iVar54);
        *pbVar1 = *pbVar1 ^ bVar23;
        uVar30 = (piVar29 + 8) ^ (pbVar28 + iVar54 + 0x2e);
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = pbVar28;
        *pbVar1 = *pbVar1 ^ bVar23;
        pbVar1 = pbVar28 + iVar54 + 0x2e;
        bVar15 = (uVar18 >> 8);
        *pbVar1 = *pbVar1 ^ bVar15;
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        bVar23 = bVar23 ^ *0x6130;
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        bVar23 = bVar23 ^ *pbVar28;
        pbVar1 = pbVar28 + iVar54 + 0x2e;
        *pbVar1 = *pbVar1 ^ bVar15;
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = pbVar28 + iVar47;
        bVar24 = (pbVar28 >> 8);
        *pbVar1 = *pbVar1 ^ bVar24;
        bVar16 = pbVar28[iVar54 + 0x2e];
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 ^ bVar24;
        pbVar1 = pbVar28 + iVar54 + 0x2e;
        *pbVar1 = *pbVar1 ^ bVar15 ^ bVar16;
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = (iVar21 + iVar54);
        *pbVar1 = *pbVar1 ^ bVar23;
        pbVar1 = pbVar28 + iVar54;
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = (iVar21 + iVar54);
        *pbVar1 = *pbVar1 ^ bVar23;
        puVar55 = uVar30;
        puVar46 = *(uVar30 + 2);
        iVar21 = (uVar30 + 4);
        uVar45 = (uVar30 + 8);
        uVar26 = (uVar30 + 10);
        uVar22 = (uVar30 + 0xc);
        uVar19 = (uVar30 + 0xe);
        pcVar2 = (uVar45 + puVar55);
        bVar13 = (uVar26 >> 8);
        *pcVar2 = *pcVar2 + bVar13;
        pbVar1 = (iVar21 + puVar55);
        *pbVar1 = *pbVar1 ^ bVar13;
        pcVar2 = (uVar45 + puVar55);
        *pcVar2 = *pcVar2 + bVar13;
        puVar4 = (uVar45 + puVar55);
        *puVar4 = *puVar4 ^ puVar46;
        pbVar1 = 0x6d62;
        bVar24 = (uVar22 >> 8);
        *pbVar1 = *pbVar1 ^ bVar24;
        puVar4 = (uVar45 + puVar55);
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = (uVar45 + puVar55);
        *puVar4 = *puVar4 ^ puVar55;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = (uVar45 + puVar46);
        *pbVar1 = *pbVar1 ^ bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = (uVar45 + puVar55);
        *pbVar1 = *pbVar1 ^ bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = (iVar21 + puVar46);
        *pbVar1 = *pbVar1 ^ bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = (iVar21 + puVar55);
        *pbVar1 = *pbVar1 ^ bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        puVar4 = puVar46;
        *puVar4 = *puVar4 ^ bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 ^ bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = 0x622e;
        *pbVar1 = *pbVar1 ^ bVar13;
        puVar44 = puVar55 + 1;
        uVar22 = _in(uVar26);
        *puVar55 = uVar22;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        pbVar1 = (uVar45 + puVar46);
        bVar16 = (uVar45 >> 8);
        *pbVar1 = *pbVar1 ^ bVar16;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        pbVar1 = (uVar45 + puVar44);
        *pbVar1 = *pbVar1 ^ bVar16;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = (uVar45 + puVar46);
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = (uVar45 + puVar44);
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = (iVar21 + puVar46);
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = (iVar21 + puVar44);
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = puVar46;
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = puVar44;
        *puVar4 = *puVar4 ^ puVar46;
        puVar4 = puVar44;
        *puVar4 = *puVar4 + bVar13;
        puVar4 = 0x622e;
        *puVar4 = *puVar4 ^ puVar46;
        puVar50 = puVar55 + 2;
        uVar43 = _in(uVar26);
        *puVar44 = uVar43;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        puVar4 = (uVar45 + puVar46);
        *puVar4 = *puVar4 ^ puVar50;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        puVar4 = (uVar45 + puVar50);
        *puVar4 = *puVar4 ^ puVar50;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *(uVar45 + puVar46);
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *(uVar45 + puVar50);
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *(iVar21 + puVar46);
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *(iVar21 + puVar50);
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *puVar46;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *puVar50;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar13 = bVar13 ^ *0x622e;
        uVar26 = uVar26 & 0xff | bVar13 << 8;
        puVar53 = puVar55 + 3;
        uVar22 = _in(uVar26);
        *puVar50 = uVar22;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        uVar45 = uVar45 & 0xff | (bVar16 ^ *(uVar45 + puVar46)) << 8;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        uVar48 = puVar46 ^ (uVar45 + puVar53);
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        uVar43 = (iVar21 + uVar48);
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = 0x6d62;
        *pbVar1 = *pbVar1 ^ bVar24;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        bVar16 = *0x6d62;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar53;
        *puVar3 = *puVar3 + bVar13;
        puVar50 = puVar55 + 4;
        uVar22 = _in(uVar26);
        *puVar53 = uVar22;
        bVar23 = uVar19 ^ 5;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar15 = (9 < (bVar23 & 0xf)) | (9 < ((uVar18 ^ 0x61) & 0xf)) | bVar11;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar23 = bVar23 + bVar15 * '\x06' & 0xf ^ 0x36;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar15 = 9 < (bVar23 & 0xf) | bVar15;
        pbVar1 = 0x6d62;
        *pbVar1 = *pbVar1 ^ bVar24 ^ bVar16;
        bVar16 = bVar23 + bVar15 * '\x06' & 0xf ^ 0x37;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar15 = 9 < (bVar16 & 0xf) | bVar15;
        bVar16 = bVar16 + bVar15 * '\x06' & 0xf ^ 0x37;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar15 = 9 < (bVar16 & 0xf) | bVar15;
        bVar16 = bVar16 + bVar15 * '\x06' & 0xf ^ 0x2e;
        puVar3 = puVar50;
        *puVar3 = *puVar3 + bVar13;
        bVar15 = 9 < (bVar16 & 0xf) | bVar15;
        puVar55 = puVar55 + 5;
        uVar22 = _in(uVar26);
        *puVar50 = uVar22;
        bVar16 = bVar16 + bVar15 * '\x06' & 0xf ^ 0x19;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        bVar15 = 9 < (bVar16 & 0xf) | bVar15;
        bVar16 = bVar16 + bVar15 * '\x06' & 0xf;
        bVar15 = 9 < bVar16 | bVar15;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        puVar3 = puVar55;
        *puVar3 = *puVar3 + bVar13;
        pbVar1 = (uVar45 + (uVar48 ^ uVar43));
        *pbVar1 = *pbVar1 ^ bVar13;
        pcVar56 = *(uVar30 + 0x10);
        iVar21 = (uVar30 + 0x18);
        pcVar2 = pcVar56;
        bVar23 = (*(uVar30 + 0x1a) >> 8);
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = (pcVar56 + iVar21);
        *pbVar1 = *pbVar1 ^ bVar23;
        iVar47 = (uVar30 + 0x22);
        iVar21 = (uVar30 + 0x24);
        pcVar2 = *(uVar30 + 0x20);
        bVar23 = (*(uVar30 + 0x2a) >> 8);
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = (iVar21 + iVar47);
        *pbVar1 = *pbVar1 ^ bVar23;
        pcVar56 = *(uVar30 + 0x30);
        iVar21 = (uVar30 + 0x34);
        pcVar2 = pcVar56;
        bVar23 = (*(uVar30 + 0x3a) >> 8);
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = (pcVar56 + iVar21);
        *pbVar1 = *pbVar1 ^ bVar23;
        pbVar28 = (uVar30 + 0x42);
        pcVar2 = *(uVar30 + 0x40);
        bVar23 = (*(uVar30 + 0x4a) >> 8);
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = pbVar28;
        *pbVar1 = *pbVar1 ^ bVar23;
        pbVar28 = (uVar30 + 0x50);
        pbVar1 = pbVar28;
        bVar23 = (*(uVar30 + 0x5a) >> 8);
        *pbVar1 = *pbVar1 + bVar23;
        pbVar1 = pbVar28;
        *pbVar1 = *pbVar1 ^ bVar23;
        pcVar56 = *(uVar30 + 0x60);
        pbVar28 = (uVar30 + 0x68);
        pcVar2 = pcVar56;
        bVar23 = (*(uVar30 + 0x6a) >> 8);
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = 0x2e61;
        *pbVar1 = *pbVar1 ^ bVar23;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + bVar23;
        pbVar1 = pbVar28;
        *pbVar1 = *pbVar1 ^ bVar23;
        iVar47 = (uVar30 + 0x72);
        iVar21 = (uVar30 + 0x78);
        pcVar2 = *(uVar30 + 0x70);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x7a) >> 8);
        pbVar1 = (iVar21 + iVar47);
        *pbVar1 = *pbVar1 ^ (iVar21 >> 8);
        pcVar56 = *(uVar30 + 0x80);
        iVar21 = (uVar30 + 0x88);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x8a) >> 8);
        pbVar1 = (pcVar56 + iVar21);
        *pbVar1 = *pbVar1 ^ (iVar21 >> 8);
        uVar43 = (uVar30 + 0x92);
        iVar21 = (uVar30 + 0x98);
        pcVar2 = *(uVar30 + 0x90);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x9a) >> 8);
        puVar4 = (iVar21 + uVar43);
        *puVar4 = *puVar4 ^ uVar43;
        pcVar56 = *(uVar30 + 0xa0);
        uVar43 = (uVar30 + 0xa2);
        iVar21 = (uVar30 + 0xa8);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + (*(uVar30 + 0xaa) >> 8);
        puVar4 = (pcVar56 + iVar21);
        *puVar4 = *puVar4 ^ uVar43;
        uVar43 = (uVar30 + 0xb2);
        iVar21 = (uVar30 + 0xb4);
        pcVar2 = *(uVar30 + 0xb0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0xba) >> 8);
        puVar4 = (iVar21 + uVar43);
        *puVar4 = *puVar4 ^ uVar43;
        pcVar56 = *(uVar30 + 0xc0);
        uVar43 = (uVar30 + 0xc2);
        iVar21 = (uVar30 + 0xc4);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + (*(uVar30 + 0xca) >> 8);
        puVar4 = (pcVar56 + iVar21);
        *puVar4 = *puVar4 ^ uVar43;
        puVar44 = *(uVar30 + 0xd2);
        pcVar2 = *(uVar30 + 0xd0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0xda) >> 8);
        puVar4 = puVar44;
        *puVar4 = *puVar4 ^ puVar44;
        puVar44 = *(uVar30 + 0xe0);
        uVar43 = (uVar30 + 0xe2);
        puVar4 = puVar44;
        *puVar4 = *puVar4 + (*(uVar30 + 0xea) >> 8);
        puVar4 = puVar44;
        *puVar4 = *puVar4 ^ uVar43;
        pcVar56 = *(uVar30 + 0xf0);
        uVar43 = (uVar30 + 0xf2);
        puVar44 = *(uVar30 + 0xf8);
        pcVar2 = pcVar56;
        cVar25 = (*(uVar30 + 0xfa) >> 8);
        *pcVar2 = *pcVar2 + cVar25;
        puVar4 = 0x2e61;
        *puVar4 = *puVar4 ^ uVar43;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        puVar4 = puVar44;
        *puVar4 = *puVar4 ^ uVar43;
        pcVar56 = *(uVar30 + 0x100);
        iVar47 = (uVar30 + 0x102);
        iVar21 = (uVar30 + 0x108);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x10a) >> 8);
        puVar4 = (iVar21 + iVar47);
        *puVar4 = *puVar4 ^ pcVar56;
        pcVar56 = *(uVar30 + 0x110);
        iVar21 = (uVar30 + 0x118);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x11a) >> 8);
        puVar4 = (pcVar56 + iVar21);
        *puVar4 = *puVar4 ^ pcVar56;
        pcVar2 = *(uVar30 + 0x120);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x12a) >> 8);
        pcVar2 = *(uVar30 + 0x130);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x13a) >> 8);
        pcVar2 = *(uVar30 + 0x140);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x14a) >> 8);
        pcVar2 = *(uVar30 + 0x150);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x15a) >> 8);
        pcVar2 = *(uVar30 + 0x160);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x16a) >> 8);
        pcVar2 = *(uVar30 + 0x170);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x17a) >> 8);
        pcVar56 = *(uVar30 + 0x180);
        pcVar2 = pcVar56;
        bVar23 = (*(uVar30 + 0x18a) >> 8);
        *pcVar2 = *pcVar2 + bVar23;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + (bVar23 ^ *0x2e61);
        pcVar2 = *(uVar30 + 400);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x19a) >> 8);
        pcVar2 = *(uVar30 + 0x1a0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x1aa) >> 8);
        pcVar2 = *(uVar30 + 0x1b0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x1ba) >> 8);
        pcVar2 = *(uVar30 + 0x1c0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x1ca) >> 8);
        pcVar2 = *(uVar30 + 0x1d0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x1da) >> 8);
        pcVar2 = *(uVar30 + 0x1e0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x1ea) >> 8);
        pcVar2 = *(uVar30 + 0x1f0);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x1fa) >> 8);
        pcVar2 = *(uVar30 + 0x200);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x20a) >> 8);
        pcVar2 = *(uVar30 + 0x210);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x21a) >> 8);
        pcVar2 = *(uVar30 + 0x220);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x22a) >> 8);
        pcVar2 = *(uVar30 + 0x230);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x23a) >> 8);
        pcVar2 = *(uVar30 + 0x240);
        *pcVar2 = *pcVar2 + (*(uVar30 + 0x24a) >> 8);
        pcVar56 = *(uVar30 + 0x250);
        iVar21 = (uVar30 + 600);
        uVar22 = (uVar30 + 0x25e);
        pcVar2 = pcVar56;
        cVar25 = (*(uVar30 + 0x25a) >> 8);
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pbVar1 = (pcVar56 + iVar21 + 0x2e);
        *pbVar1 = *pbVar1 ^ (uVar22 >> 8);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        puVar4 = (pcVar56 + iVar21 + 0x2e);
        *puVar4 = *puVar4 ^ uVar30 + 0x260;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        ppcVar31 = (uVar30 + 0x260 ^ (pcVar56 + iVar21 + 0x2e));
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        uVar22 = ppcVar31[7];
        pcVar2 = *ppcVar31;
        *pcVar2 = *pcVar2 + (ppcVar31[5] >> 8);
        pcVar56 = ppcVar31[8];
        iVar21 = ppcVar31[0xc];
        uVar18 = ppcVar31[0xf];
        pcVar2 = pcVar56;
        cVar25 = (ppcVar31[0xd] >> 8);
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        bVar23 = uVar18;
        bVar16 = (9 < (bVar23 & 0xf))
            | (9 < (uVar22 & 0xf))
            | (9 < (bVar16 + bVar15 * '\x06' & 0xf))
            | bVar15;
        bVar15 = bVar23 + bVar16 * '\x06' & 0xf;
        pbVar1 = (pcVar56 + iVar21 + 0x2e);
        *pbVar1 = *pbVar1 ^ (uVar18 >> 8) + bVar16;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        bVar16 = 9 < bVar15 | bVar16;
        bVar15 = bVar15 + bVar16 * '\x06' & 0xf;
        puVar4 = (pcVar56 + iVar21 + 0x2e);
        *puVar4 = *puVar4 ^ (ppcVar31 + 0x10);
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        bVar16 = 9 < bVar15 | bVar16;
        bVar15 = bVar15 + bVar16 * '\x06' & 0xf;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        bVar16 = 9 < bVar15 | bVar16;
        bVar15 = bVar15 + bVar16 * '\x06' & 0xf;
        ppcVar32 = ((ppcVar31 + 0x10) ^ (pcVar56 + iVar21 + 0x2e));
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        bVar16 = 9 < bVar15 | bVar16;
        bVar15 = bVar15 + bVar16 * '\x06' & 0xf ^ 0x61;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        bVar16 = 9 < (bVar15 & 0xf) | bVar16;
        pcVar2 = pcVar56;
        *pcVar2 = *pcVar2 + cVar25;
        uVar22 = ppcVar32[7];
        pcVar2 = *ppcVar32;
        *pcVar2 = *pcVar2 + (ppcVar32[5] >> 8);
        bVar23 = uVar22;
        bVar16 =
            (9 < (bVar23 & 0xf)) | (9 < ((bVar15 + bVar16 * '\x06' & 0xf ^ 0x61) & 0xf)) | bVar16;
        piVar57 = ppcVar32[8];
        puVar52 = ppcVar32[9];
        uVar48 = ppcVar32[10];
        pbVar28 = ppcVar32[0xc];
        uVar45 = ppcVar32[0xd];
        uVar26 = ppcVar32[0xe];
        uVar22 = ppcVar32[0xf];
        ppiVar36 = (ppcVar32 + 0x10);
        ppiVar38 = (ppcVar32 + 0x10);
        ppiVar35 = (ppcVar32 + 0x10);
        ppiVar34 = (ppcVar32 + 0x10);
        piVar5 = piVar57;
        bVar24 = (uVar45 >> 8);
        *piVar5 = *piVar5 + bVar24;
        bVar15 = uVar22;
        bVar16 = (9 < (bVar15 & 0xf)) | (9 < (bVar23 + bVar16 * '\x06' & 0xf)) | bVar16;
        bVar23 = bVar15 + bVar16 * '\x06' & 0xf;
        piVar5 = piVar57;
        *piVar5 = *piVar5 + bVar24;
        bVar15 = 9 < bVar23 | bVar16;
        uVar43 = CONCAT11((uVar22 >> 8) + bVar16 + bVar15, bVar23 + bVar15 * '\x06') & 0xff0f;
        pbVar1 = (pbVar28 + puVar52 + 0x6c);
        bVar60 = CARRY1(*pbVar1, bVar24);
        *pbVar1 = *pbVar1 + bVar24;
        bVar61 = *pbVar1 == 0;
        piVar49 = (puVar52 + 1);
        out(*puVar52, uVar45);
        if (bVar61) {
            // code_r0x10502336:
            piVar5 = piVar57;
            piVar57 = (piVar57 + 1);
            uVar6 = _in(uVar45);
            *piVar5 = uVar6;
            if (bVar60 || bVar61) {
                // code_r0x1050239f:
                ppiVar38 = ppiVar34;
                if (bVar60) {}
                // goto code_r0x105023a1;
                // code_r0x10502410:
                pbVar1 = (piVar57 + uVar48 + 0x6d);
                bVar16 = *pbVar1;
                bVar23 = (uVar45 >> 8);
                *pbVar1 = *pbVar1 + bVar23;
                piVar5 = piVar57;
                piVar57 = (piVar57 + 1);
                uVar6 = _in(uVar45);
                *piVar5 = uVar6;
                if (!CARRY1(bVar16, bVar23)) {
                    pcVar2 = (uVar48 + piVar49);
                    *pcVar2 = *pcVar2 + bVar23;
                    uVar45 = uVar45 & 0xff | (bVar23 ^ *pbVar28) << 8;
                    // goto code_r0x10502495;
                }
                // code_r0x10502418:
                iVar21 = _in(uVar45);
                *piVar57 = iVar21;
                pbVar1 = (uVar48 + (piVar57 + 1));
                *pbVar1 = *pbVar1 ^ (uVar45 >> 8);
                pcVar2 = (piVar57 + 0x37);
                *pcVar2 = *pcVar2 + (uVar26 >> 8);
                piVar58 = piVar57 + 2;
                iVar21 = _in(uVar45);
                piVar57[1] = iVar21;
                // code_r0x1050242a:
                piVar5 = piVar58;
                *piVar5 = *piVar5 ^ (uVar45 >> 8);
                pcVar2 = (piVar58 + 0x35);
                *pcVar2 = *pcVar2 + (uVar26 >> 8);
                bVar15 = 9 < (uVar43 & 0xf) | bVar15;
                uVar43 = CONCAT11((uVar43 >> 8) + bVar15, uVar43 + bVar15 * '\x06') & 0xff0f;
                // code_r0x10502433:
                pcVar2 = (piVar58 + 0x35);
                *pcVar2 = *pcVar2 + (uVar26 >> 8);
                ppiVar36 = ppiVar34;
                piVar57 = piVar58;
                // code_r0x10502436:
                pbVar1 = pbVar28 + piVar49;
                bVar16 = uVar43;
                bVar60 = *pbVar1 < bVar16;
                bVar61 = *pbVar1 == bVar16;
                if (bVar61) {}
                // goto code_r0x105024a5;
                if (bVar61) {}
                // goto code_r0x10502443;
            } else {
                // code_r0x1050233a:
                uVar6 = _in(uVar45);
                *piVar57 = uVar6;
                pbVar1 = (piVar49 + 0x65);
                bVar16 = (uVar26 >> 8);
                bVar60 = CARRY1(*pbVar1, bVar16);
                *pbVar1 = *pbVar1 + bVar16;
                ppiVar38 = ppiVar35;
                if (bVar60 || *pbVar1 == 0) {
                    // code_r0x105023a9:
                    puVar4 = (piVar49 + uVar48 + 0x65);
                    *puVar4 = *puVar4 + bVar60 * ((piVar49 & 3) - (*puVar4 & 3));
                    puVar52 = (ppiVar38 + 2);
                    uVar48 = (ppiVar38 + 4);
                    pbVar28 = (ppiVar38 + 8);
                    uVar45 = (ppiVar38 + 10);
                    uVar26 = (ppiVar38 + 0xc);
                    uVar43 = (ppiVar38 + 0xe);
                    ppiVar35 = (ppiVar38 + 0x10);
                    ppiVar34 = (ppiVar38 + 0x10);
                    piVar58 = *ppiVar38 + 1;
                    uVar22 = _in(uVar45);
                    **ppiVar38 = uVar22;
                    bVar15 = 9 < (uVar43 & 0xf) | bVar15;
                    bVar16 = uVar43 + bVar15 * '\x06';
                    bVar23 = (uVar43 >> 8);
                    uVar43 = uVar43 & 0xff00
                        | (bVar16
                            + (0x90 < (bVar16 & 0xf0) | bVar60 | bVar15 * (0xf9 < bVar16)) * '`');
                    piVar49 = (puVar52 + 1);
                    out(*puVar52, uVar45);
                    pbVar1 = (piVar58 + uVar48 + 0x68);
                    bVar16 = *pbVar1;
                    *pbVar1 = *pbVar1 + bVar23;
                    if (CARRY1(bVar16, bVar23)) {}
                    // goto code_r0x1050242a;
                    // code_r0x105023b7:
                    pbVar1 = (pbVar28 + piVar58 + 0x6e);
                    bVar16 = (uVar43 >> 8);
                    bVar60 = CARRY1(*pbVar1, bVar16);
                    *pbVar1 = *pbVar1 + bVar16;
                    bVar61 = *pbVar1 == 0;
                    ppiVar36 = ppiVar35;
                    piVar57 = piVar58;
                    if (bVar60) {}
                    // goto code_r0x10502436;
                    piVar59 = piVar58 + 1;
                    iVar21 = _in(uVar45);
                    *piVar58 = iVar21;
                    out(*piVar49, uVar45);
                    // code_r0x105023bf:
                    uVar6 = _in(uVar45);
                    *piVar59 = uVar6;
                    piVar58 = *ppiVar35;
                    piVar49 = ppiVar35[1];
                    uVar48 = ppiVar35[2];
                    pbVar28 = ppiVar35[4];
                    uVar45 = ppiVar35[5];
                    uVar26 = ppiVar35[6];
                    uVar43 = ppiVar35[7];
                    ppiVar34 = ppiVar35 + 8;
                    ppiVar40 = ppiVar35 + 8;
                    if (!bVar61) {}
                    // goto code_r0x10502433;
                    puVar4 = (pbVar28 + piVar49);
                    iVar21 = (uVar48 & 3) - (*puVar4 & 3);
                    bVar61 = 0 < iVar21;
                    *puVar4 = *puVar4 + bVar60 * iVar21;
                    if (!bVar60) {
                        uVar6 = _in(uVar45);
                        *piVar58 = uVar6;
                        piVar57 = ppiVar35[8];
                        piVar49 = ppiVar35[9];
                        uVar48 = ppiVar35[10];
                        pbVar28 = ppiVar35[0xc];
                        uVar45 = ppiVar35[0xd];
                        uVar26 = ppiVar35[0xe];
                        ppiVar36 = ppiVar35 + 0x10;
                        // code_r0x105023cf:
                        puVar50 = (piVar49 + 1);
                        out(*piVar49, uVar45);
                        uVar43 = (pbVar28 + puVar50) * 0x6f62;
                        piVar49 = (piVar49 + 3);
                        out(*puVar50, uVar45);
                        if (bVar61) {
                            pbVar1 = (piVar49 + uVar48 + 0x6f);
                            bVar16 = (uVar45 >> 8);
                            bVar60 = CARRY1(*pbVar1, bVar16);
                            *pbVar1 = *pbVar1 + bVar16;
                            // goto code_r0x10502445;
                        }
                        ppiVar35 = ppiVar36;
                        if (bVar61) {}
                        // goto code_r0x105023de;
                        // goto code_r0x10502450;
                    }
                    pcVar2 = (uVar48 + 0x69);
                    *pcVar2 = *pcVar2 + (uVar43 >> 8);
                    // code_r0x10502401:
                    pbVar1 = (uVar48 + 0x69);
                    bVar16 = *pbVar1;
                    bVar23 = (uVar43 >> 8);
                    *pbVar1 = *pbVar1 + bVar23;
                    if (!CARRY1(bVar16, bVar23)) {
                        puVar52 = ppiVar40[1];
                        uVar48 = ppiVar40[2];
                        pbVar28 = ppiVar40[4];
                        uVar45 = ppiVar40[5];
                        uVar26 = ppiVar40[6];
                        uVar43 = ppiVar40[7];
                        ppiVar36 = ppiVar40 + 8;
                        piVar57 = (*ppiVar40 + 1);
                        uVar6 = _in(uVar45);
                        **ppiVar40 = uVar6;
                        pbVar1 = puVar52 + uVar48 + 0x65;
                        bVar16 = (uVar45 >> 8);
                        bVar60 = CARRY1(*pbVar1, bVar16);
                        *pbVar1 = *pbVar1 + bVar16;
                        bVar16 = *pbVar1;
                        out(*puVar52, uVar45);
                        piVar58 = (puVar52 + 1);
                        // goto code_r0x1050240e;
                    }
                    // code_r0x10502469:
                    puVar4 = (pbVar28 + piVar49);
                    *puVar4 = *puVar4 ^ uVar43;
                    // code_r0x1050246e:
                    ppiVar41 = (ppiVar40 + 2);
                    uVar43 = uVar43 & 0xff00 | (uVar43 ^ pbVar28[piVar49]);
                    // goto code_r0x10502474;
                }
                uVar6 = _in(uVar45);
                *(piVar57 + 1) = uVar6;
                bVar16 = bVar16 ^ *0x6d62;
                uVar26 = uVar26 & 0xff | bVar16 << 8;
                piVar58 = (piVar57 + 3);
                uVar6 = _in(uVar45);
                *(piVar57 + 1) = uVar6;
                if (bVar16 == 0) {}
                // goto code_r0x105023b7;
                uVar6 = _in(uVar45);
                *piVar58 = uVar6;
                bVar60 = false;
                uVar48 = uVar48 ^ 0x6d62;
                bVar61 = uVar48 == 0;
                piVar59 = (piVar57 + 5);
                uVar6 = _in(uVar45);
                *(piVar57 + 2) = uVar6;
                if (bVar61) {}
                // goto code_r0x105023bf;
                piVar58 = piVar57 + 3;
                uVar6 = _in(uVar45);
                *piVar59 = uVar6;
                uVar43 = uVar43 ^ 0x2e;
                // code_r0x10502360:
                pbVar1 = (piVar58 + 0x61);
                bVar16 = (uVar26 >> 8);
                bVar60 = CARRY1(*pbVar1, bVar16);
                *pbVar1 = *pbVar1 + bVar16;
                bVar16 = *pbVar1;
                if (!bVar60) {
                    piVar57 = piVar58 + 1;
                    iVar21 = _in(uVar45);
                    *piVar58 = iVar21;
                    lVar12 = *piVar49 * 0x7730;
                    ppiVar36 = lVar12;
                    if (ppiVar36 == lVar12) {
                        pcVar2 = (piVar49 + uVar48 + 0x6f);
                        *pcVar2 = *pcVar2 + (uVar43 >> 8);
                        bVar61 = *pcVar2 == '\0';
                        // goto code_r0x105023cf;
                    }
                    iVar21 = _in(uVar45);
                    *piVar57 = iVar21;
                    lVar12 = *piVar49 * 0x6c61;
                    iVar37 = lVar12;
                    bVar60 = iVar37 != lVar12;
                    uVar42 = (iVar37 + 0xe);
                    if (!bVar60) {
                        uVar6 = _in((iVar37 + 0x1a));
                        *(iVar37 + 0x10) = uVar6;
                        piVar57 = (iVar37 + 0x20);
                        piVar49 = (iVar37 + 0x22);
                        uVar48 = (iVar37 + 0x24);
                        pbVar28 = (iVar37 + 0x28);
                        uVar45 = (iVar37 + 0x2a);
                        uVar26 = (iVar37 + 0x2c);
                        uVar43 = (iVar37 + 0x2e);
                        if (uVar42 < 0x6d) {
                            pcVar2 = (piVar49 + uVar48 + 0x6f);
                            *pcVar2 = *pcVar2 + (uVar45 >> 8);
                            bVar61 = *pcVar2 == '\0';
                        } else {
                            bVar60 = 99 < uVar43;
                            uVar6 = _in((iVar37 + 0x3a));
                            *(iVar37 + 0x30) = uVar6;
                            piVar57 = (iVar37 + 0x40);
                            piVar49 = (iVar37 + 0x42);
                            uVar48 = (iVar37 + 0x44);
                            pbVar28 = (iVar37 + 0x48);
                            uVar45 = (iVar37 + 0x4a);
                            uVar26 = (iVar37 + 0x4c);
                            uVar43 = (iVar37 + 0x4e);
                            ppiVar40 = (iVar37 + 0x50);
                            ppiVar36 = (iVar37 + 0x50);
                            if (bVar60) {
                                bVar16 = uVar43 ^ pbVar28[piVar49];
                                uVar43 = uVar43 & 0xff00 | bVar16;
                                if (bVar16 < '\0') {}
                                // goto code_r0x10502401;
                                piVar5 = piVar57;
                                piVar57 = (piVar57 + 1);
                                uVar6 = _in(uVar45);
                                *piVar5 = uVar6;
                                // goto code_r0x10502392;
                            }
                            pcVar2 = (pbVar28 + piVar49 + 0x6c);
                            *pcVar2 = *pcVar2 + (uVar45 >> 8);
                            bVar61 = *pcVar2 == '\0';
                        }
                        // goto code_r0x105023f4;
                    }
                    puVar52 = (iVar37 + 0x10);
                    uVar22 = (iVar37 + 0x1a);
                    ppiVar39 = (iVar37 + 0x20);
                    ppiVar36 = (iVar37 + 0x20);
                    if (!bVar60 && bVar16 != 0) {}
                    // goto code_r0x105023ec;
                    // goto code_r0x1050244d;
                }
                piVar57 = (piVar58 + 1);
                uVar6 = _in(uVar45);
                *piVar58 = uVar6;
                if (!bVar60) {}
                // goto code_r0x1050244f;
                if (bVar16 != 0) {}
                // goto code_r0x10502450;
                // code_r0x105023de:
                out(*piVar49, uVar45);
                pbVar1 = (piVar57 + 0x61);
                bVar16 = *pbVar1;
                bVar23 = (uVar26 >> 8);
                *pbVar1 = *pbVar1 + bVar23;
                puVar4 = (pbVar28 + piVar49 + 0x78);
                *puVar4 = *puVar4 + CARRY1(bVar16, bVar23) * ((uVar48 & 3) - (*puVar4 & 3));
                (ppiVar35 + -2) = 0x7269;
                pbVar1 = (pbVar28 + piVar57 + 0x76);
                bVar16 = (uVar43 >> 8);
                bVar60 = CARRY1(*pbVar1, bVar16);
                *pbVar1 = *pbVar1 + bVar16;
                puVar52 = (ppiVar35 + -2);
                uVar22 = (ppiVar35 + 8);
                ppiVar39 = (ppiVar35 + 0xe);
                // code_r0x105023ec:
                uVar6 = _in(uVar22);
                *puVar52 = uVar6;
                piVar57 = *ppiVar39;
                uVar48 = ppiVar39[2];
                pbVar28 = ppiVar39[4];
                uVar45 = ppiVar39[5];
                uVar26 = ppiVar39[6];
                uVar43 = ppiVar39[7];
                ppiVar35 = ppiVar39 + 8;
                piVar49 = (ppiVar39[1] + 1);
                out(*ppiVar39[1], uVar45);
                puVar4 = (pbVar28 + piVar49);
                iVar21 = (uVar48 & 3) - (*puVar4 & 3);
                bVar61 = 0 < iVar21;
                *puVar4 = *puVar4 + bVar60 * iVar21;
                if (bVar60) {}
                // goto code_r0x10502465;
                // code_r0x105023f4:
                lVar12 = *piVar57 * 0x6c70;
                ppiVar40 = lVar12;
                piVar5 = piVar49;
                piVar49 = (piVar49 + 1);
                out(*piVar5, uVar45);
                if (!bVar61) {
                    if (ppiVar40 == lVar12) {}
                    // goto code_r0x10502401;
                    bVar15 = 9 < (uVar43 & 0xf) | bVar15;
                    uVar43 = CONCAT11((uVar43 >> 8) + bVar15, uVar43 + bVar15 * '\x06') & 0xff0f;
                    // goto code_r0x1050246e;
                }
                // code_r0x10502477:
                piVar5 = piVar49;
                piVar49 = piVar49 + 1;
                out(*piVar5, uVar45);
                if (!bVar61) {}
                // goto code_r0x105024f2;
                (ppiVar40 + -2) = 0x706c;
                (ppiVar40 + -4) = 0x706c;
                pbVar1 = pbVar28 + piVar49;
                bVar16 = uVar43;
                *pbVar1 = *pbVar1 + bVar16;
                uVar26 = uVar26 & 0xff | ((uVar26 >> 8) ^ *0x6d62) << 8;
                bVar23 = (uVar45 >> 8) ^ *pbVar28 ^ *0x2e38;
                uVar45 = uVar45 & 0xff | bVar23 << 8;
                pcVar2 = (uVar48 + piVar49);
                *pcVar2 = *pcVar2 + bVar23;
                bVar15 = 9 < (bVar16 & 0xf) | bVar15;
                uVar43 = CONCAT11((uVar43 >> 8) + bVar15, bVar16 + bVar15 * '\x06') & 0xff0f;
                // code_r0x10502495:
                pcVar2 = (uVar48 + piVar49);
                *pcVar2 = *pcVar2 + (uVar45 >> 8);
                pbVar1 = 0x6d62;
                *pbVar1 = *pbVar1 ^ (uVar26 >> 8);
                pbVar1 = pbVar28 + piVar49;
                *pbVar1 = *pbVar1 + uVar43;
                // code_r0x105024a5:
                pcVar2 = (uVar48 + piVar49);
                *pcVar2 = *pcVar2 + (uVar45 >> 8);
                bVar15 = 9 < (uVar43 & 0xf) | bVar15;
                uVar43 = CONCAT11((uVar43 >> 8) + bVar15, uVar43 + bVar15 * '\x06') & 0xff0f;
                // code_r0x105024a8:
                uVar43 = uVar43 ^ 0x2e;
            }
            pcVar2 = (uVar48 + piVar49);
            cVar25 = (uVar45 >> 8);
            *pcVar2 = *pcVar2 + cVar25;
            pcVar2 = (uVar48 + piVar49);
            *pcVar2 = *pcVar2 + cVar25;
            // code_r0x105024b7:
            uVar43 = uVar43 ^ 0x2e;
            // code_r0x105024ba:
            pbVar1 = (piVar49 + uVar48 + 0x5024);
            bVar60 = CARRY1(*pbVar1, uVar43);
            *pbVar1 = *pbVar1 + uVar43;
        } else {
            piVar5 = piVar49;
            piVar49 = (puVar52 + 3);
            out(*piVar5, uVar45);
            if (!bVar61) {
                pbVar1 = (pbVar28 + piVar49 + 0x6c);
                bVar60 = CARRY1(*pbVar1, bVar24);
                *pbVar1 = *pbVar1 + bVar24;
                bVar61 = *pbVar1 == 0;
                piVar5 = piVar49;
                piVar49 = (puVar52 + 4);
                out(*piVar5, uVar45);
                ppiVar36 = (ppcVar32 + 0x10);
                if (bVar61) {
                    // code_r0x10502335:
                    ppiVar35 = ppiVar36;
                    pbVar1 = (piVar49 + 0x65);
                    bVar16 = (uVar26 >> 8);
                    bVar60 = CARRY1(*pbVar1, bVar16);
                    *pbVar1 = *pbVar1 + bVar16;
                    ppiVar34 = ppiVar35;
                    if (bVar60 || *pbVar1 == 0) {}
                    // goto code_r0x1050239f;
                    // goto code_r0x1050233a;
                }
                if (bVar60) {}
                // goto code_r0x10502336;
                piVar58 = ppcVar32[0x10];
                piVar49 = ppcVar32[0x11];
                uVar48 = ppcVar32[0x12];
                pbVar28 = ppcVar32[0x14];
                uVar45 = ppcVar32[0x15];
                uVar26 = ppcVar32[0x16];
                uVar43 = ppcVar32[0x17];
                ppcVar33 = ppcVar32 + 0x18;
                ppiVar35 = (ppcVar32 + 0x18);
                pcVar2 = (pbVar28 + piVar58 + 0x6e);
                cVar25 = (uVar26 >> 8);
                *pcVar2 = *pcVar2 + cVar25;
                cVar20 = (uVar45 >> 8);
                if (*pcVar2 != '\0') {
                    puVar4 = (piVar49 + uVar48 + 0x2e);
                    *puVar4 = *puVar4 ^ (ppcVar32 + 0x18);
                    pcVar2 = (pbVar28 + piVar58 + 0x6e);
                    *pcVar2 = *pcVar2 + cVar25;
                    ppiVar34 = (ppcVar32 + 0x18);
                    if (*pcVar2 == '\0') {}
                    // goto code_r0x10502317;
                    puVar4 = (piVar58 + uVar48 + 0x2e);
                    *puVar4 = *puVar4 ^ (ppcVar32 + 0x18);
                    pcVar2 = (pbVar28 + piVar58 + 0x6e);
                    *pcVar2 = *pcVar2 + cVar25;
                    if (*pcVar2 == '\0') {
                        out(*piVar49, uVar45);
                        ppiVar34 = (ppcVar32 + 0x18);
                        // goto code_r0x10502317;
                    }
                    puVar4 = (piVar49 + 0x17);
                    *puVar4 = *puVar4 ^ (ppcVar32 + 0x18);
                    pcVar2 = (pbVar28 + piVar58 + 0x6e);
                    *pcVar2 = *pcVar2 + cVar25;
                    if (*pcVar2 != '\0') {
                        piVar57 = piVar58 + 1;
                        iVar21 = _in(uVar45);
                        *piVar58 = iVar21;
                        puVar4 = (pbVar28 + piVar57);
                        *puVar4 = *puVar4 ^ piVar49;
                        pcVar2 = (uVar48 + 0x72);
                        cVar25 = *pcVar2;
                        *pcVar2 = *pcVar2 + cVar20;
                        ppcVar33 = ppcVar32 + 0x17;
                        ppiVar35 = (ppcVar32 + 0x17);
                        ppcVar32[0x17] = 0x6c65;
                        if (!SCARRY1(cVar25, cVar20)) {}
                        // goto code_r0x1050230c;
                        // goto code_r0x1050233a;
                    }
                    // goto code_r0x10502360;
                }
                // code_r0x1050230c:
                pcVar2 = (uVar48 + piVar49);
                *pcVar2 = *pcVar2 + cVar20;
                ppiVar34 = ppcVar33;
                // code_r0x10502317:
                pcVar2 = 0x3438;
                *pcVar2 = *pcVar2 + cVar20;
                piVar57 = *ppiVar34;
                piVar49 = ppiVar34[1];
                uVar48 = ppiVar34[2];
                pbVar28 = ppiVar34[4];
                uVar45 = ppiVar34[5];
                uVar26 = ppiVar34[6];
                uVar43 = ppiVar34[7];
                ppiVar36 = ppiVar34 + 8;
                ppiVar41 = ppiVar34 + 8;
                ppiVar40 = ppiVar34 + 8;
                pcVar2 = (uVar48 + 0x6f);
                *pcVar2 = *pcVar2 + (uVar26 >> 8);
                if (*pcVar2 != '\0') {
                    pbVar1 = (piVar57 + uVar48 + 0x75);
                    bVar16 = (uVar45 >> 8);
                    bVar60 = CARRY1(*pbVar1, bVar16);
                    *pbVar1 = *pbVar1 + bVar16;
                    ppiVar36 = ppiVar34 + 8;
                    // goto code_r0x1050232c;
                }
                piVar5 = piVar57 + 0x3c;
                cVar25 = *piVar5;
                cVar20 = (uVar43 >> 8);
                *piVar5 = *piVar5 + cVar20;
                if (!SCARRY1(cVar25, cVar20)) {}
                // goto code_r0x10502392;
                if (-1 < *piVar5) {
                    uVar6 = _in(uVar45);
                    *piVar57 = uVar6;
                    // goto code_r0x10502401;
                }
                pcVar2 = (uVar48 + 0x72);
                *pcVar2 = *pcVar2 + cVar20;
                bVar15 = 9 < (uVar43 & 0xf) | bVar15;
                uVar43 = CONCAT11(cVar20 + bVar15, uVar43 + bVar15 * '\x06') & 0xff0f;
                // code_r0x10502474:
                piVar57 = *ppiVar41;
                ppiVar40 = ppiVar41 + 1;
                uVar43 = uVar43 ^ (pbVar28 + piVar49);
                bVar61 = uVar43 == 0;
                // goto code_r0x10502477;
            }
            piVar5 = piVar57;
            piVar57 = piVar57 + 1;
            iVar21 = _in(uVar45);
            *piVar5 = iVar21;
            if (!bVar60) {}
            // goto code_r0x105023a1;
            // code_r0x1050232c:
            if (!bVar60) {
                piVar57 = *ppiVar36;
                piVar49 = ppiVar36[1];
                uVar48 = ppiVar36[2];
                pbVar28 = ppiVar36[4];
                uVar45 = ppiVar36[5];
                uVar26 = ppiVar36[6];
                uVar43 = ppiVar36[7];
                puVar4 = (piVar57 + 0x17);
                *puVar4 = *puVar4 + bVar60 * (((ppiVar36 + 8) & 3) - (*puVar4 & 3));
                ppiVar36 = ppiVar36 + 8;
                // goto code_r0x10502335;
            }
            pcVar2 = (piVar57 + 0x61);
            *pcVar2 = *pcVar2 + (uVar43 >> 8);
            // code_r0x10502392:
            piVar58 = piVar49 + 1;
            out(*piVar49, uVar45);
            pbVar1 = (piVar57 + 0x61);
            bVar16 = (uVar43 >> 8);
            bVar60 = CARRY1(*pbVar1, bVar16);
            *pbVar1 = *pbVar1 + bVar16;
            bVar16 = *pbVar1;
            if (!bVar60) {
                (ppiVar36 + -2) = 0x6171;
                lVar12 = (pbVar28 + piVar58) * 0x73;
                uVar43 = lVar12;
                bVar60 = uVar43 != lVar12;
                piVar49 = piVar49 + 2;
                out(*piVar58, uVar45);
                ppiVar38 = (ppiVar36 + -2);
                // code_r0x105023a1:
                uVar6 = _in(uVar45);
                *piVar57 = uVar6;
                if (!bVar60) {
                    lVar12 = (uVar48 + 100) * 0x7300;
                    uVar48 = lVar12;
                    bVar60 = uVar48 != lVar12;
                    // goto code_r0x105023a9;
                }
                piVar57 = *ppiVar38;
                ppiVar34 = ppiVar38 + 1;
                // goto code_r0x10502418;
            }
            // code_r0x1050240e:
            piVar49 = piVar58;
            ppiVar34 = ppiVar36;
            if (bVar16 != 0) {}
            // goto code_r0x10502410;
            // code_r0x10502443:
            if (bVar60) {
                uVar45 = uVar45 & 0xff | ((uVar45 >> 8) ^ *0x2e34) << 8;
                // goto code_r0x105024ba;
            }
            // code_r0x10502445:
            piVar5 = piVar49;
            piVar49 = piVar49 + 1;
            out(*piVar5, uVar45);
            if (bVar60) {
                // code_r0x1050244d:
                piVar57 = *ppiVar36;
                piVar49 = ppiVar36[1];
                uVar48 = ppiVar36[2];
                pbVar28 = ppiVar36[4];
                uVar45 = ppiVar36[5];
                uVar26 = ppiVar36[6];
                uVar43 = ppiVar36[7];
                ppiVar35 = ppiVar36 + 8;
                // code_r0x1050244f:
                piVar5 = piVar57;
                piVar57 = piVar57 + 1;
                iVar21 = _in(uVar45);
                *piVar5 = iVar21;
                // code_r0x10502450:
                pbVar1 = (piVar49 + 0x75);
                bVar16 = *pbVar1;
                bVar23 = (uVar45 >> 8);
                *pbVar1 = *pbVar1 + bVar23;
                if (!CARRY1(bVar16, bVar23)) {
                    iVar21 = (uVar48 + 0x65);
                    uVar48 = iVar21 * 0x6d00;
                    uVar43 = uVar43 ^ 0x5f39;
                    puVar4 = (piVar49 + iVar21 * 0x3680);
                    *puVar4 = *puVar4 ^ piVar49;
                    pbVar1 = pbVar28 + 0x6f;
                    bVar16 = (uVar43 >> 8);
                    bVar60 = CARRY1(*pbVar1, bVar16);
                    *pbVar1 = *pbVar1 + bVar16;
                    // code_r0x10502465:
                    if (!bVar60) {
                        ppiVar40 = (ppiVar35 + 2);
                        // goto code_r0x10502469;
                    }
                    pbVar1 = pbVar28 + piVar49;
                    *pbVar1 = *pbVar1 + uVar43;
                    pbVar1 = pbVar28 + piVar49;
                    *pbVar1 = *pbVar1 + uVar43;
                    uVar45 = uVar45 & 0xff | ((uVar45 >> 8) ^ *pbVar28) << 8;
                    // goto code_r0x105024a8;
                }
                // goto code_r0x105024b7;
            }
        }
        pbVar1 = (piVar57 + uVar48 + 0x5024);
        bVar23 = uVar43;
        bVar61 = CARRY1(*pbVar1, bVar23) || CARRY1(*pbVar1 + bVar23, bVar60);
        *pbVar1 = *pbVar1 + bVar23 + bVar60;
        pbVar1 = (piVar57 + uVar48 + 0x5024);
        bVar16 = *pbVar1 + uVar26;
        bVar60 = CARRY1(*pbVar1, uVar26) || CARRY1(bVar16, bVar61);
        *pbVar1 = bVar16 + bVar61;
        pbVar1 = (piVar57 + uVar48 + 0x5024);
        bVar16 = *pbVar1 + uVar45;
        bVar61 = CARRY1(*pbVar1, uVar45) || CARRY1(bVar16, bVar60);
        *pbVar1 = bVar16 + bVar60;
        pbVar1 = (piVar57 + uVar48 + 0x5024);
        bVar16 = *pbVar1 + pbVar28;
        bVar60 = CARRY1(*pbVar1, pbVar28) || CARRY1(bVar16, bVar61);
        *pbVar1 = bVar16 + bVar61;
        pbVar1 = (piVar57 + uVar48 + 0x5024);
        bVar16 = (uVar43 >> 8);
        bVar61 = CARRY1(*pbVar1, bVar16) || CARRY1(*pbVar1 + bVar16, bVar60);
        *pbVar1 = *pbVar1 + bVar16 + bVar60;
        piVar5 = piVar49 + 0x2812;
        bVar60 = CARRY1(*piVar5, bVar16) || CARRY1(*piVar5 + bVar16, bVar61);
        *piVar5 = *piVar5 + bVar16 + bVar61;
        piVar5 = piVar57 + 0x2812;
        bVar61 = CARRY1(*piVar5, bVar16) || CARRY1(*piVar5 + bVar16, bVar60);
        *piVar5 = *piVar5 + bVar16 + bVar60;
        pbVar1 = (uVar48 + 0x5024);
        bVar60 = CARRY1(*pbVar1, bVar16) || CARRY1(*pbVar1 + bVar16, bVar61);
        *pbVar1 = *pbVar1 + bVar16 + bVar61;
        pbVar1 = (uVar48 + 0x5024);
        bVar15 = (uVar26 >> 8);
        bVar16 = *pbVar1 + bVar15;
        bVar61 = CARRY1(*pbVar1, bVar15) || CARRY1(bVar16, bVar60);
        *pbVar1 = bVar16 + bVar60;
        pbVar1 = (uVar48 + 0x5024);
        bVar15 = *pbVar1;
        bVar24 = (uVar45 >> 8);
        bVar16 = *pbVar1;
        *pbVar1 = bVar16 + bVar24 + bVar61;
        pbVar1 = pbVar28 + piVar49;
        *pbVar1 = *pbVar1 + bVar23 + (CARRY1(bVar15, bVar24) || CARRY1(bVar16 + bVar24, bVar61));
        uVar45 = (bVar24 ^ pbVar28[piVar49]) << 8;
        uVar43 = (bVar23 ^ 0x2e);
        // code_r0x105024f2:
        pcVar2 = (uVar48 + piVar49);
        bVar16 = (uVar45 >> 8);
        *pcVar2 = *pcVar2 + bVar16;
        pbVar1 = pbVar28 + piVar49;
        *pbVar1 = *pbVar1 ^ (pbVar28 >> 8);
        pcVar2 = (uVar48 + piVar49);
        *pcVar2 = *pcVar2 + bVar16;
        pcVar2 = (uVar48 + piVar49);
        *pcVar2 = *pcVar2 + (bVar16 ^ pbVar28[piVar49]);
        puVar4 = (pbVar28 + piVar57);
        *puVar4 = *puVar4 ^ piVar49;
        pbVar1 = pbVar28 + piVar49;
        *pbVar1 = *pbVar1 + uVar43;
        pbVar1 = pbVar28 + piVar49;
        *pbVar1 = *pbVar1 + uVar43;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_1312() {
    let pcVar1: *mut char;
    let mut cVar2: u8;
    let mut extraout_AH: u8;
    let mut extraout_DL: u8;
    let mut extraout_DH: u8;
    let in_BX: *mut char;
    let puVar3: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut char;
    let unaff_DI: *mut char;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar3 = &stack0xfffe;
    cVar2 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar3 = puVar3 + -1;
        unsafe { *puVar3 = *unaff_BP };
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    _in(0xff);
    (**(in_BX + unaff_DI))();
    pcVar1 = unaff_DI;
    unsafe {
        *pcVar1 = *pcVar1 + extraout_AH;
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + extraout_DH;
        pcVar1 = in_BX;
        *pcVar1 = *pcVar1 + extraout_DH;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + extraout_DH;
        pcVar1 = in_BX;
        *pcVar1 = *pcVar1 + extraout_AH;
        pcVar1 = 0x100;
        *pcVar1 = *pcVar1 + extraout_DL;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_12ee() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_1234() {
    let pcVar1: *mut char;
    let ppcVar2: fn();
    let mut cVar3: u8;
    let mut in_CL: u8;
    let mut in_CH: u8;
    let mut extraout_DH: u8;
    let mut in_BX: i32;
    let puVar4: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut char;
    let unaff_DI: fn();
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar4 = &stack0xfffe;
    cVar3 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar4 = puVar4 + -1;
        unsafe { *puVar4 = *unaff_BP };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    cVar3 = (**unaff_DI)();
    pcVar1 = 0x4000;
    unsafe {
        *pcVar1 = *pcVar1 + in_CH;
        pcVar1 = (in_BX + unaff_DI);
        *pcVar1 = *pcVar1 + in_CL;
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + extraout_DH;
        ppcVar2 = unaff_DI;
        *ppcVar2 = *ppcVar2 + in_BX + 0x1;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + cVar3;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_1214() {
    let pcVar1: *mut char;
    let ppcVar2: fn();
    let mut cVar3: u8;
    let mut in_CL: u8;
    let mut in_CH: u8;
    let mut extraout_DH: u8;
    let mut in_BL: u8;
    let puVar4: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let unaff_DI: fn();
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar4 = &stack0xfffe;
    cVar3 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar4 = puVar4 + -1;
        unsafe { *puVar4 = *unaff_BP };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    cVar3 = (**unaff_DI)();
    pcVar1 = 0x4000;
    unsafe {
        *pcVar1 = *pcVar1 + in_CH;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + in_CL;
        ppcVar2 = unaff_DI;
        *ppcVar2 = *ppcVar2 + extraout_DH;
        ppcVar2 = unaff_DI;
        *ppcVar2 = *ppcVar2 + in_BL;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + cVar3;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_0e8e() {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let piVar3: *mut i32;
    let mut uVar4: i32;
    let mut cVar5: u8;
    let mut bVar6: u8;
    let mut in_AX: i32;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let mut in_CL: u8;
    let mut in_DL: u8;
    let in_BX: *mut char;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    unsafe {
        pbVar2 = (in_BX + unaff_SI);
        uVar7 = (in_AX & 0xff00 | (in_AX + *pbVar2)) + 0xc900 + CARRY1(in_AX, *pbVar2);
        piVar3 = (in_BX + unaff_DI);
        *piVar3 = *piVar3 + uVar7;
        pcVar1 = &stack0xfffe + unaff_SI;
        bVar6 = uVar7;
        *pcVar1 = *pcVar1 + bVar6;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + (uVar7 >> 8);
        piVar3 = (in_BX + unaff_DI);
        *piVar3 = *piVar3 + uVar7;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + bVar6;
        pcVar1 = in_BX;
        *pcVar1 = *pcVar1 + bVar6;
        pcVar1 = in_BX + unaff_DI + 0x18;
        *pcVar1 = *pcVar1 + in_DL + in_CL;
        pcVar1 = in_BX + unaff_SI;
        *pcVar1 = *pcVar1 + bVar6;
        pbVar2 = (in_BX + unaff_SI);
        uVar8 = uVar7 & 0xff00 | (bVar6 + *pbVar2);
        uVar7 = CARRY1(bVar6, *pbVar2);
        uVar4 = uVar8 + 0x5200;
        uVar9 = uVar4 + uVar7;
        pcVar1 = in_BX + unaff_DI;
        cVar5 = uVar9;
        *pcVar1 = (*pcVar1 - cVar5) - (0xadff < uVar8 || CARRY2(uVar4, uVar7));
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + cVar5;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + (uVar9 >> 8);
        pcVar1 = in_BX + unaff_DI + 0x18;
        *pcVar1 = *pcVar1 + cVar5;
        piVar3 = (in_BX + unaff_SI);
        *piVar3 = *piVar3 + uVar9;
        pcVar1 = in_BX + unaff_SI;
        *pcVar1 = *pcVar1 + cVar5;
        piVar3 = (in_BX + unaff_SI);
        *piVar3 = *piVar3 + uVar9;
        bVar6 = cVar5 + in_BX[unaff_SI];
        piVar3 = (in_BX + unaff_SI);
        *piVar3 = *piVar3 + (uVar9 & 0xff00 | bVar6);
        pcVar1 = in_BX + unaff_SI;
        *pcVar1 = *pcVar1 + bVar6;
        bVar6 = bVar6 + in_BX[unaff_SI] + in_BX[unaff_SI];
        pcVar1 = in_BX + unaff_SI;
        *pcVar1 = *pcVar1 + bVar6;
        piVar3 = (in_BX + unaff_SI);
        *piVar3 = *piVar3 + (uVar9 & 0xff00 | bVar6);
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_11f4() {
    let pcVar1: *mut char;
    let mut cVar2: u8;
    let mut in_CH: u8;
    let in_BX: *mut char;
    let puVar3: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut char;
    let unaff_DI: fn();
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar3 = &stack0xfffe;
    cVar2 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar3 = puVar3 + -1;
        unsafe { *puVar3 = *unaff_BP };
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    cVar2 = (**unaff_DI)();
    pcVar1 = in_BX;
    unsafe {
        *pcVar1 = *pcVar1 + in_CH;
        pcVar1 = in_BX + unaff_DI;
        *pcVar1 = *pcVar1 + cVar2;
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + cVar2;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + cVar2;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1050_0daa() {
    let pcVar1: *mut char;
    let piVar2: *mut i32;
    let pbVar3: *mut byte;
    let mut bVar4: u8;
    let mut bVar5: u8;
    let mut in_AX: i32;
    let mut uVar6: i32;
    let mut bVar11: u8;
    let mut cVar12: u8;
    let mut iVar8: i32;
    let mut uVar9: i32;
    let mut uVar10: i32;
    let mut in_CL: u8;
    let mut in_DL: u8;
    let mut in_DH: u8;
    let in_BX: *mut char;
    let pcVar13: *mut char;
    let mut bVar14: u8;
    let mut unaff_BP: u8;
    let unaff_SI: *mut char;
    let unaff_DI: *mut i32;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut in_AF: u8;
    let mut uVar7: i32;

    unsafe {
        bVar4 = in_AX - *unaff_DI;
        pcVar1 = in_BX + unaff_SI;
        *pcVar1 = *pcVar1 + bVar4;
        uVar6 = ((in_AX & 0xff00 | (bVar4 ^ in_BX[unaff_SI])) - *unaff_DI) + 0x3200;
        piVar2 = unaff_DI;
        cVar12 = (in_BX >> 8);
        *piVar2 = *piVar2 + cVar12;
        bVar4 = uVar6 - 5;
        pcVar1 = in_BX + unaff_SI;
        *pcVar1 = *pcVar1 + bVar4;
        uVar6 = (uVar6 & 0xff00 | (bVar4 ^ in_BX[unaff_SI])) - 5;
        pcVar1 = in_BX + unaff_DI;
        *pcVar1 = *pcVar1 + in_BX;
        pcVar1 = in_BX;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar13 = in_BX + -1;
        pcVar1 = (pcVar13 + unaff_SI + 0x2f00);
        bVar5 = uVar6;
        *pcVar1 = *pcVar1 + bVar5;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 - uVar6;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + 0x530;
        pbVar3 = (pcVar13 + unaff_SI);
        bVar4 = *pbVar3;
        *pbVar3 = *pbVar3 + bVar5;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = (*piVar2 - uVar6) - CARRY1(bVar4, bVar5);
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + '1';
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        bVar5 = bVar5 ^ *unaff_DI;
        uVar6 = uVar6 & 0xff00;
        uVar7 = uVar6 | bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + uVar7;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + in_CL;
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + pcVar13;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + uVar7;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = 0x1d00;
        *pcVar1 = *pcVar1 + in_DL;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + uVar7;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_DI;
        cVar12 = (uVar6 >> 8);
        *pcVar1 = *pcVar1 + cVar12;
        pcVar1 = 0x1;
        *pcVar1 = *pcVar1 + pcVar13;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + in_DH;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + cVar12;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + uVar7;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        piVar2 = unaff_DI;
        bVar14 = (pcVar13 >> 8);
        *piVar2 = *piVar2 + bVar14;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + cVar12;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + uVar7;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + in_CL;
        bVar5 = bVar5 & pcVar13[unaff_DI];
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        uVar6 = (uVar6 | bVar5) + (pcVar13 + unaff_SI);
        pcVar1 = &stack0xfffe + unaff_DI;
        bVar11 = (uVar6 >> 8);
        *pcVar1 = *pcVar1 + bVar11;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + uVar6;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + uVar6;
        pcVar1 = unaff_SI;
        *pcVar1 = *pcVar1 + bVar11;
        pcVar1 = &stack0xfffe + unaff_DI;
        bVar5 = (uVar6 & 0x101);
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13 + unaff_DI;
        *pcVar1 = *pcVar1 + in_DH;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + (uVar6 & 0x101);
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + bVar5;
        bVar4 = 9 < bVar5 | in_AF;
        bVar5 = bVar5 + bVar4 * '\x06';
        bVar5 = bVar5
            + (0x90 < (bVar5 & 0xf0) | CARRY1(unaff_BP, bVar14) | bVar4 * (0xf9 < bVar5)) * '`';
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + (uVar6 & 0x100 | bVar5);
        piVar2 = unaff_DI;
        *piVar2 = *piVar2 + bVar5;
        pcVar1 = pcVar13;
        *pcVar1 = *pcVar1 + bVar5;
        cVar12 = (bVar11 & 1) + bVar5;
        iVar8 = CONCAT11(cVar12, bVar5);
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + iVar8;
        piVar2 = unaff_DI;
        *piVar2 = *piVar2 + bVar5;
        piVar2 = unaff_DI;
        *piVar2 = *piVar2 + in_DL;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + iVar8;
        piVar2 = unaff_DI;
        *piVar2 = *piVar2 + bVar5;
        pcVar1 = &stack0xfffe + unaff_DI;
        *pcVar1 = *pcVar1 + cVar12;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + iVar8;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13;
        *pcVar1 = *pcVar1 + bVar5;
        bVar5 = bVar5 + in_CL;
        iVar8 = CONCAT11(cVar12, bVar5);
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + iVar8;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        piVar2 = unaff_DI;
        *piVar2 = *piVar2 + in_DL;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + iVar8;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + cVar12;
        piVar2 = (pcVar13 + unaff_DI);
        *piVar2 = *piVar2 + iVar8;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = pcVar13;
        *pcVar1 = *pcVar1 + bVar5;
        pcVar1 = (pcVar13 + unaff_DI + 0x18);
        *pcVar1 = *pcVar1 + in_DL + in_CL * 0x2;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar5;
        pbVar3 = (pcVar13 + unaff_SI);
        uVar9 = CONCAT11(cVar12, bVar5 + *pbVar3);
        uVar6 = CARRY1(bVar5, *pbVar3);
        uVar7 = uVar9 + 0x5200;
        uVar10 = uVar7 + uVar6;
        pcVar1 = pcVar13 + unaff_DI;
        cVar12 = uVar10;
        *pcVar1 = (*pcVar1 - cVar12) - (0xadff < uVar9 || CARRY2(uVar7, uVar6));
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + cVar12;
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + (uVar10 >> 8);
        pcVar1 = (pcVar13 + unaff_DI + 0x18);
        *pcVar1 = *pcVar1 + cVar12;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + uVar10;
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + cVar12;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + uVar10;
        bVar4 = cVar12 + pcVar13[unaff_SI];
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + (uVar10 & 0xff00 | bVar4);
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar4;
        bVar4 = bVar4 + pcVar13[unaff_SI] + pcVar13[unaff_SI];
        pcVar1 = pcVar13 + unaff_SI;
        *pcVar1 = *pcVar1 + bVar4;
        piVar2 = (pcVar13 + unaff_SI);
        *piVar2 = *piVar2 + (uVar10 & 0xff00 | bVar4);
        // WARNING: Bad instruction - Truncating control flow here
    }
    halt_baddata();
}

pub fn bad_fn_1040_d920(param_1: u32) {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let puVar3: *mut u16;
    let uVar4: u8;
    let mut bVar5: u8;
    let mut cVar6: u8;
    let mut in_AX: i32;
    let mut in_CX: u16;
    let mut bVar7: u8;
    let mut in_DX: i32;
    let mut bVar8: u8;
    let mut in_BX: i32;
    let mut bVar9: u8;
    let puVar10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let in_stack_0000bf2a: *mut u8;
    let in_stack_0000bf2c: *mut u16;
    let mut in_stack_0000bf32: i32;
    let mut in_stack_0000bf34: u16;
    let mut in_stack_0000bf38: u8;
    let local_4e: u8;
    let local_2b: u8;

    puVar10 = &stack0xfffe;
    cVar6 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar10 = puVar10 + -1;
        unsafe { *puVar10 = *unaff_BP };
        cVar6 = cVar6 + -1;
        '\0' < cVar6
    } {}
    bVar9 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar9;
    bVar8 = ((in_DX + 1) >> 8);
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(bVar8 + bVar9, in_CF);
    bVar7 = (in_DX + 1);
    pbVar2 = unaff_SI + in_BX;
    unsafe {
        bVar8 = *pbVar2;
        bVar5 = *pbVar2 - bVar7;
        bVar12 = *pbVar2 < bVar7 || bVar5 < bVar11;
        *pbVar2 = bVar5 - bVar11;
        if ((*pbVar2 != 0)
            && (SBORROW1(bVar8, bVar7) != (SBORROW1(bVar5, bVar11) == (*pbVar2 < '\0'))))
        {
            pbVar2 = unaff_SI;
            bVar8 = *pbVar2;
            bVar5 = *pbVar2;
            *pbVar2 = bVar5 + bVar9 + bVar12;
            bVar11 = CARRY1(local_4e, in_BX)
                || CARRY1(
                    local_4e + in_BX,
                    CARRY1(bVar8, bVar9) || CARRY1(bVar5 + bVar9, bVar12),
                );
            pbVar2 = unaff_SI + -0x4f;
            bVar8 = *pbVar2;
            bVar5 = *pbVar2;
            *pbVar2 = bVar5 + bVar9 + bVar11;
            bVar7 = (in_CX >> 8);
            pcVar1 = &stack0xfffe + unaff_SI;
            *pcVar1 = *pcVar1
                + in_CX
                + (CARRY1(local_2b, bVar7)
                    || CARRY1(
                        local_2b + bVar7,
                        CARRY1(bVar8, bVar9) || CARRY1(bVar5 + bVar9, bVar11),
                    ));
            if (!SBORROW2(in_AX, 1)) {
                out(*in_stack_0000bf2c, in_stack_0000bf34);
                uVar4 = _in(in_stack_0000bf34);
                *in_stack_0000bf2a = uVar4;
                puVar3 = (in_stack_0000bf2a + 0x73);
                *puVar3 = *puVar3;
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            pcVar1 = (in_stack_0000bf32 + in_stack_0000bf2c);
            *pcVar1 = *pcVar1 + in_stack_0000bf38;
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }

        if (*pbVar2 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub fn bad_fn_1040_bf16() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn bad_fn_1018_d46e() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn pass1_1038_ca75(param_1: *mut astruct_44) -> *mut astruct_44 {
    let puVar1: *mut u8;
    let pcVar2: *mut char;
    let pbVar3: *mut byte;
    let puVar4: *mut u32;
    let mut bVar5: u8;
    let mut uVar6: i32;
    let mut in_AL: u8;
    let mut in_CX: u16;
    let mut bVar7: u8;
    let mut cVar8: u8;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut in_DX: i32;
    let mut bVar12: u8;
    let mut iVar11: i32;
    let mut in_BX: i32;
    let mut uVar13: i32;
    let mut bVar14: u8;
    let mut unaff_BP: u16;
    let unaff_SI: *mut byte;
    let unaff_DI: *mut byte;
    let mut unaff_ES: i32;
    let mut uVar15: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar16: bool;
    let mut bVar17: bool;
    let mut in_SF: bool;
    let ppVar18: *mut pass1_struct_1;
    let in_stack_00000068: *mut u8;
    let mut in_stack_0000407f: u8;
    let mut in_stack_0000efc4: u32;
    let in_struct_1: *mut astruct_68;
    let in_stack_0000efc8: *mut u8;
    let mut in_stack_0000efca: u32;
    let mut in_stack_0000efce: u32;
    let in_stack_0000efd2: *mut char;
    let local_39: u8;
    let local_38: u8;
    let uStack3: u8;

    _uStack3 = CONCAT21(unaff_BP, uStack3);
    bVar9 = in_DX;
    bVar7 = (in_DX >> 8);
    bVar12 = in_BX;
    uVar13 = param_1;
    uVar15 = (param_1 >> 0x10);
    if (in_SF) {
        bVar16 = CARRY1(s__s___lu_1050_38c5[3], bVar7);
        bVar14 = s__s___lu_1050_38c5[3] + bVar7;
        s__s___lu_1050_38c5[3] = bVar14 + in_CF;
        local_39 = local_39 + bVar12 + (bVar16 || CARRY1(bVar14, in_CF));
        pbVar3 = unaff_SI + in_BX;
        unsafe {
            bVar16 = *pbVar3 < bVar9;
            in_struct_1 = (in_stack_0000efc4 & 0xffff0000 | ZEXT24(unaff_SI));
            bVar14 = (in_BX >> 8);
            if (*pbVar3 != bVar9 && bVar9 <= *pbVar3) {
                pbVar3 = unaff_SI + in_BX;
                *pbVar3 = (*pbVar3 - (bVar9 + bVar14 + bVar16))
                    - (CARRY1(bVar9, bVar14) || CARRY1(bVar9 + bVar14, bVar16));
                *unaff_DI = *unaff_SI;
                PTR_LOOP_1050_1038._0_1_ = in_AL;
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            pbVar3 = &stack0x4079 + unaff_SI;
            bVar17 = CARRY1(*pbVar3, bVar7) || CARRY1(*pbVar3 + bVar7, bVar16);
            *pbVar3 = *pbVar3 + bVar7 + bVar16;
            pbVar3 = unaff_SI;
            bVar5 = *pbVar3;
            bVar10 = *pbVar3 + bVar12;
            bVar16 = CARRY1(*pbVar3, bVar12) || CARRY1(bVar10, bVar17);
            *pbVar3 = bVar10 + bVar17;
            if ((*pbVar3 == 0)
                || (SCARRY1(bVar5, bVar12) != (SCARRY1(bVar10, bVar17) != (*pbVar3 < '\0'))))
            {
                puVar1 = &local_38 + unaff_SI;
                *puVar1 = *puVar1 + in_AL + bVar16;
                bVar7 = unaff_SI[in_BX];
                pbVar3 = (in_BX + 0x40);
                bVar12 = *pbVar3;
                *pbVar3 = bVar14;
                in_BX = in_BX & 0xff | bVar12 << 8;
                pbVar3 = unaff_SI;
                *pbVar3 = *pbVar3 + in_CX + (bVar7 < bVar9);
                pbVar3 = unaff_SI + in_BX + 0x10;
                *pbVar3 = *pbVar3 + 0x54;
                pbVar3 = unaff_SI + in_BX + 0x10;
                bVar16 = 0xa1 < *pbVar3;
                bVar9 = *pbVar3;
                *pbVar3 = *pbVar3 + 0x5e;
                if ((*pbVar3 != 0) && (SCARRY1(bVar9, '^') == (*pbVar3 < '\0'))) {}
                // goto code_r0x1038caa3;
                in_stack_00000068 = &stack0xefc4 + in_stack_00000068;
                _uStack3 = CONCAT12((unaff_BP >> 8), &stack0xefc4 + _uStack3);
                if (CARRY2(_uStack3, &stack0xefc4) || &stack0xefc4 + _uStack3 == 0x0) {
                    in_struct_1 = (in_stack_0000efc4 & 0xffff0000 | unaff_ES);
                }
            } else {
                bVar17 = CARRY1(bVar7, bVar14) || CARRY1(bVar7 + bVar14, bVar16);
                pbVar3 = unaff_SI + in_BX;
                bVar9 = *pbVar3;
                bVar10 = (in_DX & 0xff);
                bVar12 = *pbVar3;
                bVar5 = *pbVar3 - bVar10;
                *pbVar3 = bVar5 - bVar17;
                if ((*pbVar3 == 0)
                    || (SBORROW1(bVar12, bVar10) != (SBORROW1(bVar5, bVar17) != (*pbVar3 < '\0'))))
                {
                    if (*pbVar3 != 0) {
                        error_check_1000_17ce(param_1);
                    }
                    return param_1;
                }
                pbVar3 = unaff_DI + -0x75;
                *pbVar3 = *pbVar3 + bVar10 + (bVar9 < bVar10 || bVar5 < bVar17);
                _in(in_DX & 0xff | (bVar7 + bVar14 + bVar16) << 8);
            }
            process_struct_1040_7728(
                in_struct_1,
                in_stack_0000efc8,
                in_stack_0000efca,
                in_stack_0000efce,
                (in_stack_0000efce >> 0x10),
            );
            (uVar13 + 0x8e) = 0;
            param_1.ptr_a_lo = 0xcc9a;
            (uVar13 + 2) = &PTR_LOOP_1050_1038;
            in_BX = uVar13;
        }
    } else {
        bVar16 = CARRY1(bVar7, bVar12) || CARRY1(bVar7 + bVar12, in_CF);
        in_DX = in_DX & 0xff | (bVar7 + bVar12 + in_CF) << 8;
        // code_r0x1038caa3:
        pbVar3 = unaff_SI + (in_BX - 0x7f);
        unsafe {
            bVar9 = *pbVar3;
            bVar7 = *pbVar3 + in_BX;
            *pbVar3 = bVar7 + bVar16;
            bVar12 = (in_DX >> 8);
            puVar4 = (unaff_SI + in_BX + 0x10);
            uVar6 = *puVar4;
            *puVar4 = *puVar4 + 0x60ea;
            pbVar3 = unaff_SI + in_BX;
            *pbVar3 = (*pbVar3 - (in_DX & 0xff)) - (0x9f15 < uVar6);
            iVar11 = (in_DX & 0xff
                | (bVar12
                    + (in_BX >> 8)
                    + (CARRY1(in_stack_0000407f, bVar12)
                        || CARRY1(
                            in_stack_0000407f + bVar12,
                            CARRY1(bVar9, in_BX) || CARRY1(bVar7, bVar16),
                        )))
                    << 8)
                - 1;
            pbVar3 = unaff_SI + in_BX + 0x10;
            *pbVar3 = *pbVar3 + 0x66;
            pbVar3 = unaff_SI + in_BX + 0x10;
            bVar9 = *pbVar3;
            *pbVar3 = *pbVar3 - 0x22;
            if (-1 < *pbVar3) {
                bVar7 = (in_CX >> 8);
                bVar12 = (iVar11 >> 8);
                pbVar3 = unaff_SI + in_BX;
                *pbVar3 = (*pbVar3 - iVar11)
                    - (CARRY1(bVar12, bVar7) || CARRY1(bVar12 + bVar7, 0x21 < bVar9));
                loop {
                    // WARNING: Do nothing block with infinite loop
                }
            }
        }
    }
    cVar8 = (in_CX >> 8);
    func_0x47c726cc();
    pcVar2 = &stack0x002a + unaff_SI;
    unsafe { *pcVar2 = *pcVar2 + cVar8 + CARRY1((in_BX >> 8), unaff_SI[in_BX]) };
    ppVar18 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, in_stack_0000efd2);
    (uVar13 + 0x8e) = ppVar18;
    (uVar13 + 0x90) = (ppVar18 >> 0x10);
    (uVar13 + 0x74) = 0;
    return param_1;
}
