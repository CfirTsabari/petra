cmake_minimum_required(VERSION 3.10)

project(HelloWorld)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add executable
add_executable(main src/main.cpp)

# Formatting
include(cmake/ClangFormat.cmake)
add_clangformat(main)

# Testing
add_subdirectory(test)
