# Calcagebra

Programming language for evaluating mathematical expressions.

```rust
let a = [1, 2, -3; 0, -5, 1]
let b = [3; 5; 6]
let c = 3 + 4i

print(69*420) // 28980
print(|c|) // 5
print(a*b)
// ┌     ┐
// │ -5  │
// │ -19 │
// └     ┘
```

## Installation

Download the binary for your OS from the [releases page.](https://github.com/calcagebra/calcagebra/releases/latest)

## Commands

* `calcagebra repl`: Open the REPL, use CTRL+C/CTRL+D to exit the repl.
* `calcagebra run INPUT`: Run the contents of the file, if a directory is provided or file is not present then an error is thrown.
* `calcagebra --version`: Print the verson.

## Syntax

### Expression

An expression is defined as one of the following types:

* Abs: `|Expr|`
* Binary Expression: `Expr operator Expr`
> Operator: An operator is one of the following tokens: +,-,*,/,%,==.!=,>,<,>=,<=
* Branched: `if Expr then Expr else Expr end`
* Identifier: english alphabets
* Number: 32 bit float
* Matrix: `[a, b, c; d, e, f; g, h, i]` ∀ a,b,c,d,e,f,g,h,i ∈ Expr
* Complex Number: a + bi ∀ a,b ∈ ℝ
* FunctionCall: `identifier(Exprs)`

#### Note: 
* `,` can be used to separate multiple exprs in matrix and function call arguments. 
* `*` operator is optional between `identifier` and `number`. For example: `6x^2 + 5x + 1`.

### Assignment

Assignments follow the `let name = value` structure, variables can be reassigned with the same. `name` is an identifier and `value` is an `Expr`.

### Function Declaration

Functions are declared by the `fn name(params) = code` structure where `name` is an identifier, `params` are `identifiers` seperated by whitespace and `code` is an `Expr`.

### Function Calls

Functions are called by the `name(args)` structure, `name` is an identifier and `args` are `Exprs` seperated by `,`.

### If statements

If statements begin with `if` followed by the statement, `then` the expression to value if statement is evaled to true, `else` the expression in case it is not, ended by an `end`.

## Constants

The constants in calcagebra are:

* i = sqrt(-1)
* pi or π = 3.1415927
* e = 2.7182817

## Standard Library

All standard library functions follow the function calling structure.

### Print
Prints numbers to stdout, numbers are always followed by a newline, returns 0.
```hs
print(cube(5), 7) 
# 125
# 7
```

### Read
Reads a number from stdin with the prompt `Enter number: ` and returns it.
```hs
a = read()
print(a)
# Enter number: 5
# 5
```

### Round, Floor, Ceil
Return the number rounded, floored, ceiled.
```hs
print(floor(e), round(e), ceil(e))
# 2
# 3
# 3
# 1
```

### ln, log10, log(a, b)
Returns log of number.
```hs
print(ln(2))
# 0.6931472
```

### Trignometric functions

Trignometric functions include `sin`, `cos` and `tan` which take a single number in radians as input and return the value.

```hs
print(sin(0), cos(0), tan(pi/4))
# 0
# 1
# 1
```

### Roots of numbers

The `sqrt`, `cbrt` return the square and cube root of the number while `nrt` returns the nth root of the number.

```hs
print(sqrt(4), cbrt(27), nrt(343,3))
# 2
# 3
# 7
```

### Graph
Generates the graph of a function and writes it to file `graph-output-TIME_SINCE_UNIX_EPOCH.png` where `TIME_SINCE_UNIX_EPOCH` is the time since unix epoch.

An attempt to print the file to the terminal is made if possible.
```hs
f(x) = nrt(x^2,2)
graph(f)
```