cmake_minimum_required(VERSION 3.20)
project(uds-minimal-c C)
add_executable(uds_minimal main.c)
target_link_libraries(uds_minimal uds_c)
