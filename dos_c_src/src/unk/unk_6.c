
ulong *__stdcall16far pass1_1030_3af6(ulong *param_1, ushort param_2, ushort param_3, ulong *param_4, ushort param_5)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *param_1                     = *param_4;
    *(undefined2 *)(iVar1 + 0x4) = *(undefined2 *)(param_4 + 0x1);
    *(ushort *)(iVar1 + 0x6)     = param_3;
    *(ushort *)(iVar1 + 0x8)     = param_2;
    return param_1;
}


ushort __cdecl16far pass1_1030_3b28(undefined2 param_1)

{
    ushort   *puVar1;
    ulong    *puVar2;
    undefined local_8[0x6];

    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffc4, 0x0);
    pass1_1030_3af6((ulong *)&USHORT_1050_65e6, 0x115, 0x15b, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_65f0, 0x116, 0x15c, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffdd, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_65fa, 0x117, 0x15d, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x4b);
    pass1_1030_3af6((ulong *)&USHORT_1050_6604, 0x118, 0x15e, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xf, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_660e, 0x119, 0x15f, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x28, 0x7d);
    pass1_1030_3af6((ulong *)&USHORT_1050_6618, 0x11a, 0x160, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffec, 0x96);
    pass1_1030_3af6((ulong *)&USHORT_1050_6622, 0x11b, 0x161, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x14, 0xaf);
    pass1_1030_3af6((ulong *)&USHORT_1050_662c, 0x11c, 0x162, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0xc8);
    pass1_1030_3af6((ulong *)&USHORT_1050_6636, 0x11d, 0x163, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xfffb, 0xe1);
    pass1_1030_3af6((ulong *)&USHORT_1050_6640, 0x11e, 0x164, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x32, 0xfa);
    pass1_1030_3af6((ulong *)&USHORT_1050_664a, 0x11f, 0x165, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0xe1);
    pass1_1030_3af6((ulong *)&USHORT_1050_6654, 0x120, 0x166, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffe7, 0xfa);
    pass1_1030_3af6((ulong *)&USHORT_1050_665e, 0x121, 0x167, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x113);
    pass1_1030_3af6((ulong *)&USHORT_1050_6668, 0x122, 0x168, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x28, 0x12c);
    pass1_1030_3af6((ulong *)&USHORT_1050_6672, 0x123, 0x169, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xf, 0x145);
    pass1_1030_3af6((ulong *)&USHORT_1050_667c, 0x124, 0x16a, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffec, 0x15e);
    pass1_1030_3af6((ulong *)&USHORT_1050_6686, 0x125, 0x16b, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x0);
    pass1_1030_3af6((ulong *)&USHORT_1050_6690, 0x126, 0x16c, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x2d, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_669a, 0x127, 0x16d, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xa, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_66a4, 0x128, 0x16e, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffe2, 0x4b);
    pass1_1030_3af6((ulong *)&USHORT_1050_66ae, 0x129, 0x16f, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x5, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_66b8, 0x12a, 0x170, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x32, 0x7d);
    pass1_1030_3af6((ulong *)&USHORT_1050_66c2, 0x12b, 0x171, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffc9, 0x96);
    pass1_1030_3af6((ulong *)&USHORT_1050_66cc, 0x12c, 0x172, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xfffb, 0xaf);
    pass1_1030_3af6((ulong *)&USHORT_1050_66d6, 0x12d, 0x173, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffe7, 0xc8);
    pass1_1030_3af6((ulong *)&USHORT_1050_66e0, 0x12e, 0x174, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x32, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_66ea, 0x12f, 0x175, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x3c, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_66f4, 0x130, 0x176, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffc4, 0xe1);
    pass1_1030_3af6((ulong *)&USHORT_1050_66fe, 0x131, 0x177, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_6708, 0x132, 0x178, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x5, 0xaf);
    pass1_1030_3af6((ulong *)&USHORT_1050_6712, 0x133, 0x179, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_671c, 0x134, 0x17a, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x23, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_6726, 0x135, 0x17b, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xfffb, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_6730, 0x136, 0x17c, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xf, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_673a, 0x137, 0x17d, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x2d, 0x4b);
    pass1_1030_3af6((ulong *)&USHORT_1050_6744, 0x138, 0x17e, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0x4b);
    pass1_1030_3af6((ulong *)&USHORT_1050_674e, 0x139, 0x17f, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x2d, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_6758, 0x13a, 0x180, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffe7, 0x7d);
    pass1_1030_3af6((ulong *)&USHORT_1050_6762, 0x13b, 0x181, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x5, 0xaf);
    pass1_1030_3af6((ulong *)&USHORT_1050_676c, 0x13c, 0x182, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0xc8);
    pass1_1030_3af6((ulong *)&USHORT_1050_6776, 0x13d, 0x183, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffce, 0xc8);
    pass1_1030_3af6((ulong *)&USHORT_1050_6780, 0x13e, 0x184, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xf, 0xfa);
    pass1_1030_3af6((ulong *)&USHORT_1050_678a, 0x13f, 0x185, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0x113);
    pass1_1030_3af6((ulong *)&USHORT_1050_6794, 0x140, 0x186, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffe2, 0x12c);
    pass1_1030_3af6((ulong *)&USHORT_1050_679e, 0x141, 0x187, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x64, 0x12c);
    pass1_1030_3af6((ulong *)&USHORT_1050_67a8, 0x142, 0x188, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x32, 0x145);
    pass1_1030_3af6((ulong *)&USHORT_1050_67b2, 0x143, 0x189, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x64, 0x145);
    pass1_1030_3af6((ulong *)&USHORT_1050_67bc, 0x144, 0x18a, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0x15e);
    pass1_1030_3af6((ulong *)&USHORT_1050_67c6, 0x145, 0x18b, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffd3, 0x15e);
    pass1_1030_3af6((ulong *)&USHORT_1050_67d0, 0x146, 0x18c, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x32, 0xfa);
    pass1_1030_3af6((ulong *)&USHORT_1050_67da, 0x147, 0x18d, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xf, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_67e4, 0x148, 0x18e, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_67ee, 0x149, 0x18f, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0xaf);
    pass1_1030_3af6((ulong *)&USHORT_1050_67f8, 0x14a, 0x190, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xfffb, 0xe1);
    pass1_1030_3af6((ulong *)&USHORT_1050_6802, 0x14b, 0x191, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xa, 0x15e);
    pass1_1030_3af6((ulong *)&USHORT_1050_680c, 0x14c, 0x192, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x19);
    pass1_1030_3af6((ulong *)&USHORT_1050_6816, 0x14d, 0x193, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0x32);
    pass1_1030_3af6((ulong *)&USHORT_1050_6820, 0x14e, 0x194, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xfffb, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_682a, 0x14f, 0x195, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xf, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_6834, 0x150, 0x196, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x1e, 0x7d);
    pass1_1030_3af6((ulong *)&USHORT_1050_683e, 0x151, 0x197, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffdd, 0xe1);
    pass1_1030_3af6((ulong *)&USHORT_1050_6848, 0x152, 0x198, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x113);
    pass1_1030_3af6((ulong *)&USHORT_1050_6852, 0x153, 0x199, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x2d, 0x12c);
    pass1_1030_3af6((ulong *)&USHORT_1050_685c, 0x154, 0x19a, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffe7, 0x145);
    pass1_1030_3af6((ulong *)&USHORT_1050_6866, 0x155, 0x19b, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xa, 0x15e);
    pass1_1030_3af6((ulong *)&USHORT_1050_6870, 0x156, 0x19c, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x4b);
    pass1_1030_3af6((ulong *)&USHORT_1050_687a, 0x157, 0x19d, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x5, 0x64);
    pass1_1030_3af6((ulong *)&USHORT_1050_6884, 0x158, 0x19e, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0xffec, 0x96);
    pass1_1030_3af6((ulong *)&USHORT_1050_688e, 0x159, 0x19f, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_8), 0x0, 0x0, 0x113);
    puVar2
      = pass1_1030_3af6((ulong *)&USHORT_1050_6898, 0x15a, 0x1a0, (ulong *)puVar1, (ushort)((ulong)puVar1 >> 0x10));
    return (ushort)puVar2;
}

void __stdcall16far pass1_1030_1eee(ulong param_1, ulong param_2)

{
    ulong      uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3         = (undefined2)(param_1 >> 0x10);
    iVar2         = (int)param_1;
    uVar1         = *(ulong *)(iVar2 + 0xc);
    param_2._2_2_ = *(undefined2 *)(iVar2 + 0xe);
    if(uVar1 < param_2)
    {
        uVar1 = param_2 & 0xffff;
    }
    *(undefined2 *)(iVar2 + 0xc) = (int)uVar1;
    *(undefined2 *)(iVar2 + 0xe) = param_2._2_2_;
    return;
}


void __stdcall16far pass1_1030_1f16(ulong *param_1, ulong param_2)

{
    long      *plVar1;
    code     **ppcVar2;
    undefined4 uVar3;
    int        iVar4;
    undefined2 uVar5;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if((*(long *)(iVar4 + 0x4) == 0x0) || (*(ulong *)(iVar4 + 0x10) <= *(ulong *)(iVar4 + 0x8)))
    {
        ppcVar2 = (code **)((int)*param_1 + 0x20);
        (**ppcVar2)();
    }
    uVar3                                                = *(undefined4 *)(iVar4 + 0x4);
    *(ulong *)(*(int *)(iVar4 + 0x8) * 0x4 + (int)uVar3) = param_2;
    plVar1                                               = (long *)(iVar4 + 0x8);
    *plVar1                                              = *plVar1 + 0x1;
    return;
}


int __stdcall16far pass1_1030_2242(ulong param_1, int param_2)

{
    int          iVar1;
    astruct_168 *iVar2;
    astruct_168 *paVar2;
    undefined2   uVar3;

    uVar3  = (undefined2)(param_1 >> 0x10);
    iVar2  = (astruct_168 *)param_1;
    paVar2 = (astruct_168 *)&iVar2->field_0x10;
    if(-0x1 < *(int *)(&paVar2->field_0x0 + param_2 * 0x2))
    {
        iVar1  = *(int *)(&iVar2->field_0x10 + param_2 * 0x2);
        paVar2 = iVar2 + 0x1;
        if(*(int *)(&paVar2->field_0x0 + param_2 * 0x2) <= iVar1)
        {
            return iVar1;
        }
    }
    return *(int *)(&paVar2->field_0x0 + param_2 * 0x2);
}


BOOL16 __stdcall16far pass1_1030_25b2(ulong param_1, int param_2)

{
    if(*(int *)((int)param_1 + 0x10 + param_2 * 0x2) == 0x0)
    {
        return 0x1;
    }
    return 0x0;
}


void __stdcall16far pass1_1030_25d8(ulong param_1, ushort param_2, int param_3)

{
    *(ushort *)((int)param_1 + param_3 * 0x2 + 0x10) = param_2;
    return;
}


void __stdcall16far pass1_1030_25f0(ulong param_1, int param_2, int param_3)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        param_2 = *(int *)((int)param_1 + 0x116 + param_3 * 0x2) + 0x1;
    }
    *(int *)((int)param_1 + param_3 * 0x2 + 0x116) = param_2;
    return;
}


bool __stdcall16far pass1_1030_2622(ulong param_1, int param_2)

{
    int iVar1;

    if((param_2 != 0x70) && (param_2 != 0x1))
    {
        iVar1 = pass1_1030_28dc(param_1, 0x0);
        if(-0x1 < iVar1)
        {
            *(int *)((int)param_1 + iVar1 * 0x2 + 0x19c) = param_2;
        }
        return -0x1 < iVar1;
    }
    return false;
}


bool __stdcall16far pass1_1030_266c(ushort param_1, ulong param_2)

{
    int iVar1;

    iVar1 = pass1_1030_28dc(CONCAT22((int)param_2, param_1), (int)(param_2 >> 0x10));
    return iVar1 != -0x1;
}


void __stdcall16far pass1_1030_2690(ulong param_1)

{
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x2ac)), (WNDCLASS16 *)0x0, 0x106);
    return;
}


void __stdcall16far pass1_1030_26ac(ulong param_1, uint param_2, uint param_3, ushort param_4)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;
    char       cVar5;
    undefined *puVar6;
    ushort     uVar7;
    int        iVar8;
    int        iVar9;
    uint       uVar10;
    int        iVar11;
    undefined2 uVar12;
    int        iStack38;
    undefined  local_14[0x12];

    iVar11 = (int)param_1;
    uVar12 = (undefined2)(param_1 >> 0x10);
    if(param_2 != 0x13)
    {
        if(0x13 < (int)param_2)
        {
            if(param_2 != 0x5f)
            {
                if((int)(param_2 - 0x5f) < 0x6)
                {
                    return;
                }
                if(param_2 != 0x66 && 0x0 < (int)(param_2 - 0x65))
                {
                    if((int)(param_2 - 0x66) < 0x5)
                    {
                        return;
                    }
                    if(0x4 < (int)(param_2 - 0x6b))
                    {
                        return;
                    }
                }
            }
            pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
            while(true)
            {
                uVar10 = param_3;
                puVar6 = local_14;
                pass1_1028_e4ec(CONCAT22(param_4, puVar6));
                param_3 = uVar10 | (uint)puVar6;
                if(param_3 == 0x0)
                    break;
                if(*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200))
                {
                    uVar7 = *(int *)(puVar6 + 0x18) + 0x19;
                    if(0x3e8 < (int)uVar7)
                    {
                        uVar7 = 0x3e8;
                    }
                    pass1_1038_4d0e(CONCAT22(uVar10, puVar6), uVar7);
                }
            }
            return;
        }
        if(param_2 == 0x12)
        {
            pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
            while(true)
            {
                uVar10 = param_3;
                puVar6 = local_14;
                pass1_1028_e4ec(CONCAT22(param_4, puVar6));
                param_3 = uVar10 | (uint)puVar6;
                if(param_3 == 0x0)
                    break;
                if(*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200))
                {
                    uVar2   = *(undefined4 *)(puVar6 + 0x1f6);
                    iVar9   = (int)uVar2;
                    uVar4   = (undefined2)((ulong)uVar2 >> 0x10);
                    piVar1  = (int *)(iVar9 + 0x182);
                    *piVar1 = *piVar1 + -0x19;
                    iVar8   = *(int *)(iVar9 + 0x182);
                    if(iVar8 < 0x1)
                    {
                        iVar8 = 0x1;
                    }
                    *(int *)(iVar9 + 0x182) = iVar8;
                }
            }
            return;
        }
        if(0x12 < param_2)
        {
            return;
        }
        cVar5 = (char)param_2;
        if(cVar5 != '\n')
        {
            if((char)(cVar5 + -0xa) < '\x06')
            {
                return;
            }
            if('\x01' < (char)(cVar5 + -0x10))
            {
                return;
            }
        }
    }
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
    while(true)
    {
        uVar10 = param_3;
        puVar6 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar6));
        param_3 = uVar10 | (uint)puVar6;
        if(param_3 == 0x0)
            break;
        if(*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200))
        {
            uVar2    = *(undefined4 *)(puVar6 + 0x1f6);
            iVar8    = (int)uVar2 + 0x180;
            uVar4    = (undefined2)((ulong)uVar2 >> 0x10);
            iStack38 = 0x1;
            do
            {
                iVar3   = iStack38 * 0x2;
                piVar1  = (int *)(iVar3 + iVar8);
                *piVar1 = *piVar1 + -0x1;
                iVar9   = *(int *)(iVar3 + iVar8);
                if(iVar9 < 0x1)
                {
                    iVar9 = 0x1;
                }
                *(int *)(iVar3 + iVar8) = iVar9;
                iStack38                = iStack38 + 0x1;
            } while(iStack38 < 0x6);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_2852(void)

{
    return;
}


int __stdcall16far pass1_1030_28dc(ulong param_1, int param_2)

{
    int iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x4 < iStack4)
        {
            return -0x1;
        }
        if(*(int *)((int)param_1 + 0x19c + iStack4 * 0x2) == param_2)
            break;
        iStack4 = iStack4 + 0x1;
    }
    return iStack4;
}


ushort __stdcall16far pass1_1030_2a98(ulong param_1)

{
    int       *piVar1;
    undefined2 uVar2;

    uVar2   = (undefined2)(param_1 >> 0x10);
    piVar1  = (int *)((int)param_1 + 0x14);
    *piVar1 = *piVar1 + 0x1;
    return *(ushort *)((int)param_1 + 0x14);
}


ushort __stdcall16far pass1_1030_2aaa(ulong param_1)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x10) == 0x0)
    {
        return 0x0;
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x10);
    return *(ushort *)((int)uVar1 + 0xc);
}


int __stdcall16far pass1_1030_2f1a(ulong param_1, ushort *param_2, ushort *param_3)

{
    int        iVar1;
    undefined4 uVar2;
    int        iVar3;

    uVar2 = *(undefined4 *)((int)param_1 + 0x10);
    iVar3 = (int)uVar2;
    iVar1 = *(int *)(iVar3 + 0xc);
    if(iVar1 - 0x1U < 0x9)
    {
        switch(iVar1)
        {
        default:
            *param_3 = 0x19;
            *param_2 = 0x2d;
            return iVar3;
        case 0x3:
        case 0x4:
        case 0x5:
            *param_3 = 0xa;
            *param_2 = 0xf;
            return iVar3;
        case 0x6:
            *param_3 = 0xa;
            *param_2 = 0x19;
            return iVar3;
        case 0x7:
            *param_3 = 0x19;
            *param_2 = 0x37;
            return iVar3;
        }
    }
    *param_3 = 0x0;
    *param_2 = 0x0;
    return 0x0;
}


ushort __stdcall16far pass1_1030_2fac(ulong param_1)

{
    long         lVar1;
    astruct_598 *iVar2;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar2 = (astruct_598 *)param_1;
    if(iVar2->field_0x10 == 0x0)
    {
        return 0x0;
    }
    lVar1 = iVar2->field_0x10;
    if(*(uint *)((int)lVar1 + 0xc) < 0x2)
    {
        return 0x4;
    }
    lVar1 = iVar2->field_0x10;
    if(*(uint *)((int)lVar1 + 0xc) < 0x5)
    {
        return 0x3;
    }
    lVar1 = iVar2->field_0x10;
    if(*(uint *)((int)lVar1 + 0xc) < 0x8)
    {
        return 0x2;
    }
    return 0x1;
}


void __stdcall16far pass1_1030_10b0(ushort       param_1,
                                    ushort       param_2,
                                    ushort       param_3,
                                    undefined4   param_4,
                                    ulong        param_5,
                                    uchar       *param_6,
                                    astruct_179 *param_7,
                                    ushort       param_8,
                                    ushort       param_9,
                                    ushort       param_10)

{
    ulong   uVar1;
    ulong   uVar2;
    ushort  uVar3;
    ushort  uVar4;
    uint    uVar5;
    ushort *puVar6;
    uint    uStack8;

    puVar6 = switch_1030_07ac(param_1,
                              param_2,
                              param_3,
                              (ushort)param_4,
                              (ushort)((ulong)param_4 >> 0x10),
                              param_5,
                              param_6,
                              param_7,
                              param_8,
                              param_9,
                              param_10);
    uVar3  = (ushort)((ulong)puVar6 >> 0x10);
    uVar1  = *(ulong *)((ushort)puVar6 + 0x4);
    uVar2  = uVar1;
    uVar4  = uVar3;
    pass1_1028_e1ec(CONCAT22(param_2, param_1), (ushort)param_5, (uint)(param_5 >> 0x10));
    uVar5 = uVar4 | (uint)uVar2;
    if(uVar5 != 0x0)
    {
        pass1_1030_7e5a(uVar2 & 0xffff | (ulong)uVar4 << 0x10, uVar1, uVar5);
    }
    uStack8 = (uint)(uVar1 >> 0x10);
    pass1_1030_1358(
      *(ulong *)(param_1 + 0x26), (ushort)puVar6, uVar3, uVar1 & 0xffff | (ulong)(uStack8 & 0xff) << 0x10, param_10);
    return;
}


void __stdcall16far pass1_1030_12ca(ulong param_1)

{
    ulong       *puVar1;
    undefined4   uVar2;
    astruct_176 *iVar3;
    undefined2   uVar3;
    ulong        uStack6;

    uStack6 = 0x1;
    while(true)
    {
        uVar3  = (undefined2)(param_1 >> 0x10);
        iVar3  = (astruct_176 *)param_1;
        puVar1 = &iVar3->field_0xa;
        if(*puVar1 < uStack6 || *puVar1 == uStack6)
        {
            iVar3->field_0x4 = 0x0;
            return;
        }
        uVar2 = iVar3->field_0x6;
        if(*(long *)((int)uVar2 + (int)uStack6 * 0x4) == 0x0)
            break;
        uStack6 = uStack6 + 0x1;
    }
    return;
}


void __stdcall16far bad_1030_1312(void)

{
    return;
}


ushort __stdcall16far pass1_1030_13f6(ulong param_1, ulong param_2, uint param_3, uint param_4, ushort param_5)

{
    code      **ppcVar1;
    undefined2  uVar2;
    undefined4 *puStack8;
    ushort      uStack4;

    uStack4 = 0x0;
    bad_1030_1312();
    puStack8 = (undefined4 *)CONCAT22(param_4, param_3);
    if((param_4 | param_3) != 0x0)
    {
        uStack4 = 0x1;
        uVar2   = (undefined2)(param_1 >> 0x10);
        if((*(int *)((int)param_1 + 0x1a) != 0x0) && ((param_4 | param_3) != 0x0))
        {
            ppcVar1 = (code **)*puStack8;
            (**ppcVar1)();
        }
        pass1_1030_1358(param_1, 0x0, 0x0, param_2, param_5);
        *(undefined2 *)((int)param_1 + 0x4) = 0x1;
    }
    return uStack4;
}


void __stdcall16far pass1_1030_154c(void)

{
    return;
}


void __stdcall16far pass1_1030_177a(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x8) = param_2;
    return;
}


void __stdcall16far pass1_1030_18b2(ushort *param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                        = (int)param_1;
    *param_1                     = 0x1a16;
    *(undefined2 *)(iVar4 + 0x2) = 0x1030;
    puVar1                       = (undefined4 *)*(uint *)(iVar4 + 0xc);
    uVar2                        = *(uint *)(iVar4 + 0xe);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1030_16b2(param_1);
    return;
}


void __stdcall16far pass1_1030_18f0(ulong param_1, int param_2, int param_3, ushort param_4)

{
    code     **ppcVar1;
    ulong      uVar2;
    undefined2 extraout_DX;
    int        extraout_DX_00;
    int        iVar3;
    undefined2 uVar4;
    ulong      uStack10;
    ulong      uStack6;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0xc) != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xc) + 0x10);
        (**ppcVar1)();
        uStack6 = CONCAT22(extraout_DX, param_4);
        for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
        {
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xc) + 0x4);
            uVar2   = uStack6;
            (**ppcVar1)();
            if(((int)uVar2 == param_2) && (extraout_DX_00 == param_3))
            {
                ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xc) + 0x8);
                (**ppcVar1)();
            }
        }
    }
    return;
}


ushort __stdcall16far pass1_1030_1972(void)

{
    return 0x1;
}


ushort *__stdcall16far pass1_1030_1a32(ushort *param_1, uint param_2, uchar *param_3)

{
    pass1_1030_183c(param_1, 0x1, 0x16, 0xff000000, 0x0, param_2, param_3);
    PTR_LOOP_1050_5168                         = (undefined *)((ulong)param_1 >> 0x10);
    PTR_LOOP_1050_5166                         = (undefined *)param_1;
    *(undefined4 *)(PTR_LOOP_1050_5166 + 0x10) = 0x0;
    *param_1                                   = 0x1cbc;
    *(undefined2 *)(PTR_LOOP_1050_5166 + 0x2)  = 0x1030;
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_1a74(ushort *param_1)

{
    *param_1                            = 0x1cbc;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    _PTR_LOOP_1050_5166                 = 0x0;
    pass1_1030_18b2(param_1);
    return;
}


void __stdcall16far pass1_1030_1d58(ulong param_1)

{
    code     **ppcVar1;
    undefined4 uVar2;

    ppcVar1 = (code **)((int)*(undefined4 *)param_1 + 0x4);
    uVar2   = (**ppcVar1)();
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)((ulong)uVar2 >> 0x10));
    return;
}


ulong __stdcall16far pass1_1030_1daa(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0xa), *(undefined2 *)((int)param_1 + 0x8));
}


void __stdcall16far pass1_1030_1dbc(void)

{
    return;
}


void __stdcall16far pass1_1030_1dfc(ulong *param_1, ushort param_2, ushort param_3, ulong param_4)

{
    ulong     *puVar1;
    uint      *puVar2;
    code     **ppcVar3;
    undefined4 uVar4;
    int        iVar5;
    undefined2 uVar6;
    bool       bVar7;

    uVar6  = (undefined2)((ulong)param_1 >> 0x10);
    iVar5  = (int)param_1;
    puVar1 = (ulong *)(iVar5 + 0x8);
    if((*puVar1 < param_4 || *puVar1 == param_4) || (*(long *)(iVar5 + 0x4) == 0x0))
    {
        puVar2 = (uint *)(iVar5 + 0x12);
        bVar7  = *puVar2 < param_4._2_2_;
        if((bVar7 || *puVar2 == param_4._2_2_)
           && ((bVar7 || (puVar2 = (uint *)(iVar5 + 0x10), *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4))))
        {
            ppcVar3 = (code **)((int)*param_1 + 0x20);
            (**ppcVar3)();
        }
        puVar1 = (ulong *)(iVar5 + 0x10);
        if((*puVar1 < param_4 || *puVar1 == param_4) || (*(long *)(iVar5 + 0x4) == 0x0))
        {
            return;
        }
        puVar2 = (uint *)(iVar5 + 0xa);
        bVar7  = *puVar2 < param_4._2_2_;
        if((bVar7 || *puVar2 == param_4._2_2_)
           && ((bVar7 || (puVar2 = (uint *)(iVar5 + 0x8), *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4))))
        {
            *(undefined2 *)(iVar5 + 0x8) = (int)(param_4 + 0x1);
            *(undefined2 *)(iVar5 + 0xa) = (int)(param_4 + 0x1 >> 0x10);
        }
    }
    uVar4                                          = *(undefined4 *)(iVar5 + 0x4);
    uVar6                                          = (undefined2)((ulong)uVar4 >> 0x10);
    iVar5                                          = (int)uVar4;
    *(ushort *)(iVar5 + (uint)param_4 * 0x4)       = param_2;
    *(ushort *)(iVar5 + (uint)param_4 * 0x4 + 0x2) = param_3;
    return;
}


void __stdcall16far pass1_1030_1e96(ulong *param_1)

{
    ulong     *puVar1;
    code     **ppcVar2;
    undefined4 uVar3;
    undefined2 uVar4;
    ulong      uStack6;

    uStack6 = 0x0;
    while(true)
    {
        uVar4  = (undefined2)((ulong)param_1 >> 0x10);
        puVar1 = (ulong *)((int)param_1 + 0x8);
        if((*puVar1 < uStack6 || *puVar1 == uStack6)
           || (uVar3 = *(undefined4 *)((int)param_1 + 0x4), *(long *)((int)uVar3 + (int)uStack6 * 0x4) == 0x0))
            break;
        uStack6 = uStack6 + 0x1;
    }
    ppcVar2 = (code **)((int)*param_1 + 0x8);
    (**ppcVar2)();
    return;
}


void __stdcall16far pass1_1028_ee54(ulong param_1, ushort param_2, ushort *param_3, ulong param_4)

{
    undefined2  in_DX;
    uint        uVar1;
    ushort      unaff_SS;
    astruct_99 *paVar2;
    undefined   local_16[0x4];
    undefined4  uStack18;
    uint        uStack14;
    ushort      uStack12;
    undefined4  uStack10;
    ushort     *puStack6;

    puStack6 = param_3;
    pass1_1030_64ce(
      unaff_SS, (int)param_3, in_DX, _PTR_LOOP_1050_5740, param_3, param_4, (ulong *)CONCAT22(unaff_SS, local_16));
    uStack10 = *(undefined4 *)param_3;
    paVar2   = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5744);
    uVar1    = (uint)((ulong)paVar2 >> 0x10);
    uStack14 = (uint)paVar2;
    uStack12 = uVar1 | uStack14;
    if(uStack12 == 0x0)
    {
        uStack14 = 0x0;
        uStack12 = 0x0;
    }
    else
    {
        pass1_1030_684c((ushort *)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10),
                        (ulong *)puStack6,
                        (ushort)((ulong)puStack6 >> 0x10),
                        (ushort)uStack10,
                        (ushort)((ulong)uStack10 >> 0x10),
                        param_4,
                        uStack12);
    }
    uStack18 = *(ulong *)(uStack14 + 0x4);
    pass1_1030_61fe(_PTR_LOOP_1050_5740, uStack18, (ulong)puStack6, param_4, (ushort)uStack18, uStack12, unaff_SS);
    pass1_1030_1358(*(ulong *)((int)param_1 + 0x1e),
                    uStack14,
                    uStack12,
                    uStack18 & 0xffff | (ulong)(uStack18._2_2_ & 0xff) << 0x10,
                    unaff_SS);
    return;
}


void __stdcall16far pass1_1028_e332(ulong param_1, ushort param_2, uint param_3, ushort param_4)

{
    if((param_3._1_1_ != 0x0) && (param_3._1_1_ < 0xa))
    {
        pass1_1030_13f6(*(ulong *)((int)param_1 + 0xa + (uint)param_3._1_1_ * 0x4),
                        CONCAT22(param_3, param_2) & 0xffffff,
                        param_2,
                        param_3 & 0xff,
                        param_4);
    }
    return;
}


void __stdcall16far pass1_1028_e372(ulong param_1, ushort param_2, uint param_3, ushort param_4)

{
    ulong      uVar1;
    ulong      uVar2;
    undefined4 uVar3;
    code     **ppcVar4;
    ulong      uVar5;
    ulong      uVar6;
    uint       uVar7;
    uint       uVar8;
    ulong      uStack32;
    ulong      uStack16;
    uint       uStack10;

    if(param_3 >> 0x8 != 0xff)
    {
        uVar1    = *(ulong *)((int)param_1 + 0xa + (param_3 >> 0x8) * 0x4);
        uVar2    = *(ulong *)((int)uVar1 + 0xa);
        uVar7    = param_3 & 0xff;
        uStack16 = CONCAT22(param_3, param_2) & 0xffffff;
        pass1_1028_e1ec(param_1, param_2, param_3);
        uVar5 = *(ulong *)(param_2 + 0x8);
        pass1_1028_e1ec(param_1, (ushort)uVar5, (uint)(uVar5 >> 0x10));
        for(uStack32 = 0x1; uStack10 = (uint)(uVar2 >> 0x10), uStack32 < uVar2; uStack32 = uStack32 + 0x1)
        {
            if(uStack32 != uStack16)
            {
                uVar6 = uStack16;
                bad_1030_1312();
                uVar8 = uStack10 | (uint)uVar6;
                if(uVar8 != 0x0)
                {
                    uVar3 = *(undefined4 *)((uint)uVar6 + 0x4);
                    pass1_1030_13f6(uVar1, uStack32, (int)uVar3, uVar8, param_4);
                    ppcVar4 = (code **)((int)*(undefined4 *)(uVar5 & 0xffff | (ulong)uVar7 << 0x10) + 0x18);
                    (**ppcVar4)(0x1030, (int)(uVar5 & 0xffff), uVar7, uVar3);
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1028_e44a(ulong param_1, long param_2, ushort param_3)

{
    ulong      uVar1;
    ulong      uVar2;
    ulong      uVar3;
    ushort     uVar4;
    uint       uVar5;
    ulong      uVar6;
    uint       uVar7;
    undefined2 uVar8;
    ulong      uStack18;
    uint       uStack12;

    pass1_1028_e372(param_1, (ushort)param_2, (uint)((ulong)param_2 >> 0x10), param_3);
    uVar8 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(ulong *)((int)param_1 + 0x26);
    uVar2 = *(ulong *)((int)param_1 + 0x1e);
    uVar3 = *(ulong *)((int)uVar2 + 0xa);
    for(uStack18 = 0x1; uStack12 = (uint)(uVar3 >> 0x10), uStack18 < uVar3; uStack18 = uStack18 + 0x1)
    {
        uVar6 = uVar3;
        bad_1030_1312();
        uVar5 = (uint)uVar6;
        if(((uStack12 | uVar5) != 0x0) && (*(long *)(uVar5 + 0x8) != param_2))
        {
            uVar8 = *(undefined2 *)(uVar5 + 0x16);
            uVar5 = *(uint *)(uVar5 + 0x18);
            uVar7 = uVar5 & 0xff;
            uVar4 = pass1_1030_13f6(uVar1, CONCAT22(uVar5, uVar8) & 0xffffff, uVar8, uVar7, param_3);
            pass1_1030_13f6(uVar2, uStack18, uVar4, uVar7, param_3);
        }
    }
    return;
}


void __stdcall16far pass1_1028_e4ec(ulong param_1)

{
    ulong     *puVar1;
    long      *plVar2;
    ulong      uVar3;
    uint       uVar4;
    undefined4 uVar5;
    uint       in_DX;
    int        iVar6;
    undefined2 uVar7;

    uVar5 = 0x0;
    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(int *)(iVar6 + 0x10) == 0x0)
    {
        do
        {
            if(*(long *)(iVar6 + 0x8) == 0x0)
            {
                return;
            }
            plVar2  = (long *)(iVar6 + 0x8);
            *plVar2 = *plVar2 + -0x1;
            bad_1030_1312();
            in_DX = in_DX | (uint)uVar5;
        } while(in_DX == 0x0);
    }
    else
    {
        do
        {
            uVar3  = *(ulong *)(iVar6 + 0xc);
            puVar1 = (ulong *)(iVar6 + 0x8);
            if(uVar3 <= *puVar1 && *puVar1 != uVar3)
            {
                return;
            }
            uVar4   = (uint) * (undefined4 *)(iVar6 + 0x8);
            plVar2  = (long *)(iVar6 + 0x8);
            *plVar2 = *plVar2 + 0x1;
            bad_1030_1312();
            in_DX = in_DX | uVar4;
        } while(in_DX == 0x0);
    }
    return;
}


BOOL16 __stdcall16far pass1_1028_d52c(ulong *param_1, ulong param_2, ulong *param_3)

{
    code **ppcVar1;
    int    iVar2;
    BOOL16 BVar3;

    ppcVar1 = (code **)((int)*param_3 + 0x8);
    iVar2   = (**ppcVar1)();
    if(iVar2 != 0x0)
    {
        BVar3 = pass1_1028_d776(*param_1, param_2, param_3);
        if(BVar3 != 0x0)
        {
            return 0x1;
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1028_d658(ulong param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_446 *iVar4;
    undefined2   uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    iVar4  = (astruct_446 *)param_1;
    puVar1 = iVar4->field_0x4;
    uVar2  = iVar4->field_0x6;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4->field_0x8;
    uVar2  = iVar4->field_0xa;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    return;
}


ushort __stdcall16far pass1_1028_d69e(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x4);
    return *(ushort *)((int)uVar1 + 0x8);
}


void __stdcall16far fn_ptr_1028_d728(ulong param_1)

{
    code **ppcVar1;

    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}


ushort __stdcall16far fn_ptr_1028_d742(ulong param_1, ulong *param_2)

{
    code **ppcVar1;
    ulong  uVar2;

    ppcVar1 = (code **)((int)*param_2 + 0xc);
    uVar2   = (**ppcVar1)();
    pass1_1020_c872(*(ulong *)((int)param_1 + 0x4), *(undefined4 *)((int)uVar2 + 0x4), uVar2);
    return 0x1;
}


BOOL16 __stdcall16far pass1_1028_d776(ulong param_1, ulong param_2, ulong *param_3)

{
    code **ppcVar1;
    ulong  uVar2;

    ppcVar1 = (code **)((int)*param_3 + 0xc);
    uVar2   = (**ppcVar1)();
    pass1_1020_c872(*(ulong *)((int)param_1 + 0x8), param_2, uVar2);
    return 0x1;
}


void __stdcall16far pass1_1028_e0a0(ulong param_1, ulong param_2, uchar *param_3, ushort param_4, uchar param_5)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x52);
    pass1_1030_4782(param_4,
                    param_5,
                    param_3,
                    (ushort)uVar1,
                    (ushort)((ulong)uVar1 >> 0x10),
                    0x1,
                    (int)param_2,
                    (int)(param_2 >> 0x10));
    return;
}


void __stdcall16far
pass1_1028_e198(ulong param_1, ushort *param_2, ushort *param_3, ulong param_4, ushort param_5, ushort param_6)

{
    pass1_1028_e1ec(param_1, (ushort)param_4, (uint)(param_4 >> 0x10));
    pass1_1030_5b1c(CONCAT22(param_6, param_5), param_2, param_3);
    return;
}


void __stdcall16far bad_1028_e1bc(ulong param_1, ulong param_2)

{
    return;
}


void __stdcall16far pass1_1028_e1ec(ulong param_1, ushort param_2, uint param_3)

{
    if(param_3._1_1_ == '\0')
    {
        return;
    }
    if(param_3._1_1_ == -0x1)
    {
        return;
    }
    bad_1030_1312();
    return;
}


void __stdcall16far pass1_1028_e2ac(ulong param_1, uint param_2)

{
    ulong uStack6;

    uStack6 = *(ulong *)((int)param_1 + (param_2 >> 0x8) * 0x4 + 0x2e);
    (*(code *)uStack6)();
    return;
}


ulong __stdcall16far pass1_1028_e2e0(ulong param_1, ushort param_2, byte param_3)

{
    int        iVar1;
    undefined2 uVar2;
    ulong      uVar3;
    undefined2 auStack10[0x3];
    uint       uStack4;

    uStack4 = (uint)param_3;
    if(uStack4 == 0xff)
    {
        uVar3 = pass1_1028_ebee(param_1, 0x0, param_2);
        return uVar3;
    }
    uVar2        = (undefined2)(param_1 >> 0x10);
    iVar1        = (int)param_1 + 0x2e;
    auStack10[0] = *(undefined2 *)(iVar1 + uStack4 * 0x4 + 0x2);
    uVar3        = (**(code **)(iVar1 + uStack4 * 0x4))();
    return uVar3;
}


void __stdcall16far pass1_1028_c23e(ushort  param_1,
                                    ushort  param_2,
                                    ushort *param_3,
                                    ulong   param_4,
                                    long    param_5,
                                    uint    param_6,
                                    uint    param_7,
                                    ushort  param_8)

{
    ulong       uVar1;
    code      **ppcVar2;
    undefined4 *puVar3;
    uint        uVar4;
    uint        uVar5;
    ulong       uVar6;
    uint        uVar7;
    uint        uVar8;
    ushort      extraout_DX;
    undefined4 *puStack22;
    ulong       uStack10;
    undefined4  uStack6;

    pass1_1030_627e(param_8, param_6, param_7, _PTR_LOOP_1050_5740, param_3, param_5);
    uStack6 = CONCAT22(param_7, param_6);
    uVar7   = param_7 | param_6;
    if(uVar7 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, param_7);
        uStack10 = CONCAT22(uVar7, param_6);
        uVar1    = *(ulong *)(param_6 + 0x2a);
        if(uVar1 != param_4)
        {
            uVar6 = param_4;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)(uVar1 >> 0x10));
            uVar4  = (uint)uVar6;
            puVar3 = (undefined4 *)(uVar6 & 0xffff | (ulong)uVar7 << 0x10);
            uVar8  = uVar7;
            uVar5  = uVar4;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
            puStack22 = (undefined4 *)CONCAT22(uVar8, uVar5);
            if(((puVar3 == (undefined4 *)0x0) || ((uVar8 | uVar5) == 0x0))
               || (*(long *)(uVar5 + 0x200) != *(long *)(uVar4 + 0x200)))
            {
                return;
            }
            ppcVar2 = (code **)((int)*puVar3 + 0x18);
            (**ppcVar2)(0x1030, uVar4, uVar7, uStack6);
            ppcVar2 = (code **)((int)*puStack22 + 0x8);
            (**ppcVar2)();
            pass1_1030_73ee(uStack10, param_4, extraout_DX);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1028_c314(ushort  param_1,
                                      int     param_2,
                                      ushort  param_3,
                                      ushort  param_4,
                                      ushort  param_5,
                                      ushort *param_6,
                                      ushort  param_7,
                                      ushort  param_8,
                                      ulong   param_9)

{
    undefined4 *puVar1;
    int         local_14;
    int         local_12;
    int         local_10;
    int         local_e;
    undefined4  local_c;
    undefined2  uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_9, (uint)(param_9 >> 0x10));
    iStack6 = param_2;
    uStack4 = param_3;
    puVar1  = (undefined4 *)pass1_1030_5b5c(param_2, param_3);
    local_c = *puVar1;
    uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
    pass1_1008_3e94(param_6, (ushort *)CONCAT22(param_1, &local_10), (ushort *)CONCAT22(param_1, &local_e));
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_c),
                    (ushort *)CONCAT22(param_1, &local_14),
                    (ushort *)CONCAT22(param_1, &local_12));
    if((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1))
    {
        return 0x1;
    }
    PTR_LOOP_1050_50ca = (undefined *)0x6b8;
    return 0x0;
}


void __stdcall16far
pass1_1028_c522(ushort param_1, ushort param_2, ushort *param_3, ulong param_4, long param_5, ushort param_6)

{
    int        iVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    undefined  local_4[0x2];

    uVar4 = pass1_1030_bcae((ushort)local_4, param_6);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar1 = (int)uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar4 = *(ulong *)(iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
    puVar2 = local_4;
    pass1_1030_bcde(param_6, (ushort)puVar2, param_6, uVar4 & 0xffff | (ulong)uVar3 << 0x10, param_3, param_5);
    if((int)puVar2 < 0x0)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
    }
    else
    {
        if((int)puVar2 < 0x1f)
        {
            return;
        }
        PTR_LOOP_1050_50ca = (undefined *)0x6b6;
        PTR_LOOP_1050_50cc = puVar2 + -0x1e;
    }
    return;
}


BOOL16 __stdcall16far pass1_1028_c64a(
  ulong param_1, ulong *param_2, uint param_3, ushort param_4, ushort param_5, long param_6, ushort param_7)

{
    BOOL16     BVar1;
    ushort     uVar2;
    ushort     uVar3;
    undefined  local_e[0x2];
    int        local_c;
    int        local_a;
    undefined4 local_8;
    undefined2 uStack4;

    local_8 = *param_2;
    uStack4 = *(undefined2 *)(param_2 + 0x1);
    pass1_1008_3eb4((ushort *)CONCAT22(param_7, &local_8),
                    (ushort *)CONCAT22(param_7, local_e),
                    (ushort *)CONCAT22(param_7, &local_c),
                    (ushort *)CONCAT22(param_7, &local_a));
    local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
    uVar2   = (ushort)param_1;
    uVar3   = (ushort)(param_1 >> 0x10);
    BVar1   = pass1_1028_c5a6(
      uVar2, uVar3, 0x7b, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
    if(BVar1 == 0x0)
    {
        local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
        BVar1   = pass1_1028_c5a6(
          uVar2, uVar3, 0x7b, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
        if(BVar1 == 0x0)
        {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            BVar1         = pass1_1028_c5a6(
              uVar2, uVar3, 0x7c, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
            if(BVar1 == 0x0)
            {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                BVar1   = pass1_1028_c5a6(
                  uVar2, uVar3, 0x7c, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
                if(BVar1 == 0x0)
                {
                    return BVar1;
                }
            }
        }
    }
    return 0x1;
}


void __stdcall16far pass1_1028_c724(ulong param_1)

{
    uint         uVar1;
    undefined4   uVar2;
    astruct_295 *iVar3;
    undefined2   uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_295 *)param_1;
    uVar2 = iVar3->field_0x14;
    if(*(int *)((int)uVar2 + 0xac) != 0x0)
    {
        return;
    }
    uVar2 = iVar3->field_0x14;
    uVar1 = *(uint *)((int)uVar2 + 0xa6);
    if(uVar1 == 0xd)
    {
        uVar2                              = iVar3->field_0x14;
        *(undefined2 *)((int)uVar2 + 0xac) = 0x1;
        goto LAB_1028_c770;
    }
    if(uVar1 < 0xe)
    {
        if((char)uVar1 == '\0')
            goto LAB_1028_c770;
        if((char)uVar1 == '\a')
        {
            uVar2                              = iVar3->field_0x14;
            *(undefined2 *)((int)uVar2 + 0xac) = 0xa;
            goto LAB_1028_c770;
        }
    }
    uVar2                              = iVar3->field_0x14;
    *(undefined2 *)((int)uVar2 + 0xac) = 0x5;
LAB_1028_c770:
    uVar2 = iVar3->field_0x14;
    if(*(int *)((int)uVar2 + 0xac) == 0x0)
    {
        uVar2 = iVar3->field_0x14;
        if(*(int *)((int)uVar2 + 0xa8) != 0x0)
        {
            uVar2                              = iVar3->field_0x14;
            *(undefined2 *)((int)uVar2 + 0xac) = 0x1;
        }
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_c7b6(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ushort *param_5, long param_6)

{
    undefined4 *puVar1;
    uint        uVar2;
    uint        uVar3;
    ulong       uVar4;
    byte        bStack27;
    undefined4  local_a;
    undefined4  uStack6;

    puVar1 = &local_a;
    pass1_1030_64ce(
      param_1, puVar1, param_2, _PTR_LOOP_1050_5740, param_5, param_6, (ulong *)CONCAT22(param_1, puVar1));
    uStack6  = *puVar1;
    uVar3    = *(uint *)((int)puVar1 + 0x2);
    bStack27 = (byte)((ulong)uStack6 >> 0x18);
    uVar2    = (uint)bStack27;
    if(bStack27 == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack6, uVar3);
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, uVar2));
    uVar3 = (uint)(uVar4 >> 0x10);
    if((uVar3 | (uint)uVar4) != 0x0)
    {
        switch(*(undefined2 *)((uint)uVar4 + 0xc))
        {
        case 0x1:
            break;
        case 0x2:
            break;
        case 0x3:
            break;
        case 0x4:
            break;
        case 0x5:
            break;
        case 0x6:
            break;
        case 0x7:
            return;
        case 0x8:
            return;
        case 0x9:
            return;
        }
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_c89c(ulong param_1, ushort *param_2, ulong *param_3, int param_4, ushort param_5)

{
    ulong     *puVar1;
    undefined2 extraout_DX;
    undefined2 uVar2;
    ulong      local_16[0x3];
    long       lStack10;
    undefined4 uStack6;

    pass1_1028_b58e(param_1);
    uStack6  = CONCAT22(extraout_DX, param_4);
    lStack10 = *(long *)(param_4 + 0x8);
    puVar1   = local_16;
    uVar2    = extraout_DX;
    pass1_1030_64ce(
      param_5, puVar1, extraout_DX, _PTR_LOOP_1050_5740, param_2, lStack10, (ulong *)CONCAT22(param_5, puVar1));
    *param_3 = *puVar1;
    return;
}


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1028_c8ee(ushort param_1, ushort param_2, ushort param_3, int param_4, ushort *param_5)

{
    ushort     local_8;
    undefined4 local_6;

    pass1_1008_3eb4(param_5,
                    (ushort *)CONCAT22(param_1, &local_8),
                    (ushort *)CONCAT22(param_1, &local_6),
                    (ushort *)CONCAT22(param_1, (int)&local_6 + 0x2));
    if(param_4 == 0x1)
    {
        local_8 = local_8 + 0x1;
    }
    else
    {
        if(param_4 == 0x2)
        {
            local_6 = local_6 & 0xffff0000 | (ulong)((int)local_6 - 0x1);
        }
        else
        {
            if(param_4 == 0x3)
            {
                local_6 = local_6 & 0xffff0000 | (ulong)((int)local_6 + 0x1);
            }
            else
            {
                if(param_4 == 0x4)
                {
                    local_6 = local_6 & 0xffff | (ulong)(local_6._2_2_ + 0x1) << 0x10;
                }
                else
                {
                    if(param_4 == 0x5)
                    {
                        local_6 = local_6 & 0xffff | (ulong)(local_6._2_2_ - 0x1) << 0x10;
                    }
                }
            }
        }
    }
    pass1_1008_3e76(param_5, local_8, (ushort)local_6, (ushort)(local_6 >> 0x10));
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_c952(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uint         uVar1;
    undefined4   uVar2;
    ulong        uVar4;
    astruct_600 *uVar3;
    BOOL16       BVar5;
    astruct_600 *paVar6;
    astruct_600 *paVar7;
    undefined4   uVar8;
    ulong        uVar9;
    uint         uVar10;
    int          iVar11;
    undefined2   uVar12;
    undefined2   uVar13;
    undefined4   uStack30;
    ushort       uStack16;
    uint         uStack14;

    uVar12 = (undefined2)(param_1 >> 0x10);
    iVar11 = (int)param_1;
    uVar2  = *(undefined4 *)(iVar11 + 0x14);
    uVar3  = (astruct_600 *)uVar2;
    uVar10 = *(uint *)(iVar11 + 0x16) | (uint)uVar3;
    if(uVar10 != 0x0)
    {
        uVar8 = uVar2;
        pass1_1028_b58e(param_1);
        uVar4    = *(ulong *)((int)uVar8 + 0x2e);
        uStack14 = (uint)uVar4;
        if(((*(uint *)((int)uVar8 + 0x30) | uStack14) != 0x0)
           && (uVar13 = (undefined2)(uVar4 >> 0x10), *(int *)(uStack14 + 0x206) == 0x0))
        {
            BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(iVar11 + 0xc), 0x32);
            if(BVar5 == 0x0)
            {
                BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(iVar11 + 0xc), 0x33);
                if((BVar5 != 0x0) && ((int)((qword)*_PTR_LOOP_1050_65e2 % 0x5) == 0x0))
                {
                    return;
                }
            }
            else
            {
                if((int)((qword)*_PTR_LOOP_1050_65e2 % 0xa) == 0x0)
                {
                    return;
                }
            }
            uVar12 = (undefined2)((ulong)uVar2 >> 0x10);
            if(*(int *)(uStack14 + 0x204) == 0x0)
            {
                for(uStack16 = 0x0; (int)uStack16 < 0x25; uStack16 = uStack16 + 0x1)
                {
                    uStack30 = *(ulong *)(&uVar3->field_0x0 + uStack16 * 0x4);
                    paVar7   = (astruct_600 *)uStack30;
                    uVar10   = *(uint *)(&uVar3->field_0x2 + uStack16 * 0x4) | (uint)paVar7;
                    if(uVar10 != 0x0)
                    {
                        uVar9 = uStack30;
                        empty_1038_540a();
                        uStack30._2_2_ = (uint)(uStack30 >> 0x10);
                        paVar6         = uVar3;
                        if((uVar9 & 0xffff | (ulong)uVar10 << 0x10) < uStack30)
                        {
                            paVar6  = (astruct_600 *)((int)paVar7 - (int)(astruct_600 *)uVar9);
                            param_3 = (uStack30._2_2_ - uVar10) - (uint)(paVar7 < (astruct_600 *)uVar9);
                            pass1_1038_52b8(uVar4,
                                            CONCAT22(param_3, paVar6),
                                            0x21,
                                            (ushort)paVar6,
                                            param_3,
                                            (ushort)&PTR_LOOP_1050_1038,
                                            param_4);
                            uStack30 = CONCAT22((uStack30._2_2_ - param_3) - (uint)(paVar7 < paVar6),
                                                (int)paVar7 - (int)paVar6);
                        }
                        if((uStack30._2_2_ | (uint)uStack30) != 0x0)
                        {
                            pass1_1038_52b8(
                              uVar4, uStack30, uStack16, (ushort)paVar6, param_3, (ushort)&PTR_LOOP_1050_1038, param_4);
                        }
                    }
                }
            }
            else
            {
                uVar10 = uVar3->field_0x8c;
                uVar1  = uVar3->field_0x8e;
                if((uVar1 | uVar10) != 0x0)
                {
                    pass1_1038_52b8(
                      uVar4, CONCAT22(uVar1, uVar10), 0x23, param_2, param_3, (ushort)&PTR_LOOP_1050_1038, param_4);
                }
                uVar10 = uVar3->field_0x90;
                uVar1  = uVar3->field_0x92;
                if((uVar1 | uVar10) != 0x0)
                {
                    pass1_1038_52b8(
                      uVar4, CONCAT22(uVar1, uVar10), 0x24, param_2, param_3, (ushort)&PTR_LOOP_1050_1038, param_4);
                    return;
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_cb04(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    ulong      uVar1;
    undefined4 uVar2;
    ulong      uVar3;
    uint       uVar4;
    ulong      uVar5;
    undefined4 uVar6;
    long       lVar7;
    uint       extraout_DX;
    ushort     uVar8;
    undefined2 uVar9;
    undefined2 uVar10;
    bool       bVar11;
    ushort    *puVar12;
    uchar     *puStack52;
    uint       uStack38;
    uchar     *puStack36;
    int        iStack22;
    uint       uStack18;
    uchar     *puStack16;
    uint       uStack14;

    uVar1 = *(ulong *)((int)param_1 + 0x14);
    if(uVar1 != 0x0)
    {
        uVar5 = uVar1;
        pass1_1028_b58e(param_1);
        uVar3     = uVar5 & 0xffff | (ulong)extraout_DX << 0x10;
        uVar2     = *(undefined4 *)((int)uVar5 + 0x2e);
        puStack52 = *(uchar **)((int)uVar5 + 0x30);
        uStack14  = (uint)uVar2;
        uStack18  = (uint)puStack52 | uStack14;
        if(uStack18 != 0x0)
        {
            uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
            if(*(int *)(uStack14 + 0x206) != 0x0)
            {
                return;
            }
            uVar8  = (ushort)uVar1;
            uVar10 = (undefined2)(uVar1 >> 0x10);
            if(*(int *)(uStack14 + 0x204) != 0x0)
            {
                uVar2 = *(undefined4 *)(uVar8 + 0x8c);
                uVar6 = uVar2;
                empty_1038_540a();
                puStack36 = (uchar *)((ulong)uVar2 >> 0x10);
                if((puStack52 <= puStack36)
                   && ((uVar4 = (uint)uVar6, uStack38 = (uint)uVar2, puStack52 < puStack36 || (uVar4 < uStack38))))
                {
                    pass1_1030_7d7c(uVar3,
                                    uStack38 - uVar4,
                                    CONCAT22(0x23, puStack36 + (-(uint)(uStack38 < uVar4) - (int)puStack52)),
                                    uVar4,
                                    puStack52,
                                    param_2,
                                    param_3,
                                    param_4);
                    puVar12   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, puStack52, param_3);
                    puStack52 = (uchar *)((ulong)puVar12 >> 0x10);
                    pass1_1010_043a(
                      (ulong)puVar12 & 0xffff | ZEXT24(puStack52) << 0x10, *(long *)(uStack14 + 0x4), 0x12, param_4);
                }
                uVar2 = *(undefined4 *)(uVar8 + 0x90);
                uVar6 = uVar2;
                empty_1038_540a();
                puStack36 = (uchar *)((ulong)uVar2 >> 0x10);
                if((puStack52 <= puStack36)
                   && ((uVar4 = (uint)uVar6, uStack38 = (uint)uVar2, puStack52 < puStack36 || (uVar4 < uStack38))))
                {
                    pass1_1030_7d7c(uVar3,
                                    uStack38 - uVar4,
                                    CONCAT22(0x24, puStack36 + (-(uint)(uStack38 < uVar4) - (int)puStack52)),
                                    uVar4,
                                    puStack52,
                                    param_2,
                                    param_3,
                                    param_4);
                }
                return;
            }
            empty_1038_540a();
            puStack16 = puStack52;
            for(iStack22 = 0x11; iStack22 < 0x25; iStack22 = iStack22 + 0x1)
            {
                uVar1 = *(ulong *)(iStack22 * 0x4 + uVar8);
                uVar5 = uVar1;
                empty_1038_540a();
                uVar5     = uVar5 & 0xffff | ZEXT24(puStack52) << 0x10;
                puStack52 = (uchar *)(uVar1 >> 0x10);
                if(uVar5 < uVar1)
                {
                    if((((iStack22 == 0x23) || (iStack22 == 0x24)) || (puStack16 < puStack52))
                       || ((uVar4 = (uint)uVar1, puStack16 <= puStack52 && (uStack18 < uVar4))))
                    {
                        lVar7 = uVar1 - uVar5;
                        uVar4 = (uint)lVar7;
                        pass1_1030_7d7c(uVar3,
                                        uVar4,
                                        CONCAT22(iStack22, (int)((ulong)lVar7 >> 0x10)),
                                        uVar4,
                                        puStack52,
                                        uVar8,
                                        param_3,
                                        param_4);
                        if(iStack22 == 0x23)
                        {
                            puVar12   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, puStack52, param_3);
                            puStack52 = (uchar *)((ulong)puVar12 >> 0x10);
                            pass1_1010_043a((ulong)puVar12 & 0xffff | ZEXT24(puStack52) << 0x10,
                                            *(long *)(uStack14 + 0x4),
                                            0x12,
                                            param_4);
                        }
                    }
                    else
                    {
                        bVar11    = uStack18 < uVar4;
                        uStack18  = uStack18 - uVar4;
                        puStack16 = puStack16 + (-(uint)bVar11 - (int)puStack52);
                    }
                }
            }
            return;
        }
    }
    return;
}


ushort __stdcall16far pass1_1028_ced2(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uint       uVar1;
    bool       bVar2;
    bool       bVar3;
    ulong      uVar4;
    uint       uVar5;
    undefined2 uVar6;
    undefined2 uVar7;

    uVar1 = (uint)((ulong)param_1 >> 0x10);
    bVar2 = (*(byte *)((int)param_1 + 0x1a) & 0x2) == 0x0;
    if(bVar2)
    {
        uVar6   = 0x0;
        uVar7   = 0x23;
        uVar5   = 0x1;
        uVar4   = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        param_4 = 0x1030;
        pass1_1030_7d7c(
          uVar4, uVar5, CONCAT22(uVar7, uVar6), (uint)uVar4, (uchar *)(uVar4 >> 0x10), param_2, param_3, param_5);
    }
    bVar3 = (*(byte *)((int)param_1 + 0x1a) & 0x1) == 0x0;
    if(bVar3)
    {
        uVar6   = 0x0;
        uVar7   = 0xe;
        uVar5   = 0x1;
        uVar4   = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        param_4 = 0x1030;
        pass1_1030_7d7c(
          uVar4, uVar5, CONCAT22(uVar7, uVar6), (uint)uVar4, (uchar *)(uVar4 >> 0x10), param_2, param_3, param_5);
    }
    if(bVar3 || bVar2)
    {
        pass1_1028_bdac(param_1, 0x6, param_4);
        return 0x0;
    }
    return 0x1;
}


astruct_18 *__stdcall16far pass1_1028_cf44(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1028_cfd2(ulong *param_1, ulong param_2)

{
    *param_1                            = param_2;
    *(undefined4 *)((int)param_1 + 0x4) = 0x0;
    return;
}


void __stdcall16far pass1_1028_cff2(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined2  uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0x4);
    uVar2  = *(uint *)((int)param_1 + 0x6);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    return;
}


void __stdcall16far pass1_1028_d172(ushort param_1, ulong param_2, ulong param_3)

{
    uint      uVar1;
    long      lVar2;
    ulong     uVar3;
    undefined local_e[0x8];
    undefined local_6[0x4];

    pass1_1018_dcf6((ushort *)CONCAT22(param_1, local_6));
    pass1_1008_5784((ulong *)CONCAT22(param_1, local_e), param_3);
    while(true)
    {
        lVar2 = pass1_1008_5b12(local_e, param_1);
        uVar1 = (uint)((ulong)lVar2 >> 0x10);
        if(lVar2 == 0x0)
            break;
        uVar3 = pass1_1018_dd1e(param_1,
                                (uint)local_6,
                                (uchar *)(uVar1 | (uint)lVar2),
                                (ushort)local_6,
                                param_1,
                                0x0,
                                (ulong) * (uint *)((uint)lVar2 + 0x4) << 0x10);
        pass1_1008_8faa(*(undefined4 *)((int)param_2 + 0x4), uVar3);
    }
    return;
}


ulong __stdcall16far pass1_1028_b4f2(ulong param_1)

{
    undefined2 uVar1;
    undefined4 uVar2;

    uVar2 = pass1_1028_b58e(param_1);
    uVar1 = (undefined2)((ulong)uVar2 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)uVar2 + 0x30), *(undefined2 *)((int)uVar2 + 0x2e));
}
