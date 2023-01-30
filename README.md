# randoid
Rust nanoid implementation

This is a rust implementation of [`nanoid`s](https://github.com/ai/nanoid).

It generates unique IDs as strings that are more compact than UUIDs.

By default, it generates strings of 21 characters from an alphabet of 64 symbols
(a-z, A-Z, 0-9, "_", and "-").

## Features

This particular implementation of nanoid has the following features (many of which differ from the [`nanoid` crate](https://github.com/nikolay-govorov/nanoid)):

- no_std support
- can be used without allocating (by writing characters directly to output)
- Allows using any [`Rng`](https://docs.rs/rand/latest/rand/trait.Rng.html) implementation as a source of random data.
- Implementation is optimized for the size of the alphabet being a power of 2
- [`smartstring`](https://crates.io/crates/smartstring) support, if the `smarstring` features is enabled (as an additive feature).

## Limitations

- Requires knowing the size of the alphabet at compile time (the main reason for this is it can help the compiler optimize it better)
- Size of alphabet must be a power of 2
- Use of generics could increase compilation time


## Feature Flags

- `alloc`: Requires use of the `alloc` crate, and allows creating an id as a `String`
- `std`: Use full `std` library
- `std-rand`: Inlcude `rand/std` and `rand/std_rng` features, and add support for using `thread_rng()` as the default source of random data.
- `smartstring`: Add a function for creating an id as a `SmartString`

## Usage

### Install

```toml
[dependencies]
randoid = "0.2.0"
```

### Simple

```rust
use randoid::{randoid, Generator};

// All of the below generate a string like "9wxwPU-kQU-RDjYdxj6Eq"
let id = randoid();
let id = randoid!();
let id = Generator::default().gen();
```

### Custom length

```rust

use randoid::{randoid, Generator};

// The below generate a string like "M_P_lJcWfI"
let id = randoid!(10);
let id = Generator::with_size(10).gen();
```

### Custom alphabet

```rust
use randoid::{randoid, Generator};

let id = randoid!(21, ['a', 'b', 'c', 'd']);
let id = Generator::with_alphabet(&randoid::alphabet::HEX).gen();
```

### Custom random number generator

```rust
use randoid::{randoid, Generator};
use rand::rngs::OsRng;

let id = randoid!(21, &randoid::alphabet::DEFAULT, OsRng);
let id = Generator::with_random(OsRng).gen();
```

## About the name

"nanoid" was already taken by a similar library. I considered something like "nano-id" or "nanoid2",
but thought those were too similar. Since the IDs are generated randomly, I decided on "randoid" as
an abbreviation of "RANDOm ID".

## Acknowledgments

The original [nanoid](https://github.com/ai/nanoid) of course.

Also, <https://github.com/nikolay-govorov/nanoid>, as inspiration for this project.
