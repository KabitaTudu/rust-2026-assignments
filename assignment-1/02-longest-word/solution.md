# Solution notes: Longest word slice

## Approach
Iterate over all whitespace-separated words using built-in 'split_whitespace' iterator to yield &str slice borrowed from the input sentence. Each word's byte length is compared with the current maximum. If longer word is found, then the max length is updated. Updation of length only when the length of new word is strictly greater than that of the current word to preserve the first occurrence of the longest word.

## Edge cases handled
- empty input: spit_whitespace() yields no items, loop never runs, so it returns None.
- single word: loop runs once, word is stored and returned as Some(word).
- strict greater checking condition: to preserve the first occurrence of the longest word. 

## Anything special
- Zero allocation: the function returns Option<&str>, borrowing directly from the input sentence. So, no string is created.
- Performance: O(n)
