use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::unk::block_1000_1000::mem_1000_167a;
use crate::unk::block_1000_2000::pass1_1000_29b5;
use crate::unk::block_1000_3000::{str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::unk::block_1000_4000::dos3_call_set_struct_1000_42de;
use crate::dos_interrupt::{dos3_call_1000_4f94, swi};
use crate::structs::struct_825::Struct825;
use crate::utils::{CARRY2, CONCAT22, SUB42};

pub fn pass1_1000_5008(mut param_1: u16,
                              mut param_2: u16,
                              mut param_3: u16 )
{
    pass1_1000_5026(
        ctx, 0x0,
        param_1,
        param_2,
        param_3);
}




pub fn pass1_1000_5026(
    ctx: &mut AppContext, mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16)
{
    let mut var1: u16;
    let mut var2: u16;
    let mut var4: [u16;3] = [0;3];
    let mut var5: u16;
    let mut var6: [*mut u8;6] = [null_mut();6];
    let mut var7: i16;
    let mut var8: u8;
    let mut var9: u8;
    let mut var10: u8;
    // let mut var11: *mut u8;
    let mut var12: u8;
    let mut var13: u8;
    let mut var14: u8;
    let mut var15: [u8;0x101] = [0;0x101];


    let mut var17 = ctx.BP_REG + 1;
    let var16 = 0x1050u16;
    let mut var3 =  &var12;
    if param_1 == 0 {
        param_1 = dos3_call_1000_4f94();
    }
    *var3 =  param_1 + '@';
    let mut var13 = 0x3au8;
    let mut var11 = var15;
    let mut var14 = 0x5cu8;
    let mut var9 = 0x47u8;
    let mut var10 =  param_1;
    var4[0] = 0x1050u16;
    let mut var5 = 0x1050u16;
    dos3_call_set_struct_1000_42de(&mut var8,&mut var6, &mut var4);
    if var7 == 0 {
        var1 = str_op_1000_3da4( &var12);
        var1 += 0x1;
        var3 = param_2;
        var3 = param_3;
        var2 = param_3 | param_2;
        if var2 == 0 {
            if param_4 < var1 {
                param_4 = var1;
            }
            var3 = mem_1000_167a(0x0,
                                 param_4);
            var3 = var2;
            if (var2 | var3) == 0 {
                PTR_LOOP_1050_5f78 =  0xc;
                return;
            }
        }
        if param_4 < var1 {
            PTR_LOOP_1050_5f78 = 0x22;
        } else {
            unk_str_op_1000_3d3e(var3,&var12);
        }
    } else {
        PTR_LOOP_1050_5f78 =  0xd;
        PTR_LOOP_1050_5f88 = var6[0];
    }
    return;
}





pub fn dos3_call_1000_514e() -> u16
{
    // let mut pc_var1: *mut code;
    // let mut u_var2: u16;
    let mut unaff_bp: i16;
    // let mut b_var2: bool;

    let mut b_var2 = false;
    let pc_var1 =  swi(0x21);
    let u_var2 = (*pc_var1)( 0x1050,
                      unaff_bp + 1);
    if (b_var2) {
        pass1_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0x0;
}





pub fn dos3_call_1000_5174() -> u16
{
    // let mut pc_var1: *mut code;
    // let mut u_var2: u16;
    let mut unaff_bp: i16;
    // let mut b_var2: bool;

    let mut b_var2 = false;
    let mut pc_var1 =  swi(0x21);
    let u_var2 = (*pc_var1)(unaff_bp + 1);
    if (!b_var2) {
        return 0x0;
    }
    pass1_1000_29b5(u_var2);
    return u_var2 & 0xff;
}

pub fn dos3_call_1000_51aa(mut param_1: u16 ,
                        mut param_2: u16 ,
                        mut param_3: u16 ) -> u16
{
    // let mut pc_var1: *mut code;
    // let mut u_stack000a: u16;

    let mut pc_var1 =  swi(0x21);
    (*pc_var1)( 0x1050);
    pc_var1 =  swi(0x21);
    (*pc_var1)();
    pc_var1 =  swi(0x21);
    (*pc_var1)();
    pc_var1 =  swi(0x21);
    (*pc_var1)();
    if ((param_2 & 0x100) == 0) {
        return 0x0;
    }
    let mut u_stack000a = param_3;
    pass1_1000_29b5(param_3);
    return u_stack000a & 0xff;
}

pub fn pass1_1000_52be(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    if ((param_4 | param_2) == 0) {
        return  param_1 *  param_3;
    }
    return  param_1 *  param_3 & 0xffff
        |  ( ( param_1 *  param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}

pub fn pass1_1000_52f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut bVar12: bool;
    let mut bVar13: bool;

    bVar13 =  param_2 < 0x0;
    if (bVar13) {
        bVar12 = param_1 != 0;
        param_1 = -param_1;
        param_2 = - bVar12 - param_2;
    }
    uVar11 =  bVar13;
    if ( param_4 < 0x0) {
        bVar13 = param_3 != 0;
        param_3 = -param_3;
        param_4 = - bVar13 - param_4;
    }
    uVar3 = param_1;
    uVar4 = param_3;
    uVar8 = param_2;
    uVar9 = param_4;
    if (param_4 == 0) {
        iVar5 =  (( param_2 %  param_3 << 0x10 |  param_1) %  param_3);
        iVar6 = 0;
        if ( (uVar11 - 1) < 0x0) {
        // TODO: goto LAB_1000_538a;
        }
    } else {
        loop {
            uVar10 = uVar9 >> 0x1;
            uVar4 = uVar4 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar7 = uVar8 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar8 & 1) != 0) << 0xf;
            uVar8 = uVar7;
            uVar9 = uVar10;
            if uVar10 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar4;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) *  param_3;
        uVar8 =  ( lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar9 = uVar8 + uVar3;
        if (((CARRY2(uVar8,
                     uVar3)) || (param_2 < uVar9)) || ((param_2 <= uVar9 && (param_1 < uVar4)))) {
            bVar13 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar9 = (uVar9 - param_4) -  bVar13;
        }
        iVar5 = uVar4 - param_1;
        iVar6 = (uVar9 - param_2) -  (uVar4 < param_1);
        if (-0x1 <  (uVar11 - 1)) {
        // TODO: goto LAB_1000_538a;
        }
    }
    bVar13 = iVar5 != 0;
    iVar5 = -iVar5;
    iVar6 = - bVar13 - iVar6;//
//    LAB_1000_538a:
    return CONCAT22(iVar6,
                    iVar5);
}

pub fn pass1_1000_5390(mut param_1: u32,
                              mut param_3: u16,
                              mut param_4: *mut u8) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;

    uVar3 = param_1;
    uVar8 = param_4;
    uVar6 = param_2;
    uVar9 = param_3;
    if (param_4 == 0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (( param_2 %  param_3 << 0x10 |  param_1) /  param_3);
    } else {
        loop {
            uVar5 = uVar8 >> 0x1;
            uVar9 = uVar9 >> 0x1 |  ((uVar8 & 1) != 0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 1) != 0) << 0xf;
            uVar8 = uVar5;
            uVar6 = uVar7;
            if uVar5 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar9;
        iVar4 =  uVar1;
        lVar2 =  param_3 * (uVar1 & 0xffff);
        uVar3 =  ( lVar2 >> 0x10);
        uVar8 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0;
    }
    return CONCAT22(uVar3,
                    iVar4);
}

pub fn pass1_1000_545a(mut param_1: u32,
                    mut param_2: u32) -> i16
{
    let mut pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut pbVar5: *mut u8;
    let mut pbVar6: *mut u8;

    pbVar6 =  param_2;
    pbVar5 =  param_1;
    bVar4 = 0xff;
    loop {
        loop {
            if (bVar4 == 0) {
            // TODO: goto LAB_1000_5499;
            }
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 1;
            bVar4 = *pbVar1;
            bVar3 = *pbVar5;
            pbVar5 = pbVar5 + 1;
            if bVar3 != bVar4 {break;}
        }
        bVar2 = bVar4 + 0xbf + (-( (bVar4 + 0xbf) < 0x1a) & 0x20) + 0x41;
        bVar3 += 0xbf;
        bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20) + 0x41;
        if bVar4 != bVar2 {break;}
    }
    bVar4 = (bVar4 < bVar2) * -0x2 + 1;//
//    LAB_1000_5499:
    return   bVar4;
}

pub fn pass1_1000_54a0(mut param_1: u32,
                     mut param_2: u16 ,
                     mut param_3: u16 ) -> *mut u16
{
    let mut pu_var1: *mut u16;
    let mut u_var2: u8;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut pu_var7: *mut u16;
    let mut i_var8: i16;

    if (param_3 != 0) {
        i_var8 =  (param_1 >> 0x10);
        u_var5 = -  param_1;
        u_var6 = param_3;
        if (u_var5 != 0) {
            u_var6 = (u_var5 - param_3 & - (u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = param_2 & 0xff | param_2 << 0x8;
        pu_var7 =  param_1;
        // for (uVar4 = uVar6 >> 0x1; uVar4 != 0; uVar4 -= 1)
        for uVar4 in u_var6 >> 1 .. 0
        {
            pu_var1 = pu_var7;
            pu_var7 = pu_var7 + 1;
            *pu_var1 = u_var3;
        }
        u_var6 = u_var6 & 1 != 0;
        u_var2 = param_2 & 0xff;
        // for (uVar6 =  ((uVar6 & 1) != 0); uVar2 =  (param_2 & 0xff), uVar6 != 0; uVar6 -= 1) {
            while u_var6 !=0 {
            pu_var1 = pu_var7;
            pu_var7 =  ( pu_var7 + 1);
            * pu_var1 = u_var2;
            u_var6 -= 1;
        }
        if (u_var5 != 0) {
            // for (uVar6 = u_var5 >> 0x1; u_var6 != 0; u_var6 -= 1)
            for u_var6 in u_var5 >> 1 .. 0
            {
                pu_var1 = pu_var7;
                pu_var7 = pu_var7 + 1;
                *pu_var1 = u_var3;
            }
            // for (uVar6 =  ((u_var5 & 1) != 0); u_var6 != 0; u_var6 -= 1)
            u_var6 = u_var5 & 1 != 0;

            {
                pu_var1 = pu_var7;
                pu_var7 =  ( pu_var7 + 1);
                * pu_var1 = u_var2;
            }
        }
    }
    return  param_1;
}
pub fn pass1_1000_54e8(param_1: *mut u8,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: i16,
                     mut param_6: u16 )
{
    let mut i_var1: i16;

    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        ( param_1)();
    }
    return;
}


/*
Unable to decompile 'pass1_1000_55b1'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/

pub fn pass1_1000_55b1(ctx: &mut AppContext, p1: *mut Struct825, p2: i16) -> u16 {
    todo!()
}

pub fn pass1_1000_5586(param_1: code,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: i16,
                     mut param_6: u16 )
{
    let mut i_var1: i16;

    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        ( param_1)();
    }
}

pub fn ret_op_1000_55ac() -> u32
{
    todo!()
}

pub fn exit_1000_25f2(a: i16, b: i16, c: i16, d: i16) {
    todo!()
}


//dos3_call_1000_23ea( param_4,
//                         0x1050,
//                        0x0,
//                         0x1050);
pub fn dos_get_interrupt_vector_1000_23ea(a: *mut u8, b: u16, c: u16, d: u16) {
    todo!()
    //                              **************************************************************
    //                              *                          FUNCTION                          *
    //                              **************************************************************
    //                              int * __cdecl16far dos3_call_1000_23ea(ushort param_1, u
    //                                assume DS = 0x1050
    //                                assume SS = 0x1050
    //              int *             AX:2           <RETURN>
    //              ushort            BX:2           param_1
    //              ushort            ES:2           param_2
    //                              dos3_call_1000_23ea                             XREF[1]:     entry:1000:2385(c)
    //        1000:23ea 8c d8           MOV        AX,DS
    //        1000:23ec 90              NOP
    //        1000:23ed 45              INC        BP
    //        1000:23ee 55              PUSH       BP
    //        1000:23ef 8b ec           MOV        BP,SP
    //        1000:23f1 1e              PUSH       DS
    //        1000:23f2 8e d8           MOV        DS,AX
    //        1000:23f4 b8 00 35        MOV        AX,0x3500 ; 0x35 --> get interrupt vector
    //        1000:23f7 2e f7 06        TEST       word ptr CS:[DAT_1000_22f6],0x1                  = 0088h
    //                  f6 22 01 00
    //        1000:23fe 74 07           JZ         LAB_1000_2407
    //        1000:2400 9a 60 00        CALLF      KERNEL::DOS3Call                                 void DOS3Call(CONTEXT * context)
    //                  38 15
    //        1000:2405 eb 02           JMP        LAB_1000_2409
    //                              LAB_1000_2407                                   XREF[1]:     1000:23fe(j)
    //        1000:2407 cd 21           INT        0x21
    //                              LAB_1000_2409                                   XREF[1]:     1000:2405(j)
    //        1000:2409 89 1e 6a 5f     MOV        word ptr [PTR_LOOP_1050_5f6a],param_1            = 1050:0000
    //        1000:240d 8c 06 6c 5f     MOV        word ptr [PTR_LOOP_1050_5f6c],param_2            = 1050:0000
    //        1000:2411 0e              PUSH       CS
    //        1000:2412 1f              POP        DS
    //        1000:2413 b8 00 25        MOV        AX,0x2500
    //        1000:2416 ba 90 28        MOV        DX,0x2890
    //        1000:2419 2e f7 06        TEST       word ptr CS:[DAT_1000_22f6],0x1                  = 0088h
    //                  f6 22 01 00
    //        1000:2420 74 07           JZ         LAB_1000_2429
    //        1000:2422 9a 60 00        CALLF      KERNEL::DOS3Call                                 void DOS3Call(CONTEXT * context)
    //                  38 15
    //        1000:2427 eb 02           JMP        LAB_1000_242b
    //                              LAB_1000_2429                                   XREF[1]:     1000:2420(j)
    //        1000:2429 cd 21           INT        0x21
    //                              LAB_1000_242b                                   XREF[1]:     1000:2427(j)
    //        1000:242b 0e              PUSH       CS
    //        1000:242c e8 ad 05        CALL       pass1_1000_29dc                                  uint16_t pass1_1000_29dc(undefin
    //        1000:242f 8e d8           MOV        DS,AX
    //        1000:2431 8b 0e 02 62     MOV        CX,word ptr [0x6202]
    //        1000:2435 e3 29           JCXZ       LAB_1000_2460
    //        1000:2437 8e 06 7e 5f     MOV        param_2,word ptr [0x5f7e]
    //        1000:243b 26 8b 36        MOV        SI,word ptr param_2:[0x2c]
    //                  2c 00
    //        1000:2440 a1 04 62        MOV        AX,[0x6204]
    //        1000:2443 8b 16 06 62     MOV        DX,word ptr [0x6206]
    //        1000:2447 33 db           XOR        param_1,param_1
    //        1000:2449 ff 1e 00 62     CALLF      [0x6200]
    //        1000:244d 73 03           JNC        LAB_1000_2452
    //        1000:244f e9 7a 01        JMP        exit_1000_25cc
    //                              LAB_1000_2452                                   XREF[1]:     1000:244d(j)
    //        1000:2452 a1 08 62        MOV        AX,[0x6208]
    //        1000:2455 8b 16 0a 62     MOV        DX,word ptr [0x620a]
    //        1000:2459 bb 03 00        MOV        param_1,0x3
    //        1000:245c ff 1e 00 62     CALLF      [0x6200]
    //                              LAB_1000_2460                                   XREF[1]:     1000:2435(j)
    //        1000:2460 8e 06 7e 5f     MOV        param_2,word ptr [0x5f7e]
    //        1000:2464 26 8b 0e        MOV        CX,word ptr param_2:[0x2c]
    //                  2c 00
    //        1000:2469 e3 3e           JCXZ       LAB_1000_24a9
    //        1000:246b 8e c1           MOV        param_2,CX
    //        1000:246d 33 ff           XOR        DI,DI
    //                              LAB_1000_246f                                   XREF[1]:     1000:2488(j)
    //        1000:246f 26 80 3d 00     CMP        byte ptr param_2:[DI],0x0
    //        1000:2473 74 34           JZ         LAB_1000_24a9
    //        1000:2475 b9 0d 00        MOV        CX,0xd
    //        1000:2478 be 5c 5f        MOV        SI,0x5f5c
    //        1000:247b f3 a6           CMPSB.REPE param_2:DI,SI
    //        1000:247d 74 0b           JZ         LAB_1000_248a
    //        1000:247f b9 ff 7f        MOV        CX,0x7fff
    //        1000:2482 33 c0           XOR        AX,AX
    //        1000:2484 f2 ae           SCASB.RE   param_2:DI
    //        1000:2486 75 21           JNZ        LAB_1000_24a9
    //        1000:2488 eb e5           JMP        LAB_1000_246f
    //                              LAB_1000_248a                                   XREF[1]:     1000:247d(j)
    //        1000:248a 06              PUSH       param_2
    //        1000:248b 1e              PUSH       DS
    //        1000:248c 07              POP        param_2
    //        1000:248d 1f              POP        DS
    //        1000:248e 8b f7           MOV        SI,DI
    //        1000:2490 bf 90 5f        MOV        DI,0x5f90
    //        1000:2493 b1 04           MOV        CL,0x4
    //                              LAB_1000_2495                                   XREF[1]:     1000:24a5(j)
    //        1000:2495 ac              LODSB      SI
    //        1000:2496 2c 41           SUB        AL,0x41
    //        1000:2498 72 0d           JC         LAB_1000_24a7
    //        1000:249a d2 e0           SHL        AL,CL
    //        1000:249c 92              XCHG       AX,DX
    //        1000:249d ac              LODSB      SI
    //        1000:249e 2c 41           SUB        AL,0x41
    //        1000:24a0 72 05           JC         LAB_1000_24a7
    //        1000:24a2 0a c2           OR         AL,DL
    //        1000:24a4 aa              STOSB      param_2:DI
    //        1000:24a5 eb ee           JMP        LAB_1000_2495
    //                              LAB_1000_24a7                                   XREF[2]:     1000:2498(j), 1000:24a0(j)
    //        1000:24a7 06              PUSH       param_2
    //        1000:24a8 1f              POP        DS
    //                              LAB_1000_24a9                                   XREF[3]:     1000:2469(j), 1000:2473(j),
    //                                                                                           1000:2486(j)
    //        1000:24a9 be 0c 62        MOV        SI,0x620c
    //        1000:24ac bf 0c 62        MOV        DI,0x620c
    //        1000:24af e8 e2 00        CALL       fn_ptr_op_1000_2594                              void fn_ptr_op_1000_2594(void)
    //        1000:24b2 be 0c 62        MOV        SI,0x620c
    //        1000:24b5 bf 0c 62        MOV        DI,0x620c
    //        1000:24b8 e8 d9 00        CALL       fn_ptr_op_1000_2594                              void fn_ptr_op_1000_2594(void)
    //        1000:24bb be ee 61        MOV        SI,0x61ee
    //        1000:24be bf fe 61        MOV        DI,0x61fe
    //        1000:24c1 e8 d0 00        CALL       fn_ptr_op_1000_2594                              void fn_ptr_op_1000_2594(void)
    //        1000:24c4 83 ed 02        SUB        BP,0x2
    //        1000:24c7 8b e5           MOV        SP,BP
    //        1000:24c9 1f              POP        DS
    //        1000:24ca 5d              POP        BP
    //        1000:24cb 4d              DEC        BP
    //        1000:24cc cb              RETF
}
