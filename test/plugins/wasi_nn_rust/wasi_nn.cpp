// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2019-2024 Second State INC

#include "common/defines.h"
#include "runtime/instance/module.h"

#include "plugin/plugin.h"

#include <gtest/gtest.h>

namespace {

WasmEdge::Runtime::Instance::ModuleInstance *createModule() {
  using namespace std::literals::string_view_literals;
  WasmEdge::Plugin::Plugin::load(
      std::filesystem::u8path("../../../plugins/wasi_nn_rust/" WASMEDGE_LIB_PREFIX
                              "wasmedgePluginWasiNN" WASMEDGE_LIB_EXTENSION));
  if (const auto *Plugin = WasmEdge::Plugin::Plugin::find("wasi_nn"sv)) {
    WasmEdge::PO::ArgumentParser Parser;
    Plugin->registerOptions(Parser);
    if (const auto *Module = Plugin->findModule("wasi_nn"sv)) {
      return Module->create().release();
    }
  }
  return nullptr;
}

// void fillMemContent(WasmEdge::Runtime::Instance::MemoryInstance &MemInst,
//                     uint32_t Offset, uint32_t Cnt, uint8_t C = 0) noexcept {
//   std::fill_n(MemInst.getPointer<uint8_t *>(Offset), Cnt, C);
// }

// void fillMemContent(WasmEdge::Runtime::Instance::MemoryInstance &MemInst,
//                     uint32_t Offset, std::string_view Str) noexcept {
//   char *Buf = MemInst.getPointer<char *>(Offset);
//   std::copy_n(Str.data(), Str.length(), Buf);
// }

} // namespace

TEST(WasiNNTest, Burnackend) {
  auto *Mod = dynamic_cast<WasmEdge::Runtime::Instance::ModuleInstance *>(createModule());
  ASSERT_TRUE(Mod != nullptr);

  // Get the function "load".
  auto *FuncInst = Mod->findFuncExports("load_by_name");
  EXPECT_NE(FuncInst, nullptr);
  EXPECT_TRUE(FuncInst->isHostFunction());
//   auto &HostFuncLoad =
//       dynamic_cast<WasmEdge::Host::WasiNNLoad &>(FuncInst->getHostFunc());

}

GTEST_API_ int main(int argc, char **argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
