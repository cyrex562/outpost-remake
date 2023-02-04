#pragma once

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include "Windows.h"
#include "timeapi.h"
#include "ddraw.h"
#elif __linux__

#include "windows_shim.h"
#endif

