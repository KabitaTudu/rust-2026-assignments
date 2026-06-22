# Solution notes: Censor vowels in place

## Approach

Iterate through every character in the input string using the chars() iterator. For each character, check if it matches any of the vowel character(both lowercase and uppercase). If a match is found, the character is replaced with *; otherwise, the original character is kept. These processed characters are collected into a new String, which is then assigned back to the mutable reference s using *s = censored.

## Edge cases handled

- Empty strings: The iterator produces no items, resulting in an empty string.
- No vowels: If the string contains no vowels, every character is kept, and the string remains unchanged. 
- All vowels: Every character is replaced, resulting in a string of asterisks.
- Mixed vowel cases: Both uppercase and lowercase vowels are detected and replaced.
- Non-alphabetic characters: Digits, punctuation, and whitespace are ignored and preserved.

## Anything special

- Safety vs. Unsafe: The problem hint suggested using unsafe { s.as_bytes_mut() } for direct byte manipulation. While that approach avoids allocating a new String buffer, it requires the programmer to guarantee that the string only contains ASCII and that we don't break UTF-8 boundaries. Since we are replacing 1-byte ASCII characters with 1-byte ASCII characters (*), the byte-level approach is safe but more verbose and risky. The chosen chars().map().collect() approach is safer and more readable.
- Performance: Because we are replacing one character with exactly one character, the length of the string does not change. When assigning *s = censored, Rust can often optimize this by swapping the internal buffers if the capacity allows, minimizing allocation overhead.
- Readability: Using the matches! macro makes the vowel check concise and easy to extend if requirements change (e.g., adding 'y').
