use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000::block_1000_2000::pass1_1000_29b5;
use crate::block_1000::block_1000_3000;
use crate::globals::DAT_1050_1050;
use crate::utils::CARRY2;


pub enum DosInterruptVector {
    TerminateProgram = 0x20,
    MainDosAPi = 0x1,
    ProgramTerminateAddress = 0x22,
    ControlCHandlerAddress = 0x23,
    CriticalErrorHandlerAddress = 0x24,
    AbsoluteDiskRead = 0x25,
    AbsoluteDiskWrite = 0x26,
    TerminateAndStayResident = 0x27,
    IdleCallout = 0x28,
    FastConsoleOutput = 0x29,
    NetworkingAndCriticalSection = 0x2a,
    Unused2b = 0x2b,
    Unused2c = 0x2c,
    Unused2d = 0x2d,
    ReloadTransient = 0x2e,
    Multiplex = 0x2f
}

pub enum Int21Service {
    ProgramTerminate = 0x0,
    CharacterInput = 0x1,
    CharacterOutput = 0x2,
    AuxiliaryInput = 0x3,
    AuxiliaryOutput = 0x4,
    PrinterOutput = 0x5,
    DirectConsoleIo = 0x6,
    DirectConsoleInputAndWithoutEcho = 0x7,
    ConsoleInputWithoutEcho = 0x8,
    DisplayString = 0x9,
    BufferedKeyboardInput = 0xa,
    GetInputStatus = 0xb,
    FlushInputBufferAndInput = 0xc,
    DiskReset = 0xd,
    SetDefaultDrive = 0xe,
    OpenFile = 0xf,
    FindFirstFile = 0x11,
    FindNextFile = 0x12,
    DeleteFile = 0x13,
    SequentialRead = 0x14,
    SequentialWrite = 0x15,
    CreateOrTruncateFile = 0x16,
    RenameFile = 0x17,
    Reserved = 0x18,
    GetDefaultDrive = 0x19,
    SetDiskTransferAddress = 0x1a,
    GetAllocationInfoForDefaultDrive = 0x1b,
    GetAllocationInfoForSepcifiedDrive = 0x1c,
    Reserved1d = 0x1d,
    Reserved1e = 0x1e,
    GetDiskParamBlockForDefltDrive = 0x1f,
    Reserved20 = 0x20,
    RandomRead = 0x21,
    RandomWrite = 0x22,
    GetFileSizeInRecords = 0x23,
    SetRandomRecordNumber = 0x24,
    SetInterruptVector = 0x25,
    CreatePsp = 0x26,
    RandomBlockRead = 0x27,
    RandomBlockWrite = 0x28,
    ParseFilename = 0x29,
    GetDate = 0x2a,
    SetDate = 0x2b,
    GetTime = 0x2c,
    SetTime = 0x2d,
    SetVerifyFlag = 0x2e,
    GetDiskTransferAddress = 0x2f,
    GetDosVersion = 0x30,
    TerminateAndStayResident = 0x31,
    GetDiskParameterBlockForSpecifiedDrive = 0x32,
    GetOrSetCtrlBreak = 0x33,
    GetInDosFlagPtr = 0x34,
    GetInterruptVector = 0x35,
    GetFreeDiskSpace = 0x36,
    GetOrSetSwitchCharacter = 0x37,
    GetOrSetCountryInfo = 0x38,
    CreateSubdirectory = 0x39,
    RemoveSubdirectory = 0x3a,
    ChangeCurrentDirectory = 0x3b,
    CreateOrTruncateFile3c = 0x3c,
    OpenFile3d = 0x3d,
    CloseFile = 0x3e,
    ReadFileOrDevice = 0x3f,
    WriteFileOrDevice = 0x40,
    DeleteFile41 = 0x41,
    MoveFilePointer = 0x42,
    GetOrSetFileAttributes = 0x43,
    IoControlForDevices = 0x44,
    DuplicateHandle = 0x45,
    RedirectHandle = 0x46,
    GetCurrentDirectory47 = 0x47,
    AllocateMemory = 0x48,
    ReleaseMemory = 0x49,
    ReallocateMemory = 0x4a,
    ExecuteProgram = 0x4b,
    TerminateWithReturnCode = 0x4c,
    GetProgramReturnCode = 0x4d,
    FindFirstFile4e = 0x4e,
    FindNextFile4f = 0x4f,
    SetCurrentPsp = 0x50,
    GetCurrentPsp = 0x51,
    GetDosInternalPointers = 0x52,
    CreateDiskParameterBlock = 0x53,
    GetVerifyFlag = 0x54,
    CreateProgramPsp = 0x55,
    RenameFile56 = 0x56,
    GetOrSetFileDateAndTime = 0x57,
    GetOrSetAllocStrategy = 0x58,
    GetExtendedErrorInfo = 0x59,
    CreateUniqueFile = 0x5a,
    CreateNewFile5b = 0x5b,
    LockOrUnlockFile = 0x5c,
    FileSharingFunctions = 0x5d,
    NetworkFunctions = 0x5e,
    NetworkRedirectionFunctions = 0x5f,
    QualifyFilename = 0x60,
    Reserved61 = 0x61,
    GetCurrentPsp62 = 0x62,
    GetDbcsLeadByteTablePtr = 0x63,
    SetWaitForExternalEventFlags = 0x64,
    GetExtendedCountryInfo = 0x65,
    GetOrSetCodePage = 0x66,
    SetHandleCount = 0x67,
    CommitFile = 0x68,
    GetOrSetMediaId = 0x69,
    CommitFile6a = 0x6a,
    Rserved6b = 0x6b,
    ExtendedOpenCreateFile = 0x6c
}

// typedef void(*code3)(void*);

// typedef u8(*code4)();

// typedef i16(*code5)();

// typedef bool(*code6);

// typedef u16(*code7);


type code = fn(u16);
// typedef u32(*code8)();
type code8 = fn() -> u32;

pub enum InterruptResult {
    CODE = code,
    CODE8 = code8,

}

pub fn swi(ctx: &mut AppContext, int_code: u16) -> InterruptResult {
    todo!()
}

pub unsafe fn dos3_call_1000_4f20(ctx: &mut AppContext,) -> u16 {
    let mut func_ptr: *mut code;
    let mut var2 = 0u16;
    let mut var3 = false;
    ctx.AH_REG = 0x39; // create subdirectory
    let mut result = swi(ctx, 0x21);
    // uVar2 = (*pcVar1)(&DAT_1050_1050, unaff_BP + 1);
    if var3 {
        pass1_1000_29b5(var2);
        return 0xffff;
    }
    return 0x0;
}

pub unsafe fn dos3call_1000_4f54(ctx: &mut AppContext, mut param_1: u32) -> u16 {
    let mut c_var1: c_char;
    let mut u_var5: *mut c_char = null_mut();
    let mut b_var3 = false;
    // Change Current Directory
    ctx.AH_REG = 0x3b;
    let mut result = swi(ctx, 0x21);
    // u_var5 = (*pc_var2)(&DAT_1050_1050, unaff_bp + 1);
    let mut u_var3 = u_var5;
    b_var3 = gu_var5 < 0x10;
    if b_var3 && u_var5 == 0x10 {
        loop {
            c_var1 = *u_var5;
            u_var5 = u_var5 + 1;
            if (c_var1 == '\0') {
                // TODO: goto LAB_1000_4f90;
            }
            if !((c_var1 != '?') && (c_var1 != '*')) {
                break;
            }
        }
        u_var3 = 0x3; //
                      //        LAB_1000_4f90:
        b_var3 = true;
    }
    if (!b_var3) {
        return 0x0;
    }
    pass1_1000_29b5(u_var3);
    return 0xffff;
}

pub unsafe fn dos3_call_1000_4f94(ctx: &mut AppContext) -> i16 {
    // Get Default Drive
    ctx.AH_REG = 0x19;
    let result = swi(0x21);
    //    bVar2 = (*pcVar1)(unaff_BP + 1);
    // let b_var2: i16 = fn_ptr_1(unaff_BP + 1);
    // return b_var2 + 1;
    todo!()
}

pub unsafe fn dos3_call_1000_4fbe(ctx: &mut AppContext, param_1: u8) -> u16 {
    //    unaff_BP: i16;
    // set default drive
    ctx.AH_REG = 0xe;
    let result1 = swi(ctx,0x21);
    // (fn_ptr_var1)(unaff_BP + 1);
    // get default drive
    ctx.AH_REG = 0x19;
    let result2 = swi(ctx, 0x21);
    // let c_var2 = fn_ptr_var2();
    // let u_var3 = 0xffff;
    // if (c_var2 + '\x01' == param_1) {
    //     u_var3 = 0;
    // }
    // return u_var3;
}

// WARNING: Removing unreachable block (ram,0x10002589)
pub unsafe fn dos3_op_1000_256b(ctx: &mut AppContext) {
    // let mut pcVar1: *mut code;

    if (PTR_LOOP_1050_6202.is_null() == false) {
        (PTR_LOOP_1050_6200)();
    }
    // SetInterruptVector
    ctx.AH_REG = 0x25;
    let mut result = swi(ctx, 0x21);
    // (*pcVar1)();
    todo!()
    return;
}

// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)
pub unsafe fn mixed_dos3_call_1000_3636(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut pbVar1: *mut u8;
    let mut pcVar2: *mut code;
    let mut u_var3: u16;
    let mut unaff_bp: i16;
    let mut i_var4: i16;
    let mut b_var5: bool;
    let mut u_var6: u32;

    i_var4 = unaff_bp + 1;
    if (((param_1 < u16_1050_5f8a) || (u16_1050_61ec == 0)) || (0x2 < param_1)) {
        if ((u16_1050_6064 == 0) || ((param_3 & 0x8000) == 0)) {
            // TODO: goto LAB_1000_36e3;
        }
        if (param_4 == 0) {
            // TODO: goto LAB_1000_369b;
        }
        b_var5 = false;
        // Move File Pointer
        ctx.AH_REG = 0x42;
        let mut result = swi(ctx, 0x21);
        // u_var6 = (*pcVar2)();
        u_var3 = u_var6;
        if (b_var5) {
            // TODO: goto LAB_1000_299d;
        }
        if ((param_4 & 0x2) == 0) {
            if (-0x1 < ((u_var6 >> 0x10) + param_3 + CARRY2(u_var3, param_2))) {
                //
                //                LAB_1000_36e3:
                bVar5 = false;
                // mov file pointer
                ctx.AH_REG = 0x42;
                result = swi(ctx, 0x21);
                // uVar3 = (*pcVar2)(iVar4);
                if (!bVar5) {
                    pbVar1 = (param_1 + 0x5f90);
                    bVar5 = false;
                    *pbVar1 = *pbVar1 & 0xfd;
                }
                // TODO: goto LAB_1000_299d;
            }
        } else {
            // move file pointer
            ctx.AH_REG = 0x42;
            let mut result = swi(ctx, 0x21);
            // u_var6 = (*pcVar2)();
            if (-0x1 < ((u_var6 >> 0x10) + param_3 + CARRY2(u_var6, param_2))) {
                // TODO: goto LAB_1000_36e3;
            }
            // move file pointer
            ctx.AH_REG = 0x42;
            result = swi(ctx, 0x21);
            // (*pcVar2)();
        } //
          //        LAB_1000_369b:
        u_var3 = s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    b_var5 = true; //
                   //    LAB_1000_299d:
    if (b_var5) {
        pass1_1000_29b5(u_var3);
    }
    todo!();
    return;
}

pub unsafe fn mixed_dos3_call_1000_370a(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: u8,
    mut param_6: u16,
) -> u16 {
    let mut fn_ptr_1: *mut code;
    let mut uVar3: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut bVar2: u8;
    let mut uVar7: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut unaff_BP: i16;
    let mut uVar6: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    let mut uStack14: u16;
    let mut bVar9: u8;
    let mut in_stack_0000fffa: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut cVar12: u8;

    uVar6 = unaff_BP + 1;
    param_5 = param_6;
    uVar3 = param_1 & 0xff00;
    param_1 = uVar3 | param_5;
    uVar9 = 0;
    if (((param_4 & 0x8000) == 0) && ((param_4 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0))) {
        uVar9 = 0x80;
    }
    bVar10 = false;
    // Open File
    ctx.AH_REG = 0x3d;
    let mut result = swi(ctx, 0x21);
    uVar7 = param_4;
    uVar2 = (*fn_ptr_1)();
    if (bVar10) {
        if (uVar2 == 0x2) && ((uVar7 & 0x100) != 0) {
            uVar10 = uVar9 & 0xff;
            // s_____s__lu_1050_38cd + 0x3
            param_1 = 0x38d0;
            block_1000_3000::pass1_1000_39e1();
            uVar7 = 0;
            _param_5 = param_6; //
                                //            LAB_1000_38e3:
            bVar10 = false;
            // Create or Truncate File
            ctx.AH_REG = 0x3c;
            fn_ptr_1 = swi(ctx, 0x21);
            uVar2 = (*fn_ptr_1)();
            if (bVar10) {
                // TODO: goto LAB_1000_299d;
            }
            if ((param_1 != '\0') || (uVar5 = uVar2, uVar8 = uStack14, (param_4 & 0x2) == 0)) {
                // close file
                ctx.AH_REG = 0x3e;
                fn_ptr_1 = swi(ctx, 0x21);
                (*fn_ptr_1)();
                bVar10 = false;
                // open file
                ctx.AH_REG = 0x3d;
                fn_ptr_1 = swi(0x21);
                uVar2 = (*fn_ptr_1)();
                if (bVar10) {
                    // TODO: goto LAB_1000_299d;
                }
                uVar5 = uVar2;
                uVar8 = param_1;
                if (((uVar10 & 0x100) == 0) && (uVar8 = param_1, (_param_5 & 1) != 0)) {
                    uVar7 = (uVar7 | 1);
                    bVar10 = false;
                    // get or set file attributes
                    ctx.AH_REG = 0x43;
                    fn_ptr_1 = swi(0x21);
                    uVar4 = uVar2;
                    uVar2 = (*fn_ptr_1)();
                    uVar5 = uVar4;
                    uVar8 = uVar6;
                    if (bVar10) {
                        // TODO: goto LAB_1000_299d;
                    }
                }
            } //
              //            LAB_1000_3973:
            bVar9 = uVar10;
            if ((uVar10 & 0x40) == 0) {
                // get or set file attributes;
                ctx.AH_REG = 0x43;
                fn_ptr_1 = swi(0x21);
                (*fn_ptr_1)();
                bVar2 = 0;
                if ((uVar7 & 1) != 0) {
                    bVar2 = 0x10;
                }
                if ((param_4 & 0x8) != 0) {
                    bVar2 |= 0x20;
                }
            } else {
                bVar2 = 0;
            }
            if (uVar5 < &u16_1050_5f8a) {
                *(uVar5 + 0x5f90) = bVar2 | bVar9 | 0x1;
                return uVar5;
            }
            // close file
            ctx.AH_REG = 0x3e;
            fn_ptr_1 = swi(ctx, 0x21);
            (*fn_ptr_1)();
            uVar2 = 0x1800;
        }
    } else {
        if ((uVar7 & 0x500) != 0x500) {
            uVar10 = CONCAT11(0x1, uVar9);
            // Io Device Control
            ctx.AH_REG = 0x44;
            fn_ptr_1 = swi(ctx, 0x21);
            (*fn_ptr_1)();
            if ((extraout_DX & 0x80) != 0) {
                uVar10 |= 0x40;
            }
            uVar5 = uVar2;
            uVar8 = param_1;
            if ((uVar10 & 0x40) == 0) {
                if ((param_4 & 0x200) == 0) {
                    if (((uVar10 & 0x80) != 0) && ((param_4 & 0x2) != 0)) {
                        // move file pointer
                            ctx.AH_REG = 0x42;
                        fn_ptr_1 =  swi(ctx, 0x21);
                        (fn_ptr_1)();
                        // read file or device
                            ctx.AH_REG = 0x3f;
                        fn_ptr_2 =  swi(ctx, 0x21);
                        //iVar3 = (fn_ptr_2)();
                        if ((iVar3 != 0) && (param_1 = (uVar3 >> 0x8), param_1 == '\x1a')) {

                            // move file pointer
                            ctx.AH_REG = 0x42;
                            fn_ptr_1 = swi(0x21);
                            (*fn_ptr_1)(uVar6);
                            // write file or device
                            ctx.AH_REG = 0x40;

                            fn_ptr_1 = swi(0x21);
                            (*fn_ptr_1)();
                        }
                        uVar7 = 0;
                        // move file pointer
                        ctx.AH_REG = 0x42;
                        fn_ptr_1 = swi(0x21);
                        (*fn_ptr_1)();
                        uVar5 = uVar2;
                        uVar8 = uStack14;
                    }
                } else {
                    if ((param_4 & 0x3) == 0) {

                        // close file
                        ctx.AH_REG = 0x3e;
                        fn_ptr_1 = swi(0x21);
                        (*fn_ptr_1)();
                        // get or set file attributes
                        ctx.AH_REG = 0x43;
                        fn_ptr_1 = swi(0x21);
                        (*fn_ptr_1)();
                        // TODO: goto LAB_1000_38e3;
                    }
                    uVar7 = 0;
                    // write file or device
                    ctx.AH_REG = 0x40;
                    fn_ptr_1 = swi(0x21);
                    (*fn_ptr_1)();
                    uVar5 = uVar2;
                    uVar8 = param_1;
                }
            }
            // TODO: goto LAB_1000_3973;
        }
        // close file
        ctx.AH_REG = 0x3e;
        fn_ptr_1 = swi(0x21);
        (*fn_ptr_1)();
        uVar2 = 0x1100;
    }
    bVar10 = true; //
                   //    LAB_1000_299d:
    if bVar10 {
        pass1_1000_29b5(uVar2);
        uVar2 = 0xffff;
    }
    return uVar2;
}
