cmake_minimum_required(VERSION 3.21)
project(outpost_c_proj C)

set(CMAKE_C_STANDARD 23)

add_executable(outpost_c_proj main.c file2.c file2.h program.h types.h structs.h globals.h sys_api.c sys_api.h utils.c utils.h structs_2.h entry.c entry.h ops_1.c ops_1.h ops_2.c ops_2.h ops_3.c ops_3.h string_defs.h)
