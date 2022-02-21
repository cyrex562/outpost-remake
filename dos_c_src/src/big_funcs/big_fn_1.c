
void __stdcall16far big_switch_1008_15d4(u16 param_1,u16 param_2,u16 param_3,u32 param_4,i32 param_5)

{
  u16 var3;
  u8 *var2;
  u16 var4;
  u8 *var5;
  u16 var6;
  astruct_20 *paVar2;
  astruct_20 *paStack32;
  u8 local_e [0x8];
  u32 uStack6;
  ulong uVar2;
  i32 *var_1;
  int *piVar1;
  
  uStack6 = 0x0;
  var3 = (u16)param_4;
  var3 = var3 + 0xd2;
  pass1_1008_57a4((ulong *)CONCAT22(param_3,local_e),param_4 & 0xffff0000 | (ulong)var3);
  do {
    var2 = local_e;
    pass1_1008_5b12(var2,param_3);
    var5 = (u8 *)(var4 | (uint)var2);
    if (var5 == (u8 *)0x0) goto LAB_1008_162a;
    uVar2 = *(ulong *)(var2 + 0x4);
    var5 = *(u8 **)(var2 + 0x6);
    param_1 = (u16)uVar2;
  } while (*(int *)(param_1 + 0xde) != (int)param_5);
  uStack6 = uVar2 & 0xffff | ZEXT24(var5) << 0x10;
LAB_1008_162a:
  if (uStack6 != 0x0) {
    return;
  }
  var6 = (u16)(param_4 >> 0x10);
  switch((int)param_5 + -0x1) {
  case 0x0:
    mem_op_1000_179c(0xec,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) {
LAB_1008_169a:
      uStack6 = 0x0;
      goto LAB_1008_2b3a;
    }
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1020_08b6(param_3,paStack32,*(UINT16 *)(var3 + 0xcc),param_4);
    break;
  default:
    debug_print_1008_6048((ulong)s_OpWnd__getKid__Unknown_target_mo_1050_01a3,0x1008,param_3);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
    goto LAB_1008_2b3a;
  case 0x2:
    mem_op_1000_179c(0xfa,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1018_e91e(paStack32,*(ushort *)(var3 + 0xcc),var3,param_3);
    break;
  case 0x3:
    mem_op_1000_179c(0xf6,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1018_e230(param_3,paStack32,*(ushort *)(var3 + 0xcc),var3);
    break;
  case 0x4:
    mem_op_1000_179c(0xf6,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1020_7554(param_3,paStack32,*(ushort *)(var3 + 0xcc),var3);
    break;
  case 0x5:
    mem_op_1000_179c(0xf8,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1018_5840(paStack32,*(ushort *)(var3 + 0xcc),var3,param_3);
    break;
  case 0x6:
    mem_op_1000_179c(0xf6,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1020_2524(paStack32,*(ushort *)(var3 + 0xcc),var3,param_3);
    break;
  case 0x7:
    mem_op_1000_179c(0x118,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    unk_draw_op_1020_41c8(paStack32,*(ushort *)(var3 + 0xcc),var3,0x1020);
    break;
  case 0x8:
    mem_op_1000_179c(0xf6,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    pass1_1018_e5dc(param_3,paStack32,*(ushort *)(var3 + 0xcc),var3);
    break;
  case 0x9:
    mem_op_1000_179c(0xf6,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    struct_1018_66cc(paStack32,*(ushort *)(var3 + 0xcc),var3,param_3);
    break;
  case 0xa:
    win_1008_5c5c((WNDCLASS16 *)param_3,param_1,(ushort)var5,_PTR_LOOP_1050_02a0,0x1d3);
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6d02(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0xb:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6d38(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0xc:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6d6e(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0xd:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6da4(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0xe:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6dda(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0xf:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6e10(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x10:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6e46(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x11:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6e7c(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x12:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6eb2(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x13:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6ee8(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x14:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6f1e(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x15:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6f54(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x16:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6f8a(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x17:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6fc0(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x18:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_6ff6(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x19:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_702c(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x1a:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7062(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x1b:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7098(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x1c:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_70ce(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x1d:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7104(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x1e:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_713a(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x20:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7170(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x21:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_745e(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x22:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_71a6(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x23:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_71dc(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x24:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7212(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x25:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_c958(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x26:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_c9a6(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x27:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_c9f4(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x28:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_ca48(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x29:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_ca96(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x2a:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_caea(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x2b:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cb38(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x2c:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cb86(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x2d:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cbda(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x2e:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cc28(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x2f:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cc76(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x30:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_ccc4(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x31:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cd12(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x32:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cd60(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x33:
    mem_op_1000_179c(0x114,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_cf74(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x34:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_73c2(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x35:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7494(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x36:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_74ca(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x37:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7500(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x38:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_73f8(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x39:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7536(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x3a:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_756c(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x3b:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_75a2(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x3c:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = pass1_1018_75d8(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x3d:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_760e(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x3e:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7644(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x3f:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_767a(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x40:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_76b0(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x41:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_76e6(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x42:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_771c(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x43:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7752(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x44:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7788(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x45:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_77be(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x46:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_77f4(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x47:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_782a(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x48:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7860(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x49:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7896(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x4a:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_78cc(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x4b:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7902(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x4c:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7938(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x4d:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_796e(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x4e:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_79a4(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x4f:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_79da(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x50:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7a10(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x51:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7a46(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x52:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7a7c(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x54:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7ab2(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x55:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7ae8(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x56:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7b1e(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x57:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7b54(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x58:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7b8a(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x59:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7bc0(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x5a:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7bf6(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x5b:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7c2c(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x5c:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7c62(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x5d:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7c98(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x5e:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7cce(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x5f:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7d04(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x60:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7d3a(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x61:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7d70(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x62:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7248(paStack32,*(UINT16 *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x63:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_727e(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x64:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_72b4(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x65:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_72ea(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x66:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7320(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x67:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    piVar1 = (int *)(var3 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    paVar2 = struct_1018_7356(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
    break;
  case 0x68:
    mem_op_1000_179c(0xf2,var5,0x1000);
    paStack32 = (astruct_20 *)CONCAT22(var5,param_1);
    var5 = (u8 *)((uint)var5 | param_1);
    if (var5 == (uchar *)0x0) goto LAB_1008_169a;
    var_1 = (i32 *)(var3 + 0xcc);
    *(int *)var_1 = *(int *)var_1 + 0x1;
    paVar2 = struct_1018_738c(paStack32,*(ushort *)(var3 + 0xcc),param_4,param_3);
    var5 = (u8 *)((ulong)paVar2 >> 0x10);
    param_1 = (u16)paVar2;
  }
  uStack6 = CONCAT22(var5,param_1);
LAB_1008_2b3a:
  pass1_1008_6978(param_4,0x0,uStack6,param_1,var5);
  return;
}
