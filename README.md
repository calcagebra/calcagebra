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
* Identifier: `[a-zA-Z]+`
* Number: `[0-9]+`
* FunctionCall: `identifier exprs`

Note: Multiple exprs should always be seperated by a `,`. `()` should be used for nesting three or more function calls, `print cube square 5` -> `print cube(square 5)`

### Assignment

Assignments follow the `name = value` structure, variables can be reassigned with the same. `name` is an identifier and `value` is an `expr`.

### Function Declaration

Functions are declared by the `name params = code` structure where `name` is an identifier, `params` are `identifiers` seperated by whitespace and `code` is an `expr`.

### Function Calls

Functions are called by the `name args` structure, `name` is an identifier and `args` are `identifiers` seperated by `,`.

## Standard Library

All standard library functions follow the function calling structure.

### Print

The print function prints numbers to stdout, numbers are always followed by a newline.
```hs
print(cube(5), 7) 
# 125
# 7
```
