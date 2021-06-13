# Language Reference

## Variables
```
let x = 10
let y = {
	5 + 3
} // 8
```

## Functions
```
def double(x)
	x * 2
end

double 5 // 10
```

## Types
```
type fruit =
  | Apple
  | Pear
  | Strawberry
```

```
type point = {
  x,
  y
}
```

## Pattern matching
```
let fruit = Apple
match fruit {
 | Apple => TODO
 | Pear => TODO
 | Strawberry => TODO
}
```

The match statement is an expression.
```
let x = match fruit {
 | Apple => "Apple"
 | Pear => "Pear"
 | Strawberry => "Strawberry"
}
```
