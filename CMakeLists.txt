# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.25 FATAL_ERROR)

set(CMAKE_COLOR_DIAGNOSTICS ON)

# include support definitions
if(EXISTS "${CMAKE_SOURCE_DIR}/cmake/cxx-auto-cmake/Support.cmake")
  include("${CMAKE_SOURCE_DIR}/cmake/cxx-auto-cmake/Support.cmake")
else()
  message(FATAL_ERROR
    "\"cxx-auto-cmake/Support.cmake\" not found.\n"
    "Try `git submodule update --init` to fetch the cmake support files.")
endif()

cxxAutoPreInit()

project(cxx-auto
  LANGUAGES C CXX)

cxxAutoPostInit()

# export CMake configuration to `compile_commands.json` for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# configure the include dirs
list(APPEND TARGET_INCLUDE_DIRS ${CMAKE_SOURCE_DIR}/../cxx-auto/..)
list(APPEND TARGET_INCLUDE_DIRS ${CMAKE_SOURCE_DIR}/target/cxxbridge)

# configure the umbrella path
set(TARGET_UMBRELLA_INCLUDE_DIRS ${CMAKE_SOURCE_DIR}/cxx)

# configure the umbrella path
set(TARGET_UMBRELLA_PATH ${CMAKE_SOURCE_DIR}/cxx/lib/umbrella.cxx)

# configure the umbrella include prefix
set(TARGET_UMBRELLA_INCLUDE_PREFIX "cxx-auto/cxx/")

# configure the default target
add_library(cxx-auto STATIC ${TARGET_UMBRELLA_PATH})
target_include_directories(cxx-auto PUBLIC ${TARGET_INCLUDE_DIRS})
target_compile_definitions(cxx-auto PUBLIC ${CXX_AUTO_CONTEXT_COMPILE_DEFINITIONS})
target_compile_options(cxx-auto PUBLIC ${CXX_AUTO_CONTEXT_COMPILE_OPTIONS})

add_custom_target(generate_rust_cxx_header
  BYPRODUCTS ${CMAKE_SOURCE_DIR}/target/cxxbridge/rust/cxx.h
  COMMAND cargo build
  VERBATIM
  USES_TERMINAL)
add_dependencies(cxx-auto generate_rust_cxx_header)

add_custom_target(generate_umbrella
  BYPRODUCTS ${CMAKE_SOURCE_DIR}/cxx/lib/umbrella.cxx
  COMMAND ${CMAKE_COMMAND}
    -D "TARGET_UMBRELLA_PATH=${TARGET_UMBRELLA_PATH}"
    -D "TARGET_UMBRELLA_INCLUDE_DIRS=${TARGET_UMBRELLA_INCLUDE_DIRS}"
    -D "TARGET_UMBRELLA_INCLUDE_PREFIX=${TARGET_UMBRELLA_INCLUDE_PREFIX}"
    -P ${CMAKE_SOURCE_DIR}/cmake/cxx-auto-cmake/command/EmitUmbrella.cmake
  VERBATIM
  USES_TERMINAL)
add_dependencies(cxx-auto generate_umbrella)

foreach(variable ${CXX_AUTO_CONTEXT_VARIABLES})
  set(_definiendum ${variable})
  list(JOIN ${variable} "\\\;" _definiens)
  list(APPEND CXX_AUTO_CONTEXT_DEFINITIONS "-D${_definiendum}=${_definiens}")
endforeach()
set(_definiendum "CXX_AUTO_CONTEXT_VARIABLES")
list(JOIN CXX_AUTO_CONTEXT_VARIABLES "\\\;" _definiens)
list(APPEND CXX_AUTO_CONTEXT_DEFINITIONS "-D${_definiendum}=${_definiens}")
unset(_definiens)
unset(_definiendum)

# define custom target to emit the CMake context for Rust
add_custom_target(emit_cxx_auto_context
  COMMAND ${CMAKE_COMMAND}
    ${CXX_AUTO_CONTEXT_DEFINITIONS}
    -P ${CMAKE_SOURCE_DIR}/cmake/cxx-auto-cmake/command/EmitContext.cmake
  VERBATIM
  USES_TERMINAL)
