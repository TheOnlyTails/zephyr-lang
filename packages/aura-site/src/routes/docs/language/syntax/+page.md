---
title: Aura Syntax
icon: data-object
---

## Literals
### Numbers
```ts
let x: int = 1
let y: = 30920403934089209
let z: float = 1.234
let w: float = 1 // implicitly adds a trailing .0
let a: int = 3.14159 // truncates to 3
```

### Text
```ts
let s: string = "Hello world!"
let c: char = 'a'
let s = '😆' // unicode support for single-codepoint characters
let heart = "❤️" // two-wide characters must be put in a string
```