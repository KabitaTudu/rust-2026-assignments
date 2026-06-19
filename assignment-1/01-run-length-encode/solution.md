# Solution notes: Run-length encode

## Approach

Iterate over the string. Initially, set a counter to 1 for the 1st character and then increment it until a different character is encountered. When a different character is encountered, set the counter to 1. Repeat until the end of the string.

## Edge cases handled

Empty string checked earlier to return an empty vector.
