/*
Static methods have limited use, because they don't have access to the attributes of an instance of a class (like a regular method does), and they don't have access to the attributes of the class itself (like a class method does).

So they aren't useful for day-to-day methods.

However, they can be useful to group some utility function together with a class - e.g. a simple conversion from one type to another - that doesn't need access to any information apart from the parameters provided (and perhaps some attributes global to the module.)

They could be put outside the class, but grouping them inside the class may make sense where they are only applicable there.

You can also reference the method via an instance or the class, rather than the module name, which may help the reader understand to what instance the method is related.
*/


fn main() {
    println!("Hello, world!");
}
