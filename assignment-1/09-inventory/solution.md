# Solution notes: Inventory

## Approach

The restock function uses a HashMap to aggregate quantities. It consumes both input vectors by iterating over them directly. The entry API with or_insert handles both new items (inserting 0) and existing items (returning a mutable reference to the current quantity), allowing quantities to be summed efficiently. Finally, the map is converted back into a vec.

The summary function takes a slice reference &[(String, u32)], ensuring the original data is not consumed. It calculates the item count via .len() and the total units by iterating over the slice and summing the quantities. The result is formatted into a string using format!.

## Edge cases handled

- Empty inputs: restock handles empty vectors by returning an empty vector immediately. summary correctly returns "0 items, 0 units".
- Disjoint items: Items unique to either list are simply inserted into the map.
- Overlapping items: Quantities for matching names are added together via the entry API.

## Anything special

- restock takes ownership (vec) rather than borrowing. This avoids cloning the String keys, improving performance and demonstrating Rust's move semantics.
- Entry API: Using map.entry(name).or_insert(0) is more efficient than checking contains_key followed by insert or get_mut, as it performs only one hash lookup per item.
- Iterator Summation: summary uses .iter().map(|(_, qty)| qty).sum() for a concise and readable calculation of total units without explicit loops.
