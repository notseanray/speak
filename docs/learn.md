# `Learn(...)`

`learn(...)` is one of the main functions in the library, this function is used to train the algorithm with a given dataset. This **dataset being a `Hashmap<T, T>`** (in `std::collections::HashMap`).

## Parameters and types

`data: Hashmap<T, T>`: The dataset to train the algorithm with.
`T`: The type of the dataset. (`String` or `&str`).
`memory: usize`: Especial parameter. (Optional).

As always, I recommend to use the default values for the special parameters using `None` as those parameters.

For more information about the special parameters, see [special parameters](special-parameters.md)

**T** is the main type of the dataset, it must satisfy the following traits:

* `speak::Literal<String>`
* `Clone`
* `ToString`

Both `&str` and `String` satisfies all these traits. In both of the main functions every single string is converted to a `String`.

## Example

```rust
let map: HashMap<&str, &str> = HashMap::from(
	vec![
		("hello", "world"),
		("hola", "mundo")
	]
);

// I want to use the default parameter, so I'll use 'None'.

let learned = learn(map, None);
```

## Details

**Be careful with this function**, its big O is $\KaTeX$ that's pretty big.
If you need to create a closed feedback loop (training with newly created data), you can use the `relearn_direct(...)` function. In the case that you want to add data and still hash the dataset, you can use `relearn_indirect(...)`, this will return a `HashMap` and you can serialize and store it somewhere.

### How does it works? 🤔

If you want to know how the algorithm works, you can read that in the [**How does it works?**](how-does-it-works.md) file.
