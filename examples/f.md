<h1>Randomness</h1>
Randomness is an optional (but highly recommended) feature that will pass some randomness to the algorithm.

### What does this mean?
There's two ways the algorithm works, the first way is **analyzing every single entry**, this method is slow, and doesn't have the ability to *encourage* or *disencourage* some entry.

The second method is **analyzing every single entry until a break point, then aplying a distribution**, this method is more fast, when the break point is reached, the algorithm will start to ignore some cases. The distribution used is very simple just:
<h3 align="center"><img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cbigg%5C%7B%5Cbegin%7Barray%7D%7Bll%7D%09i%20%5Cleq%20%5Ctext%7Brange%7D%20%26%20%5Cdotsc%09%5C%5C%09i%20%3E%20%5Ctext%7Brange%7D%20%26%20R%5Cin%5C%7B0%2C...%2C%5C%23V%5C%7D%5C%20%5Cbigg%5C%7B%5Cbegin%7Barray%7D%7Bll%7D%09%09R%20%3C%20i%20%26%20%5Cdotsc%09%09%5C%5C%09%09R%20%5Cgeq%20i%20%26%20%5Ctext%7Bpass%7D%09%5Cend%7Barray%7D%5Cend%7Barray%7D}}" /></h3>

The distribution is very simple, and just random enough to serve our purpose.
### Why use a distribution?
Activating the randomness will change the way that the `run` algorithm works, adding a new system, the *ranking system*. The ranking system will take into account just the first `RANGE` words, and then will use the distribution. This means that, for example the entry 10 will have a probability of being chosen of (If <img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%20%5Ctext%7Brange%7D%20%3C%2010%20}}" />)<img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%20%5Cfrac%7B10%7D%7B%5C%23V%7D%20}}" />

