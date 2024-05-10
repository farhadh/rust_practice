fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa: Foo = Foo { x: (1, 2), y: 3 };

    // You do not need a match block to destructure structs:
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p: Point = Point { x: 0 , y: 10 };

    match p {
        /*
        Here, `x` and `y` in the pattern (`Point { x , y }`) are placeholders or variables that we are creating to bind/assign the values of the `x` and `y` fields from the `Point` struct `p` Specifically:
            - `x`: This variable is being bound to the value of the `x` field of the `Point` struct `p` that
            matches the pattern. In other words, `x` will hold the value of `p.x`.
            - `y`: Similarly, this variable is being bound to the value of the `y` field of the Point struct p that matches the pattern. So, `y` will hold the value of `p.y`.

        Point { x, y } => println!("x: {}, y: {}", x, y)

        In the match arm `Point { x: 0, y }`, `y` is being bound to the value of the `y` field of the `Point` struct `p`. However, `x` is not being directly used in this arm, it's only being pattern matched against the value `0` – `0` is not bound to `x`.

        the variables `x` and `y` created within the match arm to hold the values extracted from the `Point` struct `p`. These bindings are only valid within the scope of that match arm.
        */
        Point { x: a , y: b } => println!("2. On the a axis at {} and b axis at {}", a , b)

        /*
        x: a: Here, we are matching the x field of the Point struct p with a new variable a. So, a is bound to the value of the x field of p.
        y: b: Similarly, we are matching the y field of the Point struct p with a new variable b. b is bound to the value of the y field of p.
        Then, we use these newly bound variables a and b within the println! macro to print the values.

        So, in this notation:

        a represents the value of the x field of p.
        b represents the value of the y field of p.
        These bindings are specific to this match arm and are only valid within its scope. They are separate from the fields x and y declared in the Point struct.
        */


    }
}