; ModuleID = 'example'
source_filename = "example"

@print_format = global [4 x i8] c"%f\0A\00"
@read_format = global [3 x i8] c"%f\00"

declare i32 @printf(ptr, ...)

declare i32 @scanf(ptr, ...)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.pow.f64(double, double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double) #0

define double @f(double %0) {
entry:
  %x = alloca double, align 8
  store double %0, ptr %x, align 8
  %x1 = load double, ptr %x, align 8
  %eq = fcmp oeq double %x1, 1.000000e+00
  %x2 = load double, ptr %x, align 8
  %x3 = load double, ptr %x, align 8
  %sub = fsub double %x3, 1.000000e+00
  %f_call = call double @f(double %sub)
  %mul = fmul double %x2, %f_call
  %select = select i1 %eq, double 1.000000e+00, double %mul
  ret double %select
}

define i32 @main() {
entry:
  %f_call = call double @f(double 2.000000e+00)
  %print_call = call i32 (ptr, ...) @printf(ptr @print_format, double %f_call)
  ret i32 0
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
