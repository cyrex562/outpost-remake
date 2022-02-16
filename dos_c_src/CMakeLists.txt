cmake_minimum_required(VERSION 3.0)
project(outpost_1)

set_property(GLOBAL PROPERTY C_STANDARD 14)

file(GLOB SOURCES src/*.c)
file(GLOB HEADERS src/*.h)

include_directories(./src)

add_executable(efs_cpp ${SOURCES} ${HEADERS} )