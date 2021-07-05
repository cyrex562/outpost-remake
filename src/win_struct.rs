// typedef struct _CONTEXT _CONTEXT, *P_CONTEXT;

// typedef struct _CONTEXT CONTEXT;

struct CONTEXT {
    // DWORD ContextFlags;
    context_flags: u32,
    // DWORD Dr0;
    dr0: u32,
    // DWORD Dr1;
    dr1: u32,
    // DWORD Dr2;
    dr2: u32,
    // DWORD Dr3;
    dr3: u32,
    // DWORD Dr6;
    dr6: u32,
    // DWORD Dr7;
    dr7: u32,
    // FLOATING_SAVE_AREA FloatSave;
    float_save: FLOATING_SAVE_AREA,
    // DWORD SegGs;
    seg_gs: u32,
    // DWORD SegFs;
    seg_fs: u32,
    // DWORD SegEs;
    seg_es: u32,
    // DWORD SegDs;
    seg_ds: u32
    // DWORD Edi;,
    edi: u32,
    // DWORD Esi;
    esi: u32,
    // DWORD Ebx;
    ebx: u32,
    // DWORD Edx;
    edx: u32,
    // DWORD Ecx;
    ecx: u32,
    // DWORD Eax;
    eax: u32,
    // DWORD Ebp;
    ebp: u32,
    // DWORD Eip;
    eip: u32,
    // DWORD SegCs;
    seg_cs: u32,
    // DWORD EFlags;
    e_flags: u32,
    // DWORD Esp;
    esp: u32,
    // DWORD SegSs;
    seg_ss: u32,
    // BYTE ExtendedRegisters[512];
    extended_registers: [u8;512]
}