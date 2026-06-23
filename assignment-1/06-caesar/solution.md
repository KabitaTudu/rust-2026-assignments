# Solution notes: Caesar cipher

## Approach

ALPHABET constant is used to determine the alphabet size instead of hardcoding 26. The shift value is normalized to the range [0, 25] using modulo. This handles both negative shifts (like -1 becoming 25) and large shifts (like 27 becoming 1). For each character, the code checks if it is a letter, finds its position in the alphabet, applies the normalized shift, and wraps around using modulo.

## Edge cases handled

- Negative shifts: Normalized by adding the alphabet length before applying modulo.
- Shifts larger than 26: Reduced using modulo to an equivalent shift in [0, 25].
- Empty strings: Returned an empty string naturally through the map/collect pipeline.
- Non-letter characters: Passed through unchanged (digits, punctuation, whitespace).
- Case preservation: Uppercase letters stayed uppercase, lowercase stayed lowercase.

## Anything special

The double modulo operation ((shift % len) + len) % len ensures correct handling of negative numbers, where % can return negative values. Using ALPHABET.len() instead of hardcoding 26 makes the code more maintainable and satisfies the constraint.
