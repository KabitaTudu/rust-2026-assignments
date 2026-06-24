# Solution notes: Group anagrams

## Approach

The code groups words that are anagrams using a map. For each word, a "signature" is created by making the word lowercase, sorting its letters, and joining them back together. Words with the same signature are anagrams. The original word is added to a list for that signature. Since words are added in order, their original sequence is kept inside each group. Finally, all the groups are collected into a list.

## Edge cases handled

- Empty list: Returns an empty list.
- No matches: Each word ends up in its own group.
- Different casing: Words like "Listen" and "Silent" match because the signature ignores case, but the output keeps the original capitalization.
- Single word: Returns one group with that word.
- Different lengths: Words of different lengths cannot be anagrams, so they stay in separate groups.

## Anything special

- Simple Signature: Sorting letters is an easy way to check if words are anagrams.
- Order preservation: Adding words to a list one by one ensures the original order is preserved inside each group.
- Cloning: The original words are copied into the groups so the input list remains unchanged.
