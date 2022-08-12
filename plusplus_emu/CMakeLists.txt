# Set minmimum CMake version 
cmake_minimum_required(VERSION 3.0)

# Give project a name
project(plusplusEmu)

# Use C++ standard 20
set(CMAKE_CXX_STANDARD 20)

# Set .cpp and .h files to use
set(SOURCES src/main.cpp
            src/memory.cpp 
            src/cpu.cpp 
            src/registers.cpp 
            )

 # Build using above .cpp and .h files            
add_executable(plusplusEmu ${SOURCES})

