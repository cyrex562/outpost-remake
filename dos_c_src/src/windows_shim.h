#pragma once

#include "stddef.h"

typedef void* PVOID;
typedef PVOID HANDLE;
typedef unsigned int DWORD;
typedef unsigned short WORD;
typedef wchar_t WCHAR;
typedef char CHAR;
typedef char TCHAR;
typedef unsigned char BYTE;
typedef unsigned int UINT;
typedef unsigned long ULONG;
typedef int INT;
typedef long LONG;
typedef long long LONGLONG;
typedef int BOOL;
typedef unsigned int SIZE_T;
typedef char *LPSTR;
typedef char *LPTSTR;
typedef const char *LPCTSTR;
typedef const char *LPCSTR;
typedef unsigned short *LPWSTR;
typedef const unsigned short *LPCWSTR;
typedef BYTE *LPBYTE;
typedef LONG *LPLONG;
typedef LONG *PLONG;
typedef INT *LPINT;
typedef DWORD *LPDWORD;
typedef DWORD *PDWORD;
typedef BOOL *LPBOOL;
typedef void *LPVOID;
typedef void *FARPROC;
typedef int LCID;
typedef int HKEY;
typedef HKEY *PHKEY;
typedef int REGSAM;
typedef HANDLE HWND;
typedef HANDLE HINSTANCE;
typedef HINSTANCE HMODULE;
typedef int MMRESULT;
typedef int TOKEN_INFORMATION_CLASS;

//typedef void *LPCRITICAL_SECTION;
typedef void *LPSECURITY_ATTRIBUTES;
typedef void *LPSTARTUPINFO;
typedef void *LPPROCESS_INFORMATION;
typedef void *LPTIME_ZONE_INFORMATION;
typedef void *PINPUT_RECORD;
typedef void *PHANDLER_ROUTINE;
typedef void *LPWSAOVERLAPPED;
typedef void *LPWSAOVERLAPPED_COMPLETION_ROUTINE;
typedef void *PSID;
typedef void *PSID_IDENTIFIER_AUTHORITY;
typedef void *PACL;
typedef void *PSECURITY_DESCRIPTOR;
typedef void *LPSERVICE_TABLE_ENTRY;
typedef void *LPSERVICE_STATUS;
typedef void *HCRYPTPROV;
typedef void *LPOVERLAPPED;

#if defined(_WIN64)
    typedef unsigned __int64 ULONG_PTR;
#else
    typedef unsigned long ULONG_PTR;
#endif

typedef struct _LIST_ENTRY {
  struct _LIST_ENTRY *Flink;
  struct _LIST_ENTRY *Blink;
} LIST_ENTRY, *PLIST_ENTRY, PRLIST_ENTRY;


typedef struct _RTL_CRITICAL_SECTION_DEBUG {
    WORD        Type;
    WORD        CreatorBackTraceIndex;
    struct _RTL_CRITICAL_SECTION *CriticalSection;
    LIST_ENTRY  ProcessLocksList;
    DWORD       EntryCount;
    DWORD       ContentionCount;
    DWORD       Flags;
    WORD        CreatorBackTraceIndexHigh;
    WORD        SpareWORD;
} RTL_CRITICAL_SECTION_DEBUG, *PRTL_CRITICAL_SECTION_DEBUG;

typedef struct _RTL_CRITICAL_SECTION {
    PRTL_CRITICAL_SECTION_DEBUG DebugInfo;
    LONG LockCount;
    LONG RecursionCount;
    HANDLE OwningThread;
    HANDLE LockSemaphore;
    ULONG_PTR SpinCount;
} RTL_CRITICAL_SECTION, *PRTL_CRITICAL_SECTION;
typedef RTL_CRITICAL_SECTION    CRITICAL_SECTION;
typedef PRTL_CRITICAL_SECTION   PCRITICAL_SECTION;
typedef PRTL_CRITICAL_SECTION   LPCRITICAL_SECTION;

typedef enum _EXCEPTION_DISPOSITION
{
         ExceptionContinueExecution = 0,
         ExceptionContinueSearch = 1,
         ExceptionNestedException = 2,
         ExceptionCollidedUnwind = 3
} EXCEPTION_DISPOSITION, *PEXCEPTION_DISPOSITION;

struct EXCEPTION_REGISTRATION_RECORD;

typedef struct EXCEPTION_REGISTRATION_RECORD* PEXCEPTION_REGISTRATION_RECORD;

typedef struct _EXCEPTION_RESGISTRATION_RECORD {
    PEXCEPTION_REGISTRATION_RECORD Next;
    PEXCEPTION_DISPOSITION Handler;
} EXCEPTION_REGISTRATION_RECORD;

struct IDirectDraw;
typedef struct IDirectDraw* LPDIRECTDRAW;

// BOOL DeleteFileA(LPCSTR lpFileName);
// void GetLocalTime(LPSYSTEMTIME lpSystemTime);
// BOOL LocalFileTimeToFileTime(FILETIME *lpLocalFileTime, LPFILETIME lpFileTime);
// BOOL DosDateTimeToFileTime(WORD wFatDate, WORD wFatTime, LPFILETIME lpFileTime);
// BOOL FileTimeToDosDateTime(FILETIME *lpFileTime, LPWORD lpFatDate, LPWORD lpFatTime);
// BOOL FileTimeToLocalFileTime(FILETIME *lpFileTime, LPFILETIME lpLocalFileTime);
// SHORT GetKeyState(int nVirtKey);
// BOOL VirtualFree(LPVOID lpAddress, SIZE_T dwSize, u32 dwFreeType);
// u32 SetFilePointer(HANDLE hFile, LONG lDistanceToMove, PLONG lpDistanceToMoveHigh, u32 dwMoveMethod);
// u32 GetCurrentProcessId(void);
// HMODULE GetModuleHandleA(LPCSTR lpModuleName);
// LPWSTR GetCommandLineW(void);
// LPSTR GetCommandLineA(void);
// u32 GetModuleFileNameA(HMODULE hModule, LPSTR lpFilename, u32 nSize);
// u32 GetVersion(void);
// LPCH GetEnvironmentStrings(void);
// BOOL CloseHandle(HANDLE hObject);
// HANDLE CreateFileA(LPCSTR lpFileName, u32 dwDesiredAccess, u32 dwShareMode, LPSECURITY_ATTRIBUTES lpSecurityAttributes, u32 dwCreationDisposition, u32 dwFlagsAndAttributes, HANDLE hTemplateFile);
DWORD GetLastError();



// BOOL ReadFile(HANDLE hFile, LPVOID lpBuffer, u32 nNumberOfBytesToRead, Lu32* lpNumberOfBytesRead, LPOVERLAPPED lpOverlapped);
// BOOL MoveFileA(LPCSTR lpExistingFileName, LPCSTR lpNewFileName);
// u32 GetWindowsDirectoryA(LPSTR lpBuffer, u32 uSize);
// MCIERROR mciSendCommandA(MCIDEVICEID mciId, u32 uMsg, u32_PTR dwParam1, u32_PTR dwParam2);
// BOOL mciGetErrorStringA(MCIERROR mcierr, LPSTR pszText, u32 cchText);
// BOOL GetExitCodeProcess(HANDLE hProcess, Lu32* lpExitCode);
// BOOL CreateProcessA(LPCSTR lpApplicationName, LPSTR lpCommandLine, LPSECURITY_ATTRIBUTES lpProcessAttributes, LPSECURITY_ATTRIBUTES lpThreadAttributes, BOOL bInheritHandles, u32 dwCreationFlags, LPVOID lpEnvironment, LPCSTR lpCurrentDirectory, LPSTARTUPINFOA lpStartupInfo, LPPROCESS_INFORMATION lpProcessInformation);
// BOOL FindNextFileA(HANDLE hFindFile, LPWIN32_FIND_DATAA lpFindFileData);
// BOOL ScreenToClient(HWND hWnd, LPPOINT lpPoint);
// BOOL GetCursorPos(LPPOINT lpPoint);
// LRESULT DispatchMessageA(MSG *lpMsg);
// BOOL TranslateMessage(MSG *lpMsg);
// BOOL ReleaseCapture(void);
// HWND SetCapture(HWND hWnd);
// BOOL PeekMessageA(LPMSG lpMsg, HWND hWnd, u32 wMsgFilterMin, u32 wMsgFilterMax, u32 wRemoveMsg);
// MMRESULT timeEndPeriod(u32 uPeriod);
// MMRESULT timeBeginPeriod(u32 uPeriod);
// BOOL ValidateRect(HWND hWnd, RECT *lpRect);
// BOOL GetUpdateRect(HWND hWnd, LPRECT lpRect, BOOL bErase);
// HCURSOR SetCursor(HCURSOR hCursor);
// void ExitProcess(u32 uExitCode);
// LRESULT DefWindowProcA(HWND hWnd, u32 Msg, WPARAM wParam, LPARAM lParam);
// void PostQuitMessage(int nExitCode);
// HWND CreateWindowExA(u32 dwExStyle, LPCSTR lpClassName, LPCSTR lpWindowName, u32 dwStyle, int X, int Y, int nWidth, int nHeight, HWND hWndParent, HMENU hMenu, HINSTANCE hInstance, LPVOID lpParam);
// BOOL SetForegroundWindow(HWND hWnd);
// ATOM RegisterClassA(WNDCLASSA *lpWndClass);
// HICON LoadIconA(HINSTANCE hInstance, LPCSTR lpIconName);
// HCURSOR LoadCursorA(HINSTANCE hInstance, LPCSTR lpCursorName);
// HWND FindWindowA(LPCSTR lpClassName, LPCSTR lpWindowName);
// BOOL SetCurrentDirectoryA(LPCSTR lpPathName);
// int ReleaseDC(HWND hWnd, HDC hDC);
// u32 GetSystemPaletteEntries(HDC hdc, u32 iStart, u32 cEntries, LPPALETTEENTRY pPalEntries);
// int GetDeviceCaps(HDC hdc, int index);
// HDC GetDC(HWND hWnd);
// BOOL FindClose(HANDLE hFindFile);
// HANDLE FindFirstFileA(LPCSTR lpFileName, LPWIN32_FIND_DATAA lpFindFileData);
// u32 timeGetTime(void);
// BOOL WritePrivateProfileStringA(LPCSTR lpAppName, LPCSTR lpKeyName, LPCSTR lpString, LPCSTR lpFileName);
// int ShowCursor(BOOL bShow);
u32 GetPrivateProfileIntA(LPCSTR lpAppName,
                          LPCSTR lpKeyName,
                          INT nDefault,
                          LPCSTR lpFileName);
// int MessageBoxA(HWND hWnd, LPCSTR lpText, LPCSTR lpCaption, u32 uType);
// LSTATUS RegQueryValueExA(HKEY hKey, LPCSTR lpValueName, Lu32* lpReserved, Lu32* lpType, LPBYTE lpData, Lu32* lpcbData);
// LSTATUS RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, u32 ulOptions, REGSAM samDesired, PHKEY phkResult);
// void Sleep(u32 dwMilliseconds);
// LRESULT SendMessageA(HWND hWnd, u32 Msg, WPARAM wParam, LPARAM lParam);
// BOOL SetEnvironmentVariableW(LPCWSTR lpName, LPCWSTR lpValue);
// u32 CharUpperBuffA(LPSTR lpsz, u32 cchLength);
// BOOL SetEnvironmentVariableA(LPCSTR lpName, LPCSTR lpValue);
// void ExitThread(u32 dwExitCode);
// u32 WaitForSingleObject(HANDLE hHandle, u32 dwMilliseconds);
// HANDLE CreateThread(LPSECURITY_ATTRIBUTES lpThreadAttributes, SIZE_T dwStackSize, LPTHREAD_START_ROUTINE lpStartAddress, LPVOID lpParameter, u32 dwCreationFlags, Lu32* lpThreadId);
// HANDLE GetCurrentThread(void);
// BOOL SetEvent(HANDLE hEvent);
// BOOL GetCPInfo(u32 CodePage, LPCPINFO lpCPInfo);
// u32 GetOEMCP(void);
// u32 GetACP(void);
// BOOL SetConsoleCtrlHandler(PHANDLER_ROUTINE HandlerRoutine, BOOL Add);
// BOOL FreeEnvironmentStringsA(LPCH param_1);
// int WideCharToMultiByte(u32 CodePage, u32 dwFlags, LPCWSTR lpWideCharStr, int cchWideChar, LPSTR lpMultiByteStr, int cbMultiByte, LPCSTR lpDefaultChar, LPBOOL lpUsedDefaultChar);
// BOOL WriteConsoleA(HANDLE hConsoleOutput, void *lpBuffer, u32 nNumberOfCharsToWrite, Lu32* lpNumberOfCharsWritten, LPVOID lpReserved);
// BOOL SetConsoleMode(HANDLE hConsoleHandle, u32 dwMode);
// BOOL GetConsoleMode(HANDLE hConsoleHandle, Lu32* lpMode);
// BOOL ReadConsoleInputA(HANDLE hConsoleInput, PINPUT_RECORD lpBuffer, u32 nLength, Lu32* lpNumberOfEventsRead);
// u32 GetTimeZoneInformation(LPTIME_ZONE_INFORMATION lpTimeZoneInformation);
// BOOL FlushFileBuffers(HANDLE hFile);
// u32 GetCurrentDirectoryA(u32 nBufferLength, LPSTR lpBuffer);
// u32 GetFullPathNameA(LPCSTR lpFileName, u32 nBufferLength, LPSTR lpBuffer, LPSTR *lpFilePart);
// u32 GetFileAttributesA(LPCSTR lpFileName);
// LPTOP_LEVEL_EXCEPTION_FILTER SetUnhandledExceptionFilter(LPTOP_LEVEL_EXCEPTION_FILTER lpTopLevelExceptionFilter);
// LONG UnhandledExceptionFilter(_EXCEPTION_POINTERS *ExceptionInfo);
// FARPROC GetProcAddress(HMODULE hModule, LPCSTR lpProcName);
// HMODULE LoadLibraryA(LPCSTR lpLibFileName);
// SIZE_T VirtualQuery(LPCVOID lpAddress, PMEMORY_BASIC_INFORMATION lpBuffer, SIZE_T dwLength);
// int MultiByteToWideChar(u32 CodePage, u32 dwFlags, LPCSTR lpMultiByteStr, int cbMultiByte, LPWSTR lpWideCharStr, int cchWideChar);
// u32 GetModuleFileNameW(HMODULE hModule, LPWSTR lpFilename, u32 nSize);
// BOOL TlsFree(u32 dwTlsIndex);
// BOOL TlsSetValue(u32 dwTlsIndex, LPVOID lpTlsValue);
// u32 TlsAlloc(void);
// void SetLastError(u32 dwErrCode);
// LPVOID TlsGetValue(u32 dwTlsIndex);
// void LeaveCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
// void EnterCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
// u32 GetCurrentThreadId(void);
// void DeleteCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
// void InitializeCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
// u32 GetFileType(HANDLE hFile);
// HANDLE CreateEventA(LPSECURITY_ATTRIBUTES lpEventAttributes, BOOL bManualReset, BOOL bInitialState, LPCSTR lpName);
// HANDLE GetStdHandle(u32 nStdHandle);
// BOOL SetStdHandle(u32 nStdHandle, HANDLE hHandle);
// LPVOID VirtualAlloc(LPVOID lpAddress, SIZE_T dwSize, u32 flAllocationType, u32 flProtect);
// BOOL WriteFile(HANDLE hFile, LPCVOID lpBuffer, u32 nNumberOfBytesToWrite, Lu32* lpNumberOfBytesWritten, LPOVERLAPPED lpOverlapped);
// void call_fn_ptr_004bb4c4(void);
// HRESULT DirectDrawCreate(GUID *lp_guid, LPDIRECTDRAW *lp_lp_dd, IUnknown *p_unk_outer);
