; ModuleID = 'example'
source_filename = "example"

@print_format = global [5 x i8] c"%lf\0A\00"
@read_format = global [4 x i8] c"%lf\00"
@x = global double 5.000000e+00

declare i32 @printf(ptr, ...)

declare double @scanf(ptr, ...)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.pow.f64(double, double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double) #0

define double @f(double %0) {
entry:
  %y = alloca double, align 8
  store double %0, ptr %y, align 8
  %y1 = load double, ptr %y, align 8
  %x = load double, ptr @x, align 8
  %lt = fcmp olt double %y1, %x
  %retvalue = alloca double, align 8
  br i1 %lt, label %btrue, label %bfalse

btrue:                                            ; preds = %entry
  store double 0.000000e+00, ptr %retvalue, align 8
  br label %end

bfalse:                                           ; preds = %entry
  store double 1.000000e+00, ptr %retvalue, align 8
  br label %end

end:                                              ; preds = %bfalse, %btrue
  %ret = load double, ptr %retvalue, align 8
  ret double %ret
}

define i32 @main() {
entry:
  store double 5.000000e+00, ptr @x, align 8
  %f_call = call double @f(double 5.000000e+00)
  %print_call = call i32 (ptr, ...) @printf(ptr @print_format, double %f_call)
  ret i32 0
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
