# Learning rust
This document describes the process of learning a new programming language, in this case Rust. Checkout the demo for the final product or read the [journey](#journey) below.

## Demo
[demo video](https://mgdev-bucket.s3-eu-west-1.amazonaws.com/maartendev_app_project_rust.mp4)
### Local setup steps:
1. `git clone git@github.com:MaartenGDev/learning-rust.git`
2. setup rust [tutorial](https://www.rust-lang.org/learn/get-started)
3. start a redis container for in-memory storage: `docker run  -p 6379:6379 -d  redis`
3. run `cargo run` in the project to start the server.
4. send the desired state to the api(http://localhost:3000/state), example:
```
{
	"containers": [{
		"image": "nginxdemos/hello"
	},
	{
		"image": "nginxdemos/hello:plain-text"
	}]
}
```
5. Check `docker ps`
6. Success! The desired state is now automatically managed.
7. Check the demo video if any steps fails.

## Journey
### Goals
I started this journey to practice with learning new languages and getting more experience with different programming paradigms. Because I am already quite familiar with the Objected Oriented paradigm this journey will be focused on a more functional approach. 

### Why Rust
The first step of this journey was choosing the programming language. When searching for options I looked through a few repositories of tools that I like using and repositories of rapid innovating companies such as Netflix and Cloudflare. 
I looked at [kubernetes](https://github.com/kubernetes/kubernetes), [docker](https://github.com/docker/engine), [prometheus](https://github.com/prometheus/prometheus), [wrangler](https://github.com/cloudflare/wrangler), [quiche](https://github.com/cloudflare/quiche) and [rend](https://github.com/Netflix/rend). After scrolling through the repositories I watched a [few youtube video's](https://www.youtube.com/watch?v=FYGS2q1bljE) that address common questions about the used languages and the corresponding eco-systems.

One common factor between these repositories is that they are quite new and are already really popular, are focused on performance but not as low-level as the C language and focus on creating memory safe applications. The most commonly used languages in these repositories are [Go](https://golang.org/) and [Rust](https://www.rust-lang.org/), both provide type safety as a compiled language and a comprehensive eco-system. 
Both eco-systems provide enough tools to get started quickly and focus on delivering features instead of writing low-level code, all this without any impact on performance.

I chose Rust over Go because it has a unique approach for handling memory and has immutable data as a core feature of the language. Another reason for choosing rust was the pretty high-level language constructs(no need to mess around with allocating memory) with great performance, this enabled me to focus on creating features instead of worrying about memory or data mutation.

### Learning Rust
When learning a language I prefer to start with learning the syntax and getting familiar with the core concepts. The official Rust site provides a compact but comprehensive [book](https://doc.rust-lang.org/book/title-page.html) about the language. It covers the basic concepts like syntax but also covers more advanced concepts like multi-threaded applications and pointers.

The book was easy to follow and provided enough examples to see how the concepts can be used. While reading the book I focused on concepts from the functional programming paradigm such as immutability, pure functions and composition.
To improve my understanding of certain more complicated subjects I wrote summaries about each topic while reading the book. These summaries have been [added at the bottom](#key-concepts) of this document.

After learning the basic concepts I like to start trying out things and see how they work. I started with using the various concepts such as for/while/structs/arrays/logs in [small files](https://github.com/MaartenGDev/learning-rust/tree/practice-files/src) that were explained in [a tutorial](https://www.youtube.com/watch?v=zF34dRivLOw). 
All these examples were quite easy to implement so I started trying to create some brute-force algorithms. This seemed straight forward at first but turned out quite hard because of the unique memory management techniques from rust.

The hard part when working with Rust is dealing with data lifetimes and ensuring no memory cleanup is required. Data in Rust can be passed around using various approaches such as: moving, borrowing and mutable borrowing. Each of these have their advantages and disadvantages.
The move operation moves the ownership of the object to the receiver, an example is demonstrated below:
```rust
pub fn run() {
   let test = String::from("Example here"); // Create new string object
    move_ownership(test);

    // Fails because the object no longer exist because it was moved into move_ownership
    println!("{:#?}", test);
}


fn move_ownership(data: String) -> String { // data comes into scope
     data
} // Here, the data parameter goes out of scope and the data is removed from memory
```

Because moving the object is often not desired an object can be borrowed:
```rust
pub fn run() {
   let test = String::from("Example here"); // Create new string object
    borrow_value(&test);

    // Succeeds because the `borrow_value` borrowed the value and thus has not moved it.
    println!("{:#?}", test);
}


fn borrow_value(data: &String) -> &String { // data comes into scope
     data // The data cannot be modified because it is a read-only borrow, not a mutable one.
} // Here, the data parameter returns the provided reference instead of disposing the object.
```

These concept seemed quite easy to use when trying them in small examples but turned out quite complicated when creating a more complex algorithm with more complex data structures.
I tried to use [mutable references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references) to be able to pass data around without transferring the ownership and to enable updating the data. This got hard to manage and became way too complicated when trying to learn rust.
I did end up using multiple mutable Vectors with nested mutable Vectors, this didn't work out because the algorithm was too complicated as a starting point. The algorithm was too complicated because I tried to move references inside references that updated references, this resulted in the following cryptic type definition:
![Complicated type](./docs/mut_type.png)
The algorithm attempt can be found [here](https://github.com/MaartenGDev/learning-rust/blob/b5f65dd6d71295019bc005761e3ef43b1bf61835/src/schedule_generator.rs). 


### The challenge
Because the brute-force algorithm was too complicated to start with I decided to switch it up and define a clearer and more practical challenge. Instead of following a tutorial I wanted to try and implement a system that focuses on a real world use-case and thus requires the use of the eco-system and best-practices.
The final product is a trimmed down version of the popular [kubernetes](https://kubernetes.io) platform. Kubernetes is the production grade container orchestration solution that is used for automating deployments with docker as part of the DevOps mindset. 

I chose this project because I have worked with kubernetes for deploying my own production cluster and loved using it. Kubernetes has the following core concepts:
- Client provides a desired state, the desired state defines:
    - Which (docker) containers should run
    - The amount of containers and how they can be reached
    - Deployment strategies(Blue-Green/Canary)
- Infrastructure as code
    - The client defines what should be running, not how
    - Kubernetes takes care of versioning, deployments and high availability.
    
To put it simply, it helps the architects easily deploy and manage systems and it is especially well suited for a micro-service architecture. My goal for this project is to take the core principles of kubernetes and make my own implementation.
By creating my own basic implementation I get to use various libraries for HTTP(server + client) and can practice with writing functional code for the business logic.

A more concrete definition is outlined below:
- Controlplane API
    - Accept json files that describe the desired state:
        - containers
            - image
    - uses the docker engine API to control docker containers
    - uses [hyper](https://github.com/hyperium/hyper) as HTTP server and [reqwest](https://github.com/seanmonstar/reqwest) as HTTP client.
- Scheduling service
    - starts docker containers if they are required by the desired state
    - stops docker containers if they are no longer needed by the desired state
    - displays changes in state
    - runs in a separate thread

### Planning the challenge
Aside from writing the business logic it is important to focus on using a different paradigm, is this case functional. Because Rust is a multi-paradigm language extra care has to be taken to ensure the use of function patterns instead of using objects because these are more familiar.
To enforce using the functional aspects of the language I wrote down a few guidelines that state a few best practices:

1. Functions/Methods should be pure, and thus have no side effects(every call with the same input returns the same result).
2. Data is immutable, when changing a structure a copy should be made with the changes instead of mutating the original structure.
3. No shared state, the data should be passed around instead of storing it in a global space.
4. Composition is preferred
5. Uses recursion or higher order functions to iterate over collection data instead of using a loop.
6. Passing around functions instead of abstracting it to classes.

Most of these rules are quite easy to apply in Rust because data is immutable by default and [using closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html) in combination with higher order functions `map` and `filter` is recommended:
```rust
let numbers: Vec<i32> = vec![1, 2, 3];
let updated_numbers: Vec<_> = numbers.iter().map(|x| x + 1).collect();
``` 

Variables defined using `let` are immutable by default, to be able to mutate the value creating a copy is required. Another option is to explicitly mark the value as `mutable`:
```rust
let val = 1;
val = 22; // Error, reassignment of immutable variable
```
After marking it as mutable:
```rust
let mut x = 3;
x = 5; // Works fine
```
### Working on the challenge
I started working on the module that manages the running docker containers because I have created the same functionality in other languages, this turned out to take way longer than expected. 
The first challenge occurred when choosing a library to send requests to the docker engine, because the docker API communicates using Unix Socket plain HTTP isn't sufficient. 

I tried using the [reqwest](https://github.com/seanmonstar/reqwest) library as first option because of the easy to use interface, after some research I had to conclude there was no Unix Socket support.
Because of this I switched to a lower-level library called [hyper](https://github.com/hyperium/hyper). While this library supported Unix Sockets it was quite hard to use because of the many new concepts regarding async processing.

Most problems came from the unfamiliar async higher orders functions such as `.for_each`, `.map` and `map_err`:
```rust
 let work = client
        .get(url)
        .and_then(|res| {
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        });
```

These functions caused trouble because I assumed how they would work because the names were familiar to the concepts from Javascript([.then](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/then), [.map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)). 
They had some shared behaviour between the languages but they are async by default, because of the new async concepts it was hard to fix issues like these:
```rust
error[E0599]: no method named `from_err` found for type `futures::AndThen<hyper::client::ResponseFuture, hyper::Body, [closure@src/docker_service.rs:29:19: 31:10]>` in the current scope
  --> src/docker_service.rs:32:10
   |
32 |         .from_err::<FetchError>()
   |          ^^^^^^^^
   |
   = note: the method `from_err` exists but the following trait bounds were not satisfied:
           `&mut futures::AndThen<hyper::client::ResponseFuture, hyper::Body, [closure@src/docker_service.rs:29:19: 31:10]> : futures::Future`
           `&mut futures::AndThen<hyper::client::ResponseFuture, hyper::Body, [closure@src/docker_service.rs:29:19: 31:10]> : futures::Stream`
           `futures::AndThen<hyper::client::ResponseFuture, hyper::Body, [closure@src/docker_service.rs:29:19: 31:10]> : futures::Future`
```

I ended up solving the errors by comparing the [provided examples](https://github.com/hyperium/hyper/tree/e3dc6c5511b2e5673d46bd3d278a86702bd0402c/examples) from the library with my code and changing parts. 
I also looking through the code of an [existing docker library](https://github.com/softprops/shiplift) to find out how they approached this problem.

### Cleaning up the code
After making a module work I would take some time to clean it up and simplify the code. I used some techniques like introducing generics to reuse code and using build-in functions(like map, filter) to replace commonly used data modifications.
During the refactoring the code the compiler really shined, it provided clear errors with tips on how to fix the issues(checkout the **= help**). The compiler is a excellent part of the language because it enables you to write safe and fast code with the detailed error descriptions and suggested solutions. 
```rust
error[E0277]: the trait bound `T: docker_service::_IMPL_DESERIALIZE_FOR_Container::_serde::Deserialize<'_>` is not satisfied
  --> src/docker_service.rs:34:25
   |
34 |             let users = serde_json::from_slice(&body)?;
   |                         ^^^^^^^^^^^^^^^^^^^^^^ the trait `docker_service::_IMPL_DESERIALIZE_FOR_Container::_serde::Deserialize<'_>` is not implemented for `T`
   |
   = help: consider adding a `where T: docker_service::_IMPL_DESERIALIZE_FOR_Container::_serde::Deserialize<'_>` bound
   = note: required because of the requirements on the impl of `docker_service::_IMPL_DESERIALIZE_FOR_Container::_serde::Deserialize<'_>` for `std::vec::Vec<T>`
   = note: required by `serde_json::from_slice`
```

The actual fix was really similar to the suggested solution(`where T: DeserializeOwned`):
```rust
fn fetch_docker_url<T>(path: &str) -> impl Future<Item=Vec<T>, Error=FetchError> where T: DeserializeOwned{
}
```

Aside from applying already familiar techniques I watched various youtube videos about cleaning up rust code([1](https://www.youtube.com/watch?v=NBBIu8JkxGs),[2](https://www.youtube.com/watch?v=mFcX3hDcFl4)), recapped the [functional language features chapter](https://doc.rust-lang.org/book/ch13-00-functional-features.html) and looked at the [higher order functions examples](https://doc.rust-lang.org/rust-by-example/fn/hof.html).

### The experience
Learning the basics of Rust like variables, loops, structs and control flows was quite easy following the book and watching some introduction videos. Working with and learning the unique way Rust handles memory was the most challenging part, while it was explained clearly in the docs it took a lot of practice to change the way I think about memory.
Because you have to take really good care where a references goes, the lifetime and the scope. This took a massive hit on productivity at the start of learning the language. 

Most languages I have worked with such as C# and Java handle all the memory management for you with garbage collectors and behind the scene optimisations. After using Rust for a while I realised a lot of magic happens in the other languages to keep references up-to-date, safe and available.
While the memory management in Rust causes a hit on productivity in the start it enables great performance in combination with really safe and easy to follow code.

When working on features for a project such as the current challenge the eco-system really shows it's maturity. There are packages([crates](https://crates.io)) for all common use-cases such as:
Http, Caching, Database, Encoding/Decoding, Security, Algorithms. This eco-system really boosts the productivity because you don't have to write low-level code and thus can focus on business-logic.

The language has a lot of good defaults, it makes everything immutable by default, forces you to think about memory, has a small footprint, can run anywhere and provides a sensible amount of functions by default([preludes](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)).
This forces you to write safe, fast and clean code, it may come across a boring language but that is a great thing when creating mission critical programs.

While it has support for creating all kind of applications varying from web applications to complicated algorithms, I would not use it for every project. The projects I would use it for are programs that need to be really fast and memory efficient while ensuring memory and data safety, examples include:
- Algorithms
- Mission critical services
- Micro-services with heavy load

While memory safety and insane performance are great they have a major impact on productivity. They have a negative impact on productivity because it forces a new way of doing things, this makes the language harder to learn. It is quite hard to use existing experience of other languages in Rust because it has it's own constructs and rules. 
This is why I would fallback to using `C#` or `Java` for projects where memory and performance aren't critical, a few examples are:
- Web applications
- Desktop programs
- Mobile applications
 
### Similarity with other languages
There are quite some similarities between Rust and other language like C# and Javascript. The most obvious are the higher order functions, all of these languages allow iterating over the lists to filter or update the contents. Both Rust and C# have integrated them deeply into the language to allow iterating over almost any list type.
Checkout the following examples that all implement the same logic:

Rust:
```rust
fn main() {
  let items = vec![1,2,3,4];

  let updated_items: Vec<i32> = items.iter().map(|a| a + 1).filter(|x| x % 3 == 0).collect();

  println!("{:#?}", updated_items);
}
```
C# (LINQ)
```csharp
public static void Main (string[] args) {
    var items = new List<int> {1,2,3,4};
    
    var updatedItems = items.Select(x => x + 1).Where(x => x % 3 == 0).ToList();
    Console.WriteLine(updatedItems);
}
```

Javascript:
```javascript
const items = [1,2,3,4];
const updatedItems = items.map(x => x + 1).filter(x => x % 3 === 0)
console.log(updatedItems);
```

The Rust and Javascript snippets are quite similar but have one major difference, `let` is immutable by default for Rust but `let` in javascript is mutable by default. In javascript `const` can be used to simulate immutability.
All of these language have sophisticated package management systems such as [NPM](https://www.npmjs.com/) for javascript and [NuGet](https://www.nuget.org/) for C# and [Crates](https://crates.io/) for Rust.

### Conclusion
I really like using Rust, it gives confidence over the code that I have written and helped me achieve the set goals. The final product is a sophisticated proof of concept that covers many key aspects of creating a production ready program:
data structures, transforming data, structuring code, async, file/console IO, exception handling and external packages. The learning journey was a pretty smooth ride but if I had to redo it I would do more research into the async aspects of the language before building products.

Focusing on async code early on would have enabled me to get started faster with external packages and spend less time changing code until it works. I also learned to love immutable data in combination with the higher order functions for more clear code that is easy to reason about.

Rust won't be the language of choose for all future projects but will come in handy when creating algorithms or commandline tools. I prefer using higher level languages like `C#` and `Java` when writing web/mobile/desktop applications because they enforce less strict rules and provide more functionality out of the box. 

I will be using the knowledge acquired from the functional paradigm a lot because it enables me to write code that expresses the intent more clearly. Instead of writing a for loop to filter a list or looping though the items and updating the contents, the map and filter functions can be used. After some research I found similar constructs for my favorite languages:
[LINQ](https://docs.microsoft.com/en-us/dotnet/api/system.linq.enumerable.where?view=netframework-4.8) for C#, [map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map) and [reduce](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce) for Javascript and [array_map](https://www.php.net/manual/en/function.array-map.php) and [array_filter](https://www.php.net/manual/en/function.array-filter.php) for PHP.
 

## Key concepts
The following sections are summaries of the book that I have written to better understand the concepts.

### References
- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

In the various language I have worked with such as PHP and C# there is no need to think about references, copies and pointers. 
You just assign the variable and it works. When working with objects/values in Rust you get a compile time error if a a variable may go out of scope or can't be mutated at multiple places.
Without reading the documentation it is quite hard to grasp why these errors occur and how to fix them, after reading the references and borrowing chapter it became way more clear. 

Rush takes a unique approach in managing memory, it checks where which memory will be used at compile time and what values will be available. Because of this the program is way more performant and doesn't need to garbage collect at  run time. 

Borrowing an instance using `&` is preferred because it allows the instance to be used later. When using methods in a struct `self` is passed as borrowed/non-mutable(`&self`) parameter because changing the object itself to a new instance almost never happens.
To pass it as mutable reference the following can be used `&mut self`, this will cause the pointer to be moved and will no longer be available if the method runs out of scope.    

When using methods the syntax for instance and non-instance functions is the same. The only difference is the addition of `self` for instance method. If the method signature doesn't contain a `self` parameter it is marked as an associated function. Associated functions can be called without an instance but are bound to the struct.
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

When using functions from an external package the best practice is to import the module it is in instead of polluting the global scope. This way it is clear that the function isn't located in the current file but in an imported module(`namespace::my_function`). When using Structs or enum it is recommended to import them directly to make the code less verbose(`use std::collections::HashMap`, `HashMap::new()`).
Because module names have to be unique the `as` keyword can be used to alias to non-unique names:
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

### Generics
Rust has support for generics, these can be used to prevent duplicate code and allow the same `structs` and `functions` to accept various types. In most other languages this causes an overhead because it needs to determine the type on run-time. 
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

It is also possible to test if a section panics. Panicking is something the desired behaviour if the program ends up in a faulty state. To test if the code `panicks` the following annotation can be used: `#[should_panic]`

### The following chapters
I stopped reading at [ch15-00-smart-pointers.html](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) because it felt i had solid understanding of the basic concepts. To practice with acquired knowledge I defined a global outline of my practice project at the top of this document and documented the process.