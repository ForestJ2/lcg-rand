<div align="center">

# lcg-rand

Lightweight open source random number generator built in Rust, using the Linear Congruential Generator algorithm. A select few ease-of-life functions have been added.

![GitHub](https://img.shields.io/github/license/ForestJ2/lcg-rand?color=f84&style=flat-square)&nbsp;&nbsp; 
![GitHub release (latest by date)](https://img.shields.io/github/v/release/ForestJ2/lcg-rand?display_name=tag&style=flat-square)&nbsp;&nbsp;
![GitHub Release Date](https://img.shields.io/github/release-date/ForestJ2/lcg-rand?style=flat-square)

</div>
<hr>

## Installing

You can either use the `cargo` command at the root of your project, or download the source code itself.

```
cargo add lcg-rand
```

## Running the tests

If you downloaded the source code then integration tests are included.

To run the tests and ensure the software is working on your system run the below command at the project root.

```
cargo test
```

## Usage

```rust
use rand::LCG;

fn main() {
    let mut random: LCG = LCG::new();
    let arr: [&str; 2] = ["Hello World!", "hello world"];

    // Print the auto-generated seed.
    println!("{}", random.seed);
    
    // Generate a random number.
    println!("{}", random.next());
    
    // Generate a random number with the specific range (inclusive).
    println!("{}", random.range(0, 10));
    
    // Select an item out of a slice/array
    println!("{}", random.choose(&arr));
}

```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.