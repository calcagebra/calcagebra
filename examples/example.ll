; ModuleID = 'example'
source_filename = "example"

@print_format = global [5 x i8] c"%lf\0A\00"
@read_format = global [4 x i8] c"%lf\00"
@S = global [4 x double] [double 1.000000e+00, double 2.000000e+00, double 3.000000e+00, double 4.000000e+00]

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
  store [4 x double] [double 1.000000e+00, double 2.000000e+00, double 3.000000e+00, double 4.000000e+00], ptr @S, align 8
  %S = load [4 x double], ptr @S, align 8
  %counter = alloca i64, align 8
  store i64 0, ptr %counter, align 4
  br label %begin_print_loop
  %print_call2 = call i32 (ptr, ...) @printf(ptr @print_format, double 5.000000e+00)
  ret i32 0

begin_print_loop:                                 ; preds = %begin_print_loop, %entry
  %counter1 = load i64, ptr %counter, align 4
  %element = getelementptr double, ptr @S, i64 %counter1
  %value = load double, ptr %element, align 8
  %print_call = call i32 (ptr, ...) @printf(ptr @print_format, double %value)
  %new_counter = add i64 %counter1, 1
  store i64 %new_counter, ptr %counter, align 4
  %eq = icmp eq i64 %new_counter, 4
  br i1 %eq, label %end_print_loop, label %begin_print_loop

end_print_loop:                                   ; preds = %begin_print_loop
  ret i32 0
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
