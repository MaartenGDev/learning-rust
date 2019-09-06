# Learning rust

## Resources
- https://www.youtube.com/watch?v=FYGS2q1bljE
- https://www.youtube.com/watch?v=zF34dRivLOw 


## Key concepts
- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

In the various language i have worked with such as PHP and C# there is no need to think about references, copies and pointers. 
You just assign the variable and it works. When working with Rust and you try copying some objects you get a compile time error that the variable is out of scope of can't be mutated.
Without reading the documentation it is quite hard to grasp why these errors occur and how to fix them, after reading the references and borrowing chapter it became way more clear. 

Rush takes a unique approach in managing memory, it checks where which memory will be used at compile and what values will be available. Because of this the program is way more performant and doesn't need to garbage collect at  run time. 

Borrowing an instance using `&` is preferred because it allows the instance to be used later. When using methods in struct implementation `self` is passed as borrowed/non-mutable(`&self`) because changing the object itself to a new instance almost never happens.
To pass it as mutable reference the following can be used `&mut self`, this will cause the pointer to be moved and will no longer be available if the method runs out of scope.    