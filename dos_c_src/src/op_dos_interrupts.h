//
// Created by cyrex on 2/18/2023.
//

#ifndef OUTPOST_1_SRC_OP_DOS_INTERRUPTS_H_
#define OUTPOST_1_SRC_OP_DOS_INTERRUPTS_H_

static int DOS_INT_21 = 0x21;

// INT21,21 Random Read using FCB
// AH = 21h
//	DS:DX = pointer to an opened FCB
//
//
//	on return:
//	AL = 00 if read successful
//	   = 01 if EOF (no data read)
//	   = 02 if DTA is too small
//	   = 03 if EOF (partial record read)
//
//
//	- reads random records from a file opened with an FCB
//	  to the DTA
//	- FCB must be setup with drive id, filename, extension,
//	  record position and record length before call
//	- current record position field in FCB is not updated
static int DOS_INT_21_21H = 0x21;

// Get Date
//AH = 2A
//
//
//  on return:
//    AL = day of the week (0=Sunday)
//      CX = year (1980-2099)
//      DH = month (1-12)
//      DL = day (1-31)
//
//
//    - retrieves system date based on the DOS maintained clock
//    - updates BIOS Data Area current date and date rollover flag
//        at location 40:70
typedef void(*DosInt21GetDate)(u16);

// Get Time
//
//AH = 2C
//
//
//  on return:
//    CH = hour (0-23)
//      CL = minutes (0-59)
//      DH = seconds (0-59)
//      DL = hundredths (0-99)
//
//
//    - retrieves DOS maintained clock time
typedef void(*DosInt21GetTime)();


//  Set Interrupt Vector
// 	AH = 25h
//	AL = interrupt number
//	DS:DX = pointer to interrupt handler
//
//
//	returns nothing
//
//
//	- provides a safe method for changing interrupt vectors
//
//
//	- see	INT 21,35
static int DOS_INT_21_25H = 0x25;

typedef void (*DosInt21SetInterruptVector)();

//
//AH = 30h
//
//
//on return:
//AL = major version number (2-5)
//AH = minor version number (in hundredths decimal)
//BH = FF  indicates MS-DOS, only if OEM vendor chooses to identify
//   = 00  indicates PC-DOS
//BL:CX = 24 bit OEM serial number if BH is FF
//
//
//- for an example DOS version 2.1 returns AL=2 and AH=10
//- DOS versions prior to DOS 2.0 return zero in AH and AL
//- DOS version 4.0 and 4.1 usually return the same value of 4.00
//- the OEM serial number is a rarity, though some older OEM DOS
//  versions implemented this feature
//- the OS/2 compatibility box returns 10.10 for OS/2 1.1, 10.20
//  for OS/2 1.2, etc...
//- when testing for version, a specific test can often cause your
//  code to not work in following versions of DOS.  It is often better
//  to test for a version number greater or equal to the minimum rather
//  than a specific version number where possible
//- see DOS Versions
static int DOS_INT_21_30H = 0x30;

// 	AH = 35h
//	AL = interrupt vector number
//
//
//	on return:
//	ES:BX = pointer to interrupt handler
//
//
//	- standard method for retrieving interrupt vectors
//
//
//	- see	INT 21,25
static int DOS_INT_21_35H = 0x35;

// AH = 45h
//	BX = file handle
//
//
//	on return:
//	AX = new file handle if CF not set
//	   = error code if CF set  (see DOS ERROR CODES)
//
//
//	- gets another file handle for the same file
//	- both file handles move in unison
//	- often used to flush file data and update a file directory
//	  entry without closing the initial file
static int DOS_INT_21_45H = 0x45;

// AH = 4C
//	AL = return code (for batch files)
//
//
//	returns nothing
//
//
//	- approved method of program termination
//	- restores the terminate, Ctrl-Break, and critical error exit
//	  addresses, flushes all buffers, frees memory and returns to
//	  DOS via the termination handler address
//	- does not close FCBs
//	- this function is not supported in versions of DOS before 2.x,
//	  so use INT 21,0  or	 INT 20  to exit.
//
//
//	- see also  INT 27   INT 21,31
static int DOS_INT_21_4C = 0x4c;

typedef void(*DosInt21TerminateProcWithRetCode)(u16);



typedef void (*DosInt21DuplicateFileHandle)();

typedef void (*DosInt21GetInterruptVector)(void*);



#endif // OUTPOST_1_SRC_OP_DOS_INTERRUPTS_H_
