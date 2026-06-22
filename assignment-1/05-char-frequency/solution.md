# Solution notes: Character frequency, sorted

## Approach

A HashMap stores the count for each character. Iterating through the string, every character increments its corresponding value in the map. The entry API initializes new characters to 0 before adding 1. The map converts into a Vec of tuples, where each tuple holds a character and its count. The list sorts using a two-part rule:

- Primary: Counts sort from highest to lowest (descending).
- Secondary: If counts match, characters sort alphabetically (ascending).

## Edge cases handled

- Empty string: An empty input results in an empty map and an empty returned list.
- Single character: The map contains one entry with a count of 1.
- All unique characters: Every character has a count of 1, resulting in a list sorted purely alphabetically.
- Spaces and symbols: Spaces and punctuation count as valid characters and sort based on standard character codes (spaces typically appear before letters).
- Ties: The sorting logic automatically handles equal counts by applying the alphabetical rule.

## Anything special

- entry API: Using counts.entry(c).or_insert(0) simplifies logic by handling key creation and retrieval in a single step, avoiding explicit if/else checks.
- Chained Sorting: The .then_with() method chains sorting criteria cleanly. Rust applies the second rule (alphabetical) only when the first rule (count) results in a tie.
- Efficiency: The algorithm iterates through the string once (O(n)) and sorts only the unique characters (O(m log m)), making the process fast even for long strings.
