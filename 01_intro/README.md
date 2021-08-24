# Intro

Taken directly from [Wikipedia](https://en.wikipedia.org/wiki/Computer_program):

> A computer program is a collection of instructions that can be executed by a
> computer to perform a specific task.

In other words, you write a bunch of code in a file, and you either _compile_ it
or _interpret_ it. Either way, it gets transformed into a set of instructions
for the processor and executed.

## Terminal

A _terminal_ is a low-level way how to communicate with the OS. Terminals are
like graphical interfaces you're familiar with, but _much_ more powerful, and
less intuitive. 🤷‍♀️

CLI (command line interface) programs that are run via a terminal are run
relative to the current folder they're in. Their common structure is the
following:

```
> program-name [arguments] [--option value]
```

Everything is optional, only `program-name` is mandatory. You will encounter
arguments and options in various forms.

Common operations will be covered later, for now just get comfortable with
reading and running CLI programs in the code examples.

## Compiler

A _compiler_ takes nice, readable code and transforms it into efficient code for
your processor to run.

For example, compiling _example 01_ produces a `main` executable, which can be
run:

```bash
> rustc 01/main.rs
> ./main
Yo
```

In addition to translating code to machine language, compilers also check the
code for potential errors and perform various optimizations so that the code
runs as fast as possible! 🔥

## Interpreter

An _interpreter_ transforms the code to machine code during the execution.
Running interpreted scripts is done via the language engine.

Running _example 02_ is done via the `python` engine:

```bash
> python 02/main.py
Yo
```

Interpreted programs are often a lot slower than compiled ones and offer less
code quality analysis, but are much quicker to create and test.

## Exercises

- [ ] Compile and run example 01
- [ ] Run example 02
