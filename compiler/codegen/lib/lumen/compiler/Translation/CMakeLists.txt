lumen_cc_library(
  NAME
    Translation
  HDRS
    "ModuleBuilder.h"
    "ModuleBuilderSupport.h"
  SRCS
    "LLVMIR.cpp"
    "ModuleBuilder.cpp"
    "ModuleReader.cpp"
    "ModuleWriter.cpp"
  DEPS
    lumen::compiler::Dialect::EIR::IR
    lumen::compiler::Dialect::EIR::Transforms
    lumen::compiler::Target
    LLVMSupport
    LLVMTarget
    LLVMBitWriter
    MLIRIR
    MLIRSupport
    MLIRExecutionEngine
    MLIRStandardOps
    MLIRTargetLLVMIR
    MLIRLLVMIR
    MLIRStandardToLLVM
    MLIRParser
    MLIRPass
    MLIRTransforms
  ALWAYSLINK
  PUBLIC
)
