use crate::util::{POPCOUNT, CONCAT22, CARRY1, SBORROW2, SUB42, ZEXT24};
use crate::winapi::GetSystemMetrics16;
use crate::structs::prog_structs_6::{Struct676, Struct677, Struct678};
use crate::structs::prog_structs_2::{Struct679, Struct199, Struct7};
use crate::app_context::AppContext;
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_fn_1000_48a8};
use crate::struct_ops::struct_ops_2::{process_struct_1010_38f8, process_struct_1000_179c};
use crate::pass::pass8_funcs::pass1_1010_37d4;
use crate::string_ops::misc::process_string_1000_475e;
use crate::structs::prog_structs_16::Struct368;
use crate::structs::prog_structs_31::{Struct367, Struct366};
use crate::structs::prog_structs_30::Struct365;

pub unsafe fn loop_1010_11c6(ctx: &mut AppContext, param_1: &mut Struct365) {
    let var16: &mut  i32;
    let var2: &mut  fn();
    let mut var3: u32;
    let mut var4: u32;
    // let ctx.ax_reg: &mut  Struct366;
    let mut var5: i32;
    let var15: &mut  Struct367;
    let var6: &mut  Struct367;
    let mut var7: i32;
    let mut var8: u16;
    // let in_dx: &mut  Struct199;
    let mut var9: u16;
    let var13: &mut  Struct7;
    let var14: &mut  Struct7;
    let var10: &mut  Struct7;
    let var11: &mut  Struct7;
    let var12: &mut  Struct7;
    let mut var17: u16;
    // let ctx.dx_reg: &mut  Struct199;
    let var18: &mut  String;
    let mut var19: &mut Struct365;
    let mut var20: i32;
    let mut var21: i32;
    let mut var22: u16;
    let mut var23: u16;
    let var24: &mut  u16;
    let var25: &mut  Struct367;
    let mut var26: u16;
    let mut var27: u16;
    let mut var28: u16;
    let mut var29: u16;
    let mut var30: u16;
    let mut var31: u16;
    let var32: &mut  Struct368;
    let mut var33: u16;
    let mut var34: u16;
    let mut var35: u16;
    let mut var36: u16;
    let mut var37: u16;
    let mut var38: u16;
    let mut var39: u32;
    let mut var40: u32;
    let mut var41: u32;
    let mut var42: u16;
    let mut var43: u16;
    let mut var44: u16;
    let mut var45: u16;

    if ctx.u16_1050_0ecc == 0xffff {
        return;
    }
    process_struct_1000_179c(ctx, 0x1a, in_dx);
    if (in_dx | in_ax) == 0 {
        var5 = 0;
        var9 = 0;
    } else {
        var24 = pass1_1010_37d4(CONCAT22(in_dx, in_ax));
      // u_var9 = (pu16_19  >> 0x10);
        var5 = var24;
    }
    _local_a = &ctx.WORD_1050_0ece;
    var41 = 0;
    loop {
        // uVar17 = (param_1  >> 0x10);
        var19 = param_1;
        var24 = &var19.field_0x68;
        let val = unsafe { *var24 };
        if val == var41 || val < var41 {
            break;
        }
        var4 = var19.field_0x64;
        var3 = (var4 + var41 * 4);
        var18 = (var3 + u16_1050_0ecc * 8);
        _local_32 = (var3 & 0xffff0000 | var18);
        var21 = process_string_1000_475e(_local_a, *var18);
        if var21 != 0 {
            _local_a = *_local_32;
            var41 = var41 & 0xffff | (var41 + 1) << 0x10;
        }
        var41 = var41 & 0xffff0000 | (var41 + 1);
    }
    (var5 + 0x10) = var41;
    process_struct_1010_38f8(CONCAT22(var9, var5), var41);
    var15 = 0x0;
    var14 = var13;
    process_struct_1000_179c(ctx,0x400, var13);
    var10 = var14;
    var6 = var15;
    process_struct_1000_179c(ctx,0x400, var14);
    _local_26 = CONCAT22(var10, var6);
    var38 = 0;
    pass1_1000_4906(CONCAT22(var14, var15), 0, 0x400);
    pass1_1000_4906(CONCAT22(var10, var6), 0, 0x400);
    var31 = 0;
    var41 = 0;
    loop {
        var16 = (var5 + 0x10);
        let val = unsafe { *var16 };
        if val == var41 || val < var41 {
            return;
        }
        var4 = var19.field_0x64;
      // uVar18 = (u_var4  >> 0x10);
        var20 = var4;
        var21 = (var20 + var38 * 4);
        var12 = (var20 + var38 * 4 + 2);
        var20 = var21 + (u16_1050_0ecc * 6 + 0xeba) * 8;
        var40 = CONCAT22(var12, var20);
        var7 = var21 + (u16_1050_0ecc * 6 + 0xebc) * 8;
        var11 = var12;
        process_struct_1000_179c(ctx,0x1a, var12);
        if (var11 | var7) == 0 {
            var4 = (var5 + 8);
            (var4 + var41 * 4) = 0;
        } else {
            var24 = pass1_1010_37d4(CONCAT22(var11, var7));
            var4 = (var5 + 8);
          // uVar18 = (u_var4  >> 0x10);
            var21 = var4;
            (var21 + var41 * 4) = var24;
            (var21 + var41 * 4 + 2) = (var24 >> 0x10);
        }
        var31 = var31 + 1;
        var4 = (var5 + 8);
      // uVar18 = (u_var4  >> 0x10);
        var21 = var4;
        var4 = (var21 + var41 * 4);
        var2 = ((var21 + var41 * 4) + 0x1c);
        var2(0x1000, var4, (var4 >> 0x10), var31, var20, var12);
        // local_16
        loop {
            var24 = &var19.field_0x68;
            let val = unsafe { *var24 };
            if val == var38 || val < var38 {
                break;
            }
            var20 = var38 * 4;
            var4 = var19.field_0x64;
            var4 = (var4 + var20);
            var21 =
                process_string_1000_475e(*var40, *(var4 + (u16_1050_0ecc * 6 + 0xeba) * 8));
            if var21 != 0 {
                break;
            }
            var4 = var19.field_0x64;
          // uVar18 = (u_var4  >> 0x10);
            var21 = var4;
            var12 = (var21 + var20 + 2);
            var7 = (var21 + var20) + (u16_1050_0ecc * 6 + 0xebc) * 8;
            var39 = CONCAT22(var12, var7);
            process_struct_1000_179c(ctx,0x1a, var12);
            if (var12 | var7) == 0 {
                var23 = 0;
                var17 = 0;
            } else {
                var24 = pass1_1010_37d4(CONCAT22(var12, var7));
              // u_var13 = (pu16_19  >> 0x10);
                var23 = SUB42(var24, 0);
            }
            (var15 + var41 * 4) = var23;
            (var15 + var41 * 4 + 2) = var17;
            var4 = var19.field_0x64;
          // uVar18 = (u_var4  >> 0x10);
            var21 = var4;
            var31 = var31 + 1;
            var4 = (var15 + var41 * 4);
            var2 = ((var15 + var41 * 4) + 0x1c);
            var2(
                0x1000,
                var4,
                (var4 >> 0x10),
                var31,
                (var21 + var38 * 4) + (u16_1050_0ecc * 6 + 0xebc) * 8,
                (var21 + var38 * 4 + 2),
            );
            var32 = 0x0;
            var12 = in_dx;
            loop {
                var24 = &var19.field_0x68;
                let var = unsafe { *var24 };
                if val == var38 || val < var38 {
                    break;
                }
                var4 = var19.field_0x64;
                var4 = (var4 + var38 * 4);
                var21 = process_string_1000_475e(
                    *var39,
                    *(var4 + (ctx.u16_1050_0ecc * 6 + 0xebc) * 8),
                );
                if var21 != 0 {
                    break;
                }
                var4 = var19.field_0x64;
                var4 = (var4 + var38 * 4);
                var7 = process_string_1000_475e(
                    *var40,
                    *(var4 + (ctx.u16_1050_0ecc * 6 + 0xeba) * 8),
                );
                if var7 != 0 {
                    break;
                }
                process_struct_1000_179c(ctx,0x1a, var12);
                if (var12 | var7) == 0 {
                    var23 = 0;
                    var17 = 0;
                } else {
                    var24 = pass1_1010_37d4(CONCAT22(var12, var7));
                  // u_var13 = (pu16_19  >> 0x10);
                    var23 = SUB42(var24, 0);
                }
                (var6 + var32 * 4) = var23;
                (var6 + var32 * 4 + 2) = var17;
                var4 = var19.field_0x64;
              // uVar18 = (u_var4  >> 0x10);
                var21 = var4;
                var31 = var31 + 1;
                var4 = (var6 + var32 * 4);
                var2 = ((var6 + var32 * 4) + 0x1c);
                var2(
                    0x1000,
                    var4,
                    (var4 >> 0x10),
                    var31,
                    (var21 + var38 * 4) + (u16_1050_0ecc * 6 + 0xebe) * 8,
                    (var21 + var38 * 4 + 2),
                );
                var38 = var38 + 1;
                var32 = var32.field_0x1;
                var12 = ctx.dx_reg;
            }
            var4 = (var15 + var41 * 4);
            (var4 + 0x10) = var32;
            var8 = var32 << 2;
            var25 = var6;
            var12 = var10;
            var26 = var8;
            process_struct_1010_38f8((var15 + var41 * 4), var32);
            pass1_fn_1000_48a8(
                CONCAT22(ctx.dx_reg, var8),
                CONCAT22(var12, var25),
                var26,
            );
            pass1_1000_4906(_local_26, 0, 0x400);
            var41 = var41 & 0xffff | (var41 + 1) << 0x10;
        }
        var4 = (var5 + 8);
        var4 = (var4 + var41 * 4);
        (var4 + 0x10) = var41;
        var8 = var41 << 2;
        var4 = (var5 + 8);
        var25 = var15;
        var12 = var14;
        var26 = var8;
        process_struct_1010_38f8((var4 + var41 * 4), var41);
        pass1_fn_1000_48a8(
            CONCAT22(ctx.dx_reg, var8),
            CONCAT22(var12, var25),
            var26,
        );
        pass1_1000_4906(CONCAT22(var14, var15), 0, 0x400);
        var41 = (var41 + 1);
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
            if (bVar7 - val) < '\0' {
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
