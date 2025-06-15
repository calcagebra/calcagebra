# Types

Calcagebra has several types which are listed and explained here.

## Int [Z]

Variables with this type can store a minimum value of `-2,147,483,648` and a maximum value of `2,147,483,647` **(inclusive)**.

To declare a variable as an Integer, we can use one of the following methods:

```rs
let a = 5

let a: Z = 5

let a: int = 5

let a: Integer = 5
```

> Note: Using the wrong type will throw an error instead of implicit conversation.

## Real [R]

Variables with this type can store approximately, a minimum value of `-3.40282347e+38` and a maximum value of `3.40282347e+38` **(inclusive)**.

To declare a variable as a Real number (or a float), we can use one of the following methods:

```rs
let a = 5.0

let a: R = 5.0

let a: float = 5.0
```

> Note: Using the wrong type will throw an error instead of implicit conversation.

## Real [R]

Variables with this type can store approximately, a minimum value of `-3.40282347e+38` and a maximum value of `3.40282347e+38` **(inclusive)**.

To declare a variable as a Real number (or a float), we can use one of the following methods:

```rs
let a = 5.0

let a: R = 5.0

let a: float = 5.0
```

> Note: Using the wrong type will throw an error instead of implicit conversation.

## Complex Numbers [C]

Complex numbers are defined as `a + ib` where a and b can be R or Z, Although if Z is provided then it is immediately converted to R. They do not have implementations for `<, >, <=, >=`. To define a variable with a complex number as a value:

```rs
let a = 3 + 4i
```

Some specific operations such as `|a|` do not give output in the same manner as for R or Z. In the case of `|a|` the Modulus of Complex Number is returned instead of the usual conversion to non-negative values.