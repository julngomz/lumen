#ifndef LUMEN_EIR_CONVERSION_BUILTIN_OP_CONVERSION
#define LUMEN_EIR_CONVERSION_BUILTIN_OP_CONVERSION

#include "lumen/compiler/Dialect/EIR/Conversion/EIRToLLVM/ConversionSupport.h"

namespace lumen {
namespace eir {
class IncrementReductionsOpConversion;
class IsTypeOpConversion;
class MallocOpConversion;
class PrintOpConversion;
class TraceCaptureOpConversion;
class TraceConstructOpConversion;

void populateBuiltinOpConversionPatterns(OwningRewritePatternList &patterns,
                                         MLIRContext *context,
                                         LLVMTypeConverter &converter,
                                         TargetInfo &targetInfo);
}  // namespace eir
}  // namespace lumen

#endif  // LUMEN_EIR_CONVERSION_BUILTIN_OP_CONVERSION
