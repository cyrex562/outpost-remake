# Yes, ICCompress,ICDecompress,MCIWnd* and ICDraw* are cdecl (VFWAPIV).
# The rest is stdcall (VFWAPI) however. -Marcus Meissner, 990124

2 stdcall VideoForWindowsVersion()

@ stdcall DrawDibBegin(long long long long ptr long long long)
@ stdcall DrawDibChangePalette(long long long ptr)
@ stdcall DrawDibClose
@ stdcall DrawDibDraw(long long long long long long ptr ptr long long long long long)
@ stdcall DrawDibEnd
@ stdcall DrawDibGetBuffer(long ptr long long)
@ stdcall DrawDibGetPalette
@ stdcall DrawDibOpen()
@ stdcall DrawDibProfileDisplay(ptr)
@ stdcall DrawDibRealize(long long long)
@ stdcall DrawDibSetPalette(long long)
@ stdcall DrawDibStart(long long)
@ stdcall DrawDibStop
@ stdcall DrawDibTime(long ptr)
@ stdcall GetOpenFileNamePreview(ptr) GetOpenFileNamePreviewA
@ stdcall GetOpenFileNamePreviewA(ptr)
@ stdcall GetOpenFileNamePreviewW(ptr)
@ stdcall GetSaveFileNamePreviewA(ptr)
@ stdcall GetSaveFileNamePreviewW(ptr)
@ stdcall ICClose
@ cdecl   ICCompress(long long ptr ptr ptr ptr ptr ptr long long long ptr ptr)
@ stdcall ICCompressorChoose(long long ptr ptr ptr ptr)
@ stdcall ICCompressorFree(ptr)
@ cdecl   ICDecompress(long long ptr ptr ptr ptr)
@ cdecl   ICDraw(long long ptr ptr long long)
@ cdecl   ICDrawBegin(long long long long long long long long long ptr long long long long long long)
@ stdcall ICGetDisplayFormat(long ptr ptr long long long)
@ stdcall ICGetInfo(long ptr long)
@ stdcall ICImageCompress(long long ptr ptr ptr long ptr)
@ stdcall ICImageDecompress(long long ptr ptr ptr)
@ stdcall ICInfo(long long ptr)
@ stdcall ICInstall(long long long str long)
@ stdcall ICLocate(long long ptr ptr long)
@ stub    ICMThunk
@ stdcall ICOpen(long long long)
@ stdcall ICOpenFunction(long long long ptr)
@ stdcall ICRemove(long long long)
@ stdcall ICSendMessage(long long long long)
@ stdcall ICSeqCompressFrame(ptr long ptr ptr ptr)
@ stdcall ICSeqCompressFrameEnd(ptr)
@ stdcall ICSeqCompressFrameStart(ptr ptr)
@ cdecl   MCIWndCreate (long long long str) MCIWndCreateA
@ cdecl   MCIWndCreateA (long long long str)
@ cdecl   MCIWndCreateW (long long long wstr)
@ cdecl   MCIWndRegisterClass()
@ stub    StretchDIB
