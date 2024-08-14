; ModuleID = 'example'
source_filename = "example"

@print_format = global [5 x i8] c"%lf\0A\00"
@read_format = global [4 x i8] c"%lf\00"

declare i32 @printf(ptr, ...)

declare double @scanf(ptr, ...)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.pow.f64(double, double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.round.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.ceil.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.floor.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.log.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sin.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.cos.f64(double) #0

declare double @llvm.tan.f64(double)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double) #0

define i32 @main() {
entry:
  %ret = call double @llvm.pow.f64(double 5.000000e+00, double 2.000000e+00)
  %print_call = call i32 (ptr, ...) @printf(ptr @print_format, double %ret)
  %ret1 = call double @llvm.pow.f64(double 2.000000e+00, double 5.000000e+00)
  %print_call2 = call i32 (ptr, ...) @printf(ptr @print_format, double %ret1)
  ret i32 0
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
