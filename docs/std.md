# Standard Library 

## IO

### print

Print numbers to stdout.

### read

Read and return a [Real number](/docs/types.md#real-ℝ) from stdin.

## Math

### abs

#### Integers, Reals

Returns non-negative value of the number without regard to its sign.

#### Complex Numbers

Returns the modulus $\sqrt{\text{real}^2 + \text{imaginary}^2}$.

#### Matrix

Returns the [determinant of the matrix](https://en.m.wikipedia.org/wiki/Determinant).

### round

#### Reals

Rounds half to even and returns an Integer.

### ceil

#### Reals

Returns the smallest integer more than or equal to the number.

### floor

#### Reals

Returns the greatest integer less than or equal to the number.

### ln

#### Integers, Reals

Returns the natural log or log to the base $e$ of a number.

### log10

#### Integers, Reals

Returns the log to the base $10$ of a number.

### log

#### Integers, Reals

Returns the log of first number to the base of the second number.

### sin

#### Integers, Reals

Returns sin of number where number is in [radians](https://en.m.wikipedia.org/wiki/Radian).

### cos

#### Integers, Reals

Returns cos of number where number is in [radians](https://en.m.wikipedia.org/wiki/Radian).

### tan

#### Integers, Reals

Returns tan of number where number is in [radians](https://en.m.wikipedia.org/wiki/Radian).

### sqrt

#### Integers, Reals

Returns $\sqrt{\text{number}}$.

#### Complex Numbers

Returns [principal](https://en.m.wikipedia.org/wiki/Square_root#Principal_square_root_of_a_complex_number) $a + b\mathrm{i}$ such that $(a+b\mathrm{i})^2 = \text{number}$.

### cbrt

#### Integers, Reals

Returns $\sqrt[3]{\text{number}}$.

#### Complex Numbers

Returns [principal](https://en.m.wikipedia.org/wiki/Cube_root#Complex_numbers) $a + b\mathrm{i}$ such that $(a+b\mathrm{i})^3 = \text{number}$.

### nrt

#### Integers, Reals

Returns $\sqrt[\text{second number}]{\text{first number}}$.

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

That is, it should only take a single argument of type [Real](/docs/types.md#real-ℝ) and the return type must be [Real](/docs/types.md#real-ℝ) or [Integer](/docs/types.md#int-ℤ).

Then the graph function can be called as:

```rust
graph(name)
```

## Operators

[Operators avaliable](/docs/operators.md) and [types supporting operations](/docs/types.md) are documented in their respective files.

## Types

### real

#### Integers

Converts an [Integer](/docs/types.md#int-ℤ) to [Real](/docs/types.md#real-ℝ).

### int

#### Reals

Converts a [Real](/docs/types.md#real-ℝ) to [Integer](/docs/types.md#int-ℤ) by rounding towards zero.
