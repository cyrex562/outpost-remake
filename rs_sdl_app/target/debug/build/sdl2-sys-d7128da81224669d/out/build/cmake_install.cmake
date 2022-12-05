# Install script for directory: C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/Debug/SDL2d.lib")
  elseif(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/Release/SDL2.lib")
  elseif(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/MinSizeRel/SDL2.lib")
  elseif(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/RelWithDebInfo/SDL2.lib")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/Debug/SDL2maind.lib")
  elseif(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/Release/SDL2main.lib")
  elseif(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/MinSizeRel/SDL2main.lib")
  elseif(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/RelWithDebInfo/SDL2main.lib")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2Targets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2Targets.cmake"
         "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2Targets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2Targets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2Targets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2Targets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2Targets-debug.cmake")
  endif()
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2Targets-minsizerel.cmake")
  endif()
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2Targets-relwithdebinfo.cmake")
  endif()
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2Targets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Devel" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/SDL2Config.cmake"
    "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/SDL2ConfigVersion.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SDL2" TYPE FILE FILES
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_assert.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_atomic.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_audio.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_bits.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_blendmode.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_clipboard.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_android.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_emscripten.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_iphoneos.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_macosx.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_minimal.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_os2.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_pandora.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_psp.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_windows.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_winrt.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_config_wiz.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_copying.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_cpuinfo.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_egl.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_endian.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_error.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_events.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_filesystem.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_gamecontroller.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_gesture.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_haptic.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_hidapi.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_hints.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_joystick.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_keyboard.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_keycode.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_loadso.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_locale.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_log.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_main.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_messagebox.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_metal.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_misc.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_mouse.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_mutex.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_name.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengl.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengl_glext.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengles.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengles2.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengles2_gl2.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengles2_gl2ext.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengles2_gl2platform.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_opengles2_khrplatform.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_pixels.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_platform.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_power.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_quit.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_rect.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_render.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_rwops.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_scancode.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_sensor.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_shape.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_stdinc.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_surface.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_system.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_syswm.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_assert.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_common.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_compare.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_crc32.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_font.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_fuzzer.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_harness.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_images.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_log.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_md5.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_memory.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_test_random.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_thread.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_timer.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_touch.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_types.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_version.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_video.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/SDL_vulkan.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/begin_code.h"
    "C:/Users/cyrex/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-sys-0.35.2/SDL/include/close_code.h"
    "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/include/SDL_config.h"
    "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/include/SDL_revision.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "C:/Users/cyrex/Projects/outpost-rs/rs_sdl_app/target/debug/build/sdl2-sys-d7128da81224669d/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
