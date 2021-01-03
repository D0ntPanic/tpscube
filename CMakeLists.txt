cmake_minimum_required(VERSION 3.9 FATAL_ERROR)
set(CMAKE_OSX_ARCHITECTURES x86_64 CACHE STRING "Target architecture")

project(tpscube)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
find_package(Qt5 COMPONENTS Core Gui Widgets Bluetooth REQUIRED)

file(GLOB SOURCES lib/*.cpp lib/*.h ui/qt/*.cpp ui/qt/*.h ui/qt/*.qrc)

add_executable(tpscube ${SOURCES})

include_directories(lib include /usr/local/include)

set_target_properties(tpscube PROPERTIES
	CXX_STANDARD 17
	CXX_STANDARD_REQUIRED ON
	CXX_VISIBILITY_PRESET hidden
	VISIBILITY_INLINES_HIDDEN ON
	POSITION_INDEPENDENT_CODE ON
	RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

find_library(LIBLEVELDB NAMES leveldb)

target_link_libraries(tpscube PRIVATE Qt5::Core Qt5::Gui Qt5::Widgets Qt5::Bluetooth ${LIBLEVELDB})
