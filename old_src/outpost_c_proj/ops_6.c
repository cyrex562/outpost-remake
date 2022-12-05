//
// Created by cyrex on 6/8/2022.
//

#include "ops_6.h"
#include "func_ptrs.h"
#include "utils.h"
#include "globals.h"
#include "ops_4.h"
#include "string_defs.h"
#include "sys_api.h"
#include "win_ops.h"

void pass1_1008_aa28(u16 param_1,
                     Struct_1000_aa28 *param_2)

{
  code **pfn_1;
  u32 uVar2;
//  u16 DX_REG;
  i16 iVar3;
  u16 uVar4;
  u32 *puStack6;

//  uVar4 = (param_2 >> 0x10);
  // TODO: this is some kind of struct
//  iVar3 = (int)param_2;
  if (param_2->field_0x414 != 0x0) {
    uVar2 = (param_2->field_0x410);
    if (((int)uVar2 + 0x8) == 0x0) {
      param_2->field_0x414 = 0x0;
      return;
    }
      pfn_1 = (code **)((int)(u32)(u32)(param_2->field_0x410) + 0x10);
    (**pfn_1)();
    puStack6 = (u32 *)CONCAT22(DX_REG,param_1);
    if ((DX_REG | param_1) != 0x0) {
      win_1008_5c5c(param_1,DX_REG | param_1,u16_1050_02a0,(param_1 + 0x4));
      if (puStack6 != NULL) {
          pfn_1 = (code **)*puStack6;
        (**pfn_1)();
      }
      return;
    }
  }

}

void win_1008_5c5c(u16 param_1,u16 param_2,param_3: u32,u16 param_4)

{
  pass1_1010_84f8(u16_1050_14cc,param_4);
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),(WNDCLASS16 *)0x1050);
  return;
}


void pass1_1010_84f8(param_1: u32,param_2: i16)

{
  u32 uVar1;
  char *pcStack780;
  char local_308 [0x100];
  u8 local_208 [0x100];
  u8 local_108 [0x104];
  i16 iStack4;

  if ((param_2 * 0x10 + 0x10) == 0x3) {
    uVar1 = (u32)((int)param_1 + 0xe88);
    iStack4 = ((int)uVar1 + 0x70);
    str_1000_4d58(*(char **)(param_2 * 0x10 + 0x12),NULL,0x0,CONCAT22(0x1050,local_208),
                  (WNDCLASS16 *)CONCAT22(0x1050,local_308));
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_108),(char *)CONCAT22(0x1050,local_208));
    if (local_308[0] == '\0') {
      if (iStack4 == 0x0) {
        pcStack780 = s__mid_1050_14c0;
      }
      else {
        pcStack780 = s__wav_1050_14ba;
      }
    }
    else {
      pcStack780 = local_308;
    }
//    pcStack780 = (char *)CONCAT22(0x1050,(char *)pcStack780);
    pass1_1000_3cea(0, pcStack780);
    set_err_mode_1010_8b14(param_1,local_108);
    return;
  }
  return;
}

void str_1000_4d58(char *in_string_1,
                   char *in_string_2,
                   param_3: u32,
                   param_4: u32,
                   WNDCLASS16 *param_5)
{
    u16 uVar1;
    i16 iVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    char *pcStack18;
    u16 uStack12;
    u16 uStack10;
//    u16 uStack8;
    char* uStack6;

    uStack10 = 0x0;
    uStack12 = 0x0;
    uVar4 =  ((u32) in_string_1 >> 0x10);
    iVar2 =  in_string_1;
    if ((*in_string_1 == '\0') || (*(char *) (iVar2 + 0x1) != ':')) {
        if (in_string_2 != NULL) {
            *in_string_2 = '\0';
        }
    } else {
        if (in_string_2 != NULL) {
            *in_string_2 = *in_string_1;
            *(u8 *) ( in_string_2 + 0x1) = *(u8 *) (iVar2 + 0x1);
            *(u8 *) ( in_string_2 + 0x2) = 0x0;
        }
        in_string_1 = (char *) ((u32) in_string_1 & 0xffff0000 | (u32) (iVar2 + 0x2));
    }
    uStack6 = 0x0;
//    uStack8 = 0x0;
    pcStack18 = in_string_1;
    while (true) {
//        uVar5 =  ((u32) pcStack18 >> 0x10);
        uVar3 =  pcStack18;
        if (*pcStack18 == '\0') {
            break;
        }
        if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
            uStack6 = uVar3 + 0x1;
//            uStack6 = uVar5;
        } else if (*pcStack18 == '.') {
            uStack12 = uVar3;
//            uStack10 = uVar5;
        }
        pcStack18 = (char *) ((u32) pcStack18 & 0xffff0000 | (u32) (uVar3 + 0x1));
    }
    if ((uStack6) == 0x0) {
        if (param_3 != 0x0) {
            *(u8 *) param_3 = 0x0;
        }
    } else {
        if (param_3 != 0x0) {
            uVar1 = uStack6 -  in_string_1;
            if (0xff <  uVar1) {
                uVar1 = 0xff;
            }
            str_op_1000_3dbe((char *) (param_3 & 0xffff | (u32) param_3 << 0x10),
                             in_string_1,
                             uVar1);
            *(u8 *) ( param_3 + uVar1) = 0x0;
        }
        in_string_1 = uStack6;
    }
    if (((uStack10 | uStack12) != 0x0) && ( in_string_1 <= uStack12)) {
        if (param_4 != 0x0) {
//            uVar1 = uStack12 -  in_string_1;
            if (0xff <  uVar1) {
                uVar1 = 0xff;
            }
            str_op_1000_3dbe((char *) (param_4 & 0xffff | (u32) param_4 << 0x10),
                             (char *) ((u32) in_string_1 & 0xffff | (u32) in_string_1 << 0x10),
                             uVar1);
            *(u8 *) ( param_4 + uVar1) = 0x0;
        }
        if (param_5 == NULL) {
            return;
        }
        uVar1 = uVar3 - uStack12;
        if (0xff <  uVar1) {
            uVar1 = 0xff;
        }
        str_op_1000_3dbe((char *) ((u32) param_5 & 0xffff | (u32) param_5 << 0x10),
                         (char *) CONCAT22(uStack10,
                                           uStack12),
                         uVar1);
        *(u8 *) ( param_5 + uVar1) = 0x0;
        return;
    }
    if (param_4 != 0x0) {
//        uVar1 = uVar3 -  in_string_1;
        if (0xff <  uVar1) {
            uVar1 = 0xff;
        }
        str_op_1000_3dbe((char *) (param_4 & 0xffff | (u32) param_4 << 0x10),
                         (char *) ((u32) in_string_1 & 0xffff | (u32) in_string_1 << 0x10),
                         uVar1);
        *(u8 *) ( param_4 + uVar1) = 0x0;
    }
    if (param_5 != NULL) {
        *(u8 *) &param_5->style = 0x0;
    }
    return;
}


u16 *pass1_1000_3cea(StructB *param_1,
                     char *param_2)
{
    StructB *pstruct_1;
    char *string_2;
    u16 *pUVar3;
    i16 iVar4;
    u16 uVar5;
    u16 uVar6;
//    StructB *pstruct_7;
    char *string_8;
    char *pUVar9;
    StructB *pstruct_10;
    u16 uVar11;
    u16 uVar12;
    bool bVar13;

//    uVar11 =  (param_1 >> 0x10);
    bVar13 = true;
    iVar4 = -0x1;
    StructB* pstruct_7 = param_1;
    do {
        if (iVar4 == 0x0) {
            break;
        }
        iVar4 += -0x1;
        pstruct_1 = param_1;
//        pstruct_7 = ( pstruct_7 + 0x1);
        bVar13 = pstruct_1 == 0;
    } while (!bVar13);
//    pstruct_10 = (u16 *) ( pstruct_7 + -0x1);
//    uVar12 =  ((u32) param_2 >> 0x10);
    string_8 = param_2;
    uVar5 = 0xffff;
    do {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        string_2 = string_8;
        string_8 = string_8 + 0x1;
        bVar13 = *string_2 == '\0';
    } while (!bVar13);
    uVar5 = ~uVar5;
    if (!bVar13) {
        string_8 = string_8 + -uVar5;
        uVar5 += 0x1;
    }
    pUVar9 = (string_8 + -uVar5);
    if (uVar5 == 0x0) {
        pstruct_1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
//        *pstruct_10 = *pstruct_1;
        uVar5 = 0xfffe;
        pstruct_10 = (u16 *) ( pstruct_7 + 0x1);
    }
//    else if (( pUVar9 & 0x1) != 0x0) {
//        pstruct_1 = pUVar9;
//        pUVar9 = (u16 *) ( pUVar9 + 0x1);
//        *(u8 *) pstruct_10 = *(u8 *) pstruct_1;
//        uVar5 -= 0x1;
//        pstruct_10 = pstruct_7;
//    }
    for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        pUVar3 = pstruct_10;
        pstruct_10 = pstruct_10 + 0x1;
        pstruct_1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
//        *pUVar3 = *pstruct_1;
    }
    for (uVar5 =  ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
        pUVar3 = pstruct_10;
        pstruct_10 = (u16 *) ( pstruct_10 + 0x1);
        pstruct_1 = pUVar9;
        pUVar9 = (u16 *) ( pUVar9 + 0x1);
        pUVar3 = *(u8 *) pstruct_1;
    }
    return (u16 *) param_1;
}


u32 set_err_mode_1010_8b14(param_1: u32,u32 param_2)

{
  u16 mode;
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  let mut lVar4: i32;
  u8 local_a [0x8];

  uVar3 = (param_1 >> 0x10);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0xe84));
  mode = SetErrorMode16(SEM_FAILCRITICALERRORS);
  do {
    lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar4 == 0x0) {
      SetErrorMode16(mode);
      return param_2;
    }
    uVar1 = (int)param_1 + 0xa82;
    unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (u32)uVar1),*(char **)((int)lVar4 + 0x4));
    pass1_1000_3cea(param_1 & 0xffff0000 | (u32)uVar1,(char *)param_2);
    uVar2 = dos3_call_1000_51aa(uVar1,uVar3,0x1);
  } while (uVar2 != 0x0);
  SetErrorMode16(mode);
  return param_1 & 0xffff0000 | (u32)uVar1;
}


u16 *pass1_1000_4906(StructD *param_1,
                     WNDCLASS16 *in_wnd_class,
                     u16 param_3)
{
    u16 *puVar1;
    u8 uVar2;
    u16 uVar3;
    u16 uVar4;
    struct astruct_20 *struct_1;
    u16 uVar5;
    u16 *puVar6;
    struct astruct_20 *struct_1_hi;

    if (param_3 != 0x0) {
        struct_1_hi = (astruct_20 *) ((u32) param_1 >> 0x10);
        struct_1 = (astruct_20 *) - (u16 *) param_1;
        uVar5 = param_3;
        if (struct_1 != NULL) {
            uVar5 = ( struct_1 - param_3 & - (struct_1 < param_3)) + param_3;
            struct_1 = (astruct_20 *) (param_3 - uVar5);
        }
        uVar3 =  in_wnd_class & 0xff |  in_wnd_class << 0x8;
        puVar6 = (u16 *) param_1;
        for (uVar4 = uVar5 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = puVar6 + 0x1;
            *puVar1 = uVar3;
        }
        for (uVar5 =  ((uVar5 & 0x1) != 0x0);
             uVar2 = (u8) ( in_wnd_class & 0xff), uVar5 != 0x0;
             uVar5 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = (u16 *) ( puVar6 + 0x1);
            *(u8 *) puVar1 = uVar2;
        }
        if (struct_1 != NULL) {
            for (uVar5 =  struct_1 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = puVar6 + 0x1;
                *puVar1 = uVar3;
            }
            for (uVar5 =  (( struct_1 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = (u16 *) ( puVar6 + 0x1);
                *(u8 *) puVar1 = uVar2;
            }
        }
    }
    return (u16 *) param_1;
}


i16 pass1_1000_475e(param_1: u32,
                    u32 param_2)
{
    char *pcVar1;
    char cVar2;
    char cVar3;
    u8 bVar4;
    struct astruct_235 *bVar3;
    i16 bVar5;
    char *pcVar5;
    char *pcVar6;

    pcVar6 = (char *) param_2;
    pcVar5 = (char *) param_1;
    bVar5 = 0xff;
    do {
        do {
            cVar3 = (char) bVar5;
            if (cVar3 == '\0') {
                goto LAB_1000_479d;
            }
            pcVar1 = pcVar6;
            pcVar6 = pcVar6 + 0x1;
            cVar3 = *pcVar1;
            cVar2 = *pcVar5;
            bVar5 = CONCAT11(cVar2,
                             cVar3);
            pcVar5 = pcVar5 + 0x1;
        } while (cVar2 == cVar3);
        bVar4 = cVar3 + 0xbfU + (-((u8) (cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
        bVar3._0_1_ = cVar2 + 0xbf;
        bVar5._0_1_ = (u8) bVar3 + (-((u8) bVar3 < 0x1a) & 0x20U) + 0x41;
        bVar5 = CONCAT11(bVar4,
                         (u8) bVar5);
    } while ((u8) bVar5 == bVar4);
    cVar3 = ((u8) bVar5 < bVar4) * -0x2 + '\x01';
    LAB_1000_479d:
    return  cVar3;
}
