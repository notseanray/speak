###### Currently Speak is in early development, it wont execute correctly, it isn't even with all functions.
<h1 align=center> Speak </h1>
Speak is a Natural Language Processor Bot / Conversational Bot written in Rust (But with plans to expand to other languages).<br>
Speak is the tool to use if you want to:

* Have a bot to talk.
* Assist your users without sounding robotic.
* Experiment.

---
<h2 align=center> ⚙️ Installation </h2>

Currently **Speak isn't published**.

<h2 align=center> 💻 Usage </h2>

Using Speak is very simple, because you just need to know about two functions and a struct. (Being public ***just*** the necessary interface you'll use.)

<h3 align=center>

`Map<T>`:
</h3>

This struct is like a `HashMap`, but with just the necessary functions to create and modify the struct.

The `Map<T>` is built with one parameter, a vector made up of tuples with type `String` or `&str`. `Vec(T, T)>` where `T`: `String` or `&str`.

**Example:**

```Rust
let map = Map::<&str>::from(vec![
        ("How are you?", "I'm fine!"),
        ("What's your name", "My name is Speak!")
]);
```

<h3 align=center> 

`train(Map<T>)`: 

</h3>

The `train(Map<T>)` takes **1** argument, `Map<T>`, this function **returns a Vec<Vec<u32>>**, this numbers will not make sense for human, that's why we need the `run` function to interpret it, figure out a sentence based on the input and return it!.

```Rust

let map: Map::<&str>::from(vec![
    ("How are you?", "I'm fine!"),
    ("What's your name", "My name is Speak!")
]);

let trainedData: Vec<Vec<u32>> = train(map);
```

<h3 align=center>

`run(String, Vec<Vec<u32>>)`

</h3>

The `run(String, Vec<Vec<u32>>)` function creates a sentence based on the Map you gave it.

The second parameter (`Vec<Vec<u32>>`) is just the returning value of the `train(Map<T>)` function. You can just feed the `run(String, Vec<Vec<u32>>)` function the `train(Map<T>)` function

Obviously, the first parameter, `String` is your input.

```Rust
let map: Map::<&str>::from(vec![
    ("How are you?", "I'm fine!"),
    ("What's your name", "My name is Speak!")
]);

let trainedData: Vec<Vec<u32>> = train(map);
let input: String = String::from("How are you?");
let result: String = run(input, trainedData);

println!("{}", result);
```

<h2 align=center> ⚖️ License & Credits</h2>

Speak uses the [**GNU General Public License v2.0**](https://github.com/SpeakML/speak/blob/current/LICENSE/)

Currently I, [Alex G. C, aka Blyxyas](https://github.com/blyxyas), do the **100%** of this project **alone** and **just for learning and fun.**
