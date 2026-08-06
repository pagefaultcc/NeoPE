cmake_minimum_required(VERSION 3.15)
project(examples CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(from_buffer from_buffer/main.cpp)
target_include_directories(from_buffer PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include)