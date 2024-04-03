# Calcagebra

Programming language for evaluating mathematical expressions.

## Installation

Download the binary for your OS from the [releases page.](https://github.com/megatank58/calcagebra/releases/latest)

## Commands

* `calcagebra`: Open the REPL, use CTRL+C/CTRL+D to exit the repl.
* `calcagebra INPUT`: Run the contents of the file, if a directory is provided or file is not present then an error is thrown.
* `calcagebra version`: Print the verson of [calcagebra](https://github.com/megatank58/calcabegra).

## Syntax

### Expression

An expression can be one of the following types:

* Binary Expression: `expr (+|-|*|/|^) expr`
* Identifier: `[a-zA-Z.$]+`
* Number: `[0-9.]+`
* FunctionCall: `identifier exprs`

Note: Multiple exprs should always be seperated by a `,`. `()` should be used for nesting three or more function calls, `print cube square 5` -> `print cube(square 5)`

### Assignment

Assignments follow the `name = value` structure, variables can be reassigned with the same. `name` is an identifier and `value` is an `expr`.

### Function Declaration

Functions are declared by the `name params = code` structure where `name` is an identifier, `params` are `identifiers` seperated by whitespace and `code` is an `expr`.

Any function with prefix $ will have a graph generated after the program evaluates, the function will however only have access to std values and methods during evaluation.

### Function Calls

Functions are called by the `name args` structure, `name` is an identifier and `args` are `identifiers` seperated by `,`.

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
a = read
print a
# Enter number: 5
# 5
```

### Log
Returns natural log of number.
```hs
print log(2)
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