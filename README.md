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
