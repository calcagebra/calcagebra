; ModuleID = 'example'
source_filename = "example"

@print_format = global [5 x i8] c"%lf\0A\00"
@read_format = global [4 x i8] c"%lf\00"

declare i32 @printf(ptr, ...)

declare double @scanf(ptr, ...)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.pow.f64(double, double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double) #0

define double @fib(double %0) {
entry:
  %x = alloca double, align 8
  store double %0, ptr %x, align 8
  %x1 = load double, ptr %x, align 8
  %lte = fcmp ole double %x1, 1.000000e+00
  %retvalue = alloca double, align 8
  br i1 %lte, label %btrue, label %bfalse

btrue:                                            ; preds = %entry
  %x2 = load double, ptr %x, align 8
  store double %x2, ptr %retvalue, align 8
  br label %end

bfalse:                                           ; preds = %entry
  %x3 = load double, ptr %x, align 8
  %sub = fsub double %x3, 1.000000e+00
  %fib_call = call double @fib(double %sub)
  %x4 = load double, ptr %x, align 8
  %sub5 = fsub double %x4, 2.000000e+00
  %fib_call6 = call double @fib(double %sub5)
  %add = fadd double %fib_call, %fib_call6
  store double %add, ptr %retvalue, align 8
  br label %end

end:                                              ; preds = %bfalse, %btrue
  %ret = load double, ptr %retvalue, align 8
  ret double %ret
}

define i32 @main() {
entry:
  %input = alloca double, align 8
  %read_result = alloca i64, align 8
  %read_call = call double (ptr, ...) @scanf(ptr @read_format, ptr %read_result)
  %read_result1 = load i64, ptr %read_result, align 4
  store i64 %read_result1, ptr %input, align 4
  %value = alloca double, align 8
  %input2 = load double, ptr %input, align 8
  %fib_call = call double @fib(double %input2)
  store double %fib_call, ptr %value, align 8
  %value3 = load double, ptr %value, align 8
  %print_call = call i32 (ptr, ...) @printf(ptr @print_format, double %value3)
  ret i32 0
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
