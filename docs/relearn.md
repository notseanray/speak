# Relearn

The `relearn` module has two function, `relearn_direct` and `relearn_indirect`.

These two function are used to re-train the algorithm with the new dataset without the need of relearning the whole dataset.

## `relearn_direct(...)`

* Returns the original HashMap + the new dataset.

* All the original data (being literals) are still legible.

* **Doesn't really relearn the dataset**, you have to use the `learn(...)` function again.

* It's meant for having to archive the dataset in an external file or the need of serialization the dataset.

## `relearn_direct(...)`

* Returns a Vec\<u16\> for feeding directly into the `run(...)` function, without having of using the `learn(...)` function again.

* Not suited for external writing or serialization. It is just a list of unsigned 16-bit integers.

* Doesn't recompute all the values. Its several times faster than the `relearn_indirect(...)` function.
