**Hey! This isn't an explanation that you need to know, just me talking about the project and all its features and algorithm.**

# How does it works? 🤔

The Speak algorithm is fairly simple, and that's why it's so powerful, yet easy to understand.
First, you need to understand that if <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cfrac%7BA%7D%7BB%7D%20%5Csimeq%20%5Cfrac%7BA%7D%7BC%7D}}" /> then <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{B%20%5Csimeq%20C}}" />, it's easy, right? Well, understanding that is the 40 percent of the algorithm.

<img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cbigg%28%5Cfrac%7BA%7D%7BB%7D%20%5Csimeq%20%5Cfrac%7BA%7D%7BC%7D%5Cbigg%29%20%3D%20%5Cbigg%28%5Cbig%7C%5Cfrac%7B%5Cbig%28%5Cfrac%7BA%7D%7BB%7D%5Cbig%29%7D%7BC%7D%20-%201%5Cbig%7C%20%5Cleq%20%5Cmu%20%5Cbigg%29}}" />, where <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cmu}}" /> is the threshold that the users gives to the algorithm.

But, what are those generics? Well, they're *the sum of the chunks from the translated values.* What does that mean?

## Translation:

Words are bad, a word uses various characters, and those characters take up a lot of space, wouldn't it be nice if we could translate a whole world into a single integer?

When I say *"translation"* I don't mean changing a phrase between two language, I mean converting a word into a number.

If you're curious of how the `translate(...)` internal function works. It's pretty simple. Here it is, in Pseudo-code:

```pseudo-code
FOR word IN phrase:
	FOR character in word:
		sum += character AS integer

	list.push( sum ^ ( 3 / 2 ) )

RETURN list
```

This simple function gives us a list of integers, each one representing a whole word.

Being unsigned 16-bit integers, we save a lot of space doing this simple operation.

<small>(For efficiency reasons, the real function takes a list of phrases, not just one.)</small>

## Chunking:

A `Chunk` is a list of slices of a phrase. Each slice is a n-length group of words.

In Rust this is `Chunk: Ve<&[u16]>`.

Ok, let's say that we have a phrase like this: <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Ba%2C%20b%2C%20c%2C%20d%2C%20e%2C%20f%2C%20g%5D}}" /> and our memory is 3. We want to chunk that phrase into 3-length groups. So we'll have:

```mermaid
graph TB
		A(a)
		B(b)
		C(c)
		D(d)
		E(e)
		F(f)
		G(g)

		N1["Not found!"]
		style N1 stroke-dasharray: 5 5

		N2["Not found!"]
		style N2 stroke-dasharray: 5 5

		I1[Iteration 1]
		I2[Iteration 2]

		I22[Bugged iteration 3]

		I1-->A;
		I1-->B;
		I1-->C;

		I2-->D;
		I2-->E;
		I2-->F;

		I22-->G;
		I22-->N1;
		I22-->N2;
```

Mmmmm... That's not good, that's a bug, the size of the phrase, this happens because <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{C%5Cbmod%7B%5C%23P%7D%5Cneq0}}" /> (being <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{C}}" /> the memory size and <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5C%23P}}" /> the phrase size). What's the solution to this catastrophe? Well, it's fairly simple, just add as the final iteration (In this case, the 3rd iteration) the slice between the end of the phrase - the memory size and the end of the phrase.

```mermaid
graph TB
		A(a)
		B(b)
		C(c)
		D(d)
		E(e)
		F(f)
		G(g)

		I1[Iteration 1]
		I2[Iteration 2]

		I22[Real iteration 3]

		I1-->A;
		I1-->B;
		I1-->C;

		I2-->D;
		I2-->E;
		I2-->F;

		I22-->E;
		I22-->F;
		I22-->G;
```

That's it, **all problems are fixed, forever**.

**Oh no! Another problem**

What would happen if the memory size was bigger than the number of words in the phrase? Well, in that case we would get a slice from a non-existent index.

That's why, before chunking anything, we check if the phrase is long enough to be chunked by the current memory size. If it is: great!, else we modify the memory size to be the length of the phrase.

**Ok, all is ready, but what do chunks do?**

As you know, every phrase is made up of syntactic units, each unit representing a meaning.

Some of the most common units are things like: *I have a ..., my name is ..., I am a ...*, etc.

These units have a meaning, well, those units are called *chunks* in the code.

## The algorithm

The first step is to find the relations between the chunks of the key and the value.

Dividing two numbers is finding the relation between the two, so we need to divide the total value of the two chunks to find their relation.
<h3 align="center"><img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cfrac%7B%5Csum_%7Bi%3D0%7D%5E%7B%5C%23K_%7Bchunk%7D%7D%7BK_i%7D%7D%7B%5Csum_%7Bi%3D0%7D%5E%7B%5C%23V_%7Bchunk%7D%7D%7BV_i%7D%7D}}" /></h3>



Ok, now we have that relation, what do we do now?

Well, first of all, let's add all those relations together in a group named <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cpsi}}" />. That gives us:

<h3 align="center"><img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cpsi%20%3D%20%5Cbigg%5C%7B%5Csum%7B%5Cfrac%7B%5Csum_%7Bi%3D0%7D%5E%7B%5C%23K_%7Bchunk%7D%7D%7BK_i%7D%7D%7B%5Csum_%7Bi%3D0%7D%5E%7B%5C%23V_%7Bchunk%7D%7D%7BV_i%7D%7D%7D%5Cbigg%5C%7D}}" /></h3>



Ok, right now we have a group of relations, what do we do now? Was that useless?

**Now we can start using the information that I gave at the start!**

---

> First, you need to understand that if <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cfrac%7BA%7D%7BB%7D%20%5Csimeq%20%5Cfrac%7BA%7D%7BC%7D}}" /> then <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{B%20%5Csimeq%20C}}" />, it's easy, right?

With that, we now have a group of relations (division) between keys and values, and we can find the relation between input and values, so let's do that:

being <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{i}}" /> the input, already translated to numbers.

---


Chunk function:

<h3 align="center"><img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{C%28x%20%3D%20%5C%7B%5Ccdots%5C%7D%2C%20m%29%20%3D%5Csum_%7Bi%3Dm%5C%20%5Cgeq%5C%20%5C%23x%7D%5Cleft%5C%7B%5Cbegin%7Barray%7D%7Bll%7D%20i%20-%20%5C%23x%20%3C%20m%20%26%20x_%7B%5C%23x%20-%20m%5C%20..%5C%20%5C%23x%7D%5C%5C%20%5Ctext%7Botherwise%7D%20%26%20x_%7Bi%20-%20m%5C%20..%5C%20i%7D%5Cend%7Barray%7D%5Cright.}}" /></h3>





<h3 align="center"><img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{E%28I%2C%5C%20K%2C%5C%20V%2C%5C%20m%2C%5C%20t%29%3D%5Csum_%7Bi%5C%20%3D%5C%200%5C%20%5Cgeq%5C%20%5C%23a%7D%5Cfrac%7B%5Csum_%7Bkc%5C%20%3D%5C%200%7D%5E%7B%5C%23C%28K_i%2C%5C%20m%29%7Dkc%7D%7B%5Csum_%7Bvc%5C%20%3D%5C%200%7D%5E%7B%5C%23C%28V_i%2C%5C%20m%29%7Dvc%7D%20-%5Cfrac%7B%5Csum_%7Bic%5C%20%3D%5C%200%7D%5E%7B%5C%23C%28I_i%2C%5C%20m%29%7Dic%7D%7B%5Csum_%7Bvc%5C%20%3D%5C%200%7D%5E%7B%5C%23C%28V_i%2C%5C%20m%29%7Dvc%7D%5Cleft%5C%7B%5Cbegin%7Barray%7D%7Bll%7D%09%5Cleq%20t%20%26%20R_%7B%5C%23r%7D%20%3D%20V_i%5C%5C%09%5Ctext%7Botherwise%7D%20%26%20%5Ctext%7Bcontinue%7D%5Cend%7Barray%7D%5Cright.}}" /></h3>













Being <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{R}}" /> the final result.

And I know what you're thinking...
If both expressions are divided by <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Csum_%7Bvc%5C%20%3D%5C%200%7D%5E%7B%5C%23C%28V_i%2C%5C%20m%29%7Dvc}}" />, then we could simplify that!

**Well, yes but actually no**, because in the real code we have a variable `mega` (Vector) with already pre-computed learnt values. We're dividing the input by the values because the keys are already divided!

---

Maybe you're also thinking that the original equation was $$
