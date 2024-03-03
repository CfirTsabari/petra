find_program(CLANG_FORMAT_EXE NAMES "clang-format" DOC "Path to clang-format executable")

if (CLANG_FORMAT_EXE)
  message(STATUS "clang-format found: ${CLANG_FORMAT_EXE}")
else ()
  message(FATAL_ERROR "clang-format not found.")
endif ()

function(add_clangformat target)
  add_custom_target(${target}-clangformat
    COMMAND ${CLANG_FORMAT_EXE} 
    -style=Google
    -i
    test/test_main.cpp
    include/defs.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Formatting ${target} using clang-format"
  )
  add_custom_target(${target}-clangformat-check
    COMMAND ${CLANG_FORMAT_EXE} 
    -style=Google
    --dry-run --Werror
    -i
    test/test_main.cpp
    include/defs.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Formatting ${target} using clang-format"
  )

endfunction()