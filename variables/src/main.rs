/*

Rules of References
Rust primarily follows these rules of references at any given time:

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

if we write the code like this:
`
let mut a = 0;
let b = &a;
`
it means that `b` is a reference to `a`. In this case:
- `b` CANNOT be changed/pointed to something/somewhere else because `b` is immutable.
- `a` CANNOT be changed through `b` because `b` has borrowed `a` as an immutable reference.


if we write the code like this:
`
let mut a = 0;
let b = &mut a;
`
it means that `b` is a reference to `a`. In this case:
- `b` cannot be changed/pointed to something/somewhere else because `b` is immutable.
- `a` CAN be changed through `b` because `b` has borrowed `a` as an mutable reference.


if we write the code like this:
`
let mut a = 0;
let mut b = &mut a;
`
it means that `b` is a reference to `a`. In this case:
- `b` CAN be changed/pointed to something/somewhere else because `b` is mutable.
- `a` CAN be changed through `b` because `b` has borrowed `a` as an mutable reference.

----------------------------------------------------------------
let <put `mut` here and after the `&` if you want to change the value (0) through the reference (`b`)> a = 0;
let <put `mut` here if you want to assign a new reference to `b`> b = &<put `mut` here and after the `&` if you want to change the value (0) through the reference (`b`)>a;
----------------------------------------------------------------
*/

fn main() {
    let a = 0;
    println!("a: {}, &a: {:p}", a, &a);

    let x = 1;
    println!("x: {}, &x: {:p}", x, &x);

    let mut b = &a; 
  
    println!("1 b: {}, &b: {:p}", b, &b);

    /*
    In Rust, variables are immutable by default, but when you use the `let` keyword to
    create a new variable with the same name, it effectively shadows the previous
    variable, and the new variable can have a different type or be mutable if you explicitly specify it.
    */
    b = &x;
 
    println!("2 b: {}, &b: {:p}", b, &b);

}
