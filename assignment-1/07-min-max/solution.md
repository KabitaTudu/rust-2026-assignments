# Solution notes: `min_max`

## Approach

The function first checks if the slice is empty, returning None immediately if true. For non-empty slices, both min and max initialize to the first element. A single for loop then iterates over every element, updating min whenever a smaller value appears and updating max whenever a larger value appears. The function returns Some((min, max)) after the loop completes.

## Edge cases handled

- Empty slice: Returns None via an early check using is_empty().
- Single element: Initializes min and max to the same value, resulting in Some((x, x)).
- All equal values: Comparisons fail to update either variable, correctly returning Some((x, x)).
- Negative numbers: Comparisons work correctly with signed integers.
- i32::MIN and i32::MAX: Handled correctly since initialization uses actual slice values rather than arbitrary bounds.

## Anything special

The solution uses exactly one loop as required, avoiding any iterator adapters. Initializing with xs[0] instead of i32::MAX/i32::MIN avoids potential overflow issues and simplifies the logic. The pattern for &x in xs dereferences each element automatically, making comparisons cleaner (x < min instead of *x < min). This approach performs exactly n comparisons for both min and max in a single pass, which is optimal for this constraint.
