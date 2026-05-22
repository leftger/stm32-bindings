cmake_minimum_required(VERSION 3.10)
project(openthread)


set(SOURCES "../../sources")
set(STM32_WPAN "${SOURCES}/STM32CubeWB/Middlewares/ST/STM32_WPAN")
set(OPENTHREAD_STACK "${STM32_WPAN}/thread/openthread/stack")
set(INTERFACE_PATTERNS "${STM32_WPAN}/interface/patterns/ble_thread")

file(COPY "${STM32_WPAN}/thread/openthread/core"                            DESTINATION ".")
file(COPY "${STM32_WPAN}/stm32_wpan_common.h"                               DESTINATION "include")
file(COPY "${STM32_WPAN}/utilities/dbg_trace.h"                             DESTINATION "include")
file(COPY "${OPENTHREAD_STACK}/src/core/openthread-core-config.h"           DESTINATION "include")
file(COPY "${OPENTHREAD_STACK}/src/core/config"                             DESTINATION "include")
file(COPY "${OPENTHREAD_STACK}/include/openthread"                          DESTINATION "include")
file(COPY "${OPENTHREAD_STACK}/src/core/config"                             DESTINATION "include")
file(COPY "${OPENTHREAD_STACK}/third_party/mbedtls/repo/include/mbedtls"    DESTINATION "include")
file(COPY "${OPENTHREAD_STACK}/third_party/mbedtls/repo/include/psa"        DESTINATION "include")
file(COPY "${INTERFACE_PATTERNS}/tl/tl_thread_hci.h"                        DESTINATION "include")
file(COPY "${INTERFACE_PATTERNS}/tl/tl.h"                                   DESTINATION "include")
file(COPY "${INTERFACE_PATTERNS}/tl/mbox_def.h"                             DESTINATION "include")
file(COPY "${INTERFACE_PATTERNS}/shci/shci.h"                               DESTINATION "include")
file(COPY "${INTERFACE_PATTERNS}/shci/shci.h"                               DESTINATION "include")
file(COPY "${SOURCES}/STM32CubeWB/Drivers/CMSIS/Include"                    DESTINATION "include")


file(
    GLOB SRC_FILES CONFIGURE_DEPENDS
    "${CMAKE_CURRENT_BINARY_DIR}/core/openthread_api/*.c" 
)

add_library(openthread STATIC ${SRC_FILES}) 

target_include_directories(
    openthread 
    PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}"
    "${CMAKE_CURRENT_BINARY_DIR}"
    "${CMAKE_CURRENT_BINARY_DIR}/include"
    "${CMAKE_CURRENT_BINARY_DIR}/include/openthread"
    "${CMAKE_CURRENT_BINARY_DIR}/include/openthread/platform"
    "${CMAKE_CURRENT_BINARY_DIR}/include/Include"
    "${CMAKE_CURRENT_BINARY_DIR}/core/openthread_api"
    "${CMAKE_CURRENT_BINARY_DIR}/core/openthread_config"
)