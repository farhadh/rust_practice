/*
`do_something` can take a value of any type and return it.
fn do_something<DataType>(value: DataType) -> DataType {
    value
}

- The `<DataType>` declares a generic type parameter named `DataType`.
- The second and third `DataType` (in value: DataType and -> DataType) use that declared `DataType` as a placeholder for some concrete type.



- The `swap` function takes two parameters of different types, `T` and `U`, and returns a tuple containing the values in reverse order.
fn swap<T, U>(a: T, b: U) -> (U, T) {
    (b, a)
}
- T and U are just placeholders.

The actual types are determined when you call the function:
let result = swap(1, "hello");  // T = i32, U = &str
*/

fn do_something<DataType>(value: DataType) -> DataType {
    value
}
