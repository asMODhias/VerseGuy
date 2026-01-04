cmake_minimum_required(VERSION 3.25)
project(VerseguY.Launcher LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(VerseguY.launcher src/main.cpp)

# Place launcher binary in the shared bin directory
set_target_properties(VerseguY.launcher PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

if (WIN32)
    # Ensure the launcher links to default system libs
    target_compile_definitions(VerseguY.launcher PRIVATE -DWIN32_LEAN_AND_MEAN)
endif()

# Add a basic launcher smoke test (headless)
add_test(NAME launcher_smoke COMMAND $<TARGET_FILE:VerseguY.launcher> --no-gui)
