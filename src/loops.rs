pub fn loop_1010_11c6(param_1: &mut Struct365) {
    let pi32_1: &mut  i32;
    let ppc_var2: &mut  fn();
    let mut u_var3: u32;
    let mut u_var4: u32;
    let ctx.ax_reg: &mut  Struct366;
    let mut i_var5: i32;
    let local_AX_179: &mut  Struct367;
    let paVar6: &mut  Struct367;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let in_dx: &mut  Struct199;
    let mut u_var9: u16;
    let struct_a: &mut  Struct199;
    let struct_a_00: &mut  Struct199;
    let pa_var10: &mut  Struct199;
    let pa_var11: &mut  Struct199;
    let paVar12: &mut  Struct199;
    
    let mut u_var13: u16;
    let ctx.dx_reg: &mut  Struct199;
    
    
    let pp_var14: &mut  String;
    let mut pustruct_a: &mut Struct365;
    let mut iVar15: i32;
    let mut iVar16: i32;
    let mut uVar17: u16;
    let mut uVar18: u16;
    let pu16_19: &mut  u16;
    let paVar20: &mut  Struct367;
    let mut u_var21: u16;
    let mut local_36: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let local_28: &mut  Struct368;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (u16_1050_0ecc == 0xffff) {
        return;
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | ctx.ax_reg) == 0) {
        i_var5 = 0;
        u_var9 = 0;
    } else {
        pu16_19 = pass1_1010_37d4(CONCAT22(in_dx, ctx.ax_reg));
      // u_var9 = (pu16_19  >> 0x10);
        i_var5 = pu16_19;
    }
    _local_a = &WORD_1050_0ece;
    local_e = 0;
    while (true) {
      // uVar17 = (param_1  >> 0x10);
        pustruct_a = param_1;
        pu16_19 = &pustruct_a.field_0x68;
        let val = unsafe { *pu16_19 };
        if (val == local_e || val < local_e) {
            break;
        }
        u_var4 = pustruct_a.field_0x64;
        u_var3 = (u_var4 + local_e * 4);
        pp_var14 = (u_var3 + u16_1050_0ecc * 8);
        _local_32 = (u_var3 & 0xffff0000 | ZEXT24(pp_var14));
        iVar16 = unsafe { process_string_1000_475e(_local_a, *pp_var14) };
        if (iVar16 != 0) {
            _local_a = *_local_32;
            local_e = local_e & 0xffff | (local_e + 1) << 0x10;
        }
        local_e = local_e & 0xffff0000 | (local_e + 1);
    }
    (i_var5 + 0x10) = local_e;
    process_struct_1010_38f8(CONCAT22(u_var9, i_var5), local_e);
    local_AX_179 = 0x0;
    struct_a_00 = struct_a;
    process_struct_1000_179c(0x400, struct_a);
    pa_var10 = struct_a_00;
    paVar6 = local_AX_179;
    process_struct_1000_179c(0x400, struct_a_00);
    _local_26 = CONCAT22(pa_var10, paVar6);
    local_1c = 0;
    pass1_1000_4906(CONCAT22(struct_a_00, local_AX_179), 0, 0x400);
    pass1_1000_4906(CONCAT22(pa_var10, paVar6), 0, 0x400);
    local_2a = 0;
    local_e = 0;
    loop {
        pi32_1 = (i_var5 + 0x10);
        let val = unsafe { *pi32_1 };
        if (val == local_e || val < local_e) {
            return;
        }
        u_var4 = pustruct_a.field_0x64;
      // uVar18 = (u_var4  >> 0x10);
        iVar15 = u_var4;
        iVar16 = (iVar15 + local_1c * 4);
        paVar12 = (iVar15 + local_1c * 4 + 2);
        iVar15 = iVar16 + (u16_1050_0ecc * 6 + 0xeba) * 8;
        local_16 = CONCAT22(paVar12, iVar15);
        u_var7 = iVar16 + (u16_1050_0ecc * 6 + 0xebc) * 8;
        pa_var11 = paVar12;
        process_struct_1000_179c(0x1a, paVar12);
        if ((pa_var11 | u_var7) == 0) {
            u_var4 = (i_var5 + 8);
            (u_var4 + local_e * 4) = 0;
        } else {
            pu16_19 = pass1_1010_37d4(CONCAT22(pa_var11, u_var7));
            u_var4 = (i_var5 + 8);
          // uVar18 = (u_var4  >> 0x10);
            iVar16 = u_var4;
            (iVar16 + local_e * 4) = pu16_19;
            (iVar16 + local_e * 4 + 2) = (pu16_19 >> 0x10);
        }
        local_2a = local_2a + 1;
        u_var4 = (i_var5 + 8);
      // uVar18 = (u_var4  >> 0x10);
        iVar16 = u_var4;
        u_var4 = (iVar16 + local_e * 4);
        ppc_var2 = ((iVar16 + local_e * 4) + 0x1c);
        ppc_var2(0x1000, u_var4, (u_var4 >> 0x10), local_2a, iVar15, paVar12);
        // local_16
        loop {
            pu16_19 = &pustruct_a.field_0x68;
            let val = unsafe { *pu16_19 };
            if (val == local_1c || val < local_1c) {
                break;
            }
            iVar15 = local_1c * 4;
            u_var4 = pustruct_a.field_0x64;
            u_var4 = (u_var4 + iVar15);
            iVar16 =
                process_string_1000_475e(*local_16, *(u_var4 + (u16_1050_0ecc * 6 + 0xeba) * 8));
            if (iVar16 != 0) {
                break;
            }
            u_var4 = pustruct_a.field_0x64;
          // uVar18 = (u_var4  >> 0x10);
            iVar16 = u_var4;
            paVar12 = (iVar16 + iVar15 + 2);
            u_var7 = (iVar16 + iVar15) + (u16_1050_0ecc * 6 + 0xebc) * 8;
            local_1a = CONCAT22(paVar12, u_var7);
            process_struct_1000_179c(0x1a, paVar12);
            if ((paVar12 | u_var7) == 0) {
                uVar18 = 0;
                u_var13 = 0;
            } else {
                pu16_19 = pass1_1010_37d4(CONCAT22(paVar12, u_var7));
              // u_var13 = (pu16_19  >> 0x10);
                uVar18 = SUB42(pu16_19, 0);
            }
            (local_AX_179 + local_e * 4) = uVar18;
            (local_AX_179 + local_e * 4 + 2) = u_var13;
            u_var4 = pustruct_a.field_0x64;
          // uVar18 = (u_var4  >> 0x10);
            iVar16 = u_var4;
            local_2a = local_2a + 1;
            u_var4 = (local_AX_179 + local_e * 4);
            ppc_var2 = ((local_AX_179 + local_e * 4) + 0x1c);
            ppc_var2(
                0x1000,
                u_var4,
                (u_var4 >> 0x10),
                local_2a,
                (iVar16 + local_1c * 4) + (u16_1050_0ecc * 6 + 0xebc) * 8,
                (iVar16 + local_1c * 4 + 2),
            );
            local_28 = 0x0;
            paVar12 = ctx.dx_reg;
            loop {
                pu16_19 = &pustruct_a.field_0x68;
                let var = unsafe { *pu16_19 };
                if (val == local_1c || val < local_1c) {
                    break;
                }
                u_var4 = pustruct_a.field_0x64;
                u_var4 = (u_var4 + local_1c * 4);
                iVar16 = process_string_1000_475e(
                    *local_1a,
                    *(u_var4 + (u16_1050_0ecc * 6 + 0xebc) * 8),
                );
                if (iVar16 != 0) {
                    break;
                }
                u_var4 = pustruct_a.field_0x64;
                u_var4 = (u_var4 + local_1c * 4);
                u_var7 = process_string_1000_475e(
                    *local_16,
                    *(u_var4 + (u16_1050_0ecc * 6 + 0xeba) * 8),
                );
                if (u_var7 != 0) {
                    break;
                }
                process_struct_1000_179c(0x1a, paVar12);
                if ((paVar12 | u_var7) == 0) {
                    uVar18 = 0;
                    u_var13 = 0;
                } else {
                    pu16_19 = pass1_1010_37d4(CONCAT22(paVar12, u_var7));
                  // u_var13 = (pu16_19  >> 0x10);
                    uVar18 = SUB42(pu16_19, 0);
                }
                (paVar6 + local_28 * 4) = uVar18;
                (paVar6 + local_28 * 4 + 2) = u_var13;
                u_var4 = pustruct_a.field_0x64;
              // uVar18 = (u_var4  >> 0x10);
                iVar16 = u_var4;
                local_2a = local_2a + 1;
                u_var4 = (paVar6 + local_28 * 4);
                ppc_var2 = ((paVar6 + local_28 * 4) + 0x1c);
                ppc_var2(
                    0x1000,
                    u_var4,
                    (u_var4 >> 0x10),
                    local_2a,
                    (iVar16 + local_1c * 4) + (u16_1050_0ecc * 6 + 0xebe) * 8,
                    (iVar16 + local_1c * 4 + 2),
                );
                local_1c = local_1c + 1;
                local_28 = &local_28.field_0x1;
                paVar12 = ctx.dx_reg;
            }
            u_var4 = (local_AX_179 + local_e * 4);
            (u_var4 + 0x10) = local_28;
            u_var8 = local_28 << 2;
            paVar20 = paVar6;
            paVar12 = pa_var10;
            u_var21 = u_var8;
            process_struct_1010_38f8((local_AX_179 + local_e * 4), local_28);
            pass1_fn_1000_48a8(
                CONCAT22(ctx.dx_reg, u_var8),
                CONCAT22(paVar12, paVar20),
                u_var21,
            );
            pass1_1000_4906(_local_26, 0, 0x400);
            local_e = local_e & 0xffff | (local_e + 1) << 0x10;
        }
        u_var4 = (i_var5 + 8);
        u_var4 = (u_var4 + local_e * 4);
        (u_var4 + 0x10) = local_e;
        u_var8 = local_e << 2;
        u_var4 = (i_var5 + 8);
        paVar20 = local_AX_179;
        paVar12 = struct_a_00;
        u_var21 = u_var8;
        process_struct_1010_38f8((u_var4 + local_e * 4), local_e);
        pass1_fn_1000_48a8(
            CONCAT22(ctx.dx_reg, u_var8),
            CONCAT22(paVar12, paVar20),
            u_var21,
        );
        pass1_1000_4906(CONCAT22(struct_a_00, local_AX_179), 0, 0x400);
        local_e = (local_e + 1);
    }
}

pub unsafe fn infinite_loop_1020_7bba(ctx: &mut AppContext) {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let paVar3: &mut  Struct676;
    let pu_var4: &mut  u16;
    let mut cVar5: u8;
    let mut bVar6: u8;
    let mut in_AL: u8;
    let mut bVar7: u8;
    let mut bVar8: u8;
    let mut paVar9: &mut  Struct676;
    let mut iVar10: u16;
    let in_cx: &mut  Struct677;
    let mut u_var11: i32;
    let local_CX_56: &mut  Struct677;
    let mut in_dx: u16;
    let mut u_var12: i32;
    let mut in_bx: u16;
    let local_bx_59: &mut  Struct678;
    let local_bx_110: &mut  Struct679;
    let ppu_var13: &mut  &mut  u16;
    let unaff_bp: &mut  &mut  u16;
    let unaff_si: &mut  Struct676;
    let local_SI_28: &mut  Struct676;
    let paVar14: &mut  Struct676;
    let unaff_DI: &mut  Struct676;
    let paVar15: &mut  Struct676;
    let mut unaff_es: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut unaff_ss: u16;
    let mut bVar18: bool;
    let mut in_af: u8;
    let mut in_TF: u8;
    let mut in_IF: u8;
    let mut in_NT: u8;
    let mut in_stack_00000034: u16;
    let mut cStack0035: u8;
    let mut uStack2269: u16;
    let mut uStack2257: i32;
    let ppuStack2255: &mut  &mut  u16;
    let paStack2251: &mut  Struct676;
    let puStack2249: &mut  u16;
    let mut uStack2245: u32;
    let mut uStack2241: i32;
    let paStack2239: &mut  Struct677;
    let paStack2237: &mut  Struct676;
    let apaStack2235: &mut  Struct676;
    let ppuStack34: &mut  &mut  u16;
    let puStack2: &mut  u16;

    loop {
        ppuStack34 = &puStack2;
        ppu_var13 = &puStack2;
        ppuStack2255 = &puStack2;
        cVar5 = '\x0f';
        pu_var4 = unaff_bp;
        while {
            pu_var4 = pu_var4 + -1;
            ppu_var13 = ppu_var13 + -1;
            *ppu_var13 = *pu_var4;
            cVar5 = cVar5 + -1;
            '\0' < cVar5
        } {}
        local_SI_28 = in_AL;
        pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
        bVar8 = in_dx;
        *pu8_var2 = *pu8_var2 | bVar8;
        let val = *pu8_var2;
        if ((POPCOUNT(val) & 1) == 0) {
            bVar18 = bVar8;
            u_var12 = in_dx & 0xff00 | (bVar8 * 0x2);
            paVar15 = unaff_DI;
            paVar9 = unaff_si;
            if ((POPCOUNT(bVar8 * 0x2) & 1) != 0) {
                return code_r0x10207be9();
            }
        } else {
            pu8_var2 = &local_SI_28.field_0x37;
            *pu8_var2 = *pu8_var2 + bVar8;
            pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
            *pu8_var2 = *pu8_var2 | bVar8;
            uStack2245 = CONCAT22(in_bx, apaStack2235);
            puStack2249 = CONCAT22(&puStack2, local_SI_28);
            let val = *pu8_var2;
            paVar14 = ((in_NT & 1) * 0x4000
                | (in_IF & 1) * 0x200
                | (in_TF & 1) * 0x100
                | (val < '\0') * 0x80
                | (val == 0) * 0x40
                | (in_af & 1) * 0x10
                | ((POPCOUNT(val) & 1) == 0) * 4);
            pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
            *pu8_var2 = *pu8_var2 | bVar8;
            paVar3 = unaff_DI;
            paVar15 = &unaff_DI.field_0x1;
            bVar7 = unaff_si;
            bVar18 = bVar7 < *paVar3;
            uStack2241 = in_dx;
            paStack2239 = in_cx;
            paStack2237 = unaff_si;
            let val = *paVar3;
            if ((bVar7 - val) < '\0') {
                cStack0035 = cStack0035 + in_bx + bVar18;
                pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
                *pu8_var2 = *pu8_var2 | bVar8;
                local_SI_28 = &local_SI_28[-1].field_0x37;
                pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
                *pu8_var2 = *pu8_var2 | bVar8;
                pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
                *pu8_var2 = *pu8_var2 | bVar8;
                u_var12 = in_dx - 1;
                uStack2257 = (in_NT & 1) * 0x4000
                    | SBORROW2(in_dx, 1) * 0x800
                    | (in_IF & 1) * 0x200
                    | (in_TF & 1) * 0x100
                    | (u_var12 < 0) * 0x80
                    | (u_var12 == 0) * 0x40
                    | (in_af & 1) * 0x10
                    | ((POPCOUNT(u_var12 & 0xff) & 1) == 0) * 4;
                pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
                unsafe { *pu8_var2 = *pu8_var2 | u_var12 };
                in_af = 9 < (bVar7 & 0xf) | in_af;
                pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
                unsafe { *pu8_var2 = *pu8_var2 | u_var12 };
                pc_var1 = &stack0x001e + local_SI_28;
                bVar8 = in_cx & 0x1f;
                unsafe { cVar5 = *pc_var1 };
                unsafe { *pc_var1 = *pc_var1 >> bVar8 };
                bVar18 = (in_cx & 0x1f) != 0 && (cVar5 >> bVar8 - 1 & 1) != 0;
                // code_r0x10207be9:
                pc_var1 = &stack0x0068 + local_SI_28;
                unsafe { *pc_var1 = *pc_var1 + in_cx + bVar18 };
                pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
                unsafe { *pu8_var2 = *pu8_var2 | u_var12 };
                u_var11 = (&local_SI_28.field_0x0 + in_bx) * 0x10;
                pu8_var2 = &stack0x0006 + paVar15;
                local_CX_56 = (u_var11 >> 8);
                unsafe { local_CX_56 = local_CX_56 + *pu8_var2 };
                local_CX_56 = (u_var11 & 0xff | local_CX_56 << 8);
                pc_var1 = &stack0x0000 + &local_SI_28.field_0x35;
                local_bx_59 = (in_bx >> 8);
                unsafe {
                    *pc_var1 = *pc_var1 + local_bx_59 + CARRY1(local_CX_56, *pu8_var2)
                };
                in_dx = u_var12;
            } else {
                pc_var1 = ((register0x00000010 + -2) + local_SI_28);
                unsafe { *pc_var1 = *pc_var1 + bVar7 + bVar18 };
                local_CX_56 = ((&local_SI_28.field_0x0 + in_bx) * 0x10);
                paStack2251 = unaff_DI;
                let val = unsafe { *pc_var1 };
                if ((POPCOUNT(val) & 1) == 0) {
                    // code_r0x10207c14:
                    pu8_var2 = (&paVar14.field_0x0 + in_bx);
                    unsafe { *pu8_var2 = *pu8_var2 | in_dx };
                    pu8_var2 = (&paVar14.field_0x0 + in_bx);
                    unsafe { *pu8_var2 = *pu8_var2 & in_dx };
                  // uVar16 = (uStack2245  >> 0x10);
                    uStack2269 = (uStack2245 + 8);
                  // uVar17 = (puStack2249  >> 0x10);
                    local_bx_110 = puStack2249;
                    unsafe { *puStack2249 = ctx.s_1_1050_389a };
                    local_bx_110.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                    unsafe { *puStack2249 = (ctx.s_18_2_1050_3aa5 + 3) };
                    local_bx_110.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                    local_bx_110.field_0x4 = uStack2269;
                    unsafe { *puStack2249 = ctx.s_0_020_1050_3ab0 };
                    local_bx_110.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                    local_bx_110.field_0x6 = uStack2245;
                    local_bx_110.field_0xa = 0;
                    local_bx_110.field_0xe = 0;
                    local_bx_110.field_0x10 = 0;
                    local_bx_110.field_0x12 = 0;
                    unsafe { *puStack2249 = 0x7f72 };
                    local_bx_110.field_0x2 = 0x1020;
                    local_bx_110.field_0xa = (uStack2245 + 0xe4);
                    apaStack2235[0] = unaff_si;
                    puStack2 = unaff_bp;
                    iVar10 = GetSystemMetrics16(4);
                    (puStack2249 + 0xe) = iVar10;
                    iVar10 = GetSystemMetrics16(5);
                    (puStack2249 + 0x10) = iVar10;
                    iVar10 = GetSystemMetrics16(6);
                    (puStack2249 + 0x12) = iVar10;
                    return;
                }
            }
            pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
            bVar7 = in_dx;
            unsafe { *pu8_var2 = *pu8_var2 | bVar7 };
            let val = unsafe { *pu8_var2 };
            puStack2249 = CONCAT22(
                in_dx,
                (in_NT & 1) * 0x4000
                    | (in_IF & 1) * 0x200
                    | (in_TF & 1) * 0x100
                    | (val < '\0') * 0x80
                    | (val == 0) * 0x40
                    | (in_af & 1) * 0x10
                    | ((POPCOUNT(val) & 1) == 0) * 4,
            );
            pu8_var2 = (&local_SI_28.field_0x0 + in_bx);

            unsafe { *pu8_var2 = *pu8_var2 | bVar7 };
            let val = unsafe { *pu8_var2 };
            paVar14 = ((in_NT & 1) * 0x4000
                | (in_IF & 1) * 0x200
                | (in_TF & 1) * 0x100
                | (val < '\0') * 0x80
                | (val == 0) * 0x40
                | (in_af & 1) * 0x10
                | ((POPCOUNT(val) & 1) == 0) * 4);
            pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
            unsafe { *pu8_var2 = *pu8_var2 | bVar7 };
            bVar8 = (0x79);
            pu8_var2 = (&local_SI_28.field_0x0 + in_bx);
            unsafe { *pu8_var2 = *pu8_var2 & bVar7 };
            in_cx = local_CX_56;
            u_var12 = in_dx;
            unaff_DI = paVar15;
            paVar9 = bVar8;
            paStack2251 = local_SI_28;
            let val = unsafe { *pu8_var2 };
            if (-1 < val) {
                // goto code_r0x10207c14;
            }
        }
        paVar14 = local_SI_28;
        pu8_var2 = (&paVar14.field_0x0 + in_bx);
        bVar8 = u_var12;
        unsafe { *pu8_var2 = *pu8_var2 | bVar8 };
        in_cx = in_cx + -1;
        in_dx = u_var12;
        let val = unsafe { *pu8_var2 };
        if (in_cx == 0x0 || val == 0) {
            // goto code_r0x10207c14;
        }
        pu8_var2 = (&paVar14.field_0x0 + in_bx);
        unsafe { *pu8_var2 = *pu8_var2 | bVar8 };
        bVar6 = paVar9 - 1;
        bVar7 = 9 < (bVar6 & 0xf) | in_af;
        bVar6 = bVar6 + bVar7 * '\x06' & 0xf;
        pu8_var2 = (&paVar14.field_0x0 + in_bx);
        unsafe { *pu8_var2 = *pu8_var2 | bVar8 };
        in_af = 9 < bVar6 | bVar7;
        in_AL = bVar6 + in_af * '\x06' & 0xf;
        pu8_var2 = (&paVar14.field_0x0 + in_bx);
        unsafe { *pu8_var2 = *pu8_var2 | bVar8 };
        unaff_bp = &puStack2;
        unaff_si = paVar14;
    }
}
