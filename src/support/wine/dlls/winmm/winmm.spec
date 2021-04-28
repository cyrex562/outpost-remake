# ordinal exports
2 stdcall @(ptr long long) PlaySoundA
3 stub @
4 stub @

@ stdcall CloseDriver(long long long)
@ stdcall DefDriverProc(long long long long long)
@ stdcall DriverCallback(long long long long long long long)
@ stdcall DrvClose(long long long) CloseDriver
@ stdcall DrvDefDriverProc(long long long long long) DefDriverProc
@ stdcall DrvGetModuleHandle GetDriverModuleHandle
@ stdcall DrvOpen(wstr wstr long) OpenDriver
@ stdcall DrvOpenA(str str long) OpenDriverA
@ stdcall DrvSendMessage(long long long long) SendDriverMessage
@ stdcall GetDriverFlags
@ stdcall GetDriverModuleHandle
@ stdcall OpenDriver(wstr wstr long)
@ stdcall OpenDriverA(str str long)
@ stdcall PlaySound(ptr long long) PlaySoundA
@ stdcall PlaySoundA(ptr long long)
@ stdcall PlaySoundW(ptr long long)
@ stdcall SendDriverMessage(long long long long)
@ stdcall auxGetDevCapsA(long ptr long)
@ stdcall auxGetDevCapsW(long ptr long)
@ stdcall auxGetNumDevs()
@ stdcall auxGetVolume(long ptr)
@ stdcall auxOutMessage(long long long long)
@ stdcall auxSetVolume(long long)
@ stdcall joyConfigChanged
@ stdcall joyGetDevCapsA(long ptr long)
@ stdcall joyGetDevCapsW(long ptr long)
@ stdcall joyGetNumDevs()
@ stdcall joyGetPos(long ptr)
@ stdcall joyGetPosEx(long ptr)
@ stdcall joyGetThreshold(long ptr)
@ stdcall joyReleaseCapture
@ stdcall joySetCapture(long long long long)
@ stdcall joySetThreshold(long long)
@ stdcall mciDriverNotify(long long long)
@ stdcall mciDriverYield
@ stdcall mciExecute(str)
@ stdcall mciFreeCommandResource
@ stdcall mciGetCreatorTask
@ stdcall mciGetDeviceIDA(str)
@ stdcall mciGetDeviceIDFromElementIDA(long str)
@ stdcall mciGetDeviceIDFromElementIDW(long wstr)
@ stdcall mciGetDeviceIDW(wstr)
@ stdcall mciGetDriverData
@ stdcall mciGetErrorStringA(long ptr long)
@ stdcall mciGetErrorStringW(long ptr long)
@ stdcall mciGetYieldProc(long ptr)
@ stdcall mciLoadCommandResource(long wstr long)
@ stdcall mciSendCommandA(long long long long)
@ stdcall mciSendCommandW(long long long long)
@ stdcall mciSendStringA(str ptr long long)
@ stdcall mciSendStringW(wstr ptr long long)
@ stdcall mciSetDriverData(long long)
@ stdcall mciSetYieldProc(long ptr long)
@ stdcall midiConnect(long long ptr)
@ stdcall midiDisconnect(long long ptr)
@ stdcall midiInAddBuffer(long ptr long)
@ stdcall midiInClose
@ stdcall midiInGetDevCapsA(long ptr long)
@ stdcall midiInGetDevCapsW(long ptr long)
@ stdcall midiInGetErrorTextA(long ptr long) midiOutGetErrorTextA
@ stdcall midiInGetErrorTextW(long ptr long) midiOutGetErrorTextW
@ stdcall midiInGetID(long ptr)
@ stdcall midiInGetNumDevs()
@ stdcall midiInMessage(long long long long)
@ stdcall midiInOpen(ptr long long long long)
@ stdcall midiInPrepareHeader(long ptr long)
@ stdcall midiInReset
@ stdcall midiInStart
@ stdcall midiInStop
@ stdcall midiInUnprepareHeader(long ptr long)
@ stdcall midiOutCacheDrumPatches(long long ptr long)
@ stdcall midiOutCachePatches(long long ptr long)
@ stdcall midiOutClose
@ stdcall midiOutGetDevCapsA(long ptr long)
@ stdcall midiOutGetDevCapsW(long ptr long)
@ stdcall midiOutGetErrorTextA(long ptr long)
@ stdcall midiOutGetErrorTextW(long ptr long)
@ stdcall midiOutGetID(long ptr)
@ stdcall midiOutGetNumDevs()
@ stdcall midiOutGetVolume(long ptr)
@ stdcall midiOutLongMsg(long ptr long)
@ stdcall midiOutMessage(long long long long)
@ stdcall midiOutOpen(ptr long long long long)
@ stdcall midiOutPrepareHeader(long ptr long)
@ stdcall midiOutReset
@ stdcall midiOutSetVolume(long long)
@ stdcall midiOutShortMsg(long long)
@ stdcall midiOutUnprepareHeader(long ptr long)
@ stdcall midiStreamClose
@ stdcall midiStreamOpen(ptr ptr long long long long)
@ stdcall midiStreamOut(long ptr long)
@ stdcall midiStreamPause
@ stdcall midiStreamPosition(long ptr long)
@ stdcall midiStreamProperty(long ptr long)
@ stdcall midiStreamRestart
@ stdcall midiStreamStop
@ stdcall mixerClose
@ stdcall mixerGetControlDetailsA(long ptr long)
@ stdcall mixerGetControlDetailsW(long ptr long)
@ stdcall mixerGetDevCapsA(long ptr long)
@ stdcall mixerGetDevCapsW(long ptr long)
@ stdcall mixerGetID(long ptr long)
@ stdcall mixerGetLineControlsA(long ptr long)
@ stdcall mixerGetLineControlsW(long ptr long)
@ stdcall mixerGetLineInfoA(long ptr long)
@ stdcall mixerGetLineInfoW(long ptr long)
@ stdcall mixerGetNumDevs()
@ stdcall mixerMessage(long long long long)
@ stdcall mixerOpen(ptr long long long long)
@ stdcall mixerSetControlDetails(long ptr long)
@ stdcall mmGetCurrentTask()
@ stdcall mmTaskBlock
@ stdcall mmTaskCreate(ptr ptr long)
@ stdcall mmTaskSignal
@ stdcall mmTaskYield()
@ stdcall mmioAdvance(long ptr long)
@ stdcall mmioAscend(long ptr long)
@ stdcall mmioClose(long long)
@ stdcall mmioCreateChunk(long ptr long)
@ stdcall mmioDescend(long ptr ptr long)
@ stdcall mmioFlush(long long)
@ stdcall mmioGetInfo(long ptr long)
@ stub mmioInstallIOProc16
@ stdcall mmioInstallIOProcA(long ptr long)
@ stdcall mmioInstallIOProcW(long ptr long)
@ stdcall mmioOpenA(str ptr long)
@ stdcall mmioOpenW(wstr ptr long)
@ stdcall mmioRead(long ptr long)
@ stdcall mmioRenameA(str str ptr long)
@ stdcall mmioRenameW(wstr wstr ptr long)
@ stdcall mmioSeek(long long long)
@ stdcall mmioSendMessage(long long long long)
@ stdcall mmioSetBuffer(long ptr long long)
@ stdcall mmioSetInfo(long ptr long)
@ stdcall mmioStringToFOURCCA(str long)
@ stdcall mmioStringToFOURCCW(wstr long)
@ stdcall mmioWrite(long ptr long)
@ stdcall mmsystemGetVersion()
@ stdcall sndPlaySoundA(ptr long)
@ stdcall sndPlaySoundW(ptr long)
@ stdcall timeBeginPeriod
@ stdcall timeEndPeriod
@ stdcall timeGetDevCaps(ptr long)
@ stdcall timeGetSystemTime(ptr long)
@ stdcall timeGetTime() kernel32.GetTickCount
@ stdcall timeKillEvent
@ stdcall timeSetEvent(long long ptr long long)
@ stdcall waveInAddBuffer(long ptr long)
@ stdcall waveInClose
@ stdcall waveInGetDevCapsA(long ptr long)
@ stdcall waveInGetDevCapsW(long ptr long)
@ stdcall waveInGetErrorTextA(long ptr long) waveOutGetErrorTextA
@ stdcall waveInGetErrorTextW(long ptr long) waveOutGetErrorTextW
@ stdcall waveInGetID(long ptr)
@ stdcall waveInGetNumDevs()
@ stdcall waveInGetPosition(long ptr long)
@ stdcall waveInMessage(long long long long)
@ stdcall waveInOpen(ptr long ptr long long long)
@ stdcall waveInPrepareHeader(long ptr long)
@ stdcall waveInReset
@ stdcall waveInStart
@ stdcall waveInStop
@ stdcall waveInUnprepareHeader(long ptr long)
@ stdcall waveOutBreakLoop
@ stdcall waveOutClose
@ stdcall waveOutGetDevCapsA(long ptr long)
@ stdcall waveOutGetDevCapsW(long ptr long)
@ stdcall waveOutGetErrorTextA(long ptr long)
@ stdcall waveOutGetErrorTextW(long ptr long)
@ stdcall waveOutGetID(long ptr)
@ stdcall waveOutGetNumDevs()
@ stdcall waveOutGetPitch(long ptr)
@ stdcall waveOutGetPlaybackRate(long ptr)
@ stdcall waveOutGetPosition(long ptr long)
@ stdcall waveOutGetVolume(long ptr)
@ stdcall waveOutMessage(long long long long)
@ stdcall waveOutOpen(ptr long ptr long long long)
@ stdcall waveOutPause
@ stdcall waveOutPrepareHeader(long ptr long)
@ stdcall waveOutReset
@ stdcall waveOutRestart
@ stdcall waveOutSetPitch(long long)
@ stdcall waveOutSetPlaybackRate(long long)
@ stdcall waveOutSetVolume(long long)
@ stdcall waveOutUnprepareHeader(long ptr long)
@ stdcall waveOutWrite(long ptr long)
# MigrateAllDrivers
# MigrateSoundEvents
# NotifyCallbackData
# WOW32DriverCallback
# WOW32ResolveMultiMediaHandle
# WOWAppExit
# WinmmLogoff
# WinmmLogon
# mid32Message
# mmDrvInstall
# aux32Message
# joy32Message
# mci32Message
# mod32Message
# mxd32Message
# tid32Message
# wid32Message
# winmmDbgOut
# winmmSetDebugLevel
# wod32Message
