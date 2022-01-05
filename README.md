<h1 align=center>Speak</h1>
    <p><b>Speak</b> is a Rust crate focused in talking with the machine, you can get it from <bold><a href="https://www.crates.io/">crates.io</a></bold> (Currently is not uploaded)</p>

<h2>Installation</h2>
To install the crate, just add the following line into your `<bold>Cargo.toml</bold>` file: (Assuming you have Rust installed).

<br>

```toml
[dependencies]
speak = "*"
```

<h2>Usage</h2>
To use the crate, you'll need three things:
<ul>

<li>Have the crate in your <b>Cargo.toml</b></li>
<li>Have data (Expected inputs and outputs)</li>
<li>Use the function `run()` properly</li>

</ul>

To run `run()`, you'll need to pass the following arguments:

<ol>

<li>An input (`&str`)</li>
<li>A HashMap</li>
<li>A "no similarities between input and data found" function</li>

</ol>

I recommend this last function to be something like:
```rust
fn no_similarities_found() {
    println!("No similarities found");
}
```

Is not necessary to use the function `std::process::exit(0)` in this function, <b>Speak will do it for you.</b>

