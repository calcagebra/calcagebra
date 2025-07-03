# Standard Library 

## IO

### print

Print numbers to stdout.

### read

Read is extremely powerful in calcagebra and provides you with all standard library functions, all user variables and user defined functions upto that loc to be used while inputting a value which can be a complex number or a matrix.

## Math

### abs

#### Complex Numbers

Returns the modulus $\sqrt{\text{real}^2 + \text{imaginary}^2}$.

#### Matrix

Returns the [determinant of the matrix](https://en.m.wikipedia.org/wiki/Determinant).

### round

#### Complex Number

Rounds half to even both the real and imaginary part.

### ceil

#### Complex Number

Rounds to the smallest integer more than or equal to the number for both the real and imaginary part

### floor

#### Complex Number

Rounds to the greatest integer less than or equal to the number for both the real and imaginary part

### ln

#### Complex Number

Returns the natural log or log to the base $e$ of the number.

### log10

#### Complex Number

Returns the log to the base $10$ of the number.

### log

#### Complex Number

Returns the log of first number to the base of the second number.

### sin

#### Complex Number

Returns sin of number where number is in [radians](https://en.m.wikipedia.org/wiki/Radian).

### cos

#### Complex Number

Returns cos of number where number is in [radians](https://en.m.wikipedia.org/wiki/Radian).

### tan

#### Complex Number

Returns tan of number where number is in [radians](https://en.m.wikipedia.org/wiki/Radian).

### sqrt

#### Complex Numbers

Returns [principal](https://en.m.wikipedia.org/wiki/Square_root#Principal_square_root_of_a_complex_number) $a + b\mathrm{i}$ such that $(a+b\mathrm{i})^2 = \text{number}$.

### nrt

#### Complex Numbers

Returns $a + b\mathrm{i}$ such that $(a+b\mathrm{i})^{\text{second number}} = \text{first number}$.

### determinant

#### Matrix

Returns the [determinant of the matrix](https://en.m.wikipedia.org/wiki/Determinant).

### transpose

#### Matrix

Returns the [transpose of the matrix](https://en.wikipedia.org/wiki/Transpose).

### adj

#### Matrix

Returns the [adjoint of the matrix](https://en.wikipedia.org/wiki/Adjugate_matrix).

### inverse

#### Matrix

Returns the [inverse of the matrix](https://en.wikipedia.org/wiki/Invertible_matrix).

### graph

The graph function takes the name of the function ([`ident`](/docs/syntax.md#identifier)) as its argument and writes the image of the graph to the file system in PNG format with the name `graph-output-{TIME_SINCE_UNIX_EPOCH}.png`.

The function must itself be of the format:

```rust
fn name(x: R): type = expr
```

That is, it should only take a single argument of type complex where imaginary part is 0.0 and the return type must be complex too.

Then the graph function can be called as:

```rust
graph(name)
```

## Operators

[Operators avaliable](/docs/operators.md) and [types supporting operations](/docs/types.md) are documented in their respective files.