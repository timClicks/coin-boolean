# coin: a radiation-safe Boolean

A `Coin` is data type for representing Boolean values that is resistant to bit
flips. Prefer `Coin` to `bool` in safety-critical environments with long-lived
variables, such as global variables.

A standard `bool` is truth-biased, because `false` matches a single bit pattern
(all zeros). A single bit flip invalidates the value.  `Coin` counts the number
of bits to determine its truth value.  When 4 or more bits are 1, the value is
interpreted as `true`. `Coin` can tolerate 3 bit flips per byte before an
incorrect value is returned.
