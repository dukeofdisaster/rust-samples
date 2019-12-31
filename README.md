# rust-samples
A collection of samples from rust coookbook

## tests and benchmarks
testing/ covers some unit tests.

```
cargo new testing --lib
```

## Sharing code among types
Rust uses traits over interfaces (interfaces are very common in OO langs).
Traits are like abstract classes since they provide the API aspect and default
implementations. A `struct` can impelement various traits, and thus offer the 
same behavior with other structs that implement the same traits.

```
cargo new traits --lib
```

- Program to an interface, not to an implementation
  - not super easy in rust; in 2018 ed, functions can accept an impl MyTrait
  parameter, where earlier versions used Box<MyTrait> or o: T and later
  where T: MyTrait. The point is its a trade off: less complex abstractions with
  concrete type, or more generics and other complexity for cleaner encapsulation.

- favor object composition over class inheritance: There's no inheritance
  in rust. Object composition is still something that seems like a good idea. 
  Add trait type properties to your struct instead of the actual type. However
  unless it's a boxed trait (slower, dynamic dispatch), there is no way for the
  compiler to know exactly the size it should reserve -  a type instance could 
  have 10x the size of the trait from other things. Therefore, a reference is
  required. Unfortunately, though that introduces explicit lifetimes -- making
  code more verbose and complex to handl.e

Rust favors splitting behavior from data; behavior goes into a trait and data
remains in the original struct. KeyValueConfigService doesn't manage any data, 
it just reads and writes Config instances.

## web shit with Rocket
For whatever reason I decided to pay most attention to Rocket vs actix. The 
dev on rocket seems very active, and the homepage was very detailed and thorough. 

[rocket.rs - getting started](https://rocket.rs/v0.4/guide/getting-started/)

Rocket requires nightly rust, but this is pretty easy as you have 2 options
1. Use nightly as default : probably not recommended for beginners

```
rustup default nightly
```

2. After a project has been created, use nightly for that project only

```
cargo new myproject
cd myproject
rustup  override set nightly
```

3. "If your rocket app stops building, ensure you're using the latest version
of Rust nightly and Rocket by updating your toolchain"

```
rustup update && cargo update
```
