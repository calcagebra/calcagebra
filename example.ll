; ModuleID = 'example'
source_filename = "example"

@print_format = global [4 x i8] c"%f\0A\00"
@read_format = global [3 x i8] c"%f\00"

declare i32 @printf(ptr, ...)

declare i32 @scanf(ptr, ...)

define i32 @main() {
entry:
  %a = alloca float, align 4
  store float 5.000000e+00, ptr %a, align 4
  %a1 = load float, ptr %a, align 4
  %print_value = fpext float %a1 to double
  %print_call = call i32 (ptr, ...) @printf(ptr @print_format, double %print_value)
  ret i32 0
}
