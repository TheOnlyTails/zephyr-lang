---
title: Aura Syntax
icon: data-object
---

## Literals
### Numbers
There are two types of numbers: integers (`int`) and floating point numbers (`float`).
Integers may only be whole numbers, and floats are 64-bit precision floating point numbers.

```ts
let x /* : int */ = 1
let y /* : int */ = 30920403934089209
let z /* : float */ = 1.234
let w /* : float */ = 1 // implicitly adds a trailing .0
let a /* : int */ = 3.14159 // truncates to 3
```

### Text
Text can be stored as either a string of characters, or a single character (characters which are made up of 2 codepoints are considered 2 long).

```ts
let s /* : string */ = "Hello world!"
let c /* : char */ = 'a'
let s /* : char */ = '😆' // unicode support for single-codepoint characters
let heart /* : string */ = "❤️" // two-wide characters must be put in a string
```

### Booleans
Booleans are a way to represent a value which can only be one of 2 things: true or false, yes or no, etc.
```ts
let a /* : boolean */ = true
let b /* : boolean */ = false
```

### Derived types
These are types which are used to represent relationships between values of different types:

#### Lists
Lists are an indexed, ordered collection of items, all with the same type.
To avoid having to give a manual type annotation to every list, the type of each item is
inferred and the item type of the list is set to the most specific type which includes all of its items.

Empty lists are automatically inferred to be of type `#[void]`, but can be manually annotated.

```ts
let a /* : #[int] */ = #[1, 2, 3]
let b /* : #[float] */ = #[1.0, 2, 3.5] // since ints are assignable to floats, the 2 is converted to a float
let c /* : #[int | string | boolean] */ = #[1, "hello", true] // see section about union types
let d /* : #[#[int]] */ = #[#[0, 0], #[1, 2], #[2, 4], #[3, 6]] // nested lists work
let e /* : #[void] */ = #[]
```

#### Maps
Maps are a way to represent a key-value relationship between values.
In a map, each key must be unique, and each of the keys and values must be the same as other keys and values.

Type inference works similarly to lists, wherein both the keys and values are inferred to be 
the type which is the greatest common denominator of their values' types.

Empty maps are inferred as `#{void: void}`.

```ts
let a /* : #{int: int} */ = #{
  1: 2,
  2: 4,
  3: 6,
}

let b /* : #{char: int} */ = #{
  'a': 1,
  'b': 2,
  'c': 3,
}

let c /* : #{int: #[int]} */ = #{
  0: #[],
  1: #[1],
  2: #[1, 2],
  3: #[1, 2, 3],
}

let d /* : #{void: void} */ = #{}
```