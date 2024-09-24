/*
Static methods have limited use, because they don't have access to the attributes of an instance of a class (like a regular method does), and they don't have access to the attributes of the class itself (like a class method does).

So they aren't useful for day-to-day methods.

However, they can be useful to group some utility function together with a class - e.g. a simple conversion from one type to another - that doesn't need access to any information apart from the parameters provided (and perhaps some attributes global to the module.)

They could be put outside the class, but grouping them inside the class may make sense where they are only applicable there.

You can also reference the method via an instance or the class, rather than the module name, which may help the reader understand to what instance the method is related.

Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language. For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

Filename: src/main.rs

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

the 2nd self (`Self { width...`) refers to the type that is implemented on (which is the `Rectangle`). In other words, the `Self` keywords in the return type and in the body of the function are aliases for the type that appears after the `impl` keyword, which in this case is `Rectangle`.

*/


fn main() {
    println!("Hello, world!");
}
