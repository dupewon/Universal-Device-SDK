cmake_minimum_required(VERSION 3.20)
project(uds-c-bindings C)

set(CMAKE_C_STANDARD 11)

add_library(uds_c SHARED uds.c)
target_include_directories(uds_c PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(TARGETS uds_c LIBRARY DESTINATION lib ARCHIVE DESTINATION lib)
install(FILES uds.h DESTINATION include)
