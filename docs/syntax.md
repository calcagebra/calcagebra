# Ast Nodes
## Assignment 

To declare a variable:

```rust
let name: type = expr
```

Here:
* `name` is an [`ident`](#identifier).
* `type` if provided is one of [`types`](/docs/types.md), if not provided will be inferred from the `expr`.
* `expr` is an [`expression`](#expression).

> Note: Using the wrong type will throw an error instead of implicit conversation.

## Function Declaration

To declare a function:

```rust
fn name(x: type, y: type, z: type ... ): type = expr
```

Here:
* `name, x, y, z ... ` are [`idents`](#identifier).
* `type` if provided is one of [`types`](/docs/types.md), if not provided will be [`ℝ`](/docs/types.md#real-ℝ).
* `expr` is an [`expression`](#expression).

## Function Call

To call a function:

```rust
name(x, y, z ... )
```

Here:
* `name, x, y, z ... ` are [`idents`](#identifier).

# Expression

## Abs

```rust
|expr|
```

Internally calls [`std.math.abs`](/docs/std.md#abs).

## Binary

```rust
expr operator expr
```

Here:
* `operator` is one of [`operators`](/docs/operators.md).
* `expr` is an [`expression`](#Expression).

## Branched

```rust
if expr then expr else expr end
```

Here: 
* `expr` is an [`expression`](#Expression).

## Identifier

```rust
ident
```

Identifiers are of non-zero length, denoted by alphabets the English language.

## Float

```rust
3.1415926535897931
```

Float numbers ([f64](https://doc.rust-lang.org/std/primitive.f64.html)).

## Matrix

```rust
[a_1, a_2, a_3; b_1, b_2, b_3; c_1, c_2, c_3 ... ]
```

Here:
* `a_1, a_2, a_3, b_1, b_2, b_3, c_1, c_2, c_3` are all [`expressions`](#Expression).

## Function Call

[Ref](#Function-Call).