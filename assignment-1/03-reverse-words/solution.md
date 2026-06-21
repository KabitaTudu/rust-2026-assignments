# Solution notes: Reverse the word order

## Approach
split_whitespace() splits the input string by any unicode whitespaces(spaces, tabs, newline) to filter out empty strings caused by consecutive whitespace. This yields an iterator of &str slices. The order of iterator is reversed using rev(). Then the reversed slices are collected into a temporary Vec<&str>. The vector elements are concatenated into a new String by inserting a single space between each word via join(" ").

## Edge cases handled
- Empty input and whitespace-only input: split_whitespace() yields an empty interator; collect created an empty Vec; join returns an empty String. whitespace-only input is basically treated as an empty input.
- Single word: rev() doesn't affect on a single element; join returns the word without adding any separators.

## Anything special
- split_whitespace() correctly handles non-ASCII whitespace characters that split(' ') would miss.
- The intermediate Vec stores string slices borrowed from the input and avoids unnecessary string cloning until the final join step.
