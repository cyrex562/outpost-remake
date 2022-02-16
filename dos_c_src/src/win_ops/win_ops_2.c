
void __stdcall16far send_msg_1038_ed8a(ushort param_1,ulong param_2,ulong param_3,HWND16 param_4)

{
  uint uVar1;
  undefined2 uVar2;
  uint uVar3;
  ushort uVar4;
  uchar *in_DX;
  uint uVar5;
  uchar *puVar6;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar7;
  ulong uVar8;
  
  if (param_3._2_2_ != 0x1c8) {
    if (param_3._2_2_ == 0x1c9) {
      puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,in_DX,unaff_DI);
      uVar2 = (undefined2)((ulong)puVar7 >> 0x10);
      uVar1 = *(uint *)((int)puVar7 + 0x20);
      uVar5 = *(uint *)((int)puVar7 + 0x22);
      uVar3 = uVar5 | uVar1;
      if (uVar3 == 0x0) {
        return;
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,uVar5);
      puVar6 = (uchar *)(uVar5 | uVar3);
      if (puVar6 == (uchar *)0x0) {
        return;
      }
      uVar4 = pass1_1030_5b00(CONCAT22(uVar5,uVar3));
      puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar4,unaff_SS,puVar6,unaff_DI);
      if (puVar7 == (ushort *)0x0) {
        return;
      }
      param_4 = 0x1018;
      uVar8 = pass1_1018_0ad4((ulong)puVar7);
      if (uVar8 == 0x0) {
        return;
      }
      param_3._2_2_ = 0x72;
    }
    else {
      if (param_3._2_2_ != 0x1ca) {
        post_win_msg_1040_7b3c
                  ((ulong *)CONCAT22((int)param_2,param_1),(ushort)(param_2 >> 0x10),(ushort)param_3,param_3._2_2_,
                   (int)&PTR_LOOP_1050_1040);
        return;
      }
    }
  }
  SendMessage16(param_4,0x0,0x0,CONCAT22(0x111,param_3._2_2_));
  return;
}




void __stdcall16far post_win_msg_1040_0d5e(ushort param_1,ushort param_2,int param_3,HWND16 param_4)

{
  if (param_3 != 0x0) {
    PostMessage16(param_4,0x0,0x0,0x1110001);
  }
  return;
}

