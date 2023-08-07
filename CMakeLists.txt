# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-auto LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# ensure that the C++ compiler is `clang++`
set(CMAKE_CXX_COMPILER "clang++")

add_library(cxx-auto STATIC
  cxx/lib/cmake.cxx
)
target_include_directories(cxx-auto PUBLIC
  ../cxx-auto/..
  target/cxxbridge
)
target_compile_definitions(cxx-auto PUBLIC _LIBCPP_ENABLE_THREAD_SAFETY_ANNOTATIONS)
target_compile_options(cxx-auto PUBLIC
  -std=gnu++20
  # -stdlib=libc++
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
  -fno-rtti # needed to avoid "undefined reference to `typeinfo for [...]`" errors
)
