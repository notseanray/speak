## General Concepts
Speak uses some complex concepts, but I need to explain them in order to help you understand the code. I'll list them in order of importance.

### The Memory
(This isn't refering to the computer memory) As you know, we don't talk word for word, we talk forming phrases by joining several *syntactic units*. For example, *"My name is Alex"* is formed by *"My name"* & *"is Alex"*, we can think of this like we think of assigning a value in the mind of other people. The variable *"My name"* *"is"* assigned *"Alex"*.

**The memory is the length of those syntactic units.**
(With some more to clarify).

The way to analyze the phrase is (omitting details) about **chunking words**, and the length of each *chunk* is defined by the memory you used.

If you have lots of data, covering every case possible, you may want to use longer memory (like 4 or 5), and if you have very little data (like a dozen phrases hardcoded) it's recommended to use a shorter memory.

You may be asking **"What if the memory and the phrase length don't match up?"** If that happens, the last chunk will be of the last *length of phrase* - *memory* to *length of phrase* (Making sure to catch all words).

Maybe you can understand it better by looking at a graph.