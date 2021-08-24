# Code

Programs are composed of _functions_, _expressions_ and _statements_. In short:

- _functions_ are common operations and divide code into manageable chunks
- _expressions_ are pieces of code that have a _value_ and can be assigned to a
  variable or returned from a function
- _statements_ are pieces of code that just do some stuff and have no value

## Types

Values in a program have a _type_, which is just a way to tell the compiler or
interpreter how this data is intended to be used. Commonly encountered types
are:

- **integers**, signed (can be minus) or unsigned (only positive)
- **floating-point** numbers, which are approximations of real numbers
- **characters**, which represents pieces of text
- **booleans**, which represent something that is either _true_ or _false_

There are some differences between variations of some types, like the size of
their representation in bits. For example _integers_ can be signed or unsigned
and have different size.

Types have a little different names in different languages:

| **Language** | **Integer**      | **Real**          | **Character** |
| ------------ | ---------------- | ----------------- | ------------- |
| Rust         | `u32`, `i32`...  | `f32`, `f64`      | `char`        |
| Python       | `int`            | `float`           | `string`      |
| Haskell      | `Int`, `Integer` | `Float`, `Double` | `Char`        |

Some names are more intuitive, some not as much. For example, `u32` means a
32-bit unsigned integer. `f64` is a 64-bit floating-point number, often called
_double-precision_.

## Comments

You will encounter _comments_ in code that are just notes taken by the
programmer. They're commonly placed after `//` in **Rust**, `#` in **Python**,
or `--` in **Haskell**.

```rs
// just some text lol
```

## Block

A _code block_ is just a bunch of code grouped together. In many languages a
block is denoted using `{}` brackets, like in **Rust**:

```rs
{
    // code would be here
}

fn function() {
    // functions have their blocks
}
```

Some languages use indentation for their blocks, like **Python**:

```py
def function():
    # this here is a code block because it's a lil right

def function2():
    # this is another block of code
```

Various programming structures have their blocks, like _functions_,
_conditions_, _loops_.

## Variables

A _variable_ is just something to store a value into, same as assigning a number
to a letter in math:

```rs
fn main() {
    let a = 13;
    let b = 37;

    println!("a + b = {}", a + b);
}
```

## Operators

Like math, programming languages have different _operators_ that operate on
various types. The most basic operators are math ones, like `+` and `-`:

```rs
fn main() {
    let a = 5;
    let b = 2;

    let sum = a + b; // sum = 7
}
```

Commonly encountered operators, example for _signed integers_:

| **Operator**     | **Symbol** | **Example** | **Evaluation** |
| ---------------- | ---------- | ----------- | -------------- |
| plus             | `+`        | `3 + 5`     | `8`            |
| minus            | `-`        | `3 - 5`     | `-2`           |
| multiplication   | `*`        | `3 * 5`     | `15`           |
| division         | `/`        | `10 / 5`    | `2`            |
| modulo           | `%`        | `8 / 5`     | `3`            |
| equals           | `==`       | `3 == 5`    | `false`        |
| not equals       | `!=`       | `3 != 5`    | `true`         |
| greater          | `>`        | `3 > 5`     | `false`        |
| lower            | `<`        | `3 < 5`     | `true`         |
| greater or equal | `>=`       | `3 >= 5`    | `false`        |
| lower or equal   | `<=`       | `3 <= 5`    | `true`         |

There's some nuance on how some operators work for some data types in different
languages, like throwing away the fractional part of the result when dividing
whole numbers. No need to go into detail, just keep this fact in mind.

### Boolean algebra

Some more computers specific operators are _and_, _or_ and _not_, which
originate from _boolean algebra_. It is just a bunch of rules how to evaluate
expressions consisting of `true` and `false` values.

Note that `false` is commonly written as `0`, and `true` as `1`.

| **A** | **Operator** | **B** | **Result** |
| ----- | ------------ | ----- | ---------- |
| 0     | or           | 0     | 0          |
| 0     | or           | 1     | 1          |
| 1     | or           | 0     | 1          |
| 1     | or           | 1     | 1          |
| 0     | and          | 0     | 0          |
| 0     | and          | 1     | 0          |
| 1     | and          | 0     | 0          |
| 1     | and          | 1     | 1          |

The _not_ operator simply turns `true` to `false`, and vice versa.

These operators are often written using special symbols:

| **Language** | **And** | **Or** | **Not** |
| ------------ | ------- | ------ | ------- |
| Rust         | `&&`    | `\|\|` | `!`     |
| Python       | `and`   | `or`   | `not`   |
| Haskell      | `&&`    | `\|\|` | `!`     |

### Precedence

Operators have different _precedence rules_ by which are they evaluated. In
general, it goes like this:

- `/`, `*` and `%` first
- then `+` and `-`
- then comparison operators like `==`, `>`...
- then the `&&` operator
- finally, the `||` operator

So an expression `2+3 == 5 && 9/3+1 == 4` would get evaluated roughly as
follows:

- first, the `/` operator: `2+3 == 5 && 3+1 == 4`
- then the `+` operators: `5 == 5 && 4 == 4`
- then the `==` operators: `true && true`
- last, the `&&` operator: `true`

To bypass precedence rules, wrap expressions to be evaluated first in `()`, like
in math:

- `5 * 3 + 2` evaluates to `17`
- `5 * (3 + 2)` evaluates to `25`

### Laziness

Most languages have _lazy boolean expressions_, meaning they only evaluate a
part of the expression if the result is not already known. For example:

- if the first part of an `&&` expression is `false`, the result is _always_
  `false`
- if the first part of an `||` expression is `true`, the result is _always_
  `true`

In these cases, the 2nd part of the expression would _not_ get evaluated, since
it would not change the result.

To apply this on the example above, the expression `2+3 == 5 && 9/3+1 == 4`
would be evaluated as follows:

- first, the `+` on the left side: `5 == 5 && 9/3+1 == 4`
- then, the `==` on the left: `true && 9/3+1 == 4`

Since the left side of an `&&` expression is `true`, also the other part needs
to be evaluated to know the result:

- continuing, the `/` operator: `true && 3+1 == 4`
- then, the `+` operator: `true && 4 == 4`
- then, the `==` operator: `true && true`
- finally, the `&&` operator: `true`

The same principle would apply if the left side of an `||` expression would be
`false` — the right side's value needs to be known to evaluate the boolean
expression.

## Functions

Like in math, a _function_ is a chunk of code that performs an operation on its
_inputs_ and produces an _output_. An example `add` function in **Rust**:

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

In this function, `a + b` is an _expression_ returned from the `add` function.
It has a _value_ of the number produced by adding inputs `a` and `b`.

The same function would be written as `add(a, b) = a + b` in math. It is very
similar to what the same function would look like in **Haskell**:

```hs
add :: Int -> Int -> Int
add a b = a + b
```

Functions are _expressions_ that are _evaluated_ with specific inputs to produce
an output:

```rs
add(4, 20); // evalates to 24
```

### Side effects

Unlike math, most programming languages allow performing **side effects** in
functions in addition to producing values.

The `add` function can be modified to _print out_ its arguments:

```rs
fn add(a: i32, b: i32) -> i32 {
    println!("got a = {}, b = {}", a, b);
    a + b
}
```

Here, `println!(...);` is a _statement_. It has no value, it just _does stuff_.
It is a **side effect** — something that communicates with the outside world
from inside of a function.

> Functions with no side effects are called **pure functions**. Most functions
> should be written as such for easy reusability and testability.

### Procedures

Procedures are functions that _produce no value_ and just perform side effects:

```rs
fn trashtalk(a: &str) {
    println!("Yo mama is {}", a);
}
```

Procedure calls are _statements_, since they produce no value.

### Main

Most languages have so-called **main** function, which is a special function
that gets run first when the program is run:

```rs
fn main() {
    println!("Program start!");
}
```

## Exercises

- [ ] In example 01, write a comment that says `lmao this is silly`
- [ ] Evaluate `8/2 >= 3+1 && 5 < 2*(3+1) || 9/3 >= 6%4` step-by-step
- [ ] In example 02, create a function `sub` that takes two `i32` integers and
      returns their difference, then uncomment the call in `main` and run the
      program
- [ ] In example 03, change the function `div` to take and produce `f32` instead
      of `i32` and adjust the `main` function, then see how the result changes
