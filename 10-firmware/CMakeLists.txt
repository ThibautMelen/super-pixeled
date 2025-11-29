cmake_minimum_required(VERSION 3.20)
project(super_pixeled_firmware VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

option(BUILD_TESTS "Build unit tests" OFF)

# Main library
add_library(renderer STATIC
    src/renderer.cpp
)
target_include_directories(renderer PUBLIC include)

# Main executable
add_executable(super_pixeled src/main.cpp)
target_link_libraries(super_pixeled PRIVATE renderer)

# Tests
if(BUILD_TESTS)
    enable_testing()
    find_package(GTest REQUIRED)

    add_executable(test_renderer tests/test_renderer.cpp)
    target_link_libraries(test_renderer PRIVATE renderer GTest::gtest_main)

    include(GoogleTest)
    gtest_discover_tests(test_renderer)
endif()
