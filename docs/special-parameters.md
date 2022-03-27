# Special parameters

* Memory
* Multplier
* Threshold

**All this parameters are optional!**, if you don't set them, they'll be set to the recommended values. If you don't know what you're doing, I recommend you to just use `None` and flow with the default.

## Memory

Every phrase is made up from words. We make a phrase from adding sequences of words together. Well, the `memory` parameter is used to define how many words we take into account into analyzing a phrase.

The functions that takes this parameter take into account that maybe the length of the phrase divided by the number of words in the phrase is not an integer. So this functions will take into account until the last words, and then scan the words between the length of the phrase minus the memory and the length of the word.

```mermaid
graph TD
	A("Hi,")
	B("my")
	C{{"name"}}
	D("is")
	E("Alex")

	F["Not found!"]

	style F stroke-dasharray: 5 5

	X["Iteration 1"]
	Y["Iteration 2"]
	Z["Bugged iteration 2"]

	X-->A;
	X-->B;
	X-->C;

	Y-->C;
	Y-->D;
	Y-->E;

	Z-->D;
	Z-->E;
	Z-->F;
```

###### Honestly, I just wanted to show you how it works, and this graph.

## Threshold

As you know, we divide two values to find their relations. Well, that relation is then checked against the threshold, if it doesn't passes the threshold, the word is not elected.

This is the operation to determine if a word is elected. As you can see, if the threshold is too low (less than 0.1 is not recommended), the word "spaghetti" and the word "spagetti" will not be relationated. But if the threshold is too high (more than 0.3 is not recommended), a lot of words, even if they are very different, will be relationated and the final result will not have sense.