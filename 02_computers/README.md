# Computers

Chapter dedicated to basics about computers needed for programming. 🧠

## Data

All computer instructions and data are serialized into _zeros_ and _ones_ using
different protocols. Here is some commonly encountered terminology:

- **bit** is the smallest data unit that is either `0` or `1`
- **byte** is a data unit composed of **8 bits**, so it can hold 256 different
  values
- **encoding** is a way to represent real data of different kind in _bits_ and
  _bytes_
- **memory** refers to a dedicated place in the **RAM** where the running
  program and its data reside
- **address** is a place in memory that contains some data

The length of a processor instruction or memory address is given the CPU
architecture of your device, nowadays usually 32 or 64 _bits_.

## Encodings

Some common numeric encodings are:

- _binary_, which is just a summed sequence of _bits_ with each bit having value
  of `b*2^n`, where `n` is the order and `b` is the value of 0 or 1
- _decimal_ encoding is literally what you write in daily life, with each number
  representing a value of `b*10^n` and the range of numbers is 0 to 9
- _hexadecimal_, which is a summed sequence of values `0` to `F` (letters being
  values 10 to 15), each having value of `b*16^n` similar to binary, except `b`
  can have a value from 0 to 15

Hexadecimals are often represented with prefix `0x`, so a hexadecimal 32-bit
value looks something like `0x0F31C8D5`, where each letter after `0x` represetns
_four bits_. This is commonly how memory addresses are represented.

> An easy hexadecimal to binary conversion is just to "unwrap" each hexadecimal
> character into four bits, so `0xF5` would be `1111 0101`.

Real numbers are represented in a weird and unintuitive way. All you need to
know is that they're just _approximations_, with 32-bit representations being
less precise, and 64-bit ones more precise.

### Memory

Memory contains a bunch of _bytes_, which are _addressable_. Programs keep track
at which address does it have data, and the size of the data.

The smallest unit that can be addressed is a _byte_, so address pointing to
`0x00000002` points to one byte, or 8-bits, further than `0x00000001`.

For example, if your program stores a 32-bit (4-byte) number at address
`0x00000010`, the number occupies space up to address `0x00000013`.

You will encounter special variables called _pointers_ or _references_, which
hold addresses that point to data in memory, instead of the data themselves. 🤯

### Text

Encodings that work with _text_ are a table where each set of bits represents
one character. The most common encodings today are _ASCII_ and _UTF-8_.

- _ASCII_ only contains 256 different characters, like the English alphabet and
  basic characters
- _UTF-8_ can represent the whole _Unicode_ set of characters, include the vast
  majority of existing alphabets, emojis, characters, and all kinds of stuff

Text is stored as a _sequence of characters_ in memory called _strings_. For
example, each _ASCII_ character takes up only 1 byte. So the word `hello` stored
at address `0x00000005` would take up space up to address `0x00000009`.

> There's some nuance with how UTF-8 encoding works, since its characters are
> encoded with various number of bytes. Just keep in mind that such cases exist,
> but let's keep the examples simple for now.

### Conversions

- a _bit_ is literally just `0` or `1`
- a _byte_ with value in _binary encoding_ 49 would be `0011 0001`
- the letter `A` in _ASCII encoding_ is `0100 0001`
- value 46 in _hexadecimal encoding_ would be `2E`, where `2` is `2*16^1 = 32`
  and `E` is `14*16^0 = 14`
- an arbitrary _memory address_ on a _32-bit_ computer looks something like
  `0000 0010 0110 1101 1011 1000 1101 0011`

## Exercises

- [ ] Convert 420 to _binary_ and _hexadecimal_
- [ ] Convert `11001` from _binary_
- [ ] Convert `14C` from _hexadecimal_
- [ ] Convert `0xE41D` to _binary_
- [ ] Convert `1011 1001 0011 1111` to _hexadecimal_
- [ ] Find a _UTF-8_ binary representation of the _Unicode_ character `š`
