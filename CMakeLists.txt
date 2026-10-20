cmake_minimum_required(VERSION 3.20)
project(acadrust_ffi_examples VERSION 0.1.0 LANGUAGES C CXX)

set(ACADRUST_LIBRARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/release" CACHE PATH
    "Directory containing the Cargo-built shared library")
get_filename_component(ACADRUST_LIBRARY_DIR "${ACADRUST_LIBRARY_DIR}" ABSOLUTE)

add_library(acadrust_ffi SHARED IMPORTED GLOBAL)
set_target_properties(acadrust_ffi PROPERTIES
    INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_SOURCE_DIR}/include")
if(WIN32)
    set_target_properties(acadrust_ffi PROPERTIES
        IMPORTED_IMPLIB "${ACADRUST_LIBRARY_DIR}/acadrust_ffi.dll.lib"
        IMPORTED_LOCATION "${ACADRUST_LIBRARY_DIR}/acadrust_ffi.dll")
elseif(APPLE)
    set_target_properties(acadrust_ffi PROPERTIES
        IMPORTED_LOCATION "${ACADRUST_LIBRARY_DIR}/libacadrust_ffi.dylib")
else()
    set_target_properties(acadrust_ffi PROPERTIES
        IMPORTED_LOCATION "${ACADRUST_LIBRARY_DIR}/libacadrust_ffi.so")
endif()

function(acadrust_example target source)
    add_executable(${target} ${source})
    target_link_libraries(${target} PRIVATE acadrust_ffi)
    set_target_properties(${target} PROPERTIES C_STANDARD 99 C_STANDARD_REQUIRED YES)
    if(MSVC)
        target_compile_options(${target} PRIVATE /W4 /WX)
    else()
        target_compile_options(${target} PRIVATE -Wall -Wextra -Werror -pedantic)
    endif()
    if(WIN32)
        add_custom_command(TARGET ${target} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "$<TARGET_FILE:acadrust_ffi>" "$<TARGET_FILE_DIR:${target}>")
    endif()
endfunction()

acadrust_example(roundtrip examples/roundtrip.c)
include(CTest)
if(BUILD_TESTING)
    acadrust_example(c_smoke tests/c_smoke.c)
    acadrust_example(c_native tests/c_native.c)
    acadrust_example(c_builder tests/c_builder.c)
    add_test(NAME c_builder_api COMMAND c_builder)
    add_executable(cpp_header_check tests/cpp_header_check.cpp)
    target_link_libraries(cpp_header_check PRIVATE acadrust_ffi)
    set_target_properties(cpp_header_check PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED YES)
    if(WIN32)
        add_custom_command(TARGET cpp_header_check POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "$<TARGET_FILE:acadrust_ffi>" "$<TARGET_FILE_DIR:cpp_header_check>")
    endif()
    add_test(NAME c_abi COMMAND c_smoke)
    add_test(NAME c_native_abi COMMAND c_native)
    add_test(NAME cpp_abi COMMAND cpp_header_check)
    acadrust_example(cpp_builder tests/cpp_builder.cpp)
    set_target_properties(cpp_builder PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED YES)
    add_test(NAME cpp_builder_api COMMAND cpp_builder)
    add_test(NAME file_roundtrip COMMAND roundtrip "${CMAKE_CURRENT_BINARY_DIR}/roundtrip.dxf")
endif()
