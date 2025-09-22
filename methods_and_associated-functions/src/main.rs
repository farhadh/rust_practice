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

We chose `&self` here for the same reason we used `&Rectangle` in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use `&mut self` as the first parameter.


In Rust, an implementation refers to the process of defining the behavior of a type (such as a struct, enum, or trait) by providing the code for its associated functions, methods, or trait requirements. This is done using the impl keyword, which allows you to specify how a type behaves, either by defining its own methods or by implementing traits for it.

Saying "X implements Y" means that there is an `impl Y for X` block that provides the concrete code for all the required items in the trait Y. This allows instances of type X to use the behavior defined by Y.

The phrase "X implements Y" is a shorthand way of saying that the type `X` has been provided with an implementation for the trait `Y`. The key is that someone (you, the programmer, or a tool like #[derive]) has defined the behavior required by the trait `Y` for the type `X`.

"X implements Y": `X` knows how to execute/perform the `Y` trait (behavior defined by `Y`), becuase the code has been written for it via `impl Y for X`.
*/


// a tuple struct `Foo` that holds a single `i32` value
struct Foo(i32);

impl Foo {
    // `new` is an associated function (no `self`).
    // It returns a new instance of `Foo` with 0 as the internal value.
    fn new() -> Self {
        Self(0)
    }

    // Consuming `self`.
    // consume takes ownership of `self`, incrementing the value, and returning a new `Foo`.
    /*
    - self is moved into the method (i.e. ownership is taken).
    - self.0 accesses the inner i32.
    - Self(self.0 + 1) is shorthand for Foo(self.0 + 1) — it creates a new Foo instance with the incremented value.

    - Foo::consume(foo) destroys the original `foo` and gives you a new `Foo`.
    - `foo` does not stay unchanged — it becomes invalid unless reassigned.
     */
    fn consume(self) -> Self {
        Self(self.0 + 1)
    }

    // Borrowing `self`.
    // borrow takes an immutable reference to `self` and returns a reference to the inner `i32`.
    fn borrow(&self) -> &i32 {
        &self.0
    }

    // Borrowing `self` mutably.
    // borrow_mut takes a mutable reference to self and returns a mutable reference to the inner i32.
    fn borrow_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

fn main(){
    // This method must be called with a `Type::` prefix.
    let foo = Foo::new();
    assert_eq!(foo.0, 0);

    // Those two calls produces the same result.
    // you are moving ownership of `foo` into the `consume` method. That means `foo` is no longer valid after the call — it doesn't "stay the same." It’s "consumed".
    /*
    let foo = Foo(2);
    let new_foo = foo.consume(); // foo is moved and cannot be used anymore

    If you try to use foo after this, you'll get a compile-time error.
     */
    let new_foo = Foo::consume(foo);
    assert_eq!(new_foo.0, 1);
    let new_new_foo = new_foo.consume();
    assert_eq!(new_new_foo.0, 2);

    // Borrowing is handled automatically with the second syntax.
    let borrow_1 = Foo::borrow(&new_new_foo);
    let borrow_2 = new_new_foo.borrow();
    assert_eq!(borrow_1, borrow_2);

    // Borrowing mutably is handled automatically too with the second syntax.
    let mut foo = Foo::new();
    *Foo::borrow_mut(&mut foo) += 1;
    assert_eq!(foo.0, 1);
    *foo.borrow_mut() += 1;
    assert_eq!(foo.0, 2);
}