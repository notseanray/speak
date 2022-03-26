These parameters are optional, that means that `None` is a valid argument for them, as `None uses the default configuration for that parameter in particular, it's recommended to use the default configuration.

## Memory

As you know, each phrase is made up of groups of words, each group has a length, for example 3, that length is the memory.
E. g. The phrase "My name is Alex" is made up from "My name is" & "Alex"

As you can see, in real communication the length of the words is dynamic, but for the sake of being more efficient, the chunks that the bot analyzes are always the length of `memory`.

## Threshold

Each word differentiates from another with a number, for example the word *"spaghetti"* and the word *"hi"* differentiate for a lot, it wont meat the threshold if the threshold is low, for example *0.2*, because the value of *"spaghetti"* divided by the value of *"hi"* is more than 0.2, so it wont recognize those words as words that mean the "same"

## Multiplier
The function `translate(...)` converts each word to a number an then multiplies that number, this means that words with, for example, 400 of difference between them, with this multiplication, they'll be at 700, this is another way of demanding that the words are close enough to select.