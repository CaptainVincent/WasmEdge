# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2019-2024 Second State INC

wasmedge_add_executable(wasiNNTests
  wasi_nn.cpp
)

add_dependencies(wasiNNTests
  wasmedgePluginWasiNN
)

target_include_directories(wasiNNTests
  PUBLIC
  $<TARGET_PROPERTY:wasmedgePlugin,INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:wasmedgePluginWasiNN,INCLUDE_DIRECTORIES>
)

target_link_libraries(wasiNNTests
  PRIVATE
  ${GTEST_BOTH_LIBRARIES}
)

# Link to the WasmEdge library
if(WASMEDGE_LINK_PLUGINS_STATIC)
  target_link_libraries(wasiNNTests
    PRIVATE
    wasmedgeCAPI
  )
else()
  target_link_libraries(wasiNNTests
    PRIVATE
    wasmedge_shared
  )
endif()

add_test(wasiNNTests wasiNNTests)
