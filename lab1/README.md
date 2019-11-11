# Lab 1

## Non-linear equations with the single argument.


```
5 + (x ** 7) * sin(x) = (x ** 13) * cos(x) * sqrt(PI - cos(x ** 3))

5 + (x ** 7) * sin(x) - (x ** 13) * cos(x) * sqrt(PI - cos(x ** 3)) = 0

5 + (x * x ** 6) * (sin(x) - (x ** 6) * cos(x) * sqrt(PI - cos(x ** 3))) = 0

let t = x ** 3

f1 = 5 + (x * t * t) * (sin(x) - t * t * cos(x) * sqrt(PI - cos(t)))
``` 


`x` resides in `[-2.0, 10.0]`


Сhord and tangent method.

```
x ** 2 + PI * log_10(13 * PI) = 5 * PI * sin(x) + 2 * x
x ** 2 + PI * log_10(13 * PI) - 5 * PI * sin(x) - 2 * x
f = x * (x - 2) + PI * (log_10(13 * PI) - 5 * sin(x))
```

`x` resides in `(-inf, +inf)`

Chord method.
Fixed-point iteration method.


Fixed-point iteration:

