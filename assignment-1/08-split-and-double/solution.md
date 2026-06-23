# Solution notes: Split and double

## Approach

The function uses split_at_mut to divide the mutable vector into two non-overlapping slices at the given index. This method tells the borrow checker that the two slices are disjoint, allowing simultaneous mutable access. The code then iterates over each slice using .iter_mut() to double every element in place. Using .iter_mut() ensures the slices are borrowed for iteration rather than consumed, allowing the references to be returned at the end. Finally, the function returns both modified slices.

## Edge cases handled

- mid == 0: The left slice is empty, and the right slice contains all doubled elements.
- mid == xs.len(): The left slice contains all doubled elements, and the right slice is empty.
- Empty vector: Both slices are empty, so no operations occur.
- mid > xs.len(): split_at_mut panics automatically in this case.

## Anything special

Using .iter_mut() explicitly borrows the slices for the loop, preventing the references from being consumed (moved) by the for loop. This ensures left and right remain valid to return. split_at_mut guarantees memory safety and disjoint borrows.
The solution runs in O(n) time, visiting each element exactly once.
