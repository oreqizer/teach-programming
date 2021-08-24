# Control flow

Chapter dedicated learning about how to control _which_ parts of code to
execute, and _how many times_.

## Conditions

Conditions let us choose which part of a code to execute based on an expression
that evaluates to `true` or `false`:

```rs
fn main() {
    let a = 5;

    if a > 10 {
        println!("big number!");
    } else if a < 0 {
        println!("minus number");
    } else {
        println!("small number");
    }
}
```

Almost all languages use the `if` and `else` keywords, possibly combined into an
`else if`.

## Exercises

- [ ] Modify the `a` variable in example 01 so you print every possible text
- [ ] Remove both `else` code blocks in example 01 so only big numbers are
      printed
