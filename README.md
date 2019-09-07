# Learning rust

## Resources
- https://www.youtube.com/watch?v=FYGS2q1bljE
- https://www.youtube.com/watch?v=zF34dRivLOw 


## Key concepts
### References
- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

In the various language i have worked with such as PHP and C# there is no need to think about references, copies and pointers. 
You just assign the variable and it works. When working with Rust and you try copying some objects you get a compile time error that the variable is out of scope of can't be mutated.
Without reading the documentation it is quite hard to grasp why these errors occur and how to fix them, after reading the references and borrowing chapter it became way more clear. 

Rush takes a unique approach in managing memory, it checks where which memory will be used at compile and what values will be available. Because of this the program is way more performant and doesn't need to garbage collect at  run time. 

Borrowing an instance using `&` is preferred because it allows the instance to be used later. When using methods in struct implementation `self` is passed as borrowed/non-mutable(`&self`) because changing the object itself to a new instance almost never happens.
To pass it as mutable reference the following can be used `&mut self`, this will cause the pointer to be moved and will no longer be available if the method runs out of scope.    

When using methods the syntax for instance and non-instance functions is the same. The only difference is the addition of `self` for instance method. If the method signature doesn't contain a `self` parameter it is marked as a associated function. Associated functions can be called without an instance but are bound to the struct.
A instance function can be called using the `instance.method` syntax and an associated function can be called using the double colon syntax: `Rectangle::square()`.

### Enums
- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

Rust has enums that look familiar to syntax of other languages such as C# and Java but have a few key differences. One major difference is that they can be used to store values in a specific enum type:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

### Namespaces
Rust projects have various ways to structure code, the top level construct is a Package, these contain crates. Each crate produces a library or executable.
The code in a crate gets structured using modules that determine what parts can be used and which parts are internal/private. These modules can be accessed using the `use` keyword.
When exposing code from a module `Paths` can be used to specify how it should be named. 

When using function the best practice is to import the module it is in instead of the functions. This way it is clear that the function isn't placed in the current file but in an imported module(`namespace::my_function`). When using Structs or enum it is recommended to import them directly to make the code less verbose(`use std::collections::HashMap`, `HashMap::new()`).
Because module names have to be unique the `as` keyword can be used to alias to non-unique names:
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

### Generics
Rust has support for generic, these can be used to prevent duplicate code and allow the `structs` and `functions` different types. In most other languages this causes an overhead because it needs to determine the type on run-time. 
Rust uses a process called `monomorphization` to determine the possible concrete types for generics and generates the concrete implementations. Because of this the generic value doesn't have to be determined at run-time, this means no run-time impact. 

### Traits
Code can be reused or enforced to conform to a specific format using `traits`. Traits look simular to structs but they differ because function implementations are not required in `traits`. An example trait would be a `Summary` that would contain the method signature for `fn summarize(&self) -> String;` this would enforce every concrete implementation to implement the method:
```rust
impl Summary for School {
    fn summarize(&self) -> String {
        format!("{} is located in {}", self.name, self.location)
    }
}
```

Another powerful feature is the short syntax sugar for using it as requirement(see `T`) for a generic. Example without the short-hand syntax:
```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

with syntax sugar:
```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Because it is so common to restrict the `type` arguments the `impl` sugar really cleans up the code.

### Testing
Rust provides testing utilities by default such as `assert!`, `assert_eq!` and `assert_ne!`. Code can be tested by annotating the method with `#[test]` and the module with `#[cfg(test)]`. A test will be marked successful if there hasn't been a `panic`. Panics can be caused if the `assert_*` macro's evaluate to false. Checkout the following examples:
```rust
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(4, 2 + 2); // OK
        assert_eq!(4, 2); // Panic because of assert_eq!, FAILS
    }
}
``` 