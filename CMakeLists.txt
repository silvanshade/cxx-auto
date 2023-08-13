# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-auto LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# ensure that the C++ compiler is `clang++-16`
if(UNIX)
  find_program(CLANG_CXX_COMPILER clang++-16)
  if(NOT CLANG_CXX_COMPILER)
    if(APPLE)
      set(CLANG_CXX_COMPILER "/opt/homebrew/opt/llvm@16/bin/clang++")
    endif()
  endif()
  if(NOT CLANG_CXX_COMPILER)
    message(FATAL_ERROR "Could not find `clang++-16`")
  endif()
endif()

message (STATUS "Detected clang++: ${CLANG_CXX_COMPILER}")
set(CMAKE_CXX_COMPILER ${CLANG_CXX_COMPILER})

add_library(cxx-auto STATIC
  cxx/lib/cmake.cxx
)
target_include_directories(cxx-auto PUBLIC
  ../cxx-auto/..
  target/cxxbridge
)
target_compile_definitions(cxx-auto PUBLIC _LIBCPP_ENABLE_THREAD_SAFETY_ANNOTATIONS)
target_compile_options(cxx-auto PUBLIC
  -fno-rtti # needed to avoid "undefined reference to `typeinfo for [...]`" errors
  -std=gnu++20
  -Werror
  -Wall
  -Wextra
  -Wthread-safety
  -Wthread-safety-beta
  -pedantic
  -Wno-ambiguous-reversed-operator
  -Wno-deprecated-anon-enum-enum-conversion
  -Wno-deprecated-builtins
  -Wno-dollar-in-identifier-extension
  -Wno-nested-anon-types
  -Wno-unused-parameter
)
