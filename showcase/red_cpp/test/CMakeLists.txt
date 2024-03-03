# test/CMakeLists.txt
enable_testing()

add_executable(test_main test_main.cpp)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
FetchContent_MakeAvailable(googletest)
target_link_libraries(test_main gtest gmock GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(test_main)