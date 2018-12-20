cmake_minimum_required(VERSION 3.7)
project(rest-client)

SET(CMAKE_CXX_STANDARD 14)
SET(CMAKE_INCLUDE_CURRENT_DIR ON)

SET(SOURCE main.cpp)
add_executable(client ${SOURCE})
get_target_property(CLIENT_DIR client LOCATION)
target_link_libraries(gui ${CLIENT_DIR}/libclient.so)
#add_dependencies($dep_name client)
