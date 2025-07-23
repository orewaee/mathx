# mathx

## About

A deadly simple lexer/parser calculator for mathematical expressions based on the Shunting Yard algorithm.

The reverse Polish notation (RPN) is constructed from the input string and then calculated.

## Example

```bash
(2+5)*3
[Token { kind: Number(2), start: 1, end: 2 }, Token { kind: Number(5), start: 3, end: 4 }, Token { kind: Plus, start: 2, end: 2 }, Token { kind: Number(3), start: 6, end: 7 }, Token { kind: Mul, start: 5, end: 5 }]
result: 21
```
