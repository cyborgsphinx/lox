cmake_minimum_required(VERSION 3.10)
project(clox C)

set(CMAKE_C_STANDARD 11)

add_executable(clox
    main.c
    chunk.c
    debug.c
    memory.c
    value.c
    vm.c
    )
