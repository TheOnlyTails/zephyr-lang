---
title: Literals
---

In Zephyr, everything is an expression, so it can evaluate to a value.

## Literals

Literals are a class of constructs that declare a specific value up-front, be it numbers, text,
or collections of other values.

### Numbers

There are two types of numbers: integers (`int`) and floating point numbers (`float`).
Integers may only be whole, signed numbers, and floats are 64-bit precision floating point numbers.

```ts
1 /* : int */ 
30920403934089209 /* : int */
1.234 /* : float */
1 as float // implicitly adds a trailing .0
3.14159 as int // truncates to 3
```

### Text

There are 2 ways to handle text in Zephyr; There's characters, which are a single 
[unicode codepoint](https://en.wikipedia.org/wiki/Code_point), and strings, which are lists of characters.
Character literals may only contain a single character, otherwise a compile-time error is thrown.

<!-- prettier-ignore -->
```ts
"Hello world!" /* : string */
'a' /* : char */
'abc' // [!code error]
```

Characters which are 2-codepoint long (mainly emojis and other complex characters) must be contained 
in strings.
<!-- prettier-ignore -->
```ts
'😆'
"❤️"
```

Strings may be freely converted into [lists](#lists) of characters, and back.

### Booleans

Booleans are a way to represent a value that can only be one of 2 things: true or false, yes or no, etc.

```ts
true /* : boolean */
false /* : boolean */
```

### Collections

These are literals which represent a collection or a mapping of other values.

#### Lists

Lists are an indexed, ordered collection of items, all with the same type.
To avoid having to give a manual type annotation to every list, the type of each item is
inferred, and the item type of the list is set to the most specific type that includes all of its items.

Zephyr infers the type of empty lists as `#[void]`, but can be manually annotated.

```ts
#[1, 2, 3] /* : #[int] */
// since ints are assignable to floats, the 2 is converted to a float
#[1.0, 2, 3.5] /* : #[float] */
// see section about union types
#[1, "hello", true] /* : #[int | string | boolean] */
#[#[0, 0], #[1, 2], #[2, 4], #[3, 6]] /* : #[#[int]] */
#[] /* : #[void] */
```

#### Maps

Maps are a way to represent a key-value relationship between values.
In a map, each key must be unique, and each key and value must be the same as the others.

Type inference works similarly to lists, wherein Zephyr infers both the keys and values as
the type that is their greatest common denominator.

Empty maps are inferred as `#{void: void}`.

```ts
#{
  1: 2,
  2: 4,
  3: 6,
} /* : #{int: int} */

#{
  'a': 1,
  'b': 2,
  'c': 3,
} /* : #{char: int} */

#{
  0: #[],
  1: #[1],
  2: #[1, 2],
  3: #[1, 2, 3],
} /* : #{int: #[int]} */

#{} /* : #{void: void} */ 
```

#### Tuples

Unlike lists, where each element must be of the same type, tuples represent a fixed-size collection of
a group of items, which may all be of different types.

The type of a tuple is determined by the type of its elements.

```ts
#() /* : #() */

#(1, "hello") /* : #(int, string) */
```

#### Objects

Objects are like maps in that they also represent a mapping of keys to values;
However, objects require each field to have its name known in advance, and the fields are immutable 
by default.

To create an object, use the `@{}` syntax:
```ts
@{}
```

Each field consists of a name (which may be any valid identifier), a colon, and its value.
The fields must be separated by commas.

```ts
@{
  a: 1,
  b: 3,
  hello: "Hello world!",
}
```

