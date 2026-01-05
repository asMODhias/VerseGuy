cmake_minimum_required(VERSION 3.25)
project(VerseGuy.Core VERSION 0.1.0 LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(VerseguY.Core SHARED
    src/Core.cpp
    src/PluginLoader.cpp
    src/EventBus.cpp
)

target_include_directories(VerseguY.Core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
# Export symbols when building DLL on Windows
target_compile_definitions(VerseguY.Core PRIVATE VERSEGUY_CORE_EXPORTS)

# Ensure the DLL is placed next to test binaries (so tests can LoadLibrary without full path)
set_target_properties(VerseguY.Core PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# On Windows, set properties for DLL
if (WIN32)
    set_target_properties(VerseguY.Core PROPERTIES PREFIX "" SUFFIX ".dll")
endif()

# Enable CTest for test discovery and execution
enable_testing()

# Test loader executable (loads the Core DLL and calls Initialize)
add_executable(test_loader tests/test_loader.cpp)
if (WIN32)
    target_compile_definitions(test_loader PRIVATE -D_WIN32_WINNT=0x0A00)
endif()

target_link_libraries(test_loader PRIVATE VerseguY.Core)

# Ensure test_loader runs from same output dir as DLL
set_target_properties(test_loader PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
add_test(NAME test_loader COMMAND $<TARGET_FILE:test_loader>)

# Plugin verify test
add_executable(test_plugin_verify tests/test_plugin_verify.cpp)
if (WIN32)
    target_compile_definitions(test_plugin_verify PRIVATE -D_WIN32_WINNT=0x0A00)
endif()
set_target_properties(test_plugin_verify PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
add_test(NAME test_plugin_verify COMMAND $<TARGET_FILE:test_plugin_verify>)

target_link_libraries(test_plugin_verify PRIVATE VerseguY.Core)

# WASM sandbox smoke test (runs the Rust wasm-runner binary via cargo)
add_executable(test_wasm_runner tests/test_wasm_runner.cpp)
if (WIN32)
    target_compile_definitions(test_wasm_runner PRIVATE -D_WIN32_WINNT=0x0A00)
endif()

target_link_libraries(test_wasm_runner PRIVATE VerseguY.Core)
set_target_properties(test_wasm_runner PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
add_test(NAME wasm_sandbox_smoke COMMAND $<TARGET_FILE:test_wasm_runner>)

