//
// Created by cyrex on 2022-05-22.
//

#include "types.h"
#include "structs.h"
#include "globals.h"
#include "sys_api.h"
#include "block_1000.h"
#include "utils.h"
#include <stdbool.h>
#include <stdio.h>

void pass1_1000_0000(u16 *param_1, u16 *param_2, u16 param_3)
{
    u16 *puVar1;
    u16 *puVar2;
    u16 uVar3;

    for (uVar3 = param_3 >> 0x1; uVar3 != 0x0; uVar3 -= 0x1) {
        puVar2 = param_2;
        param_2 = param_2 + 0x1;
        puVar1 = param_1;
        param_1 = param_1 + 0x1;
        *puVar2 = *puVar1;
    }
}



u32 mem_1000_0016(u32 param_1)

{
  u16 unaff_CS;
  u32 uVar1;

  if (((int)param_1 + 0x14) != -0x4153) {
    pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
    return 0xffffffff;
  }
  uVar1 = mem_op_1000_0052(0x0);
  return uVar1;
}



u32 mem_op_1000_0052(u16 param_1)

{
  u16 *puVar1;
  u16 uVar2;
  i16 iVar3;
  u32 uVar4;
  u32 uVar5;
  i16 iStack14;
  i16 iStack12;
  i16 iStack10;
  u16 uStack8;

  uVar2 = (param_1 + 0x1e);
  iVar3 = (param_1 + 0x20);
  uStack8 = 0x0;
  do {
    iStack10 = (uStack8 * 0x2 + param_1);
    if ((iStack10 != 0x0) && (uStack8 != 0x3)) {
      iStack14 = 0x0;
      do {
        iStack12 = (iStack10 + 0x4);
        uVar4 = *(u32 *)(iStack10 + 0x8);
        if (((int)uVar4 + 0xa) == 0x0) {
          uVar5 = mem_op_1000_0510(0x1,0x0);
          if ((int)uVar5 == 0x0) goto LAB_1000_00f9;
          if (iStack12 == iStack10) {
            iStack12 = 0x0;
          }
        }
        else if (iStack14 == 0x0) {
          iStack14 = iStack10;
        }
        iStack10 = iStack12;
      } while (iStack12 != iStack14);
    }
    uStack8 += 0x1;
  } while (uStack8 < 0x5);
  if ((param_1 + 0x32) != 0x0) {
    (**(code **)(param_1 + 0x32))();
  }
LAB_1000_00f9:
  puVar1 = (u16 *)(param_1 + 0x1e);
  return CONCAT22((iVar3 - (param_1 + 0x20)) - (u16)(uVar2 < *puVar1),uVar2 - *puVar1);
}



u16 pass1_1000_010c(i16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 uVar1;
  u16 UVar2;
  u16 uVar2;
  u16 unaff_CS;
  bool bVar3;
  u16 UVar4;
  u16 uStack8;
  u16 uStack6;
  u16 uStack4;

  uStack6 = 0x0;
  uStack8 = 0x0;
  if ((param_4 + 0x14) != -0x4153) {
    param_5 = 0x0;
    param_4 = 0x0;
    UVar4 = 0xa;
code_r0x10000128:
    pass1_1000_1e61(unaff_CS,UVar4,param_4,param_5);
    return 0xffff;
  }
  DAT_1050_5f30 = 0x1;
  if (param_1 == 0x1) {
    uStack4 = 0x1;
    if ((param_4 + 0x18) == 0x0) {
      UVar4 = 0x4;
      goto code_r0x10000128;
    }
  }
  else if (param_1 == 0x2) {
    uStack4 = 0x2;
  }
  else {
    if (param_1 != 0x4) {
      DAT_1050_5f30 = 0x1;
      return 0xffff;
    }
    uStack4 = 0x0;
  }
  // s_version__d__d_1050_0012 + 0x8
  while ((uStack6 <= param_3 &&
         (((uStack6 < param_3 || (uStack8 < param_2)) &&
          (uVar1 = uStack4,
          UVar2 = mem_op_1000_03c6(((int)0x001a),0x0,uStack4,0x0,0x0,'\0'),
          (UVar2 | uVar1) != 0x0))))) {
    uVar1 = ((int)0x00a1a);
    bVar3 = CARRY2(uStack8,uVar1);
    uStack8 += uVar1;
    uStack6 += bVar3;
  }
  return uStack6;
}

bool mem_op_1000_01b0(astruct_7 *param_1)
{
    u16 *puVar1;
    i16 *piVar2;
    BOOL16 BVar3;
    u16 UVar4;
    u16 uVar5;
    u16 unaff_CS;
    DWORD DVar6;
    DWORD DVar7;
    u32 uVar8;
    u16 uVar9;
    u16 *pu8_var10;
    u16 uStack14;
    u16 uStack12;
    i16 iStack10;
    u16 uStack6;
    i16 iStack4;

    uStack14 = 0x0;
    if (((param_1->field_0x40) | (param_1->field_0x3e)) == 0x0) {
        uVar5 = param_1->field_0x36;
        DVar6 = mem_op_1000_1532(param_1,
                                 0x1050);
        DVar7 = DVar6;
    } else {
        DVar6 = mem_op_1000_1532(param_1,
                                 0x1050);
        uVar5 = (u16) DVar6;
        if (((int) (DVar6 >> 0x10) != 0x0) || (0xffef < uVar5)) {
            pass1_1000_1e61(unaff_CS,
                            0x8,
                            param_1,
                            0x1050);
            return false;
        }
        if (0x1fff < uVar5) {
            uVar5 = 0x2000;
        }
        while (true) {
            uVar9 = uVar5;
            DVar7 = DVar6 + 0x18;
            if (((int) (DVar7 >> 0x10) != 0x0) || (0xfff0 < (u16) DVar7)) {
                DVar7 = 0xfff0;
            }
            BVar3 = mem_op_1000_14f2((param_1->field_0x16) | 0x1000,
                                     DVar7,
                                     (u16) 0x1050,
                                     0);
            iStack4 = (int) (DVar6 >> 0x10);
            uStack6 = (u16) DVar6;
            if (BVar3 != 0x0) {
                break;
            }
            uVar5 = uVar9 >> 0x1;
            if (uVar5 < 0xc) {
                UVar4 = pass1_1000_1e61(unaff_CS,
                                        0x2,
                                        param_1,
                                        (u16) 0x1050);
                if (UVar4 == 0x0) {
                    return (bool) ('\x01' - ((param_1->field_0xa) == 0x0));
                }
                DVar6 = mem_op_1000_1532(param_1,
                                         (u16) 0x1050);
                uVar5 = uVar9 & 0xfffe;
            }
        }
        uVar8 = pass1_1000_5390(uStack6 - 0x42,
                                iStack4 - (u16) (uStack6 < 0x42),
                                0xc,
                                0x0);
        uVar5 = (int) uVar8 * 0xc + param_1 + 0x42;
    }
    puVar1 = (u16 *) (param_1 + 0x1e);
    uVar9 = *puVar1;
    *puVar1 = *puVar1 - (u16) DVar6;
    piVar2 = (i16 *) (param_1 + 0x20);
    *piVar2 = (*piVar2 - (int) (DVar6 >> 0x10)) - (u16) (uVar9 < (u16) DVar6);
    if (uVar5 != 0x0) {
        pu8_var10 = 0x0;
        uVar9 = 0xc;
        DVar7 = mem_op_1000_1532(param_1,
                                 0x1050);
        uVar8 = pass1_1000_5390((u16) DVar7 - 0x42,
                                (int) (DVar7 >> 0x10) - (u16) ((u16) DVar7 < 0x42),
                                uVar9,
                                pu8_var10);
        uStack14 = (int) uVar8 * 0xc + param_1 + 0x36;
    }
    iStack10 = (int) (DVar7 >> 0x10);
    uStack12 = (u16) DVar7;
    puVar1 = (u16 *) (param_1 + 0x1e);
    uVar9 = *puVar1;
    *puVar1 = *puVar1 + uStack12;
    piVar2 = (i16 *) (param_1 + 0x20);
    *piVar2 = *piVar2 + iStack10 + (u16) CARRY2(uVar9,
                                                uStack12);
    uVar9 = (param_1->field_0xa);
    do {
        pu8_var10 = uVar5;
        (pu8_var10 + 0x4) = uVar9;
        uVar9 = pu8_var10;
        uVar5 = pu8_var10 + 0xc;
    } while (pu8_var10 < uStack14);
    (param_1 + 0xa) = pu8_var10;
    return true;
}

i16 mem_op_1000_0308(i16 param_1,i16 param_2)

{
  i16 iVar1;
  i16 iVar2;
  bool bVar3;
  u8 extraout_AH;
  i16 *piVar4;

  if ((param_2 + 0xa) == 0x0) {
    bVar3 = mem_op_1000_01b0(param_2);
    if (CONCAT11(extraout_AH,bVar3) == 0x0) {
      return 0x0;
    }
  }
  iVar1 = (param_2 + 0xa);
  (param_2 + 0xa) = (iVar1 + 0x4);
  piVar4 = (i16 *)(param_1 * 0x2 + param_2);
  if (*piVar4 == 0x0) {
    (iVar1 + 0x6) = iVar1;
    (iVar1 + 0x4) = iVar1;
  }
  else {
    iVar2 = *piVar4;
    (iVar1 + 0x6) = iVar2;
    (iVar1 + 0x4) = (iVar2 + 0x4);
    ((iVar2 + 0x4) + 0x6) = iVar1;
    (iVar2 + 0x4) = iVar1;
  }
  *piVar4 = iVar1;
  return iVar1;
}



void pass1_1000_0368(u16 param_1,u16 param_2,u16 param_3)

{
  u16 *puVar1;

  if ((param_1 + 0x4) == param_1) {
    (param_3 + param_2 * 0x2) = 0x0;
  }
  else {
    ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
    ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
    puVar1 = (u16 *)(param_2 * 0x2 + param_3);
    if (*puVar1 == param_1) {
      *puVar1 = (param_1 + 0x4);
    }
  }
  (param_1 + 0x4) = (param_3 + 0xa);
  (param_3 + 0xa) = param_1;
  return;
}



u16 mem_op_1000_03c6(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u8 param_5,u16 param_6)

{
  u16 *puVar1;
  i16 *piVar2;
  u16 uVar3;
  u16 uVar4;
  u16 *puVar5;
  u16 UVar6;
  u16 uVar7;
  u16 unaff_CS;
  bool bVar8;
  DWORD DVar9;
  u16 uVar10;
  u16 uStack20;
  u16 uVar9;

  uVar7 = CONCAT11(param_6,param_5);
  uVar3 = param_1 + 0xfff & 0xf000;
  puVar1 = (u16 *)(param_4 + 0x1e);
  uVar4 = uVar3 + *puVar1;
  uVar3 = param_2 + (u16)(0xf000 < param_1) + (param_4 + 0x20) + (u16)CARRY2(uVar3,*puVar1);
  puVar1 = (u16 *)(param_4 + 0x28);
  bVar8 = uVar3 < *puVar1;
  if ((bVar8) ||
     ((bVar8 || uVar3 == *puVar1 && (puVar1 = (u16 *)(param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
    if (param_3 == 0x3) {
      uStack20 = (u16)((u8)(-(u16)((param_5 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
    }
    else {
      uStack20 = 0x1000;
    }
    uStack20 = (param_4 + 0x16) | uStack20;
    mem_op_1000_131c(uStack20,CONCAT22(param_2,param_1));
    if ((uVar3 | uStack20) != 0x0) {
      puVar5 = (u16 *)mem_op_1000_0308(param_3,param_4);
      if (puVar5 != NULL) {
        puVar5[0x4] = uStack20;
        puVar5[0x5] = uVar3;
        PTR_LOOP_1050_000c = param_3 | 0xcad0;
//        *NULL = param_4;
//        &u16_1050_0002 = (int)&DAT_1050_1050;
        u16_1050_0002 = 0x1050;
        u32_1050_0004 = puVar5;
        ((int)&u32_1050_0004 + 0x2) = (int)&DAT_1050_1050;
        PTR_LOOP_1050_000a = 0x0;
        // SUB42(&DAT_1050_1050,0x0);
        uVar10 = 0x1050;
        DVar9 = mem_op_1000_1532(uStack20,uVar3);
        UVar6 = (u16)DVar9;
        if (param_3 == 0x1) {
          uVar7 = pass1_1000_0782(param_4,UVar6,0x0,(int)&DAT_1050_1050);
        }
        else if (param_3 == 0x3) {
          pass1_1000_05b4(param_5,0x0);
        }
        else {
          uVar7 = pass1_1000_09ca(UVar6,NULL);
        }
        param_2 = (int)(DVar9 >> 0x10);
        *puVar5 = uVar7;
        puVar5[0x1] = 0x8000;
        puVar1 = (u16 *)(param_4 + 0x1e);
        uVar4 = *puVar1;
        *puVar1 = *puVar1 + UVar6;
        piVar2 = (i16 *)(param_4 + 0x20);
        *piVar2 = *piVar2 + param_2 + (u16)CARRY2(uVar4,UVar6);
        return uVar3;
      }
      mem_op_1000_13ce(uStack20,uVar3);
    }
  }
  else {
    pass1_1000_1e61(unaff_CS,0x7,param_4,(u16)&DAT_1050_1050);
  }
  return 0x0;
}



u32 mem_op_1000_0510(u16 param_1,u16 param_2)

{
  u16 *puVar1;
  i16 *piVar2;
  u8 bVar3;
  i16 iVar4;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  bool bVar11;
  DWORD DVar12;
  i32 lVar13;
  u16 uVar14;
  u16 uVar5;

  iVar4 = param_2;
  uVar5 = (param_2 + 0x2);
  uVar6 = (param_2 + 0x4);
  bVar3 = *(u8 *)(param_2 + 0xc);
  DVar12 = mem_op_1000_1532(param_2,(u16)&DAT_1050_1050);
  uVar9 = (u16)(DVar12 >> 0x10);
  uVar8 = (u16)DVar12;
//  uVar14 = (u16)&DAT_1050_1050;
    uVar14 = 0x1050;
if (param_1 != 0x0) {
    uVar7 = (iVar4 + 0x1e);
    uVar10 = ((iVar4 + 0x20) - uVar9) - (u16)(uVar7 < uVar8);
    puVar1 = (u16 *)(iVar4 + 0x24);
    bVar11 = uVar10 < *puVar1;
    if ((bVar11 || uVar10 == *puVar1) && ((bVar11 || (uVar7 - uVar8 < (iVar4 + 0x22))))) {
      bVar11 = false;
      uVar9 = uVar10;
      goto LAB_1000_0595;
    }
  }
  pass1_1000_0368(uVar6,bVar3 & 0x7,0x0);
  puVar1 = (u16 *)((int)s_version__d__d_1050_0012 + 0xc);
  uVar7 = *puVar1;
  *puVar1 = *puVar1 - uVar8;
  piVar2 = (i16 *)s_New_failed_in_Op__Op_1050_0020;
  *piVar2 = (*piVar2 - uVar9) - (u16)(uVar7 < uVar8);
  bVar11 = true;
LAB_1000_0595:
  if (bVar11) {
    (param_2 + 0xc) = 0x0;
    lVar13 = mem_op_1000_13ce(param_2,uVar14);
    return CONCAT22((int)((u32)lVar13 >> 0x10),0x1);
  }
  return (u32)uVar9 << 0x10;
}



void pass1_1000_05b4(u8 param_1,i16 param_2)

{
  (param_2 + 0xa) = 0x1;
  (param_2 + 0x8) = 0x668;
  *(u8 *)(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
  (param_2 + 0x10) = 0x0;
  (param_2 + 0xe) = 0x0;
  return;
}



u32 mem_op_1000_05e2(u16 param_1,i16 param_2,u16 param_3,u16 param_4)

{
  u16 *puVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;
  u16 UVar5;
  u32 UVar6;
  u16 unaff_CS;
  bool bVar5;
  u32 uVar6;

  iVar2 = param_2 + (u16)(0xffeb < param_1);
  do {
    uVar3 = 0x3;
    UVar6._0_1_ = (undefined)param_3;
    UVar6._1_1_ = (u16)(param_3 >> 0x8);
    UVar6._0_2_ = mem_op_1000_03c6(param_1 + 0x14,iVar2,0x3,param_4,(undefined)UVar6,UVar6._1_1_);
    if (((u16)UVar6 | uVar3) != 0x0) {
      return CONCAT22((u16)UVar6,uVar3 + 0x14);
    }
    uVar6 = mem_op_1000_0052(param_4);
    uVar3 = param_1 + 0x1013 & 0xf000;
    puVar1 = (u16 *)(param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = iVar2 + (u16)(0xf000 < param_1 + 0x14) + (param_4 + 0x20) + (u16)CARRY2(uVar3,*puVar1);
    puVar1 = (u16 *)(param_4 + 0x28);
    bVar5 = uVar3 < *puVar1;
  } while (((bVar5 || uVar3 == *puVar1) &&
           ((bVar5 || (puVar1 = (u16 *)(param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) &&
          ((uVar6 != 0x0 || (UVar5 = pass1_1000_1e61(unaff_CS,0x2,param_4,(u16)&DAT_1050_1050), UVar5 != 0x0))));
  return 0x0;
}



u32 mem_1000_0668(void)

{
  u32 uVar1;

  uVar1 = mem_op_1000_0510(0x0,0x0);
  return uVar1;
}



u16 mem_1000_0670(u16 param_1,u32 *param_2,i16 param_3,u16 param_4,i16 *param_5)

{
  u16 *puVar1;
  i16 *piVar2;
  u16 UVar3;
  u16 UVar4;
  i16 iVar5;
  i16 iVar6;
  u32 uVar7;
  u16 uVar8;
  u32 *puVar9;
  u16 UVar10;
  BOOL16 BVar11;
  u16 uVar12;
  u16 uVar13;
  u16 unaff_CS;
  DWORD DVar14;
  DWORD DVar15;
  u16 uStack16;

  UVar3 = param_2;
  UVar4 = ((int)param_2 + 0x2);
  DVar14 = mem_op_1000_1532((u16)param_2,(u16)&DAT_1050_1050);
  iVar6 = param_3 + (u16)(0xffeb < param_1);
  uVar7 = *param_2;
  uVar8 = -(u16)((param_4 & 0x1) != 0x0) & 0x100 | -(u16)((param_4 & 0x4) != 0x0) & 0x400 |
          ((int)uVar7 + 0x16);
  if (param_5 == NULL) {
    BVar11 = mem_op_1000_14f2(uVar8 | 0x2000, param_1 + 0x14, (u16) param_2, (u16) &DAT_1050_1050);
    if (BVar11 == 0x0) {
      return 0x0;
    }
    uStack16 = (u16)&DAT_1050_1050;
  }
  else {
    iVar5 = (param_2 + 0x1);
    uVar12 = ((int)param_2 + 0x6);
    uVar13 = uVar12;
    do {
      uStack16 = uVar13;
      puVar9 = (u32 *)(uVar8 | 0x2000);
      mem_op_1000_1408((u16)puVar9,CONCAT22(iVar6,param_1 + 0x14),(u16)param_2,(u16)&DAT_1050_1050);
      uVar13 = uStack16 | (u16)puVar9;
      if (uVar13 != 0x0) break;
      UVar10 = pass1_1000_1e61(unaff_CS,0x2,UVar3,UVar4);
    } while (UVar10 != 0x0);
    if ((uStack16 | (u16)puVar9) == 0x0) {
      ((int)param_5 + 0x2) = 0x0;
      *param_5 = 0x0;
      return 0x0;
    }
    *(u32 **)(iVar5 + 0x8) = puVar9;
    (iVar5 + 0xa) = uStack16;
    *param_5 = (int)(puVar9 + 0x5);
    ((int)param_5 + 0x2) = uStack16;
    param_2 = puVar9;
  }
  DVar15 = mem_op_1000_1532((u16)param_2,uStack16);
  uVar12 = (u16)(DVar15 - DVar14);
  puVar1 = (u16 *)(UVar3 + 0x1e);
  uVar8 = *puVar1;
  *puVar1 = *puVar1 + uVar12;
  piVar2 = (i16 *)(UVar3 + 0x20);
  *piVar2 = *piVar2 + (int)(DVar15 - DVar14 >> 0x10) + (u16)CARRY2(uVar8,uVar12);
  return 0x1;
}



u16 pass1_1000_0782(u16 param_1,u16 param_2,i16 param_3,u16 param_4)

{
  (param_3 + 0xe) = 0x0;
  (param_3 + 0x10) = param_3 + 0x14;
  (param_3 + 0x8) = 0x9a0;
  pass1_1000_07ac((param_1 + 0x18),param_2,param_3);
  return 0x1;
}



void pass1_1000_07ac(u16 param_1,i16 param_2,i16 param_3)

{
  u16 *puVar1;
  i16 iVar2;
  u16 uVar3;

  puVar1 = (u16*)(param_3 + 0x10);
  (u16*)(param_3 + 0xe) = puVar1;
  uVar3 = param_2 + (param_3 - (int)puVar1);
  iVar2 = (int)puVar1 + (uVar3 - uVar3 % param_1);
  (param_3 + 0x10) = iVar2;
  while (puVar1 < (u16 *)(iVar2 - param_1)) {
    *puVar1 = (u16 *)((int)puVar1 + param_1);
    puVar1 = (u16 *)((int)puVar1 + param_1);
  }
  *puVar1 = 0x0;
  return;
}



struct astruct_99 * pass1_1000_07fc(u32 param_1)

{
  u16 unaff_CS;
  struct astruct_99 *paVar1;

  if (((int)param_1 + 0x14) != -0x4153) {
    pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
    return NULL;
  }
  paVar1 = (astruct_99 *)mem_op_1000_0838(0x0);
  return paVar1;
}



u32 mem_op_1000_0838(u16 param_1)

{
  u16 *puVar1;
  i16 *piVar2;
  i16 iVar3;
  u16 *puVar4;
  u16 uVar5;
  u16 uVar6;
  u16 UVar7;
  u16 UVar8;
  i16 *piVar9;
  u16 unaff_CS;
  bool bVar10;
  u16 uStack6;
  i16 *piStack4;

  piVar9 = *(i16 **)(param_1 + 0x2);
  piStack4 = piVar9;
  if ((param_1 + 0x2) == 0x0) goto LAB_1000_085b;
  do {
    do {
      if (*piVar9 != 0x0) {
        iVar3 = piVar9[0x5];
        puVar4 = (u16 *)&PTR_LOOP_1050_000e;
        if (puVar4 != NULL) {
          &PTR_LOOP_1050_000e = *puVar4;
          piVar2 = (i16 *)&PTR_LOOP_1050_000a;
          *piVar2 = *piVar2 + 0x1;
          *(i16 **)(param_1 + 0x2) = piVar9;
          return CONCAT22(iVar3,puVar4);
        }
        *piVar9 = 0x0;
      }
      piVar9 = (i16 *)piVar9[0x2];
    } while (piVar9 != piStack4);
LAB_1000_085b:
    if ((param_1 + 0x18) == 0x0) {
      pass1_1000_1e61(unaff_CS,0x4,param_1,(u16)&DAT_1050_1050);
      return 0x0;
    }
    uVar5 = (param_1 + 0x1a);
    while( true ) {
      uStack6 = uVar5;
      uVar5 = 0x1;
      UVar8 = mem_op_1000_03c6(uStack6,0x0,0x1,param_1,0x0,'\0');
      if ((UVar8 | uVar5) != 0x0) break;
      uVar5 = (param_1 + 0x1e);
      uVar6 = uVar5 + uStack6;
      uVar5 = (param_1 + 0x20) + (u16)CARRY2(uVar5,uStack6);
      puVar1 = (u16 *)(param_1 + 0x28);
      bVar10 = *puVar1 <= uVar5;
      if (bVar10) {
        if (bVar10 && uVar5 != *puVar1) {
          return 0x0;
        }
        puVar1 = (u16 *)(param_1 + 0x26);
        if (*puVar1 <= uVar6 && uVar6 != *puVar1) {
          return 0x0;
        }
      }
      uVar5 = uStack6 >> 0x1;
      if (uStack6 >> 0x1 < (param_1 + 0x18) + 0x14U) {
        UVar7 = pass1_1000_1e61(unaff_CS,0x2,param_1,(u16)&DAT_1050_1050);
        uVar5 = uStack6 & 0xfffe;
        if (UVar7 == 0x0) {
          return 0x0;
        }
      }
    }
    piVar9 = *(i16 **)(param_1 + 0x2);
    piStack4 = (i16 *)piVar9[0x2];
  } while( true );
}



u16 pass1_1000_093a(astruct_611 *param_1)

{
  i16 *piVar1;
  u16 unaff_CS;

  if (&PTR_LOOP_1050_000c != -0x352f) {
    pass1_1000_1e61(unaff_CS,0xe,0x0,0x0);
    return 0x0;
  }
  param_1 = &PTR_LOOP_1050_000e;
  if (param_1 == 0x0) {
    *(u32 *)&u32_1050_0004 = 0x1;
  }
  &PTR_LOOP_1050_000e = param_1._0_2_;
  piVar1 = (i16 *)&PTR_LOOP_1050_000a;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    mem_op_1000_0510(0x1,0x0);
  }
  return 0x1;
}



uchar * pass1_1000_09a0(u16 *param_1)

{
  uchar *puVar1;
  u32 uVar2;

  *param_1 = (u16)PTR_LOOP_1050_000e;
  if (PTR_LOOP_1050_000e == NULL) {
    u32_1050_0004 = 0x1;
  }
  PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
  puVar1 = PTR_LOOP_1050_000e;
  PTR_LOOP_1050_000e = (u8 *)param_1;
  if (PTR_LOOP_1050_000a == NULL) {
    uVar2 = mem_op_1000_0510(0x1,0x0);
    puVar1 = (uchar *)uVar2;
  }
  return puVar1;
}



u16 pass1_1000_09ca(i16 param_1,u16 *param_2)

{
  u16 *puVar1;
  i16 iVar2;
  u32 uVar3;
  u16 *puVar4;

  puVar1 = param_2 + 0xa;
  puVar4 = (u16 *)(((int)param_2 + (param_1 - (int)puVar1) + -0x6 & 0xfffcU) + (int)puVar1);
  *puVar4 = 0x1;
  param_2[0x7] = (u16)puVar1;
  puVar4[0x2] = (u16)puVar4;
  puVar4[0x1] = (u16)puVar4;
  param_2[0x8] = (u16)puVar4;
  if ((*(u8 *)(param_2 + 0x6) & 0x7) == 0x2) {
    param_2[0x9] = 0x8;
  }
  else {
    uVar3 = *(u32 *)param_2;
    iVar2 = ((int)uVar3 + 0x18);
    param_2[0x9] = (iVar2 - 0x5U & ~-(u16)(iVar2 + 0x3U < 0x8)) + 0x8;
  }
  puVar4[-0x1] = (int)puVar4 - (int)puVar1;
  *puVar1 = (int)puVar4 - (int)puVar1 | 0x2;
  param_2[0xc] = (u16)puVar4;
  param_2[0xb] = puVar4[0x1];
  (u16*)(puVar4[0x1] + 0x4) = puVar1;
  puVar4[0x1] = (u16)puVar1;
  param_2[0x4] = 0xe08;
  return *puVar1 & 0xfffc;
}



i32 mem_op_1000_0a48(u8 param_1,u16 param_2,i16 param_3,u32 param_4)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar4;
  u16 uVar3;
  u16 UVar4;
  u16 unaff_CS;
  u32 uVar5;
  u8 in_stack_00000005;
  u16 *puVar1;

  UVar4 = (u16)(param_4 >> 0x10);
  if (((u16)param_4 + 0x14) == -0x4153) {
    if ((param_3 == 0x0) && (param_2 <= ((int)s_version__d__d_1050_0012 + 0x6))) {
      if (param_2 == 0x0) {
        pass1_1000_1e61(unaff_CS,0x4,(u16)param_4,UVar4);
        uVar5 = 0x0;
      }
      else {
        uVar5 = mem_op_1000_0838(0x0);
        uVar3 = (u16)(uVar5 >> 0x10);
        puVar2 = (u16 *)uVar5;
        if ((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
          uVar1 = ((int)s_version__d__d_1050_0012 + 0x6);
          for (uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar2;
            puVar2 = puVar2 + 0x1;
            *puVar1 = 0x0;
          }
          if ((uVar1 & 0x1) != 0x0) {
            *(u8 *)puVar2 = 0x0;
          }
        }
      }
    }
    else if ((param_3 == 0x0) && (param_2 <= ((int)s_version__d__d_1050_0012 + 0xa))) {
      uVar5 = mem_op_1000_0b20(_param_1 & 0xfffd,0x0,param_2);
    }
    else {
      uVar5 = mem_op_1000_05e2(param_2,param_3,_param_1 & 0xfffd,0x0);
    }
    return uVar5;
  }
  pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
  return 0x0;
}



u32 mem_op_1000_0b20(u16 param_1,u16 param_2,u16 param_3)

{
  u16 *puVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 UVar6;
  u16 *puVar7;
  u16 uVar8;
  bool bVar9;
  u32 uVar10;
  u16 uStack20;
  u16 *puStack6;

  uVar8 = SUB42(&DAT_1050_1050,0x0);
  uVar2 = param_1 & 0x2;
  uVar4 = param_3 + 0x5 & 0xfffc;
  uVar4 = uVar4 - 0x8 & ~-(u16)(uVar4 < 0x8);
  uVar5 = uVar4 + 0x8;
  puVar7 = (u16*)(uVar2 * 0x2 + param_2);
  uStack20 = param_1;
  puStack6 = puVar7;
  if (puVar7 == NULL) goto LAB_1000_0b64;
  do {
    do {
      if ((uVar5 <= *puVar7) && (uVar10 = pass1_1000_0c32(uVar5,uStack20,0x0), uVar10 != 0x0)) {
        (u16*)(uVar2 * 0x2 + param_2) = puVar7;
        return uVar10;
      }
      puVar7 = (u16 *)puVar7[0x2];
    } while (puVar7 != puStack6);
LAB_1000_0b64:
    if (((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) || ((param_2 + 0x32) == 0x0)) ||
       (uVar3 = (**(code **)(param_2 + 0x32))(), uVar3 < uVar5)) {
      if (((uStack20 & 0x10) != 0x0) ||
         (uVar3 = uVar2, UVar6 = mem_op_1000_03c6((param_2 + 0x1a),0x0,uVar2,param_2,0x0,'\0'),
         (UVar6 | uVar3) == 0x0)) {
        if ((uStack20 & 0x20) == 0x0) {
          uVar2 = uVar4 + 0x1007 & 0xf000;
          puVar1 = (u16 *)(param_2 + 0x1e);
          uVar4 = uVar2 + *puVar1;
          uVar2 = (param_2 + 0x20) + (u16)CARRY2(uVar2,*puVar1);
          puVar1 = (u16 *)(param_2 + 0x28);
          bVar9 = uVar2 < *puVar1;
          if ((bVar9 || uVar2 == *puVar1) &&
             ((bVar9 || (puVar1 = (u16 *)(param_2 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
            uVar10 = mem_op_1000_05e2(uVar5,0x0,uStack20,param_2);
            return uVar10;
          }
        }
        return 0x0;
      }
    }
    else {
      uStack20 |= 0x40;
    }
    puVar7 = (u16*)(uVar2 * 0x2 + param_2);
    puStack6 = (u16 *)puVar7[0x2];
  } while( true );
}



u32 pass1_1000_0c32(u16 param_1,u16 param_2,u16 param_3)

{
  u16 *puVar1;
  u8 *pbVar2;
  i16 *piVar3;
  u32 uVar4;
  u16 uVar5;
  u16 *puVar6;
  i16 iVar7;
  u16 *puVar8;
  u16 uVar9;
  u16 uStack14;
  u16 *puStack8;
  u16 uStack6;

  puVar8 = (u16*)(param_3 + 0xe);
  uStack6 = 0x0;
  puVar6 = puVar8;
  while( true ) {
    do {
      uVar5 = *puVar6;
      if (param_1 <= uVar5) {
        uVar5 = (uVar5 & 0xfffc) - param_1;
        puVar1 = (u16 *)(param_3 + 0x12);
        puStack8 = puVar6;
        if (*puVar1 < uVar5 || *puVar1 == uVar5) {
          uStack14 = param_1;
          if ((param_2 & 0x6) == 0x0) {
            puStack8 = (u16 *)(uVar5 + (int)puVar6);
            puStack8[-0x1] = uVar5;
            *puVar6 = uVar5 | 0x2;
            puVar8 = (u16 *)puVar6[0x1];
            pbVar2 = (u8 *)((int)puStack8 + param_1);
            *pbVar2 = *pbVar2 | 0x2;
            *puStack8 = param_1 | 0x1;
          }
          else {
            *puVar6 = param_1 & 0xff00 | *(u8 *)puVar6 & 0x2 | param_1 & 0xff | 0x1;
            (puVar6[0x2] + 0x2) = puVar6[0x1];
            (puVar6[0x1] + 0x4) = puVar6[0x2];
            puVar8 = (u16 *)((int)puVar6 + param_1);
            ((int)puVar8 + (uVar5 - 0x2)) = uVar5;
            *puVar8 = uVar5 | 0x2;
            uVar5 = (param_3 + 0x10);
            puVar8[0x2] = uVar5;
            puVar8[0x1] = (uVar5 + 0x2);
            (u16*)((uVar5 + 0x2) + 0x4) = puVar8;
            (u16*)(uVar5 + 0x2) = puVar8;
          }
        }
        else {
          puVar8 = (u16 *)puVar6[0x1];
          (u16*)(puVar6[0x2] + 0x2) = puVar8;
          (puVar6[0x1] + 0x4) = puVar6[0x2];
          puVar1 = puVar6;
          *(u8 *)puVar1 = *(u8 *)puVar1 | 0x1;
          uStack14 = *puVar6 & 0xfffc;
          ((int)puVar6 + uStack14) = ((int)puVar6 + uStack14) | 0x2;
        }
        (u16*)(param_3 + 0xe) = puVar8;
        if ((param_2 & 0x1) != 0x0) {
          puVar6 = puStack8;
          for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0; uVar5 -= 0x1) {
            *puVar6 = 0x0;
          }
          if ((uStack14 - 0x2 & 0x1) != 0x0) {
            *(u8 *)puVar6 = 0x0;
          }
        }
        if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
          *(u16*)(param_3 + 0x4) = *(u16*)((param_3 + 0x10) + 0x2) & 0xfffc;
          uVar4 = *(u32 *)(param_3 + 0x4);
          pbVar2 = (u8 *)((int)uVar4 + 0x3);
          *pbVar2 = *pbVar2 | 0x80;
        }
        piVar3 = (i16 *)(param_3 + 0xa);
        *piVar3 = *piVar3 + 0x1;
        return CONCAT22(0x1050,puStack8 + 0x1);
      }
      if (uStack6 < uVar5) {
        uStack6 = uVar5;
      }
      puVar6 = (u16 *)puVar6[0x1];
    } while (puVar6 != puVar8);
    if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) break;
    uVar4 = *(u32 *)param_3;
    uVar9 = (u16)((u32)uVar4 >> 0x10);
    iVar7 = (int)uVar4;
    if ((iVar7 + 0x34) == 0x0) break;
    uStack6 = (**(code **)(iVar7 + 0x34))();
    if ((uStack6 < param_1) || (puVar6 = (u16*)(param_3 + 0xe), puVar6 == NULL)) break;
  }
  *(u16*)(param_3 + 0x4) = uStack6 & 0xfffc;
  return 0x0;
}



BOOL16 call_fn_ptr_1000_0dc6(char *param_1)

{
  u16 unaff_CS;

  if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(unaff_CS,0xe,0x0,0x0);
    return 0x0;
  }
  (**(code **)&u16_1050_0008)((int)&DAT_1050_1050);
  return 0x1;
}



u16 pass1_1000_0e08(i16 param_1)

{
  u16 *puVar1;
  u8 *pbVar2;
  u16 uVar3;
  u16 *puVar4;
  u16 *puVar5;
  bool bVar6;
  u32 uVar7;

  puVar5 = (u16 *)(param_1 + -0x2);
  bVar6 = (*(u8 *)puVar5 & 0x2) != 0x0;
  if (bVar6) {
    puVar1 = puVar5;
    *(u8 *)puVar1 = *(u8 *)puVar1 & 0xfe;
  }
  else {
    puVar4 = (u16 *)((int)puVar5 - (param_1 + -0x4));
    puVar1 = puVar4;
    *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
    puVar5 = puVar4;
  }
  puVar4 = (u16 *)((*puVar5 & 0xfffc) + (int)puVar5);
  if ((*(u8 *)puVar4 & 0x1) == 0x0) {
    puVar1 = puVar5;
    *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
    if (puVar4 == (u16 *)PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = (u8 *)puVar5;
    }
    (puVar4[0x2] + 0x2) = puVar4[0x1];
    (puVar4[0x1] + 0x4) = puVar4[0x2];
    puVar4 = (u16 *)((*puVar5 & 0xfffc) + (int)puVar5);
  }
  puVar4[-0x1] = *puVar5 & 0xfffc;
  uVar3 = u32_1050_0004;
  puVar1 = puVar4 + -0x1;
  if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
    uVar3 = *puVar5 & 0xfffc;
    u32_1050_0004 = uVar3;
  }
  puVar1 = puVar4;
  *(u8 *)puVar1 = *(u8 *)puVar1 & 0xfd;
  if (bVar6) {
    if ((StructD *)PTR_LOOP_1050_0010->address_offset_field_0x2 != PTR_LOOP_1050_0010) {
      pbVar2 = (u8 *)((int)u32_1050_0004 + 0x3);
      *pbVar2 = *pbVar2 & 0x7f;
    }
    puVar5[0x2] = (u16)PTR_LOOP_1050_0010;
    uVar3 = PTR_LOOP_1050_0010->address_offset_field_0x2;
    puVar5[0x1] = uVar3;
    (u16*)(PTR_LOOP_1050_0010->address_offset_field_0x2 + 0x4) = puVar5;
    PTR_LOOP_1050_0010->address_offset_field_0x2 = (u16)puVar5;
  }
  PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
  if (PTR_LOOP_1050_000a == NULL) {
    uVar7 = mem_op_1000_0510(0x1,0x0);
    uVar3 = (u16)uVar7;
  }
  return uVar3;
}



i32 pass1_1000_0ed4(u16 param_1,u16 param_2,u16 param_3,astruct_172 *param_4,astruct_172 *param_5)

{
  struct astruct_172 *paVar1;
  u16 *puVar2;
  u16 uVar3;
  struct astruct_172 **ppaVar4;
  u16 uVar5;
  u16 uVar6;
  struct astruct_172 *paVar7;
  u16 *puVar8;
  u16 unaff_CS;
  i32 lVar9;
  u16 UVar10;
  u16 UVar11;
  u16 UVar12;

  if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
    UVar11 = *NULL;
    UVar12 = &u16_1050_0002;
    if ((param_1 & 0x8) == 0x0) {
      ppaVar4 = &param_4;
      uVar6 = (u16)&DAT_1050_1050;
    }
    else {
      ppaVar4 = NULL;
      uVar6 = 0x0;
    }
    uVar6 = pass1_1000_0fb8(param_2,(int)param_4,param_3,param_1,(u16 *)ppaVar4,uVar6);
    if (uVar6 == 0x0) {
      return CONCAT22(param_5,param_4);
    }
    if ((param_1 & 0x8) == 0x0) {
      lVar9 = mem_op_1000_0a48((u8)param_1,param_2,param_3,CONCAT22(UVar12,UVar11));
      uVar3 = (u16)((u32)lVar9 >> 0x10);
      puVar8 = (u16 *)lVar9;
      if (lVar9 != 0x0) {
        paVar7 = param_4;
        for (uVar5 = uVar6 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
          puVar2 = puVar8;
          puVar8 = puVar8 + 0x1;
          paVar1 = paVar7;
          paVar7 = (astruct_172 *)&paVar7->field1_0x2;
          *puVar2 = paVar1->field0_0x0;
        }
        for (uVar6 = (u16)((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
          puVar2 = puVar8;
          puVar8 = (u16 *)((int)puVar8 + 0x1);
          paVar1 = paVar7;
          paVar7 = (astruct_172 *)((int)&paVar7->field0_0x0 + 0x1);
          *(u8 *)puVar2 = *(u8 *)&paVar1->field0_0x0;
        }
        call_fn_ptr_1000_0dc6((char *)CONCAT22(param_5,param_4));
      }
      return lVar9;
    }
    if ((param_3 | param_2) == 0x0) {
      return 0x0;
    }
    UVar10 = 0x5;
  }
  else {
    UVar11 = 0x0;
    UVar12 = 0x0;
    UVar10 = 0xe;
  }
  pass1_1000_1e61(unaff_CS,UVar10,UVar11,UVar12);
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10001126)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1000_0fb8(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u16 *param_5,u16 param_6)

{
  u16 *puVar1;
  u8 bVar2;
  u16 uVar3;
  BOOL16 BVar4;
  i16 iVar5;
  u16 uVar6;
  u16 *puVar7;
  u16 *puVar8;
  u16 unaff_CS;
  u32 uVar9;
  u16 uStack4;

  if ((param_3 | param_1) == 0x0) {
    pass1_1000_1e61(unaff_CS,0x4,(u16)PTR_LOOP_1050_0000,u16_1050_0002);
    if ((param_6 | (u16)param_5) != 0x0) {
      param_5[0x1] = 0x0;
      *param_5 = 0x0;
      return 0x0;
    }
    return 0x1;
  }
  bVar2 = (u8)PTR_LOOP_1050_000c & 0x7;
  if (((u8)PTR_LOOP_1050_000c & 0x7) != 0x0) {
    if (bVar2 == 0x1) {
      uVar3 = (PTR_LOOP_1050_0000 + 0x18);
      if (param_3 != 0x0) {
        return uVar3;
      }
      if (param_1 <= uVar3) {
        return 0x0;
      }
      return uVar3;
    }
    if (bVar2 != 0x2) {
      if (bVar2 != 0x3) {
        if ((param_6 | (u16)param_5) != 0x0) {
          param_5[0x1] = 0x0;
          *param_5 = 0x0;
          return 0x0;
        }
        return 0x1;
      }
      if ((((param_6 | (u16)param_5) != 0x0) && (param_3 == 0x0)) && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c)))
      {
        uVar9 = pass1_1000_1284(CONCAT22(0x1050,param_2));
        if (CONCAT22(param_3,param_1) < uVar9) {
          return param_1;
        }
        return (u16)uVar9;
      }
      iVar5 = mem_1000_0670(param_1,NULL,param_3,param_4,(i16 *)CONCAT22(param_6,param_5));
      if (iVar5 != 0x0) {
        return 0x0;
      }
      if ((param_6 | (u16)param_5) != 0x0) {
        return 0x0;
      }
      return 0x1;
    }
  }
  puVar8 = (u16 *)(param_2 + -0x2);
  uVar3 = *puVar8 & 0x7ffc;
  uStack4 = uVar3 - 0x2;
  if ((*(u8 *)(param_2 + -0x1) & 0x80) != 0x0) {
    uStack4 = uVar3 - 0x6;
  }
  if ((((param_3 == 0x0) && (param_1 <= uStack4)) ||
      ((param_3 == 0x0 && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c))))) &&
     (BVar4 = pass1_1000_115c(param_1,puVar8), BVar4 != 0x0)) {
    if ((param_4 & 0x1) != 0x0) {
      uVar3 = (*puVar8 & 0x7ffc) - 0x2;
      if (uStack4 < param_1) {
        puVar7 = (u16 *)(uStack4 + param_2);
        iVar5 = -uStack4;
      }
      else {
        if (uVar3 <= param_1) {
          return 0x0;
        }
        puVar7 = (u16 *)(param_1 + param_2);
        iVar5 = -param_1;
      }
      uVar3 += iVar5;
      for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = 0x0;
      }
      if ((uVar3 & 0x1) != 0x0) {
        *(u8 *)puVar7 = 0x0;
      }
    }
    return 0x0;
  }
  return uStack4;
}



BOOL16 pass1_1000_115c(i16 param_1,u16 *param_2)

{
  u8 *pbVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u16 *puVar5;
  u16 UVar6;
  u16 uStack4;

  uVar3 = *param_2 & 0x7ffc;
  uVar4 = param_1 + 0x5U & 0xfffc;
  uVar4 = (uVar4 - 0x8 & ~-(u16)(uVar4 < 0x8)) + 0x8;
  if (uVar3 < uVar4) {
    puVar5 = (u16 *)(uVar3 + (int)param_2);
    if (((*(u8 *)puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
      return 0x0;
    }
    if (puVar5 == (u16 *)PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = (u8 *)puVar5[0x1];
    }
    (puVar5[0x2] + 0x2) = puVar5[0x1];
    (puVar5[0x1] + 0x4) = puVar5[0x2];
    uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      puVar2 = param_2;
      *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
      pbVar1 = (u8 *)((int)puVar5 + (*puVar5 & 0xfffc));
      *pbVar1 = *pbVar1 | 0x2;
      return 0x1;
    }
  }
  else {
    uStack4 = uVar3 - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      return 0x1;
    }
    puVar5 = (u16 *)(uVar3 + (int)param_2);
    if ((*(u8 *)puVar5 & 0x1) == 0x0) {
      uStack4 += *puVar5 & 0xfffc;
      if (puVar5 == (u16 *)PTR_LOOP_1050_000e) {
        PTR_LOOP_1050_000e = (u8 *)puVar5[0x1];
      }
      (puVar5[0x2] + 0x2) = puVar5[0x1];
      (puVar5[0x1] + 0x4) = puVar5[0x2];
    }
    if (u32_1050_0004 < uStack4) {
      u32_1050_0004 = uStack4;
    }
  }
  *param_2 = *param_2 & 0x8003 | uVar4;
  (uVar4 + (int)param_2) = uStack4 | 0x2;
  UVar6 = uVar4 + (int)param_2;
  *(StructD **)(UVar6 + 0x4) = PTR_LOOP_1050_0010;
  (UVar6 + 0x2) = PTR_LOOP_1050_0010->address_offset_field_0x2;
  (PTR_LOOP_1050_0010->address_offset_field_0x2 + 0x4) = UVar6;
  PTR_LOOP_1050_0010->address_offset_field_0x2 = UVar6;
  ((u8 *)(UVar6 + uStack4) + -0x2) = uStack4;
  pbVar1 = (u8 *)(UVar6 + uStack4);
  *pbVar1 = *pbVar1 & 0xfd;
  return 0x1;
}



u32 pass1_1000_1284(u32 param_1)

{
  u8 bVar1;
  u16 uVar2;
  u32 uVar3;
  u8 bVar4;
  u16 uVar5;
  u16 unaff_CS;
  bool bVar6;
  DWORD DVar7;
  u16 uStack6;
  i16 iStack4;

  if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(unaff_CS,0xe,0x0,0x0);
    return 0xffffffff;
  }
  bVar1 = *(u8 *)&PTR_LOOP_1050_000c;
  bVar4 = bVar1 & 0x7;
  if ((bVar1 & 0x7) != 0x0) {
    if (bVar4 == 0x1) {
      uVar3 = *NULL;
      return (u32)((int)uVar3 + 0x18);
    }
    if (bVar4 != 0x2) {
      if (bVar4 != 0x3) {
        return 0xffffffff;
      }
      DVar7 = mem_op_1000_1532(0x0,param_1._2_2_);
      return CONCAT22((int)(DVar7 >> 0x10) - (u16)((u16)DVar7 < 0x14),(u16)DVar7 - 0x14);
    }
  }
  uVar2 = ((int)param_1 + -0x2);
  uVar5 = uVar2 & 0x7ffc;
  uStack6 = uVar5 - 0x2;
  iStack4 = 0x0;
  if ((uVar2 & 0x8000) != 0x0) {
    bVar6 = uStack6 < 0x4;
    uStack6 = uVar5 - 0x6;
    iStack4 = -(u16)bVar6;
  }
  return CONCAT22(iStack4,uStack6);
}



void mem_op_1000_131c(u16 param_1,u32 param_2)

{
  HGLOBAL16 handle;
  u16 flags;
  bool bVar1;
  i32 lVar2;
  u16 uStack10;
  u16 uStack8;
  i16 iStack6;

  lVar2 = CONCAT22(uStack8,uStack10);
  flags = 0x32;
  iStack6 = 0x1;
  if (((param_1 & 0x1000) != 0x0) && ((param_2._2_2_ != 0x0 || (0xfff0 < (u16)param_2)))) {
    param_2 = 0xfff0;
  }
  if ((param_1 & 0x100) != 0x0) {
    flags = 0x72;
  }
  if ((param_1 & 0x1) != 0x0) {
    flags |= 0x2000;
  }
  if ((param_1 & 0x4) != 0x0) {
    flags &= 0xfffd;
    lVar2 = mem_op_1000_1558((u16)param_2,param_2._2_2_);
  }
  do {
    handle = GLobalAlloc16(param_2 & 0xffff | (u32)param_2._2_2_ << 0x10,flags);
    uStack8 = (u16)((u32)lVar2 >> 0x10);
    uStack10 = (u16)lVar2;
    if (handle != 0x0) break;
    flags &= 0xffcf;
    bVar1 = iStack6 != 0x0;
    iStack6 = iStack6 + -0x1;
  } while (bVar1);
  if ((param_1 & 0x4) != 0x0) {
    if (handle != 0x0) {
      GlobalPageLock16(handle);
    }
    pass1_1000_15ce((u16 *)uStack10,uStack8);
  }
  if (handle == 0x0) {
    return;
  }
  WIN16_GlobalLock16(handle);
  return;
}



i32 mem_op_1000_13ce(u16 param_1,u16 param_2)

{
  HGLOBAL16 HVar1;
  u16 uVar2;
  DWORD DVar3;

  DVar3 = GlobalHandle16(param_2);
  uVar2 = (u16)(DVar3 >> 0x10);
  if ((HGLOBAL16)DVar3 != 0x0) {
    HVar1 = GlobalFree16((HGLOBAL16)DVar3);
    return CONCAT22(uVar2,(u16)(HVar1 == 0x0));
  }
  return (u32)uVar2 << 0x10;
}



void mem_op_1000_1408(u16 param_1,u32 param_2,u16 param_3,u16 param_4)

{
  HGLOBAL16 handle;
  u32 DVar1;
  u16 realloc_flags;
  HGLOBAL16 UVar2;

  DVar1 = GlobalHandle16(param_4);
  DVar1._0_2_ = (HGLOBAL16)DVar1;
  realloc_flags = 0x32;
  if (((param_1 & 0x1000) != 0x0) && ((param_2._2_2_ != 0x0 || (0xfff0 < (u16)param_2)))) {
    param_2 = 0xfff0;
  }
  if ((param_1 & 0x100) != 0x0) {
    realloc_flags = 0x72;
  }
  if ((param_1 & 0x804) != 0x0) {
    realloc_flags &= 0xfffd;
  }
  if ((HGLOBAL16)DVar1 != 0x0) {
    if ((param_1 & 0x4) != 0x0) {
      GlobalPageUnlock16((HGLOBAL16)DVar1);
    }
    do {
      UVar2 = (HGLOBAL16)DVar1;
      handle = GlobalReAlloc16(realloc_flags,param_2,(HGLOBAL16)DVar1);
      if (handle != 0x0) break;
      realloc_flags &= 0xffcf;
    } while (UVar2 != 0x0);
    if ((handle != 0x0) && ((param_1 & 0x4) != 0x0)) {
      GlobalPageLock16(handle);
    }
    if (handle != 0x0) {
      WIN16_GlobalLock16(handle);
      return;
    }
  }
  return;
}



BOOL16 mem_op_1000_14f2(u16 param_1, u32 param_2, astruct_7 *param_4, u16 param_5)

{
  u16 in_AX;
  u16 in_DX;

  if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
    mem_op_1000_1408(param_1 & 0xfdff | 0x800,CONCAT22(param_3,param_2),param_4,param_5);
    if ((in_DX | in_AX) != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}

DWORD mem_op_1000_1532(astruct_7 *param_1, i16 selector)
{
    DWORD mem_size;

    // get handle to global memory
    mem_size = GlobalHandle16(selector);
    if ((HGLOBAL16) mem_size != 0x0) {
        // get size of memory area
        mem_size = GlobalSize16((HGLOBAL16) mem_size);
        return mem_size;
    }
    return 0x0;
}



i32 mem_op_1000_1558(u16 param_1,u16 param_2)

{
  u16 uVar1;
  u32 uVar3;
  u16 uStack12;
  u16 uStack10;
  u16 uStack8;
  u16 uVar2;

  uStack12 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x8;
  if ((param_2 < 0x9) && ((param_2 < 0x8 || (param_1 == 0x0)))) {
    do {
      while( true ) {
        uVar3 = GlobalDOSAlloc16(CONCAT22(uStack8,uStack10));
        uVar1 = (u16)uVar3;
        if (uVar1 == 0x0) break;
        *NULL = 0x0;
        &u16_1050_0002 = uStack12;
        uStack12 = uVar1;
      }
      uVar2 = uStack8 & 0x1;
      uStack8 >>= 0x1;
      uStack10 = uStack10 >> 0x1 | (u16)(uVar2 != 0x0) << 0xf;
    } while ((param_2 < uStack8) || ((param_2 <= uStack8 && (param_1 <= uStack10))));
  }
  return (u32)uStack12 << 0x10;
}



// WARNING: Unable to use type for symbol uVar3

void pass1_1000_15ce(u16 *param_1,u16 param_2)

{
  u16 *puVar1;
  u16 uVar2;
  u16 *puVar2;
  u16 uVar3;

  uVar2 = param_2 | (u16)param_1;
  while (uVar2 != 0x0) {
    puVar1 = (u16 *)*param_1;
    uVar3 = param_1[0x1];
    GlobalDOSFree16(param_2);
    param_1 = puVar1;
    param_2 = uVar3;
    uVar2 = uVar3 | (u16)puVar1;
  }
  return;
}



uchar * mem_op_1000_160a(StructD *param_1)

{
  uchar *puVar1;
  u16 uVar1;
  u32 uVar2;

  uVar1 = (u16)param_1;
  puVar1 = (uchar *)ret_true_1000_2146();
  if (puVar1 == NULL) {
    return puVar1;
  }
  if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) == 0x0) {
    DAT_1050_5f30 = 0x1;
    DAT_1050_5f32 = 0x1;
    uVar2 = mem_op_1000_18ec(DAT_1050_5f46,uVar1);
    PTR_LOOP_1050_5f2e = (u8 *)(uVar2 >> 0x10);
    PTR_LOOP_1050_5f2c = (u8 *)uVar2;
    if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) != 0x0) {
      if (PTR_LOOP_1050_5f42 != NULL) {
        pass1_1000_1a54((u16)PTR_LOOP_1050_5f42,(int)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
      }
      if (DAT_1050_5f44 != 0xffff) {
        pass1_1000_1afe(DAT_1050_5f44,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
      }
    }
  }
  empty_fn_1000_214a();
  return PTR_LOOP_1050_5f2c;
}



u16 mem_1000_167a(u16 param_1,u16 param_2)

{
  uchar *puVar1;
  u16 in_register_0000000a;
  StructD *pSVar2;
  i32 lVar3;

  pSVar2 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(pSVar2);
    if (((u16)pSVar2 | (u16)puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar3 = mem_op_1000_0a48(0x0,param_2,0x0,CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c));
  return (u16)lVar3;
}



u16 pass1_1000_16aa(u16 param_1,u16 *param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;
  i32 lVar2;

  if ((param_3 | (u16)param_2) == 0x0) {
    uVar1 = mem_1000_167a(param_1,param_4);
    return uVar1;
  }
  if (param_4 == 0x0) {
    pass1_1000_16ee((u16)param_2,param_3);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(0x0,param_4,0x0,(astruct_172 *)param_2,(astruct_172 *)param_3);
  return (u16)lVar2;
}



void pass1_1000_16ee(u16 param_1,u16 param_2)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6((char *)CONCAT22(param_2,param_1));
  }
  return;
}



u16 fn_ptr_op_1000_1708(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  i16 iVar1;
  bool bVar2;
  i32 lVar3;

  if ((param_2 | param_1) == 0x0) {
    bVar2 = 0xfffe < param_1;
    param_1 += 0x1;
    param_2 += bVar2;
  }
LAB_1000_1724:
  do {
    if ((param_5 | param_4) != 0x0) {
      lVar3 = mem_op_1000_0a48((u8)param_3,param_1,param_2,CONCAT22(param_5,param_4));
      if (lVar3 != 0x0) {
        return (u16)lVar3;
      }
    }
    if (((param_3 & 0x8000) == 0x0) || (((u16)PTR_LOOP_1050_5f3a | (u16)PTR_LOOP_1050_5f38) == 0x0)) {
      if (((u16)PTR_LOOP_1050_5f36 | (u16)PTR_LOOP_1050_5f34) == 0x0) {
        if (((u16)PTR_LOOP_1050_5f3e | (u16)PTR_LOOP_1050_5f3c) == 0x0) {
          return 0x0;
        }
        (*(code *)PTR_LOOP_1050_5f3c)();
        goto LAB_1000_1724;
      }
      iVar1 = (*(code *)PTR_LOOP_1050_5f34)();
    }
    else {
      iVar1 = (*(code *)PTR_LOOP_1050_5f38)();
    }
    if (iVar1 == 0x0) {
      return 0x0;
    }
  } while( true );
}



void mem_op_1000_179c(i16 param_1,astruct_57 *param_2)

{
  uchar *puVar1;
  u8 *puVar2;

  puVar1 = PTR_LOOP_1050_5f2c;
  puVar2 = PTR_LOOP_1050_5f2e;
  if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a((StructD *)param_2);
    puVar2 = (u8 *)param_2;
  }
  fn_ptr_op_1000_1708(param_1,0x0,0x0,(u16)puVar1,(u16)puVar2);
  return;
}



void fn_ptr_1000_17ce(char *param_1)

{
  if (param_1 != NULL) {
    call_fn_ptr_1000_0dc6(param_1);
  }
  return;
}



uchar * pass1_1000_17e8(uchar *param_1,uchar *param_2)

{
  uchar *puVar1;

  puVar1 = PTR_LOOP_1050_5f34;
  PTR_LOOP_1050_5f34 = param_1;
  PTR_LOOP_1050_5f36 = param_2;
  return puVar1;
}



u16 pass1_1000_180c(u16 param_1,u16 param_2)

{
  uchar *puVar1;
  u16 in_register_0000000a;
  StructD *pSVar2;
  i32 lVar3;

  pSVar2 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(pSVar2);
    if (((u16)pSVar2 | (u16)puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar3 = mem_op_1000_0a48(0x0,param_2,0x0,CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c));
  return (u16)lVar3;
}



u16 pass1_1000_183c(u16 param_1,u16 param_2)

{
  u32 in_EDX;
  StructD *pSVar1;
  i32 lVar2;

  pSVar1 = (StructD *)(in_EDX & 0xffff0000);
  if ((int)((u32)param_2 * (u32)param_1 >> 0x10) != 0x0) {
    return 0x0;
  }
  if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar1);
    PTR_LOOP_1050_5f2e = (u8 *)pSVar1;
    if (((u16)PTR_LOOP_1050_5f2e | (u16)PTR_LOOP_1050_5f2c) == 0x0) {
      return 0x0;
    }
  }
  lVar2 = mem_op_1000_0a48(0x1,(u16)((u32)param_2 * (u32)param_1),0x0,
                           CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c));
  return (u16)lVar2;
}



u16 pass1_1000_188e(u16 param_1,u16 *param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;
  i32 lVar2;

  if ((param_3 | (u16)param_2) == 0x0) {
    uVar1 = pass1_1000_180c(param_1,param_4);
    return uVar1;
  }
  if (param_4 == 0x0) {
    pass1_1000_18d2((u16)param_2,param_3);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(0x0,param_4,0x0,(astruct_172 *)param_2,(astruct_172 *)param_3);
  return (u16)lVar2;
}



void pass1_1000_18d2(u16 param_1,u16 param_2)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6((char *)CONCAT22(param_2,param_1));
  }
  return;
}



u32 mem_op_1000_18ec(u16 param_1,u16 param_2)

{
  u32 uVar1;

  uVar1 = mem_op_1000_1902(param_2,param_1,0x0,0x0,0xc);
  return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 mem_op_1000_1902(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 *pUVar1;
  u16 UVar2;
  BOOL16 BVar3;
  i16 iVar4;
  u16 uVar3;
  u16 UVar5;
  u16 *pUVar6;
  u16 unaff_CS;
  DWORD DVar7;
  u32 uVar8;
  u16 *puVar1;

  UVar5 = param_1;
  if (((param_2 & 0x8000) != 0x0) && (UVar5 = param_1, _SHI_INVOKEERRORHANDLER1 != -0x6f70)) {
    param_2 |= 0x1;
    UVar5 = param_1;
  }
  do {
    uVar3 = UVar5;
    pUVar1 = (u16 *)(param_2 & 0xfffb | 0x1000);
    mem_op_1000_131c((u16)pUVar1,0x100);
    UVar5 = uVar3 | (u16)pUVar1;
    if (UVar5 != 0x0) break;
    UVar2 = pass1_1000_1e61(unaff_CS,0x2,0x0,0x0);
  } while (UVar2 != 0x0);
  if ((uVar3 | (u16)pUVar1) != 0x0) {
    pUVar1[0x17] = (u16)&PTR_PTR_1050_5f1a;
    pUVar1[0x18] = (u16)&DAT_1050_1050;
    pUVar1[0x15] = (u16)PTR_LOOP_1050_5f1e;
    pUVar1[0x16] = (u16)PTR_LOOP_1050_5f20;
    pUVar6 = pUVar1;
    PTR_LOOP_1050_5f1e = (u8 *)pUVar1;
    PTR_LOOP_1050_5f20 = (u8 *)uVar3;
    for (iVar4 = 0x5; iVar4 != 0x0; iVar4 += -0x1) {
      puVar1 = pUVar6;
      pUVar6 = pUVar6 + 0x1;
      *puVar1 = 0x0;
    }
    pUVar1[0x5] = 0x0;
    pUVar1[0x7] = 0x0;
    pUVar1[0x6] = 0x0;
    pUVar1[0x9] = 0x0;
    pUVar1[0x8] = 0x0;
    pUVar1[0xa] = 0xbead;
    pUVar1[0xb] = param_2 & 0xfffd;
    pUVar1[0xc] = 0x0;
    pUVar1[0xd] = 0x2000;
    pUVar1[0xe] = 0x800;
    DVar7 = mem_op_1000_1532((u16)pUVar1,uVar3);
    pUVar1[0xf] = (u16)DVar7;
    pUVar1[0x10] = (u16)(DVar7 >> 0x10);
    pUVar1[0x12] = 0x0;
    pUVar1[0x11] = 0x0;
    pUVar1[0x13] = 0xfffe;
    pUVar1[0x14] = 0xffff;
    pUVar1[0x19] = 0x0;
    pUVar1[0x1a] = 0x0;
    pUVar1[0x20] = 0x0;
    pUVar1[0x1f] = 0x0;
    BVar3 = pass1_1000_1afe(param_5,(u16)pUVar1,uVar3);
    if (BVar3 != 0x0) {
      if ((param_4 | param_3) != 0x0) {
        pUVar6 = pUVar1;
        UVar5 = uVar3;
        uVar8 = pass1_1000_52be(param_3,param_4,param_5,0x0);
        pass1_1000_010c(0x1,(u16)uVar8,(u16)(uVar8 >> 0x10),(u16)pUVar6,UVar5);
      }
      return CONCAT22(uVar3,pUVar1);
    }
    mem_op_1000_1b9a(0x0,(u16)pUVar1,uVar3);
  }
  return 0x0;
}



u16 pass1_1000_1a54(u16 param_1,i16 param_2,u16 param_3)

{
  u16 uVar1;
  u16 uVar2;
  u16 unaff_CS;

  if ((param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
    return 0x0;
  }
  uVar1 = pass1_1000_1ab0(param_1);
  if (uVar1 < (param_2 + 0x18) + 0x14U) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = (param_2 + 0x1a);
    (param_2 + 0x1a) = uVar1;
    (param_2 + 0x1c) = uVar1 >> 0x2;
  }
  return uVar2;
}



u16 pass1_1000_1ab0(u16 param_1)

{
  u16 uVar1;
  u16 uVar2;

  if (param_1 == 0x2000) {
    return 0x2000;
  }
  if (param_1 < 0xfff0) {
    if (param_1 < 0x1001) {
      return 0x1000;
    }
    uVar1 = 0x2000;
    if (param_1 < 0x2001) {
      do {
        uVar2 = uVar1;
        uVar1 = uVar2 >> 0x1;
      } while (param_1 <= uVar1);
      return uVar2 & 0xfffe;
    }
    while (uVar1 *= 0x2, uVar1 != 0x0) {
      if (param_1 <= uVar1) {
        return (uVar1 + 0x10 & -(u16)(uVar1 < 0xfff0)) - 0x10;
      }
    }
  }
  return 0xfff0;
}



BOOL16 pass1_1000_1afe(u16 param_1,u16 param_2,u16 param_3)

{
  u16 uVar1;
  u16 unaff_CS;

  if (param_1 == 0x0) {
    uVar1 = 0x0;
  }
  else {
    uVar1 = param_1 + 0x1 & 0xfffe;
  }
  if ((param_2 + 0x14) == -0x4153) {
    if ((uVar1 < param_1) || ((param_2 + 0x1a) - 0x14U < uVar1)) {
      pass1_1000_1e61(unaff_CS,0x3,param_2,param_3);
    }
    else if ((param_2 + 0x2) == 0x0) {
      (param_2 + 0x18) = uVar1;
      return 0x1;
    }
    return 0x0;
  }
  pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
  return 0x0;
}



u32 mem_op_1000_1b68(u16 param_1,u16 param_2,u16 param_3)

{
  u16 unaff_CS;
  u32 uVar1;

  if ((param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
    return (u32)param_1 << 0x10;
  }
  uVar1 = mem_op_1000_1b9a(0x0,param_2,param_3);
  return uVar1;
}



u32 mem_op_1000_1b9a(u16 param_1,u16 param_2,u16 param_3)

{
  u16 uVar1;
  u16 uVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  i32 lVar7;
  u16 *puStack8;
  u16 uStack4;

  (param_2 + 0x14) = 0x0;
  uStack4 = 0x0;
  do {
    iVar6 = (uStack4 * 0x2);
    if (iVar6 != 0x0) {
      do {
        uVar3 = *(u32 *)(iVar6 + 0x8);
        ((int)uVar3 + 0xc) = 0x0;
        mem_op_1000_13ce((iVar6 + 0x8),(iVar6 + 0xa));
        iVar6 = (iVar6 + 0x4);
      } while ((uStack4 * 0x2) != iVar6);
    }
    uStack4 += 0x1;
  } while (uStack4 < 0x5);
  uVar4 = (param_2 + 0x10);
  uVar5 = (param_2 + 0x12);
  while( true ) {
    puStack8 = (u16 *)CONCAT22(uVar5,uVar4);
    if ((uVar5 | uVar4) == 0x0) break;
    uVar1 = *puStack8;
    uVar2 = (uVar4 + 0x2);
    mem_op_1000_13ce(uVar4,uVar5);
    uVar4 = uVar1;
    uVar5 = uVar2;
  }
  pass1_1000_20a2(param_2,param_3);
  lVar7 = mem_op_1000_13ce(param_2,param_3);
  return CONCAT22((int)((u32)lVar7 >> 0x10),0x1);
}



BOOL16 mem_op_1000_1dfa(i16 param_1,u8 param_2,u16 param_3,u16 param_4)

{
  undefined3 uVar1;
  u16 uVar2;

  if ((param_2 & 0x4) == 0x0) {
    uVar2 = (u16)(u8)(((u8)(-(u16)((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
  }
  else {
    uVar2 = 0x1800;
  }
  if ((param_4 == 0x0) ||
     ((param_4 & 0xff00 & (u16)(u8)(((u8)(-(u16)((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8) != uVar2)
     ) {
    return 0x1;
  }
  if (param_1 != 0x0) {
    uVar1 = SegmentLimit(param_4);
    if (CARRY2(param_3,param_1 - 0x1U)) {
      return 0x1;
    }
    if ((u16)uVar1 < param_3 + (param_1 - 0x1U)) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1000_1e61(u16 param_1, u16 param_2, astruct_7 *param_3, u16 param_4)

{
  i16 iVar1;
  BOOL16 BVar2;
  u16 UVar3;
  u16 UStack64;
  u16 UStack62;
  u16 UStack60;
  code *pcStack6;
  u8 *puStack4;
  u16 uVar3;

  uVar3 = (u16)&DAT_1050_1050;
  UStack62 = param_3;
  UStack60 = param_4;
  UStack64 = param_2;
  puStack4 = (u8 *)&DAT_1050_1050;
  pcStack6 = (code *)&PTR_PTR_1050_5f1a;
  if (((u16)PTR_LOOP_1050_5f1c | (u16)PTR_PTR_1050_5f1a) == 0x0) {
    pcStack6 = NULL;
    puStack4 = NULL;
  }
  else {
    iVar1 = mem_op_1000_21b6((u16)PTR_PTR_1050_5f1a,(u16)PTR_LOOP_1050_5f1c);
    pcStack6 = (code *)PTR_PTR_1050_5f1a;
    puStack4 = PTR_LOOP_1050_5f1c;
    if (iVar1 == 0x0) {
      PTR_PTR_1050_5f1a = (u8 *)&PTR_PTR_1050_1f7e;
      PTR_LOOP_1050_5f1c = (u8 *)&PTR_LOOP_1050_1000;
      pcStack6 = (code *)&PTR_PTR_1050_1f7e;
      puStack4 = (u8 *)&PTR_LOOP_1050_1000;
    }
  }
  if (((u16)puStack4 | (u16)pcStack6) == 0x0) {
    return 0x0;
  }
  BVar2 = msg_box_op_1000_1f24((int)&PTR_PTR_1050_5f1a,(u16)&DAT_1050_1050,0x0);
  if (BVar2 == 0x0) {
    UVar3 = (*pcStack6)(0x1000,&UStack64,(int)&DAT_1050_1050,uVar3);
  }
  else {
    puStack4 = NULL;
    pcStack6 = NULL;
    UVar3 = 0x0;
  }
  if (((u16)puStack4 | (u16)pcStack6) != 0x0) {
    pass1_1000_1f68();
  }
  return UVar3;
}



u16 _SHI_INVOKEERRORHANDLER1(void)

{
  i16 iVar1;
  BOOL16 BVar2;
  u16 uVar2;
  code *pcStack6;
  u16 *puStack4;
  u16 uVar3;

  puStack4 = (u16 *)&DAT_1050_1050;
  if (((u16)PTR_LOOP_1050_5f1c | (u16)PTR_PTR_1050_5f1a) == 0x0) {
    pcStack6 = NULL;
    puStack4 = NULL;
  }
  else {
    iVar1 = mem_op_1000_21b6((u16)PTR_PTR_1050_5f1a,(u16)PTR_LOOP_1050_5f1c);
    pcStack6 = (code *)PTR_PTR_1050_5f1a;
    puStack4 = PTR_LOOP_1050_5f1c;
    if (iVar1 == 0x0) {
      PTR_PTR_1050_5f1a = (u8 *)&PTR_PTR_1050_1f7e;
      PTR_LOOP_1050_5f1c = (u8 *)&PTR_LOOP_1050_1000;
      pcStack6 = (code *)&PTR_PTR_1050_1f7e;
      puStack4 = (u16 *)&PTR_LOOP_1050_1000;
    }
  }
  if (((u16)puStack4 | (u16)pcStack6) != 0x0) {
    BVar2 = msg_box_op_1000_1f24((int)&PTR_PTR_1050_5f1a,(u16)&DAT_1050_1050,0x0);
    if (BVar2 == 0x0) {
      uVar2 = (*pcStack6)();
    }
    else {
      puStack4 = NULL;
      pcStack6 = NULL;
      uVar2 = 0x0;
    }
    if (((u16)puStack4 | (u16)pcStack6) != 0x0) {
      pass1_1000_1f68();
    }
    return uVar2;
  }
  return 0x0;
}



BOOL16 msg_box_op_1000_1f24(i16 param_1,u16 param_2,u16 param_3)

{
  i16 *piVar1;
  u16 unaff_CS;

  if (param_3 < (param_1 + 0xc)) {
    msg_box_op_1000_214c(0x0,0x0,0xd940,(u16)&PTR_LOOP_1050_1040);
    return 0x1;
  }
  piVar1 = (i16 *)(param_1 + 0xc);
  *piVar1 = *piVar1 + 0x1;
  return 0x0;
}



void pass1_1000_1f68(void)

{
  PTR_LOOP_1050_5f26 = PTR_LOOP_1050_5f26 + -0x1;
  if ((int)PTR_LOOP_1050_5f26 < 0x0) {
    PTR_LOOP_1050_5f26 = NULL;
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10001f92)

BOOL16 pass1_1000_1f7e(u16 *param_1)

{
  char cVar1;
  BOOL16 BVar2;
  u16 uVar3;
  i16 iVar4;
  char *pcVar5;

  uVar3 = *param_1;
  if (uVar3 == 0xf) {
LAB_1000_1fb6:
    iVar4 = 0x1;
  }
  else {
    if (uVar3 < 0x10) {
      cVar1 = (char)uVar3;
      if (cVar1 == '\x02') goto LAB_1000_1fb6;
      if (('\0' < (char)(cVar1 + -0x2)) && ((char)(cVar1 + -0x3) < '\f')) {
        iVar4 = 0x0;
        goto LAB_1000_1fbe;
      }
    }
    iVar4 = 0x0;
    uVar3 = 0x1;
  }
LAB_1000_1fbe:
  pcVar5 = pass1_1000_1fd2(uVar3);
  BVar2 = msg_box_op_1000_214c(0x0,iVar4,(u16)pcVar5,(u16)((u32)pcVar5 >> 0x10));
  return BVar2;
}



char * pass1_1000_1fd2(i16 param_1)

{
  if (param_1 == 0x2) {
    return "Out of memory.  Please free some memory, then choose retry.";
  }
  return (char *)CONCAT22(0x1000,param_1 * 0x17 + 0x1c7a);
}



// WARNING: Removing unreachable block (ram,0x10002018)

BOOL16 pass1_1000_1fea(void)

{
  u8 *puVar1;
  bool bVar2;

  puVar1 = PTR_LOOP_1050_5f22 + 0x1;
  bVar2 = PTR_LOOP_1050_5f22 == NULL;
  PTR_LOOP_1050_5f22 = puVar1;
  if ((bVar2) && (((u16)PTR_LOOP_1050_5f20 | (u16)PTR_LOOP_1050_5f1e) != 0x0)) {
    PTR_LOOP_1050_5f22 = (u8 *)&u16_1050_0002;
  }
  return 0x1;
}



void pass1_1000_201c(i16 param_1,i16 param_2)

{
  u16 uVar1;
  u32 uVar2;
  u16 uVar3;
  BOOL16 BVar4;
  i16 iVar5;
  u16 uVar6;

  if (param_1 == 0x0) {
    (param_2 + 0x6) = 0x0;
    (param_2 + 0x4) = 0x0;
  }
  uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
  while (uVar3 != 0x0) {
    BVar4 = pass1_1000_206c((param_2 + 0x4),(param_2 + 0x6));
    if (BVar4 == 0x0) {
      uVar2 = *(u32 *)(param_2 + 0x4);
      uVar6 = (u16)((u32)uVar2 >> 0x10);
      iVar5 = (int)uVar2;
      uVar1 = (iVar5 + 0x2c);
      (param_2 + 0x4) = (iVar5 + 0x2a);
      (param_2 + 0x6) = uVar1;
    }
    else {
      mem_op_1000_1b9a(0x1,*(u16 *)(param_2 + 0x4),(param_2 + 0x6));
    }
    uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
  }
  return;
}



BOOL16 pass1_1000_206c(u16 param_1,u16 param_2)

{
  u16 uVar1;

  uVar1 = pass1_1000_21d2(0x2,0x42,param_1,param_2,0x1);
  if ((uVar1 != 0x0) && ((param_1 + 0x14) == -0x4153)) {
    return 0x1;
  }
  return 0x0;
}



void pass1_1000_20a2(u16 param_1,u16 param_2)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uStack8;
  u16 uStack4;

  iVar1 = (param_1 + 0x2e);
  uVar2 = (param_1 + 0x30);
  uStack8 = 0x0;
  uVar3 = (iVar1 + 0x4);
  uStack4 = (iVar1 + 0x6);
  uVar7 = 0x0;
  if ((uStack4 | uVar3) != 0x0) {
    while ((uVar6 = uVar3, uVar4 = uStack4, uVar6 != param_1 || (uStack4 != param_2))) {
      uVar3 = (uVar6 + 0x2a);
      uStack4 = (uVar6 + 0x2c);
      uVar7 = uVar6;
      uStack8 = uVar4;
      if ((uStack4 | uVar3) == 0x0) {
        return;
      }
    }
    if ((uStack8 | uVar7) != 0x0) {
      uVar2 = (uVar6 + 0x2c);
      (uVar7 + 0x2a) = (uVar6 + 0x2a);
      (uVar7 + 0x2c) = uVar2;
      return;
    }
    uVar5 = (uVar6 + 0x2c);
    (iVar1 + 0x4) = (uVar6 + 0x2a);
    (iVar1 + 0x6) = uVar5;
  }
  return;
}



u16 ret_true_1000_2146(void)

{
  return 0x1;
}



void empty_fn_1000_214a(void)

{
  return;
}



BOOL16 msg_box_op_1000_214c(u16 param_1,i16 param_2,u16 param_3,u16 param_4)

{
  INT16 IVar1;
  i16 iVar2;
  u16 type;

  type = 0x2 - (param_2 == 0x0) | 0x2110;
  MessageBeep16(0x0);
  do {
    IVar1 = MessageBox16(type,"SmartHeap Library",(char *)CONCAT22(param_4,param_3),0x0);
    iVar2 = IVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x0;
    }
    if ((0x0 < iVar2) && (!SBORROW2(iVar2,0x1))) {
      if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
        fatal_app_exit_1000_3e9e();
        return 0x0;
      }
      if (IVar1 == 0x4) {
        return 0x1;
      }
      if (IVar1 == 0x5) {
        return 0x0;
      }
    }
    if ((type & 0x2000) == 0x0) {
      return 0x0;
    }
    type = type & 0xdfef | 0x1010;
  } while( true );
}



bool mem_op_1000_21b6(u16 param_1,u16 param_2)

{
  BOOL16 BVar1;

  BVar1 = mem_op_1000_1dfa(0x0,0x4,param_1,param_2);
  return BVar1 == 0x0;
}



// WARNING: Removing unreachable block (ram,0x100021de)

u16 pass1_1000_21d2(u8 param_1,i32 param_2,u16 param_3,u16 param_4,u8 param_5)

{
  undefined3 uVar1;
  BOOL16 BVar2;

  BVar2 = mem_op_1000_1dfa(0x0,param_1,param_3,param_4);
  if (BVar2 == 0x0) {
    if ((param_1 & 0x4) == 0x0) {
      uVar1 = SegmentLimit((u32)param_4);
      if ((bool)((u8)((u163)uVar1 >> 0x10) & 0x1)) {
        if (param_2 == 0x0) {
          return 0x1;
        }
        if ((!CARRY4((u32)param_3,param_2 - 0x1U)) && ((u32)param_3 + (param_2 - 0x1U) <= (u32)(u16)uVar1)) {
          return 0x1;
        }
      }
    }
    else {
      BVar2 = pass1_1000_22c0((u16)param_2,_param_1,param_2._2_2_,param_3,param_4);
      if (BVar2 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



u32 pass1_1000_2242(u16 param_1,u8 *param_2,u16 param_3,u16 param_4,u16 param_5,i16 param_6)

{
  u16 uVar1;
  u16 uVar2;
  bool bVar3;

  uVar1 = param_4 | param_3;
  while( true ) {
    if (uVar1 == 0x0) {
      return 0x0;
    }
    uVar1 = param_3;
    if (param_4 != 0x0) {
      uVar1 = 0xffff;
    }
    if (CARRY2(param_5,uVar1) != false) {
      uVar1 = -param_5;
    }
    bVar3 = param_3 < uVar1;
    param_3 -= uVar1;
    param_4 -= bVar3;
    uVar2 = (*(code *)param_2)(uVar1,param_1,param_5,param_6);
    if (uVar2 != 0x0) break;
    bVar3 = CARRY2(param_5,uVar1);
    param_5 += uVar1;
    param_6 += (u16)bVar3 * 0x100;
    uVar1 = param_4 | param_3;
  }
  return CONCAT22(param_4 + CARRY2(uVar2,param_3),uVar2 + param_3);
}



BOOL16 pass1_1000_22c0(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u32 uVar1;

  uVar1 = pass1_1000_2242(param_2,(u8 *)0x1dfa,param_1,param_3,param_4,param_5);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000234c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 * entry(u16 param_1,u16 param_2,i16 param_3,u8 *param_4,u8 *param_5,u16 param_6)

{
  struct astruct_822 *paVar1;
  i16 *piVar2;
  code *pcVar3;
  u16 uVar4;
  struct astruct_822 *pcVar4;
  i16 iVar5;
  i16 *piVar4;
  i16 iVar6;
  u8 *unaff_SI;
  i16 *piVar7;
  HINSTANCE16 unaff_DI;
  struct astruct_822 *paVar8;
  char *unaff_SS;
  bool bVar9;
  u32 DVar10;
  u32 uVar11;
  u32 uVar10;
  u32 uVar12;
  struct astruct_825 *paVar13;
  i16 *piVar1;

  uVar11 = CONCAT22(param_6,PTR_LOOP_1050_5f84);
  do {
    InitTask16(NULL);
    PTR_LOOP_1050_5f84 = (u8 *)uVar11;
    if (param_3 != 0x0) {
      PTR_LOOP_1050_5f7e = (u8 *)&DAT_1050_1050;
      bVar9 = param_5 < (u8 *)0xff00;
      param_5 = param_5 + 0x100;
      if (bVar9) {
        PTR_LOOP_1050_5f50 = (u8 *)&DAT_1050_1050;
        PTR_LOOP_1050_5f48 = param_5;
        PTR_LOOP_1050_5f4a = unaff_SI;
        HINSTANCE16_1050_5f4c = unaff_DI;
        PTR_LOOP_1050_5f4e = param_4;
        LockSegment16(0xffff);
        PTR_LOOP_1050_5f52 = (u8 *)(uVar11 >> 0x10);
        PTR_LOOP_1050_5f84 = (u8 *)uVar11;
        DVar10 = GetVersion16();
        PTR_LOOP_1050_5f52 = (u8 *)(uVar11 >> 0x10);
        PTR_LOOP_1050_5f84 = (u8 *)uVar11;
        PTR_LOOP_1050_5f80 = (u8 *)CONCAT11((char)DVar10,(char)(DVar10 >> 0x8));
        pcVar3 = (code *)swi(0x21);
        uVar12 = uVar11;
        uVar11 = (*pcVar3)();
        PTR_LOOP_1050_5f52 = (u8 *)(uVar12 >> 0x10);
        PTR_LOOP_1050_5f84 = (u8 *)uVar12;
        _DAT_1050_5f82 = CONCAT11((char)uVar11,(char)(uVar11 >> 0x8));
        DAT_1050_5f87 = 0x0;
        WaitEvent16(0x0);
        PTR_LOOP_1050_5f84 = (u8 *)uVar11;
        param_3 = InitApp16(HINSTANCE16_1050_5f4c);
        PTR_LOOP_1050_5f84 = (u8 *)uVar11;
        if (param_3 != 0x0) break;
      }
    }
    param_3 = CONCAT11((char)((u16)param_3 >> 0x8),0xff);
    pass1_1000_24db(param_3);
    PTR_LOOP_1050_5f84 = (u8 *)uVar11;
  } while( true );
  dos3_call_1000_23ea((u16)param_4,(u16)&DAT_1050_1050,0x0,(u16)&DAT_1050_1050);
  PTR_LOOP_1050_5f84 = (u8 *)uVar11;
  pass1_1000_262c((int)(uVar11 >> 0x10),(u8 *)0x238d,(u8 *)s_tile2_bmp_1050_1538);
  PTR_LOOP_1050_5f84 = (u8 *)uVar11;
  pass1_1000_27d6((u16)(uVar11 >> 0x10));
  uVar10 = ret_op_1000_55ac();
  uVar4 = (u16)uVar10;
  init_1000_23be((u16)param_5,(u16)(uVar10 >> 0x10));
  fn_ptr_op_1000_24cd(uVar4);
  paVar13 = (astruct_825 *)CONCAT22(uVar4,0x15);
  pass1_1000_25a8();
  pass1_1000_2913(0x15);
  pcVar4 = (astruct_822 *)poss_str_op_1000_28dc(paVar13);
  if (pcVar4 != NULL) {
    iVar5 = 0x9;
    if (pcVar4->field0_0x0 == 'M') {
      iVar5 = 0xf;
    }
    pcVar4 = pcVar4 + iVar5;
    iVar6 = 0x22;
    paVar8 = pcVar4;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      paVar1 = paVar8;
      paVar8 = paVar8 + 0x1;
    } while (paVar1->field0_0x0 != '\r');
    (paVar8 + -0x1)->field0_0x0 = '\0';
  }
  FatalAppExit16((char *)CONCAT22(0x1050,pcVar4),0x0);
  FatalExit();
  piVar7 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar7;
    piVar7 = piVar7 + 0x1;
    piVar4 = piVar7;
    if ((*piVar1 == param_2) || (piVar4 = (i16 *)(*piVar1 + 0x1), piVar4 == NULL)) {
      return piVar4;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      piVar2 = piVar7;
      piVar7 = (i16 *)((int)piVar7 + 0x1);
    } while (*(char *)piVar2 != '\0');
  } while( true );
}



void init_1000_23be(u16 param_1,u16 param_2)

{
  init_op_1008_54aa((u16)&DAT_1050_1050,param_1,param_2,PTR_LOOP_1050_5f52,
                    CONCAT22(PTR_LOOP_1050_5f50,PTR_LOOP_1050_5f4e),PTR_LOOP_1050_5f4a,(uchar *)HINSTANCE16_1050_5f4c);
  return;
}


/*
Unable to decompile 'dos3_call_1000_23ea'
Cause:
Low-level Error: Symbol $$undef0000001e extends beyond the end of the address space
*/


// WARNING: Removing unreachable block (ram,0x10002557)

void fn_ptr_op_1000_24cd(u16 param_1)

{
  code *pcVar1;
  i16 iVar2;
  u16 uVar2;
  u16 uVar6;
  u16 uVar5;
  u16 uVar3;
  u16_t uVar4;

  u8_1050_5fc9 = '\0';
  fn_ptr_op_1000_2594();
  fn_ptr_op_1000_2594();
  ret_op_1000_55ac();
  fn_ptr_op_1000_2594();
  fn_ptr_op_1000_2594();
  dos3_op_1000_256b();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)

void pass1_1000_24db(u16 param_1)

{
  code *pcVar1;
  i16 unaff_BP;
  i16 iVar2;

  iVar2 = unaff_BP + 0x1;
  u8_1050_5fc9 = '\0';
  fn_ptr_op_1000_2594();
  fn_ptr_op_1000_2594();
  dos3_op_1000_256b();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)(iVar2);
  return;
}



// WARNING: Removing unreachable block (ram,0x10002589)

void dos3_op_1000_256b(void)

{
  code *pcVar1;

  if (PTR_LOOP_1050_6202 != NULL) {
    (*(code *)PTR_LOOP_1050_6200)();
  }
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



void fn_ptr_op_1000_2594(void)

{
  code **ppcVar1;
  code **unaff_SI;
  code **unaff_DI;
  code **ppcVar2;
  code **fn_ptr_1;

  while (unaff_SI < unaff_DI) {
    ppcVar2 = unaff_DI + -0x2;
    ppcVar1 = unaff_DI + -0x1;
    unaff_DI = ppcVar2;
    if (((u16)*ppcVar2 | (u16)*ppcVar1) != 0x0) {
      fn_ptr_1 = ppcVar2;
      (**fn_ptr_1)();
    }
  }
  return;
}



void pass1_1000_25a8(void)

{
  pass1_1000_2913(0xfc);
  pass1_1000_2913(0xff);
  return;
}


/*
Unable to decompile 'exit_1000_25cc'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/


i16 * pass1_1000_25d2(i16 param_1,i16 param_2,u8 *param_3,i16 param_4)

{
  i16 *piVar1;
  char *pcVar2;
  u8 *puVar3;
  StructD *pSVar4;
  i16 *piVar5;
  char *pcVar6;
  i16 iVar7;
  i16 *piVar8;
  char *pcVar9;
  struct astruct_825 *paVar10;

  puVar3 = (u8 *)(param_1 + 0x1U & 0xfffe);
  if ((puVar3 < &param_2) && (pSVar4 = (StructD *)-((int)puVar3 - (int)&param_2), PTR_LOOP_1050_000a <= pSVar4)) {
    if (pSVar4 < PTR_LOOP_1050_000c) {
      PTR_LOOP_1050_000c = pSVar4;
    }
    // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
    // WARNING: Treating indirect jump as call
    piVar5 = (i16 *)(*(code *)param_3)();
    return piVar5;
  }
  paVar10 = (astruct_825 *)((u32)(u16)param_2 << 0x10);
  iVar7 = 0x0;
  pass1_1000_25a8();
  pass1_1000_2913(iVar7);
  pcVar6 = poss_str_op_1000_28dc(paVar10);
  if (pcVar6 != NULL) {
    iVar7 = 0x9;
    if (*pcVar6 == 'M') {
      iVar7 = 0xf;
    }
    pcVar6 = pcVar6 + iVar7;
    iVar7 = 0x22;
    pcVar9 = pcVar6;
    do {
      if (iVar7 == 0x0) break;
      iVar7 += -0x1;
      pcVar2 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar9[-0x1] = '\0';
  }
  FatalAppExit16((char *)CONCAT22(0x1050,pcVar6),0x0);
  FatalExit();
  piVar5 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar1;
    piVar8 = piVar5;
    if ((iVar7 == param_4) || (piVar8 = (i16 *)(iVar7 + 0x1), piVar8 == NULL)) {
      return piVar8;
    }
    iVar7 = -0x1;
    do {
      if (iVar7 == 0x0) break;
      iVar7 += -0x1;
      piVar1 = piVar5;
      piVar5 = (i16 *)((int)piVar5 + 0x1);
    } while (*(char *)piVar1 != '\0');
  } while( true );
}


/*
Unable to decompile 'exit_1000_25f2'
Cause:
Low-level Error: Overlapping input varnodes
*/


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

void pass1_1000_262c(u8 *param_1,u8 *param_2,u8 *param_3)

{
  char *pcVar1;
  char cVar2;
  i16 iVar3;
  i16 iVar5;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  i16 iVar4;
  char **ppcVar6;
  char *pcVar7;
  char *pcVar8;
  char *pcVar9;
  u16 unaff_ES;
  u16 uVar12;
  u16 uVar3;
  u8 *puVar3;

  PTR_LOOP_1050_5fd2 = param_2;
  PTR_LOOP_1050_5fd4 = param_3;
  param_3 = (u8 *)0x263d;
  param_2 = (u8 *)pass1_1000_2950(0x8,param_1,0x104);
  param_3 = param_1;
  PTR_LOOP_1050_5fc2 = param_2;
  PTR_LOOP_1050_5fc4 = param_1;
  iVar5 = GetModuleFileName16(0x104,(char *)CONCAT22(param_1,param_2),HINSTANCE16_1050_5f4c);
  param_2[iVar5] = '\0';
  iVar4 = 0x1;
  PTR_LOOP_1050_5fb8 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
  pcVar7 = (char *)((int)s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
LAB_1000_266c:
  do {
    do {
      pcVar1 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == ' ');
  } while (cVar2 == '\t');
  if ((cVar2 != '\r') && (cVar2 != '\0')) {
    PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 0x1;
    do {
      pcVar7 = pcVar7 + -0x1;
LAB_1000_267f:
      pcVar1 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
      cVar2 = *pcVar1;
      if ((cVar2 == ' ') || (cVar2 == '\t')) goto LAB_1000_266c;
      if ((cVar2 == '\r') || (cVar2 == '\0')) break;
      if (cVar2 == '\"') {
LAB_1000_26b8:
        do {
          while( true ) {
            while( true ) {
              pcVar1 = pcVar7;
              pcVar7 = pcVar7 + 0x1;
              cVar2 = *pcVar1;
              if ((cVar2 == '\r') || (cVar2 == '\0')) goto LAB_1000_26e8;
              if (cVar2 == '\"') goto LAB_1000_267f;
              if (cVar2 == '\\') break;
              iVar4 += 0x1;
            }
            uVar7 = 0x0;
            do {
              pcVar9 = pcVar7;
              uVar7 += 0x1;
              pcVar7 = pcVar9 + 0x1;
              cVar2 = *pcVar9;
            } while (cVar2 == '\\');
            if (cVar2 == '\"') break;
            iVar4 += uVar7;
            pcVar7 = pcVar9;
          }
          iVar4 = iVar4 + (uVar7 >> 0x1) + (u16)((uVar7 & 0x1) != 0x0);
        } while ((uVar7 & 0x1) != 0x0);
        goto LAB_1000_267f;
      }
      if (cVar2 != '\\') {
        iVar4 += 0x1;
        goto LAB_1000_267f;
      }
      uVar6 = 0x0;
      do {
        uVar6 += 0x1;
        pcVar1 = pcVar7;
        pcVar7 = pcVar7 + 0x1;
        cVar2 = *pcVar1;
      } while (cVar2 == '\\');
      if (cVar2 == '\"') {
        iVar4 = iVar4 + (uVar6 >> 0x1) + (u16)((uVar6 & 0x1) != 0x0);
        if ((uVar6 & 0x1) == 0x0) goto LAB_1000_26b8;
        goto LAB_1000_267f;
      }
      iVar4 += uVar6;
    } while( true );
  }
LAB_1000_26e8:
  param_3 = (u8 *)&DAT_1050_1050;
  iVar3 = -((u16)(PTR_LOOP_1050_5fb8 + (int)(PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar4 + 0x1) & 0xfffe);
  PTR_LOOP_1050_5fba = &stack0x0004 + iVar3;
  PTR_LOOP_1050_5fbc = (u8 *)&DAT_1050_1050;
  pcVar9 = &stack0x0004 + (int)(PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar3;
  ((int)&param_3 + iVar3) = (int)&DAT_1050_1050;
  puVar3 = PTR_LOOP_1050_5fc4;
  uVar12 = ((int)&param_3 + iVar3);
  *(u8 **)(&stack0x0004 + iVar3) = PTR_LOOP_1050_5fc2;
  *(u8 **)(&stack0x0006 + iVar3) = puVar3;
  ppcVar6 = (char **)(&stack0x0008 + iVar3);
  *(u8 **)((int)&param_3 + iVar3) = &stack0x0004 + iVar3;
  ((int)&param_2 + iVar3) = (int)s_tile2_bmp_1050_1538;
  (&stack0xfffe + iVar3) = 0x271f;
  uVar4 = pass1_1000_29dc((int)&DAT_1050_1050);
  uVar3 = &PTR_LOOP_1050_5f7e;
  pcVar7 = (char *)((int)s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
LAB_1000_272e:
  do {
    do {
      pcVar1 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == ' ');
  } while (cVar2 == '\t');
  if ((cVar2 == '\r') || (cVar2 == '\0')) {
LAB_1000_27c1:
    ((int)&param_3 + iVar3) = (int)s_tile2_bmp_1050_1538;
    ((int)&param_2 + iVar3) = 0x27c5;
    uVar5 = pass1_1000_29dc((int)&DAT_1050_1050);
    *ppcVar6 = NULL;
    ppcVar6[0x1] = NULL;
    // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
    // WARNING: Treating indirect jump as call
    (*(code *)(u32)&PTR_LOOP_1050_5fd2)();
    _PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4,PTR_LOOP_1050_5fc2);
    return;
  }
  *ppcVar6 = pcVar9;
  ppcVar6[0x1] = (char *)&DAT_1050_1050;
  ppcVar6 = ppcVar6 + 0x2;
  do {
    pcVar7 = pcVar7 + -0x1;
LAB_1000_274f:
    pcVar1 = pcVar7;
    pcVar7 = pcVar7 + 0x1;
    cVar2 = *pcVar1;
    if ((cVar2 == ' ') || (cVar2 == '\t')) {
      pcVar1 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
      *pcVar1 = '\0';
      goto LAB_1000_272e;
    }
    if ((cVar2 == '\r') || (cVar2 == '\0')) {
LAB_1000_27be:
      *pcVar9 = '\0';
      goto LAB_1000_27c1;
    }
    pcVar8 = pcVar7;
    if (cVar2 == '\"') {
LAB_1000_278b:
      while( true ) {
        pcVar7 = pcVar8 + 0x1;
        cVar2 = *pcVar8;
        if ((cVar2 == '\r') || (cVar2 == '\0')) goto LAB_1000_27be;
        if (cVar2 == '\"') break;
        if (cVar2 == '\\') {
          uVar10 = 0x0;
          do {
            pcVar8 = pcVar7;
            uVar10 += 0x1;
            pcVar7 = pcVar8 + 0x1;
            cVar2 = *pcVar8;
          } while (cVar2 == '\\');
          if (cVar2 == '\"') {
            for (uVar11 = uVar10 >> 0x1; uVar11 != 0x0; uVar11 -= 0x1) {
              pcVar1 = pcVar9;
              pcVar9 = pcVar9 + 0x1;
              *pcVar1 = '\\';
            }
            if ((uVar10 & 0x1) == 0x0) break;
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\"';
            pcVar8 = pcVar7;
          }
          else {
            for (; uVar10 != 0x0; uVar10 -= 0x1) {
              pcVar1 = pcVar9;
              pcVar9 = pcVar9 + 0x1;
              *pcVar1 = '\\';
            }
          }
        }
        else {
          pcVar1 = pcVar9;
          pcVar9 = pcVar9 + 0x1;
          *pcVar1 = cVar2;
          pcVar8 = pcVar7;
        }
      }
      goto LAB_1000_274f;
    }
    if (cVar2 != '\\') {
      pcVar1 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
      *pcVar1 = cVar2;
      goto LAB_1000_274f;
    }
    uVar8 = 0x0;
    do {
      uVar8 += 0x1;
      pcVar1 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == '\\');
    if (cVar2 == '\"') {
      for (uVar9 = uVar8 >> 0x1; uVar9 != 0x0; uVar9 -= 0x1) {
        pcVar1 = pcVar9;
        pcVar9 = pcVar9 + 0x1;
        *pcVar1 = '\\';
      }
      pcVar8 = pcVar7;
      if ((uVar8 & 0x1) == 0x0) goto LAB_1000_278b;
      pcVar1 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
      *pcVar1 = '\"';
      goto LAB_1000_274f;
    }
    for (; uVar8 != 0x0; uVar8 -= 0x1) {
      pcVar1 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
      *pcVar1 = '\\';
    }
  } while( true );
}



void pass1_1000_27d6(u16 param_1)

{
  i16 *piVar2;
  char *pcVar3;
  char cVar4;
  u16 *puVar5;
  u16 **ppuVar6;
  i16 iVar7;
  u16 uVar7;
  i16 iVar8;
  u16 *puVar7;
  u16 *puVar8;
  i16 iVar9;
  i16 *piVar9;
  i16 *piVar10;
  char *pcVar11;
  i16 *piVar12;
  bool bVar13;
  void *dos_env;
  u16 *puVar14;
  i16 *piVar1;
  u16 *puVar4;
  i16 *piVar4;

  dos_env = GetDOSEnvironment16();
  puVar7 = (u16 *)((u32)dos_env >> 0x10);
  if ((int)dos_env != 0x0) {
    puVar7 = NULL;
  }
  iVar9 = 0x0;
  pcVar11 = NULL;
  iVar7 = -0x1;
  if (puVar7 != NULL) {
    cVar4 = *NULL;
    while (cVar4 != '\0') {
      do {
        if (iVar7 == 0x0) break;
        iVar7 += -0x1;
        pcVar3 = pcVar11;
        pcVar11 = pcVar11 + 0x1;
      } while (*pcVar3 != '\0');
      iVar9 += 0x1;
      pcVar3 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar4 = *pcVar3;
    }
  }
  uVar7 = 0x9;
  puVar8 = puVar7;
  puVar5 = pass1_1000_2950(0x9,puVar7,(u16)(pcVar11 + 0x1) & 0xfffe);
  puVar14 = puVar8;
  ppuVar6 = (u16 **)pass1_1000_2950(uVar7,puVar8,(iVar9 + 0x1) * 0x4);
  piVar9 = NULL;
  PTR_LOOP_1050_5fbe = (u8 *)ppuVar6;
  PTR_LOOP_1050_5fc0 = (u8 *)puVar8;
  do {
    if (iVar9 == 0x0) {
      *ppuVar6 = NULL;
      ppuVar6[0x1] = NULL;
      return;
    }
    bVar13 = *piVar9 == s__C_FILE_INFO__1050_5f5c._0_2_;
    if (bVar13) {
      piVar12 = (i16 *)s__C_FILE_INFO__1050_5f5c;
      iVar8 = 0x6;
      piVar10 = piVar9;
      do {
        if (iVar8 == 0x0) break;
        iVar8 += -0x1;
        piVar4 = piVar12;
        piVar12 = piVar12 + 0x1;
        piVar1 = piVar10;
        piVar10 = piVar10 + 0x1;
        bVar13 = *piVar1 == *piVar4;
      } while (bVar13);
      if (!bVar13) goto LAB_1000_2867;
    }
    else {
LAB_1000_2867:
      *ppuVar6 = puVar5;
      ppuVar6[0x1] = puVar14;
      ppuVar6 = ppuVar6 + 0x2;
    }
    do {
      piVar2 = piVar9;
      piVar9 = (i16 *)((int)piVar9 + 0x1);
      cVar4 = *(char *)piVar2;
      puVar4 = puVar5;
      puVar5 = (u16 *)((int)puVar5 + 0x1);
      *(char *)puVar4 = cVar4;
    } while (cVar4 != '\0');
    iVar9 += -0x1;
  } while( true );
}



PCHAR poss_str_op_1000_28dc(astruct_825 *param_1)

{
  struct astruct_825 **ppaVar1;
  PCHAR piVar2;
  i16 iVar3;
  PCHAR piVar3;
  struct astruct_825 *iVar2;

  piVar3 = (PCHAR)&PTR_LOOP_1050_63fe;
  do {
    ppaVar1 = (astruct_825 **)piVar3;
    piVar3 = (PCHAR)((int)piVar3 + 0x2);
    iVar2 = *ppaVar1;
    piVar2 = piVar3;
    if ((iVar2 == (astruct_825 *)param_1) || (piVar2 = (PCHAR)(iVar2 + 0x1), piVar2 == NULL)) {
      return (PCHAR)(astruct_825 **)piVar2;
    }
    iVar3 = -0x1;
    do {
      if (iVar3 == 0x0) break;
      iVar3 += -0x1;
      ppaVar1 = (astruct_825 **)piVar3;
      piVar3 = (PCHAR)((int)piVar3 + 0x1);
    } while (*(char *)ppaVar1 != '\0');
  } while( true );
}



void pass1_1000_2913(i16 param_1)

{
  char *pcVar1;
  char *pcVar2;
  i16 iVar3;
  u16 unaff_DI;
  u16 unaff_ES;
  struct astruct_825 *paVar4;
  i16 iVar5;

  iVar5 = (i16)&DAT_1050_1050;
  if (u16_1050_61ec != 0x0) {
    paVar4 = (astruct_825 *)CONCAT22(unaff_DI,param_1);
    pcVar2 = poss_str_op_1000_28dc(paVar4);
    if (pcVar2 != NULL) {
      iVar3 = -0x1;
      do {
        if (iVar3 == 0x0) break;
        iVar3 += -0x1;
        pcVar1 = pcVar2;
        pcVar2 = pcVar2 + 0x1;
      } while (*pcVar1 != '\0');
      pass1_1000_55b1((int)((u32)paVar4 >> 0x10),iVar5);
    }
  }
  return;
}



u16_t * pass1_1000_2950(i16 param_1,u16 param_2,u16 param_3)

{
  u16_t *puVar1;
  u16_t uVar2;
  char *pcVar3;
  u8 *puVar4;
  char *pcVar5;
  i16 iVar6;
  u16_t *puVar7;
  u16_t *puVar8;
  u16_t unaff_BP;
  char *pcVar9;
  u16 unaff_ES;
  struct astruct_825 *paVar10;

  puVar4 = PTR_LOOP_1050_6066;
  PTR_LOOP_1050_6066 = (u8 *)&PTR_LOOP_1050_1000;
  puVar8 = (u16_t *)mem_1000_167a(param_2,param_3);
  PTR_LOOP_1050_6066 = puVar4;
  if ((param_2 | (u16)puVar8) != 0x0) {
    return puVar8;
  }
  paVar10 = (astruct_825 *)CONCAT22(unaff_ES,param_1);
  pass1_1000_25a8();
  pass1_1000_2913(param_1);
  pcVar5 = poss_str_op_1000_28dc(paVar10);
  if (pcVar5 != NULL) {
    iVar6 = 0x9;
    if (*pcVar5 == 'M') {
      iVar6 = 0xf;
    }
    pcVar5 = pcVar5 + iVar6;
    iVar6 = 0x22;
    pcVar9 = pcVar5;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      pcVar3 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar9[-0x1] = '\0';
  }
  FatalAppExit16((char *)CONCAT22(0x1050,pcVar5),0x0);
  FatalExit();
  puVar8 = (u16_t *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar8;
    puVar8 = puVar8 + 0x1;
    uVar2 = *puVar1;
    puVar7 = puVar8;
    if ((uVar2 == unaff_BP) || (puVar7 = (u16_t *)(uVar2 + 0x1), puVar7 == NULL)) {
      return puVar7;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      puVar1 = puVar8;
      puVar8 = (u16_t *)((int)puVar8 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}



void pass1_1000_29af(u16 param_1)

{
  pass1_1000_29b5(param_1 & 0xff);
  return;
}



void pass1_1000_29b5(u16 param_1)

{
  char cVar1;

  PTR_LOOP_1050_5f88._0_1_ = (u8)param_1;
  cVar1 = (char)(param_1 >> 0x8);
  if (cVar1 != '\0') goto LAB_1000_29d2;
  if ((u8)PTR_LOOP_1050_5f88 < 0x22) {
    if ((u8)PTR_LOOP_1050_5f88 < 0x20) {
      if (0x13 < (u8)PTR_LOOP_1050_5f88) goto LAB_1000_29cc;
    }
    else {
      param_1 = 0x5;
    }
  }
  else {
LAB_1000_29cc:
    param_1 = 0x13;
  }
  cVar1 = *(char *)(u32)((param_1 & 0xff) + 0x5fd6);
LAB_1000_29d2:
  PTR_LOOP_1050_5f78 = (u8 *)(int)cVar1;
  return;
}



u16_t pass1_1000_29dc(u16 param_1)

{
  if (___EXPORTEDSTUB != (code)0xb8) {
    return (u16_t)&DAT_1050_1050;
  }
  return uRam100029ed;
}



u16_t ___EXPORTEDSTUB(void)

{
  return 0x0;
}



u16 pass1_1000_2a00(u16 *param_1)

{
  u16 uVar1;
  bool bVar2;
  i16 *piVar3;
  u16 uVar4;
  i16 unaff_BP;
  u16 uVar5;
  u16 unaff_CS;
  u8 *puStack20;
  char local_10;
  u8 uStack15;
  u8 local_e [0x8];
  u16 uStack6;
  u16 local_4;
  i16 iStack2;

  iStack2 = unaff_BP + 0x1;
  local_4 = SUB42(&DAT_1050_1050,0x0);
  uVar5 = 0xffff;
  if ((*(u8 *)(param_1 + 0x5) & 0x40) != 0x0) {
    *(u8 *)(param_1 + 0x5) = 0x0;
    return 0xffff;
  }
  if ((*(u8 *)(param_1 + 0x5) & 0x83) == 0x0) goto LAB_1000_2af2;
  uVar5 = pass1_1000_2fa4((i16 *)param_1);
  uStack6 = param_1[0x7a];
  pass1_1000_2cb0(param_1);
  uVar1 = (u16)*(u8 *)((int)param_1 + 0xb);
  if ((int)u16_1050_5f8a < (int)uVar1) {
    piVar3 = pass1_1000_55b1(unaff_CS,uVar1);
    if ((int)piVar3 < 0x0) goto LAB_1000_2a6a;
LAB_1000_2a82:
    bVar2 = false;
  }
  else {
    uVar4 = dos3_call_op_1000_35fe((u16)*(u8 *)((int)param_1 + 0xb),(i16)&iStack2);
    if (-0x1 < (int)uVar4) goto LAB_1000_2a82;
LAB_1000_2a6a:
    bVar2 = true;
  }
  if (!bVar2) {
    if (uStack6 == 0x0) goto LAB_1000_2af2;
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,&local_10),s___1050_5fea);
    puStack20 = local_e;
    if (local_10 == '\\') {
      puStack20 = &uStack15;
    }
    else {
      pass1_1000_3cea(CONCAT22(0x1050,&local_10),s___1050_5fec);
    }
    pass1_1000_3e82(uStack6,(uchar *)CONCAT22(0x1050,puStack20),0xa);
    uVar4 = dos3_call_1000_514e();
    if (uVar4 == 0x0) goto LAB_1000_2af2;
  }
  uVar5 = 0xffff;
LAB_1000_2af2:
  *(u8 *)(param_1 + 0x5) = 0x0;
  return uVar5;
}



u16 * pass1_1000_2b02(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,u8 param_6)

{
  u16 *puVar1;

  puVar1 = pass1_1000_35aa();
  if ((param_1 | (u16)puVar1) == 0x0) {
    puVar1 = NULL;
  }
  else {
    puVar1 = pass1_1000_2d34(param_2,param_3,(u8 *)CONCAT22(param_5,param_4),param_6,puVar1);
  }
  return puVar1;
}



void pass1_1000_2b3c(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,i16 param_6)

{
  pass1_1000_2b02(param_1,param_2,param_3,param_4,param_5,0x0);
  return;
}



u16 pass1_1000_2b5c(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;
  u16 uVar2;

  uVar1 = pass1_1000_2e74((u16 *)param_1);
  uVar2 = FUN_1000_30b4();
  pass1_1000_2f00(uVar1,(i16 *)param_1);
  return uVar2;
}



void pass1_1000_2ba0(uchar param_1)

{
  pass1_1000_3024();
  if (u8_1050_5fc9 != '\0') {
    pass1_1000_3f5c();
  }
  return;
}



u16 mem_1000_2bb6(u16 param_1,u16 param_2,i16 *param_3)

{
  i16 *piVar1;
  i16 iVar2;
  i16 *piVar3;
  u8 bVar4;
  u8 *puVar5;
  u8 *puVar6;
  u8 *puVar7;

  piVar3 = param_3;
  bVar4 = *(u8 *)(param_3 + 0x5);
  if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
    param_3[0x2] = 0x0;
    if ((bVar4 & 0x1) != 0x0) {
      if ((bVar4 & 0x10) == 0x0) goto LAB_1000_2c37;
      *param_3 = param_3[0x3];
      bVar4 &= 0xfe;
    }
    *(u8 *)(param_3 + 0x5) = bVar4 & 0xef | 0x2;
    puVar7 = (u8 *)(u16)*(u8 *)((int)param_3 + 0xb);
    if (((bVar4 & 0x8) == 0x0) &&
       (((bVar4 & 0x4) != 0x0 ||
        (((*(u8 *)(param_3 + 0x78) & 0x1) == 0x0 &&
         (((u16_1050_61ec != 0x0 &&
           (((param_3 == (i16 *)0x621c || (param_3 == (i16 *)0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0)))) ||
          (mem_1000_2ce8(param_1,param_3), (*(u8 *)(piVar3 + 0x5) & 0x8) == 0x0)))))))) {
      puVar5 = mixed_dos3_call_1000_39f2
                         (puVar7,(char *)CONCAT22(0x1050,&param_2),(u8 *)((int)&PTR_LOOP_1050_0000 + 0x1));
      puVar6 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    else {
      iVar2 = piVar3[0x3];
      puVar6 = (u8 *)(*piVar3 - iVar2);
      *piVar3 = iVar2 + 0x1;
      piVar3[0x2] = piVar3[0x79] + -0x1;
      if (puVar6 == NULL) {
        puVar5 = NULL;
        if ((puVar7[0x5f90] & 0x20) != 0x0) {
          mixed_dos3_call_1000_3636((u16)puVar7,0x0,0x0,0x2);
          puVar5 = NULL;
          puVar6 = puVar5;
        }
      }
      else {
        puVar5 = mixed_dos3_call_1000_39f2(puVar7,(char *)CONCAT22(piVar3[0x4],piVar3[0x3]),puVar6);
      }
      **(u8 **)(piVar3 + 0x3) = (char)param_2;
    }
    if (puVar5 == puVar6) {
      return param_2 & 0xff;
    }
  }
LAB_1000_2c37:
  piVar1 = piVar3 + 0x5;
  *(u8 *)piVar1 = *(u8 *)piVar1 | 0x20;
  return 0xffff;
}



void pass1_1000_2cb0(u16 *param_1)

{
  u16 *puVar1;
  u8 bVar2;

  bVar2 = *(u8 *)(param_1 + 0x5);
  if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
    pass1_1000_16ee(param_1[0x3],param_1[0x4]);
    puVar1 = param_1 + 0x5;
    *(u8 *)puVar1 = *(u8 *)puVar1 & 0xf7;
    param_1[0x3] = 0x0;
    param_1[0x4] = 0x0;
    *param_1 = 0x0;
    param_1[0x1] = 0x0;
    param_1[0x2] = 0x0;
  }
  return;
}



void mem_1000_2ce8(u16 param_1,i16 *param_2)

{
  i16 *piVar1;
  u16 uVar2;

  uVar2 = mem_1000_167a(param_1,0x200);
  if (param_1 == 0x0) {
    piVar1 = param_2 + 0x5;
    *(u8 *)piVar1 = *(u8 *)piVar1 | 0x4;
    param_2[0x79] = 0x1;
    param_1 = (u16)&DAT_1050_1050;
    uVar2 = (int)param_2 + 0xf1;
  }
  else {
    piVar1 = param_2 + 0x5;
    *(u8 *)piVar1 = *(u8 *)piVar1 | 0x8;
    param_2[0x79] = 0x200;
  }
  param_2[0x1] = param_1;
  *param_2 = uVar2;
  param_2[0x4] = param_1;
  param_2[0x3] = uVar2;
  param_2[0x2] = 0x0;
  return;
}



u16 * pass1_1000_2d34(u16 param_1,u16 param_2,u8 *param_3,u8 param_4,u16 *param_5)

{
  u8 bVar1;
  bool bVar2;
  bool bVar3;
  u16 uVar4;
  u16 in_stack_0000ffd8;
  u8 uStack14;
  u8 bStack8;
  u8 uStack6;

  bStack8 = (u8)PTR_LOOP_1050_6062;
  bVar3 = false;
  bVar1 = *param_3;
  if (bVar1 == 0x77) {
    uVar4 = 0x301;
  }
  else {
    if (0x77 < bVar1) {
      return NULL;
    }
    if (bVar1 != 0x61) {
      if (bVar1 != 0x72) {
        return NULL;
      }
      uVar4 = 0x0;
      uStack6 = 0x1;
      goto LAB_1000_2d6c;
    }
    uVar4 = 0x109;
  }
  uStack6 = 0x2;
LAB_1000_2d6c:
  bVar2 = true;
LAB_1000_2d71:
  param_3 = (u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x1));
  if ((*param_3 == 0x0) || (!bVar2)) {
    uVar4 = mixed_dos3_call_1000_370a(in_stack_0000ffd8,param_1,param_2,uVar4,param_4,0x1a4);
    if ((int)uVar4 < 0x0) {
      return NULL;
    }
    PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 0x1;
    *(u8 *)(param_5 + 0x5) = uStack6;
    param_5[0x1] = 0x0;
    *param_5 = 0x0;
    param_5[0x4] = 0x0;
    param_5[0x3] = 0x0;
    uStack14 = (undefined)uVar4;
    *(u8 *)((int)param_5 + 0xb) = uStack14;
    *(u8 *)(param_5 + 0x78) = bStack8;
    param_5[0x2] = 0x0;
    param_5[0x7a] = 0x0;
    return param_5;
  }
  bVar1 = *param_3;
  if (bVar1 == 0x74) {
    if ((uVar4 & 0xc000) == 0x0) {
      uVar4 |= 0x4000;
      goto LAB_1000_2d71;
    }
  }
  else {
    if (0x74 < bVar1) goto LAB_1000_2da4;
    if (bVar1 == 0x2b) {
      if ((uVar4 & 0x2) != 0x0) goto LAB_1000_2da4;
      uVar4 = uVar4 & 0xfffe | 0x2;
      uStack6 = 0x80;
      goto LAB_1000_2d71;
    }
    if (bVar1 == 0x62) {
      if ((uVar4 & 0xc000) == 0x0) {
        uVar4 |= 0x8000;
        goto LAB_1000_2d71;
      }
    }
    else {
      if (bVar1 != 0x63) {
        if ((bVar1 != 0x6e) || (bVar3)) goto LAB_1000_2da4;
        bVar3 = true;
        bStack8 &= 0xbf;
        goto LAB_1000_2d71;
      }
      if (!bVar3) {
        bVar3 = true;
        bStack8 |= 0x40;
        goto LAB_1000_2d71;
      }
    }
  }
LAB_1000_2da4:
  bVar2 = false;
  goto LAB_1000_2d71;
}



u16 pass1_1000_2e74(u16 *param_1)

{
  u16 *puVar1;
  u16 uVar2;
  u16 uVar3;
  u16 *puVar4;
  u16 *puVar5;

  if (u16_1050_61ec != 0x0) {
    puVar5 = param_1 + 0x78;
    puVar4 = (u16 *)0x5ff2;
    if ((param_1 == (u16 *)0x621c) || (puVar4 = (u16 *)&PTR_LOOP_1050_5ff6, param_1 == (u16 *)0x6228)) {
      if (((*(u8 *)(param_1 + 0x5) & 0xc) == 0x0) && ((*(u8 *)puVar5 & 0x1) == 0x0)) {
        uVar2 = *puVar4;
        uVar3 = puVar4[0x1];
        if ((uVar2 | uVar3) == 0x0) {
          uVar2 = mem_1000_167a(uVar3,0x200);
          if (uVar3 == 0x0) {
            return 0x0;
          }
          *puVar4 = uVar2;
          puVar4[0x1] = uVar3;
        }
        param_1[0x3] = uVar2;
        param_1[0x4] = uVar3;
        *param_1 = uVar2;
        param_1[0x1] = uVar3;
        param_1[0x2] = 0x200;
        param_1[0x79] = 0x200;
        puVar1 = param_1 + 0x5;
        *(u8 *)puVar1 = *(u8 *)puVar1 | 0x2;
        *(u8 *)puVar5 = 0x11;
        return 0x1;
      }
    }
    else if ((u8)u16_1050_5f8a <= *(u8 *)((int)param_1 + 0xb)) {
      puVar1 = puVar5;
      *(u8 *)puVar1 = *(u8 *)puVar1 | 0x10;
    }
  }
  return 0x0;
}



void pass1_1000_2f00(i16 param_1,i16 *param_2)

{
  if (((*(u8 *)(param_2 + 0x78) & 0x10) != 0x0) && ((*(u8 *)(*(u8 *)((int)param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)
     ) {
    pass1_1000_2fa4(param_2);
    if (param_1 != 0x0) {
      *(u8 *)(param_2 + 0x78) = 0x0;
      param_2[0x79] = 0x0;
      *param_2 = 0x0;
      param_2[0x1] = 0x0;
      param_2[0x3] = 0x0;
      param_2[0x4] = 0x0;
    }
  }
  return;
}



u16 pass1_1000_2f48(i32 param_1)

{
  u16 uVar1;
  uchar *puVar2;

  if (param_1 == 0x0) {
    uVar1 = pass1_1000_3038(0x0);
  }
  else {
    uVar1 = pass1_1000_2fa4((i16 *)param_1);
    if (uVar1 == 0x0) {
      if ((*(u8 *)((i16 *)param_1 + 0x78) & 0x40) != 0x0) {
        puVar2 = pass1_1000_400a((u16)*(u8 *)((int)(i16 *)param_1 + 0xb));
        uVar1 = -(u16)(puVar2 != NULL);
      }
    }
    else {
      uVar1 = 0xffff;
    }
  }
  return uVar1;
}



u16 pass1_1000_2fa4(i16 *param_1)

{
  i16 *piVar1;
  u8 bVar2;
  i16 iVar3;
  u8 *puVar4;
  u8 *puVar5;
  u16 uVar6;

  uVar6 = 0x0;
  bVar2 = *(u8 *)(param_1 + 0x5);
  if (((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((*(u8 *)(param_1 + 0x78) & 0x1) != 0x0)))) {
    puVar4 = (u8 *)(*param_1 - param_1[0x3]);
    if (0x0 < (int)puVar4) {
      puVar5 = mixed_dos3_call_1000_39f2
                         ((u8 *)(u16)*(u8 *)((int)param_1 + 0xb),(char *)CONCAT22(param_1[0x4],param_1[0x3]),puVar4);
      if (puVar5 == puVar4) {
        if ((*(u8 *)(param_1 + 0x5) & 0x80) != 0x0) {
          piVar1 = param_1 + 0x5;
          *(u8 *)piVar1 = *(u8 *)piVar1 & 0xfd;
        }
      }
      else {
        piVar1 = param_1 + 0x5;
        *(u8 *)piVar1 = *(u8 *)piVar1 | 0x20;
        uVar6 = 0xffff;
      }
    }
  }
  iVar3 = param_1[0x4];
  *param_1 = param_1[0x3];
  param_1[0x1] = iVar3;
  param_1[0x2] = 0x0;
  return uVar6;
}



void pass1_1000_3024(void)

{
  pass1_1000_3038(0x1);
  return;
}



i16 pass1_1000_3038(i16 param_1)

{
  u16 uVar1;
  u8 *puVar2;
  i16 iVar3;
  i16 iStack4;

  iVar3 = 0x0;
  iStack4 = 0x0;
  for (puVar2 = (u8 *)&PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
    if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
      uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2));
      if (uVar1 != 0xffff) {
        iVar3 += 0x1;
      }
    }
    else if ((param_1 == 0x0) &&
            (((puVar2[0xa] & 0x2) != 0x0 && (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2)), uVar1 == 0xffff)))) {
      iStack4 = -0x1;
    }
  }
  if (param_1 == 0x1) {
    iStack4 = iVar3;
  }
  return iStack4;
}



u16 FUN_1000_30b4(void)

{
  u8 bVar1;
  u8 bVar2;
  i16 unaff_BP;
  i16 iVar3;
  u16 unaff_SI;
  u16 unaff_CS;
  u8 *in_stack_00000008;
  u16 uVar4;

  iVar3 = unaff_BP + 0x1;
  uVar4 = SUB42(&DAT_1050_1050,0x0);
  exit_1000_25f2(0x214,0x30c5,unaff_CS,(int)&DAT_1050_1050);
  bVar1 = *in_stack_00000008;
  if (bVar1 == 0x0) {
    return 0x0;
  }
  if ((u8)(bVar1 - 0x20) < 0x59) {
    bVar2 = *(u8 *)(u32)((u8)(bVar1 - 0x20) + 0x5ffe) & 0xf;
  }
  else {
    bVar2 = 0x0;
  }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
  uVar4 = (**(code **)((char)(*(u8 *)(u32)((u8)(bVar2 * '\b') + 0x5ffe) >> 0x4) * 0x2 + 0x30a4))
                    (unaff_SI & 0xff00 | (u16)bVar1,uVar4,iVar3);
  return uVar4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3113(void)

{
  char cVar1;
  char *pcVar2;
  u8 bVar3;
  u16 uVar4;
  i16 unaff_BP;

  FUN_1000_3552();
  pcVar2 = *(char **)(unaff_BP + 0xa);
  cVar1 = *pcVar2;
  (unaff_BP + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(u8 *)(u32)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(u8 *)(u32)((u8)(bVar3 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar3;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_311e(void)

{
  char cVar1;
  char *pcVar2;
  u8 bVar3;
  u16 uVar4;
  i16 unaff_BP;

  (unaff_BP + -0x12) = 0x0;
  (unaff_BP + -0xc) = 0x0;
  (unaff_BP + -0x14) = 0x0;
  (unaff_BP + -0x6) = 0x20;
  (unaff_BP + -0xe) = 0xffff;
  pcVar2 = *(char **)(unaff_BP + 0xa);
  cVar1 = *pcVar2;
  (unaff_BP + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(u8 *)(u32)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(u8 *)(u32)((u8)(bVar3 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar3;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3134(void)

{
  u8 *pbVar1;
  char cVar2;
  char *pcVar3;
  u8 bVar4;
  u16 uVar5;
  i16 unaff_BP;

  cVar2 = *(char *)(unaff_BP + -0x4);
  if (cVar2 == '-') {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x4;
  }
  else if (cVar2 == '+') {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x1;
  }
  else if (cVar2 == ' ') {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x2;
  }
  else if (cVar2 == '#') {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x80;
  }
  else {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x8;
  }
  pcVar3 = *(char **)(unaff_BP + 0xa);
  cVar2 = *pcVar3;
  (unaff_BP + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(u8 *)(u32)((u8)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(u8 *)(u32)((u8)(bVar4 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar4;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3168(void)

{
  u8 *pbVar1;
  char cVar2;
  char *pcVar3;
  u8 bVar4;
  u16 uVar5;
  i16 unaff_BP;

  cVar2 = *(char *)(unaff_BP + -0x4);
  if (cVar2 == '*') {
    uVar5 = pass1_1000_34cf();
    if ((int)uVar5 < 0x0) {
      uVar5 = -uVar5;
      pbVar1 = (u8 *)(unaff_BP + -0x6);
      *pbVar1 = *pbVar1 | 0x4;
    }
  }
  else {
    uVar5 = (unaff_BP + -0xc) * 0xa + (u16)(u8)(cVar2 - 0x30);
  }
  (unaff_BP + -0xc) = uVar5;
  pcVar3 = *(char **)(unaff_BP + 0xa);
  cVar2 = *pcVar3;
  (unaff_BP + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(u8 *)(u32)((u8)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(u8 *)(u32)((u8)(bVar4 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar4;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3194(void)

{
  char cVar1;
  char *pcVar2;
  u8 bVar3;
  u16 uVar4;
  i16 unaff_BP;

  (unaff_BP + -0xe) = 0x0;
  pcVar2 = *(char **)(unaff_BP + 0xa);
  cVar1 = *pcVar2;
  (unaff_BP + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(u8 *)(u32)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(u8 *)(u32)((u8)(bVar3 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar3;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_319c(void)

{
  char cVar1;
  char *pcVar2;
  u8 bVar3;
  u16 uVar4;
  i16 unaff_BP;

  cVar1 = *(char *)(unaff_BP + -0x4);
  if (cVar1 == '*') {
    uVar4 = pass1_1000_34cf();
    if ((int)uVar4 < 0x0) {
      uVar4 = 0xffff;
    }
  }
  else {
    uVar4 = (unaff_BP + -0xe) * 0xa + (u16)(u8)(cVar1 - 0x30);
  }
  (unaff_BP + -0xe) = uVar4;
  pcVar2 = *(char **)(unaff_BP + 0xa);
  cVar1 = *pcVar2;
  (unaff_BP + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(u8 *)(u32)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(u8 *)(u32)((u8)(bVar3 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar3;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_31c5(void)

{
  u8 *pbVar1;
  char cVar2;
  char *pcVar3;
  u8 bVar4;
  u16 uVar5;
  i16 unaff_BP;

  cVar2 = *(char *)(unaff_BP + -0x4);
  if (cVar2 == 'l') {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x10;
  }
  else if (cVar2 == 'F') {
    pbVar1 = (u8 *)(unaff_BP + -0x6);
    *pbVar1 = *pbVar1 | 0x20;
  }
  else if (cVar2 == 'N') {
    pbVar1 = (u8 *)(unaff_BP + -0x5);
    *pbVar1 = *pbVar1 | 0x10;
  }
  else if (cVar2 == 'L') {
    pbVar1 = (u8 *)(unaff_BP + -0x5);
    *pbVar1 = *pbVar1 | 0x4;
  }
  else {
    pbVar1 = (u8 *)(unaff_BP + -0x5);
    *pbVar1 = *pbVar1 | 0x8;
  }
  pcVar3 = *(char **)(unaff_BP + 0xa);
  cVar2 = *pcVar3;
  (unaff_BP + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(u8 *)(u32)((u8)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(u8 *)(u32)((u8)(bVar4 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar4;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_31f7(u16 param_1)

{
  i16 *piVar1;
  u8 *pbVar2;
  u16 *puVar3;
  char cVar4;
  char *pcVar5;
  u8 bVar6;
  u16 uVar7;
  i16 iVar8;
  char *pcVar9;
  u16 uVar10;
  u16 extraout_DX;
  u16 extraout_DX_00;
  u16 extraout_DX_01;
  u16 extraout_DX_02;
  u16 extraout_DX_03;
  u16 extraout_DX_04;
  i16 unaff_BP;
  u16 *unaff_DI;
  u16 *puVar11;
  i16 unaff_ES;
  u8 in_AF;
  bool bVar12;
  u32 uVar13;

  cVar4 = *(char *)(unaff_BP + -0x4);
  if ((cVar4 == 'd') || (cVar4 == 'i')) {
    pbVar2 = (u8 *)(unaff_BP + -0x6);
    *pbVar2 = *pbVar2 | 0x40;
LAB_1000_3399:
    *(u8 *)(unaff_BP + -0x8) = 0xa;
LAB_1000_33d4:
    if ((*(u8 *)(unaff_BP + -0x6) & 0x10) == 0x0) {
      uVar7 = pass1_1000_34cf();
      if ((*(u8 *)(unaff_BP + -0x6) & 0x40) == 0x0) {
        uVar13 = (u32)uVar7;
      }
      else {
        uVar13 = (u32)(int)uVar7;
      }
    }
    else {
      uVar13 = pass1_1000_34d8();
    }
    if (((*(u8 *)(unaff_BP + -0x6) & 0x40) != 0x0) && ((long)uVar13 < 0x0)) {
      pbVar2 = (u8 *)(unaff_BP + -0x5);
      *pbVar2 = *pbVar2 | 0x1;
      uVar13 = CONCAT22(-((int)(uVar13 >> 0x10) + (u16)((int)uVar13 != 0x0)),-(int)uVar13);
    }
    param_1 = (u16)(uVar13 >> 0x10);
    if ((unaff_BP + -0xe) < 0x0) {
      (unaff_BP + -0xe) = 0x1;
    }
    else {
      pbVar2 = (u8 *)(unaff_BP + -0x6);
      *pbVar2 = *pbVar2 & 0xf7;
    }
    if (uVar13 == 0x0) {
      (unaff_BP + -0x12) = 0x0;
    }
    unaff_DI = (u16 *)(unaff_BP + -0x17);
    unaff_ES = (int)&DAT_1050_1050;
    pcVar9 = (char *)(u16)*(u8 *)(unaff_BP + -0x8);
    pass1_1000_356e((u16)uVar13,(u16)pcVar9,param_1);
    if (((*(u8 *)(unaff_BP + -0x5) & 0x2) != 0x0) && ((pcVar9 == NULL || (*(char *)unaff_DI != '0')))) {
      unaff_DI = (u16 *)(unaff_BP + -0x18);
      *(char *)unaff_DI = '0';
      pcVar9 = pcVar9 + 0x1;
    }
  }
  else {
    if (cVar4 == 'u') goto LAB_1000_3399;
    if (cVar4 == 'X') {
      *(u8 *)(unaff_BP + -0x3) = 0x7;
LAB_1000_33a9:
      if ((*(u8 *)(unaff_BP + -0x6) & 0x80) != 0x0) {
        (unaff_BP + -0x12) = 0x2;
        *(u8 *)(unaff_BP + -0x10) = 0x30;
        *(char *)(unaff_BP + -0xf) = *(char *)(unaff_BP + -0x3) + 'Q';
      }
      *(u8 *)(unaff_BP + -0x8) = 0x10;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'x') {
      *(u8 *)(unaff_BP + -0x3) = 0x27;
      goto LAB_1000_33a9;
    }
    if (cVar4 == 'o') {
      if ((*(u8 *)(unaff_BP + -0x6) & 0x80) != 0x0) {
        pbVar2 = (u8 *)(unaff_BP + -0x5);
        *pbVar2 = *pbVar2 | 0x2;
      }
      *(u8 *)(unaff_BP + -0x8) = 0x8;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'c') {
      uVar7 = pass1_1000_34cf();
      unaff_ES = (int)&DAT_1050_1050;
      *(u8 *)(unaff_BP + -0x216) = (char)uVar7;
      unaff_DI = (u16 *)(unaff_BP + -0x216);
      pcVar9 = (char *)0x1;
    }
    else if (cVar4 == 's') {
      uVar13 = pass1_1000_34e6(param_1);
      param_1 = (u16)(uVar13 >> 0x10);
      if ((unaff_DI == NULL) && (unaff_ES == 0x0)) {
        unaff_ES = (int)&DAT_1050_1050;
        unaff_DI = (u16 *)0x6057;
        pcVar9 = DAT_1050_605d;
      }
      else {
        iVar8 = (unaff_BP + -0xe);
        puVar11 = unaff_DI;
        if (iVar8 != 0x0) {
          bVar12 = true;
          do {
            if (iVar8 == 0x0) break;
            iVar8 += -0x1;
            puVar3 = puVar11;
            puVar11 = (u16 *)((int)puVar11 + 0x1);
            bVar12 = *(char *)puVar3 == '\0';
          } while (!bVar12);
          if (bVar12) {
            puVar11 = (u16 *)((int)puVar11 + -0x1);
          }
        }
        pcVar9 = (char *)((int)puVar11 - (int)unaff_DI);
      }
    }
    else {
      if (cVar4 == 'n') {
        pass1_1000_34e6(param_1);
        *unaff_DI = (unaff_BP + -0xa);
        if ((*(u8 *)(unaff_BP + -0x6) & 0x10) != 0x0) {
          unaff_DI[0x1] = 0x0;
        }
        goto LAB_1000_30cf;
      }
      if (cVar4 != 'p') {
        if ((cVar4 == 'E') || (cVar4 == 'G')) {
          piVar1 = (i16 *)(unaff_BP + -0x14);
          *piVar1 = *piVar1 + 0x1;
        }
        pbVar2 = (u8 *)(unaff_BP + -0x6);
        *pbVar2 = *pbVar2 | 0x40;
        bVar6 = *(u8 *)(unaff_BP + -0x4) | 0x20;
        iVar8 = (unaff_BP + -0xe);
        if (iVar8 < 0x1) {
          if (iVar8 == 0x0) {
            if (bVar6 == 0x67) {
              (unaff_BP + -0xe) = 0x1;
            }
          }
          else {
            (unaff_BP + -0xe) = 0x6;
          }
        }
        unaff_DI = (u16 *)(unaff_BP + -0x216);
        if ((*(u8 *)(unaff_BP + -0x5) & 0x4) == 0x0) {
          (*(code *)PTR_s_3_wav_1050_25cc_1050_6068)();
          piVar1 = (i16 *)(unaff_BP + 0xe);
          *piVar1 = *piVar1 + 0x8;
          param_1 = extraout_DX_00;
        }
        else {
          (*(code *)PTR_s_3_wav_1050_25cc_1050_607c)();
          piVar1 = (i16 *)(unaff_BP + 0xe);
          *piVar1 = *piVar1 + 0xa;
          param_1 = extraout_DX;
        }
        if (((*(u8 *)(unaff_BP + -0x6) & 0x80) != 0x0) && ((unaff_BP + -0xe) == 0x0)) {
          (*(code *)PTR_s_3_wav_1050_25cc_1050_6074)();
          param_1 = extraout_DX_01;
        }
        if ((bVar6 == 0x67) && (((unaff_BP + -0x6) & 0x80) == 0x0)) {
          (*(code *)PTR_s_3_wav_1050_25cc_1050_6070)();
          param_1 = extraout_DX_02;
        }
        unaff_ES = (int)&DAT_1050_1050;
        if (*(char *)unaff_DI == '-') {
          unaff_DI = (u16 *)(unaff_BP + -0x215);
          pbVar2 = (u8 *)(unaff_BP + -0x5);
          *pbVar2 = *pbVar2 | 0x1;
        }
        iVar8 = -0x1;
        puVar11 = unaff_DI;
        do {
          if (iVar8 == 0x0) break;
          iVar8 += -0x1;
          puVar3 = puVar11;
          puVar11 = (u16 *)((int)puVar11 + 0x1);
        } while (*(char *)puVar3 != '\0');
        pcVar9 = (char *)((int)puVar11 + (-0x1 - (int)unaff_DI));
        goto LAB_1000_3444;
      }
      if ((*(u8 *)(unaff_BP + -0x6) & 0x30) == 0x0) {
        uVar7 = pass1_1000_34cf();
        uVar13 = (u32)uVar7;
LAB_1000_32d8:
        *(u8 *)(unaff_BP + -0x3) = 0x7;
        param_1 = 0x0;
        pass1_1000_356e((u16)uVar13,0x10,0x0);
        pcVar9 = (char *)0x4;
      }
      else {
        uVar13 = pass1_1000_34d8();
        uVar10 = (u16)(uVar13 >> 0x10);
        if ((*(u8 *)(unaff_BP + -0x5) & 0x18) != 0x0) goto LAB_1000_32d8;
        *(u8 *)(unaff_BP + -0x3) = 0x7;
        pass1_1000_356e((u16)uVar13,0x10,0x0);
        param_1 = 0x0;
        pass1_1000_356e(uVar10,0x10,0x0);
        *(u8 *)(unaff_BP + -0x212) = 0x3a;
        pcVar9 = (char *)0x9;
      }
      unaff_ES = (int)&DAT_1050_1050;
      unaff_DI = (u16 *)(unaff_BP + -0x216);
    }
  }
LAB_1000_3444:
  if ((*(u8 *)(unaff_BP + -0x6) & 0x40) != 0x0) {
    if ((*(u8 *)(unaff_BP + -0x5) & 0x1) == 0x0) {
      if ((*(u8 *)(unaff_BP + -0x6) & 0x1) == 0x0) {
        if ((*(u8 *)(unaff_BP + -0x6) & 0x2) != 0x0) {
          *(u8 *)(unaff_BP + -0x10) = 0x20;
          (unaff_BP + -0x12) = 0x1;
        }
      }
      else {
        *(u8 *)(unaff_BP + -0x10) = 0x2b;
        (unaff_BP + -0x12) = 0x1;
      }
    }
    else {
      *(u8 *)(unaff_BP + -0x10) = 0x2d;
      (unaff_BP + -0x12) = 0x1;
    }
  }
  if ((*(u8 *)(unaff_BP + -0x6) & 0xc) == 0x0) {
    FUN_1000_3552(pcVar9,unaff_DI,unaff_ES);
    param_1 = extraout_DX_03;
  }
  pass1_1000_3534(in_AF,(unaff_BP + -0x12),param_1);
  if (((*(u8 *)(unaff_BP + -0x6) & 0x8) != 0x0) && ((*(u8 *)(unaff_BP + -0x6) & 0x4) == 0x0)) {
    FUN_1000_3552(pcVar9,unaff_DI,unaff_ES);
    param_1 = extraout_DX_04;
  }
  pass1_1000_3534(in_AF,pcVar9,param_1);
  if ((*(u8 *)(unaff_BP + -0x6) & 0x4) != 0x0) {
    FUN_1000_3552();
  }
LAB_1000_30cf:
  pcVar5 = *(char **)(unaff_BP + 0xa);
  cVar4 = *pcVar5;
  (unaff_BP + 0xa) = (int)pcVar5 + 0x1;
  *(char *)(unaff_BP + -0x4) = cVar4;
  if ((cVar4 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
    if ((u8)(cVar4 - 0x20U) < 0x59) {
      bVar6 = *(u8 *)(u32)((u8)(cVar4 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar6 = 0x0;
    }
    bVar6 = *(u8 *)(u32)((u8)(bVar6 * '\b' + *(char *)(unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
    *(u8 *)(unaff_BP + -0x7) = bVar6;
    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
    // WARNING: Treating indirect jump as call
    uVar7 = (**(code **)((char)bVar6 * 0x2 + 0x30a4))();
    return uVar7;
  }
  return (unaff_BP + -0xa);
}



u16 pass1_1000_34cf(void)

{
  u16 uVar1;
  u16 *puVar2;
  i16 unaff_BP;

  puVar2 = (u16*)(unaff_BP + 0xe);
  uVar1 = *puVar2;
  (unaff_BP + 0xe) = (int)puVar2 + 0x2;
  return uVar1;
}



u32 pass1_1000_34d8(void)

{
  u16 uVar1;
  u16 uVar2;
  u16 *puVar3;
  i16 unaff_BP;

  puVar3 = (u16 *)*(u32 *)(unaff_BP + 0xe);
  uVar1 = *puVar3;
  uVar2 = ((int)puVar3 + 0x2);
  (unaff_BP + 0xe) = (int)puVar3 + 0x4;
  return CONCAT22(uVar2,uVar1);
}



u32 pass1_1000_34e6(u16 param_1)

{
  u16 uVar1;
  i16 unaff_BP;
  u32 uVar2;

  if ((*(u8 *)(unaff_BP + -0x6) & 0x20) != 0x0) {
    uVar2 = pass1_1000_34d8();
    return uVar2;
  }
  uVar1 = pass1_1000_34cf();
  if (uVar1 == 0x0) {
    return (u32)param_1 << 0x10;
  }
  return CONCAT22(param_1,uVar1);
}



i16 pass1_1000_3503(u16 param_1,u16 param_2)

{
  i16 *piVar1;
  char *pcVar2;
  char **ppcVar3;
  u16 uVar4;
  i16 *piVar5;
  i16 unaff_BP;
  u16 uVar6;

  ppcVar3 = (char **)*(i16 **)(unaff_BP + 0x6);
  uVar6 = (u16)((u32)ppcVar3 >> 0x10);
  piVar5 = (i16 *)ppcVar3;
  piVar1 = piVar5 + 0x2;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    uVar4 = mem_1000_2bb6(param_2,(int)(char)param_1,piVar5);
    if (uVar4 == 0xffff) {
      return -0x1;
    }
  }
  else {
    pcVar2 = *ppcVar3;
    *ppcVar3 = *ppcVar3 + 0x1;
    *pcVar2 = (char)param_1;
  }
  return 0x0;
}



void pass1_1000_3534(undefined1 param_1,i16 param_2,u16 param_3)

{
  i16 *piVar1;
  u8 *pbVar2;
  u16 in_AX;
  i16 unaff_BP;
  u8 *unaff_DI;
  u16 uVar3;
  u16 unaff_ES;

  if (param_2 != 0x0) {
    piVar1 = (i16 *)(unaff_BP + -0xa);
    *piVar1 = *piVar1 + param_2;
    uVar3 = 0x0;
    do {
      pbVar2 = unaff_DI;
      unaff_DI = unaff_DI + 0x1;
      in_AX = pass1_1000_3503(in_AX & 0xff00 | (u16)*pbVar2,param_3);
      uVar3 |= in_AX;
      param_2 += -0x1;
    } while (param_2 != 0x0);
    if (uVar3 != 0x0) {
      (unaff_BP + -0xa) = 0xffff;
    }
  }
  return;
}



void FUN_1000_3552(void)

{
  i16 *piVar1;
  u16 param_3;
  i16 param_1;
  u16 param_2;
  i16 unaff_BP;
  u16 uVar2;

  if (param_1 != 0x0) {
    piVar1 = (i16 *)(unaff_BP + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar2 = 0x0;
    do {
      param_3 = pass1_1000_3503(param_3 & 0xff00 | param_2 & 0xff,param_2);
      uVar2 |= param_3;
      param_1 += -0x1;
    } while (param_1 != 0x0);
    if (uVar2 != 0x0) {
      (unaff_BP + -0xa) = 0xffff;
    }
  }
  return;
}



void pass1_1000_356e(u16 param_1,u16 param_2,u16 param_3)

{
  u8 *pbVar1;
  u32 uVar2;
  u8 bVar3;
  i16 unaff_BP;
  i16 unaff_SI;
  u8 *unaff_DI;
  u16 unaff_ES;

  while (((0x0 < unaff_SI || (param_1 != 0x0)) || (param_3 != 0x0))) {
    uVar2 = (u32)param_3;
    param_3 /= param_2;
    uVar2 = uVar2 % (u32)param_2 << 0x10 | (u32)param_1;
    param_1 = (u16)(uVar2 / param_2);
    bVar3 = (char)(uVar2 % (u32)param_2) + 0x30;
    if (0x39 < bVar3) {
      bVar3 += *(char *)(unaff_BP + -0x3);
    }
    pbVar1 = unaff_DI;
    unaff_DI = unaff_DI + -0x1;
    *pbVar1 = bVar3;
    unaff_SI += -0x1;
  }
  return;
}



u16 * pass1_1000_35aa(void)

{
  u16 *puVar1;

  puVar1 = (u16 *)&PTR_LOOP_1050_6210;
  while( true ) {
    if (PTR_LOOP_1050_5ff0 < puVar1) {
      return NULL;
    }
    if ((*(u8 *)(puVar1 + 0x5) & 0x83) == 0x0) break;
    puVar1 = puVar1 + 0x6;
  }
  *(u8 *)(puVar1 + 0x5) = 0x0;
  puVar1[0x2] = 0x0;
  puVar1[0x4] = 0x0;
  puVar1[0x3] = 0x0;
  puVar1[0x1] = 0x0;
  *puVar1 = 0x0;
  *(u8 *)((int)puVar1 + 0xb) = 0xff;
  return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10003622)

u16 dos3_call_op_1000_35fe(u16 param_1,i16 param_2)

{
  code *pcVar1;
  u16 uVar2;
  bool bVar2;

  if (param_1 < u16_1050_5f8a) {
    bVar2 = false;
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(param_2 + 0x1);
    if (!bVar2) {
      *(u8 *)(param_1 + 0x5f90) = 0x0;
    }
  }
  else {
    uVar2 = 0x900;
    bVar2 = true;
  }
  if (bVar2) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)

void mixed_dos3_call_1000_3636(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u8 *pbVar1;
  code *pcVar2;
  u16 uVar3;
  i16 unaff_BP;
  i16 iVar4;
  bool bVar5;
  u32 uVar6;

  iVar4 = unaff_BP + 0x1;
  if (((param_1 < u16_1050_5f8a) || (u16_1050_61ec == 0x0)) || (0x2 < param_1)) {
    if ((u16_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0x0)) goto LAB_1000_36e3;
    if (param_4 == 0x0) goto LAB_1000_369b;
    bVar5 = false;
    pcVar2 = (code *)swi(0x21);
    uVar6 = (*pcVar2)();
    uVar3 = (u16)uVar6;
    if (bVar5) goto LAB_1000_299d;
    if ((param_4 & 0x2) == 0x0) {
      if (-0x1 < (int)((int)((u32)uVar6 >> 0x10) + param_3 + (u16)CARRY2(uVar3,param_2))) {
LAB_1000_36e3:
        bVar5 = false;
        pcVar2 = (code *)swi(0x21);
        uVar3 = (*pcVar2)(iVar4);
        if (!bVar5) {
          pbVar1 = (u8 *)(param_1 + 0x5f90);
          bVar5 = false;
          *pbVar1 = *pbVar1 & 0xfd;
        }
        goto LAB_1000_299d;
      }
    }
    else {
      pcVar2 = (code *)swi(0x21);
      uVar6 = (*pcVar2)();
      if (-0x1 < (int)((int)((u32)uVar6 >> 0x10) + param_3 + (u16)CARRY2((u16)uVar6,param_2))) goto LAB_1000_36e3;
      pcVar2 = (code *)swi(0x21);
      (*pcVar2)();
    }
LAB_1000_369b:
    uVar3 = (u16)s_471_bmp_1050_1600;
  }
  else {
    uVar3 = 0x900;
  }
  bVar5 = true;
LAB_1000_299d:
  if (bVar5) {
    pass1_1000_29b5(uVar3);
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10003989)
// WARNING: Removing unreachable block (ram,0x100038a1)
// WARNING: Removing unreachable block (ram,0x10003867)
// WARNING: Removing unreachable block (ram,0x10003967)
// WARNING: Removing unreachable block (ram,0x1000391a)
// WARNING: Removing unreachable block (ram,0x10003803)
// WARNING: Removing unreachable block (ram,0x100037b7)
// WARNING: Removing unreachable block (ram,0x10003799)
// WARNING: Removing unreachable block (ram,0x10003765)
// WARNING: Removing unreachable block (ram,0x100037ec)
// WARNING: Removing unreachable block (ram,0x100038d9)
// WARNING: Removing unreachable block (ram,0x100038f2)
// WARNING: Removing unreachable block (ram,0x1000393a)
// WARNING: Removing unreachable block (ram,0x1000384b)
// WARNING: Removing unreachable block (ram,0x1000388b)
// WARNING: Removing unreachable block (ram,0x100038ba)
// WARNING: Removing unreachable block (ram,0x100039b9)
// WARNING: Removing unreachable block (ram,0x1000381c)
// WARNING: Could not reconcile some variable overlaps

u16 mixed_dos3_call_1000_370a(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u8 param_5,u16 param_6)

{
  code *pcVar1;
  u16 uVar3;
  u16 uVar2;
  i16 iVar3;
  u8 bVar2;
  u16 uVar7;
  u16 extraout_DX;
  u16 uVar4;
  u16 uVar5;
  i16 unaff_BP;
  u16 uVar6;
  u16 uVar8;
  bool bVar10;
  u16 uStack14;
  u8 bVar9;
  u16 in_stack_0000fffa;
  u16 uVar9;
  u16 uVar10;
  u8 cVar12;

  uVar6 = unaff_BP + 0x1;
  _param_5 = param_6;
  uVar3 = param_1 & 0xff00;
  param_1 = uVar3 | param_5;
  uVar9 = 0x0;
  if (((param_4 & 0x8000) == 0x0) && (((param_4 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0x0)))) {
    uVar9 = 0x80;
  }
  bVar10 = false;
  pcVar1 = (code *)swi(0x21);
  uVar7 = param_4;
  uVar2 = (*pcVar1)();
  if (bVar10) {
    if ((uVar2 == 0x2) && ((uVar7 & 0x100) != 0x0)) {
      uVar10 = uVar9 & 0xff;
      param_1 = (int)s_____s__lu_1050_38cd + 0x3;
      pass1_1000_39e1();
      uVar7 = 0x0;
      _param_5 = param_6;
LAB_1000_38e3:
      bVar10 = false;
      pcVar1 = (code *)swi(0x21);
      uVar2 = (*pcVar1)();
      if (bVar10) goto LAB_1000_299d;
      if (((char)param_1 != '\0') || (uVar5 = uVar2, uVar8 = uStack14, (param_4 & 0x2) == 0x0)) {
        pcVar1 = (code *)swi(0x21);
        (*pcVar1)();
        bVar10 = false;
        pcVar1 = (code *)swi(0x21);
        uVar2 = (*pcVar1)();
        if (bVar10) goto LAB_1000_299d;
        uVar5 = uVar2;
        uVar8 = param_1;
        if (((uVar10 & 0x100) == 0x0) && (uVar8 = param_1, (_param_5 & 0x1) != 0x0)) {
          uVar7 = (u16)(u8)((u8)uVar7 | 0x1);
          bVar10 = false;
          pcVar1 = (code *)swi(0x21);
          uVar4 = uVar2;
          uVar2 = (*pcVar1)();
          uVar5 = uVar4;
          uVar8 = uVar6;
          if (bVar10) goto LAB_1000_299d;
        }
      }
LAB_1000_3973:
      bVar9 = (u8)uVar10;
      if ((uVar10 & 0x40) == 0x0) {
        pcVar1 = (code *)swi(0x21);
        (*pcVar1)();
        bVar2 = 0x0;
        if ((uVar7 & 0x1) != 0x0) {
          bVar2 = 0x10;
        }
        if ((param_4 & 0x8) != 0x0) {
          bVar2 |= 0x20;
        }
      }
      else {
        bVar2 = 0x0;
      }
      if (uVar5 < &u16_1050_5f8a) {
        *(u8 *)(uVar5 + 0x5f90) = bVar2 | bVar9 | 0x1;
        return uVar5;
      }
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      uVar2 = 0x1800;
    }
  }
  else {
    if ((uVar7 & 0x500) != 0x500) {
      uVar10 = CONCAT11(0x1,(char)uVar9);
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      if ((extraout_DX & 0x80) != 0x0) {
        uVar10 |= 0x40;
      }
      uVar5 = uVar2;
      uVar8 = param_1;
      if ((uVar10 & 0x40) == 0x0) {
        if ((param_4 & 0x200) == 0x0) {
          if (((uVar10 & 0x80) != 0x0) && ((param_4 & 0x2) != 0x0)) {
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            pcVar1 = (code *)swi(0x21);
            iVar3 = (*pcVar1)();
            if ((iVar3 != 0x0) && (param_1._1_1_ = (char)(uVar3 >> 0x8), param_1._1_1_ == '\x1a')) {
              pcVar1 = (code *)swi(0x21);
              (*pcVar1)(uVar6);
              pcVar1 = (code *)swi(0x21);
              (*pcVar1)();
            }
            uVar7 = 0x0;
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            uVar5 = uVar2;
            uVar8 = uStack14;
          }
        }
        else {
          if ((param_4 & 0x3) == 0x0) {
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            goto LAB_1000_38e3;
          }
          uVar7 = 0x0;
          pcVar1 = (code *)swi(0x21);
          (*pcVar1)();
          uVar5 = uVar2;
          uVar8 = param_1;
        }
      }
      goto LAB_1000_3973;
    }
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    uVar2 = 0x1100;
  }
  bVar10 = true;
LAB_1000_299d:
  if (bVar10) {
    pass1_1000_29b5(uVar2);
    uVar2 = 0xffff;
  }
  return uVar2;
}



void pass1_1000_39e1(void)

{
  return;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)
// WARNING: Unable to use type for symbol uVar5

u8 * mixed_dos3_call_1000_39f2(u8 *param_1,char *param_2,u8 *param_3)

{
  u8 *pbVar2;
  u8 *puVar3;
  u16 uVar4;
  code *pcVar5;
  u8 *puVar6;
  u16 uVar6;
  u16 uVar7;
  i16 *piVar7;
  i16 *piVar8;
  i16 *piVar9;
  u8 *pbVar10;
  i16 iVar10;
  u8 *puVar11;
  u8 *pbVar12;
  u8 *puVar12;
  u8 *unaff_SI;
  u8 *pbVar13;
  u16 uVar14;
  u16 unaff_CS;
  u8 uVar15;
  u8 bVar16;
  char cVar17;
  char in_AF;
  bool bVar18;
  char cVar19;
  char cVar20;
  u32 uVar21;
  char *pcVar22;
  u8 *puVar23;
  i16 in_stack_0000fff6;
  i16 *piStack8;
  i16 *piStack6;
  u8 *puVar2;
  u16 uVar5;
  u8 *pbVar1;

  puVar6 = (u8 *)u16_1050_5f8a;
  if ((u16_1050_61ec != 0x0) &&
     (puVar6 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e, param_1 < (u8 *)((int)&u16_1050_0002 + 0x1U))) {
    param_1 = (u8 *)u16_1050_5f8a;
  }
  if (puVar6 <= param_1) {
    uVar15 = true;
    puVar6 = (u8 *)0x900;
    goto LAB_1000_299d;
  }
  puVar12 = param_1;
  puVar23 = (u8 *)u16_1050_5f8a;
  if ((param_1[0x5f90] & 0x20) != 0x0) {
    uVar15 = false;
    pcVar5 = (code *)swi(0x21);
    puVar6 = (u8 *)(*pcVar5)();
    unaff_CS = 0x1000;
    if ((bool)uVar15) goto LAB_1000_299d;
  }
  pbVar12 = (u8 *)param_2;
  if ((puVar12[0x5f90] & 0x80) == 0x0) {
LAB_1000_3acf:
    uVar15 = false;
    puVar6 = param_3;
    if (param_3 != NULL) {
      uVar15 = puVar12 < puVar23;
      if ((bool)uVar15) {
        uVar15 = 0x0;
        pcVar5 = (code *)swi(0x21);
        uVar21 = (*pcVar5)();
      }
      else {
        piVar8 = pass1_1000_55b1((int)&DAT_1050_1050,in_stack_0000fff6);
        uVar21 = CONCAT22(pbVar12,piVar8);
      }
      puVar6 = (u8 *)uVar21;
      if ((bool)uVar15) {
        puVar6 = (u8 *)CONCAT11(0x9,(char)uVar21);
      }
      else {
        uVar15 = false;
        if (puVar6 == NULL) {
          if (((puVar12[0x5f90] & 0x40) == 0x0) || (*(char *)((u32)uVar21 >> 0x10) != '\x1a')) {
            uVar15 = true;
            puVar6 = (u8 *)0x1c00;
          }
          else {
            uVar15 = false;
          }
        }
      }
    }
  }
  else {
    in_stack_0000fff6 = (i16)&DAT_1050_1050;
    bVar18 = true;
    piStack6 = NULL;
    piStack8 = NULL;
    piVar9 = (i16 *)param_3;
    pbVar13 = pbVar12;
    if (param_3 != NULL) {
      do {
        if (piVar9 == NULL) break;
        piVar9 = (i16 *)((int)piVar9 + -0x1);
        pbVar1 = pbVar13;
        pbVar13 = pbVar13 + 0x1;
        bVar18 = *pbVar1 == '\n';
      } while (!bVar18);
      puVar23 = unaff_SI;
      if (!bVar18) goto LAB_1000_3acf;
      pcVar22 = param_2;
      uVar6 = pass1_1000_3bac();
      pcVar22._2_2_ = (int)((u32)pcVar22 >> 0x10);
      pbVar10 = (u8 *)pcVar22;
      if (uVar6 < 0xa9) {
        exit_1000_25f2(-0x4,0x3ad9,unaff_CS,pcVar22._2_2_);
        if ((int)pbVar13 - (int)pbVar10 == 0x0) {
          return unaff_SI;
        }
        bVar16 = param_1 < unaff_SI;
        uVar4 = (int)param_1 - (int)unaff_SI;
        cVar20 = (int)uVar4 < 0x0;
        cVar19 = uVar4 == 0x0;
        cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1U) == 0x0;
        if ((bool)bVar16) {
          bVar16 = 0x0;
          cVar20 = '\0';
          cVar19 = '\x01';
          cVar17 = '\x01';
          pcVar5 = (code *)swi(0x21);
          piVar7 = (i16 *)(*pcVar5)((int)&DAT_1050_1050,piVar9,puVar12);
        }
        else {
          piVar7 = pass1_1000_55b1((int)pbVar13 - (int)pbVar10,(i16)&DAT_1050_1050);
        }
        if (!(bool)bVar16) {
          bVar16 = piVar9 < piVar7;
          uVar4 = (int)piVar9 - (int)piVar7;
          cVar20 = (int)uVar4 < 0x0;
          cVar19 = uVar4 == 0x0;
          cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1U) == 0x0;
          piStack6 = piVar7;
          if ((bool)bVar16 || (bool)cVar19) {
            return unaff_SI;
          }
        }
        uVar4 = (u16)(u8)(cVar20 << 0x7 | cVar19 << 0x6 | in_AF << 0x4 | cVar17 << 0x2 | 0x2U | bVar16) << 0x8;
        puVar6 = (u8 *)((u16)piVar7 & 0xff | uVar4);
        if (piStack6 == NULL) {
          uVar15 = (uVar4 & 0x100) != 0x0;
          if ((bool)uVar15) {
            puVar6 = (u8 *)CONCAT11(0x9,(char)((u16)piVar7 & 0xff));
          }
          else if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
            uVar15 = true;
            puVar6 = (u8 *)0x1c00;
          }
          else {
            uVar15 = false;
          }
          goto LAB_1000_299d;
        }
      }
      else {
        puVar6 = &stack0xfff0;
        iVar10 = 0x200;
        if (uVar6 < 0x228) {
          iVar10 = 0x80;
        }
        iVar10 = -iVar10;
        puVar11 = &stack0xfff0 + iVar10;
        puVar12 = &stack0xfff0 + iVar10;
        (&stack0xffee + iVar10) = (int)&DAT_1050_1050;
        uVar14 = (&stack0xffee + iVar10);
        do {
          pbVar2 = pbVar12;
          pbVar12 = pbVar12 + 0x1;
          bVar16 = *pbVar2;
          uVar5 = uVar6 & 0xff00;
          uVar6 = uVar5 | bVar16;
          if (bVar16 == 0xa) {
            uVar7 = CONCAT11((char)(uVar5 >> 0x8),0xd);
            if (puVar12 == puVar6) {
              (&stack0xffee + iVar10) = 0x3abd;
              uVar7 = mixed_dos3_call_1000_3ad9(uVar7,(i16)puVar11,param_3,(&stack0xfff0 + iVar10));
            }
            puVar3 = puVar12;
            puVar12 = puVar12 + 0x1;
            *puVar3 = (u8)uVar7;
            uVar6 = CONCAT11((char)(uVar7 >> 0x8),0xa);
            piStack8 = (i16 *)((int)piStack8 + 0x1);
          }
          if (puVar12 == puVar6) {
            (&stack0xffee + iVar10) = 0x3ac8;
            uVar6 = mixed_dos3_call_1000_3ad9(uVar6,(i16)puVar11,param_3,(&stack0xfff0 + iVar10));
          }
          puVar2 = puVar12;
          puVar12 = puVar12 + 0x1;
          *puVar2 = (u8)uVar6;
          param_3 = param_3 + -0x1;
        } while (param_3 != NULL);
        (&stack0xffee + iVar10) = 0x3ab1;
        mixed_dos3_call_1000_3ad9(uVar6,(i16)puVar11,0x0,(&stack0xfff0 + iVar10));
      }
    }
    uVar15 = piStack6 < piStack8;
    puVar6 = (u8 *)((int)piStack6 - (int)piStack8);
  }
LAB_1000_299d:
  if ((bool)uVar15) {
    pass1_1000_29b5(puVar6);
    puVar6 = (u8 *)0xffff;
  }
  return puVar6;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

u16 mixed_dos3_call_1000_3ad9(u16 param_1,i16 param_2,i16 *param_3,u16 param_4)

{
  u16 uVar2;
  code *pcVar3;
  u16 uVar4;
  i16 *piVar5;
  u16 uVar5;
  i16 unaff_BP;
  i16 unaff_DI;
  u8 bVar5;
  bool bVar6;
  u8 cVar7;
  char in_AF;
  u8 cVar8;
  u8 cVar9;
  u16 *puVar1;
  i16 *piVar1;
  u16 *puVar2;
  u16 uVar1;

  if (unaff_DI - param_2 == 0x0) {
    return param_4;
  }
  uVar5 = (unaff_BP + 0x6);
  puVar1 = (u16 *)(unaff_BP + -0xc);
  bVar5 = uVar5 < *puVar1;
  uVar1 = uVar5 - *puVar1;
  cVar9 = (int)uVar1 < 0x0;
  cVar8 = uVar1 == 0x0;
  cVar7 = (POPCOUNT(uVar1 & 0xff) & 0x1U) == 0x0;
  if ((bool)bVar5) {
    bVar5 = 0x0;
    cVar9 = '\0';
    cVar8 = '\x01';
    cVar7 = '\x01';
    pcVar3 = (code *)swi(0x21);
    piVar5 = (i16 *)(*pcVar3)();
  }
  else {
    piVar5 = pass1_1000_55b1(unaff_DI - param_2,(i16)&DAT_1050_1050);
  }
  if (!(bool)bVar5) {
    piVar1 = (i16 *)(unaff_BP + -0x4);
    *piVar1 = *piVar1 + (int)piVar5;
    bVar5 = param_3 < piVar5;
    uVar2 = (int)param_3 - (int)piVar5;
    cVar9 = (int)uVar2 < 0x0;
    cVar8 = uVar2 == 0x0;
    cVar7 = (POPCOUNT(uVar2 & 0xff) & 0x1U) == 0x0;
    if ((bool)bVar5 || (bool)cVar8) {
      return param_4;
    }
  }
  uVar2 = (u16)(u8)(cVar9 << 0x7 | cVar8 << 0x6 | in_AF << 0x4 | cVar7 << 0x2 | 0x2 | bVar5) << 0x8;
  uVar4 = (u16)piVar5 & 0xff | uVar2;
  if ((unaff_BP + -0x4) == 0x0) {
    bVar6 = (uVar2 & 0x100) != 0x0;
    if (bVar6) {
      uVar4 = CONCAT11(0x9,(char)((u16)piVar5 & 0xff));
    }
    else if (((*(u8 *)(uVar5 + 0x5f90) & 0x40) == 0x0) || (**(char **)(unaff_BP + 0x8) != '\x1a')) {
      bVar6 = true;
      uVar4 = 0x1c00;
    }
    else {
      bVar6 = false;
    }
  }
  else {
    uVar2 = (unaff_BP + -0x4);
    puVar2 = (u16 *)(unaff_BP + -0x6);
    bVar6 = uVar2 < *puVar2;
    uVar4 = uVar2 - *puVar2;
  }
  if (bVar6) {
    ((unaff_BP + -0xa) + 0x2) = 0x29a2;
    pass1_1000_29b5(uVar4);
    uVar4 = 0xffff;
  }
  return uVar4;
}



i16 pass1_1000_3bac(void)

{
  i16 iVar1;

  if (PTR_LOOP_1050_5f48 < &stack0x0004) {
    iVar1 = -((int)PTR_LOOP_1050_5f48 - (int)&stack0x0004);
  }
  else {
    iVar1 = 0x0;
  }
  return iVar1;
}



void pass1_1000_3bc0(i16 param_1,i16 param_2)

{
  i16 *piVar1;
  u16 uVar2;
  u16_t uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  u16 *unaff_SI;
  u16 *puVar7;
  u16 unaff_DI;
  bool bVar8;
  u32 uVar9;

  if ((*(u8 *)(param_2 + 0x2) & 0x1) != 0x0) {
    pass1_1000_3cb7(param_2);
    uVar5 = *unaff_SI;
    if ((uVar5 & 0x1) != 0x0) {
      param_1 = (param_1 - uVar5) + -0x1;
    }
    uVar5 = (param_2 + 0x4);
    if (uVar5 != 0x0) {
      uVar4 = param_1 + 0x2U + uVar5;
      if (!CARRY2(param_1 + 0x2U,uVar5)) {
        uVar3 = pass1_1000_29dc((int)&DAT_1050_1050);
        uVar5 = &PTR_LOOP_1050_6066;
        if (uVar5 == 0x1000) goto LAB_1000_3c12;
        uVar2 = 0x8000;
        while (uVar5 <= uVar2) {
          uVar2 >>= 0x1;
          if (uVar2 == 0x0) goto LAB_1000_3c2b;
        }
        if (uVar2 < 0x8) goto LAB_1000_3c2b;
        uVar5 = uVar2 << 0x1;
        goto LAB_1000_3c12;
      }
      uVar2 = 0x0;
      uVar5 = 0xfff0;
      if (uVar4 == 0x0) {
        while( true ) {
          bVar8 = false;
          uVar9 = mixed_mem_op_1000_3c51(uVar2,uVar4,param_2,0x3c23,unaff_DI);
          if (!bVar8) break;
          if (uVar5 == 0xfff0) {
            return;
          }
LAB_1000_3c2b:
          uVar5 = 0x10;
LAB_1000_3c12:
          uVar5 -= 0x1;
          uVar2 = uVar5 + uVar4;
          if (CARRY2(uVar5,uVar4)) {
            uVar2 = 0x0;
          }
          uVar5 = ~uVar5;
          uVar2 &= uVar5;
        }
        iVar6 = (int)uVar9 - (param_2 + 0x4);
        (param_2 + 0x4) = (int)uVar9;
        (u16*)(param_2 + 0xa) = unaff_SI;
        piVar1 = *(i16 **)(param_2 + 0xc);
        *piVar1 = iVar6 + -0x1;
        puVar7 = (u16 *)((int)piVar1 + iVar6);
        *puVar7 = 0xfffe;
        (u16*)(param_2 + 0xc) = puVar7;
      }
    }
  }
  return;
}


/*
Unable to decompile 'mixed_mem_op_1000_3c51'
Cause:
Low-level Error: Overlapping input varnodes
*/


void pass1_1000_3cb7(i16 param_1)

{
  u16 uVar1;
  u16 *puVar2;

  puVar2 = (u16*)(param_1 + 0xa);
  if (puVar2 == (u16*)(param_1 + 0xc)) {
    puVar2 = (u16*)(param_1 + 0x8);
  }
  while( true ) {
    uVar1 = *puVar2;
    if (uVar1 == 0xfffe) break;
    puVar2 = (u16 *)((int)puVar2 + (uVar1 & 0xfffe) + 0x2);
  }
  return;
}



void pass1_1000_3cd8(u16 param_1,u16 param_2)

{
  free_mem_1000_407a(param_1,param_2);
  return;
}



u16 * pass1_1000_3cea(u32 param_1,char *param_2)

{
  u16 *pUVar1;
  char *pcVar2;
  u16 *pUVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 *pUVar7;
  char *pcVar8;
  u16 *pUVar9;
  u16 *pUVar10;
  u16 uVar11;
  u16 uVar12;
  bool bVar13;

  uVar11 = (u16)(param_1 >> 0x10);
  bVar13 = true;
  iVar4 = -0x1;
  pUVar7 = (u16 *)param_1;
  do {
    if (iVar4 == 0x0) break;
    iVar4 += -0x1;
    pUVar1 = pUVar7;
    pUVar7 = (u16 *)((int)pUVar7 + 0x1);
    bVar13 = *(char *)pUVar1 == '\0';
  } while (!bVar13);
  pUVar10 = (u16 *)((int)pUVar7 + -0x1);
  uVar12 = (u16)((u32)param_2 >> 0x10);
  pcVar8 = (char *)param_2;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 -= 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
    bVar13 = *pcVar2 == '\0';
  } while (!bVar13);
  uVar5 = ~uVar5;
  if (!bVar13) {
    pcVar8 = pcVar8 + -uVar5;
    uVar5 += 0x1;
  }
  pUVar9 = (u16 *)(pcVar8 + -uVar5);
  if (uVar5 == 0x0) {
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar10 = *pUVar1;
    uVar5 = 0xfffe;
    pUVar10 = (u16 *)((int)pUVar7 + 0x1);
  }
  else if (((u16)pUVar9 & 0x1) != 0x0) {
    pUVar1 = pUVar9;
    pUVar9 = (u16 *)((int)pUVar9 + 0x1);
    *(u8 *)pUVar10 = *(u8 *)pUVar1;
    uVar5 -= 0x1;
    pUVar10 = pUVar7;
  }
  for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = pUVar10 + 0x1;
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar3 = *pUVar1;
  }
  for (uVar5 = (u16)((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = (u16 *)((int)pUVar10 + 0x1);
    pUVar1 = pUVar9;
    pUVar9 = (u16 *)((int)pUVar9 + 0x1);
    *(u8 *)pUVar3 = *(u8 *)pUVar1;
  }
  return (u16 *)param_1;
}



void unk_str_op_1000_3d3e(char *param_1,char *in_string_2)

{
  u16 *puVar4;
  u16 *puVar5;
  u16 uVar6;
  u16 uVar7;
  char *l_string_2;
  char *puVar6;
  char *puVar7;
  u16 uVar8;
  char *l_string_1;
  bool l_b_var8;
  char *puVar3;
  char *puVar2;
  char *puVar1;

  l_string_1 = (char *)((u32)in_string_2 >> 0x10);
  l_string_2 = (char *)in_string_2;
  l_b_var8 = true;
  uVar6 = 0xffff;
  puVar6 = l_string_2;
  do {
    if (uVar6 == 0x0) break;
    uVar6 -= 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    l_b_var8 = *puVar1 == '\0';
  } while (!l_b_var8);
  uVar6 = ~uVar6;
  uVar8 = (u16)((u32)param_1 >> 0x10);
  puVar7 = (char *)param_1;
  if (l_b_var8) {
    if (((u32)param_1 & 0x1) != 0x0) {
      puVar7 = puVar7 + 0x1;
      l_string_2 = l_string_2 + 0x1;
      *param_1 = *in_string_2;
      uVar6 -= 0x1;
    }
  }
  else {
    puVar7 = puVar7 + 0x2;
    l_string_2 = l_string_2 + 0x2;
    param_1 = in_string_2;
    uVar6 -= 0x1;
  }
  for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
    puVar5 = (u16 *)puVar7;
    puVar7 = (char *)((int)puVar7 + 0x2);
    puVar4 = (u16 *)l_string_2;
    l_string_2 = (char *)((int)l_string_2 + 0x2);
    *puVar5 = *puVar4;
  }
  for (uVar6 = (u16)((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
    puVar5 = (u16 *)puVar7;
    puVar7 = (char *)((int)puVar7 + 0x1);
    puVar4 = (u16 *)l_string_2;
    l_string_2 = (char *)((int)l_string_2 + 0x1);
    *(u8 *)puVar5 = *(u8 *)puVar4;
  }
  return;
}



i16 pass1_1000_3d7a(char *param_1,char *param_2)

{
  u8 *pbVar2;
  u8 *pbVar3;
  i16 iVar4;
  u16 uVar5;
  char *string_4;
  char *string_1;
  char *string_2;
  char *string_6;
  u16 uVar6;
  bool bool_1;
  bool bool_2;
  char *pbVar4;
  char *pbVar1;
  char *string_3;

  string_1 = (char *)param_1;
  uVar6 = (u16)((u32)param_2 >> 0x10);
  string_2 = (char *)param_2;
  iVar4 = 0x0;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 -= 0x1;
    string_3 = string_2;
    string_2 = string_2 + 0x1;
  } while (*string_3 != '\0');
  string_4 = (char *)~uVar5;
  bool_1 = string_2 < string_4;
  string_6 = string_2 + -(int)string_4;
  bool_2 = string_6 == NULL;
  do {
    if (string_4 == NULL) break;
    string_4 = string_4 + -0x1;
    pbVar3 = (u8 *)string_6;
    string_6 = (char *)((u8 *)string_6 + 0x1);
    pbVar2 = (u8 *)string_1;
    string_1 = (char *)((u8 *)string_1 + 0x1);
    bool_1 = *pbVar2 < *pbVar3;
    bool_2 = *pbVar2 == *pbVar3;
  } while (bool_2);
  if (!bool_2) {
    iVar4 = (0x1 - (u16)bool_1) - (u16)(bool_1 != 0x0);
  }
  return iVar4;
}



u16 str_op_1000_3da4(char *param_1)

{
  char *pcVar1;
  u16 uVar2;
  char *pcVar3;
  bool bVar4;

  pcVar3 = (char *)param_1;
  bVar4 = true;
  uVar2 = 0xffff;
  do {
    if (uVar2 == 0x0) break;
    uVar2 -= 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar4 = *pcVar1 == '\0';
  } while (!bVar4);
  uVar2 = ~uVar2;
  if (bVar4) {
    uVar2 -= 0x1;
  }
  return uVar2;
}



uchar str_op_1000_3dbe(char *param_1,char *param_2,u16 param_3)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  char *pcVar4;
  u16 uVar5;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  pcVar4 = (char *)param_1;
  pcVar3 = (char *)param_2;
  if (param_3 != 0x0) {
    do {
      pcVar1 = pcVar3;
      pcVar3 = pcVar3 + 0x1;
      cVar2 = *pcVar1;
      if (cVar2 == '\0') break;
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = cVar2;
      param_3 -= 0x1;
    } while (param_3 != 0x0);
    for (; param_3 != 0x0; param_3 -= 0x1) {
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = '\0';
    }
  }
  return (uchar)param_1;
}



u16 pass1_1000_3de8(char *param_1,char *param_2,u16 param_3,u16_t param_4,u16_t param_5)

{
  u8 *pbVar1;
  char *pcVar2;
  char *pcVar3;
  u8 bVar4;
  u16 uVar5;
  i16 iVar6;
  char *pcVar7;
  char *pcVar8;
  u16 uVar9;
  u16 uVar10;
  bool bVar11;

  if (param_3 != 0x0) {
    uVar9 = (u16)((u32)param_1 >> 0x10);
    pcVar8 = (char *)param_1;
    uVar5 = param_3;
    pcVar7 = pcVar8;
    do {
      if (uVar5 == 0x0) break;
      uVar5 -= 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 != '\0');
    iVar6 = param_3 - uVar5;
    uVar10 = (u16)((u32)param_2 >> 0x10);
    pcVar7 = (char *)param_2;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 == *pcVar3);
    bVar4 = pcVar7[-0x1];
    uVar5 = 0x0;
    pbVar1 = (u8 *)(pcVar8 + -0x1);
    bVar11 = bVar4 == *pbVar1;
    if (bVar4 < *pbVar1 || bVar11) {
      if (bVar11) {
        return 0x0;
      }
      uVar5 = 0xfffe;
    }
    param_3 = ~uVar5;
  }
  return param_3;
}



i16 pass1_1000_3e2c(u32 param_1)

{
  u8 *pbVar1;
  u8 bVar2;
  u8 bVar3;
  i16 iVar4;
  u8 *pbVar5;
  u16 uVar6;

  uVar6 = (u16)(param_1 >> 0x10);
  pbVar5 = (u8 *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (u16)(u8)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



i16 pass1_1000_3e2c(u32 param_1)

{
  u8 *pbVar1;
  u8 bVar2;
  u8 bVar3;
  i16 iVar4;
  u8 *pbVar5;
  u16 uVar6;

  uVar6 = (u16)(param_1 >> 0x10);
  pbVar5 = (u8 *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (u16)(u8)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



i16 pass1_1000_3e2c(u32 param_1)

{
  u8 *pbVar1;
  u8 bVar2;
  u8 bVar3;
  i16 iVar4;
  u8 *pbVar5;
  u16 uVar6;

  uVar6 = (u16)(param_1 >> 0x10);
  pbVar5 = (u8 *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (u16)(u8)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



u8 * pass1_1000_3e82(u16 param_1,uchar *param_2,u16 param_3)

{
  u8 *pbVar1;
  u32 uVar2;
  u8 bVar3;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u8 *pbVar8;
  u8 *pbVar9;
  u8 *pbVar10;
  u8 *pbVar11;
  u16 uVar12;
  bool bVar13;
  char cVar4;

  uVar6 = 0x0;
  if (param_3 == 0xa) {
    uVar6 = (int)param_1 >> 0xf;
  }
  uVar12 = (u16)((u32)param_2 >> 0x10);
  pbVar9 = (u8 *)param_2;
  pbVar10 = pbVar9;
  pbVar8 = pbVar9;
  if ((param_3 == 0xa) && ((int)uVar6 < 0x0)) {
    pbVar10 = pbVar9 + 0x1;
    *param_2 = '-';
    bVar13 = param_1 != 0x0;
    param_1 = -param_1;
    uVar6 = -(uVar6 + bVar13);
    pbVar8 = pbVar10;
  }
  do {
    uVar7 = 0x0;
    uVar5 = uVar6;
    if (uVar6 != 0x0) {
      uVar5 = uVar6 / param_3;
      uVar7 = uVar6 % param_3;
    }
    uVar2 = CONCAT22(uVar7,param_1);
    param_1 = (u16)(uVar2 / param_3);
    cVar4 = (char)(uVar2 % (u32)param_3);
    bVar3 = cVar4 + 0x30;
    if (0x39 < bVar3) {
      bVar3 = cVar4 + 0x57;
    }
    pbVar11 = pbVar10 + 0x1;
    *pbVar10 = bVar3;
    uVar6 = uVar5;
    pbVar10 = pbVar11;
  } while ((uVar5 | param_1) != 0x0);
  *pbVar11 = 0x0;
  do {
    pbVar11 = pbVar11 + -0x1;
    pbVar1 = pbVar11;
    bVar3 = *pbVar1;
    *pbVar1 = *pbVar8;
    *pbVar8 = bVar3;
    pbVar10 = pbVar8 + 0x2;
    pbVar8 = pbVar8 + 0x1;
  } while (pbVar10 < pbVar11);
  return pbVar9;
}



void fatal_app_exit_1000_3e9e(void)

{
  FatalAppExit16(s_ABNORMAL_PROGRAM_TERMINATION_1050_6544,0x0);
  return;
}



i16 pass1_1000_3ec0(u16 param_1,u16 param_2)

{
  u16 uVar1;
  u16 uVar2;
  u16_t uVar3;
  u16_t unaff_SI;
  u16_t uVar4;
  u32 *puVar4;

  puVar4 = (u32 *)CONCAT22(PTR_LOOP_1050_5fc0,PTR_LOOP_1050_5fbe);
  if ((((u16)PTR_LOOP_1050_5fc0 | (u16)PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,param_1));
    while( true ) {
      uVar4 = (u16_t)((u32)puVar4 >> 0x10);
      uVar3 = (u16_t)puVar4;
      if (((uVar3 + 0x2) | puVar4) == 0x0) break;
      uVar2 = str_op_1000_3da4((char *)CONCAT22((uVar3 + 0x2),puVar4));
      if (((uVar1 < uVar2) && (*(char *)((int)*puVar4 + uVar1) == '=')) &&
         (uVar2 = pass1_1000_3de8((char *)CONCAT22((uVar3 + 0x2),puVar4),
                                  (char *)CONCAT22(param_2,param_1),uVar1,unaff_SI,uVar3), uVar2 == 0x0)) {
        return puVar4 + uVar1 + 0x1;
      }
      puVar4 = (u32 *)((u32)puVar4 & 0xffff0000 | (u32)(uVar3 + 0x4));
    }
  }
  return 0x0;
}



i16 pass1_1000_3f5c(void)

{
  u16 uVar1;
  u16 *puVar2;
  i16 iVar3;

  iVar3 = 0x0;
  if (u16_1050_61ec == 0x0) {
    puVar2 = (u16 *)&PTR_LOOP_1050_6210;
  }
  else {
    puVar2 = (u16 *)0x6234;
  }
  for (; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
    uVar1 = pass1_1000_2a00(puVar2);
    if (uVar1 != 0xffff) {
      iVar3 += 0x1;
    }
  }
  return iVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 sys_1000_3f9c(char *param_1,char *param_2,u16 param_3)

{
  char *pcVar1;
  u16 uVar2;
  u16 unaff_CS;
  u16 local_4;

  PTR_LOOP_1050_68b2._0_1_ = 0x42;
  _u16_1050_68a8 = param_1;
  PTR_LOOP_1050_68ac = (u8 *)0x7fff;
  _PTR_LOOP_1050_68ae = param_1;
  uVar2 = FUN_1000_30b4();
  pcVar1 = _u16_1050_68a8;
  PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
  if ((int)PTR_LOOP_1050_68ac < 0x0) {
    mem_1000_2bb6(param_1._2_2_,0x0,(i16 *)&u16_1050_68a8);
  }
  else {
    _u16_1050_68a8 = (char *)((u32)_u16_1050_68a8 & 0xffff0000 | (u32)(u16_1050_68a8 + 0x1));
    *pcVar1 = '\0';
  }
  PTR_LOOP_1050_68b0 = (u8 *)((u32)_PTR_LOOP_1050_68ae >> 0x10);
  return uVar2;
}



uchar * pass1_1000_400a(i16 param_1)

{
  uchar *puVar1;

  if ((param_1 < 0x0) || ((int)PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
    PTR_LOOP_1050_5f78 = (u8 *)((int)&u16_1050_0008 + 0x1);
    puVar1 = (uchar *)0xffff;
  }
  else if (((u16_1050_61ec == 0x0) || ((param_1 < (int)u16_1050_5f8a && (0x2 < param_1)))) &&
          (0x31d < CONCAT11(DAT_1050_5f83,DAT_1050_5f82))) {
    puVar1 = PTR_LOOP_1050_5f88;
    if (((*(u8 *)(param_1 + 0x5f90) & 0x1) == 0x0) || (puVar1 = (uchar *)dos3_call_1000_5174(), puVar1 != NULL)) {
      PTR_LOOP_1050_5f88 = puVar1;
      PTR_LOOP_1050_5f78 = (u8 *)((int)&u16_1050_0008 + 0x1);
      puVar1 = (uchar *)0xffff;
    }
  }
  else {
    puVar1 = NULL;
  }
  return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)

void free_mem_1000_407a(u16 param_1,u16 param_2)

{
  GlobalFree16((HGLOBAL16)&DAT_1050_1050);
  return;
}



i16 * mixed_sys_op_1000_40af(u16 param_1,i16 param_2,u16 param_3)

{
  struct astruct_824 **ppaVar1;
  char *pcVar2;
  u16 *puVar4;
  char *pcVar5;
  i16 iVar6;
  struct astruct_824 **ppaVar7;
  u16 uVar7;
  u16 uVar8;
  i16 iVar8;
  HGLOBAL16 hglobal_7;
  void *SVar8;
  struct astruct_824 **ppaVar8;
  i16 unaff_SI;
  i16 *piVar9;
  char *pcVar10;
  struct astruct_824 *hglobal_di;
  u16 *puVar11;
  u8 unaff_CS;
  u16 unaff_SS;
  bool bVar12;
  void *pvVar13;
  struct astruct_825 *paVar14;
  u16 *puVar3;
  u8 uVar13;
  u8 uVar14;
  i16 iVar12;
  struct astruct_824 *temp_5fa27366cb;

  do {
    uVar7 = (u16)((u32)param_1 * (u32)param_3);
    uVar8 = param_2 * param_3 + (int)((u32)param_1 * (u32)param_3 >> 0x10);
    if ((uVar8 | uVar7) != 0x0) {
      piVar9 = NULL;
      if ((uVar8 < 0x3) && ((uVar8 < 0x2 || (uVar7 == 0x0)))) {
        if (uVar8 == 0x0) {
          uVar7 = uVar7 + 0xfff & 0xf000;
          if (uVar7 == 0x0) {
            uVar8 = 0x1;
          }
        }
        else if ((param_3 - 0x1 & param_3) != 0x0) {
          piVar9 = (i16 *)(((u32)uVar8 << 0x10) % (u32)param_3);
          bVar12 = CARRY2(uVar7,(u16)piVar9);
          uVar7 += (int)piVar9;
          if (bVar12) goto LAB_1000_41aa;
          uVar8 = 0x1;
        }
      }
      else if ((param_3 - 0x1 & param_3) != 0x0) goto LAB_1000_41aa;
      hglobal_7 = GLobalAlloc16(CONCAT13((char)(uVar8 >> 0x8),CONCAT12((char)uVar8,uVar7)),0x20);
      if ((hglobal_7 != 0x0) && ((uVar7 & 0x1) != 0x0)) {
        pvVar13 = WIN16_GlobalLock16(hglobal_7);
        SVar8._0_2_ = (int)pvVar13;
        if (((int)SVar8 != 0x0) || (pvVar13 == NULL)) {
          paVar14 = (astruct_825 *)CONCAT22(hglobal_7,0x12);
          uVar13 = '\x12';
          uVar14 = '\0';
          pass1_1000_25a8();
          pass1_1000_2913(CONCAT11(uVar14,uVar13));
          pcVar5 = poss_str_op_1000_28dc(paVar14);
          if (pcVar5 == NULL) goto LAB_1000_28cb;
          iVar6 = 0x9;
          if (*pcVar5 == 'M') {
            iVar6 = 0xf;
          }
          pcVar5 = pcVar5 + iVar6;
          iVar6 = 0x22;
          pcVar10 = pcVar5;
          break;
        }
        hglobal_7 = pass1_1000_422a((int)((u32)pvVar13 >> 0x10),hglobal_7);
        if (hglobal_7 == 0x0) {
          GlobalUnlock16(uVar8);
          GlobalFree16((HGLOBAL16)hglobal_di);
          hglobal_7 = 0x0;
        }
      }
      unaff_CS = 0x38;
      if (hglobal_7 != 0x0) {
        puVar11 = NULL;
        for (; unaff_SI != 0x0; unaff_SI += -0x1) {
          for (iVar6 = -0x8000; iVar6 != 0x0; iVar6 += -0x1) {
            puVar3 = puVar11;
            puVar11 = puVar11 + 0x1;
            *puVar3 = 0x0;
          }
          hglobal_7 += 0x100;
        }
        if (hglobal_di != NULL) {
          for (; hglobal_di != NULL; hglobal_di = hglobal_di + -0x1) {
            puVar4 = puVar11;
            puVar11 = (u16 *)((int)puVar11 + 0x1);
            *(u8 *)puVar4 = 0x0;
          }
        }
        return piVar9;
      }
    }
LAB_1000_41aa:
    if ((u16_1050_618e | (u16)PTR_LOOP_1050_618c) == 0x0) {
      return NULL;
    }
    iVar8 = (*(code *)PTR_LOOP_1050_618c)(unaff_CS,param_3,param_1,param_2);
    if (iVar8 == 0x0) {
      return NULL;
    }
  } while( true );
  while( true ) {
    iVar6 += -0x1;
    pcVar2 = pcVar10;
    pcVar10 = pcVar10 + 0x1;
    if (*pcVar2 == '\r') break;
    if (iVar6 == 0x0) break;
  }
  pcVar10[-0x1] = '\0';
LAB_1000_28cb:
  FatalAppExit16((char *)CONCAT13(0x10,CONCAT12(0x50,pcVar5)),0x0);
  FatalExit();
  ppaVar8 = (astruct_824 **)&PTR_LOOP_1050_63fe;
  do {
    ppaVar1 = ppaVar8;
    ppaVar8 = ppaVar8 + 0x1;
    temp_5fa27366cb = *ppaVar1;
    ppaVar7 = ppaVar8;
    if ((temp_5fa27366cb == hglobal_di) || (ppaVar7 = (astruct_824 **)(temp_5fa27366cb + 0x1), ppaVar7 == NULL)) {
      return (i16 *)ppaVar7;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      ppaVar1 = ppaVar8;
      ppaVar8 = (astruct_824 **)((int)ppaVar8 + 0x1);
    } while (*(astruct_824 *)ppaVar1 != (astruct_824)0x0);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1000_41e0(i16 param_1)

{
  i16 *piStack6;

  piStack6 = (i16 *)CONCAT22(PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  while( true ) {
    if (PTR_LOOP_1050_6190 + ((u16)PTR_LOOP_1050_6194 & 0xfffc) <= (u8 *)piStack6) {
      return 0x0;
    }
    if (*piStack6 == param_1) break;
    piStack6 = (i16 *)((u32)piStack6 & 0xffff0000 | ZEXT24((u8 *)piStack6 + 0x4));
  }
  *piStack6 = 0x0;
  return ((u8 *)piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1000_422a(i16 param_1,u16 param_2)

{
  u8 *puVar1;
  u8 *puVar2;
  u8 *puVar3;
  u8 *puVar4;
  i16 *piStack6;

  piStack6 = (i16 *)CONCAT22(PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  while( true ) {
    if (PTR_LOOP_1050_6190 + ((u16)PTR_LOOP_1050_6194 & 0xfffc) <= (u8 *)piStack6) {
      puVar2 = PTR_LOOP_1050_6194 + 0x28;
      puVar4 = PTR_LOOP_1050_6192;
      puVar3 = (u8 *)
               pass1_1000_16aa((u16)PTR_LOOP_1050_6192,(u16 *)PTR_LOOP_1050_6190,(u16)PTR_LOOP_1050_6192,
                               (u16)puVar2);
      if (((u16)puVar4 | (u16)puVar3) == 0x0) {
        param_1 = 0x0;
      }
      else {
        puVar1 = puVar3 + ((u16)PTR_LOOP_1050_6194 & 0xfffc);
        piStack6 = (i16 *)CONCAT22(puVar4,puVar1);
        PTR_LOOP_1050_6190 = puVar3;
        PTR_LOOP_1050_6192 = puVar4;
        *piStack6 = param_1;
        (puVar1 + 0x2) = param_2;
        PTR_LOOP_1050_6194 = puVar2;
        pass1_1000_4906((StructD *)CONCAT22(puVar4,puVar1 + 0x4),NULL,0x24);
      }
      return param_1;
    }
    if (*piStack6 == 0x0) break;
    piStack6 = (i16 *)((u32)piStack6 & 0xffff0000 | ZEXT24((u8 *)piStack6 + 0x4));
  }
  ((u8 *)piStack6 + 0x2) = param_2;
  *piStack6 = param_1;
  return param_1;
}



// WARNING: Removing unreachable block (ram,0x10004311)

void dos3_call_set_struct_1000_42de(astruct_811 *param_1,astruct_810 *param_2,u16 *param_3)

{
  u16 uVar3;
  code *pcVar4;
  u16 uVar4;
  u16 uVar5;
  struct astruct_811 *iVar4;
  struct astruct_810 *iVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  bool bVar5;
  u32 uVar12;
  u16 uVar1;
  u16 uVar2;
  u16 uVar9;

  uVar6 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (astruct_811 *)param_1;
  uVar5 = iVar4->field2_0x2;
  uVar4 = iVar4->field3_0x4;
  uVar1 = iVar4->field6_0x8;
  uVar2 = iVar4->field7_0xa;
  uVar7 = (u16)((u32)param_3 >> 0x10);
  uVar3 = *param_3;
  uVar9 = ((int)param_3 + 0x6);
  bVar5 = false;
  pcVar4 = (code *)swi(0x21);
  uVar12 = (*pcVar4)();
  *param_3 = uVar3;
  ((int)param_3 + 0x6) = uVar9;
  uVar8 = (u16)((u32)param_2 >> 0x10);
  iVar5 = (astruct_810 *)param_2;
  param_2 = (u16)uVar12;
  iVar5->field2_0x2 = uVar5;
  iVar5->field3_0x4 = uVar4;
  iVar5->field4_0x6 = (int)(uVar12 >> 0x10);
  iVar5->field5_0x8 = uVar1;
  iVar5->field6_0xa = uVar2;
  if (bVar5) {
    pass1_1000_29af((u16)uVar12);
  }
  iVar5->field7_0xc = (u16)bVar5;
  return;
}



// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)

void dos3_call_op_1000_435c(u16 param_1,u16 *param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6)

{
  code *pcVar1;
  u16 uVar2;
  u16 in_CX;
  u16 uVar3;
  u16 extraout_DX;
  u16 extraout_DX_00;
  u16 extraout_DX_01;
  u16 uVar4;
  u16 unaff_SS;
  u16 uVar6;
  char cVar7;
  u16 uVar5;
  u16 in_stack_00000002;

  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&DAT_1050_1050);
  pcVar1 = (code *)swi(0x21);
  uVar3 = in_CX;
  uVar2 = extraout_DX;
  (*pcVar1)();
  uVar6 = extraout_DX_00 >> 0x8;
  cVar7 = (char)uVar3;
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)(uVar3 >> 0x8);
  uVar4 = extraout_DX_01;
  if ((uVar2 != extraout_DX_01) && (uVar4 = extraout_DX_01, cVar7 == '\x17')) {
    uVar3 = in_CX;
    uVar4 = uVar2;
  }
  uVar2 = pass1_1000_462e(uVar4,uVar3 - 0x7bc,uVar4 >> 0x8,uVar4 & 0xff,uVar6,param_1,(int)param_2);
  if (param_2._2_2_ != 0x0) {
    ((int)param_2 + 0x2) = uVar4;
    *param_2 = uVar2;
  }
  return;
}



void pass1_1000_43f0(u16_t param_1)

{
  if (PTR_LOOP_1050_68b4 == NULL) {
    pass1_1000_440c(param_1);
    PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1000_440c(u16 param_1)

{
  char cVar1;
  char *pcVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u32 uVar7;
  u16 uVar8;
  u16 uVar9;
  char *pcStack8;

  uVar3 = pass1_1000_3ec0(0x61ca,(u16)&DAT_1050_1050);
  pcStack8 = (char *)CONCAT22(param_1,uVar3);
  if (((param_1 | uVar3) != 0x0) && (_DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0,DAT_1050_61ce), *pcStack8 != '\0')) {
    str_op_1000_3dbe((char *)CONCAT22(PTR_DAT_1050_1050_1050_61de,PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc),
                     (char *)CONCAT22(param_1,uVar3),0x3);
    pcStack8 = (char *)CONCAT22(param_1,uVar3 + 0x3);
    cVar1 = *pcStack8;
    if (cVar1 == '-') {
      pcStack8 = (char *)CONCAT22(param_1,uVar3 + 0x4);
    }
    uVar5 = 0x0;
    uVar9 = 0x0;
    uVar8 = 0xe10;
    uVar3 = pass1_1000_3e2c((u32)pcStack8 & 0xffff | (u32)param_1 << 0x10);
    _DAT_1050_61ce = pass1_1000_52be(uVar3,uVar5,uVar8,uVar9);
    for (; (pcVar2 = pcStack8, *pcStack8 == '+' || (('/' < *pcStack8 && (*pcStack8 < ':'))));
        pcStack8 = (char *)((u32)pcStack8 & 0xffff0000 | (u32)((int)pcStack8 + 0x1))) {
    }
    if (*pcStack8 == ':') {
      uVar5 = 0x0;
      uVar9 = 0x0;
      uVar8 = 0x3c;
      pcStack8 = (char *)((u32)pcStack8 & 0xffff0000 | (u32)((int)pcStack8 + 0x1));
      uVar3 = pass1_1000_3e2c((u32)pcVar2 & 0xffff0000 | (u32)((int)pcStack8 + 0x1));
      uVar7 = pass1_1000_52be(uVar3,uVar5,uVar8,uVar9);
      uVar6 = (u16)(uVar7 >> 0x10);
      _DAT_1050_61ce = uVar7 + _DAT_1050_61ce;
      for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
          pcStack8 = (char *)((u32)pcStack8 & 0xffff0000 | (u32)((int)pcStack8 + 0x1))) {
      }
      if (*pcStack8 == ':') {
        pcStack8 = (char *)((u32)pcStack8 & 0xffff0000 | (u32)((int)pcStack8 + 0x1));
        iVar4 = pass1_1000_3e2c((u32)pcVar2 & 0xffff0000 | (u32)((int)pcStack8 + 0x1));
        _DAT_1050_61ce += CONCAT22(uVar6,iVar4);
        for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
            pcStack8 = (char *)((u32)pcStack8 & 0xffff0000 | (u32)((int)pcStack8 + 0x1))) {
        }
      }
    }
    PTR_LOOP_1050_61d0 = (u8 *)(_DAT_1050_61ce >> 0x10);
    if (cVar1 == '-') {
      _DAT_1050_61ce = CONCAT22(-(int)(PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0x0)),-DAT_1050_61ce);
    }
    DAT_1050_61d2 = (int)*pcStack8;
    if (DAT_1050_61d2 == 0x0) {
      *_PTR_PTR_1050_61e0 = '\0';
    }
    else {
      str_op_1000_3dbe(_PTR_PTR_1050_61e0,pcStack8,0x3);
    }
  }
  PTR_LOOP_1050_61d0 = (u8 *)(_DAT_1050_61ce >> 0x10);
  return;
}



u16 pass1_1000_455a(u16 param_1,u16 param_2)

{
  i16 *piVar1;
  i16 iVar2;
  u16 uVar3;
  i16 iVar4;
  u16 UVar5;
  u32 uVar6;
  i16 iStack6;

  if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8)))
  goto LAB_1000_4623;
  if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
    uVar3 = (param_1 + 0xa);
    if (((int)uVar3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
      iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
    }
    else {
      iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
    }
    if ((uVar3 & 0x3) == 0x0) {
      iStack6 += 0x1;
    }
    uVar3 = (uVar3 - 0x46) * 0x16d + ((int)(uVar3 - 0x1) >> 0x2) + iStack6;
    uVar6 = pass1_1000_52f0(uVar3 - 0xd,((int)uVar3 >> 0xf) - (u16)(uVar3 < 0xd),0x7,0x0);
    iStack6 = (int)uVar6 - iStack6;
    iVar4 = -iStack6;
    if ((param_1 + 0x8) == 0x3) {
      iVar2 = (param_1 + 0xe);
      if ((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < (param_1 + 0x4))))) goto LAB_1000_460e;
    }
    else {
      piVar1 = (i16 *)(param_1 + 0xe);
      iVar2 = *piVar1;
      if ((SBORROW2(*piVar1,iVar4) != iVar2 + iStack6 < 0x0) || ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1))))
      goto LAB_1000_460e;
    }
LAB_1000_4623:
    UVar5 = 0x0;
  }
  else {
LAB_1000_460e:
    UVar5 = 0x1;
  }
  return UVar5;
}



i16 pass1_1000_462e(u16_t param_1,u16 param_2,i16 param_3,u16 param_4,u16 param_5,u16 param_6,i16 param_7)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 UVar4;
  u16 uVar5;
  u16 uVar6;
  i16 unaff_BP;
  u16 uVar7;
  u32 uVar8;
  i16 iStack26;
  u8 local_16 [0x4];
  u16 uStack18;
  i16 iStack14;
  i16 iStack12;
  i16 iStack8;
  u16 local_4;
  i16 iStack2;
  u16 uVar10;
  u16 uVar11;
  u16 uVar12;
  u16 uVar13;

  iStack2 = unaff_BP + 0x1;
  local_4 = (u16)&DAT_1050_1050;
  uVar7 = (param_3 * 0x2 + 0x61ae);
  if (((param_2 & 0x3) == 0x0) && (0x2 < param_3)) {
    uVar7 += 0x1;
  }
  pass1_1000_43f0(param_1);
  uVar13 = 0x0;
  uVar12 = 0x3c;
  uVar11 = 0x0;
  uVar10 = 0x3c;
  uVar1 = (u16)((long)(int)param_2 * 0x16d);
  uVar2 = (int)(param_2 + 0x3) >> 0x2;
  uVar3 = uVar2 + param_4;
  uVar5 = uVar1 + uVar3;
  uVar6 = uVar5 + uVar7;
  uVar8 = pass1_1000_52be(uVar6 + 0xe44,
                          (int)((u32)((long)(int)param_2 * 0x16d) >> 0x10) +
                          ((int)(param_2 + 0x3) >> 0xf) + ((int)param_4 >> 0xf) + (u16)CARRY2(uVar2,param_4) +
                          (u16)CARRY2(uVar1,uVar3) + ((int)uVar7 >> 0xf) + (u16)CARRY2(uVar5,uVar7) +
                          (u16)(0xf1bb < uVar6),0x18,0x0);
  uVar8 = pass1_1000_52be((u16)(uVar8 + (long)(int)param_5),(u16)(uVar8 + (long)(int)param_5 >> 0x10),uVar10,uVar11);
  uVar8 = pass1_1000_52be((u16)(uVar8 + (long)(int)param_6),(u16)(uVar8 + (long)(int)param_6 >> 0x10),uVar12,uVar13);
  iStack26 = (int)(uVar8 + (long)param_7 + CONCAT22(PTR_LOOP_1050_61d0,DAT_1050_61ce));
  iStack8 = param_4 + uVar7;
  iStack12 = param_2 + 0x50;
  iStack14 = param_3 + -0x1;
  uStack18 = param_5;
  if (DAT_1050_61d2 != 0x0) {
    UVar4 = pass1_1000_455a((u16)local_16,(u16)&DAT_1050_1050);
    if (UVar4 != 0x0) {
      iStack26 += -0xe10;
    }
  }
  return iStack26;
}



char * pass1_1000_472c(u32 param_1,char param_2)

{
  char *pcVar1;
  u16 uVar2;
  char *pcVar3;
  char *pcVar4;
  u16 uVar5;
  bool bVar6;

  uVar5 = (u16)(param_1 >> 0x10);
  pcVar3 = (char *)param_1;
  bVar6 = true;
  uVar2 = 0xffff;
  pcVar4 = pcVar3;
  do {
    if (uVar2 == 0x0) break;
    uVar2 -= 0x1;
    pcVar1 = pcVar4;
    pcVar4 = pcVar4 + 0x1;
    bVar6 = *pcVar1 == '\0';
  } while (!bVar6);
  uVar2 = ~uVar2;
  do {
    if (uVar2 == 0x0) break;
    uVar2 -= 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar6 = param_2 == *pcVar1;
  } while (!bVar6);
  if (!bVar6) {
    if (param_2 != '\0') {
      return NULL;
    }
    pcVar3 = pcVar3 + 0x1;
  }
  return pcVar3 + -0x1;
}



i16 pass1_1000_475e(u32 param_1,u32 param_2)

{
  char *pcVar1;
  char cVar2;
  char cVar3;
  u8 bVar4;
  struct astruct_235 *bVar3;
  i16 bVar5;
  char *pcVar5;
  char *pcVar6;

  pcVar6 = (char *)param_2;
  pcVar5 = (char *)param_1;
  bVar5 = 0xff;
  do {
    do {
      cVar3 = (char)bVar5;
      if (cVar3 == '\0') goto LAB_1000_479d;
      pcVar1 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
      cVar3 = *pcVar1;
      cVar2 = *pcVar5;
      bVar5 = CONCAT11(cVar2,cVar3);
      pcVar5 = pcVar5 + 0x1;
    } while (cVar2 == cVar3);
    bVar4 = cVar3 + 0xbfU + (-((u8)(cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
    bVar3._0_1_ = cVar2 + 0xbf;
    bVar5._0_1_ = (u8)bVar3 + (-((u8)bVar3 < 0x1a) & 0x20U) + 0x41;
    bVar5 = CONCAT11(bVar4,(u8)bVar5);
  } while ((u8)bVar5 == bVar4);
  cVar3 = ((u8)bVar5 < bVar4) * -0x2 + '\x01';
LAB_1000_479d:
  return (int)cVar3;
}



u16 pass1_1000_47a4(u32 param_1,u32 param_2)

{
  u8 *pbVar1;
  u8 bVar2;
  u16 *puVar3;
  u8 *pbVar4;
  i16 iVar5;
  u8 *pbVar6;
  u16 *puVar7;
  u16 uVar8;
  u16 local_22 [0x10];

  puVar7 = local_22;
  for (iVar5 = 0x10; iVar5 != 0x0; iVar5 += -0x1) {
    puVar3 = puVar7;
    puVar7 = puVar7 + 0x1;
    *puVar3 = 0x0;
  }
  pbVar6 = (u8 *)param_2;
  while( true ) {
    pbVar1 = pbVar6;
    pbVar6 = pbVar6 + 0x1;
    bVar2 = *pbVar1;
    if (bVar2 == 0x0) break;
    pbVar1 = (u8 *)((int)local_22 + (u16)(bVar2 >> 0x3));
    *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
  }
  pbVar1 = (u8 *)param_1;
  if (param_1 == 0x0) {
    pbVar1 = pbRam105061e4;
  }
  do {
    pbRam105061e4 = pbVar1;
    uVar8 = (u16)((u32)pbRam105061e4 >> 0x10);
    pbVar6 = (u8 *)((int)pbRam105061e4 + 0x1);
    bVar2 = *pbRam105061e4;
    if (bVar2 == 0x0) {
      return 0x0;
    }
    pbVar1 = (u8 *)((u32)pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
  } while (('\x01' << (bVar2 & 0x7) & *(u8 *)((int)local_22 + (u16)(bVar2 >> 0x3))) != 0x0);
  do {
    pbVar4 = pbVar6;
    bVar2 = *pbVar4;
    if (bVar2 == 0x0) goto LAB_1000_483c;
    pbVar6 = pbVar4 + 0x1;
  } while (('\x01' << (bVar2 & 0x7) & *(u8 *)((int)local_22 + (u16)(bVar2 >> 0x3))) == 0x0);
  *pbVar4 = 0x0;
  pbVar4 = pbVar4 + 0x1;
LAB_1000_483c:
  pbRam105061e4 = (u8 *)((u32)pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
  return (u16)pbRam105061e4;
}



u16 pass1_1000_484c(u32 param_1,u32 param_2,u16 param_3)

{
  u8 *pbVar1;
  u8 *pbVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  u8 *pbVar6;
  u8 *pbVar7;
  i16 iVar8;
  bool bVar9;
  bool bVar10;

  if (param_3 == 0x0) {
    return 0x0;
  }
  do {
    iVar8 = (int)(param_2 >> 0x10);
    pbVar7 = (u8 *)param_2;
    iVar3 = (int)(param_1 >> 0x10);
    pbVar6 = (u8 *)param_1;
    uVar4 = ~(u16)pbVar7;
    uVar4 = ((param_3 - 0x1) - uVar4 & -(u16)(param_3 - 0x1 < uVar4)) + uVar4;
    uVar5 = ~(u16)pbVar6;
    uVar4 = (uVar4 - uVar5 & -(u16)(uVar4 < uVar5)) + uVar5 + 0x1;
    bVar9 = param_3 < uVar4;
    param_3 -= uVar4;
    bVar10 = param_3 == 0x0;
    do {
      if (uVar4 == 0x0) break;
      uVar4 -= 0x1;
      pbVar2 = pbVar7;
      pbVar7 = pbVar7 + 0x1;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar9 = *pbVar1 < *pbVar2;
      bVar10 = *pbVar1 == *pbVar2;
    } while (bVar10);
    param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
    if (!bVar10) {
      return (0x1 - (u16)bVar9) - (u16)(bVar9 != 0x0);
    }
    if (param_3 == 0x0) {
      return uVar4;
    }
    if (pbVar6 == NULL) {
      iVar3 += 0x6c;
    }
    param_1 = CONCAT22(iVar3,pbVar6);
    if (pbVar7 == NULL) {
      param_2 = (u32)(iVar8 + 0x6c) << 0x10;
      param_1 = CONCAT22(iVar3,pbVar6);
    }
  } while( true );
}



u16 pass1_1000_48a8(u32 param_1,u32 param_2,i16 param_3)

{
  u16 *puVar1;
  u16 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  u16 *puVar6;
  u16 *puVar7;
  i16 iVar8;

  if (param_3 != 0x0) {
    while( true ) {
      iVar3 = (int)(param_2 >> 0x10);
      puVar6 = (u16 *)param_2;
      iVar8 = (int)(param_1 >> 0x10);
      puVar7 = (u16 *)param_1;
      uVar4 = ~(u16)puVar7;
      uVar4 = ((param_3 - 0x1U) - uVar4 & -(u16)(param_3 - 0x1U < uVar4)) + uVar4;
      uVar5 = ~(u16)puVar6;
      uVar4 = (uVar4 - uVar5 & -(u16)(uVar4 < uVar5)) + uVar5 + 0x1;
      param_3 -= uVar4;
      for (uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
        puVar2 = puVar7;
        puVar7 = puVar7 + 0x1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        *puVar2 = *puVar1;
      }
      for (uVar4 = (u16)((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 -= 0x1) {
        puVar2 = puVar7;
        puVar7 = (u16 *)((int)puVar7 + 0x1);
        puVar1 = puVar6;
        puVar6 = (u16 *)((int)puVar6 + 0x1);
        *(u8 *)puVar2 = *(u8 *)puVar1;
      }
      if (param_3 == 0x0) break;
      if (puVar6 == NULL) {
        iVar3 += 0x6c;
      }
      param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
      param_2 = CONCAT22(iVar3,puVar6);
      if (puVar7 == NULL) {
        param_1 = (u32)(iVar8 + 0x6c) << 0x10;
        param_2 = CONCAT22(iVar3,puVar6);
      }
    }
  }
  return (u16)param_1;
}



u16 * pass1_1000_4906(StructD *param_1,WNDCLASS16 *in_wnd_class,u16 param_3)

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
    struct_1_hi = (astruct_20 *)((u32)param_1 >> 0x10);
    struct_1 = (astruct_20 *)-(int)(u16 *)param_1;
    uVar5 = param_3;
    if (struct_1 != NULL) {
      uVar5 = ((int)struct_1 - param_3 & -(u16)(struct_1 < param_3)) + param_3;
      struct_1 = (astruct_20 *)(param_3 - uVar5);
    }
    uVar3 = (u16)in_wnd_class & 0xff | (int)in_wnd_class << 0x8;
    puVar6 = (u16 *)param_1;
    for (uVar4 = uVar5 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar5 = (u16)((uVar5 & 0x1) != 0x0); uVar2 = (undefined)((u16)in_wnd_class & 0xff), uVar5 != 0x0;
        uVar5 -= 0x1) {
      puVar1 = puVar6;
      puVar6 = (u16 *)((int)puVar6 + 0x1);
      *(u8 *)puVar1 = uVar2;
    }
    if (struct_1 != NULL) {
      for (uVar5 = (u16)struct_1 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        *puVar1 = uVar3;
      }
      for (uVar5 = (u16)(((u16)struct_1 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
        puVar1 = puVar6;
        puVar6 = (u16 *)((int)puVar6 + 0x1);
        *(u8 *)puVar1 = uVar2;
      }
    }
  }
  return (u16 *)param_1;
}



i16 pass1_1000_49b2(u16 param_1)

{
  return (param_1 ^ (int)param_1 >> 0xf) - ((int)param_1 >> 0xf);
}



u16 pass1_1000_49c6(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6,uchar *param_7)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  i16 iVar5;
  i16 iVar6;
  u32 uVar7;
  u16 uStack20;
  u16 uStack18;
  u16 uStack8;
  u16 uStack6;

  uStack20 = param_3;
  uStack18 = param_4;
  uVar7 = pass1_1000_52be(param_5 - 0x1,-(u16)(param_5 == 0x0),param_6,0x0);
  uStack8 = (u16)(uVar7 + 0x8);
  uStack6 = (int)(uVar7 + 0x8 >> 0x10) * 0x100 + param_4;
  while( true ) {
    if (uStack6 < uStack18) {
      return 0x0;
    }
    if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
      return 0x0;
    }
    uVar1 = param_5 >> 0x1;
    if (uVar1 == 0x0) {
      if ((param_5 != 0x0) && (iVar5 = (*(code *)param_7)(), iVar5 == 0x0)) {
        return uStack20;
      }
      return 0x0;
    }
    uVar2 = uVar1;
    if ((param_5 & 0x1) == 0x0) {
      uVar2 = uVar1 - 0x1;
    }
    uVar3 = (u16)((u32)uVar2 * (u32)param_6);
    uVar4 = uVar3 + uStack20;
    iVar6 = ((int)((u32)uVar2 * (u32)param_6 >> 0x10) + (u16)CARRY2(uVar3,uStack20)) * 0x100 + uStack18;
    iVar5 = (*(code *)param_7)();
    if (iVar5 == 0x0) break;
    if (iVar5 < 0x0) {
      uStack8 = -param_6 + uVar4;
      uStack6 = ((u16)CARRY2(-param_6,uVar4) - (u16)(param_6 != 0x0)) * 0x100 + iVar6;
      uVar2 = param_5 & 0x1;
      param_5 = uVar1;
      if (uVar2 == 0x0) {
        param_5 = uVar1 - 0x1;
      }
    }
    else {
      uStack20 = param_6 + uVar4;
      uStack18 = (u16)CARRY2(param_6,uVar4) * 0x100 + iVar6;
      param_5 = uVar1;
    }
  }
  return uVar4;
}



void pass1_1000_4aea(u16 param_1,u16 param_2,i16 param_3,u16 param_4,uchar *param_5)

{
  u16 *puVar1;
  code **ppcVar2;
  i32 lVar3;
  u16 uVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  struct astruct_171 *puVar11;
  u16 uVar9;
  u16 uVar10;
  i16 unaff_DI;
  u16 uVar11;
  u16 unaff_CS;
  bool bVar12;
  u16 uStackY26;
  u16 uStackY24;
  u16 uStackY22;
  u16 uVar13;
  u16 uVar14;
  u16 uStackY18;
  u16 uStackY16;
  u16 uStackY14;

  if ((param_4 != 0x0) && (param_3 != 0x0)) {
    uStackY14 = param_1;
    uVar11 = param_2;
    for (iVar6 = param_3 + -0x1; iVar6 != 0x0; iVar6 += -0x1) {
      uVar9 = uStackY14 + param_4;
      uVar11 += -(u16)CARRY2(uStackY14,param_4) & 0x6c;
      uStackY18 = uVar9;
      uStackY16 = uVar11;
      iVar5 = (*(code *)param_5)();
      if (iVar5 < 0x0) {
        uVar11 = param_3 - 0x1;
        iVar6 = 0x0;
        do {
          uVar11 >>= 0x1;
          iVar6 += -0x1;
        } while (iVar6 != 0x0 && uVar11 != 0x0);
        if (((int)((u32)(u16)-iVar6 * 0x8 >> 0x10) != 0x0) ||
           (uVar11 = pass1_1000_3bac(), uVar11 < (u16)((u32)(u16)-iVar6 * 0x8))) {
          exit_1000_25f2(-0x4,0x4b7b,unaff_CS,unaff_DI);
          return;
        }
        puVar11 = (astruct_171 *)&stack0xfff6;
        lVar3 = (u32)(param_3 - 0x1) * (u32)param_4;
        uVar11 = (u16)lVar3;
        uStackY14 = uVar11 + param_1;
        uVar11 = ((int)((u32)lVar3 >> 0x10) + (u16)CARRY2(uVar11,param_1)) * 0x100 + param_2;
        uStackY16 = param_2;
        uStackY18 = param_1;
LAB_1000_4b7d:
        if (puVar11 <= (astruct_171 *)&uStackY18) {
          return;
        }
LAB_1000_4b81:
        if ((uStackY16 < uVar11) || ((uStackY16 <= uVar11 && (uStackY18 < uStackY14)))) {
          uStackY22 = uStackY14;
          puVar1 = &puVar11->field20_0x14;
          uVar8 = uStackY14 + *puVar1;
          uVar7 = uVar11 + (-(u16)CARRY2(uStackY14,*puVar1) & 0x6c);
          uVar9 = uStackY16;
          uVar10 = uStackY18;
          uStackY26 = uStackY18;
          uStackY24 = uStackY16;
          uVar13 = uVar11;
LAB_1000_4bbc:
          do {
            puVar1 = &puVar11->field20_0x14;
            bVar12 = CARRY2(uVar10,*puVar1);
            uVar10 += *puVar1;
            uVar9 += -(u16)bVar12 & 0x6c;
            uVar4 = uStackY22;
            if ((uVar10 != uStackY14) || (uVar9 != uVar11)) {
              ppcVar2 = (code **)&puVar11->field21_0x16;
              iVar6 = (**ppcVar2)();
              if (iVar6 < 0x1) {
                if (iVar6 != 0x0) {
                  uStackY26 = uVar10;
                  uStackY24 = uVar9;
                }
                goto LAB_1000_4bbc;
              }
            }
            do {
              uVar14 = uVar13;
              uStackY22 = uVar4;
              puVar1 = &puVar11->field20_0x14;
              bVar12 = uVar8 < *puVar1;
              uVar8 -= *puVar1;
              uVar7 -= -(u16)bVar12 & 0x6c;
              ppcVar2 = (code **)&puVar11->field21_0x16;
              iVar6 = (**ppcVar2)();
              if (0x0 < iVar6) break;
              uVar4 = uVar8;
              uVar13 = uVar7;
            } while (((iVar6 != 0x0) || (uVar4 = uStackY22, uVar13 = uVar14, uVar8 != uStackY18)) ||
                    (uVar7 != uStackY16));
            if ((uVar7 < uVar9) || ((uVar7 <= uVar9 && (uVar8 <= uVar10)))) goto LAB_1000_4c58;
            pass1_1000_4ceb(puVar11->field20_0x14);
            uStackY26 = uVar10;
            uStackY24 = uVar9;
            uVar13 = uVar7;
            uStackY22 = uVar8;
          } while( true );
        }
        goto LAB_1000_4b7d;
      }
      uStackY14 = uVar9;
    }
  }
  return;
LAB_1000_4c58:
  pass1_1000_4ceb(puVar11->field20_0x14);
  uVar10 = ((uVar11 - (-(u16)(uStackY14 < uStackY22) & 0x6c)) - uVar14) +
           (-(u16)CARRY2(uStackY14 - uStackY22,uStackY18) & 0x6c) + uStackY16;
  uVar9 = -(u16)((uStackY14 - uStackY22) + uStackY18 < uStackY26) & 0x6c;
  if ((uVar10 < uVar9) || (uVar10 - uVar9 < uStackY24)) {
    uStackY14 = uStackY26;
    uVar11 = uStackY24;
  }
  else {
    uStackY18 = uStackY22;
    uStackY16 = uVar14;
  }
  goto LAB_1000_4b81;
}



void pass1_1000_4ceb(u16 param_1)

{
  u8 *puVar1;
  u16 *puVar2;
  u8 uVar3;
  u16 uVar4;
  i16 unaff_SI;
  i16 unaff_DI;
  u16 unaff_ES;

  if ((param_1 & 0x1) != 0x0) {
    param_1 -= 0x1;
    puVar1 = (u8 *)(param_1 + unaff_DI);
    uVar3 = *puVar1;
    *puVar1 = *(u8 *)(param_1 + unaff_SI);
    *(u8 *)(param_1 + unaff_SI) = uVar3;
    if (param_1 == 0x0) {
      return;
    }
  }
  do {
    param_1 -= 0x2;
    puVar2 = (u16 *)(param_1 + unaff_DI);
    uVar4 = *puVar2;
    *puVar2 = (param_1 + unaff_SI);
    (param_1 + unaff_SI) = uVar4;
  } while (param_1 != 0x0);
  return;
}



void pass1_1000_4d0c(u16 param_1)

{
  DAT_1050_61e8 = param_1;
  PTR_LOOP_1050_61ea = NULL;
  return;
}



u16 pass1_1000_4d24(void)

{
  u32 uVar1;

  uVar1 = pass1_1000_52be(DAT_1050_61e8,(u16)PTR_LOOP_1050_61ea,(int)s_TPPOPMENU_1050_43fa + 0x3,0x3);
  PTR_LOOP_1050_61ea = (u8 *)(uVar1 + 0x269ec3 >> 0x10);
  DAT_1050_61e8 = (int)(uVar1 + 0x269ec3);
  return (u16)PTR_LOOP_1050_61ea & 0x7fff;
}



void str_1000_4d58(char *in_string_1,char *in_string_2,u32 param_3,u32 param_4,WNDCLASS16 *param_5)

{
  u16 uVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  char *pcStack18;
  u16 uStack12;
  u16 uStack10;
  u16 uStack8;
  u16 uStack6;

  uStack10 = 0x0;
  uStack12 = 0x0;
  uVar4 = (u16)((u32)in_string_1 >> 0x10);
  iVar2 = (int)in_string_1;
  if ((*in_string_1 == '\0') || (*(char *)(iVar2 + 0x1) != ':')) {
    if (in_string_2 != NULL) {
      *in_string_2 = '\0';
    }
  }
  else {
    if (in_string_2 != NULL) {
      *in_string_2 = *in_string_1;
      *(u8 *)((int)in_string_2 + 0x1) = *(u8 *)(iVar2 + 0x1);
      *(u8 *)((int)in_string_2 + 0x2) = 0x0;
    }
    in_string_1 = (char *)((u32)in_string_1 & 0xffff0000 | (u32)(iVar2 + 0x2));
  }
  uStack6 = 0x0;
  uStack8 = 0x0;
  pcStack18 = in_string_1;
  while( true ) {
    uVar5 = (u16)((u32)pcStack18 >> 0x10);
    uVar3 = (u16)pcStack18;
    if (*pcStack18 == '\0') break;
    if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
      uStack8 = uVar3 + 0x1;
      uStack6 = uVar5;
    }
    else if (*pcStack18 == '.') {
      uStack12 = uVar3;
      uStack10 = uVar5;
    }
    pcStack18 = (char *)((u32)pcStack18 & 0xffff0000 | (u32)(uVar3 + 0x1));
  }
  if ((uStack6 | uStack8) == 0x0) {
    if (param_3 != 0x0) {
      *(u8 *)param_3 = 0x0;
    }
  }
  else {
    if (param_3 != 0x0) {
      uVar1 = uStack8 - (u16)in_string_1;
      if (0xff < (int)uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((char *)(param_3 & 0xffff | (u32)param_3._2_2_ << 0x10),in_string_1,uVar1);
      *(u8 *)((int)param_3 + uVar1) = 0x0;
    }
    in_string_1 = (char *)CONCAT22(uStack6,uStack8);
  }
  if (((uStack10 | uStack12) != 0x0) && ((u16)in_string_1 <= uStack12)) {
    if (param_4 != 0x0) {
      uVar1 = uStack12 - (u16)in_string_1;
      if (0xff < (int)uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((char *)(param_4 & 0xffff | (u32)param_4._2_2_ << 0x10),
                       (char *)((u32)in_string_1 & 0xffff | (u32)in_string_1._2_2_ << 0x10),uVar1);
      *(u8 *)((int)param_4 + uVar1) = 0x0;
    }
    if (param_5 == NULL) {
      return;
    }
    uVar1 = uVar3 - uStack12;
    if (0xff < (int)uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((char *)((u32)param_5 & 0xffff | (u32)param_5._2_2_ << 0x10),
                     (char *)CONCAT22(uStack10,uStack12),uVar1);
    *(u8 *)((int)param_5 + uVar1) = 0x0;
    return;
  }
  if (param_4 != 0x0) {
    uVar1 = uVar3 - (u16)in_string_1;
    if (0xff < (int)uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((char *)(param_4 & 0xffff | (u32)param_4._2_2_ << 0x10),
                     (char *)((u32)in_string_1 & 0xffff | (u32)in_string_1._2_2_ << 0x10),uVar1);
    *(u8 *)((int)param_4 + uVar1) = 0x0;
  }
  if (param_5 != NULL) {
    *(u8 *)&param_5->style = 0x0;
  }
  return;
}


/*
Unable to decompile 'pass1_1000_4f1a'
Cause:
Low-level Error: Symbol $$undef00000008 extends beyond the end of the address space
*/


// WARNING: Removing unreachable block (ram,0x10004f47)

u16 dos3_call_1000_4f20(void)

{
  code *pcVar1;
  u16 uVar2;
  i16 unaff_BP;
  bool bVar2;

  bVar2 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)((int)&DAT_1050_1050,unaff_BP + 0x1);
  if (bVar2) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f47)

u16 pass1_1000_4f2e(void)

{
  code *pcVar1;
  u16 uVar2;
  i16 unaff_BP;
  bool bVar3;

  bVar3 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)((int)&DAT_1050_1050,unaff_BP + 0x1);
  if (bVar3) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f6d)

u16 dos3call_1000_4f54(u32 param_1)

{
  char cVar1;
  code *pcVar2;
  u16 uVar3;
  i16 unaff_BP;
  bool bVar3;
  u32 uVar5;

  bVar3 = false;
  pcVar2 = (code *)swi(0x21);
  uVar5 = (*pcVar2)((int)&DAT_1050_1050,unaff_BP + 0x1);
  uVar5._2_2_ = (char *)(uVar5 >> 0x10);
  uVar5._0_2_ = (u16)uVar5;
  uVar3 = (u16)uVar5;
  if ((bVar3) && (bVar3 = (u16)uVar5 < 0x10, (u16)uVar5 == 0x10)) {
    do {
      cVar1 = *uVar5._2_2_;
      uVar5._2_2_ = uVar5._2_2_ + 0x1;
      if (cVar1 == '\0') goto LAB_1000_4f90;
    } while ((cVar1 != '?') && (cVar1 != '*'));
    uVar3 = 0x3;
LAB_1000_4f90:
    bVar3 = true;
  }
  if (!bVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar3);
  return 0xffff;
}



// WARNING: Removing unreachable block (ram,0x10004fa9)

i16 dos3_call_1000_4f94(void)

{
  code *pcVar1;
  u8 bVar2;
  i16 unaff_BP;

  pcVar1 = (code *)swi(0x21);
  bVar2 = (*pcVar1)(unaff_BP + 0x1);
  return bVar2 + 0x1;
}



// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)

u16 dos3_call_1000_4fbe(u8 param_1)

{
  code *pcVar1;
  char cVar2;
  u16 uVar3;
  i16 unaff_BP;

  pcVar1 = (code *)swi(0x21);
  (*pcVar1)(unaff_BP + 0x1);
  pcVar1 = (code *)swi(0x21);
  cVar2 = (*pcVar1)();
  uVar3 = 0xffff;
  if ((u8)(cVar2 + '\x01') == param_1) {
    uVar3 = 0x0;
  }
  return uVar3;
}



void pass1_1000_5008(u16 param_1,u16 param_2,u16 param_3)

{
  pass1_1000_5026(0x0,param_1,param_2,param_3);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1000_5026(i16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;
  u16 uVar2;
  i16 unaff_BP;
  u32 uStack304;
  u16 local_12c [0x3];
  u16 uStack294;
  u8 *local_124 [0x6];
  i16 iStack280;
  u8 local_116;
  u8 uStack277;
  char cStack272;
  u8 *puStack270;
  u8 local_108;
  u8 uStack263;
  u8 uStack262;
  u8 auStack261 [0x101];
  u16 local_4;
  i16 iStack2;

  iStack2 = unaff_BP + 0x1;
  local_4 = SUB42(&DAT_1050_1050,0x0);
  uStack304 = (char *)CONCAT22(0x1050,&local_108);
  if (param_1 == 0x0) {
    param_1 = dos3_call_1000_4f94();
  }
  *uStack304 = (char)param_1 + '@';
  uStack263 = 0x3a;
  puStack270 = auStack261;
  uStack262 = 0x5c;
  uStack277 = 0x47;
  cStack272 = (char)param_1;
  local_12c[0] = SUB42(&DAT_1050_1050,0x0);
  uStack294 = SUB42(&DAT_1050_1050,0x0);
  dos3_call_set_struct_1000_42de
            ((astruct_811 *)CONCAT22(0x1050,&local_116),(astruct_810 *)CONCAT22(0x1050,local_124),
             (u16 *)CONCAT22(0x1050,local_12c));
  if (iStack280 == 0x0) {
    uVar1 = str_op_1000_3da4((char *)CONCAT22(0x1050,&local_108));
    uVar1 += 0x1;
    uStack304._0_2_ = param_2;
    uStack304._2_2_ = param_3;
    uVar2 = param_3 | param_2;
    if (uVar2 == 0x0) {
      if ((int)param_4 < (int)uVar1) {
        param_4 = uVar1;
      }
      uStack304._0_2_ = mem_1000_167a(0x0,param_4);
      uStack304._2_2_ = uVar2;
      if ((uVar2 | (u16)uStack304) == 0x0) {
        PTR_LOOP_1050_5f78 = (u8 *)&PTR_LOOP_1050_000c;
        return;
      }
    }
    if ((int)param_4 < (int)uVar1) {
      PTR_LOOP_1050_5f78 = (u8 *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x2);
    }
    else {
      unk_str_op_1000_3d3e((char *)CONCAT22(uStack304._2_2_,(u16)uStack304),(char *)CONCAT22(0x1050,&local_108));
    }
  }
  else {
    PTR_LOOP_1050_5f78 = (u8 *)((int)&PTR_LOOP_1050_000c + 0x1);
    PTR_LOOP_1050_5f88 = local_124[0];
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10005167)

u16 dos3_call_1000_514e(void)

{
  code *pcVar1;
  u16 uVar2;
  i16 unaff_BP;
  bool bVar2;

  bVar2 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)((int)&DAT_1050_1050,unaff_BP + 0x1);
  if (bVar2) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000518c)

u16 dos3_call_1000_5174(void)

{
  code *pcVar1;
  u16 uVar2;
  i16 unaff_BP;
  bool bVar2;

  bVar2 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)(unaff_BP + 0x1);
  if (!bVar2) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return uVar2 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

u16 dos3_calls_1000_5198(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  code *pcVar1;

  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&DAT_1050_1050);
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  if ((param_2 & 0x100) == 0x0) {
    return 0x0;
  }
  pass1_1000_29b5(param_3);
  return param_3 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

u16 dos3_call_1000_51aa(u16 param_1,u16 param_2,u16 param_3)

{
  code *pcVar1;
  u16 uStack000a;

  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&DAT_1050_1050);
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  if ((param_2 & 0x100) == 0x0) {
    return 0x0;
  }
  uStack000a = param_3;
  pass1_1000_29b5(param_3);
  return uStack000a & 0xff;
}



u32 pass1_1000_5224(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u32 uVar1;
  i32 lVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  bool bVar10;
  char cVar11;
  u16 uVar9;

  cVar11 = (int)param_2 < 0x0;
  if ((bool)cVar11) {
    bVar10 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -(u16)bVar10 - param_2;
  }
  if ((int)param_4 < 0x0) {
    cVar11 += '\x01';
    bVar10 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -(u16)bVar10 - param_4;
  }
  uVar3 = param_1;
  uVar5 = param_3;
  uVar6 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = (int)(((u32)param_2 % (u32)param_3 << 0x10 | (u32)param_1) / (u32)param_3);
  }
  else {
    do {
      uVar8 = uVar9 >> 0x1;
      uVar5 = uVar5 >> 0x1 | (u16)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (u16)((uVar6 & 0x1) != 0x0) << 0xf;
      uVar6 = uVar7;
      uVar9 = uVar8;
    } while (uVar8 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (u32)uVar5;
    iVar4 = (int)uVar1;
    lVar2 = (u32)param_3 * (uVar1 & 0xffff);
    uVar3 = (u16)((u32)lVar2 >> 0x10);
    uVar5 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 < (u16)lVar2)))) {
      iVar4 += -0x1;
    }
    uVar3 = 0x0;
  }
  if (cVar11 == '\x01') {
    bVar10 = iVar4 != 0x0;
    iVar4 = -iVar4;
    uVar3 = -(u16)bVar10 - uVar3;
  }
  return CONCAT22(uVar3,iVar4);
}



u32 pass1_1000_52be(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  if ((param_4 | param_2) == 0x0) {
    return (u32)param_1 * (u32)param_3;
  }
  return (u32)param_1 * (u32)param_3 & 0xffff |
         (u32)((int)((u32)param_1 * (u32)param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}



u32 pass1_1000_52f0(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u32 uVar1;
  i32 lVar2;
  u16 uVar3;
  u16 uVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  bool bVar12;
  bool bVar13;

  bVar13 = (int)param_2 < 0x0;
  if (bVar13) {
    bVar12 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -(u16)bVar12 - param_2;
  }
  uVar11 = (u16)bVar13;
  if ((int)param_4 < 0x0) {
    bVar13 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -(u16)bVar13 - param_4;
  }
  uVar3 = param_1;
  uVar4 = param_3;
  uVar8 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    iVar5 = (int)(((u32)param_2 % (u32)param_3 << 0x10 | (u32)param_1) % (u32)param_3);
    iVar6 = 0x0;
    if ((int)(uVar11 - 0x1) < 0x0) goto LAB_1000_538a;
  }
  else {
    do {
      uVar10 = uVar9 >> 0x1;
      uVar4 = uVar4 >> 0x1 | (u16)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar8 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (u16)((uVar8 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar7;
      uVar9 = uVar10;
    } while (uVar10 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (u32)uVar4;
    uVar3 = (int)uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * (u32)param_3;
    uVar8 = (u16)((u32)lVar2 >> 0x10);
    uVar4 = (u16)lVar2;
    uVar9 = uVar8 + uVar3;
    if (((CARRY2(uVar8,uVar3)) || (param_2 < uVar9)) || ((param_2 <= uVar9 && (param_1 < uVar4)))) {
      bVar13 = uVar4 < param_3;
      uVar4 -= param_3;
      uVar9 = (uVar9 - param_4) - (u16)bVar13;
    }
    iVar5 = uVar4 - param_1;
    iVar6 = (uVar9 - param_2) - (u16)(uVar4 < param_1);
    if (-0x1 < (int)(uVar11 - 0x1)) goto LAB_1000_538a;
  }
  bVar13 = iVar5 != 0x0;
  iVar5 = -iVar5;
  iVar6 = -(u16)bVar13 - iVar6;
LAB_1000_538a:
  return CONCAT22(iVar6,iVar5);
}



u32 pass1_1000_5390(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u32 uVar1;
  i32 lVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;

  uVar3 = param_1;
  uVar8 = param_4;
  uVar6 = param_2;
  uVar9 = param_3;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = (int)(((u32)param_2 % (u32)param_3 << 0x10 | (u32)param_1) / (u32)param_3);
  }
  else {
    do {
      uVar5 = uVar8 >> 0x1;
      uVar9 = uVar9 >> 0x1 | (u16)((uVar8 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (u16)((uVar6 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar5;
      uVar6 = uVar7;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (u32)uVar9;
    iVar4 = (int)uVar1;
    lVar2 = (u32)param_3 * (uVar1 & 0xffff);
    uVar3 = (u16)((u32)lVar2 >> 0x10);
    uVar8 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 < (u16)lVar2)))) {
      iVar4 += -0x1;
    }
    uVar3 = 0x0;
  }
  return CONCAT22(uVar3,iVar4);
}



u32 pass1_1000_53f0(u16 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  u32 uVar1;
  i32 lVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  i16 iVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  bool bVar11;

  uVar3 = param_1;
  uVar4 = param_4;
  uVar9 = param_2;
  uVar10 = param_3;
  if (param_4 == 0x0) {
    iVar6 = (int)(((u32)param_2 % (u32)param_3 << 0x10 | (u32)param_1) % (u32)param_3);
    iVar7 = 0x0;
  }
  else {
    do {
      uVar5 = uVar4 >> 0x1;
      uVar10 = uVar10 >> 0x1 | (u16)((uVar4 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar9 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (u16)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar4 = uVar5;
      uVar9 = uVar8;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar8,uVar3) / (u32)uVar10;
    uVar3 = (int)uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * (u32)param_3;
    uVar9 = (u16)((u32)lVar2 >> 0x10);
    uVar4 = (u16)lVar2;
    uVar10 = uVar9 + uVar3;
    if (((CARRY2(uVar9,uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
      bVar11 = uVar4 < param_3;
      uVar4 -= param_3;
      uVar10 = (uVar10 - param_4) - (u16)bVar11;
    }
    iVar6 = -(uVar4 - param_1);
    iVar7 = -(u16)(uVar4 - param_1 != 0x0) - ((uVar10 - param_2) - (u16)(uVar4 < param_1));
  }
  return CONCAT22(iVar7,iVar6);
}



i16 pass1_1000_545a(u32 param_1,u32 param_2)

{
  u8 *pbVar1;
  u8 bVar2;
  u8 bVar3;
  u8 bVar4;
  u8 *pbVar5;
  u8 *pbVar6;

  pbVar6 = (u8 *)param_2;
  pbVar5 = (u8 *)param_1;
  bVar4 = 0xff;
  do {
    do {
      if (bVar4 == 0x0) goto LAB_1000_5499;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar4 = *pbVar1;
      bVar3 = *pbVar5;
      pbVar5 = pbVar5 + 0x1;
    } while (bVar3 == bVar4);
    bVar2 = bVar4 + 0xbf + (-((u8)(bVar4 + 0xbf) < 0x1a) & 0x20U) + 0x41;
    bVar3 += 0xbf;
    bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20U) + 0x41;
  } while (bVar4 == bVar2);
  bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;
LAB_1000_5499:
  return (int)(char)bVar4;
}



u16 * pass1_1000_54a0(u32 param_1,u16 param_2,u16 param_3)

{
  u16 *puVar1;
  u8 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 *puVar7;
  i16 iVar8;

  if (param_3 != 0x0) {
    iVar8 = (int)(param_1 >> 0x10);
    uVar5 = -(int)(u16 *)param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(u16)(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = param_2 & 0xff | param_2 << 0x8;
    puVar7 = (u16 *)param_1;
    for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar6 = (u16)((uVar6 & 0x1) != 0x0); uVar2 = (undefined)(param_2 & 0xff), uVar6 != 0x0; uVar6 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = (u16 *)((int)puVar7 + 0x1);
      *(u8 *)puVar1 = uVar2;
    }
    if (uVar5 != 0x0) {
      for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = uVar3;
      }
      for (uVar6 = (u16)((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = (u16 *)((int)puVar7 + 0x1);
        *(u8 *)puVar1 = uVar2;
      }
    }
  }
  return (u16 *)param_1;
}



void pass1_1000_54e8(uchar *param_1,u16 param_2,i16 param_3,i16 param_4,i16 param_5,u16 param_6)

{
  i16 iVar1;

  iVar1 = param_3;
  while (iVar1 += -0x1, -0x1 < iVar1) {
    (*(code *)param_1)();
  }
  return;
}



void pass1_1000_5512(uchar *param_1,u16 param_2,i16 param_3,i16 param_4,u16 param_5)

{
  bool bVar1;
  u16 uStack4;

  pass1_1000_52be(param_3,param_4,param_5,0x0);
  while( true ) {
    bVar1 = param_3 == 0x0;
    param_3 += -0x1;
    param_4 -= (u16)bVar1;
    if (param_4 < 0x0) break;
    uStack4 = param_2;
    (*(code *)param_1)();
  }
  return;
}

/*
Unable to decompile 'pass1_1000_55b1'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/

void pass1_1000_5586(u16 param_1,u16 param_2,i16 param_3,i16 param_4,i16 param_5,u16 param_6)

{
  i16 iVar1;

  iVar1 = param_3;
  while (iVar1 += -0x1, -0x1 < iVar1) {
    (*(code *)param_1)();
  }
  return;
}



void ret_op_1000_55ac(void)

{
  return;
}
